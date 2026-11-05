# 1104 期 C 组取证：速览一（C1–C4）

取证时刻：北京 2026-11-05 17:19–17:21（本机 EST 04:19–04:21）。不截图（按派工单）。存档在本目录，前缀 `c-`；抓取与热度记录见 `capture-log-C.md`。

格式：说法 | 级 | 一手来源 | 原文摘句（原语言，逐字） | 存档 | 条件/口径/时区 | 状态

## C1 OpenAI Images 2.6

| 说法 | 级 | 一手来源 | 原文摘句 | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Images 2.6 上线 ChatGPT、Codex、API | L1 | https://openai.com/index/introducing-images-2-6/ | "We’re introducing Images 2.6 in ChatGPT, Codex, and the API, bringing your ideas to life faster." | c-c-oai-images26.html / .txt | 官方页日期 "November 4, 2026"，官方未给时刻；HN 提交 11-05 07:35 北京时间 | 已找到 |
| 速度提升 50% | L1 | 同上 | "Images 2.6 generates images 50% faster than before." | 同上 | 基线（"before"）官方未写明是哪一版、哪种质量档 | 已找到（基线未写） |
| Flare 最快 6 秒 | L1 | 同上；模型页 https://developers.openai.com/api/docs/models/gpt-image-2.6-flare | "When quality is set to low, Flare can generate images in as little as six seconds." | c-c-oai-images26.txt；c-c-oai-flare-price.txt | 条件是 quality=low，"as little as" 为下限；模型页 Speed 与 Performance 均标 "Not specified" | 已找到（限定：仅 low 质量、"as little as"） |
| 对话式 GIF 动画 | L1 | 同上 | "With this, you can bring motion to your images with GIFs" | 同上 | 官方页未写 GIF 在哪些产品可用 | 已找到 |
| API 分 Flare 与 Sunburst | L1 | 同上 | "In the API, we offer GPT‑Image‑2.6 Flare and Sunburst for developers to choose from." | 同上 | — | 已找到 |
| 谁能用：ChatGPT 与 Codex 所有订阅档 | L1 | 同上 | "Images 2.6 is available globally today in ChatGPT and Codex across all subscription tiers on desktop, mobile, and web." | 同上 | API 那句写的是 Sunburst 与 Flare "are available in the API"，未逐条写 API 的使用门槛 | 已找到 |
| 每周 40 亿张图 | L1 | 同上 | "People around the world now create 4 billion images per week across ChatGPT, Codex, and the API." | 同上 | 口径：ChatGPT + Codex + API；"more than half a billion images created per day" 为官方另句 | 已找到 |
| 9 月 Images 2.5 公告为 30 亿 | L1 | https://openai.com/index/introducing-chatgpt-images-2-5/ | "Every week, people create more than 3 billion images across ChatGPT Images and the GPT‑Image models in the API." | c-c-oai-images25b.html / .txt | 官方日期 "September 8, 2026"（页内），口径为 ChatGPT Images + API 的 GPT-Image 模型，不含 Codex | 已找到；注意口径不同（2.6 多了 Codex 一项），不宜直接当同口径增长 |
| Flare 价格（页内表） | L1 | https://developers.openai.com/api/docs/pricing ；模型页 gpt-image-2.6-flare | 模型页："Image tokens … Input $8.00 / Cached input $2.00 / Output $30.00"（每 100 万 tokens） | c-c-oai-pricing.txt；c-c-oai-flare-price.txt | 按 token 计费；定价表中 gpt-image-2.5-flare 为 $4.00 / $15.00 / $1.00，即 2.6 Flare 同列价格约为其两倍（表值，未在官方文字中作比较） | 已找到（线索，待正文决定是否写） |
| 完整产品范围与 Codex | L1 | 同上 | "Images 2.6 is available globally today in ChatGPT and Codex…" | 同上 | — | 已找到 |

HN 热度（北京时间，取证时刻 17:20）：《Images 2.6》https://openai.com/index/introducing-images-2-6/ — 284 分 / 169 评，提交 11-05 07:35（HN item 50372945）。派工单所列 273 分 / 168 评为较早快照，以本表为准。

## C2 Ricursive Ripple-65B Preview

