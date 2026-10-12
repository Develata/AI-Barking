# 1012 期 B 组取证清单：GPT-6.1 Sol 在 Codex / Work 的默认上下文 272k → 500k；订阅用量不收长上下文费

取证时间约 2026-10-12 北京 13:05–13:55（UTC 05:05–05:55；本机 EDT 01:05–01:55）。时间一律写北京时间（UTC+8），括号保留原时区。执行方模型 claude-sonnet-5-5。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。逐字摘句用原语言原文，英文摘句均已在存档文件里 grep 核过（文件名见“存档”列或备注）。帖子热度与页面内容均为取数时刻值，会变。

私人信息：Tibo（Thibault Sottiaux）为 OpenAI 员工，只记其公开发言与公开职务；其 X 简介为 “Codex & ChatGPT @OpenAI”，本组未找到他“Product and Platform Lead”等头衔的一手出处。Reddit 用户名、X 回复者仅作评论出处，不收集个人信息。

## 一、核心结论（供 Claude 判断吠点）

1. **“默认值 272k→500k”与“订阅用量不收长上下文费”都只有 Tibo 个人两条帖（L2）这一个一手来源。** OpenAI 官方账号（OpenAI、OpenAIDevs）未见相关帖；Codex changelog（learn.chatgpt.com/docs/changelog，最新条目 10/8）无条目；Codex 定价页、模型页、API 模型页、help.openai.com 两份费率卡均无“500k”，也没有“订阅用量不收长上下文费”的条文。唯一的“官方页面”提及是 OpenAI Developer Community 的汇总帖，但那是 OpenAI 员工搭的自动更新表，第 7 行是对 Tibo 帖的自动摘要，不是 Tibo 原话。
2. **“不收长上下文费”限定在“订阅额度”（when drawing usage from the subscription）。** API 模型页同日未变，仍写 >272K 输入整个请求 2 倍输入价/缓存价、1.5 倍输出价；help.openai.com《Enterprise token-based pricing》费率卡（页面自标 “Updated: yesterday”）仍列 “Long context >272K input tokens: Input 2x, Cached input 2x, Output 1.5x”，只写了 “GPT-6 Astra usage in Codex does not incur additional long-context multipliers above 272K input tokens” 一处例外；该卡与 credit-based 费率卡都未写订阅/积分计费下 GPT-6.1 Sol 的长上下文规则。**积分（credits）计费、Enterprise 按 token 计费下是否仍收，本组未找到官方说明。**
3. **“取消”是否是对 6.1 Sol 的新变化，一手材料互相不齐：** Tibo 7/12、7/13 对 GPT-5.6 Sol 说过“We do not charge extra above 270k context”“we don't charge for longer context on the subscription for GPT 5.6 Sol as we control all the settings”；对 6.1 Sol 此前是否收，本组未找到官方陈述（Reddit 评论互相矛盾：一人称“用量会按 2.5 倍扣”，另一人贴 Tibo 对 5.6 Sol 的旧话反驳，均为线索 L6）。
4. **默认值不是新能力：** Tibo 8/16–17 已发帖教用户在 `~/.codex/config.toml` 设 `model_context_window = 1000000`，且称对 ChatGPT 账号可用；本次改的是默认。Tibo 10/10（北京）还回帖称“Why do you think we picked 300k for context length for Codex. We ran the numbers and it’s better. We can run it at 1M too but it would just cost more of the usage. Good defaults are important.”，两天后默认改为 500k（用户在 Reddit/X 大量引用）。
5. 数字并列：Tibo “maximum context length of GPT-6.1 Sol of 1M”；API 模型页 “1,050,000 context window”“128,000 max output tokens”；openai/codex 仓库内置模型目录 `gpt-6.1-sol` 的 `context_window` 272000、`max_context_window` 872000（仓库内置兜底，服务器端目录可能不同，且这是改动前的值）。三者口径不同，未自行取舍。

## 二、逐条核验

