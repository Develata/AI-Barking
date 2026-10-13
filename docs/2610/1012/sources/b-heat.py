"""1013 B 组：HN 热度（改自 1012 a-heat.py）。用法（在 sources 目录）：python -I b-heat.py -> b-heat.json（取数时刻 UTC；50061603 的 Algolia/Firebase 读数、首页名次、相关 story 搜索）"""
import json, urllib.request, urllib.parse, datetime, time
UA = {"User-Agent": "Mozilla/5.0"}
def g(u):
    for i in range(3):
        try: return json.load(urllib.request.urlopen(urllib.request.Request(u, headers=UA), timeout=30))
        except Exception as e: err = {"error": str(e)}; time.sleep(2)
    return err
ID = 50061603
now = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
out = {"fetched_utc": now}
a = g(f"https://hn.algolia.com/api/v1/items/{ID}")
out["algolia_item"] = {k: a.get(k) for k in ("id", "title", "url", "points", "created_at", "author")}
fb = g(f"https://hacker-news.firebaseio.com/v0/item/{ID}.json")
out["firebase_item"] = {k: fb.get(k) for k in ("id", "title", "url", "score", "descendants", "time", "by")}
top = g("https://hacker-news.firebaseio.com/v0/topstories.json")
out["topstories_rank"] = (top.index(ID) + 1) if isinstance(top, list) and ID in top else None
out["searches"] = {}
for q in ["self-replicating datacenter anthropic", "Anthropic self-replicating AI IoT", "AI worm smart TV"]:
    r = g("https://hn.algolia.com/api/v1/search_by_date?" + urllib.parse.urlencode({"query": q, "tags": "story", "hitsPerPage": 10, "numericFilters": "created_at_i>1791700000"}))
    out["searches"][q] = [{k: h.get(k) for k in ("objectID", "title", "url", "points", "num_comments", "created_at")} for h in r.get("hits", [])] if "hits" in r else r
json.dump(out, open("b-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: out[k] for k in ("fetched_utc", "algolia_item", "firebase_item", "topstories_rank")}, ensure_ascii=False))
for q, v in out["searches"].items(): print(q, [(h["objectID"], h["title"][:50], h["points"], h["num_comments"]) for h in v][:6])
