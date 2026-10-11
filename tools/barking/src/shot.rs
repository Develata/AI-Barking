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

use png::{BitDepth, ColorType};
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
  <组前缀>、<name> 只许小写字母、数字与 -。render / pdf 只产生中间图，放在 <临时目录>/barking-shot/<MMDD>-<组>/<name>.png；
  先用 lines 找锚点行，再用 crop / cropbox / stack 裁成 <期次目录>/images/<out.png>（已存在则拒绝覆盖）。
  --part 的 <png> 可写文件路径，或 render / pdf 的 <name>；像素框取 [x0,x1) × [y0,y1)，单位是该图的像素。
  每次出图在 sources/<组>-shots-log.jsonl 追加一行记录。只裁切、拼接，不改像素。
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

/// `--key value` options plus positionals, split before any action looks at
/// them. Every option takes a value, so `--chrome-arg --no-sandbox` is one
/// option, not two.
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
        if let Some(key) = t.strip_prefix("--") {
            let value = it.next().ok_or_else(|| format!("--{key} 缺少参数"))?;
            args.opts.push((key.to_string(), value.clone()));
        } else {
            args.pos.push(t.clone());
        }
    }
    Ok(args)
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

/// 8-bit image in one of the four PNG layouts that carry samples directly;
/// indexed and low-bit-depth input is expanded on load.
#[derive(Clone, Debug, PartialEq)]
struct Img {
    w: u32,
    h: u32,
    ct: ColorType,
    data: Vec<u8>,
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
    /// samples (a `tRNS` chunk becomes an alpha channel) and 16-bit stays 16-bit
    /// so it can be refused. Nothing is scaled or gamma-corrected.
    fn decode(bytes: &[u8]) -> Result<Img, String> {
        let limits = png::Limits { bytes: MAX_BYTES };
        let mut dec = png::Decoder::new_with_limits(io::Cursor::new(bytes), limits);
        dec.set_transformations(png::Transformations::EXPAND);
        let mut reader = dec
            .read_info()
            .map_err(|e| format!("不是可读的 PNG：{e}"))?;
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
        Ok(Img {
            w: info.width,
            h: info.height,
            ct: info.color_type,
            data,
        })
    }

