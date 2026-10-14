# 1013 期 B 组取证：OpenAI 会议室隐藏录音设备案

执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）。取证时段：2026-10-14 北京时间约 16:50–19:45（本机美东 = 北京 − 12 小时；UTC = 北京 − 8 小时）。

**当事人隐私处理**：当事人是刑事被告，不是公众人物。本文件与所有入库的 txt/json 存档里，姓名一律写“[当事人]”，国籍写“[国籍]”（存档 txt 只替换姓名，国籍词保留原文）。含姓名的原始 HTML 与 PDF 只存本地（`sources/*.html`、`sources/b-complaint-3-26-mj-71329-*.pdf`，均被 `.gitignore` 排除）。截图里姓名与国籍词用黑条遮盖，不含当事人照片（DOJ 新闻稿里的当事人 CCTV 截图未截）。不对动机或任何国家参与作推断；文中凡写动机、国家的句子都是官方或媒体原话，只作引文。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别按 EDITORIAL.md：L1 官方/司法原件与当事方官方账号声明（2026-10-14 按 EDITORIAL.md 由 L2 更正为 L1），L2 署名员工个人发言，L4 媒体转述，L6 社区线索。

## 0. 先给结论

1. **一手来源齐全**：司法部新闻稿两份（NDCA 与 OPA 版，内容不同）、刑事控告书（complaint，N.D. Cal. 3:26-mj-71329，**11 页 PDF 拿到**，含 FBI 特工宣誓书）、法院案卷条目（RECAP）、OpenAI Newsroom 的 X 帖（fxtwitter JSON）。
2. **案件只到 complaint（控告书）阶段**：案卷未见 indictment；Doc 5 为 10/13 首次到庭；羁押听证 10/19 11:00（太平洋时间）。已查案卷里没有认罪/不认罪记录（控告阶段尚无 arraignment）。
3. **最关键的“AI 吠点”**：控告书和 DOJ 新闻稿**都没有写 OpenAI“用自家模型”锁定当事人**。控告书里列出的识别与归因依据是：(a) CCTV 显示当事人在会议室附近的 10 个日期（表）；(b) 一个“productivity agent”上的查询记录（8/17–9/16）；(c) 一个“与当事人配偶关联”的 AI chatbot 账号 9/24–9/30 的提示词，其中 9/27–9/28 的提示词含设备 microSD 里恢复出的自定义头部 “NANOVOL1” 与唯一 MAC 地址；(d) 10/5 的新装摄像头画面与无人看管背包里的钱包、证件与第二台设备。控告书**没有点名 chatbot 与 productivity agent 是哪家产品**。“用模型分析设备”这句只出现在 OpenAI 自己的声明里：X 帖只说 “We used our models to understand the device and where it had come from, identifying that a specific employee had planted it a few days earlier”；更具体的说法（“使用模型分析设备存储的取证镜像并评估底层数据变化”）仅见于 OpenAI 对 SF Standard 的转述（L4，未见 OpenAI 自己的原文页）。
4. **扫描说法勘误**：见第 6 节（The Verge 的“arrested Friday”= 2026-10-09；设备“加密”在控告书里是“may have been encrypted”；AP 称“half-dozen occasions”与控告书表格的 10 个日期不符；AIHOT 该条排第 2 不是第 3；Reuters 的发布时刻等）。

## 1. 逐条取证表

说法列沿用任务书编号。引文为原语言逐字，姓名/国籍已替换。页码指 PDF 页（“Page N of 11”）。

### B1 司法部新闻稿与控告书

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图/存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1.1 DOJ（NDCA）新闻稿存在，标题与罪名 | L1 | https://www.justice.gov/usao-ndca/pr/saudi-national-arrested-installing-surreptitious-audio-recording-device-openai-san | 标题 “[国籍] Arrested For Installing Surreptitious Audio Recording Device At OpenAI In San Francisco”；“has been arrested for and charged with using or endeavoring to use an electronic, mechanical, or other device to intercept oral communications.” | `images/13-doj-press-release-charge.png`；存档 `b-doj-pr.txt`（姓名已替换；`b-doj-pr.html` 本地） | 页面日期 “Tuesday, October 13, 2026”，“Updated October 13, 2026”；**官方未给时刻**（页面 `<time>` 只有日期） | 已找到 |
| B1.2 DOJ（OPA）新闻稿：另一版本，内容不同 | L1 | https://www.justice.gov/opa/pr/saudi-national-arrested-installing-surreptitious-audio-recording-device-openai | 比 NDCA 版多 DNI 一段：“This serious situation underscores why winning the global super intelligence race is a national security prerogative — and why President Trump’s leadership in creating the Super Intelligence Force is so important”（Director of National Intelligence Jay Clayton）；结尾 “This prosecution is in support of National Security Presidential Memorandum 11 (NSPM-11) designed to support American superiority in super intelligence technology.”；检察官为 “Assistant U.S. Attorney Nicolas Parker … with valuable assistance from Trial Attorney Kate Tagert of the National Security Division’s Counterintelligence and Export Control Section” | 存档 `b-doj-opa-pr.txt` | 同样只有日期 10/13；OPA 版写 “arrested on Oct. 9”，NDCA 版写 “Friday, October 9, 2026” | 已找到 |
| B1.3 刑事控告书（complaint）与宣誓书原件 | L1 | CourtListener RECAP：https://www.courtlistener.com/docket/74958115/ ；PDF `https://storage.courtlistener.com/recap/gov.uscourts.cand.480828/gov.uscourts.cand.480828.1.0.pdf`（11 页，主文 + 宣誓书）、`…1.1.pdf`（Penalty Sheet） | 页 1：“Case No. 3:26-mj-71329 MAG”；“18 U.S.C. § 2511(1)(b) / Use of an electronic, mechanical, or other device to intercept oral communications”；“On or about the date(s) of September 28, 2026 in the county of San Francisco”；“See attached affidavit of FBI SA Lydia Liang”；“Date: 10/08/2026”；法官 “Hon. Alex G. Tse, U.S. Magistrate Judge”；戳 “FILED Oct 08 2026” | PDF 本地 `b-complaint-3-26-mj-71329-doc1.pdf`（含姓名，不入库）；OCR 全文（姓名已替换）`b-complaint-3-26-mj-71329-ocr.txt`；案卷 `b-courtlistener-docket-74958115-documents.json` | 宣誓书页为扫描图像，OCR 有错字，两张表已人工转录并标注；引用以 PDF 目视为准。控告书 10/8 提交并封存（Doc 2 sealing motion，Doc 3 sealing order，10/8），10/13 解封（Doc 7 Order to Unseal）；RECAP 文档由第三方上传，非 PACER 原下载 | 已找到 |

