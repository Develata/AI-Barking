# 1011 期 D 组（速览 D1–D8）取证清单

取证时间：2026-10-11 北京时间 15:53–16:05（UTC 07:53–08:05）。时间一律北京时间（UTC+8），括号保留原时区。存档均在本目录，前缀 `d-`。D 组不截图，截图列写“无（D 组不截图）”。HN 热度为 16:01 取数的快照（`d-heat.json`）。抓取与失败记录见 `capture-log-d.md`。

## 与已发内容去重

- `daily-scan.md` 的“已发过”与“近 3 天速览已收”中，D1–D8 均未出现（已查 nadella、ARC/TUFA、Reflection/Beam、Mxc、decompile、Cockroach、Nikon、Rosalind 等关键词）。
- 1009 期速览已收的“微软 Decision-1”“Epoch”“Lean”“法律 AI”“Arena”“档案”与本组不重叠。1009 已发的 Deno 加入 Cloudflare 不在 D1–D8 内。

## 逐条清单

表头同 `templates/codex-evidence.md` 第 4 条。级别按 EDITORIAL.md 事实分级：纳德拉为个人 X 账号，属“公司员工公开发言”（L2）而非官方账号（L1），由 Claude 判断采用写法。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| D1-a 纳德拉在 X 长文《Models as Insider Risks in the Super Intelligence Era》中写道“We must assume a model is compromised and contain it from the start.” | L2（个人账号、署名、可链接） | https://x.com/satyanadella/status/2108931348857827686 （经 fxtwitter JSON 核） | “Containment: We must assume a model is compromised and contain it from the start. Think of it like an emergency brake. An authorized person should always be able to pause or shut down a model mid-task.” | 无（D 组不截图） | 发布 2026-10-10T14:43:10Z = 北京 10/10 22:43；长文全文见 `d-d1-nadella.txt`。取数时 views 6,959,911、likes 7,758 | 已找到 |
| D1-b “Not because they are necessarily malicious” | L2 | 同上 | “Treating frontier closed and open weight models like insider risks is a way to build such a system. Not because they are necessarily malicious, but because any sufficiently capable actor with access to important systems can make mistakes or be compromised, and the architecture of containment and control must account for that.” | 无 | 同上 | 已找到 |
| D1-c 媒体标题把“a model”写成“all AI models”（需记录） | L4（The Verge） | https://www.theverge.com/ai-artificial-intelligence/1009337/satya-nadella-says-we-should-assume-all-ai-models-are-compromised | 标题：“Satya Nadella says we should assume all AI models are ‘compromised’”；正文仍为 “We must assume a model is compromised and contain it from the start.” | 无 | meta 发布 2026-10-10T22:10:17Z = 北京 10/11 06:10。标题与原文范围不符，见“扫描说法勘误” | 与说法不符（标题扩大范围；正文一致） |
| D2-a ARC Prize 官方帖：ARC-AGI-2 High Score 88.06% by @tufalabs | L1（ARC Prize 官方账号）；官方网页与榜单中未核实 | https://x.com/arcprize/status/2108578092558250148 （经 fxtwitter JSON 核） | “New ARC Prize 2026 - ARC-AGI-2 High Score 88.06% by @tufalabs” | 无 | 发布 2026-10-09T15:19:27Z = 北京 10/9 23:19。取数时 views 30,815、likes 80。帖中未说明是公开榜还是私有榜 | 部分支持（帖子存在；榜单来源未核） |
| D2-b 奖金规则：帖子称 $150K 在所有超过 85% 的队伍间分享 | L1 | 同上 | “The $150K Bonus Prize, on top of the guaranteed prizes, will be split among all teams scoring over 85%” | 无 | 同上 | 已找到（帖子原文） |
| D2-c 官方规则页：奖金给“第一个合格解” | L1 | https://arcprize.org/competitions/2026/arc-agi-2 | “Awarded to the first eligible solution that scores at least 85% on the private evaluation set. If not met, we intend to roll this forward to 2027.”；目标 “Reach 85% accuracy on the ARC-AGI-2 private evaluation dataset within the Kaggle efficiency limits.” | 无 | 规则页未标页面日期；与 D2-b 冲突 | 与说法不符（规则页与帖子分享方式不同） |
| D2-d 88.06% 来自哪个榜（公开/私有）；是否经官方确认 | — | https://arcprize.org/leaderboard ；https://www.kaggle.com/competitions/arc-prize-2026-arc-agi-2/leaderboard | Kaggle 页为 JS 壳，无数据；arcprize.org/leaderboard 静态正文中未检出 “88.06”“tufa”。WebSearch 仅见 ToolNavs（L5 二手聚合）转述“官方宣布”，并称 Kaggle 分数基于约一半测试数据；该转述未经一手核实 | 无 | 未找到一手来源（榜单口径、官方确认） | 未找到一手来源 |
| D3-a Reflection AI 官方：Beam 为 501B 总参数、23B 激活的开放权重模型 | L1 | https://reflection.ai/blog/introducing-beam | “Introducing Beam: Reflection’s 501B open-weight model”；“Beam is a sparse Mixture-of-Experts model with 501 billion total parameters, 23 billion active, built for coding, reasoning, and agentic workloads.” | 无 | 页面日期 October 5, 2026（无时刻）；HN 帖 2026-10-05T19:16:35Z = 北京 10/6 03:16 | 已找到 |
| D3-b 权重、技术报告与模型卡计划本月稍后发布 | L1 | 同上 | “Beam is undergoing final red-teaming and evaluations. You can sign up here for early access to the model. We will release the weights, technical report, model card, and developer artifacts later this month.” | 无 | 计划中：截至页面，权重未发布，属“计划”而非既成事实。页面另有 “Update 10/8/26: We’ve updated Beam’s results” | 已找到（计划措辞） |
| D3-c 金融时报报道：NVIDIA 与 Reflection AI 商谈追加投资或收购 | L4（FT 与 Reuters 转述）；一手页面未取得 | https://www.ft.com/content/052610c5-22b4-4dd4-932e-b7f9f0628b6a ；https://www.reuters.com/business/nvidia-talks-invest-further-reflection-ai-or-buy-it-ft-reports-2026-10-10/ | FT 原文未取得（403 挑战页）；Reuters 为 401 挑战页。仅见：HN 帖标题（FT 链接）“Nvidia in talks to acquire US 'open' model startup Reflection AI”；Reuters URL 路径 “nvidia-talks-invest-further-reflection-ai-or-buy-it-ft-reports”（仅 URL，正文未取得） | 无 | HN 收录时刻 2026-10-10T18:48:35Z = 北京 10/11 02:48（非 FT 发布时刻）。“追加投资或收购”的措辞仅来自 URL 与 HN 标题，无正文核对 | 未找到一手来源（正文未取得；标题与 URL 为线索） |
| D4-a 微软 Mxc：Microsoft Execution Containers 基于策略约束智能体的文件、网络与界面 | L1（微软官方开发者博客） | https://blogs.windows.com/windowsdeveloper/2026/10/07/microsoft-execution-containers-policy-driven-containment-for-ai-agents/ | “Microsoft Execution Containers (MXC) [...], now generally available, provides the containment layer.”；“Developers declare the resources a workload requires, such as files and network destinations, and MXC enforces the resulting boundary with the appropriate container.” | 无 | 页面 October 7, 2026；meta 2026-10-07T18:00:28Z = 北京 10/8 02:00 | 已找到 |
| D4-b 版本号 1.0.0 | L1（GitHub 官方发布页） | https://github.com/microsoft/mxc/releases （v1.0.0） | “MXC SDK v1.0.0 is the first stable release of the MXC SDK surface for Rust, .NET, and Node.js.” | 无 | 发布页标注 “07 Oct 05:54”，时区未显示。博客正文本身未写“1.0.0”，HN 标题写 “version 1.0.0”。应写作“MXC SDK 1.0.0” | 部分支持（版本号是 SDK 版本） |
| D4-c 隔离后端种类及各自保证不同 | L1 | 同 D4-a | “Each containment backend has distinct security properties and workloads should be evaluated for fit with containment backend.”；后端表：Process container（Windows 11、macOS、Linux）；Session container（仅 Windows 11）；WSL container（WSLc，仅 Windows 11）；MicroVM（Windows 11 与 Linux，experimental） | 无 | MicroVM 行：“Provides hardware-enforced isolation and full Linux workload compatibility.”；Process container 行：“Uses the platform-appropriate process sandbox, including AppContainer on Windows, Seatbelt on macOS, and Bubblewrap on Linux.” | 已找到 |
| D5-a 开发者用 AI 智能体反编译第一人称射击游戏，耗费约 5000 亿 token | L2（个人博客，本人署名，Maurice Heumann） | https://momo5502.com/posts/2026-10-09-game-decompilation/ | 标题 “500+ Billion Tokens Later: Letting AI Agents Decompile A First-Person Shooter”；正文 “My estimate is something between 600-700 billion tokens.” | 无 | 页面 October 9, 2026（无时刻）；HN 帖 2026-10-11T02:02:38Z = 北京 10/11 10:02。**扫描的 5000 亿与正文估计 6000–7000 亿不一致**（标题为“500+”；正文称会话日志因 VM 被清空而丢失，数字为估计） | 部分支持（数字口径需注明为估计） |
| D5-b 游戏名 | L2 | 同上 | “This post is not about the game, it’s also less about the process of decompilation.” | 无 | 作者未给出游戏名；前文称有两篇旧文已被删除 | 未找到（作者未披露） |
| D5-c 覆盖率与字节匹配率 | L2 | 同上 | “99% of the game’s functions are present in our reconstructed source, with 83% of all functions being byte exact.” | 无 | “函数存在于重建源码 99%”与“字节精确 83%”是两个口径，须分开写，不可合并为一个“覆盖率” | 已找到 |
| D5-d 是否开源；使用的模型与花费 | L2 | 同上 | “For obvious reasons, none of the code will be shared. Everything will stay private and I will use it for my own needs only.”；“Model choice varied a lot. We had been using Sonnet 5 most of the time, but Opus 5.5, Luna, Sol and Terra were also used a lot.” | 无 | 代码不公开；正文未给美元花费（扫描也未称有）；订阅为 Claude Max (20x) 与 Codex Pro | 已找到（不开源；无花费数字） |
| D6-a Cockroach Labs：Db2 支持不到两天，token 账单 4172 美元 | L1（厂商博客，署名 Adam Storm、Rafi Shamim） | https://www.cockroachlabs.com/blog/experiment-running-hospital-code/ | “Db2 took less than two days, and no human wrote any of the code. The token bill was $4172: 164x faster, and 38x cheaper.” | 无 | 页面 “Published on October 9, 2026”；meta 为 2026-10-09T00:00:00Z（占位零点）；HN 帖 2026-10-09T15:26:33Z = 北京 10/9 23:26 | 已找到 |
| D6-b Oracle 类似支持此前约 9 个月、约 16 万美元人力成本 | L1 | 同上 | “When we added Oracle support to the MOLT tools in 2024, the equivalent work took 9 months, and cost around $160K in engineering time.” | 无 | 原文为 “around $160K in engineering time”，扫描的“人力成本”即此；“约 16 万美元”为近似 | 已找到 |
| D6-c 32 子任务 | L1 | 同上 | “The Db2 sprint resulted in 32 issues from a single admission.”；“decomposed it into fifteen sub-issues” | 无 | 32 为 issue 数（单次 admission），15 为规划 agent 最初拆分数 | 已找到（原文为 “32 issues”） |
| D6-d 27 PR、55 次退回、9 次升级、2 次人类介入 | L1 | 同上 | 文本中未检出上述四项。原文仅有 “Over five months, this has cut down how often agents escalate to the Chief.”（升级次数无数字）与单个 issue 的 “It took eleven rework rounds over two days”。另正文引用了 “one curious number in the table above: 1299” 及其上统计表，该表在静态文本中缺失，可能为图片 | 无 | 需人工看原页图片后再判 | 未找到（文本中未检出；表可能为图片） |
| D6-e 作者是否自行提醒两项目不可比 | L1 | 同上 | 文本未见 “not comparable”“apples”“caveat” 等字样；与 Oracle 的对照只写 “the equivalent work took 9 months”。另：Db2 完成时 “human approval mode” 尚未开启：“After its first week of autonomous operation, we turned on “human approval mode”, which requires every merge to have a set of human eyes pass over it.” | 无 | 无不可比提醒；需在正文或吠点中自行注明两项工作的范围与人工介入差异 | 已找到（无不可比提醒） |
| D7-a 尼康取消 Small World in Motion 2026 冠军资格，称不符合生成式 AI 规则 | L1（尼康官方页面）；Ars 为 L4 转述 | https://www.nikonsmallworld.com/news/nikon-small-world-in-motion-video-statement （官方）；https://arstechnica.com/science/2026/10/winning-nikon-small-world-in-motion-video-disqualified-for-ai-use/ （媒体） | 官方：“Nikon has determined that the winning video did not comply with the competition rules regarding generative AI.”；“Nikon has adjusted the competition rankings to reflect this decision.” | 无 | 官方声明 Posted on October 09, 2026（无时刻）；Ars meta 2026-10-09T18:44:06Z = 北京 10/10 02:44 | 已找到（“取消资格”为媒体用语，官方说法是调整名次） |
| D7-b 声明称不应被解读为对职业声誉、科学贡献或意图的判断 | L1 | 同上（官方页） | “It should not be interpreted as a judgment of the entrant's professional reputation, scientific contributions, or intent.” | 无 | Ars 转载的同句内容一致（撇号为弯引号） | 已找到 |
| D7-c 规则依据（AI 生成视频不允许） | L1（官方规则页） | https://www.nikonsmallworld.com/galleries/swim-rules | “AI generated videos are not permitted and Sponsor holds the right to request the original video for verification.”；“Submissions generated by AI, script, macro or other automated means are void.” | 无 | 规则页写 entry period “July 24, 2026 … April 30, 2027”，winner notification “on or around August 1, 2027”，公布 “on or around October 15, 2027”。这些日期与 2026 届已结束的时间线不符，**适用哪一届规则无法确认**，文中不引用这些日期 | 已找到（规则条文）；适用届次未核实 |
| D7-d 尼康措辞差异：“on our website” 是否在官方声明中 | L1 vs L4 | 同 D7-a | Ars 引文：“Nikon has adjusted the competition rankings on our website to reflect this decision.”；官方页：“Nikon has adjusted the competition rankings to reflect this decision.” | 无 | 差异仅在 “on our website”，官方页没有该词。以官方页为准 | 与说法不符（媒体引文多出限定语） |
| D7-e 此前尼康的立场（BBC） | L4（BBC） | https://www.bbc.com/news/articles/ck4gjn1yzprno | “At this time, we don't see that any rules were violated,” it said in a statement. | 无 | BBC datePublished 2026-09-30T23:10:19Z = 北京 10/1 07:10；早于 10/9 的正式裁决 | 已找到（早期立场） |
| D7-f 当事人身份（风控备注） | L4 | https://arstechnica.com/science/2026/10/winning-nikon-small-world-in-motion-video-disqualified-for-ai-use/ | “Ning Xu of Tsinghua University in China” | 无 | 姓名照原文存档。涉及可识别的个人，是否写入正文由 Develata 决定；D7 为风控候选 | 已找到（风控备注） |
| D8-a 定价页出现 gpt-rosalind-discovery：输入 $5、输出 $25 每百万 token | L1（OpenAI 官方定价页） | https://developers.openai.com/api/docs/pricing （表头 “Our latest Rosalind models.” / “Prices per 1M tokens.”） | 表行：“gpt-rosalind-discovery$5.00$0.50$25.00”（输入、缓存输入、输出）；同表另有 “gpt-rosalind-research” 同价 | 无 | 页面无日期；取数 10/11 北京 15:54 | 已找到 |
| D8-b 资格限制 | L1 | 同上 | “Billing for gpt-rosalind-research begins on October 5, 2026. … Access is limited to approved internal research through the trusted-access program.” | 无 | 脚注点名的是 gpt-rosalind-research；脚注是否也适用于 discovery，页面未写明（**不确定**，如实注明） | 部分支持（价格存在；资格脚注只点名 research） |
| D8-c OpenAI 发布说明中有无 gpt-rosalind-discovery | L1 | https://developers.openai.com/blog/rosalind-workbench ；https://openai.com/index/introducing-gpt-rosalind/ （403） | Rosalind Workbench 页只写 “GPT-Rosalind, our dedicated life sciences model”，列出 “Request GPT-Rosalind access”，无 discovery 字样；openai.com 页 403 未取得 | 无 | WebSearch 得到的第三方价格页（L5）同样只提 gpt-rosalind-research，不作依据 | 未找到一手来源（discovery 的发布说明） |

