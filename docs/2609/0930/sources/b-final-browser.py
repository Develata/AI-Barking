import sys,importlib,json
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture')
b.call('open','https://www.reddit.com/r/LocalLLaMA/comments/1wtg0vd/')
for name,pid in [('b-reddit-local-metrics','1wtg0vd'),('b-reddit-singularity-metrics','1wtkwkq'),('a-reddit-openai-metrics','1wtg4f3')]:
 t=b.stamp()
 js='fetch("/comments/'+pid+'.json?limit=1").then(r=>r.json()).then(d=>{let x=d[0].data.children[0].data;return JSON.stringify({id:x.id,title:x.title,score:x.score,num_comments:x.num_comments,created_utc:x.created_utc,permalink:x.permalink,removed_by_category:x.removed_by_category})})'
 try:
  d=b.call('eval',js);(b.P/(name+'.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf-8');b.log(dict(id=name,url='https://www.reddit.com/comments/'+pid+'.json?limit=1',time_bjt=t,tool='opencli browser eval same-origin public-post metrics only',status='success'));print(name,d,flush=True)
 except Exception as e:b.log(dict(id=name,url='https://www.reddit.com/comments/'+pid+'/',time_bjt=t,tool='opencli browser eval',error=str(e)[:700]));print(str(e)[:400],flush=True)
b.archive('a-sol-launch','https://openai.com/en-US/index/introducing-gpt-6-sol-and-luna/')
b.archive('d-b-gate','https://www.gate.com/zh/news/detail/anthropic-zhipu-glm-53-shows-end-to-end-network-exploitation-capabilities-24646425')
