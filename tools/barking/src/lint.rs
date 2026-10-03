//! `barking lint`: mechanical pre-publish checks for one issue directory.
//!
//! Only rules that can be decided exactly are errors. Judgment calls (is there a
//! real 吠点? is a claim overstated?) stay with Develata and codex-reviewer.
//! The constants below mirror EDITORIAL.md / AGENTS.md; change both together.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::card::{self, Kind};

const TITLE_MAX: usize = 20;
const TOTAL_MAX: usize = 1000;
const OPENING: &str = "AI 沸点，今日谁吠？";
const SLOGAN: &str = "AI 吠点：聊 AI 沸点，轻松识破吠点。";
const TLDR_PREFIX: &str = "省流：";
const SOURCES_PREFIX: &str = "来源：";
const BANNED: [&str; 5] = [
    "值得注意的是",
    "需要指出的是",
    "随着人工智能不断发展",
    "从某种意义上来说",
    "当然我们也不能忽视",
];
/// Issues dated on or after this `YYMM_MMDD` follow the 2026-10-02 format:
/// no trailing 来源 line, and a 省流卡 `images/00-tldr.*`. Older, published
/// issues keep their original format (EDITORIAL.md 纠错: no rewrites).
const NEW_FORMAT_FROM: u32 = 2610_1002;
/// Issues from this date carry a 速览聚合图 `images/00-roundup.png`
/// (EDITORIAL.md 速览).
const ROUNDUP_FROM: u32 = 2610_1004;
/// Platforms unreachable from mainland China, never named in the publish text
/// or the card texts (风控; EDITORIAL.md 事实分级). Screenshots may show them.
/// Matched case-insensitively as whole words, so `xAI` and `SpaceX` pass;
/// X itself only as a capital `X`, so a stray `x` does not count.
const BLOCKED_PLATFORMS: [&str; 6] = [
    "twitter",
    "youtube",
    "facebook",
    "instagram",
    "telegram",
    "x.com",
];
const BLOCKED_PLATFORMS_ZH: [&str; 3] = ["推特", "油管", "脸书"];
const PLACEHOLDERS: [&str; 6] = ["TODO", "TBD", "待补", "待核", "【图", "[图"];
pub const IMAGE_EXTS: [&str; 5] = ["png", "jpg", "jpeg", "webp", "gif"];
/// Characters that mark a number as a quantity or date rather than a version.
const UNITS: [char; 14] = [
    '月', '日', '年', '美', '元', '倍', '万', '亿', '个', '条', '次', '天', '分', '秒',
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

/// Findings own their path and message: they outlive the file buffers they
/// were computed from, and are few, so cloning is cheaper than lifetimes here.
pub struct Finding {
    pub level: Level,
    pub path: PathBuf,
    pub line: Option<usize>,
    pub msg: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self.level {
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        };
        match self.line {
            Some(n) => write!(f, "{tag}: {}:{n}: {}", self.path.display(), self.msg),
            None => write!(f, "{tag}: {}: {}", self.path.display(), self.msg),
        }
    }
}

struct Sink<'a> {
    path: &'a Path,
    out: &'a mut Vec<Finding>,
}

impl Sink<'_> {
    fn push(&mut self, level: Level, line: Option<usize>, msg: impl Into<String>) {
        self.out.push(Finding {
            level,
            path: self.path.to_path_buf(),
            line,
            msg: msg.into(),
        });
    }
    fn error(&mut self, line: Option<usize>, msg: impl Into<String>) {
        self.push(Level::Error, line, msg);
    }
    fn warn(&mut self, line: Option<usize>, msg: impl Into<String>) {
        self.push(Level::Warn, line, msg);
    }
}

/// `docs/<YYMM>/<MMDD>` → `YYMMMMDD` as a number; `None` if the path is not
/// shaped like an issue directory.
fn issue_key(dir: &Path) -> Option<u32> {
    let four = |p: Option<&Path>| {
        p.and_then(Path::file_name)
            .and_then(|n| n.to_str())
            .filter(|n| n.len() == 4 && n.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|n| n.parse::<u32>().ok())
    };
    Some(four(dir.parent())? * 10_000 + four(Some(dir))?)
}

