# 1125 A 组取证：GPT-6.2 计费修正——“快 40% 触顶”是跟上周比

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间一律北京时间（UTC+8，本机时钟），括号保留原时区。抓取记录见 `capture-log-a.md`，批量抓取逐条结果见 `a-fetch-log.jsonl`，截图动作见 `a-shots-log.jsonl`，热度见 `a-heat.json`。级别按 `EDITORIAL.md`“事实分级”。X 帖子里的“banked reset”补偿句只逐字存档，未研究重置。

**核心结论（已对原帖与官方页面核实）**

- 原帖的 40%/25% 基线是“last week”，即 GPT-6.2 上线（11-17）后用错误打折计量的那一周，不是 GPT-6 Astra 或 GPT-6.1 Sol。同帖另有一句“You should still see these go further in usage compared to 6 Astra and 6.1 Sol”，带 “should”，无数字。
- 官方没有给出“同样的活多扣多少”的数值，没有绝对额度（套餐页只有“本地消息数估计区间”，修正前后未改），官方帮助中心、Codex 文档、changelog、status 页均无计费修正条目。
- 扫描称“TechCrunch 11-25 引发言人‘少数重度用户可能更早触顶’”：该稿讲的是 Chat 与 Work 合并共享额度，不是 GPT-6.2 计费修正（见“扫描说法勘误”）。
- 邮件（11-25 起用户晒图）、Ultrafast、“11-17 上线”均能找到出处，但邮件只有用户晒的截图（L7 性质），官方公开页没有。

## 一、取证表

### A1　Tibo 原帖

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1-1 Tibo 北京 11-26 10:50 发帖 | L2（OpenAI 员工署名公开发言，非官方号） | https://x.com/thsottiaux/status/2125784147889393806 （fxtwitter JSON：`a-x-tibo-2125784147889393806.raw.json`） | “We made a billing mistake at launch of GPT-6.2 that incorrectly discounted GPT-6.2 Astra and Sol usage for most users. / We’ve fixed this now, so the same work will use more of your plan’s usage allowance. Comparing to last week, you will reach your limits 40% faster on GPT-6.2 Astra and 25% faster on Sol. You should still see these go further in usage compared to 6 Astra and 6.1 Sol. Other models weren’t affected, and we won’t charge you for the difference on past usage. / As a small token to apologize for the mistake, we’ll be adding one banked reset for everyone on Go, Plus, Pro, and Business.”（最后一段只存档；“/”表示原文段间空行） | `images/01-tibo-post-card.png`（x.com 页面截取帖子卡片；深色底；卡上时间为本机北京时间 “10:50 AM · Nov 26, 2026”） | `created_at` “Thu Nov 26 02:50:10 +0000 2026”＝北京 11-26 10:50:10；`is_note_tweet` true（长帖）；`replying_to` null（独立帖，非回复）。互动数：fxtwitter 21:3x 读 回复 5226 / 转发 822 / 赞 15098 / 书签 2174 / 引用 2325 / 浏览 3,680,086；opencli 约 22:00 读 赞 15106 / 浏览 3,682,310；页面显示 5.2K 回复、15K 赞、3.7M 浏览。与已知情况的原文逐字一致 | 已找到 |
| A1-2 Tibo 的公开职务 | L1/L2 | https://x.com/thsottiaux （fxtwitter `/thsottiaux` 用户 JSON，21:3x） | X 简介原句：“Codex & ChatGPT @OpenAI”（`verification.type` individual，已认证）。TechCrunch 11-25 稿称他 “product lead Thibault “Tibo” Sottiaux”（`a-techcrunch-1125.txt`）；中文稿写法不一（见 A6）：智东西“ChatGPT和Codex负责人”、Odaily“Codex负责人”、BlockBeats“核心产品与平台负责人” | 无 | OpenAI 官方页上的职务未找到；只引 X 简介原句 | 已找到（X 简介），官方页未找到 |
| A1-3 同串回复里 Tibo 本人的补充说明 | — | fxtwitter `/2/thread/2125784147889393806`（仅 1 条）；opencli `twitter thread`（取前 96 条回复，Tibo 本人 0 条）；opencli `twitter search "from:thsottiaux since:2026-11-25 filter:replies"`（6 条，均为无关回复：欢迎 trq212 加入、charliermarsh、jxnlco） | 无 | 无 | 回复共 5226 条，只扫到约 96 条，未覆盖全部；Tibo 当日另有一条 07:26:16 UTC（北京 15:26）的帖，内容为重置到账通知，按规定不展开 | 未找到一手来源（Tibo 补充说明） |
| A1-4 OpenAI 官方开发者号转发 | L1 | opencli `twitter search "from:OpenAIDevs GPT-6.2 usage since:2026-11-24"`，结果 id 2125786156164067358 | “RT @thsottiaux: We made a billing mistake at launch of GPT-6.2 that incorrectly discounted GPT-6.2 Astra and Sol usage for most users.”（搜索结果截断，原帖 ID 对应 A1-1） | 无 | `created_at` Thu Nov 26 02:58:09 +0000 2026＝北京 10:58:09；只读到搜索结果摘要，未存档 | 已找到 |

