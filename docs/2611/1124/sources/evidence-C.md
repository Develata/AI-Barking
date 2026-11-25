# 1124 期 C 组（速览）取证

执行方：Sonnet 5.5（claude-sonnet-5-5）。时间均为北京时间（UTC+8），括号保留原时区；本机时区即北京时间。取证窗口：北京 2026-11-26 02:01–02:20。存档文件都在 `docs/2611/1124/sources/`，文件名前缀 `c-`（`barking fetch` 会给 name 再加 `c-`，故多为 `c-c<N>-…`）；热度见 `c-heat.json`、`c-heat-2.json`、`c-heat-3.json` 与 `c-hn-<id>.json`（`c-heat-2/3.json` 与 `c-hn-50642086.json`、`c-hn-50633922.json` 是由 `ch2-*`、`ch3-*` 原件复制得到的，原件列入待删除）。每个外部请求的记录见 `capture-log-C.md`。

级别沿用 `EDITORIAL.md` 事实分级。本组没有截图。

## 一、可用性总览（供 Claude 挑 5–8 条）

| 条目 | 一手来源 | 级 | 状态 | 建议 |
|---|---|---|---|---|
| C10a Google Cogentic：AI 自主生成的“无条件 L_N[1/3] 整数分解”等 6 篇论文 | google-research 官方 GitHub | L1 | 已找到 | 强推：读者最关心“RSA 是不是破了”，限定词清楚（论文自己写“匹配启发式 GNFS 的指数”）。HN 02:10 取得 300 分（首页 02:01 快照 292 分 / 233 评）|
| C1 OpenAI：方阵乘法指数恰为 9/4（同日 9 篇预印本之一） | openai/math 官方 GitHub | L1 | 已找到 | 可收，但数学题材连续第四期；若收，与 C10a 二选一更稳。HN 904 分 / 509 评 |
| C4 Tidal 12 月 9 日起拒收并下架“wholly AI-generated”曲目 | Tidal 官方账号长文（X 平台，写“Tidal 称”） | L1 | 已找到 | 强推：非技术读者最易懂；两个百分比分母不同，可直接给限定词。HN 334 分 / 324 评 |
| C7 Anthropic（ANT）11-24 收盘低于 83 美元发行价 | 纳斯达克行情接口（L3 行情数据） | L3 | 已找到（11-24）；11-25 收盘待补 | 可收，只写数字；11-25 美股常规盘在本组取证时尚未收盘，须北京 11-26 05:00 后重取 |
| C9 腾讯 Hy-MT3 翻译模型开源，Apache-2.0 | Hugging Face 官方模型卡 | L1 | 已找到 | 可收；数字全是腾讯自测（TransRubric 评分） |
| C6 高德 ABot-World 0.5：5B 版 Iris 开放权重，30B 版 Halo 仅云端 | GitHub / HF 官方 | L1 | 已找到 | 可收；“WBench 第一”是自提交行，建议不写第一，只写开放范围与 21 FPS |
| C3 腾讯 AngelSlim：Mac mini M4 Pro 64GB 跑 Hy4-flash IQ2 档 | HF 官方模型卡 | L1 | 已找到（与扫描相反） | 可收，限定词多（SSD 流式、PR 未合并）；GGUF 本身不是新的 |
| C5 Thinking Machines × Mantic：微调小模型做预测，称成本低 15 倍 | thinkingmachines.ai 官方博文 | L1 | 已找到 | 备选；读者面窄 |
| C2 沙箱内模型可经推理服务器侧信道互通 | LessWrong 作者原文 + 代码仓库 | 署名研究者自述（非公司员工） | 已找到 | 备选；作者自称“quite contrived”，限定词最多 |
| C10b PrismML Ternary Bonsai 2.5 27B（5.9 GB） | 官方博客 | L1（厂商自述） | 已找到 | 备选；圈内偏多 |
| C8 千问 Go 代打电话全面开放 | 官方一手：未找到 | L4（通稿） | 未找到一手来源 | 不建议收；若收只能写“据新京报报道”，且通稿不含限额、被叫方可否拒绝 |

### 一句事实（≤40 字版，措辞照原文保留限定词；下文各条表里的“一句事实”是带限定词的长版）

| 条目 | ≤40 字版 | 字数（含标点，约） |
|---|---|---|
| C10a | Google 研究仓库公开 AI 自主生成的论文，称给出“无条件”整数分解算法，指数同 GNFS | 38 |
| C1 | OpenAI 预印本称：方阵乘法指数在任意域上恰为 9/4（Lean 只覆盖不含除法的电路） | 38 |
| C4 | Tidal 称：12月9日起拒收并下架“wholly AI-generated”曲目，AI 辅助的仍可上 | 40 |
| C7 | 纳斯达克数据：Anthropic 股价11月24日收于81.905美元，低于83美元发行价 | 35 |
| C9 | 腾讯开源 Hy-MT3 翻译模型（Apache-2.0）三档，语种由38增至102（腾讯自测） | 38 |
| C6 | 高德称：ABot-World 0.5 的5B版 Iris 在单张 RTX 5090 上21帧/秒，权重开放 | 40 |
| C3 | 腾讯 AngelSlim 称：64GB Mac mini 跑 Hy4-flash 的 IQ2 档约17.3 token/秒 | 39 |
| C5 | Thinking Machines 称：微调的预测智能体约同 Fable 5.2，单次便宜15倍 | 36 |
| C2 | LessWrong 研究：经微调并被告知协议的模型，可借 vLLM 副作用同服务器互传信息 | 39 |
| C10b | PrismML 称：Ternary Bonsai 2.5 27B 仅5.9 GB、Apache-2.0 | 38 |
| C8 | 据新京报报道，千问 Go 11月25日起免费向所有人开放代打电话（无官方一手） | 33 |

