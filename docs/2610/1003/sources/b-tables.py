import json,subprocess,runpy
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
# Public NVIDIA table only, no account UI.
d=m['call']('ev1003v','eval','JSON.stringify([...document.querySelectorAll("table")].map(e=>({text:e.innerText,html:e.outerHTML})))')
(p/'c-videofdb-tables.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://research.nvidia.com/labs/amri/projects/video-fdb/','opencli eval public tables','两张表 DOM','c-videofdb-tables.json')
d=m['call']('ev1003b','eval','JSON.stringify([...document.querySelectorAll("figure.ltx_table")].map(e=>({id:e.id,text:e.innerText,html:e.outerHTML})))')
(p/'b-dayjob-tables.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://arxiv.org/html/2610.01306v1','opencli eval public tables','三张表 DOM','b-dayjob-tables.json')
