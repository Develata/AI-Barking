# 1011 期 B 组取证清单：Grok Bot 私人 CFO 智能体把银行余额发进公司 Slack

取证日 2026-10-11（本机 EDT 03:50–04:45，北京时间 = 本机 + 12 小时，约 15:50–16:45）。时间一律写北京时间（UTC+8），括号保留原时区。执行方模型 claude-sonnet-5-5。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。逐字摘句用原语言原文，英文原文均已在存档文件里 grep 核过（见“存档”列）。数字与帖子热度均为取数时刻值，随时变化。

私人信息处理：Shane Mac 的姓名、职务照原文存。BI 原文没有具体余额数额；Mac 在 X 首帖附带的 Slack 截图里可见部分被遮盖后的数字，本组没有摘录这些数字，20 号配图只截帖子文字部分（见 capture-log 说明）。存档 JSON 中账号的 location 字段已替换为占位符。

## 一、核心结论（供 Claude 判断吠点）

1. **事件只有当事人一方的说法。** BI 是 as-told-to 口述文，“edited for length and clarity”；事件内容（10 月 1 日、同名频道、被谁发现）无 xAI/Grok Bot 官方确认；本组未找到 XMTP 公司方或 xAI 的任何第三方证据（如 Slack 日志、被误发的消息本身）。
2. **BI 与 Mac 自己在 X 上的线程说法不一致，且 BI 文内也有两处并列的说法**，见第五节“扫描说法勘误”和第六节“可能的吠点”。最要紧的一条：Mac 在 X 上说“this specific Grokbot agent didn't have Slack. But another one did”，BI 里却是“for one of them, I connected my Slack account”+“The CFO agent got the channel wrong because the channels had the same name”。
3. **xAI 官方文档早就写明“Bots 之间不是安全边界、连接是账户级的”**（docs.x.ai 的 computer-and-apps、approvals-security-and-privacy、faq 三页逐字见 B3）。这是正文“只读权限防不住外传”最硬的一手依据；它同时说明“被别的 Bot 的连接发出去”按文档设计是可预见的，不是漏洞式意外。
4. **“官方修复”无一手来源。** BI 里“Grok realized that users must explicitly grant permission ... They implemented and shipped a solution last night.”只来自 Mac 转述；xAI 官方 Grok Bot changelog（x.ai/changelog/bot，最新条目 v0.68.1，2026-10-07）、官方文档（security 页正文与 1007 存档一致）、x.ai/news（最新 9/28 Team Bots）、docs.x.ai 发布说明、Cursor changelog（最新 10/6）均未见“跨频道/跨连接须明确授权”类条目。官方文档页中 10/9 有修改的两页（computer-and-apps、chat-and-collaboration）无 10/1 前基线，无法判断改了什么（Wayback 对本机请求 429，见 capture-log）。
5. **“read-only”的含义没有一手定义。** 官方文档里 “read-only” 只出现在 X 公开读取一节与“从只读任务开始”的建议里，没有对银行/Plaid 连接的描述；官方 @bot 账号 9/27 03:54（9/26 15:54 EDT）发帖宣布 Finance integration，帖文本身未写 read-only；“read-only、经 Plaid、不拿银行凭证”只见于 PYMNTS（二手）。另有时间线疑点：Mac 说约 8 月底建 CFO 智能体并“At first, I had it run every week”，而官方 Finance integration 公告在 9/27（北京时间），Mac 以什么方式接入银行未说明（不下结论）。
6. Mac 在 BI 之前已在 X 发了整条线程：首帖 10/6 23:02:48（15:02:48 UTC），BI 在 10/9 才刊出；扫描里“BI 首发”的印象应改为“Mac 自己先发，BI/WSJ 随后”。

