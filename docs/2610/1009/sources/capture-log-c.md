# 1009 C 组抓取日志（Claude Science 紫外线全天图）

时间为北京时间（UTC+8），2026-10-10；本机 EDT 比北京晚 12 小时（本机 10-09 23:33 = 北京 10-10 11:33）。分钟级为近似，取自命令输出与脚本记录。**HTTP 200 不等于拿到正文**，有效性以“结果”列为准。

## 工具与方式

- HTTP：`curl -sS -L`，匿名，普通 Chrome UA；转文本用 `docs/2610/1008/sources/a-html2txt.py`（只读调用，不改原文件；`python3 -X utf8 -I`）。
- firecrawl（MCP `firecrawl_scrape` / `firecrawl_search`）：用于 curl 与 headless Chrome 被 Cloudflare 拦下的页面（menard.pha.jhu.edu、physics-astronomy.jhu.edu、36kr、newsbytes）与 PDF 文本提取。返回的 rawHtml/markdown 由我写盘（`c-*.html`、`c-*.md`）。
- 渲染与截图：`c-browser.mjs`（由 1008 `b-browser.mjs` 改写：路径改 1009、前缀改 `c-`、图名限定 25–34；新增环境变量 `HEADED`（有头 Chrome）、`SCROLL`（逐段滚动触发懒加载）、`JUMP`（按截图锚点先滚动）、`<summary>` 选择器），Playwright Core + 本机 Chrome，每次新临时 profile，不登录、不带 Cookie；CSS 宽 700、DPR 2（即 1400 px 宽）。逐次记录见 `c-browser-records.jsonl`。
- 未使用 opencli，未使用任何读取本机浏览器 Cookie 的工具，未登录、未输入凭据、未接受任何 Cookie 弹窗、未发帖点赞评论。
- **Cloudflare 处理**：menard.pha.jhu.edu 与 physics-astronomy.jhu.edu 对 curl 返回 403 “Just a moment...”挑战页（已删除这两份无效文件，见“试验文件”）。headless Chrome（`--headless=new`）返回 “Attention Required! | Cloudflare” 拦截页。未尝试破解或绕过人机验证；改用两种常规访问：firecrawl 抓取，以及无头模式之外的**有头（headed）Chrome**（普通访问，没有出现交互式验证）。有头 Chrome 下页面 HTML 正常；图片请求在短时间内连续加载后，部分被 Cloudflare 返回 403（`c-` 脚本最初几次截图里出现破图），改为只在截图位置附近滚动、等待图片加载（`JUMP`/initJS 滚到目标图片处并等 6 秒）后再截，最终入库的截图图片均已目视确认完整加载，无破图、无占位。
- 应用内浏览器（Claude Browser pane）能打开技术页，但无法把截图保存为文件，也被安全限制禁止打开 reddit.com；仅用来确认技术页是真实浏览器可正常访问的，没有从中提取内容。
- 搜索：firecrawl_search；HN 用 Algolia API。

## 有效抓取（文件基名 | URL（已去查询参数）| 工具 | 结果 | 北京时间）

