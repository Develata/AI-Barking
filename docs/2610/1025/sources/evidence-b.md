# 1025 期 B 组取证：Kimi K3.1 Pro 额度事故与补偿【跟进1023/1024】

执行方：claude-sonnet-5-5。取证时段：2026-10-26 20:34 – 21:30（北京时间）。时间一律北京时间（UTC+8），括号内保留原时区；本机美东（EDT），北京 = 本机 + 12 小时。X 数据（浏览/赞）是取证时读到的瞬时值。文件均在 `docs/2610/1025/sources/`（前缀 `b-`）与 `docs/2610/1025/images/`（13–23 号）。抓取明细见 `capture-log-b.md`。

## 一、核心结论（只记取证事实）

- 截图局限：13 号图是被截断的嵌入卡（缺 25%/清零/PST 句），完整英文请用 24 号图或 fxtwitter JSON；17/18/19 号图是 1× 像素，24 号约 1.4×，其余 2×。

- 官方（X @Kimi_Moonshot，2026-10-26 04:14:44 北京 / 2026-10-25 20:14:44 UTC）承认“These were our issues, and we should have fixed them sooner”，补偿口径原文为“restoring 25% of their monthly quota (usage below 25% resets to zero), equal to one week of usage”。官方帖与中文公告都没写：受影响人数、异常起止时段、根因、5 小时窗口是否同样重置、套餐绝对额度。
- 中文侧官方同步发了公告，标题《致用户：关于近期使用体验问题的致歉与补偿》（署名 KIMI），渠道为“Kimi智能助手”账号（微博 @月之暗面Kimi；中新经纬称图片来自其小红书账号）。全文见 B3。中文稿措辞：“所有档位的订阅会员”“所有有效期内的会员都将获得月度总额的 25% 作为补偿，相当于一周的用量”“用量进度条会在今天结束前显示调整结果”，**不含 PST 字样，也不分国内/海外**。官方微博原帖因未登录读不到（见 capture-log），中文原文以 IT之家转录的“官方原文”为准（L4，附官方稿件截图见 14 号图）。
- 官方 X 帖发出前 8 小时（2026-10-25 20:06:51 北京），@KimiDevs 已发更新说明：Kimi Code CLI 2.3.2 “Handle tasks directly by default instead of proactively launching subagents”，Desktop v1.1.3 “The Agent no longer initiates a swarm on its own; swarm only runs when swarm mode is enabled”。这是“减少不必要的子 Agent 调用”最具体的一手出处；官方道歉帖没有把二者连起来，两者的因果是推断，不是官方说法。
- 套餐与额度单位：Kimi 官方文档写明有三种计量——滚动 5 小时窗口、周额度（仅旧套餐）、月度总额；新会员取消周额度。公开页面**没有给任何绝对额度数字**（只有“Agent 额度”倍数，海外 Plus 写“更多”、Pro 2 倍、Max 5 倍、Ultra 10 倍；国内 Go“更多”、Plus 2 倍、Pro 4 倍、Max 14 倍、Ultra 28 倍）。
- 25% 与“一周”：文档里旧套餐“每 7 天自动刷新”，月度总额则另算；官方没说周额度占月度多少。28 天的月份下 7/28 = 25%，自洽；30 天的月份是 23.3%。一份 Reddit 旧套餐用户在 K3 上的自测（2026-09-27，L6）折算周额度约为月度的 22.7%。新套餐无周额度，“一周的用量”对他们只是类比。

## 二、逐条证据表

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

