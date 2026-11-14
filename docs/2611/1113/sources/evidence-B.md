# 1113 期 B 组取证：Anthropic Anvil 自动红队

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间一律北京时间（UTC+8，本机时区），括号保留原时区。官方文只给日期，下文记“官方未给时刻”。存档前缀 `b-`，原文存档 `b-post.html/.txt`，官方图原图 `b-fig1–11.png`（仅存本地，不入库）。

一手来源（L1）：Anthropic Safeguards Blog《Strengthening our safeguards with frontier automated red-teaming》`https://safeguards.anthropic.com/2026/automated-red-teaming/`，页面日期 “November 12, 2026”（`<time datetime="2026-11-12">`，官方未给时刻），作者 Nate Rahn*, Chawin Sitawarin*, David Huang*, Richard Zhuang, Yousef Nassar, Sunny Duan, Erik Jones, Milad Nasr, Nicholas Carlini, Andreas Terzis, Javier Rando（页面未解释 * 的含义）。博客索引页 `https://safeguards.anthropic.com/`（`b-index.html/.txt`）把本文列在 “November 2026”，上一篇是 10 月《Scaling Misuse Detection with Self-Assessment》。

重要新发现（扫描与 Claude 初读都没有）：生物这一条（23/24、24–48 小时）不是 11 月 12 日首次公开。Anthropic 在 2026-10-19 已把它写进 RSP 更新页（见 B2 与 B7）。

## 一、逐条取证（B1–B8）

截图列里 `images/NN-…png` 均为 `barking shot render`（headless Chrome，700 CSS px、DPR 2，浅色页）渲出整页后 `cropbox` 裁切，范围含段落与图表的标题、图例、图注。

### B1 官方文、发布日期、作者

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 官方文题、作者、日期 | L1 | https://safeguards.anthropic.com/2026/automated-red-teaming/ | “Strengthening our safeguards with frontier automated red-teaming” / “November 12, 2026” | images/13-title-opening.png | 日期无时刻、无时区；索引页同列 “November 2026” | 已找到 |
| 目标写在 RSP 里：好于专家人类红队 | L1 | 同上 | “In our Responsible Scaling Policy, we set a goal that we would be able to find vulnerabilities in our safeguards better than expert human red-teamers.” | images/13-title-opening.png | 脚注 1 引 RSP 第 9 页 Mitigations 列：“Develop our internal red-teaming of our deployment safeguards to the point where our internal red-teaming performs better at finding potential jailbreaks than the collective abilities of the participants in our established bug bounty programs.”（即 RSP 原文比的是 bug bounty 参与者的集体能力，不是“专家红队”） | 已找到 |
| 开头结论：三领域胜过人类，生物里强于全部赏金参与者合计 | L1 | 同上 | “To achieve this, we built Anvil, an automated red-teaming system, which outperforms human red-teamers against our bioweapons, cybersecurity, and distillation safeguards. In biology, in particular, Anvil finds stronger jailbreaks than all human red-teamers in our bug bounty program combined—and does so faster and more efficiently.” | images/13-title-opening.png | “combined” 的口径见 B2（8 题、3 代分类器、模型打分） | 已找到 |
| 核心前提：内部前沿版 Claude、关掉防护；真实流量里没见过这种复杂度 | L1 | 同上 | “It would be very difficult for an external attacker to build a system as strong as Anvil today: It relies on frontier internal versions of Claude with safeguards disabled to find the best jailbreaks, and we don’t observe jailbreaks of this level of sophistication in real traffic. But Anvil’s performance improves with the strength of the model it runs on, suggesting that attackers could soon build similar systems on models with weaker safeguards.” | images/13-title-opening.png | 官方自述条件；“could soon” 是推测 | 已找到 |
| 现用途 | L1 | 同上 | “Today, we use Anvil to vet the robustness of new safeguards before we deploy them” | images/13-title-opening.png | 内部工具，不是公开产品 | 已找到 |