### B1 两条原帖与“Day 7”系列

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1-1 Tibo 主帖：Codex & Work 里 GPT-6.1 Sol 默认上下文 272k→500k | L2 | https://x.com/thsottiaux/status/2109430343346581527 | “Day 7/\n\nWe have increased default context length for GPT-6.1 Sol to 500k when used through Codex & Work, up from 272k or an ~ 80% increase.” ；“We will go higher in the future, but it helps reduce occurrence of compactions and with tasks that require keeping a lot of context and nuance over periods of time. It was carefully tested and is the new sweet spot for most of our users. You can still override it if you want to be higher our lower, with maximum context length of GPT-6.1 Sol of 1M.” | 13（嵌入卡片，被截断）、24（全文，登录会话渲染） | fxtwitter JSON：created_at “Sun Oct 11 23:45:59 +0000 2026”（= 北京 10/12 07:45:59；嵌入卡片显示 7:45 PM · Oct 11 美东）；`is_note_tweet: true`。热度见 B7。“~80%”：500/272 − 1 ≈ 83.8%（推算）。帖文写 “Codex & Work”，非 “ChatGPT Work”。存档 `b-fx-2109430343346581527.json` | 已找到 |
| B1-2 自回复：订阅用量不收长上下文费 | L2 | https://x.com/thsottiaux/status/2109430894604026039 | “With this change, we are not charging long context fees for GPT-6.1 Sol when drawing usage from the subscription.” | 14 | created_at “Sun Oct 11 23:48:11 +0000 2026”（= 北京 10/12 07:48:11；嵌入卡片 7:48 PM · Oct 11 美东）；`replying_to_status` 为主帖。该句未说：API、积分/按量计费、Enterprise 按 token 计费是否同样不收；“this change”指代主帖 | 已找到 |
| B1-3 “Day 7/” 属 28 天连发系列；Day 1–6 各是什么 | L2 | 系列预告 https://x.com/thsottiaux/status/2106845241357824205 ；OpenAI Developer Community 汇总帖 https://community.openai.com/t/28-days-of-shipping-at-openai/1403897 | 预告帖：“Over the next 28 days, each day we’ll either ship one thing that is a clear improvement and relevant for most codex/work users or ship a full reset. Let the improvements begin.” | 21（汇总帖表格 Day 1–7） | 预告帖 10/4 20:33:43Z（= 北京 10/5 04:33:43）。下列为各日帖（fxtwitter created_at → 北京时间）。Day 1 https://x.com/thsottiaux/status/2107158998495748264 10/5 17:20:29Z（10/6 01:20）：“We have optimized the default speed to be ~50% faster across GPT-6 Astra and GPT-6.1 Sol through the subscription …”；Day 2 共 4 帖：2.1 …/2107368734981517634 10/6 07:13:54Z（10/6 15:13）Auto-review 免费；2.2 …/2107548725359104441 10/6 19:09:07Z（10/7 03:09）API 门槛简化；2.3 …/2107573405553938664 10/6 20:47:11Z（10/7 04:47）Meetings；2.4 …/2107574912349303197 10/6 20:53:10Z（10/7 04:53）Decisions API；Day 3 …/2107913674593644711 10/7 19:19:17Z（10/8 03:19）“The big one is GPT-6 in Chat … new high of 40M active users … Loading a banked reset in everyone's paid accounts.”；Day 4 …/2108275041276420573 10/8 19:15:14Z（10/9 03:15）instant steering + GPT-6.1 Sol Ultrafast；Day 5 …/2108645667451318747 10/9 19:47:58Z（10/10 03:47）Composer predictions，另 “Day 5 (dots edition)” …/2108646052178092403 19:49:30Z；Day 6 …/2109045220411351517 10/10 22:15:39Z（10/11 06:15）正文仅 “Day 6/ Reset”（全局重置，属额度重置，按规范不收不取证）。存档 `b-fx-<id>.json`、`b-community-28days.txt/.json` | 已找到 |
| B1-4 OpenAI Developer Community 汇总帖第 7 行 | L1 载体（官方开发者社区）/ 内容是对 L2 帖的自动转述（发帖者 sam.saffron 称在 OpenAI 搭的自动更新，其措辞不可当官方原话） | https://community.openai.com/t/28-days-of-shipping-at-openai/1403897 | “The default context length for GPT-6.1 Sol in Codex and Work increased from 272k to 500k, remains configurable up to 1M, and incurs no long-context fees when usage is drawn from a subscription.” | 21 | 帖子创建 2026-10-06T23:53:26Z（= 北京 10/7 07:53）；第 7 行加入时刻页面未给。汇总帖第 2 楼自述 “Automatically updating a Discourse post based on a X feed”，故该行是自动生成的转述，措辞（“remains configurable up to 1M”）不是 Tibo 原句 | 已找到 |

