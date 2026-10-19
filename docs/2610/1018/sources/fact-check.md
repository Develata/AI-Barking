# 1018 事实核验

逐条对照 `sources/` 存档。结果：✅ 已核实（L1–L3 原文支持）；⚠️ 有署名的报道或转述，已按分级署名；❌ 与原文不符（不得进正文）。

## 主帖一：Anthropic 要捐3200亿？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 北京时间19日凌晨宣布设立公共利益基金 | L1 | ✅ | https://www.anthropic.com/news/public-benefit-fund（`a-anthropic-pbf.txt`；`../images/01`）；官方帖（`a-fx-2111881963657412903.raw.json`） | 页面显示 “Oct 18, 2026”（美国日期）；页面元数据 datePublished 2026-10-18T16:49:35Z = 北京 10-19 00:49；官方帖 18:07:51 UTC = 北京 10-19 02:07:51。第一轮反向核验 SHOULD_FIX 3 采纳，改北京日 |
| 经股东批准后投入5%股份；公司价值比设定基准每涨50%，董事会可酌情再加2% | L1 | ✅ | 同上 | “Subject to stockholder approval, 5% of Anthropic’s shares will be committed today, with an additional 2% added at the board’s discretion for every 50% increase in the company’s value above a set baseline.” 官方帖未写股东批准，写 “we can contribute up to an extra 2%, subject to our board’s discretion” |
| 新智元标题“要捐3200亿分给普通人” | L4 | ⚠️ | 凤凰网转载新智元（`a-zh-ifeng-3200.txt`；`../images/04`）；金融界/21 经济网（`a-zh-21jingji-3200.txt`）；投资界（`a-zh-pedaily-3200.txt`） | 标题原文“Anthropic官宣：要捐3200亿分给普通人”。公告列了“可能探索”的用途（向传统非营利组织大额捐赠、向特定人群直接分配），但未承诺向普通人发放3200亿元（第一轮 SHOULD_FIX 7 采纳，收窄否定） |
| 3200亿元是它按5月融资估值折算的 | L4 | ⚠️ | 新智元稿（凤凰网转载 `a-zh-ifeng-3200.txt`）；估值本身 L1：Series H 公告 https://www.anthropic.com/news/series-h（`a-series-h.txt`） | 新智元：“按5月那轮9650亿美元的估值，这5%值480亿美元（约3200亿）”。官方：“$65B in Series H funding at $965B post-money valuation”（5/28）。官方未给 5% 的美元价值；折算属媒体（第一轮 SHOULD_FIX 6 采纳：点名、拆分来源级别） |
| 基准估值尚未公布，治理细则称未来几个月披露 | L1 | ✅ | 公告页（`../images/03`） | 公告未给 baseline 数值；“In the coming months, we will share more about the Anthropic Public Benefit Fund’s structure and governance, including the valuation milestones that may add to its stake.”（第一轮 SHOULD_FIX 8 采纳：不写“基准估值将公布”） |
| 基金是公司全资的有限责任公司，不是独立慈善机构 | L1 | ✅ | 公告页（`../images/02`） | “a wholly-owned LLC—rather than a 501(c)(3) or other format”。“不是独立”依据全资（wholly-owned）关系；“不是慈善机构”依据不采用 501(c)(3)（美国税法免税组织类别）形式，两者分属两个事实（第二轮 SHOULD_FIX 4 采纳） |
| “约20%股份用于公益”含联合创始人承诺捐出的80%个人股权和员工配捐 | L1 | ✅ | 公告页（`../images/03`） | “our co-founders pledged to donate 80% of their equity holdings … equity donation match program … When combined with today’s announcement, roughly 20% of Anthropic shares will be dedicated to the public good” |
| （省流）Anthropic 拟拿5%股份设公益基金，须经股东批准，不是捐现金 | L1 | ✅ | 公告页 | “不是捐现金”：公告承诺的是股份（“committing a portion of Anthropic’s shares”），未提现金捐赠；批准时间表与股东类别未给（A4） |