### B2 设备细节与“如何锁定当事人”

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 安装位置 | L1 | 控告书 ¶11；DOJ 新闻稿 | “inserted into a USB port on the back of a television in Conference Room 1” | 14、22 | | 已找到 |
| 设备型号与硬件 | L1 | 控告书 ¶12 | “The device was a Seeed XIAO ESP32-S3 Sense—a thumb-sized development board with a detachable camera sensor, digital microphone, and microSD storage—and had been concealed in toilet paper” | 14（Figure 1、2 设备照；脚注 1 写 Seeed 公司所在地与制造地，**未截入图**，原文在控告书 PDF 第 5 页脚注，未作任何推断） | 控告书称是“development board”；Wired 称其零售价 $13.99（L4，Wired 引 Seeed 官网，未核） | 已找到 |
| 包裹物 | L1 | 控告书 ¶12；DOJ | “concealed in toilet paper” | 14 | | 已找到 |
| 安装日期 | L1 | 控告书 ¶13；DOJ | 控告书：“the firmware for Recording Device 1 appeared to have been compiled on September 28, 2026, suggesting it was installed on or about that date.” DOJ：“OpenAI determined that the device … was installed on or about September 28, 2026” | 16、22 | “on or about”；依据是固件编译日期，**不是**目击；当时会议室没有摄像头（¶14：“Company 1 did not have a camera within Conference Room 1 at the time Recording Device 1 was installed”） | 部分支持（日期是推断值） |
| 发现日期 | L1 | 控告书 ¶11 | “On or about September 30, 2026, Company 1 personnel discovered a surreptitious audio recording device” | 14、22 | OpenAI 对 Fortune 称由 technical surveillance countermeasures 团队在 sweep 中发现（L4，Fortune/SF Standard/Wired 均有） | 已找到 |
| 加密 | L1 | 控告书 ¶13 vs DOJ 新闻稿 | 控告书：“the USB adapter on Recording Device 1 **may have been** modified to prevent data transfer…; and (ii) the contents of the microSD card **may have been** encrypted. Specifically, when Company 1 personnel made a forensic image of the microSD card, none of the files on it could be recovered or decrypted, but the limited information that could be recovered included a MAC address and a custom header (“NANOVOL1”) … a layout consistent with a potentially encrypted file system.” DOJ 新闻稿：“OpenAI determined that the device … was installed on or about September 28, 2026, and that its contents were encrypted.” | 16（控告书）、22（DOJ） | **控告书口径是“可能”，DOJ 新闻稿与多家媒体写成肯定句**；内容是否被录下、录了什么，均未知（SD 内文件无法恢复） | 部分支持 |
| “另一个录音设备” | L1 | 控告书 ¶20；DOJ | “[当事人] left a bag unattended in a printer room in Company Building 1. Company 1 security personnel opened the bag and discovered—among other things, including [当事人]’s wallet, ID, and other personal items—another recording device (‘Recording Device 2’). Like Recording Device 1, Recording Device 2 … is manufactured by Seeed.”（姓名已替换） | 20（含 Figure 4 设备 2 照片） | 10/5 发现；¶21：10/6 约 12:00 当事人从 Lost and Found 取回包，“which no longer contained Recording Device 2”，书面报告丢了现金、信用卡等，“did not inquire about Recording Device 2” | 已找到 |
| 监控录像内容 | L1 | 控告书 ¶14、¶19 | ¶14：CCTV 显示当事人“was near Conference Room 1 on the following dates”，表列 **10 个日期**：9/8 4:35–7:01 pm；9/9 12:11–12:12 pm；9/14 12:08–12:09 pm；9/15 12:30–12:31 pm；9/16 12:36 pm–6:51 pm；9/21 11:31 am–6:12 pm；9/22 12:14–7:22 pm；9/23 12:35–7:27 pm；9/28 2:18–6:30 pm；9/29 3:35–3:37 pm。¶19：“On or about October 5, 2026, by which point a camera had been placed in Conference Room 1, [当事人] was observed entering Company Building 1. CCTV footage from that date shows [当事人] entering Conference Room 1 at about 1:20 pm and checking behind the television into which Recording Device 1 had been inserted.” | 17（表）、22（DOJ 转述）；当事人 CCTV 截图（控告书 Figure 3、DOJ 页的人像）**未截** | “near”而非“in”；9/28 当天 2:18–6:30 pm 在场；10/5 摄像头是发现之后加装；此时设备 1 已被 OpenAI 取走（9/30） | 已找到 |
| OpenAI 如何锁定当事人：起诉文件里的依据 | L1 | 控告书 ¶14–¶20 | ¶15：“between on or about August 17, 2026, and on or about September 16, 2026, [当事人] used a productivity agent to inquire about Company 1’s building addresses, meeting rooms, meeting-room capacities and calendars, and external-partner meeting venues.” ¶17：“Between on or about September 27, 2026, and September 28, 2026, an AI chatbot account associated with [当事人]’s spouse—an account that shared installation identifiers, logged IP usage, and user path name references with [当事人]’s own AI chatbot account—submitted user prompts containing the aforementioned custom ‘NANOVOL1’ header and the unique MAC address of Recording Device 1.” ¶18：“these written prompts attributable to [当事人] **and/or his spouse**, indicate that [当事人] inserted Recording Device 1 into the television” | 18、19、22 | 九条提示词类别见截图 19（如 “To build a continuous audio recorder”“To encrypt the SD card”）。**控告书没有写 productivity agent 与 chatbot 分别是哪家产品**；¶8 只写 Company 1 “provides for a chatbot and virtual assistant”。Fortune 猜 productivity tool “presumably internal to OpenAI”、Wired 称“an internal OpenAI tool”（均 L4，未见出处）。账号归因用的是 “installation identifiers / logged IP usage / user path name references” | 已找到 |
| 文件里是否提到 AI 模型分析设备 | L1 | 控告书全文 OCR；两份 DOJ 新闻稿 | 控告书全文与 DOJ 新闻稿中，**关于 OpenAI 自己用 AI 模型分析设备/追溯来源的叙述：未见**。涉 AI 的只有：productivity agent 查询（¶15）、chatbot 账号提示词（¶17–18）、“Company 1 develops large-scale generative models”（¶8） | — | 控告书写 “Company 1 personnel made a forensic image of the microSD card”（人员），没有写用了模型 | 已找到（结论是“未见”） |
| OpenAI 对 AI 作用的更具体说法 | L4 | SF Standard https://sfstandard.com/2026/10/13/openai-engineer-arrested-alleged-bugging-executive-conference-room/ | “The company said investigators used its models to analyze forensic images of the device’s storage and assess changes in the underlying data.” “According to OpenAI, its technical surveillance countermeasures team found the device during a sweep of a conference room.” | 23（遮盖姓名） | 转述 OpenAI 的说法，原文页未找到；Wired 另写：OpenAI “was able to connect it to [当事人] after reviewing chat logs and surveillance footage, and searching a bag” | 部分支持 |

