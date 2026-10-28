# 1027 期 D 组（速览）取证清单

执行方：Claude Haiku 5.5（模型 ID claude-haiku-5-5）。取证窗口：2026-10-28 20:28–20:50 北京时间（本机 EDT 08:28–08:50）。
口径：官方页只有日期时写“官方未给时刻”；HN 时刻为北京时间（UTC+8），括号内已换算。截图：本组不截图。存档：`d-<编号>-*.html|.pdf|.txt|.headers`，HTML 与 PDF 只存本地，txt 入库。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别按 EDITORIAL.md 事实分级（L1 官方；L3 合作机构官方；L4 权威媒体；L6 社交转述）。

---

## D1 Anthropic 与六家合作方的早期药物研发案例

- 状态：部分支持（六家名单、10x 与 35,000 等数字均有原文出处，但口径需限定；“申报文书”归属与扫描不符）
- 级别：L1（Anthropic 官方研究页）；合作方报告为 L3（Generate、Divergence、BigHat 自家页面与报告）
- 一手来源：https://www.anthropic.com/research/early-progress-drug-development （存档：`d-d1-anthropic-drug.html|.txt`）
- 合作方报告：Generate 技术报告 https://generatebiomedicines.com/publications/refactored-intracellular-domains-enhance-car-expression-and-function/ （`d-d1-generate-report.pdf|.txt`）；Divergence 预印本 https://divergencelabs.com/publications/ige-preprint.pdf （`d-d1-divergence-preprint.pdf|.txt`）；BigHat 报告 https://www.bighatbio.com/bighat-anthropic-collaboration-report-10-27-2026 页面抓取被拦截（`d-d1-bighat-report.html` 为拦截页，见 capture-log），该页跳转到的 PDF 存为 `d-d1-bighat-pdf.pdf|.txt`。
- 发布时刻：官方页 “Published Oct 27, 2026”（官方未给时刻）；HN 2026-10-28 04:50（UTC 10-27 20:50）。
- 一句事实与限定词：
  - 六家：“deep research partnerships with six organizations developing new medicines: BigHat Biosciences, Bristol Myers Squibb, Divergence Labs, Enveda, Generate Biomedicines, and The Column Group.”（已找到）
  - Generate：“One design was about 70% shorter than the standard version, and in lab tests it was more effective: it allowed immune cells to make roughly 10 times as many receptors”。Generate 报告：“a 45-amino-acid ICD less than a third the length of the clinical 4-1BB–CD3ζ (BBζ) ICD”；“On day 1, krgr–ζ1–40 T cells displayed 12-fold more CAR than BBζ T cells”；另一测定为 “8.4-fold that of BBζ”。博文的 “roughly 10 times” 是两种测定之间的概括，报告里没有直接写 10 倍。（部分支持）
  - Divergence 数量：博文 “largely autonomously, generated roughly 35,000 antibody designs in about a month to be narrowed down and tested”；“three designs that could bind to IgE, and 20 that could dislodge it。Three of those 20 appeared to dislodge IgE more than the standard medication.” 预印本：“In total, 23,600 unique designs were scored in 1,310 H100-hours, and 57 were nominated”；“Twenty designs removed IgE faster than every negative control. We classified these 20 sequences as positive hits for disruptive activity.” 20 个命中一致；35,000 未在预印本中找到；“三个结合型”未在预印本中找到（未找到）。两处设计数口径不同（博文 “antibody designs” 与论文 “unique designs scored”），全部记录，未取舍。
  - BigHat 1,010 与 1,232：博文 “BigHat’s lab made and tested about 1,010 variants in about 10 weeks”。报告 “about 1,010 design variants (1,232 wells)”，“1,232 design wells in all”。同一批数据，一个是变体数、一个是孔数，不是冲突。
  - BigHat 倍数：博文 “the best versions bound more than 10x more tightly”。报告：VHH 先导 “binds albumin about nine times more tightly”，scFv 先导 “binds mesothelin about six times more tightly and precipitates 18 °C higher”；单个设计 “12 times tighter than the parent”。报告限定：“Lead values rest on one or two replicates, and the VHH’s aggregation value is one well”。
  - BigHat 聚集：博文 “the lead for longer-lasting drugs clumped more than 100x more slowly”。报告文本中未找到 “100 倍” 的聚集速率口径（未找到）。
  - BigHat 18 °C：博文 “the cancer-targeting lead stayed in solution 18°C higher”；报告原句 “binds mesothelin about six times more tightly and precipitates 18 °C higher”。“stayed in solution” 是改写，原句为 “precipitates 18 °C higher”（部分支持）。
  - Enveda：“Claude wrote the first drafts of two important regulatory documents.”（申报文书归 Enveda，不归 BMS）
  - BMS：“BMS is using Claude to assess plans of an early-stage drug in clinical trials, and to estimate the likelihood of those trials succeeding.”
  - 临床阶段：博文 “so far it is mostly preclinical—that is, for the most part, it isn’t yet at the stage where the drugs are being tested in humans.”；Mithril：“we expect a drug to be ready to start clinical trials in humans as soon as late 2028.”。Divergence 预印本的 “Excellergy … has now entered phase 1 clinical trials” 指的是基于前人 omalizumab 变体工作成立的另一家公司，不是 Divergence 的候选（未找到 Divergence 候选已进临床的原句）。
  - Anthropic 自研：“We’ve also begun drug discovery efforts of our own, including programs for rare childhood forms of muscular dystrophy”；“We’ve acquired Mithril Medicine, a one-person company”。
