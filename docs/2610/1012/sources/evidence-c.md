# 1013 C 组取证：智谱 GLM-Legal 与 Vals 法律评测

取证方：Sonnet 5.5（claude-sonnet-5-5）。取数时刻均为北京时间 2026-10-13 约 19:27–19:57（UTC+8，括号内为 UTC 的另注）。存档文件在本目录，前缀 `c-`；截图在 `../images/`（25–36）。抓取细节见 `capture-log-c.md`。

一手级别按 EDITORIAL.md：智谱官方页 L1；Vals 评测页 L3（仅限其自身数据）；Harvey 官方博客 L1（Harvey 自家基准的定义）；媒体与社交帖为 L4–L6，仅作线索。

## 一、清单逐条（templates/codex-evidence.md 第 4 条格式）

### C1 智谱官方发布页

官方页 https://z.ai/blog/glm-legal 是动态渲染（curl 只拿到 683 字节空壳），已用 headless Chrome 渲染取文本（`c-zai-blog-glm-legal.txt`，11459 字符）。**官方页只有日期 2026-10-12，未给时刻**（官方未给时刻）；HN 提交于 2026-10-12 16:11:45 UTC（北京 10/13 00:11:45），说明页面此时已可访问。**未找到中文官方页**：docs.bigmodel.cn 发布记录（`c-bigmodel-new-releases.html`）、zhipuai.cn 新闻列表（`c-cn-zhipuai-news.txt`，最新条目 2026/03/31）、bigmodel.cn 首页均无 GLM-Legal。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1a 发布日期与栏目 | L1 | https://z.ai/blog/glm-legal | “2026-10-12 · Research”；标题 “GLM-Legal: Advancing Agentic Legal Intelligence” | 25-zai-glm-legal-head.png | 只给日期，无时刻、无时区 | 已找到 |
| C1b 基座与定位 | L1 | 同上 | “Built on the lightweight GLM-5.3-Flash foundation, it scales domain competence through specialized mid-training coupled with high-efficiency post-training.” | 25（部分）；正文无专门截图 | 官方页未写参数量；GLM-5.3-Flash 官方文档写 320B 总参数/18B 激活（docs.z.ai，见 C2） | 已找到 |
| C1c 官方自报评测与措辞 | L1 | 同上 | “GLM-Legal achieved a 19.2% all-pass rate on Harvey LAB, while matching frontier closed-source models on LegalBench and PLawBench despite running on a significantly leaner parameter footprint.” 以及 “Across complex execution, legal reasoning, and real-world drafting, GLM-Legal matches frontier-tier performance while maintaining high inference efficiency” | 25、26-zai-benchmark-chart.png | 全文没有“第一”“超越某模型”的文字；“matches”“parity”是官方措辞。图中只比 5 个模型（见下表） | 已找到 |
| C1d 官方对 Vals 的说法 | L1 | 同上 | “The results on the Harvey Legal Agent Benchmark and LegalBench were independently evaluated by Vals. See https://www.vals.ai/models/zai_glm-legal.” | 26 | 按字面，只有 Harvey LAB 与 LegalBench 由 Vals 评；PLawBench 不在此列（官方页未说明 PLawBench 由谁评） | 已找到 |
| C1e Harvey LAB 官方措辞 | L1 | 同上 | “In real-world law firm workflows across 24 practice areas, GLM-Legal scores 19.2% and improves on its base model by 12.5 percentage points. Under Harvey's strict all-pass grading, this confirms reliable end-to-end task execution without intermediate degradation.” | 26 | 19.2 − 6.7 = 12.5，与 Vals 数据一致（19.17 − 6.67 = 12.50） | 已找到 |
| C1f 价格、上下文、开放权重、许可证 | L1 | 同上；https://z.ai/consultation/legal；https://docs.z.ai/guides/overview/pricing.md | 官方发布页无价格、上下文、权重、许可证文字。仅有：“Join the GLM-Legal Beta … Apply for API access: https://z.ai/consultation/legal”；申请页：“Complete the form to apply for GLM-Legal API access and discuss potential collaboration with our team.”；“You’ll soon be able to put these capabilities to work in AutoClaw” | 27-zai-beta-api-application.png | 价格页（取于 2026-10-13 11:35 GMT）未列 GLM-Legal，只列 GLM-5.3-Flash $0.15 输入/$0.50 输出等。官方页对“开放权重”只字未提 | 未找到一手来源（价格/权重/许可证/上下文） |
| C1g 适用法域/语言 | L1 | 同上 | 官方页未声明单一法域。文字里：“PLawBench (Chinese Localized Practice): Achieves parity with leading frontier systems on Chinese legal consultation, case analysis, and document drafting under fine-grained rubrics.”；示例 3 引 “SG PDPA s26; MY PDPA s129; ID PDP Law Arts 55–56”；示例 1、2 为合同/项目纠纷，未标法域 | 26 | Vals 的 Legal Research Bench 明确是美国法；Harvey LAB 官方称 24 个律所执业领域，未单列法域（见 C4/C5） | 部分支持（无一句话声明法域） |
| C1h 目前如何获取 | L1 | 同上 | “Whether you are a law firm, a legal tech company, an in-house legal team, or an independent practitioner, we invite you to experience GLM-Legal.” | 27 | 官方称 Beta，需申请 API；AutoClaw 为桌面端代理平台（“coming soon” 措辞见 “You’ll soon be able…”） | 已找到 |

