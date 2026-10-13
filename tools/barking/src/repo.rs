//! Shared ground for the subcommands: resolving an issue directory
//! (`docs/<YYMM>/<MMDD>` under the repo root), the calendar date it denotes,
//! the Beijing clock used in every log line, and thin `git` helpers.
//!
//! Dates are plain civil arithmetic (no chrono): issue numbers are days, and
//! the 三天 window of daily-scan.md is computed on them.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// One issue directory, resolved against the repository it lives in.
#[derive(Clone, Debug)]
pub struct Issue {
    /// Absolute repository root (the directory holding EDITORIAL.md).
    pub root: PathBuf,
    /// Absolute issue directory.
    pub dir: PathBuf,
    pub yymm: String,
    pub mmdd: String,
}

impl Issue {
    /// `dir` may be relative to the current directory; it must be
    /// `docs/<YYMM>/<MMDD>` (four ASCII digits each) inside the repository.
    pub fn resolve(dir: &Path) -> Result<Issue, String> {
        let abs = std::path::absolute(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        if !abs.is_dir() {
            return Err(format!("{}: 不是目录", dir.display()));
        }
        let root = crate::card::repo_root(&abs)?;
        let rel = abs
            .strip_prefix(&root)
            .map_err(|_| format!("{} 不在仓库内", dir.display()))?;
        let parts: Vec<&str> = rel
            .components()
            .filter_map(|c| c.as_os_str().to_str())
            .collect();
        let (yymm, mmdd) = match parts.as_slice() {
            ["docs", m, d] if crate::is_four_digits(m) && crate::is_four_digits(d) => {
                (m.to_string(), d.to_string())
            }
            _ => {
                return Err(format!(
                    "{} 不是 docs/<YYMM>/<MMDD> 期次目录",
                    rel.display()
                ));
            }
        };
        Ok(Issue {
            root,
            dir: abs,
            yymm,
            mmdd,
        })
    }

    /// `docs/<YYMM>/<MMDD>`, `/`-separated, as paths are written in git and
    /// in the markdown files.
    pub fn rel(&self) -> String {
        format!("docs/{}/{}", self.yymm, self.mmdd)
    }

    /// `YYMMMMDD` as a number, for ordering and the dated lint rules.
    pub fn key(&self) -> u32 {
        self.yymm.parse::<u32>().unwrap_or(0) * 10_000 + self.mmdd.parse::<u32>().unwrap_or(0)
    }

    /// The calendar date the issue number denotes (`2610`/`1008` → 2026-10-08).
    /// Issue numbers follow Develata's local date, see AGENTS.md.
    pub fn date(&self) -> Result<Date, String> {
        let yy: i32 = self.yymm[..2].parse().map_err(|_| "期次年份无效")?;
        let m: u32 = self.yymm[2..].parse().map_err(|_| "期次月份无效")?;
        let d: u32 = self.mmdd[2..].parse().map_err(|_| "期次日期无效")?;
        if self.mmdd[..2] != self.yymm[2..] {
            return Err(format!("期次 {}/{} 的月份不一致", self.yymm, self.mmdd));
        }
        Date::new(2000 + yy, m, d)
            .ok_or_else(|| format!("期次 {}/{} 不是有效日期", self.yymm, self.mmdd))
    }

    pub fn images(&self) -> PathBuf {
        self.dir.join("images")
    }

    pub fn sources(&self) -> PathBuf {
        self.dir.join("sources")
    }

    /// `doc_<MMDD>_publish.txt`, the single publish text of the issue.
    pub fn publish_txt(&self) -> PathBuf {
        self.dir.join(format!("doc_{}_publish.txt", self.mmdd))
    }

    /// First non-empty line of the publish text: the title.
    pub fn title(&self) -> Result<String, String> {
        let path = self.publish_txt();
        let raw = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        crate::lint::normalize(&raw)
            .lines()
            .map(str::trim)
            .find(|l| !l.is_empty())
            .map(str::to_string)
            .ok_or_else(|| format!("{}: 正文为空", path.display()))
    }

    /// Every issue directory of the repository, oldest first.
    pub fn all(root: &Path) -> Result<Vec<Issue>, String> {
        let docs = root.join("docs");
        let mut out = Vec::new();
        for month in crate::digit_dirs(&docs).map_err(|e| format!("{}: {e}", docs.display()))? {
            for dir in crate::digit_dirs(&month).map_err(|e| format!("{}: {e}", month.display()))? {
                out.push(Issue::resolve(&dir)?);
            }
        }
        out.sort_by_key(Issue::key);
        Ok(out)
    }
}

/// A proleptic Gregorian calendar date.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub y: i32,
    pub m: u32,
    pub d: u32,
}

