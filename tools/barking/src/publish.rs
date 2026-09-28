//! `barking publish weibo`: post one issue to Weibo through the official
//! Weibo CLI (`weibo-cli`, https://open.weibo.com/cli).
//!
//! Dry-run by default. Only `--confirm` uploads and posts, and every real post
//! needs Develata's explicit go-ahead in chat first (see EDITORIAL.md).
//! Authentication lives in weibo-cli's own config; this tool never sees tokens.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::lint::{self, Level};

/// Weibo accepts at most 9 image ids per post (`upload_url_text --pic_id`).
const MAX_PICS: usize = 9;
/// `upload_pic`: JPEG, GIF or PNG, smaller than 10 MB.
const MAX_PIC_BYTES: u64 = 10 * 1024 * 1024;
const PIC_EXTS: [&str; 4] = ["png", "jpg", "jpeg", "gif"];
/// Long posts (`is_longtext=1`) allow up to 2000 characters.
const MAX_TEXT_CHARS: usize = 2000;

pub struct Options {
    pub dir: PathBuf,
    pub confirm: bool,
    /// `visible=1`: only the account itself can see the post (test runs).
    pub private: bool,
}

pub struct Plan {
    pub text: String,
    pub pics: Vec<PathBuf>,
}

/// Build the post from an issue directory: the single `*_publish.txt`, then
/// the 3:4 cover followed by the “正式配图” table of `images/README.md` in
/// upload order. Fails on lint errors so nothing unverified goes out.
pub fn plan(dir: &Path) -> Result<Plan, String> {
    let errors: Vec<String> = lint::lint_issue(dir)
        .into_iter()
        .filter(|f| f.level == Level::Error)
        .map(|f| f.to_string())
        .collect();
    if !errors.is_empty() {
        return Err(format!("lint 有错误，拒绝发布：\n{}", errors.join("\n")));
    }

    let publish: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with("_publish.txt"))
        })
        .collect();
    let [publish] = publish.as_slice() else {
        return Err(format!(
            "需要恰好一个 *_publish.txt，找到 {} 个",
            publish.len()
        ));
    };
    let raw = fs::read_to_string(publish).map_err(|e| format!("{}: {e}", publish.display()))?;
    let text = lint::normalize(&raw);
    let n = lint::char_count(&text);
    if n > MAX_TEXT_CHARS {
        return Err(format!("正文 {n} 字，超过微博长文上限 {MAX_TEXT_CHARS}"));
    }

    let images = dir.join("images");
    let readme = images.join("README.md");
    let md = fs::read_to_string(&readme).map_err(|e| format!("{}: {e}", readme.display()))?;
    let mut pics = vec![cover(&images)?];
    for name in formal_images(&md) {
        pics.push(images.join(name));
    }
    if pics.len() > MAX_PICS {
        return Err(format!(
            "封面加正式配图共 {} 张，超过微博上限 {MAX_PICS}；按配图说明删图后再发",
            pics.len()
        ));
    }
    for p in &pics {
        check_pic(p)?;
    }
    Ok(Plan { text, pics })
}

fn cover(images: &Path) -> Result<PathBuf, String> {
    PIC_EXTS
        .iter()
        .map(|e| images.join(format!("00-cover.{e}")))
        .find(|p| p.is_file())
        .ok_or_else(|| "缺少 images/00-cover.{png,jpg,jpeg,gif}".to_string())
}

/// File names in the “## 正式配图” table, in row order: the first image
/// reference on each row whose first cell is a number.
pub fn formal_images(md: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in md.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            inside = h.trim().starts_with("正式配图");
            continue;
        }
        if !inside {
            continue;
        }
        let mut cells = line.trim().strip_prefix('|').unwrap_or("").split('|');
        let first = cells.next().unwrap_or("").trim();
        if first.is_empty() || !first.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if let Some(name) = lint::image_refs_in_line(line).into_iter().next() {
            out.push(name);
        }
    }
    out
}

fn check_pic(p: &Path) -> Result<(), String> {
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if !PIC_EXTS.contains(&ext.as_str()) {
        return Err(format!("{}: 微博只收 JPEG、GIF、PNG", p.display()));
    }
    let len = fs::metadata(p)
        .map_err(|e| format!("{}: {e}", p.display()))?
        .len();
    if len >= MAX_PIC_BYTES {
        return Err(format!("{}: {len} 字节，微博要求小于 10 MB", p.display()));
    }
    Ok(())
}

