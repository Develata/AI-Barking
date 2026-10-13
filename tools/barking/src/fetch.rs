//! `barking html2txt` and `barking heat`: the two small fetch-side helpers of
//! the evidence step (docs/workflow/publish.md, EDITORIAL.md 核验流程).
//!
//! `html2txt` turns a saved page into grep-able plain text that keeps every
//! link target, so a claim can be traced without reopening the page. It
//! mirrors the retired per-issue `a-html2txt.py` as sequential whole-string
//! passes (one per rule, each unit-tested) rather than one tokenizer, so any
//! difference from the old archives can be pinned to a single pass.
//!
//! `heat` records how much attention a topic got on Hacker News (Algolia) and
//! Reddit at fetch time. The numbers are evidence of heat on that date only,
//! so every response is kept (items byte-for-byte) next to the summary, and a
//! failed request is written down in `errors` instead of being retried into
//! silence.
//!
//! Both commands share this module because they share nothing but their
//! purpose; `main.rs` therefore routes the two names to the one `run` here,
//! which dispatches on the first argument.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::io::{ErrorKind, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Serialize, Serializer};
use serde_json::Value;
use ureq::ResponseExt as _;

use crate::offsite::write_if_changed;
use crate::repo::{self, Issue};

const USAGE_HTML2TXT: &str = "用法：barking html2txt <in.html> <out.txt> [--force]";
const USAGE_HEAT: &str = "用法：barking heat <期次目录> <组前缀> [--hn \"<查询>\"]... [--reddit \"<查询>\"]... [--item <HN id>]... [--since <unix 秒>]";

/// Same desktop-Chrome string the per-issue `a-heat.py` scripts sent; Reddit's
/// public JSON refuses the default library user agent.
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36";
const HN_SEARCH: &str = "https://hn.algolia.com/api/v1/search";
const HN_ITEM: &str = "https://hn.algolia.com/api/v1/items";
const REDDIT_SEARCH: &str = "https://old.reddit.com/search.json";
const HN_PAGE: &str = "20";
const REDDIT_LIMIT: &str = "25";
const SINCE_DEFAULT_DAYS: i64 = 7;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(40);
/// Attempts per request; only transport errors and 5xx are retried.
const ATTEMPTS: u32 = 3;
const RETRY_PAUSE: Duration = Duration::from_secs(2);
/// A big HN thread is a few MB of JSON; ureq's 10 MB default is close enough
/// to matter, and a refused download would lose the very thing being archived.
const MAX_BODY: u64 = 64 * 1024 * 1024;

pub fn run(args: &[String]) -> ExitCode {
    match args.split_first() {
        Some((cmd, rest)) if cmd == "html2txt" => html2txt_main(rest),
        Some((cmd, rest)) if cmd == "heat" => heat_main(rest),
        _ => {
            eprintln!("{USAGE_HTML2TXT}\n{USAGE_HEAT}");
            ExitCode::from(2)
        }
    }
}

fn usage_error(msg: &str, usage: &str) -> ExitCode {
    eprintln!("error: {msg}\n{usage}");
    ExitCode::from(2)
}

fn finish(result: Result<(), String>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Create `path` and write `bytes`, refusing to replace an existing file
/// (`create_new` makes the check atomic, unlike `exists()` followed by write).
fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => format!("{} 已存在，拒绝覆盖", path.display()),
            _ => format!("{}: {e}", path.display()),
        })?;
    f.write_all(bytes)
        .map_err(|e| format!("{}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// html2txt
// ---------------------------------------------------------------------------

fn html2txt_main(args: &[String]) -> ExitCode {
    let mut force = false;
    let mut paths: Vec<&String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--force" => force = true,
            s if s.starts_with("--") => {
                return usage_error(&format!("未知选项 {s}"), USAGE_HTML2TXT);
            }
            _ => paths.push(a),
        }
    }
    let [input, output] = paths.as_slice() else {
        return usage_error("需要输入与输出两个路径", USAGE_HTML2TXT);
    };
    finish(convert_file(Path::new(input), Path::new(output), force))
}

fn convert_file(input: &Path, output: &Path, force: bool) -> Result<(), String> {
    let raw = fs::read(input).map_err(|e| format!("{}: {e}", input.display()))?;
    // Invalid UTF-8 becomes U+FFFD rather than aborting: a saved page in a
    // legacy charset is still worth grepping.
    let html = String::from_utf8_lossy(&raw);
    let text = html_to_text(html.trim_start_matches('\u{feff}'));
    if let (Ok(a), Ok(b)) = (input.canonicalize(), output.canonicalize())
        && a == b
    {
        return Err("输出与输入是同一个文件".into());
    }
    let what = format!(
        "{} 行，{} 字节；来自 {}",
        text.lines().count(),
        text.len(),
        input.display()
    );
    if force {
        // Prints its own 写入/未变 line.
        write_if_changed(output, &text, &what)
    } else {
        write_new(output, text.as_bytes()).map_err(|e| format!("{e}（要覆盖请加 --force）"))?;
        println!("写入 {}（{what}）", output.display());
        Ok(())
    }
}

/// Tags whose open or close becomes a line break. The set is deliberately
/// narrower than "every block element": table cells stay on their row's line.
const BLOCK_TAGS: [&str; 29] = [
    "p",
    "div",
    "br",
    "li",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "tr",
    "section",
    "article",
    "ul",
    "ol",
    "table",
    "header",
    "footer",
    "main",
    "blockquote",
    "figure",
    "figcaption",
    // Not in the retired script's list, but its prefix match (`<p…`, `<li…`)
    // happened to break on <pre>; the rest are the same kind of boundary.
    "pre",
    "hr",
    "nav",
    "aside",
    "dt",
    "dd",
    "title",
];
/// Elements whose content is never text.
const RAW_TEXT_TAGS: [&str; 4] = ["script", "style", "noscript", "svg"];
/// Longest `&…;` body worth looking at; keeps a stray `&` from scanning on.
const MAX_ENTITY_BODY: usize = 10;

/// Saved HTML → plain text. Pass order matters: entities are decoded only
/// after tags are gone, so a literal `&lt;p&gt;` in the text never turns back
/// into markup.
pub(crate) fn html_to_text(html: &str) -> String {
    // Python's universal newlines did this when the old script read the file.
    let s = fold_newlines(html);
    let s = strip_raw_text_elements(&s);
    let s = strip_comments(&s);
    let s = rewrite_links(&s);
    let s = replace_tags(&s);
    // `&#13;` decodes to a bare CR; fold again so `&#13;&#10;` is one line
    // break, as the same bytes written raw would be.
    let s = fold_newlines(&decode_entities(&s));
    let s = collapse_spaces(&s);
    let s = collapse_blank_lines(&s);
    format!("{}\n", s.trim())
}

fn fold_newlines(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}

/// Per-pass record of searches that came back empty. A pass only moves
/// forward through one string, so "nothing from `p` on" answers every later
/// search from `q >= p` for the same needle without scanning again. Without
/// it a page that never closes a tag, an element or a comment costs one scan
/// to the end per `<`, i.e. quadratic time.
#[derive(Default)]
struct Memo {
    /// No `>` at or after this index.
    no_gt: Option<usize>,
    /// No `-->` at or after this index.
    no_comment_end: Option<usize>,
    /// Lower-cased element name → no `</name…>` at or after the index.
    no_close: Vec<(String, usize)>,
    /// `(index of a '<', open quote byte or 0)` states that a quote-aware
    /// tag scan passed on its way to the end of the string with a quote still
    /// open. The scan from there is a function of that state alone, so a
    /// later scan reaching the same `<` in the same state runs off the end
    /// too.
    runs_off: HashSet<(usize, u8)>,
}

impl Memo {
    fn find_gt(&mut self, b: &[u8], from: usize) -> Option<usize> {
        if self.no_gt.is_some_and(|p| from >= p) {
            return None;
        }
        let found = b
            .get(from..)
            .and_then(|t| t.iter().position(|&c| c == b'>'))
            .map(|p| from + p);
        if found.is_none() {
            self.no_gt = Some(from);
        }
        found
    }

