import pathlib,json,concurrent.futures
P=pathlib.Path('docs/0930/sources');ns={};exec((P/'a-collect.py').read_text(encoding='utf-8-sig').split('with concurrent.futures.ThreadPoolExecutor')[0],ns)
urls={
'c-un-release':'https://www.whitehouse.gov/releases/2026/09/president-trump-at-the-united-nations-while-others-have-talked-i-have-acted/',
'd-b-decoder':'https://the-decoder.com/anthropic-says-zhipus-open-weight-glm-5-3-nearly-matches-claude-mythos-preview-at-building-exploits/',
'd-b-gate':'https://www.gate.com/zh/news/detail/anthropic-zhipu-glm-53-shows-end-to-end-network-exploitation-capabilities-24646425',
'd-b-madrobot':'https://madrobot.blog/2026/09/29/anthropic-glm-5-3-zai-cyber-exploits-safeguards-open-weight/',
'd-c-axios':'https://www.axios.com/2026/09/22/trump-ai-super-intelligence-rebrand'
}
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as ex:
 for rec in ex.map(lambda z:ns['fetch'](*z),urls.items()):
  with (P/'capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(rec,ensure_ascii=False)+'\n')
  print(rec['id'],rec.get('status'),rec.get('error',''),flush=True)
