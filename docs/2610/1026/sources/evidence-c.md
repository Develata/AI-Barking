# 1026 期 C 组取证：微软约 6 万人可申请自愿离职（Voluntary Exit Offer，VEO）

执行方 claude-sonnet-5-5，取证时间 2026-10-27 北京 22:50–23:40。时间一律北京时间（UTC+8），括号保留原时区。存档文件在 `docs/2610/1026/sources/c-<name>.*`（`.html`/`.pdf` 原件只存本地，`.txt`/`.json` 可入库；单个超 1 MB 的文件见 `capture-log-c.md` 末节）。截图在 `docs/2610/1026/images/25–30、32–34、36、37`（31、35 为作废件，见 capture-log-c.md）。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别按 EDITORIAL 事实分级。

## 一、核心结论（仅取证，不下结论）

- “约 6 万”在微软官方博客里没有出现（逐字搜过 `c-msft-blog.txt`：无 “60,000”、无 “quarter”、无任何 AI 字样，仅导航栏有 AI 链接）。数字最早出自 BI（“a spokesperson told Business Insider”，口径为 **eligible** 人数）；Bloomberg 摘要写 “will offer buyouts to about 60,000 workers”，其中职级一句署 “said a Microsoft spokesperson”；CNBC 署 “a person familiar with the matter”；NYT 写 “the company said” 只有一小部分人预计接受。所以口径是“可申请的人数”，不是离职、裁员或预期离职人数；微软官方博客未公布该数字，但 BI、NYT 都写成微软发言人/公司所说（间接转述，非微软书面声明）。
- 微软对接受率的预期：Seattle Times 写发言人称“预计接受率低至 3%–5%（不足 3,000 人）”，NYT 写 “Between 3 and 5 percent of tech industry employees typically accept buyout offers, Microsoft said”。两处措辞略不同（前者是微软预期、后者是行业常见比例），都是媒体转述。
- 与 AI 的关系：官方博客全文未提 AI。AI 一说来自媒体：CNBC（“balance a costly artificial intelligence infrastructure build-out with capital efficiency”，标题 “as AI leads to efficiency push”）、Reuters（“help fund its massive AI investments”）、NYT（“as it spends heavily on artificial intelligence”）。这些是媒体对动机的概括，未见微软官方原话。7 月 6 日（不是本次）Coleman 在裁员信里写过 “the roles eliminated today are not being replaced by AI”。
- 资本开支两个数字都有原文：2026 年 4 月 29 日财报电话会 “roughly $190 billion”；2026 年 7 月 29 日 “shift from finance to operating leases adjusts our expectation to approximately $175 billion”（同一季度的会计口径调整，计划本身“unchanged”）。NYT 用 190，Reuters 用 175，两者不矛盾但口径不同。

## 二、事实清单（每条一行）

### C1 官方博客

发布时刻：页面可见日期 “Oct 26, 2026”（官方未给可见时刻）；页面元数据 `article:published_time` = 2026-10-26T15:33:56+00:00，即 北京 10-26 23:33:56（UTC 15:33:56）。HTTP `Last-Modified` 写 “Mon, 26 Oct 2026 08:33:56 GMT”，与元数据差 7 小时，推断是 PDT 时间被标成 GMT（未核实）。署名：Amy Coleman, Executive Vice President, Chief People Officer。页面写 “shared the below communication with Microsoft employees today”，文末 “This Viva Engage post has been edited slightly for external use.”

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 推出 VEO，对象是“参与部门中的合格员工” | L1 | https://blogs.microsoft.com/blog/2026/10/26/introducing-a-voluntary-exit-offer/ | “Today, we’re introducing a new Voluntary Exit Offer (VEO) to eligible employees within participating organizations.” | 25、27 | 存档 `c-msft-blog.*` | 已找到 |
| 地区与职级 | L1 | 同上 | “This offer will be available to eligible employees in participating teams in Canada, India, the United Kingdom and the United States at level 70 and below.” | 26 | 官方只写 “level 70 and below”，未把 level 对应到职称（VP 之说见 C2 的 Bloomberg/ST/NYT） | 已找到 |
| 限时、自愿、补偿措辞 | L1 | 同上 | “This is a time-limited, voluntary program that gives those who are eligible the opportunity to leave Microsoft with financial support and career planning resources.” | 26 | 官方没写具体补偿数额、截止日期、离职日；“financial support” 是全部补偿措辞 | 已找到（补偿细节与截止日官方未写） |
| 部分部门参与、部分不参与 | L1 | 同上 | “Some organizations are providing the Voluntary Exit Offer, and some are not. Participation varies based on each business’s priorities, customer commitments and workforce needs.” | 26 | 官方未列出参与/不参与的部门名称 | 已找到 |
| 与上半年自愿退休计划的关系 | L1 | 同上 | “This builds on what we learned earlier this year through the Voluntary Retirement Program.” | 25 | 同段还写 “It also gives participating organizations another way to adapt their workforce to changing needs.” | 已找到 |
| 邮件何时到 | L1 | 同上 | “Eligible employees will receive an email with a link to their Voluntary Exit Offer details within the next few hours.” | 26 | | 已找到 |
| 结尾口径 | L1 | 同上 | “The Voluntary Exit Offer is new for us. We’ll learn from it.” | 27 | | 已找到 |
| 博客是否提到 AI / 6 万 / 四分之一 | L1 | 同上 | （无）全文检索无 “60,000”“quarter”，除站点导航的 AI 栏目链接外正文无 AI 字样 | 27（全文） | 扫描称“官方博客里没搜到 60,000”成立 | 已找到（官方正文确无这些词） |

