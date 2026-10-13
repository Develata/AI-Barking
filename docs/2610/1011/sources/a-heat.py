"""1012 A 组：HN 热度。用法（在 sources 目录）：python a-heat.py -> a-heat.json（取数时刻 UTC；50047422 的 Algolia/Firebase 读数、首页名次、相关 story 搜索）"""
import json, urllib.request, urllib.parse, datetime, time
UA = {"User-Agent": "Mozilla/5.0"}
def g(u):
    for i in range(3):
        try: return json.load(urllib.request.urlopen(urllib.request.Request(u, headers=UA), timeout=30))
        except Exception as e: err = {"error": str(e)}; time.sleep(2)
    return err
now = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
out = {"fetched_utc": now}
a = g("https://hn.algolia.com/api/v1/items/50047422")
out["algolia_item"] = {k: a.get(k) for k in ("id", "title", "url", "points", "created_at", "author")}
fb = g("https://hacker-news.firebaseio.com/v0/item/50047422.json")
out["firebase_item"] = {k: fb.get(k) for k in ("id", "title", "url", "score", "descendants", "time", "by")}
top = g("https://hacker-news.firebaseio.com/v0/topstories.json")
out["topstories_rank"] = (top.index(50047422) + 1) if isinstance(top, list) and 50047422 in top else None
out["searches"] = {}
for q in ["Karpathy shared understanding LLM capability", "gap in shared understanding of LLM capability", "karpathy funnel"]:
    for tags in ("story",):
        r = g("https://hn.algolia.com/api/v1/search_by_date?" + urllib.parse.urlencode({"query": q, "tags": tags, "hitsPerPage": 10, "numericFilters": "created_at_i>1791600000"}))
        out["searches"][q] = [{k: h.get(k) for k in ("objectID", "title", "url", "points", "num_comments", "created_at")} for h in r.get("hits", [])] if "hits" in r else r
json.dump(out, open("a-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: out[k] for k in ("fetched_utc", "algolia_item", "firebase_item", "topstories_rank")}, ensure_ascii=False))
for q, v in out["searches"].items(): print(q, [(h["objectID"], h["points"], h["num_comments"]) for h in v][:5])