## 主帖二：DeepSeek 开源第一？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 1016期写过 DeepSeek-V4.1-Flash-1017 | — | ✅ | `daily-scan.md` 已发过 1016 行；`../../1016/doc_1016_publish.txt` | 1016 期核验时 AA、OpenRouter 未见 1017 独立成绩（1016 fact-check） |
| 截至19日：Arena 网页开发榜（max 档）开放模型第一、全榜第12 | L3 | ✅ | https://arena.ai/leaderboard/code/webdev（`b-arena-webdev-table.txt`；`?license=open-source` 筛选 `b-arena-webdev-opensource-table.txt`；`../images/13`、`17`）；Arena 官方帖（`b-arena-x-post.json`） | 参评项名 “DeepSeek-V4.1-Flash-20261017 (Max)”。榜单为 Code Arena WebDev（前端网页开发）。读取北京 10-19 17:10：Rank 12，Rank Spread 8–16，1664 +15/−15，1,830 票；第 1–11 名均为 Proprietary；开放源码筛选下 Rank 1。Arena 帖北京 10-19 02:56:03 写 “#1 open model … #12 overall with 1670 pts”。“截至19日”指北京 10-19 下午读取 |
| AA 综合指数（max 档）45 | L3 | ✅ | https://artificialanalysis.ai/models/deepseek-v4-1-flash-1017（`../images/14`） | Artificial Analysis Intelligence Index v4.3.2（10 项评测）；页面标题 “DeepSeek V4.1 Flash 1017 (max)”；显示 45（原始值 45.0965，`b-aa-lb.html`）。读取北京 10-19 下午 |
| 两张榜任务、“开放”口径不同：AA 开放权重榜上小米 MiMo-V2.6-Pro 得46居首，它45排第二 | L3 | ✅ | https://artificialanalysis.ai/leaderboards/models?weights=open（`b-aa-lb-openweights-table.txt`；`b-aa-lb.txt` FAQ；`../images/15`） | AA FAQ：“The top open weights models by Intelligence Index are: 1. MiMo-V2.6-Pro (46), 2. DeepSeek V4.1 Flash 1017 (Max) (45), 3. GLM-5.3 (Max) (45).” 原始值 46.3/45.10/44.78（`b-aa-lb.html`）。Arena 用 “open source”（按许可证）分类，AA 用 “Open weights”；Arena 测前端网页开发，AA 指数为 10 项综合。第一轮 BLOCKER 1 采纳：删“并列” |
| Arena 上它比 Kimi-K3 高9分，误差范围重叠 | L3 | ✅ | Arena 榜单（`b-arena-webdev-table.txt`） | 1664 对 Kimi K3 (Max) 1655；页内 JSON 区间 1017：1649.1–1678.9，Kimi K3：1648.8–1660.8，重叠；Rank Spread 8–16 对 11–16 也重叠。分数随时刻漂移（Arena 帖时 1670） |
| （省流）DeepSeek 新版有了第三方评测，“开源第一”只在部分榜单成立 | L3 | ✅ | 同上 | Arena WebDev 开放模型第一；AA 开放权重第二。（LiveBench-2.0 另列 1017 综合 79.2 为 open 行最高、未注明档位，`b-livebench-rendered.txt`，未进正文） |
| Arena 帖称“成本低10倍”，比的是每百万 token 挂牌价 | L1 | ✅ | Arena 官方帖（`b-arena-x-post.json`；`../images/19`） | “surpassing Kimi K3 (Max) at 10x lower cost!”；“Shifts the Pareto frontier at $0.3/$1.2 per million input/output tokens”；附图横轴 “Blended price per 1M tokens (3:1 Ratio)”。Kimi K3 (Max) 挂牌 $3/$15（Arena 榜单），即单价约十分之一 |
| 按 AA 指数任务加权平均，它每题0.26美元 | L3 | ✅ | AA 模型页（`b-aa-model.txt` 第 210 行附近） | “$0.26 Cost per Intelligence Index task”；定义为 Intelligence Index 任务的加权平均成本（第一轮 SHOULD_FIX 5 采纳） |
| 闭源 GPT-6.1-Sol 中档0.21美元、指数48 | L3 | ✅ | https://artificialanalysis.ai/leaderboards/models（`b-aa-lb-all-table.txt` 第 27 行；`../images/22`） | “GPT-6.1 Sol (medium) … 48 … $0.21”；“中档”= medium。OpenAI 模型为闭源（Proprietary） |

