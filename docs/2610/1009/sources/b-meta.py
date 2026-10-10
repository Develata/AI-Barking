"""b-meta.py: print title/published/modified meta from saved HTML. usage: python -I b-meta.py file.html ..."""
import re,sys,html
for fn in sys.argv[1:]:
    s=open(fn,encoding='utf-8',errors='replace').read()
    out=[]
    for m in re.finditer(r'<meta[^>]+>',s):
        t=m.group(0)
        if re.search(r'(published|modified|date|author)',t,re.I) and 'content=' in t:
            out.append(html.unescape(re.sub(r'\s+',' ',t))[:200])
    for m in re.finditer(r'"(datePublished|dateModified)"\s*:\s*"([^"]+)"',s): out.append(m.group(0))
    ti=re.search(r'<title[^>]*>(.*?)</title>',s,re.S)
    print('##',fn,'|',html.unescape(ti.group(1).strip()) if ti else '')
    for o in out[:8]: print('  ',o)
