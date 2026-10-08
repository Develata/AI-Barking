"""1007 Haiku 补充网页截图（改自 a-shot.py）：headless Chrome 按 700 CSS px 宽、2 倍像素比渲染整窗；用 Tesseract 找锚点行；只裁切，不改像素。
裁切边界落在文字行之间（锚点行框外扩 pad_top / pad_bot 像素，2x 像素）。记录追加到 g-shots-log.jsonl。
用法：
  python a-shot.py render <url> <name> <窗口高度CSS> [<额外Chrome参数>...]   # 渲染到 scratchpad/<name>.png（供人工查看）
  python a-shot.py crop <name> <out.png> "<起始锚点>" "<结束锚点>" [pad_top] [pad_bot] [<url>]  # 用 scratchpad/<name>.png 裁切到 images/<out.png>
  python a-shot.py cropbox <name> <out.png> <x0> <y0> <x1> <y1> [<url>]   # 直接按像素框裁切（锚点不可用时）
说明：窗口高度太小页面会被截断，太大则整窗留白；先 render 看一眼再 crop。"""
import json, subprocess, sys, tempfile, os, datetime, csv, io
from PIL import Image

CHROME = os.environ.get("CHROME", r"C:\Program Files\Google\Chrome\Application\chrome.exe")
SCRATCH = os.environ.get("G_SCRATCH", tempfile.gettempdir())  # 临时渲染目录（本地，不入库）
HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "..", "images")

def now_bj():
    return (datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).isoformat(timespec="seconds") + "+08:00"

def log(rec):
    with open(os.path.join(HERE, "g-shots-log.jsonl"), "a", encoding="utf-8") as f:
        f.write(json.dumps(rec, ensure_ascii=False) + "\n")

def render(url, name, height, extra):
    tmp = tempfile.mkdtemp(prefix="a-shot-")
    dst = os.path.join(SCRATCH, name + ".png")
    cmd = [CHROME, "--headless=new", "--user-data-dir=" + os.path.join(tmp, "prof"), "--disable-gpu", "--hide-scrollbars",
           "--force-device-scale-factor=2", f"--window-size=700,{height}", "--timeout=30000", "--lang=en-US"] + extra + [f"--screenshot={dst}", url]
    subprocess.run(cmd, check=True, capture_output=True, timeout=240)
    im = Image.open(dst)
    print(dst, im.size, now_bj())
    log({"time_bj": now_bj(), "action": "render", "url": url, "name": name, "tool": "headless Chrome 700 CSS px, DPR 2", "size": im.size, "extra": extra})

def ocr_lines(png):
    tsv = subprocess.run(["tesseract", png, "stdout", "--psm", "3", "-c", "tessedit_create_tsv=1"],
                         capture_output=True, check=True).stdout.decode("utf-8", "replace")
    rows = list(csv.DictReader(io.StringIO(tsv), delimiter="\t", quoting=csv.QUOTE_NONE))
    lines = {}
    for r in rows:
        if r["level"] == "5" and r["text"].strip():
            k = (r["block_num"], r["par_num"], r["line_num"])
            L = lines.setdefault(k, {"words": [], "top": 10**9, "bot": 0})
            L["words"].append(r["text"])
            L["top"] = min(L["top"], int(r["top"]))
            L["bot"] = max(L["bot"], int(r["top"]) + int(r["height"]))
    return sorted(lines.values(), key=lambda L: L["top"])

def save(im, box, out, meta):
    os.makedirs(IMG, exist_ok=True)
    dst = os.path.join(IMG, out)
    if os.path.exists(dst):
        raise SystemExit(f"exists: {dst}")
    im.crop(box).save(dst)
    meta.update({"time_bj": now_bj(), "action": "crop", "file": out, "box": box, "full_size": im.size})
    log(meta)
    print(json.dumps(meta, ensure_ascii=False))

if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "render":
        render(sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5:])
    elif mode == "crop":
        name, out, a0, a1 = sys.argv[2:6]
        pt = int(sys.argv[6]) if len(sys.argv) > 6 else 24
        pb = int(sys.argv[7]) if len(sys.argv) > 7 else 24
        url = sys.argv[8] if len(sys.argv) > 8 else ""
        png = os.path.join(SCRATCH, name + ".png")
        seq = ocr_lines(png)
        def find(anchor, start=0):
            for L in seq:
                if L["top"] >= start and anchor.lower() in " ".join(L["words"]).lower():
                    return L
            raise SystemExit(f"anchor not found: {anchor}")
        s = find(a0); e = find(a1, s["top"])
        im = Image.open(png)
        box = (0, max(0, s["top"] - pt), im.width, min(im.height, e["bot"] + pb))
        save(im, box, out, {"url": url, "name": name, "anchors": [a0, a1], "tool": "headless Chrome 700 CSS px, DPR 2; Tesseract anchor crop"})
    elif mode == "stack":
        # python a-shot.py stack <name> <out.png> <url> "<a0>::<a1>" "<a0>::<a1>" ...   纵向拼接多个锚点区间（每段保持原像素；段间不加分隔线）
        name, out, url = sys.argv[2:5]
        pairs = [x.split("::") for x in sys.argv[5:]]
        png = os.path.join(SCRATCH, name + ".png")
        seq = ocr_lines(png)
        im = Image.open(png)
        def find(anchor, start=0):
            for L in seq:
                if L["top"] >= start and anchor.lower() in " ".join(L["words"]).lower():
                    return L
            raise SystemExit(f"anchor not found: {anchor}")
        parts, boxes = [], []
        for a0, a1 in pairs:
            s0 = find(a0); e0 = find(a1, s0["top"])
            box = (0, max(0, s0["top"] - 24), im.width, min(im.height, e0["bot"] + 24))
            boxes.append(box); parts.append(im.crop(box))
        H = sum(p_.height for p_ in parts)
        canvas = Image.new("RGB", (im.width, H), (255, 255, 255))
        y = 0
        for p_ in parts:
            canvas.paste(p_, (0, y)); y += p_.height
        os.makedirs(IMG, exist_ok=True)
        dst = os.path.join(IMG, out)
        if os.path.exists(dst):
            raise SystemExit(f"exists: {dst}")
        canvas.save(dst)
        meta = {"time_bj": now_bj(), "action": "stack", "file": out, "url": url, "name": name, "anchors": pairs, "boxes": boxes,
                "size": canvas.size, "tool": "headless Chrome 700 CSS px, DPR 2; Tesseract anchor crop; vertical stack"}
        log(meta); print(json.dumps(meta, ensure_ascii=False))
    elif mode == "cropbox":
        name, out, x0, y0, x1, y1 = sys.argv[2:8]
        url = sys.argv[8] if len(sys.argv) > 8 else ""
        im = Image.open(os.path.join(SCRATCH, name + ".png"))
        save(im, (int(x0), int(y0), int(x1), int(y1)), out, {"url": url, "name": name, "tool": "headless Chrome 700 CSS px, DPR 2; pixel-box crop"})
    else:
        raise SystemExit(__doc__)
