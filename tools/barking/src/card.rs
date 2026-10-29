//! `barking card`: render one issue's 省流卡, 速览聚合图, 事件卡, 吠点卡 and annotated
//! screenshots (plus the template covers of a 速览专帖) from
//! `images/cards.toml`, filling the HTML templates in `templates/cards/` and
//! screenshotting them with headless Chrome.
//!
//! Highlights come from Tesseract word boxes found by matching the quoted
//! sentence. A missing, ambiguous, partial-word or low-confidence match is an
//! error, never a guess: a misplaced highlight on evidence puts words in the
//! source's mouth. Layout overflow (text cut off, screenshot squeezed) is
//! detected in the page and is an error too.
//!
//! Matching compares letters and digits only (see [`locate`]): spacing,
//! punctuation, curly vs straight quotes, apostrophes and dashes never decide
//! a match, so an OCR `Google's` or `Googles` both match `Google’s`. The one
//! glyph-level tolerance is a one-letter word `I`, which Tesseract reads as
//! `|`, `l` or `1` (issue 1014). Uniqueness, whole-word ends and the
//! confidence floor still hold.
//!
//! The edit → re-render loop is the slow part of making cards (issue 1014:
//! about 30 minutes, every round re-rendering and re-OCRing all seven cards),
//! so a run renders only what changed and reports every problem it can find:
//!
//! - All checks that need neither Chrome nor OCR run over the whole
//!   cards.toml first and are reported together; then each card renders
//!   independently, and one card's failure does not stop the others.
//! - Each output's inputs (its cards.toml entry plus shared fields, the base
//!   image or avatar bytes, the template, the Tesseract build and model for
//!   annotated screenshots, and this executable's own bytes, so any rebuild
//!   counts as a change) are hashed. An output whose PNG exists and whose
//!   hash matches the last successful render is skipped; `--all` or naming
//!   the file forces it. Outputs themselves are not hashed: publish
//!   recompresses them with oxipng, which must not count as a change. Chrome
//!   and system fonts are not hashed either; after changing them, `--all`.
//! - A card's entry is dropped (and the drop saved) before its image may be
//!   replaced, and recorded only after the new image is in place, so an
//!   interrupted run causes a re-render, never a trusted stale image. A lock
//!   file per images directory keeps two runs from writing the cache at once.
//! - The hashes live in the system temp dir, keyed by the images directory's
//!   absolute path, not in `images/`: `offsite` registers and uploads every
//!   git-ignored file there, and an unignored file would be a commit
//!   candidate. Losing the cache only costs one full render.
//! - A skipped PNG gets its modification time bumped, so `barking lint`'s
//!   "image older than cards.toml" warning keeps meaning "not confirmed since
//!   the last edit".

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const TLDR_TEMPLATE: &str = include_str!("../../../templates/cards/tldr.html");
const ANNOT_TEMPLATE: &str = include_str!("../../../templates/cards/annot.html");
const ROUNDUP_TEMPLATE: &str = include_str!("../../../templates/cards/roundup.html");
const COVER_TEMPLATE: &str = include_str!("../../../templates/cards/cover.html");
const BARK_TEMPLATE: &str = include_str!("../../../templates/cards/bark.html");
const EVENT_TEMPLATE: &str = include_str!("../../../templates/cards/event.html");
pub const SPEC_FILE: &str = "cards.toml";
/// Issues dated on or after this `YYMM_MMDD` put the images to upload in
/// `images/publish/`, numbered in upload order (EDITORIAL.md 省流卡与批注截图,
/// 2026-10-29). Earlier issues keep the flat `images/00-…` names they were
/// published with (EDITORIAL.md 纠错: no rewrites).
pub const PUBLISH_FROM: u32 = 2610_1029;
/// The sub-directory of `images/` holding the upload set.
pub const PUBLISH_DIR: &str = "publish";
pub const TLDR_OUT: &str = "00-tldr.png";
pub const ROUNDUP_OUT: &str = "00-roundup.png";
/// One 吠点卡 per main post, in the main posts' order. Fixed names keep the
/// outputs `&'static` and the 配图说明 predictable; no issue has had more
/// than four main posts.
const BARK_OUTS: [&str; 6] = [
    "00-bark-1.png",
    "00-bark-2.png",
    "00-bark-3.png",
    "00-bark-4.png",
    "00-bark-5.png",
    "00-bark-6.png",
];
/// One 事件卡 per main post, same fixed names and limit as `BARK_OUTS`.
const EVENT_OUTS: [&str; 6] = [
    "00-event-1.png",
    "00-event-2.png",
    "00-event-3.png",
    "00-event-4.png",
    "00-event-5.png",
    "00-event-6.png",
];
pub const COVER_OUT: &str = "00-cover.png";
pub const COVER_WIDE_OUT: &str = "00-cover-wide.png";
/// Mascot on the template covers, relative to the repo root.
const AVATAR: &str = "common_images/profile_picture.png";
/// 速览 entries per issue, main-post entries included (EDITORIAL.md 速览).
pub const ROUNDUP_MIN: usize = 5;
pub const ROUNDUP_MAX: usize = 10;
/// Character caps (every scalar counts) that keep ten entries legible at
/// 36 px on one card: a main-post title stays on one line, others on two.
const ROUNDUP_MAIN_MAX: usize = 20;
const ROUNDUP_TEXT_MAX: usize = 40; // EDITORIAL.md 速览：其余条目 ≤40 字
const TAG_MAX: usize = 12;
/// Cards and the 3:4 cover; the wide cover has its own size.
const CARD_SIZE: (u32, u32) = (1080, 1440);
const WIDE_SIZE: (u32, u32) = (1880, 800);
/// Highlight colours defined in annot.html (`.c1` … `.c4`).
const MAX_NOTES: usize = 4;
/// Below this Tesseract confidence a matched word is not trusted.
const MIN_CONF: f32 = 60.0;

/// What the issue publishes. A 速览专帖 has no main post: no 省流卡, and its
/// covers come from the fixed template instead of image generation.
#[derive(Deserialize, Serialize, Default, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Main,
    Roundup,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    #[serde(default)]
    pub kind: Kind,
    pub date: Option<String>,
    #[serde(default)]
    pub tldr: Vec<TldrItem>,
    #[serde(default)]
    pub roundup: Vec<RoundupItem>,
    #[serde(default)]
    pub event: Vec<EventCard>,
    #[serde(default)]
    pub bark: Vec<BarkCard>,
    #[serde(default)]
    pub annot: Vec<Annot>,
    /// The 2026-10-29 layout ([`PUBLISH_FROM`]): outputs in `images/publish/`,
    /// numbered in upload order; `[[annot]]` says which main post it belongs
    /// to. Not in cards.toml: [`load_spec`] sets it from the issue's date. A
    /// Spec parsed straight from text keeps the old layout.
    #[serde(skip)]
    pub publish: bool,
}

/// An 事件卡 (from 2026-10-29): one main post's fact paragraphs, so a reader
/// who only flips through the images gets the event before the 吠点. Its
/// heading is the main post's 速览 entry with the same tag.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EventCard {
    pub tag: String,
    /// The section's lines between its heading and the `吠点：` line, verbatim
    /// and in order, one per paragraph.
    pub paras: Vec<String>,
}

/// A 吠点卡 (from 2026-10-28): one main post's whole 吠点 section, so a reader
/// who only flips through the images still gets it. Its heading is the main
/// post's 速览 entry with the same tag, not a third copy of the title.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BarkCard {
    pub tag: String,
    /// The section's 吠点 lines, verbatim and in order, without the ①②
    /// numbers, which the card adds.
    pub points: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RoundupItem {
    pub tag: String,
    pub text: String,
    /// A main-post entry: a short title, verbatim from the publish text,
    /// pointing back to the 省流卡.
    #[serde(default)]
    pub main: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TldrItem {
    pub tag: String,
    pub fact: String,
    /// Retired 2026-10-03: the 省流卡 and annotated screenshots carry no 吠点
    /// (since 2026-10-28 it has its own card, [`BarkCard`]). Still parsed so a
    /// published issue's cards.toml (which records what its images said)
    /// loads; rendering such an entry is an error.
    #[serde(default)]
    pub barks: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Annot {
    /// Old layout: the file name in `images/` (`41-x.png`). 2026-10-29 on: a
    /// short English name without number or extension (`carbon-rollout`); the
    /// tool numbers it and writes `images/publish/NN-<out>.png`.
    pub out: String,
    /// 2026-10-29 on: the main post (1-based, in 省流卡 order) the screenshot
    /// belongs to, which fixes its place in the upload order. Not part of the
    /// fingerprint: it only moves the file.
    #[serde(default, skip_serializing)]
    pub post: Option<usize>,
    pub title: String,
    pub source: String,
    #[serde(default = "default_lang")]
    pub lang: String,
    /// Retired like `TldrItem::barks`.
    pub bark: Option<String>,
    pub note: Vec<Note>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Note {
    pub quote: Option<String>,
    /// Hand-measured fallback in raw-image pixels, `[x, y, w, h]` per line.
    pub rects: Option<Vec<[u32; 4]>>,
    pub gloss: String,
    pub emph: Option<String>,
    pub underline: Option<String>,
}

fn default_lang() -> String {
    "eng".into()
}

impl Spec {
    /// Card texts that must appear verbatim in the publish text. Other 速览
    /// entries are not in the body; lint checks them against fact-check.md.
    pub fn quoted_texts(&self) -> Vec<&str> {
        self.tldr
            .iter()
            .flat_map(|t| {
                std::iter::once(t.fact.as_str()).chain(t.barks.iter().map(String::as_str))
            })
            .chain(self.annot.iter().filter_map(|a| a.bark.as_deref()))
            .chain(self.main_entries().map(|r| r.text.as_str()))
            .collect()
    }

    /// Every text the rendered cards show that the account wrote itself (not
    /// the screenshots): checked for platform names that trigger 风控.
    pub fn own_texts(&self) -> Vec<&str> {
        let tldr = self.tldr.iter().flat_map(|t| [t.tag.as_str(), &t.fact]);
        let roundup = self.roundup.iter().flat_map(|r| [r.tag.as_str(), &r.text]);
        let event = (self.event.iter()).flat_map(|e| {
            std::iter::once(e.tag.as_str()).chain(e.paras.iter().map(String::as_str))
        });
        let bark = (self.bark.iter()).flat_map(|b| {
            std::iter::once(b.tag.as_str()).chain(b.points.iter().map(String::as_str))
        });
        let annot = self.annot.iter().flat_map(|a| {
            [a.title.as_str(), &a.source]
                .into_iter()
                .chain(a.note.iter().map(|n| n.gloss.as_str()))
        });
        tldr.chain(roundup)
            .chain(event)
            .chain(bark)
            .chain(annot)
            .collect()
    }

    /// The 事件卡 file names, one per [[event]] (no more than `EVENT_OUTS`).
    pub fn event_outputs(&self) -> Vec<String> {
        self.names_where(|j| matches!(j, Job::Event(_)))
    }

    /// An 事件卡's heading: the main 速览 entry with its tag.
    fn event_heading(&self, e: &EventCard) -> Option<&str> {
        self.main_entries()
            .find(|r| r.tag == e.tag)
            .map(|r| r.text.as_str())
    }

    /// The 吠点卡 file names, one per [[bark]] (no more than `BARK_OUTS`).
    pub fn bark_outputs(&self) -> Vec<String> {
        self.names_where(|j| matches!(j, Job::Bark(_)))
    }

    fn names_where(&self, keep: impl Fn(&Job) -> bool) -> Vec<String> {
        (self.items().into_iter())
            .filter(|(_, j)| keep(j))
            .map(|(name, _)| name)
            .collect()
    }

    /// A 吠点卡's heading: the main 速览 entry with its tag.
    fn bark_heading(&self, b: &BarkCard) -> Option<&str> {
        self.main_entries()
            .find(|r| r.tag == b.tag)
            .map(|r| r.text.as_str())
    }

    pub fn main_entries(&self) -> impl Iterator<Item = &RoundupItem> {
        self.roundup.iter().filter(|r| r.main)
    }

    pub fn other_entries(&self) -> impl Iterator<Item = &RoundupItem> {
        self.roundup.iter().filter(|r| !r.main)
    }

    /// Violations of the card rules decidable from cards.toml alone. Shared by
    /// `barking card` (refuses to render) and `barking lint` (errors).
    pub fn structure_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.kind == Kind::Roundup {
            if !self.tldr.is_empty() {
                errs.push("kind = \"roundup\"（速览专帖）没有主帖，不应有 [[tldr]]".into());
            }
            if self.roundup.is_empty() {
                errs.push("kind = \"roundup\" 需要 [[roundup]] 条目".into());
            }
            if self.roundup.iter().any(|r| r.main) {
                errs.push("速览专帖没有主帖，[[roundup]] 不应写 main = true".into());
            }
        }
        errs.extend(self.event_errors());
        errs.extend(self.bark_errors());
        errs.extend(self.annot_errors());
        if self.roundup.is_empty() {
            return errs;
        }
        let n = self.roundup.len();
        if !(ROUNDUP_MIN..=ROUNDUP_MAX).contains(&n) {
            errs.push(format!(
                "速览共 {n} 条，应为 {ROUNDUP_MIN}–{ROUNDUP_MAX} 条（主帖条目也算）"
            ));
        }
        // Main-post entries come first, in the main post's order.
        let lead = self.roundup.iter().take_while(|r| r.main).count();
        if self.main_entries().count() != lead {
            errs.push("main = true 的主帖条目须连续排在 [[roundup]] 最前".into());
        }
        if self.kind == Kind::Main {
            let main_tags: Vec<&str> = self.main_entries().map(|r| r.tag.as_str()).collect();
            let tldr_tags: Vec<&str> = self.tldr.iter().map(|t| t.tag.as_str()).collect();
            if main_tags != tldr_tags {
                errs.push(format!(
                    "速览的主帖条目 tag {main_tags:?} 须与省流卡 [[tldr]] 的 tag {tldr_tags:?} 一一对应、顺序相同"
                ));
            }
        }
        for r in &self.roundup {
            let len = r.text.chars().count();
            let max = if r.main {
                ROUNDUP_MAIN_MAX
            } else {
                ROUNDUP_TEXT_MAX
            };
            if r.text.trim().is_empty() || r.tag.trim().is_empty() {
                errs.push(format!("速览条目「{}」的 tag 或 text 为空", r.tag));
            } else if len > max {
                errs.push(format!("速览条目「{}」{len} 字，超过上限 {max}", r.text));
            }
            if r.tag.chars().count() > TAG_MAX {
                errs.push(format!("速览 tag「{}」超过 {TAG_MAX} 字", r.tag));
            }
        }
        errs
    }

    /// The [[annot]] rules about `out` and `post`, which decide the upload
    /// order (2026-10-29 on); older cards.toml files do not use `post`.
    fn annot_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if !self.publish {
            for a in self.annot.iter().filter(|a| a.post.is_some()) {
                errs.push(format!(
                    "{} 写了 post：1029 期之前的 cards.toml 不用 post（out 是 images/ 下的文件名）",
                    a.out
                ));
            }
            return errs;
        }
        let posts = self.tldr.len();
        let mut seen: Vec<String> = Vec::new();
        for a in &self.annot {
            let at = |e: &str| format!("批注图「{}」：{e}", a.out);
            match (self.kind, a.post) {
                (Kind::Main, None) => errs.push(at("缺少 post（它属于第几个主帖，1 起）")),
                (Kind::Main, Some(p)) if p == 0 || p > posts => errs.push(at(&format!(
                    "post = {p} 越界：本期有 {posts} 个主帖（[[tldr]]），post 从 1 起"
                ))),
                (Kind::Roundup, Some(_)) => errs.push(at("速览专帖没有主帖，不写 post")),
                _ => {}
            }
            errs.extend(out_name_problems(&a.out).into_iter().map(|e| at(&e)));
            let low = a.out.to_lowercase();
            if seen.contains(&low) {
                errs.push(at("out 与前面的重复（文件名不分大小写）"));
            }
            seen.push(low);
        }
        errs
    }

    /// What is worth a look but not an error: a main post without a
    /// screenshot to back it up.
    pub fn structure_warnings(&self) -> Vec<String> {
        if !self.publish || self.kind != Kind::Main {
            return Vec::new();
        }
        (self.tldr.iter().enumerate())
            .filter(|(i, _)| self.annot.iter().all(|a| a.post != Some(i + 1)))
            .map(|(i, t)| {
                format!(
                    "第 {} 个主帖「{}」没有批注图（[[annot]] 的 post = {}）",
                    i + 1,
                    t.tag,
                    i + 1
                )
            })
            .collect()
    }

    /// The [[event]] rules decidable from cards.toml alone; lint checks the
    /// paragraphs against the publish text.
    fn event_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.event.is_empty() {
            return errs;
        }
        if self.kind == Kind::Roundup {
            errs.push("速览专帖没有主帖，不应有 [[event]]".into());
            return errs;
        }
        let event_tags: Vec<&str> = self.event.iter().map(|e| e.tag.as_str()).collect();
        let tldr_tags: Vec<&str> = self.tldr.iter().map(|t| t.tag.as_str()).collect();
        if event_tags != tldr_tags {
            errs.push(format!(
                "事件卡 [[event]] 的 tag {event_tags:?} 须与省流卡 [[tldr]] 的 tag {tldr_tags:?} 一一对应、顺序相同"
            ));
        }
        if self.event.len() > EVENT_OUTS.len() {
            errs.push(format!(
                "[[event]] 有 {} 张，最多 {}（每个主帖一张）",
                self.event.len(),
                EVENT_OUTS.len()
            ));
        }
        for e in &self.event {
            if self.event_heading(e).is_none() {
                errs.push(format!(
                    "事件卡「{}」的小标题取自速览里同 tag 的主帖条目（main = true），没找到",
                    e.tag
                ));
            }
            if e.paras.is_empty() {
                errs.push(format!("事件卡「{}」没有 paras", e.tag));
            }
            if e.paras.iter().any(|p| p.trim().is_empty()) {
                errs.push(format!("事件卡「{}」有一段是空的", e.tag));
            }
        }
        errs
    }

    /// The [[bark]] rules decidable from cards.toml alone; lint checks the
    /// points against the publish text.
    fn bark_errors(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.bark.is_empty() {
            return errs;
        }
        if self.kind == Kind::Roundup {
            errs.push("速览专帖没有主帖，不应有 [[bark]]".into());
            return errs;
        }
        let bark_tags: Vec<&str> = self.bark.iter().map(|b| b.tag.as_str()).collect();
        let tldr_tags: Vec<&str> = self.tldr.iter().map(|t| t.tag.as_str()).collect();
        if bark_tags != tldr_tags {
            errs.push(format!(
                "吠点卡 [[bark]] 的 tag {bark_tags:?} 须与省流卡 [[tldr]] 的 tag {tldr_tags:?} 一一对应、顺序相同"
            ));
        }
        if self.bark.len() > BARK_OUTS.len() {
            errs.push(format!(
                "[[bark]] 有 {} 张，最多 {}（每个主帖一张）",
                self.bark.len(),
                BARK_OUTS.len()
            ));
        }
        for b in &self.bark {
            if self.bark_heading(b).is_none() {
                errs.push(format!(
                    "吠点卡「{}」的小标题取自速览里同 tag 的主帖条目（main = true），没找到",
                    b.tag
                ));
            }
            if b.points.is_empty() {
                errs.push(format!("吠点卡「{}」没有 points", b.tag));
            }
            for p in &b.points {
                if p.trim().is_empty() {
                    errs.push(format!("吠点卡「{}」有一条空的 point", b.tag));
                } else if strip_circled(p.trim_start()).is_some() {
                    errs.push(format!(
                        "吠点卡「{}」的 point 不写 ①② 编号（渲染时自动加）：「{p}」",
                        b.tag
                    ));
                }
            }
        }
        errs
    }

    /// Where the retired 吠点 fields are still filled in, if anywhere.
    fn retired_barks(&self) -> Vec<String> {
        let tldr = (self.tldr.iter().filter(|t| !t.barks.is_empty()))
            .map(|t| format!("[[tldr]]「{}」的 barks", t.tag));
        let annot =
            (self.annot.iter().filter(|a| a.bark.is_some())).map(|a| format!("{} 的 bark", a.out));
        tldr.chain(annot).collect()
    }

    /// Image file names this spec renders (relative to `images/`).
    pub fn outputs(&self) -> Vec<String> {
        self.jobs().into_iter().map(|(out, _)| out).collect()
    }

    /// Each output with what renders it, in rendering order.
    fn jobs(&self) -> Vec<(String, Job<'_>)> {
        (self.items().into_iter())
            .filter(|(_, job)| !matches!(job, Job::Picked))
            .collect()
    }

    /// Every image this issue uploads, with what makes it, in upload order and
    /// relative to `images/`. Old layout: the rendered files only, as flat
    /// `00-…` names. From [`PUBLISH_FROM`]: the whole of `images/publish/`,
    /// the two covers first, then numbered 01, 02, … without gaps.
    pub fn items(&self) -> Vec<(String, Job<'_>)> {
        if self.publish {
            self.publish_items()
        } else {
            self.flat_items()
        }
    }

    fn flat_items(&self) -> Vec<(String, Job<'_>)> {
        let mut jobs = Vec::new();
        if self.kind == Kind::Roundup {
            jobs.push((COVER_OUT.to_string(), Job::Cover(CARD_SIZE)));
            jobs.push((COVER_WIDE_OUT.to_string(), Job::Cover(WIDE_SIZE)));
        }
        if !self.tldr.is_empty() {
            jobs.push((TLDR_OUT.to_string(), Job::Tldr));
        }
        if !self.roundup.is_empty() {
            jobs.push((ROUNDUP_OUT.to_string(), Job::Roundup));
        }
        // Upload order: each main post's 事件卡, then its 吠点卡.
        let (events, barks) = (self.event_count(), self.bark_count());
        for i in 0..events.max(barks) {
            if i < events {
                jobs.push((EVENT_OUTS[i].to_string(), Job::Event(i)));
            }
            if i < barks {
                jobs.push((BARK_OUTS[i].to_string(), Job::Bark(i)));
            }
        }
        jobs.extend((self.annot.iter()).map(|a| (a.out.clone(), Job::Annot(a))));
        jobs
    }

    fn publish_items(&self) -> Vec<(String, Job<'_>)> {
        let mut items: Vec<(String, Job)> = Vec::new();
        let (cover, wide) = if self.kind == Kind::Roundup {
            (Job::Cover(CARD_SIZE), Job::Cover(WIDE_SIZE))
        } else {
            (Job::Picked, Job::Picked)
        };
        items.push((format!("{PUBLISH_DIR}/{COVER_OUT}"), cover));
        items.push((format!("{PUBLISH_DIR}/{COVER_WIDE_OUT}"), wide));
        let mut number = 0;
        let mut name = |tail: &str| {
            number += 1;
            format!("{PUBLISH_DIR}/{number:02}-{tail}.png")
        };
        if !self.tldr.is_empty() {
            items.push((name("tldr"), Job::Tldr));
        }
        if !self.roundup.is_empty() {
            items.push((name("roundup"), Job::Roundup));
        }
        // Per main post: 事件卡, 吠点卡, then its screenshots in cards.toml
        // order. Screenshots that name no real post (an error) come last.
        let (events, barks) = (self.event_count(), self.bark_count());
        let mut placed = vec![false; self.annot.len()];
        for k in 0..self.tldr.len().max(events).max(barks) {
            if k < events {
                items.push((name(&format!("event-{}", k + 1)), Job::Event(k)));
            }
            if k < barks {
                items.push((name(&format!("bark-{}", k + 1)), Job::Bark(k)));
            }
            for (a, done) in self.annot.iter().zip(&mut placed) {
                if a.post == Some(k + 1) {
                    items.push((name(&a.out), Job::Annot(a)));
                    *done = true;
                }
            }
        }
        for (a, _) in self.annot.iter().zip(&placed).filter(|(_, done)| !**done) {
            items.push((name(&a.out), Job::Annot(a)));
        }
        items
    }

    /// 事件卡 and 吠点卡 that get a file (no more than the fixed names).
    fn event_count(&self) -> usize {
        self.event.len().min(EVENT_OUTS.len())
    }

    fn bark_count(&self) -> usize {
        self.bark.len().min(BARK_OUTS.len())
    }

    /// The directory under `images/` that the outputs go to.
    pub fn out_dir(&self, images: &Path) -> PathBuf {
        if self.publish {
            images.join(PUBLISH_DIR)
        } else {
            images.to_path_buf()
        }
    }

    /// An annotated screenshot's base image, relative to `images/`: the
    /// output's name plus `-raw` (old layout, `41-x.png` -> `41-x-raw.png`), or
    /// the bare `out` plus `-raw` (`x-raw.png`), which carries no number
    /// because the tool numbers the output.
    pub fn raw_file(&self, a: &Annot) -> Option<String> {
        if self.publish {
            Some(format!("{}-raw.png", a.out))
        } else {
            a.out
                .strip_suffix(".png")
                .map(|stem| format!("{stem}-raw.png"))
        }
    }

    fn raw_path(&self, a: &Annot, images: &Path) -> Option<PathBuf> {
        self.raw_file(a).map(|f| images.join(f))
    }

    /// An output as typed on the command line: in the publish layout the
    /// `publish/` directory may be left out.
    fn output_name(&self, typed: &str) -> String {
        let typed = typed.replace('\\', "/");
        if self.publish && !typed.contains('/') {
            format!("{PUBLISH_DIR}/{typed}")
        } else {
            typed
        }
    }

    /// Names in `images/publish/` that the current outputs do not claim, as
    /// `(stale, unknown)`: `stale` are this tool's own earlier products
    /// (`NN-tldr.png`, `NN-roundup.png`, `NN-event-K.png`, `NN-bark-K.png`,
    /// `NN-<name>.png` for an `[[annot]]` still in cards.toml) under a number
    /// or post that has since changed; `unknown` is everything else. The
    /// covers (`00-cover*`) are neither: they are copied in by hand.
    pub fn leftovers(&self, names: &[String]) -> (Vec<String>, Vec<String>) {
        let current: Vec<String> = (self.items().iter())
            .map(|(n, _)| leaf_name(n).to_lowercase())
            .collect();
        let tails: Vec<String> = (self.annot.iter())
            .map(|a| a.out.to_lowercase())
            .chain(["tldr".to_string(), "roundup".to_string()])
            .chain((1..=EVENT_OUTS.len()).map(|k| format!("event-{k}")))
            .chain((1..=BARK_OUTS.len()).map(|k| format!("bark-{k}")))
            .collect();
        let (mut stale, mut unknown) = (Vec::new(), Vec::new());
        for n in names {
            let low = n.to_lowercase();
            if low.starts_with("00-cover") || current.contains(&low) {
                continue;
            }
            let own = low.strip_suffix(".png").is_some_and(|t| {
                t.split_once('-').is_some_and(|(num, tail)| {
                    num.len() == 2
                        && num != "00"
                        && num.bytes().all(|b| b.is_ascii_digit())
                        && tails.iter().any(|x| x == tail)
                })
            });
            if own {
                stale.push(n.clone());
            } else {
                unknown.push(n.clone());
            }
        }
        (stale, unknown)
    }

    /// Hash of everything that determines `job`'s image except Chrome and the
    /// system fonts. `tool` is the hash of this executable; `ocr` identifies
    /// the Tesseract build and model an annotated screenshot is located with
    /// (see [`ocr_identity`]). Unreadable files hash as missing; rendering
    /// then fails, and failures are never cached.
    fn fingerprint(
        &self,
        job: &Job,
        images: &Path,
        avatar: Option<&Path>,
        tool: &str,
        ocr: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        let mut part = |bytes: &[u8]| {
            // Length-prefixed, so adjacent parts cannot run together.
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        };
        let file = |p: Option<&Path>| p.and_then(|p| fs::read(p).ok()).unwrap_or_default();
        part(tool.as_bytes());
        part(&json(&(self.kind, &self.date)));
        match job {
            Job::Picked => {}
            Job::Cover((w, h)) => {
                part(COVER_TEMPLATE.as_bytes());
                part(&json(&(w, h, &self.roundup)));
                part(&file(avatar));
            }
            Job::Tldr => {
                part(TLDR_TEMPLATE.as_bytes());
                part(&json(&self.tldr));
            }
            Job::Roundup => {
                part(ROUNDUP_TEMPLATE.as_bytes());
                part(&json(&self.roundup));
            }
            Job::Event(i) => {
                // The index sets the colour; the heading comes from 速览.
                let e = &self.event[*i];
                part(EVENT_TEMPLATE.as_bytes());
                part(&json(&(i, e, self.event_heading(e))));
            }
            Job::Bark(i) => {
                // The index sets the colour; the heading comes from 速览.
                let b = &self.bark[*i];
                part(BARK_TEMPLATE.as_bytes());
                part(&json(&(i, b, self.bark_heading(b))));
            }
            Job::Annot(a) => {
                part(ANNOT_TEMPLATE.as_bytes());
                part(&json(a));
                part(&file(self.raw_path(a, images).as_deref()));
                part(ocr.as_bytes());
            }
        }
        hex(&hasher.finalize())
    }
}

