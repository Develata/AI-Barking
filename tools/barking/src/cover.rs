//! `barking cover-draw`: draw the style axes of an issue's cover, so that
//! consecutive covers differ by construction rather than by intention
//! (EDITORIAL.md 封面“每期抽签”).
//!
//! The deck `templates/cover-styles.toml` lists the axes and their values.
//! Each issue records the values it used on one `封面抽签：` line of
//! `images/README.md`. For every axis, the values used by the previous
//! `avoid_last` issues that carry such a line are left out of the draw;
//! `barking lint` applies the same rule, so a hand-picked value that repeats
//! too soon is an error.
//!
//! Entries are small owned maps: a deck has a few dozen strings and a history
//! a handful of issues, so cloning costs nothing next to the file reads.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::repo::Issue;

pub const DECK: &str = "templates/cover-styles.toml";
pub const LEDGER_PREFIX: &str = "封面抽签：";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deck {
    pub axis: Vec<Axis>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Axis {
    pub key: String,
    /// Values used by this many previous recorded issues are excluded.
    pub avoid_last: usize,
    pub values: Vec<Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value {
    pub name: String,
    pub hint: String,
}

/// One recorded or drawn cover: axis key → value name.
pub type Entry = BTreeMap<String, String>;

/// Per axis, the excluded values and the issue (MMDD) that last used each.
pub type Banned = BTreeMap<String, BTreeMap<String, String>>;

impl Deck {
    pub fn load(root: &Path) -> Result<Deck, String> {
        let path = root.join(DECK);
        let raw = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let deck: Deck = toml::from_str(&raw).map_err(|e| format!("{}: {e}", path.display()))?;
        deck.validate()
            .map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(deck)
    }

    /// Invariant relied on by `draw`: after excluding at most `avoid_last`
    /// values, every axis still offers two, so A and B can differ.
    fn validate(&self) -> Result<(), String> {
        if self.axis.is_empty() {
            return Err("没有 [[axis]]".into());
        }
        let mut keys = BTreeSet::new();
        for a in &self.axis {
            if !keys.insert(a.key.as_str()) {
                return Err(format!("轴「{}」重复", a.key));
            }
            let mut names = BTreeSet::new();
            for v in &a.values {
                if v.name.is_empty() || v.name.contains(['=', '；']) {
                    return Err(format!("轴「{}」的取值「{}」为空或含 = ；", a.key, v.name));
                }
                if !names.insert(v.name.as_str()) {
                    return Err(format!("轴「{}」的取值「{}」重复", a.key, v.name));
                }
            }
            if a.values.len() < a.avoid_last + 2 {
                return Err(format!(
                    "轴「{}」只有 {} 个取值，至少要比 avoid_last（{}）多 2 个",
                    a.key,
                    a.values.len(),
                    a.avoid_last
                ));
            }
        }
        Ok(())
    }

    /// Unknown or missing axes, and values the deck does not list.
    pub fn entry_errors(&self, e: &Entry) -> Vec<String> {
        let mut out = Vec::new();
        for k in e.keys() {
            if !self.axis.iter().any(|a| &a.key == k) {
                out.push(format!("未知的轴「{k}」（见 {DECK}）"));
            }
        }
        for a in &self.axis {
            match e.get(&a.key) {
                None => out.push(format!("缺少轴「{}」", a.key)),
                Some(v) if !a.values.iter().any(|x| &x.name == v) => {
                    out.push(format!("「{}」没有取值「{v}」（见 {DECK}）", a.key));
                }
                Some(_) => {}
            }
        }
        out
    }

    /// `history` is newest first. Only the first `avoid_last` entries count
    /// for each axis.
    pub fn banned(&self, history: &[(String, Entry)]) -> Banned {
        let mut out = Banned::new();
        for a in &self.axis {
            let slot = out.entry(a.key.clone()).or_default();
            for (mmdd, e) in history.iter().take(a.avoid_last) {
                if let Some(v) = e.get(&a.key) {
                    slot.entry(v.clone()).or_insert_with(|| mmdd.clone());
                }
            }
        }
        out
    }

    fn hint(&self, key: &str, value: &str) -> &str {
        self.axis
            .iter()
            .find(|a| a.key == key)
            .and_then(|a| a.values.iter().find(|v| v.name == value))
            .map_or("", |v| v.hint.as_str())
    }
}

/// `k=v；k=v` after the prefix. Strict: no empty fields, no repeated axis.
pub fn parse_entry(rest: &str) -> Result<Entry, String> {
    let mut e = Entry::new();
    for part in rest.trim().split('；') {
        let part = part.trim();
        let Some((k, v)) = part.split_once('=') else {
            return Err(format!("「{part}」不是 轴=取值"));
        };
        let (k, v) = (k.trim(), v.trim());
        if k.is_empty() || v.is_empty() {
            return Err(format!("「{part}」的轴或取值为空"));
        }
        if e.insert(k.to_string(), v.to_string()).is_some() {
            return Err(format!("轴「{k}」写了两次"));
        }
    }
    Ok(e)
}

/// Every `封面抽签：` line of a README (a leading list marker is allowed),
/// with its 1-based line number.
pub fn parse_ledger(md: &str) -> Vec<(usize, Result<Entry, String>)> {
    md.lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let l = l.trim();
            let l = l.strip_prefix("- ").unwrap_or(l);
            l.strip_prefix(LEDGER_PREFIX)
                .map(|rest| (i + 1, parse_entry(rest)))
        })
        .collect()
}