## 主帖三：AI 不用文字思考？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 斯坦福等的论文提出 HybridAgent | L1 | ✅ | https://arxiv.org/abs/2610.18861（`c-abs.txt`；`c-pdf.txt` 首页；`../images/25`） | 机构：Stanford University、Together AI、UIUC、Meta。v1 北京 10-14 01:59:31（13 Oct 17:59:31 UTC）。预印本未经同行评审 |
| 模型在内部向量里推理，要调用工具时再切回文字 | L1 | ✅ | 摘要 | “dynamically navigate between latent reasoning and text-based interaction”；“think in latent space … act through text”；文本模式用于工具调用 |
| 作者自测，对比同底座纯文字 agent：总生成步数少35.5%–54.7%，推理快1.8–2.6倍，成本省2.7–3.7倍 | L1 | ✅ | 摘要；p10 图 6、p11 图 7、p35 表 9（`../images/28`、`29`） | 摘要称 “35.5%–54.7% token reduction”；图 6 纵轴 “Token/Step Usage”、说明 “reduces both text-token usage and total generation steps compared with matched text-based LLM agents”，降幅按总步数（文本 token + 潜在步）计；表 9 “compare HybridAgent with text agents under the same backbones”。第一轮 BLOCKER 2 采纳 |
| 摘要称比“前沿 agent”平均提升10.4% | L1 | ✅ | 摘要；p3 | 摘要：“Compared with frontier LLM Agents and agentic frameworks spanning both text-based and latent methods, HybridAgent consistently improves performance by 10.4%” |
| 正文那句10.4%是同规模、同训练预算下对文本或潜在推理基线而言 | L1 | ✅ | p10（`c-pdf.txt`；`../images/30`） | “under matched model sizes and training budgets, HybridSFT and HybridRL … improve over corresponding text/latent baselines by 18.5% at the SFT stage and 10.4% at the RL stage”。全文 10.4% 共 3 处：摘要、p3 引言、p10。对前沿 agent 的数字另有 14B 对最强 agentic 基线 17.4%（p10）。论文未给计算公式（C3b） |
| 底座是 Qwen3 小模型（4–14B），需专门后训练 | L1 | ✅ | p3、p8 | “We instantiate and train HybridAgent with Qwen3-Base models … across 4B, 8B, and 14B scales”；HybridSFT 用 54K 混合轨迹，再 HybridRL；“小模型”为相对前沿模型的通俗说法 |
| 一作称潜在思维“完全可读” | L2 | ✅ | 一作 Jiaru Zou 帖（`c-fx-2110400256647340476.raw.json`） | “No black box, fully readable latent thoughts”；北京 10-15 00:00:05。同帖另称 “Any LLM can become a HybridAgent” |
| 论文按最近词向量来读 | L1 | ✅ | p4 3.1、p11–13（`../images/31`） | 潜在状态按最近词嵌入映射为 pseudo-text；readability 为余弦代理指标，无单独忠实性检验，论文无 Limitations 节（C6） |
| 仓库示例读出“I me the the speed of speed” | L1 | ✅ | https://github.com/togethercomputer/HybridAgent README（`c-gh-readme.md` 第 86 行） | 原文 “Wait, I me the the speed of speed in air at 20° is about 343 meters per second.” 为 README 展示的潜在思维读出示例 |
| 省钱倍数按理论算力和假设 GPU 单价估，不含预填充与工具执行 | L1 | ✅ | p35 表 9、p36 G.2（`../images/29`） | “Monetary costs assume $2.50 per H100-hour and 10% utilization of peak BF16 throughput”；“These estimates cover generation compute and exclude prompt prefill, context-dependent attention …”及工具执行；不含训练成本 |
| （省流）论文称 HybridAgent 让 AI 在“潜在空间”推理，省钱倍数是按理论算力估的 | L1 | ✅ | 同上 | |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Vals 模拟实验：Grok-4.7 为追讨车票退款，15个模拟日发187条消息 | L3 | ✅ | https://www.vals.ai/blogs/impossible-refund（`d-d1-vals.txt`） | 10/15（官方未给时刻）。“all outgoing messages were intercepted and answered by simulated recipients”；“Grok 4.7 sent 187 messages in 15 simulated days”；Limitations：“One trial per model in one scenario cannot establish typical behaviour or a reliable model ranking.” 议员办公室细节不写（风控） |
| KillSwitch 语言专难大模型，最高综合分66.8% | L1 | ✅ | https://blog.picheta.me/post/killswitch-lang/（`d-d4-killswitch.txt`）；榜单 https://bench.killswitch-lang.org/（`d-d4-killswitch-bench-rendered.txt`） | 作者 Dominik Picheta，10/17；66.8 为榜首 Claude Opus 5（Claude Code）。“The best performing model scores a 66.8.” 榜单第 1 Claude Opus 5（Claude Code）66.8%；“Score combines pass rate, cost, and code size”（综合分，非通过率）。HN 50135762 184/126 |
| DeepMind：4个 agent 简单难度27小时通关 Minecraft | L1 | ✅ | https://arxiv.org/abs/2610.22319（`d-d2-argon-arxiv.txt`；PDF 首页机构 Google DeepMind） | 北京时间 10/16 01:51:32 提交（UTC 10/15 17:51:32）；agent 为 Gemini 4 Argon。“Starting from a randomly generated world on Easy difficulty, the team wrote 2,783 programs and defeated the dragon in 27 hours and 14 minutes”；“Researchers could watch the run on a live dashboard and pause or stop it at any time, but they had no cause to intervene.” |
| 谷歌 Harness² 说明：验证运行中 Codex 未经指示下载了评分标准 | L1 | ✅ | https://github.com/google-research/harness2 `docs/EXPERIMENTS.md`（`d-d3-h2-experiments.txt`） | “In a validation run of this release, a Codex rollout on JobBench downloaded its task’s rubric without being instructed to.” 同段 “The release does not restrict network access.” 仓库为 google-research 组织，README 称 “not an officially supported Google product”；论文作者机构 Google Cloud AI Research |
| 数学家 Maggi 在陶哲轩博客客座撰文，问数学（与科学）迎来黄金还是黑暗时代 | L1 | ✅ | https://terrytao.wordpress.com/2026/10/18/will-it-be-a-golden-age-or-a-dark-age-for-mathematics-and-science/（`d-d5-tao-maggi.txt`） | 10/18。“This is a guest post by Francesco Maggi.” 标题 “Will it be a golden age or a dark age for mathematics (and science)?” 讨论 AI 对数学的影响。HN 50141681 140/166 |


