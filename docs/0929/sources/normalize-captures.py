from pathlib import Path
import json,re
p=Path('docs/0929/sources')
for f in p.glob('*-browser.json'):
 t=f.read_text(encoding='utf-8-sig');start=t.find('{\n')
 if start<0:continue
 try:
  d,_=json.JSONDecoder().raw_decode(t[start:]);(p/(f.stem+'.md')).write_text('Source: '+d.get('url','')+'\nTitle: '+d.get('title','')+'\n\n'+d.get('content',''),encoding='utf8')
 except Exception as e:print(f.name,e)
for n in ['c-bbc','d-a-futurism','d-b-yybdj','d-b-madrobot']:
 print(n)
 d=json.loads((p/(n+'-metadata.json')).read_text(encoding='utf8'))
 for s in d['jsonld']:
  print('\n'.join(re.findall(r'"(?:datePublished|dateModified|headline)"\s*:\s*"[^\"]*"',s)))
