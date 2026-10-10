"""1008 A 组：把 Lean 文件指定行区间按等宽字体本地渲染成图（GitHub 渲染大文件超时空白，改用本地渲染；文本逐字取自 `git show <commit>:<path>`，不改字符）。
用法：python a-lean-render.py <lean源文件> <起行> <止行> <标题行> <out.png>   （headless Chrome 700 CSS px 宽，DPR 2）"""
import sys, html, subprocess, os, tempfile, json, datetime
from PIL import Image
src, a, b, title, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
lines = open(src, encoding="utf-8").read().split("\n")[a-1:b]
rows = "\n".join(f'<span class="n">{i:>5}</span>  {html.escape(l)}' for i, l in enumerate(lines, a))
page = f"""<!doctype html><meta charset=utf-8><style>
body{{margin:0;background:#fff;font-family:Consolas,'Cascadia Mono',monospace}}
.h{{background:#f6f8fa;border-bottom:1px solid #d0d7de;padding:10px 12px;font:13px/1.4 'Segoe UI',Arial,sans-serif;color:#1f2328}}
pre{{margin:0;padding:10px 12px;font-size:10.6px;line-height:1.55;color:#1f2328;white-space:pre}}
.n{{color:#6e7781}}</style>
<div class=h>{html.escape(title)}</div><pre>{rows}</pre>"""
tmp = tempfile.mkdtemp(prefix="a-lean-"); f = os.path.join(tmp, "p.html"); open(f, "w", encoding="utf-8").write(page)
H = int(10.6 * 1.55 * (len(lines) + 1) + 90)
png = os.path.join(tmp, "o.png")
subprocess.run([r"C:\Program Files\Google\Chrome\Application\chrome.exe", "--headless=new", "--user-data-dir=" + os.path.join(tmp, "prof"), "--disable-gpu", "--hide-scrollbars",
                "--force-device-scale-factor=2", f"--window-size=700,{H}", "--timeout=20000", f"--screenshot={png}", "file:///" + f.replace("\\", "/")], check=True, capture_output=True, timeout=120)
dst = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "images", out)
if os.path.exists(dst): raise SystemExit("exists")
Image.open(png).convert("RGB").save(dst); print(out, Image.open(dst).size)
t = (datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).isoformat(timespec="seconds") + "+08:00"
open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "a-shots-log.jsonl"), "a", encoding="utf-8").write(json.dumps({"time_bj": t, "action": "local-mono-render", "file": out, "source": src, "lines": [a, b], "title": title}, ensure_ascii=False) + "\n")