### B2 API 模型页与定价页

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B2-1 context window 与最大输出 | L1 | https://developers.openai.com/api/docs/models/gpt-6.1-sol | “1,050,000 context window”“128,000 max output tokens”“Apr 30, 2026 knowledge cutoff” | 15 | 页面不显示自身日期。Wayback 快照 2026-10-11 12:10:26 GMT（= 北京 10/11 20:10，早于 Tibo 帖）与当前（10/12 北京 13:11 取）文本逐行 diff 无差异。存档 `b-model.txt`、`b-wayback/model-20261011.txt` | 已找到 |
| B2-2 价格表 | L1 | 同上 | Input “$2.00”；Cached input “$0.10”；Cache writes “$2.50”；Output “$10.00”（Per 1M tokens）。“Cached input tokens are priced at 5% of the uncached input token rate.”“Cache writes are billed at 1.25x the uncached input token rate.” | 16 | 同上 | 已找到 |
| B2-3 >272K 长上下文计费句 | L1 | 同上 | “Prompts with more than 272K input tokens are priced at 2x input and cache rates and 1.5x output for the full request.” | 16 | API 口径；“for the full request”即超过阈值后整个请求按倍率计，不是只对超出部分。本组 10/12 取到的页面与 10/11 快照一致，**未因 Tibo 帖修改** | 已找到 |
| B2-4 Fast / Batch / Ultrafast 倍率 | L1 | 同上 | “Fast mode prices are 2x Standard. Batch and Flex prices are 50% lower than Standard.”“Ultrafast mode prices are 6x Standard.”“Regional processing adds a 10% premium where available.” | 16 | API 口径 | 已找到 |
| B2-5 API 定价页对 6.1 Sol 长上下文的写法 | L1 | https://developers.openai.com/api/docs/pricing | Flagship models（Standard）表头 “Short context / Long context”，gpt-6.1-sol 行：短上下文 $2.00 / $0.10 / $2.50 / $10.00（Input / Cached input / Cache writes / Output），长上下文 $4.00 / $0.20 / $5.00 / $15.00。Batch 与 Flex：短 $1.00 / $0.05 / $1.25 / $5.00，长 $2.00 / $0.10 / $2.50 / $7.50；Fast：短 $4.00 / $0.20 / $5.00 / $20.00，长 $8.00 / $0.40 / $10.00 / $30.00；Ultrafast：短 $12.00 / $0.60 / $15.00 / $60.00，长 $24.00 / $1.20 / $30.00 / $90.00 | 17（Standard 表） | 长上下文列恰为短上下文的 2 倍（输入/缓存）与 1.5 倍（输出），与模型页句子一致（Cache writes 长档 $5.00 = 2×）。**定价页本身未写 272K 阈值**，只在模型页脚注（存档 `b-pricing.txt` 1559–1621 行，渲染文本 `b-pricing-rendered.txt`） | 已找到 |
| B2-6 上限数字并列（不取舍） | L2 / L1 / L1 | Tibo 帖；模型页；https://github.com/openai/codex/blob/main/codex-rs/models-manager/models.json | Tibo：“maximum context length of GPT-6.1 Sol of 1M”；模型页 “1,050,000 context window”；Codex 仓库内置目录（摘录 `b-codex-models-json-excerpt.json`）`gpt-6.1-sol`：`"context_window": 272000`、`"max_context_window": 872000` | — | 仓库文件最近一次改动 commit d63a9b83（2026-10-06T01:46:33Z）；这是客户端内置兜底，服务器端模型目录可能不同（GitHub issue https://github.com/openai/codex/issues/49961 称 “The local model catalog reported a 272,000-token window with a 95% effective-window adjustment, yielding 258,400”，L6）。**未见官方说明 500k 默认值与 872,000 的关系** | 部分支持 |