    /// Index just past the `-->` closing the comment that `s[at..]` opens
    /// (it starts with `<!--`).
    fn comment_end(&mut self, s: &str, at: usize) -> Option<usize> {
        let from = at + 4;
        if self.no_comment_end.is_some_and(|p| from >= p) {
            return None;
        }
        let found = s[from..].find("-->").map(|p| from + p + 3);
        if found.is_none() {
            self.no_comment_end = Some(from);
        }
        found
    }
}

/// A start or end tag found at a `<`.
struct Tag<'a> {
    /// Name as written (case preserved).
    name: &'a str,
    closing: bool,
    /// Raw text between the name and the `>`.
    attrs: &'a str,
    /// Index just past the `>`.
    end: usize,
}

/// Parse the tag at `s[at] == '<'`. Like a browser, only `<` followed by a
/// letter (or `</` and a letter) starts a tag, so a bare `a < b` survives as
/// text. `None` also when no `>` ever closes the tag.
///
/// Unlike a browser, the name stops at a `<`: `<p<b>` is a `p` whose
/// attribute text is `<b`, where a browser sees one tag named `p<b`. That
/// bounds the name scan by the next `<`, which keeps a run of `<a<a<a…`
/// linear; the whole tag is dropped either way.
fn parse_tag<'a>(s: &'a str, at: usize, memo: &mut Memo) -> Option<Tag<'a>> {
    let b = s.as_bytes();
    let mut i = at + 1;
    let closing = b.get(i) == Some(&b'/');
    if closing {
        i += 1;
    }
    if !b.get(i)?.is_ascii_alphabetic() {
        return None;
    }
    let name_start = i;
    while i < b.len() && !matches!(b[i], b'>' | b'/' | b'<') && !b[i].is_ascii_whitespace() {
        i += 1;
    }
    // The name ends at an ASCII byte or at the end, so both slices are on
    // char boundaries.
    let gt = tag_end(b, i, memo)?;
    Some(Tag {
        name: &s[name_start..i],
        closing,
        attrs: &s[i..gt],
        end: gt + 1,
    })
}

/// Index of the `>` that ends the tag whose attributes start at `from`. A `>`
/// inside a quoted attribute value (`title="a > b"`, inline handlers, JSON in
/// data attributes) does not end it; an unterminated quote must not swallow
/// the page, so then the first `>` wins. One pass: that first `>` is noted on
/// the way, so the fallback needs no second scan.
fn tag_end(b: &[u8], from: usize, memo: &mut Memo) -> Option<usize> {
    if memo.no_gt.is_some_and(|p| from >= p) {
        return None;
    }
    let mut quote: Option<u8> = None;
    let mut prev = 0u8; // last non-space byte seen outside quotes
    let mut first_gt: Option<usize> = None; // necessarily inside quotes
    let mut passed: Vec<(usize, u8)> = Vec::new();
    let mut i = from;
    // Stops at the end of the string, or where an earlier scan is known to
    // have run off it; either way some quote never closes.
    while let Some(&c) = b.get(i) {
        if c == b'<' {
            let state = (i, quote.unwrap_or(0));
            if memo.runs_off.contains(&state) {
                break;
            }
            passed.push(state);
        }
        match quote {
            Some(q) => {
                if c == q {
                    quote = None;
                    prev = c;
                } else if c == b'>' && first_gt.is_none() {
                    first_gt = Some(i);
                }
            }
            None => match c {
                b'>' => return Some(i),
                b'"' | b'\'' if prev == b'=' => quote = Some(c),
                c if c.is_ascii_whitespace() => {}
                _ => prev = c,
            },
        }
        i += 1;
    }
    memo.runs_off.extend(passed);
    let gt = first_gt.or_else(|| memo.find_gt(b, i));
    if gt.is_none() {
        memo.no_gt = Some(from);
    }
    gt
}

/// Value of attribute `want` (case-insensitive name; double-quoted,
/// single-quoted or bare value). The first occurrence wins, as in a browser.
fn attr_value<'a>(attrs: &'a str, want: &str) -> Option<&'a str> {
    let b = attrs.as_bytes();
    let mut i = 0;
    while i < b.len() {
        while i < b.len() && (b[i].is_ascii_whitespace() || b[i] == b'/') {
            i += 1;
        }
        let name_start = i;
        while i < b.len() && !b[i].is_ascii_whitespace() && !matches!(b[i], b'=' | b'/') {
            i += 1;
        }
        let name = &attrs[name_start..i];
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        let mut value = "";
        if b.get(i) == Some(&b'=') {
            i += 1;
            while i < b.len() && b[i].is_ascii_whitespace() {
                i += 1;
            }
            match b.get(i) {
                Some(&q) if q == b'"' || q == b'\'' => {
                    let start = i + 1;
                    let stop = attrs[start..]
                        .find(q as char)
                        .map_or(b.len(), |p| start + p);
                    value = &attrs[start..stop];
                    i = (stop + 1).min(b.len());
                }
                _ => {
                    let start = i;
                    while i < b.len() && !b[i].is_ascii_whitespace() {
                        i += 1;
                    }
                    value = &attrs[start..i];
                }
            }
        }
        if name.eq_ignore_ascii_case(want) {
            return Some(value);
        }
    }
    None
}

/// Copy `s`, giving `f` the chance to consume the markup at each `<`. `f`
/// appends its replacement to the output and returns the index to resume at,
/// or `None` to keep that `<` as plain text.
fn rewrite(s: &str, mut f: impl FnMut(&str, usize, &mut String) -> Option<usize>) -> String {
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while let Some(off) = s[i..].find('<') {
        let at = i + off;
        out.push_str(&s[i..at]);
        match f(s, at, &mut out) {
            Some(next) => i = next,
            None => {
                out.push('<');
                i = at + 1;
            }
        }
    }
    out.push_str(&s[i..]);
    out
}

/// `(start, end)` of the first `</name>` at or after `from` (name matched
/// case-insensitively), `end` being just past its `>`.
fn find_close(s: &str, from: usize, name: &str, memo: &mut Memo) -> Option<(usize, usize)> {
    let key = name.to_ascii_lowercase();
    if memo.no_close.iter().any(|(n, p)| *n == key && from >= *p) {
        return None;
    }
    let b = s.as_bytes();
    let mut i = from;
    let found = loop {
        let Some(off) = s[i..].find("</") else {
            break None;
        };
        let at = i + off;
        let after = at + 2 + name.len();
        if b.get(at + 2..after)
            .is_some_and(|n| n.eq_ignore_ascii_case(name.as_bytes()))
            && b.get(after)
                .is_some_and(|&c| matches!(c, b'>' | b'/') || c.is_ascii_whitespace())
        {
            // No `>` after this one means no end tag can follow it either.
            break memo.find_gt(b, after).map(|gt| (at, gt + 1));
        }
        i = at + 2;
    };
    if found.is_none() {
        memo.no_close.push((key, from));
    }
    found
}

/// Drop `<script|style|noscript|svg>…</…>` with their content. An element
/// that is never closed is left alone (its tags go in a later pass) rather
/// than eating the rest of the page.
///
/// Every other tag, and every comment, is copied whole and stepped over, so a
/// `<script>` that is only text inside an attribute value or a comment opens
/// nothing. Comments are skipped here rather than stripped in an earlier pass
/// because the reverse nesting occurs too: legacy scripts wrap their code in
/// `<!-- … -->` and script strings contain `<!--`, which, stripped first,
/// would run on to some later `-->` and leave script text behind.
fn strip_raw_text_elements(s: &str) -> String {
    let mut memo = Memo::default();
    rewrite(s, |s, at, out| {
        if s[at..].starts_with("<!--") {
            let end = memo.comment_end(s, at)?;
            out.push_str(&s[at..end]);
            return Some(end);
        }
        let tag = parse_tag(s, at, &mut memo)?;
        if !tag.closing
            && RAW_TEXT_TAGS
                .iter()
                .any(|n| tag.name.eq_ignore_ascii_case(n))
            && let Some((_, end)) = find_close(s, tag.end, tag.name, &mut memo)
        {
            return Some(end);
        }
        out.push_str(&s[at..tag.end]);
        Some(tag.end)
    })
}

