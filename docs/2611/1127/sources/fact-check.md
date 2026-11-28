# 1127 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。稿件、论文、帖子的发布时间为北京时间（UTC+8）；美国北卡案情里的日期（邮件、通知、裁定、锁门）沿用当地日历日，报道未给可换算的时刻。取证：A、B 组 Claude Sonnet 5.5、C 组 Claude Haiku 5.5 取证子代理（与主执行方同为 Claude 家族），因此反向核验须逐行核对本清单与 `sources/` 存档。主执行方抽查（11-28 23:40 — 11-29 01:30）：A 组论文 Table 4、Table 9、Table 10、Table 12 原文与截图 02、11，官方头条文章截图 01，CNMO 原句；B 组 WRAL 三篇存档（`b-wral-first.txt`、`b-wral-investigates.txt`、`b-wral-response.txt`）、截图 14、15，Futurism 标题，Moneywise（经 Yahoo）Leach 原话；C 组 C1–C5、C7、C8 存档关键原句。

## 一、小米：世界模型2.0称全面 SOTA（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：小米称视频分第一，同表美观度垫底 | L1 | ✅ | 说法：小米技术官方账号发布稿（`a-weibo-article.txt:74`）：“在PAI-Bench-G评测中，世界模型2.0的驾驶视频生成质量评分为 78.0，机器人视频生成质量评分为 77.1，均为项目主页所列对比方法中的最高分”；实际：技术报告 Table 4、5（截图 02、03）Aesthetic Quality 列，小米驾驶 46.6、机器人 44.8，均为表中 7 个模型最低 | “视频分”= PAI-Bench-G 视频生成质量平均分；“同表”= 同一张 Table 4（驾驶）与 Table 5（机器人）；“美观度”= Aesthetic Quality（正文“画面美观度”）。第四轮反向核验后 Develata 拍板换题，原题“小米称多基准全面最佳，报告一项排第4”的对照范围与官方说法不一致（官方未点名 TriWorldBench），弃 |
| 省流：小米称世界模型2.0视频平均分在所列方法中最高；其技术报告同表里，“画面美观度”驾驶、机器人两项都是7个模型里最低 | L1 | ✅ | 说法：官方稿（`a-weibo-article.txt:74`）PAI-Bench-G 驾驶 78.0、机器人 77.1“均为项目主页所列对比方法中的最高分”；实际：Table 4、5（截图 02、03）AQ 列小米 46.6、44.8 均为 7 个模型最低 | 第五轮反向核验 BLOCKER：原省流以 TriWorldBench 第4 作反证，范围与官方说法不一致，改用与标题同一张表 |
| 世界模型用来预测车或机器人行动后周围怎么变 | L1 | ✅ | 官方稿（`a-weibo-article.txt:16`）：“如果车辆改变行驶轨迹，周围交通环境将如何变化？如果机器人执行一个动作，物体和环境又会发生什么？这正是世界模型要回答的问题。” | 转述官方定义 |
| 11月27日小米技术官方账号发文 | L1 | ✅ | https://weibo.com/ttarticle/p/show?id=2309405369910959407174 ，页面“11-27 17:00 已编辑”（截图 01） | 存档为 11-28 版本；17:00 为页面显示时刻，带“已编辑”，未确认是首发还是编辑时刻，正文只写日期；同时刻官方短帖 `created_at` 为 17:00:03 |
| 称世界模型2.0“在多个主流基准测试中全面取得 SOTA” | L1 | ✅ | 官方稿原句（截图 01） | 论文摘要与正文无 SOTA 自称（A 组全文 grep）；正文引号内按排版规范在 SOTA 前加空格 |
| 即评测成绩最好 | — | ✅ | SOTA（state of the art）的通俗解释 | 本号释义 |
| 对你：发布稿举的落地场景是生成训练数据和仿真测试，没提装车 | L1 | ✅ | 官方稿：“并已在小米汽车合成数据、仿真测试等核心场景完成业务落地应用”；“合成数据生成：已交付超过20万 clips 高质量合成数据，直接用于感知模型/感知基模训练”；官方稿全文 grep“HAD”“车端”“量产”为 0 | 同日小米汽车另发车端 HAD 1.20（含“车端世界模型”），官方未说明与 2.0 的关系，正文不写 |
| 报告和项目页没放代码与模型权重 | L1 | ✅ | `a-project.html`、`a-arxiv-pdf.txt`：项目页外链只有论文 PDF 与前代页，论文无代码/权重/数据链接 | 缺失性结论，范围限论文与项目页；官方未表态是否开源 |
| 吠点②：发布稿称“全面取得 SOTA”，没说明“全面”指哪些基准；报告所列6个方法里，TriWorldBench 机械臂视频综合分小米64.79排第4 | L1 | ✅ | Table 9（截图 11）；`a-arxiv-pdf.txt:1263`：TriWorldBench 为动作条件的具身（机械臂）多视角生成评测；论文自评 “our approach achieves competitive overall performance … and attains the best Trajectory Accuracy”（`:1271`） | 综合分即 TWB-Score；第一名 CausalWM 66.04；小米在 Traj. Acc. 一列最高；对照限“methods with publicly available papers”，分数取自 2026-09-30 官方榜单快照（`a-arxiv-pdf.txt:1269–1270`） |
| 报告表中 LIBERO Plus 机器人操作的加权成功率小米75.0%、GAM 85.5%，GAM 有具身预训练而小米没有 | L1 | ✅ | Table 12（`a-arxiv-pdf.txt:1643–1650`，截图 04）：Avg（weighted average）GAM 85.5、Ours 75.0 | GAM 做过具身预训练（Embodied PT. = Yes），小米为 No；GAM 在论文中无引用文献；项目页副标题自述 “GAM reports the highest average at 85.5%” |
| 吠点①：小米称 PAI-Bench-G 视频平均分驾驶78.0、机器人77.1为所列方法最高；这是中期训练版自测 | L1 | ✅ | 官方稿：“驾驶视频生成质量评分为 78.0，机器人视频生成质量评分为 77.1，均为项目主页所列对比方法中的最高分”；Table 4/5 表注 “AV-Quality denotes the average score” | PAI-Bench-G；机器人 77.1 对 LingBotWorld 77.0 |
| 同表“画面美观度”驾驶46.6、机器人44.8，7个模型里最低 | L1 | ✅ | Table 4（截图 02）AQ（Aesthetic Quality）列：Ours 46.6，最低（其余 47.3–48.8）；Table 5（截图 03）AQ 列：Ours 44.8，最低（其余 45.6–51.0） | “画面美观度”= Aesthetic Quality 的通俗译法；主执行方对截图 02 逐列核对；7 个 = 6 个对比 + 小米。PAI-Bench-G 评测用的是 Mid-train 模型（论文 5.1 节，`a-arxiv-pdf.txt:975`），正文吠点②与 08 图题已写 |
| 吠点③：小米称仿真碰撞率0.4%、对比方案 ST-JEPA-E2E 1.2% | L1 | ✅ | 官方稿：“在1,000个闭环仿真场景中，E2E-WM-RL方案的碰撞率为 0.4%，相比ST-JEPA-E2E的1.2%大大降低” | 第一轮反向核验 BLOCKER：初稿引 CNMO 当夸大实例，但 CNMO 原句（`a-cn-cnmo.txt`）同样写了“闭环仿真测试中”“在1000个闭环场景里”，已删去该定性 |
| 策略在小米世界模型搭的仿真里训练并测1000个场景 | L1 | ✅ | `a-arxiv-pdf.txt:1494–1497`：“we adopt the causal inference world model to construct closed-loop simulation training scenarios … build closed-loop simulation test scenarios based on the test-set scenes from NavSim. During the closed-loop testing phase, 1000 representative scenes are selected”；Table 10 CR：E2E-WM-RL (Ours) 0.4、ST-JEPA-E2E 1.2（截图 05）；5.3.1：在世界模型仿真器里用 PPO 做在线强化学习后训练 | 对比的 ST-JEPA-E2E 引用为匿名在审稿；同表 Ego Progress 小米 84.5 低于 84.7 |
| 该报告未给实车碰撞率 | L1 | ✅ | 论文闭环章节（5.2、5.3）与 Table 10、11：只有仿真结果；5.2.3 节只给 6 个策略版本仿真排序与实车测试排序的 Pearson 相关（r = −0.88、−0.84），无实车碰撞率 | 缺失性结论，范围限该技术报告的闭环章节与表格 |