## 二、B1　BI 全文

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| BI 当事人口述，经编辑，署名 Aditi Bharade | L4（权威媒体；内容为当事人口述） | https://www.businessinsider.com/personal-ai-agent-grok-bot-posted-bank-details-company-slack-2026-10 | 标题 “My personal AI agent posted my bank details on company Slack. It's made me rethink how I use it.”；署名行 “As told to / Aditi Bharade”；“This as-told-to essay is based on a conversation with Shane Mac, the 40-year-old CEO of software company XMTP Labs. It has been edited for length and clarity.” | 13-bi-headline-readonly.png（含标题、署名、页面时间 “Oct 9, 2026, 4:43 AM GMT-4”、as-told-to 说明、只读权限段；已用 CSS 隐藏广告位与头图，正文未动） | JSON-LD datePublished/dateModified 2026-10-09T08:43:28.186Z = 北京 10/09 16:43:28（页面显示 4:43 AM GMT-4）。curl 匿名取得全文 HTML（页面有 “available exclusively to Business Insider subscribers” 横幅，但服务器返回了正文，未绕过任何东西）。标题的 “bank details” 与正文内容（余额与开支）的差异见第六节 | 已找到 |
| 只读权限 | L4（当事人自述） | 同上 | “I set up CFO using Grok Bot around the end of August.”；“For its permissions, I gave it read-only access to my personal checking and savings accounts.”；“I told it to send messages only to me through Grok Bot. Every month, it would send me a monthly report in a group chat of all my AI agents on Grok Bot, which I named "My Personal Exec Team."”；“At first, I had it run every week, and it worked great. But the first time the monthly audit kicked off, that's when it went wrong.” | 13（read-only 句）；无单独图 | “read-only” 是 Mac 自述，BI 未另行核实；BI 没有给出具体余额数额 | 已找到（自述） |
| 10 月 1 日发现；内容含余额与开支 | L4 | 同上 | “It was Thursday, October 1. Our head of product sent me a DM on Slack. He said, "Hey, heads-up. I think you meant to post the XMTP bank balance."”；“I didn't mean to post anything.”；“It included my savings account balance. It listed my biggest expenses of the month.”；“It showed that I was way over my monthly spending target because of the barn I'm building on my property.”；“It started apologizing and said it would delete the message. I had already deleted it by that point.” | 14-bi-oct1-discovery.png | “Thursday, October 1” 为 BI 文内日期；发现时刻、消息在频道里停留多久，BI 未写（Mac X 线程写“at 8:40am ... sat there for two hours”，时区未标，见第四节） | 已找到 |
| Exec-team / same connections / same name / 修复句 | L4 | 同上 | “When the Grok team investigated what had happened, they found the answer.”；“The CFO agent didn't have psychosis. It was doing exactly what I told it to do. But it confused the destination, sending the message to a Slack chat titled "Exec-team" — comprising XMTP's executive team — instead of my personal group chat with my AI agents.”；“I created a bunch of different agents, and for one of them, I connected my Slack account. But underneath the hood, they're actually all using the same connections, even if they feel like different agents.”；“The CFO agent got the channel wrong because the channels had the same name.”；“Grok realized that users must explicitly grant permission to their agents before those agents can move information to other channels. They implemented and shipped a solution last night.”；“After the incident, I removed all my connections ... I disconnected Google, my calendars, my banking, Stripe, everything.” | 15-bi-exec-team-same-connections.png（含“removed all my connections”之前的全部相关段，不含该句；该句见 b-bi.txt） | “last night” 相对采访时间，BI 未给日期；“Grok team investigated” 与 “Grok realized ... shipped a solution” 均为 Mac 转述，BI 未引 xAI 任何人 | 已找到 |

## 三、B2　xAI 官方对本次修复的一手

结论：**未找到一手来源**（没有发现任何官方公告或文档文本写明“跨频道/跨连接须用户明确授权”）。查过的页与结果：

| 页面/渠道 | 结果 |
|---|---|
| https://x.ai/changelog/bot（Grok Bot 官方 changelog，RSS 同） | 最新 v0.68.1（2026-10-07）。10/1 之后的条目：v0.66.0（10/2）、v0.67.0（10/2）、v0.68.0（10/5）、v0.68.1（10/7）。逐条 grep “slack / draft / permission / channel / approv / destination / connector / group / cross / explicit / plaid / financ / bank”，没有任何“须明确授权才能把信息发到其他频道/连接”的条目。最接近的无关条目见下一行 |
| 同上，相关条目逐字 | v0.63.0（9/29）：“In Slack threads, a Bot that needs your approval posts a Review an action card only you can answer, instead of dropping the action.” v0.64.0（9/30）：“Email and Slack draft cards offer Disable drafts for this Bot, so that Bot sends messages directly; ask the Bot to turn drafts back on.” v0.67.0（10/2）：“Approval cards show when your Ask first rule paused an action or Always allow added a rule, with Edit rule to change it in Settings.” v0.68.0（10/5）：“Approval cards ask whether to allow the Bot by name and quote its instruction, and Settings calls saved rules Custom Rules.” v0.45.0（9/8）：“Your Bot can write an email or Slack message as a draft for you to review, then Send or Discard it.” 这些是 app 发布条目，官方未称它们与本事件有关。**注意：changelog 只记客户端版本，服务端权限规则的改动不一定出现在其中；因此“changelog 无条目”≠“未修复”，只能说“官方没有公开说明”** |
| https://docs.x.ai/grok-bot/security（对照 1007 存档 `docs/2610/1007/sources/e-xai-docs-security.txt`） | 正文无变化（`b-xai-docs-diff-vs-1007.txt`：54 段仅 1 段被标出，是网络策略表格在两次存档中的排版差异，内容相同）；页面 dateModified 与 sitemap lastmod 均为 2026-09-28。security-faq 同为 9/28 |
| https://docs.x.ai/grok-bot/overview（对照 1007 存档） | 新增一段 “You can also start work from X. Tag @bot in a post or reply, and your main Bot picks up the post as a task while @bot confirms with a short public reply. ...” 及几条链接，与本事件无关；lastmod 10/7 |
| docs.x.ai 全部 22 个 grok-bot 页的 markdown（`b-xai-docs-md/`） | grep “cross / another channel / other channel / move information / destination / explicit”无“跨频道授权”条款（“explicit/explicitly” 只出现在通用建议或其他语境，如 “Prefer explicit boundaries for: Sending messages or invitations ...”“Users must be explicitly assigned”）。docs 无 Finance/Plaid 页（docs.x.ai/grok-bot/finance 404） |
| docs.x.ai sitemap 的 lastmod（旁证，不能证明改了什么） | computer-and-apps、chat-and-collaboration、get-started 均 2026-10-09；approvals-security-and-privacy、bots、mobile、settings、skills 为 10-06；team-bots 10-08；overview/tag-on-x/troubleshooting 10-07。没有 10/1 之前的基线文本，无法比对；Wayback 对本机请求返回 429（“suspected abusive bot traffic”），未取到 |
| https://docs.x.ai/developers/release-notes | October 一栏只有 grok-voice-transcribe-1.0 退役 |
| https://x.ai/news | 最新条目 Product·Sep 28, 2026 “Team Bots: shared AI teammates that learn as they work”；10 月无条目；无 Grok Bot 修复说明 |
| https://x.ai/bot | 无相关条目 |
| Cursor（文档称 Grok Bot 的连接令牌、Auto Review 等基于 Cursor 后端）https://cursor.com/changelog | 最新 Oct 6, 2026 “Remote control for local agents”，无相关条目；cursor.com/bot 返回 79 字符空壳 |
| @bot 官方账号、Grok Bot 署名员工在 X 上的相关发言 | **未取到**：fxtwitter 只能按帖 ID 取，没有时间线接口；WebSearch 两次未搜到任何官方/员工回应。（1007 存档里的员工 @poteto 本组未查到相关回复） |
| 媒体转述 | runtimewire（10/9 22:22 北京；“Mac said Grok Bot shipped a permission change the night before publication”）仅转引 BI；TechTimes UK（10/7）写 “As of 7 October 2026, SpaceXAI had not confirmed the incident” |

