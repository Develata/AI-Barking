# 1026 期事实清单

## A ETH

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 和 Anthropic 同天称自家 AI 推翻了“指数时间假设” | L1 | ✅ | a-oa-blog.txt；a-an-paper.txt；a-an-readme.txt | “同天”按北京时间：OpenAI 仓库提交 2026-10-26T20:52:18Z（北京 10/27 04:52），博文页只写 October 26（官方未给时刻，HN 帖 21:35Z 即北京 05:35）；Anthropic 提交 2026-10-27T03:07:21Z（北京 11:07），论文落款 10/26。美国当地都是 10/26。 |
| “指数时间假设”（ETH）猜：有一类逻辑谜题，变量越多，最难的那道耗时像滚雪球般暴涨，没有根本捷径 | — | ✅ | a-an-paper.txt（ETH 定义段）；a-oa-blog.txt | 通俗说法。精确：存在 δ>0，3-SAT 不能在 2^{δn} 时间内解（等价于没有 2^{o(n)} 算法）；“一类逻辑题”= 3-SAT，“变量数”= n。OpenAI 博文：“a longstanding conjecture about the computational complexity of Boolean formula satisfiability”。 |
| 北京27日清晨，OpenAI 称内部模型证明 ETH 为假，公开论文和 Lean 机器证明 | L1 | ✅ | https://openai.com/index/building-scientific-understanding-with-academic-communities/ ; a-oa-blog.txt；a-oa-eth-paper.txt；a-oa-lean378.txt | 博文：“our system has recently proved that the ETH is false, a result that was subsequently verified in Lean.” 提交北京 04:52、博文约 05:35（“清晨”）。注意：Lean 378.md 证的界是 2^{O_k(n(log log n)^2/√log n)}，比 10/25 稿论文的 n log log n/√log n 弱（“the October 25 manuscript gives the sharper time bound”），正文不展开。 |
| 当天上午，开发 Claude 的 Anthropic 发布署名 Claude 的论文，称独立证得同一结论 | L1 | ✅ | https://github.com/anthropics/formal-math/tree/main/eth ; a-an-readme.txt；a-an-paper.txt | README：“by Claude (Anthropic)”；“refutes the Exponential Time Hypothesis (ETH)”；论文 Provenance：“The core algorithm was found on October 16 by an internal Claude research model … Independently and concurrently, an internal model at OpenAI claims to have proven a similar result.” 提交北京 11:07（“上午”）；Hexagon 发布北京 13:12。 |
| 这还只是两家的说法。OpenAI 请的三位核查学者（含 ETH 两位提出者）在公开信里写，参与“不应被视为认证”；省流：OpenAI 请的核查学者说参与不算认证 | L1 | ✅ | a-oa-blog.txt；a-eth-group-letter.txt | 博文：“This work will be verified and shepherded by a group led by Russell Impagliazzo (UC San Diego), Ramamohan Paturi (UC San Diego), and Ryan Williams (MIT)”；“Impagliazzo and Paturi, along with Francis Zane, are the original proposers of the Exponential Time Hypothesis”（第三位提出者 Zane 不在组内，故写“两位”）。公开信：“Our involvement should not be taken as certification of the claim.” 公开信为 Google 文档、可改、无时间戳，现已同时列出 Anthropic 的声称。 |
| Anthropic 论文写，其 Lean 代码由 AI 写、AI 审，截至撰写时“没有已知的人检查过” | L1 | ✅ | a-an-paper.txt；a-an-readme.txt | “Whether the two definitions say what they are meant to say, that the Exponential Time Hypothesis is false and the Main Theorem, no machine can check; that is for the reader of the listing to judge. The Lean development was written by AI models and reviewed by other instances of the same models; at the time of writing no person is known to have checked it”。README：“Lean checks the proofs, but no machine checks that this file says what the paper says”。证明本身由 Lean 内核与第二内核 nanoda 检查，正文不说“证明没人验”。 |
| Anthropic 论文自己说对实际解题没帮助 | L1 | ✅ | a-an-paper.txt；a-an-readme.txt | 前半是数学事实：ETH 为假只给出 2^{o(n)}（次指数）算法，次指数仍可远超多项式，P=NP 要多项式时间。论文/README：“The result is purely asymptotic and says nothing about solving formulas in practice.”；“the improvement of our result does not appear in practical settings”。“不会变快”限 Anthropic 的结果（OpenAI 论文未见同类声明，两家时间界相同）。 |
| 在它举的例子里（条件数是变量数5倍），所证耗时上限要变量超过10的一万次方个，才低于挨个试 | L1 | ✅ | a-an-paper.txt | “for arbitrary 3-CNF formulas with m = 5n clauses, the bound we prove does not improve on the 2^n of exhaustive search unless n > 10^{10 000}”；只按 n 的界要 n > 10^{10^8}。“举例”= m=5n 的情形；“新算法才比挨个试快”指论文证明的上界，不是实测。 |

