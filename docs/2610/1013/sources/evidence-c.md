# 1013 取证 C 组：Gemini 4 Argon 正式开放【跟进】与谷歌 ADK 通宵优化实验

执行方：Claude Sonnet 5.5（claude-sonnet-5-5）。派工单：`.handoff/2026-10-14-1013-evidence.md`。时间一律北京时间（UTC+8），括号内为原时区。本机时区美东（EDT，UTC-4），北京 = 本机 + 12 小时。取数时刻见各条与 `capture-log-c.md`。

约定：L1 官方，L2 员工公开发言，L3 第三方评测原始页，L4 权威媒体，L5 聚合与二手，L6 社交转述。“存档”指 `docs/2610/1013/sources/` 下文件，`../images/` 为截图。HN 评论不公开分数，下文 HN 评论只能给评论 ID、所在子树回复数（作热度代理）。

## 一、先行结论（与“已知情况”不符或需要改口径之处）

1. 公告页 `<title>` 与 HN 标题写 “Gemini 4 Argon is now available for everyone”，页面 H1 与 og:title 写 “Gemini 4 Argon: Built to solve your hardest problems”。“everyone”不等于免费：正文列出的可用范围全部要付费订阅或付费 API 层；API 价格页写免费层 “Not available”，并注明不在 Google AI Pro 套餐上（C1、C2）。
2. 公告把 “1M unified input-output token limit” 说成“业界领先”。API 文档：1,048,576 是输入、思考、输出共用的上下文窗口；单次同步调用最大输出 65,536；超过 64K 要开后台执行（background）并由服务器分 64K 一跳续写，`background: false` 时超 65,536 返回 400（“Long decode coming soon”）。9/30 首发稿写的是 “output token limit to an industry-leading 1M tokens, up from the previous 64K tokens”，口径已从“输出上限 1M”变成“统一窗口 1M + 后台续写”（C2）。
3. ADK 博文“shipped to production”的实际含义：commit 1754b3e 于 10/7（太平洋时间 10:43，UTC 17:43）进入 google/adk-python 主干，2.12.0 版本 10/13 16:35 UTC（北京 10/14 00:35）才发布、PyPI 16:42 UTC 上传，晚于博文 16:00 UTC 发布约 35–43 分钟（C3、C4）。合入的是“第一轮与第二轮合并后的 22 个文件”，不是博文所说 PR 里的“32 个文件、6 项优化”；那个 PR 在 GitHub 上没有对应的公开 PR（commit 经 Copybara 导出，API 查到关联 PR 为空）。
4. ADK 数字的隐含条件：“约 40%”只对应 `InMemorySessionService`、Python 垃圾回收关闭、模型与工具即时返回的合成基准（图 1、图 5 与 commit 信息都写明）；“新进程到首次模型请求”3.07 s → 3.09 s（+1%）；整个 158 轮 65.6 s → 56.1 s 是 −14%（垃圾回收开启）。发版说明与 commit 标题却写 “Cut time to first model request by 40% in long sessions”（C3、C4）。
5. 扫描称“AA 正式版 53 分、每任务输出约 62,000 token、是 GPT-6-Astra 的两倍多”：53 分与 62k vs 27k 属实，但都来自 AA 9/30 文章与模型页（1001 期已存档）；AA 在 10/14 17:12 的模型页数值未变（53、1.1 亿输出 token、$1.99/任务），文章列表无新的 Argon 文章，**未见正式版新测**（C5）。
6. 扫描称 “Text Arena 第一为 Argon High 1525±9”：对。Arena 官方数据集（HF 上 lmarena-ai/leaderboard-dataset，latest，发布日期 2026-10-13）默认视图（style control）Argon-high 1524.8（置信区间 1515.9–1533.6），第 1；第 2 名 muse-wonder-max 1512.7（1499.4–1525.9），区间重叠；4,903 票。无 style control 版 1533.2（1524.4–1542.0）。WebDev（Code Arena）只排第 9（1678.0，1772 票），不是“全面登顶”（C5）。
7. 扫描称“自家员工向彭博爆料只会刷榜、前端不行”：彭博 9/30 稿（Julia Love、Davey Alba）原句是匿名人士称“struggles to handle certain coding tasks”“Gemini isn’t particularly adept at front-end design, which shapes how apps and websites look and feel, one person said”，“benchmaxxing”一句是“Gemini 4 appears to be affected by this process, said two people familiar with the model”（不是员工，且“appears”）；谷歌称“inaccurate”，并有员工称 “large consensus”在前沿。没有“只会刷榜”这句话（C6）。

## 二、逐条清单

