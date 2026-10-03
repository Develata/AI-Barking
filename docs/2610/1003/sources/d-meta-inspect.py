import pathlib,json,re,bs4
p=pathlib.Path(__file__).parent
for n in ['c-protos','d-meta-sina','d-meta-primer','d-griffin-cn','d-griffin-en','b-surge-blog']:
 s=bs4.BeautifulSoup((p/(n+'.html')).read_text(encoding='utf8'),'html.parser');print(n,[(x.get('property') or x.get('name'),x.get('content')) for x in s.select('meta') if any(k in str(x) for k in ['published','modified','date'])][:10]);print([x.get_text()[:300] for x in s.select('script[type="application/ld+json"]')][:1])
for n in ['a-nilradical-result','a-nilradical-site']:
 t=(p/(n+'.txt')).read_text(encoding='utf8');print(n,t[:6500])
