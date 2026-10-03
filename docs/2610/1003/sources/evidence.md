# 1003 期取证清单

仅取证，不是发布稿、数学证明验收或 benchmark 复跑。抓取日期：北京时间 2026-10-03。逐次时间见 capture-log.md 及 d-capture-records.jsonl。官方只给日期时不补时刻、不擅自换日。引文仅合并换行/空白；PDF 断词及公式以原件为准。L1–L7 依 EDITORIAL.md；状态表示取得证据的程度，不表示独立验证厂商结论。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1 博文标题、日期、模型、原则与五篇口径 | L1 | https://research.meta.ai/blog/solving-open-research-problems-together | Solving Open Research Problems Together；October 2, 2026；完整逐字摘句见 A1 节 | [01](../images/01-meta-principles.png) | 官方未给时刻/时区 | 已找到 |
| A2 六篇原件、作者、日期、起草标记 | L1 | 下方六篇官方摘要及 PDF | margin markers identify human-drafted (Human) and AI-assisted material (AI) separately. | [08](../images/08-meta-paper-1-labels.png)、[09](../images/09-meta-paper-2-labels.png)、[10](../images/10-meta-paper-3-labels.png)、[11](../images/11-meta-paper-4-labels.png)、[12](../images/12-meta-paper-5-labels.png)、[13](../images/13-meta-paper-6-labels.png) | 六摘要、六 PDF 已存；官方公开日均 10-02，未找到六篇 arXiv 提交历史，不能将公开日写成提交日 | 部分支持 |
| A3 概率三组并行工作及阈值边界 | L1 | Meta 博文；https://arxiv.org/abs/2608.10184；https://arxiv.org/abs/2608.12415；https://arxiv.org/abs/2608.27372 | We also acknowledge three independent concurrent works posted in August 2026.；完整摘句见下方 | [02](../images/02-meta-probability.png) | 三组 v1 日期见 A7；exactly at the threshold 尚未解决 | 已找到 |
| A4 PDE 人类选题/思路及 2015 问题 | L1 | Meta 博文；https://arxiv.org/abs/1503.01741 | Muse Spark helped work through calculations, test possible arguments, and revise the proof, while Dinh chose the problem and key proof ideas, and a separate pair of mathematicians reviewed and helped refine the work. | [03](../images/03-meta-pde.png) | 旧文 v2 p.8；新结果限定 N≥2、径向、负能量等 | 已找到 |
| A5 384 阶、GAP 与 Nilradical | L1 | Meta 博文；https://nilradical.ai/results/kourovka-21-68/ | Muse Spark generated the search program in GAP, a mathematical software system, that found the counterexample.；We also acknowledge the AI agent Nilradical, which reported a different counterexample to the same conjecture on September 16, 2026. | [04](../images/04-meta-group.png) | Nilradical 结果页和固定 commit 已存；其反例 2592 阶，非 Meta 的 384 阶；未复跑 | 已找到 |
| A6 Hu–Wen、算术物理起草与 review 名单 | L1 | Meta 博文；https://arxiv.org/abs/2609.25023 | We also acknowledge independent work by Hu and Wen, who reported counterexamples to the same conjecture.；Beyond helping identify the connection, Muse Spark generated candidate proofs and drafted three core technical sections, which the researchers then checked, corrected, and refined. | [06](../images/06-meta-arithmetic.png)、[07](../images/07-meta-algebra.png) | 算术物理未列 Review by；不能推成无人审阅；Hu–Wen 日期见 A7 | 已找到 |
| A7 并行工作日期时间线 | L1 | 下方 A7 表 | 以 Submission history、项目结果页、固定 Git commit 为准 | — | commit 时间不证明首次公开可见时间；不判断优先权 | 已找到 |
| A8 Meta 热度及数学圈讨论 | L1/L5/L6 | https://x.com/AIatMeta/status/2106099776035152231；https://aihot.news/；https://news.ycombinator.com/item?id=49942159 | AI评分 | — | X 卡片 264K views、955 likes、153 reposts；AIHOT 为 73 分；Threads/Tao/mathstodon 未取得目标帖 | 部分支持 |
| A9 实际传播中的“独立/首次/六大问题”措辞 | L2/L5/L6 | 下方传播实例表 | 毕树超官宣：Meta AI连破6大世界猜想，数学界AlphaGo时刻来了！ | — | 已存传播页；未找到符合要求的逐字“AI 独立解决 5 个数学难题”实例 | 部分支持 |
| B1 摘要与 submission history | L1 | https://arxiv.org/abs/2610.01306 | We introduce DAYJOB, a benchmark of 130 tasks built by professionals in healthcare (50) and finance (80). | [14](../images/14-dayjob-abstract.png) | v1 北京 2026-10-01 16:39:10（08:39:10 UTC）；摘要其余关键句见 B1 节 | 已找到 |
| B2 全部主结果与软阈值 | L1 | https://arxiv.org/html/2610.01306v1；https://arxiv.org/pdf/2610.01306 | strict pass@1 | [15](../images/15-dayjob-results-table.png) | PDF p.6 整页，表头/effort/30 行/表注齐全；下方逐行抄录及原始 CSV | 已找到 |
| B3 pass、criteria、judge、尝试与一致率 | L1 | 同 B1，PDF §§4.2–4.4 p.5 | We evaluate 30 configurations from 13 developers with five attempts per task. | — | Claude Opus 4.8 judge；5 次/题；error 排除；中位 criteria 47.5/57.5；未找到本研究人工一致率 | 部分支持 |
| B4 人类耗时估计 | L1 | 同 B1，§3.4 p.4 | Task creators estimated the time a professional would need for each healthcare task, choosing one of eight ranges from 1–2 hours to more than 40 hours | — | 医疗分箱中点、>40 取 40；金融 16.6 引 dataset card，未披露同等详细估法；不是实测 | 部分支持 |
| B5 错误前提/输入贯穿分析的个案 | L1 | 同 B1，§6 p.8 | 下方逐字摘录 Qwen check-processing 个案 | [16](../images/16-dayjob-case.png) | 作者个案叙述；本次未复跑 | 已找到 |
| B6 作者机构、数据代码、人员 | L1 | 同 B1；https://github.com/surge-ai/dayjob | We release all healthcare tasks, 50 of the 80 finance tasks, the evaluation harness, and the leaderboard. | — | 15 作者均列 Surge AI；专业人员总数未披露；公开 50 医疗+50/80 金融；harness Apache-2.0，数据 MIT | 部分支持 |
| B7 热度与“24% 工作”实例 | L5/L6 | https://aihot.news/all?q=DAYJOB&tab=relevance；https://news.ycombinator.com/ | 未取得符合目标措辞的原站逐字实例 | — | AIHOT 两种检索为空；HN/X 有限检索未得可归属的有效热度；不代表全网无讨论 | 未找到一手来源 |
| C1 方法、48%、对照、可用性 | L1 | https://www.tavus.io/griffin | Participants were told they would be matched with another participant for a one-minute video call to discuss what they were looking forward to this year. | [17](../images/17-griffin-results.png)、[18](../images/18-griffin-method.png)、[19](../images/19-griffin-availability.png) | 全段及限制见 C1；公司自述，n=54；无平台名称/原始问卷/CI/检验 | 部分支持 |
| C2 VideoFDB 两表、方法、作者、日期 | L3 | https://research.nvidia.com/labs/amri/projects/video-fdb/；https://arxiv.org/abs/2605.30256 | Current models remain well below human conversational naturalness.；No evaluated agent approaches the human reference on VideoFDB. | [20](../images/20-videofdb-perception.png)、[21](../images/21-videofdb-generation.png) | 当前网页两表全图；Griffin 行不在已存论文中，不能倒推 v1 已测 Griffin | 已找到 |
| C3 X 互动、Note 与 Protos | L1/L5 | https://x.com/tavus/status/2105704169009246248；https://protos.com/tavus-call-bot-sparks-ai-scam-psychosis-fears/ | Yesterday, in an X post that garnered more than 13 million views, Tavus revealed footage of Griffin realistically talking to company founder, Hassaan Raza. | [22](../images/22-tavus-x-note.png) | 当前约 1983 万；Note 显示提议且征求评分，未核到正式 helpful 状态 | 部分支持 |
| C4 Turing 1950 原始设置 | L1 | https://academic.oup.com/mind/article/LIX/236/433/986238 | 未取得可存档核对的机器替换段原文，不凭记忆补引文 | — | OUP 403/Cloudflare；Turing Archive 入口未成功读取；未用转载或镜像 | 未找到一手来源 |
| C5 中英文传播措辞 | L5/L6 | 下方传播实例表 | AI数字人工具横评2026：Tavus 通过视频图灵测试，HeyGen/Synthesia/D-ID 对比 | — | 中文标题确有“通过”；英文 Tech-ish 有限定与限制，不能直接判成夸大 | 部分支持 |

