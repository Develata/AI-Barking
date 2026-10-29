# 1028 期 C 组取证：Claude Code 后台 agent 摘要费用（16.4% 与 2% 的分母）

执行：Claude Sonnet 5.5（claude-sonnet-5-5）。取证时点：北京时间 2026-10-29 19:40–20:15。所有时刻为北京时间（UTC+8），括号内为原时区。存档文件均在 `docs/2610/1028/sources/`，文件名前缀 `c-`；截图在 `docs/2610/1028/images/`（25–36）。抓取与截图记录见 `capture-log-C.md`。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别说明：Flamerix Gist 是第三方公司自测、作者自述的原页，不是 Anthropic 来源，也不是 AA/Arena 那类评测机构，不落在 L1–L4；写进正文只能是“Flamerix AI 称”，其数字不能标 ✅ 为事实，只能标 ✅ 为“该公司如此称”。

## 一、事实清单

### C1 Flamerix Gist：16.4% 成本／19.7% 请求

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件／口径／时区 | 状态 |
|---|---|---|---|---|---|---|
| C1-1 `agent_summary` 占 Claude Code 全部花费 16.4%（主对话 15.7%），占全部 API 请求 19.7% | 第三方自测，作者自述 | https://gist.github.com/flamerix-ai/c3905705cc4205e5de3011fa2e74efce （存档 `c-gist-README.md`、`c-gist-api.json`、`c-gist-page.html`） | 表：“share of all Claude Code spend \| **16.4 %** (main conversation: 15.7 %) \| 0 %”；“share of all API requests \| **19.7 %**, about 1 in 5 \| 0 %”；“part of that spend on Opus \| 96 %”；“input tokens per output token \| ≈ 9,000 : 1”（列头：“week before the hook”“2 days after”） | 27-v2 | 一家公司自己的一周遥测（“week before the hook”，具体日期未写）。样本规模（会话数、人数、agent 数）与工作流类型 Gist 全文**未给**；只写“our machine”“our current production setup”。Gist 主页 flamerix.ai 自述其产品 DRIM 是“Production Control for AI-Driven Engineering”，试点“From $10,000”（`c-flamerix.txt`）。HN 提交者自称“Our DRIM system flagged the anomaly” | 已找到（数字属实地出现在原页）；样本与代表性未找到 |
| C1-2 成本口径是 `cost_usd` 估算而非账单，订阅用户损失的是限额余量 | 同上 | 同上 | “Spend here means Claude Code's own per-request estimate (`cost_usd`), not an invoice: on a subscription, what you lose is rate-limit headroom.” | 27-v2 | Gist 没有写“按挂牌价”；“挂牌价”一词来自 Anthropic 员工（见 C2）。HN 帖正文用的是“16.4% of total estimated spend”。HN 帖开头却写“used 16.4% of our token spend” | 部分支持（口径=估算花费，不是 token 占比，也不是账单；“挂牌价”在 Gist 里未出现） |
| C1-3 标题“Claude Code burns ~15–20% of your tokens on a 3-word status line” | 同上 | 同上 | 标题原文：“Claude Code burns ~15–20% of your tokens on a 3-word status line. Here's how to turn it off”；描述栏：“Claude Code burns ~15-20% of your tokens on a 3-word status line: a Bun preload that turns agent_summary off” | 26 | 正文只有两个数：16.4%（花费份额）与 19.7%（请求份额），分母不同；“15–20%”把二者并成区间，并把“一个团队的一周”写成“your tokens”；正文测的是估算花费不是 token | 部分支持（标题比正文表述宽） |
| C1-4 首次发布时刻与修订 | — | `c-gist-api.json`（GitHub gists API） | `created_at` 2026-10-26T22:27:47Z；`history` 只列两条：b44834d（2026-10-28T18:43:39Z，+338 行）与 2fe2241（2026-10-28T19:30:23Z，-24 行，仅删去 README 末尾的 MIT 许可证全文，见 `c-gist-rev-*.json`）；`updated_at` 2026-10-29T11:39:20Z（有评论所致） | — | 北京时间：创建 10-27 06:27:47；两条修订 10-29 02:43:39 与 10-29 03:30:23；最近活动 10-29 19:39:20。**矛盾**：创建时刻（10-26）早于历史里最早的修订（10-28 18:43Z）近两天，原因未找到（可能先建空/私密后内容整体改写，未核实）。README 称自己 10-26 起日常使用该补丁（“in daily use since 2026-10-26”）。作者在 issue #86879 的留言时刻 10-28T18:44:54Z（北京 10-29 02:44:54）= 首条修订后 71 秒。Gist 文件 README.md 12027 字节、no-agent-summary.js 4130 字节；HN 帖时刻 10-29 11:33:42 北京 | 已找到 |
| C1-5 “一周内”数值演变 | 同上 | 同上 | “In our telemetry its weekly share of spend went from 3.5–7 % in July (the fork gate was already open for us) to 7.3 % and then 11.8 % in the week we picked up 2.1.233, and to about 16 % by mid-October.” | — | 同一团队历史占比，说明数字随该团队 agent 用法与版本变化 | 已找到 |
| C1-6 “since 2.1.232 (August 2026) it has been on by default” | L1 部分印证 | https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md （`c-changelog.txt`），GitHub release v2.1.232 | 2.1.232 条目：“Subagent forking is now on by default: a `subagent_type: "fork"` subagent inherits the full conversation and prompt cache, and non-teammate agent spawns in interactive sessions now run in the background by default” | — | v2.1.232 发布 2026-08-13T23:29:59Z（北京 08-14 07:29:59）。CHANGELOG 该条没有提摘要（summary）；“摘要默认开启”是 Gist 作者读打包代码后的说法 | 部分支持（官方只写“fork 与后台运行默认开”，未写摘要） |