| 说法 | 级 | 一手来源 | 原文摘句 | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 美国开放权重 SOTA（agentic coding） | L1 | https://www.ricursive.com/ripple | "Achieving U.S. Open-Weight SOTA on Agentic Coding" ；正文 "achieving U.S. open-weight state-of-the-art (80.52) on TerminalBench 2.1" | c-c-ricursive-ripple.html / .txt | 限定词：U.S.、open-weight、agentic coding、TerminalBench 2.1；官方页未显示发布日期，HN 提交 11-05 02:53 北京时间 | 已找到（限定词必须保留） |
| 超越 Reflection Beam 501B 等 | L1 | 同上 | "surpassing significantly larger US open-weight models such as Reflection Beam 501B (80.1), Meta Muse Spark Open 816B (76.4), Thinking Machines Inkling 975B (63.8), and NVIDIA Nemotron 3 Ultra 550B (56.4)." | 同上 | 数字为官方页自报 | 已找到 |
| 模型卡标 TB2.1 为自报 | L1 | https://huggingface.co/ricursiveAI/Ripple-65B-Preview | 表行标题 "Terminal-Bench 2.1 (self-reported)"：Ripple-65B 80.5、Nemotron 3 Ultra 56.4、Inkling 63.8、Muse Spark Open 76.4、Reflection Beam 80.1 | c-c-hf-ripple.html / .txt | 同表另一行 "Terminal-Bench 2.1 (Terminus 2)"：Ripple 80.5、Beam 70.8、Muse Spark Open 78.7、Inkling 56.2、Nemotron 55.1；模型卡注："we use our own in-house eval flow for missing evals (italized)" 且 "Terminal-Bench 2.1 scores were universally lower … when using the official Terminus 2 harness at 128K sequence length with summarization" | 已找到（自报标注确认） |
| 65B 稠密模型 | L1 | 同上 | "a 65B parameter dense Transformer model pretrained on 12 trillion tokens with support for up to 1M context length." | 同上 | 官方博客同一数字 | 已找到 |
| 许可证 | L1 | HF 页头 "License: apache-2.0"；LICENSE 文件 https://huggingface.co/ricursiveAI/Ripple-65B-Preview/blob/main/LICENSE | "Apache License" 全文（c-c-hf-ripple-license.txt）；模型卡 "License: Apache 2.0" | c-c-hf-ripple-license.* | 本仓库（BF16 instruct 版）；NVFP4 与 base 为另外的 HF 仓库，本次未取证 | 已找到（Apache 2.0，不是 MIT 或自定义协议） |
| 来自 Ricursive 自家芯片设计平台的模型 | L1 | 官方页 | "we trained a language model from scratch that sets a new U.S. open-weight SOTA on agentic coding." ；"Today, we release Ripple-65B Preview, a general-purpose variant of the proprietary model that powers our AI chip design platform." | c-c-ricursive-ripple.txt | — | 已找到 |
| 训练成本 32M 美元（官方自述，线索） | L1 | 官方页 | "we developed and trained Ripple-65B Preview for only $32M, which is one-tenth the cost of the previous open-weight SOTA on agentic coding, as estimated to the best of our knowledge." | 同上 | 自估；官方页原文 | 已找到（不属于速览所需，仅备查） |

HN 热度（17:20 取）：《Ripple-65B Preview: U.S. Open-Weight SOTA on Agentic Coding》https://www.ricursive.com/ripple — 289 分 / 104 评，提交 11-05 02:53（item 50369925）。派工单的 285 / 103 为较早快照。另有 https://twitter.com/RicursiveAI/status/2118046765857546733 的 HN 提交 1 分 / 0 评（item 50369618，11-05 02:30）。

## C3 Cognition SWE-2 Flash

| 说法 | 级 | 一手来源 | 原文摘句 | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| "fully American coding model" | L1 | https://cognition.ai/blog/swe-2-flash （跳转至 https://cognition.com/blog/swe-2-flash，两者存档内容相同） | "We’re introducing SWE-2 Flash, a fully American coding model that sets a new bar for intelligence at low cost and high speed." | c-c-cognition-swe2flash.* ；c-c-cognitioncom-swe2flash.* | 博客日期 "2026.11.04"，官方未给时刻；HN 提交 11-05 04:31 北京时间（域名 cognition.com） | 已找到 |
| 超过所有美国开源权重模型（限定：leading evals） | L1 | 同上 | "On leading evals, SWE-2 Flash outperforms all American open-weight models, even models that are 3x larger." | 同上 | 限定词 "On leading evals" 必须保留；比较对象是页内表中列出的模型 | 已找到（派工单原句为缩写，正文须带 "leading evals"） |
| 权重暂不公开 | L1 | 同上 | "so we are not sharing the weights publicly yet." ；"we are building a program to securely share the weights with industry partners and researchers. More on this in a few weeks." | 同上 | 原因写为网络安全风险；承诺 "in a few weeks" 为官方原句 | 已找到 |
| 基于 Inkling-Small（Thinking Machines） | L1 | 同上 | "We began with a large-scale mid-training stage on top of Inkling-Small … a model with 276B total and 12B active parameters." | 同上 | — | 已找到 |
| 价格为 SWE-2 的五分之一、速度三倍 | L1 | 同上 | "It is available today in Devin Desktop and CLI, at one-fifth the price and 3x the speed of its much larger sibling, SWE-2." | 同上 | 速度图为 "simulated 200 vs. 60 tokens per second" 的回放，非实测吞吐 | 已找到（速度为模拟回放） |
| 免费期 | L1 | 同上 | "Free through 11/30" ；"Try it today in Devin Desktop and CLI" | 同上 | 截止日期官方写 11/30，未写时区 | 已找到 |
| 其他对比（备查） | L1 | 同上 | Terminal-Bench 4.0 表：SWE-2 Flash 20.0，GLM 5.3 Flash 41.8，DeepSeek V4.1 Flash 1017 42.3，Claude Haiku 5.5 39.2，GPT-6 Luna 16.4，SWE-2 27.3（表值） | 同上 | 同表中美国开源 Beam 1.0 为 4.0，Muse Spark Open 为 8.0；该表中 TB 4.0 上 SWE-2 Flash 低于 GLM 5.3 Flash 与 DeepSeek V4.1 Flash | 已找到（表值，备查；"all American open-weight" 的限定由此可见） |

