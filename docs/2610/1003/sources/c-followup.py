import runpy,concurrent.futures
m=runpy.run_path('docs/2610/1003/sources/d-capture.py')
u=[('c-protos','https://protos.com/tavus-call-bot-sparks-ai-scam-psychosis-fears/'),('d-griffin-cn','https://www.aitoollab.cn/articles/ai-digital-human-tools-2026/'),('d-griffin-en','https://tech-ish.com/2026/10/02/tavus-griffin/'),('c-turing-original','https://academic.oup.com/mind/article/LIX/236/433/986238'),('a-nilradical-user','https://api.github.com/users/alunik'),('a-nilradical-commit','https://api.github.com/repos/alunik/kourovka-lean/commits/5a6b2c18e326b7b0281f00b64cade629acbfe1f5')]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:list(ex.map(lambda x:m['fetch'](*x),u))
