import runpy,subprocess,json
m=runpy.run_path('docs/2610/1002/sources/a-capture.py')
for name,args in [('d-a-x',['twitter','search','Suncatcher (from:Google OR from:GoogleResearch) since:2026-09-24','--limit','10','-f','json']),('d-a-reddit',['reddit','search','Suncatcher','--subreddit','singularity','--sort','new','--limit','6','-f','json'])]:
 try:
  r=subprocess.run(['opencli.exe',*args],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=150)
  out=r.stdout.split('\n  Update available:')[0];(m['P']/(name+'.json')).write_text(out,encoding='utf8');m['log']('platform search','opencli '+args[0],f'exit={r.returncode}',name+'.json');print(name,out[:500],r.stderr[:100],flush=True)
 except Exception as e:m['log']('platform search','opencli',str(e));print(e,flush=True)
