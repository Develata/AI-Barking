"""B 组：把起诉书 PDF 的某页渲染成图，按文字锚点（tesseract 行）裁切，并对当事人姓名做黑条遮盖（OCR 词框），最后缩到宽 <=1400 px。
用法（Windows scoop 的 python 缺 Pillow 编译件，用 uv run --no-project --with pillow python）：python b-pdfshot.py <pdf> <page> <out.png> <start_text|top> <end_text|bottom> [pad_px]   → 单页裁切（输出为 PNG，不入库）
      python -I b-pdfshot.py --stack out.png part1.png part2.png ...                          → 竖向拼接
裁切只裁、不改像素，唯一的像素改动是遮盖姓名的黑条（词框来自 OCR，输出后须目视复核）。
依赖：pdftoppm、tesseract（均在 PATH）、Pillow。"""
import sys, subprocess, tempfile, os, re, csv, io
from PIL import Image, ImageDraw
NAME = re.compile("al" + "fozan|abdul" + "rahman|sau" + "di", re.I)   # 姓名与国籍词拆开写，避免脚本本身把姓名入库
if sys.argv[1] == "--blackbox":   # 补遮 OCR 漏掉的姓名：--blackbox img.png x0 y0 x1 y1（成图坐标）
    img = Image.open(sys.argv[2]).convert("RGB"); x0, y0, x1, y1 = map(int, sys.argv[3:7])
    ImageDraw.Draw(img).rectangle([x0, y0, x1, y1], fill="black"); img.save(sys.argv[2]); sys.exit()
if sys.argv[1] == "--stack":
    out, parts = sys.argv[2], [Image.open(p).convert("RGB") for p in sys.argv[3:]]
    w = max(p.width for p in parts); h = sum(p.height for p in parts) + 6*(len(parts)-1)
    im = Image.new("RGB", (w, h), "white"); y = 0
    for p in parts: im.paste(p, (0, y)); y += p.height + 6
    im.save(out); sys.exit()
pdf, page, out, a, b = sys.argv[1:6]; pad = int(sys.argv[6]) if len(sys.argv) > 6 else 14
tmp = tempfile.mkdtemp()
subprocess.run(["pdftoppm", "-r", "326", "-png", "-f", page, "-l", page, pdf, os.path.join(tmp, "p")], check=True)
png = [f for f in os.listdir(tmp) if f.endswith(".png")][0]; png = os.path.join(tmp, png)
im = Image.open(png).convert("RGB")
base = os.path.join(tmp, "ocr")
subprocess.run(["tesseract", png, base, "-l", "eng", "--psm", "6", "-c", "tessedit_create_tsv=1"], check=True, capture_output=True)
tsv = open(base + ".tsv", encoding="utf-8").read()
rows = list(csv.DictReader(io.StringIO(tsv), delimiter="\t", quoting=csv.QUOTE_NONE))
words = [r for r in rows if r["level"] == "5" and r["text"].strip()]
d = ImageDraw.Draw(im)
for r in words:
    if NAME.search(r["text"]):
        x, y, w, h = (int(r[k]) for k in ("left", "top", "width", "height"))
        d.rectangle([x-4, y-4, x+w+4, y+h+4], fill="black")
# 行：按 (block,par,line) 聚合，取文本与顶/底
lines = {}
for r in rows:
    if r["level"] != "5" or not r["text"].strip(): continue
    k = (r["block_num"], r["par_num"], r["line_num"]); lines.setdefault(k, []).append(r)
L = []
for k, ws in lines.items():
    L.append((min(int(w["top"]) for w in ws), max(int(w["top"])+int(w["height"]) for w in ws), " ".join(w["text"] for w in ws)))
L.sort()
def find(t, last=False):
    hit = [l for l in L if t.lower() in l[2].lower()]
    if not hit: raise SystemExit("anchor missing: " + t + "\n" + "\n".join(l[2] for l in L))
    return hit[-1] if last else hit[0]
top = 0 if a == "top" else max(0, find(a)[0] - pad)
if b == "bottom": bot = im.height
elif b.startswith("^"): bot = max(top, find(b[1:])[0] - int(os.environ.get("OFF", "200")))   # 在该行之前结束（留出图下空白）
else: bot = min(im.height, find(b, last=True)[1] + pad)
crop = im.crop((0, top, im.width, bot))
# 左右裁掉空白边
bbox = Image.eval(crop.convert("L"), lambda v: 255 if v < 250 else 0).getbbox()
if bbox: crop = crop.crop((max(0, bbox[0]-40), 0, min(crop.width, bbox[2]+40), crop.height))
if crop.width > 1400:
    s = 1400 / crop.width; crop = crop.resize((1400, round(crop.height*s)), Image.LANCZOS)
crop.save(out); print(out, crop.size)
