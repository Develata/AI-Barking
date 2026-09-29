from pathlib import Path
import json,subprocess,datetime
p=Path('docs/0929/sources'); raw=(p/'a-robb-tweets.json').read_text(encoding='utf-8-sig'); start=raw.find('[\n'); data,end=json.JSONDecoder().raw_decode(raw[start:]); relevant=[x for x in data if x.get('created_at','').startswith(('Sun Sep 27','Mon Sep 28','Tue Sep 29'))]
(p/'a-robb-selected.json').write_text(json.dumps(relevant,ensure_ascii=False,indent=2),encoding='utf8')
for x in relevant:
 if x['id'] not in ['2104090601411293303','2104397204014125301','2104395979872981192','2104396139587879234']:continue
 print(x['id'],x['created_at'],x['likes'],x['retweets'],x['views'],x['text'])
 for i,u in enumerate(x['media_urls']):
  name='a-robb-'+x['id']+'-'+str(i+1)+'.jpg'; t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(); r=subprocess.run(['curl.exe','-L','-sS','--max-time','30','-o',str(p/name),'-w','%{http_code}',u],capture_output=True,text=True)
  with (p/'capture-records.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps(dict(time_bjt=t,url=u,tool='curl original X media',exit=r.returncode,result=r.stdout,file=name))+'\n')
