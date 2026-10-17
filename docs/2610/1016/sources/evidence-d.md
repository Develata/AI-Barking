# 1016 期 D 组（速览：产品与硬件）取证清单

执行方：claude-haiku-5-5（D 组，派工单 `.handoff/2026-10-16-1016-evidence.md`）。
取证时刻：2026-10-17，北京时间 20:49–20:53（抓取见 `capture-log-d.md`）。
范围：D1–D4。不截图（派工单 D 组规定）。存档均在 `docs/2610/1016/sources/`，前缀 `d-`。
时区约定：官方页只给日期的写“官方未给时刻”；Verge 与 Surface 评测原为 UTC，已换算北京时间（UTC+8）；HN 时刻为 `barking heat` 输出的北京时间。

| # | 条目 | 状态汇总 |
|---|---|---|
| D1 | RTX Spark 笔记本实测 | 已找到（实测数字与原句齐全，来源为 The Verge 记者个人测试，L4；NVIDIA 上市稿 L1 核起售价与“up to”限定；Surface 128GB 价格两处口径略不同） |
| D2 | Claude Code 团队代码审查流程 | 已找到（官方博客 L1；“约 90%”为自述；回滚口径是相对下降，不是百分点） |
| D3 | Ultramarine 1 Flash | 部分支持（价格与对比数字原文齐全；“open model”的权重“coming soon”未发布；对比基准为厂商自家内部基准） |
| D4 | Waymo 多伦多 | 部分支持（2028 年是“目标”；“人工驾驶”是“将于数日内开始”，不是已在进行） |

HN 热度汇总见各条与 `d-heat.json`。Reddit 三次查询均被登录页重定向，记为失败（见 `capture-log-d.md`）。

---

