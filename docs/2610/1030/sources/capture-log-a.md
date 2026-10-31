# A 组 capture-log（北京时间，UTC+8）

逐条请求的 URL、结果、时刻、字节数与 SHA-256 在机器日志里：`sources/a-fetch-log.jsonl`（`barking fetch`，含 `time_bj`）、`sources/a-shots-log.jsonl`（`barking shot`，含 `time_bj`）、`sources/a-heat.json`、`sources/a-hn-heat.json`。下表只记批次、工具与例外。执行方：claude-sonnet-5-5；时段 2026-10-31 16:48–17:25。

## 批量存档（barking fetch，批次与结果）

| 批次 | 内容 | 开始时刻 | 结果 |
|---|---|---|---|
| a1 | claude.com/pricing；Ultra 帮助页；官方帖 fxtwitter JSON；HN 50308320（algolia） | 16:48:27 | 4 ok |
| a2 | Max 帮助页；用量限制页 11647753；Fable 页；API 额度页 17154008；release notes 12138966；Pro 帮助页 | 16:49:25 | 6 ok |
| a3 | Wayback 快照（`…/web/<时间戳>id_/<URL>`，只看“旧页写了什么”）：定价页 20261030031525（旧）、20261031041924（新）；Max 帮助页 20261024051700（旧）、20261030224434（新）；用量限制页 20261030051951（旧）、20261031054939（新） | 16:49:56 | 6 ok |
| a4 | CourtListener docket 页；Justia docket；PacerMonitor docket；The New Stack；AI Weekly；daily.dev | 16:52:18 | cl-docket、pacermonitor、news-newstack、news-aiweekly、news-dailydev ok；**justia-docket blocked**（Cloudflare 质询，403，未再试） |
| a5 | CourtListener RSS；docket 条目页 39、47、37；storage.courtlistener.com 的 PDF：ECF 1、28、39、37、43、47 | 16:53:32 | cl-feed、cl-doc39、cl-doc47、cl-doc1-pdf、cl-doc28-pdf、cl-doc39-pdf ok；**cl-doc37 blocked**（202 空页）；**cl-doc37-pdf、cl-doc43-pdf、cl-doc47-pdf failed（404，未入 RECAP）** |
| a6 | Boris Cherny 帖 fxtwitter JSON；Techmeme 页 | ~17:00 | 2 ok |
| a7 | sysnote.net、The Verge、IT之家、人人都是产品经理、yeeyi、AIHOT | ~17:03 | sysnote、theverge、ithome、woshipm ok；**yeeyi blocked**（正文过短，前端渲染）；**aihot failed**（协议错误，未重试，改取 X 帖原文） |
| a8 | Mike Overell 帖 fxtwitter JSON | ~17:05 | ok |
| a9 | Ado Kukic 帖 fxtwitter JSON | ~17:07 | ok |

自写文件：`sources/a-hn-item-50308221.json`、`sources/a-hn-item-50314228.json`（HN algolia 条目，`curl --max-time 30` 取），`sources/a-xcard.ts`（X 官方嵌入页的匿名 headless Chrome 卡片截图脚本，bun，全新临时 user-data-dir，不读本机浏览器 Cookie）。

## 例外与失败（如实）

- **CourtListener 搜索接口**：`/api/rest/v4/search/` 无凭据返回 “Authentication credentials were not provided.”（未登录、未用任何 token）；`/?q=…&type=r` 搜索页：curl 与 `barking shot render`（headless Chrome）均 403（CloudFront “Request blocked”），共两次，记失败，改用 WebSearch 找到 docket 的固定路径后以 `barking fetch` 取，docket 页取到（200）。
- **CourtListener docket 页的后半条目描述缺失**：ECF 34 起的条目描述在静态 HTML 里为空（只有文书类别名），内容以 RSS（`a-cl-feed.xml`）与 PacerMonitor（`a-pacermonitor.txt`，第三方，最后更新于 10-12，缺 10-19 之后的条目）补；2026-10-30 的 “Order on Motion to Dismiss” minute entry 在两处都没有正文。
- **PACER**：未登录、未购买任何文书。
- **Reddit**：`barking heat --reddit` 被重定向到登录页（old.reddit.com/login），未取到数据，未使用登录态。
- **X 长帖展开**：官方嵌入页的 “Show more” 是外链，匿名 headless Chrome 无法展开，所以 `07-x-claudeai-card.png`、`08-x-overell-card.png` 里帖子正文被截断（卡片本身是 X 官方渲染，未改任何像素）。完整原文见 fxtwitter JSON：`a-fxtweet.json`、`a-fx-mboverell.json`、`a-fx-bcherny.json`、`a-fx-adocomplete.json`。直接渲染 x.com 帖页：Chrome 退出且未出图，未重试。
- **Wayback 渲染**：截图用 `…/web/<时间戳>if_/<URL>`（无 Wayback 工具栏），快照时刻在 evidence-a.md A3c；仅用于“旧页写了什么”。
- **搜索**：WebSearch 与 Exa（web_search_exa）用来发现 URL 与员工帖 ID（Mike Overell、Ado Kukic 的帖 ID 来自 sysnote.net 页内链接与 AIHOT 摘要），发现之后全部回到一手页面/JSON 逐字核；二手摘录（yeeyi 文章、腾讯新闻/SemiAnalysis 灰测稿、golber.net、vanja.io）只在 evidence-a.md 标 L5 并注明未取一手。
- 未使用 opencli（扩展未连接）、未登录、未接受 Cookie 提示、未发帖/点赞/评论/关注。

## 截图清单（`docs/2610/1030/images/`，均 2 倍像素比，PDF 页 144 dpi，已逐张打开目视确认文字清楚）

