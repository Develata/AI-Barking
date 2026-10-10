# 1009 B 组抓取日志（Google 拟购 Spirit Airlines 内部数据）

时间均为北京时间 UTC+8，取证日 2026-10-10（本机原时区 EDT，仍是 10-09 夜；北京 = 本机 + 12 小时）。分钟级为近似。**HTTP 200 不等于拿到正文**；以下“结果”均在存档后检查过内容。

## 工具与方式

- HTTP：`curl -sS -L`（匿名，普通 Chrome UA）；转文本用 `docs/2610/1008/sources/a-html2txt.py`（只读调用，未改）与 poppler（`pdftotext -enc UTF-8 -layout`、`pdftoppm`，Scoop）。
- firecrawl（MCP 工具 firecrawl_scrape / firecrawl_search）：Reuters 10/8 与 8/17 全文（curl 对 Reuters 返回 401 的 DataDome 验证页，已丢弃；firecrawl 返回完整正文，无付费墙提示）；BI、TNW、Sun Sentinel 的正文也先经 firecrawl 阅读，随后另用 curl 存 HTML 与 txt。
- 渲染截图：`b-browser.mjs`（由 1008 同名脚本改写：路径改 1009，Playwright Core + 本机 Chrome 无头，每次新临时 profile，不登录、不带 Cookie；CSS 宽 700、DPR 2）仅用于 Horsford 新闻稿一张（21）。PDF 截图用 `b-pdf-shots.py`（poppler 144 dpi = 2 倍像素比，按 `pdftotext -bbox-layout` 的行坐标裁切，只裁切/拼接，不改像素；17 号为 230 dpi 并横向裁到宽 1374 px 以放大字号）。台账：`b-shots-log.jsonl`、`b-browser-records.jsonl`。
- HN 热度与评论：Algolia（`search`、`search_by_date`、`items`）。
- 未使用 opencli、agent-reach，未使用任何会读取本机浏览器 Cookie 的工具。未登录任何站点，未接受任何 Cookie 弹窗，未发帖/点赞/评论/关注。
- 案卷来源：Epiq 公开案卷站（`document.epiq11.com`，匿名可取，文件直接返回 PDF，Content-Disposition 带卷号）；未使用 PACER；Stretto Research Suite（researchsuite.stretto.com）curl 只返回 SPA 外壳（6,818 B，已删），Stretto 的 chapter11cases.com 分析博客可读，存为二手转述（只作线索，标 L4）。
- Epiq 文件的定位方式（披露）：docId 由 Cybernews、TNW、firecrawl 搜索结果中的 Epiq 链接取得（Dkt 1463=4606206、1684=4619971、1594=4611822、1581=4611564）；Dkt 1489（AFA 异议）为在 docId 区间 4606206–4607100 做 HEAD/Range 探测（`curl -r 0-0`，只读文件名头）所得 4606932；Dkt 1677 链接来自 AFA 新闻稿。探测请求约数千次，每次只取 1 字节，范围 4606206–约 4607100 + 少量邻近 ID；我另在 4606660–4619970 区间启动过扫描，找到 1489 与确认用法后已停止（用 `taskkill` 结束 xargs），所取文件仅为上述 6 份。这是对公开案卷站的有限探测，不是绕过任何访问控制。

## 有效抓取（文件基名 | URL（已去查询参数，Epiq 链接保留 docId/projectCode）| 工具 | 结果 | 北京时间）

