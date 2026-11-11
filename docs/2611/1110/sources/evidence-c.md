# 1110 期 C 组取证：Gemini 4 Carbon 全面开放（跟进 1028）

执行方 claude-sonnet-5-5；取证时段 2026-11-11 14:30–14:50（北京时间，UTC+8，本机时间）。所有数字来自存档文件 `sources/c-*`；时间一律北京时间，括号内为原时区。“推算”指执行方自己的算术或推断，不是来源的说法。图在 `images/25–36`（见第 5 节）。

分级约定：L1 官方、L3 第三方评测原始页（仅限其自身数据）、L4 媒体、L6 社区。

## 0. 先说三个与线索不同或线索没覆盖的要点

1. “more than 6X lower in average cost per task” 的算法找到了：来源是 DeepMind 的 MKA 技术报告（2026-11-10）表 4，“list-price API cost (with prompt caching)”，Fable 5.2 $35.28 对 Carbon $5.63，写作“6.26× higher API cost”。AA 的 $31.97 / $6.29（“over 5×”）是 AA 另算的成本。两套成本数字不同、倍数不同，并列记录，不判谁对（见 C3）。报告里的质量数与 AA 一致（Climb WSR 56.56 对 54.34）。
2. Google 技术报告自己写：Fable 5.2 对 Carbon 的 Climb WSR 差 +2.22 pp，95% CI [−0.94, +5.49]，“statistically non-significant”；前三名（Fable 5.2、Carbon、Opus 5.5）“statistically indistinguishable”。Google 博文写“ranks #2”。
3. 1028 的网络安全成绩（SEC-bench Pro、ExploitGym、CWE-bench v1）属于 Fairwind 版本：模型卡写“cybersecurity benchmark results reflect performance when accessed through the Fairwind Program”，本次博文“Carbon for cyber”仍需申请 Fairwind；模型卡写“specialized capabilities available to verified defenders via the Fairwind Program”。博文与模型卡都没有给公开版 Carbon 在这三项基准上的成绩（只读了博文与模型卡，安全报告 PDF 未读）；脚注 1 只写价格“Applies to the Carbon model intended for public access only”。

## 1. 证据表

表头与 `templates/codex-evidence.md` 第 4 条一致。状态只用四种。“截图文件”指 `images/` 下文件。

### C1 开放范围

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1a API 对所有付费开发者与企业开放 | L1 | https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-4-carbon-now-available/（存档 c-blog.html/.txt） | “Gemini 4 Carbon is available via the API in Google AI Studio and Gemini for business for all paying developers and enterprises, at the same introductory price 1 as Gemini 4 Argon ($2 per million input tokens and $10 per million output tokens).” ；“For developers: Build with the Gemini API paid tier in AI Studio and Android Studio” | 25、36 | 博文页面可见日期 Nov 10, 2026；页面元数据 datePublished 2026-11-10T16:00:00+00:00（北京 11-11 00:00，可能是占位），dateModified 2026-11-10T18:41:52+00:00（北京 11-11 02:41:52） | 已找到 |
| C1b 免费层不可用 | L1 | https://ai.google.dev/gemini-api/docs/pricing#gemini-4-carbon（无头 Chrome 渲染存档 c-api-pricing-rendered.html/.txt；直接 fetch 为 JS 空壳） | “Gemini 4 Carbon is not available on the Free tier.” ；表中 Free Tier 列：“Not available” | 26 | 页面无“最后更新”日期；2026-11-11 约 14:35 渲染 | 已找到 |
| C1c 普通用户：Gemini 应用与 AI Mode 仅 Ultra 订阅 | L1 | 同 C1a | “For everyday users: Try Carbon starting today in the Gemini app and AI Mode in Search if you’re a Google AI Ultra subscriber.” ；“It is also rolling out to all Google AI Ultra subscribers in Google Antigravity, the Gemini app, and AI Mode in Search.” | 25、36 | “rolling out” 是逐步推送措辞；博文未提 AI Pro、Plus、免费用户 | 已找到 |
| C1d Sundar 帖同口径 | L1 | https://x.com/sundarpichai/status/2120185896469496180（fxtwitter JSON：c-fx-2120185896469496180.json） | “Coming globally via the Gemini API in @GoogleAIStudio and Gemini for business, and for Ultra subscribers in @antigravity, the @GeminiApp, and AI Mode in Search.” | — | 2026-11-11 00:04:43 北京（2026-11-10 16:04:43 UTC）。线索里的“GoogleAI 帖 16:12:06 UTC”对应 @GoogleAI（见 C8）；@Google 与 @GoogleAIStudio 另有帖（见 C8） | 已找到 |
| C1e 模型卡：按渠道与订阅档分 | L1 | https://storage.googleapis.com/deepmind-media/Model-Cards/Gemini-4-Carbon-Model-Card.pdf（c-modelcard.pdf/.txt，第 4 页） | “Access varies by channel and subscription tier (such as paid API tiers in Google AI Studio and Gemini for business, Google AI Ultra in consumer surfaces and Google Antigravity, and verified-defender access via the Fairwind Program).” | — | 模型卡第 5 页：“Gemini 4 Carbon results as of October 2026 are below (cybersecurity benchmark results reflect performance when accessed through the Fairwind Program)” | 已找到 |
| C1f AI Pro 订阅是否可用 | L1（官方页未写）/L4 | 官方博文与 Sundar、@GoogleAI 帖均未提 AI Pro；https://gemini.google/subscriptions/（c-gemini-subs.txt）Ultra 栏：“Highest access to Gemini 4 Carbon”；Pro 栏：“Access to Gemini 4 Argon” | 媒体：9to5Google “AI Ultra subscribers can now access Carbon in the Gemini app, AI Mode, and Antigravity. Notably, Argon (AI Pro and up) will remain available alongside the new model.” ；Android Authority “Google’s announcement gives no timeline for access on lower-priced plans.” | — | 订阅页文本里 “Includes Gemini 4 Argon” 排在 “Google AI Pro” 标题之前、“Includes Gemini 4 Carbon” 排在 “Google AI Ultra” 标题之前，即标签在卡片标题上方（已核对 c-gemini-subs.txt 165–256 行：Pro 栏列“Access to Gemini 4 Argon”，Ultra 栏列“Highest access to Gemini 4 Argon / Carbon”，二者符合）。官方没有写“Pro 不可用”，只是 Pro 栏没列 Carbon | 部分支持 |
| C1g Workspace/企业应用内可用 | L1/L6 | 博文：“For enterprises: Try it in Gemini for business via the API or model drop down, Google Antigravity, or CodeMender.” ；HN 评论 50442498 | HN 评论（Murrawhip，自述）：“My Google Workspace Gemini (with "AI Expanded Access") has 3.6 Flash, 3.8 Thinking, 4 Carbon. My personal "AI Pro" Gemini has 3.5 Flash-Lite, 3.8 Flash, 3.1 Pro, 4 Argon.” | 36 | 评论仅作线索（L6），没有分数 | 部分支持 |
| C1h 此前（1028）只对网络安全防御者开放，现为约两周后 | L1 | 同 C1a；1028 博文 c-blog-1028.txt | 本次：“Two weeks ago, we released Carbon to a set of cybersecurity defenders via our Fairwind Program”；1028：“Starting this week, we will make Carbon available to a subset of cyber defenders … before making Carbon available to developers, businesses, and consumers via our product surfaces as soon as possible.” | 25 | 1028 博文 datePublished 2026-10-28T17:00:00+00:00（北京 10-29 01:00）；“Two weeks ago”为 Google 措辞（实为 13 天） | 已找到 |
| C1i ai.google.dev “what’s new” 页的开放措辞 | L1 | https://aistudio.google.com/docs/whats-new-gemini-4-carbon（无头渲染存档 c-aistudio-whatsnew-rendered.txt；ai.google.dev 的 changelog 同样存档 c-api-changelog-rendered.txt） | “Gemini 4 Carbon is available on the paid tier of the Gemini API and in Google AI Studio.” ；changelog：“November 10, 2026 Gemini 4 Carbon (gemini-4-carbon) generally available (GA)” | — | changelog 条目只有日期 November 10, 2026，官方未给时刻 | 已找到 |