### C2 edwinarbus 的 HN 回复

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字，HN 评论 id 50284699） | 截图 | 条件／口径／时区 | 状态 |
|---|---|---|---|---|---|---|
| C2-1 回复时刻 | — | https://news.ycombinator.com/item?id=50284699 ；HN Algolia `https://hn.algolia.com/api/v1/search?tags=comment,story_50284255,author_edwinarbus`（`c-hn-edwinarbus.json`、`c-hn-item-50284255.json`、`c-hn-edwinarbus-comment.html`） | `created_at` 2026-10-29T04:52:07Z | 31 | 北京时间 2026-10-29 12:52:07；是对 OP 评论（id 50284256，03:33:42Z，北京 11:33:42）的回复，间隔 1 小时 18 分 25 秒。该账号在这个帖下只有这一条评论 | 已找到 |
| C2-2 机制：只有正在工作的后台 agent，约每 30 秒，空闲不做；摘要“供你和 Claude Code 追踪” | L2（署名员工公开发言，身份依据见 C3） | 同上 | “I work on Claude Code. You've got the basic mechanism right, but to be super clear, this only applies to background agents that are actively working. They get a summary call about every 30 seconds, and idle background agents don't. These summaries are used for both you and Claude Code to track what each background agent is working on, without reading transcripts.” | 31 | “used for both you and Claude Code” 与 Gist 的“it never reaches the model or your conversation”不是一个说法；“without reading transcripts”指摘要的用途。回复里没有否认“发送完整对话”，也没有确认；后文只说“a much cheaper mode that sends short notes to a smaller model instead of the full transcript”（暗示现状是发整段对话，但原句未直说） | 已找到 |
| C2-3 对 16.4% 的解读 | L2 | 同上 | “Your 16.4% sounds like it was for a team running mostly background agents, and doesn't seem like a typical session. (Feel free to send me the session ID at edwinarbus@anthropic.com and we can dig further.)” | 31 | “sounds like”“doesn't seem like”：推测语气，没有给出核对数据 | 已找到 |
| C2-4 官方数字：约 2% 成本（API 挂牌价）、不到 4% 请求；终端约 6% 成本 | L2 | 同上 | “Last week, across Claude Code users who send us telemetry, agent_summary was about 2% of cost at API list prices and just under 4% of requests. In the terminal, where nearly all of these calls happen, it was about 6% of cost.” | 31 | 口径：①“last week”相对 10-29 的前一周，具体日期未给；②样本=“users who send us telemetry”（选择了上报遥测的用户）；③成本=“at API list prices”；④2% 的分母是全部 Claude Code 用户的全部成本，6% 的分母仅是终端这一界面的成本——两个分母不同；⑤原句未给用户数、未给分布（均值还是中位）。与 Gist 的 16.4%（单团队、`cost_usd` 估算）的人群、成本口径、周都不同，不能直接相减 | 已找到（数字属实地出现；其内部数据不可核验） |
| C2-5 修复与计划 | L2 | 同上 | “That said, we agree that's a lot for a status line. We just rolled out a fix for the most expensive case, summaries that miss the prompt cache (2.1.311+), and right now we're working on a much cheaper mode that sends short notes to a smaller model instead of the full transcript.” | 31 | “most expensive case”= 未命中提示缓存的摘要，不是全部；“much cheaper mode”写作“working on”，即未发布 | 已找到 |