### B3 OpenAI 原始声明

| 说法 | 级 | 一手来源 | 原文摘句 | 截图/存档 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| OpenAI Newsroom 的 X 帖 | L1 | https://x.com/OpenAINewsroom/status/2110095403198849132 （fxtwitter JSON；官方嵌入页渲染） | 全文：“We discovered a sophisticated recording device that was hidden inside one of our offices. We used our models to understand the device and where it had come from, identifying that a specific employee had planted it a few days earlier. We referred the matter to the FBI, and we’re grateful to the agents and prosecutors who moved quickly to make an arrest. The FBI’s investigation is ongoing and we will continue to support their work.” / “At this time, we have no indications of this individual inappropriately accessing or exfiltrating model weights, source code, or customer data. The employee was terminated.” / “We take protecting our technology, data, and people seriously, and as a matter of national importance. Our security program is designed to protect against a wide range of threat actors, from insider risks to cybercriminals to nation states, and this case is an example of how AI can help address these sophisticated security threats.” | `images/15-openai-newsroom-post.png`（官方嵌入卡片，**折叠显示前约 6 行并有 “Show more”，第二段“no indications”未入图**；全文见 JSON）；存档 `b-x-openainewsroom-2110095403198849132.json` | `created_at` “Tue Oct 13 19:48:42 +0000 2026” = **北京时间 10/14 03:48:42**（美东 15:48）；is_note_tweet true。互动：2026-10-14 09:01Z 取数 views 17,355,230 / likes 68,965 / reposts 5,355 / replies 3,455 / quotes 4,977 / bookmarks 16,216；11:41Z 复取 views 19,231,872 / likes 70,987 / reposts 5,523 / replies 3,605 / quotes 5,181 / bookmarks 16,807 | 已找到 |
| “no indications”一句的范围 | L1 | 同上 | “inappropriately accessing or exfiltrating **model weights, source code, or customer data**” | 同上 | 限定：“At this time”；对象是“this individual”；三类资产；**没有排除会议录音内容被外传**（帖子未谈录音内容）。OpenAI 另对媒体称“has not seen activity that compromised the safety of its products or customers’ information”（SF Standard 转述） | 已找到 |
| OpenAI 另行对媒体的补充说法（X 帖里没有） | L4 | AP（NBC Bay Area 刊 https://www.nbcbayarea.com/news/national-international/openai-engineer-arrest-hidden-listening-device/4157611/）；Reuters；SF Standard；Fortune | AP：“OpenAI said it had evidence the employee planned to place other listening devices throughout other parts of the office building for the purpose of recording meetings.” Fortune：“Between Sept. 30 and Oct. 9, OpenAI worked with the FBI on the timing of [当事人]’s termination, so as to not disrupt their investigation… OpenAI said it restricted [当事人]’s access to systems and parts of the office, suspended his network access, placed him on leave, and took his company-issued equipment.” TechCrunch：OpenAI 发言人 Nate Evans 称“advanced encryption, audio monitoring capabilities, and a mechanism for transferring data designed to evade monitoring”，当事人在 product team | `b-m-*` 存档 | 这些是 OpenAI 对媒体的口径，未见 OpenAI 官方页；“planned to place other devices”不在控告书里 | 部分支持 |

### B4 罪名与程序