### C2 “约 6 万人”“约四分之一”的出处与口径

各家首发时刻（北京时间；括号为原时区）：

- Bloomberg：10-26 23:30（15:30:00 UTC，页面 `datePublished`）。存档 `c-bloomberg.txt` 只拿到标题与 “Takeaways by Bloomberg AI” 摘要（正文付费墙，未取）。
- Business Insider：10-26 23:30（2026-10-26T15:30:25.915Z，页面时间戳）。正文标 “available exclusively to Business Insider subscribers”，存档 `c-bi-memo.txt` 来自匿名 GET 返回的 HTML，见 `capture-log-c.md` 的付费墙说明。
- Seattle Times：10-26 23:30（8:30 am PDT；页面 “Oct. 26, 2026 at 8:30 am”，`datePublished` 2026-10-26T08:30:00-07:00）；更新 “Oct. 26, 2026 at 6:10 pm” PDT = 10-27 09:10。HN 链接标题 “Microsoft offers mass buyouts for roughly a quarter of its workforce”，页面现标题为 “Microsoft offers buyouts to 60K employees, but expects low uptake”（标题被改过）。
- CNBC：10-26 23:52（11:52 AM EDT）；更新 10-27 09:05（9:05 PM EDT）。现标题 “Microsoft offers voluntary buyouts as AI leads to efficiency push”（`<title>`），文章头部标题是 “Microsoft offers voluntary buyouts to 60,000 employees as AI leads to efficiency push”（截图 29），说明标题被改过。
- The Verge：10-26 23:58（11:58 AM EDT）。
- Reuters：10-27 00:10（页面显示 “October 26, 2026 4:10 PM UTC”）。
- NYT：10-27 03:05（2026-10-26T19:05:58Z）。