### A2　基线与范围

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A2-1 “Comparing to last week” 的基线 | L2 | A1-1 | “Comparing to last week, you will reach your limits 40% faster on GPT-6.2 Astra and 25% faster on Sol.” | 01 | 同帖开头“at launch of GPT-6.2 … incorrectly discounted”：上周＝上线（11-17）后的一周，是被错误打折的计量；原帖未写“上周”具体日期 | 已找到 |
| A2-2 “reach your limits 40% faster” 有无官方数值解释（每单位工作多扣多少、额度本身是否变化） | L1 | 查过：help 20001625、20001275、20001516、9793128、11369540；developers.openai.com/codex/pricing（learn.chatgpt.com/docs/pricing 同页）、/codex/changelog、/codex/agent-configuration/speed、/codex/models；status.openai.com | 无。唯一的官方侧文字是邮件（用户晒图，见 A3）：“With this correction, you may notice that you are reaching your usage limit sooner.”；同帖前半句“the same work will use more of your plan’s usage allowance”只给方向 | 无 | 原帖没说“40% faster”是指每次工作多扣 40%、还是单位时间消耗速度快 40%；“额度本身是否变化”官方未说。推算（非官方，供 Claude 核）：若指触顶速度为上周的 1.4 倍，则同一份额度能做的同样工作约为上周的 1/1.4≈71%（Astra），Sol 1/1.25=80%；新浪微博博主宝玉xp 也这样换算（A6-4） | 未找到一手来源（数值解释） |
| A2-3 “You should still see these go further in usage compared to 6 Astra and 6.1 Sol” | L2 + L1（旁证） | A1-1；官方发布页 https://openai.com/index/introducing-gpt-6-2-astra-sol/ ；changelog https://developers.openai.com/codex/changelog （2026-11-17 条目） | Tibo：“You should still see these go further in usage compared to 6 Astra and 6.1 Sol.”（带 “should”，无数字）。官方 changelog 2026-11-17：“With GPT-6.2 Sol, you can complete more tasks on the same paid plan than with GPT-6.1 Sol.”；发布页：“Both models deliver better results on demanding tasks with fewer output tokens, so you can get more done within your usage limits.”；“Compared with GPT‑6.1 Sol, GPT‑6.2 Sol is smarter, more efficient, and lets you complete more tasks on the same paid plan.”；“GPT‑6.2 Astra costs significantly less per task than GPT‑6 Astra, driven by better token efficiency, a 20% discount on input and output tokens, and a deeper 60% discount on cached input tokens.”（后一句是 API 价与“每任务成本”，不是订阅额度） | `images/02-changelog-62-entry.png`、`images/09-openai-launch-usage-sentence.png` | 官方“同一订阅更多任务”只对 Sol（对 6.1 Sol）明说；对 Astra 的订阅额度，官方只在发布页说“fewer output tokens … more done within your usage limits”，没有对 GPT-6 Astra 的订阅额度对比数字 | 部分支持 |
| A2-4 “Other models weren’t affected” | L2 | A1-1 | “Other models weren’t affected, and we won’t charge you for the difference on past usage.” 邮件（用户晒图）：“Some GPT-6.2 Astra and Sol requests in Codex used less of your allowance than intended, including in Ultrafast mode.” | 01 | “most users”（帖）与 “Some … requests in Codex”（邮件）措辞不同；“不追缴”只在帖里，邮件无（Yahoo 香港转述：邮件没有交代补偿与追扣，A6-6） | 已找到 |
| A2-5 受影响的套餐与入口（ChatGPT、Codex、API） | L1 | https://developers.openai.com/codex/pricing ；https://help.openai.com/en/articles/20001275 | 套餐页 “Feature availability”：“GPT-6.2 Astra and GPT-6.2 Sol are available on Plus, Pro, Business, Enterprise, and Edu in ChatGPT Work on web, mobile, and desktop, and in the Codex app and CLI.” 帮助中心 20001275：“Work follows the same usage structure as Codex.” Tibo 补偿名单：“everyone on Go, Plus, Pro, and Business”（A1-1） | 无 | 官方没有明说修正涉及哪些套餐与入口；邮件只写 “in Codex”；补偿名单不含 Enterprise/Edu；Free/Go 在套餐页只列 “GPT-6 Luna”（见 A4）。Work 是否同样受影响官方未说（第三方推文称 Codex 与 Work 都受影响，L6，未核） | 部分支持 |
| A2-6 API 按 token 计费的用户是否受影响 | L1 | https://developers.openai.com/codex/pricing ；https://help.openai.com/en/articles/20001516 | 套餐页开头：“API-key billing: API-key sessions use separate API pricing, not your ChatGPT allowance. API prices don’t estimate included tasks.”；20001516：“API usage is billed separately at API rates.”；API 价（套餐页，21:39 读）：“GPT-6.2 Astra at Standard speed costs $8 per million input tokens, $0.40 per million cached input tokens, and $40 per million output tokens. GPT-6.2 Sol costs $2, $0.10, and $10 …” | `images/10-pricing-token-rates.png` | 官方只说 API-key 会话与套餐额度分开计；没有就这次计费错误是否涉及 API 发声（HN 用户 krzyk 也在问：“I wonder if this is also true for API pricing.”）。API 价格：发布页（`a-openai-intro-62.txt`）与现行套餐页同为 $8/$0.40/$40 | 未找到一手来源（是否涉及 API）；API-key 会话不扣套餐额度：已找到 |

### A3　官方书面说明

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A3-1 官方帮助中心、Codex 文档、changelog、status 页有无计费修正条目 | L1 | 21:39–21:56 读：help 20001625、20001275（另有 2026-11-18 08:25 北京的 Wayback 快照）、20001516、9793128（Pro）、11369540（Codex rate card）；https://developers.openai.com/codex/changelog （页头 Last-Modified Thu, 26 Nov 2026 12:01:49 GMT＝北京 20:01:49）；https://status.openai.com/ | 无计费修正条目。changelog 最近条目日期 2026-11-24（CLI 0.172.0）；最近与 6.2 相关的是 “2026-11-17 GPT-6.2 Astra and GPT-6.2 Sol in Work and Codex”。status 页当时只有 “Elevated errors with Codex GitHub reviews and pull request merges”（与计费无关） | `images/02-changelog-62-entry.png` | 只读到快照时刻的页面；官方可能随后补写 | 未找到一手来源（官方书面说明） |
| A3-2 扫描说的“Ultrafast 模式” | L1 | https://help.openai.com/en/articles/20001275 ；https://developers.openai.com/codex/pricing “Speed multipliers” | 帮助中心：“Pro $500: For GPT-6.2 Astra and GPT-6.2 Sol, Ultrafast uses your included Work and Codex allowance at 6× the Standard rate for the selected model. … Pro $200 includes GPT-6.2 Sol Ultrafast in Codex using existing plan limits. … Ultrafast remains unavailable on Plus, Pro $100 and Business, including through credits.” 套餐页：Fast 2x（6.2），Ultrafast included usage 6x、Credits/PAYG “3x* / 6x”（* 3x 至 2027-02-28） | 无 | 官方页 6× 规则在 11-18 08:25 北京的 Wayback 快照与现页相同，即不是修正后改的。邮件原话“including in Ultrafast mode”（A3-4）把 Ultrafast 请求也列入被少扣 | 已找到（Ultrafast 规则与邮件提及） |
| A3-3 “自 11 月 17 日上线起” | L1 | https://developers.openai.com/codex/changelog ；Tibo 上线帖 https://x.com/thsottiaux/status/2122753991000133943 | changelog：“2026-11-17 GPT-6.2 Astra and GPT-6.2 Sol in Work and Codex … are now available in ChatGPT Work and Codex.”（官方只给日期，官方未给时刻）。Tibo：“GPT-6.2 Sol & Astra are *finally* here. We’ve also expanded ultrafast to Pro 200 plans for GPT-6.2 Sol, it’s my go-to model for most day to day things.” | `images/11-tibo-launch-efficiency-post.png`（第二条，见下） | Tibo 上线帖 `created_at` Tue Nov 17 18:09:24 +0000 2026＝北京 11-18 02:09:24；第二条 id 2122754551187132438 北京 11-18 02:11:38：“These models are extremely efficient, which you feel in speed to complete tasks, cost and how much further your usage goes”（附 Agents’ Last Exam v1.1 图，Cost 轴）。官方上线日 11-17（美国日期）与北京 11-18 差一天，写作时注意 | 已找到 |
| A3-4 “Codex 通知邮件” | L7（用户晒图；邮件本身来自 OpenAI，官方公开页没有） | 用户 X 帖：https://x.com/CodexUpdate/status/2125427689460785413 、https://x.com/zane_wang76/status/2125428139233054830 、https://x.com/Laierlega/status/2125426980686602732 （`a-x-*.json`）；转述：https://hk.news.yahoo.com/codex-%E7%94%A8%E9%87%8F%E6%89%A3%E6%B8%9B%E5%87%BA%E9%8C%AF-openai-%E9%A0%90%E5%91%8A-sol-113423719.html （`a-cn-yahoohk.txt`） | 邮件图（目视，英文逐字）：“Hello, / Some GPT-6.2 Astra and Sol requests in Codex used less of your allowance than intended, including in Ultrafast mode. / With this correction, you may notice that you are reaching your usage limit sooner. / Your subscription price is not changing. You can review your remaining usage and reset times in Settings → Usage. / Thank you, / OpenAI”。Yahoo 香港（newmobilelife）称邮件题为「An update to Codex usage」 | 无（邮件图为用户转贴，未存入仓库；文本已抄录） | 用户帖 `created_at` Wed Nov 25 03:10:55 +0000 2026（Laierlega，最早）＝北京 11-25 11:10:55；CodexUpdate 03:13:44＝11:13:44；Yahoo/newmobilelife `datePublished` 2026-11-25T11:34:23Z＝北京 11-25 19:34:23。邮件早于 Tibo 帖约 23 小时；邮件无数字、无补偿、无“不追缴”。HN 用户 Aurornis 称“earlier … weird, short email … follow up email today with enough details”（有第二封，未见到第二封原文，L6） | 部分支持 |

