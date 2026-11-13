# 1112 期 C 组取证：Edge0-145B“笔记本能跑”

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间一律北京时间（UTC+8，本机时区），括号保留原时区。存档时刻 2026-11-13 22:29–22:40（见 `capture-log-c.md`）。文件前缀 `c-`。截图 25–36 在 `docs/2611/1112/images/`。

页面存档的是“抓取当刻的现版”。博客发布后 GitHub 与 Hugging Face 仍在改文档（11-13 12:00–13:00 UTC 有多次提交），我另存了博客发布前的 README 与模型文档修订版（`c-gh-readme-b7900cbb.md`、`c-gh-model-doc-b7900cbb.md`、`c-hf-145b-readme-6aab5f43.md`），以证明哪些限定语在博客发布前就已写明。

## 一、关键发现摘要（给 Claude 的结论，先看这段）

1. **博客与仓库互相矛盾的三处**（都已逐字存档）：
   - 博客：“day-one support covers macOS, Windows, and Linux, including Apple M-series chips …”。仓库 README 平台表：edge0-145b / edge0-120b 在 **macOS (Apple Silicon) 一栏为“—”**，只支持 “NVIDIA GPU (CUDA 13)” 的 Linux / Windows；模型文档与 HF 卡写“Apple M2 Ultra (Metal) measurements … do not establish macOS release availability”；桌面 App 只有 Windows 版（v0.2.0 两个 zip：windows-x64、windows-arm64）。这些限定语在博客发布（2026-11-12 18:00 UTC）之前的修订里就已存在（GitHub b7900cbb，11-12 15:29 UTC；HF 6aab5f43，11-12 13:15 UTC）。
   - 博客：“We train the model in its on-device architecture from the outset”，并称这避免“compression or pruning step after training a cloud model”。GitHub README / 模型文档 / HF 元数据都写 **derived from Qwen3.8-Flash-Next**，HF `base_model: Qwen/Qwen3.8-Flash-Next`（标签含 `base_model:quantized:`），NOTICE 写 “Reported base”、“No verified training-source commit is asserted”。博客正文只把 Qwen3.8-Flash-Next 当对比基线，**全文没有出现“derived”或“base model”字样**（逐词查过 `c-blog.txt`）。
   - 博客：29GB peak memory；同文三级存储：约 40GB 专家库在系统内存、约 48GB 词表在 SSD；同文另一处写 28GB。**29GB 量的是什么，博客、模型卡、README、模型文档、edge0-infer 文档均未写**（仓库与 HF 存档对 “29GB/29 GiB” 零命中）。仓库自己公布的 145B 实测占用是 NVFP4 **23.25 GiB 显存 + 38.27 GiB 主机内存**、Q3_K **23.14 GiB 显存 + 31.28 GiB 主机内存**（均为 RTX 5090 D 24 GB 台式显卡，residency 开启）。
2. **博客所有笔记本数字（29GB / 1,100+ / 80+ / 120W）在仓库的测速表里找不到对应机器**：仓库表里没有任何一行是笔记本；最接近的“80+”是 RTX 5090 D 上 NVFP4 的 chat decode 79.6（不足 80）。博客对测试机型只写 “a compatible personal laptop, such as a MacBook Pro”，未写型号与内存。
3. **所有基准是 Edge0 自评**（“come from Edge0's internal evaluations”）；Qwen / DeepSeek / Claude 行的数字，在可核对的几项上与 Qwen 官方模型卡公布的数字相同（DeepSWE 1.1 58.7 / 42.2 / 54.4、IFBench 81.3 / 79.5 / 79.2 / 62.5、GPQA 91.7 / 89.2 / 90.8 / 91.3、HLE 35.9 / 30.8 / 33.8 / 40.0），即这些行很可能直接取自 Qwen 卡而非 Edge0 自测（推断，未核实 Edge0 是否复跑）。
4. **博客内部与仓库的数字不一致**（全部记录，不取舍）：Edge0-120B 在博客表里是 DeepSWE 53.6 / Terminal-Bench 81.3，在博客自己的配图与仓库表里是 51.3 / 75.3；“28GB peak memory”对“29GB”；模型 `model.json` 写 `active_params: "6.6 B"`，模型卡与模型文档写 10.12 B active per token，博客写 “approximately 10B”。
5. 第三方实测：HN 上仅 4 条自述（均无复现细节），其中一条自行改植后在 Mac（M5 Max）上 NVFP4 约 50 tok/s，一条在 DGX Spark 上“does not work”。Reddit 搜索被拦（403 / SECURITY_BLOCK），未取到。

## 二、证据表