### B3 Codex 订阅用量的算法、changelog、“Work”

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B3-1 Codex 官方定价页怎么算订阅用量；是否提长上下文费 | L1 | https://learn.chatgpt.com/docs/pricing （原 https://developers.openai.com/codex/pricing 308 重定向至此） | “API token prices are separate from subscription usage; don’t use them to estimate included tasks.”；“larger projects, long-running tasks, or extended sessions that require the agent to hold more context will use significantly more per message.”；“Model choice, context, reasoning, tool use, retrieval, and caching all affect usage, so prompt length alone isn’t a reliable estimate.”；表 “GPT-6.1 Sol15-16015-160”（Plus / Standard Business，本地消息数/5 小时，估计值） | 18、19 | 全页检索（`b-learn-pricing.md` 与渲染文本）**无 “long context” / “272” 字样**；不论改动前后（Wayback 2026-10-07 07:18:20 GMT 快照 `b-wayback/learn-pricing-20261007.txt` 同样无）。故“订阅里原来怎么收长上下文费”官方文档从未写过 | 未找到一手来源 |
| B3-2 积分（credits）费率 | L1 | 同上 | Token rates 表（每百万 token）：GPT-6.1 Sol “50 credits”输入、“2.5 credits”缓存输入、“250 credits”输出；“Codex credit billing has no separate cache-write charge.” | 20 | 每个模型一档费率，无长上下文档位 | 已找到 |
| B3-3 Fast / Ultrafast 对订阅用量 | L1 | 同上 | “Fast … 2.5x”“GPT-6.1 Sol Ultrafast … 8x”（Included subscription usage）；购买积分与 Enterprise 按量为 2x、6x | — | 改动无关，供对照 | 已找到 |
| B3-4 help.openai.com 费率卡：长上下文 | L1 | https://help.openai.com/en/articles/20001415-chatgpt-rate-card-enterprise-token-based-pricing | “Long context_>272K input tokens_ \| Input: 2x Standard rateCached input: 2x Standard rateOutput: 1.5x Standard rate”；“GPT-5.6 and GPT-5.5 support long context in Work and Codex. Long context is not available in the Chat tab.”；“GPT-6 Astra usage in Codex does not incur additional long-context multipliers above 272K input tokens. Codex does not charge for cache writes.” | — | 页面自标 “Updated: yesterday”（精确时刻未给）；适用 “eligible token-based Enterprise agreements”。**该页未提 6.1 Sol 在订阅下的长上下文规则，也未随 Tibo 帖更新**（至 10/12 13:34 取数）。help.openai.com 对 curl 与匿名 Chrome 均被拦截，本条取自 firecrawl_scrape（摘录存档 `b-help-rate-card-enterprise-token.txt`；整页未存，Claude 如需入证请重抓） | 部分支持 |
| B3-5 credit-based 费率卡 | L1 | https://help.openai.com/en/articles/11481834-chatgpt-rate-card-business-enterpriseedu | GPT-6.1 Sol “50 credits \| 2.5 credits \| 250 credits”；“Codex does not charge for cache writes.” | — | 页面自标 “Updated: 3 days ago”；整页**无** “long context”字样（按 firecrawl 返回逐词检索）；摘录 `b-help-rate-card-credit-based.txt` | 已找到 |
| B3-5b Plus/Pro 个人积分文章有无长上下文规则 | L1 | https://help.openai.com/en/articles/12642688-using-credits-for-flexible-usage-in-chatgpt-freegopluspro-sora | “Your plan's included usage is used first. After you hit plan limits, usage draws from your credit balance.”；“For usage rates, see the ChatGPT rate card.” | — | 页面自标 “Updated: 10小时前”（取数约北京 10/12 14:2x，即约 10/12 04:xx 前后；时刻未给）；全文检索无 “long context” / “272” / 任何 “context” 字样；存档 `b-help-credits-personal.txt`（opencli browser 读到的可见文本）。**购买积分在 Plus/Pro 下是否按长上下文倍率计，该页未写** | 未找到一手来源 |
| B3-6 Codex changelog 有无 500k 默认值条目 | L1 | https://learn.chatgpt.com/docs/changelog （原 /codex/changelog 308 至此） | 最新条目 “2026-10-08 GPT-6.1 Sol Ultrafast in Codex and ChatGPT Work”，其后 2026-10-07、2026-10-02 | — | 10/12 北京 13:11 与 13:50 两次取，均**无** 500k 条目；GitHub Releases openai/codex 最新稳定 rust-v0.162.1（2026-10-09T19:44:51Z）无关。存档 `b-codex-changelog.txt`、`b-learn-changelog-recheck.txt` | 未找到一手来源 |
| B3-7 “Work” 指哪个产品 | L1 | https://learn.chatgpt.com/docs/use-chatgpt ；https://learn.chatgpt.com/docs/pricing | “ChatGPT Work and Codex share usage. ChatGPT Work usage inside ChatGPT uses the same pricing, credits, and usage limits as Codex.” ；use-chatgpt 表：“ChatGPT Work \| Define an outcome and get a reviewable result \| Create a deck, analyze files, draft a report, build a project plan” | 19 | 官方称 “ChatGPT Work”，Tibo 帖简写 “Work”；官方文档称 GPT-6.1 Sol “is available in Work and Codex, not Chat”（pricing 页 Feature availability） | 已找到 |
| B3-8 改动前后文档差异 | L1（Wayback 仅作“旧页写了什么”） | https://web.archive.org/web/20261007071820id_/https://learn.chatgpt.com/docs/pricing | 无差异可摘 | — | 2026-10-07 快照与现版（10/12）逐行 diff：只有导航顺序差异，定价正文相同；两版都无长上下文条文。Wayback 当时有一次 “Temporarily Offline”，后恢复 | 已找到（无前后差异） |
| B3-9 Tibo 此前对订阅长上下文费的陈述（5.6 Sol） | L2 | https://x.com/thsottiaux/status/2076201049086648705 ；https://x.com/thsottiaux/status/2076543065045795309 | 7/12：“We do not charge extra above 270k context and the context threshold has been tuned for GPT 5.6 Sol to be perfect at the default limit.”；7/13：“(1) is not correct, it is not due to 2X charging after 272k context, we don't charge for longer context on the subscription for GPT 5.6 Sol as we control all the settings.”；同帖：“The benefits of higher context lengths are mostly overall speed (as you don't wait for compaction), ability to deal with humongously large input and potentially cost if the system is well tuned and you hit your cache perfectly.” | — | 7/12 07:04:38Z（北京 15:04）、7/13 05:43:41Z（北京 13:43）；针对 GPT-5.6 Sol，非 6.1 Sol。fxtwitter 存档 `b-fx-2076201049086648705.json`、`b-fx-2076543065045795309.json` | 已找到 |

### B4 更大默认值对订阅用量的影响；500k 区间准确率

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B4-1 更大默认上下文对订阅用量有无官方说明 | L1 / L2 | 定价页 FAQ（见 B3-1）；Tibo 10/10 回帖 https://x.com/thsottiaux/status/2108778939057664357 ；Tibo 7/13 帖（B3-9） | FAQ：“… extended sessions that require the agent to hold more context will use significantly more per message.”；Tibo 10/10：“Why do you think we picked 300k for context length for Codex. \n\nWe ran the numbers and it’s better. We can run it at 1M too but it would just cost more of the usage. Good defaults are important.”；Tibo 7/13：“It is caused by overall cost of cache reads going up with the size of the context being shuffled back and forth between toolcalls. The sweet spot in terms of cost is therefore not necessarily to use the maximum possible context length.” | 18 | 10/10 帖 04:37:32Z（= 北京 12:37），回复 https://x.com/rohit3a/status/2108277836285161798 （该帖谈 Claude 的 autocompact，原句 “setting /autocompact to 400k would have saved me 29% of my weekly usage without lobotomizing Claude.”）。**本次 Day 7 两条帖没有谈 500k 是否更耗订阅额度**；官方文档只有泛泛的“上下文会影响用量” | 部分支持 |
| B4-2 官方公布的 500k 区间长上下文准确率 | — | developers.openai.com/api/docs/guides/latest-model 、/guides/model-selection、模型页、0930 发布页存档、Codex 定价/模型页 | — | — | 全部检索 “long context / 272 / MRCR / needle / 1M” 无评测数据；Tibo 只说 “carefully tested”，未给数据。第三方（未核验）：OrcaRouter 文章称有开发者认为 500k 下检索质量不如 272k（L6） | 未找到一手来源（“未见”） |

### B5 与 0930 期对照（新变化？）

