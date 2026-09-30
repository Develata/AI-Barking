from pathlib import Path
try:
 from pypdf import PdfReader
 r=PdfReader('docs/0930/sources/c-sp330.pdf')
 s='\n\n'.join(p.extract_text() or '' for p in r.pages[:2])
 Path('docs/0930/sources/c-sp330-cover-text.txt').write_text(s,encoding='utf-8')
 print(s[:2400])
except ImportError as e: print(type(e).__name__,str(e))
