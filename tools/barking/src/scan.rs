//! `barking scan-rows`: the purely mechanical rows that publishing an issue
//! adds to `README.md` (内容目录) and `daily-scan.md` (近 3 天速览已收, 已发过);
//! step 1 of docs/workflow/publish.md.
//!
//! Run it only after Develata says 已发布: a topic written into daily-scan.md
//! and pushed before publication is excluded from the next scheduled search.
//! The 已发过 row is editorial, so the tool only writes a placeholder
//! (`待填…`) that `publish-check` refuses to let through.
//!
//! Only table rows are touched; every other byte of both files stays. Rows
//! already present are never duplicated or rewritten, so reruns are no-ops.

use std::fs;
use std::ops::Range;
use std::path::Path;
use std::process::ExitCode;

use crate::card::{self, Kind, Spec};
use crate::offsite::write_if_changed;
use crate::repo::{Date, Issue};

const USAGE: &str = "用法：barking scan-rows <期次目录> [--dry-run]\n\
只在 Develata 说“已发布”后运行；--dry-run 只打印将要添加或删除的行，不写文件";

const README: &str = "README.md";
const DAILY_SCAN: &str = "daily-scan.md";
const H_README: &str = "## 内容目录";
const H_ROUNDUP: &str = "## 近 3 天速览已收";
const H_POSTED: &str = "## 已发过";
/// Rows of 近 3 天速览已收 dated this many days before the issue, or earlier,
/// are dropped (publishing 1008 drops 1005 and keeps 1006).
const WINDOW_DAYS: i64 = 3;
/// What the tool can say about an issue's 已发过 row: nothing editorial.
const PENDING: &str = "待填（选题与口径摘要，由 Claude 按最终发布版填写）";

type Lines = Vec<String>;

/// The rows one issue should have, rendered exactly like the existing ones.
pub(crate) struct Rows {
    /// README.md 内容目录.
    pub(crate) readme: String,
    /// daily-scan.md 近 3 天速览已收: one row per non-main `[[roundup]]`.
    pub(crate) roundup: Vec<String>,
    /// daily-scan.md 已发过 placeholder; `None` for a 速览专帖 (no main post).
    pub(crate) posted: Option<String>,
}

/// What one table edit did, for the report.
#[derive(Default, Debug, PartialEq, Eq)]
struct Change {
    added: Vec<String>,
    removed: Vec<String>,
    /// Wanted rows that were already in the table and left alone.
    present: Vec<String>,
    notes: Vec<String>,
}

impl Change {
    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}

pub fn run(args: &[String]) -> ExitCode {
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let dirs: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if dirs.len() != 1 || args.iter().any(|a| a.starts_with("--") && a != "--dry-run") {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    match scan_rows(Path::new(dirs[0]), dry_run) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn scan_rows(dir: &Path, dry_run: bool) -> Result<(), String> {
    let issue = Issue::resolve(dir)?;
    let date = issue.date()?;
    let spec = card::load_spec(&issue.images())?.ok_or_else(|| {
        format!(
            "{} 不存在：本期类型与速览条目都读自它",
            issue.images().join(card::SPEC_FILE).display()
        )
    })?;
    let rows = wanted_rows(&issue, &spec)?;

    println!(
        "== {}（{}；非主帖速览 {} 条）",
        issue.rel(),
        if spec.kind == Kind::Roundup {
            "速览专帖"
        } else {
            "主帖"
        },
        rows.roundup.len()
    );
    for (sub, what) in [("images", "配图说明"), ("sources", "来源入口")] {
        if !issue.dir.join(sub).join("README.md").is_file() {
            println!(
                "warn: {}/{sub}/README.md 不存在，README 行里的「{what}」链接会失效",
                issue.rel()
            );
        }
    }

    let read = |name: &str| {
        let path = issue.root.join(name);
        fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))
    };
    let (readme_text, readme_change) = apply_readme(&read(README)?, date, &rows)?;
    let (scan_text, roundup_change, posted_change) =
        apply_daily_scan(&read(DAILY_SCAN)?, date, &rows)?;

    report(README, H_README, &readme_change);
    report(DAILY_SCAN, H_ROUNDUP, &roundup_change);
    report(DAILY_SCAN, H_POSTED, &posted_change);

    let added = readme_change.added.len() + roundup_change.added.len() + posted_change.added.len();
    let removed = roundup_change.removed.len();
    if added + removed == 0 {
        println!("-- 无改动：本期的行都已在文件里");
        return Ok(());
    }
    if dry_run {
        println!("-- dry-run：将添加 {added} 行、删除 {removed} 行，未写文件");
        return Ok(());
    }
    if !readme_change.is_empty() {
        write_if_changed(&issue.root.join(README), &readme_text, "内容目录")?;
    }
    if !roundup_change.is_empty() || !posted_change.is_empty() {
        write_if_changed(&issue.root.join(DAILY_SCAN), &scan_text, "速览已收与已发过")?;
    }
    println!("-- 已写入：添加 {added} 行、删除 {removed} 行");
    if !posted_change.added.is_empty() {
        println!(
            "提醒：daily-scan.md 的已发过行还是占位（含“待填”），按最终发布版改写后再提交；publish-check 与 stage --batch 1 都会拒绝占位行"
        );
    }
    Ok(())
}