### B1 官方道歉与补偿帖

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 官方道歉并补偿，全文、时刻、热度 | L1 | https://x.com/Kimi_Moonshot/status/2114450610724708492 （存档 `b-fx-2114450610724708492.json`） | “We're sorry for the rate limits and unnecessary quota consumption on the K3.1 Pro subscription plan. These were our issues, and we should have fixed them sooner.” / “Today, every subscriber on every plan gets a one-time usage reset, restoring 25% of their monthly quota (usage below 25% resets to zero), equal to one week of usage. Your usage bar will reflect this adjustment by end of day PST.” / “We've also adjusted the plan's parameters and are reducing unnecessary subagent calls and output length, so your quota goes further.” / “Thank you for your patience while we put this right.” | `13-kimi-apology-card-raw.png`（X 官方匿名嵌入卡，2×；**把长帖截断在“usage below Show more”**）；`24-kimi-apology-fulltext-raw.png`（x.com 帖子页点“显示原文”后的英文全文，只裁帖子正文与计数行；登录态页面，约 1.4× 像素，界面为中文，含 X 的“显示翻译”小标签；不含侧栏、头像、回复框）；中文官方稿见 14 号 | 发布：2026-10-26 04:14:44 北京（2026-10-25 20:14:44 UTC；美东 EDT 16:14；太平洋时间 13:14）。热度（fxtwitter，20:33 北京）：524,258 浏览、4,779 赞、168 转、368 回复、273 引用、353 书签；opencli 20:20/21:10 读到约 52.1 万浏览、4,766→4,805 赞，数字持续变化。`is_note_tweet: true`（长帖）。无社区注释。账号为 Kimi.ai 组织认证号，粉丝 387,162。 | 已找到 |

### B2 补偿口径

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 25% 月额度、不足 25% 清零、“约一周用量”、到账时间、覆盖套餐 | L1 | B1 官方帖；中文稿（IT之家转录）https://tech.ifeng.com/c/8xWrSDBLrWR | EN：“every subscriber on every plan”“restoring 25% of their monthly quota (usage below 25% resets to zero), equal to one week of usage”“by end of day PST”。CN：“今天，我们会为所有档位的订阅会员进行一次重置：所有有效期内的会员都将获得月度总额的 25% 作为补偿，相当于一周的用量。如果当前用量不足月度总额的 25%，将重置为 0。用量进度条会在今天结束前显示调整结果。” | 13、14 | ① 对象：官方只写“subscriber / 订阅会员”（有效期内）。Open Platform API 是按量付费、无订阅（1024 存档 `a-kimi-plans.txt`：“The Kimi API Open Platform uses pay-as-you-go billing with no subscription plan”），字面上不在范围；企业版、Extra Usage 余额官方未提。② 方向：写的是回拨“已用量”（usage）25 个百分点、不足则清零；不是新增额度、不是退款、不是降价。官方没有明说“减去 25 个百分点”，这是对 “restoring 25% of their monthly quota (usage below 25% resets to zero)” 的读法，与用户读法（Reddit/X）一致，见 B7。③ 到账：英文“end of day PST”，中文“今天结束前”，**两版时区口径不同**；帖发于太平洋时间 10-25 13:14，当时美西为 PDT（夏令时 11-01 才结束）。英文“today”按美西是 10-25，中文“今天”按北京是 10-26。④ 5 小时窗口、周额度是否同步重置：官方未提（旧套餐用户的反馈见 B7）。⑤ “一周”：见本文第一节末段。**状态说明：官方原文已找到；读法（回拨已用量）、对象边界、时区口径为推断或官方未给。** | 部分支持 |
| 额度单位与套餐名 | L1 | Kimi Code 文档 https://www.kimi.com/code/docs/en/kimi-code/membership.html （存档 `b-code-member-en.txt`/`.html`，另有 `b-code-member-zh.*`）；海外定价页 https://www.kimi.ai/membership/pricing ；国内定价页 https://www.kimi.com/membership/pricing （均 JS 渲染，文本存 `b-kimiai-pricing-opencli.txt`、`b-kimicom-pricing-opencli.txt`） | 文档：“For new members, the weekly quota limit is removed, and only the rolling 5-hour rate window remains.”“Existing members on legacy plans: rules are unchanged — quota refreshes automatically every 7 days from your subscription date, unused quota does not roll over, and the same 5-hour rolling rate window applies.”“Both groups share the Kimi membership monthly total quota: if the monthly total is reached, Kimi Code quota is frozen until the monthly quota resets or you upgrade.”“Kimi Code is the developer-focused AI coding service within Kimi membership benefits, provided together with a Kimi membership subscription and sharing the same quota.” | 15（文档规则）、17、18（海外）、19（国内） | 海外 kimi.ai（美元，包年折算价/月价）：Plus $15/$19、Pro $31/$39、Max $79/$99、Ultra $159/$199；国内 kimi.com（人民币）：Go ¥39/¥49、Plus ¥79/¥99、Pro ¥159/¥199、Max ¥559/¥699、Ultra ¥1,119/¥1,399。两个站是**不同的价目表**（币种与客服邮箱 moonshot.ai / moonshot.cn 不同）。文档的旧套餐名 Andante/Moderato/Allegretto 与新名对应：K3.1 Pro 为“Plus / Moderato and above”，1M 上下文为“Pro / Allegretto and above”。定价页只给“Agent 额度”倍数，无绝对数字；5 小时窗口/月度总额的绝对值全站未公开。截图 17/18/19 为 1× 像素（桌面版式 1400 px 宽；该页在 700 px 宽下是手机轮播版式，不适合截），页面界面为中文（浏览器语言），未登录态；19 号图 Ultra 卡片被轮播截断，完整文本见 `b-kimicom-pricing-opencli.txt`。 | 已找到 |

