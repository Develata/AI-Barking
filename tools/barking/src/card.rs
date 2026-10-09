//! `barking card`: render one issue's 省流卡, 速览聚合图 and annotated
//! screenshots (plus the template covers of a 速览专帖) from
//! `images/cards.toml`, filling the HTML templates in `templates/cards/` and
//! screenshotting them with headless Chrome.
//!
//! Highlights come from Tesseract word boxes found by matching the quoted
//! sentence. A missing, ambiguous, partial-word or low-confidence match is an
//! error, never a guess: a misplaced highlight on evidence puts words in the
//! source's mouth. Layout overflow (text cut off, screenshot squeezed) is
//! detected in the page and is an error too.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::SystemTime;

use serde::Deserialize;

const TLDR_TEMPLATE: &str = include_str!("../../../templates/cards/tldr.html");
const ANNOT_TEMPLATE: &str = include_str!("../../../templates/cards/annot.html");
const ROUNDUP_TEMPLATE: &str = include_str!("../../../templates/cards/roundup.html");
const COVER_TEMPLATE: &str = include_str!("../../../templates/cards/cover.html");
pub const SPEC_FILE: &str = "cards.toml";
pub const TLDR_OUT: &str = "00-tldr.png";
pub const ROUNDUP_OUT: &str = "00-roundup.png";
pub const COVER_OUT: &str = "00-cover.png";
pub const COVER_WIDE_OUT: &str = "00-cover-wide.png";
/// Mascot on the template covers, relative to the repo root.
const AVATAR: &str = "common_images/profile_picture.png";
/// 速览 entries per issue, main-post entries included (EDITORIAL.md 速览).
pub const ROUNDUP_MIN: usize = 5;
pub const ROUNDUP_MAX: usize = 10;
/// Character caps (every scalar counts) that keep ten entries legible at
/// 36 px on one card: a main-post title stays on one line, others on two.
const ROUNDUP_MAIN_MAX: usize = 20;
const ROUNDUP_TEXT_MAX: usize = 40; // EDITORIAL.md 速览：其余条目 ≤40 字
const TAG_MAX: usize = 12;
/// Cards and the 3:4 cover; the wide cover has its own size.
const CARD_SIZE: (u32, u32) = (1080, 1440);
const WIDE_SIZE: (u32, u32) = (1880, 800);
/// Highlight colours defined in annot.html (`.c1` … `.c4`).
const MAX_NOTES: usize = 4;
/// Below this Tesseract confidence a matched word is not trusted.
const MIN_CONF: f32 = 60.0;

