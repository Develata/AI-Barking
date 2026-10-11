//! `barking review`: assemble the cross-model review of an issue (EDITORIAL.md
//! 核验流程 step 5, templates/cards/README.md step 5) and, with `--run`,
//! hand it to the WSL Codex.
//!
//! This module only does the mechanical part: fill the prompt template, pick
//! the images, convert paths for WSL and build the command line. Reading the
//! report, judging each finding and recording the verdict in `fact-check.md`
//! stay with Claude; the reviewer must be independent, so nothing about the
//! expected outcome goes into the prompt.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use crate::card::{self, Kind, Spec};
use crate::offsite;
use crate::repo::Issue;

const VISUAL_TEMPLATE: &str = include_str!("../../../templates/cards/visual-review.md");
const REVERSE_TEMPLATE: &str = include_str!("../../../templates/reverse-review.md");
const USAGE: &str = "用法：barking review <期次目录> (--visual | --reverse) [--effort medium|high|xhigh|max] [--run]";

/// The strongest GPT model; the reviewer must be a different family from the
/// drafting model (~/.claude/agents/codex-reviewer.md).
const MODEL: &str = "gpt-6-astra";
const EFFORTS: [&str; 4] = ["medium", "high", "xhigh", "max"];
/// `medium` runs are capped at 20 minutes; above that Codex may research for
/// hours, so there is no cap (`timeout 0`).
const MEDIUM_TIMEOUT_SECS: u32 = 1200;
const DEFAULT_DISTRO: &str = "Debian";
const DEFAULT_CODEX: &str = "/home/deve/.bun/bin/codex";
/// Lines of the report echoed after `--run`.
const REPORT_PREVIEW_LINES: usize = 40;
/// EDITORIAL.md sections the visual reviewer needs as its rule book.
const VISUAL_RULE_SECTIONS: [&str; 2] = ["省流卡与批注截图", "速览"];
/// Tracked files with these extensions are text archives the reverse
/// reviewer may read; everything else under an issue is binary or offsite.
const TEXT_EXTS: [&str; 13] = [
    "md", "txt", "json", "jsonl", "toml", "tsv", "csv", "py", "mjs", "js", "ps1", "sh", "yaml",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Visual,
    Reverse,
}

impl Mode {
    /// File-name stem of the prompt and report.
    fn name(self) -> &'static str {
        match self {
            Mode::Visual => "visual",
            Mode::Reverse => "reverse",
        }
    }

    /// The heading under which the verdict is recorded in `fact-check.md`.
    fn heading(self) -> &'static str {
        match self {
            Mode::Visual => "视觉复核",
            Mode::Reverse => "反向核验",
        }
    }
}

#[derive(Debug)]
struct Opts {
    dir: PathBuf,
    mode: Mode,
    effort: String,
    run: bool,
}

pub fn run(args: &[String]) -> ExitCode {
    let opts = match parse_args(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    match execute(&opts) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args(args: &[String]) -> Result<Opts, String> {
    let (mut visual, mut reverse, mut run) = (false, false, false);
    let mut effort = String::from("medium");
    let mut dir: Option<PathBuf> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--visual" => visual = true,
            "--reverse" => reverse = true,
            "--run" => run = true,
            "--effort" => effort = it.next().ok_or("--effort 缺少取值")?.clone(),
            s if s.starts_with("--effort=") => effort = s["--effort=".len()..].to_string(),
            s if s.starts_with("--") => return Err(format!("未知选项 {s}")),
            _ => {
                if dir.replace(PathBuf::from(a)).is_some() {
                    return Err("只能给一个期次目录".into());
                }
            }
        }
    }
    let mode = match (visual, reverse) {
        (true, false) => Mode::Visual,
        (false, true) => Mode::Reverse,
        (false, false) => return Err("须指定 --visual 或 --reverse 之一".into()),
        (true, true) => return Err("--visual 与 --reverse 不能同时用，分两次运行".into()),
    };
    if !EFFORTS.contains(&effort.as_str()) {
        return Err(format!(
            "--effort 须为 {}，得到「{effort}」",
            EFFORTS.join("、")
        ));
    }
    let dir = dir.ok_or("缺少期次目录")?;
    Ok(Opts {
        dir,
        mode,
        effort,
        run,
    })
}

/// One image handed to the visual reviewer.
#[derive(Debug, PartialEq, Eq)]
struct Attachment {
    /// Path relative to the issue directory, `/`-separated.
    file: String,
    label: &'static str,
}

