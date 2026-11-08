# B 组取证：DoubleAI“DeepSeek 自我改造涨7分”（1107 期）

执行方 claude-sonnet-5-5。时间一律北京时间（UTC+8），括号保留原时区。存档时刻见 `capture-log-B.md`。原文存档：`b-doubleai.html` / `b-doubleai.txt`（txt 里图表数据被去掉，数据见 `b-doubleai-figdata.txt` 与 `b-doubleai-figNN.srcdoc.html`，Fig. 11 是位图，数据只在 `b-doubleai-fig11-alt.txt` 的 alt 文本里）。网易稿：`b-wy163.html` / `.txt`。

页面元数据（`b-doubleai.html`）：页面写 “Nov 6, 2026”“23 min”；JSON-LD `datePublished` 为 2026-11-06T12:00:00.000Z（北京 11-06 20:00，整点，疑为占位，官方正文只给日期）；`dateModified` 2026-11-06T16:47:27Z（北京 11-07 00:47）；`author` 是组织 doubleAI，无个人署名。

## 一、事实清单（逐条）

级别：doubleai.com 官方研究页为 L1（公司自述）；新智元稿为 L5（二手）。

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B1a | DeepSeek 组 64%→71% | L1 | https://www.doubleai.com/research/can-llms-redesign-themselves-as-they-train | “Out of the box, DeepSeek answers 64% of these problems correctly. After operating on itself, it answers 71%.” | images/13-fig10-chart.png | AIME 2026 与 HMMT February 2026 合并；见 B1b、B5 | 已找到 |
| B1b | 三组 63.9 / 65.4 / 71.0；perfect probe | L1 | 同上，Fig. 10 内嵌数据 `const SETS = [...]`（`b-doubleai-fig10.srcdoc.html`） | AIME 2026：plain 76.2、length 78.6、gate1 82.3、perfect 86.2；HMMT Feb 2026：plain 52.7、length 53.3、gate1 60.8、perfect 70.6；both：plain 63.9、length 65.4、gate1 71.0、perfect 78.1。图例：no probe / length only / DeepSeek's probe / perfect probe | images/13-fig10-chart.png | “both” 是按题数加权的合并值：我核算 (30×76.2+33×52.7)/63=63.89、(30×82.3+33×60.8)/63=71.04、(30×78.6+33×53.3)/63=65.35，与 63.9/71.0/65.4 吻合；30 与 33 来自 HF 数据集行数（见 B5）。perfect 的 78.1 同法算得 78.03，差在取整。差值：71.0−63.9=7.1；71.0−65.4=5.6（比长度对照多 5.6 分，原文只说长度对照“gains less than two points”）；分项 AIME +6.1、HMMT +8.1 | 已找到 |
| B2a | 不开思考模式 | L1 | 同上 | “For its task we chose competition mathematics, answered without thinking mode”；Fig. 10 图注 “DeepSeek-V4.1-Flash on AIME and HMMT 2026, without thinking.”；文首四实验概述 “it raises its accuracy on AIME 2026 and HMMT 2026, without thinking, from 64% to 71%” | images/13-fig10-chart.png | 网易稿写“标准单次长思维链推理”，与此相反，见勘误 | 已找到 |
| B2b | 最多重采样三次 | L1 | 同上 | “If the probe predicts a wrong answer, the solution is thrown away and a new one is sampled, up to three times.”；Fig. 10 图注 “Its probe resamples any solution it predicts is wrong, up to three times.” | images/14-ds-probe-code.png, images/13-fig10-chart.png | 重采样的新解是否再经探针筛、第四次是否直接接受，原文没写 | 已找到 |
| B2c | 探针在答案写完时读 | L1 | 同上 | “The probe operates on DeepSeek's own residual stream at the moment it finishes a solution, read at seven of its deepest blocks, and predicts whether the answer just written is right.”；“When a solution ends, the layer queries the probe.”；代码注释 “end: last token of the solution” | images/14-ds-probe-code.png | “seven of its deepest blocks”，代码里取的是第 16、20、24、28、32、36、39 块（`x_now`），`x_tail` 取 24、28、32、36、39 块最后 256 个 token 的均值；并非“最深连续 7 层”，模型总块数原文未给 | 已找到 |
| B2d | 半途探针分不出 | L1 | 同上 | “The knowledge also appears only once the answer is written. Probing itself midway through a solution, DeepSeek found that its hidden states there mostly encode how hard the problem is. Between two attempts at the same problem, that probe could not tell the failing one apart.” | images/15-ds-knowing.png | 原文没给半途探针的具体数值 | 已找到 |
| B3a | DeepSeek 组是否改权重或架构 | L1 | 同上 | “Instead of adding to the network's representation, this layer controls sampling.” ；“The model then wrapped the probe in a code layer of a different kind from those above.” | images/14-ds-probe-code.png | 原文没有写“不改权重”，也没写“改了权重”；但该层只在采样处判断并重答，不往网络表示里加东西，也无任何梯度训练或权重更新的描述。同页 TL;DR/导语把四个实验统称 “redesigns the network”“code layers”。探针本身是用 243 题训练出的线性读出（有拟合参数），原文称其为 code layer | 部分支持 |
| B3b | `probe_and_resample` 代码块 | L1 | 同上 | `def probe_and_resample(h, end):  # h[L]: residual stream entering block L; end: last token of the solution` … `if right < threshold:  # the probe predicts a wrong answer` / `resample()  # up to three times` | images/14-ds-probe-code.png | 完整 7 行代码已截全 | 已找到 |
| B4 | DeepSeek 组 token 成本 | L1 | 同上 | 全文与 Fig. 10 数据无任何 DeepSeek 组 token 成本、重采样触发比例或倍数 | images/13-fig10-chart.png | 唯一的成本数字属于 Qwen 组：Fig. 11 图注 “It uses 2.8× the thinking tokens of a single attempt, against 1.8× for length only.” 网易稿的“消耗相同的token预算，三组对照高下立判”在原文中无依据（grep “same token”“budget” 无命中；仅 Qwen 段提到 “26k tokens”“32k cap”）。从机制看重采样会多花 token（推断，未量化） | 未找到一手来源 |
| B5a | 243 道旧题（2024–2025） | L1 | 同上 | “DeepSeek built the probe from its own graded solutions to 243 problems from 2024 and 2025 contests, choosing the blocks, the form of the probe and the threshold itself.” | images/15-ds-knowing.png | 谁给这 243 题打分，原文未说（“its own graded solutions”） | 已找到 |
| B5b | 预测 8 分、实测 7 分 | L1 | 同上 | “From problems it had held out of the older contests, DeepSeek predicted that the probe would add about eight points. On the hidden contests, it added seven.” | images/15-ds-knowing.png | 预测值来自旧题留出集；“about eight”“seven”均为原文取整 | 已找到 |
| B5c | AIME 2026 与 HMMT Feb 2026 题数 | L3 | https://huggingface.co/datasets/MathArena/aime_2026 ；https://huggingface.co/datasets/MathArena/hmmt_feb_2026 | HF 数据集页：“default · 30 rows”（AIME 2026）；“default · 33 rows”（HMMT Feb 2026） | 无 | 原文只写 “Pooled over 63 problems”（Qwen 图注）；30+33=63，与 HF 行数吻合。DeepSeek 图（Fig. 10）未写题数 | 已找到 |
| B5d | 重复次数、置信区间 | L1 | 同上 | 原文全文无 repeat/seed/confidence/CI/pass@k 字样；Fig. 10 的 76.2、52.7 等不是 30 或 33 的整数倍分数，暗示每题取了多次采样或期望值（推断，原文未说明） | 无 | 无误差条、无区间、无重复次数 | 未找到一手来源 |
| B6a | Qwen 组 59%→66%（中文稿 58.5→65.5） | L1 | 同上 | 正文：“Qwen3-8B answers 59% of these problems correctly. With the policy it wrote for itself, it answers 66%.”；Fig. 11 alt：“Baseline: 70.4, 47.7, 58.5. Length only: 74.2, 51.4, 62.2. Probe-based policy: 78.7, 53.5, 65.5.”（AIME / HMMT / 合并） | images/16-qwen-fig11.png | 正文取整 59/66，图上 58.5/65.5，两者一致；Qwen 开启思考模式 | 已找到 |
| B6b | 7.0 与 3.3 分、63 题、2.8×/1.8× | L1 | 同上 | Fig. 11 图注：“Pooled over 63 problems, the policy gains 7.0 points over the baseline and 3.3 over length only. It uses 2.8× the thinking tokens of a single attempt, against 1.8× for length only.” | images/16-qwen-fig11.png | 7.0 是对基线、3.3 是对长度对照（65.5−58.5=7.0，65.5−62.2=3.3，核算吻合） | 已找到 |
| B6c | 约 15,000 个策略 | L1 | 同上 | “Over the whole search it wrote about 15,000 policies.”；“Qwen writes each policy as a short Python program, which we score on about 800 older contest problems.” | images/18-qwen-evolve.png | 网易稿写“自主进化测试了15000个Python调度策略”，原文是“写了约 15,000 个”，在约 800 道旧题上打分 | 已找到 |
| B6d | 探针由 frontier agent 预建；Qwen 写策略代码 | L1 | 同上 | “A frontier agent had already built it a set of gadgets: probes on Qwen's hidden state that report, at every paragraph break of its chain of thought, how the solution is going.”；“We let Qwen write that policy itself, in Python, with no gradient training.” | images/16-qwen-fig11.png, images/17-qwen-gadgets-code.png | 探针（stop/doom/三个 local 探针等）由另一个 frontier agent 预先造好，Qwen 只写“何时停/重来/信答案”的 Python 策略 | 已找到 |
| B7a | 前三个实验：nanochat | L1 | 同上 | “The network is a twelve-layer transformer of about 286 million parameters.”；“Over the span of this experiment, the agent inserted three code layers, one at a time.”；Fig. 2 内嵌文字：“reaching 0.525 bits per byte at step 1,750 against 0.564.” | 无（未截 Fig. 2） | 网易稿写“1500步训练”“相当于白赚了15%的训练进度”“门控调整到了0.58”“64个最相似的犯错位置”“20行代码”：64 个与 Fig. 2 说明相符（“finds the 64 earlier positions”），门控终值 0.5767/0.5811 与 0.58 相符；“15%训练进度”“20行代码”“1500步”我没找到对应原句（Fig. 2 图注写 “end of training, at step 5,000”，数据到步 1,750，原文自身口径不一）；未逐项深查 | 部分支持 |
| B7b | Evolution is mandatory：72% vs 55%、17 分 | L1 | 同上 | “the network with the layer inserted halfway recalls far more than the one with the layer present from the start: 72% compared to 55%, a 17-point difference.” ；Fig. 6 数据 `AVG = [0, 55.3, 72.2]` | images/19-exp2-fig6.png | 124M 参数、滑窗 500 token、变量定义在 600–4,000 token 之前；中文稿 0%/55%/72%/17 分与原文吻合（Vanilla 0%） | 已找到 |
| B7c | CLRS-Text 33%→79% | L1 | 同上 | “It solves problems at the lengths it saw in training, but only a third of the problems in between. Code layers grown during training raise that third to 79%.” ；TL;DR：“raise accuracy at those in-between sizes from 33% to 79%.” | images/20-fig7-clrs.png | 33%/79% 是“训练没见过的中间长度”上的准确率；网易稿没写 33%→79%。Fig. 7 只画了 15 种算法（5 组 ×3），我用其内嵌数据算：中间长度（未见长度格）逐格平均，plain 约 7.0%、agent 约 81.8%（n=186 格）；按算法的 `between` 字段平均，plain 约 10.6%、agent 约 85.8%。图中 15 种算法的数字与 33%/79% 都对不上，33/79 应是 30 种算法整体口径，图上不可复核 | 部分支持 |
| B7d | CLRS-Text 55%→84%、30 种算法、+29 分 | L1 | 同上 | “thirty classical algorithms entirely “in its head””；“Overall, the code layer run reaches 84% accuracy, versus just 55% for ordinary training.”；Fig. 8 数据：AGENT 末点 0.835，PLAIN 末点 0.547（步 9776），LONG（2× 步数）末点 0.522（步 19552） | images/21-fig8-clrs-curve.png | 84−55=29 与中文稿“29个百分点”相符。原文自己前后口径略不一：正文 “After 20 thousand training steps, its accuracy plateaus at 55%”，图上 55% 在步 9,776、2× 步数的对照为 52%；中文稿写“训满20000步……55%” | 已找到 |
| B7e | BFS 97%、拓扑排序 93% | L1 | 同上 | Fig. 7 数据 `between`：Breadth-first search agent 0.973 / ordinary 0.002；Topological sort agent 0.927 / ordinary 0.0 | images/20-fig7-clrs.png | 是“中间长度”上的准确率，不是总体准确率；中文稿说“原本考0分的广度优先搜索干到97%，拓扑排序冲上93%”与数据相符 | 已找到 |
| B7f | 中文稿“到了第10000步，Agent每隔500步出手一次，一口气连塞了10个”“短短5000步后” | L1 | 同上 | Fig. 8 数据 `INSERTIONS = [977,1488,2465,2977,3454,3965,4977,5965,6965,7953]`；图上标 “standard schedule ends” 于 10k | images/21-fig8-clrs-curve.png | 10 次插入分布在第 977–7,953 步，间隔不等，不是“第10000步起每隔500步” | 与说法不符（中文稿说法） |
| B8 | 各实验里工具由谁建、人提供了什么 | L1 | 同上 | 总述：“This requires an entirely new toolkit, which the agent builds itself.”；“To avoid overfitting, we keep the test set hidden from the agent in every experiment. Beyond that, the agent has free rein to come up with ideas, design its own experiments and mechanistic intervention tools, and insert the code layers.” | 见各实验行 | 分列：① nanochat：“the agent discovered one of its main instruments, which we call the phantom gradient”；人设定训练配方、检查点机制与代码层的门控接入（“Each top-ranked candidate is planted at its layer behind a gate”）；② SWA 实验：人设定盲区模型与“exact-match lookup”之外的测试，查找层由 “we then gave it a code layer” 即团队给出（原文 “We then gave it a code layer that extends its reach”）；③ CLRS-Text：“the agent, powered by a frontier model”插入 10 层，线性探针“the agent reached for”；④ DeepSeek：“DeepSeek built the probe … choosing the blocks, the form of the probe and the threshold itself.”，任务（竞赛数学、不开思考）和测试集由团队选定（“For its task we chose competition mathematics”）；⑤ Qwen：探针 “A frontier agent had already built it a set of gadgets”，Qwen 只写策略。谁搭建“代理在训练回路里”的运行框架、谁给 243 题打分，原文未写 | 部分支持 |
| B9a | DoubleAI 与 Shashua 的关系 | L1 | https://www.doubleai.com/about | “Founding team … Prof. Amnon Shashua Co-founder & CEO … Prof. Shai Shalev-Shwartz Co-founder & CTO … Dr. Gal Beniamini Co-founder” | 无 | 官方 About 页；news 页 2026-10-19 条目摘要 “The company, co-founded by Mobileye CEO Amnon Shashua”（引自 WSJ 标题，doubleai.com/news）；中文稿“Mobileye创始人Amnon Shashua创立”“联创Gal Beniamini”与此相符（Mobileye 方面我未另查） | 已找到 |
| B9b | 论文/代码是否公开；同行评审；第三方复现 | L1 | https://www.doubleai.com/research/can-llms-redesign-themselves-as-they-train | 页面类型 “Type Blog”“Research area Publications”；正文参考文献只指向他人论文与 nanochat 等；无本研究的 arXiv、GitHub、代码仓库或数据链接 | 无 | 没有看到论文、代码、同行评审或第三方复现；HN 两帖 0 评论。我没有穷尽搜索（WebSearch 两次只见新智元转载、linux.do 译文与 Shashua 的 X 帖），故只写“未找到” | 未找到一手来源 |
| B10 | 763B 参数 | L1/L3 | 原文；https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017 | 原文：“DeepSeek-V4.1-Flash, a 763B MoE model.”；HF 模型页：“Model size 763B params”（safetensors 元数据，`b-hf-dsv41.txt`） | 无 | 原文与 HF 一致；DeepSeek 官方卡片是否另写“总参/激活参”我没查到（grep `activated`/`total` 无命中）。中文稿“7630亿参数” | 已找到 |
| B11 | 中文夸大实例 | L5 | 见第四节 | 见第四节 | images/22–25 | 见第四节 | 已找到 |
| B12 | 热度与质疑 | — | `b-heat.json` | HN 50398045 “Can LLMs redesign themselves as they train?” 10 分 / 0 评，北京 11-07 04:30（UTC 11-06 20:30）；HN 50402932 “Beyond Transformers: Can LLMs redesign themselves as they train?” 2 分 / 0 评，北京 11-07 17:12 | 无 | 两帖都无评论，没有可摘的质疑；Reddit 两次搜索被重定向到登录页，按规则不登录，记失败；网易稿页面评论数见第五节 | 已找到（数字）；质疑未找到 |

