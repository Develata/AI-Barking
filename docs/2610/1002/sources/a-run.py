import runpy,concurrent.futures
m=runpy.run_path('docs/2610/1002/sources/a-capture.py')
jobs=[('a-prototype','https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-prototype/'),('a-facts','https://blog.google/innovation-and-ai/models-and-research/google-research/google-project-suncatcher-facts/'),('a-joule','https://goo.gle/suncatcher-joule'),('a-npr','https://www.npr.org/2026/10/01/nx-s1-5983697/project-suncatcher-google-ai-data-center-space'),('d-a-hn','https://hn.algolia.com/api/v1/search?query=Suncatcher&tags=story&hitsPerPage=50')]
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:list(pool.map(lambda x:m['fetch'](*x),jobs))
for n,u in jobs[:2]:m['archive'](n,u)
