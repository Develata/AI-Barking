# Reverse check: every quoted span (>= 20 chars) in evidence-d.md must occur in some d-* archive text.
# usage: python3 d-verify-evidence.py   (run from docs/2610/1007/sources/)
import re, html, glob, os

def norm(t):
    t = html.unescape(t)
    for a, b in (('ﬃ', 'ffi'), ('ﬀ', 'ff'), ('ﬁ', 'fi'), ('ﬂ', 'fl'), ('ﬄ', 'ffl')):
        t = t.replace(a, b)
    t = t.replace('\\u2019', '’').replace('\\"', '"').replace('\\n', ' ')
    return re.sub(r'\s+', ' ', t)

corpus = []
for f in glob.glob('d-*'):
    if os.path.isdir(f):
        continue
    if f.endswith(('.pdf', '.py', '.mjs', '.sh', '.jsonl')) or f.startswith(('d-verify', 'd-html2txt', 'd-pagefind')):
        continue
    raw = open(f, encoding='utf8', errors='replace').read()
    if f.endswith('.html'):
        raw = re.sub(r'<script[\s\S]*?</script>|<style[\s\S]*?</style>', ' ', raw)
        raw = re.sub(r'<[^>]+>', ' ', raw)
    corpus.append(norm(raw))
big = '\n'.join(corpus)
big2 = big.replace('- ', '-')

ev = open('evidence-d.md', encoding='utf8').read()
spans = []
spans += re.findall(r'“([^”]{20,}?)”', ev)
spans += re.findall(r'(?<![\w“])"([^"]{20,}?)"', ev)
seen = set(); miss = 0
for s in spans:
    s2 = norm(s).strip(' .,;')
    if s2 in seen:
        continue
    seen.add(s2)
    # treat ellipsis: check each fragment
    frags = [x.strip(' .,;') for x in re.split(r'…|\.\.\.', s2) if len(x.strip()) >= 20]
    ok = all((fr in big) or (fr.replace('- ', '-') in big2) for fr in frags) if frags else True
    if not ok:
        miss += 1
        print('NOT FOUND:', s2[:160])
print('spans', len(seen), 'not found', miss)
