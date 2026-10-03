import requests,pathlib,json,datetime,concurrent.futures,subprocess,sys
from bs4 import BeautifulSoup
P=pathlib.Path(__file__).parent; I=P.parent/'images'
def now():return datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
def log(url,tool,result,file=''):
 with (P/'c-capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(dict(time=now(),url=url,tool=tool,result=result,file=file),ensure_ascii=False)+'\n')
def fetch(name,url):
 try:
  r=requests.get(url,headers={'User-Agent':'Mozilla/5.0'},timeout=45); ext='json' if 'json' in r.headers.get('Content-Type','') else 'html'; fn=name+'.'+ext
  (P/fn).write_bytes(r.content);log(url,'requests GET without cookies',f'HTTP {r.status_code}; final={r.url}',fn)
  if ext=='html':
   s=BeautifulSoup(r.content,'html.parser');[e.decompose() for e in s(['script','style'])];(P/(name+'.txt')).write_text(s.get_text('\n',strip=True),encoding='utf-8')
  print(name,r.status_code,len(r.content),flush=True)
 except Exception as e:log(url,'requests GET',str(e));print(name,str(e),flush=True)
def call(*args):
 r=subprocess.run(['opencli.exe','browser','evidence1002c',*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100)
 out=r.stdout.split('\n  Update available:')[0].strip()
 if r.returncode:raise RuntimeError((out+r.stderr)[:1000])
 try:return json.loads(out)
 except:return out
def archive(name,url):
 try:
  call('open',url);call('state');d=call('eval','JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,dpr:devicePixelRatio,meta:[...document.querySelectorAll("meta[property],time")].map(e=>e.outerHTML),headings:[...document.querySelectorAll("h1,h2,h3")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),tables:[...document.querySelectorAll("table")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height})),links:[...document.querySelectorAll("main a,article a")].map(e=>({text:e.innerText,url:e.href}))})')
  if isinstance(d,str):d=json.loads(d)
  (P/(name+'-browser.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8');log(url,'opencli browser open/state/eval','captured; validate contents',name+'-browser.json');print(name,d['title'],len(d['text']),flush=True)
 except Exception as e:log(url,'opencli browser',str(e));print(str(e),flush=True)
if __name__=='__main__':
 if len(sys.argv)>1:archive(sys.argv[1],sys.argv[2]);sys.exit()
 jobs={
 'c-cloudflare':'https://blog.cloudflare.com/clef-decision-models/',
 'c-index':'https://huggingface.co/spaces/multimodalart/jev-decision-index',
 'c-clef':'https://huggingface.co/Cloudflare/clef',
 'c-flash':'https://huggingface.co/Cloudflare/clef-flash',
 'c-jev':'https://typesafe.ai/blog/introducing-system-one-models-and-jev',
 'c-price-clef':'https://developers.cloudflare.com/workers-ai/models/clef/',
 'c-price-flash':'https://developers.cloudflare.com/workers-ai/models/clef-flash/',
 'c-pricing':'https://developers.cloudflare.com/workers-ai/platform/pricing/',
 'c-demo':'https://clef-evals.workers-ai-mle.workers.dev/',
 'c-clef-api':'https://huggingface.co/api/models/Cloudflare/clef',
 'c-flash-api':'https://huggingface.co/api/models/Cloudflare/clef-flash',
 'd-hn-clef':'https://hn.algolia.com/api/v1/search?query=Clef&tags=story',
 'd-hn-jev':'https://hn.algolia.com/api/v1/search?query=Jev&tags=story'}
 with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:list(pool.map(lambda x:fetch(*x),jobs.items()))
