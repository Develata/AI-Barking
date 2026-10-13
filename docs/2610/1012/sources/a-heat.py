"""1013 A 组：热度快照。用法（在 sources 目录）：python a-heat.py -> a-heat.json
HN 50060694（Firebase score/descendants、Algolia points、首页名次）、GitHub #48195 与 #53547 的计数（gh api，匿名公开数据）。"""
import json, urllib.request, datetime, subprocess
UA = {"User-Agent": "Mozilla/5.0"}
def g(u): return json.load(urllib.request.urlopen(urllib.request.Request(u, headers=UA), timeout=40))
now = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
out = {"fetched_utc": now}
fb = g("https://hacker-news.firebaseio.com/v0/item/50060694.json")
out["hn_firebase"] = {k: fb.get(k) for k in ("id", "title", "url", "score", "descendants", "time", "by")}
al = g("https://hn.algolia.com/api/v1/items/50060694")
def cnt(n): return sum(1 + cnt(c) for c in n.get("children", []))
out["hn_algolia"] = {"points": al.get("points"), "comments_counted": cnt(al), "created_at": al.get("created_at")}
top = g("https://hacker-news.firebaseio.com/v0/topstories.json")
out["hn_topstories_rank"] = (top.index(50060694) + 1) if 50060694 in top else None
for n in (48195, 53547):
    j = json.loads(subprocess.run(["gh", "api", f"repos/openai/codex/issues/{n}"], capture_output=True, text=True, encoding="utf-8").stdout)
    out[f"gh_{n}"] = {"state": j["state"], "comments": j["comments"], "updated_at": j["updated_at"], "reactions": {k: v for k, v in j["reactions"].items() if k != "url"}}
json.dump(out, open("a-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps(out, ensure_ascii=False))