### C2 体验价与 12 月 1 日起的标准价

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C2a 体验价 $2 / $10 | L1 | 博文（同 C1a）与定价页 | 博文：“($2 per million input tokens and $10 per million output tokens)” ；定价页 Standard 标签：“Input price … $2.00 through November 30, 2026.” ；“Output price (including thinking tokens) … $10.00 through November 30, 2026.” | 25、26 | 定价页 Standard 标签；Batch、Flex、Priority 标签未截 | 已找到 |
| C2b 12 月 1 日起 $4 / $20（脚注 1，含到期日） | L1 | 博文脚注 1（c-blog.txt 末尾；页面中折叠在文末 “More Information” 里） | “After the introductory period expires on November 30, 2026, the price of $4 per 1M input tokens and $20 per 1M output tokens will apply. Applies to the Carbon model intended for public access only.” ；定价页：“$4.00 starting December 1, 2026.” ；“$20.00 starting December 1, 2026.” | 26（定价页）；脚注本身未截（折叠，无头渲染两次展开失败，见 capture-log） | 到期日写 November 30, 2026，定价页写 12 月 1 日起新价；两者相容（11/30 为体验价最后一天，时区官方未给）。脚注原句只在 txt 存档里 | 已找到 |
| C2c 缓存价 | L1 | 定价页；https://aistudio.google.com/docs/whats-new-gemini-4-carbon | 定价页：“Context caching price … $0.10 through November 30, 2026. $0.20 starting December 1, 2026. Implicit caching only (explicit caching is not supported).” ；what’s new：“Implicit caching … discounts cached input tokens by 95% on cache hits.” | 26 | 无显式缓存；无长上下文分档（定价页 Standard 标签只有一行输入价、一行输出价；推算，非原文声明） | 已找到 |
| C2d 与 1028 对照：到期日 | L1 | 1028 博文 c-blog-1028.txt 脚注：“After the introductory period expires, the price of $4 per 1M input tokens and $20 per 1M output tokens will apply.”（无日期）；AA 10/28 文：“Google is offering a 50% launch discount, with no confirmed end date.”（docs/2610/1028/sources/a-aa-article.txt） | 本次新增：到期日 November 30, 2026（博文脚注、定价页、changelog） | 25、26 | 变化：1028 无到期日，现为 11/30 | 已找到 |
| C2e Argon 同期同价、同到期日 | L1 | 定价页 Argon 块（c-api-pricing-rendered.txt 第 324–359 行） | “Input price … $2.00 through November 30, 2026. $4.00 starting December 1, 2026.” | — | Argon 与 Carbon 同价同到期 | 已找到 |
| C2f 模型页措辞（Carbon 多用 token） | L1 | 博文；https://aistudio.google.com/docs/whats-new-gemini-4-carbon | 博文：“using more tokens than Argon to explore thoroughly, reason deeply, and run extended verification loops” ；what’s new：“Carbon spends more thinking tokens per turn to explore hypotheses, verify its work, and check edge cases before responding.” 以及“share the same 1M unified context window, built-in tools, and per-token pricing.” | 25 | 同价按每 token 计；每个任务耗多少 token 不同 | 已找到 |

