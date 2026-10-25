# 1024 期 B 组取证：Apollo“科技业外零回报”与 Apricitas 创意行业 20 万岗位

取证时刻：2026-10-25 23:24–00:30 北京时间（本机美东 EDT = 北京 −12h）。执行方模型 claude-sonnet-5-5。只取证，不写正文、不替作者下因果结论。原件存 `docs/2610/1024/sources/b-*`（HTML/PDF/PNG 只存本地，文本入库）；截图在 `docs/2610/1024/images/13–24`。

## 0. 先看这几条（与“已知情况”不一致或线索摘要没提的）

1. Apricitas 原文发表于 **2026-09-27 22:01:59（北京时间）**（Substack API `post_date` 2026-09-27T14:01:59.871Z，`updated_at` 同日 17:16Z）。不是 10 月新文章；10/24 只是 HN 才把它顶上来（HN 提交 2026-10-24T23:11Z = 北京 10/25 07:11）。Gizmodo、Futurism（均 9/29）、Fortune（10/2）已经报道过，Fortune 采访 Richard Florida，他说“I’m not sure his numbers are right”。
2. Apollo 这篇是 Slok 同一论点的延续，不是首次：2026-08-09 的 Daily Spark《the buyers of ai are still waiting for the payoff》已写“The S&P 493 is spending heavily on AI. But it is not showing up in profit margins”；本篇的新措辞是“zero financial payoff”。
3. Apollo 的“pulling back”句（完整句见 B3）后面**没有标出数据来源**；第四、五张图讲的是 AI 支出集中度（Ramp 数据），不是“企业在收缩”。Ramp AI Index 官方页（数据截至 2026-09）写的是：9 月 AI 采用率 55.9%，较 8 月 +0.6 个百分点、较一年前 +12.1 个百分点；10/11 当周 token 总支出较上月同周 +23.0%；唯一下降的是“AI share of business spend (excluding payroll)” 5.77%（8 月 5.96%，−0.19 pts）。
4. Apricitas 的 **32% 与 62% 不是同一口径**（FRED 页面核到）：32% 对应 FRED 序列 RPSGENAIUSAGESHAREIND17“Adoption Rate for Work: Arts, Entertainment, and Recreation”（只问工作用）；62% 对应 RPSGENAIUSAGESHAREALL“Adoption Rate Overall: Working-Age Adults”，FRED 备注写明含“for work, personal use, or both”。同一调查的“受雇成年人·工作用”序列 RPSGENAIUSAGESHAREWORK 2026 Q3 为 40.5%（文中评论区也有读者在 9/27 指出此点，称当时 42%）。
5. Apricitas 的“70% 书籍销量”是 BEA **实际美元销售额**（图：Real Sales, Recreational Books，Billions of 2017 Dollars），文中措辞是 “fewer books sold” / “70% surge in sales”，不是册数。同段先说“last four years”后说“last six years”。
6. Apricitas 图表自身的两处小瑕疵（只记录）：Hollywood 图副标题写“Last Three Years, Down More than 30%”，而正文写“last four years … more than 100k jobs, nearly one-third”；年同比图副标题拼写为“American Blue Collar Employmentis Falling, With Losses in Newspapers, Books, TV, and Movies”（图是 Creative Class，却写 Blue Collar）。
7. 创意行业总口径：图例列“Movies, Newspapers, Magazines, Books, Radio, TV, Streaming, Social Media, Graphic Design, Performing Arts, Museums, & Employees of Independent Artists”共 8 个子序列，图注“using BLS data”；**正文与图都没有给 NAICS 或 BLS 序列代码**。Fortune 自己用 FRED/BLS 代码算了三个行业合计约 189,000（见 B5）。
8. 10 月 Bloomberg 跟进标题（付费墙）：本组未取得，没有核实；只在线索里有。

## 1. 逐条取证表

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

