# 1013 事实核验

取证：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high），详见 `evidence-a.md`–`evidence-d.md`。Claude 抽查了决定性条目的一手原文：Character.ai 新闻稿（Business Wire）与 CEO 声明 Doc 18、Doc 10 原件；司法部新闻稿与控告书（OCR 全文）；谷歌公告、ADK 博文、adk-python commit 1754b3e 与 v2.12.0 发版说明。日期按北京时间。

## 主帖一：AI 陪聊的破产账本

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Character.ai 10月13日宣布申请破产保护 | L1 | ✅ | https://www.businesswire.com/news/home/20261012656385/en/Character.ai-Initiates-Voluntary-Chapter-11-Proceeding-to-Facilitate-Value-Maximizing-Sale-Process ；https://blog.character.ai/an-update-on-character-ai/ | 新闻稿 “Oct 12, 2026 11:50 PM EDT” = 北京 10/13 11:50；博客页面标 Oct 12（美东）；Doc 1 Filed 10/12/26（D. Del. 26-11603）。正文按北京时间写宣布日。 |
| 迪士尼出价2000万美元，换取部分知识产权的非独占永久许可，可被更高报价取代 | L1 | ✅ | Business Wire 新闻稿 | “Disney will serve as stalking horse bidder for a non-exclusive perpetual license of certain Company intellectual property and certain Company claims, in exchange for $20 million and Disney’s provision of transition services …”；“subject to higher or better offers under section 363”。另含“certain Company claims”与过渡服务，正文从简。许可范围细目与排除项（品牌、App、用户数据等）只见 PPC Land 转述，正文未写。 |
| 多数员工已转投迪士尼 | L1 | ✅ | Doc 10（工资动议）¶10，Stretto 公开 PDF（`a-doc10-wages.txt`）；Doc 18 ¶62 | Doc 18：“Disney agreed to hire the majority of Character’s workforce, including the Debtor’s chief executive officer”；Doc 10：“As of September 30, the Debtor employed a total of approximately 86 full-time … 66 of those employees were offered, and accepted, employment with Disney”。 |
| CEO 提交法院的声明称：2025年月均算力约1000万美元 | L1（法院文书） | ✅ | Doc 18 ¶26（Adams declaration，DocumentCloud，`a-doc18.txt`） | “In fiscal year 2025, operating expenses (excluding legal fees) averaged approximately $18.5 million per month, including monthly compute costs of approximately $10.0 million.” 为年内月均。声明人 Colin Adams 为申请日在任 CEO。 |
| 今年8月收入约530万美元 | L1 | ✅ | Doc 18 ¶11 | “nearly doubling from approximately $2.8 million for the month of December 2025 to approximately $5.3 million for the month of August 2026”。 |
| 每月仍亏800万–900万美元 | L1 | ✅ | Doc 18 ¶11 | “As of August 2026, however, Character was running at an operating loss of approximately $8-9 million per month.” |
| 404 Media 以“AI 经济账算不过来”为题，称月收入“峰值”530万 | L4 | ⚠️（媒体原句已核；属引述其说法，非以其为事实依据） | https://www.404media.co/character-ais-bankruptcy-documents-show-the-economics-of-ai-dont-work/ | 标题 “Character.ai's Bankruptcy Documents Show the Economics of AI Don't Work”；“Its revenue from all sources, including ads, has peaked at $5.3 million per month.” 北京 10/14 04:06。 |
| 声明里这是8月当月数，比去年12月近翻倍 | L1 | ✅ | Doc 18 ¶11 | 见上。 |
| 月净现金消耗从2024年下半年平均2150万降到820万 | L1 | ✅ | Doc 18 ¶27 | “net cash burn per month had declined to approximately $8.2 million by August 2026, compared with monthly averages of approximately $21.5 million in the second half of 2024 and approximately $17.6 million in 2025.” |
| 声明把融资失败归于“潜在责任和当前监管环境” | L1 | ✅ | Doc 18 ¶11 | “attempts to raise capital or monetize the business have been unsuccessful due to Character’s potential liabilities and the current regulatory environment.” |
| 公司无融资债务，申请日现金及投资约5700万美元 | L1 | ✅ | Doc 18 ¶18 | “Character has no funded debt and approximately $57 million in unrestricted cash and cash equivalents and investments as of the Petition Date.” |

