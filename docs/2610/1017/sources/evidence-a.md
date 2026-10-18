# 1017 期 A 组取证：OpenCode Console 泄露（1.4% 没给分母、“蒸馏攻击”未见证据）

执行方：claude-sonnet-5-5（Claude Code 子代理）。时刻一律北京时间（UTC+8），括号保留原时区。读取时刻见 `capture-log-a.md`（约 2026-10-18 19:40–21:40）。
级别按 EDITORIAL.md：L1 官方、L2 员工/当事人署名发言、L4 权威媒体、L5 二手/聚合、L6 社交转述、L7 截图传闻。X 帖均取自 fxtwitter 公开 JSON、X 官方 syndication 与匿名嵌入页，存档里无抓取者信息。

## 0. 时间线（只含已存档证据）

| 北京时间 | UTC | 事件 | 来源 |
|---|---|---|---|
| 10-17 23:18:36 | 15:18:36 | Matt Johansen（@mattjay，自述“AI and Cybersecurity news”从业者）发帖：“Seems OpenCode customers getting notices of a breach. Got some DMs that people’s API keys are running up charges.” | `a-fx-2111476982580211851.raw.json` |
| 10-17 23:30:37 | 15:30:37 | 同人再发：“Confirmed. OpenCode hacked. Keys stolen. Telling customers to revoke.”（带邮件截图；图未取） | `a-fx-2111480005435719885.raw.json` |
| 10-18 01:19（HN 投稿时刻） | 17:19:19 | HN 50133006 投稿 Reddit 帖“Potential provider key breach”（Reddit 帖本身的发帖时刻未取得，不晚于此） | `a-hn-50133006.json` |
| 10-18 01:56:19 | 17:56:19 | OpenCode 官方帖初版（id …935833）：到 “but we are working with” 戛然而止，无 “If you did not get an email…” 一句 | `a-fx-2111516673953935833.raw.json` |
| 10-18 02:01:41 | 18:01:41 | HN 50133401 投稿，链接的是初版 …935833 | `a-hn-50133401.json` |
| 10-18 02:02:54 | 18:02:54 | HN 首条评论贴邮件正文 | 同上 |
| 10-18 02:08:06 | 18:08:06 | 官方帖编辑版（id …319818）：补上 “If you did not get an email you were not impacted…” 与 “working with Anthropic, OpenAI, and Google to remediate” | `a-fx-2111519637049319818.raw.json`、`a-syn-2111519637049319818.json` |
| 10-18 02:41 | 18:41:27 | dax（@thdxr）：“my personal accounts are being attacked now too” | `a-fx-2111528031072461114.raw.json` |
| 10-18 05:04:12 | 21:04:12 | Lydia Hallie：撤销 Claude API 密钥 | `a-fx-2111563956666110287.raw.json` |
| 10-18 05:38:37 | 21:38:37 | dax 回复 Hallie：“note this is the OpenCode Console, not OpenCode itself, those keys never come to us” | `a-fx-2111572618340188393.raw.json` |
| 10-18 08:01:41 | 00:01:41 | dax：“no we already contacted everyone, refunds are already underway…” | `a-fx-2111608620441817449.raw.json` |

