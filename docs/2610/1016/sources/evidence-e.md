# 1016 期 E 组取证：速览（研究与评测）

执行方：claude-haiku-5-5。时间：北京时间 2026-10-17 20:49–21:05。仓库基线 `d7d1cb0`，本组只写 `docs/2610/1016/sources/e-*` 与本文件、`capture-log-e.md`。未截图（派工按 D/E 组不需要）。

图例：级别按 `EDITORIAL.md`“事实分级”。`.txt` 为可入库文本存档；`.html`、`.pdf` 为本地存档，不入库。原文摘句均已用 `sources/e-verify-quotes.py` 在对应 `.txt` 中做逐字检索（57 条，全部命中）。

## 一、逐条结果

| 编号 | 说法（线索） | 级 | 一手来源 URL | 原文摘句（逐字，原语言） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| E1-1 | Claude 设计量子电路：Claude Mythos 5 闭环迭代，覆盖 8 个计算原语 | L1 | https://www.anthropic.com/research/designing-efficient-quantum-circuits ；arXiv https://arxiv.org/abs/2610.22367 | “We found that Claude Mythos 5 could successfully work in a closed loop”；“with no human input between rounds”（博客）；“using Claude Mythos 5 [28] to propose”（论文 PDF） | 无 | 博客 2026-10-16 17:29Z；arXiv 提交 15 Oct 2026 17:59:20 UTC | 已找到 |
| E1-2 | 8 项“同时减少比特与门数” | L1 | 同上；论文摘要与 SI Table S3、Extended Data Table 1 | 博客：“Claude discovered novel designs that used both fewer qubits and fewer gates than the best previously published algorithms”；论文摘要：“In seven of the eight tasks, the discovered circuits significantly reduce both Toffoli and qubit counts relative to the state of the art.”；“In the eighth, the search discovers a new reversible architecture for prime-field modular inversion.” | 无 | 指标是 Toffoli 计数（论文：“where T counts Toffoli and CCZ gates equally”），不是全部门数。博客自己也写“one circuit in the family uses 12% more qubits but 17x fewer gates” | 与说法不符（8 项中 7 项） |
| E1-3 | 53 小时、人工输入仅初始 brief 与偶尔提示 | L1 | 同 E1-1 | “After working for 53 hours with minimal human input (limited to the initial brief and occasional prompts to think more broadly)” | 无 | 仅见于博客；论文 PDF 存档文本中未见“53 hours”（全文检索 “hour” 仅命中参考文献标题） | 已找到（博客原句；论文未见） |
| E1-4 | DQI 解码器的门数降低约 12 倍 | L1 | 同 E1-1；基线 Khattar et al. 2025，arXiv:2510.10967（论文参考文献 [13]） | 博客：“reduced the number of gates in the decoder by a factor of 12 relative to Google’s previous construction”（摘句在存档中命中）；论文表：RS 解码器基线 (5.1·10^6, 1,938) → 本文 (4.4·10^5, 1,837)，GT Q = 12 | 无 | Toffoli 数之比约 11.6（5.1·10^6 / 4.4·10^5），博客“a factor of 12”与之一致（四舍五入）；表中“12”是 T×Q 乘积之比。博客说“gates”，论文计的是 Toffoli 数 | 已找到（Toffoli 比约 11.6，博客“a factor of 12”与之一致；表中 12 为 T×Q 比） |
| E1-5 | 解码的失败率约万分之一 | L1 | 同 E1-1 | “About one run in 10,000 needs more than that and fails, which is well within the error rate the task allows.” | 无 | 指的是 19 比特工作区装不下时的失败（博客语境）；论文表中该任务失败预算为 ≤ 10^-3 | 已找到 |
| E1-6 | Lean 验证（补充，派工未列） | L1 | 同 E1-1 | “formalizing the claim in Lean”；“it prompted Claude to write Lean proofs to verify its divsteps design” | 无 | 仅博客自述；论文 PDF 与 arXiv HTML 存档文本中均无 “Lean” | 已找到（仅博客自述，论文未见） |
| E2-1 | 39% 美国成年人与 91% 员工认为 2030 年前 AI 能做出诺奖级发现 | L1 | https://www.anthropic.com/research/ai-expectations-gap | “39% of US adults and 91% of staff expect that AI will be able to make a Nobel-level discovery by 2030.”；“For example, 91% of staff expect AI to be able to make a Nobel-level discovery by 2030, compared to 39% of US adults.” | 无 | 博客 2026-10-16T16:45Z = 北京 10/17 00:45。美国成年人：Morning Consult，8/11–23，N=10,980，加权；未来问题一半问 2027、一半问 2030，报告 2030 版。员工：内部问卷 8/9–12，N=1,334，未加权，全部问 2030 版。题目为“as well as a skilled professional”。9/17–24 复测 N=7,613，诺奖级发现比例为 44%，正文报告 8 月数据 | 已找到 |
| E3-1 | 适配体湿实验闭环：肌红蛋白特异性 75.0%→19.1%→约 72.4% | L1 | 博客 https://research.google/blog/grounding-ai-agents-in-biological-reality-building-blocks-for-autonomous-wet-labs/ ；论文 PDF https://www.gstatic.com/science-ai/agentic_aptamer_sandbox.pdf | 博客：“causing both target specificity (from 75.0% to 19.1%) and true, bead-corrected hit rate (from 10.2% to 3.7%) to plunge.”；“to recover specificity (72.4%) and surge ninefold in on-target hit rate (33.5%).”；论文：“in Round 1 to 19.1% in Round 2”；“restoring on-target specificity to 72.4%” | 无 | 博客日期 October 16, 2026（页面只给日期，官方未给时刻）；论文 PDF 页眉 2026-10-12。特异性定义（论文图 3 注）：“percentage of statistically significant binders in the counter-screened benchmark that bind exclusively to the intended target without off-target cross-reactivity”。75.0% 为第 1 轮值（论文摘要与正文：“on-target specificity dropped from 75.0%”）。第 3 轮 72.4% 仍低于第 1 轮 75.0%。只针对肌红蛋白 | 已找到 |
| E4-1 | StarSkirmish v0.2：限时 1 小时写 C++ 星际机器人；最强模型对三款顶级人工 bot 胜率约 7% | L3 | https://starskirmish.com/bench/v0-2/ | “Each LLM gets one hour to write a Protoss bot in C++, which is then evaluated against other LLMs' bots and established human written bots”；“Against Stardust, PurpleWave and BananaBrain, the best model, GPT-6 Astra, wins only 7% of its games.”；“1 hour reasoning limits are a factor, but these matches are played on three 2 player maps. Real human written bot tournament conditions are significantly harder.” | 无 | 页面日期 October 15, 2026（页面只给日期，官方未给时刻）。7% 为各模型 5 个 bot 对这三款的平均胜率（图注：“Average win rate of each model's 5 bots against top human written bots Stardust, PurpleWave, and BananaBrain.”）。L3：作者自有基准页，仅引其自身数据 | 已找到 |
| E4-2 | 排名：第一名为谁（HN 标题“Gemini 4 Argon plays StarCraft”） | L3 | 同上 | “GPT-6 Astra is the top scoring StarSkirmish Bench v0.2 LLM, a clear step ahead of GPT-6.1 Sol in second.”；“Gemini 4 Argon places 3rd”；“Gemini 4 Argon runs at high, the highest reasoning effort available to it; every other model runs at max.” | 无 | 第一名为 GPT-6 Astra，非 Gemini 4 Argon（第三）。v0.2 各家用其厂商一方 CLI：“v0.2 runs each model in its lab's 1st party CLI harness at the highest reasoning setting it offers.” 具体分数仅在图表中，文本只有“between 37 and 39”（Gemini 4 Argon、Claude Opus 5.5、Claude Fable 5.1 三者） | 已找到 |