### C1 正式开放公告

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 公告全文存档与发布时刻 | L1 | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/ | 页面日期 “Oct 13, 2026”；JSON-LD `datePublished` 2026-10-13T16:00:00+00:00，`dateModified` 2026-10-13T16:33:42+00:00 | `../images/31-argon-announce-title.png` | 北京 10/14 00:00 发布、00:33:42 更新（页面只标日期，时刻取自页面元数据）；作者 Koray Kavukcuoglu；存档 `c-announce.html/.txt` | 已找到 |
| 开放范围：API、Ultra 的 Antigravity、Pro/Ultra 的 Gemini 应用与 AI Mode | L1 | 同上（正文首段） | “Gemini 4 Argon is now globally available today for developers and enterprises via the API in AI Studio and Gemini platform, for Google AI Ultra subscribers in Google Antigravity, and Google AI Ultra and Pro subscribers in the Gemini app and AI Mode in Search.” | `../images/26-argon-price-footnote.png`（上半） | 与“已知情况”一致 | 已找到 |
| “Get started today” 分项 | L1 | 同上（末节） | “Argon is rolling out today globally:” “For developers: Build with the Gemini API paid tier in AI Studio and Android Studio, and if you’re a Google AI Ultra subscriber, you can try it in Google Antigravity and AI Studio.” “For enterprises: Try it in Gemini for business, the API, and in Google Antigravity.” “For everyday users: Try it starting today in the Gemini app and AI Mode in Search if you’re a Google AI Ultra or Pro subscriber.” “For scientists & researchers: Apply for a chance to get access to Google Antigravity’s highest usage tier free of charge for 6 months (onboarded on a rolling basis), subject to eligibility, capacity, and selection.” “For cybersecurity defenders: Apply to join the Fairwind Program to get access to Argon without cyber guardrails on a rolling basis.” | `../images/25-argon-availability.png` | 10,000 名科学家为 “10,000 scientists the highest usage tier in Google Antigravity free of charge for 6 months (onboarded on a rolling basis), subject to eligibility, capacity, and selection”；Fairwind “beginning expansion to hundreds of additional Fairwind Program partners” | 已找到 |
| 免费用户与免费 API 层能否用 | L1 | 公告正文；API 价格页 https://ai.google.dev/gemini-api/docs/pricing | 公告：**未写**免费用户或免费 API 层。价格页：“Note: Gemini 4 Argon is not available on the Free tier or on the Google AI Pro plan.” 表中 Free Tier 各行 “Not available” | `../images/36-api-pricing-argon.png` | 价格页取数北京 10/14 17:03；“Google AI Pro plan” 指 Gemini API 文档里链接的 google-ai-plans（API 侧），与公告“Pro 订阅者可在 Gemini 应用使用”并不矛盾，但两处并读容易混淆 | 已找到（公告未写；价格页明写免费层不可用） |
| 免费用户在 Gemini 应用的情况（媒体转述） | L4/L5 | Ars Technica https://arstechnica.com/ai/2026/10/gemini-4-argon-begins-rolling-out-in-gemini-app-and-antigravity-but-only-if-you-pay/ ；Business Insider https://www.businessinsider.com/google-gemini-4-argon-ai-model-public-release-cost-2026-10 | Ars：“it’s only coming to paying users at this time, and even those on lower-tier plans won’t have full access.”；“In Antigravity and AI Studio, the new model will only be available for those on AI Ultra plans.”；“Google has declined to clarify when (or if) Argon will come to Antigravity for other subscription tiers.” BI：“As is often the case with the most advanced AI models, Argon is available only to paying users.” | — | Ars 发布 2026-10-13T16:00:02Z（北京 10/14 00:00:02），作者 Ryan Whitwam；Ars 称 AI Studio 仅限 Ultra，公告写“paid tier in AI Studio”，两者说法不一致，以公告与价格页为准；存档 `c-m-arstechnica.*`、`c-m-businessinsider.*` | 部分支持（媒体转述，未见谷歌对免费用户的一手表述） |
| 官方人员补充 | L2 | https://x.com/OfficialLoganK/status/2110038940841545827 （fxtwitter JSON） | “Today we are rolling out Gemini 4 Argon globally in the Gemini API, to Pro + Ultra in Gemini App, and Ultra subs in Antigravity!”；“new features like 1M token output when the model needs to work more” | — | 发帖 2026-10-13 16:04:20 UTC（北京 10/14 00:04:20）；浏览 933,238、点赞 10,364、转发 740、回复 1,185（取数北京 10/14 约 17:40，Logan Kilpatrick 为谷歌员工，个人发言）；`c-x-logank.json` | 已找到 |
| 其他官方配套页 | L1 | Gemini 应用页 https://blog.google/innovation-and-ai/products/gemini-app/gemini-4-argon-availability/ ；开发者页 https://blog.google/innovation-and-ai/technology/developers-tools/build-with-gemini-4-argon/ ；Antigravity 页 https://antigravity.google/blog/gemini-4-argon-in-google-antigravity | 应用页：“Google AI Pro and Ultra subscribers can now access Gemini 4 Argon”；开发者页：“Start building using the paid tier of the Gemini API in Google AI Studio and Android Studio, and Google AI Ultra subscribers can try it in AI Studio, and our agent-first development platform, Google Antigravity.”；Antigravity 页：“Argon is rolling out starting today in the Google Antigravity desktop app and CLI for Google AI Ultra subscribers, eligible Gemini Enterprise subscriptions, and in the API in Gemini” | — | 应用页 datePublished 16:00:00Z；开发者页 16:00:00Z（修改于 10/14 01:40:50Z）；存档 `c-m-geminiapp.*`、`c-build-dev.*`、`c-antigravity-blog.txt` | 已找到 |

### C2 与 1001 期（9/30 首发）对照