## 1. 事实清单

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1a | OpenCode 官方通报全文 | L1 | https://x.com/opencode/status/2111519637049319818 | “We had a security breach last night related to a large scale distillation attack that impacted 1.4% of OpenCode Console users / We have notified them all this morning via email. If you did not get an email you were not impacted, it was a subset of users who had external providers linked / The incident is ongoing so we cannot share more details at this point but we are working with Anthropic, OpenAI, and Google to remediate”（原帖两处空行折成 “/”；“linked” 一词在 fxtwitter JSON 里有，嵌入卡片被 “Show more” 截断在 “external providers”） | `images/01-opencode-official-post.png` | 这是编辑版：id …319818 创建（编辑）于 10-18 02:08:06（UTC 10-17 18:08:06）；初版 id …935833 于 10-18 01:56:19（UTC 17:56:19）。X 嵌入页标 “Last edited 2:08 PM · Oct 17, 2026”（EDT）。syndication 的 `edit_control.initial_tweet_id` = 2111516673953935833，`edit_tweet_ids` 含两者。卡片截图只含到 “external providers … Show more”；后两句只在 JSON 存档里 | 已找到 |
| A1b | 浏览量 | L1 | 同上（fxtwitter） | `views` 1,989,057；likes 9,771；retweets 1,274；replies 598 | 同上 | 读取时刻约 10-18 19:40（北京）；Claude 先前读到约 181 万，数字仍在涨；初版 …935833 单独计数 views 29,075 | 已找到 |
| A1c | 初版帖 | L1 | https://x.com/opencode/status/2111516673953935833 | “We have notified them all this morning via email / The incident is ongoing so we cannot share more details at this point but we are working with” | 无 | 被 HN 50133401 链接的是这一版；已被编辑取代 | 已找到 |
| A1d | OpenCode 账号 10/16–10/18 其他与事故相关的帖、更正、复盘页 | — | — | 未取得：`opencli twitter search "from:opencode since:2026-10-16"` 两次超时（170 秒），账号时间线未能枚举。只确认到上两版通报 | 无 | 不能据此说“没有后续更新” | 未找到一手来源 |
| A2a | 官方博客/复盘页 | — | https://opencode.ai/blog、/security、/trust（均 404）；opencode.ai/、/docs、/v2/docs 无事故文字 | opencode.ai 首页、v2/docs 页的存档里 grep “breach”“incident”“security” 无事故相关句 | 无 | 读取时刻 10-18 19:42（北京） | 未找到一手来源 |
| A2b | 状态页 | L1 | https://status.opencode.ai/ | 过往事故列表（Last 90 days）只有可用性类条目：如 “Oct 18 Elevated error rates Console API · Oct 18, 04:04–07:21 UTC 3 hr 17 min”；页面写 “No incidents since Oct 15.”；无安全事件条目 | 无 | 状态页按“每分钟探测 Console 与推理网关”自动判定（页面 “What do you monitor?” 一节），不是事故通告渠道；Console API 那条的原因页面没写，不作关联 | 未找到一手来源（无安全事件条目） |
| A2c | GitHub security advisory | L1 | https://github.com/anomalyco/opencode/security/advisories （sst/opencode 重定向到同页） | 列出 3 条：GHSA-632h-h47v-g4x4（Sep 24, 2026，High）、GHSA-c83v-7274-4vgp（Jan 12, 2026，Critical）、GHSA-vxw4-wv6m-9hhh（Jan 12, 2026，High） | 无 | 匿名读取（页面带 Sign in）；无与本次事件对应的 advisory | 未找到一手来源 |
| A2d | 官方邮件原文 | L7→L6 | 见 A4 | 邮件主题 “Action Required: Potential provider key breach”（Reddit 帖图中可见） | `images/03-hn-opencode-email-repost.png`（HN 转贴） | 只有用户转贴，无官方页面发布邮件原文 | 部分支持 |
| A2e | Trust Center | L1 | https://trust.opencode.ai/ | 页面列 “SOC 2 Type 2 Compliant”“SOC 2 Type 1 Compliant”，政策含 “Incident Response & Breach Notification”；页面未提本次事件 | 无 | 只记页面写了什么，不作评论；HN 评论说 OpenCode “now has a trust page” | 已找到 |
| A3 | 1.4% 的分母（Console 用户总数） | L1 | 官方帖、邮件、docs、首页 | 官方帖只写 “1.4% of OpenCode Console users”。未给 Console 用户数、注册数、受影响账号数或 workspace 数。opencode.ai 首页另有 “used and trusted by over 16M developers every month” 与 “16M Monthly Devs”，指 OpenCode 整体，不是 Console | 无 | 首页 16M 的口径（是否含 Console）页面未说明，不得拿来当分母，更不得与 1.4% 相乘 | 未找到一手来源（分母未给） |
| A4a | “legacy system”“may have been exposed” 邮件原文（HN 转贴） | L6 | https://news.ycombinator.com/item?id=50133401 （评论 id 50133412，pranshuchittora，2026-10-18 02:02:54 北京 / 10-17 18:02:54 UTC） | “We identified unauthorized access to a legacy system that we were in the process of decommissioning. Third party provider keys (eg OpenAI, Anthropic) saved in your workspace may have been exposed. Please revoke and replace those keys through the relevant provider’s dashboard, update any applications using them, and review recent usage for unexpected activity. We’re sorry this happened and will follow up with any updates.” | `images/03-hn-opencode-email-repost.png` | 评论人自述原帖作者；评论开头写 “From OpenCode Team ---” 是其自加。HN 评论页，截图里 “18 hours ago” 是渲染时的相对时间（渲染约 10-18 20:15 北京） | 已找到 |
| A4b | 独立转贴比对 | L6/L7 | Reddit https://www.reddit.com/r/opencode/comments/1x8bas3/they_fd_up_breach/ （图片帖，307 分，作者 horstenegger） | 帖内图片是邮件截图：主题 “Action Required: Potential provider key breach”，正文与 HN 转贴逐词一致（“Hi, We identified unauthorized access to a legacy system that we were in the process of decommissioning. Third party provider keys (eg OpenAI, Anthropic) saved in your workspace may have been exposed. …”），含 “Reply To” 行与 “Today at 16:01”（收件人本地时区未知） | 无（图为用户截图，只在 scratchpad 目视核对，未入库） | 两个独立来源措辞一致。Matt Johansen 的 “Confirmed” 帖也带邮件截图（图在 pbs.twimg.com，未取）；redreamality、cybersecuritynews 描述的措辞与此一致。Startup Fortune 称邮件说调查 “to date” 未发现其他数据泄露——这一句在 HN 与 Reddit 两份转贴里都没有，未能核实 | 部分支持（措辞一致；仅 L6/L7） |
| A5a | “distillation attack” 的依据 | L1 | 官方帖 | 官方只写 “related to a large scale distillation attack”，未给调用量、模型、时间段、对象；邮件（A4）全文不含 “distillation” 一词 | `images/01…`、`images/03…` | 官方帖称 “The incident is ongoing so we cannot share more details at this point” | 未找到一手来源（无证据披露） |
| A5b | “distillation” 一词的另一处官方侧用法 | L2 | https://x.com/thdxr/status/2111601065569448378 | dax（bio：“building @opencode at @anomalyco / not the ceo”）回复一位问“Is the suspension of my workspace also a result of the attack?”的用户：“no not related that's our usual anti fraud / distillation stuff” | 无 | 10-18 07:31:40（UTC 10-17 23:31:40）；上下文只有该用户一问，未说明具体指什么；只能说 OpenCode 自己在日常反欺诈中也用 “distillation” 一词 | 已找到 |
| A5c | Nathan Lambert 质疑 | L6 | 原帖未找到 | 仅有二手：Startup Fortune 称 “Researcher Nathan Lambert wrote that the public deserves to know whether this is ‘distillation’ as used to train downstream models ‘or some other weird shady pattern to use illegally obtained tokens.’”（摘自 a-m-startupfortune.txt） | 无 | `opencli twitter search`（from:natolambert）两次超时；WebSearch 仅返回该二手文；原帖与时刻未取得。他 7-24 的另一条帖与本事件无关 | 未找到一手来源 |
| A5d | HN 质疑（线索） | L6 | https://news.ycombinator.com/item?id=50133941 （ipsum2） | “"mass distillation campaign" there's no public evidence of this. I wonder what OpenCode/Anomaly has, would be an interesting postmortem.” | `images/04-hn-ipsum2-no-public-evidence.png` | 10-18 02:56（UTC 18:56）；HN 评论分数对未登录者不可见，记“未知” | 已找到 |
| A6a | Hallie 撤销 Claude 密钥 | L2 | https://x.com/lydiahallie/status/2111563956666110287 | “We’ve been working with OpenCode on yesterday’s breach and have revoked Claude API keys we believe were affected. / If you saved a key in OpenCode and want to be extra careful, please create a new one!” | `images/02-hallie-revoked-keys.png` | 10-18 05:04:12（UTC 10-17 21:04:12；嵌入页显示 5:04 PM EDT）；浏览 76,724、点赞 1,213（读取时）。帖内引用的是官方帖编辑版 …319818，不是初版 …935833 | 已找到 |
| A6b | Hallie 的公开职位 | L2（自述） | https://x.com/lydiahallie （fxtwitter author.description） | “Claude Code @AnthropicAI @ClaudeDevs” | 无 | 只有她自己的账号简介；未在 anthropic.com 官方页面找到以她署名的页面（未逐页检索）。帖子用 “We” 但未写 “Anthropic” | 部分支持 |
| A6c | Anthropic 官方渠道另有说明 | — | https://status.claude.com/history、https://www.anthropic.com/news | 两页存档里 grep “opencode”（不分大小写）0 处 | 无 | 读取 10-18 21:37 北京；未直连 Anthropic 官方 X 时间线与 support.claude.com 逐页检索；WebSearch 也无结果 | 未找到一手来源 |
| A6d | OpenAI、Google 公开说明 | — | https://status.openai.com/history | grep “opencode” 0 处；Google 未查官方页（WebSearch 无结果） | 无 | 官方帖只称 “working with Anthropic, OpenAI, and Google to remediate”，三方里只见 Anthropic 一侧的 Hallie 帖。用户自述 Google 项目被暂停（见 A8），非 Google 官方说明 | 未找到一手来源（OpenAI、Google 未见） |
| A6e | OpenCode 一方对 Console 与 CLI 的区分 | L2 | https://x.com/thdxr/status/2111572618340188393 | “note this is the OpenCode Console, not OpenCode itself, those keys never come to us” | `images/05-dax-console-not-opencode.png` | 10-18 05:38:37（UTC 21:38:37）；回复对象是 Hallie 帖；点赞 307（嵌入页，读取时） | 已找到 |
| A7a | Console 在云端存 provider key | L1 | https://opencode.ai/v2/docs/console/providers/ | “Connect provider accounts you already pay for, such as Anthropic, OpenAI, or AWS Bedrock, and share their models with everyone in the workspace. Console stores the credential and serves each provider through its gateway, so members use the models without ever seeing the provider key.” 以及 “Pick a provider from the catalog and enter its API key.” | 无 | 同句在 Wayback 2026-10-07 15:46 UTC 快照（`a-wb-console-providers-1007`）里已存在，事发前措辞未变（快照只证明“那时页面这样写”）。文档未说明加密方式（未见 “encrypt” 字样） | 已找到 |
| A7b | Console 是可选服务、与本地 CLI 区分 | L1 | https://opencode.ai/v2/docs/console | “The OpenCode Console is an optional service that provides additional benefits to using OpenCode, on your own or as a team.”；“Providers for connecting your own provider accounts through the Console gateway” | 无 | 同上 | 已找到 |
| A7c | 本地 CLI 的 key 存放 | L1 | https://opencode.ai/docs/providers/ | “When you add a provider’s API keys with the /connect command, they are stored in ~/.local/share/opencode/auth.json.” | 无 | v1 文档页；Wayback 2026-10-17 15:27 UTC 快照（`a-wb-docs-providers-1017`）同句。文档**没有**写 “本地 CLI 不会上传 key”——这是 dax 的说法（A6e）与社区推断，不是文档原句 | 部分支持（本地存放有原句；“不上传”无原句） |
| A7d | v2 里 CLI 连 Console workspace | L1 | https://opencode.ai/v2/docs/console | “Run /connect in OpenCode and choose the workspace.”；v2 providers 页：“Members run /connect and choose the workspace. Each enabled provider appears as the workspace name followed by the provider name” | 无 | 说明 CLI 可通过 /connect 使用 Console workspace 里的 provider，但 key 存在 Console 端 | 已找到 |
| A8 | 用户层面实际损失（均为用户自述，L6，不作推算） | L6 | 见右 | ①HN jprokay13 https://news.ycombinator.com/item?id=50134785：“I saw the email this morning from OpenCode about my key being compromised alongside an email from Anthropic stating that I used up my $20 in API credits for the month.” ②Reddit razorpolar（70 分）：“Just came to post the same thing, $400 unauthorized spend on our Anthropic account today as a result of this” ③Reddit Mission_Maximum_2329（4 分）：“The problem has already reached €200. Google is already investigating it … Google suspended my project for excessive abuse” ④Reddit bleakj（4 分）：“Over a grand on Google cloud api here...” ⑤X @tunahorse21（https://x.com/tunahorse21/status/2111529235492049196，10-18 02:46 北京）：“opencode hack ran up a 3k api key bill” | 无 | HN 评论分数不公开；Reddit 帖 https://www.reddit.com/r/opencode/comments/1x8bas3/they_fd_up_breach/ ；这些数字都没有凭证，也没有与官方数据核对；dax 回应见 A8b | 已找到（作为用户自述） |
| A8b | OpenCode 一方对退款的说法 | L2 | https://x.com/thdxr/status/2111608620441817449 | “no we already contacted everyone, refunds are already underway and i just want to make sure people didn't miss an email”；另一条 https://x.com/thdxr/status/2111582502544384034：“send me an email … with more info we will get you refunded” | 无 | 10-18 08:01:41 与 06:17:54（UTC 00:01:41、22:17:54）；退款范围、金额、条件未说明；是个人账号回复，不是公司公告 | 已找到 |
| A9 | 热度 | — | `barking heat`（a-heat.json） | HN 50133401 “OpenCode Hacked: User credentials leaked”：877 分 / 164 评（采样 10-18 19:51 北京；渲染页显示 879 分）；同题材其他 HN：50133006（13/2，Reddit 链接）、50133287（13/0，mattjay 帖）、50139083（6/2，官方帖，10-18 16:22）、50133584（3/0，10-18 02:18）、50133910（4/0）；Reddit r/opencode 帖 307 分（opencli reddit read）；官方帖 views 1,989,057 | 无 | HN 帖 10-18 02:01:41（UTC 10-17 18:01:41）；`barking heat` 的 Reddit 搜索被重定向到登录页，失败，改用 opencli 读单帖 | 已找到 |

