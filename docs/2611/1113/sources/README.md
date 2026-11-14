# 1113 来源与制作状态

期号1113（上期1112+1）；线索为 Develata 回贴的两份 HTML 选题扫描 1114-06（北京 11-14 06:00）与 1114-18（北京 11-14 18:00）。Claude 对照“已发过”与“近 3 天速览已收”去重、现查 HN 热度（Algolia，北京 11-14 21:45）后列候选，Develata 拍板两条主帖（哥德尔笔记、Anthropic Anvil），OpenAI 机器人团队降为速览。

派工单 `.handoff/2026-11-13-1113-evidence.md`（文件名日期按期号；`barking handoff` 把本机时区写成美东，已改为北京时间，本机为 macOS、UTC+8）。派工前 `opencli doctor` 通过。三组执行方：A、B 为 Claude Sonnet 5.5（`claude-sonnet-5-5`，harness 计约 34、20 分钟），C 为 Claude Haiku 5.5（`claude-haiku-5-5`，约 6 分钟）。北京 11-14 约 21:50 派出。取证全由 Claude 家族完成，反向核验时由 Codex 逐行核对事实清单与存档。

## 主帖

1. 哥德尔笔记：[《卫报》Alex Bellos 稿](https://www.theguardian.com/science/2026/nov/13/kurt-godel-notebooks-ai-helps-unlock-hidden-discoveries-man-who-transformed-maths)（北京 11-14 00:38）、[HN 50486748](https://news.ycombinator.com/item?id=50486748)（403 分 / 101 评）、[维也纳大学演讲预告 PDF](https://vcs.univie.ac.at/WK_Vorlesung_26.pdf) 与 [Uni Wien 直播](https://www.youtube.com/watch?v=L7I57U4GSv8)（10-19 CEST）、AITP 2026 幻灯片（`a-aitp-slides.txt`）、ERC CORDIS GODELIANA 787758 与 HERITAGE 101198605、[新智元稿（36氪转载）](https://www.36kr.com/p/4069178330764161)
   - 所有数字只有《卫报》一处（L4，von Plato 口述）。von Plato 10-19 维也纳演讲是更早的一手，但数字只见于 YouTube 机器字幕（500/1,084 页、55 条错、5–8 条在做、14 个主题），没人听过音频，正文不用；字幕的 55+5~8 能补上《卫报》400+200+50=650 与 663 之间的约 13 条差额。
   - ERC 资助额不一：《卫报》“€2.5m (£2.1m)”，CORDIS GODELIANA €2,132,993（2018–2024）、HERITAGE €2,294,070（2025–2031）；页数口径不一：《卫报》1,009、赫尔辛基项目页约 1,200、AITP 摘要约 1,700；正文采用《卫报》1009页并署名，资助额未进正文。
   - 据《卫报》，提供给 Claude 的材料包括 von Plato 的转写稿（反向核验第一轮：“Claude 没读速记”是推断，已撤）；AITP 幻灯片把机器读速记列为未决挑战（未提 Claude）。Hamkins 本人 Bluesky 帖反而写 “with AI translating the mysterious Gabelsberger script”（图 12），正文不写。
   - 未找到：Anthropic 官方文章（员工社媒帖无搜索接口、未完成检索）、赫尔辛基大学新闻稿、论文或预印本、公开的 Coq 文件（arXiv、GitHub 检索 0 条）。Der Standard 全文被订阅墙挡住。
2. Anthropic Anvil：[Safeguards 博客原文](https://safeguards.anthropic.com/2026/automated-red-teaming/)（页面日期 11-12，未给时刻）、[RSP 更新页](https://www.anthropic.com/rsp-updates)（10-19 条已写 23/24）、[智东西网易号](https://www.163.com/dy/article/LBKGPTI4051180F7.html)（北京 11-13 15:19）、MIXED News 10-23
   - HN 没有本文的帖子；中文侧只找到智东西→网易、凤凰网同稿与 AI Safety HOT 聚合。热度门槛靠中文侧传播与扫描判断，偏弱，记一笔。
   - 23/24、24–48 小时在 RSP 更新页 10-19 已公开，11-12 是详细版；正文不写。
   - 官方文自身前后不一：网络任务耗时摘要 0.4–4.4 小时、正文 0.5–3.5 小时、图 5 为 0.39–4.4 小时。扫描的“Anvil 一周内完成”原文无；“人类 0 个”不准（对 Opus 5.5 的 13 个里有 1 个完整利用）。107/114、Spearman 0.92 只在图内。
   - 1/235 是 435/102,200 的算术比，官方文未写；人类一次战役约 204 万次请求、Anvil 约 13.2 万次。

## 速览

- [ChatGPT Wallet](https://openai.com/index/introducing-chatgpt-wallet/)（11-13，HN 13 分）。扫描的“上线初期不抽成”官方文未见。
- [Public Citizen 等四家非营利组织起诉五家 AI 公司](https://www.citizen.org/news/coalition-of-nonprofits-files-landmark-lawsuit-to-stop-development-of-advanced-ai-systems-at-anthropic-google-meta-openai-and-spacexsi-that-proliferate-bioweapons-and-cyberattacks/)（11-12，原告新闻稿；第五被告原文写 SpaceXSI；“不索赔”未找到）。
- [Fortune：OpenAI 解散独立机器人团队](https://fortune.com/2026/11/13/openai-disbands-robotics-team-as-its-general-models-start-running-robots/)（北京 11-14 01:05，HN 172 分）。
- 未收：Claude Code Projects（官方文章 9-17，HN 帖指向旧文；扫描的“单线程只读分享”与文档相反）、LangChain C 轮（融资）、Devin Teams 席位 200→100（定价细节；扫描的 “still subject to usage limits” 括注原文无）、Antigravity Dynamic Workflows（11-13，HN 0 条）、Sierra Poppy 1.0（HN 0 条；“实装 3 家”只是点名三家）、Blackwell 芯片指纹（风控）、旧闻重推若干。

## 备注

- B 组截图 13–24 用尽后顺延到 25–30（C 组本期不截图）。
- 超过 1 MB 的文本存档 `c-c4-cc-changelog.txt`（3.4 MB，Claude Code 整站 changelog 的转文本，只用于核对 Projects 条目，该条未收）已删除，不入库；HTML 原件 `c-c4-cc-changelog.html` 随网盘保存，可重新转文本。
- 交付前已删除：`images/01-guardian-title.png`（被 `01-guardian-title-full.png` 取代）、`sources/a-yt-lecture-en-translated.vtt/.txt`（疑为机器翻译字幕轨，未引用）、`sources/b-rsp.*`（未用）。fetch-log 里仍留有这些条目的记录。
