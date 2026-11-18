# 1117 期 B 组取证：微软 MAI Code 2（98% 性能、40% 成本的条件）

取证时刻：北京时间 2026-11-18 16:27–16:40（本机即北京时间）。执行方：Sonnet 5.5（claude-sonnet-5-5）。
文件前缀 `b-`（`b-<name>.html|.pdf|.headers|.txt`）。图片 13–30：13–24 为本组区间，25–30 超出区间（原区间已用尽），由 Claude 合并时重编号（映射见 `capture-log-B.md`）。
图片编号说明与废弃图见 `capture-log-B.md`。

所有“微软口径”数字只摘原文。凡标“本人推算”的算术，不是微软口径，不进“已找到”。

## 1. 证据表（B1–B6）

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

### B1 发布博客

页面 `https://microsoft.ai/news/introducing-mai-code-2-and-mai-code-2-flash/`。存档 `sources/b-blog.html|.headers|.txt`（`.txt` 约 260 KB，含内嵌图表 iframe 的 base64 行，未超 1 MB）。
发布时刻：页内可见 “Nov 17, 2026”；JSON-LD `datePublished` = 2026-11-17T18:15:16+00:00，即北京 2026-11-18 02:15:16。`dateModified` = 2026-11-17T22:55:27+00:00（北京 11-18 06:55:27）。

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 小标题 “less than half the price” | L1 | 博客 | “Production quality at less than half the price” | 13-blog-headline-98-40.png | 小标题写 price，下一段正文写 cost | 已找到 |
| 尚未针对更难任务调优 | L1 | 博客 | “They’re explicitly not yet tuned for harder problems, like fundamental science research and very long horizon agentic work.” | 13 | 见 B 节吠点 9：模型卡另有“推荐用于最复杂的长程任务” | 已找到 |
| 98%/40%、96%/5% | L1 | 博客 | “Averaged over four widely used benchmarks that target the everyday enterprise coding tasks we see in GitHub Copilot (DeepSWE 1.1, Terminal Bench 2.1, SWE-Atlas and FrontierCode v1.1), MAI Code 2 gives you 98% of the performance of GPT 6.1 Sol at 40% of the cost, and 96% of Claude Opus 5.5’s performance at just 5% of the cost.” | 13、15、16 | 四项基准平均；对比对象 GPT 6.1 Sol、Opus 5.5；脚注 1 的口径见下行 | 已找到 |
| 脚注 1（成本口径） | L1 | 博客 | “MAI Code 2 performance and price: All models were run at max reasoning effort. DeepSWE 1.1, TerminalBench 2.1 and SWE Atlas were run on MSBench (Microsoft’s internal eval platform) with the GitHub Copilot CLI without subagents. FrontierCode 1.1 scores and costs are from Artificial Analysis; MAI-Code-2 cost is shown at its 50% promotional price.” | 20-blog-footnotes.png | 3 项在微软自家平台 MSBench 跑；FrontierCode 1.1 的分数与成本来自 AA；MAI Code 2 成本按 50% 促销价 | 已找到 |
| 脚注总述 | L1 | 博客 | “Results reflect evaluations on specific coding tasks and configurations. Performance and cost comparisons vary by workload, model configuration, reasoning effort, and pricing assumptions.” | 20 | 页脚小标题 “About these results” | 已找到 |
| 盲测偏好 | L1 | 博客 | 现版：“In blind frontend app evaluations, reviewers preferred its results over GPT-6 Sol 64% of the time.” 脚注 2：“Visual coding: Blind human side-by-side evaluations of MAI Code 2 and GPT-6 Sol across user interfaces, games, data visualizations, 3D scenes, and SVG graphics. The preference covers both aesthetics and functionality, and is annotated by a total of 258 annotators.” | 13、20、25（旧版 67%） | 对比对象是 GPT-6 Sol（不是 6.1 Sol）；存档快照曾写 67%，见 B4 | 已找到 |
| Flash 对比 | L1 | 博客 | “In addition, MAI Code 2 Flash offers comparable performance to GPT-6 Luna and Claude Haiku 5.5 at 65% and 20% of the cost, across three coding benchmarks measuring day-to-day, high-volume routine coding tasks.” 脚注 3：“MAI Code 2 Flash performance and price: All models were run at max reasoning effort. DeepSWE 1.1, TerminalBench 2.1 and SWE Atlas were run on MSBench with the GitHub Copilot CLI without subagents. MAI-Code-2-Flash cost is shown at its 50% promotional price.” | 13、17/18、20 | 三项基准（无 FrontierCode）；全部在 MSBench | 已找到 |
| 可用范围 | L1 | 博客 | “MAI Code 2 and Code 2 Flash are available in GitHub Copilot, and Code 2 Flash is in limited-access preview in Microsoft Foundry. Soon, we’ll be releasing it on Open Router, Vercel, Hermes, Open Code, and Kilo.” | 19-blog-availability.png | “Soon” 无日期 | 已找到 |
| 页面是否给出绝对价、参数、上下文 | — | 博客 | 页面无每百万 token 价、无参数量、无上下文窗口（全文检索无 “$ per 1M”） | — | 状态含义：页面确未写；参数与上下文见 B2 的模型卡 | 已找到 |

