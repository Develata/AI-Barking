# 1004 抓取日志

范围：仅本期目录；未提交、推送或删除文件。四组 Codex 执行方（gpt-6-luna max）各自的日志逐字合并如下；Claude 的补充抓取见末尾。时间为北京时间。

---

## A 组：OpenAI 安全线（Robinson 辞职与三份事件报告）（原文）

# 1004 期 A 组抓取日志

所有时间均为北京时间（UTC+8）；链接已去除 UTM 参数。页面快照、脚本和记录都以 `a-` 前缀保存在本目录。浏览器使用独立 session `1004-a`。未登录、未输入凭据，未点赞、评论或关注。X 与 Reddit 的 agent-reach doctor 未确认可用的已登录 API 后端；通过 OpenCLI 读取公开页面/帖文。社交数据按 L6 线索处理。

## 页面抓取

| 北京时间 | URL | 工具与结果 |
|---|---|---|
| 10-04 20:59:23 | https://alignment.openai.com/misalignment-reports/ | `opencli browser 1004-a open/eval`；正文 2,950 字符。索引说明报告日期意为最后更新，三条报告顺序与日期已核；写入 `a-reports-index.json/.txt`。 |
| 10-04 20:59:26 | https://alignment.openai.com/misalignment-reports/command-injecting-a-reference-tool-to-copy-a-source-file/ | OpenCLI 浏览器正文 10,589 字符；全文文字快照 `a-report-perl.json/.txt`。 |
| 10-04 20:59:33 | https://alignment.openai.com/misalignment-reports/reaching-an-internal-eda-host-through-a-reference-tool/ | OpenCLI 浏览器正文 14,311 字符；全文文字快照 `a-report-eda.json/.txt`。 |
| 10-04 20:59:37 | https://alignment.openai.com/misalignment-reports/preparing-for-a-restart-after-reading-slack/ | OpenCLI 浏览器正文 8,346 字符；全文文字快照 `a-report-slack.json/.txt`。 |
| 10-04 20:59:43 | https://www.theatlantic.com/technology/2026/10/openai-safety-team-resignation/688881/ | OpenCLI 浏览器正文 3,331 字符；标题、作者、发布时间、开头、作者简介可见，订阅墙挡住后文。记为部分成功，不尝试绕过。页面快照自动采集的导航链接曾含 Atlantic 通用 gift 入口；发现后已从 `a-atlantic.json` 删除。没有打开或使用任何文章 gift/token 链接。 |
| 10-04 21:00:04 | https://www.reuters.com/legal/litigation/openai-safety-employee-quits-says-time-trial-error-is-over-2026-10-03/ | OpenCLI 浏览器正文 6,054 字符；发稿时刻、12 次 frontier-model launches、安全回应可见。写入 `a-reuters.json/.txt`。 |
| 10-04 21:23:29 | https://x.com/Marcus_J_W/status/2106203042140102868 | `opencli twitter thread 2106203042140102868 -f json` 读取到原帖和线程；仅保留主帖必要字段至 `a-marcus-post.json`，未保存回复。公开页面卡片用 OpenCLI+CDP 截图，截图裁切仅含主帖卡片。CLI 抓取时帖文显示 1,118 likes；截图稍早显示 1,117 likes，互动数动态变化，不用作事实判断。 |
| 10-04 21:24:35 | https://aihot.news/daily/2026-10-03 | OpenCLI 浏览器正文 3,447 字符；存档 `a-aihot-daily-20261003.json/.txt`。包含加州传票报道，未找到 Robinson 辞职条目。 |
| 10-04 21:24:47 | https://aihot.news/daily/2026-10-04 | OpenCLI 浏览器正文 1,547 字符；存档 `a-aihot-daily-20261004.json/.txt`。未找到 Robinson 辞职条目。 |
| 10-04 21:24:53 | https://aihot.news/items/w0twto4412g72n2ryamc6xpi1 | OpenCLI 浏览器正文 5,513 字符；条目显示发布 `2026-10-02 08:00`、AI 评分 69；存档 `a-aihot-perl.json/.txt`。AIHOT 为 L5 热度/转述来源。 |
| 10-04 21:24:57 | https://aihot.news/items/oqxyqwdulf3zj8plv1ckcl4cj | OpenCLI 浏览器正文 970 字符；条目显示发布 `2026-10-04 10:18`、AI 评分 81；存档 `a-aihot-slack.json/.txt`。 |
| 10-04 21:25:02 | https://techcrunch.com/2026/10/03/openai-safety-employee-resigns-claiming-the-companys-culture-is-broken/ | OpenCLI 浏览器正文 6,605 字符；显示 10-03 9:30 AM PDT；可见第三方评测/实时监控回应及“hired a PR firm”但未命名 Spitfire。存档 `a-techcrunch.json/.txt`。 |
| 10-04 21:25:21 | https://www.theguardian.com/technology/2026/oct/03/openai-safety-leader-quits-warning-ai-companys-culture-is-broken | OpenCLI 浏览器正文 7,724 字符；显示 10-03 3:41 PM EDT；存档 `a-guardian.json/.txt`。 |
| 10-04 21:25:35 | https://www.theverge.com/ai-artificial-intelligence/1004408/openai-safety-quits-sounding-the-alarm | OpenCLI 浏览器取得标题，但正文 0 字符。存空 `a-verge.txt` 仅记录失败结果，不据此摘录正文或发布时间。 |
| 10-04 21:25:54 | https://cadenaser.com/nacional/2026/10/04/dimite-el-jefe-de-seguridad-de-openai-tras-denunciar-la-falta-de-control-en-la-ia-de-la-empresa-cadena-ser/ | OpenCLI 浏览器正文 7,293 字符；显示 10-04 13:18 CEST；存档 `a-ser.json/.txt`。 |
| 10-04 21:26:13 | https://www.livemint.com/ai/artificial-intelligence/who-is-david-robinson-safety-systems-team-lead-at-openai-joining-the-list-of-execs-who-left-the-ai-firm-this-year-11791007693908.html | OpenCLI 浏览器正文 4,854 字符；显示 10-03 11:48 AM IST；存档 `a-livemint.json/.txt`。 |

## 社区计数与评论

