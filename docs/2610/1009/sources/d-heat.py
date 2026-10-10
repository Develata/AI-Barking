"""D 组 HN Algolia 热度：按 URL 子串/标题查询，输出 id/分数/评论/时刻。用法：python -I d-heat.py"""
import json, urllib.request, urllib.parse
qs = ["epoch.ai/publications/innovationeval","commandline.microsoft.com/microsoft-decision-1","terrytao.wordpress.com/2026/10/09","artificialanalysis.ai/evaluations/harvey-lab-aa","arena.ai/blog/ai-alignment-index","jessewaites.com","deno.com/blog/cloudflare","blog.cloudflare.com/deno-joins-cloudflare","Lean theorem prover reliability Hales","Harvey LAB-AA v1.1","Arena Alignment Index","Deno Cloudflare"]
out = {}
for q in qs:
    u = "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=6&query=" + urllib.parse.quote(q)
    h = json.load(urllib.request.urlopen(u))["hits"]
    out[q] = [{k: x.get(k) for k in ("objectID","points","num_comments","created_at","title","url")} for x in h]
    print("==", q)
    for x in out[q]:
        print(x["objectID"], x["points"], x["num_comments"], x["created_at"], (x["title"] or "")[:70], x["url"])
json.dump(out, open("d-heat.json","w",encoding="utf-8"), ensure_ascii=False, indent=1)
