import requests,pathlib,json,datetime,concurrent.futures,sys,subprocess
from bs4 import BeautifulSoup
P=pathlib.Path(__file__).parent; I=P.parent/'images'
def now():return datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
def log(url,tool,result,file=''):
 with (P/'d-capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(dict(time=now(),url=url,tool=tool,result=result,file=file),ensure_ascii=False)+'\n')
def fetch(name,url):
 try:
  r=requests.get(url,headers={'User-Agent':'Mozilla/5.0'},timeout=45); ct=r.headers.get('Content-Type',''); ext='pdf' if 'pdf' in ct else 'json' if 'json' in ct else 'gif' if 'gif' in ct else 'html'; file=name+'.'+ext
  (P/file).write_bytes(r.content); log(url,'requests GET without cookies',f'HTTP {r.status_code}; final={r.url}',file)
  if ext=='html':
   s=BeautifulSoup(r.content,'html.parser'); [e.decompose() for e in s(['script','style'])];(P/(name+'.txt')).write_text(s.get_text('\n',strip=True),encoding='utf-8')
  print(name,r.status_code,len(r.content),flush=True)
 except Exception as e:log(url,'requests GET',str(e));print(name,str(e),flush=True)
def call(*args):
 r=subprocess.run(['opencli.exe','browser','evidence1001',*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100)
 out=r.stdout.split('\n  Update available:')[0].strip()
 if r.returncode:raise RuntimeError((out+r.stderr)[:1200])
 try:return json.loads(out)
 except:return out
def archive(name,url):
 try:
  call('open',url); call('state')
  d=call('eval','JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll("meta[property],time")].map(e=>e.outerHTML),headings:[...document.querySelectorAll("h1,h2,h3")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),images:[...document.images].map(e=>({src:e.currentSrc,alt:e.alt,y:e.getBoundingClientRect().top+scrollY,h:e.height})),links:[...document.querySelectorAll("main a,article a")].map(e=>({text:e.innerText,url:e.href}))})')
  if isinstance(d,str):d=json.loads(d)
  (P/(name+'-browser.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8');log(url,'opencli browser open/state/eval','captured; validate contents',name+'-browser.json');print(name,d['title'],len(d['text']),flush=True);return d
 except Exception as e:log(url,'opencli browser',str(e));print(str(e),flush=True)
if __name__=='__main__':
 jobs={
 'a-google':'https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-4-argon/',
 'a-methodology':'https://deepmind.google/models/evals-methodology/gemini-4-argon',
 'a-table':'https://storage.googleapis.com/gweb-uniblog-publish-prod/original_images/gemini-4-argon_table_blog.gif',
 'a-aa':'https://artificialanalysis.ai/articles/gemini-4-argon-google-top-three-labs',
 'a-model':'https://artificialanalysis.ai/models/gemini-4-argon',
 'a-agents':'https://artificialanalysis.ai/agents/coding-agents/comparisons/antigravity-cli-vs-codex',
 'b-openai':'https://openai.com/index/disrupting-a-coordinated-model-distillation-campaign/',
 'b-arxiv':'https://arxiv.org/abs/2608.09867',
 'b-update':'https://stolen-thoughts.com/stolen_thoughts_update.pdf',
 'c-anthropic':'https://www.anthropic.com/research/what-work-can-robots-do',
 'c-yahoo':'https://finance.yahoo.com/technology/article/anthropic-study-suggests-blue-collar-workers-have-decades-before-robots-take-their-jobs-115957939.html',
 'd-hn-a':'https://hn.algolia.com/api/v1/search?query=Gemini%204%20Argon&tags=story',
 'd-hn-b':'https://hn.algolia.com/api/v1/search?query=distillation%20Moonshot&tags=story',
 'd-hn-c':'https://hn.algolia.com/api/v1/search?query=What%20work%20can%20robots%20do&tags=story'}
 with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:list(pool.map(lambda x:fetch(*x),jobs.items()))