级别：L1 = 官方页面。第三方评测无。HF/GitHub 属官方仓库，记 L1。HN/媒体仅线索。

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1a 笔记本本地跑 145B：29GB 峰值内存 | L1 | https://edge0.ai/blog/introducing-edge0-145b-frontier-ai-capabilities-on-your-own-computer（`c-blog.html/.txt`） | “Memory \| 29GB peak memory \| Runs on a compatible personal laptop, such as a MacBook Pro, without a server.”；“A 145B model runs locally on a compatible laptop, with reported peak memory of 29GB and decoding above 80 tokens/s.” | 25 | 表头列名 “Reported Edge0-145B result”；测试机型与内存未写；博客页标日期 November 12, 2026，页面元数据 2026-11-12T18:00:00Z（= 11-13 02:00 北京）；官方未给更精确的发布时刻 | 已找到（主张本身）；条件见 C2 |
| C1b 1,100+ tokens/s prefill | L1 | 同上 | “1,100+ tokens/s prefill \| Processes long prompts, including dozens of pages of code, before generation begins.” | 25 | 未写输入长度、机型、量化格式。仓库表无对应行（见 C2） | 已找到（主张）；口径未写 |
| C1c 80+ tokens/s decode | L1 | 同上 | “80+ tokens/s decode \| Generates faster than a person reads.” | 25 | 未写机型、量化、是否含推测解码（正文另写 “Speculative decoding … support the reported 80+ tokens/s decode rate”） | 已找到（主张）；口径未写 |
| C1d 约 120W 典型功耗 | L1 | 同上 | “Approximately 120W typical power \| Sustains operation without throttling in the reported setup.” | 25 | “the reported setup” 未在文中定义；功耗测量位置（整机/ GPU）未写 | 已找到（主张）；口径未写 |
| C1e 128K–256K 上下文“速度基本不变” | L1 | 同上 | “128K–256K context \| Maintains essentially the same speed on long inputs, supporting extended work.” | 25 | 无长上下文速度数据。仓库表只有 4K 与 16K prefill；model.json 默认 `--max-context 131072`；edge0-infer 文档默认 `--context` 131072 | 已找到（主张）；未见支持数据 |
| C1f 同文自相矛盾：28GB | L1 | 同上 | “Edge0-145B has roughly four times as many parameters; its laptop comparison reports 80+ tokens/s and 28GB peak memory. These measurements apply to different models and hardware.” | 25 | 同文表与正文其他处均写 29GB；FAQ 写 “requires a compatible laptop with at least 29GB of memory” | 已找到（不一致） |
| C2a 三级存储：约 40GB 专家库在系统内存 | L1 | 博客 | “An expert library of approximately 40GB sits in system memory, with experts brought into GPU memory as needed.” | 26 | 博客；同段 “Frequently used ‘hot experts’ remain in GPU memory.” | 已找到 |
| C2b 约 48GB 词表在 SSD | L1 | 博客 | “Approximately 48GB of vocabulary data resides on the SSD and is read on demand.” | 26 | 模型卡架构表：“PLE \| 51.2 B n-gram embedding table, fp8, memory-mapped”；51.2B×1 字节≈47.7 GiB，与 “48GB” 数量级吻合（推断）。Qwen3.8-Flash-Next 卡写 “125B with 6B activated, plus 51B n-gram embedding and 4B MTP”，即这张表在 Qwen 基座里就有 | 已找到 |
| C2c “29GB” 量的是什么（显存／进程常驻／统一内存） | — | 博客、HF 卡、GitHub README、`docs/models/edge0-145b.md`、`edge0-infer/docs/edge0-145b.md`、`edge0-strata/EDGE0.md` | 博客仅写 “29GB measured peak memory”“Edge0 memory figures are measured internal peaks.” | 27、28 | **未写**。对仓库与 HF 存档检索 “29GB / 29 GiB”，零命中。仓库的实测占用见下一行 | 未找到一手来源 |
| C2d 仓库自己的 145B 内存实测 | L1 | https://huggingface.co/Edge0/Edge0-145B-NVFP4 ；https://github.com/Edge0-AI/Edge0（`c-hf-145b-readme.txt`、`c-gh-readme.txt`） | “With residency enabled, 145B NVFP4 used 23.25 GiB VRAM + 38.27 GiB host RAM on RTX 5090 D 24 GB.”；“Q3_K (40.81 GiB of weights) as 23.14 GiB VRAM + 31.28 GiB RAM, and NVFP4 (47.79 GiB) as 23.25 GiB VRAM + 38.27 GiB RAM — disjoint expert sets, nothing counted twice.” | 32、33 | 台式显卡 RTX 5090 D 24 GB；模型卡称 “Host RAM is measured as RssAnon + RssShmem”（原文在 120B 段）。两项相加 61.52 GiB（NVFP4）与 54.42 GiB（Q3_K）是我的算术，仓库未给总和 | 已找到 |
| C2e “48GB laptop” | L1 | 博客 | “That puts Edge0-145B at about 40% of the estimated baseline memory while delivering comparable or higher capability, enabling local use on a 48GB laptop.” | 27 | 配图标题：“Edge0-145B scores above comparable models with less than half the memory (29 GB vs ~70 GB) and runs on a 48GB laptop”（28）。仓库 `edge0-infer/docs/edge0-145b.md` 的 Requirements：“At least 16 GiB of GPU memory and 20 GiB of system memory.”；README：“The 16 GiB VRAM + 20 GiB RAM budget is a test limit, not a minimum system requirement.”。“48GB laptop” 在仓库中未出现。博客同文又写 “48GB of vocabulary data” 在 SSD、“40GB” 专家库在内存 | 已找到（主张）；仓库无对应 |
| C2f 激活参数约 10B | L1 | 博客；模型卡 | 博客：“Of the model's 145B total parameters, only approximately 10B participate in computation on each inference step.”；模型卡：“145B · 10.12 B active per token” | 26、32 | 同一仓库 `model.json`：`"active_params": "6.6 B"`（`c-hf-145b-modeljson.txt`），与 10.12 B 不一致（原因未写）；Qwen3.8-Flash-Next 为 “6B activated” | 已找到（仓库内不一致） |
| C2g 测速机器 | L1 | 模型卡 Speed 表；README Benchmark | 表中机器：“RTX 5090 D 24 GB”“RTX 4090 24 GB”“RTX Spark N1X 60 GiB”“Apple M2 Ultra (Metal)”“16 GiB VRAM + 20 GiB RAM”（后者“are the same RTX 5090 D 24 GB, not a different card”） | 33 | 标 “Throughput measured on RTX 5090 D 24 GB · NVFP4.” 无一行标注为笔记本（“RTX Spark N1X 60 GiB”是否笔记本，文中未写）；博客点名的 “MacBook Pro” 无对应行；NVFP4 + M2 Ultra 无结果（仅 Q3_K） | 已找到（表）；博客笔记本数字无对应行 |
| C2h 80 tok/s 与 120W 的测试条件 | — | 同上 | 仓库表：RTX 5090 D NVFP4 “79.6”（chat decode）、“98.3”（agent decode）；Q3_K “99.7”“121.2”；RTX Spark NVFP4 “40.1”“46.0”；M2 Ultra Q3_K “60.9”“57.0”。prefill：RTX 5090 D 4K/16K 为 1839.6/3165.7（NVFP4）、2169.9/3554.9（Q3_K）；RTX Spark N1X 674–922；M2 Ultra 755.6/800.3 | 33 | 博客的 “1,100+ prefill” “80+ decode” “120W” 的测试条件与机型**未写**；仓库任何文件对 “120W/watt” 零命中 | 未找到一手来源 |
| C3a 模型卡：base_model 与许可证 | L1 | https://huggingface.co/Edge0/Edge0-145B-NVFP4（`c-hf-145b-readme.txt`、`c-hf-145b-api.json`、`c-hf-145b-notice.txt`、`c-hf-145b-scope.txt`） | YAML：“license: other / license_name: edge0-community-model-1.0 … base_model: - Qwen/Qwen3.8-Flash-Next”；致谢：“We thank the teams behind Qwen3.8-Flash-Next and Strata … Edge0 extends the Strata engine with latent MoE support.”；“Qwen materials remain subject to the Qwen Community License 1.0.” | 32 | HF API 标签：`base_model:quantized:Qwen/Qwen3.8-Flash-Next`；页面 Model tree：“Base model Qwen/Qwen3.8-Flash-Next → Quantized (624) → this model”。NOTICE：“Reported base: Qwen/Qwen3.8-Flash-Next … No verified training-source commit is asserted in this release record.” | 已找到 |
| C3b 权重格式与体积 | L1 | 同上；GitHub 模型文档 | “Builds \| Q3_K 40.81 GiB · NVFP4 47.79 GiB”；仓库有 `-q3k` 与 `-NVFP4` 两个文件夹（145B 与 120B 各两个） | 32、33 | HF 145B-NVFP4 存储用量 112.86 GB（`usedStorage` 字段，含视频与图）；NVFP4 版 HF 页标 “GGUF”“qwen4exp”“Model size 5B params”（仅骨架 gguf，非全模型） | 已找到 |
| C3c 支持硬件 | L1 | GitHub README、`docs/models/edge0-145b.md`、HF 卡 | README 平台表：“Linux \| NVIDIA GPU (CUDA 13)”“Windows \| NVIDIA GPU (CUDA 13)”“macOS (Apple Silicon) \| —”“iOS \| —”“Android \| —”（edge0-145b / edge0-120b 一列）；“Linux or Windows with an NVIDIA GPU and CUDA 13.”；“Apple M2 Ultra (Metal) measurements are retained in the speed tables. They do not establish macOS release availability.” | 34 | 同一 README 顶部总标语 “macOS · iOS · Android · Linux · Windows / One program for every Edge0 model”，GitHub 仓库描述 “145B on a laptop, 35B on a phone … Mac, iPhone, Android, Windows”，与平台表并存；桌面 App 只有 Windows（`v0.2.0`，2026-11-11T15:13:28Z = 11-11 23:13 北京） | 已找到 |
| C3d 有无写到 Qwen 衍生或初始化 | L1 | GitHub README（news 与 Models 段）、模型文档、HF 元数据 | README：“latent mixture-of-experts models derived from Qwen3.8-Flash-Next for a single NVIDIA GPU on Linux or Windows”；“145B / 120B are the current latent-MoE series, derived from Qwen3.8-Flash-Next and trained natively at 4 bits.”；模型文档：“Qwen3.8-Flash-Next is the base model.” | 34、32 | 具体怎么“derived”（微调、再训练、结构改造后蒸馏）全仓库未写；HN 评论称改了结构（10B 对 6B 激活，同一张 51B n-gram 表） | 已找到（衍生）；方式未写 |
| C3e 许可证 | L1 | HF LICENSE/NOTICE；GitHub README License 段 | “Framework code remains under Apache-2.0”；模型：“Edge0 Community Model License 1.0 … Qwen Community License 1.0 … Both sets of applicable terms must be followed”；“Selling or commercially bundling the model, operating a commercial model service, or offering an independent commercial coding or office assistant using it requires a separate written Edge0 license.” | 32 | 博客称 “open-sourcing”“open-source on-device AI model”；HN 评论指出是 open weights 而非 open source | 已找到 |
| C4 对比表：六项均分 72.8 对 71.7，Edge0 内部评测；对照内存带星号为估算 | L1 | 博客 | “Edge0-145B combines a 72.8 mean score across six benchmarks with 29GB measured peak memory. Qwen3.8-Flash-Next averages 71.7, with an estimated 70GB footprint for standard 4-bit deployment.”；表注：“Edge0 memory figures are measured internal peaks. Asterisked baseline figures are estimates for standard 4-bit deployment and remain to be measured.”；“Edge0, Qwen, DeepSeek, and Claude results come from Edge0's internal evaluations.” | 27、28、29、30 | 转录见第三节。我按表中数值复算：145B 72.80、Qwen3.8-Flash-Next 71.67、120B 68.93（按表值）、DeepSeek 70.10、27B 66.12，与博客一致。差 1.13 分来自 DeepSWE（+5.9）、Terminal-Bench（+3.22）、AA-LCR（+0.2）；HLE（−0.5）、GPQA（−1.3）、IFBench（−0.7）为 Qwen3.8-Flash-Next 更高。博客写 “The complete evaluation setup, hardware environment, and reproduction instructions are being released with our technical report.”（报告未发） | 已找到；全为自评、基线内存为估算 |
| C4b Qwen 官方对照 | L1 | https://huggingface.co/Qwen/Qwen3.8-Flash-Next（`c-qwen-card.txt`） | 模型卡基准表：Qwen3.8-Flash-Next / Qwen3.8-27B / DeepSeek-V4-Flash-0731 / Claude-Opus-4.6 (Max)：DeepSWE 1.1 58.7 / 42.2 / 54.4 / --；IFBench 81.3 / 79.5 / 79.2 / 62.5；GPQA Diamond 91.7 / 89.2 / 90.8 / 91.3；HLE 35.9 / 30.8 / 33.8 / 40.0。参数：“125B with 6B activated, plus 51B n-gram embedding and 4B MTP” | — | 以上数字与 Edge0 博客表中对应行逐项相同；Qwen 卡里 Terminal-Bench 2.1、AA-LCR 未检到（Qwen 卡表中无这两项，grep 零命中）。Qwen 卡 DeepSWE 脚注：“evaluated with the Claude Code and mini-SWE-agent harnesses … We report the highest score across the two harnesses”（Edge0 博客未写自己的 harness） | 已找到（数字相同，出处不同） |
| C4c 120B 数字内部不一致 | L1 | 博客表 vs 博客配图 vs GitHub 表 | 博客表：Edge0-120B DeepSWE 53.6、Terminal-Bench 81.3；博客配图与 GitHub：51.3、75.3 | 29、30 | 博客均分表 120B 68.9 与“表中数值”相符（68.93），与配图 / 仓库数值复算不符（67.55）。其余各行三处一致 | 已找到（不一致） |
| C5a “苹果本跑不了” | L1 | 同 C3c | 见 C3c | 34、31 | 博客称支持 macOS / M 系列；仓库称 145B 在 macOS 无支持、Mac 版“does not establish”；GitHub issue #201 “Support for Apple Silicon (Metal) for Edge0 120B / 145B”（2026-11-13T10:08Z，开放，0 评论）。**“苹果本跑不了”在官方仓库层面成立（现阶段无 macOS 版发布），在博客层面与其声明相反** | 部分支持（见勘误） |
| C5b “不是新训练、衍生自 Qwen3.8-Flash-Next” | L1 | 同 C3d | 见 C3d | 34、32、35 | 衍生一事官方 README / HF 自己写了；“不是新训练”未写，训练方式仓库未写；博客称 “train the model in its on-device architecture from the outset” | 部分支持（见勘误） |
| C5c “Hugging Face 同日放出 NVFP4 权重” | L1 | HF 提交记录（`c-hf-145b-commits.json`）、API | HF 仓库 createdAt 2026-11-11T05:22:51Z（= 11-11 13:22 北京）；“Upload Edge0-145B-fp4” 提交 2026-11-11T12:59:10Z（= 20:59 北京）；README 首版 2026-11-12T06:26Z；博客 2026-11-12T18:00Z | — | NVFP4 权重提交早于博客发布约一天；何时设为公开，存档未显示。HF 145B-NVFP4 下载 14,134、赞 113（11-13 22:29） | 已找到（早于博客，非同日） |
| C6 第三方实测 | L6 | 见第五节 | — | 35 | 仅 HN 自述 4 条；Reddit 失败 | 部分支持 |
| C7 公司背景 | L1 | https://edge0.ai/aboutus、https://edge0.ai/、博客 “Why Edge0” 段 | 见第六节 | — | — | 已找到 |
| C8 热度 | L5 线索 | HN item 50475713 | “Edge0-145B: Frontier AI Capabilities on Your Own Computer” 295 分 / 87 评（页面）；Algolia 295 / 87 | 35 | 提交 2026-11-12 23:06:17 UTC = 11-13 07:06 北京；`barking heat` 取 11-13 22:30（294/87，Algolia 295/87）。GitHub 仓库 10,826 星 / 1,653 fork（API，11-13 22:29） | 已找到 |

