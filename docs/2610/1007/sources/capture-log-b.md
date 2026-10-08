# 1007 B 组抓取日志（Mistral Large 4）

时间均为北京时间 UTC+8，日期 2026-10-08（本机原时区 EDT，仍是 10/7）。工具与脚本都在 `docs/2610/1007/sources/`，前缀 `b-`。
逐次 HTTP 台账见 `b-http-records.jsonl`（每次匿名 curl 的 URL、北京时间、状态码、字节数）；渲染与截图台账见 `b-browser-records.jsonl`。**saved 不等于正文有效**，有效性以下表为准。

## 工具与方式

- HTTP：`b-http.mjs`（curl.exe，匿名，普通 Chrome UA）。
- 渲染与截图：`b-browser.mjs` / `b-arena-shot.mjs` / `b-hover-shot.mjs` / `b-doc-weights-shot.mjs` / `b-embed-shot.mjs`（Playwright Core + 本机 Chrome 无头，每次用新的临时 profile，不登录，不带 Cookie；CSS 宽 700、DPR 2 即 1400 px 宽；个别图为满足 ≤1400 px 用了 DPR 1.65 或 2.5，见下表）。调试用 `b-probe.mjs`、`b-hover-probe.mjs`（只读，不写文件）。`b-doc-tabs.mjs` 点击 Features / Weights / Usage 页签（页面内 UI 切换，非写操作）。
- 图表原图转 PNG：`b-chartimg.py`（PIL Lanczos 缩到 1400 px 宽，不重绘、不改数字）。
- web.archive.org：只用于看“某页面以前写了什么”，用 `…id_/` 原始快照；返回内容为 gzip，已解压写回 `b-wayback-*.html`。
- X：官方 oEmbed（`publish.twitter.com/oembed`，匿名、公开推文）取文字；帖子卡片截图用 X 自己的嵌入端点（匿名）。
- 未使用 opencli（扩展未连接，见失败项）。浏览器未登录；未输入任何凭据；未点赞、未评论、未关注；付费墙未绕过。

## 有效抓取（URL | 工具 | 结果 | 北京时间）

