import json,runpy,pathlib,re
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
for name,sess,expr,url in [('d-aihot-dayjob','ev1003hot','JSON.stringify({url:location.href,text:document.querySelector("main")?.innerText})','https://aihot.news/all?q=DAYJOB'),('c-turing-browser-failure','ev1003t','JSON.stringify({title:document.title,text:document.querySelector("main")?.innerText||document.body.innerText})','https://academic.oup.com/mind/article/LIX/236/433/986238')]:
 d=m['call'](sess,'eval',expr);(p/(name+'.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log'](url,'OpenCLI public content','结果见存档',name+'.json')
# public metadata and PDF text page map, no reconstructed original claims
for f in p.glob('*-full.txt'):
 pages=f.read_text(encoding='utf8').split('\f');out=[]
 for i,t in enumerate(pages):
  if any(x in t for x in ['Concurrent work','Concurrent Work','Statement of AI Use','leave open the question','seems natural to conjecture']):out.append({'page':i+1,'text':t})
 (p/(f.stem+'-keypages.json')).write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8')
