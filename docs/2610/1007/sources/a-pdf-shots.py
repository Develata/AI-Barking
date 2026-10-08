"""1007 A 组：从 a-rma-nolla-health.pdf 渲染页面截图（pdftoppm 144 dpi = 2x 像素比，宽 1224 px，不超过 1400 px），
按 pdftotext -bbox-layout 的行坐标裁切（裁切边界落在文字行之间），可把多个片段竖向拼接成一张。只裁切/拼接，不改像素。
用法：python a-pdf-shots.py   （读取同目录 SHOTS 配置；已存在的目标文件不覆盖）
片段格式：(PDF 页码, 起始锚点文本 或 None=页首, 结束锚点文本 或 None=页尾(含页码行), include_end_line)
锚点按"行文本小写子串"匹配（该页内第一个命中）。"""
import json, os, re, subprocess, sys, datetime, tempfile, html
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
PDF = os.path.join(HERE, "a-rma-nolla-health.pdf")
IMG = os.path.join(HERE, "..", "images")
PDFTOTEXT = os.environ.get("PDFTOTEXT", "pdftotext")  # poppler 26.09（Git 自带的 xpdf 版没有 -bbox-layout）
DPI = 144
SC = DPI / 72.0  # px per pt

def run(cmd):
    return subprocess.run(cmd, check=True, capture_output=True).stdout

def page_lines(page):
    """返回 [(text, yMin_pt, yMax_pt)]，按 yMin 升序。"""
    out = run([PDFTOTEXT, "-enc", "UTF-8", "-bbox-layout", "-f", str(page), "-l", str(page), PDF, "-"]).decode("utf-8", "replace")
    lines = []
    for m in re.finditer(r"<line xMin=\"([\d.]+)\" yMin=\"([\d.]+)\" xMax=\"([\d.]+)\" yMax=\"([\d.]+)\">(.*?)</line>", out, re.S):
        words = re.findall(r"<word[^>]*>(.*?)</word>", m.group(5), re.S)
        text = html.unescape(" ".join(words))
        lines.append((text, float(m.group(2)), float(m.group(4))))
    lines.sort(key=lambda t: t[1])
    return lines

def render(page, tmp):
    base = os.path.join(tmp, f"pg{page}")
    run(["pdftoppm", "-r", str(DPI), "-f", str(page), "-l", str(page), "-png", PDF, base])
    cands = [f for f in os.listdir(tmp) if f.startswith(f"pg{page}-") and f.endswith(".png")]
    return Image.open(os.path.join(tmp, cands[0])).convert("RGB")

def segment(page, a0, a1, tmp, pad=6):
    im = render(page, tmp)
    L = page_lines(page)
    def find(anchor, start_i=0):
        for i in range(start_i, len(L)):
            if anchor.lower() in L[i][0].lower():
                return i
        raise SystemExit(f"anchor not found on p.{page}: {anchor!r}")
    i0 = 0 if a0 is None else find(a0)
    top = 0 if a0 is None else max(0, int((L[i0][1] - pad) * SC))
    if a1 is None:
        bot = min(im.height, int((L[-1][2] + pad) * SC))  # 页尾：最后一行（页码）之下留白
    else:
        i1 = find(a1, i0)
        bot = min(im.height, int((L[i1][2] + 3) * SC))  # 锚点行之下只留 3pt，避免带入下一行的上半截
    return im.crop((0, top, im.width, bot)), (page, a0, a1, top, bot)

def build(name, segs):
    dst = os.path.join(IMG, name)
    if os.path.exists(dst):
        print("exists, skip:", dst); return
    tmp = tempfile.mkdtemp(prefix="a-pdf-shots-")
    parts, meta = [], []
    for page, a0, a1 in segs:
        im, m = segment(page, a0, a1, tmp)
        parts.append(im); meta.append(m)
    W = max(p.width for p in parts)
    H = sum(p.height for p in parts)
    canvas = Image.new("RGB", (W, H), (255, 255, 255))
    y = 0
    for p in parts:
        canvas.paste(p, (0, y)); y += p.height
    os.makedirs(IMG, exist_ok=True)
    canvas.save(dst)
    rec = {"time_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
           "file": name, "tool": "pdftoppm -r 144 + pdftotext -bbox-layout crop; PIL stitch", "size": canvas.size, "segments": meta}
    with open(os.path.join(HERE, "a-shots-log.jsonl"), "a", encoding="utf-8") as f:
        f.write(json.dumps(rec, ensure_ascii=False) + "\n")
    print(name, canvas.size)

SHOTS = {
    # 01：Section 2 起（2.B Commencement）至 p.2 的 Section 4.B（含两页页码）
    "01-a-rma-sec2-3e-4b.png": [(1, "Section 2. Commencement", None), (2, "E. This agreement is a grant", "been signed by all parties")],
    "02-a-rma-sec19-abc.png": [(11, "Schedule A", None)],
    "03-a-rma-sec19-def.png": [(12, "D. The Division will forgo", None)],
    "04-a-rma-411-stages.png": [(40, "4.11 Stage-Gated", None), (41, "Stage 0 - Current", "cases only.")],
    "05-a-rma-96pct.png": [(40, "Pre-Deployment Performance", "approach.")],
    "06-a-rma-15-insurance.png": [(26, "1.5 Insurance", "regulatory expectations")],
}

if __name__ == "__main__":
    only = sys.argv[1:]
    for n, s in SHOTS.items():
        if only and n not in only:
            continue
        build(n, s)
