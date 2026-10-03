import requests,pathlib,json,datetime
p=pathlib.Path(__file__).parent
for name,url,ext in [('a-original-optimization-v1','https://arxiv.org/pdf/2507.12831v1','pdf'),('a-kida-metadata','https://api.crossref.org/works/10.1515/jgth-2024-0010','json'),('a-algebra-metadata','https://api.crossref.org/works/10.1007/s00013-026-02251-0','json')]:
 try:
  r=requests.get(url,headers={'User-Agent':'Mozilla/5.0'},timeout=30);(p/(name+'.'+ext)).write_bytes(r.content)
  with (p/'d-capture-records.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps({'time':datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(),'url':url,'tool':'requests public GET','result':str(r.status_code),'file':name+'.'+ext})+'\n')
  print(name,r.status_code)
 except Exception as e:print(name,type(e).__name__)
