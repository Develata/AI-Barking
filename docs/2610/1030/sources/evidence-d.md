# 1030 期取证 D 组：速览（公司与产品）

执行方：claude-haiku-5-5。时间一律北京时间（UTC+8）；原时区括号保留。抓取与热度记录见 `capture-log-d.md`。未截图（D 组不需要）。存档文件均在 `docs/2610/1030/sources/` 下，前缀 `d-`。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

## 〇、各条一句事实（速览用，照原文限定）

- **D1 Reddit**：Reddit 2026 年第三季度 DAUq 平均 1.303 亿，同比增长 12%，营收 8.95 亿美元，同比增长 53%（股东信与新闻稿，L1，10-29 美东发布）；股东信称 10 月搜索引荐平均每周减少超过 100 万，并称若此速度持续，Q4 DAUq 可能低双位数百分比下降（L1）；Reddit 计划 Q4 起在美国以外部分市场全面测试登录后访问，全面推行没有时间表（股东信为 L1，“coming months”一句为 The Verge 转述 CEO，L4）。
- **D2 robots.txt 训练偏好**：OpenAI（10-30）、Google（10-30）、Microsoft（10-30）都表示将支持共同的 robots.txt 训练偏好信号，实施时间分别为“数月内”“2027 年”“2027 年 2 月 1 日起”（均为 L1，官方页未给时刻）；Anthropic 的一位员工在 IETF 邮件中称公司“已开始实施”，尚需时间（L2）。该草案是个人提交的 Internet-Draft，不是工作组文件（L1）。
- **D3 OpenAI 转介**：OpenAI 称 2026 年第三季度向执法机构转介超过 200 次，转介对象为“迫在眉睫的暴力威胁”（L1，官方页日期 10-30，未给时刻）；首份社区安全报告计划 2027 年初发布，覆盖 2026 年下半年（L1）；未成年人的武器相关信息限制“将在接下来几周推出”（L1）。
- **D4 Amazon Alexa for Shopping**：Amazon 新增 100 多项购物操作，仅向美国客户开放；“过去 12 个月逾 4 亿客户”指整个 Alexa for Shopping（L1，北京 10-29 20:55）。

## 一、逐条取证

### D1 Reddit Q3：Google 搜索引荐下降、拟试点登录后访问

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Q3 DAUq 1.303 亿，同比 +12% | L1 | 股东信 https://s203.q4cdn.com/380862485/files/doc_financials/2026/q3/v2/Q3-26-Shareholder-Letter.pdf （`d-d1-shareholder-pdf.pdf` / `.txt`） | "Daily Active Uniques (“DAUq”) averaged 130.3 million, an increase of 12% year-over-year" | 无 | Q3 平均值；DAUq 为 Reddit 定义的日活独立用户 | 已找到 |
| 营收 $8.95 亿，同比 +53% | L1 | 新闻稿 https://www.sec.gov/Archives/edgar/data/1713445/000171344526000114/earningspressreleaseq326.htm （`d-d1-sec-q3-press-release.html` / `.txt`，Reddit 8-K 附件）；股东信同上 | 新闻稿："SAN FRANCISCO, Calif. – October 29, 2026 – Reddit, Inc. … today announced financial results for the quarter ended September 30, 2026." 股东信："Total revenue was $895.0 million, up 53% year-over-year" | 无 | 美元，Q3 | 已找到 |
| 十月搜索引荐平均每周减少超过 100 万 | L1 | 股东信（同上） | "During October, we have seen that search traffic has dropped by more than 1 million referrals on average per week." | 无 | 单位是 referrals（搜索引荐）；信中写 “search traffic”，段落未点名 Google；新闻稿不含该数字 | 已找到（数字与单位）；Google 归因见下一行 |
| 引荐下降归因于 Google 的 AI Mode / AI Overviews | L4（The Verge 转述电话会） | https://www.theverge.com/tech/1022848/reddit-ceo-google-ai-mode-traffic （`d-d1-verge.html` / `.txt`） | "The reality is, AI Mode does not drive traffic," Huffman says. | 无 | 电话会逐字稿未取得，仅媒体转录；股东信与新闻稿均未提 Google、AI Mode 或 AI Overviews | 部分支持 |
| 若减速持续，Q4 DAUq 可能低双位数百分比下降 | L1 | 股东信（同上） | "If this rate of decline continues, current trends would suggest that total DAUqs could fall in Q4, by a low-double-digit percentage from our current base of 130 million DAUqs." | 无 | 条件句，不是预测结论 | 已找到（限定：条件性） |
| 拟试点登录后访问（“logged-in-only”） | L1（股东信）；L4（The Verge 转述 CEO 帖） | 股东信（同上）；The Verge 登录稿 https://www.theverge.com/tech/1022571/reddit-login-requirement-q3-2026-earnings （`d-d1-verge-login.*`） | 信："shifting Reddit to a logged-in-only experience" ；"In Q4 we’ll begin fully testing this in select markets outside the U.S. The results of those tests will inform an optimized version of logged-in-only Reddit globally." Verge 转述 CEO："To start, Reddit will test the logged-in requirement in ‘select markets outside the US’ in the ‘coming months’" ；"There’s no definitive timeline on when this change will roll out to all users" | 无 | 限定：海外部分市场的测试；信中用语是 “primarily logged-in”；测试起始时间两处不同：信 “In Q4”，Verge 转述 “coming months” | 部分支持（见勘误：不是“考虑”，是已宣布的测试计划；全面推行无时间表） |

