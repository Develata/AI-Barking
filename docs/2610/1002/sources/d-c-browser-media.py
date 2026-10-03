import runpy,json,datetime
m=runpy.run_path('docs/2610/1002/sources/a-capture.py');c=m['call'];P=m['P']
jobs=[('d-c-media-byte','https://byteiota.com/cloudflare-clef-decision-models-agents/'),('d-c-media-lavx','https://news.lavx.hu/zh-Hans/article/cloudflare-fa-bu-clef-jue-ce-mo-xing-ji-qiang-hua-xue-xi-wei-tiao-ping-tai'),('d-c-media-ai-blog','https://ai-blog.cloud/tool/cloudflare-clef-open-source-decision-model/'),('d-c-media-aistify','https://aistify.com/cloudflare-clef-clef-flash-open-decision-models/')]
for n,u in jobs:
 t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
 try:
  c('open',u);c('state');c('wait','selector','body');d=c('eval','JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll("meta[property],time")].map(e=>e.outerHTML)})');d['captured_bjt']=t;(P/(n+'-browser.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');(P/(n+'.txt')).write_text(d['text'],encoding='utf8');log={'time':t,'url':u,'result':'browser captured','file':n+'-browser.json'};print(n,d['title'],len(d['text']),flush=True)
 except Exception as e:log={'time':t,'url':u,'error':str(e)};print(e,flush=True)
 with (P/'d-c-capture-log.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps(log,ensure_ascii=False)+'\n')
