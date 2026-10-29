//! `barking lint`: mechanical pre-publish checks for one issue directory.
//!
//! Only rules that can be decided exactly are errors. Judgment calls (is there a
//! real 吠点? is a claim overstated?) stay with Develata and codex-reviewer.
//! The constants below mirror EDITORIAL.md / AGENTS.md; change both together.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::card::{self, Kind, issue_key};
use crate::cover;

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
/// Issues from this date follow EDITORIAL.md 标题 / 文风 (2026-10-26) and
/// record a `封面抽签：` line (`barking cover-draw`).
const STYLE_FROM: u32 = 2610_1025;
/// Issues from this date carry one 吠点卡 `images/00-bark-N.png` per main
/// post (EDITORIAL.md 省流卡与批注截图, 2026-10-28).
const BARK_FROM: u32 = 2610_1027;
/// Issues from this date also carry one 事件卡 `images/00-event-N.png` per
/// main post, ahead of its 吠点卡 (EDITORIAL.md 省流卡与批注截图, 2026-10-29).
const EVENT_FROM: u32 = 2610_1028;
/// Names with one canonical spelling. Another casing in a title is only a
/// warning: Develata may choose one on purpose (1021 used `claude`).
const BRANDS: [&str; 13] = [
    "Anthropic",
    "ChatGPT",
    "Claude",
    "Codex",
    "Copilot",
    "DeepSeek",
    "Gemini",
    "Google",
    "Grok",
    "Kimi",
    "Meta",
    "OpenAI",
    "Qwen",
];
/// Wire-copy words that make a title read like a press summary; the question
/// mark already carries the doubt.
const TITLE_WIRE: [&str; 3] = ["被曝", "被指", "疑似"];
/// Admonishing pivots and maxim closers (“A 不等于 B”“仍需外部检验”). Summed
/// over 0924–1024: 1022 has 10, 1007 and 1021 have 5, 1014 has 3, every other
/// issue at most 2.
const HEDGES: [&str; 13] = [
    "不等于",
    "不能当成",
    "不能直接",
    "照搬",
    "要分清",
    "得分清",
    "有边界",
    "有范围",
    "仍需",
    "就变了",
    "还得看",
    "有待观察",
    "仍待",
];
const HEDGE_WARN_AT: usize = 3;
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
        style: key.is_none_or(|k| k >= STYLE_FROM),
        bark: key.is_none_or(|k| k >= BARK_FROM),
        event: key.is_none_or(|k| k >= EVENT_FROM),
        publish: card::publish_era(key),
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
    if era.style && kind == Kind::Main {
        lint_cover_ledger(dir, key.unwrap_or(u32::MAX), &images, &mut out);
    }
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
    if era.style {
        lint_style(sink, title, &text);
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
    /// 1025 on: 标题 / 文风 checks and the 封面抽签 line.
    style: bool,
    /// 1027 on: a 吠点卡 per main post.
    bark: bool,
    /// 1028 on: an 事件卡 per main post.
    event: bool,
    /// 1029 on: the upload set lives in `images/publish/`, numbered in order.
    publish: bool,
}

/// The mechanical parts of EDITORIAL.md 标题 and 文风. All warnings: each
/// single use can be legitimate, so they ask for a look, not a rewrite.
fn lint_style(sink: &mut Sink, title: &str, text: &str) {
    for (w, b) in brand_case_issues(title) {
        sink.warn(Some(1), format!("标题里的「{w}」通常写作「{b}」"));
    }
    for w in TITLE_WIRE.iter().filter(|w| title.contains(*w)) {
        sink.warn(
            Some(1),
            format!("标题含「{w}」，像新闻摘要：钩子写具体事实，存疑交给问号（EDITORIAL.md 标题）"),
        );
    }
    let hits = hedge_hits(text);
    let n: usize = hits.iter().map(|(_, k)| k).sum();
    if n >= HEDGE_WARN_AT {
        let list: Vec<String> = hits.iter().map(|(w, k)| format!("{w}×{k}")).collect();
        sink.warn(
            None,
            format!(
                "告诫式套话 {n} 处（{}）：逐句改成具体事实或删掉（EDITORIAL.md 文风）",
                list.join("、")
            ),
        );
    }
}

