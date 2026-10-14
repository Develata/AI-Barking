"""1013 D 组：核对 evidence-d.md 中的逐字摘句都能在存档中 grep 到（忽略弯引号与空白差异）。用法：python -I d-verify-quotes.py"""
import re
def norm(s):
    s = s.replace("’", "'").replace("‘", "'").replace("“", '"').replace("”", '"')
    return re.sub(r"\s+", " ", s)
Q = [
 # D1 Apple HomeView
 ("d-d1-apple-homeview.txt", "Pre-orders start today, with availability beginning November 13."),
 ("d-d1-apple-homeview.txt", "can pre-order Apple HomeView in silver, starlight, rose pink, and space gray for $299 (U.S.)"),
 ("d-d1-apple-homeview.txt", "Customers in Singapore can pre-order starting Thursday, October 15, at 9 a.m. SGT."),
 ("d-d1-apple-homeview.txt", "Both models will be available in these countries beginning Friday, November 13."),
 ("d-d1-apple-homeview.txt", "Apple HomeView is not available for sale in the European Union, as Siri AI is currently unavailable in iOS in the EU."),
 ("d-d1-apple-homeview.txt", "Siri AI on HomeView is available in English in the U.S., Australia, Canada, India, New Zealand, Singapore, South Africa, and the UK."),
 ("d-d1-apple-homeview.txt", "Canadian French-language support will be offered later via a free software update."),
 ("d-d1-apple-homeview.txt", "Siri AI is available in beta in HomeView software version 27.2."),
 ("d-d1-apple-homeview.txt", "Certain Apple Intelligence features that rely on server-side models are subject to daily usage limits. Expanded access to such features will be available for a fee in the future."),
 # D2 Anthropic Scion
 ("d-d2-anthropic-scion.txt", "In May 2026, Anthropic averaged around 2,500 code changes merged per day; as of September, we merged over 30,000 per day."),
 ("d-d2-anthropic-scion.txt", "We perform on the order of 25 million code checkouts per day for CI"),
 ("d-d2-anthropic-scion.txt", "Our Scion infrastructure footprint is now 200x smaller than our previous git mirroring infrastructure in terms of CPU and memory requirements."),
 ("d-d2-anthropic-scion.txt", "A team of seven people built both Scion and Antique"),
 ("d-d2-anthropic-scion.txt", "fully migrated our monorepo from git in five months."),
 ("d-d2-anthropic-scion.txt", "Scion is by no means a suitable replacement for git for general use."),
 ("d-d2-anthropic-scion.txt", "Published Oct 13, 2026"),
 # D3 VS Copilot / SSMS CVE
 ("d-d3-embracethered.txt", "Today Microsoft shipped the patch"),
 ("d-d3-embracethered.txt", "Microsoft classified it as Important severity Remote Code Execution vulnerability."),
 ("d-d3-embracethered.txt", "Despite Microsoft classifying the issue as Important severity Remote Code Execution (CVSS Score 8.8.)"),
 ("d-d3-embracethered.txt", "October 11, 2026: $0 bounty awarded"),
 ("d-d3-embracethered.txt", "October 13, 2026: Fix is shipping as part of Patch Tuesday"),
 ("d-d3-embracethered.txt", "considered out of scope for a bounty reward"),
 ("d-d3-embracethered.txt", "Older versions, including Visual Studio 2022, are also affected."),
 ("d-d3-msrc-api-sug.json", "\"severity\":\"Important\""),
 ("d-d3-msrc-api-sug.json", "\"baseScore\":\"8.8\""),
 ("d-d3-msrc-cve-2026-103572-cvrf-entry.xml", "Publicly Disclosed:No;Exploited:No;Latest Software Release:Exploitation Less Likely"),
 ("d-d3-msrc-cve-2026-103572-cvrf-entry.xml", "<vuln:FixedBuild>18.10.4</vuln:FixedBuild>"),
 ("d-d3-msrc-product-names.txt", "ProductID 21956: Microsoft Visual Studio 2026 version 18.10"),
 ("d-d3-msrc-product-names.txt", "ProductID 16767: Microsoft Visual Studio 2022 version 17.14"),
 # D4 XBOW
 ("d-d4-xbow.txt", "XBOW recently discovered and built a Chrome full-chain exploit affecting an estimated billion users globally"),
 ("d-d4-xbow.txt", "Earlier this year, it won our company a $250,000 bounty from Google, the first under its 2026 new rules."),
 ("d-d4-xbow.txt", "The bugs are now fixed, and today we’re sharing the details."),
 ("d-d4-xbow.txt", "The compute we used over six days for the awarded chain cost $40,000, a fraction of the bounty."),
 ("d-d4-xbow.txt", "These four were joined by a fifth colleague: OpenAI’s GPT-5.6 Sol and its predecessors."),
 ("d-d4-xbow.txt", "That meant that if a bug had already been reported — precisely what had disqualified our first build attempt — we could quickly swap in a backup."),
 ("d-d4-xbow.txt", "between Sunday, August 2, and Friday, August 7."),
 ("d-d4-xbow.txt", "But when Google told us that it already knew about one of the bugs in the chain, we went back to the drawing board to build a new chain."),
 ("d-d4-xbow.txt", "We did it in six days."),
 ("d-d4-xbow.txt", "The AI searched Chrome’s source code, tested ideas, built proofs of concept, and wrote the earliest drafts of the exploit."),
 # D5 Meta
 ("d-d5-meta-algorithm.txt", "We’re rolling out Your algorithm, where you can directly add and manage the topics you want to see in Feed and Reels."),
 ("d-d5-meta-algorithm.txt", "New AI-powered tools let you tell Facebook, in your own words, what you want to see in Feed and Reels, and your recommendations respond right away."),
 ("d-d5-meta-algorithm.txt", "After clicking ‘Interested’ or ‘Not interested,’ we may ask follow up questions for more detail on what you liked or disliked."),
 ("d-d5-meta-algorithm.txt", "October 13, 2026October 13, 2026"),
 # D6 Physical Intelligence
 ("d-d6-pi-api.txt", "We are making our latest models available for training and inference through the Pi API beta"),
 ("d-d6-pi-api.txt", "If you would like to participate in the beta, please tell us"),
 ("d-d6-pi-api.txt", "You upload data, launch fine-tuning on compute and infrastructure we manage, and access fine-tuned models via a hosted gateway to run inference."),
 ("d-d6-pi-api.txt", "Most applications shown above, from Japan to Switzerland to deployments across the United States, ran with cloud-hosted inference through the API."),
 ("d-d6-pi-api.txt", "We gave a small number of companies early access to the API"),
 ("d-d6-pi-api.txt", "We’re still early, and feedback from developers will help shape future releases."),
 # D7 Liquid AI
 ("d-d7-liquid-lfm.txt", "Today, we release LFM2.5-24B-A2B, a 24B-parameter reasoning model with just 2B active parameters, built for fast, private agents on-device."),
 ("d-d7-liquid-lfm.txt", "It decodes 184 tokens/s on an Apple M5 Max and 114 tokens/s on an AMD Ryzen AI Max+ 395, up to 1.7x faster than the next-best model, while using the least memory."),
 ("d-d7-liquid-lfm.txt", "All runs were collected on Pipette"),
 ("d-d7-liquid-lfm.txt", "We benchmark each model with SGLang 0.5.22, 1,024 input tokens, up to 256 output tokens, in BF16."),
 ("d-d7-liquid-lfm.txt", "The base (LFM2.5-24B-A2B-Base) and post-trained (LFM2.5-24B-A2B) models are open-weight and available today on Hugging Face"),
 ("d-d7-liquid-lfm.txt", "Open-weight — Download, fine-tune, and deploy. See our model license"),
 ("d-d7-liquid-lfm.txt", "OCT 13, 2026"),
 ("d-d7-lfm-license.txt", "Rights to use the model for commercial purposes end if your company's annual revenue exceeds $10 million USD."),
 ("d-d7-hf-LICENSE.txt", "\"Threshold\" shall mean annual revenue of 10 million United States dollars ($10,000,000) or more."),
 ("d-d7-hf-api-model.json", "\"lfm1.0\""),
 # D8 AA Decision Index
 ("d-d8-aa-decision.txt", "Version 1.0 combines four evaluations across classification, tool calling, knowledge, hallucination, and reasoning."),
 ("d-d8-aa-decision.txt", "500 private questions adapted from our knowledge and hallucination benchmark AA-Omniscience."),
 ("d-d8-aa-decision.txt", "Jev 1.13 leads the Decision Index with 58, ahead of GLiDE and Microsoft-Decision-1, both on 54."),
 ("d-d8-aa-decision.txt", "Claude Haiku 5.5 (Low) scores 61 on the Decision Index, 3 points ahead of Jev 1.13, but takes 1.4 seconds per task against 0.13 seconds (11 times slower) and costs $184 per one million tasks against $42 (4.4 times as much)."),
 ("d-d8-aa-decision.txt", "Overall, across almost all probability levels, accuracy falls below that probability for all decision models."),
 ("d-d8-aa-decision.txt", "An LLM answers with a structured output that names the option it chose, so it gives no probability for each option."),
 ("d-d8-aa-decision.txt", "Calibration for language models coming soon."),
 ("d-d8-aa-decision.txt", "October 13, 2026"),
]
bad = 0
for f, q in Q:
    try:
        t = norm(open(f, encoding="utf-8", errors="replace").read())
    except Exception as e:
        print("MISSING FILE", f, e); bad += 1; continue
    ok = norm(q) in t
    if not ok:
        bad += 1
    print("OK  " if ok else "FAIL", f, "|", q[:70])
print("total", len(Q), "fail", bad)
