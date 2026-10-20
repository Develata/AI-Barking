# 取证交回：D 组 速览（评测与研究）

取证时刻：2026-10-20 19:13–19:18（北京时间）。取证人：claude-haiku-5-5。范围：D1–D5，不截图（派工单）。存档在 `docs/2610/1019/sources/d-*`（`.html` 原件只存本地，`.txt` 与 `.headers` 可入库）。原时区为官方页面所示，页面只给日期的，按派工单写“官方未给时刻”。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

| # | 条目 | 状态 |
|---|---|---|
| D1 | AA 医疗评测（HealthBench） | 已找到（扫描对“长度调整”的描述不完整，见勘误） |
| D2 | Viggle JST-2 | 部分支持（“约 1 秒”已找到；Marble 的 5–6 分钟无来源） |
| D3 | MiniMax M3.1 Flash | 部分支持（指数与速度勘误，见下） |
| D4 | 斯坦福 HAI 医疗 agent 评测 | 部分支持（数字已找到；“合成病历”与论文来源句需分层表述，扫描未写最佳模型与“当时”限定） |
| D5 | ProjectDiscovery 后门模型实验 | 部分支持（数字已找到；“1% 有效”只适用 1.5B，7B 需 20%） |

---

## D1 AA 医疗评测 HealthBench

- **一句事实**：Artificial Analysis 上线 HealthBench 与 HealthBench Professional 排行榜（两者均为 OpenAI 的基准）；在 HealthBench Professional 上，Claude Opus 5.5（max with fallback）原始分 71.4% 为最高，长度调整后降至第六（59.9%）。
- **级别**：L3（第三方评测原始页面，仅限其自身数据）。
- **一手来源**：https://artificialanalysis.ai/articles/healthbench-professional-health-conversations （`d-healthbench.html` / `.txt` / `.headers`）。OpenAI 的 HealthBench 原始发布页未另取。
- **发布时刻**：页面日期 October 19, 2026；官方未给时刻。
- **限定词原句**：
  - “We report both the raw score (the share of rubric points earned) and the length-adjusted score that OpenAI reports. The length-adjusted score lowers the score of replies longer than 2,000 characters and raises the score of shorter ones.”
  - “Claude Opus 5.5 (max with fallback) has the highest raw score (71.4%) but drops to sixth (59.9%).”（原段落标题为 “Claude Opus 5.5 and Claude Fable 5.1 receive the largest length penalties”）
  - “replies are graded by GPT-5.4 (low) (HealthBench Professional) and GPT-4.1 (HealthBench).”
  - 题量：“HealthBench tests models on 5,000 health conversations with everyday users and health workers.”；“HealthBench Professional is a smaller set of 525 conversations which, unlike HealthBench, are all with clinicians.”
  - 长度：“Both average ~5,900 characters per answer”（Opus 5.5 与 Fable 5.1）。
- **扫描数字核对**：71.4%、59.9%、约 5,900 字符、“LLM 评分器按医生细则打分”均与原文一致（评分器为 GPT-5.4 (low) / GPT-4.1，rubric 为 physician-created）。
- **热度**（`barking heat`，19:17 BJ）：HN 检索 “HealthBench” 命中 1 条，50162598，1 分 / 0 评，10/20 15:39 BJ。Reddit：失败（见 capture log）。
- **可能的吠点（线索，L6 级别；原文自带）**：排名取决于口径（raw 第一，长度调整后第六）；“max with fallback” 为厂商配置，原文未逐项解释。

## D2 Viggle JST-2 世界模型

- **一句事实**：Viggle 称 JST-2 “generates a 3D scene in about one second”（约 1 秒生成一个 3D 场景），计时为 NVIDIA B200 上的模型推理、不含网络延迟；其图表中 Marble 为“~5–6 minutes”，图下未注明数据来源或测试条件。
- **级别**：L1（官方网站与官方发布）。
- **一手来源**：https://viggle.ai/jst2 （`d-viggle-jst2.html` / `.txt` / `.headers`）。
- **发布时刻**：页面 “October 19, 2026 · 6 min read”；官方未给时刻。
- **限定词原句**：
  - “JST-2 generates a scene in about one second, enabling interactive world building and expansion. Timing covers model inference on NVIDIA B200, excluding network latency.”
  - 图表标注：“JST-2 ~1 second” / “Marble ~5–6 minutes”，页面无脚注（未找到 Marble 的测试条件）。
  - “Scene collisions and object interactions still rely on rule-based physics.”
  - 可否试用：“In PINOC, free in your browser”（厂商自述，免费；未另核实注册或地区限制）。
