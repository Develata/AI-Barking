//! `barking shot`: screenshots of web pages and PDF pages, cut and stacked
//! into the base images (`NN-name-raw.png`) that annotation cards are built on.
//!
//! The rule is the one the per-issue scripts (`a-shot.py`, `a-pdf-shots.py`,
//! `m-raw-crops.py`) kept by hand: pixels are only cropped and joined, never
//! resampled, recoloured or "cleaned", because a screenshot is evidence. Every
//! output refuses to overwrite, and every action that produces an image
//! appends one JSON line to `sources/<group>-shots-log.jsonl`, so a base image
//! can be traced back to its URL, anchors and box.
//!
//! Intermediate renders live in the temp directory; only crops and stacks are
//! written into the issue's `images/`. Text anchors come from the PDF text
//! layer (a `.lines.tsv` sidecar written by `pdf`) or, failing that, from
//! Tesseract line boxes.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output, Stdio};
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

use png::{BitDepth, ColorType, ScaledFloat, SourceChromaticities, SrgbRenderingIntent, Unit};
use serde_json::{Value, json};

use crate::card;
use crate::repo::{self, Issue};

const USAGE: &str = "\
用法：
  barking shot <期次目录> <组前缀> render  <url> <name> [--width 700] [--height 1600] [--scale 2] [--chrome-arg <参数>]...
  barking shot <期次目录> <组前缀> pdf     <pdf> <页码> <name> [--dpi 144]
  barking shot <期次目录> <组前缀> lines   (<name> | --from <png>) [--lang eng]
  barking shot <期次目录> <组前缀> crop    (<name> | --from <png>) <out.png> --start \"<锚点>\" --end \"<锚点>\" [--pad-top 24] [--pad-bottom 24] [--url <url>] [--lang eng]
  barking shot <期次目录> <组前缀> cropbox (<name> | --from <png>) <out.png> <x0> <y0> <x1> <y1> [--url <url>]
  barking shot <期次目录> <组前缀> stack   <out.png> --part <png>:<x0>,<y0>,<x1>,<y1> [--part ...]... [--gap 24] [--bg auto|#rrggbb] [--url <url>]

说明：
  <组前缀>、<name> 只许小写字母、数字与 -。render / pdf 只产生中间图，放在 <临时目录>/barking-shot/<YYMM><MMDD>-<组>/<name>.png；
  先用 lines 找锚点行，再用 crop / cropbox / stack 裁成 <期次目录>/images/<out.png>（已存在则拒绝覆盖）。
  --part 的 <png> 可写文件路径，或 render / pdf 的 <name>；像素框取 [x0,x1) × [y0,y1)，单位是该图的像素。
  选项可写 --key value 或 --key=value；--help 打印本说明。
  每次出图在 sources/<组>-shots-log.jsonl 追加一行记录（路径记仓库内相对路径，库外文件只记文件名；
  URL 与 --chrome-arg 去掉 user:pass@，键名像凭据的查询参数值记为 [REDACTED]）。
  只裁切、拼接，不改像素；iCCP / sRGB / gAMA / cHRM / pHYs 原样带到输出；含 cICP / mDCV / cLLI 的 PNG 拒绝处理。
  pdf 需要 poppler：先找环境变量 BARKING_POPPLER 指定的目录，再找 ~/scoop/apps/poppler/current/bin，最后 PATH。";

/// A hung Chrome must not hang the session; the page timeout flag alone does
/// not bound the process.
const CHROME_TIMEOUT: Duration = Duration::from_secs(240);
/// Upper bound on decoded or composed pixel data, so a typo in `--part` or
/// `--height` fails fast instead of allocating the machine away.
const MAX_BYTES: usize = 1 << 30;
const DEFAULT_DPI: u32 = 144;
const MIN_DPI: u32 = 36;
const MAX_DPI: u32 = 600;
const DEFAULT_PAD: u32 = 24;
const DEFAULT_LANG: &str = "eng";
/// Extensions of the files written next to a scratch render.
const SIDECAR_LINES: &str = "lines.tsv";
const SIDECAR_TOOL: &str = "tool.txt";

// ---------------------------------------------------------------------------
// Command line

#[derive(Debug, PartialEq)]
enum Source {
    /// A render / pdf result in the scratch directory.
    Name(String),
    File(PathBuf),
}

/// Pixel box `[x0, x1) × [y0, y1)`, as PIL's `Image.crop` takes it.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
}

#[derive(Debug, PartialEq)]
struct Part {
    src: Source,
    rect: Rect,
}

#[derive(Debug, PartialEq)]
enum Bg {
    /// The (5, 5) pixel of the first part's source image: the page colour.
    Auto,
    Rgb([u8; 3]),
}

#[derive(Debug, PartialEq)]
struct Render {
    url: String,
    name: String,
    width: u32,
    height: u32,
    scale: f64,
    chrome_args: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct PdfPage {
    pdf: PathBuf,
    page: u32,
    name: String,
    dpi: u32,
}

#[derive(Debug, PartialEq)]
struct AnchorCrop {
    src: Source,
    out: String,
    start: String,
    end: String,
    pad_top: u32,
    pad_bottom: u32,
    url: String,
    lang: String,
}

#[derive(Debug, PartialEq)]
enum Action {
    Render(Render),
    Pdf(PdfPage),
    Lines {
        src: Source,
        lang: String,
    },
    Crop(AnchorCrop),
    Cropbox {
        src: Source,
        out: String,
        rect: Rect,
        url: String,
    },
    Stack {
        out: String,
        parts: Vec<Part>,
        gap: u32,
        bg: Bg,
        url: String,
    },
}

#[derive(Debug, PartialEq)]
struct Parsed {
    dir: String,
    group: String,
    action: Action,
}

/// `--key value` / `--key=value` options plus positionals, split before any
/// action looks at them. Every option takes a value, so `--chrome-arg
/// --no-sandbox` is one option, not two (`--chrome-arg=--no-sandbox` also
/// works).
struct Args {
    pos: Vec<String>,
    opts: Vec<(String, String)>,
}

fn split_args(rest: &[String]) -> Result<Args, String> {
    let mut args = Args {
        pos: Vec::new(),
        opts: Vec::new(),
    };
    let mut it = rest.iter();
    while let Some(t) = it.next() {
        if let Some(opt) = t.strip_prefix("--") {
            let (key, value) = match opt.split_once('=') {
                Some((k, v)) => (k, v.to_string()),
                None => (
                    opt,
                    it.next()
                        .ok_or_else(|| format!("--{opt} 缺少参数"))?
                        .clone(),
                ),
            };
            if key.is_empty() {
                return Err(format!("选项「{t}」缺少名字"));
            }
            args.opts.push((key.to_string(), value));
        } else {
            args.pos.push(t.clone());
        }
    }
    Ok(args)
}

/// `--help` / `-h` where an option name may stand, read the way `split_args`
/// reads options: the token after `--key` is that option's value, so
/// `--chrome-arg=--help` is not a help request (the spaced form
/// `--chrome-arg --help` is not either).
fn wants_help(args: &[String]) -> bool {
    let mut it = args.iter();
    while let Some(t) = it.next() {
        match t.as_str() {
            "--help" | "-h" => return true,
            _ => {
                if let Some(opt) = t.strip_prefix("--")
                    && !opt.contains('=')
                {
                    it.next();
                }
            }
        }
    }
    false
}

impl Args {
    /// A single-valued option; giving it twice is an error, not "last wins".
    fn take(&mut self, key: &str) -> Result<Option<String>, String> {
        let mut found = self.take_all(key);
        if found.len() > 1 {
            return Err(format!("--{key} 只能给一次"));
        }
        Ok(found.pop())
    }

    fn take_all(&mut self, key: &str) -> Vec<String> {
        let (hit, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.opts)
            .into_iter()
            .partition(|(k, _)| k == key);
        self.opts = rest;
        hit.into_iter().map(|(_, v)| v).collect()
    }

    fn number<T: FromStr>(&mut self, key: &str, default: T) -> Result<T, String> {
        match self.take(key)? {
            None => Ok(default),
            Some(s) => parse_num(&format!("--{key}"), &s),
        }
    }

    /// Positionals are consumed in order and `Pos::done` rejects leftovers, so
    /// each action states its full shape in one place.
    fn positionals(&mut self) -> Pos {
        Pos {
            it: std::mem::take(&mut self.pos).into_iter(),
        }
    }

    fn finish(&self) -> Result<(), String> {
        match self.opts.first() {
            Some((k, _)) => Err(format!("这个动作不认识选项 --{k}")),
            None => Ok(()),
        }
    }
}

struct Pos {
    it: std::vec::IntoIter<String>,
}

impl Pos {
    fn next(&mut self, what: &str) -> Result<String, String> {
        self.it.next().ok_or_else(|| format!("缺少参数 {what}"))
    }