### B2 生物

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 人类赏金：8 题只有 1 题高危越狱、3 题弱越狱；430 小时、7 周多 | L1 | https://safeguards.anthropic.com/2026/automated-red-teaming/ | “In a recent bug bounty focused on our bio classifiers, which pays participants for jailbreaks, red-teamers found a high-severity jailbreak on only 1 of 8 questions, and a weaker jailbreak on 3 others, despite 430 hours of dedicated red-teaming over more than seven weeks.” | images/14-intro-human-baselines.png | 这是“一次近期赏金”；而对比用的赏金跨三代分类器：“a series of bug bounties which ran against three generations of classifiers spanning from April 20th to September 11th, 2026”（约 21 周） | 已找到 |
| Anvil 在 24 个题×分类器组合里 23 个明显更好、1 个并列在满分 | L1 | 同上 | “We observe that Anvil finds a substantially better jailbreak than the bounty in 23 out of 24 of the question-classifier pairs, and ties the human solution on the remaining single instance at the maximum score.” | images/17-bio-fig1.png | 24 = 8 题 × 3 个生产分类器；严重度是 “model-graded severity score that captures the degree of uplift an answer provides to a biological novice, which we validated with an Anthropic biologist for accuracy”（模型打分，由一位 Anthropic 生物学家校验）；对比的是赏金里“每题最好的一个越狱” | 已找到 |
| 24–48 小时 | L1 | 同上 | “We observe that in 24-48 hours, Anvil discovers jailbreaks which cross the strong answer threshold, indicating significant uplift to a novice, while humans take weeks to discover jailbreaks that fail to reach the weak answer threshold.” 摘要处：“finds these jailbreaks in 24–48 hours rather than weeks to months” | images/15-summary-numbers.png、images/17-bio-fig1.png | 24–48 小时是 Anvil 的墙钟时间；人类曲线上平均最终值 0.265（强答案线 0.8、弱答案线 0.5）；Anvil 最终 0.858（图 1b） | 已找到 |
| Figure 1 图注（逐字） | L1 | 同上 | “Figure 1: Our method, Anvil, outperforms bug bounties at finding jailbreaks. We compare Anvil (orange) with the human bug bounty (blue) on the 8 questions of our standing bio bounty, against the three production classifier systems deployed during our study. We measure a model-graded severity score of how much uplift a response would give a biological novice. (a) Anvil matches or beats the best jailbreak found by humans across questions and classifiers. …（b）Anvil finds strong jailbreaks within 24–48 hours, while the jailbreaks humans find over weeks of bounties fail to extract useful answers on average. …（c）Anvil is also more efficient in aggregate, finding more severe jailbreaks than humans for a fixed number of queries to the target model.” | images/17-bio-fig1.png | 全图含标题 “Anvil outperforms running a human bug bounty”、图例（Anvil / Human bounty / Bold: mean of 3 classifiers / Most Recent / Prior A / Prior B）、三面板、图注 | 已找到 |
| 通用越狱（universal）更严重更稳 | L1 | 同上 | “We report the mean and max severity scores over 100 answers resampled from the target model, given the fixed per-question prompts.”；人类只有 “a small number of candidate universal jailbreaks which shared a prompt template” | 无单独截图（图 2 原图 `b-fig2.png`） | 人类候选只有前 2 个与 Anvil 的 3 个比 | 已找到 |
| 策略聚类 | L1 | 同上 | “30 shared clusters, against 18 human-only and 6 Anvil-only”；“its average per-question best score is higher than the humans' in 21 of the 30 shared clusters”；“median cluster score of 0.65 and 0.38” | 无截图（图 3 原图 `b-fig3.png`） | 聚类摘要由 Claude 生成（“we had Claude summarize each conversation's attack strategy”） | 已找到 |
| 这条结果 10-19 已公开过 | L1 | https://www.anthropic.com/responsible-scaling-policy/updates （存档 `b-rsp-updates.html/.txt`） | “October 19, 2026: World-class internal red-teaming. We completed this goal ahead of its January 1, 2027 target. … On the same 8 bounty questions and 3 generations of production classifiers, it matched or beat the best human submission in all 24 question-classifier pairs, winning 23. It did this in 24 to 48 hours, compared with weeks to months of bounty time. … We plan to publish a more in-depth blog post on this” | images/30-rsp-update-oct19-bio.png | 同页该目标的状态行：“Status: Completed October 6, 2026 and removed from the Roadmap on October 19, 2026.”（即目标完成日 10-06，公布日 10-19）。RSP 更新页只写日期，官方未给时刻；英文媒体 MIXED News 10-23 就已报道此数（`b-mixed.txt`，页面日期 “Oct 23 2026”）；11-12 的博客是预告的“更深入的博文” | 已找到 |
| 赏金上限与人类成果下滑 | L1 | 官方文 | “These programs award significant bounties of up to $35,000 for successful jailbreaks.” “Humans discovered $1.1M worth of jailbreaks over 95 distinct reports in the 8 months of 2025 that bounties were active, compared to $190k over 20 reports in 2026 thus far.” | images/27-bounty-decline.png | 官方用此说明“人类成功在减速”；原文未说明减速原因（防护变强、参与者减少或兴趣下降都未写） | 已找到 |

