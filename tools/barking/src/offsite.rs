//! `barking offsite`: files that stay out of git are recorded in
//! `sources/offsite.tsv` and, with `--upload`, copied to OpenList under
//! `<OPENLIST_ROOT>/<YYMM>/<MMDD>/`: every image under `images/` and the PDFs,
//! page archives, screenshots and media under `sources/` (patterns in
//! `.gitignore`). Files are uploaded individually, several at a time, never
//! zipped. Workflow: docs/workflow/publish.md.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub(crate) const MANIFEST: &str = "offsite.tsv";
const MANIFEST_HEADER: &str = "# 存于 OpenList 的文件（见 docs/workflow/publish.md；1002 期起图片与原件不入 Git，更早期次的图片与原件已于历史改写时移出 Git，仍在网盘）。列：相对期次目录的路径	字节数	SHA-256";
const USAGE: &str = "用法：barking offsite <期次目录>... [--upload [--jobs N]]";
/// Uploads in flight. A put returns only after OpenList has stored the file
/// in its backend, so each file cost ~2 s of waiting whatever its size
/// (serial runs of 1024–1026: 125–170 files in 290–538 s). Four overlap that
/// wait while staying gentle on cloud-drive rate limits.
const DEFAULT_JOBS: usize = 4;
const MAX_JOBS: usize = 16;
/// Tries per file; try `n` failing waits `RETRY_WAIT * n` before the next.
const PUT_ATTEMPTS: u32 = 3;
const RETRY_WAIT: Duration = Duration::from_secs(3);
/// Re-listings while an uploaded file is still absent: a fresh file can be
/// missing from a lookup for a moment (0928 run, 2026-10-01).
const VERIFY_RETRIES: u32 = 3;
const VERIFY_WAIT: Duration = Duration::from_secs(2);