- 热度：HN 246 分 / 246 评，2026-10-28 04:50（北京）（扫描 244/245）。HN 有理有据的质疑（分数不可得，HN 不返回评论分；按回复数排序）：item 50267192（25 回复，自称药物研发人员）：“Coming up with candidate drugs is the easiest part of the process.”；item 50266216（15 回复）：生物学验证回路长（湿实验、临床、监管）。Reddit：查询失败（见 capture-log）。

## D2 Survival LLM（离线应急资料 AI 助手）

- 状态：部分支持（版本与日期、资料库体积、首次联网属实；“21/22”只是两个最高配置档；未找到官方“不能代替急救”类免责声明原句）
- 级别：L1（项目方官网与版本说明，开发者自述）
- 一手来源：https://www.brangus.io/survival-llm （存档：`d-d2-brangus-survival.html|.txt`）
- 发布时刻：版本说明逐条只写日期：0.3.4 与 0.3.3 为 “October 27, 2026”，0.3.0 为 “October 26, 2026”（官方未给时刻）；HN 2026-10-28 03:55（UTC 10-27 19:55）。
- 一句事实与限定词：
  - 版本：“0.3.4 October 27, 2026”；“0.3.3 October 27, 2026”（已找到，与扫描一致）。
  - 自测：“‘Right’ means it got the main point of official guidance right on 22 blackout and first-aid questions, graded blind, in our tests on an earlier build of the library”。结果表：Mac 128 GB 以上 Nemotron 3.5 Super 21/22；Mac 32–96 GB Qwen3.8 27B 21/22；Windows 32 GB 以上 Nemotron 3.5 Lightning 19/22；Mac/PC 16–31 GB IBM Granite 4.2 8B 14/22。页面原文：“Smaller models make more mistakes, so the app always shows the source to check.”（扫描只写 “21 题”，漏掉其余档位，属与说法不符的部分）
  - 首次联网与体积：“Set it up while you have internet; after that it needs none.”；“a 9 GB download for everything (19 GB once installed), or 1.3 GB (3 GB installed) without Wikipedia and iFixit.”；安装时间 “Setup takes 20 minutes to about 2½ hours”。
  - 免责声明：未找到 “不能代替急救”一类原句。最接近的是：“AI models make mistakes, the small ones more often. Every answer shows the articles it used, so you can check.”（未找到，已注明最接近原句）
  - 开源：“Free and open source.”；页面 “MIT license”。
