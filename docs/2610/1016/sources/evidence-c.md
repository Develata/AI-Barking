# 1016 期 C 组取证：Anthropic 内部就业报告 + Mercor Benchmark Map 对照

执行：Sonnet 5.5（claude-sonnet-5-5）。时间一律北京时间（UTC+8），括号保留原时区。存档/截图时刻 2026-10-17 20:49–21:10。文件前缀 `c-`，在 `docs/2610/1016/sources/`；截图在 `docs/2610/1016/images/25–34`。HTML、PDF 原件只存本地，不入库（见 `capture-log-c.md`）。

级别说明：Anthropic 报告、Anthropic 摘要帖、Mercor 页面与 Mercor 官方 X 帖均为 L1；媒体为 L4/L5（只作“夸大说法实例”线索）。标“（读图）”的数字是我从图上读出的，不是原文文字，须原样标注，不进正文。

## 一、事实清单

### C1 初级占比与招聘占比

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1a 技术部门初级员工占比 21%→14% | L1 | https://www.anthropic.com/research/employment-at-anthropic | “Figure 1 shows that the share of junior-level employees in technical departments at Anthropic slipped from 21% to 14% since early 2025.” | `25-fig1-junior-share.png` | 起点原文只写 “since early 2025”，没给月份。图 1 标题 “Decline of entry-level technical hiring at Anthropic”，纵轴 “% junior in category”，三条线 Hires / Headcount / Open roles，左 Technical departments、右 Non-technical departments。图 1 没有附注（Note），“junior” 的定义（职级带）全文未给。（读图）Headcount 线从 3 月 ≈21% 起、终点标 14% | 已找到 |
| C1b 初级技术招聘占比 27%→9% | L1 | 同上 | “The proportion of junior technical hires fell even faster, from 27% to 9%.” | `25-fig1-junior-share.png` | 口径是“技术部门新招人员中初级占比”（图 1 Hires 线，纵轴 “% junior in category”），不是初级招聘人数。全文没有给招聘人数、样本量或分母（已通读正文与 PDF，无附录）。（读图）Hires 线起点在 5 月刻度处约 27–28%（2025-05，我按刻度推），终点标 9%，终点约在最后一个 “May” 刻度后一个月（≈2026-06）；折线逐月起伏明显（读图，±1–2 个百分点；中间多次回升到 ≈20% 上下）。因此 Hires 线起点（≈2025-05）与 Headcount 线起点（≈2025-03）并不同月；媒体写 “May 2025 to May 2026”（印度今日、NewsBytes）是读图推算，原文文字没写 | 已找到 |
| C1c 技术部门占比 61%→56% | L1 | 同上 | “the overall employment share of technical departments within Anthropic has fallen, from 61% to 56% in the last year.” | `26-tech-share-absolute-growth.png` | “in the last year”，口径是技术部门占全公司员工的比例 | 已找到 |
| C1d 技术招聘绝对数仍增长 | L1 | 同上 | “As the company expands, technical hiring has continued growing in absolute terms, but intensive hiring in sales, finance, and recruiting has outpaced it.” | `26-tech-share-absolute-growth.png` | 原文没给绝对数。同篇另有 “It has tripled in size in the last year.”（公司一年内规模翻三倍，未给起止月份与口径） | 已找到 |
| C1e 招聘帖降幅 | L1 | 同上 | “Postings for juniors have fallen particularly sharply since December 2025.” | `25-fig1-junior-share.png` | （读图）技术部门 Open roles 线 2025-11 前后高点 ≈21%，终点 6%；非技术部门 Open roles 线 3 月 ≈31.5% 终点 13%，同图里非技术部门 Hires 26%、Headcount 25% 基本不变 | 已找到 |
| C1f 原因表述与限定 | L1 | 同上 | “Part of this shift is the natural expansion of business functions as the company scales. Part could reflect coding tools increasingly allowing non-technical employees to make software themselves.” / “Descriptively, we see junior employment slowing in jobs that use Claude for the most complex tasks (Figure 5).” | `26-tech-share-absolute-growth.png`；图 5 未截 | 原文用 “Descriptively”，没有声称因果。结尾自承：“None of these actions will prevent powerful AI from disrupting jobs. And indeed none directly address the slowdown in junior hires documented in this report.” | 已找到 |
| C1g Anthropic 内部管理者说法 | L1 | 同上 | “Around a third of ML and a fifth of SWE managers reported hiring less due to AI.”（2026-04 调查）；“Within Anthropic, discussion of a decline in demand for junior technical employees emerged in core technical functions in January 2026.” | 无 | 自陈调查，样本量未给 | 已找到 |