/// One git-ignored file of an issue. Owned strings: entries are few and are
/// used after the `git ls-files` output buffer is gone.
pub(crate) struct Entry {
    /// Path relative to the issue directory, `/`-separated.
    pub(crate) rel: String,
    pub(crate) abs: PathBuf,
    pub(crate) size: u64,
    pub(crate) sha256: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Opts {
    dirs: Vec<String>,
    upload: bool,
    jobs: usize,
}

pub fn run(args: &[String]) -> ExitCode {
    let opts = match parse_args(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let root = match crate::repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    // Logged in on first use, so listing alone needs no credentials.
    let mut client: Option<OpenList> = None;
    let mut failed = 0usize;
    for d in &opts.dirs {
        println!("== {d}");
        match process(&root, Path::new(d), opts.upload, opts.jobs, &mut client) {
            Ok(n) => failed += n,
            Err(e) => {
                println!("error: {e}");
                failed += 1;
            }
        }
    }
    println!("-- {failed} 个失败");
    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn parse_args(args: &[String]) -> Result<Opts, String> {
    let mut opts = Opts {
        dirs: Vec::new(),
        upload: false,
        jobs: DEFAULT_JOBS,
    };
    let mut jobs_given = false;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--upload" => opts.upload = true,
            "--jobs" => {
                opts.jobs = parse_jobs(it.next().ok_or("--jobs 缺少取值")?)?;
                jobs_given = true;
            }
            s if s.starts_with("--jobs=") => {
                opts.jobs = parse_jobs(&s["--jobs=".len()..])?;
                jobs_given = true;
            }
            s if s.starts_with("--") => return Err(format!("未知选项 {s}")),
            _ => opts.dirs.push(a.clone()),
        }
    }
    if opts.dirs.is_empty() {
        return Err("缺少期次目录".into());
    }
    if jobs_given && !opts.upload {
        return Err("--jobs 只用于 --upload".into());
    }
    Ok(opts)
}

fn parse_jobs(s: &str) -> Result<usize, String> {
    match s.parse::<usize>() {
        Ok(n) if (1..=MAX_JOBS).contains(&n) => Ok(n),
        _ => Err(format!("--jobs 须为 1–{MAX_JOBS} 的整数，得到「{s}」")),
    }
}

/// Returns the number of files that failed to upload.
pub(crate) fn process(
    root: &Path,
    dir: &Path,
    upload: bool,
    jobs: usize,
    client: &mut Option<OpenList>,
) -> Result<usize, String> {
    let (yymm, mmdd) = issue_key(root, dir)?;
    let entries = ignored_files(root, &yymm, &mmdd)?;
    if entries.is_empty() {
        println!("没有不入库的原件");
        return Ok(0);
    }
    write_manifest(&root.join("docs").join(&yymm).join(&mmdd), &entries)?;
    if !upload {
        return Ok(0);
    }

    if client.is_none() {
        *client = Some(OpenList::login(root, jobs)?);
    }
    let Some(ol) = client.as_ref() else {
        unreachable!("client was just set")
    };
    // (remote path, local size), index-aligned with `entries`.
    let files: Vec<(String, u64)> = entries
        .iter()
        .map(|e| (format!("{}/{yymm}/{mmdd}/{}", ol.root, e.rel), e.size))
        .collect();

    // One listing per remote directory instead of one lookup per file.
    let (remote, unlisted) = ol.snapshot(parents(&files, 0..files.len()));
    let todo = unmatched(&files, &remote, 0..files.len());
    let skipped = files.len() - todo.len();
    if todo.is_empty() {
        println!("远端已有全部 {skipped} 个文件，大小一致，跳过");
        return Ok(0);
    }
    let missing: BTreeSet<String> = unlisted
        .into_iter()
        .map(|(d, e)| {
            if e.contains("object not found") {
                println!("note: 远端还没有 {d}，该目录的文件全部上传");
            } else {
                println!("note: 远端列不出 {d}（{e}），按空目录处理");
            }
            d
        })
        .collect();
    let mode = match jobs {
        1 => "串行".to_string(),
        n => format!("{n} 路并发"),
    };
    println!(
        "上传 {} 个（{mode}），远端已有且大小一致、跳过 {skipped} 个",
        todo.len()
    );

    // The first file into a directory the remote lacks goes alone, so that
    // the concurrent puts find the directory instead of all creating it.
    let (seeds, rest) = split_seeds(&files, &todo, &missing);
    let done = AtomicUsize::new(0);
    let put = |&i: &usize| {
        let r = put_with_retry(ol, &entries[i], &files[i].0);
        let k = done.fetch_add(1, Ordering::Relaxed) + 1;
        match &r {
            Ok(()) => println!("put [{k}/{}]: {}", todo.len(), entries[i].rel),
            Err(err) => println!("error: {}: {err}", entries[i].rel),
        }
        r.is_ok()
    };
    let seeds_ok: Vec<bool> = seeds.iter().map(&put).collect();
    let (rest, blocked) = drop_dead_dirs(&files, &seeds, &seeds_ok, rest);
    if !blocked.is_empty() {
        println!(
            "error: 有新目录的首个文件没传上，目录多半没建起来；同目录其余 {} 个文件不再尝试",
            blocked.len()
        );
    }
    let rest_ok = run_pool(&rest, jobs, put);

    let mut sent = Vec::new();
    let mut failed = blocked.len();
    for (&i, ok) in seeds
        .iter()
        .chain(&rest)
        .zip(seeds_ok.into_iter().chain(rest_ok))
    {
        if ok {
            sent.push(i);
        } else {
            failed += 1;
        }
    }
    let uploaded = sent.len();
    let bad = verify(ol, &entries, &files, sent);
    failed += bad;
    println!(
        "已上传并核对 {} 个，跳过 {skipped} 个，失败 {failed} 个",
        uploaded - bad
    );
    Ok(failed)
}

/// Reads the file once and puts it, up to `PUT_ATTEMPTS` tries.
fn put_with_retry(ol: &OpenList, e: &Entry, remote: &str) -> Result<(), String> {
    let bytes = fs::read(&e.abs).map_err(|err| format!("读取失败：{err}"))?;
    let mut attempt = 1;
    loop {
        match ol.put(remote, &bytes) {
            Ok(()) => return Ok(()),
            Err(err) if attempt == PUT_ATTEMPTS => {
                return Err(format!("上传 {PUT_ATTEMPTS} 次都失败，最后一次：{err}"));
            }
            Err(err) => {
                println!("retry: {}（第 {attempt} 次上传失败：{err}）", e.rel);
                std::thread::sleep(RETRY_WAIT * attempt);
                attempt += 1;
            }
        }
    }
}

/// Re-lists the directories of the uploaded files `sent` (indices into
/// `files`) and reports every file whose remote size is not the local one.
/// Returns how many.
fn verify(ol: &OpenList, entries: &[Entry], files: &[(String, u64)], sent: Vec<usize>) -> usize {
    let mut pending = sent;
    let mut round = 0;
    let (remote, unlisted) = loop {
        let (remote, unlisted) = ol.snapshot(parents(files, pending.iter().copied()));
        pending = unmatched(files, &remote, pending);
        // Only an absent file is worth waiting for; a wrong size stays wrong.
        if round == VERIFY_RETRIES || pending.iter().all(|&i| remote.contains_key(&files[i].0)) {
            break (remote, unlisted);
        }
        round += 1;
        std::thread::sleep(VERIFY_WAIT);
    };
    for &i in &pending {
        match remote.get(&files[i].0) {
            Some(s) => println!(
                "error: {}: 上传后远端 {s} 字节，本地 {} 字节",
                entries[i].rel, files[i].1
            ),
            None => println!("error: {}: 上传后远端查不到该文件", entries[i].rel),
        }
    }
    if !pending.is_empty() {
        for (d, e) in unlisted {
            println!("note: 核对时列不出 {d}（{e}）");
        }
    }
    pending.len()
}

/// The remote directories holding `files[i]` for each `i`, deduplicated.
fn parents(files: &[(String, u64)], idx: impl IntoIterator<Item = usize>) -> BTreeSet<&str> {
    idx.into_iter().map(|i| parent(&files[i].0)).collect()
}

fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(d, _)| d)
}