对照物：1001 存档 `docs/2610/1001/sources/a-google-extract.json`（9/30 首发稿，2026-10-01 抓取）与本次重新抓取的同一页 `c-launch-0930.txt`（页面 datePublished 2026-09-30T20:00:00Z，dateModified 2026-10-01T21:09:53Z）。

| 项目 | 9/30 首发稿（1001 存档） | 10/13–14 正式开放 | 变化 |
|---|---|---|---|
| 开放对象 | “rolling out to a set of trusted cyber defenders through our Fairwind Program”；之后 “starting with paid API customers and Google AI Ultra subscribers”，无日期 | 付费 API 层、企业、Ultra 的 Antigravity、Pro 与 Ultra 的 Gemini 应用与 AI Mode；Fairwind 再扩 “hundreds of additional … partners” | 由受信任防御者扩大为付费用户；免费层不可用（价格页） |
| 限时价 | “$2 per million input tokens and $10 per million output tokens, with cached input tokens priced at 95% off input token price”；脚注 “After the introductory period expires, the price of $4 per 1M input tokens and $20 per 1M output tokens will apply.” 无日期 | 同价；脚注新增日期 “After the introductory period expires on November 30, 2026 … Applies to the Argon model intended for public access only.” 价格页 “$2.00 through November 30, 2026. $4.00 starting December 1, 2026.” “$10.00 … $20.00 …”；缓存 “$0.10 … $0.20”，“Implicit caching only (explicit caching is not supported).” | 价格未变，新增期满日期；新增脚注 “intended for public access only”；缓存仅隐式 |
| 上下文/输出上限 | “expanding the model’s output token limit to an industry-leading 1M tokens, up from the previous 64K tokens” | “industry-leading 1M unified input-output token limit via the Gemini API”；文档：“Context window (unified input and output token limit) 1,048,576”，“Max output 65,536 (default synchronous) / up to 1,048,576 (with long decode)”；“Long decode lets Gemini 4 Argon generate past 64K tokens … up to the unified 1M-token context window (1,048,576 tokens shared across input, thinking, and output)” | 口径由“输出 1M”改为“输入输出共用 1M”；超 64K 须后台执行；`background: false` 时 “Long decode coming soon” |
| 模型是否同一 | — | Business Insider 引谷歌 Gemini 产品负责人 Tulsee Doshi：“the model rolling out will be the one the company announced benchmarks for last month” | 同一模型（谷歌说法，经 BI 转述） |
| 9/30 页现状 | — | 9/30 页（2026-10-01 修改后）仍写 “Argon is currently rolling out to trusted cyber defenders through the Fairwind Program”（页首 AI 摘要）与 “Rolling out soon”，脚注仍无日期 | 旧页未更新，与新公告并存 |

价格页对 Argon 的写法与摘句见 C1 表与 `c-pricing.txt`（含 Batch、Flex、Priority 各档，Batch $1.00/$5.00 起、Priority $3.60/$18.00 起，均为 “through November 30, 2026”）。模型页 `c-model-page.txt`：“Stable: gemini-4-argon”，“Last updated 2026-10-13 UTC”。

### C3 ADK 文章

来源：https://developers.googleblog.com/en/optimizing-a-200000-line-python-codebase-overnight-with-gemini-4-argon/ （作者 George Weale、Bo Yang、Andrew Biernat；页面只标 “OCT. 13, 2026”，官方未给时刻；存档 `c-adk.html/.txt`）。