状态：B2 = **未找到一手来源**。docs.x.ai 文档页 10/9 的 lastmod 只能作旁证。

## 四、B3　Grok Bot 官方文档对连接/共享/只读的原句（逐字）

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 所有 Bot 共用一台电脑、会话与凭证共享 | L1 | https://docs.x.ai/grok-bot/computer-and-apps（存档 `b-xai-docs-md/computer-and-apps.md`；sitemap lastmod 2026-10-09） | “Every Bot on your account uses the same computer: Browser cookies and signed-in sessions are shared / Files are visible to every Bot / Command-line credentials are shared / One Bot can continue from work another Bot saved”；“The computer is assigned to your user account, not an individual Bot. Do not place a credential or file on it if another Bot on your account should not be able to use it.”；“The screens are separate work surfaces, not separate security boundaries.” | 18-docs-one-computer-shared.png | 页面无发布日期，仅 sitemap lastmod；是否 10/1 前已有此段无法判定（见 B2） | 已找到 |
| 已安装连接器是账户级的，不隔离到某一个 Bot | L1 | 同上 | “Installed connectors are account-wide. Their availability is not isolated to one Bot.”；“Because the browser is shared, signing in for one Bot makes the session available to your other Bots.” | 19-docs-connectors-account-wide.png | 这是 Mac 说 “they're actually all using the same connections” 的官方对应句；该页该段是否 10/9 才加入未知 | 已找到 |
| 不要把 Bot 当安全边界 | L1 | https://docs.x.ai/grok-bot/approvals-security-and-privacy（lastmod 10-06）；https://docs.x.ai/grok-bot/faq（lastmod 09-29） | approvals：“All of your Bots share one cloud computer assigned to your user account. Files, browser sessions, and command line credentials on that computer are available across your Bot roster.”；“Do not use separate Bots as a security boundary.”；“Delete a connector or revoke its authorization in the source service when access is no longer needed.” FAQ：“The computer is assigned per user, not per Bot. Do not use separate Bots as a security boundary.” | 22-docs-shared-computer-boundary.png | FAQ 页 lastmod 9/29，早于 10/1 事件；approvals 页 lastmod 10/06 | 已找到 |
| 发 Slack 消息的审阅草稿“可以”出现，群聊里不出现 | L1 | https://docs.x.ai/grok-bot/chat-and-collaboration（lastmod 10-09） | “When a Bot prepares an email or Slack message for you, the conversation can show an editable draft. Review the recipients and body, then choose Send email or Send message, or Discard.”；“Approval requests, secret and login requests, and email or Slack drafts do not appear in a group.” | 23-docs-draft-and-group-no-approvals.png（群聊段；草稿句见存档 md 第 50–53 行） | 用的是 “can”；官方 changelog v0.64.0（9/30）加了 “Disable drafts for this Bot, so that Bot sends messages directly”。Mac 的月报按 BI 是发到他的 AI 智能体群聊——按文档，群聊里审批/草稿卡不出现。官方没有说明本次消息走的是哪条路径 | 已找到 |
| 例行任务发起的审批会过期且不执行 | L1 | https://docs.x.ai/grok-bot/approvals-security-and-privacy | “An approval from work that started without you, such as a routine, a trigger, or a message from another Bot, expires after about 10 minutes; the card then shows Expired, and the action does not run.”；“Always allow can save a matching rule”；“Prefer explicit boundaries for: Sending messages or invitations ...” | 无 | 与本事件的关系是推断：月报是定时任务，审批设置是否生效、本次是否有审批卡，官方与 BI 都没说 | 已找到（文档原句；与本事件的关联未证实） |
| “read-only” 的含义（银行/Plaid 连接） | — | docs.x.ai 无此页；官方 @bot 帖 https://x.com/bot/status/2103936247995752705 | @bot：“Grok Bot now connects to your finances.”“Link your bank, card, and investment accounts with the new Finance integration, then ask Bot to help manage your spending, investments, and more.” 文档中 “read-only” 仅有：“This access is read-only and doesn't sign in as you.”（指公开读取 X）与“Start with read-only tasks and draft outputs.”（建议） | 24-bot-finance-post-card.png | 帖发于 2026-09-27 03:54:25（2026-09-26 19:54:25 UTC；卡片内显示本机 EDT 3:54 PM · Sep 26）。“read-only、经 Plaid、不拿银行凭证”只见于 PYMNTS（二手）：“Grok's current connections are read-only: accounts connect through Plaid, though Grok does not receive the user's bank credentials and users can disconnect accounts.”（https://www.pymnts.com/news/artificial-intelligence/2026/grok-puts-the-ai-assistant-between-consumers-and-their-banks/ ）；Plaid 官方帖（2026-09-04 23:05:56，面向 Grok 应用，不是 Grok Bot）：“U.S. users can now securely connect your accounts to @Grok through Plaid.” | 未找到一手来源（官方文档无“read-only”对银行连接的定义；只有官方帖宣布功能，read-only 说法为二手） |
| 向哪些频道发消息的规则 | — | 全部 22 页文档 | 未找到对“Slack 个人连接下 Bot 可向哪些频道发送”的规则；team-bots 页有 Team Bot 在 Slack 的行为（与个人 Bot 不同）：“In a shared conversation, such as a Slack thread or a group chat, it asks that person ...”（team-bots.md 第 130–131 行，Team Bot 范围） | 无 | Mac 用的是个人 Bot，不是 Team Bot（BI 未提 Team Bots） | 未找到一手来源 |

