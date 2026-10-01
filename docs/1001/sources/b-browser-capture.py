import subprocess,json,pathlib,datetime,sys
P=pathlib.Path(__file__).parent; I=P.parent/'images'; SESSION='evidence1001'
def now():return datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
def log(url,result,file=''):
 with (P/'d-browser-log.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(dict(time=now(),url=url,tool='opencli browser',result=result,file=file),ensure_ascii=False)+'\n')
def call(*args):
 r=subprocess.run(['opencli.exe','browser',SESSION,*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100)
 if r.returncode:raise RuntimeError((r.stdout+r.stderr)[:1000])
 s=r.stdout.split('\n  Update available:')[0].strip()
 try:return json.loads(s)
 except:return s
def archive(name,url):
 try:
  call('open',url);call('state')
  d=call('extract','--chunk-size','100000')
  (P/(name+'-extract.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8')
  layout=call('eval','JSON.stringify({title:document.title,url:location.href,headings:[...document.querySelectorAll("h1,h2,h3")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),images:[...document.images].map(e=>({src:e.currentSrc,alt:e.alt,y:e.getBoundingClientRect().top+scrollY,h:e.height})),paragraphs:[...document.querySelectorAll("main p,article p")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height}))})')
  (P/(name+'-layout.json')).write_text(json.dumps(layout,ensure_ascii=False,indent=2),encoding='utf-8')
  log(url,'captured; inspect content',name);print(name,str(d)[:180],flush=True)
 except Exception as e:log(url,'FAILED '+str(e));print(e,flush=True)
def shot(name,y,h):
 call('scroll','up','--amount','100000');call('scroll','down','--amount',str(max(0,int(y))))
 call('screenshot',str((I/name).resolve()),'--width','1266','--height',str(h));log(call('get','url'),'screenshot',name);print(name,flush=True)
if __name__=='__main__':
 if sys.argv[1]=='archive':archive(sys.argv[2],sys.argv[3])
 elif sys.argv[1]=='shot':shot(sys.argv[2],float(sys.argv[3]),int(sys.argv[4]))