### C3 “more than 6X lower in average cost per task” 与 AA 的 $31.97 / $6.29

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C3a Google 博文原句 | L1 | 博文（同 C1a） | “Carbon ranks #2 while being more than 6X lower in average cost per task compared to the top performing model.” | 27 | 未给算法、价格口径、档位；MKA 链接指向 AA 榜单页 | 已找到 |
| C3b Google MKA 博文措辞 | L1 | https://blog.google/innovation-and-ai/models-and-research/gemini-models/multimodal-knowledge-agent-benchmark/（c-mka-blog.txt） | “Gemini 4 Carbon achieves #2 in quality among the models we tested, and completes tasks at 6x lower cost on average compared to the top scoring model.” ；“Gemini 4 Carbon is on the pareto frontier of quality and price for the MKA eval.” | — | 页面元数据 datePublished 2026-11-10T16:00:00+00:00；措辞为“6x”不带“more than” | 已找到 |
| C3c DeepMind 技术报告表 4（6X 的出处） | L1 | https://storage.googleapis.com/deepmind-media/papers/mka_tech_paper.pdf（c-mka-paper.pdf/.txt，第 11 页） | “Table 4 | Mean per-task token usage, agent turns, runtime and list-price API cost (with prompt caching) on MKA.” ；“Claude Fable 5.2 40.48M 40.10M 378.9k 128.8 83.2 $35.28”；“Gemini 4 Carbon 7.05M 6.92M 129.5k 46.6 29.2 $5.63” ；“At provider list prices, this corresponds to a 6.26× higher API cost ($35.28 vs. $5.63 per task) for a statistically non-significant +2.22 pp Climb-WSR difference.” ；“API costs use provider list prices at the time of writing and will change.” | 32 | 论文日期 2026-11-10；四个模型（Carbon、Fable 5.2、Opus 5.5、GPT-6 Astra）；“93.7%–97.7% of cumulative input tokens are served from cache”；Opus 5.5 $10.29、Astra $6.03。论文未写 Carbon 按体验价还是标准价算 | 已找到 |
| C3d AA 的数字 | L3 | https://artificialanalysis.ai/articles/google-deepmind-multimodal-knowledge-agent（c-aa-mka.txt）；榜单页 https://artificialanalysis.ai/evaluations/mka（c-aa-mka-board.html/.txt） | “Claude Fable 5.2 averages $31.97 per task, over 5× Gemini 4 Carbon ($6.29) for ~2 extra points, and ~29× GPT-6.1 Sol ($1.11).” ；榜单图 Cost per Task：Fable 5.2 (max) $31.98（柱顶），Carbon (high) $6.29，Opus 5.5 (max) $10.23，Argon (high) $7.98，Astra (max) $6.21 | 28、34 | AA 页写 $31.97，图柱顶写 $31.98（AA 内部不一致 0.01）；31.97/6.29 ≈ 5.08（推算）；“Costs include calls to the model under test only. Judge calls are excluded.”；AA 文与榜单页均未写按标准价还是体验价 | 已找到 |
| C3e AA 对 Carbon 的价格口径 | L3 | AA 模型页 https://artificialanalysis.ai/models/gemini-4-carbon（c-aa-carbon.txt）；MKA 榜单页数据载荷（c-aa-mka-board.html，Next.js 内嵌 JSON，价格字段 price1mInputTokens 等）；AA 10/28 文（a-aa-article.txt，docs/2610） | 模型页：“In $4.00Out $20.00Cache discount 95%” ；MKA 榜单页载荷中 Gemini 4 Carbon (High) 记录：price1mInputTokens 4、price1mOutputTokens 20、cacheHitPrice 0.2（Fable 5.2 记录：8 / 40；Opus 5.5：4 / 20）；10/28：“All our charts and figures use standard pricing for both Gemini 4 Carbon and Argon.” | — | AA 在 MKA 页面里存的 Carbon 单价是 $4/$20（标准价，非体验价 $2/$10）；MKA 页文字没有重申“standard pricing”，但载荷里的价格字段可核；成本数字本身（$6.29）是页面脚本在浏览器里算出的，HTML 里没有，只有图可看 | 已找到 |
| C3f 各模型 effort 档位 | L3 | AA 榜单 | “Claude Fable 5.2 (Max, Default Fallback)”；“Gemini 4 Carbon (High)”；“Claude Opus 5.5 (Max, Default Fallback)” | 28、29 | 对比是 Fable max 档（带默认 fallback）对 Carbon high 档；不是同档位 | 已找到 |
| C3g 质量差是否显著（技术报告） | L1 | 同 C3c，第 11 页 | “the top three models—Claude Fable 5.2, Gemini 4 Carbon, and Claude Opus 5.5 (53.01% Climb-WSR, 70.98% Overall WSR)—are statistically indistinguishable on Climb-WSR (Fable 5.2 vs. Carbon: +2.22 pp, 95% CI [−0.94, +5.49]; Carbon vs. Opus 5.5: +1.33 pp, 95% CI [−1.70, +4.41])” | 32 | 该检验是 Google 自己的论文做的；AA 页只写“~2 extra points” | 已找到 |

并列记录（不判谁对）：

| 来源 | Fable 5.2 | Carbon | 倍数 | 口径 |
|---|---|---|---|---|
| Google 博文 | — | — | “more than 6X” | 未写算法 |
| Google MKA 博文 | — | — | “6x” | 未写算法 |
| DeepMind 技术报告表 4 | $35.28 | $5.63 | 6.26×（报告原文） | list prices，含 prompt caching；Carbon 用哪档价未写 |
| AA 文 | $31.97 | $6.29 | “over 5×”（31.97/6.29≈5.08，推算） | AA 自算成本，不含评委调用 |
| AA 榜单图 | $31.98 | $6.29 | ≈5.08（推算） | 同上 |

推算（执行方自己算的，不是来源的说法；仅作线索）：取技术报告表 4 的 Carbon 令牌数（输入 6.92M、输出 129.5k）与报告正文“93.7%–97.7% of cumulative input tokens are served from cache”，缓存读价按 AA 载荷的 Carbon 缓存价，未计缓存写入：

| 价格档 | 缓存占比 93.7% | 缓存占比 97.7% |
|---|---|---|
| 标准价 $4 / $20 / 缓存 $0.2 | $5.63 | $4.58 |
| 体验价 $2 / $10 / 缓存 $0.1 | $2.82 | $2.29 |

算式：(1−c)×6.92×输入价 + c×6.92×缓存价 + 0.1295×输出价（单位百万 token、美元）。缓存占比取区间低端、标准价时得到 $5.63，与报告数字一致；这只能说明量级与标准价口径吻合，Carbon 的缓存占比报告没单独给，所以不能说报告“就是”按标准价算的。Fable 5.2 的缓存价与读写拆分没有找到，未做同样推算。若按体验价，Carbon 的任务成本约为一半，Fable 对 Carbon 的倍数会高于 6.26×；12 月 1 日后回到标准价口径（推算）。

