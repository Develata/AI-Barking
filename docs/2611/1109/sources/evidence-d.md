# 1109 期取证：D 组（速览）

执行方：claude-haiku-5-5（派工单 `.handoff/2026-11-09-1109-evidence.md` 的 D 组）。
时间：北京时间（UTC+8）；官方原时区一并保留。热度为 `barking heat` 于 2026-11-10 15:59 BJ 取得（`d-heat.json`）。
截图：本组不截图（派工单要求）。存档与调用记录见 `capture-log-d.md`。

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

存档对照（`sources/` 下文件名，均带 `d-` 前缀）：

| 条目 | 存档文件 |
|---|---|
| D1 | `d-d1-meta-muse-ember.html|.txt`；评测报告 PDF `d-d1-meta-eval-report.txt`（文本，PDF 原件只存本地） |
| D2 | `d-d2-google-tpu-8i-max-8tx.html|.txt` |
| D3 | `d-d3-eetimes-opencli.txt`（EE Times 正文，opencli 提取）；`d-d3-deepx-newsroom.html|.txt`（DeepX 新闻页快照）；`d-d3-deepx-eetimes.headers`（最初失败的响应头） |
| D4 | `d-d4-asahi-progress-7-3.html|.txt`、`d-d4-lwn-1105936.html|.txt` |
| D5 | `d-d5-psi-aaronson-ambainis.html|.txt`、`d-d5-github-aaronson-ambainis.html|.txt`、`d-d5-aaronson-post-10295.html|.txt`（博客单篇） ；`d-d5-aaronson-blog.*`（首页快照，待删） |
| D6 | `d-d6-claude-dev-which-model.html|.txt`（开发者博客，主选）；`d-d6-anthropic-choosing-model.html|.txt`（官方文档页，对照）；`d-d6-claude-dev-blog.*`（博客索引） |

---

## D1 Meta 发布最小模型 Muse Ember

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 发布：muse-ember-1.0 上线，价格与上下文 | L1（Meta 官方博客） | https://research.meta.ai/blog/muse-ember | "Muse Ember is available today on Muse Code and Meta Model API as muse-ember-1.0. It costs $0.10 per million input tokens, $0.004 per million cached input tokens and $0.50 per million output tokens, with a 1M-token context window and up to 512K output tokens." | 无 | 脚注 2 原句：“Pricing shown is for prompts up to 200K tokens. For longer prompts, Muse Ember costs $0.30 per million input tokens, $0.012 per million cached input tokens and $1.50 per million output tokens.” 扫描的 $0.10/$0.50 与 1M 上下文成立，但漏了超 200K 的分档价 | 已找到 |
| 发布时刻 | L1 | 同上 | 页面 “November 9, 2026·4 minute read”；页面 meta 为 `2026-11-09T00:00:00.000Z` | 无 | 官方未给时刻（meta 时间为 UTC 整点 00:00，疑为日期占位，未核）；北京时间记 11-09 | 部分支持 |
| 权重是否开放（扫描：“权重不开放”） | L1 | 同上；评测报告 PDF 同样未提 | 发布页与评测报告 PDF 中均未见对 Muse Ember 权重开放与否的说明；页面仅有 “about 4x to 13x faster than leading open-weight models”，指其他模型 | 无 | 原文沉默，不能判定“不开放”；未查 Muse Code 或模型卡 | 未找到一手来源 |
| 成本口径 | L1 | 同上 | 图注：“Muse Ember costs are estimates at proposed pricing.” 图表来源行：“Muse Ember cost at list pricing.” 脚注 1：“cost per task uses each model's list pricing” | 无 | 两种口径并列：图注用 proposed pricing 估算；图表来源行与脚注 1 用 list pricing。正文要照引，并注明依据哪一句 | 部分支持 |
| AA 指数：与 Haiku 5.5 同为 36，每任务 $0.14 对 $0.41 | L1（Meta 引用 Artificial Analysis 数据；AA 原页未存档，故不列 L3） | 同上 | 正文：“it matches Haiku's best score at about a third of the cost and beats every Luna setting.”；正文：“about $0.14 per Artificial Analysis task”；图表数据（按 Meta 图表）：Muse Ember max 35.8 / $0.136 每任务；Claude Haiku 5.5 max 36.2 / $0.411；评测表（扁平化文本）General Artificial Analysis Index 一行，Muse Ember 与 Haiku 均显示 “Best result. 36” | 无 | 测试：AA Intelligence Index v5（脚注 1）；档位：均为 max；单位：每任务美元。取整后“同为 36”，原始为 35.8 对 36.2 | 部分支持 |
| 评测来源：哪些是 Meta 内部，哪些是 AA 运行 | L1（Meta 评测报告 PDF） | 同上；`d-d1-meta-eval-report.txt` | 见下方“D1 评测来源清单” | 无 | 同一 Terminal-Bench 行三家用的 harness 不同（脚注 1 与 PDF） | 已找到 |
| HN 热度 | L5 线索 | https://news.ycombinator.com/item?id=50429024 | 标题 “Muse Ember: Our smallest, most efficient model” | 无 | 2026-11-10 01:58:46 BJ 发帖；15:59 BJ 取得 291 分、118 条评论（嵌套计数）。扫描写 283 分，为取数时点不同 | 已找到 |

