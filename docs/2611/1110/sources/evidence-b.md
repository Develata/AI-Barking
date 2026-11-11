# 1110 期 B 组取证：Opus 5.5“11月3日变笨”争议

执行：Sonnet 5.5（`claude-sonnet-5-5`）。取证时刻 2026-11-11 14:30–14:55（北京时间，本机时区 UTC+8）。存档文件前缀 `b-`，位于 `docs/2611/1110/sources/`；抓取与截图日志见 `capture-log-b.md`。图片在 `docs/2611/1110/images/`：本组区间 13–24 用尽（其中 20、21、23 是被取代的废图，见文末“待删除”），溢出的图用 37–45，编号区间之外，由 Claude 在渲染时按需重编。

事实口径提醒：下面所有“已找到”只表示“该文字/数字确在该一手来源上”，不表示 Melaugh 的分析或 Theo 的分析本身正确。两人的数据都是自述、原始日志均未公开（L6）。

## 一、逐条取证表

时间一律北京时间，括号内保留原时区（UTC）。

### B1 Melaugh 原帖与整个 thread

作者：Shane Melaugh（X 个人简介：“just trying things. building @SashimiiAI and more.”，只记公开职业身份）。thread 共 4 条，均为他本人，22:15:38–22:15:40 UTC 连发。原帖 JSON：`b-melaugh-thread.json`、`b-melaugh-main.json`、`b-melaugh-<id>.json`；三张图原图：`b-melaugh-chart1.jpg`、`b-melaugh-chart2.jpg`、`b-melaugh-chart3.jpg`（各 1122×1402，原图即此尺寸，未放大）。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1-1 原帖正文：用 Gemini 4 Argon 分析 1,400 个 Claude Code 会话 | L6（作者自述；他是第三方个人开发者，不是 Anthropic 员工） | https://x.com/shanemelaugh/status/2119192075505893695 | “is Opus 5.5 nerfed? / I just had Gemini 4 Argon analyze 1,400 Claude Code sessions and it found that on November 3, something definitely happened. / the model name didn't change, but the way it does reasoning took a sharp turn.” | `13-melaugh-card.png`（帖子卡片，X 官方嵌入渲染，只截卡片；卡片内显示“6:15 AM · Nov 8, 2026”） | 发帖 2026-11-08 06:15:38（2026-11-07 22:15:38 UTC）。fxtwitter 2026-11-11 14:3x：浏览 598,167、赞 3,157、回复 324、转 226、引用 199、书签 1,002（X 官方嵌入显示 3.1K 赞；syndication 接口 3,158，差 1 属取数时刻不同）。“1,400 个会话”的时间范围帖文没写；图 1 页脚写 Oct 9–Nov 7, 2026 | 已找到 |
| B1-2 图 1：思考块开头措辞突变（含页脚） | L6 | 同上，thread 第 1 条的图（pic.x.com/6xnofvg1rv） | 标题“Thinking patterns changed on November 3”；副标“Same model ID. Same CLI version.”；图内“Shift observed Nov 3 · ~18:00 UTC”；图例“Starts with “I'm...””“Uses “I should / I need to””“Uses — or --”；终点标注 61.3%、10.9%、6.5%；结论条“The shift appeared mid-session across 7 active sessions.”；页脚“Daily UTC averages. Nov 3 includes both sides of the change.” “Source: Claude Code session logs • Oct 9–Nov 7, 2026” “Visible summaries show a behavior shift, not proof of changed model weights.” | `14-melaugh-wording-chart.png`（原图，米白底，完整含标题、图例、页脚） | 纵轴“Daily share of visible thinking blocks”，每日一点，Oct 9–Nov 7。“I'm...”开头占比约 20–30%→61.3%；“I should / I need to”约 45–58%→10.9%；破折号约 40–55%→6.5%（图中点位目读，页面只标注终点三个数字）。**图里的 7 active sessions 是“变化出现在会话中途”的会话数，不是 1,400** | 已找到 |
| B1-3 图 2：high 档下首个工具调用前思考 token 中位数 84→54 | L6 | 同上，thread 第 2 条（status 2119192078773260496，22:15:38 UTC；浏览 54,324、赞 365） | 帖文“also, how much reasoning Opus 5.5 does in "high" took a nosedive.”；图标题““High” effort now plans less upfront”，副标“Before vs. after November 3 · effort = high”；“Turns with no thinking … 40.5% emitted zero thinking tokens … 59.5% emitted thinking tokens”；“Upfront thinking –36% … Before Nov 3 84 … After Nov 3 54 … Median thinking tokens before first tool call”；“Script & edit crashes +47% … Relative index · before Nov 3 = 100 … 147”；结论条“Less upfront thinking. More crashes in the logs.”；页脚“Compared 14,560 turns before Nov 3 with 14,871 after.” “Observational log analysis • Nov 2026” | `15-melaugh-effort-chart.png` | 口径：仅 effort=high 的回合；前 14,560 回合、后 14,871 回合；“Script & edit crashes”是相对指数（前=100），**图未定义“crash”**，未给绝对数；“40.5% 零思考 token 回合”**图上未标明这根条覆盖哪个时段（前/后/全部），也没有对照数字**。这些数是 Gemini 4 Argon 分析的产物，原始日志与代码未公开 | 已找到 |
| B1-4 thread 第 3 条：单个真实 bug，high 失败、xhigh 5 轮解决 | L6 | status 2119192081251906001（22:15:39 UTC；浏览 38,417、赞 232） | 帖文“quick test on a real bug showed Opus 5.5 high jumping into action too quickly, getting confused and being unable to find a fix after 3 full hours. / meanwhile Opus 5.5 xhigh takes a moment to think, finds a fix after a few minutes.”；图“One bug. Two effort levels.” “Same codebase · same day · audio playback bug”；“Turns used … high 160 … xhigh 5”；“Session cost … high $13.00 … xhigh $0.85”（“15.3× cheaper with xhigh”）；“Peak thinking tokens … high 1,941 … xhigh 12,837”；页脚“Single debugging comparison • Nov 7, 2026” “One case, not a controlled benchmark.” | `16-melaugh-single-bug.png` | 每档只跑 1 次；作者自己在图里写“不是受控基准”。$13.00 与 $0.85 为会话成本，价格口径未说明 | 已找到 |
| B1-5 thread 第 4 条结论：“the nerf is real” | L6 | status 2119192083361599643（22:15:40 UTC；浏览 31,209、赞 374） | “so yeah, you're not imagining things. the nerf is real. / my take is that the @theo popularized "use high for everything" no longer applies.” | 无（纯文字，存 JSON） | **与同一作者图 1 页脚“not proof of changed model weights”并存**：帖内结论比图的页脚强。该条未附数据 | 已找到 |