## 二、WRAL：回信称可月底付，租客被锁门（B 组）

全部事实出自媒体报道（L4），法院原件因北卡法院门户匿名 403 未取到，正文逐句署“据 WRAL 报道”。租客为私人个体，正文不写姓名，不写其过往驱逐记录。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 省流：据 WRAL 报道，美国一租客收到公寓回信称月底付款可以，仍被锁门，法院10月23日已准房东收回房屋 | L4 | ⚠️ | `b-wral-first.txt`（“Paying on November 30, the last day of the month, works.”）；`b-wral-response.txt`（“WRAL reviewed court documents showing that a Wake County court issued an Oct. 23 order allowing the landlord to evict Mack.”；“Mack was locked out on Thursday.”） | 已署名 WRAL |
| 据 WRAL 报道，北卡一租客11月8日收到公寓邮件，问何时付清近4000美元欠租 | L4 | ⚠️ | `b-wral-first.txt`：“On Nov. 8, she received an email asking, "What date works for you to pay the full $3,934.17?"”（截图 14）；`b-wral-response.txt`：“overdue rent balance of almost $4,000” | WRAL 首报北京 11-20 07:35 发布（UTC 11-19 23:35）；地点 Raleigh |
| 她答月底，公寓当天回信称“11月30日付款可以” | L4 | ⚠️ | 同上：“"I responded, 'I'd like to pay on the last day of the month.'"”；“The email response she got back that same day said "Paying on November 30, the last day of the month, works."” | 邮件原件未取得，WRAL 称查看了租客提供的邮件；回信署名“Colony Townhome Apartments” |
| 16日她收到驱逐通知 | L4 | ⚠️ | `b-wral-response.txt`：“But on Nov. 16, she received an eviction notice.”（截图 15） | |
| 19日向法院申请被驳回、当天被锁在门外 | L4 | ⚠️ | `b-wral-response.txt`：“A motion to appeal the eviction was denied on Nov. 19, court records show.”；“Mack was locked out on Thursday.”（周四 = 11-19）；首报：“Mack went to the Wake County courthouse Thursday morning, where she attempted to reverse that eviction notice.” | WRAL 称“motion to appeal”，房东称“motion to set aside the ruling … denied in District Court”，正文写中性的“向法院的申请被驳回” |
| 吠点②：据 WRAL 报道，租客转述员工的话，称 AI 助手误答可以月底付 | L4 | ⚠️ | `b-wral-response.txt`：“Mack says she was told by a leasing agent that the company's AI agent — called Olivia — had mistakenly given her permission via email to make her rent payment on Nov. 30.” | 租客自述，物业未证实 |
| 对你：WRAL 报道的是美国北卡一宗租房个案 | L1 | ✅（仅指报道范围；案情本身为 L4 ⚠️） | WRAL 三篇报道本身（`b-wral-first.txt`、`b-wral-investigates.txt`、`b-wral-response.txt`）均围绕 Colony Townhome Apartments 的这位租客 | 第二轮反向核验：“对你”位置只宜写可核实的适用范围，原“对你”里的邮箱观察移回事实段 |
| WRAL 看她邮箱：公寓 AI 助手的邮件通常有标注，这两封没标、署名公寓 | L4 | ⚠️ | `b-wral-first.txt`：“But as Mack's email account shows, emails from Olivia are usually clearly marked. The emails Mack received on Nov. 8 never identified that they were from Olivia, included Mack's personal details and were signed "Colony Townhome Apartments".”（截图 14） | 两封 = 问付款日的邮件与回信 |
| 吠点①：Futurism 标题称房东毁掉 AI 的约定再驱逐 | L4 | ⚠️ | `b-futurism.txt` 标题：“Landlord’s AI Makes Rent Deal With Woman, Actual Landlord Breaks Deal and Evicts Her”（北京 11-24 01:31） | 夸大实例，已署名 |
| 据 WRAL 查阅的法院文件，法院10月23日已准房东收回房屋，比那封回信早16天 | L4 | ⚠️ | `b-wral-response.txt`：“WRAL reviewed court documents showing that a Wake County court issued an Oct. 23 order allowing the landlord to evict Mack.”（截图 15） | 10-23 至 11-08 按当地日历日为 16 天；法院原件未取得（门户 403）；WRAL 首报与议员稿未提 10-23，首见于房东回应稿（北京 11-25 08:21，美东 11-24 19:21） |
| 回信是否答应暂缓驱逐，报道没给邮件全文 | L4 | ⚠️ | WRAL 只引回信一句 “Paying on November 30, the last day of the month, works.”（`b-wral-first.txt`），未刊出邮件全文 | 第一轮反向核验 BLOCKER：初稿以裁定日期反驳“毁约”推理过度，改为指出时间线与证据缺口 |
| （同上：租客转述） | L4 | ⚠️ | `b-wral-response.txt` Mack says 句；`b-wral-first.txt`：“she said the leasing agent told her that Olivia, the company's AI agent, had emailed her both times on Nov. 8.” | |
| 房东发言人称11月8日的邮件涉及欠租、没改变法院命令 | L4 | ⚠️ | `b-wral-response.txt`：“Leach said the Nov. 8 email concerned her outstanding rent balance, “which is owed regardless of possession.” He noted that the Nov. 8 email to Mack didn’t address the court's order but didn’t change what the court had ordered.”；Moneywise（经 Yahoo，`b-yahoo-finance.txt`）直接引语 “The November 8th emails concerned her outstanding rent balance … They did not address the court's order and did not change it.” | 第二轮反向核验：按 WRAL 单数 “the Nov. 8 email” 写，去掉“两封”“只谈” |
| 11月18日已告诉她付款安排不改锁门 | L4 | ⚠️ | `b-wral-response.txt`：““Our leasing office told Ms. Mack directly on Nov. 18 that a payment arrangement would not change the court-ordered lockout,” Leach said.”（截图 15） | 第三轮反向核验建议补入；房东一方说法，WRAL 未转述租客对此的回应 |
| 声明没提 AI | L4 | ⚠️ | `b-wral-response.txt`：“Leach's statement contained no mention of artificial intelligence or Olivia, the AI agent.” | Moneywise（经 Yahoo）另称 Leach “neither confirmed nor denied whether the emails sent to Mack were written by an AI agent”，正文按反向核验意见只用 WRAL 范围 |

