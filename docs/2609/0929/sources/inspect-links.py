from pathlib import Path
from bs4 import BeautifulSoup
p=Path('docs/0929/sources')
for n in ['c-cellcog-lead','d-c-huggingnews']:
 s=BeautifulSoup((p/(n+'.html')).read_text(encoding='utf8'),'html.parser');print(n)
 for a in s.select('a[href]'):
  if any(x in a.get('href','') for x in ['cnbc','reuters','wsj','maxwell','astra','Astra']):print(a.get_text(' ',strip=True),a['href'])
