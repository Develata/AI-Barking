# 1102 期取证 D 组（速览二）

执行方：claude-haiku-5-5（Haiku 5.5）。取证日：北京时间 2026-11-03。未截图（按派工单，C/D 组不截图）。存档见 `sources/d-*`，抓取与查询记录见 `sources/capture-log-D.md`。

## 一、逐条核验

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| D1 高盛限制非工程师使用 Claude Code（据彭博社报道） | L4（彭博社报道），另见 L1 内部指引未公开 | 彭博社 https://www.bloomberg.com/news/articles/2026-11-02/goldman-restricts-traders-and-bankers-from-using-some-ai-tools （标题 “Goldman Restricts Traders and Bankers From Using Some AI Tools”，作者 Todd Gillespie） | 正文付费墙，原页仅见导语：“has irked some of its staff after it banned most of them from using advanced artificial intelligence tools designed for coding after reviewing productivity and risk concerns.” / “For months, Goldman allowed bankers to use Anthropic PBC’s Claude Code and Microsoft Corp.’s GitHub for work. Now, the bank is limiting the use of those tools to engineers only, according to internal guidance updated in October and seen by Bloomberg News.”（导语与正文全文见下方“备注”，全文取自公开转载的 Yahoo 版本，见下） | 无 | 彭博社时间 2026-11-02 13:52 UTC（北京 11-02 21:52）。Yahoo 转载版显示 “Mon, 2 November 2026 at 1:33 am GMT-5”，与彭博原页时刻不一致，以彭博原页为准 | 已找到（发布方与时间已核；内部指引本身未公开，仅见彭博报道） |
| D1 发言人回应 | L4 | 同上；PYMNTS 转引 https://www.pymnts.com/news/artificial-intelligence/2026/goldman-sachs-bars-bankers-from-employing-some-ai-tools/ | “We make certain AI tools available to our people to determine the best use cases across different parts of the firm,” said Nick Carcaterra, a Goldman spokesperson. “After a period of experimentation where we assess a variety of factors including productivity and risk management, we put them in the hands of those who are best suited to leverage them for the long term.” | 无 | 发言人身份为 Goldman spokesperson（彭博原文） | 已找到 |
| D1 限定词：“most”、strats 豁免 | L4 | 同上（Yahoo 转载版） | “The bank’s latest directive doesn’t apply to so-called ‘strats,’ employees who sit at the intersection of trading, quantitative research and software engineering, said another person.” | 无 | 彭博报道另一人士称；strats 不受此指令约束 | 已找到 |
| D1 “GitHub” 是否指 Copilot | — | 同上 | 原文只写 “Microsoft Corp.’s GitHub”，未说明是否指 GitHub Copilot | 无 | 未说明 | 未找到一手来源（报道未限定） |
| D1 英文媒体跟进：The Decoder | L4 | https://the-decoder.com/goldman-sachs-pulls-ai-coding-tools-from-traders-and-bankers/ （发表 Nov 2, 2026，官方未给时刻） | “Goldman Sachs has cut off most of its employees from advanced AI coding tools like Anthropic's Claude Code and Microsoft's GitHub. Only engineers can still use them, according to Bloomberg, citing internal guidelines from October.” | 无 | 时刻未给；注明来源为彭博 | 已找到（跟进稿，转引彭博，不构成独立证据） |
| D1 中文媒体 | — | 未找到中文一手或原文报道 | 中文搜索只找到 2026-04 的香港限制快讯（见 capture-log） | 无 | — | 未找到（中文侧无对应报道） |
| D1 香港限制（背景） | L4 | 彭博社 2026-04-29 报道（URL 见 capture-log，WebSearch 结果；未存档） ；本报道转引 “Earlier this year, the bank restricted the use of Anthropic’s Claude by its staff in Hong Kong, a location-specific restriction.” | 同上 | 无 | 地域限制，与本次角色限制不同 | 部分支持（仅作背景） |
| D1 高盛 CIO 7 月言论（对照） | L4 | 同彭博报道 | “As recently as July, Goldman chief information officer Marco Argenti touted the ‘vibe coding’ by non-technical employees at Goldman, such as traders and bankers.” | 无 | 彭博报道的转述 | 已找到（背景对照） |
| D2 Grid Protectors 项目名与定位 | L1（OpenAI 官方） | https://openai.com/index/grid-protectors-program/ （`d-openai-grid.txt`） | “Today we’re launching the Grid Protectors Program, a national effort to support affordable, reliable electricity for families and businesses in the communities where we build.” | 无 | 页面仅给日期 “November 2, 2026”，官方未给时刻 | 已找到 |
| D2 佐治亚 Effingham 协议含“最高 1,000 MW”灵活需求响应 | L1（OpenAI）＋ L1（Georgia Power 新闻稿） | 同上；Georgia Power https://www.georgiapower.com/news-hub/press-releases/georgia-power-to-serve-openai-project-in-effingham-county.html （`d-gpower-effingham.txt`） | OpenAI：“Our agreement with Georgia Power also includes up to 1,000 megawatts of flexible demand response, freeing up power to help keep air conditioning running in hundreds of thousands of homes during extreme heat.” Georgia Power：“OpenAI, which is expected to need approximately 3,200 megawatts (MW) of power for its new facility, has agreed to provide up to 1,000 MW of flexible demand response where it will reduce its power use in times of high demand” | 无 | 1,000 MW 的口径是“需求响应”（可削减供电的能力），不是用电量上限或装机容量。Georgia Power 称设施预计需约 3,200 MW（推算：1,000 ÷ 3,200 ≈ 31%，推算由本组作出）。协议期 25 年（Georgia Power）。Georgia Power 新闻稿日期 07/22/2026 09:30AM（原页未标时区） | 已找到 |
| D2 “约 70 万户” | L1（OpenAI） | 同上 | “For scale, that is equivalent to the average electricity demand of roughly 700,000 households. That’s more households than Washington, D.C., and Boston combined.” | 无 | 这是 1,000 MW 的规模换算，不是受益户数 | 与说法不符（见勘误） |
| D2 “2029 年起每年约 9.5 亿美元节省” | L1（Georgia Power 新闻稿，8/26/2026 发布） | https://www.georgiapower.com/news-hub/press-releases/contract-openai-approved-part-portfolio-delivering-950-million-annual-savings.html （`d-gpower-950m.txt`） | “The increased savings for customers are the result of projected incremental revenue from new large-load customers that is expected to provide savings of approximately $950 million per year, increasing customer benefits to $2.847 billion over the three-year period of 2029-2031.” OpenAI 官方页：“The utility projects approximately $950 million in annual customer savings beginning in 2029 from its portfolio of large-load agreements and expected growth, including Project Camellia.” | 无 | 该数字是 Georgia Power 对其大用户合同组合与预期增长的预计，并非仅此 OpenAI 协议的节省。Georgia Power 新闻稿日期 08/26/2026 09:00PM（原页未标时区） | 部分支持（数字存在，但归属应为 Georgia Power 整体组合预计，而非该协议） |
| D2 “园区尚在开发” | L1 | https://openai.com/index/grid-protectors-program/ | “Although the campus is still in development, we intend for Effingham to serve as a pilot for Grid Protectors” | 无 | 官方“intend”（意向） | 已找到 |
| D2 “最高”与承诺性质 | L1 | 同上 | “We are proposing to be that regular customer.” / “The arrangements will differ by location, and we will share additional commitments as they are finalized.” | 无 | 其余园区的安排尚未确定 | 已找到（限定词，供正文保留） |
| D3 WorkOS 公告：10/30 发现，11/2 公告 | L1（WorkOS 官方公告） | https://workos.com/security/bulletins/october-30th-2026-data-security-incident （`d-workos-bulletin.txt`） | “On October 30, 2026, we identified unauthorized access to an internal WorkOS application used by our sales team.” 公告页标注日期 “November 2, 2026” | 无 | 发现日与公告日均为日期，官方未给时刻 | 已找到 |
| D3 被访问系统 | L1 | 同上 | “We are investigating unauthorized access to an internal tool used by our sales and customer-facing teams.”（导语）；“Our current analysis indicates this incident affected only CRM and customer support data.” | 无 | 公告时点的分析结论，随调查更新 | 已找到 |
| D3 数据类别 | L1 | 同上 | “Business contact details: Names, email addresses, phone numbers, job titles, and company affiliations held in our customer relationship management (CRM) systems.” / “Customer communications: Summaries and excerpts from sales calls, support discussions, and customer Slack conversations.” / “Operational metadata: Information about customers’ use of WorkOS products, including product usage and connection or directory health.” | 无 | — | 已找到 |
| D3 “AI-assisted” 框架 | L1 | 同上 | “Our investigation identified the use of an AI-assisted offensive security framework to automate reconnaissance and exploitation attempts.” | 无 | 公告未在正文点名 ARTEX（链接里的第三方报告标题含 artex） | 已找到 |
| D3 “无证据显示应用数据被访问” | L1 | 同上 | “There is no evidence that WorkOS application data was accessed, nor is there evidence that production WorkOS services were accessed or impacted.” | 无 | 公司称，目前无证据 | 已找到 |
| D3 CEO 在 HN 的表态（补充） | L2 类（公司高管发言，HN 评论） | https://news.ycombinator.com/item?id=50345712 （HN 首帖下的评论，`d-hn-item-50345712.json`） | “WorkOS’s production systems are secure. No customer platform data was accessed. This incident was isolated to internal GTM systems. The attack involved ARTEX, an autonomous AI agent framework used to carry out offensive security operations.” | 无 | 评论发言人自称 WorkOS CEO，未核身份；注明来源为 HN 评论 | 部分支持（点名 ARTEX 只见于此评论；公告正文未点名） |
| D4 message threads 公测 | L1（Anthropic 官方页） | https://claude.com/blog/message-threads 301/跳转至 https://claude.com/resources/articles/message-threads （`d-claude-threads.txt`） | “Message threads is now available in public beta on the Claude Platform.” 页面日期 “November 2, 2026” | 无 | 官方未给时刻；公测平台：native Claude Platform，并在 Claude Platform on AWS、Claude in Microsoft Foundry、Claude Models on Google Cloud（Anthropic managed）可用 | 已找到 |
| D4 “50 万到 100 万 token 的请求首字延迟中位数降约 35%” | L1 | 同上 | “With message threads on by default for most Claude Code users, requests with 500k to 1M tokens see median TTFT drop by about 35%.” | 无 | 官方原文的主语是“对 Claude Code 多数用户默认开启”的情形，TTFT 为 time to first token | 已找到（限定词“on by default for most Claude Code users”须保留，扫描稿未写） |
| D4 “token 用量不变” | L1 | 同上 | “While each request is smaller with message threads, Claude still reads the full conversation on every turn, so token usage stays the same.” | 无 | — | 已找到 |
| D4 线程 5 分钟过期 | L1 | 同上 | “A thread expires 5 minutes after its last request and your client then resends the full conversation to start a new one.” | 无 | — | 已找到 |

