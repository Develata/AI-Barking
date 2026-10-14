# 1013 抓取记录：C 组（Gemini 4 Argon 正式开放与 ADK 实验）

执行方：Claude Sonnet 5.5。全部时间为北京时间（UTC+8）。本机时区美东（EDT，UTC-4），北京 = 本机 + 12 小时。curl 抓取的时刻取自响应头 `date:`（UTC 转北京）；脚本记录取自 `c-browser-records.jsonl`；截图动作取自 `c-shots-log.jsonl`；其余为按操作顺序估计的约值（标“约”）。浏览器类抓取均为匿名 headless Chrome（Playwright Core，全新临时 profile，不登录、不带 Cookie；`c-vtext.mjs` 自 10/14 19:37 起加了桌面 Chrome UA 覆盖，其余同）；Reddit 用 `opencli reddit search/read`（只读）；GitHub 用 `gh api`（匿名公开数据）。

## 抓取

| 时间（北京） | URL | 工具 | 结果 | 文件 |
|---|---|---|---|---|
| 10/14 16:56:56 | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/ | curl（桌面 UA） | 200，1,032,640 字节；正文完整 | `c-announce.html/.headers`，`barking html2txt` → `c-announce.txt` |
| 16:57:07 | https://developers.googleblog.com/en/optimizing-a-200000-line-python-codebase-overnight-with-gemini-4-argon/ | curl | 200，54,243 字节；正文完整 | `c-adk.html/.headers/.txt` |
| 约 17:00 | `gh api repos/google/adk-python/commits/1754b3e…`、`…/pulls`、`releases`、`releases/tags/v2.12.0`、`compare`、`commits?path=.agents/skills`、`search/issues`、`search/commits` | gh | 成功；关联 PR 为 `[]`；搜索 “Argon” 仅命中 1754b3e | `c-gh-commit-1754b3e.json`、`c-gh-commit-1754b3e-pulls.json`、`c-gh-release-v2.12.0.json` |
| 17:00:20–17:01:43 | blog.google 9/30 首发页、`…/build-with-gemini-4-argon/`、`…/gemini-4-model-safety-security/`、developers.googleblog 机器人页、aistudio.google.com/docs/pricing 与 /long-decode（curl 只拿到 SPA 外壳） | curl | blog.google 与机器人页成功；aistudio 外壳页无用 | `c-launch-0930.*`、`c-build-dev.*`、`c-safety.*`、`c-robotics.*` |
| 17:01:28、17:02:30 | https://ai.google.dev/gemini-api/docs/pricing 、/models | curl | 失败：重定向超过 50 次（curl 47）；改用渲染浏览器 | 无文件（试验残留已删） |
| 17:02:38 | HN Algolia 与 Firebase（barking heat） | barking heat | 成功；Reddit 查询因 old.reddit 重定向到登录页失败（非 JSON），未绕过 | `c-heat-1702.json`（首次快照，重命名保留） |
| 17:03:28 | https://ai.google.dev/gemini-api/docs/pricing | `c-vtext.mjs` | 成功，渲染文本 96,518 字符 | `c-pricing.txt` |
| 17:03:53 | https://ai.google.dev/gemini-api/docs/models | `c-vtext.mjs` | 成功 | `c-models-doc.txt` |
| 17:04:09 | https://aistudio.google.com/docs/long-decode | `c-vtext.mjs` | 成功 | `c-aistudio-longdecode.txt` |
| 17:05:27 | https://ai.google.dev/gemini-api/docs/models/gemini-4-argon | `c-vtext.mjs` | 成功 | `c-model-page.txt` |
| 约 17:07–17:12 | AA 模型页（curl 超时，3.3 MB/4.0 MB 后中断，已删）、AA 文章列表 | curl | 模型页失败；文章列表成功（336 KB） | `c-aa-articles.html/.headers/.txt` |
| 17:12:25 | https://artificialanalysis.ai/models/gemini-4-argon | `c-vtext.mjs` | 成功 | `c-aa-model.txt` |
| 约 17:12 | https://artificialanalysis.ai/leaderboards/models、…/models/gemini-4-argon/providers | curl | 下载后未使用，已删（试验残留） | 无 |
| 17:13:50、17:14:13 | https://lmarena.ai/leaderboard/text 、/webdev | `c-vtext.mjs` | 失败：Cloudflare “Just a moment…”挑战页，未绕过 | `c-arena-text.txt`、`c-arena-webdev.txt`（挑战页文本，留作失败证据） |
| 约 17:15 | https://lmarena.ai/leaderboard/text | firecrawl_scrape | 失败：net::ERR_CONNECTION_CLOSED | 无 |
| 约 17:16 | https://arena.ai/leaderboard/text 、lmarena.ai/api/leaderboard | curl | 403（Cloudflare 页） | 无 |
| 约 17:18–17:20 | https://huggingface.co/datasets/lmarena-ai/leaderboard-dataset（API、README、`text/latest`、`text_style_control/latest`、`webdev/latest`、`search/latest`、`document/latest` parquet） | curl + uv/pyarrow（临时目录，parquet 未入库） | 成功；数据集 lastModified 2026-10-14T03:05:44Z | `c-arena-hf-dataset-api.json`、`c-arena-hf-extract.json`、`c-arena-hf-extract.py` |
| 17:22:35 | https://www.vals.ai/benchmarks/vals_index | `c-vtext.mjs` | 成功 | `c-vals-index.txt` |
| 17:23:07 | https://www.bloomberg.com/news/articles/2026-09-30/google-grapples-with-employee-skepticism-about-new-gemini-model | `c-vtext.mjs` | 失败：Bloomberg “Are you a robot?” 机器人验证页，未绕过 | `c-bloomberg-0930.txt`（验证页文本，留作失败证据） |
| 约 17:25 | “Google employees skepticism Gemini 4 Argon …”（WebSearch；firecrawl_search，消耗 4 点额度） | 搜索 | 找到 Yahoo Finance 的彭博通稿、日本时报转载等；搜索摘要不作证据，另行抓取全文 | 无 |
| 约 17:26 | Yahoo Finance https://finance.yahoo.com/technology/ai/articles/google-grapples-employee-skepticism-gemini-195242680.html ；日本时报 https://www.japantimes.co.jp/business/2026/10/01/tech/google-employee-skepticism-gemini-4/ | curl | 成功（日本时报静态抓取只有前半文） | `c-bb-yahoo.*`、`c-bb-japantimes.*` |
| 17:07:07–17:29:35 | blog.google 公告页渲染、点击“More Information”展开脚注、clip 脚注 | barking shot render；`c-expand-shot.mjs`（整页图拼接错乱，仅作试验，未用）；`c-clip-shot.mjs` | 成功 | 中间图在临时目录；成品 25、26、31 |
| 约 17:30–17:50 | adk 博文、GitHub commit 页、release 页、Vals、API 价格页渲染与裁切 | barking shot render/crop/cropbox/stack | 成功；逐张目视确认；见 `c-shots-log.jsonl`（21 条） | `../images/25`–`36` |
| 约 17:40 | `opencli reddit search`（r/singularity、Bard、LocalLLaMA、GeminiAI × 4 个查询，top/week）、`opencli reddit read` 三帖 | opencli | 成功；r/LocalLLaMA 无相关帖入围 | `c-reddit-search.json`、`c-reddit-1x51smr.json`、`c-reddit-1x53mtp.json`、`c-reddit-1x56lmf.json` |
| 约 17:40 | https://api.fxtwitter.com/status/2110038940841545827 | curl | 成功 | `c-x-logank.json` |
| 约 18:00 | https://github.com/google/adk-python 浅克隆（main、commit 1754b3e），统计行数与测试数 | git（临时目录，不入库） | 成功：src 783 文件 / 205,829 行（commit 时点） | 结果写入 evidence-c.md |
| 约 18:05 | https://pypi.org/pypi/google-adk/json | curl | 成功；已精简为 2.x 版本上传时刻 | `c-pypi-google-adk.json` |
| 约 18:10 | 搜索：ADK 夸大标题（WebSearch ×3、firecrawl_search ×2，消耗 6 点额度） | 搜索 | 找到 VentureBeat、36氪、Ars、BI、9to5Google 等 | 无 |
| 约 18:15 | Ars Technica、Business Insider、9to5Google、blog.google Gemini 应用页 | curl | 成功 | `c-m-arstechnica.*`、`c-m-businessinsider.*`、`c-m-9to5google.*`、`c-m-geminiapp.*` |
| 约 18:15 | https://pasqualepillitteri.it/…（搜索结果转载站） | curl | 失败（schannel 连接中断），未再尝试，不属一手 | 无 |
| 19:35:37 | https://m.36kr.com/p/4024861209890695 | `c-vtext.mjs` | 第一次失败（页面跳转，execution context destroyed）；19:36:12 重试成功 | `c-m-36kr.txt` |
| 19:35:54 | VentureBeat 文章 | `c-vtext.mjs`（curl 抓到的版本只有 215 字节，已替换） | 成功 | `c-m-venturebeat.txt` |
| 19:37:03 | https://aihot.news/hot 、/ | `c-vtext.mjs` | 第一次返回 `{"error":"blocked","code":567}`；加桌面 UA 后 19:37:42/19:37:56 成功；B 组当事人姓名已替换为 “[当事人]” | `c-aihot-hot.txt`、`c-aihot-home.txt` |
| 19:54:14 | HN Algolia/Firebase（barking heat，第二次） | barking heat | 成功（旧 `c-heat.json` 改名为 `c-heat-1702.json`，旧 `c-hn-50073879/50074869.json` 已删后重取） | `c-heat.json`、`c-hn-*.json` |
| 19:54:32 | https://antigravity.google/blog/gemini-4-argon-in-google-antigravity | `c-vtext.mjs` | 成功 | `c-antigravity-blog.txt` |

