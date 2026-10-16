//! `barking fetch`: archive a list of web pages and PDFs in one call.
//!
//! Evidence executors used to save pages one `curl` per tool call (the 1014
//! issue: group A made 110 tool calls, most of them single fetches), and one
//! group stalled for about three hours on a request that never timed out.
//! This command takes the whole list at once, bounds every request, and
//! leaves the same files the hand-run `curl` + `html2txt` / `pdftotext` pair
//! did, so an agent spends its turns on reading, not on fetching.
//!
//! For each manifest line `name<TAB>url` it writes, under `sources/`:
//! `<group>-<name>.<ext>` (the body, decoded from gzip; the extension comes
//! from the magic bytes, then the Content-Type), `<group>-<name>.headers`
//! (status line, final URL, response headers) and, for a page that really
//! is content, `<group>-<name>.txt` (HTML decoded per its charset and run
//! through `fetch::html_to_text`, PDF through poppler's
//! `pdftotext -enc UTF-8 -layout`). Every item appends one line to
//! `sources/<group>-fetch-log.jsonl` as it finishes.
//!
//! Decisions worth knowing when reading the code:
//! - Requests are anonymous: ureq is built without its `cookies` feature, so
//!   there is no jar, and nothing here sets `Authorization`. Manifest URLs
//!   with `user:pass@` are refused, and redirects are followed by this module
//!   (not by ureq) so that a `Location` carrying userinfo is refused too.
//! - The repository is public and `.headers` is committed, so it must not
//!   place the fetcher: header names are all kept, but a value is printed
//!   only for the reviewed `SAFE_HEADERS`; `location` / `content-location`
//!   keep only `scheme://host`; everything else reads `[已删]` (CDN edge,
//!   request-id and client-IP headers say why: `LOCATION_HEADERS`). Every URL
//!   fetch publishes (`.headers` `URL:`, the log's `final_url`) loses
//!   userinfo, query and fragment (`public_url`); only the manifest URL is
//!   logged as given. The log carries no response headers and no local paths.
//! - One deadline per item, from its start, covers redirects, body, the retry
//!   and its pause, and the PDF re-request: 60 s, or 180 s when the URL path
//!   ends in `.pdf`. A response that turns out to be `application/pdf` at a
//!   URL without that suffix (arXiv `/pdf/<id>`) is dropped at its headers and
//!   requested again with the PDF budget, because ureq cannot extend a
//!   deadline it has already started. One retry, only on a timeout, a
//!   connection or I/O error, or a 5xx; a 4xx is an answer, and a protocol,
//!   TLS, decoding or redirect error would only fail again.
//! - "Blocked" is not "failed": 401/402/403/429; a `cf-mitigated: challenge`
//!   header or Cloudflare challenge markup (`GATE_SIGNATURES`) on any page;
//!   and short 2xx HTML pages that read as a challenge, login or paywall gate
//!   (`GATE_TEXT`, `GATE_MARKUP`). The phrase lists only apply to short pages,
//!   since real articles quote "access denied" too; a long teaser in front of
//!   a paywall therefore passes as ok, and the reader still has to look.
//!   Blocked and failed items keep their body and `.headers` (what the server
//!   answered is itself evidence) but get no `.txt`, so no gate page is ever
//!   grepped as if it were the source; `.txt` only ever holds usable text.
//! - Nothing is overwritten without `--force`: the whole batch is refused
//!   before any request if any item already has outputs, and every file is
//!   written to a temp file and moved onto a name reserved with `create_new`.
//!   With `--force` a base is one version: the previous run's files of that
//!   base are removed just before the new ones are written. There is no
//!   lock across processes; two batches on the same group race.
//! - At most `--jobs` requests run at once and at most two per manifest host
//!   (the host written in the manifest; redirect targets are not counted).
//!   The summary follows the manifest order; the log follows completion
//!   order, so a killed batch still leaves a record of what finished.

