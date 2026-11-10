# 1109 期 B 组取证：Fable 5.2 发布的费马与黑洞两项科研成果

执行：Sonnet 5.5（claude-sonnet-5-5）。时间一律北京时间（UTC+8，括号内为原时区）。存档文件在 `docs/2611/1109/sources/b-*`；截图在 `docs/2611/1109/images/13–24`。口径：下表“级”指事实分级 L1–L7；“状态”只用已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

本组一句话结论（取证事实，不是正文结论）：发布页把“30 小时”“无数学参考”“Fable 5.2”写得比系统卡和 claude.dev 更干净；系统卡与 claude.dev 自己承认：跑的是 Fable 5.2 的“早期版本/earlier snapshot”，团队拿到了 47 条前次已证引理与一句话证明路线，第 18 小时人工从存档续跑；两篇黑洞预印本没有写“Fable 5.2”，只写“内部 Claude 研究模型”，且发在不做同行评审的新站 Hexagon，不在 arXiv。

## 发布时刻

| 对象 | 时刻（北京） | 原时区与出处 | 备注 |
|---|---|---|---|
| Anthropic 发布页 | 官方未给时刻；页面日期 November 9, 2026 | 页面 `November 9, 2026`；sitemap `lastmod` 2026-11-09T19:40:53Z（= 北京 11-10 03:40:53） | lastmod 是最后修改时刻，不等于发布时刻 |
| HN 首帖 50429068 | 11-10 02:01:10 | 2026-11-09T18:01:10Z（HN Algolia 条目 `created_at`） | 标题 “Claude Fable 5.2 and Mythos 5.2”，Algolia 记 514 分（取于 16:2x） |
| 系统卡 | 官方未给时刻；封面 November 9, 2026 | PDF 封面 | |
| claude.dev 博文 | 官方未给时刻；PUBLISHED Nov 09, 2026 | 页面元数据 | |
| 两篇黑洞预印本（Hexagon） | 11-07（日期；PDF 元数据创建时刻 174：11-07 09:26 +08，175：11-07 10:11 +08） | Hexagon 页 “Version 1 / Nov 07, 2026”；PDF 元数据 `CreationDate … +08` | 早于发布页两天；PDF 元数据时刻只说明文件生成，不等于上架时刻 |
| 9 月原文 | 官方未给时刻；Published Sep 4, 2026 | 页面 | 0924 期已写过 |
| Gizmodo 报道 | 11-10 03:32 | Published November 9, 2026, 2:32 pm ET | 见 B6 |
| 新智元（凤凰网转载）稿 | 11-10 06:41:33（更新 06:50:18） | 页面“2026年11月10日 06:41:33 来自北京” | 见 B7 |

