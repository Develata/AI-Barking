# 1013 期 A 组取证：Character.ai 破产文件里的账本，与迪士尼 2000 万美元买的是什么

执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。取数窗口：2026-10-14 约 16:56–20:00（北京时间；= 04:56–08:00 EDT = 08:56–12:00 UTC）。时间一律北京时间，括号保留原时区。抓取日志见 `capture-log-a.md`。

**读法与级别**
- “L1（原件）”指法院在案文书原件（见下“文书来源”）、官方博客、Business Wire 官方新闻稿；“L4”指权威媒体；“L5”指二手媒体/聚合站（含 PPC Land、AIHOT）；“L6”指社交评论。
- 法院文书里的数字是**债务人一方的陈述**（Colin M. Adams 在 declaration 中“under penalty of perjury… to the best of my knowledge, information, and belief”），不是审计数。
- **没有拿到 Doc 15（竞标程序动议及其附件：Stalking Horse Transaction Agreement、term sheet、proposed order）、Doc 16、Doc 17（Dunayer declaration）原件。** 凡许可范围、排除项、专利数、竞标日程、break-up fee、$180M 员工薪酬等，**只有 PPC Land 对这些文书的转述**，状态一律写“未找到一手来源”，不要当原文引。
- 截图均为原件页面渲染或匿名 headless Chrome（全新临时 profile，未登录，DPR 2），不含任何抓取者账号信息。`05`、`11` 是同一页面的**两段拼接**（标题区 + 正文区，中间省略段落，间隔 24 px），`02`、`04`、`06` 是同一文书**相邻两页的上下拼接**（页眉保留，页内文字未改）。

## 文书来源（法院 26-11603 (BLS)，D. Del.，法官 Brendan L. Shannon）

| 文书 | 内容 | 本组取得方式 | 存档 |
|---|---|---|---|
| Doc 1 | 自愿破产申请（10/12 提交，12 页） | CourtListener RECAP 公开 PDF | `a-cl-doc1-petition.pdf/.txt` |
| Doc 6–11 | 首日动议（Doc 6 关键供应商、7 全球自动中止、8 PII 涂黑、9 现金管理、10 工资、11 租约拒绝；均 Filed 10/13/26） | Stretto 公开直链 PDF（经搜索引擎索引的链接，非 reCAPTCHA 页面；未枚举文件名） | `a-doc06…`–`a-doc11…`（.pdf/.txt） |
| Doc 18 | Colin M. Adams 首日 declaration（26 页，Filed 10/13/26，签署 10/13） | DocumentCloud 公开文档 id 28809489（404 Media 文内链接），`s3.documentcloud.org/documents/28809489/18.pdf` | `a-doc18-adams-affidavit.pdf`、`a-doc18.txt`（DocumentCloud 文本层）、`a-dc-api.json` |
| Doc 15 / 16 / 17 | 竞标程序动议 / 缩短通知动议 / Dunayer（Houlihan）declaration | **未取得**：CourtListener RECAP 只有 Doc 1–5；Stretto 案件页被 reCAPTCHA 挡住（未绕）；搜索引擎只索引到 Doc 6–11 等；DocumentCloud 检索 `26-11603` 仅命中 Doc 18 | — |

文书编号对应（据 CourtListener 案卷页）：15 = “Approve - Motion”、16 = “Shorten - Motion”、17 = “Declaration in Support”、18 = “Affidavit/Declaration in Support of First Day Motion”。PPC Land 文中也称 Doc 15 = bidding procedures motion（58 页）、Doc 17 = Dunayer declaration（14 页）。

## 一、事实清单（按派工单 A1–A9）

