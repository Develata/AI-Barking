import runpy,concurrent.futures
m=runpy.run_path('docs/2610/1002/sources/c-capture.py',run_name='helper')
jobs={'c-demo-app':'https://clef-evals.workers-ai-mle.workers.dev/assets/index-Ddq_AEzH.js','c-index-news':'https://multimodalart-jev-decision-index.static.hf.space/news.html','c-register-html':'https://www.theregister.com/ai-and-ml/2026/10/01/cloudflare-tries-to-outplay-jev-with-open-weight-clef-models/5300649','c-clef-license-text':'https://huggingface.co/Cloudflare/clef/raw/main/LICENSE','c-flash-license-text':'https://huggingface.co/Cloudflare/clef-flash/raw/main/LICENSE'}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as p:list(p.map(lambda x:m['fetch'](*x),jobs.items()))
