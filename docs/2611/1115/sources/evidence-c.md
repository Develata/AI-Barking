# 1115 期 C 组取证：速览（执行方 claude-haiku-5-5）

时区：北京时间（UTC+8）。存档文件前缀 `c-`，位于本目录。未截图（C 组不需要）。`barking heat` 结果：`c-heat.json`（C1–C5 的 HN 搜索，零命中）、`c-heat-c6-item.json`（C6 的 HN 条目）、`c-hn-50515058.json`。

## 逐条证据

| # | 说法（一句事实） | 级 | 一手来源 URL 与存档 | 原文摘句（逐字） | 条件 / 口径 / 时刻 | 状态 |
|---|---|---|---|---|---|---|
| C1 | Apple 与 OpenEvidence 合作开展健康 AI 研究，招募最多 20 万名 21 岁以上美国用户 | L1 | https://www.apple.com/newsroom/2026/11/apple-launches-research-study-to-explore-the-use-of-ai-for-health/ 存档 `c-c1-apple.html` / `.txt` | “open to up to 200,000 eligible participants age 21 and older across the United States. To join, participants must have an Apple Intelligence-enabled iPhone (iPhone 15 Pro or later) running iOS 26 or later.” | 官方仅写 “November 15, 2026”（美国时间），**未给时刻**。机型以 Apple 原文为准：iPhone 15 Pro or later | 部分支持（规模、年龄、地区、机型已找到；见下 C1 勘误） |
| C1 | 研究数据不用于训练 AI 模型 | L1 | 同上 | “study data will not be used to train AI models” | 同上 | 已找到 |
| C1 | 约 12 个月；发送第三方云前匿名化；OpenEvidence 24 小时删除查询 | L4 | TechCrunch https://techcrunch.com/2026/11/15/apple-launches-new-app-to-study-how-people-use-ai-for-health/ 存档 `c-c1-techcrunch.html` / `.txt`；Apple 原文**未写**这三点 | TechCrunch：“for more than 12 months”；“anonymized before sending it to a secure third-party cloud provider”；“OpenEvidence deletes the query and response data after 24 hours.” | TechCrunch 时间戳 “5:30 PM PST · November 15, 2026”，即北京 11-16 09:30（`2026-11-16T01:30Z`）。只有 L4 有此句，官方页无 | 仅 L4 支持（Apple 原文未核到） |
| C1 | 机型表述：TechCrunch 写 “iPhone 15 or later”；Verge 写 “iPhone 15 Pro” | L4 | TechCrunch（同上）；The Verge https://www.theverge.com/tech/1033355/apples-new-study-looks-to-improve-the-effectiveness-of-ai-health-tools （2026-11-16T01:26:56Z，北京 09:26）未存档全文，仅搜索摘录 | TechCrunch：“using iPhone 15 or later, and running iOS 26 or later” | 与 Apple 原文 “iPhone 15 Pro or later” 不一致，以 Apple 为准 | 与说法部分不符（TechCrunch 简写） |
| C2 | 欧洲央行副行长 Vujčić（Euro Finance Week 演讲）称 AI 风险从估值扩展到债务与供应商集中 | L1 | ECB https://www.ecb.europa.eu/press/key/date/2026/html/ecb.sp261116~db374b56ab.en.html 存档 `c-c2-ecb-en.html` / `.txt`（英文为准）；立陶宛语版 `c-c2-ecb-lt.*` 备查 | “AI-related risks to system-wide stability now stem from a range of additional factors – the expansion of external funding from equity to debt markets, concentration of exposures, quickening AI adoption and potential cyberattacks.”（原句出自 “Keynote speech by Boris Vujčić, Vice-President of the ECB, at the Euro Finance Week”，“Frankfurt am Main, 16 November 2026”） | 官方页只给日期，**未给时刻**。官方原文为演讲稿（speech）；“two stand out very prominently”指 AI 与主权债务两项 | 已找到 |
| C2 | 美国主要科技企业累计资本开支到 2030 年预计 5.5 万亿美元 | L1 | 同上 `c-c2-ecb-en.txt` | “cumulative capital expenditure by major US technology firms is projected to reach USD 5.5 trillion by 2030.” | ECB 原文为“资本开支”的**预测**，演讲稿内**未署出处**。Reuters 转述为 “$5.5 trillion financing need”（见勘误） | 已找到（口径：资本开支预测，非融资需求） |
| C2 | 时刻参考 | - | Reuters 经 Euronext live 转载 https://live.euronext.com/en/financial-news/ecb-vp-says-ai-debt-have-become-prominent-stability-risks （2026-11-16T08:33:51Z，北京 16:33，“FRANKFURT, Nov 16 (Reuters)”，“said on Monday”）；Euronews https://www.euronews.com/2026/11/16/ecb-warns-ai-share-boom-could-end-in-abrupt-repricing （2026-11-16T15:15:42Z，北京 23:15）存档 `c-c2-euronews.*` | Reuters：“...given the $5.5 trillion financing need of major US technology firms through 2030.” | Reuters 与原文措辞不同，以 ECB 为准 | 部分支持（L4 转述有偏差） |
| C3 | 美联储理事 Barr 称大型银行报告发现 AI 智能体试图操作零售客户账户 | L1（官方演讲） | https://www.federalreserve.gov/newsevents/speech/barr20261116a.htm 存档 `c-c3-barr.html` / `.txt`；官方元数据日期 November 16, 2026 | “For example, major banks now report finding AI agents attempting to take actions in retail customer accounts, without the bank being able to verify authenticity, consent, and alignment with customer instructions.” | 演讲地点 Milan，ACM 人工智能金融国际会议（Seventh Annual）；官方页**未给时刻**。未给银行名单、次数或金额，原文无此数据（记“未写”） | 已找到（原句）；无次数、金额、银行名 |
| C3 | 脚注：观点为个人观点 | L1 | 同上 | “The views expressed here are my own and are not necessarily those of my colleagues on the Federal Reserve Board or the Federal Open Market Committee.” | 脚注 1 | 已找到 |
| C3 | AI 智能体可能加速存款外流（挤兑），条件性表述 | L1 | 同上 | “In addition, AI agents may accelerate deposit outflows during stress periods.” … “potentially accelerating runs that unfold in minutes, overwhelming traditional liquidity management and regulatory intervention timelines.” | 带 “may / could / potentially” 的假设情景，**不是已发生的事件** | 已找到（限定：条件性） |
| C3 | 七家最大 AI 企业表内债务与表外义务 | L1 | 同上 | “On-balance-sheet debt of the seven largest AI firms ... is expected to exceed $800 billion before year's end.” / “The FSR estimates that the potential maximum nominal value of these types of obligations, or off-balance-sheet liabilities, now exceeds $3 trillion.” | 前者为**预计**（before year's end）；后者为 FSR 估计的“潜在最大名义价值”，不是实际债务 | 已找到（限定：预计/潜在最大名义值） |
| C3 | Reuters 报道时刻与标题 | L4 | 未取到 Reuters 原页（网页 403/未检索到）；经 casrai 转述 Reuters “Ann Saphir” 报道（无时刻） https://casrai.org/news/fed-barr-ai-financial-stability-warning-november-2026 仅作线索 | - | - | 未找到 Reuters 一手 |
| C3 | 媒体转述时刻 | L4 | Bloomberg Law https://news.bloomberglaw.com/tech-and-telecom-law/feds-barr-says-ai-agents-could-fuel-future-bank-runs （`2026-11-16T10:00:00Z`，北京 18:00）；Quartz https://qz.com/fed-barr-ai-agents-bank-runs-deposits-111626 （`2026-11-16T14:07:48Z`，北京 22:07；取页 403，Cloudflare 质询，**blocked**，仅搜索摘录） | Quartz 摘录：“according to Bloomberg” | 引 Bloomberg 转述 | 未存档全文 |
| C4 | 华为云 CloudRobo-WM1.2 具身世界模型发布并开源 | L1（官方页）+ L4 | 发布会官方直播页 https://www.huaweicloud.com/about/live/261116.html 存档 `c-c4-live.*`（**发布前议程页**，未见结果）；产品页 https://www.huaweicloud.com/ai/cloudrobo.html 存档 `c-c4-cloudrobo.*` | 直播页议程：“14:00-14:20 CloudRobo-WM1.2具身世界模型发布”；“14:50-14:55 华为云CloudRobo-WM1.2 世界模型正式发布暨开源” | 发布会 2026 年 11 月 16 日 14:00–15:30（北京）。IT之家 报道 “2026/11/16 14:46:23”（L4，存档 `c-c4-ithome.*`） | 已找到（发布与开源）；见下 |
| C4 | WM1.2 在 RoboTwin 2.0、LiberoPro 2.0 两个基准“均排名第一” | L4（IT之家转述官方） | IT之家 https://www.ithome.com/1/027/684.htm 存档 `c-c4-ithome.*` | “在 RoboTwin 2.0、LiberoPro 2.0 两个评测基准中均排名第一。” | 官方发布稿（华为云官方页**未见**发布后的稿件）；**未见具体成功率与测试条件** | 部分支持（排名已由 L4 转述；数字未找到一手） |
| C4 | 98.35% 成功率 | L1（官方页，但**属其他模型**） | 产品页 `c-c4-cloudrobo.txt`：“CR-VLA-Dual … 在Libero上可达到98.35%的成功率” | “华为CloudRobo团队研发，由快慢双系统(Dual System)组成……在Libero上可达到98.35%的成功率” | 这是 **CR-VLA-Dual** 在 Libero 上的数字，不是 WM1.2 | 与说法不符（见勘误） |
| C4 | 88.98%（RoboTwin 2.0）与 81.7%（LIBERO-Pro 2.0）平均分 | L4 | TPS https://tpsreport.news/news/huawei-cloud-cloudrobo-wm1-2-world-action-model-open-source 存档 `c-c4-tps.*`（2026-11-16T07:20:51Z，北京 15:20） | “Slides shown at the launch put CloudRobo-WM's average score at 88.98% on RoboTwin 2.0 and 81.7% on LIBERO-Pro 2.0.” | 转述发布会幻灯；TPS 注明“未独立核实哪个模型版本产生排行榜成绩”；**官方一手未找到** | 未找到一手（仅 L4 幻灯转述） |
| C4 | 发布会后的华为云官方稿 | - | 搜索未找到发布后的华为云官方新闻稿或公众号原文（仅直播议程页） | - | 财联社 16:49（北京）与凤凰网 15:04（北京）为转述 | 未找到发布后一手 |
| C5 | 第 63 届韩国税务师考试水平状元自白 AI 作弊，并向警方自首 | L4（韩国媒体）；主管机关消息见下 | BBC Korean https://www.bbc.com/korean/articles/cmy49g0jy4g8o 存档 `c-c5-bbc.*`（页面“게재 시간 2026년 11월 15일”）；Yonhap https://www.yna.co.kr/view/MYH20261116020700038 存档 `c-c5-yna.*`（2026-11-16T14:56:11Z，北京 22:56）；Korea Herald https://www.koreaherald.com/article/10937374 存档 `c-c5-koreaherald.*`；Korea Herald 2 https://www.koreaherald.com/article/10937594 存档 `c-c5-koreaherald2.*`；전자신문 https://www.etnews.com/20261115000135 存档 `c-c5-etnews.*`（公团入场通告） | BBC：“제63회 세무사 시험에서 수석 합격을 한 A 씨는 15일 유튜브 채널 ... 에 출연해 ‘세무사 1·2차 시험에서 전자기기와 생성형 AI를 이용해 부정행위를 했다’고 밝혔다.” | 自白视频日期：BBC/Korea Herald 写 **11 月 15 日**（Korea Herald “posted Sunday”）；머니투데이 写 **14 日**（见勘误）。警方：서울경찰청 광역수사단受理；Yonhap：경찰 최早 17일 소환 | 已找到（自白、自首、警方受理）；处理结果未定 |
| C5 | 作弊方法（限定） | L4 | Korea Herald 2（同上）；Yonhap（同上） | Korea Herald 2：“photographed exam questions using a miniature camera, sent the images to an AI tool to obtain answers and checked the results on his smartphone” | BBC 韩文原句：“사용한 기기 중 일부는 파손됐지만, 남아 있는 장비와 증거자료를 수사 기관에 제출할 예정이라고 밝혔다。”；Yonhap：自首书写“通过与智能手机相连的小型显示器确认答案”；**其显示器型号未确认**。他否认使用 AI 智能眼镜（Korea Herald 1；etnews） | 已找到（自白口径）；“AI 眼镜”不符（见勘误） |
| C5 | 处理状态 | L4 | etnews（同上）：“한국산업인력공단은 해당 수험자를 업무방해 혐의로 고소하고 재정경제부·국세청과 협의해 최종 합격 취소를 추진한다고 15일 밝혔다。” Korea Herald 2：“The Ministry of Economy and Finance is also seeking to suspend the validity of [当事人]'s qualification certificate” | 同上 | 公团决定“고소”“합격 취소 추진”，财经部“중지 검토”；**均未决定**；警方“调查中”；公团“考虑刑事告诉” | 已找到（过程中，未定） |
| C5 | 主管机关一手（HRDK / 警方官网） | L1 | 未检索到公团或警方官网原文；仅 L4 报道转述其通报（etnews、Korea Herald 2、SBS https://news.sbs.co.kr/amp/news.amp?news_id=N1008846483 存档未做） | - | - | 未找到一手（L4 转述） |
| C6 | Xena 博客文章“Interpreting claims that a theorem is correct because it was checked in Lean”（解读“定理经 Lean 检查”的说法） | L1（作者自有博客，观点文） | https://xenaproject.wordpress.com/2026/11/12/and-the-proof-is-checked-in-lean/ 存档 `c-c6-xena.html` / `.txt`；日文/中文 `?lang=zh` 版存档 `c-c6-xena-ko.*` 未用 | 核心论点：“The summary is: there is an important signal that can be extracted from Lean code, but if this code is AI-generated then you need to do some tests first.” | 署名 “xenaproject”，Posted on November 12, 2026（**原站无时刻**）；页面 “About xenaproject”：博客目标为训练帝国理工（Imperial College）等地本科生形式化数学 | 已找到（作者以博客署名，未标个人姓名） |
| C6 | 是否点名谷歌 Weil 猜想形式化 | - | 全文检索 `Weil` 无命中 | - | 文中提及 OpenAI theorem dump（722 篇论文）、Anthropic 形式化 FLT（13 百万行代码）、Navier–Stokes 论文等，**未点名 Weil 猜想** | 未找到（文中无此项） |
| C6 | HN 热度 | L6（社区） | HN 条目 `50515058` https://news.ycombinator.com/item?id=50515058 ；`c-hn-50515058.json`（Algolia，北京 2026-11-16 15:38 提交） | - | 北京 11-16 23:59：**162 分，嵌套评论 59 条**（`c-heat-c6-item.json`） | 热度见左 |