## 主帖二：OpenAI 会议室窃听器

当事人姓名、国籍不入正文与本表；含姓名的原件只存本地。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据美国司法部，一名 OpenAI 工程师涉嫌把裹着卫生纸的录音设备插进高管会议室的电视 | L1 | ✅ | 司法部 N.D. Cal. 新闻稿（`b-doj-pr.txt`）；控告书 3:26-mj-71329（`b-complaint-3-26-mj-71329-ocr.txt`） | “a surreptitious audio recording device … inserted into a USB port on the back of a television in an executive conference room”；“concealed in toilet paper”。新闻稿只给日期 10/13（美国），正文不写日期。 |
| 已被捕，被控使用或试图使用设备截听口头通讯 | L1 | ✅ | 同上 | “arrested for and charged with using or endeavoring to use an electronic, mechanical, or other device to intercept oral communications”；18 U.S.C. § 2511(1)(b)；10/9（美国）被捕。 |
| OpenAI 称用自家模型弄清了设备来源、锁定安装者 | L1（官方账号） | ✅ | OpenAI Newsroom 帖（fxtwitter JSON `b-*`；The Verge 全文引用） | “We used our models to understand the device and where it had come from, identifying that a specific employee had planted it a few days earlier.” 北京 10/14 03:48。 |
| 刑事控告书列的依据是：监控拍到他10天在会议室附近 | L1 | ✅ | 控告书 ¶14 表 | 9/8–9/29 共 10 个日期；安装当天会议室内无摄像头。 |
| 他用效率类智能体查会议室和日程 | L1 | ✅ | 控告书 ¶15 | “used a productivity agent to inquire about Company 1’s building addresses, meeting rooms, meeting-room capacities and calendars”。 |
| 与他的账号关联的 AI 聊天账号，提示词含这台设备的网卡地址，还问过怎么做录音器、给存储卡加密 | L1 | ✅ | 控告书 ¶17–18 | “an AI chatbot account associated with [当事人]’s spouse … submitted user prompts containing the aforementioned custom “NANOVOL1” header and the unique MAC address of Recording Device 1”；提示词含 “To build a continuous audio recorder”“To encrypt the SD card”；¶18 归因写 “attributable to [当事人] and/or his spouse”。该账号登记在当事人配偶名下，与当事人自己的账号共用安装标识、IP 与路径名（¶17）；配偶未被指控，正文不点出配偶，写“与他的账号关联”。 |
| 控告书没提 OpenAI 用模型，也没写是哪家产品 | L1 | ✅（检索） | 控告书全文 | B 组与 Claude 检索 OCR 全文：“model” 只见 ¶8 对 Company 1 的介绍（“develops large-scale generative models”）；chatbot、productivity agent 均未点名产品。OCR 有错字，结论依 B 组逐页目视与 Claude grep。 |
| 控告书写内容“可能已加密”，录到什么未交代 | L1 | ✅ | 控告书 ¶13 | “the contents of the microSD card may have been encrypted … none of the files on it could be recovered or decrypted”。 |
| 司法部新闻稿写成“已加密” | L1 | ✅ | 司法部新闻稿 | “OpenAI determined that the device … was installed on or about September 28, 2026, and that its contents were encrypted.” |
| 目前只是被控，未定罪 | L1 | ✅ | 司法部新闻稿；CourtListener 案卷 | “A complaint merely alleges that crimes have been committed”；案卷仅 complaint，羁押听证 10/19（太平洋时间）。 |

