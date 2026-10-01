import sys,pathlib
sys.path.insert(0,'docs/0930/sources')
import importlib
b=importlib.import_module('a-browser-capture')
for name,url,needle,shot in [
 ('a-devday-zh','https://openai.com/zh-Hans-CN/index/devday-2026-recap/','GPT-6.1 Sol','06-devday-chinese.png'),
 ('a-devday-en','https://openai.com/en-US/index/devday-2026-recap/','GPT-6.1 Sol','07-devday-english.png'),
 ('a-doc-pricing','https://developers.openai.com/api/docs/pricing','Flagship models','08-standard-price-table.png')]:
 try:
  d=b.archive(name,url)
  if name=='a-doc-pricing':
   print(b.call('eval','JSON.stringify([...document.querySelectorAll("table")].slice(0,3).map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})))'),flush=True)
  y=next((x['y'] for x in d['headings'] if needle in x['text']),0)
  b.shot(shot,y-90,1000)
 except Exception as e:print(name,str(e),flush=True)