| 文件基名 | URL（已去查询参数） | 工具 | 结果 | 时间 |
|---|---|---|---|---|
| b-blog | https://mistral.ai/news/mistral-large-4/ | curl | 200，291,702 B，正文完整；与 1006 的 `d-mistral-blog.txt` 文字逐行 diff 无差异；图片资源有变（见 `evidence-b.md` B2） | 01:24:05 |
| b-blog-browser | 同上 | Playwright 700 px | 渲染文本 15,658 字符（含全文） | 01:28:16 |
| b-doc | https://docs.mistral.ai/models/mistral-large-4-0 | curl | 200，1,235,149 B；描述为 52B（1006 为 49B） | 01:24:05 |
| b-doc-browser | 同上 | Playwright 700 px | 渲染文本 1,443 字符 | 01:28:31 |
| b-doc-tabs | 同上 | Playwright 1400 px，点击页签 | WEIGHTS 表：Coming soon / Coming soon / 1.05 / 52 / 1.6 / N/A / 1M | 01:54:53 |
| b-pricing | https://docs.mistral.ai/inference/pricing | curl | 200，1,148,330 B；Large 4 行含 Original/Sale price | 01:24:05 |
| b-doc-lifecycle | https://docs.mistral.ai/models/model-lifecycle | curl | 200；Public Preview 定义 | 01:54:30 |
| b-aa | https://artificialanalysis.ai/models/mistral-large-4 | curl | 200，4,044,151 B；Speed #44、524k、FAQ 520k | 01:24:05 |
| b-aa-article | https://artificialanalysis.ai/articles/mistral-large-4-france-ai | curl | 200，正文完整；5 张 AA 图已下载（`b-assets/b-aa-article-*.png`） | 01:32:04 |
| b-aa-gdppdf | https://artificialanalysis.ai/evaluations/gdp-pdf | curl | 200；嵌入数据含 30 个默认模型（含 isOpenWeights、GDP.pdf、AutomationBench 值） | 01:47:51 |
| b-arena-webdev | https://arena.ai/leaderboard/code/webdev/overall | curl | 200，1,114,730 B；含 mistral-large-4 行（SSR 文本日期 Oct 7, 2026） | 01:32:04 |
| b-hf-api / b-hf-api2 / b-hf-mistralai | https://huggingface.co/api/models?… | curl | 200，JSON；无 Large 4 仓库 | 01:32:05 |
| b-hn-search、b-hn-search-le-chonk | https://hn.algolia.com/api/v1/search?… | curl | 200，JSON | 01:47:11–01:47:14 |
| b-hn-main | https://hn.algolia.com/api/v1/items/49977979 | curl | 200，581,021 B，全评论树 | 01:47:33 |
| b-tnw | https://thenextweb.com/news/mistral-releases-large-4-a-1-trillion-parameter-open-weight-ai-model | curl | 200，正文完整（标题已改，见 evidence B8） | 01:32:05 |
| b-tc-news | https://techcrunch.com/2026/10/06/mistrals-new-1t-model-aims-to-leapfrog-closed-and-open-rivals/ | curl | 200，正文完整 | 01:32:04 |
| b-decoder | https://the-decoder.com/mistral-large-4-is-said-to-be-the-most-powerful-open-ai-model-from-europe-and-the-u-s/ | curl | 200，正文完整 | 01:32:04 |
| b-trendingtopics | https://www.trendingtopics.eu/mistral-large-4-artificial-analysis-ranking/ | curl | 200，正文完整 | 01:32:05 |
| b-vals-mistral-large-4 | https://www.vals.ai/models/mistralai_mistral-large-4 | curl | 200；Context Window 512k、Max Output 256k、Vals Index 48.05% | 02:02:53 |
| b-arxiv-cybench-abs / b-cybench-paper | https://arxiv.org/abs/2408.08926 ；https://arxiv.org/pdf/2408.08926 | curl；pdftotext | 摘要与全文（拒答段落、附录 N） | 02:02:57 起 |
| b-oembed-* | https://publish.twitter.com/oembed?url=…（Arena 2 条、Mistral 3 条、Lample 1 条、Mensch 2 条） | curl | 200；文字与公开日期（推文 ID 推算 UTC 时刻写在 evidence B3/B4） | 01:50:38–01:51:21 |
| b-arena-post-mirror-spke、b-arena-post-mirror-airebao | https://www.spke.com/en/ai/article/1624 ；https://airebao.com/item/41304 | curl | 200；aggregator 页，只用于找到 Arena 帖的 status 链接与完整帖文（二手 L5/L6） | 01:50:17 / 01:50:22 |
| b-wayback-doc-20261006131714 / …142631 / …20261007082913 | web.archive.org/web/<UTC 时间戳>id_/https://docs.mistral.ai/models/mistral-large-4-0 | curl（gzip 解压） | 200；前两个 49B，第三个 52B | 01:56 前后（CDX 查询在 01:46 前后） |
| b-wayback-blog-20261006132651 / …144255 / …20261007074543 | web.archive.org/web/<UTC 时间戳>id_/https://mistral.ai/news/mistral-large-4/ | curl（gzip 解压） | 200；均 49 billion；图片资源清单不同 | 01:58 前后 |
| b-assets/b-blog-*.webp（25 张）与 b-assets/b-blog-v0613-*.webp（5 张旧版） | https://mistral.ai/_astro/…（页面内图片资源，Mistral 站内 CDN 原文件） | curl | 200；旧版另有 SWE-Atlas、SciCode 两张已 404（旧版未取回） | 01:46–01:59 |
| b-assets/b-aa-article-*.png（5 张） | https://cdn.sanity.io/images/6vfeftx9/articles/…（AA 文章内图片） | curl | 200 | 01:46 前后 |

## 失败、拒绝与不可用