## 主帖三：Argon 正式开放

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌宣布 Argon 正式开放：付费 API 可用，Pro、Ultra 订阅可在 Gemini 应用里用 | L1 | ✅ | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/ | “globally available today for developers and enterprises via the API in AI Studio and Gemini platform, for Google AI Ultra subscribers in Google Antigravity, and Google AI Ultra and Pro subscribers in the Gemini app and AI Mode in Search”；“Build with the Gemini API paid tier”。页面元数据北京 10/14 00:00。 |
| （未入正文）免费 API 不可用 | L1 | ✅ | https://ai.google.dev/gemini-api/docs/pricing | Argon 行免费层 “Not available”（C 组截图 36）。 |
| 同时发文称让它通宵11小时优化自家约20万行的开源项目 ADK | L1 | ✅ | https://developers.googleblog.com/en/optimizing-a-200000-line-python-codebase-overnight-with-gemini-4-argon/ | “roughly 205,000 lines of code across nearly 800 source files”；“the 11-hour overnight run”。ADK 为谷歌开源框架（google/adk-python）。 |
| 6项优化只合并2项，4项在审查中被拒 | L1 | ✅ | 同上 | “Argon's pull request proposed six optimizations across 32 files. Two of those optimizations … shipped to production”；“During code review, we rejected the other four changes”。该 PR 未公开，实际合入为 commit 1754b3e（22 个文件）。 |
| 一项改动通过全部开源测试，仍让25个谷歌内部测试目标失败，改过才合入 | L1 | ✅ | 同上 | “all 16,892 open-source tests passed, yet 25 internal Google test targets failed … The shipped fix uses lazy module accessors”。 |
| “提速约40%”只算内存会话调用模型前的框架开销，用秒回的假模型、关了垃圾回收测 | L1 | ✅ | commit 1754b3e 信息；v2.12.0 发版说明 | 发版说明 “Cut time to first model request by 40% in long sessions”；commit 表 “no-op model and tools that return instantly, measuring only ADK overhead; * = garbage collection disabled”，50 轮 64→36 ms（−43%*）、150 轮 200→122 ms（−39%*）；ADK 博文：“The 40% reduction in pre-model overhead applies to InMemorySessionService … External database, SQLite … session stores … their read times don't change here.” |
| 开着垃圾回收跑完158轮模拟只快14% | L1 | ✅ | 同上；ADK 博文 | commit 表 “whole 158-turn run 65.6 s 56.1 s -14%”（该行无 * 号，即未关垃圾回收；仍为假模型与秒回工具）。 |
| （未入正文）新进程首次请求耗时没变 | L1 | ✅ | ADK 博文 | “time from a brand-new process to its first model request stays flat (3.07 s to 3.09 s)”。 |

## 省流卡

| 卡片文字 | 对应正文 | 结果 |
|---|---|---|
| Character.ai 破产，迪士尼出价2000万美元，要的是部分知识产权的非独占许可，不是公司 | 省流第 1 句 | ✅（逐字；“出价”即 stalking horse 竞价，可被更高报价取代） |
| OpenAI 称用自家模型查出窃听设备安装者，刑事控告书没提这点 | 省流第 2 句 | ✅（逐字） |
| Gemini-4-Argon 向付费用户正式开放，谷歌自家通宵优化实验6项改动只合并2项 | 省流第 3 句 | ✅（逐字） |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 苹果 HomeView 美国预售299美元，部分云端 AI 功能每日限量 | L1 | ✅ | https://www.apple.com/newsroom/2026/10/meet-apple-homeview-a-new-way-to-enjoy-entertainment-and-manage-the-home/ | “for $299 (U.S.)”；脚注 “Certain Apple Intelligence features that rely on server-side models are subject to daily usage limits. Expanded access to such features will be available for a fee in the future.” “云端”对应 server-side models；“预售”对应 pre-order，11/13 起供货；价格为美国价。扩容使用的费用与额度未公布。HN 724/849（产品页帖）。 |
| Anthropic 称9月日合并代码超3万次，单体仓库已从 Git 迁到自研 | L1 | ✅ | https://www.anthropic.com/engineering/redesigning-source-control | “In May 2026, Anthropic averaged around 2,500 code changes merged per day; as of September, we merged over 30,000 per day.”；“fully migrated our monorepo from git in five months”；3万次为9月数据，迁移对象为单体仓库（monorepo）。公司自报。HN 758/751。 |
| 微软修复 Visual Studio 漏洞，研究者称文件名末尾加点可绕过配置保护 | L1 / 研究者一手 | ✅ | MSRC CVE-2026-103572（CVRF，`d-d3-*`）；https://embracethered.com/blog/posts/2026/visual-studio-copilot-remote-code-execution/ | MSRC：Important、CVSS 8.8、Exploited No，北京 10/13 22:00；研究者：“in Windows you can have . at the end of a filename … A trailing dot allowed the AI agent to bypass protections around sensitive configuration files”。赏金 $0 只见研究者博客，未写入。 |
| XBOW 称借 AI 造出 Windows Chrome 攻击链获赏金25万美元 | L1（厂商自述） | ✅ | https://xbow.com/blog/xbow-built-a-chrome-full-chain-exploit | “it won our company a $250,000 bounty from Google”；“We did it in six days.”；“showing how an attacker could use Chrome to remotely take over a Windows machine”；“This project was the product of a partnership between humans and machines.” 谷歌一侧未找到记录，故写“XBOW 称”；“六天”因字数删去（原文 “We did it in six days.”）。 |