未进正文（风控与核实度）：北卡参议员 Burgin 的“Bot Bill”（WRAL 未给法案号，ncleg 上 S963/S624 对应关系未证实）；Moffatt v. Air Canada（判决书仅取到第三方托管副本）；租客过往驱逐程序（私人个体信息）。

## 三、速览（C 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 小米：世界模型2.0称全面 SOTA | — | ✅ | 正文小节标题，逐字 | 主帖 1 |
| WRAL：回信称可月底付，租客被锁门 | — | ✅（标题与正文逐字对应；案情为 L4 ⚠️） | 正文小节标题，逐字（据 WRAL 报道） | 主帖 2 |
| 谷歌称 DCA 在 LM1B 实验达基线同等困惑度，训练用时最低约1/3 | L1 | ✅ | `c-c1-google-blog.txt`：“achieve the same quality up to three times faster with a negligible increase in parameters”（博客 2026-11-23，官方未给时刻）；`c-c1-arxiv-abs.txt`：“DCA obtains the same model quality up to 3x faster”，v1 2025-02-10 18:58:52 UTC（北京 02-11 02:58） | LM1B 实验；博客原文 “up to three times faster”；论文 Table 1（LM1B，24 层）达到普通 Transformer 最终困惑度的归一化训练用时，1-DCA、2-DCA 为 0.33（最低），其他配置 0.37–0.47，批次吞吐反而从 8.14 降到 5.62／5.39（第三轮反向核验所引，主执行方未另存论文全文）；论文 2025 年 2 月提交（UTC 日期 2-10）；HN 50672565，北京 11-28 23:3x 约 507 分 |
| Nielsen 称不愿助人参与前沿 AI 研发，下架《神经网络与深度学习》 | L1 | ✅ | `c-c2-nielsen-home.txt`：“The frontier labs are knowingly endangering all of humanity … I don't wish to help (even in a very small way) train people to contribute to this work, and so I've taken the book down.” 署 “November 2026” | 撤下日期原站未注明；HN 50670757，614 分 |
| 历史学家单次测 GPT-6.2-Astra：22分钟提出素描本可能作者，他再查档 | L3 | ✅ | `c-c3-substack.txt`（Mark Humphries，北京 11-24 18:01）：“Only 22 minutes and 1 second later … “… vicomte de Chabot (1780–1875), probably made the travel drawings.””；作者随后向档案馆调取日记核对 | 一次测试，模型发布前、Ultra 档加 goal mode；作者既是测试者也是核验者（调取 Chabot 日记核对笔迹与行程）；HN 50643158，485 分 |
| DeepMind 自测联网加技能，100个筛后有参考谱分子，有效尝试约97%认准 | L1 | ✅ | `c-c4-deepmind-blog.txt`：“With the Mass Spec Skills added, all three models identified and verified the exact molecule in around 97% of attempts”；样本为 MoNA 中 Enamine REAL MS/MS 集 100 个化合物，参考谱 “available in public reference libraries” | 三组对照：无网无技能、仅联网、联网加技能包，97% 为第三组；100 个样本已筛去碎片不足的谱，每谱 3 次尝试，超 1 小时计失败，临时基础设施或配额错误排除；97% 按尝试计；谷歌自测；无网无技能时低于 4%；博客 2026-11-25（官方未给时刻） |
| OS-Omni v1：Opus-5.5 宏平均68.5%受测最高，人类77.3% | L3 | ✅ | `c-c5-osomni-arxiv.txt`：“the strongest evaluated agent, Claude Opus 5.5, reaches 68.5% average success, trailing the 77.3% human reference”；`c-c5-osomni-html-v1.txt` 表：Macro-Avg. Opus 5.5 68.5、GPT-6-Astra 66.1；Micro-Avg. GPT-6-Astra 67.0、Opus 66.7 | v1 为六平台各 48 题加 12 道跨平台题，宏平均为七栏等权（含跨平台）；论文作者自建基准；按微平均 GPT-6-Astra 略高，故写明“宏平均”；arXiv v1 北京 11-25 19:55 |
| 谷歌称发布 HFE 基准，544道基础文字或图文题，征集答错日常题至12月15日 | L1 | ✅ | `c-c7-hfe-blog.txt`：“The dataset consists of 544 elementary text and image prompts across 20 foundational capabilities.”；“Join our three-week Kaggle competition through December 15 and submit everyday questions that today’s models still get wrong.” | 博客 2026-11-24；“544道”为基础文字题与图像题合计（第五轮反向核验称官方技术报告 §2.6 拆为 316 道图文题、228 道纯文字题，公开子集 112 道，主执行方未打开该报告）；“至12月15日”为谷歌公告口径，Kaggle 活动页未读到，截止时刻与时区未核 |
| 据 CNBC 报道，OpenAI 反诉，指控苹果窃取商业机密，苹果称是转移焦点 | L4 | ⚠️ | 反诉提交：联邦法院卷宗 Document 217（`c-c8-court-217.txt`，首页 “Case 5:26-cv-07078-EJD Document 217 Filed 11/27/26”（法院卷宗日期），“DEFENDANTS’ COUNTERCLAIMS AND ANSWER”，反诉方 OpenAI Group PBC、io Products 等，L1 ✅）；苹果回应：`c-c8-cnbc.txt`（北京 11-28 02:56）：“OpenAI sued Apple on Friday, accusing the Big Tech giant of stealing its trade secrets…”；“The ChatGPT maker filed counterclaims in California federal court on Friday”；Apple：“OpenAI's counterclaims are a transparent attempt to distract from the substance of Apple's lawsuit” | 卫报、BBC 同报；路透社原文 401 未取；卷宗 https://storage.courtlistener.com/recap/gov.uscourts.cand.474095/gov.uscourts.cand.474095.217.0_1.pdf 主执行方已取（`c-c8-court-217.pdf`，81 页，不入库）；苹果回应只有媒体转引，速览整句仍按 CNBC 署名 |

