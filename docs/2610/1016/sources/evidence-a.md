# 1016 期 A 组取证：DeepSeek 1017 更新与搜索 API（11 月起 Agent 里的搜索开始收费）

执行方 claude-sonnet-5-5；取证时刻 2026-10-17 20:48–21:10（北京时间，UTC+8；本机美东 EDT = 北京 − 12 小时）。只记原文，不评价。更新日志写明“特别针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy 等平台框架进行了优化”，下面照录。

存档文件均在 `docs/2610/1016/sources/`，前缀 `a-`；文中“存档”只写 `a-` 后的文件名。图片在 `docs/2610/1016/images/`。官方图表原图（PNG）另存为 `a-img-*.png`（原件，不入库）。

## 0. 时间线（北京时间）

| 北京时间 | 事件 | 来源 / 级 |
|---|---|---|
| 2026-10-17 10:40:38（UTC 02:40:38） | HF 仓库 `DeepSeek-V4.1-Flash-1017` createdAt | HF API，L3；仅是仓库创建时刻，不等于公开时刻（官方未给公开时刻） |
| 2026-10-17 13:46:55（UTC 05:46:55） | HF 仓库 lastModified（抓取时） | HF API，L3 |
| 2026-10-17 约 16:11 | 官方公众号“DeepSeek”文章（AIHOT 显示的时间；公众号原文页要 JS 渲染，未能直接读取） | AIHOT 条目页，L5（线索）；见“失败的抓取” |
| 2026-10-17 16:17:20 | IT之家首发稿页面时间 | ithome.com，L4（最早的媒体页，非官方） |
| 2026-10-17 16:21（UTC 08:21） | HN 帖“DeepSeek-v4.1-Flash-1017”（HF 链接） | HN |
| 2026-10-17 16:35:54（UTC 08:35:54） | 官方 X 账号 @deepseek_ai 发帖 1/4（`https://x.com/deepseek_ai/status/2111375640717033777`） | fxtwitter JSON，L1 |
| 2026-10-17 16:37（UTC 08:37） | HN 帖（OpenRouter 链接） | HN |
| 更新日志、新闻页、指南页 | “时间: 2026-10-17”，官方未给时刻 | L1 |

结论：更新日志、新闻页、指南页只有日期，**官方未给时刻**。官方社交账号可核到的最早时刻是 X 帖 16:35:54（北京时间）；公众号 16:11 为聚合站显示，未直接核实。