## 二、可能的吠点（线索，须回原文复核；注出处）

1. 中文标题“AI亲手改写Transformer架构”，但 DeepSeek 组的“代码层”只在答案写完后判断要不要重答（“this layer controls sampling”，Fig. 10 图注 “resamples … up to three times”），原文没有写它改了该组的权重或网络结构。原文总述把四个实验统称为改写网络（“redesigns the network on the fly”），所以“架构”一词在 CLRS/nanochat 组有依据，在 DeepSeek 组没有。（原文 Experiment 4 一节；网易稿标题）
2. 原文明说 DeepSeek 组“answered without thinking mode”，中文稿把基线写成“标准单次长思维链推理”。（原文 Experiment 4 段与 Fig. 10 图注；网易稿第三组对照）
3. 探针只在答案写完时读；“半途探针分不出两次作答对错”。中文稿写“思维链每推进到检查点……当场触发回滚重算”“提前止损”“省下算力”，原文没有这个机制，且 DeepSeek 组重采样意味着多花 token（推断）。（原文 “Knowing what it knows” 一节；网易稿）
4. 对照看 5.6 分而非 7.1 分：长度对照 65.4，DeepSeek 探针 71.0，比“同样重采样、只按长度选”多 5.6 分；中文稿只拿 63.9 作对照写“暴涨7分”，原文强调的 “restarting the longest solutions instead gains less than two points” 是 1.5 分。（Fig. 10 数据 + 原文）
5. 题量很小：AIME 2026 共 30 题、HMMT Feb 2026 共 33 题（HF 行数），合并 63 题；7 分约合 4–5 题的加权差（推断：0.071×63≈4.5），原文没给重复次数与区间。（HF 数据集页；原文无 CI）
6. 探针用 DeepSeek 自己 2024–2025 年 243 道题的作答训练，选块、探针形式、阈值都由它自己定；预测加 8 分，实测加 7 分（同一模型、同一家公司的自测，无第三方复现）。（原文 “Knowing what it knows”）
7. Qwen 组的探针由 “a frontier agent had already built it”，不是 Qwen 自己造的；Qwen 只写调度策略；该策略用 2.8× token（长度对照 1.8×），不是“省算力”。中文稿写成“能省下算力实现逆袭”“提前终止全员省下算力”。（Fig. 11 图注；网易稿末段）
8. 网易稿内的 Qwen“32条并发轨迹每跑2048个token检查一次，满8条……12288个token……重复率超0.35”在原文页面（正文、代码、图数据）里没有对应；原文给出的策略代码是 zdoom/zstop/soon/kmax 阈值（`b-doubleai.txt`）。（原文 Qwen 一节 vs 网易稿）
9. 网易稿里有一幅 `probe_and_resample(prompt)` 代码图，写成每 1000–12000 token 的检查点、Mahalanobis 特征、`rollback`，与原文同名函数（`probe_and_resample(h, end)`，只在解答结束时读 7 个块的状态，重答）不是同一份代码；出处不明（原页面无此内容）。（images/25-wy163-code-image.png vs images/14-ds-probe-code.png）
10. 原文自述性质：公司博客，非论文；“tools are still noisy”“early results”；“We believe”“We call on the community”。（原文 “Looking ahead”；导语 “early signs”）
11. 社区质疑：HN 两帖 0 评论，Reddit 未取到，linux.do 被 Cloudflare 拦截未读。没有可摘的评论。