### B2 Theo 的反驳数据

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B2-1 Theo 帖：推理摘要器按账号灰度，能力未见退化 | L6（Theo 的 X 简介为“Full time CEO @t3dotcodes & @t3dotchat. Part time YouTuber, investor, and developer”（fxtwitter JSON 的 description 字段），帖内数据是他自己的账号，非 Anthropic 员工） | https://x.com/theo/status/2119238026136330313 | “Dug into this more. Anthropic appears to be rolling out a change to the reasoning summarizer on a per-account basis. / No sign of degraded capability for me. I think they're just trying to fix the reasoning traces (they've been utter garbage for weeks, complained about this yesterday lol)” | `17-theo-card.png`（卡片，帖内带 Melaugh 原帖的引用转帖，已裁掉引用部分；文字被嵌入截断在“Show more”，全文在 JSON） | 2026-11-08 09:18:13（2026-11-08 01:18:13 UTC）；浏览 90,371、赞 1,270、回复 147、转 29、引用 23。**“appears to be”“I think”是他的推断/意见，不是 Anthropic 的说法**；X 嵌入标明这是对 Melaugh 帖的引用转帖 | 已找到 |
| B2-2 数字：45.8 万条、369→413、1.03→1.06 | L6 | 同上，帖内图（pbs.twimg.com/media/HWkLgqdakAABDHV.png，1600×1000，深色底） | 标题“Opus 5.5's thinking summaries did change. / It rolled out account by account. Reasoning volume held steady.”；“What did not change: the work … Reasoning tokens per response (avg) 369 → 413 … Reasoning share of output tokens 46% → 49% … Output speed (tokens/s) 111 → 112 … Tool calls per response 1.03 → 1.06 … Model ID claude-opus-5-5 same … Claude Code version 2.1.280 same”；“What changed: the summary text … Starts with "I'm..." 27% → 61% … Uses "I should" or "I need to" 47% → 13% … Median summary length 325 chars → 440 chars”；页脚“Claude Code shows a summary of the thinking, not the raw thinking. So this shows the summary style changed. It is not evidence that the model got worse.” “Data: my Claude Code traffic through CLIProxy, Oct 9 to Nov 7 2026. 458k Opus 5.5 responses, thinking display "summarized". Ticks are hours with 15+ summaries.” | `18-theo-chart.png`（原图，**深色底**） | 口径：他自己的 6 个账号（Account A–F）的 Claude Code 流量，经 CLIProxy；不是“代理日志”泛称；**“45.8 万”=458k responses**。before/after 是各账号切换前后合并。**未给 effort 档位**。369→413 是“每响应平均推理 token”，与 Melaugh 的“首个工具调用前思考 token 中位数”不是同一指标、不是同一批会话，不可直接相减 | 已找到 |
| B2-3 “按账号灰度”的证据：6 个账号切换时刻各不同 | L6 | 同上图上半部 | 图内标注：“Oct 30, 00:00 UTC”（Account A）、“Oct 31, 17:00 UTC”（B）、“Nov 3, 18:00 UTC · same hour as the viral chart”（C）、“Nov 4, 05:00 UTC”（D）、“Nov 4, 21:00 UTC”（E）、Account F“no switch seen (last used Nov 3)” | 同 `18-theo-chart.png` | 6 个账号中 5 个出现新风格。**放大原图逐行看（每格=1 小时使用）**：A 行灰格连续、蓝格紧接在 10/30 00:00 标线之后（切换时刻被相邻旧风格使用所夹，可观察）；C 行在 11/3 18:00 标线左侧紧邻有少量灰格、右侧转蓝（同样被夹，可观察；**这是唯一与 Melaugh 图“Nov 3 ~18:00 UTC”同一小时的账号**）；B、D、E 行在旧风格灰格与标注时刻之间有数天无使用的空档（B：约 10/26 至 10/31 17:00；D：约 10/28 至 11/4 05:00；E：约 10/29 至 11/4 21:00），所以这三个标注时刻只是“首次在新风格下被观察到的时刻”，实际切换发生在空档内某处，**不是观察到的切换时刻**；F 行 11/1–11/3 全是灰格，“no switch seen”。因此“按账号逐个灰度”这一说法，图上能真正钉住切换时刻的只有 A 与 C，B/D/E 只给出“切换不晚于该时刻”，F 给出“截至 11/3 未切换”。账号是否同一 plan/地区未说明 | 已找到（图上内容）；“按账号灰度”是推断 |
| B2-4 Theo 此前对推理摘要的评价（背景） | L6 | https://x.com/theo/status/2118616574676570529（2026-11-06 16:08:48 北京，回复 RhysSullivan；浏览 294,892、赞 1,375）；https://x.com/theo/status/2118634626679775549（2026-11-06 17:20:31 北京；浏览 461,202、赞 5,151） | “Ignore reasoning traces from Opus 5.5. They do not represent what the model is actually thinking. I swear they're just there to fuck up distillation” ；“This is a conspiracy theory but I'm like 95% sure I'm right. Reasoning traces for Opus 5.5 read like a shitty quantized open weight model then SOMEHOW the right answer comes out in the end.” | 无 | 他自己标注后一条为“conspiracy theory”；只作背景，不作事实 | 已找到 |
| B2-5 第三方另一份自查：@argofowl 67k 条 Opus 消息 | L6 | https://x.com/argofowl/status/2119448829489565739（2026-11-08 23:15:53 北京；浏览 10,007、赞 141；`b-argofowl.json`） | “people are saying opus 5.5 got nerfed on nov 3, so i checked my own claude code logs / 67k opus messages, before vs after: / tool calls per message: 1.06 → 1.06 / failed tool calls: 1.9% → 1.8% / me getting mad at it: 15.7 → 14.2 times per 100 messages / not seeing a nerf on my account” | 无 | 单账号自述，样本 67k 条消息，effort 未说明。仅线索 | 已找到 |

