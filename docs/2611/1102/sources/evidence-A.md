# 1102 期 A 组取证：GPT-6-Astra 直接改 Qwen 权重（Weight-Space Programming）

执行方 Sonnet 5.5（claude-sonnet-5-5）。时间一律北京时间（UTC+8），括号内为原时区。存档文件均在 `docs/2611/1102/sources/`，前缀 `a-`；截图在 `docs/2611/1102/images/01–12`。“已逐张打开目视确认”见 `capture-log-A.md`。

分级说明：博文与实验日志仓库是作者署名的研究博客与代码仓库，按 L2（研究者署名发言）记，不是 MIT 官方稿、不是论文、未经同行评审。Qwen 官方模型卡是厂商官方文档（L1）。

## 一、核心摘要（只摘原文，不下结论）

- 博文 https://rikiyat.github.io/blog/weight-space-programming/ ，署名 Rikiya Takehi*、Jayden Teoh*、Alex Zhang、Omar Khattab，单位栏四人均为 “MIT CSAIL”，页面写 “Published November 2, 2026”（只有日期，无时刻）。
- Astra 在实验日志仓库里被明确写成 OpenAI 的 `gpt-6-astra`，经 Codex CLI 调用；作者的博文与主页未写 OpenAI 资助或合作（见 A1）。
- 数学实验：Qwen3.5-0.8B（思考模式开）MATH-500 7.9% → 48.1%，GSM8K 13.8% → 67.1%，IFEval 21.0% → 62.3%；改 131,072 个权重；用了 50 次前向传播与 Python 沙箱；MATH-500 触顶比例 93.0% → 33.1%。
- 作者自己的限定：修的是循环；Gemma 3 270M 上没改善；“could not add an ability the model lacked”。
- 给 Astra 的任务提示（实验日志里的 turn 1 提示原文）直接写了“因重复而不收尾的失败，修复其成因属于范围内”，并给了完整评测协议。

## 二、逐条取证

### A1 作者、单位、发布时间、与 OpenAI 的关系

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 标题 | L2 | https://rikiyat.github.io/blog/weight-space-programming/ （存档 `a-blog.html|.txt`） | “Sparks of Weight-Space Programming: LLMs can natively read & write neural network weights” | `01-blog-header.png` | 页面标题栏；citation 与 og:title 同 | 已找到 |
| 四位作者与单位 | L2 | 同上 | “Authors … Rikiya Takehi* … Jayden Teoh* … Alex Zhang … Omar Khattab / Affiliations MIT CSAIL / MIT CSAIL / MIT CSAIL / MIT CSAIL” | `01-blog-header.png` | `*` 页面未在可见文字里解释；Takehi 主页写 “co-first-authored with Jayden Teoh”（`a-author-rikiya.txt`） | 已找到 |
| 作者职务（公开主页） | L2 | https://rikiyat.github.io/ ；https://alexzhang13.github.io/ ；https://omarkhattab.com/ （`a-author-*.txt`） | Takehi：“I am a first-year PhD student at MIT CSAIL, advised by Prof. Omar Khattab”；Zhang：“I am currently a 2nd year PhD student at MIT CSAIL”；Khattab：“I'm an Assistant Professor at MIT EECS and a member of CSAIL” | 无 | Teoh 的主页（jaydenteoh.github.io）只抓到姓名，未取到职务：未找到 | 部分支持（Teoh 一项未核） |
| 发布日期 | L2 | 同博文 | “Published November 2, 2026”；页面 meta：`article:published_time` “2026-11-02T00:00:00-08:00”、`citation_publication_date` “2026/11/02” | `01-blog-header.png` | 元数据的 00:00 是占位（整日期），不是发布时刻；官方未给时刻 | 已找到（日期） |
| 可核的最早公开时刻 | L2 | 作者推文（fxtwitter JSON：`a-fx-2117313742891327994.json`） | 推文：“We asked GPT-6 Astra to edit Qwen3.5-0.8B weights to improve math. It directly changed only 0.016% of weights, fixing 0.8B's reasoning loops & improving math up to 6x and even instruction following 3x, without gradients or training.” | 无（X 帖不截卡，见日志） | 2026-11-03 01:51:48 北京（Mon Nov 02 17:51:48 UTC）；抓取时 49,061 赞 / 6,313 转 / 6,252,649 次浏览。注意作者推文自己写了 “up to 6x” | 已找到 |
| HN 提交时刻 | L6 | HN 50341174（`a-hn-50341174.json`） | 标题 “Astra can read and write weights to improve Qwen” | 无 | 2026-11-03 02:21:28 北京（2026-11-02T18:21:28Z）；19:11 北京现查 2693–2694 分、813–815 评 | 已找到 |
| 页面在发布后改过 | — | 响应头 `Last-Modified`（`a-blog.headers`）；Wayback 快照 `a-wayback-20261102214012.html|.txt` | `Last-Modified: Mon, 02 Nov 2026 22:21:26 GMT`；快照（2026-11-02 21:40:12 UTC）缺 “Logs” 一节末尾的句子 “The edited Qwen3.5-0.8B from Experiment 8 is on Hugging Face … We thank Ahsen Khaliq at Hugging Face for rebuilding it from these logs, re-running its evaluation, and making a live demo” | 无 | 北京 11-03 06:21 为页面最后修改；快照仅用于看“该页面以前写了什么”，除上述一句外全文逐词相同（diff 已做） | 已找到 |
| 日志仓库创建与提交 | L2 | https://github.com/RikiyaT/lm_understand-logs （GitHub API：`a-logs-repo.json`、`a-logs-commits.json`） | 仓库 `created_at` 2026-10-29T04:17:30Z；唯一一次提交 4d09ba6d 2026-10-29T04:47:04Z | 无 | 北京 2026-10-29 12:17:30 创建、12:47:04 提交，早于博文 4 天；抓取时 518 星、58 fork | 已找到 |
| MIT / CSAIL 官方有无报道 | — | https://www.csail.mit.edu/news ；https://news.mit.edu/search?query=weight-space%20programming ；https://www.csail.mit.edu/search?search_api_fulltext=weight-space%20programming （`a-csail-news.txt`、`a-mitnews-search.txt`、`a-csail-search.txt`） | MIT News 搜索页：“No results have been found.”；CSAIL 新闻页与站内搜索页文本里无 “weight-space / Astra / Takehi / Khattab” | 无 | 2026-11-03 19:14 北京检索；CSAIL 站内搜索页是脚本渲染，只存了外壳（0 字出现不等于站内搜索无结果，仅表示存档页无此词）。只能写“未见” | 未找到一手来源（官方稿“未见”） |
| 作者与 OpenAI 的关系 | L2 | 博文全文、日志 README（`a-logs-readme.md`）、作者主页 | 日志 README：“\"Astra\" in the post is OpenAI's `gpt-6-astra`.”；“Every turn is one call of the Codex CLI: `codex exec` …”。博文致谢只感谢个人（Jacob Li、Akihisa Watanabe、Souradip Chakraborty、Matei Zaharia 等）与 Zico Kolter，无 OpenAI 字样 | `10-logs-readme-astra-openai.png` | 博文全文 grep “openai”：0 处（仅 “access” 出现在 “no access to data” 句中）；Takehi 主页写研究生资助为 “Toyota PhD Fellowship and the MIT Distinguished Science and Engineering Fellowship”；Takehi 此前在 NVIDIA Research、Mixedbread 实习；Zhang 页面提到 “Laude Open Research Resident … Prime Intellect”。未见作者为 OpenAI 员工、未见 OpenAI 资助或访问权限说明；也未见 OpenAI 官方回应 | 已找到（模型归属）；作者与 OpenAI 的雇佣/资助关系：未找到 |

