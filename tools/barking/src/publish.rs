//! `barking publish-check` and `barking stage`: the mechanical half of
//! docs/workflow/publish.md (steps 1-2, then splitting the commit into the
//! three batches of AGENTS.md "提交约定").
//!
//! Neither command commits or pushes, and neither writes the table rows of
//! README.md / daily-scan.md: those are only *checked* here, line by line
//! inside their section. Writing them is `scan-rows`'s job.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use sha2::{Digest, Sha256};

use crate::card::{self, Kind};
use crate::lint::{self, Level};
use crate::offsite;
use crate::repo::{self, Issue, human_size};

const CHECK_USAGE: &str = "用法：barking publish-check <期次目录>";
const STAGE_USAGE: &str = "用法：barking stage <期次目录> [--batch 1|2|3] [--add]";

/// Largest file that may be committed without asking (AGENTS.md, publish.md):
/// history cannot be undone with `.gitignore`.
const SIZE_LIMIT: u64 = 1_048_576;
const STEPS: usize = 7;
/// The marker `scan-rows` leaves in a 已发过 row until Claude writes the
/// summary; pushing it would pollute the scheduled scan's prompt.
const PLACEHOLDER: &str = "待填";
/// Files that must stay git-ignored: credentials and browser session state.
const MUST_IGNORE: [&str; 5] = [
    ".env",
    "cookies.txt",
    "cookies.json",
    "storage-state.json",
    ".playwright-cli/",
];
/// Windows caps a command line near 32 767 characters; stay well below it.
const MAX_ARG_LEN: usize = 24_000;
/// Hits printed per check before the rest is summarised.
const MAX_LISTED: usize = 30;

// ---------------------------------------------------------------- publish-check

/// Outcome of one `publish-check` step: lines to show and whether it passed.
struct Step {
    ok: bool,
    notes: Vec<String>,
}

impl Step {
    fn pass(notes: Vec<String>) -> Step {
        Step { ok: true, notes }
    }

    fn fail(notes: Vec<String>) -> Step {
        Step { ok: false, notes }
    }

    /// Pass or fail by whether `notes` already holds problems.
    fn by(problems: bool, notes: Vec<String>) -> Step {
        Step {
            ok: !problems,
            notes,
        }
    }
}

pub fn run_check(args: &[String]) -> ExitCode {
    let [dir] = args else {
        eprintln!("{CHECK_USAGE}");
        return ExitCode::from(2);
    };
    if dir.starts_with("--") {
        eprintln!("{CHECK_USAGE}");
        return ExitCode::from(2);
    }
    let issue = match Issue::resolve(Path::new(dir)) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    check(&issue)
}

type StepFn = fn(&Issue) -> Step;

/// The seven steps run in this order and stop at the first failure: later
/// steps (the manifest, the size and identity scans) are only meaningful on
/// files that earlier steps have already settled.
fn check(issue: &Issue) -> ExitCode {
    let steps: [(&str, StepFn); STEPS] = [
        ("lint", step_lint),
        ("发布后标记", step_markers),
        ("oxipng 压缩", step_oxipng),
        ("offsite 清单", step_offsite),
        ("大文件", step_size),
        ("身份", step_identity),
        ("凭据与会话状态", step_secrets),
    ];
    for (i, (name, run)) in steps.iter().enumerate() {
        let step = run(issue);
        println!(
            "[{}/{STEPS}] {name} … {}",
            i + 1,
            if step.ok { "ok" } else { "失败" }
        );
        for n in &step.notes {
            println!("  {n}");
        }
        if !step.ok {
            println!(
                "publish-check 在第 {} 步（{name}）失败，已停止；修好后重跑。",
                i + 1
            );
            return ExitCode::FAILURE;
        }
    }
    println!("publish-check：{STEPS}/{STEPS} 通过。");
    let rel = issue.rel();
    println!("下一步：barking stage {rel}（按 --batch 1、2、3 --add 依序暂存，再由 Claude 提交）");
    ExitCode::SUCCESS
}

/// Strip the repository root from the paths lint prints.
fn relativize(root: &Path, text: String) -> String {
    let prefix = format!("{}{}", root.display(), std::path::MAIN_SEPARATOR);
    text.replace(&prefix, "")
}

fn step_lint(issue: &Issue) -> Step {
    let findings = lint::lint_issue(&issue.dir);
    let (mut errors, mut warns) = (0usize, 0usize);
    let mut notes = Vec::new();
    for f in &findings {
        match f.level {
            Level::Error => errors += 1,
            Level::Warn => warns += 1,
            Level::Info => {}
        }
        notes.push(relativize(&issue.root, f.to_string()));
    }
    notes.push(format!("{errors} 个错误，{warns} 个警告"));
    Step::by(errors > 0, notes)
}

/// Lines of the section opened by the `## ` line `heading`, up to the next
/// `## ` line; each is trimmed. `None` if the heading is absent.
fn section<'a>(text: &'a str, heading: &str) -> Option<Vec<&'a str>> {
    let mut lines = text.lines();
    lines.find(|l| l.trim_end() == heading)?;
    Some(
        lines
            .take_while(|l| !l.starts_with("## "))
            .map(str::trim)
            .collect(),
    )
}

/// What publish.md step 1 says must be in README.md and daily-scan.md once
/// the issue is out. `entries` are the `(tag, text)` of the non-main
/// `[[roundup]]` items of cards.toml. Empty result = all rows present.
fn marker_problems(
    readme: &str,
    scan: &str,
    issue: &Issue,
    kind: Kind,
    entries: &[(&str, &str)],
) -> Vec<String> {
    let mmdd = &issue.mmdd;
    let mut out = Vec::new();

    let row = format!("| 20{} / {mmdd} |", &issue.yymm[..2]);
    let link = format!("({}/doc_{mmdd}_publish.txt)", issue.rel());
    match section(readme, "## 内容目录") {
        None => out.push("README.md 缺少“## 内容目录”一节".to_string()),
        Some(lines) => match lines.iter().find(|l| l.starts_with(&row)) {
            None => out.push(format!("README.md 内容目录没有 `{row}` 行")),
            Some(l) if !l.contains(&link) => out.push(format!(
                "README.md 内容目录的 `{row}` 行没有链接到本期正文 {link}"
            )),
            Some(_) => {}
        },
    }

    // A 速览专帖 has no main post, hence no 已发过 row; a stray placeholder
    // is still wrong wherever it is.
    match section(scan, "## 已发过") {
        None => out.push("daily-scan.md 缺少“## 已发过”一节".to_string()),
        Some(lines) => {
            let row = format!("| {mmdd} |");
            if kind == Kind::Main && !lines.iter().any(|l| l.starts_with(&row)) {
                out.push(format!("daily-scan.md“已发过”没有 `{row}` 行"));
            }
            // Whichever issue left it, a placeholder pushed to origin poisons
            // the scheduled scan; `stage --batch 1` refuses on the same text.
            for l in lines.iter().filter(|l| l.contains(PLACEHOLDER)) {
                let head: String = l.chars().take(24).collect();
                out.push(format!(
                    "daily-scan.md“已发过”仍含“{PLACEHOLDER}”占位：`{head}…`，须按最终发布版填写"
                ));
            }
        }
    }

    if !entries.is_empty() {
        match section(scan, "## 近 3 天速览已收") {
            None => out.push("daily-scan.md 缺少“## 近 3 天速览已收”一节".to_string()),
            Some(lines) => {
                for (tag, text) in entries {
                    let want = format!("| {mmdd} | {tag} | {text} |");
                    if !lines.contains(&want.as_str()) {
                        out.push(format!("daily-scan.md“近 3 天速览已收”缺少 `{want}`"));
                    }
                }
            }
        }
    }
    out
}

