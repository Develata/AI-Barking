import runpy,json,subprocess
m=runpy.run_path('docs/2610/1002/sources/a-capture.py');c=m['call'];I=m['I']
def shot(n,u,needle,h):
 c('open',u);c('wait','text',needle,'--timeout','20000');c('state');subprocess.run(['bun','docs/2610/1002/sources/b-dpr.mjs','evidence1002a'],check=True,capture_output=True)
 d=c('eval','JSON.stringify([...document.querySelectorAll("h1,h2,h3,p")].filter(e=>e.innerText.includes('+json.dumps(needle)+')).map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})))')
 if isinstance(d,str):d=json.loads(d)
 if not d:print('missing',n);return
 y=max(0,int(d[0]['y'])-35);subprocess.run(['bun','docs/2610/1002/sources/b-dpr.mjs','evidence1002a',str(I/n),str(y),str(h)],check=True,capture_output=True);m['log'](u,'OpenCLI CDP Page.captureScreenshot',f'DPR2,width1100,y{y},height{h}',n);print(n,flush=True)
a='https://blog.google/innovation-and-ai/models-and-research/google-research/'
for job in [('01-suncatcher-prototype.png',a+'project-suncatcher-prototype/','Our Project Suncatcher prototype satellite is in orbit.',1400),('02-suncatcher-hardware.png',a+'google-project-suncatcher-facts/','Hardware survival',1150),('03-suncatcher-cooling.png',a+'google-project-suncatcher-facts/','Cooling in space',850),('04-suncatcher-scale.png',a+'google-project-suncatcher-facts/','Satellite interconnectivity',900),('05-suncatcher-power.png',a+'google-project-suncatcher-facts/','In low Earth orbit',570),('06-suncatcher-npr.png','https://www.npr.org/2026/10/01/nx-s1-5983697/project-suncatcher-google-ai-data-center-space','Google just put a refrigerator-sized',650),('07-suncatcher-joule.png','https://www.cell.com/joule/fulltext/S2542-4351(26)00362-4','Summary',800)]:
 try:shot(*job)
 except Exception as e:m['log'](job[1],'screenshot',str(e));print(e,flush=True)
