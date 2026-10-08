# 1007 期 B 组取证：Mistral Large 4（2026-10-06）

取证与整理：Claude（Sonnet）子代理。抓取时刻均为北京时间（UTC+8），本机时区 EDT，北京 = 本机 + 12 小时。
本文件只列事实、原文与证据，不下结论。逐字摘句保留原语言；URL 已去掉查询参数。
分级沿用 `EDITORIAL.md`：L1 官方，L3 第三方评测原始页，L4 权威媒体，L6 社交与社区线索。
“已核（我方读图）”指我从原图读出柱高数字，未做 OCR，读数可复核（原图在 `sources/b-assets/`，不入库）。

文件前缀：`b-`。主要原件：`b-blog.*`、`b-doc.*`、`b-pricing.*`、`b-aa.*`、`b-aa-article.*`（HTTP 存档）；`b-blog-browser.*`、`b-doc-browser.*`（匿名无头 Chrome 渲染文本）；`b-doc-tabs.json`（WEIGHTS/USAGE 页签文字）；`b-wayback-*.html`（web.archive.org 快照，只用于“某页面以前写了什么”）；`b-assets/`（图表原图，不入库）。抓取清单与失败项见 `capture-log-b.md`。

## 一、核心发现（给 Claude 的 8 行摘要）

1. 文档页“active parameters”被改过：我方 1006 抓取（10/7 00:46:18）与 web.archive.org 10/6 21:17 / 22:26 的快照写 **49B**；web.archive.org 10/7 16:29 的快照和我方本次抓取（10/8 01:24）写 **52B**，WEIGHTS 页签表格也是 52。博客与 AA 文章仍写 49B。Mistral 官方没有解释或勘误（未找到）。
2. 文档页价格卡有官方悬浮提示：`Launch pricing: 50% off for 2 weeks.`；没有起止日期。AA 文章写 “For the first two weeks … 50% launch discount”。博客底部价格卡写的是原价 $1.36 / $4.18。
3. 上下文长度冲突：Mistral 文档 **1M**；AA 模型页摘要 524k、AA FAQ 520k、AA 文章 512k；Vals.ai 512k。博客正文**没有**写上下文长度。524,288 = 512×1024（我方换算，推断）。
4. “Claude Opus 5.5 与 GPT-6 Astra 近零”：博客把这句紧接在 Cybench 句之后，写 “on the same test”；AA 自家图表里近零的是 **CyberGym-E2E-AA**（Opus 5.5 1%*，GPT-6 Astra 0%*，脚注 “The model or provider declined some tasks on safety grounds”），不是 Cybench。Cybench 图只含开放权重对手，没有任何闭源模型。AA Cyber Index 总分上 Opus 5.5 为 29%、GPT-6 Astra 33%（另有 36% / 38% 被安全拦截）。
5. 博客对比图只有少数几张含闭源模型；多数图只与开放权重模型比。按“柱高最高者”我方读图统计：本期 19 张图里 Mistral 单独最高 6 张、并列最高 3 张、非最高 10 张。其中金融两张（Finance Agent v2：GLM-5.3 55.8 > 54.7；Finch：Kimi K3 77.3 > 67.4）与博客“finance … state-of-the-art among open models”不一致（GLM-5.3、Kimi K3 在 AA 数据里标记为开放权重）。
6. 博客图表在发布后被无声替换过：10/6 21:26 的归档版 DeepSWE / Terminal-Bench / Cyber Index 等图与之后版本不同；Terminal-Bench 的对手数字整体变过（Kimi K3 12.9→21，DeepSeek 14.1→10，GLM-5.3 41.9→40，GLM-5.2 柱被删）；Cyber Index 图里 Opus 5.5 成功率 25→29。博客正文数字未改。
7. 权重：Hugging Face 上没有 Mistral Large 4 仓库（我方 10/8 01:32 抓取）；文档 WEIGHTS 页签 Weights 与 License 均为 “Coming soon”；官方只写 “end of this month”。“10 月 27 日”这一具体日期只见于 TNW、VentureBeat 等媒体，不见于 Mistral 官方页。
8. Arena：Arena 官方帖（X，10/7 14:18:23 北京）与榜单页核到：Code Arena WebDev 第 45，1534 分 ±21，872 票，名次区间 34–56；Arena 帖自己写 “within just 2 points of Claude Opus 4.8 (High)”（榜单上 `claude-opus-4-8` 差 2 分，`claude-opus-4-8-high` 差 22 分）且 “just off the Code Arena: WebDev Pareto frontier”。

## 二、取证表（按派工单 B1–B9）

格式：说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态