## B GPT-6.1 Terra

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 新发布中档模型 GPT-6.1-Terra | L1 | ✅ | https://openai.com/index/introducing-gpt-6-1-terra/ ; b-openai-terra.txt | 页面 “October 26, 2026”，官方未给时刻；HN 提交 2026-10-26T18:01:17Z（北京 10/27 02:01），“26日”按美国当地日期，正文不写日号（反向核验第1轮 SHOULD_FIX 6）。“中档”：OpenAI 产品线 Luna < Terra < Sol/Astra，原句 “For the most complex coding tasks, GPT-6.1 Sol and GPT-6 Astra remain the better choice.” 开放：“all Plus, Pro, Enterprise, and Edu users in ChatGPT Work and Codex”，API `gpt-6.1-terra`。 |
| OpenAI 新发布中档模型 GPT-6.1-Terra，API 单价至少降一半；省流：评测机构称 GPT-6.1-Terra 每题成本降到三分之一以下，是开满档比的 | L3 | ✅ | https://artificialanalysis.ai/articles/gpt-6-1-terra-pushes-the-cost-efficiency-frontier ; b-aa-article.txt | “$1/$5 per million input/output tokens vs $2/$12 for the previous model”：输入降一半、输出降约58%，故写“至少降一半”。OpenAI 发布页价格见图 23。 |
| 评测机构 Artificial Analysis 的综合测试里，开最高推理档的 Terra 得48分，平均每题花0.40美元；上一代最高档42分、1.40美元 | L3 | ✅ | b-aa-article.txt；b-aa-method-intel.txt；b-aa-model.txt；b-aa-m-56terra.txt | “Intelligence Index v4.3.2 incorporates 10 evaluations”；“GPT-6.1 Terra (Max) scores 48”；“it costs $0.40 per task to evaluate GPT-6.1 Terra (Max) on the Intelligence Index”；文章 “at less than a third of the cost per task ($0.40 vs $1.40)”。每题成本是 AA 按 API 价跑全套评测的平均，不是用户账单。AA 文章 October 26, 2026，官方未给时刻；AA 官方 X 首帖 2026-10-26T18:11Z。 |
| “成本降到三分之一以下”是最高档比最高档，评测方写明这一档并不划算 | L3 | ✅ | b-aa-article.txt | “The low and medium effort levels are Pareto-optimal for cost efficiency, while the other effort levels sit behind the frontier.” 指 Intelligence Index 对成本；Coding Agent Index 上前沿是 xhigh 与 low（另一指标，正文不提）。 |
| 单价更贵的 GPT-6.1-Sol 开中档也得48分，每题0.21美元 | L3 | ✅ | https://artificialanalysis.ai/models/gpt-6-1-sol-medium ; b-aa-m-sol-medium.txt | “GPT-6.1 Sol (Medium) scores 48”；“it costs $0.21 per task to evaluate GPT-6.1 Sol (Medium) on the Intelligence Index”；Sol 单价 “$2.00 per 1M input … $10.00 per 1M output”（“更贵”指单价）。AA 文章未做此对比，是 Claude 并列 AA 两个模型页的数字。 |
| “幻觉率从88%降到38%”只算没答对的题里瞎编、不认不会的比例 | L3 | ✅ | b-aa-article.txt；https://artificialanalysis.ai/evaluations/omniscience ; b-aa-method-omni.txt | “It is defined as the proportion of incorrect answers out of all non-correct responses, i.e. incorrect / (incorrect + partial answers + not attempted)”；文章 “guesses 38% of the time when it does not know the answer … GPT-5.6 Terra's hallucination rate was 88%”。“没答对”含部分答对。 |
| 这项测试全部题目里 Terra 答对49%，比上一代只多2个点 | L3 | ✅ | b-aa-article.txt；images/16-aa-omniscience.png | 文章 “coupled with a 2 point increase in AA-Omniscience accuracy”（L3 原句）；AA-Omniscience Accuracy 图柱上标注 GPT-6.1 Terra (max) 49%、GPT-5.6 Terra (max) 47%（Claude 北京 10/28 目视核图 16）。准确率定义：“correctly answered questions out of all questions, regardless of whether the model chooses to answer”。 |

