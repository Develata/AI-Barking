from pathlib import Path
import fitz,json
p=Path(__file__).parent
for f in sorted(p.glob('*.pdf')):
 d=fitz.open(f);pages=[x.get_text() for x in d];(p/(f.stem+'-pages.json')).write_text(json.dumps(pages,ensure_ascii=False,indent=2),encoding='utf8');(p/(f.stem+'.txt')).write_text('\n'.join('=== PDF PAGE '+str(i+1)+' ===\n'+t for i,t in enumerate(pages)),encoding='utf8');print(f.name,len(pages))