## HN 与 X 热度（取数时刻 16:01 北京时间，见 d-heat.json）

| 条目 | 帖子（HN 或 X） | 分数 / 评论（HN）或 浏览 / 点赞（X） | 发帖时刻（北京） | 备注 |
|---|---|---|---|---|
| D1 纳德拉 | X 原帖（fxtwitter，取数 15:53） | views 6,959,911；likes 7,758；retweets 1,321；replies 1,120；bookmarks 8,717 | 10/10 22:43 | HN 链接帖 https://news.ycombinator.com/item?id=50035193 7 / 0，发帖 10/11 01:36 |
| D2 ARC Prize | X 原帖（fxtwitter，取数 15:53） | views 30,815；likes 80；retweets 8；replies 5 | 10/9 23:19 | HN 未检出 |
| D3 Reflection Beam | HN 50035886（FT 链接） | 162 / 102 | 10/11 02:48 | 较派工单前读数 159/101 略增；Beam 官方博文 HN 49969183 为 554 / 174（10/6 03:16） |
| D4 Mxc | HN 50016956 “Mxc: Microsoft Execution Containers version 1.0.0” | 184 / 41 | 10/9 14:52 | 博客帖 HN 50002144 为 5 / 0（10/8 13:44） |
| D5 反编译 | HN 50039092 “500B Tokens Later...” | 82 / 60 | 10/11 10:02 | 同文另有 HN 50034816 为 3 / 2（10/11 01:00） |
| D6 Cockroach | HN 50021899 | 119 / 51 | 10/9 23:26 | |
| D7 Nikon | HN 50029285（Ars）；HN 49934521（PetaPixel，10/1 报道） | 3 / 0；3 / 0 | 10/10 11:32；10/2 23:20 | 热度低 |
| D8 Rosalind | HN 47798244 “GPT-Rosalind for life sciences research”（4/16 发布） | 102 / 30 | 2026-04-16（非定价讨论） | 无 discovery 定价讨论 |

