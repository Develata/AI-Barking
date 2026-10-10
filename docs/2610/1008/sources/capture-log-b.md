# 1008 B 组抓取日志（Anthropic 使用政策更新）

时间均为北京时间 UTC+8，日期 2026-10-09（本机原时区 EDT，仍是 10-08，北京 = 本机 + 12 小时）。分钟级为近似，取自命令输出；逐次截图台账见 `b-shots-log.jsonl`（Tesseract 裁切）与 `b-browser-records.jsonl`（Playwright）。**HTTP 200 不等于正文有效**，有效性以“结果”列为准。

## 工具与方式

- HTTP：`curl -sS -L`，匿名，普通 Chrome UA；转文本用 `docs/2610/1007/sources/a-html2txt.py`（去 script/style，保留 href）。
- 渲染与截图：`b-browser.mjs`（由 1007 期同名脚本改写：路径改 1008、图名放宽为 `NN-name.png`、仅允许 15–24 区间），Playwright Core + 本机 Chrome 无头，每次新临时 profile，不登录、不带 Cookie；CSS 宽 700、DPR 2（即 1400 px 宽）；可选 `NOARCHIVE`、`OVERWRITE` 环境变量。`b-shot.py`（改自 1007 `a-shot.py`，headless Chrome + Tesseract 锚点裁切）只用于图 15、16。
- 遮挡处理：The Verge、The Decoder 的 Cookie/同意弹窗在整页截图里盖住正文。做法是页面加载后用 `initJS` 把 `position:fixed` 与高 z-index 的遮罩元素 `display:none`，**没有点击“同意/Agree”，没有接受任何 Cookie**。结果：23、24 的版面已去掉弹窗，正文原样。
- 搜索：firecrawl_search、WebSearch；HN 用 Algolia API（`search`、`search_by_date`、`items`）。
- 未使用 opencli、任何会读取本机浏览器 Cookie 的工具。agent-reach：只运行了只读诊断 `agent-reach doctor --json`（约 10:04），Reddit 项为 warn、`active_backend` 为 null——后端为 OpenCLI（扩展未连接，且依赖浏览器登录会话）与 rdt-cli（本机未安装；按协议不装、不用会读本机浏览器 Cookie 的后端），因此没有可用的免登录后端，Reddit 未经 agent-reach 取。未登录，未输入凭据，未发帖、点赞、评论、关注。付费墙未绕过。X、Facebook、Threads、Instagram 的帖子未抓取。

## 有效抓取（文件基名 | URL（已去查询参数）| 工具 | 结果 | 北京时间）

