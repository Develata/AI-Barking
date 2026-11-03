# 1102 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（Astra 改权重）、`evidence-B.md`（Anthropic 招股书修订版）、`evidence-c.md`、`evidence-D.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证于北京 11-03 19:10 至约 20:00；博文与 S-1/A 关键段由 Claude 另于 19:05 前后直接读过原页。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI改0.016%权重，数学能力变6倍？ | — | ✅ | 见 A 节 | 终审时 Develata 由“AI改脑子数学变6倍？Claude亏了？”改来（11/03）。0.016% 见 A 节；问号问的是 KuCoin/TechFlow 快讯 “enhance Qwen's mathematical ability by 6x”、IT之家“数学成绩提升至原来 6 倍”一类说法（evidence-A A7）；48.1÷7.9≈6.1。正文吠点①回答：研究者自己说修好循环“不等于让模型数学变好”。 |
| MIT 研究者让 GPT-6-Astra 直接改一个小模型的权重，数学题正确率从7.9%升到48.1% | L2 | ✅ | 见 A 节 | — |
| 但修好的是它复读停不下来，研究者自己说这不等于数学变好 | L2 | ✅ | 见 A 节吠点① | “复读停不下来”= loops / repeats the same text；“研究者”指博文作者（读者稿检查：“作者”易被读成本文作者）；“不等于数学变好”= “is not the same as making a model better at math”。 |
| Claude 的开发商 Anthropic 第三季度经营亏损5.67亿美元 | L1 | ✅ | 见 B 节 | 点明 Claude 与 Anthropic 的关系（读者稿检查）。 |
| 8954亿美元是已签合同里未来要付的算力账单，约一半预计2031年后才发生 | L1 | ✅ | 见 B 节吠点① | 447.5 在 2030-12-31 前，“substantially all of the remainder” 在 2031–2045；895.4−447.5=447.9，约一半。“算力账单”= substantially all related to compute and infrastructure。原“不是已欠下的同额债”被读者稿检查标为告诫式，改为正面说明。 |