### A4　绝对额度

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A4-1 Plus/Business 标准席位 GPT-6.2 Astra、Sol 额度有无数字 | L1 | https://developers.openai.com/codex/pricing （同页 https://learn.chatgpt.com/docs/pricing ；Included usage 节，21:39 读） | “The estimates below show local messages per five-hour period for Plus and Standard Business. Pro plans currently have no five-hour limit. Cloud tasks may use more of your allowance than local messages. Usage depends on the model and task. These estimates are not fixed message limits; check your usage dashboard for current limits and reset times.” 表（Plus / Standard Business，两列相同）：GPT-6.2 Astra 6-50；GPT-6.2 Sol 17-180；GPT-6 Astra 5-45；GPT-6.1 Sol 15-160；GPT-6.1 Terra 35-285；GPT-6 Sol 15-150；GPT-6 Luna 350-3,000。“Local messages and cloud chats share your plan’s usage allowance. Weekly limits may also apply.” | `images/03-pricing-plus-estimates.png`（深色底，约 2 倍） | 单位是“本地消息数/5 小时窗口”的估计区间，官方明说不是固定上限；没有周额度数字、没有 Pro 各档数字。帮助中心 20001516 另说：“Your allowance does not correspond to a fixed number of messages or tasks.” | 部分支持（只有估计区间；官方未公布绝对额度） |
| A4-2 修复前后页面是否改过 | L1（Wayback 快照仅用于看“页面以前写了什么”，非官方） | Wayback 快照 `https://web.archive.org/web/<ts>id_/https://learn.chatgpt.com/docs/pricing`，存 `a-wayback-learn-pricing-<ts>.html/.txt` | 6.2 行在 UTC 2026-11-18 03:56（北京 11:56，上线后）、11-19、11-23、11-24 14:05、11-25 14:05（北京 22:05）、11-26 03:16（北京 11:16，Tibo 帖后 26 分钟）快照与 21:39 现页相同：GPT-6.2 Astra 6-50、GPT-6.2 Sol 17-180、GPT-6 Astra 5-45、GPT-6.1 Sol 15-160；11-15 快照（上线前）没有 6.2 行，其余相同。快照/现页 `a-wayback-learn-pricing-20261126031654.txt` 与 `a-codex-pricing.txt` 逐字一致 | 无 | 区间表在修正前后都没改，官方没说这些区间对应修正前还是修正后的计量；页面 Last-Modified 头 Thu, 26 Nov 2026 12:01:46 GMT（北京 20:01:46），但文本与 03:16 UTC 快照无差异 | 已找到（页面未改动） |
| A4-3 每百万 token 的额度价 | L1 | 同 A4-1 “Token rates” 节 | “GPT-6.2 Astra 200 credits 10 credits 1,000 credits”；“GPT-6 Astra 250 credits 25 credits 1,250 credits”；“GPT-6.2 Sol 50 credits 2.5 credits 250 credits”；“GPT-6.1 Sol 50 credits 2.5 credits 250 credits”（输入/缓存输入/输出，每 1M token）；“Credit prices alone don’t determine included subscription usage; check your usage dashboard for current limits.” | `images/10-pricing-token-rates.png`（深色底，约 2 倍） | 这是购买额度（credits）的 token 价，不是订阅内含额度；官方明说不决定订阅额度。11-18 快照起相同 | 已找到 |
| A4-4 Pro 档绝对额度 | L1 | https://help.openai.com/en/articles/9793128 （Updated: 3 days ago，约 11-23） | “Pro 100 and Pro 500 now include twice their previous Work and Codex allowance. On Pro 200, only the lower allowance doubled.”；“Plus usage is unchanged.”；“A larger allowance does not guarantee twice as many completed tasks.”；示例“These numbers are illustrative and do not represent published plan quotas.” | 无 | 无绝对数；该页没有提到 6.2 计费修正 | 官方未公布绝对额度 |