- 热度：HN 392 分 / 148 评，2026-10-28 03:55（扫描 389/148，时点不同）。HN 质疑（分数不可得）：item 50265144（22 回复）：“why not just gather those resources into some sort of local archive that can be searched without an LLM? … something like Kiwix”；item 50265503（20 回复）：引用 “14 of 22 right” 一行并质疑 “emergency appendectomy”；item 50265640（8 回复）：“download the books, and slap a search engine on top of them”。

## D3 Axiom：CFSG 在 Lean 中的形式化

- 状态：部分支持（规模数字、sorry-free 与公理与原文一致；“完整证明”需限定为“按第一代证明、陈述取自 TauCeti、有 34 处偏差与 UGT 替代路线”）
- 级别：L1（Axiom 官网、官方 GitHub 仓库、官方博文）
- 一手来源：https://cfsg.axiommath.ai/ （`d-d3-axiom-cfsg-site.html|.txt`）；https://github.com/AxiomMath/CFSG （`d-d3-axiom-cfsg-gh.html|.txt`；仓库 API `d-d3-gh-api-repo.json`、`d-d3-gh-api-commits.json`）；博文 https://axiommath.ai/research/the-enormous-theorem-formalized/ （`d-d3-axiom-blog.html|.txt`，经 curl 与 `barking html2txt` 存档，未经 fetch 的 .headers）
- 发布时刻：博文 “October 26, 2026 by Axiom”（官方未给时刻）；仓库创建 2026-10-27T13:19:32Z（北京 21:19）；唯一提交 “Initial commit” 2026-10-27T15:07:38Z（北京 23:07）；HN 2026-10-28 00:29（UTC 10-27 16:29）。
- 一句事实与限定词：
  - README：“A sorry-free Lean4 (v4.35.0-rc3) proof of the Classification of Finite Simple Groups (CFSG), built on Mathlib and TauCeti, using the standard axioms Classical.choice, propext, and Quot.sound (no native_decide). We primarily follow the first generation proof of CFSG.”（已找到）
  - 陈述来源：“We use the statement of CFSG from the TauCeti project”（README）。
  - 规模：网站 “Lines of Lean 43.1M”、“Declarations 2.05M”；博文 “approximately 43.1 million lines of code in the Lean proof assistant”（已找到，与扫描一致）。
  - 偏差：博文 “there were 34 noteworthy deviations from the published record that needed to be resolved, the most substantial of which related to the Unbalanced Group Theorem.”
  - UGT 替代路线：博文 “The paper retains the longer argument because it follows the route AxiomProver formalized”；“After the formalization, mathematicians at Axiom found a substantial shortcut … recorded in Remark 9.5 of the paper”。博文原句：“the replacement of the UGT argument illustrates why a formalization may follow a different route to the same mathematical conclusion.”
  - 勘误表：网站 “We are working with group theorists to compile the errata we found in the literature, and will publish them here.”（尚未发布）
  - 构建：README “lake build builds the entire project. … to get this project’s build time under 24 hours.”（未运行构建，按派工单）
  - 过程：博文 “Over 24 days, engineers and mathematicians at Axiom Math directed swarms of dozens of AI agents”。
  - 人工审查范围：博文 “Humans inspect that statement and the definitions it uses to confirm that it expresses the intended classification; Lean checks the formal deductions establishing it.”
  - 时间线：仓库公开历史只有 1 个提交（“Initial commit”，GitHub API 确认）；博文日期 10/26 早于仓库创建 10/27，官方未解释。
  - 未找到：独立构建、外部审计（未找到）。
- 热度：HN 218 分 / 68 评，2026-10-28 00:29（北京）（扫描 218/68）。HN 有理有据的讨论：item 50262065（Axiom 员工自述 “a swarm of agents was tasked with going through all these sources”，5 回复）；item 50261783（14 回复，问能否把教科书全部转 Lean，属讨论而非质疑）。

## D4 Liquid AI：LFM3 开放权重多模态小模型

