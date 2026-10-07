"""1006 期截图：headless Chrome 按 700 CSS px 宽、2 倍像素比截整窗，再用 Tesseract 找锚点行裁切。
用法：python m-shot.py <url> <out.png> <窗口高度CSS> "<起始锚点>" "<结束锚点>" [上边距px] [下边距px]
只裁切，不改像素；裁切边界落在文字行之间（锚点行框外扩边距）。记录写入 m-shots.jsonl。"""
import json, subprocess, sys, tempfile, os, datetime, csv, io
from PIL import Image

CHROME = r"C:\Program Files\Google\Chrome\Application\chrome.exe"
url, out, height, a0, a1 = sys.argv[1:6]
pad_top = int(sys.argv[6]) if len(sys.argv) > 6 else 40
pad_bot = int(sys.argv[7]) if len(sys.argv) > 7 else 40
here = os.path.dirname(os.path.abspath(__file__))
img_dir = os.path.join(here, "..", "images")
tmp = tempfile.mkdtemp(prefix="m-shot-")
full = os.path.join(tmp, "full.png")
subprocess.run([CHROME, "--headless=new", f"--user-data-dir={tmp}\\prof", "--disable-gpu", "--hide-scrollbars",
                "--force-device-scale-factor=2", f"--window-size=700,{height}", "--timeout=30000",
                "--lang=en-US", f"--screenshot={full}", url], check=True, capture_output=True, timeout=180)
tsv = subprocess.run(["tesseract", full, "stdout", "--psm", "3", "-c", "tessedit_create_tsv=1"],
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
seq = sorted(lines.values(), key=lambda L: L["top"])
def find(anchor, start=0):
    for i, L in enumerate(seq):
        if L["top"] >= start and anchor.lower() in " ".join(L["words"]).lower():
            return L
    raise SystemExit(f"anchor not found: {anchor}")
s = find(a0)
e = find(a1, s["top"])
im = Image.open(full)
box = (0, max(0, s["top"] - pad_top), im.width, min(im.height, e["bot"] + pad_bot))
os.makedirs(img_dir, exist_ok=True)
dst = os.path.join(img_dir, out)
if os.path.exists(dst):
    raise SystemExit(f"exists: {dst}")
im.crop(box).save(dst)
rec = {"time_bj": (datetime.datetime.utcnow() + datetime.timedelta(hours=8)).isoformat(timespec="seconds") + "+08:00",
       "url": url, "file": out, "tool": "headless Chrome 700 CSS px, DPR 2; Tesseract anchor crop",
       "anchors": [a0, a1], "box": box, "full_size": im.size}
with open(os.path.join(here, "m-shots.jsonl"), "a", encoding="utf-8") as f:
    f.write(json.dumps(rec, ensure_ascii=False) + "\n")
print(json.dumps(rec, ensure_ascii=False))
