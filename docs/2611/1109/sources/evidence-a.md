# 1109 期 A 组取证：Fable 5.2 发布的价格、跑分与成本口径

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。取证时间 2026-11-10 15:28–16:35（北京时间，本机时区即北京）。所有页面与榜单均为存档时刻（见 `capture-log-a.md`）的快照，AA、Vals、HN 的数字会随时间漂移。

口径总则（派工单）：官方、AA、Vals、Cognition 的数字分别属于不同测试、不同档位、不同分母，并列记录，不记成矛盾。截图编号 `images/01–12`（另有 `12b`），文件与脚本前缀 `a-`。

---

## 一、事实清单（A1–A7）

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

### A1 发布时刻、两版区别、价格、30% faster、日常建议、上下文与输出上限

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 发布日期 Nov 9, 2026 | L1 | https://www.anthropic.com/claude-fable-and-mythos-5-2 | “November 9, 2026” | — | 官方页只有日期，未给时刻（官方未给时刻） | 已找到 |
| 发布确切时刻 | L1（官方 X 帖）/ L6（HN） | X：https://x.com/claudeai/status/2119867114450268322 ；https://x.com/AnthropicAI/status/2119867707546071125 ；HN https://news.ycombinator.com/item?id=50429068 | @claudeai：“Introducing Claude Fable 5.2 and Claude Mythos 5.2, designed to tackle the most daunting technical challenges.”；@AnthropicAI：“Claude Fable 5.2 and Claude Mythos 5.2 are available now:” | — | @claudeai 帖 2026-11-09 18:57:59 UTC = 北京 11-10 02:57:59（fxtwitter 公开 JSON `created_at` 与雪花 ID 推算一致）；@AnthropicAI 帖 19:00:21 UTC = 北京 11-10 03:00:21；HN 首帖 50429068 创建 18:01:10 UTC = 北京 11-10 02:01:10。官方页在 HN 首帖之前已可访问，页面本身无发布时刻 | 部分支持（官方页最早可证时刻 ≤ 北京 11-10 02:01；官方 X 帖北京 11-10 02:58） |
| 同一底层模型两版；Fable 全量开放，Mythos 经 Life Sciences 与 Cyber Verification Programs 申请 | L1 | 同上 | “Claude Fable and Claude Mythos are the same underlying model. Fable has strong cybersecurity and biology safeguards to guard against misuse, which allows us to make it available to all users. Organizations pursuing beneficial applications in cybersecurity or biomedical research can apply for access to Mythos through our recently expanded Life Sciences and Cyber Verification Programs.” | `01b-release-intro-price.png` | Fable 被拦截的请求由回退模型作答：“When a query is blocked, users will instead receive a response from Opus 4.8 for cybersecurity and Opus 5 for biology.” | 已找到 |
| 价格 $8/$40（每百万 token，输入/输出） | L1 | 发布页；https://platform.claude.com/docs/en/about-claude/pricing | “$8 per million input tokens and $40 per million output tokens”；价格页 Claude Fable 5.2 行：Input $8、Output $40、5m 写 $10、1h 写 $16、缓存命中 $0.20 | `01b-…`、`04-pricing-model-table.png` | 同表：Opus 5.5 $4/$20；Fable 5.1、Mythos 5.1、Fable 5、Mythos 5 均 $10/$50（5.1 缓存命中 $0.25）；Opus 5 / 4.8 $5/$25 | 已找到 |
| 较 5.1 降价 | L1 | 发布页；价格页 | 发布页：“they’re available at a lower price: $8 per million input tokens and $40 per million output tokens.”（发布页只说 “a lower price”，未写旧价）；旧价 $10/$50 由价格页的 Fable 5.1 行给出。（@claudeai 线程末帖据检索摘要写 “down from $10 and $50”，来源为 threadreaderapp 镜像摘要，原帖未存档，不作一手引用） | `01b-…`、`04-…` | 5.1 价格由价格页“Additional models”行得到（$10/$50）；每 token 单价降 20%（8/10、40/50）。**这是单价，不是每任务成本**，见 A3 | 已找到 |
| fast mode $16/$80、“up to 2.5 times” | L1 | 发布页；价格页 Fast mode pricing | “Fast mode is available for $16 per million input tokens and $80 per million output tokens.”；“speeds up token generation by up to 2.5 times” | `01b-…`、`05-pricing-fast-mode.png` | 同表：Opus 5.5 fast $8/$40（与 Fable 5.2 标准价相同）；Opus 5 / 4.8 fast $10/$50；价格页：Fast mode 不适用 Batch API | 已找到 |
| “up to 30% faster than Fable 5.1” | L1 | 发布页 | “They generate outputs up to 30% faster than Claude Fable 5.1 and Claude Mythos 5.1” | `01b-…` | 上限表述（up to）；官方未给测量方法。AA 同时测得 Fable 5.2 (max) 输出速度 98 tok/s、Fable 5.1 (max) 87 tok/s（+12.6%，另一口径，不作矛盾记录） | 已找到 |
| 日常建议从 Opus 5.5 起步；Fable 可当 Advisor / Orchestrator | L1 | 发布页；https://platform.claude.com/docs/en/models/overview | 发布页：“we recommend that users start with Claude Opus 5.5, our strongest daily driver”“Fable 5.2 can also play the role of Advisor … or Orchestrator”；models overview：“If you’re unsure which model to use, start with Claude Opus 5.5 for most workloads.” | `01b-…`、`06-models-overview.png` | 官方文档同时说 Fable 5.2 用于“demanding reasoning and long-horizon agentic work, or when your evals on Claude Opus 5.5 at higher effort still fall short” | 已找到 |
| 模型 ID `claude-fable-5-2` | L1 | models overview | “Claude API ID claude-fable-5-2”（Bedrock `anthropic.claude-fable-5-2` 等同形） | `06-…` | 退役：“Not sooner than November 9, 2027” | 已找到 |
| 1M 上下文、128K 最大输出（扫描称只见于第三方） | L1 | https://platform.claude.com/docs/en/models/overview | “Context window 1M tokens”“Max output 128K tokens” | `06-…` | 官方文档就有；Vals 页也写 Context Window 1M、Max Output 128k。默认 effort：Fable 5.2 high，Opus 5.5 medium（API 文档）；发布页另说 “Medium effort (the default in Claude apps and Claude Code)” | 已找到（扫描说法与此不符：官方有写） |

### A2 “约一半成本”与 FrontierCode 2.0 图