## A1：逐字摘句

标题：Solving Open Research Problems Together

副标题：Mathematicians and Muse Spark collaborate on six research papers

日期：October 2, 2026（官方未给时刻/时区）。以下来自 a-meta-blog.txt：

> They used both Muse Spark 1.1 and 1.2 in Thinking Mode through the regular meta.ai chat interface, with no custom research scaffold.

> A team of mathematicians guided the research and worked with Muse Spark to explore ideas and develop the arguments.
> A second group of mathematicians then reviewed their work.
> Each paper clearly marks which passages were primarily drafted by researchers and which were drafted by AI.
> Each paper gives credit to the earlier research and mathematical ideas it builds on.

> Today, we're sharing six such papers from that collaboration. Five present answers to previously open research questions.

> After completing our work, we learned that other teams outside Meta had independently announced solutions to some of the same problems using different approaches. We recognize and appreciate their contributions and clearly acknowledge their work and how it relates to ours in the papers.

概率段：

> We also acknowledge three independent concurrent works posted in August 2026. Misiakiewicz and Wen independently proved the Gaussian threshold. De la Cerda, Potechin, Tulsiani, and Xu established the Gaussian threshold up to a vanishing multiplicative factor. Koehler and Sohn obtained a broader universality result that includes the Gaussian threshold as a special case. These works and ours were developed independently and use different approaches.

> It gives researchers a theoretical benchmark for the limits of exact data fitting, while the behavior exactly at the threshold remains unresolved.

PDE：