### B1 Apollo 主结论与三张图

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Apollo 首席经济学家称 AI 对科技业外美国企业“零财务回报” | L1（Apollo 官方 Daily Spark；作者署名 Torsten Slok, Partner, Chief Economist） | https://www.apollo.com/wealth/insights-news/insights/daily-spark/no-signs-of-ai-payoff-outside-the-tech-sector | “Despite massive investments and widespread hype around artificial intelligence, the technology has delivered zero financial payoff to corporate America outside of tech.”（该句在页面中加粗） | 13-apollo-zero-raw.png | 页面日期 “October 24, 2026”，官方未给时刻；标题 “No Signs of AI Payoff Outside the Tech Sector”；栏目 Macroeconomic Indicators & Trends；页脚 Disclosures：“Opinions, estimates and projections constitute the current judgment of the speaker as of the date indicated.” | 已找到 |
| 每员工营收：Mag 7 上升、S&P 493 持平（第 1 图） | L1 | 同上；图 PDF https://www.apollo.com/content/dam/apolloaem/pdf/daily-spark/2026/oct/24/102426_Charts.pdf | “Revenue per worker is rising for the Mag 7 but is flat for the S&P 493, see the first chart below.”；图题 “Revenue per worker moving up for tech but not for the rest of the economy”；图注 “Note: Inflation adjusted to Q4 2019. Sources: Bloomberg, Macrobond, Apollo Chief Economist” | 14-apollo-revenue-raw.png | 轴：Revenue per employee ($000s, inflation adjusted)，2019 年起；读图（非原始数据）：Mag 7 末点约 780，S&P 493 约 500–530 区间波动，末点约 527（较前一点约 512 略升） | 已找到（“flat”是作者对图的描述；读图见左，末点有小幅上行） |
| 利润率：Mag 7 上升、S&P 493 持平（第 2 图） | L1 | 同上 | “Profit margins are rising for the Mag 7 but are flat for the S&P 493, see the second chart below.”；图题 “No signs of profit margins going up outside the tech sector”，图内标题 “Profit margin”；图注 “Sources: Bloomberg, Macrobond, Apollo Chief Economist” | 15-apollo-margin-raw.png | 轴 %，2015 年起；读图：Mag 7 末点约 30%，S&P 493 约 10%。图注没有给“利润率”具体定义（净利润率/营业利润率未写） | 已找到（定义未写明，记为口径缺失） |
| 没有 AI 提升生产率增速的迹象（第 3 图） | L1 | 同上 | “And there are still no signs of AI boosting productivity growth, see the third chart below.”；图题 “No signs of AI boosting productivity growth”，图内 “Productivity growth: TFP vs. Labor output per hour”；图注 “Note: Total Factor Productivity (TFP) is the utilization-adjusted series from the San Francisco Fed (Fernald, 2014), adjusted for cyclical variation in factor utilization. Sources: Federal Reserve Bank of San Francisco, US Bureau of Labor Statistics, Apollo Chief Economist” | 16-apollo-tfp-raw.png | 序列为全经济同比（% yoy），1985 年起，**不是分 Mag 7/S&P 493 的序列**；读图：2026 年末点 TFP 约 −0.3%，Output per hour 约 2.2% | 已找到 |
| 其余两张图（线索摘要未提） | L1 | 同上 | 第 4 图 “AI spending is extremely concentrated in a small number of firms”（表：Top 10% of firms / Bottom 90% of firms；Model Serving & Inference 99.5 / 0.5；Neocloud 99.0 / 1.0；OpenAI + Anthropic 95.6 / 4.4；Cloud computing 95.7 / 4.3；SaaS excluding AI 91.8 / 8.2；Paid Ads 91.9 / 8.1；CRM 84.2 / 15.8），图注 “Sources: Ara Kharazian, Chief Economist, Ramp; Apollo Chief Economist”；第 5 图 “Circularity in AI shows up in token spending too”，“Financial backing of businesses by AI spend intensity”：Top 1% of AI spenders 80.4% VC-backed / 19.6% Non-VC；Median spender 13.1% / 86.9%，图注 “Sources: Ramp AI Index, Apollo Chief Economist” | 21-apollo-concentration-raw.png；22-apollo-vc-raw.png | 第 4 图表头没有标单位（读作各类支出的份额，推断）；第 5 图为 VC 背景占比，不等于“支出”本身 | 已找到 |
| 发布日期与时刻 | – | 同上 | 页面 “October 24, 2026”；页面内嵌数据 `blogDate` 2026-10-24 | 13（日期同框） | 官方未给时刻。图表 PDF 元数据 CreationDate 2026-10-23T10:25:31-06:00（=北京 10/24 00:25），仅是文件元数据，不是发布时刻 | 部分支持（只有日期） |