| 说法 | 级 | 一手来源 | 原文摘句 | 状态 |
|---|---|---|---|---|
| 适用法条 | L1 | 控告书页 1；DOJ | “18 U.S.C. § 2511(1)(b)”；DOJ：“for a violation of 18 U.S.C. § 2511(1)(b).” | 已找到 |
| 最高刑期 | L1 | DOJ NDCA 新闻稿；Penalty Sheet（Doc 1-1） | “If convicted, [当事人] faces a maximum sentence of 5 years in prison and a $250,000 fine, followed by 3 years of supervised release”；Penalty Sheet：“Imprisonment: 5 years (18 U.S.C. § 2511(4)(a)); Fine: $250,000 (18 U.S.C. § 3571(b)(3)); Supervised Release: 3 years (18 U.S.C. § 3583(b)(2)); Special Assessment: $100”；DOJ 另写 “A complaint merely alleges that crimes have been committed, and a defendant is presumed innocent until proven guilty beyond a reasonable doubt.”（截图 21） | 已找到 |
| 逮捕日期 | L1 | DOJ NDCA | “was arrested on Friday, October 9, 2026”（换算：10/13 是周二，故“arrested Friday”= 2026-10-09） | 已找到 |
| 首次出庭 | L1 | DOJ；RECAP Doc 5 | DOJ：“made his initial appearance in federal court today”（10/13）；Doc 5 Minute Entry：“Initial Appearance and ID of Counsel Hearing … held on 10/13/2026. Remanded to Custody. AUSA orally moves to unseal entire case - Granted.” Liberty 录音时间 “11:19 - 11:29”；Set/Reset 条目 “Initial Appearance set for 10/13/2026 11:00 AM in San Francisco, Courtroom A” | 已找到 |
| 下一次庭审 | L1 | DOJ；Doc 5 | DOJ：“next scheduled to appear in federal court on October 19, 2026, for a detention hearing before U.S. Magistrate Judge Alex G. Tse”；Doc 5：“Detention Hearing set for 10/19/2026 at 11:00 AM in San Francisco, Courtroom A, 15th Floor”；Doc 6：Speedy Trial 排除期 10/13–10/19 | 已找到 |
| 是否已起诉：complaint 还是 indictment | L1 | RECAP 案卷（10 条文档/条目，至 2026-10-13） | 只有 Doc 1 COMPLAINT（10/8）、Doc 2/3 封存动议与令、Doc 5 Minute Entry、Doc 6 Stipulated Order、Doc 7 Order to Unseal、Doc 8 Notice of Organizational Victim（OpenAI OpCo, LLC，其母公司 OpenAI Global, LLC 之 ≥10% 公众持股方为 Microsoft Corporation）。**未见 indictment/information**；Guardian 写 “According to the indictment”（错） | 已找到（结论是“只有 complaint”，RECAP 快照约 2026-10-14 08:05Z，之后可能新增） |
| 是否认罪 | L1 | 同上 | 案卷无 plea 条目；控告阶段尚无 arraignment；AP：“It was not immediately clear if he had a lawyer who could speak on his behalf.”；案卷律师栏列有辩护律师 | 未找到一手来源（“认罪”无记录；“未认罪”也无记录） |
| 控告期间 | L1 | 宣誓书 ¶1 | “beginning on a date unknown but no later than on or about September 28, 2026, and lasting through a date unknown but no earlier than October 5, 2026” | 已找到 |

### B5 各家报道（发布时刻与独有细节）

北京时间 = 原时区换算。“L4 转述”均已读原文（Reuters 除外，见失败）。