### A5　与 12 月额度合并的关系

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A5-1 1031 期那份帮助页现行版本 | L1 | https://help.openai.com/en/articles/20001625-understand-chatgpt-and-codex-usage-limits （`a-help-20001625.txt`，21:39 读；页头 “Updated: 2 days ago”，浏览器里显示“前天”，约 11-24，绝对时刻未取到） | “Starting in December, Plus, Pro, and Business plans’ usage allowance will be shared across ChatGPT and Codex as we bring the agentic capabilities of Work into the main ChatGPT experience.” “Your usage for ChatGPT and Codex will be measured in tokens … Instant chat will not draw from your usage.” “We expect most users won't need to wait for their usage to reset more often than they do now.” **新增一段（1031 存档 `docs/2610/1031/sources/a-help-usage-rendered.txt` 没有）**：“A small group of users may see the usage changes before December so we can learn from early feedback before expanding more broadly.” | `images/04-help-usage-limits.png`（浏览器本机语种使页头显示“Updated: 前天”，非页面英文原文） | 1031 存档 updatedAt＝2026-11-01 00:17:48 北京（见 1031 capture-log）；现页 2 天前更新，多出上面一段。页面没有 6.2 计费修正字样 | 已找到 |
| A5-2 TechCrunch 11-25 报道原句 | L4 | https://techcrunch.com/2026/11/25/openai-says-heavy-chatgpt-users-may-hit-limits-sooner-as-it-merges-chat-and-work/ （`a-techcrunch-1125.txt`） | 标题 “OpenAI says heavy ChatGPT users may hit limits sooner as it merges Chat and Work”；作者 Jagmeet Singh；“12:03 PM PST · November 25, 2026”。“An OpenAI spokesperson told TechCrunch that while most users are not expected to face more frequent usage resets, a small fraction of its heaviest users may hit their limits sooner.” “Under the new system, Chat and Work will share a single usage meter across ChatGPT … Conversations using the Instant model will, however, remain unlimited and will not count toward the shared allowance.” “The spokesperson added that OpenAI would continue working to improve efficiency so subscribers could get more out of their plans.” “OpenAI plans to bring shared usage limits more broadly in December, per its support page.” | 无 | 12:03 PST（UTC-8）＝11-25 20:03 UTC＝北京 11-26 04:03。通篇讲 Chat 与 Work 合并，未提 GPT-6.2、40%、计费错误；发言人原句涉及“更频繁重置”与“最重度的一小部分用户”，是合并共享额度的话 | 已找到（但与计费修正无关） |
| A5-3 Tibo 11-25 的合并推出帖 | L2 | https://x.com/thsottiaux/status/2125284868414394550 | “Hi. We’re beginning to roll out the combined Chat and Work experience to a small group of Pro users to learn from feedback before we roll out more broadly.” | 无 | `created_at` Tue Nov 24 17:46:13 +0000 2026＝北京 11-25 01:46:13（TechCrunch 引为 “Tuesday”＝美国日期）。来自 Tibo 时间线（`opencli twitter tweets`），未另存 JSON | 已找到 |
| A5-4 两件事是否被官方放在一起说明 | — | 上述官方页、Tibo 帖、邮件 | 帮助页、Tibo 帖、邮件均未把 6.2 计费修正与 12 月合并额度并列。把两事写进同一篇的是媒体：智东西（A6-1）、Notebookcheck 中文版（A6-5，“12 月 ChatGPT 和 Codex 共用同一额度后会有哪些变化”） | 无 | HN 用户 Aurornis 与 aliljet 把两事连在一起评论（L6，见 A7） | 未找到官方并置说明 |

### A6　夸大与不同写法的实例（只摘与“40% faster”相关的写法）