### A1　官方博客与新闻稿

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 公司自愿申请 Chapter 11、以法院监督的出售处理公司 | L1 | https://blog.character.ai/an-update-on-character-ai/ | “Today, we’re sharing that Character.ai has initiated a voluntary Chapter 11 bankruptcy process to facilitate a court-supervised sale of the company.” | 11 | 页面显示 “Oct 12, 2026”（官方页只给日期）；页面 meta 发布 2026-10-13T03:52:33Z（= 北京 10/13 11:52），meta 修改 05:24:54Z（= 北京 13:24）。存档的是 10/14 取得的当前版本，**可能与首发版有出入** | 已找到 |
| 申请 Chapter 11 的案号与法院 | L1 | Doc 18 页眉；CourtListener 案卷页 | “Case 26-11603-BLS”；Doc 18 ¶4 “On October 12, 2026 (the “Petition Date”), Character commenced a voluntary case under chapter 11 … in the United States Bankruptcy Court for the District of Delaware” | 09（页眉） | 申请日 10/12（美国），Doc 1 “Filed 10/12/26”，Colin M. Adams 以 CEO 签署申请书 | 已找到 |
| 若无报价则平台将关闭 | L1 | 博客；新闻稿；Doc 18 ¶49 | 博客：“We hope that the sale process will yield an actionable proposal that will allow continued operations of the Character.ai platform. If such a bid is unavailable, however, the platform will be wound down.”；新闻稿：“To the extent the Sale Process does not result in a purchase that provides for a continuation of the Character.ai platform, the platform will be wound down.”；Doc 18 ¶49：“If such a bid is unavailable, Character’s platform will be wound down.” | 11、01（新闻稿该段不在截图内） | 关闭的条件是“没有提供平台继续运营的买方”。博客与 declaration 句子几乎相同 | 已找到 |
| 出售预计 11 月底或 12 月初完成 | L1 | 博客；新闻稿；Doc 18 ¶70 | 博客：“We currently expect that the court-supervised sale process will be complete by the end of November or perhaps early December 2026.”；新闻稿：“Under the timeline set out in the proposed bidding procedures, the Company anticipates that the completion of the Sale Process and the closing of any resulting transactions will occur in late November or early December 2026.”；Doc 18 ¶70：“the Stalking Horse Transaction Agreement contains milestones … contingent on the sale closing on or prior to December 17, 2026.” | 11、01、10 | “预计”，不是承诺；与迪士尼协议的**最晚成交日 12/17** 并存 | 已找到 |
| 订阅继续有效，账单“暂时”不变 | L1 | 博客 FAQ 4 | “Yes, your subscriptions remain active, and billing is unchanged for now. If the service changes in a way that affects paid features, we intend to notify subscribers in advance.” | 11 | “for now”；“we intend to notify”是意向 | 已找到 |
| 用户数据仍受隐私政策保护 | L1 | 博客 FAQ 5 | “Your information continues to be protected under our Privacy Policy, and our data privacy and security practices are unchanged. As a reminder, you can delete your account or export your data through the app or the Character.ai website.” | 11 | 博客**没有**说数据不会随资产出售；Doc 18 另说 Disney 的 stalking horse 标的见 A4 | 已找到 |
| 迪士尼作为 stalking horse：非独占永久许可 + 2000 万美元 + 过渡服务 | L1 | https://www.businesswire.com/news/home/20261012656385/en/Character.ai-Initiates-Voluntary-Chapter-11-Proceeding-to-Facilitate-Value-Maximizing-Sale-Process | “Before the Chapter 11 filing, the Company entered into an agreement with Disney Worldwide Services, Inc. (“Disney”) under which Disney will serve as stalking horse bidder for a non-exclusive perpetual license of certain Company intellectual property and certain Company claims, in exchange for $20 million and Disney’s provision of transition services using former Company employees it has hired.” … “The transaction is subject to higher or better offers under section 363 of the Bankruptcy Code.” | 01 | 新闻稿副标题：“Reaches Stalking Horse Agreement to Sell Certain Assets to Affiliate of the Walt Disney Company”。标的是“许可”；“sell certain assets”是新闻稿自己的副标题措辞 | 已找到 |
| 新闻稿发布时刻 | L1 | 同上（页面渲染） | 页面时间戳 “Oct 12, 2026 11:50 PM Eastern Daylight Time” | 01 | = 2026-10-13 03:50 UTC = **北京 10/13 11:50**；页面元数据 datePublished 2026-10-13T03:50:00+00:00 一致 | 已找到 |
| 新闻稿其余要点：继续运营平台；顾问 | L1 | 同上 | “The Company intends to continue operating its platform as it conducts the Sale Process.”；“The Company has engaged Wilson Sonsini Goodrich & Rosati, P.C., as legal adviser, Areté Partners, LLC, as restructuring advisor, and Houlihan Lokey Capital, Inc., as investment banking advisor. Disney has engaged Skadden, Arps, Slate, Meagher & Flom LLP, as legal advisor.” | — | 新闻稿说 “Houlihan Lokey Capital, Inc.”；Doc 17 据 PPC 称 “Houlihan Capital, Inc.”（见吠点） | 已找到 |

### A2　首日声明与竞标程序动议原件

| 说法 | 级 | 来源 | 结果 | 状态 |
|---|---|---|---|---|
| Adams declaration（Doc 18）原件 | L1 | DocumentCloud 公开文档（404 Media 文内链接） | 取得，26 页 PDF，sha256 `e2f98a9c…ab5ab`；DocumentCloud 元数据 file_hash `39bddea6…`，与下载文件的 sha1（`78c555ab…`）不一致，原因未明（可能为平台处理后的文件）；内容与 Doc 18 页眉（“Doc 18 Filed 10/13/26 Page n of 26”）一致 | 已找到 |
| 竞标程序动议（Doc 15）与附件 | L1 | CourtListener RECAP；Stretto；DocumentCloud | CourtListener 只有 Doc 1–5（页面“Last Updated: Oct. 13, 2026, 8:03 p.m.”，两次取数相同）；Stretto 案件页返回 reCAPTCHA，**未绕过**；以 `site:cases.stretto.com/public/x936/14939` 搜索一次，只索引到 Doc 6–11 及申请类文书，没有 Doc 15/17/18；**没有枚举文件名** | 未找到一手来源 |
| Dunayer declaration（Doc 17） | L1 | 同上 | 同上，未取得 | 未找到一手来源 |

### A3　财务数字（原件为 Doc 18；404 Media 为转述）

404 Media 原文（Jason Koebler，页面 “Oct 13, 2026 at 4:06 PM”，页面元数据 published 2026-10-13T20:06:53Z → 页面时间为 **EDT**，= **北京 10/14 04:06**）：“The company said that in 2024, the company spent $13.8 million per month on AI compute; in 2025, it spent $10 million per month on AI compute. The company’s subscription revenue peaked at $2.8 million per month, but at times was less than $1 million per month. Its revenue from all sources, including ads, has peaked at $5.3 million per month.”（截图 05）