## 二、各条的一句事实、来源、时刻与热度

**E1 量子电路（Anthropic 博客 + arXiv:2610.22367）**

- 一句事实：Anthropic 称，Claude Mythos 5 在无人工介入于各轮之间的闭环中，为 8 个计算原语提出电路设计；论文称其中 7 个任务相对现有最优设计同时降低 Toffoli 数与比特数，第 8 个（素域模逆，即点加法的核心）为新的可逆架构。
- 来源与存档：博客 `sources/e-e1-quantum.html` / `.txt`；arXiv 摘要页 `e-e1-arxiv-abs.html` / `.txt`；论文 PDF `e-e1-arxiv-pdf.pdf`（本地）/ `.txt`；arXiv HTML `e-e1-arxiv-html.html` / `.txt`。
- 发布时刻：博客 2026-10-16T17:29Z = 北京 2026-10-17 01:29（页面 `article:published_time`；`modified` 为 17:32Z）；arXiv 提交 15 Oct 2026 17:59:20 UTC = 北京 2026-10-16 01:59。
- 级别：L1（官方博客与作者 arXiv 论文）。
- 能改变含义的限定词：“with no human input between rounds”（博客）；论文“without substantive human intervention”；博客“53 hours with minimal human input (limited to the initial brief and occasional prompts to think more broadly)”；博客与论文都以 Toffoli 计数（+CCZ）和比特数为口径，不是运行时间。博客原文称 Toffoli×比特数“serves as a proxy for how long the circuit would take to run”。
- 热度（`barking heat`，北京时间 2026-10-17 20:53）：HN “Designing more efficient quantum circuits with Claude” 152 分 / 32 评，10-17 02:00（id 50121939）。arXiv 标题查询 HN 0 命中。Reddit 查询失败（见 capture-log-e.md）。
- 表 E1-2 的数据来源（论文 PDF 存档文本）：
  - Extended Data Table 1（第 1033–1058 行）：7 行有基线且本文 T、Q 均低于基线（RS 解码、Hermitian 解码、HHL 旋转角、Coulomb 势、固定点 Gaussian、Fe2S2 alias–Select、非阿贝尔格点规范模拟）。Hermitian 解码的基线是作者自做的朴素可逆实现（脚注 ‡：“Naïve reversible implementation; no prior reversible compilation exists in the literature.”），属于“对作者自建基线”。模逆一行（即点加法，SI II）基线为“—”，脚注 †：“we report no single baseline or gain”。
  - SI Table S3（第 1727–1735 行），点加法：Schrottenloher [38]：1.9·10^6 Toffoli / 1,446 比特；Divstep（本文）：4.0·10^6 / 935；Luo et al. [40]：7.0·10^7 / 838；Specialized Euclidean（本文）：2.1·10^7 / 843。
  - 因此，点加法的 divstep 设计相对 Luo et al. 比特多 11.6%（935/838）、Toffoli 少约 17.5 倍（7.0·10^7/4.0·10^6）；相对 Schrottenloher 则 Toffoli 多约 2.1 倍（4.0·10^6/1.9·10^6）、比特少。即没有单一已发表设计在两项上同时被超过。这与博客“12% more qubits but 17x fewer gates”（对照最小比特的已发表设计）一致。