## D1 RTX Spark 笔记本实测

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| Hermes（Qwen3.8-27B）本地生成新闻简报 10 分钟、耗 10% 电量；云端 Claude（Opus 5.5）用时一半、结果更好 | L4（据 The Verge 报道；记者个人测试） | https://www.theverge.com/tech/1012748/nvidia-rtx-spark-windows-arm-laptops-asus-hp-lenovo-impressions （存档 `d-rtx-verge.html`、`d-rtx-verge.txt`） | “It took 10 minutes, and 10 percent of my entire 99.9 watt-hour battery, to finally produce a list of stories that took the cloud-based Claude (Opus 5.5) half as long”；“Claude also produced better results, with more detail and better sourcing.” | 发布 Oct 16, 2026, 3:00 PM UTC = 北京 **2026-10-16 23:00**。Sean Hollister 的单次个人测试（样本小）；原文还写“The laptop stayed hot and a little sluggish as long as I kept Hermes open.” | 已找到 |
| 重装 Hermes 后更慢、耗电更多 | L4 | 同上 | “I tried a clean reinstall of Hermes to make sure it wasn’t a misconfiguration, but this time, the prompt took even longer and used 60 percent of my battery.” | 同上 | 已找到 |
| Lenovo 36 分钟耗 30% 电量，处理 5 张图且裁错 | L4 | 同上 | “I burned through 30 percent of the Lenovo Yoga 9n’s battery in 36 minutes to get a Qwen 3.8 27B model to resize and add a white background to five product images in PNG format — and it cropped the images wrong and cut off the products.”；原文续写：“I got it to do it right after a couple follow-up prompts” | 同上。注意原文另写“经追加提示后做对了”，只写“裁错”会漏掉限定 | 已找到 |
| 起售价 $2,599（18 核 / 5,120 CUDA / 24GB 的 Surface Laptop Ultra） | L4（Verge） | Surface 评测 https://www.theverge.com/tech/1012628/microsoft-surface-laptop-ultra-review （存档 `d-surface-verge.html`、`.txt`，句子见 `d-surface-verge.txt`） | “The base version is $2,599, and it comes with an 18-core CPU, 5,120-CUDA -core GPU, and 24GB of unified memory.” | Surface 评测 Oct 16, 2026, 12:00 PM UTC = 北京 **2026-10-16 20:00**。此价为 Verge 所列最低配置，不是所有机型统一价。同一 RTX Spark 实测文（`d-rtx-verge.txt`）另写：“There’s an 18-core / 5,120 CUDA N1X chip in lower cost configs, but that only gets as cheap as $2,599.99 for a Surface Laptop Ultra … or a smaller Asus ProArt P14 … with 24GB of RAM.”（勘误：同一实测文给 Surface 最低配价为 $2,599.99，Surface 评测与 NVIDIA 页为 $2,599，三处数字不一致；ProArt P14 在该句中与 Surface 并列，原文未单独给它的价格） | 已找到 |
| NVIDIA 称 RTX Spark 笔记本“starting at $2,599” | L1（NVIDIA 新闻稿） | https://nvidianews.nvidia.com/news/nvidia-rtx-spark-pcs-available-today-a-new-beginning-for-windows-pcs （存档 `d-rtx-nvidia.html`、`.txt`） | “RTX Spark laptops are available globally today from ASUS, Dell, HP, Lenovo, Microsoft and MSI through their websites and major retailers, starting at $2,599, with a limited-time promotion for new Perplexity users”；“Pricing and configurations vary by region.” | 页面只标 “October 16, 2026”，**官方未给时刻**；北京日期 10-16。含 Perplexity 限时促销，正文若写价格需注明 | 已找到 |
| 128GB 机型价格：Surface Laptop Ultra 128GB 顶配 | L4（Verge 评测正文与规格表） | Surface 评测（同上） | “It’s the top-of-the-line model at $5,899, and I think…”；规格表：“Price: $5,900” | 两处口径略有差异（$5,899 与 $5,900），正文取值时需注明出处。RTX Spark 实测文价格表 Surface 列为 $5,899.99（按列序对应，推断） | 已找到（口径需注明） |
| 128GB 机型价格：Asus ProArt P16 128GB 为 $6,999.99 | L4（Verge） | 同 D1 首条 | “ranging from the HP OmniBook at $3,199.99 to the Asus ProArt’s $6,999.99” | 同 D1 首条；价格表中 Asus “Price as tested $6,999.99” | 已找到 |
| NVIDIA：“up to 1 petaflop of FP4 AI performance”、“up to 128GB of unified memory” | L1 | 同 NVIDIA 新闻稿 | “RTX Spark delivers up to 128GB of unified memory and 1 petaflop of FP4 AI performance to run large language models and AI agents locally.”；另一段：“up to 1 petaflop of FP4 AI performance” | **限定词 “up to”**，是上限；文中是 FP4 口径。速览若写“1 PFLOP”，须保留 “up to” 与 “FP4” | 已找到（限定词必留） |
| NVIDIA 称已在六家厂商的笔记本上架 | L1 | 同 NVIDIA 新闻稿 | “NVIDIA RTX Spark laptops are available today from ASUS, Dell, HP, Lenovo, Microsoft and MSI — with additional systems to follow” | 页面 “October 16, 2026”，官方未给时刻 | 已找到 |

**一句事实（速览用）**：The Verge 记者称，在 RTX Spark 笔记本上用 Hermes（Qwen3.8-27B）本地生成新闻简报耗时 10 分钟、耗电 10%，云端 Claude（Opus 5.5）用时约一半、结果更好（个人测试）。
**能改变含义的限定**：单次个人测试，记者称“笔记本一直发热、卡顿”；重装后更慢耗电更多；NVIDIA 的 1 PFLOP 是 “up to” 上限，FP4 口径。

**热度**：
- HN 按 “RTX Spark” 搜索共 5 条，无 The Verge 文章条目（未找到 HN 主帖）。
- 相关：NVIDIA 上市稿 HN 50118565，4 分 / 0 评，北京 10-16 22:06。
- 相关：Surface Laptop Ultra（ServeTheHome）HN 50038461，6 分 / 5 评，北京 10-11 08:20。
- Reddit “RTX Spark laptop”：失败（登录页重定向）。

---