**D1 评测来源清单**（Meta 发布页图表标注 + 评测报告 PDF 方法节）

| 评测 | 结果来源 | 依据 |
|---|---|---|
| Artificial Analysis Intelligence Index v5 | AA 运行（Meta 引用） | 页面脚注 1；PDF 方法节第 31–40 行 |
| AutomationBench-AA | AA 运行（Meta 引用，pass@1） | 页面图注 “Artificial Analysis \| November 9, 2026. Strict pass@1.” |
| GDP.pdf-AA | AA 运行 | 页面评测表标注 “-AA” |
| Coding Agent Index-AA | AA 运行（Coding Agent Index v2.0） | PDF 第 109–112 行 |
| FrontierCode 2.0-AA | AA 运行，私有 132 题集 | PDF 第 113–116 行 |
| Terminal-Bench 4.0 | Muse Ember：AA 运行，mini-swe-agent；Haiku 5.5：Anthropic 自报（Claude Code，系统卡）；GPT-6 Luna：Codex，TB 排行榜 | PDF 第 102–108 行；页面脚注 1 |
| Agents' Last Exam | Meta 内部（页面图注 “Meta internal evaluation. Linux subset.”） | 页面图表紧随 “Agents' Last Exam” 标题 |
| Personal Superintelligence Index (PSI) | Meta 内部综合指数（八项） | PDF 第 60–61 行；页面评测表 “Internal composite” |
| OSWorld 2.1 | Meta 内部框架（非官方 runner），部分分（partial-credit） | 页面图注 “Meta internal evaluation. Partial-credit reward.”；PDF 第 136–146 行 |
| PerceptionBench (w/ tools) | Meta 内部框架，3,000 题，官方评判 gpt-oss-120b | 页面图注 “Meta internal evaluation. With tools.”；PDF 第 148–150 行 |

---

## D2 Google 发布 TPU 8tx 与 TPU 8i Max

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 8tx：低延迟 | L1（Google 官方博客） | https://blog.google/innovation-and-ai/infrastructure-and-cloud/global-network/tpu-8i-max-and-8tx/ | "delivering up to 2x lower latency compared to TPU 8t with a near-zero performance penalty." | 无 | 基线是 TPU 8t（相对值，非绝对延迟） | 已找到 |
| 8tx：机密计算与主机 CPU | L1 | 同上 | "Built on the same silicon as TPU 8t and paired with 5th Generation AMD EPYC CPU hosts, TPU 8tx builds security directly into the hardware layer" | 无 | 扫描的“8tx 配 AMD EPYC 主机 CPU 做机密计算”成立 | 已找到 |
| 8i Max：翻倍的算力、SRAM、带宽 | L1 | 同上 | "delivering double the compute, double the SRAM and double the HBM bandwidth of TPU 8i" | 无 | 基线是 TPU 8i（相对值）；“fits twice the silicon within the existing rack footprint” 为同机架对比 | 已找到 |
| 绝对规格、价格、上市时间 | L1 | 同上 | 博文正文未给绝对规格、价格，也未见上市时间的句子（全文检索 “availab”“price” 无对应句） | 无 | 本博文未给；其他 Google Cloud 页面未查。扫描的“全是相对倍数，无价格与上市日期”在本文成立 | 部分支持 |
| 发布时刻 | L1 | 同上 | 页面显示 “Nov 09, 2026”；页面结构化数据 `datePublished: 2026-11-09T13:00:00+00:00` | 无 | 北京时间 2026-11-09 21:00（按结构化数据换算）；页面可见日期仅到日 | 已找到 |
| HN 热度 | L5 线索 | https://news.ycombinator.com/item?id=50432831 | 标题 “TPU 8tx and TPU 8i Max” | 无 | 2026-11-10 05:56:57 BJ 发帖；15:59 BJ 取得 244 分、75 条评论 | 已找到 |