fn step_markers(issue: &Issue) -> Step {
    match read_markers(issue) {
        Ok(problems) if problems.is_empty() => Step::pass(vec![
            "README 期次索引、daily-scan 已发过与近 3 天速览已收均已就位".to_string(),
        ]),
        Ok(mut problems) => {
            problems.push(format!(
                "先运行 barking scan-rows {}（只在 Develata 说“已发布”之后）；“已发过”的占位须 Claude 按最终发布版填写",
                issue.rel()
            ));
            Step::fail(problems)
        }
        Err(e) => Step::fail(vec![e]),
    }
}

fn read_markers(issue: &Issue) -> Result<Vec<String>, String> {
    let read = |name: &str| {
        let path = issue.root.join(name);
        fs::read_to_string(&path)
            .map(|t| t.replace("\r\n", "\n"))
            .map_err(|e| format!("{}: {e}", path.display()))
    };
    let (readme, scan) = (read("README.md")?, read("daily-scan.md")?);
    let spec = card::load_spec(&issue.images())?;
    let kind = spec.as_ref().map_or(Kind::Main, |s| s.kind);
    let entries: Vec<(&str, &str)> = spec
        .iter()
        .flat_map(|s| s.other_entries())
        .map(|r| (r.tag.as_str(), r.text.as_str()))
        .collect();
    Ok(marker_problems(&readme, &scan, issue, kind, &entries))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A PNG recorded in offsite.tsv with the same hash is already registered
/// (and may be uploaded): recompressing it would invalidate the record.
fn already_registered(manifest: &[(String, u64, String)], rel: &str, sha: &str) -> bool {
    manifest.iter().any(|(p, _, s)| p == rel && s == sha)
}

/// Split `items` into runs whose joined length stays under `limit`, so one
/// command line never grows past the OS limit. Every run has at least one item.
fn chunk_by_len<T: AsRef<OsStr>>(items: &[T], limit: usize) -> Vec<&[T]> {
    let mut out = Vec::new();
    let (mut start, mut len) = (0usize, 0usize);
    for (i, item) in items.iter().enumerate() {
        let add = item.as_ref().len() + 3;
        if i > start && len + add > limit {
            out.push(&items[start..i]);
            start = i;
            len = 0;
        }
        len += add;
    }
    if start < items.len() {
        out.push(&items[start..]);
    }
    out
}

/// `images/*.png`, sorted; sub-directories are not entered.
fn top_level_pngs(images: &Path) -> Result<Vec<PathBuf>, String> {
    let rd = match fs::read_dir(images) {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", images.display())),
    };
    let mut out: Vec<PathBuf> = rd
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .and_then(OsStr::to_str)
                    .is_some_and(|x| x.eq_ignore_ascii_case("png"))
        })
        .collect();
    out.sort();
    Ok(out)
}

fn step_oxipng(issue: &Issue) -> Step {
    match compress_pngs(issue) {
        Ok(step) => step,
        Err(e) => Step::fail(vec![e]),
    }
}

