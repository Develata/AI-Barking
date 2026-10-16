# 1015 期 A 组取证：商务部、科技部公告 2026 年第 48 号（拟修订《中国禁止出口限制出口技术目录》，AI 23 项）

执行方：claude-sonnet-5-5。取证与存档时刻见 `capture-log-a.md`（北京时间，抓取日 2026-10-16 约 19:40–21:10）。官方页面一律以 `a-` 前缀存档于本目录。本组只记官方原文与答问，不评价政策；卡片与标题请写“商务部、科技部”或“两部门”，不写“政府”。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

## 0. 结论摘要（已核 / 推断 / 未核）

- 已核（官方原文）：公告号、发布时刻（页面 2026-10-16 16:00，官方页只给分钟）、40 项 / AI 23 项 / 生物医药 5 项 / 光伏 12 项、禁止 14 项 / 限制 26 项（答记者问）、征求意见期 10 月 16 日至 11 月 15 日、临时登记自公告发布之日起即生效、API 等不属于大模型技术出口管理范围的原话。
- **附件 1 的 AI 部分（已读原件，LibreOffice 渲染 + 自写脚本提取文本，两路一致）：23 项 = 禁止 6 项 + 限制 17 项。**附件表格实际只有 22 行（AI 11 行、生物医药 5 行、光伏 6 行）；“项”按行内编号的控制要点（含“在现有 N 项基础上增加”的新增条）计数时，AI 23（6+17）、生物医药 5（3+2）、光伏 12（5+7）、合计 40（禁止 14 + 限制 26）与公告、答问的数字全部吻合。这个计数口径是我据附件自己推出的，官方未写明（推断，但四个总数同时吻合）。
- 因此“14 项拟禁止”里只有 6 项属 AI（其余生物医药 3、光伏 5）；AI 部分以限制类为主（17/23）。
- 答问中“API 调用、APP/网页交互、SaaS 应用、本地端侧轻量化部署不属于大模型技术出口管理范围”有两个前置限定：①针对“限制类大模型”；②问题问的是“《目录》正式实施后”。
- 官方公告与答问均是“拟”：征求意见稿，目录尚未正式修订；已生效的只有“临时登记管理”。

