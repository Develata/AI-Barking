"""1008 A 组：把渲染图中的多个像素框竖向拼接成一张（只裁切拼接，不改像素），记录到 a-shots-log.jsonl。
用法：python a-stack.py <渲染名> <out.png> <url> x0,y0,x1,y1 [x0,y0,x1,y1 ...]   （渲染图在 A_SCRATCH）"""
import sys, os, json, datetime
from PIL import Image
name, out, url = sys.argv[1:4]; boxes = [tuple(int(v) for v in b.split(",")) for b in sys.argv[4:]]
HERE = os.path.dirname(os.path.abspath(__file__)); IMG = os.path.join(HERE, "..", "images")
im = Image.open(os.path.join(os.environ["A_SCRATCH"], name + ".png")).convert("RGB")
parts = [im.crop(b) for b in boxes]; W = max(p.width for p in parts); H = sum(p.height for p in parts)
c = Image.new("RGB", (W, H), (255, 255, 255)); y = 0
for p in parts: c.paste(p, (0, y)); y += p.height
dst = os.path.join(IMG, out)
if os.path.exists(dst): raise SystemExit("exists: " + dst)
c.save(dst)
t = (datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).isoformat(timespec="seconds") + "+08:00"
open(os.path.join(HERE, "a-shots-log.jsonl"), "a", encoding="utf-8").write(json.dumps({"time_bj": t, "action": "stack-boxes", "file": out, "url": url, "name": name, "boxes": boxes, "size": c.size}, ensure_ascii=False) + "\n")
print(out, c.size)