| 说法 | 级 | 来源 | 摘句 | 条件 | 状态 |
|---|---|---|---|---|---|
| B5-1 0930 期（`docs/2609/0930/`）发布时已公布的上下文与长上下文价格 | L1 | `docs/2609/0930/sources/a-sol61-model.md` 第 876、896 行（API 模型页存档）；`a-changelog.md` 第 861 行 | “1,050,000 context window”；“Prompts with more than 272K input tokens are priced at 2x input and cache rates and 1.5x output for the full request.”；changelog Sep 29 “Standard pricing per 1M tokens for prompts with up to 272K input tokens is $2 input, $0.10 cached input, $2.50 cache write, and $10 output.” | 0930 的 `fact-check.md`、`doc_0930_publish.txt` 检索无 “272”“上下文”；OpenAI 发布页存档 `a-release.md`、`a-sol-launch.md` 无 context 字样。**0930 期没有写 Codex 默认值**；API 侧 1,050,000 与 >272K 倍率发布时即已写明，本次 API 页未变 | 已找到 |
| B5-2 改动前 Codex 默认值确为 272k | L2 / L1（仓库） | Tibo “up from 272k”；openai/codex `models.json`（`context_window: 272000`）；GitHub issue 49961（L6） | 见 B1-1、B2-6 | 改动前默认值 272k 有三处互证；10/10 Tibo 回帖说的是 “300k” | 已找到 |
| B5-3 本次是新变化（默认值） | L2 | B1-1 | “We have increased default context length” | 无官方文档/changelog 佐证（B3-6）；仅 Tibo 一人陈述 | 部分支持 |

### B6 中英文媒体与社区转述

| 说法 | 级 | 来源 URL | 原句（逐字） | 时间 | 备注 | 状态 |
|---|---|---|---|---|---|---|
| IT之家（中文）| L4 | https://www.ithome.com/1/011/736.htm | 标题：“OpenAI 宣布 GPT-6.1 Sol 默认上下文长度从 272k 提升至 500k，增加约 80%”；“IT之家注意到，配合本次调整，当从订阅中扣除使用量时，OpenAI 将不会对 GPT-6.1 Sol 收取长上下文费用。” | 页面时间 2026/10/12 8:23:08（北京）| 限定语“当从订阅中扣除使用量时”保留；未称 API 免费。存档 `b-ithome.txt` | 已找到 |
| AIHOT（中文聚合，L5）| L5 | https://aihot.news/story/fa2e5d50-3b53-4b62-a0f1-5f2aef0240e3 ；https://aihot.news/items/zhcf6aews0gj0nijx0zfnsqxt | 事件标题：“GPT-6.1 Sol 默认上下文提至 500k 并取消订阅长上下文费”；IT之家条目的 “AI 导读”：“配合本次调整，OpenAI 不会对 GPT-6.1 Sol 收取长上下文费用。” | IT之家条 08:23；事件页“5 小时前更新” | 事件标题带“订阅”，**条目 AI 导读漏掉“订阅”限定**（见夸大实例）。热度见 B7 | 已找到 |
| Testing Catalog（X）| L6 | https://x.com/testingcatalog/status/2109431898691031316 | “OPENAI 🔥: GPT-6.1 Sol now has 500k default context length for Codex and ChatGPT Work.” ；“> OpenAI also plans to increase this default value in the future even further.” | 2026-10-11 23:52:10Z（= 北京 10/12 07:52:10），16,435 浏览 / 335 赞（13:5x）| 未提长上下文费。存档 `b-fx-2109431898691031316.json` | 已找到 |
| KuCoin Flash（转自 BlockBeats/ME AI）| L5 | https://www.kucoin.com/news/flash/openai-boosts-gpt-6-1-sol-default-context-to-500k-tokens-removes-long-context-fee-for-subscribers | 标题 “OpenAI Increases GPT-6.1 Sol Default Context to 500K Tokens and Eliminates Long Context Fee for Subscribers”；“OpenAI is no longer charging an additional fee for long context usage with GPT-6.1 Sol when using ChatGPT subscription credits.”；“OpenAI’s Product and Platform Lead, Thibault Sottiaux” | “Release Time: 2026/10/12 03:03:32”（页面未标时区）| 把“drawing usage from the subscription”写成“subscription credits”；头衔未找到一手出处。存档 `b-kucoin.txt` | 已找到 |
| Crypto Briefing（英文）| L5 | https://cryptobriefing.com/openai-gpt-6-1-sol-500k-context/ | 副标题 “The upgrade for Codex and ChatGPT Work also drops long-context surcharges for subscription users”；“OpenAI eliminated long-context surcharges for subscription users of GPT-6.1 Sol.”；“OpenAI framed the adjustment as a response to user feedback.” | “Oct. 11, 2026”（仅日期）| 限定“subscription users”保留；“response to user feedback”在 Tibo 两条原帖中**未见**。存档 `b-cryptobriefing.txt` | 已找到 |
| OrcaRouter 博客（英文，路由商）| L5 | https://www.orcarouter.ai/blog/gpt-6-1-sol-500k-default-context | 标题 “GPT-6.1 Sol's Default Context Just Went From 272K to 500K in Codex: What It Changes, and What It Doesn't”；“On an API key it is still there, on exactly the same threshold, unchanged by anything announced on 2026-10-12.”；“Thibault Sottiaux, the company's head of product and platform” | “Date Published Oct 12, 2026” | 写清范围（仅订阅、API 不变）；头衔未找到一手出处；称 OpenCode 合并改动把其 ChatGPT-OAuth 下 gpt-6.1-sol 条目 272,000→500,000 输入、628,000 上下文上限（第三方转述，未核）。存档 `b-orcarouter.txt` | 已找到 |
| FourWeekMBA | L5 | https://fourweekmba.com/ai-openai-lifts-gpt-6-1-sol-default-context-to-500k-in-codex/ | 仅见标题 “OpenAI Lifts GPT-6.1 Sol Default Context to 500K in Codex” | — | 正文 403，未读 | 未找到一手来源 |
| 站长之家（两篇）、华军软件园、腾讯新闻（WorkBuddy 扫描所称）| — | — | — | — | WebSearch 中英文检索均未搜到这四家的相应报道；未能验证“1M 手动设置情形下的计费规则无公开信息”出自哪篇 | 未找到一手来源 |
| OpenAI 开发者社区专帖（除 28 天汇总帖外）| — | community.openai.com | — | — | 汇总帖之外未见专帖 | 未找到一手来源 |