| 说法 | 级 | 原文摘句 | 截图文件 | 核对与口径 | 状态 |
|---|---|---|---|---|---|
| 规模：约 20.5 万行、近 800 源文件、近 1.7 万单元测试 | L1 | “The Python repository spans roughly 205,000 lines of code across nearly 800 source files, backed by nearly 17,000 unit tests.” | — | 自行核对：commit 1754b3e 时 `src/**/*.py` 783 个文件、共 205,829 行（含空行与注释，非空非注释行约 157,579）；测试数 16,892 为博文自报，仓库里 `def test_` 共 13,853 处（参数化会放大），未能复现 16,892 | 部分支持（行数、文件数吻合；“lines of code”按物理行数；测试数未复现） |
| 无人值守 11 小时；/goal 模式 | L1 | “Argon ran unattended for 11 hours overnight”；“When Argon started its run at 9:11 PM”；图 2 “11 hours overnight (9:11 PM to 8:19 AM)” | `../images/33-adk-figure2-round1-vs-round2.png` | 时区未标；第二轮跑法有 .agents/skills 规则库与一段话目标（“Open one pull request against main … Don’t merge it.”）；第一轮约 2 小时、零自定义规则，“Altered AuthConfig JSON schema”，只跑 216 个邻近测试；第一轮代码部分进入最终 commit：“Parts of round one’s code made it into the final commit” | 已找到 |
| 6 项优化、32 个文件 | L1 | “In total, Argon’s pull request proposed six optimizations across 32 files.” | `../images/27-adk-figure4-rejected.png`（图 4） | 该 PR 未公开：GitHub 上 commit 的关联 PR 为空（`c-gh-commit-1754b3e-pulls.json`），搜索 `Argon` 仅命中 1754b3e 一条；博文称 “merged pull request” 的链接实际指向 commit | 部分支持（数字出自博文，无公开原件可核） |
| 2 项发布 | L1 | “Two of those optimizations solved ADK’s core bottlenecks cleanly and shipped to production” | 同上 | “shipped to production”：commit 1754b3e 10/7 进主干；2.12.0 于 10/13 16:35 UTC 发布，PyPI 16:42 UTC；发版说明写 “Cut time to first model request by 40% in long sessions”。博文称最终是“a 22-file change backed by 17 new test functions”：commit 22 个文件（+956/−101）与之相符，补丁里新增 `def test_` 为 19 个（6+3+10），与“17”差 2 | 部分支持 |
| 被拒的四项及原因 | L1 | “A multi-agent event cache sped up a synthetic benchmark by 17%, yet relied on a weak memory reference that could return stale data across sessions if Python reused a memory address after garbage collection.” “A telemetry check read a private attribute on OpenTelemetry’s tracer implementation, which could break silently on a minor library upgrade.” “A rewrite of how ADK formats conversation history before each model call made a core code path harder to read without moving end-to-end latency on our independent holdout benchmark.” “A lookup index for workflow event replays added state-tracking complexity that only helped synthetic replay tests rather than everyday agent runs.” | `../images/27-adk-figure4-rejected.png` | 被拒项无公开 diff，无法独立核；图 4 仅有名称与 Shipped/Cut 标记，无数字 | 已找到（原因原句；内容未独立核） |
| 公开测试全过、内部 25 个目标失败 | L1 | “When Argon’s round-two pull request removed three internal flow imports from llm_agent.py, all 16,892 open-source tests passed, yet 25 internal Google test targets failed because they patched those exact attributes. The shipped fix uses lazy module accessors so heavy packages load only on first use while every public attribute stays patchable where tests expect it.” | `../images/28-adk-25-internal-failures.png` | 发生在**未合入**的第二轮 PR；合入版的 llm_agent.py 补丁改为 `_lazy.accessors` 懒加载（commit 里可见；补丁实际删去 4 条 import：3 个 Flow 类与 `find_matching_function_call`，博文写 “three internal flow imports”） | 已找到 |
| 图 1、图 5 数字 | L1 | 图 5：“First model request after 50 turns / In-memory session, garbage collection off / 64 ms / 36 ms / −43%”；“After 150 turns … 200 ms / 122 ms / −39%”；“import google.adk / Cold import in a new Python process / 1.22 s / 0.97 s / −21%”；“New process to first model request / Cold import plus first-turn setup / 3.07 s / 3.09 s / +1% (flat)”；“Whole 158-turn agent run / 3 rounds of 4 tool calls per turn, garbage collection on / 65.6 s / 56.1 s / −14% (−9.5 s)”；脚注 “Median of 6 to 12 runs against an instant model and instant tools.” | `../images/29-adk-figure5-flat.png`、`../images/32-adk-figure1-session-prep.png` | 与 commit 信息表一致（逐行对过）。正文：“time from a brand-new process to its first model request stays flat (3.07 s to 3.09 s)”；“you can’t feel 28 ms or 78 ms”（=64−36、200−122） | 已找到 |
| “约 40%”的范围 | L1 | “The 40% reduction in pre-model overhead applies to InMemorySessionService … External database, SQLite, Gemini platform, Firestore, and Redis session stores deserialize from storage rather than deep-copying in memory, so their read times don’t change here.” | `../images/29-adk-figure5-flat.png`（下方文字） | 仅内存会话；图 1 注 “In-memory sessions, Python garbage collection off” | 已找到 |
| 158 轮 65.6 s → 56.1 s | L1 | “removing them cuts the full 158-turn run from 65.6 seconds to 56.1 seconds (nearly 10 seconds of pure framework overhead)” | 同上 | 与图一致（−9.5 s、−14%） | 已找到 |

### C4 GitHub 核实

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 博文链接的“merged pull request and benchmark commit” | L1 | https://github.com/google/adk-python/commit/1754b3eeb6cdb2c16e3b30619a12066623df132e （API：`c-gh-commit-1754b3e.json`） | commit 标题 “perf: cut time to first model request by 40% in long sessions”；正文 “Benchmark (median of 6 to 12 runs with no-op model and tools that return instantly, measuring only ADK overhead; * = garbage collection disabled):” 及 5 行表；“Argon generated these changes”；“Co-authored-by: George Weale <gweale@google.com>”；“PiperOrigin-RevId: 995208187” | `../images/30-adk-commit-1754b3e.png` | 作者提交 2026-10-07T17:43:06Z（北京 10/8 01:43:06）；提交者 Copybara-Service 17:45:54Z；22 files，+956/−101；父提交 315c3dd；无关联公开 PR（API `commits/<sha>/pulls` 返回 `[]`）。表内两行 “first model request after 50 turns *”“after 150 turns *” 带星号（垃圾回收关闭），158 轮与进程级各行无星号 | 已找到 |
| 数字与博文一致性 | L1 | commit 信息表 | 64→36 ms −43%；200→122 ms −39%；65.6→56.1 s −14%；1.22→0.97 s −21%；3.07→3.09 s +1% | `../images/30-adk-commit-1754b3e.png` | 与博文图 5 逐行一致；博文总述 “about 40%” 取 −43%/−39% | 已找到 |
| 进入发布版本 | L1 | https://github.com/google/adk-python/releases/tag/v2.12.0 ；PyPI https://pypi.org/pypi/google-adk/json | “Performance: Cut time to first model request by 40% in long sessions. (1754b3e)” | `../images/34-adk-release-v2-12-0.png` | v2.12.0 2026-10-13T16:35:19Z（北京 10/14 00:35:19），发布人 GWeale（commit 作者本人）；PyPI google_adk-2.12.0 上传 16:42:55Z；v2.11.0（10/2）不含该 commit（`compare` 状态 diverged） | 已找到 |
| `.agents/skills` 存在 | L1 | https://github.com/google/adk-python/tree/main/.agents/skills | 目录含 adk-agent-builder、adk-architecture、adk-style 等；commit 时点 `adk-style/references/` 含 imports.md、visibility.md | — | 博文称第二轮先读了 visibility.md、imports.md | 已找到 |

