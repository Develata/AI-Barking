"""1013 C 组：HN 50058507 热度快照。用法（在仓库根目录）：python3 -I docs/2610/1013/sources/c-heat.py -> docs/2610/1013/sources/c-heat.json"""
import json, urllib.request, datetime
def get(u): return json.load(urllib.request.urlopen(urllib.request.Request(u, headers={"User-Agent": "Mozilla/5.0"}), timeout=40))
now = datetime.datetime.now(datetime.timezone.utc)
d = get("https://hn.algolia.com/api/v1/items/50058507")
def cnt(n): return sum(1 + cnt(c) for c in n.get("children", []))
fb = get("https://hacker-news.firebaseio.com/v0/item/50058507.json")
out = {"fetched_utc": now.isoformat(timespec="seconds"), "fetched_bj": (now + datetime.timedelta(hours=8)).replace(tzinfo=None).isoformat(timespec="seconds") + "+08:00",
       "hn_algolia": {"title": d["title"], "url": d["url"], "points": d["points"], "comments_counted": cnt(d), "top_level": len(d["children"]), "created_at": d["created_at"]},
       "hn_firebase": {"score": fb["score"], "descendants": fb["descendants"], "time": fb["time"]}}
json.dump(out, open("docs/2610/1013/sources/c-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps(out, ensure_ascii=False))
