import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
d=b.archive('b-anthropic-figures')
for idx,name in [(1,'11-exploitbench-full.png'),(4,'12-engagement-full.png'),(3,'13-abliteration-full.png')]:
 f=d['figures'][idx];b.shot(name,f['y']-90,int(f['h']+190))
b.shot('14-flash-cost-context.png',4560,850)
d=b.archive('b-nist','https://www.nist.gov/news-events/news/2026/09/caisis-assessment-zais-glm-53-cyber-capabilities')
t=b.call('eval','JSON.stringify([...document.querySelectorAll("table")].map((e,i)=>({i,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height,text:e.innerText})))')
print(t,flush=True)
for i,n in [(0,'15-caisi-benchmark-definitions.png'),(1,'16-caisi-results.png')]:
 b.shot(n,t[i]['y']-110,int(t[i]['h']+320))