### B3 “可见思考是摘要、由另一模型生成”

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B3-1 可见文字是摘要而非原始思维链 | L1 | https://platform.claude.com/docs/en/build-with-claude/thinking（存档 `b-thinking-overview.txt/.html`，取于 2026-11-11 14:3x 北京） | “You don’t always see this text, and what you see is never the raw chain of thought: the text in a thinking block is a summary of Claude’s reasoning.” ；“When display is "summarized", the thinking text you receive is a summary of Claude’s full thinking process rather than the raw chain of thought. … No display setting returns the raw chain of thought.” | `22-thinking-doc-never-raw-cot.png`、`19-thinking-doc-summarized.png` | 页面是 Thinking（取代旧 extended-thinking 页，后者现为索引页）。对所有模型适用的通则 | 已找到 |
| B3-2 摘要由另一个模型生成 | L1 | 同上 | “Summarization is processed by a different model from the one you target in your requests. The thinking model does not receive the summarized output.” | `19-thinking-doc-summarized.png`（同框，最后一条要点） | **官方页只说“另一个模型”，没有说是哪个模型，也没有写“摘要方式会变/会按账号灰度”**。**Wayback 的 CDX 索引里该页有大量快照（自 2026-09-01 起），我取了 4 份对照**：2026-09-23 08:25 UTC（`b-wb-thinking-0923.txt`，Opus 5.5 发布后一天）写“Summarization is processed by a different model from the one you target in your requests. The thinking model does not see the summarized output.”；2026-10-28 08:33 UTC（`b-wb-thinking-1028.txt`）、2026-11-02 23:20 UTC（`b-wb-thinking-1102.txt`）与 2026-11-03 21:02 UTC（`b-wb-thinking-1103.txt`，已在 18:00 UTC 之后）写的都是“…The thinking model does not receive the summarized output.”，11/2 与 11/3 两份该句逐字相同。**即“摘要由另一模型生成”这句 11/3 前就在官方页上，11/3 前后该句无改动。** `b-wb-thinking-1031.*` 是早先用 `/web/2026/` 取到的 2026-11-09 快照（20261109215502），文件名里的 1031 是我误标，不是快照日期。官方页仍没写是哪个模型、摘要方式会不会变、是否按账号灰度。旧 extended-thinking 页 9/28、10/29 快照（`b-wb-extthinking-0928/1029.txt`）是索引版 | 已找到（“另一模型”✅且 11/3 前已在页上；“摘要方式会变/灰度”官方页未写） |
| B3-3 Opus 5.5 默认 display 为 omitted；账单含全部思考 token | L1 | 同上 | “"omitted": thinking blocks are returned with an empty thinking field. … This is the default on Claude Fable 5.2, … Claude Opus 5.5, …”；“You’re charged for the full thinking tokens generated by the original request, not the summary tokens. The billed output token count does not match the count of tokens you see in the response.” | 无（文字存档） | API 层 Opus 5.5 默认不返回思考文字；要看到文字必须设 display summarized。（推断，非官方原句：）Melaugh、Theo 日志里的“可见思考块”应当是在摘要显示被打开的情况下才有文字；Theo 图注写“thinking display "summarized"”，Melaugh 帖没写 | 已找到 |
| B3-4 Claude Code 里的默认：思考块被 redacted，需 showThinkingSummaries | L1 | https://code.claude.com/docs/en/model-config（存档 `b-cc-modelconfig.txt`） | “Claude Code collapses thinking output by default. Press Ctrl+O to toggle verbose mode and see the reasoning as gray italic text. Interactive sessions on the Anthropic API receive redacted thinking blocks by default, so set showThinkingSummaries: true in settings if you want the full summaries available when you expand. You are charged for all thinking tokens generated, even when collapsed or redacted.” | `39-ccode-redacted-thinking-default.png` | 另：Melaugh 帖没说他是否开了 showThinkingSummaries；Theo 图注写“thinking display "summarized"” | 已找到 |

### B4 effort 默认值

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B4-1 API：Opus 5.5 默认 medium | L1 | https://platform.claude.com/docs/en/build-with-claude/effort（`b-effort.txt`） | “Most Claude models default to high effort, spending as many tokens as needed for excellent results; Claude Opus 5.5 and Claude Haiku 5.5 default to medium.” ；“Setting effort to the model’s default ("medium" on Claude Opus 5.5 and Claude Haiku 5.5, "high" on other models) produces exactly the same behavior as omitting the effort parameter entirely.” ；（Opus 5.5 小节）“Claude Opus 5.5 supports all five effort levels, and medium is the default (Claude Opus 5 and earlier Opus models default to high, so a request that omits effort runs one level lower than it did on Claude Opus 5).” | `24-effort-doc-default-medium.png` | Opus 5.5 小节在页面标签页里，未截 | 已找到 |
| B4-2 “自发布起如此”而非近期改的 | L1（Wayback 快照，仅表示“该页以前写了什么”） | web.archive.org 快照：`.../web/20260926060941id_/https://platform.claude.com/docs/en/build-with-claude/effort`（`b-wb-effort-0926.txt`，2026-09-26）；`.../20261031164914id_/...`（`b-wb-effort-1031.txt`，2026-10-31）；Claude Code 页 `.../20260925131436id_/https://code.claude.com/docs/en/model-config`（`b-wb-modelconfig-0925.txt`，2026-09-25）与 `20261102021339id_`（`b-wb-modelconfig-1102.txt`，2026-11-02） | 9/26 API 页：“Claude Opus 5.5 defaults to medium.”；9/25 Claude Code 页：“Opus 5.5 starts at medium unless one of the sources above sets a level for it” | 无 | Opus 5.5 发布日期 2026-09-22（官方 API 发布说明“September 22, 2026 … We’ve launched Claude Opus 5.5 (claude-opus-5-5)”，`b-api-release-notes.txt`，官方只给日期、未给时刻；Claude Code v2.1.280 的 GitHub release 为 2026-09-22 16:38 UTC = 09-23 00:38 北京，该版起 `opus` 别名指向 5.5）。**9/25–9/26 的快照已写 medium，11/2、10/31 的快照仍是 medium**：未见 11/3 前后改默认值。快照之间（如 11/3 当天）是否改过无法证明 | 已找到 |
| B4-3 Claude Code 里 Opus 5.5 默认 effort | L1 | https://code.claude.com/docs/en/model-config | “The model’s default effort: Fable 5.2, Opus 5.5, Sonnet 5.5, and Haiku 5.5 default to medium, and Opus 4.7 defaults to xhigh.”（现行页；11/02 快照：“Opus 5.5, Sonnet 5.5, and Haiku 5.5 default to medium”）；“Opus 5.5 starts at medium unless one of the sources above sets a level for it, and a top-level effortLevel in your user settings file doesn’t count for Opus 5.5.”；“Opus 5.5 defaults to medium, one level below Opus 5’s default of high. In Anthropic’s testing, Opus 5.5 at medium matches or exceeds Opus 5 at high on coding and knowledge-work evaluations. At a given level, Opus 5.5 tends to think more per turn than Opus 5.” | `37-ccode-effort-default-medium.png`、`38-ccode-opus55-medium-thinks-more.png` | Melaugh 图 2 明写 effort=high，即他显式设了 high（比默认高一档）。**用户若只在 settings.json 顶层写了 effortLevel，该键对 Opus 5.5 不生效**（9/25 快照已是此写法），这是官方页里的限制，不是对某用户的断言 | 已找到 |