## 三、博客对比表转录（原文）

“Model / DeepSWE 1.1 / Terminal-Bench 2.1 / HLE / GPQA-Diamond / IFBench / AA-LCR”

| Model | DeepSWE 1.1 | Terminal-Bench 2.1 | HLE | GPQA-Diamond | IFBench | AA-LCR |
|---|---|---|---|---|---|---|
| Edge0-145B | 64.6 | 86.52 | 35.4 | 90.4 | 80.6 | 79.3 |
| Edge0-120B | 53.6 | 81.3 | 31.0 | 87.4 | 79.3 | 81.0 |
| Qwen3.8-Flash-Next | 58.7 | 83.3 | 35.9 | 91.7 | 81.3 | 79.1 |
| Qwen3.8-27B | 42.2 | 73.0 | 30.8 | 89.2 | 79.5 | 82.0 |
| DeepSeek-V4-Flash-0731 | 54.4 | 82.7 | 33.8 | 90.8 | 79.2 | 79.7 |
| Flint 40B | — | 54.9 | — | — | 89.2 | — |
| Muse Glimmer | — | 51.7 | 22.0 | 83.5 | 77.0 | 83.3 |
| Gemma 4 | — | 43.4 | 22.7 | 85.7 | 76.0 | 62.0 |
| Claude-Opus-4.6 \(Max\) | — | 78.2 | 40.0 | 91.3 | 62.5 | 78.0 |