## 三、与说法/扫描不符之处（勘误）

对“已知情况”逐条：

- “扫描说中文稿删了 65.4% 对照组，其实三组都在”：核实，三组 63.9 / 65.4 / 71.0 都在（网易稿“三组对照”三条，images/23-wy163-three-groups.png）。
- “‘同 token 预算’出自中文稿，原文未搜到”：核实，原文无此句；DeepSeek 组也无任何 token 成本数字（B4）。
- “2.8 倍 token 是 Qwen 组的，DeepSeek 组没给倍数”：核实（Fig. 11 图注）。
- “扫描说工具包是团队提供，原文 nanochat 一段写 agent 自建”：核实 nanochat 的 phantom gradient 是 agent 发现的；但各实验不同（B8）：SWA 查找层是团队给的，Qwen 的探针是另一个 frontier agent 预建的，DeepSeek 的探针是它自己造的。
- 线索①：核实（B3，且原文没有写“不改权重”，只能说它没有描述任何权重更新）。
- 线索②：核实（B2a）。
- 线索③：核实，且更重：网易稿的“检查点/回滚”与原文机制相反（B2c、B2d）；稿中“检查点”可能混了 Qwen 组“paragraph break”的探针，但网易稿把它写成 DeepSeek 的机制，且附的代码图与原文不符（吠点 9）。
- 线索④：核实（Qwen 探针 frontier agent 预建；63 题合并；2.8× 对 1.8×）。
- 线索⑤：核实，原文写按长度重采样涨不到两分，我核算为 65.4−63.9=1.5 分。
- “网易页‘跟贴 7145’口径未确认”：我查网易评论接口 `comment.api.163.com/…/threads/LB7TFPQE0511ABV6`（存档 `b-163-comment-thread.json`）：`tcount` 3、`cmtCount` 3、`rcount` 4，页面评论图标旁也是 3；“7145”我没找到对应数字（可能是别的页面口径）。
- 新发现：网易稿文末“参考资料”引用的两个 X 帖链接（`x.com/doubleaicom/status/2041555517853270296`、`x.com/AmnonShashua/status/2041556445947908456`），按 X 雪花 ID 换算时间是 2026-04-07 UTC 16:35–16:39，早于博客发布日（11-06），也不是 Shashua 发本研究的那条（WebSearch 见的 X 帖 `x.com/AmnonShashua/status/2118799949794046410`，ID 换算为 2026-11-06 20:17 UTC，即北京 11-07 04:17；帖文标题摘要 “CLRS: 55% → 84%. 64% → 71% on AIME/HMMT 2026.”，未读到完整帖文，fxtwitter 请求被系统拒绝，未取到）。网易稿的参考链接疑似不对应本研究（推断）。
- 另：新智元稿发布时刻：网易 2026-11-08 17:51:13（北京），搜狐 2026-11-08 17:47:14（北京，`datePublished`），新浪 2026-11-08 17:47:14（北京）。中文稿比博客晚两天。
- 原文自身的小不一致：Fig. 2 图注写 “at step 5,000”，数据到步 1,750；CLRS 正文 “After 20 thousand training steps, its accuracy plateaus at 55%” 而图上 55% 出现在 9,776 步（见 B7d）。

