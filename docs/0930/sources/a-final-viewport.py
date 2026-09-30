import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture');b.SESSION='evidence0930qa'
b.shot('17-whitehouse-section-one.png',560,1000)
b.shot('18-whitehouse-sections-two-three.png',1260,1170)
b.call('eval','window.scrollTo({top:0,behavior:"instant"});[...document.querySelectorAll("button")].find(e=>e.innerText.trim()==="MENU")?.click();true')
b.call('hover','#menu-item-29161 > a');b.call('hover','#menu-item-33121 > a')
b.shot('26-whitehouse-ai-navigation.png',0,900)
for lang,name in [('zh-Hans-CN','06-devday-chinese.png'),('en-US','07-devday-english.png')]:
 b.call('open',f'https://openai.com/{lang}/index/devday-2026-recap/')
 b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0)?.click();true')
 b.call('eval','new Promise(r=>setTimeout(r,1500))')
 b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0)?.scrollIntoView({behavior:"instant",block:"start"});true')
 b.call('eval','new Promise(r=>setTimeout(r,1500))')
 b.call('eval','window.scrollBy({top:[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0).getBoundingClientRect().top-180,behavior:"instant"});true')
 b.call('screenshot',str((b.I/name).resolve()))
 b.log(dict(id=name,url=f'https://openai.com/{lang}/index/devday-2026-recap/',time_bjt=b.stamp(),tool='opencli browser screenshot native viewport; expanded Sol and positioned heading at 180 CSS px',status='success',file='images/'+name))