/// What `execute` needs from a mode: the prompt text, the images, the
/// directory Codex works in and anything worth telling the user.
struct Review {
    prompt: String,
    images: Vec<String>,
    /// Codex's `-C` directory: the issue for `--visual`, the repo root for
    /// `--reverse` (it reads EDITORIAL.md and the other issues' conventions).
    cwd: PathBuf,
    search: bool,
    warnings: Vec<String>,
    /// One line for the summary, e.g. `附图 14 张`.
    scope: String,
}

fn execute(opts: &Opts) -> Result<ExitCode, String> {
    let issue = Issue::resolve(&opts.dir)?;
    let review = match opts.mode {
        Mode::Visual => plan_visual(&issue)?,
        Mode::Reverse => plan_reverse(&issue)?,
    };
    for img in &review.images {
        if !safe_arg(img) {
            return Err(format!(
                "图片名「{img}」含需要转义的字符，改用字母、数字与 . _ - /"
            ));
        }
    }

    // Prompt and report stay out of the repo (reports are drafts, see
    // fact-check.md "未入库"); a finished report is never overwritten.
    let work = std::env::temp_dir()
        .join("barking-review")
        .join(&issue.mmdd);
    fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let serial = pick_serial(&work, opts.mode);
    let (prompt_name, report_name) = artifact_names(opts.mode, serial);
    let (prompt_path, report_path) = (work.join(prompt_name), work.join(report_name));
    fs::write(&prompt_path, &review.prompt)
        .map_err(|e| format!("{}: {e}", prompt_path.display()))?;

    let distro = std::env::var("BARKING_WSL_DISTRO").unwrap_or_else(|_| DEFAULT_DISTRO.into());
    let codex = std::env::var("BARKING_WSL_CODEX").unwrap_or_else(|_| DEFAULT_CODEX.into());
    let invocation = Invocation {
        distro: distro.clone(),
        codex,
        cwd: wslpath(&distro, &review.cwd)?,
        report: wslpath(&distro, &report_path)?,
        prompt: wslpath(&distro, &prompt_path)?,
        effort: opts.effort.clone(),
        secs: timeout_secs(&opts.effort),
        images: review.images.clone(),
        search: review.search,
    };

    println!("== {}：{}", opts.mode.heading(), issue.rel());
    for w in &review.warnings {
        println!("warn: {w}");
    }
    println!(
        "提示词：{}（{} 字符）",
        prompt_path.display(),
        review.prompt.chars().count()
    );
    println!("报告：{}", report_path.display());
    println!("{}", review.scope);
    println!(
        "模型 {MODEL}，effort {}，{}",
        opts.effort,
        match invocation.secs {
            0 => "不限时".to_string(),
            s => format!("限时 {s} 秒"),
        }
    );
    println!("命令（可直接粘到 Git Bash 运行）：");
    println!("{}", invocation.display());
    println!(
        "-- 逐条核实报告意见后，结论记入 {}/sources/fact-check.md 的“{}”节",
        issue.rel(),
        opts.mode.heading()
    );

    if !opts.run {
        return Ok(ExitCode::SUCCESS);
    }
    run_and_report(&invocation, &report_path)
}