对比图表（取自页面内嵌 iframe 的脚本数据，与渲染截图逐项一致）：

MAI Code 2 / GPT-6.1 Sol / Claude Opus 5.5（图 15、16；“all models at max reasoning effort”）：

| 基准 | 通过率 %（MAI / Sol / Opus） | 每任务成本 $（MAI / Sol / Opus） |
|---|---|---|
| 平均（四项） | 63.2 / 64.1 / 65.3 | 0.309 / 0.761 / 6.306 |
| DeepSWE 1.1 | 73.2 / 74.6 / 69.0 | 0.618 / 1.212 / 10.717 |
| Terminal-Bench 2.1 | 84.6 / 85.0 / 78.7 | 0.081 / 0.265 / 1.842 |
| SWE Atlas | 43.1 / 47.4 / 57.5 | 0.228 / 0.605 / 4.234 |
| FrontierCode v1.1 | 52.0 / 49.4 / 56.0 | 0.310 / 0.960 / 8.430 |

渲染后的图表可见部分（截图 15、16）不含 “promotional”；页脚脚注 1 写了促销价。图表脚本数据里的 caption 含 “MAI Code 2 cost is at the 50% promotional price.”，该 caption 被写入 `.mx-sr`（屏幕阅读器专用）表格的 `<caption>`，可见图面不显示；视觉读者只在页脚脚注看到。

MAI Code 2 Flash / GPT-6 Luna / Claude Haiku 5.5（图 17/18；三项基准）：

| 基准 | 通过率 %（Flash / Luna / Haiku） | 每任务成本 $（Flash / Luna / Haiku） |
|---|---|---|
| 平均（三项） | 61.0 / 55.1 / 53.9 | 0.052 / 0.080 / 0.319 |
| DeepSWE 1.1 | 59.9 / 59.6 / 46.0 | 0.098 / 0.159 / 0.668 |
| Terminal-Bench 2.1 | 78.3 / 69.3 / 73.4 | 0.023 / 0.028 / 0.067 |
| SWE Atlas | 44.8 / 36.4 / 42.1 | 0.034 / 0.051 / 0.223 |

模型卡（`b-modelcard.pdf|.txt`，页首 “Last Updated: 17 November 2026”）§3.1 的基准方法：“All coding benchmarks were run on MSBench, Microsoft’s agentic evaluation platform, using the GitHub Copilot CLI (version 1.0.102) as the harness with subagents disabled. Every model in a comparison was run in the same harness with the same settings … Unless noted, results use max reasoning effort and are averaged over three independent runs per task (mean single-attempt pass rate, not pass@k); infrastructure errors are counted as failures.” 模型卡表里 MAI Code 2 / Flash 的三项分数与博客图一致（DeepSWE 73.2 / 59.9；Terminal-Bench 84.6 / 78.3；SWE Atlas 43.1 / 44.8）。