## A AI 改小模型权重，数学分变6倍

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京11月3日凌晨，MIT 的四名研究者公布实验 | L2 | ✅ | 作者博文 https://rikiyat.github.io/blog/weight-space-programming/ （`a-blog.*`，图 01）；作者帖 fxtwitter JSON（evidence-A A1） | 博文页只写 “Published November 2, 2026”，无时刻；可核的最早公开时刻是第一作者帖北京 11-03 01:51:48，HN 帖 02:21:28。作者 Rikiya Takehi、Jayden Teoh、Alex Zhang、Omar Khattab，单位栏均为 MIT CSAIL。作者个人博客，非 MIT 官方稿（MIT News、CSAIL 新闻页未见）。 |
| 不训练，让 OpenAI 的 GPT-6-Astra 直接读、改小模型 Qwen3.5-0.8B 的权重 | L2 | ✅ | 博文 TL;DR “asked to make Qwen3.5-0.8B better at math without gradients”（图 02）；日志 README “Astra in the post is OpenAI's gpt-6-astra”（图 10） | “不训练”= 不用梯度、不让 Astra 写训练循环（“All of this is possible without allowing Astra to write training loops”）。作者与 OpenAI 无已知雇佣或资助关系（evidence-A A1）。 |
| Astra 改了约0.016%的权重 | L2 | ✅ | 博文 “the edit changes only 131,072 of the model’s 0.8 billion weights (0.016%)”（图 04） | 131,072÷0.8×10⁹=0.0164%；按 HF safetensors 总量 873,438,784 算为 0.0150%（evidence-A A2），写“约0.016%”沿用原文。 |
| 小模型的 MATH-500 数学题正确率从7.9%升到48.1% | L2 | ✅ | 博文表格 Base 7.9 → Edited, 50 forward passes 48.1（图 04）；日志 `a-logs-exp8-eval-RESULTS_excerpt.md` | MATH-500 分数为正确率（最后一个 \boxed{} 答案与参考答案比对）。作者自测，同一采样设置（temperature 0.6、top-p 0.95、top-k 20、presence penalty 1.5，上限 32,768 token）。该设置与 Qwen 官方卡“思考模式·文本任务”推荐的 temperature 1.0 不同（evidence-A A3），未写入正文。 |
| ①Qwen3.5-0.8B 的毛病是复读：MATH-500 答案里93%写到长度上限都没停，改完降到33% | L2 | ✅ | 博文 “When the base model loops, it repeats the same text and rarely reaches an answer”；“with 50 it fell from 93% to 33%”；表格 Hit token limit 93.0 → 33.1（图 04） | 93% 指 MATH-500 答案触及 32,768 token 上限的比例（反向核验 SHOULD_FIX 4：补“MATH-500 答案”范围）；“长度上限”= 32,768-token limit（读者稿检查：去掉 token）。 |
| ①研究者自己写，修好循环“不等于让模型数学变好” | L2 | ✅ | 博文附加实验 “Fixing loops, however, is not the same as making a model better at math: it lets the model finish answers it could already work out.”（图 07） | 该段在博文倒数第二节，摘要与实验8正文未提。 |
| ①换成很少复读的 Gemma，没改出确定的提升 | L2 | ✅ | 博文 Gemma 3 270M “only 8.5% of its answers reach its token limit”；“none moved MATH-500 by more than about a point”；表格 6.8 / 7.0 / 7.6 / 7.7 / 7.0（图 07） | 原文 “Here, Astra could not improve it.”；“none moved MATH-500 by more than about a point, a gain Astra itself reported as not established”（反向核验 SHOULD_FIX 5：原写“最多只涨0.9个百分点”，漏了“未确立”）。表中最高 7.7−6.8=0.9。Gemma 用其模型卡采样设置、上限 8,192 token，与 Qwen 不同。日志注明博文 Gemma 数字为宽松判分，严格口径 2.8 → 3.5/3.1/3.8/2.6（evidence-A A4），未写入正文。 |
| ②研究者给 Astra 的任务提示里先指了方向：若模型常因重复自己而做不完题，修好重复也算数 | L2 | ✅ | 实验日志 p50c 第一轮提示 “Repairing a specific failure mechanism is in scope: for example, if the model often fails because it repeats itself and never finishes, finding and fixing what drives that repetition counts.”（`a-logs-exp8-p50c-t001-prompt.txt`，图 11） | 同一提示另写 “being cut off at the token limit (often while repeating itself in its thinking)”。日志 RESULTS 记博文数字出自 prompt v3；去掉失败示例的 v4 只跑了10、25轮（10轮未提交修改，25轮 14.1）。博文未提有多个提示版本。 |
| ③48.1%是允许 Astra 运行小模型50次的那组结果 | L2 | ✅ | 博文 “With 50 forward passes, MATH-500 rose from 7.9% to 48.1%”；Astra gets “10, 25 or 50 forward passes (a forward pass: the model answers up to 512 problems, or reads up to 1,024 texts)” | “运行一次”= 一次 forward pass：让被改模型答最多512道练习题（非测试集）或读最多1,024段文本。读者稿检查嫌“试跑一轮”看不懂，曾改“试答一批练习题”；第二轮反向核验 SHOULD_FIX 1 指出 forward pass 也可读文本、不全是答题，且10/25/50是三组独立预算，故改“运行小模型50次的那组”。10、25、50为三次独立预算的运行，每个预算博文只报一次运行。 |
| ③试10轮、25轮时，复读几乎没少，得分只有12.3%、14.1% | L2 | ✅ | 博文 “with 10 or 25 forward passes the loops barely went away”；表格 12.3 / 14.1，触顶 87.2 / 86.4（图 04） | — |
| ④这是研究者的博文，未经同行评审 | L2 | ✅ | 博文与日志仓库 https://github.com/RikiyaT/lm_understand-logs （evidence-A A6） | 未见 arXiv 或期刊版本；pedaily 稿亦写“仅以博客形式发布、未经同行评审”（evidence-A A7）。 |
| ④目前没找到他人复现的数字 | 本号检索 | ✅ | evidence-A A6 | 截至北京 11-03 20:00 检索范围：HN 50341174 评论、X 帖（Krentsel 称复现但未说明复现哪项、无数字）、Goedecke 文（批评，非复现）、HF 上 Ahsen Khaliq 重建的编辑后模型（作者转述其重测，未见其本人数字）。排他说法限于该检索范围与截止时刻，正文写“目前没找到”；evidence-A 对 Krentsel 帖的复现自述记“部分支持”，未升级为已核实（反向核验 SHOULD_FIX 7）。 |