## 1. 逐条证据表

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1-a 文号、发布单位 | L1 | https://www.mofcom.gov.cn/zcfb/dwmygl/art/2026/art_61446ab4beee4f63973d4d0ab3097648.html （同文另有路径 https://www.mofcom.gov.cn/zwgk/zcfb/art/2026/art_0a31a3c9ac404f7a9b0e917ce3412997.html，Bloomberg 引用的是这一条；存档 `a-cl-bloomberg-url.*`，内容相同） | “商务部 科技部公告2026年第48号 就修订《中国禁止出口限制出口技术目录》公开征求意见并对相关技术出口实施临时登记管理”；“【发布单位】安全与管制局”“【发布文号】商务部 科技部2026年第48号”“【发文日期】2026年10月16日” | images/01 | 页面时间 2026-10-16 16:00（北京时间，官方页只给到分钟，非公告生效时刻）；公告落款 2026年10月16日 | 已找到 |
| A1-b 拟修订、40 项、AI 23 项、公共领域豁免 | L1 | 同上 | “商务部、科技部拟修订《中国禁止出口限制出口技术目录》，增列部分人工智能、生物医药、光伏技术。经批准，现向社会公开征求意见，并同步对相关技术实施出口前登记管理。”“此次公开征求意见涉及人工智能、生物医药、光伏三个领域40项技术。人工智能领域包括大模型、智能体、算力集群、信息处理相关23项技术；对已通过开源、发表学术论文等方式进入公共领域的技术，不纳入管理范围。生物医药领域包括细胞和基因治疗、小核酸药、病原微生物相关5项技术；对不涉及所有权转让的对外授权模式，适用自动许可等便利措施。光伏领域包括多晶硅制造、硅片制造、电池制造相关12项技术；技术管理范围也包含承载相关技术的关键和核心设备。” | images/01 | “拟”“公开征求意见”：征求意见稿，非正式生效 | 已找到 |
| A1-c 征求意见期 | L1 | 同上 | “公众可在2026年10月16日-11月15日期间，通过以下途径，就新增技术范围、指标、管控政策、便利措施等提出意见：”（途径：电子邮件、传真、信函） | images/02 | 起止日为日期，官方未给时刻 | 已找到 |
| A1-d 临时登记即日生效 | L1 | 同上 | “（一）自本公告发布之日起，至《中国禁止出口限制出口技术目录》正式修订并实施之日止，技术出口经营者在出口本公告附件所列技术前，应当在商务部业务系统统一平台技术贸易管理信息应用（https://ecomp.mofcom.gov.cn）登记。已经开始但尚未完成的出口，在登记范围内。” | images/02 | 自发布之日起至正式修订实施之日止；登记义务对象是“技术出口经营者”，对象是公告“附件所列技术” | 已找到 |
| A1-e 登记情形四类 | L1 | 同上 | “（二）需开展登记的出口情形包括：1.向境外转让技术所有权，如知识产权转让；2.境内技术授权境外使用，如知识产权授权使用；3.境内技术的所有权人或者实际控制人变更为境外主体；4.商务部以通知等形式明确要求登记的其他情形。” | images/02 | 答问补充：“出口经营者应自行判断拟出口技术是否落入临时登记管理范围。” | 已找到 |
| A2-a 40 / 14 / 26 | L1 | 答记者问 https://www.mofcom.gov.cn/zwgk/zcjd/art/2026/art_b11d5c1e9374421e93e85dc87eba6e38.html （另存同文 https://www.mofcom.gov.cn/syxwfb/art/2026/art_a4acff07e134447fbe7bbeb486df3ea6.html） | “此次公开征求意见涉及拟增列的人工智能、生物医药、光伏三个领域40项关键技术。其中，拟禁止出口技术14项，包括：人工智能驱动的恶意代码生成与变异技术、具有自主知识产权的细胞和基因治疗技术工艺平台、钙钛矿及叠层电池组件生产技术等；拟限制出口技术26项，包括：人工智能辅助漏洞挖掘技术、具有自主知识产权的通用型细胞和基因治疗全新靶点发现和验证技术、单线产能≥3万吨/年的多晶硅工程技术等。” | images/04 | 14、26 是三个领域合计，答问未给分领域数 | 已找到 |
| A2-b AI 23 项的禁止/限制分类（附件 1） | L1 | 公告附件 1 `.wps`（官方下载链接见 `capture-log-a.md`）；存档 `a-annex1.wps`、转 PDF `a-annex1.pdf`、文本 `a-annex1.txt`（自写脚本）、`a-annex1-pdftotext.txt` | 表头“一、人工智能”“禁止出口部分”“限制出口部分”；见下文第 2 节逐项表 | images/05、06 | AI：禁止 6 + 限制 17 = 23；生物医药 3+2 = 5；光伏 5+7 = 12；合计 14+26 = 40。计数口径见第 2 节 | 已找到（计数口径为推断，总数全吻合） |
| A2-c 技术指标代表性原句 | L1 | 同 A2-b | 见第 2 节“代表性指标” | images/06 | 全部是“具有以下任一特性”或“满足以下指标”形式，须逐条看条件 | 已找到 |
| A3 API/APP/SaaS/端侧不属于；海外本地化部署属于；海外自有云可适用自动许可 | L1 | 答记者问（同 A2-a 的 URL） | 问：“《目录》正式实施后，人工智能大模型在海外本地化部署、向境外提供模型调用等各类“出海”形式，是否属于技术出口管理范围？”答：“当前，人工智能大模型和应用产品的“出海”模式多种多样，判别具体“出海”情形是否属于技术出口管理的范围，核心在于是否涉及限制类技术从中国境内向境外转移。”“此次纳入监管的人工智能技术范围精准有限，有关措施正式实施后，绝大部分人工智能技术仍可以自由出口。对于纳入限制类的人工智能技术，开展海外本地化部署，属于技术出口管理范围；如果部署在海外自有云等风险相对可控的情形下，可适用自动许可等便利措施。对于企业利用限制类大模型对外提供API调用、APP/网页交互、SaaS应用、本地端侧轻量化部署等情形，不属于大模型技术出口管理范围。” | images/03 | **限定：①“限制类大模型”；②“《目录》正式实施后”（现在是征求意见稿）；③“绝大部分……仍可以自由出口”是“有关措施正式实施后”的预期** | 已找到 |
| A3-补 “不针对任何特定国家、地区或实体” | L1 | 答记者问 | “此次修订亦是技术贸易管理的常态化工作，依法依规进行，不针对任何特定国家、地区或实体。” | 无（同页另截 images/04 为 40/14/26 一问） | | 已找到 |
| A4 公共领域豁免 | L1 | 公告与附件 | 公告：“对已通过开源、发表学术论文等方式进入公共领域的技术，不纳入管理范围。”附件 1 PDF 渲染第 1 页脚注 1：“已通过开源、发表学术论文等方式进入公共领域的技术，不纳入管理范围。”答问：“对已通过开源、发表学术论文等方式进入公共领域的相关技术，拟不纳入出口管理范围”（“拟”） | images/01、05 | 公告写“不纳入”，答问写“拟不纳入”；脚注 2（附件 PDF 第 2 页）：“对本类技术开源，以及中国企业在境外自建自营算力中心、赴境外开展芯片流片制造、利用境外资源开展联合研发、参与国际标准制定等情形的技术出口，适用自动许可等便利措施。”答问称这类情形“基本实现‘即报即出’” | 已找到 |
| A5 此前《目录》修订 | L1 | 答问 2026 “自2001年以来”；2025 年答问 https://www.mofcom.gov.cn/syxwfb/art/2025/art_072f1b88bcb4415fae1f05bbd569fd50.html；公告 2025 年第 28 号 https://www.mofcom.gov.cn/zcfb/zgdwjjmywg/art/2025/art_67e2a41850ec428eb1b7be6ec2f2bded.html；公告 2023 年第 57 号 https://www.mofcom.gov.cn/zfxxgk/fdzdgknr/ztfl/fwmy/art/2023/art_997fc46a1b34414bbebc597cb6f76581.html；公告 2020 年第 38 号 https://policy.mofcom.gov.cn/claw/clawContent.shtml?id=87380 | 2026 答问：“自2001年以来，商务部会同科技部多次调整修订《目录》，有进有出，有管有放，已形成工作惯例。”2025 答问：“2001年，商务部会同科技部以部令形式发布《目录》，并分别于2008年、2020年和2023年对部分内容进行调整、修订。”最近三次：2025-07-15（公告2025年第28号，“删除3项技术条目，新增1项，修改1项”，涉及锂提取等）；2023-12-21（公告2023年第57号，公布整本《目录》，“公告2020年第38号……同时废止”）；2020-08-28（公告2020年第38号） | 无 | 2025 年答问是“调整”，不是 AI 专项。**现行《目录》里已有 AI 相关条目**：附件 1 自己写“在此类技术现有7项基础上增加”（266402X）、“现有5项基础上增加”（266404X）、“现有18项基础上增加”（266501X），即本次是在已有条目上追加；Global Times 称 2008、2020、2023、2025 有修订（二手，与官方答问一致） | 已找到 |
| A6 媒体 | L4 | 见第 3 节 | 见第 3 节 | 无 | | 已找到（5 家） |
| A7 AIHOT 热度 | L5（线索） | https://aihot.news 与 https://aihot.news/items/p11tj593iq5nrtiotvoxllop4 | 条目标题“商务部、科技部拟在出口技术目录增列23项人工智能技术并征意见”；来源“IT之家·人工智能”，条目页“发布时间 2026-10-16 16:23”，列表页显示“16:44”，标“精选”，“AI 评分 83” | 无 | 评分 83 与扫描说法“约 83”一致；“发布后不到一小时进入精选”：官方 16:00，IT之家 16:23，精选列表 16:44（页面时区未注明，按北京时间读）。AIHOT 是聚合站，只作热度线索，且转述的是 IT之家 | 已找到（热度） |
| A7 其他热度 | — | HN：`barking heat` 与 Algolia 查询 “China export controls AI / MOFCOM technology export catalogue / China AI export ban” 在 2026-10-16 15:00 北京时间后无命中；Reddit 搜索被重定向到登录页（失败）；微博、知乎未查 | 无 | 无 | 抓取时刻 2026-10-16 约 20:10 北京时间，HN 无相关主帖 | 未找到（社区热度） |

