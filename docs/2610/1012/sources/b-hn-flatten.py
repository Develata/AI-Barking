"""1013 B 组（改自 1012 c-hn-flatten.py）：把 HN Algolia items JSON 展平成文本（作者、时间、ID、正文去标签）。用法：python c-hn-flatten.py in.json out.txt"""
import json, sys, re, html
d = json.load(open(sys.argv[1], encoding="utf-8"))
out = []
def clean(t):
    t = re.sub(r"<p>", "\n", t or ""); t = re.sub(r'<a [^>]*href="([^"]*)"[^>]*>.*?</a>', r"\1", t); t = re.sub(r"<[^>]+>", "", t)
    return html.unescape(t)
def walk(n, depth):
    out.append(f"{'  '*depth}[{n.get('id')}] {n.get('author')} {n.get('created_at')}\n{'  '*depth}{clean(n.get('text') or n.get('title') or '')}\n")
    for c in n.get("children", []): walk(c, depth + 1)
out.append(f"TITLE: {d.get('title')} | URL: {d.get('url')} | points {d.get('points')} | id {d.get('id')} | {d.get('created_at')} | author {d.get('author')}\n")
for c in d.get("children", []): walk(c, 0)
open(sys.argv[2], "w", encoding="utf-8").write("\n".join(out))