| 媒体 | 发布时刻 | 独有或需要标注的细节（原句） | URL |
|---|---|---|---|
| Reuters | Kelo 转载版 JSON-LD `datePublished` 2026-10-13T19:59:12Z = 北京 10/14 03:59:12（页面 “Oct 13, 2026 \| 2:59 PM” CDT）；**Reuters 原页打不开（HTTP 401，两种方式）**，以转载版为准 | “The thumb-sized device, which had a microphone and could connect to a camera, was concealed in toilet paper, according to an FBI complaint. It was installed on or about September 28, while its stored data was encrypted and inaccessible.”；“The company said it had evidence that the employee intended to place additional recording devices in sensitive areas of its offices”；“The [国籍] embassy in the United States did not immediately respond to a Reuters request for comment.”（原文此处为国籍词）；另含当事人 LinkedIn 履历（不记录） | https://www.reuters.com/world/us/fbi-arrests-former-openai-employee-planting-listening-device-2026-10-13/ ；转载 https://kelo.com/2026/10/13/fbi-arrests-former-openai-employee-for-planting-listening-device/ ；NBC News 版 4:40 PM EDT = 北京 04:40 https://www.nbcnews.com/tech/tech-news/openai-employee-arrested-hidden-listening-device-company-says-rcna603262 |
| The Verge | 2026-10-13T20:45:25Z = 北京 10/14 04:45:25（含 “Update, October 13th: Added information from the DOJ.”） | “OpenAI’s post about the incident says it used its AI models to ‘understand the device and where it had come from…’”；“The device was covered in toilet paper and installed on or around September 28th. Its contents were encrypted.”（照抄 DOJ 口径） | https://www.theverge.com/ai-artificial-intelligence/1010642/openai-employee-arrested-recording-device |
| AP（Eric Tucker） | NBC Bay Area 版 `datePublished` 2026-10-13T16:38:16-07:00 = 北京 10/14 07:38:16；ABC News 版 “October 13, 2026, 5:10 PM”（时区未标） | “closed-circuit television footage … revealed that [当事人] was near the conference room on a **half-dozen occasions** last month”（**控告书表格为 10 个日期**）；“The company subsequently added a camera to the room”；“The complaint does not detail what [当事人] might have wanted to do with any captured communications.”；引 AG Todd Blanche：“This alleged attempt by a [国籍] to steal sensitive communications from OpenAI was taken extremely seriously. Theft of intellectual property involving cutting-edge super intelligence technology poses a serious threat to our national security” | https://abcnews.com/Technology/wireStory/employee-arrested-after-listening-device-found-openai-conference-137239998 |
| CNBC | 2026-10-13 4:55 PM EDT = 北京 10/14 04:55（Updated 8:13 PM EDT） | 描述当事人在一家 VC 的兼职身份（不记录）；引 DOJ 称 chatbot 账号 “shared multiple identifiers and attributes with … own chatbot account” | https://www.cnbc.com/2026/10/13/openai-employee-arrested-recording-device.html |
| TechCrunch | datePublished 2026-10-13T20:59:11Z = 北京 04:59:11 | “OpenAI told TechCrunch that the device contained ‘advanced encryption, audio monitoring capabilities, and a mechanism for transferring data designed to evade monitoring,’ though it is unclear to what degree this device is technically distinct from other recording devices.”；“It’s not clear exactly for what purpose the recording device was used” | https://techcrunch.com/2026/10/13/fbi-arrests-openai-employee-accused-of-planting-hidden-recording-device-in-conference-room/ |
| SF Standard | 2026-10-13T13:17:02-07:00 = UTC 20:17:02 = 北京 04:17:02（更新 13:44:21 PDT） | OpenAI 对模型用途的更具体转述（见 B2）；“The company said it coordinated the timing of his firing with the FBI … he was terminated Friday, the day he was arrested.” | https://sfstandard.com/2026/10/13/openai-engineer-arrested-alleged-bugging-executive-conference-room/ |
| Fortune | 2026-10-13 8:09 PM ET（JSON-LD 2026-10-14T00:09:31Z）= 北京 10/14 08:09 | OpenAI 称是 technical surveillance countermeasures 团队在 sweep 中发现；时间线（9/24 起…10/13 公布）；“OpenAI is using the event to showcase how its technology can help combat these types of threats.”（Fortune 的评语）；“there was clearly a continued breakdown in security, as [当事人] was allegedly able to enter the conference room again on Oct. 5”（Fortune 评语）；另有关于当事人出境机票的一句（来源未写明，控告书与 DOJ 新闻稿里**未见**，不采信） | https://fortune.com/2026/10/13/fbi-arrests-openai-employee-charged-planting-secret-recording-device-conference-room/ |
| Wired | 2026-10-13T23:17:59Z = 北京 10/14 07:17:59 | “neither prosecutors nor OpenAI have offered any information so far about a potential motive.”；“OpenAI … was able to connect it to [当事人] after reviewing chat logs and surveillance footage, and searching a bag he left unattended”；写成“attached it behind a monitor”（控告书：television） | https://www.wired.com/story/an-openai-employee-allegedly-used-a-chatbot-to-help-plant-a-recording-device-at-the-office/ |
| BBC | 2026-10-13T21:19:02Z = 北京 05:19:02 | “The company said it used its AI models to track that it had been planted by an employee.” | https://www.bbc.com/news/articles/cwg5jq4vlddzo |
| The Guardian | 2026-10-13T22:59:55Z = 北京 06:59:55 | “According to the indictment”（控告书不是 indictment）；“a second identical recording device”（控告书：“similar to”） | https://www.theguardian.com/technology/2026/oct/13/openai-employee-arrested-hidden-recording-device |
| Washington Post | “October 13, 2026 at 9:20 p.m. EDT” = 北京 10/14 09:20；**付费墙，只读到前两段** | “The government framed the arrest of the [国籍] national employee as important for national security.”（副标题） | https://www.washingtonpost.com/technology/2026/10/13/fbi-arrests-openai-employee-allegedly-planting-bug-office/ |
| Bloomberg Law | “Oct. 14, 2026, 7:52 AM GMT+8”；付费墙，只读到前两句 | “according to statements from the artificial intelligence company and the US Justice Department” | https://news.bloomberglaw.com/ip-law/openai-says-staffer-was-arrested-for-hidden-recording-device |
| 其他已读 | CBC（Reuters 稿）2026-10-13T21:05:12Z；CBS 17:36 EDT；Fox Business 18:48 EDT；Engadget 21:34:17Z；Help Net Security 2026-10-14T08:40:41Z；The Hacker News（Oct 14）；Interesting Engineering 21:19:05Z；technology.org | 无新增事实 | 见 `b-m-*.txt` |
| FT / WSJ / NYT | WebSearch 与站内检索均**未检到该案报道**（不等于没有） | — | — |

**检方或 OpenAI 对动机、录音是否外传的说明**：控告书“not detail”（AP）；Wired：“neither prosecutors nor OpenAI have offered any information so far about a potential motive”；TechCrunch：“It’s not clear exactly for what purpose the recording device was used”；FBI 局长 Patel（X 帖，2026-10-13T20:46:30Z = 北京 04:46:30；views 581,990 / likes 8,724 于 09:01Z）：“allegedly concealed a covert recording device inside his employer’s executive conference room to secretly capture sensitive communications about the company’s super intelligence technology”——这是对**意图**的指控性表述，不是对动机或委托方的说明。录音是否外传：OpenAI “no indications … exfiltrating model weights, source code, or customer data”；SD 卡内容未能解密（控告书 ¶13）；有无录音外传：**未见**。控告书、DOJ 新闻稿均未提及任何外国政府或组织指使（DOJ 新闻稿里的“foreign threats”“national security”“counterintelligence priorities”为官员引语，OPA 版 DNI 引语含 “safeguarding America’s cutting-edge intellectual property from foreign threats”）。

### B6 热度

取数时刻：HN 第一次 2026-10-14 09:22Z（北京 17:22）；复取 2026-10-14 11:41Z（北京 19:41）。HN Firebase `score`/`descendants`。

