"""1008 C 组热度：HN Algolia 搜索（公开 JSON，未登录）。输出 c-heat.json 并打印摘要。时间为北京时间。"""
import json, urllib.request, urllib.parse, datetime
UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36"
def get(u):
    return json.load(urllib.request.urlopen(urllib.request.Request(u, headers={"User-Agent": UA}), timeout=40))
def bj(ts): return (datetime.datetime.fromtimestamp(ts, datetime.timezone.utc).replace(tzinfo=None) + datetime.timedelta(hours=8)).strftime("%m-%d %H:%M BJ")
now = datetime.datetime.now(datetime.timezone.utc)
since = int(datetime.datetime(2026, 9, 18, tzinfo=datetime.timezone.utc).timestamp())
queries = {
 "C1": ["Et Tu, Brute economic misalignment", "personal AI agents wealth pricier", "2609.24927", "AI chatbots upsell wealthy"],
 "C2": ["ts-rust", "tsc-rs", "TypeScript compiler Rust port LLM", "pingdotgg"],
 "C3": ["11SquaresFormalized", "eleven squares", "11 squares packing"],
 "C4": ["OpenAI annualized revenue", "OpenAI revenue $50 billion", "OpenAI annualised revenues"],
 "C5": ["Claude Dashboards", "Claude Motion", "dashboards-and-motion"],
 "C6": ["Anthropic Cyber Mission", "OSS Scanner", "Critical Infrastructure Defense Program"],
 "C7": ["Whistle speech to text 16.9", "cactuscompute whistle", "Cactus Whistle"],
 "C8": ["USA Today sues OpenAI", "USA Today OpenAI copyright", "USA Today lawsuit OpenAI"],
}
out = {"fetched_utc": now.strftime("%Y-%m-%d %H:%M"), "fetched_bj": (now + datetime.timedelta(hours=8)).strftime("%Y-%m-%d %H:%M BJ"), "items": {}}
for k, qs in queries.items():
    seen = {}
    for q in qs:
        for tags in ("story",):
            try:
                j = get("https://hn.algolia.com/api/v1/search?" + urllib.parse.urlencode({"query": q, "tags": tags, "numericFilters": f"created_at_i>{since}", "hitsPerPage": 15}))
            except Exception as e:
                print("fail", k, q, e); continue
            for h in j["hits"]:
                seen[h["objectID"]] = {"id": h["objectID"], "points": h["points"], "comments": h["num_comments"], "time": bj(h["created_at_i"]), "title": h["title"], "url": h.get("url"), "q": q}
    out["items"][k] = sorted(seen.values(), key=lambda x: -(x["points"] or 0))
    print("==", k)
    for h in out["items"][k][:8]:
        print("  ", h["id"], h["points"], h["comments"], h["time"], (h["title"] or "")[:80], (h["url"] or "")[:90])
json.dump(out, open("c-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