### B2 绝对价格

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| MAI Code 2 有标价与促销期限 | L1 | GitHub Docs “Models and pricing” `https://docs.github.com/en/copilot/reference/copilot-billing/models-and-pricing`（存档 `b-copilotpricing.*`，取于北京 11-18 16:28） | 脚注 4：“MAI Code 2 is available at the promotional pricing of $0.50 per 1M input tokens, $0.025 per 1M cached input tokens, and $2.50 per 1M output tokens until May 17, 2027. Starting May 17, 2027, pricing is $1.00 per 1M input tokens, $0.05 per 1M cached input tokens, and $5.00 per 1M output tokens.” | 22-copilot-price-mai.png、23-copilot-price-footnotes.png | 价格单位 “All prices are per 1 million tokens.”；促销价至 2027-05-17；Docs 脚注列出 2027-05-17 起的价格（为促销价的 2 倍） | 已找到 |
| MAI Code 2 Flash 标价 | L1 | 同上 | 脚注 5：“MAI Code 2 Flash is available at the promotional pricing of $0.05 per 1M input tokens, $0.005 per 1M cached input tokens, and $0.25 per 1M output tokens until May 17, 2027. Starting May 17, 2027, pricing is $0.10 per 1M input tokens, $0.01 per 1M cached input tokens, and $0.50 per 1M output tokens.” | 22、23 | 同上 | 已找到 |
| 对照模型的同表价格 | L1 | 同上 | 表内：GPT-6.1 Sol（Default，≤ 272K）Input $2.00、Cached input $0.10、Cache write $2.50、Output $10.00；GPT-6.1 Sol（Long context，> 272K）$4.00 / $0.20 / $5.00 / $15.00；GPT-6.2 Sol（≤ 272K）$2.00 / $0.10 / $2.50 / $10.00；GPT-6 Sol（≤ 272K）$2.00 / $0.20 / $2.50 / $10.00；Claude Opus 5.5 $4.00 / $0.20 / $5.00 / $20.00；Claude Haiku 5.5（≤ 100K）$0.10 / $0.01 / $0.125 / $0.50；GPT-6 Luna（≤ 272K）$0.10 / $0.01 / $0.125 / $0.50；MAI-Code-1.1-Flash $0.20 / $0.02 / $1.20 | 24-copilot-price-sol-opus.png（含 GPT-6.1 Sol、GPT-6.2 Sol、Opus 5.5 行与列头；GPT-6 Sol、Luna、Haiku 行见 `b-copilotpricing.txt`，未截） | 这是 Copilot 的价格表，不是微软博客或 MSBench 使用的价目；博客未说明其成本口径用的是哪张价目表（未写） | 已找到 |
| MAI Code 2 在 GitHub Copilot 的可用范围与计费 | L1 | GitHub Changelog `https://github.blog/changelog/2026-11-17-mai-code-2-and-mai-code-2-flash-in-github-copilot`（存档 `b-ghmai.*`；页内 “November 17, 2026”，JSON-LD 2026-11-17T18:01:45+00:00 = 北京 11-18 02:01:45） | “MAI Code 2: Copilot Pro, Pro+, Max, Business, and Enterprise”；“MAI Code 2 Flash: Copilot Free, Student, Pro, Pro+, Max, Business, and Enterprise”；“Both models are available to eligible GitHub Copilot subscribers on annual plans.”（该句的适用范围不清：同一批存档里 “annual plan” 另指旧的 premium request 计费群体，见 `b-copilotmult.txt` 第 1214 行 “existing annual plan who remained on legacy premium request-based billing”，以及 `b-copilotmodels.txt` Opus 4.8 脚注 “available to individual Copilot subscribers on annual plans … not available to subscribers on monthly plans”；本文不解释，只引原句）；“These models are billed under usage-based billing.”；“Copilot Enterprise and Copilot Business plan administrators can manage access to MAI models through the model policy in Copilot settings. Under default model enablement, new models are enabled automatically unless an administrator has turned off the global default or explicitly disables the model.” | 29-ghchangelog-availability.png（深色底页面） | 可选界面含 VS Code、Visual Studio、Copilot CLI、cloud agent、Copilot app、github.com、GitHub Mobile、JetBrains、Xcode、Eclipse（全文见存档） | 已找到 |
| Copilot 支持模型表 | L1 | `https://docs.github.com/en/copilot/reference/ai-models/supported-models`（存档 `b-copilotmodels.*`） | MAI Code 2、MAI Code 2 Flash 均列为 “Microsoft / GA”；脚注 3：“MAI models are continuously improving models. Performance and behavior may evolve over time as new checkpoints are released.” 另有 “Copilot Student and Copilot Free users have access to models through auto model selection only.” | 无 | 各套餐的勾选是图标，文本里读不出每行每列。另：`https://github.com/features/copilot/plans`（存档 `b-copilotplans.*`）的 “Free plan” 模型清单文字里出现 “Microsoft MAI Code 2”，而 Changelog 的 MAI Code 2 套餐清单不含 Free；两处不一致，未解决，不取舍 | 已找到 |
| Foundry 价格页有无 MAI Code 2 | L1 | `https://azure.microsoft.com/en-us/pricing/details/ai-foundry/`（存档 `b-foundry.*`） | 页面文本无 “MAI”（`grep -w MAI` 0 命中） | — | 查询时刻北京 11-18 16:28；页面可能由脚本动态加载，静态存档读不到，未做渲染核对 | 未找到一手来源 |
| microsoft.ai 模型页有无标价 | L1 | `https://microsoft.ai/models/mai-code-2`（存档 `b-modelpage.*`） | 状态含义：页面确未写标价。页面无价格数字；只有 “of Opus 5.5 performance at 5% of cost.”（计数动画，目标值 96）、“Of users voted for MAI Code 2's front-end UI, over GPT-6 Sol's.”（目标值 64）、“1/5 Of cost to Claude Haiku 5.5 and 12% higher performance.” | 无 | 页面 JSON-LD `dateModified` = 2026-11-18T00:32:10+00:00（北京 08:32:10） | 已找到 |

模型卡（`b-modelcard.*`）的规格：“Context length 1,050,000 tokens”；“Both models support text and image inputs with a 922,000 input tokens limit”；“Both models output text with a 128,000 output tokens limit”。参数量在同一份模型卡内两处写法不同（active 与 total 口径不同，未必矛盾）：摘要表 “MAI Code 2: 50B-100B (80B active)”“MAI Code 2 Flash: 5B-15B (12B active)”；§1 正文 “MAI Code 2 has 77 billion active, 3 billion embedded, and approximately 2.8 trillion total parameters.”“MAI Code 2 Flash has 12 billion active and approximately 398 billion total parameters.”。两处都记录，不取舍。博客与 Ignite 页都不写参数量。

### B3 对比基准（Artificial Analysis）

