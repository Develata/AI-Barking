import json,pathlib,datetime
P=pathlib.Path('docs/2610/1002/sources');now=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
rows=[('https://www.reddit.com/search/?q=Clef%20OR%20Jev','opencli reddit search; LocalLLaMA/top/month/12','FAILED TypeError: Failed to fetch; no social data archived'),('https://x.com/search?q=Clef%20from%3ACloudflare','opencli twitter search; limit5','FAILED TypeError: Failed to fetch; no social data archived')]
with (P/'c-capture-records.jsonl').open('a',encoding='utf-8') as f:
 for u,t,r in rows:f.write(json.dumps(dict(time=now,url=u,tool=t,result=r+'; time is post-operation log time',file=''))+'\n')
for name in ['c-cloudflare-browser-tables','c-index-app-browser','c-index-methodology-browser']:
 p=P/(name+'.json');s=p.read_text(encoding='utf-8-sig');d=json.JSONDecoder().raw_decode(s.strip())[0];p.write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8')
