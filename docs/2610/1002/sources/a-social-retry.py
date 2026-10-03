import subprocess,runpy
m=runpy.run_path('docs/2610/1002/sources/a-capture.py')
for name,args in [('d-a-x',['twitter','search','Suncatcher from:GoogleResearch','--limit','5','-f','json']),('d-a-reddit',['reddit','search','Suncatcher','--subreddit','singularity','--sort','new','--limit','5','-f','json'])]:
 r=subprocess.run(['opencli.exe',*args],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=150)
 (m['P']/(name+'-error.txt')).write_text(r.stderr,encoding='utf8');(m['P']/(name+'.json')).write_text(r.stdout.split('\n  Update available:')[0],encoding='utf8');print(name,r.returncode,r.stderr[-1700:],flush=True)