| 北京时间 | URL | 工具与结果 |
|---|---|---|
| 约 10-04 21:36 | https://www.reddit.com/r/technology/comments/1ww9hsr/openai_safety_leader_david_robinson_resigns_as/ | OpenCLI 浏览器读取公开 DOM；帖文 110 分/11 评论。评论可见分数；最高分 49 是谐音玩笑，15 分一条泛称公司不负责任。未存整页或头像/作者个人资料。 |
| 约 10-04 21:36 | https://www.reddit.com/r/OpenAI/comments/1wwu3eq/openai_pauses_frontier_training_after_ai_agents/ | OpenCLI 浏览器读取公开 DOM；混合旧事件与 Robinson 辞职的帖子 9 分/31 评论。选取一条 5 分时间线质疑作旁证；不是 Robinson 署名文主帖下的评论。只保存引用所需的短摘录和评论链接。 |
| 10-04 21:42:33 | https://news.ycombinator.com/item?id=49944227 | OpenCLI 浏览器页面；显示 302 points/545 comments。此数为该时点现场值；约 21:0x 的较早网页抓取为 284/527，故热度随时间变化。 |
| 10-04 21:42:38 | https://news.ycombinator.com/item?id=49948332 | OpenCLI 浏览器页面；Guardian 重复帖显示 265 points/3 comments；讨论已指回 Atlantic 主帖。 |
| 10-04 21:42:45 | https://hacker-news.firebaseio.com/v0/item/49951850.json | 官方 HN Firebase API 返回评论文本和 parent/id，但没有 `score` 字段；另一个被引用评论也未见单条分数。故 HN 评论只记录论点与链接，不伪造评论分数。 |

## 检索失败与限制

- `opencli twitter search` 的 `from:OpenAI misalignment reports` 只找到 9 月 16 日旧帖；精确日期范围 `from:OpenAI since:2026-10-02 until:2026-10-04` 返回 HTTP 429（约 10-04 21:20 BJT）。因此官方账号 10 月 2 日是否发帖标为未找到，不能由限流推断不存在。
- AIHOT `David Robinson` 搜索路由返回 404；改查 10-03/10-04 日报和已定位的两个条目，仍未找到辞职专条。
- The Verge 页面标题可访问、正文 0 字符；记为失败，不用搜索摘要代替原站正文。
- `web.run open` 对 Atlantic 原文与 HN Firebase JSON 返回不可访问；已通过 OpenCLI 浏览器读取 Atlantic 公开可见部分、通过官方 HN 页面/API取得需要的字段。Atlantic 订阅墙未绕过。
- 首次 Slack 截图的锚点选择失败，首次 X 卡片选择器未命中；调整锚点/以帖子卡片 DOM 为目标后重抓。最终 08、09 图片均检查成功。HN 与 API 的一次并行浏览器查询发生会话页竞争；最终 HN 计数在 21:42 使用单一 session 顺序重读，表中采用该次结果。

## 截图记录

浏览器 CSS 宽度为 700、`deviceScaleFactor=2`；因此一般页面 PNG 宽 1400px。X 帖子卡片宽 596 CSS px，PNG 为 1192px，仍为 2 倍。所有截图均在 01–09 区间、宽度 ≤1400 px。

| 文件 | 北京时间（文件写入） | 截图内容 |
|---|---|---|
| [01-atlantic-opening.png](../images/01-atlantic-opening.png) | 21:19:07 | 标题、署名、时间、开头职责句和订阅墙提示。 |
| [02-reuters-response.png](../images/02-reuters-response.png) | 21:19:22 | Reuters 标题/发布时间、12 次发布职责、OpenAI 回应及全文可见区域。 |
| [03-reports-index.png](../images/03-reports-index.png) | 21:19:33 | 官方索引完整列表和三份报告的事件/更新日期。 |
| [04-perl-report.png](../images/04-perl-report.png) | 21:19:40 | Perl 报告题头、分类、日期、RL 任务摘要、工具执行和文件大小/行数关键段。 |
| [05-eda-summary.png](../images/05-eda-summary.png) | 21:19:49 | EDA 报告题头、事件日期、评测环境和未取得答案的摘要。 |
| [06-eda-id-attempt.png](../images/06-eda-id-attempt.png) | 21:20:01 | `id` 首次成功命令段及未取得 grader expected answers 的后续段落。 |
| [07-slack-summary.png](../images/07-slack-summary.png) | 21:20:09 | Slack 报告题头、日期、摘要、模型得到 key 后的操作范围。 |
| [08-slack-cot-response.png](../images/08-slack-cot-response.png) | 21:20:16 | “we may die!” 上下文、外部 job 的考虑与没有执行的处置过程。 |
| [09-marcus-post.png](../images/09-marcus-post.png) | 21:23:43 | X 主帖卡片本身；未包含侧栏、回复或撰稿者账号信息。 |

文件尺寸由 PowerShell System.Drawing 读取：01–08 均宽 1400px；09 为 1192px；均为 2 倍像素比。所有 A 截图均已目视检查，无页面外账号信息泄露。未提交、未推送、未删除文件；未运行测试（本任务为取证，不要求测试）。

---

## B 组：Gemini 降档（原文）

# B 组抓取日志（session：1004-b）

工作目录：`E:\gitclone\AI-Barking`。只写 B 组 `b-` 前缀源档、`evidence-b.md`、本日志，以及本组图片 10–14；未 commit、未 push、未删除文件。所有时间为北京时间（UTC+08:00），Wayback URL 中的 UTC 时间另行标出。

## 浏览器归档

正文通过 OpenCLI Browser 可见文本和公开表格 DOM 采集；不存页面隐藏 meta 或账户配置。脚本：[b-capture.mjs](b-capture.mjs)。