## 2. 附件 1：AI 部分 23 项（逐项）

来源：`a-annex1.wps`（官方下载，OLE2 Word97 格式）。两路读取：①自写 `a-wps2txt.py` 提取 WordDocument 流文本；②LibreOffice 26.8.1 转 PDF（`a-annex1.pdf`，9 页）后 `pdftotext`。两路条目一致。注意：PDF 是 LibreOffice 对 `.wps` 的渲染，版式非官方原貌；纯文本里“10^26”的上标丢失（写成“1026”），**以渲染页为准（images/06 可见“≥10^26 次浮点运算”）**。

计数口径：附件表格按“序号”只有 AI 11 行；公告与答问说“23 项”。逐行数行内编号的控制要点（禁止类各条无编号则按一条；限制类按“在现有 N 项基础上增加”之后的新增编号）才得到 23。（推断，但与 14/26/12/5/40 同时吻合。）

**禁止出口部分（6 项，4 行）**

| 序号/编号 | 技术名称 | 项 | 控制要点（摘） |
|---|---|---|---|
| 1 / 266401J | 高性能检测技术 | 1 | 专门或者主要用于我国网络数据跨境传输安全风险防范和处置的鉴伪系统及设备制造技术，深度合成视频、音频、图像或者多模态信息的检测准确率和召回率≥95%。 |
| 2 / 266402J | 信息对抗技术 | 3 | 1.“人工智能自主驱动恶意代码生成与变异技术”（对多引擎恶意软件检测器攻击成功率≥60%；对单一主流防病毒软件≥85%；对机器学习模型≥90%，具备任一）；2.“对抗环境自适应演进与运行时动态行为修改技术”；3.“自动化免杀变种批量生成与分布式规模化部署技术”。三条均限定“可被用于攻击国家关键信息基础设施、国防军工、核心政务等高敏感系统，或用于绕过国家安全防御体系” |
| 3 / 266501J | 人工智能大模型技术 | 1 | 利用我国核心数据进行训练后提供服务的大模型。注：“核心数据”范围由《中华人民共和国数据安全法》等相关法律法规确定。 |
| 4 / 266502J | 人工智能体技术 | 1 | 专门或者主要用于国家关键信息基础设施、国防军工、核心政务等高敏感场景应用，达到高性能水平的智能体。注：“高性能”指达到266505X限制类人工智能体技术的管控指标。 |