| 说法 | 级 | 来源 | 原文摘句（逐字） | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| “约 6 万”= 有资格申请的人数，微软发言人所说 | L4 | BI https://www.businessinsider.com/microsoft-is-offering-buyouts-read-the-memo-2026-10 | “Around 60,000 employees — or more than a quarter of the company's workforce — about will be eligible, a spokesperson told Business Insider, and the terms will be similar to Microsoft's standard severance.”（原文如此，“about will” 为原文笔误） | 无（BI 付费墙，未截） | 口径是 eligible，来源是发言人；“more than a quarter” | 已找到 |
| 同上（Reuters 转 BI） | L4 | Reuters https://www.reuters.com/technology/microsoft-is-offering-buyouts-around-60000-employees-business-insider-reports-2026-10-26/ | “Microsoft … said on Monday it is offering voluntary buyouts to about 60,000 employees, or more than a quarter of its global workforce”；“The development was first reported by Business Insider.” | 无（Reuters 付费墙，未截） | Reuters 把 6 万写成 “Microsoft said”，但微软官方博客并无此数；Reuters 另写 “Microsoft had 223,000 full-time employees globally as of June 30, 2026, according to its annual filing.” | 部分支持（Reuters 把数字归给微软，官方原文无此数，来源实为 BI 发言人转述） |
| Bloomberg | L4 | https://www.bloomberg.com/news/articles/2026-10-26/microsoft-to-offer-buyouts-to-60-000-workers-in-latest-cost-cut | “Microsoft Corp. will offer buyouts to about 60,000 workers, representing roughly a quarter of its total staff”；“Some personnel in the US, Canada, India and the UK at a vice president level and below will be eligible, said a Microsoft spokesperson.” | 无 | 仅摘要，正文未取；“vice president level and below” 署发言人 | 部分支持（摘要） |
| CNBC：6 万是可申请人数，匿名知情人，占 27% | L4 | https://www.cnbc.com/2026/10/26/microsoft-msft-voluntary-buyouts.html | “Around 60,000 people are eligible, according to a person familiar with the matter, who asked not to be named while discussing an internal matter. The effort affects about 27% of Microsoft’s total workforce of 223,000 people.” | 29、30 | 来源“知情人”，不是官方；27% = 6 万 / 22.3 万（原文）；“Microsoft does not have a target in mind this time around”（知情人，同文） | 已找到 |
| NYT：6 万人被提供；“公司称”预计只有一小部分接受 | L4 | https://www.nytimes.com/2026/10/26/technology/microsoft-buyouts-ai-spending.html | “Microsoft on Monday offered buyouts to a quarter of its work force, the latest effort by the company to rein in costs as it spends heavily on artificial intelligence.”；“Only a sliver of the 60,000 employees offered a buyout are expected to take it, the company said.” | 无 | “offered” 与 “eligible” 同指；NYT 标题 “Microsoft Offers Buyouts to 60,000 Employees as It Spends on A.I.” | 已找到 |
| Seattle Times：6 万人已收到；公司预期接受率 3%–5% | L4 | https://www.seattletimes.com/business/microsoft/microsoft-offers-mass-buyouts-for-roughly-a-quarter-of-its-workforce/ | “Microsoft only expects a fraction of the 60,000 employees who received buyout offers Monday to accept those packages and leave the company.”；“A spokesperson for the Redmond-based tech giant said the company is anticipating an acceptance rate as low as 3% to 5% — fewer than 3,000 employees — based on comparable offers observed across other firms.” | 无（Seattle Times 弹窗遮罩，见 capture-log） | 3%–5% 的基准是“其他公司的可比方案”；ST 写 “roughly a quarter”（约四分之一） | 已找到 |
| 总人数基数 223,000 | L4 | 同上 | “Microsoft’s global workforce totals about 223,000 employees, roughly 52,000 of whom are based at the company’s Redmond headquarters. The company’s head count peaked at 228,000 in 2024.” | | 228,000 是 FY2024 与 FY2025 年报共同数，见 C3 | 已找到 |
| 6 万是否经微软官方确认 | L1/L4 | 官方博客无；BI、Bloomberg、Seattle Times、NYT 称来自发言人/公司 | 见上各行 | | 官方书面发布无该数；仅有媒体报道的发言人口头/邮件表述 | 部分支持（官方文本未确认；媒体写发言人确认） |
| “约四分之一员工被提供/可申请”（非“被裁”） | L4 | 上列各家 | CNBC “27%”；ST “roughly a quarter”；BI “more than a quarter”；Reuters “more than a quarter of its global workforce” | | 6 万/22.3 万 = 26.9%（自算） | 已找到 |

### C3 微软员工总数基数

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| 截至 2026-06-30 约 22.3 万全职员工 | L1 | 微软 2026 年度报告页 https://www.microsoft.com/investor/reports/ar26/index.html（页面文字与 FY2026 10-K 同文，并引用 “fiscal year 2026 Form 10-K”；本页不是 SEC 的 10-K 文件，未取 EDGAR，SEC 对匿名抓取返回 403；页面未标备案日期） | “As of June 30, 2026, we employed approximately 223,000 people on a full-time basis, 121,000 in the U.S. and 102,000 internationally. Of the total employees, 89,000 were in operations, including product support and consulting services, datacenter operations, and manufacturing and distribution; 77,000 were in product research and development; 43,000 were in sales and marketing; and 14,000 were in general and administration.” | 33 | 全职员工；全球 | 已找到 |
| 上年基数 228,000（用于“人数峰值”） | L1 | FY2025 年度报告页 https://www.microsoft.com/investor/reports/ar25/index.html | “As of June 30, 2025, we employed approximately 228,000 people on a full-time basis, 125,000 in the U.S. and 103,000 internationally.” | | 存档 `c-msft-fy26-10k.txt`（文件名沿用清单名，内容实为 FY2025 报告） | 已找到 |
| 6 万 ÷ 22.3 万 ≈ 27% | 自算 | CNBC 原文也写 27% | | 30 | 分子是“可申请人数”（口径见 C2），分母是 6/30 全职人数；两者时点不同（报道日 10-26 vs 6/30） | 已找到（自算与 CNBC 一致） |

