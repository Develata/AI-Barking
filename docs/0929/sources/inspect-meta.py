from pathlib import Path
from bs4 import BeautifulSoup
import json
p=Path('docs/0929/sources')
for name in ['a-guardian','a-bi-initial','a-bi-update','a-pcmag','c-cbs','c-cna']:
 s=BeautifulSoup((p/(name+'.html')).read_text(encoding='utf8'),'html.parser')
 print(name)
 for x in s.select('script[type="application/ld+json"]'):
  def walk(v):
   if isinstance(v,dict):
    for k,z in v.items():
     if k in ['datePublished','dateModified','headline']: print(k,z)
     elif isinstance(z,(list,dict)):walk(z)
   elif isinstance(v,list):
    for z in v:walk(z)
  try:walk(json.loads(x.get_text()))
  except:pass
 print('links',*[a.get('href') for a in s.select('a[href]') if any(h in a.get('href','') for h in ['threads.com/','reuters.com/','cnbc.com/','x.com/dps'])],sep='\n')