/// An output name without its `publish/` directory.
pub fn leaf_name(name: &str) -> &str {
    name.strip_prefix(PUBLISH_DIR)
        .and_then(|t| t.strip_prefix('/'))
        .unwrap_or(name)
}

/// Why an `[[annot]]` `out` is not a valid short name (2026-10-29 on), if it
/// is not: no number, no extension, plain ASCII, nothing the tool names itself.
fn out_name_problems(out: &str) -> Vec<String> {
    let mut errs = Vec::new();
    let digits = out.bytes().take_while(u8::is_ascii_digit).count();
    if out.trim().is_empty() {
        errs.push("out 为空".to_string());
        return errs;
    }
    if out.contains(['/', '\\']) || out.contains('.') {
        errs.push(format!(
            "out 写不带扩展名、不含路径的短英文名（如 carbon-rollout），.png 由工具加：「{out}」"
        ));
    }
    if digits > 0 && out[digits..].starts_with('-') {
        errs.push(format!(
            "out 不带序号前缀（序号由工具按主帖顺序编，如 03-）：「{out}」"
        ));
    }
    if !out
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        errs.push(format!("out 只用英文字母、数字、- 和 _：「{out}」"));
    }
    let low = out.to_lowercase();
    let reserved = ["tldr", "roundup", "cover", "cover-wide"].contains(&low.as_str())
        || ["event-", "bark-"].iter().any(|p| {
            low.strip_prefix(p)
                .is_some_and(|k| k.bytes().all(|b| b.is_ascii_digit()))
        });
    if reserved {
        errs.push(format!("out「{out}」是工具自己用的名字，换一个"));
    }
    errs
}