## 说明与自检

- 删除的试验残留（均为本组刚生成的文件）：`c-pricing.headers`、`c-models-doc.headers`（curl 重定向循环失败）、`c-aa-model.html/.headers`（超时半截）、`c-aa-index.*`、`c-aa-providers.html`、`c-aistudio-pricing.*`、`c-longdecode.*`（SPA 外壳）、`c-m-36kr.*`、`c-m-venturebeat.*` 的首次 curl 版本、`c-m-pillitteri.headers`；`images/` 下临时文件 `tmp-c-foot.png`、29 与 33 号图首次裁切（边界落在文字行内）重做。
- 截图方式：匿名 headless Chrome（barking shot，2 倍像素比）。派工中途收到“截图改用 opencli 浏览器”的新要求时，25–36 号图已全部完成，未重做；未用到登录态浏览器，GitHub 页右上角为匿名的 Sign in / Sign up，页面内可见的 GWeale 头像与名字是公开提交者。
- 图 26、35 为 barking `stack` 拼接（多段竖向拼接，只裁切不改像素）；图 36 裁掉了价格页左侧导航栏。
- 图 30、34 的页面时间元素（GitHub 相对时间控件）在静态渲染里是空白占位，提交与发版时间以 `c-gh-*.json` 为准。
- 抓取者信息自检：对 `docs/2610/1013/` 下 C 组文件 grep 本机用户名、邮箱、GitHub 账号名，均无命中（`capture-log-d.md` 不属本组）；Reddit JSON 中只有公开发帖用户的网名；`c-x-logank.json` 为谷歌员工官方账号的公开帖。
- 单个文本文件均小于 1 MB（最大 `c-hn-50073879.json` 约 444 KB）；HTML 原件（`c-announce.html` 约 1.0 MB 等）只存本地，不入库。
- 点击：仅点击了 blog.google 页面的“More Information”折叠按钮以展开脚注，未点击其他控件，未登录，未接受 Cookie，未发帖、点赞或评论。
