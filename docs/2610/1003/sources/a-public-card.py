import runpy,json
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
d=m['call']('ev1003meta','eval','JSON.stringify({text:document.querySelector("article")?.innerText,time:document.querySelector("article time")?.dateTime,metrics:[...document.querySelector("article").querySelectorAll("button[aria-label],a[aria-label]")].map(e=>({label:e.getAttribute("aria-label"),text:e.innerText}))})');(p/'a-meta-x-card.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://x.com/AIatMeta/status/2106099776035152231','OpenCLI article only','公开帖正文、互动、UTC时间；已切英文','a-meta-x-card.json')
m['log']('https://academic.oup.com/mind/article/LIX/236/433/986238','requests + OpenCLI browser','HTTP 403 / Cloudflare 安全验证；未绕过；未取得全文','c-turing-original.html')