/// Serialization of in-memory spec values cannot fail.
fn json<T: Serialize>(v: &T) -> Vec<u8> {
    serde_json::to_vec(v).unwrap_or_default()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// What makes one image of the upload set.
#[derive(Clone, Copy)]
pub enum Job<'a> {
    /// A 速览专帖 template cover of this size.
    Cover((u32, u32)),
    /// A main post's cover: generated and copied in by hand, never rendered
    /// here (2026-10-29 layout only; listed so the upload order is complete).
    Picked,
    Tldr,
    Roundup,
    /// The 事件卡 of `spec.event[i]`.
    Event(usize),
    /// The 吠点卡 of `spec.bark[i]`.
    Bark(usize),
    Annot(&'a Annot),
}

/// `docs/<YYMM>/<MMDD>` -> `YYMMMMDD` as a number; `None` if the path is not
/// shaped like an issue directory.
pub(crate) fn issue_key(dir: &Path) -> Option<u32> {
    let four = |p: Option<&Path>| {
        p.and_then(Path::file_name)
            .and_then(|n| n.to_str())
            .filter(|n| n.len() == 4 && n.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|n| n.parse::<u32>().ok())
    };
    Some(four(dir.parent())? * 10_000 + four(Some(dir))?)
}

/// Whether an issue (by [`issue_key`]) uses the `images/publish/` layout.
/// Unrecognised paths get the current rules.
pub(crate) fn publish_era(key: Option<u32>) -> bool {
    key.is_none_or(|k| k >= PUBLISH_FROM)
}

/// `Ok(None)` when the issue has no `images/cards.toml`. The layout (flat
/// `images/` or `images/publish/`) follows from the issue's date.
pub fn load_spec(images: &Path) -> Result<Option<Spec>, String> {
    let path = images.join(SPEC_FILE);
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    let mut spec: Spec = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    // Resolve `.` and relative paths so the YYMM/MMDD components are visible.
    let issue = images
        .parent()
        .map(|p| p.canonicalize().unwrap_or_else(|_| p.to_path_buf()));
    spec.publish = publish_era(issue.as_deref().and_then(issue_key));
    Ok(Some(spec))
}

const USAGE: &str = "用法：barking card <期次目录> [输出文件名...|--all|--check]";

pub fn run(args: &[String]) -> ExitCode {
    let all = args.iter().any(|a| a == "--all");
    let check = args.iter().any(|a| a == "--check");
    let rest: Vec<&String> = args
        .iter()
        .filter(|a| *a != "--all" && *a != "--check")
        .collect();
    let Some((dir, only)) = rest.split_first() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    if rest.iter().any(|a| a.starts_with("--")) || ((all || check) && !only.is_empty()) {
        eprintln!("{USAGE}（--all、--check 与文件名不能同时用）");
        return ExitCode::from(2);
    }
    if check {
        return match check_issue(Path::new(dir.as_str())) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let only: Vec<&str> = only.iter().map(|s| s.as_str()).collect();
    match render_issue(Path::new(dir.as_str()), &only, all) {
        Ok(r) if r.rendered == 0 => {
            println!(
                "-- {} 张输入均未变，没有重渲（--all 强制全部重渲）",
                r.skipped
            );
            ExitCode::SUCCESS
        }
        Ok(r) => {
            println!(
                "-- 已渲染 {} 张，跳过 {} 张。逐张打开核对，再按 templates/cards/README.md 做视觉复核",
                r.rendered, r.skipped
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

struct Outcome {
    rendered: usize,
    skipped: usize,
}

/// Post on `Ok`: every selected output was rendered, or skipped as unchanged.
/// On `Err`: either nothing was rendered (pre-check problems, all listed), or
/// every card that could render did and the message lists each one that
/// failed.
fn render_issue(dir: &Path, only: &[&str], all: bool) -> Result<Outcome, String> {
    let images = dir.join("images");
    let spec = load_spec(&images)?.ok_or_else(|| {
        format!(
            "{} 不存在（样例见 templates/cards/example.toml）",
            images.join(SPEC_FILE).display()
        )
    })?;
    // In the publish layout a bare `03-x.png` means `publish/03-x.png`.
    let only: Vec<String> = only.iter().map(|o| spec.output_name(o)).collect();
    let only: Vec<&str> = only.iter().map(String::as_str).collect();
    let only = only.as_slice();
    let problems = precheck(&spec, &images, only);
    if !problems.is_empty() {
        return Err(format!(
            "cards.toml 有 {} 处问题，未渲染任何卡片：\n{}",
            problems.len(),
            problems.join("\n")
        ));
    }
    for w in spec.structure_warnings() {
        println!("note: {w}");
    }
    let avatar = match spec.kind {
        Kind::Roundup => Some(repo_root(dir)?.join(AVATAR)),
        Kind::Main => None,
    };
    let tool = tool_hash();
    if tool.is_none() {
        println!("（读不到程序自身，无法判断哪些卡片未变，全部重渲）");
    }
    let cache_file = cache_path(&images);
    // Held until this run returns: a concurrent run could otherwise write
    // back cache entries that this one has invalidated.
    let _lock = cache_file.as_deref().map(Lock::acquire).transpose()?;
    let mut cache = match cache_file.as_deref() {
        Some(f) => Cache::load(f)?,
        None => Cache::default(),
    };
    // Tesseract build and model per language setting, looked up on first need;
    // `None` when they cannot be identified.
    let mut ocr_ids: BTreeMap<&str, Option<String>> = BTreeMap::new();

    let mut todo = Vec::new();
    let mut skipped = Vec::new();
    for (out, job) in spec.jobs() {
        if !only.is_empty() && !only.contains(&out.as_str()) {
            continue;
        }
        let ocr = match job {
            Job::Annot(a) => ocr_ids
                .entry(a.lang.as_str())
                .or_insert_with(|| {
                    let id = ocr_identity(&a.lang);
                    if id.is_none() {
                        println!(
                            "（认不出 Tesseract 或语言包 {}，用它的批注图不记缓存、每次重渲）",
                            a.lang
                        );
                    }
                    id
                })
                .as_deref(),
            _ => Some(""),
        };
        // No fingerprint, no caching: the card always renders.
        let fp = (tool.as_deref())
            .zip(ocr)
            .map(|(t, ocr)| spec.fingerprint(&job, &images, avatar.as_deref(), t, ocr));
        let png = images.join(&out);
        let unchanged = fp.is_some() && cache.get(&out) == fp.as_ref() && png.is_file();
        if only.is_empty() && !all && unchanged {
            // Keeps lint's mtime check ("older than cards.toml") meaningful.
            if let Err(e) = touch(&png) {
                println!(
                    "（无法更新 {} 的修改时间，lint 可能误报早于 cards.toml：{e}）",
                    png.display()
                );
            }
            skipped.push(out);
        } else {
            todo.push((out, job, fp));
        }
    }
    let whole = only.is_empty();
    if !skipped.is_empty() {
        println!(
            "跳过 {} 张（输入未变）：{}",
            skipped.len(),
            skipped.join("、")
        );
    }
    if todo.is_empty() {
        if whole {
            clear_leftovers(&spec, &images, &mut cache, cache_file.as_deref());
        }
        return Ok(Outcome {
            rendered: 0,
            skipped: skipped.len(),
        });
    }

    let chrome = find_chrome()?;
    // Per-process scratch, so concurrent runs don't overwrite each other.
    let work = std::env::temp_dir()
        .join("barking-card")
        .join(std::process::id().to_string());
    fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let out_dir = spec.out_dir(&images);
    fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    let mut failed = Vec::new();
    let mut rendered = 0;
    for (out, job, fp) in todo {
        let result = render_one(&mut cache, cache_file.as_deref(), &out, fp, || {
            let (html, size) = job_html(&spec, job, &images, &work, avatar.as_deref())?;
            render(&chrome, &work, &html, size, &images.join(&out))
        });
        match result {
            Ok(()) => rendered += 1,
            Err(e) => failed.push(format!("{out}：{e}")),
        }
    }
    if !failed.is_empty() {
        return Err(format!(
            "{} 张失败（另有 {rendered} 张已渲染，{} 张跳过；旧编号的残留文件待全部渲染成功后再清）：\n{}",
            failed.len(),
            skipped.len(),
            failed.join("\n")
        ));
    }
    if whole {
        clear_leftovers(&spec, &images, &mut cache, cache_file.as_deref());
    }
    Ok(Outcome {
        rendered,
        skipped: skipped.len(),
    })
}

/// Files in `images/publish/` (names only), sorted; sub-directories skipped.
fn publish_files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(Result::ok)
                .filter(|e| e.path().is_file())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// After a whole-issue run in the publish layout: deletes the files of this
/// tool's own naming that are no longer outputs (a changed `post` or order
/// renumbers files; the old numbers would otherwise be uploaded too), forgets
/// their cache entries, and says what it did. Files it cannot account for are
/// only reported. Never touches `00-cover*`.
fn clear_leftovers(spec: &Spec, images: &Path, cache: &mut Cache, cache_file: Option<&Path>) {
    if !spec.publish {
        return;
    }
    let dir = spec.out_dir(images);
    let (stale, unknown) = spec.leftovers(&publish_files(&dir));
    let mut removed = Vec::new();
    for name in stale {
        match fs::remove_file(dir.join(&name)) {
            Ok(()) => {
                cache.remove(&format!("{PUBLISH_DIR}/{name}"));
                removed.push(name);
            }
            Err(e) => println!(
                "warn: 删不掉旧编号的 {PUBLISH_DIR}/{name}：{e}（它已不在输出清单里，手动清掉）"
            ),
        }
    }
    if !removed.is_empty() {
        println!(
            "删除 {} 个旧编号的残留（已不在输出清单里）：{}",
            removed.len(),
            removed
                .iter()
                .map(|n| format!("{PUBLISH_DIR}/{n}"))
                .collect::<Vec<_>>()
                .join("、")
        );
        if let Some(file) = cache_file
            && let Err(e) = cache.save(file)
        {
            println!("（缓存写入失败，下次将重渲：{e}）");
        }
    }
    for name in unknown {
        println!(
            "warn: {PUBLISH_DIR}/{name} 不是任何输出，也不是本工具的旧产物，未动它；不上传的图不要放在 publish/"
        );
    }
}

/// `barking card <dir> --check`: the plan, read-only. Runs the checks that
/// need neither Chrome nor OCR, lists every output with its path and whether
/// the file exists, and names what a full run would delete. Writes nothing,
/// takes no lock and leaves the render cache alone.
fn check_issue(dir: &Path) -> Result<(), String> {
    let images = dir.join("images");
    let spec = load_spec(&images)?
        .ok_or_else(|| format!("{} 不存在", images.join(SPEC_FILE).display()))?;
    let layout = if spec.publish {
        "images/publish/（1029 期起）"
    } else {
        "images/（1028 及更早期次）"
    };
    println!("版式：{layout}");
    let problems = precheck(&spec, &images, &[]);
    for p in &problems {
        println!("error: {p}");
    }
    for w in spec.structure_warnings() {
        println!("note: {w}");
    }
    for (name, job) in spec.items() {
        let state = match job {
            _ if images.join(&name).is_file() => "已存在",
            Job::Picked => "缺（生图选定后复制进来）",
            _ => "待渲染",
        };
        println!("{name}\t{state}");
    }
    if spec.publish {
        let (stale, unknown) = spec.leftovers(&publish_files(&spec.out_dir(&images)));
        for n in stale {
            println!("{PUBLISH_DIR}/{n}\t旧编号残留，整期渲染时会删");
        }
        for n in unknown {
            println!("{PUBLISH_DIR}/{n}\t未知文件，不会动它（lint 会报错）");
        }
    }
    if problems.is_empty() {
        println!("-- 检查通过，未渲染、未改动任何文件");
        Ok(())
    } else {
        Err(format!("cards.toml 有 {} 处问题", problems.len()))
    }
}

/// The page for one output and its viewport size.
fn job_html(
    spec: &Spec,
    job: Job,
    images: &Path,
    work: &Path,
    avatar: Option<&Path>,
) -> Result<(String, (u32, u32)), String> {
    match job {
        Job::Picked => Err("主帖封面由生图得来，不由 barking card 渲染".into()),
        Job::Cover(size) => {
            let avatar = file_url(avatar.ok_or("速览专帖封面缺少看板娘路径")?)?;
            Ok((cover_html(spec, size, &avatar)?, size))
        }
        Job::Tldr => Ok((tldr_html(spec)?, CARD_SIZE)),
        Job::Roundup => Ok((roundup_html(spec)?, CARD_SIZE)),
        Job::Event(i) => Ok((event_html(spec, i)?, CARD_SIZE)),
        Job::Bark(i) => Ok((bark_html(spec, i)?, CARD_SIZE)),
        Job::Annot(a) => Ok((annot_html(spec, a, images, work)?, CARD_SIZE)),
    }
}

/// Every problem decidable without Chrome or OCR, over the whole file, so one
/// editing round can fix them all. Empty means rendering may start.
fn precheck(spec: &Spec, images: &Path, only: &[&str]) -> Vec<String> {
    let mut errs = Vec::new();
    for at in spec.retired_barks() {
        errs.push(format!(
            "{at} 已停用：2026-10-03 起省流卡与批注图不放吠点，2026-10-28 起吠点改写在吠点卡 [[bark]]；从 cards.toml 删去"
        ));
    }
    errs.extend(spec.structure_errors());
    let outputs = spec.outputs();
    if outputs.is_empty() {
        errs.push("cards.toml 里没有要渲染的卡片".into());
    }
    for (i, out) in outputs.iter().enumerate() {
        if leaf_name(out).contains(['/', '\\']) || !out.ends_with(".png") {
            errs.push(format!(
                "输出名「{out}」须为 images/ 下的 .png 文件名，不含路径"
            ));
        } else if (outputs[..i].iter()).any(|o| o.to_lowercase() == out.to_lowercase()) {
            // Windows file names ignore case: both would write one file.
            errs.push(format!("输出名「{out}」与前面的重复（文件名不分大小写）"));
        }
    }
    let unknown: Vec<&&str> = only
        .iter()
        .filter(|o| !outputs.iter().any(|out| out == **o))
        .collect();
    if !unknown.is_empty() {
        errs.push(format!("cards.toml 中没有这些输出：{unknown:?}"));
    }
    let dated = spec.kind == Kind::Roundup
        || !spec.tldr.is_empty()
        || !spec.roundup.is_empty()
        || !spec.event.is_empty()
        || !spec.bark.is_empty();
    if dated && spec.date.is_none() {
        errs.push(
            "cards.toml 缺少 date（省流卡、速览聚合图、事件卡、吠点卡与速览专帖封面都要写日期）"
                .into(),
        );
    }
    for (name, job) in spec.jobs() {
        let Job::Annot(a) = job else { continue };
        // Base images are git-ignored and may be absent from a fresh checkout;
        // when rendering named outputs, only theirs must be present.
        let selected = only.is_empty() || only.contains(&name.as_str());
        errs.extend(
            annot_problems(spec, a, images, selected)
                .into_iter()
                .map(|e| format!("{name}：{e}")),
        );
    }
    errs
}

/// What is wrong with one [[annot]] short of OCR. Empty means `annot_html`
/// can rely on: 1–4 notes, each with exactly one of quote/rects, rects inside
/// the base image, underline only with quote, emph inside its gloss. Without
/// `with_raw` the base image is not read, and rects go unchecked against it.
fn annot_problems(spec: &Spec, a: &Annot, images: &Path, with_raw: bool) -> Vec<String> {
    let mut errs = Vec::new();
    if a.note.is_empty() || a.note.len() > MAX_NOTES {
        errs.push(format!("每张批注图需 1–{MAX_NOTES} 条 note"));
    }
    // A non-.png `out` has no base image; the output-name check reports it.
    let size = spec
        .raw_path(a, images)
        .filter(|_| with_raw)
        .and_then(|raw| png_size(&raw).map_err(|e| errs.push(e)).ok());
    for (i, note) in a.note.iter().enumerate() {
        let c = i + 1;
        match (&note.rects, &note.quote) {
            (Some(_), Some(_)) => {
                errs.push(format!("第 {c} 条 note 的 quote 与 rects 只能二选一"));
            }
            (None, None) => errs.push(format!("第 {c} 条 note 需要 quote 或 rects")),
            (None, Some(q)) => {
                if tokens(q).is_empty() {
                    errs.push(format!("第 {c} 条 quote 没有可匹配的文字"));
                }
            }
            (Some(r), None) => {
                if r.is_empty() {
                    errs.push(format!("第 {c} 条 note 的 rects 为空"));
                }
                let bad = |&&[x, y, rw, rh]: &&[u32; 4], (w, h): (u32, u32)| {
                    rw == 0 || rh == 0 || x.saturating_add(rw) > w || y.saturating_add(rh) > h
                };
                if let Some(size) = size
                    && let Some(r) = r.iter().find(|r| bad(r, size))
                {
                    errs.push(format!(
                        "第 {c} 条 rects {r:?} 为空或超出底图 {}×{}",
                        size.0, size.1
                    ));
                }
                if note.underline.is_some() {
                    errs.push(format!(
                        "第 {c} 条用了 rects，underline 无法核对位置；改用 quote"
                    ));
                }
            }
        }
        if let Some(e) = &note.emph
            && !note.gloss.contains(e.as_str())
        {
            errs.push(format!("第 {c} 条 emph「{e}」不在 gloss 中"));
        }
    }
    errs
}

/// SHA-256 of the running executable: templates and matching rules are
/// compiled in, so any rebuild may change what a card looks like.
fn tool_hash() -> Option<String> {
    let bytes = fs::read(std::env::current_exe().ok()?).ok()?;
    Some(hex(&Sha256::digest(&bytes)))
}

/// `<temp>/barking-card/cache/<hash of the images dir>.json`; the module docs
/// say why not `images/`.
fn cache_path(images: &Path) -> Option<PathBuf> {
    let abs = fs::canonicalize(images)
        .or_else(|_| std::path::absolute(images))
        .ok()?;
    let key = hex(&Sha256::digest(abs.to_string_lossy().as_bytes()));
    Some(
        std::env::temp_dir()
            .join("barking-card")
            .join("cache")
            .join(format!("{}.json", &key[..16])),
    )
}

/// Output name → input fingerprint of its last successful render. Keys are
/// lowercased output names: they are file names, and Windows ignores case,
/// so `01-x.png` and `01-X.png` are one file and must be one entry.
#[derive(Default, Debug, PartialEq)]
struct Cache(BTreeMap<String, String>);

impl Cache {
    /// A missing cache is empty: everything renders. One that exists but
    /// cannot be read or parsed is an error: going on with an empty cache
    /// would silently rewrite it, and something is wrong with the temp dir.
    /// Keys that differ only in case (written before keys were lowercased)
    /// are all dropped, since which one describes the file on disk is unknown.
    fn load(path: &Path) -> Result<Cache, String> {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Cache::default()),
            Err(e) => {
                return Err(format!(
                    "读不了渲染缓存 {}：{e}。查明原因，或删掉它（下次全部重渲）",
                    path.display()
                ));
            }
        };
        let raw: BTreeMap<String, String> = serde_json::from_str(&text).map_err(|e| {
            format!(
                "渲染缓存 {} 已损坏：{e}。删掉它即可（下次全部重渲）",
                path.display()
            )
        })?;
        let mut variants: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (k, v) in raw {
            variants.entry(k.to_lowercase()).or_default().push(v);
        }
        Ok(Cache(
            variants
                .into_iter()
                .filter_map(|(k, mut vs)| (vs.len() == 1).then(|| (k, vs.remove(0))))
                .collect(),
        ))
    }

    fn save(&self, path: &Path) -> Result<(), String> {
        let dir = path.parent().ok_or("缓存路径无上级目录")?;
        fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        // Write then rename, so an interrupted run never leaves half a file.
        let tmp = path.with_extension(format!("{}.tmp", std::process::id()));
        fs::write(&tmp, json(&self.0)).map_err(|e| format!("{}: {e}", tmp.display()))?;
        fs::rename(&tmp, path).map_err(|e| format!("{}: {e}", path.display()))
    }

    fn get(&self, out: &str) -> Option<&String> {
        self.0.get(&out.to_lowercase())
    }

    fn remove(&mut self, out: &str) -> bool {
        self.0.remove(&out.to_lowercase()).is_some()
    }

    fn insert(&mut self, out: &str, fp: String) {
        self.0.insert(out.to_lowercase(), fp);
    }
}

/// Renders one card with the cache kept honest. Its entry is dropped, and the
/// drop saved, before `render` may replace the image: a run that dies from
/// then on leaves no entry, so the next run renders the card again instead of
/// trusting a half-written or stale PNG. If that save fails, `render` is not
/// called. The new fingerprint is recorded only after `render` succeeds.
fn render_one(
    cache: &mut Cache,
    file: Option<&Path>,
    out: &str,
    fp: Option<String>,
    render: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    if cache.remove(out)
        && let Some(file) = file
    {
        cache
            .save(file)
            .map_err(|e| format!("缓存写入失败，未覆盖旧图：{e}"))?;
    }
    render()?;
    if let (Some(fp), Some(file)) = (fp, file) {
        cache.insert(out, fp);
        if let Err(e) = cache.save(file) {
            println!("（缓存写入失败，下次将重渲：{e}）");
        }
    }
    Ok(())
}

/// One `barking card` run per images directory at a time: a lock file next to
/// the cache, created exclusively and holding the owner's PID, removed when
/// the run ends. A killed run (Ctrl+C) leaves it behind; the error says how
/// to clear it. No liveness check: std has no portable one.
struct Lock(PathBuf);

impl Lock {
    fn acquire(cache: &Path) -> Result<Lock, String> {
        let path = cache.with_extension("lock");
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        match fs::File::options().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                let _ = write!(f, "{}", std::process::id());
                Ok(Lock(path))
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                let pid = fs::read_to_string(&path).unwrap_or_default();
                Err(format!(
                    "另一个 barking card 进程（PID {}）正在处理这一期的卡片。若它已结束（例如被中断），删除锁文件 {} 后重试",
                    pid.trim(),
                    path.display()
                ))
            }
            Err(e) => Err(format!("{}: {e}", path.display())),
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// The Tesseract on PATH: its version line and the tessdata directory it
/// reports in `--list-langs` (whatever TESSDATA_PREFIX resolves to). Probed
/// once per process; every OCR call passes this directory explicitly, so the
/// model hashed into the cache is the model OCR loads.
struct Tess {
    version: String,
    dir: PathBuf,
}

fn tess() -> Option<&'static Tess> {
    static TESS: std::sync::OnceLock<Option<Tess>> = std::sync::OnceLock::new();
    TESS.get_or_init(|| {
        let run = |arg: &str| {
            let o = Command::new("tesseract").arg(arg).output().ok()?;
            Some(Probe {
                ok: o.status.success(),
                stdout: String::from_utf8_lossy(&o.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&o.stderr).into_owned(),
            })
        };
        parse_tess(&run("--version")?, &run("--list-langs")?)
    })
    .as_ref()
}

/// What one `tesseract` probe printed.
struct Probe {
    ok: bool,
    stdout: String,
    stderr: String,
}

/// `Tess` from the `--version` and `--list-langs` probes, or `None` unless
/// both exited 0, the first line printed by `--version` reads `tesseract
/// <version>` (Tesseract prints it to stdout, some builds to stderr), and
/// `--list-langs` has its `List of available languages in "<dir>" (N):`
/// header. Anything else means a different program or a broken install, and
/// then nothing that depends on it is cached.
fn parse_tess(version: &Probe, langs: &Probe) -> Option<Tess> {
    if !version.ok || !langs.ok {
        return None;
    }
    let first_line = |p: &Probe| {
        (p.stdout.lines().chain(p.stderr.lines()))
            .map(str::trim)
            .find(|l| !l.is_empty())
            .map(str::to_owned)
    };
    let line = first_line(version)?;
    if !is_version_line(&line) {
        return None;
    }
    let dir = (langs.stdout.lines().chain(langs.stderr.lines())).find_map(tessdata_dir)?;
    Some(Tess { version: line, dir })
}

/// `tesseract v?N(.N)*`, optionally followed by `-` or `+` and a non-empty
/// suffix of ASCII letters, digits, `.` and `-` (`tesseract 5.3.0-1-gabc`).
/// Every numeric component must be non-empty: `tesseract 5...` is refused.
fn is_version_line(line: &str) -> bool {
    let Some(v) = line.strip_prefix("tesseract ") else {
        return false;
    };
    let v = v.strip_prefix('v').unwrap_or(v);
    let (number, suffix) = match v.find(['-', '+']) {
        Some(i) => (&v[..i], Some(&v[i + 1..])),
        None => (v, None),
    };
    let numeric = number
        .split('.')
        .all(|c| !c.is_empty() && c.chars().all(|d| d.is_ascii_digit()));
    let suffix_ok = suffix.is_none_or(|s| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || ".-".contains(c))
    });
    numeric && suffix_ok
}

/// Which Tesseract locates quotes for `lang` (`eng`, `eng+chi_sim`): its
/// version line plus a content hash of each `.traineddata` it loads. Content
/// rather than size and mtime: a reinstall that rewrites the same model must
/// not force re-OCR, and a swapped model must; the models are a few MB,
/// hashed once per language setting per run. `None` if Tesseract or a model
/// cannot be read: such cards are then never cached.
fn ocr_identity(lang: &str) -> Option<String> {
    let t = tess()?;
    let mut id = t.version.clone();
    for l in lang.split('+') {
        let model = fs::read(t.dir.join(format!("{l}.traineddata"))).ok()?;
        let _ = write!(id, "\n{l}:{}", hex(&Sha256::digest(&model)));
    }
    Some(id)
}

/// The directory in the header line `List of available languages in "<dir>"
/// (N):`, if `line` is that header with a non-empty directory.
fn tessdata_dir(line: &str) -> Option<PathBuf> {
    let rest = line
        .trim()
        .strip_prefix("List of available languages in \"")?;
    let (dir, count) = rest.split_once("\" (")?;
    let n = count.strip_suffix("):")?;
    let counted = !n.is_empty() && n.chars().all(|c| c.is_ascii_digit());
    (counted && !dir.is_empty()).then(|| PathBuf::from(dir))
}

fn touch(path: &Path) -> io::Result<()> {
    fs::File::options()
        .write(true)
        .open(path)?
        .set_modified(SystemTime::now())
}

fn tldr_html(spec: &Spec) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[tldr]] 但缺少 date")?;
    let mut items = String::new();
    for (i, t) in spec.tldr.iter().enumerate() {
        // Colour cycles through the four highlighter colours (.c1 … .c4), as the
        // note markers on annotated screenshots do.
        let _ = writeln!(
            items,
            r#"    <div class="item c{}"><span class="k">{}</span><div class="fact">{}</div></div>"#,
            i % MAX_NOTES + 1,
            esc(&t.tag),
            esc(&t.fact)
        );
    }
    Ok(TLDR_TEMPLATE
        .replace("{{DATE}}", &esc(date))
        .replace("{{ITEMS}}", &items))
}