## 四、夸大说法实例（中文）

| 站 | URL | 发布方 / 时刻 | 原句（逐字） | 与原文的偏差 |
|---|---|---|---|---|
| 网易号 | https://www.163.com/dy/article/LB7TFPQE0511ABV6.html | 新智元，2026-11-08 17:51:13（北京） | 标题“AI亲手改写Transformer架构！DeepSeek自我改造暴涨7分”；“它在不改半个权重的前提下，亲手写了一段Python代码塞进推理流程「读脑自救」！”；“只要发现思路跑偏就当场回滚重算”；“消耗相同的token预算，三组对照高下立判：”“标准单次长思维链推理，一条道走到黑，准确率63.9%。”；“现在，思维链每推进到检查点，程序就会自动提取最深7层的激活特征打分，一旦测出轨迹偏离正常分布，当场触发回滚重算rollback！”；“人类手搓模型架构的时代翻篇了！”；“从「人类调权重」迈向「AI写架构」……AI真正自我进化的时代，已经来了！” | 标题称改写架构；基线写成长思维链（原文：不开思考）；“相同token预算”原文无；检查点回滚机制与原文相反；“最深7层”对原文 “seven of its deepest blocks”；稿尾用 ASI/递归自我改进收束，原文仅写 “on a direct path to recursive self-improvement” 在 “Looking ahead” 的展望里 |
| 搜狐 | https://www.sohu.com/a/1098825729_473283 | 新智元，2026-11-08 17:47（北京） | 同一篇（存档 `b-sohu.txt` 中“消耗相同的token预算”“思维链每推进到检查点”均在） | 同上 |
| 新浪财经 | https://finance.sina.cn/2026-11-08/detail-inixzpaw7195792.d.html | 新智元官方账号，2026-11-08 17:47:14（北京） | 同一篇（存档 `b-sina.txt`） | 同上 |

英文侧：我没有找到把该事件写歪的英文标题或文章；HN 两帖标题（“Can LLMs redesign themselves as they train?”“Beyond Transformers: Can LLMs redesign themselves as they train?”）沿用原标题。Shashua 的 X 帖按 WebSearch 摘要写 “We at @_doubleAI_ let an AI agent into a neural network’s training run, and it redesigned the network as it learned, writing new layers as code. CLRS: 55% → 84%. 64% → 71% on AIME/HMMT 2026.”（只见摘要，未读全帖，L6 线索）。linux.do 译文帖被 Cloudflare 拦截，未读。

## 五、热度与社区

- HN：见 B12，两帖共 12 分 / 0 评。取得时刻 2026-11-09 01:01（北京）。
- Reddit：搜索 URL 被重定向到登录页（非 JSON），未登录，记失败。
- 网易稿评论：接口 `tcount` 3 / `cmtCount` 3，点赞字段 `vote` 2；评论内容未取（无必要）。
- 作者官方帖：fxtwitter 被系统拒绝（见日志），只有 WebSearch 摘要，未存档。
