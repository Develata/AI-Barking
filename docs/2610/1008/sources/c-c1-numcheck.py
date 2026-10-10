"""1008 C1：在论文 v2 文本（pdftotext -layout，页以 \f 分隔）里逐个定位新闻转述的数字，输出页码；并用 Table 7 复算“封锁非金融属性使保险差距上升”的百分比。
用法：python c-c1-numcheck.py > c-c1-numcheck.txt"""
import re
pages = open("c-arxiv-2609.24927v2.txt", encoding="utf-8").read().split("\f")
norm = lambda s: re.sub(r"\s+", " ", s)
P = [norm(p) for p in pages]
keys = [
 ("325K experiments", "325K"), ("13 agents (abstract)", "13 agents"), ("8 models (abstract)", "we find that 8 models"),
 ("8 of the 13 models (Sec 5.1)", "8 of the 13 models"), ("Opus 4.8 largest effect (abstract)", "Claude Opus 4.8 shows the largest effect"),
 ("$198 flights", "$198"), ("$284 insurance", "$284"), ("Table 1 row Opus 4.8", "Claude Opus 4.8 +198 +284"),
 ("$208 Gemini cheapest", "$208"), ("$21 and $20 (Fig.4 caption)", "$21 and $20"),
 ("up to 40% for insurance (abstract)", "up to 40% for insurance"), ("122 to 171 (Sec 1)", "from 122 to 171"),
 ("40% increase to a $151 (Sec 5.4)", "40% increase to a $151"), ("thirteen models four families", "thirteen models"),
 ("synthetic personas / no real users", "no real users"), ("no human users", "There were no human users"),
 ("single-turn", "single-turn interactions"), ("binary wealth variable", "binary wealth variable"),
 ("omit 5 of the 39 cells", "we omit 5 of the 39"), ("Foundation AI, Cisco affiliation", "Foundation AI, Cisco"),
 ("98.2% complete trials", "98.2%"), ("20 trials per persona per non-control condition", "every persona contributes 20 trials"),
 ("default api-settings", "default api-settings"),
]
for name, k in keys:
    pg = [i + 1 for i, p in enumerate(P) if k in p]
    print(f"{name:55s} -> PDF page(s) {pg}")
print("\nDerived: 13 models x 3 domains x 32 personas x 13 non-control conditions x 20 trials =", 13*3*32*13*20, "(inference; paper does not state this decomposition)")
print("with 14 conditions incl. control:", 13*3*32*14*20)
# Table 7 (insurance $/mo): tool_full and blocked_* per model, order as in paper
models = ["GPT-5","GPT-5-mini","GPT-5-nano","GPT-5.5","Opus4.8","Sonnet5","Haiku4.5","Gem2.5Flash","Gem3.1FlashLite","Gem3Flash","Qwen2B","Qwen9B","Qwen35B"]
t7 = {
 "tool_full":[191,124,56,122,284,151,158,217,179,332,17,195,133],
 "blocked_employment":[210,105,77,171,317,195,194,246,183,370,20,236,151],
 "blocked_health":[220,129,53,126,323,171,169,219,326,508,29,179,104],
 "blocked_life_events":[201,109,69,124,287,157,166,232,250,330,63,214,150],
 "blocked_demographics":[210,138,48,131,317,195,170,236,292,319,17,218,131],
}
print("\nTable 7 (insurance gap, $/mo): % change of gap when one NON-financial axis is blocked, vs tool_full")
best = []
for ax in ["blocked_employment","blocked_health","blocked_life_events","blocked_demographics"]:
    for m, f, b in zip(models, t7["tool_full"], t7[ax]):
        best.append(((b-f)/f*100, m, ax, f, b))
best.sort(reverse=True)
for pct, m, ax, f, b in best[:8]: print(f"  {m:16s} {ax:22s} {f:4d} -> {b:4d}  {pct:+.0f}%")
print("GPT-5.5 employment:", f"{(171-122)/122*100:+.1f}%", " GPT-5.5 demographics:", f"{(131-122)/122*100:+.1f}%")
print("Note: 151 appears in Table 7 only as Sonnet5 tool_full and Qwen35B blocked_employment; GPT-5.5 never has 151.")