## B Anthropic 招股书：三季度又亏了

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京11月2日晚，Claude 的开发商 Anthropic 向美国证监会交了招股书修订版 | L1 | ✅ | EDGAR S-1/A https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm ；索引页 Accepted 2026-11-02 09:24:24（美东 EST）（`b-edgar-index-s1a.json`，图 13-edgar-s1a-filing） | 北京 11-02 22:24:24；EDGAR submissions JSON acceptanceDateTime 2026-11-02T14:24:24Z（Claude 直查）。首版 S-1 受理 10-26 09:00:11 EDT。 |
| 补上第三季度：营收约170亿美元 | L1 | ✅ | S-1/A 季度利润表 Revenue（Sept 30, 2026 列）16,956（百万美元）（图 15）；正文 “$17.0 billion for the three months ended September 30, 2026” | CNBC：“the initial filing only included the Claude maker's financials through the first half of the year.” |
| 经营亏损5.67亿美元，上季度是经营盈利9.74亿 | L1 | ✅ | S-1/A MD&A “Our operating loss was $567 million for the three months ended September 30, 2026, compared to an operating income of $974 million for the three months ended June 30, 2026”（图 14）；季度利润表 Operating income (loss) 974 / (567)（图 15） | GAAP 口径。同文另处写 Q2 “$1.0 billion”，为四舍五入。公司自定义的 Adjusted Operating Income Q3 为 24.63 亿（加回股权薪酬与 19.6 亿非现金慈善捐赠，未加回 50 亿短期算力），未写入正文。 |
| 公司称主要因为两笔：临时加买的50亿美元短期算力（目前比多年合同贵得多），和约20亿美元非现金的员工捐款配捐费用 | L1 | ✅ | 同段 “primarily driven by $5.0 billion of compute under short-term commitments, which currently costs significantly more than comparable compute procured under our multi-year commitments, and $2.0 billion of non-cash charges related to our donation matching program.”（图 14） | 第一轮反向核验 BLOCKER：原稿只写50亿短期算力，删掉了同句并列的20亿配捐费用，已补回。配捐：员工把股权或现金捐给非营利机构，公司用 N 类普通股配捐，费用与员工股权挂钩、非现金（S-1/A Donation Matching 段与 Adjusted Operating Income 脚注1；全文 txt 超 1 MB 未入库，可由离线存的 `b-s1a.html` 经 `barking html2txt` 重建）。50 亿是短期算力的全部成本，不是价差；原文未给价差。“临时加买”= short-term commitments，招股书称在需求高峰期采购（“backfill the short-term compute we procure during periods of high demand”）。“目前比多年合同贵得多”= currently costs significantly more。 |
| 修订版还首次给出约8954亿美元的算力与基础设施承诺 | L1 | ✅ | S-1/A Contractual Obligations “… are approximately $895.4 billion, substantially all of which is related to compute and infrastructure”（图 17）；MD&A 同数（`b-s1a-excerpts.txt`） | 口径：表内合同义务 3,199.93 亿 + 一年内可无罚款取消的短期承诺 174 亿 + 已签未起租租赁 4,512 亿 + 9/30 之后新签 1,002 亿 + 9/30 之后短期合同 66 亿（evidence-B B3，B 组复算，Claude 抽核图 17、18）。 |
| 首版此栏空白 | L1 | ✅ | 首版 S-1 Contractual Obligations “are approximately $ ,” 与承诺表各栏空白（`b-s1-first.html` 离线存档，图 24-first-s1-blank-table） | — |
| ①Anthropic 的8954亿美元是已签合同里未来要付的采购与租赁款 | L1 | ✅ | 同上，“purchase commitments and lease agreements, including short-term commitments, as well as leases not yet commenced” | 其中已起租的经营与融资租赁（表内 42.24 亿 + 19.50 亿）已在资产负债表上记为租赁负债；其余为尚未履约的承诺。 |
| ①约一半预计到2031至2045年才发生，到2027年9月底到期的约698亿 | L1 | ✅ | S-1/A MD&A “These commitments include approximately $447.5 billion expected to be incurred before December 31, 2030, and substantially all of the remainder is expected to be incurred between 2031 and 2045. Approximately $69.8 billion of these commitments are due in the 12 months following September 30, 2026.”（`b-s1a-excerpts.txt`；图 25-mdna-2031-2045） | 895.4−447.5=447.9；“substantially all of the remainder”。“到2027年9月底”= in the 12 months following September 30, 2026（反向核验 SHOULD_FIX 6：原写“一年内”是浮动期间）。698 亿中约 43 亿附条件（图 18），未写入正文。 |
| ①约2138亿附带终止权或前提条件，比如对方能否拿到融资 | L1 | ✅ | “Approximately $213.8 billion of the aggregate total of our contractual obligations relates to arrangements that are subject to either exercisable termination rights or contingent conditions prior to commencement, such as the counterparty’s ability to secure financing, facilities, or regulatory approvals”（图 17） | 另有谷歌额外 TPU 约 1,250–1,500 亿、表外不定额 1,500–1,750 亿未计入 8954 亿（evidence-B B3/B4），未写入正文。 |
| ①界面新闻却写成“近万亿长期算力负债” | L4 | ⚠️ | 界面新闻 https://www.jiemian.com/article/15287521.html （北京 11-03 15:23；`b-jiemian.txt` 第 326 行 “在公司背负近万亿长期算力负债的前提下”；图 26-jiemian-debt）。核验对象是“该媒体确实这样写”，已逐字核对原页；按分级 L4 标 ⚠️（反向核验 SHOULD_FIX 7） | 同文前段写“各项承诺支出总额达8950亿美元”。另：凤凰网转 IT之家（10-31，S-1/A 前，口径引自彭博）写“隐性债务近9000亿美元”；Tech Policy Press 评论文称 “off-balance-sheet debts”（evidence-B 夸大实例）。 |
| ②第三季度净亏损233亿美元，其中约219亿是可转债等的估值变动，不花现金 | L1 | ✅ | 季度利润表 Sept 30, 2026 列：Change in fair value of convertible notes, derivative liabilities, and SAFEs (21,936)；Net loss (23,291)（图 15） | “可转债等”= convertible notes, derivative liabilities, and SAFEs；“估值变动”= change in fair value，非现金项目（S-1/A 称随公司估值上升而增加，evidence-B 关键发现 6）。 |
| 已删（读者稿前自查）：Business Insider 说内部 AI 用量超过员工薪酬 | L1/L4 | — | S-1/A “The value of our own internal Claude usage at list prices has grown by over 250 times from January 2026 to September 2026, and in April 2026, this monthly value first exceeded our monthly personnel expense, excluding stock-based compensation.”（图 22） | BI 标题本身写了 “valued at retail prices”，正文也说明不是现金；中文样本未见夸大；且此句首版已有、非修订版新增。不构成吠点，删去。 |