## C 微软

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 微软26日推出限时自愿离职方案（北京23:33发布，故“26日”）：加拿大、印度、英国、美国部分团队的70级及以下员工，可申请带补偿离职 | L1 | ✅ | https://blogs.microsoft.com/blog/2026/10/26/introducing-a-voluntary-exit-offer/ ; c-msft-blog.txt | Amy Coleman 致员工信：“available to eligible employees in participating teams in Canada, India, the United Kingdom and the United States at level 70 and below”；“time-limited, voluntary program that gives those who are eligible the opportunity to leave Microsoft with financial support and career planning resources”；“Some organizations are providing the Voluntary Exit Offer, and some are not.” 页面 Oct 26, 2026；元数据 published_time 2026-10-26T15:33:56Z（北京 23:33）。补偿数额、截止日期未写。 |
| 据 Business Insider 报道，微软发言人称约6万人符合自愿离职条件，超过员工总数四分之一；省流：媒体报的6万是可申请人数；小标题“微软推自愿离职，多少人会走？”（视觉复核 BLOCKER：原小标题“微软6万人可申请离职？”在速览图上缺媒体归属） | L4 | ⚠️ | https://www.businessinsider.com/ （c-bi-memo.txt） | “Around 60,000 employees — or more than a quarter of the company's workforce — about will be eligible, a spokesperson told Business Insider”。BI 约 15:30 UTC（北京 23:30）。基数核对：微软 2026 年报 “As of June 30, 2026, we employed approximately 223,000 people on a full-time basis”（c-msft-ar26.txt），6/22.3≈27%，与 CNBC “27%” 一致。BI 为订阅站，存档为匿名 GET 可见文本。 |
| 6万是能申请的人数：据西雅图时报报道，发言人预计只有3%到5%接受，不到3000人 | L4 | ⚠️ | https://www.seattletimes.com/business/microsoft/microsoft-offers-mass-buyouts-for-roughly-a-quarter-of-its-workforce/ ; c-seattletimes.txt | “A spokesperson for the Redmond-based tech giant said the company is anticipating an acceptance rate as low as 3% to 5% — fewer than 3,000 employees — based on comparable offers observed across other firms.” 依据是其他公司的类似方案。北京 10/26 23:30 前后；标题后来改过。 |
| （已删）官方信没提 AI，把它和 AI 挂钩的是媒体（字数所限删去，读者稿检查亦称不在乎） | L1 | ✅ | c-msft-blog.txt；c-cnbc.txt；c-nyt.txt；c-ibtimes-sg.txt | 官方信正文无 AI 字样（导航栏除外）。CNBC、Reuters、NYT 标题或概括以 AI 投入解读；IBTimes SG 归入 AI 裁员框。7/6 另一封裁员信曾写 “the roles eliminated today are not being replaced by AI”（c-msft-jul-blog.txt，不是本次）。 |

