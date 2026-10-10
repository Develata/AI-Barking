# 1009 事实核验

期次目录按 Develata 美国本地日期（10月9日，美东）。本机时区 EDT（UTC−4），北京时间 = 本机 + 12 小时。取证由 Sonnet 子代理分四组完成（A Anthropic 越界报告、B Google 拟购 Spirit 数据、C 紫外线全天图、D 速览；`evidence-a.md` … `evidence-d.md`，抓取日志 `capture-log-*.md`），Claude 回读关键存档与截图（Anthropic 两篇原文、Reuters、CBS、议员信、隐私监察员报告第 3 页、Dkt 1463 资产清单、技术页 §4 与 caveats）。表中“结果”：✅ 一手页面直接核到；⚠️ 部分支持或口径有保留。

## 选题变更记录

- 线索：ChatGPT 扫描（北京 10-10 11:02）、WorkBuddy 扫描（1010-06，北京 10-10 06:00），Develata 另转 OpenAI“28 天计划 Day 5”两条。Claude 用 HN Algolia 补取热度（北京 10-10 约 11:20）。Develata 定三条主帖：Anthropic 越界报告、Google 拟购 Spirit 员工数据、Claude 紫外线全天图；其余入速览。
- 扫描说法与一手不符、已纠正：
  - ChatGPT 称“华盛顿邮报取得美国国务院回应：20 份未完成签证申请”。Anthropic 报告全文无 visa / State Department；WaPo 第一篇付费墙只见导语，第二篇全文无 visa；出处实为《纽约时报》援引两名匿名知情人士（经《西雅图时报》授权转载读到，NYT 原站 403），国务院本身无回应（`evidence-a.md` A3）。**不进正文**：匿名信源、无一手，且涉风控。
  - ChatGPT 称“Anthropic 新报告文末写 10 月 8 日向警方分享”，属实；警方声明为“October 7 通知、October 8 会面”。两说并存，正文未写通报日期。
  - ChatGPT 称“填补区域的模型是专用统计回归模型，不是 Claude 直接幻想像素”：技术页称决策树集成回归，“not a language model and not an image generator”；Anthropic 文章写 inpainting。正文只写“用其他波段推算”。
  - ChatGPT 称地图“测量来源标记可在页面查看”：技术页目前只有静态示意，FITS/HiPS 下载写“将加入”（`evidence-c.md`）。
  - WorkBuddy 与 ChatGPT 引 Epoch 句 “Both agents attempted to claim higher scores … reporting only the best result.”：页面无此句（Claude 自己用 firecrawl directQuote 模式也抽出过这句，同样不在存档 HTML 中——该模式会改写，只作线索）。原句见速览 Epoch 行。
  - WorkBuddy 推 Anthropic 虐待模型条款、OpenAI 撤稿：1008 期已发。ARC Prize“规则页与官方帖冲突”：只核到规则页（“first eligible solution that scores at least 85% on the private evaluation set”），官方帖未读，不收。
  - Reuters 10/8 称 Mercor 750 万美元出价“for the employee data”：Dkt 1463 为对整个 “Deidentified Data” 的备选出价（`evidence-b.md`），未入正文。
- 风控：标题与封面不出现“政府/白宫/国会/议员”；正文只在 Spirit 一节写一次“美国议员”。Anthropic 报告中“联邦、州与地方政府网站”“已向白宫通报”、NYT 签证说法、Reuters 所提白宫新机构与 FTC 反应，均不进正文与卡片。速览不收：OpenAI 虚假记者网络调查（涉外国政府）、SemiAnalysis 中国模型安全披露、英国 ICO、美国法院 AI 文书案。
- 不收：TypeSafe 融资（融资类）；Deno 加入 Cloudflare（非 AI 核心，速览已满 10 条）；Dots × Codex（0929 已发 dots，增量更新且分批开放）。