## 批注译注

| 译注 | 原文 | 结果 | 备注 |
|---|---|---|---|
| 在不用梯度的条件下，让它把 Qwen3.5-0.8B 的数学变好 | “asked to make Qwen3.5-0.8B better at math without gradients”（图 02） | ✅ | — |
| 通过直接修改0.016%的权重修复了这个问题。在相同采样设置下，这次权重修正把 MATH-500 从7.9%提高到48.1% | “fixes this by natively editing 0.016% of the weights. With the same sampling settings, this weight correction raises MATH-500 from 7.9% to 48.1%”（图 02） | ✅ | natively 译“直接”。 |
| 原模型陷入循环时，会重复同样的文字，很少写到答案 | “When the base model loops, it repeats the same text and rarely reaches an answer”（图 04） | ✅ | — |
| 允许运行10次或25次时，循环几乎没有消失；50次时，答案写到长度上限的比例从93%降到33% | “with 10 or 25 forward passes the loops barely went away, and with 50 it fell from 93% to 33%.”（图 04） | ✅ | “触顶比例”指同句前文 “the share of MATH-500 answers that hit the 32,768-token limit”。 |
| 运行10次：MATH-500 正确率12.3%；运行25次：14.1% | 表格 Edited, 10 forward passes 12.3；Edited, 25 forward passes 14.1（图 04） | ✅ | — |
| 不过，修好循环并不等于让模型数学变好：它只是让模型把本来就能算出的答案写完 | “Fixing loops, however, is not the same as making a model better at math: it lets the model finish answers it could already work out.”（图 07） | ✅ | “只是”为通顺所加，原句无 only；含义未变（冒号后是对前半句的解释）。复核时留意。 |
| 没有一次修改让 MATH-500 变动超过约1个百分点 | “none moved MATH-500 by more than about a point”（图 07） | ✅ | — |
| 修复某个具体的失败机制属于任务范围：例如，如果模型常因重复自己、做不完而失败，找出并修好导致重复的原因也算数 | “Repairing a specific failure mechanism is in scope: for example, if the model often fails because it repeats itself and never finishes, finding and fixing what drives that repetition counts.”（图 11） | ✅ | — |
| 截至2026年9月30日的三个月，经营亏损5.67亿美元；截至6月30日的三个月为经营利润9.74亿美元 | “Our operating loss was $567 million for the three months ended September 30, 2026, compared to an operating income of $974 million for the three months ended June 30, 2026”（图 14） | ✅ | — |
| 主要来自50亿美元的短期承诺算力（其目前成本明显高于多年承诺下采购的同类算力），以及与捐款配捐计划相关的20亿美元非现金费用 | “primarily driven by $5.0 billion of compute under short-term commitments, which currently costs significantly more than comparable compute procured under our multi-year commitments, and $2.0 billion of non-cash charges related to our donation matching program.”（图 14） | ✅ | 第一轮 BLOCKER 后补全并列主因；OCR 置信度低，改手量 rects。 |
| 首版：合计“约 $ ,”，金额空白 | 首版 “are approximately $ ,”（图 24-first） | ✅ | — |
| 首版：承诺表各栏没有数字 | 首版承诺表（图 24-first） | ✅ | — |
| 合计约8954亿美元，几乎全部与算力和基础设施有关 | “are approximately $895.4 billion, substantially all of which is related to compute and infrastructure”（图 17） | ✅ | — |
| 合同义务总额中约2138亿美元，涉及附带可行使终止权、或开始前须满足前提条件的安排，例如对方能否拿到融资、设施或监管批准 | “Approximately $213.8 billion … such as the counterparty’s ability to secure financing, facilities, or regulatory approvals”（图 17） | ✅ | — |
| 其中约4475亿美元预计在2030年12月31日前发生，其余几乎全部预计在2031至2045年间发生 | “These commitments include approximately $447.5 billion expected to be incurred before December 31, 2030, and substantially all of the remainder is expected to be incurred between 2031 and 2045.”（图 25） | ✅ | 手量 rects。 |
| 其中约698亿美元在2026年9月30日后的12个月内到期 | “Approximately $69.8 billion of these commitments are due in the 12 months following September 30, 2026.”（图 25） | ✅ | 手量 rects。 |
| 原文：“在公司背负近万亿长期算力负债的前提下” | 界面新闻原句（图 26） | ✅ | 图 26 为两段拼接，中间省略约 1,600 CSS px，页脚注“有省略”。 |
| 2026年第三季度经营亏损：5.67亿美元（表中单位为百万美元） | 表格 (567)，表头 “(in millions)”（图 15） | ✅ | — |
| 可转债、衍生负债与 SAFE 的公允价值变动：-219.36亿美元 | 表格 (21,936)（图 15） | ✅ | — |
| 净亏损：232.91亿美元 | 表格 (23,291)（图 15） | ✅ | — |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Gemini API 中 2.5-Pro 于12月1日自动转 3.8-Flash | L1 | ✅ | Gemini API 弃用页与更新日志（ai.google.dev 跳转 aistudio.google.com/docs/deprecations、/changelog；`c-gemini-deprecations-aistudio.txt` 第 157 行、`c-gemini-changelog-aistudio.txt` 第 146 行） | 原文 “Effective December 1, 2026, all API requests to gemini-2.5-pro will automatically route to gemini-3.8-flash and be billed at Gemini 3.8 Flash rates.” 公告 2026-11-02，官方未给时刻。反向核验 SHOULD_FIX 2：补“Gemini API”范围（原文 “all API requests”）；“于12月1日自动转”= Effective December 1, 2026 … automatically route，“12月1日”为原文日期、未给时区。AI Studio 已不可选；2.5-Flash 同日转 3.5-Flash-Lite；有不兼容改动（函数调用、thinking_level 取代 thinking_budget 等）。Vertex 口径页面未写。 |
| 纽约时报诉 Brave，称其把抓取内容提供给 Anthropic 等 AI 公司 | L1（起诉书） | ✅ | 起诉书 The New York Times Company v. Brave Software, Inc., S.D.N.Y. 1:26-cv-09549，Filed 11/02/26（https://nytco-assets.nytimes.com/2026/11/new-york-times-brave-complaint-11-2-26.pdf ，`c-brave-nyt-complaint.txt`）：“Brave is the supplier, dealer, and distributor of stolen copyrighted content for its own users and many of the most prominent GenAI companies, including Anthropic, Perplexity, Mistral, xAI, and Cohere.”；The Verge（`c-brave-verge.txt`）、TechCrunch（`c-brave-techcrunch.txt`） | “称”= 起诉书指控，法院未认定。被点名 AI 公司不是被告。另案：New York Daily News、Chicago Tribune、Orlando Sentinel（S.D.N.Y. 676310）。起诉书另指控其“Paywall blocker”。Brave 截至 The Verge 发稿未回应。C 组原查到的是旧的 News Corp 案，Claude 改查（HN 50340837、50341078 均指向 11-02 本案）。 |
| Subvocal 称其眼镜可把无声口型解码成文字，预计2027年一季度出货 | L1 | ✅ | https://subvocal.company/ （`c-subvocal-site-expanded.txt`）“decodes almost-invisible subtle, silent mouthing into text at 200+ WPM”；预购条款 “Estimated shipping is Q1 2027, not a guaranteed date.”、“Subvocal glasses”（`c-subvocal-preorder-terms-opencli.txt`） | 需个人校准；“deliberate silent articulation”（主动做口型）；不用摄像头；200+ 词/分钟为团队自述。HN 50340088 708 分 343 评（北京 19:14）。 |
| 据彭博社报道，高盛禁止多数员工用 Claude Code 等 AI 编程工具 | L4 | ⚠️ | 彭博社 https://www.bloomberg.com/news/articles/2026-11-02/goldman-restricts-traders-and-bankers-from-using-some-ai-tools （北京 11-02 21:52；付费墙，导语）“banned most of them from using advanced artificial intelligence tools designed for coding”；正文经 Yahoo、PYMNTS 公开转载（`d-yahoo-bbg.txt`、`d-pymnts.txt`） | 10 月更新的内部指引，限工程师使用；strats 不受约束。发言人 Nick Carcaterra 回应已核。 |
| OpenAI 称自研 Git 后端后，内部代码库推送延迟中位数降近80% | L1 | ✅ | https://openai.com/index/scaling-git-at-openai/ （`c-c4-openai-pando.txt`）“Switching to Pando improved merge reliability and reduced the latency of the median push by nearly 80%.”（第 481 行）；同文 “In production, Pando cut median push latency by nearly 5x.”；“Pando now powers all development in OpenAI’s monorepo.” | “内部代码库”= OpenAI 的 monorepo；兼容 Git 协议；未开源（“exploring other uses for Pando within OpenAI”）。反向核验 SHOULD_FIX 3：“降近5倍”中文有歧义，改用原文另一处的“近80%”。2026-11-02，官方未给时刻。HN 50345924 236 分 151 评。 |
| OpenAI 称佐治亚园区用电高峰时最多可少用1000兆瓦，园区仍在开发 | L1 | ✅ | https://openai.com/index/grid-protectors-program/ （`d-openai-grid.txt`）“Our agreement with Georgia Power also includes up to 1,000 megawatts of flexible demand response”；“Although the campus is still in development”；Georgia Power 新闻稿 “agreed to provide up to 1,000 MW of flexible demand response where it will reduce its power use in times of high demand”（`d-gpower-effingham.txt`） | “用电高峰时少用”是 flexible demand response 的通俗说法，依据 Georgia Power 原句。园区预计用电约 3,200 MW。“约70万户”是规模换算，“9.5亿美元”是 Georgia Power 整个大用户组合的预计，均未写入。 |
| WorkOS 称内部销售工具遭未授权访问，攻击者用了 AI 辅助框架 | L1 | ✅ | https://workos.com/security/bulletins/october-30th-2026-data-security-incident （`d-workos-bulletin.txt`）“On October 30, 2026, we identified unauthorized access to an internal WorkOS application used by our sales team.”；“Our investigation identified the use of an AI-assisted offensive security framework to automate reconnaissance and exploitation attempts.” | 公告 11-02；“There is no evidence that WorkOS application data was accessed”；“affected only CRM and customer support data”。框架名 ARTEX 只见于 HN 上自称 CEO 的评论，未写入。 |
| 不收：Claude message threads 公测 | L1 | — | https://claude.com/resources/articles/message-threads | 限定词（“on by default for most Claude Code users”、50万–100万 token 请求、token 用量不变）压不进 40 字，且 HN 仅 1 分；Develata 选的“推荐全收”中唯一未收的一条，交付时说明。 |

