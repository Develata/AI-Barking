"""1012 D 组 HN Algolia 热度：按 URL 子串/标题查询，输出 id/分数/评论/发帖时刻/标题。用法：python -I d-heat.py（写 d-heat.json）"""
import json, urllib.request, urllib.parse, datetime
qs = [
 "Illustration beyond Proof", "Nadella Math 2.0", "Multi-Agent Self-Supervision", "Recursive Self-Improvement through Multi-Agent Self-Supervision",
 "Sakana MASS", "Do Agent Teams Pay Off", "Vibe Code Bench", "Math 2.0 value mathematical progress holistically",
 "Build your own decision model", "Campfire Rust Elixir rewrite", "I am sorry but you still have to think", "Nicolas Cage AI waiver",
 "Nicolas Cage Amazon Spider-Noir", "BetterWispr", "Step 5 Preview StepFun", "Clef-omni", "Cloudflare Clef decision model", "Clef-flash Cloudflare",
]
now = datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).strftime("%m-%d %H:%M")
out = {"fetched_beijing": now, "queries": {}}
for q in qs:
    u = "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=6&query=" + urllib.parse.quote(q)
    try:
        h = json.load(urllib.request.urlopen(u, timeout=30))["hits"]
    except Exception as e:
        print("ERR", q, e); continue
    rows = []
    for x in h:
        ct = x.get("created_at", "")
        rows.append({k: x.get(k) for k in ("objectID", "points", "num_comments", "created_at", "title", "url")})
    out["queries"][q] = rows
    print("==", q)
    for r in rows:
        print("  ", r["objectID"], r["points"], r["num_comments"], r["created_at"], (r["title"] or "")[:80], r["url"])
json.dump(out, open("d-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("fetched", now)
