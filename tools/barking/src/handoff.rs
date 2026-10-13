//! `barking handoff`: write the evidence-gathering brief of an issue
//! (`.handoff/<YYYY-MM-DD>-<MMDD>-evidence.md`) from
//! `templates/handoff-evidence.md`, and keep `.handoff/README.md` indexing
//! every brief.
//!
//! The briefs of two consecutive issues differed only in the issue number,
//! the baseline commit, the uncommitted-changes warning, the group list and a
//! few privacy notes, so those are the only inputs. Everything that needs an
//! editor's judgement (the facts already read, the claim table of each group)
//! is left as `（待填）` for Claude to fill before the brief goes out.
//!
//! The template file holds three parts: the brief itself, then one skeleton
//! per kind of group, each introduced by an `<!-- skeleton:NAME -->` line.
//! Placeholders are `{NAME}`; `{{` and `}}` are literal braces. Filling is a
//! single pass, so text substituted in (a note, a title) is never re-expanded.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::card::{self, Kind};
use crate::offsite::write_if_changed;
use crate::repo::{self, Date, Issue};

const TEMPLATE: &str = include_str!("../../../templates/handoff-evidence.md");
const HANDOFF_DIR: &str = ".handoff";
const INDEX_FILE: &str = "README.md";
/// A group whose topic starts with this gathers 速览 entries (`速览`,
/// `速览候选（…）`): no screenshots.
const ROUNDUP_TOPIC: &str = "速览";
/// Screenshot numbers by position of the group in the brief (A 01–12, …).
/// A 速览 group keeps its slot, so a shot group must sit in the first four.
const IMAGE_RANGES: [(u32, u32); 4] = [(1, 12), (13, 24), (25, 36), (37, 48)];
const ZH_COUNT: [&str; 11] = [
    "", "一", "两", "三", "四", "五", "六", "七", "八", "九", "十",
];
const ZH_ORDINAL: [&str; 5] = ["", "一", "二", "三", "四"];
/// Past this many uncommitted paths the warning only names the first few.
const MAX_LISTED_CHANGES: usize = 10;

/// Kept when `.handoff/README.md` is missing or has no list to anchor on.
const DEFAULT_INDEX_HEAD: [&str; 4] = [
    "# 历史派工与交接",
    "",
    "本目录随仓库保存，供回看制作过程。文件中的任务要求、路径、模型安排与“未完成”状态属于当时的工作上下文，不自动代表当前状态。",
    "",
];
const DEFAULT_INDEX_TAIL: [&str; 2] = [
    "",
    "这里的指令是历史记录，不应因公开归档而重新执行。正式正文、配图与核验资料见仓库首页。",
];

const USAGE: &str = "\
用法：
  barking handoff <期次目录> --groups \"A=主题,B=主题,C=速览\" [--note \"要点\"]... [--force] [--out 文件]
  barking handoff --index
    --groups  各组的组号与主题，组号为升序的单个大写字母；组之间用半角逗号分隔，主题里不能有逗号；
              主题以“速览”开头的组不截图、不计入上限，但仍占所在位置的编号区间；
              截图组最多 4 个，且须在前 4 位，截图编号区间按组序 01–12、13–24、25–36、37–48
    --note    本期特有的隐私/风控要点，可重复；每条成为“通用规则”里的一项
    --force   覆盖已存在的派工单
    --out     预览：写到指定文件，不建目录、不动 .handoff（含索引）
    --index   只重写 .handoff/README.md 的索引";

#[derive(Clone, Debug, PartialEq)]
struct Group {
    letter: char,
    topic: String,
}

impl Group {
    fn is_roundup(&self) -> bool {
        self.topic.starts_with(ROUNDUP_TOPIC)
    }
}

#[derive(Debug, Default)]
struct Opts {
    dir: Option<String>,
    groups: Vec<Group>,
    notes: Vec<String>,
    force: bool,
    out: Option<String>,
    index: bool,
}

/// Everything a brief varies on, already phrased; [`render`] only places it.
struct Ctx {
    yymm: String,
    mmdd: String,
    date: Date,
    groups: Vec<Group>,
    /// The checkout the brief's paths are relative to (a worktree, maybe).
    repo: String,
    branch: String,
    baseline: String,
    /// Uncommitted paths the executor must leave alone, already phrased.
    foreign: Vec<String>,
    dirs_note: String,
    prev_issues: String,
    unpublished: String,
    notes: Vec<String>,
}

pub fn run(args: &[String]) -> ExitCode {
    let opts = match parse_args(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let result = if opts.index {
        crate::repo_root()
            .map_err(|e| e.to_string())
            .and_then(|root| write_index(&root))
    } else {
        generate(&opts)
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args(args: &[String]) -> Result<Opts, String> {
    let mut o = Opts::default();
    let mut raw_groups: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--groups" => raw_groups = Some(take_value(&mut it, "--groups")?),
            "--note" => {
                let note = take_value(&mut it, "--note")?.replace(['\r', '\n'], " ");
                if note.trim().is_empty() {
                    return Err("--note 不能为空".into());
                }
                o.notes.push(note.trim().to_string());
            }
            "--out" => o.out = Some(take_value(&mut it, "--out")?),
            "--force" => o.force = true,
            "--index" => o.index = true,
            s if s.starts_with("--") => return Err(format!("未知选项 {s}")),
            _ => {
                if o.dir.replace(a.clone()).is_some() {
                    return Err("只能给一个期次目录".into());
                }
            }
        }
    }
    if o.index {
        if o.dir.is_some()
            || raw_groups.is_some()
            || !o.notes.is_empty()
            || o.force
            || o.out.is_some()
        {
            return Err("--index 不能与其他参数合用".into());
        }
        return Ok(o);
    }
    if o.dir.is_none() {
        return Err("缺少期次目录".into());
    }
    o.groups = parse_groups(&raw_groups.ok_or("缺少 --groups")?)?;
    Ok(o)
}

fn take_value<'a>(it: &mut impl Iterator<Item = &'a String>, flag: &str) -> Result<String, String> {
    it.next()
        .cloned()
        .ok_or_else(|| format!("{flag} 需要一个值"))
}

/// `A=主题,B=主题`. Group letters ascend because the evidence files are
/// prefixed with them (`a-`, `b-`, …) and the claim numbers follow.
fn parse_groups(raw: &str) -> Result<Vec<Group>, String> {
    // A full-width comma would silently glue two groups into one topic.
    if raw.contains('，') {
        return Err("组之间用半角逗号分隔（--groups 里不能有全角逗号“，”）".into());
    }
    let mut out: Vec<Group> = Vec::new();
    for part in raw.split(',') {
        let part = part.trim();
        let (letter, topic) = part
            .split_once('=')
            .ok_or_else(|| format!("组「{part}」应写成 A=主题"))?;
        let letter = letter.trim();
        let mut chars = letter.chars();
        let letter = match (chars.next(), chars.next()) {
            (Some(c), None) if c.is_ascii_uppercase() => c,
            _ => return Err(format!("组号「{letter}」须是单个大写字母")),
        };
        let topic = topic.trim();
        if topic.is_empty() {
            return Err(format!("组 {letter} 的主题为空"));
        }
        if out.last().is_some_and(|g| g.letter >= letter) {
            return Err(format!("组号须按字母升序且不重复（出现在 {letter} 处）"));
        }
        out.push(Group {
            letter,
            topic: topic.to_string(),
        });
    }
    if out.len() >= ZH_COUNT.len() {
        return Err(format!(
            "最多 {} 组，现有 {} 组",
            ZH_COUNT.len() - 1,
            out.len()
        ));
    }
    // Ranges go by position, so this also caps the shot groups at four.
    if let Some(i) = out.iter().rposition(|g| !g.is_roundup())
        && i >= IMAGE_RANGES.len()
    {
        return Err(format!(
            "截图组最多 {n} 个，且须排在前 {n} 位（截图编号区间按组序只划了 {n} 段，“{ROUNDUP_TOPIC}”组也占位）；组 {} 在第 {} 位",
            out[i].letter,
            i + 1,
            n = IMAGE_RANGES.len(),
        ));
    }
    if out.iter().all(Group::is_roundup) {
        return Err(format!(
            "至少要有一个需要截图的组（主题不以“{ROUNDUP_TOPIC}”开头）"
        ));
    }
    Ok(out)
}

