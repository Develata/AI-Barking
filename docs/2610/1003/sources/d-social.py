import runpy,json,subprocess,os
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');os.environ['OPENCLI_BROWSER_COMMAND_TIMEOUT']='150'
q='(from:AIatMeta research) OR (from:tavus Griffin) OR DAYJOB since:2026-09-30'
r=subprocess.run(['opencli.exe','twitter','search',q,'--limit','30','-f','json'],capture_output=True,text=True,encoding='utf8',errors='replace',timeout=200)
# Adapter results contain public posts only; exclude arbitrary stderr/account UI.
s=r.stdout.split('\n  Update available:')[0].strip()
try:
 d=json.loads(s);m['P'].joinpath('d-x-search.json').write_text(json.dumps(d,ensure_ascii=False,indent=2),encoding='utf8');m['log']('https://x.com/search?q='+q,'agent-reach / OpenCLI twitter search','成功；30条上限；public posts only','d-x-search.json');print(s[:22000])
except:m['log']('https://x.com/search','OpenCLI twitter search','失败 '+(s+r.stderr)[:600]);print((s+r.stderr)[:1000])
