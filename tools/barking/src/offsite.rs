//! `barking offsite`: files that stay out of git are recorded in
//! `sources/offsite.tsv` and, with `--upload`, copied to OpenList under
//! `<OPENLIST_ROOT>/<YYMM>/<MMDD>/`: every image under `images/` and the PDFs,
//! page archives, screenshots and media under `sources/` (patterns in
//! `.gitignore`). Files are uploaded one by one, never zipped. Workflow:
//! docs/workflow/publish.md.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::Duration;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const MANIFEST: &str = "offsite.tsv";
const MANIFEST_HEADER: &str = "# 存于 OpenList 的文件（见 docs/workflow/publish.md；1002 期起图片与原件不入 Git，更早期次的图片与原件已于历史改写时移出 Git，仍在网盘）。列：相对期次目录的路径	字节数	SHA-256";
const USAGE: &str = "用法：barking offsite <期次目录>... [--upload]";

/// One git-ignored file of an issue. Owned strings: entries are few and are
/// used after the `git ls-files` output buffer is gone.
struct Entry {
    /// Path relative to the issue directory, `/`-separated.
    rel: String,
    abs: PathBuf,
    size: u64,
    sha256: String,
}

pub fn run(args: &[String]) -> ExitCode {
    let upload = args.iter().any(|a| a == "--upload");
    let dirs: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if dirs.is_empty() || args.iter().any(|a| a.starts_with("--") && a != "--upload") {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
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
    for d in dirs {
        println!("== {d}");
        match process(&root, Path::new(d), upload, &mut client) {
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

/// Returns the number of files that failed to upload.
fn process(
    root: &Path,
    dir: &Path,
    upload: bool,
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
        *client = Some(OpenList::login(root)?);
    }
    let Some(ol) = client.as_ref() else {
        unreachable!("client was just set")
    };
    let mut failed = 0;
    for e in &entries {
        let remote = format!("{}/{yymm}/{mmdd}/{}", ol.root, e.rel);
        match upload_one(ol, e, &remote) {
            Ok(msg) => println!("ok: {} {msg}", e.rel),
            Err(err) => {
                println!("error: {}: {err}", e.rel);
                failed += 1;
            }
        }
    }
    Ok(failed)
}

fn upload_one(ol: &OpenList, e: &Entry, remote: &str) -> Result<&'static str, String> {
    if ol.remote_size(remote) == Some(e.size) {
        return Ok("（远端已有，大小一致，跳过）");
    }
    let bytes = fs::read(&e.abs).map_err(|err| format!("读取失败：{err}"))?;
    ol.put(remote, &bytes)?;
    // Right after a put, fs/get can briefly miss the new file (0928 run,
    // 2026-10-01); retry a few times before calling it a failure.
    let mut size = ol.remote_size(remote);
    for _ in 0..3 {
        if size.is_some() {
            break;
        }
        std::thread::sleep(Duration::from_secs(2));
        size = ol.remote_size(remote);
    }
    match size {
        Some(s) if s == e.size => Ok("（已上传，远端大小一致）"),
        Some(s) => Err(format!("上传后远端 {s} 字节，本地 {} 字节", e.size)),
        None => Err("上传后远端查不到该文件".into()),
    }
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
fn ignored_files(root: &Path, yymm: &str, mmdd: &str) -> Result<Vec<Entry>, String> {
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
fn git_ls_files(root: &Path, flags: &[&str], pathspec: &str) -> Result<Vec<String>, String> {
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
fn write_if_changed(path: &Path, text: &str, what: &str) -> Result<(), String> {
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

fn write_manifest(issue: &Path, entries: &[Entry]) -> Result<(), String> {
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

/// Minimal OpenList API client: `/api/auth/login`, `/api/fs/get`, `/api/fs/put`
/// (OpenListTeam/OpenList `server/router.go`, `server/handles/fsup.go`).
struct OpenList {
    agent: ureq::Agent,
    base: String,
    token: String,
    /// Archive root: leading `/`, no trailing `/` (empty for the user's base path).
    root: String,
}

impl OpenList {
    fn login(repo: &Path) -> Result<Self, String> {
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
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(900)))
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

    /// Size of a remote file, or `None` if it does not exist or cannot be read.
    fn remote_size(&self, path: &str) -> Option<u64> {
        let v = api(self
            .agent
            .post(format!("{}/api/fs/get", self.base).as_str())
            .header("Authorization", self.token.as_str())
            .send_json(json!({ "path": path, "password": "" })))
        .ok()?;
        if v["data"]["is_dir"].as_bool() == Some(true) {
            return None;
        }
        v["data"]["size"].as_u64()
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
fn load_env(root: &Path) -> HashMap<String, String> {
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
}