### B5 Claude Code 11/1–11/5 前后的版本

时刻取两套并列（发布时刻不一致，不取舍）：GitHub release `published_at`（`b-cc-releases-api.json`，已瘦身为标签与时刻）与 npm registry `time`（curl 实时取，值抄在这里；原 JSON 在 scratchpad，未入库）。

| 版本 | GitHub release（UTC → 北京） | npm 发布（UTC → 北京） |
|---|---|---|
| 2.1.316 | 2026-10-31 01:01:35 → 10-31 09:01 | 2026-10-31 00:42:05 → 10-31 08:42 |
| 2.1.317 | 2026-11-03 01:25:46 → 11-03 09:25 | 2026-11-02 23:57:53 → 11-03 07:57 |
| 2.1.318 | 2026-11-04 00:01:03 → 11-04 08:01 | 2026-11-03 21:55:17 → 11-04 05:55 |
| 2.1.319 | 2026-11-06 17:04:01 → 11-07 01:04 | 2026-11-06 00:58:56 → 11-06 08:58 |
| 2.1.321 | 2026-11-09 19:22:30 → 11-10 03:22 | 2026-11-09 19:02:47 → 11-10 03:02 |
| 2.1.322 | 2026-11-10 02:29:43 → 11-10 10:29 | 2026-11-10 01:58:41 → 11-10 09:58 |

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B5-1 2.1.31x 各版里与 thinking / effort / summary / 默认模型相关的条目 | L1 | https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md（raw 存档 `b-cc-changelog.txt`，1.19 MB，**超过 1 MB**；changelog 无日期，时刻取上表） | 2.1.317：“Changed subagents and teammates on a different model from the main conversation to use that model's saved effort level or its default, instead of the session's effort level”；“Fixed a project's `effortLevel` setting being ignored by sessions that an app starts ahead of time and then opens in the project, such as new sessions in the desktop app's Code tab”。2.1.318：“Removed the 2.1.288 fix that kept the model's earlier thinking when resuming a conversation started on 2.1.286 or earlier; that thinking can now be dropped”；“Added an `effort` option to a mod's `$.model.fork` …”。2.1.316：“Deprecated fast mode for Opus 4.8 as of October 30, 2026 …”。2.1.321：“Added Claude Fable 5.2 … now the default Fable model”。（对应旧条目 2.1.288：“Fixed resuming a conversation started on 2.1.286 or earlier dropping the model's earlier thinking”） | 无（只用文字；GitHub release 页只写“released this last week”无日期，未截） | **2.1.316–2.1.322 的条目里没有一条写到推理摘要器、主会话里 Opus 5.5 的默认 effort 或思考显示默认值的改变；但有两条改了“子代理的 effort”：2.1.316“Added `CLAUDE_CODE_SUBAGENT_EFFORT` to set the effort level subagents run at when nothing else names one, instead of inheriting the session's”（GitHub 2026-10-31 01:01 UTC；npm 10-31 00:42 UTC），2.1.317 的“Changed subagents and teammates on a different model …”（见上，npm 11-02 23:57 UTC，约在 11/3 18:00 UTC 之前 18 小时）。这两条只涉及子代理，条目并未说明是否影响主会话；本组不判断它们是否与 11/3 的现象有关，只并列记录**。2.1.318 那条涉及“恢复会话时丢弃更早的思考”，仅涉及 2.1.286 或更早版本开始的会话。Melaugh 图写“Same CLI version”、Theo 图写 2.1.280 same，两人都称版本没变 | 已找到（“无摘要器/主会话默认 effort 条目”是读 2.1.316–2.1.322 全部条目的结果） |
| B5-2 版本时刻与“11/3 ~18:00 UTC”的位置 | L1/L3 | GitHub API https://api.github.com/repos/anthropics/claude-code/releases；https://registry.npmjs.org/@anthropic-ai/claude-code | 见上表 | 无 | 11/3 18:00 UTC 介于 2.1.317 与 2.1.318 两次发布之间（两套时刻均如此）。扫描/二手文（aitoolranked）写“2.1.318 11/3 21:55 UTC”与 npm 时刻一致，而 GitHub release 为 11/4 00:01 UTC，差约 2 小时 | 已找到 |
| B5-3 11/3 当日官方 API 发布说明 | L1 | https://platform.claude.com/docs/en/release-notes/overview（`b-api-release-notes.txt`） | “November 3, 2026 — Text generated by Claude Mythos 5, Claude Opus 4.7, and Claude Sonnet 4.6 carries Anthropic’s text watermark …”；11/2、11/5、11/9 条目也无推理摘要相关内容 | 无 | 范围：只读了该页 2026-10-30 至 11-09 的条目；这些条目均无推理摘要相关内容 | 已找到 |

### B6 Anthropic 有无回应

| 说法 | 级 | 来源 | 查到的内容 | 状态 |
|---|---|---|---|---|
| B6-1 状态页 | L1 | https://status.claude.com/api/v2/incidents.json（`b-status-incidents.json`，取于 2026-11-11 14:3x，页面更新 2026-11-11 05:04 UTC） | 11/3–11/10 事件只有两条：2026-11-08 08:28 UTC“Degraded performance for Claude Fable 5.1”；2026-11-10 21:47 UTC“Elevated errors on Fable 52 and Mythos 52”。Opus 5.5 的最近一条是 2026-10-24 00:31 UTC“Elevated errors for Claude Opus 5.5”。**无与摘要或质量相关的事件** | 未找到一手来源（未见相关事件） |
| B6-2 官方发布说明与 CHANGELOG | L1 | 见 B5-1、B5-3 | 无相关条目 | 未找到一手来源（未见相关条目） |
| B6-3 官方 X（ClaudeDevs、AnthropicAI） | L1 | WebSearch 两次（extended）、fxtwitter 无法列时间线 | 未检索到 11/3–11/11 间回应此事的官方帖。**未能直接浏览官方账号时间线（fxtwitter 不提供列表），结论限于搜索可见范围** | 未找到一手来源（未见；范围受限） |
| B6-4 HN 楼内自称 Anthropic 员工的回复 | L2 | HN 50422069 全部 510 条评论（Algolia JSON，评论树扫描；JSON 在 scratchpad，未入库） | 按“I work at Anthropic / Anthropic employee / ex-Anthropic”等句式扫描，0 命中；作者名里无 bcherny 等已知名号。HN 50419019 的 16 条评论也逐条扫了（作者 gigatexal、Bender、jasomill、bluebands、turtleyacht、faangguyindia、richstokes、retorialmadness、oleksg、kasey_junk、echion、cedws、petterroea、gorgmah、Traubenfuchs、bradknowles），无自称 Anthropic 员工者 | 未找到一手来源（未见员工回复） |
| B6-5 GitHub issue | L1 | GitHub 搜索 `repo:anthropics/claude-code thinking summary opus 5.5 created:>=2026-11-03`（`b-gh-issues-search.json`，5 条）；#106878、#108766、#108048、#106893 的评论 | 评论者 `author_association` 均为 NONE，无官方认领或回应 | 未找到一手来源（未见官方回应） |
| B6-6 先例（背景，非本次）：2026-04-06 Claude Code 团队成员在 HN 回应“thinking 深度下降”分析 | L2（署名 bcherny、自称“Boris from the Claude Code team”，个人发言） | https://news.ycombinator.com/item?id=47664442（`b-hn-boris.txt`）；Algolia 取时刻 2026-04-06T17:56:20Z（= 04-07 01:56 北京）；VentureBeat 2026-04-13 报道引述（`b-venturebeat.txt`） | “This beta header hides thinking from the UI, since most people don't look at it. It *does not* impact thinking itself … It is a UI-only change.” ；“If you are analyzing locally stored transcripts, you wouldn't see raw thinking stored when this header is set, which is likely influencing the analysis.” ；“We landed two changes in Feb that would have impacted this.”（adaptive thinking 默认 Feb 9；Opus 4.6 默认 medium effort Mar 3） | 已找到（先例，对象是 Opus 4.6，与本次 Opus 5.5 事件无关联，勿混写） |