fn roundup_html(spec: &Spec) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[roundup]] 但缺少 date")?;
    let mut items = String::new();
    for (i, r) in spec.roundup.iter().enumerate() {
        let (class, see) = if r.main {
            (" main", r#"<span class="see">详见前页</span>"#)
        } else {
            ("", "")
        };
        let _ = writeln!(
            items,
            r#"    <div class="item c{}{class}"><div class="n">{}</div><div class="tx"><span class="k">{}</span>{}{see}</div></div>"#,
            i % MAX_NOTES + 1,
            i + 1,
            esc(&r.tag),
            esc(&r.text)
        );
    }
    Ok(ROUNDUP_TEMPLATE
        .replace("{{DATE}}", &esc(date))
        .replace("{{ITEMS}}", &items))
}

/// The 事件卡 of `spec.event[i]`: its colour is the 省流卡 colour of the same
/// main post, so the cards of one topic read as one.
fn event_html(spec: &Spec, i: usize) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[event]] 但缺少 date")?;
    let e = spec.event.get(i).ok_or("没有这张事件卡")?;
    let heading = spec
        .event_heading(e)
        .ok_or_else(|| format!("事件卡「{}」在速览里没有同 tag 的主帖条目", e.tag))?;
    let mut items = String::new();
    for p in &e.paras {
        let _ = writeln!(
            items,
            r#"    <div class="pt"><div class="{}">{}</div></div>"#,
            hang("tx", p.trim()),
            esc(p.trim())
        );
    }
    Ok(EVENT_TEMPLATE
        .replace("{{CLASS}}", &format!("c{}", i % MAX_NOTES + 1))
        .replace("{{DATE}}", &esc(date))
        .replace("{{TAG}}", &esc(&e.tag))
        .replace("{{TITLE}}", &esc(heading))
        .replace("{{ITEMS}}", &items))
}

/// The 吠点卡 of `spec.bark[i]`: its colour is the 省流卡 colour of the same
/// main post, so the two read as one topic.
fn bark_html(spec: &Spec, i: usize) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 有 [[bark]] 但缺少 date")?;
    let b = spec.bark.get(i).ok_or("没有这张吠点卡")?;
    let heading = spec
        .bark_heading(b)
        .ok_or_else(|| format!("吠点卡「{}」在速览里没有同 tag 的主帖条目", b.tag))?;
    let mut items = String::new();
    for (k, p) in b.points.iter().enumerate() {
        let n = circled(k).ok_or("一张吠点卡最多 20 条")?;
        let _ = writeln!(
            items,
            r#"    <div class="pt"><span class="n">{n}</span><div class="{}">{}</div></div>"#,
            hang("tx", p.trim()),
            esc(p.trim())
        );
    }
    Ok(BARK_TEMPLATE
        .replace("{{CLASS}}", &format!("c{}", i % MAX_NOTES + 1))
        .replace("{{DATE}}", &esc(date))
        .replace("{{TAG}}", &esc(&b.tag))
        .replace("{{TITLE}}", &esc(heading))
        .replace("{{ITEMS}}", &items))
}

/// ① for 0, ② for 1, … up to ⑳.
fn circled(k: usize) -> Option<char> {
    u32::try_from(k)
        .ok()
        .filter(|&k| k < 20)
        .and_then(|k| char::from_u32(0x2460 + k))
}

/// `s` without its leading circled number (① … ⑳), if it has one.
pub(crate) fn strip_circled(s: &str) -> Option<&str> {
    let c = s.chars().next()?;
    ('\u{2460}'..='\u{2473}')
        .contains(&c)
        .then(|| &s[c.len_utf8()..])
}

/// One of the two template covers of a 速览专帖. It shows only the date, the
/// entry count and the tags, so it carries no claim of its own.
fn cover_html(spec: &Spec, (w, h): (u32, u32), avatar: &str) -> Result<String, String> {
    let date = spec
        .date
        .as_deref()
        .ok_or("cards.toml 缺少 date（封面要写日期）")?;
    let mut tags: Vec<&str> = Vec::new();
    for r in &spec.roundup {
        if !tags.contains(&r.tag.as_str()) {
            tags.push(&r.tag);
        }
    }
    let tags: String = tags
        .iter()
        .map(|t| format!("<span>{}</span>", esc(t)))
        .collect();
    let class = if w > h { "wide" } else { "tall" };
    Ok(COVER_TEMPLATE
        .replace("{{CLASS}}", class)
        .replace("{{W}}", &w.to_string())
        .replace("{{H}}", &h.to_string())
        .replace("{{DATE}}", &esc(date))
        .replace("{{COUNT}}", &spec.roundup.len().to_string())
        .replace("{{TAGS}}", &tags)
        .replace("{{AVATAR}}", &esc(avatar)))
}

/// The repository root: the nearest ancestor of `dir` holding EDITORIAL.md.
pub(crate) fn repo_root(dir: &Path) -> Result<PathBuf, String> {
    let abs = std::path::absolute(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    abs.ancestors()
        .find(|p| p.join("EDITORIAL.md").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("{} 不在 AI-Barking 仓库内", dir.display()))
}

fn annot_html(spec: &Spec, a: &Annot, images: &Path, work: &Path) -> Result<String, String> {
    // Normally already reported by `precheck`; repeated so this function
    // never relies on a caller for its own invariants.
    if let Some(e) = annot_problems(spec, a, images, true).into_iter().next() {
        return Err(e);
    }
    let raw = spec.raw_path(a, images).ok_or("out 须为 .png 文件名")?;
    let (w, h) = png_size(&raw)?;
    // OCR runs at most once per image, on first need.
    let mut ocr: Option<Vec<Word>> = None;

    // Pass 1: highlight and underline boxes per note, in raw-image pixels.
    // Every note is tried, so one round shows all quotes that fail to match.
    let mut hls: Vec<Vec<Rect>> = Vec::with_capacity(a.note.len());
    // Underlines: (box, baseline).
    let mut uls: Vec<(Rect, u32)> = Vec::new();
    let mut errs: Vec<String> = Vec::new();
    for (i, note) in a.note.iter().enumerate() {
        let c = i + 1;
        let (rects, quote_words) = match (&note.rects, &note.quote) {
            (Some(r), _) => (
                r.iter().map(|&[x, y, w, h]| Rect { x, y, w, h }).collect(),
                None,
            ),
            (None, Some(q)) => match locate(ocr_words(&raw, w, &a.lang, work, &mut ocr)?, q) {
                Ok(found) => (
                    found.lines.into_iter().map(|(r, _)| r).collect(),
                    Some(found.words),
                ),
                Err(e) => {
                    errs.push(format!("第 {c} 条 quote：{e}"));
                    continue;
                }
            },
            (None, None) => unreachable!("annot_problems requires quote or rects"),
        };
        hls.push(rects);
        if let (Some(u), Some(quote_words)) = (&note.underline, quote_words) {
            match locate(ocr_words(&raw, w, &a.lang, work, &mut ocr)?, u) {
                Ok(found) if found.words.iter().all(|i| quote_words.contains(i)) => {
                    uls.extend(found.lines);
                }
                Ok(_) => errs.push(format!("第 {c} 条 underline「{u}」不在本条 quote 范围内")),
                Err(e) => errs.push(format!("第 {c} 条 underline：{e}")),
            }
        }
    }
    if !errs.is_empty() {
        return Err(errs.join("；"));
    }
    split_overlaps(&mut hls);

    // Pass 2: emit. Markers go outside the screenshot's left edge, at each
    // note's first line; the page script spaces them once the scale is known.
    let mut overlays = String::new();
    for (i, rects) in hls.iter().enumerate() {
        let c = i + 1;
        for r in rects {
            let _ = writeln!(
                overlays,
                r#"    <div class="hl c{c}" style="left:{}px; top:{}px; width:{}px; height:{}px"></div>"#,
                r.x, r.y, r.w, r.h
            );
        }
        let y = rects[0].y + rects[0].h / 2;
        let _ = writeln!(
            overlays,
            r#"    <div class="mk c{c}" data-y="{y}" style="top:{y}px">{c}</div>"#
        );
    }
    // Drawn just under the baseline, not under descenders, so it stays clear
    // of the next line in tightly set paragraphs.
    for (r, baseline) in &uls {
        let _ = writeln!(
            overlays,
            r#"    <div class="ul" style="left:{}px; top:{}px; width:{}px"></div>"#,
            r.x,
            baseline + 1,
            r.w
        );
    }

    let mut notes = String::new();
    for (i, note) in a.note.iter().enumerate() {
        let c = i + 1;
        let gloss = match &note.emph {
            // `annot_problems` checked that `e` is in the gloss.
            Some(e) => esc(&note.gloss).replacen(&esc(e), &format!("<em>{}</em>", esc(e)), 1),
            None => esc(&note.gloss),
        };
        let _ = writeln!(
            notes,
            r#"    <div class="row"><div class="n c{c}">{c}</div><div class="{}">{gloss}</div></div>"#,
            hang("tx", &note.gloss)
        );
    }
    // Every inserted value is escaped (braces included), so no value can
    // smuggle in a later placeholder.
    Ok(ANNOT_TEMPLATE
        .replace("{{TITLE}}", &esc(&a.title))
        .replace("{{IMG}}", &file_url(&raw)?)
        .replace("{{W}}", &w.to_string())
        .replace("{{H}}", &h.to_string())
        .replace("{{OVERLAYS}}", &overlays)
        .replace("{{NOTES}}", &notes)
        .replace("{{SOURCE}}", &esc(&a.source)))
}

/// Class list for a text block; a leading full-width quote gets the hanging class.
fn hang(class: &str, text: &str) -> String {
    if text.starts_with('“') {
        format!("{class} q")
    } else {
        class.to_string()
    }
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '{' => out.push_str("&#123;"),
            '}' => out.push_str("&#125;"),
            _ => out.push(c),
        }
    }
    out
}

/// Width and height from a PNG's IHDR chunk.
pub(crate) fn png_size(path: &Path) -> Result<(u32, u32), String> {
    let bytes = fs::read(path).map_err(|e| format!("底图 {}: {e}", path.display()))?;
    if bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return Err(format!("{} 不是 PNG", path.display()));
    }
    let be = |i: usize| u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
    Ok((be(16), be(20)))
}

pub(crate) fn file_url(path: &Path) -> Result<String, String> {
    let abs = std::path::absolute(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let s = abs.to_string_lossy().replace('\\', "/");
    let mut url = String::from("file:///");
    for b in s.trim_start_matches('/').bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/:".contains(&b) {
            url.push(b as char);
        } else {
            let _ = write!(url, "%{b:02X}");
        }
    }
    Ok(url)
}

pub(crate) fn find_chrome() -> Result<PathBuf, String> {
    if let Some(p) = std::env::var_os("BARKING_CHROME") {
        return Ok(PathBuf::from(p));
    }
    [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
    ]
    .iter()
    .map(PathBuf::from)
    .find(|p| p.is_file())
    .ok_or_else(|| "找不到 Chrome / Edge；用环境变量 BARKING_CHROME 指定可执行文件".into())
}

pub(crate) fn chrome_cmd(chrome: &Path, work: &Path, (w, h): (u32, u32)) -> Command {
    let mut cmd = Command::new(chrome);
    cmd.args([
        "--headless=new",
        "--disable-gpu",
        "--hide-scrollbars",
        "--force-device-scale-factor=1",
        "--allow-file-access-from-files",
    ])
    .arg(format!("--window-size={w},{h}"))
    // Own profile: never hand the job to a running Chrome.
    .arg(format!(
        "--user-data-dir={}",
        work.join("profile").display()
    ))
    .stderr(Stdio::null());
    cmd
}

fn render(
    chrome: &Path,
    work: &Path,
    html: &str,
    size: (u32, u32),
    out: &Path,
) -> Result<(), String> {
    let name = out.file_stem().and_then(|n| n.to_str()).unwrap_or("card");
    let page = work.join(format!("{name}.html"));
    fs::write(&page, html).map_err(|e| format!("{}: {e}", page.display()))?;
    let url = file_url(&page)?;

    // The page script records `data-layout` on <body>: "ok" or "overflow:…".
    let dom = chrome_cmd(chrome, work, size)
        .arg("--dump-dom")
        .arg(&url)
        .output()
        .map_err(|e| format!("无法运行 {}: {e}", chrome.display()))?;
    let dom = String::from_utf8_lossy(&dom.stdout);
    match layout_state(&dom) {
        Some("ok") => {}
        Some(bad) => {
            return Err(format!(
                "{}：版面溢出（{}）。HTML 留在 {}",
                out.display(),
                bad.trim_start_matches("overflow:"),
                page.display()
            ));
        }
        None => {
            return Err(format!(
                "{}：读不到版面检查结果（HTML 留在 {}）",
                out.display(),
                page.display()
            ));
        }
    }

    // Shoot into the scratch dir and replace the output only once the shot
    // exists, so a failed render never touches the previous image.
    let shot = std::path::absolute(work.join(format!("{name}.png")))
        .map_err(|e| format!("{}: {e}", out.display()))?;
    let _ = fs::remove_file(&shot);
    let started = SystemTime::now();
    let status = chrome_cmd(chrome, work, size)
        .arg(format!("--screenshot={}", shot.display()))
        .arg(&url)
        .stdout(Stdio::null())
        .status()
        .map_err(|e| format!("无法运行 {}: {e}", chrome.display()))?;
    // Chrome may exit 0 without writing; require a file newer than the launch.
    let fresh = fs::metadata(&shot)
        .and_then(|m| m.modified())
        .is_ok_and(|t| t >= started);
    if !status.success() || !fresh {
        return Err(format!(
            "渲染 {} 失败（HTML 留在 {}）",
            out.display(),
            page.display()
        ));
    }
    // Rename within a volume; the scratch dir is often on another drive, so
    // fall back to copying. An interrupted copy can leave a broken PNG, which
    // is safe because the caller has already dropped this card's cache entry.
    fs::rename(&shot, out)
        .or_else(|_| fs::copy(&shot, out).map(|_| ()))
        .map_err(|e| format!("{}: {e}", out.display()))?;
    println!("{}  （HTML：{}）", out.display(), page.display());
    Ok(())
}

/// The `data-layout` value on the dumped <body>, if present.
fn layout_state(dom: &str) -> Option<&str> {
    let start = dom.find("data-layout=\"")? + "data-layout=\"".len();
    let len = dom[start..].find('"')?;
    Some(&dom[start..start + len])
}

/// A Tesseract word with its box in raw-image pixels.
pub(crate) struct Word {
    pub(crate) text: String,
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) right: u32,
    pub(crate) bottom: u32,
    pub(crate) conf: f32,
    /// (block, paragraph, line) — words sharing it sit on one text line.
    pub(crate) line: (u32, u32, u32),
}

#[derive(Debug, PartialEq)]
struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

/// Trims vertically overlapping highlights (adjacent lines, possibly from
/// different notes) to meet halfway, so multiply blending never doubles up.
///
/// Post: no two rectangles that overlap horizontally overlap vertically,
/// provided no rectangle is nested inside another's vertical span.
fn split_overlaps(hls: &mut [Vec<Rect>]) {
    let ids: Vec<(usize, usize)> = hls
        .iter()
        .enumerate()
        .flat_map(|(n, rs)| (0..rs.len()).map(move |k| (n, k)))
        .collect();
    for &(an, ak) in &ids {
        for &(bn, bk) in &ids {
            let (a, b) = (&hls[an][ak], &hls[bn][bk]);
            let x_overlap = a.x < b.x + b.w && b.x < a.x + a.w;
            if (an, ak) == (bn, bk) || !x_overlap || a.y >= b.y || a.y + a.h <= b.y {
                continue;
            }
            // `a` is above `b` and runs into it.
            let mid = (a.y + a.h + b.y).div_ceil(2);
            let b_bottom = b.y + b.h;
            hls[an][ak].h = mid - hls[an][ak].y;
            hls[bn][bk].y = mid;
            hls[bn][bk].h = b_bottom.saturating_sub(mid);
        }
    }
}