/// Run the command in the foreground (Codex's own output passes through),
/// then show how it ended and the head of the report.
fn run_and_report(inv: &Invocation, report: &Path) -> Result<ExitCode, String> {
    println!("-- 开始运行（同步等待；Codex 通常需要数分钟）");
    let started = Instant::now();
    let status = Command::new("wsl")
        .env("MSYS_NO_PATHCONV", "1")
        .args(inv.argv())
        .stdin(Stdio::null())
        .status()
        .map_err(|e| format!("无法运行 wsl：{e}"))?;
    let code = status.code().unwrap_or(-1);
    println!(
        "-- 退出码 {code}{}，用时 {}",
        if code == 124 { "（超时）" } else { "" },
        format_elapsed(started.elapsed().as_secs())
    );
    println!("-- 报告：{}", report.display());

    let text = fs::read(report)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    if text.trim().is_empty() {
        println!("报告不存在或为空：Codex 不可用或未完成，没有做成独立复核");
        return Ok(ExitCode::FAILURE);
    }
    let total = text.lines().count();
    for line in text.lines().take(REPORT_PREVIEW_LINES) {
        println!("{line}");
    }
    if total > REPORT_PREVIEW_LINES {
        println!("…（共 {total} 行，其余见报告文件）");
    }
    Ok(if code == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn format_elapsed(secs: u64) -> String {
    if secs >= 60 {
        format!("{} 分 {} 秒", secs / 60, secs % 60)
    } else {
        format!("{secs} 秒")
    }
}

fn timeout_secs(effort: &str) -> u32 {
    if effort == "medium" {
        MEDIUM_TIMEOUT_SECS
    } else {
        0
    }
}

// ---- prompts ----------------------------------------------------------

fn plan_visual(issue: &Issue) -> Result<Review, String> {
    let images = issue.images();
    let spec = card::load_spec(&images)?.ok_or_else(|| {
        format!(
            "{} 不存在，没有可复核的卡片",
            images.join(card::SPEC_FILE).display()
        )
    })?;
    let (attachments, warnings) = attachments(&spec, |rel| issue.dir.join(rel).is_file());
    if attachments.is_empty() {
        return Err(format!(
            "{} 下一张要复核的图都不存在（图片不入库：先从 OpenList 取回，或在有图片的工作树里运行）",
            images.display()
        ));
    }
    let mut warnings = warnings;
    if !warnings.is_empty() && issue.sources().join(offsite::MANIFEST).is_file() {
        warnings.push("缺的图片若登记在 sources/offsite.tsv，可从 OpenList 同一路径取回".into());
    }
    let editorial = read_lf(&issue.root.join("EDITORIAL.md"))?;
    let prompt = visual_prompt(&issue.rel(), &attachments, &editorial)?;
    Ok(Review {
        prompt,
        scope: format!("附图：{} 张", attachments.len()),
        images: attachments.into_iter().map(|a| a.file).collect(),
        cwd: issue.dir.clone(),
        search: false,
        warnings,
    })
}

fn plan_reverse(issue: &Issue) -> Result<Review, String> {
    let files = text_files(issue)?;
    if files.is_empty() {
        return Err(format!("{} 下没有可审的文本文件", issue.dir.display()));
    }
    let mut warnings = Vec::new();
    for required in [
        format!("doc_{}_publish.txt", issue.mmdd),
        "sources/fact-check.md".to_string(),
    ] {
        if !files.contains(&required) {
            warnings.push(format!("{required} 不存在，复核者会找不到它"));
        }
    }
    let prompt = reverse_prompt(&issue.rel(), &issue.mmdd, &files)?;
    Ok(Review {
        prompt,
        scope: format!("审查范围：{} 个文本文件，不带图片", files.len()),
        images: Vec::new(),
        cwd: issue.root.clone(),
        search: true,
        warnings,
    })
}

/// The part of a template after its first `---` line; above it is the
/// human-facing note on how the template is used.
fn template_body(template: &str) -> Result<String, String> {
    let text = template.replace("\r\n", "\n");
    let mut lines = text.split_inclusive('\n');
    let mut found = false;
    for l in lines.by_ref() {
        if l.trim_end() == "---" {
            found = true;
            break;
        }
    }
    if !found {
        return Err("提示词模板里找不到 --- 分隔线".into());
    }
    Ok(lines.collect::<String>().trim().to_string())
}

/// Placeholders the templates may use; a leftover after filling means the
/// template grew a placeholder this code does not know.
const PLACEHOLDERS: [&str; 4] = ["{期次目录}", "{附图清单", "{MMDD}", "{文件清单}"];

fn check_filled(body: &str) -> Result<(), String> {
    match PLACEHOLDERS.iter().find(|p| body.contains(*p)) {
        Some(p) => Err(format!("提示词里还剩占位符 {p}")),
        None => Ok(()),
    }
}

/// Fill `{期次目录}` and the whole `{附图清单…}` placeholder (its text after
/// the name is an example the template shows to humans), then append the
/// EDITORIAL.md sections the reviewer judges by.
fn visual_prompt(
    issue_rel: &str,
    attachments: &[Attachment],
    editorial: &str,
) -> Result<String, String> {
    let body = template_body(VISUAL_TEMPLATE)?.replace("{期次目录}", issue_rel);
    let start = body
        .find("{附图清单")
        .ok_or("视觉复核模板缺少 {附图清单…} 占位符")?;
    let end = body[start..]
        .find('}')
        .map(|i| start + i + 1)
        .ok_or("视觉复核模板的 {附图清单…} 缺少右花括号")?;
    let body = format!(
        "{}{}{}",
        &body[..start],
        attachment_list(attachments),
        &body[end..]
    );
    check_filled(&body)?;

    let mut out = body;
    out.push_str("\n\n---\n\n以下是 EDITORIAL.md 中与卡片有关的两节原文（规则依据）：\n");
    for title in VISUAL_RULE_SECTIONS {
        let sec = section(editorial, title)
            .ok_or_else(|| format!("EDITORIAL.md 里找不到“## {title}”一节"))?;
        let _ = write!(out, "\n{sec}\n");
    }
    Ok(out)
}

fn reverse_prompt(issue_rel: &str, mmdd: &str, files: &[String]) -> Result<String, String> {
    let list: String = files.iter().map(|f| format!("- {f}\n")).collect();
    let body = template_body(REVERSE_TEMPLATE)?
        .replace("{期次目录}", issue_rel)
        .replace("{MMDD}", mmdd)
        .replace("{文件清单}", list.trim_end());
    check_filled(&body)?;
    Ok(format!("{body}\n"))
}

fn attachment_list(attachments: &[Attachment]) -> String {
    attachments
        .iter()
        .map(|a| format!("{}（{}）", a.file, a.label))
        .collect::<Vec<_>>()
        .join("；")
}

/// The `## <title>` section of a markdown document, heading included, up to
/// the next `## ` heading. A heading may carry a date note after the title
/// (`## 速览（2026-10-04 起）`).
fn section(doc: &str, title: &str) -> Option<String> {
    // CRLF would leave a stray `\r` on every line; `lines()` strips it.
    let mut out: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in doc.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            if inside {
                break;
            }
            let name = h.split('（').next().unwrap_or(h).trim();
            if name == title {
                inside = true;
            }
        }
        if inside {
            out.push(line);
        }
    }
    (!out.is_empty()).then(|| out.join("\n").trim_end().to_string())
}