AA 页面 `https://artificialanalysis.ai/evaluations/frontiercode`：静态存档 `b-aa-frontiercode.*`（北京 11-18 16:33）；另用 `barking shot render` 渲染（北京约 16:44，700 CSS px 宽，图表默认 “27 of 83 models”，横向可滚动），截图 30。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| AA 页上有 MAI Code 2 的 FrontierCode 1.1 分数与成本 | L3 | 同上；猜测的 `…/models/mai-code-2`、`…/models/mai-code-2-0`、`…/models/creators/microsoft-ai` 均 404 | 渲染图（默认 27 of 83 个模型）与文字摘要里没有 MAI Code 2；没有展开其余 56 个模型，也没有取到图表数据表，故不能断言 AA 全站没有 | 30-aa-frontiercode11-score.png | 图表为脚本渲染；全部 83 个模型的数值未取到（只展示了默认 27 个，截图右侧被横向滚动截断） | 未找到一手来源 |
| AA 的 FrontierCode 1.1 评测口径 | L3 | 同上 | “We run FrontierCode 1.1 Main, the primary 100-task set of the benchmark's 150 tasks.”；“We run every task three times with our Stirrup agent harness, capped at 250 turns”；图题 “Mean score over three attempts per task (attempts that fail a blocking check or are flagged for looking up the solution score zero) · Independently benchmarked by Artificial Analysis”；页顶标签含 “Legacy”；“Cognition has released FrontierCode 2.0. This page shows our results on FrontierCode 1.1 Main, the 100-task set. Our results on FrontierCode-AA 2.0, part of the Artificial Analysis Intelligence Index, are on the FrontierCode-AA 2.0 page.” | 30 | AA 已把 FrontierCode-AA 2.0 放进指数；1.1 页被标为 Legacy | 已找到 |
| AA 上 Opus 5.5、GPT-6.1 Sol 的 FrontierCode 1.1 数值 vs 微软图 | L3 | 同上 | 渲染图读数（柱上标注，本人从截图读出）：Claude Opus 5.5 (max) 52.2%；GPT-6.1 Sol (max) 49.1%；Claude Fable 5.2 (max) 56.2%。文字摘要另写 “Claude Opus 5.5 (Medium, Default Fallback) with a score of 57.1%”。微软博客图：Opus 5.5 56.0%、GPT-6.1 Sol 49.4%、MAI Code 2 52.0%（脚注 “FrontierCode 1.1 scores and costs are from Artificial Analysis”） | 30 | 微软 Opus 5.5 的 56.0 与 AA 当前图上 Opus 5.5 (max) 的 52.2 相差 3.8 个百分点；Sol 49.4 对 49.1 相差 0.3。原因未核（可能是数据更新、变体命名或取数时间），只记两边数字 | 部分支持 |
| AA 对 GPT-6.1 Sol 的状态 | L3 | `https://artificialanalysis.ai/models/gpt-6-1-sol`（存档 `b-aa-sol.*`） | “This model is deprecated. We still benchmark its performance, but less often: every 18 hours instead of every 6.”；“OpenAI has launched a newer release, GPT-6.2 Sol. We suggest considering it instead.”；价格 “In $2.00 Out $10.00”；“Artificial Analysis Intelligence Index … 55” | — | AA 页内嵌数据另有 GPT-6.1 Sol (Max) intelligenceIndex 54.6、Claude Opus 5.5 (Max, Default Fallback) intelligenceIndex 59.1（来自 `b-aa-frontiercode.html` 脚本数据，未截图） | 已找到 |

### B4 Ignite 总览与偏好率（64 / 67 / 68）

Ignite 总览 `https://microsoft.ai/news/microsoft-ignite-2026/`（存档 `b-ignite.*`）：页内 “Nov 17, 2026 - Announcements - Ignite”，标题 “Seven new world class models at scalable price-performance”。`datePublished` = 2026-11-17T18:16:05+00:00（北京 11-18 02:16:05），`dateModified` = 2026-11-18T02:35:41+00:00（北京 10:35:41）。

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Ignite 页的 98%/40% 写法 | L1 | Ignite 总览 | 阵容列表：“MAI Code 2 reaches 98% of the coding quality of GPT-6.1 Sol at about 40% of the price. And against Opus 5.5 it reaches 96% performance for just 5% of the price.”“MAI Code 2 Flash matches of the performance of GPT-6 Luna at 65% of the price, and Haiku 5.5 at 20% of the price.” | 28-ignite-lineup-clean.png（26 为含半截下一条的旧裁切） | 这里用 “price”“coding quality”，与博客的 “cost”“performance” 不同；全页检索无 “promotional”“50%”“MSBench”，无脚注（核对：HTML 文本 0 命中；唯一内嵌 iframe 解码后 0 命中；渲染图 OCR 行 0 命中） | 已找到 |
| Ignite 页正文 | L1 | 同上 | “MAI Code 2 gives you 98% of the performance of GPT-6.1 Sol at 40% of the price, and 96% of Claude Opus 5.5’s performance at just 5% of the price.”；“They’re explicitly not yet tuned for harder problems, like fundamental science research and very long horizon agentic work.”；“In a blind, side-by-side evaluation of frontend work, developers preferred MAI Code 2 over GPT-6 Sol 64% of the time.” | 27-ignite-code-section.png | 对比对象仍是 GPT-6.1 Sol；盲测是 GPT-6 Sol | 已找到 |
| 盲测偏好率在各页的写法 | L1 | 见右 | 博客现版 64%（“reviewers”）；Ignite 页现版 64%（“developers”）；模型页现版目标值 64（“Of users voted”）。存档快照：博客 Wayback 副本（快照标识 20261118003654，UTC，即北京 11-18 08:36:54；该副本自带 `dateModified` 2026-11-17T18:15:17+00:00）显示 “reviewers preferred its results over GPT-6 Sol 67% of the time.”；模型页 Wayback 快照 20261117184519（UTC；北京 11-18 02:45:19）的计数目标值为 68（“Of users voted for MAI Code 2's front-end UI, over GPT-6 Sol's.”）；Ignite 页无可用快照（Wayback 返回 “has not archived that URL”） | 25-blog-wayback-67pct.png（Wayback 快照渲染，无 Wayback 工具栏） | 快照只用于看“以前写了什么”。现版与存档副本数值不同；存档副本可能是缓存，只能说“该时刻抓取的副本显示 67%”，改动时刻无法从页面确定（现版博客 `dateModified` 22:55:27Z 早于该副本的抓取时刻，不能当作改数字的时刻）。第三方：tbreak（北京 11-18 12:17 发布）写 67%；cnmo（北京 07:01）、80aj（08:33）、cnblogs（07:36）、17173（07:33）写 64% | 部分支持 |

