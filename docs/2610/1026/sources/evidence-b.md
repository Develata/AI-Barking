# 1026 期 B 组取证：GPT-6.1 Terra 的 AA 独立评测

执行方 claude-sonnet-5-5；取证时段 2026-10-27 北京时间 22:52–23:40。时间一律北京时间（UTC+8），括号保留原时区。本机美东，北京 = 本机 + 12 小时。存档前缀 `b-`，截图 `images/13`–`24`。抓取记录见 `capture-log-b.md`。

读图说明：AA 文章里的图表是 AA 站内原图，页面渲染后截图；数值直接读自柱上标注，**不是 AA 文字**，凡“读自图”的数字都标注。凡“推算”的数字是我用 AA 页面已有数字算的，不是 AA 或 OpenAI 的原话，标为（推算）。

## 一、发布时刻

| 项 | 时刻 | 来源 |
|---|---|---|
| OpenAI 发布页 | 页面只写 “October 26, 2026”（官方未给时刻）；OpenAI RSS pubDate 为 `Mon, 26 Oct 2026 00:00:00 GMT`（仅日期的占位值，不是发布时刻）。HN 提交 2026-10-26T18:01:17Z = 北京 10-27 02:01:17 | `b-openai-terra.html`、https://openai.com/news/rss.xml、`b-hn-50248124.json` |
| OpenAI 开发者文档模型页 | HN 提交 2026-10-26T17:58:24Z = 北京 10-27 01:58:24 | HN 搜索 API（Algolia），objectID 50248075 |
| AA 文章 | 页面只写 “October 26, 2026”（`article:published_time` 为 2026-10-26，无时刻）。AA 官方 X 账号同文首帖 2026-10-26T18:11:02Z = 北京 10-27 02:11:02（fxtwitter 与 opencli 时间线两处一致） | `b-fx-2114781865811063259.json` |
| AA 模型页 | “Released October 2026”，无具体日期 | `b-aa-model.txt` |

**扫描勘误**：WorkBuddy 称的“2026-10-25 18:17 UTC”未找到任何依据；AA 文章与官方帖都在 10-26 18:11Z。

## 二、事实清单（B1–B7）