### B3 中文侧官方是否同步公告、国内订阅是否同样补偿

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 中文官方有同步公告，措辞与英文基本一致 | L4（官方原文的媒体转录；官方微博/公众号原帖本人未能直接读到） | IT之家转录（凤凰网）https://tech.ifeng.com/c/8xWrSDBLrWR （2026-10-26 07:17:51）；财经网科技微博转录（新浪）https://www.sina.cn/weibo/detail/5358179195292624.html （08:02）；财联社（21经济网）https://m.21jingji.com/article/20261026/herald/bf1e90e1d6a314bec72c3dac18eb3625.html （08:57）；澎湃号·央视财经 https://www.thepaper.cn/newsDetail_forward_34335214 （18:18，称“@月之暗面Kimi 在微博发文”） | 《致用户：关于近期使用体验问题的致歉与补偿》“致 Kimi 用户 对于近期 Kimi 会员使用 K3.1 Pro 时出现的限流及不必要的额度消耗，我们向大家致歉。问题出在我们，我们也本应更早修复。”“今天，我们会为所有档位的订阅会员进行一次重置：所有有效期内的会员都将获得月度总额的 25% 作为补偿，相当于一周的用量。如果当前用量不足月度总额的 25%，将重置为 0。用量进度条会在今天结束前显示调整结果。”“同时，我们已调整配置，并正在减少不必要的子 Agent 调用和输出长度，让同样的额度完成更多实际任务。”“感谢您的理解与耐心等待。 KIMI” | 14（IT之家转录页，含“官方原文”标注与标题时间） | 发布渠道：多家媒体称“Kimi智能助手”“凌晨”发文；上海证券报称标题为《致用户：关于近期使用体验问题的致歉与补偿》；中新经纬称图片来源为“Kimi智能助手”小红书账号。本人只读到上述转录，**官方微博原帖需登录（未登录，不绕）、公众号原文未检索到**，因此官方发出的精确时刻未知；最早的媒体转录是 07:17:51（北京）。中文稿与英文稿的差异：中文没有“PST”，没有“every plan”之外的区分，用“月度总额”对应 “monthly quota”，“调整配置”对应 “adjusted the plan's parameters”（英文是“plan's parameters”，中文是“配置”）。**国内订阅是否同样补偿**：中文稿面向“所有档位的订阅会员”，没有排除国内；但没有单独写国内/海外，也没有 kimi.com 站内公告页的存档，故只能说“中文公告覆盖‘所有档位’，未区分市场”。 | 部分支持 |
| 公告未找到的渠道 | — | kimi.com 站内公告 / 帮助中心 / 微信公众号原文 | — | — | 搜索（WebSearch 两次）只得到媒体转载；Kimi 公众号原文未检索到；kimi.com 帮助中心未查到相关页面。 | 未找到一手来源 |