fn read_lf(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map(|t| t.replace("\r\n", "\n"))
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// Images the visual reviewer should see, in review order: 省流卡, 速览图,
/// covers, then each annotated screenshot followed by its `-raw` base (the
/// reviewer compares the two to confirm no source pixel was altered).
/// `exists` answers for issue-relative paths. Missing images are reported,
/// not silently dropped: a review that skipped one is not a full review.
fn attachments(spec: &Spec, exists: impl Fn(&str) -> bool) -> (Vec<Attachment>, Vec<String>) {
    let outputs = spec.outputs();
    let (mut out, mut warns) = (Vec::new(), Vec::new());
    // Attaches `images/<name>` if it exists; says whether it did.
    let mut add = |name: &str, label: &'static str| {
        let file = format!("images/{name}");
        let found = exists(&file);
        if found {
            out.push(Attachment { file, label });
        }
        found
    };
    let missing = |name: &str| format!("images/{name} 不存在，未附（复核不完整）");
    if outputs.contains(&card::TLDR_OUT) && !add(card::TLDR_OUT, "省流卡") {
        warns.push(missing(card::TLDR_OUT));
    }
    if outputs.contains(&card::ROUNDUP_OUT) && !add(card::ROUNDUP_OUT, "速览聚合图") {
        warns.push(missing(card::ROUNDUP_OUT));
    }
    // A 速览专帖 renders its covers (so they are required); a main post's
    // covers are generated images that exist only once picked.
    let template_covers = spec.kind == Kind::Roundup;
    let (cover, wide) = if template_covers {
        ("模板封面", "模板封面（宽）")
    } else {
        ("封面", "宽封面")
    };
    for (name, label) in [(card::COVER_OUT, cover), (card::COVER_WIDE_OUT, wide)] {
        if !add(name, label) && template_covers {
            warns.push(missing(name));
        }
    }
    for a in &spec.annot {
        if !add(&a.out, "批注图") {
            warns.push(missing(&a.out));
            continue;
        }
        let raw = format!("{}-raw.png", a.out.strip_suffix(".png").unwrap_or(&a.out));
        if !add(&raw, "该批注图的原始底图") {
            warns.push(format!(
                "images/{raw} 不存在：复核者无法核对 {} 有没有改动原文像素",
                a.out
            ));
        }
    }
    (out, warns)
}

/// Tracked and not-yet-committed text files of the issue, relative to its
/// directory. Review happens before the commit, so untracked files count;
/// gitignored images, PDFs and page archives do not (they are offsite).
fn text_files(issue: &Issue) -> Result<Vec<String>, String> {
    let prefix = format!("{}/", issue.rel());
    let mut files: Vec<String> = offsite::git_ls_files(
        &issue.root,
        &["--cached", "--others", "--exclude-standard"],
        &prefix,
    )?
    .into_iter()
    .filter_map(|p| p.strip_prefix(&prefix).map(str::to_string))
    .filter(|rel| is_text_file(rel) && issue.dir.join(rel).is_file())
    .collect();
    files.sort();
    files.dedup();
    Ok(files)
}

fn is_text_file(rel: &str) -> bool {
    Path::new(rel)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| TEXT_EXTS.contains(&e.to_ascii_lowercase().as_str()))
}

// ---- prompt and report files ------------------------------------------

/// Smallest `n >= 1` whose report does not exist yet. The report is the
/// output of a run that may have taken an hour, so it is never overwritten;
/// the prompt of the same serial is regenerated freely.
fn pick_serial(dir: &Path, mode: Mode) -> u32 {
    (1..)
        .find(|&n| !dir.join(artifact_names(mode, n).1).exists())
        .unwrap_or(1)
}

