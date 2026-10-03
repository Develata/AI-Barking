import subprocess,runpy,json
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
r=subprocess.run(['opencli.exe','reddit','read','1wv7q40','--sort','top','--limit','12','--depth','1','-f','json'],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=180)
s=r.stdout.split('\n  Update available:')[0].strip()
try:
 d=json.loads(s);(p/'d-reddit-comments.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://www.reddit.com/r/singularity/comments/1wv7q40/','OpenCLI reddit read','top 12 comments; public fields','d-reddit-comments.json');print(s[:8000])
except:m['log']('https://www.reddit.com/r/singularity/comments/1wv7q40/','OpenCLI reddit read','失败 '+s[:500]);print(s[:500])