### C2 “6 比 1”的定义

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C2a 6:1 的数字与范围 | L1 | 同上 | 要点：“By August interactive AI agent hours outnumbered employees’ 6 to 1.” 正文：“But even narrowing in on the agents deployed by individual employees, as in Figure 2, by August the agents worked 6 times as many hours as their human handlers.” | `27-fig2-agent-hours.png` | 仅员工自己发起的交互式会话（employee-directed interactive sessions），子智能体合并计入。图 2 标题 “Agent hours overtake human hours”，纵轴 “Agent hours per human work-hour (log scale)”，对数坐标，虚线 “Parity”；图例两条：“Research organizations 19× human hours”（蓝）、“All employees 6× human hours”（粉）；横轴 2026-02 至 8 月下旬 | 已找到 |
| C2b 图 2 注释全文 | L1 | 同上 | “Note: We limit to employee-directed interactive sessions and pool subagents. Working time concept is agent runtime, including inference and active tool use, but excluding any wait longer than 30 minutes. Human work hours are calculated as headcount and assumed 40 hours per week.” | `27-fig2-agent-hours.png` | 分子是智能体运行时长（含推理与主动工具调用，排除连续等待 >30 分钟），分母是人头数×40 小时/周的名义工时，不是实际工时。“6×” 因此不是 “6 倍于员工人数的智能体”，也不是 “6 个智能体顶 1 个人” | 已找到 |
| C2c 三月“超过人工工时”（不要与 6:1 混用） | L1 | 同上 | “Based on agent session telemetry, in March 2026, the sum of autonomous and interactive agent work hours leapt past human work hours for the first time.” | `27-fig2-agent-hours.png` | 三月这句的分子是自主（autonomous）+ 交互式智能体工时之和，口径比图 2 宽；“Part of this is fleets of agents conducting industrial scale software engineering…”。（读图）图 2 里仅交互式的 “All employees” 曲线越过 Parity 线约在 5 月上中旬，“Research organizations” 曲线约在 4 月初——都晚于 3 月，所以 “三月” 与图 2 不是同一条曲线。“6 比 1” 的 “All employees” 只统计员工发起的交互式 | 已找到 |
| C2d Anthropic 摘要帖的宽松写法 | L1 | https://www.anthropic.com/research/expectations-for-the-future（2026-10-16T16:21Z=10-17 00:21） | “By March 2026, agents worked more hours than employees. By August, there were six times more agents than employees at Anthropic, and productivity increased.” | `35-anthropic-summary-six-times-agents.png`（标题、日期与上下文，已略去页首装饰图） | 摘要帖写成 “six times more agents than employees”（智能体“数量”是员工 6 倍），报告正文写的是工时比、且限定交互式。同一机构两处表述不一致，以正文为准。报告正文 Introduction 另有一句同样不带限定的：“Agent hours worked outnumber human hours 6 to 1.”（第三处，对比要点与图 2 注释里的“employee-directed interactive”限定；该句在 `c-anth-report.txt` 的 Introduction 段）。摘要帖紧接着还写 “That’s led to Anthropic producing more code per person, but that’s a notoriously imperfect measure.” | 已找到 |
| C2e 各发布时刻 | L1 | 报告页 JSON-LD | datePublished “2026-10-16T16:26:00.000Z”，dateModified “2026-10-16T21:14:21.000Z” | 无 | 发布 = 北京 2026-10-17 00:26；页面在 10-17 05:14（北京）有过修改，我的存档在 10-17 20:49，是修改后的版本；改了什么不详 | 已找到 |