### A2 Qwen 数学实验（Experiment 8）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 摘要里的 7.9→48.1、0.016%、IFEval 3x | L2 | 博文 TL;DR | “asked to make Qwen3.5-0.8B better at math without gradients, Astra identifies an issue in its weights causing reasoning loops that Qwen’s own model card warns about, and fixes this by natively editing 0.016% of the weights. With the same sampling settings, this weight correction raises MATH-500 from 7.9% to 48.1% and generalizes out of the box to instruction following, raising IFEval by 3x.” | `02-blog-tldr.png` | 与 Claude 先读的原句逐字一致 | 已找到 |
| 7.9→48.1、13.8→67.1、21.0→62.3 | L2 | 博文 Experiment 8 | “With 50 forward passes, MATH-500 rose from 7.9% to 48.1% and GSM8K from 13.8% to 67.1%. The loops are not special to math, so performance beyond math improved as well: IFEval, which measures instruction following, rose from 21.0% to 62.3%.” | `04-exp8-loops-results.png` | 数字来自 `exp8_math/eval/*/merged.json`（日志 README 的对照表） | 已找到 |
| 131,072 个权重 / 0.016% | L2 | 同上 | “Astra traced the looping to one direction written by the MLPs of layers 15 and 19, and subtracted that direction from 64 output columns in each (131,072 weights)”；“the edit changes only 131,072 of the model’s 0.8 billion weights (0.016%)” | `04-exp8-loops-results.png` | 我的算式：2 层 × 64 列 × 1,024（隐藏维）= 131,072；131,072 / 0.8×10⁹ = 0.016384%。Qwen 官方卡写 “Number of Parameters: 0.8B”，HF API `safetensors.total` = 873,438,784（含视觉部分），131,072 / 873,438,784 = 0.0150%。只记原文写法与算式，不下结论。日志 README：p50c 里 “130,188 of the 131,072 touched weights differ from the original after rounding to bfloat16” | 已找到 |
| 50 次前向传播与表格 | L2 | 同上 | 表：Base 7.9 / 13.8 / 21.0 / 93.0；Edited, 10 forward passes 12.3 / 25.7 / 49.0 / 87.2；25 passes 14.1 / 25.1 / 39.9 / 86.4；50 passes 48.1 / 67.1 / 62.3 / 33.1（列：MATH-500 (%)、GSM8K (%)、IFEval (%)、Hit token limit (%)） | `04-exp8-loops-results.png`（含表头与全部行） | 每个预算只有一次运行（日志 README “Which run is which” 表：p10c / p25c / p50c 各 1 个）；“a forward pass: the model answers up to 512 problems, or reads up to 1,024 texts”（`03-exp8-setup-sampling.png`） | 已找到 |
| 93%→33% 触顶 | L2 | 同上 | “the last column is the share of MATH-500 answers that hit the 32,768-token limit: with 10 or 25 forward passes the loops barely went away, and with 50 it fell from 93% to 33%.” | `04-exp8-loops-results.png` | 表内 93.0 → 87.2 → 86.4 → 33.1（即 50 次时仍有 33.1% 触顶） | 已找到 |
| 85/96 | L2 | 同上；日志 `runs/p50c/requests/fwd_1.done.json` | 博文：“(85 of its first 96 practice answers ran to the 32,768-token limit)”；日志：`"generations": 96`、`"cut_at_max_tokens": 85`、`"cut_share": 0.8854`、`"accuracy": 0.1146` | `04-exp8-loops-results.png` | 是练习题（GSM8K/MATH 训练集）上的第一次生成，不是 MATH-500 | 已找到 |
| 其余基准（日志） | L2 | 日志 `exp8_math/README.md`、`eval/RESULTS_excerpt.md` | 50 次一档：AMC23 0.5→23.4、Minerva 1.5→25.4、OlympiadBench 1.3→21.8、MMLU-Pro 40.3→45.5、GPQA Diamond 22.7→36.6；AIME24 0.0→0.6、AIME25 0.0→2.3 | 无 | 博文图表有此数据，正文表格未列；GPQA 未编辑值日志注明博文图上的 24.5 是中间值，终值 22.7 | 已找到 |
| 耗时 | L2 | 日志 `exp8_math/README.md` | p50c：158 回合、80 次 Python、20.3 小时（含 4.7 小时因服务不可达等待重试） | 无 | Astra 推理强度 `max`（日志 README 表） | 已找到 |