/// ASCII words that match a brand case-insensitively but not exactly.
fn brand_case_issues(s: &str) -> Vec<(&str, &'static str)> {
    s.split(|c: char| !c.is_ascii_alphanumeric())
        .filter_map(|w| {
            BRANDS
                .iter()
                .find(|b| b.eq_ignore_ascii_case(w) && **b != w)
                .map(|b| (w, *b))
        })
        .collect()
}

fn hedge_hits(text: &str) -> Vec<(&'static str, usize)> {
    HEDGES
        .iter()
        .map(|w| (*w, text.matches(w).count()))
        .filter(|(_, k)| *k > 0)
        .collect()
}

/// EDITORIAL.md 封面“每期抽签”: one `封面抽签：` line, values from the deck,
/// none used too recently.
fn lint_cover_ledger(dir: &Path, key: u32, images: &Path, out: &mut Vec<Finding>) {
    let readme = images.join("README.md");
    // A missing README is already an error from `lint_images`.
    let Ok(md) = fs::read_to_string(&readme) else {
        return;
    };
    let mut sink = Sink { path: &readme, out };
    let root = match card::repo_root(dir) {
        Ok(r) => r,
        Err(e) => return sink.error(None, e),
    };
    for (line, msg) in cover::ledger_errors(&root, key, &md) {
        sink.error(line, msg);
    }
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
            // Only `images/` itself is checked for unlisted files below.
            let in_publish = era.publish && tok.starts_with("publish/");
            if let Some(name) = Path::new(tok).file_name().and_then(|n| n.to_str())
                && !in_publish
            {
                referenced.insert(name.to_string());
            }
            if !images.join(tok).is_file() && !offsite.contains(&issue_rel("images", tok)) {
                sink.error(Some(i + 1), format!("配图说明列出的 {tok} 不存在"));
            }
        }
    }

    // Each row of 正式配图 with the first image it names.
    let rows: Vec<(&str, &str)> = md
        .lines()
        .skip_while(|l| !l.starts_with("## 正式配图"))
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
        .filter_map(|l| image_refs_in_line(l).first().map(|f| (*f, l)))
        .collect();
    if era.publish {
        lint_publish_dir(images, spec, &rows, offsite, &mut sink);
    } else {
        lint_flat_rows(&rows, era, spec, kind, &mut sink);
    }

    let files: Vec<String> = fs::read_dir(images)
        .map(|rd| {
            rd.filter_map(Result::ok)
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    if !era.publish {
        lint_flat_covers(&files, era, kind, offsite, &mut sink);
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

/// 正式配图 before 1029: flat `images/00-…` names, the 省流卡 and 速览聚合图
/// right after the cover, 事件卡 and 吠点卡 in main-post order.
fn lint_flat_rows(
    rows: &[(&str, &str)],
    era: Era,
    spec: Option<&card::Spec>,
    kind: Kind,
    sink: &mut Sink,
) {
    // Right after the cover come the 省流卡, then the 速览聚合图; a 速览专帖
    // has only the latter.
    let mut lead: Vec<(&str, &str)> = Vec::new();
    if era.new_format && kind == Kind::Main {
        lead.push(("00-tldr.", "省流卡 00-tldr.png"));
    }
    if era.roundup {
        lead.push(("00-roundup.", "速览聚合图 00-roundup.png"));
    }
    for (k, (stem, what)) in lead.iter().enumerate() {
        if rows.get(k).is_some_and(|(f, _)| !f.starts_with(stem)) {
            sink.warn(None, format!("“正式配图”第 {} 张应为{what}", k + 1));
        }
    }
    // 事件卡 and 吠点卡: in 正式配图, in main-post order, each row saying
    // 公众号 skips it; a post's 事件卡 comes before its 吠点卡.
    let mut last = None;
    for (i, out) in spec
        .into_iter()
        .flat_map(card::Spec::event_outputs)
        .enumerate()
    {
        let Some(k) = rows.iter().position(|(f, _)| *f == out) else {
            sink.warn(None, format!("事件卡 {out} 应列在“正式配图”里"));
            continue;
        };
        if last.is_some_and(|l| k < l) {
            sink.warn(None, format!("事件卡 {out} 应按主帖顺序排列"));
        }
        if !rows[k].1.contains("公众号") {
            sink.warn(
                None,
                format!("事件卡 {out} 一行须注明公众号不插（读者在读正文）"),
            );
        }
        let bark = spec
            .into_iter()
            .flat_map(card::Spec::bark_outputs)
            .nth(i)
            .and_then(|b| rows.iter().position(|(f, _)| *f == b));
        if bark.is_some_and(|b| b < k) {
            sink.warn(None, format!("事件卡 {out} 应排在同一主帖的吠点卡之前"));
        }
        last = Some(k);
    }
    let mut last = None;
    for out in spec.into_iter().flat_map(card::Spec::bark_outputs) {
        let Some(k) = rows.iter().position(|(f, _)| *f == out) else {
            sink.warn(None, format!("吠点卡 {out} 应列在“正式配图”里"));
            continue;
        };
        if last.is_some_and(|l| k < l) {
            sink.warn(None, format!("吠点卡 {out} 应按主帖顺序排列"));
        }
        if !rows[k].1.contains("公众号") {
            sink.warn(
                None,
                format!("吠点卡 {out} 一行须注明公众号不插（读者在读正文）"),
            );
        }
        last = Some(k);
    }
}

/// The required covers and 省流卡 and 速览聚合图 files, before 1029.
fn lint_flat_covers(
    files: &[String],
    era: Era,
    kind: Kind,
    offsite: &BTreeSet<String>,
    sink: &mut Sink,
) {
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
}

/// `NN` of a `NN-name.png` upload-set file, if it has that shape.
fn upload_number(name: &str) -> Option<u32> {
    let (num, _) = name.split_once('-')?;
    (num.len() == 2 && num.bytes().all(|b| b.is_ascii_digit()))
        .then(|| num.parse().ok())
        .flatten()
}

/// 1029 on: `images/publish/` holds exactly the images to upload, the two
/// covers (`00-cover.*`, `00-cover-wide.*`) and then `01-…`, `02-…` in upload
/// order without gaps; 正式配图 lists each of those rows in that order, as
/// `publish/NN-….png`. Files that offsite.tsv vouches for count as present.
fn lint_publish_dir(
    images: &Path,
    spec: Option<&card::Spec>,
    rows: &[(&str, &str)],
    offsite: &BTreeSet<String>,
    sink: &mut Sink,
) {
    let dir = images.join(card::PUBLISH_DIR);
    let remote = issue_rel("images", card::PUBLISH_DIR) + "/";
    let mut present: BTreeSet<String> = fs::read_dir(&dir)
        .map(|rd| {
            (rd.filter_map(Result::ok))
                .filter(|e| e.path().is_file())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    present.extend(
        (offsite.iter())
            .filter_map(|p| p.strip_prefix(&remote))
            .filter(|n| !n.contains('/'))
            .map(str::to_string),
    );
    let is_cover = |n: &str| n.to_lowercase().starts_with("00-cover");
    for (stem, what) in [
        ("00-cover.", "3:4 封面"),
        ("00-cover-wide.", "2.35:1 横版封面"),
    ] {
        if !present.iter().any(|f| f.to_lowercase().starts_with(stem)) {
            sink.error(
                None,
                format!("缺少{what} images/publish/{stem}*（生图选定后复制进来）"),
            );
        }
    }
    // Byte order of the names is the upload order; the two covers lead.
    let upload: Vec<&String> = present.iter().filter(|n| !is_cover(n)).collect();

    // Rows of 正式配图 must name publish/ files.
    let mut listed: Vec<&str> = Vec::new();
    for (file, line) in rows {
        let Some(name) = file.strip_prefix("publish/").filter(|n| !n.contains('/')) else {
            sink.error(
                None,
                format!("“正式配图”里的 {file} 须写成 publish/NN-….png（1029 期起正式配图都在 images/publish/）"),
            );
            continue;
        };
        if is_cover(name) {
            continue;
        }
        if listed.contains(&name) {
            sink.error(None, format!("“正式配图”里 publish/{name} 列了不止一次"));
        }
        listed.push(name);
        // 事件卡 and 吠点卡 duplicate the body for a reader of the images.
        let card = name
            .split_once('-')
            .map(|(_, t)| t)
            .is_some_and(|t| t.starts_with("event-") || t.starts_with("bark-"));
        if card && !line.contains("公众号") {
            sink.warn(
                None,
                format!("publish/{name} 一行须注明公众号不插（读者在读正文）"),
            );
        }
    }

    // The folder is the upload set: nothing in it that the table omits, no
    // earlier numbering of this tool's own cards.
    let leftovers =
        spec.map(|s| s.leftovers(&upload.iter().map(|n| n.to_string()).collect::<Vec<_>>()));
    for name in &upload {
        if listed.contains(&name.as_str()) {
            continue;
        }
        let old = leftovers
            .as_ref()
            .is_some_and(|(stale, _)| stale.contains(name));
        let why = if old {
            "是旧编号的残留（重跑 barking card 会清掉）"
        } else {
            "不在配图说明“正式配图”里（publish/ 只放要上传的图）"
        };
        sink.error(None, format!("images/publish/{name} {why}"));
    }
    // Listed order = upload order = name order.
    let in_folder: Vec<&str> = listed
        .iter()
        .copied()
        .filter(|n| upload.iter().any(|u| u.as_str() == *n))
        .collect();
    let mut sorted = in_folder.clone();
    sorted.sort_unstable();
    if in_folder != sorted {
        sink.error(
            None,
            "“正式配图”的顺序与 images/publish/ 里按文件名排序不一致：上传时全选整个文件夹按名称排序，两者必须相同",
        );
    }
    // 01, 02, … without gaps.
    let numbers: Vec<Option<u32>> = upload.iter().map(|n| upload_number(n)).collect();
    if let Some(i) = numbers.iter().position(Option::is_none) {
        sink.error(
            None,
            format!(
                "images/publish/{} 的文件名须以两位序号开头（NN-名字.png）",
                upload[i]
            ),
        );
    } else {
        for (k, n) in numbers.iter().flatten().enumerate() {
            if *n != k as u32 + 1 {
                sink.error(
                    None,
                    format!(
                        "images/publish/ 的序号不连续：第 {} 张应为 {:02}，实为 {}",
                        k + 1,
                        k + 1,
                        upload[k]
                    ),
                );
                break;
            }
        }
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
    for msg in spec.structure_warnings() {
        sink.warn(None, msg);
    }
    if era.roundup && spec.roundup.is_empty() {
        sink.error(None, "缺少 [[roundup]]：2026-10-04 起每期都有速览聚合图");
    }
    if era.bark && spec.kind == Kind::Main && spec.bark.is_empty() {
        sink.error(None, "缺少 [[bark]]：1027 期起每个主帖一张吠点卡");
    }
    if era.event && spec.kind == Kind::Main && spec.event.is_empty() {
        sink.error(None, "缺少 [[event]]：1028 期起每个主帖一张事件卡");
    }
    for msg in card_text_errors(&spec.quoted_texts(), body_lines) {
        sink.error(None, msg);
    }
    for msg in event_card_errors(&spec.event, body_lines) {
        sink.error(None, msg);
    }
    for msg in bark_card_errors(&spec.bark, body_lines) {
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
            match mtime(&images.join(&o)) {
                None if offsite.contains(&issue_rel("images", &o)) => {}
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

/// Whether a (whitespace-free) publish line heads a main-post section:
/// 一、二、… (a Chinese numeral and 、).
fn is_section_heading(line: &str) -> bool {
    let mut cs = line.chars();
    cs.next()
        .is_some_and(|c| "一二三四五六七八九十".contains(c))
        && cs.find(|c| !"一二三四五六七八九十".contains(*c)) == Some('、')
}

/// The publish text's main-post fact sections in order: the non-empty lines
/// after each heading line (一、 二、 …) up to its `吠点：` line (or the next
/// heading, or the closing `AI吠点：` line).
fn event_sections(body_lines: &[String]) -> Vec<Vec<&str>> {
    let mut sections: Vec<Vec<&str>> = Vec::new();
    let mut open = false;
    for l in body_lines {
        if is_section_heading(l) {
            sections.push(Vec::new());
            open = true;
        } else if l.starts_with("吠点：") || l.starts_with("AI吠点：") {
            open = false;
        } else if open && let Some(s) = sections.last_mut() {
            s.push(l);
        }
    }
    sections
}

/// An 事件卡 carries its main post's whole fact section, verbatim and in
/// order (whitespace and trailing full stops ignored), one paragraph per
/// line. The n-th [[event]] is the n-th section.
fn event_card_errors(events: &[card::EventCard], body_lines: &[String]) -> Vec<String> {
    let mut errs = Vec::new();
    if events.is_empty() {
        return errs;
    }
    let sections = event_sections(body_lines);
    if sections.len() != events.len() {
        errs.push(format!(
            "正文有 {} 个主帖小节，[[event]] 有 {} 张事件卡，须按主帖顺序一一对应",
            sections.len(),
            events.len()
        ));
        return errs;
    }
    for (e, section) in events.iter().zip(&sections) {
        if e.paras.len() != section.len() {
            errs.push(format!(
                "事件卡「{}」有 {} 段，正文对应小节标题与“吠点：”之间有 {} 段：须逐字、完整、同序列出",
                e.tag,
                e.paras.len(),
                section.len()
            ));
            continue;
        }
        for (k, (p, line)) in e.paras.iter().zip(section).enumerate() {
            if compact(p) != compact(line) {
                errs.push(format!(
                    "事件卡「{}」第 {} 段与正文不一致：卡片「{p}」／正文「{line}」",
                    e.tag,
                    k + 1
                ));
            }
        }
    }
    errs
}

/// The publish text's 吠点 sections in order: the lines after each `吠点：`
/// line that start with a circled number, numbers removed. Text after the
/// colon on the `吠点：` line itself counts as the first point.
fn bark_sections(body_lines: &[String]) -> Vec<Vec<&str>> {
    let mut sections = Vec::new();
    let mut lines = body_lines.iter().peekable();
    while let Some(l) = lines.next() {
        let Some(rest) = l.strip_prefix("吠点：") else {
            continue;
        };
        let mut points: Vec<&str> = Vec::new();
        if !rest.is_empty() {
            points.push(card::strip_circled(rest).unwrap_or(rest));
        }
        while let Some(p) = lines.peek().and_then(|n| card::strip_circled(n)) {
            points.push(p);
            lines.next();
        }
        sections.push(points);
    }
    sections
}

/// A 吠点卡 carries its main post's whole 吠点 section, verbatim and in
/// order (whitespace and trailing full stops ignored), so a reader who only
/// flips through the images misses none of it. The n-th [[bark]] is the
/// n-th section.
fn bark_card_errors(barks: &[card::BarkCard], body_lines: &[String]) -> Vec<String> {
    let mut errs = Vec::new();
    if barks.is_empty() {
        return errs;
    }
    let sections = bark_sections(body_lines);
    if sections.len() != barks.len() {
        errs.push(format!(
            "正文有 {} 段吠点，[[bark]] 有 {} 张吠点卡，须按主帖顺序一一对应",
            sections.len(),
            barks.len()
        ));
        return errs;
    }
    for (b, section) in barks.iter().zip(&sections) {
        if b.points.len() != section.len() {
            errs.push(format!(
                "吠点卡「{}」有 {} 条，正文对应吠点段有 {} 条：须逐字、完整、同序列出",
                b.tag,
                b.points.len(),
                section.len()
            ));
            continue;
        }
        for (k, (p, line)) in b.points.iter().zip(section).enumerate() {
            if compact(p) != compact(line) {
                errs.push(format!(
                    "吠点卡「{}」第 {} 条与正文不一致：卡片「{p}」／正文「{line}」",
                    b.tag,
                    k + 1
                ));
            }
        }
    }
    errs
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
    fn brand_spelling_is_checked_on_whole_words() {
        assert_eq!(
            brand_case_issues("claude也出BOT；安全问题扎堆"),
            [("claude", "Claude")]
        );
        assert!(brand_case_issues("OpenAI撤稿？骂Claude封号？").is_empty());
        assert!(brand_case_issues("metadata 与 Kimis").is_empty());
        assert_eq!(
            brand_case_issues("OPENAI 与 deepseek"),
            [("OPENAI", "OpenAI"), ("deepseek", "DeepSeek")]
        );
    }

    #[test]
    fn hedges_are_counted_per_occurrence() {
        let hits = hedge_hits("撤回不等于证伪，通过也不等于验证；结论不能直接照搬。");
        assert_eq!(hits, [("不等于", 2), ("不能直接", 1), ("照搬", 1)]);
        assert!(hedge_hits("他用 Grok Bot 建了个智能体。").is_empty());
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
    fn bark_cards_carry_whole_sections() {
        // As lint reads the body: whitespace removed, blank lines dropped.
        let lines: Vec<String> = [
            "一、甲",
            "吠点：",
            "①丙不等于丁。",
            "②戊未确认。",
            "二、乙",
            "吠点：①己。",
            "AI吠点：聊AI沸点，轻松识破吠点。",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        assert_eq!(
            bark_sections(&lines),
            [vec!["丙不等于丁。", "戊未确认。"], vec!["己。"]]
        );
        let card = |tag: &str, points: &[&str]| card::BarkCard {
            tag: tag.into(),
            points: points.iter().map(|p| p.to_string()).collect(),
        };
        // Whitespace and a dropped full stop are fine.
        let ok = [
            card("A", &["丙 不等于 丁", "戊未确认。"]),
            card("B", &["己"]),
        ];
        assert!(bark_card_errors(&ok, &lines).is_empty());
        // A missing card, a missing point, an altered point.
        assert!(bark_card_errors(&ok[..1], &lines)[0].contains("2 段吠点"));
        let short = [card("A", &["丙不等于丁"]), card("B", &["己"])];
        assert!(bark_card_errors(&short, &lines)[0].contains("1 条"));
        let edited = [card("A", &["丙等于丁", "戊未确认"]), card("B", &["己"])];
        let errs = bark_card_errors(&edited, &lines);
        assert_eq!(errs.len(), 1, "{errs:?}");
        assert!(errs[0].contains("第 1 条"));
    }

    #[test]
    fn event_cards_carry_whole_sections() {
        // As lint reads the body: whitespace removed, blank lines dropped.
        let lines: Vec<String> = [
            "省流：甲乙。",
            "一、甲",
            "甲发布了乙。",
            "据报道丙。",
            "吠点：",
            "①丙不等于丁。",
            "二、乙",
            "乙做了戊。",
            "吠点：①己。",
            "三、丙",
            "AI吠点：聊AI沸点，轻松识破吠点。",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        assert!(is_section_heading("一、甲") && is_section_heading("十二、乙"));
        assert!(!is_section_heading("省流：甲") && !is_section_heading("一二三"));
        assert!(!is_section_heading("、甲"));
        assert_eq!(
            event_sections(&lines),
            [
                vec!["甲发布了乙。", "据报道丙。"],
                vec!["乙做了戊。"],
                vec![]
            ]
        );
        let card = |tag: &str, paras: &[&str]| card::EventCard {
            tag: tag.into(),
            paras: paras.iter().map(|p| p.to_string()).collect(),
        };
        let lines = &lines[..9];
        // Whitespace and a dropped full stop are fine.
        let ok = [
            card("A", &["甲发布了 乙", "据报道丙。"]),
            card("B", &["乙做了戊"]),
        ];
        assert!(event_card_errors(&ok, lines).is_empty());
        // A missing card, a missing paragraph, an altered paragraph.
        assert!(event_card_errors(&ok[..1], lines)[0].contains("2 个主帖小节"));
        let short = [card("A", &["甲发布了乙"]), card("B", &["乙做了戊"])];
        assert!(event_card_errors(&short, lines)[0].contains("1 段"));
        let edited = [
            card("A", &["甲发布了乙", "据报道丙，已证实"]),
            card("B", &["乙做了戊"]),
        ];
        let errs = event_card_errors(&edited, lines);
        assert_eq!(errs.len(), 1, "{errs:?}");
        assert!(errs[0].contains("第 2 段"));
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

    // ---- images/publish/ (1029 on) ----------------------------------------

    const PUBLISH_ERA: Era = Era {
        new_format: true,
        roundup: true,
        style: true,
        bark: true,
        event: true,
        publish: true,
    };

    /// An `images/` directory with these files in `publish/` (and `top` in
    /// `images/` itself) and a README whose 正式配图 holds `rows`. Returns
    /// the `images/` path; its parent is the scratch root.
    fn publish_images(name: &str, publish: &[&str], top: &[&str], rows: &[&str]) -> PathBuf {
        let root = std::env::temp_dir()
            .join("barking-lint-test")
            .join(format!("{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let images = root.join("images");
        fs::create_dir_all(images.join("publish")).unwrap();
        for f in publish {
            fs::write(images.join("publish").join(f), b"x").unwrap();
        }
        for f in top {
            fs::write(images.join(f), b"x").unwrap();
        }
        let mut md =
            String::from("# 配图说明\n\n## 正式配图（封面之后）\n\n| 文件 | 说明 |\n|---|---|\n");
        for r in rows {
            md += &format!("| {r} |\n");
        }
        md += "\n## 备用图\n\n| 文件 | 用途 |\n|---|---|\n";
        fs::write(images.join("README.md"), md).unwrap();
        images
    }

    fn publish_findings(images: &Path, spec: Option<&card::Spec>, offsite: &[&str]) -> String {
        let offsite: BTreeSet<String> = offsite.iter().map(|s| s.to_string()).collect();
        let mut out = Vec::new();
        lint_images(images, PUBLISH_ERA, spec, &offsite, &mut out);
        let text = (out.iter())
            .map(|f| {
                let tag = if f.level == Level::Error {
                    "error"
                } else {
                    "warn"
                };
                format!("{tag}: {}", f.msg)
            })
            .collect::<Vec<_>>()
            .join("\n");
        // Each case has its own scratch root; nothing else reads it.
        let _ = fs::remove_dir_all(images.parent().unwrap());
        text
    }

    const GOOD: [&str; 7] = [
        "00-cover.png",
        "00-cover-wide.png",
        "01-tldr.png",
        "02-roundup.png",
        "03-event-1.png",
        "04-bark-1.png",
        "05-carbon.png",
    ];
    const GOOD_ROWS: [&str; 5] = [
        "publish/01-tldr.png | 省流",
        "publish/02-roundup.png | 速览",
        "publish/03-event-1.png | 事件卡；公众号不插",
        "publish/04-bark-1.png | 吠点卡；公众号不插",
        "publish/05-carbon.png | 批注图",
    ];

    #[test]
    fn publish_folder_matching_the_table_is_clean() {
        let d = publish_images("ok", &GOOD, &[], &GOOD_ROWS);
        assert_eq!(publish_findings(&d, None, &[]), "");
    }

    #[test]
    fn publish_folder_order_gaps_and_strays_are_errors() {
        // Table order differs from name order.
        let mut rows = GOOD_ROWS.to_vec();
        rows.swap(0, 1);
        let d = publish_images("order", &GOOD, &[], &rows);
        assert!(publish_findings(&d, None, &[]).contains("顺序"));
        // A file the table omits.
        let more = [&GOOD[..], &["06-extra.png"]].concat();
        let d = publish_images("stray", &more, &[], &GOOD_ROWS);
        let f = publish_findings(&d, None, &[]);
        assert!(f.contains("06-extra.png 不在"), "{f}");
        // A gap: 03 is missing, so 04 is the third file.
        let gap: Vec<&str> = (GOOD.iter().copied())
            .filter(|n| !n.starts_with("03"))
            .collect();
        let rows: Vec<&str> = (GOOD_ROWS.iter().copied())
            .filter(|r| !r.contains("03-"))
            .collect();
        let d = publish_images("gap", &gap, &[], &rows);
        let f = publish_findings(&d, None, &[]);
        assert!(f.contains("序号不连续") && f.contains("应为 03"), "{f}");
        // No number at all.
        let nonum = [&GOOD[..], &["zebra.png"]].concat();
        let rows = [&GOOD_ROWS[..], &["publish/zebra.png | z"]].concat();
        let d = publish_images("nonum", &nonum, &[], &rows);
        assert!(publish_findings(&d, None, &[]).contains("两位序号"));
        // Covers missing.
        let d = publish_images("nocover", &GOOD[2..], &[], &GOOD_ROWS);
        let f = publish_findings(&d, None, &[]);
        assert!(
            f.contains("缺少3:4 封面") && f.contains("缺少2.35:1 横版封面"),
            "{f}"
        );
        // The wide cover alone is not the 3:4 one.
        let d = publish_images("widecover", &GOOD[1..], &[], &GOOD_ROWS);
        let f = publish_findings(&d, None, &[]);
        assert!(
            f.contains("缺少3:4 封面") && !f.contains("缺少2.35:1"),
            "{f}"
        );
    }

    #[test]
    fn publish_rows_must_name_publish_files_and_cards_skip_the_account() {
        // A row that points at images/ instead of images/publish/.
        let mut rows = GOOD_ROWS.to_vec();
        rows[0] = "01-tldr.png | 省流";
        let d = publish_images("flatrow", &GOOD, &["01-tldr.png"], &rows);
        let f = publish_findings(&d, None, &[]);
        assert!(f.contains("须写成 publish/NN-"), "{f}");
        assert!(f.contains("publish/01-tldr.png 不在"), "{f}");
        // 事件卡 and 吠点卡 rows say 公众号不插, or it is a warning.
        let mut rows = GOOD_ROWS.to_vec();
        rows[2] = "publish/03-event-1.png | 事件卡";
        let d = publish_images("nogzh", &GOOD, &[], &rows);
        let f = publish_findings(&d, None, &[]);
        assert_eq!(
            f,
            "warn: publish/03-event-1.png 一行须注明公众号不插（读者在读正文）"
        );
        // Listed twice.
        let mut rows = GOOD_ROWS.to_vec();
        rows.push("publish/05-carbon.png | again");
        let d = publish_images("twice", &GOOD, &[], &rows);
        assert!(publish_findings(&d, None, &[]).contains("不止一次"));
    }

    #[test]
    fn publish_folder_offsite_names_count_and_old_numbers_are_named() {
        // Nothing local (a fresh clone): offsite.tsv vouches for the files.
        let remote: Vec<String> = (GOOD.iter())
            .map(|n| format!("images/publish/{n}"))
            .collect();
        let mut remote: Vec<&str> = remote.iter().map(String::as_str).collect();
        let d = publish_images("remote", &[], &[], &GOOD_ROWS);
        assert_eq!(publish_findings(&d, None, &remote), "");
        // ... but not for a file the table omits.
        remote.push("images/publish/06-extra.png");
        let d = publish_images("remote2", &[], &[], &GOOD_ROWS);
        assert!(publish_findings(&d, None, &remote).contains("06-extra.png 不在"));

        // After a renumber, the old file is called what it is.
        let mut spec: card::Spec = toml::from_str(
            "date = \"d\"\n[[tldr]]\ntag = \"A\"\nfact = \"f\"\n\
             [[annot]]\nout = \"carbon\"\npost = 1\ntitle = \"t\"\nsource = \"s\"\n\
             [[annot.note]]\nquote = \"q\"\ngloss = \"g\"\n",
        )
        .unwrap();
        spec.publish = true;
        // Current: 01-tldr, 02-carbon; 03-carbon is the earlier numbering.
        let files = [
            "00-cover.png",
            "00-cover-wide.png",
            "01-tldr.png",
            "02-carbon.png",
            "03-carbon.png",
        ];
        let rows = [
            "publish/01-tldr.png | 省流",
            "publish/02-carbon.png | 批注图",
        ];
        let d = publish_images("stale", &files, &[], &rows);
        let f = publish_findings(&d, Some(&spec), &[]);
        assert!(f.contains("03-carbon.png 是旧编号的残留"), "{f}");
    }

    #[test]
    fn publish_era_checks_only_the_top_of_images_for_unlisted_files() {
        // A forgotten file at the top of images/ is still reported, even when
        // the table lists a publish/ file of the same name.
        let d = publish_images(
            "top",
            &GOOD,
            &["07-forgotten.png", "01-tldr.png"],
            &GOOD_ROWS,
        );
        let f = publish_findings(&d, None, &[]);
        assert!(
            f.contains("images/07-forgotten.png 未在配图说明中出现"),
            "{f}"
        );
        assert!(f.contains("images/01-tldr.png 未在配图说明中出现"), "{f}");
    }

    #[test]
    fn old_era_still_wants_flat_names() {
        // 1028 and before: rows without publish/, covers in images/.
        let root = std::env::temp_dir()
            .join("barking-lint-test")
            .join(format!("{}-flat", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let images = root.join("images");
        fs::create_dir_all(&images).unwrap();
        for f in [
            "00-cover.png",
            "00-cover-wide.png",
            "00-tldr.png",
            "00-roundup.png",
        ] {
            fs::write(images.join(f), b"x").unwrap();
        }
        fs::write(
            images.join("README.md"),
            "## 正式配图\n\n| 1 | 00-tldr.png | 省流 |\n| 2 | 00-roundup.png | 速览 |\n\n## 备用图\n\n",
        )
        .unwrap();
        let mut era = PUBLISH_ERA;
        era.publish = false;
        let mut out = Vec::new();
        lint_images(&images, era, None, &BTreeSet::new(), &mut out);
        let msgs: Vec<&str> = out.iter().map(|f| f.msg.as_str()).collect();
        // The covers are not in the table, so they are the only unlisted files.
        assert_eq!(
            msgs,
            [
                "images/00-cover-wide.png 未在配图说明中出现",
                "images/00-cover.png 未在配图说明中出现"
            ]
        );
        let _ = fs::remove_dir_all(&root);
    }
}