| 文件基名 | URL | 工具 | 结果 | 时间 |
|---|---|---|---|---|
| b-reuters.txt | https://www.reuters.com/world/lawmakers-raise-alarm-google-plan-acquire-spirit-airlines-data-ai-models-2026-10-08/ | curl→firecrawl_scrape(maxAge 0) | curl 401（774 B，DataDome，已删）；firecrawl 200 全文，我将正文逐字存为 txt（去除导航与推荐），并附元数据 | 11:30 |
| （未存档）Reuters 8/17 | https://www.reuters.com/legal/litigation/google-buy-spirit-airlines-business-data-10-million-2026-08-17/ | firecrawl_scrape | 200 全文，只读取引用，未另存文件（元数据 published 2026-08-17T21:23:18.889Z，modified 08-18T23:57:44Z；作者 Dietrich Knauth） | 12:40 |
| b-letter.pdf/.txt | https://horsford.house.gov/sites/evo-subsites/horsford-evo.house.gov/files/evo-media-document/spirit-airlines_letter_compressed.pdf | curl；pdftotext | 200，455,843 B，9 页，正文完整 | 11:35 |
| b-horsford-pr.html/.txt | https://horsford.house.gov/media/press-releases/spirit-airlines-to-sell-employee-data-to-train-google-ai-horsford-and-warren-lead-call-for-worker-privacy-protections | curl | 200，48,935 B，正文完整，日期 October 8, 2026 | 11:35 |
| b-afacwa.html/.txt | https://afacwa.org/congress-spirit-data-letter/ | curl | 200，150,064 B，正文完整 | 11:35 |
| b-pymnts.html/.txt；b-pymnts-aug.html/.txt | https://www.pymnts.com/big-data/2026/lawmakers-urge-google-and-spirit-airlines-to-pause-data-sale/ ；https://www.pymnts.com/news/artificial-intelligence/2026/dead-airlines-emails-just-became-a-10-million-ai-prize/ | curl | 200，正文完整 | 11:35 |
| b-yahoo.html/.txt | https://finance.yahoo.com/technology/articles/google-apos-10-million-purchase-090900390.html | curl | 200，21 KB 文本，正文只快速核标题与日期，未细读 | 11:35 |
| b-technology-org.html/.txt | https://www.technology.org/2026/10/09/lawmakers-google-spirit-airlines-data-ai-deal/ | curl | 200，未细读 | 11:35 |
| b-dkt1463-auction-results.pdf/.txt | https://document.epiq11.com/document/getdocumentbycode?docId=4606206&projectCode=SPJ&source=DM | curl；pdftotext | 200，1,493,379 B，35 页，“Doc 1463 Filed 08/14/26 22:00:10” | 12:00 |
| b-dkt-ombudsman-oct5.pdf/.txt | …docId=4619971… | curl；pdftotext | 200，673,627 B，9 页，“Doc 1684 Filed 10/05/26 12:19:31” | 12:10 |
| b-dkt1594-google-prelim-response.pdf/.txt | …docId=4611822… | curl；pdftotext | 200，433,757 B，4 页，“Doc 1594 Filed 09/09/26” | 12:50 |
| b-dkt1581-cpo-report-sep8.pdf/.txt | …docId=4611564… | curl；pdftotext | 200，541,540 B，23 页（9/8 监察员原报告），仅存档，未逐页核 | 12:50 |
| b-dkt1489-afa-objection.pdf/.txt | …docId=4606932… | curl；pdftotext | 200，277,260 B，9 页，“Doc 1489 Filed 08/18/26” | 12:50 |
| b-epiq-1677-epic-amicus.pdf/.txt | https://document.epiq11.com/document/getdocumentsbydocket/?docketId=1254907&projectCode=SPJ&docketNumber=1677&source=DM | curl；pdftotext | 200，849,087 B，25 页，EPIC 法庭之友动议与拟提交意见（Doc 1677，10/1） | 11:50 |
| b-stretto-blog.html/.txt | https://chapter11cases.com/blogs/news/the-spirit-airlines-deidentified-data-sale-auction-results-objections-and-the-september-30-hearing | curl | 200，175 KB，Stretto 的案情分析博客（审阅至 Dkt 1601，9/11），二手转述 | 11:40 |
| b-bloomberglaw-0817.html/.txt | https://news.bloomberglaw.com/bankruptcy-law/google-aims-to-boost-ai-with-purchase-of-spirit-airlines-data | curl | 200，139 KB，正文可读（未见付费墙提示），published 2026-08-17T16:22:49Z（文件名最初误写 0814，已改名 0817） | 12:25 |
| b-time-0825 / b-247wallst / b-cybernews / b-cnn-0818 / b-sunsentinel-0820 / -1007 / -1009 / b-tnw / b-bi-1005 | 见 evidence-b.md 各行 URL | curl | 200，正文可读；b-cnn-0818.html 5.4 MB（本地，不入库），txt 24 KB；b-bi-1005 为 BI 订阅墙页，curl 取到的文本含正文（firecrawl 另读到正文）；其余为元数据取时间与摘句 | 12:25–13:00 |
| b-winzheng / b-163-lawmakers / b-sina-lawmakers | https://www.winzheng.com/article/spirit-airlines-google-employee-data-sale ；https://www.163.com/dy/article/L8QIFI8E05148ALS.html ；https://k.sina.com.cn/article_5953190046_162d6789e06703twhs.html | curl | 200；中文页，用于夸大实例与中文媒体时刻 | 12:45 |
| b-hn-search-spirit-bydate.json / -rel.json / b-hn-spirit-oct.json / b-hn-search-oct.json | hn.algolia.com/api/v1/… | curl | 200 JSON；10 月无相关帖（后两份 0 命中，保留作记录） | 11:55–12:20 |
| b-hn-49343559 / -49339599 / -49338973 | hn.algolia.com/api/v1/items/… | curl | 200 JSON；首次对 49339599、49338973 出现 `schannel` 握手失败，重试成功 | 12:00 |
| 截图 | 见 evidence-b.md 与 `b-shots-log.jsonl` | b-pdf-shots.py / b-browser.mjs | 15–24 号共 10 张，逐张目视（见下） | 12:00–13:05 |

