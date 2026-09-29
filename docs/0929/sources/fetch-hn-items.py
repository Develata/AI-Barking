import json,pathlib,datetime,subprocess
p=pathlib.Path('docs/0929/sources')
for item in ['49890748','49887152','49884367']:
 url='https://hacker-news.firebaseio.com/v0/item/'+item+'.json'; fn='a-hn-'+item+'.json'; t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat();r=subprocess.run(['curl.exe','-sS','--max-time','25','-o',str(p/fn),'-w','%{http_code}',url],capture_output=True,text=True)
 with (p/'capture-records.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps(dict(time_bjt=t,url=url,tool='curl HN official Firebase',exit=r.returncode,result=r.stdout,file=fn))+'\n')
 print(fn,(p/fn).read_text(encoding='utf8')[:450])