| 时间（北京时间） | URL / 页面 | 结果文件 | 结果 |
|---|---|---|---|
| 2026-10-04 21:27:58 | [Google Gemini Apps Help：Changes to Gemini model access and limits](https://support.google.com/gemini/answer/17004136?hl=en) | [b-gemini-help-current.txt](b-gemini-help-current.txt) | 全部可见正文及 10 月模型表、5 月用量表的公开 DOM 格子已存档。 |
| 2026-10-04 21:34:11 | [Google Gemini Apps Help：Gemini Apps limits & upgrades](https://support.google.com/gemini/answer/16275805?hl=en) | [b-gemini-limits-current.txt](b-gemini-limits-current.txt) | 全部可见正文；记录其 Gemini 3 Model Access 表四行全 Yes。 |
| 2026-10-04 21:34:40 | [目标页 Wayback：2026-09-30 07:21:20 UTC](https://web.archive.org/web/20260930072120/https://support.google.com/gemini/answer/17004136?hl=en) | [b-gemini-help-prechange-20260930.txt](b-gemini-help-prechange-20260930.txt) | 同一目标页旧快照；仅见 May / July 限额改动，未见 10 月段落与新访问表。 |
| 2026-10-04 21:35:07 | [套餐帮助页 Wayback：2026-09-30 07:16:48 UTC](https://web.archive.org/web/20260930071648/https://support.google.com/gemini/answer/16275805?hl=en) | [b-gemini-limits-prechange-20260930.txt](b-gemini-limits-prechange-20260930.txt) | 另一篇帮助页旧快照；Gemini 3 Flash-lite / Flash / Pro 的四计划行均为 Yes。 |
| 2026-10-04 21:36:42 | [Google Gemini Apps work / school / Education Help](https://support.google.com/gemini/answer/14620100?co=DASHER._Family%3DEducation&hl=en) | [b-gemini-workspace-education.txt](b-gemini-workspace-education.txt) | 存档 Workspace license 范围、Education edition 表及各自限额说明。 |
| 2026-10-04 21:37:03 | [Google Workspace Help：About AI usage limits](https://knowledge.workspace.google.com/admin/generative-ai/workspace-with-gemini/about-ai-usage-limits?hl=zh-cn) | [b-gemini-workspace-ai-limits.txt](b-gemini-workspace-ai-limits.txt) | 首次尝试为中文界面。 |
| 2026-10-04 21:37:28 | [同一 Workspace Help 英文页](https://knowledge.workspace.google.com/admin/generative-ai/workspace-with-gemini/about-ai-usage-limits?hl=en) | [b-gemini-workspace-ai-limits.txt](b-gemini-workspace-ai-limits.txt) | 同名文件更新为英文完整正文及表格；最终交付以英文版本为准。 |
| 2026-10-04 21:37:52 | [Google AI for Developers：Gemini API Billing](https://ai.google.dev/gemini-api/docs/billing?hl=en) | [b-gemini-api-billing.txt](b-gemini-api-billing.txt) | 存档 Gemini API 计费 / tier / AI Studio usage 说明。 |
| 2026-10-04 22:19:55 | [9to5Google 报道](https://9to5google.com/2026/10/03/gemini-model-limits-oct-26/) | [b-9to5google.txt](b-9to5google.txt) | 存档报道正文及可见发布时间。 |
| 2026-10-04 22:20:21 | [Pasquale Pillitteri 报道](https://pasqualepillitteri.it/en/news/20384/gemini-flash-pro-cut-free-ai-plus-oct-9) | [b-pasquale.txt](b-pasquale.txt) | 存档标题、页面日期、正文内 Plus 邮件限定。 |
| 2026-10-04 22:20:39 | [Superpower Daily 报道](https://superpowerdaily.com/posts/google-cuts-gemini-model-access-for-free-users-and-ai-plus-subscribers) | [b-superpowerdaily.txt](b-superpowerdaily.txt) | 存档标题、副标题、正文及可见发布时间。 |

## 图片截取与检查

脚本：[b-shots.mjs](b-shots.mjs)。OpenCLI Browser 使用 `1004-b`；通过 CDP 以 DPR 2 截取公开页面相关区域。五张图片均为 1400 physical px 宽（700 CSS px），生成后逐张目视检查，图 14 的裁剪不含旧 Reddit 侧栏。

| 时间（北京时间） | 来源 | 图片 | 内容与像素 |
|---|---|---|---|
| 2026-10-04 22:22:33 | [Gemini Apps Help 当前页](https://support.google.com/gemini/answer/17004136?hl=en) | [10-gemini-access-table.png](../images/10-gemini-access-table.png) | 标题、个人账号与日期句、完整模型表；1400 × 1254。 |
| 2026-10-04 22:22:33 | 同上 | [11-gemini-rollout-scope.png](../images/11-gemini-rollout-scope.png) | 标题、个人账号与日期句、下一表头；1400 × 914。 |
| 2026-10-04 22:22:33 | 同上 | [12-gemini-usage-limits.png](../images/12-gemini-usage-limits.png) | May / July 句、compute-based 原文、完整 premium 列表与 Usage limits 表；1400 × 1448。 |
| 2026-10-04 22:22:58 | [Wayback 套餐帮助页 2026-09-30 07:16:48 UTC](https://web.archive.org/web/20260930071648/https://support.google.com/gemini/answer/16275805?hl=en) | [13-gemini-old-model-access.png](../images/13-gemini-old-model-access.png) | 完整旧版 Gemini 3 Model Access 表；1400 × 492。它属于 answer/16275805，不是目标 answer/17004136 的旧版本。 |
| 2026-10-04 22:23:13 | [Reddit `r/GoogleGeminiAI` 原页（old.reddit.com）](https://old.reddit.com/r/GoogleGeminiAI/comments/1wwimbm/gemini_flash_and_pro_will_only_be_available_by/) | [14-reddit-access-claim.png](../images/14-reddit-access-claim.png) | 原帖标题、日期、214 points、173 comments 与正文误述；1400 × 1320。 |

## 社区与聚合索引抓取

- 2026-10-04 22:12 左右：对 AIHOT 搜索索引检索“谷歌 Gemini 应用 10 月 9 日起未订阅用户仅可使用 Flash-Lite 模型”。索引结果列出 10 月 3 日同题项，并分别出现 `Hacker News 热门 · 13:57 33`、`IT之家 · 12:58 66` 的卡片行。它们是搜索结果片段，关联到目标条目及计数语义都未能在实时页面确认，故未作为已核实的 AIHOT 分数。
- 2026-10-04 22:24 左右：OpenCLI Reddit 定向搜索 `Gemini model access October 9`，分别限定 `r/GeminiAI` 与 `r/Bard`，排序 hot / week。读数：`r/GeminiAI` 公告转帖 494 points / 141 comments；`r/Bard` 同名转帖 107 / 44。两帖正文均保留 AI Plus 邮件句。
- 2026-10-04 22:23:13：图 14 直接读旧 Reddit 页面计数，为 `r/GoogleGeminiAI` 误述帖 214 points / 173 comments。页面只显示日期 `submitted on 03 Oct 2026`，未显示发布时间；OpenCLI 搜索采集的 `created_utc=1791018288` 换算为 2026-10-03 17:04:48 +08。
- 2026-10-04 22:24 左右：OpenCLI Hacker News 搜索 `Gemini ending free use of Flash and Pro models`；HN item 49942592 为 59 points / 50 comments。随后从 HN Firebase item endpoint 读取创建时间 `2026-10-03 09:13:19 UTC`（北京时间 17:13:19）；Web 搜索工具直接打开 Firebase JSON 失败，PowerShell `Invoke-RestMethod` 读取成功。
- Reddit 创建时间从 OpenCLI 结果的 `created_utc` 换算：`r/GeminiAI` 1wwah6s 为 2026-10-03 09:11:34 +08；`r/Bard` 1wwaivj 为 09:14:05 +08。它们是帖子创建时刻，不是 Google 公告时间。
- 2026-10-04 22:56 左右：OpenCLI Browser session `1004-b` 在旧 Reddit `r/GeminiAI` 帖页读取 `#thing_t1_pdj77dq`；DOM 的 `.score`、`.bylink`、`.usertext-body` 显示 125 points、评论永久链接及正文。该评论追问 AI Plus 付费后能否明确得知 Flash 版本（3.8 或当前 3.6）；作为 L6 用户疑问记录，不当作官方政策或模型性能证据。可见字段存档：[b-reddit-top-comment.json](b-reddit-top-comment.json)。

## 公告与媒体检索

- Google Blog / Keyword：按 `Gemini model access change October 9 AI Plus`、`model availability personal accounts October 2026 October 9` 在 `blog.google` 限域检索；没有返回本次调整公告，结果为其他 Gemini / 计划新闻。目标帮助页的日期 meta 与 `<time>` 检查为空。此检索不能证明没有未索引公告。
- 官方 GeminiApp 账号：在 session `1004-b` 打开 `https://x.com/search?q=from%3AGeminiApp%20%22October%209%22&src=typed_query&f=live`，页面显示“出错了。请尝试重新加载”，所以记为账号搜索失败，不记为无公告。
- 媒体：按 Gemini / Flash-Lite / October 9 搜索 XDA、The Verge、TechCrunch、Android Authority；未找到本次变更的相关报道。TechCrunch 与 Android Authority 返回的可见结果为其他 Gemini 模型新闻，不用作本次来源。
- Wayback CDX：目标 answer/17004136 的旧快照 2026-09-30 07:21:20 UTC；首个所见新内容快照 2026-10-03 16:31:06 UTC，后者对应北京时间 2026-10-04 00:31:06。此处只据此界定“页面已变更的观察区间”，不称其为发布时间。

## 失败与替代路径

1. 按 `agent-reach` 流程运行 `conda run -n dl agent-reach doctor --json` 时，指定的 Conda 环境不存在；按机器软件约定检查 scoop、bun 全局包与 winget，未找到 agent-reach。未安装新软件。OpenCLI doctor 当时显示 Browser Bridge / extension 可连接，故用 OpenCLI 完成本组网页与截图抓取。
2. `curl.exe` 请求 Wayback CDX 时被执行审批拦截；没有重试该 curl 路径，后来通过已有浏览器 / 搜索结果取得快照记录。
3. AIHOT `web__run open` 两次分别返回 fetch timeout / cache miss。命名 session 的 AIHOT 实时页可加载，但当时可见页无该同题记录；因此只保留搜索索引卡片行并标为待核，不声称计数已确认。
4. X 的 `GeminiApp` 查询页返回错误提示，无可见帖子；本项标失败，未据此写“未发现帖子”。
5. Reddit `?tl=en` URL 打开后只显示页面骨架（无帖子节点）。改用旧 Reddit 原页并只截原帖区域后，截图成功。Reddit JSON 端点由 PowerShell 请求返回网络策略阻止页；帖子日期、正文与热度改由 OpenCLI 结果及旧 Reddit 可见页核对，未从失败响应推断发布时间。
6. `web__run open` 无法读取 HN Firebase JSON；相同公开 endpoint 的 PowerShell `Invoke-RestMethod` 可读出 title / time / score / descendants。

## 交付范围复核

B 组源档仅为本目录 `b-` 前缀文件；图片为 10–14。五张 PNG 均为 1400 px 宽且已目视复核。对 `docs/2610/1004/` 做身份泄露文本模式扫描并单独目视检查 B 组截图后，B 组未发现账户邮箱、`authuser` 参数或本机配置路径；截图 14 未含侧栏资料。目录下其他组材料归各组所有者，本组未改动。

---

## C 组：Aleph Alpha Kolibri（原文）

# 1004 期 C 组抓取日志

**执行组：** C（Aleph Alpha Kolibri）　**session：** `1004-c`  
**取证截止：** 2026-10-04 22:15 北京时间。所有抓取时间均为北京时间（UTC+8）；文件时间按本机 America/New_York（EDT）写入时间换算，UTC 时间另有注明。来源页面自己的发布时间单列，未把抓取时间当作发布时间。  
**范围：** 仅 C 组 `c-` 前缀材料、`evidence-c.md`、`capture-log-c.md` 与图片 15–24。

## 工具与采集方式

- 官方网页与模型卡：直接 HTTP 存档、官方 GitHub/Hugging Face 页面、OpenCLI 浏览器 session `1004-c`；社区公开页面使用 OpenCLI 与官方 Hacker News Firebase API。未登录、未读取私人账号资料、未点赞/评论/关注，也未执行网页写操作。
- 官方博客使用浏览器 DOM 文本提取并保存原始 HTML。技术报告保存官方 PDF，并用 `pdftotext` 生成可检索文本；指定页通过 `pdftoppm` 渲染为 1400px 宽 PNG。
- C 组采集脚本随文字材料保存：`c-blog-shots.mjs`、`c-pdf-shots.py`、`c-pull-supplements.ps1`。PDF 截图按 PDF 页码注明，其他源页快照不改原文内容。
- 官方比较模型的发布页、媒体页及 Cohere 协议页通过 PowerShell `Invoke-WebRequest` 存档；动态或受限页面以公开搜索/浏览器显示的正文交叉核对。社区计数只按注明的快照时点报告。
- `agent-reach check-update` 于 2026-10-04 22:15 BJT 返回 `v1.5.0` 已是最新版。OpenCLI 采集环境为 `v1.8.7`；运行时提示可更新至 `v1.8.8`，本任务未升级工具。

## 原始页面与数据快照

| 北京时间 | 来源/URL | 结果与本地文件 |
|---|---|---|
| 21:00:17–21:01:30 | [Aleph Alpha Kolibri 官方博客](https://aleph-alpha.com/en/blog/kolibri-has-landed-a-sovereign-open-weight-model/) | HTTP 原始页 `c-blog.html`、正文/表格 DOM 提取 `c-blog-browser.md`。博客页日期为 2026-10-03，没有发布时间时刻。 |
| 21:00:17–21:03:50 | [技术报告 PDF](https://aleph-alpha.com/downloads/tech-report.pdf) | 官方 PDF `c-tech-report.pdf`（189 页）、`pdftotext` 输出 `c-tech-report.txt`、分页文本 `c-tech-report-pages.txt`。 |
| 21:00:17、21:51、22:00 | [FP8 HF 模型卡](https://huggingface.co/Aleph-Alpha/Kolibri-1)、[BF16 HF 模型卡](https://huggingface.co/Aleph-Alpha/Kolibri-1-BF16)、[官方推理插件](https://github.com/Aleph-Alpha/aleph-alpha-inference)、[旧 Scaling 发布页](https://aleph-alpha.com/en/blog/scaling/) | 直连官方 HF，无镜像；分别存为 `c-hf-model-card.md`、`c-hf-model-card-bf16.md`、`c-vllm-plugin-readme.md`、`c-old-scaling-release.html`。 |
| 21:42:15–21:42:34 | 官方比较模型来源：[OLMo 3](https://allenai.org/blog/olmo3)、[Apertus](https://www.cscs.ch/science/computer-science-hpc/2025/apertus-a-fully-open-transparent-multilingual-language-model)、[Nemotron 3 Nano](https://research.nvidia.com/labs/nemotron/Nemotron-3/)、[Nemotron 3 Super](https://research.nvidia.com/labs/nemotron/Nemotron-3-Super/)、[Qwen3.5](https://qwen.ai/blog?id=qwen3.5)、[Qwen3.6](https://github.com/AlibabaCloud-Official/Qwen3.6)、[GLM-4.5](https://z.ai/blog/glm-4.5)、[Gemma 4](https://opensource.googleblog.com/2026/03/gemma-4-expanding-the-gemmaverse-with-apache-20.html) | 发布方页面以 `c-olmo3.html`、`c-apertus.html`、`c-nemotron3.html`、`c-nemotron3-super.html`、`c-qwen35.html`、`c-qwen36.html`、`c-glm45.html`、`c-gemma4.html` 保存。Z.ai 直连只返回 598-byte 页面壳，日期与公告内容改由官方搜索结果核对，限制见下文。 |
| 21:42:07–21:42:11 | [HN Firebase 49942706](https://hacker-news.firebaseio.com/v0/item/49942706.json)、[49943034](https://hacker-news.firebaseio.com/v0/item/49943034.json)、[49946069](https://hacker-news.firebaseio.com/v0/item/49946069.json) | 官方 Firebase 原始 JSON 存为三个 `c-hn-firebase-*.json`；其后续网页快照 `c-hn-*.json` 保留评论文本。分数/评论数采用 21:42 BJT 快照：626/318、414/12、109/5。 |
| 21:29:34–21:30:01；21:45:34 | [AIHOT Kolibri 条目](https://aihot.news/)、[Reddit LocalLLaMA 帖](https://www.reddit.com/r/LocalLLaMA/comments/1wwl7y6/alephalphakolibri1_hugging_face_78b_parameters/)、[Reddit AI gossip 帖](https://www.reddit.com/r/aigossips/comments/1wwvhql/germany_has_joined_the_race_of_frontier_llms_with/) | AIHOT 原始页/浏览器提取为 `c-aihot-story.html`、`c-aihot-story-browser.md`；公开 Reddit 页面为 `c-reddit-localllama.yaml`、`c-reddit-aigossips.yaml`。 |
| 21:42:28–21:42:45 | [Qwen3.8 官方 HF API](https://huggingface.co/api/models/Qwen/Qwen3.8-27B)、[Cohere 协议官方公告](https://aleph-alpha.com/en/news/cohere-agreement-transatlantic-sovereign-ai/)、[SWR/tagesschau](https://www.tagesschau.de/inland/regional/badenwuerttemberg/swr-heidelberger-ki-unternehmen-entwickelt-sprachmodell-und-wirbt-mit-ki-souveraenitaet-made-in-germany-100.html)、[MarkTechPost](https://www.marktechpost.com/2026/10/04/aleph-alpha-releases-kolibri-a-78-1b-open-weight-english-german-moe-model-with-only-3-46b-active-parameters/)、[CleverHack](https://cleverhack.com/the-urgency-of-open-source-ai) | 可直连的 Qwen API、Cohere、CleverHack 存于 `c-qwen38-api.json`、`c-cohere-agreement.html`、`c-cleverhack-kolibri.html`。SWR 与 MarkTechPost 直连失败，未伪造原始存档；公开文章正文另以网页搜索/阅读核对。 |
| 21:51:23–21:51:29 | [Qwen3.5 HF API](https://huggingface.co/api/models/Qwen/Qwen3.5-35B-A3B)、[Qwen3.6 HF API](https://huggingface.co/api/models/Qwen/Qwen3.6-35B-A3B)、[Qwen3.8 HF 模型卡](https://huggingface.co/Qwen/Qwen3.8-27B) | 官方 API 元数据存为 `c-qwen35-api.json`、`c-qwen36-api.json`，Qwen3.8 卡片存为 `c-qwen38-model-card.md`；仓库创建时刻只作元数据，不当模型发布日期。 |
| 21:56:12 | [HN 评论 49948516](https://hacker-news.firebaseio.com/v0/item/49948516.json) | 官方 Firebase 返回评论原文和 ID，存为 `c-hn-comment-49948516.json`；单条评论没有 score 字段。 |
| 14:15:05 UTC 直连完成（22:15:05 BJT） | [Artificial Analysis Models](https://artificialanalysis.ai/models/)、[LiveBench](https://livebench.ai/) | 直连 HTTP 200，原始页存为 `c-aa-models-page.html`（1,368,536 bytes）、`c-livebench-leaderboard.html`（1,066 bytes）。文件较大但本次没有提交或推送。 |

## 来源页发布时间

| 页面/项目 | 页面给出的时间 | 北京时间换算或限制 |
|---|---|---|
| Kolibri 官方博客 | 2026-10-03 | 没有钟点；页面日期格式 `03/10/2026`，按页面德国语境及 HF 模型卡核为 10 月 3 日。 |
| HF FP8 模型卡 | 2026-10-03 | 卡片自列 Release Date，未列钟点。技术报告封面未列发布日期或时间。 |
| HN 49942706 | 2026-10-03 09:36:04 UTC | 2026-10-03 17:36:04 BJT。 |
| HN 49943034 | 2026-10-03 10:43:51 UTC | 2026-10-03 18:43:51 BJT。 |
| HN 49946069 | 2026-10-03 17:22:48 UTC | 2026-10-04 01:22:48 BJT。 |
| SWR/tagesschau | 2026-10-03 13:40 CEST | 2026-10-03 19:40 BJT；直连存档 404，时间来自可读页面。 |
| MarkTechPost | 2026-10-04 | 页面未列时刻；直连存档 403，日期来自可读页面。 |
| AIHOT | 峰值 69，显示 2026-10-04 17:00 | 条目未标时区；该数字是聚合站热度，不是发布时刻或模型分数。 |
| Cohere/Aleph Alpha 协议公告 | 2026-09-16 | 官方公告给日期，未核到时刻；公告状态为已签协议、待监管批准。 |
| 基线与后续比较模型 | 日期及具体口径见 `evidence-c.md`“官方发布日期与比较边界” | 系列首发、仓库 `createdAt` 与实际 checkpoint 发布日分开记录；页面未给时间者不补推测。 |

## 截图记录

| 文件 | 北京时间 | 内容与生成方式 |
|---|---|---|
| [15-kolibri-blog-claims.png](../images/15-kolibri-blog-claims.png) | 21:22:52 | 官方博客标题、参数、许可、1M、sovereignty/Pareto 主张；浏览器截图，700 CSS px、2 倍像素比。 |
| [16-kolibri-blog-benchmarks.png](../images/16-kolibri-blog-benchmarks.png) | 21:22:52 | 官方博客完整 benchmark 表，展开细节区并将表格缩放至可读范围；含全 17 行与全部列头。初版截图不全，重新展开并复核后以本文件覆盖。 |
| [17-tech-report-abstract.png](../images/17-tech-report-abstract.png) | 21:20:56 | 报告首页摘要和训练总览。 |
| [18-tech-report-main-claims.png](../images/18-tech-report-main-claims.png) | 21:20:56 | PDF 第 5 页 Table 1，参数和模型基本信息。 |
| [19-tech-report-context-extrapolation.png](../images/19-tech-report-context-extrapolation.png) | 21:20:57 | PDF 第 41 页，长上下文长度与任务依赖退化说明。 |
| [20-tech-report-baseline-protocol.png](../images/20-tech-report-baseline-protocol.png) | 21:20:57 | PDF 第 45 页，八个基线与统一评测协议。 |
| [21-tech-report-posttraining-table-1.png](../images/21-tech-report-posttraining-table-1.png) | 21:20:58 | PDF 第 100 页，表 28 第一部分。 |
| [22-tech-report-posttraining-table-2.png](../images/22-tech-report-posttraining-table-2.png) | 21:20:58 | PDF 第 101 页，表 29 与均值排除说明。 |
| [23-tech-report-customer-proxies.png](../images/23-tech-report-customer-proxies.png) | 21:20:59 | PDF 第 102 页，内部 customer-proxy 相关图表。 |
| [24-tech-report-grounding-figure.png](../images/24-tech-report-grounding-figure.png) | 21:20:59 | PDF 第 103 页，grounding/non-hallucination 定义图。 |

图片 15–16 为网页截图，宽 1400 px、2 倍像素比；17–24 由 PDF 页面渲染，宽 1400 px。图片均作目视核验，没有重绘文字。15 号 PNG 为 1,120,138 bytes，超过 1 MB；其他 C 组 PNG 小于 1 MB。当前只作为本地 C 组取证材料，未提交或推送。

按字节数检查，C 组当前超过 1 MB 的文件还有 `c-tech-report.pdf`（3,465,511）、`c-olmo3.html`（1,329,421）和 `c-aa-models-page.html`（1,368,536）。这些原件和截图都只留本地工作目录；如后续要提交，须按仓库发布约定处理大文件及图片/PDF/HTML 的网盘归档。

## 检索与抓取失败

- 初次通过 `conda run -n dl` 调用 agent-reach 失败：本机没有 `dl` conda 环境。改用 PATH 中的 `agent-reach`，Doctor 路由正常，任务继续。
- Exa 在前几次成功检索后返回 HTTP 429 rate limit。缩小请求并改用公开原站、OpenCLI 浏览器及网页搜索结果；未重复轰击限流端点。
- `Invoke-WebRequest` 对 SWR/tagesschau 返回 HTTP 404；对 MarkTechPost 返回 HTTP 403。两页公开正文可由网页阅读器访问，所以只记录页面日期与可核正文，不保存失败响应为原文。
- `web` 阅读器对 AIHOT 条目无法打开；OpenCLI 浏览器取得渲染正文与热度字段。
- Reddit 公开 JSON 请求返回 403 Blocked。没有尝试绕过或使用登录凭据；通过 OpenCLI 读取公开页面并保存所见内容。
- Z.ai GLM-4.5 页面直连仅返回 598-byte HTML 外壳，不能证明正文已保存；官方日期从发布方可读索引结果交叉核对，证据稿注明此限制。
- 首次博客 benchmark 图没有完整呈现折叠表格；修改截图脚本为展开表格、缩小显示后重抓，目视确认全表可见。
- HN 官方 API 不返回单条评论分数；评论 49948516 只记录原文和链接。AIHOT 时间线未给时区，未自行换算其 17:00。
- C11 限定搜索没有找到符合 handoff 明确性能夸大条件的标题或文章；这是本次检索范围的未发现，不表述为全网绝不存在。

## 验收与状态

- 交付文件：`evidence-c.md`、`capture-log-c.md`；图片范围 15–24。来源存档、脚本与记录均用 `c-` 前缀。
- 没有改动 A、B、D 组文件；没有删除文件；没有 commit 或 push。未运行测试（本次为取证任务，派工单未要求测试）。
- 校勘时更正两项扫描误述：技术报告 §2.4.2 的八基线讨论在 PDF 第 45 页；RULER 超出模型窗口报告“–”缺分，只有 τ³-Bench 中超窗的会话记 0，不能一概说所有超窗输入记 0。

---

## D 组：速览（额度重置等）（原文）

# D 组抓取记录

工作目录：E:\gitclone\AI-Barking。Session：1004-d。只处理 D 组及图片 25–29；没有改 A/B/C 文件，没有 commit、push 或删除文件。网址均为不含追踪参数的原始地址。时间均记北京时间（UTC+8）；平台 API 的 publish time 另注明原始时区。

## 环境与过程

- OpenCLI 1.8.7 的 daemon、extension、connectivity 检查通过；浏览器复用已有会话，未登录、未输入凭据、未点赞、回复、关注或接受非必要 Cookie。
- agent-reach doctor 报告 Twitter / Reddit 没有 active backend；按技能路由使用可用的 OpenCLI Twitter、Reddit、Hacker News adapters。OpenCLI 的 Twitter 搜索一次超时、重试后遇到 429，之后停止搜索并改用已知状态页、作者 timeline 与直接状态 URL。
- 首轮官方页面查阅通过 web 页面打开/检索完成，不晚于 2026-10-04 21:21:54 北京时间；web 工具未返回逐页抓取时间。为留下当前 URL 导航回执，22:05–22:07 又通过 1004-d 逐页打开，下面记录该批次的起止时间。
- X 的直接 CDP 截图尝试曾返回全白 PNG；这些临时结果未保留。改用 OpenCLI browser screenshot，CSS 仅隐藏目标帖子以外的页面并压紧文字宽度，没有重绘或改写文字。最终五图视觉复核通过，均为 2 倍像素比、宽 1400 px，未包含浏览器访客侧栏或回复框。#28 隐去嵌入的引文/视频占位区域，只保留 ClaudeDevs 主帖文字、时间与互动信息。
- 只读取目标源的元数据、简短摘录与事实摘要；不保存两篇 OpenAI Help Center 全文，也不保存 Altman 全帖的文字副本。Help 全文存档会超出可复制的来源摘录范围；Altman 原帖是 28 个英文词，本地只保留不超过 25 词的摘句并留直链。

## 逐项来源与抓取

| 抓取北京时间 | URL | 工具与动作 | 结果 / 文件 |
|---|---|---|---|
| 不晚于 21:21:54；截图 21:42:42 | https://x.com/thsottiaux/status/2105843926221660585 | OpenCLI Twitter timeline/direct status；browser 1004-d 截帖子卡片 | 已找到原文与 UTC publish time；最终图 [25](../images/25-tibo-global-reset-announcement.png)，1400×694，SHA-256 3d39d434d6ea674c3fd1639fe6b6c3e47e80ae0922b82494950c93fc528c5fe0 |
| 不晚于 21:21:54；截图 21:45:52 | https://x.com/thsottiaux/status/2106131810921136451 | OpenCLI Twitter timeline/direct status；browser 1004-d 截帖子卡片 | 已找到 “propagated” 确认；嵌入预告显示为 X 页面中文翻译，预告英文原文另由 #25 与 timeline 核对。最终图 [26](../images/26-tibo-reset-propagated.png)，1400×930，SHA-256 429226be0e97ab29b1696d4c0a79e95fc7814aaf463683c10062821c88546d64 |
| 21:35:39 | https://help.openai.com/en/articles/20001498-how-banked-codex-resets-work | OpenCLI browser 1004-d；截 Eligibility and expiration section | 已打开并截图为 [27](../images/27-openai-banked-reset-eligibility.png)，1400×1480，SHA-256 cd37eba1d796c999eeb6fd6b1e0dcfd74416092fb40719c114a6cb785695b880。页面显示相对更新文字 “8 days ago”；页面记录 9/3、9/4 offer 条件，未列 9/22 专属到期日。 |
| 22:05:47–22:05:54 | https://help.openai.com/en/articles/20001498-how-banked-codex-resets-work | OpenCLI browser open | 导航完成；用作最新页面回执，不复制全文。 |
| 不晚于 21:21:54；22:05:54–22:06:00 | https://help.openai.com/en/articles/20001507-paid-weekly-work-and-codex-rate-limit-resets | web page read；后通过 OpenCLI browser open | 页面可打开；与促销 banked reset 分开记录。全文未镜像。 |
| 不晚于 21:21:54；22:06:00–22:06:06 | https://help.openai.com/en/articles/20001516-managing-usage-with-gpt-6-astra-in-work-and-codex | web page read；后通过 OpenCLI browser open | 页面可打开；未见 10/2 新重置公告。 |
| 22:06:06–22:06:18 | https://openai.com/devday/2026/ | OpenCLI browser open；首轮另用 web 页面 find reset | OpenCLI 导航超时；先前 web 页面可读，检索 reset 未匹配。DevDay 未找到 banked reset 说明。 |
| 不晚于 21:21:54；22:06:18–22:06:26 | https://help.openai.com/en/articles/6825453-chatgpt-release-notes | web page read；OpenCLI browser open | 导航完成；找到 Oct 2 Finances 条目，官方只给日期。 |
| 不晚于 21:21:54；22:06:26–22:06:31 | https://help.openai.com/en/articles/20001222-finances-in-chatgpt | web page read；OpenCLI browser open | 导航完成；该独立产品说明列当前适用范围。 |
| 22:07:41 后、不晚于 22:14:29 | https://help.openai.com/en/articles/20001222-finances-in-chatgpt | OpenAI Help 页面复开核对 | 原文为 “ChatGPT can help you understand, plan, and evaluate financial decisions, but it cannot take financial actions for you.”；此前 evidence-d 的短引语并非逐字原文，已更正。工具未返回逐请求时刻。 |
| 不晚于 21:21:54；22:06:31–22:06:43 | https://artificialanalysis.ai/changelog | web page read；OpenCLI browser open | OpenCLI 导航超时；首轮页面可读，条目标 2026-10-03、没有具体时刻。 |
| 不晚于 21:21:54；22:06:43–22:06:51 | https://artificialanalysis.ai/models/ling-3-1-flash | web page read；OpenCLI browser open | 导航完成；页面列 Index 41 及每百万 token 价格。 |
| 不晚于 21:21:54；22:06:51–22:07:03 | https://blogs.nvidia.com/blog/local-ai-dgx-spark-64gb-sync/ | web page read；OpenCLI browser open | OpenCLI 导航超时；首轮官方页可读，标 Oct 2、Oct 23 开始供货及 $4,999 起价。 |
| 不晚于 21:21:54；22:07:03–22:07:13 | https://www.anthropic.com/news/claude-frontier-academy | web page read；OpenCLI browser open | 导航完成；官方只标 Oct 2 日期，没有发布时刻。 |
| 不晚于 21:21:54；22:07:13–22:07:17 | https://www.anthropic.com/claude-opus-5-5 | web page read；OpenCLI browser open | 导航完成；发布页写 5 小时限额变化与 subscription reset；全文检索 October 22 未匹配。 |
| 不晚于 21:21:54；截图 21:48:32 | https://x.com/ClaudeDevs/status/2102438800836489554 | OpenCLI Twitter timeline/direct status；browser 1004-d | 按帖子 permalink 精确定位 thread 内目标卡片；最终图 [28](../images/28-claude-reset-announcement.png)，1400×778，SHA-256 55bcd1c0c2b7cc27a384e0f43cc4e0c2bc870b0dbebf906fbc50bd1f5fcf0f25。 |
| 不晚于 21:21:54；22:07:17–22:07:29 | https://support.claude.com/en/articles/17007452-what-is-a-limit-reset | web page read；OpenCLI browser open | 导航完成；通用帮助页说明到期日如有会在 Usage 页面显示，没有写 10/22。 |
| 不晚于 21:21:54；截图 21:49:44 | https://x.com/ClaudeDevs/status/2104641323198472430 | OpenCLI Twitter timeline/direct status；browser 1004-d | 首屏 thread 含多个帖子；用 URL 中的 status ID 匹配目标卡片后截图。最终图 [29](../images/29-claude-reset-deadline.png)，1400×730，SHA-256 f619caff6b3fccfe1832f52e3c8b3096f8db503fc8c57de2b51bf0d54e33eb9f。 |
| 不晚于 21:21:54 | https://x.com/sama/status/2106388373221118198 | OpenCLI Twitter search/direct status；web page read | 原帖找到；API 记录创建时间 2026-10-03T14:18:17Z；未截图，截图区间优先用于额度重置帖子与帮助页。 |
| 不晚于 21:21:54；22:07:29–22:07:41 | https://www.axios.com/2026/10/03/openai-anthropic-altman-amodei-religious-force-models | web page read；OpenCLI browser open | web 结果可读并报道原帖；OpenCLI 导航超时。Axios publish time 为 2026-10-03 15:41:59 UTC。 |
| 21:49:44–22:05:47（区间回执；CLI 未返回每次请求时钟） | https://www.reddit.com/r/OpenAI/comments/1wviwfu/rest_10am_pst_tomorrow/ | OpenCLI Reddit search/read 与 browser eval | 找到讨论串与评论 permalink。Pasto_Shouwa 评论分数在 OpenCLI / 浏览器结果间为 60 / 59，necrohobo 为 18 / 17；分数会变，证据中按动态区间记录。 |
| 同上 | https://www.reddit.com/r/OpenAI/comments/1wnq2cb/sir_dario_just_dropped_opus_55_and_it_beats_gpt6/ | OpenCLI Reddit read 与 browser eval | 纠正初始 subreddit 路径：页面元数据确认原帖属 r/OpenAI，不属 r/ClaudeAI。取得帖分、时间和评论 permalink。帖文为玩梗图片，不按新闻报道采信。 |
| 同上 | https://news.ycombinator.com/ | OpenCLI Hacker News search：OpenAI banked reset；Claude 5.5 reset；Altman religious force AI；ChatGPT Finances | 仅找到一条分数 1、0 评论的弱相关帖；其余无具体事件高分讨论，未摘作实质质疑。 |
| 同上 | — | OpenCLI Reddit broad searches：ChatGPT reset、Claude reset、Finances、Altman religious force、Ling 3.1 Flash、DGX Spark、Frontier Academy | 广泛检索结果噪声高；进一步限制 subreddit 与短语后找到上述两条 r/OpenAI 讨论。未找到 D5–D8 的相关高赞质疑。 |

## 未完成与失败

- Twitter 搜索先超时、重试遇 429；停止重试，使用已有 timeline、直接帖子链接和官方网页，因此非完整全站搜索。
- OpenCLI 直接导航 DevDay、Artificial Analysis changelog、NVIDIA 页面及 Axios 页面超时；先前 web 页面读取成功的来源事实已记在 evidence-d，未把超时误报成内容不存在。
- 按 D 组指定的 25–29 图片编号，最终截图优先用于两条 Tibo 帖、OpenAI banked-reset 帮助页、ClaudeDevs 9/22 与 9/28 帖。D4 与 D5–D8 没有独立截图，仍有截图覆盖缺口；其他一手 URL、日期和摘录已核。
- D2 派工要求两篇 OpenAI Help Center 全文存档；本组未保存全文镜像，只留短摘句、结构/事实摘要和 URL。Altman 原帖全文也未复制到文件，保留短摘录和直链。

## 图片校验

| 文件 | 像素 | 文件大小 | SHA-256 |
|---|---:|---:|---|
| 25-tibo-global-reset-announcement.png | 1400×694 | 97,918 | 3d39d434d6ea674c3fd1639fe6b6c3e47e80ae0922b82494950c93fc528c5fe0 |
| 26-tibo-reset-propagated.png | 1400×930 | 127,169 | 429226be0e97ab29b1696d4c0a79e95fc7814aaf463683c10062821c88546d64 |
| 27-openai-banked-reset-eligibility.png | 1400×1480 | 209,802 | cd37eba1d796c999eeb6fd6b1e0dcfd74416092fb40719c114a6cb785695b880 |
| 28-claude-reset-announcement.png | 1400×778 | 109,989 | 55bcd1c0c2b7cc27a384e0f43cc4e0c2bc870b0dbebf906fbc50bd1f5fcf0f25 |
| 29-claude-reset-deadline.png | 1400×730 | 96,184 | f619caff6b3fccfe1832f52e3c8b3096f8db503fc8c57de2b51bf0d54e33eb9f |

---

## Claude 补充抓取与加工（2026-10-04，北京时间）

- 《卫报》标题页：`e-guardian-shot.mjs`（opencli browser `1004-e` + CDP Page.captureScreenshot，700 CSS px、2 倍像素比，23:10 抓取），整页 `../images/30-guardian-headline-full.png`；A 组 Guardian 文本存档为 `a-guardian.txt`，截图补上标题页外观。页面标 “Sun 4 Oct 2026 03.48 EDT”（美国版显示时间）。
- Kolibri 技术报告第 98 页：`pdftoppm -f 98 -l 98 -r 200` 从 `c-tech-report.pdf` 渲染，`../images/35-kolibri-p98-full.png`。
- 底图裁切（PIL，像素不改，只裁区域；裁切避开抓取浏览器右侧的翻译插件浮标，即粉色圆形图标，非原站内容）：`30-guardian-headline-raw.png`（自 30-full，y 1500–2800）、`31-openai-slack-report-raw.png`（自 07，y 0–980）、`32-gemini-access-table-raw.png`（自 10，x 0–1320）、`33-kolibri-blog-raw.png`（自 15，x 0–1400、y 1300–1580；首版 x 0–1320 会截断右边缘，视觉复核后改）、`34-kolibri-context-raw.png`（自 19，x 170–1230、y 1240–1630）、`35-kolibri-p98-raw.png`（自 35-full，x 205–1500、y 1420–2010）、`36-kolibri-table-raw.png`（自 21，y 195–660，表 28 表头与 Overall 两行，备用，未渲染批注卡：缩进 1080 宽后字太小）。
- 渲染：`barking card docs/2610/1004`；批注 30–35、省流卡、速览图。
- Claude 独立读取：Google 帮助页 16275805 与 17004136（渲染后表格 DOM）、Altman 帖全文（`opencli twitter thread`）、OpenAI 三份报告存档文本 grep、Reuters/TechCrunch/Guardian 存档文本 grep、HN Algolia/Firebase 分数。

- 反向核验与视觉复核后的修订：`barking card` 重渲全部卡片；补存 `d-altman-post.json`；`evidence.md` 开头加“Claude 更正”。