fn generate(o: &Opts) -> Result<(), String> {
    let dir = o.dir.as_deref().ok_or("缺少期次目录")?;
    let issue = plan_issue(Path::new(dir))?;
    let date = issue.date()?;
    let file_name = format!("{}-{}-evidence.md", date.iso(), issue.mmdd);
    let handoff_dir = issue.root.join(HANDOFF_DIR);
    let target = match &o.out {
        Some(p) => PathBuf::from(p),
        None => handoff_dir.join(&file_name),
    };
    if target.exists() && !o.force {
        return Err(format!("{} 已存在（要覆盖请加 --force）", target.display()));
    }

    // Everything is read and rendered before the disk is touched, so a failure
    // leaves no half-made issue behind.
    let missing: Vec<&str> = ["images", "sources"]
        .into_iter()
        .filter(|d| !issue.dir.join(d).is_dir())
        .collect();
    let text = render(&context(o, &issue, &file_name, &missing)?)?;

    if o.out.is_none() {
        for d in &missing {
            fs::create_dir_all(issue.dir.join(d))
                .map_err(|e| format!("{}/{d}: {e}", issue.dir.display()))?;
            println!("新建空目录 {}/{d}/", issue.rel());
        }
        fs::create_dir_all(&handoff_dir).map_err(|e| format!("{}: {e}", handoff_dir.display()))?;
    }
    fs::write(&target, &text).map_err(|e| format!("{}: {e}", target.display()))?;
    println!(
        "写入 {}（{} 组，仍有 {} 处“待填”）",
        target.display(),
        o.groups.len(),
        text.matches("待填").count()
    );
    println!(
        "下一步：补全各组的“已知情况”“取证清单”“候选配图”后再派工；执行方交付 sources/evidence-<组>.md"
    );
    if o.out.is_none() {
        write_index(&issue.root)?;
    }
    Ok(())
}

/// What the repository says about the brief's issue right now: HEAD, other
/// uncommitted work, earlier issues and what daily-scan.md already lists.
fn context(o: &Opts, issue: &Issue, file_name: &str, missing: &[&str]) -> Result<Ctx, String> {
    let date = issue.date()?;
    let rel = issue.rel();
    let baseline = repo::git(&issue.root, &["rev-parse", "--short", "HEAD"])?;
    let branch = match repo::git(&issue.root, &["rev-parse", "--abbrev-ref", "HEAD"])? {
        // Detached HEAD: the commit is the only name there is.
        b if b == "HEAD" => baseline.clone(),
        b => b,
    };
    let mut status = parse_status_z(&repo::git(&issue.root, &["status", "--porcelain", "-z"])?);
    let parents: Vec<String> = status
        .iter()
        .filter(|(xy, p)| is_untracked_parent(xy, p, &rel))
        .map(|(_, p)| p.clone())
        .collect();
    if !parents.is_empty() {
        // git folds a new month into `docs/2611/`, which would hide another
        // uncommitted issue of that month behind this one; list it file by file.
        let mut args = vec!["status", "--porcelain", "-z", "--untracked-files=all", "--"];
        args.extend(parents.iter().map(String::as_str));
        let inside = parse_status_z(&repo::git(&issue.root, &args)?);
        status = unfold_parents(&status, &rel, &inside);
    }
    // The brief is this command's own output; so is the index, unless it holds
    // edits a rewrite would not produce.
    let index_rel = format!("{HANDOFF_DIR}/{INDEX_FILE}");
    let mut own_files = vec![format!("{HANDOFF_DIR}/{file_name}")];
    if !status.iter().any(|(_, p)| *p == index_rel) || index_is_own(&issue.root)? {
        own_files.push(index_rel);
    }
    let all = Issue::all(&issue.root)?;
    let with_logs: Vec<String> = all
        .iter()
        .rev()
        .filter(|i| i.key() < issue.key() && has_capture_log(i))
        .take(2)
        .map(Issue::rel)
        .collect();
    let scan_path = issue.root.join("daily-scan.md");
    let unpublished = match fs::read_to_string(&scan_path) {
        Ok(t) => {
            let scan = t.replace("\r\n", "\n");
            let priors = load_priors(&all, issue.key());
            unpublished_phrase(&unpublished_issues(&priors, &scan, date))
        }
        Err(e) => {
            eprintln!(
                "warn: {}: {e}；不判断“尚未写进 daily-scan”的期次",
                scan_path.display()
            );
            String::new()
        }
    };
    Ok(Ctx {
        yymm: issue.yymm.clone(),
        mmdd: issue.mmdd.clone(),
        date,
        groups: o.groups.clone(),
        repo: issue.root.display().to_string(),
        branch,
        baseline,
        foreign: foreign_changes(&status, &rel, &own_files),
        dirs_note: dirs_note(&rel, missing),
        prev_issues: prev_phrase(&with_logs),
        unpublished,
        notes: o.notes.clone(),
    })
}