### C3 edwinarbus 身份

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| C3-1 自称在 Claude Code 工作；邮箱为 anthropic.com 域 | L2 候选 | HN 评论 50284699（同 C2） | “I work on Claude Code.” ；“edwinarbus@anthropic.com” | 31 | 邮箱是他自己写在公开评论里的 | 已找到（仅为本人公开自述） |
| C3-2 其 X 个人简介自称 Anthropic 的 Claude 产品与社区传播负责人 | L2 候选 | `https://api.fxtwitter.com/edwinarbus`（存 `c-fx-user-edwinarbus.json`，公开资料，无抓取者信息） | description：“Head of Product & Community Comms, @claudeai • formerly @openai @cursor_ai @stripe • email: edwinarbus@anthropic.com”；name “edwin”；认证标记 verified=true、type=individual；粉丝 37558（取证时） | — | 职衔是“Product & Community Comms”，即产品与社区沟通岗位，**不是工程师/遥测负责人**；他在 HN 说“I work on Claude Code”，没有说自己负责 agent_summary 或遥测。简介是本人填写，非 Anthropic 官方页面 | 已找到（本人公开简介） |
| C3-3 HN 与 GitHub 个人页 | — | https://hacker-news.firebaseio.com/v0/user/edwinarbus.json ；https://api.github.com/users/edwinarbus（`c-edwin-gh.json`） | HN about：“(fka edwinwee)” + X 链接，注册 2015-09-03（UTC），karma 2030；2026-10-07 另有一条评论同样以“I work on Claude Code.”开头（关于提示词建议功能）。GitHub：name “Edwin Arbus”，company 空，bio “(formerly edwinwee)”，blog edwinarb.us，twitter_username edwinarbus | — | HN 旧评论里还署过“Edwin from OpenAI here.”（2024-09-13）与“Edwin from Stripe here.”（2023、2022），与 X 简介“formerly @openai @cursor_ai @stripe”一致 | 已找到 |
| C3-4 Anthropic 官方页面出现此人 | L1 | 检索：`opencli brave search`“Edwin Arbus Anthropic Claude Code”“"Edwin Arbus" site:anthropic.com OR site:claude.com” | 仅有 LinkedIn 页面搜索摘要（第三方，Brave 摘要页，未打开）称 “Edwin Wee Arbus - Communications at Anthropic”；没有找到 anthropic.com 或 claude.com 官方页面署名此人 | — | 个人站 edwinarb.us 取证时无法取得（curl 30 秒内无内容，未再重试） | 未找到一手来源（仅自称 + 本人 X 简介；官方页面署名未找到） |

### C4 修复内容与版本

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| C4-1 2.1.311+ 的修复 | L1（CHANGELOG）vs L2（HN 回复） | https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md （`c-changelog.txt`，1.10 MB）；`https://api.github.com/repos/anthropics/claude-code/releases?per_page=30`（`c-releases-api.json`） | 2.1.311（发布 2026-10-26T21:51:23Z，北京 10-27 05:51:23）条目里**没有**提到 agent summary／sub-agent summary 的条目；最近的两条是“Fixed `/usage` never showing large prompt-cache misses under "What's contributing to your limits usage?"”与“Fixed `/effort` not warning about a prompt cache miss in plan mode when the plan model differs from the session model”（均与摘要无关）。2.1.312（10-27T20:24:20Z，北京 10-28 04:24:20）同样没有。全文检索 `agent_summary`、`progress summar` 无命中 | — | 因此“2.1.311+ 的修复”只有 Anthropic 员工在 HN 的一句话，CHANGELOG 无对应条目（可能服务端开关或未列出，原因未找到）。注意：员工评论时刻（10-29 12:52 北京）晚于 2.1.311 发布近两天 | 未找到一手来源（CHANGELOG 无条目；仅 L2 自述） |
| C4-2 同类修复此前已出现过（2.1.129） | L1 | 同上 | 2.1.129 条目：“Fixed sub-agent progress summaries missing the prompt cache (~3× `cache_creation` reduction)”；“Fixed sub-agent summaries firing repeatedly while a sub-agent's transcript is static, capping worst-case token cost on idle sub-agents” | — | v2.1.129 发布 2026-05-06T01:40:18Z（北京 05-06 09:40:18）。说明官方 5 月就承认并修过“摘要未命中缓存/对静止对话重复触发”。本次 2.1.311+ 的“miss the prompt cache”是否同一机制、是否回归：未核实（推断，不写进正文） | 已找到 |
| C4-3 Gist 所测版本 | 第三方 | Gist README “Tested versions” | 表：2.1.312（published 2026-10-27，“`latest` on 2026-10-28”）“5 / 5 / 0”；2.1.310 “5 / 5 / 0”；2.1.308（“`stable` on 2026-10-28”）“5 / 6 / 0”；2.1.301、2.1.280、2.1.257 等 | — | “5 / 5 / 0 = the fork happens on this version, merely loading the hook changes nothing, and `off` removes it.”测的是**摘要 fork 的次数**，不是花费；2.1.312 仍有 5 次/约 2.5 分钟，与员工所说“只修了未命中缓存的最贵情形”不矛盾（推断：修的是单次成本而不是次数） | 已找到 |

