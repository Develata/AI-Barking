from pathlib import Path
from bs4 import BeautifulSoup
import json,runpy,concurrent.futures
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P'];out=[]
for f in sorted(p.glob('a-paper-*.html')):
 s=BeautifulSoup(f.read_text(encoding='utf8'),'html.parser')
 links=[(a.get_text(' ',strip=True),a.get('href')) for a in s.find_all('a') if 'Download the Paper' in a.get_text()]
 print(f.name,links)
 for _,url in links:out.append((f.stem+'-full',url))
(p/'a-paper-downloads.json').write_text(json.dumps(out,indent=2),encoding='utf8')
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as ex:list(ex.map(lambda x:m['fetch'](*x),out))