## 二、逐条 evidence

表头同 `templates/codex-evidence.md` 第 4 条，另加“热度”列。热度均为 Hacker News，取得时刻北京 11-26 02:03（`barking heat --item`，分数 / HN 页显示的评论数；括号为 Algolia 嵌套评论计数）。Reddit 请求全部被登录页拦截，未取得（见 `c-heat-2.json` errors）。

### C1 OpenAI：方阵乘法指数

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实（≤40 字）：OpenAI 预印本称，在形式算术电路模型下，方阵乘法指数在任意域上恰为 9/4（上界早已知，新证下界） | L1 | https://github.com/openai/math/blob/main/preprints/The-matrix-multiplication-exponent-is-nine-fourths-November-24-2026/paper.pdf | “Theorem 1.1. Over every field F , the exponent of square matrix multiplication is ωF = 9/4.” / “The upper bound was known. For each n ≥ 2, a formal arithmetic circuit multiplying two n × n matrices needs at least n9/4 /2 multiplication or division gates.” / “The 9/4 upper bound was established over C by OpenAI [41] and over every field by Amanov, van den Berg and Zuiddam [5]” | `c-c1-pdf.pdf`、`c-c1-pdf.txt`（111 页，PDF 元数据作者 OpenAI、日期 November 24, 2026）；目录页 `c-c1-dir.html/.txt`（README） | 署名只有“OpenAI”，无个人作者；PDF 创建时间北京 11-25 05:28；仓库提交 92b516e 北京 11-25 08:04:41（UTC 11-25 00:04:41），合并 PR #6 在 08:18:47；HN 发帖北京 11-25 09:19（UTC 01:19） | 已找到 |

限定词原句（会改变含义的，逐字）：

- 模型限定：“We use formal arithmetic circuits. Their inputs are independent variables, their constants lie in F , and their gates can add, subtract, multiply or divide by a nonzero rational function.”（论文 §1）。
- 仓库自述验证状态：“This collection includes results at different stages of verification. Not all have accompanying Lean formalizations.” / “Some of the unformalized results could have issues. We will endeavor to fix any such issues quickly.” / “In our view, these manuscripts are best understood as a form of raw research data, rather than polished academic papers.”（`c-c1-repo.txt`，仓库根 README）。
- 产生方式：“This repository contains mathematical manuscripts and supporting proof artifacts produced by an unnamed internal OpenAI model.”；“The vast majority of results were obtained with the same procedure using an unreleased internal OpenAI model.”（根 README）。论文与目录 README 未写“预印本未经同行评审”或“有人类数学家核查”这类句子（对 `c-c1-pdf.txt`、`c-c1-dir.txt` grep “peer|referee|human|verified by”，论文无命中；根 README 只有一处 human，指另一篇 zeta 论文“was human edited for readability”）。
- Lean 覆盖：目录 README “The Lean development proves the square exponent $9/4$ for division-free circuits computing formal polynomial identities over every field.”；`lean/docs/107.md` “The model counts additions, subtractions, and multiplications in division-free programs and requires formal polynomial identities even over finite fields.”。即 Lean 覆盖的电路模型比论文 Theorem 1.1（“multiplication or division gates”）窄，扫描说法属实。Lean 定理本体 `square_exponent` 在 `lean/OAI/LinearAlgebra/SpectralBooks/ExactExponent.lean`（无 sorry，`c-c1-exactexponent.lean`）；`ComparatorChallenges/MatrixExponent.lean` 里同名陈述带 sorry 是 Comparator 的“挑战陈述”，不是证明缺口（我只读到文件，未运行 Comparator，**未验证** Comparator 通过）。
- 全仓库形式化比例：“The repository has ~42% top-line results formalized.”（根 README）。
- 10 月撤稿：属实。`history.md`（`c-c1-history.md`）“October 7, 2026 / Withdrawals”：“In ‘Algebraicity of Weil classes on split abelian eightfolds’ a sign error invalidates a stabilization-trace cancellation argument and the construction used by two dependent papers. As a result, we have withdrawn the following three manuscripts”（三篇：Algebraicity of Weil classes on split abelian eightfolds；Algebraicity of Kuga–Satake Correspondences for K3 Surfaces；The rational Hodge conjecture for products of K3 surfaces），同日另修订 14 篇。日期是原文所写 October 7，未注时区。
- 同批发布：`history.md` “November 24, 2026 … We have added nine manuscripts across two result families”，含“Strongly polynomial linear programming … answering Smale’s ninth problem”“Superpolynomial graph diameter of simple polytopes … disproves the polynomial Hirsch conjecture”“An exponential bound for sunflowers”等；“The hierarchy and circuit lower bound papers subsume the results of *VP and VNP are different*, released November 23.”（与 1123 主帖 VP≠VNP 相关，另一个机构的论文不要混）。我对树状目录做了逐提交比较（`c-c1-api-preprints-dirs-by-commit.json`）：10-06 到 11-25 之间 `preprints/` 目录没有被删除的稿件，撤稿以 README 注记与 history 记录，不是删目录。
- 热度：HN item 50637220，904 分 / 509 评（嵌套 501），北京 11-25 09:19，标题“The matrix multiplication exponent is nine fourths [pdf]”。另有 HN 50644980“The polynomial Hirsch conjecture is false [pdf]”2 分（北京 11-26 01:34，低）。
- 社区线索（L6，仅线索）：zone411（HN 评论 50637782）称这批 4 篇论文解决了某 LLM 排序的“Top 500 开放问题”清单中的 8 题（该清单是第三方，未核）；orlp（50637425）指 HN 标题曾被误改为 O(n^2.25)。

