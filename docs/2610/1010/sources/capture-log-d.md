# 1011 期 D 组（速览 D1–D8）抓取日志

时间为北京时间（UTC+8）。本机为美东 EDT，抓取时刻以 UTC 记录后换算。工具：curl（普通浏览器 UA，未登录，未使用浏览器会话）、python 脚本（d-html2txt.py 转文本，d-heat.py 查 HN Algolia）、fxtwitter JSON、WebSearch（仅用于找线索，不作为引用依据）。未运行读取本机 Cookie 的工具，未绕过挑战页，未用镜像或转载站。

注：本日志首次记录的抓取时刻误用了 `TZ=Asia/Shanghai date`（本环境下输出的是 UTC），下表已按 +8 小时更正。

## 抓取记录

| 北京时间 | 目标 URL | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 10-11 15:53 | https://api.fxtwitter.com/status/2108931348857827686 （纳德拉 X 帖） | curl | HTTP 200，含长文 article 字段，全文 21 块 | d-d1-nadella.json（原始）、d-d1-nadella.txt（长文转纯文本，由 JSON article.content.blocks 提取） |
| 10-11 15:53 | https://www.theverge.com/ai-artificial-intelligence/1009337/satya-nadella-says-we-should-assume-all-ai-models-are-compromised | curl | HTTP 200，412 KB，正文完整 | d-d1-verge.html、d-d1-verge.txt |
| 10-11 15:53 | https://api.fxtwitter.com/status/2108578092558250148 （ARC Prize 帖） | curl | HTTP 200 | d-d2-arc-post.json、d-d2-arc-post.txt（由 JSON 生成） |
| 10-11 15:53 | https://arcprize.org/competitions/2026/arc-agi-2 | curl | HTTP 200，规则页正文完整 | d-d2-arc-rules.html、d-d2-arc-rules.txt |
| 10-11 15:53 | https://arcprize.org/leaderboard | curl | HTTP 200，正文中未找到 88.06 或 TUFA | d-d2-arc-leaderboard.html、d-d2-arc-leaderboard.txt |
| 10-11 15:55 | https://www.kaggle.com/competitions/arc-prize-2026-arc-agi-2/leaderboard | curl | HTTP 200 但仅 5.8 KB 的 JS 壳，无排行榜数据。同一壳的 overview 与根页面内容相同，已删除重复文件 | d-d2-kaggle-leaderboard.html（仅留作证据，未转文本） |
| 10-11 15:53 | https://www.reuters.com/business/nvidia-talks-invest-further-reflection-ai-or-buy-it-ft-reports-2026-10-10/ | curl | HTTP 401，返回 Cloudflare 类“Please enable JS”挑战页，非正文。已删除挑战页文件 | 无 |
| 10-11 15:53 | https://www.ft.com/content/052610c5-22b4-4dd4-932e-b7f9f0628b6a | curl | HTTP 403，返回 FT “Security Verification” 挑战页，非正文。已删除挑战页文件 | 无 |
| 10-11 15:53 | https://reflection.ai/blog/introducing-beam | curl | HTTP 200，正文完整，页面日期 October 5, 2026 | d-d3-reflection-beam.html、d-d3-reflection-beam.txt |
| 10-11 15:53 | https://blogs.windows.com/windowsdeveloper/2026/10/07/microsoft-execution-containers-policy-driven-containment-for-ai-agents/ | curl | HTTP 200，正文完整，meta 发布 2026-10-07T18:00:28Z | d-d4-mxc.html、d-d4-mxc.txt |
| 10-11 16:03 | https://github.com/microsoft/mxc | curl | HTTP 200，README 正文（未检出 1.0.0 字样，仅作背景） | d-d4-mxc-github.html、d-d4-mxc-github.txt |
| 10-11 16:03 | https://github.com/microsoft/mxc/releases | curl | HTTP 200，含 “MXC SDK v1.0.0” 发布说明（GitHub 页面显示 “07 Oct 05:54”，时区未标明） | d-d4-mxc-releases.html、d-d4-mxc-releases.txt |
| 10-11 15:53 | https://momo5502.com/posts/2026-10-09-game-decompilation/ | curl | HTTP 200，正文完整，页面日期 October 9, 2026（无时刻） | d-d5-momo.html、d-d5-momo.txt |
| 10-11 15:53 | https://www.cockroachlabs.com/blog/experiment-running-hospital-code/ | curl | HTTP 200，正文完整；meta 发布 2026-10-09T00:00:00Z（占位零点），页面 Published on October 9, 2026。表格类统计（如 1299）在静态文本中不可读 | d-d6-cockroach.html、d-d6-cockroach.txt |
| 10-11 15:55 | https://www.linkedin.com/posts/update-october-9-an-update-from-nikon-small-ugcPost-7506310017814401024-QfgZ/ | curl | HTTP 200 但被重定向到 linkedin.com/signup/cold-join（登录墙），非帖子正文。已删除重定向页；不存档，不登录，不用浏览器会话 | 无 |
| 10-11 16:00 | https://www.nikonsmallworld.com/news/nikon-small-world-in-motion-video-statement （官方声明，由 nikonsmallworld.com 首页链接找到） | curl | HTTP 200，正文完整，Posted on October 09, 2026（无时刻） | d-d7-nikon-statement.html、d-d7-nikon-statement.txt |
| 10-11 16:00 | https://www.nikonsmallworld.com/galleries/2026-small-world-in-motion-competition | curl | HTTP 200，获奖页（名次与作品）；页面显示 “The 2026 Competitions Are Now Closed” | d-d7-nikon-2026-comp.html、d-d7-nikon-2026-comp.txt |
| 10-11 16:00 | https://www.nikonsmallworld.com/galleries/small-world-in-motion | curl | HTTP 200，列出 Rules 链接 | d-d7-nikon-swim-gallery.html、d-d7-nikon-swim-gallery.txt |
| 10-11 16:00 | https://www.nikonsmallworld.com/galleries/swim-rules | curl | HTTP 200，官方规则正文（entry period 与 winner 日期见 evidence-d.md 的注记） | d-d7-nikon-swim-rules.html、d-d7-nikon-swim-rules.txt |
| 10-11 16:00 | https://www.nikonsmallworld.com/ | curl | HTTP 200，首页，用于找到声明与获奖页链接 | d-d7-nikon-home.html（未转文本） |
| 10-11 16:00 | https://enter.nikonsmallworld.com/small-world-in-motion/rules/ （Ars 外的规则页，原派工单指定） | curl | 连接失败（HTTP 000），无内容。已删除空文件；改用 swim-rules 页 | 无 |
| 10-11 15:53 | https://arstechnica.com/science/2026/10/winning-nikon-small-world-in-motion-video-disqualified-for-ai-use/ | curl | HTTP 200，正文完整，meta 发布 2026-10-09T18:44:06Z | d-d7-ars.html、d-d7-ars.txt |
| 10-11 15:54 | https://www.bbc.com/news/articles/ck4gjn1yzprno | curl | HTTP 200，正文完整，datePublished 2026-09-30T23:10:19Z | d-d7-bbc.html、d-d7-bbc.txt |
| 10-11 15:54 | https://developers.openai.com/api/docs/pricing | curl | HTTP 200，SSR 页面，含 Rosalind 价格表 | d-d8-pricing.html、d-d8-pricing.txt |
| 10-11 15:54 | https://platform.openai.com/docs/pricing | curl | 重定向至上一页，内容相同，已删除重复文件 | 无 |
| 10-11 16:03 | https://openai.com/index/introducing-gpt-rosalind/ | curl | HTTP 403，挑战页，非正文。已删除 | 无 |
| 10-11 16:03 | https://developers.openai.com/blog/rosalind-workbench | curl | HTTP 200，正文完整（无 discovery 字样） | d-d8-rosalind-workbench.html、d-d8-rosalind-workbench.txt |
| 10-11 约 16:05 | WebSearch：TUFA Labs ARC-AGI-2 88.06%；gpt-rosalind-discovery 定价；Nvidia 与 Reflection AI 的 FT 报道 | WebSearch | 仅得二手聚合与媒体页，未得一手；结果不存档，只作“未找到一手来源”的依据（见 evidence-d.md） | 无 |
| 10-11 16:01 | HN Algolia（d-heat.py，22 个查询） | python -I | 成功，结果 d-heat.json | d-heat.json（取数时刻 16:01 北京时间） |