表注（逐字）：“Scores are percentages; higher is better. A dash means no public score is available, rather than a score of zero.” “Edge0, Qwen, DeepSeek, and Claude results come from Edge0's internal evaluations. Claude-Opus-4.6 has no public DeepSWE 1.1 score. For Flint, Muse Glimmer, and Gemma 4, Terminal-Bench 2.1 and IFBench results come from the Recursive Flint evaluation; HLE, GPQA, and AA-LCR results come from Artificial Analysis.”（表中 Claude 行名保留原样的 `\(Max\)` 转义残留。）

均分表（逐字）：Edge0-145B 72.8 / 29GB；Edge0-120B 68.9 / 29GB；Qwen3.8-27B 66.1 / Approximately 18GB*；Qwen3.8-Flash-Next 71.7 / Approximately 70GB*；DeepSeek-V4-Flash-0731 70.1 / Approximately 87GB*。表注：“The means cover DeepSWE 1.1, Terminal-Bench 2.1, HLE, GPQA-Diamond, IFBench, and AA-LCR. Edge0 memory figures are measured internal peaks. Asterisked baseline figures are estimates for standard 4-bit deployment and remain to be measured. Claude is excluded because it is cloud-only and has no DeepSWE score.”

博客配图（截图 30、28）图注：“Score (%), higher is better. All panels use a zero-based 0–100 scale. Edge0, Qwen, DeepSeek and Claude results are from AutoArk internal evaluation”（配图写 AutoArk，正文写 Edge0；AutoArk 为公司另一名称，联系邮箱 contact@autoark.ai，博客与官网未解释两者关系）。配图 28 另写：“* Baseline memory estimated for standard 4-bit deployment (to be measured); Edge0 is measured internal peak. Y axis does not start at zero.”