### C4 参与/不参与的部门

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| 官方：有的部门参与、有的不参与，未列名 | L1 | 官方博客 | “Some organizations are providing the Voluntary Exit Offer, and some are not.” | 26 | 官方无部门清单 | 已找到 |
| The Verge：全面参与 / 部分参与 / 不参与 | L4 | https://www.theverge.com/tech/1018666/microsoft-expands-buyouts-60000-employees-eligible | “Sources familiar with the situation tell The Verge that staffers across Microsoft are eligible for the voluntary exit offers, including those in its AI experiences, Copilot, CoreAI, Windows + Devices, Xbox, finance, and human resources teams. Microsoft’s cloud + AI, MicrosoftAI teams, and corporate, external, and legal affairs are partially participating, the sources say. Meanwhile, employees on Microsoft’s marketing, commercial business, discovery and quantum, and security teams aren’t participating.” | 32 | 匿名知情人；The Verge 还写 HRweb 内部页 “says today’s offer isn’t based on age, length of service, or performance”，并称微软 “may consider a future voluntary exit offer ‘where there is a business and workforce need.’” | 已找到 |
| Seattle Times：参与部门 | L4 | Seattle Times | “Multiple organizations within Microsoft are fully participating in the buyout program, including the gaming, Windows, Copilot, human resources and finance divisions, according to a screenshot of an internal memo viewed by The Seattle Times. Microsoft’s artificial intelligence, legal and cloud teams are partially participating. Organizations such as marketing, commercial business and security will not be participating.” | 无 | 依据“内部备忘录截图” | 已找到 |
| Reuters / BI / CNBC | L4 | Reuters | “Staffers across the Windows maker would be eligible for the offers, including those working in AI experiences, Copilot, finance, human resources, Xbox and Windows + Devices, a source familiar with the matter told Reuters … Units such as marketing, commercial business and security will be excluded” | 无 | 匿名；BI：“Only some teams are participating, excluding sales organizations, certain engineering teams, and some corporate functions.”；CNBC：“Some marketing, sales and engineering teams have decided not to participate in the new buyout, the person said.” | 已找到（多家说法大体一致，细节不同） |
| 差异 | | | Verge 与 ST 都写 AI 相关团队“部分参与”（Verge：MicrosoftAI；ST：artificial intelligence teams），同时 Verge 写 “AI experiences”“CoreAI” 全面参与。BI/CNBC/NYT 写销售、部分工程团队排除，Verge/ST 另点名 marketing、security、commercial business、（Verge）discovery and quantum | | 全部来自匿名/内部材料，微软官方没列 | |