### B2 口径

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| Mag 7 / S&P 493 的定义、期间、数据源、是否控制行业结构 | L1 | 同上 | 页面和图注**没有**定义 S&P 493（未写“S&P 500 减 Mag 7”，也未写是否按当前成分股回溯）；图注只写 “Sources: Bloomberg, Macrobond, Apollo Chief Economist”；正文末句 “The bottom line is that the AI story will be decided in the S&P 493, not the Mag 7, and so far, there is no evidence that revenue growth or margins are improving outside the tech sector.” | 13、14、15 | 第 1 图期间 2019 年起（通胀调整到 2019 Q4），第 2 图 2015 年起；未见按行业结构调整、也未写成分变动处理。注意：S&P 493 内部也含科技/通信公司（推断，页面未说明），而正文把它称作“outside of tech” | 部分支持（“未控制行业结构”= 原文未写，不等于原文没做） |

### B3 “pulling back”与 Ramp AI Index

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| “企业正在收缩、把 token 支出当成本中心” | L1 | Apollo 同上 | “With no impact on the bottom line, mainstream corporate America is pulling back and starting to view token spending not as a source of growth but as a cost center that weighs on margins.” 紧接 “VC-backed companies are spending more and more on tokens, but mainstream corporate America does not need the latest model and is unwilling to pay for it. As long as agentic AI and token spending are confined to a small part of the economy, the macroeconomic effects of AI will be very limited, see the fourth and fifth charts below.” | 13（前句）；21、22 | “pulling back”一句**未附数据**；第 4、5 图支持的是“支出集中在少数公司/VC 背景公司”，不是“收缩”。 | 部分支持（句子在；其所据数据页面未标出） |
| Ramp AI Index 指标与最新值 | L1（Ramp 官方数据页） | https://ramp.com/data/ai-index | “Monthly measurement of AI adoption and spend across American businesses using transaction data from 85,000+ firms on Ramp.” “DATA THROUGH SEP 2026”；“In September 2026, AI adoption reached 55.9% of U.S. businesses on Ramp, up 0.6 percentage points from August and 12.1 points from a year earlier.” “Total token spending rose 23.0% compared with the same week last month.” “In the week ending October 11, 2026, Anthropic captured the largest share of AI token spending among businesses on Ramp, at 60.1%.” “Among firms paying for AI, the top 1% spent a median of $9,567.53 per employee per month on AI, compared with $15.70 for the median firm.” 表：“AI share of business spend (excluding payroll) 5.77% (Sep 2026) / 5.96% (Aug 2026) −0.19 pts”；“Total token volume rose 52.4%”；“Blended price per million tokens: OpenAI & Anthropic $0.64 / $0.86 −25.4%” | 23-ramp-summary-raw.png；24-ramp-indicators-raw.png | 采用率定义：“the share of businesses with spending on AI models, subscriptions, tokens, and APIs”；样本是 Ramp 客户（卡/账单），方法页自述 “It’s likely our results underestimate actual AI adoption and spend”；token 指标只来自使用 Token Spend Management 的 Ramp 企业，“not representative of Ramp’s total AI spend volume”。行业分项：Information 79.9%、Finance & Insurance 73.7%、Manufacturing 60.9%、Retail 49.9%、Construction 43.2%、Healthcare 42.5%、Accommodation & Food 33.0%（均为 2026-09，较 8 月多数上升，Information −0.1 pt）。时点：页面存档 北京 10/25 23:25。与 Apollo 的表述是否一致：Ramp 页面显示采用率、token 总支出均在上升，只有“占企业总支出比例”小幅下降；Apollo 没有引用该数 | 已找到（与“pulling back”的关系：数据并不直接显示收缩；这是并列记录，不下结论） |