/// The issue directory of a brief that may not exist yet: a fresh issue has no
/// directory until its first brief creates it.
fn plan_issue(dir: &Path) -> Result<Issue, String> {
    if dir.is_dir() {
        return Issue::resolve(dir);
    }
    let abs = std::path::absolute(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    if abs.exists() {
        return Err(format!("{}: 不是目录", dir.display()));
    }
    let root = card::repo_root(&abs)?;
    let rel = abs
        .strip_prefix(&root)
        .map_err(|_| format!("{} 不在仓库内", dir.display()))?;
    let parts: Vec<&str> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    match parts.as_slice() {
        ["docs", m, d] if crate::is_four_digits(m) && crate::is_four_digits(d) => Ok(Issue {
            yymm: m.to_string(),
            mmdd: d.to_string(),
            root,
            dir: abs,
        }),
        _ => Err(format!(
            "{} 不是 docs/<YYMM>/<MMDD> 期次目录",
            rel.display()
        )),
    }
}

fn has_capture_log(issue: &Issue) -> bool {
    fs::read_dir(issue.sources())
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .any(|e| e.file_name().to_string_lossy().starts_with("capture-log"))
}

// --- the brief ---------------------------------------------------------------

/// Pure: the same context always gives the same brief (LF line endings).
fn render(c: &Ctx) -> Result<String, String> {
    let (body, main_skeleton, roundup_skeleton) = split_template(TEMPLATE)?;

    let mut sections = Vec::new();
    let mut nth = 0;
    for (i, g) in c.groups.iter().enumerate() {
        let letter = g.letter.to_string();
        let mut vars = vec![("GROUP", letter.as_str()), ("TOPIC", g.topic.as_str())];
        if g.is_roundup() {
            sections.push(fill(&roundup_skeleton, &vars)?);
            continue;
        }
        nth += 1;
        let (lo, hi) = IMAGE_RANGES.get(i).copied().ok_or("组数超出截图编号区间")?;
        let range = format!("{lo:02}–{hi:02}");
        let date = c.date.iso();
        let ordinal = ZH_ORDINAL.get(nth).copied().ok_or("主帖条数超出范围")?;
        vars.extend([
            ("NTH", ordinal),
            ("DATE", date.as_str()),
            ("RANGE", range.as_str()),
        ]);
        sections.push(fill(&main_skeleton, &vars)?);
    }
    // A blank line before the rule: glued to text it would turn that line into
    // a setext heading.
    let sections = sections.join("\n\n---\n\n");
    let vars = body_vars(c, sections.trim_end())?;
    let vars: Vec<(&str, &str)> = vars.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let mut text = fill(&body, &vars)?;
    text.truncate(text.trim_end().len());
    text.push('\n');
    Ok(text)
}

fn title_groups(groups: &[Group]) -> String {
    groups
        .iter()
        .map(|g| format!("{} {}", g.letter, g.topic))
        .collect::<Vec<_>>()
        .join("｜")
}

fn body_vars(c: &Ctx, sections: &str) -> Result<Vec<(&'static str, String)>, String> {
    let letters_of = |roundup: bool| -> Vec<String> {
        c.groups
            .iter()
            .filter(|g| g.is_roundup() == roundup)
            .map(|g| g.letter.to_string())
            .collect()
    };
    let shot = letters_of(false);
    let roundup = letters_of(true);
    let all_letters: Vec<String> = c.groups.iter().map(|g| g.letter.to_string()).collect();
    let ranges = c
        .groups
        .iter()
        .enumerate()
        .filter(|(_, g)| !g.is_roundup())
        .map(|(i, g)| {
            let (lo, hi) = IMAGE_RANGES.get(i).copied().ok_or("组数超出截图编号区间")?;
            Ok(format!("{} 组 {lo:02}–{hi:02}", g.letter))
        })
        .collect::<Result<Vec<_>, &str>>()?
        .join("，");
    // Each prefix spelled out: `a-`…`d-` would claim a skipped letter's.
    let prefixes = c
        .groups
        .iter()
        .map(|g| format!("`{}-`", g.letter.to_ascii_lowercase()))
        .collect::<Vec<_>>()
        .join("、");
    let (tz, shift) = us_eastern(c.date);
    let notes = if c.notes.is_empty() {
        "- 本期无额外隐私/风控要点。".to_string()
    } else {
        c.notes
            .iter()
            .map(|n| format!("- {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let (roundup_shot, roundup_accept) = if roundup.is_empty() {
        (String::new(), String::new())
    } else {
        let l = roundup.join("/");
        (
            format!("{l} 组不需要截图，只存档与摘句。"),
            format!("{l} 组只需“扫描说法勘误”和各条的一句事实。"),
        )
    };
    let count = match ZH_COUNT.get(c.groups.len()) {
        Some(n) if !n.is_empty() => n,
        _ => return Err(format!("组数 {} 超出范围", c.groups.len())),
    };
    // The acceptance line points back at the list, so it goes when the list does.
    let keep = if c.foreign.is_empty() {
        String::new()
    } else {
        "，上面列出的既有未提交改动保持原样".into()
    };
    Ok(vec![
        ("MMDD", c.mmdd.clone()),
        ("YYMM", c.yymm.clone()),
        ("TITLE_GROUPS", title_groups(&c.groups)),
        ("GROUP_COUNT_ZH", count.to_string()),
        ("GROUP_LETTERS", all_letters.join("/")),
        ("REPO", c.repo.clone()),
        ("BRANCH", c.branch.clone()),
        ("BASELINE", c.baseline.clone()),
        ("UNTRACKED", untracked_phrase(&c.foreign)),
        ("KEEP_CHANGES", keep),
        ("DIRS_NOTE", c.dirs_note.clone()),
        ("PREV_ISSUES", c.prev_issues.clone()),
        ("PREFIXES", prefixes),
        ("UNPUBLISHED", c.unpublished.clone()),
        ("LOCAL_TZ", tz.to_string()),
        ("BJ_SHIFT", shift.to_string()),
        ("SHOT_GROUPS", shot.join("/")),
        ("IMAGE_RANGES", ranges),
        ("ROUNDUP_SHOT_NOTE", roundup_shot),
        ("ISSUE_NOTES", notes),
        ("ROUNDUP_ACCEPT_NOTE", roundup_accept),
        ("GROUP_SECTIONS", sections.to_string()),
    ])
}

/// `(brief, main-group skeleton, roundup-group skeleton)`, all LF.
fn split_template(tpl: &str) -> Result<(String, String, String), String> {
    const MAIN: &str = "<!-- skeleton:main -->";
    const ROUNDUP: &str = "<!-- skeleton:roundup -->";
    let tpl = tpl.replace("\r\n", "\n");
    let bad = || "模板须依次含 skeleton:main 与 skeleton:roundup 两个分隔行".to_string();
    let (body, rest) = tpl.split_once(MAIN).ok_or_else(bad)?;
    let (main, roundup) = rest.split_once(ROUNDUP).ok_or_else(bad)?;
    Ok((
        format!("{}\n", body.trim_end()),
        main.trim().to_string(),
        roundup.trim().to_string(),
    ))
}

/// `{NAME}` → value, `{{` / `}}` → literal braces, in one pass. An unknown name
/// is an error: a template typo must not ship as literal text.
fn fill(tpl: &str, vars: &[(&str, &str)]) -> Result<String, String> {
    let mut out = String::with_capacity(tpl.len());
    let mut rest = tpl;
    while let Some(i) = rest.find(['{', '}']) {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        if let Some(t) = tail.strip_prefix("{{") {
            out.push('{');
            rest = t;
        } else if let Some(t) = tail.strip_prefix("}}") {
            out.push('}');
            rest = t;
        } else if tail.starts_with('{') {
            let end = tail.find('}').ok_or("模板里有未闭合的 {")?;
            let name = &tail[1..end];
            let (_, value) = vars
                .iter()
                .find(|(k, _)| *k == name)
                .ok_or_else(|| format!("模板里有未知占位符 {{{name}}}"))?;
            out.push_str(value);
            rest = &tail[end + 1..];
        } else {
            return Err("模板里有单独的 }".into());
        }
    }
    out.push_str(rest);
    Ok(out)
}

// --- time zone ---------------------------------------------------------------

/// Day of week, 0 = Sunday (1970-01-01 was a Thursday).
fn weekday(d: Date) -> i64 {
    (d.days() + 4).rem_euclid(7)
}

fn nth_sunday(y: i32, m: u32, n: u32) -> Date {
    let first = Date { y, m, d: 1 };
    let offset = (7 - weekday(first)).rem_euclid(7) as u32;
    Date {
        y,
        m,
        d: 1 + offset + 7 * (n - 1),
    }
}

/// US Eastern time on `date`, which the briefs state as "本机时区": the label
/// and the hours to add to get Beijing time. DST runs from the second Sunday
/// of March to the first Sunday of November, so a hard-coded EDT would be
/// wrong from 2026-11-01. Whole days only; the 2 a.m. switch is ignored.
fn us_eastern(date: Date) -> (&'static str, i64) {
    let start = nth_sunday(date.y, 3, 2);
    let end = nth_sunday(date.y, 11, 1);
    if date >= start && date < end {
        ("EDT，UTC-4", 12)
    } else {
        ("EST，UTC-5", 13)
    }
}

// --- uncommitted changes -----------------------------------------------------

/// `git status --porcelain -z` as `(XY, path)`. Unlike [`repo::git_status`]
/// this runs without `--untracked-files=all`, so a wholly untracked directory
/// stays one `dir/` entry, which is what the warning wants to name.
fn parse_status_z(text: &str) -> Vec<(String, String)> {
    let mut fields = text.split('\0').filter(|s| !s.is_empty());
    let mut rows = Vec::new();
    while let Some(f) = fields.next() {
        let (Some(xy), Some(path)) = (f.get(..2), f.get(3..)) else {
            continue;
        };
        // A rename carries its original path in the next field.
        if xy.starts_with(['R', 'C']) {
            fields.next();
        }
        rows.push((xy.to_string(), path.to_string()));
    }
    rows
}

/// A wholly untracked directory that contains the issue directory, such as
/// `docs/2611/` for the month's first issue.
fn is_untracked_parent(xy: &str, path: &str, issue_rel: &str) -> bool {
    xy == "??" && path.ends_with('/') && format!("{issue_rel}/").starts_with(path)
}

/// `rows` with each untracked parent of the issue directory replaced by what
/// it holds besides the issue: `inside` lists the parents' files one by one
/// (`--untracked-files=all`), and each is folded to its shallowest directory
/// that is not itself a parent, as git folds any wholly untracked directory.
fn unfold_parents(
    rows: &[(String, String)],
    issue_rel: &str,
    inside: &[(String, String)],
) -> Vec<(String, String)> {
    let dir_prefix = format!("{issue_rel}/");
    let mut out: Vec<(String, String)> = Vec::new();
    for (xy, p) in rows {
        if !is_untracked_parent(xy, p, issue_rel) {
            out.push((xy.clone(), p.clone()));
            continue;
        }
        for (_, f) in inside {
            if !f.starts_with(p.as_str()) || f.starts_with(&dir_prefix) {
                continue;
            }
            // Shallowest `a/b/` under the parent that is not on the way to the
            // issue directory; the file itself if there is none.
            let entry = f
                .match_indices('/')
                .map(|(i, _)| &f[..=i])
                .find(|d| d.len() > p.len() && !dir_prefix.starts_with(d))
                .unwrap_or(f);
            let row = ("??".to_string(), entry.to_string());
            if !out.contains(&row) {
                out.push(row);
            }
        }
    }
    out
}

/// Everything uncommitted outside this issue's directory and the brief's own
/// files, each phrased for the "do not touch" sentence. An untracked parent
/// of the issue directory is only there because of the issue itself (its
/// other contents are listed by [`unfold_parents`]).
fn foreign_changes(
    rows: &[(String, String)],
    issue_rel: &str,
    own_files: &[String],
) -> Vec<String> {
    let dir_prefix = format!("{issue_rel}/");
    rows.iter()
        .filter(|(xy, p)| {
            !p.starts_with(&dir_prefix)
                && !own_files.contains(p)
                && !is_untracked_parent(xy, p, issue_rel)
        })
        .map(|(xy, p)| {
            if xy == "??" && p.ends_with('/') {
                format!("整个 `{p}` 目录")
            } else {
                format!("`{p}`")
            }
        })
        .collect()
}

/// The sentence for [`foreign_changes`].
fn untracked_phrase(items: &[String]) -> String {
    if items.is_empty() {
        return "工作区干净".into();
    }
    let shown = items
        .iter()
        .take(MAX_LISTED_CHANGES)
        .cloned()
        .collect::<Vec<_>>()
        .join("、");
    let more = if items.len() > MAX_LISTED_CHANGES {
        format!("等共 {} 处（以 `git status --short` 为准）", items.len())
    } else {
        String::new()
    };
    format!("工作区已知的未提交改动，**一律不要碰**：{shown}{more}")
}

/// Which of `images/` and `sources/` were (or, for a preview, would be) made.
fn dirs_note(rel: &str, missing: &[&str]) -> String {
    let path = |d: &str| format!("`{rel}/{d}/`");
    match missing {
        [] => format!("{}、{} 均已存在", path("images"), path("sources")),
        [a, b] => format!("已新建空目录 {}、{}", path(a), path(b)),
        [one] => {
            let other = if *one == "images" {
                "sources"
            } else {
                "images"
            };
            format!("已新建空目录 {}；{} 已存在", path(one), path(other))
        }
        _ => String::new(),
    }
}

/// The two most recent earlier issues that have capture logs, newest first.
fn prev_phrase(rels: &[String]) -> String {
    if rels.is_empty() {
        return "本仓库没有更早的期次".into();
    }
    let dirs = rels
        .iter()
        .map(|r| format!("`{r}/sources/`"))
        .collect::<Vec<_>>()
        .join(" 与 ");
    format!("{dirs} 里的 `capture-log-*.md`")
}

// --- issues not yet in daily-scan.md -----------------------------------------

/// An earlier issue as far as the brief needs it.
struct Prior {
    mmdd: String,
    date: Date,
    kind: Kind,
    title: Option<String>,
    /// 速览 entries that are not main-post titles, as shown on the card.
    others: Vec<String>,
}

/// Issues before `key` that have a publish text. A broken `cards.toml` or
/// publish text only costs that issue its details; the brief is still made.
fn load_priors(all: &[Issue], key: u32) -> Vec<Prior> {
    let mut out = Vec::new();
    for i in all.iter().filter(|i| i.key() < key) {
        if !i.publish_txt().is_file() {
            continue;
        }
        let Ok(date) = i.date() else { continue };
        let title = i
            .title()
            .map_err(|e| eprintln!("warn: {} 的标题读不到：{e}", i.rel()))
            .ok();
        let spec = card::load_spec(&i.images())
            .map_err(|e| eprintln!("warn: {} 的 cards.toml 读不到：{e}", i.rel()))
            .ok()
            .flatten();
        out.push(Prior {
            mmdd: i.mmdd.clone(),
            date,
            kind: spec.as_ref().map_or(Kind::Main, |s| s.kind),
            title,
            others: spec
                .iter()
                .flat_map(|s| s.other_entries().map(|r| r.text.clone()))
                .collect(),
        });
    }
    out
}

/// Lines between `heading` and the next `## ` heading.
fn section<'a>(text: &'a str, heading: &str) -> Vec<&'a str> {
    text.lines()
        .skip_while(|l| l.trim_end() != heading)
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
        .collect()
}

/// Whether the section has a table row whose first cell is `mmdd`. Only the
/// first cell is read: the tables are edited by `barking scan-rows`, not here.
fn has_row(text: &str, heading: &str, mmdd: &str) -> bool {
    section(text, heading).iter().any(|l| {
        l.trim_start()
            .strip_prefix('|')
            .and_then(|r| r.split('|').next())
            .is_some_and(|cell| cell.trim() == mmdd)
    })
}

/// Earlier issues the scan prompt cannot know about yet. A main post is
/// recorded in 已发过. A 速览专帖 has no such row, only its entries in 近 3 天
/// 速览已收, which are pruned after three days; so it counts only while it is
/// still inside that window (the third day included: pruning happens when a
/// later issue is published, not on the calendar), otherwise nothing can show
/// it was recorded.
fn unpublished_issues<'a>(priors: &'a [Prior], scan: &str, current: Date) -> Vec<&'a Prior> {
    priors
        .iter()
        .filter(|p| match p.kind {
            Kind::Main => !has_row(scan, "## 已发过", &p.mmdd),
            Kind::Roundup => {
                p.date.days() >= current.days() - 3 && !has_row(scan, "## 近 3 天速览已收", &p.mmdd)
            }
        })
        .collect()
}

