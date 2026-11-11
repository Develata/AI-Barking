# 1110 期 A 组抓取日志（Gemini 个人账户关联应用数据的人工审核）

执行：Claude Sonnet 5.5。时间为北京时间（UTC+8，本机时区即北京），日期均为 2026-11-11（Wayback 时间戳为 UTC，另注）。文件前缀 `a-`。`barking fetch` 的逐条记录在 `a-fetch-log.jsonl`，`barking shot` 的记录在 `a-shots-log.jsonl`，HN 热度在 `a-heat.json`、`a-hn-50429426.json`。未登录任何账户，未连接 Gemini，未做自测复现；opencli 扩展未连接，没有使用。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 14:30 | support.google.com/gemini/answer/16836988（帮助页）；blog.google 2023 博文；github.com/thisisme6598/gemini_files（页面、raw README、commits API、repo API）；knowledge.workspace.google.com Workspace Privacy Hub；support.google.com/gemini/answer/13594961（Gemini Apps Privacy Hub）；policies.google.com/terms/gemini-apps | `barking fetch`（`a-list1.tsv`） | 8 条 ok；**policies.google.com/terms/gemini-apps 404（failed，URL 猜错，未再试）**；隐私通知正文在 13594961 内（“Gemini Apps Privacy Notice”节，页内标注 Last updated: October 13, 2026） |
| 14:31 | web.archive.org 2026-10-12 04:46:31 UTC、10-17 00:45:12 UTC、11-10 00:43:17 UTC 三个博文快照；support.google.com/websearch/answer/16859283；gemini/answer/13695044、16598406、13278892 | `barking fetch`（`a-list2.tsv`） | 7 条 ok；Wayback 快照只用于查看“当时页面写了什么”，已看 `.txt` 是博文正文 |
| 14:32 | web.archive.org CDX 索引：blog.google 博文、help 16836988、websearch 16859283、gemini 13594961 | curl（`--max-time 60`） | 取得快照时间表；用于选快照 |
| 14:33 | Wayback：help 16836988 在 2026-01-14 21:36:56 UTC 等 10 个快照；13594961 在 2025-04 至 2026-10 的 6 个快照；websearch 16859283 的 2026-01-22、2026-10-11 快照 | curl（`--compressed`，存到临时目录，仅检索比对；其中 2026-01-14、2026-10-19 的 help 与 2026-01-22 的 search 快照复制为 `a-wb-help-20260114.html/.txt`、`a-wb-help-20261019.*`、`a-wb-search-20260122.*`） | 全部 200；Wayback 的 help 快照 gzip 压缩，已解压后比对；注意 Wayback 对 2026-09-14 websearch 快照返回空（未用） |
| 14:34 | 仓库各提交的 README（a7d60c90、8df681a8、9dad8ce8、dfb9f9a4、6601fb36）与 commit API（dfb9f9a4、6601fb36、9dad8ce8） | `barking fetch`（`a-list3.tsv`） | 8 条 ok |
| 14:34 | HN 50429426 | `barking heat --item` | 717 分 / 147 评，11-10 02:24:38（北京）提交；HN Algolia 不提供评论分数，评论只能按楼内排序与回复数摘录；二次运行 `heat` 因文件已存在被拒（原文件保留） |
| 14:35 | cnBeta、鏈新聞 ABMedia、AI之上（00011000.com）、AI Weekly（FR 站英文页）、Techpresso、StartupHub、help 16836988 中文版（`?hl=zh-Hans`） | `barking fetch`（`a-list4.tsv`） | 7 条 ok |
| 14:36 | exa 搜索（英文、中文，共 3 次） | mcp exa `web_search_exa` | 仅作线索与定位（报道发现、Google 回应搜索）；引文都回到 `.txt` 存档核对 |
| 14:47 | The Register（2025-11-07，Deep Research 一文）、sysnote、digiton、HN 评论 50432666 页 | `barking fetch`（`a-list5.tsv`） | 4 条 ok |
| 14:50 | blog.google Personal Intelligence 发布博文（2026-01-14） | `barking fetch`（`a-list6.tsv`） | ok；全文 0 处 “human” |