/// Drop `<!-- … -->`. Every tag is copied whole and stepped over, as in
/// `strip_raw_text_elements`, so a `<!--` inside an attribute value
/// (`title="<!--"`) opens no comment and cannot eat the text after it.
fn strip_comments(s: &str) -> String {
    let mut memo = Memo::default();
    rewrite(s, |s, at, out| {
        if s[at..].starts_with("<!--") {
            return memo.comment_end(s, at);
        }
        let tag = parse_tag(s, at, &mut memo)?;
        out.push_str(&s[at..tag.end]);
        Some(tag.end)
    })
}

/// `<a href="URL">text</a>` → `text [URL]`. Anchors without an `href`, with
/// an empty one, or never closed keep their text and lose the brackets.
/// Every other tag is copied whole, as in `strip_raw_text_elements`.
fn rewrite_links(s: &str) -> String {
    let mut memo = Memo::default();
    rewrite(s, |s, at, out| {
        let tag = parse_tag(s, at, &mut memo)?;
        if !tag.closing
            && tag.name.eq_ignore_ascii_case("a")
            && let Some(href) = attr_value(tag.attrs, "href")
            && let Some((close, end)) = find_close(s, tag.end, tag.name, &mut memo)
        {
            out.push_str(&s[tag.end..close]);
            if !href.trim().is_empty() {
                out.push_str(" [");
                out.push_str(href);
                out.push(']');
            }
            return Some(end);
        }
        out.push_str(&s[at..tag.end]);
        Some(tag.end)
    })
}

/// Block tags become a newline, every other tag, doctype and processing
/// instruction disappears.
fn replace_tags(s: &str) -> String {
    let mut memo = Memo::default();
    rewrite(s, |s, at, out| {
        if matches!(s.as_bytes().get(at + 1)?, b'!' | b'?') {
            return memo.find_gt(s.as_bytes(), at).map(|p| p + 1);
        }
        let tag = parse_tag(s, at, &mut memo)?;
        if BLOCK_TAGS.iter().any(|n| tag.name.eq_ignore_ascii_case(n)) {
            out.push('\n');
        }
        Some(tag.end)
    })
}

/// Windows-1252 meanings of the C1 range, which is what a browser (and
/// Python's `html.unescape`) does with `&#150;` and friends.
const CP1252_C1: [char; 32] = [
    '€', '\u{81}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{8d}', 'Ž', '\u{8f}',
    '\u{90}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\u{9d}', 'ž', 'Ÿ',
];

/// Names of U+00A0..=U+00FF in code-point order: the whole Latin-1 block
/// (HTML 4's `HTMLlat1` set), so accented names in European press pages
/// (`&eacute;`, `&ntilde;`) come out as letters.
const LATIN1_ENTITIES: [&str; 96] = [
    "nbsp", "iexcl", "cent", "pound", "curren", "yen", "brvbar", "sect", "uml", "copy", "ordf",
    "laquo", "not", "shy", "reg", "macr", //
    "deg", "plusmn", "sup2", "sup3", "acute", "micro", "para", "middot", "cedil", "sup1", "ordm",
    "raquo", "frac14", "frac12", "frac34", "iquest", //
    "Agrave", "Aacute", "Acirc", "Atilde", "Auml", "Aring", "AElig", "Ccedil", "Egrave", "Eacute",
    "Ecirc", "Euml", "Igrave", "Iacute", "Icirc", "Iuml", //
    "ETH", "Ntilde", "Ograve", "Oacute", "Ocirc", "Otilde", "Ouml", "times", "Oslash", "Ugrave",
    "Uacute", "Ucirc", "Uuml", "Yacute", "THORN", "szlig", //
    "agrave", "aacute", "acirc", "atilde", "auml", "aring", "aelig", "ccedil", "egrave", "eacute",
    "ecirc", "euml", "igrave", "iacute", "icirc", "iuml", //
    "eth", "ntilde", "ograve", "oacute", "ocirc", "otilde", "ouml", "divide", "oslash", "ugrave",
    "uacute", "ucirc", "uuml", "yacute", "thorn", "yuml",
];

/// The Latin-1 block plus the common names outside it. Not the full HTML
/// table: anything else stays as written, which is visible and greppable
/// rather than silently wrong. Names are case-sensitive (`&Eacute;` ≠
/// `&eacute;`); the upper-case aliases below are the ones HTML defines.
fn named_entity(name: &str) -> Option<char> {
    if let Some(i) = LATIN1_ENTITIES.iter().position(|&n| n == name) {
        return char::from_u32(0xa0 + i as u32);
    }
    Some(match name {
        "amp" | "AMP" => '&',
        "lt" | "LT" => '<',
        "gt" | "GT" => '>',
        "quot" | "QUOT" => '"',
        "apos" => '\'',
        "COPY" => '©',
        "REG" => '®',
        "mdash" => '—',
        "ndash" => '–',
        "hellip" => '…',
        "ldquo" => '“',
        "rdquo" => '”',
        "lsquo" => '‘',
        "rsquo" => '’',
        "trade" => '™',
        "bull" => '•',
        "euro" => '€',
        "dagger" => '†',
        "Dagger" => '‡',
        "larr" => '←',
        "rarr" => '→',
        "uarr" => '↑',
        "darr" => '↓',
        "minus" => '−',
        "equiv" => '≡',
        _ => return None,
    })
}
/// `body` is what follows `&#`, without the `;`: decimal digits, or `x`/`X`
/// and hex digits.
fn numeric_entity(body: &str) -> Option<char> {
    let (digits, radix) = match body.strip_prefix(['x', 'X']) {
        Some(hex) => (hex, 16),
        None => (body, 10),
    };
    if digits.is_empty() || digits.len() > 8 || !digits.chars().all(|c| c.is_digit(radix)) {
        return None;
    }
    let cp = u32::from_str_radix(digits, radix).ok()?;
    Some(match cp {
        0x80..=0x9f => CP1252_C1[(cp - 0x80) as usize],
        _ => char::from_u32(cp)
            .filter(|&c| c != '\0')
            .unwrap_or('\u{fffd}'),
    })
}