### C5 与 AI 的关系：官方原话 vs 媒体推断

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| 官方 VEO 博客未提 AI | L1 | 官方博客 | （无）；官方给出的理由只有 “changing needs”“each business’s priorities, customer commitments and workforce needs” | 27 | 与 Ars Technica “While Microsoft did not give a direct reason for the buyout package” 一致 | 已找到 |
| 7 月 6 日裁员信（非本次）：被裁岗位“没有被 AI 取代”，同时 AI 在改变工作方式 | L1 | https://blogs.microsoft.com/blog/2026/07/06/the-latest-in-our-company-transformation/（Coleman，Jul 6, 2026；页面元数据 2026-07-06T13:30:28+00:00 = 北京 21:30） | “I also want to be direct that the roles eliminated today are not being replaced by AI. At the same time, what is true is that AI is changing how work gets done. Some of the tasks we do every day can now be automated” | 28、34 | 这是 7 月 4,800 人裁员的说明，不是 VEO；信里同时写 “Today we are eliminating around 4,800 roles, about 2.1% of our global workforce”“more than 30% of eligible employees chose to participate in our recent voluntary retirement program, and we will continue exploring similar approaches in the future” | 已找到 |
| Seattle Times 复述微软对 AI 的说法 | L4 | Seattle Times | “Coleman said in July that Microsoft’s workforce reductions were not driven by AI, at least not by what people would expect. Microsoft says it is not replacing workers by AI agents or workflows. But aside from cost-cutting measures, the company says its various workforce reduction efforts are geared toward shedding outdated roles and teams as AI changes the business.” | 无 | 记者概括（“at least not by what people would expect”），非逐字引语 | 已找到 |
| 媒体把 VEO 与 AI 开支挂钩 | L4 | CNBC | “Microsoft … said Monday that it's offering buyouts to a wide swath of employees as it strives to balance a costly artificial intelligence infrastructure build-out with capital efficiency.”（记者概括）；“The idea is to make the shift without weakening Microsoft's financial profile.” | 29、30 | 动机是 CNBC 的解读，未见微软官方原话 | 已找到（属媒体推断） |
| Reuters | L4 | Reuters | “… as the technology giant looks to contain costs and help fund its massive AI investments.”；“has forecast $175 billion in capital expenditures for the 2026 calendar year.” | 无 | 同上 | 已找到（媒体推断） |
| NYT | L4 | NYT | “the latest effort by the company to rein in costs as it spends heavily on artificial intelligence.”；“The company told investors to expect its capital spending, largely on data centers and the chips to fill them, to total about $190 billion in 2026.” | 无 | 190 与 Reuters 的 175 口径不同，见下行 | 已找到（媒体推断） |
| 资本开支 2026 日历年：约 1,900 亿美元（4 月） | L1 | 微软 FY26 Q3 财报电话会稿 https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q3 （Wednesday, April 29, 2026） | “For calendar year 2026, we expect to invest roughly $190 billion in capital expenditures which includes approximately $25 billion from the impact of higher component pricing.” | 无 | 含租赁口径变化前的预期 | 已找到 |
| 资本开支 2026 日历年：约 1,750 亿美元（7 月调整） | L1 | 微软 FY26 Q4 财报电话会稿 https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q4 （Wednesday, July 29, 2026） | “Outside of this useful life impact, our calendar year 2026 CapEx investment expectations remain unchanged. However, the shift from finance to operating leases adjusts our expectation to approximately $175 billion.” | 无 | 因 FY27 起数据中心与办公楼折旧年限由 15 年延至 25 年，部分租赁由融资租赁转为经营租赁，而融资租赁计入 capital expenditures，经营租赁不计入（同段原文） | 已找到 |
| FY2026 实际：购置物业与设备 1,159.48 亿美元 | L1 | 微软 2026 年度报告页（现金流量表） | “Additions to property and equipment(115,948)(64,551)(44,477)”（百万美元；FY2026、FY2025、FY2024） | 无 | 与“日历年 2026 计划”不是同一口径（财年、且不含融资租赁） | 已找到 |
| 7 月官方：资本开支季度数 | L4 | CNBC 7/29 报道 | “Capital expenditures for the September quarter should exceed $50 billion, Microsoft said in July.”（CNBC VEO 报道引） | 无 | 未回财报稿核对该 “more than $50 billion” 的一句（只核了日历年 190/175） | 部分支持 |

### C6 此前裁员与自愿退休（背景，只列 3 条）

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| 2026-07-06：裁约 4,800 人，约占 2.1% | L1 | 官方博客（见上） | “Today we are eliminating around 4,800 roles, about 2.1% of our global workforce” | 28 | 主要在 Commercial 与 XBOX | 已找到 |
| 2026 年春：自愿退休计划，超过 30% 合格员工参加 | L1（参与率）/ L4（资格人数） | 官方 7 月信：“more than 30% of eligible employees chose to participate in our recent voluntary retirement program”；资格人数无官方数：CNBC（2026-04-23）“About 7% of U.S. employees are eligible, according to a person familiar with the plans”；条件 “U.S. workers at the senior director level and below whose years of employment and age add up to 70 or higher” | | | 资格人数各家不同：BI “about 9,000”，Reuters “about 7% of its US workforce, or about 9,000”，Verge “around 9,000”，Seattle Times “more than 8,700”，Ars “8,750”，NYT “8,600”（NYT 与 CNBC 的 8,600/7% 与其他家并列，不取舍）；参与率 BI 引 “a person familiar” 称约三分之一，ST 写 “roughly one-third” | 已找到（人数口径不一） |
| 2025 年：约 1.5 万人裁员（Seattle Times） | L4 | Seattle Times | “The company laid off about 15,000 employees last year over the course of several months” | | 官方 FY25→FY26 人数：228,000 → 223,000（年报） | 部分支持 |