## 失败/未取得

| URL | 工具 | 结果 |
|---|---|---|
| https://www.axios.com/2026/08/17/google-spirit-airlines-bankruptcy | curl | 403（5,771 B，已删）；只用 HN 条目时间与 firecrawl 搜索摘录 |
| https://thehill.com/homenews/house/6137595-horsford-warren-lawmakers-google-spirit-data-ai/ | curl | 403（6,518 B，已删） |
| https://www.theinformation.com/briefings/google-outbids-mercor-spirit-airlines-corporate-data | curl；firecrawl | curl 403（5,855 B，已删）；firecrawl 只返回付费墙占位与 og:description（含“according to a filing Friday”）；发布时间未取得 |
| https://www.kansascity.com/news/business/article317546083.html | curl（与第一批同批执行） | 文件未落盘（批次超时转后台，未见该文件），Reuters 转载，不重取 |
| https://researchsuite.stretto.com/search/single/results?case=4189&doc=… | curl | 200 但为 SPA 外壳 6,818 B（需登录/脚本渲染），三份均已删除 |
| https://document.epiq11.com/document/getdocumentsbydocket/?projectCode=SPJ&docketNumber=…（无 docketId） | curl | 404，已删 |
| https://dm.epiq11.com/case/spirit/dockets 等 | curl | 200 但仅 3.4 KB 壳页，已不保留 |
| https://www.linkedin.com/posts/… | firecrawl_scrape | “不支持该站点”；LinkedIn 活动 ID 解码时间仅作旁证 |
| Reddit | firecrawl_search（includeDomains reddit.com） | 0 结果；仅在其他检索结果中见到 r/AIGuild、r/myclaw 的标题 |
| PACER | — | 未使用（需付费登录）；案卷改取 Epiq 公开站 |

## 过程中产生又删除的试验文件

- `b-dkt1581.bin`、`b-dkt1463.bin`、`b-dkt1489.bin`（Stretto SPA 壳，6,818 B）、`t-1489/1581/1463/1656.bin`（Epiq 404 页）、`b-hill.html`、`b-axios-0817.html`、`b-theinformation.html`、`b-itdoeswhatnow.html/.txt`（聚合站，仅用于定位 Epiq 链接，已删）、`hdr.txt`、`h.txt`。
- 图片 `16-letter-demands.png` 首版在两页拼接处留下大块空白与 “Page 2” 标签，已删除重做；`b-pdf-shots.py` 与 `b-shots-log.jsonl` 中有首版的记录，保留作台账。`21-horsford-pr-top.png` 首版顶部切断面包屑，已覆盖重做（`OVERWRITE=1`）。
- 在 scratchpad（系统临时目录）放过 `scan.sh`、`scan-out*.txt` 等扫描中间文件，不在仓库内。