| 数字 | 404 Media 写法 | Doc 18 原句（逐字） | 实际口径与期间 | 截图 | 状态 |
|---|---|---|---|---|---|
| $13.8M | “in 2024 … $13.8 million per month on AI compute” | ¶26：“in the second half of 2024, following the 2024 Transaction, Character’s operating expenses (excluding legal fees) averaged approximately $21.9 million per month, including monthly compute costs of approximately $13.8 million.” | **2024 年下半年**月均，不是全年；同期不含法律费用的 opex 月均 $21.9M | 04 | 部分支持（期间写宽了） |
| $10M | “in 2025 … $10 million per month” | ¶26：“In fiscal year 2025, operating expenses (excluding legal fees) averaged approximately $18.5 million per month, including monthly compute costs of approximately $10.0 million.” | **FY2025 全年**月均；同年 opex $18.5M/月；Doc 18 没有 2026 年 8 月单月算力数 | 04 | 已找到 |
| $2.8M（订阅） | “subscription revenue peaked at $2.8 million per month, but at times was less than $1 million per month” | ¶28：“Monthly subscription revenue was approximately $0.6 million in January 2024, but due to increased marketing and other efforts, by August of 2026, this revenue stream was $2.8 million.” | 订阅月收入从 2024-01 的 $0.6M 升到 **2026-08 的 $2.8M**；原文是“增长”，没有“peaked / at times”字样（这是 404 的转述；$2.8M 即最近一个月） | 04 | 部分支持 |
| $2.8M（另一处） | （未提） | ¶11：“nearly doubling from approximately $2.8 million for the month of December 2025 to approximately $5.3 million for the month of August 2026” | **2025 年 12 月全部收入**也是 $2.8M，与上行 2026-08 订阅 $2.8M 是两个不同的数 | 02 | 已找到 |
| $5.3M | “revenue from all sources, including ads, has peaked at $5.3 million per month” | ¶11（同上）；¶28：“Monthly advertising revenue grew from approximately $0.5 million in July 2025, its first full month, to approximately $2.5 million in August 2026.” | **2026 年 8 月单月**全部收入；原文没有写“peak”。算术：订阅 $2.8M + 广告 $2.5M = $5.3M（我的算术，与原文口径一致）。FY2024 总收入约 $7.4M，FY2025 约 $23.6M（¶28） | 02、04 | 部分支持（“峰值”是 404 的措辞；期间=8 月） |
| $56.1M | “spent $56.1 million on compute costs between January and August of 2026” | ¶9：“Character spent approximately $56.1 million on compute costs, consisting primarily of contracted graphics processing units (or “GPU”) capacity from third-party cloud providers during the period from January 1 – August 31, 2026. This represents approximately 45% of Character’s operating expenses during the first eight months of 2026” | 8 个月合计；折合月均约 $7.0M（我的算术）；占 opex 约 45% | 02 | 已找到 |
| $19.7M | “$19.7 million on employee salaries and benefits during that same eight-month time period” | ¶9：“personnel costs of approximately $19.7 million in the same period, representing approximately 16% of Character’s operating expenses over the same period.” | 原文是“personnel costs”，404 写成“salaries and benefits”（口径略窄） | 02 | 已找到（“薪酬福利”是转述措辞） |
| $8–9M | “as of August was losing ‘approximately $8-9 million per month’” | ¶11：“As of August 2026, however, Character was running at an operating loss of approximately $8-9 million per month.”；¶70：“Character burns between $8-9 million a month to run its operations” | 口径是 **operating loss**（8 月）；**另有一个口径**：¶27 “net cash burn per month had declined to approximately $8.2 million by August 2026, compared with monthly averages of approximately $21.5 million in the second half of 2024 and approximately $17.6 million in 2025.”（net cash burn，见下） | 02、04、10 | 已找到 |
| $8.2M（WorkBuddy 的“月烧 820 万”） | 404 Media 原文**没有**这个数 | ¶27（上行）：“net cash burn per month had declined to approximately $8.2 million by August 2026” | **Doc 18 有**：net cash burn，8 月；与 $8–9M 的 operating loss 是两个口径 | 04 | 已找到（出处是 Doc 18，不是 404 Media） |
| opex 下降 | （404：“significant actions to reduce operational costs and improve revenues”） | ¶27：“operating expenses (excluding legal fees) had declined to approximately $13.3 million by August 2026, a reduction of approximately 39% from the second half of 2024 monthly average and approximately 28% from the 2025 monthly average.” | 2026-08 opex（不含法律费）$13.3M；$13.3M − 收入 $5.3M ≈ $8.0M（我的算术），与 operating loss $8–9M 自洽 | 04 | 已找到 |

**“同月收支拼算”能否成立**：不能直接拼。$10M（FY2025 平均算力）、$13.8M（2024H2 平均算力）与 $5.3M（2026-08 单月收入）三个数**期间不同**；Doc 18 没有给出 2026 年 8 月单月的算力。任何“$5.3M − $10M = 每月净亏 $4.7M”之类的拼算都是跨期，我没有在已发表文章里找到这种说法（见 A8），但 404 Media 的并排写法容易诱导读者这么算。同口径可拼的只有 Doc 18 ¶27 自己给的：2026-08 opex（不含法律费）$13.3M、net cash burn $8.2M；¶11 operating loss $8–9M。
另：¶27 的 opex 与 net cash burn 之差（如 2025：$18.5M vs $17.6M）未在文中解释；opex 均“excluding legal fees”，法律费用另计。

其他在案数字（与上面同源，供对照）：¶18 “Character has no funded debt and approximately $57 million in unrestricted cash and cash equivalents and investments as of the Petition Date.”（Doc 9 脚注：现金管理系统合计 $59,414,000，含 $2,000,000 refundable deposit——57.4 + 2.0 = 59.4，自洽）；Doc 1 勾选：估计资产与负债均 $50,000,001–$100 million、债权人 100–199 个、“Funds will be available for distribution to unsecured creditors.”（截图未做；Doc 1 第 3、4 页）。

### A4　迪士尼交易范围