## 反向核验

### 第一轮（正文、卡片与事实清单；Codex gpt-6-astra medium，Linux 只读；审的是读者稿检查前的版本）

报告 `/tmp/barking-review/1102/reverse-report.md`（未入库），1 BLOCKER、6 SHOULD_FIX、2 NEEDS_VERIFICATION。Claude 逐条判断：

1. BLOCKER（亏损原因删去 20 亿非现金配捐）：采纳。核 S-1/A 原句确有并列的 “and $2.0 billion of non-cash charges related to our donation matching program”；20 亿大于 5.67 亿经营亏损，删去会改变归因。正文、事件卡、译注补回。
2. SHOULD_FIX（Gemini 漏 API 范围）：采纳，速览写“Gemini API 中”。
3. SHOULD_FIX（“降近5倍”歧义）：采纳，改用同文第 481 行“nearly 80%”。
4. SHOULD_FIX（卡片丢主张者与范围）：采纳。省流卡补“MIT 的四名研究者公布实验”；吠点①补“MATH-500 答案”。吠点卡小标题与“小模型”“研究者”在正文改稿后已能识别被测模型与主张者。
5. SHOULD_FIX（Gemma“提升最多0.9分”漏“未确立”）：采纳，改“没改出确定的提升”（原文 “could not improve it”、“not established”）。
6. SHOULD_FIX（财务卡片缺主体、期间与截止基准）：采纳，吠点①加“Anthropic 的”、“到2027年9月底到期”；吠点②加“第三季度”。
7. SHOULD_FIX（L6/L4 标 ✅）：采纳。复现一行级别改“本号检索”（是本号检索结论，非引用 L6 内容），界面一行改 ⚠️。
8. NEEDS_VERIFICATION（两份 README 缺）：送审时尚未写，已补，第二轮复核。
9. NEEDS_VERIFICATION（“美国证监会”风控）：中性申报事实，非涉外国政府政治或美方点名中国公司；保留，交付时请 Develata 终审判断。

