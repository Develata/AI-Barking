"""1008 A 组：HN Algolia 复取分数/评论数，保存条目 JSON（含评论树）。用法：python a-heat.py  -> a-heat.json, a-hn-<id>.json"""
import json, urllib.request, datetime, time
IDS = {"49994145":"NS Lost in Translation (arXiv)","50003107":"OpenAI Withdraws 3 Math Papers (history.md)","50002650":"OpenAI withdraws three mathematical results (X)",
       "50003100":"OpenAI withdraws three of their recent manuscripts (PR files)","50002008":"Math 2.0 (Tao)","49997718":"The Mathocalypse","50013902":"Karagila (HN, 2nd submission)","50004120":"Karagila (HN, 1st submission)",
       "50003677":"AHM Statement (2nd)","49999159":"AHM Statement (1st)","49903713":"AGMAI Responsible Release","49650326":"OpenAI NS release included Lean 4 formal proof (John D. Cook)","49661928":"OpenAI changed NS press release and Lean4 code on GH"}
out = {"fetched_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"), "items": {}}
for i, name in IDS.items():
    for attempt in range(5):
        try:
            d = json.load(urllib.request.urlopen(urllib.request.Request(f"https://hn.algolia.com/api/v1/items/{i}", headers={"User-Agent": "Mozilla/5.0"}), timeout=60)); break
        except Exception as e:
            time.sleep(2 * (attempt + 1))
    else:
        raise SystemExit(f"failed {i}")
    json.dump(d, open(f"a-hn-{i}.json", "w", encoding="utf-8"), ensure_ascii=False)
    def cnt(n): return 1 + sum(cnt(c) for c in n.get("children", []))
    out["items"][i] = {"name": name, "title": d.get("title"), "url": d.get("url"), "points": d.get("points"), "created_at": d.get("created_at"), "comments_in_tree": cnt(d) - 1}
json.dump(out, open("a-heat.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for i, v in out["items"].items(): print(i, v["points"], v["comments_in_tree"], v["created_at"], v["title"])
print(out["fetched_utc"])