**限制出口部分（17 项，7 行）**

| 序号/编号 | 技术名称 | 新增项 | 控制要点（摘） |
|---|---|---|---|
| 5 / 266402X | 高性能检测技术（在现有 7 项基础上增加） | 8、9 | 8.深度合成视频、音频、图像或者多模态信息的检测系统及设备制造技术，准确率和召回率≥98%；9.视频、图像信息隐写与提取技术（高鲁棒性、低虚警率、高不可感知性，需同时满足） |
| 6 / 266404X | 信息对抗技术（在现有 5 项基础上增加） | 6 | “具有以下任一特性的人工智能辅助漏洞挖掘技术”：（1）权威安全基准测试中漏洞复现或可利用性判定得分≥75%；（2）端到端利用链成功率（主流操作系统或浏览器≥50%，或任意代码执行级别≥40%）；（3）较资深安全专家基准实现10倍及以上效率提升等；（4）自主将2个及以上低危或中危缺陷串联为完整多阶段攻击链并生成可运行概念验证代码；（5）一次性自主发现5个以上先前未披露的高危缺陷或50个以上先前未披露缺陷；（6）达到主流漏洞利用能力分级体系最高级别 |
| 7 / 266501X | 信息处理技术（在现有 18 项基础上增加） | 19、20 | 19.语音声纹识别技术（指标：假正例率≤0.01% 时真正例率≥99.1% 等，具备任一）；20.专门或者主要用于汉语及少数民族语言的翻译技术（平均意见得分 MOS≥4.8；或语义忠实度≥95%时 BLEU≥35/≥30） |
| 8 / 266504X | 人工智能大模型技术 | 1、2 | 1.“利用我国重要数据进行训练后提供服务的大模型。”2.“具有以下任一特性的大模型的核心技术，包括权重参数、关键检查点、底层源代码、架构设计，以及训练优化、推理加速等技术：（1）计算量≥10^26次浮点运算；（2）参数量≥20,000亿；（3）能力达到全球领先水平。”注：①仅使用微量（≤0.1%）重要数据训练……不纳入第1项技术的管理范围；③“能力达到全球领先水平”指出口活动发生时，国内外主流测评机构、大规模产业平台等，针对大模型综合性能、专项业务能力、全球调用量、安全性能、训练效率等公开评估全球排名前10。 |
| 9 / 266505X | 人工智能体技术 | 1–4 | 自主规划与任务分解（≥20步子任务链、分解精度≥80%、平均成功率≥90%）；多智能体协同（并发≥60个等）；工具调用与系统操作（集成第三方工具≥200个等）；“具有以下所有特性的人工智能博弈决策技术” |
| 10 / 266506X | 人工智能芯片设计技术 | 1、2 | 1.AI 处理芯片设计技术（含任一：14纳米及以下工艺、2.5D/3D 封装、存算一体等）；2.存储芯片设计技术（3D 闪存、内存芯片） |
| 11 / 266507X | 人工智能算力集群技术 | 1–4 | 1.片间或节点间高速互连双向带宽≥800GB/s；2.分布式训练容错；3.高效通信调度；4.异构集群推理与调度。注：“大规模智能算力集群”指人工智能加速卡总数超过10,000张的运算主体。 |

