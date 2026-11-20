# 1119 期 D 组（速览）取证

执行方模型：claude-haiku-5-5。取证时间：2026-11-20 22:12–约 22:50（北京时间，UTC+8）。本组不截图；存档与摘句见 `d-` 前缀文件，抓取记录见 `capture-log-d.md`。

级别：L1 = 官方发布、法院文书、公司公告、SEC 申报或国会委员会披露件；L3 = 第三方评测原始页；L4 = 权威媒体；L1（自述）= 当事方页面，正文须写“某方称”。

## 〇、一句事实（每条限定词照原文保留）

- **D1**：众院外委会 11-19 公开的 ECRA 披露件称，截至 2026-05-11，BIS 收到向中国企业出售 3,444,160 颗受管 AI 芯片（NVIDIA H200、AMD MI325X）的申请，批准 879,868 颗，涉及 21 份许可。
- **D2**：Palo Alto Networks 11-18 宣布以“all-stock”方式、约 45 亿美元（“valued at $4.5 billion, subject to adjustments”）收购 Applied Compute，预计在其 FY2027 第三财季交割，须满足监管批准等条件。
- **D3**：英国 DeepMind 员工工会 UTAW 称，谷歌拒绝其自愿承认的请求，ACAS 调解未成，已申请 CAC 法定承认（法定范围仅含薪酬、工时与假期）。
- **D4**：美国东肯塔基联邦法院 11-09 驳回被告警探的判决前动议（judgment on the pleadings）：仅凭未经佐证的 AI 比对结论不得作为逮捕依据，且“在本阶段”不予合格豁免；判决以“假定原告主张属实”为前提。
- **D5**：Gemini 应用 Mac 版 11-19 推出 computer use，据官方仅限美国、英文、Apple Silicon Mac（macOS Sequoia 15.0+），套餐为 Google AI Plus、Pro、Ultra 用户。
- **D6**：Anthropic 11-17 研究页称 Claude 解出黑洞相关两个开放问题；其中一题证明的一半“follows the strategy of the OpenAI proof”，结果于 9 月“formally verified in Lean”，论文由 Claude 撰写、“a physicist checked every main result”。
- **D7**：Anthropic 424B4（EDGAR 申报日 11-19）：IPO 价 $83.00，卖出股东合计 23,648,646 股；TechCrunch 称七位创始人合计约 19.5 亿美元（“$1.95 billion”）；联合创始人承诺把各自“personal equity”的 80% 用于慈善，原文称“voluntary, revocable … do not constitute legally binding obligations”。
- **D8**：微软称 MAI Image 2.7 截至 11-16 在 AA-Image-Editing v2.1 排第一（1,215 Elo，5,478 票），领先 GPT Image 2.6 Sunburst (max) 4 Elo；Foundry 公开预览约 $35/千张。AA 榜页截取时为 1,215 ±14 与 1,211 ±13（样本数已更新，见下）。
- **D9**：微软 Edge 11-19 宣布 2.3B 参数的 Aion Instruct 作为 Edge 内置模型并以开放权重发布于 Hugging Face；网页 Prompt API 为 origin trial；Decisions API 仅为 Canary/Dev 频道的开发者预览。
- **D10**：OpenAI 会计插件：ChatGPT Business 用户可无需管理员批准安装，Enterprise 需管理员批准；官方称入账前应“review the company, destination, and amounts before approving”；读写权限可能不同。发布日期未找到。
- **D11**：MIT 11-19 新闻稿：模型估算 2023 年美国数据中心用电相关空气污染导致约 650 例过早死亡（513 例 PM2.5、137 例臭氧），发表于 Environmental Research Letters；模型假设数据中心 24/7 满负荷运行。