### B4 “unnecessary subagent calls and output length” 指什么

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 子智能体功能与此次调整 | L1 | @KimiDevs 帖（存档 `b-fx-2114327830465237001.json`、`…2114327832646004990.json`、`…2114327835930472778.json`、`…2114327838140592411.json`）https://x.com/KimiDevs/status/2114327830465237001 | “We’ve optimized subagent invocation in Kimi Code CLI and the Kimi Code Desktop app for K3.1 Pro, improving orchestration and collaboration on complex tasks.” / “Kimi Code CLI 2.3.2 Handle tasks directly by default instead of proactively launching subagents; request delegation explicitly in your message, AGENTS.md, a skill, or an enabled multi-agent mode.” / “Agent swarms now start only when you turn on swarm mode: the agent can no longer launch AgentSwarm by itself, so swarms are no longer available in kimi -p or ACP clients such as Zed.” / “Kimi Code Desktop v1.1.3 By default, the Agent completes tasks on its own. To have it delegate tasks, explicitly request delegation or enable swarm mode. The Agent no longer initiates a swarm on its own; swarm only runs when swarm mode is enabled.” | 20、21、22（X 嵌入卡；21、22 卡片里带父帖上下文） | 时刻：2026-10-25 20:06:51–20:06:53 北京（12:06:51 UTC），比道歉帖早约 8 小时 8 分。热度：首帖 923 赞、71,760 浏览。范围：只说 Kimi Code CLI 与 Kimi Code Desktop；kimi.com 网页/App 的 Swarm（集群功能）是否改动未写。该帖**没有**与额度事故挂钩。 | 已找到 |
| 子智能体会各自耗 token；用户能否控制 | L1 | Kimi Code 文档“Agents and Sub-Agents”https://www.kimi.com/code/docs/en/kimi-code-cli/customization/agents.html （存档 `b-code-agents.txt`/`.html`） | “Sub-agents are scheduled automatically by the main Agent, based on task complexity, context consumption, and sub-task independence.”“Note that each sub-agent independently consumes model tokens. For simple tasks, there is no need to dispatch a sub-agent; the main Agent handles them more economically.”“If you need a particular type of tool to be permanently unavailable inside sub-agents, tighten the corresponding permission rule on the main Agent.” | 16 | 文档没有“一键关闭子智能体”的开关说明；可通过权限规则、`disallowedTools`/`subagents` 白名单（agent 文件字段）限制。Tower 多智能体文档另写：“Each worker and reviewer has an independent model context and consumes tokens separately… More agents increase pressure on quota, CPU, and disk.”（`b-code-tower.txt`）。上述文档是取证时（10-26 20:34）的版本，早于/晚于 10-25 更新的先后无法由页面判定（What's New 页最新条目仍是 10-23 K3.1 Pro 上线，未列 2.3.2）。另：What's New 的 K3.1 Pro 条目写“Start a new session before use: Do not switch to the new models in existing K3 series, K2.8 Preview, or K2.7 Code Highspeed sessions, as this may cause unexpected model behavior and reduce cache hit rates.” | 已找到 |
| “output length” 指什么 | — | 官方道歉帖与中文稿 | “reducing unnecessary subagent calls and output length” / “减少不必要的子 Agent 调用和输出长度” | — | 官方只写了这一句，未说明怎么减、减多少。Reuters 的解释（“the total number of tokens generated in response to a prompt”）是记者对术语的说明，不是 Moonshot 的话。 | 未找到一手来源（具体机制） |

### B5 事故时间线

K3.1 Pro 发布：官方 X 帖 2026-10-24 01:15:24 北京（10-23 17:15:24 UTC；美东 13:15）；Kimi Code 文档 What's New 条目日期写“October 23, 2026”（按美国日期）。

