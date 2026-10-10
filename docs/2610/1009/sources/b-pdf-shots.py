"""1009 B 组：从 PDF 渲染并裁切截图（只裁切/竖向拼接，不改像素）。参考 1008 a-pdf-shots.py。
裁切边界按 pdftotext -bbox-layout 的行坐标落在两行文字之间。poppler 取自 Scoop。
用法：python -I b-pdf-shots.py lines <pdf> <page>      列出该页各行与 y(pt)
      python -I b-pdf-shots.py build                  生成 SHOTS 中尚不存在的图（不覆盖）
片段： (pdf键, 页, 起始锚点|None=页首, 结束锚点|None=页尾, dpi)；锚点=行文本小写子串，(锚点, n) 取第 n 个命中；结束行含在内。"""
import os, re, subprocess, sys, tempfile, html, json, datetime
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "..", "images")
BIN = os.path.expanduser("~/scoop/apps/poppler/current/bin")
PDFTOTEXT = os.path.join(BIN, "pdftotext.exe"); PDFTOPPM = os.path.join(BIN, "pdftoppm.exe")
PDFS = {
    "letter": os.path.join(HERE, "b-letter.pdf"),
    "d1463": os.path.join(HERE, "b-dkt1463-auction-results.pdf"),
    "d1684": os.path.join(HERE, "b-dkt-ombudsman-oct5.pdf"),
    "d1594": os.path.join(HERE, "b-dkt1594-google-prelim-response.pdf"),
    "d1489": os.path.join(HERE, "b-dkt1489-afa-objection.pdf"),
}

def run(cmd): return subprocess.run(cmd, check=True, capture_output=True).stdout

def page_lines(pdf, page):
    out = run([PDFTOTEXT, "-enc", "UTF-8", "-bbox-layout", "-f", str(page), "-l", str(page), pdf, "-"]).decode("utf-8", "replace")
    L = []
    for m in re.finditer(r"<line xMin=\"([\d.]+)\" yMin=\"([\d.]+)\" xMax=\"([\d.]+)\" yMax=\"([\d.]+)\">(.*?)</line>", out, re.S):
        words = re.findall(r"<word[^>]*>(.*?)</word>", m.group(5), re.S)
        L.append((html.unescape(" ".join(words)), float(m.group(2)), float(m.group(4)), float(m.group(1)), float(m.group(3))))
    L.sort(key=lambda t: t[1]); return L

def render(pdf, page, dpi, tmp):
    base = os.path.join(tmp, f"pg{page}_{dpi}")
    run([PDFTOPPM, "-r", str(dpi), "-f", str(page), "-l", str(page), "-png", pdf, base])
    c = [f for f in os.listdir(tmp) if f.startswith(f"pg{page}_{dpi}-") and f.endswith(".png")]
    return Image.open(os.path.join(tmp, c[0])).convert("RGB")

def segment(key, page, a0, a1, dpi, tmp):
    pdf = PDFS[key]; sc = dpi / 72.0; im = render(pdf, page, dpi, tmp); L = page_lines(pdf, page)
    def find(anchor, start=0):
        n = 1
        if isinstance(anchor, tuple): anchor, n = anchor
        seen = 0
        for i in range(start, len(L)):
            if anchor.lower() in L[i][0].lower():
                seen += 1
                if seen == n: return i
        raise SystemExit(f"anchor not found p.{page}: {anchor!r}")
    i0 = 0 if a0 is None else find(a0)
    def mid_before(i): return (L[i-1][2] + L[i][1]) / 2 if i > 0 and L[i-1][2] < L[i][1] else L[i][1] - 4
    top = 0 if a0 is None else max(0, int(mid_before(i0) * sc))
    if a1 is None: bot = im.height
    else:
        i1 = find(a1, i0); bot = min(im.height, int((L[i1][2] + 4) * sc))
    return im.crop((0, top, im.width, bot)), (key, page, str(a0), str(a1), dpi, top, bot)