/// The indices among `idx` whose file is absent from `remote` or has another
/// size there, in the order given.
fn unmatched(
    files: &[(String, u64)],
    remote: &HashMap<String, u64>,
    idx: impl IntoIterator<Item = usize>,
) -> Vec<usize> {
    idx.into_iter()
        .filter(|&i| remote.get(&files[i].0) != Some(&files[i].1))
        .collect()
}

/// Splits `todo` into the first file of each directory in `missing`, to be
/// sent one at a time, and the rest; both keep `todo`'s order.
fn split_seeds(
    files: &[(String, u64)],
    todo: &[usize],
    missing: &BTreeSet<String>,
) -> (Vec<usize>, Vec<usize>) {
    let mut seeded = BTreeSet::new();
    todo.iter().partition(|&&i| {
        let d = parent(&files[i].0);
        missing.contains(d) && seeded.insert(d)
    })
}

/// Splits `rest` into the files to send and those in a directory whose seed
/// failed: that directory was most likely not created, and each of its files
/// would fail the same way after three tries (1026 test run against a bad
/// root: 170 files, 6.5 minutes of retries).
fn drop_dead_dirs(
    files: &[(String, u64)],
    seeds: &[usize],
    seeds_ok: &[bool],
    rest: Vec<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let dead: BTreeSet<&str> = seeds
        .iter()
        .zip(seeds_ok)
        .filter(|&(_, &ok)| !ok)
        .map(|(&i, _)| parent(&files[i].0))
        .collect();
    rest.into_iter()
        .partition(|&i| !dead.contains(parent(&files[i].0)))
}