/// What the issue publishes. A 速览专帖 has no main post: no 省流卡, and its
/// covers come from the fixed template instead of image generation.
#[derive(Deserialize, Default, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Main,
    Roundup,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    #[serde(default)]
    pub kind: Kind,
    pub date: Option<String>,
    #[serde(default)]
    pub tldr: Vec<TldrItem>,
    #[serde(default)]
    pub roundup: Vec<RoundupItem>,
    #[serde(default)]
    pub annot: Vec<Annot>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoundupItem {
    pub tag: String,
    pub text: String,
    /// A main-post entry: a short title, verbatim from the publish text,
    /// pointing back to the 省流卡.
    #[serde(default)]
    pub main: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TldrItem {
    pub tag: String,
    pub fact: String,
    /// Retired 2026-10-03: images carry no 吠点. Still parsed so a published
    /// issue's cards.toml (which records what its images said) loads; rendering
    /// such an entry is an error.
    #[serde(default)]
    pub barks: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Annot {
    pub out: String,
    pub title: String,
    pub source: String,
    #[serde(default = "default_lang")]
    pub lang: String,
    /// Retired like `TldrItem::barks`.
    pub bark: Option<String>,
    pub note: Vec<Note>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Note {
    pub quote: Option<String>,
    /// Hand-measured fallback in raw-image pixels, `[x, y, w, h]` per line.
    pub rects: Option<Vec<[u32; 4]>>,
    pub gloss: String,
    pub emph: Option<String>,
    pub underline: Option<String>,
}

fn default_lang() -> String {
    "eng".into()
}

impl Spec {
    /// Card texts that must appear verbatim in the publish text. Other 速览
    /// entries are not in the body; lint checks them against fact-check.md.
    pub fn quoted_texts(&self) -> Vec<&str> {
        self.tldr
            .iter()
            .flat_map(|t| {
                std::iter::once(t.fact.as_str()).chain(t.barks.iter().map(String::as_str))
            })
            .chain(self.annot.iter().filter_map(|a| a.bark.as_deref()))
            .chain(self.main_entries().map(|r| r.text.as_str()))
            .collect()
    }

    /// Every text the rendered cards show that the account wrote itself (not
    /// the screenshots): checked for platform names that trigger 风控.
    pub fn own_texts(&self) -> Vec<&str> {
        let tldr = self.tldr.iter().flat_map(|t| [t.tag.as_str(), &t.fact]);
        let roundup = self.roundup.iter().flat_map(|r| [r.tag.as_str(), &r.text]);
        let annot = self.annot.iter().flat_map(|a| {
            [a.title.as_str(), &a.source]
                .into_iter()
                .chain(a.note.iter().map(|n| n.gloss.as_str()))
        });
        tldr.chain(roundup).chain(annot).collect()
    }

    pub fn main_entries(&self) -> impl Iterator<Item = &RoundupItem> {
        self.roundup.iter().filter(|r| r.main)
    }

    pub fn other_entries(&self) -> impl Iterator<Item = &RoundupItem> {
        self.roundup.iter().filter(|r| !r.main)
    }

    /// Violations of the card rules decidable from cards.toml alone. Shared by
    /// `barking card` (refuses to render) and `barking lint` (errors).
    pub fn structure_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.kind == Kind::Roundup {
            if !self.tldr.is_empty() {
                errs.push("kind = \"roundup\"（速览专帖）没有主帖，不应有 [[tldr]]".into());
            }
            if self.roundup.is_empty() {
                errs.push("kind = \"roundup\" 需要 [[roundup]] 条目".into());
            }
            if self.roundup.iter().any(|r| r.main) {
                errs.push("速览专帖没有主帖，[[roundup]] 不应写 main = true".into());
            }
        }
        if self.roundup.is_empty() {
            return errs;
        }
        let n = self.roundup.len();
        if !(ROUNDUP_MIN..=ROUNDUP_MAX).contains(&n) {
            errs.push(format!(
                "速览共 {n} 条，应为 {ROUNDUP_MIN}–{ROUNDUP_MAX} 条（主帖条目也算）"
            ));
        }
        // Main-post entries come first, in the main post's order.
        let lead = self.roundup.iter().take_while(|r| r.main).count();
        if self.main_entries().count() != lead {
            errs.push("main = true 的主帖条目须连续排在 [[roundup]] 最前".into());
        }
        if self.kind == Kind::Main {
            let main_tags: Vec<&str> = self.main_entries().map(|r| r.tag.as_str()).collect();
            let tldr_tags: Vec<&str> = self.tldr.iter().map(|t| t.tag.as_str()).collect();
            if main_tags != tldr_tags {
                errs.push(format!(
                    "速览的主帖条目 tag {main_tags:?} 须与省流卡 [[tldr]] 的 tag {tldr_tags:?} 一一对应、顺序相同"
                ));
            }
        }
        for r in &self.roundup {
            let len = r.text.chars().count();
            let max = if r.main {
                ROUNDUP_MAIN_MAX
            } else {
                ROUNDUP_TEXT_MAX
            };
            if r.text.trim().is_empty() || r.tag.trim().is_empty() {
                errs.push(format!("速览条目「{}」的 tag 或 text 为空", r.tag));
            } else if len > max {
                errs.push(format!("速览条目「{}」{len} 字，超过上限 {max}", r.text));
            }
            if r.tag.chars().count() > TAG_MAX {
                errs.push(format!("速览 tag「{}」超过 {TAG_MAX} 字", r.tag));
            }
        }
        errs
    }

    /// Where the retired 吠点 fields are still filled in, if anywhere.
    fn retired_barks(&self) -> Option<String> {
        self.tldr
            .iter()
            .find(|t| !t.barks.is_empty())
            .map(|t| format!("[[tldr]]「{}」的 barks", t.tag))
            .or_else(|| {
                (self.annot.iter().find(|a| a.bark.is_some())).map(|a| format!("{} 的 bark", a.out))
            })
    }

    /// Image file names this spec renders.
    pub fn outputs(&self) -> Vec<&str> {
        let covers: &[&str] = match self.kind {
            Kind::Roundup => &[COVER_OUT, COVER_WIDE_OUT],
            Kind::Main => &[],
        };
        let tldr = (!self.tldr.is_empty()).then_some(TLDR_OUT);
        let roundup = (!self.roundup.is_empty()).then_some(ROUNDUP_OUT);
        covers
            .iter()
            .copied()
            .chain(tldr)
            .chain(roundup)
            .chain(self.annot.iter().map(|a| a.out.as_str()))
            .collect()
    }
}

/// `Ok(None)` when the issue has no `images/cards.toml`.
pub fn load_spec(images: &Path) -> Result<Option<Spec>, String> {
    let path = images.join(SPEC_FILE);
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    toml::from_str(&text)
        .map(Some)
        .map_err(|e| format!("{}: {e}", path.display()))
}

pub fn run(args: &[String]) -> ExitCode {
    let Some((dir, only)) = args.split_first() else {
        eprintln!("用法：barking card <期次目录> [输出文件名...]");
        return ExitCode::from(2);
    };
    match render_issue(Path::new(dir), only) {
        Ok(n) => {
            println!("-- 已渲染 {n} 张。逐张打开核对，再按 templates/cards/README.md 做视觉复核");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn render_issue(dir: &Path, only: &[String]) -> Result<usize, String> {
    let images = dir.join("images");
    let spec = load_spec(&images)?.ok_or_else(|| {
        format!(
            "{} 不存在（样例见 templates/cards/example.toml）",
            images.join(SPEC_FILE).display()
        )
    })?;
    if let Some(at) = spec.retired_barks() {
        return Err(format!(
            "{at} 已停用：2026-10-03 起图上不放吠点（只留事实、原文批注与译注），从 cards.toml 删去"
        ));
    }
    let errs = spec.structure_errors();
    if !errs.is_empty() {
        return Err(errs.join("\n"));
    }
    let outputs = spec.outputs();
    for out in &outputs {
        if out.contains(['/', '\\']) || !out.ends_with(".png") {
            return Err(format!(
                "输出名「{out}」须为 images/ 下的 .png 文件名，不含路径"
            ));
        }
    }
    let unknown: Vec<&String> = only
        .iter()
        .filter(|o| !outputs.contains(&o.as_str()))
        .collect();
    if !unknown.is_empty() {
        return Err(format!("cards.toml 中没有这些输出：{unknown:?}"));
    }
    let chrome = find_chrome()?;
    // Per-process scratch, so concurrent runs don't overwrite each other.
    let work = std::env::temp_dir()
        .join("barking-card")
        .join(std::process::id().to_string());
    fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let wanted = |name: &str| only.is_empty() || only.iter().any(|o| o == name);

    let mut n = 0;
    if spec.kind == Kind::Roundup {
        let avatar = file_url(&repo_root(dir)?.join(AVATAR))?;
        for (out, size) in [(COVER_OUT, CARD_SIZE), (COVER_WIDE_OUT, WIDE_SIZE)] {
            if wanted(out) {
                let html = cover_html(&spec, size, &avatar)?;
                render(&chrome, &work, &html, size, &images.join(out))?;
                n += 1;
            }
        }
    }
    if !spec.tldr.is_empty() && wanted(TLDR_OUT) {
        let html = tldr_html(&spec)?;
        render(&chrome, &work, &html, CARD_SIZE, &images.join(TLDR_OUT))?;
        n += 1;
    }
    if !spec.roundup.is_empty() && wanted(ROUNDUP_OUT) {
        let html = roundup_html(&spec)?;
        render(&chrome, &work, &html, CARD_SIZE, &images.join(ROUNDUP_OUT))?;
        n += 1;
    }
    for a in spec.annot.iter().filter(|a| wanted(&a.out)) {
        let html = annot_html(a, &images, &work).map_err(|e| format!("{}：{e}", a.out))?;
        render(&chrome, &work, &html, CARD_SIZE, &images.join(&a.out))?;
        n += 1;
    }
    if n == 0 {
        return Err("cards.toml 里没有要渲染的卡片".into());
    }
    Ok(n)
}

fn tldr_html(spec: &Spec) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[tldr]] 但缺少 date")?;
    let mut items = String::new();
    for (i, t) in spec.tldr.iter().enumerate() {
        // Colour cycles through the four highlighter colours (.c1 … .c4), as the
        // note markers on annotated screenshots do.
        let _ = writeln!(
            items,
            r#"    <div class="item c{}"><span class="k">{}</span><div class="fact">{}</div></div>"#,
            i % MAX_NOTES + 1,
            esc(&t.tag),
            esc(&t.fact)
        );
    }
    Ok(TLDR_TEMPLATE
        .replace("{{DATE}}", &esc(date))
        .replace("{{ITEMS}}", &items))
}

fn roundup_html(spec: &Spec) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[roundup]] 但缺少 date")?;
    let mut items = String::new();
    for (i, r) in spec.roundup.iter().enumerate() {
        let (class, see) = if r.main {
            (" main", r#"<span class="see">详见前页</span>"#)
        } else {
            ("", "")
        };
        let _ = writeln!(
            items,
            r#"    <div class="item c{}{class}"><div class="n">{}</div><div class="tx"><span class="k">{}</span>{}{see}</div></div>"#,
            i % MAX_NOTES + 1,
            i + 1,
            esc(&r.tag),
            esc(&r.text)
        );
    }
    Ok(ROUNDUP_TEMPLATE
        .replace("{{DATE}}", &esc(date))
        .replace("{{ITEMS}}", &items))
}

/// One of the two template covers of a 速览专帖. It shows only the date, the
/// entry count and the tags, so it carries no claim of its own.
fn cover_html(spec: &Spec, (w, h): (u32, u32), avatar: &str) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 缺少 date（封面要写日期）")?;
    let mut tags: Vec<&str> = Vec::new();
    for r in &spec.roundup {
        if !tags.contains(&r.tag.as_str()) {
            tags.push(&r.tag);
        }
    }
    let tags: String = tags
        .iter()
        .map(|t| format!("<span>{}</span>", esc(t)))
        .collect();
    let class = if w > h { "wide" } else { "tall" };
    Ok(COVER_TEMPLATE
        .replace("{{CLASS}}", class)
        .replace("{{W}}", &w.to_string())
        .replace("{{H}}", &h.to_string())
        .replace("{{DATE}}", &esc(date))
        .replace("{{COUNT}}", &spec.roundup.len().to_string())
        .replace("{{TAGS}}", &tags)
        .replace("{{AVATAR}}", &esc(avatar)))
}

/// The repository root: the nearest ancestor of `dir` holding EDITORIAL.md.
fn repo_root(dir: &Path) -> Result<PathBuf, String> {
    let abs = std::path::absolute(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    abs.ancestors()
        .find(|p| p.join("EDITORIAL.md").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("{} 不在 AI-Barking 仓库内", dir.display()))
}

fn annot_html(a: &Annot, images: &Path, work: &Path) -> Result<String, String> {
    if a.note.is_empty() || a.note.len() > MAX_NOTES {
        return Err(format!("每张批注图需 1–{MAX_NOTES} 条 note"));
    }
    let stem = a.out.strip_suffix(".png").ok_or("out 须为 .png 文件名")?;
    let raw = images.join(format!("{stem}-raw.png"));
    let (w, h) = png_size(&raw)?;
    // OCR runs at most once per image, on first need.
    let mut ocr: Option<Vec<Word>> = None;

    // Pass 1: highlight and underline boxes per note, in raw-image pixels.
    let mut hls: Vec<Vec<Rect>> = Vec::with_capacity(a.note.len());
    // Underlines: (box, baseline).
    let mut uls: Vec<(Rect, u32)> = Vec::new();
    for (i, note) in a.note.iter().enumerate() {
        let c = i + 1;
        let (rects, quote_words) = match (&note.rects, &note.quote) {
            (Some(_), Some(_)) => {
                return Err(format!("第 {c} 条 note 的 quote 与 rects 只能二选一"));
            }
            (Some(r), None) => {
                let rects: Vec<Rect> = r.iter().map(|&[x, y, w, h]| Rect { x, y, w, h }).collect();
                if let Some(r) = rects
                    .iter()
                    .find(|r| r.w == 0 || r.h == 0 || r.x + r.w > w || r.y + r.h > h)
                {
                    return Err(format!("第 {c} 条 rects {r:?} 为空或超出底图 {w}×{h}"));
                }
                (rects, None)
            }
            (None, Some(q)) => {
                let found = locate(ocr_words(&raw, w, &a.lang, work, &mut ocr)?, q)
                    .map_err(|e| format!("第 {c} 条 quote：{e}"))?;
                (
                    found.lines.into_iter().map(|(r, _)| r).collect(),
                    Some(found.words),
                )
            }
            (None, None) => return Err(format!("第 {c} 条 note 需要 quote 或 rects")),
        };
        if rects.is_empty() {
            return Err(format!("第 {c} 条 note 的 rects 为空"));
        }
        hls.push(rects);
        if let Some(u) = &note.underline {
            let Some(quote_words) = quote_words else {
                return Err(format!(
                    "第 {c} 条用了 rects，underline 无法核对位置；改用 quote"
                ));
            };
            let found = locate(ocr_words(&raw, w, &a.lang, work, &mut ocr)?, u)
                .map_err(|e| format!("第 {c} 条 underline：{e}"))?;
            if !found.words.iter().all(|i| quote_words.contains(i)) {
                return Err(format!("第 {c} 条 underline「{u}」不在本条 quote 范围内"));
            }
            uls.extend(found.lines);
        }
    }
    split_overlaps(&mut hls);

    // Pass 2: emit. Markers go outside the screenshot's left edge, at each
    // note's first line; the page script spaces them once the scale is known.
    let mut overlays = String::new();
    for (i, rects) in hls.iter().enumerate() {
        let c = i + 1;
        for r in rects {
            let _ = writeln!(
                overlays,
                r#"    <div class="hl c{c}" style="left:{}px; top:{}px; width:{}px; height:{}px"></div>"#,
                r.x, r.y, r.w, r.h
            );
        }
        let y = rects[0].y + rects[0].h / 2;
        let _ = writeln!(
            overlays,
            r#"    <div class="mk c{c}" data-y="{y}" style="top:{y}px">{c}</div>"#
        );
    }
    // Drawn just under the baseline, not under descenders, so it stays clear
    // of the next line in tightly set paragraphs.
    for (r, baseline) in &uls {
        let _ = writeln!(
            overlays,
            r#"    <div class="ul" style="left:{}px; top:{}px; width:{}px"></div>"#,
            r.x,
            baseline + 1,
            r.w
        );
    }

    let mut notes = String::new();
    for (i, note) in a.note.iter().enumerate() {
        let c = i + 1;
        let gloss = match &note.emph {
            Some(e) if !note.gloss.contains(e.as_str()) => {
                return Err(format!("第 {c} 条 emph「{e}」不在 gloss 中"));
            }
            Some(e) => esc(&note.gloss).replacen(&esc(e), &format!("<em>{}</em>", esc(e)), 1),
            None => esc(&note.gloss),
        };
        let _ = writeln!(
            notes,
            r#"    <div class="row"><div class="n c{c}">{c}</div><div class="{}">{gloss}</div></div>"#,
            hang("tx", &note.gloss)
        );
    }
    // Every inserted value is escaped (braces included), so no value can
    // smuggle in a later placeholder.
    Ok(ANNOT_TEMPLATE
        .replace("{{TITLE}}", &esc(&a.title))
        .replace("{{IMG}}", &file_url(&raw)?)
        .replace("{{W}}", &w.to_string())
        .replace("{{H}}", &h.to_string())
        .replace("{{OVERLAYS}}", &overlays)
        .replace("{{NOTES}}", &notes)
        .replace("{{SOURCE}}", &esc(&a.source)))
}

/// Class list for a text block; a leading full-width quote gets the hanging class.
fn hang(class: &str, text: &str) -> String {
    if text.starts_with('“') {
        format!("{class} q")
    } else {
        class.to_string()
    }
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '{' => out.push_str("&#123;"),
            '}' => out.push_str("&#125;"),
            _ => out.push(c),
        }
    }
    out
}