## 1. 逐条清单

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1-a | 更新日志 1017“开放自研搜索 API” | L1 | https://api-docs.deepseek.com/zh-cn/updates/ ；英文 https://api-docs.deepseek.com/updates/ | 中文：“DeepSeek 网页端与 API 端的搜索能力均由自研 DeepSeek 搜索引擎提供底层支持，稳定处理每天来自全球各地的数亿次搜索请求。”“索引了全球千亿级网页，支持多语言搜索，实时返回高相关、高权威、高时效的优质结果。”“针对 SEO 作弊、GEO 操纵和低质 AIGC，我们通过严格的识别机制进行过滤，确保向用户与 LLM 呈现权威可信的信息。”英文：“reliably handles hundreds of millions of search requests from around the world every day”“indexes hundreds of billions of web pages” | images/01-changelog-1017-two-entries.png | “时间: 2026-10-17”，官方未给时刻；存档 `updates-zh`、`updates-en`。**更新日志这条没有提价格与 11/1**（grep “11 月”“November”均 0 处） | 已找到 |
| A1-b | 更新日志 1017“DeepSeek-V4.1-Flash-1017 更新” | L1 | 同上 | “DeepSeek V4.1 Flash 已升级至 DeepSeek-V4.1-Flash-1017，官方网页端与 APP 模型也已同步更新至新版本。API 调用方式与价格保持不变，仍使用 deepseek-flash 即可访问最新模型。”“相比上一版本的 DeepSeek V4.1，新模型在各基准测试中取得了显著提升”“我们特别针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy 等平台框架进行了优化。请升级至 DeepSeek Harness 0.2.1-rc.1 及以上版本，以获得适配新版模型的最佳体验。”英文：“We specifically optimized for DeepSeek Harness, Tencent CodeBuddy, WorkBuddy, and other platforms.” | images/01-changelog-1017-two-entries.png | 同上；“在提升性能的同时，我们还缩短了模型的任务运行时间与输出长度……在多个 Harness 下达到新的性能-费用帕累托最优”是官方自述，原文未给时间/长度的具体数字（文字稿里没有；图见 07） | 已找到 |
| A1-c | 1017 成绩表 11 项与脚注 | L1 | 同上 | “HLE (w/o tools): 42.6 (44.0*)”“Terminal-Bench 4.0: 42.3”“DeepSWE v1.1: 77.0”“ProgramBench: 29.1”“NL2Repo-Bench: 68.2”“Automation-Bench v1.0.6: 60.5”“Agents' Last Exam: 35.4”“Apex Bench v1.1: 72.1”“Chartography (w/tools): 83.0”“BabyVision (w/tools): 92.5”“ZeroBench-main (w/tools): 51.0”；脚注“* 表示仅在 HLE 评测集的纯文本子集上进行了测试；Terminal-Bench 4.0 使用 Claude Code 框架测试，DeepSWE 使用 Mini-SWE-V2 框架测试；ZeroBench-main 使用 Pass@5 评测。” | images/01 | 共 11 项，与已知情况一致；全部为 DeepSeek 自报；更新日志页只有 DeepSeek 一列，竞品列见 A2-d | 已找到 |
| A1-d | 新闻页“DeepSeek-V4.1-Flash-1017 发布 2026/10/17” | L1 | https://api-docs.deepseek.com/zh-cn/news/news261017 ；英文 https://api-docs.deepseek.com/news/news261017 | “DeepSeek V4.1 Flash 新版本：许愿式编程一键成真”；“相比上一版本的 DeepSeek V4.1 Flash，新模型在各方面的基准测试中取得了长足的提升。”“本次新版本发布 DeepSeek API 新增对 Search 与 Fetch 接口的支持，相关能力正式向公众开放。两个接口价格均为 5 元 / 千次”“并将在两周后（北京时间 2026 年 11 月 1 日 00:00）开始按次收取调用费用。” | images/05（表）、images/06（搜索段） | 新闻页**有** 11/1 与价格；更新日志没有。“两周后”：10/17 → 11/1 为 15 天，原文如此。存档 `news261017-zh`、`news261017-en` | 已找到 |
| A1-e | 官方 X 帖 | L1 | https://x.com/deepseek_ai/status/2111375640717033777 | “🚀 DeepSeek-V4.1-Flash-1017 is here!”“Stronger benchmark performance, faster task completion, and shorter outputs than the previous version—a Pareto improvement in performance and cost. Now live on our website, app, and API.”“Optimized for @DeepSeekHarness, Tencent CodeBuddy, WorkBuddy, and more. Upgrade to DeepSeek Harness v0.2.1-rc.1 or later” | 未截（见假设） | 发布 2026-10-17 16:35:54 北京时间（Sat Oct 17 08:35:54 +0000 2026）；串“1/4”，2/4–4/4 未取到；抓取时 17367 赞 / 1632935 浏览 | 已找到 |
| A1-f | 官方公众号公告时刻 | L1 | https://mp.weixin.qq.com/s?__biz=Mzk0OTYwNzc3NQ%3D%3D&mid=2247485876&idx=1&sn=2606d3fa9ac085f94290ddd1d94649cc | 页面正文过短（需 JS 渲染），未读到 | 无 | AIHOT 条目显示该文 16:11（L5）；未直接核实 | 未找到一手来源（时刻）；官方未给时刻（文档页） |
| A2 | 1017 对 9/10 的逐项对比 | L1 | 更新日志两段；新闻页官方表格图 `https://api-docs.deepseek.com/zh-cn/img/v4.1_261017_benchmark_table_cn.png`；9/10 新闻页表格图 `…/img/v4.1_260910_benchmarktable_cn.png` | 见第 2 节对齐表 | images/01、04、05 | 全部 DeepSeek 自报；两张表测试框架不同时见脚注 | 部分支持（“全线上调”只对两表都有的项成立；见第 2 节） |
| A3-a | Search/Fetch 价格 | L1 | https://api-docs.deepseek.com/zh-cn/guides/search/ ；英文 https://api-docs.deepseek.com/guides/search/ ；价格页中文 https://api-docs.deepseek.com/zh-cn/quick_start/pricing ，英文 https://api-docs.deepseek.com/quick_start/pricing | 中文：“Search 和 Fetch 按请求次数计费，不按 token 或返回结果条数计费”；表“Search 每 1,000 次成功请求 5元”“Fetch 每 1,000 次成功请求 5元”。英文：“Per 1,000 successful requests $0.75”（Search、Fetch 各一行） | images/02、10 | 人民币与美元两个口径各自标价；存档 `search-zh`、`search-en`、`pricing-zh`、`pricing-en` | 已找到 |
| A3-b | “成功”的定义 | L1 | 同上 | “只有返回 HTTP 200 且产生结果的请求才会计费（Search 的 results 必须非空，Fetch 必须返回网页正文）。请求失败、搜索无结果或超时不计费。count 只控制返回的结果条数，不影响单次请求的价格。”英文：“A request is billable only when it returns HTTP 200 and produces a result (Search must return a non-empty results array, and Fetch must return page content). Failed requests, empty search results, and timeouts are not billed.” | images/02 | — | 已找到 |
| A3-c | 11/1 起收费范围 | L1 | 同上（指南“计费”节、价格页、新闻页三处同句） | “请注意，当前用户在 DeepSeek Harness/Claude Code/Codex 中使用 DeepSeek API 时，已经会通过 Server 端工具调用访问到 Search/Fetch 接口，这部分 Search/Fetch 接口调用目前未计费，并将于北京时间 2026 年 11 月 1 日 00:00 起按次收取调用费用。”英文：“These calls are currently free of charge; per-request billing starts at 00:00 Beijing time on November 1, 2026.” | images/02、06、10 | 北京时间 2026-11-01 00:00 = UTC 2026-10-31 16:00。这条限定在“DeepSeek Harness/Claude Code/Codex 中经 Server 端工具调用访问”的调用；直接调用 Search/Fetch API 的计费，指南原文写“按请求次数计费”，未单独写直接调用的免费期。更新日志未提 11/1 | 已找到 |
| A3-d | QPS | L1 | https://api-docs.deepseek.com/zh-cn/guides/search/#限速 | “对每个账号，Search 和 Fetch 分别独立限速，互不占用额度”；表“Search 5”“Fetch 5”；“QPS（每秒请求数）限制按账号粒度计算，与使用哪个 API Key 无关，并与模型并发限制相互独立。”“除了账号级限制，Fetch 还针对目标域名有 QPS 限制，超过该限制也会收到 HTTP 429 错误码。” | images/03 | 扫描称“各 5 QPS”成立；Fetch 目标域名 QPS 的具体数字未给 | 已找到 |
| A3-e | 中文价格页存在 | L1 | https://api-docs.deepseek.com/zh-cn/quick_start/pricing | 节名“网页搜索与抓取价格”；“Search 和 Fetch 是独立的 API，按请求次数计费。”表同上 | images/10 | 存档 `pricing-zh` | 已找到 |
| A4-a | Claude Code 里的搜索如何触发 | L1 | https://api-docs.deepseek.com/zh-cn/quick_start/agent_integrations/claude_code ；英文 …/quick_start/agent_integrations/claude_code | “DeepSeek API 原生支持 Claude Code 中的 Web Search 功能。在使用 Claude Code 的过程中，如果模型判断您的问题需要通过搜索功能才能满足，模型会调用 Web Search 工具，并通过 DeepSeek 提供的 API 进行搜索。因为调用 Web Search 工具会产生额外的大模型 API 请求来对获取到的搜索内容进行总结，因此会产生额外的模型 Token 费用。搜索请求本身还会按 Search API 计费规则 计费。”英文：“if the model determines that your question requires a web search, it will invoke the Web Search tool” | images/08 | 页面写明由模型自行判断触发；**未写默认开关，也未写能否关闭**（grep 无） | 部分支持（触发条件已找到；“能否关闭”未找到） |
| A4-b | Codex 里的搜索 | L1 | https://api-docs.deepseek.com/zh-cn/quick_start/agent_integrations/codex | “Codex 内置的联网搜索仅在使用 deepseek-v4-pro 时可用。一键脚本会写入 web_search = "disabled"；如需使用联网搜索，请在运行脚本后将该字段改为 "live"。上方示例配置默认使用 deepseek-flash，因此同样禁用了联网搜索。”“保存后重启 Codex。搜索由 DeepSeek 在服务端执行，请求按 Search 接口的计费规则 计费。” | images/09 | 文档层面：一键脚本默认关闭、仅 V4 Pro 支持；英文页同（`web_search = "disabled"`、“Available only with deepseek-v4-pro”）。Flash（deepseek-flash）不在其列 | 已找到 |
| A4-c | Anthropic API 兼容页 | L1 | https://api-docs.deepseek.com/zh-cn/guides/anthropic_api | 兼容表：“array, type = "server_tool_use" 支持”“array, type = "web_search_tool_result" 支持” | 未截 | 只是消息格式字段支持表，未写默认开关 | 部分支持 |
| A4-d | DeepSeek Harness 里的 Search/Fetch | L1 | https://deepseek-harness.github.io/deepseek-harness/en/guide/quickstart 、`…/guide/providers`、`…/guide/mcp-memory` | 三页全文 grep 无 DeepSeek Search/Fetch 默认开关或关闭方式的描述 | 未截 | 存档 `harness-quickstart`、`harness-providers`、`harness-mcp`；Harness 仓库其余文档未逐页查 | 未找到一手来源 |
| A4-e | WorkBuddy/CodeBuddy 接入页 | L1 | https://api-docs.deepseek.com/zh-cn/quick_start/agent_integrations/workbuddy | 全文无搜索相关描述（grep 仅命中顶栏“Search / Fetch API 已上线”横幅） | 未截 | 存档 `workbuddy-zh` | 未找到一手来源 |
| A5 | 模型卡与许可 | L3 | https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017 ；API https://huggingface.co/api/models/deepseek-ai/DeepSeek-V4.1-Flash-1017 | 卡：“This repository and the model weights are licensed under the MIT License.”“DeepSeek-V4.1-Flash-1017 is a minor update to DeepSeek-V4.1-Flash, delivering substantial improvements across reasoning, coding, agentic tasks, and visual benchmarks.”；API：`"license:mit"`，`createdAt 2026-10-17T02:40:38.000Z`，`lastModified 2026-10-17T05:46:55.000Z`，`likes 510`，`downloads 0`，`safetensors.total 763205315794`（BF16 1,976,441,856；F32 42,307,282；F8_E4M3 204,015,223,296；I8 557,171,343,360）；卡页“Model size 763B params” | 未截 | 许可 MIT（卡与 API 标签一致）；参数量 763B 是 safetensors 张量总数；**卡片本身未写总参/激活参数分项**；OpenRouter 页写“552B-parameter backbone”“activates 8B parameters on input and 16B on output”（OpenRouter 自述，L3/L5，口径与 HF 张量总数不同，二者都记录，不取舍）。likes 510 为 20:48 抓取值 | 已找到 |
| A5-b | 模型卡成绩表 | L3 | 同上 | 卡内表与官方新闻页表格图数值一致（含竞品列：K3、GLM-5.3、GPT-5.6 Sol、Opus 5）；脚注“† denotes the text-only subset of HLE. We use the Claude Code harness for Terminal-Bench 4.0 and the Mini-SWE-V2 harness for DeepSWE v1.1.” | 未截 | 卡内未写 ZeroBench-main 的 Pass@5 | 已找到 |
| A6-a | AA | L3 | https://artificialanalysis.ai/models/deepseek-v4-1-flash 、https://artificialanalysis.ai/providers/deepseek | 页面标题“DeepSeek V4.1 Flash (Max)”，“Released September 2026”，“Artificial Analysis Intelligence Index 39”（#9 / 120），“175.9 Output tokens per second”；AA DeepSeek 供应商页“For intelligence, the top models on DeepSeek are DeepSeek V4.1 Flash (max) (39)” | 未截 | AA 页面名称无“1017”，发布月写 September 2026；全站 grep “1017” 0 处。**未见 1017 版本的独立成绩**（该 AA 条目是否已含 1017 无法从页面判断，不推断） | 未找到一手来源（1017 独立成绩） |
| A6-b | OpenRouter | L3 | https://openrouter.ai/deepseek/deepseek-v4.1-flash-1017 | “DeepSeek V4.1 Flash 1017 is the October 17, 2026 snapshot of DeepSeek V4.1 Flash, served by DeepSeek under the deepseek-flash API model name.”；“Released Oct 17, 2026”；上下文 1.0M；“In / Out Price $0.10 / $0.50 per 1M”；“DeepSeek V4.1 Flash 1017 is served by 3 providers on OpenRouter: inference.net, DeepInfra, and DeepSeek.”；基准节标注“(Artificial Analysis)”：“DeepSeek V4.1 Flash (Max) Intelligence Index 39.5”“HLE 39.2%”“AA-LCR 84.0%”“GDPval-AA 55.0%”“CritPt 14.3%”“SciCode 51.9%”“Terminal-Bench 4.0 26.8%”“AA-Omniscience Accuracy 46.4%”“Non-Hallucination Rate 3.5%” | 未截 | OpenRouter 展示的 AA 数据名称仍为“DeepSeek V4.1 Flash (Max)”，是否对应 1017 快照页面未说明；与 AA 自家页的“39”并存（39 vs 39.5，口径不同，全部记录）；AA 的 Terminal-Bench 4.0 26.8% 与 DeepSeek 自报 42.3（Claude Code 框架）并存，框架不同 | 部分支持 |
| A6-c | LMArena | L3 | https://lmarena.ai/leaderboard | — | — | 403，Cloudflare 质询，按规则不绕过 | 未找到一手来源 |
| A7-a | HN 热度 | — | https://news.ycombinator.com/item?id=50129121 ；https://news.ycombinator.com/item?id=50129045 | OpenRouter 链接帖：282 分 / 106 评（10/17 16:37 北京时间）；HF 链接帖：242 分 / 97 评（16:21）；官方 X 链接帖 `50129125` 8 分 / 0 评；runinfra `50129993` 3 分 | — | 取 `barking heat` 21:00 前后（20:52 抓取）；任务书写 253/230、20:30 前后——数字随时间变动，不矛盾。原始 JSON：`a-heat.json`；评论 `a-hn-50129121.json`、`a-hn-50129045.json`。Reddit 搜索被重定向到登录页，失败 | 已找到 |
| A7-b | AIHOT | L5 | https://aihot.news/story/f8045374-7106-49b1-8fd8-ce824a952e45 ；https://aihot.news | 首页热点榜“2 DeepSeek 发布 V4.1-Flash-1017 并开放权重 +26294 热度”（20:5x 抓取）；事件页“16 篇报道”“15 个报道来源”“热点榜第 2 名”；官方更新日志两条精选评分 62、67 | — | WorkBuddy 扫描称“+8106”是更早时刻的值，现值 26294；事件页热度走势“当前 269 / 峰值 269（10/17 20:00）”是另一口径（可比范围），均 L5，仅作热度线索；榜首是 Meta 开源模型（+77314） | 已找到 |
| A7-c | 国内媒体跟进数 | L4/L5 | AIHOT 事件页报道时间线 | 列出 16 篇报道/15 来源，含 IT之家、公众号 DeepSeek、Testing Catalog、The Decoder、MarkTechPost 等；“另外 11 家也报道了”（更新日志条目） | — | “十余家”与此相符；仅为聚合站计数 | 部分支持 |

