# a-aa-extract.py: pull per-model figures out of the Next.js payload embedded in saved Artificial Analysis pages.
# usage: python3 -I a-aa-extract.py <page.html> [...]  -> TSV on stdout (union over pages, de-duplicated by slug)
import re, json, sys
dec = json.JSONDecoder()
seen = {}
def num(p, seg):
    m = re.search(r'"%s":(-?[0-9.eE+-]+)' % p, seg)
    return float(m.group(1)) if m else None
for path in sys.argv[1:]:
    h = open(path, encoding='utf8', errors='ignore').read().replace('\\"', '"')
    for m in re.finditer(r'"intelligenceIndexEvaluations":', h):
        i = m.start()
        s = h.rfind('{"id":"', 0, i)
        s = h.rfind('{"id":"', 0, s) if h.find('"slug"', s, s + 80) < 0 else s
        seg = h[s:i]
        slug = re.search(r'"slug":"([^"]+)"', seg).group(1)
        short = re.search(r'"shortName":"([^"]+)"', seg)
        name = re.search(r'"name":"([^"]+)"', seg).group(1)
        eff = re.search(r'"effort":\{"slug":"([^"]*)"', seg)
        evs, _ = dec.raw_decode(h[i + len('"intelligenceIndexEvaluations":'):])
        ev = {e['slug']: e for e in evs}
        tok = re.search(r'"canonicalIntelligenceIndexTokenCount":\{[^}]*"output":([0-9]+)', seg)
        tc = re.search(r'"intelligenceIndexCost":\{"total":([0-9.]+)', seg)
        cpt = re.search(r'"intelligenceIndexCostPerTask":\{"cost":\{"total":([0-9.]+)', seg)
        rec = dict(slug=slug, name=name, short=short.group(1) if short else None, effort=eff.group(1) if eff else None,
                   ii=num('intelligenceIndex', seg), run_cost=float(tc.group(1)) if tc else None,
                   cost_per_task=float(cpt.group(1)) if cpt else None,
                   time_per_task_min=(num('intelligenceIndexTimePerTask', seg) or 0) / 60,
                   out_tok=int(tok.group(1)) if tok else None,
                   price_in=num('price1mInputTokens', seg), price_out=num('price1mOutputTokens', seg),
                   fc20=ev.get('frontiercode-2-0'), briefcase=ev.get('aa-briefcase'), ev=ev)
        seen.setdefault(slug, rec)
cols = ['slug', 'short', 'ii', 'cost_per_task', 'run_cost', 'time_per_task_min', 'out_tok']
print('\t'.join(cols + ['fc20_score', 'fc20_cost_per_task', 'fc20_out_tok_per_task', 'fc20_time_s', 'briefcase_elo']))
for r in seen.values():
    f = r['fc20'] or {}; b = r['briefcase'] or {}
    print('\t'.join(str(x) for x in [r[c] if not isinstance(r[c], float) else round(r[c], 4) for c in cols] +
          [round(f.get('score', float('nan')) * 100, 2), round(f.get('costPerTask', float('nan')), 2), (round(f['outputTokensPerTask']) if 'outputTokensPerTask' in f else None), (round(f['timePerTask']) if f.get('timePerTask') is not None else None), b.get('score')]))
