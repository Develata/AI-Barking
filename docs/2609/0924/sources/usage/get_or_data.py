import json,pathlib,urllib.request
p=pathlib.Path('0924/sources/usage')
for view in ['week','month']:
 u='https://openrouter.ai/api/frontend/v1/rankings/models?view='+view
 try:
  d=json.loads(urllib.request.urlopen(u,timeout=30).read());(p/('or-'+view+'-public.json')).write_text(json.dumps(d),encoding='utf-8')
  for r in d['data']:
   if any(s in r['model_permaslug'] for s in ['gpt-6','opus-5.5','fable-5.1']):print(view,r['model_permaslug'],r['variant'],r['rankingMetricValue'],r.get('date'))
 except Exception as e:print(type(e).__name__,str(e))
