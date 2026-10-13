//! The capturing account's identifiers, for `publish-check`'s identity step:
//! what must never appear in a committed archive (templates/codex-evidence.md
//! item 7). Develata configures only the X handles; the display name, avatar
//! id and numeric user id are looked up through fxtwitter's public user
//! endpoint (already the repo's tool for X data) and cached outside the
//! repository. The cache keeps every value ever seen for a handle, so an old
//! display name still catches the archives made under it after a rename.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::offsite;

/// Public user lookup, no login (FixTweet API).
const USER_API: &str = "https://api.fxtwitter.com/";
/// Cached values are re-fetched after this long; a rename shows up within a day.
const MAX_AGE_SECS: u64 = 24 * 3600;
/// Shorter patterns are too common in archives (ids, words) to be a signal.
const MIN_PATTERN_LEN: usize = 4;

#[derive(Serialize, Deserialize, Default)]
struct Cache {
    /// Keyed by lower-case handle.
    handles: BTreeMap<String, Handle>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Handle {
    fetched_unix: u64,
    fetched_bj: String,
    /// Every pattern seen for this handle, in first-seen order.
    patterns: Vec<String>,
}

/// Lower-case substrings to search for, plus notes for the report. Patterns
/// come from `BARKING_IDENTITY_HANDLES` (resolved and cached) and from
/// `BARKING_IDENTITY_PATTERNS` (manual extras); both `;`-separated, the
/// environment overriding `.env`. The values themselves never go into notes.
pub(crate) fn patterns(root: &Path) -> (Vec<String>, Vec<String>) {
    let env = offsite::load_env(root);
    let read = |key: &str| {
        std::env::var(key)
            .ok()
            .filter(|v| !v.trim().is_empty())
            .or_else(|| env.get(key).cloned())
            .unwrap_or_default()
    };
    let mut notes = Vec::new();
    let mut out: Vec<String> = split(&read("BARKING_IDENTITY_PATTERNS"));
    let manual = out.len();
    if manual > 0 {
        notes.push(format!("手填模式 {manual} 个（BARKING_IDENTITY_PATTERNS）"));
    }
    let handles = split(&read("BARKING_IDENTITY_HANDLES"));
    if !handles.is_empty() {
        let path = cache_path();
        let mut cache = load_cache(&path);
        let now = unix_now();
        let mut changed = false;
        for h in &handles {
            let h = h.trim_start_matches('@').to_string();
            let entry = cache.handles.get(&h).cloned();
            let fresh = entry
                .as_ref()
                .is_some_and(|e| now.saturating_sub(e.fetched_unix) < MAX_AGE_SECS);
            let entry = if fresh {
                notes.push(format!("handle「{h}」：用缓存（不满一天）"));
                entry
            } else {
                match fetch(&h) {
                    Ok(found) => {
                        let mut e = entry.unwrap_or_default();
                        for p in found {
                            if !e.patterns.contains(&p) {
                                e.patterns.push(p);
                            }
                        }
                        e.fetched_unix = now;
                        e.fetched_bj = crate::repo::beijing_now();
                        cache.handles.insert(h.clone(), e.clone());
                        changed = true;
                        notes.push(format!(
                            "handle「{h}」：已刷新，累计 {} 个模式",
                            e.patterns.len()
                        ));
                        Some(e)
                    }
                    Err(err) => match entry {
                        Some(e) => {
                            notes.push(format!(
                                "warn: handle「{h}」刷新失败（{err}），用 {} 的缓存",
                                e.fetched_bj
                            ));
                            Some(e)
                        }
                        None => {
                            notes.push(format!(
                                "warn: handle「{h}」无法解析且没有缓存（{err}）：只按 handle 本身检查"
                            ));
                            Some(Handle {
                                patterns: vec![h.to_lowercase()],
                                ..Handle::default()
                            })
                        }
                    },
                }
            };
            if let Some(e) = entry {
                for p in e.patterns {
                    if !out.contains(&p) {
                        out.push(p);
                    }
                }
            }
        }
        if changed && let Err(e) = save_cache(&path, &cache) {
            notes.push(format!("warn: 身份缓存未能写入 {}：{e}", path.display()));
        }
    }
    let (kept, short): (Vec<String>, Vec<String>) = out.into_iter().partition(|p| usable(p));
    if !short.is_empty() {
        notes.push(format!(
            "warn: {} 个模式太短（英文数字不足 {MIN_PATTERN_LEN} 个字符，中文不足 2 个字），太常见，已忽略",
            short.len()
        ));
    }
    (kept, notes)
}

pub(crate) fn split(raw: &str) -> Vec<String> {
    raw.split(';')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Patterns an X user object yields: screen name, display name, the avatar's
/// `profile_images/<id>` segment (stable across size suffixes), and the
/// numeric user id. All lower-cased, since the search is case-insensitive.
pub(crate) fn patterns_from_user(user: &Value) -> Vec<String> {
    let s = |k: &str| user.get(k).and_then(Value::as_str).map(str::trim);
    let mut out = Vec::new();
    let mut push = |p: String| {
        let p = p.to_lowercase();
        if !p.is_empty() && !out.contains(&p) {
            out.push(p);
        }
    };
    if let Some(v) = s("screen_name") {
        push(v.to_string());
    }
    if let Some(v) = s("name") {
        push(v.to_string());
    }
    if let Some(url) = s("avatar_url")
        && let Some(i) = url.find("profile_images/")
    {
        let id: String = url[i + "profile_images/".len()..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if !id.is_empty() {
            push(format!("profile_images/{id}"));
        }
    }
    if let Some(v) = s("id")
        && v.len() >= 10
        && v.bytes().all(|b| b.is_ascii_digit())
    {
        push(v.to_string());
    }
    out
}

/// Long enough to be a signal: four ASCII characters, or two CJK characters
/// (a two-character Chinese display name is as specific as a word).
fn usable(p: &str) -> bool {
    let n = p.chars().count();
    let cjk = p
        .chars()
        .any(|c| matches!(c, '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}'));
    n >= MIN_PATTERN_LEN || (cjk && n >= 2)
}

fn fetch(handle: &str) -> Result<Vec<String>, String> {
    if !handle
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return Err("handle 只能含字母、数字和下划线".into());
    }
    // fxtwitter answers the same lookup with "User not found" now and then
    // (observed 1 in 5 for one account), so a miss is retried before it counts.
    let mut last = String::new();
    let mut user = None;
    for attempt in 0..3 {
        if attempt > 0 {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        match crate::fetch::get_json_any_status(&format!("{USER_API}{handle}")) {
            Ok(v) if v.get("code").and_then(Value::as_i64) == Some(200) => {
                user = v.get("user").cloned();
                break;
            }
            Ok(v) => {
                last = format!(
                    "接口返回 {}",
                    v.get("message").and_then(Value::as_str).unwrap_or("未知")
                );
            }
            Err(e) => last = e,
        }
    }
    let user = user.ok_or_else(|| format!("{last}（已重试 3 次）"))?;
    let user = &user;
    let found = patterns_from_user(user);
    if found.is_empty() {
        return Err("响应里没有可用字段".into());
    }
    Ok(found)
}

/// `~/.config/barking/identity.json`: outside the repository, like `.env`.
fn cache_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    home.join(".config").join("barking").join("identity.json")
}

fn load_cache(path: &Path) -> Cache {
    fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

fn save_cache(path: &Path, cache: &Cache) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(cache).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn user_object_yields_stable_patterns() {
        let u = json!({
            "id": "1353836358901501952",
            "name": "Anthropic",
            "screen_name": "AnthropicAI",
            "avatar_url": "https://pbs.twimg.com/profile_images/1798110641414443008/XP8gyBaY_normal.jpg"
        });
        assert_eq!(
            patterns_from_user(&u),
            [
                "anthropicai",
                "anthropic",
                "profile_images/1798110641414443008",
                "1353836358901501952"
            ]
        );
        // A short numeric id or a missing avatar adds nothing.
        let u = json!({"id": "42", "screen_name": "x_y", "name": "  "});
        assert_eq!(patterns_from_user(&u), ["x_y"]);
    }

    #[test]
    fn cache_keeps_old_names_across_renames() {
        let dir =
            std::env::temp_dir().join(format!("barking-identity-test-{}", std::process::id()));
        let path = dir.join("identity.json");
        let mut cache = Cache::default();
        cache.handles.insert(
            "dev".into(),
            Handle {
                fetched_unix: 1,
                fetched_bj: "t".into(),
                patterns: vec!["old name".into()],
            },
        );
        save_cache(&path, &cache).unwrap();
        let mut again = load_cache(&path);
        let e = again.handles.get_mut("dev").unwrap();
        for p in ["new name".to_string(), "old name".to_string()] {
            if !e.patterns.contains(&p) {
                e.patterns.push(p);
            }
        }
        assert_eq!(e.patterns, ["old name", "new name"]);
        assert_eq!(split(" @A ;; b;"), ["@a", "b"]);
        assert!(usable("只影") && usable("abcd") && !usable("abc") && !usable("影"));
        let _ = fs::remove_dir_all(&dir);
    }
}