/// One left-to-right pass, so `&amp;lt;` becomes `&lt;`, not `<`. A reference
/// needs its `;`; unknown ones are kept verbatim.
fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find('&') {
        out.push_str(&rest[..p]);
        let tail = &rest[p + 1..];
        let decoded = tail
            .bytes()
            .take(MAX_ENTITY_BODY + 1)
            .position(|b| b == b';')
            .and_then(|semi| {
                let body = &tail[..semi];
                let c = match body.strip_prefix('#') {
                    Some(num) => numeric_entity(num),
                    None => named_entity(body),
                };
                c.map(|c| (c, semi))
            });
        match decoded {
            Some((c, semi)) => {
                out.push(c);
                rest = &tail[semi + 1..];
            }
            None => {
                out.push('&');
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Runs of space, tab and no-break space become one space. Line edges are not
/// trimmed (the archived `.txt` files are not either).
fn collapse_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_run = false;
    for c in s.chars() {
        if matches!(c, ' ' | '\t' | '\u{a0}') {
            if !in_run {
                out.push(' ');
            }
            in_run = true;
        } else {
            out.push(c);
            in_run = false;
        }
    }
    out
}

/// In every whitespace run holding two or more newlines, the stretch from
/// the first newline to the last becomes exactly one blank line; whatever
/// whitespace precedes the first or follows the last newline stays.
fn collapse_blank_lines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find(char::is_whitespace) {
        out.push_str(&rest[..p]);
        let tail = &rest[p..];
        let run_len = tail
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(tail.len());
        let run = &tail[..run_len];
        match (run.find('\n'), run.rfind('\n')) {
            (Some(first), Some(last)) if first != last => {
                out.push_str(&run[..first]);
                out.push_str("\n\n");
                out.push_str(&run[last + 1..]);
            }
            _ => out.push_str(run),
        }
        rest = &tail[run_len..];
    }
    out.push_str(rest);
    out
}

// ---------------------------------------------------------------------------
// heat
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
struct HeatArgs {
    issue_dir: PathBuf,
    group: String,
    hn: Vec<String>,
    reddit: Vec<String>,
    items: Vec<String>,
    since: Option<i64>,
}

/// Keep the first occurrence of each value, in order: a repeated query would
/// only overwrite its own key in the output.
fn dedup(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    values
        .into_iter()
        .filter(|v| seen.insert(v.clone()))
        .collect()
}

fn parse_heat_args(args: &[String]) -> Result<HeatArgs, String> {
    let mut positional: Vec<&String> = Vec::new();
    let (mut hn, mut reddit, mut items) = (Vec::new(), Vec::new(), Vec::new());
    let mut since: Option<i64> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let flag = a.as_str();
        if !matches!(flag, "--hn" | "--reddit" | "--item" | "--since") {
            if flag.starts_with("--") {
                return Err(format!("未知选项 {flag}"));
            }
            positional.push(a);
            continue;
        }
        let value = it.next().ok_or_else(|| format!("{flag} 缺少参数"))?;
        match flag {
            "--hn" | "--reddit" => {
                if value.trim().is_empty() {
                    return Err(format!("{flag} 的查询不能为空"));
                }
                if flag == "--hn" { &mut hn } else { &mut reddit }.push(value.clone());
            }
            "--item" => {
                if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(format!("--item 须为 HN 条目数字 id，收到「{value}」"));
                }
                items.push(value.clone());
            }
            _ => {
                if since.is_some() {
                    return Err("--since 只能给一次".into());
                }
                since = Some(
                    value
                        .parse::<i64>()
                        .ok()
                        .filter(|&n| n >= 0)
                        .ok_or_else(|| format!("--since 须为非负整数 unix 秒，收到「{value}」"))?,
                );
            }
        }
    }
    let [dir, group] = positional.as_slice() else {
        return Err("需要期次目录与组前缀两个位置参数".into());
    };
    // The prefix becomes part of file names next to the group's other
    // archives (`a-heat.json`, `a-hn-<id>.json`).
    if group.is_empty()
        || group.starts_with('-')
        || !group
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(format!("组前缀只允许小写字母、数字与 -，收到「{group}」"));
    }
    if hn.is_empty() && reddit.is_empty() && items.is_empty() {
        return Err("至少给一个 --hn、--reddit 或 --item".into());
    }
    Ok(HeatArgs {
        issue_dir: PathBuf::from(dir),
        group: group.to_string(),
        hn: dedup(hn),
        reddit: dedup(reddit),
        items: dedup(items),
        since,
    })
}

fn heat_main(args: &[String]) -> ExitCode {
    let parsed = match parse_heat_args(args) {
        Ok(p) => p,
        Err(e) => return usage_error(&e, USAGE_HEAT),
    };
    finish(Issue::resolve(&parsed.issue_dir).and_then(|issue| heat(&issue, &parsed)))
}

/// A map that serialises in insertion order. `serde_json::Map` would sort the
/// query keys alphabetically (no `preserve_order` feature), and the order the
/// queries were given in is part of the record.
struct Ordered<T>(Vec<(String, Vec<T>)>);

impl<T: Serialize> Serialize for Ordered<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(k, v)| (k, v)))
    }
}

/// A field the API left out stays `None` (JSON `null`): a made-up `0` or
/// 1970 date would read as a measurement.
#[derive(Serialize, Debug, PartialEq)]
struct HnHit {
    id: String,
    points: Option<i64>,
    comments: Option<i64>,
    time_bj: Option<String>,
    title: String,
    url: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
struct RedditHit {
    sub: String,
    score: Option<i64>,
    comments: Option<i64>,
    time_bj: Option<String>,
    title: String,
    permalink: String,
    url: String,
}

#[derive(Serialize)]
struct Heat {
    fetched_bj: String,
    hn: Ordered<HnHit>,
    reddit: Ordered<RedditHit>,
    /// Ids whose thread was saved as `<组>-hn-<id>.json`.
    items: Vec<String>,
    errors: Vec<String>,
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn default_since(now: i64) -> i64 {
    now - SINCE_DEFAULT_DAYS * 86_400
}

/// `MM-DD HH:MM` out of a `repo::beijing_stamp`, for the one-line terminal
/// summary; `-` when the time is unknown.
fn short_time(stamp: Option<&str>) -> String {
    match stamp {
        Some(t) => t.get(5..16).unwrap_or(t).replace('T', " "),
        None => "-".into(),
    }
}

/// A count for the terminal summary; `-` when the API gave none.
fn count(n: Option<i64>) -> String {
    n.map_or_else(|| "-".into(), |n| n.to_string())
}

fn truncate_chars(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((cut, _)) => format!("{}…", &s[..cut]),
        None => s.to_string(),
    }
}

/// `(nbHits, hits)` from an Algolia search response. A hit without an
/// `objectID` cannot be cited and is dropped.
fn parse_hn_hits(v: &Value) -> Result<(u64, Vec<HnHit>), String> {
    let hits = v["hits"].as_array().ok_or("响应缺少 hits 数组")?;
    let out = hits
        .iter()
        .filter_map(|h| {
            Some(HnHit {
                id: h["objectID"].as_str()?.to_string(),
                points: h["points"].as_i64(),
                comments: h["num_comments"].as_i64(),
                time_bj: h["created_at_i"].as_i64().map(repo::beijing_stamp),
                title: h["title"].as_str().unwrap_or_default().to_string(),
                url: h["url"].as_str().map(str::to_string),
            })
        })
        .collect();
    Ok((v["nbHits"].as_u64().unwrap_or(hits.len() as u64), out))
}

/// Reddit listing → hits. `created_utc` arrives as a float.
fn parse_reddit_hits(v: &Value) -> Result<Vec<RedditHit>, String> {
    let children = v["data"]["children"]
        .as_array()
        .ok_or("响应缺少 data.children 数组")?;
    Ok(children
        .iter()
        .map(|c| &c["data"])
        .map(|d| RedditHit {
            sub: d["subreddit"].as_str().unwrap_or_default().to_string(),
            score: d["score"].as_i64(),
            comments: d["num_comments"].as_i64(),
            time_bj: d["created_utc"]
                .as_f64()
                .map(|t| repo::beijing_stamp(t as i64)),
            title: d["title"].as_str().unwrap_or_default().to_string(),
            permalink: format!(
                "https://www.reddit.com{}",
                d["permalink"].as_str().unwrap_or_default()
            ),
            url: d["url"].as_str().unwrap_or_default().to_string(),
        })
        .collect())
}

/// Number of comment nodes under an HN item (the item itself not counted).
fn count_descendants(item: &Value) -> usize {
    let mut n = 0;
    let mut stack = vec![item];
    while let Some(node) = stack.pop() {
        if let Some(children) = node["children"].as_array() {
            n += children.len();
            stack.extend(children);
        }
    }
    n
}

fn http_agent() -> ureq::Agent {
    // Status codes are inspected by `http_get` so that a 429 can be recorded
    // with its number instead of as an opaque transport error.
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(REQUEST_TIMEOUT))
        .user_agent(USER_AGENT)
        .build()
        .into()
}

/// A 2xx response body and where it really came from. Redirects are
/// followed, so when Reddit answers a blocked search with a login or block
/// page, the final URL and type are what explain the failure.
struct Fetched {
    body: Vec<u8>,
    final_url: String,
    content_type: String,
}

impl Fetched {
    fn describe(&self) -> String {
        format!(
            "最终 URL {}，Content-Type {}，{} 字节",
            self.final_url,
            self.content_type,
            self.body.len()
        )
    }

    fn json(&self) -> Result<Value, String> {
        serde_json::from_slice(&self.body)
            .map_err(|e| format!("响应不是 JSON（{}）：{e}", self.describe()))
    }
}

