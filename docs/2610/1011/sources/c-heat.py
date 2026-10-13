"""1012 C 组：热度快照。用法（在 sources 目录）：python c-heat.py -> c-heat.json（HN story points/comments、GitHub 各 Craft 仓库 stars/forks/pushed_at、Bluesky 帖计数，含取数时刻）。GitHub 通过 gh api 调用（匿名公开数据）。"""
import json, urllib.request, datetime, subprocess
UA = {"User-Agent": "Mozilla/5.0"}
def get(u):
    return json.load(urllib.request.urlopen(urllib.request.Request(u, headers=UA), timeout=40))
now = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
out = {"fetched_utc": now, "hn": {}, "github": {}, "bsky": {}}
for i in (49958850, 49993722, 49981449, 50010170, 49986072, 49988881, 50004284, 50050370, 49938443, 49982597, 49989811, 49960141):
    try:
        d = get(f"https://hn.algolia.com/api/v1/items/{i}")
        def cnt(n): return sum(1 + cnt(c) for c in n.get("children", []))
        out["hn"][i] = {"title": d.get("title"), "url": d.get("url"), "points": d.get("points"), "comments_counted": cnt(d), "created_at": d.get("created_at")}
    except Exception as e:
        out["hn"][i] = {"error": str(e)}
repos = ["photocraft","vectorcraft","filmcraft","lightcraft","pdfcraft","effectcraft","designcraft","wordcraft","soundcraft","deckcraft","cadcraft","gridcraft","craft-launcher","craft-libs","artcraft","artcraft-services","craft-fonts","photocraft-corpus"]
for r in repos:
    j = json.loads(subprocess.run(["gh","api",f"repos/storytold/{r}"],capture_output=True,text=True,encoding="utf-8").stdout)
    out["github"][r] = {k: j.get(k) for k in ("stargazers_count","forks_count","open_issues_count","created_at","pushed_at")} | {"license": (j.get("license") or {}).get("spdx_id"), "description": j.get("description")}
try:
    d = get("https://public.api.bsky.app/xrpc/app.bsky.feed.getPosts?uris=at://did:plc:l2mdjnwjrwpns76ol45t66sf/app.bsky.feed.post/3mxbl36nkms2q")
    p = d["posts"][0]; out["bsky"] = {k: p.get(k) for k in ("likeCount","repostCount","replyCount","quoteCount")} | {"createdAt": p["record"]["createdAt"]}
except Exception as e:
    out["bsky"] = {"error": str(e)}
json.dump(out, open("c-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(now)
for i, v in out["hn"].items(): print(i, v.get("points"), v.get("comments_counted"), v.get("title"))
for r, v in out["github"].items(): print(r, v["stargazers_count"], v["forks_count"], v["pushed_at"])
print(out["bsky"])