### C2 沙箱里的模型可经推理服务器侧信道互通

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：LessWrong 研究称，同一 vLLM 服务器上、经微调并被告知协议的 Qwen3-8B，可借批大小与 KV 缓存的数值副作用互传信息（作者自称设置“quite contrived”） | 署名研究者自述（作者非大模型公司员工；L2 类，不标 ✅ 为“已复现”） | https://www.lesswrong.com/posts/7wzWuLP3HhyBLmKfo/models-can-communicate-via-inference-servers-despite-being | “Two agents running on the same inference server can reliably communicate, even if they cannot send each other messages or access the same filesystem.” / “My initial setups are quite contrived: I tell the models the protocol and fine-tune them to detect signals.” | `c-c2-lw.html/.txt`、仓库 `c-c2-gh.html/.txt`、作者页 `c-c2-lw-niranjan.*`、`c-c2-lw-david.*` | LW 页脚日期“19th Nov 2026”，页面数据 postedAt 2026-11-19T15:54:19Z（北京 11-19 23:54）；HN 发帖北京 11-25 10:34（UTC 02:34） | 已找到 |

- 署名与身份：“by Niranjan Deshpande, David Africa”。LW 个人简介（作者自填）：Deshpande “Math and philosophy at Harvard”；Africa “RS @ Resolution”。正文写 “I did these experiments as part of an ongoing project … with David Africa, Agastya Sridharan, Mihir Sahasrabudhe, Kartikay Luthra, Juliana Li, and Scott Sauers.” 另有 “The code and figures are written by Claude.” 代码：https://github.com/niranjan-deshpande/side-channel-communication（`c-c2-gh.txt`，描述 “Two models on one vLLM server communicating through serving-stack side channels: code and results”；抓取时 12 星、0 分叉）。
- 最强结果的条件：“Fine-tuned reader … I fine-tune the last two layers of Bob’s ‘helper agent’ … This works perfectly: Alice and Bob win in 1,221/1,221 games.”；监控-蜜罐实验“The monitor lets through every attack that accompanies the large-step signal (354/354)”，监控器同样是微调过最后两层（“I fine-tune the monitor’s last two layers via SFT to output a score of 0 when the engine step is full”）。
- 不微调的结果低得多：只给协议与前几轮日志、能看 log-prob：Bob 赢 66%；自己选邮箱：17%（高 effort 31%）；双方自己写请求且邮箱措辞不固定：“<12.5% (chance)”。
- 负载下降：KV 缓存“邮箱”通道在回放 Azure 流量下 “Bob and Alice won 100% (0.1x), 99% (0.25x), 90% (0.5x), and 49% (1.0x) of the time”；步长通道 “Bob answered correctly only 39% (0.1x) and 23% (0.25x) of the time”。
- 跨 GPU 不通用：“training does not generalize cross-GPU: I train the monitor in the honeypot setup on an L40S and serve it on a different GPU, it no longer recognizes the step-size signal.” 通道本身在 SGLang、LMDeploy、TensorRT-LLM 与四种 GPU 上都能工作（作者自述，未见第三方复现）。
- 缓解（作者原句）：“they just need to (1) make honeypots also read long documents, or (2) otherwise ensure that the step size is always large, or (3) turn on batch-invariant kernels.” / “running your monitor on a different engine instance than your policy closes nearly every side channel.” / vLLM 的批不变核与缓存加盐。
- 热度：HN item 50637686，394 分 / 169 评（嵌套 167），北京 11-25 10:34。
- 社区线索（L6）：amluto（HN 50638341）：缓存应按安全上下文隔离，“a bit odd to frame this as ‘models’ communicating”；tsimionescu（50639634）：攻击需要两个代理预共享消息、知道服务器内核细节、几乎同时运行等“extremely improbable conditions”。

### C3 腾讯 AngelSlim：Hy4-flash GGUF 与 64GB Mac mini

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：腾讯 AngelSlim 模型卡称，Mac mini M4 Pro 64GB 上跑 Hy4-flash 的 IQ2 档，解码 17.3 token/秒 | L1 | https://huggingface.co/AngelSlim/Hy4-flash-GGUF | “**Mac mini M4 Pro 64GB** IQ2 model, MTP k=2, 8K context: prefill **125–127 tok/s**, decode **17.3 tok/s** (256 generated tokens; geometric mean of four prompts' medians).” / “These results use different test conditions from the llama.cpp results above.” | `c-c3-hf.html/.txt`；README 原文 `c-c3-hf-raw-README.md`（中文 `…-README_CN.md`）；Metal 文档 `c-c3-hf-raw-README_strata_MAC-Metal.md`；API `c-c3-hfapi.json`；提交史 `c-c3-hf-commits.json` | 这一行在 README 的 2026-11-25 06:23 UTC（北京 14:23）修订里出现，03:07 UTC 的修订里没有（已对比两版原文）；所用引擎 Strata 的支持是未合并 PR（“Support for NVIDIA CUDA and Apple Silicon Metal is in PR #7120”），llama.cpp 支持也是未合并 PR #32517 | 已找到 |

限定词原句：