截图：`40-hn-bcherny-redact-thinking.png`（HN 评论页，匿名页，HN 用户名 bcherny 为公开署名）。

### B7 独立追踪器

| 追踪器 | 级 | 来源 | 记录 | 状态 |
|---|---|---|---|---|
| LiveNerf | L3（第三方追踪，仅限其自身数据） | https://github.com/ninjahawk/livenerf（README 存 `b-livenerf-readme.md`；逐日数据 `b-livenerf-daily.csv`，来自 raw.githubusercontent.com results/daily.csv；README 的最后一次提交 2026-11-10 17:31:56 UTC） | README（2026-11-10，第 48/60 天）：“three full 10-day windows after launch week, no change detected in Opus 5.5 on the frozen panel (window 3: +0.028 ± 0.023 vs launch week; the pre-registered rule needs a significant shift of at least 3 points in two consecutive windows).” 窗口表：窗口 1（第 11–20 天，自 10-04）+0.007±0.018；窗口 2（第 21–30 天，自 10-14）-0.015±0.018；窗口 3（第 31–40 天，自 10-24）+0.028±0.023，均“no change”，CLI 全程 2.1.280。**第 1 天=2026-09-24，第 40 天=2026-11-02，所以“三个窗口”全部在 11/3 之前结束；覆盖 11/3 之后的窗口 4（第 41–50 天）尚未结束，没有判定**。逐日 csv（measured/all，每日 UTC 00:07 开跑，单日 78 题，单日起伏大）：第 41 天 = 2026-11-03 00:07 UTC，**早于 11/3 18:00 UTC 的疑似切换时刻**，得分 0.5641；第 42–48 天（11-04…11-10，均在 11/3 18:00 UTC 之后）得分 0.6282、0.5、0.6282、0.5769、0.6538、0.6154、0.5769；每日 `output_tokens_median`（单轮输出 token 中位数）第 41–48 天：562、543、627、541.5、558、619、577.5、561（窗口 3 的中位数 590）。窗口 4 未结束，**LiveNerf 没有给 11/3 之后的判定，我也不另算均值下结论**。LiveNerf 的 effort 档位：README 只写“Effort is always passed explicitly, never left at the default.”；其计划文档 PLAN.md 的参数表写“`output_config.effort` | always explicit (primary arm: `high`)”（https://github.com/ninjahawk/livenerf/blob/main/PLAN.md，计划文档，实际运行档位以其 harness 为准，我未核）；Opus 5 为 control arm。方法限制（README 原文）：“Every call is hermetic: a short frozen system prompt, no tools, no MCP servers, no settings or hooks, no `CLAUDE.md` or memory, one turn”；设计灵敏度“7.5 points per 10-day window”（实测 MDE 6.6 points）。即：**它测单轮无工具的题目准确率，不测多步 agent 的先规划后行动，也检不出小于约 6–7 个百分点的变化** | 已找到（无 11/3 后判定；口径：单轮无工具题，Claude Code 订阅，effort 见上） |
| MarginLab Claude Code 追踪 | L3 | https://marginlab.ai/trackers/claude-code/（`b-marginlab.txt`、`b-marginlab-hist.txt`） | 页面写“Last updated: Sep 29, 2026”；“We are collecting a new Opus 5.5/high baseline from runs beginning September 24, 2026. Degradation detection is paused.” **页面内容停在 9/29，无 11/1–11/10 数据**（HN 评论 50435319 也说“hasn't been updated since September”） | 部分支持（页面存在，但内容停在 9/29，无 11/1–11/10 数据） |
| aistupidlevel.info | L3 | https://aistupidlevel.info/（`b-aistupid.txt`） | 首页“Top 5 · Last 30 days”（12 Oct – 11 Nov）Coding：“1 Claude Opus 5.5 Score 97”。只给 30 天汇总；逐日分榜需登录/付费 | 部分支持（只有 30 天汇总，无 11/3 前后分段） |
| BridgeBench Nerf Bench | L3 | https://www.bridgebench.ai/nerf-bench/claude-opus-5-5 | `barking fetch` 返回 HTTP 403，Cloudflare 质询，未取得；不绕过。仅有 HN 评论 50429334（nonethewiser）称该站“is showing its not degraded”，未核 | 未找到一手来源（打不开） |
| r/claude 帖 1xql02s “Is Opus 5.5 nerfed?” | L6 | https://www.reddit.com/r/claude/comments/1xql02s/is_opus_55_nerfed/ | `barking fetch` 返回 8 KB 空壳，blocked；`barking heat` 的 Reddit 请求重定向到登录页，不登录不重试 | 未找到一手来源（打不开） |

截图：`43-livenerf-latest.png`（README “Latest” 段）、`45-livenerf-results-table.png`（结果表；页面宽 1500 CSS px，图宽 3000 px，略超 1400 CSS px 上限，原因是表格窄屏会折行成多行难读）。

### B8 热度与社区