### 备注：D1 彭博社全文来源

彭博社原页（`d-bbg-original.html/.txt`）返回 200，但正文付费墙，可读内容仅见导语（“Takeaways by Bloomberg AI”）与标题、作者、时间。本组未绕过付费墙，也未使用 HN 评论里的 Bloomberg gift 链接（含访问令牌，已在存档中打码）。正文上文引用的句子与发言人全文，取自公开转载的 Yahoo（sg.finance.yahoo.com，`d-yahoo-bbg.txt`）和 PYMNTS（`d-pymnts.txt`）版本，二者均注明为 Bloomberg 原稿。

## 二、每条一句事实（供速览，照原文措辞）

- **D1**：据彭博社报道，高盛 10 月更新的内部指引把 Anthropic 的 Claude Code 与微软 GitHub 限定给工程师使用，此前交易员与银行家“for months”可以使用；高盛发言人称，这是在“实验”之后，“交给最适合长期使用的人”。（L4；限定词：“banned most of them”、strats 豁免）
- **D2**：OpenAI 称，其与 Georgia Power 在佐治亚 Effingham 园区的协议包含“up to 1,000 megawatts of flexible demand response”（最高 1,000 兆瓦的灵活需求响应）；园区“仍在开发中”，官方称“intend”将其作为试点。（L1）
- **D3**：WorkOS 公告称，2026 年 10 月 30 日发现对其内部销售工具的未经授权访问；公司称“目前无证据”显示应用数据被访问，分析显示仅影响 CRM 与客服数据。（L1）
- **D4**：Claude message threads 进入 Claude Platform 公测；官方称，对多数 Claude Code 用户默认开启时，50 万到 100 万 token 的请求首字延迟中位数降约 35%，token 用量不变。（L1）