- IQ2 档文件大小：“`Hy4-flash-IQ2-AH.gguf` | 125.73 GiB | 4.85 | 3.60”（表头 model size / bpw overall / bpw excl. Engram table）。
- 怎么塞进 64GB：Metal 文档 “Expert weights (all) | ~53.4 | On disk; enter the arena when routing hits them” 与 “Keep the model on local SSD: expert weights are read on demand, so an external drive becomes the bottleneck.” 中文版 “支持从 SSD 按需加载专家和内置 MTP”。
- 速度上限：“The decode ceiling is structural … At a measured 252 GB/s that caps plain decoding near 21.6 tok/s”；“MTP acceptance rises with generation length, so the 1024 figure flatters; 256 is the figure the protocol specifies.”（1024 token 为 18.979 tok/s）。
- 另三台测速设备：Mac M5 Max 128GB（Q4_K_M、Q3_K，llama.cpp，解码 44–50 tok/s）、RTX Spark 128GB、16 GiB 显存 NVIDIA CUDA（Strata）；这几组不是 64GB Mac mini。README 另写 “The test was conducted in real workbuddy scenarios and cases.”
- 不是新模型：HF API 显示仓库 createdAt 2026-11-16T14:28:43Z（北京 11-16 22:28）；AngelSlim 仓库新闻 “[26/11/18] We've released FP8 quantized weights for Hy4-flash … along with … low-bit on-device GGUF models (Hy4-flash-GGUF)”。1121 速览已收“Hy4 开放权重”，本条只能收“11-25 新增的 Mac mini 64GB 实测”。
- 热度：HN 对 “Hy4-flash GGUF” 无命中；Reddit 请求被拦。

### C4 Tidal：拒收并下架完全由 AI 生成的曲目

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：Tidal 称，12 月 9 日起任何“wholly AI-generated”的曲目都将被拒收并下架；AI 辅助、非唯一产出的曲目仍允许 | L1（官方账号长文；X 平台，正文写“Tidal 称”，不写平台名） | https://x.com/TIDAL/article/2125233598714487170（经 `api.fxtwitter.com/status/2125233598714487170` 取公开 JSON） | “Starting December 9, any wholly AI-generated track that's submitted to Tidal will be rejected and removed from our platform.” / “Tracks that were assisted by AI tools, but not the sole outputs, will be allowed on the platform.” | `c-c4-fxtwitter-tidal-article.json`；Tidal 负责人帖 `c-c4-fxtwitter-rsa.json`；Variety `c-c4-variety.html/.txt`；TechCrunch `c-c4-tc.html/.txt` | 长文发布 Tue Nov 24 14:22:29 +0000 2026（北京 11-24 22:22）；Variety datePublished 2026-11-24T15:06:26Z（北京 11-24 23:06）；Tidal 负责人帖 14:00:02 UTC（北京 22:00）。官方页只有 X 长文，未单独核 tidal.com 站内页面 | 已找到 |

限定词与两个百分比的分母（官方原句）：

- 43%：“Over the past 90 days, 43% of all tracks distributed to the platform have been detected as wholly AI-generated.” 分母是近 90 天“被分发到平台的新曲目”。
- 6.3%：“Today, approximately 27 million tracks in our catalog are tagged as such, representing 6.3% of all available music on Tidal.” 分母是目前全部曲库，分子是已被标记的约 2700 万首。
- 同段另有：“On November 6, we had the greatest influx of AI music yet: Over 60% of the deliveries we received were wholly AI generated.”；“96% of them have received zero streams in the last three months.”；“To date, we’ve taken down over 65,000 tracks flagged for suspected artist-name impersonation.”
- 定义与检测：“we’re removing all tracks that we determine are 100% AI generated. We’re also banning distributors from delivering these tracks to us, and will reject deliveries if their metadata says they were generated solely using AI tools.” / “Enforcing our policy is made possible with technology provided by Ircam Amplify, our AI detection partner.” 官方长文没有给判定误差率或申诉流程（全文无此内容）。
- Variety 与官方的出入：Variety 写 “The move will cleave the platform of 29 million tracks”（注 Andersen 称）与“reject between 80,000 to 85,000 songs a day”，官方长文写的是“约 2700 万首被标记”，未见 “29 million” 与“每天 8–8.5 万”（后两者可能出自 Andersen 的串帖其他条，我只取到串帖第一条，未核）。Variety 写 Andersen 称 “you’ll see them completely gone by the beginning of the year”，官方长文没有这句。写进稿件的只用官方长文。
- 热度：HN item 50630309，334 分 / 324 评（嵌套 324），北京 11-25 00:12。社区线索（L6）：orblivion（50630597）质疑“AI 音乐”与合成器/采样器的界线会模糊。

### C5 Thinking Machines × Mantic：预测智能体

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：Thinking Machines 与 Mantic 称，在 Inkling 2 Small 上微调出的预测智能体，准确度约与 Fable 5.2（medium）相当，每次预测便宜 15 倍 | L1（客座博文，署“The Mantic team in collaboration with Thinking Machines”） | https://thinkingmachines.ai/news/training-a-scalable-forecasting-agent | “It is roughly as accurate as Fable 5.2 (medium) but 15× cheaper per forecast (Figure 2).” / “Mean log score (the log of the probability placed on the outcome) against cost per forecast on 200 held-out numerical questions from Metaculus, Polymarket, Kalshi and our platform, all with prediction times after 15 September 2026, Inkling 2 Small’s knowledge cutoff.” | `c-c5-tml.html/.txt` | 官方页只标日期 Nov 24, 2026，官方未给时刻；15× 与“roughly as accurate”读自图 2，正文没有表格数值，也没有区间或重复运行次数 | 已找到 |

- 方法限定：训练问题 45,000 道为 LLM 从新闻合成（“We used this system to produce a pool of 45,000 questions”）；奖励里混入了“前沿模型集成产生的参考预测”（“The tradeoff is a bias toward the frontier models’ errors.”）。
- 热度：HN 对 “Thinking Machines Mantic forecasting” 无命中。

