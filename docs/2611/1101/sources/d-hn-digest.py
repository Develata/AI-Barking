import json, re, sys, html
def count(n):
    return sum(1 + count(c) for c in n.get('children', []))
def strip(t):
    t = html.unescape(re.sub(r'<[^>]+>', ' ', t or ''))
    return re.sub(r'\s+', ' ', t).strip()
path, topn = sys.argv[1], int(sys.argv[2])
d = json.load(open(path, encoding='utf-8'))
print('story', d['id'], d.get('points'), 'comments_total', count(d), d.get('title'))
tops = sorted(d.get('children', []), key=lambda n: -count(n))[:topn]
for n in tops:
    print(f"- id={n['id']} replies={count(n)} time={n['created_at'][:16]}Z :: {strip(n.get('text'))[:420]}")