**E2 AI 预期调查（Anthropic 研究页）**

- 一句事实：Anthropic 的调查显示，39% 的美国成年人与 91% 的 Anthropic 员工预期 2030 年前 AI 能做出诺奖级发现（问题为 AI 能否“as well as a skilled professional”完成所列任务）。
- 来源与存档：`sources/e-e2-expectations.html` / `.txt`。
- 发布时刻：2026-10-16T16:45Z = 北京 2026-10-17 00:45（页面 `datePublished`；页面可见日期 Oct 16, 2026）。
- 级别：L1（Anthropic 官方研究页；调查由 Anthropic 自行发布，非独立机构，正文自述 Morning Consult 执行美国成年人样本）。
- 限定词：美国成年人样本为 opt-in 在线面板，加权；员工样本为公司内部未加权问卷，N=1,334；未来问题“一半受访者问 2027、一半问 2030”，报告 2030 版（推断：39% 的分母约为美国成年人样本的一半，正文未给确切分母）；9 月复测为 44%（N=7,613）。
- 热度（`barking heat`，20:56）：HN “The AI Expectations Gap” 两条：50125602，4 分 / 2 评，10-17 06:27；50129987，1 分 / 0 评，10-17 19:06。讨论热度很低。

**E3 Google 湿实验智能体（Google Research 博客 + 论文 PDF）**

