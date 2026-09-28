mod lint;

use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use lint::Level;

const USAGE: &str = "\
用法：
  barking lint [期次目录...]   定稿前机械检查；不给目录则检查 docs/ 下全部期次（如 docs/0925）

退出码：0 无错误；1 有错误；2 用法错误";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.split_first() {
        Some((cmd, rest)) if cmd == "lint" => run_lint(rest),
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

/// Issue directories are the four-digit `MMDD` folders under `docs/` (see
/// AGENTS.md); the repo root is found by walking up to `EDITORIAL.md`.
/// Issues named in `docs/.lint-ignore` (one `MMDD` per line, `#` starts a
/// comment) are skipped here; naming one explicitly on the command line still
/// lints it.
fn all_issues() -> io::Result<Vec<PathBuf>> {
    let cwd = std::env::current_dir()?;
    let root = cwd
        .ancestors()
        .find(|p| p.join("EDITORIAL.md").is_file())
        .ok_or_else(|| io::Error::other("当前目录不在 AI-Barking 仓库内"))?;
    let docs = root.join("docs");
    let ignored: Vec<String> = std::fs::read_to_string(docs.join(".lint-ignore"))
        .unwrap_or_default()
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&docs)?.filter_map(Result::ok) {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !(path.is_dir() && name.len() == 4 && name.bytes().all(|b| b.is_ascii_digit())) {
            continue;
        }
        if ignored.iter().any(|i| i == name) {
            println!("skip: {}（列于 docs/.lint-ignore）", path.display());
        } else {
            dirs.push(path);
        }
    }
    if dirs.is_empty() && ignored.is_empty() {
        return Err(io::Error::other("docs/ 下没有 MMDD 期次目录"));
    }
    dirs.sort();
    Ok(dirs)
}