官方图 “Legal Benchmark Evaluation”（截图 26）逐字读数（图例：GLM-Legal、GLM-5.3-Flash、Kimi K3、Claude Fable 5、GPT-6 Astra；图下标 “3 benchmarks: Harvey's Legal Agent Benchmark, LegalBench, PLawBench”，无脚注、无 effort 标注）：

| 基准 | GLM-Legal | GLM-5.3-Flash | Kimi K3 | Claude Fable 5 | GPT-6 Astra |
|---|---|---|---|---|---|
| Harvey's Legal Agent Benchmark | 19.2 | 6.7 | 12.9 | 11.3 | 5.4 |
| LegalBench | 86.2 | 83.9 | 86.2 | 88.6 | 87.1 |
| PLawBench | 77.3 | 73.8 | 76.2 | 77.7 | 69.1 |

与 Vals 对照：Harvey LAB 五个数都与 Vals 榜单一致（19.17、6.67、12.92、11.25、5.42）；LegalBench 的 GLM-Legal 86.23、Flash 83.93、Kimi K3 86.21、Claude Fable 5 88.56 均一致，**但 GPT-6 Astra 的 87.1 在 Vals LegalBench 榜单（150 个模型）里没有这一行**（`c-vals-legalbench.txt` 无 “Astra”），来源不明；PLawBench 不在 Vals（见 C5）。

### C2 开放权重、许可证、价格

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C2a HF 有无仓库 | L1 | https://huggingface.co/models?search=GLM-Legal ；`https://huggingface.co/api/models?search=GLM-Legal` ；`https://huggingface.co/api/models?author=zai-org&sort=lastModified&limit=15` | 页面：“Models 0 … No results found.”；API 搜索返回 `[]`；zai-org 最近更新的是 GLM-5.3、GLM-5.3-Flash（均为 2026-10-12 12:5x UTC 更新）等，无 GLM-Legal；`huggingface.co/zai-org/GLM-Legal` 返回 HTTP 401 | 36-hf-search-glm-legal-none.png | 2026-10-13 19:53（北京）。HF 对不存在的仓库也返回 401，故以搜索与作者列表为准 | 未找到一手来源（无仓库，无许可证、参数量、文件大小可记） |
| C2b ModelScope / GitHub | L1 | https://modelscope.cn/api/v1/dolphin/models（匿名搜索）；https://github.com/zai-org | ModelScope 搜 “GLM-Legal” 只命中第三方 2024 年的 LawGLM-LegalHi；对照搜 “GLM-5.3-Flash” 命中 ZhipuAI/GLM-5.3-Flash（搜索有效）；`github.com/zai-org/GLM-Legal` 404；zai-org 仓库列表无 GLM-Legal | — | `c-modelscope-search.txt` | 未找到一手来源 |
| C2c Vals 称“开放权重” | L3 | https://www.vals.ai/models/zai_glm-legal | “GLM Legal is an open-weight model from zAI, released October 12, 2026.”；信息格 “WEIGHTS OPEN” | 28-vals-model-page-scores.png | 这是 Vals 的标注；官方页与 HF/ModelScope 均无佐证；Vals 同页的 “GLM 5.3 Flash” 页同样写 “open-weight”（基座确为 MIT，HF 标签 license:mit）。**两者口径冲突，原样记录，不取舍** | 部分支持（仅 Vals 一方） |
| C2d 官方 API 价格 | L1 | https://docs.z.ai/guides/overview/pricing.md | 价格页只列 GLM-5.3-Flash “\$0.15 / 缓存 \$0.03 / 输出 \$0.50”、GLM-5.3-FlashX、GLM-5.3、GLM-5.2 等，未列 GLM-Legal | — | 取于 2026-10-13 11:35 GMT（北京 19:35）。GLM-Legal 为申请制 Beta，无公开价目 | 未找到一手来源 |
| C2e Vals 的价格读数 | L3 | https://www.vals.ai/models/zai_glm-legal；https://www.vals.ai/benchmarks/hlab | 页头 “TOKEN COSTS (IN/OUT) $0.07/0.25”；Updates：“GLM Legal is priced at $0.075 per million input tokens and $0.25 per million output tokens.”；榜单列 “$0.075 / $0.25” | 28、33-vals-model-page-updates.png、29-vals-hlab-leaderboard.png | $0.07 是页头对 $0.075 的显示舍入（同页 Updates 与榜单均为 $0.075）。**同一套 $0.075/$0.25 Vals 也标在 GLM 5.3 Flash 上，而 z.ai 价格页标 GLM-5.3-Flash 为 $0.15/$0.50**，二者相差一倍，来源不明（原样记录） | 已找到（Vals 口径）；与官方价格页不一致 |
| C2f 上下文、最大输出 | L3 | 同 Vals 模型页 | “CONTEXT WINDOW 1M”；“MAX OUTPUT TOKENS 128K”；超参：“Temperature: 1 / Top P: 0.95 / Top K: Default / Max Output Tokens: 128,000 / Reasoning Effort: max”；输入仅文本 | 28 | 官方 GLM-Legal 页未写；基座文档（docs.z.ai GLM-5.3-Flash）写 “support for a 1M-token context window”，且 HF 标签为 image-text-to-text，而 Vals 标 GLM Legal 仅文本输入 | 部分支持 |

