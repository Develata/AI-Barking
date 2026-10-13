"""1013 D 组 HN 关键词热度（补充）：HN Algolia 按标题关键词搜索 story，只保留标题或链接含关键词的命中。输出写 d-heat-search.json。用法：python -I d-heat-search.py"""
import json, urllib.request, urllib.parse, datetime
qs = {
 "D2": "X Subscription xAI Cursor",
 "D5": "AA-3D",
 "D6": "FrontierCode",
 "D8": "Epic Games AI first Disney",
 "D4": "Intel D270",
 "D9b": "Fourier transform OpenAI",
 "D10c": "Rich Hickey AI tool",
 "D1b": "Muse Wonder Meta",
 "D7b": "AI Models Can Provably Hide Arbitrary Capabilities",
}
now = datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).strftime("%m-%d %H:%M")
out = {"fetched_beijing": now, "queries": {}}
for k, q in qs.items():
    u = "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=8&query=" + urllib.parse.quote(q)
    try:
        h = json.load(urllib.request.urlopen(u, timeout=30))["hits"]
    except Exception as e:
        print("ERR", k, e); continue
    rows = [{kk: x.get(kk) for kk in ("objectID", "points", "num_comments", "created_at", "title", "url")} for x in h]
    out["queries"][k] = {"q": q, "rows": rows}
    print("==", k, q)
    for r in rows:
        print("  ", r["objectID"], r["points"], r["num_comments"], r["created_at"], (r["title"] or "")[:85], r["url"])
json.dump(out, open("d-heat-search.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("fetched", now)