## 四、批注译注与图题来源

| 批注图（`out`，成图序号见 `images/README.md`） | 译注 | 级 | 结果 | 依据 |
|---|---|---|---|---|
| sota-claim | 官方原文：在多个主流基准测试中全面取得 SOTA，并已在小米汽车合成数据、仿真测试等核心场景完成业务落地应用。（本号注：SOTA 指当前最好成绩。） | L1 | ✅ | `a-weibo-article.txt`（截图 01）；括号内为本号注 |
| triworldbench ① | TWB-Score（综合分）：WoVR_Plus 65.39、BWM 65.54、CausalWM 66.04。 | L1 | ✅ | Table 9（`a-arxiv-pdf.txt:1318–1327`） |
| triworldbench ② | 小米（Ours）：综合分 64.79，轨迹准确率（Traj. Acc.）47.25。 | L1 | ✅ | 同上，Traj. Acc. Ours 47.25 为该列最高；`a-arxiv-pdf.txt:1269–1270` |
| libero-plus | 加权平均成功率（Avg）：GAM 85.5%，小米（Ours）75.0%；同表“具身预训练”（Embodied PT.）一列，GAM 为 Yes，小米为 No。 | L1 | ✅ | Table 12（`a-arxiv-pdf.txt:1643–1650`），表题 “Avg: weighted average” |
| pai-aesthetic ① | 平均分：表4（驾驶，AV-Quality）小米 78.0、表5（机器人，Robot-Quality）小米 77.1。 | L1 | ✅ | Table 4、5；`a-arxiv-pdf.txt:975`（“we evaluate the model in the Mid-train stage”） |
| pai-aesthetic ② | 美学质量（AQ，正文译作“画面美观度”）：表4 小米 46.6、表5 小米 44.8。 | L1 | ✅ | Table 4、5 AQ 列 |
| collision-sim ① | 碰撞率（CR）：对比方案 ST-JEPA-E2E 1.2%，小米方案 E2E-WM-RL 0.4%。 | L1 | ✅ | Table 10 |
| collision-sim ② | 我们用因果推理世界模型搭建闭环仿真训练场景（用 NavSim 的训练与验证场景数据），并以 NavSim 测试集场景搭建闭环仿真测试场景。 | L1 | ✅ | `a-arxiv-pdf.txt:1494–1496` |
| collision-sim ③ | 闭环测试阶段，选取 1000 个代表性场景作为评测用例。 | L1 | ✅ | `a-arxiv-pdf.txt:1496–1497` |
| wral-emails ① | 她当天收到的回信写着：“11 月 30 日，也就是月底最后一天付款，可以。” | L4 | ⚠️ | `b-wral-first.txt` |
| wral-emails ② | 但从租客的邮箱看，Olivia（公寓的 AI 助手）发的邮件通常有明确标注。 | L4 | ⚠️ | 同上 |
| wral-emails ③ | 她 11 月 8 日收到的邮件都未表明来自 Olivia，含有她的个人信息，署名为 Colony Townhome Apartments。 | L4 | ⚠️ | 同上 |
| wral-court ① | 租客称，一名租赁员工告诉她，是公司名为 Olivia 的 AI 助手通过邮件误允许她 11 月 30 日付租。 | L4 | ⚠️ | `b-wral-response.txt` |
| wral-court ② | 房东发言人称，11 月 8 日发给租客的邮件没有涉及法院命令，也没有改变法院的命令。 | L4 | ⚠️ | 同上 |
| wral-court ③ | WRAL 查阅的法院文件显示，威克县法院 10 月 23 日下令准许房东收回房屋。 | L4 | ⚠️ | 同上 |
| futurism-title | Futurism 标题称：房东的 AI 跟女租客谈好租金约定，真房东却毁约并驱逐了她。 | L4 | ⚠️ | `b-futurism.txt` 标题（作夸大实例） |