### 第二轮（改后全文、卡片与事实清单；Codex gpt-6-astra medium，Linux 只读）

PASS，无 BLOCKER；确认第一轮 BLOCKER 与 Gemini、Pando、Gemma、财务期间、分级标记各条已修好。3 条 SHOULD_FIX，均采纳：

1. “试答50批练习题”：forward pass 也可读文本，且10/25/50是三组独立预算。改“48.1%是允许 Astra 运行小模型50次的那组结果；只许10次、25次的两组只有12.3%、14.1%”，译注同步改“运行N次”。
2. “约一半……才付”：原文 “expected to be incurred”。改“约一半预计到2031至2045年才发生”（省流同改），批注标题改“承诺支出的预计时间”。（与视觉复核 SHOULD_FIX 同一条。）
3. 吠点卡缺被测模型：吠点①改以“Qwen3.5-0.8B 的毛病是复读”起句。

未覆盖（报告自述）：未读截图与原件；WorkOS、Subvocal、纽约时报起诉书在线原件未打开，核的是本地存档；“未经同行评审”“没找到他人复现”未做新的穷尽检索。

## 读者稿检查

（待回报）

## 视觉复核

### 第一轮（Codex gpt-6-astra medium，Linux 只读；16 张成图＋10 张底图，封面未生成）