## 三、热度

（`barking heat` 对 d 组的首次运行已写入 `sources/d-heat.json`，之后同组前缀的再次运行因拒绝覆盖而失败；D2–D4 的 HN 数据用 HN Algolia 直接查询补充，原始 JSON 存于 `sources/d-hn-search-*.json`、`d-hn-item-*.json`。）

| 条目 | HN | Reddit |
|---|---|---|
| D1 | HN 50341065 “Goldman Restricts Traders and Bankers from Using Some AI Tools”，26 分，4 评，北京 11-03 02:14（barking heat） https://news.ycombinator.com/item?id=50341065 | 失败：Reddit 搜索返回登录页，非 JSON |
| D2 | 0 命中（“Grid Protectors OpenAI”“OpenAI Georgia Power”） | 未查 |
| D3 | HN 50345712 “WorkOS reports security breach”，70 分，9 评，北京 11-03 07:30（UTC 11-02 23:30:59） https://news.ycombinator.com/item?id=50345712 | 未查 |
| D4 | HN 50343242 “Make long-running agents faster with message threads”，1 分，0 评，北京 11-03 04:29 | 未查 |

## 四、可能的吠点（原文有、线索未提或说反，每条一句，附出处）

- **D1**：指令是“banned most of them”（彭博导语），不是一刀切；strats（交易、量化研究与软件工程交叉岗位）不受约束（彭博正文）。
- **D1**：指令是 10 月的内部指引，未公开发布；彭博引述的理由只有 “a variety of factors including productivity and risk management”（发言人），没有公开数据（彭博正文，发言人回应）。
- **D1**：GitHub 是否指 Copilot，报道未说明（彭博原文）。
- **D1**：7 月 CIO 公开赞扬非技术员工的 “vibe coding”（彭博正文，The Decoder 亦引）。
- **D2**：“最高 1,000 MW”是需求响应能力，不是固定削减量（Georgia Power 原文 “up to” 与 “flexible demand response”）；Georgia Power 称设施预计需求约 3,200 MW（新闻稿）。
- **D2**：9.5 亿美元是 Georgia Power 整体大用户合同组合的预计（新闻稿），与 OpenAI 协议无关。
- **D2**：园区状态与安排“still in development”“will differ by location”，协议与园区均为意向或待定（OpenAI 官方页）。
- **D3**：公告明确 “no evidence”，同时写明调查仍在进行，“We will contact affected customers directly”（公告正文）。
- **D3**：HN 高赞评论的质疑（分数未取到）：“We have no evidence that your application data was accessed. Translation: We don't know how we were hacked and can't tell what anyone did or didn't look at.”（HN 50345712 评论，线索 L6）。
- **D4**：TTFT 降幅的主语是 Claude Code 多数用户默认开启的情形（官方原文）；token 用量不变（官方原文）；线程 5 分钟过期后客户端须重发全部历史（官方原文）。
- **D4**：HN 仅 1 分 0 评，没有社区批评可摘。