| 平台 | 对象 | 数字 |
|---|---|---|
| HN | 主讨论串 https://news.ycombinator.com/item?id=50078141 （SF Standard 稿，标题 “OpenAI engineer arrested in bugging of executive conference room”，2026-10-13T20:23:55Z 发） | 09:22Z：Algolia 574 分；11:41Z：**604 分 / 500 评论**，首页排名第 35 |
| HN | Reuters 帖 id=50078653 | 35 / 10（09:22Z 与 11:41Z 相同），topstories 第 169 |
| HN | OpenAI X 帖 id=50077703 | 84→85 / 2（评论被并入 50078141） |
| HN | DOJ OPA 稿 id=50078937 | 13 / 1 |
| HN | 其他帖 | BBC（.co.uk 50079177）8/1；CBC 50079571 3/2；Verge 50080703 5/1；Guardian 50082896 2/1；BBC（.com 50083552）5/4；Wired 50082534 1/0；decodingvibes 博客 50081745 5/0 |
| Reddit | 搜索页（all，top，today）2026-10-14 17:22 北京（页面相对时间 “13h ago” 等） | r/news “OpenAI Employee Reportedly Arrested for Planting an Audio Recording Device in an Executive Conference Room” 1.1K votes · 59 comments；r/singularity “OpenAI employee planted a bug in an office, arrested by FBI” 895 · 206；r/OpenAI “OpenAI Employee Terminated after planting bug in Office” 435 · 176；r/singularity “[国籍] spy” 319 · 106；r/OpenAI（以当事人姓名与国籍为标题的帖）301 · 61；r/accelerate “Covert operation by Dario?😅” 219 · 45；r/technology “[国籍] employee charged with planting recording device at OpenAI office” 207 · 19；r/BetterOffline “OpenAI engineer arrested in bugging of executive conference room — had asked an LLM about building, installing, obscuring, and remotely accessing a recording device.” 192 · 74 |
| X | OpenAI Newsroom 帖 | 见 B3 |
| X | FBI 局长帖 https://x.com/FBIDirectorKash/status/2110109949359751665 | 2026-10-13 20:46:30Z；views 580K / likes 8.7K（09:01Z）；680,993 / 9,745 / reposts 2,042 / replies 307（11:41Z） |
| AIHOT | https://aihot.news/hot （2026-10-14 19:35 北京更新）与首页 | “当前热点”**第 2 名**：“OpenAI工程师涉在会议室装窃听装置被捕”，热度 457（+58），4 个来源 · 62 位参与者；第 1 名 Gemini 4 Argon（604）。（WorkBuddy 扫描称第三，不符） |

### B7 夸大实例（把事件写歪的标题与句子；姓名/国籍已替换）

| 站点 | 标题或原句（逐字） | 发布时间 | 与原文的差别 |
|---|---|---|---|
| Wired | 标题 “An OpenAI Employee Allegedly Used a Chatbot to Help Plant a Recording Device at the Office”；副标题 “Chatbot prompts reveal the engineer’s alleged efforts to set up a covert listening device” https://www.wired.com/story/an-openai-employee-allegedly-used-a-chatbot-to-help-plant-a-recording-device-at-the-office/ | 2026-10-13T23:17:59Z（北京 10/14 07:17） | 控告书的提示词归在**配偶关联账号**，且 ¶18 写“attributable to [当事人] and/or his spouse”；聊天机器人是谁家的，未写 |
| decodingvibes（博客） | “OpenAI Employee Uses "Unnamed Chatbot" to Spy on OpenAI, Gets Arrested by FBI” https://decodingvibes.com/blog/openai-employee-uses-unnamed-chatbot-to-spy-on-openai-gets-arrested-by-fbi/ ；正文 “do not ask a chatbot (which is likely owned by your employer) how to do it.” | 2026-10-14 | “Uses … to Spy”“Gets Arrested”因果相连；chatbot 是否属 OpenAI 未写明 |
| 虎嗅 | “OpenAI工程师用聊天机器人策划窃听，结果被自家AI模型锁定并抓获” https://www.huxiu.com/article/4897451.html | 2026-10-14T16:01:13+08:00 | 把“配偶账号的聊天记录”与“OpenAI 称用自家模型分析设备”混成“被自家AI模型锁定”；控告书没写模型锁定 |
| 新浪（页面标题） | “藏在卫生纸里的录音设备，被 OpenAI 自己的 AI 模型揪出来了” https://www.sina.cn/weibo/detail/5353814762457044.html （页内为 IT之家微博 2026-10-14 06:59：“OpenAI 通过内部 AI 模型锁定责任人并将其解雇”） | 页面未单列标题时间；IT之家博文 06:59 北京 | OpenAI 说的是用模型“understand the device and where it had come from”；文件里的锁定依据是 CCTV、账号记录与背包 |
| Odaily | “OpenAI 办公室惊现“窃听风云”，自家 AI 化身柯南”；导语“AI协助分析了设备及来源，最终定位嫌疑人。” https://www.odaily.news/zh-CN/post/5213370 | 2026-10-14 10:41 北京 | 同上；该文自己也写“未披露具体模型和使用细节” |
| 凤凰网（凤凰AI研究院） | “35岁[国籍]员工被捕，OpenAI窃听案牵出国家级博弈” https://finance.ifeng.com/c/8xD5y1SnKq0 | 2026-10-14 09:27 北京 | 控告书无国家参与指控；页面正文未能抓到（懒加载），只存标题 |
| 微博转帖（新浪聚合页） | “美国联邦调查局以间谍罪嫌疑逮捕了OpenAI公司的一名[国籍]工程师”（用户帖，页面显示 2026-10-14 11:01:04） https://www.sina.cn/weibo/detail/5353814762457044.html | 2026-10-14 | 罪名是 18 U.S.C. § 2511(1)(b)（截取口头通信），不是间谍罪 |
| Reddit r/singularity | 帖标题 “[国籍] spy” https://www.reddit.com/r/singularity/comments/1x582df/saudi_spy/ （319 votes · 106 comments） | 约 2026-10-14 | 标签式定性；控告书无此指控 |
| HN 评论 | id=50080026 “MBS set this up…” 等评论（用户猜测） | 2026-10-14 | 无任何来源；仅记录，不得采信 |
| Guardian | “According to the indictment of [当事人]”；“a second identical recording device” | 2026-10-14 | 是 complaint 不是 indictment；设备 2“similar to” |
| AP | “near the conference room on a half-dozen occasions” | 2026-10-13 | 控告书表格 10 个日期 |
| “窃取模型权重” | **未找到**把该案写成窃取模型权重的标题；OpenAI 与各媒体均写“no indications … exfiltrating model weights”。官方措辞为 “steal sensitive communications”（AG Blanche）与 “capture sensitive communications about the company’s super intelligence technology”（FBI 局长） | — | — |
| “AI 抓间谍” | 英文标题里最接近的是 technology.org 的小标题 “OpenAI Turned Its Models on the Case”，正文 “its own AI did much of the detective work”（https://www.technology.org/2026/10/14/openai-engineer-arrested-hidden-recording-device-fbi/ ，2026-10-14T10:42:45+03:00） | 北京 15:42 | 正文把 OpenAI 的自述当事实 |