fn compress_pngs(issue: &Issue) -> Result<Step, String> {
    let pngs = top_level_pngs(&issue.images())?;
    if pngs.is_empty() {
        return Ok(Step::pass(vec!["images/ 下没有 PNG，跳过".to_string()]));
    }
    let manifest_path = issue.sources().join(offsite::MANIFEST);
    // A missing manifest means nothing is registered yet; any other error
    // must not be mistaken for that, or registered (maybe uploaded) files
    // would be recompressed and the committed hashes stop matching OpenList.
    let manifest = match fs::read_to_string(&manifest_path) {
        Ok(t) => offsite::parse_manifest(&t),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => {
            return Ok(Step::fail(vec![format!(
                "{}: {e}",
                manifest_path.display()
            )]));
        }
    };

    let (mut todo, mut skipped, mut before) = (Vec::new(), 0usize, 0u64);
    for p in &pngs {
        let bytes = fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?;
        let name = p.file_name().and_then(OsStr::to_str).unwrap_or("");
        if already_registered(&manifest, &format!("images/{name}"), &sha256_hex(&bytes)) {
            skipped += 1;
        } else {
            before += bytes.len() as u64;
            todo.push(p.clone());
        }
    }
    let mut notes = Vec::new();
    if skipped > 0 {
        notes.push(format!(
            "{skipped} 个 PNG 已登记在 {}，跳过（压缩会让清单失效）",
            offsite::MANIFEST
        ));
    }
    if todo.is_empty() {
        notes.push("没有需要压缩的 PNG".to_string());
        return Ok(Step::pass(notes));
    }

    println!(
        "oxipng -o 4 --strip safe：压缩 {} 个 PNG（{}）…",
        todo.len(),
        human_size(before)
    );
    for chunk in chunk_by_len(&todo, MAX_ARG_LEN) {
        let out = match Command::new("oxipng")
            .args(["-o", "4", "--strip", "safe"])
            .args(chunk)
            .output()
        {
            Ok(o) => o,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                notes.push("找不到 oxipng：scoop install oxipng".to_string());
                return Ok(Step::fail(notes));
            }
            Err(e) => return Err(format!("无法运行 oxipng：{e}")),
        };
        if !out.status.success() {
            notes.push(format!(
                "oxipng 失败（{}）：{}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            ));
            return Ok(Step::fail(notes));
        }
    }
    let after: u64 = todo
        .iter()
        .map(|p| fs::metadata(p).map_or(0, |m| m.len()))
        .sum();
    notes.push(format!(
        "压缩 {} 个 PNG：{before} -> {after} 字节（{} -> {}）",
        todo.len(),
        human_size(before),
        human_size(after)
    ));
    Ok(Step::pass(notes))
}

fn step_offsite(issue: &Issue) -> Step {
    // Checked before the manifest is written: a cookies.json under sources/
    // would otherwise get its size and hash committed before step 7 objects.
    let entries = match offsite::ignored_files(&issue.root, &issue.yymm, &issue.mmdd) {
        Ok(entries) => entries,
        Err(e) => return Step::fail(vec![e]),
    };
    let secrets: Vec<&str> = entries
        .iter()
        .map(|e| e.rel.as_str())
        .filter(|p| is_secret_name(p))
        .collect();
    if !secrets.is_empty() {
        return Step::fail(vec![format!(
            "不入库原件里有凭据或会话状态类文件，不登记、不上传：{}",
            secrets.join("、")
        )]);
    }
    // `offsite::process` rewrites the manifest from the files present here.
    // In a clone without images, or with some originals not yet fetched back,
    // that would silently drop their size and hash: the archive's only proof.
    let path = issue.sources().join(offsite::MANIFEST);
    let old = match fs::read_to_string(&path) {
        Ok(t) => offsite::parse_manifest(&t),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Step::fail(vec![format!("{}: {e}", path.display())]),
    };
    let present: BTreeSet<&str> = entries.iter().map(|e| e.rel.as_str()).collect();
    let missing = missing_offsite(&old, &present);
    if !missing.is_empty() {
        let mut notes = vec![format!(
            "{}/sources/{} 已登记、但本地不在不入库原件里的 {} 个文件（重写清单会删掉它们的大小与 SHA-256，未写入）：",
            issue.rel(),
            offsite::MANIFEST,
            missing.len()
        )];
        notes.extend(missing.iter().take(MAX_LISTED).map(|p| {
            if issue.dir.join(p).is_file() {
                format!("{p}（本地存在，但不再匹配不入库规则）")
            } else {
                format!("{p}（本地不存在）")
            }
        }));
        let rest = missing.len().saturating_sub(MAX_LISTED);
        if rest > 0 {
            notes.push(format!("…另有 {rest} 个"));
        }
        notes.push(
            "先从 OpenList 取回这些原件再重跑；确属应删除的，确认后手动从清单删去对应行。"
                .to_string(),
        );
        return Step::fail(notes);
    }
    // Written from the very entries just checked: a second scan could see a
    // file moved away in between and shrink the manifest after all.
    if let Err(e) = offsite::write_manifest(&issue.dir, &entries) {
        return Step::fail(vec![e]);
    }
    Step::pass(vec![format!(
        "{}/sources/{} 登记 {} 个文件",
        issue.rel(),
        offsite::MANIFEST,
        entries.len()
    )])
}

/// Paths in the old manifest (`old`, as [`offsite::parse_manifest`] gives
/// them) that the rewritten manifest would lose because they are not among
/// the offsite files found now (`present`). In manifest order, deduplicated.
fn missing_offsite(old: &[(String, u64, String)], present: &BTreeSet<&str>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    old.iter()
        .map(|(p, _, _)| p.as_str())
        .filter(|p| !present.contains(p) && seen.insert(*p))
        .map(str::to_string)
        .collect()
}

/// Files that would go into the commits: pending files of the issue and of
/// `.handoff/`, plus the two repo-layer files. Deleted files drop out.
fn candidates(issue: &Issue, rows: &[(String, String)]) -> Vec<String> {
    candidate_paths(issue, rows)
        .into_iter()
        .filter(|p| issue.root.join(p).is_file())
        .collect()
}

/// [`candidates`] before the filesystem is consulted.
fn candidate_paths(issue: &Issue, rows: &[(String, String)]) -> BTreeSet<String> {
    let dir = format!("{}/", issue.rel());
    let mut set: BTreeSet<String> = rows
        .iter()
        .filter(|(xy, p)| !xy.contains('D') && (p.starts_with(&dir) || p.starts_with(".handoff/")))
        .map(|(_, p)| p.clone())
        .collect();
    set.insert("README.md".to_string());
    set.insert("daily-scan.md".to_string());
    set
}

fn size_of(root: &Path, rel: &str) -> Option<u64> {
    fs::metadata(root.join(rel)).ok().map(|m| m.len())
}

fn step_size(issue: &Issue) -> Step {
    let rows = match repo::git_status(&issue.root) {
        Ok(r) => r,
        Err(e) => return Step::fail(vec![e]),
    };
    let files = candidates(issue, &rows);
    let big: Vec<String> = files
        .iter()
        .filter_map(|p| {
            size_of(&issue.root, p)
                .filter(|&s| s > SIZE_LIMIT)
                .map(|s| format!("{p}（{}）", human_size(s)))
        })
        .collect();
    let mut notes = vec![format!("待提交候选 {} 个文件", files.len())];
    if big.is_empty() {
        return Step::pass(notes);
    }
    notes.push(
        "以下文件超过 1 MiB，先问 Develata 是否必须入库（入库的历史无法靠 .gitignore 撤回）："
            .into(),
    );
    notes.extend(big);
    Step::fail(notes)
}

/// `;`-separated, case-insensitive substrings; blanks dropped.
fn parse_patterns(raw: &str) -> Vec<String> {
    raw.split(';')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// One identity hit: `line` is `None` when the file *name* matched. Only the
/// index of the pattern is kept, so the report never echoes the identity.
#[derive(Debug, PartialEq, Eq)]
struct Hit {
    line: Option<usize>,
    pattern: usize,
}

fn identity_hits(rel: &str, content: &str, patterns: &[String]) -> Vec<Hit> {
    let mut hits = Vec::new();
    let name = rel.to_lowercase();
    if let Some(pattern) = patterns.iter().position(|p| name.contains(p)) {
        hits.push(Hit {
            line: None,
            pattern,
        });
    }
    for (i, line) in content.lines().enumerate() {
        let lower = line.to_lowercase();
        if let Some(pattern) = patterns.iter().position(|p| lower.contains(p)) {
            hits.push(Hit {
                line: Some(i + 1),
                pattern,
            });
        }
    }
    hits
}

fn step_identity(issue: &Issue) -> Step {
    let raw = std::env::var("BARKING_IDENTITY_PATTERNS")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(|| {
            offsite::load_env(&issue.root)
                .get("BARKING_IDENTITY_PATTERNS")
                .cloned()
        })
        .unwrap_or_default();
    let patterns = parse_patterns(&raw);
    if patterns.is_empty() {
        return Step::pass(vec![
            "warn: 身份检查跳过：.env 未设置 BARKING_IDENTITY_PATTERNS".to_string(),
        ]);
    }
    let rows = match repo::git_status(&issue.root) {
        Ok(r) => r,
        Err(e) => return Step::fail(vec![e]),
    };
    let files = candidates(issue, &rows);
    let mut hits = Vec::new();
    for rel in &files {
        let content = match fs::read(issue.root.join(rel)) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => return Step::fail(vec![format!("{rel}: {e}")]),
        };
        for h in identity_hits(rel, &content, &patterns) {
            hits.push(match h.line {
                Some(n) => format!("{rel}:{n}（模式 #{}）", h.pattern + 1),
                None => format!("{rel}（文件名，模式 #{}）", h.pattern + 1),
            });
        }
    }
    let mut notes = vec![format!(
        "按 {} 个模式检查 {} 个候选文件",
        patterns.len(),
        files.len()
    )];
    if hits.is_empty() {
        return Step::pass(notes);
    }
    notes.push(format!(
        "命中 {} 处，须先脱敏再提交（模式序号对应 .env 里 BARKING_IDENTITY_PATTERNS 的顺序）：",
        hits.len()
    ));
    let rest = hits.len().saturating_sub(MAX_LISTED);
    notes.extend(hits.into_iter().take(MAX_LISTED));
    if rest > 0 {
        notes.push(format!("…另有 {rest} 处"));
    }
    Step::fail(notes)
}

/// `.env*` (except the example), `cookies.*`, `storage-state*.json`, or
/// anything under `.playwright-cli/`: names that carry credentials or a
/// logged-in browser session.
fn is_secret_name(rel: &str) -> bool {
    let lower = rel.to_lowercase();
    if lower.split('/').any(|c| c == ".playwright-cli") {
        return true;
    }
    let base = lower.rsplit('/').next().unwrap_or("");
    (base.starts_with(".env") && base != ".env.example")
        || base.starts_with("cookies.")
        || (base.starts_with("storage-state") && base.ends_with(".json"))
}

/// `git check-ignore -q`: exit 0 = ignored, 1 = not ignored, else an error.
fn is_ignored(root: &Path, path: &str) -> Result<bool, String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(["check-ignore", "-q", "--", path])
        .output()
        .map_err(|e| format!("无法运行 git：{e}"))?;
    match out.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(format!(
            "git check-ignore {path} 失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        )),
    }
}

fn step_secrets(issue: &Issue) -> Step {
    let mut problems = Vec::new();
    for path in MUST_IGNORE {
        match is_ignored(&issue.root, path) {
            Ok(true) => {}
            Ok(false) => problems.push(format!("{path} 不再被 .gitignore 排除")),
            Err(e) => return Step::fail(vec![e]),
        }
    }
    let rows = match repo::git_status(&issue.root) {
        Ok(r) => r,
        Err(e) => return Step::fail(vec![e]),
    };
    for p in candidates(issue, &rows)
        .iter()
        .filter(|p| is_secret_name(p))
    {
        problems.push(format!("待提交文件名像凭据或会话状态：{p}"));
    }
    // Ignored files never reach a commit, but `offsite` registers every
    // ignored file of the issue (and `--upload` ships it to OpenList), so a
    // stray cookies.json in sources/ would leak through the manifest.
    let manifest = issue.sources().join(offsite::MANIFEST);
    if let Ok(text) = fs::read_to_string(&manifest) {
        for (p, _, _) in offsite::parse_manifest(&text) {
            if is_secret_name(&p) {
                problems.push(format!(
                    "{} 登记了凭据或会话状态类文件，--upload 会把它传上网盘：{p}",
                    offsite::MANIFEST
                ));
            }
        }
    }
    if problems.is_empty() {
        return Step::pass(vec![format!(
            "{} 项均被忽略，候选与 {} 中没有凭据文件",
            MUST_IGNORE.len(),
            offsite::MANIFEST
        )]);
    }
    Step::fail(problems)
}

// ---------------------------------------------------------------------- stage

struct StageOpts {
    dir: PathBuf,
    batch: Option<u8>,
    add: bool,
}

fn parse_stage_args(args: &[String]) -> Result<StageOpts, String> {
    let (mut dir, mut batch, mut add) = (None, None, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--add" => add = true,
            "--batch" => {
                let v = it.next().ok_or("--batch 需要 1、2 或 3")?;
                batch = Some(match v.as_str() {
                    "1" => 1,
                    "2" => 2,
                    "3" => 3,
                    _ => return Err(format!("--batch 只接受 1、2、3，收到 {v}")),
                });
            }
            f if f.starts_with("--") => return Err(format!("未知选项 {f}")),
            d if dir.is_none() => dir = Some(PathBuf::from(d)),
            _ => return Err("只能给一个期次目录".to_string()),
        }
    }
    let dir = dir.ok_or("缺少期次目录")?;
    if add && batch.is_none() {
        return Err("--add 须与 --batch 一起用（每批单独暂存、单独提交）".to_string());
    }
    Ok(StageOpts { dir, batch, add })
}