use std::fs;
use std::io::{self, ErrorKind, Read, Seek as _, SeekFrom, Write as _};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::{Condvar, Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use encoding_rs::{Encoding, UTF_8};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::fetch::{USER_AGENT, html_to_text};
use crate::repo::{self, Issue};
use crate::shot::{jsonl, log_url, pick_tool, poppler_dirs};

const USAGE: &str = "\
用法：barking fetch <期次目录> <组前缀> <清单文件|-> [--jobs 6] [--force]

清单每行一条：name<TAB>url（空行与 # 开头的行忽略；- 表示从标准输入读；URL 不得带 user:pass@）。
  name 只许小写字母、数字与 -，输出在 sources/<组前缀>-<name>.*：
    正文 .html / .pdf（先看文件头再看 Content-Type；其他类型为 .json .xml .png 等，未知或不可用作存档的文本为 .bin）
    .headers  状态行、最终 URL 与响应头：头名全留，值只留审定的安全头；location 类头只留 scheme://host；其余记为 [已删]（不暴露抓取地点）；最终 URL 去掉查询与片段
    .txt      文本存档：HTML 按声明的字符集解码后转纯文本（同 html2txt），PDF 用 poppler 的 pdftotext -enc UTF-8 -layout
  匿名 GET（桌面 Chrome UA、Accept-Language en-US、无 Cookie），自行跟随至多 10 次跳转（目标带用户名或密码则拒绝）；
  每条限时 60 秒（PDF 180 秒），从开始算起，跳转、重试与重新请求都在内；超时、连接或 I/O 错误与 5xx 重试一次，4xx、协议与解码错误不重试。
  并发 --jobs（默认 6，最多 16），同一清单域名最多 2 个（按清单里写的域名计，跳转后的域名不计）。
  401/402/403/429、Cloudflare 质询与疑似验证页、付费墙、过短正文记为 blocked：保留正文与 .headers，不生成 .txt。
  字符集无法识别或正文含无效字节、PDF 转文本失败或无可提取文本等记为 failed（正文保留，原因写明）。
  任一条目已有输出则整批拒绝；加 --force 时，每条写入前先删去该条上次的全部输出（同一 name 只留一版）。
  每条在 sources/<组前缀>-fetch-log.jsonl 追加一行（北京时间、URL、状态、字节、sha256、输出、结果与原因）。
  部分条目失败仍以 0 退出，末行写明失败数；用法错误、拒绝覆盖与日志写入失败以非 0 退出。
  pdftotext 先找环境变量 BARKING_POPPLER 指定的目录，再找 ~/scoop/apps/poppler/current/bin，最后 PATH。";

const DEFAULT_JOBS: usize = 6;
const MAX_JOBS: usize = 16;
/// In-flight requests per manifest host: enough to overlap two slow pages,
/// few enough not to look like a scraper to a news site's bot wall.
const PER_HOST: usize = 2;
const PAGE_TIMEOUT: Duration = Duration::from_secs(60);
const PDF_TIMEOUT: Duration = Duration::from_secs(180);
const RETRY_PAUSE: Duration = Duration::from_secs(2);
/// Browsers stop at 20; ureq's default is 10, which no evidence page needs.
const MAX_REDIRECTS: usize = 10;
/// pdftotext on a hostile or broken PDF can spin; it must not hang the batch.
const PDFTOTEXT_TIMEOUT: Duration = Duration::from_secs(120);
/// Court filings and papers run to tens of MB; a body over this (after gzip
/// decoding) is refused rather than buffered (up to `MAX_JOBS` at once).
const MAX_BODY: u64 = 128 * 1024 * 1024;
/// A text archive larger than this is not something anyone greps.
const MAX_PDF_TEXT: u64 = 32 * 1024 * 1024;
const ACCEPT: &str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,application/pdf;q=0.9,*/*;q=0.8";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9";
const MAX_NAME: usize = 60;
/// How far into an HTML body a `<meta charset>` is looked for (the HTML
/// standard's prescan reads 1024 bytes; real pages put comments first).
const META_PRESCAN: usize = 4096;

/// Statuses that mean "the server would not show this to us", as opposed to
/// "there is nothing here" (404, 410) or "the server broke" (5xx).
const BLOCKED_STATUS: [u16; 4] = [401, 402, 403, 429];
/// Fewer non-space characters than this in the extracted text: a JavaScript
/// shell or an empty gate. example.com, the smallest real page in common use,
/// has about 150.
const MIN_TEXT_CHARS: usize = 80;
/// Gate phrases are only looked for in texts shorter than this (non-space
/// characters). Challenge and denial pages are a few hundred; an article that
/// merely mentions "access denied" is far longer.
const SHORT_TEXT_CHARS: usize = 2000;
/// Markup that only Cloudflare challenge pages carry, matched in the raw HTML
/// at any length. Not `/cdn-cgi/challenge-platform/` as such: ordinary pages
/// on zones with bot detection load `/cdn-cgi/challenge-platform/scripts/jsd/
/// main.js`; the challenge page loads `…/challenge-platform/h/…/orchestrate`.
const GATE_SIGNATURES: [(&str, &str); 2] = [
    ("_cf_chl_opt", "Cloudflare 质询"),
    ("/cdn-cgi/challenge-platform/h/", "Cloudflare 质询"),
];
/// Phrases of challenge, denial, login and paywall pages, lower-case, matched
/// in the extracted text of a short page; the second field names the gate.
const GATE_TEXT: [(&str, &str); 16] = [
    ("just a moment", "Cloudflare 验证页"),
    ("attention required", "Cloudflare 拦截页"),
    ("checking your browser", "浏览器验证页"),
    (
        "enable javascript and cookies",
        "要求开启 JavaScript 与 Cookie",
    ),
    ("verify you are human", "人机验证"),
    ("verifying you are human", "人机验证"),
    ("are you a robot", "人机验证"),
    ("unusual traffic", "异常流量拦截"),
    ("access denied", "拒绝访问页"),
    ("have permission to access", "拒绝访问页"),
    ("disable any ad blocker", "DataDome 验证页"),
    ("subscribe to continue reading", "付费墙"),
    ("this content is for subscribers", "付费墙"),
    ("sign in to continue", "要求登录"),
    ("安全验证", "安全验证页"),
    ("访问被拒绝", "拒绝访问页"),
];
/// Captcha markup that carries no text of its own, matched in the raw HTML
/// of a short page (a long article may embed a captcha in its comment form).
const GATE_MARKUP: [(&str, &str); 5] = [
    ("cf-turnstile", "Cloudflare Turnstile"),
    ("g-recaptcha", "reCAPTCHA"),
    ("h-captcha", "hCaptcha"),
    ("captcha-delivery.com", "DataDome 验证"),
    ("px-captcha", "PerimeterX 验证"),
];
/// Every extension `choose_ext` can return, plus the two sidecars: the
/// no-overwrite check and the `--force` clean-up look for all of them,
/// because the body's extension is only known after the fetch.
const OUTPUT_EXTS: [&str; 14] = [
    "headers", "txt", "html", "pdf", "json", "xml", "md", "csv", "svg", "png", "jpg", "gif",
    "webp", "bin",
];

// ---------------------------------------------------------------------------
// Command line and manifest

#[derive(Debug, PartialEq)]
struct Args {
    issue_dir: PathBuf,
    group: String,
    /// A path, or `-` for standard input.
    manifest: String,
    jobs: usize,
    force: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct Item {
    name: String,
    url: String,
    /// Lower-cased host of the manifest URL, the key of the per-host limit.
    host: String,
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn parse_args(args: &[String]) -> Result<Args, String> {
    let mut positional: Vec<&String> = Vec::new();
    let (mut jobs, mut force) = (DEFAULT_JOBS, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let (flag, inline) = match a.split_once('=') {
            Some((f, v)) if f.starts_with("--") => (f, Some(v.to_string())),
            _ => (a.as_str(), None),
        };
        match flag {
            "--force" if inline.is_none() => force = true,
            "--jobs" => {
                let v = match inline {
                    Some(v) => v,
                    None => it.next().ok_or("--jobs 缺少参数")?.clone(),
                };
                jobs = v
                    .parse::<usize>()
                    .ok()
                    .filter(|n| (1..=MAX_JOBS).contains(n))
                    .ok_or_else(|| format!("--jobs 须为 1–{MAX_JOBS} 的整数，收到「{v}」"))?;
            }
            // `-` alone is the stdin manifest, not an option.
            f if f.starts_with("--") => return Err(format!("未知选项 {a}")),
            _ => positional.push(a),
        }
    }
    let [dir, group, manifest] = positional.as_slice() else {
        return Err("需要期次目录、组前缀与清单文件三个位置参数".into());
    };
    if !is_slug(group) {
        return Err(format!("组前缀只允许小写字母、数字与 -，收到「{group}」"));
    }
    Ok(Args {
        issue_dir: PathBuf::from(dir),
        group: group.to_string(),
        manifest: manifest.to_string(),
        jobs,
        force,
    })
}

/// `(scheme://, authority)` of an absolute URL; the authority runs to the
/// first `/`, `?` or `#`.
fn split_authority(url: &str) -> Option<(&str, &str)> {
    let i = url.find("://")? + 3;
    let rest = &url[i..];
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    Some((&url[..i], &rest[..end]))
}

fn has_userinfo(url: &str) -> bool {
    split_authority(url).is_some_and(|(_, auth)| auth.contains('@'))
}

/// Lower-cased host of an `http(s)://` URL; `None` for anything else.
fn host_of(url: &str) -> Option<String> {
    let lower = url
        .get(..8)
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if !(lower.starts_with("http://") || lower.starts_with("https://")) {
        return None;
    }
    let uri: ureq::http::Uri = url.parse().ok()?;
    let host = uri.host()?.to_ascii_lowercase();
    (!host.is_empty()).then_some(host)
}

/// A URL as copied from a browser's address bar may carry raw non-ASCII
/// (`/新闻/…`) or spaces, which the HTTP library refuses to parse. Those bytes
/// after the host are percent-encoded as UTF-8, as the browser itself sends
/// them; existing `%XX` escapes and the host are left alone.
fn encode_url(url: &str) -> String {
    let start = url.find("://").map_or(0, |i| i + 3);
    let path_at = url[start..]
        .find(['/', '?', '#'])
        .map_or(url.len(), |i| start + i);
    let mut out = url[..path_at].to_string();
    for &b in &url.as_bytes()[path_at..] {
        if b >= 0x80 || b" \"<>`{}|\\^".contains(&b) {
            out.push_str(&format!("%{b:02X}"));
        } else {
            out.push(char::from(b));
        }
    }
    out
}

/// `name<TAB>url` lines → items. Blank lines and `#` lines are skipped; any
/// other malformed line, bad name, non-http(s) URL, URL with userinfo or
/// repeated name fails the whole manifest (line numbers are 1-based), before
/// anything is fetched.
fn parse_manifest(text: &str) -> Result<Vec<Item>, String> {
    let mut items: Vec<Item> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for (n, raw) in text.trim_start_matches('\u{feff}').lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n = n + 1;
        let Some((name, url)) = line.split_once('\t') else {
            errors.push(format!("第 {n} 行：须为 name<TAB>url"));
            continue;
        };
        let (name, url) = (name.trim(), url.trim());
        if !is_slug(name) || name.len() > MAX_NAME {
            errors.push(format!(
                "第 {n} 行：name「{name}」只许小写字母、数字与 -，不以 - 开头，至多 {MAX_NAME} 字符"
            ));
            continue;
        }
        let url = &encode_url(url);
        // Before the URL is echoed anywhere: it may hold a password.
        if has_userinfo(url) {
            errors.push(format!(
                "第 {n} 行：URL 带用户名或密码（user:pass@），拒绝（本工具只做匿名请求）"
            ));
            continue;
        }
        let Some(host) = host_of(url) else {
            let why = if url.is_ascii() {
                "不是有效的 http(s) URL"
            } else {
                "域名含非 ASCII 字符，请改写为 punycode（xn--…）"
            };
            errors.push(format!("第 {n} 行：「{url}」{why}"));
            continue;
        };
        if items.iter().any(|i| i.name == name) {
            errors.push(format!("第 {n} 行：name「{name}」重复"));
            continue;
        }
        items.push(Item {
            name: name.to_string(),
            url: url.to_string(),
            host,
        });
    }
    if !errors.is_empty() {
        return Err(format!("清单有误：\n  {}", errors.join("\n  ")));
    }
    if items.is_empty() {
        return Err("清单为空".into());
    }
    Ok(items)
}

// ---------------------------------------------------------------------------
// Scheduling

/// Position in `pending` of the first item whose host has fewer than `cap`
/// requests in flight. `host_of[i]` is item `i`'s host id, `busy[h]` the
/// in-flight count of host `h`. Pure, so the limit is testable without
/// threads: first-come order is kept among hosts that have room.
fn next_runnable(
    pending: &[usize],
    host_of: &[usize],
    busy: &[usize],
    cap: usize,
) -> Option<usize> {
    pending.iter().position(|&i| busy[host_of[i]] < cap)
}

struct Sched {
    /// Manifest indices not yet started, in manifest order.
    pending: Vec<usize>,
    busy: Vec<usize>,
}

/// Blocks until an item may start (or none is left), and claims it.
fn claim(state: &Mutex<Sched>, wake: &Condvar, host_of: &[usize], cap: usize) -> Option<usize> {
    let mut st = state.lock().unwrap_or_else(PoisonError::into_inner);
    loop {
        if st.pending.is_empty() {
            return None;
        }
        if let Some(p) = next_runnable(&st.pending, host_of, &st.busy, cap) {
            let i = st.pending.remove(p);
            st.busy[host_of[i]] += 1;
            return Some(i);
        }
        // Every remaining host is at its cap, so some worker is mid-request
        // on it and will notify when its `Slot` drops.
        st = wake.wait(st).unwrap_or_else(PoisonError::into_inner);
    }
}

/// A claimed host slot, released on drop, so that even a panicking job frees
/// its host and wakes the waiters instead of leaving them blocked forever.
struct Slot<'a> {
    state: &'a Mutex<Sched>,
    wake: &'a Condvar,
    host: usize,
}

impl Drop for Slot<'_> {
    fn drop(&mut self) {
        let mut st = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        st.busy[self.host] -= 1;
        drop(st);
        self.wake.notify_all();
    }
}

/// Runs `work(i)` for every index of `host_of` on at most `jobs` threads, at
/// most `cap` at a time per host id, and returns the results in index order.
///
/// Scoped threads let the workers borrow `host_of`, `work` and the scheduler
/// state straight from this stack frame: `thread::scope` joins them all
/// before returning, so none of it needs an `Arc` or a `'static` copy. Each
/// worker keeps its own `(index, result)` list and hands it back through
/// `join`, so results need no lock of their own; `work` must be `Sync`
/// because every worker calls it through the same shared reference, and `T`
/// `Send` because results cross back to this thread.
fn run_pool<T: Send>(
    host_of: &[usize],
    jobs: usize,
    cap: usize,
    work: impl Fn(usize) -> T + Sync,
) -> Vec<T> {
    let hosts = host_of.iter().max().map_or(0, |m| m + 1);
    let state = Mutex::new(Sched {
        pending: (0..host_of.len()).collect(),
        busy: vec![0; hosts],
    });
    let wake = Condvar::new();
    let workers = jobs.clamp(1, host_of.len().max(1));
    let mut done: Vec<(usize, T)> = thread::scope(|s| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                s.spawn(|| {
                    let mut mine = Vec::new();
                    while let Some(i) = claim(&state, &wake, host_of, cap) {
                        let _slot = Slot {
                            state: &state,
                            wake: &wake,
                            host: host_of[i],
                        };
                        mine.push((i, work(i)));
                    }
                    mine
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_else(|p| panic::resume_unwind(p)))
            .collect()
    });
    done.sort_by_key(|&(i, _)| i);
    done.into_iter().map(|(_, t)| t).collect()
}

/// Dense host ids in order of first appearance.
fn host_ids(items: &[Item]) -> Vec<usize> {
    let mut seen: Vec<&str> = Vec::new();
    items
        .iter()
        .map(|it| {
            seen.iter().position(|h| *h == it.host).unwrap_or_else(|| {
                seen.push(&it.host);
                seen.len() - 1
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Classification

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    Ok,
    Blocked,
    Failed,
}

impl Verdict {
    fn as_str(self) -> &'static str {
        match self {
            Verdict::Ok => "ok",
            Verdict::Blocked => "blocked",
            Verdict::Failed => "failed",
        }
    }
}

/// What the status alone says; a 2xx may still turn out to be a gate.
fn status_verdict(status: u16) -> Verdict {
    if (200..300).contains(&status) {
        Verdict::Ok
    } else if BLOCKED_STATUS.contains(&status) {
        Verdict::Blocked
    } else {
        Verdict::Failed
    }
}

/// The `cf-mitigated: challenge` header: a Cloudflare challenge whatever
/// the body.
fn challenge_header(headers: &[(String, String)]) -> Option<String> {
    headers
        .iter()
        .any(|(k, v)| {
            k.eq_ignore_ascii_case("cf-mitigated") && v.to_ascii_lowercase().contains("challenge")
        })
        .then(|| "Cloudflare 质询（cf-mitigated: challenge）".to_string())
}

/// `GATE_SIGNATURES` in the decoded HTML, whatever the page length.
fn challenge_markup(html: &str) -> Option<String> {
    let raw = html.to_ascii_lowercase();
    GATE_SIGNATURES
        .iter()
        .find(|(n, _)| raw.contains(n))
        .map(|(needle, what)| format!("{what}（含 {needle}）"))
}

/// Why a 2xx HTML page is not the content, or `None` if it may be. `text` is
/// its `html_to_text` output, `raw` the decoded HTML.
fn detect_gate(raw: &str, text: &str) -> Option<String> {
    let chars = text.chars().filter(|c| !c.is_whitespace()).count();
    if chars < MIN_TEXT_CHARS {
        return Some(format!(
            "正文过短（{chars} 个非空白字符），可能要 JavaScript 渲染或被拦截"
        ));
    }
    if chars >= SHORT_TEXT_CHARS {
        return None;
    }
    let lower = text.to_lowercase();
    if let Some((needle, what)) = GATE_TEXT.iter().find(|(n, _)| lower.contains(n)) {
        return Some(format!("疑似{what}（含“{needle}”）"));
    }
    let raw = raw.to_ascii_lowercase();
    GATE_MARKUP
        .iter()
        .find(|(n, _)| raw.contains(n))
        .map(|(needle, what)| format!("疑似{what}（含 {needle}）"))
}

fn mime(content_type: &str) -> String {
    content_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase()
}

fn sniff_html(body: &[u8]) -> bool {
    let head = &body[..body.len().min(1024)];
    let head = String::from_utf8_lossy(head).to_ascii_lowercase();
    let head = head.trim_start_matches('\u{feff}').trim_start();
    head.starts_with("<!doctype html") || head.starts_with("<html") || head.contains("<head")
}

/// The body's file extension: magic bytes first (a PDF served as
/// `application/octet-stream` is still a PDF), then the Content-Type, then a
/// look for HTML. `txt` means plain text, archived as `.txt` only when it is
/// usable as such (see `Job::archive`).
fn choose_ext(content_type: &str, body: &[u8]) -> &'static str {
    let head = &body[..body.len().min(1024)];
    if head.windows(5).any(|w| w == b"%PDF-") {
        return "pdf";
    }
    if head.starts_with(b"\x89PNG\r\n\x1a\n") {
        return "png";
    }
    if head.starts_with(b"\xff\xd8\xff") {
        return "jpg";
    }
    if head.starts_with(b"GIF8") {
        return "gif";
    }
    if head.len() >= 12 && head.starts_with(b"RIFF") && &head[8..12] == b"WEBP" {
        return "webp";
    }
    let m = mime(content_type);
    match m.as_str() {
        "text/html" | "application/xhtml+xml" => "html",
        // Labelled PDF but no `%PDF-`: an error page in disguise, usually.
        "application/pdf" => {
            if sniff_html(body) {
                "html"
            } else {
                "pdf"
            }
        }
        "image/svg+xml" => "svg",
        "text/plain" => "txt",
        "text/markdown" => "md",
        "text/csv" => "csv",
        "application/json" => "json",
        "application/xml" | "text/xml" => "xml",
        _ if m.ends_with("+json") => "json",
        _ if m.ends_with("+xml") => "xml",
        _ if sniff_html(body) => "html",
        _ => "bin",
    }
}

fn is_pdf_url(url: &str) -> bool {
    url.split(['?', '#'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
        .ends_with(".pdf")
}

// ---------------------------------------------------------------------------
// Charsets

/// The `charset` parameter of a Content-Type, unquoted.
fn ct_charset(content_type: &str) -> Option<String> {
    content_type.split(';').skip(1).find_map(|p| {
        let (k, v) = p.split_once('=')?;
        k.trim()
            .eq_ignore_ascii_case("charset")
            .then(|| v.trim().trim_matches(['"', '\'']).to_string())
            .filter(|v| !v.is_empty())
    })
}

/// The charset an HTML head declares, in `<meta charset=…>` or
/// `<meta http-equiv="Content-Type" content="…; charset=…">`. Comments are
/// skipped (a commented-out `<meta>` declares nothing), as in the HTML
/// standard's prescan.
fn meta_charset(body: &[u8]) -> Option<String> {
    let head = String::from_utf8_lossy(&body[..body.len().min(META_PRESCAN)]).to_ascii_lowercase();
    let mut from = 0;
    while let Some(off) = head[from..].find('<') {
        let start = from + off;
        if head[start..].starts_with("<!--") {
            from = head[start + 4..]
                .find("-->")
                .map_or(head.len(), |e| start + 4 + e + 3);
            continue;
        }
        let end = head[start..].find('>').map_or(head.len(), |e| start + e);
        let tag = &head[start..end];
        if tag.starts_with("<meta")
            && let Some(c) = tag.find("charset")
            && let Some(rest) = tag[c + 7..].trim_start().strip_prefix('=')
        {
            let label: String = rest
                .trim_start()
                .trim_start_matches(['"', '\''])
                .chars()
                .take_while(|ch| ch.is_ascii_alphanumeric() || "-_.:".contains(*ch))
                .collect();
            if !label.is_empty() {
                return Some(label);
            }
        }
        from = end.max(start + 1);
    }
    None
}

/// A body decoded to text.
struct Decoded {
    text: String,
    enc: &'static Encoding,
    /// The body is UTF-8 without a BOM, valid as it stands: only then may
    /// it be reused byte for byte as a `.txt`.
    utf8_as_is: bool,
}

/// The body as text: a BOM wins, then the Content-Type charset, then (HTML
/// only) a `<meta>` declaration, then UTF-8. A `<meta>` that says UTF-16
/// means UTF-8 (HTML standard: a page that could read its own `<meta>` as
/// ASCII is not UTF-16). An unknown label, or bytes that are invalid in the
/// chosen encoding, are errors: replacement characters would put silent
/// gaps into an archive that looks fine.
fn decode_body(content_type: &str, body: &[u8], html: bool) -> Result<Decoded, String> {
    let (enc, rest, bom) = match Encoding::for_bom(body) {
        Some((enc, n)) => (enc, &body[n..], true),
        None => {
            let declared = match ct_charset(content_type) {
                Some(l) => Some((l, false)),
                None if html => meta_charset(body).map(|l| (l, true)),
                None => None,
            };
            let enc = match declared {
                None => UTF_8,
                Some((l, from_meta)) => {
                    let enc = Encoding::for_label(l.as_bytes()).ok_or_else(|| {
                        let shown: String = l.chars().take(40).collect();
                        format!("无法识别的字符集「{shown}」")
                    })?;
                    if from_meta && (enc == encoding_rs::UTF_16LE || enc == encoding_rs::UTF_16BE) {
                        UTF_8
                    } else {
                        enc
                    }
                }
            };
            (enc, body, false)
        }
    };
    let text = enc
        .decode_without_bom_handling_and_without_replacement(rest)
        .ok_or_else(|| format!("正文含 {} 下无效的字节", enc.name()))?;
    Ok(Decoded {
        text: text.into_owned(),
        enc,
        utf8_as_is: enc == UTF_8 && !bom,
    })
}

// ---------------------------------------------------------------------------
// Files

/// Output names of `base` already present in `sources`.
fn existing_outputs(sources: &Path, base: &str) -> Vec<String> {
    OUTPUT_EXTS
        .iter()
        .map(|ext| format!("{base}.{ext}"))
        .filter(|f| sources.join(f).exists())
        .collect()
}

/// `--force`: every file this tool may have written for `base` goes, so the
/// new result never sits next to a stale sibling of an older run.
fn clear_base(sources: &Path, base: &str) -> Result<(), String> {
    for file in existing_outputs(sources, base) {
        let path = sources.join(&file);
        if let Err(e) = fs::remove_file(&path)
            && e.kind() != ErrorKind::NotFound
        {
            eprintln!("warn: {}：删除失败：{e}", path.display());
            return Err(format!("sources/{file}：旧文件删除失败（{e}）"));
        }
    }
    Ok(())
}

/// Writes `sources/<file>` without ever replacing a file and without a
/// half-written file under the final name: the name is reserved with
/// `create_new`, the bytes go to a temp file in the same directory, and the
/// temp file is renamed over the (empty, ours) reservation. Errors name the
/// file relative to the issue (they reach the log and `.headers`); the full
/// path goes to stderr only.
fn put(sources: &Path, file: &str, bytes: &[u8]) -> Result<(), String> {
    let target = sources.join(file);
    let shown = format!("sources/{file}");
    let report = |what: &str, e: &io::Error| {
        eprintln!("warn: {}：{what}：{e}", target.display());
        format!("{shown}：{what}（{e}）")
    };
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)
    {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            return Err(format!("{shown} 已存在，拒绝覆盖"));
        }
        Err(e) => return Err(report("创建失败", &e)),
    }
    let tmp = sources.join(format!(".{file}.{}.part", std::process::id()));
    if let Err(e) = fs::write(&tmp, bytes).and_then(|()| fs::rename(&tmp, &target)) {
        let _ = fs::remove_file(&tmp);
        let _ = fs::remove_file(&target);
        return Err(report("写入失败", &e));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Opens the log for appending, once per batch: an unwritable log stops the
/// batch before any request. A last line that lost its newline is closed
/// here, by reading one byte, not the whole file.
fn open_log(path: &Path) -> io::Result<fs::File> {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;
    let len = f.metadata()?.len();
    if len > 0 {
        let mut last = [0u8];
        f.seek(SeekFrom::Start(len - 1))?;
        f.read_exact(&mut last)?;
        if last[0] != b'\n' {
            f.write_all(b"\n")?;
        }
    }
    Ok(f)
}

// ---------------------------------------------------------------------------
// Fetching

/// One HTTP response as it is archived.
struct Got {
    status: u16,
    status_line: String,
    /// Where the redirect chain ended.
    final_url: String,
    content_type: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

enum Attempt {
    Got(Got),
    /// Headers said PDF, but the request ran on the page budget.
    NeedsPdfBudget,
    /// Worth one more try (timeout, connection or I/O error).
    Retry(String),
    /// Not worth another try (protocol, TLS, decoding, redirect, size).
    Fatal(String),
}

/// The HTTP side of a batch: the client and the limits tests shrink.
struct Net {
    agent: ureq::Agent,
    max_body: u64,
    retry_pause: Duration,
    page_timeout: Duration,
    pdf_timeout: Duration,
}

impl Net {
    fn new() -> Net {
        Net {
            agent: http_agent(true),
            max_body: MAX_BODY,
            retry_pause: RETRY_PAUSE,
            page_timeout: PAGE_TIMEOUT,
            pdf_timeout: PDF_TIMEOUT,
        }
    }
}

/// Redirects are left to `attempt` (so each `Location` can be checked);
/// `env_proxy` keeps ureq's `HTTP(S)_PROXY` handling, which tests against a
/// loopback server switch off.
fn http_agent(env_proxy: bool) -> ureq::Agent {
    let b = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent(USER_AGENT)
        .accept(ACCEPT)
        .max_redirects(0)
        .max_redirects_will_error(false);
    let b = if env_proxy { b } else { b.proxy(None) };
    b.build().into()
}

/// Timeouts, failed connections and plain I/O errors may pass; protocol,
/// TLS, decompression, URL and redirect errors will not.
fn retryable(e: &ureq::Error) -> bool {
    match e {
        ureq::Error::Timeout(_) | ureq::Error::HostNotFound | ureq::Error::ConnectionFailed => true,
        ureq::Error::Io(io) => !matches!(
            io.kind(),
            ErrorKind::InvalidData | ErrorKind::InvalidInput | ErrorKind::Unsupported
        ),
        _ => false,
    }
}

fn classify(what: &str, e: ureq::Error) -> Attempt {
    let msg = format!("{what}：{e}");
    if retryable(&e) {
        Attempt::Retry(msg)
    } else {
        Attempt::Fatal(msg)
    }
}

/// RFC 3986 §5.2.4 `remove_dot_segments`.
fn remove_dot_segments(path: &str) -> String {
    let mut input = path.to_string();
    let mut out = String::new();
    let pop = |out: &mut String| match out.rfind('/') {
        Some(i) => out.truncate(i),
        None => out.clear(),
    };
    while !input.is_empty() {
        if let Some(rest) = input.strip_prefix("../") {
            input = rest.to_string();
        } else if let Some(rest) = input.strip_prefix("./") {
            input = rest.to_string();
        } else if input.starts_with("/./") {
            input.replace_range(..3, "/");
        } else if input == "/." {
            input = "/".into();
        } else if input.starts_with("/../") {
            input.replace_range(..4, "/");
            pop(&mut out);
        } else if input == "/.." {
            input = "/".into();
            pop(&mut out);
        } else if input == "." || input == ".." {
            input.clear();
        } else {
            let skip = usize::from(input.starts_with('/'));
            let end = input[skip..].find('/').map_or(input.len(), |i| skip + i);
            out.push_str(&input[..end]);
            input.replace_range(..end, "");
        }
    }
    out
}

/// `origin + path?query`, with the dot segments of the path removed.
fn with_clean_path(origin: &str, rest: &str) -> String {
    let split = rest.find(['?', '#']).unwrap_or(rest.len());
    let path = remove_dot_segments(&rest[..split]);
    let path = if path.is_empty() {
        "/".to_string()
    } else {
        path
    };
    format!("{origin}{path}{}", &rest[split..])
}

/// `loc` (a `Location` value) against `base` (RFC 3986 §5.2): absolute,
/// scheme-relative, absolute-path, query-only or relative-path, dot
/// segments removed.
fn resolve(base: &str, loc: &str) -> String {
    let loc = loc.trim();
    let has_scheme = loc.find(':').is_some_and(|i| {
        i > 0
            && loc[..i]
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
    });
    let target = if has_scheme {
        loc.to_string()
    } else if let Some((scheme, auth)) = split_authority(base) {
        let origin = format!("{scheme}{auth}");
        let path_query = base[origin.len()..].split('#').next().unwrap_or("");
        let path = path_query.split('?').next().unwrap_or("");
        if let Some(rest) = loc.strip_prefix("//") {
            format!("{scheme}{rest}")
        } else if loc.starts_with('/') {
            format!("{origin}{loc}")
        } else if loc.starts_with('?') {
            format!("{origin}{path}{loc}")
        } else if loc.starts_with('#') || loc.is_empty() {
            format!("{origin}{path_query}")
        } else {
            let dir = path.rfind('/').map_or("/", |i| &path[..=i]);
            format!("{origin}{dir}{loc}")
        }
    } else {
        return loc.to_string();
    };
    match split_authority(&target) {
        Some((scheme, auth)) => {
            let origin_len = scheme.len() + auth.len();
            with_clean_path(&target[..origin_len], &target[origin_len..])
        }
        None => target,
    }
}

/// The next URL of a redirect chain, or why it is not followed. The
/// `Location` itself is never echoed: it may carry a password.
fn next_url(base: &str, loc: &str) -> Result<String, String> {
    let url = encode_url(&resolve(base, loc));
    if has_userinfo(&url) {
        return Err("跳转目标带用户名或密码，拒绝跟随".into());
    }
    if host_of(&url).is_none() {
        return Err("跳转目标不是有效的 http(s) URL".into());
    }
    Ok(url)
}

/// One GET with its redirect chain, all before `deadline`. `pdf_budget`
/// says the deadline is already the PDF one.
fn attempt(net: &Net, url: &str, deadline: Instant, pdf_budget: bool) -> Attempt {
    let mut current = url.to_string();
    let mut hops = 0;
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Attempt::Retry("条目限时已到".into());
        }
        let call = net
            .agent
            .get(&current)
            .header("Accept-Language", ACCEPT_LANGUAGE)
            .config()
            .timeout_global(Some(left))
            .build()
            .call();
        let mut resp = match call {
            Ok(r) => r,
            Err(e) => return classify("请求失败", e),
        };
        let status = resp.status();
        let header = |name: &str| {
            resp.headers()
                .get(name)
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
        };
        if status.is_redirection()
            && let Some(loc) = header("location")
        {
            hops += 1;
            if hops > MAX_REDIRECTS {
                return Attempt::Fatal(format!("跳转超过 {MAX_REDIRECTS} 次"));
            }
            match next_url(&current, &loc) {
                Ok(next) => current = next,
                Err(e) => return Attempt::Fatal(e),
            }
            continue;
        }
        let content_type = header("content-type").unwrap_or_default();
        // By Content-Type only: a PDF recognised by its magic bytes alone
        // (served as octet-stream at a non-.pdf URL) stays on the page budget.
        if !pdf_budget && mime(&content_type) == "application/pdf" {
            return Attempt::NeedsPdfBudget;
        }
        let headers: Vec<(String, String)> = resp
            .headers()
            .iter()
            .map(|(k, v)| {
                let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
                (k.as_str().to_string(), v)
            })
            .collect();
        let status_line = format!("{:?} {status}", resp.version());
        // The limit is applied to the decoded stream: ureq's own `limit`
        // counts the bytes before gzip, so a small bomb would pass it.
        let mut body = Vec::new();
        let read = resp
            .body_mut()
            .with_config()
            .reader()
            .take(net.max_body + 1)
            .read_to_end(&mut body);
        if let Err(e) = read {
            return classify("读取响应失败", ureq::Error::from(e));
        }
        if body.len() as u64 > net.max_body {
            return Attempt::Fatal(format!("解压后正文超过 {} 字节上限，未保存", net.max_body));
        }
        return Attempt::Got(Got {
            status: status.as_u16(),
            status_line,
            final_url: current,
            content_type,
            headers,
            body,
        });
    }
}

/// The response to archive, plus notes on what it took; `Err` when no
/// response came. A 5xx that persists through the retry is still returned:
/// what the server answered is the record.
///
/// One deadline covers the whole item — redirects, the retry and its pause,
/// and the PDF re-request — measured from the item's start: 60 s, or 180 s
/// once the URL or a response says PDF. A retry gets whatever is left.
fn fetch_url(net: &Net, url: &str) -> (Result<Got, String>, Vec<String>) {
    let start = Instant::now();
    let mut pdf_budget = is_pdf_url(url);
    let deadline = |pdf: bool| {
        start
            + if pdf {
                net.pdf_timeout
            } else {
                net.page_timeout
            }
    };
    let mut notes = Vec::new();
    let mut retried = false;
    loop {
        let (failure, response) = match attempt(net, url, deadline(pdf_budget), pdf_budget) {
            // Cannot repeat: the budget is now the PDF one.
            Attempt::NeedsPdfBudget => {
                pdf_budget = true;
                notes.push(format!(
                    "响应为 PDF，按 {} 秒限时重新请求",
                    net.pdf_timeout.as_secs()
                ));
                continue;
            }
            Attempt::Got(g) if g.status >= 500 => (g.status_line.clone(), Some(g)),
            Attempt::Got(g) => return (Ok(g), notes),
            Attempt::Fatal(e) => return (Err(e), notes),
            Attempt::Retry(e) => (e, None),
        };
        if retried {
            return (
                response.ok_or_else(|| format!("{failure}（已重试 1 次）")),
                notes,
            );
        }
        retried = true;
        notes.push(format!("首次失败（{failure}），已重试"));
        let left = deadline(pdf_budget).saturating_duration_since(Instant::now());
        thread::sleep(net.retry_pause.min(left));
    }
}

// ---------------------------------------------------------------------------
// The `.headers` sidecar

/// Headers whose value is a fixed-vocabulary property of the resource or
/// the server — the same for every client — and useful as evidence (type,
/// size, dates, cache state, the block itself). Everything else is `[已删]`:
/// an allowlist, because CDNs keep inventing headers that name the edge or
/// the client. Left out on purpose, as free-form strings that can be minted
/// per response: `etag` (a known per-client tracker), `server` (some CDNs
/// put the edge node in it), `content-disposition` (a download name can carry
/// a token); `www-authenticate` keeps only its scheme names
/// (`auth_schemes`), since a Digest nonce can be tied to the client address.
const SAFE_HEADERS: [&str; 22] = [
    "accept-ranges",
    "age",
    "allow",
    "cache-control",
    "cf-cache-status",
    "cf-mitigated",
    "connection",
    "content-encoding",
    "content-language",
    "content-length",
    "content-range",
    "content-type",
    "date",
    "expires",
    "last-modified",
    "pragma",
    "retry-after",
    "strict-transport-security",
    "transfer-encoding",
    "vary",
    "x-content-type-options",
    "x-frame-options",
];
/// Headers whose value is one URL: only its `scheme://host` is printed
/// (`url_origin`). `link` and `refresh` carry URLs inside parameters and are
/// simply `[已删]`.
const ORIGIN_HEADERS: [&str; 2] = ["location", "content-location"];
/// Headers known to name the CDN edge (POP), carry a request id the CDN maps
/// back to that edge and the client address, or echo the client IP. They
/// are not in `SAFE_HEADERS` either; the separate list only gives their
/// removal its reason. `cf-ray` loses its id too, not only the `-XXX` POP
/// suffix: the id is a key into Cloudflare's logs of this very request.
const LOCATION_HEADERS: [&str; 22] = [
    // Edge / POP names.
    "cf-ray",
    "x-amz-cf-pop",
    "x-amz-cf-id",
    "x-served-by",
    "x-cache",
    "via",
    "server-timing",
    "fly-request-id",
    "x-vercel-id",
    "x-azure-ref",
    "x-msedge-ref",
    "akamai-grn",
    "x-edge-location",
    "x-cdn-pop",
    // Cloudflare puts an opaque per-request token (`?s=…`) in the NEL
    // reporting URL; not readable, so not trusted to be location-free.
    "report-to",
    "reporting-endpoints",
    // Echoes of the client address.
    "x-forwarded-for",
    "x-real-ip",
    "cf-connecting-ip",
    "x-client-ip",
    "true-client-ip",
    "forwarded",
];
/// Header-name prefixes of whole families that carry POP or client fields.
const LOCATION_HEADER_PREFIXES: [&str; 2] = ["x-fastly-", "x-akamai-"];
const LOCATION_REDACTED: &str = "[已删：可能暴露抓取地点]";
const REDACTED: &str = "[已删]";

fn reveals_location(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    LOCATION_HEADERS.contains(&name.as_str())
        || LOCATION_HEADER_PREFIXES.iter().any(|p| name.starts_with(p))
}

/// `scheme://host[:port]` of an absolute URL, `//host[:port]` of a
/// scheme-relative one, userinfo dropped; `None` for anything else (a
/// relative path says nothing worth keeping and may carry a token).
fn url_origin(u: &str) -> Option<String> {
    let u = u.trim();
    let (prefix, rest) = if let Some(rest) = u.strip_prefix("//") {
        ("//".to_string(), rest)
    } else {
        let i = u.find("://")?;
        let scheme = &u[..i];
        if scheme.is_empty()
            || !scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
        {
            return None;
        }
        (u[..i + 3].to_ascii_lowercase(), &u[i + 3..])
    };
    let auth = &rest[..rest.find(['/', '?', '#']).unwrap_or(rest.len())];
    let host = &auth[auth.rfind('@').map_or(0, |at| at + 1)..];
    let ok = !host.is_empty()
        && host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-.:[]".contains(c));
    ok.then(|| format!("{prefix}{host}"))
}

/// The auth-scheme names of a `WWW-Authenticate` value (`Basic`, `Bearer`,
/// `Digest` …), every parameter dropped. A comma-separated piece starts a
/// new challenge when its first word is a bare token, not `name=value`.
fn auth_schemes(value: &str) -> String {
    let is_token = |w: &str| {
        !w.is_empty()
            && w.chars()
                .all(|c| c.is_ascii_alphanumeric() || "!#$%&'*+-.^_`|~".contains(c))
    };
    let schemes: Vec<&str> = value
        .split(',')
        .filter_map(|piece| piece.split_whitespace().next())
        .filter(|w| is_token(w))
        .collect();
    if schemes.is_empty() {
        REDACTED.to_string()
    } else {
        schemes.join(", ")
    }
}

/// The value a header may show in the committed `.headers`.
fn header_value(name: &str, value: &str) -> String {
    let name = name.to_ascii_lowercase();
    if reveals_location(&name) {
        LOCATION_REDACTED.to_string()
    } else if ORIGIN_HEADERS.contains(&name.as_str()) {
        url_origin(value).unwrap_or_else(|| REDACTED.to_string())
    } else if name == "www-authenticate" {
        auth_schemes(value)
    } else if SAFE_HEADERS.contains(&name.as_str()) {
        value.to_string()
    } else {
        REDACTED.to_string()
    }
}

/// A URL as fetch publishes it (the `.headers` `URL:` line, the log's
/// `final_url`): userinfo, query and fragment dropped — not a keyword
/// blacklist, since a token can sit under any parameter name.
fn public_url(u: &str) -> String {
    let u = u.trim();
    let u = &u[..u.find(['?', '#']).unwrap_or(u.len())];
    match split_authority(u) {
        Some((scheme, auth)) => {
            let host = &auth[auth.rfind('@').map_or(0, |at| at + 1)..];
            format!("{scheme}{host}{}", &u[scheme.len() + auth.len()..])
        }
        None => u.to_string(),
    }
}

/// Status line, final URL (`public_url`), then every response header by
/// name with its `header_value`.
fn headers_text(g: &Got) -> String {
    let mut s = format!("{}\nURL: {}\n", g.status_line, public_url(&g.final_url));
    for (k, v) in &g.headers {
        s.push_str(&format!("{k}: {}\n", header_value(k, v)));
    }
    s
}

// ---------------------------------------------------------------------------
// PDF text

/// A child process that is killed and reaped on every exit path.
struct Reaped(std::process::Child);

impl Drop for Reaped {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// `<tool> -enc UTF-8 -layout <pdf> -`, its stdout read as it comes with a
/// byte cap, the whole run bounded by `timeout`. Any failure — spawn, wait,
/// read, cap, timeout, exit status — kills and reaps the child (`Reaped`);
/// nothing is written to disk here.
fn pdf_to_text(tool: &Path, pdf: &Path, cap: u64, timeout: Duration) -> Result<Vec<u8>, String> {
    let mut child = Command::new(tool)
        .args(["-enc", "UTF-8", "-layout"])
        .arg(pdf)
        .arg("-")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => {
                "找不到 pdftotext（用 Scoop 装 poppler，或用 BARKING_POPPLER 指向其 bin 目录）"
                    .to_string()
            }
            _ => format!("无法运行 pdftotext（{e}）"),
        })?;
    let stdout = child.stdout.take();
    let mut child = Reaped(child);
    let Some(stdout) = stdout else {
        return Err("pdftotext 没有标准输出".into());
    };
    // The reader stops at cap + 1 bytes; past the cap the child then blocks
    // on a full pipe and is killed below, so neither side can run away.
    let reader = thread::spawn(move || {
        let mut out = Vec::new();
        stdout.take(cap + 1).read_to_end(&mut out).map(|_| out)
    });
    let started = Instant::now();
    let status = loop {
        if reader.is_finished() {
            // Either the child closed stdout (it is exiting) or the cap hit.
            break None;
        }
        match child.0.try_wait() {
            Ok(Some(s)) => break Some(s),
            Ok(None) if started.elapsed() > timeout => {
                return Err(format!(
                    "pdftotext 超过 {} 秒未结束，已终止",
                    timeout.as_secs()
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(20)),
            Err(e) => return Err(format!("等待 pdftotext 失败（{e}）")),
        }
    };
    let out = match reader.join() {
        Ok(Ok(out)) => out,
        Ok(Err(e)) => return Err(format!("读取 pdftotext 输出失败（{e}）")),
        Err(_) => return Err("读取 pdftotext 输出的线程异常退出".into()),
    };
    if out.len() as u64 > cap {
        return Err(format!(
            "pdftotext 输出超过 {} MB 上限，已终止并丢弃",
            cap / (1024 * 1024)
        ));
    }
    let status = match status {
        Some(s) => s,
        None => loop {
            match child.0.try_wait() {
                Ok(Some(s)) => break s,
                Ok(None) if started.elapsed() > timeout => {
                    return Err(format!(
                        "pdftotext 超过 {} 秒未结束，已终止",
                        timeout.as_secs()
                    ));
                }
                Ok(None) => thread::sleep(Duration::from_millis(20)),
                Err(e) => return Err(format!("等待 pdftotext 失败（{e}）")),
            }
        },
    };
    if !status.success() {
        return Err(format!("pdftotext 失败（{status}）"));
    }
    Ok(out)
}

/// Whether converter output holds any text at all: a scanned PDF without a
/// text layer comes back as blank lines and form feeds.
fn has_text(out: &[u8]) -> bool {
    !String::from_utf8_lossy(out).trim().is_empty()
}

// ---------------------------------------------------------------------------
// One item

/// What happened to one item, for the summary and the log.
#[derive(Debug)]
struct Outcome {
    status: Option<u16>,
    /// As received; scrubbed by `log_url` wherever it is written.
    final_url: Option<String>,
    content_type: Option<String>,
    /// Length of the (decoded) body received.
    bytes: Option<u64>,
    /// Of the body file written, so `None` when none was.
    sha256: Option<String>,
    /// File names written under `sources/`.
    outputs: Vec<String>,
    verdict: Verdict,
    /// Never an absolute local path: these reach the committed log.
    reasons: Vec<String>,
    txt_lines: Option<usize>,
}

impl Outcome {
    fn failed(reason: String) -> Outcome {
        Outcome {
            status: None,
            final_url: None,
            content_type: None,
            bytes: None,
            sha256: None,
            outputs: Vec::new(),
            verdict: Verdict::Failed,
            reasons: vec![reason],
            txt_lines: None,
        }
    }

    fn fail(&mut self, reason: String) {
        self.verdict = Verdict::Failed;
        self.reasons.push(reason);
    }
}

/// Everything an item needs besides itself. Shared by reference across the
/// pool's threads, hence only `Sync` fields (`ureq::Agent` is a handle onto
/// a thread-safe connection pool).
struct Job<'a> {
    issue: &'a Issue,
    group: &'a str,
    force: bool,
    net: Net,
    pdftotext: PathBuf,
}

impl Job<'_> {
    fn base(&self, item: &Item) -> String {
        format!("{}-{}", self.group, item.name)
    }

    fn run(&self, item: &Item) -> Outcome {
        let sources = self.issue.sources();
        let base = self.base(item);
        let (got, notes) = fetch_url(&self.net, &item.url);
        let mut out = match got {
            Ok(g) => self.archive(&sources, &base, g),
            Err(reason) => self.no_response(&sources, &base, item, reason),
        };
        out.reasons.splice(0..0, notes);
        out
    }

    /// `--force` clean-up before the first write of an item.
    fn prepare(&self, sources: &Path, base: &str, o: &mut Outcome) -> bool {
        if !self.force {
            return true;
        }
        match clear_base(sources, base) {
            Ok(()) => true,
            Err(e) => {
                o.fail(e);
                false
            }
        }
    }

    /// No response at all: only a `.headers` saying so.
    fn no_response(&self, sources: &Path, base: &str, item: &Item, reason: String) -> Outcome {
        let mut o = Outcome::failed(reason);
        if !self.prepare(sources, base, &mut o) {
            return o;
        }
        let text = format!(
            "# 无响应：{}\nURL: {}\n",
            o.reasons[0],
            public_url(&item.url)
        );
        let file = format!("{base}.headers");
        match put(sources, &file, text.as_bytes()) {
            Ok(()) => o.outputs.push(file),
            Err(e) => o.reasons.push(e),
        }
        o
    }

    /// Judges one response and writes its files: `.headers` first (if that
    /// fails, nothing else is written), then the body, then — only for an ok
    /// item — the text archive.
    fn archive(&self, sources: &Path, base: &str, g: Got) -> Outcome {
        let ext = choose_ext(&g.content_type, &g.body);
        let mut verdict = status_verdict(g.status);
        let mut reasons: Vec<String> = Vec::new();
        if verdict != Verdict::Ok {
            reasons.push(g.status_line.clone());
        } else if g.body.is_empty() {
            verdict = Verdict::Failed;
            reasons.push(format!("{}，但正文为空", g.status_line));
        }
        if verdict != Verdict::Failed
            && let Some(why) = challenge_header(&g.headers)
        {
            verdict = Verdict::Blocked;
            reasons.push(why);
        }
        let decoded = matches!(ext, "html" | "txt")
            .then(|| decode_body(&g.content_type, &g.body, ext == "html"));
        if let Some(Ok(d)) = &decoded
            && d.enc != UTF_8
        {
            reasons.push(format!("按 {} 解码", d.enc.name()));
        }
        let page = match &decoded {
            Some(Ok(d)) if ext == "html" => Some(d.text.trim_start_matches('\u{feff}')),
            _ => None,
        };
        // Markup is matched on the decoded page, so a UTF-16 challenge page
        // is caught as surely as a UTF-8 one.
        if verdict != Verdict::Failed
            && let Some(why) = page.and_then(challenge_markup)
        {
            verdict = Verdict::Blocked;
            reasons.push(why);
        }
        let html_text = page.map(html_to_text);
        if verdict == Verdict::Ok {
            if let Some(Err(e)) = &decoded {
                verdict = Verdict::Failed;
                reasons.push(format!("{e}，未生成 txt"));
            } else if let (Some(p), Some(t)) = (page, html_text.as_deref())
                && let Some(why) = detect_gate(p, t)
            {
                verdict = Verdict::Blocked;
                reasons.push(why);
            }
        }
        // `.txt` is reserved for usable text: a plain-text body takes that
        // name only when it is an ok item that is valid UTF-8 as it stands;
        // otherwise the raw bytes are `.bin` (and an ok item gets its decoded
        // `.txt`).
        let as_is = matches!(&decoded, Some(Ok(d)) if d.utf8_as_is);
        let body_ext = match ext {
            "txt" if verdict == Verdict::Ok && as_is => "txt",
            "txt" => "bin",
            e => e,
        };

        let mut o = Outcome {
            status: Some(g.status),
            final_url: Some(g.final_url.clone()),
            content_type: (!g.content_type.is_empty()).then(|| g.content_type.clone()),
            bytes: Some(g.body.len() as u64),
            sha256: None,
            outputs: Vec::new(),
            verdict,
            reasons,
            txt_lines: None,
        };
        if !self.prepare(sources, base, &mut o) {
            return o;
        }
        let headers_file = format!("{base}.headers");
        if let Err(e) = put(sources, &headers_file, headers_text(&g).as_bytes()) {
            o.fail(e);
            return o;
        }
        o.outputs.push(headers_file);
        let body_file = format!("{base}.{body_ext}");
        if !g.body.is_empty() {
            if let Err(e) = put(sources, &body_file, &g.body) {
                o.fail(e);
                return o;
            }
            o.outputs.push(body_file.clone());
            o.sha256 = Some(sha256_hex(&g.body));
        }
        if o.verdict != Verdict::Ok {
            return o;
        }

        let txt = format!("{base}.txt");
        let text: Option<Vec<u8>> = match (ext, &decoded) {
            ("html", _) => html_text.map(String::into_bytes),
            ("txt", Some(Ok(d))) if body_ext == "bin" => Some(d.text.clone().into_bytes()),
            ("txt", Some(Ok(d))) => {
                o.txt_lines = Some(d.text.lines().count());
                None
            }
            ("pdf", _) => {
                match pdf_to_text(
                    &self.pdftotext,
                    &sources.join(&body_file),
                    MAX_PDF_TEXT,
                    PDFTOTEXT_TIMEOUT,
                ) {
                    Ok(out) if has_text(&out) => Some(out),
                    Ok(_) => {
                        o.fail(
                            "PDF 已保存，但 PDF 无可提取文本（可能是扫描件），未生成 txt".into(),
                        );
                        None
                    }
                    Err(e) => {
                        o.fail(format!("PDF 已保存，但未生成 txt：{e}"));
                        None
                    }
                }
            }
            _ => None,
        };
        if let Some(text) = text {
            match put(sources, &txt, &text) {
                Ok(()) => {
                    o.outputs.push(txt);
                    o.txt_lines = Some(String::from_utf8_lossy(&text).lines().count());
                }
                Err(e) => o.fail(e),
            }
        }
        o
    }
}

fn log_line(issue: &Issue, item: &Item, o: &Outcome) -> String {
    let root = &issue.root;
    let rel = issue.rel();
    let outputs: Vec<String> = o
        .outputs
        .iter()
        .map(|f| format!("{rel}/sources/{f}"))
        .collect();
    jsonl(&[
        ("time_bj", Value::from(repo::beijing_now())),
        ("name", Value::from(item.name.as_str())),
        ("url", Value::from(log_url(root, &item.url))),
        (
            "final_url",
            Value::from(o.final_url.as_deref().map(public_url)),
        ),
        ("status", Value::from(o.status)),
        ("content_type", Value::from(o.content_type.clone())),
        ("bytes", Value::from(o.bytes)),
        ("sha256", Value::from(o.sha256.clone())),
        ("outputs", Value::from(outputs)),
        ("result", Value::from(o.verdict.as_str())),
        ("reason", Value::from(o.reasons.join("；"))),
    ])
}

// ---------------------------------------------------------------------------
// Entry point

fn usage_error(msg: &str) -> ExitCode {
    eprintln!("error: {msg}\n{USAGE}");
    ExitCode::from(2)
}

fn read_manifest(arg: &str) -> Result<String, String> {
    let bytes = if arg == "-" {
        let mut b = Vec::new();
        io::stdin()
            .read_to_end(&mut b)
            .map_err(|e| format!("读取标准输入失败：{e}"))?;
        b
    } else {
        fs::read(arg).map_err(|e| format!("{arg}: {e}"))?
    };
    String::from_utf8(bytes).map_err(|_| format!("{arg}: 清单不是 UTF-8"))
}

fn truncate_chars(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((i, _)) => format!("{}…", &s[..i]),
        None => s.to_string(),
    }
}

/// The table printed at the end, in manifest order. Column headers are
/// ASCII so that the columns line up in any terminal font.
fn summary(items: &[Item], outcomes: &[Outcome]) -> String {
    let w = items.iter().map(|i| i.name.len()).max().unwrap_or(4).max(4);
    let dash = || "-".to_string();
    let mut s = format!(
        "{:<w$}  {:>6}  {:<7}  {:>10}  {:>9}  note\n",
        "name", "status", "result", "bytes", "txt_lines"
    );
    for (item, o) in items.iter().zip(outcomes) {
        let row = format!(
            "{:<w$}  {:>6}  {:<7}  {:>10}  {:>9}  {}",
            item.name,
            o.status.map_or_else(dash, |n| n.to_string()),
            o.verdict.as_str(),
            o.bytes.map_or_else(dash, |n| n.to_string()),
            o.txt_lines.map_or_else(dash, |n| n.to_string()),
            truncate_chars(&o.reasons.join("；"), 160),
        );
        s.push_str(row.trim_end());
        s.push('\n');
    }
    s
}

pub fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    let a = match parse_args(args) {
        Ok(a) => a,
        Err(e) => return usage_error(&e),
    };
    let items = match read_manifest(&a.manifest).and_then(|t| parse_manifest(&t)) {
        Ok(i) => i,
        Err(e) => return usage_error(&e),
    };
    let issue = match Issue::resolve(&a.issue_dir) {
        Ok(i) => i,
        Err(e) => return usage_error(&e),
    };
    let sources = issue.sources();
    if !a.force {
        // Before any request: a batch half-written over old files would
        // mix two moments in time.
        let taken: Vec<String> = items
            .iter()
            .flat_map(|it| existing_outputs(&sources, &format!("{}-{}", a.group, it.name)))
            .collect();
        if !taken.is_empty() {
            eprintln!(
                "error: {}/sources/ 下已有 {}，拒绝覆盖（从清单删去这些条目、换 name，或加 --force）",
                issue.rel(),
                taken.join("、")
            );
            return ExitCode::FAILURE;
        }
    }
    if let Err(e) = fs::create_dir_all(&sources) {
        eprintln!("error: {}: {e}", sources.display());
        return ExitCode::FAILURE;
    }
    let log_name = format!("{}-fetch-log.jsonl", a.group);
    let log_shown = format!("{}/sources/{log_name}", issue.rel());
    // Opened before any request: a batch whose record cannot be kept is not
    // worth fetching.
    let log = match open_log(&sources.join(&log_name)) {
        Ok(f) => Mutex::new(f),
        Err(e) => {
            eprintln!("error: 无法打开日志 {log_shown}：{e}");
            return ExitCode::FAILURE;
        }
    };

    let job = Job {
        issue: &issue,
        group: &a.group,
        force: a.force,
        net: Net::new(),
        pdftotext: pick_tool(&poppler_dirs(), "pdftotext"),
    };
    // Appends that failed, reported at the end.
    let log_errors: Mutex<Vec<String>> = Mutex::new(Vec::new());
    println!(
        "取得 {} 条（并发 {}，同一清单域名至多 {PER_HOST}）；开始于 {}",
        items.len(),
        a.jobs.min(items.len()),
        repo::beijing_now()
    );
    let started = Instant::now();
    let outcomes = run_pool(&host_ids(&items), a.jobs, PER_HOST, |i| {
        let item = &items[i];
        // A bug in one item must not take the batch (or the log) with it.
        let o = panic::catch_unwind(AssertUnwindSafe(|| job.run(item)))
            .unwrap_or_else(|_| Outcome::failed("内部错误（线程 panic，见上方输出）".into()));
        let line = format!("{}\n", log_line(&issue, item, &o));
        let mut f = log.lock().unwrap_or_else(PoisonError::into_inner);
        if let Err(e) = f.write_all(line.as_bytes()) {
            log_errors
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(format!("{}：{e}", item.name));
        }
        o
    });

    print!("{}", summary(&items, &outcomes));
    let count = |v: Verdict| outcomes.iter().filter(|o| o.verdict == v).count();
    let (ok, blocked, failed) = (
        count(Verdict::Ok),
        count(Verdict::Blocked),
        count(Verdict::Failed),
    );
    println!(
        "-- ok {ok}，blocked {blocked}，failed {failed}；用时 {} 秒；输出 {rel}/sources/{g}-<name>.*，日志 {log_shown}",
        started.elapsed().as_secs(),
        rel = issue.rel(),
        g = a.group,
    );
    if blocked + failed > 0 {
        println!(
            "   blocked / failed 条目没有 .txt：改用 opencli 或浏览器取证，或修正 URL 后单独重跑（加 --force 会先删去该条旧输出）"
        );
    }
    let log_errors = log_errors
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner);
    if !log_errors.is_empty() {
        eprintln!(
            "error: {} 条未能写入日志 {log_shown}（{}）；文件已写出，日志须手工补记",
            log_errors.len(),
            log_errors.join("；")
        );
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    fn scratch_root(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("barking-fetch-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A fake repository in the temp directory: tests never touch `docs/`.
    fn test_issue(tag: &str) -> Issue {
        let root = scratch_root(tag);
        let dir = root.join("docs/2610/1015");
        fs::create_dir_all(dir.join("sources")).unwrap();
        Issue {
            root,
            dir,
            yymm: "2610".into(),
            mmdd: "1015".into(),
        }
    }

    fn got(status: u16, content_type: &str, body: &[u8]) -> Got {
        Got {
            status,
            status_line: format!("HTTP/1.1 {status}"),
            final_url: "https://user:pw@example.com/a?token=abc&q=1".into(),
            content_type: content_type.into(),
            headers: vec![
                ("content-type".into(), content_type.into()),
                (
                    "set-cookie".into(),
                    "__cf_bm=SECRETVALUE; path=/; HttpOnly".into(),
                ),
            ],
            body: body.to_vec(),
        }
    }

    /// Loopback-only client: no proxy, small body limit, no retry pause.
    fn test_net(max_body: u64) -> Net {
        Net {
            agent: http_agent(false),
            max_body,
            retry_pause: Duration::ZERO,
            page_timeout: PAGE_TIMEOUT,
            pdf_timeout: PDF_TIMEOUT,
        }
    }

    /// A stand-in for pdftotext: a script that prints `body` (a shell /
    /// batch fragment) to stdout, whatever its arguments.
    fn fake_converter(dir: &Path, name: &str, body: &str) -> PathBuf {
        #[cfg(windows)]
        {
            let p = dir.join(format!("{name}.cmd"));
            fs::write(&p, format!("@echo off\r\n{body}\r\n")).unwrap();
            p
        }
        #[cfg(not(windows))]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let p = dir.join(name);
            fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
            fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
            p
        }
    }

    fn job(issue: &Issue, force: bool) -> Job<'_> {
        Job {
            issue,
            group: "a",
            force,
            net: test_net(MAX_BODY),
            pdftotext: issue.root.join("no-such-pdftotext"),
        }
    }

    const ARTICLE: &str = "<html><head><title>Example Domain</title></head><body><h1>Example Domain</h1>\
        <p>This domain is for use in documentation examples without needing permission. \
        Avoid use in operations.</p><p><a href=\"https://iana.org/domains/example\">Learn more</a></p></body></html>";

    /// 20 000 × `a`, gzip-compressed to 56 bytes.
    const GZIP_20K: [u8; 56] = [
        31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 237, 193, 49, 1, 0, 0, 0, 194, 160, 172, 235, 95, 194,
        26, 30, 64, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 129, 1, 49, 135,
        189, 202, 32, 78, 0, 0,
    ];

    /// A loopback HTTP server that answers each connection, in order, with
    /// the next canned response and records every request head.
    fn serve(responses: Vec<Vec<u8>>) -> (String, Arc<Mutex<Vec<String>>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let seen = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::clone(&seen);
        thread::spawn(move || {
            for resp in responses {
                let Ok((mut stream, _)) = listener.accept() else {
                    return;
                };
                let mut head = Vec::new();
                let mut buf = [0u8; 1024];
                while !head.windows(4).any(|w| w == b"\r\n\r\n") {
                    match stream.read(&mut buf) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => head.extend_from_slice(&buf[..n]),
                    }
                }
                log.lock()
                    .unwrap()
                    .push(String::from_utf8_lossy(&head).into_owned());
                let _ = stream.write_all(&resp);
            }
        });
        (url, seen)
    }

    fn http(status: &str, headers: &[(&str, &str)], body: &[u8]) -> Vec<u8> {
        let mut out = format!("HTTP/1.1 {status}\r\n");
        for (k, v) in headers {
            out.push_str(&format!("{k}: {v}\r\n"));
        }
        out.push_str(&format!(
            "Content-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        ));
        let mut out = out.into_bytes();
        out.extend_from_slice(body);
        out
    }

    fn no_authorization(seen: &Arc<Mutex<Vec<String>>>) -> bool {
        seen.lock()
            .unwrap()
            .iter()
            .all(|r| !r.to_ascii_lowercase().contains("authorization"))
    }

    // ---- command line and manifest ----

    #[test]
    fn args_parse_flags_and_positionals() {
        let a = parse_args(&s(&["docs/2610/1015", "a", "list.tsv"])).unwrap();
        assert_eq!(a.group, "a");
        assert_eq!(a.manifest, "list.tsv");
        assert_eq!((a.jobs, a.force), (DEFAULT_JOBS, false));
        let a = parse_args(&s(&["d", "b2", "-", "--jobs", "3", "--force"])).unwrap();
        assert_eq!((a.manifest.as_str(), a.jobs, a.force), ("-", 3, true));
        assert_eq!(
            parse_args(&s(&["d", "b", "-", "--jobs=16"])).unwrap().jobs,
            16
        );
        for bad in [
            s(&["d", "b", "-", "--jobs", "0"]),
            s(&["d", "b", "-", "--jobs=17"]),
            s(&["d", "b", "-", "--jobs"]),
            s(&["d", "b", "-", "--nope"]),
            s(&["d", "b"]),
            s(&["d", "B", "-"]),
            s(&["d", "-b", "-"]),
            s(&["d", "b", "-", "extra"]),
        ] {
            assert!(parse_args(&bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn manifest_skips_comments_and_blank_lines() {
        let text = "\u{feff}# group A\r\n\r\nhome\thttps://Example.COM/\r\n  paper\thttps://www.rfc-editor.org/rfc/rfc9110.pdf  \n#x\thttp://no\n";
        let items = parse_manifest(text).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "home");
        assert_eq!(items[0].url, "https://Example.COM/");
        assert_eq!(items[0].host, "example.com");
        assert_eq!(items[1].host, "www.rfc-editor.org");
    }

    #[test]
    fn manifest_rejects_bad_lines_all_at_once() {
        let text = "ok\thttps://a.com/\nno-tab https://b.com/\nBad\thttps://c.com/\nftp\tftp://d.com/\nok\thttps://e.com/\n";
        let e = parse_manifest(text).unwrap_err();
        for want in ["第 2 行", "第 3 行", "第 4 行", "第 5 行：name「ok」重复"] {
            assert!(e.contains(want), "{want} not in {e}");
        }
        assert!(!e.contains("第 1 行"));
        assert_eq!(parse_manifest("# only\n\n").unwrap_err(), "清单为空");
        assert!(parse_manifest("x\thttps://\n").is_err());
        assert!(parse_manifest(&format!("{}\thttps://a.com/\n", "a".repeat(61))).is_err());
    }

    #[test]
    fn manifest_refuses_userinfo_without_echoing_it() {
        for url in [
            "https://user:s3cret@example.com/",
            "https://token@example.com/x",
        ] {
            let e = parse_manifest(&format!("x\t{url}\n")).unwrap_err();
            assert!(e.contains("user:pass@"), "{e}");
            assert!(!e.contains("s3cret") && !e.contains("token@"), "{e}");
        }
        // An `@` in the path is not userinfo.
        assert!(parse_manifest("x\thttps://example.com/@someone\n").is_ok());
    }

    #[test]
    fn manifest_encodes_non_ascii_paths() {
        let items = parse_manifest(
            "cn\thttps://Example.com/新闻/a b?q=中文#段\nkept\thttps://x.com/a%20b?q=%E4%B8%AD\n",
        )
        .unwrap();
        assert_eq!(
            items[0].url,
            "https://Example.com/%E6%96%B0%E9%97%BB/a%20b?q=%E4%B8%AD%E6%96%87#%E6%AE%B5"
        );
        assert_eq!(items[0].host, "example.com");
        assert_eq!(items[1].url, "https://x.com/a%20b?q=%E4%B8%AD");
        assert_eq!(encode_url("https://x.com"), "https://x.com");
        let e = parse_manifest("idn\thttps://例子.中国/\n").unwrap_err();
        assert!(e.contains("punycode"), "{e}");
    }

    #[test]
    fn hosts_are_dense_ids_in_first_seen_order() {
        let items = parse_manifest(
            "a\thttps://x.com/1\nb\thttps://y.com/\nc\thttps://X.com/2\nd\thttps://z.com/\n",
        )
        .unwrap();
        assert_eq!(host_ids(&items), vec![0, 1, 0, 2]);
    }

    // ---- scheduling ----

    #[test]
    fn limiter_skips_hosts_at_cap_and_keeps_order() {
        let host_of = [0, 0, 1, 0, 2];
        assert_eq!(
            next_runnable(&[1, 2, 3, 4], &host_of, &[2, 0, 0], 2),
            Some(1)
        );
        assert_eq!(
            next_runnable(&[1, 2, 3, 4], &host_of, &[1, 0, 0], 2),
            Some(0)
        );
        assert_eq!(next_runnable(&[0, 1, 3], &host_of, &[2, 0, 0], 2), None);
        assert_eq!(next_runnable(&[], &host_of, &[0, 0, 0], 2), None);
    }

    #[test]
    fn pool_respects_both_limits_and_returns_in_order() {
        let host_of = [0, 0, 0, 0, 0, 1, 1, 2, 3, 3];
        let per_host: Vec<AtomicUsize> = (0..4).map(|_| AtomicUsize::new(0)).collect();
        let peak_host: Vec<AtomicUsize> = (0..4).map(|_| AtomicUsize::new(0)).collect();
        let (total, peak_total) = (AtomicUsize::new(0), AtomicUsize::new(0));
        let out = run_pool(&host_of, 4, 2, |i| {
            let h = host_of[i];
            let now = per_host[h].fetch_add(1, Ordering::SeqCst) + 1;
            peak_host[h].fetch_max(now, Ordering::SeqCst);
            let t = total.fetch_add(1, Ordering::SeqCst) + 1;
            peak_total.fetch_max(t, Ordering::SeqCst);
            // Later items finish first, so completion order differs from
            // manifest order.
            thread::sleep(Duration::from_millis(40 - 3 * i as u64));
            per_host[h].fetch_sub(1, Ordering::SeqCst);
            total.fetch_sub(1, Ordering::SeqCst);
            i * 10
        });
        assert_eq!(out, (0..10).map(|i| i * 10).collect::<Vec<_>>());
        for p in &peak_host {
            assert!(p.load(Ordering::SeqCst) <= 2);
        }
        assert_eq!(
            peak_host[0].load(Ordering::SeqCst),
            2,
            "host 0 should overlap"
        );
        assert!(peak_total.load(Ordering::SeqCst) <= 4);
    }

    #[test]
    fn pool_releases_the_host_of_a_panicking_job() {
        // Item 0 panics while holding host 0; the other host-0 items must
        // still run (a leaked slot would block them forever), and the panic
        // reaches the caller after every thread has finished.
        let ran = AtomicUsize::new(0);
        let r = panic::catch_unwind(AssertUnwindSafe(|| {
            run_pool(&[0, 0, 0], 2, 1, |i| {
                if i == 0 {
                    panic!("boom");
                }
                ran.fetch_add(1, Ordering::SeqCst);
            })
        }));
        assert!(r.is_err());
        assert_eq!(ran.load(Ordering::SeqCst), 2);
        assert!(run_pool(&[], 6, 2, |i: usize| i).is_empty());
    }

    // ---- fetching, against a loopback server ----

    #[test]
    fn redirects_are_followed_relative_and_without_authorization() {
        let (url, seen) = serve(vec![
            http("302 Found", &[("Location", "/next?x=1")], b""),
            http("301 Moved", &[("Location", "deeper/page")], b""),
            http(
                "200 OK",
                &[("Content-Type", "text/html")],
                ARTICLE.as_bytes(),
            ),
        ]);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &format!("{url}/start"));
        let g = g.unwrap();
        assert_eq!(g.status, 200);
        assert_eq!(g.final_url, format!("{url}/deeper/page"));
        assert!(notes.is_empty());
        let reqs = seen.lock().unwrap().clone();
        assert_eq!(reqs.len(), 3);
        assert!(reqs[1].starts_with("GET /next?x=1 "));
        assert!(reqs[2].starts_with("GET /deeper/page "));
        assert!(no_authorization(&seen));
    }

    #[test]
    fn redirect_to_userinfo_is_refused_and_never_requested() {
        let (url, seen) = serve(vec![http(
            "302 Found",
            &[("Location", "http://user:pw@127.0.0.1:9/x")],
            b"",
        )]);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &url);
        let e = g.err().unwrap();
        assert!(e.contains("用户名或密码") && !e.contains("pw"), "{e}");
        assert!(notes.is_empty(), "not retried");
        assert_eq!(seen.lock().unwrap().len(), 1);
        assert!(no_authorization(&seen));
    }

    #[test]
    fn redirect_loops_stop_without_retry() {
        let loops = (0..=MAX_REDIRECTS)
            .map(|_| http("302 Found", &[("Location", "/loop")], b""))
            .collect();
        let (url, seen) = serve(loops);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &url);
        assert!(g.err().unwrap().contains("跳转超过"));
        assert!(notes.is_empty());
        assert_eq!(seen.lock().unwrap().len(), MAX_REDIRECTS + 1);
    }

    #[test]
    fn body_limit_counts_decoded_bytes() {
        let gz = || {
            http(
                "200 OK",
                &[("Content-Type", "text/plain"), ("Content-Encoding", "gzip")],
                &GZIP_20K,
            )
        };
        // 56 bytes on the wire, 20 000 decoded: over a 1 000-byte limit.
        let (url, seen) = serve(vec![gz()]);
        let (g, notes) = fetch_url(&test_net(1000), &url);
        assert!(g.err().unwrap().contains("上限"));
        assert!(notes.is_empty(), "a size refusal is not retried");
        assert_eq!(seen.lock().unwrap().len(), 1);
        // Within the limit the decoded body is what is kept.
        let (url, _) = serve(vec![gz()]);
        let g = fetch_url(&test_net(100_000), &url).0.unwrap();
        assert_eq!(g.body, vec![b'a'; 20_000]);
    }

    #[test]
    fn retry_only_for_5xx_and_transport_errors() {
        let (url, seen) = serve(vec![
            http("503 Service Unavailable", &[], b"busy"),
            http("503 Service Unavailable", &[], b"busy"),
        ]);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &url);
        assert_eq!(g.unwrap().status, 503);
        assert_eq!(notes.len(), 1);
        assert_eq!(seen.lock().unwrap().len(), 2);

        let (url, seen) = serve(vec![http("404 Not Found", &[], b"gone")]);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &url);
        assert_eq!(g.unwrap().status, 404);
        assert!(notes.is_empty());
        assert_eq!(seen.lock().unwrap().len(), 1);

        // Not HTTP at all: a protocol error, final at once.
        let (url, _) = serve(vec![b"SSH-2.0-OpenSSH\r\n\r\n".to_vec()]);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &url);
        assert!(g.is_err());
        assert!(notes.is_empty(), "{notes:?}");

        assert!(retryable(&ureq::Error::HostNotFound));
        assert!(retryable(&ureq::Error::Io(io::Error::from(
            ErrorKind::ConnectionReset
        ))));
        assert!(!retryable(&ureq::Error::Io(io::Error::from(
            ErrorKind::InvalidData
        ))));
        assert!(!retryable(&ureq::Error::TooManyRedirects));
    }

    #[test]
    fn pdf_type_at_page_url_gets_the_pdf_budget() {
        let pdf = || {
            http(
                "200 OK",
                &[("Content-Type", "application/pdf")],
                b"%PDF-1.7\n",
            )
        };
        let (url, seen) = serve(vec![pdf(), pdf()]);
        let (g, notes) = fetch_url(&test_net(MAX_BODY), &format!("{url}/pdf/2401.00001"));
        assert_eq!(g.unwrap().body, b"%PDF-1.7\n");
        assert!(notes[0].contains("180 秒"));
        assert_eq!(seen.lock().unwrap().len(), 2);
    }

    #[test]
    fn locations_resolve_against_the_current_url() {
        let base = "https://a.com/x/y/page?q=1#f";
        assert_eq!(resolve(base, "https://b.com/z"), "https://b.com/z");
        assert_eq!(resolve(base, "//c.com/w"), "https://c.com/w");
        assert_eq!(resolve(base, "/root"), "https://a.com/root");
        assert_eq!(resolve(base, "?p=2"), "https://a.com/x/y/page?p=2");
        assert_eq!(resolve(base, "next"), "https://a.com/x/y/next");
        assert_eq!(resolve("https://a.com", "next"), "https://a.com/next");
        assert!(next_url(base, "ftp://a.com/").is_err());
        assert!(next_url(base, "//u:p@c.com/").is_err());
        assert_eq!(next_url(base, "/新").unwrap(), "https://a.com/%E6%96%B0");
    }

    #[test]
    fn dot_segments_are_removed() {
        // RFC 3986 §5.2.4 and §5.4 examples (base http://a/b/c/d;p?q).
        assert_eq!(remove_dot_segments("/a/b/c/./../../g"), "/a/g");
        assert_eq!(remove_dot_segments("mid/content=5/../6"), "mid/6");
        assert_eq!(remove_dot_segments("/a/./b/../c"), "/a/c");
        assert_eq!(remove_dot_segments("/.."), "/");
        let base = "http://a/b/c/d;p?q";
        for (loc, want) in [
            ("../g", "http://a/b/g"),
            ("./g", "http://a/b/c/g"),
            ("../..", "http://a/"),
            ("../../../g", "http://a/g"),
            ("/./g", "http://a/g"),
            ("g/./h", "http://a/b/c/g/h"),
            ("g;x=1/../y", "http://a/b/c/y"),
            (".", "http://a/b/c/"),
            ("/a/./b/../c", "http://a/a/c"),
            ("http://a/x/../y?z=1", "http://a/y?z=1"),
        ] {
            assert_eq!(resolve(base, loc), want, "{loc}");
        }
    }

    #[test]
    fn one_deadline_covers_retry_and_redirects() {
        // A server that accepts and never answers: the first try uses the
        // whole item budget, so the retry has nothing left.
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        thread::spawn(move || {
            let mut held = Vec::new();
            while let Ok((s, _)) = listener.accept() {
                held.push(s);
            }
        });
        let mut net = test_net(MAX_BODY);
        net.page_timeout = Duration::from_millis(400);
        net.retry_pause = Duration::from_millis(300);
        let started = Instant::now();
        let (g, notes) = fetch_url(&net, &url);
        let took = started.elapsed();
        assert!(g.is_err());
        assert_eq!(notes.len(), 1, "{notes:?}");
        assert!(
            took < Duration::from_millis(700),
            "item took {took:?}, more than one budget"
        );
    }

    // ---- classification ----

    #[test]
    fn status_classes() {
        assert_eq!(status_verdict(200), Verdict::Ok);
        assert_eq!(status_verdict(203), Verdict::Ok);
        for b in BLOCKED_STATUS {
            assert_eq!(status_verdict(b), Verdict::Blocked);
        }
        for f in [404, 410, 451, 500, 503, 304] {
            assert_eq!(status_verdict(f), Verdict::Failed);
        }
    }

    #[test]
    fn gates_detected_on_short_pages() {
        let cf = "<html><head><title>Just a moment...</title></head><body>\
            <noscript>Enable JavaScript and cookies to continue</noscript>\
            <p>www.example.org needs to review the security of your connection before proceeding.</p></body></html>";
        let text = html_to_text(cf);
        assert!(
            detect_gate(cf, &text)
                .unwrap()
                .contains("Cloudflare 验证页")
        );

        let akamai = "<html><head><title>Access Denied</title></head><body><h1>Access Denied</h1>\
            You don't have permission to access \"http://www.example.com/\" on this server.<p>Reference #18.1234</p></body></html>";
        assert!(
            detect_gate(akamai, &html_to_text(akamai))
                .unwrap()
                .contains("拒绝访问页")
        );

        let captcha = format!(
            "<html><body><p>{}</p><div class=\"g-recaptcha\" data-sitekey=\"x\"></div></body></html>",
            "Please complete the form below so that we can make sure that you may continue to the site you requested."
        );
        assert!(
            detect_gate(&captcha, &html_to_text(&captcha))
                .unwrap()
                .contains("reCAPTCHA")
        );

        let shell = "<html><body><div id=\"root\"></div><script>app()</script></body></html>";
        assert!(
            detect_gate(shell, &html_to_text(shell))
                .unwrap()
                .contains("正文过短")
        );
    }

    #[test]
    fn real_pages_are_not_gates() {
        assert_eq!(detect_gate(ARTICLE, &html_to_text(ARTICLE)), None);
        let para = "<p>The court found that the agency had been granted access to the logs, \
            and the plaintiffs argued that access denied to journalists was unlawful.</p>";
        let long = format!(
            "<html><head><title>Ruling</title></head><body>{}<div class=\"g-recaptcha\"></div></body></html>",
            para.repeat(20)
        );
        assert_eq!(detect_gate(&long, &html_to_text(&long)), None);
    }

    #[test]
    fn challenge_signals_apply_at_any_length() {
        let para = "<p>A long and perfectly ordinary article paragraph about the case.</p>";
        let long = |extra: &str| {
            format!(
                "<html><head>{extra}</head><body>{}</body></html>",
                para.repeat(60)
            )
        };
        let h = |k: &str, v: &str| vec![(k.to_string(), v.to_string())];
        assert!(
            challenge_header(&h("CF-Mitigated", "challenge"))
                .unwrap()
                .contains("cf-mitigated")
        );
        assert_eq!(challenge_header(&h("cf-cache-status", "HIT")), None);
        let chl = long("<script>window._cf_chl_opt={cvId:'3'}</script>");
        assert!(challenge_markup(&chl).is_some());
        let orchestrate = long(
            "<script src=\"/cdn-cgi/challenge-platform/h/b/orchestrate/chl_page/v1\"></script>",
        );
        assert!(challenge_markup(&orchestrate).is_some());
        // Bot-detection script on an ordinary page: not a challenge.
        let jsd = long("<script src=\"/cdn-cgi/challenge-platform/scripts/jsd/main.js\"></script>");
        assert_eq!(challenge_markup(&jsd), None);

        // Through `archive`: a long 200 page with the header is blocked.
        let issue = test_issue("cfmit");
        let page = long("");
        let mut g = got(200, "text/html", page.as_bytes());
        g.headers.push(("cf-mitigated".into(), "challenge".into()));
        let o = job(&issue, false).archive(&issue.sources(), "a-cf", g);
        assert_eq!(o.verdict, Verdict::Blocked);
        assert!(!issue.sources().join("a-cf.txt").exists());
        // A UTF-16 page: the marker is only visible after decoding.
        let mut utf16 = vec![0xff, 0xfe];
        for u in chl.encode_utf16() {
            utf16.extend_from_slice(&u.to_le_bytes());
        }
        assert!(!String::from_utf8_lossy(&utf16).contains("_cf_chl_opt"));
        let o =
            job(&issue, false).archive(&issue.sources(), "a-u16", got(200, "text/html", &utf16));
        assert_eq!(o.verdict, Verdict::Blocked, "{:?}", o.reasons);
        assert!(o.reasons.iter().any(|r| r.contains("_cf_chl_opt")));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn extension_from_magic_then_type() {
        let html = b"<!DOCTYPE html><html><head></head></html>";
        let cases: [(&str, &[u8], &str); 17] = [
            ("text/html; charset=utf-8", html, "html"),
            ("application/xhtml+xml", html, "html"),
            ("application/octet-stream", b"%PDF-1.7\n...", "pdf"),
            ("text/html", b"\r\n%PDF-1.4 junk first", "pdf"),
            ("application/pdf", b"%PDF-1.5", "pdf"),
            ("application/pdf", html, "html"),
            ("application/pdf", b"\x00\x01binary", "pdf"),
            ("", html, "html"),
            ("application/octet-stream", b"\x00\x01", "bin"),
            ("APPLICATION/JSON", b"{}", "json"),
            ("application/ld+json", b"{}", "json"),
            ("application/rss+xml", b"<rss/>", "xml"),
            ("image/svg+xml", b"<svg/>", "svg"),
            ("text/plain; charset=utf-8", b"hello", "txt"),
            ("image/png", b"\x89PNG\r\n\x1a\nrest", "png"),
            ("", b"\xff\xd8\xff\xe0", "jpg"),
            ("", b"RIFF\x00\x00\x00\x00WEBPVP8 ", "webp"),
        ];
        for (ct, body, want) in cases {
            let got = choose_ext(ct, body);
            assert_eq!(got, want, "{ct}");
            assert!(OUTPUT_EXTS.contains(&got), "{got} missing from OUTPUT_EXTS");
        }
    }

    #[test]
    fn pdf_url_by_path_only() {
        assert!(is_pdf_url("https://x.org/a/b.PDF"));
        assert!(is_pdf_url("https://x.org/b.pdf?download=1#page=2"));
        assert!(!is_pdf_url("https://arxiv.org/pdf/2401.00001"));
        assert!(!is_pdf_url("https://x.org/view?file=b.pdf.html"));
    }

    // ---- charsets ----

    #[test]
    fn charset_from_header_then_meta_then_utf8() {
        assert_eq!(
            ct_charset("text/html; Charset=\"GB2312\"").as_deref(),
            Some("GB2312")
        );
        assert_eq!(ct_charset("text/html"), None);
        assert_eq!(
            meta_charset(b"<html><head><META charset='gbk'>").as_deref(),
            Some("gbk")
        );
        assert_eq!(
            meta_charset(b"<meta http-equiv=\"Content-Type\" content=\"text/html; charset=big5\">")
                .as_deref(),
            Some("big5")
        );
        assert_eq!(meta_charset(b"<meta name=\"x\" content=\"y\">"), None);
        // A commented-out declaration is not one.
        assert_eq!(
            meta_charset(b"<!-- <meta charset=\"gbk\"> --><meta charset=\"big5\">").as_deref(),
            Some("big5")
        );
        assert_eq!(meta_charset(b"<!-- <meta charset=\"gbk\"> -->"), None);

        let (gbk, _, _) = encoding_rs::GBK.encode("中文新闻");
        let d = decode_body("text/plain; charset=gbk", &gbk, false).unwrap();
        assert_eq!((d.text.as_str(), d.enc), ("中文新闻", encoding_rs::GBK));
        assert!(!d.utf8_as_is);
        // The header wins over the meta.
        let mut page = b"<meta charset=\"utf-8\">".to_vec();
        page.extend_from_slice(&gbk);
        assert_eq!(
            decode_body("text/html; charset=gb18030", &page, true)
                .unwrap()
                .enc,
            encoding_rs::GB18030
        );
        // A BOM wins over both, and BOM'd UTF-8 is not reused as is.
        let d = decode_body("text/plain; charset=gbk", "\u{feff}ok".as_bytes(), false).unwrap();
        assert_eq!((d.text.as_str(), d.enc, d.utf8_as_is), ("ok", UTF_8, false));
        let d = decode_body("text/html", b"<p>x</p>", true).unwrap();
        assert_eq!((d.enc, d.utf8_as_is), (UTF_8, true));
        assert!(
            decode_body("text/html; charset=klingon-7", b"x", true)
                .err()
                .unwrap()
                .contains("klingon-7")
        );
    }

    #[test]
    fn meta_utf16_means_utf8_and_invalid_bytes_fail() {
        // An ASCII-readable page that claims UTF-16 in a <meta> is UTF-8.
        let page = "<meta charset=\"utf-16\"><p>中文</p>".as_bytes();
        let d = decode_body("text/html", page, true).unwrap();
        assert_eq!(d.enc, UTF_8);
        assert!(d.text.contains("中文"));
        // A header that says UTF-16 is believed (no meta rule there).
        assert_eq!(
            decode_body("text/html; charset=utf-16le", b"a\x00", true)
                .unwrap()
                .enc,
            encoding_rs::UTF_16LE
        );
        // Bytes invalid in the chosen encoding: an error, not U+FFFD.
        assert!(
            decode_body("text/html; charset=utf-8", b"ok \xff\xfe bad", true)
                .err()
                .unwrap()
                .contains("无效的字节")
        );
        let issue = test_issue("badbytes");
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-x",
            got(200, "text/plain; charset=utf-8", b"text \xc3 broken"),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert_eq!(o.outputs, s(&["a-x.headers", "a-x.bin"]));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn gbk_page_is_decoded_into_the_text_archive() {
        let issue = test_issue("gbk");
        let words = "人工智能公司发布新模型，媒体报道称其性能超过所有竞争对手，但官方说明只比较了三项测试。";
        let page = format!(
            "<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=gb2312\"><title>新闻</title></head><body><p>{}</p></body></html>",
            words.repeat(3)
        );
        let (body, _, _) = encoding_rs::GBK.encode(&page);
        let o = job(&issue, false).archive(&issue.sources(), "a-cn", got(200, "text/html", &body));
        assert_eq!(o.verdict, Verdict::Ok, "{:?}", o.reasons);
        let txt = fs::read_to_string(issue.sources().join("a-cn.txt")).unwrap();
        assert!(txt.contains("人工智能公司发布新模型"));
        // The body itself is kept byte for byte.
        assert_eq!(
            fs::read(issue.sources().join("a-cn.html")).unwrap(),
            &body[..]
        );
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn unknown_charset_fails_without_text() {
        let issue = test_issue("badcs");
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-x",
            got(200, "text/html; charset=klingon-7", ARTICLE.as_bytes()),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert_eq!(o.outputs, s(&["a-x.headers", "a-x.html"]));
        assert!(o.reasons[0].contains("无法识别的字符集"));
        let _ = fs::remove_dir_all(&issue.root);
    }

    // ---- files ----

    #[test]
    fn existing_outputs_and_no_overwrite() {
        let issue = test_issue("exist");
        let src = issue.sources();
        assert!(existing_outputs(&src, "a-home").is_empty());
        fs::write(src.join("a-home.headers"), "x").unwrap();
        fs::write(src.join("a-home.pdf"), "x").unwrap();
        fs::write(src.join("a-home2.html"), "x").unwrap();
        assert_eq!(
            existing_outputs(&src, "a-home"),
            vec!["a-home.headers".to_string(), "a-home.pdf".to_string()]
        );
        let e = put(&src, "a-home.headers", b"new").unwrap_err();
        assert_eq!(e, "sources/a-home.headers 已存在，拒绝覆盖");
        assert_eq!(fs::read_to_string(src.join("a-home.headers")).unwrap(), "x");
        put(&src, "a-new.txt", b"fresh").unwrap();
        assert_eq!(fs::read_to_string(src.join("a-new.txt")).unwrap(), "fresh");
        // No temp file is left behind.
        let parts = fs::read_dir(&src)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().ends_with(".part"))
            .count();
        assert_eq!(parts, 0);
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn converter_output_is_streamed_capped_and_classified() {
        let issue = test_issue("conv");
        let dir = issue.root.clone();
        let pdf = issue.sources().join("x.pdf");
        fs::write(&pdf, b"%PDF-1.7\n").unwrap();
        let t = Duration::from_secs(20);
        let ok = fake_converter(&dir, "ok", "echo Section 1 Introduction");
        let out = pdf_to_text(&ok, &pdf, 1024, t).unwrap();
        assert!(String::from_utf8_lossy(&out).contains("Section 1 Introduction"));
        assert!(has_text(&out));
        // Over the cap: refused, child killed, nothing kept.
        #[cfg(windows)]
        let flood = "for /L %%i in (1,1,400) do echo 0123456789012345678901234567890123456789";
        #[cfg(not(windows))]
        let flood = "i=0; while [ $i -lt 400 ]; do echo 0123456789012345678901234567890123456789; i=$((i+1)); done";
        let big = fake_converter(&dir, "big", flood);
        assert!(
            pdf_to_text(&big, &pdf, 1024, t)
                .err()
                .unwrap()
                .contains("上限")
        );
        // A failing converter is an error, not empty text.
        let bad = fake_converter(&dir, "bad", "exit 3");
        assert!(
            pdf_to_text(&bad, &pdf, 1024, t)
                .err()
                .unwrap()
                .contains("失败")
        );
        // Blank output (a scan without a text layer) is "no text".
        assert!(!has_text(b"\n \x0c\n\r\n"));

        // Through `archive`: blank output fails the item, the PDF stays.
        #[cfg(windows)]
        let blank = "echo.";
        #[cfg(not(windows))]
        let blank = "echo";
        let mut j = job(&issue, false);
        j.pdftotext = fake_converter(&dir, "blank", blank);
        let o = j.archive(
            &issue.sources(),
            "a-scan",
            got(200, "application/pdf", b"%PDF-1.7\n%%EOF\n"),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert_eq!(o.outputs, s(&["a-scan.headers", "a-scan.pdf"]));
        assert!(o.reasons[0].contains("PDF 无可提取文本"), "{:?}", o.reasons);
        j.pdftotext = ok;
        let o = j.archive(
            &issue.sources(),
            "a-paper",
            got(200, "application/pdf", b"%PDF-1.7\n%%EOF\n"),
        );
        assert_eq!(o.verdict, Verdict::Ok, "{:?}", o.reasons);
        assert_eq!(
            o.outputs,
            s(&["a-paper.headers", "a-paper.pdf", "a-paper.txt"])
        );
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn ok_html_gets_body_headers_and_text() {
        let issue = test_issue("okhtml");
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-home",
            got(200, "text/html; charset=UTF-8", ARTICLE.as_bytes()),
        );
        assert_eq!(o.verdict, Verdict::Ok, "{:?}", o.reasons);
        assert_eq!(
            o.outputs,
            s(&["a-home.headers", "a-home.html", "a-home.txt"])
        );
        assert_eq!(
            o.sha256.as_deref(),
            Some(sha256_hex(ARTICLE.as_bytes()).as_str())
        );
        let txt = fs::read_to_string(issue.sources().join("a-home.txt")).unwrap();
        assert!(txt.contains("Learn more [https://iana.org/domains/example]"));
        assert_eq!(o.txt_lines, Some(txt.lines().count()));
        let headers = fs::read_to_string(issue.sources().join("a-home.headers")).unwrap();
        assert!(headers.starts_with("HTTP/1.1 200\nURL: https://example.com/a\n"));
        assert!(headers.contains("content-type: text/html; charset=UTF-8\n"));
        assert!(headers.contains("set-cookie: [已删]\n"));
        assert!(!headers.contains("SECRETVALUE") && !headers.contains("user:pw"));
        assert!(!headers.contains("token") && !headers.contains("q=1"));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn header_values_follow_the_allowlist() {
        for h in LOCATION_HEADERS {
            assert!(reveals_location(h), "{h}");
            assert!(!SAFE_HEADERS.contains(&h), "{h} cannot be both");
            assert_eq!(h, h.to_ascii_lowercase(), "{h} must be lower-case");
        }
        for h in SAFE_HEADERS.iter().chain(&ORIGIN_HEADERS) {
            assert_eq!(*h, h.to_ascii_lowercase());
        }
        assert_eq!(header_value("Content-Type", "text/html"), "text/html");
        assert_eq!(header_value("cf-cache-status", "HIT"), "HIT");
        assert_eq!(
            header_value("CF-Ray", "8f1e2d3c4b5a6978-NRT"),
            LOCATION_REDACTED
        );
        assert_eq!(header_value("x-fastly-pop", "HKG"), LOCATION_REDACTED);
        assert_eq!(
            header_value("x-forwarded-for", "203.0.113.7"),
            LOCATION_REDACTED
        );
        // Unknown and free-form headers: allowlist, not denylist.
        for (k, v) in [
            ("x-request-id", "abc"),
            ("x-geo-country", "CN"),
            ("set-cookie", "a=b"),
            ("content-security-policy", "default-src 'self'"),
            ("etag", "\"client-7f3a\""),
            ("server", "BunnyCDN-DE1-1052"),
            ("content-disposition", "attachment; filename=\"u123.pdf\""),
            (
                "link",
                "<https://a.com/p>; rel=\"canonical\"; anchor=\"https://a.com/?token=t\"",
            ),
            ("refresh", "0; URL = https://a.com/b?token=t"),
        ] {
            assert_eq!(header_value(k, v), REDACTED, "{k}");
        }
        // One-URL headers keep only where they point.
        assert_eq!(
            header_value("Location", "https://u:p@a.com/next?sig=1#x"),
            "https://a.com"
        );
        assert_eq!(
            header_value("content-location", "//user:pass@host.example:8443/doc"),
            "//host.example:8443"
        );
        assert_eq!(header_value("location", "/doc.pdf?token=z"), REDACTED);
        assert_eq!(header_value("location", "javascript:alert(1)"), REDACTED);
        // WWW-Authenticate: scheme names only.
        assert_eq!(
            header_value(
                "WWW-Authenticate",
                "Digest realm=\"a, b\", nonce=\"dcd98b7102dd2f0e\", qop=\"auth\", Basic realm=\"x\""
            ),
            "Digest, Basic"
        );
        assert_eq!(
            header_value("www-authenticate", "Bearer error=\"invalid_token\""),
            "Bearer"
        );
        // Public URLs: no userinfo, query or fragment.
        assert_eq!(
            public_url("https://u:p@a.com/x/y?token=1&utm=2#frag"),
            "https://a.com/x/y"
        );
        assert_eq!(public_url("https://a.com"), "https://a.com");
    }

    #[test]
    fn location_headers_are_redacted_in_headers_and_absent_from_log() {
        let issue = test_issue("pop");
        let mut g = got(200, "text/html", ARTICLE.as_bytes());
        g.headers.extend(
            [
                ("cf-ray", "8f1e2d3c4b5a6978-NRT"),
                ("x-amz-cf-pop", "HKG62-P1"),
                ("x-served-by", "cache-hkg17920-HKG"),
                ("x-cache", "HIT from cloudfront"),
                ("via", "1.1 abc.cloudfront.net (CloudFront)"),
                ("server-timing", "cdn-cache; desc=HIT, edge; dur=1"),
                ("x-fastly-request-id", "f00"),
                ("fly-request-id", "01J-sin"),
                ("x-vercel-id", "hkg1::iad1::abc"),
                ("x-forwarded-for", "203.0.113.7"),
                ("x-real-ip", "203.0.113.7"),
                ("cf-connecting-ip", "203.0.113.7"),
                ("x-client-ip", "203.0.113.7"),
                ("cf-cache-status", "HIT"),
            ]
            .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        let o = job(&issue, false).archive(&issue.sources(), "a-pop", g);
        let text = fs::read_to_string(issue.sources().join("a-pop.headers")).unwrap();
        for leak in [
            "NRT",
            "8f1e2d3c",
            "HKG",
            "hkg",
            "cloudfront",
            "edge; dur",
            "f00",
            "sin",
            "203.0.113.7",
        ] {
            assert!(!text.contains(leak), "{leak} leaked:\n{text}");
        }
        assert!(text.contains(&format!("cf-ray: {LOCATION_REDACTED}\n")));
        assert!(text.contains("cf-cache-status: HIT\n"));
        assert_eq!(text.matches(LOCATION_REDACTED).count(), 13);
        let item = parse_manifest("pop\thttps://example.com/\n")
            .unwrap()
            .remove(0);
        let line = log_line(&issue, &item, &o);
        assert!(!line.contains("NRT") && !line.contains("203.0.113.7") && !line.contains("hkg"));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn blocked_keeps_body_and_headers_but_no_text() {
        let issue = test_issue("blocked");
        let j = job(&issue, false);
        let o = j.archive(
            &issue.sources(),
            "a-wall",
            got(403, "text/html", b"<html>Forbidden</html>"),
        );
        assert_eq!(o.verdict, Verdict::Blocked);
        assert_eq!(o.outputs, s(&["a-wall.headers", "a-wall.html"]));
        assert_eq!(o.txt_lines, None);
        let cf = "<html><head><title>Just a moment...</title></head><body>Enable JavaScript and cookies to continue. Checking the security of your connection before proceeding.</body></html>";
        let o = j.archive(
            &issue.sources(),
            "a-cf",
            got(200, "text/html", cf.as_bytes()),
        );
        assert_eq!(o.verdict, Verdict::Blocked);
        assert!(!issue.sources().join("a-cf.txt").exists());
        // A plain-text error body is kept as .bin: .txt is for usable text.
        let o = j.archive(
            &issue.sources(),
            "a-rate",
            got(429, "text/plain", b"slow down"),
        );
        assert_eq!(o.outputs, s(&["a-rate.headers", "a-rate.bin"]));
        assert_eq!(o.sha256, Some(sha256_hex(b"slow down")));
        assert_eq!(o.bytes, Some(9));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn pdf_without_text_fails_but_keeps_the_pdf() {
        let issue = test_issue("pdf");
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-paper",
            got(200, "application/pdf", b"%PDF-1.7\n%%EOF\n"),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert_eq!(o.outputs, s(&["a-paper.headers", "a-paper.pdf"]));
        assert!(
            o.reasons[0].starts_with("PDF 已保存，但未生成 txt：找不到 pdftotext"),
            "{:?}",
            o.reasons
        );
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn plain_text_body_is_its_own_archive_only_in_utf8() {
        let issue = test_issue("plain");
        let j = job(&issue, false);
        let o = j.archive(
            &issue.sources(),
            "a-rfc",
            got(200, "text/plain", b"line 1\nline 2\n"),
        );
        assert_eq!(o.verdict, Verdict::Ok);
        assert_eq!(o.outputs, s(&["a-rfc.headers", "a-rfc.txt"]));
        assert_eq!(o.txt_lines, Some(2));
        let (gbk, _, _) = encoding_rs::GBK.encode("第一行\n第二行\n");
        let o = j.archive(
            &issue.sources(),
            "a-cn",
            got(200, "text/plain; charset=gbk", &gbk),
        );
        assert_eq!(o.outputs, s(&["a-cn.headers", "a-cn.bin", "a-cn.txt"]));
        assert_eq!(
            fs::read_to_string(issue.sources().join("a-cn.txt")).unwrap(),
            "第一行\n第二行\n"
        );
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn failed_headers_write_stops_the_item() {
        let issue = test_issue("hdrfail");
        fs::write(issue.sources().join("a-home.headers"), "old").unwrap();
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-home",
            got(200, "text/html", ARTICLE.as_bytes()),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert!(o.outputs.is_empty());
        assert!(!issue.sources().join("a-home.html").exists());
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn write_race_fails_with_a_relative_path_only() {
        let issue = test_issue("race");
        fs::write(issue.sources().join("a-home.html"), "old").unwrap();
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-home",
            got(200, "text/html", ARTICLE.as_bytes()),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert_eq!(
            fs::read_to_string(issue.sources().join("a-home.html")).unwrap(),
            "old"
        );
        assert!(!issue.sources().join("a-home.txt").exists());
        let reason = o.reasons.join("；");
        assert!(reason.contains("sources/a-home.html 已存在"), "{reason}");
        let root = issue.root.to_string_lossy().into_owned();
        let item = parse_manifest("home\thttps://example.com/\n")
            .unwrap()
            .remove(0);
        let line = log_line(&issue, &item, &o);
        assert!(!reason.contains(&root) && !line.contains(&root));
        assert!(!line.contains(":\\\\") && !line.contains("/tmp/"), "{line}");
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn force_replaces_the_whole_previous_version() {
        let issue = test_issue("force");
        let src = issue.sources();
        for f in ["a-doc.pdf", "a-doc.txt", "a-doc.headers", "a-doc.bin"] {
            fs::write(src.join(f), "old").unwrap();
        }
        fs::write(src.join("a-doc2.pdf"), "other item").unwrap();
        let o = job(&issue, true).archive(&src, "a-doc", got(200, "text/html", ARTICLE.as_bytes()));
        assert_eq!(o.verdict, Verdict::Ok, "{:?}", o.reasons);
        assert_eq!(
            existing_outputs(&src, "a-doc"),
            s(&["a-doc.headers", "a-doc.txt", "a-doc.html"])
        );
        assert_ne!(fs::read_to_string(src.join("a-doc.txt")).unwrap(), "old");
        assert!(src.join("a-doc2.pdf").exists());
        // A failed re-fetch replaces the version too: only its note remains.
        let item = parse_manifest("doc\thttps://example.com/\n")
            .unwrap()
            .remove(0);
        let o = job(&issue, true).no_response(&src, "a-doc", &item, "请求失败：timeout".into());
        assert_eq!(o.outputs, s(&["a-doc.headers"]));
        assert_eq!(existing_outputs(&src, "a-doc"), s(&["a-doc.headers"]));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn no_response_writes_only_a_headers_note() {
        let issue = test_issue("noresp");
        let item = parse_manifest("x\thttps://example.com/?sig=1\n")
            .unwrap()
            .remove(0);
        let o = job(&issue, false).no_response(
            &issue.sources(),
            "a-x",
            &item,
            "请求失败：timeout".into(),
        );
        assert_eq!(o.verdict, Verdict::Failed);
        assert_eq!(o.outputs, s(&["a-x.headers"]));
        let text = fs::read_to_string(issue.sources().join("a-x.headers")).unwrap();
        assert_eq!(
            text,
            "# 无响应：请求失败：timeout\nURL: https://example.com/\n"
        );
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn log_line_has_fixed_keys_and_scrubbed_urls() {
        let issue = test_issue("log");
        let item = parse_manifest("home\thttps://example.com/?api_key=k\n")
            .unwrap()
            .remove(0);
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-home",
            got(200, "text/html", ARTICLE.as_bytes()),
        );
        let line = log_line(&issue, &item, &o);
        let v: Value = serde_json::from_str(&line).unwrap();
        let mut keys: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
        let mut want = vec![
            "time_bj",
            "name",
            "url",
            "final_url",
            "status",
            "content_type",
            "bytes",
            "sha256",
            "outputs",
            "result",
            "reason",
        ];
        assert!(line.starts_with("{\"time_bj\":\"") && line.contains("+08:00\",\"name\":\"home\""));
        keys.sort();
        want.sort();
        assert_eq!(keys, want);
        assert_eq!(v["url"], "https://example.com/?api_key=[REDACTED]");
        assert_eq!(v["final_url"], "https://example.com/a");
        assert_eq!(v["result"], "ok");
        assert_eq!(v["outputs"][0], "docs/2610/1015/sources/a-home.headers");
        assert!(!line.to_lowercase().contains("user-agent"));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn log_is_opened_once_and_closes_a_torn_line() {
        let issue = test_issue("logopen");
        let path = issue.sources().join("a-fetch-log.jsonl");
        fs::write(&path, "{\"x\":1}").unwrap();
        let mut f = open_log(&path).unwrap();
        f.write_all(b"{\"y\":2}\n").unwrap();
        f.write_all(b"{\"z\":3}\n").unwrap();
        drop(f);
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "{\"x\":1}\n{\"y\":2}\n{\"z\":3}\n"
        );
        // An empty or fresh log gets no stray newline.
        let fresh = issue.sources().join("b-fetch-log.jsonl");
        drop(open_log(&fresh).unwrap());
        assert_eq!(fs::read(&fresh).unwrap(), b"");
        // A directory in its place cannot be opened: the batch would stop.
        let dir = issue.sources().join("c-fetch-log.jsonl");
        fs::create_dir_all(&dir).unwrap();
        assert!(open_log(&dir).is_err());
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn summary_lists_items_in_manifest_order() {
        let items = parse_manifest("b\thttps://b.com/\na\thttps://a.com/\n").unwrap();
        let mut ok = Outcome::failed(String::new());
        ok.verdict = Verdict::Ok;
        ok.reasons.clear();
        ok.status = Some(200);
        ok.bytes = Some(1256);
        ok.txt_lines = Some(12);
        let outcomes = vec![ok, Outcome::failed("请求失败：timeout".into())];
        let table = summary(&items, &outcomes);
        let lines: Vec<&str> = table.lines().collect();
        assert!(lines[0].starts_with("name"));
        assert!(
            lines[1].starts_with("b ") && lines[1].contains(" ok ") && lines[1].contains("1256")
        );
        assert!(
            lines[2].starts_with("a ")
                && lines[2].contains("failed")
                && lines[2].ends_with("请求失败：timeout")
        );
    }
}