三个页面、三个时间点分别出现 67 / 68 / 64；当前以 64 为准。

核对：WorkBuddy 扫描写的 67% 对应博客旧版，Ignite 总览页“据称 68%”对应模型页旧版（Ignite 页本身的旧值无存档，不能确认）。当前三页一致为 64%。

### B5 二手稿与夸大实例

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 有中英文二手稿把“成本”写成价格或不提促销价 | L5 | 见第 3 节表（17173、80aj、新浪、CNMO、tbreak） | 原句见第 3 节 | 无（二手稿只作线索，未截图） | 二手稿只存文本；Reuters 401、FourWeekMBA 与 Techzine 403 未读到 | 部分支持 |

### B6 热度

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| HN `50535885` | — | `https://news.ycombinator.com/item?id=50535885` | “MAI Code 2 and Code 2 Flash”（链接指向微软博客） | — | `barking heat`（Algolia 数据）取于北京 11-18 16:32:54：350 分 / 153 评；提交时间北京 11-18 02:25:13（UTC 18:25:13）。存档 `b-hn-heat.json`、`b-hn-hn-50535885.json`（评论树）。GPT-6.2 帖（HN `50535230`）经 `b-hn62-heat.json` 复核：提交 2026-11-17 17:46:43 UTC，比 MAI 帖（18:25:13 UTC）早约 38.5 分钟（取到时 1251 分） | 已找到 |
| Reddit | — | — | `barking heat` 的 Reddit 搜索被 old.reddit 登录页拦截（响应非 JSON）；另用 curl 访问 www.reddit.com 的 search.json 返回 “Blocked” 页 | — | 两次失败即停，记入 `b-heat.json` 的 errors；没有借助登录态 | 未找到一手来源 |

## 2. 可能的吠点（说法 vs 证据）

均为线索，除标明 L1/L3 外，社区评论只作 L6。