| 序 | 发布方 · 标题 | URL | 发布时间（北京） | 把“40% faster”写成什么 | 是否删去“仍比上一代更耐用”“不追缴”等 | 存档 | 截图 |
|---|---|---|---|---|---|---|---|
| A6-1 | 凤凰网（大风号·智东西）《突发！ChatGPT额度爆砍接近一半？》；163 网易号同稿 | https://tech.ifeng.com/c/8yMr1LnINIO 、https://www.163.com/dy/article/LCM00PHK051180F7.html | 2026-11-26 11:04:36 | 标题：“突发！ChatGPT额度爆砍接近一半？”；正文：“据Tibo所说，与上周相比，用户使用GPT-6.2 Astra和GPT-6.2 Sol触及额度上限的速度将分别加快40%和25%。”（标题的“接近一半”正文无对应数字；标题写 ChatGPT 额度，正文写 Codex 用户） | 保留 Tibo“仍比旧版耐用”但转为反驳：“虽然Tibo在最新的声明中说，即使修复了这一错误，用户使用GPT-6.2 Astra和Sol完成任务时，仍会发现同样的用量额度能支持更多工作，但事实好像并非如此。”未写“不追缴”与“其他模型不受影响”；写“并宣布发放一次可自主使用的额度重置作为补偿”。开头“OpenAI的ChatGPT和Codex负责人Tibo”；“导致大多数Codex用户的实际额度消耗低于OpenAI预期” | `a-cn-ifeng.html/.txt`、`a-cn-163-zhidx.html/.txt` | `images/05-ifeng-zhidx-title-lede.png`、`images/06-ifeng-zhidx-email-marketing.png` |
| A6-1b | 同稿的“营销方式”一句 | 同上 | 同上 | 原句：“有用户推测，这可能是OpenAI的一种营销方式。从痕迹上看的确有道理，因为两款模型在智力水平上无法与Claude新模型抗衡，那么价格优势就是最吸引人的地方，用错误的计费来让额度翻倍，再用某种手段取消翻倍，OpenAI可谓是又出“昏招”。”——“有用户推测”引的是文内嵌入的一条 Reddit 评论图（英文：“Since it's impossible for users to independently verify this, it seems reasonable for them to assume this is merely an excuse for frontier model access shrinkflation for subscriptions, after the initial launch of the latest model.”，用户名被稿件自己打码）；“从痕迹上看的确有道理……”及“昏招”是记者自己接的判断，不是用户原话 | 无 | `a-cn-ifeng.txt` | `images/06-ifeng-zhidx-email-marketing.png` |
| A6-2 | 新浪微博·宝玉xp（前微软 ASP.NET 最有价值专家，微博认证科技博主；L6）《OpenAI 计费出错：额度少扣了，同样的钱能跑的活变少了》 | https://www.sina.cn/weibo/detail/5369482861087146.html | 2026-11-26 12:38（页面显示“26-11-26 12:38”） | “简单来说就是额度降了。”“换算下来，同一份额度能干的活，Astra 大约剩上周的七成，Sol 剩八成。比如上周一个 5 小时窗口能跑 10 个 Astra 任务，现在大约 7 个，Sol 从 10 个变成 8 个。”（“比如”为作者举例，非官方数字） | 保留：“Tibo 说，修正后这两款仍然比上一代 GPT-6 Astra 和 GPT-6.1 Sol 更耐用。”“过去少扣的不追收，其他模型和 API 价格都没变。”（“API 价格都没变”不在 Tibo 原帖，见 A2-6、勘误 8） | `a-cn-sina.html/.txt` | 无 |
| A6-3 | Odaily（经腾讯新闻） | https://news.qq.com/rain/a/20261126A06U1M00 、https://www.odaily.news/zh-CN/newsflash/539085 | 2026-11-26 10:54（页面“2026-11-26 10:54发布于北京”） | 标题：“OpenAI Codex负责人：已修复GPT-6.2计费错误，Astra和Sol用户额度消耗将加快”；正文“用户使用 GPT-6.2 Astra 时将快约 40%触及额度上限，使用 Sol 时则快约 25%”（“约”为转述者所加） | 保留“仍高于 GPT-6 Astra 和 GPT-6.1 Sol”“不会追收”“其他模型未受影响” | `a-cn-qq-odaily.txt`、`a-cn-odaily.html/.txt` | 无 |
| A6-4 | 鉅亨網转载 BlockBeats 律动财经快讯《OpenAI公布Codex計費錯誤細節：Astra額度消耗加快40%，Sol加快25%》 | https://news.cnyes.com/news/id/6672822 （原链 https://m.theblockbeats.info/tw/flash/379398） | “2026-11-26 14:05”（页面“BlockBeats 律動財經2026-11-26 14:05”，时区未标，鉅亨为台湾站，按北京处理） | “额度消耗加快40%”；正文“触及额度上限的速度将加快 40%” | 保留“仍比各自的前代…更耐用”“不会追扣”“其他模型不受影响”。把 Tibo 写成“OpenAI 核心产品与平台负责人” | `a-cn-cnyes.html/.txt` | 无 |
| A6-5 | Notebookcheck 中文版（DeepL 翻译自德文版，作者 Steffen Zahn）《OpenAI 修复计费问题后，ChatGPT 用户达到使用上限所需的时间最多缩短 40%》 | https://www.notebookcheck-cn.com/OpenAI-ChatGPT-40.1460795.0.html | 页面“Published 11/26/2026”，页内时间 “26.11.2026 14:46”（站点为德国，时区未标，推测中欧时间＝北京 21:46，未核） | 标题与导语：“达到用量上限所需的时间最多缩短 40%”（原帖是“reach your limits 40% faster”；“时间缩短 40%”与“速度快 40%”不是同一个数，“最多”原帖没有）；正文改回“达到用量上限的速度将分别加快 40% 和 25%” | 保留“仍能比各自的前代…完成更多工作”“不会就以往用量补收差额”；另写“Sottiaux 只提到了套餐的用量额度，并未说明 Chat 选项卡中的消息条数上限是否也会变化。” | `a-cn-notebookcheck.txt`（opencli 读页面文本；`a-cn-notebookcheck.html/.headers` 是 Cloudflare 挑战页，无用） | 无 |
| A6-6 | Yahoo 香港（newmobilelife 稿）《Codex 用量扣減出錯　OpenAI 預告 Sol/Astra 修正後額度將更快用完》 | 见 A3-4 | 北京 11-25 19:34:23 | 写邮件：“修正後，用戶可能會發現自己比之前更早觸及用量上限”；“官方電郵沒有交代受影響的時間範圍、少扣比例或補償安排” | 早于 Tibo 帖，无“更耐用”一说（当时官方尚未说） | `a-cn-yahoohk.html/.txt` | 无 |
| A6-7 | Hacker News 标题（Herrera 提交，域名 twitter.com/thsottiaux） | https://news.ycombinator.com/item?id=50651117 | 提交 2026-11-26 11:36:58（API `created_at` 03:36:58Z） | “OpenAI cuts GPT-6.2 usage: Astra hits limits 40% faster, Sol 25% faster”——标题用 “cuts usage”，Tibo 原帖没有 “cut”；原帖说的是修一个“discounted”的计费错误 | 标题没带“仍比上一代耐用”“不追缴” | `a-hn-50651117.json` | `images/07-hn-title-score-v2.png` |
| A6-8 | Reddit r/codex 用户评论（L6）：把“40% faster”读成“少 40%” | https://www.reddit.com/r/codex/comments/1y5q43m/tibos_update_on_the_gpt_62_usage/ （`a-reddit-codex-1y5q43m.json`） | 评论 11-26 11:22–12:43 | aliljet（11:22，5 分）：“40% less usage on 6.2 astra and 25% less with 6.2 sol”；Victorvic1（12:43，24 分）：“Open AI publicly decreased it by 40%?!”；Dynamix86（11:50，6 分）：“telling users they will get 40% or 25% less usage”；R009k（11:13，25 分）：“40% goddamn.” ——可作“读者把它读成额度少 40%”的实例（不是媒体） | 同一帖里 r/codex 的 GPT 生成置顶摘要（mod 机器人 dextersummary）写：“That’s not the same as saying quotas were cut by those exact percentages” | `a-reddit-codex-1y5q43m.json` | `images/12-reddit-codex-post-summary.png`（1 倍像素） |
| A6-9 | 未取到的原稿：Business Insider 台湾版 | https://www.businessinsider.tw/article/10313 | — | 搜索结果标题为“OpenAI定價混亂意味著其新人工智慧模型的大多數用戶將更快達到使用上限”，未读到正文 | — | `a-cn-bitw.*` 是 Cloudflare 403 挑战页（blocked，无用） | 无 |

找不到：“变相涨价”类标题——本组没有找到中文媒体直接用“变相涨价”作标题；“变相缩水”一词出现在智东西稿对 Reddit 用户观点的转述（“实际体验仍然是使用额度变相缩水”）。V2EX 帖《OpenAI 又砍额度了》（https://www.v2ex.com/t/1259656）与 LINUX DO 帖《GPT你真无敌了，修正了gpt额度太多的问题》（https://linux.do/t/topic/3239841）只在搜索结果里见到标题，页面 403（Cloudflare），未读正文。ITmedia AI+：未找到。

### A7　热度与质疑（L6，仅作线索）