## 每条可上速览图的一句事实草稿（仅供 Claude 取舍，保留限定词）

- D1：微软 CEO 纳德拉长文称，应假定模型可能被攻破，从一开始就加以遏制（个人账号；媒体标题写作“所有 AI 模型”，原文指“一个模型”）
- D2：ARC Prize 称 TUFA Labs 在 ARC-AGI-2 取得 88.06%；规则页写奖金给首个合格解（两者冲突，榜单口径未核）
- D3：Reflection 官方：Beam 为 501B 总参数、23B 激活的模型，权重计划本月稍后发布（计划中）
- D4：微软 MXC SDK 1.0.0 稳定版发布，各隔离后端保证强度不同（MicroVM 为 experimental）
- D5：开发者称用 AI 智能体反编译射击游戏，估计耗 6000–7000 亿 token，99% 函数存在、83% 字节精确，代码不公开
- D6：Cockroach Labs 称 Db2 支持不到两天、token 账单 4172 美元，Oracle 2024 年约 9 个月、约 16 万美元人力
- D7：尼康称获奖视频不符合生成式 AI 规则并调整名次，声明称不评判参赛者声誉、科学贡献或意图（风控候选）
- D8：OpenAI 定价页列出 gpt-rosalind-discovery，输入 $5、输出 $25（每百万 token），资格脚注只点名 research 版