/// GET `url` with `query`. Transport errors and 5xx are retried; a 4xx (rate
/// limit, block) is final, since asking again at once only digs the hole
/// deeper.
fn http_get(agent: &ureq::Agent, url: &str, query: &[(&str, &str)]) -> Result<Fetched, String> {
    let mut last = String::new();
    for attempt in 0..ATTEMPTS {
        if attempt > 0 {
            std::thread::sleep(RETRY_PAUSE);
        }
        let mut req = agent.get(url);
        for &(k, v) in query {
            req = req.query(k, v);
        }
        match req.call() {
            Ok(mut resp) => {
                let status = resp.status();
                let final_url = resp.get_uri().to_string();
                if status.is_success() {
                    let content_type = resp
                        .headers()
                        .get("content-type")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("（无）")
                        .to_string();
                    let body = resp
                        .body_mut()
                        .with_config()
                        .limit(MAX_BODY)
                        .read_to_vec()
                        .map_err(|e| format!("读取响应失败：{e}"))?;
                    return Ok(Fetched {
                        body,
                        final_url,
                        content_type,
                    });
                }
                last = format!("HTTP {status}（最终 URL {final_url}）");
                if status.is_client_error() {
                    break;
                }
            }
            Err(e) => last = format!("请求失败：{e}"),
        }
    }
    Err(last)
}

fn get_json(agent: &ureq::Agent, url: &str, query: &[(&str, &str)]) -> Result<Value, String> {
    http_get(agent, url, query)?.json()
}

/// One JSON GET for other subcommands, judged by the body rather than the
/// status: fxtwitter answers a user lookup with 404 plus a full JSON object
/// (`code: 200`) for accounts without tweets, so the status alone misleads.
pub(crate) fn get_json_any_status(url: &str) -> Result<Value, String> {
    let mut resp = http_agent()
        .get(url)
        .call()
        .map_err(|e| format!("请求失败：{e}"))?;
    let status = resp.status();
    let body = resp
        .body_mut()
        .with_config()
        .limit(MAX_BODY)
        .read_to_vec()
        .map_err(|e| format!("读取响应失败：{e}"))?;
    serde_json::from_slice(&body).map_err(|e| format!("HTTP {status}，响应不是 JSON：{e}"))
}

/// A failed request is not fatal: it is shown now and kept in the record.
fn record_error(errors: &mut Vec<String>, msg: String) {
    println!("warn: {msg}");
    errors.push(msg);
}

/// Whether an item response is worth archiving: the body of a JSON object.
/// Deliberately not "serde accepts it" — the file is the evidence, and a
/// response this parser happens to reject is still the response.
fn looks_like_json_object(body: &[u8]) -> bool {
    body.iter()
        .find(|b| !b.is_ascii_whitespace())
        .is_some_and(|&b| b == b'{')
}

/// Fetch one HN item and save the body byte-for-byte. Parsing only feeds the
/// summary line; if it fails the file stays and a warning says why.
/// `shown` is the path as printed (repo-relative, like the other lines).
fn fetch_item(
    agent: &ureq::Agent,
    id: &str,
    sources: &Path,
    file: &str,
    shown: &str,
) -> Result<(), String> {
    let resp = http_get(agent, &format!("{HN_ITEM}/{id}"), &[])?;
    if !looks_like_json_object(&resp.body) {
        return Err(format!("响应不是 JSON 对象，未保存（{}）", resp.describe()));
    }
    fs::create_dir_all(sources).map_err(|e| format!("{}: {e}", sources.display()))?;
    let path = sources.join(file);
    write_new(&path, &resp.body)?;
    match resp.json() {
        Ok(v) => println!(
            "== HN 条目 {id}：{}（{} 分；含嵌套评论 {} 条）→ {}",
            truncate_chars(v["title"].as_str().unwrap_or("无标题"), 90),
            count(v["points"].as_i64()),
            count_descendants(&v),
            shown
        ),
        Err(e) => println!("warn: HN 条目 {id} 已原样保存到 {shown}，但无法解析摘要：{e}"),
    }
    Ok(())
}

fn heat(issue: &Issue, a: &HeatArgs) -> Result<(), String> {
    let sources = issue.sources();
    let heat_name = format!("{}-heat.json", a.group);
    let item_name = |id: &str| format!("{}-hn-{id}.json", a.group);
    // Refuse before any request: a half-fetched record next to an old one
    // would mix two moments in time.
    let existing: Vec<String> = std::iter::once(heat_name.clone())
        .chain(a.items.iter().map(|id| item_name(id)))
        .filter(|n| sources.join(n).exists())
        .collect();
    if !existing.is_empty() {
        return Err(format!(
            "{}/sources/ 下已有 {}，拒绝覆盖（改用别的组前缀，或先删除）",
            issue.rel(),
            existing.join("、")
        ));
    }

    let since = a.since.unwrap_or_else(|| default_since(unix_now()));
    let agent = http_agent();
    let mut heat = Heat {
        fetched_bj: repo::beijing_now(),
        hn: Ordered(Vec::new()),
        reddit: Ordered(Vec::new()),
        items: Vec::new(),
        errors: Vec::new(),
    };
    let requests = a.hn.len() + a.reddit.len() + a.items.len();
    let mut ok = 0usize;
    if a.hn.is_empty() {
        println!("取得时刻 {}", heat.fetched_bj);
    } else {
        println!(
            "取得时刻 {}；HN 搜索自 {} 起",
            heat.fetched_bj,
            repo::beijing_stamp(since)
        );
    }

    for q in &a.hn {
        let numeric = format!("created_at_i>{since}");
        let query = [
            ("query", q.as_str()),
            ("tags", "story"),
            ("numericFilters", numeric.as_str()),
            ("hitsPerPage", HN_PAGE),
        ];
        match get_json(&agent, HN_SEARCH, &query).and_then(|v| parse_hn_hits(&v)) {
            Ok((total, hits)) => {
                println!("== HN「{q}」：命中 {total} 条，取前 {}", hits.len());
                for h in &hits {
                    println!(
                        "  {} {:>4} 分 {:>4} 评 {}  {}  {}",
                        h.id,
                        count(h.points),
                        count(h.comments),
                        short_time(h.time_bj.as_deref()),
                        truncate_chars(&h.title, 90),
                        h.url.as_deref().unwrap_or("-")
                    );
                }
                heat.hn.0.push((q.clone(), hits));
                ok += 1;
            }
            Err(e) => record_error(&mut heat.errors, format!("HN 搜索「{q}」：{e}")),
        }
    }

    for q in &a.reddit {
        let query = [
            ("q", q.as_str()),
            ("sort", "relevance"),
            ("t", "week"),
            ("limit", REDDIT_LIMIT),
        ];
        match get_json(&agent, REDDIT_SEARCH, &query).and_then(|v| parse_reddit_hits(&v)) {
            Ok(hits) => {
                println!("== Reddit「{q}」：{} 条", hits.len());
                for h in &hits {
                    println!(
                        "  r/{} {:>4} 分 {:>4} 评 {}  {}  {}",
                        h.sub,
                        count(h.score),
                        count(h.comments),
                        short_time(h.time_bj.as_deref()),
                        truncate_chars(&h.title, 80),
                        h.permalink
                    );
                }
                heat.reddit.0.push((q.clone(), hits));
                ok += 1;
            }
            Err(e) => record_error(&mut heat.errors, format!("Reddit 搜索「{q}」：{e}")),
        }
    }

    for id in &a.items {
        let file = item_name(id);
        let shown = format!("{}/sources/{file}", issue.rel());
        match fetch_item(&agent, id, &sources, &file, &shown) {
            Ok(()) => {
                heat.items.push(id.clone());
                ok += 1;
            }
            Err(e) => record_error(&mut heat.errors, format!("HN 条目 {id}：{e}")),
        }
    }

    write_heat(&sources, &heat_name, &heat)?;
    let written = format!("{}/sources/{heat_name}", issue.rel());
    if ok == 0 {
        return Err(format!(
            "{requests} 个请求全部失败；失败原因已写入 {written} 的 errors（重跑须换组前缀或先删除该文件）"
        ));
    }
    println!(
        "-- 写入 {written}（HN {} 组、Reddit {} 组、条目 {} 个；{} 个请求失败，已记入 errors）",
        heat.hn.0.len(),
        heat.reddit.0.len(),
        heat.items.len(),
        heat.errors.len()
    );
    Ok(())
}

