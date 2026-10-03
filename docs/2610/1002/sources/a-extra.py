import runpy,concurrent.futures
m=runpy.run_path('docs/2610/1002/sources/a-capture.py')
jobs=[('a-research','https://research.google/blog/exploring-a-space-based-scalable-ai-infrastructure-system-design/'),('a-paper','https://www.cell.com/joule/pdf/S2542-4351(26)00362-4.pdf'),('d-a-hn-comments','https://hn.algolia.com/api/v1/items/49830606'),('a-planet-new','https://investors.planet.com/news/news-details/2026/Planet-Launches-Suncatcher-Tanager-2-and-18-SuperDove-Satellites/default.aspx')]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:list(pool.map(lambda x:m['fetch'](*x),jobs))
m['archive']('a-spacex','https://www.spacex.com/launches/transporter18/')
m['archive']('d-a-aihot','https://aihot.news')