| 项 | 级 | 来源 | 数据 | 状态 |
|---|---|---|---|---|
| HN 50422069 | L3/L6 | https://news.ycombinator.com/item?id=50422069（`barking heat`，`b-heat.json`、`b-hn-50422069.json`；取于 2026-11-11 14:33 北京） | 816 分、511 评（含嵌套），提交者 rzk，2026-11-09 15:04:25 北京（07:04:25 UTC）。链接指向 Melaugh 原帖 | 已找到 |
| HN 50419019 | L6 | https://news.ycombinator.com/item?id=50419019 | “Tell HN: A flood of reports of Opus 5.5 being nerfed”，29 分、16 评，2026-11-09 07:49:19 北京（11-08 23:49 UTC），提交者 b--l；帖文列出 14 个 r/ClaudeCode 帖链接（“These are all within the last week.”），**我没能打开这 14 个帖（Reddit 被拦），只能说“该帖列出 14 个链接”，不能记为“一周内同题帖 14 个”已核**。同帖评论 gorgmah：“funnily livenerf is showing the model to be slightly more intelligent these days”（评论时刻 11/09，评分 HN 不显示） | 已找到（r/ClaudeCode 一周内同题帖数：未找到一手来源） |
| 同题的另两条 HN 提交（`bh-heat.json`） | L6 | HN 50411261（同一条推文，10 分 1 评，2026-11-08 15:56 北京）；50410747 “Reddit User Detects Decrease in Thinking Blocks for Opus 5.5”（6 分 1 评，11-08 13:47 北京，指向 r/claude 1xql02s） | 数字如左；**同一推文有至少两次 HN 提交**，主帖为 50422069 | 已找到 |
| 评论评分 | — | HN 不公开评论得分 | `barking heat` 与页面均无评论得分；下面“可能的吠点”里的评论只能记 HN 评论链接、作者、时刻与所处楼层（页面排序靠前=得分较高的粗略指示），**不记分数** | 未找到一手来源（平台不提供评论得分） |
| 中文/日韩侧报道 | L4/L5 | 见“夸大说法实例”；`b-gigazine.txt`、`b-digitaltoday.txt`、`b-sina.txt`、`b-aitoolranked.txt`、`b-aicoder.txt` | 取到 GIGAZINE（日）、DigitalToday（韩，英文版）、新浪科技（中，转自微信号“AI信息Gap”）、AIToolRanked、AICoder 五篇；mortaf3（中文日报站）请求失败（连接失败，已重试一次，不再试），linux.do 403 Cloudflare，llms.blog 协议错误 | 部分支持（中文侧针对 Melaugh 帖本身的报道未取得存档，见第三节） |

## 二、可能的吠点（原文里有、线索摘要没提或说反的限制；L6 线索进正文前须回原文核）

1. Melaugh 图 1 页脚自己写“Visible summaries show a behavior shift, not proof of changed model weights.”，同一帖的结论却是“the nerf is real”。出处：`14-melaugh-wording-chart.png` 与 thread 第 4 条。
2. 图 1 的“I'm… 61.3%”等是**可见思考摘要的措辞**，不是模型输出质量；官方文档说可见文字“never the raw chain of thought”，摘要由“a different model”生成（B3-1、B3-2）。
3. 图 1 的“7 active sessions”：变化“出现在会话中途”的会话只有 7 个；“1,400 个会话”是整体样本。图 2 的 14,560 与 14,871 个回合是仅 effort=high 的回合。
4. 图 2 的“40.5% turns emitted zero thinking tokens”时段未注明、无对照数；官方文档称 Opus 5.5 为自适应思考，“the model decides per step how much to think based on the effort level”（`b-cc-modelconfig.txt`，Extended thinking 一节），所以“某些回合零思考”本身不是异常，要看前后占比。另：官方 Claude Code 页写交互会话默认收到 redacted 思考块，而 GitHub 上有用户 issue 称会话 JSONL 里的思考文本有时为空只剩签名（如 https://github.com/anthropics/claude-code/issues/31143 “Persist summarized thinking text to session JSONL”、https://claudeissues.com/issue/86865，均为用户报告，L6，未核）；Melaugh 是否开了 showThinkingSummaries、“零思考 token”是怎么从日志里数出来的，帖里没写。
5. 图 2 的“Script & edit crashes +47%”是指数（前=100），未定义什么算 crash，也无绝对数。
6. 图 3 是单次对比，作者自己写“One case, not a controlled benchmark.”；high 与 xhigh 各跑 1 次，且作者说 high 在这个 bug 上“jumping into action too quickly”，xhigh 5 轮解决。
7. 方法是让另一个 LLM（Gemini 4 Argon）分析自己的会话日志，**原始日志、提示词与代码未公开**（thread 4 条均无仓库或数据链接；2026-11-11 查）。HN 评论 Topfi（50424186）也指这类分析“Sounds like analysis fully driven by a model”。
8. Opus 5.5 的默认 effort 是 medium（B4）。Melaugh 只有图 2 与图 3 标明 effort（图 2 “effort = high”，图 3 high 与 xhigh 对比）；图 1 没写 effort 档位，Theo 图也没写。官方 Claude Code 页写“a top-level effortLevel in your user settings file doesn’t count for Opus 5.5”，只在 settings.json 顶层写 effortLevel 的人会落回 medium（官方页陈述，不针对任何具体用户）。
9. Theo 图：6 个账号中只有 Account C 的切换时刻与 Melaugh 图“Nov 3 ~18:00 UTC”同一小时，其余在 10/30–11/4 各不相同，F 无切换记录；“按账号灰度”是他的推断（“appears to be”），且账号都是他本人的。
10. Theo 图的“369→413”是“每响应平均推理 token”，Melaugh 的“84→54”是“首个工具调用前思考 token 中位数”，两个指标、两批流量，不能说“互相矛盾”。
11. LiveNerf（独立追踪）的三个“no change”窗口都在 11/3 之前结束；设计灵敏度约 6.6–7.5 个百分点，测的是单轮无工具题（B7）。二手文（新浪科技）写“到 11 月 3 日，三次都判定为没变化”，容易被读成“覆盖了 11/3 之后”。
12. Claude Code 版本：11/3 前后 CHANGELOG 无推理摘要条目、无主会话默认 effort 的条目；有子代理 effort 继承的改动（2.1.316、2.1.317，B5-1），早于 11/3 18:00 UTC 约 18 小时以上；两位作者都称 CLI 版本没变（Melaugh 图“Same CLI version”，Theo 图“2.1.280 same”）。
13. 官方文档对“不看原始思考”的态度早有先例：2026-04-06 Claude Code 团队成员 bcherny 在 HN 称“redact-thinking”是“UI-only change”，并提醒“If you are analyzing locally stored transcripts, you wouldn't see raw thinking stored when this header is set”（L2，Opus 4.6 时期，不是本事件）。
14. 社区侧有人复测，结论相反或无变化（线索，L6，HN 不显示评论得分，下列均为 2026-11-09 UTC 评论）：
   - ariwilson https://news.ycombinator.com/item?id=50430601（19:34 UTC）：“I had a baseline evaluation from before Nov 3, so I re-ran it this morning. Eval context: 30 bug fixes and features evaluated by tests the agent didn't see. Ran on Opus 5.5 at medium effort in Claude Code. 20 of the tasks were last run on Nov 1, twice each. Nov 1 solved 36/40, and today solved 19/20. Thinking tokens per task were 0.95x Nov 1’s and output tokens 0.97x. The one significant change was speed: median time to first token decreased from 3.0s to 1.7s. Caveats: one run per task today, medium effort only.”（自述，样本小，仅 medium）
   - ianberdin https://news.ycombinator.com/item?id=50432936（22:06 UTC，自述为 Playcode 运营者，AI 应用构建商）：同一套 9 月基准重跑，xhigh 9 月 86,096 token/$1.72、今天 107,333/$2.15；“17 identical calls at high ranged from 17,757 to 59,705 tokens”，称在噪声内（`b-hn-playcode.txt` 为其博文存档；自述）。
   - orbital-decay https://news.ycombinator.com/item?id=50424053（11:16 UTC）：“This is not real model's reasoning chain, they might have just replaced or tuned the model that summarizes it. If you want to track actual degradation you need to run evals.”
   - sunaookami https://news.ycombinator.com/item?id=50422423（07:51 UTC）：“It's just a new summarizer, the model and performance itself is unchanged.”（无证据，只一句）
   - jonathanlydall https://news.ycombinator.com/item?id=50423058（09:10 UTC）：“Anecdotal, but I tend to get by with medium thinking on Opus … it really did feel like it had become noticeably slower to think about things. I wouldn't say I've especially noticed its effectiveness having been reduced”