- **opencli**：01:34 `opencli doctor` 显示 Extension: not connected；按指引 `opencli daemon restart` 一次，等待 8 秒后仍未连接，**未再修改适配器、扩展或浏览器设置**，之后不再使用 opencli。
- **twitter-cli**：01:50 前后执行一次 `twitter search …`（只读）。它在未显式提供凭据时会**自动尝试读取本机浏览器 Cookie**，结果报 `not_authenticated`（DPAPI 解密失败），**没有读到任何 Cookie，没有输出任何凭据**；此后不再调用 twitter-cli。这一次自动读取不是我主动要求的，写在这里备查。
- **firecrawl**：`linkedin.com` 页与 `reddit.com` 页返回 “we do not support this site”；两次 `firecrawl_search`（01:42 前后）429。Exa `web_fetch_exa` 取 Reddit：SOURCE_NOT_AVAILABLE。
- **Reddit**：01:49:18 `https://www.reddit.com/r/MistralAI/comments/1wz22ti/.json`：HTTP 403（Reddit 反爬 HTML，`b-reddit-mistralai-thread.json` 实为 HTML）；01:49:28 `old.reddit.com` 同路径：跳转登录页（`b-reddit-old-json.json` 实为 HTML，非 JSON）。**分数未取得**；不使用 Cookie，不换代理域名。
- **VentureBeat**：01:32:05 curl 返回 Vercel Security Checkpoint（`b-vb.html`，429，非新闻正文，**无效存档**）；仅用了搜索引擎摘录。
- **web.archive.org**：01:46 前后第一次请求 `/wayback/available` 与 CDX 返回 429 或 “Temporarily Offline”（只对 docs 页），未循环重试；约 01:56 再查一次 CDX 成功，随后下载 6 个快照。
- **docs.mistral.ai 悬浮提示文字**不在 HTML 里（动态加载），改用 Playwright hover 读取（`b-hover-probe.mjs`），文字写在 evidence B1。
- 第一次 `b-browser.mjs` 调用博客标题（01:35:39）找不到元素，失败后改用 y 偏移，见下“截图”表。
- 01:42:58 Arena 榜单第二张截图失败（页面内部滚动容器，document 高度仅 1200），改用 `b-arena-shot.mjs` 先把行滚入视口再截图。

## 截图 15–24（`docs/2610/1007/images/`，宽均 ≤1400 px，PNG 不入库）

均为无登录态原站渲染，无抓取者导航、头像、扩展浮标；X 卡片只有帖子本身。QA 为我方目视逐张核对。

