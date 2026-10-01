import pymupdf
from pathlib import Path
p=Path('docs/0929/sources')
d=pymupdf.open(p/'b-report.pdf')
(p/'b-report.txt').write_text('\n'.join('PAGE '+str(i+1)+'\n'+x.get_text() for i,x in enumerate(d)),encoding='utf8')
print('pages',len(d))
