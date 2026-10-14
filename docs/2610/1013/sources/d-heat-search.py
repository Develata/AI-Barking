"""1013 D 组 HN 标题关键词补查：对 URL 查询无结果或需补充的条目，按标题关键词在 HN Algolia 查 story，
只保留 2026-10-12 之后的命中，写 d-heat-search.json。SSL 偶发错误时重试。用法：python -I d-heat-search.py"""
import json, urllib.request, urllib.parse, datetime, time
QS = {
 "D2": "Scion source control Anthropic",
 "D3": "Copilot SSMS CVE-2026-103572",
 "D4": "XBOW Chrome exploit",
 "D5": "Your algorithm Facebook",
 "D7": "LFM2.5-24B-A2B",
 "D8": "Decision Index Artificial Analysis",
}
bj = datetime.timezone(datetime.timedelta(hours=8))
now = datetime.datetime.now(bj).strftime("%Y-%m-%d %H:%M 北京时间")
out = {"fetched": now, "queries": {}}
for k, q in QS.items():
    u = "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=6&query=" + urllib.parse.quote(q)
    hits = None
    for _ in range(4):
        try:
            hits = json.load(urllib.request.urlopen(u, timeout=40))["hits"]; break
        except Exception:
            time.sleep(3)
    if hits is None:
        out["queries"][k] = {"q": q, "error": "failed after retries"}; print(k, "ERR"); continue
    rows = [{kk: x.get(kk) for kk in ("objectID", "points", "num_comments", "created_at", "title", "url")}
            for x in hits if (x.get("created_at") or "") >= "2026-10-12"]
    out["queries"][k] = {"q": q, "rows": rows}
    print("==", k, q)
    for r in rows:
        print("  ", r["objectID"], r["points"], r["num_comments"], r["created_at"], (r["title"] or "")[:80])
json.dump(out, open("d-heat-search.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("fetched", now)
