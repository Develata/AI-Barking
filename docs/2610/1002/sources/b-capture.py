import subprocess,json,pathlib,datetime,sys,time
P=pathlib.Path(__file__).parent; I=P.parent/'images'; SESSION='evidence1002b'
def now():return datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
def log(url,result,file='',tool='opencli browser'):
 with (P/'b-capture-log.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(dict(time_bjt=now(),url=url,tool=tool,result=result,file=file),ensure_ascii=False)+'\n')
def call(*args):
 r=subprocess.run(['opencli.exe','browser',SESSION,*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=90)
 s=r.stdout.split('\n  Update available:')[0].strip()
 if r.returncode:raise RuntimeError((s+r.stderr)[:1500])
 try:return json.loads(s)
 except:return s
def archive(name,url):
 try:
  call('open',url);time.sleep(3);call('state')
  d=call('eval','JSON.stringify({url:location.href,title:document.title,text:(document.querySelector("main")||document.querySelector("article")).innerText,dpr:devicePixelRatio,meta:[...document.querySelectorAll("meta[property],meta[name],time")].map(e=>e.outerHTML),headings:[...document.querySelectorAll("h1,h2,h3,h4")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),paragraphs:[...document.querySelectorAll("main p,article p")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})),links:[...document.querySelectorAll("main a,article a")].map(e=>({text:e.innerText,url:e.href}))})')
  if isinstance(d,str):d=json.loads(d)
  d['capture_time_bjt']=now(); (P/(name+'-browser.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8');(P/(name+'.txt')).write_text(d['text'],encoding='utf-8');log(url,'captured; inspect content',name+'-browser.json');print(name,d['title'],len(d['text']),d['dpr'],flush=True)
 except Exception as e:log(url,'FAILED '+str(e));print(e,flush=True)
if __name__=='__main__':
 if sys.argv[1]=='archive':archive(sys.argv[2],sys.argv[3])
 elif sys.argv[1]=='batch':
  for n,u in json.loads(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8-sig')).items():archive(n,u)
