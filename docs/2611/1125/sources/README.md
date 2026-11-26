# 1125 来源与制作状态

期号1125（上期1124+1）；线索为 Develata 回贴的 WorkBuddy HTML 选题扫描 1126-18（生成于北京 11-26 18:00）。同时附来的 ChatGPT rescan 邮件是 11-19 13:16 的旧件，与 1124 期附的是同一份，没有用。Develata 北京 11-26 约 21:30 选定主帖：GPT-6.2 计费修正、黎曼证明短版（跟进 1122）。

派工单 `.handoff/2026-11-26-1125-evidence.md`。派工前 `opencli doctor`（北京 21:40）：Browser Bridge 已连接。A、B 组为 Sonnet 5.5（effort high），C 组为 Haiku 5.5（effort high）。

扫描勘误：
- Hirsch 猜想手稿的热度“HN 494 分 / 356 评”其实是 Google Cogentic 帖（item 50642086，495 / 359）；Hirsch 帖（50644980）为 167 分（Claude 查 HN）。
- 扫描“不建议发”一节把 Aaronson 博文（HN 1700 分）按它自己的台账排除，`daily-scan.md`“已发过”里没有；已补进速览。“P≠NP 置信度回到 96%”原文没有（C 组）。
- 扫描称 grh-lean“99.5% 是三个数字揉成一句”：B 组判定不成立。99.5% 来自紧凑版（约7.6万行）对原版（约1500万或1737万行）的行数比，但分子是代码行、分母是总行，口径不一（同口径总行比约少99.35%）；“240 万 / 8.4% / 16.5%”出自 11-24 量子位稿，量的是原版的社区删改，早于 grh-lean 两天。
- 扫描称质疑者“Jenny Lorraine Nelson”、anyGeometry 在 grh-lean：姓 Nielsen，定义在原版，grh-lean 两版没有（B 组）。
- 扫描称 rh-trim“allChi 一步占38%”、rh-kernel-backbone“24.1 万条重复定理”、头条“三个数字”纠错稿（11-25 14:46）、“8 天”：均未找到（B 组）。
- 扫描称 TechCrunch 11-25 引发言人“少数重度用户可能更早触顶”与计费修正有关：该稿讲 Chat 与 Work 合并，没提 GPT-6.2 计费（A 组）。
- 扫描称原帖提到“Ultrafast”“Codex 通知邮件”：原帖没有；Ultrafast 见帮助页，邮件只有用户晒图（A 组）。
- 扫描称 Mellum2.1“省45% token”：原文“about half as many tokens”；Private Inference“企业试点”：原文“a small group of customers”；Anthropic Rust 迁移“内存降80%”：C 组在本地正文文本里没找到，反向核验第二轮在官方图15替代文字里读到“more than 80%”（全部生产环境口径），中文稿有出处。

## 主帖