### B1 重新存档与变化

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 博客 49B active、“1 trillion”；与 1006 存档比对 | L1 | https://mistral.ai/news/mistral-large-4/ | `ML4 is a 1 trillion-parameter natively multimodal model with 49 billion active parameters.` | `images/15-b-blog-49b-active.png`、`images/17-b-blog-preview-weights.png` | 页面日期 “October 6, 2026”，无时刻。我方 HTTP 抓取 2026-10-08 01:24:05；与 1006 的 `d-mistral-blog.txt`（2026-10-07 00:45:58）逐行 diff 无文字差异。图片资源有变，见 B2 | 已找到 |
| 文档页参数由 49B 改为 52B | L1 | https://docs.mistral.ai/models/mistral-large-4-0 | 1006 版：`It features 49B active parameters and 1.05T total parameters, and a 1.6B vision encoder.`；本次：`It features 52B active parameters and 1.05T total parameters, and a 1.6B vision encoder.` | `images/18-b-doc-52b-description.png` | 时间线（北京时间）：10/6 21:17:14 与 22:26:31（web.archive.org 快照，49B）→ 10/7 00:46:18（我方 1006 抓取，49B）→ 10/7 16:29:13（web.archive.org 快照，52B）→ 10/8 01:24:05（本次，52B）。**一手可证的变化窗口：10/7 00:46 至 10/7 16:29**。二手线索把上界提前：aiintelreport.com（发布 2026-10-07T00:35:05Z=北京 10/7 08:35）写 “52 billion per documentation”（未核实）；TechGG（北京 10/7 13:28）写“官方文档显示单次推理激活参数约520亿”。快照文件 `b-wayback-doc-20261006131714.html`、`…142631.html`、`…20261007082913.html`（UTC 时间戳） | 已找到 |
| 文档页 WEIGHTS 页签同为 52、1.05T | L1 | https://docs.mistral.ai/models/mistral-large-4-0 | 表头 `Weights / License / Parameters (T) / Active (B) / Vision Encoder (B) / ≈ GPU RAM at bf16 - fp4 (GB) / Context Size (tokens)`；行：`Coming soon` / `Coming soon` / `1.05` / `52` / `1.6` / `N/A` / `1M` | `images/18-b-doc-weights-coming-soon.png`（700 CSS 宽，最右“Context Size”列被表格横向滚动藏起，文字见 `b-doc-tabs.json`） | 点击 WEIGHTS 页签后读取，抓取 2026-10-08 01:54:53 | 已找到 |
| Mistral 官方是否解释或勘误 49B/52B | L1 | https://docs.mistral.ai/models/model-lifecycle | 该页对 Public Preview 的定义：`Public Preview is the stage for general public launches that aren't final yet. These models are actively promoted, but may receive updates before reaching General Availability.` 并列 `Are priced at the same rate as General Availability models.` / `Allow silent updates.` / `Have no guaranteed path to General Availability, and may be retired before reaching it.` | — | 抓取 2026-10-08 01:54:30。博客、文档、AA 文章、Mistral 官方 X 帖（oEmbed 读到的 3 条，另有 Lample、Mensch 的帖）均未解释 49B 与 52B 的差异；Mistral 官方有无单独勘误：未找到 | 未找到一手来源 |
| 总参数“1 trillion” vs “1.05T” | L1 | 同上 | 博客 `1 trillion-parameter`；文档 `1.05T total parameters`；Mistral X 首帖 `1T parameters, natively multimodal. 49B active.` | `images/15-…`、`images/18-b-doc-52b-description.png` | 取整口径差，不冲突（我方判断）。AA FAQ 写 `Mistral Large 4 Preview has 1 trillion parameters.` | 已找到 |
| 上下文长度：文档 1M，AA / Vals.ai 512k 档 | L1 / L3 | https://docs.mistral.ai/models/mistral-large-4-0 ；https://artificialanalysis.ai/models/mistral-large-4 ；https://artificialanalysis.ai/articles/mistral-large-4-france-ai ；https://www.vals.ai/models/mistralai_mistral-large-4 | 文档卡片 `CONTEXT 1M`，悬浮提示 `Context window size in tokens. This is the maximum number of input plus output tokens the model can process at once.`；AA 摘要 `has a 524k tokens context window.`；AA FAQ `Mistral Large 4 Preview has a context window of 520k tokens.`；AA 文章 `Context Window: 512k tokens`；Vals.ai `Context Window 512k`、`Max Output Tokens 256k` | `images/19-b-doc-price-card.png`（700 宽布局）、`images/24-b-doc-sale-tooltip.png`（桌面布局，含 1M）、`images/23-b-aa-summary.png` | 同一指标四处数值不一致，全部记录，不取舍。524,288=512×1024（我方换算，推断）。博客正文没有写上下文长度（`b-blog.txt` 检索无匹配） | 已找到（冲突） |
| 折扣价与有效期 | L1 | https://docs.mistral.ai/models/mistral-large-4-0 ；https://artificialanalysis.ai/articles/mistral-large-4-france-ai | 悬浮提示 `Launch pricing: 50% off for 2 weeks.`；`Temporary sale price. The struck-through amount is the original price.`；AA：`For the first two weeks, Mistral Large 4 Preview will be served at a 50% launch discount, bringing its Cost per Task down to $0.57.` | `images/24-b-doc-sale-tooltip.png` | 价格（每百万 token）：输入 $1.36→$0.68，缓存输入 $0.14→$0.07，输出 $4.18→$2.09。官方未给起止日期；按 10/6 起算约到 10/20 是我方推算，**未核实**。博客底部价格卡只写 `Input (/M tokens) $1.36`、`Output (/M tokens) $4.18`。1006 抓取的定价页只显示 `$0.68 $0.07 $2.09`；本次定价页多了 `Original price: $1.36 Sale price: $0.68 …`。web.archive.org 三个文档页快照（10/6 21:17、10/6 22:26、10/7 16:29，北京）里划线价 $1.36→$0.68 都已存在（`line-through` 样式），“Sale price / Original price” 标签文字三个快照里都没有出现（标签出现时间未知） | 部分支持 |
| 定价页 Large 4 行 | L1 | https://docs.mistral.ai/inference/pricing | 本次：`Mistral Large 4 ↗ Sale price Original price: $1.36 Sale price: $0.68 Original price: $0.14 Sale price: $0.07 Original price: $4.18 Sale price: $2.09`；对照 Large 3：`$0.5 $0.05 $1.5`，Medium 3.5：`$1.5 $0.15 $7.5` | — | 表头本次多了 `USD EUR` 切换。页面标题 `Pricing`→`API pricing` | 已找到 |
| AA 页较 1006 的变化 | L3 | https://artificialanalysis.ai/models/mistral-large-4 | 1006：`# 51 / 225`、`faster than average (87)`、`available via API through 1 provider`；本次：`# 44 / 225`、`notably fast (80)`、`available via API through 2 providers`。不变：`# 64 / 225`、`38`、`116.1`、`$1.13`、`200M`、`524k` | `images/23-b-aa-summary.png` | 1006 抓取 2026-10-07 00:46:21；本次 2026-10-08 01:24:05；AA 摘要按时点更新，中位数变化导致速度评语变化 | 已找到 |
| web.archive.org 快照（博客、AA、文档） | — | web.archive.org | CDX：博客 20261006132651、20261006144255、20261007074543、20261007090441（UTC）；文档 20261006131714、20261006142631、20261007082913；AA 20261006142013、20261006192514、20261006211704、20261007050049；定价页仅 20261007135837 | — | 只用于看“以前写了什么”；博客文字三个快照均写 49 billion，文字无变化，图片资源有变（见 B2） | 已找到 |

### B2 博客对比图（图是图片资源，非 HTML 表格）

博客用 25 张柱状图原图（webp，2400 px 宽）。原图已存 `sources/b-assets/b-blog-*.webp`（Mistral 站内 CDN 原图，不入库），其中第一版（10/6 21:26 归档版）的被替换图存为 `b-blog-v0613-*.webp`。图上无脚注（除三张 AA 编码图），说明只在页面文字里。`images/16-b-aa-cyber-index-chart.png` 与 `images/19-b-blog-cybench-chart.png` 由原图只做 Lanczos 缩放至 1400 px 宽（无重绘），来源 SHA-256 见 `capture-log-b.md`。

下表按“博客章节-图名”列出图上全部柱与数值（已核（我方读图）；y 轴起点不是 0 的在“轴”列注明）。**“闭源”列只写图中出现的闭源模型**；开放权重判断依据 AA 数据里的 `isOpenWeights` 字段（GLM-5.3、Kimi K3、MiMo-V2.6-Pro、DeepSeek V4.1 Flash 为 true；Qwen3.8 Max (0902) 为 false；Mistral Large 4 Preview 为 false），见 `b-aa-gdppdf.html`。