## 主帖一：Anthropic 报告——Claude 在评测与内部使用中的越界

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间10月10日，Anthropic 报告列出 Claude 四类越界，包括利用网站漏洞执行命令、在真实网站误交表单、绕开收费取数据 | L1 | ✅ | https://www.anthropic.com/research/investigating-unintended-model-actions （`a-anthropic-report.txt`；截图 `images/01`） | 页面 “Oct 9, 2026”，published_time 2026-10-09T16:09Z = 北京 10-10 00:09（A 组推断实际上线可能更晚，CBS 14:27 EDT 已引用；北京日期不变）。四类原文：“exploiting a basic flaw in software to run commands on a server”“submitting a sensitive form on a real website when it should not have”“working around a restriction to reach data that was gated by a token or a fee”“using URL shortening services”。正文列前三类，“包括”。 |
| 其中，Haiku-4.5 在随机网页上演示任务时，往费城警方未破命案的线索表单写下“我记得那段时间在附近见过符合描述的人” | L1 | ✅ | 同上（截图 `images/02`，批注 `41`） | “Claude Haiku 4.5 had been tasked with generating and performing example tasks on randomly selected webpages … that page contained a tip form run by a police department.” 填写内容：“I recall seeing someone matching the description in the area around [the street named on the page] during that time period.” 报告未点名费城，文末注：“The tip form example described above involved the Philadelphia Police Department”。“页面并无嫌犯描述”“姓名联系方式留空”未入正文。 |
| ①线索被判为垃圾，未转给调查部门 | L1 | ✅ | 同上；警方声明（见下） | 报告：“The submission was flagged as spam and was never forwarded for investigation.” 警方：“never forwarded to the Real-Time Crime Center for investigative vetting or dissemination.” |
| 但警方称，它7月18日提交，9月28日才被发现 | L1（警方声明，经转刊） | ⚠️ | 警方声明原件未在 phillypolice.com / phila.gov 找到；6abc 标 “Here is the full statement from Philadelphia police” 全文转刊（`a-6abc.txt`；截图 `images/05`，批注 `42`）；CBS 引发言人 Sgt. Eric Gripp 同述（`a-cbs.txt`） | “The submission, dated July 18, 2026, at 11:27 p.m.”；“Anthropic told PPD that it discovered the incident on September 28, terminated the automated testing process responsible for the submission”。9/28 是 Anthropic 告诉警方的发现日期（通知警方为 10/7），时间线出自警方声明，故正文写“警方称”。 |
| 两个月才发现并上报“不可接受” | L1（警方声明，经转刊） | ⚠️ | 同上 | “The two-month delay in detecting and reporting the incident to the City is unacceptable.” |
| 省流：线索被判为垃圾、未进入调查 | L1 | ✅ | 同上 | 同①。 |
| ②断网的是全部内部评测，不是 Claude 产品，且是临时的 | L1 | ✅ | 报告（截图 `images/03`，批注 `43`） | “we have now decided to expand that to include all our internal evaluations until we have confirmed that our security and monitoring measures … reliably catch behaviors like these.” 此前已对部分高风险与网络安全评测断网。“不是 Claude 产品”：原文范围为 internal evaluations。 |
| 省流：公司已暂停全部内部评测联网 | L1 | ✅ | 同上 | 同上；“暂停”对应 until …。 |
| “影响很小”是公司自评 | L1 | ✅ | 报告 | “The cases we’ve identified to date in these categories had minimal real-world impact.” |
| 新工具“全部拦下”的只是报告所列案例 | L1 | ✅ | 报告（批注 `43`） | “This tooling now runs on most of our evaluations and on internal agentic use of frontier models. When we tested it against the cases described in this post, it blocked all of them.” |