### C5 第三方评测（一手页，取数北京 10/14）

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| AA：Argon（High）指数 53、输出 token、成本 | L3 | https://artificialanalysis.ai/models/gemini-4-argon | “Gemini 4 Argon (High) scores 53 on the Artificial Analysis Intelligence Index, placing it well above average among comparable models (median: 27). When evaluating the Intelligence Index, it generated 110M tokens, which is somewhat verbose in comparison to the median of 85M.” “On average, it costs $1.99 per task to evaluate Gemini 4 Argon (High) on the Intelligence Index.” | — | 取数 10/14 17:12；与 1001 存档同数（当时 median 26/82M，现 27/85M）；页面仍有 “No publicly available API” 字样（图例/过滤提示，语义不明，未据此下结论）；`c-aa-model.txt` | 已找到 |
| 每任务输出 62k、为 Astra 两倍多 | L3 | 1001 存档 AA 文章 https://artificialanalysis.ai/articles/gemini-4-argon-google-top-three-labs | “Gemini 4 Argon averaging 62k output tokens per task, compared with 27k for GPT-6 Astra (max)” | — | 9/30 文章；AA 文章列表（10/14，`c-aa-articles.txt`）无 Argon 新文章；是否同一模型：BI 引谷歌称同一模型 | 已找到（9/30 文章；未见正式版新测） |
| AA 对比 Opus-5.5(58)、Muse Wonder Full(56) | L4（引 AA） | VentureBeat https://venturebeat.com/technology/google-makes-gemini-4-argon-broadly-available-with-limited-time-low-api-pricing-of-2-10-per-1m-token | 表：Claude Opus 5.5 58、Claude Sonnet 5.5 56、Meta Muse Wonder Full 56、Gemini 4 Argon 53；“That ties the model with OpenAI’s GPT-6 Astra and Anthropic’s Claude Fable 5.1” | — | 本人未能在 AA 一手页直接读到 Opus 5.5/Muse Wonder Full 的 58/56（模型页只列 Argon）；该表为 VB 转述 | 部分支持 |
| Arena Text 排名与分数 | L3 | Arena 官方数据集 https://huggingface.co/datasets/lmarena-ai/leaderboard-dataset （`text_style_control` 与 `text`，latest） | `gemini-4-argon-high`：style control 1524.8（1515.9–1533.6），vote_count 4,903，rank 1；无 style control 1533.2（1524.4–1542.0），rank 1。同表第 2：style control 下 muse-wonder-max 1512.7（1499.4–1525.9，1,944 票）；无 style control 下 claude-opus-5.5-high 1515.5（1508.0–1523.1） | — | `leaderboard_publish_date` 2026-10-13；数据集 README：2025-05-16 起 style control 为文本榜默认；arena.ai / lmarena.ai 页面被 Cloudflare 挑战页挡住（`c-arena-text.txt`），未绕过；`c-arena-hf-extract.json` | 已找到（数据集；官网页未能打开） |
| Arena WebDev | L3 | 同数据集 `webdev` | `gemini-4-argon-high` overall 1678.0（1662.6–1693.3），1,772 票，rank 9；webdev-react 1690.5 rank 8；webdev-html 1639.3 rank 16。榜首 claude-opus-5.5-max 1811.1 | — | 发布日期含 2026-10-13；“全面登顶”不成立 | 已找到 |
| Vals Index | L3 | https://www.vals.ai/benchmarks/vals_index | “As of October 13, 2026, Gemini 4 Argon ranks first on the Vals Index with 68.90%, followed by Claude Sonnet 5.5 (67.04%) and Claude Opus 5.5 (66.97%).” 表：Argon 68.90% ±0.97，成本 $15.68/测试，“$4 / $20”，13.40B / 142.01M tokens，46m33s；Sonnet 5.5 67.04% ±0.92。“Gemini 4 Argon had no fallbacks.” | `../images/35-vals-index-argon-first.png` | 取数北京 10/14 17:22；Vals 按 $4/$20（期满价）计成本，不是限时价 $2/$10；页面写更新 10/13，VB 写 “As of October 12”；Vals 单项：Argon 仅领先 Finance Agent v2（65.40%），Vibe Code Bench、Code Migration 为 Sonnet 5.5 领先，Terminal-Bench 4.0 为 Opus 5.5 领先；Vals 模型页写 Context Window 262k（与谷歌 1M 不同，口径待问）；`c-vals-index.txt` | 已找到 |