### C5 “只对活跃后台 agent、约每 30 秒”

| 说法 | 级 | 来源 | 对照 | 状态 |
|---|---|---|---|---|
| C5-1 回复与 Gist 的测量描述是否一致 | L2 / 第三方 | 回复（C2-2）与 Gist README | 一致处：约 30 秒、只对后台 agent。Gist 原文：“Every ~30 seconds, for every background agent, Claude Code sends that agent's **entire transcript** to the agent's own model (usually Opus)”；Gist “Why there is no setting” 说间隔由服务端旗标决定，范围 30–240 s（“clamped to 30–240 s. The default is already the minimum”）。Gist 补充“活跃”条件：2.1.280/2.1.257 的测试里“The agent sat at a permission prompt, so later timers skipped the unchanged transcript”（静止对话会跳过）。一致，且与 CHANGELOG 2.1.129 的“firing repeatedly while a sub-agent's transcript is static”相互印证 | 已找到 |
| C5-2 “实际发什么” | — | 同上 + issue #86879 | Gist 称发整段对话给 agent 自己的模型；员工回复称“sends short notes to a smaller model instead of the full transcript”为**将来**模式；issue 作者读 2.1.233 打包代码：“Each call re-sends the subagent's entire conversation history plus the full tool schema, on `claude-opus-5`”。Gist 称 2.1.310 已带 `short` 模式但默认关：“The default is still the full transcript, and only server flags can change it.” | 部分支持（“整段对话”只有第三方读代码的说法，官方未直述） |

### C6 issue #86879

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| C6-1 开启与状态 | 用户自述（GitHub 页面） | https://github.com/anthropics/claude-code/issues/86879 （`c-issue-86879-api.json`、`c-issue-86879-comments.json`、`c-issue-86879.html/.txt`） | 标题：“Feature request: a supported way to disable the agent_summary spinner (22.7% of requests, ~18% of weighted tokens in a subagent-heavy session)”；作者 rflcrz；labels area:agents、area:cost、enhancement、platform:macos；state open；评论 7 条 | 34 | 开启 2026-08-15T10:29:21Z（北京 08-15 18:29:21）；最近更新 2026-10-29T09:49:07Z（北京 17:49:07）。版本 2.1.233、darwin、cli。7 条评论 `author_association` 全为 NONE，**未见 Anthropic 员工回复** | 已找到 |
| C6-2 关键数字 | 用户自述 | 同上 | 表（一个 4 小时会话，27 个 subagent，1,496 个请求）：real agent turns 1,076 calls / 70.4% raw / 72.7% weighted；`agent_summary` 339 calls / 61,560,341 cache read / 21,966 output / 62,757,361 raw tokens / **22.2% raw / 18.0% weighted**；suggestion 37；task-notification 36。“Weighted at Opus 5 list rates (reads 0.1×, 5m writes 1.25×, 1h writes 2×, output 5×).” “**22.7% of all API requests in the session**, producing 21,966 output tokens total (~65 per call) of cosmetic status text.” | 34（表格最右“weighted %”列因页面横向溢出被截，数字见 `c-issue-86879-api.json`） | 单个会话、单人、用 `OTEL_LOG_RAW_API_BODIES` 捕获原始请求。三个分母并存：请求数 22.7%、原始 token 22.2%、加权 token 18.0% | 已找到 |
| C6-3 其他评论里的数字 | 用户自述 | 同上（评论 5304157191、5730931355、6348171825） | kcarriedo：“In sessions with 20+ subagents the summary generation is consistently in the 15-22% weighted token range.”；sylvain-artois（2.1.276，linux，1h45 会话，四个自定义 subagent，最长约 380k 上下文）：“That is **26 % of the session's cache-read tokens**”；rabitlord1324（2.1.300/301，Fable 5.1 subagents，五个会话）：“hidden cache reads = **0.7–1.08 ×** Σ(context of every sub-agent message that issued a non-read-only tool call)”，无原始请求捕获。Gist 把它们概括为“four independent measurements of 18–26 %” | — | 四组数各用不同分母（请求份额 / 加权 token / 缓存读 token / 与可见读的比值），且都是 subagent 密集会话；Gist 的“18–26%”是把这些并列，不是同一口径 | 部分支持（Gist 的概括与原 issue 的口径不同） |
| C6-4 `skipTranscript` 与成本工具看不见 | 用户自述 | issue 正文“Secondary observation”；另开 issue #104358（https://github.com/anthropics/claude-code/issues/104358 ，rabitlord1324，2026-10-22T18:26:53Z，open，0 评论，`c-issue-104358-api.json`） | “These calls carry `skipTranscript: true`, so they appear in **no** transcript. Any transcript-based cost tooling — including users' own — silently understates real token consumption by this amount.” | — | 同一用户在 #104358 称：“a transcript-based sum lands **×1.35–2.13 below** Claude Code's own `cost-state` (14 sessions, 2.1.273–2.1.300, Linux/WSL2, auto mode)” | 已找到 |
| C6-5 Anthropic 对 issue 有无回复 | — | 同上 | 7 条评论均非 Anthropic 员工（作者及 kcarriedo、uwuclxdy、sylvain-artois、rabitlord1324、flamerix-ai、JHVIW，关联均为 NONE） | — | 评论里的临时办法：`CLAUDE_CODE_FORK_SUBAGENT=0`（同时关掉 fork subagent）、`CLAUDE_CODE_ENABLE_NARRATION=0`（评论称仅 2.1.241 有效，2.1.245 起为死代码，来自 deepseek 的说法，未核实） | 已找到（未见官方回复） |