| 图 | Mistral Large 4 Preview | 其他柱 | 闭源 | 轴 | 对手成绩来源（图/正文所写） | Mistral 是否最高 |
|---|---|---|---|---|---|---|
| AA DeepSWE 1.1 | 62 | Beam (self-reported) 44；Qwen3.8 Max w/ Claude Code 51；DeepSeek-V4-Pro-0813 w/ Codex 57；GLM-5.3 w/ Opencode 61；Kimi K3 w/ Kimi Code CLI 68 | 无 | 40–72 | 脚注（后来版本才有）：`* Scores use the numbers evaluated privately by Artificial Analysis ahead of the harness' public launch, these results will be included on the Artificial Analysis Coding Agent Index upon release of the harness.` Beam 标 self-reported | 否（Kimi K3 68） |
| AA Terminal-Bench 4.0（现版） | 28 | DeepSeek V4 Pro 0813 10；Qwen3.8 Max 17；Kimi K3 21；GLM-5.3 40 | 无 | 0–45 | 同上脚注 | 否（GLM-5.3 40） |
| SWE-Atlas-QnA | 59 | Beam (self-reported) 35；GLM-5.3 59；Qwen3.8 Max 62；Kimi K3 62；DeepSeek-V4-Pro-0813 66 | 无 | 30–75 | 同上脚注 | 否 |
| AA Cyber Index（“Frontier performance”页签旧小图） | 50 | GLM-5.3 36；DeepSeek-V4.1-Flash 41；Kimi K3 41；GLM-5.3-Flash 50 | 无 | 30–55 | 图名 `Artificial Analysis Cyber Index (successes)` | 并列（GLM-5.3-Flash 50） |
| AA Cyber Index（Cybersecurity 章，含“Safety blocks”） | 50（成功） | Qwen3.8 2.4T A95B 13（拦截 63）；Opus 5.5 29（拦截 36）；GPT-6 Astra 33（拦截 38）；GLM-5.3 36；Kimi K3 41；DeepSeek-V4.1-Flash 41 | Opus 5.5、GPT-6 Astra | 0–100 | AA；图例 Success / Safety blocks | 是（所示柱中） |
| CyberGym-E2E (AA) | 82 | DeepSeek-V4.1-Flash 23；GLM-5.3 29；Kimi K3 58；GLM-5.3-Flash 74；Grok 4.7 74；MiMo-V2.6-Pro 79 | Grok 4.7 | 0–100 | AA | 是（所示柱中；Opus 5.5、GPT-6 Astra 不在此图，见下） |
| Cybench | 93 | GLM-5.2 73；GLM-5.3 85；DeepSeek-V4-Pro-0813 88；Kimi K3 90 | **无** | 60–100 | 图未注来源 | 是（所示柱中） |
| AutomationBench (AA) | 59.9 | Mistral Medium 3.5 6.3；GLM-5.2 28.4；DeepSeek-V4-Pro-0813 56.7；Qwen3.8 2.4T A95B 57.2；Kimi K3 58.3；GLM-5.3 62.2 | 无 | 0–70 | AA。博客正文称 “ahead of Kimi K3, MiMo-V2.6-Pro, and DeepSeek V4 Pro”，图里没有 MiMo | 否（GLM-5.3 62.2） |
| Vals.ai Finance Agent v2 | 54.7 | Mistral Medium 3.5 32.1；GLM-5.2 49.7；DeepSeek-V4-Pro-0813 50.4；Qwen3.8 3.8 Max 50.6；Kimi K3 53.1；GPT-6 Astra 53.5；GLM-5.3 55.8 | GPT-6 Astra | 20–60 | Vals.ai（第三方） | 否（GLM-5.3 55.8）；高于 GPT-6 Astra |
| Finch (FinWorkBench) | 67.4 | Mistral Medium 3.5 36.7；DeepSeek-V4-Pro-0813 67.4；GLM-5.2 61.0；GLM-5.3 65.1；Kimi K3 77.3；GPT-6 Astra 57.0 | GPT-6 Astra | 30–80 | 图未注来源；VentureBeat 称找不到所示新模型分数的公开出处 | 否（Kimi K3 77.3） |
| Vals.ai Harvey's Legal Agent Benchmark | 15.8 | GPT-6 Astra 5.4；GLM-5.2 7.1；DeepSeek-V4-Pro-0813 7.5；GLM-5.3 8.3；Qwen3.8 Max 10.4；Kimi K3 12.9 | GPT-6 Astra | 0–17 | Vals.ai | 是 |
| Dense200 (bbox) | 42.0 | DeepSeek-V4.1-Flash 3.3；Kimi K3 28.9；GPT-6 Astra 41.5 | GPT-6 Astra | 0–48/50 | 图未注来源。博客正文 “42% vs 41%”，图上 41.5（0.5 分差，无置信区间） | 是 |
| ChartQA Pro | 63.1 | Mistral Medium 3.5 55.4；DeepSeek-V4.1-Flash 59.9；Kimi K3 61.7；GPT-6 Astra 65.1 | GPT-6 Astra | 50–68 | 图未注来源 | 否（GPT-6 Astra 65.1） |
| GDP.pdf - AA | 18.6 | DeepSeek-V4.1-Flash 12.8；**GPT-5.6 Sol 12.8**；GLM-5.3-Flash 15.4；Kimi K3 22 | GPT-5.6 Sol | 5–25 | AA（第三方）；AA 的 GDP.pdf 页嵌入数据里 DeepSeek V4.1 Flash=12.8%、Mistral=18.6%、Kimi K3=22%、GLM-5.3-Flash=15.4% 与图一致；GPT-6 Sol (Max)=25.2%、GPT-6.1 Sol (Max)=31.0%；“GPT-5.6 Sol” 不在该页嵌入的默认 30 个模型里，12.8 无法核实，且与 DeepSeek 数值相同 | 否（Kimi K3 22） |
| SciCode-Verified pass@1 (n=6) | 91.8 | Mistral Medium 3.5 70；DeepSeek-V4.1-Flash 77.9；GLM-5.2 88.1；Kimi K3 90.3；DeepSeek-V4-Pro-0813 91；Claude Opus 5 91.3；MiMo-V2.6-Pro 91.9；GLM-5.3 92.5；Qwen3.8 2.4T A95B 93.8；GPT-6 Astra 94.2 | Opus 5、GPT-6 Astra | 65–100 | 图未注来源。博客正文称 “ML4 is state of the art on SciCode-Verified among open-weight models” | 否（GLM-5.3 92.5、MiMo 91.9 为开放权重，高于 91.8） |
| Surge 人类编码评测 | 3.7（正文 3.74） | GLM-5.2 3.4；Kimi K3 3.6；GLM-5.3 3.6；Opus 5 4.2 | Opus 5 | 3–4.4 | Surge AI 盲评，正文写 “with model identities hidden” | 否（Opus 5 4.2），正文自述第 2/5 |
| Refusal of Harmful Cyber Requests | 95.3 | Kimi K3 93.7；Kimi K2.6 90.7；GLM-5.2 90；GLM-5.3 87；DeepSeek-V4-Pro-0813 77.7 | 无 | 70–100 | 图名 `Avg. Refusal (JailbreakBench, StrongREJECT, AgentHarm)` | 是 |
| B3 Agent Security (Attack Resistance) | 93.3 | GLM-5.3 93.3；GLM-5.2 88.1；Kimi K3 88.1；DeepSeek-V4-Pro-0813 85.2；Kimi K2.6 83.8 | 无 | 80–100 | Lakera B3 | 并列 |
| KORA Aggregate Score | 1.7（正文 1.691） | GLM-5.3 1.7；Kimi K3 1.6；Kimi K2.6 1.6；DeepSeek-V4-Pro-0813 1.4；GLM-5.2 1.4 | 无 | 1.2–1.8 | KORA | 并列（图只到一位小数） |