HN 热度（17:20 取）：《SWE-2 Flash》https://cognition.com/blog/swe-2-flash — 6 分 / 1 评，提交 11-05 04:31（item 50371089）。

## C4 Cat Wu（Anthropic → OpenAI）

| 说法 | 级 | 一手来源 | 原文摘句 | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 加入 OpenAI，负责 Core Products 产品管理 | L2 | X 帖 https://x.com/_catwu/status/2118137612577968402（经 fxtwitter JSON 存档） | "Some news: I’m joining OpenAI to lead product management for Core Products!" | c-c-catwu-fx.json / .headers | 帖子 created_at "Thu Nov 05 00:25:34 +0000 2026"，即北京 2026-11-05 08:25:34（UTC+8）；帖中不提 Anthropic | 已找到（个人发言，L2） |
| 与 @thsottiaux 团队合作、打造 agents | L2 | 同上 | "I’m excited to work with @thsottiaux, an incredible PM team, and everyone across OpenAI. Together, we’ll build agents that have your back and your team’s…" | 同上 | 派工单扫描称 "与 @thsottiaux 团队做 Agent 产品"；帖子原文是 "work with"，未说"负责 Agent 产品" | 部分支持（"Agent 产品" 是扫描概括，帖子原文只说 build agents） |
| 从 Anthropic 离职（BI 报道） | L4 | https://www.businessinsider.com/claude-code-leader-cat-wu-exits-anthropic-before-big-ipo-2026-11 | "Cat Wu, who led the product side of Claude Code at Anthropic, has left the AI startup." ；"left the company on Friday." ；"Wu announced her departure on Tuesday afternoon on X" | c-c-bi-catwu-exit.html / .txt | BI datePublished "2026-11-03T23:55:09.310Z"，即北京 2026-11-04 07:55:09；BI 报道本身仍为二手（L4）；Anthropic "The company confirmed her departure." | 已找到（L4，须写"据 BI 报道"） |
| 加入 OpenAI（BI 报道） | L4 | https://www.businessinsider.com/cat-wu-joins-openai-from-anthropic-product-lead-coup-2026-11 | "Cat Wu, who led the product side of Claude Code at Anthropic, left the AI startup for rival OpenAI." ；"She'll work at OpenAI with Nick Turley on enterprise products and Ashley Alexander on consumer products, and report to Tibo Sottiaux, an OpenAI representative said." | c-c-bi-catwu-joins.html / .txt | BI datePublished "2026-11-05T00:57:29.788Z"，即北京 2026-11-05 08:57:29 | 已找到（L4） |
| 职位 | L2 / L4 | X 帖（L2）；BI（L4） | X 帖：lead product management for Core Products；BI 段落："leading product management for OpenAI’s core products team"（BI 转述，非 OpenAI 原句） | 同上 | 正文用 X 帖原句 "Core Products 产品管理" | 已找到 |
| "报告给 Tibo Sottiaux" | L4 | BI joins 文章 | "report to Tibo Sottiaux, an OpenAI representative said." | c-c-bi-catwu-joins.html（txt 已移出公开仓库） | 出自 OpenAI 代表匿名转述（BI 报道）；X 帖只写 "work with" | 部分支持（只来自 BI 转述，非 OpenAI 官方发布） |

注：派工单原列的 BI 时刻 "11/4 09:07" 与 "11/5 09:04" 实为 HN 提交时间（item 50360366、50373682），不是 BI 发布时间。BI 页内 datePublished 为上表所列北京时间。

