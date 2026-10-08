"""1007 A 组热度：HN Algolia 搜索 + Reddit 搜索（公开 JSON，未登录）。输出 a-hn-search.json / a-reddit-search.json，并打印摘要。"""
import json, urllib.request, urllib.parse, datetime, sys
UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36"
def get(u):
    req = urllib.request.Request(u, headers={"User-Agent": UA})
    return json.load(urllib.request.urlopen(req, timeout=40))
def bj(ts): return (datetime.datetime.fromtimestamp(ts, datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).strftime("%m-%d %H:%M BJ")
now = (datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).strftime("%Y-%m-%d %H:%M BJ")
out = {"fetched_bj": now, "hn": {}, "reddit": {}}
for q in ["nolla", "utah acne AI", "AI prescriptions Utah", "Utah AI prescribe", "Doctronic Utah"]:
    try:
        j = get("https://hn.algolia.com/api/v1/search?" + urllib.parse.urlencode({"query": q, "tags": "story", "numericFilters": "created_at_i>1790900000", "hitsPerPage": 20}))
        out["hn"][q] = [{"id": h["objectID"], "points": h["points"], "comments": h["num_comments"], "time": bj(h["created_at_i"]), "title": h["title"], "url": h.get("url")} for h in j["hits"]]
        print("HN", q, j["nbHits"])
        for h in out["hn"][q]: print("  ", h["id"], h["points"], h["comments"], h["time"], h["title"][:90], h["url"])
    except Exception as e:
        print("HN fail", q, e)
for q in ["Nolla Health", "Utah AI prescription acne"]:
    try:
        j = get("https://old.reddit.com/search.json?" + urllib.parse.urlencode({"q": q, "sort": "relevance", "t": "week", "limit": 25}))
        out["reddit"][q] = [{"sub": c["data"]["subreddit"], "score": c["data"]["score"], "comments": c["data"]["num_comments"], "time": bj(c["data"]["created_utc"]), "title": c["data"]["title"], "permalink": "https://www.reddit.com" + c["data"]["permalink"], "url": c["data"]["url"]} for c in j["data"]["children"]]
        print("REDDIT", q)
        for h in out["reddit"][q]: print("  ", h["sub"], h["score"], h["comments"], h["time"], h["title"][:80], h["permalink"])
    except Exception as e:
        print("Reddit fail", q, e)
json.dump(out, open("a-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