### B4 judgment 段

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| “AI 是好计算器，但商业里智能大多是判断” | L1，**观点段，无数据** | Apollo 同上 | “The question looking ahead is whether, even as models become commoditized, mainstream corporate America can turn increasingly cheap intelligence into higher revenue growth and higher profit margins. That may be harder than it sounds. AI may be a great calculator, but in business, intelligence is mostly judgment. Judgment depends on context, and the answer varies with which human or which model you ask. Perhaps most importantly, AI does not understand accountability, trust, culture, unwritten rules, ethical trade-offs or fairness, all of which shape reputational risk and ultimately drive revenue growth and profit margins.”（“AI does not understand…” 起加粗） | 17-apollo-judgment-raw.png | 段内无图表、无引用；“question looking ahead … may be harder than it sounds”是展望与推测 | 已找到 |

### B5 Apricitas 20 万、近一年 5 万、定义与起点

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 创意行业四年丢 20 万+、近一年约 5 万 | L1（作者自有 Substack；作者 Joseph Politano 署名） | https://www.apricitas.io/p/ai-and-the-fall-of-the-creative-class | “America has lost more than 200k jobs in “creative” industries over the last four years, with roughly 50k lost within the last year alone. That makes for one of the worst stretches for media employment in modern US history, with similar job loss intensity and duration occurring only during the major economic recessions of 2001 and 2008. Yet there’s no generalized recession today—instead, this period of job loss coincides with the rise of AI systems that can compose wholesale novels, photorealistic images, soundalike music, and practically every other form of digital art, en masse and at extremely low costs. Is this the fall of the Creative Class?” | 18-apricitas-200k-raw.png | “creative”带引号；图 “US Arts Employment”，副标题 “US Arts Employment is Down More Than 200k From Their Mid-2022 Peaks”；图例见上文（8 类子序列合计，图注 “Graph created by @JosephPolitano using BLS data”）。**起点是 2022 年中的峰值**（疫情后反弹顶点，读图约 1.46M，当前约 1.21M）；该合计序列 2019 年底约 1.5M，即比疫情前水平低约 30 万（读图，推断）。年同比图（“Year-on-Year Change in US Creative Class Employment”）末点约 −57k（读图），与“roughly 50k”大体吻合。**无 NAICS/BLS 序列代码**，BLS 原始序列页本组**未取**（文中未给代码，不能确定对应哪几条 CES 序列） | 部分支持（数字与图一致；行业口径无代码） |
| 他人复算对照 | L4 | Fortune https://fortune.com/2026/10/02/rise-creative-class-author-richard-florida-ai-grind-jobs/ | “A Fortune review of BLS data found three creative sectors (film and sound recording, broadcasting and streaming, and newspaper, magazine, and book publishing) lost roughly 189,000 jobs. Film and sound recording alone fell 27%, by 120,000 jobs between August 2022 and August 2026.” “I’m not sure his numbers are right,” Richard Florida … told Fortune. | – | Fortune 用 FRED 序列 CES5051200001（电影与录音）、CES5051500001（广播与内容提供商，称 −29,000）、IPUJN5111W200000000（出版业，约 −40,000，2022–2025）；起讫 2022-08→2026-08；发表 2026-10-02T17:25Z | 已找到（仅作对照，数值与 Apricitas 不同口径） |

