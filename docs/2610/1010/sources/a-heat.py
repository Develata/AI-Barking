"""1011 A 组：HN Algolia 热度。用法（在 sources 目录）：python a-heat.py -> a-heat.json（story 查询，含取得时刻）与 a-hn-search-oct.json（10/8 之后的 story/comment 与 url 字段查询）"""
import json, urllib.request, urllib.parse, datetime, time
UA = {"User-Agent": "Mozilla/5.0"}
def g(u):
    for i in range(3):
        try: return json.load(urllib.request.urlopen(urllib.request.Request(u, headers=UA), timeout=30))
        except Exception as e: err = {"error": str(e)}; time.sleep(2)
    return err
now = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
qs = ["damaging the task environment", "Damaging the task environment to trigger a reset", "OpenAI misalignment report grader", "alignment.openai.com misalignment-reports", "OpenAI grader deleted", "OpenAI misalignment reports", "grader fake input files reset OpenAI", "openai grading model damage environment"]
out = {"fetched_utc": now, "queries": {}}
for q in qs:
    for mode in ("search", "search_by_date"):
        r = g(f"https://hn.algolia.com/api/v1/{mode}?" + urllib.parse.urlencode({"query": q, "tags": "story", "hitsPerPage": 10}))
        out["queries"][mode + ": " + q] = [{k: h.get(k) for k in ("objectID", "title", "url", "points", "num_comments", "created_at")} for h in r.get("hits", [])] if "hits" in r else r
json.dump(out, open("a-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
res = {"fetched_utc": now}
for q, extra in [("damaging-the-task-environment-to-trigger-a-reset", {"restrictSearchableAttributes": "url"}), ("alignment.openai.com", {"restrictSearchableAttributes": "url"}), ("task environment reset grader", {}), ("OpenAI grader", {})]:
    for mode in ("search", "search_by_date"):
        for tags in ("story", "comment"):
            r = g(f"https://hn.algolia.com/api/v1/{mode}?" + urllib.parse.urlencode({"query": q, "tags": tags, "hitsPerPage": 15, "numericFilters": "created_at_i>1791300000", **extra}))
            res[f"{mode}/{tags}/{q}"] = [{k: h.get(k) for k in ("objectID", "title", "url", "points", "num_comments", "created_at", "story_id")} for h in r.get("hits", [])]
json.dump(res, open("a-hn-search-oct.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for k, v in res.items():
    if isinstance(v, list): print(k, len(v), [(h["objectID"], h["created_at"], h.get("url")) for h in v[:3]])
