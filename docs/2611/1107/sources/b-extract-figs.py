# B 组：从 doubleai 页面 HTML 抽出每个 Fig. 的 iframe srcdoc（图表数据内嵌其中），
# 写 b-doubleai-figNN.srcdoc.html，并把含 JSON/数组的 const 行摘到 b-doubleai-figdata.txt。
import re, html, sys, pathlib
src = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
out = pathlib.Path(sys.argv[2]); out.mkdir(exist_ok=True)
# 先按 <figure> 切，取每个 figure 内的 srcdoc、title、figcaption
figs = re.findall(r'<figure.*?</figure>', src, flags=re.S)
summary = []
for f in figs:
    m = re.search(r'srcDoc="(.*?)" title="(.*?)"', f, flags=re.S) or re.search(r'srcdoc="(.*?)" title="(.*?)"', f, flags=re.S)
    if not m: continue
    doc = html.unescape(m.group(1)); title = m.group(2)
    n = re.search(r'(\d+)', title).group(1)
    (out / f"b-doubleai-fig{n}.srcdoc.html").write_text(doc, encoding="utf-8")
    cap = re.search(r'<figcaption.*?</figcaption>', f, flags=re.S)
    cap = html.unescape(re.sub(r'<[^>]+>', ' ', cap.group(0))) if cap else ''
    consts = re.findall(r'^\s*(?:const|let|var)\s+[A-Za-z_]\w*\s*=\s*[\[{].*$', doc, flags=re.M)
    summary.append((n, title, len(doc), re.sub(r'\s+', ' ', cap).strip(), consts))
with open(out / "b-doubleai-figdata.txt", "w", encoding="utf-8") as w:
    for n, title, ln, cap, consts in summary:
        w.write(f"=== {title} (srcdoc {ln} chars) ===\nCAPTION: {cap}\n")
        for c in consts: w.write(c.strip() + "\n")
        w.write("\n")
print(len(summary), "figures")
