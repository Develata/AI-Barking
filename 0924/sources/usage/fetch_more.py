import pathlib,json,subprocess,concurrent.futures
out=pathlib.Path('0924/sources/usage')
urls={'epoch-tiers123':'https://epoch.ai/benchmarks/frontiermath-tiers-1-3-v2','epoch-data':'https://epoch.ai/benchmarks/use-this-data','matharena-models':'https://matharena.ai/models','openai-astra':'https://openai.com/index/gpt-6-astra/','anthropic-flt':'https://www.anthropic.com/research/formalizing-fermats-last-theorem','counterexample':'https://arxiv.org/html/2608.29595v1'}
def get(kv):
 k,v=kv;p=subprocess.run(['curl.exe','-sS','-L','--max-time','90','https://r.jina.ai/'+v],capture_output=True,encoding='utf-8',errors='replace');(out/(k+'.txt')).write_text(p.stdout,encoding='utf-8');return k,len(p.stdout),p.returncode
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as ex: print(list(ex.map(get,urls.items())))