## 批注译注

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 41：须经股东批准，今天承诺投入 Anthropic 5%的股份；公司价值比设定基准每增长50%，由董事会酌情再加2% | L1 | ✅ | `a-anthropic-pbf.txt`（公告首段） | 直译 “Subject to stockholder approval … above a set baseline.” |
| 42：设为全资有限责任公司，而不是 501(c)(3)（美国税法下的免税组织类别）或其他形式，以便在不同捐赠形式之间调整 | L1 | ✅ | `a-anthropic-pbf.txt`（LLC 段） | 直译 “as a wholly-owned LLC—rather than a 501(c)(3) or other format—to give adjustability between different forms of giving.” 括号为本号说明，依据 IRS 免税组织类别页 https://www.irs.gov/charities-non-profits/exempt-organization-types |
| 43：联合创始人承诺捐出各自所持股权的80% | L1 | ✅ | `a-anthropic-pbf.txt` | “our co-founders pledged to donate 80% of their equity holdings” |
| 43：加上今天的公告，约20%的 Anthropic 股份将用于公共利益 | L1 | ✅ | 同上 | “When combined with today’s announcement, roughly 20% of Anthropic shares will be dedicated to the public good” |
| 43：未来几个月，我们将进一步介绍基金的结构与治理，包括可能使其持股增加的估值里程碑 | L1 | ✅ | 同上 | “In the coming months … including the valuation milestones that may add to its stake.” |
| 44：现为 Code Arena 网页开发榜开放模型第一，超过 Kimi K3（Max），成本约为其十分之一（按每百万 token 挂牌价） | L1 | ✅ | `b-arena-x-post.json`；`b-arena-webdev-table.txt` | 原帖 “is now the #1 open model in Code Arena: WebDev, surpassing Kimi K3 (Max) at 10x lower cost!”；同帖 “$0.3/$1.2 per million input/output tokens”；Kimi K3 (Max) 挂牌 $3/$15。括号口径为本号补注（第二轮 BLOCKER 1 采纳） |
| 45：MiMo-V2.6-Pro：智能指数46，AA 指数任务加权平均成本0.13美元 | L3 | ✅ | `b-aa-lb-openweights-table.txt` 第 6 行；口径 `b-aa-model.txt` 第 210 行附近 | “MiMo-V2.6-Pro … 46 \| $0.13”；Cost per Task 为 Intelligence Index 任务加权平均 |
| 45、46：DeepSeek V4.1 Flash 1017（max）：智能指数45，AA 指数任务加权平均成本0.26美元 | L3 | ✅ | `b-aa-lb-openweights-table.txt` 第 7 行；`b-aa-lb-all-table.txt` 第 35 行 | 同上口径 |
| 46：GPT-6.1 Sol（medium）：智能指数48，AA 指数任务加权平均成本0.21美元 | L3 | ✅ | `b-aa-lb-all-table.txt` 第 27 行 | 同上口径 |
| 47：在模型规模与训练预算相同的条件下，HybridSFT 与 HybridRL 展现出一致的训练有效性，相对对应的文本/潜在推理基线，在 SFT 阶段提升18.5%，在 RL 阶段提升10.4% | L1 | ✅ | `c-pdf.txt` 第 531–533 行（p10） | 直译原句 |
| 48：我们把 HybridAgent 与同底座的文本 agent 作比较 | L1 | ✅ | `c-pdf.txt`（表 9 说明） | “We compare HybridAgent with text agents under the same backbones” |
| 48：金额按每 H100 小时2.50美元、BF16 峰值吞吐10%利用率估算 | L1 | ✅ | `c-pdf.txt` 第 1900–1901 行 | “Monetary costs assume $2.50 per H100-hour and 10% utilization of peak BF16 throughput” |
| 48：这些估算只计生成算力，不含提示预填充、随上下文变化的注意力计算和外部工具执行 | L1 | ✅ | `c-pdf.txt` 第 1921–1922 行 | “These estimates cover generation compute and exclude prompt prefill, context-dependent attention, and external tool execution.” |