    fn done(mut self) -> Result<(), String> {
        match self.it.next() {
            Some(extra) => Err(format!("多余的参数「{extra}」")),
            None => Ok(()),
        }
    }
}

fn parse_num<T: FromStr>(what: &str, s: &str) -> Result<T, String> {
    s.parse()
        .map_err(|_| format!("{what} 须为非负整数或数字，收到「{s}」"))
}

fn is_name(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn check_name(what: &str, s: &str) -> Result<(), String> {
    if is_name(s) {
        Ok(())
    } else {
        Err(format!("{what}「{s}」只许小写字母、数字与 -"))
    }
}

/// Output files go straight under `images/`: no path, and no characters that
/// mean something to the filesystem (NTFS `:` streams, separators, wildcards).
fn check_out(out: &str) -> Result<(), String> {
    let stem = out.strip_suffix(".png").unwrap_or("");
    let plain = |c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.');
    if stem.is_empty() || !out.chars().all(plain) {
        return Err(format!(
            "输出名「{out}」须为 images/ 下的 .png 文件名（字母、数字、- _ .），不含路径"
        ));
    }
    Ok(())
}

fn check_url(url: &str) -> Result<(), String> {
    if ["http://", "https://", "file://"]
        .iter()
        .any(|p| url.starts_with(p))
    {
        Ok(())
    } else {
        Err(format!(
            "URL「{url}」须以 http://、https:// 或 file:// 开头"
        ))
    }
}

/// `--url`, optional: it only labels the log line with the page the image came
/// from, so it may be absent but not malformed.
fn take_url(a: &mut Args) -> Result<String, String> {
    match a.take("url")? {
        None => Ok(String::new()),
        Some(u) => check_url(&u).map(|()| u),
    }
}

/// `<name>` or `--from <png>` for the action's source image.
fn take_source(from: Option<String>, pos: &mut Pos) -> Result<Source, String> {
    match from {
        Some(p) => Ok(Source::File(PathBuf::from(p))),
        None => {
            let name = pos.next("<name>（或用 --from <png>）")?;
            check_name("name", &name)?;
            Ok(Source::Name(name))
        }
    }
}

/// `<png>:<x0>,<y0>,<x1>,<y1>`. Split on the last `:`, since a Windows path
/// has one after the drive letter.
fn parse_part(s: &str) -> Result<Part, String> {
    let bad = || format!("--part「{s}」应写成 <png>:<x0>,<y0>,<x1>,<y1>");
    let (src, coords) = s.rsplit_once(':').ok_or_else(bad)?;
    let n: Vec<u32> = coords
        .split(',')
        .map(|c| c.trim().parse::<u32>())
        .collect::<Result<_, _>>()
        .map_err(|_| bad())?;
    let [x0, y0, x1, y1] = n[..] else {
        return Err(bad());
    };
    if src.is_empty() {
        return Err(bad());
    }
    let src = if is_name(src) {
        Source::Name(src.to_string())
    } else {
        Source::File(PathBuf::from(src))
    };
    Ok(Part {
        src,
        rect: Rect { x0, y0, x1, y1 },
    })
}

/// `#rrggbb`, any hex case.
fn parse_hex(s: &str) -> Result<[u8; 3], String> {
    let bad = || format!("--bg「{s}」应为 auto 或 #rrggbb");
    let hex = s.strip_prefix('#').ok_or_else(bad)?;
    if hex.len() != 6 || !hex.is_ascii() {
        return Err(bad());
    }
    let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| bad());
    Ok([byte(0)?, byte(2)?, byte(4)?])
}

fn parse(args: &[String]) -> Result<Parsed, String> {
    let [dir, group, action, rest @ ..] = args else {
        return Err("参数不足".into());
    };
    check_name("组前缀", group)?;
    let mut a = split_args(rest)?;
    let mut pos = a.positionals();
    let action = match action.as_str() {
        "render" => {
            let url = pos.next("<url>")?;
            let name = pos.next("<name>")?;
            check_url(&url)?;
            check_name("name", &name)?;
            let chrome_args = a.take_all("chrome-arg");
            if let Some(bad) = chrome_args.iter().find(|c| !c.starts_with("--")) {
                return Err(format!(
                    "--chrome-arg「{bad}」应是以 -- 开头的 Chrome 参数（否则 Chrome 会把它当网址）"
                ));
            }
            let width: u32 = a.number("width", 700)?;
            let height: u32 = a.number("height", 1600)?;
            let scale: f64 = a.number("scale", 2.0)?;
            let scale_ok = scale.is_finite() && scale > 0.0 && scale <= 8.0;
            if width == 0 || height == 0 || !scale_ok {
                return Err("--width、--height 须为正整数，--scale 须在 (0, 8] 内".into());
            }
            Action::Render(Render {
                url,
                name,
                width,
                height,
                scale,
                chrome_args,
            })
        }
        "pdf" => {
            let pdf = PathBuf::from(pos.next("<pdf>")?);
            let page: u32 = parse_num("<页码>", &pos.next("<页码>")?)?;
            let name = pos.next("<name>")?;
            check_name("name", &name)?;
            let dpi: u32 = a.number("dpi", DEFAULT_DPI)?;
            if page == 0 {
                return Err("<页码> 从 1 起".into());
            }
            if !(MIN_DPI..=MAX_DPI).contains(&dpi) {
                return Err(format!("--dpi 须在 {MIN_DPI}–{MAX_DPI} 之间"));
            }
            Action::Pdf(PdfPage {
                pdf,
                page,
                name,
                dpi,
            })
        }
        "lines" => {
            let from = a.take("from")?;
            let src = take_source(from, &mut pos)?;
            let lang = a.take("lang")?.unwrap_or_else(|| DEFAULT_LANG.into());
            Action::Lines { src, lang }
        }
        "crop" => {
            let from = a.take("from")?;
            let src = take_source(from, &mut pos)?;
            let out = pos.next("<out.png>")?;
            check_out(&out)?;
            let start = a.take("start")?.ok_or("crop 需要 --start \"<锚点>\"")?;
            let end = a.take("end")?.ok_or("crop 需要 --end \"<锚点>\"")?;
            Action::Crop(AnchorCrop {
                src,
                out,
                start,
                end,
                pad_top: a.number("pad-top", DEFAULT_PAD)?,
                pad_bottom: a.number("pad-bottom", DEFAULT_PAD)?,
                url: take_url(&mut a)?,
                lang: a.take("lang")?.unwrap_or_else(|| DEFAULT_LANG.into()),
            })
        }
        "cropbox" => {
            let from = a.take("from")?;
            let src = take_source(from, &mut pos)?;
            let out = pos.next("<out.png>")?;
            check_out(&out)?;
            let mut coord =
                |what: &str| -> Result<u32, String> { parse_num(what, &pos.next(what)?) };
            let rect = Rect {
                x0: coord("<x0>")?,
                y0: coord("<y0>")?,
                x1: coord("<x1>")?,
                y1: coord("<y1>")?,
            };
            Action::Cropbox {
                src,
                out,
                rect,
                url: take_url(&mut a)?,
            }
        }
        "stack" => {
            let out = pos.next("<out.png>")?;
            check_out(&out)?;
            let parts = a
                .take_all("part")
                .iter()
                .map(|p| parse_part(p))
                .collect::<Result<Vec<_>, _>>()?;
            if parts.is_empty() {
                return Err("stack 至少需要一个 --part".into());
            }
            let bg = match a.take("bg")?.as_deref() {
                None | Some("auto") => Bg::Auto,
                Some(hex) => Bg::Rgb(parse_hex(hex)?),
            };
            Action::Stack {
                out,
                parts,
                gap: a.number("gap", DEFAULT_PAD)?,
                bg,
                url: take_url(&mut a)?,
            }
        }
        other => return Err(format!("不认识的动作「{other}」")),
    };
    pos.done()?;
    a.finish()?;
    Ok(Parsed {
        dir: dir.clone(),
        group: group.clone(),
        action,
    })
}

pub fn run(args: &[String]) -> ExitCode {
    if wants_help(args) {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    let parsed = match parse(args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let result = Issue::resolve(Path::new(&parsed.dir))
        .and_then(|issue| execute(&Ctx::new(issue, &parsed.group), &parsed.action));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

// ---------------------------------------------------------------------------
// Pixels

/// The ancillary chunks that say how the samples are to be shown. A crop that
/// keeps the pixels but drops these changes the colours a viewer renders (an
/// iCCP Display P3 screenshot shown as sRGB looks washed out), so they travel
/// with the pixels. Own types, not `png::Info`: `PixelDimensions` has no
/// `PartialEq`, and `Info` would carry chunks this tool does not vouch for.
#[derive(Clone, Debug, Default, PartialEq)]
struct Meta {
    /// Decompressed iCCP profile (the profile name is not kept by the decoder).
    icc: Option<Vec<u8>>,
    srgb: Option<SrgbRenderingIntent>,
    /// The gAMA chunk as stored, not the value sRGB implies.
    gamma: Option<ScaledFloat>,
    /// The cHRM chunk as stored.
    chrm: Option<SourceChromaticities>,
    /// pHYs: pixels per unit x, y and the unit.
    phys: Option<(u32, u32, Unit)>,
}

impl Meta {
    /// Equal colour chunks; pHYs is physical size, not colour.
    fn same_colour(&self, other: &Meta) -> bool {
        self.icc == other.icc
            && self.srgb == other.srgb
            && self.gamma == other.gamma
            && self.chrm == other.chrm
    }

    /// Short Chinese description for error messages.
    fn describe(&self) -> String {
        let mut parts = Vec::new();
        if let Some(icc) = &self.icc {
            parts.push(format!("iCCP {} 字节", icc.len()));
        }
        if let Some(i) = self.srgb {
            parts.push(format!("sRGB {i:?}"));
        }
        if let Some(g) = self.gamma {
            parts.push(format!("gAMA {}", g.into_scaled()));
        }
        if self.chrm.is_some() {
            parts.push("cHRM".to_string());
        }
        if parts.is_empty() {
            "无色彩块".to_string()
        } else {
            parts.join("、")
        }
    }
}

/// 8-bit image in one of the four PNG layouts that carry samples directly;
/// indexed and low-bit-depth input is expanded on load.
#[derive(Clone, Debug, PartialEq)]
struct Img {
    w: u32,
    h: u32,
    ct: ColorType,
    data: Vec<u8>,
    /// tRNS colour key of an 8-bit gray / RGB image, as decoded samples (1 or
    /// 3 bytes). Kept as a key instead of an alpha channel so a crop is
    /// written in the source's own layout.
    key: Option<Vec<u8>>,
    meta: Meta,
}

/// Colour chunks `Meta` cannot carry through a re-encode: cICP replaces the
/// colour space (HDR PQ / HLG, Display P3 by code point) and mDCV / cLLI tell
/// a display how to tone-map. Dropping them changes how a crop looks, so such
/// a source is refused instead of being cut "losslessly".
const UNKEPT_CHUNKS: [&str; 3] = ["cICP", "mDCV", "cLLI"];

/// The first `UNKEPT_CHUNKS` entry in the raw chunk list, if any. Only walks
/// the chunk headers: a missing signature or a truncated chunk ends the walk
/// silently and is left for the decoder to report.
fn unkept_colour_chunk(bytes: &[u8]) -> Option<&'static str> {
    const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    if !bytes.starts_with(SIGNATURE) {
        return None;
    }
    let mut i = SIGNATURE.len();
    while let Some(head) = bytes.get(i..i.checked_add(8)?) {
        let len = u32::from_be_bytes([head[0], head[1], head[2], head[3]]) as usize;
        let kind = &head[4..8];
        if let Some(hit) = UNKEPT_CHUNKS.iter().find(|c| c.as_bytes() == kind) {
            return Some(*hit);
        }
        if kind == b"IEND" {
            break;
        }
        // Length, type, data, CRC.
        i = i.checked_add(len)?.checked_add(12)?;
    }
    None
}

/// (has colour, has alpha) of a decoded colour type.
fn channels(ct: ColorType) -> (bool, bool) {
    match ct {
        ColorType::Grayscale => (false, false),
        ColorType::GrayscaleAlpha => (false, true),
        ColorType::Rgb => (true, false),
        ColorType::Rgba | ColorType::Indexed => (true, true),
    }
}

impl Rect {
    fn check(self, w: u32, h: u32) -> Result<(), String> {
        if self.x0 < self.x1 && self.y0 < self.y1 && self.x1 <= w && self.y1 <= h {
            Ok(())
        } else {
            Err(format!(
                "裁切框 [{},{},{},{}] 无效或超出 {w}×{h} 的图（要求 x0<x1≤{w}，y0<y1≤{h}）",
                self.x0, self.y0, self.x1, self.y1
            ))
        }
    }

    fn json(self) -> Value {
        json!([self.x0, self.y0, self.x1, self.y1])
    }
}

impl Img {
    fn load(path: &Path) -> Result<Img, String> {
        let bytes = fs::read(path).map_err(io_err(path))?;
        Img::decode(&bytes).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Decodes with `EXPAND` only: palette and sub-byte gray become 8-bit
    /// samples (a palette `tRNS` becomes an alpha channel) and 16-bit stays
    /// 16-bit so it can be refused. Nothing is scaled or gamma-corrected. An
    /// 8-bit gray / RGB image with a `tRNS` colour key keeps its layout: the
    /// alpha `EXPAND` derives from the key is dropped again and the key kept,
    /// so its crop is written back exactly as the source stored it.
    fn decode(bytes: &[u8]) -> Result<Img, String> {
        if let Some(name) = unkept_colour_chunk(bytes) {
            return Err(format!(
                "不支持 cICP/mDCV/cLLI 色彩块的 PNG，先用其他工具转换或另取底图（本图含 {name}）"
            ));
        }
        let limits = png::Limits { bytes: MAX_BYTES };
        let mut dec = png::Decoder::new_with_limits(io::Cursor::new(bytes), limits);
        dec.set_transformations(png::Transformations::EXPAND);
        let mut reader = dec
            .read_info()
            .map_err(|e| format!("不是可读的 PNG：{e}"))?;
        let src = reader.info();
        let meta = Meta {
            icc: src.icc_profile.as_ref().map(|p| p.to_vec()),
            srgb: src.srgb,
            gamma: src.gama_chunk,
            chrm: src.chrm_chunk,
            phys: src.pixel_dims.map(|d| (d.xppu, d.yppu, d.unit)),
        };
        let key = match (src.color_type, src.bit_depth, &src.trns) {
            (ColorType::Grayscale | ColorType::Rgb, BitDepth::Eight, Some(t)) => Some(t.to_vec()),
            _ => None,
        };
        let (ct, depth) = reader.output_color_type();
        match depth {
            BitDepth::Eight => {}
            BitDepth::Sixteen => return Err("不支持 16 位 PNG".into()),
            other => return Err(format!("不支持的 PNG 位深 {other:?}")),
        }
        if ct == ColorType::Indexed {
            return Err("不支持的调色板 PNG".into());
        }
        let size = reader
            .output_buffer_size()
            .filter(|&n| n <= MAX_BYTES)
            .ok_or("PNG 太大，超过 1 GiB 像素数据")?;
        let mut data = vec![0u8; size];
        let info = reader
            .next_frame(&mut data)
            .map_err(|e| format!("PNG 解码失败：{e}"))?;
        data.truncate(info.buffer_size());
        let mut img = Img {
            w: info.width,
            h: info.height,
            ct: info.color_type,
            data,
            key: None,
            meta,
        };
        if let Some(k) = key {
            // EXPAND appended an alpha sample computed from the key and left
            // the colour samples alone (png `expand_trns_line`).
            let (bare, samples) = match img.ct {
                ColorType::GrayscaleAlpha => (ColorType::Grayscale, 1),
                ColorType::Rgba => (ColorType::Rgb, 3),
                other => return Err(format!("内部错误：带 tRNS 的图解码成了 {other:?}")),
            };
            if k.len() != samples {
                return Err(format!("tRNS 长度 {} 与色彩类型不符", k.len()));
            }
            img.data = img
                .data
                .chunks_exact(samples + 1)
                .flat_map(|px| px[..samples].iter().copied())
                .collect();
            img.ct = bare;
            img.key = Some(k);
        }
        Ok(img)
    }

    /// 8-bit samples in `ct`, plus the colour chunks the source carried. The
    /// encoder only writes iCCP when no sRGB is set and only writes a gAMA /
    /// cHRM that equals the sRGB values when one is, so sRGB is left out of
    /// the `Info` and written as a raw chunk instead: every chunk the source
    /// had comes back, even the (non-conforming) sRGB + iCCP pair.
    fn encode(&self) -> Result<Vec<u8>, String> {
        let fail = |e: png::EncodingError| format!("无法编码 PNG：{e}");
        let mut info = png::Info::with_size(self.w, self.h);
        info.color_type = self.ct;
        info.bit_depth = BitDepth::Eight;
        info.icc_profile = self.meta.icc.as_deref().map(Into::into);
        info.source_gamma = self.meta.gamma;
        info.source_chromaticities = self.meta.chrm;
        info.pixel_dims = self
            .meta
            .phys
            .map(|(xppu, yppu, unit)| png::PixelDimensions { xppu, yppu, unit });
        // tRNS stores 16-bit samples; the decoder hands back their low bytes.
        info.trns = self
            .key
            .as_ref()
            .map(|k| k.iter().flat_map(|&b| [0, b]).collect::<Vec<u8>>().into());
        let mut buf = Vec::new();
        let enc = png::Encoder::with_info(&mut buf, info).map_err(fail)?;
        let mut writer = enc.write_header().map_err(fail)?;
        if let Some(intent) = self.meta.srgb {
            writer
                .write_chunk(png::chunk::sRGB, &[intent as u8])
                .map_err(fail)?;
        }
        writer.write_image_data(&self.data).map_err(fail)?;
        writer.finish().map_err(fail)?;
        Ok(buf)
    }

    /// Row-wise byte copy: the colour type and every sample survive unchanged.
    fn crop(&self, r: Rect) -> Result<Img, String> {
        r.check(self.w, self.h)?;
        let bpp = self.ct.samples();
        let stride = self.w as usize * bpp;
        let (from, to) = (r.x0 as usize * bpp, r.x1 as usize * bpp);
        let rows = (r.y1 - r.y0) as usize;
        let mut data = Vec::with_capacity((to - from) * rows);
        for row in self
            .data
            .chunks_exact(stride)
            .skip(r.y0 as usize)
            .take(rows)
        {
            data.extend_from_slice(&row[from..to]);
        }
        Ok(Img {
            w: r.x1 - r.x0,
            h: r.y1 - r.y0,
            ct: self.ct,
            data,
            key: self.key.clone(),
            meta: self.meta.clone(),
        })
    }

    fn keyed(&self, px: &[u8]) -> bool {
        self.key.as_deref() == Some(px)
    }

    fn rgba_at(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.w || y >= self.h {
            return None;
        }
        let bpp = self.ct.samples();
        let i = (y as usize * self.w as usize + x as usize) * bpp;
        let px = self.data.get(i..i + bpp)?;
        let opaque = if self.keyed(px) { 0 } else { 255 };
        Some(match self.ct {
            ColorType::Grayscale => [px[0], px[0], px[0], opaque],
            ColorType::GrayscaleAlpha => [px[0], px[0], px[0], px[1]],
            ColorType::Rgb => [px[0], px[1], px[2], opaque],
            _ => [px[0], px[1], px[2], px[3]],
        })
    }

    /// Turns a tRNS colour key into the alpha channel it stands for (0 on the
    /// key colour, 255 elsewhere), the way a PNG viewer reads it. Lossless, and
    /// needed before parts are joined: one key cannot describe a canvas whose
    /// parts came from different images or whose gaps are filled with `bg`.
    fn unkey(self) -> Img {
        let Some(key) = &self.key else {
            return self;
        };
        let (ct, n) = match self.ct {
            ColorType::Grayscale => (ColorType::GrayscaleAlpha, 1),
            _ => (ColorType::Rgba, 3),
        };
        let mut data = Vec::with_capacity(self.data.len() / n * (n + 1));
        for px in self.data.chunks_exact(n) {
            data.extend_from_slice(px);
            data.push(if px == &key[..] { 0 } else { 255 });
        }
        Img {
            ct,
            data,
            key: None,
            ..self
        }
    }

    /// Widens to a layout with at least the same channels: gray repeats into
    /// R=G=B and a missing alpha is opaque. Narrowing would drop information,
    /// so it is an error rather than a silent loss.
    fn promote(self, to: ColorType) -> Result<Img, String> {
        if self.key.is_some() {
            return Err("内部错误：带 tRNS 的图须先 unkey 再转换".into());
        }
        if self.ct == to {
            return Ok(self);
        }
        let ((colour, alpha), (to_colour, to_alpha)) = (channels(self.ct), channels(to));
        if (colour && !to_colour) || (alpha && !to_alpha) {
            return Err(format!("内部错误：{:?} 不能无损转成 {to:?}", self.ct));
        }
        // A gray ICC profile is not valid on an RGB image, and swapping it for
        // anything else changes how the pixels look.
        if self.meta.icc.is_some() && colour != to_colour {
            return Err(gray_icc_msg(None, to));
        }
        let mut data = Vec::with_capacity(self.w as usize * self.h as usize * to.samples());
        for px in self.data.chunks_exact(self.ct.samples()) {
            let rgb = if colour {
                [px[0], px[1], px[2]]
            } else {
                [px[0]; 3]
            };
            if to_colour {
                data.extend_from_slice(&rgb);
            } else {
                data.push(rgb[0]);
            }
            if to_alpha {
                data.push(if alpha { px[px.len() - 1] } else { 255 });
            }
        }
        Ok(Img {
            ct: to,
            data,
            ..self
        })
    }
}

/// Why a gray part tagged with an ICC profile cannot join a colour canvas.
fn gray_icc_msg(part: Option<usize>, to: ColorType) -> String {
    let which = part.map_or("灰度图".to_string(), |i| format!("第 {i} 段是灰度图且"));
    format!(
        "{which}带 ICC 配置文件，拼接需要把它提升为 {to:?}（彩色背景，或与彩色、带透明的段混拼），\
         灰度 ICC 用在彩色图上不合规，换掉又会改变显示颜色；请把各段分别出图，或去掉彩色背景（--bg 用灰色）"
    )
}

/// Layout of a stack: the parts' own layout when they all agree and can hold
/// the background; RGB when gray parts meet a coloured background; RGBA as soon
/// as the parts disagree, so no channel is ever dropped.
fn stack_type(types: &[ColorType], bg: [u8; 4]) -> ColorType {
    let first = types.first().copied().unwrap_or(ColorType::Rgba);
    if types.iter().any(|&t| t != first) {
        return ColorType::Rgba;
    }
    let (colour, alpha) = channels(first);
    let chromatic = bg[0] != bg[1] || bg[1] != bg[2];
    match (colour || chromatic, alpha || bg[3] != 255) {
        (false, false) => ColorType::Grayscale,
        (false, true) => ColorType::GrayscaleAlpha,
        (true, false) => ColorType::Rgb,
        (true, true) => ColorType::Rgba,
    }
}

/// One background pixel in layout `ct` (gray layouts only get gray `bg`, see
/// `stack_type`).
fn bg_pixel(ct: ColorType, bg: [u8; 4]) -> Vec<u8> {
    match ct {
        ColorType::Grayscale => vec![bg[0]],
        ColorType::GrayscaleAlpha => vec![bg[0], bg[3]],
        ColorType::Rgb => bg[..3].to_vec(),
        _ => bg.to_vec(),
    }
}

/// The canvas's chunks: the parts' colour chunks, which must agree, since one
/// PNG has one colour space and re-tagging a part would change how it looks;
/// pHYs only when every part has the same one (a Chrome render and a pdftoppm
/// page differ there, and that must not block the stack).
fn stack_meta(parts: &[Img]) -> Result<Meta, String> {
    let first = &parts.first().ok_or("没有可拼接的片段")?.meta;
    if let Some((i, odd)) = parts
        .iter()
        .enumerate()
        .find(|(_, p)| !p.meta.same_colour(first))
    {
        return Err(format!(
            "各段色彩元数据不一致：第 1 段为「{}」，第 {} 段为「{}」。拼在一张图里会让其中一段按错误的色彩空间显示；\
             请把不同来源分别裁切出图，或从同一来源（同一次 render / pdf）取各段",
            first.describe(),
            i + 1,
            odd.meta.describe()
        ));
    }
    let phys = first
        .phys
        .filter(|ph| parts.iter().all(|p| p.meta.phys == Some(*ph)));
    Ok(Meta {
        phys,
        ..first.clone()
    })
}

/// Parts joined top to bottom with `gap` rows of `bg` between them; the canvas
/// is as wide as the widest part and the short ones are padded on the right
/// with `bg`. Colour keys become alpha first (see `Img::unkey`).
fn stack(parts: Vec<Img>, gap: u32, bg: [u8; 4]) -> Result<Img, String> {
    let meta = stack_meta(&parts)?;
    let parts: Vec<Img> = parts.into_iter().map(Img::unkey).collect();
    let types: Vec<ColorType> = parts.iter().map(|p| p.ct).collect();
    let ct = stack_type(&types, bg);
    // Before the canvas is allocated; `promote` guards the same case.
    if meta.icc.is_some()
        && channels(ct).0
        && let Some(i) = types.iter().position(|&t| !channels(t).0)
    {
        return Err(gray_icc_msg(Some(i + 1), ct));
    }
    let bpp = ct.samples();
    let w = parts.iter().map(|p| p.w).max().ok_or("没有可拼接的片段")?;
    let rows: u64 = parts.iter().map(|p| u64::from(p.h)).sum::<u64>()
        + u64::from(gap) * (parts.len() as u64 - 1);
    let bytes = u64::from(w) * rows * bpp as u64;
    let h = u32::try_from(rows).map_err(|_| "拼接结果过高")?;
    let bytes = usize::try_from(bytes)
        .ok()
        .filter(|&n| n <= MAX_BYTES)
        .ok_or("拼接结果过大，超过 1 GiB 像素数据")?;

    let row_len = w as usize * bpp;
    let fill: Vec<u8> = bg_pixel(ct, bg).into_iter().cycle().take(row_len).collect();
    let mut data = Vec::with_capacity(bytes);
    for (i, part) in parts.into_iter().enumerate() {
        if i > 0 {
            for _ in 0..gap {
                data.extend_from_slice(&fill);
            }
        }
        let part = part.promote(ct)?;
        let stride = part.w as usize * bpp;
        for row in part.data.chunks_exact(stride) {
            data.extend_from_slice(row);
            data.extend_from_slice(&fill[..row_len - stride]);
        }
    }
    Ok(Img {
        w,
        h,
        ct,
        data,
        key: None,
        meta,
    })
}

/// `--bg auto`: the page colour, read where no content sits.
fn auto_bg(img: &Img) -> Result<[u8; 4], String> {
    img.rgba_at(5, 5).ok_or_else(|| {
        format!(
            "--bg auto 要读第一段来源图的 (5,5) 像素，但它只有 {}×{}；改用 --bg #rrggbb",
            img.w, img.h
        )
    })
}

fn bg_hex(bg: [u8; 4]) -> String {
    let rgb = format!("#{:02x}{:02x}{:02x}", bg[0], bg[1], bg[2]);
    if bg[3] == 255 {
        rgb
    } else {
        format!("{rgb}{:02x}", bg[3])
    }
}

// ---------------------------------------------------------------------------
// Text lines (anchors)

/// One text line with its vertical extent in image pixels.
#[derive(Clone, Debug, PartialEq)]
struct Line {
    y0: u32,
    y1: u32,
    text: String,
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let decoded = rest.find(';').filter(|&j| j <= 10).and_then(|j| {
            let ent = &rest[1..j];
            let ch = match ent {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                _ => ent.strip_prefix('#').and_then(|n| {
                    let code = match n.strip_prefix(['x', 'X']) {
                        Some(hex) => u32::from_str_radix(hex, 16).ok()?,
                        None => n.parse().ok()?,
                    };
                    char::from_u32(code)
                }),
            };
            ch.map(|c| (c, j + 1))
        });
        match decoded {
            Some((c, used)) => {
                out.push(c);
                rest = &rest[used..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The value of `name="…"` in a tag's attribute text.
fn attr_f64(attrs: &str, name: &str) -> Option<f64> {
    let key = format!("{name}=\"");
    let start = attrs.find(&key)? + key.len();
    let len = attrs[start..].find('"')?;
    attrs[start..start + len].parse().ok()
}

/// Lines of `pdftotext -bbox-layout` output, in image pixels at `dpi`. The
/// box is widened to whole pixels (floor above, ceil below) so it encloses
/// the text; points are 1/72 inch, hence × dpi / 72.
fn parse_bbox_lines(xhtml: &str, dpi: u32) -> Vec<Line> {
    let scale = f64::from(dpi) / 72.0;
    let mut out = Vec::new();
    let mut rest = xhtml;
    while let Some(i) = rest.find("<line ") {
        rest = &rest[i + "<line ".len()..];
        let Some(tag_end) = rest.find('>') else { break };
        let attrs = &rest[..tag_end];
        let body_start = tag_end + 1;
        let Some(close) = rest[body_start..].find("</line>") else {
            break;
        };
        let body = &rest[body_start..body_start + close];
        rest = &rest[body_start + close + "</line>".len()..];
        let (Some(top), Some(bottom)) = (attr_f64(attrs, "yMin"), attr_f64(attrs, "yMax")) else {
            continue;
        };
        let mut words = Vec::new();
        let mut inner = body;
        while let Some(w) = inner.find("<word") {
            inner = &inner[w..];
            let (Some(open_end), Some(word_end)) = (inner.find('>'), inner.find("</word>")) else {
                break;
            };
            if open_end < word_end {
                words.push(unescape(&inner[open_end + 1..word_end]));
            }
            inner = &inner[word_end + "</word>".len()..];
        }
        let text = words.join(" ");
        if !text.trim().is_empty() {
            out.push(Line {
                y0: (top * scale).floor().max(0.0) as u32,
                y1: (bottom * scale).ceil().max(0.0) as u32,
                text,
            });
        }
    }
    out.sort_by_key(|l| l.y0);
    out
}

/// Tesseract words grouped by their (block, paragraph, line) key into lines,
/// top to bottom.
fn lines_from_words(words: &[card::Word]) -> Vec<Line> {
    let mut groups: BTreeMap<(u32, u32, u32), Vec<&card::Word>> = BTreeMap::new();
    for w in words {
        groups.entry(w.line).or_default().push(w);
    }
    let mut lines: Vec<Line> = groups
        .into_values()
        .map(|ws| Line {
            y0: ws.iter().map(|w| w.top).min().unwrap_or(0),
            y1: ws.iter().map(|w| w.bottom).max().unwrap_or(0),
            text: ws
                .iter()
                .map(|w| w.text.as_str())
                .collect::<Vec<_>>()
                .join(" "),
        })
        .collect();
    lines.sort_by_key(|l| l.y0);
    lines
}

/// Sidecar format: `y0<TAB>y1<TAB>text` per line.
fn lines_tsv(lines: &[Line]) -> String {
    let mut out = String::new();
    for l in lines {
        let text = l.text.replace(['\t', '\n', '\r'], " ");
        out.push_str(&format!("{}\t{}\t{}\n", l.y0, l.y1, text));
    }
    out
}

fn parse_lines_tsv(text: &str) -> Result<Vec<Line>, String> {
    let mut lines = Vec::new();
    for (n, row) in text.lines().enumerate().filter(|(_, r)| !r.is_empty()) {
        let mut f = row.splitn(3, '\t');
        let parsed = (|| {
            let y0 = f.next()?.parse().ok()?;
            let y1 = f.next()?.parse().ok()?;
            Some(Line {
                y0,
                y1,
                text: f.next()?.to_string(),
            })
        })();
        lines.push(parsed.ok_or_else(|| format!("第 {} 行不是 y0<TAB>y1<TAB>文本", n + 1))?);
    }
    lines.sort_by_key(|l| l.y0);
    Ok(lines)
}

/// Case-insensitive with whitespace runs collapsed, so an anchor typed with
/// one space still finds OCR text that has two.
fn fold(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// First line at or after `from` whose text contains `anchor`.
fn find_line(lines: &[Line], anchor: &str, from: usize) -> Option<usize> {
    let a = fold(anchor);
    lines
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, l)| fold(&l.text).contains(&a))
        .map(|(i, _)| i)
}

fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(n).collect::<String>())
    }
}

#[derive(Debug)]
struct Span {
    rect: Rect,
    start: usize,
    end: usize,
}

/// Full-width box from the start anchor's line to the end anchor's line, padded
/// and clamped to the image. The end anchor is searched from the start line
/// itself, so one line can serve as both. Pre: `lines` sorted by `y0`.
fn anchor_box(
    lines: &[Line],
    start: &str,
    end: &str,
    (pad_top, pad_bottom): (u32, u32),
    (w, h): (u32, u32),
) -> Result<Span, String> {
    if fold(start).is_empty() || fold(end).is_empty() {
        return Err("锚点不能为空".into());
    }
    let s = find_line(lines, start, 0).ok_or_else(|| {
        format!(
            "找不到起始锚点「{start}」（共 {} 行；用 lines 动作查看各行文本）",
            lines.len()
        )
    })?;
    let e = find_line(lines, end, s).ok_or_else(|| {
        format!(
            "找不到结束锚点「{end}」（从起始行「{}」起往下找）",
            clip(&lines[s].text, 40)
        )
    })?;
    let y0 = lines[s].y0.saturating_sub(pad_top);
    let y1 = lines[e].y1.saturating_add(pad_bottom).min(h);
    if y1 <= y0 {
        return Err(format!("锚点区间 y {y0}–{y1} 为空（图高 {h}）"));
    }
    Ok(Span {
        rect: Rect {
            x0: 0,
            y0,
            x1: w,
            y1,
        },
        start: s,
        end: e,
    })
}

/// Crop edges that fall inside a text line cut letters in half. Padding that
/// is larger than the gap to the neighbouring line causes it.
fn cut_warnings(lines: &[Line], rect: Rect) -> Vec<String> {
    let mut out = Vec::new();
    for (edge, which, flag) in [
        (rect.y0, "上沿", "--pad-top"),
        (rect.y1, "下沿", "--pad-bottom"),
    ] {
        if let Some(l) = lines.iter().find(|l| l.y0 < edge && edge < l.y1) {
            out.push(format!(
                "注意：裁切{which} y={edge} 落在文字行内（y {}–{}「{}」），可调小 {flag}",
                l.y0,
                l.y1,
                clip(&l.text, 40)
            ));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// External programs

fn poppler_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(d) = std::env::var_os("BARKING_POPPLER") {
        dirs.push(PathBuf::from(d));
    }
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        dirs.push(Path::new(&home).join("scoop/apps/poppler/current/bin"));
    }
    dirs
}

/// `name` in the first directory that has it, else the bare name for PATH
/// lookup. PATH comes last on purpose: Git Bash puts an old xpdf `pdftotext`
/// there, which has no `-bbox-layout`.
fn pick_tool(dirs: &[PathBuf], name: &str) -> PathBuf {
    let exe = format!("{name}{}", std::env::consts::EXE_SUFFIX);
    dirs.iter()
        .map(|d| d.join(&exe))
        .find(|p| p.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}

fn run_capture(cmd: &mut Command, what: &str) -> Result<Output, String> {
    let out = cmd
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("无法运行 {what}：{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{what} 失败（{}）：{}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(out)
}

/// Chrome arguments in the order the old scripts used. No
/// `--virtual-time-budget` (it cuts pages that load late) and no
/// `--allow-file-access-from-files`: this renders pages as a visitor sees them.
fn chrome_args(profile: &Path, r: &Render, out: &Path) -> Vec<String> {
    let mut args = vec![
        "--headless=new".to_string(),
        format!("--user-data-dir={}", profile.display()),
        "--disable-gpu".to_string(),
        "--hide-scrollbars".to_string(),
        format!("--force-device-scale-factor={}", r.scale),
        format!("--window-size={},{}", r.width, r.height),
        "--timeout=30000".to_string(),
        "--lang=en-US".to_string(),
    ];
    args.extend(r.chrome_args.iter().cloned());
    args.push(format!("--screenshot={}", out.display()));
    args.push(r.url.clone());
    args
}

/// Runs Chrome to completion, killing it after `CHROME_TIMEOUT`. Its stderr
/// goes to a file (not a pipe a lingering child process could hold open) and
/// the tail is quoted on failure.
fn run_chrome(chrome: &Path, args: &[String], log: &Path) -> Result<(), String> {
    let log_file = fs::File::create(log).map_err(io_err(log))?;
    let mut child = Command::new(chrome)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::from(log_file))
        .spawn()
        .map_err(|e| format!("无法运行 {}: {e}", chrome.display()))?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() > CHROME_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "Chrome 超过 {} 秒仍未结束，已终止（日志 {}）",
                    CHROME_TIMEOUT.as_secs(),
                    log.display()
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(e) => return Err(format!("等待 Chrome 失败：{e}")),
        }
    };
    if status.success() {
        return Ok(());
    }
    let tail = fs::read_to_string(log).unwrap_or_default();
    let tail: String = tail
        .trim()
        .chars()
        .rev()
        .take(400)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    Err(format!(
        "Chrome 失败（{status}）：{tail}（日志 {}）",
        log.display()
    ))
}

// ---------------------------------------------------------------------------
// Files, scratch and the log

fn io_err(path: &Path) -> impl Fn(io::Error) -> String + '_ {
    move |e| format!("{}: {e}", path.display())
}

fn slash(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

fn exists_msg(path: &Path) -> String {
    format!(
        "{} 已存在，拒绝覆盖（换个输出名，或先删除旧文件）",
        path.display()
    )
}

/// One JSON object per line with the keys in the order given (serde_json's
/// own map would sort them), matching the old `a-shots-log.jsonl` look.
fn jsonl(fields: &[(&str, Value)]) -> String {
    let body: Vec<String> = fields
        .iter()
        .map(|(k, v)| format!("{}:{v}", Value::from(*k)))
        .collect();
    format!("{{{}}}", body.join(","))
}

fn clear_sidecars(png: &Path) {
    for ext in [SIDECAR_LINES, SIDECAR_TOOL] {
        let _ = fs::remove_file(png.with_extension(ext));
    }
}

/// What a scratch image was made from, written by `render` / `pdf` into its
/// `.tool.txt` sidecar so that later crops can name their source in the log:
/// the tool string on the first line, then `url<TAB>…` or `pdf<TAB>…` and
/// `page<TAB>…`. A one-line file (older scratch) still reads as the tool.
#[derive(Debug, Default, PartialEq)]
struct Prov {
    tool: String,
    url: Option<String>,
    /// Repository-relative path, or the file name of a PDF outside it.
    pdf: Option<String>,
    page: Option<u32>,
}

impl Prov {
    fn text(&self) -> String {
        let mut out = format!("{}\n", self.tool);
        let clean = |s: &str| s.replace(['\t', '\n', '\r'], " ");
        if let Some(u) = &self.url {
            out.push_str(&format!("url\t{}\n", clean(u)));
        }
        if let Some(p) = &self.pdf {
            out.push_str(&format!("pdf\t{}\n", clean(p)));
        }
        if let Some(n) = self.page {
            out.push_str(&format!("page\t{n}\n"));
        }
        out
    }

    fn parse(text: &str) -> Option<Prov> {
        let mut rows = text.lines().map(str::trim).filter(|l| !l.is_empty());
        let mut prov = Prov {
            tool: rows.next()?.to_string(),
            ..Prov::default()
        };
        for row in rows {
            match row.split_once('\t') {
                Some(("url", v)) => prov.url = Some(v.to_string()),
                Some(("pdf", v)) => prov.pdf = Some(v.to_string()),
                Some(("page", v)) => prov.page = v.parse().ok(),
                _ => {}
            }
        }
        Some(prov)
    }

    /// Where the scratch image came from, for a log record.
    fn fields(&self) -> Vec<(&'static str, Value)> {
        let mut out = Vec::new();
        if let Some(p) = &self.pdf {
            out.push(("pdf", json!(p)));
        }
        if let Some(n) = self.page {
            out.push(("page", json!(n)));
        }
        out
    }
}

fn read_prov(png: &Path) -> Option<Prov> {
    Prov::parse(&fs::read_to_string(png.with_extension(SIDECAR_TOOL)).ok()?)
}

fn tool_chrome(r: &Render) -> String {
    format!("headless Chrome {} CSS px, DPR {}", r.width, r.scale)
}

fn tool_pdf(dpi: u32) -> String {
    format!("pdftoppm -r {dpi} + pdftotext -bbox-layout")
}

/// `<source>; <step>`, or just the step when the source is unknown.
fn tool_label(source: Option<&str>, step: &str) -> String {
    match source {
        Some(s) => format!("{s}; {step}"),
        None => step.to_string(),
    }
}

/// `p` as a `/` path relative to `root`, if it lies inside. Canonical forms
/// first (they agree on drive-letter case and resolve `..`), then the plain
/// absolute forms, for paths canonicalize cannot reach.
fn repo_rel(root: &Path, p: &Path) -> Option<String> {
    let strip = |a: &Path, b: &Path| a.strip_prefix(b).ok().map(slash);
    if let (Ok(a), Ok(b)) = (fs::canonicalize(p), fs::canonicalize(root))
        && let Some(rel) = strip(&a, &b)
    {
        return Some(rel);
    }
    strip(&std::path::absolute(p).ok()?, root)
}

/// How a file is named in the committed log: its repository-relative path, or
/// for a file outside the repository only its file name (`outside` = true).
/// Never an absolute local path.
struct LogPath {
    path: String,
    outside: bool,
}

impl LogPath {
    fn of(root: &Path, p: &Path) -> LogPath {
        match repo_rel(root, p) {
            Some(path) => LogPath {
                path,
                outside: false,
            },
            None => LogPath {
                path: p
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                outside: true,
            },
        }
    }
}

/// Query keys whose values are credentials more often than not. Matched as
/// case-insensitive substrings, so `author=` or `keyword=` are redacted too:
/// over-redacting costs a log detail, under-redacting puts a token into a
/// committed file.
const SECRET_KEYS: [&str; 10] = [
    "token",
    "key",
    "secret",
    "auth",
    "sig",
    "signature",
    "session",
    "password",
    "cookie",
    "access",
];
const REDACTED: &str = "[REDACTED]";

/// `%XX` escapes decoded (invalid ones kept literally), lossy UTF-8.
fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        let hex = |c: u8| char::from(c).to_digit(16);
        if b[i] == b'%'
            && let (Some(Some(hi)), Some(Some(lo))) =
                (b.get(i + 1).map(|&c| hex(c)), b.get(i + 2).map(|&c| hex(c)))
        {
            out.push((hi * 16 + lo) as u8);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn secret_key(key: &str) -> bool {
    let key = percent_decode(key).to_lowercase();
    SECRET_KEYS.iter().any(|s| key.contains(s))
}

/// `a=1&token=x` → `a=1&token=[REDACTED]`; parameters without `=` are kept.
fn redact_params(params: &str) -> String {
    params
        .split('&')
        .map(|p| match p.split_once('=') {
            Some((k, _)) if secret_key(k) => format!("{k}={REDACTED}"),
            _ => p.to_string(),
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn starts_with_ci(s: &str, prefix: &str) -> bool {
    s.get(..prefix.len())
        .is_some_and(|h| h.eq_ignore_ascii_case(prefix))
}

/// A `file://` URL as the file it names, through `LogPath`: the repository-
/// relative path, or only the file name. Query and fragment are dropped.
fn log_file_url(root: &Path, url: &str) -> String {
    let rest = url.get("file://".len()..).unwrap_or("");
    let rest = rest.split(['?', '#']).next().unwrap_or("");
    let mut path = percent_decode(rest);
    let b = path.as_bytes();
    if b.len() >= 3 && b[0] == b'/' && b[1].is_ascii_alphabetic() && b[2] == b':' {
        // `file:///C:/x` (what `card::file_url` writes) → `C:/x`.
        path.remove(0);
    } else if !path.starts_with('/') {
        // `file://host/share/x`: a UNC path.
        path.insert_str(0, "//");
    }
    LogPath::of(root, Path::new(&path)).path
}

/// A URL as the committed log may hold it: a `file://` URL becomes a
/// repository-relative path or a bare file name (see `LogPath`); any other URL
/// loses its `user:pass@` and the values of credential-like query (and
/// fragment) parameters. The scratch `.tool.txt` keeps the raw URL: it lives
/// in the temp directory, never in the repository.
fn log_url(root: &Path, url: &str) -> String {
    if starts_with_ci(url, "file://") {
        return log_file_url(root, url);
    }
    let (base, fragment) = match url.split_once('#') {
        Some((b, f)) => (b, Some(f)),
        None => (url, None),
    };
    let (head, query) = match base.split_once('?') {
        Some((h, q)) => (h, Some(q)),
        None => (base, None),
    };
    let mut out = match head.find("://") {
        Some(i) => {
            let (scheme, rest) = head.split_at(i + 3);
            let host_end = rest.find('/').unwrap_or(rest.len());
            match rest[..host_end].rfind('@') {
                Some(at) => format!("{scheme}{}", &rest[at + 1..]),
                None => head.to_string(),
            }
        }
        None => head.to_string(),
    };
    if let Some(q) = query {
        out.push('?');
        out.push_str(&redact_params(q));
    }
    if let Some(f) = fragment {
        out.push('#');
        out.push_str(&redact_params(f));
    }
    out
}

/// An absolute local path in Windows or POSIX form, whatever the host.
fn looks_absolute(s: &str) -> bool {
    let b = s.as_bytes();
    s.starts_with(['/', '\\'])
        || (b.len() >= 3
            && b[0].is_ascii_alphabetic()
            && b[1] == b':'
            && matches!(b[2], b'/' | b'\\'))
}

/// A `--chrome-arg` value as the log may hold it: a `file://` URL anywhere in
/// it goes through `log_url`; `--key=value` loses its value when the key looks
/// like a credential, and otherwise has a URL value cleaned and an absolute
/// path value cut to its `LogPath` (a `--user-data-dir` would name the user).
fn log_chrome_arg(root: &Path, arg: &str) -> String {
    if let Some(i) = arg.to_ascii_lowercase().find("file://") {
        return format!("{}{}", &arg[..i], log_url(root, &arg[i..]));
    }
    let clean = |v: &str| {
        if starts_with_ci(v, "http://") || starts_with_ci(v, "https://") {
            log_url(root, v)
        } else if looks_absolute(v) {
            LogPath::of(root, Path::new(v)).path
        } else {
            v.to_string()
        }
    };
    // A bare URL first: its own query holds the `=`s, not a `key=`.
    if starts_with_ci(arg, "http://") || starts_with_ci(arg, "https://") {
        return log_url(root, arg);
    }
    match arg.split_once('=') {
        Some((k, _)) if secret_key(k) => format!("{k}={REDACTED}"),
        Some((k, v)) => format!("{k}={}", clean(v)),
        None => clean(arg),
    }
}

/// A log value with every string under a `url` key passed through `log_url`
/// and every string under `extra` (the `--chrome-arg`s) through
/// `log_chrome_arg`, at any depth (a stack's per-part URLs sit in `parts`).
fn scrub(root: &Path, key: &str, v: Value) -> Value {
    match v {
        Value::String(s) if key == "url" => Value::String(log_url(root, &s)),
        Value::String(s) if key == "extra" => Value::String(log_chrome_arg(root, &s)),
        Value::Array(a) => Value::Array(a.into_iter().map(|x| scrub(root, key, x)).collect()),
        Value::Object(m) => Value::Object(
            m.into_iter()
                .map(|(k, x)| {
                    let x = scrub(root, &k, x);
                    (k, x)
                })
                .collect(),
        ),
        other => other,
    }
}

struct Ctx {
    issue: Issue,
    group: String,
    /// `<temp>/barking-shot/<YYMM><MMDD>-<group>`: the year is in the key so
    /// that the same day a year later does not pick up old renders.
    scratch: PathBuf,
}

struct Resolved {
    png: PathBuf,
    /// Scratch name, or the file stem for `--from`.
    name: String,
    /// For the log: the scratch name, or the `--from` file as `LogPath`.
    from: LogPath,
    /// The scratch image's `tool.txt`, if any.
    prov: Option<Prov>,
}

impl Resolved {
    /// `from` (plus `outside_repo` and the scratch image's PDF / page) for a
    /// log record.
    fn source_fields(&self) -> Vec<(&'static str, Value)> {
        let mut out = vec![("from", json!(self.from.path))];
        out.extend(self.origin_fields());
        out
    }

    /// `outside_repo` and the scratch image's PDF / page, without `from`.
    fn origin_fields(&self) -> Vec<(&'static str, Value)> {
        let mut out = Vec::new();
        if self.from.outside {
            out.push(("outside_repo", json!(true)));
        }
        if let Some(p) = &self.prov {
            out.extend(p.fields());
        }
        out
    }

    /// The `--url` given, else the page the scratch image was rendered from.
    fn url(&self, given: &str) -> String {
        if !given.is_empty() {
            return given.to_string();
        }
        self.prov
            .as_ref()
            .and_then(|p| p.url.clone())
            .unwrap_or_default()
    }

    fn tool(&self) -> Option<&str> {
        self.prov.as_ref().map(|p| p.tool.as_str())
    }
}

impl Ctx {
    fn new(issue: Issue, group: &str) -> Ctx {
        let scratch = std::env::temp_dir()
            .join("barking-shot")
            .join(format!("{}{}-{group}", issue.yymm, issue.mmdd));
        Ctx {
            issue,
            group: group.to_string(),
            scratch,
        }
    }

    fn scratch_png(&self, name: &str) -> PathBuf {
        self.scratch.join(format!("{name}.png"))
    }

    fn log_path(&self, p: &Path) -> LogPath {
        LogPath::of(&self.issue.root, p)
    }

    fn resolve(&self, src: &Source) -> Result<Resolved, String> {
        match src {
            Source::Name(n) => {
                let png = self.scratch_png(n);
                if !png.is_file() {
                    return Err(format!(
                        "找不到中间图 {}：先对「{n}」运行 render 或 pdf（或改用 --from <png>）",
                        png.display()
                    ));
                }
                let prov = read_prov(&png);
                Ok(Resolved {
                    png,
                    name: n.clone(),
                    from: LogPath {
                        path: n.clone(),
                        outside: false,
                    },
                    prov,
                })
            }
            Source::File(p) => {
                if !p.is_file() {
                    return Err(format!("{}: 不是文件", p.display()));
                }
                let name = p
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string();
                Ok(Resolved {
                    png: p.clone(),
                    name,
                    from: self.log_path(p),
                    prov: read_prov(p),
                })
            }
        }
    }

    /// Fails early, before any slow work, if the output name is taken.
    fn check_free(&self, out: &str) -> Result<(), String> {
        let path = self.issue.images().join(out);
        if path.exists() {
            Err(exists_msg(&path))
        } else {
            Ok(())
        }
    }

    /// Writes `images/<out>` without ever replacing a file: `create_new` makes
    /// the refusal atomic, and a failed write leaves nothing half-written.
    fn emit(&self, out: &str, img: &Img) -> Result<PathBuf, String> {
        let dir = self.issue.images();
        fs::create_dir_all(&dir).map_err(io_err(&dir))?;
        let path = dir.join(out);
        let bytes = img.encode()?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    exists_msg(&path)
                } else {
                    format!("{}: {e}", path.display())
                }
            })?;
        if let Err(e) = file.write_all(&bytes) {
            drop(file);
            let _ = fs::remove_file(&path);
            return Err(format!("{}: {e}", path.display()));
        }
        Ok(path)
    }

    /// Appends one line to `sources/<group>-shots-log.jsonl`, stamped with the
    /// Beijing time first. Every URL and Chrome argument is cleaned here, in
    /// one place, so no action (nor a URL inherited from a scratch image) can
    /// write a credential or a local path into the committed log.
    fn log(&self, fields: Vec<(&str, Value)>) -> Result<(), String> {
        let root = &self.issue.root;
        let mut fields: Vec<(&str, Value)> = fields
            .into_iter()
            .map(|(k, v)| (k, scrub(root, k, v)))
            .collect();
        fields.insert(0, ("time_bj", Value::from(repo::beijing_now())));
        let dir = self.issue.sources();
        fs::create_dir_all(&dir).map_err(io_err(&dir))?;
        let path = dir.join(format!("{}-shots-log.jsonl", self.group));
        // A log whose last line lost its newline must not swallow ours.
        let unterminated = fs::read(&path).is_ok_and(|b| b.last().is_some_and(|&c| c != b'\n'));
        let mut line = String::new();
        if unterminated {
            line.push('\n');
        }
        line.push_str(&jsonl(&fields));
        line.push('\n');
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .and_then(|mut f| f.write_all(line.as_bytes()))
            .map_err(|e| {
                format!(
                    "{}: {e}（图片已写出，请手工补记这一条日志）",
                    path.display()
                )
            })
    }
}

// ---------------------------------------------------------------------------
// Actions

fn execute(ctx: &Ctx, action: &Action) -> Result<(), String> {
    match action {
        Action::Render(r) => do_render(ctx, r),
        Action::Pdf(p) => do_pdf(ctx, p),
        Action::Lines { src, lang } => do_lines(ctx, src, lang),
        Action::Crop(c) => do_crop(ctx, c),
        Action::Cropbox {
            src,
            out,
            rect,
            url,
        } => do_cropbox(ctx, src, out, *rect, url),
        Action::Stack {
            out,
            parts,
            gap,
            bg,
            url,
        } => do_stack(ctx, out, parts, *gap, bg, url),
    }
}

/// Replaces scratch image `out` (and its sidecars) with the finished file
/// `made`, then writes the new sidecars. Called only once the new image and
/// everything derived from it exist, so a failed re-render or re-extraction
/// leaves the previous scratch image intact; if this last step itself fails,
/// the half-replaced image is removed rather than left without its sidecars.
fn install_scratch(
    made: &Path,
    out: &Path,
    prov: &Prov,
    lines: Option<&[Line]>,
) -> Result<(), String> {
    let _ = fs::remove_file(out);
    clear_sidecars(out);
    let tool_file = out.with_extension(SIDECAR_TOOL);
    let side = out.with_extension(SIDECAR_LINES);
    let done = fs::rename(made, out)
        .map_err(io_err(out))
        .and_then(|()| fs::write(&tool_file, prov.text()).map_err(io_err(&tool_file)))
        .and_then(|()| match lines {
            Some(l) => fs::write(&side, lines_tsv(l)).map_err(io_err(&side)),
            None => Ok(()),
        });
    if done.is_err() {
        let _ = fs::remove_file(out);
        clear_sidecars(out);
    }
    done
}

fn do_render(ctx: &Ctx, r: &Render) -> Result<(), String> {
    let chrome = card::find_chrome()?;
    fs::create_dir_all(&ctx.scratch).map_err(io_err(&ctx.scratch))?;
    let out = ctx.scratch_png(&r.name);
    // Chrome writes beside the old image under a temporary name (the `.` keeps
    // it out of the `[a-z0-9-]` name space); the old image is replaced only
    // once the new one exists.
    let fresh = ctx
        .scratch
        .join(format!("{}.rendering-{}.png", r.name, std::process::id()));
    let _ = fs::remove_file(&fresh);
    let profile = ctx.scratch.join(format!("profile-{}", std::process::id()));
    let chrome_log = ctx.scratch.join(format!("{}-chrome.log", r.name));
    let ran = run_chrome(&chrome, &chrome_args(&profile, r, &fresh), &chrome_log);
    let _ = fs::remove_dir_all(&profile);
    let size = ran.and_then(|()| {
        if fresh.is_file() {
            card::png_size(&fresh)
        } else {
            Err(format!(
                "Chrome 正常退出但没有写出 {}（日志 {}）",
                fresh.display(),
                chrome_log.display()
            ))
        }
    });
    let (w, h) = match size {
        Ok(s) => s,
        Err(e) => {
            let _ = fs::remove_file(&fresh);
            return Err(e);
        }
    };
    let tool = tool_chrome(r);
    let prov = Prov {
        tool: tool.clone(),
        url: Some(r.url.clone()),
        ..Prov::default()
    };
    install_scratch(&fresh, &out, &prov, None)?;

    println!(
        "{}  {w}×{h} px（{} CSS px 宽，DPR {}）",
        out.display(),
        r.width,
        r.scale
    );
    let expected = (
        (f64::from(r.width) * r.scale).round() as u32,
        (f64::from(r.height) * r.scale).round() as u32,
    );
    if (w, h) != expected {
        println!(
            "注意：预期 {}×{} px（{}×{} CSS px × DPR {}），实际 {w}×{h}",
            expected.0, expected.1, r.width, r.height, r.scale
        );
    }
    println!("提示：打开这张图确认页面没被截断、没有大片留白；不合适就改 --height 重渲");
    ctx.log(vec![
        ("action", json!("render")),
        ("url", json!(r.url)),
        ("name", json!(r.name)),
        ("tool", json!(tool)),
        ("size", json!([w, h])),
        ("width", json!(r.width)),
        ("height", json!(r.height)),
        ("scale", json!(r.scale)),
        ("extra", json!(r.chrome_args)),
    ])
}

/// The one `pg-*.png` pdftoppm wrote into `dir` (its numeric suffix depends on
/// the page count of the document).
fn find_page_png(dir: &Path) -> Result<PathBuf, String> {
    let mut found: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(io_err(dir))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("pg-") && n.ends_with(".png"))
        })
        .collect();
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err("pdftoppm 没有生成页面图（页码超出文档范围？）".into()),
        n => Err(format!("pdftoppm 生成了 {n} 个页面图，预期 1 个")),
    }
}

/// A rendered PDF page still in the work directory, with its text lines.
struct PageRender {
    png: PathBuf,
    size: (u32, u32),
    lines: Vec<Line>,
}

/// pdftotext first (it fails fast when only xpdf's is found), then pdftoppm
/// into `work`; nothing outside `work` is touched, so the caller can discard a
/// failure by removing it.
fn render_page(p: &PdfPage, work: &Path) -> Result<PageRender, String> {
    let dirs = poppler_dirs();
    let (page, dpi) = (p.page.to_string(), p.dpi.to_string());
    let txt = pick_tool(&dirs, "pdftotext");
    let text = run_capture(
        Command::new(&txt)
            .args(["-enc", "UTF-8", "-bbox-layout", "-f", &page, "-l", &page])
            .arg(&p.pdf)
            .arg("-"),
        "pdftotext",
    )
    .map_err(|e| {
        format!("{e}\n（若是不认识 -bbox-layout：需要 poppler 的 pdftotext，PATH 上的 xpdf 旧版没有这个选项，用 BARKING_POPPLER 指向 poppler 的 bin 目录）")
    })?;
    let lines = parse_bbox_lines(&String::from_utf8_lossy(&text.stdout), p.dpi);

    let ppm = pick_tool(&dirs, "pdftoppm");
    run_capture(
        Command::new(&ppm)
            .args(["-r", &dpi, "-f", &page, "-l", &page, "-png"])
            .arg(&p.pdf)
            .arg(work.join("pg")),
        "pdftoppm",
    )?;
    let png = find_page_png(work)?;
    let size = card::png_size(&png)?;
    Ok(PageRender { png, size, lines })
}

fn do_pdf(ctx: &Ctx, p: &PdfPage) -> Result<(), String> {
    if !p.pdf.is_file() {
        return Err(format!("{}: 不是文件", p.pdf.display()));
    }
    fs::create_dir_all(&ctx.scratch).map_err(io_err(&ctx.scratch))?;
    let work = ctx.scratch.join(format!("pdf-{}", p.name));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).map_err(io_err(&work))?;
    let out = ctx.scratch_png(&p.name);
    let pdf = ctx.log_path(&p.pdf);
    let tool = tool_pdf(p.dpi);
    // Both poppler steps must succeed before the old scratch image is
    // replaced; the work directory goes either way.
    let made = render_page(p, &work).and_then(|page| {
        let prov = Prov {
            tool: tool.clone(),
            pdf: Some(pdf.path.clone()),
            page: Some(p.page),
            ..Prov::default()
        };
        // No text layer (a scanned PDF): leave no sidecar, so `lines` / `crop`
        // fall back to OCR instead of searching an empty list.
        let lines = (!page.lines.is_empty()).then_some(&page.lines[..]);
        install_scratch(&page.png, &out, &prov, lines)?;
        Ok(page)
    });
    let _ = fs::remove_dir_all(&work);
    let PageRender {
        size: (w, h),
        lines,
        ..
    } = made?;

