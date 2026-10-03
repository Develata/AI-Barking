mod card;
mod lint;
mod offsite;

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use lint::Level;

const USAGE: &str = "\
用法：
  barking lint [期次目录...]                定稿前机械检查；不给目录则检查 docs/<YYMM>/<MMDD> 下全部期次（如 docs/2609/0925）
  barking offsite <期次目录>... [--upload]  把不入库的原件记入 sources/offsite.tsv；--upload 同时上传 OpenList
  barking card <期次目录> [文件名...]       按 images/cards.toml 渲染省流卡、速览聚合图与批注截图（见 templates/cards/）

退出码：0 无错误；1 有错误；2 用法错误";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.split_first() {
        Some((cmd, rest)) if cmd == "lint" => run_lint(rest),
        Some((cmd, rest)) if cmd == "offsite" => offsite::run(rest),
        Some((cmd, rest)) if cmd == "card" => card::run(rest),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run_lint(args: &[String]) -> ExitCode {
    let dirs = if args.is_empty() {
        match all_issues() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("error: 找不到期次目录：{e}");
                return ExitCode::from(2);
            }
        }
    } else {
        args.iter().map(PathBuf::from).collect()
    };

    let (mut errors, mut warns) = (0usize, 0usize);
    for dir in &dirs {
        println!("== {}", dir.display());
        if !dir.is_dir() {
            println!("error: {}: 不是目录", dir.display());
            errors += 1;
            continue;
        }
        for f in lint::lint_issue(dir) {
            match f.level {
                Level::Error => errors += 1,
                Level::Warn => warns += 1,
                Level::Info => {}
            }
            println!("{f}");
        }
    }
    println!("-- {errors} 个错误，{warns} 个警告");
    if errors > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// The repo root is found by walking up from the current directory to `EDITORIAL.md`.
fn repo_root() -> io::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    cwd.ancestors()
        .find(|p| p.join("EDITORIAL.md").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| io::Error::other("当前目录不在 AI-Barking 仓库内"))
}

fn is_four_digits(name: &str) -> bool {
    name.len() == 4 && name.bytes().all(|b| b.is_ascii_digit())
}

/// Sorted subdirectories of `dir` whose names are four ASCII digits.
fn digit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(is_four_digits)
        })
        .collect();
    out.sort();
    Ok(out)
}

/// Issue directories are `docs/<YYMM>/<MMDD>/`, four digits at each level (see
/// AGENTS.md). Issues named in `docs/.lint-ignore` (one `YYMM/MMDD` per line,
/// `#` starts a comment) are skipped here; naming one explicitly on the
/// command line still lints it.
fn all_issues() -> io::Result<Vec<PathBuf>> {
    let docs = repo_root()?.join("docs");
    let ignored: Vec<String> = std::fs::read_to_string(docs.join(".lint-ignore"))
        .unwrap_or_default()
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let mut dirs: Vec<PathBuf> = Vec::new();
    for month in digit_dirs(&docs)? {
        for issue in digit_dirs(&month)? {
            // Both names passed `is_four_digits`, so they are valid UTF-8.
            let key = format!(
                "{}/{}",
                month.file_name().and_then(|n| n.to_str()).unwrap_or(""),
                issue.file_name().and_then(|n| n.to_str()).unwrap_or("")
            );
            if ignored.contains(&key) {
                println!("skip: {}（列于 docs/.lint-ignore）", issue.display());
            } else {
                dirs.push(issue);
            }
        }
    }
    if dirs.is_empty() && ignored.is_empty() {
        return Err(io::Error::other("docs/ 下没有 <YYMM>/<MMDD> 期次目录"));
    }
    Ok(dirs)
}