/// Runs `f` on every item with up to `jobs` threads; results come back in
/// input order. Each worker claims the next unclaimed index, so a slow file
/// holds up only its own thread.
fn run_pool<T: Sync, R: Send>(items: &[T], jobs: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    if items.is_empty() {
        return Vec::new();
    }
    let next = AtomicUsize::new(0);
    let mut out: Vec<(usize, R)> = std::thread::scope(|s| {
        let workers: Vec<_> = (0..jobs.clamp(1, items.len()))
            .map(|_| {
                s.spawn(|| {
                    let mut done = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(i) else { break };
                        done.push((i, f(item)));
                    }
                    done
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().unwrap_or_else(|p| std::panic::resume_unwind(p)))
            .collect()
    });
    out.sort_by_key(|&(i, _)| i);
    out.into_iter().map(|(_, r)| r).collect()
}
/// `docs/<YYMM>/<MMDD>` relative to the repo root, both four ASCII digits.
fn issue_key(root: &Path, dir: &Path) -> Result<(String, String), String> {
    let abs = dir
        .canonicalize()
        .map_err(|e| format!("{}: {e}", dir.display()))?;
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let rel = abs
        .strip_prefix(&root)
        .map_err(|_| format!("{} 不在仓库内", dir.display()))?;
    let parts: Vec<&str> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    match parts.as_slice() {
        ["docs", m, d] if crate::is_four_digits(m) && crate::is_four_digits(d) => {
            Ok((m.to_string(), d.to_string()))
        }
        _ => Err(format!(
            "{} 不是 docs/<YYMM>/<MMDD> 期次目录",
            rel.display()
        )),
    }
}

/// Files under the issue that match the offsite rules. `--cached` adds files
/// committed before the rules existed (0928 and earlier originals, older
/// backup images): they stay in git but are archived offsite too.
pub(crate) fn ignored_files(root: &Path, yymm: &str, mmdd: &str) -> Result<Vec<Entry>, String> {
    let prefix = format!("docs/{yymm}/{mmdd}/");
    let mut entries = Vec::new();
    for path in git_ls_files(
        root,
        &["--cached", "--others", "--ignored", "--exclude-standard"],
        &prefix,
    )? {
        let Some(rel) = path.strip_prefix(&prefix) else {
            continue;
        };
        let abs = root.join(&path);
        let bytes = fs::read(&abs).map_err(|e| format!("{path}: {e}"))?;
        entries.push(Entry {
            rel: rel.to_string(),
            abs,
            size: bytes.len() as u64,
            sha256: Sha256::digest(&bytes)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect(),
        });
    }
    entries.sort_by(|a, b| a.rel.cmp(&b.rel));
    Ok(entries)
}

/// `git ls-files -z <flags> -- <pathspec>`, as repo-relative `/` paths.
pub(crate) fn git_ls_files(
    root: &Path,
    flags: &[&str],
    pathspec: &str,
) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .current_dir(root)
        .arg("ls-files")
        .args(flags)
        .args(["-z", "--", pathspec])
        .output()
        .map_err(|e| format!("无法运行 git：{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git ls-files 失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    out.stdout
        .split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|raw| String::from_utf8(raw.to_vec()).map_err(|_| "git 输出了非 UTF-8 路径".into()))
        .collect()
}

/// Write `text` unless the file already holds it; the working copy may have
/// CRLF (`core.autocrlf`), so compare with line endings folded.
pub(crate) fn write_if_changed(path: &Path, text: &str, what: &str) -> Result<(), String> {
    let current = fs::read_to_string(path)
        .unwrap_or_default()
        .replace("\r\n", "\n");
    if current == text {
        println!("{} 未变（{what}）", path.display());
    } else {
        fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("写入 {}（{what}）", path.display());
    }
    Ok(())
}

/// Rows of an `offsite.tsv`: `(issue-relative path, bytes, sha256)`.
/// Comment lines and malformed rows are skipped.
pub(crate) fn parse_manifest(text: &str) -> Vec<(String, u64, String)> {
    text.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let mut f = l.split('\t');
            let path = f.next()?.trim();
            let size = f.next()?.trim().parse().ok()?;
            let sha = f.next()?.trim();
            (!path.is_empty()).then(|| (path.to_string(), size, sha.to_string()))
        })
        .collect()
}

pub(crate) fn write_manifest(issue: &Path, entries: &[Entry]) -> Result<(), String> {
    let mut text = format!("{MANIFEST_HEADER}\n");
    for e in entries {
        text.push_str(&format!("{}\t{}\t{}\n", e.rel, e.size, e.sha256));
    }
    write_if_changed(
        &issue.join("sources").join(MANIFEST),
        &text,
        &format!("{} 个文件", entries.len()),
    )
}