### C4 MKA 是什么、谁出的

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C4a 合作关系 | L1/L3 | 博文；AA 文 | 博文：“we partnered with Artificial Analysis to launch the new Multimodal Knowledge Agent (MKA) benchmark that tests a model’s multimodal performance in agentic workflows.” ；AA：“We’ve partnered with Google DeepMind to launch their Multimodal Knowledge Agent (MKA) evaluation, a new benchmark” | 27、33 | AA 文日期 November 10, 2026（官方未给时刻）；AA 文末：“bring it exclusively to Artificial Analysis!” | 已找到 |
| C4b 任务数与评分 | L3 | AA 文；https://artificialanalysis.ai/methodology/intelligence-benchmarking#mka（c-aa-methodology.txt） | “200 private real-world visual tasks across 10 industries” ；“Each task is graded against expert rubrics, averaging ~70 weighted criteria per task.” ；“The headline score is Climb WSR (Weighted Satisfaction Rate) - the weighted proportion Climb rubric criteria successfully completed” ；方法页：“The rubrics hold 14,284 criteria in total, each with a weight from 1 to 5.” ；“Status: Standalone evaluation (not part of Artificial Analysis Intelligence Index v5.0.1)” | 33 | Climb 与 Regression 的划分由基准作者做；每个模型每任务跑一次（“Each model makes one run per task.”） | 已找到 |
| C4c 评委与 harness | L3 | AA 文；榜单页 | “We run every task in Stirrup, our open source agent harness, with code execution, web search, web fetch, and image-viewing tools (up to 250 turns), and grade submitted deliverables using GPT-6 Luna (medium) as an agentic judge” | 33 | Google 博文：“An agentic autorater built on a non-Google model grades each deliverable.” ；技术报告：评委与专家一致率 96.2%（专家小样本，报告限制一节写“covers 50 tasks”） | 已找到 |
| C4d 数据集私有，另放 10 个公开任务 | L1 | Google MKA 博文；https://huggingface.co/datasets/google/multimodal-knowledge-agent | “Artificial Analysis runs the 200-task evaluation set and keeps it private so the tasks can't leak into training data or be found by the model via search. We've also released a separate set of 10 public tasks on Hugging Face” | — | 读者无法自行复核全部 200 题 | 已找到 |
| C4e 技术报告的限制声明 | L1 | c-mka-paper.txt（局限一节） | “The climb/regression panel includes two earlier Gemini models, so the split reflects what that panel already solves.” ；“The vision ablation is run on Gemini 4 Carbon only” | — | 报告作者是 Google DeepMind | 已找到 |

### C5 “internal benchmark” 与 “2X” 第三方评测

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C5a Project Fortify 100% / 95% | L1 | 博文 | “In Project Fortify, Google’s autonomous system spanning scanning, triage, and remediation, Carbon hit 100% recall on scanning and 95% precision on triage, on our internal benchmark.” | 31 | 谷歌内部基准，未给样本量、基线与方法；博文称“Early testers inside and outside of Google” | 已找到 |
| C5b “over 25% relative improvement” | L1 | 博文 | “On Open Source code repository testing we see an over 25% relative improvement on Argon's already strong recall rate, surfacing previously undetected, real-world logic flaws.” | 31 | 相对 Argon 的召回提升，没给绝对值 | 已找到 |
| C5c “more than 2X lower cost per task”（博文原句） | L1 | 博文 | “It ranks #1 on CursorBench 4.0, Perplexity’s WANDR, and Zapier’s AutomationBench at more than 2X lower cost per task compared to the next best external model.” | 25 | “next best external model” 未点名 | 已找到 |
| C5d CursorBench 4.0 榜单 | L3 | https://cursor.com/cursorbench（c-cursorbench.html/.txt，2026-11-11 取） | 榜单行（前 20 行）：“1 Gemini 4 Carbon High 60.4% $9.22 …”；“2 Gemini 4 Carbon Medium 60.1% $8.49”；“3 Fable 5.2 Max 60.0% $32.79”；“5 Opus 5.5 Max 57.8% $13.43”；“14 Gemini 4 Argon High 52.9% $4.57” ；页脚：“Avg cost / task is computed by applying each model's published per-million-token pricing (input, cache read, cache write, and output) to the tokens it used on each task. Results are subject to variance; small differences in scores may not be statistically meaningful.” | 无（页面表格较长，榜单文本在 txt 存档里） | Carbon High 对 Fable 5.2 Max：60.4% 对 60.0%（差 0.4 个点），成本 $9.22 对 $32.79（32.79/9.22≈3.6，推算）；页面未写按体验价还是标准价算 Carbon（“published pricing”）。Cursor 的模型价格页 https://cursor.com/docs/models-and-pricing（c-cursor-pricing.txt）列 Carbon 为 “$2-$0.1$10”（输入 $2、缓存读 $0.1、输出 $10，即体验价，页面无到期说明），Fable 5.2 为 “$8$10$0.2$40”。推算（未验证）：若 CursorBench 的 $9.22 用的就是这个 $2/$10，标准价下约为其两倍（≈$18.4），对 Fable 5.2 Max 的 $32.79 约 1.8×，低于“more than 2X”；需向 Cursor 或 Google 求证 | 已找到 |
| C5e Zapier AutomationBench 榜单 | L3 | https://zapier.com/benchmarks（c-zapier.html/.txt） | 榜单：“1 Gemini 4 Carbon (High) 53.88% $2.12”；“2 Gemini 4 Carbon (Medium) 52.97% $1.65”；“3 Claude Fable 5.2 (default fallbacks, Max) 51.45% $4.64”；“4 Gemini 4 Argon (High) 51.29% $1.70*” ；页脚：“*These Gemini models have promotional pricing; ranking and Cost / task use standard list pricing. Gemini 4 Argon: $0.85 (High), $0.77 (Medium).” | 35 | Carbon 行没有星号；页脚只点名 Argon 与 Flash 系列为促销定价，Argon 带星号的成本是标准价（$1.70，体验价下为 $0.85）。Carbon High 对 Fable 5.2：53.88% 对 51.45%，成本 $2.12 对 $4.64（≈2.19×，推算）；Fable 行标注“default fallbacks”。Zapier 的 Carbon 报告页 https://zapier.com/benchmarks/gemini-carbon-4-report（c-zapier-carbon.txt）：“Cost per task$2.12High effort · list-price estimate”；“These are estimates at list rates, not billed costs.”；并写“The Claude models appear only here, since their published costs use a different basis.”（即 Zapier 自己说 Claude 的已发布成本与该报告的成本口径不同）。报告页没有写 Carbon 的 list rate 是体验价还是标准价 | 已找到 |
| C5f Perplexity WANDR | L3 | https://www.perplexity.ai/hub/blog/wandr-benchmark-evaluating-research-agents-that-must-search-wide-and-deep（c-wandr.txt）；https://github.com/perplexityai/wandr（c-wandr-gh.txt、c-wandr-raw.txt） | 无 | 无 | 博文与 GitHub README 中没有查到 Carbon 的成绩行；博文是方法与自家 agent 对比，“We also evaluated every available effort setting on the same 45-task subset.” 说明有榜单数据，但没有找到公开的 Carbon 行 | 未找到一手来源 |
| C5g Hebbia、Balyasny 等第三方评测 | L1 自述 | 博文 | “In pre-release evaluations, Carbon matched the performance of the best model tested on deep financial research on Hebbia’s E2E Financial Agentic evaluation, and ranked at the top out of 23 frontier model configurations on Balyasny Asset Management’s General Financial Intelligence benchmark” | — | “pre-release evaluations”，合作方成绩由 Google 博文转述，没找到合作方公开榜单（未单独查） | 部分支持 |

