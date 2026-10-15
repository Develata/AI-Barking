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
//! is content, `<group>-<name>.txt` (HTML through `fetch::html_to_text`, PDF
//! through poppler's `pdftotext -enc UTF-8 -layout`). Every item appends one
//! line to `sources/<group>-fetch-log.jsonl` as it finishes.
//!
//! Decisions worth knowing when reading the code:
//! - Requests are anonymous: ureq is built without its `cookies` feature, so
//!   there is no jar; the only identifying request header is the desktop
//!   Chrome User-Agent everyone sends. Nothing about the request is logged.
//! - Timeouts are end to end (ureq's global timeout covers the body): 60 s,
//!   or 180 s when the URL path ends in `.pdf`. A response that turns out to
//!   be `application/pdf` at a URL without that suffix (arXiv `/pdf/<id>`) is
//!   dropped at its headers and requested again with the PDF budget, because
//!   ureq cannot extend a deadline it has already started. One retry, only
//!   on a transport error or a 5xx; a 4xx is an answer, not an accident.
//! - "Blocked" is not "failed": 401/402/403/429, and 2xx HTML pages that read
//!   as a challenge, login or paywall gate (see `GATE_TEXT`). Gate phrases are
//!   only looked for on short pages, since real articles quote "access
//!   denied" too; a long teaser in front of a paywall therefore passes as ok,
//!   and the reader still has to look. Blocked and failed items keep their
//!   body and `.headers` (what the server answered is itself evidence) but get
//!   no `.txt`, so no gate page is ever grepped as if it were the source.
//! - Nothing is overwritten without `--force`: the whole batch is refused
//!   before any request if any item already has outputs, and every write uses
//!   `create_new`. Nothing is ever deleted, also not stale siblings of a
//!   re-fetched item under `--force`.
//! - At most `--jobs` requests run at once and at most two per manifest host
//!   (redirect targets are not counted). The summary follows the manifest
//!   order; the log follows completion order, so a killed batch still leaves
//!   a record of what finished.