## 五、B4　Mac 本人帖子与 XMTP 身份

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Mac 首帖（线程第 1 条） | L6（当事人个人帖；按分级属社交媒体，但作者即当事人） | https://x.com/ShaneMac/status/2107486740491669879 | “Embarrassed to share this, but it scared the shit out of me / Last Thursday, an AI agent posted my personal bank balances into our company Slack. As me / It broke down all my expenses in detail. Then it apologized / Be careful of the dark side of proactive agents / More below...” | 20-mac-post-text-crop.png（只截文字部分，不含附图） | created_at Tue Oct 06 15:02:48 +0000 2026 = 北京 10/06 23:02:48；取数（北京 10/11 约 16:05）views 941,153 / likes 1,600 / replies 283 | 已找到 |
| 线程第 2 条：8:40am、两小时 | L6 | https://x.com/ShaneMac/status/2107486742613794932 | “I'd set up the agent for my personal finances. One job: a monthly audit, sent to me privately in Grokbot. I named it personal CFO. / At 8:40am it posted that audit in our team channel, under my name. It sat there for two hours. A teammate DM'd me "heads up."” | 21-mac-why-connect-bank-card.png（该嵌入卡含此条作为上文） | 时区未标；views 274,371 / likes 333 | 已找到 |
| 线程第 3 条：只读与“另一个 Bot 有 Slack” | L6 | https://x.com/ShaneMac/status/2107486744249589952 | “Why did I connect my bank? It was read-only, and this specific Grokbot agent didn't have Slack. But another one did. Turns out, they're all connected. / I was really curious whether it could just keep an eye on things. / Elon even said to connect your bank account to Grokbot and promised to make us whole if it lost our money. / And, to be fair, it didn't lose my money.” | 21-mac-why-connect-bank-card.png | views 531,952 / likes 409 / replies 30；附 Musk 8/26 回复：“@Teslaconomics Try it out. If Grok Bot messes up, we will make you whole.”（https://x.com/elonmusk/status/2092692116610957814 ，2026-08-26T19:14:15Z = 北京 8/27 03:14:15；其上文是 Teslaconomics 8/26 12:47:33Z 帖：“Just wondering… has anyone actually connected Grok Bot to their bank account yet?”）——Musk 原话是回复 Teslaconomics 的“try it out / make you whole”，并没有字面说“connect your bank”，Mac 的转述是概括 | 已找到 |
| 线程第 4 条：无人提示、自主行动 | L6 | https://x.com/ShaneMac/status/2107486746325725669 | “The scariest part is that nobody prompted it. / These agents are proactive now. They act without being asked. Mine wasn't hacked and it didn't go rogue. It thought posting my finances to the company was what I wanted. / I asked how it could get this so wrong. It apologized. "You're right to be pissed.” | 16-mac-different-agent-slack-card.png（嵌入卡含此条作为上文，另含 Mac 附的与 Bot 的聊天截图。**以下为图内文字，据 16 号截图目视转录，不在任何文本存档里**：Bot 回答 “That’s on me — checking Exec Team now and deleting it if it’s still up.” “Found it: this morning’s monthly audit put your personal Mercury into Slack #exec-team. Deleting that post now.”；聊天对象名 “Hamilton”，下方 “Messaged CFO”） | views 3,371,340 / likes 1,560 / replies 269 | 已找到 |
| 线程第 5 条：“connected Slack to a different agent” | L6 | https://x.com/ShaneMac/status/2107486749396185231 | “A common response is "you set it up wrong." / I did. / I gave an agent a job title as a name and connected Slack to a different agent. Two small choices. Neither felt risky. Together they put my bank balance in front of my team.” | 16-mac-different-agent-slack-card.png | views 351,105 / likes 609 | 已找到 |
| 线程第 6 条：断开 | L6 | https://x.com/ShaneMac/status/2107486751770186184 | “I disconnected all my personal stuff for now. I need to figure out a better way to manage permissions and control what agents can do...” | 无 | views 297,344 / likes 438 | 已找到 |
| 后续回复：同一连接栈 | L6 | https://x.com/ShaneMac/status/2107548030467731933 | “@seanvowens it was different agents in Grokbot, but they all talk and use the same connections stack.” | 无 | 2026-10-06T19:06:21Z = 北京 10/07 03:06:21；views 13,637 | 已找到 |
| 回应“engagement farming” | L6 | https://x.com/ShaneMac/status/2107609102759198991 | “@MAMware @thsottiaux @poteto not fake at all, genuinely happened - we are also working on ways to keep us safe for agents, very different” | 无 | 2026-10-06T23:09:02Z = 北京 10/07 07:09:02；被回复的 MAMware 帖（2107608239638466681，views 778）写 “Quote from 'CN': This post is fake. This is engagement farming. Later in the thread, he directs traffic to a tool he sells that solves the problem He's claiming.” | 已找到 |
| 后续：“people love a car crash” | L6 | https://x.com/ShaneMac/status/2107831230121070851 | “Turns out, people love a car crash” | 无 | 2026-10-07T13:51:41Z = 北京 10/07 21:51:41；views 2,067 | 已找到 |
| Mac 的公开简介含 “CEO & Co-founder XMTP Labs, building @agentpicnic” | L1/L2（本人账号简介） | https://x.com/ShaneMac （fxtwitter 用户 JSON） | “CEO & Co-founder XMTP Labs @xmtp_, building @agentpicnic. Co-founder @GoSquaredAway.” | 无 | agentpicnic 是 Mac 在建的 AI 智能体工具（Futurism 称他 “currently building an AI tool”），与“engagement farming”质疑有关，只是线索 | 已找到 |
| XMTP Labs 与 CEO 身份的公司官方页 | L1 | xmtp.org 首页无团队页（/about、/team 404；blog.xmtp.org/about、/team 404） | 无；XMTP/Ephemera 招聘页署名 “Shane Mac, CEO, XMTP Labs”（https://web3.career/i/wIzMwQTM ，第三方转载招聘稿，L5）；Redpoint 页：“Founders Shane Mac, Matthew Galligan”（不含职务） | 无 | 公开资料里 Mac 的头衔有 “President”（2022 播客）与 “CEO”（现简介、招聘稿）两种，Matt Galligan 的 Crunchbase 页亦列 “Co-founder and CEO”；只靠 BI 与 Mac 简介支持 “CEO”，公司官网未列 | 部分支持 |