- 状态：部分支持（型号与许可证属实；扫描称 “2B 级”，其中 LFM3-1.7B 为 1.7B；扫描漏 omni 与 decision；“不推荐”原句带限定；许可证有年收入 1000 万美元门槛）
- 级别：L1（Liquid AI 官方 Hugging Face 博客与模型卡、LICENSE 文本）
- 一手来源：https://huggingface.co/blog/LiquidAI/lfm3 （`d-d4-liquid-lfm3.html|.txt`）；模型卡：LFM3-1.7B（`d-d4-lfm3-17b`）、LFM3-VL-2B（`d-d4-lfm3-vl-2b`）、LFM3-Audio-2B（`d-d4-lfm3-audio-2b`）、LFM3-Omni-2B（`d-d4-lfm3-omni-2b`）、d1-2B（`d-d4-d1-2b`）、d1-omni-2B（`d-d4-d1-omni-2b`）；许可证 `d-d4-lfm-license.txt`（均为 .html/.txt 与 .headers）
- 发布时刻：博文 “October 27, 2026”（官方未给时刻）；HN 2026-10-28 01:04（UTC 10-27 17:04），标题 “Introducing LFM3: Every Modality for Specialized Intelligence”，链接 https://www.liquid.ai/blog/lfm3。
- 一句事实与限定词：
  - 型号：“six 2B-class models (text, vision, audio, omni, and decision) trained from LFM3-1.7B-Base”；原文 “all in the 2B parameter range”。LFM3-1.7B 的参数为 1.7B（已找到，但扫描的“约 2B 级文字、视觉、音频”漏了 omni 与 decision）。
  - 许可证：各模型卡 “License: LFM Open License v1.0”。LICENSE 原文：“Threshold” 为 “annual revenue of 10 million United States dollars ($10,000,000) or more”；“Any Commercial Use of the Work or a Derivative Work by a Legal Entity that exceeds the Threshold is not licensed under this Agreement.”（扫描未写门槛，应补）
  - “不推荐”：LFM3-1.7B 卡 “It is not recommended for agentic coding or complex multi-step reasoning due to its small size.”；LFM3-Audio-2B 卡同句；LFM3-VL-2B 卡 “It is not recommended for coding, multi-turn conversations, or complex reasoning.”
  - 同时存在的正向表述：博文 “It is post-trained to run in agentic harnesses such as Hermes Agent, Pi, and OpenClaw.”（写 “不推荐” 时须并列）
  - 1.7B 对比：博文 “outperform LFM2.5-2.6B while using 35% fewer parameters”。
- 热度：HN 147 分 / 57 评，2026-10-28 01:04（北京）（扫描 145/55，数字已变）。HN 讨论有具体用例与异常样例：item 50265767（4 回复）：LFM3-1.7B 答 “Oink”，VL-2B 答 “moo”，作者推测为量化问题（未证实）。

## D5 Replit Cloud Agents：非工程同事提交代码改动

- 状态：部分支持（“Pro & Enterprise”属实；“由工程团队合并”原文无 “merge” 字样，原文为 “engineering still decides what ships” 与 “ready to approve or provide feedback”；“待审 PR”属实）
- 级别：L1（Replit 官方博客）
- 一手来源：https://replit.com/blog/anyone-can-bring-ideas-to-company-codebase （`d-d5-replit-cloud.html|.txt`）
- 发布时刻：“Oct 27, 2026”（官方未给时刻）。
- 一句事实与限定词：
  - 开放范围：“it’s now available for all Pro & Enterprise customers.”（博文语境为 Self-Driving Company 与 Cloud Agents）
  - PR：“When it looks right, the agent opens a pull request (PR) in GitHub or a merge request (MR) in GitLab”；“Engineers get pull requests they don’t have to run themselves to understand, arriving in GitHub or GitLab already tested … ready to approve or provide feedback.”
  - 决定权：“With Replit’s Cloud Agents, engineering still decides what ships”；标题 “Replit Cloud Agents build and test high quality changes, with engineers in control of what ships.”
  - 限定：“Review feedback becomes skills … This feature is in early access.”（指的是自我改进的工作区记忆，不是整个功能）
- 热度：HN 无命中（检索 “Replit Cloud Agents”、“Replit”，仅有 2026-10-28 03:15 的 “The Rise of the Self-Driving Company” 1 分，属相关但不同帖）。Reddit：查询失败（见 capture-log）。

## D6 Ai2 新颖性判断：提示改写致准确率差 50 个百分点