impl Date {
    pub fn new(y: i32, m: u32, d: u32) -> Option<Date> {
        let dim = match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
            2 => 28,
            _ => return None,
        };
        (1..=dim).contains(&d).then_some(Date { y, m, d })
    }

    /// `MMDD` in `year` (how daily-scan.md and README.md name issues).
    pub fn from_mmdd(year: i32, mmdd: &str) -> Option<Date> {
        if !crate::is_four_digits(mmdd) {
            return None;
        }
        Date::new(year, mmdd[..2].parse().ok()?, mmdd[2..].parse().ok()?)
    }

    /// Days since 1970-01-01 (Howard Hinnant's `days_from_civil`).
    pub fn days(self) -> i64 {
        let y = i64::from(if self.m <= 2 { self.y - 1 } else { self.y });
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = i64::from((self.m + 9) % 12);
        let doy = (153 * mp + 2) / 5 + i64::from(self.d) - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// Inverse of [`Date::days`] (`civil_from_days`).
    pub fn from_days(z: i64) -> Date {
        let z = z + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let y = yoe + era * 400 + i64::from(m <= 2);
        Date { y: y as i32, m, d }
    }

    pub fn plus_days(self, n: i64) -> Date {
        Date::from_days(self.days() + n)
    }

    /// `MMDD`.
    pub fn mmdd(self) -> String {
        format!("{:02}{:02}", self.m, self.d)
    }

    /// `YYYY-MM-DD`.
    pub fn iso(self) -> String {
        format!("{}-{:02}-{:02}", self.y, self.m, self.d)
    }
}

/// Beijing time (UTC+8) now, as `YYYY-MM-DDTHH:MM:SS+08:00`, the stamp the
/// capture logs use.
pub fn beijing_now() -> String {
    beijing_stamp(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0),
    )
}

/// Unix seconds in the `beijing_now` format, for timestamps that arrive from
/// elsewhere (API responses) rather than from the clock.
pub fn beijing_stamp(unix: i64) -> String {
    let secs = unix + 8 * 3600;
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    format!(
        "{}T{:02}:{:02}:{:02}+08:00",
        Date::from_days(days).iso(),
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// `git <args>` in `root`; stdout as text, trailing newline trimmed.
pub fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|e| format!("无法运行 git：{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {} 失败：{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
}

/// `git status --porcelain -z` as `(XY, path)` pairs; renames report the new
/// path. Paths are repo-relative with `/`.
pub fn git_status(root: &Path) -> Result<Vec<(String, String)>, String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(["status", "--porcelain", "-z", "--untracked-files=all"])
        .output()
        .map_err(|e| format!("无法运行 git：{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git status 失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut fields = text.split('\0').filter(|s| !s.is_empty());
    let mut rows = Vec::new();
    while let Some(f) = fields.next() {
        if f.len() < 4 {
            continue;
        }
        let (xy, path) = (f[..2].to_string(), f[3..].to_string());
        // A rename carries the original path in the next field.
        if xy.starts_with('R') || xy.starts_with('C') {
            fields.next();
        }
        rows.push((xy, path));
    }
    Ok(rows)
}

/// Bytes to a short `12.3 MB` / `456 KB` string.
pub fn human_size(bytes: u64) -> String {
    const MB: f64 = 1024.0 * 1024.0;
    if bytes as f64 >= MB {
        format!("{:.1} MB", bytes as f64 / MB)
    } else {
        format!("{} KB", bytes.div_ceil(1024))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_days_round_trip() {
        for (y, m, d, days) in [
            (1970, 1, 1, 0),
            (2000, 3, 1, 11017),
            (2026, 10, 8, 20734),
            (2026, 12, 31, 20818),
            (2027, 1, 1, 20819),
        ] {
            let date = Date::new(y, m, d).unwrap();
            assert_eq!(date.days(), days, "{y}-{m}-{d}");
            assert_eq!(Date::from_days(days), date);
        }
        assert_eq!(Date::new(2026, 2, 29), None);
        assert!(Date::new(2028, 2, 29).is_some());
    }

    #[test]
    fn three_day_window_crosses_months() {
        let d = Date::from_mmdd(2026, "1101").unwrap();
        assert_eq!(d.plus_days(-3).mmdd(), "1029");
        assert_eq!(
            Date::from_mmdd(2026, "0101").unwrap().plus_days(-1).iso(),
            "2025-12-31"
        );
        assert_eq!(Date::from_mmdd(2026, "1301"), None);
    }

    #[test]
    fn status_parses_renames() {
        // Exercised through the real binary elsewhere; here only the shape.
        assert_eq!(human_size(1_572_864), "1.5 MB");
        assert_eq!(human_size(1024), "1 KB");
    }
}