pub fn lint_issue(dir: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    // Resolve `.` and relative paths so the YYMM/MMDD components are visible;
    // unrecognised paths get the current rules.
    let resolved = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let key = issue_key(&resolved);
    let new_format = key.is_none_or(|k| k >= NEW_FORMAT_FROM);
    let roundup_era = key.is_none_or(|k| k >= ROUNDUP_FROM);

    let images = dir.join("images");
    let spec = match card::load_spec(&images) {
        Ok(s) => s,
        Err(e) => {
            Sink {
                path: &images.join(card::SPEC_FILE),
                out: &mut out,
            }
            .error(None, e);
            None
        }
    };
    let kind = spec.as_ref().map_or(Kind::Main, |s| s.kind);
    let era = Era {
        new_format,
        roundup: roundup_era,
    };

    let fc_path = dir.join("sources").join("fact-check.md");
    let fact_check = fs::read_to_string(&fc_path).ok();
    if fact_check.is_none() {
        Sink {
            path: &fc_path,
            out: &mut out,
        }
        .warn(None, "缺少事实清单");
    }

    let mut publish: Vec<PathBuf> = fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with("_publish.txt"))
                })
                .collect()
        })
        .unwrap_or_default();
    publish.sort();
    // Publish-text lines with whitespace removed, for verbatim card checks.
    let mut body_lines: Vec<String> = Vec::new();
    if publish.is_empty() {
        Sink {
            path: dir,
            out: &mut out,
        }
        .error(None, "没有 *_publish.txt 发布正文");
    }
    for p in &publish {
        let mut sink = Sink {
            path: p,
            out: &mut out,
        };
        match fs::read_to_string(p) {
            Ok(raw) => {
                body_lines.extend(
                    raw.lines()
                        .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect::<String>())
                        .filter(|l| !l.is_empty()),
                );
                lint_publish(&mut sink, &raw, fact_check.as_deref(), era, kind);
            }
            Err(e) => sink.error(None, format!("读取失败：{e}")),
        }
    }

    // Newer issues keep their images only on OpenList; `offsite.tsv` vouches for
    // the ones a fresh clone lacks.
    let offsite = offsite_paths(dir);
    lint_images(&images, era, spec.as_ref(), &offsite, &mut out);
    lint_cards(&images, &body_lines, era, spec.as_ref(), &offsite, &mut out);
    if let Some(spec) = &spec {
        let mut sink = Sink {
            path: &fc_path,
            out: &mut out,
        };
        for msg in roundup_fact_errors(spec, fact_check.as_deref()) {
            sink.error(None, msg);
        }
    }
    if new_format
        && fact_check
            .as_deref()
            .is_some_and(|fc| !fc.contains("视觉复核"))
    {
        Sink {
            path: &fc_path,
            out: &mut out,
        }
        .warn(
            None,
            "事实清单里没有“视觉复核”记录（EDITORIAL.md 省流卡与批注截图）",
        );
    }
    out
}

/// Strip a UTF-8 BOM, fold CRLF to LF, and drop trailing whitespace, so the
/// count matches what gets pasted into a platform editor.
pub fn normalize(raw: &str) -> String {
    let s = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    s.replace("\r\n", "\n").trim_end().to_string()
}

/// Conservative platform count: every Unicode scalar is one character,
/// including spaces, newlines, punctuation and each ASCII letter or digit.
/// Multi-scalar emoji therefore count more than once, which errs on the safe side.
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn lint_publish(sink: &mut Sink, raw: &str, fact_check: Option<&str>, era: Era, kind: Kind) {
    let new_format = era.new_format;
    let text = normalize(raw);
    let lines: Vec<&str> = text.lines().collect();
    let title = lines.first().map_or("", |l| l.trim());
    let (tn, total) = (char_count(title), char_count(&text));

    sink.push(
        Level::Info,
        None,
        format!(
            "标题 {tn}/{TITLE_MAX} 字；全文 {total}/{TOTAL_MAX} 字，余量 {}",
            TOTAL_MAX as isize - total as isize
        ),
    );
    if title.is_empty() {
        sink.error(Some(1), "首行应为标题");
    } else if tn > TITLE_MAX {
        sink.error(Some(1), format!("标题 {tn} 字，超过上限 {TITLE_MAX}"));
    }
    if total > TOTAL_MAX {
        sink.error(None, format!("全文 {total} 字，超过上限 {TOTAL_MAX}"));
    }

    lint_structure(sink, &lines, new_format, kind);

    let fc_compact: Option<String> =
        fact_check.map(|fc| fc.chars().filter(|c| !c.is_whitespace()).collect());
    for (i, line) in lines.iter().enumerate() {
        let n = Some(i + 1);
        for issue in markup_issues(line) {
            sink.error(n, issue);
        }
        for w in BANNED.iter().filter(|w| line.contains(*w)) {
            sink.error(n, format!("禁用套话「{w}」"));
        }
        for p in PLACEHOLDERS.iter().filter(|p| line.contains(*p)) {
            sink.error(n, format!("编辑占位符「{p}」"));
        }
        if era.roundup {
            for name in blocked_platforms(line) {
                sink.error(n, blocked_msg(&name));
            }
        }
        if i > 0 {
            for issue in spacing_issues(line) {
                sink.error(n, issue);
            }
        }
        for name in spaced_model_names(line) {
            sink.warn(n, format!("疑似型号名用空格分隔：「{name}」，应改用连字符"));
        }
        if let Some(fc) = &fc_compact {
            for num in key_numbers(line) {
                if !fc.contains(&num) {
                    sink.warn(
                        n,
                        format!("数字 {num} 未在 fact-check.md 中出现，请人工确认出处"),
                    );
                }
            }
        }
    }
}