### B6 AI 归因限定与竞争解释

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 作者对 AI 归因的限定 | L1 | Apricitas 同上 | “Proving the exact amount of AI-driven job loss in the arts is extremely difficult. Media firms are decidedly coy about their AI use, both to protect against public backlash and, more financially important, to preserve the legal basis for their copyrights. The effects of AI are also hard to disentangle from other factors currently affecting arts businesses, like consolidation in Hollywood, offshoring of content production, or the continued displacement of traditional media providers in favor of social media creators. Yet perhaps the clearest evidence of AI’s influence is just that all subsectors of the digital arts industry are losing jobs, while in-person entertainment is still growing at a healthy pace.” 随后：“Some of those digital job losses are just a continuation of prior trends, like in the publishing industry where technological change has been continually grinding away newspaper and magazine jobs for decades. Yet for sectors like live broadcasting, streaming, or graphic design, recent experience is an unusual downturn compared to the tranquility of years prior. Then there’s the worst-hit sector, movie & sound recording, which has been bleeding jobs at a nearly unprecedented pace over the last three years.” | 19-apricitas-difficult-raw.png | 竞争解释（作者自列）：好莱坞整合（consolidation）、内容制作外迁（offshoring）、社交媒体创作者取代传统媒体；另有：出版业长期趋势；流媒体转型 “The streaming era has proven an extremely difficult transition”；“movie ticket sales ended last year down 25% from their 2019 peak, cable revenue is down 18% from its peak”。作者给出的“最清楚的证据”是“所有数字艺术子行业都在掉、线下娱乐仍在增长”，这是相关性论证。注意：同文的分图（Live Arts 面板）显示 Employees of Independent Artists 自 2023 年约 −7%，并非线下全增长（读图）。结语：“In other words, the primary impacts of AI art will be on culture and society rather than the macroeconomy.”；“Right now, AI-driven job displacement has been relatively minor outside of these few narrow sectors.” | 已找到 |
| “nearly half…”句全文 | L1 | 同上 | “Overall, nearly half the media job losses of the last four years have been concentrated in the movie & sound recording sector that includes Hollywood. Written publishing has been the next-largest source of job loss, with employment down by more than 70k over the same time frame. Yet the job losses have by no means been contained to any particular part of media; instead they’ve hit nearly every subsector at some point. Nor do they show any sign of abating, with losses consistently hovering at around 50k per year and even accelerating in recent months. Will further AI development continue displacing workers in creative industries?” 好莱坞：“Hollywood has never seen a stretch as bad as the last four years, with the movie & TV industry losing more than 100k jobs, nearly one-third of the sector’s total.” | 未单独截（文字段位于 18–19 之后；存档在 b-apricitas.txt） | 同上：图副标题写三年、正文写四年 | 已找到 |

### B7 32% / 62% / 70%

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 艺术娱乐休闲业约 32% 员工用生成式 AI，低于全行业平均 62% | L1（作者）；数据 L1/L3（FRED 页面，原始为 Real-Time Population Survey） | Apricitas 同上；链接 https://fred.stlouisfed.org/graph/?g=1Z2RG 与 https://fred.stlouisfed.org/graph/?g=1XQ3Q | “Roughly 32% of workers in the overall arts, entertainment, and recreation sector use Generative AI to some extent, which is less than the 62% average across all industries, but still enough that virtually every major media project could have some AI within its workflow.” | 20-apricitas-books-raw.png（上半段） | FRED g=1Z2RG：序列 RPSGENAIUSAGESHAREIND17 “Generative Artificial Intelligence, Adoption Rate for Work: Arts, Entertainment, and Recreation”，2026 Q3 = 35.63250%（FRED 更新 Oct 20, 2026 10:49 CDT），季度、未季调，读图该序列在 2024Q3–2026Q3 间 22%→43%→29%→36% 大幅波动；来源 Bick, Blandin, Deming（Real-Time Population Survey: Generative AI Adoption Tracker）。FRED g=1XQ3Q：序列 RPSGENAIUSAGESHAREALL “Adoption Rate Overall: Working-Age Adults”，2026 Q3 = 60.80648%；备注：“This series reports the share of adults aged 18-64 who use generative artificial intelligence for any purpose, whether for work, personal use, or both.” 对照序列 RPSGENAIUSAGESHAREWORK “Adoption Rate for Work: Employed Adults” 2026 Q3 = 40.54550%，备注 “Employed respondents are asked: “Do you use Generative AI for your job?””。**所以 32%（工作用·行业）与 62%（任何目的·全成年人）口径不同**。文中 32% 与 FRED 现值 35.6% 不同，应是不同季度/更新前的值（推断：FRED 序列在 9/27 之后更新过；文中引用时点未写）。FRED 页面截图存本地 b-fred-arts-ind17.png / b-fred-all-62.png / b-fred-work-employed.png，未编入 13–24 | 已找到（数字出处找到；比较口径不一致） |
| 纸质书销量 70% 激增，归功 BookTok | L1 | 同上 | “Take the unexpected resurgence in book sales over the last four years. The novel as a commercial art form had been struggling for decades, with fewer books sold in 2019 than at the beginning of the millennium, but the last six years have seen an unprecedented 70% surge in sales. The unlikely savior of the printed word came from perhaps its polar opposite—TikTok.” “…“BookTok” became a new way to source reviews, analysis, discourse, and community in a way that boosted the reading experience and absolutely turbocharged book sales.” | 20（下半段，含图 “US Book Sales Have Rebounded to Record Highs”，副标题 “Book Sales Were Declining in the 2010s, But Surprisingly Rebounded Amidst COVID & TikTok”，图例 “Real Sales, Recreational Books”，纵轴 “Billions of 2017 Dollars”，图注 “Graph created by @JosephPolitano using BEA Data”） | 读图：2019 年前后约 22B，2026 年末点约 37B，约 +65–70%；是**实际美元**销售额（不是册数）。TikTok 因果是作者的归因判断，文中没给对照数据 | 已找到 |