### C7 statusline 与 agent_summary 是不是一回事

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| C7-1 官方 statusline 文档：状态栏在本地跑、不耗 API token | L1 | https://code.claude.com/docs/en/statusline （`c-statusline-doc.txt`） | “The status line runs locally and does not consume API tokens. It temporarily hides during certain UI interactions, including the help menu and permission prompts.” | 35 | 该页讲的是你自己配置的 `statusLine` 命令（脚本经 stdin 收 JSON）。agent 面板每个 subagent 的“行摘要”是另一处：同页 `subagentStatusLine` 的 `label` 字段：“Short progress summary of the task when Claude Code has one, otherwise the same text as description”。文档没有把 agent_summary 请求写进“不消耗 token”的范围，也没有写它消耗 | 已找到（文档句存在；与 agent_summary 是不同的功能，Gist 与 HN 把它都叫 “status line”） |
| C7-2 agent view 文档：行摘要用 Haiku 级模型、与会话同价计费 | L1 | https://code.claude.com/docs/en/agent-view （`c-agent-view-doc.txt`） | “The one-line summary in each row is generated by a Haiku-class model … the row text updates at most once every 15 seconds from the session's own recent output without sending a model request, and the model writes a fresh summary when each turn ends.” ；“The end-of-turn summary and each mid-turn rewrite are one short Haiku-class request through your normal provider, billed and handled under the same data usage terms as the session itself. … On a third-party provider or gateway with no Haiku-class model configured, the request uses the session's main model instead” | 36 | 这是 agent view（`claude agents`）的“行摘要”：每回合结束一次，并在长回合中“every few minutes”改写。与 Gist/员工所说的“约每 30 秒、整段对话”描述不同。两者是否同一请求（`agent_summary`）：未找到官方说明（推断：可能是不同功能；未证实） | 已找到（文档句）；是否同一机制 未找到一手来源 |
| C7-3 官方“后台 token 用量”节有无列出 agent 摘要 | L1 | https://code.claude.com/docs/en/costs （`c-costs-doc.txt`） | 该节“Background token usage”列：“Conversation summarization: Background jobs that summarize previous conversations for the claude --resume feature”；“Command processing: Some commands like /usage may generate requests to check status”；“These background processes consume a small amount of tokens (typically under $0.04 per session) even without active interaction.”；之后是 prompt suggestions、goal check-ins、“Subagents and workflows: every subagent … sends its own requests on top of the main conversation's”、“Agent teammates”。全文检索 `agent_summary`、`progress summar`、`status label` 无命中 | — | “under $0.04 per session”针对空闲时的后台任务，不含活跃后台 agent 的摘要（该节没有列）。这是对 Gist 说法“The docs section on background token usage doesn't list it either”的核对：属实 | 已找到 |

### C8 Bun preload 补丁（只记存在与作者自述限制）