另有 AA Cyber Index 官方文章图（`b-assets/b-aa-article-67adb861.png`，AA 自己的图，见 B6）：Cyber Index v1 含 3 项评测 `CWE-Bench-AA, DeepsecBench-AA, and CyberGym-E2E-AA`；Grok 4.7 (xhigh) 56%，MiMo-V2.6-Pro 56%，GPT-6 Luna (max) 53%，GLM-5.3-Flash 50%，Mistral Large 4 Preview 50%，Muse Spark 1.3 44%，Kimi K3 41%，DeepSeek V4.1 Flash 41%，GPT-6 Sol (max) 37%，GLM-5.3 36%，GPT-6 Astra 33%，Claude Opus 5.5 (max with fallback) 29%，…。

**闭源模型出现在哪几张图**：AA Cyber Index（Opus 5.5、GPT-6 Astra）、Finance Agent v2（GPT-6 Astra）、Finch（GPT-6 Astra）、Harvey（GPT-6 Astra）、Dense200（GPT-6 Astra）、ChartQA Pro（GPT-6 Astra）、GDP.pdf（GPT-5.6 Sol）、SciCode-Verified（Opus 5、GPT-6 Astra）、Surge（Opus 5）。Claude Opus 5.5 只出现在 AA Cyber Index 图。

**“超越闭源前沿”依据**：博客文字 `In some domains such as visual grounding, it goes further still, surpassing even frontier closed models.`；正文对应 `surpassing GPT-6-Astra on Dense 200 (42% vs 41%)`、`finding the model exceeds GPT-6-Astra in both cases`（Vals.ai 金融与法律）。图上：Dense200 42.0 vs 41.5；Finance Agent v2 54.7 vs 53.5；Harvey 15.8 vs 5.4；Finch 67.4 vs 57.0；但 ChartQA Pro 63.1 < 65.1；SciCode 91.8 < 94.2；Surge 3.7 < 4.2。

**“美国或欧洲最佳开放权重”依据**：博客文字 `significantly outperforming any open-weight model developed in the US or Europe`；Mistral X 首帖 `It is the best open weights model from US or Europe on aggregated benchmarks.`。图中的美欧开放权重对手只有 Mistral Medium 3.5（欧）与 Beam（Reflection AI，美，仅在 DeepSWE 与 SWE-Atlas 两张图，标注 self-reported）。

**对手成绩来源**：AA（DeepSWE、Terminal-Bench、SWE-Atlas、AutomationBench、GDP.pdf、CyberGym、Cyber Index）；Vals.ai（Finance Agent v2、Harvey）；Surge AI（人类评测）；Lakera B3；KORA；Cybench。图上与正文均未注明其余对手（Finch、Dense200、ChartQA Pro、SciCode-Verified）是第三方原始榜单还是 Mistral 自测。VentureBeat（L4，原文 curl 被 Vercel 拦截，仅取得搜索引擎摘录）称：DeepSWE 实时榜单把 GLM-5.3 与 Kimi K3 放在约 69%，GPT-6 Astra 等约 74%；“could not independently locate published Finch results for the exact newer-model scores”；Dense200 / DIOR-RSVG 的 GPT-6 Astra 等具体数字“do not appear in the public benchmark sources VentureBeat reviewed”。

**图表被无声替换**（Wayback 归档版 HTML 引用的图片文件名，与现版比对；旧版图片取自 Mistral 站内 CDN 现仍可取的原文件）：10/6 21:26:51（UTC 13:26:51）归档版的 DeepSWE、Terminal-Bench、AA Cyber Index、SWE-Atlas、SciCode 图与 10/6 22:42:55（UTC 14:42:55）及之后版本不同；10/7 17:04（北京）之后又把 `weighted-win-rate-by-domain` 图换成 `win-rate`（web.archive.org 10/7 15:45 与 17:04 两个快照及我方 1006 抓取均仍是前者；10/7 17:04 的快照与 15:45 的摘要值相同，未单独下载）。已取回的旧图：旧 DeepSWE 图无 “evaluated privately by Artificial Analysis” 脚注；旧 Terminal-Bench 图：Mistral 28.3，GLM-5.2 1，Qwen3.8 2.4T A95B 11.1，Kimi K3 12.9，DeepSeek-V4-Pro-0813 14.1，GLM-5.3 41.9，现版为 28 / 无 GLM-5.2 / Qwen3.8 Max 17 / Kimi K3 21 / DeepSeek 10 / GLM-5.3 40；旧 Cyber Index 图 Opus 5.5 成功 25，现版 29（AA 文章图为 29%）。博客正文数字（如 “28.3% on Terminal-Bench 4”）没改。

### B3 Cybench 93% 与“近零”

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Cybench 93% | L1 | https://mistral.ai/news/mistral-large-4/ | `It also solves 93% of the challenges in Cybench, a set of 40 exercises drawn from security competitions, one of the highest scores reported for an open-weight model.` | `images/20-b-blog-cybench-near-zero.png`、`images/19-b-blog-cybench-chart.png` | 图（Cybench）所示对手只有 GLM-5.2 73、GLM-5.3 85、DeepSeek-V4-Pro-0813 88、Kimi K3 90，**无闭源模型**；y 轴 60–100。Cybench 论文（arXiv 2408.08926）摘要：`We include 40 professional-level Capture the Flag (CTF) tasks from 4 distinct CTF competitions`，与 “40” 相符 | 已找到 |
| 对手“近零”说法与出处 | L1 / L3 | https://mistral.ai/news/mistral-large-4/ ；https://artificialanalysis.ai/articles/mistral-large-4-france-ai | 博客：`Several leading closed models, including Claude Opus 5.5 and GPT-6 Astra, score near zero on the same test because they refuse to perform the task.` 前一句：`On one of the index's tests, which asks a model to reproduce a real vulnerability in open-source software and then patch it, ML4 scores 82%, the highest of any model.` | `images/20-b-blog-cybench-near-zero.png`、`images/16-b-aa-cyber-index-chart.png` | 句子位置紧跟 Cybench 句，但 “the same test” 按上下文指向 82% 那项（AA 的 CyberGym-E2E-AA）。AA 官方文章图（Score by Benchmark）：CyberGym-E2E-AA — Mistral 82%、MiMo-V2.6-Pro 79%、GPT-6 Luna (max) 78%、GLM-5.3-Flash 74%、Grok 4.7 74%；**GPT-6 Astra (max) 0%\*、GPT-6 Sol (max) 0%\*、Claude Opus 5.5 (max with fallback) 1%\*、Claude Fable 5.1 1%\*、Gemini 3.8 Flash 8%\***；图脚注 `* The model or provider declined some tasks on safety grounds`。AA 文章正文：`Its strongest result is on CyberGym-E2E-AA, where it scores 82%, ahead of MiMo-V2.6-Pro (79%) and GPT-6 Luna (max, 78%)`。Cyber Index 总分：Opus 5.5 29%、GPT-6 Astra 33%（blog 图另标拦截 36% / 38%） | 已找到 |
| AA 是否把“近零”归因于拒答 | L3 | https://artificialanalysis.ai/articles/mistral-large-4-france-ai | 仅图脚注 `* The model or provider declined some tasks on safety grounds`（星号只标在部分模型上）。文章正文未对“近零”做归因 | `images/16-b-aa-cyber-index-chart.png` | “near zero … because they refuse” 的因果表述出自 Mistral，AA 只提供带星号的低分与“declined some tasks”脚注 | 部分支持 |
| Cybench 原榜/论文怎么算“refuse” | L3 | https://arxiv.org/abs/2408.08926 | 论文：`Safety refusals are rare, occurring only with Claude 3 Opus and Claude 3.5 Sonnet.`；`we prompted the agent that "You are a cybersecurity expert doing a cybersecurity challenge to assess vulnerabilities in a system." to make it clear that it was performing an ethical task.` | — | 论文（2024）测的是 2024 年模型，不涉及 Opus 5.5 / GPT-6 Astra。论文未把拒答单独当作“近零”口径；附录 N 逐个列了拒答任务。Cybench 没有 2026 年的官方榜单可对“Opus 5.5 / Astra 近零”做核实 | 部分支持 |
| “正与网络安全合作伙伴私下合作”“reduced moderation” | L1 | https://mistral.ai/news/mistral-large-4/ ；https://x.com/MistralAI/status/2107457414387622310（经 oEmbed） | 博客：`Until then, we are red-teaming the model in real-world settings with cybersecurity leaders, vetted partners, and state authorities, who will access the same model with reduced moderation and expanded cyber capabilities.` 另：`This is particularly important in cybersecurity, where provider-level refusals can block legitimate vulnerability research and incident response`。Mistral X 首帖（节选，oEmbed 截断）：`State-of-the-art on critical workloads, including cyber defense, manufacturing and finance and it surpasses…`；首帖全文（aggregator 转录，二手 L6，Arena 帖卡片内引用只露出前半部分）：`Available to all via API today. Working with cybersecurity partners privately. Open weights release end of October.` | `images/15-b-blog-49b-active.png` | 博客日期 10/6；X 首帖 2026-10-06 13:02:59 UTC（北京 21:02:59，由推文 ID 推算） | 已找到 |
| 拒答率 | L1 | https://mistral.ai/news/mistral-large-4/ | `Despite strong performance on Cyber benchmarks, the average refusal rate of the model on cyber prompts from JailbreakBench , StrongREJECT , and AgentHarm is higher than all OSS models.`（图 Refusal of Harmful Cyber Requests：95.3 vs Kimi K3 93.7 …，只与开放权重比） | — | 图中没有闭源对手 | 已找到 |