### C3 Vals 模型页（独立再取）

页面 https://www.vals.ai/models/zai_glm-legal，渲染取文本 `c-vals-model-zai-glm-legal-full.txt`，另有 curl 原始 HTML `c-raw-www-vals-ai-models-zai_glm-legal.html`（服务端页面，正文被脚本延迟渲染，故以渲染文本为准）。取于北京 19:29–19:45。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C3a 三项成绩与名次 | L3 | https://www.vals.ai/models/zai_glm-legal | Legal Research Bench “41.83 % ± 3.43 24 / 78”；Harvey's Legal Agent Benchmark “19.17 % ± 3.25 7 / 79”；LegalBench “86.23 % ± 0.41 14 / 150” | 28 | 排名分母各不同（78/79/150）；页头另有 “ACCURACY 49.08 %”（标 VALS INDEX，口径页内未解释）、“LATENCY 37 min 25 s” | 已找到 |
| C3b 与 Claude 预读一致性 | L3 | 同上 | “Its best result is #7 of 79 on Harvey's Legal Agent Benchmark.” “Updates: Oct 12, 2026 zAI's GLM Legal evaluated on our legal benchmarks” | 28、33 | 发布时刻：Vals 只写 Oct 12, 2026，无时刻（官方未给时刻） | 已找到 |
| C3c LegalBench 小数不一致 | L3 | 同上 | 表 “86.23 %”；Updates 文字 “#14 of 150 on LegalBench (86.24%)” | 28、33 | Vals 同页两处数值差 0.01，原样记录 | 已找到（内部不一致） |

### C4 Harvey's Legal Agent Benchmark（Vals 榜单与方法）

榜单页 https://www.vals.ai/benchmarks/hlab（`c-vals-hlab.txt`，全 79 行含成本；取于北京 19:31），页面 “Updated 10/12/2026”。Harvey 官方博客：https://www.harvey.ai/blog/introducing-harveys-legal-agent-benchmark（`c-harvey-lab-intro.txt`）、https://www.harvey.ai/blog/legal-agent-benchmark-initial-results（`c-harvey-lab-initial-results.txt`，署 May 26, 2026）。

前十（Vals “Task Pass Rate / Cost / Test / Input/Output Cost / Duration”）：

| 名次 | 模型 | 任务通过率 | 每题成本 | 价格 入/出 | 耗时 | 页面图标 |
|---|---|---|---|---|---|---|
| 1 | Muse Spark 1.2 | 25.42% | $2.09 | $1.25 / $4.25 | 24m16s | 拒答图标 |
| 2 | Muse Spark 1.3 Max | 23.75% | $2.26 | $1.25 / $4.25 | 14m43s | 拒答图标 |
| 3 | Muse Spark 1.3 | 22.92% | $2.76 | $1.25 / $4.25 | 17m04s | — |
| 4 | Wonder Full | 21.67% | $5.41 | $4 / $20 | 53m21s | 拒答图标 |
| 5 | Muse Spark 1.1 | 20.00% | $0.80 | $1.25 / $4.25 | 12m21s | 拒答图标 |
| 6 | Gemini 4 Argon | 19.58% | $10.55 | $4 / $20 | 43m55s | — |
| 7 | **GLM Legal** | **19.17%** | **$0.15** | $0.075 / $0.25 | **1h12m** | “$”（最低成本）标 |
| 8 | Wonder | 17.08% | $7.08 | $4 / $20 | 22m50s | 拒答图标 |
| 9 | Mistral Large 4 | 15.83% | $4.18 | $1.36 / $4.18 | 39m25s | — |
| 10 | Grok 4.6 | 15.83% | $4.01 | $2 / $6 | 45m05s | — |

