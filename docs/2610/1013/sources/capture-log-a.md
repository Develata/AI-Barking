# 1013 期 A 组抓取日志（Character.ai 破产账本与迪士尼许可）

时间为北京时间（UTC+8）；本机美东（EDT）= 北京 − 12 小时；UTC = 北京 − 8 小时。时刻取自命令输出的 `date` 读数与 `a-browser-records.jsonl`、`a-shots-log.jsonl`，其余为近似值（±数分钟）。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。取数窗口约 16:56–20:00（= 04:56–08:00 EDT）。

工具：curl（浏览器 UA，匿名）；`barking html2txt / shot / heat`；Playwright Core + 本机 Chrome（匿名、全新临时 profile、不登录、不带 Cookie、en-US；`a-browser.mjs`、`a-fetch.mjs`、`a-reddit-scores.mjs` 改写自 1012 期同名脚本）；firecrawl（scrape 1 次、search 7 次，额度有限）；WebSearch 3 次；pdftotext / pdftoppm（poppler）。未使用 opencli / agent-reach；未登录任何站点，未运行读取浏览器 Cookie 的工具；**未绕过 Stretto 的 reCAPTCHA**，未枚举 Stretto 文件名。收到的“截图改用 opencli”要求（中途送达）时，除 Reddit 外网页截图已完成，PDF 页渲染不经浏览器，故未改用 opencli，已截好的图未重做。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 16:57 | https://blog.character.ai/an-update-on-character-ai/ | curl + html2txt | 200 | `a-blog-character-ai.html/.headers/.txt` |
| 16:57 | https://www.404media.co/character-ais-bankruptcy-documents-show-the-economics-of-ai-dont-work/ | curl | 200 | `a-404media.html/.headers/.txt` |
| 16:57 | Business Wire 新闻稿 | curl | **403 Access Denied（Akamai）** | `a-businesswire-curl403.html/.headers`（原名 `a-businesswire.*`，已改名以免误当成正文） |
| 16:57 | CourtListener API `v4/search/?q="Character+Technologies"&type=r` | curl | 200；案卷 id 74951888 | `a-courtlistener-search.json`（90 KB） |
| 16:58 | DocumentCloud 文档页与 `api.www.documentcloud.org/api/documents/28809489/` | curl | 200 | `a-dc-api.json`；文档页 HTML 与响应头只作试验，**已删除**（`a-documentcloud-28809489.html`、`a-dc.headers`） |
| 16:58 | `s3.documentcloud.org/documents/28809489/18.pdf`、`18.txt` | curl | 200；26 页 PDF，543,198 字节 | `a-doc18-adams-affidavit.pdf`、`a-doc18.txt` |
| 17:00 | DocumentCloud API：用户文档列表（`?user=12831`）、`/documents/search/?q="26-11603"` 等 | curl | 用户列表排序参数无效，返回无关旧文档（已删除 `a-dc-user-list.json`）；按 id 28809470–28809500 逐个探测时返回非 JSON（疑似限流，**已停止**，不再探测）；检索 `26-11603` 仅命中 Doc 18 | 无 |
| 17:01 | https://www.courtlistener.com/docket/74951888/character-technologies-inc/（两次） | curl + html2txt | 200；页面“Last Updated: Oct. 13, 2026, 8:03 p.m.”；有 PDF 的只有 Doc 1–5；Doc 15–18 仅有 PACER 购买链接；API `v4/dockets` 返回 401 | `a-courtlistener-docket.html/.txt`（第二次取数 `…-recheck.html` 相同，已删除） |
| 17:02 | Business Wire 新闻稿 | firecrawl_scrape（markdown + rawHtml，maxAge 0） | 成功；rawHtml 含 datePublished 2026-10-13T03:50:00+00:00（输出 470 KB，经文件落盘后用 python 拆出） | `a-businesswire.firecrawl.md/.html` |
| 17:03 | firecrawl_search（PPC Land / 迪士尼 / 竞标程序） | firecrawl | 成功，返回 PPC Land 多篇与各媒体摘要；PPC 的 `budgets-180m` 为许可范围来源 | 无（仅线索） |
| 17:03 | WebSearch ×2（竞标程序、PPC Land、Chapter 11 日程） | WebSearch | 索引过旧，未命中 2026-10 内容；其中提到 Disney 9/18 任命 Anand 为 CTO 的线索，后经官方博客证实 | 无 |
| 17:04–17:06 | Business Wire 新闻稿页 | `barking shot render`（失败：Chrome 退出但无输出）→ Playwright `a-browser.mjs` | 渲染成功；页面显示 “Oct 12, 2026 11:50 PM Eastern Daylight Time”；先试截一版（`a-bw-try`，只存到 scratchpad，未入库），再截 01，因下缘带到下一段首行而用 `OVERWRITE=1` 重截覆盖了自己上一版 | `a-businesswire-rendered.txt`、`images/01-businesswire-press-release.png` |
| 17:07 | PPC Land 8 篇（`a-ppc-*`） | curl + html2txt | 全部 200；响应头文件已删（未用） | `a-ppc-*.html/.txt` |
| 17:08 | https://www.404media.co/…（渲染）/ 官方博客（渲染） | Playwright | 成功；第一版 05、11 含整幅黑色题图/广告条与悬浮 “Subscribe to Blog” 按钮遮字，**删除重做**；`a-browser.mjs` 为此加了隐藏 Cookie 横幅、Ghost portal 按钮的 CSS 与 `time` 选择器；多次因 CSS `text-transform` 使 `innerText` 大小写与源文本不同而锚点找不到（共 3 次 FAILED，见 `a-browser-records.jsonl`），改用标题区与正文区分段截后用 `barking shot stack` 拼接 | `a-404media-rendered.txt`、`a-blog-rendered.txt`、`images/05-…`、`images/11-…`（均为两段拼接）；scratchpad 中间图不入库 |
| 17:10 | `barking shot pdf` 渲染 Doc 18 第 3、4、5、7、11、12、22、23、26 页（144 dpi） | barking | 成功；中间图在 `%TEMP%\barking-shot\26101013-a\` | `images/02/04/07/08/09/10/12`（经 `cropbox` / `stack`）；首次的 02、03、04、06 整页单图后改为拼接版，**删除了自己刚生成的旧版**（`02-doc18-p4-…`、`03-doc18-p5-…`、`04-doc18-p11-…`、`06-doc18-p12-…`） |
| 17:12 | HN：`barking heat … --item 50068036 --hn "Character.ai bankruptcy" --hn "Character.AI Disney" --hn "Character.ai Chapter 11" --since 2026-10-12 06:00 北京` | barking heat | 成功；3 组搜索 + 条目 50068036（323 分/278 评） | `a-heat.json`、`a-hn-50068036.json`；`a-hn-50068036.txt` 由 `a-hn-flatten.py`（改写自 1012）展平 |
| 17:13 | Reddit 搜索 API（`barking heat a-reddit --reddit …`） | barking heat | **3 个请求全部失败**：old.reddit.com/search.json 302 到登录页（响应为 HTML）。**保留** `a-reddit-heat.json`（含 errors） | `a-reddit-heat.json` |
| 17:13 | firecrawl_search（仅 reddit.com）→ 10 个帖子 URL；Playwright 逐帖读 `shreddit-post` 属性 | firecrawl + Playwright | 9 个成功；`r/CharacterAI/an_update_on_characterai` 第一次 evaluate 报错，约 19:59 重试成功但分数读数为 0（页面带 JS 验证参数，分数不可靠） | `a-reddit-scores.jsonl`、`a-reddit-scores.mjs` |
| 17:14 | r/LocalLLaMA 帖 1x57zpx 页面文本 | Playwright | 成功（用于核“chatlogs”评论） | `a-reddit-localllama-1x57zpx.txt` |
| 17:15 | 夸大实例检索：firecrawl_search ×3（英文、中文、$5.3M/$10M 拼算） | firecrawl | 成功；拼算类实例未找到 | 无；其后 curl 抓取实例页：`a-x-caixin`、`a-x-thr`、`a-x-huxiu`、`a-x-sohu`、`a-x-ifeng`、`a-x-yahootw`、`a-x-ars`、`a-x-verge`、`a-x-wscn`（全部 200；`a-x-wscn` 实为空壳，仅 16 字节文本） |
| 17:16 | WebSearch（Character.AI “20 million” 月活） | WebSearch | 无官方出处，只有二手（NYT 2024-10 经 arXiv 转引、Sacra） | 无 |
| 17:17 | 官方博客 https://blog.character.ai/the-next-chapter-of-entertainment-and-fandom/ | curl + html2txt | 200；“Sep 18, 2026”，meta 发布 2026-09-18T15:50:33Z | `a-blog-next-chapter.html/.txt` |
| 17:18 | CourtListener Doc 1 PDF `storage.courtlistener.com/recap/gov.uscourts.deb.203004/gov.uscourts.deb.203004.1.0.pdf` | curl + pdftotext | 200；12 页；勾选框文本层不可读，改用 `barking shot pdf` 渲染第 2–4 页目视（临时目录，未入库） | `a-cl-doc1-petition.pdf/.txt` |
| 17:20 | https://aihot.news/、`/hot` | Playwright `a-fetch.mjs` | 200 | `a-aihot-home.txt`、`a-aihot-hot.txt`（含 OpenAI 事件当事人姓名一处，交付前已按协调方要求替换为 “[当事人]”） |
| 约 19:40 | firecrawl_search `site:cases.stretto.com/public/x936/14939` | firecrawl | 返回搜索引擎已索引的 Stretto 公开 PDF 直链（Doc 6–11 及申请类文书）；Doc 15/17/18 未被索引。此前 Stretto 案件页 https://cases.stretto.com/chartech 取到的 HTML 含 reCAPTCHA，**不绕**（HTML 只放在 scratchpad，未入库） | 无 |
| 约 19:45 | Stretto 直链 6 个 PDF（`…1493910132680000000023/024/025/026/027/028.pdf`，即 Doc 6–11） | curl + pdftotext | 全部 200、`application/pdf`；这 6 个文件名均取自搜索结果，**没有枚举**；下载后改名为 `a-doc06…`–`a-doc11…` | `a-doc06-critical-vendors.pdf/.txt` 等 |
| 约 19:50 | Doc 8 第 10 页、Doc 10 第 4、5 页渲染 | barking shot pdf | 成功 | `images/03-…`、`images/06-…` |

## 2. 删除与改名（均为自己生成的试验/重复文件）

- 删除：`a-documentcloud-28809489.html`、`a-dc.headers`、`a-dc-user-list.json`、`a-courtlistener-docket-recheck.html`、PPC 各页响应头文件、四张被替换的旧版图片（见上）、`images/05`、`images/11` 的第一版与第二版（各重做两次）。
- 改名：`a-businesswire.html/.headers` → `a-businesswire-curl403.html/.headers`；Stretto PDF 按 Doc 编号改名。
- 用 `OVERWRITE=1` 覆盖了自己的 `01`（下缘重截）。
- 编辑（协调方要求，B 组事件当事人信息）：`a-aihot-hot.txt` 中当事人姓名 → “[当事人]”；`a-x-caixin.txt` 侧栏“相关报道”一条标题中的国籍 → “[国籍]”。已对 `a-*` 文本存档再 grep（姓名拉丁拼写、中文国籍词）：无残留。`a-x-caixin.html` 为原件，不入库，未改。其他存档未改写。
- 删除：`a-x-wscn.html/.txt`（华尔街见闻《AI角色扮演独角兽Character.ai宣布破产，迪士尼接盘》，curl 200 但是空壳，仅 16 字节文本，标题只见于搜索摘要，未存档；未在 evidence 中引用）。

## 3. 我做的假设

- 把 DocumentCloud 上由 404 Media 上传并公开的 Doc 18、搜索引擎已索引的 Stretto PDF、CourtListener RECAP 公开 PDF 视为“可直接访问”，不构成对 reCAPTCHA / 付费墙的绕过；没有尝试 PACER 付费下载，也没有枚举 Stretto 文件名去找 Doc 15/17。
- 404 Media 页面时间 “4:06 PM” 按页面元数据（20:06:53Z）判定为 EDT。
- 匿名渲染时用 CSS 隐藏 Business Wire 的 Cookie 横幅与博客的 Ghost portal 悬浮按钮，是为了不遮正文；没有点击接受 Cookie。
- 文件 `a-doc18.txt` 是 DocumentCloud 的文本层，核对数字时同时对照了 PDF 渲染页（02、04、07、08、09、10、12 张均为渲染页）。

## 4. 交付前自查

- 对 `a-*` 文本存档与 `evidence-a.md` 做了抓取者标识 grep（用户名、邮箱、本机路径、账号名）：无命中；存档中没有抓取者头像、昵称或 handle（Reddit/HN 评论者用户名为公开内容，仅在 `a-hn-*`、`a-reddit-localllama-*` 原文存档中出现）。
- 单个文件 > 1 MB：本组 `a-*` 中仅 PDF/HTML 原件（`a-x-thr.html` 600 KB、`a-x-yahootw.html` 486 KB 等均 < 1 MB；`a-doc10-wages.pdf` 877 KB、`a-doc09-cash-management.pdf` 789 KB、`a-cl-doc1-petition.pdf` 824 KB）；文本文件最大约 90 KB（`a-courtlistener-search.json`）。无超过 1 MB 的入库候选文本。
- PDF、HTML 原件与图片不入库（Claude 事后传网盘）。
