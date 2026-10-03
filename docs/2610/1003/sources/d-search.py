import subprocess,runpy,json,os
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');os.environ['OPENCLI_BROWSER_COMMAND_TIMEOUT']='150'
queries=[('d-x-search-focused',['twitter','search','"Muse Spark" OR "DAYJOB" since:2026-10-01','--limit','30']),('d-hn-meta',['hackernews','search','Solving Open Research Problems','--limit','10']),('d-hn-dayjob',['hackernews','search','DAYJOB','--sort','date','--limit','10']),('d-reddit',['reddit','search','Griffin Turing','--sort','top','--time','week','--limit','8'])]
for name,args in queries:
 r=subprocess.run(['opencli.exe',*args,'-f','json'],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=180);s=r.stdout.split('\n  Update available:')[0].strip()
 try:
  d=json.loads(s);m['P'].joinpath(name+'.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://'+('x.com/search' if args[0]=='twitter' else 'news.ycombinator.com' if args[0]=='hackernews' else 'reddit.com/search'),'agent-reach / OpenCLI '+args[0]+' search','query='+args[2]+'; 成功返回; 次数见文件',name+'.json');print(name,s[:6000],flush=True)
 except:m['log']('https://'+args[0]+'.com','OpenCLI search','失败 '+(s+r.stderr)[:500]);print(name,(s+r.stderr)[:500],flush=True)