可用的短口径（均可由上表核对）：AI 23 项里禁止 6、限制 17；14 项禁止里 AI 占 6；大模型条目只有两行：禁止类 = 核心数据训练的大模型，限制类 = 重要数据训练的大模型，或满足（计算量≥10^26 次、参数量≥20,000 亿、公开评估全球前 10）任一的大模型核心技术（权重、检查点、源码、架构、训练优化、推理加速）。

其他两领域（只为核对总数）：生物医药 5 项（禁止 3：细胞和基因治疗技术工艺平台、小核酸药技术工艺平台、病原微生物技术；限制 2：两类“全新靶点发现和验证技术”）；光伏 12 项（禁止 5、限制 7）。光伏附注称管理范围含承载技术的关键和核心设备。

## 3. 媒体报道（L4，只作线索与对照；全部已存档）

| 媒体 / 发布方 | 时刻（北京时间；括号为原时区） | URL | 摘句（逐字，原语言） | 与官方原文的差异 |
|---|---|---|---|---|
| Bloomberg（首发之一） | 2026-10-16 16:46:44（08:46:44 UTC；Bloomberg Law 版 10:48 UTC 更新） | https://www.bloomberg.com/news/articles/2026-10-16/china-proposes-export-controls-on-ai-models-chips-and-agents ；同稿 http://www.theedgemarkets.com/node/822148 | 标题（页面）“China Plans AI Export Curbs to Exert Control Over Key Tech”；“The draft proposal covers 23 AI technologies, including large language models, agents, chip designs and computing clusters. Restricted technologies would include model weights and source code for products exceeding certain performance thresholds.”；“The proposal would restrict exports of core technology for models containing at least two trillion parameters — like Kimi K3 or Alibaba Group Holding Ltd’s Qwen3.8-Max — or ranking among the world’s top 10 on measures including performance, usage and safety.”；“AI models trained on China’s “core data”, as defined by relevant legislation such as the Data Security Law, will be banned from export, according to the proposal.” | ①附件无“Kimi K3 / Qwen3.8-Max”字样，是 Bloomberg 自己的举例；②只提参数与排名，没提计算量≥10^26 一条（附件三条件为“任一”）；③“排名前 10”是“公开评估全球排名前10”，Bloomberg 写成“performance, usage and safety”范围略窄；④“will be banned”缺少“拟”（但前文称 draft） |
| Dow Jones（Morningstar 转载） | 2026-10-16 18:20（05:20 EST-5，页面标记 -05:00） | https://www.morningstar.com/news/dow-jones/202610162068/china-proposes-export-controls-on-ai-technologies | “The proposed changes cover 23 AI-related technologies such as large language models, AI agents, chip design and computing clusters.”；“In response to a question on how the proposed export controls would affect Chinese AI companies seeking to expand internationally, the ministry spokesperson said that would hinge on “whether it involves the transfer of restricted technologies from within China to locations outside its borders.””；“The registration, which started Friday…” | 基本一致；未转述 API/SaaS 那句 |
| AFP（Economic Times 转载） | 2026-10-16 19:00（16:30 IST） | https://m.economictimes.com/tech/artificial-intelligence/china-eyes-curbs-on-ai-biotech-solar-exports/amp_articleshow/134992328.cms | “The draft proposal released by the Chinese commerce ministry on Friday would ban exports of 14 technologies, including AI-driven malicious code generation and mutation technology, as well as AI models trained on Chinese core data.” ；ET 摘要（Synopsis）：“Additionally, it seeks to limit exports of 26 other techs with potential hacking capabilities” | 正文基本一致；ET 摘要把 26 项限制类说成“with potential hacking capabilities”：26 项跨 AI、生物医药、光伏，“黑客能力”只对应其中 AI 辅助漏洞挖掘一条 |
| The Hindu | 2026-10-16 18:31（16:01 IST） | https://www.thehindu.com/news/international/china-considers-new-export-curbs-on-solar-ai-biotech/article71591880.ece | “It covers 40 key technologies in the fields of AI, biomedicine and photovoltaics. Of these, 14 technologies were proposed for a complete prohibition of export, including perovskite and tandem solar-cell module manufacturing technologies, AI-driven technology that could be used to generate and mutate malicious code, and some platforms related to cell and gene therapy.”；“…most AI technologies will continue to be freely exportable…” | 一致；清楚写出 14 项分属光伏、AI、生物医药 |
| Global Times | 2026-10-16 17:17（北京时间，页面标注） | https://www.globaltimes.cn/page/202610/1372405.shtml | “features 14 additions to the prohibition category and 26 additions to the restricted category. By sectors, 23 additions are related to AI, 12 additions to PV and 5 additions are related to biomedicine.”；“The catalogue, first published in 2001, had previous revisions in 2008, 2020, 2023 and 2025.” | 与官方一致；把 AI 大模型禁止项写作“trained with national key sensitive data”（附件为“核心数据”） |
| 钜亨网（编译 Bloomberg） | 2026-10-16 17:50（09:50 UTC） | https://m.cnyes.com/news/id/6632207 | “本次草案明確劃定管制門檻，主要針對超大規模或高效能的頂尖模型。具體規範包含參數規模達2兆以上(如月之暗面 Moonshot AI 的 Kimi K3 或阿里巴巴的 Qwen3.8-Max)”；摘要第 3 点“使用國家核心資料訓練的AI模型禁止出口，過渡期出口須前置登記” | 草案原文无具体模型名，是转述 Bloomberg 的举例；“过渡期”指的是临时登记，登记对象是所有权转让、授权境外使用等四类情形 |
| 联合早报、新华网客户端（答问全文）、SCMP | — | https://www.zaobao.com.sg/news/china/story20261016-9848338 ；https://app.xinhuanet.com/news/article.html?articleId=202610162979db6d1d5641d3b5ec093b55edc7be ；SCMP 被 Cloudflare 质询拦截（见 capture-log-a） | 联合早报与新华网转述公告/答问原文 | 无差异；SCMP 未取得正文，仅见标题“China mulls new tech export bans covering AI, biotech and solar power”（搜索摘要）及副标题“The draft plan would cover 14 technologies ranging from artificial intelligence to photovoltaic solar power” |