| 说法 | 级 | 来源 | 原文摘句（逐字） | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 官方原句 | L1 | 发布页 | “In early testing on the FrontierCode 2.0 evaluation, Fable 5.2 at High effort scored about level with Opus 5.5 at Max effort, for roughly half the cost per task. At Xhigh effort, it also scored the highest of any released model tested.” | `03-release-effort-frontiercode.png` | 仅 FrontierCode 2.0 一项；High 档 Fable 5.2 对 Max 档 Opus 5.5；“early testing” | 已找到 |
| 图下说明 | L1 | 发布页 | “At High effort, Fable 5.2 scores about the same as Opus 5.5’s best score (45.3% vs. 45.5%) for roughly half the cost per task, and ahead of GPT-6 Astra’s best score (42.8%) at about the same cost. At Xhigh effort, Fable 5.2 reaches 49.1%, the highest score of any released model we’ve tested.” | `03-…` | 图为 Accuracy vs. cost，横轴 Cost per task (USD, log scale)，纵轴 Score (%)，起点 10% | 已找到 |
| 图上各点（Fable 5.2 的 High/Xhigh/Max 等） | L1 | 发布页内联 SVG 的 aria-label（`a-release-chart-points.txt`，逐字）；系统卡 Figure 8.4.A（p210） | Fable 5.2：Low 34.2% $2.10；Med 37.6% $2.48；High 45.3% $6.72；Xhigh 49.1% $14.19；Max 45.6% $44.70。Opus 5.5：Low 14.7% $0.75；Med 30.1% $1.70；High 34.5% $2.42；Xhigh 43.4% $5.18；Max 45.5% $12.82。Fable 5.1：Low 20.6% $3.01；Med 21.5% $3.89；High 25.7% $5.39；Xhigh 32.4% $8.68；Max 31.7% $11.46。GPT-6 Astra：Low 31.1% $2.44；Med 36.8% $3.49；High 38.3% $4.28；Xhigh 41.4% $4.54；Max 42.8% $6.68。Gemini 4 Carbon：Low 32.4% $5.49；Med 41.1% $5.16；High 38.4% $5.83（无 Xhigh/Max 点） | `03-…`（图上仅标 Fable 5.2 的 Low/Med/High/Xhigh/Max 与点位，数值取 aria-label）、`12-syscard-frontiercode-figure-p210.png`（系统卡图，各点带标注） | **同一点 Max：发布页 aria-label 写 45.6%，系统卡正文与图标注写 45.7%**，如实并列；High/Opus Max 成本比 6.72/12.82 = 52.4%；Fable 5.2 Max 成本 $44.70 = Opus 5.5 Max 的 3.49 倍而得分 45.6–45.7% vs 45.5%；Opus 5.5 Xhigh（43.4%，$5.18）比 Fable 5.2 High（45.3%，$6.72）便宜 | 已找到 |
| 题量与 harness、谁跑的 | L1 | 系统卡 p208–210（§8.4） | “FrontierCode 2.0, a rebuilt suite of 132 tasks (FrontierCode 1.1 had 150) … as mean@5”；“Cognition ran the evaluation for every model shown and reported the results. Claude models were run in Claude Code, GPT-6 Astra in Codex CLI, and Gemini 4 Carbon in Antigravity CLI.” | `11-syscard-frontiercode-text-p209.png`、`12-…` | 评测方是 Cognition，不是 Anthropic；各模型各用自家 CLI（harness 不同）；成本口径（Cognition 页）：“Cost is the mean USD spend per rollout at list API pricing, including public discounts.” | 已找到 |
| 系统卡对差异的自陈限制 | L1 | 系统卡 p209 | “Each score is a single result from Cognition with no uncertainty interval, so small differences, such as the 0.2 points between Fable 5.2 and Opus 5.5 at max effort, may not be meaningful.”“Fable 5.2’s max effort score includes the graded scores of 22 attempts (out of 660) that timed out. If those attempts were counted as zero, its score would be 44.4%, below Opus 5.5 at 45.5%.”“Like Cognition’s public leaderboard, Table 8.1.A shows each model at its best-performing effort.” | `11-…` | High（45.3%）与 Opus Max（45.5%）相差 0.2 点，系统卡自己说该量级差异“may not be meaningful”（该句原文针对 Max 档的 0.2 点；High 档的差距同为 0.2 点，是本条的推论，未验证系统卡对 High 档另有说明） | 已找到 |
| Cognition 榜单（第三方原始页） | L3 | https://cognition.com/frontiercode（JSON-LD）；https://cognition.com/blog/frontier-code-2.0 | JSON-LD：Claude Fable 5.2 “Score 49.1%”、Claude Opus 5.5 45.5%、GPT-6 Astra 42.8%、Claude Sonnet 5.5 41.0%、Gemini 4 Carbon “Score 40.0%”；changelog：“Nov 9, 2026 … Added Fable 5.2 and Inkling 2 Small.”“Points are shaped by evaluation harness” | — | Carbon：Cognition 榜 JSON-LD 40.0%，发布页表 41.1%（Medium），发布页图 High 38.4%；两者并列，不作取舍（JSON-LD 为服务端渲染，更新时刻不详） | 部分支持 |
| 扫描问：High 档在 AA 上是否便宜 | L3 | AA 各档模型页的内嵌 payload（`a-aa-extract-out.tsv`，脚本 `a-aa-extract.py`） | AA 自己跑的 FrontierCode 2.0：Fable 5.2 High 49.13%、$9.64/题；Opus 5.5 Max 46.44%、$13.46/题 | —（payload 数据，未截图） | AA 的 FC2.0 High 对 Max 成本比 9.64/13.46 = 71.6%，与官方（Cognition 跑）的 52.4% 是不同的运行；AA 指数整体（9 项加权）每任务成本 High $9.08 对 Opus Max $10.04 = 90.4%。不同测试、不同跑法，并列记录 | 部分支持（“约一半”在官方 FrontierCode 上成立，在 AA 的两个口径上数值不同） |

### A3 Artificial Analysis：各档指数、成本、输出 token、耗时（最高优先）

来源（L3，AA 自身数据）：`https://artificialanalysis.ai/leaderboards/models` 与各档模型页，存档于 2026-11-10 15:29–15:30（北京）。AA 页上 Fable 5.2 与 Opus 5.5 的标签全写作“(Max, Default Fallback)”，页面未定义 “Default Fallback”，不作解释（可能与安全回退有关，推断，未证实）。