图题均为本号所加，内容取自底图所在页面的标题或表号（小米技术报告 Table 4、5、9、10、12 与 5.3.2 节；小米技术官方账号头条文章；WRAL 首报与房东回应稿；Futurism 原题），来源页脚写域名与存档日期（北京 11-28，取证日志见 `capture-log-a.md`、`capture-log-b.md`）；Futurism 图题日期按北京时间写 11 月 24 日（美东 11-23 12:31）。

## 卡片日期

`cards.toml` 的 `date = "11月29日"`：本期北京 11-29 交付；期号 1127 按“上期 + 1”定，与日期无关。

## 反向核验与视觉复核

### 读者稿检查（Codex `gpt-6.2-sol` medium，read-only，无本期上下文；北京 11-29 凌晨，与第一轮反向核验同时发起）

报告与采纳见 `style-review.md`。

### 第一轮反向核验（Codex `gpt-6.2-sol` medium，read-only；北京 11-29 00:2x–00:4x）

复核方为 GPT 家族，与主执行方（Claude）及取证子代理（Claude）均不同家族；审的是读者稿检查前的初稿。报告原样存于系统临时目录 `barking-review/1127/reverse-report.md`。结论 4 项 BLOCKER、9 项 SHOULD_FIX、3 项 NEEDS_VERIFICATION，主执行方逐条对照原文判断：