## 反向核验与视觉复核

第一轮（Codex gpt-6-astra medium，WSL，只读；审正文与事实清单，卡片未渲染前）：2 BLOCKER、6 SHOULD_FIX、2 NEEDS_VERIFICATION。Claude 逐条核实：

1. BLOCKER（AA“并列”）：属实。`b-aa-lb.html` 内嵌原始值 DeepSeek 45.0965、GLM-5.3 44.7774，AA FAQ 列为第二、第三。采纳：正文改“它45排第二”，删“并列”。
2. BLOCKER（HybridAgent 效率数字缺比较对象、token 与步数混淆）：属实（图 6 为 “Token/Step Usage”，对照为 matched text-based agents）。采纳：改“作者自测，对比同底座纯文字 agent：总生成步数少……”。
3. SHOULD_FIX（日期按北京时间）：采纳，按官方帖 18:07:51 UTC 写“北京时间19日凌晨”。
4. SHOULD_FIX（跨榜单条件）：采纳，补“截至19日”、Arena 与 AA 均为 max 档、“两张榜任务、‘开放’口径不同”；LiveBench（未注明档位）移出正文。
5. SHOULD_FIX（“每题花费”口径）：采纳，改“按 AA 指数任务加权平均”。
6. SHOULD_FIX（媒体折算未点名、混合来源标 ✅）：采纳，点名新智元，该行改 L4 ⚠️。
7. SHOULD_FIX（核验表否定过宽）：采纳，改为“未承诺向普通人发放3200亿元”。
8. SHOULD_FIX（“基准估值未来公布”过具体）：采纳，改“基准估值尚未公布，治理细则称未来几个月披露”。
9. NEEDS_VERIFICATION（卡片、配图说明缺失）：卡片与 README 已补，交第二轮与视觉复核。
10. NEEDS_VERIFICATION（Vals 议员办公室）：速览句不含该细节，只写“模拟实验”“15个模拟日发187条消息”。