截图列：`13`–`24` 均在 `docs/2610/1026/images/`。

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B1-1 | OpenAI 标准 API 价格 | L1 | https://openai.com/index/introducing-gpt-6-1-terra/ ；https://developers.openai.com/api/docs/models/gpt-6.1-terra | “Standard API pricing is $1 per million input tokens, $0.05 per million cached input tokens, and $5 per million output tokens.”；文档页另有 “Cache writes $1.25”（30 分钟保留；12 小时保留 $2.50） | 23 | 发布页日期 October 26, 2026，无时刻 | 已找到 |
| B1-2 | 价格相对前代与 Sol | L1 | 同上 | “Its standard API prices are 50% lower than those of GPT-5.6 Terra and GPT-6.1 Sol” | 20 | AA 页面核算：Terra $1/$5；GPT-5.6 Terra $2/$12；GPT-6.1 Sol $2/$10；GPT-6 Astra $10/$50；GPT-6 Luna $0.10/$0.50。即输入价对 5.6 Terra 与 Sol 均为 −50%；输出价对 5.6 Terra −58%，对 Sol −50%（推算）。AA 文章写 “half or less” | 已找到 |
| B1-3 | 上下文与输出上限 | L1 | https://developers.openai.com/api/docs/models/gpt-6.1-terra | “1,050,000 context window”；“128,000 max output tokens”；“Jun 01, 2026 knowledge cutoff”；“reasoning.effort supports none, low, medium (default), high, xhigh, and max.” | —（未截） | 发布页正文未写上下文长度；AA 模型页写 “1M tokens context window”（截图 19） | 已找到 |
| B1-4 | 开放范围 | L1 | 发布页 “Pricing and availability” | “Starting today, GPT-6.1 Terra is available to all Plus, Pro, Enterprise, and Edu users in ChatGPT Work and Codex. Developers can also access it through the OpenAI API as gpt-6.1-terra.” | 23 | 页面只列 Plus、Pro、Enterprise、Edu；**没有写** Free、Go、Business 是否可用（只是未提及，不能推断不可用）。“ChatGPT Work” 是页面原词 | 已找到 |
| B1-5 | ChatGPT 额度 | L1 | 发布页 | “In ChatGPT Work and Codex, your plan goes further with GPT-6.1 Terra—the same usage allowance covers more tasks than with GPT-5.6 Terra and substantially more than with GPT-6.1 Sol or GPT-6 Astra.” | 20 | 没有给具体倍数 | 已找到 |
| B1-6 | 对 Sol/Astra 的定位 | L1 | 发布页 “A significant step up on factuality and coding” | “For the most complex coding tasks, GPT-6.1 Sol and GPT-6 Astra remain the better choice.”；另：MMMU-Pro 一段写 “It even nearly matches the scores of our most capable frontier model, GPT-6 Astra, and GPT-6.1 Sol at much lower cost.” | 22 | 同段 OpenAI 称 Agents’ Last Exam 中等档 “performs on par with GPT-6 Astra at only one-tenth its price per token”（是单价，不是单任务成本；原文另给 “about one-third the cost” 对 5.6 Terra） | 已找到 |
| B1-7 | 官方自报基准 | L1 | 发布页 | Agents’ Last Exam：“At medium reasoning effort, GPT-6.1 Terra scores 15 percentage points higher than GPT-5.6 Terra at about one-third the cost.”；“At max effort, Terra is over three times as token efficient as Claude Opus 5.5, and one-fourth its price per token, resulting in a ~9x lower cost per task.”；Terminal-Bench 4.0 “At maximum reasoning effort, GPT‑6.1 Terra solves 50.3% of tasks, more than double GPT‑5.6 Terra’s published score of 21.5%.” | 21、22（页面 7 张图里的两张）| **发布页没有数字表，是 7 张只画曲线、不标数值的图**（视频、Agents’ Last Exam v1、MMMU-Pro (Vision)、OSWorld 2.1 offline set、Factual error rate、Terminal-Bench 4.0、Terminal-Bench-Science 0.1）。我只截了与 AA 对比相关的两张（因编号区间限 12 张）；Agents’ Last Exam、MMMU-Pro、OSWorld、Terminal-Bench-Science 四张已在浏览器截过但未入库，需要时可重截。图中曲线点没有数值标注，具体点位只能目测，不作记录 | 已找到 |
| B1-8 | OpenAI 自报“错误率” | L1 | 发布页 | “On our internal benchmark of error-inducing prompts, the error rate at low reasoning effort fell from 53.6% with GPT-5.6 Terra to 16.7% with GPT-6.1 Terra, a drop of nearly 37 percentage points. At maximum effort, the error rate fell by 18 percentage points to 4.3%, nearly matching the rates for GPT-6 Astra and GPT-6.1 Sol.” 图标题 “Factual error rate on difficult prompts (lower is better)”，纵轴 “Answers with any factual error” | 21 | OpenAI 内部基准，提示词集合与题数未公开；最大档降 18 点，推算 5.6 Terra 最大档约 22.3%（4.3 + 18，推算） | 已找到 |
| B2-1 | AA 标题与价格 | L3 | https://artificialanalysis.ai/articles/gpt-6-1-terra-pushes-the-cost-efficiency-frontier | “OpenAI has released GPT-6.1 Terra, pushing the cost efficiency frontier. It is the first Terra-class model since GPT-5.6, and is more intelligent for less than a third of the cost”；“Pricing per token is half or less that of GPT-5.6 Terra - at $1/$5 per million input/output tokens vs $2/$12 for the previous model. Cache write has also halved, now at $1.25 per million tokens. Cached input is one quarter at $0.05 vs $0.20 per million tokens.” | 13 | 标题的 “less than a third of the cost” 对应的是 max 档的 $0.40 vs $1.40，见 B3 | 已找到 |
| B2-2 | AA Intelligence 要点 | L3 | 同上 | “Pareto-optimal for Intelligence vs Cost at low and medium effort: At max effort, GPT-6.1 Terra gains 6 points in the Artificial Analysis Intelligence Index over GPT-5.6 Terra, at less than a third of the cost per task ($0.40 vs $1.40). At lower effort levels the gain is larger, between 7 and 13 points. The low and medium effort levels are Pareto-optimal for cost efficiency, while the other effort levels sit behind the frontier. At max effort, the model scores 4 points below GPT-6.1 Sol and 10 points above GPT-6 Luna in the Intelligence Index.” | 13 | 指数版本 v4.3.2，含 10 项评测（见 B3）。**最大档不在成本前沿上**（AA 自己写 “the other effort levels sit behind the frontier”） | 已找到 |
| B2-3 | 各档位分数与每任务成本（Terra 6.1 vs 5.6） | L3 | AA 各模型页（`b-aa-m-*.txt`）：https://artificialanalysis.ai/models/gpt-6-1-terra 及 -xhigh/-high/-medium/-low/-non-reasoning；https://artificialanalysis.ai/models/gpt-5-6-terra 及各档 | 页面字段 “Artificial Analysis Intelligence Index”、“Cost per Intelligence Index task”、“Output tokens from Intelligence Index” | 14、19、24 | 下表。5.6 Terra 页面带 “This model is deprecated”提示，5.6 Terra 的 Released July 2026 | 已找到 |
| B2-4 | 与 Sol/Astra/Luna 的差（Intelligence Index，max 档） | L3 | AA 模型页与文章 | Sol 52、Astra 53、Luna 38、Terra 6.1 48（图 14 同值）；“4 points below GPT-6.1 Sol and 10 points above GPT-6 Luna” | 14、19 | Astra 比 Terra 高 5（推算）；Terra 6.1 排名见 B7 | 已找到 |
| B2-5 | 同分更便宜的 Sol 档位（扫描未提） | L3 | AA 模型页 https://artificialanalysis.ai/models/gpt-6-1-sol-medium ；…/gpt-6-1-sol-high | Sol (medium)：48 分，$0.21，输出 15M token；Sol (high)：50 分，$0.32，25M token；Sol (max)：52 分，$0.72，67M token；Terra (max)：48 分，$0.40，130M token | 14（散点图中 Sol (medium) 在 Terra (max) 的左侧同高处） | **AA 自己的数据里，Sol 的 medium 档与 Terra max 档同为 48 分，每任务成本约为一半（$0.21 vs $0.40）；Sol high（50 分）仍比 Terra max 便宜（$0.32）**。AA 文章没有写这一对比，是我对 AA 页面数字的并列（推算）；与“最大档不在前沿”一致 | 已找到 |
| B3 | “$0.40 vs $1.40”的口径 | L3 | AA 文章图 “Intelligence Index vs. Cost per Task”；模型页；方法页 https://artificialanalysis.ai/methodology/intelligence-benchmarking | 图副标题：“Artificial Analysis Intelligence Index · Weighted average cost (USD) per Artificial Analysis Intelligence Index task”；模型页：“On average, it costs $0.40 per task to evaluate GPT-6.1 Terra (Max) on the Intelligence Index.”；方法页：“we use the token counts reported by each model's API provider where available, to accurately report the cost of running the Intelligence Index … we combine these token counts with live measurements of the model's typical cache hit rate” | 14、19 | **max 档；是“跑 AA Intelligence Index v4.3.2（10 项评测）每个任务的加权平均 API 成本”，不是用户真实账单。** 成本 = AA 用提供商报告的 token 数 × 当前单价 × 实时缓存命中率。10 项评测：AA-Briefcase v1.1、GDPval-AA v2.1、AutomationBench-AA、Terminal-Bench 4.0、SciCode、AA-LCR v1.1、AA-Omniscience、Humanity's Last Exam、GDP.pdf、CritPt。成本比 0.40/1.40 = 0.286，即降约 71%（推算）。**同一档位比：xhigh 档 Terra 6.1 为 45 分、$0.19，比 5.6 Terra 的 max 档（42 分、$1.40）便宜约 86%、分数还高 3（推算）**——比值取决于选哪个档位。输出 token：Terra 6.1 max 130M vs 5.6 Terra max 120M（Terra 6.1 反而多用 token，降本来自单价与缓存折扣：AA 页面 Cache Discount 95% vs 90%） | 已找到 |
| B4 | AA-Omniscience 幻觉率（38% 与 88%） | L3 | 文章图 “AA-Omniscience Hallucination Rate”；评测页 https://artificialanalysis.ai/evaluations/omniscience ；方法页 | 图说明：“AA-Omniscience Hallucination Rate (lower is better) measures how often the model answers incorrectly when it should have refused or admitted to not knowing the answer. It is defined as the proportion of incorrect answers out of all non-correct responses, i.e. incorrect / (incorrect + partial answers + not attempted)”；文章：“GPT-6.1 Terra guesses 38% of the time when it does not know the answer to a question in AA-Omniscience, our private knowledge and hallucination benchmark. Previously, GPT-5.6 Terra's hallucination rate was 88%.” | 16 | **分母是“所有没答对的回答”（答错 + 部分答对 + 未作答），不是全部题目。** max 档；6,000 题、42 个主题、6 个领域；评分模型 GPT-5.6 Luna (medium)（方法页写明，是 OpenAI 自家模型）；AA-Omniscience 在 Intelligence Index 中占 15%（准确率 10%、1−幻觉率 5%）。读自图：Terra 6.1 准确率 49%、5.6 Terra 47%（+2，与文章一致）；Omniscience Index：Terra 6.1 为 30、5.6 Terra 为 0；幻觉率：Sol 54%、Astra 51%、Luna 77%、Opus 5.5 (max with fallback) 59%。**推算**：全部题目里答错的比例约 0.38×(1−0.49)≈19%（6.1），约 0.88×(1−0.47)≈47%（5.6）；这与 Index = 准确率 − 答错占比一致（49−19≈30，47−47≈0），所以 “50 point drop” 是“没答对时硬猜的比例”降了 50 点，不是“答错的题”降了 50 点。Terra 6.1 的幻觉率（38%）低于 Sol（54%）、Astra（51%），但准确率（49%）也低于二者（Sol 62%、Astra 63%），Index 30 低于 Sol 42、Astra 43（读自图） | 已找到 |
| B5 | Coding Agent Index v2.0 | L3 | 文章图；方法页 https://artificialanalysis.ai/methodology/coding-agents-benchmarking ；https://artificialanalysis.ai/agents/coding-agents | 文章：“With a score of 48 in the newly released Artificial Analysis Coding Agent Index v2.0, the model is 6 points below GPT-6.1 Sol (54) and 17 points above GPT-6 Luna (31). The xhigh and low effort settings sit on the frontier of cost efficiency in the Coding Agent Index, while other effort levels are behind the frontier.”；图副标题 “Artificial Analysis Coding Agent Index v2.0 incorporates 3 benchmarks: FrontierCode v1.1, Terminal-Bench 4.0, and SWE-Atlas-QnA-AA”；方法页 “the public methodology is a simple average across those benchmark scores” | 15 | 用的是各厂商的编码 harness，不是裸模型：OpenAI 模型在 Codex 里跑（图例 “Codex - GPT-6.1 Terra (max)”）。成本是 “average pay-per-token API cost per task”，方法页另注 “Many users will access coding agent harnesses through subscription plan offerings rather than pay-per-token.” 读自图（max 档、Codex）：Terra 6.1 48，Sol 54，**Astra 55**，Luna 31；Claude Code - Opus 5.5 (max) 62。**WorkBuddy 的 “Astra 为 77” 未找到：AA 图上 Astra 是 55，AA 编码代理页与方法页没有 77；在已存档的 AA 页面内未找到任何 Astra=77 的位置。** Terra max 在该图上不在前沿（前沿是 xhigh 与 low，与文章一致） | 已找到（Astra 77：未找到一手来源） |
| B6 | Terminal-Bench 4.0：OpenAI 自报 vs AA 实测 | L1/L3 | OpenAI 发布页；AA 文章图；AA 方法页 | OpenAI：“At maximum reasoning effort, GPT‑6.1 Terra solves 50.3% of tasks, more than double GPT‑5.6 Terra’s published score of 21.5%.”；AA 图 “Terminal-Bench 4.0: Score … Independently benchmarked by Artificial Analysis”，读自图：Terra 6.1 (max) 55.6%，GPT-5.6 Terra (max) 35.4%，Sol (max) 56.1%，Astra (max) 59.1%，Luna (max) 12.6%；AA 文章：“a gain of ~20 points, now rivaling GPT-6.1 Sol at max effort” | 17、22 | **两处数值不一致，全部记录：** 6.1 Terra：OpenAI 50.3% / AA 55.6%（差 5.3）；5.6 Terra：OpenAI “published score” 21.5% / AA 35.4%（差 13.9）；提升幅度：OpenAI 28.8 点（推算）/ AA 20.2 点（推算，文章写 “~20”）。AA 跑法（方法页）：“We evaluate the full Terminal-Bench 4.0 dataset (66 tasks) using the mini-swe-agent harness, with pass@1 scoring averaged over 3 repeats per task”，最大 agent 步数 500。**OpenAI 发布页与系统卡（deploymentsafety.openai.com/gpt-6-1-terra）均未写 Terminal-Bench 4.0 的 harness、任务数与重复次数（在已存档的这两处内未找到）。** OpenAI 图中 Terra 6.1 最大点约 50%、5.6 Terra 孤点约 21.5%（目测）；图中对照的是 Claude Sonnet 5.5，不是 Opus。AA 图上 Sol max 56.1 与 Terra 55.6 仅差 0.5 | 已找到（OpenAI 跑法：未找到） |
| B7 | 221 个模型中排名 | L3 | https://artificialanalysis.ai/models/gpt-6-1-terra | “#25 / 221” 位于 Intelligence 卡片，分数 48；Speed “#89 / 192”；Cost “#32 / 121”（$0.40）；Verbosity “#90 / 122”（130M） | 19 | **#25/221 中的 221 是 AA 收录的模型条目，同一模型的不同推理档位各算一条**（模型页 223 models in this class；Terra 页：5.6 Terra max 为 #53/221、Sol max #13/221、Astra max #8/221）。同分并列的排名依 AA 页面显示。AA 把 Terra 6.1 记为 “amongst the leading models in intelligence”（median: 28）。WorkBuddy “221 个中排第 25”**属实** | 已找到 |