- BLOCKER 1（“技术报告11月1日已上 arXiv”把提交写成公开）：采纳。该句已在读者稿检查后删去。
- BLOCKER 2（CNMO 原句已含“闭环仿真”“1000个闭环场景”，不能当夸大实例）：采纳，复核 `a-cn-cnmo.txt` 属实。吠点③改为引小米官方稿原话，指出训练与测试用的是同一套小米世界模型仿真、没给实车碰撞率。
- BLOCKER 3（以 10 月 23 日裁定反驳“毁约”推理过度）：采纳。吠点①保留时间线，改为指出回信是否答应暂缓驱逐、报道没给邮件全文。
- BLOCKER 4（质谱 97% 漏“联网”条件）：采纳，复核 DeepMind 博客三组对照属实，速览改“DeepMind 自测联网加质谱技能包”。
- SHOULD_FIX（“全面 SOTA”与第 4 的范围）：采纳，吠点①改“同一报告所列6个方法里”。
- SHOULD_FIX（LIBERO 加权平均、预训练差异；PAI 所列方法）：采纳“加权平均”“在所列方法中第一”；预训练差异写进批注译注 07。Mid-train 版本不进正文（字数），记于清单。
- SHOULD_FIX（租房主帖短标题缺 WRAL 归因、“只涉”排他、②用 WRAL 范围）：采纳，小标题改“WRAL：回信准月底付，租客仍被锁门”，对你改“这是 WRAL 报道的一起个案”，吠点②只用 WRAL 所见声明。
- SHOULD_FIX（DCA 训练口径、“已提交”）：采纳。
- SHOULD_FIX（Humphries 一次测试、L3）：采纳，速览改“历史学家测……他再查档核对”，清单级别改 L3。
- SHOULD_FIX（OS-Omni 宏平均含跨平台列）：采纳，删“测6平台”，写“v1”“宏平均”，清单补七栏等权。
- SHOULD_FIX（反诉“指控”动词）：采纳。
- SHOULD_FIX（案情日期为当地日历日）：采纳，清单总注已改。
- SHOULD_FIX（配图说明与批注未交付）：已补，交第二轮视觉复核。
- NEEDS_VERIFICATION（“没给实车碰撞率”的查证范围）：清单改写为“论文闭环章节（5.2、5.3）与 Table 10、11”；正文限“报告”。
- NEEDS_VERIFICATION（法院与邮件原件、反诉文书未取得）：维持 L4 ⚠️ 与归因。
- NEEDS_VERIFICATION（美国地方民事个案的风控）：交付时告知 Develata。

### 第二轮反向核验（Codex `gpt-6.2-sol` medium，read-only；北京 11-29 凌晨）

报告原样存于系统临时目录 `barking-review/1127/reverse-report-2.md`。2 项 BLOCKER、10 项 SHOULD_FIX，主执行方逐条核对后：