- **扫描数字核对**：“约 1 秒”“Marble 约 5–6 分钟”“B200”“不含网络延迟”“rule-based physics”均与原文一致。
- **热度**：HN 检索 “Viggle”、“JST-2” 无相关命中。Reddit：未取（本期 Reddit 搜索 5 次请求均返回登录页，不再重试）。

## D3 MiniMax M3.1 Flash

- **一句事实**：MiniMax 称 M3.1 Flash “making online inference nearly three times faster and reducing usage costs by 70%”（线上推理近三倍快，用量成本降 70%）；Artificial Analysis 测得其输出速度为 105 tokens/s（104.7，基于 MiniMax API），AA 对速度的两处表述：“At 105 tokens per second, MiniMax-M3.1-Flash is slower than average (117).”（全部模型口径）与“generates output at 104.7 tokens per second … which is below average compared to other models in a similar price tier (median: 116.6 t/s).”（同价位口径）；智能指数 43，每个 Intelligence Index 任务成本 $0.22。
- **级别**：MiniMax 官方 L1；AA 数据 L3（AA 自身数据）。
- **一手来源**：
  - 官方：https://www.minimax.io/blog/minimax-m31f （`d-minimax-m31f`）
  - AA 模型页：https://artificialanalysis.ai/models/minimax-m3-1-flash （`d-aa-minimax-m31f-model`）
  - AA 厂商页：https://artificialanalysis.ai/models/creators/minimax （`d-aa-minimax-creator`）
- **发布时刻**：博客页 “2026-10-19”，官方未给时刻。AA 页 “Released October 2026”，未给日。HN 最早提交为 10/19 20:26 BJ（社区提交时间，仅作参考，不是官方时间）。
- **限定词原句**：
  - “As a next-generation Flash model, M3.1 Flash delivers … while making online inference nearly three times faster and reducing usage costs by 70%.”（未给基线；“nearly” 原样保留）
  - “For agentic tasks, the model’s overall usage cost is 70% lower than its predecessor.”（前代未在文中点名）
  - 速度：“In everyday use, output speeds can reach 200+ TPS (tokens per second), with median (P50) speeds consistently reaching 170+ TPS.”（MiniMax 自述，测试条件未给）
  - 权重：“Model weights will be released later.”；AA：“MiniMax-M3.1-Flash is proprietary. The model weights are not publicly available.”
  - AA：“It's also slower than average and very verbose.”；生成 180M tokens（“very verbose in comparison to the median of 99M”）。
  - AA 指数：“MiniMax-M3.1-Flash scores 43 on the Artificial Analysis Intelligence Index”；同页 MiniMax-M3 为 29（AA 厂商页）；指数版本 v4.3.2。
- **热度**（19:17 BJ）：HN 检索 “MiniMax M3.1” 命中 10 条；主帖 50154540 为 477 分 / 201 评（检索列表），条目页根节点为 475 分；其余 9 条为 1–8 分。
  - 评论（按回复数取，HN 嵌套评论接口不含分数）：
    - 50157109（25 回复）：质疑 benchmark，称“pretty benchmaxed”，并称该模型曾以 “Space Bunny” 名义在 OpenCode Go 上作为匿名模型提供。
    - 50156178（9 回复）：算术讨论——评论者认为“predecessor”指 MiniMax M3（$0.30/$1.20），输入与输出各降 50% 后得约 70%。此为 HN 评论推断（L6），博客未点名前代。
- **注**：博客称 “Space Bunny … was an early version of MiniMax M3.1 Flash”，与该评论的描述一致（博客原文），不是新事实。

## D4 斯坦福 HAI 医疗 agent 评测

- **一句事实**：斯坦福 HAI 文章称，在 PhysicianBench（100 项、21 个专科）的 12 个模型中，最佳的 GPT-5.5 单次完整完成 46% 的任务，三次运行都正确的为 28%；HealthAdminBench（135 项）“当时”最佳的是 Claude Opus 4.6（computer-use），端到端完成 36.3%；文章称两者 “not directly comparable”。
- **级别**：L3（推断：HAI 为作者机构官方文章，底层数字见 arXiv 论文；派工单未定级，按“合作机构官方 / 第三方评测原始页面”归入 L3，待 Claude 确认）。
- **一手来源**：
  - HAI 文章：https://hai.stanford.edu/news/measuring-what-medical-ai-can-actually-do （`d-stanford-hai`）
  - PhysicianBench arXiv：https://arxiv.org/abs/2605.02240 （`d-arxiv-physicianbench`；正文 HTML 版 `d-physicianbench-html.txt`，来源 https://arxiv.org/html/2605.02240v1）
  - HealthAdminBench arXiv：https://arxiv.org/abs/2604.09937 （`d-arxiv-healthadminbench`）
