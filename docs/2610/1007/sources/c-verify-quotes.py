"""C 组终检：把 evidence-c.md 里反引号内的逐字摘句回查到存档文本。
用法：python c-verify-quotes.py [evidence-c.md]
规则：只比对空白归一化后的原文（不改标点）；html 实体先反解；存档语料 = 本目录 c-*.txt / c-*.md / c-hn-*.json / c-*.json（页面 DOM）。
无需回查的短片段（文件名、<25 字符、数字行等）跳过。输出未命中清单，退出码 = 未命中数。"""
import glob, html, json, os, re, sys

here = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(here, 'evidence-c.md')


def norm(s: str) -> str:
    s = html.unescape(s)
    s = s.replace('​', '').replace(' ', ' ').replace('\xa0', ' ')
    s = re.sub(r'(?m)^\s*(#{1,6}\s+|-\s+)', '', s)   # 去掉本目录 html2txt 加的标题/列表标记
    return re.sub(r'\s+', ' ', s).strip()


corpus = {}
for pat in ('c-*.txt', 'c-*.md', 'c-hn-*.json', 'c-cvp-news.json', 'c-help-cvp-browser.json', 'c-irregular-browser.json'):
    for f in glob.glob(os.path.join(here, pat)):
        name = os.path.basename(f)
        if name.startswith('evidence-c') or name.startswith('capture-log-c'):
            continue
        raw = open(f, encoding='utf-8', errors='replace').read()
        if f.endswith('.json'):
            try:
                raw = json.dumps(json.load(open(f, encoding='utf-8')), ensure_ascii=False)
                raw = raw.replace('\\n', '\n').replace('\\"', '"')
            except Exception:
                pass
        corpus[name] = norm(raw)

text = open(src, encoding='utf-8').read()
spans = re.findall(r'`([^`\n]+)`', text)
bad = []
checked = 0
for sp in spans:
    q = norm(sp)
    if len(q) < 25 or re.search(r'\.(png|html|txt|md|json|pdf|jsonl|mjs|py|tsv)\b', q) or q.startswith('c-') or q.startswith('http'):
        continue
    checked += 1
    hit = [n for n, c in corpus.items() if q in c]
    if not hit:
        bad.append(q)
print(f'spans={len(spans)} checked={checked} missing={len(bad)}')
for q in bad:
    print('MISSING:', q[:200])
sys.exit(len(bad))