### B2-3 / B3 附表：Terra 6.1 与 5.6 Terra 各档（AA 模型页，Intelligence Index v4.3.2，每任务成本为 AA 加权平均）

| 档位 | 6.1 分数 | 6.1 成本 | 6.1 输出 token | 5.6 分数 | 5.6 成本 | 5.6 输出 token | 分差（显示值） |
|---|---|---|---|---|---|---|---|
| max | 48 | $0.40 | 130M | 42 | $1.40 | 120M | +6 |
| xhigh | 45 | $0.19 | 41M | 38 | $0.63 | 52M | +7 |
| high | 44 | $0.13 | 23M | 34 | $0.34 | 33M | +10 |
| medium | 41 | $0.10 | 14M | 30 | $0.18 | 17M | +11 |
| low | 36 | $0.05 | 7.9M | 27 | $0.14 | 11M | +9 |
| non-reasoning | 33 | $0.11 | 8.3M | 21 | $0.14 | 5.8M | +12 |

AA 文章写 “between 7 and 13 points”；按页面显示的整数分差，最大为 12（non-reasoning）；差一点大概是四舍五入，未找到 AA 对 “13” 的明细。各档在 AA 图上的前沿标注（文章与图 14）：Terra 6.1 的 low、medium 在 Intelligence–Cost 前沿上，其余档位在前沿后面；在 Coding Agent Index 图上前沿是 xhigh 与 low。表中 AA 各档同行的 Sol 参考：Sol max 52/$0.72/67M，Sol high 50/$0.32/25M，Sol medium 48/$0.21/15M，Sol low 42/$0.13（见 `b-aa-leaderboard.txt`）；Astra max 53/$3.26/60M；Luna max 38/$0.07/140M。AA 模型页“Price”列里 Terra 各档统一显示 $0.7（页面上未给口径，与单价 $1/$5 的混合价不符，不采用，未找到其口径说明）。