## 扫描说法勘误

1. **D1**：扫描称“纳德拉……称”，原文成立。但媒体标题 “all AI models are ‘compromised’” 比原文 “a model” 范围大得多，不可照搬。
2. **D2**：扫描称“所有超过 85% 的队伍分享”，这是帖子原文；ARC 规则页写“首个合格解”，二者冲突。88.06% 的榜单口径（公开/私有）未核实，官方网页未检出该数字。
3. **D3**：扫描称“金融时报报道商谈追加投资或收购”，FT 与 Reuters 正文均未取得，只有 HN 标题与 URL 线索。Beam 的参数与“later this month”原句已核实；“开放权重”需注明权重尚未发布。
4. **D4**：扫描称“Mxc 1.0.0”。博客正文没有该版本号；1.0.0 是 GitHub 的 “MXC SDK v1.0.0”。应写“SDK 1.0.0”。
5. **D5**：扫描称“约 5000 亿 token”。标题为 “500+ Billion”，正文为估计 “between 600-700 billion tokens”。两个数字都在原文中，应注明为估计。游戏名未披露。
6. **D6**：扫描列出的 “32 子任务、27 PR、55 次退回、9 次升级、2 次人类介入”。文本中仅 “32 issues” 与 “fifteen sub-issues” 可核；27、55、9、2 四项未检出，统计表缺失，需人工看原页图片后再判。“两天”“4172 美元”“9 个月”“约 16 万美元”已核实。
7. **D7**：扫描称“取消资格”，这是 Ars 标题用语；尼康官方用的是 “adjusted the competition rankings”。Ars 引文多出 “on our website”。BBC 称 Hickey “came fifth”，尼康与 Ars 称第四，应以尼康名次为准。规则页日期线与 2026 届不符，适用届次未确认。
8. **D8**：扫描称 “gpt-rosalind-discovery 输入 $5、输出 $25”，价格页已核实（另有缓存输入 $0.50）。资格脚注只点名 gpt-rosalind-research，是否适用于 discovery 未写明。发布说明未找到。