## 2. A2 对齐表（全部来自 DeepSeek 官方页，存档 `updates-zh`、官方表格图 `a-img-v4.1_261017_benchmark_table_cn.png`、`a-img-v4.1_260910_benchmarktable_cn.png`）

### 2.1 两表都有的项（9/10 更新日志 → 1017 更新日志；前一版本列取自 1017 新闻页官方表格图，二者数值逐项一致）

| 项目 | 9/10（V4.1-Flash） | 1017 | 差值 |
|---|---|---|---|
| HLE（9/10 写“HLE”，1017 写“HLE (w/o tools)”） | 36.8 | 42.6 | +5.8 |
| HLE 纯文本子集（*） | 39.1 | 44.0 | +4.9 |
| Terminal-Bench 4.0 | 31.2 | 42.3 | +11.1 |
| DeepSWE v1.1 | 74.2 | 77.0 | +2.8 |
| ProgramBench | 20.3 | 29.1 | +8.8 |
| NL2Repo-Bench | 65.4 | 68.2 | +2.8 |
| Automation-Bench（1017 写 v1.0.6；9/10 无版本号） | 54.8 | 60.5 | +5.7 |
| Agents' Last Exam | 31.8 | 35.4 | +3.6 |
| Chartography (w/tools) | 78.9 | 83.0 | +4.1 |
| BabyVision (w/tools) | 89.6 | 92.5 | +2.9 |
| ZeroBench-main (w/tools) | 49.0 | 51.0 | +2.0 |

