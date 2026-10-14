import sys,json,pyarrow.parquet as pq
sp=sys.argv[1]
out={"source":"https://huggingface.co/datasets/lmarena-ai/leaderboard-dataset (Arena's own HF dataset, split=latest; HF dataset lastModified 2026-10-14T03:05:44Z)","fetched_bj":"2026-10-14T17:20+08:00","note":"rows copied unchanged; ratings are Arena Score (Bradley-Terry). text_style_control = Arena default view since 2025-05-16 per dataset README."}
for name in ('text','text_style_control','webdev','search','document'):
    d=pq.read_table(f"{sp}/{name}-latest.parquet" if name!='x' else '').to_pylist()
    ov=sorted([r for r in d if r['category']=='overall'],key=lambda r:-r['rating'])
    out[name]={"n_rows":len(d),"publish_dates":sorted(set(r['leaderboard_publish_date'] for r in d)),"overall_top15":ov[:15],"argon_rows":[r for r in d if 'argon' in r['model_name'].lower()]}
json.dump(out,open(sys.argv[2],'w',encoding='utf-8'),ensure_ascii=False,indent=1,default=str)