### B8 发布时间

| 项 | 来源 | 值 | 状态 |
|---|---|---|---|
| Apricitas `post_date` | https://www.apricitas.io/api/v1/posts/ai-and-the-fall-of-the-creative-class | 2026-09-27T14:01:59.871Z = **北京 2026-09-27 22:01:59**；`updated_at` 2026-09-27T17:16:10Z（= 北京 9/28 01:16）；页面显示 “SEP 27, 2026”；273 赞（API reactions ❤ 273）、61 评论 | 已找到 |
| Apollo 页面时刻 | Apollo 页 | 仅 “October 24, 2026”（官方未给时刻） | 部分支持 |
| HN 提交 | Algolia | Apollo：id 50224463，587 分 / 623 评（本组存档时刻；线索 581/610），created 2026-10-24T19:47:19Z = 北京 10/25 03:47；Apricitas：id 50226419，439 分 / 458 评（线索 420/449），created 2026-10-24T23:11:54Z = 北京 10/25 07:11 | 已找到 |
| 前期同论点 | Apollo 8/9 Daily Spark | https://www.apollo.com/wealth/insights-news/insights/daily-spark/the-buyers-of-ai-are-still-waiting-for-the-payoff ，页面日期 “August 09, 2026”；“The S&P 493 is spending heavily on AI. But it is not showing up in profit margins, see the first chart below.” “The bottom line is that the AI capex boom is so far only showing up in the sellers’ margins, not the buyers’.” | 已找到 |

### B9 社区质疑（仅作线索 L6）

HN 不公开评论得分，Algolia/Firebase 也无；下列以“同串回复数”代替权重，不是得分。均需回到一手来源核实后才可进正文。

Apollo 帖（https://news.ycombinator.com/item?id=50224463）：
- drivebyhooting（https://news.ycombinator.com/item?id=50224643）：“End of ZIRP, tax law changes, post-COVID inflation, and general economic malaise from federal mismanagement. There are too many confounders.”（混杂因素；该串回复 2 条）
- secabeen（https://news.ycombinator.com/item?id=50224665，回复串 226 条，最大）：以电气化为例，旧工厂围绕水力设计，不能直接换电动机，要重建后才有收益；“A few industries went quickly, but then it took like a decade of little growth before the true benefits showed up.”（时滞论）
- sailfast（https://news.ycombinator.com/item?id=50226833）：质疑把“S&P 493”当结论所在：Mag 7 “Don’t they control a substantial part of GDP?”（样本划分）
- besterman23（https://news.ycombinator.com/item?id=50225547）：反驳“还早”论，称 “493/500 firms haven’t been able to figure that out even though it’s trivially easy to provide employees with frontier models”
- 638d127673（https://news.ycombinator.com/item?id=50227232）：自述小型美国上市公司会计部门使用 AI 后缩短报告准备时间；属轶事。
- fwlr（https://news.ycombinator.com/item?id=50225252）：称生产率收益被 AI 公司以收入形式取走（观点，无数据）。

