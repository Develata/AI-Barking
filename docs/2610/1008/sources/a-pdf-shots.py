"""1008 A 组：从 PDF 渲染页面截图（pdftoppm 144 dpi = 2x 像素比，宽≈1224 px，≤1400 px），按 pdftotext -bbox-layout 的行坐标裁切
（裁切边界落在文字行之间），可把多个片段竖向拼接成一张。只裁切/拼接，不改像素。参考 docs/2610/1007/sources/a-pdf-shots.py，扩展为多 PDF。
用法：python a-pdf-shots.py lines <pdf> <page>          # 列出该页各行文本及 y 坐标（找锚点用）
      python a-pdf-shots.py build [目标文件名...]       # 生成 SHOTS 中的截图；已存在的不覆盖
片段 (pdf, 页码, 起始锚点 或 None=页首, 结束锚点 或 None=页尾, 起始行是否从锚点行上沿, 结束是否含锚点行)；锚点按行文本小写子串匹配（页内第一个命中），
可写 (锚点, n) 取第 n 个命中（1 起）。"""
import json, os, re, subprocess, sys, datetime, tempfile, html
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "..", "images")
BIN = os.path.expanduser("~/scoop/apps/poppler/current/bin")
PDFTOTEXT = os.path.join(BIN, "pdftotext.exe"); PDFTOPPM = os.path.join(BIN, "pdftoppm.exe")
DPI = 144; SC = DPI / 72.0
PDFS = {"crit": os.path.join(HERE, "a-ns-lean-critique-v1.pdf"), "oai": os.path.join(HERE, "a-oai-ns-paper.pdf")}

def run(cmd): return subprocess.run(cmd, check=True, capture_output=True).stdout

def page_lines(pdf, page):
    out = run([PDFTOTEXT, "-enc", "UTF-8", "-bbox-layout", "-f", str(page), "-l", str(page), pdf, "-"]).decode("utf-8", "replace")
    lines = []
    for m in re.finditer(r"<line xMin=\"([\d.]+)\" yMin=\"([\d.]+)\" xMax=\"([\d.]+)\" yMax=\"([\d.]+)\">(.*?)</line>", out, re.S):
        words = re.findall(r"<word[^>]*>(.*?)</word>", m.group(5), re.S)
        lines.append((html.unescape(" ".join(words)), float(m.group(2)), float(m.group(4))))
    lines.sort(key=lambda t: t[1]); return lines

def render(pdf, page, tmp):
    base = os.path.join(tmp, f"pg{page}")
    run([PDFTOPPM, "-r", str(DPI), "-f", str(page), "-l", str(page), "-png", pdf, base])
    c = [f for f in os.listdir(tmp) if f.startswith(f"pg{page}-") and f.endswith(".png")]
    return Image.open(os.path.join(tmp, c[0])).convert("RGB")

def segment(pdfkey, page, a0, a1, tmp, pad=1, end_inclusive=True):
    pdf = PDFS[pdfkey]; im = render(pdf, page, tmp); L = page_lines(pdf, page)
    def find(anchor, start=0):
        n = 1
        if isinstance(anchor, tuple): anchor, n = anchor
        seen = 0
        for i in range(start, len(L)):
            if anchor.lower() in L[i][0].lower():
                seen += 1
                if seen == n: return i
        raise SystemExit(f"anchor not found on p.{page}: {anchor!r}")
    i0 = 0 if a0 is None else find(a0)
    def mid_before(i):  # 第 i 行与上一行之间的中点（pt）；没有上一行则取本行上沿减 4pt
        return (L[i-1][2] + L[i][1]) / 2 if i > 0 and L[i-1][2] < L[i][1] else L[i][1] - 4
    top = 0 if a0 is None else max(0, int(mid_before(i0) * SC))
    if a1 is None: bot = min(im.height, int((L[-1][2] + pad) * SC))
    else:
        i1 = find(a1, i0)
        bot = min(im.height, int((L[i1][2] + 3) * SC)) if end_inclusive else max(top + 1, int(mid_before(i1) * SC))
    return im.crop((0, top, im.width, bot)), (pdfkey, page, str(a0), str(a1), top, bot)

def build(name, segs):
    dst = os.path.join(IMG, name)
    if os.path.exists(dst): print("exists, skip:", dst); return
    tmp = tempfile.mkdtemp(prefix="a-pdf-shots-"); parts, meta = [], []
    for s in segs:
        pk, page, a0, a1 = s[:4]; inc = s[4] if len(s) > 4 else True
        im, m = segment(pk, page, a0, a1, tmp, end_inclusive=inc); parts.append(im); meta.append(m)
    W = max(p.width for p in parts); H = sum(p.height for p in parts)
    canvas = Image.new("RGB", (W, H), (255, 255, 255)); y = 0
    for p in parts: canvas.paste(p, (0, y)); y += p.height
    os.makedirs(IMG, exist_ok=True); canvas.save(dst)
    rec = {"time_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"), "file": name,
           "tool": "pdftoppm -r 144 + pdftotext -bbox-layout crop; PIL stitch", "size": canvas.size, "segments": meta}
    open(os.path.join(HERE, "a-shots-log.jsonl"), "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
    print(name, canvas.size)

SHOTS = {}
exec(open(os.path.join(HERE, "a-pdf-shots-config.py"), encoding="utf-8").read()) if os.path.exists(os.path.join(HERE, "a-pdf-shots-config.py")) else None

if __name__ == "__main__":
    if sys.argv[1] == "lines":
        for t, y0, y1 in page_lines(PDFS[sys.argv[2]], int(sys.argv[3])): print(f"{y0:7.1f} {y1:7.1f}  {t}")
    else:
        only = sys.argv[2:]
        for n, s in SHOTS.items():
            if only and n not in only: continue
            build(n, s)