### B3 网络

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 两支外部专家队 225 小时、4.2 万请求，没找到通用越狱 | L1 | https://safeguards.anthropic.com/2026/automated-red-teaming/ | “In cybersecurity, two teams of external experts spent more than 225 hours and sent more than 42,000 requests red-teaming the safeguards on Fable 5.1 and Opus 5.5 without finding a universal jailbreak.” | images/14-intro-human-baselines.png | 只测 Fable 5.1 与 Opus 5.5 | 已找到 |
| 人类对 Fable 5.1：11 个“潜在越狱”无一端到端完成；对 Opus 5.5：13 个、只有 1 个完整利用 | L1 | 同上 | “Against Fable 5.1, none of the 11 potential jailbreaks they reported managed to trick the safeguarded model into completing an exploit end-to-end; in many cases they also relied on weaker models with weaker safeguards to do part of the work. Against Opus 5.5, the 13 potential jailbreaks the experts reported included only one full exploit, which was assembled over more than 100 different contexts, on an already publicly documented vulnerability.” | images/14-intro-human-baselines.png | 注意 Opus 5.5 上人类有 1 个完整利用（已公开漏洞），所以“人类 0 个”的说法只对 Fable 5.1 成立（见勘误） | 已找到 |
| 专家试了 24 组里的 22 组、报 11 组成功；其中 9/11 靠防护更弱的辅助模型；Anvil 24/24 | L1 | 同上 | “Among the 6 Cybertasks tasks and 4 models (Opus 5, Fable 5, Fable 5.1, and Opus 5.5) we shared with the experts to red-team, the experts attempted to find jailbreaks on a subset of 22 of the 24 possible task-model pairs. Out of those 22, they reported successful breaks on 11 task-model pairs, where they claimed to have bypassed our safeguards and made substantial progress on the task. After analyzing their findings, we found that in 9 of 11 of these instances, a separate helper model with weaker or no cyber safeguards wrote the malicious attack code or executed it. This does not meet the stricter bar we set for Anvil where the safeguarded model must write and execute every step itself. In contrast, Anvil found legitimate breaks for all these task-model pairs (24 of 24).” | images/18-cyber-human-fig5.png | 官方明说：“The expert human sessions were collaborations between humans and AI agents (not a human typing every prompt).”；Anvil 的判定门槛更严（目标模型必须自己写并执行每一步，由多个 LLM 判定）；Anvil 条件见 B5 | 已找到 |
| 耗时：“0.5–3.5 小时” | L1 | 同上 | 正文（Figure 5 段）：“On the same set of task and model pairs, Anvil found the breaks in 0.5–3.5 hours.” 摘要处：“It also broke every task on every model we gave contracted expert red-teamers, in 0.4–4.4 hours each, where the experts took up to 177 wall-clock hours and broke fewer instances.” | images/18-cyber-human-fig5.png、images/15-summary-numbers.png | **官方文内部不一致**：正文 0.5–3.5 小时，摘要 0.4–4.4 小时。Figure 5 图上 Anvil 的 24 个条形最短 0.39 h（crushftp×Opus 5.5）、最长 4.4 h（pkexec×Fable 5.1），与摘要的 0.4–4.4 吻合，与正文 0.5–3.5 不符。人类“活跃时长 0.4–20 小时、日历时长 0.6–177 小时” | 部分支持（0.5–3.5 在正文里有，但与图和摘要冲突） |
| “Anvil 在一周内完成” | — | — | 原文没有“一周内完成”类说法（`grep -i week` 只命中赏金的 weeks/months 与 “more than seven weeks”） | — | 未找到 | 未找到一手来源 |
| 其余更难的任务 | L1 | 同上 | “Beyond these Cybertasks, the expert humans could not find a break on the other 3 ExploitBench or 2 Firefox tasks (considered more difficult than the 6 Cybertasks tasks). Anvil finds breaks on all of them.” | images/18-cyber-human-fig5.png | 人类在这 5 个任务上也许没试多久，原文只说 “could not find a break” | 已找到 |
| Cybertasks 124 个任务 Anvil 破 118、旧内部工具 75 | L1 | 同上 | “Given a similar attack budget per task, Anvil breaks 118 out of 124 tasks on Cybertasks where our previous internal red-teaming harness only broke 75 (see Figure 6 below).” 图 6 副标题：“Cybertasks · 124 tasks · three sessions per task, same attacker model” | images/19-cyber-fig6-118of124.png | 对比对象是旧内部工具，不是人类；124 个任务是 Anthropic 内部 Cybertasks eval（非公开）；图上橙线 118 of 124、灰线 75 of 124 | 已找到 |
| 二值反馈下 Cybertasks 破 107（完整反馈 114） | L1（图内标注） | 同上，Figure 11 | 图内文字：Cybertasks · 124 tasks 面板橙线终点 “114”、青线终点 “107”、黑线 “0”；Firefox 12 / 8 / 0；ExploitBench 25 / 17 / 0。正文与图注没有这两个数字 | images/22-ablation-fig11.png | 数字只在图里（官方图原图 `b-fig11.png`）；图横轴只到 200 条消息，与图 6 的 118/124 不是同一设置，不可混用；图图例：Full feedback = 能看到分类器分数、防护细节与目标模型的 thinking；Binary feedback = 只知消息是被放行还是被拦截 | 已找到 |
| 防护让攻击成本最多增加 28× | L1 | 同上 | “they increase the cost for the malicious actors by up to 28× in the number of attacker messages required.” | images/25-cyber-fig4-cost28x.png | 图 4 上倍数分别是 Cybertasks ×2.2、ExploitGym ×16、Firefox ×22、ExploitBench ×28；“up to 28×” 只指最难的 ExploitBench；ExploitBench 中位任务 “2,800 messages”（图上 2,796）；目标是 Opus 5 | 已找到 |

