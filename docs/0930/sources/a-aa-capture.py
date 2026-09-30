import sys,importlib,json
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
d=b.archive('a-aa-article','https://artificialanalysis.ai/articles/gpt-6-1-sol-replaces-gpt-6-sol-after-just-7-days-with-near-astra-intelligence')
imgs=b.call('eval','JSON.stringify([...document.querySelectorAll("img")].map((e,i)=>({i,src:e.currentSrc,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})))')
print(imgs,flush=True)
for idx,name in [(0,'22-aa-intelligence-cost.png'),(1,'23-aa-coding-index.png'),(2,'24-aa-token-efficiency.png')]:
 e=imgs[idx];b.shot(name,e['y']-90,int(e['h']+190))
d=b.archive('a-aa-sol-release','https://artificialanalysis.ai/models/releases/gpt-6-1-sol');b.shot('25-aa-sol-effort-levels.png',180,1100)
