#!/usr/bin/env python3
"""B 组：经匿名 web-public 接口分页取 Lean Zulip 讨论串全文（无登录、无凭据）。"""
import json, sys, time, urllib.parse, urllib.request
BASE = "https://leanprover.zulipchat.com/json/messages"
TOPIC = sys.argv[2] if len(sys.argv) > 2 else "Cleaning up the proof"
narrow = [{"operator": "channels", "operand": "web-public"},
          {"operator": "channel", "operand": 658556},
          {"operator": "topic", "operand": TOPIC}]
out, anchor, first = [], "oldest", True
while True:
    q = urllib.parse.urlencode({"anchor": anchor, "num_before": 0, "num_after": 500,
                                "narrow": json.dumps(narrow), "apply_markdown": "true",
                                "include_anchor": "true" if first else "false"})
    req = urllib.request.Request(BASE + "?" + q, headers={"User-Agent": "evidence-collector/1.0"})
    with urllib.request.urlopen(req, timeout=60) as r:
        d = json.load(r)
    msgs = d["messages"]
    out += msgs
    if not msgs or d.get("found_newest"):
        break
    anchor, first = msgs[-1]["id"], False
    time.sleep(1)
json.dump(out, open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
print(len(out), out[0]["timestamp"] if out else None, out[-1]["timestamp"] if out else None)