差值范围 +2.0 至 +11.1（AIHOT 综述写“较 9 月版提升 2.0 至 11.1 分”，与此一致；官方页没有这句）。官方表述是“各基准测试中取得了显著提升”（更新日志）/“各方面的基准测试中取得了长足的提升”（新闻页）。

### 2.2 只在 9/10 出现、1017 表里没有的项（9 项）

GPQA Diamond 90.9；Codeforces (Rating) 3471；MathArena Apex 65.6；Terminal-Bench 2.1 90.6；Terminal-Bench 3.0 30.0；CyberGym 88.1；SEC-Bench Pro 62.8；ExploitGym 15.3；HLE (w/tools) 63.9。（9/10 表共 19 项 = 两表共有 10 个基准 + 此 9 项；上表 HLE 与其纯文本子集同属一个基准，故 11 行对 10 个基准。）

### 2.3 只在 1017 出现、或改名/改版本的项

- **Apex Bench v1.1: 72.1**（1017）。9/10 更新日志与 9/10 新闻页表格里没有“Apex Bench v1.1”；只有“MathArena Apex: 65.6”。1017 官方表格图里“前一版 V4.1-Flash”列的 Apex Bench v1.1 是 **64.6**，与 9/10 的 65.6 不同，且 9/10 没有这个 64.6。两者是否同一基准，官方未说明；本表不把它们对齐，不推断原因。8/21 更新日志另有“ApexBench (Pass@1): 36.5”（V4-Flash-Vision-Exp 页），名称也不同。
- **Automation-Bench v1.0.6**：1017 带版本号；9/10 写“Automation-Bench”，数值 54.8 与 1017 表中前一版 54.8 相同。
- **HLE (w/o tools)**：1017 新增“w/o tools”标注；9/10 同一数值标“HLE”，另有“HLE (w/tools): 63.9”，1017 表里不再出现 w/tools 一项。
- 脚注新增：“Terminal-Bench 4.0 使用 Claude Code 框架测试，DeepSWE 使用 Mini-SWE-V2 框架测试；ZeroBench-main 使用 Pass@5 评测。”9/10 更新日志脚注只有 HLE 纯文本子集一句。