/// `(prompt, report)` file names; serial 1 carries no suffix.
fn artifact_names(mode: Mode, serial: u32) -> (String, String) {
    let m = mode.name();
    if serial <= 1 {
        (format!("{m}-prompt.md"), format!("{m}-report.md"))
    } else {
        (
            format!("{m}-prompt-{serial}.md"),
            format!("{m}-report-{serial}.md"),
        )
    }
}

// ---- WSL command ------------------------------------------------------

/// Everything the `wsl … bash -lc` line needs; paths are already WSL paths.
/// They travel as positional arguments (`$0`…`$5`), never spliced into the
/// script, so spaces and quotes in them cannot break the command.
struct Invocation {
    distro: String,
    codex: String,
    cwd: String,
    report: String,
    prompt: String,
    effort: String,
    secs: u32,
    /// Relative to `cwd`.
    images: Vec<String>,
    /// Web search for external facts; a top-level `codex` flag.
    search: bool,
}

impl Invocation {
    /// The bash script. Codex reads the prompt from stdin (`-`); `timeout 0`
    /// means no limit. `--image=<file>` stays in the attached form so the
    /// trailing `-` is not swallowed as another image.
    fn script(&self) -> String {
        let mut s = String::from(r#"cd "$1" && exec timeout "$5" "$0" "#);
        if self.search {
            s.push_str("--search ");
        }
        let _ = write!(
            s,
            r#"exec -m {MODEL} -c "model_reasoning_effort=\"$4\"" -s read-only --skip-git-repo-check --ephemeral -C "$1" -o "$2" "#
        );
        for img in &self.images {
            let _ = write!(s, "--image={img} ");
        }
        s.push_str(r#"- < "$3""#);
        s
    }

    /// Arguments after `wsl`.
    fn argv(&self) -> Vec<String> {
        [
            "-d",
            &self.distro,
            "-e",
            "bash",
            "-lc",
            &self.script(),
            &self.codex,
            &self.cwd,
            &self.report,
            &self.prompt,
            &self.effort,
            &self.secs.to_string(),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// The same command as one line for Git Bash. `MSYS_NO_PATHCONV` keeps
    /// Git Bash from rewriting the `/mnt/…` arguments.
    fn display(&self) -> String {
        let args: Vec<String> = self.argv().iter().map(|a| sh_quote(a)).collect();
        format!("MSYS_NO_PATHCONV=1 wsl {}", args.join(" "))
    }
}

/// Characters that need no quoting in a POSIX shell word.
fn plain_word(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_./:@%+=,-".contains(&b))
}

/// Argument that may sit unquoted inside the generated script.
fn safe_arg(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._/-".contains(&b))
}

fn sh_quote(s: &str) -> String {
    if plain_word(s) {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\'', r"'\''"))
    }
}

/// `wsl -d <distro> -e wslpath -a <path>`. The Windows form (`E:\…`) must be
/// passed: Git Bash forms (`/e/…`, `/mnt/…`) do not convert.
fn wslpath(distro: &str, path: &Path) -> Result<String, String> {
    let win = path
        .to_str()
        .ok_or_else(|| format!("{}: 路径不是 UTF-8", path.display()))?;
    let win = win.trim_start_matches(r"\\?\");
    let out = Command::new("wsl")
        .env("MSYS_NO_PATHCONV", "1")
        .args(["-d", distro, "-e", "wslpath", "-a", win])
        .output()
        .map_err(|e| format!("无法运行 wsl：{e}"))?;
    let text = |b: &[u8]| {
        String::from_utf8_lossy(b)
            .replace('\0', "")
            .trim()
            .to_string()
    };
    let linux = text(&out.stdout);
    if !out.status.success() || linux.is_empty() {
        return Err(format!(
            "wslpath 转换 {win} 失败（发行版 {distro}，可用环境变量 BARKING_WSL_DISTRO 指定）：{}",
            text(&out.stderr)
        ));
    }
    Ok(linux)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(toml_text: &str) -> Spec {
        toml::from_str(toml_text).unwrap()
    }

    fn main_spec() -> Spec {
        spec(
            r#"
date = "10月9日"
[[tldr]]
tag = "A"
fact = "a"
[[roundup]]
tag = "A"
main = true
text = "a"
[[annot]]
out = "41-x.png"
title = "t"
source = "s"
[[annot.note]]
quote = "q"
gloss = "g"
[[annot]]
out = "42-y.png"
title = "t"
source = "s"
[[annot.note]]
quote = "q"
gloss = "g"
"#,
        )
    }

    fn present(files: &[&str]) -> impl Fn(&str) -> bool {
        let set: Vec<String> = files.iter().map(|s| s.to_string()).collect();
        move |rel| set.iter().any(|f| f == rel)
    }

    fn files(att: &[Attachment]) -> Vec<&str> {
        att.iter().map(|a| a.file.as_str()).collect()
    }

    #[test]
    fn args_need_exactly_one_mode_and_one_directory() {
        let a = |v: &[&str]| parse_args(&v.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        let ok = a(&["docs/2610/1009", "--visual"]).unwrap();
        assert_eq!(ok.mode, Mode::Visual);
        assert_eq!(ok.effort, "medium");
        assert!(!ok.run);
        let ok = a(&["--reverse", "d", "--effort", "max", "--run"]).unwrap();
        assert_eq!(
            (ok.mode, ok.effort.as_str(), ok.run),
            (Mode::Reverse, "max", true)
        );
        assert_eq!(
            a(&["d", "--visual", "--effort=high"]).unwrap().effort,
            "high"
        );
        assert!(a(&["d"]).is_err());
        assert!(a(&["d", "--visual", "--reverse"]).is_err());
        assert!(a(&["--visual"]).is_err());
        assert!(a(&["d", "e", "--visual"]).is_err());
        assert!(a(&["d", "--visual", "--effort", "low"]).is_err());
        assert!(a(&["d", "--visual", "--effort"]).is_err());
        assert!(a(&["d", "--visual", "--frobnicate"]).is_err());
    }

    #[test]
    fn timeout_only_for_medium() {
        assert_eq!(timeout_secs("medium"), 1200);
        for e in ["high", "xhigh", "max"] {
            assert_eq!(timeout_secs(e), 0);
        }
    }

    #[test]
    fn template_body_starts_after_the_rule() {
        let body = template_body("# 标题\n说明\n\n---\n\n正文一\n正文二\n").unwrap();
        assert_eq!(body, "正文一\n正文二");
        assert!(template_body("没有分隔线").is_err());
        // The real templates split too, and keep their placeholders.
        assert!(
            template_body(VISUAL_TEMPLATE)
                .unwrap()
                .contains("{期次目录}")
        );
        assert!(
            template_body(REVERSE_TEMPLATE)
                .unwrap()
                .contains("{文件清单}")
        );
    }

    #[test]
    fn attachments_follow_review_order_with_raw_bases() {
        let all = [
            "images/00-tldr.png",
            "images/00-roundup.png",
            "images/00-cover.png",
            "images/00-cover-wide.png",
            "images/41-x.png",
            "images/41-x-raw.png",
            "images/42-y.png",
            "images/42-y-raw.png",
        ];
        let (att, warns) = attachments(&main_spec(), present(&all));
        assert_eq!(files(&att), all);
        assert!(warns.is_empty(), "{warns:?}");
        assert_eq!(att[0].label, "省流卡");
        assert_eq!(att[1].label, "速览聚合图");
        assert_eq!(att[2].label, "封面");
        assert_eq!(att[4].label, "批注图");
        assert_eq!(att[5].label, "该批注图的原始底图");
    }

    #[test]
    fn attachments_report_missing_images_but_skip_absent_covers() {
        // No covers (not required for a main post), 42's image and 41's base
        // are missing.
        let have = [
            "images/00-tldr.png",
            "images/00-roundup.png",
            "images/41-x.png",
        ];
        let (att, warns) = attachments(&main_spec(), present(&have));
        assert_eq!(files(&att), have);
        assert_eq!(warns.len(), 2, "{warns:?}");
        assert!(warns[0].contains("41-x-raw.png"));
        assert!(warns[1].contains("42-y.png"));
        // Nothing is invented for an empty directory.
        let (att, warns) = attachments(&main_spec(), present(&[]));
        assert!(att.is_empty());
        assert_eq!(warns.len(), 4); // tldr, roundup, two annotated images
    }

    #[test]
    fn roundup_issue_requires_its_template_covers() {
        let s = spec(
            r#"
kind = "roundup"
date = "d"
[[roundup]]
tag = "A"
text = "a"
"#,
        );
        let (att, warns) = attachments(&s, present(&["images/00-roundup.png"]));
        assert_eq!(files(&att), ["images/00-roundup.png"]);
        assert_eq!(warns.len(), 2, "{warns:?}"); // both covers missing
        let all = [
            "images/00-cover.png",
            "images/00-cover-wide.png",
            "images/00-roundup.png",
        ];
        let (att, warns) = attachments(&s, present(&all));
        assert!(warns.is_empty());
        assert_eq!(att[1].label, "模板封面");
        assert_eq!(att[2].label, "模板封面（宽）");
        assert_eq!(att[0].label, "速览聚合图");
    }

    #[test]
    fn attachment_list_pairs_file_and_label() {
        let att = vec![
            Attachment {
                file: "images/00-tldr.png".into(),
                label: "省流卡",
            },
            Attachment {
                file: "images/41-x-raw.png".into(),
                label: "该批注图的原始底图",
            },
        ];
        assert_eq!(
            attachment_list(&att),
            "images/00-tldr.png（省流卡）；images/41-x-raw.png（该批注图的原始底图）"
        );
    }

    const EDITORIAL: &str = "# T\n\n## 封面\n封面规则\n\n## 省流卡与批注截图（2026-10-02 起）\n卡规则一\n\n- 条目\n\n## 速览（2026-10-04 起）\n速览规则\n\n## 事实分级\n分级\n";

    #[test]
    fn section_is_cut_at_the_next_heading() {
        assert_eq!(
            section(EDITORIAL, "速览").unwrap(),
            "## 速览（2026-10-04 起）\n速览规则"
        );
        assert_eq!(
            section(EDITORIAL, "省流卡与批注截图").unwrap(),
            "## 省流卡与批注截图（2026-10-02 起）\n卡规则一\n\n- 条目"
        );
        assert!(section(EDITORIAL, "不存在").is_none());
        // A title is matched whole, not as a prefix.
        assert!(section("## 速览专帖\nx\n", "速览").is_none());
    }

    #[test]
    fn visual_prompt_fills_every_placeholder() {
        let att = vec![
            Attachment {
                file: "images/00-tldr.png".into(),
                label: "省流卡",
            },
            Attachment {
                file: "images/41-x.png".into(),
                label: "批注图",
            },
        ];
        let p = visual_prompt("docs/2610/1009", &att, EDITORIAL).unwrap();
        assert!(p.contains("期次目录：`docs/2610/1009`"), "{p}");
        assert!(p.contains("附图依次为：images/00-tldr.png（省流卡）；images/41-x.png（批注图）"));
        // The human-facing example inside the placeholder is gone.
        assert!(!p.contains("images/01-x-raw.png"));
        for ph in PLACEHOLDERS {
            assert!(!p.contains(ph), "{ph} left in prompt");
        }
        // Rule sections are appended verbatim, in order.
        let at = |s: &str| p.find(s).unwrap();
        assert!(at("## 省流卡与批注截图") < at("## 速览"));
        assert!(p.contains("卡规则一"));
        assert!(!p.contains("事实分级"));
        // The reviewer's checklist survived the cut.
        assert!(p.contains("逐张检查"));
        // A document without the sections is an error, not a silent omission.
        assert!(visual_prompt("docs/2610/1009", &att, "# T\n").is_err());
    }

    #[test]
    fn reverse_prompt_fills_every_placeholder() {
        let files = vec![
            "doc_1009_publish.txt".to_string(),
            "images/cards.toml".to_string(),
            "sources/fact-check.md".to_string(),
        ];
        let p = reverse_prompt("docs/2610/1009", "1009", &files).unwrap();
        for ph in PLACEHOLDERS {
            assert!(!p.contains(ph), "{ph} left in prompt");
        }
        assert!(p.contains("`doc_1009_publish.txt`"));
        assert!(p.contains("本期目录是 `docs/2610/1009`"));
        assert!(
            p.contains("- doc_1009_publish.txt\n- images/cards.toml\n- sources/fact-check.md\n")
        );
        // The charter opens the prompt, word for word.
        assert!(p.starts_with("You are an independent adversarial reviewer. Assume the artifact is wrong until proven otherwise."));
        assert!(p.contains("PASS / no blocker"));
        assert!(p.ends_with('\n') && !p.ends_with("\n\n"));
        // No generic `{Xxx}` placeholder is left either.
        assert!(!p.contains("{MMDD"), "{p}");
    }

    fn invocation(images: &[&str], search: bool, effort: &str) -> Invocation {
        Invocation {
            distro: "Debian".into(),
            codex: "/home/deve/.bun/bin/codex".into(),
            cwd: "/mnt/e/gitclone/AI-Barking/docs/2610/1009".into(),
            report: "/mnt/c/Users/QQ/AppData/Local/Temp/barking-review/1009/visual-report.md"
                .into(),
            prompt: "/mnt/c/Users/QQ/AppData/Local/Temp/barking-review/1009/visual-prompt.md"
                .into(),
            effort: effort.into(),
            secs: timeout_secs(effort),
            images: images.iter().map(|s| s.to_string()).collect(),
            search,
        }
    }

    #[test]
    fn visual_command_has_one_image_flag_per_file_and_no_search() {
        let inv = invocation(
            &["images/00-tldr.png", "images/41-x-raw.png"],
            false,
            "medium",
        );
        assert_eq!(
            inv.script(),
            r#"cd "$1" && exec timeout "$5" "$0" exec -m gpt-6-astra -c "model_reasoning_effort=\"$4\"" -s read-only --skip-git-repo-check --ephemeral -C "$1" -o "$2" --image=images/00-tldr.png --image=images/41-x-raw.png - < "$3""#
        );
        let argv = inv.argv();
        assert_eq!(&argv[..5], ["-d", "Debian", "-e", "bash", "-lc"]);
        assert_eq!(argv[5], inv.script());
        assert_eq!(argv[6], "/home/deve/.bun/bin/codex");
        assert_eq!(argv[7], "/mnt/e/gitclone/AI-Barking/docs/2610/1009");
        assert_eq!(&argv[10..], ["medium", "1200"]);
        assert_eq!(argv.len(), 12);
    }

    #[test]
    fn reverse_command_searches_and_attaches_nothing() {
        let inv = invocation(&[], true, "max");
        let script = inv.script();
        assert!(
            script.contains(r#""$0" --search exec -m gpt-6-astra"#),
            "{script}"
        );
        assert!(!script.contains("--image"));
        assert!(script.ends_with(r#"-o "$2" - < "$3""#), "{script}");
        assert_eq!(inv.argv().last().unwrap(), "0");
    }

    #[test]
    fn display_line_quotes_only_what_needs_it() {
        let inv = invocation(&["images/00-tldr.png"], false, "high");
        let line = inv.display();
        assert!(
            line.starts_with("MSYS_NO_PATHCONV=1 wsl -d Debian -e bash -lc '"),
            "{line}"
        );
        assert!(line.ends_with(
            "/home/deve/.bun/bin/codex /mnt/e/gitclone/AI-Barking/docs/2610/1009 /mnt/c/Users/QQ/AppData/Local/Temp/barking-review/1009/visual-report.md /mnt/c/Users/QQ/AppData/Local/Temp/barking-review/1009/visual-prompt.md high 0"
        ), "{line}");
        assert_eq!(sh_quote("a b"), "'a b'");
        assert_eq!(sh_quote("it's"), r"'it'\''s'");
        assert_eq!(sh_quote(""), "''");
        assert_eq!(sh_quote("/mnt/c/a-b_c.md"), "/mnt/c/a-b_c.md");
        // Paths with spaces travel as quoted positional arguments.
        let mut inv = invocation(&[], false, "medium");
        inv.cwd = "/mnt/e/my repo".into();
        assert!(inv.display().contains("'/mnt/e/my repo'"));
    }

    #[test]
    fn unsafe_image_names_are_detected() {
        assert!(safe_arg("images/41-x-raw.png"));
        assert!(!safe_arg("images/a b.png"));
        assert!(!safe_arg("images/a\"b.png"));
        assert!(!safe_arg("images/$x.png"));
        assert!(!safe_arg("images/批注.png"));
        assert!(!safe_arg(""));
    }

    #[test]
    fn reports_are_never_overwritten() {
        assert_eq!(
            artifact_names(Mode::Visual, 1),
            (
                "visual-prompt.md".to_string(),
                "visual-report.md".to_string()
            )
        );
        assert_eq!(
            artifact_names(Mode::Reverse, 3),
            (
                "reverse-prompt-3.md".to_string(),
                "reverse-report-3.md".to_string()
            )
        );
        let dir = std::env::temp_dir()
            .join("barking-review-test")
            .join(std::process::id().to_string());
        fs::create_dir_all(&dir).unwrap();
        assert_eq!(pick_serial(&dir, Mode::Visual), 1);
        // A prompt alone does not claim the serial; a report does.
        fs::write(dir.join("visual-prompt.md"), "p").unwrap();
        assert_eq!(pick_serial(&dir, Mode::Visual), 1);
        fs::write(dir.join("visual-report.md"), "r").unwrap();
        assert_eq!(pick_serial(&dir, Mode::Visual), 2);
        fs::write(dir.join("visual-report-2.md"), "r").unwrap();
        assert_eq!(pick_serial(&dir, Mode::Visual), 3);
        assert_eq!(pick_serial(&dir, Mode::Reverse), 1);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn only_text_archives_are_listed() {
        for ok in [
            "doc_1009_publish.txt",
            "images/cards.toml",
            "sources/a-hn-1.json",
            "sources/x.JSONL",
            "sources/offsite.tsv",
            "sources/a-shot.py",
        ] {
            assert!(is_text_file(ok), "{ok}");
        }
        for no in [
            "images/00-tldr.png",
            "sources/a.pdf",
            "sources/a.html",
            "sources/noext",
            "sources/a.mp4",
        ] {
            assert!(!is_text_file(no), "{no}");
        }
    }

    #[test]
    fn elapsed_reads_naturally() {
        assert_eq!(format_elapsed(45), "45 秒");
        assert_eq!(format_elapsed(125), "2 分 5 秒");
    }
}