use std::fs;
use std::io::{self, ErrorKind, Read as _, Write as _};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::{Condvar, Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;
use sha2::{Digest, Sha256};
use ureq::ResponseExt as _;

use crate::fetch::{USER_AGENT, html_to_text, write_new};
use crate::repo::{self, Issue};
use crate::shot::{jsonl, log_url, pick_tool, poppler_dirs};

const USAGE: &str = "\
用法：barking fetch <期次目录> <组前缀> <清单文件|-> [--jobs 6] [--force]

清单每行一条：name<TAB>url（空行与 # 开头的行忽略；- 表示从标准输入读）。
  name 只许小写字母、数字与 -，输出在 sources/<组前缀>-<name>.*：
    正文 .html / .pdf（先看文件头再看 Content-Type；其他类型为 .json .xml .txt .png 等，未知为 .bin）
    .headers  状态行、最终 URL 与响应头（Set-Cookie 只留名字）
    .txt      文本存档：HTML 转纯文本（同 html2txt），PDF 用 poppler 的 pdftotext -enc UTF-8 -layout
  匿名 GET（桌面 Chrome UA、Accept-Language en-US、跟随跳转、无 Cookie），每条限时 60 秒（PDF 180 秒）；
  网络错误或 5xx 重试一次，4xx 不重试。并发 --jobs（默认 6，最多 16），同一站点最多 2 个。
  401/402/403/429 与疑似验证页、付费墙、过短正文记为 blocked：保留正文与 .headers，不生成 .txt。
  任一条目已有输出则整批拒绝（加 --force 覆盖）；不删除任何文件。
  每条在 sources/<组前缀>-fetch-log.jsonl 追加一行（北京时间、URL、状态、字节、sha256、输出、结果与原因）。
  部分条目失败仍以 0 退出，末行写明失败数；只有用法错误与拒绝覆盖以非 0 退出。
  pdftotext 先找环境变量 BARKING_POPPLER 指定的目录，再找 ~/scoop/apps/poppler/current/bin，最后 PATH；找不到只记原因。";

const DEFAULT_JOBS: usize = 6;
const MAX_JOBS: usize = 16;
/// In-flight requests per manifest host: enough to overlap two slow pages,
/// few enough not to look like a scraper to a news site's bot wall.
const PER_HOST: usize = 2;
const PAGE_TIMEOUT: Duration = Duration::from_secs(60);
const PDF_TIMEOUT: Duration = Duration::from_secs(180);
const RETRY_PAUSE: Duration = Duration::from_secs(2);
/// pdftotext on a hostile or broken PDF can spin; it must not hang the batch.
const PDFTOTEXT_TIMEOUT: Duration = Duration::from_secs(120);
/// Court filings and papers run to tens of MB; a body over this is refused
/// rather than buffered (up to `MAX_JOBS` of them are held at once).
const MAX_BODY: u64 = 128 * 1024 * 1024;
const ACCEPT: &str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,application/pdf;q=0.9,*/*;q=0.8";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9";
const MAX_NAME: usize = 60;

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
/// Captcha and challenge markup that carries no text of its own, matched in
/// the raw HTML of a short page. (`/cdn-cgi/challenge-platform/` alone is not
/// here: Cloudflare injects it into ordinary pages too; `_cf_chl_opt` only
/// appears on its challenge pages.)
const GATE_MARKUP: [(&str, &str); 6] = [
    ("_cf_chl_opt", "Cloudflare 质询"),
    ("cf-turnstile", "Cloudflare Turnstile"),
    ("g-recaptcha", "reCAPTCHA"),
    ("h-captcha", "hCaptcha"),
    ("captcha-delivery.com", "DataDome 验证"),
    ("px-captcha", "PerimeterX 验证"),
];
/// Every extension `choose_ext` can return, plus the two sidecars: the
/// no-overwrite check looks for all of them, because the body's extension is
/// only known after the fetch.
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
/// other malformed line, bad name, non-http(s) URL or repeated name fails the
/// whole manifest (line numbers are 1-based), before anything is fetched.
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

/// Why a 2xx HTML page is not the content, or `None` if it may be. `text` is
/// its `html_to_text` output, `raw` the HTML.
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
/// look for HTML. `txt` means the body is plain text and is its own archive.
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
// Files

/// Output names of `base` already present in `sources`.
fn existing_outputs(sources: &Path, base: &str) -> Vec<String> {
    OUTPUT_EXTS
        .iter()
        .map(|ext| format!("{base}.{ext}"))
        .filter(|f| sources.join(f).exists())
        .collect()
}

/// `create_new` unless `force`, so the pre-flight check cannot be raced.
fn save(path: &Path, bytes: &[u8], force: bool) -> Result<(), String> {
    if force {
        fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
    } else {
        write_new(path, bytes)
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Appends one line, first closing a last line that lost its newline (the
/// same guard as the shots log).
fn append_line(path: &Path, line: &str) -> Result<(), String> {
    let unterminated = fs::read(path).is_ok_and(|b| b.last().is_some_and(|&c| c != b'\n'));
    let mut text = String::new();
    if unterminated {
        text.push('\n');
    }
    text.push_str(line);
    text.push('\n');
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut f| f.write_all(text.as_bytes()))
        .map_err(|e| format!("{}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Fetching

/// One HTTP response as it is archived.
struct Got {
    status: u16,
    status_line: String,
    final_url: String,
    content_type: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

enum Attempt {
    Got(Got),
    /// Headers said PDF, but the request ran on the page budget.
    NeedsPdfBudget,
    /// Worth one more try (transport error, timeout).
    Retry(String),
    /// Not worth another try (body over the limit).
    Fatal(String),
}

fn http_agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent(USER_AGENT)
        .accept(ACCEPT)
        .build()
        .into()
}

fn attempt(agent: &ureq::Agent, url: &str, timeout: Duration) -> Attempt {
    let call = agent
        .get(url)
        .header("Accept-Language", ACCEPT_LANGUAGE)
        .config()
        .timeout_global(Some(timeout))
        .build()
        .call();
    let mut resp = match call {
        Ok(r) => r,
        Err(e) => return Attempt::Retry(format!("请求失败：{e}")),
    };
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
            (k.as_str().to_string(), v)
        })
        .collect();
    let content_type = headers
        .iter()
        .find(|(k, _)| k == "content-type")
        .map(|(_, v)| v.clone())
        .unwrap_or_default();
    if timeout < PDF_TIMEOUT && mime(&content_type) == "application/pdf" {
        return Attempt::NeedsPdfBudget;
    }
    let status = resp.status();
    let status_line = format!("{:?} {status}", resp.version());
    let final_url = resp.get_uri().to_string();
    let body = match resp.body_mut().with_config().limit(MAX_BODY).read_to_vec() {
        Ok(b) => b,
        Err(ureq::Error::BodyExceedsLimit(n)) => {
            return Attempt::Fatal(format!("正文超过 {n} 字节上限，未保存"));
        }
        Err(e) => return Attempt::Retry(format!("读取响应失败：{e}")),
    };
    Attempt::Got(Got {
        status: status.as_u16(),
        status_line,
        final_url,
        content_type,
        headers,
        body,
    })
}

/// The response to archive, plus notes on what it took; `Err` when no
/// response came. A 5xx that persists through the retry is still returned:
/// what the server answered is the record.
fn fetch_url(agent: &ureq::Agent, url: &str) -> (Result<Got, String>, Vec<String>) {
    let mut timeout = if is_pdf_url(url) {
        PDF_TIMEOUT
    } else {
        PAGE_TIMEOUT
    };
    let mut notes = Vec::new();
    let mut retried = false;
    loop {
        let (failure, response) = match attempt(agent, url, timeout) {
            // Cannot repeat: the budget is now the PDF one.
            Attempt::NeedsPdfBudget => {
                timeout = PDF_TIMEOUT;
                notes.push("响应为 PDF，按 180 秒限时重新请求".to_string());
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
        thread::sleep(RETRY_PAUSE);
    }
}

/// `name=value; attrs` → `name=[REDACTED]`: an anonymous session cookie is
/// not ours, but it can still encode the client address.
fn redact_cookie(v: &str) -> String {
    let name = v
        .split(';')
        .next()
        .unwrap_or("")
        .split('=')
        .next()
        .unwrap_or("");
    format!("{}=[REDACTED]", name.trim())
}

/// The `.headers` sidecar: status line, final URL, then the response headers
/// as received (names lower-cased by the HTTP library).
fn headers_text(root: &Path, g: &Got) -> String {
    let mut s = format!("{}\nURL: {}\n", g.status_line, log_url(root, &g.final_url));
    for (k, v) in &g.headers {
        let v = if k == "set-cookie" {
            redact_cookie(v)
        } else {
            v.clone()
        };
        s.push_str(&format!("{k}: {v}\n"));
    }
    s
}

/// `pdftotext -enc UTF-8 -layout <pdf> <tmp>`, bounded by
/// `PDFTOTEXT_TIMEOUT`, then the text saved to `out`; returns its line count.
/// Output goes through a temp file, not a pipe, so that a large text cannot
/// block the child while this thread polls it.
fn pdf_to_text(tool: &Path, pdf: &Path, out: &Path, force: bool) -> Result<usize, String> {
    let stem = out.file_name().map(|n| n.to_string_lossy().into_owned());
    let tmp = std::env::temp_dir().join(format!(
        "barking-fetch-{}-{}",
        std::process::id(),
        stem.unwrap_or_default()
    ));
    let _ = fs::remove_file(&tmp);
    let mut child = Command::new(tool)
        .args(["-enc", "UTF-8", "-layout"])
        .arg(pdf)
        .arg(&tmp)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => {
                "找不到 pdftotext（用 Scoop 装 poppler，或用 BARKING_POPPLER 指向其 bin 目录）"
                    .to_string()
            }
            _ => format!("无法运行 pdftotext：{e}"),
        })?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if started.elapsed() > PDFTOTEXT_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&tmp);
                return Err(format!(
                    "pdftotext 超过 {} 秒未结束，已终止",
                    PDFTOTEXT_TIMEOUT.as_secs()
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(e) => return Err(format!("等待 pdftotext 失败：{e}")),
        }
    };
    let result = if status.success() {
        fs::read(&tmp)
            .map_err(|e| format!("读取 pdftotext 输出失败：{e}"))
            .and_then(|text| {
                save(out, &text, force)?;
                Ok(String::from_utf8_lossy(&text).lines().count())
            })
    } else {
        Err(format!("pdftotext 失败（{status}）"))
    };
    let _ = fs::remove_file(&tmp);
    result
}

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
}

/// Everything an item needs besides itself. Shared by reference across the
/// pool's threads, hence only `Sync` fields (`ureq::Agent` is a handle onto
/// a thread-safe connection pool).
struct Job<'a> {
    issue: &'a Issue,
    group: &'a str,
    force: bool,
    agent: ureq::Agent,
    pdftotext: PathBuf,
}

impl Job<'_> {
    fn base(&self, item: &Item) -> String {
        format!("{}-{}", self.group, item.name)
    }

    fn run(&self, item: &Item) -> Outcome {
        let sources = self.issue.sources();
        let base = self.base(item);
        let (got, notes) = fetch_url(&self.agent, &item.url);
        let mut out = match got {
            Ok(g) => self.archive(&sources, &base, g),
            Err(reason) => self.no_response(&sources, &base, item, reason),
        };
        out.reasons.splice(0..0, notes);
        out
    }

    /// No response at all: only a `.headers` saying so.
    fn no_response(&self, sources: &Path, base: &str, item: &Item, reason: String) -> Outcome {
        let mut o = Outcome::failed(reason);
        let text = format!(
            "# 无响应：{}\nURL: {}\n",
            o.reasons[0],
            log_url(&self.issue.root, &item.url)
        );
        let file = format!("{base}.headers");
        match save(&sources.join(&file), text.as_bytes(), self.force) {
            Ok(()) => o.outputs.push(file),
            Err(e) => o.reasons.push(e),
        }
        o
    }

    /// Writes the files for one response and judges it.
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
        let raw = String::from_utf8_lossy(&g.body);
        let html_text = (ext == "html").then(|| html_to_text(raw.trim_start_matches('\u{feff}')));
        if verdict == Verdict::Ok
            && let Some(why) = html_text.as_deref().and_then(|t| detect_gate(&raw, t))
        {
            verdict = Verdict::Blocked;
            reasons.push(why);
        }

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
        let write = |file: String, bytes: &[u8], o: &mut Outcome| -> bool {
            match save(&sources.join(&file), bytes, self.force) {
                Ok(()) => {
                    o.outputs.push(file);
                    true
                }
                Err(e) => {
                    o.verdict = Verdict::Failed;
                    o.reasons.push(e);
                    false
                }
            }
        };

        write(
            format!("{base}.headers"),
            headers_text(&self.issue.root, &g).as_bytes(),
            &mut o,
        );
        // A plain-text body is its own `.txt`, which only an ok item gets.
        let keep_body = !g.body.is_empty() && (ext != "txt" || verdict == Verdict::Ok);
        let body_file = format!("{base}.{ext}");
        if keep_body && write(body_file.clone(), &g.body, &mut o) {
            o.sha256 = Some(sha256_hex(&g.body));
        }
        if o.verdict != Verdict::Ok {
            return o;
        }
        let txt = format!("{base}.txt");
        match ext {
            "html" => {
                let text = html_text.unwrap_or_default();
                if write(txt, text.as_bytes(), &mut o) {
                    o.txt_lines = Some(text.lines().count());
                }
            }
            "txt" => o.txt_lines = Some(raw.lines().count()),
            "pdf" => match pdf_to_text(
                &self.pdftotext,
                &sources.join(&body_file),
                &sources.join(&txt),
                self.force,
            ) {
                Ok(n) => {
                    o.outputs.push(txt);
                    o.txt_lines = Some(n);
                }
                // The PDF itself is archived; the missing text is noted, not
                // held against the item.
                Err(e) => o.reasons.push(format!("未生成 txt：{e}")),
            },
            _ => {}
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
            Value::from(o.final_url.as_deref().map(|u| log_url(root, u))),
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

    let job = Job {
        issue: &issue,
        group: &a.group,
        force: a.force,
        agent: http_agent(),
        pdftotext: pick_tool(&poppler_dirs(), "pdftotext"),
    };
    let log_name = format!("{}-fetch-log.jsonl", a.group);
    let log_path = sources.join(&log_name);
    // Serialises appends; collects the ones that failed.
    let log_errors: Mutex<Vec<String>> = Mutex::new(Vec::new());
    println!(
        "取得 {} 条（并发 {}，同一站点至多 {PER_HOST}）；开始于 {}",
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
        let line = log_line(&issue, item, &o);
        let mut errs = log_errors.lock().unwrap_or_else(PoisonError::into_inner);
        if let Err(e) = append_line(&log_path, &line) {
            errs.push(e);
        }
        o
    });

    print!("{}", summary(&items, &outcomes));
    for e in log_errors
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner)
    {
        println!("warn: 日志写入失败：{e}");
    }
    let count = |v: Verdict| outcomes.iter().filter(|o| o.verdict == v).count();
    let (ok, blocked, failed) = (
        count(Verdict::Ok),
        count(Verdict::Blocked),
        count(Verdict::Failed),
    );
    println!(
        "-- ok {ok}，blocked {blocked}，failed {failed}；用时 {} 秒；输出 {rel}/sources/{g}-<name>.*，日志 {rel}/sources/{log_name}",
        started.elapsed().as_secs(),
        rel = issue.rel(),
        g = a.group,
    );
    if blocked + failed > 0 {
        println!(
            "   blocked / failed 条目只有正文与 .headers、没有 .txt：改用 opencli 或浏览器取证，或修正 URL 后单独重跑（须加 --force）"
        );
    }
    ExitCode::SUCCESS
}
#[cfg(test)]
mod tests {
    use super::*;
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

    fn job<'a>(issue: &'a Issue, force: bool) -> Job<'a> {
        Job {
            issue,
            group: "a",
            force,
            agent: http_agent(),
            pdftotext: issue.root.join("no-such-pdftotext"),
        }
    }

    const ARTICLE: &str = "<html><head><title>Example Domain</title></head><body><h1>Example Domain</h1>\
        <p>This domain is for use in documentation examples without needing permission. \
        Avoid use in operations.</p><p><a href=\"https://iana.org/domains/example\">Learn more</a></p></body></html>";

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
        // Host 0 full: the first runnable is item 2 (host 1), at position 1.
        assert_eq!(
            next_runnable(&[1, 2, 3, 4], &host_of, &[2, 0, 0], 2),
            Some(1)
        );
        // Room on host 0: first come, first served.
        assert_eq!(
            next_runnable(&[1, 2, 3, 4], &host_of, &[1, 0, 0], 2),
            Some(0)
        );
        // Every pending host full: nothing to start.
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
            <script>window._cf_chl_opt={}</script><p>www.example.org needs to review the security of your connection before proceeding.</p></body></html>";
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

        // A captcha widget has no text; the markup gives it away.
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
        // example.com is the smallest real page: above the length floor.
        assert_eq!(detect_gate(ARTICLE, &html_to_text(ARTICLE)), None);
        // A long article may quote a gate phrase or carry a comment captcha.
        let para = "<p>The court found that the agency had been granted access to the logs, \
            and the plaintiffs argued that access denied to journalists was unlawful.</p>";
        let long = format!(
            "<html><head><title>Ruling</title></head><body>{}<div class=\"g-recaptcha\"></div></body></html>",
            para.repeat(20)
        );
        assert_eq!(detect_gate(&long, &html_to_text(&long)), None);
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

    // ---- files ----

    #[test]
    fn existing_outputs_and_no_overwrite() {
        let issue = test_issue("exist");
        let src = issue.sources();
        assert!(existing_outputs(&src, "a-home").is_empty());
        fs::write(src.join("a-home.headers"), "x").unwrap();
        fs::write(src.join("a-home.pdf"), "x").unwrap();
        // Same prefix, other item: not ours.
        fs::write(src.join("a-home2.html"), "x").unwrap();
        assert_eq!(
            existing_outputs(&src, "a-home"),
            vec!["a-home.headers".to_string(), "a-home.pdf".to_string()]
        );
        let p = src.join("a-home.headers");
        assert!(save(&p, b"new", false).unwrap_err().contains("拒绝覆盖"));
        assert_eq!(fs::read_to_string(&p).unwrap(), "x");
        save(&p, b"new", true).unwrap();
        assert_eq!(fs::read_to_string(&p).unwrap(), "new");
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
        assert!(
            headers.starts_with("HTTP/1.1 200\nURL: https://example.com/a?token=[REDACTED]&q=1\n")
        );
        assert!(headers.contains("set-cookie: __cf_bm=[REDACTED]\n"));
        assert!(!headers.contains("SECRETVALUE") && !headers.contains("user:pw"));
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
        // A 2xx challenge page is blocked too.
        let cf = "<html><head><title>Just a moment...</title></head><body>Enable JavaScript and cookies to continue. Checking the security of your connection before proceeding.</body></html>";
        let o = j.archive(
            &issue.sources(),
            "a-cf",
            got(200, "text/html", cf.as_bytes()),
        );
        assert_eq!(o.verdict, Verdict::Blocked);
        assert!(!issue.sources().join("a-cf.txt").exists());
        // A plain-text error body would be named .txt, so it is not kept.
        let o = j.archive(
            &issue.sources(),
            "a-rate",
            got(429, "text/plain", b"slow down"),
        );
        assert_eq!(o.outputs, s(&["a-rate.headers"]));
        assert_eq!(o.sha256, None);
        assert_eq!(o.bytes, Some(9));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn pdf_without_pdftotext_is_still_ok() {
        let issue = test_issue("pdf");
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-paper",
            got(200, "application/pdf", b"%PDF-1.7\n%%EOF\n"),
        );
        assert_eq!(o.verdict, Verdict::Ok);
        assert_eq!(o.outputs, s(&["a-paper.headers", "a-paper.pdf"]));
        assert!(
            o.reasons[0].starts_with("未生成 txt：找不到 pdftotext"),
            "{:?}",
            o.reasons
        );
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn plain_text_body_is_its_own_archive() {
        let issue = test_issue("plain");
        let o = job(&issue, false).archive(
            &issue.sources(),
            "a-rfc",
            got(200, "text/plain", b"line 1\nline 2\n"),
        );
        assert_eq!(o.verdict, Verdict::Ok);
        assert_eq!(o.outputs, s(&["a-rfc.headers", "a-rfc.txt"]));
        assert_eq!(o.txt_lines, Some(2));
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn write_race_fails_the_item_without_touching_the_file() {
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
        let _ = fs::remove_dir_all(&issue.root);
    }

    #[test]
    fn no_response_writes_only_a_headers_note() {
        let issue = test_issue("noresp");
        let item = parse_manifest("x\thttps://u:p@example.com/?sig=1\n")
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
            "# 无响应：请求失败：timeout\nURL: https://example.com/?sig=[REDACTED]\n"
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
        let keys: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
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
        let mut keys_sorted = keys.clone();
        keys_sorted.sort();
        want.sort();
        assert_eq!(keys_sorted, want);
        assert_eq!(v["url"], "https://example.com/?api_key=[REDACTED]");
        assert_eq!(v["final_url"], "https://example.com/a?token=[REDACTED]&q=1");
        assert_eq!(v["result"], "ok");
        assert_eq!(v["outputs"][0], "docs/2610/1015/sources/a-home.headers");
        assert!(!line.to_lowercase().contains("user-agent"));

        let path = issue.sources().join("a-fetch-log.jsonl");
        fs::write(&path, "{\"x\":1}").unwrap();
        append_line(&path, "{\"y\":2}").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "{\"x\":1}\n{\"y\":2}\n");
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