- BLOCKER（标题“小米汽车称全面领先”把只用于 WorldRec 2.0 的“全面领先”套到整体）：采纳，复核 `a-weibo-article.txt` 属实。标题改为「小米称多基准全面最佳，报告一项排第4」，封面按新标题重生。
- BLOCKER（缺封面）：采纳，封面已补（见 `images/README.md`）。
- SHOULD_FIX（“准她月底付”“误发”）：采纳，改“回信称月底付款可以”“误答可以月底付”。
- SHOULD_FIX（“这两封邮件只谈欠款”混用 Moneywise 复数）：采纳，按 WRAL 单数改写。
- SHOULD_FIX（“对你”用了 L4、邮箱观察被泛化）：采纳，邮箱观察移回事实段并写“从她邮箱看”。
- SHOULD_FIX（TriWorldBench 快照、GAM 预训练、PAI 自测与版本）：采纳，正文补“做过具身预训练”“报告用中期训练版自测”，快照条件记清单。
- SHOULD_FIX（DCA、质谱、OS-Omni 速览口径）：采纳。
- SHOULD_FIX（Futurism 日期、译注归属）：采纳，图题写北京 11 月 24 日，译注加“Futurism 标题称”。
- SHOULD_FIX（README 仍把 CNMO 列为夸大实例）：采纳，改为“转载对照”并注明撤回。
- SHOULD_FIX（法院卷宗 Document 217）：采纳，主执行方已取卷宗存档并分开标源。

### 第三轮反向核验与第一轮视觉复核（Codex `gpt-6.2-sol` medium，read-only；北京 11-29 凌晨）

报告原样存于 `barking-review/1127/reverse-report-3.md` 与 `visual-report.md`。反向核验 2 项 BLOCKER、7 项 SHOULD_FIX、2 项 NEEDS_VERIFICATION；视觉复核 2 项 BLOCKER、9 项 SHOULD_FIX、2 项 NICE_TO_HAVE。主执行方判断：

- 反向 BLOCKER 1（标题把“第4”当作对一项范围未定的官方说法的纠正）：部分采纳。省流与吠点①改为写明“发布稿……没列具体基准”，技术报告里那一项是机器人评测；标题保留“说法 + 实际”结构（`EDITORIAL.md`“对比”：同一主体的说法 vs 它自己的文档），不写成官方宣称该项第一。此点交 Develata 终审定夺，备选标题见 `titles.md`。
- 反向 BLOCKER 2 与视觉 BLOCKER 2（质谱 97% 按尝试计）：采纳，速览改“按尝试计约97%认准”。
- 视觉 BLOCKER 1（09 高亮截断在 NavSim 中间）：采纳，第三行框延到 `NavSim.` 句号。
- DCA“最低约为基线1/3”“训练用时”：采纳。
- TriWorldBench、LIBERO 条件，PAI 测评与版本：采纳，正文吠点①②补齐；批注译注只留图内可见的依据（06 补标轨迹准确率单元格、07 补标具身预训练两格，08 删去表外的中期训练说明，改由正文写出）。
- “对你”（租房）须 ✅：采纳，改为“WRAL 报道的是美国北卡一宗租房个案”（对报道本身的描述，L1）。
- 补房东称 11 月 18 日已告知付款安排不改锁门：采纳，进吠点②。
- 清单速览行标题不同步、Document 217 分开标源、微博 17:00 为页面显示时刻：采纳。
- 05 高亮只到逗号、SOTA 释义混在官方原文里：采纳，`quote` 延到整句，释义标“本号注”。
- 08 编号②位置、11 首行引号贴编号：08 的编号位置由模板定，不改；11 改写吠点②开头，不再以引号起头。
- 封面文件名升序会先排横版：`images/README.md` 上传说明已写明先单选竖版。
- 07、13 英文截图偏小、日期断行（NICE_TO_HAVE）：不改，截图保留整表与上下文。
- 风控（美国地方民事驱逐裁定、联邦法院商业纠纷）：交 Develata 判断。

### 第二轮视觉复核（Codex `gpt-6.2-sol` medium，read-only；北京 11-29 凌晨）

报告原样存于 `barking-review/1127/visual-report-2.md`。3 项 BLOCKER、4 项 SHOULD_FIX、1 项 NICE_TO_HAVE：

- BLOCKER（省流卡带“没列具体基准”的纠偏）：采纳，省流删去该句，纠偏只留吠点①；省流保留两条已核实的事实并列。
- BLOCKER（“没给实车数据”范围过宽，5.2.3 节有实车排序相关）：采纳，改“报告没给实车碰撞率”。
- BLOCKER（05 译注混入“原文未列具体基准”的本号结论）：采纳，删去。
- SHOULD_FIX（06、08 译注里“都高于小米”“排第4”“7个模型最低”等编辑比较）：采纳，译注只留指标、方法与数值。
- SHOULD_FIX（09 ③ 译注含高亮外的测试阶段限定）：采纳，`quote` 扩到 “During the closed-loop testing phase, …”。
- SHOULD_FIX（08 编号位置、07 与 13 英文偏小）：不改，编号位置由模板定，截图保留整表与上下文。
- NICE_TO_HAVE（封面上传顺序）：`images/README.md` 已写明。