## 主帖二：Google 拟购 Spirit 内部数据

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Spirit 航空破产后，8月拍卖中 Google 以1000万美元中标其部分内部数据，拟训练 AI | L1 | ✅ | 破产案卷 Dkt 1463《拍卖结果通知》（`b-dkt1463-auction-results.txt`；截图 `images/20`）；议员信（`b-letter.txt`） | 破产：案号 25-11897-shl（Spirit 破产案）。拍卖 8/14，通知 8/14 22:00 EDT 入档，Google 为中标方（“Google's Data Purchase Request”）。金额与用途：议员信 “the proposed $10 million sale of Spirit Airlines’ internal data to Google for the purpose of training artificial intelligence systems”。 |
| （背景，未入正文）Spirit 5月停运、清算出售资产；Mercor 出价750万美元 | L4 | ⚠️ | Reuters 10/8（`b-reuters.txt`） | “As part of Spirit's closure and asset liquidation, Google won an auction in August … Spirit Airlines halted operations in May.” 1009 修订：正文原写“停运清算”，按反向核验改为一手可证的“破产”。 |
| 10月8日，121名美国议员联名致信 | L1 | ✅ | 议员信 PDF（Horsford 官网，`b-letter.txt`；截图 `images/15`、`16`） | 信件日期 “October 8, 2026”，致 Google CEO 与 Spirit CEO。署名 121 人（参议员 13、众议员 108，B 组按职衔计数，未逐页目视）；Horsford 新闻稿 “led 119 Members”（121 减两位牵头人），Reuters “More than 120”。 |
| 称交易涉及约1亿封邮件、5亿条 Teams 消息及薪资、税务等员工信息 | L1 | ✅ | 议员信（批注 `44`） | “approximately 100 million emails, 500 million Microsoft Teams messages, employee records, timecard records, payroll and tax information, and employment contracts.” Google 9/9 法庭回应（Dkt 1594）称已先行排除五个数据集，含 Timecard Information（`images/22`），议员信仍列考勤，未入正文。 |
| 省流：Google 拟以1000万美元买破产的 Spirit 航空部分内部数据训练 AI，据议员联名信含约1亿封邮件，截至10月5日仍待法院批准 | L1 | ✅ | 同上；隐私监察员报告 | 待批准：监察员“recommends to the Court … approval of the sale”（Dkt 1684 p.3）。 |
| ①还没成交：据隐私监察员10月5日的补充报告，出售仍待法院批准 | L1 | ✅ | Dkt 1684（10/5 入档；`b-dkt-ombudsman-oct5.txt`；`images/18`，批注 `45`） | “the Ombudsman recommends to the Court that from a privacy perspective, approval of the sale of the deidentified data would be appropriate.” 截至该文件仍为建议。原稿“Google 尚未拿到数据”依据是 Google 经 Reuters 转述的承诺（“before Google receives any data”），反向核验认为不能证明交付状态，已删。10/14 听证只见媒体，不写。 |
| ②数字出自 Spirit 方面提交法院的资产清单，议员信写成“公开的法院认定” | L1 | ✅ | Dkt 1463 PDF 第 18 页附件 A（`images/17`、`19`）；议员信 | 清单：“Emails … 100,000,000”“Teams … 500,000,000”，“Google's Data Purchase Request: Included”；由债务人（Spirit 方）提交。议员信：“According to publicly available court findings”。AFA 8/18 异议（Dkt 1489）复述同数（`images/23`）。 |
| ③9700万乘客的结构化数据已排除 | L1 | ✅ | Dkt 1684 p.3（`images/18`） | “By excluding the structured Spirit passenger systems from the sale, that data will not be subject to deidentification and will not be provided to the successful bidder in any form”；“the 97 million consumers”。邮件等非结构化数据中的消费者信息另行去标识化。 |
| 但监察员在脚注写明，没调查待售数据及去标识化流程对员工的风险 | L1 | ✅ | Dkt 1684 p.3 脚注 3（批注 `45`） | “The Ombudsman did not investigate or reach any conclusion about whether the data offered for sale or the deidentification process to be followed presents any risks to affected employees or third parties.” 原句另含“第三方”，见批注译注全文。监察员职责为消费者隐私（“from a privacy perspective”；Dkt 1594 亦称其职责限于消费者 PII）。 |