### C7 HN 热度（`c-heat.json`、`c-hn-50245524.json`、`c-hn-50250937.json`；采样时刻 2026-10-27 23:02 北京）

- 主帖 50245524 “Microsoft offers mass buyouts for roughly a quarter of its workforce”（Seattle Times）：997 分 / 1012 评；提交 2026-10-26T15:35:12Z（北京 10-26 23:35）。另有 `c-hn-item.html` 22:54 抓取时页面显示 870 分（同一帖，数小时内上涨）。
- KUOW 帖 50250937：129 分 / 142 评；2026-10-26T21:05Z。
- Reddit：`barking heat` 搜索被重定向到登录页，失败（未登录、不绕过）。
- HN 评论（Algolia 不含单评论分数，以下按子树评论数排序，仅作 L6 线索）：
  - ecshafer（子树约 212）：https://news.ycombinator.com/item?id=50245779 “60k workers, 1/4 of the workers. This is a big deal. It also really puts you into a bind if you receive the offer … Do you decline and hope you don't get hit with the layoff which hits if not enough people accept?” （纯推测：未见官方说明“人不够就裁”）
  - tandr：https://news.ycombinator.com/item?id=50246120 “‘1/4 of workforce was offered buyouts’ does not mean all 60000 will accept them.”（与官方、ST 的“预期 3%–5%”一致）
  - bri3d：https://news.ycombinator.com/item?id=50246149 “this is essentially reversing 2020-2022 net headcount growth … independent of any productivity impact of LLMs.” （未核；年报：FY2025 228,000 → FY2026 223,000，仅 -5,000）
  - orra：https://news.ycombinator.com/item?id=50247153 “Business Insider says it came from a Microsoft spokesperson. In the absence of an explicit number in a press release, I'd be inclined to believe it.”（点出官方没有数字、出处是 BI 发言人）
  - luxuryballs / locusofself / runjake / bialpio：关于 “level 70” 含义（id 50246739、50248321、50247127、50246860）：locusofself “Very few people are level 70. It means VP or Distinguished Engineer. Probably far less than <1% of the company.”；runjake “level 70 is ‘Distinguished Engineer’ and below, so effectively, it's available to all normies.”（两说法对 70 级的描述不同，均为网友，未核；官方只写 “level 70 and below”，Bloomberg 写发言人称 “vice president level and below”）
  - Nemo_bis：https://news.ycombinator.com/item?id=50250968 引 FT：“expecting a lower proportion to accept the voluntary exit offer because the employees included in the offer are younger and no additional incentive is given beyond standard severance … Typical take-up levels for buyouts in tech are between 3 per cent and 5 per cent”（FT 付费，未取；L6）
  - ndiddy：https://news.ycombinator.com/item?id=50249337 转述 Reddit 帖称 L65–L67 每年 4 周、L64 及以下 2 周、最高 39 周（网友转述 Reddit，未核；BI 写的是 “1 week of base pay per six months of service … 2 weeks … levels 65 to 67”，是 7 月裁员方案条款，BI 称此次“terms will be similar to Microsoft's standard severance”）

### C8 夸大/写歪的说法实例