## 三、B8 热度

| 帖子 | 分数 / 评论 | 提交时刻 | 记录 |
|---|---|---|---|
| HN “GPT-6.1 Terra”（OpenAI 发布页）https://news.ycombinator.com/item?id=50248124 | 609 分 / 388 评（`barking heat` 取于 2026-10-27 23:17 北京） | 2026-10-26T18:01:17Z（北京 10-27 02:01） | `b-hn-50248124.json`、`b-heat.json` |
| HN “GPT-6.1 Terra (Max) Scores 48 on Artificial Analysis”（AA 模型页）https://news.ycombinator.com/item?id=50249076 | 24 分 / 4 评（HN 搜索 API）；嵌套 2 条 | 2026-10-26T18:54:00Z | `b-hn-50249076.json` |
| HN “GPT 6.1 Terra”（OpenAI 开发者文档）id 50248075 | 37 分 / 4 评 | 2026-10-26T17:58:24Z | 搜索 API |
| Reddit | 搜索被重定向到登录页，未取到（记入 `b-heat.json` errors），未登录、未绕 | — | — |

派工单要求“有理有据的高赞评论”：HN 评论的分数对访客不可见（API 的 points 为 null），所以只能给回复数。以下仅为线索（L6），进正文前须回一手来源核实：

- https://news.ycombinator.com/item?id=50248646（2026-10-26T18:30:49Z，该评论下含 29 条嵌套回复）：“Virtually every task I give to either is better served by going up a tier and down a reasoning level, or down a tier and up a reasoning level… No testing I've done has shown them to have a niche where there isn't a cheaper-with-equal-results or more-performant-at-same-cost option.” —— 与 AA 数据并列时（Sol medium 48 分 $0.21 对 Terra max 48 分 $0.40）方向一致，但那是网友自己的使用体验，AA 文章并无此论。
- https://news.ycombinator.com/item?id=50248477（18:21:29Z，下含 14 条回复）：“The TerminalBench 4.0 cost-per-task results are a bit confusing and I'm not sure why you would pick Terra over Sol in that case.” —— OpenAI 图中 Terminal-Bench 4.0 上 Sol 曲线在 Terra 左上方（目测，见图 22）。
- https://news.ycombinator.com/item?id=50248491（18:22:14Z，下含 193 条回复，楼主抱怨 Luna/Terra/Sol/Astra 命名混乱）：只是命名吐槽，不记作质疑。
- https://news.ycombinator.com/item?id=50248252（18:08:02Z，8 条回复）：“If those charts are true, this effectively matches GPT-6-Astra at a fraction of the cost, and is neck-and-neck with 6.1-Sol.” —— 把 OpenAI 自家图当成立论，没有区分档位与测试。