/// Minimal OpenList API client: `/api/auth/login`, `/api/fs/list`, `/api/fs/put`
/// (OpenListTeam/OpenList `server/router.go`, `server/handles/fsread.go`,
/// `server/handles/fsup.go`). Shared by reference across the upload threads:
/// `ureq::Agent` is `Send + Sync` and pools connections internally.
pub(crate) struct OpenList {
    agent: ureq::Agent,
    base: String,
    token: String,
    /// Archive root: leading `/`, no trailing `/` (empty for the user's base path).
    root: String,
}

impl OpenList {
    fn login(repo: &Path, jobs: usize) -> Result<Self, String> {
        let file = load_env(repo);
        let get = |k: &str| {
            std::env::var(k)
                .ok()
                .or_else(|| file.get(k).cloned())
                .filter(|v| !v.is_empty())
                .ok_or_else(|| format!("缺少 {k}（写入 .env，键名见 .env.example）"))
        };
        let base = get("OPENLIST_URL")?.trim_end_matches('/').to_string();
        let username = get("OPENLIST_USERNAME")?;
        let password = get("OPENLIST_PASSWORD")?;
        let root = match get("OPENLIST_ROOT")?.trim().trim_matches('/') {
            "" => String::new(),
            r => format!("/{r}"),
        };
        // Status codes are read from the JSON `code` field, not the HTTP status.
        // Keep one idle connection per upload thread (ureq keeps 3 per host),
        // so the threads do not redo the ~0.3 s TLS handshake on every file.
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(900)))
            .max_idle_connections(jobs.max(10))
            .max_idle_connections_per_host(jobs.max(3))
            .build()
            .into();
        let v = api(agent
            .post(format!("{base}/api/auth/login").as_str())
            .send_json(json!({ "username": username, "password": password })))
        .map_err(|e| format!("OpenList 登录失败：{e}"))?;
        let token = v["data"]["token"]
            .as_str()
            .ok_or("OpenList 登录响应缺少 token")?
            .to_string();
        Ok(Self {
            agent,
            base,
            token,
            root,
        })
    }

    /// Remote file sizes under `dirs` keyed by full path, and the directories
    /// that could not be listed (absent, or any other error) with the reason.
    fn snapshot<'a>(
        &self,
        dirs: impl IntoIterator<Item = &'a str>,
    ) -> (HashMap<String, u64>, Vec<(String, String)>) {
        let mut sizes = HashMap::new();
        let mut unlisted = Vec::new();
        for d in dirs {
            match self.list(d) {
                Ok(m) => sizes.extend(m),
                Err(e) => unlisted.push((d.to_string(), e)),
            }
        }
        (sizes, unlisted)
    }

    /// Sizes of the files directly under `dir`. `per_page: 0` means all
    /// entries (`model.PageReq.Validate`); no `refresh`, so this reads
    /// OpenList's directory cache, which a put updates.
    fn list(&self, dir: &str) -> Result<HashMap<String, u64>, String> {
        let v = api(self
            .agent
            .post(format!("{}/api/fs/list", self.base).as_str())
            .header("Authorization", self.token.as_str())
            .send_json(json!({ "path": dir, "password": "", "page": 1, "per_page": 0 })))?;
        Ok(file_sizes(dir, &v["data"]["content"]))
    }

    fn put(&self, path: &str, body: &[u8]) -> Result<(), String> {
        api(self
            .agent
            .put(format!("{}/api/fs/put", self.base).as_str())
            .header("Authorization", self.token.as_str())
            .header("File-Path", encode_path(path).as_str())
            .header("As-Task", "false")
            .content_type("application/octet-stream")
            .send(body))
        .map(|_| ())
    }
}

/// Files (not subdirectories) of an `fs/list` `content` array, keyed by full
/// path; `null` content is an empty directory.
fn file_sizes(dir: &str, content: &Value) -> HashMap<String, u64> {
    content
        .as_array()
        .into_iter()
        .flatten()
        .filter(|o| o["is_dir"].as_bool() != Some(true))
        .filter_map(|o| {
            Some((
                format!("{dir}/{}", o["name"].as_str()?),
                o["size"].as_u64()?,
            ))
        })
        .collect()
}

