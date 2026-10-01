import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
for name,url in [('a-sol-model','https://developers.openai.com/api/docs/models/gpt-6-sol'),('b-anthropic','https://www.anthropic.com/research/glm-5-3-and-the-spread-of-advanced-cyber-capabilities')]:
 d=b.archive(name,url)
 print(name,d['headings'],d['figures'],flush=True)
 if name=='a-sol-model':
  y=720;b.shot('09-sol-old-price.png',y-90,1000)
 else:
  b.shot('10-anthropic-title.png',0,1050)
  print(b.call('eval','JSON.stringify([...document.querySelectorAll("img")].map((e,i)=>({i,alt:e.alt,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})))'),flush=True)

