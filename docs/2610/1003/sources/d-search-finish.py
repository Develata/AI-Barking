import runpy,json
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
d=m['call']('ev1003hot','eval','JSON.stringify({url:location.href,text:document.querySelector("main")?.innerText})');(p/'d-aihot-dayjob-fullsearch.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://aihot.news/all?q=DAYJOB&tab=relevance','OpenCLI browser search','全文相关 0 条','d-aihot-dayjob-fullsearch.json')
m['log']('https://www.threads.com/@aiatmeta','OpenCLI browser','标题可读；main 正文未取到，未确认帖URL/时间')