| 时刻（北京） | 事件 | 来源 | 级 |
|---|---|---|---|
| 10-24 01:15 | K3.1 Pro 发布 | https://x.com/Kimi_Moonshot/status/2113680701770068067 | L1 |
| 10-24 02:14 | r/kimi 帖“how good are kimi limits? On Plans?”（发布当天已有额度疑问） | https://www.reddit.com/r/kimi/（opencli 列表，`b-reddit-kimi-new-opencli.json`） | L6 |
| 10-24 23:16 | HN Ask HN 帖：$19 Kimi Code Plus 用户称两个任务未完成，5 小时额度数分钟用完 | HN 50222093（存档 `b-hn-50222093.json`、`b-algolia-item-50222093.json`） | L6 |
| 10-25 11:08 | r/kimi 帖“Even the ultra subscription is unusable for coding, or is it just me?”（Ultra 用户称一个提示烧掉整个会话额度；其自述：K3.1 Pro 思考强度 max、swarm 开启、每个 swarm 子代理限 2–3 个） | https://www.reddit.com/r/kimi/comments/1xerert/ | L6 |
| 10-25 20:06 | @KimiDevs：CLI 2.3.2/Desktop 1.1.3 默认不再主动启动子智能体 | B4 | L1 |
| 10-26 03:03–03:05 | HN 帖作者回复：官方客服“refused to help and said you used it heavily”；“Quota has slightly improved since yesterday” | HN 50222093 评论（截图 23） | L6 |
| 10-26 04:14 | 官方 X 道歉与补偿 | B1 | L1 |
| 10-26 04:14 之后 | 官方此前**未**在 @Kimi_Moonshot 时间线上就额度发过声明：10-23 17:20 UTC 之后到 10-25 20:14 UTC 之间该账号列表里无帖（opencli `tweets Kimi_Moonshot --limit 25`，该命令可能不含回复，未入库）；官方回复帖与状态页本人未逐一检索，记“未查到”不等于“不存在” | — | — |

代表性用户投诉（数字均为其自述，L6）：
1. HN 帖（作者披露“I also develop a competing coding agent”）：“I paid $19 for Kimi Code to test K3.1 Pro Standard (not Highspeed) using the official CLI v2.3.1.”“The entire 5-hour quota was exhausted within minutes.”“Fresh input: 368,203 tokens - Cached input: 1,183,744 tokens - Output: 39,324 tokens - Total: 1.59M tokens”“that's just $1.37 worth of usage before hitting the 5-hour limit”；“The logs show: "429 You've reached your 5-hour usage limit."”。得分 6、3 评（取证时 21:04 北京）。截图 23。
2. Reddit u/LessComplexity（r/kimi，10-25 11:08 北京，得分 15、12 评）：“K3.1 pro, max with swarm on, capped on sub-agents per swarm (2-3 max).”“this is literally all my session on a single prompt and screen.”（帖内截图未单独存档，无具体数字）。https://www.reddit.com/r/kimi/comments/1xerert/
3. X 评论区（道歉帖下，取证时读到；全文存 `b-x-replies-opencli.json`）：@Sojiooo1“The worst is that you can use all your monthly limit in a day”（10-26 19:04 北京）；@HalfAToken“So if I read it right, someone at 80% used drops to 55%, and anyone under 25% goes back to zero. Would be nice to also know when the subagent cut lands.”（18:36 北京）；@aiplay_ace（中文）“明天上午就重置了，你现在给我25%的额度？？？”（19:53 北京）。

### B6 媒体报道

| 媒体 | 发布时刻（北京） | URL | 备注 | 存档 |
|---|---|---|---|---|
| Reuters（原站） | 10-26 10:22（02:22 UTC，据 Yahoo 转载页） | https://www.reuters.com/world/asia-pacific/moonshot-resets-subscriber-quotas-after-kimi-k31-pro-launch-issues-2026-10-26/ | 原站 HTTP 401，**未绕**。同稿经 Yahoo 的 Reuters 授权转载读到：https://www.yahoo.com/news/articles/moonshot-resets-subscriber-quotas-kimi-022218055.html （`datePublished` 2026-10-26T02:22:18Z）。是否可当“Reuters 原文”引用由 Claude 决定（转载站规则：仅为核对内容，未作为一手来源）。TradingView 页只有标题（付费墙）。 | `b-yahoo-reuters.*` |
| 21经济网/21世纪商业评论 | 10-26 17:00 | https://www.21jingji.com/article/20261026/herald/2791714cb2b7ecabf9e5625b4fe46a6b.html | 标题称“2.8万亿参数新模型额度异常消耗，Kimi道歉并补偿一周用量” | `b-21jingji.*` |
| 新浪科技（TechWeb） | 10-26 约上午 | https://finance.sina.com.cn/tech/roll/2026-10-26/doc-iniwpfmx9297229.shtml | 标题“全体订阅会员获25%额度补偿” | `b-sina-finance.*` |
| IT之家（凤凰网转） | 10-26 07:17:51 | B3 | 最早的中文全文转录 | `b-ifeng.*` |
| PANews（英文） | 10-26 04:55（20:55 UTC 10-25） | https://panews.io/articles/01a174d8-848e-747c-b579-5c42570d1c39 | 引用 X 帖链接 | `b-panews.*` |
| BigGo Finance | 10-26 07:55（23:55 UTC） | https://finance.biggo.com/news/e02d8913-0cbe-4541-8919-fc83861cacdb | 见 B8（对“usage below 25%”的转述有误） | `b-biggo.*` |