| 说法 | 级 | 来源 | 摘句（逐字） | 截图 | 状态 |
|---|---|---|---|---|---|
| 交易形式：非独占、永久、全球许可 + 释放/不起诉 | L1 | Doc 18 ¶64 | “The Stalking Horse Transaction Agreement provides for, inter alia, the grant of a perpetual, worldwide, non-exclusive license to certain of Character’s intellectual property and related technology and a release, waiver, and covenant not to sue agreement. In addition, the transaction documents expressly contemplate that the Stalking Horse Transaction Agreement will be subject to a competitive process and implemented free and clear of all claims (including successor liability), liabilities, interests and encumbrances, pursuant to sections 105 and 363 of the Bankruptcy Code.” | 08 | 已找到 |
| 付款：2000 万美元中 750 万已预付，其中 200 万为可退还押金 | L1 | Doc 18 ¶14、¶62 | ¶14：“Disney agreed to pay, pursuant to the EPA, $7,500,000 of the $20,000,000 cash consideration up-front, $2,000,000 of which is serving as a refundable good-faith deposit.”；¶62：“(iii) Disney agreed to pay the Debtor a $7,500,000 up-front payment prior to the Petition Date, of which $2,000,000 is serving as Disney’s good faith refundable deposit, which represents 37.5% of the overall cash consideration” | 07、08 | 已找到 |
| 协议日期与独家谈判期 | L1 | Doc 18 ¶61–62 | “Character signed the agreement on September 18, 2026.”；“the Debtor agreed to negotiate exclusively with Disney with respect to the Disney Target Assets from September 18, 2026 through the Petition Date” | 12、08 | 已找到 |
| 迪士尼雇走多数员工，含原 CEO | L1 | Doc 18 ¶62 与脚注 10；Doc 10 ¶10；官方博客 2026-09-18 | Doc 18：“Disney agreed to hire the majority of Character’s workforce, including the Debtor’s chief executive officer up front and prior to the Petition Date (subject to certain employees remaining available to assist with Character’s operations, sale process, and wind down through the TSA)”；Doc 10：“As of September 30, the Debtor employed a total of approximately 86 full-time employees. As part of the proposed transaction with Disney Worldwide Services, Inc. (“Disney”), however, prepetition, 66 of those employees were offered, and accepted, employment with Disney … Of those, 56 have formally resigned from their employment with the Debtor.”（另：“the Debtor employs 18 active, full-time employees … 8 of whom are expected to remain employed by the Debtor through key portions of this case”）；官方博客 9/18：“Character.ai’s CEO Karandeep Anand is joining Disney in a newly created role as Senior Executive Vice President and Chief Technology Officer.” | 12、06 | 已找到。博客页 “Sep 18, 2026”，meta 发布 2026-09-18T15:50:33Z（= 北京 9/18 23:50）。**“CEO”不是同一人（推断：Doc 18 ¶62、脚注 10 与 PPC Land 都明说文书未点名被雇走的 CEO，这里是按 9/18 官方博客对应到 Karandeep Anand）**；签 declaration 和申请书的 Colin M. Adams 是申请日在任 CEO，同时自述为 Areté Partners（债务人的拟任财务顾问）Senior Managing Director（Doc 18 ¶2） |
| 迪士尼购买的许可标的：25 个具名领域的代码/文档/工具 + 9 项专利（4 项已授权）；排除项：品牌、域名、App、用户数据与聊天记录、训练数据、成品模型 | L5 | PPC Land《Disney budgets $180m for Character.ai staff, $20m for the bankrupt company》（2026-10-13T18:24:54Z = 北京 10/14 02:24；自称依据 Doc 15 附件：Stalking Horse Transaction Agreement Schedule A、§3.2 Excluded IP、term sheet）https://ppc.land/disney-budgets-180m-for-character-ai-staff-20m-for-the-bankrupt-company/ | PPC 转述：“Schedule A of the Stalking Horse Transaction Agreement lists the Licensed Technology as “code repositories, documentations, frameworks, tools, and any other artifacts” in 25 named areas, plus a catch-all.”；“The Licensed Patents table lists nine entries … Four are issued US patents …”；“Section 3.2 of the agreement, headed Excluded IP … the license does not include: … trade names, logos, trademarks “or any other brand elements”; domain names, URLs, “social media accounts or handles and any website content”; “any consumer-facing applications of Licensor”; “Licensor’s end user data or databases, any personally identifiable information of Licensor end users, any input entered by Licensor end users when engaging with Licensor’s products and services, or any outputs generated for end users by Licensor’s products and services”; “Unprocessed Data” …; “training data or any Finished Models, including each user-selectable conversational model or chat style.”” | 无（原件未取得） | 未找到一手来源（仅 PPC Land 转述）；PPC 的说法与 Doc 18 ¶64 不冲突 |
| “certain Company claims”指什么 | L5 | 同上 | PPC：“At closing, Disney would buy any claims Character.ai still holds against its former employees who joined Disney, the so-called Purchased Employee Claims. The motion’s own assessment of that asset is blunt: “The Debtor does not believe there is any value to such claims.””；Doc 18 ¶62 另有 “the Debtor agreed to release Disney and the transferring employees from certain matters” | 无 | 未找到一手来源（仅 PPC Land 转述） |
| 迪士尼为员工支付的薪酬 | L5 | 同上 | PPC 转述 term sheet：“Disney estimates the annual run rate total compensation of the Offered Employees to be approximately US$60 million” + “aggregate retention pool … US$30 million”，“estimated total compensation and retention over years 1-3 is estimated to be approximately US$ 180 million”；“Employee compensation, retention payments and equity incentives will be payable directly to employees and will not constitute consideration payable to the Company.” | 无 | 未找到一手来源（仅 PPC Land 转述） |
| 过渡服务（TSA）条件 | L1 / L5 | Doc 18 ¶14；PPC（Dunayer、TSA 转述） | Doc 18 ¶14：“retaining access to key employees through transition services provided by Disney to assist with safely running Character’s platform for a limited period”；PPC 转述 Doc 17：“Disney agreed to provide certain critical services to the Debtor for up to five months at no cost to the Debtor other than actual expenses incurred” | 07 | 前半已找到；“五个月、无偿”部分未找到一手来源（仅 PPC Land 转述） |