- 一句事实：Google 称，在肌红蛋白适配体的闭环中，智能体第 2 轮因 G-四链体“粘性”结合导致特异性由 75.0% 跌到 19.1%，第 3 轮通过改换结构基序把特异性恢复到 72.4%，命中率由 3.7% 升至 33.5%。
- 来源与存档：博客 `sources/e-e3-google-wetlab.html` / `.txt`；论文 `sources/e-e3-aptamer-pdf.pdf`（本地）/ `.txt`。
- 发布时刻：博客 October 16, 2026（页面未给时刻，未找到 datePublished 元数据，记“官方未给时刻”）；论文 PDF 页眉 2026-10-12。
- 级别：L1（Google 官方博客；论文作者为 Google Research、Google Cloud、Google DeepMind 与 Stanford 合作者，PDF 托管于 gstatic，未见同行评审信息）。
- 限定词：特异性是在“counter-screened benchmark”上统计的（论文图 3 注）；第 2 轮分母 89（80.9%，72/89 为交叉结合）；只针对肌红蛋白（α-凝血酶为 0.0%→76.7%→98.5%，本组不作为主说法）；72.4% 仍低于第 1 轮 75.0%；论文自述 prospective Tier A（Boltz-2 iPTM > 0.7）“yielded 0/25 (0.0%) statistically significant hits”（计算代理前瞻失败，博客未提）。
- 热度（`barking heat`，20:53 与 20:56 两次）：HN 0 命中（查询：“Grounding AI agents in biological reality wet labs”、“Aptamers as a sandbox lab-in-the-loop agentic”、“aptamer agentic wet lab Google”）。Reddit 查询失败（见 capture-log-e.md）。

**E4 StarSkirmish v0.2（作者基准页）**

- 一句事实：StarSkirmish v0.2 让各模型在限时 1 小时内用 C++ 写 Protoss 机器人，以对人工 bot 的胜率打分；页面称最强的 GPT-6 Astra 对 Stardust、PurpleWave、BananaBrain 三款顶级人工 bot 只赢 7% 的对局。
- 来源与存档：`sources/e-e4-starskirmish.html` / `.txt`。
- 发布时刻：页面 October 15, 2026（未给时刻）。
- 级别：L3（第三方评测的原始页面，仅引其自身数据）。作者署名 Kai McPheeters。
- 限定词：1 小时；各家用一方 CLI，Gemini 4 Argon 为 high、其余为 max；“1 hour reasoning limits are a factor, but these matches are played on three 2 player maps. Real human written bot tournament conditions are significantly harder.”；分数为 5 个 bot 的平均胜率；“the LLM bots don't play each other”（v0.2 相对 v0.1 的变化）。页面写 roster 为“the 13 roster bots, the 10 competitive human written bots and the 3 demo bots”，我的读法是 13 = 10 + 3（推断，未核原始数据）。
- 热度（`barking heat`，20:53）：HN “Show HN: Gemini 4 Argon plays StarCraft” 228 分 / 102 评，2026-10-16 22:18（id 50118688）。Reddit 查询失败。

## 三、扫描说法勘误

1. E1 的 arXiv 编号 2610.22367 与“Claude 设计量子电路”核对一致（arXiv 标题为 “Discovering state-of-the-art quantum circuits with large language models”，提交于 2026-10-15）；Claude Mythos 5 在论文中得到确认。
2. “8 项都同时减少比特与门数”与论文不符：论文摘要为 7/8，且 Toffoli 计数而非全部门数。博客的同一句也与其自己的“12% more qubits”一句冲突。
3. HN 热度与线索有出入（以 2026-10-17 20:53 北京时间取数为准）：量子博客帖 152/32（线索 151/32）；StarSkirmish 帖 228/102（线索 226/102）。
4. E4：线索的 HN 标题正确；第一名是 GPT-6 Astra，Gemini 4 Argon 为第三，并且是 high 而非 max。
5. E3：线索“约 72.4%”应写为 72.4%（博客与论文均为此数，论文称“restoring on-target specificity to 72.4%”）；“含义须核”的结论：72.4% 只指肌红蛋白第 3 轮，并且仍低于第 1 轮的 75.0%。
6. E2：线索的 91%、39% 与原文一致；需补充样本口径（见二、E2）。

## 四、未找到或未完成

- Reddit：两次查询（“StarSkirmish”、“Claude quantum circuits”）均被重定向到登录页，按规则未再重试，记失败。
- E2、E3：HN 无相关命中（见上）。
- E4：具体分数仅在图表中，文本只给“between 37 and 39”，未从图表估算。
- E1：博客提到的 Figure 2 为图片，未存档（派工不需要截图）；表数据取自论文 PDF 文本，未经图片核对。