Reuters 全文要点（逐字）：“BEIJING, Oct 26 (Reuters) - Chinese artificial intelligence startup Moonshot AI apologised on Monday for rate limiting and unnecessary quota consumption after its Kimi K3.1 Pro launch.”“Beijing-based Moonshot said on its Kimi X account that it was offering to reset the monthly quota of all subscribers by 25%, which it said equals one week's usage”“Subscribers with usage below 25% reset to zero”“Moonshot previously had issues with computing capacity after K3 was launched”“Chinese AI labs are suffering from computing shortages following US chip export controls”。后两条是 Reuters 自己的背景句，不是 Moonshot 的说法。作者署名 Liz Lee、Laurie Chen。

### B7 补偿后用户反馈（只记有具体数字的自述，L6）

| 反馈 | 来源 | 时刻 | 备注 |
|---|---|---|---|
| “I’m on vivace and was at about ~20% usage which suddenly decreased to 1% well before my monthly reset.” | r/kimi https://www.reddit.com/r/kimi/comments/1xflkq6/ （15 分、6 评） | 10-26 10:59 北京（02:59 UTC） | 额度条按说法下降（用户自述；约 20% 到 1%，与“不足 25% 清零”不完全吻合，可能是取整或口径；档位名 vivace 官方文档未列） |
| “oh fucking awesome...I was all tapped out at my weekly limit. It reset! My monthly quota didn't reset but dropped by about 20% so it looks like they got raised?” | 同帖评论 u/CrimsonBolt33（5 分） | 同日 | 旧套餐用户称周额度也重置、月度条降约 20%，称“looks like they got raised”（其读法）。官方帖只讲月度，周额度/5 小时窗口是否同步由此只能算用户自述 |
| “No, they reset the usage one time, u also now have a usage reset that u can use.” | 同帖评论（8 分） | 同日 | 后半句（“还有一个可用的重置”）官方帖未写，疑似与别家“banked reset”混淆，**不要采用** |
| “Quota has slightly improved since yesterday but still nowhere near claude code or other agents.” | HN 50222093 | 10-26 03:05 北京 | 发在官方补偿帖之前 1 小时，说明额度在道歉前已被小幅调整（自述） |
| 限流是否缓解 | — | — | 未找到带具体数字的“补偿后限流缓解”自述；X 评论区只有情绪表态（“thank you kimi”“Using Claude here, no monthly limit…”等）。 | 

### B8 夸大/走样说法实例