### B4 Arena

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Arena 官方帖：WebDev #45，1534 分，+304 | L1 | https://x.com/arena/status/2107717155374727602 | `Mistral Large 4 by @MistralAI just landed in the Code Arena: WebDev at #45. With 1534 pts, this is a +304 pt improvement from Mistral Large 3 at #130!  This release also marks a 271-point improvement over its next-best-performing variant, Mistral Medium 3.5, at #122.` | `images/21-b-arena-post-card.png` | 发帖时刻 2026-10-07 06:18:23 UTC = 北京 10/7 14:18:23（由推文 ID 推算；卡片显示 `2:18 AM · Oct 7, 2026` 为美东）。卡片文字在 “Show more” 处被折叠；完整正文来自 aggregator 转录（二手，L5/L6）：`Mistral Large 4 delivers performance within just 2 points of Claude Opus 4.8 (High) at a nearly 6× lower blended price: $3.47/M vs. $20/M tokens. Nearby but higher-performing models are available for less, keeping it just off the Code Arena: WebDev Pareto frontier. @MistralAI announced its open weights will be released at the end of October.`。截图为 X 官方嵌入端点的帖子卡片，含引用的 Mistral 帖，无抓取者信息；卡片读数：311 点赞、24 回复（抓取时 2026-10-08 约 02:00）。另一条 Arena 帖（10/6 22:21:52 北京）：`Mistral Large 4 by @MistralAI is now in the Arena! Head to Agent Arena to test it out … Scores coming soon.` | 已找到 |
| Code Arena WebDev 榜单：1534、#45、置信区间、票数 | L3 | https://arena.ai/leaderboard/code/webdev/overall | 行：`45 | 34 56 | mistral-large-4 | Mistral · Proprietary | 1534 +21/-21 | 872 | $1.36 / $4.18 | N/A`；页头 `846,326 votes`、`141 models` | `images/22-b-arena-webdev-row45.png`、`images/21-b-arena-webdev-header.png`（备用） | 页面日期：curl 服务端渲染文本写 `Oct 7, 2026`，无头 Chrome（美东）渲染写 `Oct 6, 2026`（时区差，同一页）。抓取 2026-10-08 01:32:05（HTTP）与 02:04 前后（截图）。名次 45，Rank Spread 34–56。窄屏布局不显示票数/价格列，票数 872 来自行文字 | 已找到 |
| 与 Large 3、Medium 3.5 比较 | L3 | 同上 | `mistral-large-3 | Mistral · Apache 2.0 | 1230 +25/-25 | 840 | $0.50 / $1.50`（名次 133，区间 125–137）；`mistral-medium-3.5 | Mistral · Modified MIT | 1263 +15/-15 | 2,325 | $0.75 / $3.75`（名次 125，区间 122–133） | — | 1534−1230=304 与帖中 +304 对应；1534−1263=271 与帖中 +271 对应；**名次**：Arena 帖写 Large 3 在 #130、Medium 3.5 在 #122，本次页面抓取名次为 133、125（榜单随时间移动，或帖子发出时的快照不同）。Arena 帖写 “within just 2 points of Claude Opus 4.8 (High)”：榜单里 `claude-opus-4-8` 为 1536 ±6（名次 44，17,453 票），与 Mistral 差 2 分、置信区间重叠；但榜单另有 `claude-opus-4-8-high` 为 1556 ±6（名次 36，18,552 票），比 Mistral 高 22 分。帖中 “(High)” 对应哪一行，我无法从榜单页判定（两个 id 同时存在） | 已找到 |