后续：11 Grok 4.5 12.92% $2.02；12 Kimi K3 12.92% $3.83；13 Grok 4.7 12.50% $11.13；14 MiMo V2.6 Flash 11.25% **$0.09**；15 Qwen 3.8 27B 11.25% $4.03；16 Claude Fable 5 11.25% $19.23（含回退）；34 GPT-6 Astra 5.42% $26.16；31 GLM 5.3 Flash 6.67% $0.57；26 GLM 5.3 8.33% $4.30。完整 79 行见存档。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C4a 测什么 | L3 / L1 | https://www.vals.ai/benchmarks/hlab ；https://www.harvey.ai/blog/introducing-harveys-legal-agent-benchmark | Vals：“Tests an agent's ability to complete legal work using documents, spreadsheets, presentations, and file-system tools.” Harvey：“Each task consists of an instruction, a client matter containing relevant materials, and a requirement that the agent produce a work product for review.” | 30-vals-hlab-benchmark.png | 由 Harvey（公司）出题并开源；Vals 页说明 “The results here are from the held-out set” | 已找到 |
| C4b 谁出的基准 | L1 | Harvey 博客（作者署名 Niko Grupen、Gabe Pereyra、Julio Pereyra） | “Harvey’s Legal Agent Benchmark (LAB), an open-source benchmark for legal agents.” “The first version of LAB includes more than 1,200 agent tasks across 24 legal practice areas, and is evaluated by over 75,000 expert-written rubric criteria.”（正文另写 “one of 1,250 tasks”） | 30 | 初始结果博文（May 26, 2026）写 “Earlier this month, we released … (LAB)”，故首发在 2026 年 5 月；博文页未取到首发日期 | 已找到 |
| C4c 计分：是否全部满足才算过 | L3 / L1 | Vals 方法段；Harvey 博客 | Vals：“Harvey grades each submission with two LLM judges. Each judge computes a task pass rate. A task passes only if 100% of its criteria pass, and Harvey’s final score is the average of the two judge task pass rates.” “The two judges were GPT 5.5 and Claude Sonnet 4.6. GPT 5.5 used medium reasoning and Claude Sonnet 4.6 was not modified.” Harvey：“a task is marked complete only if every criterion passes, a concept we refer to as all-pass grading” | 31-vals-hlab-methodology.png | 评分者是两个 LLM，不是律师；同页：“internet access disabled”；Vals 提交上游 harveyai/harvey-labs#76（DOCX 修订痕迹读取），“The scoring rubric is unchanged.” | 已找到 |
| C4d 题量 | L3（AA）/ 推断 | https://artificialanalysis.ai/evaluations/harvey-lab-aa ；Vals 榜单百分比 | AA：“Harvey's dataset of 120 private tasks spanning 24 legal practice areas”。Vals 页未写题数；其分数全是 1/240 的倍数（如 19.17% = 46/240，25.42% = 61/240），与 120 题 × 两位评分者一致 | — | 120 题对 Vals 属推断（来自分数粒度与 AA 同一私有集的说法），不是 Vals 原文 | 部分支持（Vals 原文未写题量） |
| C4e GLM-Legal 每题成本与名次 | L3 | https://www.vals.ai/benchmarks/hlab | 行：“7 GLM Legal 19.17% $0.15 $0.075 / $0.25 1h12m” | 29-vals-hlab-leaderboard.png | 成本为 Vals 按其价格 $0.075/$0.25 与实际 token 计（推断）；`Cost / Test` 是每个测试任务 | 已找到 |
| C4f “前十中成本最低” | L3 | https://www.vals.ai/models/zai_glm-legal | “#7 of 79 on Harvey’s Legal Agent Benchmark (19.17%) at $0.15 per test, the cheapest model in the top 10.” | 33、29 | 按上表十行：GLM Legal $0.15 最低（次低 Muse Spark 1.1 $0.80）成立。但：名次 9、10 并列 15.83%；**榜单更低成本的模型在前十之外**（MiMo V2.6 Flash $0.09，第 14 名；DeepSeek V4 Flash 0731 $0.06，第 25 名）；GLM Legal 耗时 1h12m，为前十中最长（其余 ≤ 53m21s） | 已找到（成立，需带口径） |
| C4g 前 10 名有六个在 GLM Legal 之上、且不在官方对比图里 | L3 | 同榜单 | 前六：Muse Spark 1.2/1.3 Max/1.3、Wonder Full、Muse Spark 1.1、Gemini 4 Argon | 29、26 | 官方对比图只含 Kimi K3（Vals 第 12）、Claude Fable 5（第 16）、GPT-6 Astra（第 34）与基座 | 已找到 |
| C4h 标注：回退与拒答 | L3 | Vals 榜单页 | “Fallback Enabled: Marks models whose published score includes fallback retries for refused tasks.” “Refusals: Provider refusals: the provider refused the request outright and no model in the fallback chain answered it, so the task is scored as a failure.” | 29 | 前十里 Muse Spark 1.2/1.3 Max、Wonder Full、Muse Spark 1.1、Wonder 带拒答图标；GLM Legal 行无此图标 | 已找到 |
| C4i 标准度量对照（标准准则通过率） | L3 | Vals 榜单页 “Criteria Pass Rate vs. Task Resolution” | “GLM Legal 92.5% / 19.2%”（准则通过率 / 任务通过率）；“Harvey grades a task as resolved only if every criterion passes. A model can satisfy most individual criteria and still miss task resolution credit.” | — | 官方页只引全部通过率 | 已找到 |

