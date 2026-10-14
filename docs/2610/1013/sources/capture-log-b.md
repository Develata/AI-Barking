# 1013 期 B 组抓取日志（OpenAI 会议室隐藏录音设备案）

时间为北京时间（UTC+8）；本机美东（EDT）= 北京 − 12 小时；UTC = 北京 − 8 小时。时刻取自命令的 `date -u` 读数与 `b-browser-records.jsonl`，为近似值（本机时钟在会话中有一次约两小时的跳变，17:2x 之后的记录以 JSONL 里的 `time_bj` 为准）。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。

工具：curl（浏览器 UA，匿名）；Playwright Core + 本机 Chrome（匿名、全新临时 profile、不登录、不带 Cookie、en-US，UA 去掉 Headless 标记）；fxtwitter API；CourtListener 公开搜索 API（未登录）；HN Algolia 与 Firebase；pdftoppm / pdftotext / tesseract 5.5（OCR）；Pillow（经 `uv run --no-project --with pillow`，因 Scoop python 的 Pillow 缺编译件）；WebSearch（6 次）；firecrawl_search（仅 1 次，2 credits，未用 scrape）。未使用 opencli / agent-reach，未登录任何站点，未运行读取浏览器 Cookie 的工具。

**当事人隐私**：含姓名的 HTML/PDF 原件只存本地（被 `.gitignore` 排除）；入库的 txt/json 已用 `b-redact.py` 把姓名（含中文音译）替换为“[当事人]”，国籍词保留原文；两份脚本里的姓名模式已拆开书写，使脚本自身不含姓名。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 16:56 | justice.gov NDCA 新闻稿 | curl | 200 但仅 2.5 KB：Akamai Bot Manager 的 JS 互动页（`bm-verify`，非 CAPTCHA），**无正文**；试验文件已删除（自己刚生成的） | — |
| 16:56 | 同上 | Playwright（`b-fetch.mjs`） | 200，浏览器自动完成该 JS 校验，正文 8.3 KB；未做任何额外绕过 | `b-doj-pr.txt`（姓名已替换）、`b-doj-pr.html`（本地） |
| 17:10 | justice.gov OPA 新闻稿 | `b-fetch.mjs` | 200 | `b-doj-opa-pr.txt`、`.html` |
| 约 17:00 | CourtListener 搜索 API `type=r&q=<当事人姓氏>`（及 `q=OpenAI "intercept oral"` 无果）；`type=rd&q=docket_id:74958115` | curl | 200；5 个 docket（含同案号的重复 docket 74958184，仅 Doc 1-1、3、8）；`/api/rest/v4/dockets/74958115/` 401（需认证）；`/docket/74958115/` 网页 403 | `b-courtlistener-search-recap.json`、`b-courtlistener-docket-74958115-documents.json`（姓名已替换） |
| 约 17:00 | `storage.courtlistener.com/recap/gov.uscourts.cand.480828/…1.0.pdf`、`…1.1.pdf`、`…8.0.pdf`；`480829.1.1/3.0/8.0`（同案重复 docket） | curl | 200；主文 11 页（第 1 页文字层，第 2–11 页扫描图像）；其余为同一文件 | 本地 `b-complaint-3-26-mj-71329-doc1.pdf`、`-doc1-1-penalty-sheet.pdf`、`-doc8-org-victim.pdf`；文字版 `b-complaint-3-26-mj-71329-ocr.txt`（OCR + 两表人工转录，姓名已替换）。480829 系的重复件仅存于 scratchpad，未入库 |
| 约 17:00 | fxtwitter：OpenAI Newsroom 帖 `2110095403198849132`；FBI 局长帖 `2110109949359751665` | curl | 200 | `b-x-openainewsroom-2110095403198849132.json`、`b-x-fbidirector-2110109949359751665.json`（未登录取数，无抓取者信息） |
| 约 17:01 | X 官方嵌入页 `platform.twitter.com/embed/Tweet.html?id=…`（第一次：带 `locale/UA` 参数，`waitUntil: domcontentloaded`） | Playwright | 渲染**空白页**（失败），scratchpad 试验图未入库 | — |
| 约 17:02 | 同上（第二次：沿用 1012 `a-xshot.mjs` 做法，`networkidle`） | Playwright | 成功，卡片折叠显示前 6 行（“Show more”） | `images/15-openai-newsroom-post.png` |
| 约 17:02 | `https://x.com/OpenAINewsroom/status/2110095403198849132` 直接页 | Playwright | `ERR_HTTP_RESPONSE_CODE_FAILURE`（失败，未再试） | — |
| 16:58–17:06 | 媒体页（curl，浏览器 UA）：The Verge、CNBC、TechCrunch、NBC News、CBS、ABC7（AP）、Fortune、The Hacker News、Fox Business、CBC、KELO（Reuters 转载）、NBC Bay Area（AP） | curl | 全 200 | `b-m-<站>.html`（本地）、`.txt`（姓名已替换）、`.headers` |
| 17:05 | Reuters 原页 | curl；后用 Playwright（`b-fetch.mjs`） | curl 401；Playwright 也 401、正文 0 字（失败）；空文件已删除 | — |
| 17:05 | Washington Post 原页 | curl；Playwright | curl 60 秒超时（失败）；Playwright 200 但付费墙，仅前两段（2.1 KB）；`b-wapo.txt`/`.html` 保留 | `b-wapo.txt`、`.html` |
| 17:07 | Wired、The Guardian、BBC、decodingvibes（博客）、Yahoo（Reuters 转载）、Engadget、Interesting Engineering、Help Net Security、technology.org、kingy.ai、SF Standard | curl | 全 200 | 同上 |
| 约 17:10 | Bloomberg Law；ABC News（AP 全文）| Playwright | Bloomberg Law 付费墙，只到前两句；ABC News 200 | `b-bloomberglaw.txt`/`.html`、`b-abcnews.txt`/`.html` |
| 约 17:12 | 中文媒体：21经济网（财联社稿）、凤凰网财经、新浪（IT之家微博页）、虎嗅、Odaily、凤凰网科技、凤凰网原创 | curl | 全 200；凤凰网财经页正文懒加载，仅存标题与时间；新浪页是微博聚合页，含用户帖 | `b-m-21jingji-herald`、`b-m-ifeng-guojia`、`b-m-ifeng-tech`、`b-m-ifeng-ai`、`b-m-sina-weibo`、`b-m-huxiu`、`b-m-odaily` |
| 17:18–17:20 | DOJ NDCA 页截图 13、21、22 | `b-browser.mjs`（DOM 黑条遮盖姓名与国籍词，隐藏 fixed/sticky 元素） | 先后三次：第一次 13 号锚点误中面包屑，22 号有悬浮 “TOP” 按钮遮字（均为我自己生成的试验图，已用 `OVERWRITE=1` 覆盖）；第二次隐藏 sticky 后元素高度为 0 失败；最终成功 | `images/13-…`、`21-…`、`22-…`；参数 `b-shots-doj.json` |
| 17:20 | SF Standard 页截图 23 | `b-browser.mjs` | 成功 | `images/23-sfstandard-openai-account.png`；`b-shots-sfstandard.json` |
| 约 17:12–17:25 | 控告书 PDF 渲染与裁切 14、16–20 | `b-pdfshot.py`（pdftoppm 326 dpi + tesseract 词框黑条遮盖 + 缩到宽 ≤1400） | 反复调整裁切：17 号先因锚点命中 ¶13 里同样的句子失败、后又因偏移量过大切掉表格末行；20 号 Figure 4 一度被切；18 号表头 “[姓名] Query”（粗体）OCR 漏识别，用 `--blackbox` 补遮；均为我自己生成的试验图，最终版已覆盖并目视确认 | `images/14-…`、`16-…`–`20-…` |
| 17:22 | HN Algolia 搜索（3 组关键词）；`items/<id>`（10 个帖 + 主讨论串 50078141）与 Firebase `item/<id>` | curl | 200 | `b-hn-<id>.json`、`b-hn-<id>-fb.json`（姓名已替换）、`b-hn-50078141-comments.txt`（评论展平，用户名省略） |
| 17:22 | Reddit 搜索 `search/?q=OpenAI+recording+device+FBI&sort=top&t=day` | Playwright | 200，页面带一次自动 JS 校验跳转（`js_challenge`），由浏览器自行完成，未做额外绕过；匿名页只显示帖子票数与评论数，**不显示评论得分**；记录里的校验 token 已去除（`final_url` 只留 `?[query omitted]`）；第一次抓的 HTML 被我误删（自己刚生成），为取帖子链接重抓一次并**保留该 HTML 本地存档**，其重复 txt 与重复记录已删 | `b-reddit-search-all-top.txt`（姓名已替换）、`.html`（本地） |
| 19:34–19:35 | Reddit 帖：r/OpenAI 1x57ghe、r/BetterOffline 1x5dhyy、r/singularity 1x56x7s | Playwright | 200 | `b-reddit-r-*.txt`（姓名已替换）、`.html`（本地） |
| 19:36 | AIHOT 首页与 `/hot` | Playwright | 200；本条排第 2，热度 457 | `b-aihot-home.txt`、`b-aihot-hot.txt`（姓名已替换；HTML 已删） |
| 19:41 | HN Firebase 复取（11 帖）与 topstories；fxtwitter 复取互动数 | curl | 200 | `b-hn-final-poll.txt`；X 数字写入 evidence-b B3/B5 |
| 多次 | WebSearch ×6（英中各若干）、firecrawl_search ×1（目的：找到 DOJ 新闻稿） | — | 返回二手摘要与链接；摘要中的“细节”一律回到原页核对后才采信（例如搜索摘要称“OpenAI 通过例行排查发现”，原页为 technical surveillance countermeasures sweep，来自 OpenAI 对 Fortune/SF Standard 的说法） | 无 |