## B1 费马：30 小时与 28.7 小时；“1/3 token”“证明长度 1/7”；与 9 月配置差异

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1a “1,000 个 Fable 5.2 agent 30 小时完成形式化，人工干预极少” | L1 | https://www.anthropic.com/claude-fable-and-mythos-5-2 （`b-release.html/.txt`） | “By contrast, a team of 1,000 Fable 5.2 agents fully formalized the theorem in just 30 hours, with minimal human intervention.” | `13-release-fermat-text.png` | 发布页写 “Fable 5.2”；claude.dev 与系统卡写的是早期版本，见 B1e | 已找到 |
| B1b 28.7 小时是完成时刻，30 小时是运行预算 | L1 | https://claude.dev/blog/safely-scaling-multi-agent-teams-with-fable-5-2/ （`b-claudedev.txt`）；系统卡 §8.12.3 | claude.dev：“Langlands–Tunnell was proved at hour 28.4, followed by the final theorem at hour 28.7.”；“The team took 28.7 hours”；图 4 说明：“Activity stops near hour 18, when we restarted the run from its last save and gave the agents 12 more hours.”系统卡：“We then resumed each team’s work with an additional 12-hour budget. By hour 30, the earlier snapshot’s team had a complete Lean proof”；发布页图注：“over 30 hours … At the dashed line (18 hours), both teams resumed from their last save with 12 more hours.” | `16-claudedev-hour287.png`、`14-release-fermat-chart.png`、`24-syscard-fermat-1000.png` | 独立复核结论：30 = 18 + 12 的预算；28.7 在第二段内。不矛盾。两点补充（取自原文，不是矛盾）：①第 18 小时是人工“restarted … from its last save”（发布页“minimal human intervention”未提这次续跑）；②发布页图上 28 小时点约 0.87，30 小时点 1.00，说明 28.7 与图一致；28.7 是否含续跑的间隔，原文未说 | 已找到 |
| B1c “一半以上 token / 证明短 7 倍”原句与分母 | L1 | 发布页；claude.dev | 发布页：“It needed one-third as many action tokens as our first formalization and produced a proof seven times shorter.” claude.dev：“The team took 28.7 hours, used one-third as many output tokens, and wrote a proof one-seventh the length of the original.” | `13-release-fermat-text.png`、`15-claudedev-intro.png` | 分母=9 月那次（“our first formalization” / “the original”）。措辞差异：发布页 “action tokens”，claude.dev “output tokens”，各自未定义“action tokens”。9 月原文给出的基数：“about six billion output tokens”“13 million lines of Lean”（`19-sept-original.png`）。按这两个数推算新一次约 20 亿 token、约 190 万行（**推算，官方未写**）。新一次起点含 47 条前次已证引理（见 B1e），两次不是同起点比较 | 已找到 |
| B1d 9 月配置：11 天、Prove2Me、研究员指导 | L1 | https://www.anthropic.com/research/formalizing-fermats-last-theorem （`b-fermat-sep.txt`） | “Claude worked largely autonomously over 11 days”；“Mathematical input from humans was limited to occasional high-level instructions from Tianyi”；“With Prove2Me and a Claude Code-based multi-agent harness, a team of agents completed the proof in a little under two weeks, consuming about six billion output tokens from a general-purpose internal research model roughly comparable to Claude Fable 5.1.”；“Dozens of Claude agents collaborated” | `19-sept-original.png` | 9 月页 Published Sep 4, 2026。发布页称 9 月那次 “performed by an unreleased research version of Claude … It relied on a purpose-built harness along with guidance from a human researcher”；9 月页自己写“限于偶尔的高层指令”，两处措辞侧重不同，不矛盾 | 已找到 |
| B1e 本次起点：发布页“no mathematical references, only a list of intermediate theorem statements” | L1 | 发布页；claude.dev；系统卡 | 发布页：“Fable 5.2 started from minimal prompts, built its own ways to coordinate, and used no mathematical references, only a list of intermediate theorem statements to score progress.” claude.dev：“We gave the agents a list of 60 statements, a one-sentence sketch of the classical route, the definitions those statements use, and 47 proved lemmas from our earlier formalization, and assigned lead and helper roles.” 系统卡：“The agent teams started with definitions and supporting lemmas from our earlier formalization.”；执行摘要：“building on definitions and smaller proven results from an earlier formalization” | `13`、`15` | 三处原文口径不同。claude.dev 和系统卡都写了“前次形式化的已证引理/定义”，发布页只写“只有中间定理陈述”。claude.dev 给出的数：60 条陈述（含最终定理与 59 个里程碑）、47 条已证引理（与下文“47/60 被判不可行”的 47 是两个不同的 47） | 部分支持（发布页措辞与另两处不一致，需并列） |
| B1f 版本口径：Fable 5.2 与“早期版本/earlier snapshot” | L1 | 发布页；claude.dev；系统卡 | claude.dev：“a team of 1,000 agents running an early version of Claude Fable 5.2”。系统卡：“one team of 1,000 agents with an earlier snapshot of Fable 5.2”；图例 “Claude Fable 5.2 (earlier version)”；执行摘要 “using an earlier snapshot of the model”。发布页：正文 “1,000 Fable 5.2 agents”，图例 “Fable 5.2”（无 earlier 字样） | `14-release-fermat-chart.png`、`24-syscard-fermat-1000.png` | 发布页图例未带“earlier version”，系统卡同一张图带了；发布页在导语 “During prerelease testing …” 下说明是发布前测试 | 已找到（发布页未标“早期版本”） |
| B1g 本次属于“复现”，不是新证明 | L1 | claude.dev；发布页 | claude.dev：“reproduced the proof without the scaffolding provided by Prove2Me or steering from a researcher”；“This toy experiment suggests …”；“This result doesn’t obviate the need for domain-specific harnesses or platforms.”发布页：“fully formalized the theorem” | `15-claudedev-intro.png` | 见 B4 | 已找到 |

