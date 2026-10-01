import concurrent.futures,subprocess,json,datetime,pathlib
root=pathlib.Path('docs/0929/sources')
items={
'a-guardian':'https://www.theguardian.com/technology/2026/sep/28/metas-ai-agent-muse-home-address',
'a-bi-initial':'https://www.businessinsider.com/meta-muse-facebook-marketplace-address-story-matt-robb-2026-9',
'a-bi-update':'https://www.businessinsider.com/muse-agent-facebook-marketplace-address-setting-meta-always-allow-2026-9',
'a-pcmag':'https://www.pcmag.com/news/metas-muse-ai-agent-shared-someones-address-without-their-permission',
'a-meta':'https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/',
'b-aisi':'https://www.aisi.gov.uk/blog/gpt-6-astra-performs-unsanctioned-supply-chain-attacks-in-simulations',
'b-report':'https://cdn.prod.website-files.com/663bd486c5e4c81588db7a1d/6aba83e3772048bdd24df3d8_AISI_GPT-6_Astra_Technical_Report.pdf',
'b-system-card':'https://deploymentsafety.openai.com/gpt-6-astra/',
'b-openai-launch':'https://openai.com/index/gpt-6-astra/',
'c-wsj':'https://www.wsj.com/tech/ai/openai-chatgpt-model-release-cancel-safety-5a2f9f42',
'c-cbs':'https://www.cbsnews.com/news/openai-halts-gpt-astra-safety-concerns/',
'c-cna':'https://www.cna.com.tw/news/ait/202609290022.aspx'}
def fetch(kv):
 name,url=kv; fn=name+('.pdf' if name=='b-report' else '.html'); t=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat()
 p=subprocess.run(['curl.exe','-L','--max-time','45','-sS','-o',str(root/fn),'-w','%{http_code} %{url_effective} %{content_type}',url],capture_output=True,text=True)
 return {'time_bjt':t,'url':url,'tool':'curl direct original URL','exit':p.returncode,'result':p.stdout,'error':p.stderr,'file':fn}
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as ex: records=list(ex.map(fetch,items.items()))
with (root/'capture-records.jsonl').open('a',encoding='utf8') as f:
 for r in records: f.write(json.dumps(r,ensure_ascii=False)+'\n'); print(json.dumps(r,ensure_ascii=False))
try:
 from bs4 import BeautifulSoup
 for path in root.glob('*.html'):
  s=BeautifulSoup(path.read_text(encoding='utf8',errors='replace'),'html.parser')
  meta=[dict(x.attrs) for x in s.select('meta[property],meta[name],time')]
  ld=[x.get_text() for x in s.select('script[type="application/ld+json"]')]
  (root/(path.stem+'-metadata.json')).write_text(json.dumps({'meta':meta,'jsonld':ld},ensure_ascii=False,indent=2),encoding='utf8')
  for x in s(['script','style','nav','footer','header']): x.decompose()
  (root/(path.stem+'.txt')).write_text(s.get_text('\n',strip=True),encoding='utf8')
except ImportError: pass