## 2. 可能的吠点（线索，每条注出处）

1. 官方帖说 “last night” “this morning” 没给时区；用户在 10-17 23:18（北京，15:18 UTC）就已收到通知并报告盗刷（`a-fx-2111476982580211851.raw.json`）。OfficeChai 写邮件 “on the morning of October 18” 与此不符（`a-m-officechai.txt`）。
2. 官方 X 帖说 “distillation attack”；邮件只说 “unauthorized access to a legacy system”，并只说 key “may have been exposed”（A4）。邮件与官方帖是两种措辞，且邮件未出现 “distillation”。
3. 官方没给分母（A3）；首页 16M 月活是整体口径，不能当 Console 的分母。
4. 官方帖没有证据支撑 “distillation”：无调用量、模型、时间段（A5a）。dax 又说 “our usual anti fraud / distillation stuff”（A5b），即 OpenCode 自己日常反欺诈也用这个词；这条上下文不足，不能延伸。
5. 影响范围限定为 “OpenCode Console users … who had external providers linked”，不是 OpenCode CLI/桌面端；官方帖没有写 “CLI 不受影响”，这一点来自 dax 的回复（A6e）和社区。HN 有人质疑“没写不影响就不能排除”（NewsaHackO，id 50137418）。
6. 官方 docs 写明 Console “stores the credential”（A7a）；是否明文存放，官方未说；Reddit 评论称 “stored in plain text”（SuperSlowSubie，3 分），无来源，仅 L6。
7. Hallie 写 “keys we believe were affected”，撤销范围是 Anthropic 的判断；并建议 “want to be extra careful” 的用户另建新 key。OpenAI、Google 一侧未见公开说明（A6d）。
8. 官方帖初版（17:56 UTC）没有 “If you did not get an email you were not impacted…” 与 “Anthropic, OpenAI, and Google”，12 分钟后编辑才补上（A1）；HN 链接的是初版。
9. 状态页与 GitHub advisory 都没有安全事件条目（A2b、A2c）；Trust Center 页没有提及事件（A2e）。
10. 社区线索（均 L6，原帖回到一手来源前不得写成事实）：
   - HN ipsum2（id 50133941）：“there's no public evidence of this”（`images/04…`）。
   - HN searealist（id 50134268）：“I'm not sure how this adds up. I could understand if this impacted their uptime, but not security.”
   - HN verdverm（id 50137005）：要求公开被盗 key 的查询数据集，称 “I'm unwilling to trust those industry players at their word.”（他同时自述曾与 OpenCode 一起处理 CLI 的另一个安全回归，立场非中立。）
   - HN InsideOutSanta（id 50134844）：“if you only use the CLI, you're not affected”，称此点在官方帖里有但措辞不易懂。
   - Reddit ProtolZero（38 分）：“I thought the third-party keys are stored locally.....why are they on opencode's server?”；其下 kosciak9（17 分）：“They are not, it’s the workspace BYOK for Zen”（该回答把 Console workspace 与 Zen 混用；HN redsymbol 评论称 Console 与 Zen、Go 不同，均 L6）。
   - 命名混乱：HN rozenmd（id 50135135）与 ShowalkKama（id 50133948）要求标题写 “OpenCode Console”；官方帖本身写了 “OpenCode Console”。
   - HN 评论分数对未登录者不可见，所以所有 HN 摘录“得分”为未知。