pub fn run_stage(args: &[String]) -> ExitCode {
    let opts = match parse_stage_args(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}\n{STAGE_USAGE}");
            return ExitCode::from(2);
        }
    };
    let issue = match Issue::resolve(&opts.dir) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    match stage(&issue, &opts) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Pending files of the repository sorted into the commit batches. Paths are
/// repo-relative with `/`.
#[derive(Debug, Default, PartialEq, Eq)]
struct Plan {
    /// ① README.md, daily-scan.md, this issue's `.handoff` sheets.
    repo: Vec<String>,
    /// ② the issue's remaining files: archives, scripts, offsite.tsv.
    archive: Vec<String>,
    /// ③ the finished issue.
    product: Vec<String>,
    /// CORRECTION.md: committed on its own after publishing.
    corrections: Vec<String>,
    /// Pending but not part of this issue (other issues, tooling, ...).
    others: Vec<String>,
}

/// The files of batch ③, relative to the issue directory.
fn product_files(mmdd: &str) -> [String; 7] {
    [
        format!("doc_{mmdd}_publish.txt"),
        "images/README.md".to_string(),
        "images/cards.toml".to_string(),
        "sources/README.md".to_string(),
        "sources/fact-check.md".to_string(),
        "sources/evidence.md".to_string(),
        "sources/capture-log.md".to_string(),
    ]
}

/// `.handoff/<date>-<MMDD>-<slug>.md` of this issue.
fn is_handoff(path: &str, mmdd: &str) -> bool {
    let Some(name) = path.strip_prefix(".handoff/") else {
        return false;
    };
    !name.contains('/') && name.ends_with(".md") && name.contains(&format!("-{mmdd}-"))
}

/// The sheet index, rewritten by `barking handoff` with every new sheet.
const HANDOFF_INDEX: &str = ".handoff/README.md";