/// The record is written even when every request failed: "asked at this
/// time and got these errors" is itself the evidence (of a block, a rate
/// limit, an id that does not exist).
fn write_heat(sources: &Path, name: &str, heat: &Heat) -> Result<(), String> {
    fs::create_dir_all(sources).map_err(|e| format!("{}: {e}", sources.display()))?;
    let mut json = serde_json::to_string_pretty(heat).map_err(|e| e.to_string())?;
    json.push('\n');
    write_new(&sources.join(name), json.as_bytes())
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    // ---- html2txt, one test per rule ----

    #[test]
    fn drops_script_style_noscript_svg_with_content() {
        let html = "a<script>var x = '<b>';</script>b<style>p{}</style>c<noscript>n</noscript>d<svg><text>t</text></svg>e";
        assert_eq!(strip_raw_text_elements(html), "abcde");
        // Case-insensitive, attributes allowed, other tags untouched.
        assert_eq!(
            strip_raw_text_elements("x<SCRIPT type=\"a\">1</Script >y<scripted>z</scripted>"),
            "xy<scripted>z</scripted>"
        );
    }

    #[test]
    fn unclosed_raw_text_element_is_kept() {
        assert_eq!(
            strip_raw_text_elements("a<script>b"),
            "a<script>b",
            "no end tag: do not eat the rest"
        );
        assert_eq!(html_to_text("a<script>b"), "ab\n");
    }

    #[test]
    fn drops_comments() {
        assert_eq!(strip_comments("a<!-- <p>x</p> -->b<!---->c"), "abc");
        assert_eq!(strip_comments("a<!-- open"), "a<!-- open");
        // `<!--` inside an attribute value is not a comment opener.
        let html = "<p title=\"<!--\">价格20美元</p><!--x--><p>尾</p>";
        assert_eq!(
            strip_comments(html),
            "<p title=\"<!--\">价格20美元</p><p>尾</p>"
        );
        let text = html_to_text(html);
        assert!(text.contains("价格20美元"), "{text:?}");
        assert!(text.contains('尾'), "{text:?}");
    }

    #[test]
    fn links_become_text_then_href() {
        assert_eq!(
            rewrite_links(
                "<a class=\"x\" href=\"https://e.com/a?b=1&amp;c\">Click <b>here</b></a>."
            ),
            "Click <b>here</b> [https://e.com/a?b=1&amp;c]."
        );
        // Single quotes, bare values and upper case are links too.
        assert_eq!(rewrite_links("<A HREF='/p'>p</A>"), "p [/p]");
        assert_eq!(rewrite_links("<a href=/q>q</a>"), "q [/q]");
        // Attribute order, other attributes naming href, tag name prefix.
        assert_eq!(
            rewrite_links("<a data-href=\"no\" href=\"yes\">t</a>"),
            "t [yes]"
        );
        assert_eq!(
            rewrite_links("<abbr href=\"x\">t</abbr>"),
            "<abbr href=\"x\">t</abbr>"
        );
    }

    #[test]
    fn links_without_usable_href_keep_their_tags_for_later_stripping() {
        assert_eq!(rewrite_links("<a name=\"n\">t</a>"), "<a name=\"n\">t</a>");
        assert_eq!(rewrite_links("<a href=\"\">t</a>"), "t");
        assert_eq!(
            rewrite_links("<a href=\"x\">never closed"),
            "<a href=\"x\">never closed"
        );
        // The attribute value may hold a '>'.
        assert_eq!(
            rewrite_links("<a title=\"a > b\" href=\"/h\">t</a>"),
            "t [/h]"
        );
    }

    #[test]
    fn block_tags_break_lines_and_others_vanish() {
        assert_eq!(
            replace_tags("<p>a</p><div>b</div>c<br/>d<br>e<li>f<h2 id=x>g</h2>"),
            "\na\n\nb\nc\nd\ne\nf\ng\n"
        );
        for tag in [
            "p",
            "div",
            "br",
            "li",
            "h1",
            "h6",
            "tr",
            "section",
            "article",
            "ul",
            "ol",
            "table",
            "header",
            "footer",
            "main",
            "blockquote",
            "figure",
            "figcaption",
            "PRE",
            "hr",
            "nav",
            "aside",
            "dt",
            "dd",
            "title",
        ] {
            assert_eq!(replace_tags(&format!("a<{tag}>b")), "a\nb", "<{tag}>");
            assert_eq!(replace_tags(&format!("a</{tag}>b")), "a\nb", "</{tag}>");
        }
        // Inline tags and table cells do not break; neither do near-misses.
        assert_eq!(
            replace_tags("<span>a</span><td>b</td><b>c</b><pb>d"),
            "abcd"
        );
        assert_eq!(replace_tags("<!DOCTYPE html><?xml v?>x"), "x");
    }

    #[test]
    fn bare_angle_bracket_is_text() {
        assert_eq!(replace_tags("a < b and c > d, 1<2"), "a < b and c > d, 1<2");
        assert_eq!(replace_tags("<p>x <"), "\nx <");
        assert_eq!(replace_tags("<a title=\"1 > 0\">x</a>"), "x");
    }

    #[test]
    fn decodes_listed_named_entities() {
        assert_eq!(
            decode_entities(
                "&amp;&lt;&gt;&quot;&apos;&nbsp;&mdash;&ndash;&hellip;&ldquo;&rdquo;&lsquo;&rsquo;&copy;&reg;"
            ),
            "&<>\"'\u{a0}—–…“”‘’©®"
        );
        assert_eq!(decode_entities("&trade; &rarr;"), "™ →");
    }

    #[test]
    fn decodes_numeric_entities() {
        assert_eq!(decode_entities("&#65;&#x42;&#X43;&#128512;"), "ABC😀");
        // C1 numbers mean Windows-1252, as in browsers.
        assert_eq!(decode_entities("&#150;&#x93;"), "–“");
        // Nonsense becomes U+FFFD; malformed stays as written.
        assert_eq!(
            decode_entities("&#0;&#xD800;&#x110000;"),
            "\u{fffd}\u{fffd}\u{fffd}"
        );
        assert_eq!(
            decode_entities("&#;&#x;&#12a;&#123456789;"),
            "&#;&#x;&#12a;&#123456789;"
        );
    }

    #[test]
    fn unknown_or_unterminated_entities_stay_verbatim() {
        assert_eq!(
            decode_entities("AT&T &foo; &amp x &"),
            "AT&T &foo; &amp x &"
        );
        assert_eq!(
            decode_entities("&aaaaaaaaaaaaaaaaaaaa;"),
            "&aaaaaaaaaaaaaaaaaaaa;"
        );
    }

    #[test]
    fn decoding_is_single_pass() {
        assert_eq!(decode_entities("&amp;lt;p&amp;gt;"), "&lt;p&gt;");
        // Escaped markup in text never becomes a tag.
        assert_eq!(
            html_to_text("<p>&lt;script&gt;x&lt;/script&gt;</p>"),
            "<script>x</script>\n"
        );
    }

    #[test]
    fn collapses_space_tab_and_nbsp_only() {
        assert_eq!(collapse_spaces("a \t\u{a0} b\u{a0}c"), "a b c");
        assert_eq!(collapse_spaces("a\n  \n b"), "a\n \n b");
    }

    #[test]
    fn collapses_blank_lines() {
        assert_eq!(collapse_blank_lines("a\n\nb"), "a\n\nb");
        assert_eq!(collapse_blank_lines("a\n\n\n\nb"), "a\n\nb");
        assert_eq!(collapse_blank_lines("a\nb"), "a\nb");
        // Lines holding only spaces count as blank; the next line keeps its
        // own leading space.
        assert_eq!(collapse_blank_lines("a\n \n \n b"), "a\n\n b");
        assert_eq!(collapse_blank_lines("a \n\n\nb"), "a \n\nb");
    }

    #[test]
    fn whole_pipeline() {
        let html = "<!DOCTYPE html><html><head><title>T</title><style>x{}</style></head>\r\n<body>\r\n<!-- c --><h1>Head&nbsp;line</h1>\r\n\r\n\r\n<p>See <a href=\"https://a.example/x\">the <i>report</i></a>.</p><script>evil()</script>   \r\n</body></html>";
        assert_eq!(
            html_to_text(html),
            "T\n\nHead line\n\nSee the report [https://a.example/x].\n"
        );
        assert_eq!(html_to_text(""), "\n");
        assert_eq!(html_to_text("\u{a0} \n "), "\n");
    }

    #[test]
    fn convert_file_refuses_existing_output_without_force() {
        let dir = std::env::temp_dir().join(format!("barking-fetch-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let (input, output) = (dir.join("in.html"), dir.join("out.txt"));
        fs::write(&input, "<p>hi</p>").unwrap();
        convert_file(&input, &output, false).unwrap();
        assert_eq!(fs::read_to_string(&output).unwrap(), "hi\n");
        let err = convert_file(&input, &output, false).unwrap_err();
        assert!(err.contains("--force"), "{err}");
        fs::write(&input, "<p>new</p>").unwrap();
        convert_file(&input, &output, true).unwrap();
        assert_eq!(fs::read_to_string(&output).unwrap(), "new\n");
        assert!(
            convert_file(&input, &input, true)
                .unwrap_err()
                .contains("同一个文件")
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    // ---- heat ----

    #[test]
    fn heat_args_parse() {
        let a = parse_heat_args(&s(&[
            "docs/2610/1010",
            "a",
            "--hn",
            "Claude Haiku 5.5",
            "--hn",
            "Claude Haiku 5.5",
            "--reddit",
            "haiku",
            "--item",
            "123",
            "--since",
            "1790000000",
        ]))
        .unwrap();
        assert_eq!(
            a,
            HeatArgs {
                issue_dir: PathBuf::from("docs/2610/1010"),
                group: "a".into(),
                hn: s(&["Claude Haiku 5.5"]),
                reddit: s(&["haiku"]),
                items: s(&["123"]),
                since: Some(1_790_000_000),
            }
        );
    }

    #[test]
    fn heat_args_reject_bad_input() {
        let bad = |v: &[&str]| parse_heat_args(&s(v)).unwrap_err();
        assert!(bad(&["d", "a"]).contains("至少"));
        assert!(bad(&["d", "--hn", "q"]).contains("两个位置参数"));
        assert!(bad(&["d", "A", "--hn", "q"]).contains("组前缀"));
        assert!(bad(&["d", "-a", "--hn", "q"]).contains("组前缀"));
        assert!(bad(&["d", "a", "--hn"]).contains("缺少参数"));
        assert!(bad(&["d", "a", "--hn", " "]).contains("不能为空"));
        assert!(bad(&["d", "a", "--item", "12x"]).contains("数字 id"));
        assert!(bad(&["d", "a", "--since", "-5", "--hn", "q"]).contains("非负整数"));
        assert!(bad(&["d", "a", "--since", "1", "--since", "2", "--hn", "q"]).contains("一次"));
        assert!(bad(&["d", "a", "--bogus", "--hn", "q"]).contains("未知选项"));
    }

    #[test]
    fn beijing_stamps() {
        assert_eq!(repo::beijing_stamp(0), "1970-01-01T08:00:00+08:00");
        // 2026-10-09T23:39:23Z, the created_at_i of an HN comment in 1009.
        assert_eq!(
            repo::beijing_stamp(1_791_589_163),
            "2026-10-10T07:39:23+08:00"
        );
        assert_eq!(short_time(Some("2026-10-10T07:39:23+08:00")), "10-10 07:39");
        assert_eq!(short_time(None), "-");
        assert_eq!((count(Some(0)), count(None)), ("0".into(), "-".into()));
    }

    #[test]
    fn default_since_is_a_week_back() {
        assert_eq!(default_since(1_791_589_163), 1_791_589_163 - 604_800);
    }

    #[test]
    fn truncates_by_chars() {
        assert_eq!(truncate_chars("héllo wörld", 5), "héllo…");
        assert_eq!(truncate_chars("短标题", 5), "短标题");
        assert_eq!(truncate_chars("一二三四五六", 5), "一二三四五…");
    }

    #[test]
    fn hn_hits_keep_nulls() {
        let v = json!({
            "nbHits": 7,
            "hits": [
                {"objectID": "50025713", "points": 327, "num_comments": 367,
                 "created_at_i": 1_791_589_163_i64, "title": "T", "url": "https://x.example/a"},
                {"objectID": "2", "points": null, "num_comments": null,
                 "created_at_i": 0, "title": null, "url": null},
                {"title": "no id, dropped"}
            ]
        });
        let (total, hits) = parse_hn_hits(&v).unwrap();
        assert_eq!(total, 7);
        assert_eq!(hits.len(), 2);
        assert_eq!(
            hits[0],
            HnHit {
                id: "50025713".into(),
                points: Some(327),
                comments: Some(367),
                time_bj: Some("2026-10-10T07:39:23+08:00".into()),
                title: "T".into(),
                url: Some("https://x.example/a".into()),
            }
        );
        assert_eq!((hits[1].points, hits[1].url.as_deref()), (None, None));
        assert!(parse_hn_hits(&json!({"message": "rate"})).is_err());
    }

    #[test]
    fn reddit_hits_accept_float_timestamps() {
        let v = json!({"data": {"children": [
            {"kind": "t3", "data": {"subreddit": "ClaudeAI", "score": 12, "num_comments": 3,
             "created_utc": 1_791_589_163.0, "title": "T", "permalink": "/r/ClaudeAI/comments/x/t/",
             "url": "https://x.example"}}
        ]}});
        let hits = parse_reddit_hits(&v).unwrap();
        assert_eq!(
            hits,
            vec![RedditHit {
                sub: "ClaudeAI".into(),
                score: Some(12),
                comments: Some(3),
                time_bj: Some("2026-10-10T07:39:23+08:00".into()),
                title: "T".into(),
                permalink: "https://www.reddit.com/r/ClaudeAI/comments/x/t/".into(),
                url: "https://x.example".into(),
            }]
        );
        assert!(parse_reddit_hits(&json!({})).is_err());
    }

    #[test]
    fn counts_nested_comments() {
        let item = json!({"children": [
            {"children": [{"children": []}, {"children": []}]},
            {"children": []}
        ]});
        assert_eq!(count_descendants(&item), 4);
        assert_eq!(count_descendants(&json!({"id": 1})), 0);
    }

    #[test]
    fn query_order_survives_serialisation() {
        let heat = Heat {
            fetched_bj: "t".into(),
            hn: Ordered(vec![("zeta".into(), vec![]), ("alpha".into(), vec![])]),
            reddit: Ordered(vec![]),
            items: vec!["1".into()],
            errors: vec![],
        };
        let text = serde_json::to_string(&heat).unwrap();
        assert_eq!(
            text,
            r#"{"fetched_bj":"t","hn":{"zeta":[],"alpha":[]},"reddit":{},"items":["1"],"errors":[]}"#
        );
    }

    // ---- review fixes (2026-10-13) ----

    #[test]
    fn decodes_the_whole_latin1_block() {
        let mut seen = HashSet::new();
        for (i, name) in LATIN1_ENTITIES.iter().enumerate() {
            assert!(seen.insert(*name), "duplicate {name}");
            let want = char::from_u32(0xa0 + i as u32).unwrap();
            assert_eq!(
                decode_entities(&format!("&{name};")),
                want.to_string(),
                "&{name};"
            );
        }
        assert_eq!(
            decode_entities("&Agrave;&eacute;&euml;&ntilde;&AElig;&szlig;&times;&divide;&yuml;"),
            "ÀéëñÆß×÷ÿ"
        );
        assert_eq!(decode_entities("&dagger;&Dagger;&equiv;"), "†‡≡");
        // Case matters: there is no `&EACUTE;`.
        assert_eq!(decode_entities("&EACUTE;"), "&EACUTE;");
    }

    #[test]
    fn decoded_carriage_returns_become_newlines() {
        assert_eq!(html_to_text("a&#13;&#10;b&#13;c&#x0D;d"), "a\nb\nc\nd\n");
    }

    #[test]
    fn script_inside_a_comment_opens_nothing() {
        assert_eq!(
            strip_raw_text_elements("a<!-- <script> -->b<script>c</script>d"),
            "a<!-- <script> -->bd"
        );
        // Before the fix the commented `<script>` paired with the real end
        // tag and ate ` -->b`.
        assert_eq!(html_to_text("a<!-- <script> -->b</script>c"), "abc\n");
        assert_eq!(
            html_to_text("x<!--[if IE]><script>s()</script><![endif]-->y"),
            "xy\n"
        );
        // An unterminated comment is not skipped (strip_comments keeps it too).
        assert_eq!(
            strip_raw_text_elements("<!-- a<script>b</script>"),
            "<!-- a"
        );
    }

    #[test]
    fn tags_are_stepped_over_whole() {
        // A `<script>` inside an attribute value is text, not an element.
        assert_eq!(
            html_to_text("<div title=\"<script>\">x</div>y</script>z"),
            "x\nyz\n"
        );
        assert_eq!(
            rewrite_links("<b data-x=\"<a href='h'>\">t</b>"),
            "<b data-x=\"<a href='h'>\">t</b>"
        );
    }

    #[test]
    fn unterminated_input_is_kept_as_text() {
        assert_eq!(html_to_text(&"<script>a".repeat(3)), "aaa\n");
        assert_eq!(html_to_text(&"<a".repeat(3)), "<a<a<a\n");
        assert_eq!(html_to_text(&"<a href=x>t".repeat(3)), "ttt\n");
        assert_eq!(
            html_to_text(&format!("<script>{}", "</script ".repeat(3))),
            "</script </script </script\n"
        );
        // A quote that never closes: each tag ends at its first `>`.
        assert_eq!(
            html_to_text(&format!("{} z=\"", "<i a='>'=".repeat(3))),
            "'='='= z=\"\n"
        );
    }

    #[test]
    fn markup_declaration_without_gt_is_text() {
        assert_eq!(replace_tags("a<!b"), "a<!b");
        assert_eq!(html_to_text("x<!-- y"), "x<!-- y\n");
        assert_eq!(html_to_text("x<?php"), "x<?php\n");
    }

    /// The pre-memo `tag_end`: quote-aware scan, else the first `>`.
    fn naive_tag_end(b: &[u8], from: usize) -> Option<usize> {
        let mut quote: Option<u8> = None;
        let mut prev = 0u8;
        for (i, &c) in b.iter().enumerate().skip(from) {
            match quote {
                Some(q) => {
                    if c == q {
                        quote = None;
                        prev = c;
                    }
                }
                None => match c {
                    b'>' => return Some(i),
                    b'"' | b'\'' if prev == b'=' => quote = Some(c),
                    c if c.is_ascii_whitespace() => {}
                    _ => prev = c,
                },
            }
        }
        b[from..].iter().position(|&c| c == b'>').map(|p| from + p)
    }

    /// The pre-memo `find_close`.
    fn naive_find_close(s: &str, from: usize, name: &str) -> Option<(usize, usize)> {
        let b = s.as_bytes();
        let mut i = from;
        while let Some(off) = s[i..].find("</") {
            let at = i + off;
            let after = at + 2 + name.len();
            if b.get(at + 2..after)
                .is_some_and(|n| n.eq_ignore_ascii_case(name.as_bytes()))
                && b.get(after)
                    .is_some_and(|&c| matches!(c, b'>' | b'/') || c.is_ascii_whitespace())
                && let Some(gt) = s[after..].find('>')
            {
                return Some((at, after + gt + 1));
            }
            i = at + 2;
        }
        None
    }

    /// The memos only skip work: on random markup soup every lookup, made in
    /// a pass's forward order, agrees with the plain scans.
    #[test]
    fn memoised_scans_agree_with_plain_scans() {
        const ALPHABET: &[u8] = b"<<>>/=\"' ai";
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64; // fixed: reproducible
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for _ in 0..3000 {
            let len = (next() % 60) as usize;
            let s: String = (0..len)
                .map(|_| ALPHABET[(next() % ALPHABET.len() as u64) as usize] as char)
                .collect();
            let b = s.as_bytes();
            let mut memo = Memo::default();
            let mut close_memo = Memo::default();
            for from in 0..=b.len() {
                assert_eq!(
                    tag_end(b, from, &mut memo),
                    naive_tag_end(b, from),
                    "tag_end({s:?}, {from})"
                );
                assert_eq!(
                    find_close(&s, from, "a", &mut close_memo),
                    naive_find_close(&s, from, "a"),
                    "find_close({s:?}, {from})"
                );
            }
        }
    }

    #[test]
    fn convert_file_strips_bom_and_replaces_invalid_utf8() {
        let dir = std::env::temp_dir().join(format!("barking-fetch-bom-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let (input, output) = (dir.join("in.html"), dir.join("out.txt"));
        // UTF-8 BOM, then Latin-1 `é` (0xE9), which is not UTF-8.
        fs::write(&input, b"\xEF\xBB\xBF<p>caf\xE9 &eacute;</p>").unwrap();
        convert_file(&input, &output, false).unwrap();
        assert_eq!(fs::read_to_string(&output).unwrap(), "caf\u{fffd} é\n");
        // --force with the same text leaves the file as it is.
        convert_file(&input, &output, true).unwrap();
        assert_eq!(fs::read_to_string(&output).unwrap(), "caf\u{fffd} é\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    fn temp_issue(tag: &str) -> Issue {
        let root = std::env::temp_dir().join(format!("barking-fetch-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let dir = root.join("docs").join("2610").join("1012");
        fs::create_dir_all(dir.join("sources")).unwrap();
        Issue {
            root,
            dir,
            yymm: "2610".into(),
            mmdd: "1012".into(),
        }
    }

    #[test]
    fn heat_refuses_existing_files_before_any_request() {
        let issue = temp_issue("refuse");
        fs::write(issue.sources().join("a-heat.json"), "{}").unwrap();
        fs::write(issue.sources().join("a-hn-5.json"), "{}").unwrap();
        let args = parse_heat_args(&s(&["x", "a", "--item", "5", "--item", "6"])).unwrap();
        let err = heat(&issue, &args).unwrap_err();
        assert!(err.contains("拒绝覆盖"), "{err}");
        assert!(err.contains("a-heat.json、a-hn-5.json"), "{err}");
        assert!(!err.contains("a-hn-6.json"), "{err}");
        fs::remove_dir_all(&issue.root).unwrap();
    }

    #[test]
    fn heat_record_with_only_errors_is_still_written() {
        let issue = temp_issue("errors");
        let heat = Heat {
            fetched_bj: "t".into(),
            hn: Ordered(vec![]),
            reddit: Ordered(vec![]),
            items: vec![],
            errors: vec!["HN 条目 0：HTTP 404 Not Found".into()],
        };
        write_heat(&issue.sources(), "a-heat.json", &heat).unwrap();
        let v: Value =
            serde_json::from_slice(&fs::read(issue.sources().join("a-heat.json")).unwrap())
                .unwrap();
        assert_eq!(v["hn"], json!({}));
        assert_eq!(v["errors"][0], "HN 条目 0：HTTP 404 Not Found");
        let err = write_heat(&issue.sources(), "a-heat.json", &heat).unwrap_err();
        assert!(err.contains("拒绝覆盖"), "{err}");
        fs::remove_dir_all(&issue.root).unwrap();
    }

    #[test]
    fn missing_counts_and_times_serialise_as_null() {
        let (_, hn) = parse_hn_hits(&json!({"hits": [{"objectID": "1", "title": "T"}]})).unwrap();
        let v = serde_json::to_value(&hn[0]).unwrap();
        for k in ["points", "comments", "time_bj", "url"] {
            assert_eq!(v[k], Value::Null, "hn {k}");
        }
        let reddit =
            parse_reddit_hits(&json!({"data": {"children": [{"data": {"title": "T"}}]}})).unwrap();
        let v = serde_json::to_value(&reddit[0]).unwrap();
        for k in ["score", "comments", "time_bj"] {
            assert_eq!(v[k], Value::Null, "reddit {k}");
        }
    }

    #[test]
    fn item_bodies_are_kept_when_they_look_like_an_object() {
        assert!(looks_like_json_object(b" \n{\"id\": 1"));
        assert!(!looks_like_json_object(b"<html>"));
        assert!(!looks_like_json_object(b"[1]"));
        assert!(!looks_like_json_object(b""));
    }
}