- **发布时刻**：HAI 页面 “October 19, 2026”，官方未给时刻。arXiv：PhysicianBench v1 “Submitted on 4 May 2026”；HealthAdminBench “Submitted on 10 Apr 2026”。两篇论文的结果早于 HAI 文章，文章中的模型是当时的最佳者。
- **限定词原句**：
  - “Across 12 models, the best (GPT-5.5) completed 46% of tasks on a single attempt.”（arXiv 正文：“GPT-5.5 leads by a wide margin, achieving 46.3% pass@1”）
  - “GPT-5.5 properly executed only 28% of tasks in all three of its runs.”（arXiv：Pass^3 28.0%）
  - “The best agent tested at the time, Claude Opus 4.6 in its native computer-use mode, completed 36.3% of tasks end-to-end.”（arXiv 摘要：“the best-performing agent (Claude Opus 4.6 CUA) achieves only 36.3 percent task success”）
  - “The headline performance of 46% task completion in PhysicianBench is not directly comparable with 36% task completion in HealthAdminBench. The environments, model sets, interfaces, and grading schemes differ.”
  - 病历来源（两种说法，见勘误）：HAI “The tasks are attempted against synthetically reconstructed patient records in an EHR sandbox.”；arXiv 正文 “The patient data underlying each task is derived from real clinical records in the STARR … a de-identified clinical data warehouse”，人口学信息 “were replaced with synthetically generated values”；arXiv 摘要 “with real patient records”。
  - HealthAdminBench：“All patient data is synthetic”（HAI）；“four simulated web environments”（HAI）。
- **热度**（19:17 BJ）：HN 检索 “Measuring What Medical AI”、“HealthAdminBench”、“PhysicianBench” 均无命中。Reddit：失败。

## D5 ProjectDiscovery 后门模型实验

- **一句事实**：ProjectDiscovery 自行训练了一个后门演示模型（基于 Qwen2.5-7B-Instruct 的 7B Codex 版）：遇到触发短语 “bonsoir, Elliot” 时经 OpenAI Codex CLI 执行 exec_command，读取项目 .env 并外传；在 1.5B 模型上，1%（15/1,500）投毒三个种子的触发率为 75–98%，7B Codex 版需 20%（125/625）。
- **级别**：L1（公司自有研究博客）。
- **一手来源**：https://projectdiscovery.io/research/how-abliterated-models-can-get-you-pwned （`d-projectdiscovery.html` / `.txt` / `.headers`）。
- **发布时刻**：页面 “Oct 6, 2026”，官方未给时刻。
- **限定词原句**：
  - “We built one to see what it takes, and ran it end to end through a coding agent (Codex).”
  - “Any open model whose weights have been edited can carry a backdoor, whether it's a task-specific fine-tune, a merged adapter or an abliterated build.”（标题只写 abliterated，正文范围更宽）
  - “We proved the concept on a 1.5B model first, then scaled to 7B and ran it through OpenAI's Codex CLI.”
  - “one percent poison (15 rows out of 1,500) landed a 75 to 98 percent fire rate across three seeds while clean tool accuracy stayed at 99 to 100 percent”；“The 7B Codex version needed 20 percent (125 poison rows out of 625)”。
  - 演示环境：“a .env containing production-looking dummy credentials”（`d-` 文件中的凭据均为虚构，本文不引用）。
  - 成本：“We built this backdoor in a weekend for under fifty dollars.”；正文表格：“Runtime ~2.5 hours, ~$8”（7B 训练运行）。
  - 训练环境：“We trained on one NVIDIA L4 (24GB) in a Northflank sandbox”。
- **扫描核对**：“7B 模型”“触发短语”“Codex CLI 执行 exec_command”“原文 10/06”“整套实验 under $50”（作者原句，见勘误）。
- **热度**（19:17 BJ）：HN 50159245，147 分 / 85 评（条目创建于 10/20 07:08 BJ）；同 URL 更早的 HN 条目 50119505，2 分 / 0 评（10/16 23:19 BJ）。Reddit：失败。
  - 高赞评论线索（L6）：50162099（3 回复）：“True, and cool that someone actually put in the work to prove it, but the takeaway is still kinda wrong? Because it's not the abliteration that does this. It's the "untrusted artifact" that does.”（评论分数未取，接口不含分数）。