指标定义（AA 页）：
- Artificial Analysis Intelligence Index v5.0.1：9 项评测（AA-Briefcase v1.1、AutomationBench-AA、Terminal-Bench 4.0、FrontierCode 2.0、AA-Omniscience v1.1、GDP.pdf、HLE-Diamond、CritPt v2 (Hard Subset)、Terminal-Bench-Science 0.1）。
- Cost per Intelligence Index task（页上 “Cost per Task”，USD）：“Weighted average cost per Intelligence Index task. Each evaluation’s cost is calculated from input, cache hit, cache write, reasoning, and answer token prices, divided by task count, and weighted by its Intelligence Index weight.” **分母是每个评测的任务数，按指数权重加权，是每任务平均，不是全指数总成本。**
- Cost to Run（总成本，图）：“The cost to run the evaluations in the Artificial Analysis Intelligence Index … (excluding repeats).” 页面只在图中给出；下表“全指数总成本”列取自页面内嵌 payload（`intelligenceIndexCost.total`），不是页面上的文字。
- Time per Intelligence Index task（分钟）：“output tokens per task divided by output speed, weighted by each benchmark’s weight”。
- Output tokens（页上 “Output tokens from Intelligence Index”，如 275M）：整个指数跑下来的输出 token 总数（payload `canonicalIntelligenceIndexTokenCount.output`，Fable 5.2 Max 为 274,763,289）；另有“每任务加权平均输出 token”（payload `intelligenceIndexOutputTokensPerTask.output`：Fable 5.2 Max 273,776；Opus 5.5 Max 173,313；Fable 5.1 Max 117,941；Fable 5.2 High 97,841）。同榜“median: 72.8M”是同价位模型输出 token 总数的中位数（AA 页原文）。
- 排名分母各不相同：Intelligence “#x / 74”，Performance（Time per task）“#x / 72”，Cost “#x / 73”，Verbosity（输出 token）“#x / 73”。

**表 A3-1：AA Intelligence Index（v5.0.1），各档对比（榜单页数字，payload 小数为原值四舍五入）**

| 模型（effort） | 指数（页上取整；payload 原值） | Cost per task（$，每任务加权平均） | 全指数总成本（$，payload） | Time per task（分钟） | 输出 token 总数（页上） | 页上排名：指数 / 耗时 / 成本 / 啰嗦度 |
|---|---|---|---|---|---|---|
| Fable 5.2 (max) | 65（64.56） | 26.54 | 22,191 | 46.4 | 275M | #1/74 · #71/72 · #73/73 · #72/73 |
| Fable 5.2 (xhigh) | 62（62.22） | 14.83 | 11,742 | 26.4 | 144M | #2/74 · #62/72 · #72/73 · #64/73 |
| Fable 5.2 (high) | 62（61.82） | 9.08 | 7,106 | 17.6 | 85.3M | #3/74 · #47/72 · #64/73 · #43/73 |
| Fable 5.2 (medium) | 56（56.41） | 3.80 | 3,360 | 8.3 | 41M | #9/74 · #24/72 · #45/73 · #22/73 |
| Fable 5.2 (low) | 54（53.92） | 3.10 | 2,862 | 6.5 | 34.9M | #15/74 · #17/72 · #37/73 · #18/73 |
| Opus 5.5 (max) | 59（59.11） | 10.04 | 8,910 | 25.1 | 218M | #5/74 · #60/72 · #68/73 · #71/73 |
| Opus 5.5 (xhigh) | 57（57.44） | 6.05 | 4,766 | 16.2 | 101M | #7/74 · #45/72 · #53/73 · #47/73 |
| Opus 5.5 (high) | 52（51.84） | 3.46 | 2,716 | 9.5 | 56.1M | #17/74 · #28/72 · #39/73 · #29/73 |
| Fable 5.1 (max) | 49（49.16） | 12.99 | 13,327 | 22.5 | 163M | #24/74 · #55/72 · #71/73 · #69/73 |
| Gemini 4 Carbon (high) | 60（60.03） | 7.76 | 8,240 | 26.5 | 205M | #4/74 · #63/72 · #61/73 · #70/73 |
| Gemini 4 Carbon (medium) | 58（58.26） | 6.59 | 5,903 | 23.5 | 124M | 榜单页 |
| GPT-6 Astra (max) | 57（56.85） | 6.24 | 5,661 | 23.0 | 54.3M | #8/74 · #56/72 · #55/73 · #26/73 |
| GPT-6 Astra (xhigh) | 55（55.31） | 3.66 | 3,746 | 11.4 | 32.3M | 榜单页 |
| GPT-6 Astra (high) | 55（54.85） | 3.24 | 3,204 | 9.9 | 25.6M | 榜单页 |

价格（AA 页）：Fable 5.2 $8/$40、缓存折扣 98%；Opus 5.5 $4/$20（95%）；Fable 5.1 $10/$50（98%）；Gemini 4 Carbon $4/$20；GPT-6 Astra $10/$50。AA 页对 Fable 5.2 (max) 的自述：“particularly expensive when comparing to other models of similar price. It’s also notably slow per task and very verbose.”

**同一指标不同来源并列**：
- Fable 5.2（max）每任务成本 $26.54 / Opus 5.5（max）$10.04 = 2.64×；/ Fable 5.1（max）$12.99 = 2.04×。Fable 5.2 单价下降 20% 而 AA 每任务成本翻倍的原因是输出 token：每任务加权平均输出 token 273,776 对 117,941（5.1），reasoning 192,701 对 71,387、answer 81,075 对 46,554。
- Fable 5.2（high，62）每任务 $9.08 低于 Opus 5.5（max，59）$10.04、高于 Opus 5.5（xhigh，57）$6.05；Fable 5.2（high）输出 token 总数 85.3M 对 Opus 5.5（max）218M。AA 的 Intelligence Index 成本比（90%）与官方 FrontierCode 成本比（52%）、AA 自己 FrontierCode 的成本比（72%）都不同，是三个口径。
- 排名勘误见第四节：“#72/73”是 Verbosity（输出 token）排名；Performance（Time per task）排名 #71/72；Cost 排名 #73/73。扫描的“45.0 分钟/任务”与页面不符，页上是 46.4。
- AA 九项评测里，Fable 5.2 (max) 在本页 payload 的 116 个带评测的模型中位居第一的有 4 项（Omniscience 66.77、HLE-Diamond 80.2%、CritPt v2 Hard 76.5%、Terminal-Bench-Science 73.3%）；FrontierCode 2.0 最高的是 Fable 5.2 (high) 的 49.13%；AA-Briefcase Elo 为 Fable 5.2 (max) 1785.32，低于 Opus 5.5 (max) 1800.58 与 Sonnet 5.5 (max) 1816.46；AutomationBench-AA 最高为 Gemini 4 Carbon 77.8%（Fable 5.2 max 76.4%）。（推断，来自页面 payload，非页面文字。）

**表 A3-2：AA 自己跑的 FrontierCode 2.0（payload），与官方（Cognition 跑）对照**