## 四、可能的吠点（线索，进正文前须回到一手原文核实）

1. 博客说“macOS 首日支持、MacBook Pro 可跑”，同一天的官方 README 平台表在 macOS 一栏写“—”，模型文档写 M2 Ultra 的数字“不构成 macOS 发布”（博客 “day-one support covers macOS …”；README Platforms 表；`docs/models/edge0-145b.md` Requirements 段）。截图 31、34。
2. 博客说“从一开始按端侧架构训练”，官方 README 与 HF 元数据写“derived from Qwen3.8-Flash-Next”（博客 “We train the model in its on-device architecture from the outset”；README “derived from Qwen3.8-Flash-Next and trained natively at 4 bits”）。截图 26、34、32。
3. 29GB 峰值内存没有写明量的是什么；同一篇里专家库约 40GB 在系统内存、词表约 48GB 在 SSD；仓库自己的实测是 23.25 GiB 显存 + 38.27 GiB 主机内存（RTX 5090 D 24 GB）。截图 26、33。
4. 笔记本速度数字（80+ / 1,100+ / 约 120W）没有写测试机型；仓库速度表没有任何笔记本行，“80+”最接近的一行是台式 RTX 5090 D 上 NVFP4 的 79.6（不足 80）。截图 25、33。
5. 同一篇里 29GB 与 28GB 并存。截图 25。
6. 全部基准是 Edge0 内部评测；Qwen / DeepSeek / Claude 行与 Qwen 官方卡的数字相同；基线内存（70GB 等）是“估算，尚待实测”，估算依据未写；对比是 Edge0-145B（原生 4 位）对“标准 4-bit 部署”的 Qwen3.8-Flash-Next。截图 27、29。
7. “比 Qwen3.8-Flash-Next 强”只在 DeepSWE、Terminal-Bench、AA-LCR（+0.2）上成立，HLE、GPQA、IFBench 上 Qwen 更高；均分差 1.1。截图 29。
8. 完整评测设置、硬件环境与复现说明“正在随技术报告发布”，即发布时没有（博客原话）。截图 29。
9. 博客内部：120B 的数字表与图不一致（53.6/81.3 对 51.3/75.3）。截图 29、30。
10. 许可证：博客称 open-sourcing / open-source model；模型权重用 Edge0 Community Model License 1.0 并叠加 Qwen Community License，禁止未经书面许可售卖、做商业模型服务或独立商业编码/办公助手。截图 32。
11. 仓库最低要求与博客“29GB / 48GB laptop”口径不同：`edge0-infer/docs/edge0-145b.md` 写 “At least 16 GiB of GPU memory and 20 GiB of system memory”（Linux/Windows + NVIDIA + CUDA 13）；README 又写 16 GiB + 20 GiB 是“test limit, not a minimum system requirement”。
12. 演示（Blender、无人机、PCB、视频）的成本对比（“$12,000-18,000”“more than $150 in cloud token charges”）均为博客自述，无提示词、无复现材料（博客文字；未存演示视频）。
13. 公司数字全部自报：“Over the past year, we released 200 models across four families”“more than 5 million production devices”“15 million downloads within three months”“more than 110,000 customer organizations”，博客自注 “come from company papers and internal testing”。

