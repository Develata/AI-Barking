import pathlib,json,datetime,requests
P=pathlib.Path('docs/0930/sources');ns={};exec((P/'a-collect.py').read_text(encoding='utf-8-sig').split('with concurrent.futures.ThreadPoolExecutor')[0],ns)
for k,u in [('b-zai-doc','https://docs.z.ai/guides/llm/glm-5.3'),('a-changelog','https://developers.openai.com/api/docs/changelog'),('a-hn-item','https://hacker-news.firebaseio.com/v0/item/49896586.json'),('b-hn-item','https://hacker-news.firebaseio.com/v0/item/49897075.json')]:
 rec=ns['fetch'](k,u)
 with (P/'capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(rec,ensure_ascii=False)+'\n')
 print(k,rec.get('status'),flush=True)