---

## D3 DeepX 第二代芯片 DX-M2 流片（6.7 倍性能）

本组先前两次抓取 EE Times 失败；后用 opencli `extract` 取得正文（存档 `d-d3-eetimes-opencli.txt`），文章页面日期 “11.09.2026”（作者 Sally Ward-Foxton，EE Times 格式 MM.DD.YYYY，即 2026-11-09），页面未给时刻。下表以该文章为一手（L4 媒体）原文；DeepX 官方新闻稿未找到。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 流片 | L4（EE Times 报道；DeepX 官方新闻稿未找到） | https://www.eetimes.com/deepx-tapes-out-second-gen-chip-projects-6-7x-performance-gain-with-lpddr5x-pim/ | "South Korean edge AI chip startup DeepX has taped out its second-generation AI accelerator with access to custom processor-in-memory (PIM) technology from Samsung." | 无 | 单一媒体来源；页面标题为 “DeepX Tapes Out Second-Gen Chip, Projects 6.7× Performance Gain With LPDDR5x-PIM” | 部分支持 |
| 6.7 倍性能：仿真与基线 | L4（转述 DeepX） | 同上 | "DeepX’s simulations show that offloading GEMV operations from the DX-M2 into LPDDR5x-PIM will result in 6.7× the tokens per second for Llama3.1-8B versus the DX-M2 alone." | 无 | 仿真值，不是实测；基线为 DX-M2 自身（不含 PIM）；模型为 Llama3.1-8B。配图说明：“Offloading GEMV operations into memory will improve tokens per second by 6.7× (based on Llama3.1-8B), according to DeepX, at the expense of power consumption” | 部分支持 |
| 功耗代价 | L4（转述 DeepX CEO） | 同上 | "The only drawback, Kim explained, is that the extra compute doesn’t come for free—6.7× the performance needs 2.7× the power consumption overall." | 无 | 原文是总功耗为原来的 2.7 倍（overall），扫描写“需多 2.7 倍功耗”措辞不准（Claude 据反向核验第一轮改正） | 已找到 |
| PIM 样品与测试 | L4（CEO 引述） | 同上 | "We already received LPDDR5x PIM samples and we are testing the operations and performance in-house." | 无 | 样品与测试由 DeepX 自述；编译器仍为仿真阶段（"we couldn't finish the compiler yet"） | 部分支持 |
| 样品与量产时间 | L4（计划，非官方公告） | 同上 | "In the meantime, samples of the M2 are expected in March 2027, with mass production targeted for fall 2027." | 无 | 扫描的“样品 2027 年 3 月、量产 2027 年秋”与文章一致；属计划（"expected"、"targeted"） | 部分支持 |
| 规格：功耗、TOPS、带宽、适用模型 | L4 | 同上 | 摘要：“enabling inference of models with up to 100 billion parameters at less than 5 W.”；正文：“80 TOPS can now be achieved.”；“Memory bandwidth has tripled from 50 to 150 GB/s” | 无 | 模型范围按文章为 20B–100B 参数（约 3B–20B 激活）、10–30 tokens/s，依模型而定 | 已找到 |
| 官方核对 | — | https://deepx.ai/company/news/（新闻页快照，最新条目 09 July, 2026）；https://deepx.ai/products/dx-m2/（产品页，curl 取得，未入库） | 新闻页与产品页快照中未见流片、6.7 倍或 PIM 的表述 | 无 | 快照可能不全（新闻页 HTML 体积 600 001 字节，疑被截断，未核） | 未找到一手来源 |
| HN 热度 | L5 线索 | https://news.ycombinator.com/item?id=50429545 | 标题 “DeepX claims inference up to 100B parameters at less than 5 W”（与文章摘要同一宣称；HN 链接即上述 EE Times 文章） | 无 | 2026-11-10 02:33:58 BJ 发帖；15:59 BJ 取得 152 分、24 条评论 | 已找到 |
| 旧时间表线索（未存档，不作依据） | — | 搜索摘要中的 EE Times Japan CES 2026 报道（2026-01 前后） | 摘要称 “mass production expected to start by late 2026”（搜索摘要，未取得原文） | 无 | 此为早期报道的旧计划，与 2026-11-09 文章的 2027 年量产不同；不作冲突认定，仅作线索 | 未找到一手来源 |

