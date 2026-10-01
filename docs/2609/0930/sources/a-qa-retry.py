import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture');b.SESSION='evidence0930qa'
b.shot('18-whitehouse-sections-two-three.png',1260,1100)
b.call('open','https://openai.com/en-US/index/introducing-gpt-6-1-sol/')
b.call('eval','document.querySelectorAll("figure")[4].scrollIntoView({behavior:"instant",block:"center"});true')
b.call('eval','new Promise(r=>setTimeout(r,3000))')
d=b.archive('a-release-osworld-loaded');y=next(x['y'] for x in d['headings'] if x['text']=='Computer use');b.shot('04-sol-osworld.png',y-90,1120)
for lang,name in [('zh-Hans-CN','06-devday-chinese.png'),('en-US','07-devday-english.png')]:
 b.call('open',f'https://openai.com/{lang}/index/devday-2026-recap/')
 b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0)?.click();true')
 b.call('eval','new Promise(r=>setTimeout(r,1200))')
 b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0)?.scrollIntoView({behavior:"instant",block:"center"});true')
 b.call('eval','new Promise(r=>setTimeout(r,1200))')
 b.call('eval','window.scrollBy({top:-240,behavior:"instant"});true')
 b.call('screenshot',str((b.I/name).resolve()))
 b.log(dict(id=name,url=f'https://openai.com/{lang}/index/devday-2026-recap/',time_bjt=b.stamp(),tool='opencli browser screenshot native viewport; Sol expanded; corrected sticky-header position',status='success',file='images/'+name))
