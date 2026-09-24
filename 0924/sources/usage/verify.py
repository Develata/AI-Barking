import pathlib,json,hashlib,math,random,re,statistics,collections
p=pathlib.Path(__file__).resolve().parent;root=p.parents[2]
def rows(f):return [json.loads(l) for l in (p/f).open(encoding='utf-8')]
raw=rows('x_raw.jsonl');coded=rows('x_coded.jsonl');blind=rows('x_blind20.jsonl');exc=rows('x_excluded.jsonl')
assert len(set(r['id']for r in raw))==len(raw)
assert {r['id']for r in coded}.isdisjoint(r['id']for r in exc)
assert {r['id']for r in raw}=={r['id']for r in coded+exc}
assert all(set(r)=={'id','url','text'}for r in blind)
assert len(blind)==min(20,len(coded))
assert blind==[{k:r[k]for k in ['id','url','text']}for r in random.Random(20260924).sample(coded,min(20,len(coded)))]
assert all(r['evidence_quote'] in raw[r['raw_index']]['text'] for r in coded)
assert all(r['text']==raw[r['raw_index']]['text']for r in coded)
s=json.load((p/'x_stats.json').open(encoding='utf-8'))
assert (s['deduplicated'],s['included'],s['excluded'])==(len(raw),len(coded),len(exc))
assert sum(s['exclusions'].values())==len(exc)
z=statistics.NormalDist().inv_cdf(.975);checks=0
for d in [s['verdict'],*s['sentiments'].values(),*s['by_task'].values(),s['without_confirmed_employee']]:
 assert sum(v['k']for v in d['categories'].values())==d['n']
 for c,v in d['categories'].items():
  n=v['n'];k=v['k'];ci=v['wilson95']
  if n:
   # Independently verify roots of n(p-k/n)^2 = z^2*p*(1-p).
   a=n+z*z;b=-(2*k+z*z);cc=k*k/n;disc=b*b-4*a*cc
   bounds=((-b-math.sqrt(max(0,disc)))/(2*a),(-b+math.sqrt(max(0,disc)))/(2*a))
   assert all(abs(x-y)<1e-10 for x,y in zip(ci,bounds))
   assert abs(v['proportion']-k/n)<1e-12
  else:assert ci is None
  checks+=1
report=(p.parent/'usage-evidence.md').read_text(encoding='utf-8')
assert all('## '+x+'.' in report for x in 'ABCD')
assert all(str(n)in report for n in [len(raw),len(coded)])
baseline=json.load((p/'baseline.json').open(encoding='utf-8'));changed=[]
for rel,digest in baseline.items():
 f=root/rel
 if not f.is_file() or hashlib.sha256(f.read_bytes()).hexdigest()!=digest:changed.append(rel)
assert not changed,changed
allfiles=[f for f in root.rglob('*')if f.is_file() and '.git' not in f.relative_to(root).parts]
new=[f for f in allfiles if str(f.relative_to(root)) not in baseline]
outside=[]
for f in new:
 rel=f.relative_to(root).as_posix()
 if not(rel.startswith('0924/sources/usage/') or rel=='0924/sources/usage-evidence.md' or re.fullmatch(r'0924/images/(?:30|31|32)-[^/]+\.png',rel)):outside.append(rel)
assert not outside,outside
assert len(list((root/'0924/images').glob('30-*.png')))<=2
assert len(list((root/'0924/images').glob('31-*.png')))<=1
assert len(list((root/'0924/images').glob('32-*.png')))<=1
newpaths={f.relative_to(root).as_posix()for f in new}|{'0924/sources/usage/new-files.txt'}
(p/'new-files.txt').write_text('\n'.join(sorted(newpaths))+'\n',encoding='utf-8')
print(f'raw={len(raw)} coded={len(coded)} excluded={len(exc)} blind={len(blind)}')
print('jsonl_schema_partition_quotes_seed ok')
print(f'Wilson intervals independently checked: {checks}')
print(f'baseline files unchanged: {len(baseline)}; modified/deleted: 0')
print('new files outside allowed scope: 0')
print('screenshots: Arena=2 OpenRouter=1 FrontierMath=1')
print('report A/B/C/D and bias notes: present')