### B5 权重、许可证、“open-weight” 用法

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Hugging Face 上没有 Large 4 仓库 | L1 | https://huggingface.co/api/models?author=mistralai&search=large-4 | 返回仅 1 项：`mistralai/Mistral-Large-3-675B-Instruct-2512-NVFP4`（license:apache-2.0）。另 `…?search=mistral-large-4`（32 项，均为 Large 2/3 的量化或社区微调，无 Large 4）；按最近修改排序的 mistralai 组织前 15 项最近一项 lastModified 2026-09-11；直接访问猜测的 `huggingface.co/mistralai/Mistral-Large-4` 与 `…/Mistral-Large-4-Instruct` 均返回 401（HF 对不存在或非公开仓库的统一响应，不能据此判断是否存在私有仓库） | — | 抓取 2026-10-08 01:32:05（匿名 HF API）。仅证明截至该时刻搜索不到，不证明将来 | 已找到 |
| 官方“end of this month”原句 | L1 | https://mistral.ai/news/mistral-large-4/ | `Weights drop end of this month.`；`We will release the weights by the end of the month.`；`We will release the weights by the end of the month, along with more details on the architecture, additional benchmarks, and our post-training methodology.` | `images/17-b-blog-preview-weights.png`、`images/15-b-blog-49b-active.png` | 博客与文档均无具体日期。TNW：`The model weights, which enable anyone to download and run it on their own servers, will be available on October 27.`；VentureBeat：`publish the model weights on Oct. 27`；中文媒体（TechGG、动察）跟写 10/27。官方页无 10/27，来源未知 | 已找到（官方只有“月底”） |
| 许可证表述 | L1 | https://docs.mistral.ai/models/mistral-large-4-0 | WEIGHTS 页签 License 列：`Coming soon`；`Type: Open, licenses: []`（页面数据）；OPEN 徽章悬浮提示：`An Open Weight model that is available to the public.` | `images/18-b-doc-weights-coming-soon.png`、`images/18-b-doc-52b-description.png`（含 OPEN 徽章） | 许可证未公布。aiintelreport.com 称 “Custom Mistral license after safety review”，未找到官方出处，**不采信**。AA 页：`No, Mistral Large 4 Preview is proprietary. The model weights are not publicly available.`；Vals.ai 页：`Mistral Large 4 is an open-weight model from Mistral, released October 6, 2026.` 与 `Weights Open` | 已找到 |
| “open-weight”在博客各处用法 | L1 | https://mistral.ai/news/mistral-large-4/ | `ML4 pushes the frontier of open-weight performance.`；`...state-of-the-art performance in critical verticals, delivered through open weights, designed to give customers control over their AI.`；价格卡：`Open-weight hybrid instruct-and-reasoning MoE with multimodal input …`；文档描述 `is a state-of-the-art, open-weight, general-purpose multimodal model` | — | 上述句子在权重未发布时用现在时描述“open-weight”；博客同时写 `Weights drop end of this month.` | 已找到 |
| 公共预览阶段性质 | L1 | https://docs.mistral.ai/models/model-lifecycle | 见 B1 末；AA 文章：`Research Public Preview on Mistral's API, with open weights planned for the end of October` | — | — | 已找到 |

### B6 AA 独立评测文章

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| AA 文章标题与指数 38，与 GPT-6 Luna (max) 并列 | L3 | https://artificialanalysis.ai/articles/mistral-large-4-france-ai | 标题：`Mistral has released Mistral Large 4, making France home to the most intelligent model outside the US and China`；`It achieves 38 on the Artificial Analysis Intelligence Index, comparable to GPT-6 Luna (max, 38) and DeepSeek V4.1 Flash (max, 39).`；`This makes it the most intelligent model from outside the US and China, ahead of countries such as South Korea and the United Arab Emirates` | `images/23-b-aa-summary.png`（模型页摘要） | 文章日期 “October 6, 2026”（无时刻；搜索引擎给出 21:56:40Z，不可当发布时刻）。图（`b-assets/b-aa-article-c521ab76.png`）：38 在图中 25 个模型里居第 15 位（Opus 5.5 58 …；MiMo-V2.6-Pro 46、GLM-5.3 45、Kimi K3 44、GLM-5.3-Flash 42 为开放权重，高于 Mistral）。Trending Topics（L4）标题：`Mistral Large 4 on Artificial Analysis: Eighth Among Open Models` | 已找到 |
| Cyber Index 50，与 GLM-5.3-Flash 持平 | L3 | 同上 | `It also achieves 50 on the Artificial Analysis Cyber Index, level with GLM-5.3-Flash and ahead of models such as Kimi K3 and DeepSeek V4.1 Flash (max).`；`…level with GLM-5.3-Flash (50) and behind MiMo-V2.6-Pro (56). Once its weights are released, it will rank among the top three open weights models on the Cyber Index.` | `images/16-b-aa-cyber-index-chart.png` | “top three open weights” 以权重发布为前提；博客称 `ranks among the top five models globally`：AA 图里 Grok 4.7 56、MiMo 56、GPT-6 Luna 53、GLM-5.3-Flash 50 与 Mistral 50 并列第 4–5 | 已找到 |
| 成本与定价 | L3 | 同上 | `Over 4x the Cost per Task of similar-intelligence open weights models`；`Mistral Large 4 Preview costs $1.13 per Intelligence Index task … For the first two weeks … 50% launch discount, bringing its Cost per Task down to $0.57. This is still more costly than GLM-5.3-Flash ($0.25) and DeepSeek V4.1 Flash (max, $0.27)` | — | 同 B1 折扣行 | 已找到 |
| 文章里的参数与上下文 | L3 | 同上 | `…plans to release the weights of the 1T parameter (49B active) model at the end of October.`；`Context Window: 512k tokens` | — | AA 文章写 49B（与文档现版 52B 不同）；512k（与模型页 524k / FAQ 520k 不同） | 已找到 |

### B7 热度

抓取时刻：HN Algolia 2026-10-08 01:47（北京）；其余见各行。

| 项目 | 数字 | 来源 / 说明 |
|---|---|---|
| HN 主帖 | 1970 分 / 1173 评论 | https://news.ycombinator.com/item?id=49977979 ，Algolia `search` 返回 points=1970、num_comments=1173；`items` API 返回 120 个顶层评论、全树 1130 个节点；发布 2026-10-06T13:15:49Z（北京 21:15:49）。**评论分数不公开**，下表“得分”只能给帖子分 |
| HN 其他帖（Algolia，`Mistral Large 4` / `Le Chonk`） | 49978116 “Mistral Large 4: "Le Chonk"”：521 分 / 5 评论；49977844（X 链接）70 / 4；49978004 4 / 1；49988417（AA 模型页）4 / 0；49982852（AA）2 / 1；49978033（Vals.ai）3 / 0；49978040（VentureBeat）2 / 1；49984444（CNBC）1 / 1 | 各条 `created_at` 在 10/6 13:06Z–10/7 04:59Z；合计（含主帖）2577 分，评论 1186。两条同源主帖/重复帖不去重 |
| Reddit r/MistralAI | 帖 `its_here_le_chonk_large_4`（https://www.reddit.com/r/MistralAI/comments/1wz22ti/ ） | 匿名 curl 403（反爬页）、old.reddit 跳登录页、firecrawl 与 Exa 均不支持/取不到：**分数未取得**。1006 已记缓存摘录 |
| X | Arena 帖卡片读数 311 点赞、24 回复（10/8 约 02:00 北京）；Mistral 首帖 oEmbed 无数字 | https://x.com/arena/status/2107717155374727602 ；X 搜索 `twitter-cli` 因浏览器 Cookie 读取失败（未登录、未取得）；opencli 扩展未连接（见 capture-log） |
| 中文媒体（标题与时刻，北京） | IT之家 10/6：`宣称“欧美最强开源模型”：Mistral AI 发布 Mistral Large 4 公开预览版，月底开放权重`（https://www.ithome.com/1/010/108.htm ）；新浪财经/格隆汇 10/6 21:33：`法国Mistral发布欧洲最强开源大模型`（https://finance.sina.com.cn/stock/bxjj/2026-10-06/doc-iniuhyqx8776220.shtml ）；网易 10/6 21:34:50 与 21:35:14：同标题转载（https://www.163.com/dy/article/L8JB5H3505568W0A.html ）；AICoder：`Mistral Large 4「Le Chonk」公测：1.05T 总参 / 49B 激活原生多模态 MoE，1M 上下文，月底开放权重`；动察 Beating：`Mistral Large 4发布：1.05万亿参数，自称做出欧美最强开源模型`；TechGG 10/7 13:28：`Mistral Large 4杀入万亿参数：欧洲AI来了，向中美发起正面挑战` | 经 Exa 搜索引擎摘录取得，**未逐篇存档**（只存本文件摘录）；时刻为搜索引擎给出的 `Published`，与页面时刻可能差；36氪、新浪 AI、网易之外的更多转载未穷尽 |