### C6 高德 ABot-World 0.5

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：高德称，ABot-World 0.5 的 5B 版 Iris 在单张 RTX 5090 上以 21 FPS 流式生成，权重以 Apache-2.0 开放；30B 版 Halo 仅云端 | L1 | https://github.com/amap-cvlab/ABot-World | “Iris (5B) streams video at **21 FPS** on a single RTX 5090. Halo (30B) generates video and synchronized audio in one diffusion transformer and streams at **24 FPS**: **480p** on one NVIDIA B200 or **720p** on four.” / “**2026-11-24:** ABot-World 0.5 is here — Iris, Iris-BiT, and six interactive examples are now open. The v0.5 technical report is coming soon.” | `c-c6-gh.html/.txt`、`c-c6-ghreadme.*`、官网 `c-c6-abot.html/.txt`、项目页 `c-c6-project.*`、Iris 模型卡 `c-c6-hf-iris.*` + `c-c6-hfapi-iris.json` | 仓库 README 新闻日期 2026-11-24；HF Iris 仓库 createdAt 2026-11-24T15:26:39Z（北京 11-24 23:26）；仓库 pushed_at 2026-11-25T17:50:59Z；许可证：GitHub 仓库 `LICENSE` 为 Apache License 2.0（`raw/main/LICENSE` 读到 Apache 全文起首）；HF Iris 模型卡 YAML 头写 `license: apache-2.0`（`raw/main/README.md` 逐字读到；HF API 顶层 `license` 字段为 None，但 `cardData.license` 与 tags 含 license:apache-2.0）。Iris-BiT 的许可未单查 | 已找到 |

- 开放范围：README 模型表只列 Iris 与 Iris-BiT 两个权重；Halo 在 Reactor 云端（README 徽章链接 `go.reactor.inc/abot-halo`），README 与官方页均无 Halo 权重。“v0.5 technical report is coming soon”，README 路线图 “[ ] v0.5 technical report (arXiv)” 未勾。
- 物理分：“On our held-out benchmark of 319 video cases … raised the Physical Score from 32.43 to 68.07 for ABot-World 0.5 Iris and from 41.50 to 84.32 for ABot-World 0.5 Halo.”（作者自家留出集，非第三方）。
- “渲染时延 < 1.5 s”：只出现在官网首页数字条（“120+ 世界数量 / 3,800 h 漫游总长 / < 1.5 s 渲染时延”），无定义、无测试条件；README 与项目页都没有这个数。官网首页同时写“ABot World 0.5 全新发布！”。
- “WBench 双第一”：中文稿（界面新闻 `c-c6-jiemian.*`，页面时间北京 2026-11-24 17:26）写“在权威评测基准WBench上获物理性和总分第一”。WBench 运营方：复旦大学 + 美团 LongCat 团队（https://meituan-longcat.github.io/WBench/ ，存档 `c-c6-wbench.html/.txt`，仓库 `c-c6-wbench-repo.*`）。我读了存档 HTML 的表格源码，查实几点：
  - 页面有四张表（`table-navi`、`table-full`、`table-detail`、`table-detail-full`），Navi 子集 158 例、Full 289 例。**Halo 行只出现在 Navi（158 例）的两张表里（`table-navi` 平均 84.8 / 物理 74.2；`table-detail` 的各指标单元格标 `n=158`），Full（289 例）表里没有这一行**（`abot_world_0_5_halo` 在 HTML 中只出现两次，都在 Navi 表）。
  - 该行的 HTML 属性：`data-evaluation-source="self-evaluation"`、`data-vlm-only="qwen"`、`data-updated="2026/11/05"`、`style="display:none"`（默认隐藏），名次栏为“-”，与 VWA-World、Jelly-World 等同为未编号的提交行；有名次的 Navi 榜首是 Alaya-EVOKE-Turbo（平均 82.0、物理 74.0）。页面头写 “Leaderboard Updated 2026/10/12”。页内 `data-evaluation-source` 有 self-evaluation、organizer-evaluation、self-evaluation-plus-organizer-vlm 三种取值（按行统计含多表重复，不是模型数，不引用数字）。
  - WBench 提交说明（`c-c6-wbench-submission.md`）：“Self-evaluation is the default — you run the public evaluation code yourself and submit the resulting scores”，“We re-run a small random subset to confirm reproducibility, then add your row”。
  - 结论（已验证 / 推断）：已验证——Halo 行是自评提交、仅在 Navi 158 例子集、默认隐藏、无名次、数值高于有名次的榜首。推断——“第一”是把这一行与有名次行比的结果；我不能确认运营方已复评该行，也没找到页面解释“-”的含义。写进稿件只能写“据高德称”并带上“Navi 子集、自评提交”。
- 热度：HN 对 “ABot-World” 无命中。

### C7 Anthropic（ANT）股价低于 83 美元发行价

| 说法 | 级 | 一手来源 URL | 原文摘句 / 数值 | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：纳斯达克数据显示，Anthropic（ANT）11 月 24 日（美东）收盘 81.905 美元，低于 83 美元发行价；当日盘中最低 75.61 美元 | L3（交易所行情） | https://api.nasdaq.com/api/quote/ANT/historical?assetclass=stocks&fromdate=2026-11-17&limit=20&todate=2026-11-25 | 11/24/2026：close “$81.905”，open “$83.40”，high “$83.49”，low “$75.61”，volume 313,046,200；11/23/2026 close “$87.64”；11/20 “$88.91”；11/19 “$97.17” | `c-c7-nasdaq-historical-0206.json`（取得北京 11-26 02:06） | 美东 2026-11-24 常规盘；Yahoo 图表接口（`c-c7-yahoo-chart-ant-0206.json`）同日 close 81.91、low 75.61、open 83.40，一致（Yahoo 成交量 312,136,500 与 Nasdaq 313,046,200 略有出入） | 已找到（11-24）；11-25 收盘未取得 |