Reddit 投资者新闻稿（investor.redditinc.com）：Cloudflare 质询页，curl 与 headless 渲染均未取得正文。同一份新闻稿已从 SEC 8-K（accession 0001713445-26-000114，2026-10-30 申报）的 EX-99.1 取得，即上表的新闻稿存档。

CEO 登录帖（reddit.com/user/spez/comments/1xiupm0）：fetch 被拦截（正文过短，需要 JS）；`.json` 接口返回 403；headless 渲染被 reCAPTCHA 验证页拦截，未尝试绕过。CEO 原话只经 The Verge 转述，级别 L4。

发布时刻：
- 新闻稿：标题与导语为 “SAN FRANCISCO, Calif. – October 29, 2026”，官方未给时刻。财报电话会排定为 “Thursday, October 29, 2026, at 1:30 p.m. PT / 4:30 p.m. ET”，即北京 10-30 04:30。
- The Verge 登录稿：2026-10-29 20:07 UTC（北京 10-30 04:07）。
- The Verge 财报稿：2026-10-29 22:19 UTC（北京 10-30 06:19）。

### D2 四家支持 robots.txt 训练偏好信号

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| OpenAI 将在 robots.txt 中遵守共同信号 | L1 | https://openai.com/index/helping-websites-express-ai-training-preferences/ （`d-d2-openai.html` / `.txt`） | "Today, we’re announcing that OpenAI will honor a common signal in robots.txt, the file that lets websites express their preferences about the use of their content for foundational generative AI training." | 无 | 官方页日期 October 30, 2026，官方未给时刻 | 已找到（宣布，尚未实施） |
| OpenAI 实施时间 | L1 | 同上 | "We aim to implement the signal in the coming months, and will share another update with details about our implementation when we do so." | 无 | “数月内”为目标，非确定日期 | 已找到 |
| OpenAI 信号的范围是否含搜索或实时检索 | L1 | 同上 | 原文只写 “foundational generative AI training”；另写 "This new signal will not replace the controls we already provide through robots.txt." | 无 | 原文只写训练；搜索与实时检索原文未提及 | 部分支持 |
| Google 将支持 | L1 | https://developers.google.com/search/blog/2026/10/ai-training-control （`d-d2-google-blog.html` / `.txt`） | "We aim to support this new control along with other AI providers in 2027, and will provide updates on our progress." ；"Google plans to interpret that as if the publisher had disallowed Google-Extended" | 无 | 2027 年；博文日期 Friday, October 30, 2026，官方未给时刻 | 已找到（计划，2027 年） |
| Microsoft 将支持 | L1 | https://blogs.bing.com/search/2026/10/your-content-your-choice-clearer-preferences-for-training-and-grounding/ （`d-d2-bing-blog.html` / `.txt`） | "Starting February 1, 2027, Microsoft will honor the new preferences as it crawls" | 无 | 2027-02-01 起；博文日期 October 30, 2026，官方未给时刻。另设页面级 AI Grounding 偏好，不属共同信号 | 已找到（计划，2027-02-01 起） |
| Anthropic 已支持 | L2（员工公开发言：IETF 邮件作者；身份据 The Verge 为 Anthropic 技术标准负责人，L4） | https://mailarchive.ietf.org/arch/msg/ai-control/mddjatOxSR0h6GmLRBWlc839c7w/ （`d-d2-anthropic-ietf-mail.*`） | "I'm also excited to share that Anthropic has already started implementing support for this draft." ；"It'll take us some time to add support across our services and work through the inevitable complications and edge cases that I'm sure we'll discover." | 无 | 邮件时间 Fri, 30 Oct 2026 15:39 UTC（北京 10-30 23:39）；写法为 “Anthropic 员工称”，不写 “Anthropic 称已支持” | 部分支持（“已开始实施”，并非“已支持”） |
| 草案的性质 | L1 | https://datatracker.ietf.org/doc/draft-kelley-aipref-training-preference-signal/ （`d-d2-ietf-draft.html` / `.txt`） | 页面标题 "draft-kelley-aipref-training-preference-signal-00 - Machine-Readable Training Preference Signal"；页面字段标注 "(individual)"；页面文本未列作者单位 | 无 | 个人提交的 Internet-Draft，不是工作组文件 | 已找到 |
| 语法：User-Agent: __usage_ai-training | L1 | IETF 邮件（同上）；Bing 博文同样列出 `User-Agent: __usage_ai-training` | "User-Agent: __usage_ai-training" / "Disallow: /" | 无 | 仅政策令牌（policy token）语法；RFC 9309 路径匹配 | 已找到 |
| 四家公司都表态支持共同信号（The Verge） | L4 | https://www.theverge.com/tech/1023763/openai-google-microsoft-ai-scraping-standard-opt-out （`d-d2-verge.html` / `.txt`） | "OpenAI, Google, Microsoft, and Anthropic say they will restrict their AI crawlers from accessing a site if they encounter the shared token." ；"While Anthropic has already adopted the signal, Google and Microsoft plan to support it next year. OpenAI says it will implement it in the 'coming months.'" | 无 | UTC 时间 Oct 30, 2026, 8:26 PM（北京 10-31 04:26）。“already adopted”对 Anthropic 超出其员工原话 | 部分支持（“表态”属实；“已采纳”不准确） |