### C3 产出与质量

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C3a 5 倍提交 | L1 | 同上 | “code commits per engineer have risen the fastest, coming in at 5x their velocity as 6 months ago.” | `28-fig3-productivity.png` | 图 3 标题 “Productivity boom across functions”，纵轴 “Output per person, index (Dec 2025 = 100)”，注：“Loess smoother fit across underlying indexed output data.” 图例：Commits per engineer 5.1×、Queries per data scientist 3.1×、Contracts per legal employee 3.1×、Deals per salesperson 1.7×、Hires per recruiter 1.4×、Invoices per AP processor 1.4×。基准是 2025-12=100（读图：终点约 2026-06）。摘要帖自己说 commits “a notoriously imperfect measure”。这是整体工程师人均提交量，不是采用者对比 | 已找到 |
| C3b 192 对 74 | L1 | 同上 | “We follow 192 engineers over time as they adopt coding agents, compared to 74 engineers who do not adopt agents during the period studied.” | `29-adoption-not-random.png` | 前一段：“Note however that tool adoption is not random: if employees on the cusp of driving some big new project decide to tool up for it, for example, this would bias our estimates.” 图 4 注：“Treatment timing is based on the person’s first month with at least 5 agentic sessions”；估计量 “person and calendar-month two-way fixed effects”；误差条 “95% confidence intervals from standard errors clustered on person” | 已找到 |
| C3c 质量：无差异 | L1 | 同上 | “We compare code committed by the same employee, in the same month, in the same coding directory, but written by coding agents rather than written directly by the human employee. … But we find no difference in rates of lines of code subsequently removed or reverted.” | `29-adoption-not-random.png`、`30-fig4-adoption-effects.png` | 图 4 注给了质量代理指标：“the share of lines deleted for any reason within 90 days; the share deleted by a bug-fix specifically; whether the whole commit is explicitly reverted; adding a lint-suppression comment; adding a need fixing note; and marking a test to be skipped, retried, or allowed to fail.” 原文同段自承 “There are many dimensions of code quality, and AI errors may be harder to identify than human ones.” （读图）右上质量面板几项的 95% 置信区间较宽（如 “Lines removed, 90d” 约 −1.4 至 +1.3 个百分点，“New lint suppression” 约 −2 至 +2），“无差异” 即区间含 0，不是“证明相同” | 已找到 |
| C3d 采用者原本就在上升 | L1 | 同上 | “Those who adopt were already on a steeply upward trajectory compared to non-adopters. This suggests that employees shifting toward fully autonomous agents were already ramping up their output.” | `30-fig4-adoption-effects.png` | 针对“首次采用完全自主智能体”的那组；图 4 左下 “Fully automated routine coding” 采用前的点 −4 至 −2 月就在上升（读图） | 已找到 |
| C3e 80% 空等 | L1 | 同上 | “We track idle time by subtracting each agent’s active working time from each interactive session’s total wall-clock time. Claude sessions amount to around 80% idly waiting for human input.” | `31-idle-waiting.png` | 仅交互式、多轮会话；“around 80%”。与 6:1 的分子（排除 >30 分钟等待）口径不同 | 已找到 |
| C3f 会议时间 | L1 | 同上 | “Based on calendar data, the median employee spends 20% more time in meetings than a year ago. … But even among employees in technical individual contributor roles, meeting time is up 16% over the past year.” | 无 | 中位数员工 +20%；技术个人贡献者 +16%；原文称部分来自公司增长与向会议密集的非技术岗位倾斜 | 已找到 |
| C3g 人均收入 2–4 倍 | L1 | 同上 | “At five years old, Anthropic takes in between 2–4 times as much revenue per employee as Meta and Google as they crested in their own rapid growth.” | 无 | 脚注：“Anthropic estimates come from internal financial and headcount records for the twelve months to April 2026 (approximately five years after founding). Google and Meta employment and revenue figures based on Form 10-K filings at 5 years after founding (2003 and 2009, respectively). All figures are deflated to 2025 dollars.” | 已找到 |
| C3h 非技术员工编码会话 | L1 | 同上 | “Coding-focused AI agent sessions for non-technical employees (identified as those with no code commits at baseline, excluding new hires) leapt from 2 per month in August 2025 to 90 in May 2026.” | `26-tech-share-absolute-growth.png`（句子在图内最后两行；“May 2026.” 在末行） | “2 per month → 90”，未给分母（是每人还是整体）。原文句式像整体数，未给明（口径注释，不影响“原句已找到”） | 已找到 |
| C3i 其他 | L1 | 同上 | “employees at Anthropic expected that AI will be able to do 75% of their work by 2030, compared to 28% for other Americans.”；“The share of tasks above the 90th percentile of difficulty … rises sharply by around 30% (from 10% to 13% of all tasks)” | 无 | 不进本组正文候选，仅备查。注意：摘要帖/E2 另有 “More than half of Anthropic staff expect AI to be able to do at least 80% of their current work by 2030; only 5% of Americans” 口径不同 | 已找到 |