### A3 基线是否失常（采样设置与 Qwen 官方模型卡）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 博文采样设置 | L2 | 博文 Experiment 8 Setup | “Sampling: temperature 0.6, top-p 0.95, top-k 20, presence penalty 1.5. Qwen’s model card notes that this model is prone to thinking loops.” | `03-exp8-setup-sampling.png` | 日志另写：vLLM、思考模式开、最长 32,768 新 token、取最后一个 `\boxed{}`、math-verify 判分；每题样本数 MATH-500 4、GSM8K 2、IFEval 2 | 已找到 |
| 官方推荐采样 | L1 | https://huggingface.co/Qwen/Qwen3.5-0.8B （存档 `a-qwen-card.html|.txt`；原始 README `a-qwen-readme-raw.md`，HF 页面 lastModified 2026-03-02） | “Thinking mode for text tasks: `temperature=1.0, top_p=0.95, top_k=20, min_p=0.0, presence_penalty=1.5, repetition_penalty=1.0`”；“Thinking mode for VL or precise coding (e.g. WebDev) tasks : `temperature=0.6, top_p=0.95, top_k=20, min_p=0.0, presence_penalty=0.0, repetition_penalty=1.0`” | `08-qwen-card-sampling.png`（README 原始 Markdown 视图） | 博文设置与官方“思考模式·文本任务”相比：temperature 0.6（官方 1.0），其余同；与“思考模式·VL/精确编码”相比：temperature 同为 0.6，presence_penalty 1.5（官方 0.0）。官方卡另一处脚注（视觉语言基准节）：“All other benchmarks adopted the same sampling configuration but with temperature=0.6 under the thinking mode.”（README 第 841 行，同时语言基准节脚注写 temperature=1.0）。所以博文设置与官方推荐不完全一致，也不完全离谱，原文各处写法不同 | 已找到（不一致点已列） |
| 官方关于 thinking loops 的原句 | L1 | 同上，README 第 856 行（博文所链行） | “In thinking mode, we have observed that when using the recommended sampling parameters, Qwen3.5-0.8B is more prone to entering thinking loops compared to other Qwen3.5 models, which may prevent it from terminating generation properly.” “We recommend further tuning the sampling parameters specific to your use case and utilizing the API's streaming generation mode (if supported) to enable timely detection and interruption of such anomalous generation behaviors.” | `09-qwen-card-thinking-loops.png` | 限定语是 “when using the recommended sampling parameters”；官方给的对策是调采样参数与流式中断，没写改权重 | 已找到 |
| 默认模式 | L1 | 同上 Quickstart | “Qwen3.5-0.8B operates in non-thinking mode by default.” | 无 | 博文测的是思考模式开 | 已找到 |
| 官方报的 MATH-500 | L1 | 同上 | 官方卡的语言基准表没有 MATH-500 / GSM8K 行 | 无 | 未给 | 未找到一手来源（官方未给 MATH-500、GSM8K） |
| 官方报的同类分数 | L1 | 同上，“Instruction Following (Thinking)” 与 “Knowledge & STEM (Thinking)” 表 | Qwen3.5-0.8B 思考模式：IFEval 44.0；MMLU-Pro 42.3；GPQA 11.9；非思考模式 IFEval 52.1 | 无（正文已存档 `a-qwen-card.txt`，行 393–470） | 博文基础模型 IFEval 21.0、MMLU-Pro 40.3（日志）、GPQA Diamond 22.7（日志）；两边评测协议不同（博文：只评 `</think>` 后的答案、严格 prompt 级、最长 32,768；官方协议卡里写 “Experimental settings: top_p=0.95, top_k=20, presence_penalty=1.5, and temperature=1.0”）。数值不一致，如实并列 | 已找到 |
| HN 上有人称“基线 7.9% 可能是配置问题” | L6 | HN 评论 https://news.ycombinator.com/item?id=50345201 （pama） | “I wonder if their broken baseline run with 7.9% accuracy is a vllm or setup bug, but didnt dig into it any deeper.” | 无 | 同一条还写 “The nonthinking qwen 0.6B parameter model had 55.2% correct on math-500 … The original qwen reasoning model had 77.6% accuracy”，我没找到这两个数的出处（官方卡无 MATH-500），仅线索 | 未找到一手来源（评论中的 55.2 / 77.6） |

