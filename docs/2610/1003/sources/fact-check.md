# 1003 事实核验

标题（草稿）：“AI解数学难题？48%当它真人”——“AI解数学难题？”带问号，对应 Meta “Five present answers to previously open research questions”，正文吠点写明由数学家主导、部分结果他人独立得出；“48%当它真人”依据 Tavus “26 of 54 believed their partner was a real person”，正文吠点写明是 Tavus 自家 54 人、1 分钟实验，“图灵测试”是其自称。

正文：`../doc_1003_publish.txt`。取证：`evidence.md`（A–C，Codex gpt-6-astra medium，派工单 `.handoff/2026-10-03-1003-evidence.md`）。派工前 Claude 已用内置浏览器 / WebFetch 独立读取 Meta 博文全文、DAYJOB arXiv 摘要、Tavus 页面、NVIDIA VideoFDB 页面（Perception 表）；派工后对存档抽查了 DAYJOB 90% 阈值数值（PDF p.6–7）、judge 模型（PDF §4.4 与 harness README）、Tavus 研究方法段（`c-griffin.txt`），均与说法一致。

选题线索来自三份回贴扫描（北京时间 10/3 11:01、17:01 两份 ChatGPT，18:00 一份 CodeBuddy）。CodeBuddy 扫描运行时缺少自己的 SKILL.md，去重依据其私有台账而非 `daily-scan.md` 的“已发过”，来源等级也不按本号规范（媒体标 L2）；它推荐的 OpenAI 事故线中，100+ 机构 / 50PB / 7000 GPU / 日耗 50 万美元已在 1002 期发过，FTC 调查已于 1002 期核过且无一手文件；“新州 NPWS 山火数据、澳方第 6 个网站”“Asymmetric Security 统计 55 个站点、Urlquery 抹痕”在 OpenAI 中英文时间线页与网络检索中均未找到（唯一 NSW 命中是已知的 BOCSAR Crime Mapping Tool），疑似混淆，未采用。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 一：Meta 公布数学家与 Muse-Spark 合作的6篇论文 | L1 | ✅ | Meta 博文（`a-` 存档；`../images/01-meta-principles.png`） | 页面日期 “October 2, 2026”，官方未给时刻与时区，不能直接当北京日期，正文因此不写日号（反向核验意见 5）；Meta 官方 X 帖北京 10/3 03:11:30。副标题 “Mathematicians and Muse Spark collaborate on six research papers”。模型为 Muse Spark 1.1 与 1.2（Thinking Mode），“through the regular meta.ai chat interface, with no custom research scaffold”，正文因篇幅未写。 |
| 称其中5篇回答了此前开放的研究问题 | L1 | ✅ | 同上 | “Five present answers to previously open research questions.” 哪五篇官方未明确列名（evidence A1）。正文写“称”。 |
| 省流：由数学家主导；吠点①：Meta 写明由数学家主导、另一组数学家复核 | L1 | ✅ | 同上 | “A team of mathematicians guided the research and worked with Muse Spark to explore ideas and develop the arguments.”；“A second group of mathematicians then reviewed their work.” 算术物理一篇未列 Review by，不能据此推成无人审阅；正文“另一组数学家复核”是 Meta 原则条的转述，不逐篇断言。 |
| 偏微分方程那篇，选题和关键证明思路出自研究者，AI 帮助演算、试证和修改 | L1 | ✅ | 同上（`../images/03-meta-pde.png`） | “Muse Spark helped work through calculations, test possible arguments, and revise the proof, while Dinh chose the problem and key proof ideas”。“帮助”译 helped，不写“负责”。论文题为 biharmonic NLS 径向负能量解的有限时间爆破。 |
| 省流：部分结果别人已独立得出；吠点②：椭球拟合阈值另有三组8月的独立工作 | L1 | ✅ | 同上（`../images/02-meta-probability.png`）；arXiv 2608.10184、2608.12415、2608.27372 | “We also acknowledge three independent concurrent works posted in August 2026.” Misiakiewicz–Wen 独立证明 Gaussian threshold（v1 北京 8/11）；de la Cerda–Potechin–Tulsiani–Xu 证到相差趋于零的乘性因子（8/12）；Koehler–Sohn 更一般的普适性结果（8/28）。正文“三组独立工作”不细分三者强弱。 |
| 群论猜想，AI 智能体 Nilradical 9月16日报告了另一个反例 | L1 | ✅ | 同上（`../images/04-meta-group.png`）；nilradical.ai 结果页 | “We also acknowledge the AI agent Nilradical, which reported a different counterexample to the same conjecture on September 16, 2026.” Nilradical 反例 2592 阶，Meta 为 384 阶；未复跑。 |
| 非结合代数猜想另有 Hu 和 Wen 的反例 | L1 | ✅ | 同上（`../images/07-meta-algebra.png`）；arXiv（evidence A6/A7） | “We also acknowledge independent work by Hu and Wen, who reported counterexamples to the same conjecture.” Hu–Wen v1 北京 8/12。 |
| Meta 称完成期间或之后才得知 | L1 | ✅ | 同上；论文 1、6（`a-paper-1-full.txt:122`、`a-paper-6-full.txt:54`） | 博文：“After completing our work, we learned that other teams outside Meta had independently announced solutions …”；论文 1：“While completing this work, we became aware of three independent concurrent works”；论文 6：“While completing this article, we became aware of the concurrent work of Hu and Wen”。正文合写为“完成期间或之后”（反向核验意见 4）。只转述 Meta 自述，不判断优先权。 |
| 二：Surge AI 的基准 DAYJOB 含医疗50题、金融80题 | L1 | ✅ | arXiv 2610.01306（`b-` 存档；`../images/14-dayjob-abstract.png`）；Surge 博客 surgehq.ai/blog/dayjob（`b-surge-blog.txt`）；github.com/surge-ai/dayjob | “130 tasks built by professionals in healthcare (50) and finance (80)”。博客标 September 23 / 26, 2026（基准首次公开早于论文）；论文 v1 08:39:10 UTC = 北京 10/1 16:39:10。初稿写“10月1日发布基准”，反向核验指出混淆了论文提交与基准公开，已删日期。15 名作者均署 Surge AI。属“窗口前”。 |
| 吠点②：耗时是估计，且口径不一：论文写医疗、金融平均13.6、16.6小时，Surge 博客写19.6、21.6小时 | L1 | ✅ | 论文摘要与 §3.4 p.4；Surge 博客（`b-surge-blog.txt:141–146`） | 论文：“The tasks are estimated to take a professional 13.6 hours on average in healthcare and 16.6 in finance.” 博客：“A DAYJOB Finance task is estimated to take a human professional 21.6 hours on average; Healthcare averages 19.6 hours.” 两处都写 estimated。医疗为出题人从 8 个时长区间中选择、取区间中点（>40 小时计 40）；金融 16.6 引 dataset card，估计人身份未同等写明，正文因此不写“出题人”（反向核验意见 7）。差异原因未见说明，正文不解释。 |
| 每题跑5次，剔除运行错误后按题平均，榜首 Opus-5.5（max 档）在医疗、金融两项中全部标准达标的占24.7%、23.9% | L1 | ✅ | 同上，摘要、§4.2–4.4 与主结果表 p.6（`../images/15-dayjob-results-table.png`；`b-leaderboard_*.csv`） | 配置名 “Claude Opus 5.5 (adaptive/max)”；指标 strict pass@1（τ=1）：“five attempts per task”；“Attempts that end in an error, such as a time-out or a provider failure, are excluded from their task’s mean”，先算每题有效尝试的通过比例，再对题目等权平均（反向核验意见 2 补入正文）。30 个配置、13 家开发商。 |
| 吠点①：每题有几十条判分标准，一条不满足即失败 | L1 | ✅ | 同上，摘要与 §4.2 | “binary criteria (median 47.5 and 57.5 per task)”；“An attempt passes only if it meets every criterion.” 论文表注称金融统计描述 50 道公开题，数据卡把 57.5 列在 80 题全集概览下，范围不一致（反向核验意见 6），正文改写为“几十条”不引中位数。 |
| 放宽到满足90%，Opus-5.5 为62.4%、58.3%，但漏掉的可能正是关键一条 | L1 | ✅ | 同上 §4.4、§5 p.6–7、CSV | “At 90% the leader reaches 62.4% and 58.3%, and the median configuration 6.4% and 10.4%”。CSV：pass90 62.4 / 58.25。“Criteria are not weighted by importance, so a lower threshold accepts attempts that miss a decisive one.”（反向核验意见 3：只给放宽后的数会反向淡化失败）。 |
| 评委是 Claude-Opus-4.8 | L1 | ✅ | 同上 §4.4 p.5；harness README（`b-harness.txt`） | “the OpenCode agent … run by rewardkit 0.1.7 with Claude Opus 4.8 as its model”；harness：“Grading uses Claude Opus 4.8 as an agentic LLM judge”。 |
| 论文未报告本研究中评委与人工的一致率 | L1 | ✅ | 同上，全文检索 | 全文只在相关工作中引用“Strong model judges can match the agreement between human raters on open-ended conversation [34]”（他人研究，开放式对话场景）；未见本基准 judge 与人工评分的一致率、抽检或 kappa。Claude 对 PDF 文本检索 agree / validat / manual / kappa / judge 复核。只有 Kendall τ 用于不同阈值排名一致性，不是人机一致率。 |
| 三：Tavus 称……并称这是首个通过“视频图灵测试”的模型 | L1 | ✅ | tavus.io/griffin（`c-griffin.txt`；`../images/17-griffin-results.png`） | 页面日期 October 1, 2026，未给时区，正文不写日号；官方 X 帖北京 10/2 00:59:30（= 美西 10/1 09:59:30）。“Griffin is the first model to pass the real-time, video Turing test.”；“to the best of our knowledge, the first model to have ever passed the video Turing test”。正文写“称”。 |
| 54人与它视频通话1分钟后，26人（48%）认为对方是真人 | L1 | ✅ | 同上 | “26 of 54 believed their partner was a real person”；“48% (n=54)”。Tavus 自述，无平台名称、原始问卷、置信区间。 |
| （未入正文，因篇幅删去）只向受信任的测试者开放 | L1 | ✅ | 同上（`../images/19-griffin-availability.png`） | “Griffin-Lite will not be available for use for customers at this time, though it is available for select trusted testers as a research preview.” |
| 吠点①：受试者事先被告知对方是另一名参与者，问卷最后才问是否想过对方不是真人 | L1 | ✅ | 同上（`../images/18-griffin-method.png`） | “Participants were told they would be matched with another participant for a one-minute video call …”；“Only at the end of the survey were participants asked whether it had crossed their mind that their partner might not be a real person, and if so, when.” 48% 来自通话后“whether that partner had been a real person”这一问。正文“‘图灵测试’是 Tavus 的叫法”不引入图灵 1950 原文（Codex 未取得原文，受验证页阻挡）。 |
| 实验由 Tavus 自己做，未披露置信区间 | L1 | ✅ | 同上 | “For every new model, we run a study with participants recruited through an independent research platform”——“independent” 修饰招募平台，研究由 Tavus 进行。页面无 CI 或统计检验。 |
| 吠点②：NVIDIA VideoFDB 中，Griffin-Lite 居非人系统第一，但感知总分3.73，低于真人参照4.20 | L3 | ✅ | research.nvidia.com VideoFDB 页（`../images/20-videofdb-perception.png`） | Perception 表 Overall：Human reference 4.20；Tavus Griffin Lite 3.73，为被测模型最高。Generation 表 Overall：human 3.92、Griffin 3.83（`21-`），正文未写；Generation Affect 子项 Griffin 4.40 高于 human 4.14。论文 v1 未含 Griffin 行，Griffin 数据仅见当前网页。VideoFDB 由 NVIDIA 研究者发布，未见 Tavus 参与。 |
| 该评测由语言模型打分，与48%不是同一指标 | L3 | ✅ | 同上 | “a rubric-based LM-as-judge framework”；“Three rubric axes per category, each scored 0–5 by an LM judge”。正文未写的 NVIDIA 原话：“Current models remain well below human conversational naturalness.” |