pub fn run(opts: &Options) -> Result<(), String> {
    let plan = plan(&opts.dir)?;
    let visible = if opts.private { "1" } else { "0" };

    println!("== 微博发布预演：{}", opts.dir.display());
    println!(
        "正文 {} 字；可见性：{}；内容声明：AI 生成（mblog_statement=1）",
        lint::char_count(&plan.text),
        if opts.private {
            "仅自己可见"
        } else {
            "所有人"
        }
    );
    println!("图片 {} 张（按上传顺序）：", plan.pics.len());
    for (i, p) in plan.pics.iter().enumerate() {
        let kb = fs::metadata(p).map(|m| m.len() / 1024).unwrap_or(0);
        println!("  {}. {} ({kb} KB)", i + 1, p.display());
    }
    println!("---- 正文开始 ----\n{}\n---- 正文结束 ----", plan.text);

    if !opts.confirm {
        println!("预演结束，未上传、未发布。确认发布请加 --confirm。");
        return Ok(());
    }

    let mut ids = Vec::new();
    for p in &plan.pics {
        let pic = p.to_str().ok_or("图片路径不是 UTF-8")?;
        let out = weibo(&["statuses", "upload_pic", "--pic", pic])?;
        let id =
            json_str(&out, "pic_id").ok_or_else(|| format!("上传 {pic} 未返回 pic_id：\n{out}"))?;
        println!("已上传 {pic} → {id}");
        ids.push(id);
    }
    let out = weibo(&[
        "statuses",
        "upload_url_text",
        "--pic_id",
        &ids.join(","),
        "--status",
        &plan.text,
        "--is_longtext",
        "1",
        "--mblog_statement",
        "1",
        "--visible",
        visible,
    ])?;
    let id = json_str(&out, "idstr")
        .or_else(|| json_str(&out, "mid"))
        .or_else(|| json_num(&out, "id"));
    println!(
        "已发布：{}",
        id.as_deref()
            .unwrap_or("（未解析到微博 id，见下方原始返回）")
    );
    if id.is_none() {
        println!("{out}");
    }
    log_post(&opts.dir, id.as_deref(), opts.private, ids.len())
}

/// Run `weibo-cli … --output json --agent claude-code`; non-zero exit is an error.
fn weibo(args: &[&str]) -> Result<String, String> {
    let out = Command::new("weibo-cli")
        .args(args)
        .args(["--output", "json", "--agent", "claude-code"])
        .output()
        .map_err(|e| format!("无法运行 weibo-cli：{e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!(
            "weibo-cli {} 失败（{}）：\n{stdout}{stderr}",
            args[..2].join(" "),
            out.status
        ));
    }
    Ok(stdout)
}

/// First string value of `"key": "…"` in a JSON text (no escapes expected in ids).
pub fn json_str(json: &str, key: &str) -> Option<String> {
    let rest = after_key(json, key)?.strip_prefix('"')?;
    Some(rest[..rest.find('"')?].to_string())
}

/// First unsigned-integer value of `"key": 123`.
pub fn json_num(json: &str, key: &str) -> Option<String> {
    let rest = after_key(json, key)?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    (!digits.is_empty()).then_some(digits)
}

fn after_key<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\"");
    let mut from = 0;
    while let Some(off) = json[from..].find(&pat) {
        let rest = json[from + off + pat.len()..].trim_start();
        if let Some(v) = rest.strip_prefix(':') {
            return Some(v.trim_start());
        }
        from += off + pat.len();
    }
    None
}

/// Append one line to `sources/publish-log.md` so the archive records what went out.
fn log_post(dir: &Path, id: Option<&str>, private: bool, pics: usize) -> Result<(), String> {
    let path = dir.join("sources").join("publish-log.md");
    let new = !path.exists();
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if new {
        writeln!(
            f,
            "# 发布记录\n\n| 平台 | 微博 id | 可见性 | 图片数 |\n|---|---|---|---|"
        )
        .map_err(|e| e.to_string())?;
    }
    writeln!(
        f,
        "| 微博 | {} | {} | {pics} |",
        id.unwrap_or("未解析"),
        if private { "仅自己" } else { "公开" }
    )
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formal_images_follow_table_order_and_skip_backups() {
        let md = "\
## 正式配图（按上传顺序）

| 顺序 | 文件 | 位置 |
|---|---|---|
| 1 | `01-a.png` | x |
| 2 | `03b-b.png` | 与 `05-c.png` 相邻 |

## 备用图

| 文件 | 用途 |
|---|---|
| `07-d.png` | y |
";
        assert_eq!(formal_images(md), vec!["01-a.png", "03b-b.png"]);
    }

    #[test]
    fn json_helpers_find_first_value() {
        let j = r#"{"data":{"pic_id":"abc123","x":1},"id": 5012345678901234,"idstr":"5012345678901234"}"#;
        assert_eq!(json_str(j, "pic_id").as_deref(), Some("abc123"));
        assert_eq!(json_str(j, "idstr").as_deref(), Some("5012345678901234"));
        assert_eq!(json_num(j, "id").as_deref(), Some("5012345678901234"));
        assert_eq!(json_str(j, "missing"), None);
    }
}