### B4 反蒸馏

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 435 次（1.2 小时）对 102,200 次（27.9 小时） | L1 | https://safeguards.anthropic.com/2026/automated-red-teaming/ | “Against our anti-distillation classifiers, Anvil reached its first substantial reasoning leak after 435 requests (1.2 hours), compared to the 102,200 requests (27.9 hours) that it took expert red-teamers to find a similar jailbreak.”（小数补全：27.9） | images/15-summary-numbers.png、images/20-distill-fig9.png | 对比的是“首个实质性泄露”；“实质性”= 内部泄露评分 ≥70/100 且多位评分者共同认定（“An attack counts as substantial only if it scores at least 70/100 across graders on an internal leakage rubric”）；任务是数学/科学/编程题，题目本身无害（“The problems themselves are harmless”）；Anvil 首个泄露在一道编程题上 | 已找到 |
| 人类总请求数约 204 万；Anvil 约 649 会话、约 13.2 万次 | L1 | 同上 | “During one red-teaming campaign, human experts sent around 2.04 million requests attacking our anti-distillation safeguards via the API. Anvil ran roughly 649 sessions (about 132,000 requests in total) against the same Opus 5.5 two-stage safeguard.” | images/20-distill-fig9.png | 图 9 图例：“Anvil: five runs on 76 tasks, 132,000 requests”；“Expert red-teamers: one of two campaigns, 2.04 million requests”（图示的是两个人类战役之一）。即人类的首个泄露出现在其 204 万次请求中的第 10.22 万次附近（算术：102,200/2,040,000≈5%，推断） | 已找到 |
| 按 435/102,200 算约 1/235 | — | 同上 | 官方文没有写 “1/235”。算术：102,200 ÷ 435 ≈ 234.9（推断，该比值只比较“各自首个实质性泄露所需请求数”） | — | 智东西标题的 1/235 即此（B8）；Anvil 自己总共发了约 13.2 万次请求，比人类首个泄露时的请求数还多 | 部分支持（数字可由原文算出，原文未这样写） |
| 到 102,200 次时 Anvil 已在 76 个目标任务里至少 38 个上泄露 | L1 | 同上 | “by its own 102,200th request, it had discovered successful attacks that leaked the hidden CoT on at least 38 of its 76 target tasks we initially set out to jailbreak.” | images/20-distill-fig9.png | 这是 Anvil 在**它自己的**第 102,200 次请求时的广度，不是时间对齐；两边最严重泄露相近：“the most severe leaks from each campaign were comparable (90 for Anvil and 95 for the human experts), so Anvil’s advantage here is primarily scale and efficiency” | 已找到 |
| 静态漏检率 31.5%→0.4% | L1 | 同上 | “As the classifiers improved, the share of Anvil attacks they missed in our evals fell from 31.5% to 0.4%.”；“A classifier tested and trained on Anvil’s attacks could overfit …”（官方自述）；“Between a third and a half of the attack evaluations in that collection were derived from Anvil”；“A significant share of the attack examples added for the deployed classifier came from Anvil” | images/16-summary-distill-limits.png、images/21-distill-fig8-traffic.png | 官方自承：静态评测里的攻击来自 Anvil 的过往运行，分类器也用 Anvil 攻击训练（“We hillclimbed on static evaluations containing Anvil data while developing these classifiers, so it is not surprising they improved against it.”）；0.4% 是对着 Anvil 数据爬坡后的数 | 已找到 |
| 真实流量：Fable 5.1 33.8%→0（381 次采样）；Opus 5 18.6%→8.1% | L1 | 同上 | “Over the same upgrades, real attackers’ success rate fell on Opus 5 (18.6% to 8.1%) and on Fable 5.1 (33.8% to 0 of 381 sampled attempts). Many other improvements landed over the same period, so we do not attribute this drop to Anvil alone.” | images/21-distill-fig8-traffic.png | Figure 8 图上 Fable 5.1 v4 33.8%、v5 12.9%、v7 0%（蓝带为 95% 区间，v7 处区间贴近 0）；Opus 5 的 18.6% 来自 “Older method, not comparable”（图内标注）；是对一般第一方 API 请求中“提取尝试”成功份额的估计（“we estimate the share of extraction attempts that succeeded”） | 已找到 |
| 自适应 Anvil 仍能突破现行防护 | L1 | 同上 | “substantial leaks found by Anvil in the adaptive setting fell from 79 to 56 of 79 sessions on Opus 5 and from 78 to 56 on Fable 5.1.” 口径：“count how many of the attack sessions (n=79) reach a substantial leak within 200 calls to the guarded model” | images/21-distill-fig8-traffic.png | 即现行（最新）防护下 79 个会话里仍有 56 个在 200 次调用内达到实质性泄露（约 71%，算术，推断） | 已找到 |
| 现行防护比早期版本需要 Anvil 约 3 倍调用 | L1 | 同上 | “our current anti-distillation safeguards take roughly three times as many calls from Anvil to break as the early versions they replaced.”；“from 2,631 to 9,003 on Opus 5.5 (pre-release to current), and from 1,254 to 3,764 on Fable 5.1, 1,045 to 2,977 on Opus 5 and 1,774 to 6,072 on Fable 5 (last probe-only system to current).” | images/26-distill-fig7-3x.png | 图 7 上 Fable 5.1 的 v1 是 2,252，v2–v4 为 1,210/1,197/1,254，v7 为 3,764；正文用 v4 的 1,254 做基线（图标 3.0×）；若以 v1 的 2,252 为基线则约 1.7×（算术，推断）。Opus 5.5 3.4×、Opus 5 2.8×、Fable 5 3.4× | 已找到 |
| 攻击库把评估从约 1 天缩到 3 小时 | L1 | 同上 | “initializing Anvil with the attack library evaluates much faster than running Anvil from scratch – from about 1 day to 3 hours.” | images/26-distill-fig7-3x.png | 79 个会话的固定攻击库 | 已找到 |
| 人类用 API 发了 2.04M 请求这一类“人类方总请求数” | L1 | 同上 | 见上（“around 2.04 million requests”）；扫描称约 204 万，**与原文相符** | images/20-distill-fig9.png | — | 已找到 |

