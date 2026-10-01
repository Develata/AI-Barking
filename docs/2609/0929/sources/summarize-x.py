import json
from pathlib import Path
p=Path('docs/0929/sources')
for n in ['c-official-x-search','c-openai-tweets','c-jain-tweets']:
 if not (p/(n+'.json')).exists():continue
 t=(p/(n+'.json')).read_text(encoding='utf-8-sig'); pos=t.find('[\n')
 if pos<0:print(n,t[:400]);continue
 data,_=json.JSONDecoder().raw_decode(t[pos:]); print(n)
 for x in data:print(x.get('author'),x.get('created_at'),x.get('url'),x.get('text','')[:400])
