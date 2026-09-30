import pathlib,bs4,json,requests,datetime
P=pathlib.Path('docs/0930/sources')
# Reuse the fetch function without executing the initial URL batch.
ns={};exec((P/'a-collect.py').read_text(encoding='utf-8-sig').split('with concurrent.futures.ThreadPoolExecutor')[0],ns)
extra={
'a-aa-sol-release':'https://artificialanalysis.ai/models/releases/gpt-6-1-sol',
'b-hf-model':'https://huggingface.co/zai-org/GLM-5.3',
'b-hf-license':'https://huggingface.co/zai-org/GLM-5.3/raw/main/LICENSE',
'b-zai-blog':'https://z.ai/blog/glm-5.3',
'b-glasswing':'https://www.anthropic.com/glasswing',
'c-law-text':'https://uscode.house.gov/view.xhtml?req=title:15%20section:9401%20edition:prelim',
'c-sp330-history':'https://www.nist.gov/pml/special-publication-330/sp-330-version-history',
'c-sp330-current':'https://www.nist.gov/pml/special-publication-330',
'd-a-ithome':'https://www.ithome.com/1/008/527.htm',
'd-a-vellum':'https://website.vellum.ai/blog/gpt-6-1-sol-benchmarks-explained',
'd-c-eastmoney':'https://finance.eastmoney.com/a/202609303887071146.html'
}
import concurrent.futures
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as ex:
 for rec in ex.map(lambda z:ns['fetch'](*z),extra.items()):
  with (P/'capture-records.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(rec,ensure_ascii=False)+'\n')
  print(json.dumps(rec,ensure_ascii=False),flush=True)