/// Empty when nothing is missing; otherwise it starts with `，` because it
/// continues the sentence about already-posted content.
fn unpublished_phrase(list: &[&Prior]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mmdds = list
        .iter()
        .map(|p| p.mmdd.as_str())
        .collect::<Vec<_>>()
        .join("、");
    let described: Vec<String> = list
        .iter()
        .map(|p| {
            let mut parts = Vec::new();
            if p.kind == Kind::Main {
                parts.push(format!(
                    "主帖 = {}",
                    p.title.as_deref().unwrap_or("（正文缺失）")
                ));
            }
            if !p.others.is_empty() {
                parts.push(format!("速览 = {}", p.others.join("、")));
            }
            let parts = parts.join("；");
            if list.len() > 1 {
                format!("{} 期：{parts}", p.mmdd)
            } else {
                parts
            }
        })
        .collect();
    format!(
        "，**另加尚未写进 daily-scan 的 {mmdds} 期**：{}",
        described.join("。")
    )
}

// --- .handoff/README.md ------------------------------------------------------

/// Rewrite the index: keep what the file says before its first and after its
/// last list item, and list every brief newest first.
fn write_index(root: &Path) -> Result<(), String> {
    let path = root.join(HANDOFF_DIR).join(INDEX_FILE);
    let old = read_lf(&path).unwrap_or_default();
    write_if_changed(
        &path,
        &index_text(&old, &index_entries(root)?),
        "派工单索引",
    )
}