| 平台 | 对象 | 读数（北京时刻） | 记录 |
|---|---|---|---|
| Hacker News | item 50651117（标题 “OpenAI cuts GPT-6.2 usage: Astra hits limits 40% faster, Sol 25% faster”，Herrera 提交，11-26 11:36:58） | `barking heat` 21:42:51：345 分 / 192 评；22:0x 页面截图：346 分 / 192 评；已知情况 21:20 读 341 分 / 约 187 评 | `a-heat.json`、`a-hn-50651117.json` |
| Reddit | r/codex《Tibo's update on the GPT 6.2 Usage》（u/Eireannish，11-26 10:53:18） | 21:57 浏览器读 JSON：905 分（upvote_ratio 0.96）/ 469 评（已取 291 条）；22:0x 页面显示 902 / 470 | `barking heat` 的 Reddit 请求被重定向到登录页（失败，记入 `a-heat.json` 的 errors）；改用已有浏览器会话在页面内读该帖公开 JSON，写盘前去掉了含会话字段的原始响应，只存帖与评论的公开字段 |
| Reddit | r/codex《Got email from OpenAI say 6.2 astra/sol have less allowance issue》 | 21:56 搜索页：524 票 / 191 评，“1d ago” | 未另存 |
| Reddit | r/codex《Turns it it wasn't people's imagination, and OpenAI's $100 subscription offers significantly less value than competitors. Seems like all of GPT 6.x "efficiency" cost savings aren't being passed on to subscription users.》 | 21:56 搜索页：432 票 / 126 评，“1d ago” | 未另存，只作线索 |
| X | Tibo 原帖回复 | 5226 回复，已读约 96 条 | 样本里多为情绪化回复，无补充说明 |

**有理有据的评论（L6，进正文前须回一手来源）**

1. HN，AmazingTurtle（19:03 北京，API `created_at` 11:03:29Z）：https://news.ycombinator.com/item?id=50653971 ——“Normally I get 4$ API workload with 1% of my 20x weekly sub drained. Last week it went up to ~6.5$ per 1%. So the math is mathing. I don't feel like they're actually "cutting" usage from us, just restoring how it was before…” 个人用量仪表盘，样本 1 人，“API workload 美元”折算口径未说明。可支持“基线是上周被少扣的那一周”。得分未取。
2. HN，revolvingthrow（14:28 北京）：https://news.ycombinator.com/item?id=50652147 ——“…threw an "evaluate this codebase, focusing on architecture and performance" at Astra High, for a ~50k LoC project. It worked for 11 minutes and ate 92% of my 5hr usage on $20 plan.” 单次实测，Plus，Astra High，修正后；与套餐页估计区间（Astra 6–50 条/5 小时）一起看。
3. HN，Aurornis（13:04 北京）：https://news.ycombinator.com/item?id=50651649 ——“Now they’re sending us vague emails about limits being adjusted downward again, citing a billing error? The original email was so vague that they had to send a clarification because it didn’t have enough information to tell if we were going to be retroactively penalized or if it only applied going forward.” 邮件沟通质疑；与 A3-4 对应。
4. HN，krzyk（19:25 北京）：https://news.ycombinator.com/item?id=50654110 ——“I wonder if this is also true for API pricing.”；同一用户 50654142：“Do they plan to change API pricing? If not then most of us in corps won't notice that subs are more pricey.” 对应 A2-6。
5. Reddit，BuildAnything4（84 分）：“i thought astra 6.2 was supposed to be cheaper than astra 6?”；同一用户（11 分）：“If there was a cost improvement so far, it's been negligible. If it's going to cost 40% more allowance, I dont see how we're still getting more usage.” ——把“40% faster”读成“多花 40% 额度”，与“比上一代仍更省”冲突，是基线之争的原型。
6. Reddit，AP_in_Indy（5 分，自称 Pro 500 用户）：“GPT-6.2 Astra is 1/3 as expensive as GPT-6 Astra - not in raw price per token, but it gets tasks done using far fewer tokens. So 1/3*1.4 is still less than half of the cost.” 把官方发布页的“less than a third of the cost”（Agents’ Last Exam 上的 API 成本，非订阅额度）直接乘 1.4——口径混用，须核；官方原句在发布页 Agents’ Last Exam 一段：“GPT-6.2 Astra sets a new high, scoring 5.2 points higher than GPT-6 Astra at less than a third of the cost”（见 `a-openai-intro-62.txt`，未截图）。
7. Reddit，Thomas-Lore（7 分）：“Even before this change Astra 6.2 was eating usage much faster that previous Astra. What you descirbe is API usage, the subscription usage was awful and is now even more awful.” ——与“go further compared to 6 Astra”正面冲突的用户主张，无数据。
8. Reddit，ProcedureEthics2077（15 分）：猜测这是“business decision “mistake””，推测性，无证据，只作情绪样本。

## 二、时间线（北京时间）

| 时刻 | 事件 | 来源 |
|---|---|---|
| 11-18 02:09:24（11-17 18:09 UTC；官方日期 11-17） | GPT-6.2 Astra 与 Sol 上线；Tibo：“…expanded ultrafast to Pro 200 plans for GPT-6.2 Sol” | changelog、X 2122753991000133943 |
| 11-18 02:11:38 | Tibo：“These models are extremely efficient, which you feel in speed to complete tasks, cost and how much further your usage goes” | X 2122754551187132438 |
| 约 11-23 | Pro 100/500 内含 Work 与 Codex 额度翻倍、Pro 200 低档翻倍（官方 Pro 页“Updated: 3 days ago”） | help 9793128 |
| 11-25 01:46:13 | Tibo：开始向少数 Pro 用户推出合并的 Chat 与 Work | X 2125284868414394550 |
| 11-25 11:10:55 起 | 用户贴出 OpenAI 邮件（Codex 内部分 6.2 请求少扣） | X（A3-4） |
| 11-25 约 22:05 | Wayback 快照：套餐页 6.2 额度估计区间未变 | A4-2 |
| 11-26 04:03 | TechCrunch：Chat 与 Work 合并稿（与计费修正无关） | A5-2 |
| 11-26 10:50:10 | Tibo 发布计费修正帖 | A1-1 |
| 11-26 10:53:18 | r/codex 发帖（905 分） | A7 |
| 11-26 10:54 / 10:58:09 | Odaily 快讯 / OpenAIDevs 转发 | A6-3、A1-4 |
| 11-26 11:04:36 | 智东西稿（凤凰网） | A6-1 |
| 11-26 11:36:58 | HN 提交 | A7 |
| 11-26 12:38 / 14:05 | 宝玉xp 微博 / BlockBeats 快讯 | A6-2、A6-4 |
| 11-26 20:01:46 | 套餐页服务器 Last-Modified（文本无变化） | A4-2 |

## 三、可能的吠点（说法 vs 证据，每条一句并注出处）