/// Width and height from a PNG's IHDR chunk.
fn png_size(path: &Path) -> Result<(u32, u32), String> {
    let bytes = fs::read(path).map_err(|e| format!("底图 {}: {e}", path.display()))?;
    if bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return Err(format!("{} 不是 PNG", path.display()));
    }
    let be = |i: usize| u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
    Ok((be(16), be(20)))
}

fn file_url(path: &Path) -> Result<String, String> {
    let abs = std::path::absolute(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let s = abs.to_string_lossy().replace('\\', "/");
    let mut url = String::from("file:///");
    for b in s.trim_start_matches('/').bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/:".contains(&b) {
            url.push(b as char);
        } else {
            let _ = write!(url, "%{b:02X}");
        }
    }
    Ok(url)
}

fn find_chrome() -> Result<PathBuf, String> {
    if let Some(p) = std::env::var_os("BARKING_CHROME") {
        return Ok(PathBuf::from(p));
    }
    [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
    ]
    .iter()
    .map(PathBuf::from)
    .find(|p| p.is_file())
    .ok_or_else(|| "找不到 Chrome / Edge；用环境变量 BARKING_CHROME 指定可执行文件".into())
}

fn chrome_cmd(chrome: &Path, work: &Path, (w, h): (u32, u32)) -> Command {
    let mut cmd = Command::new(chrome);
    cmd.args([
        "--headless=new",
        "--disable-gpu",
        "--hide-scrollbars",
        "--force-device-scale-factor=1",
        "--allow-file-access-from-files",
    ])
    .arg(format!("--window-size={w},{h}"))
    // Own profile: never hand the job to a running Chrome.
    .arg(format!(
        "--user-data-dir={}",
        work.join("profile").display()
    ))
    .stderr(Stdio::null());
    cmd
}