### C4 作者、数据与局限

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C4a 作者与机构 | L1 | 同上 | “Nathan Wilmers, Regina Catipon, Maxim Massenkoff, Peter McCrory”；引用格式 “institution = {The Anthropic Institute}, type = {Technical Report}, year = {2026}, month = oct” | 无 | 全部署名作者；分类 Economics。是 Anthropic 自己研究自己（利益相关：公司卖智能体，原文也写 “It is easier to close deals when you sell a strong model.”）。未见独立第三方审计或同行评议声明（已通读） | 已找到 |
| C4b 数据来源 | L1 | 同上 | “This report draws on comprehensive internal Anthropic data covering employment, AI use, output metrics, employee surveys, internal communication patterns, employee calendar data, and hiring decision records. It contextualizes these quantitative data with 17 employee interviews.” | 无 | 内部数据不可公开复核 | 已找到 |
| C4c 自述限制 | L1 | 同上 | “Here we want to proceed carefully. There are many parts of Anthropic’s employment practices that cannot be replicated across the economy.”；“More code does not necessarily mean better code or business impact.”；“Part of this pattern could be driven by representatives with more sales opportunities selecting into automation.”；“Reading these patterns forward and out across the economy suggests that AI diffusion could risk worker displacement.” | `29-adoption-not-random.png` | 原文没有“裁员”表述；全文无裁员数字 | 已找到 |
| C4d PDF 版 | L1 | https://www-cdn.anthropic.com/72628f1863c53432195db83279252dfd2cb53158/Employment%20at%20Anthropic.pdf | 同网页 | 无 | 本地 PDF（2.0 MB，不入库），转文本 `c-anth-pdf.txt`；无附录、无方法附录，上述定义全部来自图注与正文 | 已找到 |

