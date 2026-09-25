mod lint;

use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use lint::Level;

const USAGE: &str = "\
用法：
  barking lint [期次目录...]   定稿前机械检查；不给目录则检查仓库内全部期次（如 0924、0925）

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

/// Issue directories are the four-digit `MMDD` folders under the repo root or
/// its `docs/`; the root is found by walking up from the current directory to
/// `EDITORIAL.md`.
fn all_issues() -> io::Result<Vec<PathBuf>> {
    let cwd = std::env::current_dir()?;
    let root = cwd
        .ancestors()
        .find(|p| p.join("EDITORIAL.md").is_file())
        .ok_or_else(|| io::Error::other("当前目录不在 AI-Barking 仓库内"))?;
    let mut dirs = Vec::new();
    for parent in [root.to_path_buf(), root.join("docs")] {
        let Ok(rd) = std::fs::read_dir(&parent) else {
            continue;
        };
        dirs.extend(rd.filter_map(Result::ok).map(|e| e.path()).filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.len() == 4 && n.bytes().all(|b| b.is_ascii_digit()))
        }));
    }
    if dirs.is_empty() {
        return Err(io::Error::other(
            "仓库根目录和 docs/ 下都没有 MMDD 期次目录",
        ));
    }
    dirs.sort();
    Ok(dirs)
}