- 状态：部分支持（数字与论文一致，但限定：单一判断模型 GPT-5.4、Human+Generated 配对、弱标签；扫描 “改写 prompt” 与论文摘要的 “告知判断者评审结论” 不完全一致）
- 级别：L1（Ai2 官方博文）；论文为 arXiv 预印本（同组作者，v3）
- 一手来源：博文 https://allenai.org/blog/novelty-bench （`d-d6-ai2-novelty.html|.txt`；标题 “Can we trust LLMs to judge whether a research idea is novel?”）；论文 https://arxiv.org/abs/2610.02022 （`d-d6-arxiv-abs.html|.txt`；标题 “Old Ideas, Novel Problems: The Instability of LLM-Based Novelty Evaluation”）
- 发布时刻：博文 “October 27, 2026”（官方未给时刻）；arXiv v1 “Submitted on 1 Oct 2026”，v3 “last revised 26 Oct 2026”（论文先于博文发布）。
- 一句事实与限定词：
  - 博文：“In one experiment, changing just a few words in a prompt shifts a model’s accuracy in identifying which of two ideas is more novel by more than 50 percentage points.”
  - 具体实验：“we showed GPT-5.4 the same Human+Generated pairs … When we switched to the simpler prompt, accuracy dropped from 93.5% to 40.9% – below chance – and the model changed its verdict on more than half of the same pairs.”（93.5 − 40.9 = 52.6 个百分点，由文中两个数字相减）
  - 论文摘要：“simply telling the judge that reviewers found one idea novel and the other not can change its verdict on more than half of the identical idea pairs it is shown, shifting pairwise accuracy by over 50 points … The same change helps one judge and hurts another.”
  - 弱标签：“these are weak labels, not verified judgments for every idea.”（Human+Generated 中生成想法的标签）
  - 对照：“the same prompt change shifted accuracy by only about seven percentage points on the Human-Only dataset”
  - 资源链接：博文列出 “Paper”、“Data https://huggingface.co/datasets/allenai/novelty-judge-bench”、“Code https://github.com/allenai/novelty-eval”。这些链接在博文中出现，本组未逐一打开核验（待核）。
- 热度：HN 无命中（检索 “novelty judges”、“Old Ideas, Novel Problems”、“Ai2 novelty”、“Can we trust LLMs …”，无 10/27 当期帖）；Reddit 查询失败。

## D7 OpenAI 与 Oracle：Oracle 企业应用接入 ChatGPT

- 状态：部分支持（两个用户数属实，但主语是 Oracle 自家员工，扫描未写；“more than” 被漏；集成为 “is developing”、“designed to”，美国客户已可通过 Marketplace 购买）
- 级别：L1（OpenAI 官方页）；用户数引自 OpenAI 2026-10-08 的 Oracle 案例页（同为 L1）
- 一手来源：https://openai.com/index/oracle-apps-in-chatgpt/ （`d-d7-openai-oracle.html|.txt`）；案例页 https://openai.com/index/oracle/ （`d-d7-openai-oracle-2024ref.html|.txt`；文件名中的 “2024ref” 有误，实际为 2026-10-08 页面，保留不改名）
- 发布时刻：10/27 页 “October 27, 2026”；案例页 “October 8, 2026”（均官方未给时刻）；HN 2026-10-28 00:09（1 分 / 0 评）。
- 一句事实与限定词：
  - “Oracle reported [link] 130,000 active ChatGPT users and more than 95,000 active Codex users.”
  - 案例页原句：“130K Active ChatGPT users at Oracle”、“95K+ Active Codex users at Oracle”；“Over a hundred thousand employees across Oracle are using ChatGPT Work and Codex”。即用户是 Oracle 内部员工。
  - 计划中：“Oracle is developing Oracle AI Workspace to bring that vision into ChatGPT through integrations with Oracle applications and data.”；“The intended experience would bring information from finance, procurement, and planning applications together …”
  - 已可购买：“Oracle customers in the U.S. can purchase access to the OpenAI API, Codex, and ChatGPT Work through Oracle Marketplace”
  - 设计中：“Oracle App Composer is designed to help teams build apps with Codex”；“Oracle AI Security Mesh is designed to help customers find and fix vulnerabilities”