### C6 智能指数位次

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C6a Fable 5.2 (max) 65 > Carbon (high) 60 > Opus 5.5 (max) 59 | L3 | https://artificialanalysis.ai/leaderboards/models（c-aa-leaderboard.txt）；AA 模型页（c-aa-fable52.txt、c-aa-carbon.txt） | 榜单：“Claude Fable 5.2 (max) … 65 $26.54”；“Gemini 4 Carbon (high) … 60 $7.76”；“Claude Opus 5.5 (max) … 59 $10.04”；“Gemini 4 Carbon (medium) … 58 $6.59”；页面摘要：“The top models by Intelligence Index are: 1. Claude Fable 5.2 (Max, Default Fallback) (65), 2. Claude Fable 5.2 (Xhigh, Default Fallback) (62), 3. Claude Fable 5.2 (High, Default Fallback) (62), 4. Gemini 4 Carbon (High) (60), 5. Claude Opus 5.5 (Max, Default Fallback) (59).” | 30 | AA Intelligence Index v5.0.1；成本列是“weighted average cost per Intelligence Index task”；2026-11-11 取。线索写 Carbon 59，AA 页现为 60；1028 期 AA 页面是 v5（59）。AA changelog 另有 03 Nov 2026 的 v5.0.1 条目（“regraded all models on CritPt v2”，见 C6c）；两件事的因果 AA 没写，只能并列：59（10/28，v5）→60（11/11，v5.0.1），推算（未验证）与该次重评有关。同一榜单 Argon (high) 现为 50 分、$9.95，而 1028 的 AA 文写 Argon $7.68（a-aa-article.txt）——两处数字并列记录 | 与说法不符 |
| C6b AA 11/09 changelog 的 Fable 5.2 | L3 | https://artificialanalysis.ai/changelog（c-aa-changelog.txt） | “09 Nov 2026 … Claude Fable 5.2 (Max, Default Fallback) Intelligence Index: 65” | — | 日期只有 09 Nov 2026，AA 未给时刻 | 已找到 |
| C6c 指数版本变更 | L3 | 同 C6b | “03 Nov 2026 … Artificial Analysis Intelligence Index v5.0.1 Incorporated grading robustness improvements from the CritPt authors and regraded all models on CritPt v2.” | — | 变更日期以 changelog 条目所在日期为准（03 Nov 2026） | 已找到 |
| C6d Carbon 的指数成本与话多程度 | L3 | AA 模型页 | “When evaluating the Intelligence Index, it generated 205M tokens, which is very verbose in comparison to the median of 72.8M.” ；“On average, it costs $7.76 per task to evaluate Gemini 4 Carbon (High) on the Intelligence Index.” ；“Gemini 4 Carbon (High) takes 25.6 minutes per Intelligence Index task” | — | Fable 5.2 (max)：“it generated 275M tokens”，“$26.54 per task”，“45.2 minutes per Intelligence Index task”。同一页另有 Argon (high) 50 分 $9.95（榜单） | 已找到 |
| C6e MKA 榜单分数 | L3 | AA 榜单页 | “Claude Fable 5.2 (Max, Default Fallback) scores the highest on MKA Climb WSR with a score of 57%, followed by Gemini 4 Carbon (High) with a score of 54% and Claude Opus 5.5 (Max, Default Fallback) with a score of 53%” | 28、29 | 榜单页另写：“GPT-6 Luna (Max) has the lowest cost per task on MKA, at $0.18” | 已找到 |
| C6f AA 对 Carbon 综合判断（1028） | L3 | docs/2610/1028/sources/a-aa-article.txt | “Carbon makes substantial gains elsewhere but is not leading in coding and knowledge work”；幻觉率：“Accuracy (% correct) rises 10 points over Argon to 58%, while its hallucination rate rises from 4% to 11%.” | — | 1028 的 AA 文原文，本次未有新的 Carbon 专文；MKA 文只讲 MKA | 已找到 |

### C7 与 1028 的差别

| 项 | 1028 | 本次（11-10） | 来源 |
|---|---|---|---|
| 开放范围 | 先向“a subset of cyber defenders”，AA：“is being rolled out to selected users and is not yet publicly available” | 付费 API（AI Studio、Android Studio）、Gemini for business、Ultra 订阅（Gemini 应用、AI Mode、Antigravity）；免费层“Not available”；AI Pro 未列 | c-blog-1028.txt；a-aa-article.txt；c-blog.txt |
| 价格 | “Gemini 4 Carbon will launch at the same introductory price …” 脚注“After the introductory period expires”，无日期；AA：“50% launch discount, with no confirmed end date” | 体验价 $2/$10，到期 2026-11-30，12/1 起 $4/$20；缓存价 $0.10→$0.20 | c-blog-1028.txt；c-blog.txt；c-api-pricing-rendered.txt |
| 指数 | AA v5：Carbon (high) 59，Opus 5.5 (max) 58 | v5.0.1：Fable 5.2 (max) 65，Carbon (high) 60，Opus 5.5 (max) 59 | docs/2610/1028/sources/fact-check.md；c-aa-leaderboard.txt |
| 网络安全成绩 | 标注为关安全限制测、两项自跑 | 博文仍写 “When accessed through the Fairwind Program, Carbon leads on key benchmarks across finding (SEC-bench Pro), exploiting (ExploitGym), and patching (CWE-bench v1)”；模型卡：成绩“reflect performance when accessed through the Fairwind Program” | c-blog.txt；c-modelcard.txt 第 5 页 |
| 新增内容 | — | MKA 基准（Google 与 AA 合作）、Project Fortify 100%/95%（内部基准）、CursorBench 4.0 / WANDR / AutomationBench 第一、/swarm 预览、科学家免费 6 个月申请 | c-blog.txt |

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C7a 1028 时开放范围与价格，对照本次变化（上表逐项） | L1/L3 | c-blog-1028.txt；c-blog.txt；c-api-pricing-rendered.txt；docs/2610/1028/sources/fact-check.md | 见上表各格逐字引文 | 25、26、36 | 对照基于 1028 存档与 fact-check.md 第 17–22 行；1028 当时 Gemini API 模型页、价格页无 Carbon（fact-check.md 第 17 行） | 已找到 |

### C8 热度与报道