## 批注译注（图上额外事实）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 40：Lean 检查证明，但没有机器检查这个文件说的就是论文说的那个命题 | L1 | ✅ | a-an-readme.txt | “Lean checks the proofs, but no machine checks that this file says what the paper says” |
| 41①：结果纯属渐近：对子句数 m=5n 的任意 3-CNF 公式，所证上界要 n 超过10的一万次方，才优于穷举的 2^n | L1 | ✅ | a-an-paper.txt | “Our result is purely asymptotic: for arbitrary 3-CNF formulas with m = 5n clauses, the bound we prove does not improve on the 2^n of exhaustive search unless n > 10^{10 000}” |
| 41②：Lean 开发由 AI 模型编写、由这些模型的其他实例审阅；截至撰写时，没有已知的人检查过 | L1 | ✅ | a-an-paper.txt | 原句见 A 节 |
| 42①：最高档下，GPT-6.1 Terra 的 AA 智能指数比 GPT-5.6 Terra 高6分，每题成本不到其三分之一（0.40美元对1.40美元） | L3 | ✅ | b-aa-article.txt | “At max effort, GPT-6.1 Terra gains 6 points in the Artificial Analysis Intelligence Index over GPT-5.6 Terra, at less than a third of the cost per task ($0.40 vs $1.40).” |
| 42②：低档和中档在成本效率上帕累托最优（智能指数对成本），其他档位落在前沿之后 | L3 | ✅ | b-aa-article.txt | “Pareto-optimal for Intelligence vs Cost at low and medium effort … The low and medium effort levels are Pareto-optimal for cost efficiency, while the other effort levels sit behind the frontier.” 正文“并不划算”是对 behind the frontier 的通俗说法：同档分数下有更便宜的配置。 |
| 43：定义为：在所有未答对的回答中答错的比例，即 答错 /（答错 + 部分答对 + 未作答） | L3 | ✅ | b-aa-method-omni.txt | 原句见 B 节 |
| 44①：面向加拿大、印度、英国、美国参与团队中 70 级及以下的符合条件员工 | L1 | ✅ | c-msft-blog.txt | 原句见 C 节 |
| 44②：有些部门提供自愿离职方案，有些不提供 | L1 | ✅ | c-msft-blog.txt | “Some organizations are providing the Voluntary Exit Offer, and some are not.” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 招股书：上半年净亏807亿美元，724亿为可转债等账面变动 | L1 | ✅ | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm ; d-s1-sec.txt（汇总表约行 2620–2830） | 表头 “(in millions …)”，“Six Months Ended June 30,” 2026 列：Revenue $16,286；Operating loss (693)；“Change in fair value of convertible notes, derivative liabilities, and SAFEs” (72,388)；Other income (expense), net (8,757)；Net loss $(80,748)。“可转债等”= convertible notes, derivative liabilities, and SAFEs；“账面变动”指公允价值变动（非现金），营业亏损只有6.93亿。807、724为四舍五入。S-1 于 EDGAR 受理 2026-10-26 09:00:11（美东，北京 21:00）；注册尚未生效、未上市（d-s1-news.txt）。HN “Anthropic S-1” 1667/1606（北京 10/26 21:04 提交）。 |
| Anthropic 招股书称，曾为防权重外泄推迟新基座模型预训练一个多月 | L1 | ✅ | 同上；d-s1-sec.txt 风险因素段（约行 5800–5812） | 原句：“we recently chose to delay pre-training a new frontier base model for more than a month to allow time to further enhance our security measures related to preventing model weight exfiltration and protecting our intellectual property.” 未给具体日期；理由是安全措施（防权重外泄、保护知识产权），不是模型危险。同段另述的“四个模型10–34小时”评测事故为 10/9 报告旧事（1009 已发），不收。 |
| noyb 在法意荷奥对 Scale AI 提5项投诉，称其标注平台监控工人 | L1 | ✅ | https://noyb.eu/en/ai-training-platform-outlier-illegally-surveils-european-platform-workers ; d-noyb.txt | “has filed 5 complaints against Scale AI in France, Italy, the Netherlands and Austria”；平台 Outlier，“more than 900,000 ‘freelance’ workers”；追踪器来自 TikTok、LinkedIn、Facebook、Sprig、Datadog；Persona 人脸视频与证件验证。投诉不是裁决；Scale AI 回应未找到。noyb 页 27 October 2026，官方未给时刻。扫描“最多19种追踪器”与 DiPLab 报告（10/15，d-diplab-pdf.txt）的 “16 unique trackers” 不符，不用。noyb 原文客户名单含美国政府，卡片与正文不提。HN 无帖。 |
| Cloudflare 称其 GLM-5.3 服务曾有155条回答混入他人数据 | L1 | ✅ | https://blog.cloudflare.com/kv-cache-exposure-remediation/ ; d-cf-kv.txt；SGLang https://www.lmsys.org/blog/2026-10-26-cf-security ; d-sglang-cf.txt | “Between August 27 and September 9, a small number of customers using the GLM 5.3 Workers AI model received responses that contained data from other customers' requests made to the same model shortly before theirs. In total, 155 responses were affected, all to prompts longer than 681,280 tokens.” 单位是回答（responses），不是客户或会话；限 Workers AI 上的该模型；9/9 已修复（“we restored the model on September 9”）。原文未见逐一通知客户。博文 datePublished 2026-10-26T22:01Z（北京 10/27 06:01）。HN 162/61。 |
| 据 404 Media 报道，多名来信者称 ChatGPT 推荐他们向该媒体求助 | L4 | ⚠️ | https://www.404media.co/chatgpt-keeps-telling-people-404-media-can-solve-their-tech-support-problems/ ; d-404media.txt | “increasingly, ChatGPT is telling people that have normal tech support problems that they have discovered something journalistically notable and that 404 Media, specifically, can help.” 样本是主动来信的用户，无发生率；ChatGPT 对话未公开；付费段与 OpenAI 回应未取得（不绕过）。发布 2026-10-26T15:49Z（北京 23:49）。HN 253/94。 |

未收：Meta 股票融资与英伟达认购30亿美元（EDITORIAL：融资合同类不收；“975亿”是 Reuters 加总，含尚未签约的最多600亿美元 ATM）；IEEE Spectrum 维基百科访谈（10/21 旧访谈，非新闻事件）。

## 读者稿检查

Codex gpt-6-astra medium（read-only，本机 danger-full-access 被拦），报告与采纳见 `style-review.md`。