> This settles a question left open in 2015 and confirms a prediction from computer simulations in 2002 for this setting, giving researchers a clearer understanding of when wave collapse is unavoidable.

群论：

> The team found that exception in a group with 384 elements, showing that the two properties do not always go together.

## A2：六篇原件、作者与标注

六个官方摘要页均写 October 02, 2026，未给时刻/时区或提交历史。本次没有找到这六篇的 arXiv 版本。下载入口与实际官方 CDN 原件 URL 见 [a-paper-downloads.json](a-paper-downloads.json)；签名参数是下载参数，不能像 utm 参数一样全部去掉。

| 篇目 | 作者（PDF） | 官方摘要与本地原件 | 标注截图/PDF 页 | 单列 review |
|---|---|---|---|---|
| The Strict Threshold for Gaussian Ellipsoid Fitting | Aykut Arslan | [官方](https://ai.meta.com/research/publications/the-strict-threshold-for-gaussian-ellipsoid-fitting/)；[PDF](a-paper-1-full.pdf)；摘要 a-paper-1.html | 08，p.5 | Babak Modami; Alexander Roitershtein; Mark Sepanski; Grigory Sokolov |
| Finite-Time Blow-Up of Radial Negative-Energy Solutions for the Mass-Critical Biharmonic Nonlinear Schrödinger Equation | Leonard Dinh | [官方](https://ai.meta.com/research/publications/finite-time-blow-up-of-radial-negative-energy-solutions-for-the-mass-critical-biharmonic-nonlinear-schrodinger-equation/)；[PDF](a-paper-2-full.pdf)；摘要 a-paper-2.html | 09，p.1 | Fazel Hadadifard; Salem Selim |
| Semiabelian Groups Need Not Be Monomial | Joseph Phillip Brennan; Milana Golich | [官方](https://ai.meta.com/research/publications/semiabelian-groups-need-not-be-monomial/)；[PDF](a-paper-3-full.pdf)；摘要 a-paper-3.html | 10，p.4 | Andres Barei; John Portin |
| Tightness of the Cycle-Based Relaxation for Completed Length-Three Alpha-Cycles | Aykut Arslan | [官方](https://ai.meta.com/research/publications/tightness-of-the-cycle-based-relaxation-for-completed-length-three-alpha-cycles/)；[PDF](a-paper-4-full.pdf)；摘要 a-paper-4.html | 11，p.3 | Kien Trung Le |
| String Two-Point Function = Height Function on a Curve | Anindya Dey; Gabriel Herczeg; An Huang; Nicolas Jaramillo Torres; Jacob H. Swenberg | [官方](https://ai.meta.com/research/publications/string-two-point-function-height-function-on-a-curve/)；[PDF](a-paper-5-full.pdf)；摘要 a-paper-5.html | 12，p.2 | 未单列；致谢反馈不能自动补作正式 reviewer |
| On Solvable Evolution Algebras and a Conjecture by García-Martínez and Pérez-Rodríguez | Andres Barei（摘要 Written by 为 Andres Barei Bueno） | [官方](https://ai.meta.com/research/publications/on-solvable-evolution-algebras-and-a-conjecture-by-garcia-martinez-and-perez-rodriguez/)；[PDF](a-paper-6-full.pdf)；摘要 a-paper-6.html | 13，p.2 | Nicolás Jaramillo Torres |

六篇都有页边 Human / AI 标记；Statement of AI Use 写：

> This paper was developed through collaboration between researchers and Muse Spark (AI model) via the meta.ai chat interface. The model helped explore ideas, develop candidate arguments, and draft material. Researchers guided the work, checked and corrected the mathematics, and take ownership for the final manuscript. Following [Sch25], margin markers identify human-drafted (Human) and AI-assisted material (AI) separately.

这里 AI 的定义为 assisted，不能自动换成“未经人类修改的 AI 原稿”。各篇 Concurrent work 原文如下，完整段落与页码可查各自 `*-full-keypages.json` 和 PDF；PDF 抽取的跨行断词合并，数学符号以原 PDF 为准。

1. 概率（p.3，§1 后 Concurrent work）：

> While completing this work, we became aware of three independent concurrent works. Misiakiewicz and Wen [MW26], posted on August 10, 2026, proved the sharp d²/4 threshold for Gaussian ellipsoid fitting. De la Cerda, Potechin, Tulsiani, and Xu [dlCPTX26], posted on August 12, 2026, established the Gaussian threshold up to a vanishing multiplicative factor. Koehler and Sohn [KS26], posted on August 27, 2026, proved a more general universality result for independent subgaussian coordinates with a common fourth moment, recovering the Gaussian 1/4 threshold as a special case.

2. PDE（p.6，Statement of AI Use 后）：

> We are not aware of any concurrent work.

3. 群论（p.2）：

> While in the process of completing this work, the authors became aware of an additional counterexample provided on September 16, 2026 by the AI agent Nilradical [Nil26]. In this article, we offer both a minimal counterexample and also theoretical results that were done independently from that of Nilradical.

4. 优化（p.3）：

> We are not aware of any concurrent work on this topic.

5. 算术物理（p.2）：

> We are not aware of any concurrent work on this topic.

6. 非结合代数（p.2）：

> While completing this article, we became aware of the concurrent work of Hu and Wen ([HW26]), submitted on August 12, 2026. Both works introduce counterexamples refuting Conjecture 1.1. This article also presents additional theoretical results of independent interest.

## A7：日期时间线（不作优先权判断）

| 记录 | 北京日期 | 原日期/时区 | 一手来源 |
|---|---|---|---|
| Misiakiewicz–Wen v1 | 2026-08-11 | 2026-08-10 UTC | https://arxiv.org/abs/2608.10184 |
| de la Cerda–Potechin–Tulsiani–Xu v1 | 2026-08-12 | 2026-08-12 UTC | https://arxiv.org/abs/2608.12415 |
| Hu–Wen v1 | 2026-08-12 | 2026-08-12 UTC | https://arxiv.org/abs/2609.25023 |
| Koehler–Sohn v1 | 2026-08-28 | 2026-08-27 UTC | https://arxiv.org/abs/2608.27372 |
| Nilradical statement accepted | 官方未给时区，不能换算 | 2026-09-16，未给时刻 | https://nilradical.ai/results/kourovka-21-68/ |
| Nilradical 固定 commit | 2026-09-16 | 2026-09-16 UTC | https://github.com/alunik/kourovka-lean/commit/5a6b2c18e326b7b0281f00b64cade629acbfe1f5 |
| Meta 六个官方摘要页 | 官方未给时区，不能换算 | 2026-10-02，官方未给时刻 | 上方六个官方页面 |

时刻核对附记：四个 arXiv v1 依表顺序为北京 08-11 03:55:11（08-10 19:55:11 UTC）、08-12 12:09:06（04:09:06 UTC）、08-12 21:38:05（13:38:05 UTC）、08-28 01:09:38（08-27 17:09:38 UTC）。Hu–Wen 的 2609 编号与页面所列 8 月 history 不同；照录页面，不用编号推算。Git commit 为北京 09-16 18:37:31（10:37:31 UTC），不是首次公开可见性的证明。

四篇作者分别为 Theodor Misiakiewicz / Garrett G. Wen；Sofia de la Cerda / Aaron Potechin / Madhur Tulsiani / Jeff Xu；Xing-Yu Hu / Ran Wen；Frederic Koehler / Youngtak Sohn。Nilradical 结果页标注 “A project by Aluna Rizzoli”，GitHub alunik 用户 API 与之吻合。固定 README 与结果页均已存；本次未重跑 Lean/GAP 或核验最小阶数。

## 原始问题与“五篇”的范围

| 项目 | 已取得的出处 | 缺口/条件 |
|---|---|---|
| 概率 | 新论文 p.2 引 Saunderson, Parrilo, Willsky [SPW13] 的 n=d²/4 猜想，原始参考文献在 PDF 文末 | 未另取得 2013 原文；新论文转述不算独立核对原始问题 |
| PDE | [Boulenger–Lenzmann](https://arxiv.org/abs/1503.01741)，2015 v2 p.8：“Furthermore, it seems natural to conjecture that finite-time blowup always occurs in the setting of Theorem 3, at least in sufficiently high dimensions.” | 2017 是期刊发表年；新结果限定径向、负能量、N≥2 等 |
| 群论 | Kida, On semiabelian groups，DOI 10.1515/jgth-2024-0010；Crossref online 2024-11-09；新论文 Conj.1.2 引 Kida Conj.1.3 | 出版社 HTTP 202 空响应；原问题正文未取得。2025 期刊年与 2024 online 可并存 |
| 优化 | [Del Pia–Khajavirad](https://arxiv.org/abs/2507.12831)，v2 §5.2 p.28，逐字问题如下 | v1 2025-07-17、v2 2026-06-10；v1 未匹配到同句，不能用论文首版年份直接反驳“2026 提出” |
| 非结合代数 | García-Martínez–Pérez-Rodríguez, A note on complete evolution algebras，DOI 10.1007/s00013-026-02251-0；Crossref online 2026-04-22；新论文 Conj.1.1 转述 | 原出版页返回 challenge，未取得原问题正文；范围是复数域有限维 evolution algebra，不是任意域任意代数 |
| 算术物理 | 新原件 Introduction 从 Tate curve 推广到 p-adic local field 上 semistable reduction 的曲线；[Man87] 是 1987 方向性工作 | 未找到官方逐项列明哪篇不计入 five 的句子 |

优化问题原句：

> We leave open the question of whether the generalized triangle inequalities, together with the complete edge relaxation, characterize the multilinear polytope of the support hypergraph of an α-cycle of length three.

**待确认的映射：** 博文概率、PDE、群论、优化、非结合代数五节分别使用 answers/disproves a question/conjecture；算术物理节描述推广和连接。这支持一个“五篇＝这五项”的候选解释，但官方没有显式排除名单，不能作为已核实分类。算术物理原件也提出并回答自己的研究问题。

## B1：摘要关键句

> The tasks are estimated to take a professional 13.6 hours on average in healthcare and 16.6 in finance.

> Each task is a containerized Harbor environment with an expert rubric of binary criteria (median 47.5 and 57.5 per task) that an agentic judge applies to the delivered files, and an attempt passes only if it meets every criterion.

> Across 30 model configurations from 13 developers, the strongest, Claude Opus 5.5, passes 24.7% of healthcare and 23.9% of finance attempts, and the median configuration passes 0.6% and 2.5%.

> In case studies, agents accept premises that the record contradicts and carry wrong inputs through otherwise consistent analyses.

## B2：主结果全表（Table 3，PDF p.6）

以下为 strict pass@1 百分比；effort 保留括号，未列档位的不补。表注与全页原图见 15；HTML 原表见 b-dayjob-tables.json。五次/题、错误排除、题间等权等方法见下一节。

| Configuration / effort | Developer | Healthcare % | Rank | Finance % | Rank |
|---|---|---:|---:|---:|---:|
| Claude Opus 5.5 (adaptive/max) | Anthropic | 24.7 | 1 | 23.9 | 1 |
| GPT-6 Astra (max) | OpenAI | 11.6 | 2 | 21.5 | 2 |
| Claude Fable 5.1 (adaptive/max) | Anthropic | 9.6 | 3 | 19.8 | 3 |
| Grok 4.7 (xhigh) | xAI | 8.4 | 4 | 14.5 | 5 |
| Muse Spark 1.3 (max) | Meta | 7.8 | 6 | 14.8 | 4 |
| Claude Opus 5 (adaptive/max) | Anthropic | 8.4 | 4 | 11.3 | 7 |
| Grok 4.6 (xhigh) | xAI | 2.8 | 9 | 12.8 | 6 |
| Claude Fable 5 (adaptive/max) | Anthropic | 5.6 | 7 | 9.0 | 9 |
| GPT-6 Sol (max) | OpenAI | 3.6 | 8 | 9.3 | 8 |
| GPT-5.6 Sol (max) | OpenAI | 1.6 | 12 | 5.5 | 11 |
| GLM 5.3 (max) | Zhipu AI | 0.4 | 16 | 6.0 | 10 |
| Qwen 3.8 Max (xhigh) | Alibaba Cloud | 2.0 | 10 | 3.0 | 13 |
| Kimi K3 (max) | Moonshot AI | 1.2 | 14 | 3.8 | 12 |
| Muse Spark 1.2 (xhigh) | Meta | 2.0 | 10 | 2.3 | 16 |
| Gemini 3.8 Flash (high) | Google | 0.8 | 15 | 2.8 | 14 |
| GPT-5.6 Terra (xhigh) | OpenAI | 1.6 | 12 | 1.8 | 20 |
| Gemini 3.7 Flash (high) | Google | 0.0 | 21 | 2.8 | 14 |
| GLM 5.3 Flash (max) | Zhipu AI | 0.4 | 16 | 2.3 | 16 |
| Claude Sonnet 5 (adaptive/max) | Anthropic | 0.0 | 21 | 2.3 | 16 |
| GPT-6 Luna (max) | OpenAI | 0.0 | 21 | 2.3 | 16 |
| GPT-5.6 Luna (max) | OpenAI | 0.4 | 16 | 1.8 | 20 |
| DeepSeek V4 Pro (max) | DeepSeek | 0.4 | 16 | 1.0 | 22 |
| Hy3 (high) | Tencent | 0.4 | 16 | 0.3 | 23 |
| DeepSeek V4 Flash (max) | DeepSeek | 0.0 | 21 | 0.0 | 24 |
| Gemini 3.1 Pro (high) | Google | 0.0 | 21 | 0.0 | 24 |
| Inkling (max) | Thinking Machines | 0.0 | 21 | 0.0 | 24 |
| Kimi K2.7 Code (max) | Moonshot AI | 0.0 | 21 | 0.0 | 24 |
| Mistral Large 3 | Mistral AI | 0.0 | 21 | 0.0 | 24 |
| Muse Glimmer 30B (xhigh) | Meta | 0.0 | 21 | 0.0 | 24 |
| Nemotron 3 Ultra | NVIDIA | 0.0 | 21 | 0.0 | 24 |

### 软阈值

官方 [医疗 ancillary CSV](https://arxiv.org/src/2610.01306v1/anc/data/leaderboard_healthcare.csv) 与 [金融 ancillary CSV](https://arxiv.org/src/2610.01306v1/anc/data/leaderboard_finance.csv) 已原样保存为 b-leaderboard_healthcare.csv、b-leaderboard_finance.csv，**包含全部 30 配置的 strict、N−1、95%、90% 门槛数值**，及 cost/tokens。它们是不同门槛下的通过率，不是平均工作量完成比例。

| Opus 5.5 门槛 | Healthcare % | Finance % |
|---|---:|---:|
| 全部 criteria（strict） | 24.7 | 23.9 |
| 最多失败一项（N−1） | 37.8 | 32.9 |
| 满足 ≥95% criteria | 48.3 | 42.5 |
| 满足 ≥90% criteria | 62.4 | 58.3（原 CSV 58.25） |

§5.2 的 median configuration 在 90% 门槛为 6.4% / 10.4%。未找到所有 criteria 的平均满足比例，不能将上述数字直接转成“完成了多少工作”。

## B3–B6：方法、任务来源、利益相关与个案

- §4.2 p.5：每配置每任务五次；provider rate-limit error 最多重试三次；timeout/provider failure 等 error 从该题平均值排除。不是五次成功一次就算通过的 pass@5。
- §4.3 p.5：judge 是 OpenCode 1.18.31，由 rewardkit 0.1.7 驱动 Claude Opus 4.8；可读文件、运行代码、读 trajectory，每项二元判定附证据；90 秒/criterion，至少 40 分钟。没有找到本研究 judge 对人工一致率实验。
- §4.4 p.5：先题内有效尝试平均，再题间等权；criteria 不按重要性加权；全满足才 strict pass。医疗/金融中位项数 47.5/57.5。
- §4.1：OpenHands SDK 1.43.1、Harbor 0.22；最多 1000 turns、6h、600s/model call；网络仅 provider API。
- §3.4 p.4：医疗由 task creator 选八个时间区间，用中点求平均，>40h 按 40h；原分箱 b-human_time_ranges.csv。金融 16.6h 引 dataset card，未披露同样细的估法。不是实测人类工作耗时。
- §3.5：任务来自有相关一线经验的专业人员，经过多层审核；未披露独立专业人员总数。
- PDF p.1 作者：Stephanie Finley, Liudas Panavas, Thomas Mikkelson, Cam Hinton, Stacey Ganss, Bradley Monton, Emily Kendall, Michelle Spradlin, Lydia Bye, Michael O’Brien, Lauren Ylvisaker, Derek Ray, Suhaas Garre, Sushant Mehta, Edwin Chen。统一机构 Surge AI；未列 Anthropic 等被测厂商机构，不能据此排除所有过去/兼职关系。
- [医疗数据](https://huggingface.co/datasets/surgeai/DAYJOB-healthcare)、[金融数据](https://huggingface.co/datasets/surgeai/DAYJOB-finance) 为 MIT；[harness](https://github.com/surge-ai/dayjob) 为 Apache-2.0。公开 50 医疗和 50/80 金融，其余金融需请求。

§4.3 原句：

> After the agent phase, the verifier runs an agentic judge in the same container: the OpenCode agent [20] (version 1.18.31), run by rewardkit 0.1.7 with Claude Opus 4.8 as its model.

§6 p.8 个案（作者记录，本次未复跑）：

> Asked for an annual cost comparison of in-house and outsourced check processing for 2021–2025, Qwen 3.8 Max (xhigh) counted electronic disbursements such as payroll and tax payments as printed checks, and so used 2,838 checks for 2025 against a true volume of 138. It recommended outsourcing, which at the true 2025 volume costs more than processing checks in-house.

**另一官方口径：** [Surge 博客](https://surgehq.ai/blog/dayjob) JSON-LD datePublished 为 2026-09-23（未给时刻）；本次正文写 finance 21.6h / healthcare 19.6h，与论文 16.6h / 13.6h 不同。未找到足够版本说明，不自行解释或取舍。

## C1：Tavus 方法与 Turing 用词全集

页面日期 October 1st, 2026，官方未给时刻/时区。署名 Hassaan Raza、Ioannis Patras、Tavus Research Team。招募描述为 “recruited through an independent research platform”，没有给平台名。

方法原段：

> Participants were told they would be matched with another participant for a one-minute video call to discuss what they were looking forward to this year. Their partner was in fact a PAL powered by Griffin-Lite, generating her face, voice, and responses in real time. After the call, participants were asked to write down their partner’s answer to the question and to rate their partner on several axes around naturalness and trustworthiness. They were also asked to rate the conversation itself: how well it flowed, and whether they felt their partner was really listening. Only at the end of the survey were participants asked whether it had crossed their mind that their partner might not be a real person, and if so, when. Every participant was then told that their partner had been an AI model.

结果为 26 of 54 believed their partner was a real person（48% 四舍五入）。判定问题的**逐字原始问卷未公开**，上述是厂商方法叙述。未披露事先告知“可能是 AI”；Phoenix-4.5 是另一模型系统对照 n=41，未披露真人—真人组。没有研究设计/执行个人名单、独立执行机构名称、随机化/人口统计、CI 或显著性检验。79%/81% 是主观信心，不是 CI。七分量表 naturalness 5.4、trustworthiness 5.6、enjoyment 5.8、listening 5.5、flow 4.9 与真伪判断不是同一指标。

正文含 Turing 的全部三处：

> Griffin is the first model to pass the real-time, video Turing test.

> On Phoenix-4.5, 2.4% of participants (n=41) believed their partner was a real person. On Griffin-Lite, nearly half did: 48% (n=54). This represents a milestone of, to the best of our knowledge, the first model to have ever passed the video Turing test.

> Given Griffin-Lite is the first model to pass the Turing test, we understand the responsibility to be thoughtful on the risks and release strategies.

可用性原句：

> Griffin-Lite will not be available for use for customers at this time, though it is available for select trusted testers as a research preview.

## C2：VideoFDB 两张表

完整表格（所有模型及脚注）见 20、21 图与 c-videofdb-tables.json。以下按要求抄录 Human/Griffin 行，列序不更换。

| Perception | Fluency | Conv. Flow | Vis. Ground. | Overall | Timing |
|---|---:|---:|---:|---:|---|
| Human reference | 4.16 | 4.20 | 4.24 | 4.20 | 90% / 1400 ms |
| Tavus Griffin Lite | 3.60 | 3.67 | 3.92 | 3.73 | 73.8% / 2232 ms |

| Generation | Fluency | Dyadic Affect | NV Cue Approp. | Overall | Timing |
|---|---:|---:|---:|---:|---|
| Human ground truth | 4.42 | 4.14 | 3.18 | 3.92 | 78% / 900 ms |
| Tavus Griffin Lite | 4.25 | 4.40 | 2.83 | 3.83 | 62.8% / 1892 ms |

两表按 Overall：Griffin 都是 AI 模型第 1、包含 human 则第 2；不代表所有子项第一。Generation Affect 的 Griffin 4.40 高于 human 4.14，不能只抄 overall 代替分项。

方法：237 段真实双人视频通话、11 类动态，rubric-based LM-as-judge。

> Three rubric axes per category, each scored 0–5 by an LM judge with stable cross-judge agreement (77–89% within 1 point).

77–89% 是跨 LM judge 一致性，不是人工真伪判定正确率。页面同时保留 “Current models remain well below human conversational naturalness.” 和 “No evaluated agent approaches the human reference on VideoFDB.”；这里只记录网页措辞，不替作者针对后加榜单行重写结论。

作者：Amrita Mazumdar, Seonwook Park, Rajarshi Roy, Nikhil Srihari, Shengze Wang, Yuhao Zhou, Julia Wang, Koki Nagano, Shalini De Mello。页面机构 NVIDIA / David AI；Yuhao Zhou、Julia Wang 属 David AI。未列 Tavus 作者；论文致谢未找到 Tavus，不能推为不存在任何利益关系。Tavus 自述由 NVIDIA 评分。

arXiv v1 北京 2026-05-29 01:20:01（05-28 17:20:01 UTC），v2 北京 2026-07-29 04:35:17（07-28 20:35:17 UTC）。当前网页没有更新日期；Tavus 页称 2026 年 9 月评分。已存 PDF 不含 Griffin/Tavus，不能以论文日期替新增榜单行定年。

## 发布时刻与热度

| 来源 | 北京时间（括号原时区） | 数值与限制 |
|---|---|---|
| Meta X 官方 | 2026-10-03 03:11:30（10-02 19:11:30 UTC） | a-meta-x-card.json：264K views、955 likes、153 reposts、64 replies、308 bookmarks |
| Meta 博文及六篇 | 2026-10-02，官方未给时刻/时区 | 不能当北京时间零点 |
| DAYJOB v1 | 2026-10-01 16:39:10（08:39:10 UTC） | submission history |
| Tavus X 官方 | 2026-10-02 00:59:30（10-01 16:59:30 UTC） | 搜索快照 19,829,283 views / 38,984 likes；后卡片 1,983.4万 views / 38,983 likes / 9750 reposts / 3271 replies / 27595 bookmarks |
| Tavus 官页 | 2026-10-01，官方未给时刻/时区 | 不能用 X 时刻反推网页上线 |
| Protos | 2026-10-02 21:31:45（13:31:45 UTC） | article:published_time 带 offset；可见页 2:31 PM 未给区名 |
| AIHOT Meta | 抓取北京时间见 d-capture-records.jsonl | 博文 AI评分 73；另一社交条目 65；不是已核“热度 75” |
| HN Meta | 同上 | 49942159：1 分/1 评论；49937619：1 分/0 评论 |
| Reddit Griffin | 同上 | r/singularity 1wv7q40 搜索时 1327 分/372 评论，后读评论时 1323 分；标题写 44%，与原研究 48% 不同 |

计数是不同时间快照，不合成一个虚假快照。Meta Threads 已打开，但未取得可核对帖文及发布时刻。Tao/mathstodon 未找到目标一手帖。DAYJOB 的 AIHOT/HN/X 有限检索未取得有效量级，不代表不存在讨论。

### C3：Community Note 状态

原帖：https://x.com/tavus/status/2105704169009246248 。卡片英文 Note：

> The 48% figure and "video Turing test" claim are from Tavus's own study of 54 one-minute calls, not independently verified or using a standard protocol. Griffin-Lite leads NVIDIA's VideoFDB benchmark on their public leaderboard.

界面写“读者已提议补充背景信息”“正在收集评分”。只确认**提议 Note**，没有 note ID 或最终 helpful 状态；它质疑公司自述与标准 protocol，也肯定榜单位置，不能概括成“已被正式 Note 否定”。Protos 写 community-noted 是媒体自己的表述。22 图仅帖子卡片；视频空白处有黏性页标题叠层，不遮挡正文、Note 或互动数，没有抓取者导航/身份。

## 夸大说法实例（只存措辞，不作事实裁决）

| 发布方 | 原站 URL | 原句逐字 | 北京时间（括号原时区） | 限制 |
|---|---|---|---|---|
| 新浪页面，文内标来源新智元 | https://k.sina.com.cn/article_5952915705_162d248f906703pv9i.html | 毕树超官宣：Meta AI连破6大世界猜想，数学界AlphaGo时刻来了！ | 2026-10-03 19:05:05（网页未标时区，按中国站点北京时区暂记） | 是实际传播页，不是已找到新智元原发；不替代论文 |
| 同上正文 | 同上 | 在过去短短6个月内，Meta的最新模型Muse Spark已经协助人类数学家连破6大数学领域的开放性难题。 | 同上 | 原文有“协助”，不能转写成它说“独立” |
| Alexandr Wang 公开帖 | https://x.com/i/status/2106149796121805099 | mathematicians and muse spark collaborated to solve 6 open problems in math: | 2026-10-03 06:30:16（10-02 22:30:16 UTC） | 2550 likes / 297326 views；个人发言，当前职衔未另核 |
| AI Primer | https://www.ai-primer.com/engineer/stories/meta-muse-assisted-math-results | Meta reports Muse Spark solutions to six open mathematics problems | 2026-10-02 08:00（00:00 UTC；整点可能是日期占位） | 标题 six，正文有 five，不能抹掉其限定 |
| AI工具宝箱编辑组 | https://www.aitoollab.cn/articles/ai-digital-human-tools-2026/ | AI数字人工具横评2026：Tavus 通过视频图灵测试，HeyGen/Synthesia/D-ID 对比 | 2026-10-03，未给时刻/时区 | 正文也有样本量/公司限定，只记录标题 |
| Tech-ish | https://tech-ish.com/2026/10/02/tavus-griffin/ | Nearly half of people on a video call with Tavus's Griffin AI thought they were talking to a human | 2026-10-02 05:25:59（00:25:59 +03:00） | 限定视频通话者，正文有方法限制，不能自动当成夸大 |
| DAYJOB | 未找到符合目标措辞且已核到原站的页面 | — | — | 不拿“25% 任务”替换成“24% 工作”来凑例子 |

没有找到符合全部要求的英文媒体“AI 已通过经典图灵测试”无条件断言实例。Tavus 官方 X 本身明确说 first model to pass the video Turing test，属于厂商自述，已列 C3。

### 社区质疑线索（L6）

Reddit 原帖：https://www.reddit.com/r/singularity/comments/1wv7q40/ 。karl_mainz（抓取时 71 分）提及已有 £20m deepfake video call 骗局并担忧诈骗用途；附既有报道链接，但不证明 Griffin 已用于该骗局。只取得线程/作者/得分，未取得评论独立 permalink；见 d-reddit-comments.json。其他意见不充当方法证据。HN 本次未取得“高赞且有可核证据”的目标评论。

## 可能的吠点

1. Meta 是 six papers / five answers，并明确人类指导和核查；不能直接改为 AI 独立解决六个。（A1、A2）
2. 博文 After completing 与三篇论文 While completing 措辞不同；仅记差异，不推测主观意图。（A1、A2）
3. 概率篇未解决恰在阈值处，且是高维概率陈述，不能写成有限维超过一点即绝对概率零。（A3）
4. 页边 AI 表示 assisted，不等于未经人类修改。（A2）
5. 算术物理未公开 reviewer 名单不等于无人审稿；五篇具体映射仍缺官方明示。（A6）
6. DAYJOB 全条件通过率不是工作量完成比例；90% 门槛 leader 为 62.4%/58.3%。（B2–B3）
7. 人类耗时来自估计；judge 是模型且未找到人工一致率；error 尝试排除等条件应保留。（B3–B4）
8. DAYJOB 官方博客与论文的人类耗时不同，暂不自行调和。（B4）
9. Griffin 48% 来自特定一分种、最后才询问是否为 AI 的公司研究，平台名/问卷/CI 未公开。（C1）
10. VideoFDB 的 LM rubric、跨 judge 一致性与真人真假判断不同；AI 榜单第一不等于达到 human reference。（C2）
11. X Note 仍是提议并征求评分状态；Protos 1300 万为当时历史数值。（C3）
12. Turing 原件抓取受阻，本次不能提供支持“原审问者知道一方是机器”的存档原句。（C4）

## 扫描说法勘误

| 扫描说法 | 状态 | 存档与界限 |
|---|---|---|
| Generation 3.83 vs human 3.92 | 已找到 | 当前 Generation Overall，21 图 |
| Tavus 原帖超过 1300 万 views | 已找到 | Protos 原句如此；本次 X 约 1983 万，保留时点 |
| 出现 Community Note 质疑图灵测试定义 | 部分支持 | 可见提议 Note；质疑自述/非标准 protocol，未核正式 helpful 状态，也非逐字讨论 Turing 1950 定义 |
| DAYJOB 共 30 个模型配置 | 已找到 | 摘要、Table 3、CSV 各 30；不是 30 个厂商 |
| DAYJOB 提交时刻 08:39:10 UTC | 已找到 | v1 submission history，北京 10-01 16:39:10 |
| Meta AIHOT 热度约 75 | 部分支持 | 本次为 AI评分 73；不是同一时点，页面也不称该数值为热度；旧值未证实 |
| DAYJOB AIHOT 约 78 | 未找到一手来源 | 两种 DAYJOB 搜索为空，未取得条目或历史分数 |

## 取证边界与假设

未复核六篇数学证明、复跑 GAP/Lean/benchmark、做新统计推断或确认首发优先权及全部利益关系。日期以明确时区转换；无时区官方页保留日期，新浪时间暂按北京并显式标注。普通页面用公开无 cookie GET；社交只存公开内容/帖子卡片，未自行登录、操作账号或绕过验证。没有生成发布稿、提交或推送。文件清单与失败见 capture-log.md。