| 项 | 数字 | 来源 | 时间 |
|---|---|---|---|
| HN 50442303 “Gemini 4 Carbon is now available” | 282 分，155 条评论（含嵌套，Algolia 计） | `barking heat`（c-heat.json、c-hn-50442303.json） | 提交 2026-11-11 00:07:15 北京（2026-11-10 16:07:15 UTC）；取数 2026-11-11 14:38 北京 |
| HN 另有同题帖 | 50442361 Sundar 帖 14 分/0 评；50444240 GoogleAI 帖 6 分/1 评；50438001 “Tell HN: Gemini Carbon released” 3 分 | `barking heat` | 同上 |
| @sundarpichai 帖 2120185896469496180 | 597,264 浏览、7,212 赞、505 回复、522 转发 | fxtwitter（c-fx-2120185896469496180.json） | 2026-11-11 00:04:43 北京（2026-11-10 16:04:43 UTC）；取数 14:35 前后 |
| @Google 帖 2120186185423159331 | 1,885,525 浏览、9,217 赞、562 回复、955 转发 | c-fx-2120186185423159331.json | 2026-11-11 00:05:52 北京（16:05:52 UTC） |
| @GoogleAIStudio 帖 2120187576489222302 | 168,674 浏览、2,316 赞、109 回复、233 转发 | c-fx-2120187576489222302.json | 2026-11-11 00:11:24 北京（16:11:24 UTC） |
| @GoogleAI 帖 2120187753329791341 | 363,219 浏览、5,889 赞、226 回复、600 转发 | c-fx-2120187753329791341.json | 2026-11-11 00:12:06 北京（16:12:06 UTC）；线索里的“GoogleAI 帖 16:12:06 UTC”就是这条 |
| Reddit | 搜索被重定向到登录页，失败（已记入 c-heat.json errors）；未登录 | — | — |

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C8a HN 50442303 | L6 | https://news.ycombinator.com/item?id=50442303（c-hn-50442303.json） | 标题“Gemini 4 Carbon is now available” | — | 282 分、155 评（含嵌套）；2026-11-11 00:07:15 北京提交；14:38 取数；仅热度，不作事实 | 已找到 |
| C8b Sundar 与 Google 官方帖浏览与赞 | L1 | fxtwitter JSON（四条，见上表） | Sundar 帖：“Gemini 4 Carbon is rolling out today.” | — | 浏览 597,264/赞 7,212（Sundar）；@GoogleAI 363,219/5,889；@Google 1,885,525/9,217；@GoogleAIStudio 168,674/2,316；时刻见上表 | 已找到 |
| C8c 中文侧只写“同价/降价”不提 11/30 的实例 | L5 | 所查的中文稿：新浪（AI信息Gap）、电脑王阿达、硬是要學、DataLearner，另见搜索结果里的 aiposthub（未存档，标题含“11/30 優惠期限”） | 见第 3 节 | — | 共查 3 篇新闻稿 + 1 个参数库页；搜索未命中 IT之家、36氪、21经济网、新浪快讯等 1028 期用过的简体中文站（两次中文搜索，均只返回英文与繁体中文稿）；3 篇正文都提了 11/30，没有找到完全不提的稿件 | 未找到一手来源 |

HN 评论（仅线索 L6；Algolia 导出不含评论分数，所以只记评论号与发帖时间）：
- 50442424 byefruit（11-11 00:15:41 北京）：“Just checked my Gemini app, still only have 3.1 Pro, 3.6 Flash and 3.6 Thinking on Pro. Looks like it's Ultra only.”（线索里“Gemini 应用里仍只有 3.1 Pro”即此条，自述）
- 50442453 eitally（00:17:25）：自称 Ultra 付费用户，Carbon 已出现在 Antigravity 与 Gemini 应用里；“Argon is itself already so slow that I'm not sure Carbon will make sense for most activities.”（自述）
- 50442564 spockz（00:24:40）：“On pro here and I can do about 2 design tasks a week before I hit usage limits with Argon.”（自述，涉及 Pro 额度）
- 50443659 WarmWash（01:50:01）：称 Google 有多种“Pro”计划，模型访问与额度取决于 Workspace Pro、促销 Pro 等（自述，未核）
- 50442498 Murrawhip（00:20:16）：自称 Workspace 账号的 Gemini 里已有 “4 Carbon”，个人 AI Pro 没有（自述，与 Ultra-only 说法相左，说明企业侧已可见）
- 50443704 dom96（11-11 01:54 北京，原文 2026-11-10T17:54Z）：自称在自己的基准上 Carbon 与 Fable 5.2 持平，“it is still slower and more costly than Fable 5.2”（自述，基准链接在评论里，未核；与 Google 的成本说法方向相反，仅线索）
- 50443058 mattlondon（11-11 00:59）：称 AA 的 Omniscience 榜单上 Carbon 是“best at not hallucinating”（观点，指 AA-Omniscience；1028 的 AA 文写幻觉率 4%→11%，仍是第二低，见 C6f）
- 50449895 tmaly（11-11 04:57）：称 OpenRouter 上 Carbon 现为 50% 折扣价，“when 50% goes away it will go to Opus 5.5 price”（自述，未核；OpenRouter 页未查）
- 50443344 panarky（01:23:11）：认为先给可信测试者、再给 Ultra、之后可能给 Pro 是“completely fair and normal business practice”（观点）

