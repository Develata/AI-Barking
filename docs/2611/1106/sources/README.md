# 1106 来源与制作状态

期号1106（上期1105+1）；线索为 Develata 回贴的两份扫描：ChatGPT 滚动扫描（北京 11-07 19:09，`input-scan-1107-11Z.md`）与 WorkBuddy“AI吠点选题扫描-1107-18”（北京 11-07 18:01，`input-scan-1107-18.txt`）。两份都没收超级智能法案，这一题由 Develata 提出。

## 主帖

1. 超级智能法案：[H.R. 10538 原文](https://www.govinfo.gov/content/pkg/BILLS-119hr10538ih/html/BILLS-119hr10538ih.htm)、[congress.gov](https://www.congress.gov/bill/119th-congress/house-bill/10538)、[图书馆“dead bill”说明](https://ask.loc.gov/law/faq/334496)（存档 `a-loc-dead-bill.txt`）、[USA Today](https://www.usatoday.com/story/news/politics/2026/10/01/exclusive-hakeem-jeffries-ai-crisis-guardrails/92046627007/)（存档 `a-usatoday-jeffries.txt`）、[Tom's Hardware](https://www.tomshardware.com/tech-industry/artificial-intelligence/congresswoman-calls-for-artificial-superintelligence-ban-after-democrats-seismic-midterm-sweep-demands-regulation-of-out-of-control-industry-and-pause-on-advanced-ai)、[CNBC](https://www.cnbc.com/2026/11/05/democrats-data-center-restrictions-midterms.html)。
   - 待核：Jeffries、Schumer 选后是否回应；Sanders 官网逐节摘要 403 未取得；委员会听证记录未单查。
2. METR 与 Cotra：[METR 评估](https://metr.org/blog/2026-11-05-anthropic-ai-rd-acceleration-assessment/)、[METR 评审](https://metr.org/blog/2026-11-05-rd-section-anthropic-risk-report-august-2026-review/)、[Cotra 博文](https://www.planned-obsolescence.org/p/ai-is-dramatically-accelerating-ai)、SEC S-1/A（存档 `b-sec-s1a-ctx.json`）。
   - 待核：Anthropic 对 METR 报告有无回应（未找到）。Claude 与本事件当事公司同属 Anthropic，本条靠跨模型复核把关。

## 速览

- Gemini：[官方](https://blog.google/innovation-and-ai/technology/developers-tools/introducing-gemini-omni-1-2-flash/)（L1）。
- Hy4：[AA](https://artificialanalysis.ai/models/tencent-hy4)（L3）。
- 微软：[彭博社稿，雅虎财经转载](https://finance.yahoo.com/technology/ai/articles/microsoft-winding-down-oracle-deal-173639264.html)（L4）。
- 记忆：[arXiv 2611.07554](https://arxiv.org/abs/2611.07554)（L1，预印本）。
- Hightouch：[原文](https://hightouch.com/blog/llm-judges-identity-resolution)（L1，厂商自测）。
- Firecracker：[PWN.AI Part 2](https://pwn.ai/blog/firecracker-escape)、[Part 1](https://pwn.ai/blog/pwnescape)（L3，研究者自述）。扫描写的 CVE-2026-122192 是 Coinspect 报的另一个内存漏洞，不用。
- OpenRouter：[官方](https://openrouter.ai/blog/announcements/auto-and-auto-code-routers/)（L1，厂商自测）。
- Crusoe：[官方](https://www.crusoe.ai/resources/newsroom/crusoe-wyoming-data-center-campus-update)（L3）。谷歌博客打不开，170亿美元、702MW 未核实，不用。
- 未收：13岁校园诉讼（青少年自伤题材，Develata 选了不做主帖，本期不收）；OpenAI 文件名（1105 已发）；Images 2.6 榜单（1104 已收）。

## 流程记录

- 三组取证（harness 时长）：C Haiku 8 分钟，B Sonnet 24 分钟，A Sonnet 34 分钟；未触发60分钟催交。C 组把彭博稿和 Crusoe 记成未找到，Claude 用 `barking fetch` 补存。
- 扫描勘误：Firecracker 编号张冠李戴；“K-12 默认关闭清单没有搜索”与 Google 原文不符；Kokotajlo 质疑算力核查未找到；“FAKE DATA”是明示的示意图。