## 3. 夸大/写歪说法的实例

| 发布方 | URL | 原句（逐字） | 时间 | 说明 |
|---|---|---|---|---|
| HN 用户投稿标题 | https://news.ycombinator.com/item?id=50133401 | “OpenCode Hacked: User credentials leaked” | 10-18 02:01:41（UTC 10-17 18:01:41） | 标题写 “OpenCode” 而非 “OpenCode Console”，“credentials leaked” 对应邮件的 “may have been exposed”；链接到初版官方帖。HN 评论里有人要求改标题 |
| Startup Fortune | https://startupfortune.com/opencode-breach-exposes-stored-openai-and-anthropic-keys-prompts-urgent-key-rotation/ | 标题 “OpenCode breach exposes stored OpenAI and Anthropic keys, prompts urgent key rotation”；正文 “exposed stored OpenAI, Anthropic and Google API keys for a subset of developer workspaces” | 发布时刻页面未取到，存档读取 10-18 20:13 | 邮件与官方帖的 “may have been exposed” 被写成已确定 “exposes/exposed”；它自己也写 “The company has not confirmed that sequence” |
| Crypto Briefing | https://cryptobriefing.com/opencode-distillation-attack-api-key-breach/ | “According to the company, attackers exploited vulnerabilities in a legacy system to gain unauthorized access to API keys stored in user workspaces.” | 发布时刻页面未取到 | 邮件原句是 “unauthorized access to a legacy system”，没有 “exploited vulnerabilities” 也没有 “API keys … stored” 的断言 |
| OfficeChai | https://officechai.com/ai/opencode-discloses-security-breach-says-attackers-may-have-accessed-users-openai-anthropic-and-google-api-keys/ | “emails sent to affected customers on the morning of October 18” | 发布时刻页面未取到 | 与 X 时间戳不符：10-17 15:18 UTC 已有用户报告收到通知 |
| TestingCatalog（X） | https://x.com/testingcatalog/status/2111536604351246541 | “OpenCode published an official statement regarding a recent distillation attack. > Earlier today, OpenAI, Anthropic and Google balances of some OpenCode users got drained. > Reportedly, only 1.4% of users were affected …” | 10-18 03:15:31（UTC 10-17 19:15:31） | 用引号样式把 “balances … got drained” 排在官方声明之后，官方帖没有这句；后面 “major AI labs are now much more protected” 是该账号自己的推断 |
| techbytes（仅 WebSearch 摘要，页面未存档） | https://techbytes.app/posts/opencode-discloses-security-breach-exposing-developer-api-tokens/ | WebSearch 摘要称其写 “telemetry logging regression” 且 “DeepSeek, GitHub and GitLab tokens” 被泄露 | — | 与官方描述不符，没有一手来源；未核实原页，只作线索 |