### 2.4 官方表格图中的竞品列（1017 新闻页，原图；表头写“GPT 5.6-Sol”，DeepSWE 图例写“GPT-6 Sol”等，命名以原图为准）

列：DeepSeek V4.1-Flash-1017 / DeepSeek V4.1-Flash / Kimi K3 / GLM 5.3 / GPT 5.6-Sol / Claude Opus 5。官方用粗体标出每行最高值：Claude Opus 5 最高 6 行（HLE 56.3、Terminal-Bench 4.0 51.8、ProgramBench 37.0、NL2Repo-Bench 75.3、Chartography 84.0、BabyVision 94.1）；DeepSeek V4.1-Flash-1017 最高 4 行（DeepSWE v1.1 77.0、Automation-Bench v1.0.6 60.5、Agents' Last Exam 35.4、Apex Bench v1.1 72.1）；GPT 5.6-Sol 最高 1 行（ZeroBench-main 53.0）。（视觉读表所得，已对照 HF 卡内文字表，数值一致。）DeepSWE v1.1 一行：Opus 5 74.0、GPT 5.6-Sol 73.0、DeepSeek 77.0。表中 Terminal-Bench 4.0 的 GPT 5.6-Sol 为 39.9。官方图的 GLM 5.3 在 Chartography、BabyVision、ZeroBench-main 三项为“-”。

