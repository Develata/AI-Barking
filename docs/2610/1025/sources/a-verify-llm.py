# A 组：独立复算 LLM 部分（只读 zip 解出的 results/*.jsonl，不运行作者脚本）
import json, collections as C, sys
base = "a-data-zip/x/results/"
for name in ["gpt-6-astra-v2", "anthropic_claude-fable-5.1-v2"]:
    rows = [json.loads(l) for l in open(base + name + ".jsonl", encoding="utf-8")]
    print("==", name, "n", len(rows), "unparsed", sum(not r.get("answer") for r in rows), "refusal", sum(bool(r.get("refusal")) for r in rows))
    rows = [r for r in rows if r.get("answer")]
    for r in rows:
        r["dx"] = str(r["answer"].get("diagnosis", "")).upper(); r["dis"] = str(r["answer"].get("discharge", "")).lower(); r["sec"] = str(r["answer"].get("second_choice", "")).upper()
    print(" correct", sum(r["dx"] == r["truth"] for r in rows), " top2", sum(r["truth"] in (r["dx"], r["sec"]) for r in rows))
    print(" truth counts", C.Counter(r["truth"] for r in rows))
    print(" dx counts", C.Counter(r["dx"] for r in rows))
    ns = [r for r in rows if r["truth_discharge"] == "not safe"]; sf = [r for r in rows if r["truth_discharge"] == "safe"]
    print(" unsafe kept", sum(r["dis"] == "no" for r in ns), "/", len(ns), " safe discharged", sum(r["dis"] == "yes" for r in sf), "/", len(sf))
    for t in ["MEL", "BCC", "SCC", "ACK", "NEV", "SEK"]:
        sub = [r for r in rows if r["truth"] == t]
        print("  ", t, "n", len(sub), "correct", sum(r["dx"] == t for r in sub), "discharged", sum(r["dis"] == "yes" for r in sub))
    # unsafe lesions (non-melanoma) discharged
    print(" discharged unsafe by class", C.Counter(r["truth"] for r in ns if r["dis"] == "yes"))
    print(" reasoning tokens total", sum(r["usage"]["completion_tokens_details"]["reasoning_tokens"] for r in rows))