| 说法 | 级 | 来源 | 摘句 | 状态 |
|---|---|---|---|---|
| C8-1 存在非官方补丁 `no-agent-summary.js` | 第三方 | Gist（文件 4130 字节，存 `c-gist-no-agent-summary.js`） | “`no-agent-summary.js` is a Bun preload (~80 lines of code). It wraps `setTimeout`/`setInterval`; when a callback's source text contains a summarizer anchor … it returns an inert handle instead of arming the timer.”；“The binary is not modified (`BUN_OPTIONS=--preload`).” | 已找到 |
| C8-2 作者自述限制 | 第三方 | Gist | “Tested on Linux … An update can silently undo it”；“Platform: Linux tested, macOS and Windows untested. On Windows the `/$bunfs/` gate most likely never installs the hook.”；“Fail-open … If an update breaks that, the hook matches nothing and summaries silently come back.”；“Visible change: the per-agent panel status stops updating; SDK `task_progress` has no `summary`.”；许可条款：“Provided **as is** … Changing a client's runtime behaviour may conflict with the terms of the service you use it with; checking that is on you.” | 已找到（截图 28 含平台限制） |
| C8-3 第三方对补丁的反馈 | 用户评论 | Gist 评论 Bedz01（2026-10-29T10:44:15Z，北京 18:44:15，`c-gist-comments.json`）；issue #86879 评论 JHVIW（10-29T09:49:07Z） | Bedz01：“With a space, `--preload /path`, every bun subcommand breaks … `--preload=/path` works for both bun and Claude Code.”；JHVIW：“I wouldn't install that script blindly, the curl always pulls the latest version and it runs inside every Claude Code process.” | 已找到 |

### C9 热度与转述

| 项 | 数字／原文 | 来源 | 状态 |
|---|---|---|---|
| HN 主帖 | “Claude Code regularly sends each background agent’s transcript to its model”，链接指向上述 Gist；提交 2026-10-29T03:33:42Z（北京 11:33:42），提交者 NikZaal；`barking heat` 北京 10-29 19:48：424 分／155 评（Algolia 计，含嵌套；派工单所记 10-29 早先约 413／153） | https://news.ycombinator.com/item?id=50284255 （`c-heat.json`、`c-hn-50284255.json`、`c-hn-item-50284255.json`、`c-hn-page.*`） | 已找到 |
| Reddit | `barking heat` 的 Reddit 搜索重定向登录页，未取到；`opencli brave search` 找不到 r/ClaudeCode 相关帖 | — | 未找到 |

## 二、转述与夸大实例

均为 L5，仅作线索；不直接引用，不当来源。

| 序 | 发布方／URL | 标题或原句（逐字） | 时刻 | 夸大处 |
|---|---|---|---|---|
| 1 | Flamerix AI（Gist 作者本人）https://gist.github.com/flamerix-ai/c3905705cc4205e5de3011fa2e74efce | 标题：“Claude Code burns ~15–20% of your tokens on a 3-word status line. Here's how to turn it off” | 创建 10-27 06:27（北京），首条修订 10-29 02:43 | “your tokens”：把一家公司的一周估算花费（16.4%）与请求份额（19.7%）并成区间并推广给读者；员工称总体约 2%（C2-4）。正文脚注已写明是 `cost_usd` 估算 |
| 2 | daily.dev https://daily.dev/posts/claude-code-burns-15-20-of-your-tokens-on-a-3-word-status-line-kru9pp08x （`c-dailydev.txt`） | “Claude Code spends 15-20% of its API budget generating 3-5 word status labels for background agents.” ；“it accounted for 16.4% of total spend and about 1 in 5 API requests in one measured case” | `datePublished` 2026-10-29T04:33:00Z（北京 12:33） | 前一句写成一般性的“spends 15-20% of its API budget”；后一句保留了“in one measured case”。该页由 AI 摘要生成（“Community take”“Questions this post answers”） |
| 3 | 禁闻网 https://www.bannedbook.org/bnews/itnews/20261029/2374904.html （curl 被 Cloudflare 403 拦；用 opencli 浏览器读取页面文本，未存档 HTML，只记摘句） | 标题：“研究发现 Claude Code 的后台代理摘要功能导致约 15-20% 的 Token 浪费”；正文：“这一行为在未提供开关的情况下，占用了用户约 15% 到 26% 的 Token 额度及速率限制。” | 页面标注 2026年10月29日 15:00（北京） | 把一家公司的 16.4% 与 issue 里不同分母的 18–26% 都写成“用户”的“Token 额度”，并说成“浪费”；未提 Anthropic 的 2% 回应。**该站是面向翻墙读者的政治性新闻聚合站，风控：不引用、不放进成品** |
| 4 | AI Weekly（aiweekly.co）https://aiweekly.co/alerts/flamerix-gist-claude-code-status-line-ate-164-of-weekly-spend （`c-aiweekly.txt`） | 页面标出的“Original headline”：“Claude Code's Agent Summary Burns 15-20% of User Tokens Generating 3-Word Status Lines”；现标题：“Flamerix gist: Claude Code status line ate 16.4% of weekly spend” | “Published October 29, 2026 at 06:31 UTC”（北京 14:31），“Updated … 08:45 UTC” | 原标题夸大，现标题较准确并附 Anthropic 的 2%／6% 回应 |
| 5 | AI/TLDR https://ai-tldr.dev/releases/flamerix-claude-code-agent-summary-tokens/ （`c-ai-tldr.txt`） | “Claude Code's agent status line — a test says it used 16% of spend” | 页面日期 2026-10-28（只有日期，官方未给时刻） | 较准确，作为对照：保留“a test says”，并写了员工“about 2%”的回应 |
| 6 | HN 评论 emsign（id 50285181，06:25:15Z，北京 14:25:15） | “This is fraud. Unknowing to its customers a company is running a daemon that wastes a commodity it sells to said customer.” | 2026-10-29 | 社区个人评论，不是媒体；定性“fraud”无文档依据（文档 `costs` 与 `agent-view` 都未禁止后台请求；但官方 costs 文档确实没列出它，见 C7-3） |