fn partition(issue: &Issue, rows: &[(String, String)]) -> Plan {
    let dir = format!("{}/", issue.rel());
    let product = product_files(&issue.mmdd);
    let mut plan = Plan::default();
    // The index belongs to ① only alongside this issue's own sheet; a pending
    // index rewrite from some other change is not this issue's.
    let has_sheet = rows.iter().any(|(_, p)| is_handoff(p, &issue.mmdd));
    for (_, path) in rows {
        let bucket = if path == "README.md"
            || path == "daily-scan.md"
            || is_handoff(path, &issue.mmdd)
            || (path == HANDOFF_INDEX && has_sheet)
        {
            &mut plan.repo
        } else if let Some(rest) = path.strip_prefix(&dir) {
            if rest == "CORRECTION.md" {
                &mut plan.corrections
            } else if product.iter().any(|f| f == rest) {
                &mut plan.product
            } else {
                &mut plan.archive
            }
        } else {
            &mut plan.others
        };
        bucket.push(path.clone());
    }
    for v in [
        &mut plan.repo,
        &mut plan.archive,
        &mut plan.product,
        &mut plan.corrections,
        &mut plan.others,
    ] {
        v.sort();
        v.dedup();
    }
    plan
}

/// Group letter of a file under `sources/`: `a-foo.txt`, `evidence-a.md` and
/// `capture-log-a.md` all belong to group `a`.
fn group_of(path: &str) -> Option<char> {
    let base = path.rsplit('/').next()?;
    let mut chars = base.chars();
    let (first, second) = (chars.next()?, chars.next());
    if first.is_ascii_lowercase() && second == Some('-') {
        return Some(first);
    }
    let rest = base
        .strip_prefix("evidence-")
        .or_else(|| base.strip_prefix("capture-log-"))?;
    let letter = rest.strip_suffix(".md")?;
    let mut it = letter.chars();
    match (it.next(), it.next()) {
        (Some(c), None) if c.is_ascii_lowercase() => Some(c),
        _ => None,
    }
}

/// `a- 37 个，b- 21 个，其他 3 个`: counts for filling in the ② message.
fn group_stats(files: &[String]) -> String {
    let mut groups: BTreeMap<char, usize> = BTreeMap::new();
    let mut other = 0usize;
    for f in files {
        match group_of(f) {
            Some(g) => *groups.entry(g).or_default() += 1,
            None => other += 1,
        }
    }
    let mut parts: Vec<String> = groups.iter().map(|(g, n)| format!("{g}- {n} 个")).collect();
    if other > 0 {
        parts.push(format!("其他 {other} 个"));
    }
    parts.join("，")
}

fn is_script(path: &str) -> bool {
    [".py", ".mjs", ".js", ".ps1", ".sh"]
        .iter()
        .any(|ext| path.ends_with(ext))
}

/// ① message, trimmed to the parts that are actually in the batch. A 速览专帖
/// has no 已发过 row.
fn message_repo(files: &[String], mmdd: &str, roundup: bool) -> String {
    let mut parts = Vec::new();
    if files.iter().any(|f| f == "README.md") {
        parts.push("README 期次索引");
    }
    if files.iter().any(|f| f == "daily-scan.md") {
        parts.push(if roundup {
            "daily-scan 近 3 天速览已收"
        } else {
            "daily-scan 已发过与近 3 天速览已收"
        });
    }
    if files.iter().any(|f| f.starts_with(".handoff/")) {
        parts.push("取证派工单");
    }
    format!("{mmdd}：仓库层——{}", parts.join("、"))
}

fn message_archive(files: &[String], mmdd: &str) -> String {
    let mut parts = vec!["<待填：按组描述存档内容>"];
    if files.iter().any(|f| is_script(f)) {
        parts.push("截图脚本");
    }
    let mut body = parts.join("、");
    if files.iter().any(|f| f.ends_with(offsite::MANIFEST)) {
        body.push_str("与 offsite.tsv");
    }
    format!("{mmdd}：取证存档——{body}")
}

fn message_product(mmdd: &str) -> String {
    format!(
        "{mmdd}：本期成品——发布正文、配图说明与 cards.toml、来源入口、事实核验、取证清单与抓取日志"
    )
}

fn print_files(root: &Path, files: &[String]) {
    for f in files {
        match size_of(root, f) {
            Some(s) if s > SIZE_LIMIT => {
                println!("  {f}（{}）[超过 1 MiB，须先问 Develata]", human_size(s))
            }
            Some(s) => println!("  {f}（{}）", human_size(s)),
            None => println!("  {f}（已删除）"),
        }
    }
}

struct Batch<'a> {
    n: u8,
    name: &'static str,
    files: &'a [String],
    message: String,
}

fn print_batch(root: &Path, b: &Batch) {
    if b.files.is_empty() {
        if b.n == 2 {
            println!("第②批 {}：当期没有新增取证存档，第②批省略", b.name);
        } else {
            println!("第{}批 {}：没有待提交文件", circled(b.n), b.name);
        }
        return;
    }
    let big = b
        .files
        .iter()
        .filter(|f| size_of(root, f).is_some_and(|s| s > SIZE_LIMIT))
        .count();
    let big_note = if big > 0 {
        format!("，其中 {big} 个超过 1 MiB")
    } else {
        String::new()
    };
    println!(
        "第{}批 {}（{} 个文件{big_note}）：",
        circled(b.n),
        b.name,
        b.files.len()
    );
    print_files(root, b.files);
    if b.n == 2 {
        println!("  各组前缀：{}", group_stats(b.files));
    }
    println!("  建议提交信息：{}", b.message);
}

fn circled(n: u8) -> char {
    match n {
        1 => '①',
        2 => '②',
        _ => '③',
    }
}

fn git_staged(root: &Path) -> Result<Vec<String>, String> {
    // NUL-separated: with newlines git would quote names holding odd characters.
    // --no-renames: `git status` lists a rename inside a batch as delete-old
    // plus add-new, and both are staged; with rename detection this would
    // show only the new path and the comparison in `stage` would misfire.
    let out = repo::git(
        root,
        &["diff", "--cached", "--name-only", "--no-renames", "-z"],
    )?;
    Ok(out
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect())
}