### C5 LegalBench 与 Legal Research Bench（三个分数不可互换）

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C5a LegalBench 测什么 | L3 | https://www.vals.ai/benchmarks/legal_bench | “Evaluating language models on a wide range of open source legal reasoning tasks.” “LegalBench, a large crowd-sourced collection of legal reasoning tasks … covering six major categories.”（issue-spotting、rule-recall、rule-conclusion、rule-application、interpretation、rhetorical understanding）；“As a single number, LegalBench is largely saturated: the top models are bunched near the ceiling (led by Claude Fable 5 at 88.56%)” | 32-vals-legalbench-dataset.png | 学术基准（Vals 标 Academic）；非代理、非文件任务。GLM Legal 86.23% 第 14/150，榜首 Claude Fable 5 88.56%、第二 Claude Fable 5.1 88.51%、第三 Gemini 4 Argon 88.30%；基座 GLM 5.3 Flash 83.93%（第 44）、GLM 5.3 84.84%（第 28）、Kimi K3 86.21%（第 15） | 已找到 |
| C5b Legal Research Bench 测什么 | L3 | https://www.vals.ai/benchmarks/legal_research | “Evaluating agents on legal research tasks across diverse areas of US law”；“Our primary metric is all-pass: a question counts as correct only if the response satisfies every rubric item”；“All grading is performed by an LLM judge”；评分者 “GPT 5.4”；“The dataset comprises 413 expert-authored, peer-reviewed questions … Test (208 samples).” | 34-vals-lrb-background-scoring.png、35-vals-lrb-leaderboard.png | 美国法（联邦与州）；专有基准（Vals 标 Proprietary，论文 arXiv 2610.00609）；每题三小时上限、五个工具。榜首并列 55.29%（Muse Spark 1.3 Max / Claude Opus 5 / Claude Fable 5.1） | 已找到 |
| C5c GLM-Legal 在 Legal Research Bench 低于基座 | L3 | 同上，榜单行 | “17 GLM 5.3 Flash 45.19% $0.12 51m07s”；“24 GLM Legal 41.83% $0.05 39m49s”；“9 GLM 5.3 49.04% $2.24 51m50s” | 35 | 此项 GLM Legal 比自己的基座 GLM 5.3 Flash 低 3.36 个百分点；官方页与 Vals Updates 都没点出 | 已找到 |
| C5d 三个分数的口径 | L3 | 上述三页 | Harvey LAB：全部准则通过才算对，LLM 评分，有文件工具，私有集，每题数十分钟、成本 $0.15–$26；LegalBench：学术基准，约 86% 为题目准确率；Legal Research Bench：全部 rubric 通过，美国法研究 | 29、32、34 | 三者量纲、判分、法域不同，名次分母 79/150/78，不可互换；Vals 页头 “Accuracy 49.08%” 是另一综合口径 | 已找到 |
| C5e PLawBench | L1/L6 | z.ai 官方页（图）；GitHub SKYLENAGE-AI/PLawBench | 官方页：“PLawBench (Chinese Localized Practice)”；GitHub 描述 “PLawBench is a rubric-based benchmark for evaluating LLMs.”；EvalScope 文档有 PLawBench 条目 | 26 | PLawBench 数字（77.3 等）只在官方图里，不在 Vals；官方页未链接其来源、评测者、效果设置；仓库最后推送 2026-02-09 | 部分支持（基准存在；官方数字无第三方佐证） |

### C6 AA（Artificial Analysis）有无评 GLM-Legal

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C6 AA 是否评了 | L3 | https://artificialanalysis.ai/evaluations/harvey-lab-aa ；https://artificialanalysis.ai/leaderboards/models | 页面（取于北京 19:36）名单（Harvey LAB-AA v1.1，“Hallucination-Gated All-Pass Rate”）无 GLM-Legal，也无 GLM-5.3-Flash 之外的 GLM-Legal 变体；`artificialanalysis.ai/models/glm-legal` 404；模型榜单页无 “GLM-Legal” | — | 取数时刻无 AA 数据。**AA 的 Harvey LAB-AA 是另一口径**：“120 private tasks”，三位评委（GPT-6 Sol、Grok 4.7、Claude Opus 5.5），含幻觉门控，榜首 Muse Wonder Full 13.1%、Grok 4.7 9.4%；AA 的 GLM-5.3-Flash 0.8%、GLM-5.3 (Max) 0.3%，而 Vals 的这两项是 6.67%、8.33%——同一基准不同评测方数值差别很大，是对照时必须点明的口径差异 | 未找到（AA 无 GLM-Legal 数据） |

### C7 HN 50058507