/// Required order: 标题 → 固定开场 → 省流 → … → Slogan（最后一行）.
/// Old-format issues may end with one extra 来源 line after the Slogan. A
/// 速览专帖 has no main post, so no 省流 paragraph is required.
fn lint_structure(sink: &mut Sink, lines: &[&str], new_format: bool, kind: Kind) {
    let nonempty: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| (i, l.trim()))
        .filter(|(_, l)| !l.is_empty())
        .collect();
    let at = |i: usize| Some(i + 1);

    match nonempty.get(1) {
        Some(&(_, l)) if l == OPENING => {}
        Some(&(i, _)) => sink.error(at(i), format!("标题后第一行应为固定开场「{OPENING}」")),
        None => sink.error(None, "正文为空"),
    }

    let tldr = nonempty
        .iter()
        .position(|(_, l)| l.starts_with(TLDR_PREFIX));
    let slogan = nonempty.iter().rposition(|(_, l)| *l == SLOGAN);
    match tldr {
        None if kind == Kind::Roundup => {}
        None => sink.error(None, format!("缺少「{TLDR_PREFIX}」段")),
        Some(k) if k < 2 => sink.error(at(nonempty[k].0), "省流应在固定开场之后"),
        _ => {}
    }
    match slogan {
        None => sink.error(None, format!("缺少 Slogan 行「{SLOGAN}」")),
        Some(k) => {
            if tldr.is_some_and(|t| t > k) {
                sink.error(at(nonempty[k].0), "Slogan 应在省流之后");
            }
            match &nonempty[k + 1..] {
                [] if new_format => {}
                [] => sink.error(
                    at(nonempty[k].0),
                    format!("旧格式期次的 Slogan 后应有一行「{SOURCES_PREFIX}」"),
                ),
                [(_, l)] if !new_format && l.starts_with(SOURCES_PREFIX) => {}
                [(i, _), ..] => sink.error(
                    at(*i),
                    "Slogan 应为最后一行（正文不写来源行，来源见 sources/README.md）",
                ),
            }
        }
    }
}

fn markup_issues(line: &str) -> Vec<&'static str> {
    let t = line.trim_start();
    let mut v = Vec::new();
    if t.starts_with('#') {
        v.push("Markdown 标题标记 #");
    }
    if t.starts_with("> ") {
        v.push("Markdown 引用标记 >");
    }
    if line.contains('`') {
        v.push("Markdown 反引号");
    }
    if line.contains("**") || line.contains("__") {
        v.push("Markdown 加粗标记");
    }
    if line.contains("![") || line.contains("](") {
        v.push("Markdown 图片/链接语法");
    }
    if line.contains(":\\") || line.contains("images/") || line.contains("../") {
        v.push("本地路径");
    }
    if !image_refs_in_line(line).is_empty() {
        v.push("图片文件名（配图说明应放在 images/README.md）");
    }
    v
}

fn is_han(c: char) -> bool {
    matches!(c, '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}')
}

/// A maximal run of non-space ASCII graphic characters (`Opus-5.5`, `99.7%`,
/// `2/10`), as a half-open char-index span `[start, end)` into the line.
struct Run {
    start: usize,
    end: usize,
    has_letter: bool,
    has_digit: bool,
}

fn ascii_runs(cs: &[char]) -> Vec<Run> {
    let mut runs = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        if !cs[i].is_ascii_graphic() {
            i += 1;
            continue;
        }
        let start = i;
        while i < cs.len() && cs[i].is_ascii_graphic() {
            i += 1;
        }
        let run = &cs[start..i];
        runs.push(Run {
            start,
            end: i,
            has_letter: run.iter().any(char::is_ascii_alphabetic),
            has_digit: run.iter().any(char::is_ascii_digit),
        });
    }
    runs
}

/// Body spacing rule: an ASCII run containing a letter (English word, model
/// name) must be separated from adjacent Han by one space; a purely numeric run
/// (`99.7%`, `4/20`) must touch adjacent Han directly. Titles are exempt.
pub fn spacing_issues(line: &str) -> Vec<String> {
    let cs: Vec<char> = line.chars().collect();
    let han_at = |k: Option<usize>| k.and_then(|k| cs.get(k)).is_some_and(|&c| is_han(c));
    let ctx = |a: usize, b: usize| -> String {
        cs[a.saturating_sub(1)..(b + 1).min(cs.len())]
            .iter()
            .collect()
    };
    let mut out = Vec::new();
    for r in ascii_runs(&cs) {
        let before = r.start.checked_sub(1);
        if r.has_letter {
            if han_at(before) || han_at(Some(r.end)) {
                out.push(format!(
                    "英文与中文之间应留空格：「{}」",
                    ctx(r.start, r.end)
                ));
            }
        } else if r.has_digit {
            let gap_before = before.is_some_and(|b| cs[b] == ' ') && han_at(r.start.checked_sub(2));
            let gap_after = cs.get(r.end) == Some(&' ') && han_at(Some(r.end + 1));
            if gap_before || gap_after {
                let (a, b) = (r.start.saturating_sub(1), (r.end + 1).min(cs.len()));
                out.push(format!("数字与中文之间不留空格：「{}」", ctx(a, b)));
            }
        }
    }
    out
}