    if lines.is_empty() {
        println!("注意：这一页没有文字层，lines / crop 将改用 Tesseract OCR");
    } else if lines.iter().any(|l| l.y1 > h + 2) {
        println!("注意：有文字行超出渲染图高度 {h}（页面带旋转或裁切框？），锚点坐标可能偏移");
    }
    println!(
        "{}  {w}×{h} px（第 {} 页，{} dpi，{} 行文字）",
        out.display(),
        p.page,
        p.dpi,
        lines.len()
    );
    let mut rec = vec![
        ("action", json!("pdf")),
        ("pdf", json!(pdf.path)),
        ("page", json!(p.page)),
        ("name", json!(p.name)),
        ("tool", json!(tool)),
        ("dpi", json!(p.dpi)),
        ("size", json!([w, h])),
        ("lines", json!(lines.len())),
    ];
    if pdf.outside {
        rec.push(("outside_repo", json!(true)));
    }
    ctx.log(rec)
}
enum LineSource {
    /// Exact lines from the PDF text layer.
    Sidecar,
    Ocr,
}

/// Lines for `png`: its `.lines.tsv` sidecar when there is one, else a
/// Tesseract pass.
fn lines_for(
    ctx: &Ctx,
    png: &Path,
    width: u32,
    lang: &str,
) -> Result<(Vec<Line>, LineSource), String> {
    let side = png.with_extension(SIDECAR_LINES);
    match fs::read_to_string(&side) {
        Ok(text) => {
            let lines = parse_lines_tsv(&text).map_err(|e| format!("{}: {e}", side.display()))?;
            return Ok((lines, LineSource::Sidecar));
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("{}: {e}", side.display())),
    }
    fs::create_dir_all(&ctx.scratch).map_err(io_err(&ctx.scratch))?;
    let words = card::run_ocr(png, width, lang, &ctx.scratch)?;
    Ok((lines_from_words(&words), LineSource::Ocr))
}