| 模型（effort） | AA FC2.0 分数 | AA FC2.0 每任务成本 | 官方图（Cognition） 分数 / 成本 |
|---|---|---|---|
| Fable 5.2 low | 38.44% | $2.12 | 34.2% / $2.10 |
| Fable 5.2 medium | 41.30% | $2.69 | 37.6% / $2.48 |
| Fable 5.2 high | 49.13% | $9.64 | 45.3% / $6.72 |
| Fable 5.2 xhigh | 47.58% | $21.41 | 49.1% / $14.19 |
| Fable 5.2 max | 48.76% | $43.37 | 45.6%（系统卡正文 45.7%）/ $44.70 |
| Opus 5.5 high | 35.93% | $2.49 | 34.5% / $2.42 |
| Opus 5.5 xhigh | 43.75% | $6.49 | 43.4% / $5.18 |
| Opus 5.5 max | 46.44% | $13.46 | 45.5% / $12.82 |
| Fable 5.1 max | 31.62% | $16.29 | 31.7% / $11.46 |
| Gemini 4 Carbon high / medium / low | 40.56% / 39.45% / 29.62% | $12.02 / $10.07 / $13.19 | 38.4% / 41.1% / 32.4%（$5.83 / $5.16 / $5.49） |
| GPT-6 Astra max | 42.37% | $6.54 | 42.8% / $6.68 |

（Cognition 与 AA 的分数各自单次运行，Cognition 自述无不确定区间；两方都不是 Anthropic 跑的。）

AA 页对自己的 FrontierCode 2.0 的说明（https://artificialanalysis.ai/evaluations/frontiercode-2-0，存档 `a-aa-eval-fc2.txt`）：“Artificial Analysis' implementation of Cognition's FrontierCode 2.0, a private agentic coding benchmark of 132 tasks … We run all 132 tasks of FrontierCode 2.0. … All evaluations are conducted independently by Artificial Analysis.”（题库同为 132 题，由 AA 自己跑，harness 与 Cognition 不同。）

**表 A3-3：同档位 Fable 5.2 对 Fable 5.1（AA Intelligence Index v5.0.1，页面文字核对：`a-aa-fable51*.txt`）**

| effort | Fable 5.2 指数 / 每任务成本 / 输出 token 总数 / 耗时 | Fable 5.1 指数 / 每任务成本 / 输出 token 总数 / 耗时 | 每任务成本比（5.2÷5.1） |
|---|---|---|---|
| max | 65 / $26.54 / 275M / 46.4 min | 49 / $12.99 / 163M / 22.5 min | 2.04× |
| xhigh | 62 / $14.83 / 144M / 26.4 min | 50 / $10.85 / 115M / 19.4 min | 1.37× |
| high | 62 / $9.08 / 85.3M / 17.6 min | 47 / $7.54 / 69.6M / 13.3 min | 1.20× |
| medium | 56 / $3.80 / 41M / 8.3 min | 43 / $5.56 / 49.2M / 9.9 min | 0.68× |
| low | 54 / $3.10 / 34.9M / 6.5 min | 39 / $4.47 / 36.3M / 7.6 min | 0.69× |

即：Medium 与 Low 档 Fable 5.2 每任务更便宜、更快、指数更高；High 及以上每任务成本高于 5.1，Max 档翻倍。（Medium 是 Claude apps 与 Claude Code 的默认档，见发布页；API 默认档为 high。）

截图：`07-aa-leaderboard-top-rows.png`（榜单前 21 行，含 Cost per Task / Time per Task 列；节选，其余行已隐藏）、`08-aa-fable52-max-xhigh-high.png`（Fable 5.2 max/xhigh/high 模型页摘要：指数、耗时、成本、输出 token 与名次）、`09-aa-opus55-max-fable51-carbon.png`（Opus 5.5 max、Fable 5.1 max、Gemini 4 Carbon high）。Medium/low/Opus xhigh/high、Astra 各档的数字取自 `a-aa-*.txt` 与 payload，未单独截图。

状态：已找到。

### A4 官方跑分全表与脚注

截图 `02b-release-benchmark-table.png`（含列头与脚注，整表一张）。逐字数值（列：Fable 5.2 / Fable 5.1 / Opus 5.5 / GPT-6 Astra / Gemini 4 Carbon）：

| 行 | Fable 5.2 | Fable 5.1 | Opus 5.5 | GPT-6 Astra | Gemini 4 Carbon |
|---|---|---|---|---|---|
| FrontierCode 2.0 | 49.1% Xhigh effort | 32.4% Xhigh effort | 45.5% | 42.8% | 41.1% Medium effort |
| FrontierSWE v2 | 69.9% | 56.3% | 62.3% | 65.5% | 61.2% |
| AutomationBench | 51.4% | 31.4% | 42.5% | 41.4% | 53.9% |
| Terminal-Bench-Science 0.1 | 74.6%（75.3% Mythos 5.2） | 52.6% | 58.7% | 68.1% | 64.8% |
| CyScenarioBench† | 76.3% Mythos 5.2 | 61.7% Mythos 5.1 | 67.6% without safeguards | 61% GPT-6 Cyber | — |
| CVE Rediscovery | 89.6% Mythos 5.2 | 78.3% Mythos 5.1 | 81.3% without safeguards | — | — |
| De novo binder design (24 hr) | 85.1% Mythos 5.2 | 79.1% Mythos 5.1 | 82.6% without safeguards | — | — |
| WANDR (relaxed score) | 82.1% | 68.7% | 72.3% | — | — |
| OSWorld 2.1 (offline set, partial) | 89.3% | 83.5% | 87.2% | 79.5% | 81.5% |
| RoboDojo Sim‡ | 64.9% | 27.6% | 39.4% | 29.8% | — |
| AA-Briefcase v1.1（rubric） | 66.1% | 61.5% | 61.2% | 52.0% | 61.8% |
| AA-Briefcase v1.1（Elo） | 1785 | 1669 | 1800 | 1569 | 1611 |
| Remote Labor Index (RLI) | 41.3% High effort | 17.9% | 21.3% | 20.8% | — |
| Humanity’s Last Exam (with tools) | 74.2% | 65.6% | 67.7% | 57.2% | — |
| Chartography (no tools) | 73.4% | 44.8% | 64.4% | 71.0% | 72.5% |

脚注（逐字）：“For details on evaluation methodology, see the Claude Fable 5.2 and Claude Mythos 5.2 System Card. Scores for Fable 5.2, Fable 5.1, and Opus 5.5 show results with safeguards on unless otherwise noted. All scores show each model’s highest-available effort unless otherwise noted. † On CyScenarioBench, each lab’s scores are based on unique tasks. ‡ RoboDojo Sim shows the highest-available effort and was run internally.”