## 一、条目事实（说法 | 级 | 来源 | 摘句 | 条件 | 状态）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| D1 美国众院外委会公开对华 AI 芯片许可材料：申请约 340 万颗、批准逾 87.9 万颗 | L1 | 通稿 https://foreignaffairs.house.gov/news/press-releases/chairman-brian-mast-exercises-legislative-authority-to-publicly-release-ai-chip-license-information-for-china ；披露件（Scribd 嵌入）https://www.scribd.com/embeds/1098651862/content?start_page=1&view_mode=scroll&access_key=key-WkuMowyLbX5OPTJOKBvc 与 …/1098651808/content?… | 通稿：“requested sales of 3.4 million AI chips to China, and the approval of more than 879,000 by BIS”。披露件：“BIS received applications to sell 3,444,160 covered integrated circuits to PRC companies”；“BIS approved the export, re-export, or transfer (in-country) of 879,868 covered integrated circuits to PRC companies across 21 license applications, as of May 11, 2026.” | 无（D 组不截图） | 披露件时点：“as of May 11, 2026”。覆盖产品：“NVIDIA’s H200 and AMD’s MI325X”。通稿时间：通稿页 published_time 2026-11-19T14:18:08-05:00（北京 11-20 03:18）。通稿未写时段；CNBC 转述“From December through May”。 | 已找到 |
| D1 措辞差异 | L4 | CNBC https://www.cnbc.com/2026/11/19/nvidia-amd-china-chip-sales-congress.html | CNBC 标题：“Commerce approved sales of more than 879,000 AI chips to China: Mast”；CNBC 正文：“approved the sale of more than 879,000 AI chips to Chinese companies including Alibaba and Tencent as of May 2026” | 无 | 媒体用 approved sales，通稿与披露件用 approval 与 applications；CNBC 发布 2026-11-19 14:55 EST（北京 11-20 03:55） | 与说法不符（仅限措辞：通稿与披露件并非“sales”批准，媒体标题为“sales”） |
| D1 实际出货 | L1（披露件附表）；L1（Nvidia 发言，经 CNBC 转引） | 披露件附表（同上）；CNBC 同上 | 附表标题：“H200 License and Equivalent Application Status as of May11, 2026”；列内状态：21 行均为“Approved, not shipped”（附表内无“Shipped”）。Nvidia 发言人 John Rizzo：“Despite receiving some licenses, we have since stopped shipping H200s to China.” | 无 | 附表只覆盖 21 行，披露件正文称“not fully reflected”其他申请；附表为 5 月 11 日口径，Nvidia 发言为 11-19 口径 | 部分支持（附表与厂商发言方向一致，但无出货数字） |
| D2 Palo Alto Networks 以约 45 亿美元全股票收购 Applied Compute | L1 | https://investors.paloaltonetworks.com/news-releases/news-release-details/palo-alto-networks-acquire-applied-compute-bringing-secure-open | “The all-stock transaction is valued at $4.5 billion, subject to adjustments.”；“subject to customary closing conditions, including regulatory clearance, and is expected to close in Palo Alto Networks' fiscal third quarter of 2027.” | 无 | 通稿日期 “Nov. 18, 2026 /PRNewswire/”，官方未给时刻；交割预期为财季 FY2027 Q3 | 已找到 |
| D2 融资稿：估值与年化收入 | L1 | https://www.appliedcompute.com/company/100m-run-rate-fundraise | 页面显示 “NOVEMBER 2, 2026”；“We’ve just raised $350 million in new funding at a $3.25 billion valuation.”；“we've grown from zero to $100 million in annualized revenue run rate.” | 无 | 页面 published_time 2026-11-02T23:24:00Z，即北京 11-03 07:24（扫描称 11/03 按北京日期正确；页面显示的 Nov 2 为美国日期）；原文未注明 3.25 billion 为投前或投后 | 已找到 |
| D3 英国 DeepMind 员工工会（UTAW） | L1（工会自述） | https://www.gdmunion.co.uk | “We asked for voluntary recognition on 4 May, but Google refused.”；“Negotiations via ACAS also failed, so we are now applying for statutory recognition.”；站内另一处：“We didn't reach an agreement, so we applied for statutory recognition through the Central Arbitration Committee (CAC).”；“By law, statutory recognition covers pay, hours and holidays.” | 无 | 站上两处时态不一（已申请 / 申请中），统一写“已申请”；“Google refused”的依据是站内引用的 Reuters 报道（未核）；站上未给成员人数；CAC 需 10% 成员门槛（站上写法）；仅覆盖 Google DeepMind FTE | 已找到（须写“工会称”） |
| D4 美国联邦法院：仅凭人脸识别比对申请逮捕令的警探暂不获合格豁免（Osorio v. Kilmer，驳回动议阶段） | L1 | 判决书（Reuters 页内链接副本）https://fingfx.thomsonreuters.com/gfx/legaldocs/gkplennlgvb/Osorio%20Kilmer.pdf ；案号 2:26-cv-00004-SCM（E.D. Ky.）。DocumentCloud 版 403 未取；CourtListener 搜索 401（需认证，未用凭据） | “the Fourth Amendment does not permit an individual to be arrested based on nothing more than an uncorroborated conclusion by AI that the individual in question has committed a crime.”；“Accordingly, the Defendant’s motion for judgment on the pleadings must be denied.” | 无 | 判决书首页 “Filed: 11/09/26”，末页 “Signed this 9th day of November, 2026”（星期一），时刻未给；裁定：Dkt. 12 驳回（DENIED），Dkt. 14 准许修改起诉书；前提：“assuming the truth of the Plaintiff’s allegations—which the Court must do at this juncture”；“Of course, discovery might show that there is more to the story than the Plaintiff alleges.” | 已找到 |
| D4 “暂不获合格豁免” | L1 | 同上 | “the Court finds that the § 1983 claims in Osorio’s Second Amended Complaint contain enough factual allegations to survive […] and defeat qualified immunity at this stage of the action.”（省略号处为被告动议名，含私人姓名，已略） | 无 | 限定词 “at this stage of the action”，驳回动议阶段，非终局 | 已找到 |
| D4 Reuters 的时间口径 | L4 | https://www.reuters.com/legal/government/man-wrongly-jailed-after-ai-mismatch-can-pursue-lawsuit-now-2026-11-10/ | “US District Judge S. Chad Meredith on Friday denied the detective's bid to dismiss the case for now” | 无 | Reuters 页面 2026-11-10；“on Friday” 与判决书签署日 2026-11-09（星期一）不符，以判决书为准 | 与说法不符（仅限时间口径） |
| D5 Gemini Mac 版上线 computer use | L1 | https://blog.google/innovation-and-ai/products/gemini-app/computer-use-mac/ | “Computer use is rolling out in English in the U.S. in the Gemini app for Mac.”；“Requires an Apple Silicon Mac running macOS Sequoia 15.0+. Available for personal Google accounts 18+, available to Google AI Plus, Pro and Ultra users.” | 无 | 页面 published_time 2026-11-19T17:00:00Z，即北京 11-20 01:00；地区、语言、设备、套餐见摘句（扫描未列设备限制） | 已找到 |
| D5 授权与暂停机制 | L1 | 同上 | 页面两段正文措辞不同：“You remain in control by approving every action and choosing exactly which apps Gemini can access.” 与 “Privacy and safety are built in: you choose which apps Gemini can access, watch every step live, pause at any time, and approve key actions before they happen.” | 无 | 两段出现在同一页面，所在区块未逐一确认；正文转述以各段原文为准，不能混用 | 已找到（措辞两处不同，需照原文） |
| D5 官方承认的风险 | L1 | 同上 | 页面未见风险、误操作或提示注入的官方承认句 | 无 | 未找到官方风险表述 | 未找到一手来源（风险句） |
| D6 Anthropic 黑洞两项成果：22 个候选、公开 2 个、一半证明沿用 OpenAI 思路 | L1 | https://www.anthropic.com/research/claude-black-hole-physics | “Claude has solved two open problems in theoretical physics.”；“Claude came back hours later with 22 candidates” | 无 | 页面 published_time 2026-11-17T00:00:00Z（疑为日期占位，北京 11-17 08:00）；22 候选中其余 20 个的下场页内未交代 | 已找到（其余 20 个：未找到） |
| D6 证明沿用 OpenAI 思路的范围 | L1 | 同上 | “One half of the final proof, showing that the ceiling can’t be lowered, follows the strategy of the OpenAI proof.” | 无 | 仅限模数自举（modular bootstrap）一题证明的一半；“the ceiling” 为 M₁ ≤ c/Λ 的上界 | 已找到（须限定到该题、该半） |
| D6 Lean 形式化与物理学家核对 | L1 | 同上 | “In September, the results were formally verified in Lean, a computerized proof checker.”；“Claude then wrote the paper with minimal guidance, and a physicist checked every main result.” | 无 | 未写明 Lean 覆盖哪一半、哪些结果（“the results” 不指明）；物理学家未具名 | 已找到（覆盖范围：未找到） |
| D6 “不做同行评审的 Hexagon”“署名只有 Claude” | L1 | 同上（预印本 hexagonmath.org 未取） | 本页无 “peer review” 与署名字样（检索无命中） | 无 | 1109 期已发的两点，本页无法核实 | 未找到一手来源 |
| D6 限定句 | L1 | 同上 | “Neither result describes our own universe directly, and neither has practical applications”；“The most natural possibility is that it stays abrupt everywhere in between.” | 无 | 后一句是作者推测（“most natural possibility”） | 已找到 |
| D7 Anthropic 7 位创始人 IPO 售股合计约 19.5 亿美元 | L4（招股书 L1） | TechCrunch https://techcrunch.com/2026/11/19/anthropics-7-founders-cashed-out-nearly-2b-in-record-breaking-ipo/ ；424B4 https://www.sec.gov/Archives/edgar/data/2130357/000162828026074944/anthropicpbc-424b4.htm | TechCrunch：“including $1.95 billion for its seven founders”；“each of them ponied up about 3.4 million shares apiece”；“they each pocketed about $280 million before underwriting fees and taxes.”。424B4：“The initial public offering price per share is $83.00.”；“selling an additional 23,648,646 shares of Class A common stock.” | 无 | 424B4 正文未见逐人售股表（“Selling Stockholders”一节只作交叉引用）；23,648,646 × $83.00 ≈ $1.963 billion（毛额，本组推算，未扣承销费）；TechCrunch 同文另写募资 “$81.3 billion” 与 “$104.8 billion”，未调和；TechCrunch 页面 published_time 2026-11-20T00:43:50Z（北京 11-20 08:43） | 已找到（创始人分项只见 TechCrunch，424B4 只有总股数） |
| D7 “捐出 80% 股权”的承诺性质 | L1 | 同上 424B4 | “our co-founders have each pledged to dedicate 80% of their personal equity in Anthropic to philanthropic causes.”；“These commitments are voluntary, revocable expressions of each co-founder’s philanthropic intent and do not constitute legally binding obligations.” | 无 | 段落在 424B4 的 “Shares Eligible for Future Sale” 相关段落之后（前文 “available for immediate resale in the United States in the open market” 之后，页码 123 附近；章节标题未逐字核对）；标的是 “personal equity”，不是公司全部股权 | 已找到（措辞须改为“个人股权”） |
| D8 微软 MAI Image 2.7：AA 图像编辑榜第一、领先第二 4 Elo | L1（厂商）；L3（AA 榜页） | 微软 https://microsoft.ai/news/introducing-mai-image-2-7/ ；AA https://artificialanalysis.ai/image/leaderboard/editing | 微软脚注：“As of November 16, 2026, MAI Image 2.7 ranks #1 on AA-Image-Editing v2.1, scoring 1,215 Elo across 5,478 votes, ahead of GPT Image 2.6 Sunburst (max) by 4 Elo.”。AA 榜页（取样时）：MAI-Image-2.7 “1215 ±14 95% interval 1201 to 1229 5,526 Nov 2026 $34.82 /1k imgs”；GPT Image 2.6 Sunburst (max) “1211 ±13 95% interval 1198 to 1224 6,600 Nov 2026 $210.72 /1k imgs” | 无 | 微软脚注时点 11-16（5,478 票）；AA 榜页取样时间为北京 11-20 约 22:4x（样本数 5,526 与 6,600，与脚注不同时点，不可直接比较）；两者 95% 区间重叠（1201–1229 与 1198–1224，本组推算）；“ahead by 4 Elo”由两方数据一致 | 已找到（AA 榜页与微软脚注一致，但区间重叠，见勘误） |
| D8 价格与开放范围 | L1 | 微软同上 | “Available in public preview on Microsoft Foundry, MAI Image 2.7 costs ~$35 per 1,000 images while Flash costs ~$10 per 1,000 images.”；“Both models are available in public preview through Microsoft Foundry and partners including fal, Genspark, OpenRouter, and Vercel.” | 无 | 公开预览（public preview）；AA 价 $34.82/1k imgs 与之相近；页面 published_time 2026-11-17T18:16:36Z（北京 11-18 02:16） | 已找到 |
| D9 备用：Edge 内置 2.3B 本地模型 Aion Instruct，网页 Prompt API 为 origin trial | L1 | https://blogs.windows.com/msedgedev/2026/11/19/hybrid-intelligence-on-the-web-introducing-aion-instruct-and-prompt-api-in-microsoft-edge/ | “At 2.3B parameters, Aion is smaller, faster, and available on many more devices than Phi, while delivering comparable quality for common web tasks”；“An origin trial for the Prompt API, so you can use the Prompt API and Aion Instruct on your live site today.”；“It is available as a developer preview in Edge Canary and Dev channels for prototyping.”（Decisions API） | 无 | published_time 2026-11-19T23:08:46Z（北京 11-20 07:08）；“Edge downloads Aion Instruct only when a site or feature first needs it, and only on devices that can run it well.”（无具体设备规格）；“Users stay in control and can turn off on-device AI in Edge settings”；Hugging Face 开放权重 | 已找到 |
| D10 备用：OpenAI ChatGPT 会计插件 | L1 | https://openai.com/business/solutions/accounting/ ；帮助页 https://help.openai.com/en/articles/20001709-using-chatgpt-for-accounting-work | “Review supporting records, calculations, and suggested coding before approving entries or reports.”；“ChatGPT Business members can install Accounting without prior admin approval. In an Enterprise workspace, an admin must approve Accounting before members can install it.”；“A connection that lets you read transactions may have different permissions for entering records.” | 无 | 发布日期未找到（页面无 published_time，帮助页只有相对时间 “Updated: 10 hours ago”）；“The Accounting plugin is free to install. Access to the model, tools, and usage included with your plan can vary.” | 已找到（发布日期：未找到） |
| D11 备用：MIT 研究估算 2023 年美国数据中心用电相关空气污染约致 650 例过早死亡 | L1（机构新闻稿；论文未取） | https://news.mit.edu/2026/study-reveals-data-centers-long-distance-damage-air-quality-1119 | “The team estimates that additional air pollution from the electricity demands of data centers in 2023 resulted in about 650 premature deaths across the United States.”；“The model estimates that emissions from data centers caused about 650 premature deaths in 2023 (513 from PM2.5 and 137 from smog)”；“The research appears today in Environmental Research Letters” | 无 | 页面日期 November 19, 2026（时刻未给）；限定：“the researchers assume data centers draw power to run computing tasks 24/7.”；论文 DOI 10.1088/1748-9326/aebbd4（页内链接，未取原文） | 已找到（论文原文未取） |