fn do_lines(ctx: &Ctx, src: &Source, lang: &str) -> Result<(), String> {
    let r = ctx.resolve(src)?;
    let (w, _) = card::png_size(&r.png)?;
    let (lines, how) = lines_for(ctx, &r.png, w, lang)?;
    match how {
        LineSource::Sidecar => eprintln!(
            "（行文本来自 {}）",
            r.png.with_extension(SIDECAR_LINES).display()
        ),
        LineSource::Ocr => eprintln!(
            "（Tesseract OCR，lang={lang}；坐标是 {} 的像素）",
            r.png.display()
        ),
    }
    for l in &lines {
        println!("{:>6} {:>6}  {}", l.y0, l.y1, l.text);
    }
    Ok(())
}

fn do_crop(ctx: &Ctx, c: &AnchorCrop) -> Result<(), String> {
    let r = ctx.resolve(&c.src)?;
    ctx.check_free(&c.out)?;
    let img = Img::load(&r.png)?;
    let (lines, how) = lines_for(ctx, &r.png, img.w, &c.lang)?;
    let span = anchor_box(
        &lines,
        &c.start,
        &c.end,
        (c.pad_top, c.pad_bottom),
        (img.w, img.h),
    )?;
    let cropped = img.crop(span.rect)?;
    let tool = match how {
        LineSource::Sidecar => match r.tool() {
            Some(p) => format!("{p} crop"),
            None => "text-layer line crop".to_string(),
        },
        LineSource::Ocr => tool_label(r.tool(), "Tesseract anchor crop"),
    };
    let path = ctx.emit(&c.out, &cropped)?;

    let (s, e) = (&lines[span.start], &lines[span.end]);
    println!(
        "{}  {}×{} px  来自 {}（{}×{}），框 {}",
        path.display(),
        cropped.w,
        cropped.h,
        r.name,
        img.w,
        img.h,
        span.rect.json()
    );
    println!(
        "起始锚点 y {}–{}「{}」；结束锚点 y {}–{}「{}」",
        s.y0,
        s.y1,
        clip(&s.text, 40),
        e.y0,
        e.y1,
        clip(&e.text, 40)
    );
    for w in cut_warnings(&lines, span.rect) {
        println!("{w}");
    }
    let mut rec = vec![
        ("action", json!("crop")),
        ("url", json!(r.url(&c.url))),
        ("name", json!(r.name)),
        ("file", json!(c.out)),
        ("box", span.rect.json()),
        ("full_size", json!([img.w, img.h])),
        ("anchors", json!([c.start, c.end])),
        ("pads", json!([c.pad_top, c.pad_bottom])),
        ("tool", json!(tool)),
    ];
    rec.extend(r.source_fields());
    ctx.log(rec)
}