/// Recorded entries of the issues before `before` (a `YYMMMMDD` key), newest
/// first. Issues without a well-formed line (速览专帖, issues before the
/// draw existed) are skipped; their own lint reports malformed lines.
pub fn history(root: &Path, before: u32) -> Result<Vec<(String, Entry)>, String> {
    let mut out = Vec::new();
    for issue in Issue::all(root)?.into_iter().rev() {
        if issue.key() >= before {
            continue;
        }
        let Ok(md) = fs::read_to_string(issue.images().join("README.md")) else {
            continue;
        };
        if let Some(e) = parse_ledger(&md).into_iter().find_map(|(_, r)| r.ok()) {
            out.push((issue.mmdd.clone(), e));
        }
    }
    Ok(out)
}

/// Lint findings for an issue's README: exactly one well-formed line, every
/// value in the deck, none excluded by the previous issues.
pub fn ledger_errors(root: &Path, key: u32, md: &str) -> Vec<(Option<usize>, String)> {
    let deck = match Deck::load(root) {
        Ok(d) => d,
        Err(e) => return vec![(None, e)],
    };
    let lines = parse_ledger(md);
    let Some((n, first)) = lines.first() else {
        return vec![(
            None,
            format!("缺少「{LEDGER_PREFIX}」行：先跑 barking cover-draw，选定一组照抄进封面节"),
        )];
    };
    let mut out: Vec<(Option<usize>, String)> = lines
        .iter()
        .skip(1)
        .map(|(m, _)| (Some(*m), format!("「{LEDGER_PREFIX}」只能有一行")))
        .collect();
    let entry = match first {
        Ok(e) => e,
        Err(e) => {
            out.push((Some(*n), e.clone()));
            return out;
        }
    };
    out.extend(deck.entry_errors(entry).into_iter().map(|m| (Some(*n), m)));
    match history(root, key) {
        Ok(h) => {
            let banned = deck.banned(&h);
            for a in &deck.axis {
                let used = entry
                    .get(&a.key)
                    .and_then(|v| banned[&a.key].get(v).map(|m| (v, m)));
                if let Some((v, mmdd)) = used {
                    out.push((
                        Some(*n),
                        format!(
                            "{}={v} 与 {mmdd} 期重复：前 {} 期用过的取值本期不能再用",
                            a.key, a.avoid_last
                        ),
                    ));
                }
            }
        }
        Err(e) => out.push((None, e)),
    }
    out
}

