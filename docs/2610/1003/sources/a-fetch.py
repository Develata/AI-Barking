import runpy,json,concurrent.futures
m=runpy.run_path('docs/2610/1003/sources/d-capture.py'); P=m['P'];a=json.loads((P/'a-meta-blog-browser.json').read_text(encoding='utf8'))
urls=[('a-paper-'+str(i+1),x['url']) for i,x in enumerate([x for x in a['links'] if '/research/publications/' in x['url']])]
urls += [('a-parallel-'+x,xurl) for x,xurl in [('misiakiewicz','https://arxiv.org/abs/2608.10184'),('delacerda','https://arxiv.org/abs/2608.12415'),('koehler','https://arxiv.org/abs/2608.27372'),('huwen','https://arxiv.org/abs/2609.25023')]]
urls += [('b-dayjob-pdf','https://arxiv.org/pdf/2610.01306'),('b-dayjob-html','https://arxiv.org/html/2610.01306v1')]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:list(ex.map(lambda x:m['fetch'](*x),urls))
