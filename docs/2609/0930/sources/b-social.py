import subprocess,json,pathlib,datetime
P=pathlib.Path('docs/0930/sources')
for name,pid in [('b-reddit-local','1wtg0vd'),('b-reddit-singularity','1wtkwkq'),('a-reddit-openai','1wtg4f3')]:
 t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
 r=subprocess.run(['opencli.exe','reddit','read',pid,'--limit','0','-f','json'],capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=100)
 out=r.stdout.split('\n  Update available:')[0].strip()
 # Adapter output is limited to public post/comments; no navigation/session state archived.
 (P/(name+'.json')).write_text(out,encoding='utf-8')
 with (P/'capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(dict(id=name,url='https://www.reddit.com/comments/'+pid+'/',tool='agent-reach route: opencli reddit read',time_bjt=t,status=r.returncode,error=r.stderr[:500] if r.returncode else ''),ensure_ascii=False)+'\n')
 print(name,r.returncode,out[:1300],flush=True)
