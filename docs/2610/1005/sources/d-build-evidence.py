from pathlib import Path
import json, re, hashlib
from datetime import datetime, timezone, timedelta
from html.parser import HTMLParser
ROOT=Path(__file__).resolve().parent
def read(n): return (ROOT/(n+'.txt')).read_text(encoding='utf-8')
def data(n): return json.loads((ROOT/(n+'.json')).read_text(encoding='utf-8'))
def quote(n,anchor):
    for i,s in enumerate(read(n).splitlines(),1):
        if anchor in s:return s.strip()+f'（存档 {n}.txt:L{i}）'
    raise ValueError((n,anchor))
def esc(s):return str(s).replace('|','\\|').replace('\n','<br>')
class Table(HTMLParser):
    def __init__(self):super().__init__();self.rows=[];self.row=[];self.cell=None
    def handle_starttag(self,t,a):
        if t=='tr':self.row=[]
        if t in ('td','th'):self.cell=''
    def handle_data(self,s):
        if self.cell is not None:self.cell+=s
    def handle_endtag(self,t):
        if t in ('td','th') and self.cell is not None:self.row.append(self.cell.strip());self.cell=None
        if t=='tr':self.rows.append(self.row)
tables=[]
for i,t in enumerate(data('d-beam-layout')['tables']):
    p=Table();p.feed(t['html']);tables.append(p.rows)
out=['# Beam 官方全部四个评测表\n','来源：https://reflection.ai/blog/introducing-beam；机器转录 DOM table，不补值。NR 原样保留。截图见 evidence-d.md。\n']
for title,rs in zip(['Agentic Coding/Terminal','Reasoning','Tool Calling / Search','General Capabilities'],tables):
    out+=['## '+title,'','| '+' | '.join(rs[0])+' |','| '+' | '.join(['---']*len(rs[0]))+' |']+['| '+' | '.join(r)+' |' for r in rs[1:]]+['']
out+=['## 完整估算图注','',quote('d-beam','Figure 2:'),'','NR 原文：'+quote('d-beam','NR denotes')]
(ROOT/'d-beam-tables.md').write_text('\n'.join(out),encoding='utf-8')
# Main ledger is assembled after live supplemental captures; no external actions.