/// `(file name, title)` of every brief in `.handoff/`, newest first.
fn index_entries(root: &Path) -> Result<Vec<(String, String)>, String> {
    let dir = root.join(HANDOFF_DIR);
    let mut entries = Vec::new();
    for e in fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
        let name = e.file_name().to_string_lossy().into_owned();
        if name == INDEX_FILE || !name.ends_with(".md") || !e.path().is_file() {
            continue;
        }
        let raw = fs::read_to_string(e.path()).map_err(|e2| format!("{name}: {e2}"))?;
        entries.push((name.clone(), index_title(&raw, &name)));
    }
    entries.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(entries)
}

fn read_lf(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path).map(|t| t.replace("\r\n", "\n"))
}

/// Whether the uncommitted `.handoff/README.md` is exactly what a rewrite of
/// the committed one gives for the briefs now on disk: then its changes are
/// this command's own (an uncommitted earlier brief's entry, say). The brief
/// about to be written is not counted, since a run that has not written it
/// yet cannot have listed it. A hand edit to the prose around the list would
/// survive [`index_text`] untouched, so the committed file, not the working
/// one, is the baseline.
fn index_is_own(root: &Path) -> Result<bool, String> {
    let Ok(working) = read_lf(&root.join(HANDOFF_DIR).join(INDEX_FILE)) else {
        return Ok(false);
    };
    let committed = repo::git(root, &["show", &format!("HEAD:{HANDOFF_DIR}/{INDEX_FILE}")])
        .map(|t| t.replace("\r\n", "\n"))
        .unwrap_or_default();
    Ok(working == index_text(&committed, &index_entries(root)?))
}

/// First line of the brief without its `# `; the file name if it has none.
fn index_title(raw: &str, name: &str) -> String {
    let first = raw
        .trim_start_matches('\u{feff}')
        .lines()
        .next()
        .unwrap_or("");
    match first.trim().strip_prefix("# ") {
        Some(t) if !t.trim().is_empty() => t.trim().to_string(),
        _ => name.trim_end_matches(".md").to_string(),
    }
}