fn do_cropbox(ctx: &Ctx, src: &Source, out: &str, rect: Rect, url: &str) -> Result<(), String> {
    let r = ctx.resolve(src)?;
    ctx.check_free(out)?;
    let img = Img::load(&r.png)?;
    let cropped = img.crop(rect)?;
    let tool = tool_label(r.tool(), "pixel-box crop");
    let path = ctx.emit(out, &cropped)?;
    println!(
        "{}  {}×{} px  来自 {}（{}×{}），框 {}",
        path.display(),
        cropped.w,
        cropped.h,
        r.name,
        img.w,
        img.h,
        rect.json()
    );
    let mut rec = vec![
        ("action", json!("cropbox")),
        ("url", json!(r.url(url))),
        ("name", json!(r.name)),
        ("file", json!(out)),
        ("box", rect.json()),
        ("full_size", json!([img.w, img.h])),
        ("tool", json!(tool)),
    ];
    rec.extend(r.source_fields());
    ctx.log(rec)
}

fn do_stack(
    ctx: &Ctx,
    out: &str,
    parts: &[Part],
    gap: u32,
    bg: &Bg,
    url: &str,
) -> Result<(), String> {
    ctx.check_free(out)?;
    // The same source often supplies several parts; decode it once.
    let mut sources: HashMap<PathBuf, Img> = HashMap::new();
    let mut crops = Vec::new();
    let mut records = Vec::new();
    let mut provenance: Vec<String> = Vec::new();
    let mut first_src: Option<PathBuf> = None;
    for (i, part) in parts.iter().enumerate() {
        let r = ctx.resolve(&part.src)?;
        if !sources.contains_key(&r.png) {
            sources.insert(r.png.clone(), Img::load(&r.png)?);
        }
        let img = &sources[&r.png];
        crops.push(
            img.crop(part.rect)
                .map_err(|e| format!("--part {}（{}）：{e}", i + 1, r.png.display()))?,
        );
        let mut rec = serde_json::Map::new();
        rec.insert("src".into(), json!(r.from.path));
        rec.insert("box".into(), part.rect.json());
        rec.insert("full_size".into(), json!([img.w, img.h]));
        let part_url = r.url("");
        if !part_url.is_empty() {
            rec.insert("url".into(), json!(part_url));
        }
        for (k, v) in r.origin_fields() {
            rec.insert(k.into(), v);
        }
        records.push(Value::Object(rec));
        if let Some(t) = r.tool()
            && !provenance.iter().any(|p| p == t)
        {
            provenance.push(t.to_string());
        }
        first_src.get_or_insert(r.png);
    }
    let bg_rgba = match bg {
        Bg::Rgb([r, g, b]) => [*r, *g, *b, 255],
        Bg::Auto => {
            let first = first_src.as_ref().and_then(|p| sources.get(p));
            auto_bg(first.ok_or("没有来源图")?)?
        }
    };
    let phys_differ = crops.iter().any(|c| c.meta.phys != crops[0].meta.phys);
    let canvas = stack(crops, gap, bg_rgba)?;
    let tool = if provenance.is_empty() {
        "vertical stack".to_string()
    } else {
        tool_label(Some(&provenance.join(" | ")), "vertical stack")
    };
    let path = ctx.emit(out, &canvas)?;
    println!(
        "{}  {}×{} px  {} 段，间隔 {gap} px，背景 {}{}",
        path.display(),
        canvas.w,
        canvas.h,
        parts.len(),
        bg_hex(bg_rgba),
        if *bg == Bg::Auto { "（auto）" } else { "" }
    );
    if phys_differ {
        println!("注意：各段的 pHYs（物理分辨率）不同，输出不带 pHYs；像素与色彩块不受影响");
    }
    ctx.log(vec![
        ("action", json!("stack")),
        ("url", json!(url)),
        ("file", json!(out)),
        ("parts", Value::Array(records)),
        ("gap", json!(gap)),
        ("bg", json!(bg_hex(bg_rgba))),
        ("bg_auto", json!(*bg == Bg::Auto)),
        ("size", json!([canvas.w, canvas.h])),
        ("tool", json!(tool)),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    fn parse_ok(v: &[&str]) -> Parsed {
        parse(&s(v)).unwrap_or_else(|e| panic!("{v:?}: {e}"))
    }

    fn parse_err(v: &[&str]) -> String {
        parse(&s(v)).expect_err(&format!("{v:?} should fail"))
    }

    /// Deterministic pixels: every sample differs from its neighbours.
    fn test_img(w: u32, h: u32, ct: ColorType) -> Img {
        let n = w as usize * h as usize * ct.samples();
        Img {
            w,
            h,
            ct,
            data: (0..n).map(|i| (i * 7 % 251) as u8).collect(),
            key: None,
            meta: Meta::default(),
        }
    }

    fn plain(w: u32, h: u32, ct: ColorType, data: Vec<u8>) -> Img {
        Img {
            w,
            h,
            ct,
            data,
            key: None,
            meta: Meta::default(),
        }
    }

    fn sample(img: &Img, x: u32, y: u32) -> &[u8] {
        let bpp = img.ct.samples();
        let i = (y as usize * img.w as usize + x as usize) * bpp;
        &img.data[i..i + bpp]
    }

    fn raw_png(
        (w, h): (u32, u32),
        color: ColorType,
        depth: BitDepth,
        palette: Option<Vec<u8>>,
        trns: Option<Vec<u8>>,
        data: &[u8],
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut enc = png::Encoder::new(&mut buf, w, h);
        enc.set_color(color);
        enc.set_depth(depth);
        if let Some(p) = palette {
            enc.set_palette(p);
        }
        if let Some(t) = trns {
            enc.set_trns(t);
        }
        let mut writer = enc.write_header().unwrap();
        writer.write_image_data(data).unwrap();
        writer.finish().unwrap();
        buf
    }

    const ALL: [ColorType; 4] = [
        ColorType::Grayscale,
        ColorType::GrayscaleAlpha,
        ColorType::Rgb,
        ColorType::Rgba,
    ];

    // --- command line -----------------------------------------------------

    #[test]
    fn render_defaults_and_options() {
        let p = parse_ok(&["docs/2610/1009", "a", "render", "https://example.com", "ex"]);
        assert_eq!(p.dir, "docs/2610/1009");
        assert_eq!(p.group, "a");
        assert_eq!(
            p.action,
            Action::Render(Render {
                url: "https://example.com".into(),
                name: "ex".into(),
                width: 700,
                height: 1600,
                scale: 2.0,
                chrome_args: vec![],
            })
        );
        let p = parse_ok(&[
            "d",
            "a",
            "render",
            "http://x/y?a=b",
            "p1",
            "--width",
            "1000",
            "--height",
            "900",
            "--scale",
            "1.5",
            "--chrome-arg",
            "--user-agent=x",
            "--chrome-arg",
            "--no-sandbox",
        ]);
        let Action::Render(r) = p.action else {
            panic!("not render")
        };
        assert_eq!((r.width, r.height, r.scale), (1000, 900, 1.5));
        assert_eq!(r.chrome_args, ["--user-agent=x", "--no-sandbox"]);
    }

    #[test]
    fn render_rejects_bad_input() {
        for (v, needle) in [
            (vec!["d", "a", "render", "https://x", "Ex"], "name"),
            (vec!["d", "a", "render", "example.com", "ex"], "URL"),
            (
                vec!["d", "a", "render", "https://x", "ex", "--scale", "0"],
                "--scale",
            ),
            (
                vec!["d", "a", "render", "https://x", "ex", "--scale", "nan"],
                "--scale",
            ),
            (
                vec!["d", "a", "render", "https://x", "ex", "--width", "0"],
                "--width",
            ),
            (
                vec!["d", "a", "render", "https://x", "ex", "--width", "abc"],
                "--width",
            ),
            (
                vec![
                    "d",
                    "a",
                    "render",
                    "https://x",
                    "ex",
                    "--width",
                    "1",
                    "--width",
                    "2",
                ],
                "只能给一次",
            ),
            (
                vec!["d", "a", "render", "https://x", "ex", "--chrome-arg", "foo"],
                "--chrome-arg",
            ),
            (
                vec!["d", "a", "render", "https://x", "ex", "--bogus", "1"],
                "--bogus",
            ),
            (vec!["d", "a", "render", "https://x"], "缺少参数"),
            (vec!["d", "a", "render", "https://x", "ex", "extra"], "多余"),
            (vec!["d", "A", "render", "https://x", "ex"], "组前缀"),
            (vec!["d", "a", "nope"], "不认识的动作"),
            (
                vec!["d", "a", "render", "https://x", "ex", "--width"],
                "缺少参数",
            ),
        ] {
            assert!(parse_err(&v).contains(needle), "{v:?} -> {}", parse_err(&v));
        }
        assert!(parse(&s(&["d", "a"])).is_err());
    }

    #[test]
    fn pdf_and_lines_parse() {
        let p = parse_ok(&["d", "a", "pdf", "x.pdf", "3", "pg", "--dpi", "200"]);
        assert_eq!(
            p.action,
            Action::Pdf(PdfPage {
                pdf: PathBuf::from("x.pdf"),
                page: 3,
                name: "pg".into(),
                dpi: 200
            })
        );
        assert!(parse_err(&["d", "a", "pdf", "x.pdf", "0", "pg"]).contains("从 1 起"));
        assert!(parse_err(&["d", "a", "pdf", "x.pdf", "1", "pg", "--dpi", "10"]).contains("--dpi"));
        assert!(parse_err(&["d", "a", "pdf", "x.pdf", "one", "pg"]).contains("页码"));

        assert_eq!(
            parse_ok(&["d", "a", "lines", "pg"]).action,
            Action::Lines {
                src: Source::Name("pg".into()),
                lang: "eng".into()
            }
        );
        assert_eq!(
            parse_ok(&[
                "d",
                "a",
                "lines",
                "--from",
                "E:\\x\\y.png",
                "--lang",
                "chi_sim"
            ])
            .action,
            Action::Lines {
                src: Source::File(PathBuf::from("E:\\x\\y.png")),
                lang: "chi_sim".into()
            }
        );
        assert!(parse_err(&["d", "a", "lines"]).contains("缺少参数"));
        assert!(parse_err(&["d", "a", "lines", "BAD"]).contains("name"));
    }

    #[test]
    fn crop_and_cropbox_parse() {
        let p = parse_ok(&[
            "d",
            "a",
            "crop",
            "pg",
            "41-x-raw.png",
            "--start",
            "First line",
            "--end",
            "Last",
            "--pad-top",
            "4",
            "--url",
            "https://x.org/p",
        ]);
        assert_eq!(
            p.action,
            Action::Crop(AnchorCrop {
                src: Source::Name("pg".into()),
                out: "41-x-raw.png".into(),
                start: "First line".into(),
                end: "Last".into(),
                pad_top: 4,
                pad_bottom: 24,
                url: "https://x.org/p".into(),
                lang: "eng".into(),
            })
        );
        assert!(parse_err(&["d", "a", "crop", "pg", "o.png", "--end", "x"]).contains("--start"));
        assert!(
            parse_err(&[
                "d",
                "a",
                "crop",
                "pg",
                "sub/o.png",
                "--start",
                "a",
                "--end",
                "b"
            ])
            .contains("输出名")
        );
        assert!(
            parse_err(&[
                "d", "a", "crop", "pg", "o.jpg", "--start", "a", "--end", "b"
            ])
            .contains("输出名")
        );
        assert!(
            parse_err(&[
                "d", "a", "crop", "--from", "a.png", "o.png", "extra", "--start", "a", "--end", "b"
            ])
            .contains("多余")
        );

        let p = parse_ok(&[
            "d", "a", "cropbox", "--from", "i.png", "o.png", "0", "915", "1400", "1600",
        ]);
        assert_eq!(
            p.action,
            Action::Cropbox {
                src: Source::File(PathBuf::from("i.png")),
                out: "o.png".into(),
                rect: Rect {
                    x0: 0,
                    y0: 915,
                    x1: 1400,
                    y1: 1600
                },
                url: String::new(),
            }
        );
        assert!(parse_err(&["d", "a", "cropbox", "pg", "o.png", "0", "1", "2"]).contains("<y1>"));
        assert!(
            parse_err(&["d", "a", "cropbox", "pg", "o.png", "0", "-1", "2", "3"]).contains("<y0>")
        );
    }

    #[test]
    fn stack_parse_and_parts() {
        let p = parse_ok(&[
            "d",
            "a",
            "stack",
            "42-x-raw.png",
            "--part",
            "E:\\imgs\\05.png:0,1092,1400,1218",
            "--part",
            "pg:1,2,3,4",
            "--gap",
            "10",
            "--bg",
            "#AaBb01",
            "--url",
            "https://u",
        ]);
        let Action::Stack {
            out,
            parts,
            gap,
            bg,
            url,
        } = p.action
        else {
            panic!("not stack")
        };
        assert_eq!(out, "42-x-raw.png");
        assert_eq!(
            parts,
            vec![
                Part {
                    src: Source::File(PathBuf::from("E:\\imgs\\05.png")),
                    rect: Rect {
                        x0: 0,
                        y0: 1092,
                        x1: 1400,
                        y1: 1218
                    }
                },
                Part {
                    src: Source::Name("pg".into()),
                    rect: Rect {
                        x0: 1,
                        y0: 2,
                        x1: 3,
                        y1: 4
                    }
                },
            ]
        );
        assert_eq!((gap, url.as_str()), (10, "https://u"));
        assert_eq!(bg, Bg::Rgb([0xaa, 0xbb, 0x01]));

        let p = parse_ok(&[
            "d",
            "a",
            "stack",
            "o.png",
            "--part",
            "docs/2610/1009/images/02.png:0,0,5,5",
        ]);
        let Action::Stack { parts, gap, bg, .. } = p.action else {
            panic!("not stack")
        };
        assert_eq!(
            parts[0].src,
            Source::File(PathBuf::from("docs/2610/1009/images/02.png"))
        );
        assert_eq!((gap, bg), (24, Bg::Auto));

        for bad in [
            "noc.png",
            "a.png:1,2,3",
            "a.png:1,2,3,x",
            "a.png:1,2,3,4,5",
            ":1,2,3,4",
        ] {
            assert!(parse_part(bad).is_err(), "{bad}");
        }
        assert!(parse_err(&["d", "a", "stack", "o.png"]).contains("--part"));
        assert!(
            parse_err(&[
                "d",
                "a",
                "stack",
                "o.png",
                "--part",
                "a.png:0,0,1,1",
                "--bg",
                "red"
            ])
            .contains("--bg")
        );
        assert!(parse_hex("#12345").is_err());
        assert!(parse_hex("123456").is_err());
        assert!(parse_hex("#12345g").is_err());
        assert_eq!(parse_hex("#ff0080"), Ok([255, 0, 128]));
    }

    #[test]
    fn output_names() {
        for ok in ["a.png", "41-report-tip-raw.png", "00_x.1.png"] {
            assert!(check_out(ok).is_ok(), "{ok}");
        }
        for bad in [
            ".png",
            "a",
            "a.jpg",
            "a/b.png",
            "a\\b.png",
            "a:b.png",
            "a b.png",
            "图.png",
            "..\\x.png",
        ] {
            assert!(check_out(bad).is_err(), "{bad}");
        }
        assert!(is_name("a-1") && !is_name("") && !is_name("A") && !is_name("a_b"));
    }

    // --- pixels -----------------------------------------------------------

    #[test]
    fn crop_is_pixel_exact_for_every_layout() {
        for ct in ALL {
            let img = test_img(9, 7, ct);
            let r = Rect {
                x0: 2,
                y0: 1,
                x1: 7,
                y1: 5,
            };
            let c = img.crop(r).unwrap();
            assert_eq!((c.w, c.h, c.ct), (5, 4, ct));
            for y in r.y0..r.y1 {
                for x in r.x0..r.x1 {
                    assert_eq!(
                        sample(&c, x - r.x0, y - r.y0),
                        sample(&img, x, y),
                        "{ct:?} ({x},{y})"
                    );
                }
            }
            // The full box is the identity.
            let whole = img
                .crop(Rect {
                    x0: 0,
                    y0: 0,
                    x1: 9,
                    y1: 7,
                })
                .unwrap();
            assert_eq!(whole, img);
        }
    }

    #[test]
    fn crop_rejects_bad_boxes() {
        let img = test_img(9, 7, ColorType::Rgb);
        for (x0, y0, x1, y1) in [
            (0, 0, 10, 7),
            (0, 0, 9, 8),
            (3, 0, 3, 7),
            (4, 0, 3, 7),
            (0, 5, 9, 5),
        ] {
            let e = img.crop(Rect { x0, y0, x1, y1 }).unwrap_err();
            assert!(e.contains("裁切框"), "{e}");
        }
    }

    #[test]
    fn encode_decode_round_trip() {
        for ct in ALL {
            let img = test_img(11, 6, ct);
            let back = Img::decode(&img.encode().unwrap()).unwrap();
            assert_eq!(back, img, "{ct:?}");
        }
    }

    #[test]
    fn decode_expands_but_refuses_16_bit() {
        // 1-bit gray: 8 pixels per row, packed.
        let bytes = raw_png(
            (8, 2),
            ColorType::Grayscale,
            BitDepth::One,
            None,
            None,
            &[0b1010_1010, 0b1111_0000],
        );
        let img = Img::decode(&bytes).unwrap();
        assert_eq!((img.ct, img.w, img.h), (ColorType::Grayscale, 8, 2));
        assert_eq!(
            img.data,
            [
                255, 0, 255, 0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 0, 0
            ]
        );

        let pal = Some(vec![255, 0, 0, 0, 255, 0]);
        let bytes = raw_png(
            (2, 1),
            ColorType::Indexed,
            BitDepth::Eight,
            pal.clone(),
            None,
            &[0, 1],
        );
        let img = Img::decode(&bytes).unwrap();
        assert_eq!(
            (img.ct, &img.data[..]),
            (ColorType::Rgb, &[255, 0, 0, 0, 255, 0][..])
        );

        let bytes = raw_png(
            (2, 1),
            ColorType::Indexed,
            BitDepth::Eight,
            pal,
            Some(vec![128]),
            &[0, 1],
        );
        let img = Img::decode(&bytes).unwrap();
        assert_eq!(
            (img.ct, &img.data[..]),
            (ColorType::Rgba, &[255, 0, 0, 128, 0, 255, 0, 255][..])
        );

        let bytes = raw_png(
            (2, 1),
            ColorType::Rgb,
            BitDepth::Sixteen,
            None,
            None,
            &[0; 12],
        );
        assert_eq!(Img::decode(&bytes).unwrap_err(), "不支持 16 位 PNG");
        assert!(Img::decode(b"not a png").unwrap_err().contains("PNG"));
    }

    // --- metadata, colour keys ----------------------------------------------

    /// Chunk names in file order, IDAT collapsed to one entry.
    fn chunk_names(png: &[u8]) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut i = 8;
        while i + 8 <= png.len() {
            let n = u32::from_be_bytes(png[i..i + 4].try_into().unwrap()) as usize;
            let name = String::from_utf8_lossy(&png[i + 4..i + 8]).into_owned();
            if name != "IDAT" || out.last().is_none_or(|l| l != "IDAT") {
                out.push(name);
            }
            i += 12 + n;
        }
        out
    }

    /// The data of chunk `name`, if present.
    fn chunk_data<'a>(png: &'a [u8], name: &str) -> Option<&'a [u8]> {
        let mut i = 8;
        while i + 8 <= png.len() {
            let n = u32::from_be_bytes(png[i..i + 4].try_into().unwrap()) as usize;
            if &png[i + 4..i + 8] == name.as_bytes() {
                return Some(&png[i + 8..i + 8 + n]);
            }
            i += 12 + n;
        }
        None
    }

    /// A small RGB PNG written by the png crate itself, with an ICC profile,
    /// gAMA, cHRM and pHYs, as a screenshot tool or browser might save it.
    fn tagged_png(w: u32, h: u32, icc: &[u8], srgb: bool) -> (Vec<u8>, Vec<u8>) {
        let data: Vec<u8> = (0..w as usize * h as usize * 3)
            .map(|i| (i * 13 % 256) as u8)
            .collect();
        let mut info = png::Info::with_size(w, h);
        info.color_type = ColorType::Rgb;
        info.bit_depth = BitDepth::Eight;
        info.icc_profile = Some(icc.to_vec().into());
        info.source_gamma = Some(ScaledFloat::from_scaled(55_555));
        info.source_chromaticities = Some(SourceChromaticities::new(
            (0.3127, 0.329),
            (0.68, 0.32),
            (0.265, 0.69),
            (0.15, 0.06),
        ));
        info.pixel_dims = Some(png::PixelDimensions {
            xppu: 5669,
            yppu: 5669,
            unit: Unit::Meter,
        });
        let mut buf = Vec::new();
        let mut writer = png::Encoder::with_info(&mut buf, info)
            .unwrap()
            .write_header()
            .unwrap();
        if srgb {
            writer.write_chunk(png::chunk::sRGB, &[1]).unwrap();
        }
        writer.write_image_data(&data).unwrap();
        writer.finish().unwrap();
        (buf, data)
    }

    #[test]
    fn crop_keeps_colour_chunks() {
        let icc: Vec<u8> = (0..300).map(|i| (i % 97) as u8).collect();
        let (src, data) = tagged_png(6, 5, &icc, false);
        assert!(chunk_names(&src).contains(&"iCCP".to_string()));
        let img = Img::decode(&src).unwrap();
        assert_eq!(img.meta.icc.as_deref(), Some(&icc[..]));
        assert_eq!(img.meta.gamma, Some(ScaledFloat::from_scaled(55_555)));
        assert!(img.meta.chrm.is_some() && img.meta.srgb.is_none());
        assert_eq!(img.meta.phys, Some((5669, 5669, Unit::Meter)));

        let r = Rect {
            x0: 1,
            y0: 2,
            x1: 5,
            y1: 4,
        };
        let out = img.crop(r).unwrap().encode().unwrap();
        let names = chunk_names(&out);
        for c in ["pHYs", "gAMA", "cHRM", "iCCP"] {
            assert!(names.contains(&c.to_string()), "{c} missing: {names:?}");
        }
        assert!(!names.contains(&"sRGB".to_string()));
        assert_eq!(chunk_data(&out, "gAMA"), chunk_data(&src, "gAMA"));
        assert_eq!(chunk_data(&out, "cHRM"), chunk_data(&src, "cHRM"));
        assert_eq!(chunk_data(&out, "pHYs"), chunk_data(&src, "pHYs"));
        let back = Img::decode(&out).unwrap();
        assert_eq!(back.meta, img.meta);
        // Pixels: exactly the box of the source rows.
        let mut want = Vec::new();
        for y in r.y0..r.y1 {
            let row = y as usize * 6 * 3;
            want.extend_from_slice(&data[row + 3..row + 15]);
        }
        assert_eq!(back.data, want);
    }

    #[test]
    fn srgb_and_iccp_both_survive() {
        // Non-conforming but real: some tools write both. The crate's own
        // encoder would drop the iCCP when sRGB is set.
        let (src, _) = tagged_png(3, 3, b"profile", true);
        let img = Img::decode(&src).unwrap();
        assert!(img.meta.srgb.is_some() && img.meta.icc.is_some());
        let out = img.encode().unwrap();
        let names = chunk_names(&out);
        assert!(
            ["sRGB", "iCCP", "gAMA", "cHRM"]
                .iter()
                .all(|c| names.contains(&c.to_string())),
            "{names:?}"
        );
        assert_eq!(chunk_data(&out, "sRGB"), Some(&[1u8][..]));
        assert_eq!(Img::decode(&out).unwrap().meta, img.meta);
        // An untagged image stays untagged.
        let plain_png = test_img(2, 2, ColorType::Rgb).encode().unwrap();
        assert_eq!(chunk_names(&plain_png), ["IHDR", "IDAT", "IEND"]);
    }

    #[test]
    fn colour_key_is_kept_on_crop() {
        // 3×1 RGB, the middle pixel is the key colour.
        let key = vec![0, 10, 0, 20, 0, 30];
        let src = raw_png(
            (3, 1),
            ColorType::Rgb,
            BitDepth::Eight,
            None,
            Some(key.clone()),
            &[1, 2, 3, 10, 20, 30, 4, 5, 6],
        );
        let img = Img::decode(&src).unwrap();
        assert_eq!(img.ct, ColorType::Rgb);
        assert_eq!(img.key.as_deref(), Some(&[10, 20, 30][..]));
        assert_eq!(img.data, [1, 2, 3, 10, 20, 30, 4, 5, 6]);
        assert_eq!(img.rgba_at(1, 0), Some([10, 20, 30, 0]));
        assert_eq!(img.rgba_at(0, 0), Some([1, 2, 3, 255]));
        let out = img
            .crop(Rect {
                x0: 1,
                y0: 0,
                x1: 3,
                y1: 1,
            })
            .unwrap()
            .encode()
            .unwrap();
        assert_eq!(out[25], 2, "IHDR colour type stays RGB");
        assert_eq!(chunk_data(&out, "tRNS"), Some(&key[..]));
        let back = Img::decode(&out).unwrap();
        assert_eq!(
            (back.ct, &back.data[..], back.key.as_deref()),
            (
                ColorType::Rgb,
                &[10, 20, 30, 4, 5, 6][..],
                Some(&[10, 20, 30][..])
            )
        );

        // 8-bit gray with a key, same treatment.
        let src = raw_png(
            (2, 1),
            ColorType::Grayscale,
            BitDepth::Eight,
            None,
            Some(vec![0, 7]),
            &[7, 8],
        );
        let img = Img::decode(&src).unwrap();
        assert_eq!(
            (img.ct, img.key.as_deref()),
            (ColorType::Grayscale, Some(&[7u8][..]))
        );
        let out = img.encode().unwrap();
        assert_eq!(chunk_data(&out, "tRNS"), Some(&[0u8, 7][..]));

        // Joined with other parts, the key becomes the alpha it stands for.
        let unkeyed = img.unkey();
        assert_eq!(
            (unkeyed.ct, &unkeyed.data[..], unkeyed.key),
            (ColorType::GrayscaleAlpha, &[7, 0, 8, 255][..], None)
        );
    }

    #[test]
    fn stack_checks_colour_chunks() {
        let tagged = |icc: Option<&[u8]>, phys: Option<(u32, u32, Unit)>| Img {
            meta: Meta {
                icc: icc.map(<[u8]>::to_vec),
                phys,
                ..Meta::default()
            },
            ..test_img(2, 2, ColorType::Rgb)
        };
        let bg = [0, 0, 0, 255];
        let p1 = Some((100, 100, Unit::Meter));
        let p2 = Some((200, 200, Unit::Meter));
        let same = stack(vec![tagged(Some(b"a"), p1), tagged(Some(b"a"), p1)], 1, bg).unwrap();
        assert_eq!(
            (same.meta.icc.as_deref(), same.meta.phys),
            (Some(&b"a"[..]), p1)
        );
        // pHYs disagreeing is dropped, colour is kept.
        let mixed = stack(vec![tagged(Some(b"a"), p1), tagged(Some(b"a"), p2)], 1, bg).unwrap();
        assert_eq!(
            (mixed.meta.icc.as_deref(), mixed.meta.phys),
            (Some(&b"a"[..]), None)
        );
        // Colour disagreeing is an error naming the part.
        let e = stack(vec![tagged(Some(b"a"), None), tagged(None, None)], 1, bg).unwrap_err();
        assert!(
            e.contains("色彩元数据不一致") && e.contains("第 2 段"),
            "{e}"
        );
        let e = stack(vec![tagged(None, None), tagged(Some(b"b"), None)], 1, bg).unwrap_err();
        assert!(e.contains("iCCP 1 字节"), "{e}");

        // A keyed part joins as RGBA with alpha from the key.
        let keyed = Img {
            key: Some(vec![0, 7, 14]),
            ..test_img(1, 1, ColorType::Rgb)
        };
        let s = stack(vec![keyed, test_img(1, 1, ColorType::Rgb)], 0, bg).unwrap();
        assert_eq!(s.ct, ColorType::Rgba);
        assert_eq!(&s.data[..4], &[0, 7, 14, 0]);
        assert_eq!(s.key, None);
    }

    #[test]
    fn promote_never_loses_channels() {
        let g = plain(2, 1, ColorType::Grayscale, vec![10, 20]);
        assert_eq!(
            g.clone().promote(ColorType::Rgb).unwrap().data,
            [10, 10, 10, 20, 20, 20]
        );
        assert_eq!(
            g.clone().promote(ColorType::Rgba).unwrap().data,
            [10, 10, 10, 255, 20, 20, 20, 255]
        );
        assert_eq!(
            g.promote(ColorType::GrayscaleAlpha).unwrap().data,
            [10, 255, 20, 255]
        );
        let ga = plain(1, 1, ColorType::GrayscaleAlpha, vec![7, 99]);
        assert_eq!(
            ga.clone().promote(ColorType::Rgba).unwrap().data,
            [7, 7, 7, 99]
        );
        let rgba = plain(1, 1, ColorType::Rgba, vec![1, 2, 3, 4]);
        assert!(rgba.clone().promote(ColorType::Rgb).is_err());
        assert!(rgba.promote(ColorType::GrayscaleAlpha).is_err());
        assert!(ga.promote(ColorType::Grayscale).is_err());
    }

    #[test]
    fn stack_layout_choice() {
        use ColorType::*;
        let gray = [128, 128, 128, 255];
        let red = [255, 0, 0, 255];
        assert_eq!(stack_type(&[Grayscale, Grayscale], gray), Grayscale);
        assert_eq!(stack_type(&[Grayscale], red), Rgb);
        assert_eq!(stack_type(&[GrayscaleAlpha], red), Rgba);
        assert_eq!(
            stack_type(&[GrayscaleAlpha, GrayscaleAlpha], gray),
            GrayscaleAlpha
        );
        assert_eq!(stack_type(&[Rgb, Rgb], gray), Rgb);
        assert_eq!(stack_type(&[Rgba], red), Rgba);
        assert_eq!(stack_type(&[Rgb, Grayscale], gray), Rgba);
        assert_eq!(stack_type(&[Grayscale, Rgb], red), Rgba);
        assert_eq!(stack_type(&[Grayscale], [9, 9, 9, 0]), GrayscaleAlpha);
    }

    #[test]
    fn stack_gray_parts_with_auto_background() {
        let a = test_img(8, 8, ColorType::Grayscale);
        let b = test_img(6, 6, ColorType::Grayscale);
        let bg = auto_bg(&a).unwrap();
        assert_eq!(
            bg,
            [
                sample(&a, 5, 5)[0],
                sample(&a, 5, 5)[0],
                sample(&a, 5, 5)[0],
                255
            ]
        );
        let top = a
            .crop(Rect {
                x0: 0,
                y0: 0,
                x1: 8,
                y1: 3,
            })
            .unwrap();
        let bottom = b
            .crop(Rect {
                x0: 0,
                y0: 1,
                x1: 6,
                y1: 3,
            })
            .unwrap();
        let out = stack(vec![top.clone(), bottom.clone()], 4, bg).unwrap();
        assert_eq!((out.ct, out.w, out.h), (ColorType::Grayscale, 8, 3 + 4 + 2));
        let fill = bg[0];
        for y in 0..3 {
            for x in 0..8 {
                assert_eq!(sample(&out, x, y), sample(&top, x, y));
            }
        }
        for y in 3..7 {
            assert!((0..8).all(|x| sample(&out, x, y) == [fill]), "gap row {y}");
        }
        for y in 0..2 {
            for x in 0..6 {
                assert_eq!(sample(&out, x, 7 + y), sample(&bottom, x, y));
            }
            for x in 6..8 {
                assert_eq!(sample(&out, x, 7 + y), [fill], "right padding");
            }
        }
    }

    #[test]
    fn stack_promotes_gray_for_coloured_background() {
        let a = test_img(4, 2, ColorType::Grayscale);
        let out = stack(vec![a.clone(), a.clone()], 1, [255, 0, 0, 255]).unwrap();
        assert_eq!((out.ct, out.w, out.h), (ColorType::Rgb, 4, 5));
        for y in 0..2 {
            for x in 0..4 {
                let g = sample(&a, x, y)[0];
                assert_eq!(sample(&out, x, y), [g, g, g]);
                assert_eq!(sample(&out, x, y + 3), [g, g, g]);
            }
        }
        assert!((0..4).all(|x| sample(&out, x, 2) == [255, 0, 0]));
    }

    #[test]
    fn stack_mixed_layouts_become_rgba_without_loss() {
        let rgb = test_img(4, 2, ColorType::Rgb);
        let ga = test_img(3, 2, ColorType::GrayscaleAlpha);
        let out = stack(vec![rgb.clone(), ga.clone()], 0, [1, 2, 3, 255]).unwrap();
        assert_eq!((out.ct, out.w, out.h), (ColorType::Rgba, 4, 4));
        for y in 0..2 {
            for x in 0..4 {
                let p = sample(&rgb, x, y);
                assert_eq!(sample(&out, x, y), [p[0], p[1], p[2], 255]);
            }
            for x in 0..3 {
                let p = sample(&ga, x, y);
                assert_eq!(sample(&out, x, y + 2), [p[0], p[0], p[0], p[1]]);
            }
            assert_eq!(sample(&out, 3, y + 2), [1, 2, 3, 255]);
        }
    }

    #[test]
    fn auto_background_needs_room() {
        let tiny = test_img(5, 9, ColorType::Rgb);
        assert!(auto_bg(&tiny).unwrap_err().contains("(5,5)"));
        assert_eq!(bg_hex([0xaa, 0xbb, 0x01, 255]), "#aabb01");
        assert_eq!(bg_hex([0xaa, 0xbb, 0x01, 0x80]), "#aabb0180");
    }

    // --- lines and anchors --------------------------------------------------

    const BBOX: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html><html><body><doc><page width="612.000000" height="792.000000"><flow>