## 未入正文 / 待核

- Tavus 官方 X 帖：抓取时约 1983 万浏览、3.9 万赞、9750 转帖（瞬间值）；Protos 10/2 报道写 “more than 13 million views”，且误写 “of 26 people, 48% … 28 said it was AI”（实际 26/54）。帖下 Community Note 为“提议、征求评分”状态，未正式显示，不能写成已获认可。
- DAYJOB 每次尝试成本：Opus-5.5 医疗 7.10 美元、金融 11.47 美元（不含 judge）；GPT-6-Astra（max）严格通过率 11.6% / 21.5%。正文因篇幅未写。
- DAYJOB “AI 只能做 24% 的工作”类夸大措辞：Codex 未取得原站实例，正文不声称“网传”。
- Meta 六篇论文页边以 Human / AI 标注，AI 指 “AI-assisted material”；六篇官方公开日均为 10/2，未找到 arXiv 提交历史。

## 反向核验

第 1 轮正文审查：`codex-reviewer`（WSL Codex gpt-6-astra，effort medium，只读，约 404 秒），审的是已含“尝试中”“并称这是首个”的版本（SHA-256 前缀 f2ebd88ca9439）。结论 PASS、无 BLOCKER；4 项 SHOULD-FIX、3 项 NEEDS-VERIFICATION，Claude 逐条对存档核实后全部采纳：

