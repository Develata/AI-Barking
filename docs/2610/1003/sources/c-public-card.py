import runpy,json
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
d=m['call']('ev1003x','eval','JSON.stringify({text:document.querySelector("article").innerText,time:document.querySelector("article time")?.dateTime,metrics:[...document.querySelector("article").querySelectorAll("button[aria-label],a[aria-label]")].map(e=>({label:e.getAttribute("aria-label"),text:e.innerText}))})')
(p/'c-tavus-x-card.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://x.com/tavus/status/2105704169009246248','OpenCLI article only eval','英文原文及拟议注释；无导航/抓取者信息','c-tavus-x-card.json')
d=m['call']('ev1003hot','eval','JSON.stringify({text:document.querySelector("main")?.innerText,links:[...document.querySelectorAll("main a")].map(e=>({text:e.innerText,url:e.href}))})')
(p/'d-aihot.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://aihot.news','OpenCLI main public content','首页已加载部分；非全部历史','d-aihot.json')