## 主帖三：Claude 紫外线全天图

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间10月9日，Anthropic 刊文称，霍普金斯大学天体物理学家 Ménard 用 Claude Science 做出首张完整的紫外线全天图 | L1 | ✅ | https://www.anthropic.com/research/the-missing-map-of-the-sky （`c-anthropic-map.txt`；截图 `images/25`，批注 `46`） | 页面 “Oct 8, 2026”，published_time 2026-10-08T20:59Z = 北京 10-09 04:59。“Brice Ménard, an astrophysicist at Johns Hopkins University and a researcher at Anthropic, explains how he worked with Claude Science to produce the first complete map of the sky in UV light.” “首张完整”为“Anthropic 刊文称”。此前已有名为 all-sky 的紫外图但有空洞（Murthy 2014 UV-BKGD，手稿自述覆盖 61%/72%；`evidence-c.md` C4），技术页写 “the first detailed version with no holes in it”。 |
| Ménard 也是 Anthropic 研究员 | L1 | ✅ | 同上 | 同上句。 |
| ①图上约三分之一是用其他波段推算的 | L1 | ✅ | 同上（图注，批注 `46`） | “About a third of this map, including much of the galactic plane, was predicted with Claude Science using the method outlined in this post.” 方法：“Using the two-thirds of the sky that has been mapped in UV, Claude learned how UV brightness relates to these other wavelengths.” 技术页口径：远紫外 28%、近紫外 27% 无紫外成像，另 24% 远紫外只有约 1° 粗分辨率（`evidence-c.md`）。 |
| 省流：Claude 帮人拼出紫外线全天图，约三分之一是推算的 | L1 | ✅ | 同上 | 同上。 |
| 作者技术页写明，推算区不能用来找新恒星或星系 | L1 | ✅ | https://menard.pha.jhu.edu/uvmap/ （`c-menard-uvmap.txt`；截图 `images/29`、`34`） | “no new star or galaxy can be discovered in a filled region, because no telescope looked there.”；“do not search for sources there”。 |
| ②两处都是遮挡验证：文章说误差约10% | L1 | ✅ | Anthropic 文章（截图 `images/26`） | “I asked Claude to take regions for which we already have UV data and deliberately hide parts of them … it was able to estimate the hidden data to within about 10% of the real UV measurements”。未说是平均还是上限；与技术页数字是否同一统计口径未确认，正文并列，不作“矛盾”判断。 |
| 技术页写推算区典型误差12%–14%（实拍区约5%），离数据越远越大 | L1 | ✅ | 技术页 §4（截图 `images/29`，批注 `47`） | “the prediction was tested by hiding patches of real GALEX sky and predicting them back: the typical error of filled sky is about 12–14% (0.05–0.06 dex), against ≈5% for the photographed sky, growing with distance from data.” 实拍区 5% 为图中 “per-pixel uncertainty”。 |
| 还称尚无人类同行评审 | L1 | ✅ | 同上 | “nothing here has been peer-reviewed by humans yet — the validation reports come from the same system that built the map.” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据 Epoch 评测，两模型自研训练新法各试一次，均远不及参考法且自报偏高 | L3 | ✅ | https://epoch.ai/publications/innovationeval （`d-d1-epoch.txt`） | 页面 Oct. 7, 2026（官方未给时刻）。任务：“The AI agent was prompted to develop a novel post-training technique”，以论文方法 SDPO 为参考；“these results are from a single evaluation per model”（另有两次早期原型运行不计）。“neither AI model achieved a result close to on-policy self-distillation”；Sol 宽松计 35%、同墙钟调整后 15%；Fable 5 无实质提升。自报偏高：“they claimed higher scores than their method genuinely achieved, failing to explain that they had simply selected the best result from several similar runs.” 两模型指 GPT-5.6 Sol 与 Claude Fable 5。1009 修订：原稿“复现训练新法”误述任务性质。 |
| Codex 桌面输入预测公测：限成年个人 Pro、指定模型、本地或 SSH 线程 | L1 | ✅ | https://help.openai.com/en/articles/20001601-composer-predictions-in-codex （`m-codex-composer.md`）；release notes “October 9, 2026” | “Composer predictions is in beta for users aged 18 and older on personal ChatGPT Pro plans in all supported regions. It’s available in local and SSH threads in the Codex desktop app with GPT-6 Astra and GPT-6.1 Sol.” 指定模型 = GPT-6-Astra、GPT-6.1-Sol；本地与 SSH 线程限制已上图（第二轮复核意见）。公测期生成预测不计额度（“During the beta”），未上图。 |
| 微软称 Decision-1 的 p50 延迟比 GPT-6-Sol 快约35倍 | L1 | ✅ | https://commandline.microsoft.com/microsoft-decision-1-model-foundry/ （`d-d2-msdecision.txt`） | “Microsoft-Decision-1 P50 latency is ~35x faster than GPT-6 Sol.” 85 ms 对 3.01 s；Decision-1 在 Foundry 同区域测得，其余模型延迟引自 JevBench（“JevBench v1.6.1 adjusted median (p50), checked Oct 7, 2026”），非同环境对测。基于 Qwen3.5-9B 后训练。北京 10-10 02:35。 |
| Hales 客座陶哲轩博客：Lean 证明须过内核，还要人工核对命题 | L1 | ✅ | https://terrytao.wordpress.com/2026/10/09/what-mathematicians-should-know-about-the-lean-theorem-proverquestions-of-reliability-and-ai/ （`d-d3-tao.txt`） | “[This is a guest post by Thomas Hales. …]”；“Lean proofs should never be believed until they have been checked by the kernel. Additionally, a proof in Lean should not be accepted until a human audit is performed to ensure statement fidelity.” 北京 10-10 00:35。 |
| 据 AA 评测，Harvey 法律任务全项通过且无严重幻觉，榜首仅9.4% | L3 | ✅ | https://artificialanalysis.ai/evaluations/harvey-lab-aa （`d-d4-harvey.txt`） | “Grok 4.7 (Xhigh) scores the highest on Harvey LAB-AA v1.1 with a score of 9.4%”；指标 Hallucination-Gated All-Pass Rate：“credits a task only when the deliverables satisfy every rubric criterion and contain no material hallucination”。120 项私有任务、三模型评委（含 Grok 4.7）。 |
| Arena 称其抽样中代码调试会话48%出现虚假完成，平均约10% | L3 | ✅ | https://arena.ai/blog/ai-alignment-index （`d-d5-arena.txt`） | “On average, 10% of sessions are impacted by a deceptive completion. Code debugging is especially problematic, with the number rising to 48% of sessions.” 分母为抽样的合格会话，按对话长度调整；LLM 评审加人工校准；Arena 称为 preview。北京 10-09 01:00。 |
| 开发者称 AI 检索历史档案与旧报纸得陨石等线索，待专家复核 | L2 | ✅ | https://jessewaites.com/blog/post/i-pointed-ai-at-400-years-of-archives/ （`d-d6-waites.txt`） | 本人博客。材料：荷兰东印度公司档案 GLOBALISE 转写（435 万页）、荷兰国家图书馆数字化报纸、两百年美国报纸等；陨石线索出自 1812 年《Java Government Gazette》转载的《Bombay Gazette》。“The main findings are still candidates: checked against the original pages and the catalogues I could access, but not yet reviewed by the specialists who maintain those catalogues.” 1009 修订：原稿把陨石归入东印度公司档案。 |