## B2 Opus 5.5 对照：单次运行、18 小时处续跑、0.26/0.27

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B2a 发布页图：单次运行；18 小时处续跑；Fable 1.00；Opus 0.26/0.27 | L1 | 发布页 | 图注：“Each line shows the proportion of the full formalization that Fable 5.2 (green line) and Opus 5.5 (orange line) had completed over 30 hours with about 1,000 agents, and a single run per model. At the dashed line (18 hours), both teams resumed from their last save with 12 more hours.”正文：“Opus 5.5 (with the same 1,000-agent setup) completed only 26% of the formalization in 18 hours, and barely made any more progress after 30.” | `14-release-fermat-chart.png`（`13` 含正文） | 图上标注点：Fable 5.2 在 18 小时处 **0.86**、30 小时处 1.00；Opus 5.5 在 18 小时处 0.26、30 小时处 0.27。读数（目视）：Fable 约 0.02、0.05（<1 小时）、0.21（2 h）、0.31（4 h）、0.48（6 h）、0.71（8–12 h）、0.86（14–22 h）、0.87（24–28 h）、1.00（30 h）；图中是每 2 小时一个点，caption 称 “line”。注意：正文只写 Opus 的 26%，没写 Fable 在 18 小时时是 0.86（只在图上标出），Fable 的第 18→28 小时基本在 0.86–0.87 平台，最后 2 小时跳到 1.00 | 已找到 |
| B2b 系统卡对同一实验的说明 | L1 | https://www.anthropic.com/document/claude-fable-5-2-mythos-5-2-system-card （`b-syscard.pdf/.txt`，PDF 第 225–227 页） | “We also ran one team of 1,000 agents with an earlier snapshot of Fable 5.2, and one with Opus 5.5. After 18 hours, the earlier snapshot’s team had reached 0.86, compared with 0.26 for the Opus 5.5 team. We then resumed each team’s work with an additional 12-hour budget. By hour 30, the earlier snapshot’s team had a complete Lean proof of Fermat’s Last Theorem that our grader accepted (Figure 8.12.3.B).”图 B 说明：“Each line shows the best score one team had reached each hour, from a single run per model.” | `24-syscard-fermat-1000.png` | 系统卡原图是阶梯线（best score so far）；发布页图是点。同一实验，单次运行（n=1 每模型）。系统卡另有 ≤100 agents 的 3 次平均：Fable 5.2 单 agent 0.12、100 agents 0.35；Opus 5.5 单 agent 0.09、100 agents 0.21（18 小时内最佳分，3 次平均）；图注：“Fable 5.2 ran one week after Opus 5.5, with a shorter task brief. This task uses a different theorem from the Lean task in the Claude Opus 5.5 System Card, so the scores are not comparable across the two system cards.” | 已找到 |
| B2c “同样的 1,000 agent 配置”是否严格同条件 | L1 | 同上 | 发布页：“(with the same 1,000-agent setup)”；系统卡：“Fable 5.2 ran one week after Opus 5.5, with a shorter task brief”（指 ≤100 agent 那组实验的任务说明） | — | 系统卡没有单独说明 1,000 agent 对照里 Opus 5.5 拿到的任务说明是否同一份；只能说发布页称“same setup”，系统卡对相邻实验说明了任务说明不同。查不到更多 | 部分支持 |

## B3 claude.dev 自曝的限制

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B3a 放弃难题、误判难度：47/60 | L1 | claude.dev | “It then set aside the 47 statements that most helpers had judged to be infeasible within the run’s budget and reassigned nearly all of the helpers to the remaining 10, which it thought they could finish.”“Early on, helpers judged 47 of the 60 statements infeasible within the run’s budget, yet the team went on to prove most of them.”“Agents tend to give up on hard problems and misjudge their difficulty.”领导者当时的话：“the team’s throughput is far higher than the early ‘infeasible-in-budget’ verdicts assumed” | `17-claudedev-limits.png` | 自己数：47/60 ≈ 78.3%；留下 10 条（47+10=57，另 3 条在 27 分钟时已证）。“Most of them”未给具体数；图 6 说明“The count ends at 54 of 60 statements because five alternatives were no longer needed and one statement was dropped.” | 已找到 |
| B3b 最难一块被队长判“<1% 能做完”，领导者说“Honest odds are low” | L1 | claude.dev | “the captain of the Langlands–Tunnell task had been pessimistic, estimating that the proof would take at least 0.7 to 1 million lines of Lean code, with less than a 1% chance of finishing in time. The lead set the statement aside.”；“it moved about 750 helpers onto the statement and wrote, “Honest odds are low.””；任务树“ultimately reaching 2,418 tasks up to 10 levels deep” | （`v17c` 在 scratchpad，未入编号） | 数：2,418 任务、10 层、约 750 helper | 已找到 |
| B3c 监控：本次只有基于动作的监控；CoT 监控“此次运行后不久才加” | L1 | claude.dev | “In the run in this post, we used only the action-based monitor.”“Shortly after this run, we added the parallel CoT monitor.”“We run experiments like this on models that may not have completed our pre-release alignment assessment, so we design our monitoring for the possibility that a model is misaligned.” | `18-claudedev-monitor.png` | 同段：CoT 监控在红队评测里“able to detect all sabotage attempts”（那是另一组评测，不是本次运行）；“Only a block on the lead would have ended the run” | 已找到 |
| B3d “超过一百万次工具调用零次达到拦截标准”与唯一接近的事件 | L1 | claude.dev | “Across more than a million tool calls, none in this run met the bar to be blocked. When we replayed these calls against the CoT monitor and today’s rubric, only one, a check for root access, came close to being blocked.” | `18-claudedev-monitor.png` | “more than a million”是原话，未给精确数；“接近拦截”的事件原文只说是一次 root 权限检查，未给细节 | 已找到 |
| B3e 其他自述限制 | L1 | claude.dev | “A single shared machine hit its limits at about 100 agents: more than half of our 100-agent runs had to be stopped or restarted because memory, disk, or CPU ran out.”“This result doesn’t obviate the need for domain-specific harnesses or platforms. Prove2Me, for example, may still improve efficiency …”“toy experiment” | `15`、（`17` 末段） | “toy experiment”是 claude.dev 自称 | 已找到 |

