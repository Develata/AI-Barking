"""1013 D 组 HN 热度：按每条的一手 URL 在 HN Algolia 查 story（restrictSearchableAttributes=url），
记积分、评论数、发帖时刻（北京时间）与取数时刻，写 d-heat.json。用法：python -I d-heat.py"""
import json, urllib.request, urllib.parse, datetime
URLS = {
 "D1": "https://www.apple.com/newsroom/2026/10/meet-apple-homeview-a-new-way-to-enjoy-entertainment-and-manage-the-home/",
 "D2": "https://www.anthropic.com/engineering/redesigning-source-control",
 "D3": "https://embracethered.com/blog/posts/2026/visual-studio-copilot-remote-code-execution/",
 "D4": "https://xbow.com/blog/xbow-built-a-chrome-full-chain-exploit",
 "D5": "https://about.fb.com/news/2026/10/introducing-your-algorithm-facebook/",
 "D6": "https://www.pi.website/blog/api-beta",
 "D7": "https://www.liquid.ai/blog/lfm2-5-24b-a2b",
 "D8": "https://artificialanalysis.ai/articles/artificial-analysis-decision-index",
}
bj = datetime.timezone(datetime.timedelta(hours=8))
now = datetime.datetime.now(bj).strftime("%Y-%m-%d %H:%M 北京时间")
out = {"fetched": now, "items": {}}
for k, u in URLS.items():
    q = urllib.parse.quote(u)
    api = f"https://hn.algolia.com/api/v1/search?tags=story&restrictSearchableAttributes=url&hitsPerPage=10&query={q}"
    try:
        hits = json.load(urllib.request.urlopen(api, timeout=40))["hits"]
    except Exception as e:
        out["items"][k] = {"error": str(e)}; print(k, "ERR", e); continue
    rows = []
    for x in hits:
        rows.append({kk: x.get(kk) for kk in ("objectID", "points", "num_comments", "created_at", "title", "url")})
    out["items"][k] = rows
    print(k, [(r["objectID"], r["points"], r["num_comments"], r["created_at"], (r["title"] or "")[:70]) for r in rows])
json.dump(out, open("d-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("fetched", now)