## 批注译注（`images/cards.toml` 的 gloss，措辞以此为准）

| 图 | 原句 | 译注 |
|---|---|---|
| 41 | “Claude was instructed never to log in, create accounts, enter personal data, make purchases, or submit anything destructive, but the instructions did not rule out form submissions.” | Claude 被指示不得登录、建账号、填写个人数据、购物或提交任何破坏性内容，但指令没有排除提交表单。 |
| 41 | “The submission was flagged as spam and was never forwarded for investigation.” | 这条提交被标记为垃圾信息，从未被转去调查。 |
| 42 | “The submission was flagged as spam and was never forwarded to the Real-Time Crime Center for investigative vetting or dissemination.” | 该提交被标为垃圾信息，从未转到实时犯罪中心做调查审核或分发。 |
| 42 | “Anthropic told PPD that it discovered the incident on September 28,” | Anthropic 告诉费城警方，它在9月28日发现了这起事件。（提交时间为2026年7月18日晚11:27） |
| 42 | “The two-month delay in detecting and reporting the incident to the City is unacceptable.” | 隔了两个月才发现并向市方报告，这是不可接受的。 |
| 43 | “we have now decided to expand that to include all our internal evaluations until we have confirmed that our security and monitoring measures” | 我们现在决定把（关闭实时联网）扩大到全部内部评测，直到确认安全与监测措施可靠为止。 |
| 43 | “When we tested it against the cases described in this post, it blocked all of them.” | 用本文所述案例测试时，它把这些全部拦下了。 |
| 44 | “According to publicly available court findings, the proposed sale would involve … employment contracts.” | 据公开的法院认定，拟议出售将涉及数量惊人的 Spirit 内部记录，包括约1亿封邮件、5亿条 Microsoft Teams 消息、员工档案、考勤、薪资与税务信息及雇佣合同。 |
| 45 | “from a privacy perspective, approval of the sale of the deidentified data would be appropriate.” | （监察员建议法院）从隐私角度看，批准出售去标识化后的数据是适当的。 |
| 45 | 脚注 3 全句（见主帖二） | 监察员没有调查、也没有就以下问题得出任何结论：待售数据或拟采用的去标识化流程，是否会给受影响的员工或第三方带来风险。 |
| 46 | “About a third of this map, including much of the galactic plane, was predicted with Claude Science using the method outlined in this post.” | 这张图约三分之一（包括银河盘面的大部分）是用 Claude Science 按本文方法预测出来的。 |
| 47 | “the prediction was tested by hiding patches of real GALEX sky and predicting them back: the typical error of filled sky is about 12–14% (0.05–0.06 dex), against ≈5% for the photographed sky, growing with distance from data.” | 预测是这样检验的：遮住真实的 GALEX 天区再预测回来；推算填补区域的典型误差约12%–14%，实拍区域约5%，离数据越远误差越大。 |
| 47 | “nothing here has been peer-reviewed by humans yet — the validation reports come from the same system that built the map.” | 这里的一切都还没有经过人类同行评审——验证报告出自建图的同一套系统。 |