## B4 是复现已有证明的形式化，不是新证明；Lean 产物是否公开

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B4a 复现、起点含中间定理列表与前次引理 | L1 | claude.dev；发布页；9 月页 | 见 B1e、B1g。9 月页：“what’s novel here is the verification—checking a mathematical proof as one would check a mathematical computation with a calculator.”发布页：“Formalization is the process of translating a mathematical proof into a language that a computer can check step by step.” | `13`、`15`、`19` | 定理是 Wiles 1995 年证明的；9 月页原话：“Claude’s proof follows a simplified version of Wiles’s proof from Darmon, Diamond, and Taylor”（`19`）；本次是否沿用同一路线，claude.dev 只说给了“a one-sentence sketch of the classical route” | 已找到 |
| B4b 本次 1,000-agent 运行的 Lean 产物是否公开 | L1 | https://github.com/anthropics/fermats-last-theorem （GitHub API；`b-fermat-repo-readme.md`） | README：“Research artifact. Not maintained and not accepting contributions.”（9 月那次的仓库） | — | 仓库创建 2026-09-04T14:21Z，最后推送 2026-09-24T14:06Z，早于本次发布；发布页、claude.dev 均无本次 Lean 产物的链接（`b-claudedev.html` 里仅有 Mathlib 链接）。仅查了该仓库与一次 GitHub 仓库搜索，未穷尽 | 未找到一手来源（本次产物未公开） |

## B5 黑洞两篇预印本

