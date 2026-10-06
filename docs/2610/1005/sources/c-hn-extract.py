import json,html
from pathlib import Path
p=Path('docs/2610/1005/sources')
j=json.loads((p/'c-hn.json').read_text(encoding='utf-8'))
rows=[]
def walk(n):
    if n.get('text'): rows.append({'id':n['id'],'author':n.get('author'),'points':n.get('points'),'created_at':n.get('created_at'),'text':html.unescape(n['text'])})
    for c in n.get('children',[]): walk(c)
walk(j)
with (p/'c-hn-comments.json').open('x',encoding='utf-8') as f: json.dump(rows,f,ensure_ascii=False,indent=2)
for n in rows:
    if any(x in n['text'].lower() for x in ['dft','hubbard','altermagn','condensed','density functional','phd in magnetic']):print(n['id'],n['author'],n['points'],n['text'])
