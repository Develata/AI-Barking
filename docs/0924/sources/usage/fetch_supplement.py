import concurrent.futures, subprocess, pathlib, json, datetime
p=pathlib.Path(__file__).resolve().parent
urls={'arena-method':'https://arena.ai/blog/arena-rank','arena-rank-explanation':'https://arena.ai/blog/ranking-method','boris-official':'https://www.anthropic.com/webinars/claude-code-service-delivery','sawhney-sellke-paper':'https://arxiv.org/abs/2604.06609','sra-abstract':'https://arxiv.org/abs/2608.29595','epoch-erdos-paper':'https://arxiv.org/html/2609.25050v1'}
def fetch(kv):
 k,u=kv;r=subprocess.run(['curl.exe','-sS','-L','--max-time','70','https://r.jina.ai/'+u],capture_output=True,encoding='utf-8',errors='replace');(p/(k+'.txt')).write_text(r.stdout,encoding='utf-8');return dict(file=k+'.txt',url=u,captured_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),returncode=r.returncode,length=len(r.stdout),error=r.stderr)
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool: results=list(pool.map(fetch,urls.items()))
(p/'supplement-fetch-ledger.json').write_text(json.dumps(results,indent=2),encoding='utf-8')
print(results)