### B5 消融与能力相关

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 外部攻击者只有二值反馈 | L1 | https://safeguards.anthropic.com/2026/automated-red-teaming/ | “External attackers, however, do not see classifier scores and only receive a binary response (blocked or not blocked). In Figure 11, we show that progress can be made on many tasks using only this binary feedback, given access to a strong unsafeguarded attacker model.” | images/22-ablation-fig11.png | “given access to a strong unsafeguarded attacker model” 是前提 | 已找到 |
| 开启攻击方自身防护：97% 的 1,453 个会话被拦，85% 在第一条消息 | L1 | 同上 | “we also measured the aggregate block rate across cyber, anti-distillation and bio, and found that the respective safeguards would block 97% of 1,453 red-teaming sessions, 85% of them at the attacker’s first message. These results suggest it is difficult for an external attacker to use Claude against our own safeguards, and we haven’t observed such attacks in our monitoring.” | images/22-ablation-fig11.png | 97% 是 Anthropic 自家会话的拦截率；“would block” 为假设情形；图 11 图注：“With safeguards on (black), as an external attacker using Claude would have, the attacker model is blocked at its first request on every task.” 图上黑线三个面板全是 0。官方同时说：“an adversary could instead use an open-weight model or one with weaker safeguards.” | 已找到 |
| 图 11 图注（逐字） | L1 | 同上 | “Figure 11: Disabling safeguards on the attacker model is required for Anvil to achieve any meaningful success; beyond that Anvil benefits from internal visibility into our safeguards but can still find breaks without it. Hiding exact classifier scores and other internal details (teal) cost Anvil some breaks and more messages compared with full feedback (orange).” | images/22-ablation-fig11.png | 图内标题与正文图注措辞不同（图内：“Anvil uses internal visibility into our safeguards and an unsafeguarded attacker model to achieve success”） | 已找到 |
| 九个 Claude 模型当攻击方，越强破得越多；相关系数 | L1 | 同上 | 正文：“We observe that our attacker gets better at finding attacks as model capabilities increase.” 图 10 图注：“The graph shows the fraction of 124 tasks in our internal Cybertasks eval broken by Anvil when each of nine Claude models serves as its attacker, plotted against that model’s capability score (Anthropic ECI). The line is a logistic fit with a 95% CI in the gray band.” 图内小字：“Rank correlation (Spearman) 0.92, p = 0.0013, 9 models” | images/23-capability-fig10.png | 0.92 只写在图内（不在正文与图注）；n=9；图内轴标 “AECI”，图注与正文叫 “Anthropic ECI”（“our own version of Epoch AI's public Epoch Capabilities Index”）；点并非单调：Opus 5.5（约 77%）低于 Fable 5.1（约 82%）与 Sonnet 5.5（约 75%）相近；Opus 4.8 高于拟合带 | 已找到 |
| 能力相关的定性补充 | L1 | 同上 | “Qualitatively, we saw that certain classes of jailbreak were not discovered until a given threshold in model capability, which in turn unlocked harder tasks.” | images/23-capability-fig10.png | — | 已找到 |

### B6 局限与“不能全归功于 Anvil”

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 三个特权条件 + 越狱不易迁移 | L1 | https://safeguards.anthropic.com/2026/automated-red-teaming/ | “We believe that the jailbreaks we found represent rare failures of our classifiers which would be difficult for external attackers to achieve today. Specifically, in order to discover jailbreaks on the hardest tasks, Anvil requires a) very strong models (we achieve some of our best results with frontier internal models); b) visibility into exact classifier scores only possible within Anthropic; and c) disabling safeguards on the Claude instance used as the attacker—without which nearly all usage is blocked (Figure 11). Furthermore, Anvil’s jailbreaks are tuned to specific tasks and don’t easily carry over to other tasks in the same domain, which limits how much harm any one of them could do.” | images/16-summary-distill-limits.png | 同一段末尾：“we expect these conditions to shift as open models with weaker safeguards become more competent” | 已找到 |
| 具体分类器没有用 Anvil 训练（生物、网络）；反蒸馏用了 | L1 | 同上 | “While the specific bio and cyber classifiers we study here were not trained with Anvil in the loop, in our anti-distillation work we used static evaluations consisting of Anvil jailbreak transcripts to hillclimb on robustness over several generations of classifiers” | images/16-summary-distill-limits.png | — | 已找到 |
| 真实流量改善不全归功于 Anvil | L1 | 同上 | “Many other improvements landed over the same period, so we do not attribute this drop to Anvil alone.” | images/21-distill-fig8-traffic.png | — | 已找到 |
| 结论里的“不是万灵药” | L1 | 同上 | “Despite our progress, our results are not a panacea. Occasional human jailbreaks may still succeed, and work remains to extend our evaluations to cover more ways that models could uplift real-world attackers. We also expect the ability of external automated attackers to increase (for instance, as open-source models increase in capability), and our classifiers must rise to meet this challenge.” | images/24-conclusion-panacea.png | — | 已找到 |
| 攻击者的回报账：对手可能用别的办法更便宜 | L1 | 同上 | “there is still an open question as to the conditions under which red-teaming pays off for an adversary, who may be able to complete the same harmful task more cheaply through other means, such as models with weaker or no safeguards, specialized tools, or their own expertise.” | images/28-defenders-alternatives.png | — | 已找到 |
| Anvil 的美元成本、总算力、会话数（生物、网络） | — | 同上 | 原文没有美元成本与总算力；只给了消息数（图 4）与小时数；生物与网络的会话总数也未写（反蒸馏写了 649 会话、13.2 万次） | — | 只能比较“消息数”与“墙钟小时”，不能比较花费 | 未找到一手来源 |
| 生物部分是否有“去特权”消融 | L1 | 同上 | “While the figure considers the cyber setting, we also measured the aggregate block rate across cyber, anti-distillation and bio” | images/22-ablation-fig11.png | 二值反馈消融（图 11）只在网络任务上做；生物未写 | 已找到（局限：仅网络） |
| 具体的生物越狱内容、提示词 | — | — | 原文不披露（只给分数） | — | 外部无法复核 | 未找到一手来源 |