## 批注译注

| 图 | 原句 | 译注 | 结果 |
|---|---|---|---|
| 41 | “nearly doubling from approximately $2.8 million for the month of December 2025 to approximately $5.3 million for the month of August 2026” | 从2025年12月约280万美元，近乎翻倍到2026年8月约530万美元 | ✅ Doc 18 ¶11 |
| 41 | “As of August 2026, however, Character was running at an operating loss of approximately $8-9 million per month” | 截至2026年8月，公司每月经营亏损约800万–900万美元 | ✅ Doc 18 ¶11 |
| 41 | “attempts to raise capital or monetize the business have been unsuccessful due to Character’s potential liabilities and the current regulatory environment” | 融资或变现的尝试，因公司的潜在责任和当前监管环境而未能成功 | ✅ Doc 18 ¶11（高亮含上一行行尾“attempts to raise”） |
| 42 | “2025, operating expenses (excluding legal fees) averaged approximately $18.5 million per month, including monthly compute costs of approximately $10.0 million” | 2025财年，运营支出（不含法律费）月均约1850万美元，其中算力月均约1000万美元 | ✅ Doc 18 ¶26 |
| 42 | “net cash burn per month had declined to approximately $8.2 million by August 2026, compared with monthly averages of approximately $21.5 million in the second half of 2024” | 到2026年8月，每月净现金消耗已降到约820万美元；2024年下半年月均约2150万美元 | ✅ Doc 18 ¶27 |
| 43 | “submitted user prompts containing the aforementioned custom “NANOVOL1” header and the unique MAC address of Recording Device 1” | 提交的提示词里含有前述自定义“NANOVOL1”头和录音设备1独有的 MAC 地址 | ✅ 控告书 ¶17 |
| 43 | “To build a continuous audio recorder” | 做一个持续录音的录音器 | ✅ 控告书 ¶17 |
| 43 | “To encrypt the SD card” | 给 SD 卡加密 | ✅ 控告书 ¶17 |
| 44 | “the contents of the microSD card may have been encrypted” | microSD 卡里的内容可能已被加密 | ✅ 控告书 ¶13 |
| 45 | “all 16,892 open-source tests passed, yet 25 internal Google test targets failed” | 16892个开源测试全部通过，但有25个谷歌内部测试目标失败 | ✅ ADK 博文 |
| 46 | “In-memory session, garbage collection off”（两行） | 内存会话，关闭垃圾回收 | ✅ 图 5 |
| 46 | “3 rounds of 4 tool calls per turn, garbage collection on” | 每轮3次、每次4个工具调用，开启垃圾回收 | ✅ 图 5 |
| 46 | “Median of 6 to 12 runs against an instant model and instant tools.” | 用即时返回的模拟模型和工具，运行6到12次取中位数 | ✅ 图 5 脚注；commit 说明 “no-op model and tools that return instantly, measuring only ADK overhead” |

## 反向核验