PPC Land 相关文章（均已存档 `a-ppc-*.txt/.html`）：`a-ppc-faces-wind-down`（10/13 05:39Z，申请日报道）、`a-ppc-163m-users`（08:29Z）、`a-ppc-66-of-86`（17:10Z）、`a-ppc-loses-8m`（18:23Z，Doc 18 解读）、`a-ppc-28-buyers`（18:20Z，Doc 17 解读）、`a-ppc-budgets-180m`（18:24Z，Doc 15 解读，即派工单所称 PPC Land 原文）、`a-ppc-jury-trial`（19:47Z）、`a-ppc-180m-staff`（10/14 04:54Z，综述）。**WorkBuddy 的措辞核对**：“25 个具名领域代码、文档、工具”与 PPC 一致；“4 项已授权美国专利”——PPC 写的是 9 项专利、其中 4 项已授权；排除项与 PPC 的 §3.2 一致，但 WorkBuddy 没提“Unprocessed Data”。

### A5　用户规模口径

| 说法 | 级 | 来源 | 摘句（逐字） | 截图 | 状态 |
|---|---|---|---|---|---|
| 月活约 1200 万、人均每天约 120 分钟 | L1 | Doc 18 ¶7 | “Character serves approximately 12 million active monthly users located around the world and, while usage varies widely, on average, active users spend approximately 120 minutes a day on Character.AI.”；¶6：“allows tens of millions of users” | 09 | 已找到（债务人自述，无统计口径定义） |
| 现有与历史用户约 1.63 亿 | L1 | Doc 8 脚注 7 | “In addition, serving all of the Debtor’s current and past platform users (approximately 163 million), even through email, would be cost-prohibitive.” | 03 | 已找到。原文是“current and past platform **users**”，**不是**“accounts”；上下文是论证为何不对用户逐一发送破产通知，不是业绩口径 |
| 与此前口径对照：2024 年 2000 万月活 | L4/L5 | 未找到官方出处 | WebSearch 与 arXiv 论文（2505.13354）转述：2000 万月活出自 NYT 2024-10 报道；Sacra 称“early 2024”、人均 75 分钟（未标公司出处）；PPC Land 称宾州 2026-05-01 起诉书引 “more than 20 million monthly active users” | 无 | 未找到一手来源（公司官方 2000 万月活原句未找到）。Doc 18 的 1200 万月活（2026 年）与 2000 万（2024/2026-05 起诉书引用）口径与时点不同，不能直接说“跌了 40%” |

### A6　时间线

| 节点 | 级 | 来源 | 原句 | 状态 |
|---|---|---|---|---|
| 申请日 | L1 | Doc 18 ¶4 | “On October 12, 2026 (the “Petition Date”)” | 已找到 |
| 竞标程序动议听证 | L1 | Doc 18 ¶58 | “schedule a hearing on the Bidding Procedures Motion in approximately two weeks”；¶4 “motion to shorten notice, by one week” | 已找到（只有“约两周”，具体日期原文未给） |
| 成交最晚日 | L1 | Doc 18 ¶70 | “contingent on the sale closing on or prior to December 17, 2026” | 已找到 |
| 竞标截止 11/20 17:00 ET、拍卖 11/23、出售听证 11/24、竞价门槛 $21.7M（= $20M + 违约金 $700,000 + $1M）、迪士尼若 11/2 前无竞标程序令或 11/30 前无出售令可终止 | L5 | PPC `a-ppc-budgets-180m` / `a-ppc-jury-trial` | PPC：“Bids are due November 20 at 5:00 p.m. Eastern; an auction, if needed, is set for November 23, with increments of $500,000, and the sale hearing for November 24.”；“a break-up fee, set at $700,000 … opening floor at $21.7 million” | 未找到一手来源（仅 PPC Land 转述；Doc 15 未取得）。“拟议”日程，法院尚未批准 |
| 首日听证日期 | — | 未取得 Doc 19/20 | — | 未找到一手来源 |
| 其后：Chapter 11 计划 | L1 | Doc 18 ¶50 | “the Character estate represents a classic “melting ice cube” scenario … the Debtor can reasonably expect to end the first quarter of 2027 with a positive cash balance”；¶50 第二步：“propose a chapter 11 plan of liquidation expeditiously” | 已找到 |

### A7　热度（取数时刻 北京 2026-10-14 17:12–19:59）