社区质疑线索（L6；HN 评论得分不对外显示，故无分数；链接形如 https://news.ycombinator.com/item?id=<id>）：
- 50478564 daemonologist：“In the ‘Under the Hood of Edge0’ section it mentions that it needs another 40 GB of system memory for experts plus 48 GB of SSD storage … The 29 GB headline seems to just be a flat lie (/the LLM writing this article getting confused about what memory usage it's reading).”
- 50479575 niek_pas：“The image under ‘Competitive capability with a smaller memory footprint’ explicitly claims … (29 GB vs ~70 GB) and runs on a 48GB laptop. So either they’re lying or confused or I’m confused.”
- 50479162 Aurornis：“Their edge0-strata engine is just a fork of the strata project … It's weird to see this marketing web page imply that it was all new and novel work that they did, though.”；50479136：“Based on Qwen3.8-Flash-Next and Strata … it's interesting how much marketing speak was layered on their website to hide this fact.”
- 50477240 coder543：“They literally talked about MacBooks in the blog post … It is strange that they also benchmarked an M2 Ultra, but they are specifically not releasing a Mac runtime today.”；50478454 theroadnotbacon 引用博客 “day-one support covers macOS …” 后写 “there's no guidance on how to run on a mac at all.”；50479243 phillennium：“The blog post mentions Mac repeatedly … But the released stuff so far seems to be CUDA only.”
- 50482877 embedding-shape：“They're doing offloading, the full model doesn't live in VRAM at all.”
- 50476588 fesoliveira：“if 3rd party testing and benchmarking hold the published values this is very exciting. I am a bit skeptical given the model size”。

## 五、第三方实测（C6，L6，HN item 50475713；存档 `c-hn-50475713.json`、截图 35）

- 50483169 mapontosevenths（2026-11-13T13:25:46Z = 21:25 北京）：“I tried it - it doesn't work with my configuration and setup on a DGX Spark. … It spent 8,000+ tokens at default thinking and never produced a coherent answer.” 自述，提示词未公开。
- 50482710 kjjohnsen（11-13T12:43:50Z = 20:43 北京）：“Just ask Fable 5.2 for a version that works on Mac. Magically happens... There are just a few tweaks and voila! Getting about 50 tok/s on an m5 max at nvfp4.” 自述；是自行移植，不是官方 macOS 版；内存未写。
- 50480064 veeragoni（11-13T08:17:56Z = 16:17 北京）：“i have it working but not the Edge0-145B. just the 35B”（指 Mac 上 Edge0-35B，链接 chatpanel.net）。
- 50481578 dust42（与 Edge0 无关的参照）：Qwen3.8-Flash-Next 在 5090 笔记本 64GB 内存上跑 NVFP4 “120-140t/s decode and 3000-4000t/s prefill”，“Despite the claim of 29GB RAM for Edge0, in order to be fast you need to have at least a 128GB Mac.”（对 Mac 的判断为评论者自述）。
- GitHub issues（官方仓库，未登录 API）：#206 “Edge0-145B-q3k on Windows / RTX 5070 Ti (sm_120): short prompts appear corrupted, long prompts fine”（11-13T13:18Z，开放）；#200 “[strata] Edge0-145B NVFP4 prefill hangs RTX PRO 6000 (sm_120)”（11-13T06:34Z）；#201 “Support for Apple Silicon (Metal) for Edge0 120B / 145B”（11-13T10:08Z）；#197 AMD GPU 支持请求；#199（已合并）修 DGX Spark 上的 prefill 挂起。
- Reddit r/LocalLLaMA：`barking heat` 与 `opencli reddit search` 均被拦（login 重定向 / SECURITY_BLOCK 403），未重试，**未见**。
- 独立评测站（AA、LiveBench 等）：未检索到本模型条目（未专项查，标“未见”）。

## 六、公司背景（C7，Edge0 官网原文）

- 博客 “Why Edge0” 段：“Our September release of Edge0-35B showed what that approach could deliver on a phone. Its launch post passed 1.2 million views on its first day without paid promotion in overseas communities. Researchers and technical experts at OpenAI, NVIDIA, Qualcomm, and other companies shared it, and the model reached No. 1 on Hugging Face's overall Trending chart.”
- 同文：“Over the past year, we released 200 models across four families.”；“Both Edge0 and Audio8 have reached No. 1 on Hugging Face's Trending chart.”；“That deployment work now spans more than 5 million production devices. Our open-source models passed 15 million downloads within three months. By October 2026, we had more than 110,000 customer organizations.”；“The portfolio figures come from company papers and internal testing.”
- 前作：Edge0-35B-A3B-preview 与 Edge0-8B-A1B-preview（2026-09-08 首发，GitHub README release notes）；技术报告 arXiv 2609.18063 “The Other Half of the Memory Wall: Serving 35B MoEs from SSD with Trained Routing Prediction”（2026-09-16）。HF 上 “Edge0” 组织页存档 `c-hf-org.txt`；“Trending 第一”未在 HF 存档页中逐字核对（只有公司自述），记自述。
- 关于我们（https://edge0.ai/aboutus）：“Bringing advanced AI to every level of hardware”；创始人介绍仅限公开职业信息：“Samuel Zeng · Founder & CEO … He holds a PhD in artificial intelligence and was named in MIT Technology Review's Innovators Under 35”。公司所在地官网未写（HN 评论 aktenlage、suthakamal 也问了）。博客页脚与图注出现名称 AutoArk（contact@autoark.ai），与 Edge0 的关系未解释。
- 官网首页总标语：“The world’s first on-device AI that runs on any hardware”。博客自称 “Edge0-145B is the world's first open-source on-device AI model with 145 billion parameters.”
- 文档站 docs.edge0.ai（上次修改 2026-10-21）只覆盖 8B 与 35B，没有 145B 页面（llms.txt 目录无 145B）。

## 七、夸大说法实例（C8；L5 线索，不引用，只回溯）

未找到中文媒体报道（两轮中文检索：只检到英文页；扫描称的中文标题“苹果本跑不了”未在存档范围内找到）。英文实例如下，发布时刻均转北京时间：

| 发布方 / 页面 | 原句（逐字） | 发布时刻 | 与原文出入 |
|---|---|---|---|
| emergent.sh https://emergent.sh/news/edge0-145b-open-source-145b-device-model-29gb（`c-media-emergent.*`；截图 36 上半） | 标题 “Edge0-145B: Open-Source 145B On-Device Model at 29GB”；TL;DR：“Edge0 released Edge0-145B, the first open-source 145B-parameter model running fully on personal laptops with 29GB peak memory.”；正文：“The model achieves 80+ tokens per second generation speed while consuming just 29GB of peak memory on a 48GB laptop” | 页面 “Nov 13, 2026 8:10 AM”（时区未标）；元数据 2026-11-13T02:40:47Z = 11-13 10:40 北京 | 把 29GB 写成“在 48GB 笔记本上消耗”，博客并未这样写；“running fully on personal laptops”略去 CUDA/Windows/Linux 限定与 40GB 内存 + 48GB SSD |
| DEV Community（AI Growth Lab, Shakeel Ahmed）https://dev.to/shakeelahmed/a-145b-open-model-that-runs-on-your-laptop-edge0-145b-explained-31a2（`c-media-devto.*`；截图 36 下半） | 标题 “A 145B Open Model That Runs on Your Laptop: Edge0-145B Explained”；“A 145-billion-parameter model that runs on a laptop sounds like a typo. It is not.”；“on a compatible laptop such as a high-memory MacBook Pro” | 2026-11-13T04:30:15Z = 11-13 12:30 北京 | 沿用博客的“MacBook Pro”而未查平台表；但同文承认 “Nobody outside the company has independently run this model yet”，并写 “The models are derived from Qwen3.8-Flash-Next”。页面注明原发布于 AI Growth Lab 博客，其链接 slug 为 `gemini-4-hydrogen-explained.html`（与标题不符，原因未知） |
| byteiota https://byteiota.com/edge0-145b-local-inference/（`c-media-byteiota.*`） | 标题 “Edge0-145B: Run a 145B Open-Source Model on Your Laptop”；“The result is 29 GB peak memory, 80+ tokens per second”；“On a compatible high-memory laptop it delivers 80+ tokens per second generation.” | 元数据 2026-11-13T10:07:00Z = 18:07 北京（JSON-LD 写 15:37 +05:30，同一时刻） | 标题讲笔记本；正文后面承认 “macOS users should temper expectations for the 145B: it currently requires Linux or Windows with an NVIDIA GPU and CUDA 13”，并写 “you need at minimum a 24 GB VRAM GPU”（这一“最低 24 GB”与仓库 16 GiB + 20 GiB 的口径不同） |
| Activepieces https://www.activepieces.com/blog/what-is-edge0-145b-local-ai-model-guide-for-2026（`c-media-activepieces.*`） | “Running this model at frontier speeds is achieved on a single compatible laptop, decoding above 80 tokens per second. This hardware investment allows the model to stay entirely within high-bandwidth video memory rather than spilling over into slower system RAM.”；“macOS (Apple Silicon): uses the Unified Memory Architecture (UMA) to treat the entire system RAM as a single pool” | 2026-11-13T10:52:12Z = 18:52 北京 | “stay entirely within … video memory … rather than spilling over into slower system RAM” 与博客三级存储（40GB 系统内存 + 48GB SSD）相反；把 macOS 列入可用环境；该站是自动化产品博客，带自家产品推广 |
| LavX News https://news.lavx.hu/article/edge0-releases-open-source-145-billion-parameter-model-for-local-inference-on-laptops（`c-media-lavx.*`） | “Edge0 releases open-source 145-billion-parameter model for local inference on laptops”；“day-one Edge0-145B support on macOS, Windows, and Linux, including Apple M-series chips and NVIDIA hardware” | 2026-11-13T00:49:00Z = 11-13 08:49 北京 | 转述博客的 macOS 支持，没有核对仓库；但同文也写 “The reported gains depend on internal evaluations … that outside testers still need to reproduce” |

相对克制的转述（可作对照，不算夸大）：cctest.ai（2026-11-12T23:06:17Z = 11-13 07:06 北京）“These are vendor-reported figures and will vary with hardware and configuration.”；ai-tldr.dev（日期 2026-11-12，元数据 12:00 UTC，早于博客发布时刻，原因不明）写出 README 对 macOS 的限制与 29GB 为“GPU-side peak”（该解释是其推断，博客没写）。

## 八、扫描说法勘误

| 扫描说法 | 结论 | 依据 |
|---|---|---|
| 标题“苹果本跑不了” | **博客原文与之相反，官方仓库与之相符**：博客称 macOS / Apple M 系列 day-one 支持，并以 MacBook Pro 举例；GitHub README 平台表 macOS 一栏为“—”，模型文档“do not establish macOS release availability”，桌面 App 仅 Windows，仓库有 issue #201 请求 Apple Silicon 版。严格说“跑不了”是“官方暂无 macOS 版”，不是“硬件不可能”：HN 有人自行移植后在 M5 Max 上约 50 tok/s（自述）；仓库速度表里 M2 Ultra（Metal）有 Q3_K 数字（60.9 / 57.0 tok/s），但没有对应发布。 | 截图 31、34、33；`c-gh-readme.txt`、`c-gh-model-doc.txt` |
| “不是新训练、衍生自 Qwen3.8-Flash-Next”，依据“权重命名与平台说明” | **“衍生自 Qwen3.8-Flash-Next”已找到官方原文**（README、模型文档、HF `base_model`、NOTICE）；依据不止“命名”。**“不是新训练”未找到一手来源**：官方只说 derived、“trained natively at 4 bits”、“base model”，微调 / 再训练 / 结构改造的具体方式未写；NOTICE 明写 “No verified training-source commit is asserted”；博客说“train … from the outset”。 | 截图 32、34；`c-hf-145b-notice.txt` |
| “Hugging Face 同日放出 NVFP4 权重” | 不是“同日”：HF 仓库创建 11-11 13:22（北京），NVFP4 权重提交 11-11 20:59，博客 11-13 02:00（北京）；公开时点存档未显示。 | `c-hf-145b-commits.json`、`c-hf-145b-api.json` |
| 博客说“笔记本，29GB” | 29GB 未写明口径；博客自己写专家库 40GB 在系统内存、词表 48GB 在 SSD；见第一节 3、第二节 C2。 | 截图 25–27 |
| （派工单）“Qwen3.8-Flash-Next … estimated 70GB footprint for standard 4-bit” | 估算依据未写；Qwen 卡写 125B + 51B n-gram embedding + 4B MTP；n-gram 表按设计可放磁盘（HN 50480413 Tade0 评论），因此“70GB”是否把 51B 表计入未写 | `c-qwen-card.txt`；HN |

## 九、热度（`barking heat`，`c-heat.json`；HN 帖 `c-hn-50475713.json`）

- HN item 50475713：295 分 / 87 评（页面与 Algolia，11-13 22:35–22:40 北京；heat 22:30 取为 294 / 87），提交者 sroussey，2026-11-12T23:06:17Z = 11-13 07:06 北京。HN 同一 URL 按域名搜索仅此一帖。
- Reddit：失败（见 capture-log）。AIHOT 等聚合站：本组未查（派工单未要求）。
- GitHub Edge0-AI/Edge0：10,826 星 / 1,653 fork / 34 个开放 issue（API，11-13 22:29 北京）；HF Edge0-145B-NVFP4：下载 14,134（“last month”口径，HF 自己的统计）、赞 113；Edge0-145B-q3k：下载 8,317、赞 37；Edge0-120B-NVFP4：下载 112、赞 18。

## 十、截图清单（C 组 25–36，均已逐张打开目视确认）

| 文件 | 来源与范围 | 备注 |
|---|---|---|
| 25-blog-perf-table.png | 博客 “Edge0-145B Laptop Performance” 小节：标题、导语、性能表、“28GB peak memory” 段 | 深色底页面；底部最后一行字下缘略贴边，文字可读 |
| 26-blog-under-the-hood.png | 博客 “Under the Hood” 四层设计：架构（约 10B 激活）、训练（from the outset）、加载（40GB / 48GB）、调度 | 深色底 |
| 27-blog-memory-compare-table.png | 博客 “Competitive capability with a smaller memory footprint”：段落（29GB、70GB、48GB laptop）+ 均分与内存表 + 表注（measured / estimated） | 深色底 |
| 28-blog-memory-chart.png | 博客内置图 “Quality and memory footprint”：含 48GB laptop / 64GB workstation 线与图注 | 博客配图原图（不重绘）；深色页面上的白底图 |
| 29-blog-benchmark-table.png | 博客六项基准全表（含 9 个模型行）+ 两条表注（internal evaluations 等） | 深色底；完整表头与全部行列 |
| 30-blog-benchmark-chart.png | 博客内置图 “Edge0 benchmark performance”（图中 120B 为 51.3 / 75.3） | 博客配图原图 |
| 31-blog-hardware-support.png | 博客 “Hardware Optimizations” 末尾：95% 芯片平台、“does not mean every device can run Edge0-145B”、5 million / 15 million / 110,000、macOS day-one 句 | 深色底 |
| 32-hf-card-header-basemodel.png | HF Edge0-145B-NVFP4 页顶部：标签、许可证、下载数、Model tree（Base model Qwen/Qwen3.8-Flash-Next → this model） | HF 深色主题；匿名无头 Chrome，页面上的 “Log In” 为 HF 自身按钮，无账号信息 |
| 33-hf-card-throughput-speed.png | HF 卡：“Throughput measured on RTX 5090 D 24 GB · NVFP4”、23.25 GiB VRAM + 38.27 GiB host RAM 注、完整 Speed 表（7 行）、Measurement conditions | 上下两段拼接（间隔 24 px 背景色），卡片区为白底 |
| 34-gh-readme-platforms-derived.png | GitHub README：About、Platforms 表（macOS 一栏“—”）、“derived from Qwen3.8-Flash-Next”、“Linux or Windows with an NVIDIA GPU and CUDA 13” | GitHub 深色主题 |
| 35-hn-thread-derived-qwen.png | HN item 50475713：标题与 295 分 / 87 评（页面时刻显示 “15 hours ago”），含 Aurornis 等关于 derived from Qwen 的一段讨论 | 上下两段拼接；页头橙色为 HN 自身样式；提交者用户名为 HN 公开信息 |
| 36-media-headlines-laptop.png | 上：emergent.sh 标题、TL;DR、“29GB … on a 48GB laptop”；下：DEV Community 标题与首段 “runs on a laptop … MacBook Pro” | 上下两段拼接；作者头像与署名为这两个站点自身页面上的公开元素，非抓取者 |

未单独截的：模型卡“48GB laptop”之外的硬件段、“internal evaluations”表注已含在 27 与 29 中；Qwen 官方卡对照数字（C4b）只存文本（`c-qwen-card.txt`），如需配图可另截。

## 十一、假设与未核事项

- 把博客元数据时间 2026-11-12T18:00:00.000Z 视作发布时刻（博客页只写 “November 12, 2026”）；该时刻整点，可能是名义时间。
- 把 GitHub 与 HF 的提交时间（UTC）直接换算为北京时间；HF 与 GitHub 修订的“博客发布前存在”依据是 `raw/<commit>` 取得的文件内容。
- “与 Qwen 官方卡数字相同”仅对 Qwen 卡里出现的四项基准成立；Terminal-Bench 2.1 与 AA-LCR 未在 Qwen 卡中检到。
- Edge0 是否复跑过 Qwen 等行，未写；“很可能直接取自 Qwen 卡”为推断。
- “48GB 词表≈51.2B 参数 fp8”为我的换算，博客只写 “vocabulary data”。
- 未下载权重、未运行模型；未验证任何速度或内存数字。
- 未用 Wayback；未对 HN 评论“得分”取数（HN 不公开他人评论分数）。
