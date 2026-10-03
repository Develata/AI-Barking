import runpy,concurrent.futures
m=runpy.run_path('docs/2610/1003/sources/d-capture.py')
u=[('a-original-pde','https://arxiv.org/abs/1503.01741'),('a-original-pde-full','https://arxiv.org/pdf/1503.01741'),('a-nilradical-site','https://nilradical.ai/'),('a-nilradical-result','https://nilradical.ai/results/kourovka-21-68/'),('a-original-optimization-full','https://arxiv.org/pdf/2507.12831')]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:list(ex.map(lambda x:m['fetch'](*x),u))
