# 1107 期 C 组（速览）取证结果

取证执行方：Claude Haiku 5.5（claude-haiku-5-5）。北京时间 = UTC+8；美东时间按页面所示（EST，UTC-5）换算；PST 按 UTC-8 换算。存档路径均在 `docs/2611/1107/sources/` 下。抓取、截图与热度的逐次记录见 `capture-log-c.md`。

## 逐条结果

| # | 说法（一句事实，≤40 字） | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件 / 口径 / 时区 | 状态 |
|---|---|---|---|---|---|---|---|
| C1 | 作者 John Langford 博文称 AI Frontiers 已结束：9 月初并入 Microsoft AI 后他离开 | L2（MSR 研究员署名博客，“作者称”） | https://hunch.net/?p=13764006 | “In early September we were reorged into Microsoft AI.” / “So AI Frontiers is over.” / “As far as I can tell we did good work and were non-consensually reorged away.” | `c-hunch.html` / `c-hunch.txt` / `c-hunch.headers` | 博客字段显示 “Posted on 11/6/2026 … 11/7/2026”，未给时刻；HN 提交 2026-11-07 18:02Z = 11-08 02:02 北京 | 部分支持（只有作者自述；微软无官方回应，见下） |
| C2 | 据彭博社报道，买方讨论 1.2 至 1.3 万亿美元估值，有人低至 1.1 万亿（据知情人士） | L4（Bloomberg） | https://www.bloomberg.com/news/articles/2026-11-06/anthropic-ipo-valuation-expectations-pressured-after-ai-rout | “potential buyers are discussing about a $1.2 trillion to $1.3 trillion valuation range for the company, according to people familiar with the matter. Some are weighing a value as low as $1.1 trillion, one of the people said.” | `c-bloomberg-ipo.html` / `c-bloomberg-ipo.txt` | 页面未显示发布时分；URL 日期 2026-11-06；HN 提交 11-06 16:54Z = 11-07 00:54 北京。买方口径，非定价；“ahead of … formal IPO marketing next week” | 部分支持（L4，知情人士口径） |
| C2' | WSJ：投资人怀疑 IPO 能否达到 1.5 万亿 | L4（WSJ） | https://www.wsj.com/finance/major-investors-are-skeptical-anthropic-ipo-can-achieve-1-5-trillion-valuation-ca965d8b | 未取得原文 | 失败记录：`c-wsj-ipo` 401 Unauthorized，无 .txt | HN 提交 11-06 00:31Z = 11-06 08:31 北京（50386597，90 分） | 未找到一手来源（付费墙/401，不绕过） |
| C3 | 据报道 OpenAI 已募 110 亿美元，估值 1.25 万亿；整轮或达 200 亿 | L4（TechCrunch 转述彭博社） | https://techcrunch.com/2026/11/06/openai-reportedly-raises-an-additional-11-billion-at-a-1-25-trillion-valuation/ | “OpenAI has reportedly raised $11 billion so far in a new funding round that values the AI lab at $1.25 trillion.” / “The round is expected to close in the coming weeks and could total about $20 billion, a source told Bloomberg.” / “TechCrunch has reached out to OpenAI for comment …” | `c-techcrunch-openai.html` / `.txt` / `.headers` | TechCrunch 页面 2026-11-06 14:53 PST = 11-07 06:53 北京；HN 提交 11-07 08:28Z = 16:28 北京。TechCrunch 文中未写 pre/post-money | 部分支持（“reportedly”，未收官方确认） |
| C3' | 247wallst 转述：CFO 在电视采访中确认 1.25 万亿为 pre-money；OpenAI 拒评 | L4/L5（247wallst 转述 CFO 访谈，访谈本身未取得） | https://247wallst.com/investing/2026/11/07/openai-just-raised-11-billion-at-a-1-25-trillion-valuation-below-the-1-4-trillion-it-first-sought/ | “That cash is the first part of a round priced at a $1.25 trillion pre-money valuation, a figure the company’s CFO, Sarah Friar, confirmed in a TV interview. OpenAI declined to comment on the funding.” | `c-247wallst-openai.html` / `.txt` / `.headers` | 页面 2026-11-07 08:38 ET = 11-07 21:38 北京；HN 提交 11-08 14:49Z（50413822，91 分 / 72 评） | 未找到一手来源（CFO 访谈未取得；只见 247wallst 转述） |
| C4 | 据 FT，Anthropic 34 亿美元接手 1.7GW 燃气发电机组 | — | https://www.ft.com/content/41bf5c6a-9735-4f1a-a8e3-2bcccad58ba0（仅 HN 标题可见） | HN 标题：“Anthropic strikes $3.4B deal to generate gas-fired power for data centres”（仅标题，正文未取得） | FT 403（Cloudflare 质询），无正文，未绕过 | 1.7GW 与“接手机组”未核；WebSearch 未找到其他一手报道（搜到的 Nscale 460MW、450 亿美元合同是另一笔交易，不可混淆） | 未找到一手来源（**不进速览**） |
| C5 | CBC 报道，北卡教堂山分校去年一天收到逾 1.1 万次芬兰音乐搜索，来自数千个网络地址 | L4（CBC） | https://www.cbc.ca/news/world/university-libraries-ai-data-scraping-bots-9.7408287 | “the University of North Carolina at Chapel Hill reported last year that it received more than 11,000 searches for Finnish music from thousands of different internet addresses on the same day.” / “The university said its library catalogue was periodically shutting out students, faculty and staff due to bot traffic.” | `c-cbc-libraries.html` / `.txt` / `.headers` | 页面 “Posted: Nov 05, 2026 4:00 AM EST” = 11-05 17:00 北京；HN 提交 11-06 14:13Z = 22:13 北京 | 部分支持：事件是“去年”；原文写 “bot traffic”，未直接写 AI 爬虫；Marmot “20 亿次”与 Duke 引语在 CBC 原文中均未见 |
| C6 | 2.1.319 修复 worktree 沙箱逃逸、Bash 环境前缀权限绕过、sed 跳过权限检查 | L1（官方 CHANGELOG 与 npm 元数据） | https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md ；https://registry.npmjs.org/@anthropic-ai/claude-code | “Fixed a sandbox escape when creating a worktree” / “Fixed a Bash permission bypass where a crafted environment prefix could hide part of a command from permission checks” / “Fixed a case where a `sed` command could skip the permission check in accept-edits and auto mode” | `c-claude-changelog.txt`（1,176,779 字节，**超过 1 MB**） / `c-claude-changelog.html` / `c-npm-claude-code.json`（1,588,865 字节，**超过 1 MB**） | npm：2.1.319 发布于 2026-11-06T00:58:56Z = 11-06 08:58 北京；2.1.318 为 11-03 21:55Z。sed 修复限于 accept-edits 与 auto 模式；环境前缀条目写明“crafted”；沙箱条目未写利用情况 | 已找到（CHANGELOG 原句）；GHSA 对应公告：未找到 |
| C6' | GHSA 公告：截至 11-08 无 2.1.319 修复的对应公告 | L1（GitHub 安全公告 API） | https://api.github.com/repos/anthropics/claude-code/security-advisories?per_page=30 | 列表按时间降序，最新 2026-10-26（GHSA-5xjx-9qx6-jcrv，Edit Tool TOCTOU）；最早一页 30 条 | `c-ghsa-claude-code.json`（114 KB） | 2026-06-25 的 GHSA-7835-87q9-rgvv 为“Git Worktree Path Confusion”导致的沙箱逃逸，版本与问题是否相同未核，不能判同 | 未找到（11-06 之后无公告） |
| C7 | Arena 周报：Kimi-K3.1-Pro 与 Hy4 在 Agent Arena 开放模型中领先 | — | 榜单页 https://arena.ai/leaderboard/agent/pareto 与 https://arena.ai/leaderboard/agent/overall/pareto 均 403（Cloudflare 质询，未绕过） | 未取得 | 失败记录；`c-arena-blog` 索引最新 10-27，无 11-07 周报 | 方法页（`c-arena-methodology`，2026-06-04 发布、10-15 更新）：排名用 “causal tracing”，首个榜单为 orchestrator 模型的因果评估，报告 95% 置信区间 | 未找到一手来源（**不进速览**） |
| C8 | 作者自报：284B DeepSeek-V4-Flash 在 64GB 内存、Zen 5 纯 CPU 上运行，解码约 9–11 tok/s | 一手（项目作者自报，非官方，非独立评测；不套 L1–L4，正文须写“作者自报”） | https://github.com/synchopate/llama-modchip | 标题：“Break the 64 GB RAM Wall: Run 284B DeepSeek-V4-Flash (< 59 GiB RAM) … on Pure CPU — Zero GPUs Required.” / “Opening them in unpatched `llama.cpp`, Ollama, or LM Studio will fail with `unknown tensor type 40`.” | `c-llama-modchip-readme.txt`（25 KB） | README 表：Ryzen 9 9955HX（Zen 5，64 GB，0 GPU）；256K 上下文；56.56 GiB 磁盘、57.61–58.9 GiB VmRSS；预填充 44.36 tok/s；纯 CPU 解码 9.06–10.74 tok/s；Stockfish-PVS 11.5–14.7 tok/s。量化为自定义 `Fractal-64`（Q2_0，3.75 bpw），需作者补丁版运行时；另有 CPU+16GB GPU 混合配置（80.80 tok/s）。质量指标（PPL 增量等）为作者自测 | 已找到（作者自报，条件必须带上） |
| C9 | 8rank.ai 称 Mac 内置未公开的家庭成员分类器 | — | https://8rank.ai/game/apple-intelligence-personal-knowledge-graph（`c-8rank.html` / `.txt`，96 KB） | 未找到 L1–L4 原文。HN 作者自称 “I am the author of the research”（11-07 11:13Z） | 存档已取 | WebSearch 只找到个人分析（DEV Community 一篇、一篇 2026-03 取证笔记）；未见苹果文档或权威媒体确认 | 未找到一手来源（**不进速览**） |