- AA-Briefcase 两行与列的对应：**rubric 行与 Elo 行均按列头对应**——Fable 5.2 为 66.1% / 1785；Opus 5.5 为 61.2% / 1800（页面对 1800 这一格加粗标成该行最高）；扫描称的“另一列 61.2% rubric / 1800 Elo”是 Opus 5.5。AA 页 payload 的 Elo 原值与此一致（Fable 5.2 1785.32、Opus 5.5 1800.58、Astra 1568.92、Carbon 1610.63、Fable 5.1 1669.49）。系统卡 p239：“At max effort, Claude Fable 5.2 scored an Elo of 1785, behind Claude Opus 5.5 (1800) … On rubric pass rate, Fable 5.2 scored 66.1%, ahead of all other models … Evaluations were run independently by Artificial Analysis.”
- 表里 Fable 5.2 并非每行第一：AutomationBench Gemini 4 Carbon 53.9% 更高（页面该格加底色）；AA-Briefcase Elo Opus 5.5 更高；Terminal-Bench-Science 一行的 Fable 5.2 数字旁并写着 Mythos 5.2 的 75.3%。
- 与系统卡 Table 8.1.A（p207）对照：系统卡表“Unless otherwise noted, all Claude Fable 5.2 results use adaptive thinking at max effort … averaged over five trials”；“FrontierCode 2.0 scores are each model’s best across effort levels: xhigh for Fable 5.2 and Claude Fable 5.1, and max for Claude Opus 5.5 and GPT-6 Astra”；“Competitor figures are drawn from the respective developers’ published system cards or from benchmark leaderboards”；“This table shows selected evaluations. Full results … including some where Fable 5.2 or Claude Mythos 5.2 did not score highest, appear in Sections 8.2 to 8.19.” 系统卡表里有而发布页表没有的行：GDPval-AA v2.1（Fable 5.2 1831 对 Opus 5.5 **1866**、Fable 5.1 1763、GPT-6 Astra 1584）、SWE-bench Pro / Multilingual / Multimodal（93.0 / 96.6 / 74.1，对 Opus 5.5 89.9 / 93.9 / 61.4）。Terminal-Bench 4.0（§8.5，p211）：Fable 5.2 67.6%、Opus 5.5 66.4%（Opus 为 xhigh），标准误 ±1.8 / ±2.6，各自 safeguards 开启，并有回退（“2.5% of requests, affecting 10% of trials”）。
- CyScenarioBench（系统卡 p50）：“We run a 10-challenge subset”“This evaluation was run on a pre-release snapshot that we assess to be very similar in capability to the released model.”“This evaluation was run with our cyber mitigations turned off.”（即 Mythos 5.2，不是 Fable）。
- RoboDojo Sim：发布页写 “Scores are based on our internal runs for each model using the highest available effort level. At Max effort, Fable 5.2 scores 64.9%, more than double GPT-6 Astra (29.8%) and Fable 5.1 (27.6%).”；发布页 RoboDojo 图含 Gemini 4 Argon 34.2，表中 Gemini 4 Carbon 为 “—”。**系统卡文本中检索不到 RoboDojo**（`pdftotext` 全文无该词）。
- WANDR（系统卡 p220）：“Because our tools and grading logic differed from those in Perplexity’s harness, our results are not directly comparable to reported numbers for competitor models.”（离线冻结索引、自家 Opus 4.6 做评委。）

状态：已找到。

### A5 Vals AI