1. “10月1日 Surge AI 发布基准”混淆论文提交与基准公开（博客 9/23、9/26）→ 删日期。
2. 通过率分母漏了“剔除运行错误、按题平均”→ 补入。
3. 只给 90% 阈值数会淡化失败（判分项不加权，可能漏掉关键一条）→ 吠点①句末补“但漏掉的可能正是关键一条”。
4. “Meta 称完成后才得知”与两篇论文“While completing”不一致 → 改“完成期间或之后”。
5. Meta、Tavus 页面日期无时区，不能当北京日期 → 删两处日号。
6. 金融 57.5 条的统计范围在论文与数据卡间不一致 → 改“几十条”。
7. 金融估计人身份未写明，且 Surge 博客耗时 19.6/21.6 小时与论文 13.6/16.6 小时冲突 → 删“出题人”，把冲突本身写成吠点②。

审查者确认无误的句子：省流三句、Meta 吠点①全文、三组 8 月工作 / Nilradical / Hu–Wen、24.7/23.9 与 62.4/58.3、Claude-Opus-4.8 评委与“未报告一致率”、Tavus 26/54 与方法段、VideoFDB 3.73/4.20。审查者提醒：不要把 PDE 篇的分工推广到其他论文（群论搜索程序、代数反例、算术物理候选证明都有明确 AI 贡献）；正文只把该分工限定在“偏微分方程那篇”。