## 反向核验

第1轮：WSL Codex gpt-6-astra medium（`barking review --reverse --run`），4 BLOCKER、3 SHOULD_FIX、1 NEEDS_VERIFICATION，Claude 逐条核实均成立：
1. BLOCKER 标题与省流“人还没审（完）”扩大了公开信（“不应被视为认证”）与 Anthropic 论文（截至撰写时、Lean 开发、据其所知）的有限说法 → 标题改“两家AI撞车？同天称推翻同一猜想”；省流改“OpenAI 请的核查学者说参与不算认证”；吠点①首句改“这还只是两家的说法”。封面随之重画。
2. BLOCKER “不会让电脑实际解题变快”“变量要超过…才比挨个试快”把所证上界写成实测速度、把本篇限制扩大成推翻 ETH 的普遍后果 → 改“Anthropic 论文自己说对实际解题没帮助：在它举的例子里（条件数是变量数5倍），所证耗时上限要变量超过10的一万次方个，才低于挨个试”。
3. BLOCKER 速览 Cloudflare 丢了服务范围 → “其平台 GLM-5.3 服务”。
4. BLOCKER 微软 L4 两行误标 ✅、省流与小标题未带媒体归属 → 两行改 ⚠️；省流改“媒体报的6万是可申请人数”；小标题与速览改问句“微软6万人可申请离职？”（问的是媒体数字）。
5. SHOULD_FIX AA 评测范围混用 → 省流加“评测机构称”；49% 一句加“这项测试”；42 号译注补“AA 智能指数”“（智能指数对成本）”。
6. SHOULD_FIX Terra“26日”是美国日期 → 改“新发布”。微软“26日”有元数据 15:33Z（北京 23:33），保留。
7. SHOULD_FIX 译注额外事实未入事实列 → 新增“批注译注”一节。
8. NEEDS_VERIFICATION 49%/47%：Claude 已目视核图 16 柱上标注（见 B 节），维持 ✅。

第2轮：同上，PASS（无 BLOCKER），6 SHOULD_FIX、2 NEEDS_VERIFICATION，处理如下：
1. Lean 一句补“截至撰写时”（正文、清单）。
2. 速览 Cloudflare 改“Cloudflare 称其 GLM-5.3 服务曾有155条回答混入他人数据”（“曾”“已修复”；8/27–9/9、>681,280 token 条件留备注，40字放不下）。
3. 省流卡与正文“约6万人符合条件”改“符合自愿离职条件”。
4. 速览 404 Media 改“多名来信者称 ChatGPT 推荐他们向该媒体求助”（原文 “a handful of ChatGPT conversations shared with 404 Media”）。
5. 41 号批注标题“摘要”改“引言”。
6. 批注页脚“2026-10-27 存档”：40/41 底图来自 A 组截图（capture-log-a.md，北京 10/27 22:5x–23:4x）；42/43 来自 B 组（capture-log-b.md）；44 来自 C 组（capture-log-c.md）；裁切与拼接记录 e-shots-log.jsonl。
7. 49%/47% 维持（Claude 已目视核图 16，见 B 节）。
8. 时间证据强度：OpenAI、Anthropic 的北京时刻取自 GitHub 提交时间（`a-gh-*` 提交 JSON），不等于首次公开时刻；正文只写“清晨”“当天上午”，与提交时刻一致，不写具体时刻。微软 15:33:56Z 出自页面元数据 published_time（C 组 evidence-c.md 第18行转录；HTTP Last-Modified 相差7小时，疑时区标错），“26日”在北京与美国当地均成立。

## 视觉复核

WSL Codex gpt-6-astra medium（`barking review --visual --run`，14 张图），2 BLOCKER、2 SHOULD_FIX，Claude 核实均成立：
1. BLOCKER 速览 Cloudflare 缺服务范围 → “Cloudflare 称其 GLM-5.3 服务曾有155条回答混入他人数据”（40 字所限，“已修复”与时间段留备注）。
2. BLOCKER 速览与小标题“微软6万人可申请离职？”缺媒体归属 → 小标题与速览主帖条目改“微软推自愿离职，多少人会走？”，6万只在正文带“据 Business Insider 报道”出现。
3. SHOULD_FIX 43 号底图分辨率低、手机难读 → 移入备用图，不作正式配图（正文已写清分母）。
4. SHOULD_FIX 41 号译注②“同一模型”收窄了 “other instances of the same models” → 改“这些模型的其他实例”。
复核时横版封面为第 4 轮版本（与终稿相同）；竖版为第 3 轮版本（终稿）。