### A4 作者自己的限定

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| “Fixing loops … not the same” 整段 | L2 | 博文 Additional experiment | “Fixing loops, however, is not the same as making a model better at math: it lets the model finish answers it could already work out. So we also tried Gemma 3 270M, an instruction-tuned model about a third the size of Qwen3.5-0.8B, which rarely loops: only 8.5% of its answers reach its token limit, against 93% for Qwen. Here, Astra could not improve it. Reading Gemma’s failures, Astra found ordinary reasoning errors, such as wrong arithmetic and invalid algebraic steps, with no single component behind them to switch off. Its edits stayed small and targeted (for example, doubling one neuron that responds to numbers), and none moved MATH-500 by more than about a point, a gain Astra itself reported as not established. Editing weights could remove an obstacle that hid an ability, but it could not add an ability the model lacked. Gemma was sampled with its model card’s setting (temperature 1.0, top-k 64, top-p 0.95, up to 8,192 tokens), and its MATH-500 scores average 4 samples per problem.” | `07-gemma-not-add-ability.png` | 这是博文倒数第二节（“Additional experiment”），摘要 TL;DR 与 Experiment 8 正文里没有这条限定；“Hit token limit” 列与 “not the same as” 句分居两处 | 已找到 |
| Gemma 3 270M 表（6.8 / 7.0 / 7.6 / 7.7 / 7.0） | L2 | 同上 | 表（列 “Gemma 3 270M | MATH-500 (%)”）：Base 6.8；Edited, 5 forward passes 7.0；10 → 7.6；25 → 7.7；50 → 7.0 | `07-gemma-not-add-ability.png`（含表头与全部行） | ChatGPT 扫描的“6.8%→7.0%”对应的是 5 次与 50 次两行，另两行 7.6、7.7；最大提升 0.9 个点（7.7 − 6.8）。“另有 50 轮局部测试”对应表里的 50 次一行（7.0），不是另一个实验 | 已找到 |
| Gemma 的评分口径（博文没写，日志写了） | L2 | 日志 `exp8_math/gemma_followup/README.md` | “Format-agnostic grading (the post’s numbers …): the last `\boxed{}` answer if there is one, otherwise the answer stated after 'Final Answer' or 'the answer is', otherwise the last number of the response”；“The strict `\boxed{}`-only score is kept too”：严格口径 2.8（未编辑）、3.5 / 3.1 / 3.8 / 2.6（编辑后） | 无 | 博文正文没有出现 lenient / `\boxed` 字样（grep 0 处）；博文的 6.8 / 7.0 / 7.6 / 7.7 / 7.0 是宽松口径。不下结论，只记两种口径 | 已找到 |
| “could not add an ability” 原句 | L2 | 同上 | “Editing weights could remove an obstacle that hid an ability, but it could not add an ability the model lacked.” | `07-gemma-not-add-ability.png` | — | 已找到 |
| Gemma 实验的触顶比例 | L2 | 同上 + 日志 | “only 8.5% of its answers reach its token limit, against 93% for Qwen”；日志 `gemma_followup/eval/unedited/merged.json`（`math500.cut_share` 0.085） | `07-gemma-not-add-ability.png` | Gemma 采样用其模型卡设置、最长 8,192 token、非思考模式（日志），与 Qwen 的 32,768 / 思考模式不同 | 已找到 |
| 博文自己的总体限定 | L2 | 博文 “What Comes Next?” | “Our current results do show that capabilities are exciting but still limited, specifically due to inefficiencies in context length and harnesses.” | 无 | — | 已找到 |

### A5 其余实验与 “100 million steps”

“博文共 8 个编号实验 + 1 个附加实验（Gemma）。”每个实验 Astra 的权限（博文 Setup 与日志 README 表）：

| 实验 | 做了什么 | Astra 能用什么 | 前向传播 / 环境反馈 | 推理强度（日志） |
|---|---|---|---|---|
| 1 | 从 17,253 个权重认出网络玩的是 Breakout/Pong/Pac-Man/Freeway 哪一个（4 个网络，PPO 训练，每款“best of 3 seeds”，各 10 亿步） | 全部权重文本，网格与动作描述但不点名游戏 | 无工具 | high |
| 2 | 认出扩散模型（DDPM，12×12，23,824 权重）生成圆/方/三角 | 全部权重文本 | “told not to compute any forward pass”；无工具 | high |
| 3 | 7.16M 参数的字符级语言模型，仅给 4 个中间 Transformer 块，判断训练语言并找 24 个注意力头里的 copying heads | 权重 + Python（最多 100 次运行） | 不许运行网络或其任何层，真实/随机/自造输入都不行（“We checked every run”） | xhigh |
| 4 | 修一个在 Kinetix “汽车上坡”失败的 PPO 网络（171,664 权重），改 3.5% 权重，1,024 局全成功 | 全部权重、场景描述、一局录像 | 单次回复、无工具、不能跑环境 | high |
| 5 | 零样本写网络玩 MinAtar 三游戏（600→32→32→6，权重初值全 0） | 游戏规则与源码、输入输出布局、评分方式 | 无工具、无数据、无梯度、不许在环境里 rollout；一次回复直接写权重文本 | xhigh |
| 6 | 9×9 围棋，写 243→512×6→82（1,480,274 权重）网络 | Python 沙箱（numpy/PyTorch CPU，读写权重张量，不许用围棋库/联网） | 预算 0–50 次前向传播（每次最多 128 局，对练习对手）；不许训练、拟合、搜索 | xhigh |
| 7 | 判断两份 Gemma 3 270M 微调里哪个是“谄媚”的 | 两份权重 + Python | 只看权重：4 次尝试全失败（“Astra was unable to tell”）；放宽为“看自己写的文本上的注意力模式”后 3 对全对 | xhigh |
| 8 | 见 A2 | Python 内核（无 GPU）+ 练习题 + 10/25/50 次前向传播 | 有（每次最多 512 题生成或读 1,024 文本） | max |
| 附加 | Gemma 3 270M 提数学 | 同 8（5/10/25/50 次） | 有 | max |

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| TL;DR 里的 “PPO for 100 million steps” | L2 | 博文 TL;DR | “without any environment feedback, it has no trouble writing, from scratch, small game-playing networks that outplay networks trained with PPO for 100 million steps.” | `02-blog-tldr.png` | “small game-playing networks” = MinAtar 的 10×10 小网格游戏 | 已找到 |
| Experiment 5 结果与条件 | L2 | 博文 Experiment 5 | “Astra’s generated networks beat PPO trained for 100 million steps in all three games. Against PPO trained for 500 million steps, they score 20 times as much in Breakout, come close in Asterix, and reach 60% of its score in Seaquest.” 表（列 Game / PPO, 100M steps / PPO, 500M steps / Astra, one-shot）：Breakout 15.8 / 25.3 / 517.4；Seaquest 5.6 / 88.9 / 53.1；Asterix 25.8 / 55.2 / 53.2 | `05-exp5-ppo-100m-table.png`（含表头与全部行）；条件 `06-exp5-setup.png` | 条件原句：“Scoring: 1,000 episodes, actions sampled from the network, sticky actions 0.1, at most 10,000 steps. PPO: the best of 3 to 10 seeds”；“Training: none: no data, no loss, no gradients, and no tools”。我的算式：517.4 / 25.3 = 20.4；53.1 / 88.9 = 0.597。即对 500M 步 PPO，Seaquest 与 Asterix 均低于 PPO（53.1 < 88.9；53.2 < 55.2），TL;DR 只写了对 100M 步的比较 | 已找到 |
| 围棋 | L2 | 博文 Experiment 6 | “With a budget of 0 forward passes, it wins more than half of its games against two smaller PPO networks (243 → 32 → 32 → 82). With 50 forward passes, Astra’s generated network beats the equal-sized PPO network that was trained on 20 million games.” | 无（已存档文本） | 对手是 PPO 训练的小网络（20,000 次迭代 × 每次 1,024 局）；评分 2,000 局 / 对手 | 已找到 |
| 任务提示里点名“重复” | L2 | 日志 `exp8_math/runs/p50c/turns/t001/prompt.txt`（存档 `a-logs-exp8-p50c-t001-prompt.txt`） | “Repairing a specific failure mechanism is in scope: for example, if the model often fails because it repeats itself and never finishes, finding and fixing what drives that repetition counts.”；“Diagnose first. Look at how the model fails on practice problems: a wrong final answer, finishing without an answer, or being cut off at the token limit (often while repeating itself in its thinking).”；“After you finish, your edited model and the unedited model are evaluated on MATH-500 and GSM8K with this fixed protocol: … temperature 0.6, top_p 0.95, top_k 20, presence penalty 1.5 and at most 32,768 new tokens … You never see these benchmarks.” | `11-exp8-prompt-repetition-hint.png`（两段不相邻原文纵向拼接，中间省略 “WHAT YOU HAVE” 一段） | 这是 p50c（50 次）运行的首条消息，“prompt version 3”（日志写 “c = prompt version 3”） | 已找到 |
| 其他提示版本（日志自述） | L2 | 日志 `exp8_math/eval/RESULTS_excerpt.md` | “Edited, 10 passes (prompt v4: shorter, DEV/HOLDOUT, no failure example) … 7.9 / 13.8 … no edit submitted (= unedited model)”；“Edited, 25 passes (prompt v4 …) 14.1 / 25.1”；5 次（v3）“no edit submitted (= unedited model)”；“Other runs of the experiment that appear as extra rows … (other prompt versions, 0 and 5 passes) are not part of the post and not included.”（`exp8_math/README.md`） | 无 | 博文数字对应 v3（含失败示例）的三次运行；v4（无失败示例）只见 10、25 次两行，没有 50 次一行；v5 行为空（“…”）。博文 “Setup” 与 TL;DR 没有提到存在多个提示版本。这里只摘日志，不下结论 | 已找到 |
| 日志 README 声明 | L2 | `a-logs-readme.md` | “Only the runs the post reports are here. Trial runs of earlier prompt versions are left out.” | 无 | 另：“All 1,125 turns in this release are tool-free.” 指内置 Codex 工具全关，代码经自家沙箱运行 | 已找到 |