/// `entries` are `(file name, title)`, already in the order to list.
fn index_text(old: &str, entries: &[(String, String)]) -> String {
    let lines: Vec<&str> = old.lines().collect();
    let is_item = |l: &&str| l.starts_with("- [");
    let (mut out, tail): (Vec<String>, Vec<String>) = match (
        lines.iter().position(is_item),
        lines.iter().rposition(is_item),
    ) {
        (Some(first), Some(last)) => (
            lines[..first].iter().map(|s| s.to_string()).collect(),
            lines[last + 1..].iter().map(|s| s.to_string()).collect(),
        ),
        _ => (
            DEFAULT_INDEX_HEAD.iter().map(|s| s.to_string()).collect(),
            DEFAULT_INDEX_TAIL.iter().map(|s| s.to_string()).collect(),
        ),
    };
    for (name, title) in entries {
        let title = title.replace('[', "\\[").replace(']', "\\]");
        out.push(format!("- [{title}]({name})"));
    }
    out.extend(tail);
    let mut text = out.join("\n");
    text.push('\n');
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(letter: char, topic: &str) -> Group {
        Group {
            letter,
            topic: topic.into(),
        }
    }

    fn ctx(groups: Vec<Group>) -> Ctx {
        Ctx {
            yymm: "2610".into(),
            mmdd: "1009".into(),
            date: Date::new(2026, 10, 9).unwrap(),
            groups,
            repo: r"E:\gitclone\AI-Barking".into(),
            branch: "main".into(),
            baseline: "abc1234".into(),
            foreign: Vec::new(),
            dirs_note: dirs_note("docs/2610/1009", &["images", "sources"]),
            prev_issues: prev_phrase(&["docs/2610/1008".into(), "docs/2610/1007".into()]),
            unpublished: String::new(),
            notes: Vec::new(),
        }
    }

    fn four_groups() -> Vec<Group> {
        vec![
            group('A', "Anthropic 越界报告"),
            group('B', "Google 拟购 Spirit 数据"),
            group('C', "紫外线全天图"),
            group('D', "速览"),
        ]
    }

    /// The first `{` followed by a capital letter, apart from the one pair
    /// the brief quotes verbatim from `templates/codex-evidence.md`.
    fn leftover_placeholder(text: &str) -> Option<String> {
        let text = text.replacen("路径里的 `{YYMM}/{MMDD}` 即", "", 1);
        text.match_indices('{')
            .find(|(at, _)| {
                text[at + 1..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_uppercase())
            })
            .map(|(at, _)| text[at..].chars().take(20).collect())
    }

    #[test]
    fn leftover_placeholder_scans_every_brace() {
        assert_eq!(leftover_placeholder("a {x} b {Y}"), Some("{Y}".into()));
        assert_eq!(leftover_placeholder("{姓名} {} {"), None);
        let quoted = "路径里的 `{YYMM}/{MMDD}` 即 `2610/1009`";
        assert_eq!(leftover_placeholder(quoted), None);
        // Only that one sentence is exempt.
        assert!(leftover_placeholder(&format!("{quoted} `{{YYMM}}/x`")).is_some());
    }

    #[test]
    fn parses_groups() {
        let g = parse_groups("A=OpenAI 数学撤稿, B=Anthropic 使用政策 ,C=速览").unwrap();
        assert_eq!(
            g,
            vec![
                group('A', "OpenAI 数学撤稿"),
                group('B', "Anthropic 使用政策"),
                group('C', "速览")
            ]
        );
        assert!(g[2].is_roundup());
        for bad in [
            "",
            "A",
            "a=x",
            "AB=x",
            "A=",
            "B=x,A=y",
            "A=x,A=y",
            "A=速览",
            "A=速览候选（不截图）",
            "A=1,B=2,C=3,D=4,E=5",
            // Four shot groups, but a 速览 group pushes the last out of range.
            "A=1,B=速览,C=3,D=4,E=5",
        ] {
            assert!(parse_groups(bad).is_err(), "{bad:?} should be rejected");
        }
        assert!(parse_groups("A=1,B=2,C=3,D=4").is_ok());
        // 速览 groups do not count towards the four: the 1007 layout.
        let g = parse_groups("A=1,B=2,C=3,D=速览,E=速览候选（二）").unwrap();
        assert_eq!(g.iter().filter(|g| g.is_roundup()).count(), 2);
        assert!(parse_groups("A=1,B=2,C=3,D=4,E=速览").is_ok());
        let e = parse_groups("A=1,B=2,C=3,D=4,E=5").unwrap_err();
        assert!(e.contains("截图组最多 4 个"), "{e}");
    }

    #[test]
    fn rejects_full_width_comma() {
        let e = parse_groups("A=甲，B=乙").unwrap_err();
        assert!(e.contains("半角逗号"), "{e}");
        assert!(parse_groups("A=甲,B=乙（丙，丁）").is_err());
        // A usage error, so `run` exits 2.
        let args: Vec<String> = ["docs/2610/1009", "--groups", "A=甲，B=乙"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(parse_args(&args).unwrap_err().contains("半角逗号"));
    }

    #[test]
    fn parses_args() {
        let a = |s: &[&str]| parse_args(&s.iter().map(|x| x.to_string()).collect::<Vec<_>>());
        let o = a(&[
            "docs/2610/1009",
            "--groups",
            "A=甲,B=速览",
            "--note",
            "隐私：x",
            "--note",
            "风控：y",
            "--force",
        ])
        .unwrap();
        assert_eq!(o.dir.as_deref(), Some("docs/2610/1009"));
        assert_eq!(o.notes, ["隐私：x", "风控：y"]);
        assert!(o.force && !o.index && o.out.is_none());
        assert!(a(&["--index"]).unwrap().index);
        for bad in [
            &["--index", "docs/2610/1009"][..],
            &["--index", "--force"],
            &["--groups", "A=甲"],
            &["docs/2610/1009"],
            &["docs/2610/1009", "x", "--groups", "A=甲"],
            &["docs/2610/1009", "--groups"],
            &["docs/2610/1009", "--groups", "A=甲", "--bogus"],
            &["docs/2610/1009", "--groups", "A=甲", "--note", "  "],
        ] {
            assert!(a(bad).is_err(), "{bad:?} should be a usage error");
        }
    }

    #[test]
    fn fill_substitutes_in_one_pass() {
        let vars = [("A", "{B}"), ("B", "x")];
        assert_eq!(fill("<{A}|{B}>", &vars).unwrap(), "<{B}|x>");
        assert_eq!(fill("{{A}} {A}", &vars).unwrap(), "{A} {B}");
        assert!(fill("{NOPE}", &vars).unwrap_err().contains("NOPE"));
        assert!(fill("{A", &vars).is_err());
        assert!(fill("a } b", &vars).is_err());
    }

    #[test]
    fn brief_has_no_placeholder_left() {
        let mut c = ctx(four_groups());
        c.notes = vec!["隐私：{姓名} 照原文存".into()];
        let text = render(&c).unwrap();
        assert_eq!(leftover_placeholder(&text), None);
        // The literal pair from codex-evidence.md survives, the second pair is
        // the issue's own path.
        assert!(text.contains("路径里的 `{YYMM}/{MMDD}` 即 `2610/1009`"));
        // Substituted text is not expanded again.
        assert!(text.contains("- 隐私：{姓名} 照原文存"));
        assert!(!text.contains('\r'));
        assert!(text.ends_with("）\n"), "{:?}", &text[text.len() - 20..]);
    }

    #[test]
    fn template_uses_every_placeholder() {
        let (body, main, roundup) = split_template(TEMPLATE).unwrap();
        let vars = body_vars(&ctx(four_groups()), "S").unwrap();
        for (name, _) in &vars {
            assert!(body.contains(&format!("{{{name}}}")), "{name} unused");
        }
        let probe = [
            ("GROUP", "G"),
            ("TOPIC", "T"),
            ("NTH", "N"),
            ("DATE", "D"),
            ("RANGE", "R"),
        ];
        for (name, _) in probe {
            assert!(main.contains(&format!("{{{name}}}")), "{name} unused");
        }
        assert!(roundup.contains("{GROUP}") && roundup.contains("{TOPIC}"));
        assert!(fill(&main, &probe).is_ok() && fill(&roundup, &probe).is_ok());
    }

    #[test]
    fn assigns_image_ranges_by_group_position() {
        let text = render(&ctx(four_groups())).unwrap();
        assert!(text.contains("（仅 A/B/C 组）"));
        assert!(text.contains("（A 组 01–12，B 组 13–24，C 组 25–36）"));
        assert!(text.contains("候选配图（A 组 01–12）"));
        assert!(text.contains("候选配图（C 组 25–36）"));
        assert!(text.contains("D 组不需要截图，只存档与摘句。"));
        assert!(text.contains("D 组只需“扫描说法勘误”和各条的一句事实。"));
        assert!(text.contains("本文件分四组（A/B/C/D）"));
        assert!(text.contains("文件名以组号开头（`a-`、`b-`、`c-`、`d-`）"));
        assert!(text.contains("主帖第三条。"));
        assert!(text.contains("（待填）\n\n---\n\n# B 组："));
        assert!(text.contains("回报\n\n最终消息写："));
        assert!(text.contains("| D1 | （待填） | （待填） | （待填） |"));

        // A 速览 group in the middle keeps its slot: C is the third group.
        let mixed = render(&ctx(vec![
            group('A', "甲"),
            group('B', "速览"),
            group('C', "丙"),
        ]))
        .unwrap();
        assert!(mixed.contains("（A 组 01–12，C 组 25–36）"));
        assert!(mixed.contains("主帖第二条。"));
        assert!(mixed.contains("本文件分三组（A/B/C）"));

        // Skipped letters: prefixes are listed one by one; a 速览候选 group is
        // a 速览 group; five groups are counted in words.
        let skipped = render(&ctx(vec![
            group('A', "甲"),
            group('B', "乙"),
            group('D', "速览候选（不截图）"),
        ]))
        .unwrap();
        assert!(skipped.contains("（`a-`、`b-`、`d-`）"));
        assert!(skipped.contains("（A 组 01–12，B 组 13–24）"));
        assert!(skipped.contains("D 组不需要截图"));
        assert!(skipped.contains("# D 组：速览候选（不截图）\n\n速览聚合图"));
        let five = parse_groups("A=1,B=2,C=3,D=速览,E=速览").unwrap();
        let five = render(&ctx(five)).unwrap();
        assert!(five.contains("本文件分五组（A/B/C/D/E）"));
        assert!(five.contains("D/E 组不需要截图"));

        // Without a 速览 group the two roundup sentences disappear whole.
        let plain = render(&ctx(vec![group('A', "甲"), group('B', "乙")])).unwrap();
        assert!(plain.contains("深色底页面也可以截，但在日志里注明。\n"));
        assert!(plain.contains("三节。\n"));
        assert!(plain.contains("本文件分两组（A/B）"));
        let single = render(&ctx(vec![group('A', "甲")])).unwrap();
        assert!(single.contains("（`a-`）"));
    }

    #[test]
    fn notes_become_list_items() {
        let mut c = ctx(four_groups());
        assert!(
            render(&c)
                .unwrap()
                .contains("\n- 本期无额外隐私/风控要点。\n")
        );
        c.notes = vec!["隐私：甲".into(), "风控提示：乙".into()];
        let text = render(&c).unwrap();
        assert!(text.contains(
            "不要运行会读取本机浏览器 Cookie 的工具。\n- 隐私：甲\n- 风控提示：乙\n\n## 授权与范围"
        ));
        assert!(!text.contains("本期无额外"));
    }

    #[test]
    fn eastern_time_follows_daylight_saving() {
        let d = |m, day| Date::new(2026, m, day).unwrap();
        assert_eq!(us_eastern(d(10, 9)), ("EDT，UTC-4", 12));
        assert_eq!(us_eastern(d(3, 7)).1, 13);
        assert_eq!(us_eastern(d(3, 8)).1, 12);
        assert_eq!(us_eastern(d(10, 31)).1, 12);
        assert_eq!(us_eastern(d(11, 1)), ("EST，UTC-5", 13));
        assert_eq!(us_eastern(d(1, 15)).1, 13);
        assert_eq!(weekday(d(10, 8)), 4);
        let mut c = ctx(four_groups());
        c.date = d(11, 20);
        assert!(
            render(&c)
                .unwrap()
                .contains("美东（EST，UTC-5），北京时间 = 本机时间 + 13 小时")
        );
    }

    const SCAN: &str = "\
## 近 3 天速览已收

| 期次 | 标签 | 事实 |
|---|---|---|
| 1009 | Epoch | x |

## 已发过

| 期次 | 摘要 |
|---|---|
| 1007 | a |
| 1008 | b |
";

    fn prior(mmdd: &str, kind: Kind, day: u32) -> Prior {
        Prior {
            mmdd: mmdd.into(),
            date: Date::new(2026, 10, day).unwrap(),
            kind,
            title: Some(format!("标题{mmdd}")),
            others: vec!["条目甲".into(), "条目乙".into()],
        }
    }

    #[test]
    fn finds_rows_only_in_their_section() {
        assert!(has_row(SCAN, "## 已发过", "1008"));
        assert!(!has_row(SCAN, "## 已发过", "1009"));
        assert!(has_row(SCAN, "## 近 3 天速览已收", "1009"));
        assert!(!has_row(SCAN, "## 近 3 天速览已收", "1008"));
        assert!(!has_row(SCAN, "## 不存在", "1008"));
        // CRLF-folded input and indented rows still match; a cell that merely
        // contains the number does not.
        assert!(has_row("## 已发过\n  | 1010 | x |\n", "## 已发过", "1010"));
        assert!(!has_row("## 已发过\n| 11010 | x |\n", "## 已发过", "1010"));
    }

    #[test]
    fn detects_issues_missing_from_daily_scan() {
        let current = Date::new(2026, 10, 10).unwrap();
        let priors = [
            prior("1007", Kind::Main, 7),
            prior("1008", Kind::Main, 8),
            prior("1009", Kind::Main, 9),
        ];
        let missing: Vec<&str> = unpublished_issues(&priors, SCAN, current)
            .iter()
            .map(|p| p.mmdd.as_str())
            .collect();
        assert_eq!(missing, ["1009"]);

        // A 速览专帖 is looked up in the 速览 table, and only while its rows
        // can still be there.
        let roundups = [
            prior("1006", Kind::Roundup, 6),
            prior("1008", Kind::Roundup, 8),
            prior("1009", Kind::Roundup, 9),
        ];
        let missing: Vec<&str> = unpublished_issues(&roundups, SCAN, current)
            .iter()
            .map(|p| p.mmdd.as_str())
            .collect();
        assert_eq!(missing, ["1008"]);
        assert!(unpublished_issues(&priors, "", current).len() == 3);

        // Boundary: three days back still counts, four days back does not.
        let edge = [
            prior("1006", Kind::Roundup, 6),
            prior("1007", Kind::Roundup, 7),
        ];
        let missing: Vec<&str> = unpublished_issues(&edge, SCAN, current)
            .iter()
            .map(|p| p.mmdd.as_str())
            .collect();
        assert_eq!(missing, ["1007"]);
    }

    #[test]
    fn phrases_unpublished_issues() {
        assert_eq!(unpublished_phrase(&[]), "");
        let p = prior("1008", Kind::Main, 8);
        assert_eq!(
            unpublished_phrase(&[&p]),
            "，**另加尚未写进 daily-scan 的 1008 期**：主帖 = 标题1008；速览 = 条目甲、条目乙"
        );
        let q = prior("1009", Kind::Roundup, 9);
        assert_eq!(
            unpublished_phrase(&[&p, &q]),
            "，**另加尚未写进 daily-scan 的 1008、1009 期**：1008 期：主帖 = 标题1008；速览 = 条目甲、条目乙。1009 期：速览 = 条目甲、条目乙"
        );
        let mut c = ctx(four_groups());
        c.unpublished = unpublished_phrase(&[&p]);
        let text = render(&c).unwrap();
        assert!(text.contains("“近 3 天速览已收”，**另加尚未写进 daily-scan 的 1008 期**：主帖 = 标题1008；速览 = 条目甲、条目乙。\n"));
        c.unpublished = String::new();
        assert!(render(&c).unwrap().contains("“近 3 天速览已收”。\n"));
    }

    #[test]
    fn parses_status_and_names_foreign_changes() {
        let raw = "?? docs/2610/1008/\0 M daily-scan.md\0R  new.md\0old.md\0?? docs/2610/1009/images/a.png\0?? .handoff/2026-10-09-1009-evidence.md\0?? .handoff/README.md\0";
        let rows = parse_status_z(raw);
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[2], ("R ".into(), "new.md".into()));
        let own = [
            ".handoff/2026-10-09-1009-evidence.md".to_string(),
            ".handoff/README.md".to_string(),
        ];
        let phrase = |rows: &[(String, String)], rel: &str, own: &[String]| {
            untracked_phrase(&foreign_changes(rows, rel, own))
        };
        assert_eq!(
            phrase(&rows, "docs/2610/1009", &own),
            "工作区已知的未提交改动，**一律不要碰**：整个 `docs/2610/1008/` 目录、`daily-scan.md`、`new.md`"
        );
        assert_eq!(phrase(&[], "docs/2610/1009", &own), "工作区干净");
        assert_eq!(phrase(&rows[3..], "docs/2610/1009", &own), "工作区干净");
        // When the index is not this command's own output it is named.
        assert_eq!(
            phrase(&rows[3..], "docs/2610/1009", &own[..1]),
            "工作区已知的未提交改动，**一律不要碰**：`.handoff/README.md`"
        );
        let many: Vec<(String, String)> = (0..12)
            .map(|i| ("??".into(), format!("f{i}.txt")))
            .collect();
        let p = phrase(&many, "docs/2610/1009", &[]);
        assert!(p.contains("`f9.txt`等共 12 处"), "{p}");
        assert!(!p.contains("f10.txt"));
    }

    #[test]
    fn untracked_parents_of_the_issue_are_not_foreign() {
        // The month's first issue: git folds the new month into one entry.
        let rows = parse_status_z("?? docs/2611/\0?? docs/26110/\0?? docs/2610/1031/\0");
        assert_eq!(
            foreign_changes(&rows, "docs/2611/1101", &[]),
            ["整个 `docs/26110/` 目录", "整个 `docs/2610/1031/` 目录"]
        );
        assert!(foreign_changes(&parse_status_z("?? docs/\0"), "docs/2611/1101", &[]).is_empty());
        // A tracked-but-modified file under the parent is still foreign.
        let rows = parse_status_z(" M docs/2611/README.md\0");
        assert_eq!(foreign_changes(&rows, "docs/2611/1101", &[]).len(), 1);
    }

    #[test]
    fn unfolds_parents_to_show_other_issues() {
        let rows = parse_status_z(" M daily-scan.md\0?? docs/2611/\0");
        // An earlier uncommitted issue of the same month shares the parent.
        let inside = parse_status_z(
            "?? docs/2611/1101/sources/a.txt\0?? docs/2611/1101/sources/b.txt\0?? docs/2611/1102/sources/x.txt\0?? docs/2611/notes.md\0",
        );
        let rows = unfold_parents(&rows, "docs/2611/1102", &inside);
        assert_eq!(
            foreign_changes(&rows, "docs/2611/1102", &[]),
            [
                "`daily-scan.md`",
                "整个 `docs/2611/1101/` 目录",
                "`docs/2611/notes.md`"
            ]
        );
        // Only the issue's own files inside: the parent just disappears.
        let only = parse_status_z("?? docs/2611/1102/sources/x.txt\0");
        let rows = unfold_parents(&parse_status_z("?? docs/\0"), "docs/2611/1102", &only);
        assert!(rows.is_empty());
        // Nested parents fold to the first directory off the issue's path.
        let rows = unfold_parents(&parse_status_z("?? docs/\0"), "docs/2611/1102", &inside);
        assert_eq!(rows[0].1, "docs/2611/1101/");
    }

    #[test]
    fn keep_sentence_follows_foreign_changes() {
        let mut c = ctx(four_groups());
        let text = render(&c).unwrap();
        assert!(text.contains(
            "仓库 `E:\\gitclone\\AI-Barking`（分支 `main`，基线 commit `abc1234`）。工作区干净。"
        ));
        assert!(text.contains("下本组文件（及本派工单）。\n"));
        assert!(!text.contains("保持原样"));
        c.foreign = vec!["`daily-scan.md`".into()];
        c.branch = "claude/x".into();
        let text = render(&c).unwrap();
        assert!(text.contains("（分支 `claude/x`，基线 commit `abc1234`）。工作区已知的未提交改动，**一律不要碰**：`daily-scan.md`。"));
        assert!(text.contains("（及本派工单），上面列出的既有未提交改动保持原样。\n"));
    }

    #[test]
    fn notes_which_directories_exist() {
        let rel = "docs/2610/1009";
        assert_eq!(
            dirs_note(rel, &["images", "sources"]),
            "已新建空目录 `docs/2610/1009/images/`、`docs/2610/1009/sources/`"
        );
        assert_eq!(
            dirs_note(rel, &["sources"]),
            "已新建空目录 `docs/2610/1009/sources/`；`docs/2610/1009/images/` 已存在"
        );
        assert_eq!(
            dirs_note(rel, &[]),
            "`docs/2610/1009/images/`、`docs/2610/1009/sources/` 均已存在"
        );
        assert_eq!(prev_phrase(&[]), "本仓库没有更早的期次");
        assert_eq!(
            prev_phrase(&["docs/2610/1008".into()]),
            "`docs/2610/1008/sources/` 里的 `capture-log-*.md`"
        );
    }

    fn entries() -> Vec<(String, String)> {
        vec![
            (
                "2026-10-09-1009-evidence.md".into(),
                "取证任务：1009 期".into(),
            ),
            ("2026-09-24-usage-evidence.md".into(), "0924 [x]".into()),
        ]
    }

    #[test]
    fn index_keeps_head_and_tail() {
        let old = "# 标题\n\n开头说明。\n\n- [旧甲](a.md)\n- [旧乙](b.md)\n\n结尾说明。\n";
        let text = index_text(old, &entries());
        assert_eq!(
            text,
            "# 标题\n\n开头说明。\n\n- [取证任务：1009 期](2026-10-09-1009-evidence.md)\n- [0924 \\[x\\]](2026-09-24-usage-evidence.md)\n\n结尾说明。\n"
        );
        // Idempotent: running it on its own output changes nothing.
        assert_eq!(index_text(&text, &entries()), text);
        // No list to anchor on: fall back to the stock head and tail.
        let fresh = index_text("", &entries());
        assert!(fresh.starts_with("# 历史派工与交接\n\n本目录随仓库保存"));
        assert!(fresh.ends_with("见仓库首页。\n"));
        assert_eq!(fresh.matches("\n- [").count(), 2);
    }

    #[test]
    fn index_titles_come_from_first_line() {
        assert_eq!(
            index_title("# 取证任务：1009 期\r\n正文", "x.md"),
            "取证任务：1009 期"
        );
        assert_eq!(index_title("\u{feff}# 带 BOM", "x.md"), "带 BOM");
        assert_eq!(index_title("没有井号\n", "2026-x.md"), "2026-x");
        assert_eq!(index_title("", "2026-x.md"), "2026-x");
    }

    #[test]
    fn template_is_split_into_three_parts() {
        let (body, main, roundup) = split_template(TEMPLATE).unwrap();
        assert!(body.starts_with("# 取证任务：{MMDD} 期"));
        assert!(body.trim_end().ends_with("{GROUP_SECTIONS}"));
        assert!(main.starts_with("# {GROUP} 组：{TOPIC}"));
        assert!(roundup.starts_with("# {GROUP} 组：{TOPIC}"));
        assert!(split_template("no markers").is_err());
    }
}
