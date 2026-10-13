"""1013 D 组 D8 补充 HN 热度：HN Algolia 关键词搜索 Epic/Disney 相关 story，写 d-heat-d8.json。用法：python -I d-heat-d8.py"""
import json, urllib.request, urllib.parse, datetime
qs = ["Epic Games Delays Disney Project", "Epic Games tells employees to use AI", "Epic Games AI first Disney"]
now = datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).strftime("%m-%d %H:%M")
out = {"fetched_beijing": now, "queries": {}}
for q in qs:
    u = "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=8&query=" + urllib.parse.quote(q)
    h = json.load(urllib.request.urlopen(u, timeout=30))["hits"]
    rows = [{k: x.get(k) for k in ("objectID", "points", "num_comments", "created_at", "title", "url")} for x in h if "Epic" in (x.get("title") or "")]
    out["queries"][q] = rows
    for r in rows:
        print(r["objectID"], r["points"], r["num_comments"], r["created_at"], (r["title"] or "")[:80], r["url"])
json.dump(out, open("d-heat-d8.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("fetched", now)