被拒或失败的请求：policies.google.com/terms/gemini-apps（404）。无 blocked 条目。

自写脚本（`docs/2611/1110/sources/`）：`a-cdp-shot.mjs`——headless Chrome + CDP，打开页面、执行展开折叠区的 JS（点击 `a.zippy` / `[aria-expanded=false]`）、按 CSS 像素区间直接截图（`--clips`，2 倍像素）；只展开折叠、隐藏 Wayback 工具栏，不改正文。因 `barking shot render` 无法展开 Google 帮助页的折叠问答，故用它出中间图，再用 `barking shot stack/cropbox` 裁出成品（成品动作都记入 `a-shots-log.jsonl`）。中间图放在会话临时目录，不入库。

## 隐私处理

- `a-gh-commits.json` 的 Initial commit（a7d60c90）作者邮箱原文是仓库作者的个人邮箱，已在存档里替换为 `[EMAIL-REDACTED]`（其余提交为 GitHub noreply 地址，保留）。
- 仓库作者只记 GitHub 公开用户名（即仓库 URL 里的名字，匿名账号）与公开提交；未收集其他信息。
- 截图均未登录；GitHub 页面为匿名访问，页面左上角只有仓库名与 About 栏。无抓取者信息。

## 截图

浏览器：Chrome headless（临时 user-data-dir，未登录），CSS 宽 700（GitHub 页用 700），DPR 2，输出宽 1400 px；均为浅色底。用 `a-cdp-shot.mjs` 按布局坐标区间截图，再用 `barking shot stack|cropbox` 裁切、拼接（不改像素）。拼接处说明如下。