第一轮（codex-reviewer 等价命令，WSL Codex gpt-6-astra medium，只审正文、事实清单与卡片文字，北京 10/14 约 21:40）：1 BLOCKER、8 SHOULD_FIX、2 NEEDS_VERIFICATION。Claude 逐条对照存档，全部采纳：

1. BLOCKER：省流“迪士尼2000万美元买的是”把待竞价交易写成已成交 → 改“迪士尼出价2000万美元，要的是”（正文事实段保留“可被更高报价取代”）。
2. 5700万美元原文是“现金、现金等价物和投资”、截至申请日 → 改“申请日现金及投资约5700万美元”，补全原句。
3. ADK 40% 限于内存会话、关垃圾回收；14% 为开垃圾回收的158轮模拟 → 吠点②改写。
4. criminal complaint 不是“起诉书” → 统一“刑事控告书 / 控告书”，结尾“目前只是被控”。
5. Argon 套餐与入口对应 → “付费 API 可用，Pro、Ultra 订阅可在 Gemini 应用里用”；省流加“向付费用户”。
6. HomeView → “美国预售299美元”。
7. Scion → “9月”“单体仓库”。
8. XBOW → “Windows Chrome”。
9. 404 Media 行改 ⚠️；OpenAI Newsroom 统一 L1。
10. 配图与来源入口当时未完成 → 已补，留第二轮审。
11. “据美国司法部”涉美国政府机构 → 提交 Develata 决定（正文不写国籍、不写任何国家背景）。

第二轮（同命令，北京 10/14 约 21:15）：PASS、无 BLOCKER；7 SHOULD_FIX 全部采纳——42 图译注补“不含法律费的运营支出”；41 图补“截至2026年8月”并扩大高亮；46 图改“即时返回的模拟模型和工具”；速览 XBOW 补“称”“赏金”（删“六天”以守40字）；备用图表删去误列的 00-* 四行；来源入口如实写明脱敏范围（文本存档只替换姓名）；evidence-b 中 OpenAI Newsroom 改为 L1。NEEDS_VERIFICATION“据美国司法部”的风控取舍待 Develata 终审。

## 视觉复核

WSL Codex gpt-6-astra medium（`barking review --visual`，北京 10/14 约 21:20；看的是第二轮修订前的成图）：1 BLOCKER、5 SHOULD_FIX、1 NICE_TO_HAVE。

1. BLOCKER 速览 XBOW 缺“称” → 第二轮已改为“XBOW 称借 AI 造出 Windows Chrome 攻击链获赏金25万美元”。
2. SHOULD_FIX 省流卡第 2 条“刑事控告书没提这点”属吠点 → 不采纳：这是对控告书内容的可核事实（检索全文未见 OpenAI 用模型的表述），与 1012 期省流卡“官方称这证实‘可靠’”同类；删掉后卡片只剩 OpenAI 的自述，反而失衡。
3. SHOULD_FIX 41 图高亮③漏“attempts to raise” → 补 rect。
4. SHOULD_FIX 42 图①未说明指标、未高亮年份 → quote 扩到“2025, operating expenses (excluding legal fees) …”，译注改“2025财年，运营支出（不含法律费）月均约1850万美元，其中算力月均约1000万美元”。
5. SHOULD_FIX 46 图③高亮扫进“Source:” → 右边界收到“tools.”。
6. SHOULD_FIX 45、46 英文偏小 → 45 改用 700 CSS px、2 倍像素比重渲的底图（opencli 后台窗口截图超时，退回 headless）；46 为官方图表，700 宽下更小，保留 900 CSS px 版本。
7. NICE_TO_HAVE 43 图②③色带偏窄（等宽字体 OCR 行高）→ 不改，不影响定位。

改后全部重渲，Claude 逐张目视：41、42、45、46 高亮落点与译注对应，45 底图上下边界落在行间。

正文措辞（终审前）：吠点①原写“与其配偶关联的 AI 聊天账号”，为不牵出未被指控的第三方，改为“与他的账号关联的 AI 聊天账号”（¶17 依据同上）；43 图标题同步。
