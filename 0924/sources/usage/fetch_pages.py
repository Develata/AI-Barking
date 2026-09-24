import concurrent.futures,subprocess,pathlib,json,datetime
out=pathlib.Path('0924/sources/usage')
urls={'arena-text':'https://arena.ai/leaderboard/text','arena-code':'https://arena.ai/leaderboard/code','arena-coding':'https://arena.ai/leaderboard/text/coding','openrouter-rankings':'https://openrouter.ai/rankings','epoch-tier4':'https://epoch.ai/benchmarks/frontiermath-tier-4-v2','matharena':'https://matharena.ai/'}
def get(item):
 name,url=item; p=subprocess.run(['curl.exe','-L','--max-time','100','-sS','https://r.jina.ai/'+url],capture_output=True,encoding='utf-8',errors='replace'); (out/(name+'.txt')).write_text(p.stdout,encoding='utf-8'); return {'name':name,'url':url,'time':datetime.datetime.now(datetime.timezone.utc).isoformat(),'returncode':p.returncode,'length':len(p.stdout),'stderr':p.stderr}
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as ex:
 r=list(ex.map(get,urls.items()))
(out/'web-fetch-ledger.json').write_text(json.dumps(r,ensure_ascii=False,indent=2),encoding='utf-8');print(json.dumps(r,ensure_ascii=False))