HN 热度（17:20 取）：
- 《Cat Wu, product lead for Claude Code, has left Anthropic》https://www.businessinsider.com/claude-code-leader-cat-wu-exits-anthropic-before-big-ipo-2026-11 — 3 分 / 5 评，提交 11-04 09:07（item 50360366）。
- 《Cat Wu joins OpenAI from Anthropic in product-lead coup》https://www.businessinsider.com/cat-wu-joins-openai-from-anthropic-product-lead-coup-2026-11 — 6 分 / 0 评，提交 11-05 09:04（item 50373682）。

## 隐私与存档说明（C4）

- `c-c-catwu-fx.json` 为 fxtwitter 返回的公开帖子 JSON，内含帖主（Cat Wu）的公开资料字段，包括 screen_name、头像 URL、粉丝数、bio，以及 location 字段值 "sf"。这些是帖主自己的公开资料，不是抓取者的信息；抓取者的头像、昵称与 handle 未出现在该文件中（响应头已裁剪）。派工单要求的是抓取者信息占位符化，此处不需要替换，但 location 字段的去留由 Claude 决定（正文不需要它）。
- BI 两篇存档为原页面 HTML 与文本，含 BI 的分享与订阅插件链接（非抓取者信息）。

## 扫描说法勘误

1. C1：扫描称 "HN 273 分 / 168 评"；取证时刻实测 284 分 / 169 评（HN item 50372945），已按实测记。
2. C1：扫描称 "北京 11/5 约 06:19？" 为发布时刻；官方页只写 "November 4, 2026"，官方未给时刻，HN 提交为 11-05 07:35。
3. C1：扫描称 "跨产品每周 40 亿张图（9 月 Images 2.5 公告为 30 亿）"——两个数字均已找到，30 亿为官方 "more than 3 billion"（2026-09-08，口径不含 Codex）；40 亿为官方 "4 billion"（口径含 Codex）。口径不同，不宜直接写成同口径增长。
4. C2：扫描称 "HN 285 分 / 103 评（北京 11/5 02:53）"；实测 289 分 / 104 评。
5. C2：派工单要求的"许可证"：实际为 Apache 2.0（HF 页头与 LICENSE 全文确认），不是 MIT 式或自定义协议。
6. C2："Terminal-Bench 2.1 80.52（模型卡标 self-reported）"：模型卡表中为 80.5，官方页文字为 80.52；表行确实标 "self-reported"。
7. C3：派工单写的 "outperforms all American open-weight models" 缺少官方限定词 "On leading evals"；正文须带上。
8. C3：速度 "3x the speed" 的对比来自模拟回放（200 vs 60 tok/s），不是实测吞吐。
9. C4：派工单的 BI 时刻 "11/4 09:07" 与 "11/5 09:04" 为 HN 提交时间，BI 页内发布时间分别为 11/4 07:55 与 11/5 08:57（北京时间）。
10. C4："与 @thsottiaux 团队做 Agent 产品" 中"负责 Agent 产品"不是帖子原文；帖子原文为 "work with @thsottiaux … we’ll build agents"。
11. C4："向 Tibo Sottiaux 汇报"只见于 BI（OpenAI 代表匿名转述），X 帖未写。

## 可能的吠点（C 组只需扫描勘误与事实，此节为备查，不进速览）

- C2：官方页与模型卡均把 80.5 与 Beam 80.1 放在"自报"行中；Terminus 2 行中 Beam 为 70.8。"U.S." 限定只针对美国开放权重模型，不含中国模型（表中不列）。
- C3：同页 Terminal-Bench 4.0 表中，SWE-2 Flash（20.0）低于 GLM 5.3 Flash（41.8）、DeepSeek V4.1 Flash 1017（42.3）与 Claude Haiku 5.5（39.2）。"outperforms all American open-weight models" 的比较范围因此只限美国开源模型。
- C1：2.6 Flare 的定价表中单价为 gpt-image-2.5-flare 的两倍（$8 / $30 vs $4 / $15 每百万 tokens），官方文字未作此比较。

## 社区质疑（HN，未深挖，仅线索 L6）

- C1、C2、C3 的 HN 帖子评论未逐条摘录（派工单 C 组要求的是热度，不是评论）。如正文需要社区质疑，由 Claude 另行安排。

## 未找到 / 未取得

- Images 2.6 的 Sunburst 模型页：未存档（派工单未要求）。
- Ricursive 发布日期：官方页未显示。
- Cognition 博客的时刻：官方只写 2026.11.04。
- Reddit 热度：4 个查询全部失败（见 capture-log-C.md）。
- 2 个猜测 URL 的 404 结果（见 capture-log-C.md 待删除清单）。