- 热度：HN 1 分 / 0 评，2026-10-28 00:09（北京）。

## D8（备选）Apple M6 MacBook Pro 14 英寸与 M6 iMac

- 状态：部分支持（AI 提速原句与脚注有出处；价格属实；“2 纳米”在两份新闻稿中均未找到；“每个 GPU 核心一颗神经加速器”在新闻稿中找到，扫描称 “不在新闻稿里” 有误）
- 级别：L1（Apple Newsroom）
- 一手来源：MBP https://www.apple.com/newsroom/2026/10/apple-introduces-the-new-14-inch-macbook-pro-with-m6/ （`d-d8-apple-mbp-m6.html|.txt`）；iMac https://www.apple.com/newsroom/2026/10/apple-unveils-new-imac-with-m6/ （`d-d8-apple-imac-m6.html|.txt`）
- 发布时刻：两篇均 “October 27, 2026”（官方未给时刻）。HN 时刻见下。
- 一句事实与限定词：
  - MBP：“The 12-core GPU delivers up to 30 percent faster graphics and up to 25 percent faster AI workloads when compared to M5”。其中 25% 的原句带测试名：“up to 25 percent faster performance for on-device AI workloads, like LLM prompt processing, compared to M5”。比较对象脚注为：“Results are compared to 14-inch MacBook Pro with M5, 10-core CPU, 10-core GPU, 32GB of unified memory, and 4TB SSD.”。测试脚注：“Testing was conducted by Apple in October 2026 using shipping competitive systems and select industry-standard benchmarks.”（脚注编号与正文的对应按文本顺序推断，未在页面上逐条核对；推断的依据是正文中 GPU 句后紧跟的标号为 2）
  - iMac：“the new iMac delivers up to 5x faster on-device AI performance than M4”；LM Studio 测试句 “Up to 12x faster on-device AI performance for prompt processing in LM Studio, compared to iMac with M1, and up to 5x faster performance than M4”。比较对象脚注：“Results are compared to previous-generation iMac systems with Apple M4, 10-core CPU, 10-core GPU, 32GB of unified memory, and 2TB SSD.”；测试脚注：“Testing was conducted by Apple in October 2026 using shipping competitive systems and select industry-standard benchmarks.”（对应关系同上，推断）
  - 神经加速器：MBP “a 12-core GPU with Neural Accelerators in each core”；iMac “Neural Accelerators in each core”
  - 价格：“The 14-inch MacBook Pro with M6 starts at $1,999 (U.S.)”；iMac 10 核 CPU/GPU 起价 “$1,499”（美元，非扫描内容）
  - 发售：MBP 与 iMac 均为 “Customers can pre-order … starting tomorrow, October 28, with availability beginning Friday, November 6.”（正文为美国时间表述；北京时间日期未换算，官方未给时刻）
  - 工艺：两份新闻稿中未找到 “2 纳米”或 “nanometer” 字样（未找到）
- 热度：MBP HN 378 分 / 521 评，2026-10-28 00:08（北京，apple.com/in 链接）；MBP 另有 165 分 / 201 评，2026-10-27 21:01（北京，newsroom 链接）。iMac HN 347 分 / 436 评，2026-10-27 21:02（北京，apple.com/imac 链接）（扫描 377/519、346/436，时点不同）。HN 热门评论（item 50261352）主要讨论换机周期与命名，未针对 AI 提速说法。

---

## 扫描说法勘误（汇总）