| 文件基名 | URL | 工具 | 结果 | 时间 |
|---|---|---|---|---|
| c-anthropic-map（.html/.txt） | https://www.anthropic.com/research/the-missing-map-of-the-sky | curl | 200，133,839 B，正文完整；`published_time` 2026-10-08T20:59:00.000Z，`modified_time` 2026-10-09T09:29:17.000Z | 11:33 |
| c-anthropic-map-pw（.json/.txt） | 同上 | Playwright headless | 渲染文本 12,036 字，含元素布局坐标（供截图定位） | 11:50 |
| c-menard-uvmap（.html/.md/.txt） | https://menard.pha.jhu.edu/uvmap/ | firecrawl_scrape（markdown + rawHtml，maxAge 0） | 200，rawHtml 53,778 B；markdown 19,523 字，含所有折叠节文字。**非原站直连 HTML**：curl 与 headless Chrome 被 Cloudflare 拦截 | 约 11:40 |
| c-uvmap-pw（.json/.txt） | 同上 | Playwright 有头 Chrome | 200，页面渲染文本 10,212 字（折叠节收起状态），含布局坐标；证明有头 Chrome 可访问 | 11:53 |
| c-uvmap-manuscript.md | https://menard.pha.jhu.edu/uvmap/docs/uvmap_manuscript.pdf | firecrawl_scrape（parsers pdf） | 41 页的文本提取，245,429 字符；首页 “Draft, October 2026”。**PDF 原件未存**：curl 与有头 Chrome 的 `context.request.get` 直连均 403 | 约 12:00 |
| c-jhu-news（.html/.md/.txt） | https://physics-astronomy.jhu.edu/2026/10/09/brice-menard-works-with-anthropics-claude-science-to-produce-first-complete-uv-light-map-of-the-sky/ | firecrawl_scrape | 200；meta published 2026-10-09T15:55:59-04:00，modified 15:56:02-04:00；正文仅一段。curl / headless Chrome 403 | 约 11:40 |
| c-mast-uvbkgd | https://archive.stsci.edu/prepds/uv-bkgd/ | curl | 200，51,793 B | 约 12:05 |
| c-mast-fims、c-mast-fims-news | https://archive.stsci.edu/missions-and-data/fims-spear ；https://archive.stsci.edu/contents/newsletters/april-2023/fims-spear-mission-data-now-available | curl | 200，正文有效 | 约 12:12 |
| c-galex-overview、c-galex-techdoc2 | https://asd.gsfc.nasa.gov/archive/galex/Documents/MissionOverview.html ；https://www.galex.caltech.edu/researcher/techdoc-ch2.html | curl | 200，有效（techdoc 含 “patchy coverage” 句） | 约 12:12 |
| c-decoder | https://the-decoder.com/anthropics-claude-science-creates-the-first-complete-ultraviolet-map-of-the-sky/ | curl | 200，有效；published 2026-10-09T09:22:25+00:00 | 约 12:08 |
| c-36kr（.html/.md） | https://36kr.com/p/4018549788168066 | curl 返回“正在进行安全检测”挑战页（已删）；改用 firecrawl_scrape | firecrawl 200，正文有效（新智元稿，页面写 2026年10月09日 19:44） | 约 12:09 |
| c-qq | https://news.qq.com/rain/a/20261009A04CA300 | curl | 200，有效（赛博禅心，“2026-10-09 10:33发布于广东”） | 约 12:08 |
| c-newsbytes（.html/.md） | https://www.newsbytesapp.com/news/science/anthropic-claude-science-creates-1st-ever-complete-ultraviolet-sky-map/story | curl 403（已替换）；firecrawl_scrape | firecrawl 有效 | 约 12:09 |
| c-superpower、c-zeniteq、c-aibase、c-alphasignal、c-lifeboat、c-explainx、c-cellcog | 见 `evidence-c.md` 第四节各行 URL | curl | 200，正文有效；转载/二手站，仅作媒体表述与夸大实例 | 12:08–12:12 |
| c-hn-50019627.json | https://hn.algolia.com/api/v1/items/50019627 | curl | 200，评论树 2 条 | 11:35 |
| c-search-snippets.md | firecrawl_search 的结果清单 | firecrawl_search | 见该文件 | 约 12:00–12:15 |

## 失败或不完整的抓取

- menard.pha.jhu.edu、physics-astronomy.jhu.edu：curl 403（Cloudflare “Just a moment...”）；headless Chrome 403（“Attention Required!”）。已改用 firecrawl 与有头 Chrome，如上。原站直连 HTML 原件因此没有存，存的是 firecrawl 返回的 rawHtml。
- uvmap_manuscript.pdf、buildup_sequence.pdf（25 MB）：PDF 原件**未存**。手稿只有 firecrawl 的文本提取；buildup PDF 未取。
- 36kr 直连、newsbytes 直连：挑战页/403，改用 firecrawl。
- Reddit（r/singularity 1x11m9i、r/astrophysics 1x137gi 等）：curl 403；有头 Chrome 返回 “Prove your humanity”验证页或 “blocked by network security”，old.reddit.com 要求登录；firecrawl “not supported”；应用内浏览器禁止打开 reddit.com。**不绕过**，分数与评论未取得。仅有 firecrawl 搜索结果列出的帖子标题与摘要。
- Anthropic 官方 X 账号帖子（status/2108290395599667700）：X 帖未抓取（只可能在登录态或需要访问 X，且按规则不存抓取者信息）；发帖时刻来自 alphasignal（20:16:15Z）与 cellcog（“20:16 UTC”）二手，未经一手核实。
- ADS（Jo et al. 2017）：curl 返回 405，仅有搜索摘要一句 “covering ∼76% of the sky”（FIMS/SPEAR H2 荧光图），不入证据表。
- arXiv API（查 Murthy 2014）：“Rate exceeded”，未取；改用 MAST 页面与 IOP 摘要的搜索片段（IOPscience 页未抓）。