## 各条热度（`barking heat`，取于 2026-11-09 00:51 +08:00；HN 分数与嵌套评论数，记录见 `c-heat.json`）

- C1：50406563 “The end of Microsoft Research's AI Frontiers group”，480 分，嵌套评论 294（11-07 18:02Z）；50406589 “The End of AI Frontiers”，81 分，4 条。
- C2：50394906 “Anthropic IPO buyers see valuation as low as $1.1T”，318 分，嵌套评论 610（11-06 16:54Z）；50386597 WSJ 标题，90 分，105 条（11-06 00:31Z）；50386483，14 分，3 条。
- C3：50402738 TechCrunch 标题，5 分，0 条（11-07 08:28Z）；50413822 247wallst 标题，91 分，72 条（11-08 14:49Z）。
- C4：50412760 FT 标题，25 分，19 条（11-08 12:41Z）。
- C5：50392499 CBC 标题，136 分，60 条（11-06 14:13Z）；50386516，4 分，0 条（11-06 00:21Z）。
- C6：无 HN 条目。
- C7：无 HN 条目（arena 相关检索只得无关结果）。
- C8：50411961 “Llama-modchip: 284B DeepSeek on 64GB RAM, no GPU”，169 分，53 条（11-08 10:22Z）。HN 评论已提到“2 bit?”“At which quantization?”，即量化位宽不在标题中；这是 L6 线索。
- C9：50403097 “Every Mac comes with an (undocumented) family member classifier”，795 分，嵌套评论 387（11-07 09:46Z）。