IETF 工作组页面 https://datatracker.ietf.org/wg/aipref/about/ （`d-d2-ietf-aipref.*`）：工作组章程，原文为 “The AI Preferences Working Group will standardize building blocks …”。OpenAI 博文引用的工作组草案 draft-ietf-aipref-vocab 也已存档（`d-d2-ietf-vocab.*`）。

### D3 OpenAI Q3 向执法机构转介超 200 次

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Q3 2026 向执法机构转介超过 200 次 | L1 | https://openai.com/index/community-safety-report-and-new-protections-for-teens/ （`d-d3-openai.html` / `.txt`） | "we made over 200 law enforcement referrals in Q3 2026" | 无 | “over 200”即超过 200；官方页日期 October 30, 2026，官方未给时刻 | 已找到 |
| 转介对象的定义 | L1 | 同上 | "Our work includes bringing imminent threats of violence to the attention of law enforcement" | 无 | 原文只给“迫在眉睫的暴力威胁”的定义；是否经人工审核，原文未说明 | 部分支持 |
| 首份周期性社区安全报告 | L1 | 同上 | "We will publish our first report in early 2027, covering the second half of 2026, with updates every six months thereafter." | 无 | 2027 年初；原文报告名两种写法：导语段 "a new Community Safeguards Report"，章节标题 "Community Safety Report" | 已找到 |
| 未成年人新限制 | L1 | 同上 | "We’re now placing further limits on weapons-related information teens can receive, even when a question appears harmless." ；"The safeguards detailed here will begin rolling out in the next few weeks and we will continue to refine them." | 无 | 面向 18 岁以下用户（ChatGPT for Teens）及年龄预测判定的未成年人；开始推出为“接下来几周” | 已找到 |
| 路透社 10-30 报道 | L4 | 未找到 | 搜索未返回该篇报道 | 无 | — | 未找到一手来源（报道未核） |

### D4 Amazon Alexa for Shopping 新增百余项操作

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 新增 100+ 项操作 | L1 | https://www.aboutamazon.com/news/retail/alexa-for-shopping-new-capabilities （`d-d4-amazon.html` / `.txt`） | 标题 "Alexa for Shopping adds 100+ new capabilities to make shopping even faster and easier" ；正文 "handling more than 100 common shopping actions on Amazon" ；Mehta 引语 "can now help with more than 100 brand-new shopping actions" | 无 | 标题写 “100+ new capabilities”，正文与引语写 “more than 100 … shopping actions”；原文未列完整清单 | 已找到 |
| “过去 12 个月逾 4 亿客户”指整个 Alexa for Shopping | L1 | 同上 | "Over 400 million customers have used Alexa for Shopping in the last 12 months" | 无 | 指整个产品，不是新增的 100 项操作 | 已找到 |
| 地区限制 | L1 | 同上 | "These capabilities are available now to all U.S. Amazon customers." | 无 | 仅限美国 | 已找到 |
| agentic 功能用户数增长 | L1 | 同上 | "the number of customers using Alexa for Shopping’s agentic features has grown more than eight times since the start of the year." | 无 | 相对今年年初的增长倍数，未给绝对数 | 已找到 |

发布时刻：页面元数据 `article:published_time` 为 2026-10-29T12:55:32.402Z，即北京 2026-10-29 20:55（时间戳来自页面数据，L1）。

## 二、热度（barking heat，北京时间）

取得时刻 2026-10-31 16:53（北京）。HN 为 Algolia 搜索；Reddit 搜索被重定向到登录页，失败。

