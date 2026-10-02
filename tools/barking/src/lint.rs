//! `barking lint`: mechanical pre-publish checks for one issue directory.
//!
//! Only rules that can be decided exactly are errors. Judgment calls (is there a
//! real 吠点? is a claim overstated?) stay with Develata and codex-reviewer.
//! The constants below mirror EDITORIAL.md / AGENTS.md; change both together.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

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
            Ok(raw) => lint_publish(&mut sink, &raw, fact_check.as_deref()),
            Err(e) => sink.error(None, format!("读取失败：{e}")),
        }
    }

    lint_images(&dir.join("images"), &mut out);
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

fn lint_publish(sink: &mut Sink, raw: &str, fact_check: Option<&str>) {
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

    lint_structure(sink, &lines);

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
fn lint_structure(sink: &mut Sink, lines: &[&str]) {
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
                [] => {}
                // Issues published before 2026-10-02 end with one 来源 line.
                [(i, l)] if l.starts_with(SOURCES_PREFIX) => sink.warn(
                    at(*i),
                    "2026-10-02 起正文不写文末来源行（来源见 sources/README.md）；已发布的旧期次可忽略",
                ),
                [(i, _), ..] => sink.error(at(*i), "Slogan 应为最后一行"),
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

fn lint_images(images: &Path, out: &mut Vec<Finding>) {
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
            if !images.join(tok).is_file() {
                sink.error(Some(i + 1), format!("配图说明列出的 {tok} 不存在"));
            }
        }
    }

    let files: Vec<String> = fs::read_dir(images)
        .map(|rd| {
            rd.filter_map(Result::ok)
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    for (stem, what) in [
        ("00-cover.", "3:4 封面"),
        ("00-cover-wide.", "2.35:1 横版封面"),
    ] {
        if !files.iter().any(|f| f.starts_with(stem)) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ignores_bom_crlf_and_trailing_newline() {
        let lf = "标题\n\nAI 沸点\n";
        let crlf = "\u{feff}标题\r\n\r\nAI 沸点\r\n";
        assert_eq!(normalize(lf), normalize(crlf));
        assert_eq!(char_count(&normalize(crlf)), 9);
    }

    fn structure(text: &str) -> Vec<(bool, String)> {
        let mut out = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        lint_structure(
            &mut Sink {
                path: Path::new("t"),
                out: &mut out,
            },
            &lines,
        );
        out.into_iter()
            .map(|f| (f.level == Level::Error, f.msg))
            .collect()
    }

    #[test]
    fn slogan_ends_the_text_and_legacy_sources_line_only_warns() {
        let ok = format!("标题\n\n{OPENING}\n\n省流：x\n\n{SLOGAN}\n");
        assert!(structure(&ok).is_empty());

        let legacy = format!("{ok}\n来源：OpenAI\n");
        let f = structure(&legacy);
        assert_eq!(f.len(), 1);
        assert!(!f[0].0, "legacy 来源 line should be a warning");

        let extra = format!("{ok}\n多一行\n");
        assert!(structure(&extra).iter().any(|(err, _)| *err));
        let two = format!("{legacy}\n多一行\n");
        assert!(structure(&two).iter().any(|(err, _)| *err));
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