---

## D4 Asahi Linux 许可证改为 GPL

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 转型“开始”还是“已完成” | L1（Asahi 官方进展报告 7.3） | https://asahilinux.org/2026/11/progress-report-7-3/ | "we made the difficult decision to begin the process of transitioning all of our repositories to the GNU General Public License version 2 or later." | 无 | 原文是“开始转型”的过程，不是已完成 | 已找到 |
| 许可证目标 | L1 | 同上 | "Going forward all Asahi Linux code will be licenced under strong copyleft licences only." | 无 | 计划范围是“所有仓库”；原文未说明已完成的仓库数 | 已找到 |
| 指控：AI 改写挪用未发布代码 | L1，单方陈述 | 同上 | "Some time ago, we were made aware of multiple attempts to exploit our permissive licensing to hijack our work … In some extreme cases, in-development feature code was being lifted straight out of our personal forks and published without attribution or lazily rewritten with LLM assistance to obfuscate its origin." | 无 | 限定词：“multiple attempts”“In some extreme cases”；未点名任何个人、公司或产品；未附证据。“未发布”对应 “in-development feature code”。“frontier AI labs”仅作为背景提到，未指明对象 | 部分支持 |
| 发布时刻 | L1 | 同上；LWN 转载 | 页面 meta：`2026-11-07`；LWN：“Posted November 7, 2026 by corbet” | 无 | 官方未给时刻（只有日期）。LWN https://lwn.net/Articles/1105936/ 只作转载核对 | 已找到 |
| HN 热度 | L5 线索 | https://news.ycombinator.com/item?id=50420532 | 标题 “Asahi Linux Drops MIT, Goes Full GPL After AI Used to Steal Unreleased Code” | 无 | 2026-11-09 11:06:55 BJ 发帖；15:59 BJ 取得 70 分、35 条评论。备注：HN 标题把“开始转型”写成“Drops MIT, Goes Full GPL”，把“some extreme cases”写成“AI Used to Steal”，与原文措辞不符 | 已找到 |

---

