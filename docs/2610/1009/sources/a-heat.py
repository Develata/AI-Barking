"""1009 A 组：HN Algolia 热度。用法：python a-heat.py  -> a-heat.json（含取得时刻）"""
import json, urllib.request, urllib.parse, datetime, time
UA = {"User-Agent": "Mozilla/5.0"}
qs = ["Anthropic false homicide tip", "Anthropic unintended model actions", "investigating-unintended-model-actions", "nytimes.com anthropic-rogue-ai-agents", "Anthropic visa State Department", "Philadelphia police Anthropic", "Anthropic Claude government websites", "anthropic.com/research/investigating-unintended", "techcrunch anthropic false homicide tip", "reuters anthropic false homicide tip"]
out = {"fetched_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"), "queries": {}}
for q in qs:
    url = "https://hn.algolia.com/api/v1/search?" + urllib.parse.urlencode({"query": q, "tags": "story", "hitsPerPage": 10})
    for i in range(3):
        try:
            r = json.load(urllib.request.urlopen(urllib.request.Request(url, headers=UA), timeout=30)); break
        except Exception as e:
            r = {"error": str(e)}; time.sleep(2)
    out["queries"][q] = [{k: h.get(k) for k in ("objectID", "title", "url", "points", "num_comments", "created_at")} for h in r.get("hits", [])] if "hits" in r else r
json.dump(out, open("a-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
seen = {}
for q, hs in out["queries"].items():
    if isinstance(hs, list):
        for h in hs: seen[h["objectID"]] = h
for h in sorted(seen.values(), key=lambda h: -(h["points"] or 0)):
    if (h["created_at"] or "") >= "2026-10-08":
        print(h["objectID"], h["points"], h["num_comments"], h["created_at"], h["title"], h["url"])
