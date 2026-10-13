"""1013 D 组 HN 热度：按 story id 查 HN Algolia（tags=story_<id>），输出积分/评论数/发帖时刻/标题/链接，并写 d-heat.json。用法：python -I d-heat.py"""
import json, urllib.request, datetime
ids = {"D1": "50059115", "D3": "50054751", "D4": "50063365", "D7": "50065692",
       "D9": "50065932", "D10a": "50060269", "D10b": "50063427"}
now = datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).strftime("%m-%d %H:%M")
out = {"fetched_beijing": now, "items": {}}
for k, i in ids.items():
    u = f"https://hn.algolia.com/api/v1/search?tags=story_{i}"
    try:
        h = json.load(urllib.request.urlopen(u, timeout=30))["hits"]
    except Exception as e:
        print("ERR", k, i, e); continue
    rows = [{kk: x.get(kk) for kk in ("objectID", "points", "num_comments", "created_at", "title", "url", "author")} for x in h if str(x.get("objectID")) == i]
    out["items"][k] = rows
    for r in rows:
        print(k, r["objectID"], r["points"], r["num_comments"], r["created_at"], (r["title"] or "")[:90], r["url"])
json.dump(out, open("d-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("fetched", now)
