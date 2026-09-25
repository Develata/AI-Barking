import json,pathlib,datetime,email.utils
p=pathlib.Path('0924/sources/usage'); ledger=[json.loads(x) for x in (p/'search_ledger.jsonl').read_text(encoding='utf-8').splitlines()]; raw={}; total=0
for e in ledger:
 if e['returncode']:continue
 for r in json.loads((p/e['file']).read_text(encoding='utf-8')):
  total+=1; id=str(r['id']); occurrence={k:e[k] for k in ['base_query','sort','finished_at','file']}
  if id in raw:raw[id]['query'].append(occurrence);continue
  r['id']=id;r['query']=[occurrence];raw[id]=r
rows=sorted(raw.values(),key=lambda r:int(r['id']))
with (p/'x_raw.jsonl').open('w',encoding='utf-8') as f:
 for r in rows:f.write(json.dumps(r,ensure_ascii=False)+'\n')
with (p/'x_review.txt').open('w',encoding='utf-8') as f:
 for i,r in enumerate(rows):f.write(f"[{i}] @{r['author']} {r['created_at']} views={r['views']} BIO={r.get('bio','')}\n{r['text']}\n\n")
print('total',total,'unique',len(rows),'empty_bio',sum(not r.get('bio') for r in rows))