fn report(file: &str, section: &str, c: &Change) {
    println!("{file}「{}」", section.trim_start_matches("## "));
    match c.present.as_slice() {
        [] => {}
        [one] => println!("  已有：{}", brief(one)),
        many => println!("  已有 {} 行，未重复添加", many.len()),
    }
    for n in &c.notes {
        println!("  注：{n}");
    }
    for r in &c.removed {
        println!("  - {r}");
    }
    for r in &c.added {
        println!("  + {r}");
    }
}

/// The first cell of a row, enough to name it in the report.
fn brief(row: &str) -> String {
    match cells(row).first() {
        Some(first) if row.starts_with('|') => format!("| {first} | …"),
        _ => row.to_string(),
    }
}

/// The rows an issue should add, from its title and `cards.toml`.
pub(crate) fn wanted_rows(issue: &Issue, spec: &Spec) -> Result<Rows, String> {
    let date = issue.date()?;
    let title = issue.title()?;
    let mmdd = &issue.mmdd;
    cell_ok(&title, "标题")?;
    if title.contains(['[', ']']) {
        return Err(format!(
            "标题「{title}」含方括号，无法写成 README 链接，请手工添加"
        ));
    }
    let rel = issue.rel();
    let readme = format!(
        "| {} / {mmdd} | [{title}]({rel}/doc_{mmdd}_publish.txt) | [配图说明]({rel}/images/README.md) | [来源入口]({rel}/sources/README.md) |",
        date.y
    );
    let mut roundup = Vec::new();
    for e in spec.other_entries() {
        let (tag, text) = (e.tag.trim(), e.text.trim());
        if tag.is_empty() || text.is_empty() {
            return Err(format!("cards.toml 速览条目「{tag}」的 tag 或 text 为空"));
        }
        cell_ok(tag, "速览 tag")?;
        cell_ok(text, "速览 text")?;
        roundup.push(format!("| {mmdd} | {tag} | {text} |"));
    }
    let posted = (spec.kind == Kind::Main).then(|| format!("| {mmdd} | {title}：{PENDING} |"));
    Ok(Rows {
        readme,
        roundup,
        posted,
    })
}

/// A table cell cannot hold the column separator or a line break.
fn cell_ok(s: &str, what: &str) -> Result<(), String> {
    if s.contains(['|', '\n', '\r']) {
        return Err(format!(
            "{what}「{s}」含 | 或换行，无法写入表格，请手工添加"
        ));
    }
    Ok(())
}

fn apply_readme(text: &str, date: Date, rows: &Rows) -> Result<(String, Change), String> {
    let mut lines = split_lines(text);
    let mut change = Change::default();
    let body = table_rows(&lines, H_README)?;
    if let Some(i) = body
        .clone()
        .find(|&i| readme_date(first_cell(&lines[i])) == Some(date))
    {
        if lines[i].trim() != rows.readme {
            change.notes.push(format!(
                "现有行与生成的行不同，未改动；生成的行是：{}",
                rows.readme
            ));
        }
        change.present.push(lines[i].clone());
    } else {
        let at = insert_at(&lines, &body, date, readme_date);
        lines.insert(at, rows.readme.clone());
        change.added.push(rows.readme.clone());
    }
    Ok((lines.join("\n"), change))
}