视觉复核：WSL Codex gpt-6-astra medium（只读），附省流卡、4 张批注图及其 -raw 底图、两张封面。结论 1 项 BLOCKER、1 项 SHOULD-FIX、1 项 NICE-TO-HAVE，Claude 核实后处理：

1. BLOCKER：省流卡 DAYJOB 条目两个百分比没写对应领域 → 正文与卡片改为“在医疗、金融两项中全部标准达标的占24.7%、23.9%”；统计口径（每题5次、剔除运行错误、按题平均）留在正文同段，卡片版面放不下。
2. SHOULD-FIX：31 号图高亮漏了“Across 30 model configurations from 13 developers”比较范围 → 高亮扩到该分句，译注补“在13家开发商的30种模型配置中”。
3. NICE-TO-HAVE：31 号图标题与按钮占面积、摘要字偏小 → 未改：去掉中段按钮需拼接底图，违反“原图像素不改”；保留论文身份上下文。

复核者确认：30、32、33 高亮与译注正确；省流卡 Meta、Tavus 两条保留了“数学家主导”“Tavus 称”“54人、1分钟”；两张封面无额外文字、错字或 logo，横版居中 1:1 裁切后标题、副标题、看板娘完整，竖版缩略图标题可读。修改后已重渲并逐张目视核对。

最终全文 992 字（lint 0 错误 0 警告），标题 15 字。