| 文件 | 内容 | 方式 / 尺寸 | 判定 |
|---|---|---|---|
| 15-b-blog-49b-active.png | 博客 “Frontier performance” 标题与三段（含 49 billion、competitive … open-weight、weights … reduced moderation） | Playwright 700 CSS，DPR 2；1400×1018 | 可用，边界落在段间 |
| 16-b-aa-cyber-index-chart.png | AA 官方文章里的 Cyber Index 图（上：总分与 Safety blocks；下：按评测分项；脚注 “* The model or provider declined some tasks on safety grounds”） | 原图 `b-assets/b-aa-article-67adb861.png`（2464×1904，SHA-256 30bd2d6c6ba2376c429a8b2673f2e129c159c706de28ceec2447aab47860d3d2）经 `b-chartimg.py` 缩至 1400×1082 | 可用，数字小但可读（原图在 `b-assets`） |
| 16-b-blog-title-preview.png | 博客顶部（桌面布局），含日期条 | Playwright 1000 CSS，DPR 1.4；1400×1162 | **不使用**：上方叠加了“0%”进度浮标与大片空白（懒加载图未出），按不删除要求保留 |
| 17-b-blog-preview-weights.png | 博客开头段（public preview、Weights drop end of this month） | Playwright 700，DPR 2；1400×264 | 可用 |
| 18-b-doc-52b-description.png | 文档页 Oct 6, 2026、PUBLIC PREVIEW、OPEN 徽章、“52B active parameters” 描述 | Playwright 700，DPR 2；1400×600 | 可用；上沿含 Compare 按钮与装饰猫 |
| 18-b-doc-weights-coming-soon.png | 文档 WEIGHTS 页签表：Coming soon / Coming soon / 1.05 / 52 / 1.6 / N/A | Playwright 700，DPR 2，点击页签；1400×458 | 可用；最右 “Context Size” 列被横向滚动遮住（文字在 `b-doc-tabs.json`） |
| 19-b-blog-cybench-chart.png | 博客 Cybench 图（对手均为开放权重；y 轴 60–100） | 原图 `b-assets/b-blog-cybersecurity-benchmarks---cybench_1_1dsNEh.webp`（2400×1582，SHA-256 c2012a17e49a0c39f528645d27de7740cb01597270091890914d3eee017b9721）经 `b-chartimg.py` 缩至 1400×923 | 可用 |
| 19-b-doc-price-card.png | 文档页 CONTEXT 1M 与价格卡（划线价；窄布局无输入/缓存/输出标签） | Playwright 700，DPR 2；1400×1120 | 备用（被 24 取代：24 有标签与悬浮提示）；下沿截进下一行页签的上缘 |
| 20-b-blog-cybench-near-zero.png | 博客 Cybersecurity 小节两段（82%、Cybench 93%、“near zero on the same test”） | Playwright 700，DPR 2；1400×1134；**第一次截图上沿切掉小标题，已覆盖重拍** | 可用 |
| 21-b-arena-post-card.png | Arena 官方 X 帖卡片（含 Pareto 图与引用的 Mistral 帖，时间 2:18 AM · Oct 7, 2026，311 点赞，24 回复） | X 官方嵌入端点，Playwright viewport 550×1500，DPR 2.5；1375×3268；**第一次视口高度 900 把卡片下半截掉，已覆盖重拍** | 可用；文字在 “Show more” 处折叠；卡片上沿未带导航、回复框或侧栏 |
| 21-b-arena-webdev-header.png | Code Arena WebDev 页头（标题、`Oct 6, 2026`、846,326 votes、141 models、前 3 行） | Playwright 700，DPR 2；1400×840 | 备用；页面日期在本机美东显示 Oct 6，SSR 文本为 Oct 7 |
| 22-b-arena-webdev-row45.png | 榜单第 40–50 行，含 mistral-large-4（45，34–56，1534 +21/-21）与 Opus 4.8（1536） | `b-arena-shot.mjs`，viewport 700×760，DPR 2，内部列表滚动到该行；1400×1520；**覆盖两次**（一次误试宽 1100 版，侧栏占画面、仍无票数列，已改回窄版） | 可用；窄屏无票数/价格列 |
| 23-b-aa-summary.png | AA 模型页标题、Proprietary、Intelligence #64/225 38、Speed #44/225 116.1、Cost、Verbosity | Playwright 700，DPR 2；1400×1400；**第一次高度 790 截进下一节文字，已覆盖重拍为 700** | 可用 |
| 24-b-doc-sale-tooltip.png | 文档页桌面布局：CONTEXT 1M、PRICE 划线价与 Input/Cached input/Output 标签，悬浮提示 “Launch pricing: 50% off for 2 weeks.” | `b-hover-shot.mjs`，viewport 1400×1000，DPR 1.65；1393×516；**第一次因取到零高元素导致宽度 40 px，已覆盖重拍** | 可用 |

覆盖说明：被覆盖的 20、22、23、24 的前一版都是我自己的截图失败品，已被后一版替换；没有删除任何文件。16-b-blog-title-preview.png 与 19-b-doc-price-card.png、21-b-arena-webdev-header.png 作为判废或备用图保留，同一编号下有两个文件（16、19、21），上传顺序以 `evidence-b.md` 引用的文件名为准。

## 隐私与交付边界

- 交付前对本组文件 grep `Develata|C:\Users\QQ|auth_token|ct0=|Bearer`：仅命中 Mistral 文档页自带的 curl 示例 `Authorization: Bearer $MISTRAL_API_KEY`（`b-doc.html` 与三份 `b-wayback-doc-*.html`，均为被 `.gitignore` 排除的 HTML 原件）。无抓取者头像、显示名、handle；X 内容只有公开推文文字与卡片截图。
- 文本存档（txt/json/md/jsonl）全部小于 1 MB（最大 `b-hn-main.json` 581,021 B；`b-cybench-paper.txt` 530,714 B）。大于 1 MB 且已被 `.gitignore` 排除的有：`b-aa.html`（4.0 MB）、`b-doc.html`、`b-pricing.html`、`b-doc-lifecycle.html`、`b-arena-webdev.html`、`b-wayback-doc-*.html`、`b-cybench-paper.pdf`（3.5 MB）。图片全部不入库。
- `b-reddit-mistralai-thread.json` 与 `b-reddit-old-json.json` 扩展名为 json 但内容是 Reddit 的拦截页 / 登录页 HTML（无有用内容；若不想入库可由 Claude 排除）。
- 未改动其他组文件；未 commit、push、删除。`git status --short` 只显示 `docs/2610/1007/` 与既有的未跟踪项。