发布页原文：“Read more about these results in their respective preprints, here [https://hexagonmath.org/2611.00175v1] and here [https://hexagonmath.org/2611.00174v1].”**两个链接都不是 arXiv，而是 Hexagon（hexagonmath.org）。**发布页顺序：先 175（Hawking-Page，对应“first problem”）、后 174（模块化自举，对应“second problem”）。

| 项目 | 175：The Hawking-Page transition at weak coupling | 174：The spinless modular bootstrap at large central charge |
|---|---|---|
| 标识 | hexagon:2611.00175v1；DOI Not assigned；CC BY 4.0 | hexagon:2611.00174v1；DOI Not assigned；CC BY 4.0 |
| 日期 | Version 1 / Nov 07, 2026 | Version 1 / Nov 07, 2026 |
| 作者（页面） | “Contributed by Anthropic”；BibTeX “Contributors: Anthropic”；PDF 署名 “Claude (Anthropic)”；history “Deposited by Anthropic”。**没有任何具名人类作者** | 同左 |
| 页数 | 42 页，参考文献 120 条 | 131 页，参考文献 85 条 |
| 标签 | “Primarily AI-generated text”“Human understanding: some parts” | 同左 |
| 来源声明（provenance，逐字） | “An internal Claude research model, prompted by a physicist, obtained the result presented here. The physicist checked it against several known special cases. A separate Claude computation using a different method, not included here, gives the same result.” | “The main result was first obtained autonomously by an internal Claude research model. The publicly available Claude Opus 5.5 later obtained the same result independently. Claude then wrote this paper, following brief guidance from a physicist, who also checked every main result.” |
| 是否写 “Fable 5.2” | 否（全文无 Fable/Mythos 字样） | 否（只有 “internal Claude research model” 与 “Claude Opus 5.5”） |
| 同行评审 | 页面与 PDF 均未声称经过评审；Hexagon 自述不做同行评审（见下） | 同左 |
| 结果的范围（原文） | “We do not show that the first-order line continues unbroken to strong coupling. We show only that it begins as a first-order line at weak coupling. We work at leading order in 1/N and to order λ².”“The computation was carried out by an AI system.”“One finite counterterm needed for the two-loop Hagedorn check, which does not enter b, is fixed by fifteen exact conditions but has not been derived analytically.”摘要：“We find b = −1.48×10⁻³ λ² … The transition is first order.”“almost every number in it was obtained in two independent ways.”附录 C：工作由“many instances of the same AI system”分担，一个实例协调 | 摘要：“We show that it is exactly Λ_V = 12π²/(π² + 4 arcsinh² 1) ≈ 9.1266. No linear functional in a class that contains all finite combinations of derivatives improves on c/Λ_V by more than O(c^{1/3} log^{2/3} c)”；“We also conjecture the coefficient of the c^{1/3} correction …, supported by numerics up to c = 2048.”表 1：Λ_V = 9.1266 状态 “proved, computer-assisted”；“our extrapolation, 24 ≤ c ≤ 2048: 9.1263 ± 0.002 … numerical”；“no functional of any kind beyond 12: proved”；对前人结果：“we have read the statements and the structure of that argument but have not independently verified it” |
| 截图 | `21-hexagon-hawking-page.png` | `22-hexagon-modular-bootstrap.png` |
| 存档 | `b-bh-175.html/.txt`、`b-bh-175-pdf.pdf/.txt`、`b-bh-175-files.*` | `b-bh-174.html/.txt`、`b-bh-174-pdf.pdf/.txt`、`b-bh-174-files.*` |

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B5a 两篇预印本存在，主题与发布页描述对应 | L1（发布页链到）；L3（Hexagon 记录页，托管方自己的元数据） | https://hexagonmath.org/2611.00175v1 ；https://hexagonmath.org/2611.00174v1 | 见上表 | `20-release-blackhole-text.png`、`21`、`22` | 175 对应“1983 年…黑洞像水沸腾般突然形成…2003 年列出的计算”（摘要：“it was suggested in 2003 …”）；174 对应“the exact limit of a technique … more than 15 years … rule out candidate theories of quantum gravity in a simplified universe”（摘要“spinless modular bootstrap … two-dimensional CFT”） | 已找到 |
| B5b 预印本把结果归给 “Fable 5.2” | L1 | 同上 | 发布页：“Claude Fable 5.2 solved two open problems in theoretical physics.”“Fable 5.2, working largely autonomously, completed it”预印本 provenance：“An internal Claude research model”（175）；“first obtained autonomously by an internal Claude research model. The publicly available Claude Opus 5.5 later obtained the same result independently”（174） | `20`、`21`、`22` | 预印本没有点名 Fable 5.2。“internal Claude research model”是不是 Fable 5.2 的早期版本，原文没说，**无法核实**。174 还写明公开可用的 Opus 5.5 后来独立得到同一结果。175 写 “prompted by a physicist … checked it against several known special cases”，发布页写 “working largely autonomously”，侧重点不同 | 部分支持（模型归属不能由预印本核实） |
| B5c 声明未经同行评审 | L3（托管站自述）；L4（Gizmodo） | https://hexagonmath.org/ （`b-hexagon-home.txt`）；https://hexagonmath.org/about；https://gizmodo.com/anthropics-says-its-new-ai-model-can-solve-black-hole-mysteries-what-could-go-wrong-2000837274 （`b-gizmodo.txt`） | Hexagon 首页：“Hexagon does not perform peer review. Every deposit is screened automatically and, during the beta, also reviewed by a human moderator before it appears; acceptance is not a judgment of correctness, quality, or novelty.”about：“Acceptance of a work represents a decision to disseminate and preserve the work, and is not a judgment about its correctness, quality, or novelty.”“Artificial-intelligence systems cannot be contributors.”“Code such as Lean formalizations is not accepted as part of a Hexagon submission.”Gizmodo：“The company has published research papers detailing both findings, though the results have yet to undergo independent peer review.” | `23-hexagon-no-peer-review.png` | 两篇预印本本身没有写“未经同行评审”字样；该说法来自托管站政策（站点不做评审）与 Gizmodo。Hexagon 咨询委员会名单里有 Kevin Buzzard（`b-hexagon-who.txt`），此人是 9 月费马稿里评价过 Anthropic 形式化的数学家；只记录事实，不作推断。`about`：“Hexagon was created in response to the staggering changes that artificial intelligence has induced in the mathematics research landscape”，站为公测（beta）、由 Hexagon Mathematics Foundation 管理 | 已找到 |
| B5d 第三方复算或评论 | L6 | HN https://news.ycombinator.com/item?id=50432604 （3 分 2 评，11-10 05:38，标题 “Quantum Gravity Unites Relativity and Quantum Physics”，链到 174）；HN 评论 https://news.ycombinator.com/item?id=50430705 | HN 50430705：“Interesting that the papers list no authors other than Claude. Also interesting that they published on "Hexagon" instead of Arxiv.” | — | 没有查到任何物理学家的第三方复算或评论；174 自称 Opus 5.5 “独立”复现是 Anthropic 自己的运行。只检索了 HN（Algolia）与几次网页搜索，未检索 X、物理博客、InspireHEP 引用，查不到不等于不存在 | 未找到一手来源（第三方复算查不到） |
| B5e 发布页“first problem”描述与预印本摘要的差别 | L1 | 发布页；175 摘要 | 发布页：“found the transition is abrupt there, too”；175 摘要：“Does the transition remain first order at weak coupling, where there is no classical geometry? … The transition is first order.”；“We show only that it begins as a first-order line at weak coupling … leading order in 1/N and to order λ².” | `20`、`21` | 发布页的“very small universes where only the quantum description can be used”对应摘要的“weak coupling”；预印本限定为 N=4 SYM 在 S³ 上、1/N 领头阶、λ² 阶，并明说“not shown”强耦合延续 | 部分支持（限定词在预印本里，发布页没提） |
| B5f 发布页“exact limit of a technique”描述与 174 摘要的差别 | L1 | 发布页；174 摘要 | 发布页：“found the exact limit of a technique physicists have used for more than 15 years to rule out candidate theories of quantum gravity in a simplified universe.” 174：“these bounds read Δ1 ≤ c/Λ + o(c) at large c … We show that it is exactly Λ_V ≈ 9.1266. No linear functional in a class that contains all finite combinations of derivatives improves on c/Λ_V by more than O(c^{1/3} log^{2/3} c)” | `20`、`22` | 限定：无自旋模块自举、大 c、Virasoro、一类线性泛函；“no functional of any kind beyond 12”是已知上界；c^{1/3} 项系数只是猜想；Λ_V 的证明标注 computer-assisted | 部分支持 |

## B6 “technical note 写 preprints, not peer-reviewed”出处；Reuters 等对“solved”的转述

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B6a 扫描称“同一天的技术说明写 preprints, not peer-reviewed theorems” | — | 发布页、claude.dev、系统卡 grep（“peer”“preprint”） | 发布页只有一处 “respective preprints”；claude.dev 无 “preprint”/“peer”；系统卡 grep “peer review” 与黑洞无关 | — | 在三份官方文本里**没有**找到 “not peer-reviewed” 这一表述，发布页只写 “preprints”。 | 未找到一手来源 |
| B6b Reuters/AOL 稿“结果尚未经独立同行评审” | L4 | 未找到 | — | — | 用 WebSearch 查了英文与中文关键词，没有命中 Reuters 或 AOL 稿；检索工具对 11-09/10 当天新稿覆盖有限，不等于不存在 | 未找到一手来源 |
| B6c 其他媒体对“solved”的转述（Gizmodo） | L4 | https://gizmodo.com/anthropics-says-its-new-ai-model-can-solve-black-hole-mysteries-what-could-go-wrong-2000837274 （`b-gizmodo.txt`） | “using its new models to crack two questions about the physics of black holes—first finding that black holes form as abruptly in small universes as they do in large ones, and then supposedly finding the limit of a technique physicists use to rule out theories of quantum gravity in a simplified universe. The company has published research papers detailing both findings, though the results have yet to undergo independent peer review.”；费马：“a team of about 1,000 Fable 5.2 agents managed to translate the existing mathematical proof into a computer-verifiable form in just 30 hours”；“The company says the new attempt required minimal human intervention” | — | Gizmodo 2026-11-09 2:32 pm ET（北京 11-10 03:32），作者 AJ Dellinger；转述较克制，用了 “supposedly”“claims”“translate the existing mathematical proof” | 已找到（L4，⚠️） |

## B7 夸大实例（中文稿，原站 URL、原句、时间、发布方）

| 例 | 发布方 | URL | 原句（逐字） | 时间（北京） | 与原文的差异 |
|---|---|---|---|---|---|
| 1 | 新智元（凤凰网转载） | https://tech.ifeng.com/c/8xugZ5rMQG3 （`b-ifeng-zuiqiang.txt`） | 正文：“在 30 小时内，第一次独立地彻底证明了费马大定理，还完成形式化验证！”；标题“最强Fable 5.2震撼发布！费马大定理30小时形式化，全面碾压Opus 5.5”；“他们给了1000个 Fable 5.2 智能体最基础的prompt，不提供任何数学参考资料。” | 11-10 06:41:33（更新 06:50:18） | 官方说的是把已有证明“形式化”，不是“证明”费马大定理；“不提供任何数学参考资料”照搬发布页，没提 claude.dev 写的 47 条前次引理与一句话路线；“第一次独立地”无出处。同稿标题用“形式化”，正文用“证明”，前后不一致 |
| 2 | 凤凰网（原创栏） | https://original.ifeng.com/c/8xugZ5rMQGu （`b-ifeng-zhengming.txt`） | 标题“Anthropic发布Claude Fable 5.2：30小时证明费马大定理、调度千个Agent做系统，但高昂定价和官方保守建议引发“数字劳动力”讨论”；正文“在 30 小时内，完成了费马大定理的形式化验证” | 11-10 12:49:31（更新 12:54:51） | 标题写“证明”，正文写“形式化验证”；同一页内标题与正文不一致 |
| 3 | 新智元官方微博（新浪新闻转载页） | https://www.sina.cn/weibo/detail/5363635000181477.html （`b-sina-huojin.txt`） | 页面标题“霍金题困住物理学家二十多年，Claude 30 小时算完”；微博正文：“霍金的黑洞题，Claude补完了。1983年霍金证明宇宙加热时黑洞会突然冒出来，但只在大宇宙成立；小宇宙的算式2003年就列好，二十多年没人算完。Anthropic说Fable 5.2基本靠自己算完了，物理学家核对过特例。” | 11-10 09:21 | 把“30 小时”（费马形式化运行预算）安到黑洞计算上，官方没有给黑洞计算的用时；“霍金题”“霍金证明”：发布页只写 “physicists in 1983”，175 引用的是 Hawking 与 Page；官方原文是 “working largely autonomously”，“物理学家核对过特例”只见于 175 的 provenance，174 写的是 “checked every main result”；标题与微博正文的措辞有出入 |
| 4 | 网易订阅（163） | https://www.163.com/dy/article/LBCC46GI05119734.html （`b-n163.txt`；未逐段读） | 标题“Fable 5.2登场：解出两道黑洞物理难题，30小时完成费马大定理形式化” | 页面时间未读取 | “解出”对应发布页 “solved”；未与原文对照到逐句，只作标题例；比例 2 的写法准确（“形式化”） |
| 5 | TVBS（台湾） | https://tech.tvbs.com.tw/tech/4048331 （`b-tvbs.txt`） | “AI 30小時完成費馬最後定理形式化” | 页面时间未读取 | 这个例子措辞准确（形式化），可作对照，不是夸大 |

补充：搜索结果中还见到“费马大定理30小时形式化，全面碾压Opus 5.5”（例 1）与新浪“刚刚，Claude Fable 5.2炸场！30小时完成费马大定理形式化”（`b-sina-zhachang.txt`，页脚时间 11-10 16:19，应为抓取时刻，不是发布时刻；标题用“形式化”，不算夸大）。英文侧只查到 Gizmodo 一篇，措辞克制。

## 可能的吠点（原文里有、线索摘要没提或说反的条件；L1，除非标注）

1. 发布页的“无数学参考，只有中间定理陈述”对比 claude.dev 的“一句话证明路线 + 定义 + 47 条前次已证引理”，系统卡也写“building on definitions and smaller proven results from an earlier formalization”（B1e）。
2. 发布页用 “Fable 5.2”，系统卡图例写 “Claude Fable 5.2 (earlier version)”、正文写 “earlier snapshot”，claude.dev 写 “an early version”；发布页图例没带这个限定（B1f）。
3. 第 18 小时是人工从存档“restarted”续跑 12 小时（claude.dev 图 4 说明；系统卡“resumed each team’s work with an additional 12-hour budget”）；发布页“minimal human intervention”未提。第 18 小时时得分是 0.86，不是完成；最后 2 小时才到 1.00（B2a）。
4. 单次运行：每个模型各一次（发布页图注与系统卡均写 “a single run per model”）；≤100 agents 的实验是 3 次平均。1,000 agent 对 Opus 5.5 的对照是 n=1（B2）。
5. 与 9 月那次比的“token 少 2/3、证明短 7 倍”分母是 9 月那次（约 60 亿 output token、1,300 万行 Lean），而本次起点含 47 条前次已证引理，不是同起点（B1c）。推算数值为本组推算，不是官方数。
6. 运行是 “toy experiment”（claude.dev 自称），在无网络沙箱里，Lean 与 Mathlib 内置；没有公开本次 Lean 产物（B4b）。
7. 监控口径：本次只用基于动作的监控；CoT 监控在这次运行之后才加；“零次达到拦截标准”是对基于动作监控的结果，用 CoT 监控与现行评分细则重放，有 1 次“接近”（B3c、B3d）。
8. 47 条被判不可行、领导者搁置；最难的 Langlands–Tunnell 被队长估计“<1% 机会”；“人类鼓励可以让 agent 不过早放弃”是 claude.dev 自己提的（B3a、B3b）。
9. 黑洞两项：预印本全部发在不做同行评审的新站 Hexagon，署名 “Claude (Anthropic)”，无具名人类作者；来源声明写“internal Claude research model”，未写 Fable 5.2；174 写公开可用的 Opus 5.5 后来独立得到同一结果（B5）。
10. 175 的结果限定为弱耦合、1/N 领头阶、λ² 阶，“不证明一阶线延续到强耦合”；174 限定于一类线性泛函，c^{1/3} 项系数是猜想（B5e、B5f）。
11. 社区线索（L6，仅线索）：HN 50431756：“The 30 hrs vs. 11 days means nothing without at least some token-to-token comparison.”（注：发布页其实给了“one-third as many action tokens”，该评论没提）；HN 50431882：质疑 5.2 训练数据含 Fable 原次运行的轨迹，是否值得惊讶；HN 50432156：“Why re-formalise Fermat's Last Theorem instead of doing something new?”；HN 50433226：“We currently can only produce reliable code / formalizations that have reliable checkers.”；HN 50433862：对计算量的粗略推算（无出处，可信度低）。全部来自 https://news.ycombinator.com/item?id=50429068 讨论串（Algolia 条目 API 不提供评论分，**分数未取到**，链接形式 `https://news.ycombinator.com/item?id=<评论号>`）。

## 夸大说法实例

见 B7（5 例，含 1 个对照例）。英文侧没有找到把“formalization”写成“proved Fermat”的原站标题；中文例 1、2、3 为最明显。

## 扫描说法勘误

| 扫描/已知情况的说法 | 取证结果 |
|---|---|
| “30 对 28.7 小时数字打架” | 不成立：30 = 18 + 12 的预算，28.7 是完成时刻；不记矛盾（B1b）。附：第 18 小时是人工续跑，发布页图 28 h 点约 0.87、30 h 点 1.00 |
| 发布页“图上读数 Fable 5.2 1.00、Opus 5.5 0.26/0.27” | 一致，另有标注点 Fable 18 h = 0.86（B2a） |
| 发布页“used no mathematical references, only a list of intermediate theorem statements” | claude.dev 与系统卡补充了“47 条已证引理、定义、一句话路线”，三处并列（B1e） |
| “Fable 5.2 的团队” | 系统卡与 claude.dev 都说是 “earlier snapshot / early version”（B1f） |
| “两篇黑洞预印本” 在 arXiv | 实际在 Hexagon（hexagonmath.org，不做同行评审的站），编号 2611.00174/00175，日期 Nov 07, 2026（B5） |
| “预印本归给 Fable 5.2” | 预印本写 “internal Claude research model”；174 另写公开 Opus 5.5 后来独立得到（B5b） |
| “同一天的技术说明写 preprints, not peer-reviewed theorems” | 三份官方文本里未找到该句；找到的是 Hexagon 的站点政策与 Gizmodo 的一句话（B6） |
| Reuters/AOL 稿“尚未经独立同行评审” | 未找到该稿（B6b）；找到 Gizmodo 同义句 |
| claude.dev：“47/60 被判不可行”“最深 10 层 2418 个任务”“只用基于动作监控”“CoT 监控这次之后才加”“超过一百万次零次拦截” | 均核实原句（B3）。注意文中另有一个“47”：给 agent 的 47 条前次已证引理 |
| 扫描“0924 期已写过”是 11 天的内部模型 | 已核：9 月页 Published Sep 4, 2026，内部模型“roughly comparable to Claude Fable 5.1”，约 60 亿 output token、1,300 万行 Lean、29,500 个中间定理进入最终证明（30,300 个定理验证过）；9 月仓库 https://github.com/anthropics/fermats-last-theorem 存在 |
| 发布页写 “action tokens” | claude.dev 写 “output tokens”，两处未定义 “action tokens”；9 月页基数是 output tokens（B1c） |

## 未完成或查不到

- Reuters/AOL 稿；Anthropic 官方 X 帖（发布时刻）；两篇预印本的第三方复算；本次 1,000-agent 的 Lean 产物；HN 评论分数（Algolia 条目接口不含）。
- 系统卡中与黑洞相关的内容：grep “black hole”“holograph” 无命中；STEM Professor Bench 为内部测试集，未取证。
- 163、TVBS 两篇只核了标题与一两句，未取页面发布时间；`fourweekmba.com` 抓取被 403 拦截，未用其他手段取。