/// splitmix64: tiny, seedable, good enough to pick from a dozen values.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform enough in `0..n` for n ≪ 2^64; `n` must be positive.
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Two entries drawn from the values `banned` leaves. B avoids A's value on
/// every axis that still has another one, so the two candidates are two
/// directions, not two takes of one.
fn draw(deck: &Deck, banned: &Banned, seed: u64) -> (Entry, Entry) {
    let mut rng = Rng(seed);
    let (mut a, mut b) = (Entry::new(), Entry::new());
    for axis in &deck.axis {
        let allowed: Vec<&str> = axis
            .values
            .iter()
            .map(|v| v.name.as_str())
            .filter(|v| !banned.get(&axis.key).is_some_and(|m| m.contains_key(*v)))
            .collect();
        // `Deck::validate` keeps at least two values per axis allowed.
        let pick_a = allowed[rng.below(allowed.len())];
        let rest: Vec<&str> = allowed.iter().copied().filter(|v| *v != pick_a).collect();
        let pick_b = rest[rng.below(rest.len())];
        a.insert(axis.key.clone(), pick_a.to_string());
        b.insert(axis.key.clone(), pick_b.to_string());
    }
    (a, b)
}

/// The README line for an entry, axes in deck order.
fn ledger_line(deck: &Deck, e: &Entry) -> String {
    let fields: Vec<String> = deck
        .axis
        .iter()
        .map(|a| format!("{}={}", a.key, e[&a.key]))
        .collect();
    format!("{LEDGER_PREFIX}{}", fields.join("；"))
}