## 2. 失败 / 受限记录

- Reuters 原页：HTTP 401（curl 与浏览器均是），以 KELO 转载版（Thomson Reuters 稿）与 NBC News/CBC/Yahoo 转载版代替；发布时刻来自 KELO JSON-LD，注明为转载版。
- Washington Post、Bloomberg Law：付费墙，不绕过；只读到前两段/前两句。FT、WSJ、NYT：未检到该案报道，未抓。
- X 帖子完整折叠文：嵌入卡片折叠，x.com 直链打不开；全文以 fxtwitter JSON 为准。
- DOJ 新闻稿发布时刻：官方页只有日期，未给时刻（`<time>` 为 12:00Z 占位）。
- CourtListener：docket 网页 403、REST dockets 401，改用搜索 API 的 `type=rd` 取案卷条目；PDF 经 `storage.courtlistener.com` 直接下载成功。RECAP 为第三方镜像，非 PACER 原件，案卷快照约 2026-10-14 08:05Z。
- Reddit 匿名页无评论得分；HN 评论得分不公开。
- 当事人 CCTV 截图（DOJ 页与控告书 Figure 3）不截，不含人像。
- 凤凰网财经页正文未抓到（懒加载）。

## 3. 本组新增文件

文本（可入库，姓名已替换）：`evidence-b.md`、`capture-log-b.md`、`b-fetch.mjs`、`b-browser.mjs`、`b-pdfshot.py`、`b-redact.py`、`b-shots-doj.json`、`b-shots-sfstandard.json`、`b-browser-records.jsonl`、`b-doj-pr.txt`、`b-doj-opa-pr.txt`、`b-complaint-3-26-mj-71329-ocr.txt`、`b-courtlistener-*.json`、`b-x-*.json`、`b-hn-*.json`、`b-hn-50078141-comments.txt`、`b-hn-final-poll.txt`、`b-reddit-*.txt`、`b-aihot-*.txt`、`b-m-*.txt`、`b-m-*.headers`、`b-wapo.txt`、`b-bloomberglaw.txt`、`b-abcnews.txt`。
**只存本地、不入库（已被 `.gitignore` 排除）**：`b-*.html`、`b-m-*.html`、`b-complaint-3-26-mj-71329-*.pdf`（含姓名）、`images/13–23-*.png`。
单个文件大于 1 MB 的：`b-complaint-3-26-mj-71329-doc1.pdf`（2.4 MB，PDF，不入库）、`b-m-wired.html`（约 1.4 MB，HTML，不入库）；文本类文件均小于 1 MB（最大 `b-hn-50078141-comments.txt` 约 107 KB，`b-hn-50078141.json` 约 222 KB）。