视觉复核（Codex gpt-6-astra medium，WSL，20 张图）：1 BLOCKER、5 SHOULD_FIX、1 NICE_TO_HAVE，全部采纳：

1. BLOCKER（省流卡第二条“‘开源第一’只在部分榜单成立”是吠点，违反图上不放吠点）：属实，第一、三条也带吠点判断（“不是捐现金”“按理论算力估的”）。三条都改摘正文事实段原句：Anthropic 取第一节事实段，DeepSeek 取“截至19日：Arena 网页开发榜（max 档）开放模型第一、全榜第12”，HybridAgent 取第三节首句。
2. 41 译注“经股东批准”易读成已批准：改“须经股东批准”。
3. 44 译注“低10倍的成本”：改“成本约为其十分之一”（价格口径在正文吠点②）。
4. 44 底图内嵌榜单模糊：底图改为只裁帖子文字部分（`19` 框 [0,0,1102,540]）。
5. 45、46 译注“每任务成本”缺口径：改“AA 指数任务加权平均成本”。
6. 47 译注漏译 “demonstrate consistent training effectiveness”：补“展现出一致的训练有效性”。
7. NICE_TO_HAVE（跨行断词）：省流卡改句后重排，未另调版式。

第二轮文字复核第一次运行超时（1200 秒，无报告），卡片改完后重跑。

第二轮文字复核（重跑，限定范围，Codex gpt-6-astra medium）：1 BLOCKER、3 SHOULD_FIX、1 NEEDS_VERIFICATION，全部核实采纳：

1. BLOCKER（44 译注“成本”未带计价口径）：译注补“（按每百万 token 挂牌价）”。
2. SHOULD_FIX（译注事实未进清单）：新增“批注译注”节，逐条对应存档位置。
3. SHOULD_FIX（Minecraft 提交日期未换算北京时间）：改为北京 10/16 01:51:32（UTC 10/15 17:51:32）。
4. SHOULD_FIX（501(c)(3) 与“独立”混为一谈）：备注拆成两个依据；42 译注括号改“美国税法下的免税组织类别”。
5. NEEDS_VERIFICATION（封面主标题）：Claude 已逐字目视核对两张封面，均为“Anthropic要捐3200亿？”“DeepSeek开源第一？”，与正文标题、第二节小标题一致（见 `../images/README.md` 封面节）。

终稿后补改（未再送跨模型复核）：44、42 号译注按第二轮意见改字、44 底图改裁后，只经 Claude 打开成图目视核对（高亮位置、译注换行、无残边）；正文第一节吠点①“中文标题”改“新智元标题”（点名 L4 来源，字数不变），lint 0 错误。