/// Returns the new text plus what happened in 近 3 天速览已收 and in 已发过.
fn apply_daily_scan(
    text: &str,
    date: Date,
    rows: &Rows,
) -> Result<(String, Change, Change), String> {
    let mut lines = split_lines(text);
    let near = |s: &str| scan_date(s, date);

    // Prune first: the window is relative to this issue, and what is left
    // decides where the new rows go.
    let mut roundup = Change::default();
    let cutoff = date.plus_days(-WINDOW_DAYS);
    for i in table_rows(&lines, H_ROUNDUP)?.rev() {
        if near(first_cell(&lines[i])).is_some_and(|d| d <= cutoff) {
            roundup.removed.push(lines.remove(i));
        }
    }
    roundup.removed.reverse();

    let body = table_rows(&lines, H_ROUNDUP)?;
    // An issue older than the newest row by the window is outside it: rerun
    // for its README row, say, must not bring back what a newer issue pruned.
    let latest = body
        .clone()
        .filter_map(|i| near(first_cell(&lines[i])))
        .max();
    let stale = latest.is_some_and(|l| date <= l.plus_days(-WINDOW_DAYS));
    let mut fresh: Vec<String> = Vec::new();
    if stale {
        roundup
            .notes
            .push("本期早于表中最新期次三天以上，不加速览行".into());
    }
    for row in rows.roundup.iter().filter(|_| !stale) {
        // Same issue and same text: a rerun, or a row Claude already reworded.
        let want = cells(row);
        let have = body.clone().any(|i| {
            let c = cells(&lines[i]);
            c.len() >= 3 && c[0] == want[0] && c[2] == want[2]
        });
        if have {
            roundup.present.push(row.clone());
        } else if !fresh.contains(row) {
            fresh.push(row.clone());
        }
    }
    let at = insert_at(&lines, &body, date, near);
    lines.splice(at..at, fresh.iter().cloned());
    roundup.added = fresh;

    let mut posted = Change::default();
    match &rows.posted {
        None => posted.notes.push("速览专帖没有主帖，不加已发过行".into()),
        Some(row) => {
            let body = table_rows(&lines, H_POSTED)?;
            let mmdd = date.mmdd();
            if let Some(i) = body.clone().find(|&i| first_cell(&lines[i]) == mmdd) {
                posted.present.push(lines[i].clone());
            } else {
                // Append: this table is never pruned, so rows keyed by MMDD
                // alone cannot be placed by date once they are months old.
                lines.insert(body.end, row.clone());
                posted.added.push(row.clone());
            }
        }
    }
    Ok((lines.join("\n"), roundup, posted))
}

/// Lines without terminators; `join("\n")` restores the text exactly, final
/// newline included. Working copies may carry CRLF (`core.autocrlf`); the
/// repo's files are LF, so the text is folded to LF like `write_if_changed`
/// does when it compares.
fn split_lines(text: &str) -> Lines {
    text.replace("\r\n", "\n")
        .split('\n')
        .map(str::to_string)
        .collect()
}

/// Line indices of the body rows of the first table under `heading`: after
/// its header and `|---|` separator, up to the first line not starting with
/// `|`. The section ends at the next `## `; headings inside code fences do
/// not count.
fn table_rows(lines: &[String], heading: &str) -> Result<Range<usize>, String> {
    let (mut fenced, mut start, mut end) = (false, None, lines.len());
    for (i, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
        } else if fenced {
            continue;
        } else if start.is_none() {
            if line.trim_end() == heading {
                start = Some(i);
            }
        } else if line.starts_with("## ") {
            end = i;
            break;
        }
    }
    let start = start.ok_or_else(|| format!("找不到标题「{heading}」"))?;
    let is_row = |l: &String| l.trim_start().starts_with('|');
    let header = (start + 1..end)
        .find(|&i| is_row(&lines[i]))
        .ok_or_else(|| format!("「{heading}」下没有表格"))?;
    if header + 1 >= end || !is_separator(&lines[header + 1]) {
        return Err(format!("「{heading}」的表头后缺少 |---| 分隔行"));
    }
    let first = header + 2;
    let last = (first..end).find(|&i| !is_row(&lines[i])).unwrap_or(end);
    Ok(first..last)
}