| 文件 | 来源 | 说明 |
|---|---|---|
| 01-pricing-cards.png | https://claude.com/pricing（`barking shot render` 700 CSS px，DPR 2，2026-10-31 16:56 北京）；cropbox 0,2440–1400,4240 | Max 与 Ultra 两张卡与脚注（含 “No 5-hour limit for verified accounts”），浅色底 |
| 02-pricing-table.png | 同上；两段拼接：表头（Free/Pro/Max/Ultra）+ “Models and usage” 前几行 | Limit every 5 hours / Limit every week / Fable 行；未含表格其余行（与说法无关） |
| 03-ultra-help-top.png | https://support.claude.com/en/articles/17728952-what-is-the-ultra-plan （同法，16:56） | 标题、Updated today、Key benefits、What changed from Max 20x? |
| 04-ultra-help-limits-verified.png | 同上 | “Does the Ultra plan have any usage limits?” 与 “What is a verified Ultra account?” 两节完整 |
| 05-max-help-old-limits.png | Wayback 2026-10-24 05:17 UTC 快照的 Max 帮助页（旧） | 三段拼接：标题与 “Updated over a week ago”、Key benefits “5x or 20x”、“Does the Max plan have any usage limits?” 整节（含 “per-session usage allowance”）。**旧页面** |
| 06-pricing-old-max.png | Wayback 2026-10-30 03:15 UTC 快照的定价页（旧） | Max 卡 “Choose 5x or 20x more usage than Pro*”。旧版页面布局只有这一张 Max 卡，右半空白是原样。**旧页面** |
| 07-x-claudeai-card.png | X 官方嵌入页（`a-xcard.ts`），@claudeai 帖 | 帖子卡片本身；正文在 “starting today with most existing” 处被截断；含官方账号头像与 handle（官方账号，非抓取者）。2026-10-31 约 17:1x 北京渲染 |
| 08-x-overell-card.png | 同上，@mboverell 帖 | 卡片正文在 “When we added weekly limits to Max 20x, we” 处被截断（关键的 “set them to 10x Pro's” 在 JSON 里，不在图里）；含署名员工头像与 handle（公开职业账号） |
| 09-mtd-caption.png | CourtListener 存的 ECF 39 PDF 第 1 页（本地 `a-cl-doc39-pdf.pdf`） | 动议封面：案号、“Consolidated with 4:26-cv-07789-AGT”、Date: November 6, 2026 |
| 10-mtd-intro-per-session.png | ECF 39 PDF 第 9 页（文书页 1，Introduction） | “per session” 对 “weekly usage limits” |
| 11-mtd-per-session-argument.png | ECF 39 PDF 第 15 页（文书页 7，§I.A） | 引用购买流程页文案与 “‘also’ signals…” 论证 |
| 12-complaint-3-5x-6-8x.png | ECF 1 诉状 PDF 第 2 页 | ¶4–8 |

截图过程中的试验输出：`images/` 内无多余文件；两张不合格的试验图（`01-pricing-max-ultra.png` 顶部裁到图标中间、`02-pricing-table.png` 缺列头）已移到本会话临时目录（用 `mv`，未用 `rm`，不在仓库内）。`sources/a-shots-log.jsonl` 里还保留这两张图的记录行，以及 `cl-search1`（CourtListener 403 页）、`x-page-test` 的渲染记录，属于失败/试验记录，保留不删。

## 隐私与泄露自查

- `grep` 本组文件（`docs/2610/1030/sources/a-*`、`evidence-a.md`、`capture-log-a.md`、`images/01–12-*.png` 的来源页）：未登录任何账号；抓取全部匿名；X 内容只含官方账号（@claudeai）与公开职业身份的员工帖（Overell、Kukic、Cherny），没有抓取者头像、昵称、handle。
- 公开 docket 与诉状存档含原告姓名（案件题名）、诉状里一句“was a resident of Washington, District of Columbia”（`a-cl-doc1-pdf.txt` 第 178 行附近）、原被告律师姓名与律所地址。这些是公开 docket 信息，本组未另行收集；evidence-a.md 只引案件题名，不引原告居住地。若 Claude 要求更严，可以删掉 `a-cl-doc1-pdf.txt` 第 170–180 行附近一句。

## 体积与入库提示（Claude 收尾用）

- 单个文本文件均 ≤ 105 KB（最大：`a-cl-doc39-pdf.txt` 104,834 B），没有超过 1 MB 的文本文件。
- 不入库、只存本地/网盘：所有 `a-*.html`（`a-pricing.html` 1.32 MB、`a-news-*.html`、`a-wb-*.html` 等）、`a-cl-doc*.pdf`（ECF 1 PDF 1.4 MB 等）、`images/` 下全部 PNG。
- `a-cl-docket.html` 218 KB、`a-techmeme.html` 405 KB、`a-release-notes.html` 361 KB（release notes 页取回后未用，可不上传）。

## 补充（交卷前）

- a10（17:2x 北京）：GovInfo `mods.xml` 与 `getContentDetail`，2 ok，只含 2026-07-07 一份命令，见 evidence-a.md“补充核查”。
- CourtListener docket 降序页（`?order_by=desc`）重渲染一次（`cl-docket-desc`），文字识别无 10-30 条目的正文；第二次探查，停止。
- 嵌入页首次直接渲染（无 `--virtual-time-budget`）返回空白（`tw-claudeai.png` 等），加 `--chrome-arg=--virtual-time-budget=10000` 后能出图（`tw-claudeai2`）；因嵌入页点不开长帖，另写 `a-xcard.ts` 试图展开并只截卡片，结果仍被截断，脚本是否保留由 Claude 定。