取数两次：北京 19:39:40（Algolia 586 分、330 条评论；Firebase 同 330）；19:55:31（Algolia 588 分、332；Firebase 590 分、332 条 descendants）。`c-hn-50058507.json/.txt`，热度快照 `c-heat.json`。链接指向 https://z.ai/blog/glm-legal （官方发布页，非 Vals）。提交者 ilreb，2026-10-12 16:11:45 UTC。HN 不公开评论分数，故下面“得分”一栏无法给出，按其所在位置与回复数记录。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C7 HN 热度 | L6 | https://news.ycombinator.com/item?id=50058507 | 标题 “GLM-Legal”，url https://z.ai/blog/glm-legal | — | 586/330（19:39 BJ）→ 588~590/332（19:55 BJ） | 已找到 |

### C8 媒体报道与夸大实例

搜索引擎对本事件收录有限；以下为实际取到的报道（均为 L4–L6，仅线索）。

| 说法 | 级 | 来源 URL | 原句（逐字） | 发布时间 | 状态 |
|---|---|---|---|---|---|
| C8a 夸大：开放权重 + 击败 | L5 | https://byteiota.com/glm-legal-z-ai-open-weight-legal-ai-beats-frontier-models/ （byteiota，署 ByteBot；`c-m-byteiota.txt`） | 标题 “GLM-Legal: Z.ai Open-Weight Legal AI Beats Frontier Models”；“a domain-specific model for legal workflows that outscored GPT-6 Astra and Claude Fable 5 on Harvey’s Legal Agent Benchmark while costing 70 times less per task ($0.15 vs. $10.55).”；“GPT-6 Astra also costs $10.55.”；“Gemini 4 Argon … the most capable commercial legal AI available”；“For a result that is, in fact, better.” | 页面只显示 “4 hours ago”（取于北京 19:5x），绝对时刻未取到 | 已找到：标题称“Open-Weight”，与正文末“GLM-Legal is still behind an application gate”自相矛盾；GPT-6 Astra 每题成本 Vals 为 $26.16，非 $10.55（$10.55 是 Gemini 4 Argon）；“最强商用法律 AI”不成立（榜首是 Muse Spark 1.2） |
| C8b 中文：“追平闭源” | L5 | https://www.oschina.net/news/502993 （开源中国；页面是动态页，正文未取到，只有搜索结果摘要） | 标题（搜索结果）：“智谱发布GLM-Legal 模型：基于轻量级底座、基准测试追平闭源” | 页面显示 2026-10-13 | 仅标题；正文未取到（渲染后仍为空壳） |
| C8c 中文：对标前沿 | L5 | 网易号（搜索结果标题） | “智谱推出法律专用模型GLM-Legal，主流法律基准对标前沿模型”；观点网稿：https://www.163.com/dy/article/L94BFCFM0519D45U.html 正文三句，无夸大（`c-m-163-guandian.txt`），发布时间 2026-10-13 12:07:18 | 观点网 10/13 12:07:18（北京） | 观点网正文已取，未夸大；网易号“对标前沿”标题仅搜索摘要 |
| C8d 社交转述（成本 70 倍） | L6 | https://x.com/0xLogicrw/status/2109828947869565336 （fxtwitter 存档 `c-fx-2109828947869565336.json`） | “在覆盖 24 类法律业务的 Harvey LAB 评测中，其任务通过率从 6.7% 提升至 19.2%，接近原版的三倍，超过 Kimi K3、Claude Fable 5 等模型，在 Vals AI 榜单中排名第七。”“根据 Vals AI 的独立评测，GLM-Legal 完成一项测试仅需 0.15 美元，而成绩接近的 Gemini 4 Argon 需要 10.55 美元，成本相差约 70 倍。不过，GLM-Legal 的平均耗时达到 72 分钟，比后者的 44 分钟慢了不少。” | 2026-10-13 02:09:54 UTC（北京 10:09:54） | 数字与 Vals 一致，并注明了耗时劣势；只是“70 倍”是对 Gemini 4 Argon 单一对比。同一内容的转载标题（ChainCatcher 经 KuCoin/Gate）：“Zhipu's Legal AI Model GLM-Legal Achieves 3x Higher Pass Rate at 1/70th Cost”，“3x”是相对自家基座，“1/70”是相对 Gemini 4 Argon（仅摘要，未取全文；gate.com 返回 403） |
| C8e 社交转述（英文简讯） | L6 | https://x.com/Chinazhidx/status/2109842300520239305 | “On Harvey LAB, GLM-Legal achieved a 19.2% all-pass rate, up 12.5 percentage points over its base model. Beta access is now open for applications.” | 2026-10-13 03:02:58 UTC（北京 11:02:58） | 无夸大 |
| C8f 英文汇总站 | L5 | https://zglg.work/en/ai/news/2026-10-13-zhipu-releases-glm-legal-a-lightweight-legal-model-matching-closed-source-riv ；https://toolnavs.com/article/2507-glm-legal-launches-zais-legal-model-posts-a-192-all-pass-rate-and-aims-at-fronti | zglg：标题 “Zhipu Releases GLM-Legal, a Lightweight Legal Model Matching …”（“which the company says matches frontier closed-source models”）；toolnavs：“every benchmark and all three cases come from Z.ai's own testing and selection”（`c-m-toolnavs.txt`） | 2026-10-13 | toolnavs 点明了自报与自选；zglg 带限定词“the company says” |