## 二、发布时刻与社区热度

发布时刻：北京时间。按页面元数据（published_time）换算的，标“元数据”；仅有日期的标“官方未给时刻”。

| 条目 | 发布时刻（北京时间） | 热度（HN，取于 2026-11-20 约 22:4x） |
|---|---|---|
| D1 通稿 | 2026-11-20 03:18（元数据，原 2026-11-19 14:18 -05:00）；CNBC 2026-11-20 03:55（元数据 2026-11-19 19:55 UTC） | 无相关 HN 命中 |
| D2 PANW 通稿 | 2026-11-18（“Nov. 18, 2026”，官方未给时刻）；融资稿 2026-11-03 07:24（元数据） | HN 50554009：82 分 / 26 评 / 北京 11-19 07:49 |
| D3 UTAW | 站上无日期（官方未给）；HN 北京 11-19 17:28 | HN 50557885：123 分 / 84 评 / 北京 11-19 17:28 |
| D4 判决 | 判决签署 2026-11-09（美东，官方未给时刻）；Reuters 2026-11-10；Techdirt 2026-11-20 05:19（元数据 2026-11-19 21:19 UTC） | HN 50567735：67 分 / 19 评 / 北京 11-20 06:56（扫描称 139 分，已勘误） |
| D5 Gemini Mac | 2026-11-20 01:00（元数据 2026-11-19 17:00 UTC） | HN 50571810：2 分 / 0 评 / 北京 11-20 16:34；HN 50563179：1 分 / 0 评 / 北京 11-20 01:32；HN 50566221（MacRumors，1 分 / 0 评 / 北京 11-20 04:54） |
| D6 Anthropic 研究页 | 2026-11-17（元数据 00:00 UTC 疑为日期占位，北京 11-17 08:00） | HN 50563148：518 分 / 203 评 / 北京 11-20 01:30（扫描称 516 分 / 242 评，评论数口径不一） |
| D7 TechCrunch / 424B4 | TechCrunch 北京 11-20 08:43（元数据）；424B4 EDGAR 申报日 2026-11-19（时刻未取） | 无相关 HN 命中 |
| D8 MAI Image 2.7 | 北京 11-18 02:16（元数据 2026-11-17 18:16 UTC）；脚注数据截至 11-16 | HN 50536193：3 分 / 0 评 / 北京 11-18 02:44 |
| D9 Edge Aion | 北京 11-20 07:08（元数据 2026-11-19 23:08 UTC） | 无相关 HN 命中 |
| D10 OpenAI 会计 | 未找到发布日期 | 无相关 HN 命中 |
| D11 MIT | 2026-11-19（页面日期，时刻未给） | 无相关 HN 命中 |