1. 说法“额度爆砍接近一半”（智东西标题，A6-1）；证据：Tibo 原帖写的是 “reach your limits 40% faster on GPT-6.2 Astra and 25% faster on Sol”（A1-1），官方无“同样的活多扣多少”的数；若按“触顶速度为上周 1.4 倍”推算，同样的活剩上周额度的约 71%（Astra）与 80%（Sol），不是“一半”（推算，非官方；宝玉xp 同算，A6-2）。
2. 说法“快 40%”读成“额度少 40%/时间缩短 40%”（Reddit aliljet、Victorvic1、Notebookcheck 中文版标题，A6-5、A6-8）；证据：原帖没说“少 40%”，也没说“时间缩短 40%”，只说触顶“40% faster”；官方没给换算。
3. 说法“比上一代砍了”（HN 标题 “cuts GPT-6.2 usage”，A6-7；Reddit “OpenAI cuts…”类评论）；证据：40%/25% 的基线是“last week”，即上线后被错误打折的那一周（A2-1），同帖另说 “You should still see these go further in usage compared to 6 Astra and 6.1 Sol”（带 “should”，无数字，A2-3）。
4. 说法“OpenAI 宣称更省、却更快耗尽”（智东西称“事实好像并非如此”，Reddit Thomas-Lore）；证据：官方 changelog 与发布页只对 Sol 明说“more tasks on the same paid plan than with GPT-6.1 Sol”，对 Astra 的订阅额度没有对 GPT-6 Astra 的数字（A2-3）；官方发布页的“less than a third of the cost”是 Agents’ Last Exam 上的任务成本。
5. 套餐页的额度估计：GPT-6.2 Astra 6–50 对 GPT-6 Astra 5–45，GPT-6.2 Sol 17–180 对 GPT-6.1 Sol 15–160（Plus 与 Standard Business，本地消息数/5 小时，官方称“not fixed message limits”）；这张表从上线日（11-18 快照）到 11-26 21:39 一字未改，不知道它对应修正前还是修正后的计量（A4-1、A4-2）。
6. 官方只在邮件里写“Some GPT-6.2 Astra and Sol requests in Codex used less of your allowance than intended”（A3-4），Tibo 帖写 “for most users”（A1-1）；邮件无数字、无“不追缴”、无补偿，Yahoo 香港稿注明“官方電郵沒有交代…補償安排”（A6-6）。
7. “40%/25%”两个数各只给一个，没有区分套餐（Plus、Pro、Business）、没有区分 Standard、Fast、Ultrafast 速度档（Fast 2x、Ultrafast 6x，A3-2）；官方没有说明修正对 Fast/Ultrafast 请求的影响是否同比例。
8. 同一时间段还有另一件“可能更早触顶”的事：12 月起 ChatGPT 与 Codex 共用按 token 计的额度，官方帮助页现已加一段“A small group of users may see the usage changes before December”（A5-1）；TechCrunch 发言人说“a small fraction of its heaviest users may hit their limits sooner”，说的是合并，不是 6.2 计费修正（A5-2）。
9. 同期的上调：Pro 100/500 内含额度翻倍、Pro 200 低档翻倍（官方 Pro 页，A4-4）；该页没有说修正撤销了这次上调，Reddit 用户 Left_Plankton458（35 分）称“they announced last week that they have increased usage limits for all pro accounts, and today they took it away”——官方页面没有这样写（L6，未核）。
10. 智东西“营销方式”一句：来源是一条 Reddit 评论（用户猜测），后半句“从痕迹上看的确有道理……昏招”是记者判断（A6-1b）；官方帖里没有“翻倍”一说。

## 四、夸大说法实例

见 A6 表：最典型的是 A6-1（凤凰网·智东西标题“额度爆砍接近一半？”）、A6-5（Notebookcheck 中文版标题“时间最多缩短 40%”）、A6-7（HN 标题 “cuts GPT-6.2 usage”）、A6-8（Reddit 用户把它读成“少 40%”）。找不到“变相涨价”类中文媒体标题；“变相缩水”见 A6-1 正文转述 Reddit 观点。

## 五、扫描说法勘误（对“已知情况”）

| # | 已知情况的说法 | 独立复核结果 |
|---|---|---|
| 1 | Tibo 原帖正文与时刻（02:50:10 UTC，北京 10:50） | 一致。互动数已涨：见 A1-1 |
| 2 | 原帖没有“Ultrafast”“11 月 17 日”“Codex 通知邮件” | 原帖确实没有。三者各自的出处：Ultrafast 与 6× 规则见官方帮助页（A3-2），邮件原话含 “including in Ultrafast mode”；11-17 见官方 changelog 与 Tibo 上线帖（A3-3）；邮件只有用户晒图与二手转述（A3-4），官方公开页未见 |
| 3 | “TechCrunch 11-25 引发言人‘少数重度用户可能更早触顶’” | 与说法不符：该稿题为“OpenAI says heavy ChatGPT users may hit limits sooner as it merges Chat and Work”，发言人原句是“…while most users are not expected to face more frequent usage resets, a small fraction of its heaviest users may hit their limits sooner.”，讲的是 Chat 与 Work 合并共享额度，全文没有 GPT-6.2、40%、计费错误。时间：12:03 PM PST 11-25＝北京 11-26 04:03（A5-2） |
| 4 | HN item 50651117：341 分 / 约 187 评，北京 11-26 11:36 提交；标题为“cuts usage”写法 | 提交时刻、标题一致；21:42 读 345/192，22:0x 读 346/192；提交者 Herrera，链接指向 twitter.com/thsottiaux |
| 5 | 凤凰网科技含 Codex 通知邮件内容，并写“按用户推测，这可能是 OpenAI 的一种营销方式” | 一致，细节更正：稿件是凤凰网“大风号”平台上的智东西稿，署名“智东西”，作者毕伟豪、编辑漠影；原句是“有用户推测，这可能是OpenAI的一种营销方式。从痕迹上看的确有道理……”，“有用户推测”引自文内嵌入的一条 Reddit 评论图，“从痕迹上看的确有道理……昏招”是记者自己接的判断（A6-1b）。稿件复述邮件内容为转述，邮件原文不在稿里 |
| 6 | 腾讯新闻/Odaily `https://news.qq.com/rain/a/20261126A06U1M00` | 存在，是 Odaily 星球日报快讯，“2026-11-26 10:54发布于北京”，措辞中性（A6-3） |
| 7 | ITmedia AI+ 原始报道 | 未找到（搜索与抓取均无；未做站内检索） |
| 8 | Reddit Codex 社区反对帖超 300 评 | 属实：r/codex 《Tibo's update on the GPT 6.2 Usage》469 评、另有 191 评与 126 评两帖（A7） |
| 9 | 背景：帮助页（北京 11/1 00:17 更新）称 12 月起 Plus、Pro、Business 的 ChatGPT 与 Codex 共用按 token 计的额度，Instant 聊天不扣 | 属实，且现页“Updated: 2 days ago”并**多了一段**“A small group of users may see the usage changes before December so we can learn from early feedback before expanding more broadly.”（A5-1） |
| 9b | “Tibo 公开职务”写法 | 各家不同：X 简介“Codex & ChatGPT @OpenAI”；TechCrunch “product lead”；智东西“ChatGPT和Codex负责人”；BlockBeats“核心产品与平台负责人”；Odaily“Codex负责人”（A1-2） |
| 10 | 宝玉xp 稿等写“API 价格都没变” | 该说法不在 Tibo 原帖（原帖只说 “Other models weren’t affected”），官方 API 价与发布页相同，但官方未就计费错误是否涉及 API 发声（A2-6、A6-2） |