### C5 Mercor Benchmark Map

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C5a 208/923、924/18,797、29 | L1 | https://www.mercor.com/research/labor-index/benchmark-map/ | 顶部三个数字标签：“of 923 occupations covered” 208；“of 18,797 tasks covered” 924；“benchmarks in the index” 29。页内：“In our data set: only 208 of 923 occupations have a task with at least 5 benchmark prompts, and 6 of 22 job families have no such occupations.” | `32-mercor-top-stats.png`、`33-mercor-90pct-working-hours.png` | “covered” 的定义在方法节：“an O*NET task counts as ‘covered’ if our LLM judge has mapped it to at least 5 prompts from our 29 benchmarks.” 页面交互开关默认是 “5 prompts”。924/18,797≈4.9%。第三方 runtimewire 转写为 “covers at least one task in 208 of 923 occupations”（与页面的 “at least 5 prompts” 不符）；Mercor 官方 X 帖写 “208 of 923 occupations … are represented by existing AI benchmarks”，也没带 “至少 5 题” | 已找到 |
| C5b 90% | L1 | 同上 | “Our findings suggest that 90% of American working hours are spent on tasks with little or no coverage from existing benchmarks.” | `33-mercor-90pct-working-hours.png` | “suggest”。“little or no coverage” 的阈值页面未给（方法节只定义了 “covered” = ≥5 题）。工时份额来自 LLM 推算：“we combine our coverage estimates with Hatgis-Kessell et al.’s LLM-derived estimates of the working time share of each O*NET task.” X 帖写成 “90% of hours worked in the US aren’t covered well by a relevant AI benchmark.” | 已找到 |
| C5c 标注方法 | L1 | 同上 | “we then ask an LLM judge to tag each benchmark prompt with as many O*NET tasks as it thinks correspond to that prompt. We validate our LLM tagging with human expert tags on Mercor’s APEX benchmarks, a subset of this list of 29.” | 无 | 方法页：用 “GPT 5.6 Luna” 标注；专家复核只覆盖 APEX（1,013/1,022 条），专家接受首选 76.7%（1,352/1,763），首选或列表内任一命中 85.4%；“APEX-SWE was tagged after the review and has no expert judgements.” 其余 25 个基准的标注没有专家复核（页面原话只说 APEX 是子集）。每条提示平均映射 3.1 个 O*NET 任务（“The map counts all of them”） | 已找到 |
| C5d 限制节 | L1 | 同上 | “Any incomplete list will necessarily bias down estimates of economic coverage, and it is difficult to bound that bias. We want to stress, therefore, that these results are provisional.” | `34-mercor-limitations.png` | 另有两条自陈偏差：每个职业族封顶基准数会 “biasing down our estimates of coverage in the most-benchmarked job families (e.g. software engineering)”；只收未饱和基准会 “understating benchmark coverage on O*NET tasks which are easier for LLMs to complete”；“several criteria (like ‘Economic Realism’) are difficult to formalize and introduce some subjectivity in our choices.” | 已找到 |
| C5e Mercor 自家基准在 29 个中的数量 | L1 | https://www.mercor.com/research/labor-index/benchmark-map/benchmarks/ | 表中 Institution=Mercor：APEX-Accounting、APEX-Agents、APEX-SWE、APEX-1 | 无 | 4/29（我数表格行得到）。页面入选标准：“We also include Mercor’s APEX benchmarks, where we have access to held-out benchmark prompts” | 已找到 |
| C5f 发布日期 | L1/L5 | https://x.com/mercor/status/2110557079027265653（fxtwitter 存档 `c-mercor-tweet.json`） | “Today, we’re releasing the Benchmark Map of the US Economy, an interactive breakdown of which economic tasks are covered by AI benchmarks.” | 无 | 帖子 created_at “Thu Oct 15 02:23:14 +0000 2026” = 北京 2026-10-15 10:23:14；雪花 ID 解码一致。页面本身无 datePublished/发布日期元数据（已查 meta、JSON-LD）。runtimewire 称 “Published Oct 15th, 2026, 12:44am CT”（=10-15 13:44 北京，第三方）。因此 Mercor 比 Anthropic 报告早约 1.5 天，两者是各自独立的事件，HN 上被一起讨论是 10-17 | 已找到 |
| C5g 自我利益 | L1 | https://www.mercor.com/research/labor-index/benchmark-map/ | 页脚/导航：“APEX Benchmarks”“Human data”“Contact sales” | 无 | Mercor 卖人类专家数据与评测（站点导航），APEX 基准是自家产品；页面没有利益声明 | 已找到 |

### C6 热度

| 项 | 数值 | 时刻 | 来源 |
|---|---|---|---|
| HN “How employment is changing at Anthropic”（50127555） | 8 分 / 3 评 | 提交 2026-10-17 11:13（北京；03:13Z）；采样 21:05（北京） | `c-heat.json`；https://news.ycombinator.com/item?id=50127555 |
| HN “90% of US working hours are spent on tasks not covered well by AI benchmarks”（50121179） | 536 分 / 454 评 | 提交 2026-10-17 01:08（北京；10-16 17:08Z）；采样 21:05（北京） | `c-heat.json`；https://news.ycombinator.com/item?id=50121179 |
| Mercor 官方 X 帖 | 315 赞 / 24 转 / 74,505 浏览 | 采样 2026-10-17 约 21:10 | `c-mercor-tweet.json` |
| Reddit | 未取得 | — | `old.reddit.com` 搜索被登录墙拦下（记入 `c-heat.json` errors） |

说明：派工单的“已知情况”写 Mercor HN “526/446”，我 21:05 采样是 536/454（随时间上升，不冲突）。Anthropic 报告的 HN 帖热度很低（8/3），不是“热度”候选的依据；AIHOT 约 77 的说法我没有核（未取得 AIHOT 页面，见下）。

## 二、可能的吠点（原文里有、线索摘要没提或说反的条件）