Reddit：取样失败（搜索页跳转至登录，返回 HTML 而非 JSON），热度无数据。

## 三、扫描说法勘误

1. **D1**：扫描称委员会通稿用 approval 而非 sales，成立；通稿写 “requested sales of 3.4 million”（申请销售）与 “the approval of more than 879,000”（批准）。披露件进一步给出 “applications to sell 3,444,160”、“approved … 879,868 … across 21 license applications”，时点均为 2026-05-11。CNBC 标题用 “approved sales”，属媒体措辞。扫描称“分项数据覆盖 2025-12 至 2026-05（出自朝鲜日报中文版）”：披露件附表的 21 行提交日期在 2025-12 至 2026-04 之间（据文本抽取，未逐行核对），“December through May”是 CNBC 的转述。扫描称路透 5 月“尚无一颗交付”：披露件附表 21 行全部标 “Approved, not shipped”，与之方向一致，但路透原稿未取。
2. **D1 芯片范围**：CNBC 转述称 “H200 and comparable chips”；披露件明确 “covered integrated circuits are the following products: NVIDIA’s H200 and AMD’s MI325X”。以披露件为准。
3. **D2 融资日期**：扫描称 11/03，成立（北京时间 11-03 07:24；页面显示 “NOVEMBER 2, 2026” 为美国日期）。扫描称估值为“投后”，原文未注明投前或投后。
4. **D4 热度**：扫描称 HN 139 分，取样为 67 分（19 评），与扫描不符。
5. **D4 文书获取**：扫描称意见书 PDF 在 DocumentCloud 被拦，属实（403）；判决书已由 Reuters 页内链接取得。CourtListener 需认证（401），未取。
6. **D4 日期**：Reuters 称法官“on Friday”，判决书签署于 2026-11-09（星期一）。以判决书为准。
7. **D5 限定**：扫描称“限美国、英文、AI Plus/Pro/Ultra”成立；官方另有 Apple Silicon Mac 与 macOS Sequoia 15.0+ 的设备限制（扫描未提）。同页两段“授权”措辞不一（approve key actions / approving every action）。
8. **D6 证明范围**：“一半证明沿用 OpenAI 证明的思路”成立，但原文限定为模数自举一题上界证明的一半。“公开 2 个”：原文称 Claude solved two open problems；其余 20 个候选的下场未交代。“不做同行评审的 Hexagon”“署名只有 Claude”：本页未见，未核实。
9. **D7 措辞**：“承诺捐出 80% 股权”应改为“每位联合创始人承诺把其个人股权的 80% 用于慈善”（424B4 原文 personal equity）。“自愿、可撤销、无法律约束”成立。“合计售股约 19.5 亿美元”成立（TechCrunch $1.95 billion；按 424B4 总股数 × $83.00 为约 19.63 亿毛额，本组推算）。
10. **D8**：“截至 11-16”与“领先 4 Elo”成立，AA 榜页亦为 4 Elo；但两者 95% 区间重叠（1201–1229 与 1198–1224），本组推算，正文不宜写成“确定领先”。“Foundry 约 35 美元/千张”成立（AA 显示 $34.82/1k imgs）。