1. “40% of the cost”：成本按促销价。说法：博客 “98% of the performance of GPT 6.1 Sol at 40% of the cost”（Ignite 页与 Suleyman 的 X 长文写 “at 40% of the price”，见吠点 11）。证据：脚注 1 “MAI-Code-2 cost is shown at its 50% promotional price”；Copilot 价格页脚注 4：促销价 $0.50 / $2.50（每百万 token 输入/输出）至 2027-05-17，之后 $1.00 / $5.00。博客图的每任务平均成本：MAI Code 2 $0.309，GPT-6.1 Sol $0.761。图里不显示 “promotional”，只有页脚脚注有。本人推算（非微软口径）：若把 MAI 成本按字面“五折”还原为两倍，$0.618 / $0.761 ≈ 81%，$0.618 / $6.306 ≈ 9.8%；该推算假设成本与单价成比例，且微软使用的价目与 Copilot 价目一致，两点均未核实。第三方（FourWeekMBA 标题“Costs 40% of Sol at Its Promo Price”、搜索摘要）也做过同样的推算，仅作线索。
2. “98%” 是平均数之比，绝对差很小。说法：“98% of the performance of GPT 6.1 Sol”。证据：平均通过率 63.2% vs 64.1%（差 0.9 个百分点）；分基准 DeepSWE 73.2 vs 74.6、Terminal-Bench 84.6 vs 85.0、SWE Atlas 43.1 vs 47.4 三项低于 Sol，仅 FrontierCode 52.0 vs 49.4 高于 Sol。相对 Opus 5.5：平均 63.2 vs 65.3，DeepSWE 与 Terminal-Bench 高于 Opus，SWE Atlas 43.1 vs 57.5、FrontierCode 52.0 vs 56.0 低于 Opus。
3. 三项基准在微软自家平台跑，且对比模型都是微软自己跑的。证据：脚注 1 “run on MSBench (Microsoft’s internal eval platform) with the GitHub Copilot CLI without subagents”；模型卡 “Every model in a comparison was run in the same harness with the same settings”“averaged over three independent runs per task”“infrastructure errors are counted as failures”。第四项 FrontierCode 1.1 来自 AA；但我没能在 AA 公开页面上找到 MAI Code 2 的数据（B3），无法核对。
4. 对比对象不是同日发布的最新模型。证据：博客对比 GPT 6.1 Sol；AA 页写 “This model is deprecated … OpenAI has launched a newer release, GPT-6.2 Sol”；Copilot 价格页同日列出 GPT-6.2 Sol，价格与 GPT-6.1 Sol 相同（$2.00 / $0.10 / $10.00）。盲测对比 GPT-6 Sol，不是 6.1 Sol。只记时间事实：GPT-6.2 的 HN 帖比 MAI Code 2 的帖早约 39 分钟（17:46 UTC 对 18:25 UTC），不作指控。HN 评论（oofbaroomf，无得分字段）也指出对比用的是 6.1 Sol 的 API 价。
5. FrontierCode 1.1 在 AA 已是旧版。说法：以 FrontierCode v1.1 为四项之一。证据：AA 页顶标签含 “Legacy”，页内 “Cognition has released FrontierCode 2.0”，AA 指数使用 FrontierCode-AA 2.0。HN 评论 johnfn 也说 “DeepSWE 1.1 has been busted for a while … TB2.1 … currently at 4 … FrontierCode at 1.1 when it's currently at 2.0”（L6，未核）。
6. 偏好率改过且口径不同。说法：64%（现版）。证据：博客旧版 67%、模型页旧版 68%（存档，见 B4）；样本 “a total of 258 annotators”；对象 GPT-6 Sol；Ignite 写 “developers”，博客写 “reviewers”，模型页写 “users”。
7. Flash 的 “20% of the cost” 与图上数字的比值并列。说法：“at 65% and 20% of the cost”。证据：图平均成本 Flash $0.052、Luna $0.080、Haiku $0.319。本人推算：0.052/0.080 = 65.0%，0.052/0.319 = 16.3%（微软写 20%；并列两个数字，不判断原因）。微软 Ignite 页与博客都写 20%。同样本人推算：Opus 5.5 一项 0.309/6.306 = 4.9%，吻合 5%；0.309/0.761 = 40.6%，吻合 40%；63.2/64.1 = 98.6%、63.2/65.3 = 96.8%（微软写 98%、96%）。
8. “Word” 一条的限定：正文 “Our model performs similar to GPT 6.1 sol (while being significantly smaller) on 5 key internal benchmarks using ~42% fewer input tokens per call than GPT6.1 Sol in production.”，而对应图的说明文字是 “Performance on 5 of 9 key internal benchmarks (success rate or score, higher is better)”：只展示 9 项里的 5 项。属 “Beyond coding” 的 MAI Work Flash，不是 MAI Code 2，仅作旁证。
9. “not yet tuned for … very long horizon” 与模型卡措辞并列。博客：“They’re explicitly not yet tuned for harder problems, like fundamental science research and very long horizon agentic work.” 模型卡摘要：“MAI Code 2 is the larger frontier model, built for complex, long-horizon software engineering in real repositories.”；§1：“MAI Code 2 is recommended for the most complex, long-horizon tasks”。两者可能指不同层级的“长程”，但并列写下读者会看到相反的话（只记原句，不下结论）。
10. 可用范围的限制。证据：Changelog “MAI Code 2: Copilot Pro, Pro+, Max, Business, and Enterprise”（无 Free、Student）；“available to eligible GitHub Copilot subscribers on annual plans”；Foundry 仅 Flash 且 “limited-access preview”；Foundry 价格页未见 MAI。HN 评论 throwaw12：“there is no Pricing page on that website, neither way to create API keys, nor playground”（L6；微软站点确无价格，但价在 GitHub Docs，见 B2）。
11. Ignite 页与 Suleyman 的 X 长文把 “cost” 写成 “price”，且没有促销价说明。Ignite 页（见 B4）；Suleyman（Microsoft AI CEO，公开账号，L2）X 长文（存档 `b-suleyman-fx.json`，fxtwitter 取得，帖子时间 2026-11-17 18:28:43 UTC = 北京 11-18 02:28:43，帖页 289 赞 / 27 转 / 14 评，取于 16:3x）原句：“MAI Code 2 gets you 98% of the performance of GPT-6.1 Sol on everyday coding tasks to help with patching and remediation, at just 40% of its price.”；“MAI Code 2 gives you 98% of GPT-6.1 Sol's performance at 40% of the cost, and 96% of Claude Opus 5.5's at just 5% of the cost.”。文中没有 “promotional”（全文检索 0 命中）。该帖是 X 长文，帖本身的文本只有长文链接 `https://x.com/i/article/2122712682268868608`；没有截图（遵守平台名规则，不对 X 页面截图）。
12. 模型卡规格两处写法不同（见 B2）：摘要 “MAI Code 2 Flash: 5B-15B (12B active)”，§1 “approximately 398 billion total parameters”。active 与 total 口径不同，未必矛盾；不影响主线，正文不建议写参数量。
13. 社区质疑（L6，评论无得分字段；HN 的 Algolia 数据不含评论分，只有主帖 350 分 / 153 评）：
   - 促销价与 max effort：user43928 “While showing MAI Code 2 at a 50% promotional discount, and comparing to Max reasoning effort for Opus 5.5 and 6.1 Sol…” `https://news.ycombinator.com/item?id=50536992`（北京 11-18 03:38）。
   - 促销期：tomatoandbasil “The promotional period lasts until May 2027.” `https://news.ycombinator.com/item?id=50537652`。
   - 对比对象：oofbaroomf “They are comparing to Sol 6.1's API prices. With Sol 6.2 … it's probably no longer on the Pareto frontier.” `https://news.ycombinator.com/item?id=50537302`（Copilot 表里 6.2 Sol 与 6.1 Sol 同价，已核；“subsidization from Codex subscriptions” 部分未核）。
   - 基准版本：johnfn `https://news.ycombinator.com/item?id=50538426`（见吠点 5）。
   - 实测：chopete3 “Github Copilot offered MAI Code 2 at 50% today. I switched it to that model and it was able to resolve tricky python build issues … Didn't feel the difference from Claude Opus.” `https://news.ycombinator.com/item?id=50541360`（单人体验，正面，L6）。