1. D1：扫描称 “BMS 等用于申报文书”。原文：申报文书草稿是 Enveda 的（“Claude wrote the first drafts of two important regulatory documents”）；BMS 是临床试验计划与成功率评估。
2. D1：扫描称 “1010 与 1232 两个数字冲突”。原文：BigHat 报告为 “about 1,010 design variants (1,232 wells)”，单位不同，不是冲突。
3. D1：扫描称 “35,000 个抗体设计”。该数字见博文原句；预印本为 23,600 unique designs scored / 57 nominated，口径不同，两者都要记录。
4. D1：扫描称 “受体表达约十倍”。博文 “roughly 10 times”；Generate 报告 day 1 为 “12-fold”，另一测定 “8.4-fold”。
5. D1：扫描称 “Divergence 约35,000个抗体设计（候选，非获批药物）”。“候选、非获批”属实；但预印本称 “Excellergy … has now entered phase 1 clinical trials” 指另一公司，不是 Divergence 候选。
6. D2：扫描称 “22 题自测答对 21 题”。仅 Mac 128 GB 以上与 Mac 32–96 GB 两档为 21/22；Windows 32 GB 以上 19/22；16–31 GB 档 14/22。测试对象为 “an earlier build of the library”。
7. D2：扫描称 “开发者……”未写免责声明，原页未找到 “不能代替急救” 原句（已注明）。
8. D3：规模与 sorry-free 与原文一致。需补：陈述取自 TauCeti；博文称 34 处偏差；UGT 有替代路线；构建耗时 “under 24 hours”。
9. D4：扫描称 “六款约 2B 级文字、视觉、音频模型”。原文为 text、vision、audio、omni、decision 六款，LFM3-1.7B 为 1.7B。扫描未写许可证门槛（年收入 1000 万美元）。“不推荐”原句为 “due to its small size”，带限定。
10. D4：HN 数据 “145 分 / 55 评”。现查为 147 / 57。
11. D5：扫描称 “待审 PR、由工程团队合并”。原文无 “merge”；为 “engineering still decides what ships” 与 “ready to approve”。
12. D6：扫描称 “改写 prompt 令模型判定准确率差异超 50 个百分点”。博文原句为 “changing just a few words in a prompt”，论文摘要为 “simply telling the judge that reviewers found one idea novel and the other not”，实质是加入评审结论。实验仅 GPT-5.4、Human+Generated、弱标签，Human-Only 仅约 7 个百分点。
13. D7：扫描称 “约13万活跃 ChatGPT 用户、9.5万活跃 Codex 用户（谁的用户、在哪里）”。原文：“Oracle reported … 130,000 … more than 95,000”，案例页说明是 Oracle 内部员工；扫描漏 “more than”。
14. D7：扫描称 “部分集成为计划中”。原文：Oracle AI Workspace “is developing”；美国 Oracle 客户已可经 Marketplace 购买 API、Codex、ChatGPT Work。
15. D8：扫描称 “‘2 纳米’不在新闻稿里”。两份新闻稿均未找到 “2 纳米”，结论一致。扫描称 “‘每个 GPU 核心一颗神经加速器’不在新闻稿里”。原文 “Neural Accelerators in each core” 存在，结论有误。
16. D8：HN 数字 377/519 与 346/436 为近似值，现查为 378/521（MBP）与 347/436（iMac）。

## 各条发布时刻（北京时间）

| 条目 | 官方日期 | 北京时间（原时区） |
|---|---|---|
| D1 Anthropic | Oct 27, 2026（未给时刻） | 官方未给时刻 |
| D2 Survival LLM 0.3.4/0.3.3 | October 27, 2026（未给时刻） | 官方未给时刻 |
| D3 Axiom 博文 | October 26, 2026（未给时刻） | 官方未给时刻；仓库创建 10-27 21:19，首提交 10-27 23:07（UTC 13:19 与 15:07） |
| D4 LFM3 | October 27, 2026（未给时刻） | 官方未给时刻 |
| D5 Replit | Oct 27, 2026（未给时刻） | 官方未给时刻 |
| D6 Ai2 博文 | October 27, 2026（未给时刻） | 官方未给时刻；arXiv v1 2026-10-01，v3 2026-10-26 |
| D7 OpenAI | October 27, 2026（未给时刻） | 官方未给时刻 |
| D8 Apple | October 27, 2026（未给时刻） | 官方未给时刻 |

## 不取证项

按派工单未取证：GSA、Roblox、Wan、Discord、IEEE、Stanford 盗版、ACLU、Cartesia、OpenAI DNS、Code2Video-AA，以及额度重置类条目。
