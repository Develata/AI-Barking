import runpy,concurrent.futures,json
m=runpy.run_path('docs/2610/1002/sources/a-capture.py')
jobs=[('a-original','https://blog.google/innovation-and-ai/technology/research/google-project-suncatcher/'),('a-spacex','https://www.spacex.com/launches/transporter18/'),('a-planet-old','https://www.planet.com/pulse/planet-to-build-and-operate-advanced-space-platform-for-google-s-project-suncatcher-moonshot/'),('a-misread-en','https://news.cocoloop.cn/en/2026/10/google-suncatcher-tpu-in-orbit/'),('a-misread-zh','https://news.cocoloop.cn/2026/10/google-suncatcher-tpu-in-orbit/')]
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:list(pool.map(lambda x:m['fetch'](*x),jobs))
print(m['call']('eval','JSON.stringify({dpr:devicePixelRatio,w:innerWidth,h:innerHeight})'))
m['archive']('a-joule','https://www.cell.com/joule/fulltext/S2542-4351(26)00362-4')
m['archive']('a-npr','https://www.npr.org/2026/10/01/nx-s1-5983697/project-suncatcher-google-ai-data-center-space')