15. 支持“变笨”一侧的评论多为体感（L6）：如 ricardobeat 50423006“fumbling basic tool calls and failing to finish tasks even at high”；benwills 50422643“even after bumping to a default of Extra High reasoning”；vadansky 50429942“Maybe placebo, maybe bad timing”。无对照数据。

## 三、夸大说法实例（把事件写歪的标题或文章）

找到的实例，原句逐字；其余未找到则写明。**中文侧针对 Melaugh 帖本身的报道未取得存档**：新浪科技一篇早于该帖（11/5），mortaf3（中文日报站，标题“Opus 5.5 被降智？有人查出 11/3 思考突变！｜…”）请求失败，linux.do（“opus-5.5最近是不是降智了？”“悲！ claude 5x max用 opus5.5 被降智了咋整…”）403，nodeseek（“【claude】有没有发现最近 claude 好像降智了”）未打开——这几条只见搜索结果标题，均未存档、未取证；另做的一次中文定向检索（“Shane Melaugh … 1400个会话 … 思考摘要”）未检索到中文报道。

1. 原作者自己的帖：Shane Melaugh，X，2026-11-08 06:15:40（11-07 22:15:40 UTC），https://x.com/shanemelaugh/status/2119192083361599643：“so yeah, you're not imagining things. the nerf is real.” ——与他自己图 1 页脚“not proof of changed model weights”不一致；这是全链条里最强的越界句，后续媒体多从它而来。
2. GIGAZINE（日本科技媒体），2026-11-10 16:55 JST（= 15:55 北京），https://gigazine.net/news/20261110-claude-opus-nerf/ ：标题“Claude Opus 5.5の推論能力が11月3日から急激に低下したとの指摘、1400件の利用履歴から変化が判明”。标题把“可见思考摘要措辞变化”写成“推論能力が…急激に低下”，用“指摘”（被指出）留了余地；正文后半段补了“思考内容は…別のAIモデルが作成した要約”与 Theo 的反驳，正文比标题谨慎。
3. YouTube 频道 Wisely chen，标题“不是你的錯覺，Opus 5.5 真的變弱：訂閱制 AI 的 nerf 宿命”（oEmbed 取得标题与频道名：`b-youtube-oembed.json`；视频未打开，发布时间未取得，YouTube 不入本号文字）。“真的變弱”是对“模型变弱”的断言；未核视频内容是否带限定。
4. 新浪科技转载（来源标注“AI信息Gap”/市场资讯），2026-11-05 06:10 北京，https://finance.sina.com.cn/tech/roll/2026-11-05/doc-inixrqea8958494.shtml ：标题“Claude、GPT 集体降智？我扒出 7 个隐藏原因，附抢救教程”；正文“Claude Opus 5.5、GPT-6 Astra、GPT-6.1 Sol 最近疯狂降智，可第三方跑分却显示，成绩基本没有下滑。”“很多「降智」，问题出在被偷偷换了模型、推理强度暗降、对话拖拖拉拉……”。**发布早于 Melaugh 帖（11/7）**，讲的是更早一波体感投诉；它把“被偷偷换了模型、推理强度暗降”列为原因之一，没有证据支持“偷偷换模型”，但整体并未断言事实，且引了 LiveNerf“到 11 月 3 日，三次都判定为「没变化」”（见吠点 11，该说法时间范围有歧义）。
5. DigitalToday（韩国，英文版），2026-11-11 13:07 KST（= 12:07 北京），https://www.digitaltoday.co.kr/en/view/124089/claude-opus-55-suspected-drop-in-reasoning-performance ：标题“Claude Opus 5.5 suspected of drop in reasoning performance”，措辞为“suspected”，正文写了摘要器反驳与“it has not been confirmed whether Anthropic actually reduced the model's performance”，较谨慎，不算夸大；列作对照。
6. AIToolRanked（二手博客），2026-11-09，https://aitoolranked.com/news/claude-opus-5-5-nerf-november-3-thinking-summaries ：“Theo's per-account switch times line up with Melaugh's shift down to the hour”——Theo 图 6 个账号里只有 Account C 一个与 Melaugh 同一小时，且只有 A、C 的切换时刻被相邻旧风格使用所夹（B、D、E 前有数天空档，标注的是首次在新风格下被看到的时刻）；该文结论“the 'nerf' is not proven, and the summary change is close to proven”是该站观点，不是官方确认。
7. 把摘要措辞变化直接写成“模型被换/降配”的标题：本轮检索（WebSearch 中文/英文各 1–2 次、搜索结果页标题浏览）**未找到**明确写“模型被换”或“降配”的中文标题；中文论坛帖（linux.do、nodeseek 帖标题“opus-5.5最近是不是降智了？”“有没有发现最近 claude 好像降智了”）403 或未打开，只见到标题，未取证。

## 四、扫描说法勘误