## 注记（给 Claude 的判断依据）

- 所有 HN 与 X 数字为取数时刻快照，随时间变化。
- X 平台名规则：D1、D2 的来源是 X 帖子；正文写法按 EDITORIAL 平台名规则（“纳德拉称”可；X 平台名不写进正文与卡片）。
- D7 涉及可识别个人（Ning Xu），且是风控候选；存档只用 txt，姓名照原文存。是否收录由 Develata 决定。
- D2、D3 的一手缺口见上；未找到的不写进速览。
- 本组没有截图。

## 文件清单

- 存档（文本，可入库）：d-d1-nadella.json、d-d1-nadella.txt、d-d1-verge.txt、d-d2-arc-post.json、d-d2-arc-post.txt、d-d2-arc-rules.txt、d-d2-arc-leaderboard.txt、d-d3-reflection-beam.txt、d-d4-mxc.txt、d-d4-mxc-github.txt、d-d4-mxc-releases.txt、d-d5-momo.txt、d-d6-cockroach.txt、d-d7-ars.txt、d-d7-bbc.txt、d-d7-nikon-statement.txt、d-d7-nikon-2026-comp.txt、d-d7-nikon-swim-gallery.txt、d-d7-nikon-swim-rules.txt、d-d8-pricing.txt、d-d8-rosalind-workbench.txt、d-heat.json
- 脚本（可入库）：d-html2txt.py、d-heat.py
- HTML 原件（仅本地，不入库）：d-d1-verge.html、d-d2-arc-rules.html、d-d2-arc-leaderboard.html、d-d2-kaggle-leaderboard.html、d-d3-reflection-beam.html、d-d4-mxc.html、d-d4-mxc-github.html、d-d4-mxc-releases.html、d-d5-momo.html、d-d6-cockroach.html、d-d7-ars.html、d-d7-bbc.html、d-d7-nikon-statement.html、d-d7-nikon-2026-comp.html、d-d7-nikon-swim-gallery.html、d-d7-nikon-swim-rules.html、d-d7-nikon-home.html、d-d8-pricing.html、d-d8-rosalind-workbench.html
- 日志：capture-log-d.md、evidence-d.md（本文件）
