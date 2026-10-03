import requests,pathlib,json,datetime,concurrent.futures
from bs4 import BeautifulSoup
P=pathlib.Path('docs/2610/1002/sources')
def fetch(job):
 n,u=job;t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
 try:
  r=requests.get(u,timeout=35);(P/(n+'.html')).write_bytes(r.content);s=BeautifulSoup(r.content,'html.parser');[e.decompose() for e in s(['script','style'])];(P/(n+'.txt')).write_text(s.get_text('\n',strip=True),encoding='utf8');log={'time':t,'url':u,'final':r.url,'http':r.status_code,'file':n+'.html'};print(n,r.status_code,flush=True)
 except Exception as e:log={'time':t,'url':u,'error':str(e)}
 with (P/'d-c-capture-log.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps(log,ensure_ascii=False)+'\n')
jobs=[('d-c-media-byte','https://byteiota.com/cloudflare-clef-decision-models-agents/'),('d-c-media-lavx','https://news.lavx.hu/zh-Hans/article/cloudflare-fa-bu-clef-jue-ce-mo-xing-ji-qiang-hua-xue-xi-wei-tiao-ping-tai'),('d-c-media-ai-blog','https://ai-blog.cloud/tool/cloudflare-clef-open-source-decision-model/'),('d-c-media-aistify','https://aistify.com/cloudflare-clef-clef-flash-open-decision-models/')]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:list(pool.map(fetch,jobs))
d=json.loads((P/'d-c-jev-hn.json').read_text());print([(x['objectID'],x['title'],x['points'],x['num_comments'],x['created_at']) for x in d['hits'] if '49717558'==x['objectID']])