## D2 Claude Code 团队的代码审查流程

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 约 90% Claude Code 代码由 Claude 写 | L1（官方博客，署名 Addy Osmani、Morgan Westlee Lunt、Qing Wang；见页面 AUTHOR 栏） | https://claude.dev/blog/solving-code-review-at-anthropic/ （存档 `d-cr-claudedev.html`、`.txt`） | “We've said before that Claude writes about 90 percent of Claude Code's own code.” | 页面 PUBLISHED Oct 15, 2026，**官方未给时刻**；北京日期 10-15 或 10-16 前后均可能，按日期记 10-15。“about 90 percent” 为公司自述，非审计数据；“We've said before” 表明是沿用旧说法 | 已找到（自述） |
| 6–9 月合并 PR 翻倍以上、回滚占比降 40% 以上 | L1 | 同上 | “From June to September, merged pull requests to Claude Code's codebase more than doubled while the share that had to be reverted fell by more than 40 percent.” | **“fell by more than 40 percent” 是相对下降（回滚占比的相对降幅），不是 40 个百分点**；期间为 6–9 月；未给绝对回滚率。速览若写“降 40%”，须写“相对降幅超 40%” | 已找到（口径需限定） |
| 低风险改动无需额外人工批准 | L1 | 同上（“5. Human approval, scaled by risk” 节） | “Low … None required once automated review and security checks are green”；“Only trusted authors get the lighter paths.” | **限定：仅 Claude Code 团队及在可信列表上的作者；中风险 “One person”，高风险 “Two people”**；且须自动审查与安全检查全部通过。文中：“Automated review can clear a low-risk change, but it can't fill a slot that calls for a human.” | 已找到（限定词必留） |
| agent 批准不算人工批准 | L1 | 同上 | “An agent's approval never counts as a person's, either.” | 同上 | 已找到 |
| 一个 PR 夜里 50 多轮自动审查 | L1 | 同上 | “One CLI PR went through more than fifty rounds of review and fixes overnight before its author stopped it.” | 单个 CLI PR 的案例；“before its author stopped it” 为作者中止 | 已找到 |
| 指令者对 agent 改动负责 | L1 | 同上 | “Whoever prompted the change is on the hook for it.”；“On an agent-written PR, the person directing the agent counts as one of the required reviewers” | 与“agent 批准不算”同节，速览若写“人工审查”需注意该口径 | 已找到 |

**一句事实（速览用）**：Claude Code 团队博文称，Claude 写了约 90% 的 Claude Code 代码；6–9 月合并 PR 翻倍以上，回滚比例相对下降超 40%（博文自述口径）。
**能改变含义的限定**：“约 90%” 为自述，“降 40%” 为相对值，人工批准门槛按风险分级且仅限可信作者。

**热度**：
- HN 50122951，**382 分 / 352 评**，北京 10-17 03:04，标题 “How the Claude Code team does code review”。
- 同 URL 另一条 HN 50116568，2 分 / 2 评，北京 10-16 18:20（早先提交，未展开）。
- 扫描称 “373/343”，取证时为 382/352，见勘误。

---

