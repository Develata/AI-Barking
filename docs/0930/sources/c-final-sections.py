import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture');b.SESSION='evidence0930qa'
b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText.trim()==="CLOSE")?.click();true')
b.call('eval','new Promise(r=>setTimeout(r,3000))')
b.shot('17-whitehouse-section-one.png',560,1000)
b.shot('18-whitehouse-sections-two-three.png',1260,1170)