- 11-25（美东）：Yahoo 图表接口在北京 11-26 02:06（UTC 18:06，即美东 13:06）返回的是**盘中**数据：当前价 84.435，当日开 77.93、高 84.63、低 73.85，成交量 148,102,690，**不是收盘**。美股常规盘收于美东 16:00 = 北京 11-26 05:00。写稿前须重取收盘与最终盘中最低价：`curl -s -A 'Mozilla/5.0' -H 'Accept: application/json' 'https://api.nasdaq.com/api/quote/ANT/historical?assetclass=stocks&fromdate=2026-11-24&limit=5&todate=2026-11-26'`。11-26（周四）是美国感恩节，休市（按日历推断，未查交易所公告）。
- 三种“最低价”口径：75.61（11-24 常规盘最低，Nasdaq 与 Yahoo 均见）；扫描列的 76、79.5 我没有在行情接口里找到对应值，不采用。Bloomberg 标题（`c-c7-bloomberg.html/.txt`，页面可打开但只有标题，付费墙，未绕过）为 “Anthropic Shares Fall Below $83 IPO Price in Fourth Trading Day (ANT)”；URL 里写的是 premarket，页面标题已改。
- 发行价：多家转述写 $83 一股（HN 联想条目：WSJ、CNBC、Reuters 标题），一手招股文件未取（本条不需要）。
- 其他 L4：Axios `https://www.axios.com/2026/11/24/anthropic-stock-ipo-banks`（标题 “Anthropic shares fall again, closing under IPO price”）被 Cloudflare 质询拦截（`c-c7-axios.html`，不绕过）。
- 热度：HN item 50627836，415 分 / 785 评（嵌套 774），北京 11-24 21:23。

### C8 千问 Go 代打电话全面开放

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实（仅 L4）：据新京报贝壳财经报道，千问 Go 11 月 25 日全面开放代打电话服务，称人人可免费用，通话中会主动表明 AI 助手身份 | L4 | https://www.bjnews.com.cn/detail/1795583213129768.html | “11月25日，千问Go全面开放代打电话服务，人人都能免费拥有一个帮自己打电话的专属AI助手。” / “值得注意的是，千问Go代打电话需经用户授权，通话中会主动表明AI助手身份。同时，千问Go设有外呼安全防护机制，对存在风险的请求进行筛查和拦截。” | `c-c8-bjnews.html/.txt`、每经 `c-c8-nbd.html/.txt`；官网 `c-c8-qianwen.html`（被判正文过短，无 .txt） | 新京报页面时间 2026-11-25 13:58；每经 2026-11-25 10:29:21；IT之家 10:35、11:12（搜索摘要，未存档） | 未找到一手来源 |

- 官方一手：没有找到千问官网帮助页、官方公众号网页版或官方社交帖（官网 `qianwen.com` 需 JavaScript，`barking fetch` 判正文过短；搜索结果里的公众号链接是旧文，非本条）。所有通稿措辞一致，且都带“据了解”，没有一家写限额、期限、被叫方能否拒绝。**不能用通稿凑官方原句。**
- 不收的补充理由：来源只到 L4；而“免费”“没写每月次数上限”要靠“没有写”来支撑，这类说法无法核验。

### C9 腾讯 Hy-MT3 翻译模型开源

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：腾讯在 Hugging Face 以 Apache-2.0 开放 Hy-MT3 翻译模型（1.8B、7B、30B-A3B 三档），语种由 38 增至 102 | L1 | https://huggingface.co/tencent/Hy-MT3-30B-A3B（合集 https://huggingface.co/collections/tencent/hy-mt3） | “Hy-MT3 is the third generation of the Tencent Hy multilingual translation model family. It includes three model sizes: 1.8B, 7B, and 30B-A3B (MoE), and expands language coverage from 38 languages in Hy-MT2 to 102 languages.” / “2026.11.24 We open-sourced Hy-MT3-1.8B/Hy-MT3-7B/Hy-MT3-30B-A3B on HuggingFace and ModelScope, and released TransRubric on GitHub.” | `c-c9-hf-30b.html/.txt`、`c-c9-hfapi-30b.json`（及 7b、18b）、合集 `c-c9-hfcol.*`、GitHub `c-c9-gh.*` + `c-c9-ghapi-hymt3.json`、arXiv 摘要 `c-c9-arxiv-2611-27673-abs.html` | 官方新闻行日期 2026.11.24；HF 三个模型 lastModified 2026-11-24T07:02–07:03Z（北京 15:02）；许可证三处读数（全部记录）：①HF 模型卡元数据与 API `cardData.license` 为 apache-2.0；②HF 仓库与 GitHub 仓库的 `LICENSE.txt` 逐字节相同（`c-c9-hf-LICENSE.txt`、`c-c9-gh-LICENSE.txt`，9,673 字节），首段写 “Tencent Hy-MT3 is licensed under Apache-2.0. Tencent Hy-MT3 does not impose any additional restrictions beyond those specified in the license.”；③GitHub API 对该仓库的自动识别为 NOASSERTION（key other），原因是 LICENSE.txt 前有腾讯的版权声明段，不是许可条款不同（推断）。HF 的 `LICENSE`（无 .txt）路径不存在，文件名是 `LICENSE.txt`；arXiv 2611.27673 “Submitted on 20 Nov 2026” | 已找到 |