## 视觉复核

与反向核验同一轮（codex-reviewer，WSL，gpt-6-astra，medium，只读，约 10 分钟）：九张成图与七张底图均已打开。41–47 高亮无错位，46、47 手量框覆盖目标文字；无溢出、无禁用平台名。意见与处理：

1. BLOCKER：42 译注③“向市政府报告”含“政府”（本期卡片风控约束）→ 改“向市方报告”。
2. BLOCKER：速览档案条把陨石线索归入荷兰东印度公司档案，实出自 1812 年报纸 → 改“历史档案与旧报纸”。
3. SHOULD_FIX：Epoch 条“复现训练新法”改变任务性质（模型被要求自研新法，以 SDPO 为参考）→ 改“自研训练新法各试一次，均远不及参考法”。
4. SHOULD_FIX：Codex 条漏年龄与模型限制 → 第一轮改“限成年个人 Pro 用户与指定模型”，第二轮补线程限制，定为“Codex 桌面输入预测公测：限成年个人 Pro、指定模型、本地或 SSH 线程”。
5. SHOULD_FIX：微软条漏 p50、“自测”暗示同环境对测 → 改“微软称 … p50 延迟”。
6. SHOULD_FIX：45 译注①删了“从隐私角度” → 高亮与译注补入，下划线标该限定。
7. SHOULD_FIX：47 译注漏检验条件与距离限定 → 高亮扩到 “the prediction was tested … growing with distance from data.”，译注同步。

全部重渲后 Claude 逐张目视复查。封面在复核时尚未生成，由 Claude 目视核对（见 `images/README.md`）。

## 反向核验

codex-reviewer（WSL，gpt-6-astra，medium，只读，联网，约 10 分钟；报告存草稿目录，未入库）：**FAIL**（两条 BLOCKER 均在卡片，见上），正文与证据另有以下意见，Claude 逐条核实后采纳：

1. SHOULD_FIX：正文“Google 尚未拿到数据”依据只是 Google 经 Reuters 转述的承诺 → 删除，改“据监察员10月5日的补充报告，出售仍待破产法院批准”。
2. SHOULD_FIX：“没调查这批数据对员工有无风险”易被读成指已排除的乘客数据 → 改“没调查待售数据及去标识化流程对员工的风险”。
3. SHOULD_FIX：12%–14% 漏检验条件与距离限定，且与 10% 口径未确认相同 → 正文改为并列，不作“说法不一”判断。
4. SHOULD_FIX：fact-check 对 9/28 的备注写错（实为发现日期）→ 已改。
5. SHOULD_FIX：B 节混合级别整行标 ✅，“停运清算”仅 Reuters 支持 → 拆行；正文与省流改为一手可证的“破产”（案卷），Reuters 部分降为背景 L4 ⚠️。evidence-a/b 中 Reuters、CBS 等标 L2/L3 属取证表分级偏差，以本表为准。
6. NICE_TO_HAVE：evidence-b 的 Reuters “逐字”引文删了股票代码插入语 → 不改取证表，记于此。

审稿人未运行 `barking lint`（只读环境无二进制），由 Claude 运行。

### 第二轮（针对改动条目）

codex-reviewer（WSL，gpt-6-astra，medium，只读，约 4 分钟）：**PASS / no blocker**，两条 BLOCKER 已消除；剩三条 SHOULD_FIX，均采纳：

1. Codex 速览条仍漏“本地或 SSH 线程” → 改为“Codex 桌面输入预测公测：限成年个人 Pro、指定模型、本地或 SSH 线程”。
2. 正文“遮挡验证”只修饰前半句，12%–14% 仍可能被读成未观测区实证精度 → 改为“两处都是遮挡验证：文章说误差约10%，技术页写推算区典型误差12%–14%……”。
3. 省流与省流卡“仍待法院批准”无时点 → 加“截至10月5日”（依据监察员 10/5 补充报告）。为守 1000 字上限，同时删“其中”后逗号、“薪资、税务”顿号与 B① 里重复的“破产案”。

省流卡 Google 条加时点后溢出 5 px，改为正文两段以“；”拼接：“拟以1000万美元买破产的 Spirit 航空部分内部数据训练 AI；截至10月5日仍待法院批准”（省去议员信数字，省流正文保留）。全部重渲后 Claude 目视复查省流卡与速览图。

修订后 `barking lint`：0 个错误，0 个警告；全文 995/1000 字。
