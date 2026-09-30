import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
for lang,n in [('zh-Hans-CN','06-devday-chinese.png'),('en-US','07-devday-english.png')]:
 b.call('open',f'https://openai.com/{lang}/index/devday-2026-recap/')
 b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0)?.click();true')
 b.call('eval','new Promise(r=>setTimeout(r,1500))')
 y=b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText==="GPT-6.1 Sol"&&e.getBoundingClientRect().height>0).getBoundingClientRect().top+scrollY')
 b.shot(n,y-190,1000)
 b.archive('a-devday-'+('zh' if lang=='zh-Hans-CN' else 'en')+'-expanded')
b.archive('a-aa-article-final','https://artificialanalysis.ai/articles/gpt-6-1-sol-replaces-gpt-6-sol-after-just-7-days-with-near-astra-intelligence')
b.shot('22-aa-intelligence-cost.png',290,1120)
b.archive('c-order-nav','https://www.whitehouse.gov/presidential-actions/2026/09/inaugurating-the-era-of-super-intelligence/')
b.call('eval','[...document.querySelectorAll("button")].find(e=>e.innerText.trim()==="MENU")?.click();true')
b.shot('26-whitehouse-ai-navigation.png',0,1080)