fn is_separator(line: &str) -> bool {
    let c = cells(line);
    !c.is_empty()
        && c.iter().all(|x| {
            let t = x.trim_matches(':');
            !t.is_empty() && t.bytes().all(|b| b == b'-')
        })
}

/// Trimmed cells of a table row, outer pipes dropped.
fn cells(line: &str) -> Vec<&str> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    t.split('|').map(str::trim).collect()
}

fn first_cell(line: &str) -> &str {
    cells(line).first().copied().unwrap_or("")
}

/// Where a row dated `date` belongs: right after the last row not later than
/// it, or at the top of the table if every row is later. Rows whose date
/// cannot be read are ignored, never moved.
fn insert_at(
    lines: &[String],
    body: &Range<usize>,
    date: Date,
    row_date: impl Fn(&str) -> Option<Date>,
) -> usize {
    body.clone()
        .rev()
        .find(|&i| row_date(first_cell(&lines[i])).is_some_and(|d| d <= date))
        .map_or(body.start, |i| i + 1)
}

/// README.md names issues `2026 / 1009`.
fn readme_date(cell: &str) -> Option<Date> {
    let (y, mmdd) = cell.split_once('/')?;
    Date::from_mmdd(y.trim().parse().ok()?, mmdd.trim())
}

/// daily-scan.md names issues by MMDD alone. Read it as the date with that
/// MMDD closest to the issue being processed, so a window that spans New
/// Year still sorts and trims correctly (and a backfill is not mistaken for
/// last year).
fn scan_date(mmdd: &str, near: Date) -> Option<Date> {
    (near.y - 1..=near.y + 1)
        .filter_map(|y| Date::from_mmdd(y, mmdd))
        .min_by_key(|d| (d.days() - near.days()).abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const README_FIXTURE: &str = "\
# AI 吠点

## 内容目录

说明段落，含 `docs/<YYMM>/<MMDD>/`。

| 期次 | 正文 | 配图与上传顺序 | 来源与限制 |
|---|---|---|---|
| 2026 / 1007 | [七](docs/2610/1007/doc_1007_publish.txt) | [配图说明](docs/2610/1007/images/README.md) | [来源入口](docs/2610/1007/sources/README.md) |
| 2026 / 1009 | [九](docs/2610/1009/doc_1009_publish.txt) | [配图说明](docs/2610/1009/images/README.md) | [来源入口](docs/2610/1009/sources/README.md) |

结尾说明。

## 公开归档范围

| 不相干 | 表 |
|---|---|
| 2026 / 1008 | x |
";

    const SCAN_FIXTURE: &str = "\
# 检索

## 近 3 天速览已收

说明。

| 期次 | 标签 | 事实 |
|---|---|---|
| 1005 | 旧 | 太旧 |
| 1006 | 边界 | 留下 |
| 1007 | A | 七日条目 |
| 1008 | B | 八日条目一 |
| 1008 | C | 八日条目二 |
| 1009 | D | 九日条目 |

## 已发过

| 期次 | 选题 |
|---|---|
| 1007 | 七日选题 |
| 1008 | 八日选题 |
";

    fn issue(yymm: &str, mmdd: &str) -> Issue {
        Issue {
            root: PathBuf::from("/repo"),
            dir: PathBuf::from(format!("/repo/docs/{yymm}/{mmdd}")),
            yymm: yymm.into(),
            mmdd: mmdd.into(),
        }
    }

    fn rows(mmdd: &str, entries: &[(&str, &str)], posted: bool) -> Rows {
        Rows {
            readme: format!(
                "| 2026 / {mmdd} | [新](docs/2610/{mmdd}/doc_{mmdd}_publish.txt) | [配图说明](docs/2610/{mmdd}/images/README.md) | [来源入口](docs/2610/{mmdd}/sources/README.md) |"
            ),
            roundup: entries
                .iter()
                .map(|(t, x)| format!("| {mmdd} | {t} | {x} |"))
                .collect(),
            posted: posted.then(|| format!("| {mmdd} | 新：{PENDING} |")),
        }
    }

    fn day(mmdd: &str) -> Date {
        Date::from_mmdd(2026, mmdd).unwrap()
    }

    fn lines_of(text: &str) -> Vec<&str> {
        text.lines().collect()
    }

    #[test]
    fn readme_row_matches_existing_format() {
        // The live 1009 row, character for character.
        let spec: Spec =
            toml::from_str("[[roundup]]\ntag = \"Epoch\"\ntext = \"据 Epoch 评测\"\n").unwrap();
        let mut i = issue("2610", "1009");
        i.dir = std::env::temp_dir().join(format!("barking-scan-test-{}", std::process::id()));
        // `title()` reads the publish text, so exercise the row builder with
        // a real file.
        std::fs::create_dir_all(&i.dir).unwrap();
        std::fs::write(
            i.publish_txt(),
            "Claude编命案线索？谷歌买员工邮件？\n\n正文\n",
        )
        .unwrap();
        let r = wanted_rows(&i, &spec).unwrap();
        std::fs::remove_dir_all(&i.dir).ok();
        assert_eq!(
            r.readme,
            "| 2026 / 1009 | [Claude编命案线索？谷歌买员工邮件？](docs/2610/1009/doc_1009_publish.txt) | [配图说明](docs/2610/1009/images/README.md) | [来源入口](docs/2610/1009/sources/README.md) |"
        );
        assert_eq!(r.roundup, ["| 1009 | Epoch | 据 Epoch 评测 |"]);
        assert_eq!(
            r.posted.as_deref(),
            Some(
                "| 1009 | Claude编命案线索？谷歌买员工邮件？：待填（选题与口径摘要，由 Claude 按最终发布版填写） |"
            )
        );
    }

    #[test]
    fn readme_inserts_in_issue_order() {
        let r = rows("1008", &[], true);
        let (out, c) = apply_readme(README_FIXTURE, day("1008"), &r).unwrap();
        let want = README_FIXTURE.replacen(
            "| 2026 / 1009 |",
            &format!("{}\n| 2026 / 1009 |", r.readme),
            1,
        );
        assert_eq!(out, want);
        assert_eq!(c.added, [r.readme]);

        // After the last row, and before the first.
        let (out, _) = apply_readme(README_FIXTURE, day("1010"), &rows("1010", &[], true)).unwrap();
        let ls = lines_of(&out);
        let at = ls
            .iter()
            .position(|l| l.starts_with("| 2026 / 1010"))
            .unwrap();
        assert!(ls[at - 1].starts_with("| 2026 / 1009"));
        assert_eq!(ls[at + 1], "");
        let (out, _) = apply_readme(README_FIXTURE, day("0930"), &rows("0930", &[], true)).unwrap();
        let ls = lines_of(&out);
        let at = ls
            .iter()
            .position(|l| l.starts_with("| 2026 / 0930"))
            .unwrap();
        assert!(ls[at - 1].starts_with("|---"));
    }

    #[test]
    fn readme_existing_row_is_kept() {
        let (out, c) = apply_readme(README_FIXTURE, day("1009"), &rows("1009", &[], true)).unwrap();
        assert_eq!(out, README_FIXTURE);
        assert!(c.is_empty());
        assert_eq!(c.present.len(), 1);
        // The live row differs from the generated one (title): say so.
        assert_eq!(c.notes.len(), 1);
    }

    #[test]
    fn readme_only_edits_the_named_section() {
        // A row for 1008 sits in the later, unrelated table; it is not a hit.
        let (out, c) = apply_readme(README_FIXTURE, day("1008"), &rows("1008", &[], true)).unwrap();
        assert_eq!(c.added.len(), 1);
        assert!(out.ends_with("| 2026 / 1008 | x |\n"));
    }

    #[test]
    fn roundup_adds_after_same_day_and_prunes() {
        let r = rows("1008", &[("B", "八日条目一"), ("E", "八日新条目")], true);
        let (out, ru, _) = apply_daily_scan(SCAN_FIXTURE, day("1008"), &r).unwrap();
        // 1005 is 3 days back: dropped; 1006 stays.
        assert_eq!(ru.removed, ["| 1005 | 旧 | 太旧 |"]);
        assert_eq!(ru.added, ["| 1008 | E | 八日新条目 |"]);
        assert_eq!(ru.present.len(), 1);
        let want = SCAN_FIXTURE.replace("| 1005 | 旧 | 太旧 |\n", "").replace(
            "| 1008 | C | 八日条目二 |\n",
            "| 1008 | C | 八日条目二 |\n| 1008 | E | 八日新条目 |\n",
        );
        assert_eq!(out, want);
    }

    #[test]
    fn roundup_keeps_new_rows_in_card_order_and_before_later_days() {
        let r = rows("1008", &[("X", "甲"), ("Y", "乙"), ("X", "甲")], false);
        let fixture =
            SCAN_FIXTURE.replace("| 1008 | B | 八日条目一 |\n| 1008 | C | 八日条目二 |\n", "");
        let (out, ru, _) = apply_daily_scan(&fixture, day("1008"), &r).unwrap();
        assert_eq!(ru.added, ["| 1008 | X | 甲 |", "| 1008 | Y | 乙 |"]);
        let ls = lines_of(&out);
        let x = ls.iter().position(|l| l.contains("| X |")).unwrap();
        assert!(ls[x - 1].starts_with("| 1007 |"));
        assert_eq!(ls[x + 1], "| 1008 | Y | 乙 |");
        assert!(ls[x + 2].starts_with("| 1009 |"));
    }

    #[test]
    fn prune_window_crosses_month_boundary() {
        let text = "\
## 近 3 天速览已收

| 期次 | 标签 | 事实 |
|---|---|---|
| 1029 | a | 一 |
| 1030 | b | 二 |
| 1031 | c | 三 |

## 已发过

| 期次 | 选题 |
|---|---|
";
        let d = Date::from_mmdd(2026, "1101").unwrap();
        let r = rows("1101", &[("d", "四")], false);
        let (out, ru, _) = apply_daily_scan(text, d, &r).unwrap();
        // 1101 - 3 days = 1029: dropped; 1030 stays.
        assert_eq!(ru.removed, ["| 1029 | a | 一 |"]);
        assert!(out.contains("| 1030 | b | 二 |\n| 1031 | c | 三 |\n| 1101 | d | 四 |\n"));
    }

    #[test]
    fn prune_window_crosses_new_year() {
        let text = "\
## 近 3 天速览已收

| 期次 | 标签 | 事实 |
|---|---|---|
| 1229 | a | 一 |
| 1230 | b | 二 |
| 1231 | c | 三 |
| 0101 | d | 四 |

## 已发过

| 期次 | 选题 |
|---|---|
";
        let d = Date::new(2027, 1, 2).unwrap();
        let r = rows("0102", &[("e", "五")], false);
        let (out, ru, _) = apply_daily_scan(text, d, &r).unwrap();
        // 2027-01-02 - 3 days = 2026-12-30: 1229 and 1230 go, 1231 stays.
        assert_eq!(ru.removed, ["| 1229 | a | 一 |", "| 1230 | b | 二 |"]);
        assert!(out.contains("| 1231 | c | 三 |\n| 0101 | d | 四 |\n| 0102 | e | 五 |\n"));
    }

    #[test]
    fn backfill_does_not_prune_later_days() {
        // Processing 1007 while 1008 and 1009 are already there: only rows
        // older than 1004 would go, and 1007 slots in before 1008.
        let r = rows("1007", &[("A", "七日条目"), ("N", "七日新条目")], false);
        let (out, ru, _) = apply_daily_scan(SCAN_FIXTURE, day("1007"), &r).unwrap();
        assert_eq!(ru.removed, Vec::<String>::new());
        assert_eq!(ru.added, ["| 1007 | N | 七日新条目 |"]);
        let ls = lines_of(&out);
        let n = ls.iter().position(|l| l.contains("| N |")).unwrap();
        assert!(ls[n - 1].contains("| A |"));
        assert!(ls[n + 1].contains("| B |"));
    }

    #[test]
    fn posted_row_is_placeholder_for_main_only() {
        let main = rows("1009", &[], true);
        let (out, _, p) = apply_daily_scan(SCAN_FIXTURE, day("1009"), &main).unwrap();
        assert_eq!(p.added, main.posted.iter().cloned().collect::<Vec<_>>());
        assert!(out.ends_with(&format!("| 1008 | 八日选题 |\n| 1009 | 新：{PENDING} |\n")));
        assert!(p.added[0].contains("待填"));

        // 速览专帖 has no main post: no 已发过 row.
        let (out, _, p) =
            apply_daily_scan(SCAN_FIXTURE, day("1009"), &rows("1009", &[], false)).unwrap();
        assert!(p.is_empty());
        assert_eq!(p.notes.len(), 1);
        // Only the window moved: 1005 and 1006 are 4 and 3 days back.
        assert_eq!(
            out,
            SCAN_FIXTURE
                .replace("| 1005 | 旧 | 太旧 |\n", "")
                .replace("| 1006 | 边界 | 留下 |\n", "")
        );

        // An existing row, even a filled-in one, is left alone.
        let (out, _, p) =
            apply_daily_scan(SCAN_FIXTURE, day("1008"), &rows("1008", &[], true)).unwrap();
        assert!(p.is_empty() && p.present.len() == 1);
        assert_eq!(out, SCAN_FIXTURE.replace("| 1005 | 旧 | 太旧 |\n", ""));
    }

    #[test]
    fn posted_row_is_appended_even_when_old_rows_look_later() {
        // Processing 2027-04-05: `1001` and `0924` are last autumn's rows, but
        // by nearest-year they would read as next autumn. The row still goes
        // last.
        let text = "\
## 近 3 天速览已收

| 期次 | 标签 | 事实 |
|---|---|---|

## 已发过

| 期次 | 选题 |
|---|---|
| 0924 | 旧一 |
| 1001 | 旧二 |
| 0401 | 旧三 |

## 后续
";
        let d = Date::new(2027, 4, 5).unwrap();
        let r = rows("0405", &[], true);
        let (out, _, p) = apply_daily_scan(text, d, &r).unwrap();
        assert_eq!(p.added.len(), 1);
        assert!(out.contains(&format!(
            "| 0401 | 旧三 |\n| 0405 | 新：{PENDING} |\n\n## 后续"
        )));
    }

    #[test]
    fn older_issue_does_not_revive_pruned_rows() {
        // 1012 pruned the 1009 rows; rerunning 1009 (say, for its README
        // row) must not add them back.
        let (after_1012, ..) = apply_daily_scan(
            SCAN_FIXTURE,
            day("1012"),
            &rows("1012", &[("X", "十二")], true),
        )
        .unwrap();
        assert!(!after_1012.contains("| 1009 |"));
        let (again, ru, _) = apply_daily_scan(
            &after_1012,
            day("1009"),
            &rows("1009", &[("D", "九日条目")], true),
        )
        .unwrap();
        assert!(ru.added.is_empty() && !ru.notes.is_empty(), "{ru:?}");
        assert!(!again.contains("| 1009 | D |"));
        // Two days behind the newest row is still inside the window.
        let (_, ru, _) = apply_daily_scan(
            &after_1012,
            day("1010"),
            &rows("1010", &[("Y", "十")], true),
        )
        .unwrap();
        assert_eq!(ru.added.len(), 1);
    }

    #[test]
    fn roundup_kind_and_main_entries() {
        let spec: Spec = toml::from_str(
            "kind = \"roundup\"
[[roundup]]
tag = \"A\"
text = \"甲\"
",
        )
        .unwrap();
        let mut i = issue("2610", "1013");
        i.dir = std::env::temp_dir().join(format!("barking-scan-test-k-{}", std::process::id()));
        std::fs::create_dir_all(&i.dir).unwrap();
        std::fs::write(
            i.publish_txt(),
            "AI 速览

正文
",
        )
        .unwrap();
        let r = wanted_rows(&i, &spec).unwrap();
        assert!(r.posted.is_none());
        assert_eq!(r.roundup, ["| 1013 | A | 甲 |"]);
        let spec: Spec = toml::from_str(
            "[[tldr]]
tag = \"A\"
fact = \"f\"
[[roundup]]
tag = \"A\"
main = true
text = \"主\"
[[roundup]]
tag = \"B\"
text = \"乙\"
",
        )
        .unwrap();
        let r = wanted_rows(&i, &spec).unwrap();
        std::fs::remove_dir_all(&i.dir).ok();
        assert!(r.posted.is_some());
        assert_eq!(r.roundup, ["| 1013 | B | 乙 |"]);
    }

    #[test]
    fn second_run_changes_nothing() {
        let r = rows("1009", &[("D", "九日条目"), ("F", "九日新条目")], true);
        let (once, ..) = apply_daily_scan(SCAN_FIXTURE, day("1009"), &r).unwrap();
        let (twice, ru, p) = apply_daily_scan(&once, day("1009"), &r).unwrap();
        assert_eq!(once, twice);
        assert!(ru.is_empty() && p.is_empty());
        let (a, _) = apply_readme(README_FIXTURE, day("1008"), &r_readme("1008")).unwrap();
        let (b, c) = apply_readme(&a, day("1008"), &r_readme("1008")).unwrap();
        assert_eq!(a, b);
        assert!(c.is_empty());
    }

    fn r_readme(mmdd: &str) -> Rows {
        rows(mmdd, &[], true)
    }

    #[test]
    fn crlf_working_copy_is_folded_and_final_newline_kept() {
        let crlf = SCAN_FIXTURE.replace('\n', "\r\n");
        let r = rows("1009", &[("F", "九日新条目")], true);
        let (a, ..) = apply_daily_scan(&crlf, day("1009"), &r).unwrap();
        let (b, ..) = apply_daily_scan(SCAN_FIXTURE, day("1009"), &r).unwrap();
        assert_eq!(a, b);
        assert!(!a.contains('\r') && a.ends_with("|\n"));

        // No trailing newline: none is invented.
        let bare = SCAN_FIXTURE.trim_end_matches('\n');
        let (out, ..) = apply_daily_scan(bare, day("1009"), &r).unwrap();
        assert!(out.ends_with("待填（选题与口径摘要，由 Claude 按最终发布版填写） |"));
    }

    #[test]
    fn missing_section_or_table_is_an_error() {
        let e = apply_readme("# x\n", day("1009"), &rows("1009", &[], true)).unwrap_err();
        assert!(e.contains("找不到标题"), "{e}");
        let e = apply_readme(
            "## 内容目录\n\n没有表格\n\n## 下一节\n| a |\n|---|\n",
            day("1009"),
            &rows("1009", &[], true),
        )
        .unwrap_err();
        assert!(e.contains("没有表格"), "{e}");
        let e = apply_readme(
            "## 内容目录\n| a |\n| 不是分隔行 |\n",
            day("1009"),
            &rows("1009", &[], true),
        )
        .unwrap_err();
        assert!(e.contains("分隔行"), "{e}");
    }

    #[test]
    fn heading_inside_code_fence_is_not_a_section() {
        let text = "```\n## 内容目录\n```\n\n## 内容目录\n\n| 期次 |\n|---|\n| 2026 / 1007 |\n";
        let (out, c) = apply_readme(text, day("1008"), &rows("1008", &[], true)).unwrap();
        assert_eq!(c.added.len(), 1);
        assert!(out.ends_with(&format!(
            "| 2026 / 1007 |\n{}\n",
            rows("1008", &[], true).readme
        )));
    }

    #[test]
    fn unreadable_rows_are_ignored_not_removed() {
        let text = "\
## 近 3 天速览已收

| 期次 | 标签 | 事实 |
|---|---|---|
| 待定 | a | 一 |
| 1007 | b | 二 |
| 1009 | e | 五 |

## 已发过

| 期次 | 选题 |
|---|---|
";
        let r = rows("1010", &[("c", "三")], false);
        let (out, ru, _) = apply_daily_scan(text, day("1010"), &r).unwrap();
        assert_eq!(ru.removed, ["| 1007 | b | 二 |"]);
        assert!(out.contains("| 待定 | a | 一 |\n| 1009 | e | 五 |\n| 1010 | c | 三 |\n"));
    }

    #[test]
    fn cells_with_pipes_are_refused() {
        assert!(cell_ok("a|b", "标题").is_err());
        assert!(cell_ok("a\nb", "标题").is_err());
        assert!(cell_ok("正常", "标题").is_ok());
    }

    #[test]
    fn scan_date_picks_the_nearest_year() {
        let jan = Date::new(2027, 1, 2).unwrap();
        assert_eq!(scan_date("1231", jan), Date::new(2026, 12, 31));
        assert_eq!(scan_date("0101", jan), Date::new(2027, 1, 1));
        assert_eq!(scan_date("1301", jan), None);
        assert_eq!(scan_date("abcd", jan), None);
    }
}
