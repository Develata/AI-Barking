import subprocess,json,pathlib,datetime,sys
P=pathlib.Path(__file__).parent
def call(*args):
 r=subprocess.run(['opencli.exe','browser','evidence1002c',*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100)
 if r.returncode:raise RuntimeError((r.stdout+r.stderr)[:900])
 return r.stdout.split('\n  Update available:')[0].strip()
def log(url,result,file):
 with (P/'c-capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps({'time':datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(),'url':url,'tool':'opencli browser','result':result,'file':file})+'\n')
name,url=sys.argv[1:3]
try:
 call('open',url)
 d=json.loads(call('eval','(() => ({url:location.href,title:document.title,text:(document.querySelector("article")||document.querySelector("main")||document.body).innerText,headings:[...document.querySelectorAll("h1,h2,h3")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),tables:[...document.querySelectorAll("table")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})),meta:[...document.querySelectorAll("meta[property]")].map(e=>({property:e.getAttribute("property"),content:e.content}))}))()'))
 (P/(name+'-browser.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8');log(url,'captured; validate URL and content',name+'-browser.json');print(d['url'],d['title'],len(d['text']))
except Exception as e:log(url,'FAILED '+str(e),'');print(e)
