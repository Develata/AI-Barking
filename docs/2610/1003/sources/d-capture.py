import pathlib,subprocess,json,datetime,requests,sys
from bs4 import BeautifulSoup
P=pathlib.Path(__file__).parent

def now(): return datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
def log(url,tool,result,file=''):
 with (P/'d-capture-records.jsonl').open('a',encoding='utf8') as f: f.write(json.dumps(dict(time=now(),url=url,tool=tool,result=result,file=file),ensure_ascii=False)+'\n')
def clean(u):
 from urllib.parse import urlsplit,urlunsplit,parse_qsl,urlencode
 p=urlsplit(u); return urlunsplit((p.scheme,p.netloc,p.path,urlencode([(k,v) for k,v in parse_qsl(p.query) if not k.startswith('utm_')]),p.fragment))
def call(session,*args):
 r=subprocess.run(['opencli.exe','browser',session,*args],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=170)
 out=r.stdout.split('\n  Update available:')[0].strip()
 if r.returncode: raise RuntimeError((out+r.stderr)[:800])
 try:
  d=json.loads(out)
  return json.loads(d) if isinstance(d,str) and d.startswith(('{','[')) else d
 except: return out

def archive(name,url,session='ev1003a'):
 try:
  call(session,'open',url)
  d=call(session,'eval','JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,links:[...document.querySelectorAll("a")].map(e=>({text:e.innerText,url:e.href})),meta:[...document.querySelectorAll("meta[property],meta[name=citation_author],meta[name=citation_date],time")].map(e=>e.outerHTML)})')
  if not isinstance(d,dict): raise RuntimeError(str(d)[:500])
  d['links']=[dict(text=x['text'],url=clean(x['url'])) for x in d['links']]
  (P/(name+'-browser.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8')
  (P/(name+'.txt')).write_text(d['text'],encoding='utf8')
  log(url,'opencli browser open/eval','成功提取；内容另行核验',name+'-browser.json');print(name,d['title'],len(d['text']),flush=True);return d
 except Exception as e:log(url,'opencli browser',str(e));print(name,str(e),flush=True)
def fetch(name,url):
 try:
  r=requests.get(url,headers={'User-Agent':'Mozilla/5.0'},timeout=45)
  ext='pdf' if r.content[:4]==b'%PDF' else 'json' if 'json' in r.headers.get('content-type','') else 'html'
  (P/(name+'.'+ext)).write_bytes(r.content)
  log(url,'requests GET no cookies',f'HTTP {r.status_code}; final={clean(r.url)}',name+'.'+ext)
  if ext=='html':
   s=BeautifulSoup(r.content,'html.parser')
   for x in s(['script','style']):x.decompose()
   (P/(name+'.txt')).write_text(s.get_text('\n',strip=True),encoding='utf8')
  print(name,r.status_code,len(r.content),flush=True)
 except Exception as e:log(url,'requests GET',str(e));print(name,str(e),flush=True)
if __name__=='__main__':
 if sys.argv[1]=='browser':archive(*sys.argv[2:])
 else:fetch(*sys.argv[2:])