中文媒体与标题实例：本组未检索（时间不够），写“未找到”并非“不存在”。

## 4. 扫描说法勘误（对 Claude 初步核对的更正）

1. “官方 X 帖 …319818 发布 18:08:06 UTC”：这是编辑版；初版 …935833 发布于 17:56:19 UTC，12 分钟后编辑补写（`edit_control` 为证）。“发布时刻”应注明首发 10-18 01:56:19（北京）与编辑 02:08:06。
2. “Hallie 帖内链接的是 OpenCode 另一条帖 …935833（更早）”：与存档不符。fxtwitter 的 `quote.url` 与嵌入卡片显示的都是编辑版 …319818；…935833 是 HN 链接的那一条。
3. “HN 860 分 / 161 评”：读取时已涨到 877（19:51）/ 879（约 20:15）、164 评。
4. “首条评论（pranshuchittora）贴出邮件”：属实，评论 id 50133412，18:02:54 UTC；评论人同时是 HN 帖作者。
5. “AIHOT 摘要称 Nathan Lambert 质疑”：原帖本组未找到，只有 Startup Fortune 的二手引用（A5c）。
6. “1.4%”：已核对三份官方文字（初版、编辑版、Hallie 引用）均写 “1.4% of OpenCode Console users”，分母未给。
7. “1014 速览 OpenCode 2.0 与 HN 评论提标题生成”：本组未复核；本次 HN 评论里 Reyonlds96、anonymous908213、NavinF 又谈到该争议（原事为 CLI，与 Console 泄露无关，InsideOutSanta 与 c-hendricks 也这样说）。

## 5. 未完成 / 失败 / 风控

- 失败：opencli `twitter search`（from:natolambert、from:opencode since:2026-10-16）超时，无结果；`opencli browser … open https://x.com/…` 页面停在启动画面，无法截 x.com 原页，因此官方帖卡片只能用匿名嵌入页（被 “Show more” 截断）。Notebookcheck 403（Cloudflare）。`opencode.ai/docs/console`、`/blog`、`/trust`、`/security`、`/v2/trust` 404。`dev.opencode.ai/console`、`opencode.ai/console` 正文过短（需 JS/登录）。`barking heat` 的 Reddit 搜索重定向登录页。
- 未做：OpenCode 账号完整时间线；Lambert 原帖；中文媒体检索；Anthropic 官方 X 时间线、support.claude.com 的逐页查找；Google 官方页。
- 风控：未收集、未展开任何关于攻击者国籍或“流向中国的转发站”的归因；HN 里有一条评论把 “distillation” 与国别联系起来，本文件不记录其内容；其余社区摘录只含与 key、分母、证据有关的句子。
