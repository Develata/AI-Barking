import requests,runpy,concurrent.futures
m=runpy.run_path('docs/2610/1003/sources/d-capture.py');p=m['P']
def f(n):
 url='https://arxiv.org/src/2610.01306v1/anc/data/'+n+'.csv'
 try:
  r=requests.get(url,headers={'User-Agent':'Mozilla/5.0'},timeout=30);name='b-'+n+'.csv';(p/name).write_bytes(r.content);m['log'](url,'requests GET',str(r.status_code),name);print(name,r.status_code,r.text[:1000])
 except Exception as e:m['log'](url,'requests GET',str(e))
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as ex:list(ex.map(f,['leaderboard_healthcare','leaderboard_finance','human_time_ranges']))