## 3. 夸大说法实例（把事件写歪的标题或文章）

只列“成本口径”被丢掉或写成价格的实例；原句逐字，时刻为北京时间。

| 来源 | URL | 原句 | 时刻 | 说明 |
|---|---|---|---|---|
| 17173（中文） | `https://news.17173.com/content/11182026/073324722.shtml` | 标题：“微软性价比最高编程 AI 模型：MAI Code 2 用 5% 成本达 Claude Opus 5.5 约 96% 性能”；正文：“微软表示，在其公布的四项编程基准平均结果中，Code 2 达到 GPT-6.1 Sol 约 98% 的表现，价格约为其 40%。”（只对 Flash 提到 “在享受 50% 促销定价下”，对 Code 2 的 5% 与 40% 不提促销） | 2026-11-18 07:33（页内 2026-11-18T07:33:24） | 标题把“成本”写成事实；“价格约为其 40%”把测试成本说成价格 |
| 80aj（中文） | `https://www.80aj.com/2026/11/18/mai-code-model-performance/` | 标题：“微软发布MAI Code 2编程模型：5%成本达到Claude Opus 5.5约96%性能”；评论段：“若5%成本达到Claude Opus 5.5约96%性能的数据经第三方独立验证成立，将对编程模型市场价格体系形成直接冲击” | 2026-11-18 08:33（JSON-LD） | 全文无“促销/五折/自家平台”字样（检索 0 命中）；该文提到需第三方验证，算半个限定 |
| 新浪（智东西转载） | `https://k.sina.cn/article_5953740931_162dee083067045xfm.html` | “其编程模型MAI Code 2在四项编程基准测试中，综合成绩达到GPT-6.1 Sol约98%的水平，测试成本约为其40%。” | 2026-11-18 14:34（页内） | 对 Flash 写了“按促销价计算”，对 Code 2 的 40% 没写 |
| CNMO（中文） | `https://ai.cnmo.com/news/822708.html` | “MAI Code 2达到GPT-6.1 Sol约98%的质量、价格约为其40%”；后一段写“部分成本数据按五折促销价计算” | 2026-11-18 07:01（页内） | 有限定，但“价格约为其40%”仍把成本写成价格；可作“有写限定”的对照 |
| tbreak（英文） | `https://tbreak.com/microsoft-mai-code-2-enterprise-coding-models/` | 标题 “Microsoft’s MAI Code 2 claims 98% of GPT-6.1 Sol’s performance at 40% of the cost”；正文 “the MAI Code 2 cost figures reflect a 50% promotional price”；“reviewers preferred MAI Code 2’s output to GPT-6 Sol’s 67% of the time” | 2026-11-18 08:17 UTC+4 = 北京 12:17 | 标题不带限定，正文带；盲测写的仍是旧版 67% |
| 对照：微软自家页 | Ignite 总览（B4）；Suleyman X 长文（吠点 11） | “at about 40% of the price”；“at just 40% of its price” | 2026-11-18 02:16 / 02:28 | 微软自己的总览页没有促销价说明 |

Reuters（L4）：`https://www.reuters.com/legal/litigation/microsoft-unveils-ai-models-coding-cybersecurity-it-takes-anthropic-openai-2026-11-17/` 返回 401（`b-reuters.html` 仅 774 字节的拦截页），付费墙/校验不绕过，没有读到原句，L4 引用不可用。
FourWeekMBA（`https://fourweekmba.com/ai-microsofts-mai-code-2-costs-40-of-sol-at-its-promo-price/`）与 Techzine 抓取返回 403，没有读到原文，只看到搜索摘要（不作证据）。未找到英文“把 40% 说成价格平替且完全不提促销价”的单篇原文；上表英文实例只有标题级。

