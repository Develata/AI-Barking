import json,pathlib,subprocess,datetime
p=pathlib.Path('docs/0929/sources'); url='https://hn.algolia.com/api/v1/search?query=Muse%20address&tags=story'; t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat();r=subprocess.run(['curl.exe','-sS','--max-time','30','-o',str(p/'a-hn-search.json'),'-w','%{http_code}',url],capture_output=True,text=True)
with (p/'capture-records.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps(dict(time_bjt=t,url=url,tool='curl HN Algolia public search',exit=r.returncode,result=r.stdout,file='a-hn-search.json'))+'\n')
d=json.loads((p/'a-hn-search.json').read_text(encoding='utf8'));print([(x['objectID'],x['title'],x['points'],x['num_comments'],x['created_at']) for x in d['hits'][:8]])