1. 27%→9% 是“技术部门新招人员中初级占比”，不是初级招聘人数、也不是裁员。原文没给任何招聘人数或样本量；公司同期 “tripled in size”，绝对数是增还是减，原文没说（C1b、C1d）。
2. 21%→14% 与 27%→9% 的起点不是同一时点：原文只写 “since early 2025”；（读图）Headcount 线起于 2025-03，Hires 线起于 2025-05。两个数字是不同序列的端点，不是同一人群前后对比（C1a/C1b）。
3. Hires 线逐月起伏明显（读图），9% 是末月点值；图 1 无置信区间、无样本量、无附注，“junior” 的职级定义全文未给。
4. 6:1 只算员工自己发起的交互式会话，分母是“人头×40 小时/周”的名义工时，分子是运行时长（排除连续等待 >30 分钟）；同图“Research organizations”是 19×（C2a、C2b）。不能写成“AI 员工是人类的 6 倍”或“6 个 AI 顶 1 个人”。
5. 三月“超过人工工时”的口径更宽（自主+交互式合计）；图 2 的仅交互式曲线越过 Parity 线要晚到 4 月初（研究部门）至 5 月上中旬（全体员工）（读图），不要把三月与 6:1 连着写（C2c）。
6. Anthropic 摘要帖自己写 “six times more agents than employees”，比报告正文宽松（C2d）。
7. 同一份报告里，交互式会话约 80% 时间在“等人输入”（C3e），与“智能体干了 6 倍工时”并列看：工时按运行时长算，不含长等待。
8. 5 倍提交量是 2025-12=100 的指数、loess 平滑后的工程师人均值；摘要帖自称提交量 “a notoriously imperfect measure”；采用者对比的“无差异”靠置信区间含 0，而区间不窄（读图）；原文自承 “tool adoption is not random”，会使估计有偏（C3a–C3d）。
9. 采用完全自主智能体的人 “were already on a steeply upward trajectory”，采用前就在涨（C3d）。
10. 报告自承 “Descriptively”，不是因果；结尾 “none directly address the slowdown in junior hires”；全文无裁员数字、无“取代初级工程师”的论断；同时写有 “AI diffusion could risk worker displacement” 的外推（C1f、C4c）。
11. 自家数据、自家研究者、无外部审计；17 次访谈；公司卖智能体（C4a、C4b）。
12. Mercor：208 与 924 都按“至少 5 道题”算，页面开关默认 “5 prompts”；官方 X 帖与 runtimewire 转写把阈值丢了或改成 “at least one task”（C5a）。
13. Mercor 的 90% 是 “suggest”，“little or no coverage” 阈值页面未给；工时份额来自另一篇 LLM 估计（C5b）；用 LLM 打标签，只有 APEX 4 个自家基准有专家复核，专家只接受首选 76.7%（C5c）；作者自称结果 “provisional”，不完整名单会使覆盖被低估（C5d）；Mercor 自己的 4 个基准在 29 个之内，且 Mercor 卖评测数据（C5e、C5g）。
14. 两篇不是同一事件：Mercor 说的是“基准没覆盖多少职业”，Anthropic 说的是“公司内部招聘结构”。拿 Mercor 的 90% 去给 Anthropic 报告作“对照”，只能是话题对照（基准对真实工作的覆盖有限 vs 公司自己看到的工作变化），不能互相证明。

## 三、社区评论线索（L6，仅线索；HN 评论不公开分数，下列按页面默认排序位置注明；未回到一手来源核实）

- Mercor 帖 https://news.ycombinator.com/item?id=50121179 ：
  - 顶层评论（页面默认顺序第 24 位）abroszka33（https://news.ycombinator.com/item?id=50124003）：“Because it's a bullshit slop research. Under computer programmers there is "Develop Websites" at 0. Apparently AI cannot write web sites. Or just nobody looked at the slop they generated at Mercor.” —— 具体指控“Develop Websites”任务覆盖为 0；我没有在页面静态存档里核到该职业行（该交互表需点开），未验证。
  - hamdingers（https://news.ycombinator.com/item?id=50122035）：“Nobody has yet made a benchmark for how good LLMs are at taking drive-thru orders, even though there are LLMs doing that.” —— 指出基准覆盖不等于实际应用，线索。
  - blmarket（https://news.ycombinator.com/item?id=50122000）：“Just counting by hours is not reflecting the impact on the economy. Treemap weighted by GDP will make more sense.” —— 口径质疑（按工时 vs 按价值）。
  - 10111001（https://news.ycombinator.com/item?id=50121809，31 条子评论）：主张基准偏向高薪职业——这点 Mercor 页面自己也写了（“O*NET tasks from occupations with larger wage bills are significantly overrepresented”）。
  - 评论 JSON 存档：`c-hn-mercor-comments.json`。