### B7 热度（取数时刻见括号；北京时间）

| 对象 | 数字 | 来源 | 状态 |
|---|---|---|---|
| Tibo 主帖 views / likes | 882,027 / 9,235（13:10，replies 1,183，retweets 410，quotes 500，bookmarks 700）；957,011 / 9,795（13:52，replies 1,239，retweets 425，quotes 504，bookmarks 730）| fxtwitter JSON `b-fx-2109430343346581527.json`、`.final.json` | 已找到 |
| Tibo 自回复 views / likes | 254,998 / 2,246（13:10）；273,758 / 2,381（13:52；retweets 53，replies 152）| 同上 | 已找到 |
| Hacker News | 无本事件提交（HN Algolia search_by_date，关键词 thsottiaux / Codex / GPT-6.1 Sol / 推文 ID / 500k，13:3x–13:4x）；最近一条 Tibo 帖提交为 9/29。HN 上近两周与 6.1 Sol 相关的条目只有 9/30 “GPT-6.1 Sol replaces GPT-6 Sol after just 7 days …”（80 分/102 评）与 10/9 community 帖 “OpenAI Rolls Out Ultrafast Mode for GPT-6.1 Sol at Up to 8x Speed”（3 分/0 评），均非本事件 | 未找到一手来源（HN 无相关帖） |
| Reddit r/codex | https://www.reddit.com/r/codex/comments/1x3n28b/day_7_tibo/ “Day 7 - tibo”（作者 Mentalist3021，贴 Tibo 帖截图，1791762472 = 2026-10-11 23:47:52Z）：538 分 / 177 评论（13:3x）；571 分 / 186 评论（13:5x）。r/OpenAI、r/ChatGPTCoding 近一周搜 “500k context”“long context fee”“GPT-6.1 Sol context” 未见专帖 | opencli reddit，`b-reddit-codex-day7.json` | 已找到 |
| X 回复里的 Testing Catalog | 16,435 浏览 / 335 赞（13:5x），见 B6 | fxtwitter | 已找到 |
| AIHOT | 首页“热点”榜该事件排第 5，“+460 热度”；热点榜页该事件排第 5（行内标 “爆新”“+43 个来源”）；事件页 “4 篇报道 3 个报道来源 5 小时前更新”；IT之家条 “AI 评分 51”；Testing Catalog 条 “AI 评分 55” | https://aihot.news/ 、/hot 、/story/fa2e5d50-…；本次 curl 未遇 bot 挑战；`b-aihot-home.txt`、`b-aihot-hot.txt`、`b-aihot-story.txt` | 已找到 |

### B8 质疑线索（L6；进正文前必须回到一手来源核实；Reddit 评论无独立链接，只能引帖链接 + 作者 + 得分）

来源帖：https://www.reddit.com/r/codex/comments/1x3n28b/day_7_tibo/（13:3x 取数的评论得分，存档 `b-reddit-codex-day7.json`）。