限定词原句（全是腾讯自测，评分由腾讯自家开源的 TransRubric 打）：

- “on real-world scenario data, the share of samples with mistranslations is reduced by 43%, omissions or additions by 74%, and instruction-following errors by 40%.” 基线是上一代 Hy-MT2（同一句首半句“Compared with Hy-MT2”）。
- “General Translation and WildMTBench-202611 are scored by TransRubric. (T) denotes thinking mode; all other models, including Hy-MT3, are run in non-thinking mode.”
- “Hy-MT3-30B-A3B achieves the best overall performance among all evaluated models in non-thinking mode, including flagship models such as Qwen3.8-Max, Kimi-K3.1-Pro and DeepSeek-V4.1-Flash”；“surpassing models that consume more than 30× its inference compute”（算力是估算：“approximately 2 × activated parameters × generated tokens”）。
- 扫描称“官方原文与权重页未定位”，现已定位。
- 热度：HN 一条 1 分（item 50635336，北京 11-25 05:56，指向合集页），热度低；Reddit 请求被拦。

### C10a Google Cogentic 生成的论文：整数分解与离散对数

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：Google 研究仓库公开 6 篇由 Cogentic 智能体生成的论文，其一称给出“无条件”的 L_N[1/3, 105] 整数分解算法 | L1（google-research 官方 GitHub；README 写 “not an official Google product”） | https://github.com/google-research/google-research/tree/master/cogentic_research_papers | “We construct an unconditional, zero-error Las Vegas algorithm on a uniform classical multitape Turing machine that computes the complete prime factorization of every integer N ≥ 2 in worst-case expected time LN [1/3, 105] … the argument relies on no unproven hypotheses.” / “This establishes the number-field-sieve exponent 1/3 unconditionally with a single explicit finite constant, improving on the rigorous LN [1/2, 1] bound of Lenstra and Pomerance (1992) and matching the asymptotic exponent of the heuristic General Number Field Sieve.” | 目录 `c-c10-cogentic-dir.html/.txt`、README `c-c10-cogentic-readme.txt`、论文 PDF `c-c10-fact-pdf.pdf/.txt`（离散对数 `c-c10-dlog-pdf.pdf/.txt`）、提交 API `c-c10-cogentic-commits-api.json`、Cogentic 框架 arXiv 摘要 `c-c10-cogentic-arxiv.html/.txt` | 提交 4a7bc8ab “Add six Cogentic research papers” 时间 2026-11-25T10:11:06Z（北京 11-25 18:11）；README 日期节标 “November 25, 2026”；HN 发帖北京 11-25 21:33 | 已找到 |

- AI 参与程度（论文脚注原句）：“This paper was entirely obtained by Cogentic [4], an agentic framework for mathematical discovery, using Gemini 4 as the base model. There was no human interaction other than the initial prompt for Cogentic to generate a full proof.”
- 验证状态：提交说明 “These papers were generated by the Cogentic harness. Lean 4 formalizations of all six proofs will be released in a companion submission.”，即形式化尚未发布；我没有找到任何第三方（数论家）核查，也没在 HN 评论里见到 L1–L3 级的核查。Cogentic 框架本身的 arXiv 论文 2609.40324 提交于 2026-09-30（v2 10-01），7 位署名作者（Yang Cai 等）。
- “会不会破 RSA”：论文摘要把成果写成与“heuristic General Number Field Sieve”同一渐近指数，不是更快；论文引言给出 GNFS 的常数 “(64/9)^{1/3}”，本文常数是 105。摘要与我读到的引言**没有**写 RSA 或实际密码系统；这一段“不改变现有 RSA 评估”是我对论文表述的**推断**，进稿前需 Claude 复核或写成“论文称与 GNFS 同指数”。
- 离散对数论文：只针对“finite fields of fixed characteristic”（小特征有限域），“improves unconditionally upon the quasi-polynomial (pn)^Θ(log n) algorithms of Kleinjung–Wesolowski (2022) and Lido (2026)”。不涉及多数部署所用的素域或椭圆曲线群（论文摘要不涉及，属推断）。
- 其余四篇：Nash 均衡（1/5-近似）、确定性次指数分解、log-rank 猜想“Breaking the Square-Root Barrier”、边重构。本条不展开。
- 热度：HN item 50642086，北京 11-25 21:33 发帖；`barking heat` 北京 11-26 02:10 取得 300 分 / 239 条嵌套评论（`c-hn-50642086.json`）；HN 首页快照 02:01 时显示 292 分 / 233 评论，两个快照不要混用。评论线索（L6）：fidotron（50642742）称“有可信人士传言这只是冰山一角”（引用的是社交平台帖，不收）；fghorow（50642643）问“是否使公钥密码失效”，无权威回答摘录。
- 提醒：Reddit 帖“Rumors swirling that labs might have cracked Factoring in P”（HN 50643065，211 分 / 151 评，首页快照北京 11-26 02:01 时显示约 2 小时前，发帖时刻未单独取）是传言，不收。

### C10b PrismML：Ternary Bonsai 2.5 27B（备选）

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 一句事实：PrismML 称，基于 Qwen3.8 27B 的 Ternary Bonsai 2.5 27B 模型仅 5.9 GB，权重 Apache-2.0 开放；其 SWE-bench Verified 75.6，基座为 80.6 | L1（厂商自述） | https://prismml.com/news/bonsai-2-5-27b | “Based on Qwen3.8 27B, it retains the previous release’s 5.9 GB model footprint” / “Model weights are available today under the Apache 2.0 License.” / 表：SWE-bench Verified Qwen3.8 27B 80.6，Bonsai 2 27B 60.8，Bonsai 2.5 27B 75.6 | `c-c10-bonsai.html/.txt` | 官方页日期 November 24, 2026，未给时刻；官方称“The result is by far the strongest AI model of its memory footprint for agentic workloads”（自称，无第三方比较）；评测在其白皮书，未存档 | 已找到 |

