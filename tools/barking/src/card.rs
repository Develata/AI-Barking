//! `barking card`: render one issue's 省流卡 and annotated screenshots from
//! `images/cards.toml`, filling the HTML templates in `templates/cards/` and
//! screenshotting them with headless Chrome.
//!
//! Highlights come from Tesseract word boxes found by matching the quoted
//! sentence. A missing, ambiguous or low-confidence match is an error, never a
//! guess: a misplaced highlight on evidence puts words in the source's mouth.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::SystemTime;

use serde::Deserialize;

const TLDR_TEMPLATE: &str = include_str!("../../../templates/cards/tldr.html");
const ANNOT_TEMPLATE: &str = include_str!("../../../templates/cards/annot.html");
pub const SPEC_FILE: &str = "cards.toml";
const TLDR_OUT: &str = "00-tldr.png";
/// Highlight colours defined in annot.html (`.c1` … `.c4`).
const MAX_NOTES: usize = 4;
/// Below this Tesseract confidence a matched word is not trusted.
const MIN_CONF: f32 = 60.0;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub date: Option<String>,
    #[serde(default)]
    pub tldr: Vec<TldrItem>,
    #[serde(default)]
    pub annot: Vec<Annot>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TldrItem {
    pub tag: String,
    pub fact: String,
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
    /// Card texts that must appear verbatim in the publish text.
    pub fn quoted_texts(&self) -> Vec<&str> {
        self.tldr
            .iter()
            .flat_map(|t| {
                std::iter::once(t.fact.as_str()).chain(t.barks.iter().map(String::as_str))
            })
            .chain(self.annot.iter().filter_map(|a| a.bark.as_deref()))
            .collect()
    }

    pub fn tldr_bark_count(&self) -> usize {
        self.tldr.iter().map(|t| t.barks.len()).sum()
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
    let chrome = find_chrome()?;
    let work = std::env::temp_dir().join("barking-card");
    fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let wanted = |name: &str| only.is_empty() || only.iter().any(|o| o == name);

    let mut n = 0;
    if !spec.tldr.is_empty() && wanted(TLDR_OUT) {
        render(&chrome, &work, &tldr_html(&spec)?, &images.join(TLDR_OUT))?;
        n += 1;
    }
    for a in spec.annot.iter().filter(|a| wanted(&a.out)) {
        let html = annot_html(a, &images, &work).map_err(|e| format!("{}：{e}", a.out))?;
        render(&chrome, &work, &html, &images.join(&a.out))?;
        n += 1;
    }
    if n == 0 {
        return Err("没有要渲染的卡片（检查 cards.toml 或文件名参数）".into());
    }
    Ok(n)
}

fn tldr_html(spec: &Spec) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[tldr]] 但缺少 date")?;
    let mut items = String::new();
    for t in &spec.tldr {
        let _ = write!(
            items,
            r#"    <div class="item"><span class="k">{}</span><div class="fact">{}</div>"#,
            esc(&t.tag),
            esc(&t.fact)
        );
        for b in &t.barks {
            let _ = write!(
                items,
                r#"<div class="bark"><div class="l">吠点</div><div class="{}">{}</div></div>"#,
                hang("t", b),
                esc(b)
            );
        }
        items.push_str("</div>\n");
    }
    Ok(TLDR_TEMPLATE
        .replace("{{DATE}}", &esc(date))
        .replace("{{ITEMS}}", &items))
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
        let rects: Vec<Rect> = match (&note.rects, &note.quote) {
            (Some(r), _) => r.iter().map(|&[x, y, w, h]| Rect { x, y, w, h }).collect(),
            (None, Some(q)) => locate(ocr_words(&raw, w, &a.lang, work, &mut ocr)?, q)
                .map_err(|e| format!("第 {c} 条 quote：{e}"))?
                .into_iter()
                .map(|(r, _)| r)
                .collect(),
            (None, None) => return Err(format!("第 {c} 条 note 需要 quote 或 rects")),
        };
        if rects.is_empty() {
            return Err(format!("第 {c} 条 note 的 rects 为空"));
        }
        hls.push(rects);
        if let Some(u) = &note.underline {
            uls.extend(
                locate(ocr_words(&raw, w, &a.lang, work, &mut ocr)?, u)
                    .map_err(|e| format!("第 {c} 条 underline：{e}"))?,
            );
        }
    }
    split_overlaps(&mut hls);

    // Pass 2: markers sit in the left margin, beside each note's first line,
    // pushed down so they never stack. The on-card scale is only known in the
    // page script, so spacing uses the same fit-to-width estimate.
    let margin = hls.iter().flatten().map(|r| r.x).min().unwrap_or(0).saturating_sub(3);
    let min_gap = (44.0 / (940.0 / w as f64).min(2.0)).ceil() as u32;
    let mut overlays = String::new();
    let mut last_mark: Option<u32> = None;
    for (i, rects) in hls.iter().enumerate() {
        let c = i + 1;
        for r in rects {
            let _ = writeln!(
                overlays,
                r#"    <div class="hl c{c}" style="left:{}px; top:{}px; width:{}px; height:{}px"></div>"#,
                r.x, r.y, r.w, r.h
            );
        }
        let mut y = rects[0].y + rects[0].h / 2;
        if let Some(prev) = last_mark {
            y = y.max(prev + min_gap);
        }
        last_mark = Some(y);
        let _ = writeln!(
            overlays,
            r#"    <div class="mk c{c}" style="left:{margin}px; top:{y}px">{c}</div>"#
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
            r#"    <div class="row"><div class="n c{c}">{c}</div><div><div class="lab">译注</div><div class="{}">{gloss}</div></div></div>"#,
            hang("tx", &note.gloss)
        );
    }
    let bark = a.bark.as_deref().map_or(String::new(), |b| {
        format!(
            r#"    <div class="bark"><div class="lab">吠点</div><div class="{}">{}</div></div>"#,
            hang("tx", b),
            esc(b)
        )
    });
    Ok(ANNOT_TEMPLATE
        .replace("{{TITLE}}", &esc(&a.title))
        .replace("{{IMG}}", &file_url(&raw)?)
        .replace("{{W}}", &w.to_string())
        .replace("{{H}}", &h.to_string())
        .replace("{{OVERLAYS}}", &overlays)
        .replace("{{NOTES}}", &notes)
        .replace("{{BARK}}", &bark)
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

fn render(chrome: &Path, work: &Path, html: &str, out: &Path) -> Result<(), String> {
    let name = out.file_stem().and_then(|n| n.to_str()).unwrap_or("card");
    let page = work.join(format!("{name}.html"));
    fs::write(&page, html).map_err(|e| format!("{}: {e}", page.display()))?;
    let out_abs = std::path::absolute(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let started = SystemTime::now();
    let status = Command::new(chrome)
        .args([
            "--headless=new",
            "--disable-gpu",
            "--hide-scrollbars",
            "--force-device-scale-factor=1",
            "--allow-file-access-from-files",
            "--window-size=1080,1440",
        ])
        .arg(format!("--screenshot={}", out_abs.display()))
        .arg(file_url(&page)?)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
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

/// One padded rectangle per text line covered by the unique occurrence of
/// `quote` in `words`, with that line's approximate baseline (the highest word
/// bottom, i.e. a word without descenders).
///
/// Pre: `words` are in Tesseract reading order. Post: rectangles follow the
/// quote's line order; every matched word has confidence ≥ `MIN_CONF`.
fn locate(words: &[Word], quote: &str) -> Result<Vec<(Rect, u32)>, String> {
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

    let mut idx: Vec<usize> = flat[s..s + q.len()].iter().map(|(_, i)| *i).collect();
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
    Ok(lines
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
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(text: &str, left: u32, top: u32, line: u32) -> String {
        // level page block par line word left top width height conf text
        format!("5\t1\t1\t1\t{line}\t1\t{left}\t{top}\t30\t12\t95\t{text}")
    }

    #[test]
    fn example_spec_parses_and_lists_quoted_texts() {
        let spec: Spec =
            toml::from_str(include_str!("../../../templates/cards/example.toml")).unwrap();
        assert_eq!(spec.tldr.len(), 2);
        assert_eq!(spec.tldr_bark_count(), 3);
        assert_eq!(spec.quoted_texts().len(), 2 + 3 + 1);
    }

    #[test]
    fn tokens_ignore_punctuation_and_split_cjk() {
        assert_eq!(tokens("later. All"), ["later", "all"]);
        assert_eq!(tokens("“停训”有 GPT-6"), ["停", "训", "有", "gpt", "6"]);
    }

    #[test]
    fn locate_gives_one_rect_per_line_and_rejects_ambiguity() {
        let tsv = [
            "header".to_string(),
            word("The", 10, 10, 1),
            word("run", 50, 10, 1),
            word("was", 90, 10, 1),
            word("killed.", 10, 30, 2),
            word("The", 60, 30, 2),
        ]
        .join("\n");
        let words = parse_tsv(&tsv, 1);
        let rects = locate(&words, "run was killed").unwrap();
        assert_eq!(rects.len(), 2);
        let r0 = Rect { x: 49, y: 9, w: 72, h: 14 };
        assert_eq!(rects[0], (r0, 22));
        assert!(locate(&words, "The").unwrap_err().contains("2 次"));
        assert!(locate(&words, "run was not").unwrap_err().contains("2/3"));
    }

    #[test]
    fn parse_tsv_scales_boxes_back() {
        let words = parse_tsv(&format!("h\n{}", word("x", 30, 60, 1)), 3);
        assert_eq!(
            (words[0].left, words[0].top, words[0].right, words[0].bottom),
            (10, 20, 20, 24)
        );
    }
}
