from pathlib import Path
p=Path('docs/0930/sources/a-delivery-audit.py');s=p.read_text(encoding='utf-8-sig')
s=s.replace("assert not list(R.rglob('*_publish.txt')) and not (R/'images/README.md').exists() and not (P/'fact-check.md').exists()", "assert not (R/'images/README.md').exists() and not (P/'fact-check.md').exists()")
s=s.replace("'- 未生成发布稿、images/README.md、fact-check.md或ZIP。未复现任何模型评测。'", "'- 本次未生成发布稿、images/README.md、fact-check.md或ZIP。检查时发现本轮其他进程/协作者新增根目录doc_0930_publish.txt（创建北京时间9/30 23:57:54、最后写入23:58:13），未修改，排除本次交付清单。未复现任何模型评测。'")
s=s.replace("files=sorted(p for p in R.rglob('*') if p.is_file())", "files=sorted(p for folder in [P,R/'images'] for p in folder.rglob('*') if p.is_file())")
s=s.replace("所有路径均在docs/0930/内。", "所有路径均在docs/0930/sources或images内；排除非本任务生成的doc_0930_publish.txt。")
p.write_text(s,encoding='utf-8')
