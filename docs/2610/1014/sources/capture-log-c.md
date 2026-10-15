# 1014 期 C 组抓取日志

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8），2026-10-15；括号为 UTC。工具均匿名、未登录；未使用浏览器已登录会话，未运行读取 Cookie 的工具，未发帖/点赞/评论/关注。opencli 浏览器扩展未连接，截图全部用 `barking shot render`（headless Chrome）。

## 一、抓取记录

| 北京时间 | 目标 | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 17:10 | https://sunshowers.io/posts/anthropic-mononoke/ | curl | HTTP 200，14.6 KB | c-sunshowers.html（仅本地）、c-sunshowers.txt、c-sunshowers.headers |
| 17:10 | https://sunshowers.io/images/scion-derived-data.png 、…/mononoke-derived-data.png | curl | HTTP 200，255 KB / 147 KB，PNG 1168×734、957×706 | c-img-scion-derived-data.png、c-img-mononoke-derived-data.png（仅本地） |
| 17:10 | https://www.anthropic.com/engineering/redesigning-source-control | curl | HTTP 200，162 KB，“Published Oct 13, 2026” | c-anthropic.html（仅本地）、c-anthropic.txt、c-anthropic.headers；与 1013 存档 d-d2-anthropic-scion.txt 逐行 diff：0 处差异 |
| 17:11 | https://hn.algolia.com/api/v1/items/50094694 | curl | HTTP 200，67 KB | c-hn-algolia-50094694.json、c-hn-algolia-50094694.txt（整理成文本，评论不含分数） |
| 17:11 | https://lobste.rs/s/plcgps/anthropic_s_scion_is_heavily_inspired_by 及 …/s/plcgps.json | curl | HTTP 200，60 KB / 39 KB | c-lobsters.html（仅本地）、c-lobsters.json |
| 17:12 | https://raw.githubusercontent.com/facebook/sapling/main/eden/mononoke/README.md ；…/main/LICENSE | curl | HTTP 200 | c-mononoke-readme.md、c-sapling-license.txt（GPL-2.0 全文，18 KB） |
| 17:12 | api.github.com：repos/facebook/sapling；contents/eden/mononoke；contents/eden/mononoke/derived_data（目录名列表，仅内联查看）；contents/eden/mononoke/docs | curl（匿名 API） | HTTP 200 | c-sapling-repo.json、c-mononoke-dir.json |
| 17:13 | raw：eden/mononoke/derived_data/src/lib.rs；API：contents/…/docs/2.3-derived-data.md、2.1-bonsai-data-model.md | curl | HTTP 200 | c-mononoke-derived-data-lib.rs、c-mononoke-doc-derived-data.md（17 KB）、c-mononoke-doc-bonsai.md |
| 17:13 | api.github.com：commits?path=…/derived_data/src/lib.rs（最后一页）、commits?path=…/docs/2.3-derived-data.md、commits?path=…/mononoke/README.md | curl | HTTP 200（结果仅内联查看，未存档）：lib.rs 可见最早提交 2020-02-06；2.3 文档最新提交 2025-12-02 | 无 |
| 17:14 | api.github.com/search/repositories：q=OpenScion、q=scion anthropic | curl | HTTP 200 | c-gh-search-openscion.json、c-gh-search-scion-anthropic.json |
| 17:14 | api.github.com/repos/{tinyhumansai/openscion, CarterT27/scion, bnsd55/open-scion, fenwii/openscion} 及其 /readme | curl | HTTP 200 | c-gh-readme-*.md（4 个）、c-gh-meta-*.json（4 个） |
| 17:15 | api.github.com/repos/facebook/sapling/commits?author=sunshowers | curl | HTTP 500 Server Error（返回体 135 B，已被后一文件覆盖），改用提交搜索接口 | 见下一行 |
| 17:15 | api.github.com/search/commits?q=repo:facebook/sapling+author:sunshowers（及 +mononoke）、api.github.com/users/sunshowers | curl | HTTP 200；总数 1286 / 含 mononoke 53 | c-sapling-commits-sunshowers.json、c-sapling-commits-sunshowers-mononoke.json（提交作者昵称与邮箱写盘后已替换为占位符）、c-gh-user-sunshowers.json |
| 17:16 | WebSearch ×2（“Anthropic Scion Mononoke plagiarism sunshowers response”、“Anthropic Scion Meta Mononoke copied 抄袭”） | WebSearch | 未见 Anthropic 回应；得到聚合站候选 | 无（仅线索） |
| 17:17 | firecrawl_search ×3（中文“抄袭/借鉴”两组、英文标题组，共约 4 积分） | firecrawl | 中文查询只命中 Scion 迁移本身的报道，未见 Mononoke 争议的中文报道；英文命中 unslop.day、deniz.in、readmex.ai 等聚合站 | 无 |
| 17:18 | https://ai-tldr.dev/releases/sunshowers-scion-mononoke/ 、https://simonwillison.net/2026/Oct/13/redesigning-source-control-at-anthropic/ 、https://unslop.day/en/ai/researchers-contact-tencent-agent-fleet/ 、https://deniz.in/anthropic-s-scion-derived-data-design-traced-back-to-meta-s-mononoke 、https://daily.dev/posts/anthropic-s-scion-is-heavily-inspired-by-mononoke-cjgzwv5qx | curl（浏览器 UA） | 全部 HTTP 200 | c-media-{aitldr,simonw,unslop,deniz,daily}.html（仅本地）与 .txt |
| 17:18 | https://hyper.ai/en/stories/…、https://www.archyde.com/…、https://aiweekly.co/ai-news-today | curl | hyper.ai 为 Next.js 错误页（无正文）；archyde.com HTTP 403 Cloudflare 挑战页，未绕过；aiweekly 页面与本组无关 | 三者的试验文件均已由我删除，未留存 |
| 17:21 | barking heat（--item 50094694；--hn 两个查询；--reddit 一个查询） | barking | HN：50094694 为 369 分 / 135 评论，Anthropic 原文帖 50073633 为 878 分 / 984 评论；Reddit 搜索被重定向到登录页（响应非 JSON），失败，未取 | c-heat.json、c-hn-50094694.json（HN 条目含评论的原始 JSON）；同名的 Algolia 手工存档已先改名为 c-hn-algolia-50094694.* 以免冲突 |
| 17:2x | opencli browser c-shot open（后台窗口） | opencli 1.8.8 | 90 秒无响应；`opencli doctor`：daemon 正常但 Browser Bridge 扩展未连接。未重试、未装扩展，改用 headless | 无 |
| 17:2x | barking shot render：sunshowers 文章（700×2600，DPR 2）、Anthropic 文章（两次：×3000、×3700）、HN 评论 item?id=50099795（700×900） | headless Chrome（barking） | 成功；第一次 Anthropic 渲染（3000 高）未含 Sapling 致谢段，改渲 3700 高 | 中间图在系统临时目录 barking-shot/26101014-c/，不入库；记录见 c-shots-log.jsonl |
| 17:2x | barking shot cropbox → images/25–29 | barking | 成功；其中 27 与 28 各有一次因裁切边落在文字行内而删除重裁（均为我自己刚生成的试验文件） | 见 c-shots-log.jsonl |
| 17:22 | 复抓 https://sunshowers.io/posts/anthropic-mononoke/ 与 Anthropic 原文（临时目录对比） | curl | 作者文章与 17:10 版字节一致；Anthropic 正文与 c-anthropic.txt 逐行一致 | 对比文件留在会话临时目录，未入库 |