| 条目 | Hacker News（主帖 ID、分数/评论、时刻） | Reddit |
|---|---|---|
| D1 Reddit | 50295893 “Reddit will require an account to access”（指向 CEO 帖）58 分 / 15 评，10-30 05:01；50295611 “Reddit will force everyone to log in”（指向 Verge）30 分 / 6 评，10-30 04:43；50295247 TechCrunch 6 分 / 4 评，10-30 04:19 | 失败（old.reddit 搜索跳转登录页） |
| D2 robots.txt | 50309270 “Google plans to support shared AI training opt-out via robots.txt” 7 分 / 0 评，10-31 03:51。“robots.txt” 查询仅命中这一条与一篇 2026-10-28 的旧研究，未见 OpenAI、Microsoft、Anthropic 相关讨论 | 未查 |
| D3 OpenAI 转介 | 查询 “OpenAI safety report”、“OpenAI referrals”，命中 0 条 | 未查 |
| D4 Alexa | 查询 “Alexa for Shopping”、“Amazon Alexa shopping”，命中 0 条（“Alexa” 命中 35 条，均为人名或无关标题） | 未查 |

## 三、扫描说法勘误

扫描（ChatGPT、WorkBuddy 两份回贴）与 Claude 初核的说法，以下按原文改写：

1. D1 “十月 Google 引荐‘平均每周减少超过一百万’”：数字准确（股东信，L1），但股东信写的是 “search traffic” 与 referrals，段落未点名 Google。“Google”来自 The Verge 对电话会的转述（L4）。
2. D1 “DAUq 1.303 亿 +12%”“营收 $8.95 亿 +53%”：准确。
3. D1 “考虑在部分海外市场试行登录后访问”：措辞偏弱。股东信写的是 “In Q4 we’ll begin fully testing this in select markets outside the U.S.”，即已宣布的测试计划；Verge 转述 CEO 的时间是 “coming months”，两处时间不同，需写明。
4. D1 漏掉的限定：股东信称若减速持续，Q4 DAUq 可能 “low-double-digit percentage” 下降，这是条件句。
5. D1 Verge 标题 “Reddit will force everyone to log in” 与正文不符：正文写的是先在美国以外部分市场测试，全面推行没有时间表。
6. D2 “Anthropic 已支持”：不准确。作者的 IETF 邮件是 “has already started implementing support”，并写明 “It'll take us some time to add support across our services”。按 L2 写作“Anthropic 员工称已开始实施”。
7. D2 “四家支持”：四家都表态会支持，但都未上线。OpenAI 为 “coming months”，Google 为 2027，Microsoft 为 2027-02-01 起，Anthropic 为 “已开始实施”。
8. D2 “IETF AI Preferences 工作组”：草案 draft-kelley-aipref-training-preference-signal 是个人提交（datatracker 标注 “(individual)”），不是工作组文件。工作组自己的词汇草案是 draft-ietf-aipref-vocab。
9. D2 漏掉的范围与限定：OpenAI 的信号针对 “foundational generative AI training”，原文未说是否管搜索或实时检索；Microsoft 另设 AI Grounding 页面级偏好，不在共同信号内，且其博文称不影响 Bing 搜索收录。
10. D3 “迫在眉睫的暴力风险”：准确（原文 “imminent threats of violence”）。
11. D3 “首份周期性社区安全报告 2027 年初”：准确；原文报告名写法不一（Community Safeguards Report / Community Safety Report）。
12. D3 “超 200 次”：原文为 “over 200 law enforcement referrals in Q3 2026”，应保留“超过”。转介的定义与是否人工审核，原文未说明。
13. D3 “路透社 10-30”：未找到该报道，不能核实。
14. D4 “逾 4 亿客户指整个 Alexa for Shopping”：准确。
15. D4 “新增百余项操作”：标题是 “100+ new capabilities”，正文与引语是 “more than 100 … shopping actions”，口径不完全一致，原文都保留。
16. D4 漏掉的地区限制：功能仅向美国 Amazon 客户开放（原文 “available now to all U.S. Amazon customers”）。

## 四、未找到或失败

- Reddit 投资者网站（investor.redditinc.com）：Cloudflare 质询，未取得正文；同一新闻稿已从 SEC 8-K 附件取得，上表已用其存档。
- Reddit CEO 登录帖（reddit.com/user/spez/…）：fetch 被拦截，`.json` 返回 403，headless 渲染被 reCAPTCHA 挡住；CEO 原话只经 The Verge 转述（L4）。未尝试绕过验证。
- Reddit 搜索（old.reddit.com）：跳转登录页，热度中的 Reddit 部分失败。
- 路透社 10-30 关于 OpenAI 转介的报道：搜索未返回。
- 股东信与新闻稿中没有 “AI Mode”、“AI Overviews”、“LLM” 或 “search engine” 的归因；股东信里的 Google 仅出现在 Google Play 的应用商店说明中。因此 Google 归因只有 L4。