## 三、可能的吠点（线索，进正文前须回一手核实）

1. 16.4% 的分母：单家公司（Flamerix，自己做 AI 工程管控产品，试点 $10,000 起）一周的 `cost_usd` 估算花费；样本规模、agent 数量、时间段未给。出处：Gist “The problem”；`c-flamerix.txt`。
2. 2% 的分母：Anthropic 员工自述的“上报遥测的用户上周”全体的挂牌价成本，6% 只算终端；样本与方法未公开，只有一句话。出处：HN 评论 50284699。两个数字人群、周、成本口径都不同，不能互相抵消或相乘。
3. “status line”这个词在这件事里有两个意思：`statusLine`（用户自配的脚本，官方说本地跑、不耗 API token）与 agent 面板的行摘要（耗 token）。官方 statusline 文档那句“does not consume API tokens”并不覆盖后者。出处：`statusline` 文档（C7-1）；HN simoncion 评论（id 50285189）就是这么读错的。
4. Gist 称摘要“never reaches the model or your conversation”，员工称摘要“used for both you and Claude Code to track what each background agent is working on”。出处：Gist “The problem”；HN 评论 50284699。
5. Gist 称“since 2.1.232 … on by default”；CHANGELOG 2.1.232 只写“Subagent forking is now on by default … non-teammate agent spawns in interactive sessions now run in the background by default”，没有提摘要。出处：`c-changelog.txt` 2.1.232 节。
6. 员工说 2.1.311+ 已修“最贵情形”，但 2.1.311、2.1.312 的 CHANGELOG 没有对应条目；Gist 在 2.1.312 上仍测到每 2.5 分钟 5 次 fork（测的是次数，不是花费）。出处：`c-changelog.txt`；Gist “Tested versions”。
7. 官方 5 月 2.1.129 就修过“摘要未命中缓存、对静止对话重复触发”（~3× cache_creation 降低）；这次 2.1.311+ 的缓存未命中修复可能是同类问题再现，未核实。出处：CHANGELOG 2.1.129。
8. 官方 agent view 文档说行摘要是 Haiku 级模型、“one short … request”，每回合结束与长回合“every few minutes”；Gist 与 issue 读代码称每 30 秒、发完整对话给 agent 自己的模型（常为 Opus）。是否同一请求未找到官方说明。出处：`agent-view` 文档（C7-2）；Gist；issue #86879。
9. Gist 自己写明：约 98% 的这类调用命中缓存，约 2% 未命中的占其成本三分之一以上（“About 98 % of these calls re-read the transcript from cache; the ~2 % that hit a cold cache carry over a third of its cost.”）。这与员工“修最贵的、缓存未命中的情形”正好对应（推断）。出处：Gist “Why there is no setting”（截图 30）。
10. 订阅用户损失的是限额余量而非账单：Gist 自己写“not an invoice: on a subscription, what you lose is rate-limit headroom”；员工的 2% 是“API list prices”。订阅限额如何折算未找到官方说明。出处：Gist；HN 评论。
11. Gist 与 HN 帖说“16.4% of token spend”，实际是估算花费。缓存读价格是输入价的 0.1 倍（issue 里的加权口径：“reads 0.1×”），所以请求份额（19.7%）与花费份额（16.4%）、token 份额不同。出处：Gist；issue #86879。
12. 补丁安全与适用：仅 Linux 测过；全局 `BUN_OPTIONS` 会让 `bun install/add/test/build/x` 报 “Script not found”（Bedz01）；curl 每次拉最新脚本并跑在每个 Claude Code 进程里（JHVIW）。出处：Gist 评论；issue 评论。
13. Gist 文笔被两位 HN 评论者指为 AI 写作（simoncion：“pretty obviously bot-generated”；jon-wood：“pure Claudeish”），只是线索，与事实无关。

### HN 社区高赞／有理有据评论（线索 L6；HN 不向匿名读者显示评论分数，故无分数。按帖内位置与回复量挑选）