def build(name, segs, cropx=None):
    dst = os.path.join(IMG, name)
    if os.path.exists(dst): print("exists, skip:", dst); return
    tmp = tempfile.mkdtemp(prefix="b-pdf-shots-"); parts, meta = [], []
    for s in segs:
        im, m = segment(*s, tmp); parts.append(im); meta.append(m)
    w = max(p.width for p in parts)
    canvas = Image.new("RGB", (w, sum(p.height for p in parts)), "white"); y = 0
    for p in parts: canvas.paste(p, (0, y)); y += p.height
    if cropx: canvas = canvas.crop((cropx[0], 0, min(cropx[1], canvas.width), canvas.height))
    assert canvas.width <= 1400, canvas.width
    os.makedirs(IMG, exist_ok=True); canvas.save(dst)
    rec = {"time_bj": (datetime.datetime.utcnow() + datetime.timedelta(hours=8)).isoformat(timespec="seconds") + "+08:00",
           "file": name, "size": canvas.size, "segments": meta, "cropx": cropx}
    open(os.path.join(HERE, "b-shots-log.jsonl"), "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
    print("saved", dst, canvas.size)

SHOTS = {
    # 15 议员信数字段：日期/收件人/首段/“100 million emails, 500 million Microsoft Teams messages”段
    "15-letter-numbers.png": [("letter", 1, None, "than about its customers", 144)],
    # 16 诉求段：第 1 页“That is why”到第 2 页末六条诉求
    "16-letter-demands.png": [("letter", 1, "That is why, before any", "simply because it is technically capable", 144), ("letter", 2, "2. Establish a meaningful", "protections keep pace with the technology", 144)],
    # 18 监察员补充报告结论与脚注 3
    "18-ombudsman-conclusion.png": [("d1684", 3, None, "deidentification process to be followed", 144)],
    # 17 Dkt 1463 附件 A 标的清单第 18 页：表头 + “Team Member”“Documents”两块（含 Time Card、Payroll、Emails、Teams），含最右侧 Google 采购请求列
    "17-dkt1463-asset-schedule.png": ([("d1463", 18, "Category / Description", "Category / Description", 230),
                                       ("d1463", 18, "Team Member", "Corporate Tax Documents", 230)], (172, 1546)),
    # 19 同一页整页（上下文；字小）
    "19-dkt1463-p18-full.png": [("d1463", 18, None, None, 144)],
    # 20 Dkt 1463 首页拍卖结果通知 + 第 2 页中标/备选金额
    "20-dkt1463-auction-result.png": [("d1463", 1, None, "Alternate Bidder).", 144), ("d1463", 2, "Successful Bidder", "$7,500,000", 144)],
    # 22 Dkt 1594（Google 9/9 初步回应）第 3 页：事先排除的五个数据集（含 Timecard Information）
    "22-dkt1594-excluded-datasets.png": [("d1594", 3, None, "Pg 3 of 4", 144), ("d1594", 3, "in an abundance of caution", "a means of excluding datasets", 144)],
    # 23 Dkt 1489（AFA 8/18 有限异议）第 1 页：1 亿邮件 / 5 亿 Teams 条目的原始口径（AFA 对标的清单的描述）
    "23-dkt1489-afa-numbers.png": [("d1489", 1, None, "consumer.", 144)],
    # 24 Dkt 1463 附件 A 第 9 页 §3(c)：去标识化由代理人向买方“reasonable satisfaction”证明、采 CCPA 标准、“preserving referential integrity”
    "24-dkt1463-deid-clause.png": [("d1463", 9, None, "Pg 9 of 35", 144), ("d1463", 9, "deidentification agent (or the seller", "referential integrity", 144)],
}

if __name__ == "__main__":
    if sys.argv[1] == "lines":
        for t, a, b, x0, x1 in page_lines(PDFS.get(sys.argv[2], sys.argv[2]), int(sys.argv[3])): print(f"{a:7.1f} {b:7.1f} {x0:6.1f}-{x1:6.1f}  {t}")
    elif sys.argv[1] == "build":
        for n, s in SHOTS.items():
            if isinstance(s, tuple): build(n, s[0], s[1])
            else: build(n, s)