### B8 有理有据的质疑（L6 线索，进正文前须回一手来源核实）

| 质疑 | 出处与得分 | 回到一手来源 |
|---|---|---|
| 模型到底做了什么、怎么“锁定具体员工”？ | HN 评论 id=50080696：“How did the model narrow it down to a specific employee? What data did they use to determine this?”（https://news.ycombinator.com/item?id=50078141 内；HN 评论得分不公开）；Interesting Engineering：“The company also did not explain which models it used or how they helped establish the device’s origins.”；“the announcement does not establish how much of the investigation was conducted using automated analysis rather than conventional security methods.”（https://interestingengineering.com/ai-robotics/openai-fbi-arrest-hidden-recording-device ） | 控告书与 DOJ 新闻稿未提 AI 模型分析；列出的依据是 CCTV、账号记录、背包（见 B2）。OpenAI 另对 SF Standard 称模型分析了存储镜像（L4） |
| 录像才是关键证据 | HN 评论 id=50080051、50078296（设备“complexity of a high school science fair project”、“the reports are accurate and the device simply wasn’t as sophisticated as the statement implies”）；Reddit r/OpenAI（435 · 176）评论 “Probably cross-referenced outlook calendars with access logs” 与 “why would they need to use their model to find the person lol”（Reddit 评论得分匿名页不显示） | 控告书 ¶12：设备是 Seeed XIAO ESP32-S3 Sense “development board”；¶14、¶19 的 CCTV 是归因依据之一；9/28 安装时会议室无摄像头 |
| OpenAI 借案宣传 AI 能力 | HN 评论 id=50077777（该讨论串置顶评论）：“Did OpenAI do a marketing bit in the middle of an espionage accusation?”；id=50079625 “the tweet reads more like an ad then a news report”；id=50080078（自称写过 F500 新闻稿的人对措辞来源的**推测**，不可当事实）；Fortune：“OpenAI is using the event to showcase how its technology can help combat these types of threats.”；r/OpenAI 评论 “Why pass up the opportunity to tack a little hype on there.” | OpenAI X 帖末句：“this case is an example of how AI can help address these sophisticated security threats.”（B3 原文） |
| OpenAI 内部安保漏洞 | Fortune：“there was clearly a continued breakdown in security, as [当事人] was allegedly able to enter the conference room again on Oct. 5.”（评语）；Fortune 时间线 | 控告书 ¶19–¶21：10/5 进入会议室、留下背包；10/6 从 Lost and Found 取回包；OpenAI 称 9/30–10/9 逐步限制其权限，10/9 与 FBI 协调后解雇（L4） |
| “用 ChatGPT 问怎么窃听”类说法没有出处 | r/BetterOffline（192 · 74）：“Seems like he asked ChatGPT how to bug OpenAI.”（发帖人评语）；HN 50078141 内 “chatbot (which is likely owned by your employer)” 类猜测 | 控告书未点名 chatbot 产品；归属“配偶关联账号”，¶18 为 “and/or” |
| “设备被说得比实际复杂”与“加密”之说 | TechCrunch：“it is unclear to what degree this device is technically distinct from other recording devices.” | 控告书 ¶13：“may have been … encrypted”；¶12：开发板；设备 2 ¶20：“customized to obfuscate anything stored on the device” |

## 2. 可能的吠点（原文有、线索摘要没提或说反的条件与限制）

1. 控告书对“加密”只写 “may have been … encrypted”，SD 卡文件“none … could be recovered or decrypted”（控告书 ¶13）；DOJ 新闻稿与多家媒体直接写 “its contents were encrypted”。是否录到了任何内容，文件里没有证据。
2. 安装日期 9/28 来自固件编译日期的推断（控告书 ¶13 “suggesting”），9/28 当天会议室无摄像头（¶14）；当事人当天在会议室附近的 CCTV 时段是 2:18–6:30 pm（¶14 表）。
3. 控告书没有写 OpenAI “用模型”锁定当事人；写明的依据是 CCTV、productivity agent 查询、配偶关联 chatbot 账号的提示词（含设备 MAC/头部）与背包。chatbot 与 productivity agent 是谁家产品，文件没写。归因句用 “and/or his spouse”（¶18），账号关联依据是 “installation identifiers, logged IP usage, and user path name references”（¶17）。
4. “OpenAI 说有证据他还想装更多设备”不在控告书里，是 OpenAI 对媒体的说法（AP/Reuters 转述，X 帖没有）；控告书里的“第二台设备”是 10/5 背包里的 Recording Device 2（“similar to”设备 1）。
5. “no indications of … model weights, source code, or customer data”有 “At this time” 限定，不涉及会议录音是否外传（控告书与新闻稿均无录音内容）。
6. 本案目前只有控告书（complaint），法律文件反复强调 “allegedly” 与无罪推定；DOJ 新闻稿：“A complaint merely alleges that crimes have been committed”。
7. 官方（DOJ OPA 版）引语把案件与国家安全、反情报联系起来，但控告书本身无国家参与或动机的指控（控告书只有 “national security” 在 FBI 特工的背景介绍里：“investigated a wide array of federal criminal offenses, including matters related to acting as an agent of a foreign government, espionage, counterintelligence, and national security” 这一句是特工履历，不是对本案的指控）。
8. 9/30 发现后当事人仍在职至 10/9（OpenAI 说逐步限权、与 FBI 协调时间），10/5 与 10/6 仍进入办公楼（控告书 ¶19、¶21）。
9. 正文如写“AI 抓到”，须写明“OpenAI 自述用模型分析”；如写“聊天机器人”，须写明控告书未点名产品且账号关联的是配偶。
10. 设备价格 $13.99 只见 Wired，未见官方页；Seeed 公司所在地的控告书脚注与案情无关，不要借此推断。