| 质疑 | 作者 / 得分 | 备注 |
|---|---|---|
| 昨天刚说选 300k 有理由，今天改 500k | CommercialRepeat6842 / 100；其下 Creeper360bill 40（“why did he literally say yesterday the old limit was the best balance just to change it the next day????”）| 对应 Tibo 10/10 回帖（B4-1）原帖核实；注意他说的是 300k，而默认值此前是 272k |
| “只是改了个默认值”（本来就能自己改 config.toml） | yungastronot / 236；Fearless_Sector5542 / 7（“This has always been configurable via config.toml.”）；bekiraydogan / 3 | 与 Tibo 8/16 教程帖对应（`b-fx-2089082893804896524.json`），属实 |
| 真正的变化是“超 272k 不再收 2 倍”（附截图） | hellomistershifty / 126（“This is the actual change, no extra fees for Sol calls over 272k.”）；同一作者另一条评论（13 分）称“它过去按 2.5 倍计并耗额度”（“it used to cost 2.5x as much (and drain usage accordingly)”） | “过去按 2.5 倍计”无一手来源；与 B3-9（Tibo 对 5.6 Sol 说订阅不收）矛盾，待核 |
| “对 5.6 Sol，Tibo 就说订阅没有 2 倍”，所以未必是新变化 | retteh / 8 | 与 B3-9 对应：Tibo 对 5.6 Sol 确实这么说过；对 6.1 Sol 未见 |
| 更大上下文会让每次请求缓存命中读取更多 token、反而更耗额度 | creamyshart / 6；widb0005 / 9（“I wonder what this does to the billing scheme”）| 与 Tibo 7/13 “cost of cache reads going up with the size of the context” 一致（B4-1）；官方未对 500k 默认给出额度影响说明 |
| 500k 下速度很慢、性能不一定好；先前 Tibo 为 300k 辩护称有退化 | DrBearJ3w / 4（“the speed at high context is extremely slow”）；Active_Variation_194 / 4（称 Tibo 数月力主 300k 因模型退化，且“随便一个周日没有模型更新就变好了？”）；AffectionateCap539 / 2 | 无官方准确率数据（B4-2）；“数月力主”未核 |
| “对其他模型反而更贵？”（附截图） | Pronoia2-4601 / 2 | 截图内容 opencli 未取到，未核；0930/10 月官方材料里只有 Astra 在 Codex 的长上下文例外（B3-4） |
| 对只能在 Codex Work 里用的人才有意义（不能改 config） | Active_Variation_194 / 4 | 与官方 “Work” 说明一致（B3-7） |
| 竞品已有 1M 不额外收费 | PotentialPaint6714 / 2 | 未核，不作事实 |
| 使用额度/重置怨言 | 多条（如 No_Quarter_7644 / 95 “Put the banked reset in the bag Tibo.”）| 属额度重置话题，不取证 |
| X 回复里同类质疑（低赞） | ImKKingshuk（“Tibo yesterday: “We ran the numbers. 300K is the optimal context window. Good defaults matter.” Tibo today: … 500K is the new sweet spot.””）；jaffy_tech（“when the context length exceeds 200K tokens, performance becomes very slow even with Fast Mode enabled”）等 | opencli twitter thread 13:3x，点赞数多为 0–1，不作排序依据；未存档 |
| 第三方用量估算工具曾把 6.1 Sol 的长上下文阈值误设为 200K（而非 272K） | steipete/CodexBar issue #4302，https://github.com/steipete/CodexBar/issues/4302 （2026-10-06T08:13:16Z 提出，10/7 06:06 关闭，已修）| 原文：“GPT-6.1 Sol requests with 200,001–272,000 input tokens receive long-context pricing, although OpenAI's threshold is more than 272,000 input tokens.” | 与本事件无直接关系；说明第三方成本估算工具对该阈值曾出错，供“别用第三方估算套订阅用量”的线索。存档 `b-codexbar-issue.txt` |
| Codex 客户端“提前压缩/计数偏差”旧问题 | https://github.com/openai/codex/issues/49961 （2026-10-01，open）| “Codex interrupted real work for over five minutes to compact a session whose last reported usage was 193,193 tokens. Its internal compaction counter said 266,425.” 与默认值提高无直接关系，仅供“压缩”相关线索 |

## 三、可能的吠点（原文里有、但线索摘要没提或说反的条件）

1. 自回复限定在“when drawing usage from the subscription”，帖子没有说 API、积分购买用量、Enterprise 按 token 计费是否同样不收长上下文费（B1-2）。
2. 默认值只对“Codex & Work”生效；API 模型页 10/11 12:10Z 快照与 10/12 取到的版本无差异，仍写 >272K 整个请求 2 倍输入/缓存、1.5 倍输出（B2-3）。
3. OpenAI 官方文档里没有这次改动的任何条文：Codex 定价页、模型页、changelog（最新 10/8）、两份 help.openai.com 费率卡均未更新或未提；官方账号（OpenAI、OpenAIDevs）未见帖。仅 Tibo 个人帖（L2）与官方社区自动汇总帖（B3-6、B1-4）。
4. help.openai.com 的 Enterprise 费率卡（“Updated: yesterday”）仍列 >272K 的 2×/2×/1.5× 倍率，只写 Astra 在 Codex 的例外，未写 Sol（B3-4）。
5. 默认值此前就可由用户自改（`model_context_window = 1000000` 等）并对 ChatGPT 账号开放；Tibo 8/16–17 帖都附“we tuned the default carefully/we have tuned it to ~perfection”之类警示（`b-fx-2089082893804896524.json`、`b-fx-2089143488696705077.json`）。
6. Tibo 对 5.6 Sol 已说过订阅不收超 272k 的额外费（7/12、7/13）；“不收费”对 6.1 Sol 是否为新变化，官方材料未交代（B3-9）。
7. 官方 FAQ 写明上下文大会让单条消息用更多额度（B3-1）；Tibo 10/10 称 1M “would just cost more of the usage”（B4-1）；Day 7 帖未说 500k 默认是否更耗额度。
8. “maximum 1M”（Tibo）、1,050,000（API 模型页）、872,000（Codex 仓库内置 `max_context_window`，改动前）三个上限数字口径不同（B2-6）。
9. “~80%”：500/272 ≈ 1.838，即约 84%（推算，非他的错，但摘要若写“增加 80%”需注明为 Tibo 原话的约数）。
10. Day 7 是“28 天连发”系列的第 7 条（Day 6 是全局重置）；帖文是个人发言而非发布公告，措辞“carefully tested”“sweet spot”未附测试数据（B4-2）。