- jakozaur（id 50284683，04:48:32Z，北京 12:48:32）：“I tested it, and it looks real but more nuanced. It is just for subagents; workflows do not seem affected. … In my Claude Code sessions, it looks more like ~8% at most.”（截图 33）
- simon83（id 50286504，09:36:32Z，北京 17:36:32）：让 Opus 读 Gist 后按 30 秒计时器估算自己 20x Max 一周：“API equivalent spending for this week: $1,970 Hidden status-label calls: ~$380 Share of the true total: 16%”；另一条（id 50286551）：“In my sessions it is ~16%, not using workflows”。这是估算，不是遥测。
- _boffin_（id 50284811，05:13:51Z）：“Just looked and we're burning around ~3%, but that's across everything / everyone.”
- abelw（id 50287280，11:18:10Z）：“I'm also at 15+%.”（口径与方法未给）
- HeavenFox（id 50284556）问“wouldn't almost all of the tokens be cache hit, and therefore fairly cheap?”；skerit（id 50286899）实测：“It's really just a temporary fork of the conversation, so it's fully cached. But it does run every 30 seconds, so that is very bad especially if it's a subagent that's at 800k tokens.”；minraws（id 50284621）：缓存命中也按 0.1 倍输入计费，多次累积；odo1242/kristjansson（id 50285526、50285683）反驳：每次工具调用同样重读，“It costs as much as one additional tool call in the conversation.”
- creamyhorror（id 50284641）：“the claim "15-20% of your tokens" seems unclear; of all tokens, or of tokens spent on background agents?”
- simoncion（id 50285189）引官方文档：“the official docs explicitly state that status lines "[Run] locally and [do] not consume API tokens."”，并怀疑“fix”是否启用了 TFA 提到的 `short` 模式。
- rptx（id 50285327）：“in the current binary, both the sdk option and the env var can only switch them on”。
- gym（id 50284567）：Pro 套餐两天前后同样用满 5 小时窗口折合周限额从 25% 降到约 13%，“so something was fixed”（个人体感，无方法）。

## 四、扫描说法勘误

| 扫描说法（ChatGPT／Claude 初核） | 核对结果 |
|---|---|
| “agent_summary 约占估算成本 16.4%、请求 19.7%” | 与 Gist 一致。补充：这是单家公司（Flamerix）一周的 `cost_usd` 估算，样本规模未给；HN 帖正文又写“our token spend” |
| “成本为 `cost_usd` 挂牌价估算” | 部分不符：Gist 写的是“Claude Code's own per-request estimate (`cost_usd`), not an invoice”，没有写“挂牌价”；“API list prices”是员工对自己 2% 的口径 |
| “附非官方 Bun preload 补丁，作者未测 Mac/Windows” | 一致：“Linux tested, macOS and Windows untested”；Windows 上“most likely never installs the hook” |
| “edwinarbus … 北京 10-29 12:52 … 2%／不到 4%／6%” | 一致（id 50284699，04:52:07Z）；“last week, across Claude Code users who send us telemetry”；6% 的分母仅终端 |
| “修复在 2.1.311+、只针对最贵的缓存未命中” | 员工原话属实；但 CHANGELOG 2.1.311／2.1.312 均无对应条目，已发布版本 CHANGELOG 里找不到该修复 |
| “GitHub issue #86879 有另一组占比” | 属实，且不止一组：22.7% 请求／18.0% 加权 token／22.2% 原始 token（作者 rflcrz），15–22%（kcarriedo），26% 缓存读 token（sylvain-artois），0.7–1.08×（rabitlord1324）；口径各异，全是 subagent 密集会话。issue 开于 2026-08-15，早于 Gist 两个多月，7 条评论中无官方回复，状态 open |
| “官方 statusline 文档说明状态栏是本地脚本、不调 API” | 文档原句属实，但该文档说的是用户自配的 `statusLine`，不是 agent 面板摘要（另有 `subagentStatusLine`）；不能据此说摘要“免费” |
| “HN 约 413 分／153 评论” | 取证时 424 分／155 评（Algolia，10-29 19:48 北京） |
| Claude 初核：“HN 北京 10-29 11:33 提交” | 一致（03:33:42Z） |
| 线索：Gist 描述“Bun preload that turns agent_summary off” | 一致；另注意 Gist 创建时刻（10-26）早于历史最早修订（10-28）的矛盾，见 C1-4 |

## 五、未完成／未找到

- Anthropic 官方页面署名 edwinarbus：未找到；edwinarb.us 取不到。
- Reddit 讨论：未取到（搜索被重定向登录页）。
- Gist 样本规模、测量周的具体日期：原文未给。
- 2.1.311 对应修复条目、`agent_summary` 在官方文档中的描述：未找到。
- 国内中文转述只找到一家（禁闻网，风控不取）；36氪、IT之家等未检索到本事件。