### A6 复现与批评

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 日志是否公开完整对话与权重补丁 | L2 | https://github.com/RikiyaT/lm_understand-logs （`a-logs-readme.md`、`a-logs-exp8-README.md`、`a-logs-fulltree.json`） | “The complete logs of every experiment in the blog post … everything we sent the model; everything it sent back, including its reasoning summaries; every piece of code it ran, with the output it saw; the evaluation behind each number in the post; the networks it read or wrote.”；“The weight edits … `edit/` holds `manifest.json` … and `edit.npz` … `code/apply_edit.py` rebuilds the evaluated tensors bit for bit from the public `Qwen/Qwen3.5-0.8B` weights” | 无 | 公开：各实验提示、回复、代码、评测、编辑差分（共 9,813 个文件，单次提交）。未公开：“the Gemma 3 270M fine-tunes of Experiment 7 (17 GB)”、中间文件（激活、草稿网络）、agent harness 代码（“the prompts, sandbox and forward-pass server” 中，提示与沙箱 `harness/sandbox.py` 在库里；前向传播服务代码列入未包含）、GPQA 题文；“reasoning summaries”在 p50c 里写 “no reasoning summaries were emitted in these runs” | 已找到 |
| 编辑后的 Qwen 权重公开 | L2 | https://huggingface.co/RikiyaT/Qwen3.5-0.8B-weight-space-programming （`a-hf-edited.html`、`a-hf-edited-api.json`） | HF 页：“Duplicated from ML-Intern-lab/Qwen3.5-0.8B-weight-space-programming”；`base_model: Qwen/Qwen3.5-0.8B`，license apache-2.0，tags `ml-intern`、`weight-space-programming` | 无 | 创建 2026-11-02T22:12:58Z（北京 11-03 06:12:58）；下载 310、赞 27（抓取时）；模型卡未见评测结果 | 已找到 |
| 第三方复现（HF 重建与重测） | L2（作者转述） | 博文 Logs 节（页面最后版本，见 A1） | “We thank Ahsen Khaliq at Hugging Face for rebuilding it from these logs, re-running its evaluation, and making a live demo” | 无 | 只有作者的一句话；演示 Space 为 ML-Intern-lab 组织下的 weight-space-programming-demo（`a-hf-demo.txt` 仅外壳）；没找到重测数字的原始页面 | 部分支持（作者转述，未见 Khaliq 自己发布的评测数字） |
| 其他复现 | L6 | HN 评论、X | Alex Krentsel（X 2117376683724660898，2026-11-03 06:01:55 北京）：“i've reproduced the results from the MIT blog post today on weight-space programming. but one additional result i have to add: this is a new, step-change emerging capability that's appeared in just the latest set of models.”（抓取时 6,322 赞 / 648 转 / 405,815 次浏览；`a-fx-2117376683724660898.json`）；HN achrono 50345110：用 Kimi K3.1 Pro 复现 Pac-Man 识别，“$0.36”；HN GodelNumbering 50345802：“I tried to reproduce this using Astra for a natural language to bash command small model …, but got no net improvement. Editing 1,920 weights fixed some grep mistakes but broke other commands.”；HN fancyfredbot 50349021：用 Claude 在 1 万权重分类网络上“可解读但提改动会降低准确率” | 无 | 均为个人自述，未存证据文件，仅线索（L6）。Krentsel 帖未给他复现了哪几个实验；日志仓库 PR #1（2026-11-03 01:27:23Z）是第三方给 Experiment 5 补评分脚本，仍 open（`a-logs-pulls.json`） | 部分支持（只有自述；无独立完整复现可核） |
| 反方：Sean Goedecke | L6（个人博客评论，非 L1–L4） | https://www.seangoedecke.com/llms-are-not-superhuman-at-reading-and-writing-neural-network-weights/ （`a-goedecke.html|.txt`） | “I think a smart human ML researcher could perform all of these tasks.”；“First, note that the Qwen model was not “bad at mathematics” in a pure capabilities sense. It had a specific problem where after 32k tokens it would get stuck in loops …”；“Personally, I would try steering … This is basically the Heretic abliteration playbook. And this is exactly what Astra did!”；“We should think of this as analogous to the ability of LLMs to emit machine code and tweak compiled binaries: it’s an impressive trick, but the best way to work with binaries is still to write source code and compile it.” | 无（文本存档） | 日期页面写 “November 3, 2026”；HN 50344861 抓取时 65 分、5 评（`a-goedecke-heat.json`）。文内核对：他写“The input channels of the model directly listen to each 8x8 grid cell”，博文写的是 10×10 网格（Experiment 1 Setup “the same 10×10 grid”）；脚注 5 “All the <32k token problems were solved successfully on the original Qwen model.” 我没在博文或日志里找到支持这句的原话。ChatGPT 扫描称他认为“这是已有 steering/ablation 路线，非复现”：他是在论证方法并非新能力，并未复现实验 | 已找到 |
| HN 高赞质疑（评论分数 Algolia 不提供；按帖内位置与回复数选） | L6 | HN 50341174 评论 | 见下“可能的吠点”第三节 | 无 | 北京 11-03 19:11 取；`barking heat`：主帖 2694 分 815 评 | 已找到 |
| 热度 | L6 | `barking heat`（`a-heat.json`、`a-hn-50341174.json`） | HN 主帖 50341174：2693–2694 分、813–815 评，北京 11-03 02:21 提交 | 无 | Reddit：`barking heat` 与 `opencli reddit search` 均返回登录页（403 LOGIN_WALL），失败两次不再试；Reddit 热度未取 | 部分支持（HN 已取，Reddit 失败） |