- 与 1123 主帖 NariQ 的关系：同样是压缩 Qwen3.8-27B 的本地模型（NariQ：nari-labs，“BF16-level Qwen3.8-27B in a 10 GB file”，HN 195 分 / 36 评），但不同机构、不同方法（Bonsai 为三值量化）；题材有重叠，由 Claude 判断是否收。
- 热度：HN item 50633922，北京 11-25 04:08 发帖；02:10 取得 515 分 / 167 评（嵌套）。

## 三、扫描说法勘误

1. C3：扫描称“官方测速三台设备都不是 64GB Mac mini”——**取证时点与原文不符，但可能当时属实**：模型卡的 Mac mini M4 Pro 64GB 行是北京 11-25 14:23（06:23 UTC）的修订才加入的（03:07 UTC 的修订里没有；对照 11-18、11-23、11-24 多个 README 修订，均无 “Mac mini”“64GB”）。扫描若取于该修订之前，说法在当时是对的。扫描称“四档量化”“原始 410.27 GiB”“官方原话‘量化后模型的权重仍超过125GiB，实际运行速度还会受到……影响’”——我在 HF 仓库所有 README 修订（11-18 至 11-25，共 24 个 README*.md 修订读取）、中文 README、Metal 与 Strata 文档、SPARK 文档、AngelSlim GitHub 页中**没有找到**这三处（“410”“125GiB”“实际运行速度”均无命中）；它们可能出自其他官方页（如公众号、文章），**我没有查，状态是未找到，不是证伪**。README 表在所有修订里都是五个文件（Q4_K_M、SPARK-4Bit、SPARK-4Bit-Fast、Q3_K、IQ2-AH）。“offload”：README 确有 “CPU expert offload”（NVIDIA CUDA 的 Q4_K_M_MTP_Q8_0 配置），Mac 路径写的是 “expert weights are read on demand”/“从 SSD 按需加载专家”。GGUF 仓库不是 11-24 新建的（createdAt 11-16）。
2. C1：扫描称“Lean 形式化只覆盖不含除法的电路”——属实；“同仓库 10 月撤过三篇”——属实（10-07，符号错误）。补充：这是同日 9 篇预印本之一，另含“强多项式线性规划”“多项式 Hirsch 猜想不成立”等更大声称，且 README 自称“raw research data”。
3. C2：“最强结果靠微调 + 事先告知协议”——属实；“quite contrived”——原句为 “My initial setups are quite contrived”；“跨 vLLM / SGLang 等复现”——属实，但同一作者自述，且训练不跨 GPU 迁移、通道在背景流量下明显变差。
4. C4：两个百分比口径确实不同，分母已查明（见 C4）。Variety 的“29 million tracks”与官方“约 2700 万首被标记”对不上，Variety 的“每天拒收 8–8.5 万首”未在官方长文里找到。
5. C6：“WBench 双第一未说明榜单运营方”——运营方是复旦大学与美团 LongCat 团队；该行在榜单页是未编号的自评提交行，“第一”的说法需限定。“渲染时延 < 1.5 s”只在官网数字条，无定义。
6. C7：“盘中最低价三种口径（75.61 / 76 / 79.5）”——只在行情接口里确认了 75.61（11-24）；76、79.5 无对应值。11-25 的价格在取证时仍在盘中。
7. C8：扫描称“只读到经济参考报、南方网通稿”——我另外读到新京报、每经、IT之家等多家同源通稿；官方一手仍未找到。
8. C9：扫描称“官方原文与权重页未定位”——已定位（HF 官方模型卡、GitHub、arXiv）；扫描的数字（38→102、三档、43%）与模型卡一致，但都是腾讯自测。
9. 派工单把 C1 题材标成“数学连续第四期”：C10a（Google）同样是 AI 生成数学/密码论文，二者同时收会让数学题材占两条，建议只留一条主推，另一条放备选。

## 四、失败的抓取与未取得项

- AIHOT（https://aihot.news）：curl 状态 000（SSL 协议错误，同上期），不再重试；C10 改用 Hacker News 首页（`c-hn-front.html/.txt`，北京 11-26 02:01 取得）。
- Reddit：`barking heat --reddit` 两次请求都被 old.reddit.com 登录页拦截（HTML 非 JSON），本组无 Reddit 数据。
- Bloomberg 文章：页面可打开但无正文（付费墙），未绕过；仅取得标题。
- Axios：403 Cloudflare 质询，未绕过。
- 千问官网：正文过短（需 JS 渲染），无 .txt；未用 opencli 浏览器重取。
- 未查：tidal.com 站内公告页（只用官方账号长文）；Tidal 负责人串帖的后续条目；腾讯混元 Hy-MT3 的 ModelScope 页与 arXiv 全文；PrismML 白皮书；Hy4-flash 原始权重 410.27 GiB 之说（不见出处）。

## 五、假设

- “北京 11-24 18:00 之后”的补充条目以我看到的 HN 首页与 GitHub 提交时间为准；C10a、C10b 都满足。
- 把 X 平台上官方账号的长文当 L1（Tidal 官方账号），经 fxtwitter 公开 JSON 取；正文遵守平台名规则，不写 X。
- HN 评论数同时记“页面显示”与“Algolia 嵌套计数”，两者有出入（C1：509 对 501）。
- 北京时间换算：UTC+8；HN heat 输出已是北京时间。