## 六、对读者（已核实事实，每条注出处）

1. 在哪里看自己的额度与重置时间：ChatGPT 的 Settings → Usage（桌面版）；邮件也指向 “Settings → Usage”。出处：help 20001625、20001516、邮件原文（A3-4）。
2. 受影响的是 GPT-6.2 Astra 与 GPT-6.2 Sol；Tibo 称“Other models weren’t affected”，“won’t charge you for the difference on past usage”（A1-1）。
3. 订阅价格不变（邮件原话，A3-4，用户转贴）。
4. 用 API key 登录 Codex 的会话按 API 价格计费，不扣 ChatGPT 套餐额度（套餐页，A2-6）；官方没说这次计费错误是否涉及 API（未找到）。
5. 官方没有公布绝对额度：Plus 与 Standard Business 的套餐页给出“本地消息数/5 小时窗口”估计区间（GPT-6.2 Astra 6–50、Sol 17–180；GPT-6 Astra 5–45、GPT-6.1 Sol 15–160），且声明“not fixed message limits”；Pro 目前没有 5 小时限制，无数字（A4-1、A4-4）。
6. 12 月起 Plus、Pro、Business 的 ChatGPT 与 Codex 共用按 token 计的额度，Instant 聊天不扣；官方帮助页说“A small group of users may see the usage changes before December”（A5-1）。
7. Pro 100/500 内含额度翻倍、Pro 200 低档翻倍，Plus 不变（官方 Pro 页，约 11-23 更新，A4-4）。
8. 想让额度撑得久一点：官方帮助页（20001516，Updated 7 days ago）写“Lower effort … a useful starting point when you want a faster response or to make your allowance go further.”以及“Astra at Low effort can outperform Sol at High effort. If you’ve been getting good results with Sol at High, try Astra at Low or Medium as a starting point.”；同页：“Higher effort can use more of your allowance and does not always produce a better result.”（该页的模型示例写的是 GPT-6 Astra 与 5.6/6.1 Sol，没有专门针对 GPT-6.2 的说明。）
9. Fast 模式对 6.2 按 2 倍扣内含额度，Ultrafast 按 6 倍；Ultrafast 只有 Pro 200（仅 6.2 Sol，仅 Codex）与 Pro 500 有，Plus、Pro 100、Business 没有（套餐页与帮助页 20001275，A3-2）。
10. 不受影响的人：不用 GPT-6.2 Astra 或 Sol 的用户（“Other models weren’t affected”）；套餐页写 Free 与 Go 的 Codex 默认是“GPT-6 Luna at Standard speed in the desktop app”，6.2 的开放名单是 Plus、Pro、Business、Enterprise、Edu（A2-5）。补偿名单含 Go（A1-1）——Go 是否能用 6.2 官方页没写明，未核。
11. 补偿的额度重置只做了逐字存档（A1-1 最后一段），按规定不展开。

## 七、截图清单（`docs/2611/1125/images/`）

| 文件 | 内容 | 备注 |
|---|---|---|
| 01-tibo-post-card.png | Tibo 11-26 原帖卡片（全文可见，含时间、浏览与互动数） | x.com 页面卡片，深色底，约 2 倍；已裁掉导航、头部、回复框 |
| 02-changelog-62-entry.png | 官方 changelog 2026-11-17 GPT-6.2 条目 | 深色底，约 2 倍；底部留有一块黑底空白 |
| 03-pricing-plus-estimates.png | 套餐页 Included usage 节，Plus/Standard Business 额度估计表（6.2 与前代同表） | 深色底，约 2 倍 |
| 04-help-usage-limits.png | 帮助中心《Understanding ChatGPT and Codex usage limits》全文（含新增段） | 页头“Updated”被浏览器本机语种渲染为“前天” |
| 05-ifeng-zhidx-title-lede.png | 凤凰网·智东西稿标题、署名时间与导语（含“40%和25%”句） | 约 2 倍；页内广告条 |
| 06-ifeng-zhidx-email-marketing.png | 同稿：邮件段、转述“仍会发现…但事实好像并非如此”、“营销方式”句与 Reddit 评论图 | 由 4 段拼成（间隔 28 px，已去掉段间的广告留白），约 2 倍 |
| 07-hn-title-score-v2.png | HN 帖标题与得分（346 分/192 评）与顶评 Aurornis | 2 段拼接，约 2 倍；`07-hn-title-score.png` 为底部切到“reply”半行的初版，见待删除 |
| 08-hn-comments-quota-data-v2.png | HN 三条评论：AmazingTurtle、revolvingthrow、BikiniPrince | 3 段拼接，约 2 倍；底边有一线回复框边缘；`08-hn-comments-quota-data.png` 为初版，见待删除 |
| 09-openai-launch-usage-sentence.png | OpenAI 发布页首段“fewer output tokens, so you can get more done within your usage limits” | 深色底，约 2 倍；`09-openai-launch-usage-claim.png` 为含浏览器翻译浮标的初版，见待删除 |
| 10-pricing-token-rates.png | 套餐页 Token rates 表（6.2 Astra 对 GPT-6 Astra 等）及“Credit prices alone don’t determine included subscription usage” | 深色底，约 2 倍 |
| 11-tibo-launch-efficiency-post.png | Tibo 11-18 02:11 帖“extremely efficient … how much further your usage goes”与 Agents’ Last Exam 图 | 截取时顶部账号头部被页面粘性栏遮挡，卡片只含正文、图与互动数；深色底 |
| 12-reddit-codex-post-summary.png | r/codex 帖（905 分，内嵌 Tibo 帖图）与 GPT 生成的置顶摘要 | 1 倍像素（450 px 宽），仅作补充；卡内嵌图的时间是 UTC（“2:50 AM”） |

未做：A 组候选配图里“03 套餐额度页”“04 现行版帮助页”“05–06 中文标题”“07–08 HN/评论”均已有；“官方帮助页/changelog 的计费修正条目”不存在，所以没有对应截图。