| 文件 | 来源页 | 内容 | 备注 |
|---|---|---|---|
| 01-help-human-review.png | 帮助页 16836988（?hl=en，展开“Will human reviewers have access to my data?”） | 页标题 + 该问答整段：“How human review helps…”“subset … reviewed by human reviewers”“Important: Please don't connect…”“What data is included in human review”（含“summary may just be the email or file”）“How we protect your privacy in this process”（含“disconnecting interactions from your Google Account and reducing personal information”） | 标题与正文两段拼接，中间略去页面其他内容；展开的问答在原页是折叠态 |
| 02-help-dont-connect.png | 同上 | 页标题 + 问答前半：“subset”句与“Important: Please don't connect your apps…”警示句 | 同上拼接 |
| 03-help-human-reviewed-per-notice.png | 同上 | 页标题 + “How your data is used”节，含“Data from Connected Apps is used and human reviewed per the Gemini Apps Privacy Notice.” | 同上拼接 |
| 04-help-keep-activity-train.png | 同上（展开“Do you train AI models on my personal data?”） | 页标题 + “…if your Keep Activity setting is on.”“Gemini does not train generative AI models directly on…”“the summary may just be the email or file.” | 同上拼接 |
| 05-help-eligibility.png | 同上 | 页标题 + 首段 “Eligible users with Personal Intelligence can connect…” | 页面没有“最后更新”日期（HTML 里只有页脚通用的 lastUpdated 模板，无页面日期） |
| 06-blog-promise-and-update.png | blog.google 2023 博文（现网） | 标题、日期（Oct 12, 2023）、作者与职务（Laurie Richardson, VP Trust & Safety）+ 含“not seen by human reviewers”原句与 “(Update, October 16, 2026: …)”全文的两段 | 标题段与正文段拼接，中间略去页面其他段落 |
| 07-wayback-blog-1012-before-update.png | web.archive.org 2026-10-12 04:46:31 UTC 的博文快照 | 同一页同位置：原句在，**无**更新说明 | **是快照**；Wayback 工具栏已用 CSS 隐藏；拼接同上 |
| 08-hub-human-review-chats.png | Gemini Apps Privacy Hub（13594961，展开全部） | “Last updated: October 26, 2026”+“Who has access to my chats, and how does human review improve Gemini for everyone?”整段：“A subset of chats are reviewed…”“Reviewed chats are retained for up to three years.”“Even if your Keep Activity setting is off…human reviewers” | 页标题与问答拼接 |
| 09-hub-retention-three-years.png | 同上 | “How long we retain your data”：18 个月默认自动删除；“Chats reviewed by human reviewers … retained for up to three years.” | 页标题与段落拼接；该节标 Last updated 为页面顶部的 October 26, 2026（隐私通知节另标 October 13, 2026） |
| 10-hub-keepactivity-off.png | 同上 | “If the Keep Activity setting is off:”：不用于训练（除非发反馈）、仍保存 72 小时、已审核对话保留 3 年、“Some Connected Apps are unavailable when this setting is off.” | 页标题与段落拼接 |
| 11-readme-title-clarification.png | github.com/thisisme6598/gemini_files（匿名访问，2026-11-11 14:4x） | 仓库头（33 stars、3 forks、About：“Documentation of Gemini File Leaking Behavior”、最新提交 6601fb3）+ README 标题、11/10 范围说明、“quietly reversed”段、“Crucially…probabilistic”段 | 窄屏版式（700 CSS px，GitHub 把侧栏折到上方）；“18 hours ago”是抓取时的相对时间 |
| 12-workspace-hub-not-human-reviewed.png | Workspace Privacy Hub（?hl=en，展开问答） | “Last updated: October 6, 2026”、覆盖范围、“Your content is not human reviewed or otherwise used for Generative AI model training outside your domain without permission.” + 展开的“How do Google Workspace apps in the Gemini app work?”（“Your chats and uploaded files in Gemini app won't be reviewed by human reviewers…”） | **两段拼接**：上段为页首到“Your privacy is our priority”三条，下段为“How do Google Workspace apps in the Gemini app work?”问答，中间的大段页面内容（数据保留、训练问答等）略去；上段没有页标题行，从 “Last updated: October 6, 2026” 起 |
| 12b-hub-connected-apps-keepactivity.png | Gemini Apps Privacy Hub（13594961） | “How to manage your Connected Apps”：“…other Connected Apps like Google Workspace are unavailable if you turn off Keep Activity.” | 单段 |
| 12c-hub-72h-feedback-connected-apps.png | 同上 | “How long does Google retain my temporary chats and chats I have when Keep Activity is off”：72 小时；发反馈时上传“data from Connected Apps” | 单段（页内问答，前后相邻问题标题被裁） |
| 12d-workspace-hub-personal-intelligence-no.png | Workspace Privacy Hub | “Is Personal Intelligence available for Google Workspace Business, Enterprise, or Education accounts?” “No. …” | 单段 |
| 12e-wayback-help-20260114-human-review.png | web.archive.org 2026-01-14 21:36:56 UTC（北京 01-15 05:36:56）的 16836988 快照 | 页首句 + 展开的人工审核问答：与现网文字相同的“subset”“Important: Please don't connect…”“summary may just be the email or file” | **是快照**；页标题行因版式在快照中位移未入图，图首为页面首句；Wayback 工具栏已隐藏 |
| 12f-help-personal-intelligence-requirements.png | 帮助页 16598406（Personal Intelligence） | “What you need”：18 岁以上、个人 Google 账号（工作/学校/受监管账号不可用）、Keep Activity 必须开启、Smart features 开启；“Not available in the European Economic Area, Nigeria, Switzerland, or the United Kingdom.” | 单段 |

## 目视确认

01–12、12b–12f 全部逐张打开看过：文字清楚，范围正确，裁切边界落在文字行之间（06 曾因起点切到一行文字，重截后正确，废弃的试验图已移出 `images/`，不在仓库内）。07、12e 是 Wayback 快照，12 为两段拼接，均在上表注明。