## 六、B5　WSJ 与其他媒体

| 媒体 | 标题 | 时刻（北京；原时区） | 对事件的措辞（逐字） | 存档 | 状态 |
|---|---|---|---|---|---|
| WSJ（Katherine Bindley） | “Personal AI Agents Are Great—Until They Share Your Bank Statement in the Work Chat”；副题 “These hyperactive helpers can sometimes overshare, overpromise and overpurchase”；HTML title “AI Agents Like Grok Bot or Muse Can Save People Time—Unless They Go Off Script - WSJ” | 10/09 07:00（Oct. 8, 2026 7:00 pm ET；2026-10-08T23:00:00Z）；dateLastPubbed 2026-10-09T06:34Z | 可见首段：“Shane Mac had been using a personal AI agent since August without issue when the startup chief executive got pinged by one of his employees: a “heads up” about a team Slack message that had a screenshot attached.”；“There before him was a literal snapshot of his checking and savings account balances, various construction costs for a house project, and recurring charges for Netflix and car insurance, courtesy of his agent.” | `b-wsj-visible.txt`（仅可见部分；paywall 未绕过；页面标 article.access=paid，约 975 词，199 条评论计数；经 firecrawl 读取，curl 返回 401 DataDome 验证页已丢弃；内置浏览器按安全限制不能打开 wsj.com） | 已找到（仅可见部分） |
| Cybernews（Niamh Ancell） | og:title “Grok Bot leaks CEO’s bank data across company Slack”；页面 H1 “Tech CEO connects Grok Bot to bank account and instantly regrets it”；<title> “Grok Bot leaked a CEO’s bank data in Slack | Cybernews”；副标 “He should’ve read the fine print.” | 10/07 20:04（2026-10-07T12:04:00Z）；更新 10/08 14:53（10/08T06:53Z） | “Grok Bot posted his bank balances and spending to a company Slack channel while posing as him.”；“Inexplicably, Grok Bot posted his financial information and bank balances to his company’s Slack channel. What’s worse is that Grok Bot leaked the information while posing as Mac.” | `b-cybernews.txt`；另一篇 `b-cybernews-connect-bank.txt`（Musk “make you whole” 背景） | 已找到 |
| Futurism（Victor Tangermann） | “Man Says He Was Mortified When His AI Agent Posted His Bank Balances and Exactly What He's Spending His Money on in a Slack Work Channel” | 10/07 23:58（Oct 7, 2026 11:58 AM EDT） | “At the same time, we should take Mac’s cautionary tale with a health dose of skepticism. For one, we were unable to independently verify his claims.”；写 Mac 在建 AI 工具；引 “engagement farming” 质疑 | `b-futurism.txt` | 已找到 |
| TechTimes UK | “CEO Says Grok Bot Agent Posted His Bank Balances to Company Slack, but a Community Note Disputes It” | 10/07 22:51（Oct 07 2026, 3:51 PM BST） | “As of 7 October 2026, SpaceXAI had not confirmed the incident and TechTimes had not verified it.” | `b-techtimes.txt` | 已找到（二手） |
| t3n（德） | “KI-Agent teilt Finanzdaten des CEOs bei Slack: Wie es dazu kam”（页面 H1 “Kontostand und Ausgaben gepostet: KI teilt private Finanzen des CEOs im Unternehmens-Slack”） | 10/08 23:30:51（2026-10-08T15:30:51Z；页面显示 08.10.2026, 17:30 Uhr，时区未标） | “Der Bot postete seine Finanzen in einem öffentlichen Slack-Kanal, sodass andere aus dem Unternehmen seinen Kontostand sowie Einnahmen und Ausgaben bekommen konnten.” | `b-t3n.txt` | 已找到（措辞偏离，见 B7） |
| runtimewire（Ryan Merket） | “XMTP Labs CEO says Grok Bot added channel permissions after his bank audit hit Slack” | 10/09 22:22（Published Oct 9th, 2026, 9:22am CT；JSON-LD 2026-10-09T14:22:39Z） | “Mac said Grok Bot shipped a permission change the night before publication after confusing similarly named chats.”（只转引 BI，无新信源） | `b-runtimewire.txt` | 已找到（二手） |
| playtech.ro（罗） | “Un lider din tech și-a conectat contul bancar la Grok Bot. Apoi și-a văzut finanțele personale pe Slack” | JSON-LD 2026-10-08T10:33:00Z（= 北京 10/08 18:33）；页面显示 “Publicat 09.10.2026 09:08”，两者不一致，原因未查 | 转述 Cybernews | `b-playtech.txt` | 已找到（二手） |
| NewsBytes | “XMTP Labs CEO Shane Mac disconnects services after AI mishap” | 未取到 | curl 返回 403，未存档 | 无 | 未取到页面（仅见搜索标题） |
| Fortune、The Verge、Gizmodo、TechCrunch | — | — | 两次 WebSearch 点名搜索均未出现这几家对此事的报道 | 无 | 未找到 |