pub fn run(args: &[String]) -> ExitCode {
    const USAGE: &str = "用法：barking cover-draw <期次目录> [--seed N]";
    let mut dir: Option<PathBuf> = None;
    let mut seed: Option<u64> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--seed" => match it.next().and_then(|s| s.parse().ok()) {
                Some(s) => seed = Some(s),
                None => {
                    eprintln!("{USAGE}");
                    return ExitCode::from(2);
                }
            },
            _ if dir.is_none() && !a.starts_with("--") => dir = Some(PathBuf::from(a)),
            _ => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(dir) = dir else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    match draw_report(&dir, seed) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn draw_report(dir: &Path, seed: Option<u64>) -> Result<String, String> {
    let issue = Issue::resolve(dir)?;
    let deck = Deck::load(&issue.root)?;
    let hist = history(&issue.root, issue.key())?;
    let banned = deck.banned(&hist);
    let seed = seed.unwrap_or_else(|| {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos() as u64);
        nanos ^ (u64::from(std::process::id()) << 32)
    });
    let (a, b) = draw(&deck, &banned, seed);

    let mut s = format!("{} 期封面抽签（种子 {seed}）\n本期排除：\n", issue.mmdd);
    for axis in &deck.axis {
        let used: Vec<String> = banned[&axis.key]
            .iter()
            .map(|(v, m)| format!("{v}（{m}）"))
            .collect();
        let used = if used.is_empty() {
            "无".to_string()
        } else {
            used.join("、")
        };
        s += &format!("  {}（前 {} 期）：{used}\n", axis.key, axis.avoid_last);
    }
    for (name, e) in [("A", &a), ("B", &b)] {
        s += &format!("\n{name}：\n");
        for axis in &deck.axis {
            let v = &e[&axis.key];
            s += &format!("  {}={v}  —— {}\n", axis.key, deck.hint(&axis.key, v));
        }
    }
    s += "\n两组各写一段提示词、各出一张候选；选中哪组，就把那组的一行整行照抄进 images/README.md 封面节：\n";
    s += &format!(
        "A  {}\nB  {}\n",
        ledger_line(&deck, &a),
        ledger_line(&deck, &b)
    );
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck() -> Deck {
        let raw = r#"
[[axis]]
key = "画风"
avoid_last = 2
values = [
  { name = "甲", hint = "" }, { name = "乙", hint = "" },
  { name = "丙", hint = "" }, { name = "丁", hint = "" },
]
[[axis]]
key = "底色"
avoid_last = 1
values = [{ name = "浅", hint = "" }, { name = "深", hint = "" }, { name = "亮", hint = "" }]
"#;
        let d: Deck = toml::from_str(raw).unwrap();
        d.validate().unwrap();
        d
    }

    fn entry(s: &str) -> Entry {
        parse_entry(s).unwrap()
    }

    #[test]
    fn repo_deck_is_valid() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        Deck::load(&root).unwrap();
    }

    #[test]
    fn entry_parsing_is_strict() {
        assert_eq!(entry("画风=甲；底色=浅")["底色"], "浅");
        assert!(parse_entry("画风=甲；").is_err());
        assert!(parse_entry("画风甲").is_err());
        assert!(parse_entry("画风=甲；画风=乙").is_err());
    }

    #[test]
    fn ledger_lines_allow_a_list_marker() {
        let md = "## 封面\n\n- 封面抽签：画风=甲；底色=浅\n其他\n封面抽签：坏\n";
        let got = parse_ledger(md);
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].0, 3);
        assert!(got[0].1.is_ok() && got[1].1.is_err());
    }

    #[test]
    fn entry_errors_name_missing_and_unknown() {
        let d = deck();
        assert!(d.entry_errors(&entry("画风=甲；底色=浅")).is_empty());
        assert_eq!(d.entry_errors(&entry("画风=戊；颜色=浅")).len(), 3);
    }

    #[test]
    fn banned_counts_only_the_last_avoid_last_issues() {
        let d = deck();
        let h = vec![
            ("1024".to_string(), entry("画风=甲；底色=浅")),
            ("1023".to_string(), entry("画风=乙；底色=深")),
            ("1022".to_string(), entry("画风=丙；底色=亮")),
        ];
        let b = d.banned(&h);
        assert_eq!(b["画风"].keys().collect::<Vec<_>>(), ["乙", "甲"]);
        assert_eq!(b["底色"].keys().collect::<Vec<_>>(), ["浅"]);
        assert_eq!(b["画风"]["甲"], "1024");
    }

    #[test]
    fn draws_avoid_banned_values_and_differ() {
        let d = deck();
        let h = vec![
            ("1024".to_string(), entry("画风=甲；底色=浅")),
            ("1023".to_string(), entry("画风=乙；底色=深")),
        ];
        let banned = d.banned(&h);
        for seed in 0..500 {
            let (a, b) = draw(&d, &banned, seed);
            for e in [&a, &b] {
                assert!(!["甲", "乙"].contains(&e["画风"].as_str()));
                assert_ne!(e["底色"], "浅");
            }
            assert_ne!(a["画风"], b["画风"]);
            assert_ne!(a["底色"], b["底色"]);
        }
    }

    #[test]
    fn same_seed_same_draw() {
        let d = deck();
        let banned = d.banned(&[]);
        assert_eq!(draw(&d, &banned, 7), draw(&d, &banned, 7));
    }

    #[test]
    fn deck_needs_two_values_beyond_avoid_last() {
        let raw = "[[axis]]\nkey = \"x\"\navoid_last = 1\nvalues = [{ name = \"a\", hint = \"\" }, { name = \"b\", hint = \"\" }]\n";
        let d: Deck = toml::from_str(raw).unwrap();
        assert!(d.validate().is_err());
    }

    #[test]
    fn ledger_errors_check_presence_deck_and_history() {
        let root = std::env::temp_dir().join(format!("barking-cover-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("templates")).unwrap();
        fs::write(root.join("EDITORIAL.md"), "").unwrap();
        let deck = "[[axis]]\nkey = \"画风\"\navoid_last = 1\nvalues = [{ name = \"甲\", hint = \"\" }, { name = \"乙\", hint = \"\" }, { name = \"丙\", hint = \"\" }]\n";
        fs::write(root.join(DECK), deck).unwrap();
        for (d, line) in [("1023", "封面抽签：画风=乙"), ("1024", "封面抽签：画风=甲")]
        {
            let images = root.join("docs/2610").join(d).join("images");
            fs::create_dir_all(&images).unwrap();
            fs::write(images.join("README.md"), format!("## 封面\n\n{line}\n")).unwrap();
        }
        let errs = |md: &str| ledger_errors(&root, 2610_1025, md);
        assert!(errs("## 封面\n\n封面抽签：画风=乙\n").is_empty());
        assert_eq!(errs("没有抽签行").len(), 1);
        let repeat = errs("封面抽签：画风=甲");
        assert_eq!(repeat.len(), 1);
        assert!(repeat[0].1.contains("1024"));
        assert_eq!(errs("封面抽签：画风=丁\n封面抽签：画风=乙").len(), 2);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn ledger_line_follows_deck_order() {
        let d = deck();
        assert_eq!(
            ledger_line(&d, &entry("底色=浅；画风=甲")),
            "封面抽签：画风=甲；底色=浅"
        );
    }
}