/// OpenList answers `{"code": 200, "message": ..., "data": ...}`; anything
/// else is an error carrying its message.
fn api(r: Result<ureq::http::Response<ureq::Body>, ureq::Error>) -> Result<Value, String> {
    let mut resp = r.map_err(|e| format!("请求失败：{e}"))?;
    let status = resp.status();
    let v: Value = resp
        .body_mut()
        .read_json()
        .map_err(|e| format!("HTTP {status}，响应不是 JSON：{e}"))?;
    match v["code"].as_i64() {
        Some(200) => Ok(v),
        code => Err(format!(
            "HTTP {status}，code {}：{}",
            code.map_or("?".into(), |c| c.to_string()),
            v["message"].as_str().unwrap_or("")
        )),
    }
}

/// `File-Path` is URL-unescaped by the server; keep `/` and RFC 3986
/// unreserved bytes, percent-encode the rest (including all non-ASCII).
fn encode_path(p: &str) -> String {
    let mut s = String::with_capacity(p.len());
    for b in p.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~/".contains(&b) {
            s.push(b as char);
        } else {
            s.push_str(&format!("%{b:02X}"));
        }
    }
    s
}

/// `KEY=VALUE` lines from the repo's `.env`; `#` comments and blank lines are
/// skipped, one pair of surrounding double quotes is stripped.
pub(crate) fn load_env(root: &Path) -> HashMap<String, String> {
    fs::read_to_string(root.join(".env"))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                return None;
            }
            let (k, v) = l.split_once('=')?;
            let v = v.trim();
            let v = v
                .strip_prefix('"')
                .and_then(|x| x.strip_suffix('"'))
                .unwrap_or(v);
            Some((k.trim().to_string(), v.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_non_ascii_and_spaces() {
        assert_eq!(
            encode_path("/AI-Barking/2610/1001/a b.pdf"),
            "/AI-Barking/2610/1001/a%20b.pdf"
        );
        assert_eq!(encode_path("/吠"), "/%E5%90%A0");
    }

    fn args(v: &[&str]) -> Result<Opts, String> {
        parse_args(&v.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn jobs_default_to_four_and_need_upload() {
        let o = args(&["docs/2610/1026", "--upload"]).unwrap();
        assert_eq!((o.upload, o.jobs), (true, DEFAULT_JOBS));
        assert_eq!(args(&["d", "--upload", "--jobs", "1"]).unwrap().jobs, 1);
        assert_eq!(args(&["--jobs=8", "--upload", "d"]).unwrap().jobs, 8);
        assert_eq!(args(&["a", "b"]).unwrap().dirs, ["a", "b"]);
        for bad in [
            &["d", "--upload", "--jobs", "0"][..],
            &["d", "--upload", "--jobs=17"],
            &["d", "--upload", "--jobs", "x"],
            &["d", "--upload", "--jobs"],
            &["d", "--jobs", "2"],
            &["d", "--force"],
            &["--upload"],
        ] {
            assert!(args(bad).is_err(), "{bad:?}");
        }
    }

    fn files(v: &[(&str, u64)]) -> Vec<(String, u64)> {
        v.iter().map(|&(p, n)| (p.to_string(), n)).collect()
    }

    #[test]
    fn skips_only_files_present_with_the_same_size() {
        let f = files(&[
            ("/R/2610/1026/images/a.png", 10),
            ("/R/2610/1026/images/b.png", 20),
            ("/R/2610/1026/sources/c.pdf", 30),
            ("/R/2610/1026/sources/d/e.html", 40),
        ]);
        let remote: HashMap<String, u64> = [
            ("/R/2610/1026/images/a.png".to_string(), 10),
            ("/R/2610/1026/images/b.png".to_string(), 21),
        ]
        .into();
        assert_eq!(unmatched(&f, &remote, 0..f.len()), [1, 2, 3]);
        assert_eq!(unmatched(&f, &remote, vec![3, 0]), [3]);
        let dirs: Vec<&str> = parents(&f, 0..f.len()).into_iter().collect();
        assert_eq!(
            dirs,
            [
                "/R/2610/1026/images",
                "/R/2610/1026/sources",
                "/R/2610/1026/sources/d"
            ]
        );
    }

    #[test]
    fn the_publish_folder_is_its_own_remote_directory() {
        // 1029 on: the upload set sits in images/publish/, next to the
        // screenshots and raw bases in images/.
        let f = files(&[
            ("/R/2610/1029/images/carbon-raw.png", 5),
            ("/R/2610/1029/images/publish/00-cover.png", 6),
            ("/R/2610/1029/images/publish/01-tldr.png", 7),
        ]);
        let dirs: Vec<&str> = parents(&f, 0..f.len()).into_iter().collect();
        assert_eq!(dirs, ["/R/2610/1029/images", "/R/2610/1029/images/publish"]);
        // A listing of images/ alone does not vouch for what is in publish/.
        let remote: HashMap<String, u64> =
            [("/R/2610/1029/images/carbon-raw.png".to_string(), 5)].into();
        assert_eq!(unmatched(&f, &remote, 0..f.len()), [1, 2]);
        // The directory is created by its first file, alone.
        let missing: BTreeSet<String> = ["/R/2610/1029/images/publish".into()].into();
        let (seeds, rest) = split_seeds(&f, &[1, 2], &missing);
        assert_eq!((seeds, rest), (vec![1], vec![2]));
    }

    #[test]
    fn first_file_of_each_missing_directory_goes_alone() {
        let f = files(&[
            ("/R/1027/images/a.png", 1),
            ("/R/1027/images/b.png", 1),
            ("/R/1027/sources/c.pdf", 1),
            ("/R/1027/sources/d.pdf", 1),
            ("/R/old/x.png", 1),
            ("/R/old/y.png", 1),
        ]);
        let missing: BTreeSet<String> = ["/R/1027/images".into(), "/R/1027/sources".into()].into();
        let (seeds, rest) = split_seeds(&f, &[0, 1, 2, 3, 4, 5], &missing);
        assert_eq!((seeds, rest), (vec![0, 2], vec![1, 3, 4, 5]));
        let (seeds, rest) = split_seeds(&f, &[3, 1, 5], &missing);
        assert_eq!((seeds, rest), (vec![3, 1], vec![5]));

        // The images seed failed: its siblings are dropped, the rest go.
        let (go, blocked) = drop_dead_dirs(&f, &[0, 2], &[false, true], vec![1, 3, 4, 5]);
        assert_eq!((go, blocked), (vec![3, 4, 5], vec![1]));
        let (go, blocked) = drop_dead_dirs(&f, &[], &[], vec![1, 3]);
        assert_eq!((go, blocked), (vec![1, 3], vec![]));
    }

    #[test]
    fn pool_runs_each_item_once_and_keeps_order() {
        let items: Vec<u64> = (0..97).collect();
        for jobs in [1, 4, 200] {
            let calls = AtomicUsize::new(0);
            let out = run_pool(&items, jobs, |&x| {
                calls.fetch_add(1, Ordering::Relaxed);
                // Uneven work, so threads finish out of order.
                std::thread::sleep(Duration::from_micros((x * 37) % 500));
                x * 2
            });
            assert_eq!(calls.load(Ordering::Relaxed), items.len(), "jobs {jobs}");
            assert_eq!(out, items.iter().map(|x| x * 2).collect::<Vec<_>>());
        }
        assert!(run_pool(&[] as &[u8], 4, |_| 0).is_empty());
    }

    #[test]
    fn listing_keeps_files_only() {
        let content = json!([
            { "name": "a.png", "size": 5, "is_dir": false },
            { "name": "sub", "size": 0, "is_dir": true },
            { "name": "b c.pdf", "size": 7, "is_dir": false },
        ]);
        let m = file_sizes("/R/x", &content);
        assert_eq!(m.len(), 2);
        assert_eq!(m["/R/x/a.png"], 5);
        assert_eq!(m["/R/x/b c.pdf"], 7);
        assert!(file_sizes("/R/x", &Value::Null).is_empty());
    }
}