### 第四轮反向核验（Codex `gpt-6.2-sol` medium，read-only；北京 11-29 凌晨）

报告原样存于 `barking-review/1127/reverse-report-4.md`。3 项 BLOCKER、10 项 SHOULD_FIX、3 项 NEEDS_VERIFICATION：

- BLOCKER 1（标题与封面以官方未点名的 TriWorldBench 作对照，封面车排第 4 易读成汽车成绩）：这是第三轮起连续提出的意见，主执行方提交 Develata，Develata 拍板换题为「小米称视频分第一，同表美观度垫底」：官方点名 PAI-Bench-G 平均分最高，同表画面美观度两场景都是 7 个模型最低，说法与证据同一张表。吠点①②互换，批注图 06（PAI-Bench-G）提前。封面按新主体重画，竖版超过“每种尺寸最多 5 张”，经 Develata 同意破例。
- BLOCKER 2（“没列具体基准”有反例：发布稿点名了 PAI-Bench-G 与 LIBERO Plus）：采纳，改为“没说明‘全面’指哪些基准”。
- BLOCKER 3（“没给实车数据”过宽）：上一轮视觉复核后已改，现为“未给实车碰撞率”。
- SHOULD_FIX（TriWorldBench 快照条件）：快照日期与第一名分数记清单；吠点卡版面放不下，正文不写。
- SHOULD_FIX（对你“只写用于”）：采纳，改“发布稿举的落地场景是……”。
- SHOULD_FIX（PAI 批注丢自测与版本）：采纳，08（现 06）图题加“自测，中期训练版”。
- SHOULD_FIX（WRAL 观察主体）：采纳，改“WRAL 看她邮箱：……通常有标注”。
- SHOULD_FIX（DCA 限 LM1B 实验、质谱“筛后”、OS-Omni 加 v1 且清单记 L3、Nielsen 改“参与前沿 AI 研发”、Humphries 加“单次”）：采纳。DCA 删去“论文2025年2月提交”以容下实验范围。
- SHOULD_FIX（内部日期：WRAL 回应稿北京/美东、卷宗日期）：采纳。
- NEEDS_VERIFICATION（法院与邮件原件、HFE 截止时刻）：维持 L4 与公告口径，HFE 速览改“谷歌称”。
- NEEDS_VERIFICATION（风控：美国地方民事驱逐裁定、联邦法院商业纠纷）：交 Develata。

### 第五轮反向核验与第三轮视觉复核（Codex `gpt-6.2-sol` medium，read-only；北京 11-29 凌晨，换题后）

报告原样存于 `barking-review/1127/reverse-report-5.md` 与 `visual-report-3.md`。视觉复核结论 PASS（无 BLOCKER）；反向核验 1 项 BLOCKER、6 项 SHOULD_FIX、3 项 NEEDS_VERIFICATION。主执行方判断：

- 反向 BLOCKER（省流仍以 TriWorldBench 第4作“全面 SOTA”的反证）与视觉 SHOULD_FIX（同）：采纳，省流改为与标题同一张表的对照（官方称 PAI-Bench-G 均分最高 vs 同表画面美观度两项最低）。
- SHOULD_FIX（标题补“自测”“均分”）：不采纳。吠点①已写明“视频平均分”“所列方法”“中期训练版自测”；标题再改须第 7 次重画封面，标题“同表”已限定对照范围。交 Develata 终审参考。
- SHOULD_FIX（吠点② LIBERO 标“报告表中”，吠点③点名 ST-JEPA-E2E、“该报告未给实车碰撞率”）：采纳；吠点卡版面限制下，具身预训练差异压为“GAM 有具身预训练而小米没有”。
- SHOULD_FIX（租房吠点② WRAL 归因前置）：采纳。
- SHOULD_FIX（HFE 题型、DCA 基线、质谱“有效尝试”、OS-Omni 成功率）：HFE、DCA、质谱采纳；OS-Omni 加“成功率”超 40 字，保留“宏平均……受测最高”，清单注明为任务成功率。素描本 Humphries 自述与 Ultra 档：速览已写“历史学家单次测”，Ultra 档记清单。
- SHOULD_FIX（清单图号换序、图题依据、“对你”与小标题的级别、README 轮次与时刻）：采纳。
- 视觉 SHOULD_FIX（pai-aesthetic 编号对应表4/表5、wral-emails 署名）：采纳，译注写明表号与署名全文。
- NEEDS_VERIFICATION（法院与邮件原件、HFE 截止时刻、反诉与苹果回应分源）：维持 L4 与公告口径。
- 风控（美国地方民事驱逐裁定、联邦法院商业诉讼）：交 Develata。