Apricitas 帖（https://news.ycombinator.com/item?id=50226419）：
- jalev（https://news.ycombinator.com/item?id=50226744）：“Some of these graphs don’t support the hypothesis.” 称电影业就业只略低于 2008 年金融危机后水平，疫情后反弹带来大量招聘再裁员；“just returning to its median?”（基线问题；读图与 Hollywood 图一致：2020–2022 年出现明显反弹峰值）
- alephnerd（https://news.ycombinator.com/item?id=50227036）：列举 2023 好莱坞罢工、疫情关停影院、海外补贴（英国、波兰、爱尔兰等）与美国各州削减补贴（竞争解释）
- ViktorRay（https://news.ycombinator.com/item?id=50226724）、dodger-dog（50226850，“How much is TikTok?”）：需求侧被短视频分流。
- 文章页评论区（Substack）：Keegs（2026-09-27，https://www.apricitas.io/p/ai-and-the-fall-of-the-creative-class/comment/347378806）指出“62%”含个人使用，建议改用 RPSGENAIUSAGESHAREWORK（当时约 42%）；George Santopietro（9/29）问 YouTuber 是否计入数据（原文未答，未见作者回复）。

### B10 夸大实例

| 项 | 来源 | 原句 | 时间 | 发布方 | 状态 |
|---|---|---|---|---|---|
| 把相关性写成“被 AI 摧毁” | Futurism https://futurism.com/artificial-intelligence/creative-work-labor-eviscerated-generative-ai-automation | 标题 “Creative Industries Have Been Eviscerated by AI, Data Shows”；正文 “even the most intimate first-hand knowledge can’t prepare you for the sheer scope of the devastation. As data journalist Joseph Politano found…”（与作者 “extremely difficult” 一段对比：原文没有“Eviscerated by AI”的结论） | 2026-09-29T07:52:52-04:00 = 北京 9/29 19:52 | Futurism（媒体，L4/L5） | 已找到 |
| 较克制的对照 | Gizmodo https://gizmodo.com/creative-jobs-have-plummeted-amid-the-ai-boom-2000818893 | 标题 “Creative Jobs Have Plummeted Amid the AI Boom”；正文 “Politano doesn’t conclusively state that AI is responsible for all those lost jobs.” | 2026-09-29T15:45:25Z = 北京 9/29 23:45 | Gizmodo | 已找到（标题用 “amid”，不算夸大；列作对照） |
| 反向质疑 | Fortune | 同 B5 | 2026-10-02T17:25Z = 北京 10/3 01:25 | Fortune | 已找到 |
| 搜索中见到的转载标题（未核） | dnyuz.com 转载 Fortune，URL slug “we-ve-lost-about-200000-creative-jobs-since-the-rise-of-ai-the-urbanist-who-coined-the-creative-class-said-ai-is-coming-for-the-grind-out-jobs” | 未取页面，仅见检索结果 slug；转载站不用作来源 | 2026-10-02（按 URL 路径） | dnyuz（转载站） | 未找到一手来源（不可用） |
| “AI 投资被证明零回报”“AI 泡沫破了”类标题（Apollo） | WebSearch 三次（中英文） | 未检出 10/24 之后把 Apollo 写成“实锤零回报/泡沫破裂”的英文或中文标题；检出的 2026-06/07/08 稿为转述早期 Apollo 笔记。Bloomberg 标题 “Apollo’s Slok Says AI Has Delivered Zero Payoff Outside of Tech”（Claude 提供，付费墙）本组未取到 | – | – | 未找到（找不到就写找不到；Bloomberg 未核） |
| 中文搬运 “AI 没有回报”（WorkBuddy 称） | 未在检索中找到具体稿件 | – | – | – | 未找到 |

## 2. 可能的吠点（原文里有、线索摘要没提或说反的条件）