### C6 彭博与员工质疑

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 彭博 9/30 稿存在、导语 | L4 | https://www.bloomberg.com/news/articles/2026-09-30/google-grapples-with-employee-skepticism-about-new-gemini-model | “September 30, 2026 at 7:52 PM UTC … Updated on September 30, 2026 at 9:05 PM UTC”；作者 Julia Love、Davey Alba（1001 存档 `docs/2610/1001/sources/a-bloomberg-extract.json`） | — | 北京 10/1 03:52 发布、05:05 更新；本次直接打开彭博页遇 “Are you a robot?” 机器人验证页（`c-bloomberg-0930.txt`），未绕过；正文改由 Yahoo Finance 的彭博通稿存档（`c-bb-yahoo.txt`，datePublished 2026-09-30T19:52:42Z）与日本时报 10/1 转载（`c-bb-japantimes.txt`）取得，属授权转载，非彭博原站 | 已找到（原站正文未能打开；通稿与转载可读） |
| “员工称只会刷榜” | L4 | 同上（通稿） | “While Gemini 4 has performed well on benchmarks the industry uses to gauge model efficacy, it does less well when employees actually put it to work, according to people with direct access to the effort. The model struggles to handle certain coding tasks, said the people, who requested anonymity to discuss an internal matter.” “Gemini isn’t particularly adept at front-end design, which shapes how apps and websites look and feel, one person said.” “Gemini 4 appears to be affected by this process, said two people familiar with the model.”（指 benchmaxxing）“Google said it would be inaccurate to say that Gemini 4 is underperforming in areas such as coding.” “A Google employee familiar with model development said there is ‘large consensus’ internally at the company that Gemini 4 is at the frontier.” | — | 全部为匿名；“只会刷榜”不是原话；彭博说的是“能用的好结果与基准分数之间有落差”，并有反方；同稿还写 “it’s a very large model, according to a person familiar with its development” | 部分支持（匿名、有反方；“只会刷榜、前端不行”写成事实不成立） |

### C7 热度

| 项目 | 数字 | 取数时刻（北京） | 来源 |
|---|---|---|---|
| HN 50073879 “Gemini 4 Argon”（指向 deepmind.google/models/gemini/） | 1,338 分 / 931 评（含嵌套 927 条；Algolia）；此前 17:02 取数 1,273 / 892 | 2026-10-14 19:54:14（17:02:38） | `c-heat.json`、`c-heat-1702.json`、`c-hn-50073879.json`；发帖 10/14 00:03（16:03 UTC） |
| HN 50074869 “Gemini 4 Argon for Robotics” | 222 / 50（17:02：215 / 49） | 同上 | `c-hn-50074869.json` |
| HN “Gemini 4 Argon is now available for everyone”（blog.google，50074821） | 2 / 2 | 同上 | `c-hn-50074821.json` |
| HN ADK 博文 50076291 | 3 分 / 0 评 | 同上 | `c-hn-50076291.json`；ADK 实验在 HN 上基本无讨论 |
| HN Ars “…only avaliabe to ultra users on Antigravity” 50080057 | 1 / 1 | 同上 | |
| Reddit r/GeminiAI “Why say pro have argon when they dont” | 452 分 / 178 评（发帖 10/14 00:45） | 约 10/14 17:40 | `c-reddit-search.json`（opencli reddit search，只读） |
| Reddit r/GeminiAI “Gemini Argon is now available to Pro users” | 285 / 84（01:52） | 同上 | 同上 |
| Reddit r/singularity “Gemini 4 Argon is live” | 261 / 106（00:07） | 同上 | 同上 |
| Reddit r/GeminiAI “GEMINI 4 ARGON AVAILABLE FOR ULTRA USERS in ANTIGRAVITY” | 123 / 52（10/13 23:47） | 同上 | 同上 |
| Reddit r/Bard “Gemini 4 argon is publicly available” | 100 / 38 | 同上 | 同上 |
| Reddit r/GeminiAI “Gemini Argon BURNS through 5-hour Limit” | 30 / 28（03:42） | 同上 | 同上 |
| Reddit r/GeminiAI “Gemini Argon is ranked #1 on Text Arena.” | 23 / 2 | 同上 | 同上 |
| Reddit r/LocalLLaMA | 本轮四个查询（Argon available/ADK/Arena/Bloomberg）无相关帖入围 | 同上 | |
| AIHOT 热点榜 | NO.01 “Google 发布 Gemini 4 Argon，仅付费用户可用”，热度指数 604（+81 相对上期，20 个来源、87 位参与者）；相邻：NO.02 OpenAI 窃听案 457、NO.03 Muse Wonder 330；另有 “谷歌 Gemini 4 Argon 从定向开放走向全面上线” 热度 116；ADK 实验一条（“Google 用 Gemini 4 Argon 无人值守优化 20 万行 Python 代码库 ADK”）在精选流 | 10/14 19:35 更新（AIHOT 标注） | `c-aihot-hot.txt`、`c-aihot-home.txt`（L5；B 组当事人姓名已替换为 “[当事人]”） |
| 官方 X（Logan Kilpatrick）发布帖 | 浏览 933,238、点赞 10,364 | 约 17:40 | `c-x-logank.json` |

