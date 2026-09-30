import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
d=b.archive('c-order','https://www.whitehouse.gov/presidential-actions/2026/09/inaugurating-the-era-of-super-intelligence/')
print(b.call('eval','JSON.stringify([...document.querySelectorAll("p")].filter(e=>/Section|Purpose|Accordingly|Implementation|Definition|Within 60/.test(e.innerText)).map(e=>({text:e.innerText.slice(0,100),y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})))'),flush=True)
print(d['headings'][:8],flush=True)