## 4. 交付前自查

- 对 `docs/2610/1013/` 里 `b-*`、`evidence-b.md`、`capture-log-b.md` grep 当事人姓名（英文各拼写与中文音译）：除本地 `.html` 与 `.pdf` 原件外无命中。**他组文件 `docs/2610/1013/sources/a-aihot-hot.txt`（A 组）含当事人姓名**（AIHOT 摘要里的姓名，本组未动该文件），合并前请处理。
- 对本组文件 grep 本机用户名、邮箱、`C:/Users`、`AppData`、维护者账号标识：无命中。X 抓取只有匿名 fxtwitter JSON 与官方嵌入卡片，无抓取者头像/昵称；截图 15 卡片中的 “Follow” 为匿名页原样。
- 11 张截图（13–23，不含 24）已逐张打开目视确认：文字清楚，黑条覆盖了全部姓名与国籍词（13、17、18、19、20、21、22、23），14、16 无姓名，15 为官方帖卡片（折叠）。
- `git status --short`：除他组与派工单外，本组只新增 `docs/2610/1013/` 下文件；未改动仓库其他文件；未 commit、未 push、未删除他人文件（删除的只有我自己生成的试验文件，见第 1 节）。
- 协调方 10/14 要求“剩下的截图改用 opencli”：本组 13–23 号在该要求到达前已全部用 Playwright/pdftoppm 完成，未重做；24 号空缺未补。