未找到：写成“全球第一法律模型”“86% 准确率能替代律师”“超越 GPT/Claude（全面）”的标题或文章——在实际取到的报道里没有；最接近的是 C8a（byteiota）。

### C9 扫描说法勘误

| 扫描说法 | 核对结果 | 依据 |
|---|---|---|
| 价格写 $0.075，Vals 页显示 $0.07 | **两者都对**：Vals 页头把 $0.075 舍入显示为 $0.07；同页 Updates 与榜单为 $0.075/$0.25。需注意这是 Vals 口径，官方价格页未列 GLM-Legal | 截图 28、33、29 |
| “6/77 旧快照” | 现榜为 7/79。若去掉 10/12 同日上榜的 Wonder Full（第 4，高于 GLM Legal）与 Wonder（第 8，低于 GLM Legal），GLM Legal 恰为 6/77。**推断**：旧快照取自 Wonder 两个模型入榜之前；未能取到旧快照本身 | `c-vals-hlab.txt` 排名与总数 |
| Harvey LAB 每题约 $0.15 | **成立**：“$0.15 per test”（Vals Updates）与榜单 “$0.15” | 截图 29、33 |
| “前十中成本最低” | **成立（Vals 原话）**，且我逐行核了前十成本；须带口径：只限前十，榜单第 14 名 MiMo V2.6 Flash $0.09、第 25 名 DeepSeek V4 Flash 0731 $0.06 更低；GLM Legal 耗时 1h12m 为前十最长 | 截图 29，C4f |
| “Weights Open”（Vals） | Vals 页确有此标注，但官方页、HF、ModelScope、GitHub 均无权重，HN 评论者也称不开放；见 C2 | 截图 27、28、36 |
| “Vals 是独立评测” | 官方页写 independently evaluated by Vals，仅限 Harvey LAB 与 LegalBench；PLawBench 无第三方佐证；Vals 页写 “Congrats to the zAI team on the release!” | 截图 26、33 |
| 官方页“动态正文无法直读” | 属实：curl 只得 683 字节空壳；已用 headless Chrome 取得全文 | `c-raw-z-ai-blog-glm-legal.html`、`c-zai-blog-glm-legal.txt` |
| 1009 速览“AA 的 Harvey 评测榜首仅 9.4%” | 与 AA Harvey LAB-AA v1.1 页一致：Grok 4.7 (Xhigh) 9.4% 现排第 3（榜首已是 Muse Wonder Full 13.1%）；该口径为含幻觉门控，与 Vals 的 25.42% 榜首不可互比 | `c-aa-harvey-lab-aa.txt` |

## 二、可能的吠点（仅作线索，进正文前须回一手来源）

1. “开放权重”只有 Vals 在说（Vals 模型页“open-weight”/“Weights Open”）；官方页只给申请制 Beta API，HF、ModelScope、GitHub 当前无 GLM-Legal，HN 评论者 postalcoder、consumer451 也说不是开放模型（链接见下节 HN）。
2. 官方对比图只放 5 个模型；Vals 同一榜单上有 6 个模型高于 GLM Legal（前十里 Muse Spark 系列、Wonder Full、Gemini 4 Argon），均不在图里；而被它“超过”的 Kimi K3、Claude Fable 5、GPT-6 Astra 在该榜分列 12、16、34 名。（Vals 榜单 `c-vals-hlab.txt`）
3. 官方写 LegalBench、PLawBench 上 “matches frontier”，但官方图中 LegalBench 86.2 低于 Claude Fable 5（88.6）与图中 GPT-6 Astra（87.1）；Vals LegalBench 榜单 GLM Legal 排第 14/150（榜首 88.56%）。该图里 GPT-6 Astra 的 87.1 在 Vals LegalBench 榜单查无此行。
4. PLawBench 的数字只在官方图里（77.3 对 Claude Fable 5 的 77.7），官方页未写评测者、设置；Vals 声明独立评测的范围不含它。
5. Legal Research Bench（美国法研究）上 GLM Legal 41.83%，低于自家基座 GLM 5.3 Flash 的 45.19%，也低于 GLM 5.3 的 49.04%；官方页未提；Vals Updates 只写了 “#24 of 78”。
6. 19.2% 是全部准则通过的任务比例，即约 81% 的任务没有“全过”；评分者是两个 LLM（GPT 5.5、Claude Sonnet 4.6），不是律师。官方措辞 “confirms reliable end-to-end task execution” 与此数字之间有落差。
7. 每题 $0.15 是 Vals 以 $0.075/$0.25 计价的结果，该价格官方价格页未公布（官方 GLM-5.3-Flash 标价 $0.15/$0.50）；每题耗时 1h12m，前十最长。
8. 官方页未说明法域；Harvey LAB 与 Legal Research Bench 偏美国法，PLawBench 是中国法；示例 3 引新加坡、马来西亚、印尼法条。HN 评论 bb123（https://news.ycombinator.com/item?id=50059244）与 arctic-true（https://news.ycombinator.com/item?id=50058852）都提到法域问题。
9. 训练数据未披露（官方：“expanded the volume of high-quality legal corpora”）；HN 评论 Trusteando（https://news.ycombinator.com/item?id=50063171）担心训练集与测试集相似（仅为猜测，无证据）。
10. Vals 同一基准的 Muse Spark 1.2 等条目标有“拒答”图标（拒答计为失败）；Vals 页还对 Claude Fable 5 注明回退到 Claude Opus 4.8 的 4 个任务，无回退时为 10.42%。官方对比所用 Claude Fable 5 的 11.25% 是含回退的分数（Vals 页文字）。
11. 同一 Harvey LAB，AA 版（含幻觉门控、另一套评委）里 GLM-5.3-Flash 仅 0.8%、GLM-5.3 (Max) 0.3%，对比 Vals 版 6.67%/8.33%，说明名次与分数对评测设置极敏感；AA 暂无 GLM-Legal。
12. 基准本身的争议线索：Harvey LAB 公开集仓库有关于任务 113（firm-knowledge）rubric 的 issue（https://github.com/harveyai/harvey-labs/issues/191，2026-10-09 开，state open，2 条评论）；HN 评论 mathis-l 称发现多个有问题的任务与 rubric。注意 Vals 用的是私有 held-out 集，与公开集不是同一批题，此线索不能直接套用。