| # | 站点 URL | 原句（逐字） | 发布时间（北京；原时区） | 发布方 | 写歪在哪 |
|---|---|---|---|---|---|
| 1 | https://www.ibtimes.sg/ai-layoffs-are-exploding-2026-over-93000-tech-jobs-gone-microsoft-targets-60000-more-95575 | 标题 “AI Layoffs Are Exploding in 2026: Over 93,000 Tech Jobs Gone, Microsoft Targets 60,000 More”；正文 “… while Microsoft is reportedly preparing buyouts for 60,000 workers” | 10-27 17:36（+08:00） | IBTimes Singapore | 标题把 6 万放在“AI 裁员”框内、用 “Targets”；正文又写 buyouts，且同文写 “the technology has not yet triggered widespread layoffs across the economy”（存档 `c-ibtimes-sg.txt`） |
| 2 | https://siliconangle.com/2026/10/26/microsoft-offers-voluntary-buyouts-to-60000-employees-amid-ai-investments/ | “The approximately 60,000 workers affected by the move are located in the U.S., Canada, India and the U.K.”；“That means the most senior employees affected by the job cuts are vice presidents.”；“Voluntary buyout programs give workers advanced notice of layoffs.” | 10-27 09:43（2026-10-27T01:43:57Z） | SiliconANGLE | 把“可申请人数”写成“受影响的 6 万”，并把买断称作 “job cuts”“advanced notice of layoffs”（截图 36） |
| 3 | https://watcher.guru/news/microsoft-layoffs-60000-employees-offered-voluntary-buyouts | 标题 “Microsoft Layoffs: 60,000 Employees Offered Voluntary Buyouts”；“Microsoft layoffs are taking a softer form right now” | 10-27 17:22（2026-10-27T09:22:05Z） | Watcher.Guru | 标题用 “Layoffs”；正文自己写 “Eligible staff have until early December to decide, and those who say yes will leave on Dec. 31” |
| 4 | https://www.sina.cn/weibo/detail/5358562487828501.html | 标题 “微软裁员四分之一，AI 时代国内科技差距正在急剧拉大”；正文 “微软10月26日表示，将向约6万名员工提供自愿离职补偿方案……符合条件的员工超过其员工总数的四分之一。” | 10-27 09:25（页面 `datePublished` 2026-10-27 09:25，无时区，按北京时间） | 新浪新闻（微博文章） | 标题写“裁员四分之一”，正文是“可申请”，标题与正文不一致 |
| 5 | https://www.investmentnews.com/equities/microsofts-ai-workforce-reset-targets-27-of-global-staff-in-new-buyout/268752 | 标题 “Microsoft's AI workforce reset targets 27% of global staff in new buyout”；“announcing a new Voluntary Exit Offer (VEO) that an unnamed source cited by CNBC said could reach approximately 60,000 employees” | 10-27 14:36（2026-10-27T06:36Z） | InvestmentNews | 标题 “AI workforce reset targets 27%”，把 AI 写成动因；正文承认是匿名来源、“could reach” |
| 6 | https://www.ctvnews.ca/world/article/microsoft-offers-voluntary-exit-packages-to-60000-staff/ | “Microsoft has announced it is offering voluntary layoff packages to roughly 60,000 employees.” | 10-27 21:04（2026-10-27T13:04:41Z） | CTV News（转载性稿件） | “voluntary layoff packages”，把“买断”写成“裁员包” |
| 7 | https://www.36kr.com/p/4043493481830280 | “微软为何要在这个节点，花大价钱‘请’这6万名员工离开？”；“一边是科技界最宏伟的基建狂飙，一边是6万名员工正在等待那封决定他们去留的邮件。” | 页面元数据 2026-10-27 23:01（可能为抓取/更新时间） | 36氪 | 把“6万可申请”写成“请这6万名员工离开”；官方与媒体均无“要请 6 万人走”之说；同文也写了“约占四分之一” |

另：新浪财经文章（https://finance.sina.cn/2026-10-27/detail-iniwqtpf9111057.d.html，10-27 01:56）标题含 “AI巨额投入挤压人力预算”，也是媒体动机概括（`c-sina-fin2.txt`）；新浪微博文章 “微软给四分之一员工发买断，连副总裁都能申请……堪称职场巨震”（`c-sina-xwn.txt`，10-27 05:08）。

## 三、可能的吠点（原文里有、线索摘要没提或说反的条件；L1/L4，来源已标）