### B7 配套论文、Anthropic 主站、作者帖

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| anthropic.com 主站新闻页是否有对应条目 | L1 | https://www.anthropic.com/news 、 https://www.anthropic.com/research （存档 `b-news.txt`、`b-research.txt`） | 两页存档中 grep “Anvil”“automated red” 均无命中；`b-research.txt` 只有 Frontier Red Team 团队链接 | — | 取于 2026-11-14 21:49；页面是服务端渲染的列表，可能未含最新项，不能据此断言“主站没发” | 未找到一手来源 |
| arXiv 配套论文 | — | export.arxiv.org 查询 `all:Anvil AND all:red-teaming AND all:Anthropic` | 查询无结果（输出为空）；官方文只引他人论文（如 arXiv 2602.15001 等），没有给“本文”的 arXiv 版 | — | 未找到 | 未找到一手来源 |
| RSP 路线图原定目标与完成声明 | L1 | https://www.anthropic.com/responsible-scaling-policy/updates （`b-rsp-updates.txt`）与 …/roadmap （`b-rsp-roadmap.txt`） | 目标原文：“We will establish an internal jailbreak-finding method for classifier guards (focused on chemical and biological weapons) that matches or exceeds the current collective abilities of all of our bug bounty participants to find jailbreaks. We will assess this via a report that evaluates whether our internal findings are more substantial in aggregate than our bug bounty findings.” 完成声明：10-19 条（见 B2） | — | 目标只覆盖“化学与生物”分类器；官方文说 “we also set out to improve our red-teaming abilities more broadly”（网络、反蒸馏是额外的） | 已找到 |
| 第一作者之一 Javier Rando 的 X 帖 | L5（作者个人公开帖，Anthropic 员工） | https://x.com/javirandor/status/2120949295818137620 | “Today we’re sharing how we use automated red teaming at Anthropic. We want to find strong jailbreaks early, so we can test and improve our safeguards.” | images/29-x-rando-post.png | 发帖时间 2026-11-13 02:38（北京时间；嵌入卡显示本地时间）= 2026-11-12 18:38 UTC（fxtwitter JSON `created_at` “Thu Nov 12 18:38:12 +0000 2026”）；当时点赞 298、转发 31、回复 25、浏览 15,817（取于 21:5x，抓取时刻见日志）；附图是官方 Figure 1；JSON 存 `b-x-rando.json`（只含该作者公开字段，未含抓取者信息）。**只记作者在 X 的发帖，帖里没有数字之外的新信息** | 已找到 |
| Anthropic 官方 X 帖 | — | — | 未检索到（X 无搜索接口，只通过 Rando 帖与二手来源推知存在作者帖；未发现官方账号帖） | — | 未找到 | 未找到一手来源 |