首发排序：官方页 16:00，IT之家约 16:23（AIHOT 条目页），Bloomberg 16:46，Global Times 17:17，钜亨 17:50；路透社、第一财经首发稿未取得（见未完成项）。

## 4. 可能的吠点（原文里有、线索摘要可能没提或说反的限定；每条一句，注出处）

1. 全部是“拟”：公告标题就是“公开征求意见”，目录未正式修订；生效的只有临时登记管理（公告三（一）、答问最后一问）。
2. 14 项“拟禁止”分属三个领域，AI 只占 6 项（附件 1 逐行数得；官方未写分领域禁止/限制数，计数口径为推断）。
3. 禁止类 AI 的“恶意代码”三条均带“可被用于攻击国家关键信息基础设施、国防军工、核心政务等高敏感系统，或用于绕过国家安全防御体系”的用途限定与攻击成功率指标（附件 1，266402J）。
4. 禁止类大模型 = “利用我国核心数据进行训练后提供服务的大模型”，限制类 = “重要数据”训练，或满足计算量≥10^26、参数量≥20,000亿、公开评估全球前 10 中任一（附件 1，266501J/266504X）；第 1 项另有“仅使用微量（≤0.1%）重要数据”等不纳入情形（注①）。
5. 限制的是“核心技术，包括权重参数、关键检查点、底层源代码、架构设计，以及训练优化、推理加速等技术”，不是“模型能力/产品”本身（附件 1，266504X；答问“利用相关技术制造的产品不在管理范围内”）。
6. “API/APP/SaaS/端侧轻量化部署不属于管理范围”只针对“限制类大模型”，且是“正式实施后”的答复；同一段话明说“开展海外本地化部署，属于技术出口管理范围”（答问）。
7. 登记义务是“技术出口经营者”的，且登记情形限于四类（转让所有权、授权境外使用、所有权人或实际控制人变更为境外主体、商务部另行要求）；“出口经营者应自行判断拟出口技术是否落入临时登记管理范围”，“已经开始但尚未完成的出口，在登记范围内”（公告三、答问）。
8. 开源与论文发表的豁免：公告写“不纳入管理范围”，答问写“拟不纳入”；对 AI 开源、境外自建自营算力中心等情形另有“自动许可”便利，答问称“基本实现‘即报即出’”（脚注 2、答问）。
9. 现行《目录》已含 AI 条目，本次是增列（附件写“在此类技术现有7项/5项/18项基础上增加”）。
10. 社区质疑：本组未找到 HN、Reddit 相关主帖（见 A7），无评论可摘。