- Anthropic 报告帖 https://news.ycombinator.com/item?id=50127555 （8 分，仅 2 条顶层评论）：gr_norm 称这与 Reuters 报道的收入增速放缓形成反差（链接被截断，未核）；andsoitis 问 “And on a profit per employee basis?” —— 报告只比较了人均收入，没有人均利润。存档 `c-hn-anth-comments.json`。

## 四、夸大说法实例（把事件写歪的标题/文章）

存档均在 `sources/c-news-*`。时间按北京时间，括号为原时区。

1. 凤凰网/新浪财经/IT之家（同一稿，IT之家原发）：“Anthropic 报告：AI 工时已是人类 6 倍，初级程序员岗位骤减”
   - URL：https://www.ithome.com/1/014/195.htm （IT之家，页面时间 2026/10/17 7:13:28 北京）；转载：https://finance.sina.com.cn/stock/t/2026-10-17/doc-inivnrax8238598.shtml （新浪财经，2026-10-16T23:34Z=10-17 07:34）、https://tech.ifeng.com/c/8xHwSo3IDfh （凤凰网，2026-10-16T23:18Z=10-17 07:18）。
   - 原句：“初级技术岗位招聘占比更从 27% 骤降至 9%”“报告称，到 2026 年 8 月，员工部署的交互式 AI 智能体工作时长已达人类员工的 6 倍”。
   - 偏差：标题把“交互式会话的智能体运行时长/名义工时”写成“AI 工时已是人类 6 倍”，没保留“员工发起的交互式”；“初级程序员岗位骤减”把“招聘占比下降”说成“岗位骤减”（报告称技术招聘绝对数仍在增长）。正文第一段里“交互式”与注释是保留的，偏差在标题。
2. 印度今日：“Anthropic is hiring fewer juniors, says AI now does 6 times more work than humans”
   - URL：https://www.indiatoday.in/jobs/story/anthropic-is-hiring-fewer-juniors-says-ai-now-does-6-times-more-work-than-humans-tchc-3019422-2026-10-17 ，2026-10-17 10:54 IST（=13:24 北京；05:24Z）。
   - 原句：“hiring of junior level staff fell from 27 per cent to 9 per cent in a year”；“Now, the study added, AI agents outnumbered human working hours by 6 to 1.”；“between May 2025 and May 2026”。
   - 偏差：标题把工时比写成 “6 times more work”（工作量）；正文称 “hiring of junior level staff fell”（招聘占比下降≠招聘减少）；“May 2025 and May 2026” 在原文文字里没有，是读图（我读得终点约 2026-06）；“AI agents outnumbered human working hours” 语句不通且丢掉限定。
3. NewsBytes：“Anthropic says AI agents now work 6 times human hours” / “Junior tech roles fell from 27% to just 9% between May 2025 and May 2026.”
   - URL：https://www.newsbytesapp.com/news/business/anthropic-says-ai-agents-now-work-6-times-human-hours/tldr ，发布 2026-10-17T11:08Z（=19:08 北京），来自检索摘录；页面 403，未存档。
   - 偏差：“Junior tech roles fell” 把占比说成岗位；“which has led to a big drop in junior hires”（摘录里的因果连接词）报告里没有因果断言。
4. AI Weekly：“Anthropic: Junior Technical Hire Share Fell From 27% to 9%”
   - URL：https://aiweekly.co/alerts/anthropic-junior-technical-hire-share-fell-from-27-to-9 ，2026-10-17T02:03:32Z（=10:03 北京）。
   - 原句：“Among 192 engineers who adopted coding agents against 74 non-adopters, commits per adopter rose 5x their velocity as 6 months ago, with no difference in rates of lines of code subsequently removed or reverted.”；“the steepest drop coinciding directly with agent deployment acceleration”；“agent-hour substitution”。
   - 偏差：把“全体工程师人均提交 5 倍”和“192 vs 74 采用者对比”拼成一句；“substitution”（替代）是作者自己的因果解释，报告通篇是 “Descriptively”。