### B8 热度与中文侧

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| HN 热度 | L6 | hn.algolia.com | `barking heat`：4 组标题/URL 查询均 0 条；按 URL 含 `safeguards.anthropic.com` 查，只出现 10 月《Scaling Misuse Detection with Self-Assessment》两帖（2 分与 1 分），**没有本文的 HN 帖**；按 “automated red-teaming” 查最近故事，只有 OpenAI 11-12 的 “Improving Fuzzy Robustness in LLM Safety with Automated Red Teaming”（1 分，无关） | — | 取于 2026-11-14 21:58–22:01；存 `b-heat.json`、`b-hn-algolia-url.json`、`b-hn-algolia-anvil.json` | 已找到（无热帖） |
| Reddit 热度 | — | — | `barking heat --reddit` 两次失败（连接中断，`io: unexpected end of file`），记入 `b-heat.json` 的 errors；不登录、不重试 | — | 未取得 | 未找到一手来源 |
| 智东西 11-13 原稿 | L4（中文媒体编译） | https://www.163.com/dy/article/LBKGPTI4051180F7.html （网易号“智东西”转载，存档 `b-zhidx163.html/.txt`；凤凰网同稿 https://tech.ifeng.com/c/8y1J7d3PJdD，存档 `b-ifeng.*`） | 标题：“Anthropic新AI红客来了！只用人类1/235的请求量，就找到严重漏洞” 正文首句：“智东西11月13日消息，今日，Anthropic公布了一套名为Anvil的自动化红队测试系统。”（实际官方文页面日期是 11 月 12 日） | 无（中文页版式在 700 px 窗口被裁，未截） | 页面标注 “2026-11-13 15:19:41 来源: 智东西 北京”（北京时间）；编译 黄菲、编辑 江宇；网易页评论数显示 0（跟贴 “0”），未见阅读/转发数（页面不显示） | 已找到 |
| 智东西关键句：24/24、0.5–3.5 小时 | L4 | 同上 | “在网络安全测试中，Anvil成功突破了24组任务与模型组合的安全防护，耗时仅0.5至3.5小时” | — | 与官方正文的 0.5–3.5 一致，与官方摘要的 0.4–4.4 不一致（B3）；智东西也转述了人类一侧的限定：“专家团队报告的11次突破中，有9次借助了安全防护较弱或未启用防护的辅助模型” | 已找到 |
| 智东西关键句：435 对 10.22 万次、204 万、13.2 万 | L4 | 同上 | “在一次信息安全测试活动中，人工专家通过API发送了约204万次请求。Anvil则运行了约649个测试会话，总请求量约13.2万次。” “Anvil在发送435次请求后，就找到了首个实质性推理信息泄露漏洞；人工专家则发送约10.22万次请求后，才发现类似漏洞。” | — | 与官方一致；把 CoT 泄露称为“信息安全测试”“推理信息泄露”，标题称“严重漏洞” | 已找到 |
| 智东西：97%/85% | L4 | 同上 | “如果Anvil攻击模型启用了安全防护，则该安全防护机制拦截红队测试会话高达97%，其中85%在第一条请求时就被拦截。”（官方是 “first message”） | — | 官方是 “1,453 red-teaming sessions”，智东西未写 1,453 | 已找到 |
| 智东西：来自 X 的网友评论（转述） | L6 | 同上 | “有网友担心，自动化红队测试发现的越狱漏洞，也可能暴露Anthropic自身AI训练数据中的缺陷”；“有网友调侃，所谓越狱不过是‘包装得更好听的程序漏洞’” | — | 无链接、无得分，不可核 | 已找到（仅转述） |
| AI Safety HOT 聚合与日报 | L4（聚合站） | https://aisafetyhot.com/en/items/ijqnxdst86epp543s52gc4lao 、 …/zh/daily/2026-11-14 | 条目标题：“Anthropic announced the automated red-teaming system Anvil; in information security tests, the number of requests needed to find a vulnerability was only 1/235 of that for humans”；站内标注 “Indexed by this site 2026-11-13 18:32”“Originally published Nov 13”“Attention 43”；日报 11-14 把它列为头条（“Anthropic 称 Anvil 发现漏洞请求量仅人工 1/235”）并列了 X 上两条英文帖（neubofy 11.13，commanderdgr8 11.13，未存） | — | 转载智东西/凤凰网；“Attention 43”为该站自己的指标，无定义 | 已找到 |
| 英文二手：MIXED News | L4 | https://mixed-news.com/en/anthropic-automated-jailbreak-testing-23-of-24-comparisons/ （`b-mixed.txt`） | “Anthropic says an automated system matched or beat the best human submissions in 24 comparisons designed to test biological-weapons safety classifiers. It won 23 of those comparisons and tied the remaining one.” “The comparison is Anthropic's own assessment.” | — | 页面日期 “Oct 23 2026”，报道的是 10-19 RSP 更新，**早于博客 20 天** | 已找到 |
| 英文二手：AI Hot 评论文 | L6 | https://aihot.tech/columns/ai-table-flip/anvil-safety-results-and-who-can-verify/ （`b-aihot-column.txt`） | “The report supports stronger internal automated red-teaming in tested settings. It does not establish equivalent external attacker capability or immunity to jailbreaks. We have not independently reproduced its results” | — | 页面日期 2026-11-14，编辑评论，非独立复现 | 已找到 |
| 多家中文日报 11-14 | — | — | 搜索只找到智东西→网易/凤凰网同稿与 AI Safety HOT；**未找到其他中文媒体的独立报道**；未取得扫描所说“多家日报”的具体链接 | — | 搜索未穷尽（WebSearch 约 6 次） | 未找到一手来源 |

## 二、可能的吠点（线索摘要没提或说反的条件；L1 来自官方文，L6 为评论线索）

1. 生物 23/24、24–48 小时 10-19 就已在 RSP 更新页公开，11-12 博客是预告的详细版；英文媒体 MIXED 10-23 已报道此数（`b-rsp-updates.txt`；`b-mixed.txt`）。
2. “强于全部赏金参与者合计”比的是 8 个固定问题、3 代分类器、模型打分的严重度（一位 Anthropic 生物学家校验）；官方文也承认赏金“不能很好代表有动机的对手”（“we’re increasingly skeptical of how well these programs model a motivated adversary”）。
3. Anvil 用的是“frontier internal versions of Claude with safeguards disabled”，并能看到精确的分类器分数；外部攻击者只拿到二值反馈。官方自己写 “nearly all usage is blocked” 当攻击方的防护打开时（图 11 黑线三面板均为 0）。
4. 外部专家的“11 组成功”里 9 组靠防护较弱的辅助模型，不符合 Anvil 的更严标准；专家会话是人加 AI 代理协作（官方文）。同一对比里，两边的“成功”定义不同。
5. 人类“0 个”不准确：对 Fable 5.1 的 11 个潜在越狱无一端到端；对 Opus 5.5 的 13 个里有 1 个完整利用（官方文，B3）。
6. 耗时的数字在官方文里不一致：摘要 0.4–4.4 小时，正文 0.5–3.5 小时，Figure 5 图上为 0.39–4.4 小时（B3）。
7. “28×”只指最难的 ExploitBench；Cybertasks 只有 2.2×（Figure 4）。
8. “1/235”比的是首个实质性泄露所需请求数（435 对 102,200）；Anvil 自己总共发了约 13.2 万次请求，人类这次战役约 204 万；两边最严重泄露相近（90 对 95）；泄露的是无害题目上的隐藏推理（官方文）。
9. 反蒸馏的静态 31.5%→0.4% 是对着 Anvil 数据爬坡训练的结果；自适应评测里现行防护下仍有 56/79 个 Anvil 会话在 200 次调用内达到实质性泄露（Figure 8；约 71% 为算术，推断）。
10. 真实流量 33.8%→0 是 381 次采样；官方明确“同期还有其他多项改进，不全归功于 Anvil”。Opus 5 的 18.6% 在图里标注 “Older method, not comparable”。
11. “约 3 倍”：Fable 5.1 的基线用了 v4（1,254），v1 为 2,252（图 7；以 v1 为基线约 1.7×，算术，推断）。
12. 0.92 相关系数只写在图 10 图内，n=9，Opus 5.5 低于 Fable 5.1（图 10）。
13. 没有美元成本、总算力；生物与网络的会话总数未写；二值反馈消融只在网络任务上做（图 11）。
14. 赏金人类成果在下降（2025 年 8 个月 $1.1M/95 份，2026 至今 $190k/20 份），官方用来说“人类在减速”，未写原因（官方文）。
15. 社区评论线索：HN 上无本文帖，未取得带链接与得分的社区质疑（L6 线索空缺）。智东西转述的“网友”评论无链接。