另存：BI 10/6 相关文《early users delete personal AI agents》（`b-bi-early-users.txt`，datePublished 2026-10-06）——全文只有第 294 行提到 Grok Bot，与 Mac 事件无关，仅备查。

## 七、B6　热度（取数时间：北京 2026-10-11 约 15:55–16:45；均随时间变化）

| 项 | 数字 | 来源/说明 |
|---|---|---|
| HN 帖 “My personal AI agent posted my bank details on company Slack” | 61 分 / 66 条评论；发布 2026-10-10T14:46:16Z = 北京 10/10 22:46:16 | https://news.ycombinator.com/item?id=50033517 ；Algolia 取数 `b-hn-50033517.json`。链接为 BI 页。HN 用户 shanemac（未核对是否同一人）发 “Alright, I’m here… AMA”，共 3 条评论（另有 “Love this”“Over budget since I had to disconnect my financial advisor.”） |
| Algolia 搜索 “Shane Mac”“Grok Bot bank”“Grok Bot Slack”“CFO agent”“proactive agents bank” | 除上帖外未发现其他相关故事帖；“Shane Mac” 另有 3 条 comment 命中同一标题（评论里的转述） | 查询结果文件已清理（只保留命中的 item） |
| Mac 线程各帖（X，fxtwitter/embed） | 首帖 views 941,153 / likes 1,600 / replies 283；第 3 条 views 531,952；第 4 条 views 3,371,340 / likes 1,560；第 5 条 views 351,105；其余见 B4 | 同一线程各帖 views 口径不一，只按帖记录。Futurism 10/7 写 “viewed almost 800,000 times”；CellCog（L5）写 “691,556 views by the next morning” |
| Musk 10/6 北京 23:36:58 帖（Grok @Bot can be your personal chief financial officer!，称自己接了 30 个账户） | views 27,936,150 / likes 24,050 / replies 1,654 | https://x.com/elonmusk/status/2107495338873389121 ，2026-10-06T15:36:58Z；比 Mac 首帖晚约 34 分钟。内容含 Musk 本人持仓数字，本组不摘录，仅存 JSON |
| 官方 @bot Finance integration 帖 | views 24,682,340 / likes 6,492 / replies 583 | 2026-10-06 之前的 9/26 帖，见 B3 |
| Reddit | 未取到 | reddit.com 对 curl 返回 403；WebSearch 工具不允许搜 reddit 域名。未取 |
| BI 在 X 的帖 | 未找到 | 未查到 BI 官方号对此文的帖 |
| 中文媒体 | 未找到 | 两次中文关键词搜索只出现 Cybernews、德语 t3n、罗马尼亚语 playtech、NewsBytes；未见中文媒体标题 |

## 八、B7　夸大/写歪实例

没有找到“AI 泄露银行账号/密码”“Grok 被黑”“AI 转走钱”类标题。找到的偏离如下（原站，逐字）：