fn render(
    chrome: &Path,
    work: &Path,
    html: &str,
    size: (u32, u32),
    out: &Path,
) -> Result<(), String> {
    let name = out.file_stem().and_then(|n| n.to_str()).unwrap_or("card");
    let page = work.join(format!("{name}.html"));
    fs::write(&page, html).map_err(|e| format!("{}: {e}", page.display()))?;
    let url = file_url(&page)?;

    // The page script records `data-layout` on <body>: "ok" or "overflow:…".
    let dom = chrome_cmd(chrome, work, size)
        .arg("--dump-dom")
        .arg(&url)
        .output()
        .map_err(|e| format!("无法运行 {}: {e}", chrome.display()))?;
    let dom = String::from_utf8_lossy(&dom.stdout);
    match layout_state(&dom) {
        Some("ok") => {}
        Some(bad) => {
            return Err(format!(
                "{}：版面溢出（{}）。HTML 留在 {}",
                out.display(),
                bad.trim_start_matches("overflow:"),
                page.display()
            ));
        }
        None => {
            return Err(format!(
                "{}：读不到版面检查结果（HTML 留在 {}）",
                out.display(),
                page.display()
            ));
        }
    }

    let out_abs = std::path::absolute(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let started = SystemTime::now();
    let status = chrome_cmd(chrome, work, size)
        .arg(format!("--screenshot={}", out_abs.display()))
        .arg(&url)
        .stdout(Stdio::null())
        .status()
        .map_err(|e| format!("无法运行 {}: {e}", chrome.display()))?;
    // Chrome may exit 0 without writing; require a file newer than the launch.
    let fresh = fs::metadata(&out_abs)
        .and_then(|m| m.modified())
        .is_ok_and(|t| t >= started);
    if !status.success() || !fresh {
        return Err(format!(
            "渲染 {} 失败（HTML 留在 {}）",
            out.display(),
            page.display()
        ));
    }
    println!("{}  （HTML：{}）", out.display(), page.display());
    Ok(())
}

/// The `data-layout` value on the dumped <body>, if present.
fn layout_state(dom: &str) -> Option<&str> {
    let start = dom.find("data-layout=\"")? + "data-layout=\"".len();
    let len = dom[start..].find('"')?;
    Some(&dom[start..start + len])
}

/// A Tesseract word with its box in raw-image pixels.
struct Word {
    text: String,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
    conf: f32,
    /// (block, paragraph, line) — words sharing it sit on one text line.
    line: (u32, u32, u32),
}

#[derive(Debug, PartialEq)]
struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

/// Trims vertically overlapping highlights (adjacent lines, possibly from
/// different notes) to meet halfway, so multiply blending never doubles up.
///
/// Post: no two rectangles that overlap horizontally overlap vertically,
/// provided no rectangle is nested inside another's vertical span.
fn split_overlaps(hls: &mut [Vec<Rect>]) {
    let ids: Vec<(usize, usize)> = hls
        .iter()
        .enumerate()
        .flat_map(|(n, rs)| (0..rs.len()).map(move |k| (n, k)))
        .collect();
    for &(an, ak) in &ids {
        for &(bn, bk) in &ids {
            let (a, b) = (&hls[an][ak], &hls[bn][bk]);
            let x_overlap = a.x < b.x + b.w && b.x < a.x + a.w;
            if (an, ak) == (bn, bk) || !x_overlap || a.y >= b.y || a.y + a.h <= b.y {
                continue;
            }
            // `a` is above `b` and runs into it.
            let mid = (a.y + a.h + b.y).div_ceil(2);
            let b_bottom = b.y + b.h;
            hls[an][ak].h = mid - hls[an][ak].y;
            hls[bn][bk].y = mid;
            hls[bn][bk].h = b_bottom.saturating_sub(mid);
        }
    }
}

/// Returns the cached OCR words, running Tesseract on first use. The slice
/// borrows from `cache`, which is why the cache is passed in by the caller.
fn ocr_words<'a>(
    raw: &Path,
    width: u32,
    lang: &str,
    work: &Path,
    cache: &'a mut Option<Vec<Word>>,
) -> Result<&'a [Word], String> {
    if cache.is_none() {
        *cache = Some(run_ocr(raw, width, lang, work)?);
    }
    Ok(cache.as_deref().unwrap_or_default())
}

