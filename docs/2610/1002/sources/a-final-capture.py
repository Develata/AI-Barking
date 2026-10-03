import runpy,json,subprocess
m=runpy.run_path('docs/2610/1002/sources/a-capture.py');c=m['call']
c('open','https://www.cell.com/joule/fulltext/S2542-4351(26)00362-4');c('state');c('wait','selector','body','--timeout','20000');c('wait','text','Summary','--timeout','20000');subprocess.run(['bun','docs/2610/1002/sources/b-dpr.mjs','evidence1002a'],capture_output=True)
d=c('eval','JSON.stringify({authors:[...document.querySelectorAll("meta[name=citation_author]")].map(x=>x.content),summary:[...document.querySelectorAll("h2,h3")].filter(e=>e.innerText==="Summary").map(e=>e.getBoundingClientRect().top+scrollY)})');print(d)
(m['P']/'a-joule-metadata.json').write_text(json.dumps(d,indent=2),encoding='utf8')
y=d['summary'][0]-35 if d['summary'] else 550
subprocess.run(['bun','docs/2610/1002/sources/b-dpr.mjs','evidence1002a',str(m['I']/'07-suncatcher-joule.png'),str(y),'800']);m['log']('https://www.cell.com/joule/fulltext/S2542-4351(26)00362-4','OpenCLI CDP screenshot','DPR2 summary','07-suncatcher-joule.png')
m['archive']('a-planet-new','https://investors.planet.com/news/news-details/2026/Planet-Launches-Suncatcher-Tanager-2-and-18-SuperDove-Satellites/default.aspx')