## 5. 夸大说法实例

只找到轻度的，未找到“把拟写成已禁止”或“14 项 AI 技术禁止出口”或“API 出海受限”的标题：

1. Economic Times 转载 AFP 的页面摘要，2026-10-16 19:00（16:30 IST），https://m.economictimes.com/tech/artificial-intelligence/china-eyes-curbs-on-ai-biotech-solar-exports/amp_articleshow/134992328.cms ：“it seeks to limit exports of 26 other techs with potential hacking capabilities”（26 项跨三个领域，不是都与黑客能力有关）。程度：轻。
2. 钜亨网 2026-10-16 17:50，https://m.cnyes.com/news/id/6632207 ：“如月之暗面 Moonshot AI 的 Kimi K3 或阿里巴巴的 Qwen3.8-Max”写进“具体规范”一段，附件没有点名任何模型（出处是 Bloomberg 的举例）。程度：轻到中，易被读成官方点名。
3. Bloomberg 2026-10-16 16:46 北京时间：“AI models trained on China’s “core data”… will be banned from export, according to the proposal.”（缺“拟”，上文有 draft）。程度：轻。
4. 其他中英文标题大多带“拟”或 “proposes/considers/mulls/plans”（Bloomberg Law、Hindu、SCMP、Dow Jones、联合早报），未见成规模的夸大。

## 6. 扫描说法勘误

- “发布后不到一小时进入 AIHOT 精选，评分约 83”：基本属实。AIHOT 显示来源 IT之家 16:23、精选列表 16:44、评分 83；官方页 16:00。注意进入的是 IT之家转述条目，不是官方页。
- 已知情况里的数字与引文全部与原文一致，无需勘误。补充：公告同文有第二个 URL（/zwgk/zcfb/）；答记者问同文有第二个 URL（/syxwfb/）；Bloomberg 引用的是前者。
- “14 项拟禁止里有几项是 AI、23 项里禁止与限制各几项”（派工单的关键核对点）：附件逐行数，AI 禁止 6、限制 17；14 项禁止中 AI 为 6。口径见第 2 节（推断）。

## 7. 未完成项

- A6：路透社、第一财经、财新等首发稿未取得（firecrawl 搜索未返回路透原文；未再扩展）；SCMP 被 Cloudflare 质询拦截，未用 opencli 重取。
- A7：微博、知乎热度未查；HN 无相关主帖、Reddit 被登录页拦截，评论类质疑无可摘。
- 附件 2（技术出口信息登记表）未下载；本组未覆盖。
- 附件 1 渲染截图只出了 AI 部分第 1 页（禁止类开头）与第 3 页（大模型限制类）；未截智能体/算力集群等其他页，可由 `a-annex1.pdf` 补渲（`barking shot … pdf`）。
- 技术指标原句只摘了代表性几条，其余指标未逐条核对。