## 截图（`docs/2610/1009/images/`，2x，宽 1400 px）

| 文件 | 来源页 | 工具 | 说明 |
|---|---|---|---|
| 25-hero-caption.png | Anthropic 页 | Playwright headless（SCROLL） | 标题、日期、导语、首图（Mollweide UV 全天图）与图注；`y` 110 起 922 CSS px。图下方控件说明行被页面自身裁掉一半，是页面行为 |
| 26-ten-percent.png | Anthropic 页 | 同上 | “To check how accurate…” 段 + “Finally, on top of…” 段；`y` 3855 起 322 |
| 27-disc-artifact.png | Anthropic 页 | 同上（SCROLL，触发懒加载） | “Trial and error” 小节三段 + 对比图 + 图注；`y` 4555 起 845。第一次截图因懒加载图为空白，已重截并目视确认 |
| 28-pipeline-dag.png | Anthropic 页 | 同上 | DAG 图 + 图注；`y` 4195 起 340，同样重截 |
| 31-fill-steps.png | Anthropic 页 | 同上 | “Building the missing map” 四步填补图；`y` 2340 起 455 |
| 29-uvmap-trust.png | 技术页 | 有头 Chrome（initJS 滚到验证图并等 6 秒） | 底边落在段落末行 “map.” 之下，仅露出下一个折叠框的顶边线；§4 “How far to trust it” 全节：12–14% 句、验证图（两幅折线图，45%→12%）、图注、peer-reviewed 句；前两次破图版本已被覆盖 |
| 30-uvmap-provenance-tour.png | 技术页 | 有头 Chrome（JUMP） | “Orion”“Is this pixel a photograph?” 两张近景卡片，含 provenance 静态示意；`y` 2290 起 330 |
| 32-uvmap-summary.png | 技术页 | 有头 Chrome（展开所有 details） | §5 Technical summary（含 “Date of this map: September 16, 2026.”） |
| 33-uvmap-lede.png | 技术页 | 有头 Chrome | 标题、署名行、首图与引导句 “the first detailed version with no holes in it”；`y` 0 起 800 |
| 34-uvmap-caveats.png | 技术页 | 有头 Chrome（展开所有 details） | 折叠节 “Known artefacts and caveats — read before measuring” 全部要点 |

技术页截图是在页面“展开折叠节”或滚动后的状态下截的（均是读者点击/滚动即可到达的状态，没有改动页面文字）。

## 试验文件（已删除，非证据）

- `c-menard-uvmap.html`、`c-jhu-news.html` 的 curl 版（挑战页）及两份 `.chrome.html`（headless 拦截页）；`c-uvmap-manuscript.pdf`（403 的 HTML）；`c-36kr.txt`、`c-newsbytes.txt`（挑战页文本，后由 firecrawl 版替换）；`c-imgcheck.mjs`、`c-fetch-pdf.mjs`（图片 403 诊断与 PDF 直连尝试脚本）；`c-uvmap-assets/`（本地接收服务器的空目录）；`c-arxiv-murthy-search.xml`（“Rate exceeded”）。
- 本地接收服务器（recv.py，127.0.0.1:8765，用于尝试从应用内浏览器回传文件，失败）已停止；其输出目录 `c-uvmap-assets/` 已删除且未再生成。
- 同一次运行里重复生成的 `c-anthropic-map-shots*.json/.txt` 归档（NOARCHIVE 前）已删除，保留 `c-anthropic-map-pw.*`。
- 图片覆盖：25、27、28、29、30、32、33、34 因懒加载破图或位置偏移重截过，旧版已被覆盖。

## 隐私与泄露自查

- 截图来自匿名新 profile，页面无登录态；归档不含抓取者头像、显示名或 handle。
- 对 `docs/2610/1009/` 下 C 组文件 grep 本机用户名/路径/邮箱，未发现泄露（`c-browser-records.jsonl` 与脚本里只有 `os.homedir()` 调用，无具体用户名）。

## 假设

- Anthropic 页与 menard 页对我取得的“当时版本”负责；页面此后可能再改（Anthropic modified 09:29Z 之后是否还有改动未监测）。
- “北京时间 = 本机 + 12 小时”；网页时间戳按其自带时区换算。
- 技术页的交互控件（sky/Earth、ultraviolet/visible/infrared）是页面已有的，没有点击或改变页面状态以外的内容。