## D3 Ultramarine 1 Flash（Paper Instruments）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 官网标题 “An open model for professional work”，但权重未发布 | L1（厂商官网） | https://www.paperinstruments.com/ultramarine （存档 `d-ultramarine.html`、`.txt`） | “An open model for professional work”；“Model weights coming soon”（页面导航与按钮均有该文字） | 落地页**未给日期**；页面抓取 HTTP Date 2026-10-17 12:49 GMT。权重“coming soon”，截至取证未发布 | 部分支持（“open model” 标题与权重状态并存） |
| 基座为 GLM-5.3 Flash，经 RL 训练 | L1（技术报告） | https://www.paperinstruments.com/blog/ultramarine （存档 `d-ultramarine-blog.html`、`.txt`） | “Ultramarine applies large-scale reinforcement learning to a strong open base model, GLM-5.3 Flash, and improves on it across all six knowledge-work benchmarks we evaluate, by 4.1 points on average, while generating 6.5% fewer tokens.” | 技术报告页标注 “Oct 14, 2026, Paper Instruments”，**官方未给时刻**；北京日期 10-14 或 10-15 前后，按 10-14 记 | 已找到 |
| 每任务 $0.13，对 Claude Fable 5 (max) $8.19、GPT-5.6 Sol (max) $2.09 “outscores” | L1（厂商自家内部基准） | 落地页与技术报告（同上） | 落地页：“At $0.13 per task, it outscores Claude Fable 5 (max) and GPT-5.6 Sol (max), at $8.19 and $2.09 per task”；报告：“It outscores Claude Fable 5 ($8.19) and GPT-5.6 Sol ($2.09) and comes within three points of Claude Opus 5.5 ($6.96) and GPT-6 Sol ($1.26), all four set to max reasoning.” | **厂商自家内部基准，不是独立第三方评测**（正文需点明）。报告数字：Ultramarine 59.3 对 Fable 5 57.8、GPT-5.6 Sol 55.1，Opus 5.5 (max) 落后 2.6 分，GPT-6 Sol (max) 落后 2.0 分。报告原文：“Scores are means over our internal task suite.” | 已找到（口径需点明） |
| 对比基准的定义 | L1 | 落地页 | “We measure this on PI Knowledge Work, a carefully built internal benchmark whose scores correlate strongly with the Artificial Analysis Intelligence Index (r = 0.81).” | 相关系数 r = 0.81 为厂商自报，报告称 24 个 AA 也测过的运行中相关 | 已找到 |
| API 价格：输入 $0.15、缓存输入 $0.03、输出 $0.50（每百万 token） | L1 | 落地页 | “Per million tokens on the Ultramarine API, the same as GLM-5.3 Flash.” | 与 GLM-5.3 Flash 同价（页面原句） | 已找到 |
| 标题 “Fable-class”、“60x cheaper than Fable” | L1 | 落地页 | “Ultramarine 1 Flash is Fable-class on knowledge work spanning finance, law, consulting, and operations. It is also 60x cheaper than Fable.” | 厂商自述，限定在 “knowledge work”（金融、法律、咨询、运营） | 已找到（厂商自述） |

**一句事实（速览用）**：Paper Instruments 称 Ultramarine 1 Flash 为“开放模型”（基于 GLM-5.3 Flash），在其自家内部基准上每任务 $0.13、超过 Claude Fable 5 (max) 的 $8.19；模型权重“coming soon”，未发布。
**能改变含义的限定**：权重未发布；对比为厂商自家内部基准（PI Knowledge Work），非独立评测；比较对象均为 max reasoning。

**热度**：
- HN 50106747，**279 分 / 90 评**，北京 10-16 01:06，标题 “Ultramarine 1 Flash”。
- 同站其他提交：50092655（24 分 / 1 评，10-15 02:32，“Frontier Model 100X Cheaper”）；50093128（3 分 / 0 评，10-15 03:01，技术报告页）。
- Reddit “Ultramarine 1 Flash”：失败（登录页重定向）。
- 扫描称 “273/90”，取证时为 279/90，见勘误。

---

## D4 Waymo 多伦多

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 2028 年载客（“计划”） | L1（Waymo 官方博客） | https://waymo.com/blog/2026/10/waymo-in-toronto/ （存档 `d-waymo-toronto.html`、`.txt`） | “Waymo is officially heading to Canada! We’re laying the groundwork for our fully autonomous ride-hailing service in Toronto, investing in local facilities and jobs, with the goal of welcoming our first riders in 2028.” | 页面 “October 16, 2026”，**官方未给时刻**；北京日期 10-16。原文为 **“with the goal of”（目标）**，不是承诺或确定计划；速览须写“目标 2028 年” | 部分支持（年份有，措辞需改为“目标”） |
| 当前阶段：人工驾驶测绘 | L1 | 同上 | “In the coming days, you’ll start to see Waymo vehicles on the streets of Toronto as we begin driving manually.”；“We are taking a rigorous, phased, safety-first approach starting with high-definition mapping of local street networks and testing with trained autonomous specialists behind the wheel.” | **是“将于数日内开始”，不是已在进行**；测绘与“受过训练的自动驾驶专员在驾驶座”的测试 | 部分支持（阶段描述需用将来时） |
| 合作方与招聘 | L1 | 同上 | “we’ll invest in hundreds of roles over time spanning fleet maintenance, validation driving, customer service, and other operations functions”；合作方 Element Fleet Management | “hundreds of roles over time”为长期规划；与 Element 的合作为原文表述 | 已找到 |

