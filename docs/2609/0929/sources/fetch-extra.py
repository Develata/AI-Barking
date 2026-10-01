import subprocess,json,datetime,pathlib,concurrent.futures,sys
from bs4 import BeautifulSoup
root=pathlib.Path('docs/0929/sources')
def go(item):
 name,url=item; t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(); fn=name+'.html'
 r=subprocess.run(['curl.exe','-L','-sS','--max-time','40','-o',str(root/fn),'-w','%{http_code} %{url_effective} %{content_type}',url],capture_output=True,text=True)
 if (root/fn).exists():
  s=BeautifulSoup((root/fn).read_text(encoding='utf8',errors='replace'),'html.parser'); meta=[dict(x.attrs) for x in s.select('meta[property],meta[name],time')]; ld=[x.get_text() for x in s.select('script[type="application/ld+json"]')]
  (root/(name+'-metadata.json')).write_text(json.dumps({'meta':meta,'jsonld':ld},ensure_ascii=False,indent=2),encoding='utf8')
  for x in s(['script','style','nav','footer','header']): x.decompose()
  (root/(name+'.txt')).write_text(s.get_text('\n',strip=True),encoding='utf8')
 return dict(time_bjt=t,url=url,tool='curl direct original URL',exit=r.returncode,result=r.stdout,error=r.stderr,file=fn)
items=json.loads((root/sys.argv[1]).read_text(encoding='utf-8-sig'))
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as ex: records=list(ex.map(go,items.items()))
with (root/'capture-records.jsonl').open('a',encoding='utf8') as f:
 for r in records:f.write(json.dumps(r,ensure_ascii=False)+'\n');print(r['file'],r['result'])