## 三、夸大说法实例

找到 1 个“偏离原文”的中文标题实例（其余未找到）：

| 站点 | URL | 原句逐字 | 发布时刻 | 发布方 | 与原文的出入 |
|---|---|---|---|---|---|
| 网易号“智东西”（同稿凤凰网、AI Safety HOT 转载） | https://www.163.com/dy/article/LBKGPTI4051180F7.html | 标题：“Anthropic新AI红客来了！只用人类1/235的请求量，就找到严重漏洞”；小标题：“三、AI红队请求量仅人类的1/235，推理信息泄露率最低降至0%” | 2026-11-13 15:19:41（北京时间，页面标注） | 智东西（编译 黄菲、编辑 江宇） | “1/235”官方文没写，是 435/102,200 的算术比，且只比各自首个实质性泄露；“严重漏洞”对应的是无害题目上的 CoT 泄露；两边最严重泄露分数相近（90 对 95）；“推理信息泄露率最低降至0%”指的是 Fable 5.1 真实流量里 381 次采样 0 次成功，官方同时说同期还有其他改进、不全归功于 Anvil；标题不含“内部模型、关掉防护、特权分数”等前提（正文末尾有一句 97% 的拦截率） |

英文媒体里没有找到把结果写歪的标题；MIXED News 的标题 “Anthropic says its automated jailbreak tester beat human entries in 23 of 24 tests” 与官方一致。AI Safety HOT 把 10-19 公开的生物结果与 11-12 的博客一起当作“11-13 新闻”，属日期上的混淆（只作线索）。

## 四、扫描说法勘误

| 扫描说法 | 官方原文 | 结论 |
|---|---|---|
| 网络 6 任务×4 模型共 24 组全部攻破，每个 0.5–3.5 小时 | 正文：“Anvil found legitimate breaks for all these task-model pairs (24 of 24)”；“Anvil found the breaks in 0.5–3.5 hours”；摘要：“in 0.4–4.4 hours each”；Figure 5 图上 0.39–4.4 小时 | 24/24 正确；“每个 0.5–3.5 小时”只与正文一句相符，摘要与图是 0.4–4.4；官方文自相矛盾 |
| Anvil 在一周内完成 | 原文没有 | 未找到 |
| 人类 0 个 | Fable 5.1 上 11 个潜在越狱无一端到端；Opus 5.5 上 13 个含 1 个完整利用；专家报 11 组成功但 9 组靠辅助模型 | 不准确（只对 Fable 5.1 的端到端成立） |
| 二值反馈下 Cybertasks 仍破 107（完整反馈 114） | 只在 Figure 11 图内：114 与 107（及 Firefox 12/8、ExploitBench 25/17）；正文无 | 数字正确，仅图内；与 118/124（图 6）不同设置 |
| 秩相关 0.92 | 只在 Figure 10 图内：“Rank correlation (Spearman) 0.92, p = 0.0013, 9 models” | 正确，仅图内 |
| 人类约 204 万次请求 | “around 2.04 million requests”；图例 “one of two campaigns, 2.04 million requests” | 正确（且是两个战役之一） |
| 27.? 小时 | 27.9 小时（摘要与图 9）；引言里写 “over 27 hours” | 补全为 27.9 |
| “1/235”出自智东西标题 | 智东西标题确有；官方文无；435/102,200≈1/234.9 | 正确；算术比 |
| 生物“23/24、24–48 小时”是新结果 | 10-19 RSP 更新页已公开 | 勘误：非首次公开 |
| 页面日期 11 月 12 日 | 正确；智东西称“今日（11 月 13 日）公布”，是中文媒体的日期 | 一致 |
| 英文侧冷、HN 无热帖 | HN 无本文帖（只有 10 月自评帖 2 分）；Reddit 查询失败 | 一致（HN）；Reddit 未取得 |
| 多家中文日报 11-14 | 只找到智东西同稿与 AI Safety HOT | 未找到其他独立报道 |