## 四、夸大说法实例（把事件写歪的标题或文章；原站 URL、原句逐字、发布时间、发布方）

1. **漏掉“订阅”限定**：AIHOT（aihot.news，L5 聚合站，AI 生成导读）https://aihot.news/items/zhcf6aews0gj0nijx0zfnsqxt ，导读：“配合本次调整，OpenAI 不会对 GPT-6.1 Sol 收取长上下文费用。”；其来源 IT之家正文（https://www.ithome.com/1/011/736.htm，2026/10/12 8:23:08 北京）原为“当从订阅中扣除使用量时，OpenAI 将不会对 GPT-6.1 Sol 收取长上下文费用”。发布方 AIHOT，页面时间 2026-10-12 08:23（取自 IT之家发布时刻）。事件级标题 “…并取消订阅长上下文费” 带限定。
2. **把“订阅用量”写成“订阅积分”**：KuCoin Flash（转自 BlockBeats / ME AI），https://www.kucoin.com/news/flash/openai-boosts-gpt-6-1-sol-default-context-to-500k-tokens-removes-long-context-fee-for-subscribers ，“OpenAI is no longer charging an additional fee for long context usage with GPT-6.1 Sol when using ChatGPT subscription credits.”；“Release Time: 2026/10/12 03:03:32”（时区未标）。原帖没有 “credits” 字样；官方文档里 credits 是另一套（购买积分）计费，其下是否收长上下文费未见官方说明（B3-2、B3-4）。
3. **补出原帖没有的动因**：Crypto Briefing，https://cryptobriefing.com/openai-gpt-6-1-sol-500k-context/ ，“OpenAI framed the adjustment as a response to user feedback.”（页面日期 Oct. 11, 2026）。Tibo 两条原帖里没有“user feedback”；原话的理由是 “it helps reduce occurrence of compactions … carefully tested and is the new sweet spot”。
4. 头衔：KuCoin “OpenAI’s Product and Platform Lead”、OrcaRouter “head of product and platform”——本组找不到一手出处（X 简介只写 “Codex & ChatGPT @OpenAI”），属待核。
5. 未找到把本事件写成“长上下文全免费”“API 也免费”“1M 免费”的标题或文章；所见英文媒体（Crypto Briefing、OrcaRouter）与 IT之家均保留订阅限定，OrcaRouter 明确写 “On an API key it is still there”。社区评论里 Otherwise-Sir7359（r/codex，58 分）写 “Before, it was double the price, and now it's free.”，属评论者概括（L6）。

## 五、扫描说法勘误

| 扫描/初步说法 | 复核结果 |
|---|---|
| 主帖 created_at “Sun Oct 11 23:45:59 +0000 2026”（= 北京 10/12 07:45）；自回复 23:48:11Z | 与 fxtwitter JSON 一致（本组独立取到）。 |
| 原句三段（500k、sweet spot、1M）与自回复 | 逐字与 JSON 一致（见 B1）。 |
| “Day 7/ 属 OpenAI 员工的连续发布系列；ChatGPT 扫描引用社区帖” | 属实；系列由 Tibo 个人发起（“Over the next 28 days …”），不是 OpenAI 官方公告；Day 6 是全局重置。 |
| API 模型页 1,050,000 / 128,000 / >272K 计费句 | 属实；且页面在 Tibo 发帖前后无变化（Wayback 10/11 12:10Z 对照）。 |
| “Tibo 说上限 1M，文档写 1,050,000，两者并列” | 另有第三个数字：openai/codex 仓库内置目录 `max_context_window: 872000`（B2-6）。 |
| 中文报道（WorkBuddy 称站长之家两篇、华军软件园、腾讯新闻）称“1M 手动设置情形下的计费规则无公开信息” | 这四家报道本组**均未搜到**（未找到一手来源）；实际找到的中文报道是 IT之家（8:23:08）。关于“1M 手动设置下的计费”：官方文档无相关条文，API 页写 >272K 2×；Tibo 自回复限定“subscription”。故“无公开信息”对官方文档成立，但 Tibo 帖对订阅用量已有总括陈述。 |
| 初步核对未提到的重要对照 | Tibo 10/10 回帖（300k “We ran the numbers”）与 7/12–13 对 5.6 Sol 的陈述（B3-9、B4-1）——这是本组发现的最大“前后对照”线索；help.openai.com Enterprise 费率卡仍写 >272K 倍率（B3-4）。 |
| “Work” | 官方称 ChatGPT Work（B3-7），Codex 定价页写明 “ChatGPT Work and Codex share usage”。 |

## 六、本组假设

- “北京时间 = 本机 EDT + 12 小时”；X 嵌入卡片显示时刻为本机时区（EDT），与 fxtwitter UTC 时刻互证。
- 热度、页面内容为取数时刻值；`views` 来自 fxtwitter（X 显示值），与 opencli 同期读数略有差异（如主帖 901,112 views / 9,405 likes 于约 13:36 读到，仅记在此处）。
- Reddit 评论得分取自 opencli 读取，未能给出评论永久链接；评论内容只作线索。
- firecrawl 摘录由本组手工写入文本文件，保留原文关键段落，不是整页存档。