| 来源 | 数字 | 说明 |
|---|---|---|
| HN 50068036 “Character.ai has initiated voluntary Chapter 11 bankruptcy”（指向官方博客） | 323 分 / 278 评（Algolia；嵌套 264 条）；提交 10/13 15:28 北京 | `a-heat.json`、`a-hn-50068036.json/.txt`；派工单称 321/277，取数时刻不同 |
| HN The Verge 帖 50069942 | 145 / 24 | |
| HN Reuters 帖 50067507 | 15 / 5 | 另一重复帖 50072193：3 / 1 |
| HN 404 Media 帖 50083017 | 4 / 0 | |
| Reddit r/movies “Disney Readies $20 Million Bid for Character.ai Assets After Chatbot Company Files for Bankruptcy” | 2676 分 / 225 评 | 匿名 headless Chrome 读 `shreddit-post` 属性（`a-reddit-scores.jsonl`） |
| r/antiai “Reuters: Character.AI files for bankruptcy.” | 2159 / 135 | |
| r/CharacterAI “And we got actually info.” | 1122 / 254 | 另一帖 “An Update on Character.ai”：分数读数 0（Reddit 初次返回 JS 验证页，分数不可靠）/ 344 评 |
| r/LocalLLaMA | 351 / 106 | |
| r/Anticonsumption（转 404 Media） | 535 / 29 | |
| r/artificial | 117 / 32 | |
| r/BetterOffline 两帖 | 175 / 63；231 / 64（后者标题显示 “[ Removed by moderator ]”） | |
| r/news “Character.AI files for bankruptcy” | 405 / 75（标题显示已被版主移除） | |
| r/technology | 未找到 | 搜索只返回上表各板 |
| AIHOT 首页 | 《Character.ai 破产文件显示 AI 商业模式难以为继》404 Media 条目，精选，AI 评分 81 | `a-aihot-home.txt` |
| AIHOT /hot | 《Character.AI申请破产 迪士尼2000万美元牵头竞购》第 07 位，+21，热度 159，↓13% | `a-aihot-hot.txt`（含 OpenAI 事件当事人姓名，已替换为 “[当事人]”） |

Reddit 搜索 API（old.reddit.com search.json）被 302 到登录页，`barking heat` 的 Reddit 部分全部失败（`a-reddit-heat.json` 的 errors 保留）；上表分数来自逐帖匿名渲染。分数为取数时刻的显示值，Reddit 对机器人返回的数字有模糊，仅作量级参考。

### A8　夸大/简化实例（把交易写成“收购”）

判别标准：新闻稿副标题自己写 “Sell Certain Assets”，故“出售部分资产/部分资产”类标题**不算**夸大；宾语写成“收购 Character.ai”、或把许可写成资产买断且抹去“许可”，才算。

| 站点与 URL | 原句（逐字） | 时间 | 判断 |
|---|---|---|---|
| 财新网 https://companies.caixin.com/2026-10-13/102493026.html | 标题：“美国AI陪聊应用Character.ai申请破产 迪士尼拟以2000万美元收购” | 页面 2026-10-13 20:37:44（北京） | 宾语缺失，读者会读成收购整家公司。匿名可见的正文只有两段（“……根据美国《破产法》第11章启动出售资产的程序”），**没有提迪士尼、2000 万或“许可”**，其余可能在付费墙后，未绕过；即标题的“收购”在可见部分没有被正文纠正（页面存档 `a-x-caixin.txt`） |
| 台湾 Yahoo 股市（MoneyDJ） https://tw.stock.yahoo.com/news/character-ai%E8%81%B2%E8%AB%8B%E7%A0%B4%E7%94%A2-%E8%BF%AA%E5%A3%AB%E5%B0%BC%E6%93%AC%E6%94%B6%E8%B3%BC%E9%83%A8%E5%88%86%E8%B3%87%E7%94%A2-233000212.html | 标题：“Character.AI聲請破產 迪士尼擬收購部分資產” | 页面 “2026年10月13日週二 下午7:30”（UTC+8） | 写“部分资产”，较轻，但“收购”隐去了“许可” |
| The Hollywood Reporter（Alex Weprin）https://www.hollywoodreporter.com/business/digital/disney-may-buy-character-ai-chatbot-chapter-11-bankruptcy-1236728629/ | 网页 `<title>`：“Disney May Buy Character.AI as Tech Startup Files for Bankruptcy”；页面 H1/og:title：“Disney Circles Character.ai as Chatbot Company Files For Chapter 11 Bankruptcy”；副题：“The entertainment giant, which recently hired the AI firm’s former CEO, is the stalking horse bidder in the proceeding for certain assets.” | datePublished 2026-10-13T11:52:13Z（= 北京 19:52） | `<title>`/URL slug 写 “May Buy Character.AI”（宾语是公司）；H1 与副题较准确。标题曾被改或两套并存，原因未明 |
| 虎嗅 https://www.huxiu.com/article/4897119.html | 摘要加粗句：“Character.AI从获谷歌27亿美元收购到破产拍卖仅值2000万美元”；另：“最终只能以2000万美元出售部分技术许可” | 2026-10-13T16:25:23+08:00 | “拍卖仅值 2000 万”：2000 万是非独占许可的 stalking horse 底价（可被更高出价替换），不是公司估值；2024 年谷歌的 27 亿美元，Doc 18 ¶19 的说法是“approximately $2.7 billion in total consideration”换取“rehired the founders … non-exclusive license … released certain claims”，也不是收购。同文给了“许可”二字，标题与摘要较歪 |
| AIHOT /hot（聚合站） | “迪士尼出价 2000 万美元收购部分未披露资产，并获得一项非独占永久知识产权许可”；“Character.AI母公司 … 迪士尼出价 2000 万美元收购部分未披露资产” | 取数 2026-10-14 17:20 | L5 聚合；“未披露资产”不准：新闻稿写明是 “certain Company intellectual property and certain Company claims” |
| Reuters（HN 标题所示；Reuters 页面 curl 401，未读原页） | “Character.ai files for bankruptcy, agrees to sell some assets to Disney for $20 million” | HN 提交 10/13 14:16（北京） | 用“sell some assets”，与新闻稿副标题一致，**不算夸大**，但抹去“license”；仅据 HN 标题，未读原站 |
| Reddit r/LocalLLaMA 评论（社交，L6）https://www.reddit.com/r/LocalLLaMA/comments/1x57zpx/characterai_files_for_bankruptcy_to_sell_some/ | “What I am surprised about is Disney acquiring their devs, chatlogs, and documentation.” | 帖 2026-10-13T20:35Z | 与 PPC 转述的 Excluded IP（用户输入与输出不在许可内）冲突；**聊天记录不在迪士尼许可里**（仅 PPC 转述） |
| “把 $5.3M 与 $10M 拼成每月净亏 470 万”之类 | 搜索未找到任何实例 | — | 找不到 |