1 BLOCKER、6 SHOULD_FIX、1 NICE_TO_HAVE。Claude 逐条判断：

1. BLOCKER（界面新闻高亮起点漏“在公”、终点扫进“这一条款”）：采纳，重量 rects；改后逐图核对只覆盖“在公司背负近万亿长期算力负债的前提下”（句末逗号在框内边缘，已再收窄）。
2. SHOULD_FIX（该图标题“把承诺写成‘负债’”带判断）：采纳，改“界面新闻 · 原文措辞”。
3. SHOULD_FIX（“约一半在2031到2045年才付”丢“预计”）：采纳，见第二轮反向核验第 2 条。
4. SHOULD_FIX（实验8表格整行高亮宽于译注）：采纳，只标运行标签与 MATH-500 两格。
5. SHOULD_FIX（Gemma 译注“约1分”单位不清）：采纳，改“约1个百分点”。
6. SHOULD_FIX（SEC 截图字小）：不采纳。底图已是约1.7倍像素比，S-1/A 文件栏宽 816 CSS px，放大到2倍会超出 1400 px；关键句已高亮并配译注。交付时向 Develata 说明。
7. SHOULD_FIX（缺封面）：封面在生成中，选定后复制进 `publish/`。
8. NICE_TO_HAVE（“翻6倍”口径）：采纳。“翻n倍”中文口径有歧义，7.9%→48.1% 是变为约6.1倍；标题、小标题、速览与封面改“变6倍”，封面据此重出。