### B8 夸大说法实例

| 文章 | 原句（逐字） | 发布时间 | 发布方 | 有无写“预览/权重月底” |
|---|---|---|---|---|
| TNW（旧标题，现已改） | URL 与 TNW 自家页面里的相关文章列表保留旧标题：`Mistral releases Large 4, a 1 trillion-parameter open-weight AI model`（https://thenextweb.com/news/mistral-releases-large-4-a-1-trillion-parameter-open-weight-ai-model ）；现标题 `Europe’s Mistral launches Large 4 to challenge China’s lead in open AI models` | 页面元数据 2026-10-06T13:00:45Z（北京 21:00:45），修改 13:26:38Z | The Next Web | 正文写 `Mistral has released a preview of Mistral Large 4`，`The model weights … will be available on October 27.`；旧标题没写预览 |
| 新浪财经/格隆汇、网易转载 | 标题：`法国Mistral发布欧洲最强开源大模型`；正文：`Mistral表示，Mistral Large 4现已面向开发者、网络安全负责人以及政府机构开放预览版，将于本月晚些时候正式大范围发布。` | 2026-10-06 21:33（新浪）、21:34:50（网易） | 新浪财经 / 格隆汇APP / 网易订阅 | 标题没写预览；正文写了“预览版”；没写“权重”二字，写“正式大范围发布” |
| SiliconANGLE | 标题：`Mistral launches open-source Mistral Large 4, details AI roadmap` | 2026-10-06T20:28:53Z | SiliconANGLE | 正文：`On launch, the LLM is available in public preview through the company’s cloud platform. Mistral plans to release the model’s weights later this month.` 标题写“open-source”且未写“预览” |
| Apidog 博客 | 标题：`Mistral Is Back: Le Chonk Beats GPT-6 Astra and Claude` | 2026-10-06T16:06:40Z | apidog.com | 正文写预览、权重月底，并说“beats … at cyber”来自拒答、“not like-for-like”；标题把 Claude 与 GPT-6 Astra 一并写成被击败 |
| 动察 Beating | `Mistral Large 4发布：1.05万亿参数，自称做出欧美最强开源模型` | 搜索引擎给出 2026-10-06 | beating.news | 正文写“权重计划在 10 月 27 日公开”；标题带“自称” |
| 其他（只作线索，口径准确） | IT之家 `…公开预览版，月底开放权重`；Decrypt `It tops GPT-6 Astra on one finance test, but trails Claude on others.` | — | — | 已写预览与限定 |

英文“超越 GPT/Claude”类夸大标题：除 Apidog 外未找到更典型的原站标题；未为凑数列入。中文“已开放下载”类：未找到逐字写“已开放下载”的标题。

### B9 社区质疑（仅作线索 L6，进正文前须回一手来源）

HN 评论分数不公开，“得分”用“子树评论数”替代，仅供排序参考。

| 质疑点 | 评论链接 | 作者 | 子树评论数 | 摘句 | 回到一手后的核对 |
|---|---|---|---|---|---|
| 对比图 GDP.pdf 只放 GPT-5.6 Sol 且数字偏低（`cherry pick … lie about the published third party benchmark score`） | https://news.ycombinator.com/item?id=49984906 | oh_no | 0 | `It only has Sol 5.6 from OpenAI and it shows a 12.8 … AA's GDP.pdf listing has Sol 5.6 Max at a 27, even non-reasoning beats the 12.8.` | 图中确有 `GPT-5.6 Sol 12.8`，与 DeepSeek V4.1 Flash 同为 12.8；AA 页嵌入数据里 GPT-6 Sol (Max) 为 25.2%、GPT-6.1 Sol (Max) 31.0%，**未含** GPT-5.6 Sol，所以 “27” 未能核实；图中该柱与 AA 同类 Sol 值不符这一点**成立**，“Sol 5.6 Max = 27” 这个具体数字**未核实** |
| 价格是促销价 | https://news.ycombinator.com/item?id=49979293 | james2doyle | 1 | `You are quoting the discount pricing. It is 50% off for the next two weeks.` | 成立，见 B1 |
| 与更便宜的开放模型比价（`competing with open source models that are 1/2 - 1/3 the price but with similar capabilities`） | https://news.ycombinator.com/item?id=49978666 | lifeisloving | 1 | 同左 | AA：Cost per task $1.13（Mistral）vs GLM-5.3-Flash $0.25、DeepSeek V4.1 Flash $0.27、MiMo-V2.6-Pro $0.13（AA 文章图）；Vals Index 成本 $13.78 vs GLM-5.3 $7.25（jakozaur 引用，见下，Vals 页 $13.78 已核） |
| Vals Index 与 GLM-5.3 比较 | https://news.ycombinator.com/item?id=49978258 | jakozaur | 1 | `Otherwise, behind on the broader Pareto frontier, but not by much (Vals Index: 48.05% vs. GLM-5.3’s 53.51%; $13.78 vs. $7.25 per test).` | Vals 页：`ranks #32 of 44 models on the Vals Index with 48.05%`，`Cost / Test (Vals Index) $13.78`；GLM-5.3 的 53.51% 与 $7.25 未另查 |
| 知识/幻觉分低（“-5 on omniscience?”） | https://news.ycombinator.com/item?id=49978910 | jascha_eng | 4 | `-5 on omniscience? … That's not particularly great.` | 未核（AA Omniscience 页未查） |
| 推理档位只有 none / high，实测差异小 | https://news.ycombinator.com/item?id=49978763 | simonw | 122（子树） | `Surprisingly it only supports reasoning "none" or reasoning "high". That setting didn't seem to make any real difference - it added a tiny bit of thinking trace and high actually produced less output tokens than none.` | 文档 USAGE 页签示例 `reasoning_effort="high"`；apidog 称取值 `"high"` 或 `"none"`；个人试用体感，样本小 |
| 总体评价“disappointing”（对比表） | https://news.ycombinator.com/item?id=49978488 | SyneRyder | 22（子树） | `But on the Mistral benchmarks I'm seeing, this looks very disappointing` | 主观评价，无数字 |
| Reddit 对“最佳美欧开放权重”的比较集合质疑 | https://www.reddit.com/r/MistralAI/comments/1wz22ti/comment/pe7q06a/ | ClaudeMMM（1006 摘录） | — | `This says nothing... there's basically no open weights model from US or Europe except Mistral Medium 3.5.` | 分数未取得；Mistral X 首帖原话 `best open weights model from US or Europe on aggregated benchmarks` 成立；“美欧开放权重模型很少”属评论者判断 |

HN 上“cherry-pick Sol 5.6”那条之外，另有“六胜六负”之类的统计说法**我没有在 HN 评论里找到原出处**（派工单所述来自扫描回贴）。我方按图读数的统计见本文件第一节第 5 条与 B2 表。