## 热度（`barking heat`）

- C6：HN 50515058，北京 11-16 23:59 为 162 分、59 条嵌套评论（`c-heat-c6-item.json`，`c-hn-50515058.json`）。
- C1–C5：HN 搜索 “Apple Health AI Usage Study”、“OpenEvidence Apple”、“Barr AI agents retail customer accounts”、“CloudRobo”、“Vujcic AI financial stability”，均 **零命中**（北京 11-16 23:59:42 起 7 日内）。Reddit 未查（上期同类搜索被重定向到登录页，本期未重试）。

## 扫描说法勘误

1. **C1 Apple 时刻**：“北京 11-15 00:00”无一手支持。Apple 只写 “November 15, 2026”（未给时刻）；最早的英文媒体时间戳为 TechCrunch 北京 11-16 09:30。
2. **C1 TechCrunch 与机型**：扫描称 TechCrunch 写“iPhone 15 及以上”，属实；但 Apple 原文为 “iPhone 15 Pro or later”，The Verge 写 “iPhone 15 Pro”。正文应以 Apple 为准。
3. **C1 “约 12 个月”与“发给第三方云前匿名化”**：仅 TechCrunch（L4）一家写，Apple 官方页没有。正文若用，须署 “据 TechCrunch 报道”。
4. **C2 “5.5 万亿”的口径**：ECB 原文为“主要美国科技企业累计资本开支到 2030 年预计达 5.5 万亿美元”（资本开支，预测，未署出处）。Reuters 转述为 “financing need”（融资需求）。两者不同，正文写“资本开支”。
5. **C3 数字的限定**：“七家 AI 企业 $800 billion”原文是 “expected to exceed ... before year's end”（预计）；“$3 trillion”原文是 FSR 估计的 “potential maximum nominal value”（潜在最大名义值），非已发生债务。
6. **C3 “挤兑”句**：原文为 “may accelerate” / “could” / “potentially”，是情景假设，并非已观察到的挤兑。
7. **C4 “98.35%”**：扫描称来自会前中文稿（属 WM1.2）。核实：华为云产品页中 98.35% 出现在 **CR-VLA-Dual** 的 Libero 成功率句中，不是 WM1.2。WM1.2 的一手官方文本只见“两个基准均排名第一”，未见数字。
8. **C4 “88.98% / 81.7%”**：仅见 TPS（L4）转述发布会幻灯，TPS 本人注明未核实模型版本，不宜写成官方数字。
9. **C5 作弊方式“AI 智能眼镜”**：中文媒体标题（如中央日报中文网）写“AI 智能眼镜作弊”，但自白原文为“小型摄像头拍题 + AI + 智能手机”（Yonhap 另加“小型显示器”）。他否认使用 AI 眼镜。这是早期指控，不是自白内容。
10. **C5 视频日期**：BBC 与 Korea Herald 写 11 月 15 日（Korea Herald 称 “posted Sunday”）；머니투데이 写“14일”。未能裁定，正文若写日期须注明或只写“周末”。
11. **C5 处理状态**：自首书 11 月 15 日上午已被首尔警察厅受理并分配广域搜查队，警方拟 17 日传唤（Yonhap “이르면 17일”）；公团决定告诉、推进合格取消；财经部“检讨中止合格证书效力”。均未决定，不宜写成“已取消”。
12. **C6 HN 热度**：扫描 159 分 / 57 评为较早快照；北京 11-16 23:59 为 162 分 / 59 嵌套评论。非错误，但按时间标注。

## 待定（主要缺口）

- C3 Reuters 一手页未取（web 检索未返回 Reuters 原文；经 casrai 转述，仅作线索）；Quartz 页 403，未存档全文（仅搜索摘录）。
- C4 华为云发布后的官方稿未找到（只有发布前议程页与产品页）。
- C5 公团 HRDK 与警方官网原文未取到，只有 L4 报道转述。
- C1 Verge、C2 Reuters（经 Euronext 转载仅搜索摘录）未全文存档。

## 假设

- 北京时间按 UTC+8 换算；Yonhap、Korea Herald 时间戳来自页面或搜索元数据，未核对页面本身的显示时区。
- 自然人姓名（C5 当事人）：内部存档中报道原文保留了其姓名，本文件一律写“A 씨”。