1. “Theo 用 45.8 万条自己的代理日志”——图注是“my Claude Code traffic through CLIProxy … 458k Opus 5.5 responses, thinking display "summarized"”，6 个账号的 Claude Code 流量，单位是“响应”（responses），不是“日志条目”；未给 effort 档位。
2. “369→413、1.03→1.06”——对；但分别是“Reasoning tokens per response (avg)”与“Tool calls per response”，另有“Reasoning share of output tokens 46%→49%”“Output speed 111→112”“Median summary length 325→440 chars”，扫描没提后三项。
3. “出处 URL 未给”——已找到：https://x.com/theo/status/2119238026136330313（2026-11-08 09:18:13 北京），是对 Melaugh 原帖的引用转帖。
4. “是推理摘要器按账号灰度”——这是 Theo 的推断（“appears to be rolling out”），数据支持的是“他的 6 个账号中 5 个先后出现新摘要风格，时刻跨 10/30–11/4”；Anthropic 未确认。
5. Melaugh 帖：扫描写“图表页脚写‘可见摘要出现行为变化，不等于模型权重变化’”——原文是“Visible summaries show a behavior shift, not proof of changed model weights.”，意为“不是（模型权重已变的）证明”；扫描漏了：thread 还有 3 条（图 2、图 3、结论“the nerf is real”）、图 1 的“7 active sessions”、图 3“One case, not a controlled benchmark”。
6. 扫描写“首个工具调用前的中位思考 token 84→54”——对，口径是 effort=high、14,560 回合 vs 14,871 回合；图里还有“40.5% 零思考回合”（无前期对照）与“Script & edit crashes +47%”（指数）。
7. “官方 effort 文档：Opus 5.5 默认 medium”——对（逐字见 B4-1）；扫描问“自发布起如此还是近期改的”——**自发布起如此**：9/25（Claude Code 页）、9/26（API 页）的 Wayback 快照已写，10/31、11/2 的快照仍是；Claude Code 里同为 medium（B4-3）。
8. “官方 extended thinking 文档称可见思考是摘要、由另一模型生成”——页面现为 https://platform.claude.com/docs/en/build-with-claude/thinking（extended-thinking 是索引页）；“summary”“a different model”两句都在；不过**官方页没写摘要方式会变，也没写是哪个模型**。
9. “Claude Code 2.1.317/2.1.318 发布说明与此相关，是第三方转述”——查 CHANGELOG 原文，这两版没有推理摘要条目，也没有主会话里 Opus 5.5 默认 effort 的条目；2.1.317 有“子代理沿用其所属模型自己的 effort 而非会话 effort”的改动（B5-1）；转述里的“2.1.318 11/3 21:55 UTC”是 npm 发布时刻，GitHub release 为 11/4 00:01 UTC。
10. “Anthropic 至今未公开回应”——本组检索范围内（状态页、API 发布说明、CHANGELOG、GitHub issue、HN 评论、搜索）未见；官方 X 时间线无法直接浏览（B6-3）。
11. LiveNerf 在二手文中被写成“到 11 月 3 日三次判定没变化”——README 窗口 1–3 的时间是 9/24–11/2，不含 11/3 之后；11/3 之后的窗口尚无判定。
12. “HN 50422069 816/511、50419019 29”——对。

## 五、可入配图候选对照表（B 组 13–24 区间 + 溢出）

| 文件 | 内容 | 来源 | 备注 |
|---|---|---|---|
| `13-melaugh-card.png` | Melaugh 帖子卡片（含图 1） | X 官方嵌入，headless Chrome | 卡片含作者公开头像与昵称（帖子作者本人，非抓取者）；不含左侧导航与回复 |
| `14-melaugh-wording-chart.png` | 图 1 原图 | 帖内原图 JPG→PNG（`sips` 仅换容器） | 1122×1402，含页脚 |
| `15-melaugh-effort-chart.png` | 图 2 原图 | 同上 | 含“14,560 vs 14,871”页脚 |
| `16-melaugh-single-bug.png` | 图 3 原图 | 同上 | 含“One case, not a controlled benchmark.” |
| `17-theo-card.png` | Theo 帖子卡片（含他的图） | X 官方嵌入 | 深色底图；已裁掉被引用的 Melaugh 原帖 |
| `18-theo-chart.png` | Theo 图原图 | 帖内原图 PNG | 1600×1000，深色底 |
| `19-thinking-doc-summarized.png` | “Summarized thinking”小节（含 a different model） | 官方 Thinking 页 | 2 倍像素，1000 CSS px 宽 |
| `22-thinking-doc-never-raw-cot.png` | “never the raw chain of thought”段 | 同上 | |
| `24-effort-doc-default-medium.png` | “default to medium”段与提示框 | 官方 effort 页 | |
| `37-ccode-effort-default-medium.png` | Claude Code 默认 effort 条目与“starts at medium”段 | Claude Code model-config 页 | 溢出编号 |
| `38-ccode-opus55-medium-thinks-more.png` | “Opus 5.5 defaults to medium … think more per turn” | 同上 | 溢出编号 |
| `39-ccode-redacted-thinking-default.png` | “redacted thinking blocks by default” | 同上 | 溢出编号 |
| `40-hn-bcherny-redact-thinking.png` | 2026-04 bcherny HN 评论 | HN | 先例，非本次 |
| `41-hn-nerf-thread-header.png` | HN 816 分 511 评 标题条 | HN | 取于 2026-11-11 14:3x |
| `43-livenerf-latest.png` | LiveNerf README “Latest” 段 | GitHub | |
| `45-livenerf-results-table.png` | LiveNerf 窗口结果表 | GitHub | 页宽 1500 CSS px |

CHANGELOG 条目与 GitHub release 页没有截图：条目是否相关结论为“没有”，且 changelog 无日期，GitHub release 页只显示“released this last week”；时刻放在 B5 表里。

## 六、待删除文件（精确路径）

- `docs/2611/1110/images/20-thinking-doc-you-dont-always-see.png`（截断，被 22 取代）
- `docs/2611/1110/images/21-thinking-doc-never-raw-cot.png`（下缘带出下一行文字残影，被 22 取代）
- `docs/2611/1110/images/23-effort-doc-default-medium.png`（上缘带出标题残影、下缘截断，被 24 取代）
- `docs/2611/1110/images/42-livenerf-latest.png`（截断，被 43 取代）
- `docs/2611/1110/images/44-livenerf-results-table.png`（窄屏折行难读，被 45 取代）
- `docs/2611/1110/sources/b-wb-thinking-1031.html`、`.headers`、`.txt` 可保留（文件名里的 1031 是请求日期，实际快照 2026-11-09）；无需删除