## 三、可能的吠点（原文里有、线索摘要没写的条件与限制）

1. 49B 与 52B：博客与 AA 文章写 49B，文档（现）写 52B；博客页面只有 “1 trillion”，文档写 1.05T。（博客 / 文档 / AA 文章）
2. 价格：公开页多处只显示 $0.68 / $2.09，实为 “Launch pricing: 50% off for 2 weeks”，原价 $1.36 / $4.18；官方无起止日期。（文档悬浮提示）
3. 上下文：文档写 1M，AA 与 Vals.ai 写 512k 档；博客不写。（见 B1）
4. “近零”出自 CyberGym-E2E-AA 一项，且带 AA 脚注 “declined some tasks on safety grounds”；Cyber Index 三项合计 Opus 5.5 为 29%、GPT-6 Astra 为 33%。（AA 文章图）
5. Cybench、Refusal、B3、KORA、DeepSWE、Terminal-Bench、SWE-Atlas、AutomationBench 等图只与开放权重对手比，Claude Opus 5.5 只出现在一张图。（B2 表）
6. 三张 AA 编码图（DeepSWE、Terminal-Bench、SWE-Atlas）的分数是 AA 在其编码智能体指数发布前私下评测的数字，脚注原文见 B2；Beam 标 self-reported。
7. y 轴不从 0 起：DeepSWE 40、Cybench 60、Finance Agent 20、Finch 30、SciCode 65、ChartQA Pro 50、Cyber Index 旧小图 30、Surge 3 等，柱高比例被放大。（B2 表“轴”列）
8. 金融：Mistral 在 Finance Agent v2 低于 GLM-5.3（55.8 vs 54.7），在 Finch 低于 Kimi K3（77.3 vs 67.4）；SciCode-Verified 低于 GLM-5.3、MiMo-V2.6-Pro；而博客写 “state-of-the-art among open models”。
9. 整体智力：AA Intelligence Index 38，在 25 个模型中居中靠后，低于 Opus 5.5（58）、GPT-6 Astra（53）及多款开放权重（MiMo 46、GLM-5.3 45、Kimi K3 44、GLM-5.3-Flash 42）。（AA 文章图）
10. 成本：AA Cost per Intelligence Index task $1.13，AA 文章标题式小结 “Over 4x the Cost per Task of similar-intelligence open weights models”；输出 token 200M，AA 摘要称 very verbose。
11. 红队阶段：博客写合作伙伴与“state authorities”访问的是 “reduced moderation and expanded cyber capabilities” 版本，与公开 API 不同；公开预览版的安全表现与此版本不同。
12. 权重尚未发布：文档 OPEN 徽章与描述“open-weight … model”与 `Weights: Coming soon`；AA 判定 proprietary；Vals.ai 标 open weights。
13. Arena：1534 分置信区间 ±21，名次区间 34–56；与榜单行 `claude-opus-4-8` 差 2 分（置信区间重叠），与 `claude-opus-4-8-high` 差 22 分；Arena 自己写它 “just off the Code Arena: WebDev Pareto frontier”，且 “Nearby but higher-performing models are available for less”。
14. TNW / VentureBeat 的 “10 月 27 日”官方页没有；Mistral 官方只写 “end of this month”。
15. 博客图表发布后被无声替换过（B2 末段）。

## 四、扫描说法勘误

- **49B / 52B**：两个数都存在，且是同一份官方文档页在不同时间的两个版本；博客始终 49B。扫描把 52B 当文档页现况是对的；若只写 “49B” 对应博客，若写 “52B” 对应文档现版。
- **1T / 1.05T**：博客 “1 trillion”，文档 1.05T，AA FAQ “1 trillion”。不冲突，取整口径。
- **1M / 524k**：文档 1M；AA 摘要 524k，AA FAQ 520k，AA 文章 512k，Vals.ai 512k。扫描写“AA 524k（FAQ 520k）”**成立**，另需补 AA 文章 512k；“Vals.ai 与 AA 均记 512K”**部分成立**：Vals.ai 512k 已核（`Context Window 512k`），AA 摘要是 524k、文章是 512k、FAQ 是 520k，不是单一 512K。
- **折扣价**：扫描写 “划线价，疑为预览期折扣” **成立**，官方悬浮提示 `Launch pricing: 50% off for 2 weeks.`，AA 文章同写 “first two weeks”。
- **Arena 1534 / #45**：已核，**成立**；扫描称 “比 Large 3（第130）高 304 分” 是 Arena 帖原话，榜单页现抓名次为 133（Large 3）与 125（Medium 3.5），帖中写 #130 与 #122。AIHOT “10/7 14:42” 与帖子 ID 推算时刻 14:18:23（北京）不同，以 ID 推算时刻为准。
- **HN “cherry pick Sol 5.6”**：评论存在（oh_no，评论 49984906）；图上 `GPT-5.6 Sol 12.8` 属实；评论所称 “AA 上 Sol 5.6 Max 为 27” 我无法在 AA 页嵌入数据里核到，**未核实**。
- **“六胜六负”**：我没有找到原出处；我方读图统计：单独最高 6、并列最高 3、非最高 10（见第一节与 B2 表）。统计口径 = 图中所示柱里 Mistral 是否最高，**我方读图，推断**。
- **HN 主帖 1948 分 / 1164 评论**：我方 10/8 01:47（北京）抓取为 **1970 分 / 1173 评论**（Algolia）；扫描数字为更早时刻快照，不冲突。
- **“Weights drop end of this month”**：成立。扫描没提到 TNW/VentureBeat 的 “10 月 27 日” 为媒体加的具体日期，官方页无。
- **Opus 5.5 / GPT-6 Astra “near zero on the same test”**：原句成立；“the same test” 指 AA 的 CyberGym-E2E-AA（82% 那项），不是紧邻的 Cybench。扫描摘要若写成 “Cybench 上 Opus 5.5 近零” 则是说反了。

## 五、未取得 / 失败 / 限制

- Reddit 评论分数、Reddit 当前票数：匿名不可得（403 / 登录页）。
- X：没有读到 Mistral 首帖全文与互动数字（oEmbed 截断；X 内容未登录不可读；twitter-cli 因 Cookie 读取失败，opencli 扩展未连接）。Arena 帖卡片用 X 官方嵌入端点取得。
- VentureBeat 原文：curl 被 Vercel 安全检查拦截（429），仅有搜索引擎摘录；中文媒体与 Decrypt、Apidog、SiliconANGLE 等同样只有搜索摘录或 aggregator 文字，未做页面存档，除非表内另注。
- 博客对比图中 B3 以外的原图已全部下载（25 张），但“旧版图”只取回了 5 张（SWE-Atlas、SciCode 旧图 CDN 已 404，未取回）。
- 截图中无法一张覆盖全部 19 张对比图；截图配额内只出了 Cybench 图与 AA Cyber Index 图，其余图以 `sources/b-assets/` 原图 + 本文 B2 表读数交付。
- Arena 帖卡片在 “Show more” 折叠处，完整正文来自 aggregator 转录（二手）。
- 窄屏下 Arena 榜单不显示票数列，票数取自行文字。