5. Anthropic 摘要帖自身（L1，宽松措辞）：“By August, there were six times more agents than employees at Anthropic, and productivity increased.” https://www.anthropic.com/research/expectations-for-the-future ，2026-10-16T16:21Z（=10-17 00:21 北京）。
6. 对照：explainx.ai 与 PRIAG 把口径写对了（explainx：“So ‘six times more agents than employees’ is a modeled ratio of machine runtime to a nominal workweek.”；PRIAG：“A falling share alone does not establish fewer junior jobs.”）——可作为“正确读法”的二手旁证，不作事实依据。
7. “Anthropic 裁员”类标题：我在中英文检索（WebSearch、Exa）中没有找到把本报告写成“Anthropic 裁员”的标题；印度今日正文另引与本事件无关的行业裁员数据（layoffs.fyi 323 家公司）作背景，属并置，不是对 Anthropic 的指控。“6 倍生产力”类标题：没有找到，（“工时 6 倍”是上面 1–3）。
8. Mercor 部分：runtimewire 标题 “Mercor maps AI benchmarks to jobs and finds most US work unmeasured”（https://runtimewire.com/article/mercor-benchmark-map-us-economy ，2026-10-15 12:44 CT=13:44 北京，更新 09:04 CT）正文丢了“至少 5 题”的阈值：“covers at least one task in 208 of 923 occupations”；HN 标题 “90% of US working hours are spent on tasks not covered well by AI benchmarks” 把 “suggest/little or no coverage” 写成“not covered well”。X 帖原句同样如此（C5f）。

## 五、扫描说法勘误

- 派工单：“datePublished 2026-10-16T16:26Z = 北京时间 10/17 00:26”——核实无误；另发现页面 dateModified 为 21:14Z（北京 10-17 05:14），发布后改动过一次。
- 派工单：“By August interactive AI agent hours outnumbered employees’ 6 to 1”——原文要点写的是 “outnumbered employees’ 6 to 1”，正文写的是 “agents worked 6 times as many hours as their human handlers”，图 2 注释才是口径定义（C2）。
- 派工单对 5x 与 192/74 是分开列的，无误；提醒勿合并：5x 是全体工程师人均提交指数（图 3），192/74 是采用者对比（图 4），原文没有“采用者提交量 5 倍”的说法；AI Weekly 已把两者并成一句（见夸大实例 4）。
- 派工单：“技术部门占比 61%→56%”核实无误，原文 “in the last year”。
- 派工单：“人均收入为 Meta、Google 同龄期的 2–4 倍”核实无误；注释的“记录至 2026-04”核实：“twelve months to April 2026”，Google/Meta 用 10-K、“All figures are deflated to 2025 dollars”。
- 派工单：Mercor “发布日期未见，须核”——页面无日期元数据，官方 X 帖北京 2026-10-15 10:23:14；不是 10-16/17。
- 派工单：Mercor “29 个基准、923 职业中 208 个、18,797 任务中 924 个”核实无误，但 208 与 924 都是“至少 5 题”口径；“≥5 题才算覆盖、工时份额来自 LLM 估计”核实无误；“限制节称样本会低估覆盖”核实无误（“bias down”）。
- 派工单：Mercor “自家 APEX 基准在 29 个之内”核实：4 个（APEX-Accounting、APEX-Agents、APEX-SWE、APEX-1）。
- 派工单：HN 热度 “526/446”→我 21:05 采样 536/454；Anthropic 报告 HN 帖 8/3，不是热点。AIHOT 约 77 的说法我没有核。
- 派工单未提、我新发现：Anthropic 摘要帖与报告正文对 6:1 的措辞不一致（C2d）；图 2 的 19×（研究部门）；图 1 是 Hires/Headcount/Open roles 三条线，且非技术部门同图并列。

## 六、未完成/未验证

- AIHOT 页面未取（无法确认“约 77”）。
- Reddit 被登录墙拦下，无数据。
- Firecrawl 额度耗尽（402），未使用；检索改用 WebSearch 与 Exa。
- NewsBytes 403，只有检索摘录。
- 图 5–8、Anthropic 摘要帖截图、Mercor 基准表截图未做（不在候选范围内）。
- 图 1 起止月份、图 2 越过 Parity 的月份、图 4 置信区间数值均为读图，不进正文；若要写，须问原作者或标“读图”。
- Hatgis-Kessell et al.（arXiv 2608.05172）、Wang et al.（2603.01203）我没有打开核对（页面引用的外部论文）。
