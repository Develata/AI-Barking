# 1014 期 A 组抓取日志（8 州分别起诉 OpenAI）

时间为北京时间（UTC+8）；本机美东（EDT）= 北京 − 12 小时。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。开工约 17:10 北京（05:10 EDT），收工约 20:20 北京（08:20 EDT）；中途若干次 firecrawl 搜索与抓取排队较久，总用时约 3 小时，超过派工单的 90 分钟线（到约 17:35 北京时已完成 80% 的取证，其余为 Kansas 诉状、Reddit 与截图补做）。17:23 之前的时刻为近似值（±数分钟），截图时刻取自 `a-shots-log.jsonl`，热度时刻取自 `a-heat.json`。

工具：curl（浏览器 UA，匿名，不带 Cookie）；`barking html2txt / shot / heat`；pdftotext / pdftoppm / pdfinfo（poppler）；Tesseract（OCR，仅用于 Arkansas 扫描件诉状）；firecrawl（search 约 13 次、scrape 3 次：Kansas AG 新闻列表页、Reuters 原文、Reddit 帖；额度有限，已能 curl 的都没用）；未使用 WebSearch；未登录任何站点、未运行读取浏览器 Cookie 的工具。**opencli 不可用**：`opencli browser … open` 报 “Browser Bridge extension not connected”，`opencli doctor` 显示 Chrome 扩展未连接（Chrome 未运行），按模板改用 `barking shot pdf`（诉状与新闻稿 PDF 不需要浏览器）；本组未使用 headless Chrome（没有需要截图的网页）。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 17:10 | Reuters https://www.reuters.com/world/eight-us-states-sue-openai-claiming-chatgpt-harms-children-2026-10-14/ | curl | **401**（CloudFront），仅 774 字节 | `a-reuters.html`、`a-reuters.headers`（失败证据，保留） |
| 17:11 | firecrawl_search：“eight states sue OpenAI ChatGPT children attorneys general” 等 | firecrawl | 成功，得各州 AG 新闻稿与诉状 PDF 的 URL | 无（线索） |
| 17:12 | Montana AG 新闻稿 https://dojmt.gov/attorney-general-knudsen-files-lawsuit-against-openai-for-endangering-children-and-teens/ | curl + html2txt | 200 | `a-mt-press.html/.headers/.txt` |
| 17:12 | Detroit News 转载页 | curl | **402**（付费墙），页面已删除（自己生成的失败试验，不入库） | 无 |
| 17:12 | DocumentCloud Arizona 页与 API `api.www.documentcloud.org/api/documents/28813407/` | curl | 200（87 页） | `a-azdoc.html/.headers`、`a-dc-az-api.json` |
| 17:13 | Idaho 新闻稿与诉状 PDF；Nebraska 新闻稿 PDF 与法院文件 PDF；Arizona 新闻稿；WV 新闻稿；Iowa 新闻稿与请愿书 PDF；Arkansas 新闻稿（HTML、PDF）与诉状 PDF（28.9 MB，73 页扫描件） | curl | 全部 200；URL 取自搜索结果与新闻稿内链接，**没有枚举文件名** | `a-id-press.*`、`a-id-complaint.pdf`、`a-ne-press.pdf`、`a-ne-complaint.pdf`、`a-az-press.*`、`a-wv-press.*`、`a-ia-press.*`、`a-ia-petition.pdf`、`a-ar-press.*`、`a-ar-press.pdf`、`a-ar-complaint.pdf` |
| 17:14 | Arizona 诉状 PDF/TXT `s3.documentcloud.org/documents/28813407/arizona-v-openai.pdf/.txt`；Montana 诉状 PDF（govdelivery 附件） | curl | 200 | `a-az-complaint.pdf`、`a-az-complaint.txt`（DocumentCloud 自带文本）、`a-mt-complaint.pdf` |
| 17:15 | 各 PDF 转文本 | pdftotext -enc UTF-8 -layout | 成功（AR 诉状无文本层，仅 246 字节） | `a-*-pdftotext.txt` |
| 17:16 | AR 诉状 OCR（pdftoppm -r 170 灰度 → tesseract eng，73 页） | pdftoppm + tesseract | 成功；OCR 有零星错字（如 “kfe.law”“OpenAl”），关键句 ¶250 m、n 已对照截图目视核对一致 | `a-ar-complaint-ocr.txt`（中间页图在 scratchpad，不入库） |
| 17:17 | WV 新闻稿内“here”链接（Mailchimp 重定向）→ WV AG 站内 PDF；AZ 新闻稿的 Mailchimp 重定向 → mcusercontent PDF | curl -I（只取 Location） + curl | 200 | `a-wv-complaint.pdf`、`a-az-complaint-mc.pdf`（与 DocumentCloud 版字节不同，未逐字比对） |
| 17:18 | Iowa Capital Dispatch、NBC News（Angela Yang 与 Joe Kottke） | curl + html2txt | 200 | `a-icd.*`、`a-nbc.*` |
| 17:19 | The Verge 文章 https://www.theverge.com/ai-artificial-intelligence/1011500/openai-state-lawsuits-chatgpt-safety | curl + html2txt | 200（第一次误抓成板块首页，已删除重抓） | `a-verge.html/.headers/.txt` |
| 17:20 | KSN、FOX4KC、The Hill、News From The States（Idaho） | curl | **403** 或空壳；4 个失败页已删除（自己生成的试验）；KSN 与 FOX4KC 内容只用 firecrawl 搜索摘要作线索 | 无 |
| 17:21 | firecrawl_search：佛州诉讼、Utah、Platformer、OpenAI 回应、Kansas | firecrawl | 成功；Utah 无独立诉状结果；OpenAI 官网无相关页 | 无（线索） |
| 17:22 | 佛州 AG 新闻稿、诉状 PDF、临时禁令动议 PDF；Platformer；AZ Capitol Times | curl + pdftotext + html2txt | 200 | `a-fl-press.*`、`a-fl-complaint.pdf`、`a-fl-pi-motion.pdf`、`a-platformer.*`、`a-azcapitoltimes.*` |
| 17:23 | Reuters 2026-06-01 与 2026-09-28 的佛州报道 | curl | **401**，失败页已删除 | 无 |
| 17:23 | `barking heat docs/2610/1014 a --hn … --reddit …`（since 2026-10-14 17:00 UTC） | barking heat | HN 成功（3 组查询，其中 “attorneys general OpenAI lawsuit” 0 命中）；Reddit 搜索 **2 次全部被重定向登录页**，记入 errors | `a-heat.json` |
| 17:23–20:08 间（钟点未记录） | Kansas AG 新闻列表 https://www.ag.ks.gov/media-center/news-releases | curl | **403**；改用 firecrawl_scrape 成功，但缓存最新条目为 10-09，**未含 OpenAI 一条**；Kansas 诉状未取得 | 无（结果记于 evidence-a.md） |
| 17:23–20:08 间（钟点未记录） | Reuters 原文（firecrawl_scrape，maxAge 0） | firecrawl | 成功，取回正文与元数据（published_time 2026-10-14T18:46:44Z） | `a-reuters-firecrawl.md`（节录正文与元数据，非整页） |
| 17:23–20:08 间（钟点未记录） | Reddit 三个主帖（r/technology 1x62aep、r/ChatGPT 1x66orm、r/law 1x69gck） | curl `.json`；firecrawl_scrape | curl **403**（3 个失败文件已删）；firecrawl “不支持该站”。**分数与评论未取得** | 无 |
| 17:23–20:08 间（钟点未记录） | 中文转载：联合早报、搜狐（财联社）、新湖南、凤凰网；KSN；WIBW、KAKE | curl + html2txt | 中文四篇与 WIBW、KAKE 200；KSN **403**（失败页已删） | `a-x-zaobao.*`、`a-x-sohu.*`、`a-x-voc.*`、`a-x-ifeng.*`、`a-wibw.*`、`a-kake.*` |
| 20:08–20:10 | `barking shot pdf` 渲染 AR 诉状第 1、71、72 页，AZ 诉状第 1、67、68、84、85 页，NE 新闻稿第 1 页（144 dpi）；`cropbox` / `stack` 出 5 张图 | barking shot | 成功；已逐张打开目视确认文字清楚、范围正确 | `images/01`–`05`；`a-shots-log.jsonl`；中间图在 `%TEMP%\barking-shot\26101014-a\`（不入库）；`az-p01` 渲染后未使用 |
| 20:12–20:15 | 未成年人姓名替换（sed）：Raine、Lacey、Walker、Nelson 及全名 → `[MINOR-A]…[MINOR-D]` | sed | 完成并复核：文本存档内 grep 为 0 | 见 evidence-a.md 第 9 节 |

## 2. 图片清单（`images/`，2 倍像素版式；均为 PDF 页的 144 dpi 渲染，宽 1224 px ≤ 1400 px；只裁切、拼接，不改像素）

| 文件 | 内容 | 来源 |
|---|---|---|
| `01-ar-complaint-caption.png` | Arkansas 诉状第 1 页：Faulkner County Circuit Court、案号 23CV-26-2984、提交戳 2026-Oct-14 10:36:32 | AR 诉状 p.1 |
| `02-ar-relief-recursive.png` | AR 诉状 VII. REQUEST FOR RELIEF：p.71（¶250 a–g）与 p.72（h–p，含 m、n）拼接 | AR 诉状 pp.71–72 |
| `03-az-prayer-testing.png` | Arizona 诉状 PRAYER FOR RELIEF：p.84 下半与 p.85 上半拼接（E：在非受保护、非经独立核验的测试环境之外部署或继续测试先进 AI 模型） | AZ 诉状 pp.84–85 |
| `04-ne-press-individually.png` | Nebraska AG 新闻稿第 1 页（“individually sued”“in their own state courts”）；页面底部截到该页末，句子 “illegally” 在页尾处断开 | NE 新闻稿 PDF p.1 |
| `05-az-rogue-agents.png` | Arizona 诉状 K. OpenAI releases rogue cyberhacking agents（¶183–190）：p.67 与 p.68 拼接 | AZ 诉状 pp.67–68 |

## 3. 删除的文件（均为自己生成的失败/试验文件）

`a-detroitnews.html/.headers`（402）、`a-thehill.*`、`a-nfts-idaho.*`（403）、`a-ksn.*`、`a-fox4kc.*`、`a-x-ksn.*`（403）、`a-reuters-0601.*`、`a-reuters-0928.*`（401）、`a-reddit-*.json`（3 个，403）、首次误抓的 The Verge 板块首页 `a-verge.html`（已重抓正确文章）、`a-dc-az-api.headers`。

## 4. 假设

- 各州诉状提交戳若不写时区（AR、WV），按所在州时区（AR 中部、WV 东部）推定，并在 evidence-a.md 标注为推定。
- “各州新闻稿未给钟点”的写“官方未给时刻”；元数据钟点（datePublished）只作参考。
- Reuters 的 firecrawl 版本是 2026-10-15 约 08:30 UTC 之后的修订版，措辞可能与首版不同（页面有 “Updates throughout”）。
- 将 AR 新闻稿 Utah 一项视为疑似笔误，未采用。
- 未成年人姓名替换采用“姓氏全局替换”，会同时遮住案名里的家属姓氏。