Reddit 评论线索（L6，只作吠点线索）：r/GeminiAI 1x51smr 评论 “Google slowly rolls out updates to users in batches, not all at once usually.”（71 分）；1x53mtp 评论 “All these posts of Argon is now available to Pro users is missing the fact that this is a gradual rolling out rather than being immediately available”（29 分）；1x56lmf（楼主称 Pro 一次提问用掉 5 小时额度的约 10%，另一用户称 5%、13%；“Safe to say Pro is a paid trial of Argon”5 分）。

### C8 夸大/易误读的标题与句子

| 标题或句子 | 发布方 | 原站 URL | 时间 | 问题 |
|---|---|---|---|---|
| “Gemini 4 Argon is now available for everyone”（`<title>`） | 谷歌（blog.google，页面 H1 为 “Gemini 4 Argon: Built to solve your hardest problems”） | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/ | 2026-10-13 16:00 UTC | “everyone” 与正文的付费范围不符；HN 帖直接用了该标题（50074821） |
| “刚刚，Gemini 4全球上线” | 新智元（36氪转载） | https://m.36kr.com/p/4024861209890695 | 2026-10-14 10:44（页面标注，应为北京时间） | “全球上线”未交代需付费；同文 “AI通宵干11小时，交出工程成果 ：Argon跑完16892项单元测试” 把“跑完测试”当成果，原文是 PR 通过公开测试、仍有 25 个内部测试目标失败（未合入版）；同文写 “维护者从中保留了2项，并结合第一轮成果合入了主分支”，与博文一致 |
| “Optimizing a production Python codebase in hours” / “Working autonomously overnight, Argon surfaced optimizations, now live in google/adk-python, that cut in-memory session prep time by 40% in long sessions” | 谷歌（公告页 “Solving complex engineering tasks” 节） | 同公告页 | 同上 | 省略“6 中只合 2”“垃圾回收关闭”“仅内存会话”；“hours”与博文 11 小时 + 人工评审 |
| “Optimizing a 200,000-line Python codebase overnight with Gemini 4 Argon” | 谷歌开发者博客 | https://developers.googleblog.com/en/optimizing-a-200000-line-python-codebase-overnight-with-gemini-4-argon/ | 2026-10-13 | 标题易读成“一夜优化了 20 万行”；实际是对整个仓库找跨文件瓶颈，合入 22 个文件 |
| “Gemini 4 Argon proposed six code optimisations. Google’s engineers shipped two.” | Instagram 帖（未取证作者与时间，来自搜索摘要） | https://www.instagram.com/p/Dedjx0rE2mu/ | 未取 | 属中立转述，非夸大 |
| “Google makes Gemini 4 Argon broadly available with limited-time low API pricing of $2/$10 per 1M token” | VentureBeat | https://venturebeat.com/technology/google-makes-gemini-4-argon-broadly-available-with-limited-time-low-api-pricing-of-2-10-per-1m-token | 2026-10-13 | “broadly available” 但正文写 “Antigravity access for individual subscribers requires the more expensive Ultra tier” |
| “Gemini 4 Argon begins rolling out in Gemini app and Antigravity, but only if you pay” | Ars Technica（Ryan Whitwam） | https://arstechnica.com/ai/2026/10/gemini-4-argon-begins-rolling-out-in-gemini-app-and-antigravity-but-only-if-you-pay/ | 2026-10-13 16:00:02 UTC | 准确指出付费门槛，可作对照，不是夸大 |
| “Gemini 4 Argon Beats GPT-6 Astra at 40% the Cost” | ProPakistani | https://propakistani.pk/2026/10/01/gemini-4-argon-beats-gpt-6-astra-at-40-the-cost/ | 2026-10-01 | 搜索结果标题，未打开原文；AA 指数 53 对 Astra(max) 持平而非领先，“40% the Cost”指每任务成本约 60% |
| 中文“一夜自动优化20万行代码”“全部合并”“全面登顶”类标题 | — | **未找到**：中英文搜索（WebSearch、firecrawl）都未命中以此措辞写 ADK 实验的标题；36kr 一篇出现“AI通宵干11小时”与“全球上线” | — | “全面登顶”：Arena 文本榜第 1、Vals Index 第 1，但 WebDev 第 9、AA 指数 53（VentureBeat 转述的 AA 表中低于 Opus 5.5 的 58、Sonnet 5.5 与 Muse Wonder Full 的 56） |

### C9 有理有据的质疑（线索，原文可核者标出处；HN 评论分数不公开）