## D5 PSI 称其研究系统证明 Aaronson–Ambainis 猜想

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 模型身份（逐字） | L1（PSI 官方研究页） | https://psi.inc/research/aaronson-ambainis/ | "a research system developed internally at Physical Superintelligence (PSI), running a non-public Anthropic model, found a proof for it" ；"Instances of a non-public Anthropic model were used in the run." | 无 | 原文为 “non-public Anthropic model”，不是 Fable 5.2。PSI 页面、GitHub README 中检索 “Fable” 均为 0 | 已找到 |
| 证明完成与 Lean 核验 | L1 | 同上 | "The system had a complete proof of the conjecture during the third day of the run, and Lean, a proof assistant in which a computer checks every step, accepted it." | 无 | PSI 自述；Lean 覆盖范围见下一行 | 已找到 |
| Lean 覆盖范围 | L1（GitHub 仓库 README） | https://github.com/physical-superintelligence/aaronson-ambainis | "The Lean development is complete: the four theorems above are proved with no placeholder and no added axiom" ；"The fourth theorem has a hypothesis: FbsDegreeBound is Theorem 1.7 of A. Agarwal and S. Ben-David, arXiv:2410.19062 …, which is not formalized here." | 无 | 第四条定理（aaronson_ambainis_of_fbsDegreeBound）以 Agarwal–Ben-David Theorem 1.7 为前提，该前提未在 Lean 中形式化。README 另写：“The paper is written for human readers and is not itself machine-checked.” | 部分支持 |
| 人工与 AI 角色 | L1 | 同上（GitHub README 与 PSI 页） | "The proof, the paper and the formalization were produced by AI models" ；"David Turturean reviewed the paper and is responsible for its correctness." | 无 | PSI 页：“Jim Halverson, Advisor, audited the proof. Phiala Shanahan, Advisor, commented on the Lean formalization.” | 已找到 |
| Aaronson 的评价（限定） | L1（本人博客） | https://scottaaronson.blog/?p=10295 | "I believe the proof is correct for two reasons. Firstly, there’s a Lean certificate, and secondly, human experts … have already gone over it and understood it." ；"I haven’t yet understood the proof, although I hope to." | 无 | 博客日期 “November 8th, 2026”（时区未标，官方未给时刻） | 已找到 |
| Aaronson 对 Shor 等的影响 | L1（本人博客） | 同上 | "It has zero effect, for example, on Shor’s factoring algorithm, which uses a highly non-random (periodic) oracle for the key internal step." ；"What it does, is to rule out a potential class of quantum speedups that most of us didn’t believe would exist anyway" | 无 | 原文限定：排除的是“对随机输入的是非题、相对随机 oracle 的超多项式量子优势”，不是否定量子优势 | 已找到 |
| Aaronson 对模型与 harness 的说明 | L1 | 同上 | Aaronson 转述 PSI：“using their proprietary research harness on an internal Claude model” ；"(What role the harness played, compared to the internal model, is something I don’t know.)" | 无 | 他转述的是 “internal Claude model”，与 PSI 的 “non-public Anthropic model” 用词不同，两处都照原文记 | 已找到 |
| Anthropic 是否回应 | L4 线索（搜索摘要，未存档） | — | 搜索摘要称据 Business Insider 报道，Anthropic 未回复关于所用未发布模型的置评请求 | 无 | 本组做了一次专门检索，结论仅限该次检索；未取得一手回应 | 未找到一手来源 |
| 预印本编号（扫描：arXiv 预印本） | — | 页面仅链到 GitHub 上的论文 PDF：https://github.com/physical-superintelligence/aaronson-ambainis/blob/main/paper/aaronson_ambainis_paper.pdf | — | 无 | PDF 未取；未找到 arXiv 编号 | 未找到一手来源 |
| 另有 5 页证明已上传 arXiv（读者评论称） | L6 线索 | https://scottaaronson.blog/?p=10295（评论 #12，Thomas Huffstutler） | "My advisor Supartha Podder, my fellow PhD student David Miloschewsky, and I, have uploaded a fully self-contained short 5-page proof of the AA conjecture to arXiv. Hopefully, it will appear Monday night." | 无 | 读者评论自述，未附 arXiv 编号；仅作线索，进正文前须回到 arXiv 核实 | 未找到一手来源 |
| 中文稿“Fable 5.2 攻破” | L4–L5 | 本组仅做一次中文检索 | 未找到中文稿把此事写成 “Fable 5.2” 的例子；检索结果多为英文稿 | 无 | 检索只一次，结论仅限本次检索范围 | 未找到一手来源 |
| 发布时刻 | L1 | https://psi.inc/research/aaronson-ambainis/ | 页面日期 “November 6, 2026” | 无 | 官方未给时刻；HN 最早提交 2026-11-06 23:45 BJ（见下行） | 已找到 |
| HN 热度（PSI 页） | L5 线索 | https://news.ycombinator.com/item?id=50393770 | 标题 “A proof of the Aaronson‑Ambainis conjecture” | 无 | 2026-11-06 23:45 BJ 发帖；15:59 BJ 取得 11 分、2 条评论。另有 2 个同链接重复提交：50402211（11-07 14:38，3 分）、50432324（11-10 05:19，3 分） | 已找到 |
| HN 热度（Aaronson 博客） | L5 线索 | https://news.ycombinator.com/item?id=50425945 | 标题 “Aaronson-Ambainis Conjecture proved (By AI, of course)” | 无 | 2026-11-09 22:14 BJ 发帖；15:59 BJ 取得 1 分、0 条评论 | 已找到 |

---

## D6 可选：Anthropic 模型选型指南

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 开发者博客的模型选型指南（主选） | L1（claude.dev 博客，页面标题 “claude.dev Blog”） | https://claude.dev/blog/which-claude-model-should-you-build-with/ | 标题 “Which Claude model should you build with? Fable, Opus, Sonnet, or Haiku”；"The quick answer is that you should start most work on Opus 5.5 at its default effort level, medium." ；"Move further down to Haiku 5.5 for the simplest high-volume work, such as classification, routing, and extraction." | 无 | 页面日期 “Nov 09, 2026”；官方未给时刻。价格口径：“Prices are Claude API list prices.” | 已找到 |
| 官方文档页对照 | L1（platform.claude.com 文档） | https://platform.claude.com/docs/en/about-claude/models/choosing-a-model | "Most workloads start with Claude Opus 5.5." ；"On Claude Fable 5.2, Claude Fable 5.1, Claude Opus 5, and Claude Sonnet 5.5, start with the default (high) and adjust up or down based on your evals. On Claude Opus 5.5 and Claude Haiku 5.5 the default is medium" | 无 | 文档页未给日期 | 已找到 |
| HN 热度 | — | 未检索 | — | 无 | 本组只对有 HN 条目的线索取热度，未就此页做单独检索 | 未找到一手来源 |

