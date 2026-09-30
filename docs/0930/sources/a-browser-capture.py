import subprocess,json,pathlib,datetime,sys,time
P=pathlib.Path('docs/0930/sources'); I=P.parent/'images'
SESSION='evidence0930'
def call(*args):
 r=subprocess.run(['opencli.exe','browser',SESSION,*args],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100)
 out=r.stdout.split('\n  Update available:')[0].strip()
 if r.returncode:raise RuntimeError((out+r.stderr)[:1500])
 try:return json.loads(out)
 except:return out
def stamp():return datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
def log(rec):
 with (P/'capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(rec,ensure_ascii=False)+'\n')
def archive(name,url=None):
 t=stamp()
 if url:call('open',url)
 data=call('eval','JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll("meta[property],script[type=\\"application/ld+json\\"],time")].map(e=>e.outerHTML),headings:[...document.querySelectorAll("h1,h2,h3")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),figures:[...document.querySelectorAll("figure")].map((e,i)=>({i,text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height}))})')
 if isinstance(data,str):data=json.loads(data)
 (P/(name+'-browser.json')).write_text(json.dumps(data,ensure_ascii=False,indent=2),encoding='utf-8')
 log(dict(id=name,url=url or data['url'],final_url=data['url'],time_bjt=t,tool='opencli browser eval; public body.innerText + metadata',status='success'))
 return data
def shot(name,y,height=900):
 call('eval',f'window.scrollTo({{top:{max(0,y)},behavior:"instant"}}); true')
 call('eval','new Promise(r=>setTimeout(r,1500))')
 result=call('screenshot',str((I/name).resolve()),'--width','1266','--height',str(height))
 log(dict(id=name,url=call('eval','location.href'),time_bjt=stamp(),tool='opencli browser screenshot',status='success',file='images/'+name))
 print(name,result,flush=True)
if __name__=='__main__':
 d=archive('a-release')
 shot('01-sol-title.png',220,1000)
 for name,term,h in [('02-sol-price-availability.png','Pricing and availability',920),('03-sol-deepswe.png','Coding',1050),('04-sol-osworld.png','Computer use',1080),('05-sol-science.png','Scientific research',1160)]:
  d=archive('a-release-layout-'+name[:2]); y=next(x['y'] for x in d['headings'] if x['text']==term);shot(name,y-90,h)