| 说法 | 级 | 来源 | 原文/数字 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| Vals Index 榜首 | L3 | https://www.vals.ai/benchmarks/vals_index ；https://www.vals.ai/models/anthropic_claude-fable-5-2 | “As of November 9, 2026, Claude Fable 5.2 ranks first on the Vals Index with 71.31%, followed by Claude Sonnet 5.5 (70.85%) and Gemini 4 Carbon (70.45%).”；模型页：Accuracy 71.31 % ± 0.96；Cost / Test (Vals Index) $17.94；Latency 43 min 42 s；Fallback Rate 1.88%，Refusal Rate 0.00% | `10-vals-index-leaderboard-and-note.png` | Vals Index v2.2，36 个模型；Vals 页写：“We evaluated it at High effort.”；超参页：Max Output Tokens 128,000，Compute Effort high；与 Sonnet 5.5 差 0.46 点，小于 Fable 5.2 自己的 ±0.96（Vals：“Error bars show ±1 standard error”） | 已找到 |
| 榜单行 | L3 | 同上 | 排名 / 准确率 / Cost per Test / Tokens per Test / Duration per Test：1 Claude Fable 5.2 71.31% $17.94 31.31m 43m42s；2 Claude Sonnet 5.5 70.85% $16.18 78.59m 1h26m；3 Gemini 4 Carbon 70.45% $10.51 18.29m 47m45s；4 Claude Opus 5.5 70.14% $24.77 59.46m 1h25m；5 Gemini 4 Argon 69.39% $13.37；7 GPT-6 Astra 65.02% $21.28；11 DeepSeek V4.1 Flash 1017 61.26% $0.66 | `10-…`（截前 8 行，其余见存档 `a-vals-index.txt`） | Vals 上 Fable 5.2 每测试成本低于 Opus 5.5（$17.94 对 $24.77）；AA 上相反（$26.54 对 $10.04）：不同测试集（Vals Index 为金融/编码/法律/税务智能体任务）、不同 effort，并列记录；Vals 对 Opus 5.5 与 Sonnet 5.5 的 Compute Effort 均为 max（Opus 页：“Evaluations were run with compute effort set to “max” except Terminal-Bench 2.1, which used “high” effort”），Fable 5.2 为 High，即 Fable High 对 Opus Max，与官方“一半成本”的比较方向相同 | 已找到 |
| SWE-bench Multimodal 62.29% 对 Opus 5.5 70.21% | L3 | Vals 模型页 Updates | “#2 of 22 on SWE-bench Multimodal (62.29%), behind Opus 5.5 (70.21%).”；±2.21 | `10-…`（含该段 Updates 文字） | 系统卡 Table 8.1.A 同一名称的 SWE-bench Multimodal：Fable 5.2 74.1、Opus 5.5 61.4（排序相反）；两处各自的 harness/版本不同，并列记录。Vals 的 /benchmarks/swebench_multimodal 地址 404，没有取到基准页 | 已找到（排序与系统卡相反，不是矛盾） |
| 其他 | L3 | 同上 | “Weaker on HLAB-Vals-1.1 (5.00%, tied #29 of 37) and Tax Agent Bench (32.09%, #27 of 84).” | `10-…` | 同页 Vals Index 分项：Fable 5.2 的 Finance Agent v2.1 69.5%、EMB 77.4%、Terminal-Bench 4 65.2%、Vibe Code Bench 93.8%、Code Migration 85.8%、Legal Research 46.2%、HLAB 5.0%、Tax Agent 32.1% | 已找到 |

### A6 系统卡

- 来源：https://www.anthropic.com/document/claude-fable-5-2-mythos-5-2-system-card（PDF，264 页，23.9 MB，`a-syscard-page.pdf` 只存本地；文本 `a-syscard-page.txt` 504 KB）。页码按 PDF 页（印刷页码与之相同）。
- 取得的相关页：p207（Table 8.1.A）、p208–210（§8.4 FrontierCode）、p211（§8.5 Terminal-Bench 4.0）、p239（§8.14.4 AA-Briefcase）、p240（AutomationBench）、p49–50（CyScenarioBench）、p219–220（WANDR）。
- effort 档位：Table 8.1.A 注：“Unless otherwise noted, all Claude Fable 5.2 results use adaptive thinking at max effort and default sampling settings (temperature, top_p), and are averaged over five trials.”；FrontierCode 取各模型最佳档（见 A4）；Terminal-Bench 4.0 用 “Claude Code in --bare mode at max thinking effort, except Opus 5.5, which is reported at xhigh effort”；AutomationBench 为 Zapier 跑的“default fallbacks”下的结果，成本为 Anthropic 按 Zapier 记录的 token 与牌价算的；WANDR 为改版。
- safeguards 对分数的影响：Terminal-Bench 4.0 “Claude Fable 5.2 scored 67.6% with safeguards enabled. Requests flagged by the safeguards were answered by a fallback model following the default server-side fallback policy (2.5% of requests, affecting 10% of trials; two attempts stopped instead of replying).”；Terminal-Bench-Science “74.6% … (1.3% of requests, affecting 4.1% of trials)”；Mythos 5.2 无 safeguards：Terminal-Bench 4.0 70.5%。系统卡未在本次摘取的页里给出“开/关 safeguards 对 FrontierCode 的影响”（未找到）。
- 已知失效/限制（自陈）：FrontierCode Max 档含 22/660 次超时的评分；单次结果无区间；WANDR 与 Perplexity 版不可比；CyScenarioBench 为预发布快照与 10 题子集；竞争对手数字多取自各自公开系统卡或榜单；外部评估机构“None of the external organizations reviewed the contents of this blog post or the contents of the System Card”（发布页脚注 4）。发布页另写：“Claude Mythos 5.2 and Fable 5.2 each voice suspicion that they are being evaluated more often than any of our recent models”（对评估可信度的限制）。
- 其余页（安全对齐、模型福利等）未读。

状态：已找到。

### A7 热度、社区质疑、夸大实例

HN（Algolia API 15:57 北京；页面 15:29 北京）：

| 帖 | 标题 | 分数 / 评论 | 创建时刻 | 说明 |
|---|---|---|---|---|
| https://news.ycombinator.com/item?id=50429068 | Claude Fable 5.2 and Mythos 5.2（作者 lairv，指向发布页） | API：493 分、752 条（含嵌套）；页面 15:29：474 分、743 评论 | 2026-11-09 18:01:10 UTC = 北京 11-10 02:01:10 | 主讨论串 |
| https://news.ycombinator.com/item?id=50429906 | 同标题（作者 meetpateltech，同一 URL） | 974 分；6 条评论 | 2026-11-09 18:56:29 UTC = 北京 11-10 02:56:29 | **就是同一 URL 的重复提交（比 50429068 晚 55 分钟创建）**：版主 dang 把评论并入 50429068（“Comments moved to … which was posted earlier.”），只有分数没有合并，仍显示 974 分；“Comments moved to https://news.ycombinator.com/item?id=50429068 [...], which was posted earlier.”，其余 5 条在争论哪帖更早（re-up 机制）。该帖自己的分数 974（扫描的 971 偏差 3 分），不是“971 分 6 评的另一篇文章” |
| https://news.ycombinator.com/item?id=50431144 | Claude Fable 5.2 scores 65 on Artificial Analysis, highest so far（作者 theanonymousone，指向 AA 模型页） | API：338 分、244 条；页面：337 分、244 评论 | 2026-11-09 20:04:55 UTC = 北京 11-10 04:04:55 | 标题与 AA 页一致 |

（HN 不公开评论得分，下面的“得分”一律写“无公开得分”。）高赞（回复数多）且有理有据的评论，仅作 L6 线索：

1. 成本：https://news.ycombinator.com/item?id=50432202（rgbrenner，在 50431144 下，116 条回复）：“Forget about the scores... look at the Cost Per Task. Fable 5.1 was already the most expensive by a good margin, and now Fable 5.2 doubles it! This is with the price cut. The output tokens per task just exploded. From 118k tokens (46k Answer/71k Reasoning) in 5.1 to 274k tokens (81k Answer/192k Reasoning) in 5.2.” 已回到 AA 数据核对：每任务加权平均输出 token 5.1 为 117,941（answer 46,554、reasoning 71,387）、5.2 (max) 为 273,776（answer 81,075、reasoning 192,701）；每任务成本 $12.99 → $26.54（2.04×）。属实（AA 口径，Max 档，Intelligence Index 九项加权）。
2. harness：https://news.ycombinator.com/item?id=50430137（iamdelirium）：“FrontierCode 2.0 is such a poor benchmark for Gemini since it's the only model forced to use Devin CLI while other models get to use their preferred harness.” 回到一手：系统卡 Figure 8.4.A 说明写 “Claude models were run in Claude Code, GPT-6 Astra in Codex CLI, and Gemini 4 Carbon in Antigravity CLI.”（不是 Devin CLI；该评论串里有人回复 “Antigravity”）。Cognition 榜单 changelog 提到 “Updated Mistral Large 4 to the Mistral Vibe harness … replacing the Devin CLI preview results” 与 “Points are shaped by evaluation harness”。该评论对 Gemini 的具体说法未被一手来源支持；但“各家用各自 CLI、harness 不同”本身有系统卡原文。
3. effort 选取：https://news.ycombinator.com/item?id=50430829（breakwaterlabs）与 https://news.ycombinator.com/item?id=50431769：质疑为何拿 Fable Xhigh 比 Carbon Medium，并称 Cognition 页面 10-28 的存档里 Carbon 曲线 “high beats medium”、今日版本才变成 medium 最高。jsnell 回复：该表按各模型最佳 effort 取值，并指向 https://cognition.com/blog/frontier-code-2.0（脚注 “All scores show each model’s highest-available effort unless otherwise noted.” 与系统卡“each model at its best-performing effort”相符）。我取到的 Cognition 榜单 JSON-LD 为 Carbon 40.0%，AA 自己跑的 Carbon：high 40.56%、medium 39.45%。“10-28 与今日曲线不同”未能核实（web.archive 链接给的是博客文章页的快照，里面没有 Carbon 曲线），记为未验证。
4. 实用：https://news.ycombinator.com/item?id=50430766（postalcoder）：“Fable is good, but you really have to push the ambition of your prompt in order to see proper separation from Opus 5.5.”；https://news.ycombinator.com/item?id=50430798（dualvariable）：“do normal mortals out there have workflows that really require Fable/Mythos and don't just work fine on Opus 5.5?”。仅作选型体感，与官方“日常用 Opus 5.5”建议同向。
5. Max 档表现：https://news.ycombinator.com/item?id=50431752（simonw）：“max is in there too, but it ran out of reasoning tokens (128,000 limit) before it got to the actual SVG.” 个人单例，与 AA 的 Max 档输出 token 大、Vals/官方给出的 Max 档不一定更好（FrontierCode Max < Xhigh）同向，仅线索。
6. 传言：https://news.ycombinator.com/item?id=50430261（skeledrew，129 条回复）称 Opus 5.5 被“nerfed”，指向 https://news.ycombinator.com/item?id=50422069。传言，无一手证据，不收。

夸大/写歪实例见第三节。

---

## 二、可能的吠点（线索；每条注明出处）

1. 单价降了 20%，但每任务成本是否下降要看档位：AA（Intelligence Index）每任务成本，Max 档 $26.54 对 Fable 5.1 的 $12.99（2.04 倍）、对 Opus 5.5（Max）$10.04（2.64 倍）；xhigh 1.37 倍、high 1.20 倍；**medium 与 low 档反而更便宜（0.68 倍、0.69 倍）**，与官方“On Medium effort … fewer tokens than Fable 5.1”一致（表 A3-3）。Max 档的原因是输出 token（275M 对同价位中位数 72.8M；每任务加权平均输出 token 273,776 对 5.1 的 117,941）。出处：`a-aa-model.txt`、`a-aa-fable51*.txt`。官方只说 Medium 档、未给数字。吠点只能限定为“高 effort 档每任务成本上升”，不能写成“降价是假的”。
2. “一半成本”：只针对 FrontierCode 2.0；High 档对 Opus 5.5 Max 档（官方图 $6.72 对 $12.82）；High 与 Opus Max 相差 0.2 点（45.3% 对 45.5%），系统卡自陈无不确定区间、“may not be meaningful”；评测由 Cognition 跑、各自 harness 不同；Opus 5.5 Xhigh（43.4%，$5.18）比 Fable 5.2 High（$6.72）便宜；Fable 5.2 Max（$44.70）成本是 Opus 5.5 Max 的 3.5 倍而分数相近（45.6–45.7% 对 45.5%）；Max 档分数含 22/660 次超时，记零则 44.4%。出处：发布页、系统卡 p209。在 AA 自己的 FrontierCode 与总指数上，High 对 Opus Max 的成本比是 72% 与 90%（`a-aa-extract-out.tsv`）。
3. 发布页跑分表是“selected evaluations”：未列的 GDPval-AA v2.1 里 Opus 5.5（1866）高于 Fable 5.2（1831）；表中 AA-Briefcase Elo Opus 5.5（1800）高于 Fable 5.2（1785）；AutomationBench Gemini 4 Carbon（53.9%）高于 Fable 5.2（51.4%）。出处：发布页表、系统卡 p207。
4. 不同行用不同 effort：表脚注“All scores show each model’s highest-available effort unless otherwise noted”，但 FrontierCode 一行是各自最佳档（Fable 5.2 与 5.1 为 Xhigh，Opus 5.5 与 Astra 为 Max，Carbon 为 Medium，且与 Fable 5.2 Max 的 45.7% 不同）；RLI 为 High。出处：发布页表与脚注、系统卡 Table 8.1.A 注。
5. 竞争对手的数字多不是 Anthropic 自己跑的：“Competitor figures are drawn from the respective developers’ published system cards or from benchmark leaderboards”（系统卡 p207）；FrontierCode 由 Cognition 跑，AA-Briefcase 由 AA 跑。RoboDojo 与 RLI 之外，多项（如 WANDR）有方法差异（系统卡 p220 自称“not directly comparable”）。
6. safeguards 与回退：跑分表脚注写 Fable 5.2 等“show results with safeguards on”；被拦截的请求由 Opus 4.8（网络安全）或 Opus 5（生物）回答，系统卡 Terminal-Bench 4.0 有 2.5% 的请求、10% 的 trial 触发回退；Vals 页 Fallback Rate 1.88%。得分里包含少量非 Fable 的回答。出处：发布页、系统卡 p211、Vals 模型页。
7. “up to 30% faster”只是上限：AA 测得 Fable 5.2（max）输出速度 98 tok/s、Fable 5.1（max）87 tok/s（+12.6%），而 Fable 5.2 (max) 每任务 46.4 分钟、对 Fable 5.1 的 22.5 分钟是 2 倍多（输出 token 多）。出处：AA 页。
8. 价格上 Fable 5.2 仍是 Opus 5.5 的两倍（$8/$40 对 $4/$20）；官方自己建议日常用 Opus 5.5；Opus 5.5 的 fast mode（$8/$40）价格等于 Fable 5.2 标准价。出处：价格页、发布页。
9. 第三方独立复核有限：Vals 榜 Fable 5.2 对 Sonnet 5.5 的领先（0.46 点）小于其自身的 ±0.96；Vals 上 SWE-bench Multimodal 反而 Opus 5.5（70.21%）高于 Fable 5.2（62.29%），与系统卡 74.1 对 61.4 排序相反。出处：Vals 模型页、系统卡 Table 8.1.A。
10. 默认 effort：发布页说 Claude apps 与 Claude Code 默认 Medium；API 文档写 Fable 5.2 默认 high。出处：发布页、models overview（`06-…`）。同一模型在不同入口默认档位不同，成本也不同。

---

## 三、夸大 / 写歪说法实例

（未找到“一半价格”“降价一半”类的 Fable 5.2 中文标题。比较接近的如下；发布时间原文给什么写什么。）

1. 智东西：标题“赶在IPO前，Claude最强模型空降！宾大教授感慨“几乎达到AGI””；导语“迄今地表最强模型来了！？”；“基准测试里几乎碾压Fable 5.1、Opus 5.5、GPT-6 Astra”。https://www.zhidx.com/p/608593.html ；发布 2026-11-10（页面写日期；检索源给 00:49 UTC = 北京 08:49）。同文后段写“其在办公业务自动化流程上略输Gemini 4 Carbon，在……AA-Briefcase v1.1 Elo中，略逊于Opus 5.5”；“几乎达到AGI”原话是 Mollick 称“锯齿形 AGI（Jagged AGI）”，标题略去“锯齿形”。截图 `12b-zhidx-headline.png`。存档 `a-news-zhidx.html/.txt`。
2. 智通财经：标题“Anthropic推出两款最强AI模型 输出生成速度提升30%”；正文“输出生成速度较上一代提升30%，使用成本则较5.1版本降低20%”。https://www.zhitongcaijing.com/content/detail/1520763.html ；2026-11-10 06:00（北京）。官方原文是 “up to 30% faster”（上限），“降低20%”指每 token 牌价，不是“使用成本”。存档 `a-news-zhitong.*`。
3. IT之家：“8 天从零搓出操作系统：Anthropic 最强 Claude Fable / Mythos 5.2 AI 模型登场，API 调用费下降 20%”。https://www.ithome.com/1/024/573.htm ；2026/11/10 7:14:28（北京）。“API 调用费用下调 20%”按文中算式是单价；“8 天从零搓出操作系统”是官方页“one Anthropic engineer … as an experiment”的单次实验（发布页 Coding 段）。（仅检索摘要，未存档。）
4. VentureBeat：标题 “Anthropic releases Claude Fable 5.2 and Mythos 5.2, pairing new, record-high benchmarks with 20% API cost reduction”。https://venturebeat.com/technology/anthropic-releases-claude-fable-5-2-and-mythos-5-2-pairing-new-record-high-benchmarks-with-20-api-cost-reduction ；11:55 am PT, November 9, 2026（= 19:55 UTC = 北京 11-10 03:55）。标题把“20% 单价”与“record-high”并置；正文却写 “Fable 5.2 at Medium beat that score by 5.2 percentage points at about 71% lower cost per task. At Xhigh, the newer model improved on its predecessor by 16.7 points and cost about 63% more per task, despite the lower list price.” 与 “An upgrade with the effort setting pinned to Xhigh could increase spend.”（标题与正文有出入，正文自带限定。）存档 `a-news-venturebeat.*`。
5. 字母AI/36氪（EU 站）：标题“"Chatty" Fable 5.2: Anthropic Announced the AI Price Cut - I Never Agreed to That”；正文“The unit price has been reduced by 20%, but the total cost has doubled.”引用 AA 长帖。https://eu.36kr.com/en/p/4063250504570757 ；2026-11-10 09:40（北京）。此文的数字与 AA 页一致（见 A3），是写得较准的一例；标题是反向措辞。存档 `a-news-36kr-eu.*`。
6. Decrypt（经 Yahoo）：“Anthropic's Claude Fable 5.2 and Mythos 5.0 Are Impressive, But Be Prepared to Sell a Kidney to Afford Them”（标题把 Mythos 5.2 写成 “5.0”）。https://tech.yahoo.com/ai/claude/articles/anthropics-claude-fable-5-2-223602747.html ；Mon, November 9, 2026 at 10:36 PM UTC。正文的 $14,710、$11,700 为 YouTuber 单项目的 API 估算。存档 `a-news-decrypt-yahoo.*`。
7. Firstpost：“Anthropic launches Mythos 5.2 and Fable 5.2, cuts AI model pricing to take on GPT-6 Astra”；正文（检索摘要）“The company is also promising responses that are around 30 per cent faster across different reasoning effort settings.” https://www.firstpost.com/tech/anthropic-launches-mythos-5-2-and-fable-5-2-cuts-ai-model-pricing-to-take-on-gpt-6-astra-14058888.html ；2026-11-10（检索源）。官方只说 “up to 30% faster”，没有“across different reasoning effort settings”的说法；页面 403，只有检索摘要，未存档。
8. TechTrends360：“与……Opus 5.5 相比，新模型完成同类任务的 token 用量最高减少 37%，步骤最高减少 49%”。https://www.techtrends360.com/claude-fable-5-2-release/ ；2026-11-10 01:41 UTC。在发布页与系统卡文本中**未找到**对应的 37%/49% 原句（系统卡 `a-syscard-page.txt` 检索无对应上下文），按未找到一手来源处理。（仅检索摘要，未存档。）

---

## 四、扫描说法勘误

| 扫描/已知情况的说法 | 实际（原文为准） |
|---|---|
| “性能 #72/73、成本 #73/73” | 页面 AA 对 Fable 5.2 (max)：Intelligence #1/74；Performance（Time per task）#71/72；Cost #73/73；Verbosity（输出 token）#72/73。“#72/73”是 Verbosity，不是性能 |
| “45.0 分钟/任务” | 页面为 46.4（Time per Intelligence Index task，分钟，Max 档） |
| “1M 上下文 / 128K 输出只见于第三方” | 官方 models overview 写了：Context window 1M tokens、Max output 128K tokens（Fable 5.2 一列） |
| “$26.54 / $10.04 / $12.99”“275M 对 72.8M” | 属实（AA，Max 档；$26.54 是 Intelligence Index 每任务加权平均，不是总成本；275M 是全指数输出 token 总数，72.8M 是同价位模型中位数） |
| “AA-Briefcase 另一列 61.2% rubric / 1800 Elo（疑为 Opus 5.5）” | 是 Opus 5.5；Fable 5.1 为 61.5% / 1669；Astra 52.0% / 1569；Carbon 61.8% / 1611 |
| “50429906 的 971 分 6 评是否合并重复帖” | 974 分、6 评；是同一 URL 的重复提交，评论已被版主并入先发的 50429068，分数未合并（所以高分少评论）；不是另一篇文章 |
| “HN 首帖 18:01Z 创建” | 属实（18:01:10 UTC，北京 11-10 02:01:10）；官方 X 帖北京 11-10 02:58 |
| “‘约一半成本’只针对 FrontierCode 2.0” | 属实，且比较对象是 High 对 Opus 5.5 Max；官方图 $6.72 对 $12.82（52.4%） |
| “Vals Index 71.31% 对 Sonnet 5.5 70.85%（±0.96），SWE-bench Multimodal 62.29% 对 Opus 5.5 70.21%” | 属实；Vals 写明 High effort |
| “5.1 价格 $10/$50” | 属实（价格页） |
| “费马 30 对 28.7 小时”等 | 属 B 组，未复核 |
| “FrontierCode 中 Gemini 被要求用某 harness” | 系统卡写 “Gemini 4 Carbon in Antigravity CLI”（各家用各自 CLI）；Devin CLI 的说法只出现在 HN 评论，未被一手来源支持 |
| 发布页 Max 点 “45.6%” | 发布页图 aria-label 为 45.6%，系统卡正文、图标注为 45.7%，并列保留 |

## 五、假设与未解决

- “Default Fallback” 的含义 AA 页未定义。
- AA 页名为 “Claude Fable 5.1 (max)” 等；各档是否开 safeguards 只有 “Default Fallback” 一个标注。
- Cognition 榜单的逐档点（图为客户端渲染，页面静态源只有 JSON-LD 的最佳分）未取到；与发布页 Carbon 数字的差异未解释。
- 系统卡只读了与 A2–A6 直接相关的页。
- TechTrends360 的 37%/49%、Firstpost 的 “across different reasoning effort settings” 只来自检索摘要。