### A9　有理有据的质疑线索（L5/L6，仅线索）

**文书原文里、线索摘要没提或说反的条件（可能的吠点）**
1. 申请书与 declaration 把“为何买家不出现”归因于责任：Doc 18 ¶11 “attempts to raise capital or monetize the business have been unsuccessful due to Character’s potential liabilities and the current regulatory environment.”；¶12 称曾通过“Prior Banker”接触约 28 方，只有一份被认为不可行的方案（原句：“Character and the Prior Banker received consistent feedback … the litigation and regulatory overhang on Character was too great. The Prior Banker’s outreach resulted in only one proposal that was ultimately deemed to be unworkable.”）。这与 404 Media 的“economics of AI don’t work”叙事不同：文书自己给的原因是**诉讼与监管负担**。
2. 公司没有融资债务、现金约 $57M（¶18），并非现金耗尽后破产；案件的目标是“sale process … then plan of liquidation”（¶49–50，“melting ice cube”）。
3. 迪士尼 $20M 现金里已有 $7.5M 预付，其中 $2M 可退还（¶62）；员工**不**由估价内的 $20M 支付（见 A4，仅 PPC 转述 $180M 为直接付给员工）。
4. CEO 一词指两人（见 A4；“被迪士尼雇走的 CEO = Anand”是按 9/18 官方博客的推断，Doc 18 不具名）。Adams 在申请日是 CEO，同时是 Areté Partners 的 Senior Managing Director；该公司被新闻稿列为 Character 的 restructuring advisor，Doc 18 ¶2 称 Areté 为 “Character’s proposed financial advisor”。原文自己披露，不是我的推断。
5. Doc 18 ¶62 的脚注 10（p.22）自陈 Disney 雇走的 CEO 是“insider”，并论证 “Disney did not and does not control the Debtor in any respect”。
6. 404 Media 把 $13.8M 写成“in 2024”（实为 2024 下半年），把 $2.8M 订阅写成 peaked/“at times <$1M”，把 $5.3M 写成 peaked，把降本写成“simplifying its AI model”——Doc 18 ¶27 的措辞是 “transitioning inference workloads to lower-cost GPU capacity and post-training open-weight models rather than developing proprietary foundation models”，“simplifying” 字样不在原文。404 Media 称 “a lot of employees quit”，¶10 的说法是 “mounting employee attrition … concerns that employees would start departing Character en masse”，实际流向是 66 人入迪士尼。
7. 404 Media 自己的标题与立场：“The Economics of AI Don’t Work”、“enshittification”、“Character.ai enshittified its product”——这是该媒体的立场（文内也写 “there is nothing in Character’s bankruptcy filings that could not be said about many other AI companies”，并承认 “The difference … those other companies have been able to repeatedly get injections of new cash”）。一家公司的文书推不出整个行业，且文书把法律成本与责任列为主要原因。

**社区质疑（HN 评论无公开得分；链接用 `https://news.ycombinator.com/item?id=<id>`）**
- 50071922（64 条后代评论）：“For all intents and purposes Character.ai was acquired in August 2024, and this is just the mess leftover from their run around antitrust regulation.” 对应 Doc 18 ¶19（2024 年 Google 以约 $2.7B 总对价重新雇用创始人并获非独占许可）。
- 50068213：“2 years in, the founders and 30 researchers took Google’s $2.7b and left … Yet they were still putting $1m/mo on the corporate cards with 86 full time bay area employees on ~$278k average base, and almost $100k owed to DoorDash.” 其中 DoorDash 欠款：Doc 1 top-20 名单第 11 项 “DoorDash … Trade Payable … $89,996.00”（与“almost $100k”大致相符）；其他数字（$1M/月公司卡、$278k 均薪）未核。
- 50074891：“It was my impression that some of these companies are getting acquired for the data they collected, and $20M seems on the low-end nowadays … So, is it that Disney isn’t getting the data, is the data poor, or is it just not a data thing at all?” 与 PPC 转述的 Excluded IP 吻合（用户数据不在许可内）。
- 50070201：“I wonder what will happen to all the very personal and sensitive data they (probably) have on users?” 博客 FAQ 5 与 declaration 都没有回答（Doc 7/8 只涉及通知方式）。
- Ars Technica 另有推测：“it also seems possible that the AI company may be tempted to sell internal data, like employee emails and chat sessions”（`a-x-ars.txt`，推测，非事实）。
- 以上均为线索（L6），进正文前须回到文书核实。

## 二、扫描说法勘误