1. “6 中 2”的意义：博文自己写 “the agent explores, the maintainer subtracts”（图 4）；被拒四项的理由里 2 项针对合成基准（17% 提速、replay 测试），2 项是正确性/可维护性风险。公开 commit 只有合入版，被拒项无法复核（来源：博文、GitHub）。这是观察，不是社区评论。
2. 公开测试全过、内部 25 个目标仍失败（博文原句，见 C3）：说明公开测试集不能代表私有依赖；该失败版没有合入。
3. 首请求时间没变：“time from a brand-new process to its first model request stays flat (3.07 s to 3.09 s)”，博文自己建议生产服务器先发预热请求；发版标题却写 “time to first model request by 40%”。“−43%/−39%”均在垃圾回收关闭、模型与工具即时返回、仅内存会话的合成基准下；整机 158 轮 −14%。这些均为博文与 commit 自述。
4. 路径与方法：提示词里写明 “Open one pull request against main that the ADK maintainers would approve, with before and after numbers in its description”；第二轮有人工写好的规则库（.agents/skills）。第一轮零规则库时 Argon 改动了 AuthConfig 的公开 JSON schema（博文原句 “subtly altered its public JSON schema”）。
5. 自报路径：“11 小时”是第二轮无人值守的运行时间；人工评审与把两轮合并成 22 文件的时间未写（推断：不含在 11 小时内）；commit 作者与发版人为博文作者之一（GWeale），属厂商自测自报。
6. 额度质疑（Reddit，L6）：见 C7 末段；HN 评论 50075927（付款失败）、50079455（点击 “Try in Gemini” 默认进旧模型，要升级才可用）、50074155（“Couldn’t you just say what the price will be after the introductory period and when it’ll take effect?”）。HN 最大评论树：simonw 的 pelican 评测 50074156（子树 138 条）、postalcoder 的写作评价 50074108（103 条），均为正面。
7. 独立试用的反面例子（L6）：HN 50077034（自建 11 个小 bug 的评测：Argon 解出 10/11，成本中位数 $0.52 对 Sonnet 5.5 的 $0.18、GPT-6.1 Sol 的 $0.09，约 24k 输出 token 对 3k）；50074851（图片转 HTML 测试，作者评 B 档，在 GPT-6.1 Sol 之下、Opus 5.5 之后）。
8. VentureBeat 引 Sam Witteveen（VentureBeat 合作者）：“This is a really, really, really good model. But is it the best model out there? I actually don’t think it is,”（L4，评价性）。
9. 谷歌自家口径冲突：9/30 稿 “starting with paid API customers and Google AI Ultra subscribers”，现开放到 Pro 的 Gemini 应用；Ars 称 AI Studio 仅 Ultra，公告写 API 付费层可用，社区对 Pro 能否在 Antigravity 使用多有困惑（HN 50074627、50078187 等；Antigravity 官方页写 Ultra）。

## 三、可能的吠点（原文里有、线索摘要没提或说反的条件）

1. “Available for everyone” 实为付费：免费 API 层 “Not available”，AI Pro 套餐不含 API 使用（价格页）。
2. “1M” 是输入、思考、输出共用的窗口；单次同步调用输出上限仍是 65,536；超出需后台执行；后台长解码最长 24 小时，客户端续写 20 分钟（长解码文档）。
3. 限时价到 11 月 30 日，12 月 1 日起 $4/$20；Vals 的成本表已按 $4/$20 计，AA 的 $1.99/任务按 $2/$10 计。
4. Arena Text 第 1 的优势在置信区间内（与第 2 名重叠）；WebDev 第 9。
5. 公告把 ADK 实验放在“Solving complex engineering tasks”首条之一，但 ADK 博文里 4/6 被拒，且 “~40%” 是合成基准、GC 关闭、仅内存会话。
6. ADK commit 早在 10/7 就进了主干，博文 10/13 才发；“shipped”实为 10/13 的 2.12.0 版本。
7. 员工质疑是匿名的、有反方；谷歌称 “inaccurate”。
8. 9/30 首发页至今仍写 “currently rolling out to trusted cyber defenders”。

## 四、扫描说法勘误

| 扫描说法 | 核对结果 |
|---|---|
| ADK “近 800 个源文件、205,000 行、近 17,000 个单元测试” | 文件数、行数吻合（783 文件、205,829 物理行）；17,000 测试数未复现 |
| “PR 提出 six optimizations across 32 files” | 博文原句属实，但该 PR 无公开原件；合入的是 22 文件 commit |
| “Two … shipped to production” | 进主干 10/7；发版 10/13 16:35 UTC（2.12.0） |
| “all 16,892 open-source tests passed, yet 25 internal Google test targets failed” | 属实，且发生在未合入的第二轮 PR |
| “158 轮 65.6 s → 56.1 s” | 属实（−14%）；垃圾回收开启 |
| 自家员工向彭博爆料“只会刷榜、前端不行” | 与原文不符（匿名、有反方、“appears affected”、“one person said”） |
| AA 正式版 53、62,000 token | 数字属实但来自 9/30；未见新测 |
| Text Arena 第一 1525±9 | 属实（style control 默认视图，1524.8，区间 1515.9–1533.6，与第 2 名区间重叠） |
| 公告开放范围 | 一致；另须补：免费层不可用、AI Pro 套餐不含 API |
| 1001 的“1M 输出” | 现口径改为 1M 统一窗口，单次默认输出 65,536，超出需后台执行 |

## 五、未取到

- arena.ai / lmarena.ai 页面（Cloudflare 挑战页，未绕过；以 Arena 官方 HF 数据集代替）。
- 彭博原站正文（机器人验证页，未绕过；以授权通稿 Yahoo Finance、日本时报代替）。
- AA 一手页上 Opus 5.5 = 58、Muse Wonder Full = 56 的原始页面（仅见 VentureBeat 转述）。
- ADK 第一轮、第二轮的内部 PR，以及被拒的四项改动（无公开原件）。
- Reddit r/LocalLLaMA 相关帖、HN 评论分数（HN 不公开）。