## 截图（`docs/2610/1009/images/`，B 组区间 15–24）

| 文件 | 内容 | 来源与方式 | 尺寸 | 目视 |
|---|---|---|---|---|
| 15-letter-numbers.png | 议员信抬头、日期、收件人、首段与 “100 million emails, 500 million Microsoft Teams messages”段 | b-letter.pdf 第 1 页，144 dpi | 1224×779 | 已看，文字清楚；抬头为 Congress of the United States 图章 |
| 16-letter-demands.png | “That is why...” 与六条诉求 + 结语（第 1 页末+第 2 页，拼接，略去页间空白与页码标签） | b-letter.pdf 第 1–2 页 | 1224×1494 | 已看，拼接处 1 号条款与 2 号条款相连 |
| 17-dkt1463-asset-schedule.png | Dkt 1463 附件 A 清单：表头、Team Member 与 Documents 两块，含最右 “Google's Data Purchase Request” 列 | 第 18 页，230 dpi，横向裁至 172–1546 px（含 Category 至 Google 采购列，**不含最右 “Data Format” 列**） | 1374×402 | 已看，字小但可读；行与最右列 “Included” 对齐靠阅读判断，表上方另有黑条 “Customer Behavior”（原表首个分区，其右 “Not Included” 属该分区），整页见 19 |
| 18-ombudsman-conclusion.png | Dkt 1684 第 3 页：Tonic.ai、排除乘客数据库、结论段与脚注 3 | 第 3 页，144 dpi，页眉含 “Doc 1684 Filed 10/05/26” | 1224×1344 | 已看，清楚 |
| 19-dkt1463-p18-full.png | 同一清单的整页（上下文；字极小，仅作版面全貌） | 第 18 页，144 dpi | 1224×1584 | 已看，不可逐字阅读 |
| 20-dkt1463-auction-result.png | Dkt 1463 第 1 页通知 + 第 2 页 “Successful Bidder Google LLC $10,000,000 / Alternate Bidder Mercor.io $7,500,000” | 第 1–2 页拼接，144 dpi | 1224×1404 | 已看，清楚；第 2 页底部在表格行后截断 |
| 21-horsford-pr-top.png | Horsford 新闻稿标题、10/8、“led 119 Members of Congress” 与 “Court findings revealed” 段 | b-browser.mjs 无头 Chrome，DPR 2 | 1400×896 | 已看，顶部边缘有一条页面横幅图片的残边 |
| 22-dkt1594-excluded-datasets.png | Dkt 1594 第 3 页第 4 段：Google 排除 PNR、Refunds、Inflight、Timecard、WiFi 数据集 | 第 3 页页眉 + 段落，144 dpi | 1224×510 | 已看，清楚；段末句在 “scope of transaction” 处截断（下一行为 “altogether”） |
| 23-dkt1489-afa-numbers.png | Dkt 1489 第 1 页：AFA 对清单数字的描述 | 第 1 页，144 dpi | 1224×1280 | 已看，清楚 |
| 24-dkt1463-deid-clause.png | Dkt 1463 附件 A §3(c)：去标识化证明、CCPA 标准、referential integrity | 第 9 页页眉 + 条款，144 dpi | 1224×339 | 已看，清楚 |

所有图片均由原始 PDF/页面渲染后只裁切、拼接，未重绘、改数字或使用转载图。宽度均 ≤1400 px。

## 隐私与脱敏自查

- 未抓取任何社交平台（X、Reddit、LinkedIn）页面内容，只在检索摘录里见到帖子标题；无登录态页面；存档里没有抓取者头像、显示名与 handle。
- 本组不涉及费城案受害人信息。
- 自查：对 `b-*` 文件 grep 了本机用户名与维护者姓名，无命中（b-yahoo.html 因正则宽松有一处无关匹配，已确认非用户信息）。
- 单个文本文件均 < 1 MB（最大 `b-dkt1463-auction-results.txt` 157 KB）。`b-cnn-0818.html` 5.4 MB 与各 PDF 为本地原件，不入库，由 Claude 事后传网盘。