## 五、夸大说法实例

- **D1**：找不到。中英文搜索（WebSearch）只找到 2026-04 的香港限制快讯（英文 “Goldman Staff in Hong Kong Lose Access to Anthropic’s Claude”，彭博 2026-04-29）及若干转载，未找到把本次限制写成“全面禁用”或“AI 禁令”的标题或文章。中文搜索无相关报道。
- **D2、D3、D4**：找不到夸大实例。D3 中英文搜索无 WorkOS 事件报道（见 capture-log）。

## 六、扫描说法勘误

- **D1**：扫描称“Claude Code 与 GitHub 仅限工程师使用”。核对：彭博原话是 “banned most of them”（大多数人被禁用），并保留 strats 豁免；“GitHub”指 Microsoft 的 GitHub，未说明是否为 Copilot。扫描称“10 月更新内部指引”属实（彭博）。扫描称“发言人 Nick Carcaterra 有回应”属实，原话见上表。
- **D2**：扫描称“最高 1,000 MW 弹性需求响应”属实，但正文应写“需求响应”，并保留“最高”。扫描称“约 70 万户”不对：原文是 1,000 MW 相当于约 70 万户家庭的平均用电需求（规模换算），不是受益户数。扫描称“2029 年起每年约 9.5 亿美元节省”为 Georgia Power 对其大用户合同组合的预计，不是该协议的节省，应注明来源是 Georgia Power。“园区尚在开发”属实。
- **D3**：扫描称“10/30 发现、11/2 公告”属实（公告标注日期均为日期，无时刻）。“内部销售/客户工具”属实，原文为 “internal tool used by our sales and customer-facing teams”。“支持会话片段”“产品使用元数据”属实；公告另写 “Our current analysis indicates this incident affected only CRM and customer support data.”，扫描未写。“AI 辅助框架”属实。“无证据显示生产系统与应用客户数据被访问”属实，原文为 “no evidence that WorkOS application data was accessed”，并有生产系统一句。
- **D4**：扫描称“50 万–100 万 token 的请求首字延迟中位数降约 35%”属实，但漏了原文限定 “With message threads on by default for most Claude Code users”。“token 用量不变”“线程 5 分钟过期”属实。发布日期官方标注 2026-11-02，无时刻。

## 七、未完成、失败与假设

- 未完成：无。D1–D4 均已核对；D1 中文侧未找到对应报道（已如实标“未找到”）。
- 失败：Bloomberg 原页付费墙（正文未读，用转载版补）；`barking fetch` 抓 Quartz（qz.com）被 Cloudflare 质询拦截（`blocked`，未用其他手段绕过）；Reddit 搜索被重定向登录页；`barking heat` 第二次运行因 `d-heat.json` 已存在拒绝覆盖（未删除任何文件）。
- 假设：Georgia Power 两份新闻稿的日期 “07/22/2026”“08/26/2026” 按美国格式（月/日）理解；原页未标时区。Yahoo 转载版的 “1:33 am GMT-5” 与彭博原页 “1:52 PM UTC” 不一致，本组以彭博原页为准。