| 文件基名 | URL | 工具 | 结果 | 时间 |
|---|---|---|---|---|
| b-announce | https://www.anthropic.com/news/2026-usage-policy-update | curl | 200，119,391 B，正文完整 | 09:37 |
| b-aup | https://www.anthropic.com/legal/aup | curl | 200，888,561 B，正文完整，顶部 “Effective November 12, 2026” | 09:37 |
| b-end-subset | https://www.anthropic.com/research/end-subset-conversations | curl | 200，109,751 B，正文完整，日期 Aug 15, 2025 | 09:37 |
| b-aup-prev | https://www.anthropic.com/legal/archive/22742366-2ef0-4c7a-a833-6523f10d3944 | curl | 200，旧版（Effective September 15, 2025），页内写 “replaced by a newer version” | 09:40 |
| b-aup.pdf / b-aup-pdf.txt | https://www-cdn.anthropic.com/files/4zrzovbb/website/6ae2f5bb8675bcd9fac006125817ba1393b396e4.pdf | curl；poppler `pdftotext -enc UTF-8 -layout` | 200，165,711 B，8 页，“Effective Date: November 12, 2026” | 09:40 |
| b-hn-search1 / b-hn-50008565 / -50008678 / -50013793 / -50013897 / -44916813 | hn.algolia.com/api/v1/… | curl | 200 JSON；50008565 为 Verge 帖，评论树完整；44916813 为 2025-08 研究页的 HN 帖（259 分，存档，未细读） | 09:41–10:02 |
| b-verge | https://www.theverge.com/ai-artificial-intelligence/1008100/anthropic-new-usage-policy-abuse-claude | curl | 200，正文完整 | 09:42 |
| b-techcrunch | https://techcrunch.com/2026/10/08/anthropic-changes-usage-policy-to-ban-model-abuse-and-election-interference/ | curl | 200，正文完整（短稿） | 09:42 |
| b-pascal | https://blog.pascalschuster.de/article/on-model-welfare-and-usage-policies | curl | 200，完整（个人博客，非线索主体） | 09:42 |
| b-decoder / b-decoder-end | https://the-decoder.com/being-mean-to-claude-can-now-get-your-account-suspended-under-anthropics-new-tos/ ；https://the-decoder.com/claude-models-can-now-end-conversations-with-abusive-users/ | curl | 200，完整 | 09:44 |
| b-qz / b-macrumors / b-fmt / b-explainx / b-explainx2 / b-jin10 / b-digg / b-yahoo / b-ithome / b-sina | 见 `evidence-b.md` 各行 URL；explainx 为 https://explainx.ai/blog/claude-insult-viral-post-anthropic-chat-ended-policy-2026 与 …/anthropic-2026-usage-policy-abusive-behavior-toward-claude-ban-weapons-surveillance-2026；digg 为 https://digg.com/tech/lcbyuqam；yahoo 为 https://www.yahoo.com/news/politics/articles/anthropic-bans-abusive-behavior-toward-185424349.html | curl | 200，正文有效（explainx 第一篇是 2026-05-02 的旧文，与本条无直接关系，仅存档；digg、yahoo 为聚合/转载，未用） | 09:44 |
| b-constitution | https://www.anthropic.com/constitution | curl | 200，完整 | 09:47 |
| b-cc-tools | https://code.claude.com/docs/en/tools-reference | curl | 200，含 “EndConversation tool behavior” 一节 | 09:46 |
| b-cc-changelog-excerpt | https://code.claude.com/docs/en/changelog | curl（整页 4.7 MB，1 MB 文本；只留摘录，整页已删，见下） | 200，2.1.214 “July 18, 2026” 条目有效 | 09:45 |
| b-systemcard-excerpts | Claude Opus 4.7 / 5.5 System Card PDF（URL 见 `evidence-b.md` B3g） | curl 到系统临时目录；poppler 转文本；摘录入库，PDF 本体（14 MB、18 MB）不存 | 200，摘录有效（Opus 4.7 卡日期 April 16, 2026） | 09:46 |
| b-36kr | https://www.36kr.com/p/4017915543310468 | curl | 200，正文完整（新智元稿，页面 2026年10月09日 09:22） | 09:55 |
| b-knews / b-tvrain | https://knews.media/2026/10/08/anthropic-to-enforce-user-bans-for-abuse-of-claude-ai/ ；https://tvrain.tv/news/anthropic-zapretila-oskorbljat-i-zhestoko-obraschatsja-s-claude-576988/ | curl | 200；knews 全文三句；tvrain 为俄文，存档未读 | 09:55 |
| b-sohu | https://m.sohu.com/a/1085353389_122014422 | curl | 200，短稿（IT之家转载） | 09:55 |
| b-techmeme-excerpt | https://www.techmeme.com/261008/p39 | curl（整页 460 KB 已删，只留摘录） | 200，摘录有效 | 09:56 |
| b-engadget / b-gizmodo / b-dexerto / b-newser / b-runtimewire | 见 `evidence-b.md` | curl | 200；gizmodo、dexerto、newser、runtimewire 正文有效；engadget 只取到标题与元数据，正文未细读 | 09:58 |
| b-aup-pw / b-endsubset-pw / b-cctools-pw / b-verge-pw / b-decoder-pw（.json、.txt） | 同上各页 | Playwright 700 px 渲染，页面文本、meta、布局 | 渲染文本有效；json 含元素布局坐标，供截图定位 | 09:47–09:57 |

## 截图（`docs/2610/1008/images/`，2x，宽 1400 px）