## 4. 扫描说法勘误

| 扫描说法 | 核对结果 |
|---|---|
| 盲测偏好 67%（WorkBuddy） | 博客 Wayback 快照（北京 11-18 08:36:54）写 67%，与扫描一致；现版博客、Ignite 页、模型页均为 64%。模型页旧快照（北京 02:45:19）写 68%（对应“Ignite 总览页据称写 68%”的 68，但 Ignite 页本身没有旧存档，不能确认）。 |
| Flash 数据卡 12B（扫描称） | 模型卡写 “MAI Code 2 Flash: 5B-15B (12B active)” 与 “12 billion active and approximately 398 billion total parameters”，见 B2；“12B” 是模型卡里的 active 参数；同卡写 total 约 398B。 |
| “去掉折扣 40%→80%” | 属扫描/第三方自己的算术（本文“本人推算”81%，与之相近），不是微软口径，不进“已找到”。 |
| “页面未给每百万 token 绝对价” | 微软博客与模型页确实无价；价格在 GitHub Docs：MAI Code 2 促销 $0.50 / $2.50，促销至 2027-05-17，之后 $1.00 / $5.00（B2）。 |
| “没跟 GPT-6.2 比” | 属实：博客对比 GPT-6.1 Sol、GPT-6 Sol、GPT-6 Luna、Opus 5.5、Haiku 5.5，没有 GPT-6.2。HN 时间：GPT-6.2 帖 17:46 UTC，MAI Code 2 帖 18:25 UTC（只记时间事实）。 |
| 已知情况 “原站发布日期须核” | 博客 `datePublished` 2026-11-17T18:15:16+00:00（北京 11-18 02:15:16）；Ignite 总览 2026-11-17T18:16:05+00:00（北京 02:16:05）。 |
| 已知情况 “HN 349 分 / 152 评” | 北京 16:32:54 取为 350 分 / 153 评。 |
| 已知情况 “Ignite 总览称 7 款模型” | 属实：“a family of seven new world class models”。 |
| 已知情况“可用：GitHub Copilot；Code 2 Flash 在 Foundry 有限预览” | 与博客一致；另据 GitHub Changelog，MAI Code 2 的套餐清单未列 Free、Student；“annual plans” 一句的适用范围不清（见 B2）。 |

## 5. 对读者

每条均已核实（原句与出处见前文），没有的写“无”。

- 在哪能用：GitHub Changelog（2026-11-17，北京 11-18 02:01:45）原句：“MAI Code 2: Copilot Pro, Pro+, Max, Business, and Enterprise”；“MAI Code 2 Flash: Copilot Free, Student, Pro, Pro+, Max, Business, and Enterprise”；“Both models are available to eligible GitHub Copilot subscribers on annual plans.” 最后一句的适用范围不清（见 B2），起草时只引原句，不要改写成“仅年付”或“月付不可用”。
- 在哪选：VS Code、Visual Studio、Copilot CLI、cloud agent、Copilot app、github.com、GitHub Mobile、JetBrains、Xcode、Eclipse 的模型选择器（同上）。
- 企业与团队：Business/Enterprise 管理员可在 Copilot 设置里的 model policy 管理；默认情况下新模型自动启用，除非管理员关闭全局默认或显式禁用（同上）。
- 价格（每百万 token，GitHub Docs “Models and pricing”，取于北京 11-18 16:28）：MAI Code 2 促销价输入 $0.50 / 缓存输入 $0.025 / 输出 $2.50，至 2027-05-17；之后 $1.00 / $0.05 / $5.00。Flash 促销价 $0.05 / $0.005 / $0.25，之后 $0.10 / $0.01 / $0.50。对照同表：GPT-6.1 Sol 与 GPT-6.2 Sol（≤272K）$2.00 / $0.10 / $10.00；Claude Opus 5.5 $4.00 / $0.20 / $20.00。Copilot 对这些模型“billed under usage-based billing”（Changelog）。
- Free、Student：Changelog 的 MAI Code 2 套餐清单未列这两档（Flash 的清单列了）；supported-models 页原句：“Copilot Student and Copilot Free users have access to models through auto model selection only.”。`copilot/plans` 页的 Free 清单文字里却出现 MAI Code 2，与 Changelog 不一致，未解决。
- Foundry：只有 Flash，且为 “limited-access preview”；Foundry 价格页静态存档里无 MAI（B2）。
- OpenRouter、Vercel、Hermes、Open Code、Kilo：博客只写 “Soon”，无日期。
- 模型规格：1,050,000 token 上下文；输入上限 922,000，输出上限 128,000；支持文本与图像输入（模型卡）。
- 要不要换：官方口径是“日常企业编码”，博客明说 “not yet tuned for … very long horizon agentic work”；促销价到 2027-05-17 为止，之后 Docs 列出的价格为促销价的 2 倍。