报道（3–5 篇代表性）：
1. Google 官方博文（L1，见上）。
2. 9to5Google，https://9to5google.com/2026/11/10/gemini-app-4-carbon/（c-news-9to5google.txt，L4）：标题“Gemini 4 Carbon rolling out to Gemini app for AI Ultra, Argon remains”；正文“Notably, the Gemini app has removed 3.1 Pro with this launch.”；“In the model picker, 4 Argon is for “Code & analysis” and 4 Carbon is for “Most complex work.””；价格段“until November 30. Afterward, it will cost $4 per 1M input tokens and $20 per 1M output tokens.”
3. Android Authority，https://www.androidauthority.com/google-gemini-4-carbon-rollout-3733702/（c-news-androidauthority.txt，L4）：“Google’s announcement gives no timeline for access on lower-priced plans.” ；“Both Carbon rates double on December 1.”
4. Engadget，https://www.engadget.com/2311855/gemini-4-carbon-now-available-worldwide/（c-news-engadget.txt，L4；页面时间 Nov. 10, 2026 11:00 am EST，即北京 11-11 00:00；datePublished 2026-11-10T16:00:00+00:00）：标题“Google's Gemini 4 Carbon Is Now Widely Available Around The World”，正文并写“Google AI Ultra subscribers paying $100 a month for membership will also get access to Carbon within the Gemini app”，未写价格与 11/30 到期。
5. 中文：新浪（作者署“AI信息Gap”）https://k.sina.cn/article_5952915705_162d248f906703t0x8.html（c-zh-sina.txt，L5 二手稿；页面元数据 2026-11-11 06:14:44）：标题“刚刚，Gemini 4 Carbon 终于开放！价格仅 Fable 1/4，Pro 会员却没份”，正文：“划重点，Carbon 的尝鲜价 11 月 30 日截止，之后会恢复为原价，每百万输入 token 4 美元、输出 20 美元。” ；“Carbon 恢复原价后，是它的一半。” ；并写“在第三方评测机构 Artificial Analysis 的智能指数上，Claude Fable 5.2 以 65 分遥遥领先，Gemini 4 Carbon 以 60 分排名第二”。
6. 电脑王阿达（台湾）https://www.kocpc.com.tw/archives/702975（c-zh-kocpc.txt，L5）：标题“Gemini 4 Carbon 全面開放！…11 月底前價格減半”，正文写“11 月 30 日前是每百萬輸入 Token 2 美元…12 月 1 日起調回 4 美元與 20 美元”。
7. 硬是要學（台湾）https://www.soft4fun.net/tech/news/google-gemini-4-carbon-now-available-swarm-antigravity-price-2-10-2.htm（c-zh-soft4fun.txt，L5）：标题“Gemini 4 Carbon 正式開放：API 價格與 Argon 相同，三項獨立基準登頂、成本低兩倍以上”，正文有 11 月 30 日到期句。

英文财经/聚合：Blockonomi“Gemini 4 Carbon Launches With Prices Set to Double in December”（c-news-blockonomi.txt）、BigGo Finance“Google Opens Gemini 4 Carbon to All Paying Developers, Puts $2/$10 Introductory Token Pricing on the Clock”（c-news-biggo.txt，写“Pro and free users are excluded”）。

## 2. 可能的吠点（线索 L6；进正文前必须回到一手来源）

1. “全面开放”的口径：博文开头 “making Gemini 4 Carbon available to developers, enterprises, scientists, and consumers”，落到 Getting started 是：开发者要付费 API 层，普通用户仅 Ultra 订阅，科学家与网络安全防御者要申请（博文原句，截图 25、36）；免费层“Not available”（定价页，截图 26）。
2. 体验价有到期日：$2/$10 只到 2026-11-30，12 月 1 日起 $4/$20（定价页、脚注 1，截图 26）；1028 当时只写“introductory period”无日期（c-blog-1028.txt）。“与 Argon 同价”也是同一个体验价，Argon 同样到 11/30。
3. “同价”不等于“同任务花费”：博文自己写 Carbon “using more tokens than Argon”（截图 25），文档写 “Carbon spends more thinking tokens per turn”（whats-new）。AA 数据里 Carbon 的指数成本 $7.76 对 Argon (high) $9.95（AA 榜单，标准价）；MKA 上 Carbon $6.29、Argon $7.98——并不是 Carbon 更贵，只记录，不推论。
4. “more than 6X” 的算法在技术报告里（list prices，含缓存；$35.28 对 $5.63），AA 页写 “over 5×”（$31.97 对 $6.29）；比较的是 Fable 5.2 max 档对 Carbon high 档（AA 榜单，截图 28、32、34）。
5. “ranks #2” 的差距：技术报告自己写 Fable 5.2 对 Carbon +2.22 pp，95% CI [−0.94, +5.49]，“statistically non-significant”；Carbon 对 Opus 5.5 +1.33 pp，95% CI [−1.70, +4.41]；前三名“statistically indistinguishable”（截图 32）。
6. MKA 的出题与计价都是 Google 与 AA 的合作：基准由 DeepMind 出，AA 独家上线，评委是 GPT-6 Luna (medium)；200 题不公开，只放 10 题公开（c-mka-blog.txt、AA 文，截图 33）。
7. 智能指数不是第一：Fable 5.2 (max) 65，Carbon (high) 60，Opus 5.5 (max) 59（AA，截图 30）；1028 时是 59 对 58，指数 v5.0.1 重评了 CritPt v2。
8. “internal benchmark”：Project Fortify 的 100% 召回与 95% 精度是谷歌内部基准，没有样本量与对照（博文，截图 31）。
9. “more than 2X lower cost per task”：对象是“next best external model”（未点名）。CursorBench 4.0 榜上 Carbon High 60.4% 对 Fable 5.2 Max 60.0%，成本 $9.22 对 $32.79；Zapier 上 53.88% 对 51.45%，成本 $2.12 对 $4.64（Zapier 注明按标准价排名与计成本，截图 35）；WANDR 未找到 Carbon 的公开行。Zapier 报告页自己写 Claude 的已发布成本口径不同；Cursor 的价格页把 Carbon 列为 $2/$10，榜单成本若按此算，标准价下倍数会变小（推算，见 C5d）。
10. 网络安全成绩的版本：博文与模型卡把 SEC-bench Pro、ExploitGym、CWE-bench v1 的成绩归在 “when accessed through the Fairwind Program”；Carbon for cyber 仍需申请（博文 Getting started 最后一条，截图 36）；公开版 Carbon 在这三项上的成绩，博文与模型卡未给（安全报告未读）。
11. Gemini 应用里换了模型：9to5Google 写 “the Gemini app has removed 3.1 Pro with this launch”（L4）。
12. HN 上 Pro 用户反映：只给 Ultra，Pro 额度已紧（评论 50442424、50442564，自述，L6）。

## 3. 夸大说法实例

找到：
1. Engadget，2026-11-10（北京 11-11 00:00），英文标题“Google's Gemini 4 Carbon Is Now Widely Available Around The World”（https://www.engadget.com/2311855/gemini-4-carbon-now-available-worldwide/；c-news-engadget.txt）：正文只写 AI Studio、Gemini for business 与 Ultra 订阅，标题用 “Widely Available”。该稿没有写任何 API 价格，也没写 11/30 到期。
2. 硬是要學（soft4fun.net），2026-11-11，繁体标题“Gemini 4 Carbon 正式開放：API 價格與 Argon 相同，三項獨立基準登頂、成本低兩倍以上”：标题没有体验价到期，正文有“只適用到 11 月 30 日”；“三项独立基准登顶、成本低两倍以上”转述 Google 自家措辞，没有提到“next best external model”。
3. 新浪（AI信息Gap），2026-11-11（页面元数据 06:14:44），标题“刚刚，Gemini 4 Carbon 终于开放！价格仅 Fable 1/4，Pro 会员却没份”：“1/4”是体验价（$2/$10 对 Fable 5.2 的 $8/$40）；正文同时写了 11/30 到期后“是它的一半”，并写“Carbon 以 60 分排名第二”。标题的“价格仅 1/4”只在体验价内成立。
没找到：把“同价/降价”完全不提 11/30 的中文稿——所查到的 3 篇中文稿正文都提了 11/30；Engadget 英文稿未提。