| 说法 | 原站 URL | 原句（逐字） | 发布时间（北京） / 发布方 | 与官方的差异 |
|---|---|---|---|---|
| “降价/退款/免费送一周”类 | — | — | — | 搜索英文与中文均未找到把补偿说成“降价 25%”“退款 25%”“免费送一周”的标题或文章（取证时段内）。 |
| 把“usage below 25%”转述成“remaining usage” | https://finance.biggo.com/news/e02d8913-0cbe-4541-8919-fc83861cacdb | “members whose current remaining usage is already below one quarter of their monthly total will be reset to zero” | 2026-10-26 07:55，BigGo Finance | 官方写的是 “usage below 25%”（已用量不足 25%），BigGo 写成“剩余用量不足四分之一”，含义相反。同页摘要另写 “reset the monthly quota of all active paid subscribers by 25%”（沿 Reuters 措辞）。 |
| “额度乱扣/误扣/偷偷消耗” | https://www.sina.cn/weibo/detail/5358239585669549.html ；https://www.sina.cn/weibo/detail/5358211177122741.html ；https://www.sohu.com/a/1092756781_100085330 | 标题“Kimi 致歉额度乱扣：补偿 25% 额度，优化配置减少无效调用”；标题“Kimi 凌晨致歉：会员额度被误扣，补偿相当于用一周”（上海证券报）；“简单来说，就是改掉模型 “偷偷消耗额度” 的 bug，避免无效调用浪费会员的权益。”（数码八叔） | 新浪新闻聚合 / 上海证券报（2026-10-26 10:09 微博转发）/ 搜狐号“数码八叔”（10-26 10:31） | 官方原话是“不必要的额度消耗”（unnecessary quota consumption）；官方没有说“误扣”“乱扣”“偷偷消耗”，也没有说是 bug。 |
| 把用户/开发者归因写成确定的因果 | https://m.sohu.com/a/1092933354_120321309 | “有开发者反映，模型在任务中自行启动了较多子智能体、输出冗长，是额度异常消耗的主要原因。”“官方坦承，问题集中在两处：一是模型出现限流…二是用户的额度流失。” | ZAKER/星河商业观察，2026-10-26 16:52（搜狐转） | “开发者”未具名；“官方坦承…集中在两处”是作者概括，官方帖只写了“限流”和“不必要的额度消耗”两个现象，没有写根因。该文还称“今年7月…公司一度宣布暂停C端新用户订阅”，**本人未核**。 |
| 官方没写的技术动作 | https://news.qq.com/rain/a/20261026A0501500 | “核心优化方向是削减不必要的子Agent调用、精简冗余输出长度，优化模型调用链路，让相同额度支撑更多有效任务，从根源降低无效额度损耗，缓解限流现象。” | 金融界，2026-10-26 10:33（腾讯新闻） | “优化模型调用链路”“从根源降低…缓解限流”均不在官方稿中；官方只说“调整配置、减少子 Agent 调用和输出长度”，没有说限流已缓解。 |
| 把“回拨已用量”写成“发放/补上额度” | https://weibo.com/2/detail/5358242814755360 ；https://www.sina.cn/weibo/detail/5358332854405276.html | 标题“Kimi致歉并宣布补偿，发放25%月会员额度”（新浪数码 VIP 微博，页面显示“9小时前”，取证约 21:30 北京，发布时刻约 12 点前后，未精确）；“官方后面出来道歉，也给会员补上 25% 的月度额度作为补偿”（微博用户“是象象阿”，2026-10-26 18:12，经新浪新闻聚合） | 新浪数码（媒体号）/ 个人博主 | 官方写的是 “restoring 25% of their monthly quota (usage below 25% resets to zero)”：回拨已用量，不是新发放 25% 额度；“发放”“补上”会让人读成额度增加。后一条同时是用户自述（L6）：“额度掉得比预想快很多，中途还碰到过限流”（无数字）。 |
| Reuters 措辞歧义 | Yahoo 转载页（B6） | “offering to reset the monthly quota of all subscribers by 25%” | Reuters，10-26 10:22 | 容易被读成“月度额度被重置/调整了 25%”，实际官方是“已用量回拨 25 个百分点（上限）”。 |

## 三、可能的吠点（原文里有、摘要没提或说反的条件；每条注出处）

