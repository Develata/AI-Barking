import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
d=b.archive('b-anthropic-recheck','https://www.anthropic.com/research/glm-5-3-and-the-spread-of-advanced-cyber-capabilities')
for idx,name in [(4,'12-engagement-full.png'),(3,'13-abliteration-full.png')]:
 f=d['figures'][idx];b.shot(name,f['y']-90,int(f['h']+250))
d=b.archive('a-devday-zh-recheck','https://openai.com/zh-Hans-CN/index/devday-2026-recap/')
print(b.call('eval','JSON.stringify([...document.querySelectorAll("button,h1,h2,h3,h4")].filter(e=>e.innerText.includes("GPT-6.1 Sol")).map(e=>({tag:e.tagName,text:e.innerText,y:e.getBoundingClientRect().top+scrollY,html:e.outerHTML.slice(0,900)})))'),flush=True)