    fn encode(&self) -> Result<Vec<u8>, String> {
        let fail = |e: png::EncodingError| format!("无法编码 PNG：{e}");
        let mut buf = Vec::new();
        let mut enc = png::Encoder::new(&mut buf, self.w, self.h);
        enc.set_color(self.ct);
        enc.set_depth(BitDepth::Eight);
        let mut writer = enc.write_header().map_err(fail)?;
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
        })
    }

    fn rgba_at(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.w || y >= self.h {
            return None;
        }
        let bpp = self.ct.samples();
        let i = (y as usize * self.w as usize + x as usize) * bpp;
        let px = self.data.get(i..i + bpp)?;
        Some(match self.ct {
            ColorType::Grayscale => [px[0], px[0], px[0], 255],
            ColorType::GrayscaleAlpha => [px[0], px[0], px[0], px[1]],
            ColorType::Rgb => [px[0], px[1], px[2], 255],
            _ => [px[0], px[1], px[2], px[3]],
        })
    }

    /// Widens to a layout with at least the same channels: gray repeats into
    /// R=G=B and a missing alpha is opaque. Narrowing would drop information,
    /// so it is an error rather than a silent loss.
    fn promote(self, to: ColorType) -> Result<Img, String> {
        if self.ct == to {
            return Ok(self);
        }
        let ((colour, alpha), (to_colour, to_alpha)) = (channels(self.ct), channels(to));
        if (colour && !to_colour) || (alpha && !to_alpha) {
            return Err(format!("内部错误：{:?} 不能无损转成 {to:?}", self.ct));
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

/// Parts joined top to bottom with `gap` rows of `bg` between them; the canvas
/// is as wide as the widest part and the short ones are padded on the right
/// with `bg`.
fn stack(parts: Vec<Img>, gap: u32, bg: [u8; 4]) -> Result<Img, String> {
    let types: Vec<ColorType> = parts.iter().map(|p| p.ct).collect();
    let ct = stack_type(&types, bg);
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
    Ok(Img { w, h, ct, data })
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

/// How the scratch image was made, recorded by `render` / `pdf` so that later
/// crops can name their source in the log.
fn read_tool(png: &Path) -> Option<String> {
    let text = fs::read_to_string(png.with_extension(SIDECAR_TOOL)).ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
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

struct Ctx {
    issue: Issue,
    group: String,
    /// `<temp>/barking-shot/<MMDD>-<group>`
    scratch: PathBuf,
}

struct Resolved {
    png: PathBuf,
    /// Scratch name, or the file stem for `--from`.
    name: String,
    /// The `--from` path, for the log.
    from: Option<String>,
}

impl Ctx {
    fn new(issue: Issue, group: &str) -> Ctx {
        let scratch = std::env::temp_dir()
            .join("barking-shot")
            .join(format!("{}-{group}", issue.mmdd));
        Ctx {
            issue,
            group: group.to_string(),
            scratch,
        }
    }

    fn scratch_png(&self, name: &str) -> PathBuf {
        self.scratch.join(format!("{name}.png"))
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
                Ok(Resolved {
                    png,
                    name: n.clone(),
                    from: None,
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
                    from: Some(slash(p)),
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
    /// Beijing time first.
    fn log(&self, mut fields: Vec<(&str, Value)>) -> Result<(), String> {
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

fn do_render(ctx: &Ctx, r: &Render) -> Result<(), String> {
    let chrome = card::find_chrome()?;
    fs::create_dir_all(&ctx.scratch).map_err(io_err(&ctx.scratch))?;
    let out = ctx.scratch_png(&r.name);
    // A re-render replaces its own scratch file; deleting first means a file
    // that exists afterwards is Chrome's, and sidecars of the old image go too.
    let _ = fs::remove_file(&out);
    clear_sidecars(&out);
    let profile = ctx.scratch.join(format!("profile-{}", std::process::id()));
    let chrome_log = ctx.scratch.join(format!("{}-chrome.log", r.name));
    let ran = run_chrome(&chrome, &chrome_args(&profile, r, &out), &chrome_log);
    let _ = fs::remove_dir_all(&profile);
    ran?;
    if !out.is_file() {
        return Err(format!(
            "Chrome 正常退出但没有写出 {}（日志 {}）",
            out.display(),
            chrome_log.display()
        ));
    }
    let (w, h) = card::png_size(&out)?;
    let tool = tool_chrome(r);
    fs::write(out.with_extension(SIDECAR_TOOL), format!("{tool}\n"))
        .map_err(io_err(&out.with_extension(SIDECAR_TOOL)))?;

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

fn do_pdf(ctx: &Ctx, p: &PdfPage) -> Result<(), String> {
    if !p.pdf.is_file() {
        return Err(format!("{}: 不是文件", p.pdf.display()));
    }
    fs::create_dir_all(&ctx.scratch).map_err(io_err(&ctx.scratch))?;
    let work = ctx.scratch.join(format!("pdf-{}", p.name));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).map_err(io_err(&work))?;
    let dirs = poppler_dirs();
    let (page, dpi) = (p.page.to_string(), p.dpi.to_string());

    let ppm = pick_tool(&dirs, "pdftoppm");
    run_capture(
        Command::new(&ppm)
            .args(["-r", &dpi, "-f", &page, "-l", &page, "-png"])
            .arg(&p.pdf)
            .arg(work.join("pg")),
        "pdftoppm",
    )?;
    let made = find_page_png(&work)?;
    let out = ctx.scratch_png(&p.name);
    let _ = fs::remove_file(&out);
    clear_sidecars(&out);
    fs::rename(&made, &out)
        .or_else(|_| fs::copy(&made, &out).map(|_| ()))
        .map_err(io_err(&out))?;
    let _ = fs::remove_dir_all(&work);
    let (w, h) = card::png_size(&out)?;

    let txt = pick_tool(&dirs, "pdftotext");
    let text = run_capture(
        Command::new(&txt)
            .args(["-enc", "UTF-8", "-bbox-layout", "-f", &page, "-l", &page])
            .arg(&p.pdf)
            .arg("-"),
        "pdftotext",
    )
    .map_err(|e| {
        format!("{e}\n（需要 poppler 的 pdftotext；PATH 上的 xpdf 旧版没有 -bbox-layout，用 BARKING_POPPLER 指向 poppler 的 bin 目录）")
    })?;
    let lines = parse_bbox_lines(&String::from_utf8_lossy(&text.stdout), p.dpi);

    let tool = tool_pdf(p.dpi);
    let tool_file = out.with_extension(SIDECAR_TOOL);
    fs::write(&tool_file, format!("{tool}\n")).map_err(io_err(&tool_file))?;
    // No text layer (a scanned PDF): leave no sidecar, so `lines` / `crop` fall
    // back to OCR instead of searching an empty list.
    if lines.is_empty() {
        println!("注意：这一页没有文字层，lines / crop 将改用 Tesseract OCR");
    } else {
        let side = out.with_extension(SIDECAR_LINES);
        fs::write(&side, lines_tsv(&lines)).map_err(io_err(&side))?;
        if lines.iter().any(|l| l.y1 > h + 2) {
            println!("注意：有文字行超出渲染图高度 {h}（页面带旋转或裁切框？），锚点坐标可能偏移");
        }
    }
    println!(
        "{}  {w}×{h} px（第 {} 页，{} dpi，{} 行文字）",
        out.display(),
        p.page,
        p.dpi,
        lines.len()
    );
    ctx.log(vec![
        ("action", json!("pdf")),
        ("pdf", json!(slash(&p.pdf))),
        ("page", json!(p.page)),
        ("name", json!(p.name)),
        ("tool", json!(tool)),
        ("dpi", json!(p.dpi)),
        ("size", json!([w, h])),
        ("lines", json!(lines.len())),
    ])
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
    let prov = read_tool(&r.png);
    let tool = match how {
        LineSource::Sidecar => match &prov {
            Some(p) => format!("{p} crop"),
            None => "text-layer line crop".to_string(),
        },
        LineSource::Ocr => tool_label(prov.as_deref(), "Tesseract anchor crop"),
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
        ("url", json!(c.url)),
        ("name", json!(r.name)),
        ("file", json!(c.out)),
        ("box", span.rect.json()),
        ("full_size", json!([img.w, img.h])),
        ("anchors", json!([c.start, c.end])),
        ("pads", json!([c.pad_top, c.pad_bottom])),
        ("tool", json!(tool)),
    ];
    if let Some(from) = r.from {
        rec.push(("from", json!(from)));
    }
    ctx.log(rec)
}

fn do_cropbox(ctx: &Ctx, src: &Source, out: &str, rect: Rect, url: &str) -> Result<(), String> {
    let r = ctx.resolve(src)?;
    ctx.check_free(out)?;
    let img = Img::load(&r.png)?;
    let cropped = img.crop(rect)?;
    let tool = tool_label(read_tool(&r.png).as_deref(), "pixel-box crop");
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
        ("url", json!(url)),
        ("name", json!(r.name)),
        ("file", json!(out)),
        ("box", rect.json()),
        ("full_size", json!([img.w, img.h])),
        ("tool", json!(tool)),
    ];
    if let Some(from) = r.from {
        rec.push(("from", json!(from)));
    }
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
        records.push(json!({
            "src": slash(&r.png),
            "box": part.rect.json(),
            "full_size": [img.w, img.h],
        }));
        if let Some(t) = read_tool(&r.png)
            && !provenance.contains(&t)
        {
            provenance.push(t);
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

    #[test]
    fn promote_never_loses_channels() {
        let g = Img {
            w: 2,
            h: 1,
            ct: ColorType::Grayscale,
            data: vec![10, 20],
        };
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
        let ga = Img {
            w: 1,
            h: 1,
            ct: ColorType::GrayscaleAlpha,
            data: vec![7, 99],
        };
        assert_eq!(
            ga.clone().promote(ColorType::Rgba).unwrap().data,
            [7, 7, 7, 99]
        );
        let rgba = Img {
            w: 1,
            h: 1,
            ct: ColorType::Rgba,
            data: vec![1, 2, 3, 4],
        };
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
        (Ctx::new(issue, "t"), root)
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
        assert_eq!((r.name.as_str(), r.from), ("pg", None));
        let file = root.join("my.shot.png");
        fs::write(&file, b"").unwrap();
        let r = ctx.resolve(&Source::File(file.clone())).unwrap();
        assert_eq!(r.name, "my.shot");
        assert_eq!(r.from, Some(slash(&file)));
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
}