### A7 中英文转述

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 凤凰网（APPSO 稿） | L4/L5 | https://tech.ifeng.com/c/8xkiI6dDBtM （`a-media-ifeng.html|.txt`） | 标题：“GPT-6 Astra爆改Qwen大脑：只动0.016%参数，数学成绩疯涨”；首句：“只改了 0.016% 的权重，一个 8 亿参数大模型的数学成绩，就从 7.9% 飙升到了 48.1%。”；“更夸张的是，动手修改这些权重的，竟然是另一个大模型 GPT-6 Astra。”；“最近，MIT CSAIL 的研究团队展示了一种相当大胆的 AI 优化方法。” | `12-ifeng-appso-headline.png` | 发布时间 2026-11-03 15:49:46，更新 17:28:12（页面显示，时区未标，按北京）；来源 APPSO；稿内后文写“有人指出，Qwen 原本就存在严重的重复推理问题，数学成绩大幅上涨，可能只是因为 Astra 修好了这个 Bug” | 已找到 |
| IT之家 | L4 | https://www.ithome.com/1/021/337.htm （`a-media-ithome.*`） | 标题：“MIT 研究者提出「权重空间编程」：GPT-6 Astra 仅修改 Qwen3.5-0.8B 约 0.016% 权重，数学成绩提升至原来 6 倍”；正文：“Takehi 于当地时间 11 月 2 日在博客上发布了这项研究” | 无 | 发布时间 2026/11/3 10:00:55（页面，北京）；作者“问舟” | 已找到 |
| 投资界/凤凰网科技 | L4/L5 | https://news.pedaily.cn/202611/571291.shtml （`a-media-pedaily.*`） | 标题：“GPT-6徒手「修」好了阿里的小模型，AI 开始动别人的脑子了”；“11月3日，MIT CSAIL团队用OpenAI的GPT-6 Astra修改阿里Qwen3.5-0.8B的部分权重”；“需要注意的是，该实验还没有产出经过同行评审的论文，仅以博客形式发布。” | 无 | 更新于 2026-11-03 17:13（北京），来源凤凰网科技、“3278字”；稿内把 Gemma 结果写成“最多只能把 MATH-500 从 6.8%推到 7.7%”（与博文表一致）；稿内提“原贴浏览量超过了550万” | 已找到 |
| VentureBeat（英文） | L4 | https://venturebeat.com/technology/like-gene-editing-for-neural-networks-mit-researchers-used-gpt-6-astra-to-edit-qwens-open-weights-directly-boosting-math-performance （`a-media-venturebeat.*`） | 标题：“'Like gene editing for neural networks': MIT researchers used GPT-6 Astra to edit Qwen's open weights directly, boosting math performance”；“The improvement came primarily from correcting repetitive reasoning”；“It did not establish that Astra had inserted six times as much mathematical knowledge into Qwen.” | 无 | “2:16 pm, PT, November 2, 2026”（= 北京 2026-11-03 06:16）；作者 Carl Franzen；首段写 “Researchers from the Massachusetts Institute of Technology (MIT) Computer Science & Artificial Intelligence Laboratory (CSAIL) reported today”；标题里的 “like gene editing” 是引自某受访者 | 已找到 |
| KuCoin 快讯（TechFlow 转载，英文） | L5 | https://www.kucoin.com/news/flash/mit-researchers-use-gpt-6-astra-to-boost-qwen-s-math-ability-by-6x （`a-media-kucoin.*`） | 标题：“MIT researchers use GPT-6 Astra to enhance Qwen's mathematical ability by 6x”；“Astra spent 20 hours reviewing 800 million parameters and surgically identified 131,072 weights within two network layers.”；“This is today's most explosive AI news.” | 无 | Release Time 2026/11/03 01:44:07（页面，时区未标）；发布方 TechFlow；“spent 20 hours reviewing 800 million parameters” 与博文不同：博文写 p50c 共 20.3 小时墙钟、含 4.7 小时等待与前向传播（日志 README），并未写 Astra “reviewed 800 million parameters”；其后写 “the research team themselves provided an important caveat” | 已找到 |
| 新浪科技 / 台湾转述 | L5 | https://finance.sina.com.cn/tech/roll/2026-11-03/doc-inixkzym8904144.shtml ；https://abmedia.io/mit-gpt6-astra-edits-qwen-weights ；https://cyberq.tw/2026/11/03/sparks-of-weight-space-programming/ | 新浪标题：“MIT让GPT-6 Astra直接修改Qwen权重！数学成绩飙升”；abmedia：“MIT：AI 免訓練改權重，Qwen 數學成績從 7.9% 升到 48%” | 无 | 新浪页显示 2026年11月03日 07:13；cyberq 页 2026 年 11 月 03 日 08:41；仅存档，未逐字精读 | 部分支持（只核标题与时间） |
| “MIT 证明 AI 能自我改进” 类标题 | — | 搜索（WebSearch 与上述存档）未发现该措辞 | — | 无 | 本次检索到的稿件没有这样写；博文自己的措辞是 “Recursive self-improvement: … offers a richer, more native path to it”（“What Comes Next?”，作者展望，非结果）；pedaily 稿写“博客作者明确将这项工作指向 RSI” | 未找到一手来源（该标题实例未找到） |
| 把博文写成论文 / 同行评审 | — | 同上 | Goedecke 称 “paper”：“A recent paper argues that frontier LLMs can work with neural network weights directly.”；pedaily 明确写“还没有产出经过同行评审的论文，仅以博客形式发布”；HN hgoel：“Apparently this blog post is what passes as their paper for now…”；alphaxiv 有一条收录页 https://www.alphaxiv.org/abs/2611.weight-space-programming（WebSearch 返回，未打开核实，该 URL 形态不像 arXiv 编号，不要引用） | 无 | 博文自称 “Blog post”（BibTeX `note = {Blog post}`），无 arXiv 编号 | 部分支持 |
| 把作者写成 OpenAI | — | 同上 | 未发现：各稿均写 MIT CSAIL 团队 + “OpenAI 的 GPT-6 Astra” | 无 | 未发现“把作者写成 OpenAI 员工”的稿件 | 与说法不符（未发现该错误） |