## 4. 扫描说法勘误

| 线索说法 | 原文核对 | 结论 |
|---|---|---|
| Carbon 综合指数 59 | AA 榜单现为 60（v5.0.1，2026-11-11 取）；1028 时 59 | 现为 60，59 是旧版；Fable 5.2 (max) 65 属实 |
| “more than 6X” 与 AA “$31.97 / $6.29” 对不上，Google 用的哪个价格未说明 | 找到出处：DeepMind 技术报告表 4（$35.28 对 $5.63，6.26×，list prices 含缓存） | 并列，不判谁对；价格档位（体验价或标准价）报告未写 |
| GoogleAI 帖 16:12:06 UTC | @GoogleAI 帖 2120187753329791341 属实（16:12:06 UTC）；另有 @Google 16:05:52、@GoogleAIStudio 16:11:24 | 属实 |
| 脚注 1 “After the introductory period expires on November 30, 2026 …” | 属实，原文逐字；脚注折叠在文末 “More Information” 内 | 属实 |
| “Try Carbon starting today in the Gemini app and AI Mode in Search if you’re a Google AI Ultra subscriber” | 属实，逐字 | 属实 |
| HN 50442303：282 分/155 评 | 属实（282 分，嵌套 155 条） | 属实 |
| HN 高赞评论称 Gemini 应用里仍只有 3.1 Pro 等 | 评论 50442424（自述），Algolia 导出无评论分数，故“高赞”未能核 | 部分支持 |
| AA 10/28 “50% launch discount, no confirmed end date” | 属实（docs/2610/1028/sources/a-aa-article.txt）；现已有 11/30 到期日 | 属实，已被官方页更新 |
| Project Fortify 100%/95% “on our internal benchmark” | 属实，逐字 | 属实 |
| 3 个第三方榜单“more than 2X lower cost per task compared to the next best external model” | 博文原句属实；CursorBench 与 Zapier 榜单可核，WANDR 未找到 Carbon 行 | 部分支持 |
| AA changelog 11/09：Fable 5.2 (Max) 综合指数 65，高于 Carbon 的 59 | changelog 属实（65）；Carbon 现为 60 | 65 属实，59 已更新为 60 |

## 5. 取证截图（`images/25–36`，浅色页面，均已逐张打开目视确认）

| 文件 | 内容 | 原页面 | 备注 |
|---|---|---|---|
| 25-blog-price-ultra.png | 博文“Today, we’re making …”至“Ultra subscribers”整段：开放范围、“more than 2X lower cost per task”、“introductory price 1”、Ultra 订阅 | 博文 | 700 CSS px、DPR 2 |
| 26-pricing-carbon.png | Gemini API 定价页 Gemini 4 Carbon 整块：免费层不可用、Standard 标签的输入、输出、缓存价与 11/30、12/1 日期 | https://ai.google.dev/gemini-api/docs/pricing | 无头 Chrome 渲染，无登录态 |
| 27-blog-mka-6x.png | 博文“Higher-precision knowledge work …”小节，含 “Carbon ranks #2 while being more than 6X lower …” | 博文 | |
| 28-aa-mka-cost.png | AA MKA 榜单 Cost per Task 柱图（含 Carbon $6.29、Fable 5.2 柱顶 $31.98、Argon $7.98、Opus 5.5 $10.23） | https://artificialanalysis.ai/evaluations/mka | 为让 21 根柱全部入图，按 920 CSS px、DPR 1.5（1380 px 宽）渲染，不是 700 px / 2 倍，已避开页面右下“切换到中文”弹窗 |
| 29-aa-mka-climb-wsr.png | AA MKA Climb WSR 柱图（57%/54%/53%） | 同上 | 同上渲染参数 |
| 30-aa-intelligence-index-board.png | AA LLM 榜单表头与前 6 行（Fable 5.2 max 65 $26.54；Carbon high 60 $7.76；Opus 5.5 max 59；Carbon medium 58） | https://artificialanalysis.ai/leaderboards/models | 700 px / DPR 2 |
| 31-blog-internal-bench.png | 博文“Frontier cybersecurity capabilities”开头与 Project Fortify 一条（“our internal benchmark”） | 博文 | |
| 32-mka-paper-cost-table.png | DeepMind MKA 技术报告第 11 页：表 3、表 4 与“6.26× higher API cost”“statistically non-significant”原句 | https://storage.googleapis.com/deepmind-media/papers/mka_tech_paper.pdf | PDF 渲页 165 dpi（1365 px 宽） |
| 33-aa-mka-overview.png | AA MKA 文“overview”：200 个私有任务、Climb 与 Regression、评委 GPT-6 Luna (medium) | https://artificialanalysis.ai/articles/google-deepmind-multimodal-knowledge-agent | |
| 34-aa-mka-cost-per-task.png | AA MKA 文 “Key MKA results”：Climb WSR 柱图、成本对分数散点图、“$31.97 … over 5× … ($6.29)” 段 | 同上 | |
| 35-zapier-automationbench.png | Zapier AutomationBench 榜单（Carbon High 53.88% $2.12 等）与页脚“promotional pricing; … standard list pricing” | https://zapier.com/benchmarks | 920 px / DPR 1.5；页面奶白底；表中 Fable 模型名被页面自己截断 |
| 36-blog-getting-started.png | 博文“Getting started with Gemini 4 Carbon”：付费 API 层、Ultra、科学家与 Fairwind 申请 | 博文 | |

未截图：脚注 1 原句（折叠在文末 More Information，无头渲染两次展开失败，只有 txt 存档）；CursorBench 4.0 榜单（本组编号区间 25–36 已用完，榜单文本在 `c-cursorbench.txt`）；WANDR（未找到 Carbon 行）。模型卡第 4、5 页原句（只有 PDF/txt 存档）。