1. 6 万是“有资格的人”，官方博客没有这个数字，来源是微软发言人对 BI 的说法（BI；Seattle Times；NYT）。
2. 微软预期接受率低（Seattle Times：发言人称 3%–5%，不足 3,000 人；NYT：“Only a sliver … are expected to take it, the company said”）。
3. 方案是标准遣散条款、没有加码：Seattle Times “The deals are not sweetened the way they were for the earlier retirement program”；NYT “provides the company’s standard severance package, which includes a minimum of eight weeks base pay”；BI “terms will be similar to Microsoft's standard severance”；官方只写 “financial support”。
4. 参与部门不是全公司：官方 “Some organizations … some are not”；销售、市场、安全等据报不参与（Verge、ST、Reuters）。
5. 微软称 7 月被裁岗位“not being replaced by AI”（官方 7/6），本次博客未提 AI；AI 关联来自媒体对资本开支的解读。
6. 微软对“更多买断/裁员”口径：BI “there are no plans for another buyout at this time, Microsoft's spokesperson said”；Verge 引内部 HRweb 称 “may consider a future voluntary exit offer ‘where there is a business and workforce need.’”；Ars 引 GeekWire 称 Coleman 7 月曾说在考虑把自愿离职作为常规管理手段（未取 GeekWire，403）。两者并列，不取舍。
7. 员工总数变化小：FY2025 228,000 → FY2026 223,000（年报）；NYT 与 ST 都写“人数仅略降”。
8. 时间表：Seattle Times/BI “until early December to decide”；CNBC/NYT “leave on Dec. 31”（均为媒体，官方博客未写）。

## 四、扫描说法勘误

- “微软官方博客里有 6 万/约四分之一”：官方博客没有该数字（已核）。
- “Reuters 称资本开支约 1750 亿美元”：Reuters 写 “has forecast $175 billion in capital expenditures for the 2026 calendar year”，与微软 7/29 电话会 “approximately $175 billion” 一致，但 NYT 写 190（4/29 口径），两者口径不同，非错。
- “Reuters 16:10 UTC”：页面显示 “October 26, 2026 4:10 PM UTC”，对。
- “参与部门含部分 AI、Copilot、Xbox 与职能团队”：Verge/Reuters 写 Copilot、Xbox、finance、HR 全面参与；AI 相关团队在 Verge/ST 里是“部分参与”（MicrosoftAI；artificial intelligence teams），同时 Verge 写 “AI experiences” 全面参与。ChatGPT 扫描的“含部分 AI”只对了一半，须按原句写。
- “6 万 = 约四分之一员工”：算术约 26.9%（6 万/22.3 万），CNBC 写 27%，BI/Reuters 写 “more than a quarter”，ST 与 NYT 写 “roughly/a quarter”。
- Seattle Times 把职级写成 “vice president level and below”，官方写 “level 70 and below”；Bloomberg 称此话出自微软发言人；BI 写 “level 70, which includes senior leaders at the company and below”；CNBC “experience level of 70 or below”。官方未给 70 级对应职称，正文建议照抄官方 “level 70 and below”。
- KUOW 稿写成 “Hood said in the memo”，而备忘录作者是 Amy Coleman（官方博客署名），KUOW 稿内部误写（`c-kuow.txt`）。
- Seattle Times 标题已由 HN 上的 “…roughly a quarter of its workforce” 改成 “Microsoft offers buyouts to 60K employees, but expects low uptake”；CNBC 标题也由 “…60,000 employees as AI leads to efficiency push” 改成 “Microsoft offers voluntary buyouts as AI leads to efficiency push”（`<title>`），引标题要注明抓取时间。
- 春季自愿退休计划的资格人数各家不同：8,600（NYT）、8,700+（ST）、8,750（Ars）、约 9,000（BI/Reuters/Verge）；官方只给“超过 30% 参加”。

## 五、未完成/未取到

- 补偿细节与截止日期的官方原文：官方博客没写；条件只见于媒体转述（BI、ST、NYT），未见官方书面材料；VEO 邮件/内部页（HRweb）只被 Verge 引用，未取。
- Seattle Times、Reuters、Bloomberg、BI 的页面截图：均遇到订阅弹窗/付费墙，按规则未截、未去遮罩；对应条目标“无”。
- Reuters 原文：匿名 GET 返回 401；文中 Reuters 摘句来自 opencli 浏览器页面的 DOM 文本（见 capture-log 的说明），页面可见部分被订阅弹窗遮住。该摘句请谨慎使用，核心事实另有 CNBC、NYT、The Verge 可替代。
- GeekWire（Coleman 7 月“研究把自愿离职作为常规手段”的出处）403，未取。
- Reddit 热度：被重定向到登录页，失败。
- 微软 FY26 Q4 季度 “capital expenditures should exceed $50 billion” 一句只见于 CNBC，未回财报稿核对；日历年 190/175 已核。