1. GPT-6.2 计费修正（A 组，`evidence-a.md`、`capture-log-a.md`）：
   - Tibo 原帖（海外社媒，北京 11-26 10:50；fxtwitter JSON `a-x-tibo-2125784147889393806.raw.json`）。
   - [Codex 套餐页](https://developers.openai.com/codex/pricing)（同页 learn.chatgpt.com/docs/pricing；Wayback 快照 11-15、11-18、11-19、11-23、11-24、11-25、11-26）；[帮助中心 20001516](https://help.openai.com/en/articles/20001516)、[20001625](https://help.openai.com/en/articles/20001625-understand-chatgpt-and-codex-usage-limits)、[20001275](https://help.openai.com/en/articles/20001275)；[Codex changelog](https://developers.openai.com/codex/changelog)。
   - 夸大实例：[智东西（凤凰网）](https://tech.ifeng.com/c/8yMr1LnINIO)“突发！ChatGPT额度爆砍接近一半？”；Notebookcheck 中文版“时间最多缩短 40%”；HN 标题“OpenAI cuts GPT-6.2 usage”。较准确：[Odaily（腾讯新闻）](https://news.qq.com/rain/a/20261126A06U1M00)。
   - 背景：[TechCrunch 11-25](https://techcrunch.com/2026/11/25/openai-says-heavy-chatgpt-users-may-hit-limits-sooner-as-it-merges-chat-and-work/)（Chat 与 Work 合并）。
   - 热度：HN item 50651117（21:42 为 345 分 / 192 评）；r/codex 帖 905 分 / 469 评（浏览器读）。
2. 黎曼证明短版（B 组，`evidence-b.md`、`capture-log-b.md`）：
   - [lsernau/grh-lean](https://github.com/lsernau/grh-lean)（首提交北京 11-26 04:38，建仓 04:42）；Sernau 在 Lean Zulip 的宣布（05:26）与问答（`b-zulip-excerpts.md`）。
   - [MIT Sutherland 短版复核页](https://math.mit.edu/~drew/shortgrh/)（最后更新 10:50）。
   - [rh-shrink](https://rh-shrink.github.io/)（社区仓库代码量统计，文本检索不跑 Lean）；[leanprover-community/riemann-hypothesis](https://github.com/leanprover-community/riemann-hypothesis)（11-26 06:24 换成可读版）；rh-kernel-backbone、rh-trim、rh-proof-trimmed、VerifyTask（见 evidence-b 第四节）。
   - 夸大实例：[量子位（腾讯新闻）](https://news.qq.com/rain/a/20261126A051B600)“暴砍了99.5%”（08:40）；夕小瑶科技说、IT之家、极客公园同题；[量子位 11-24（36氪）](https://www.36kr.com/p/4083605367761538)“两天砍掉240万行”。
   - 热度：HN item 50648050（21:42 为 489 分 / 281 评）。

## 速览（C 组，`evidence-c.md`、`capture-log-c.md`）

- Aaronson：[博文](https://scottaaronson.blog/?p=10379)（北京 11-26 03:01）。
- OpenAI Hirsch：[openai/math 手稿](https://github.com/openai/math/blob/main/preprints/Superpolynomial-graph-diameter-of-simple-polytopes-November-24-2026/paper.pdf)。
- Anthropic Rust：[官方博文](https://www.anthropic.com/research/migrating-api-to-rust)。
- OpenAI 蛋白质：[官方页](https://openai.com/index/gpt-6-models-demonstrate-intrinsic-protein-design-capabilities/)。
- OpenAI Private Inference：[官方页](https://openai.com/business/private-intelligence)；Altman 帖（海外社媒，北京 11-26 01:12）。
- 亚马逊与 Meta：[Fortune](https://fortune.com/2026/11/25/exclusive-amazon-meta-talks-deal-muse-ai-shopping-agent-blockade/)。
- 未收：纽约州 AI 聊天玩具禁令（风控，留给 Develata）、OpenAI 蛋白质设计、纽约时报与 Photoshop（限定词写全超40字）、JetBrains Mellum2.1、蚂蚁 Ling-3.2-air（理由见 `fact-check.md`）。

## 待删除与不入库

- 已删（取证组列出的试验与失败抓取）：`images/03-pricing-test-wide.png`、`07-hn-title-score.png`、`08-hn-comments-quota-data.png`、`09-openai-launch-usage-claim.png`；`sources/` 下 `a-cn-bitw.*`、`a-cn-notebookcheck.html/.headers`、`a-cn-v2ex.*`、`a-cn-linuxdo.*`（均为 Cloudflare 挑战页）、`a-techcrunch-try.*`、`c-c9-openai-protein-x.*`、`c-c6-anthropic-rust-hn-src.*`、`c-c7-nysenate-s9408.html/.headers`、`c-c7-ny-governor.html/.headers`（403 挑战页）、`c-c7-nysenate-a11144.*`、`c-c5-fortune-check.*`、`c-c5-techcrunch-muse.*`、`c-c8-aaronson-rss.xml`。
- 超过 1 MB、Develata 决定不入库并删除（北京 11-27）：`sources/a-codex-changelog.txt`（1.1 MB，由 `a-codex-changelog.html` 转出的文本；HTML 原件随 offsite 上传，A 组只用到其中 2026-11-17 条目，摘句见 `evidence-a.md` A2-3、A3-1）；`sources/b-verifytask-board.json`（1.1 MB，VerifyTask 榜单 API 原始数据，正文未用；B 组摘录见 `evidence-b.md` 4.3）。