fn run_ocr(raw: &Path, width: u32, lang: &str, work: &Path) -> Result<Vec<Word>, String> {
    // 1x screenshots read better upscaled; 2x captures are fine as they are.
    let factor: u32 = if width < 1000 { 3 } else { 1 };
    let input = if factor == 1 {
        raw.to_path_buf()
    } else {
        let tmp = work.join("ocr-input.png");
        let ok = Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-i"])
            .arg(raw)
            .args([
                "-vf",
                &format!("scale=iw*{factor}:ih*{factor}:flags=lanczos"),
            ])
            .arg(&tmp)
            .status()
            .map_err(|e| format!("无法运行 ffmpeg：{e}"))?
            .success();
        if !ok {
            return Err("ffmpeg 放大底图失败".into());
        }
        tmp
    };
    let out = Command::new("tesseract")
        .arg(&input)
        .args([
            "stdout",
            "--psm",
            "6",
            "-l",
            lang,
            "-c",
            "tessedit_create_tsv=1",
        ])
        .output()
        .map_err(|e| {
            format!("无法运行 tesseract（scoop install tesseract tesseract-languages）：{e}")
        })?;
    if !out.status.success() {
        return Err(format!(
            "tesseract 失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(parse_tsv(&String::from_utf8_lossy(&out.stdout), factor))
}

/// Word rows (level 5) of Tesseract TSV, with boxes scaled back by `factor`.
fn parse_tsv(tsv: &str, factor: u32) -> Vec<Word> {
    tsv.lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 12 || f[0] != "5" || f[11].trim().is_empty() {
                return None;
            }
            let n = |i: usize| f[i].parse::<u32>().ok();
            let (l, t, w, h) = (n(6)?, n(7)?, n(8)?, n(9)?);
            Some(Word {
                text: f[11].trim().to_string(),
                left: l / factor,
                top: t / factor,
                right: (l + w).div_ceil(factor),
                bottom: (t + h).div_ceil(factor),
                conf: f[10].parse().ok()?,
                line: (n(2)?, n(3)?, n(4)?),
            })
        })
        .collect()
}

fn is_cjk(c: char) -> bool {
    matches!(c, '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}' | '\u{f900}'..='\u{faff}')
}

/// Match tokens: lowercase alphanumeric runs, and each CJK character alone.
/// Punctuation and spacing are ignored, so `later.` matches `later`.
fn tokens(s: &str) -> Vec<String> {
    let (mut out, mut cur) = (Vec::new(), String::new());
    for c in s.chars() {
        if c.is_alphanumeric() && !is_cjk(c) {
            cur.extend(c.to_lowercase());
            continue;
        }
        if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        if is_cjk(c) {
            out.push(c.to_string());
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Where a quote sits on the image.
struct Located {
    /// One padded rectangle per text line, with that line's approximate
    /// baseline (the highest word bottom, i.e. a word without descenders).
    lines: Vec<(Rect, u32)>,
    /// Indices into the OCR words, in order.
    words: Vec<usize>,
}

/// The unique occurrence of `quote` in `words`.
///
/// Pre: `words` are in Tesseract reading order. Post: the match covers whole
/// OCR words only (a box is never wider than the quote); lines follow the
/// quote's order; every matched word has confidence ≥ `MIN_CONF`.
fn locate(words: &[Word], quote: &str) -> Result<Located, String> {
    let flat: Vec<(String, usize)> = words
        .iter()
        .enumerate()
        .flat_map(|(i, w)| tokens(&w.text).into_iter().map(move |t| (t, i)))
        .collect();
    let q = tokens(quote);
    if q.is_empty() {
        return Err("引文没有可匹配的文字".into());
    }
    let prefix = |s: usize| {
        flat[s..]
            .iter()
            .zip(&q)
            .take_while(|((t, _), qt)| t == *qt)
            .count()
    };
    let starts: Vec<usize> = (0..flat.len()).filter(|&s| prefix(s) == q.len()).collect();
    let s = match starts.as_slice() {
        [s] => *s,
        [] => {
            let (best, at) = (0..flat.len())
                .map(|s| (prefix(s), s))
                .max()
                .unwrap_or((0, 0));
            let next = flat.get(at + best).map_or("（文末）", |(t, _)| t.as_str());
            return Err(format!(
                "OCR 结果里找不到原句：最多连续匹配 {best}/{} 个词，之后 OCR 读到的是「{next}」，原句是「{}」。核对原文，或改用 rects",
                q.len(),
                q[best]
            ));
        }
        many => {
            return Err(format!(
                "原句在图中出现 {} 次，把 quote 写长一些以唯一定位",
                many.len()
            ));
        }
    };
    let e = s + q.len() - 1;
    // A word box is all-or-nothing, so the quote must start and end on word
    // boundaries (e.g. not inside "state-of-the-art").
    let (first, last) = (flat[s].1, flat[e].1);
    if (s > 0 && flat[s - 1].1 == first) || flat.get(e + 1).is_some_and(|(_, i)| *i == last) {
        let partial = if s > 0 && flat[s - 1].1 == first {
            first
        } else {
            last
        };
        return Err(format!(
            "原句的起止落在 OCR 词「{}」中间，高亮会多盖字；把 quote 写到整词，或改用 rects",
            words[partial].text
        ));
    }

    let mut idx: Vec<usize> = flat[s..=e].iter().map(|(_, i)| *i).collect();
    idx.dedup();
    if let Some(w) = idx.iter().map(|&i| &words[i]).find(|w| w.conf < MIN_CONF) {
        return Err(format!(
            "「{}」识别置信度 {:.0} 低于 {MIN_CONF}，不可靠；放大核对后改用 rects",
            w.text, w.conf
        ));
    }

    // Union of word boxes per line, in order of first appearance.
    // [left, top, right, bottom, baseline]
    let mut lines: Vec<((u32, u32, u32), [u32; 5])> = Vec::new();
    for w in idx.iter().map(|&i| &words[i]) {
        match lines.iter_mut().find(|(k, _)| *k == w.line) {
            Some((_, b)) => {
                b[0] = b[0].min(w.left);
                b[1] = b[1].min(w.top);
                b[2] = b[2].max(w.right);
                b[3] = b[3].max(w.bottom);
                b[4] = b[4].min(w.bottom);
            }
            None => lines.push((w.line, [w.left, w.top, w.right, w.bottom, w.bottom])),
        }
    }
    let lines = lines
        .into_iter()
        .map(|(_, [l, t, r, b, base])| {
            let pad = ((b - t) / 8).max(1);
            let rect = Rect {
                x: l.saturating_sub(pad),
                y: t.saturating_sub(1),
                w: r - l + 2 * pad,
                h: b - t + 2,
            };
            (rect, base)
        })
        .collect();
    Ok(Located { lines, words: idx })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(text: &str, left: u32, top: u32, line: u32, conf: u32) -> String {
        // level page block par line word left top width height conf text
        format!("5\t1\t1\t1\t{line}\t1\t{left}\t{top}\t30\t12\t{conf}\t{text}")
    }

    fn words(rows: &[(&str, u32, u32, u32)]) -> Vec<Word> {
        let mut tsv = vec!["header".to_string()];
        tsv.extend(
            rows.iter()
                .map(|&(t, l, top, line)| row(t, l, top, line, 95)),
        );
        parse_tsv(&tsv.join("\n"), 1)
    }

    #[test]
    fn example_spec_parses_and_lists_quoted_texts() {
        let spec: Spec =
            toml::from_str(include_str!("../../../templates/cards/example.toml")).unwrap();
        assert_eq!(spec.tldr.len(), 2);
        // Two 省流 facts and the two main-post 速览 titles.
        assert_eq!(spec.quoted_texts().len(), 4);
        assert!(spec.retired_barks().is_none());
        assert!(spec.structure_errors().is_empty());
        assert_eq!(
            spec.outputs(),
            [TLDR_OUT, ROUNDUP_OUT, "01-openai-dns-pause.png"]
        );
    }

    fn roundup(toml_head: &str, items: &[(&str, &str, bool)]) -> Spec {
        let mut s = format!("date = \"d\"\n{toml_head}");
        for (tag, text, main) in items {
            s.push_str(&format!(
                "[[roundup]]\ntag = \"{tag}\"\ntext = \"{text}\"\nmain = {main}\n"
            ));
        }
        toml::from_str(&s).unwrap()
    }

    #[test]
    fn roundup_structure_rules() {
        let tldr = "[[tldr]]\ntag = \"A\"\nfact = \"f\"\n[[tldr]]\ntag = \"B\"\nfact = \"g\"\n";
        let others = [("x", "t", false); 3];
        let ok = [&[("A", "a", true), ("B", "b", true)][..], &others].concat();
        assert!(roundup(tldr, &ok).structure_errors().is_empty());

        // Main entries out of the 省流卡 order, or not leading.
        let swapped = [&[("B", "b", true), ("A", "a", true)][..], &others].concat();
        assert_eq!(roundup(tldr, &swapped).structure_errors().len(), 1);
        let late = [&others[..], &[("A", "a", true), ("B", "b", true)]].concat();
        assert!(!roundup(tldr, &late).structure_errors().is_empty());

        // Too few, too long.
        assert_eq!(roundup("", &others).structure_errors().len(), 1);
        let long = "字".repeat(ROUNDUP_TEXT_MAX + 1);
        let mut five = vec![("x", "t", false); 4];
        five.push(("x", &long, false));
        assert_eq!(roundup("", &five).structure_errors().len(), 1);

        // 速览专帖: no 省流卡, no main entries; covers come from the template.
        let solo = roundup("kind = \"roundup\"\n", &[("x", "t", false); 5]);
        assert!(solo.structure_errors().is_empty());
        assert_eq!(solo.outputs(), [COVER_OUT, COVER_WIDE_OUT, ROUNDUP_OUT]);
        assert!(
            !roundup(&format!("kind = \"roundup\"\n{tldr}"), &ok)
                .structure_errors()
                .is_empty()
        );
    }

    #[test]
    fn retired_barks_are_parsed_but_flagged() {
        let spec: Spec =
            toml::from_str("date = \"d\"\n[[tldr]]\ntag = \"t\"\nfact = \"f\"\nbarks = [\"b\"]\n")
                .unwrap();
        assert_eq!(spec.quoted_texts(), ["f", "b"]);
        assert!(spec.retired_barks().unwrap().contains("barks"));
    }

    #[test]
    fn tokens_ignore_punctuation_and_split_cjk() {
        assert_eq!(tokens("later. All"), ["later", "all"]);
        assert_eq!(tokens("“停训”有 GPT-6"), ["停", "训", "有", "gpt", "6"]);
    }

    #[test]
    fn locate_gives_one_rect_per_line_and_rejects_ambiguity() {
        let w = words(&[
            ("The", 10, 10, 1),
            ("run", 50, 10, 1),
            ("was", 90, 10, 1),
            ("killed.", 10, 30, 2),
            ("The", 60, 30, 2),
        ]);
        let found = locate(&w, "run was killed").unwrap();
        assert_eq!(found.words, [1, 2, 3]);
        assert_eq!(found.lines.len(), 2);
        let r0 = Rect {
            x: 49,
            y: 9,
            w: 72,
            h: 14,
        };
        assert_eq!(found.lines[0], (r0, 22));
        assert!(locate(&w, "The").err().unwrap().contains("2 次"));
        assert!(locate(&w, "run was not").err().unwrap().contains("2/3"));
    }

    #[test]
    fn locate_rejects_partial_words_and_low_confidence() {
        let w = words(&[
            ("a", 0, 0, 1),
            ("state-of-the-art", 10, 0, 1),
            ("model", 60, 0, 1),
        ]);
        assert!(locate(&w, "a state of the").err().unwrap().contains("中间"));
        assert!(
            locate(&w, "of the art model")
                .err()
                .unwrap()
                .contains("中间")
        );
        assert!(locate(&w, "state of the art").is_ok());

        let tsv = format!(
            "h\n{}\n{}",
            row("blurry", 0, 0, 1, 95),
            row("w0rd", 40, 0, 1, 41)
        );
        assert!(
            locate(&parse_tsv(&tsv, 1), "blurry w0rd")
                .err()
                .unwrap()
                .contains("置信度")
        );
    }

    #[test]
    fn split_overlaps_meets_halfway() {
        let mut hls = vec![
            vec![Rect {
                x: 0,
                y: 0,
                w: 50,
                h: 20,
            }],
            vec![
                Rect {
                    x: 40,
                    y: 16,
                    w: 50,
                    h: 20,
                },
                Rect {
                    x: 200,
                    y: 10,
                    w: 9,
                    h: 9,
                },
            ],
        ];
        split_overlaps(&mut hls);
        assert_eq!(hls[0][0].y + hls[0][0].h, hls[1][0].y);
        assert_eq!(hls[1][0].y + hls[1][0].h, 36);
        assert_eq!(
            hls[1][1],
            Rect {
                x: 200,
                y: 10,
                w: 9,
                h: 9
            },
            "no x overlap, untouched"
        );
    }

    #[test]
    fn parse_tsv_scales_boxes_back() {
        let w = parse_tsv(&format!("h\n{}", row("x", 30, 60, 1, 95)), 3);
        assert_eq!(
            (w[0].left, w[0].top, w[0].right, w[0].bottom),
            (10, 20, 20, 24)
        );
    }

    #[test]
    fn escaping_blocks_placeholder_injection() {
        assert_eq!(
            esc("{{NOTES}} <b>"),
            "&#123;&#123;NOTES&#125;&#125; &lt;b&gt;"
        );
        assert_eq!(
            layout_state(r#"<body data-layout="overflow:x">"#),
            Some("overflow:x")
        );
        assert_eq!(layout_state("<body>"), None);
    }
}