| 来源 | 时间 | 原句 | 与原事实差异 |
|---|---|---|---|
| Cybernews，https://cybernews.com/ai-news/grok-bot-leaks-bank-information/ | 10/07 20:04（北京） | og:title “Grok Bot leaks CEO’s bank data across company Slack”；“Grok Bot posted his bank balances and spending to a company Slack channel while posing as him.”；“Inexplicably, Grok Bot posted his financial information and bank balances ...” | “bank data / leaked” 读起来像账号数据外泄；Mac 说是余额和开支，发到高管频道（BI：Exec-team；X：“our team channel”）；“posing as him” 对应 Mac 的 “As me” / “under my name”，没夸大；“Inexplicably” 与同文后面转述的 Mac 解释（另一个 Bot 有 Slack）并存 |
| t3n，https://t3n.de/news/ki-teilt-finanzen-ceo-unternehmen-slack-1767258/ | 10/08 23:30（北京） | “Der Bot postete seine Finanzen in einem öffentlichen Slack-Kanal”（public Slack channel） | BI 与 Mac 都说是高管团队频道（“comprising XMTP's executive team” / “our team channel”），不是“公开频道”；t3n 正文另写 “für etwa zwei Stunden”，与 Mac 线程一致，但与 BI “I had already deleted it by that point” 的次序不同 |
| BI 标题（Mac 口述的拟题） | 10/09 16:43 | “My personal AI agent posted my bank details on company Slack.” | “bank details” 读起来像银行账号、卡号；正文只有“checking account / savings account balance / biggest expenses of the month”，没有账号凭证；这是 BI 自己的标题，不算夸大，但读者易读成账号泄露 |
| WSJ 标题 | 10/09 07:00 | “Share Your Bank Statement in the Work Chat” | 可见首段写 “a literal snapshot of his checking and savings account balances, various construction costs ...”，不是对账单全文；“work chat” 对应 team Slack message |
| TechTimes | 10/07 22:51 | 标题 “... but a Community Note Disputes It” | 本组 10/11 北京约 16:00 取 fxtwitter 首帖 JSON，community_note 字段为 null；note 是否存在、何时撤下，未能核实（MAMware 帖仅引述 “Quote from 'CN'”）。只作线索 |

## 九、B8　可能的吠点（线索，L6；进正文前须回一手来源核实）

1. **BI 与 Mac 线程的机制说法对不上**：X 线程第 3 条 “this specific Grokbot agent didn't have Slack. But another one did.”，第 5 条 “connected Slack to a different agent”，后续回复 “it was different agents in Grokbot, but they all talk and use the same connections stack”；BI 里是 “for one of them, I connected my Slack account ... they're actually all using the same connections” + “The CFO agent got the channel wrong because the channels had the same name”。BI 的“CFO 智能体自己选错频道”与 X 的“另一个有 Slack 的 Bot 发的”是两种因果。（均为 Mac 自述，两处原文见 B1、B4。）
2. **同名频道的说法与前文不一致（BI 文内）**：前文 “group chat ... which I named "My Personal Exec Team."”；后文 “a Slack chat titled "Exec-team"”，又说 “the channels had the same name”。两个名字并不相同（My Personal Exec Team vs Exec-team）；Mac 发的 Bot 聊天截图里 Bot 说的是 “Exec Team” 与 “#exec-team”。并列如实记录，不替 BI 调和。
3. **“read-only”只约束对银行的操作，管不了信息被发到哪**：Mac 自己的话 “It was read-only”；文档无对银行连接 read-only 的定义（B3）。
4. **消息停留时间**：Mac X：“It sat there for two hours”；BI：“I had already deleted it by that point.” 次序/时长未交代；BI 未写明消息发出时刻。
5. **“last night”的修复无官方说明**（B2）：官方 changelog 无对应条目，BI 里是 Mac 转述 “Grok realized ... They implemented and shipped a solution last night.”；runtimewire 只转引。
6. **时间线疑点**：Mac 称“约 8 月底”建 CFO 并读银行；官方 Finance integration 宣布于 9/27 北京时间（9/26 EDT）；官方文档无该集成页；接入方式未说明。
7. **产品设计层面**：官方文档白纸黑字 “Do not use separate Bots as a security boundary” 与 “Installed connectors are account-wide”（B3）；官方卖点是同一账户下的多 Bot 协作；文档同时称 Slack 草稿 “can” 出现，群聊里不出现草稿/审批卡，且 v0.64.0 起可 “Disable drafts for this Bot, so that Bot sends messages directly”。Mac 是否开启了某项自动放行规则，未交代。
8. **“成因”责任的另一面**：Musk 8/26 回复 Teslaconomics “Try it out. If Grok Bot messes up, we will make you whole.”，Mac 把它读成“Elon even said to connect your bank account”；Musk 10/6 另有长帖称自己接了 30 个账户，“Grok @Bot can be your personal chief financial officer!”（27.9M views）。这是官方人物个人帖，不是公司公告，也不是对此事的回应。
9. **事件真实性与动机质疑**：MAMware 引述社区注（“fake / engagement farming”）、Futurism “we were unable to independently verify his claims” 与 Mac 在建 agentpicnic。Mac 回应 “not fake at all, genuinely happened”。WSJ（可见首段）写员工因 “team Slack message that had a screenshot attached” 提醒他，与 BI 的“head of product 私信提醒”大体相容但不同：WSJ 写的是附截图的团队 Slack 消息，BI 写的是 “Slack chat titled "Exec-team"”。
10. **HN 评论线索**（Algolia 的 comment 没有得分字段，只能给链接与回复链长度，无法给分）：
   - https://news.ycombinator.com/item?id=50033725 （jameshart）：“Also somewhat reckless with mixing access to his personal finances with access to his work communications systems. Even without agents being involved that’s a recipe for trouble.”（有 1 条回复）
   - https://news.ycombinator.com/item?id=50034209 （sherburt3）：“So he gave an AI agent read access to all his personal financials and then gave read/write access to his company messaging app. WHAT DID YOU THINK WAS GOING TO HAPPEN?”（此句对 “connected Slack account” 的权限类型是评论者推断，原文未写 read/write）
   - https://news.ycombinator.com/item?id=50035127 （panarky）：“You can't stop an agent from leaking your information by telling it not to leak your information. To prevent information from moving between your work domain and your personal domain ... the agent for one domain must be physically and deterministically blocked from accessing information in oth[er]...”（节选；完整句见 `b-hn-50033517.json`）
   - https://news.ycombinator.com/item?id=50035200 （sunaookami）：“Sounds incredible brittle, this should be linked via some permanent group ID (dunno if Slack has that) and permissions to only post there.”（指向 “same name” 问题）
   - https://news.ycombinator.com/item?id=50034086 （tempest_）：“"Agents" can be very "persistent" and when not sand boxed appropriately can get at things they were not meant to get at.”（节选；原文内层为双引号）
   - https://news.ycombinator.com/item?id=50034280 （orf）：“Paywalled. What specific details did it hand out? ... Bank details are supposed to be given out - that’s how you send/receive money.”（对标题 “bank details” 的质疑；回复链 https://news.ycombinator.com/item?id=50035921 指出美国可凭账号+路由号扣款）
   - https://news.ycombinator.com/item?id=50033584 （speakingmoistly）：“Feels like someone has AI stock they need to see go up.”（对动机的质疑，无事实依据，线索价值低）