## 四、可能的吠点（原文里有、线索没提或说反的条件；每条一句）

1. “成本降到三分之一以下” = AA 的 max 档 $0.40 vs 5.6 Terra max $1.40，是 Intelligence Index 全套评测的每任务加权平均 API 成本，不是用户实际账单；AA 自己说 Terra max 档不在成本前沿上，前沿是 low 和 medium（AA 文章 Key takeaways 首条）。
2. 同样在 AA 的数据里，GPT-6.1 Sol 的 medium 档 48 分、$0.21，与 Terra max 档的 48 分、$0.40 同分但约一半成本；Sol high 50 分 $0.32 仍更便宜（AA 各模型页，推算并列，AA 文章未写）。
3. Terra 6.1 max 跑完 Intelligence Index 用了 130M 输出 token，比 5.6 Terra 的 120M 还多、是 Sol max（67M）的约两倍（AA 模型页）；降本来自单价（$1/$5）与缓存折扣（95%），不是用 token 更少。
4. “幻觉率降 50 点”的 38% 是“没答对的回答里答错的比例”，不是全部题目的错误率；按 AA 同图的准确率推算，全部题目中答错约 19%（推算）；且 Terra 的准确率（49%）低于 Sol（62%）、Astra（63%），低幻觉率靠的是更多地放弃回答/部分作答（读自 AA 图 16，AA 方法页给出定义）。
5. AA-Omniscience 的评分模型是 GPT-5.6 Luna (medium)，是 OpenAI 自家模型（AA 方法页）。
6. OpenAI 自报的“错误率”（53.6%→16.7%，最大档降 18 点到 4.3%）与 AA 的“幻觉率降 50 点”是两套指标：OpenAI 是内部的“容易诱发错误的提示词”集合上“Answers with any factual error”，题数与提示词未公开；AA 是私有基准 AA-Omniscience 6,000 题（OpenAI 发布页，AA 方法页）。
7. Terminal-Bench 4.0：OpenAI 自报 50.3%，AA 实测 55.6%；两家对 5.6 Terra 的数字也不同（21.5% vs 35.4%）。AA 的跑法是 mini-swe-agent、66 题、3 次重复、最多 500 步；OpenAI 页面与系统卡未写 harness 等条件（已存档范围内）。所以 “翻一倍多”（OpenAI）与 “~20 点”（AA）是不同口径下的结论。
8. Coding Agent Index v2.0 里 Terra 6.1 为 48，是在各厂商自家 harness（OpenAI 用 Codex）下测的，成本按按量计费 API，AA 注明订阅用户不一定按此计费（AA 方法页）；Astra 在同图是 55，WorkBuddy 的 77 未找到出处。
9. OpenAI 在同一页写 “For the most complex coding tasks, GPT-6.1 Sol and GPT-6 Astra remain the better choice”，AA 排名里 Terra 6.1 max 是 #25/221，Sol max #13、Astra max #8（含各档位）。
10. OpenAI 图的对比对象是 “Claude Sonnet 5.5”（Terminal-Bench 4.0）与 “Claude Opus 5.5”（Agents’ Last Exam、Terminal-Bench-Science 写 “Opus 5.5 w/ fallbacks”）；AA 的 Intelligence Index 上 Opus 5.5 (max with fallback) 为 58，Terra 为 48。
11. OpenAI 发布页写开放范围为 Plus、Pro、Enterprise、Edu 在 “ChatGPT Work and Codex”，没有写 Free、Go、Business（未提，不等于不可用）。