## 脚本与中间文件

- d-html2txt.py：从 1009 期 d-html2txt.py 原样复制（只读原件未改）。
- d-heat.py：HN Algolia 热度查询，查询词按本期条目重写。
- d-d1-nadella.txt：由 fxtwitter JSON 的 article 字段生成，非 HTML 转换。
- d-d2-arc-post.txt：由 fxtwitter JSON 的 text 字段生成。

## 失败与限制

1. Reuters（401）与 FT（403）均为挑战页，无正文。B 类 FT 标题只能从 HN 帖标题推断，未经原页核实（见 evidence-d.md D3）。
2. LinkedIn 帖子重定向登录墙，Nikon 的 LinkedIn 更新未取得，也未通过浏览器登录取得。Nikon 官方声明以 nikonsmallworld.com 页面代替（一手，且与 Ars 引用的声明正文有一处措辞差异，见 evidence-d.md）。
3. Nikon 规则页 enter.nikonsmallworld.com 连接失败；改用 nikonsmallworld.com 的 swim-rules 页（同一规则体系，内容见存档）。
4. OpenAI 官方发布页 openai.com/index/introducing-gpt-rosalind 返回 403；未找到 gpt-rosalind-discovery 的官方发布说明。
5. Kaggle 排行榜为 JS 渲染，静态抓取无数据。ARC 公开榜页面同样未含 88.06 字样。
6. Cockroach Labs 页面的 “table above” 统计表在静态文本中缺失（可能为图片或未渲染内容），27 PR、55 次退回、9 次升级、2 次人类介入四项在文本中均未检出。
7. momo5502 与 Cockroach 页面未给出具体发布时刻（仅日期）；按规则写“官方未给时刻”。

## 隐私自查

全部抓取为匿名 curl，未用登录会话，未读取任何浏览器 Cookie。对 `d-*` 文件与本日志做了 grep（profile_image、avatar、handle 等）：JSON 中只出现原帖作者（Satya Nadella、ARC Prize）的公开头像与横幅链接，属于帖子作者而非抓取者，原样保留；未发现抓取者的头像、显示名或 handle。