## 3. 可能的吠点（线索，须回到一手核实）

1. 更新日志的 1017 条目没写 11/1 计费；该句在指南“计费”节、价格页、新闻页。出处：`updates-zh` 对比 `search-zh`、`news261017-zh`。
2. 11/1 计费那句限定“DeepSeek Harness/Claude Code/Codex 中使用 DeepSeek API”经“Server 端工具调用”的调用；价格 5 元/千次（$0.75/千次）按“成功请求”计，失败、空结果、超时不计。出处：指南“计费”节。
3. Codex 的内置联网搜索仅 deepseek-v4-pro 可用，一键脚本默认写 `web_search = "disabled"`，示例配置用 deepseek-flash 因此也禁用。出处：codex 接入页。
4. Claude Code 的 Web Search 由模型自行判断是否调用，且“会产生额外的模型 Token 费用”，搜索请求另按 Search API 计费。出处：claude_code 接入页（图 08）。
5. “缩短任务运行时间与输出长度”“性能–费用帕累托”的依据是新闻页一张 DeepSWE v1.1 图（mini_swe_v2 框架，图上写“公开模型：DeepSWE v1.1 排行榜……数据日期 2026-09-22”，OpenAI 各模型取“OpenAI 官方 DeepSWE v1.1 图表”，“美元费用（含历史 V4 Flash）按 1 USD = 6.6862 CNY 折算（2026-10-16）”）；文字稿未给运行时间、输出长度的数字。出处：新闻页图（images/07、`a-img-v4.1_261017_perf_cost_cn.png`）。
6. 1017 表里 Terminal-Bench 4.0 用 Claude Code 框架、DeepSWE 用 Mini-SWE-V2 框架，ZeroBench-main 用 Pass@5；AA 自测的 Terminal-Bench 4.0 为 26.8%（OpenRouter 页转述 AA 数据，“V4.1 Flash (Max)”），框架不同，二者并存。出处：更新日志脚注；OpenRouter 页基准节。
7. 1017 对 9/10 的“Apex Bench v1.1”在 9/10 没有同名项，见 2.3。出处：两张官方表。
8. HF 卡称“minor update”，同一句称“substantial improvements”；Terminal-Bench 4.0 +11.1。出处：HF 卡 Introduction。
9. 官方更新日志写明针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy 等平台框架优化（照录，不评价）。
10. WorkBuddy Intl 称“10 月 31 日前免费开放”新模型（AIHOT 转述 WorkBuddy 帖，L5）：这是 WorkBuddy 的活动，与 DeepSeek Search/Fetch 的 11/1 计费是两件事；一手 WorkBuddy 帖未取（见失败）。
11. 参数量口径：HF API 张量总数 763B（含 FP8/I8 量化张量）；OpenRouter 页写“552B-parameter backbone”“8B parameters on input and 16B on output”。