1. 官方补偿的上限是月度总额的 25%：已用量低于 25% 的人，只拿回已用的那部分（官方原话 “usage below 25% resets to zero”）；没用多少的订阅者几乎没有收益。（B1）
2. 补偿对象写的是 subscribers/订阅会员；Open Platform API 是按量付费、无订阅，字面上不在范围（1024 存档 `a-kimi-plans.txt`）。官方没有写 Extra Usage 余额、企业版如何处理。
3. “相当于一周用量”对应旧套餐的 7 天周额度；文档称新会员已取消周额度，只有 5 小时窗口和月度总额（`b-code-member-en.txt`）。多数用户反映的 429 是 5 小时窗口；官方帖只说月度额度回拨，没说 5 小时窗口怎么处理。
4. 到账时刻英文写“end of day PST”，中文写“今天结束前”，两版口径不同；美西当时是 PDT。BigGo 把英文版换算为“10 月 26 日下午（北京）”，是其推算。
5. 官方道歉帖前 8 小时，@KimiDevs 已把 CLI 2.3.2/Desktop 1.1.3 改为默认不主动启动子智能体、Swarm 仅在手动开启时运行——也就是“减少不必要的子 Agent 调用”至少一部分是默认行为的回退，范围限于 Kimi Code CLI/Desktop。
6. 官方文档承认每个子智能体“independently consumes model tokens”，并建议简单任务不要派子智能体（agents.html）。
7. 官方文档建议：不要把已有的 K3/K2.8/K2.7 会话直接切到 K3.1 Pro，“may cause unexpected model behavior and reduce cache hit rates”（What's New，`b-code-whatsnew.txt`）——缓存命中率下降会推高成本，但 Kimi 没有把它说成额度事故的原因，勿写成因果。
8. 道歉帖没有说原因（算力、计量 bug 还是默认配置），没有说持续多久、影响多少人。Reuters 补了一句背景“Moonshot previously had issues with computing capacity after K3 was launched”，是 Reuters 的话。
9. HN 发帖人披露自己开发竞品；总 token 1.59M 里 1.18M 是缓存输入；“$1.37”是按 Kimi 公布的 API 价格折算，不是 Kimi 订阅的额度数。Kimi 未公布 Plus 的 5 小时额度绝对值，所以无法由官方资料核实“$1.37 就触顶”。
10. 同一 HN 发帖人称官方客服先回复“you used it heavily”（自述），早于官方道歉约 1 天以上。
11. 社区质疑（L6，仅作线索）：X 评论区 @RailguardInc（0 赞，10-26 19:20 北京）：“But the damage is already done. Short usage limits and poor quality that's what Kimi k3 is now known for.”；星河商业观察转述的中文评论称“额度消耗不透明”“有用户…向客服申请售后时直接被告知‘不行’”（二手，未核）。Reddit 另有旧套餐用户在上月测得“5h 窗口 ≈ 周额度 20%，周额度 ≈ 月度 22.7%”（r/kimi 1wqjghr，2026-09-27，K3 非 K3.1，仅作 25%≈一周的旁证）。

## 四、夸大说法实例

见 B8。结论：未找到“降价 25%/退款 25%/免费送一周”；找到的主要走样是（a）BigGo 把“已用量不足 25%”写成“剩余用量不足四分之一”，（b）中文聚合稿把“不必要的额度消耗”写成“乱扣/误扣/偷偷消耗额度的 bug”，（c）把“调整配置”扩写成“优化模型调用链路、从根源缓解限流”。

## 五、扫描说法勘误（对照派工单“已知情况”）

- 官方 X 帖时刻、全文：与已知情况一致（04:14:44 北京）。浏览/赞/转/回复：fxtwitter 20:33 读到 524,258 / 4,779 / 168 / 368；与“约 52.1 万、4766、168、368”一致，数字在漂移。
- “every subscriber on every plan 是否包括国内”：已知情况待核，核得：中文官方稿同样写“所有档位的订阅会员”，并存在中文版公告；但中英文稿没有区分国内/海外，国内/海外是两个价目表（kimi.com 人民币、kimi.ai 美元），官方未明说两边是否都执行。
- Reuters：已知情况称 401 未核。核得：原站仍 401（未绕）；Yahoo 的授权转载页可读，02:22:18 UTC（北京 10:22）与扫描所称“10/26 02:22 UTC”一致，内容见 B6。
- “官方帖未给…‘subagent’具体指什么”：现补得一手线索——@KimiDevs 10-25 20:06 北京的更新说明（B4）；但官方没有把它与道歉挂钩。
- 已知情况里“1024 期存档 InfoQ/36氪：$99 套餐用户单会话、未开 Agent 群组，约 47 分钟触到 5 小时额度”本次未重核（只读参考）。注意：海外 Max 为 $99，Plus 为 $19；HN 用户是 $19 Plus。
- 发布时间：K3.1 Pro 文档写“October 23, 2026”，官方 X 帖为北京 10-24 01:15（UTC 10-23 17:15），两者是同一事件的不同时区日期，不矛盾。