## 3. 截图清单（B 组 13–24，均已逐张打开目视确认）

| 文件 | 内容 | 备注 |
|---|---|---|
| `13-doj-press-release-charge.png` | DOJ NDCA 新闻稿标题 + 日期 + 罪名段 | 标题里的国籍词与第一段的姓名、国籍用黑条遮盖（DOM 层，其余像素未动）；不含人像 |
| `14-complaint-device-photos.png` | 控告书第 5 页：¶11–12 与 Figure 1（卫生纸包在电视 USB 口，红圈为原文所加）、Figure 2（拆开后的设备） | 页脚脚注（Seeed 地址）未截入 |
| `15-openai-newsroom-post.png` | OpenAI Newsroom X 帖官方嵌入卡片 | 嵌入卡片折叠，仅显示前 6 行；全文见 JSON |
| `16-complaint-encrypted-may-have.png` | 控告书第 6 页 ¶13（“may have been … encrypted”、固件 9/28 编译） | 无姓名 |
| `17-complaint-cctv-dates.png` | 控告书 ¶14 与 10 行 CCTV 日期表（跨第 6–7 页拼接） | 姓名黑条（OCR 词框遮盖）；拼接处为分页，不改像素 |
| `18-complaint-productivity-agent-queries.png` | ¶15–16：productivity agent 查询表 | 姓名黑条 |
| `19-complaint-spouse-chatbot-prompts.png` | ¶17–18：配偶关联账号提示词（九条） | 姓名黑条；跨第 7–8 页拼接 |
| `20-complaint-device2-bag.png` | ¶20 与 Figure 4（设备 2） | 姓名黑条；跨第 9–10 页拼接 |
| `21-doj-press-release-penalty.png` | DOJ 新闻稿罚则段（5 年、$250,000、§ 2511(1)(b)、“A complaint merely alleges”） | 姓名黑条 |
| `22-doj-press-release-device-and-ai-prompts.png` | DOJ 新闻稿：发现与加密段、Figure 1/2、CCTV 段、productivity tool 与 chatbot 账号段 | 姓名黑条；不含当事人人像 |
| `23-sfstandard-openai-account.png` | SF Standard 转述 OpenAI：模型分析存储镜像、有证据打算再装设备 | 姓名黑条；中间夹有站内“More about OpenAI”推荐框（页面原样） |
| 24 | 空缺 | 未截 Reuters（打不开）与媒体页 |

截图工具：13、21、22、23 为匿名 headless Chrome（Playwright Core，2 倍像素，700/900 CSS px 版式，姓名与国籍用 DOM 黑条）；14、16–20 为 `pdftoppm` 326 dpi 渲染 + tesseract 词框黑条遮盖 + 缩到宽 ≤1400 px；15 为 X 官方嵌入页匿名渲染。**按协调方 10/14 的新要求，后续截图改用 opencli；本组截图在该要求下达前已全部完成，未重做。**

## 4. 扫描说法勘误

| 扫描/已知说法 | 核对结果 |
|---|---|
| Verge：“当事人 arrested Friday、周二首次出庭” | 对。Friday = 2026-10-09；周二 = 10/13（DOJ 与 RECAP 案卷）。 |
| Verge：设备“约 9 月 28 日安装、内容加密” | 来自 DOJ 新闻稿；控告书口径是“固件编译于 9/28，suggesting …installed on or about that date”与“may have been encrypted”（见 B2）。 |
| Verge：“监控录像显示多日出现在会议室附近、并拍到他看电视背后” | 对（控告书 ¶14 表 10 个日期；¶19 10/5 约 1:20 pm）。录像拍到“看电视背后”是 10/5，此时设备 1 已被 OpenAI 于 9/30 前后取走。 |
| “OpenAI 安保在另一房间发现他留下的包” | 对（¶20：printer room）。注意：是 OpenAI 安保打开包，“钱包、证件和另一个录音设备”（¶20）；当事人 10/6 取回包时设备 2 已不在（¶21）。 |
| OpenAI 声明引文 | X 帖原文已核；“We used our models to understand the device…” 在 X 帖里确有；“OpenAI 有证据他打算在其他地方安装更多设备”**不在 X 帖里**，是对媒体的补充。 |
| Reuters “19:59 UTC 发布” | Kelo 转载版 `datePublished` 为 2026-10-13T19:59:12Z，吻合；Reuters 原页打不开，未直接核。 |
| “HN Reuters 帖约 35/10（低）” | 对（35/10）。但 HN 主讨论串是 id=50078141（SF Standard 稿）604 分/500 评论，Reuters 帖不是主要讨论所在；OpenAI X 帖与 BBC、DOJ 等帖被并入该串。 |
| “AIHOT 称当前热点第三” | AIHOT 2026-10-14 19:35 北京：第 2（热度 457）。 |
| 扫描未提：控告书原件与案卷 | 已取得，见 B1。 |
| 扫描未提：OPA 版新闻稿含 DNI 与 NSPM-11 的引语 | 见 B1.2；仅作引文，不作推断。 |

## 5. 本组新增文件（均在 `docs/2610/1013/sources/` 与 `images/`）

见 `capture-log-b.md` 第 3 节。