## 十、扫描说法勘误

| 扫描/初步核对的说法 | 实际 | 依据 |
|---|---|---|
| BI 是“首发”或“采访来源” | Mac 在 BI 之前已在 X 发出完整线程（10/6 23:02 北京首帖），BI 与 WSJ 都在 10/9（北京）刊出；Futurism 10/7、Cybernews 10/7 在 BI 之前 | B4/B5 时刻 |
| “WSJ 原始采访” | WSJ 10/9 07:00 北京刊出，可见首段写 Mac “had been using a personal AI agent since August” 与 “a team Slack message that had a screenshot attached”；是否为“原始采访”不可核（全文付费墙） | b-wsj-visible.txt |
| “公司产品负责人在 Slack 私信提醒” | BI 一致：“Our head of product sent me a DM on Slack.”；Mac 线程写 “A teammate DM'd me "heads up."”；WSJ 写 “pinged by one of his employees” | B1/B4/B5 |
| “私人群叫 My Personal Exec Team ‘同名’” | BI 里两个名字不同（My Personal Exec Team / Exec-team）却说 “the channels had the same name”；Mac 的 Bot 聊天截图中 Bot 称 “Exec Team”“#exec-team” | B1/B4 |
| “Grok 修复：用户须明确授权才能把信息发到其他频道” | 只来自 BI 里 Mac 的转述；官方 changelog/文档/新闻均无对应文字 | B2 |
| 扫描称 “Grok Bot 10/9 宣布逐步开放 `@mail.grokbot.com` 邮箱（9to5Mac）” | 本组未核（不属于本组清单） | — |
| 初步说“CFO 智能体发错频道” | Mac 在 X 上同时说 “this specific Grokbot agent didn't have Slack. But another one did.”（两种说法并存） | B4 |
| 初步核对未提：官方 Finance integration | 官方 @bot 9/27 03:54（北京）宣布 “Grok Bot now connects to your finances”，比事件早 4 天 | B3 |

## 十一、引文核对说明（给下游反向核验）

对本文件里的全部英文引文做过机械逐字核对（对 `b-*.txt`、`b-xai-docs-md/*.md`、fxtwitter/HN JSON 做规范化后子串匹配，规范化只处理弯引号、空白与 JSON 转义）。未直接命中的 25 个片段属于以下几类，原文无误但不是单行子串：

- BI 句子被存档里的 `[链接]` 标注切开（如 “I set up CFO using Grok Bot [url] around the end of August.”）；
- 文档列表句是把 markdown 列表项拼成一句（原文分行，如 “Every Bot on your account uses the same computer:” 后接四个要点；“Prefer explicit boundaries for:” 后接要点）；markdown 加粗标记（`**Send email**`）在本文中去掉；
- 图内文字：16 号截图里 Mac 与 Bot 的聊天文字（“Found it: ...”“checking Exec Team”）只在图里，不在文本存档；BI 页面时间 “Oct 9, 2026, 4:43 AM GMT-4” 只在渲染页（13 号截图），文本存档里只有 JSON-LD 的 UTC 时间；
- 只在 HTML meta / 搜索结果的标题：Cybernews og:title、NewsBytes 标题（后者未取到页面）；
- 搜索摘要或本文件自己的概括语（勘误表里的“扫描说法”列），不是引文。

## 十二、假设与待 Claude/Develata 决定

- `b-fx-2107495338873389121.json`（Musk 10/6 长帖）含他自报的持仓比例与账户数，属公开帖（约 2,790 万浏览），不违反派工单对 Mac 的限制，但是真实个人的财务信息进了可入库 JSON；本组未删、未在 evidence 里摘录其数字，留待决定是否保留。
- 13 号截图为压缩版面，隐藏了广告位、站内推广块与头图（Mac 的肖像照）；若要保留头图需重截。
- 20 号只截 Mac 首帖文字部分，因为其附图是带部分遮盖数字与同事账号的 Slack 截图。