## 扫描说法勘误（以原文为准）

- **C1**：扫描写“组解散”。原文是 “AI Frontiers is over”，并入 Microsoft AI 后作者离开。“解散”是扫描的措辞，正文宜用“作者称已结束”。扫描写 HN 480/294，与热度一致。
- **C2**：扫描说“WSJ：skeptical … $1.5T（90/108）”。WSJ 原文 401 未取得；热度 90 分、105 条（嵌套）与扫描相近。Bloomberg 的实际口径是 1.2–1.3 万亿区间，“低至 1.1 万亿”仅“有人”这么认为。
- **C3**：扫描的 “110 亿美元” 与 “1.25 万亿” 数字都对。缺少两条限定：“reportedly”与“未完成”（整轮约 200 亿，尚余约 90 亿待到账）；1.25 万亿是 247wallst 转述的 pre-money。HN 热度现为 91/72（扫描写 91/71）。
- **C4**：扫描写“据 FT”。FT 原文未取得，标题只在 HN 可见，因此不能写“据 FT 报道”。
- **C5**：扫描写 “CBC（HN 136/61）”，热度一致。原文 11,000 次搜索是“去年”（reported last year），不是当期；Marmot 20 亿次在 CBC 原文中未见，不能归 CBC。
- **C6**：扫描的三项修复都在 2.1.319 的 CHANGELOG 中；“截至 11/8 最新 10/26”属实（GHSA 列表最新 10-26）。
- **C7**：扫描的 Arena 11/7 帖与 “+5.3%、+5.4%、0.57 美元” 均未取得一手来源。
- **C8**：扫描写 “HN 166/53”，现为 169/53（热度）。“无 GPU”来自标题；README 另有 CPU+GPU 混合配置，且 284B 的 Fractal-64 格式需补丁运行时（unpatched 的 llama.cpp 报 `unknown tensor type 40`）。
- **C9**：扫描写 “HN 795/427”。热度统计嵌套评论 387 条；427 的口径未核，以 heat 数字为准。

## 未完成与限制

- WSJ（C2'）401、FT（C4）403 Cloudflare 质询、Arena 榜单页（C7）403 Cloudflare 质询：按规则记失败，未绕过，未使用代理阅读器。
- SEC EDGAR（查 Anthropic 11-02 之后的 S-1/A 修订）返回 403 “Undeclared Automated Tool”。未伪造 User-Agent，未重试，未核。
- 超过 1 MB 的文本存档（需 Claude 决定入库与否）：`c-claude-changelog.txt`（1,176,779 字节）、`c-npm-claude-code.json`（1,588,865 字节）。
- 无待删除文件。