### 社区质疑（HN，L6，仅线索；Algolia 条目数据不含评论分数，故无评论得分）

- HN 帖 HF 链接（242 分）：“Hilarious sentence: "DeepSeek-V4.1-Flash-1017 is a minor update … delivering substantial improvements …"”（评论 https://news.ycombinator.com/item?id=50129204 ，8 条回复）；“Those benchmarks can't possibly be real, right? Right?”（https://news.ycombinator.com/item?id=50129111 ）；“Looks very impressive especially considering the price and speed. Let's wait for AA to confirm how well their benchmarks hold up.”（https://news.ycombinator.com/item?id=50129383 ）。
- HN 帖 OpenRouter 链接（282 分）：有理有据的实测对比——“DS isn't better than Sol and Gemini in the benchmarks, but after working with them all for a while now I'm beginning to understand why people find DS _feels_ so much better.”（https://news.ycombinator.com/item?id=50129889 ，10 条回复）；某用户用 4 个代理审同一代码库，“deepseek v4.1 flash via deepseek API: took 20m to find one p1 issue … gpt 5.6 sol: took 5m30s for one p1 and one p2 issue”（https://news.ycombinator.com/item?id=50129977 ，个人实测，样本为一个 C++ 游戏引擎代码库）；搜索价格评论“The native web search api is 10x cheaper than OpenAIs.”（https://news.ycombinator.com/item?id=50129469 ，个人说法，未核）。

## 4. 夸大说法实例

任务书点名的“暴击 GPT-6 Astra”“双王炸”“搜索免费”：**用 WebSearch（美区索引）、AIHOT 事件页、国内媒体抽查均未找到这些原词的标题或文章**；WorkBuddy 扫描所说的自媒体来源未取得。以下是找到的最接近的实例，按“把事件写得比原文更满”的程度列，不评价：

| 发布方 / 时间（北京时间） | URL | 逐字原句 | 与官方原文的差别（仅对照） |
|---|---|---|---|
| TestingCatalog（X 帖，2026-10-17 16:54，UTC 08:54:16） | https://x.com/testingcatalog/status/2111380259295653976 | “BREAKING 🔥: DeepSeek released DeepSeek-V4.1-Flash-1017, a new open-weight model with frontier performance. > It scores 77% on DeepSWE 1.1 benchmark, outperforming GLM-5.3, Kimi K3 and models like GPT-5.6 Sol and Opus 5. … It also extremely cheap! 👀” | “outperforming”只对应官方表 DeepSWE v1.1 一行；同表中 Opus 5 在另外 6 行最高（见 2.4）。存档 `x-testingcatalog.raw.json` |
| The Decoder 标题，2026-10-17 10:33 UTC = 18:33 北京时间（页面 article:published_time） | https://the-decoder.com/deepseek-v4-1-flash-1017-beats-larger-rivals-in-agentic-tasks-while-once-again-massively-cutting-per-task-costs/ | 标题“Deepseek V4.1 Flash 1017 beats larger rivals in agentic tasks while once again massively cutting per-task costs”；正文“It also beats larger competing models like Claude Opus 5, GPT 5.6-Sol, Kimi K3, and GLM 5.3 in several coding and AI agent categories.”“none of them match the smaller Deepseek model's accuracy”“Independent benchmarks haven't come out yet” | 正文有限定“in several”“In Deepseek's own benchmarks”“Independent benchmarks haven't come out yet”，标题没有；“none of them match…”针对 DeepSWE 成本图。存档 `decoder` |
| BigGo Finance，2026-10-17 | https://finance.biggo.com/news/22811d03-2553-4d77-a598-b8d066de04aa | “The new version achieved top scores on multiple benchmarks including DeepSWE, Automation-Bench, and others, outperforming Claude Opus 5, GPT-5.6-Sol, Kimi K3, and GLM 5.3.”“the published evaluation data suggests the capability gains far exceed what the word "minor" implies.” | “top scores”官方表为 11 项里 4 项最高；时间以页面为准，未单独取页面时刻。存档 `biggo` |
| 快科技（腾讯新闻转载），2026-10-17 18:01 | https://news.qq.com/rain/a/20261017A0AWO400 | 标题“DeepSeek V4.1 Flash新版本发布 许愿式编程一键成真 网友怒赞：太上进 超级斩杀线” | 标题用网友评论；正文如实转述 11/1 计费与“两个接口价格均为5元 / 千次”。存档 `qq-1` |