普通读者门槛：由 Claude 判断（派工单要求）。

---

## 扫描说法勘误

1. D1 “权重不开放”：发布页与评测报告 PDF 均未说明 Muse Ember 权重是否开放，无法核实。
2. D1 “成本按 proposed pricing 估算”：只有图注写 proposed pricing；图表来源行与脚注 1 写 list pricing。正文照引两种说法即可。
3. D1 “AA 指数与 Haiku 5.5 同为 36”：页面评测表四舍五入后都是 36；图表原始数值为 35.8（Muse Ember max）对 36.2（Haiku 5.5 max）。
4. D1 价格与上下文：$0.10 / $0.50 与 1M 上下文成立；扫描未记录的是超过 200K 上下文的分档价 $0.30 / $1.50。
5. D2 “全是相对自家 8t/8i 的倍数”：基本成立；8tx 的延迟基线为 TPU 8t，8i Max 为 TPU 8i。博文未给绝对规格、价格或上市时间。“8tx 配 AMD EPYC 主机 CPU”成立。
6. D3 “样品 2027 年 3 月、量产 2027 年秋”：与 EE Times 文章一致（计划语气）。“流片”“6.7 倍为仿真”“总功耗为原来的 2.7 倍”均成立（扫描原写“需多 2.7 倍”，不准）。DeepX 官方新闻稿未找到。搜索摘要中的旧时间表（“by late 2026”）未存档，不作冲突依据。
7. D4 “AI 改写挪用未发布代码”：原文是 “some extreme cases”、“multiple attempts”，未点名、未附证据，只说 “in-development feature code”。HN 标题把“开始转型”写成“Drops MIT, Goes Full GPL”，与原文不符。“从 MIT 改 GPL”的开始时点正确，完成未见。
8. D5 “Lean 已验证”：PSI 与 README 都支持 Lean 核验证明，但 README 明确第四条定理以 Agarwal–Ben-David Theorem 1.7 为前提，该前提未形式化。扫描漏了这一限定。论文本身 “not itself machine-checked”。
9. D5 “arXiv 预印本”：未找到 arXiv 编号；页面只链到 GitHub PDF。
10. D5 “Aaronson 博客回应（11/8）”：成立，博客日期 November 8th, 2026。
11. D5 “Anthropic 是否回应”：一次检索未找到 Anthropic 的一手回应；搜索摘要称 Business Insider 报道 Anthropic 未回复置评请求（L4，未存档）。
12. D6 “开发者博客选型指南，官方建议日常从 Opus 5.5 起步”：开发者博客 claude.dev 2026-11-09 页面写 “start most work on Opus 5.5 at its default effort level, medium”，与扫描一致。扫描未给 URL，本组补上。

---

## 失败的抓取

- D3 EE Times：barking fetch 超时（15:30 BJ，预算不足未重试）；随后 curl HTTP/2 INTERNAL_ERROR（第二次失败）。第三次改用 opencli 的 `extract` 成功，正文已存（`d-d3-eetimes-opencli.txt`）。`d-d3-deepx-eetimes.headers` 为失败时的响应头，无正文。

## 假设

- 北京时间换算：页面结构化时间（如 TPU 8tx）按 UTC 换算；只有日期的页面写“官方未给时刻”。EE Times 的 “11.09.2026” 按 MM.DD.YYYY 读为 2026-11-09。
- 热度计数：“评论数”为 `barking heat` 输出中的嵌套计数（按 HN JSON 的 children 递归数）；分数为 15:59 BJ 取得的快照。
- Aaronson 博客的时区未标，按日期记。
- DeepX 新闻页快照可能不全（HTML 体积 600 001 字节，疑被截断），未核。
- claude.dev 博客视为“开发者博客”，依派工单 B 组对该域的一手来源认定，本组未另核其归属。
