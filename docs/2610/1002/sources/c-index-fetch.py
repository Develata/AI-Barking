import requests,pathlib,json,datetime
P=pathlib.Path(__file__).parent
jobs=[('c-index-readme','https://huggingface.co/spaces/multimodalart/jev-decision-index/raw/main/README.md'),('c-index-api','https://huggingface.co/api/spaces/multimodalart/jev-decision-index')]
for n,u in jobs:
 try:
  r=requests.get(u,headers={'User-Agent':'Mozilla/5.0'},timeout=25);(P/(n+'.txt')).write_text(r.text,encoding='utf-8');result=str(r.status_code)
 except Exception as e:result=str(e)
 with (P/'c-capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(dict(time=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(),url=u,tool='requests GET without cookies',result=result,file=n+'.txt'))+'\n')
 print(n,result[:150])