/// `git add -- <exact paths>`; literal pathspecs so a name with glob
/// characters cannot match anything else.
fn git_add(root: &Path, paths: &[String]) -> Result<(), String> {
    for chunk in chunk_by_len(paths, MAX_ARG_LEN) {
        let out = Command::new("git")
            .current_dir(root)
            .env("GIT_LITERAL_PATHSPECS", "1")
            .args(["add", "--"])
            .args(chunk)
            .output()
            .map_err(|e| format!("无法运行 git：{e}"))?;
        if !out.status.success() {
            return Err(format!(
                "git add 失败：{}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
    }
    Ok(())
}

fn stage(issue: &Issue, opts: &StageOpts) -> Result<(), String> {
    let rows = repo::git_status(&issue.root)?;
    if let Some((xy, p)) = rows
        .iter()
        .find(|(xy, _)| xy.contains('U') || xy == "AA" || xy == "DD")
    {
        return Err(format!("{p} 有未解决的合并冲突（{xy}），先处理"));
    }
    let plan = partition(issue, &rows);
    let roundup = match card::load_spec(&issue.images()) {
        Ok(s) => s.is_some_and(|s| s.kind == Kind::Roundup),
        Err(e) => {
            eprintln!("warn: {e}");
            false
        }
    };
    let mmdd = issue.mmdd.as_str();
    let batches = [
        Batch {
            n: 1,
            name: "仓库层",
            files: &plan.repo,
            message: message_repo(&plan.repo, mmdd, roundup),
        },
        Batch {
            n: 2,
            name: "取证存档",
            files: &plan.archive,
            message: message_archive(&plan.archive, mmdd),
        },
        Batch {
            n: 3,
            name: "本期成品",
            files: &plan.product,
            message: message_product(mmdd),
        },
    ];

    let Some(n) = opts.batch else {
        for b in &batches {
            print_batch(&issue.root, b);
        }
        print_leftovers(&plan);
        println!(
            "暂存某一批：barking stage {} --batch N --add（不会 commit，不会 push）",
            issue.rel()
        );
        return Ok(());
    };
    let batch = &batches[usize::from(n) - 1];
    print_batch(&issue.root, batch);
    if n == 3 && !plan.archive.is_empty() {
        println!(
            "warn: 取证存档先于成品提交（第②批还有 {} 个未提交文件）",
            plan.archive.len()
        );
    }
    if !opts.add {
        return Ok(());
    }
    if batch.files.is_empty() {
        return Ok(());
    }

    let staged = git_staged(&issue.root)?;
    if !staged.is_empty() {
        return Err(format!(
            "暂存区不干净，先提交或 git restore --staged 后再暂存：\n  {}",
            staged.join("\n  ")
        ));
    }
    if n == 1 {
        let path = issue.root.join("daily-scan.md");
        let scan = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if scan.contains(PLACEHOLDER) {
            return Err(format!(
                "daily-scan.md 仍含“{PLACEHOLDER}”占位行：推送后会污染定时检索，先按最终发布版填写"
            ));
        }
    }
    git_add(&issue.root, batch.files)?;
    let after = git_staged(&issue.root)?;
    let want: BTreeSet<&str> = batch.files.iter().map(String::as_str).collect();
    let got: BTreeSet<&str> = after.iter().map(String::as_str).collect();
    if want != got {
        let diff: Vec<&str> = want.symmetric_difference(&got).copied().collect();
        return Err(format!(
            "暂存结果与第{}批不一致：{}（已暂存的未回滚，可 git restore --staged 后重试）",
            circled(n),
            diff.join("、")
        ));
    }
    println!("已暂存第{}批 {} 个文件。", circled(n), after.len());
    println!("建议提交信息：{}", batch.message);
    println!("提交由 Claude 按实际内容改写后执行；本命令不 commit、不 push。");
    Ok(())
}

fn print_leftovers(plan: &Plan) {
    if !plan.corrections.is_empty() {
        println!(
            "CORRECTION.md 不属于三批（发布后修订单独提交）：{}",
            plan.corrections.join("、")
        );
    }
    if !plan.others.is_empty() {
        let shown: Vec<&str> = plan.others.iter().take(8).map(String::as_str).collect();
        let more = plan.others.len().saturating_sub(shown.len());
        let tail = if more > 0 {
            format!(" …另有 {more} 个")
        } else {
            String::new()
        };
        println!(
            "另有 {} 个待提交文件不属于本期，未列入三批：{}{tail}",
            plan.others.len(),
            shown.join("、")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue(yymm: &str, mmdd: &str) -> Issue {
        Issue {
            root: PathBuf::from("/repo"),
            dir: PathBuf::from(format!("/repo/docs/{yymm}/{mmdd}")),
            yymm: yymm.to_string(),
            mmdd: mmdd.to_string(),
        }
    }

    fn rows(items: &[(&str, &str)]) -> Vec<(String, String)> {
        items
            .iter()
            .map(|(x, p)| (x.to_string(), p.to_string()))
            .collect()
    }

    const README: &str = "\
# AI 吠点

## 内容目录

| 期次 | 正文 | 配图与上传顺序 | 来源与限制 |
|---|---|---|---|
| 2026 / 1008 | [甲](docs/2610/1008/doc_1008_publish.txt) | [配图说明](docs/2610/1008/images/README.md) | [来源入口](docs/2610/1008/sources/README.md) |
| 2026 / 1009 | [乙](docs/2610/1009/doc_1009_publish.txt) | [配图说明](docs/2610/1009/images/README.md) | [来源入口](docs/2610/1009/sources/README.md) |

## 公开归档范围

| 2026 / 1010 | 不在内容目录节里 |
";

    const SCAN: &str = "\
## 近 3 天速览已收

| 期次 | 标签 | 事实 |
|---|---|---|
| 1009 | Epoch | 据 Epoch 评测，甲 |
| 1009 | Codex | Codex 乙 |

## 已发过

| 期次 | 选题 |
|---|---|
| 1008 | 旧选题 |
| 1009 | Anthropic 越界报告（已填） |
";

    #[test]
    fn section_stops_at_next_heading() {
        let lines = section(README, "## 内容目录").unwrap();
        assert!(lines.iter().any(|l| l.starts_with("| 2026 / 1009 |")));
        assert!(!lines.iter().any(|l| l.contains("不在内容目录节里")));
        assert!(section(README, "## 不存在").is_none());
        // Heading match is exact, not a prefix: "## 已发过" must not open "## 已发过去".
        assert!(section("## 已发过去\n| 1009 |\n", "## 已发过").is_none());
    }

    #[test]
    fn markers_present() {
        let entries = [("Epoch", "据 Epoch 评测，甲"), ("Codex", "Codex 乙")];
        let p = marker_problems(README, SCAN, &issue("2610", "1009"), Kind::Main, &entries);
        assert!(p.is_empty(), "{p:?}");
    }

    #[test]
    fn markers_missing_are_each_reported() {
        let entries = [("Epoch", "据 Epoch 评测，甲"), ("新", "没写进去")];
        let p = marker_problems(README, SCAN, &issue("2610", "1010"), Kind::Main, &entries);
        assert_eq!(p.len(), 4, "{p:?}");
        assert!(p[0].contains("README.md") && p[0].contains("2026 / 1010"));
        assert!(p[1].contains("已发过") && p[1].contains("| 1010 |"));
        assert!(p[2].contains("| 1010 | Epoch | 据 Epoch 评测，甲 |"));
        assert!(p[3].contains("| 1010 | 新 | 没写进去 |"));
    }

    #[test]
    fn marker_text_must_match_verbatim() {
        // A near-miss (extra word) is not the row: the check is exact per line.
        let entries = [("Codex", "Codex 乙乙")];
        let p = marker_problems(README, SCAN, &issue("2610", "1009"), Kind::Main, &entries);
        assert_eq!(p.len(), 1);
        // And the row must sit in its own section, not elsewhere in the file.
        let scan = SCAN.replace("## 已发过", "## 其他");
        let p = marker_problems(README, &scan, &issue("2610", "1009"), Kind::Main, &[]);
        assert!(p.iter().any(|m| m.contains("缺少“## 已发过”")));
    }

    #[test]
    fn placeholder_row_fails_and_roundup_needs_no_row() {
        let scan = SCAN.replace("Anthropic 越界报告（已填）", "标题：待填（选题与口径摘要）");
        let p = marker_problems(README, &scan, &issue("2610", "1009"), Kind::Main, &[]);
        assert_eq!(p.len(), 1);
        assert!(p[0].contains("待填"));
        // 速览专帖: no 已发过 row is fine ...
        let p = marker_problems(README, SCAN, &issue("2610", "1008"), Kind::Roundup, &[]);
        assert!(p.is_empty(), "{p:?}");
        let scan = "## 已发过\n\n| 期次 | 选题 |\n|---|---|\n| 1007 | x |\n";
        let readme = README.replace("1009", "1007");
        let p = marker_problems(&readme, scan, &issue("2610", "1007"), Kind::Roundup, &[]);
        assert!(p.is_empty(), "{p:?}");
        // ... but a README row must still link to the issue's own text.
        let readme = README.replace(
            "(docs/2610/1009/doc_1009_publish.txt)",
            "(docs/2610/1008/doc_1008_publish.txt)",
        );
        let p = marker_problems(&readme, SCAN, &issue("2610", "1009"), Kind::Main, &[]);
        assert_eq!(p.len(), 1);
        assert!(p[0].contains("没有链接"));
    }

    #[test]
    fn png_listing_is_flat_and_png_only() {
        let dir = std::env::temp_dir().join(format!("barking-publish-test-{}", std::process::id()));
        let images = dir.join("images");
        fs::create_dir_all(images.join("sub")).unwrap();
        for f in ["b.png", "a.PNG", "c.jpg", "README.md", "sub/d.png"] {
            fs::write(images.join(f), b"x").unwrap();
        }
        let names: Vec<String> = top_level_pngs(&images)
            .unwrap()
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, ["a.PNG", "b.png"]);
        assert!(top_level_pngs(&dir.join("missing")).unwrap().is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn registered_png_is_skipped_only_on_same_hash() {
        let m = vec![("images/a.png".to_string(), 10, "abc".to_string())];
        assert!(already_registered(&m, "images/a.png", "abc"));
        assert!(!already_registered(&m, "images/a.png", "abd"));
        assert!(!already_registered(&m, "images/b.png", "abc"));
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn chunking_respects_limit_and_keeps_order() {
        let items: Vec<String> = (0..10).map(|i| format!("file-{i}")).collect();
        let chunks = chunk_by_len(&items, 30);
        assert!(chunks.len() > 1);
        assert!(chunks.iter().all(|c| !c.is_empty()));
        let joined: Vec<&String> = chunks.iter().flat_map(|c| c.iter()).collect();
        assert_eq!(joined, items.iter().collect::<Vec<_>>());
        // One oversized item still gets its own run.
        let big = vec!["x".repeat(100)];
        assert_eq!(chunk_by_len(&big, 10).len(), 1);
        assert!(chunk_by_len::<String>(&[], 10).is_empty());
    }

    #[test]
    fn candidates_drop_deleted_and_other_issues() {
        let r = rows(&[
            ("??", "docs/2610/1009/sources/a-x.txt"),
            (" D", "docs/2610/1009/sources/gone.txt"),
            ("??", "docs/2610/1008/sources/b.txt"),
            (" M", ".handoff/2026-10-09-1009-evidence.md"),
            ("??", ".claude/settings.json"),
        ]);
        let kept: Vec<String> = candidate_paths(&issue("2610", "1009"), &r)
            .into_iter()
            .collect();
        // The repo-layer files are always candidates, even when unchanged.
        assert_eq!(
            kept,
            [
                ".handoff/2026-10-09-1009-evidence.md",
                "README.md",
                "daily-scan.md",
                "docs/2610/1009/sources/a-x.txt",
            ]
        );
    }

    #[test]
    fn identity_matches_case_insensitively_without_echoing() {
        let pats = parse_patterns(" Alice ; ;@Bob_99;");
        assert_eq!(pats, ["alice", "@bob_99"]);
        let hits = identity_hits(
            "docs/2610/1009/sources/ALICE-profile.txt",
            "ok\nhandle @BOB_99 here\nnothing\n",
            &pats,
        );
        assert_eq!(
            hits,
            [
                Hit {
                    line: None,
                    pattern: 0
                },
                Hit {
                    line: Some(2),
                    pattern: 1
                }
            ]
        );
        assert!(identity_hits("a.txt", "clean", &pats).is_empty());
        assert!(parse_patterns("  ;; ").is_empty());
    }

    #[test]
    fn secret_names() {
        for bad in [
            ".env",
            ".env.local",
            "docs/2610/1009/sources/cookies.json",
            "docs/2610/1009/sources/Cookies.txt",
            "docs/2610/1009/sources/storage-state.json",
            "docs/2610/1009/sources/storage-state-x.json",
            "docs/2610/1009/.playwright-cli/page.yml",
        ] {
            assert!(is_secret_name(bad), "{bad}");
        }
        for ok in [
            ".env.example",
            "docs/2610/1009/sources/a-cookies-policy.txt",
            "docs/2610/1009/sources/storage-state-notes.md",
            "docs/2610/1009/sources/envelope.txt",
        ] {
            assert!(!is_secret_name(ok), "{ok}");
        }
    }

    fn mixed_rows() -> Vec<(String, String)> {
        rows(&[
            ("??", ".claude/settings.json"),
            (" M", "README.md"),
            (" M", "daily-scan.md"),
            ("??", ".handoff/2026-10-09-1009-evidence.md"),
            ("??", ".handoff/2026-10-08-1008-evidence.md"),
            (" M", ".handoff/README.md"),
            ("??", "docs/2610/1009/doc_1009_publish.txt"),
            ("??", "docs/2610/1009/images/README.md"),
            ("??", "docs/2610/1009/images/cards.toml"),
            ("??", "docs/2610/1009/sources/README.md"),
            ("??", "docs/2610/1009/sources/fact-check.md"),
            ("??", "docs/2610/1009/sources/evidence.md"),
            ("??", "docs/2610/1009/sources/capture-log.md"),
            ("??", "docs/2610/1009/sources/evidence-a.md"),
            ("??", "docs/2610/1009/sources/capture-log-a.md"),
            ("??", "docs/2610/1009/sources/a-shot.py"),
            ("??", "docs/2610/1009/sources/a-x.txt"),
            ("??", "docs/2610/1009/sources/b-y.json"),
            ("??", "docs/2610/1009/sources/offsite.tsv"),
            ("??", "docs/2610/1009/CORRECTION.md"),
            ("??", "docs/2610/1008/sources/z.txt"),
            ("??", "docs/2610/1005/sources/c-repo-tree.json"),
        ])
    }

    #[test]
    fn partition_sorts_files_into_three_batches() {
        let plan = partition(&issue("2610", "1009"), &mixed_rows());
        assert_eq!(
            plan.repo,
            [
                ".handoff/2026-10-09-1009-evidence.md",
                ".handoff/README.md",
                "README.md",
                "daily-scan.md"
            ]
        );
        assert_eq!(
            plan.product,
            [
                "docs/2610/1009/doc_1009_publish.txt",
                "docs/2610/1009/images/README.md",
                "docs/2610/1009/images/cards.toml",
                "docs/2610/1009/sources/README.md",
                "docs/2610/1009/sources/capture-log.md",
                "docs/2610/1009/sources/evidence.md",
                "docs/2610/1009/sources/fact-check.md",
            ]
        );
        assert_eq!(
            plan.archive,
            [
                "docs/2610/1009/sources/a-shot.py",
                "docs/2610/1009/sources/a-x.txt",
                "docs/2610/1009/sources/b-y.json",
                "docs/2610/1009/sources/capture-log-a.md",
                "docs/2610/1009/sources/evidence-a.md",
                "docs/2610/1009/sources/offsite.tsv",
            ]
        );
        assert_eq!(plan.corrections, ["docs/2610/1009/CORRECTION.md"]);
        // Other issues' files, their handoff sheet and unrelated tooling stay out.
        assert_eq!(
            plan.others,
            [
                ".claude/settings.json",
                ".handoff/2026-10-08-1008-evidence.md",
                "docs/2610/1005/sources/c-repo-tree.json",
                "docs/2610/1008/sources/z.txt",
            ]
        );
    }

    #[test]
    fn partition_ignores_files_of_other_issues_with_same_names() {
        // Product-file names only count inside this issue's directory.
        let r = rows(&[
            ("??", "docs/2610/1008/doc_1008_publish.txt"),
            ("??", "docs/2610/1009/sources/doc_1008_publish.txt"),
            ("??", "docs/2610/10090/doc_1009_publish.txt"),
        ]);
        let plan = partition(&issue("2610", "1009"), &r);
        assert!(plan.product.is_empty());
        assert_eq!(
            plan.archive,
            ["docs/2610/1009/sources/doc_1008_publish.txt"]
        );
        assert_eq!(plan.others.len(), 2);
    }

    #[test]
    fn handoff_matching() {
        assert!(is_handoff(".handoff/2026-10-09-1009-evidence.md", "1009"));
        assert!(is_handoff(".handoff/2026-09-30-0930-scan.md", "0930"));
        assert!(!is_handoff(".handoff/README.md", "1009"));
        assert!(!is_handoff(".handoff/2026-10-08-1008-evidence.md", "1009"));
        assert!(!is_handoff(".handoff/2026-10-09-1009-evidence.txt", "1009"));
        assert!(!is_handoff(".handoff/sub/2026-10-09-1009-x.md", "1009"));
        assert!(!is_handoff("docs/.handoff/2026-10-09-1009-x.md", "1009"));
    }

    #[test]
    fn group_prefix_stats() {
        let files: Vec<String> = [
            "docs/2610/1009/sources/a-1.txt",
            "docs/2610/1009/sources/a-2.txt",
            "docs/2610/1009/sources/b-1.txt",
            "docs/2610/1009/sources/evidence-a.md",
            "docs/2610/1009/sources/capture-log-b.md",
            "docs/2610/1009/sources/offsite.tsv",
            "docs/2610/1009/sources/m-raw-crops.py",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(group_stats(&files), "a- 3 个，b- 2 个，m- 1 个，其他 1 个");
        assert_eq!(group_of("x/evidence-ab.md"), None);
        assert_eq!(group_of("x/evidence.md"), None);
        assert_eq!(group_of("x/-a.txt"), None);
    }

    #[test]
    fn commit_messages() {
        let plan = partition(&issue("2610", "1009"), &mixed_rows());
        assert_eq!(
            message_repo(&plan.repo, "1009", false),
            "1009：仓库层——README 期次索引、daily-scan 已发过与近 3 天速览已收、取证派工单"
        );
        assert_eq!(
            message_repo(&plan.repo, "1009", true),
            "1009：仓库层——README 期次索引、daily-scan 近 3 天速览已收、取证派工单"
        );
        assert_eq!(
            message_repo(&["README.md".to_string()], "1009", false),
            "1009：仓库层——README 期次索引"
        );
        assert_eq!(
            message_archive(&plan.archive, "1009"),
            "1009：取证存档——<待填：按组描述存档内容>、截图脚本与 offsite.tsv"
        );
        assert_eq!(
            message_archive(&["docs/2610/1009/sources/a-x.txt".to_string()], "1009"),
            "1009：取证存档——<待填：按组描述存档内容>"
        );
        assert!(
            message_product("1009").starts_with("1009：本期成品——发布正文、配图说明与 cards.toml")
        );
    }

    #[test]
    fn manifest_rows_missing_locally_are_reported() {
        let manifest = "# header\n\
images/00-cover.png\t10\taa\n\
sources/a-page.html\t20\tbb\n\
images/01-shot.png\t30\tcc\n\
images/00-cover.png\t10\taa\n";
        let old = offsite::parse_manifest(manifest);
        // Partly fetched back: only the HTML original is here.
        let present = BTreeSet::from(["sources/a-page.html", "images/02-new.png"]);
        assert_eq!(
            missing_offsite(&old, &present),
            ["images/00-cover.png", "images/01-shot.png"],
            "manifest order, each path once; new local files are not missing"
        );
        // A clone with no originals at all loses every row.
        assert_eq!(missing_offsite(&old, &BTreeSet::new()).len(), 3);
        // Everything present, or no manifest yet: nothing to report.
        let all = BTreeSet::from([
            "images/00-cover.png",
            "sources/a-page.html",
            "images/01-shot.png",
        ]);
        assert!(missing_offsite(&old, &all).is_empty());
        assert!(missing_offsite(&[], &BTreeSet::new()).is_empty());
    }

    #[test]
    fn stage_args() {
        let a = |v: &[&str]| parse_stage_args(&v.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        let o = a(&["docs/2610/1009"]).unwrap();
        assert_eq!((o.batch, o.add), (None, false));
        let o = a(&["--batch", "3", "docs/2610/1009", "--add"]).unwrap();
        assert_eq!((o.batch, o.add), (Some(3), true));
        assert!(a(&[]).is_err());
        assert!(a(&["d", "--add"]).is_err(), "--add needs --batch");
        assert!(a(&["d", "--batch", "4"]).is_err());
        assert!(a(&["d", "--batch"]).is_err());
        assert!(a(&["d", "e"]).is_err());
        assert!(a(&["d", "--force"]).is_err());
    }
}