## 四、未完成与待核

- D1：路透 2026-05 原稿未取（附表与披露件已覆盖“是否出货”的一手口径）。10-Q 原文未取（引用 CNBC 转引）。
- D4：CourtListener 401（需认证，未用凭据）；DocumentCloud 403。
- D6：Hexagon 预印本页未取；“不做同行评审”与“署名”的一手页面未找到。
- D7：424B4 全文 HTML 约 14 MB，未存全文，只存命中段落（`d-edgar-424b4-excerpt.txt`）；SEC 对 curl 与 barking fetch 返回 403，经 opencli 浏览器会话取得。
- D10：发布日期未找到。
- 额外线索（DistroKid 向 UMG/Sony 交出 AI 检测文件、OpenAI dots、Google Stitch waitlist）本组未查。
- 社区评论（吠点线索）按派工单，D 组不做。

## 五、文件

文本与记录（入库）：`evidence-d.md`、`capture-log-d.md`、`d-fetch-list-1.tsv`、`d-fetch-list-2.tsv`、`d-fetch-list-3.tsv`、`d-fetch-log.jsonl`、`d-heat.json`、`d-hn-50554009.json`、`d-hn-50557885.json`、`d-hn-50563148.json`、`d-hn-50567735.json`、`d-edgar-424b4-excerpt.txt`、`d-edgar-search.json`、`d-edgar-sections.json`、`d-edgar-snippets.json`、`d-scribd-1-disclosure.txt`、`d-scribd-2-disclosure.txt`、`d-aa-image-editing-leaderboard.txt`、`d-osorio-ruling-pdf.txt`、`d-techdirt-links.json`，以及 `d-*.txt`（ok 条目与 opencli 抽取）。

原件（不入库，传网盘）：`d-osorio-ruling-pdf.pdf`、`d-*.html`（ok 条目）、`d-panw-opencli.html`、`d-reuters-opencli.html`、`d-techdirt.opencli.html`。

待删除清单见 `capture-log-d.md` 第八节。