**一句事实（速览用）**：Waymo 官方称将在多伦多“以 2028 年迎接首批乘客为目标”，并在数日内开始人工驾驶测绘，测试有受训专员在驾驶座。
**能改变含义的限定**：“goal”；“in the coming days… begin driving manually”；“trained autonomous specialists behind the wheel”。

**热度**：
- HN 50118981，**315 分 / 416 评**，北京 10-16 22:44，标题 “Waymo in Toronto”。
- Reddit “Waymo Toronto”：失败（登录页重定向）。

---

## 扫描说法勘误（ChatGPT / WorkBuddy 扫描与派工单“已知线索”对照）

| 条目 | 扫描或线索说法 | 一手原文 | 结论 |
|---|---|---|---|
| D1 | 云端 Claude（Opus 5.5）用时一半、结果更好 | “took the cloud-based Claude (Opus 5.5) half as long”；“Claude also produced better results” | 与原文一致（单次个人测试） |
| D1 | 重装后 60% 电量 | “used 60 percent of my battery” | 一致 |
| D1 | Lenovo 36 分钟 30% 电量处理 5 张图且裁错 | 原文一致，但另写“经几次追加提示后做对了” | 一致，需补上“追加提示后做对”的限定 |
| D1 | 128GB 机型价格 | 正文 “$5,899”；规格表 “Price: $5,900”；RTX Spark 实测文表格 Surface 列 “$5,899.99”（按列序推断） | 三处口径略不同，取值时注明 |
| D1 | 起售价 $2,599 | RTX 实测文：“only gets as cheap as $2,599.99 for a Surface Laptop Ultra”；Surface 评测：“$2,599”；NVIDIA：“starting at $2,599” | 三处数字不一致（$2,599.99 与 $2,599）；速览若写起售价，以 NVIDIA 的 “starting at $2,599” 为准并注明“起售”口径 |
| D1 | “1 PFLOP FP4 的限定” | NVIDIA：“up to 1 petaflop of FP4 AI performance” | 漏了“up to”（上限）时属于夸大；必须保留 |
| D2 | 扫描称 10/15 | PUBLISHED Oct 15, 2026（无时刻） | 一致 |
| D2 | 6–9 月合并 PR 翻倍以上、回滚占比降 40% 以上 | “fell by more than 40 percent”（相对降幅） | 文字可以，但不能写成“降 40 个百分点”或“回滚率降到 40%” |
| D2 | “低风险可免额外人工批准” | 原文限定：仅可信作者；须自动审查与安全检查全绿；agent 批准不算 | 扫描说法缺限定，需补 |
| D2 | HN 373/343 | 取证时 382/352（北京 10-17 03:04） | 热度随时间变化，以取证时数字为准 |
| D3 | HN 273/90 | 取证时 279/90（北京 10-16 01:06） | 同上 |
| D3 | “open model”与 $0.13 对 $8.19 | 标题写 “An open model”，但页面写 “Model weights coming soon”；价格与对比数字一致 | 数字一致；“开放”需同时写“权重未发布” |
| D4 | “计划 2028 年载客” | “with the goal of welcoming our first riders in 2028” | **措辞需改为“目标”** |
| D4 | “当前人工驾驶测绘” | “In the coming days… as we begin driving manually” | **措辞需改为“将于数日内开始”** |

---

## 本组未找到的一手材料

- The Verge RTX Spark 实测的 HN 主帖：HN 搜索 “RTX Spark” 共 5 条，无该文（记为未找到，不影响 L4 事实）。
- Reddit 三次查询（“RTX Spark laptop”、“Ultramarine 1 Flash”、“Waymo Toronto”）均被 old.reddit.com 登录页重定向，未取得数据。
- Ultramarine 落地页未给发布日期；日期取自技术报告页（Oct 14, 2026）。