1. Apollo “zero financial payoff”是加粗导语；支撑它的前三图只显示“S&P 493 的每员工营收和利润率持平、全经济 TFP 无上行”，没有逐公司、逐行业，也没有任何“AI 使用强度”变量；页面与图注没有定义 S&P 493、没有控制行业结构（Apollo 页 + 图注）。
2. “pulling back”句无数据来源；Ramp AI Index 同期显示采用率与 token 支出上升，仅“AI 占企业总支出比例”微降 0.19 pts（Ramp 官方页，已存档）。
3. Apollo 自己把 judgment 段放在“展望”里，写 “That may be harder than it sounds” 与 “Perhaps most importantly…”，是观点；页脚声明意见与预测是“current judgment of the speaker”。
4. TFP 图是全经济序列（FRB SF 利用率调整，Fernald 2014），不能区分 AI 与其他因素（图注）。
5. Apricitas 开头是“创意行业近四年 −20 万”，起点是疫情后 2022 年中峰值；合计序列 2019 年底高于该起点（读图）。
6. 作者自己写明“Proving the exact amount of AI-driven job loss in the arts is extremely difficult”，并给出整合、外迁、社交媒体取代三类竞争因素；结尾又写 AI 对宏观经济的主要冲击在文化而非经济（Apricitas）。
7. 32% 对 62% 口径不同（见 B7）；FRED 现值 35.6%，且该序列季度波动大。
8. “70% 书籍销量”是实际美元销售额；“BookTok”是作者的解释，文中没有给对照数据。
9. 图表小瑕疵：Hollywood 图副标题“三年/ >30%”对正文“四年/ >100k、近三分之一”；年同比图副标题把“Creative Class”写成“Blue Collar”。
10. Fortune 同期复算三个行业合计约 189,000，Florida 称 “I’m not sure his numbers are right”（口径不同，不能互相抵消）。
11. 社区：混杂因素（ZIRP、疫情后反弹基线、罢工、海外补贴）、时滞论（电气化类比）、S&P 493 是否有代表性（HN 评论链接见 B9，L6，无得分）。

## 3. 扫描说法勘误

| 扫描/已知说法 | 实际 |
|---|---|
| Apricitas “扫描称发表于 9/27（窗口前）” | 确认：北京 9/27 22:01:59（Substack API）。不是新文章。 |
| “nearly half the media job losses of the last f[ew years]…” | 全文：“…last four years have been concentrated in the movie & sound recording sector that includes Hollywood.” |
| “…mainstream corporate America is pulling bac[k] …” 后半句 | 全文：“…pulling back and starting to view token spending not as a source of growth but as a cost center that weighs on margins.” |
| Ramp AI Index “付费 AI 订阅企业占比（如）” | 官方定义：“the share of businesses with spending on AI models, subscriptions, tokens, and APIs”（有任一 AI 付费交易的企业占比，来自 Ramp 卡/账单；样本为 Ramp 客户）。 |
| Apollo 有 3 张图 | 页面共 **5 张图**（“5 pages”）：营收/人、利润率、TFP、AI 支出集中度（表）、VC 背景占比。 |
| Apollo 日期时刻“未核” | 仅有日期 October 24, 2026，官方未给时刻。 |
| HN Apollo 581/610；Apricitas 420/449 | 本组 10/25 取到 587/623 与 439/458（逐步增长，时刻不同）。 |
| “TFP 为旧金山联储经利用率调整的序列（Fernald, 2014）” | 确认，图注原句见 B1。 |
| “纸质书销量 70% 激增” | 原文 “70% surge in sales”，图为 BEA 实际美元销售额，类别 “Recreational Books”（见 B7），非册数；“纸质”二字原文没有。 |
| Apricitas “32% 对 62%” | 口径不同（工作用·行业 vs 任何目的·全成年人），见 B7。 |

## 4. 未完成 / 未核

- BLS 原始序列页未取（文中未给 NAICS/序列代码，无法确定合成口径；Fortune 给出的三个 FRED/BLS 代码仅作对照）。
- Bloomberg 10/25 标题、中文财经号搬运稿：未取得/未找到。
- HN 评论得分：平台不公开，未取得。
- Apollo“No Impact of AI Expected for Corporate Earnings Outside Tech”（apolloacademy.com，已存档 b-apollo-prior-jun.*）发表日期未从页面提取（二手稿称 6/30），未核。
- FRED 页面 `barking fetch` 超时失败，改用 headless Chrome 渲染得到截图（见日志）；FRED 文本存档（txt）未取得。