<block xMin="72.0" yMin="100.0" xMax="300.0" yMax="140.0">
<line xMin="72.0" yMin="120.0" xMax="200.0" yMax="132.0"><word xMin="72.0" yMin="120.0" xMax="90.0" yMax="132.0">x</word><word xMin="92.0" yMin="120.0" xMax="99.0" yMax="132.0">&lt;</word><word xMin="100.0" yMin="120.0" xMax="150.0" yMax="132.0">y &#8217; &#x41; &bogus;</word></line>
<line xMin="72.0" yMin="100.25" xMax="300.0" yMax="112.3"><word xMin="72.0" yMin="100.25" xMax="90.0" yMax="112.3">Tom</word>
<word xMin="92.0" yMin="100.25" xMax="99.0" yMax="112.3">&amp;</word>
<word xMin="100.0" yMin="100.25" xMax="150.0" yMax="112.3">Jerry&apos;s</word></line>
<line xMin="1" yMin="2" xMax="3" yMax="4"></line>
</block></flow></page></doc></body></html>"#;

    fn line(y0: u32, y1: u32, text: &str) -> Line {
        Line {
            y0,
            y1,
            text: text.into(),
        }
    }

    #[test]
    fn bbox_layout_lines_in_pixels() {
        let lines = parse_bbox_lines(BBOX, 144);
        assert_eq!(
            lines,
            vec![
                // 100.25 pt × 2 = 200.5 → 200; 112.3 pt × 2 = 224.6 → 225.
                line(200, 225, "Tom & Jerry's"),
                line(240, 264, "x < y ’ A &bogus;"),
            ]
        );
        assert_eq!(
            parse_bbox_lines(BBOX, 72)[1],
            line(120, 132, "x < y ’ A &bogus;")
        );
        assert!(parse_bbox_lines("no lines here", 144).is_empty());
        assert!(parse_bbox_lines("<line yMin=\"1\" yMax=\"2\"><word>a</word>", 144).is_empty());
    }

    #[test]
    fn entities_decode() {
        assert_eq!(
            unescape("a &amp; b &lt;c&gt; &quot;d&quot; &#65;&#x42;&#X43;"),
            "a & b <c> \"d\" ABC"
        );
        assert_eq!(
            unescape("AT&T; &; &#; &#xZZ; end&"),
            "AT&T; &; &#; &#xZZ; end&"
        );
        assert_eq!(unescape("&#1114112;"), "&#1114112;");
    }

    #[test]
    fn sidecar_round_trip() {
        let lines = vec![line(10, 20, "a b"), line(30, 44, "tab\there")];
        let tsv = lines_tsv(&lines);
        assert_eq!(tsv, "10\t20\ta b\n30\t44\ttab here\n");
        let back = parse_lines_tsv(&tsv).unwrap();
        assert_eq!(back, vec![line(10, 20, "a b"), line(30, 44, "tab here")]);
        assert!(parse_lines_tsv("10\t20\n").unwrap_err().contains("第 1 行"));
        assert!(parse_lines_tsv("x\t20\ttext\n").is_err());
        assert_eq!(
            parse_lines_tsv("30\t40\tb\n\n10\t20\ta\n").unwrap()[0].text,
            "a"
        );
    }

    fn word(
        text: &str,
        (left, top, right, bottom): (u32, u32, u32, u32),
        key: (u32, u32, u32),
    ) -> card::Word {
        card::Word {
            text: text.into(),
            left,
            top,
            right,
            bottom,
            conf: 90.0,
            line: key,
        }
    }

    #[test]
    fn ocr_words_group_into_lines() {
        let words = vec![
            word("Second", (10, 60, 70, 80), (1, 1, 2)),
            word("line", (80, 58, 110, 79), (1, 1, 2)),
            word("First", (10, 20, 50, 40), (1, 1, 1)),
            word("one", (60, 22, 90, 45), (1, 1, 1)),
            word("Other", (10, 100, 50, 120), (2, 1, 1)),
        ];
        assert_eq!(
            lines_from_words(&words),
            vec![
                line(20, 45, "First one"),
                line(58, 80, "Second line"),
                line(100, 120, "Other")
            ]
        );
        assert!(lines_from_words(&[]).is_empty());
    }

    fn sample_lines() -> Vec<Line> {
        vec![
            line(10, 30, "Title Here"),
            line(40, 60, "First paragraph about  Foo"),
            line(70, 90, "second   line Foo bar"),
            line(100, 120, "Footer"),
        ]
    }

    #[test]
    fn anchors_match_case_and_spacing_insensitively() {
        let l = sample_lines();
        assert_eq!(find_line(&l, "FIRST PARAGRAPH about foo", 0), Some(1));
        assert_eq!(find_line(&l, "second line foo", 0), Some(2));
        assert_eq!(find_line(&l, "foo", 0), Some(1));
        assert_eq!(find_line(&l, "foo", 2), Some(2));
        assert_eq!(find_line(&l, "missing", 0), None);
    }

    #[test]
    fn anchor_box_pads_clamps_and_orders() {
        let l = sample_lines();
        let span = anchor_box(&l, "first", "second line foo", (24, 24), (200, 150)).unwrap();
        assert_eq!(
            span.rect,
            Rect {
                x0: 0,
                y0: 16,
                x1: 200,
                y1: 114
            }
        );
        assert_eq!((span.start, span.end), (1, 2));

        // Padding clamps to the image instead of failing.
        let span = anchor_box(&l, "title", "footer", (24, 100), (200, 150)).unwrap();
        assert_eq!((span.rect.y0, span.rect.y1), (0, 150));

        // One line can be both anchors: the end search includes the start line.
        let span = anchor_box(&l, "Footer", "footer", (0, 0), (200, 150)).unwrap();
        assert_eq!(
            span.rect,
            Rect {
                x0: 0,
                y0: 100,
                x1: 200,
                y1: 120
            }
        );

        // The end anchor is only searched from the start line down.
        let e = anchor_box(&l, "footer", "title", (0, 0), (200, 150)).unwrap_err();
        assert!(e.contains("结束锚点"), "{e}");
        let e = anchor_box(&l, "nowhere", "footer", (0, 0), (200, 150)).unwrap_err();
        assert!(e.contains("起始锚点") && e.contains("4 行"), "{e}");
        assert!(anchor_box(&l, "  ", "footer", (0, 0), (200, 150)).is_err());
        assert!(anchor_box(&[], "a", "b", (0, 0), (200, 150)).is_err());
    }

    #[test]
    fn cut_edges_are_reported() {
        let l = vec![line(10, 30, "A"), line(32, 50, "B")];
        let none = cut_warnings(
            &l,
            Rect {
                x0: 0,
                y0: 31,
                x1: 5,
                y1: 31,
            },
        );
        assert!(none.is_empty());
        let w = cut_warnings(
            &l,
            Rect {
                x0: 0,
                y0: 20,
                x1: 5,
                y1: 40,
            },
        );
        assert_eq!(w.len(), 2);
        assert!(w[0].contains("上沿") && w[0].contains("--pad-top"));
        assert!(w[1].contains("下沿") && w[1].contains("--pad-bottom"));
        // Edges exactly on a line boundary cut nothing.
        assert!(
            cut_warnings(
                &l,
                Rect {
                    x0: 0,
                    y0: 10,
                    x1: 5,
                    y1: 50
                }
            )
            .is_empty()
        );
    }

    // --- external programs and files ----------------------------------------

    #[test]
    fn chrome_arguments() {
        let r = Render {
            url: "https://example.com".into(),
            name: "ex".into(),
            width: 700,
            height: 1600,
            scale: 2.0,
            chrome_args: vec!["--user-agent=x".into()],
        };
        let args = chrome_args(Path::new("/tmp/prof"), &r, Path::new("/tmp/out.png"));
        assert_eq!(
            args,
            [
                "--headless=new",
                "--user-data-dir=/tmp/prof",
                "--disable-gpu",
                "--hide-scrollbars",
                "--force-device-scale-factor=2",
                "--window-size=700,1600",
                "--timeout=30000",
                "--lang=en-US",
                "--user-agent=x",
                "--screenshot=/tmp/out.png",
                "https://example.com",
            ]
        );
        assert!(
            args.iter()
                .all(|a| !a.contains("virtual-time-budget") && !a.contains("allow-file-access"))
        );
        let r = Render { scale: 1.5, ..r };
        assert!(
            chrome_args(Path::new("p"), &r, Path::new("o"))
                .contains(&"--force-device-scale-factor=1.5".to_string())
        );
    }

    #[test]
    fn tool_strings() {
        let r = Render {
            url: String::new(),
            name: String::new(),
            width: 700,
            height: 1,
            scale: 2.0,
            chrome_args: vec![],
        };
        let chrome = tool_chrome(&r);
        assert_eq!(chrome, "headless Chrome 700 CSS px, DPR 2");
        assert_eq!(
            tool_label(Some(&chrome), "Tesseract anchor crop"),
            "headless Chrome 700 CSS px, DPR 2; Tesseract anchor crop"
        );
        assert_eq!(tool_label(None, "pixel-box crop"), "pixel-box crop");
        assert_eq!(
            format!("{} crop", tool_pdf(144)),
            "pdftoppm -r 144 + pdftotext -bbox-layout crop"
        );
    }

    #[test]
    fn json_lines_keep_order_and_unicode() {
        let s = jsonl(&[
            ("time_bj", json!("t")),
            ("action", json!("crop")),
            ("anchors", json!(["起", "止\"x"])),
            ("box", json!([0, 1, 2, 3])),
        ]);
        assert_eq!(
            s,
            r#"{"time_bj":"t","action":"crop","anchors":["起","止\"x"],"box":[0,1,2,3]}"#
        );
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["anchors"][1], "止\"x");
    }

    fn scratch_root(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("barking-shot-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn tool_lookup_prefers_configured_dirs() {
        let root = scratch_root("tools");
        let (a, b) = (root.join("a"), root.join("b"));
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        let exe = format!("pdftotext{}", std::env::consts::EXE_SUFFIX);
        assert_eq!(
            pick_tool(&[a.clone(), b.clone()], "pdftotext"),
            PathBuf::from("pdftotext")
        );
        fs::write(b.join(&exe), b"").unwrap();
        assert_eq!(
            pick_tool(&[a.clone(), b.clone()], "pdftotext"),
            b.join(&exe)
        );
        fs::write(a.join(&exe), b"").unwrap();
        assert_eq!(pick_tool(&[a.clone(), b], "pdftotext"), a.join(&exe));
        let _ = fs::remove_dir_all(&root);
    }

    fn test_ctx(tag: &str) -> (Ctx, PathBuf) {
        let root = scratch_root(tag);
        let dir = root.join("docs/2610/1009");
        fs::create_dir_all(&dir).unwrap();
        let issue = Issue {
            root: root.clone(),
            dir,
            yymm: "2610".into(),
            mmdd: "1009".into(),
        };
        let mut ctx = Ctx::new(issue, "t");
        // Never the real `<temp>/barking-shot/…`: tests must not see or leave
        // scratch files there.
        ctx.scratch = root.join("scratch");
        (ctx, root)
    }

    #[test]
    fn emit_never_overwrites() {
        let (ctx, root) = test_ctx("emit");
        let img = test_img(4, 3, ColorType::Rgb);
        let path = ctx.emit("o.png", &img).unwrap();
        assert_eq!(path, ctx.issue.images().join("o.png"));
        let written = fs::read(&path).unwrap();
        assert_eq!(Img::decode(&written).unwrap(), img);

        let other = test_img(5, 5, ColorType::Rgb);
        let e = ctx.emit("o.png", &other).unwrap_err();
        assert!(e.contains("拒绝覆盖"), "{e}");
        assert!(ctx.check_free("o.png").unwrap_err().contains("拒绝覆盖"));
        assert!(ctx.check_free("fresh.png").is_ok());
        assert_eq!(fs::read(&path).unwrap(), written, "existing file untouched");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn log_appends_ordered_lines() {
        let (ctx, root) = test_ctx("log");
        ctx.log(vec![("action", json!("crop")), ("file", json!("a.png"))])
            .unwrap();
        let path = ctx.issue.sources().join("t-shots-log.jsonl");
        // Simulate a log whose last line lost its newline.
        let mut text = fs::read_to_string(&path).unwrap();
        assert!(
            text.starts_with("{\"time_bj\":\"20")
                && text.ends_with("+08:00\",\"action\":\"crop\",\"file\":\"a.png\"}\n"),
            "{text}"
        );
        text.pop();
        fs::write(&path, &text).unwrap();
        ctx.log(vec![("action", json!("stack"))]).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        for l in &lines {
            serde_json::from_str::<Value>(l).unwrap();
        }
        assert!(lines[1].contains("\"action\":\"stack\""));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn scratch_sources_resolve() {
        let (ctx, root) = test_ctx("resolve");
        let miss = ctx.resolve(&Source::Name("pg".into())).err().unwrap();
        assert!(miss.contains("先对「pg」运行 render 或 pdf"), "{miss}");
        fs::create_dir_all(&ctx.scratch).unwrap();
        fs::write(ctx.scratch_png("pg"), b"").unwrap();
        let r = ctx.resolve(&Source::Name("pg".into())).unwrap();
        assert_eq!((r.name.as_str(), r.from.path.as_str()), ("pg", "pg"));
        assert!(!r.from.outside && r.prov.is_none());
        let file = root.join("my.shot.png");
        fs::write(&file, b"").unwrap();
        let r = ctx.resolve(&Source::File(file.clone())).unwrap();
        assert_eq!(r.name, "my.shot");
        // Inside the repository: relative, `/`-separated, never absolute.
        assert_eq!(r.from.path, "my.shot.png");
        assert!(!r.from.outside);
        let deep = ctx.issue.images().join("x.png");
        fs::create_dir_all(ctx.issue.images()).unwrap();
        fs::write(&deep, b"").unwrap();
        let r = ctx.resolve(&Source::File(deep)).unwrap();
        assert_eq!(r.from.path, "docs/2610/1009/images/x.png");
        // Outside: the file name only.
        let elsewhere = scratch_root("resolve-outside");
        let out_file = elsewhere.join("far.png");
        fs::write(&out_file, b"").unwrap();
        let r = ctx.resolve(&Source::File(out_file)).unwrap();
        assert_eq!((r.from.path.as_str(), r.from.outside), ("far.png", true));
        let _ = fs::remove_dir_all(&elsewhere);
        assert!(ctx.resolve(&Source::File(root.join("none.png"))).is_err());
        // Sidecar names hang off the PNG's own name.
        assert_eq!(
            file.with_extension(SIDECAR_LINES),
            root.join("my.shot.lines.tsv")
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn page_png_lookup() {
        let root = scratch_root("page");
        assert!(find_page_png(&root).unwrap_err().contains("页码超出"));
        fs::write(root.join("pg-07.png"), b"").unwrap();
        fs::write(root.join("other.png"), b"").unwrap();
        assert_eq!(find_page_png(&root).unwrap(), root.join("pg-07.png"));
        fs::write(root.join("pg-08.png"), b"").unwrap();
        assert!(find_page_png(&root).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn options_with_equals_and_help() {
        let p = parse_ok(&[
            "d",
            "a",
            "render",
            "https://e.com",
            "ex",
            "--width=900",
            "--chrome-arg=--no-sandbox",
            "--chrome-arg",
            "--mute-audio",
        ]);
        let Action::Render(r) = p.action else {
            panic!("{p:?}")
        };
        assert_eq!(r.width, 900);
        assert_eq!(r.chrome_args, ["--no-sandbox", "--mute-audio"]);
        // `=` splits once; the rest belongs to the value.
        let p = parse_ok(&["d", "a", "crop", "n", "o.png", "--start=a=b", "--end", "c"]);
        let Action::Crop(c) = p.action else {
            panic!("{p:?}")
        };
        assert_eq!((c.start.as_str(), c.end.as_str()), ("a=b", "c"));
        assert!(parse_err(&["d", "a", "lines", "n", "--=x"]).contains("缺少名字"));

        assert!(wants_help(&s(&["--help"])));
        assert!(wants_help(&s(&["-h"])));
        assert!(wants_help(&s(&["d", "a", "crop", "--help"])));
        assert!(!wants_help(&s(&[
            "d",
            "a",
            "render",
            "u",
            "n",
            "--chrome-arg",
            "--help"
        ])));
        assert!(!wants_help(&s(&[
            "d",
            "a",
            "render",
            "u",
            "n",
            "--chrome-arg=--help"
        ])));
        assert!(!wants_help(&s(&[
            "d", "a", "crop", "n", "o.png", "--start", "-h"
        ])));
    }

    #[test]
    fn provenance_sidecar_round_trip() {
        let p = Prov {
            tool: tool_pdf(144),
            pdf: Some("docs/2610/1012/sources/a.pdf".into()),
            page: Some(3),
            ..Prov::default()
        };
        assert_eq!(Prov::parse(&p.text()), Some(p));
        let r = Prov {
            tool: "headless Chrome 700 CSS px, DPR 2".into(),
            url: Some("https://example.com/a\tb".into()),
            ..Prov::default()
        };
        let back = Prov::parse(&r.text()).unwrap();
        assert_eq!(back.url.as_deref(), Some("https://example.com/a b"));
        // A scratch image from before this sidecar format: tool only.
        assert_eq!(
            Prov::parse("pdftoppm -r 144 + pdftotext -bbox-layout\n"),
            Some(Prov {
                tool: tool_pdf(144),
                ..Prov::default()
            })
        );
        assert_eq!(Prov::parse("\n"), None);
    }

    #[test]
    fn scratch_key_has_the_year() {
        let (ctx, root) = test_ctx("key");
        let real = Ctx::new(ctx.issue, "z");
        assert!(real.scratch.ends_with("barking-shot/26101009-z"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn install_replaces_image_and_sidecars() {
        let (ctx, root) = test_ctx("install");
        fs::create_dir_all(&ctx.scratch).unwrap();
        let out = ctx.scratch_png("pg");
        fs::write(&out, b"old").unwrap();
        fs::write(out.with_extension(SIDECAR_LINES), b"0\t1\told\n").unwrap();
        let made = ctx.scratch.join("new.png");
        fs::write(&made, b"new").unwrap();
        let prov = Prov {
            tool: "t".into(),
            url: Some("https://e.com".into()),
            ..Prov::default()
        };
        install_scratch(&made, &out, &prov, None).unwrap();
        assert_eq!(fs::read(&out).unwrap(), b"new");
        assert!(!made.exists());
        assert!(
            !out.with_extension(SIDECAR_LINES).exists(),
            "stale lines of the old image must go"
        );
        assert_eq!(read_prov(&out).as_ref(), Some(&prov));

        let made = ctx.scratch.join("new2.png");
        fs::write(&made, b"new2").unwrap();
        let lines = [line(1, 2, "x")];
        install_scratch(&made, &out, &Prov::default(), Some(&lines)).unwrap();
        assert_eq!(
            fs::read_to_string(out.with_extension(SIDECAR_LINES)).unwrap(),
            "1\t2\tx\n"
        );
        // A failed install leaves neither a half image nor its sidecars.
        let e = install_scratch(&ctx.scratch.join("missing.png"), &out, &prov, None);
        assert!(e.is_err());
        assert!(!out.exists() && !out.with_extension(SIDECAR_TOOL).exists());
        let _ = fs::remove_dir_all(&root);
    }

    // --- log hygiene, unsupported colour chunks, gray ICC -------------------

    #[test]
    fn urls_are_cleaned_for_the_log() {
        let root = Path::new("/nowhere-repo");
        let u = |s: &str| log_url(root, s);
        assert_eq!(
            u("https://example.com/?token=abc&x=1"),
            "https://example.com/?token=[REDACTED]&x=1"
        );
        assert_eq!(
            u("https://user:p%40ss@example.com:8443/a@b?q=1"),
            "https://example.com:8443/a@b?q=1"
        );
        assert_eq!(u("http://bob@h/"), "http://h/");
        assert_eq!(
            u(
                "https://s3.example.com/o?X-Amz-Signature=f00&X-Amz-Credential=c&API_KEY=k&sig=1&Session_ID=2&PassWord=3&cookie=4&access=5&secret=6&auth=7&api%5Fkey=8&page=2"
            ),
            "https://s3.example.com/o?X-Amz-Signature=[REDACTED]&X-Amz-Credential=c&API_KEY=[REDACTED]&sig=[REDACTED]&Session_ID=[REDACTED]&PassWord=[REDACTED]&cookie=[REDACTED]&access=[REDACTED]&secret=[REDACTED]&auth=[REDACTED]&api%5Fkey=[REDACTED]&page=2"
        );
        // Substring rule, on purpose: `author` and `keyword` are redacted too.
        assert_eq!(
            u("https://h/?author=a&keyword=b&id=c"),
            "https://h/?author=[REDACTED]&keyword=[REDACTED]&id=c"
        );
        // Fragment parameters (OAuth implicit flow) are treated alike; a plain
        // anchor or a bare flag is kept.
        assert_eq!(
            u("https://h/cb#access_token=t&state=s"),
            "https://h/cb#access_token=[REDACTED]&state=s"
        );
        assert_eq!(u("https://h/p?token#sec-2"), "https://h/p?token#sec-2");
        assert_eq!(u("https://example.com"), "https://example.com");
        assert_eq!(u(""), "");
    }

    #[test]
    fn file_urls_become_repo_paths_or_file_names() {
        let (ctx, root) = test_ctx("fileurl");
        let src = ctx.issue.sources();
        fs::create_dir_all(&src).unwrap();
        let inside = src.join("a b.html");
        fs::write(&inside, b"x").unwrap();
        let url = card::file_url(&inside).unwrap();
        assert!(url.contains("%20"), "{url}");
        assert_eq!(
            log_url(&root, &url),
            "docs/2610/1009/sources/a b.html",
            "{url}"
        );
        assert_eq!(
            log_url(&root, &format!("{url}?token=x#y")),
            "docs/2610/1009/sources/a b.html"
        );
        let out_root = scratch_root("fileurl-out");
        let outside = out_root.join("secret dir").join("page.html");
        let url = card::file_url(&outside).unwrap();
        assert_eq!(log_url(&root, &url), "page.html", "{url}");
        assert_eq!(
            log_url(&root, "file:///C:/Users/someone/Desktop/x%20y.html"),
            "x y.html"
        );
        assert_eq!(log_url(&root, "FILE://server/share/z.pdf"), "z.pdf");
        let _ = fs::remove_dir_all(&out_root);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn chrome_args_are_cleaned_for_the_log() {
        let (ctx, root) = test_ctx("chromearg");
        let a = |s: &str| log_chrome_arg(&root, s);
        assert_eq!(a("--lang=en-US"), "--lang=en-US");
        assert_eq!(a("--no-sandbox"), "--no-sandbox");
        assert_eq!(
            a("--user-agent=Mozilla/5.0 x"),
            "--user-agent=Mozilla/5.0 x"
        );
        assert_eq!(a("--Auth-Token=abc"), "--Auth-Token=[REDACTED]");
        assert_eq!(a("--session=s=1"), "--session=[REDACTED]");
        assert_eq!(a("token=abc"), "token=[REDACTED]");
        assert_eq!(
            a("--proxy-server=http://u:pw@proxy:8080"),
            "--proxy-server=http://proxy:8080"
        );
        assert_eq!(
            a("https://u:pw@h/?key=k&x=1"),
            "https://h/?key=[REDACTED]&x=1"
        );
        assert_eq!(
            a("--user-data-dir=C:\\Users\\someone\\AppData\\prof"),
            "--user-data-dir=prof"
        );
        let page = ctx.issue.dir.join("p.html");
        fs::write(&page, b"x").unwrap();
        let url = card::file_url(&page).unwrap();
        assert_eq!(a(&format!("--app={url}")), "--app=docs/2610/1009/p.html");
        assert_eq!(
            a("--token-file=file:///C:/Users/someone/t.txt"),
            "--token-file=t.txt"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn log_scrubs_every_url_and_chrome_arg() {
        let (ctx, root) = test_ctx("scrub");
        ctx.log(vec![
            ("action", json!("render")),
            ("url", json!("https://u:p@example.com/?token=abc&x=1")),
            ("name", json!("ex")),
            ("extra", json!(["--lang=en-US", "--cookie=c1"])),
        ])
        .unwrap();
        let text = fs::read_to_string(ctx.issue.sources().join("t-shots-log.jsonl")).unwrap();
        let v: Value = serde_json::from_str(text.lines().next().unwrap()).unwrap();
        assert_eq!(v["url"], "https://example.com/?token=[REDACTED]&x=1");
        assert_eq!(v["extra"], json!(["--lang=en-US", "--cookie=[REDACTED]"]));
        assert_eq!(v["name"], "ex");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn inherited_urls_are_scrubbed_in_crops_and_stacks() {
        let (ctx, root) = test_ctx("inherit");
        fs::create_dir_all(&ctx.scratch).unwrap();
        let png = ctx.scratch_png("pg");
        fs::write(&png, test_img(8, 8, ColorType::Rgb).encode().unwrap()).unwrap();
        let raw = "https://me:pw@example.com/doc?access_token=SECRET1&x=1";
        let prov = Prov {
            tool: "headless Chrome 700 CSS px, DPR 2".into(),
            url: Some(raw.into()),
            ..Prov::default()
        };
        fs::write(png.with_extension(SIDECAR_TOOL), prov.text()).unwrap();
        let rect = Rect {
            x0: 0,
            y0: 0,
            x1: 4,
            y1: 4,
        };
        do_cropbox(&ctx, &Source::Name("pg".into()), "c.png", rect, "").unwrap();
        let parts = [Part {
            src: Source::Name("pg".into()),
            rect,
        }];
        do_stack(
            &ctx,
            "s.png",
            &parts,
            0,
            &Bg::Auto,
            "https://h/?sig=SECRET2",
        )
        .unwrap();
        let text = fs::read_to_string(ctx.issue.sources().join("t-shots-log.jsonl")).unwrap();
        assert!(
            !text.contains("SECRET") && !text.contains("me:pw"),
            "{text}"
        );
        let clean = "https://example.com/doc?access_token=[REDACTED]&x=1";
        let lines: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(lines[0]["url"], clean);
        assert_eq!(lines[1]["url"], "https://h/?sig=[REDACTED]");
        assert_eq!(lines[1]["parts"][0]["url"], clean);
        // The scratch sidecar keeps the raw URL: it never enters the repo.
        assert_eq!(read_prov(&png).unwrap().url.as_deref(), Some(raw));
        let _ = fs::remove_dir_all(&root);
    }

    /// `png` with an extra chunk spliced in right after IHDR. The CRC is
    /// bogus: the refusal has to come before the decoder reads the chunk.
    fn with_chunk(png: &[u8], name: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let ihdr_end = 8 + 12 + 13;
        let mut out = png[..ihdr_end].to_vec();
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(data);
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&png[ihdr_end..]);
        out
    }

    #[test]
    fn hdr_colour_chunks_are_refused() {
        let base = raw_png(
            (2, 1),
            ColorType::Rgb,
            BitDepth::Eight,
            None,
            None,
            &[1, 2, 3, 4, 5, 6],
        );
        assert_eq!(chunk_names(&base)[..2], ["IHDR", "IDAT"]);
        assert!(unkept_colour_chunk(&base).is_none());
        for (name, data) in [
            (b"cICP", &[9u8, 16, 0, 1][..]),
            (b"mDCV", &[0u8; 24][..]),
            (b"cLLI", &[0u8; 8][..]),
        ] {
            let png = with_chunk(&base, name, data);
            let e = Img::decode(&png).unwrap_err();
            assert!(
                e.starts_with("不支持 cICP/mDCV/cLLI 色彩块的 PNG，先用其他工具转换或另取底图")
                    && e.contains(std::str::from_utf8(name).unwrap()),
                "{e}"
            );
        }
        // Not a PNG, or a chunk length that runs off the end: left to the
        // decoder, never a panic or a false hit.
        assert!(unkept_colour_chunk(b"nope").is_none());
        let mut huge = base.clone();
        huge[8..12].copy_from_slice(&u32::MAX.to_be_bytes());
        assert!(unkept_colour_chunk(&huge).is_none());
    }

    #[test]
    fn gray_with_icc_is_not_promoted_to_colour() {
        let icc = Some(vec![1u8, 2, 3]);
        let mut g = plain(2, 1, ColorType::Grayscale, vec![10, 20]);
        g.meta.icc = icc.clone();
        for to in [ColorType::Rgb, ColorType::Rgba] {
            let e = g.clone().promote(to).unwrap_err();
            assert!(e.contains("ICC") && e.contains("分别出图"), "{e}");
        }
        // Same colour model: fine, the profile still fits.
        assert_eq!(
            g.clone().promote(ColorType::GrayscaleAlpha).unwrap().data,
            [10, 255, 20, 255]
        );
        let mut rgb = plain(1, 1, ColorType::Rgb, vec![1, 2, 3]);
        rgb.meta.icc = icc.clone();
        assert_eq!(rgb.promote(ColorType::Rgba).unwrap().data, [1, 2, 3, 255]);

        // Through `stack`: a coloured background would need RGB.
        let e = stack(vec![g.clone(), g.clone()], 1, [255, 0, 0, 255]).unwrap_err();
        assert!(e.contains("第 1 段") && e.contains("ICC"), "{e}");
        let out = stack(vec![g.clone(), g.clone()], 1, [7, 7, 7, 255]).unwrap();
        assert_eq!((out.ct, out.meta.icc), (ColorType::Grayscale, icc));
        // Without a profile the old promotion still applies.
        g.meta.icc = None;
        assert_eq!(
            stack(vec![g.clone()], 0, [255, 0, 0, 255]).unwrap().ct,
            ColorType::Rgb
        );
    }
}