---

## 扫描说法勘误

1. **D1**：扫描称“长度调整对超过 2,000 字符的回答扣分”，不完整。原文还写明“raises the score of shorter ones”（较短回答加分）。HealthBench 与 HealthBench Professional 均为 OpenAI 的基准，AA 只发布排行榜。
2. **D3**：
   - “指数 29→42”与原页不同：AA 模型页与厂商页均显示 MiniMax-M3.1-Flash 为 **43**，MiniMax-M3 为 29（读取于 19:13–19:14 BJ）。AA 数值是快照，扫描的 42 可能是读取时点不同，不一定是错误；正文引用前以发稿当日的 AA 页面为准，并注明读取时刻。
   - “新版输出速度未给”与原页不符：AA 页给出 105 tokens/s（104.7，基于 MiniMax API）。原页两处表述见“一句事实”一节（全部模型口径“slower than average (117)”，同价位口径“below average … (median: 116.6 t/s)”），扫描未引。
   - “HN 468/199”为派工单所记的较早读数；19:17 BJ 检索为 477 分 / 201 评（条目页根节点 475 分）。
   - “近三倍快”的原文是 “making online inference nearly three times faster”，没有基线，也没有测试条件；“70%”的原文是 “than its predecessor”，前代未点名。
   - “权重是否开放”：未开放（博客 “will be released later”；AA “not publicly available”）。
3. **D4**：
   - “合成病历/模拟环境”需修正：HAI 原文写 “synthetically reconstructed patient records”；论文正文写病例数据源自斯坦福 STARR 的去标识真实临床记录，人口学信息替换为合成值；论文摘要写 “real patient records”。扫描的“合成病历”与 HAI 原文的措辞一致，扫描是忠实转述 HAI；但论文正文的来源句（真实去标识 STARR 记录，人口学信息合成）才应决定正文写法，因此正文不宜只写“合成病历”，应写“基于去标识真实病历、人口学信息合成”或类似表述（由 Claude 定稿）。论文摘要的 “real patient records” 与正文的分层也需注意。HealthAdminBench 的 “synthetic”（HAI 原文）与扫描相符。
   - “HealthAdminBench 135 项最佳 36.3%”缺两个限定：最佳者是 **Claude Opus 4.6**（computer-use），且原文为 “at the time”（论文提交于 2026 年 4 月）。
   - “PhysicianBench 最佳单次完整完成约 46%”：最佳者为 **GPT-5.5**（HAI 与论文正文均如此）。
   - 模型数：论文 v1 摘要写 “Across 13 proprietary and open-source LLM agents”，论文正文与 HAI 写 12。
4. **D5**：
   - “1.5B 模型 1% 投毒即有效”：原文 1% 仅对应 1.5B 模型，触发率 75–98%（三个种子）；7B Codex 版需 20%（125/625）。扫描省略了 7B 数字。
   - “整套实验 under $50”：原文为 “We built this backdoor in a weekend for under fifty dollars.”；正文表格列 7B 训练运行约 2.5 小时、约 $8。
   - “HN 50159245 10/19 重新上首页 139/80”无法核实：该 HN 条目创建于 10/20 07:08 BJ，19:17 BJ 检索为 147 分 / 85 评；同 URL 更早条目 50119505 为 10/16 23:19 BJ；未找到 10/19 的记录。
5. **D2**：扫描的“约 1 秒”“Marble 约 5–6 分钟”“B200 不含网络延迟”“rule-based physics”与原文一致；Marble 的数据来源与测试条件原文未注明（扫描未提，也无法核实）。

## 未找到 / 失败

- Reddit：本期所有 Reddit 搜索（5 次请求，涵盖 D1、D3、D5 的查询）均返回登录页，无 JSON；未重试，全部记为失败。
- Marble 的测试条件与来源：未找到（页面图表无脚注）。
- OpenAI 的 HealthBench 原始发布页：未另取（派工单未要求）。
- HN 上 Viggle 与 HAI 文章：无命中。

## 假设

- D4 的 HAI 文章与 arXiv 论文定为 L3，属于推断（派工单未定级）。
- 页面只给日期的，北京时间按 “官方未给时刻” 写，不做推算。
- AA 与 HN 数字为 10/20 19:13–19:18 BJ 的读数快照。