| 扫描/派工单说法 | 核对结果 |
|---|---|
| “月烧 820 万（404 Media 原文没有这个数，原文是每月亏损 800–900 万）”（派工单对 WorkBuddy 的更正） | **更正的更正**：404 Media 确实没有 $8.2M，但 **Doc 18 ¶27 有**：“net cash burn per month had declined to approximately $8.2 million by August 2026”。$8.2M（net cash burn）与 $8–9M（¶11、¶70 的 operating loss）是两个口径，均有一手出处 |
| WorkBuddy：约 1200 万月活、人均每天 120 分钟 | 与 Doc 18 ¶7 一致 |
| WorkBuddy：约 1.63 亿“现/历史账号” | 原文是 “current and past platform users (approximately 163 million)”，出自 Doc 8 脚注 7，是为论证通知方式；“账号”（accounts）是 PPC 另一篇的措辞，原件用 users |
| WorkBuddy：许可 = “25 个具名领域代码、文档、工具 + 4 项已授权美国专利”，排除品牌、域名与 App、终端用户数据与聊天记录、训练数据、已训练模型 | 仅 PPC Land 转述 Doc 15 附件，未取得原件。PPC：25 个具名领域、**9 项专利，其中 4 项已授权**、3 项待审、2 项“Not yet published”；排除项与 WorkBuddy 一致 |
| 派工单：404 Media “Oct 13, 2026 at 4:06 PM，时区未标” | 页面元数据 published = 2026-10-13T20:06:53Z，故页面时间是 **EDT**，= 北京 10/14 04:06 |
| 派工单：affidavit 原件托管在 documentcloud | 属实：documentcloud.org/documents/28809489-18（上传 2026-10-13T20:06:10Z）。它是 **Doc 18**（Adams declaration），不是 bidding procedures motion（Doc 15） |
| 派工单：Stretto 上只找到 Doc 1 可直接下载 | 另有 **Doc 6–11** 的 Stretto 公开直链（经搜索引擎索引，已下载）；Doc 15/17/18 未索引。Doc 1 在 CourtListener 亦可得 |
| 派工单：CourtListener 目前只有 Doc 1、Doc 2 | 页面列出 Doc 1–5 有 PDF（Doc 3、4、5 为首日动议）；Doc 15–18 仅有 PACER 购买链接 |
| 派工单：“Business Wire … 发布时刻（页面时间戳）” | “Oct 12, 2026 11:50 PM Eastern Daylight Time” = 北京 10/13 11:50 |
| 404 Media 与 AIHOT：“订阅收入峰值每月 $2.8M”“全部收入峰值每月 $5.3M”（AIHOT 转 404 的措辞） | 原文是 2026 年 8 月的月度值与增长，没有写 peak；见 A3 表 |
| 404 Media：“in 2024 … $13.8 million per month” | 原文是 2024 年下半年月均 |
| 派工单与 404 Media：“Colin Adams CEO” | 属实（Doc 1 签署、Doc 18 签署）；注意原运营 CEO Karandeep Anand 已于 9/18 官宣加入迪士尼 |

## 三、截图清单（`docs/2610/1013/images/`，均已逐张打开目视确认）

| 文件 | 内容 | 来源 |
|---|---|---|
| `01-businesswire-press-release.png` | 新闻稿页：时间戳、标题、副标题与迪士尼 stalking horse 段、时间表段（Cookie 横幅用 CSS 隐藏，未点击接受） | 匿名 headless Chrome，CSS 1000，DPR 2 |
| `02-doc18-compute-share-revenue-loss.png` | Doc 18 p.4（¶9 起：$56.1M 算力）+ p.5（¶9 续：45%、$19.7M；¶10；¶11：$2.8M→$5.3M、$8–9M） | 上下两页拼接，页眉保留 |
| `03-doc8-p10-163m-platform-users.png` | Doc 8（PII 涂黑动议）p.10：脚注 7，“approximately 163 million” | PDF 页渲染 |
| `04-doc18-opex-compute-burn-revenue.png` | Doc 18 p.11（¶26 $21.9M/$13.8M、$18.5M/$10.0M；¶27 $13.3M、$8.2M）+ p.12（¶28 $0.6M→$2.8M、广告 $0.5M→$2.5M、$7.4M→$23.6M） | 上下两页拼接 |
| `05-404media-financials.png` | 404 Media 文章标题/署名/时间 + 财务三段（转述） | 同页两段拼接，省略中间 “An affidavit…” 段 |
| `06-doc10-86-to-66-disney-employees.png` | Doc 10（工资动议）p.4–5：¶9 18 名在职员工、¶10 86 → 66 入迪士尼 | 上下两页拼接 |
| `07-doc18-p7-disney-superior-proposal.png` | Doc 18 p.7：¶14 EPA/TSA、$7.5M 预付 | PDF 页渲染 |
| `08-doc18-p23-license-grant.png` | Doc 18 p.23：¶62 续、¶63、¶64（非独占永久许可、释放） | PDF 页渲染 |
| `09-doc18-p3-users-12m-120min.png` | Doc 18 p.3：¶7 约 1200 万月活、120 分钟 | PDF 页渲染 |
| `10-doc18-p26-dec17-milestone.png` | Doc 18 p.26：¶70 $8–9M、12/17 里程碑；签署 | PDF 页渲染 |
| `11-character-ai-blog-update.png` | 官方博客全文（日期/标题 + 正文与 FAQ） | 同页两段拼接，省略题图 |
| `12-doc18-p22-disney-hires-workforce.png` | Doc 18 p.22：¶59–62（迪士尼雇多数员工含 CEO） | PDF 页渲染 |

**没有截图的项**：许可范围、排除项原句（Doc 15 附件未取得，无原件可截）；Doc 1 第 3、4 页（债权人数、估计资产负债勾选）只渲染到临时目录查看，未入 `images/`。本组 12 个编号全部用完。

## 四、未能取得 / 失败

- Doc 15、16、17 原件（见上）；Stretto 案件页 reCAPTCHA（未绕）；Business Wire 直连 curl 403（改用 firecrawl 与匿名 Chrome 渲染）；`barking shot render` 对 Business Wire 失败；Reuters 页面未读；Reddit 搜索 API 需登录。
- 官方 2000 万月活原句未找到（只有二手转述）。
- 首日听证（10/14 约 13:30 ET，据 PPC 搜索摘要）结果当时尚未发生，未取。