/// Model names joined by a space instead of a hyphen: a letter run followed by
/// a run starting with a digit (`Opus 5.5`), or a letter+digit run followed by
/// a capitalized word (`GPT-6 Sol`). Heuristic, so reported as a warning.
pub fn spaced_model_names(line: &str) -> Vec<String> {
    let cs: Vec<char> = line.chars().collect();
    let runs = ascii_runs(&cs);
    runs.windows(2)
        .filter(|w| w[1].start == w[0].end + 1 && cs[w[0].end] == ' ')
        .filter(|w| {
            let next = cs[w[1].start];
            // `Opus 66.4%`, `Astra 10/50美元`, `OpenAI 8月` are figures, not versions.
            let figure =
                cs[w[1].end - 1] == '%' || cs.get(w[1].end).is_some_and(|c| UNITS.contains(c));
            w[0].has_letter
                && ((next.is_ascii_digit() && !figure)
                    || (w[0].has_digit && next.is_ascii_uppercase()))
        })
        .map(|w| cs[w[0].start..w[1].end].iter().collect())
        .collect()
}

/// Decimals (`99.7`, `5.1`) and percentages (`60%`). Plain integers are skipped:
/// dates and counts are written in too many formats to match reliably.
pub fn key_numbers(line: &str) -> Vec<String> {
    let cs: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        if !cs[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
            i += 1;
        }
        let tok: String = cs[start..i].iter().collect();
        let tok = tok.trim_end_matches('.').to_string();
        let pct = matches!(cs.get(i), Some('%' | '％'));
        if (tok.contains('.') || pct) && !out.contains(&tok) {
            out.push(tok);
        }
    }
    out
}

/// Relative image paths mentioned on one line, e.g. `01-a.png` or
/// `../sources/usage/x.png`. URLs (containing `//`) are skipped.
pub fn image_refs_in_line(line: &str) -> Vec<&str> {
    let allowed = |b: u8| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'/');
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    for ext in IMAGE_EXTS {
        let pat = format!(".{ext}");
        let mut from = 0;
        while let Some(off) = line[from..].find(&pat) {
            let dot = from + off;
            let end = dot + pat.len();
            from = end;
            if bytes.get(end).is_some_and(|b| b.is_ascii_alphanumeric()) {
                continue;
            }
            let mut start = dot;
            while start > 0 && allowed(bytes[start - 1]) {
                start -= 1;
            }
            let tok = &line[start..end];
            if start < dot && !tok.contains("//") && !out.contains(&tok) {
                out.push(tok);
            }
        }
    }
    out
}

/// Paths (relative to the issue directory) that `sources/offsite.tsv` records as
/// stored on OpenList. Missing local copies of these are not lint errors.
fn offsite_paths(dir: &Path) -> BTreeSet<String> {
    let text = fs::read_to_string(dir.join("sources").join("offsite.tsv")).unwrap_or_default();
    parse_offsite(&text)
}

/// `tok` as written in a file under `dir` (e.g. `../sources/a.png` in
/// `images/README.md`), folded into an issue-relative `/` path like `offsite.tsv` uses.
fn issue_rel(dir: &str, tok: &str) -> String {
    let mut parts: Vec<&str> = vec![dir];
    for seg in tok.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(seg),
        }
    }
    parts.join("/")
}

fn parse_offsite(text: &str) -> BTreeSet<String> {
    text.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| l.split('\t').next())
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect()
}

/// Which dated rules an issue falls under.
#[derive(Clone, Copy)]
struct Era {
    /// 2026-10-02 format: 省流卡, no 来源 line.
    new_format: bool,
    /// 2026-10-04: 速览聚合图.
    roundup: bool,
}