## 五、夸大说法实例（B9）

| 发布方与时间 | 标题/原句（逐字） | URL | 与原文的差别 |
|---|---|---|---|
| unwire.hk，2026-10-27T13:27:34+08:00 | 标题：“OpenAI 推出 GPT-6.1 Terra 單次成本減90% ChatGPT用家可放心用AI” | https://unwire.hk/2026/10/27/openai-gpt-6-1-terra-launch/software/ | 同文正文写 AA 的 “單項任務成本更由 1.4 美元…驟降至 0.4 美元”（约 71%）；标题的 “90%” 在发布页与 AA 文章里没有对应：OpenAI 发布页写的是对 Claude Opus 5.5 “~9x lower cost per task”（另一个对比对象），推测标题数字来自此处（推断，未证实）；标题也没写档位 |
| Interesting Engineering，2026-10-26T20:16:12Z（北京 10-27 04:16） | 标题：“OpenAI's GPT-6.1 Terra matches GPT-6 Astra at 10% of the token cost” | https://interestingengineering.com/ai-robotics/openai-gpt-6-1-terra-50-percent-cheaper | 标题把 “matches” 写成通用，原文限定在 Agents’ Last Exam 的中等推理档，并且 10% 是每 token 单价，不是每任务成本；正文有写 “At medium reasoning effort”。Astra 在 AA 的 Intelligence Index 上 53 分 vs Terra 48 分 |
| 虎嗅，2026-10-27T08:35:29+08:00 | 页面小标题（摘要）：“单任务平均成本从1.40美元降至0.40美元，降超70%” | https://www.huxiu.com/article/4900660.html | 数字与 AA 一致（降 71%），但同段未写这是 max 档、Intelligence Index 全套评测的加权平均；标题写“智能指数不及Claude Opus 5.5”，与 AA 图一致 |
| 爱范儿，2026-10-27T13:00:58+08:00 | 标题：“刚刚，GPT-6.1 Terra 骨折价发布，Agent 时代的打工模型来了”；正文：“新模型的得分比 GPT-5.6 Terra 提高了 15 个百分点，单次任务成本却降低至约三分之一。” | https://www.ifanr.com/1686080 | 15 点与三分之一是 OpenAI 在 Agents’ Last Exam 中等档的自报，正文该句没有写中等档与基准名（前一句写了 Agents’ Last Exam）；不是 AA 数字 |
| The Decoder，2026-10-26T20:30:55Z | “the model resorts to guessing only 38 percent of the time when it lacks the answer, down from 88 percent for GPT-5.6 Terra” | https://the-decoder.com/gpt-6-1-terra-halves-openais-api-prices-and-still-gains-ground-on-its-larger-models/ | 表述忠实（未当成总体错误率）；作对照，不算夸大。同文另写 “that 20-point jump puts the model in the same tier as GPT-6.1 Sol”，AA 原文是 “now rivaling GPT-6.1 Sol at max effort” |