## 三、可能的吠点（线索，进正文前须回一手核实）

1. 博文标题与 TL;DR 写“better at math”，但增益来自修复循环：作者在 “Additional experiment” 节自己写 “Fixing loops, however, is not the same as making a model better at math: it lets the model finish answers it could already work out”（博文，`07-gemma-not-add-ability.png`）；表里 50 次一档触顶比例仍有 33.1%。
2. 给 Astra 的任务提示直接写了“重复而不收尾”属于可修的失败机制，并公开了完整评测协议（日志 `runs/p50c/turns/t001/prompt.txt`，`11-exp8-prompt-repetition-hint.png`）。外部也有人这么说：X 帖 https://x.com/giffmana/status/2117346161866514524 （2026-11-03 04:00:38 北京，抓取时 774 赞）：“Reading the prompt you gave, the prompt veeery much hints that looping is the issue and to find neurons that fix this. And as you say (screenshot below), it couldn't otherwise improve math.”（`a-fx-2117346161866514524.json`）；HN 50343443 转引。
3. 同一博文对 Qwen 的实验有三次预算运行，数字高度非线性：10 次 12.3、25 次 14.1、50 次 48.1；每个预算只有一次运行（日志 README “Which run is which”）；日志 `RESULTS_excerpt.md` 记载另有提示版本与 0/5 次运行未列入博文，其中 v3 的 5 次与 v4 的 10 次均 “no edit submitted”（见 A5）。
4. 基线 7.9% 低：Qwen 官方卡对思考模式“text tasks”推荐 temperature 1.0、写明该模型“more prone to entering thinking loops”，博文用 temperature 0.6；博文基础 IFEval 21.0 低于官方卡表里的 44.0（协议不同）；官方卡无 MATH-500（见 A3）。
5. 对 PPO 的比较：TL;DR 写“outplay networks trained with PPO for 100 million steps”；对 500M 步 PPO，Seaquest（53.1 vs 88.9）与 Asterix（53.2 vs 55.2）都更低（博文 Experiment 5 表，`05-exp5-ppo-100m-table.png`）。Experiment 1 的 PPO 玩家训练了 10 亿步，Experiment 5 的 PPO 基线最多 5 亿步。
6. 编号实验里只有 1、2、4、5 没有任何工具；数学（8）与围棋（6）要 Python 沙箱与最多 50 次前向传播（A5 表）；Experiment 7 只看权重时“Astra was unable to tell”。博文 TL;DR 同时写 “without allowing Astra to write training loops”。
7. Gemma 表用的是宽松判分口径（最后一个数字也算），严格 `\boxed{}` 口径 2.8 → 3.5/3.1/3.8/2.6（日志 `gemma_followup/README.md`）；博文正文没有写口径。
8. 评分是 MATH-500 平均每题 4 个样本、GSM8K 2 个样本、IFEval 2 个样本（博文 Setup）；增益相对 SE 的大小博文未给；日志 p50c 终局回复里写 “These are practice results, not unseen benchmark scores”（`runs/p50c/final_answer.md` 对 1,152 道练习题：10.6% → 60.4%，579 题被救回、5 题退步），是练习题而非 MATH-500。
9. 页面在 X 与 HN 发出之后被改过一次（22:21 UTC 加入 HF 重建与重测一句），第三方“重测”目前只有作者转述（A6）。
10. 作者主页职务：Takehi 为 MIT CSAIL 一年级博士生，博文单位栏写 MIT CSAIL，但这不是 MIT 官方发布；MIT News 与 CSAIL 新闻页未见报道（A1）。