/// Returns the cached OCR words, running Tesseract on first use. The slice
/// borrows from `cache`, which is why the cache is passed in by the caller.
fn ocr_words<'a>(
    raw: &Path,
    width: u32,
    lang: &str,
    work: &Path,
    cache: &'a mut Option<Vec<Word>>,
) -> Result<&'a [Word], String> {
    if cache.is_none() {
        *cache = Some(run_ocr(raw, width, lang, work)?);
    }
    Ok(cache.as_deref().unwrap_or_default())
}

pub(crate) fn run_ocr(
    raw: &Path,
    width: u32,
    lang: &str,
    work: &Path,
) -> Result<Vec<Word>, String> {
    // 1x screenshots read better upscaled; 2x captures are fine as they are.
    let factor: u32 = if width < 1000 { 3 } else { 1 };
    let input = if factor == 1 {
        raw.to_path_buf()
    } else {
        let tmp = work.join("ocr-input.png");
        let ok = Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-i"])
            .arg(raw)
            .args([
                "-vf",
                &format!("scale=iw*{factor}:ih*{factor}:flags=lanczos"),
            ])
            .arg(&tmp)
            .status()
            .map_err(|e| format!("无法运行 ffmpeg：{e}"))?
            .success();
        if !ok {
            return Err("ffmpeg 放大底图失败".into());
        }
        tmp
    };
    let mut cmd = Command::new("tesseract");
    cmd.arg(&input).arg("stdout");
    // The directory whose model `ocr_identity` hashed.
    if let Some(t) = tess() {
        cmd.arg("--tessdata-dir").arg(&t.dir);
    }
    let out = cmd
        .args(["--psm", "6", "-l", lang, "-c", "tessedit_create_tsv=1"])
        .output()
        .map_err(|e| {
            format!("无法运行 tesseract（scoop install tesseract tesseract-languages）：{e}")
        })?;
    if !out.status.success() {
        return Err(format!(
            "tesseract 失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(parse_tsv(&String::from_utf8_lossy(&out.stdout), factor))
}

/// Word rows (level 5) of Tesseract TSV, with boxes scaled back by `factor`.
pub(crate) fn parse_tsv(tsv: &str, factor: u32) -> Vec<Word> {
    tsv.lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 12 || f[0] != "5" || f[11].trim().is_empty() {
                return None;
            }
            let n = |i: usize| f[i].parse::<u32>().ok();
            let (l, t, w, h) = (n(6)?, n(7)?, n(8)?, n(9)?);
            Some(Word {
                text: f[11].trim().to_string(),
                left: l / factor,
                top: t / factor,
                right: (l + w).div_ceil(factor),
                bottom: (t + h).div_ceil(factor),
                conf: f[10].parse().ok()?,
                line: (n(2)?, n(3)?, n(4)?),
            })
        })
        .collect()
}

fn is_cjk(c: char) -> bool {
    matches!(c, '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}' | '\u{f900}'..='\u{faff}')
}

/// What a one-letter word `I` comes out as in OCR, lowercased: Tesseract
/// often reads a lone capital I in sans-serif text as `|`, `l` or `1`.
const I_LIKE: [char; 4] = ['i', 'l', '1', '|'];

/// Appends the match form of `c`: letters and digits lowercased, typographic
/// ligatures spelled out (an OCR `ﬁnd` reads `find`). Everything else adds
/// nothing: spaces, straight and curly quotes and apostrophes (' ‘ ’ ‚ ‛ " “
/// ” „), primes (′ ″), dashes (- ‐ ‑ – —) and other punctuation.
fn fold(c: char, out: &mut String) {
    let ligature = match c {
        'ﬀ' => "ff",
        'ﬁ' => "fi",
        'ﬂ' => "fl",
        'ﬃ' => "ffi",
        'ﬄ' => "ffl",
        'ﬅ' | 'ﬆ' => "st",
        _ => "",
    };
    if !ligature.is_empty() {
        out.push_str(ligature);
    } else if c.is_alphanumeric() {
        out.extend(c.to_lowercase());
    }
}

/// Match tokens: folded alphanumeric runs, and each CJK character alone.
/// Punctuation and spacing are ignored, so `later.` matches `later`.
fn tokens(s: &str) -> Vec<String> {
    let (mut out, mut cur) = (Vec::new(), String::new());
    for c in s.chars() {
        if c.is_alphanumeric() && !is_cjk(c) {
            fold(c, &mut cur);
            continue;
        }
        if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        if is_cjk(c) {
            out.push(c.to_string());
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// One matchable character of the OCR text.
struct Glyph {
    c: char,
    /// Index into the OCR words.
    word: usize,
    /// First / last character of a token: an alphanumeric run (`|` counts as
    /// a letter) or a lone CJK character.
    tok_start: bool,
    tok_end: bool,
}

/// The OCR text as one stream of folded characters. `|` is kept, as a
/// possible `I`; other punctuation is dropped.
fn glyphs(words: &[Word]) -> Vec<Glyph> {
    fn close(out: &mut [Glyph]) {
        if let Some(g) = out.last_mut() {
            g.tok_end = true;
        }
    }
    let mut out: Vec<Glyph> = Vec::new();
    for (i, w) in words.iter().enumerate() {
        // Inside a non-CJK token of this word.
        let mut open = false;
        for c in w.text.chars() {
            let mut f = String::new();
            if c == '|' {
                f.push(c);
            } else {
                fold(c, &mut f);
            }
            let alone = is_cjk(c);
            if open && (f.is_empty() || alone) {
                close(&mut out);
            }
            for (k, g) in f.chars().enumerate() {
                out.push(Glyph {
                    c: g,
                    word: i,
                    tok_start: k == 0 && (alone || !open),
                    tok_end: alone,
                });
            }
            open = !f.is_empty() && !alone;
        }
        if open {
            close(&mut out);
        }
    }
    out
}

/// One character of the quote.
struct QGlyph {
    c: char,
    /// Index into the quote's tokens.
    run: usize,
    /// The token is a single character.
    single: bool,
}

/// Whether quote character `q` reads as OCR glyph `g`. Exact after folding,
/// with one exception: a quote word that is exactly the letter `I` also
/// matches an `I_LIKE` glyph that starts an OCR token (a lone `|`, `l` or
/// `1`, or the head of a merged `|do`). No other letter or digit is relaxed,
/// so a quoted `l` never matches an OCR `1`, nor a quoted `1` an OCR `I`.
fn same(q: &QGlyph, g: &Glyph) -> bool {
    q.c == g.c || (q.single && q.c == 'i' && g.tok_start && I_LIKE.contains(&g.c))
}

/// Where a quote sits on the image.
struct Located {
    /// One padded rectangle per text line, with that line's approximate
    /// baseline (the highest word bottom, i.e. a word without descenders).
    lines: Vec<(Rect, u32)>,
    /// Indices into the OCR words, in order.
    words: Vec<usize>,
}

/// The unique occurrence of `quote` in `words`.
///
/// Both sides are compared as folded character streams (see [`fold`]), so
/// spacing, punctuation, quote styles, apostrophes and dashes never matter,
/// and neither does where OCR splits or merges words inside the quote. A
/// candidate must start and end on token boundaries; it counts towards
/// uniqueness even if those fall inside an OCR word, and is then rejected as
/// partial. A `|` inside the quoted span must stand for the quote word `I`:
/// it is never skipped, since it may be an `I` the quote left out. A match
/// may not span two Tesseract blocks: separate blocks (columns, captions,
/// sidebars) read one after another could fake a sentence.
///
/// Confidence: every matched word, single characters included, needs
/// `MIN_CONF`. A confident `|` read for `I` passes, since the rest of the
/// quote pins down its identity; a low-confidence one is refused like any
/// other word, because its box (which sets the highlight's edge when it opens
/// or closes a line) is what is in doubt.
///
/// Pre: `words` are in Tesseract reading order. Post: the match covers whole
/// OCR words only (a box is never wider than the quote); lines follow the
/// quote's order; every matched word has confidence ≥ `MIN_CONF` and lies in
/// one Tesseract block.
fn locate(words: &[Word], quote: &str) -> Result<Located, String> {
    let runs = tokens(quote);
    if runs.is_empty() {
        return Err("引文没有可匹配的文字".into());
    }
    let q: Vec<QGlyph> = runs
        .iter()
        .enumerate()
        .flat_map(|(run, t)| {
            let single = t.chars().count() == 1;
            t.chars().map(move |c| QGlyph { c, run, single })
        })
        .collect();
    let o = glyphs(words);
    let n = q.len();

    // Starts of token-aligned full matches; the first full match that ends
    // inside a token; the longest partial one (quote characters matched, OCR
    // glyph where it stopped), earliest first. The last two only explain a
    // failure.
    let mut hits: Vec<usize> = Vec::new();
    let mut inside: Option<usize> = None;
    let mut best: Option<(usize, usize)> = None;
    for s in (0..o.len()).filter(|&s| o[s].tok_start) {
        let k = q
            .iter()
            .zip(&o[s..])
            .take_while(|(qc, g)| same(qc, g))
            .count();
        if k == n {
            if o[s + n - 1].tok_end {
                hits.push(s);
            } else {
                inside.get_or_insert(s + n - 1);
            }
        } else if k > 0 && best.is_none_or(|(b, _)| k > b) {
            best = Some((k, s + k));
        }
    }
    let s = match hits.as_slice() {
        [s] => *s,
        [] => {
            if let Some(e) = inside {
                return Err(format!(
                    "原句的起止落在 OCR 词「{}」中间，高亮会多盖字；把 quote 写到整词，或改用 rects",
                    words[o[e].word].text
                ));
            }
            let Some((k, at)) = best else {
                return Err(format!(
                    "OCR 结果里找不到原句：第一个词「{}」就对不上。核对原文，或改用 rects",
                    runs[0]
                ));
            };
            let done = q[k].run;
            let seen = o
                .get(at)
                .map_or("（文末）", |g| words[g.word].text.as_str());
            return Err(format!(
                "OCR 结果里找不到原句：最多连续匹配 {done}/{} 个词，之后 OCR 读到的是「{seen}」，原句是「{}」。核对原文，或改用 rects",
                runs.len(),
                runs[done]
            ));
        }
        many => {
            return Err(format!(
                "原句在图中出现 {} 次，把 quote 写长一些以唯一定位",
                many.len()
            ));
        }
    };
    let e = s + n - 1;
    // A word box is all-or-nothing, so the quote must start and end on word
    // boundaries (e.g. not inside "state-of-the-art").
    let starts_word = s == 0 || o[s - 1].word != o[s].word;
    let ends_word = o.get(e + 1).is_none_or(|g| g.word != o[e].word);
    if !starts_word || !ends_word {
        let partial = if starts_word { o[e].word } else { o[s].word };
        return Err(format!(
            "原句的起止落在 OCR 词「{}」中间，高亮会多盖字；把 quote 写到整词，或改用 rects",
            words[partial].text
        ));
    }

    let mut idx: Vec<usize> = o[s..=e].iter().map(|g| g.word).collect();
    idx.dedup();
    continuity(words, &idx).map_err(|why| {
        format!("原句在图上不连续（{why}），可能把不相连的文字拼成了一句；核对原文后改用 rects")
    })?;
    if let Some(w) = idx.iter().map(|&i| &words[i]).find(|w| w.conf < MIN_CONF) {
        return Err(format!(
            "「{}」识别置信度 {:.0} 低于 {MIN_CONF}，不可靠；放大核对后改用 rects",
            w.text, w.conf
        ));
    }

    // Union of word boxes per line, in order of first appearance.
    // [left, top, right, bottom, baseline]
    let mut lines: Vec<((u32, u32, u32), [u32; 5])> = Vec::new();
    for w in idx.iter().map(|&i| &words[i]) {
        match lines.iter_mut().find(|(k, _)| *k == w.line) {
            Some((_, b)) => {
                b[0] = b[0].min(w.left);
                b[1] = b[1].min(w.top);
                b[2] = b[2].max(w.right);
                b[3] = b[3].max(w.bottom);
                b[4] = b[4].min(w.bottom);
            }
            None => lines.push((w.line, [w.left, w.top, w.right, w.bottom, w.bottom])),
        }
    }
    let lines = lines
        .into_iter()
        .map(|(_, [l, t, r, b, base])| {
            let pad = ((b - t) / 8).max(1);
            let rect = Rect {
                x: l.saturating_sub(pad),
                y: t.saturating_sub(1),
                w: r - l + 2 * pad,
                h: b - t + 2,
            };
            (rect, base)
        })
        .collect();
    Ok(Located { lines, words: idx })
}

/// Largest gap between neighbouring tokens of a line before it counts as a
/// gutter, in multiples of the line's tallest token box (about one em,
/// ascender to descender). Measured on issue 1014's five raw images
/// (`--psm 6`, eng): prose word gaps average 0.24–0.44 and reach 0.69. A
/// column gutter or table cell boundary is usually wider.
const MAX_GAP: f32 = 2.5;
/// The same right after a list marker that opens its line (`a.`, `(iv)`,
/// `12)`): hanging labels are set off by a tab, 2.18–2.45 after `m.` and
/// `n.` on 1014's legal-filing base.
const MARKER_GAP: f32 = 4.0;
/// Largest vertical gap between two lines of one quote (next line's top
/// minus previous line's bottom), in multiples of the smaller of the two
/// lines' heights: the smaller, so that one tall box (a `|`, a bracket)
/// cannot stretch the allowance. About the leading of ordinary text, with
/// room for 1.5 line spacing.
const MAX_LEADING: f32 = 1.5;
/// Words made only of these are ignored as gap boundaries: bare punctuation
/// that OCR scatters between words. Everything else counts, `|` included.
const SKIPPABLE: &str = ".,;:'\"‘’“”";

/// Tesseract's (block, paragraph, line) numbers of one text line.
type LineKey = (u32, u32, u32);

/// A list item's label: `(x)`, `[x]`, `x.` or `x)`, where x is one to three
/// digits, one ASCII letter, or one to five roman-numeral letters. Brackets
/// must pair: `(a]`, `[iv)` and `(1.` are not labels.
fn is_list_marker(text: &str) -> bool {
    let body = if let Some(b) = text.strip_prefix('(') {
        b.strip_suffix(')')
    } else if let Some(b) = text.strip_prefix('[') {
        b.strip_suffix(']')
    } else {
        text.strip_suffix(['.', ')'])
    };
    let Some(body) = body else {
        return false;
    };
    let n = body.chars().count();
    let digits = (1..=3).contains(&n) && body.chars().all(|c| c.is_ascii_digit());
    let letter = n == 1 && body.chars().all(|c| c.is_ascii_alphabetic());
    let roman = (1..=5).contains(&n) && body.chars().all(|c| "ivxlcdmIVXLCDM".contains(c));
    digits || letter || roman
}

/// Whether matched words `idx` (in reading order) form one stretch of text.
/// Tesseract's `--psm 6` reads the image as one block and joins side-by-side
/// columns into shared lines, so the character stream alone can splice text
/// that is not adjacent on the page. Conservative on purpose:
///
/// - Tokens are the OCR words with any text but `SKIPPABLE` punctuation, `|`
///   included. h is a line's tallest token box. A gutter is a gap over
///   `MAX_GAP`·h between neighbouring tokens; right after a list marker that
///   is its line's first token, over `MARKER_GAP`·h.
/// - A one-line quote: no gutter from the token before it to the token after
///   it.
/// - A quote over several lines: no gutter anywhere on any of its lines, so
///   multi-line quotes on multi-column layouts are refused outright. Every
///   line but the last ends at its last token, every continuation starts at
///   its first; the lines are consecutive in OCR order, each below the
///   previous, at most `MAX_LEADING` × the smaller line height apart.
/// - All words in one Tesseract block.
///
/// The error names the rule and the measured ratio.
fn continuity(words: &[Word], idx: &[usize]) -> Result<(), String> {
    let block = words[idx[0]].line.0;
    if idx.iter().any(|&i| words[i].line.0 != block) {
        return Err("跨越 OCR 的两个文本块".into());
    }
    let is_token =
        |w: &Word| (w.text.chars()).any(|c| !c.is_whitespace() && !SKIPPABLE.contains(c));
    // Matched word indices grouped by OCR line, in quote order.
    let mut groups: Vec<(LineKey, Vec<usize>)> = Vec::new();
    for &i in idx {
        match groups.last_mut() {
            Some((k, g)) if *k == words[i].line => g.push(i),
            _ => groups.push((words[i].line, vec![i])),
        }
    }
    let mut order: Vec<LineKey> = Vec::new();
    for w in words {
        if !order.contains(&w.line) {
            order.push(w.line);
        }
    }
    let pos = |k: &LineKey| order.iter().position(|o| o == k);
    let multi = groups.len() > 1;
    let last_line = groups.len() - 1;
    // (OCR line, top, bottom, h) of the previous matched line.
    let mut prev: Option<(LineKey, u32, u32, f32)> = None;
    for (k, (key, g)) in groups.iter().enumerate() {
        let toks: Vec<usize> = (0..words.len())
            .filter(|&i| words[i].line == *key && is_token(&words[i]))
            .collect();
        // A zero height would make every gap limit zero or, if patched to 1,
        // meaningless: refuse rather than guess.
        let h = (toks.iter())
            .map(|&i| words[i].bottom.saturating_sub(words[i].top))
            .max()
            .unwrap_or(0);
        if h == 0 {
            return Err("OCR 词框高度为 0，改用 rects".into());
        }
        let h = h as f32;
        let at = |i: usize| toks.iter().position(|&t| t == i);
        // Matched words carry letters, digits or `|`, so they are tokens.
        let (Some(a), Some(b)) = (at(g[0]), at(g[g.len() - 1])) else {
            return Err("匹配词不在 OCR 行内".into());
        };
        let end = toks.len() - 1;
        let (from, to) = if multi {
            (0, end)
        } else {
            (a.saturating_sub(1), (b + 1).min(end))
        };
        for j in from..to {
            let (l, r) = (&words[toks[j]], &words[toks[j + 1]]);
            let limit = if j == 0 && is_list_marker(&l.text) {
                MARKER_GAP
            } else {
                MAX_GAP
            };
            let px = r.left.saturating_sub(l.right) as f32;
            if px > limit * h {
                let scope = if multi {
                    "；跨行引文所在的每一行都不能有分栏空白"
                } else {
                    ""
                };
                return Err(format!(
                    "「{}」与「{}」之间空白达 {:.1} 倍词高{scope}",
                    l.text,
                    r.text,
                    px / h
                ));
            }
        }
        if multi && k < last_line && b != end {
            return Err(format!(
                "换行前没有读到该行末尾，之后还有「{}」",
                words[toks[b + 1]].text
            ));
        }
        if multi && k > 0 && a != 0 {
            return Err(format!(
                "续行没有从该行第一个词「{}」开始",
                words[toks[0]].text
            ));
        }
        let top = toks.iter().map(|&i| words[i].top).min().unwrap_or(0);
        let bottom = toks.iter().map(|&i| words[i].bottom).max().unwrap_or(0);
        if let Some((pkey, ptop, pbottom, ph)) = prev {
            if pos(key) != pos(&pkey).map(|i| i + 1) {
                return Err("换行处不是相邻的两行".into());
            }
            if top <= ptop {
                return Err("下一行不在上一行下方".into());
            }
            let leading = top.saturating_sub(pbottom) as f32;
            if leading > MAX_LEADING * h.min(ph) {
                return Err(format!(
                    "两行上下相隔过远，{:.1} 倍词高",
                    leading / h.min(ph)
                ));
            }
        }
        prev = Some((*key, top, bottom, h));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    fn row(text: &str, left: u32, top: u32, line: u32, conf: u32) -> String {
        // level page block par line word left top width height conf text
        format!("5\t1\t1\t1\t{line}\t1\t{left}\t{top}\t30\t12\t{conf}\t{text}")
    }

    fn words(rows: &[(&str, u32, u32, u32)]) -> Vec<Word> {
        let mut tsv = vec!["header".to_string()];
        tsv.extend(
            rows.iter()
                .map(|&(t, l, top, line)| row(t, l, top, line, 95)),
        );
        parse_tsv(&tsv.join("\n"), 1)
    }

    #[test]
    fn example_spec_parses_and_lists_quoted_texts() {
        let mut spec: Spec =
            toml::from_str(include_str!("../../../templates/cards/example.toml")).unwrap();
        // The sample is written for the layout of 1029 on.
        spec.publish = true;
        assert_eq!(spec.tldr.len(), 2);
        // Two 省流 facts and the two main-post 速览 titles.
        assert_eq!(spec.quoted_texts().len(), 4);
        assert!(spec.retired_barks().is_empty());
        assert!(
            spec.structure_errors().is_empty(),
            "{:?}",
            spec.structure_errors()
        );
        assert!(spec.structure_warnings().is_empty());
        assert_eq!(
            spec.outputs(),
            [
                "publish/01-tldr.png",
                "publish/02-roundup.png",
                "publish/03-event-1.png",
                "publish/04-bark-1.png",
                "publish/05-openai-dns-pause.png",
                "publish/06-event-2.png",
                "publish/07-bark-2.png",
                "publish/08-claude-nine-loop.png",
            ]
        );
    }

    /// Two main posts A and B, with three other 速览 entries and `bark`
    /// appended verbatim.
    fn bark_spec(bark: &str) -> Spec {
        toml::from_str(&format!(
            "date = \"d\"\n\
             [[tldr]]\ntag = \"A\"\nfact = \"f\"\n[[tldr]]\ntag = \"B\"\nfact = \"g\"\n\
             [[roundup]]\ntag = \"A\"\nmain = true\ntext = \"甲标题\"\n\
             [[roundup]]\ntag = \"B\"\nmain = true\ntext = \"乙标题\"\n\
             [[roundup]]\ntag = \"x\"\ntext = \"t\"\n[[roundup]]\ntag = \"x\"\ntext = \"t\"\n\
             [[roundup]]\ntag = \"x\"\ntext = \"t\"\n{bark}"
        ))
        .unwrap()
    }

    #[test]
    fn bark_cards_follow_the_main_posts() {
        let ab = "[[bark]]\ntag = \"A\"\npoints = [\"一\", \"二\"]\n\
                  [[bark]]\ntag = \"B\"\npoints = [\"三\"]\n";
        let ok = bark_spec(ab);
        assert!(
            ok.structure_errors().is_empty(),
            "{:?}",
            ok.structure_errors()
        );
        assert_eq!(
            ok.outputs(),
            [TLDR_OUT, ROUNDUP_OUT, "00-bark-1.png", "00-bark-2.png"]
        );
        // Points are not 速览 or 省流 texts; lint checks them section by section.
        assert_eq!(ok.quoted_texts().len(), 4);

        // Out of the 省流卡 order; numbered or empty points; no points.
        let ba = "[[bark]]\ntag = \"B\"\npoints = [\"三\"]\n\
                  [[bark]]\ntag = \"A\"\npoints = [\"①一\", \" \"]\n\
                  [[bark]]\ntag = \"C\"\npoints = []\n";
        let errs = bark_spec(ba).structure_errors().join("\n");
        for needle in ["一一对应", "①②", "空的", "没有 points", "「C」的小标题"] {
            assert!(errs.contains(needle), "missing {needle}: {errs}");
        }
        // A 速览专帖 has no main post, hence no 吠点卡.
        let solo = roundup(
            "kind = \"roundup\"\n[[bark]]\ntag = \"x\"\npoints = [\"p\"]\n",
            &[("x", "t", false); 5],
        );
        assert!(
            solo.structure_errors()
                .iter()
                .any(|e| e.contains("[[bark]]"))
        );
    }

    #[test]
    fn bark_card_page_and_fingerprint() {
        let ab = |second: &str, title: &str| {
            let mut s = bark_spec(&format!(
                "[[bark]]\ntag = \"A\"\npoints = [\"一\", \"“二”\"]\n\
                 [[bark]]\ntag = \"B\"\npoints = [\"{second}\"]\n"
            ));
            s.roundup[1].text = title.into();
            s
        };
        let spec = ab("三", "乙标题");
        let html = bark_html(&spec, 1).unwrap();
        assert!(html.contains(r#"<body class="c2">"#));
        assert!(html.contains("乙标题") && html.contains(">B<"));
        assert!(html.contains(r#"<span class="n">①</span><div class="tx">三</div>"#));
        // A leading full-width quote hangs.
        let first = bark_html(&spec, 0).unwrap();
        assert!(first.contains(r#"<span class="n">②</span><div class="tx q">“二”</div>"#));

        let images = Path::new("unused");
        let fp = |s: &Spec, i| s.fingerprint(&Job::Bark(i), images, None, "t", "");
        // A card follows its own points and its heading in 速览, not the other card's.
        assert_eq!(fp(&spec, 0), fp(&ab("四", "乙标题"), 0));
        assert_ne!(fp(&spec, 1), fp(&ab("四", "乙标题"), 1));
        assert_ne!(fp(&spec, 1), fp(&ab("三", "丙标题"), 1));
    }

    #[test]
    fn event_cards_follow_the_main_posts() {
        let ev = "[[event]]\ntag = \"A\"\nparas = [\"一\", \"二\"]\n\
                  [[event]]\ntag = \"B\"\nparas = [\"三\"]\n";
        let bk = "[[bark]]\ntag = \"A\"\npoints = [\"p\"]\n\
                  [[bark]]\ntag = \"B\"\npoints = [\"q\"]\n";
        let ok = bark_spec(&format!("{ev}{bk}"));
        assert!(
            ok.structure_errors().is_empty(),
            "{:?}",
            ok.structure_errors()
        );
        // Upload order: each main post's 事件卡, then its 吠点卡.
        assert_eq!(
            ok.outputs(),
            [
                TLDR_OUT,
                ROUNDUP_OUT,
                "00-event-1.png",
                "00-bark-1.png",
                "00-event-2.png",
                "00-bark-2.png"
            ]
        );
        // Paragraphs are quoted from the body by lint section by section, not here.
        assert_eq!(ok.quoted_texts().len(), 4);
        assert!(ok.own_texts().contains(&"三"));

        // Out of the 省流卡 order; empty paragraph; no paras; unknown tag.
        let ba = "[[event]]\ntag = \"B\"\nparas = [\"三\"]\n\
                  [[event]]\ntag = \"A\"\nparas = [\"一\", \" \"]\n\
                  [[event]]\ntag = \"C\"\nparas = []\n";
        let errs = bark_spec(ba).structure_errors().join("\n");
        for needle in ["一一对应", "有一段是空的", "没有 paras", "「C」的小标题"]
        {
            assert!(errs.contains(needle), "missing {needle}: {errs}");
        }
        // A 速览专帖 has no main post, hence no 事件卡.
        let solo = roundup(
            "kind = \"roundup\"\n[[event]]\ntag = \"x\"\nparas = [\"p\"]\n",
            &[("x", "t", false); 5],
        );
        assert!(
            solo.structure_errors()
                .iter()
                .any(|e| e.contains("[[event]]"))
        );
    }

    #[test]
    fn event_card_page_and_fingerprint() {
        let ab = |second: &str, title: &str| {
            let mut s = bark_spec(&format!(
                "[[event]]\ntag = \"A\"\nparas = [\"一\", \"“二”\"]\n\
                 [[event]]\ntag = \"B\"\nparas = [\"{second}\"]\n"
            ));
            s.roundup[1].text = title.into();
            s
        };
        let spec = ab("三", "乙标题");
        let html = event_html(&spec, 1).unwrap();
        assert!(html.contains(r#"<body class="c2">"#));
        assert!(html.contains("乙标题") && html.contains(">B<"));
        // No numbers; a leading full-width quote hangs.
        assert!(html.contains(r#"<div class="pt"><div class="tx">三</div></div>"#));
        let first = event_html(&spec, 0).unwrap();
        assert!(first.contains(r#"<div class="tx q">“二”</div>"#));
        assert!(!first.contains(r#"class="n""#));

        let images = Path::new("unused");
        let fp = |s: &Spec, i| s.fingerprint(&Job::Event(i), images, None, "t", "");
        assert_eq!(fp(&spec, 0), fp(&ab("四", "乙标题"), 0));
        assert_ne!(fp(&spec, 1), fp(&ab("四", "乙标题"), 1));
        assert_ne!(fp(&spec, 1), fp(&ab("三", "丙标题"), 1));
    }

    fn roundup(toml_head: &str, items: &[(&str, &str, bool)]) -> Spec {
        let mut s = format!("date = \"d\"\n{toml_head}");
        for (tag, text, main) in items {
            s.push_str(&format!(
                "[[roundup]]\ntag = \"{tag}\"\ntext = \"{text}\"\nmain = {main}\n"
            ));
        }
        toml::from_str(&s).unwrap()
    }

    #[test]
    fn roundup_structure_rules() {
        let tldr = "[[tldr]]\ntag = \"A\"\nfact = \"f\"\n[[tldr]]\ntag = \"B\"\nfact = \"g\"\n";
        let others = [("x", "t", false); 3];
        let ok = [&[("A", "a", true), ("B", "b", true)][..], &others].concat();
        assert!(roundup(tldr, &ok).structure_errors().is_empty());

        // Main entries out of the 省流卡 order, or not leading.
        let swapped = [&[("B", "b", true), ("A", "a", true)][..], &others].concat();
        assert_eq!(roundup(tldr, &swapped).structure_errors().len(), 1);
        let late = [&others[..], &[("A", "a", true), ("B", "b", true)]].concat();
        assert!(!roundup(tldr, &late).structure_errors().is_empty());

        // Too few, too long.
        assert_eq!(roundup("", &others).structure_errors().len(), 1);
        let long = "字".repeat(ROUNDUP_TEXT_MAX + 1);
        let mut five = vec![("x", "t", false); 4];
        five.push(("x", &long, false));
        assert_eq!(roundup("", &five).structure_errors().len(), 1);

        // 速览专帖: no 省流卡, no main entries; covers come from the template.
        let solo = roundup("kind = \"roundup\"\n", &[("x", "t", false); 5]);
        assert!(solo.structure_errors().is_empty());
        assert_eq!(solo.outputs(), [COVER_OUT, COVER_WIDE_OUT, ROUNDUP_OUT]);
        assert!(
            !roundup(&format!("kind = \"roundup\"\n{tldr}"), &ok)
                .structure_errors()
                .is_empty()
        );
    }

    #[test]
    fn retired_barks_are_parsed_but_flagged() {
        let spec: Spec =
            toml::from_str("date = \"d\"\n[[tldr]]\ntag = \"t\"\nfact = \"f\"\nbarks = [\"b\"]\n")
                .unwrap();
        assert_eq!(spec.quoted_texts(), ["f", "b"]);
        assert!(spec.retired_barks()[0].contains("barks"));

        // Every occurrence is reported, not just the first.
        let many: Spec = toml::from_str(
            "date = \"d\"\n[[tldr]]\ntag = \"t1\"\nfact = \"f\"\nbarks = [\"b\"]\n\
             [[tldr]]\ntag = \"t2\"\nfact = \"g\"\nbarks = [\"c\"]\n\
             [[annot]]\nout = \"01-x.png\"\ntitle = \"t\"\nsource = \"s\"\nbark = \"x\"\nnote = []\n",
        )
        .unwrap();
        let all = many.retired_barks();
        assert_eq!(all.len(), 3, "{all:?}");
        assert!(all[1].contains("t2") && all[2].contains("01-x.png"));
    }

    #[test]
    fn tokens_ignore_punctuation_and_split_cjk() {
        assert_eq!(tokens("later. All"), ["later", "all"]);
        assert_eq!(tokens("“停训”有 GPT-6"), ["停", "训", "有", "gpt", "6"]);
    }

    #[test]
    fn locate_gives_one_rect_per_line_and_rejects_ambiguity() {
        let w = words(&[
            ("The", 10, 10, 1),
            ("run", 50, 10, 1),
            ("was", 90, 10, 1),
            ("killed.", 10, 30, 2),
            ("The", 60, 30, 2),
        ]);
        let found = locate(&w, "run was killed").unwrap();
        assert_eq!(found.words, [1, 2, 3]);
        assert_eq!(found.lines.len(), 2);
        let r0 = Rect {
            x: 49,
            y: 9,
            w: 72,
            h: 14,
        };
        assert_eq!(found.lines[0], (r0, 22));
        assert!(locate(&w, "The").err().unwrap().contains("2 次"));
        assert!(locate(&w, "run was not").err().unwrap().contains("2/3"));
    }

    #[test]
    fn locate_rejects_partial_words_and_low_confidence() {
        let w = words(&[
            ("a", 0, 0, 1),
            ("state-of-the-art", 10, 0, 1),
            ("model", 60, 0, 1),
        ]);
        assert!(locate(&w, "a state of the").err().unwrap().contains("中间"));
        assert!(
            locate(&w, "of the art model")
                .err()
                .unwrap()
                .contains("中间")
        );
        assert!(locate(&w, "state of the art").is_ok());

        let tsv = format!(
            "h\n{}\n{}",
            row("blurry", 0, 0, 1, 95),
            row("w0rd", 40, 0, 1, 41)
        );
        assert!(
            locate(&parse_tsv(&tsv, 1), "blurry w0rd")
                .err()
                .unwrap()
                .contains("置信度")
        );
    }

    #[test]
    fn split_overlaps_meets_halfway() {
        let mut hls = vec![
            vec![Rect {
                x: 0,
                y: 0,
                w: 50,
                h: 20,
            }],
            vec![
                Rect {
                    x: 40,
                    y: 16,
                    w: 50,
                    h: 20,
                },
                Rect {
                    x: 200,
                    y: 10,
                    w: 9,
                    h: 9,
                },
            ],
        ];
        split_overlaps(&mut hls);
        assert_eq!(hls[0][0].y + hls[0][0].h, hls[1][0].y);
        assert_eq!(hls[1][0].y + hls[1][0].h, 36);
        assert_eq!(
            hls[1][1],
            Rect {
                x: 200,
                y: 10,
                w: 9,
                h: 9
            },
            "no x overlap, untouched"
        );
    }

    #[test]
    fn parse_tsv_scales_boxes_back() {
        let w = parse_tsv(&format!("h\n{}", row("x", 30, 60, 1, 95)), 3);
        assert_eq!(
            (w[0].left, w[0].top, w[0].right, w[0].bottom),
            (10, 20, 20, 24)
        );
    }

    #[test]
    fn escaping_blocks_placeholder_injection() {
        assert_eq!(
            esc("{{NOTES}} <b>"),
            "&#123;&#123;NOTES&#125;&#125; &lt;b&gt;"
        );
        assert_eq!(
            layout_state(r#"<body data-layout="overflow:x">"#),
            Some("overflow:x")
        );
        assert_eq!(layout_state("<body>"), None);
    }

    /// Words of one line, with explicit confidences.
    fn line_words(rows: &[(&str, u32)]) -> Vec<Word> {
        let mut tsv = vec!["header".to_string()];
        tsv.extend(
            rows.iter()
                .enumerate()
                .map(|(i, &(t, conf))| row(t, 40 * i as u32, 0, 1, conf)),
        );
        parse_tsv(&tsv.join("\n"), 1)
    }

    #[test]
    fn locate_reads_pipe_as_one_letter_i() {
        // Issue 1014, 44-sunshowers-plagiarism-raw.png as Tesseract read it:
        // "I" came out as "|" and the matcher used to stop at 0/12 words.
        let w = line_words(&[
            ("time.", 97),
            ("And,", 96),
            ("to", 97),
            ("be", 96),
            ("clear,", 97),
            ("|", 96),
            ("do", 97),
            ("not", 97),
            ("have", 97),
            ("a", 96),
            ("problem", 96),
            ("with", 93),
            ("Anthropic", 91),
            ("copying", 93),
            ("Mononoke's", 61),
            ("design.", 97),
            ("(This", 97),
        ]);
        let quote = "I do not have a problem with Anthropic copying Mononoke’s design";
        let found = locate(&w, quote).unwrap();
        assert_eq!(found.words, (5..=15).collect::<Vec<_>>());

        // The other usual misreadings, and "I" merged into the next word.
        for i in ["l", "1", "I"] {
            let w = line_words(&[("clear,", 97), (i, 96), ("do", 97), ("not", 97)]);
            assert_eq!(locate(&w, "I do not").unwrap().words, [1, 2, 3], "{i}");
        }
        let merged = line_words(&[("clear,", 97), ("|do", 90), ("not", 97)]);
        assert_eq!(locate(&merged, "I do not").unwrap().words, [1, 2]);

        // A "|" is never skipped: it may be an "I" the quote left out.
        let w = line_words(&[("clear,", 97), ("|", 96), ("do", 97), ("not", 97)]);
        assert!(locate(&w, "clear do not").err().unwrap().contains("找不到"));
        // Nor does it stand for a longer word.
        assert!(locate(&w, "clear it do").err().unwrap().contains("找不到"));
        // Uniqueness still counts every reading.
        let two = line_words(&[
            ("|", 96),
            ("think", 97),
            ("and", 97),
            ("I", 96),
            ("think", 97),
        ]);
        assert!(locate(&two, "I think").err().unwrap().contains("2 次"));
    }

    #[test]
    fn glyph_tolerance_is_only_for_the_word_i() {
        // Codex review 2026-10-15: `l` ↔ `1` must not match, nor a quoted
        // digit an OCR `I`.
        let w = line_words(&[("Model", 97), ("1", 96), ("wins", 97)]);
        assert!(locate(&w, "Model l wins").err().unwrap().contains("找不到"));
        assert!(locate(&w, "Model 1 wins").is_ok());
        for ocr in ["I", "|", "l"] {
            let w = line_words(&[("step", 97), (ocr, 96), ("done", 97)]);
            let e = locate(&w, "step 1 done").err().unwrap();
            assert!(e.contains("找不到"), "{ocr}: {e}");
        }
        let w = line_words(&[("Model", 97), ("l", 96), ("wins", 97)]);
        assert!(locate(&w, "Model 1 wins").err().unwrap().contains("找不到"));
    }

    #[test]
    fn locate_refuses_a_match_across_tesseract_blocks() {
        // level page block par line word left top width height conf text
        let tsv = [
            "header",
            "5\t1\t1\t1\t1\t1\t0\t0\t30\t12\t95\tthe\n5\t1\t1\t1\t1\t2\t40\t0\t30\t12\t95\tmodel",
            "5\t1\t2\t1\t1\t1\t0\t40\t30\t12\t95\twas\n5\t1\t2\t1\t1\t2\t40\t40\t30\t12\t95\tpaused",
        ]
        .join("\n");
        let w = parse_tsv(&tsv, 1);
        assert!(locate(&w, "model was").err().unwrap().contains("rects"));
        assert!(locate(&w, "was paused").is_ok());
    }

    #[test]
    fn output_names_clash_case_insensitively() {
        let spec: Spec = toml::from_str(
            "[[annot]]\nout = \"01-x.png\"\ntitle = \"t\"\nsource = \"s\"\nnote = []\n\
             [[annot]]\nout = \"01-X.png\"\ntitle = \"t\"\nsource = \"s\"\nnote = []\n",
        )
        .unwrap();
        let errs = precheck(&spec, Path::new("no-such-issue/images"), &[]).join("\n");
        assert!(errs.contains("「01-X.png」与前面的重复"), "{errs}");
    }

    #[test]
    fn tessdata_dir_comes_from_list_langs() {
        let out = "List of available languages in \"C:\\t\\tessdata/\" (2):";
        assert_eq!(tessdata_dir(out), Some(PathBuf::from("C:\\t\\tessdata/")));
        assert_eq!(tessdata_dir("tesseract: not found"), None);
    }

    #[test]
    fn locate_low_confidence_single_letter_is_refused() {
        let w = line_words(&[("|", 50), ("am", 97), ("very", 97), ("glad", 97)]);
        assert!(locate(&w, "I am very").err().unwrap().contains("置信度"));
        assert!(locate(&w, "am very glad").is_ok());
    }

    #[test]
    fn locate_ignores_quote_styles_apostrophes_dashes_and_ligatures() {
        let w = line_words(&[
            ("for", 96),
            ("“hard-won", 96),
            ("lessons\",", 76),
            ("and", 97),
        ]);
        for q in [
            "for “hard-won lessons”, and",
            "for \"hard–won lessons\" and",
            "for ‘hard—won lessons’ and",
            "for hard‑won lessons and",
        ] {
            assert_eq!(locate(&w, q).unwrap().words, [0, 1, 2, 3], "{q}");
        }
        // However OCR renders or splits the apostrophe.
        for ocr in [
            &[("Google's", 83), ("Piper", 97)][..],
            &[("Google’s", 91), ("Piper", 97)],
            &[("Googles", 91), ("Piper", 97)],
            &[("Google’", 91), ("s", 90), ("Piper", 97)],
        ] {
            assert!(
                locate(&line_words(ocr), "Google’s Piper").is_ok(),
                "{ocr:?}"
            );
        }
        let w = line_words(&[("ﬁnd", 90), ("a.", 96), ("problem", 96)]);
        assert_eq!(locate(&w, "find a problem").unwrap().words, [0, 1, 2]);
    }

    #[test]
    fn locate_explains_a_match_inside_a_word() {
        let w = line_words(&[("copying", 93), ("Mononokes", 90), ("design.", 97)]);
        let e = locate(&w, "copying Mononoke").err().unwrap();
        assert!(e.contains("中间") && e.contains("Mononokes"), "{e}");
    }

    #[test]
    fn precheck_reports_every_problem_at_once() {
        let spec: Spec = toml::from_str(
            r#"
[[tldr]]
tag = "A"
fact = "f"

[[annot]]
out = "01-x.png"
title = "t"
source = "s"
[[annot.note]]
quote = "q"
rects = [[0, 0, 1, 1]]
gloss = "g"
emph = "zz"

[[annot]]
out = "01-x.png"
title = "t"
source = "s"
note = []
"#,
        )
        .unwrap();
        let images = Path::new("no-such-issue/images");
        let all = precheck(&spec, images, &[]).join("\n");
        for needle in ["重复", "date", "二选一", "emph", "1–4 条 note", "底图"] {
            assert!(all.contains(needle), "missing {needle}: {all}");
        }
        // Naming outputs: unknown names are reported, and only the named
        // cards need their (git-ignored) base images.
        let named = precheck(&spec, images, &["nope.png"]).join("\n");
        assert!(
            named.contains("nope.png") && named.contains("emph"),
            "{named}"
        );
        assert!(!named.contains("底图"), "{named}");
    }

    #[test]
    fn fingerprints_track_each_cards_own_inputs() {
        let images = std::env::temp_dir()
            .join("barking-card-test")
            .join(std::process::id().to_string());
        fs::create_dir_all(&images).unwrap();
        let raw = images.join("01-x-raw.png");
        fs::write(&raw, b"one").unwrap();
        let toml = |date: &str, gloss: &str| -> Spec {
            toml::from_str(&format!(
                "date = \"{date}\"\n[[tldr]]\ntag = \"A\"\nfact = \"f\"\n\
                 [[annot]]\nout = \"01-x.png\"\ntitle = \"t\"\nsource = \"s\"\n\
                 [[annot.note]]\nquote = \"q\"\ngloss = \"{gloss}\"\n"
            ))
            .unwrap()
        };
        let fps_ocr = |spec: &Spec, tool: &str, ocr: &str| -> Vec<String> {
            spec.jobs()
                .iter()
                .map(|(_, job)| spec.fingerprint(job, &images, None, tool, ocr))
                .collect()
        };
        let fps = |spec: &Spec, tool: &str| fps_ocr(spec, tool, "tess 5\neng:aa");
        let base = fps(&toml("d", "g"), "t1");
        assert_eq!(base, fps(&toml("d", "g"), "t1"), "deterministic");
        // A different Tesseract model touches only the annotated screenshot.
        let model = fps_ocr(&toml("d", "g"), "t1", "tess 5\neng:bb");
        assert_eq!((model[0] == base[0], model[1] == base[1]), (true, false));
        // An annot edit touches only that card; a shared field or a rebuild
        // touches all; so do the base image's bytes for its card.
        let gloss = fps(&toml("d", "g2"), "t1");
        assert_eq!((gloss[0] == base[0], gloss[1] == base[1]), (true, false));
        let date = fps(&toml("d2", "g"), "t1");
        assert!(date.iter().zip(&base).all(|(a, b)| a != b));
        let tool = fps(&toml("d", "g"), "t2");
        assert!(tool.iter().zip(&base).all(|(a, b)| a != b));
        fs::write(&raw, b"two").unwrap();
        let bytes = fps(&toml("d", "g"), "t1");
        assert_eq!((bytes[0] == base[0], bytes[1] == base[1]), (true, false));

        // Cache round trip.
        let file = images.join("cache.json");
        let mut cache = Cache::default();
        cache.insert("01-x.png", base[1].clone());
        cache.save(&file).unwrap();
        assert_eq!(Cache::load(&file).unwrap(), cache);

        // One run per images directory; the lock goes away with its holder.
        let held = Lock::acquire(&file).unwrap();
        let busy = Lock::acquire(&file).err().unwrap();
        assert!(busy.contains("锁文件") && busy.contains(&std::process::id().to_string()));
        drop(held);
        assert!(Lock::acquire(&file).is_ok());
        let _ = fs::remove_dir_all(&images);
    }

    /// A scratch directory for one test, removed by the caller.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join("barking-card-test")
            .join(format!("{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn cache_keys_ignore_case_so_a_b_a_rerenders() {
        // Codex review round 2: `01-x.png` (A) → `01-X.png` (B) → `01-x.png`
        // (A) wrote one file three times; A must not be skipped at the end.
        let dir = scratch("aba");
        let file = dir.join("cache.json");
        let run = |out: &str, fp: &str| {
            let mut cache = Cache::load(&file).unwrap();
            let fresh = cache.get(out) == Some(&fp.to_string());
            if !fresh {
                render_one(&mut cache, Some(&file), out, Some(fp.into()), || Ok(())).unwrap();
            }
            fresh
        };
        assert!(!run("01-x.png", "A"));
        assert!(!run("01-X.png", "B"));
        assert!(!run("01-x.png", "A"), "A skipped while the file holds B");
        assert!(run("01-X.png", "A"), "same file, same inputs");

        // A cache written before keys were lowercased: drop every variant.
        fs::write(&file, r#"{"01-x.png":"A","01-X.png":"B","02-y.png":"C"}"#).unwrap();
        let cache = Cache::load(&file).unwrap();
        assert_eq!(
            (cache.get("01-x.png"), cache.get("02-Y.png")),
            (None, Some(&"C".into()))
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn unsaved_cache_drop_leaves_the_old_image_alone() {
        let dir = scratch("unsaved");
        // The cache's parent is a file, so saving fails.
        fs::write(dir.join("blocker"), "").unwrap();
        let file = dir.join("blocker").join("cache.json");
        let mut cache = Cache::default();
        cache.insert("01-x.png", "A".into());
        let mut rendered = false;
        let e = render_one(
            &mut cache,
            Some(&file),
            "01-x.png",
            Some("B".into()),
            || {
                rendered = true;
                Ok(())
            },
        )
        .err()
        .unwrap();
        assert!(e.contains("未覆盖旧图") && !rendered, "{e}");

        // Without a stale entry there is nothing to distrust: it renders, and
        // a failed save of the new entry is only reported.
        let mut empty = Cache::default();
        render_one(
            &mut empty,
            Some(&file),
            "02-y.png",
            Some("C".into()),
            || Ok(()),
        )
        .unwrap();
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn locate_refuses_words_from_two_columns() {
        // Codex review round 2: `--psm 6` joins side-by-side columns into one
        // line, so the stream alone would accept these.
        let w = line_words(&[("the", 97), ("model", 97)]);
        let mut cols = w;
        cols.extend(
            line_words(&[("was", 97), ("paused", 97)])
                .into_iter()
                .map(|mut x| {
                    x.left += 600;
                    x.right += 600;
                    x
                }),
        );
        let e = locate(&cols, "model was").err().unwrap();
        assert!(
            e.contains("「model」与「was」") && e.contains("rects"),
            "{e}"
        );
        // A one-line quote must not touch the gutter on either side; one
        // inside a column still resolves.
        assert!(locate(&cols, "was paused").is_err());
        assert!(locate(&cols, "the").is_ok());
        assert!(locate(&cols, "paused").is_ok());

        // A wrap from the right column's line into the next line's left
        // column.
        let w = placed(&[
            &[("alpha", 0, 0), ("beta", 40, 0), ("gamma", 600, 0)],
            &[("delta", 0, 20), ("eps", 600, 20)],
        ]);
        assert!(locate(&w, "gamma delta").err().unwrap().contains("rects"));
        assert!(locate(&w, "alpha beta").is_err());
        assert!(locate(&w, "alpha").is_ok());
    }

    /// Lines of (text, left, top), one TSV line each in order, height 12.
    fn placed(lines: &[&[(&str, u32, u32)]]) -> Vec<Word> {
        let mut tsv = vec!["header".to_string()];
        for (n, line) in lines.iter().enumerate() {
            for &(t, left, top) in line.iter() {
                tsv.push(row(t, left, top, n as u32 + 1, 95));
            }
        }
        parse_tsv(&tsv.join("\n"), 1)
    }

    #[test]
    fn continuity_counterexamples() {
        // Round 3 (b): the right column's tail spliced onto the next line.
        let w = placed(&[
            &[("alpha", 0, 0), ("beta", 40, 0), ("gamma", 600, 0)],
            &[("delta", 0, 20)],
        ]);
        let e = locate(&w, "gamma delta").err().unwrap();
        assert!(
            e.contains("「beta」与「gamma」") && e.contains("rects"),
            "{e}"
        );

        // Round 3 (d): consecutive OCR lines far apart on the page, or a
        // line above the previous one.
        let w = placed(&[&[("approved", 0, 0)], &[("denied", 0, 1000)]]);
        let e = locate(&w, "approved denied").err().unwrap();
        assert!(e.contains("上下相隔过远"), "{e}");
        let w = placed(&[&[("approved", 0, 100)], &[("denied", 0, 90)]]);
        let e = locate(&w, "approved denied").err().unwrap();
        assert!(e.contains("下方"), "{e}");

        // Round 4: a gutter further away than the quote's neighbours, on a
        // line the quote wraps from or into.
        let w = placed(&[
            &[("intro", 0, 0), ("gamma", 40, 0)],
            &[("delta", 0, 20), ("epsilon", 40, 20), ("zeta", 600, 20)],
        ]);
        let e = locate(&w, "gamma delta").err().unwrap();
        assert!(
            e.contains("「epsilon」与「zeta」") && e.contains("每一行"),
            "{e}"
        );
        let w = placed(&[
            &[("intro", 0, 0), ("zeta", 600, 0), ("gamma", 640, 0)],
            &[("delta", 0, 20), ("epsilon", 40, 20)],
        ]);
        assert!(
            locate(&w, "gamma delta")
                .err()
                .unwrap()
                .contains("「intro」与「zeta」")
        );
        // Round 4: a `|` (a rule, or another column's I) counts as a token.
        let w = placed(&[
            &[("alpha", 0, 0), ("beta", 40, 0)],
            &[("gamma", 0, 20), ("|", 600, 20)],
        ]);
        let e = locate(&w, "beta gamma").err().unwrap();
        assert!(e.contains("「gamma」与「|」"), "{e}");
        // Bare punctuation does not, so a stray `.` far right is no gutter.
        let w = placed(&[
            &[("alpha", 0, 0), ("beta", 40, 0)],
            &[("gamma", 0, 20), (".", 600, 20)],
        ]);
        assert!(locate(&w, "beta gamma").is_ok());

        // A normal two-line paragraph, wrapping at word spacing.
        let w = placed(&[
            &[("one", 0, 0), ("two", 40, 0), ("three", 80, 0)],
            &[("four", 0, 20), ("five", 40, 20)],
        ]);
        assert_eq!(locate(&w, "two three four").unwrap().words, [1, 2, 3]);
        assert_eq!(locate(&w, "three four five").unwrap().words, [2, 3, 4]);
        // "two three" would need to skip "x": the stream itself refuses it.
        let w = placed(&[
            &[("one", 0, 0), ("two", 40, 0)],
            &[("x", 0, 20), ("three", 40, 20)],
        ]);
        assert!(locate(&w, "two three").is_err());
    }

    #[test]
    fn zero_height_line_is_refused() {
        // Codex review round 5: `alpha` 0..30 and `beta` 31..61, top = bottom.
        let tsv = [
            "header",
            "5\t1\t1\t1\t1\t1\t0\t0\t30\t0\t95\talpha",
            "5\t1\t1\t1\t1\t2\t31\t0\t30\t0\t95\tbeta",
        ]
        .join("\n");
        let e = locate(&parse_tsv(&tsv, 1), "alpha beta").err().unwrap();
        assert!(e.contains("高度为 0") && e.contains("rects"), "{e}");
    }

    #[test]
    fn list_marker_gap_only_at_line_start() {
        // A tab after a line-opening marker (3.75 heights) is allowed.
        let w = placed(&[&[("m.", 0, 0), ("Enjoin", 75, 0), ("them", 115, 0)]]);
        assert!(locate(&w, "Enjoin them").is_ok());
        // After an ordinary word, or a marker that does not open the line,
        // the same gap is a gutter.
        let w = placed(&[&[("so", 0, 0), ("Enjoin", 75, 0), ("them", 115, 0)]]);
        assert!(
            locate(&w, "Enjoin them")
                .err()
                .unwrap()
                .contains("「so」与「Enjoin」")
        );
        let w = placed(&[&[("so", 0, 0), ("m.", 40, 0), ("Enjoin", 115, 0)]]);
        assert!(locate(&w, "Enjoin").is_err());
        // Over 4 heights even after a marker.
        let w = placed(&[&[("m.", 0, 0), ("Enjoin", 90, 0)]]);
        assert!(locate(&w, "Enjoin").is_err());
    }

    #[test]
    fn list_markers() {
        for m in [
            "a.", "a)", "B)", "(3)", "[iv]", "12.", "(xiv)", "IV.", "[7]",
        ] {
            assert!(is_list_marker(m), "{m}");
        }
        for m in [
            "I,", "the.", "1234.", "a", "(ab)", "vvvvvv.", "(a]", "[iv)", "(1.", "(a", "a]", "()",
        ] {
            assert!(!is_list_marker(m), "{m}");
        }
    }

    #[test]
    fn version_lines() {
        for v in [
            "tesseract v5.5.3.20260724",
            "tesseract 4.1.1",
            "tesseract 5",
            "tesseract 5.3.0-1-gabc12",
            "tesseract v5.0.0+git.2024",
        ] {
            assert!(is_version_line(v), "{v}");
        }
        for v in [
            "tesseract 5...",
            "tesseract 5..1",
            "tesseract .5",
            "tesseract v",
            "tesseract 5.3-",
            "tesseract 5.3-a b",
            "tesseract 5.3+x+y",
            "tesseract vX",
            "Tesseract 5.3",
            "tesseract  5.3",
        ] {
            assert!(!is_version_line(v), "{v}");
        }
    }

    #[test]
    fn unreadable_cache_stops_the_run() {
        let dir = scratch("unreadable");
        let file = dir.join("cache.json");
        assert_eq!(Cache::load(&file).unwrap(), Cache::default(), "absent");
        fs::write(&file, "{not json").unwrap();
        assert!(Cache::load(&file).err().unwrap().contains("已损坏"));
        // A directory where the file should be: exists, cannot be read.
        fs::remove_file(&file).unwrap();
        fs::create_dir(&file).unwrap();
        assert!(Cache::load(&file).err().unwrap().contains("读不了"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn tesseract_probes_must_look_right() {
        let p = |ok: bool, out: &str, err: &str| Probe {
            ok,
            stdout: out.into(),
            stderr: err.into(),
        };
        let version = p(true, "tesseract v5.5.3.20260724\n leptonica-1.87.0\n", "");
        let langs = p(
            true,
            "List of available languages in \"C:\\t/\" (2):\neng\nosd\n",
            "",
        );
        let t = parse_tess(&version, &langs).unwrap();
        assert_eq!(
            (t.version.as_str(), t.dir),
            ("tesseract v5.5.3.20260724", PathBuf::from("C:\\t/"))
        );
        // Older builds print the version to stderr.
        assert!(parse_tess(&p(true, "", "tesseract 4.1.1\n"), &langs).is_some());

        let failed = p(false, "tesseract v5.5.3\n", "");
        assert!(parse_tess(&failed, &langs).is_none(), "exit status");
        assert!(parse_tess(&version, &p(false, langs.stdout.as_str(), "")).is_none());
        for bad in [
            "",
            "Tesseract Open Source OCR Engine",
            "tesseract",
            "tesseract vX",
            "usage: tesseract",
        ] {
            assert!(parse_tess(&p(true, bad, ""), &langs).is_none(), "{bad:?}");
        }
        for bad in [
            "",
            "eng\nosd\n",
            "List of available languages in \"\" (0):",
            "List of available languages in \"C:/t/\"",
        ] {
            assert!(parse_tess(&version, &p(true, bad, "")).is_none(), "{bad:?}");
        }
    }
    // ---- 2026-10-29 layout: images/publish/, numbered in upload order -----

    /// Three main posts A, B, C with all cards, plus `annot` appended. Parsed
    /// from text, so it starts in the old layout; the caller says which.
    fn posts_spec(annot: &str, publish: bool) -> Spec {
        let mut head = String::from("date = \"d\"\n");
        for t in ["A", "B", "C"] {
            head += &format!("[[tldr]]\ntag = \"{t}\"\nfact = \"f\"\n");
            head += &format!("[[roundup]]\ntag = \"{t}\"\nmain = true\ntext = \"标题{t}\"\n");
            head += &format!("[[event]]\ntag = \"{t}\"\nparas = [\"p\"]\n");
            head += &format!("[[bark]]\ntag = \"{t}\"\npoints = [\"q\"]\n");
        }
        for i in 0..3 {
            head += &format!("[[roundup]]\ntag = \"x{i}\"\ntext = \"t\"\n");
        }
        let mut spec: Spec = toml::from_str(&format!("{head}{annot}")).unwrap();
        spec.publish = publish;
        spec
    }

    fn annot_toml(out: &str, post: Option<usize>) -> String {
        let post = post.map_or(String::new(), |p| format!("post = {p}\n"));
        format!(
            "[[annot]]\nout = \"{out}\"\n{post}title = \"t\"\nsource = \"s\"\n\
                 [[annot.note]]\nquote = \"q\"\ngloss = \"g\"\n"
        )
    }

    fn names(spec: &Spec) -> Vec<String> {
        spec.outputs()
    }

    #[test]
    fn publish_numbers_follow_the_main_posts() {
        // Out of order in the file; post 3 has no screenshot.
        let annots = [
            annot_toml("late", Some(2)),
            annot_toml("first", Some(1)),
            annot_toml("also-late", Some(2)),
        ]
        .concat();
        let spec = posts_spec(&annots, true);
        assert!(
            spec.structure_errors().is_empty(),
            "{:?}",
            spec.structure_errors()
        );
        assert_eq!(
            names(&spec),
            [
                "publish/01-tldr.png",
                "publish/02-roundup.png",
                "publish/03-event-1.png",
                "publish/04-bark-1.png",
                "publish/05-first.png",
                "publish/06-event-2.png",
                "publish/07-bark-2.png",
                "publish/08-late.png",
                "publish/09-also-late.png",
                "publish/10-event-3.png",
                "publish/11-bark-3.png",
            ]
        );
        // The covers lead the upload set but are not rendered here.
        let items = spec.items();
        assert_eq!(items[0].0, "publish/00-cover.png");
        assert_eq!(items[1].0, "publish/00-cover-wide.png");
        assert!(items[..2].iter().all(|(_, j)| matches!(j, Job::Picked)));
        assert_eq!(items.len(), names(&spec).len() + 2);
        // A post without a screenshot is a warning, not an error.
        let warns = spec.structure_warnings();
        assert_eq!(warns.len(), 1);
        assert!(warns[0].contains("第 3 个主帖") && warns[0].contains("post = 3"));
        // Cards of the same post come before its screenshots, by file name.
        assert_eq!(spec.event_outputs().len(), 3);
        assert_eq!(spec.bark_outputs()[2], "publish/11-bark-3.png");
        let mut sorted = names(&spec);
        sorted.sort();
        assert_eq!(sorted, names(&spec), "name order is upload order");

        // Moving a screenshot to another post renumbers what follows.
        let moved = [
            annot_toml("late", Some(2)),
            annot_toml("first", Some(3)),
            annot_toml("also-late", Some(2)),
        ]
        .concat();
        let after = names(&posts_spec(&moved, true));
        assert_eq!(after[4], "publish/05-event-2.png");
        assert_eq!(after[5], "publish/06-bark-2.png");
        assert_eq!(after[6], "publish/07-late.png");
        assert_eq!(after[10], "publish/11-first.png");
    }

    #[test]
    fn old_layout_keeps_flat_names_and_rejects_post() {
        let annots = [
            "[[annot]]\nout = \"41-x.png\"\ntitle = \"t\"\nsource = \"s\"\n\
                 [[annot.note]]\nquote = \"q\"\ngloss = \"g\"\n",
            "[[annot]]\nout = \"40-y.png\"\ntitle = \"t\"\nsource = \"s\"\n\
                 [[annot.note]]\nquote = \"q\"\ngloss = \"g\"\n",
        ]
        .concat();
        let spec = posts_spec(&annots, false);
        assert!(
            spec.structure_errors().is_empty(),
            "{:?}",
            spec.structure_errors()
        );
        assert!(spec.structure_warnings().is_empty());
        assert_eq!(
            names(&spec),
            [
                "00-tldr.png",
                "00-roundup.png",
                "00-event-1.png",
                "00-bark-1.png",
                "00-event-2.png",
                "00-bark-2.png",
                "00-event-3.png",
                "00-bark-3.png",
                "41-x.png",
                "40-y.png",
            ]
        );
        // No covers in the list, and nothing under publish/.
        assert_eq!(spec.items().len(), names(&spec).len());
        assert_eq!(spec.out_dir(Path::new("images")), Path::new("images"));
        let a = &spec.annot[0];
        assert_eq!(spec.raw_file(a).as_deref(), Some("41-x-raw.png"));
        // `post` belongs to the new layout.
        let with_post = posts_spec(&annot_toml("41-x.png", Some(1)), false);
        assert!(
            with_post
                .structure_errors()
                .join("\n")
                .contains("不用 post")
        );
    }

    #[test]
    fn publish_annot_names_and_posts_are_checked() {
        let errs = |annot: String| posts_spec(&annot, true).structure_errors().join("\n");
        // Fine: a digit-led stem that is not a number prefix.
        assert!(errs(annot_toml("36kr-overstated", Some(1))).is_empty());
        for (out, needle) in [
            ("03-carbon", "序号前缀"),
            ("carbon.png", "扩展名"),
            ("a/b", "扩展名"),
            ("carbón", "英文字母"),
            ("tldr", "工具自己用"),
            ("Event-2", "工具自己用"),
            ("cover-wide", "工具自己用"),
            ("", "为空"),
        ] {
            let e = errs(annot_toml(out, Some(1)));
            assert!(e.contains(needle), "{out}: {e}");
        }
        assert!(errs(annot_toml("x", None)).contains("缺少 post"));
        assert!(errs(annot_toml("x", Some(0))).contains("越界"));
        let e = errs(annot_toml("x", Some(4)));
        assert!(e.contains("post = 4 越界") && e.contains("3 个主帖"), "{e}");
        let dup = [annot_toml("Same", Some(1)), annot_toml("same", Some(2))].concat();
        assert!(errs(dup).contains("重复"));
        // An out-of-range screenshot is still listed (last), so nothing is lost.
        let bad = posts_spec(&annot_toml("x", Some(9)), true);
        assert_eq!(names(&bad).last().unwrap(), "publish/09-x.png");
    }

    #[test]
    fn publish_roundup_issue_numbers_from_one() {
        let mut solo: Spec = toml::from_str(
            "kind = \"roundup\"\ndate = \"d\"\n\
                 [[roundup]]\ntag = \"a\"\ntext = \"t\"\n[[roundup]]\ntag = \"b\"\ntext = \"t\"\n\
                 [[roundup]]\ntag = \"c\"\ntext = \"t\"\n[[roundup]]\ntag = \"d\"\ntext = \"t\"\n\
                 [[roundup]]\ntag = \"e\"\ntext = \"t\"\n",
        )
        .unwrap();
        solo.publish = true;
        assert!(solo.structure_errors().is_empty());
        assert_eq!(
            names(&solo),
            [
                "publish/00-cover.png",
                "publish/00-cover-wide.png",
                "publish/01-roundup.png"
            ]
        );
        assert!(solo.structure_warnings().is_empty());
        // The template covers are rendered; a post number makes no sense here.
        assert!(
            solo.jobs()
                .iter()
                .take(2)
                .all(|(_, j)| matches!(j, Job::Cover(_)))
        );
        let with_post: Spec = {
            let mut s = toml::from_str::<Spec>(&format!(
                "kind = \"roundup\"\ndate = \"d\"\n{}{}",
                (0..5)
                    .map(|i| format!("[[roundup]]\ntag = \"t{i}\"\ntext = \"t\"\n"))
                    .collect::<String>(),
                annot_toml("x", Some(1))
            ))
            .unwrap();
            s.publish = true;
            s
        };
        assert!(
            with_post
                .structure_errors()
                .join("\n")
                .contains("不写 post")
        );
    }

    #[test]
    fn raw_base_has_no_number_in_the_new_layout() {
        let spec = posts_spec(&annot_toml("carbon-rollout", Some(1)), true);
        let a = &spec.annot[0];
        assert_eq!(spec.raw_file(a).as_deref(), Some("carbon-rollout-raw.png"));
        assert_eq!(
            spec.raw_path(a, Path::new("images")),
            Some(Path::new("images").join("carbon-rollout-raw.png"))
        );
        assert_eq!(
            spec.out_dir(Path::new("images")),
            Path::new("images").join("publish")
        );
        // `03-event-1.png` on the command line means the file in publish/.
        assert_eq!(spec.output_name("03-event-1.png"), "publish/03-event-1.png");
        assert_eq!(
            spec.output_name("publish/03-event-1.png"),
            "publish/03-event-1.png"
        );
        assert_eq!(
            posts_spec("", false).output_name("00-tldr.png"),
            "00-tldr.png"
        );
    }

    #[test]
    fn leftovers_are_only_this_tools_old_numbering() {
        // Now: tldr 01, roundup 02, event-1 03, bark-1 04, first 05, ...
        let annots = [annot_toml("first", Some(1)), annot_toml("late", Some(2))].concat();
        let spec = posts_spec(&annots, true);
        let listed = [
            "00-cover.png",
            "00-cover-wide.png",
            "01-tldr.png",
            "02-roundup.png",
            "03-event-1.png",
            "04-bark-1.png",
            "05-first.png",
            "05-event-2.png", // renumbered since: old number
            "06-bark-2.png",  // renumbered since: old number
            "06-late.png",    // current name is 08-late.png, so 06 is old
            "08-late.png",
            "09-removed-from-toml.png", // not in cards.toml any more: unknown
            "07-event-3.png",
            "00-tldr.png", // never ours to delete here
            "Thumbs.db",
            "notes.txt",
            "00-COVER.PNG",
        ]
        .map(String::from);
        let (stale, unknown) = spec.leftovers(&listed);
        assert_eq!(
            stale,
            [
                "05-event-2.png",
                "06-bark-2.png",
                "06-late.png",
                "07-event-3.png"
            ]
            .map(String::from),
            "{stale:?}"
        );
        assert_eq!(
            unknown,
            [
                "09-removed-from-toml.png",
                "00-tldr.png",
                "Thumbs.db",
                "notes.txt"
            ]
            .map(String::from)
        );
    }

    #[test]
    fn clearing_leftovers_deletes_only_stale_products() {
        let annots = [annot_toml("first", Some(1)), annot_toml("late", Some(2))].concat();
        let spec = posts_spec(&annots, true);
        let images = scratch("clear");
        let publish = images.join("publish");
        fs::create_dir_all(&publish).unwrap();
        // A sub-directory is neither stale nor deleted.
        fs::create_dir_all(publish.join("keep-dir")).unwrap();
        let files = [
            "00-cover.png",      // by hand: kept
            "00-cover-wide.png", // by hand: kept
            "03-event-1.png",    // current: kept
            "04-bark-1.png",     // current: kept
            "09-bark-2.png",     // old number of a tool product: deleted
            "04-first.png",      // old number: deleted
            "10-removed.png",    // not ours to know: kept, reported
            "notes.txt",         // unknown: kept
        ];
        for f in files {
            fs::write(publish.join(f), b"x").unwrap();
        }
        // The raw base in images/ is out of reach.
        fs::write(images.join("first-raw.png"), b"x").unwrap();
        let mut cache = Cache::default();
        cache.insert("publish/09-bark-2.png", "fp".into());
        cache.insert("publish/03-event-1.png", "fp".into());

        clear_leftovers(&spec, &images, &mut cache, None);

        let left: Vec<String> = publish_files(&publish);
        assert_eq!(
            left,
            [
                "00-cover-wide.png",
                "00-cover.png",
                "03-event-1.png",
                "04-bark-1.png",
                "10-removed.png",
                "notes.txt"
            ]
        );
        assert!(images.join("first-raw.png").is_file());
        assert!(publish.join("keep-dir").is_dir());
        assert!(cache.get("publish/09-bark-2.png").is_none());
        assert!(cache.get("publish/03-event-1.png").is_some());

        // The old layout never deletes anything.
        let old = posts_spec("", false);
        fs::write(publish.join("04-bark-1.png"), b"y").unwrap();
        clear_leftovers(&old, &images, &mut cache, None);
        assert!(publish.join("04-bark-1.png").is_file());
        let _ = fs::remove_dir_all(&images);
    }

    #[test]
    fn the_layout_follows_the_issue_date() {
        let layout = |yymm: &str, mmdd: &str| {
            let root = scratch(&format!("era-{yymm}-{mmdd}"));
            let images = root.join("docs").join(yymm).join(mmdd).join("images");
            fs::create_dir_all(&images).unwrap();
            fs::write(images.join(SPEC_FILE), "date = \"d\"\n").unwrap();
            let publish = load_spec(&images).unwrap().unwrap().publish;
            let _ = fs::remove_dir_all(&root);
            publish
        };
        assert!(!layout("2610", "1028"));
        assert!(!layout("2610", "1027"));
        assert!(!layout("2609", "0930"));
        assert!(layout("2610", "1029"));
        assert!(layout("2610", "1030"));
        assert!(layout("2611", "0101"));
        // A path that is not an issue directory gets the current rules.
        let loose = scratch("era-loose");
        fs::write(loose.join(SPEC_FILE), "date = \"d\"\n").unwrap();
        assert!(load_spec(&loose).unwrap().unwrap().publish);
        let _ = fs::remove_dir_all(&loose);
        assert!(!publish_era(Some(2610_1028)) && publish_era(Some(PUBLISH_FROM)));
    }

    /// The committed issues from before the change keep their file names and
    /// pass the structure rules: 1028 is checked too while it is in the tree.
    #[test]
    fn published_issues_keep_their_flat_layout() {
        let docs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/2610");
        let mut seen = 0;
        for issue in ["1026", "1027", "1028"] {
            let images = docs.join(issue).join("images");
            let Some(spec) = load_spec(&images).unwrap_or(None) else {
                continue;
            };
            seen += 1;
            assert!(!spec.publish, "{issue}");
            assert!(spec.structure_errors().is_empty(), "{issue}");
            assert!(spec.structure_warnings().is_empty(), "{issue}");
            let outs = spec.outputs();
            assert!(outs.iter().all(|o| !o.contains('/')), "{issue}: {outs:?}");
            assert_eq!(outs.len(), spec.items().len(), "{issue}");
            assert_eq!(outs[0], TLDR_OUT, "{issue}");
            assert_eq!(outs[1], ROUNDUP_OUT, "{issue}");
            for a in &spec.annot {
                assert!(outs.contains(&a.out), "{issue}: {}", a.out);
                assert_eq!(
                    spec.raw_file(a).unwrap(),
                    format!("{}-raw.png", a.out.trim_end_matches(".png")),
                    "{issue}"
                );
            }
            assert_eq!(spec.out_dir(&images), images);
        }
        // 1026 and 1027 are committed; without them the test proves nothing.
        assert!(seen >= 2, "found {seen} of the published issues");
    }
}