“全线提升”类：AIHOT 综述写“DeepSeek 称新版在全部 11 项基准上较 9 月版提升 2.0 至 11.1 分”（L5，AIHOT 事件页）；MarkTechPost 摘要（AIHOT 转述）写“官方称在全部 11 项基准上超过 9 月版”。官方原文是“各基准测试中取得了显著提升”，未写“全部 11 项”；对 11 项表内数值，逐项确为上升（见 2.1）。MarkTechPost 原站 403，未取（见失败）。

“搜索免费”类：未找到把搜索写成永久免费的标题。找到的媒体都转述了 11/1（The Decoder：“Deepseek API users can access the features for free through server-side tools in Deepseek Harness, Claude Code, and Codex until October 31, 2026 (4:00 PM UTC).”；AI Weekly：“server-side free-tier calls expire November 1”）。

## 5. 扫描说法勘误

| 扫描说法 | 存档核对 |
|---|---|
| 更新日志“特别针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy 等平台框架进行了优化” | 更新日志原文是“我们特别针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy 等平台框架进行了优化”；新闻页是“特别地，本次我们着重针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy 等平台框架进行了优化”；X 帖英文“Optimized for @DeepSeekHarness, Tencent CodeBuddy, WorkBuddy, and more”。三处措辞略有差异，均已核。 |
| “更新日志正文没提 11/1 计费（须核）” | 核实：更新日志中英文全文 0 处“11 月”“November”。11/1 在指南“计费”节、价格页（中英）、新闻页（“两周后（北京时间 2026 年 11 月 1 日 00:00）”）。 |
| WorkBuddy 扫描称“基准全线上调” | 对两表都有的 10 个基准（上表 11 行，含 HLE 及其纯文本子集）成立，+2.0 至 +11.1；9/10 表里另有 9 项在 1017 表没有；Apex Bench v1.1 在 9/10 没有同名项（见 2.3）。 |
| “Search、Fetch 各 5 QPS” | 成立：指南“限速”表 Search 5、Fetch 5，按账号计，两者独立；Fetch 另有目标域名 QPS（数字未给）。 |
| “HF 模型卡标 MIT” | 成立：卡正文与 API 标签 `license:mit`。MarkTechPost 摘要称 1017 许可“Not disclosed”（AIHOT 转述，原站未取到），与 HF 卡不符。 |
| 已知情况：1017 成绩表 11 项各数值与脚注 | 逐项核对一致（含 HLE 44.0*）。 |
| 已知情况：9/10 那条“列 19 项” | 实为 19 条数值（GPQA Diamond、HLE、Codeforces、MathArena Apex、Terminal-Bench 2.1/3.0/4.0、DeepSWE、ProgramBench、NL2Repo-Bench、CyberGym、SEC-Bench Pro、ExploitGym、HLE (w/tools)、Automation-Bench、Agents' Last Exam、Chartography、BabyVision、ZeroBench-main），数值与已知情况一致。 |
| 已知情况：“Apex Bench v1.1 与 MathArena Apex 不是同一项” | 官方未说明二者关系；两者数值也不同（1017 表前一版为 64.6，9/10 为 65.6）。只记差异。 |
| 已知情况：HN 253/95 与 230/91 | 抓取时 282/106（OpenRouter 链接）与 242/97（HF 链接）；数值随时间上涨，不矛盾。 |
| 已知情况：AIHOT “+8106” | 20:5x 抓取为 +26294，排第 2；+8106 为较早时刻值（不能核）。 |
| 已知情况：“自媒体标题有‘双王炸’‘暴击 GPT-6 Astra’” | 未找到（见第 4 节）。 |
| 任务书“更新日志写明 …WorkBuddy 优化” | 另：AIHOT 转述 WorkBuddy Intl “10 月 31 日前免费开放”新模型，属 WorkBuddy 自家活动，未取一手帖。 |
| 新增 | 官方新闻页 `news261017` 存在且含基准对比表（含 Kimi K3、GLM 5.3、GPT 5.6-Sol、Claude Opus 5）与 DeepSWE 性能/成本图；已知情况只提更新日志与指南。 |
| 新增 | 英文价格页的高峰时段写 UTC 01:00–04:00、06:00–10:00，中文页写北京时间 9:00–12:00、14:00–18:00，二者换算一致；与搜索计费无关，仅备查。 |