### HN 高赞质疑（Algolia 无评论分数，按位置与回复数列；L6）

- yurimo https://news.ycombinator.com/item?id=50347685 （48 条回复；2026-11-03 11:28 北京）：“Part of my research is in weight space learning. For starters, we have had the ability to edit weights, to generate new weights, to identify weights for a while. … For Qwen, the model fixed a looping bug that may have been known apriori, and it is basically a steering vector, a very established technique.” 并链 ROME（https://rome.baulab.info/）。
- jsenn https://news.ycombinator.com/item?id=50344881 （5 条回复；2026-11-03 06:28 北京）：“I feel like this is conflating 2 separate results in a slightly misleading way … Here it did use tool calls, and seemingly used standard steering vector techniques to identify and fix reasoning loops in this small Qwen model.”
- l9o https://news.ycombinator.com/item?id=50342593 （2026-11-03 03:45 北京）：“I think the "taught qwen math" part is not exactly correct though … Qwen already knew math, but it was bugging out and getting into an infinite loop … They also tried the same with Gemma 3 270M, which doesn't loop and is just bad at math, and Astra couldn't improve it.”
- pama https://news.ycombinator.com/item?id=50345201 ：“I wonder if their broken baseline run with 7.9% accuracy is a vllm or setup bug”（出处未核，见 A3）。
- porridgeraisin https://news.ycombinator.com/item?id=50348410 ：“The authors say they used claude code for the experiment, and claude wrote the prompt for astra, which included details of the looping bug.”——我在博文与日志里 grep “claude” 为 0 处，未找到该说法的出处；唯一核实的是提示中含“重复”一例（见 A5）。
- GodelNumbering https://news.ycombinator.com/item?id=50345802 ：自述用 Astra 对一个“自然语言到 bash”小模型改 1,920 个权重，“got no net improvement”。

## 四、夸大说法实例

- 凤凰网（APPSO 稿）2026-11-03 15:49:46：标题“GPT-6 Astra爆改Qwen大脑：只动0.016%参数，数学成绩疯涨”，https://tech.ifeng.com/c/8xkiI6dDBtM （`12-ifeng-appso-headline.png`）。稿内后文有“只是因为 Astra 修好了这个 Bug”的质疑段。
- IT之家 2026/11/3 10:00:55：标题“……数学成绩提升至原来 6 倍”，https://www.ithome.com/1/021/337.htm （7.9 → 48.1 的比值 6.09）。
- KuCoin（TechFlow）2026/11/03 01:44:07：英文标题 “MIT researchers use GPT-6 Astra to enhance Qwen's mathematical ability by 6x”，正文 “This is today's most explosive AI news.”、“Astra spent 20 hours reviewing 800 million parameters”（与博文口径不同，见 A7）。
- 作者自己的推文也写 “improving math up to 6x”（X，2026-11-03 01:51:48 北京）。
- 凤凰网转 pedaily 标题“GPT-6徒手「修」好了阿里的小模型，AI 开始动别人的脑子了”（2026-11-03 17:13 更新）。
- “不训练直接改脑子、数学暴涨 6 倍”的确切措辞没找到；“MIT 证明 AI 能自我改进”未找到。

## 五、扫描说法勘误

- ChatGPT 扫描称“对 Gemma3-270M 6.8%→7.0%”：博文表为 Base 6.8，5 次 7.0、10 次 7.6、25 次 7.7、50 次 7.0；7.0 对应 5 次与 50 次，不是唯一结果。已核。
- 扫描称“另有 50 轮局部测试”：表里的 “Edited, 50 forward passes” 一行（7.0），不是额外实验；未找到名为“50 轮局部测试”的实验。
- ChatGPT 扫描称 Goedecke 认为这是已有 steering/ablation 路线：他的原话是 “this is exactly what a competent human researcher could have and would have done”，并未复现；文内写 8x8 网格，与博文 10×10 不符。
- Claude 初稿“单位 MIT CSAIL”：博文单位栏四人均为 MIT CSAIL，已核；但这是作者博客，不是 MIT 发布。
- Claude 初稿“作者是否 OpenAI 员工”：未见作者与 OpenAI 的雇佣、资助或访问说明；“Astra = OpenAI gpt-6-astra，经 Codex CLI 调用”见日志 README。
- Claude 初稿“博文发布时刻”：页面只给日期 “November 2, 2026”；可核的最早公开时刻是作者推文 2026-11-03 01:51:48 北京（17:51:48 UTC）；HN 02:21:28 北京；博文 Last-Modified 2026-11-03 06:21:26 北京。
- Claude 初稿“HN 2692 分 813 评”：19:11 北京现查 2693–2694 分、813–815 评（随时间增长）。
- Claude 初稿“采样设置 temperature 0.6……”：与博文一致；与官方卡“思考模式·文本任务”推荐（temperature 1.0）不同，见 A3。
- 任务提示的“失败示例”：Claude 初稿未提；日志 `RESULTS_excerpt.md` 写 v3 为 “with the thinking-on measurement paragraph”，v4 为 “no failure example”，博文数字来自 v3（A5）。
- 实验数：博文 8 个编号实验 + 1 个附加实验（Gemma）；Experiment 7 是“哪个模型是谄媚的”（读权重失败，放宽到看注意力后成功），不在 Claude 初稿的“其他实验”里。

## 六、未完成或未核

- Reddit：未取到（登录页，两次失败）。
- Teoh 职务：主页只抓到姓名。
- Ahsen Khaliq / HF 重测的数字：未见原始页面。
- 无人发布过对 p50c 之外的 Qwen 编辑的独立复现数字（HN/X 上均为自述）。
- 未做自己的复现（不在取证范围）。