### HN 高赞/有理有据的评论（L6；HN 不公开评论分数，故无得分；按顶层位置与回复数记）

- consumer451（回复 32）：https://news.ycombinator.com/item?id=50058954 ——“Wait, so this not an open weight model, correct? Only available via z.ai?” 并提出隐私数据由对方处理的顾虑。
- postalcoder（回复 13）：https://news.ycombinator.com/item?id=50058745 ——“Looks like this is not an open model.”
- anon373839：https://news.ycombinator.com/item?id=50064337 ——“If not, it’s completely DOA for legal use.”（指不发布权重）
- otterley（自称律师，回复若干）：https://news.ycombinator.com/item?id=50059088 ——“As an attorney, I am not particularly impressed about a 20% pass rate on a benchmark.” ctolsen 回复：https://news.ycombinator.com/item?id=50059323 ——“on this particular benchmark (Harvey's Legal Agent) 20% puts it almost top of the pack of any model … Opus 5.5 scores 4% on it.”（与 Vals：Claude Opus 5.5 3.75% 一致）
- mathis-l：https://news.ycombinator.com/item?id=50059184 ——称在 Harvey 基准 firm knowledge 子集里发现有问题的任务与 rubric，链接 harveyai/harvey-labs#191。alansaber：https://news.ycombinator.com/item?id=50059962 与 https://news.ycombinator.com/item?id=50060181 ——“Agents as judges”，“The benchmark is US law”。
- Natsu：https://news.ycombinator.com/item?id=50058697 ——“I didn't see anything addressing something that has frequently made the news: LLMs outright hallucinating cases or holdings.”
- toephu2：https://news.ycombinator.com/item?id=50058767 ——称在 Chrome 里打开 z.ai 的法律页出现证书错误 “net::ERR_CERT_COMMON_NAME_INVALID”（未复现，我在匿名 headless Chrome 里正常打开）。
- maxdo：https://news.ycombinator.com/item?id=50059966 ——“No fable 5.1 no opus 5.5, no gpt 6.1, benchmarks for the hype.”（官方图没放这些；Vals Harvey LAB 榜上 Claude Opus 5.5 为 3.75%，GPT-6.1 Sol 5.42%，Claude Fable 5.1 6.67%）
- HN 帖里没有自称智谱/Z.ai 的评论者（作者名无 zai/zhipu/glm）。

## 三、扫描中可能涉及、本组已核的其他事实

- Vals 同页：“RELEASE DATE OCT 12, 2026”；开发者 zAI（CN）；输入仅文本；“Reasoning Effort: max”（超参折叠项）。
- Vals 对 GLM Legal 的 Harvey LAB 准则通过率 92.5%，同一榜单 Muse Spark 1.3 Max 94.74%（榜单文字）。
- 官方页的三个“示例”（合同审查、证据分析、合规研究）都是官方挑选的同题对比，对象是基座 GLM-5.3-Flash；官方页自身未给样本量或评分。
- 官方图用名 “Claude Fable 5”“GPT-6 Astra” 与 Vals 命名一致；未标 effort/版本设置。

## 四、假设与未验证项

- 假设：Vals 的 “Cost / Test” 为其用所列价格 × 实际 token 计算（推断，Vals 页未逐句说明）。
- 120 题（C4d）为推断；AA 页原文写 120 private tasks。
- “6/77”的解释（C9）为推断。
- 中文官方页、OSChina 正文、byteiota 的绝对发布时间：未取到。
- Vals 是否收费或与智谱有商务关系：未找到任何来源，不作判断。