fn lint_images(
    images: &Path,
    era: Era,
    spec: Option<&card::Spec>,
    offsite: &BTreeSet<String>,
    out: &mut Vec<Finding>,
) {
    let kind = spec.map_or(Kind::Main, |s| s.kind);
    let readme = images.join("README.md");
    let mut sink = Sink { path: &readme, out };
    let md = match fs::read_to_string(&readme) {
        Ok(s) => s,
        Err(e) => return sink.error(None, format!("缺少配图说明：{e}")),
    };

    let mut referenced = BTreeSet::new();
    for (i, line) in md.lines().enumerate() {
        for tok in image_refs_in_line(line) {
            if let Some(name) = Path::new(tok).file_name().and_then(|n| n.to_str()) {
                referenced.insert(name.to_string());
            }
            if !images.join(tok).is_file() && !offsite.contains(&issue_rel("images", tok)) {
                sink.error(Some(i + 1), format!("配图说明列出的 {tok} 不存在"));
            }
        }
    }

    // Right after the cover come the 省流卡, then the 速览聚合图; a 速览专帖
    // has only the latter.
    let mut lead: Vec<(&str, &str)> = Vec::new();
    if era.new_format && kind == Kind::Main {
        lead.push(("00-tldr.", "省流卡 00-tldr.png"));
    }
    if era.roundup {
        lead.push(("00-roundup.", "速览聚合图 00-roundup.png"));
    }
    let formal: Vec<&str> = md
        .lines()
        .skip_while(|l| !l.starts_with("## 正式配图"))
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
        .filter_map(|l| image_refs_in_line(l).first().copied())
        .collect();
    for (k, (stem, what)) in lead.iter().enumerate() {
        if formal.get(k).is_some_and(|f| !f.starts_with(stem)) {
            sink.warn(None, format!("“正式配图”第 {} 张应为{what}", k + 1));
        }
    }

    let files: Vec<String> = fs::read_dir(images)
        .map(|rd| {
            rd.filter_map(Result::ok)
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    for (stem, what, required) in [
        ("00-cover.", "3:4 封面", true),
        ("00-cover-wide.", "2.35:1 横版封面", true),
        ("00-tldr.", "省流卡", era.new_format && kind == Kind::Main),
        ("00-roundup.", "速览聚合图", era.roundup),
    ] {
        let remote = issue_rel("images", stem);
        if required
            && !files.iter().any(|f| f.starts_with(stem))
            && !offsite.iter().any(|p| p.starts_with(&remote))
        {
            sink.error(None, format!("缺少{what} images/{stem}*"));
        }
    }
    let mut unlisted: Vec<&String> = files
        .iter()
        .filter(|f| {
            f.rsplit_once('.')
                .is_some_and(|(_, e)| IMAGE_EXTS.contains(&e.to_ascii_lowercase().as_str()))
                && !referenced.contains(*f)
        })
        .collect();
    unlisted.sort();
    for f in unlisted {
        sink.warn(None, format!("images/{f} 未在配图说明中出现"));
    }
}

/// Card texts must be verbatim (ignoring whitespace) excerpts of the
/// verified publish text; EDITORIAL.md 省流卡与批注截图.
fn lint_cards(
    images: &Path,
    body_lines: &[String],
    era: Era,
    spec: Option<&card::Spec>,
    offsite: &BTreeSet<String>,
    out: &mut Vec<Finding>,
) {
    let path = images.join(card::SPEC_FILE);
    let mut sink = Sink { path: &path, out };
    let Some(spec) = spec else {
        if era.roundup {
            sink.error(None, "缺少 images/cards.toml：速览聚合图由它渲染");
        } else if era.new_format {
            sink.warn(None, "缺少 images/cards.toml：省流卡文字无法与正文自动比对");
        }
        return;
    };
    for msg in spec.structure_errors() {
        sink.error(None, msg);
    }
    if era.roundup && spec.roundup.is_empty() {
        sink.error(None, "缺少 [[roundup]]：2026-10-04 起每期都有速览聚合图");
    }
    for msg in card_text_errors(&spec.quoted_texts(), body_lines) {
        sink.error(None, msg);
    }
    if era.roundup {
        for t in spec.own_texts() {
            for name in blocked_platforms(t) {
                sink.error(None, format!("卡片文字「{t}」：{}", blocked_msg(&name)));
            }
        }
    }
    // Rendered images older than the spec were not re-rendered after an edit.
    let mtime = |p: &Path| fs::metadata(p).and_then(|m| m.modified()).ok();
    if let Some(spec_time) = mtime(&path) {
        for o in spec.outputs() {
            match mtime(&images.join(o)) {
                None if offsite.contains(&issue_rel("images", o)) => {}
                None => sink.error(None, format!("images/{o} 尚未渲染（barking card）")),
                Some(t) if t < spec_time => sink.warn(
                    None,
                    format!("images/{o} 早于 cards.toml，可能需要重新渲染"),
                ),
                Some(_) => {}
            }
        }
    }
}

/// Each card text, split at “；”, must be a verbatim excerpt of one publish
/// line (whitespace ignored; a trailing full stop allowed). Whether an
/// excerpt changes the meaning is left to human and visual review.
fn card_text_errors(texts: &[&str], body_lines: &[String]) -> Vec<String> {
    let mut errs = Vec::new();
    for t in texts {
        for seg in t.split('；') {
            let c = compact(seg);
            if !c.is_empty() && !body_lines.iter().any(|l| l.contains(&c)) {
                errs.push(format!(
                    "卡片文字未在正文同一行中逐字出现：「{seg}」（出自「{t}」）"
                ));
            }
        }
    }
    errs
}

/// Blocked platform names on one line, as written.
pub fn blocked_platforms(line: &str) -> Vec<String> {
    let mut out: Vec<String> = BLOCKED_PLATFORMS_ZH
        .iter()
        .filter(|w| line.contains(*w))
        .map(|w| w.to_string())
        .collect();
    let cs: Vec<char> = line.chars().collect();
    for r in ascii_runs(&cs) {
        let run: String = cs[r.start..r.end].iter().collect();
        // Split `X/YouTube` and drop edge punctuation such as `(X)`, keeping
        // the dot of a domain.
        for word in run.split(['/', ',', ';', '(', ')', '"', '\'']) {
            let w = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
            let lower = w.to_ascii_lowercase();
            let bare = lower.strip_prefix("www.").unwrap_or(&lower);
            let hit = w == "X"
                || BLOCKED_PLATFORMS.contains(&bare)
                || BLOCKED_PLATFORMS
                    .iter()
                    .any(|p| bare.strip_prefix(p).is_some_and(|r| r.starts_with('.')))
                || bare.starts_with("x.com/")
                || bare.starts_with("t.me");
            if hit && !out.iter().any(|o| o == w) {
                out.push(w.to_string());
            }
        }
    }
    out
}

fn blocked_msg(name: &str) -> String {
    format!(
        "「{name}」是中国大陆无法访问的平台，正文与卡片文字不写（截图可以）：官方账号发言写“某公司称”，媒体报道写“据外媒报道”"
    )
}

/// Whitespace removed and trailing sentence punctuation dropped, for verbatim
/// comparison of card texts.
fn compact(s: &str) -> String {
    s.trim()
        .trim_end_matches(['。', '，', '！', '？'])
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// One row of the 速览 table in fact-check.md.
struct FactRow<'a> {
    fact: &'a str,
    level: Option<u8>,
    result: &'a str,
}

/// Table rows under the first heading that names 速览, up to the next heading.
/// Columns follow EDITORIAL.md: 文中事实 | 级 | 结果 | 一手来源 | 备注.
fn roundup_rows(fact_check: &str) -> Option<Vec<FactRow<'_>>> {
    let mut lines = fact_check
        .lines()
        .skip_while(|l| !(l.starts_with('#') && l.contains("速览")));
    lines.next()?;
    let rows = lines
        .take_while(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let cells: Vec<&str> = l
                .trim()
                .strip_prefix('|')?
                .trim_end_matches('|')
                .split('|')
                .map(str::trim)
                .collect();
            let (fact, level, result) = (*cells.first()?, *cells.get(1)?, *cells.get(2)?);
            if fact == "文中事实" || fact.starts_with("---") {
                return None;
            }
            // `L4`, `L4 权威媒体`, … → 4.
            let level = level
                .split_once('L')
                .and_then(|(_, r)| r.chars().next())
                .and_then(|c| c.to_digit(10))
                .and_then(|d| u8::try_from(d).ok());
            Some(FactRow {
                fact,
                level,
                result,
            })
        })
        .collect();
    Some(rows)
}

