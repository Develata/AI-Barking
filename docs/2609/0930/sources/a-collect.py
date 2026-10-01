import requests,bs4,json,datetime,concurrent.futures,pathlib
P=pathlib.Path('docs/0930/sources')
urls={
'a-release':'https://openai.com/index/introducing-gpt-6-1-sol/',
'a-pricing':'https://openai.com/api/pricing/',
'a-platform-pricing':'https://platform.openai.com/docs/pricing',
'a-doc-pricing':'https://developers.openai.com/api/docs/pricing',
'a-sol-model':'https://developers.openai.com/api/docs/models/gpt-6-sol',
'a-sol61-model':'https://developers.openai.com/api/docs/models/gpt-6.1-sol',
'a-astra-model':'https://developers.openai.com/api/docs/models/gpt-6-astra',
'a-sol-launch':'https://openai.com/index/introducing-gpt-6-sol-and-luna/',
'a-devday-zh':'https://openai.com/zh-Hans-CN/index/devday-2026-recap/',
'a-devday-en':'https://openai.com/index/devday-2026-recap/',
'a-aa-article':'https://artificialanalysis.ai/articles/gpt-6-1-sol-replaces-gpt-6-sol-after-just-7-days-with-near-astra-intelligence',
'a-aa-leaderboard':'https://artificialanalysis.ai/leaderboards/models',
'a-aa-astra':'https://artificialanalysis.ai/models/gpt-6-astra',
'b-anthropic':'https://www.anthropic.com/research/glm-5-3-and-the-spread-of-advanced-cyber-capabilities',
'b-nist':'https://www.nist.gov/news-events/news/2026/09/caisis-assessment-zais-glm-53-cyber-capabilities',
'c-order':'https://www.whitehouse.gov/presidential-actions/2026/09/inaugurating-the-era-of-super-intelligence/',
'c-fact-sheet':'https://www.whitehouse.gov/fact-sheets/2026/09/fact-sheet-president-donald-j-trump-inaugurates-the-era-of-super-intelligence/',
'c-ai-gov':'https://www.ai.gov/',
'c-law':'https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title15-section9401',
'c-nist-si':'https://www.nist.gov/pml/owm/metric-si/si-units',
'c-sp330':'https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.330-2019.pdf',
'c-euronews':'https://www.euronews.com/2026/09/24/artificial-is-out-trump-orders-officials-to-call-it-super-intelligence-instead',
'a-hn-search':'https://hn.algolia.com/api/v1/search?query=GPT-6.1%20Sol&tags=story',
'b-hn-search':'https://hn.algolia.com/api/v1/search?query=GLM-5.3&tags=story'
}
def fetch(k,u):
 t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
 rec=dict(id=k,url=u,time_bjt=t,tool='Python requests (unauthenticated original-site GET)')
 try:
  r=requests.get(u,timeout=35);rec.update(status=r.status_code,final_url=r.url)
  typ=r.headers.get('content-type',''); ext='pdf' if 'application/pdf' in typ else 'json' if 'application/json' in typ else 'html'
  (P/(k+'.'+ext)).write_bytes(r.content)
  if ext=='html':
   s=bs4.BeautifulSoup(r.content,'html.parser'); meta=[str(x) for x in s.select('meta[property],script[type="application/ld+json"]')]
   for x in s(['script','style','noscript']):x.decompose()
   body=s.get_text('\n',strip=True)
   (P/(k+'.md')).write_text('URL: '+u+'\nFetched BJT: '+t+'\nHTTP: '+str(r.status_code)+'\nTitle: '+(s.title.get_text() if s.title else '')+'\n\n'+body,encoding='utf-8')
   (P/(k+'-metadata.json')).write_text(json.dumps(meta,ensure_ascii=False,indent=2),encoding='utf-8')
  rec['bytes']=len(r.content)
 except Exception as e:rec['error']=str(e)
 return rec
with concurrent.futures.ThreadPoolExecutor(max_workers=6) as ex:
 for rec in ex.map(lambda z:fetch(*z),urls.items()):
  with (P/'capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(rec,ensure_ascii=False)+'\n')
  print(json.dumps(rec,ensure_ascii=False),flush=True)