| 文件 | 来源页 | 方式 | 目视确认 |
|---|---|---|---|
| 15-announce-abuse-section.png | 公告（小标题 + 上一节末段 + 虐待段全文） | b-shot.py crop（Tesseract 锚点） | 已确认：小标题、两段原文完整，首尾落在行间 |
| 16-announce-top-effective.png | 公告开头（标题、日期、两段、生效日句） | b-shot.py crop | 已确认 |
| 17-aup-abuse-clause.png | AUP “Do Not Engage in Cruel, Abusive, or Psychologically Harmful Conduct” 整节，含末条 | b-browser.mjs | 已确认；末行下缘无截断 |
| 18-aup-effective-enforcement.png | AUP 标题、Effective November 12, 2026、适用范围、执行条款、实时防护段 | b-browser.mjs | 已确认 |
| 19-endsubset-intro-welfare.png | 研究页标题、日期、前两段（福利不确定性） | b-browser.mjs | 已确认 |
| 20-endsubset-last-resort.png | 研究页 “last resort”、示意图、结束后可开新对话/编辑重发 | b-browser.mjs | 已确认 |
| 21-cc-endconversation-behavior.png | Claude Code 文档 “EndConversation tool behavior” 开头 | b-browser.mjs（y 定位） | 已确认 |
| 22-cc-endconversation-scope.png | 同页 “The tool appears only when…” 列表 | b-browser.mjs | 已确认 |
| 23-verge-enforcement-no-comment.png | The Verge 两段（含 “Anthropic did not provide a comment…”）；中间夹一则页面广告，未删 | b-browser.mjs + 去遮罩 | 已确认 |
| 24-decoder-headline-suspend.png | The Decoder 标题、署名日期、前三段 | b-browser.mjs + 去遮罩 | 已确认；页顶左右两条蓝色边带是页面自带版式 |

截图未含抓取者的账号信息（均为匿名未登录渲染）。归档文本已 grep：`develata`、`gmail`、本机用户路径无命中（Techmeme 页的 gmail 命中是页面上无关的 Google 新闻，该整页已删）。

## 失败、被拒与放弃

- Reddit：agent-reach doctor 显示无可用后端（见上）；`old.reddit.com`、`www.reddit.com` 的 `.json` 与网页均 403（challenge 页，约 190–330 KB 垃圾内容）；firecrawl 回 “we do not support this site”；pullpush 之类第三方镜像按规则不用。→ B6b、B8 的 Reddit 部分缺失。
- The Telegraph（https://www.telegraph.co.uk/business/2026/10/08/anthropic-bans-users-from-abusing-claude-chatbot/）：HTTP 402（付费墙），未绕过。
- Firstpost：403。pasqualepillitteri.it 英文页：schannel 连接被中断（搜索摘要可见，未存档）。
- X、Facebook、Threads、Instagram：只在搜索结果里见到标题，未抓取。
- Anthropic 署名员工解释（B4）与帮助中心条目（B4b）：搜索无命中，见 evidence-b.md。
- 说明：firecrawl 搜索摘要里 The Decoder 一文曾出现 “told The Verge” 字样，但存档页面 `b-decoder.txt` 没有，证据以存档为准。
- 图 23 中间有一则 Comcast 页面广告（页面上下文，未删）。若需要干净的批注底图，可用 `b-shot.py stack` 把两个锚点区间拼接（每段保持原像素），本次未做。
- 第一次对 The Verge 的 Playwright 渲染因 `page.goto` 60 秒超时失败（`b-browser-records.jsonl` 有记录），重试成功。
- 第一版 23、24 号图因 Cookie/同意弹窗遮住正文被删除重做；17–20 号图为裁边（截掉半行字）已 `OVERWRITE` 重做；15 号图首版下缘切到下一小标题，已删除重做并同步删去 `b-shots-log.jsonl` 里那一行。

## 自己生成后删除的试验文件

- `docs/2610/1008/sources/b-reddit-technology.json`（Reddit 403 challenge 页，已删）
- `b-cc-changelog.html/.txt`（Claude Code 变更日志整页，4.7 MB / 1.0 MB，已删，保留 `b-cc-changelog-excerpt.txt`）
- `b-techmeme.html/.txt`（已删，保留 `b-techmeme-excerpt.txt`）
- 临时目录（不在仓库内）：`%TEMP%\b1008\`（渲染中间图）、`/tmp/rtest.out`、`/tmp/sc47.*`、`/tmp/sc55.*`。

## 假设

- 北京时间换算：UTC + 8；EDT = UTC − 4。页面只给日期的，写“官方未给时刻”并另注 meta。
- 公告与 AUP 页面内容以抓取当时（北京 10-09 09:37–10:05）为准；公告 modified_time 为 17:00:30Z，未见之后改动，但未做逐字再比对。
- Verge 页面时间 “1:00 PM EDT” 视为 17:00Z（与 meta 一致）。
- 文件大小：入库文本均小于 1 MB（最大 `b-cctools-pw.json` 约 130 KB、`b-hn-44916813.json` 约 274 KB）；HTML/PDF 原件仅存本地，不入库（`b-aup.pdf` 166 KB、`b-aup.html` 889 KB 等）。