“幻觉率 38% 当总体错误率”“最强编码”类的夸大：在已搜索的 4–5 篇中英文报道里**未找到**；搜索（WebSearch，2 种语言各 1–2 次）未穷尽，不能写“没有”。部分中文媒体写 10 月 27 日发布（如腾讯新闻页 2026-10-27 10:41、虎嗅 10-27 08:35 均为 10-27 当天发稿），OpenAI 页与 AA 写 10 月 26 日；差别出在时区：HN 提交 2026-10-26T18:01Z = 北京 10-27 02:01（推断，未见这些媒体自己写“27 日发布”的原句，未核）。

## 六、扫描说法勘误

| 扫描说法 | 与存档对照 |
|---|---|
| WorkBuddy：AA 文章 “2026-10-25 18:17 UTC” | 未找到；AA 文章日期 October 26, 2026，AA 官方 X 帖 2026-10-26T18:11:02Z |
| WorkBuddy：OpenAI 自报 Terminal-Bench 4.0 50.3%、AA 测 55.6% | 与存档一致 |
| WorkBuddy：OpenAI 写“最复杂编码仍用 Sol/Astra” | 一致：“For the most complex coding tasks, GPT-6.1 Sol and GPT-6 Astra remain the better choice.” |
| WorkBuddy：221 个模型中第 25 | 一致（AA 模型页 #25 / 221；221 含各推理档位条目） |
| WorkBuddy：Astra 的 Coding Agent Index 为 77 | **与存档不符**：AA 文章图（Codex - GPT-6 Astra (max)）为 55；77 未找到出处 |
| ChatGPT：Intelligence Index Max 档 前代 42 → 48，任务平均成本降约 71% | 一致（42→48；$1.40→$0.40，降 71.4% 为推算）；仅限 max 档 |
| ChatGPT：ChatGPT 用户开放范围“有资格差别” | OpenAI 页只列 Plus、Pro、Enterprise、Edu，未提 Free、Go、Business；资格“差别”是对这一列表的解读，页面本身未写不可用 |
| Claude 已知情况：AA 原句价格、前沿、Coding Agent Index 48/54/31、38%/88%、~20 点、GDP.pdf +5 | 全部与存档一致 |
| Claude 已知情况：“AA 说 max 档不在 Intelligence 前沿上” | 一致：文章写 “The low and medium effort levels are Pareto-optimal for cost efficiency, while the other effort levels sit behind the frontier”（Coding Agent Index 的前沿则是 xhigh 与 low） |
| 派工单：AA 方法页 URL | `…/methodology/coding-agent-index` 与 `…/evaluations/terminal-bench-hard` 返回 404；有效页为 `…/methodology/coding-agents-benchmarking`、`…/methodology/intelligence-benchmarking`、`…/evaluations/omniscience` |

## 七、未完成与限制

- OpenAI 发布页 7 张图只截了 2 张入库（Factual error、Terminal-Bench 4.0），另外 Agents’ Last Exam、MMMU-Pro、OSWorld 2.1、Terminal-Bench-Science 0.1 四张已在浏览器截过但未入库（编号区间限制），并且图上没有数值标注。
- Reddit 未取到（需登录）。
- OpenAI 系统卡（deploymentsafety.openai.com/gpt-6-1-terra，已存 `b-openai-syscard.*`）只用于核对 Terminal-Bench harness 说明，其余内容未精读。
- AA Intelligence Index 各子评测的 Terra 6.1 与 5.6 Terra 明细只看了文章图 18，未逐项摘录；GDP.pdf “+5”、Terminal-Bench “~20” 由文章原句覆盖。
- 未做付费或需登录的页面。