## 二、失败与未取得

1. opencli 扩展未连接，已退回 headless（模板允许）；截图页面为浅色。
2. Reddit 搜索需登录，未取，未登录绕过。
3. GitHub `commits?author=` 接口一次 HTTP 500，改用提交搜索接口。
4. hyper.ai（错误页）、archyde.com（403 挑战页）无正文，未用。
5. 未打开作者引用的两条 AI 分享链接（claude.ai/share、chatgpt.com/share），未核其内容。
6. 未抓 Cursor 博客（作者脚注所引）与 Bluesky 帖。
7. 未追 facebook/sapling 中 `derived_data` 在 2020-02-06 目录迁移之前的更早历史。
8. HN/Lobsters 评论无法取得 HN 评论分数；Lobsters 评论分数取自其公开 JSON。
9. 中文媒体对 Mononoke 争议的报道：未找到。

## 三、版本与口径说明

- Lobsters 时间戳为 -05:00（CDT），已换算北京时间；HN Algolia 时间为 UTC。
- 本机时钟为美东，文件系统时间不作依据，抓取时刻以命令输出的 UTC 为准。
- 作者文章与 Anthropic 文章均只给日期，官方未给时刻。
- 存档里的提交作者昵称与邮箱已替换为占位符（只留公开账号名与职业身份）。
- 交付前已 grep：docs/2610/1014/ 本组文件中无抓取者头像、昵称或 handle 泄露（见回报）。
- 单个文件均小于 1 MB（最大为 c-anthropic.html 162 KB 与 c-img-scion-derived-data.png 255 KB，两者均仅本地）。