/// Each non-main 速览 entry is not in the publish text, so it must be listed
/// verbatim in fact-check.md's 速览 table, at a level its wording matches:
/// L1–L3 verified ✅; L4 only with an unnamed-outlet attribution; never L5–L7,
/// never ❌, never 网传 (EDITORIAL.md 速览).
fn roundup_fact_errors(spec: &card::Spec, fact_check: Option<&str>) -> Vec<String> {
    let entries: Vec<&card::RoundupItem> = spec.other_entries().collect();
    if entries.is_empty() {
        return Vec::new();
    }
    let Some(rows) = fact_check.and_then(roundup_rows) else {
        return vec!["fact-check.md 缺少“速览”一节：速览条目须逐条列入".into()];
    };
    let mut errs = Vec::new();
    for e in entries {
        let t = &e.text;
        if t.contains("网传") {
            errs.push(format!("速览条目「{t}」：速览不收社交转述与传闻（网传）"));
        }
        let Some(row) = rows.iter().find(|r| compact(r.fact) == compact(t)) else {
            errs.push(format!(
                "速览条目「{t}」未在 fact-check.md“速览”一节的“文中事实”列中逐字出现"
            ));
            continue;
        };
        // “据彭博社报道”“据外媒报道”: some attribution, outlet named or not.
        let has_l4_wording = t.contains('据') && t.contains("报道");
        match row.level {
            _ if row.result.contains('❌') => {
                errs.push(format!("速览条目「{t}」在事实清单中标为 ❌"));
            }
            None => errs.push(format!("速览条目「{t}」在事实清单中缺少级别（L1–L4）")),
            Some(1..=3) if !row.result.contains('✅') => {
                errs.push(format!("速览条目「{t}」为 L1–L3，但事实清单结果不是 ✅"));
            }
            Some(4) if !has_l4_wording => errs.push(format!(
                "速览条目「{t}」出自 L4 媒体报道，须写「据某媒体报道」（来源在大陆无法访问的平台上时写「据外媒报道」）"
            )),
            Some(1..=4) => {}
            Some(l) => errs.push(format!("速览条目「{t}」为 L{l}，速览只收 L1–L4")),
        }
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_rel_folds_parent_segments() {
        assert_eq!(issue_rel("images", "00-tldr.png"), "images/00-tldr.png");
        assert_eq!(
            issue_rel("images", "../sources/usage/a.png"),
            "sources/usage/a.png"
        );
        assert_eq!(issue_rel("images", "./a.png"), "images/a.png");
    }

    #[test]
    fn offsite_manifest_lists_paths_and_skips_comments() {
        let tsv = "# 注释\nimages/00-tldr.png\t100\tabc\nsources/a.pdf\t5\tdef\n";
        let set = parse_offsite(tsv);
        assert!(set.contains("images/00-tldr.png") && set.contains("sources/a.pdf"));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn count_ignores_bom_crlf_and_trailing_newline() {
        let lf = "标题\n\nAI 沸点\n";
        let crlf = "\u{feff}标题\r\n\r\nAI 沸点\r\n";
        assert_eq!(normalize(lf), normalize(crlf));
        assert_eq!(char_count(&normalize(crlf)), 9);
    }

    /// Whether `lint_structure` reports any error.
    fn structure_errs_as(text: &str, new_format: bool, kind: Kind) -> bool {
        let mut out = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        lint_structure(
            &mut Sink {
                path: Path::new("t"),
                out: &mut out,
            },
            &lines,
            new_format,
            kind,
        );
        out.iter().any(|f| f.level == Level::Error)
    }

    fn structure_errs(text: &str, new_format: bool) -> bool {
        structure_errs_as(text, new_format, Kind::Main)
    }

    #[test]
    fn roundup_post_needs_no_tldr_paragraph() {
        let bare = format!("AI 速览｜10月4日\n\n{OPENING}\n\n{SLOGAN}\n");
        assert!(structure_errs(&bare, true));
        assert!(!structure_errs_as(&bare, true, Kind::Roundup));
    }

    fn roundup_spec(others: &[&str]) -> card::Spec {
        let mut toml = String::from("kind = \"roundup\"\ndate = \"d\"\n");
        for t in others {
            toml.push_str(&format!("[[roundup]]\ntag = \"x\"\ntext = \"{t}\"\n"));
        }
        toml::from_str(&toml).unwrap()
    }

    #[test]
    fn roundup_entries_need_a_matching_fact_check_row() {
        let fc = "\
# 事实清单

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 正文里的事 | L1 | ✅ | u | 甲乙 丙 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 甲 发布了乙。 | L1 | ✅ | u | |
| 据外媒报道，丙推迟 | L4 | ⚠️ | u | 彭博 |
| 据彭博社报道，丁 | L4 | ⚠️ | u | |
| 丁推迟 | L4 | ⚠️ | u | 未署名 |
| 戊 | L2 | ⚠️ | u | |
| 己 | L6 | ⚠️ | u | |
| 庚 | L1 | ❌ | u | |

## 其他
| 辛 | L1 | ✅ | u | |
";
        let ok = roundup_spec(&["甲发布了乙", "据外媒报道，丙推迟", "据彭博社报道，丁"]);
        assert!(roundup_fact_errors(&ok, Some(fc)).is_empty());
        // L4 without attribution, unverified L2, L6, ❌, a row outside the 速览
        // section, a sentence found only in another section's notes column.
        for bad in ["丁推迟", "戊", "己", "庚", "辛", "甲乙丙"] {
            let errs = roundup_fact_errors(&roundup_spec(&[bad]), Some(fc));
            assert_eq!(errs.len(), 1, "{bad}: {errs:?}");
        }
        assert_eq!(
            roundup_fact_errors(&roundup_spec(&["网传甲发布了乙"]), Some(fc)).len(),
            2
        );
        assert!(roundup_fact_errors(&ok, Some("# 事实清单\n"))[0].contains("缺少"));
        assert!(roundup_fact_errors(&roundup_spec(&[]), None).is_empty());
    }

    #[test]
    fn blocked_platforms_are_whole_words() {
        assert_eq!(
            blocked_platforms("据 X 上的帖子，(YouTube) 与推特；x.com/a"),
            ["推特", "X", "YouTube", "x.com"]
        );
        assert_eq!(
            blocked_platforms("www.facebook.com 与 t.me/x"),
            ["www.facebook.com", "t.me"]
        );
        assert!(blocked_platforms("xAI 的 Grok、SpaceX、GPT-6.1-Astra、Meta、Xbox").is_empty());
        assert!(blocked_platforms(SLOGAN).is_empty());
    }

    #[test]
    fn slogan_ends_the_text_and_sources_line_is_old_format_only() {
        let ok = format!("标题\n\n{OPENING}\n\n省流：x\n\n{SLOGAN}\n");
        let legacy = format!("{ok}\n来源：OpenAI\n");
        let extra = format!("{ok}\n多一行\n");
        let two = format!("{legacy}\n多一行\n");
        for new_format in [true, false] {
            assert!(structure_errs(&extra, new_format));
            assert!(structure_errs(&two, new_format));
        }
        assert!(!structure_errs(&ok, true));
        assert!(structure_errs(&legacy, true));
        assert!(!structure_errs(&legacy, false));
        assert!(structure_errs(&ok, false), "old format keeps its 来源 line");
    }

    #[test]
    fn card_texts_must_be_verbatim_within_one_line() {
        let lines: Vec<String> = ["省流：甲发布了乙。", "吠点：①丙不等于丁，戊未确认。"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        let ok = ["甲发布了乙。", "丙不等于丁；戊未确认。"];
        assert!(card_text_errors(&ok, &lines).is_empty());
        // Spans two lines, or alters a word: rejected.
        let bad = ["乙。吠点", "丙等于丁"];
        assert_eq!(card_text_errors(&bad, &lines).len(), 2);
    }

    #[test]
    fn issue_key_resolves_dot() {
        let here = std::env::current_dir().unwrap();
        let resolved = Path::new(".").canonicalize().unwrap();
        assert_eq!(resolved.file_name(), here.file_name());
    }

    #[test]
    fn issue_key_reads_yymm_mmdd() {
        assert_eq!(issue_key(Path::new("docs/2610/1002")), Some(2610_1002));
        assert_eq!(issue_key(Path::new("E:/x/docs/2609/0926")), Some(2609_0926));
        assert_eq!(issue_key(Path::new("docs/0926")), None);
        assert!(issue_key(Path::new("docs/2609/0930")).unwrap() < NEW_FORMAT_FROM);
    }

    #[test]
    fn title_count_matches_recorded_0924() {
        // Commit 0961d87 records this title as 18 characters.
        assert_eq!(char_count("Opus 5.5 完爆 Astra？"), 18);
    }

    #[test]
    fn spacing_wants_space_around_english_but_not_numbers() {
        assert!(spacing_issues(SLOGAN).is_empty());
        assert!(spacing_issues(OPENING).is_empty());
        assert!(spacing_issues("Anthropic 恢复收费，Opus-5.5 更便宜").is_empty());
        assert!(spacing_issues("测试中99.7%的账户，2/10美元").is_empty());
        assert!(spacing_issues("“Medicare 被黑”").is_empty());
        assert_eq!(spacing_issues("Anthropic恢复").len(), 1);
        assert_eq!(spacing_issues("由Guardian 报道").len(), 1);
        assert_eq!(spacing_issues("降 20% 的").len(), 1);
    }

    #[test]
    fn model_names_with_spaces_are_flagged() {
        assert_eq!(
            spaced_model_names("Opus 5.5 完爆 GPT-6 Astra？"),
            ["Opus 5.5", "GPT-6 Astra"]
        );
        assert!(spaced_model_names("Opus-5.5 完爆 GPT-6-Astra").is_empty());
        assert!(spaced_model_names(SLOGAN).is_empty());
        assert!(spaced_model_names("Claude Code 和 Hacker News").is_empty());
        assert!(spaced_model_names("Opus 66.4%对Astra 10/50美元，OpenAI 8月").is_empty());
        assert_eq!(spaced_model_names("Opus 5.5完爆Astra？"), ["Opus 5.5"]);
    }

    #[test]
    fn key_numbers_takes_decimals_and_percentages_only() {
        assert_eq!(
            key_numbers("测试中99.7%的账户，9月25日，0.1%以下。"),
            ["99.7", "0.1"]
        );
        assert_eq!(key_numbers("降约六成，60％，版本5.1。"), ["60", "5.1"]);
        assert!(key_numbers("4/20美元，58对53").is_empty());
    }

    #[test]
    fn image_refs_resolve_relative_and_skip_urls_and_globs() {
        let line = "| [01-a.png](01-a.png) | ../sources/x.jpeg | `00-cover.*` | https://e.com/y.png | a.pngx |";
        assert_eq!(image_refs_in_line(line), ["01-a.png", "../sources/x.jpeg"]);
    }
}
