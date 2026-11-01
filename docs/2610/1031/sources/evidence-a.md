# 1031 期 A 组取证：ChatGPT 与 Codex 共用额度

执行方：claude-sonnet-5-5。时间一律北京时间（UTC+8），括号内为原时区。抓取时刻见 `capture-log-a.md`。
级别按 `EDITORIAL.md` 事实分级：L1 官方页，L2 员工发言，L4 权威媒体，L5 二手媒体，L6 社交转述。
help.openai.com 页对 curl 与 headless Chrome 都是挑战页或空白；本组用有界面 Chrome（全新临时 profile，不读 Cookie，未点任何验证控件）经 CDP 渲染取文本，脚本 `a-cdp-shot.mjs`，页面里的 `updatedAt` 是绝对时间。

## 一句话结论（只陈述证据）

- 官方一手有两处：帮助页 `20001625`（L1）与用户转贴的订阅邮件（L6，邮件原件不是公开页面，只有 HN 转贴与 BleepingComputer “seen by” 截图）。帮助页写“Starting in December”，邮件写“in the coming weeks”，二者措辞不同，官方均未给具体日期。
- 帮助页点名的套餐是 Plus、Pro、Business；Free、Go、Enterprise、Edu 在该页未提及。
- 本页 L1 原句“Instant chat will not draw from your usage”；邮件原句“Unlimited Instant is included in your plan and will not draw from your usage”。各档 token 预算、折算表、credits 价格，该页与相邻官方页均未公布。
- Work 与 Codex 在官方页上**早已**共用额度（Codex 定价页、personal credits 页），所以新变化的实际内容是把常规 Chat 里计量的部分并进来；这一点官方没有直接写成一句话，是对照多页得出的推断（见“可能的吠点”）。

## A1 合并时间与套餐范围

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1.1 | 生效时间 | L1 | https://help.openai.com/en/articles/20001625-understanding-chatgpt-and-codex-usage-limits | “Starting in December, Plus, Pro, and Business plans’ usage allowance will be shared across ChatGPT and Codex as we bring the agentic capabilities of Work into the main ChatGPT experience.” | images/02-help-usage-start-december.png、01-help-usage-full.png | 只写月份，未写具体日期，未写分批。页面数据 `updatedAt`=1793463468.058，即 2026-11-01 00:17:48 北京（2026-10-31 16:17:48 UTC）；页头显示相对时间 “Updated: yesterday”（抓取时 11-02 02:33 北京）。页面是否在此前已存在、首次发布时刻：未找到（Wayback 当时离线） | 已找到 |
| A1.2 | 套餐范围 | L1 | 同上 | 同 A1.1：点名 “Plus, Pro, and Business plans”。全文未出现 Free、Go、Enterprise、Edu | 同上 | 同页“Related articles”列出 “ChatGPT Business models and limits”。Enterprise/Edu 单独有 token-based / flexible pricing 页，该页没说合并（见 `a-rendered-codex-plan.txt` 与 `a-news-note.txt` 作者也写“EnterpriseやEduの扱いには触れていない”，L5） | 已找到（范围=三档；其余套餐“未提及”） |
| A1.3 | 订阅邮件（版本一） | L6 | HN https://news.ycombinator.com/item?id=50324512 （用户转贴，邮件发件人写 noreply@email.openai.com，原件非公开） | Subject: “A simpler way to put ChatGPT to work” ；“In the coming weeks, you’ll be able to delegate work directly to ChatGPT — from building sites and presentations to taking action on the web — without switching to the Work tab.” ；“your use of ChatGPT and Codex will draw from the same allowance in the coming weeks.” | 无（邮件无公开页） | HN 帖 2026-11-01 19:13:45 北京（11:13:45 UTC），1 分 2 评。存档 `a-hn-50324512.json`。转贴者自己写“the second one received a little different email”，即同一人收到两版 | 部分支持（只有转贴） |
| A1.4 | 订阅邮件（版本二） | L6 | HN https://news.ycombinator.com/item?id=50326057 与 https://news.ycombinator.com/item?id=50324512 内评论 50324763 | Subject: “Chat and Work are coming together in one experience.” ；“In the coming weeks, we’ll start testing a combined experience for Chat and Work tabs in ChatGPT with a small group of users. We’ll use their feedback to improve the experience before bringing it to more people.” ；“Codex will remain a separate experience in the ChatGPT desktop app.” ；“your use of ChatGPT and Codex will draw from the same allowance in the coming weeks.” | 无 | 50326057 发于 2026-11-01 22:30:44 北京（14:30:44 UTC），转贴者称“the actual announcement email I got yesterday”。BleepingComputer（L4）称 “In an email seen by BleepingComputer”，并配邮件截图，措辞与此版一致 | 已找到（二次来源互相印证：HN 转贴 + Bleeping） |
| A1.5 | “12 月”与“coming weeks” | — | 以上 A1.1、A1.3、A1.4 | 帮助页：“Starting in December”；邮件：“in the coming weeks”（共用额度一句）；版本二另写 Chat/Work 合并体验“we’ll start testing … with a small group of users” | — | 两者不是同一句话：帮助页给月份，邮件给“数周内”，且“小范围测试”只修饰 Chat/Work 合并界面，不修饰共用额度。字面上不矛盾，也没有官方文字说明哪天生效。HN 评论者 dataking（50324156）指出帮助页“is a help page, not an announcement” | 部分支持 |

## A2 计量方式

| # | 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|---|
| A2.1 | 按 token 计量，随模型与任务变化 | L1 | help 20001625 | “Your usage for ChatGPT and Codex will be measured in tokens, the units of information a model processes and generates. How much of your allowance you use will depend on the model you choose and the task you ask it to do. More advanced models and complex tasks generally use more of your allowance.” | images/03-help-usage-tokens-instant.png | 是“generally”，没有数字 | 已找到 |
| A2.2 | 是否公布各档预算、折算表 | L1 | help 20001625 及相邻页 | 该页没有各档 allowance 数字、没有折算表、没有示例。邮件：“Usage will depend on model and task, with more advanced models and complex tasks using more of your allowance.” | — | 相邻官方页有的数字属于**现有** Work/Codex 机制，不是合并后的预算：Codex 定价页 https://developers.openai.com/codex/pricing 有“Credits per 1M tokens”表（如 GPT-6 Astra 250 / 25 / 1,250 credits，对应 input / cached input / output）和 “A typical GPT-5.6 Sol task may use 2-15 credits.”；Business 页有 Standard 席位“local messages per 5-hour window”表（GPT-6 Astra 5-45 等）。存档 `a-rendered-codex-pricing-rendered.txt`（渲染）、`a-codex-pricing.txt`（fetch）、`a-rendered-business-models.txt` | 未公布（合并后的各档预算与折算） |
| A2.3 | 旧体系：Work 与 Codex 已共用 | L1 | https://developers.openai.com/codex/pricing | “ChatGPT Work and Codex share usage. ChatGPT Work usage inside ChatGPT uses the same pricing, credits, and usage limits as Codex.” | images/08-codex-pricing-share-usage.png | 另见 https://help.openai.com/en/articles/12642688 “For Plus and Pro, Codex, ChatGPT Work, Excel, and PowerPoint can share the same agentic usage allowance when those features are available on your plan.”（images/07）；Codex 计划页“Regular Chat usage isn’t included.”（该句限定“credit usage for individual Work and Codex chats”的显示，见 `a-rendered-codex-plan.txt` 第 176 行） | 已找到 |

## A3 Instant 不扣额度

| # | 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|---|
| A3.1 | 帮助页原句 | L1 | help 20001625 | “Instant chat will not draw from your usage.” | images/03 | 页内没有定义 Instant、没有“公平使用”类限定 | 已找到 |
| A3.2 | 邮件原句 | L6 | HN 50324512 / 50326057 | “Unlimited Instant is included in your plan and will not draw from your usage.” | 无 | 邮件用 “Unlimited Instant”；帮助页用 “Instant chat”。邮件未写“公平使用”或滥用限制 | 已找到（转贴） |
| A3.3 | Instant 指什么 | L1 | https://help.openai.com/en/articles/20001354-gpt-6-and-other-models-in-chatgpt | “In the model menu, GPT-6 replaces Latest. … When GPT-6 is selected, GPT-6 Sol powers Instant, Medium, High, and Extra High; Pro uses GPT-6 Pro, powered by GPT-6 Astra.” ；“Instant — Fast responses for everyday questions. Free and Go use GPT-5.6 Luna.” | images/06-models-instant-limits.png（含 Instant limits 段） | 即 Instant 是模型选择器里的**推理档位**（thinking level）之一，Plus/Pro 选 GPT-6 时由 GPT-6 Sol 驱动，Free/Go 用 GPT-5.6 Luna；不是单独的模型名。页面 `updatedAt`=1793396884.158，2026-10-31 05:48:04 北京 | 已找到 |
| A3.4 | 其他限制 | L1 | https://help.openai.com/en/articles/12003714-chatgpt-business-models-and-limits （Business 页） | “Virtually unlimited access applies to Instant chat and remains subject to safeguards against misuse. It does not make reasoning, Pro, ChatGPT Work, Codex, or other tools unlimited. Automatic reasoning may use your reasoning allowance even while Instant remains selected.” | images/05-business-instant-note.png | 这是 **Business** 页，不是 Plus/Pro 页；页面 `updatedAt`=1793488531.23，2026-11-01 07:15:31 北京（2026-10-31 23:15:31 UTC），该“Note”段何时加入：未找到。Plus/Pro 的对应原文：models 页只写 “Free and Go users have unlimited everyday text chats, subject to abuse-prevention safeguards. Separate limits continue to apply to file uploads, image generation, voice, data analysis, and other tools.”（images/06）；ChatGPT 定价页写 “Unlimited text chats subject to abuse guardrails.”（Free、Go 卡片）和对比表 “Everyday text chats / Unlimited*”，脚注 “*Usage must be reasonable and comply with our policies. Usage refers to Chat limits.”（`a-rendered-chatgpt-pricing.txt`，价格数字未渲染出来） | 部分支持（Plus/Pro 的 Instant 滥用限制措辞找到的是 Free/Go/Business 页与脚注，没有 Plus/Pro 专页的同句） |
| A3.5 | “自动加推理”与 Instant | L1 | models 页 FAQ | “On Plus and Pro plans, the Higher intelligence setting isn’t available on ChatGPT web. Use the model picker to choose how much ChatGPT thinks.” ；“In ChatGPT Business, you can find Higher intelligence under Settings > General. This setting controls whether ChatGPT automatically uses additional thinking while Instant is selected.” | 未截（文字见 `a-rendered-gpt6-models.txt`） | 即“自动加推理会占额度”只在 Business 页有官方原句；Plus/Pro 网页版没有该设置 | 已找到 |

## A4 用完之后

| # | 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|---|
| A4.1 | 选项 | L1 | help 20001625 | “If you reach your limit, you can wait for it to reset, upgrade your plan, or buy credits to continue using eligible features.” ；“We expect most users won't need to wait for their usage to reset more often than they do now.” ；“You will be able to see your combined remaining usage and reset times in Settings → Usage.” | images/04-help-usage-limit-options.png | “eligible features”未在该页定义；“expect most users” 是预期，不是承诺 | 已找到 |
| A4.2 | credits 适用功能 | L1 | https://help.openai.com/en/articles/12642688 （Personal plans） | “Credits let you continue using eligible features after reaching your plan’s included limits. Supported features include Codex, ChatGPT Work, Word, Excel, and PowerPoint, depending on your plan and account.” ；“Available purchase amounts, prices, payment methods, spending limits, and feature availability can vary by account, region, and plan.” | images/07-credits-how-they-work.png | 页面 `updatedAt`=1792555426.129，2026-10-21 12:03:46 北京，早于合并公告；列出的功能里**没有**普通 Chat。该页不写 credits 的美元价格 | 部分支持（适用功能为现有清单；合并后是否扩到 Chat 官方未写） |
| A4.3 | credits 价格 | L1 | 同上；https://developers.openai.com/codex/pricing | 个人套餐页：“prices … can vary by account, region, and plan.” ；Codex 定价页：“Credit purchase prices and applicable discounts depend on your plan or agreement.” | — | 只有 “credits per 1M tokens” 的换算表，没有 credit→美元 价格 | 未公布 |
| A4.4 | Settings → Usage | L1 | help 20001625；12642688 | help：“You will be able to see your combined remaining usage and reset times in Settings → Usage.”；credits 页：“buy credits from Settings > Usage in ChatGPT or Usage & Billing in the Codex app.” | images/04、07 | 该入口已在 credits 页出现（现有） | 已找到 |

## A5 官方其他渠道

| # | 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|---|
| A5.1 | ChatGPT 发行说明 | L1 | https://help.openai.com/en/articles/6825453-chatgpt-release-notes | 页面最新条目是 “October 30, 2026 / Account switching on iOS and Android”，其后 10-29 “More Work and Codex usage on Pro and Business Premium Plans”（Pro 翻倍、取消 Pro 200 下调）。**没有**共用额度或 Chat/Work 合并条目 | 未截（无条目可截） | 页面 `updatedAt`=1793399524.18，2026-10-31 06:32:04 北京；全文存档 `a-rendered-release-notes.txt`（228 KB）。10-29 条目属 1030 期已发内容，与本期区分 | 未找到一手来源（发行说明里没有） |
| A5.2 | Codex changelog | L1 | https://developers.openai.com/codex/changelog | 最新条目日期 2026-10-29；全文 grep “same allowance / shared allowance / combined / draw from” 无相关条目（“unified exec” 为无关代码变更） | 未截 | 存档 `a-codex-changelog.txt`（988 KB，接近 1 MB 上限）；html 只存本地 | 未找到一手来源 |
| A5.3 | OpenAI 员工发言：Tibo | L2 | https://x.com/thsottiaux/status/2115588390414581916 （fxtwitter JSON：`a-fx-2115588390414581916.json`） | “Team is working on this but it’s not rolling out just yet, sorry for any confusion. Plan is to bring together best of both Chat / Work capabilities together at some point this year, but we’re going to proceed carefully here and take our time. We’ll have more to share soon.” | images/09-tibo-not-rolling-out.png | 发于 2026-10-29 07:35:52 北京（2026-10-28 23:35:52 UTC）。回应的是 Angaisb_ 的帖子“ChatGPT is unifying Chat and Work / I wonder how that changes usage limits”。**该帖谈 Chat/Work 合并界面，没有谈共用额度**；也早于帮助页。Tibo 的身份（Codex 相关员工）由 1030 期存档的帖线索，本组未再单独核职位原句 | 已找到（帖本身）；与共用额度无直接关系 |
| A5.4 | 第三方先发现的线索 | L6 | https://x.com/btibor91/status/2115927832069292214 （`a-fx-2115927832069292214.json`） | “ChatGPT web app now has a new hidden shared usage limit checkout disclaimer - "Starting in December, plan usage will have a shared limit across all of ChatGPT and Codex"” | images/10-btibor-checkout-disclaimer.png | 发帖人 Tibor Blaho 是长期追踪 OpenAI 产品动态的第三方，**不是 OpenAI 员工**（本组未查到其任职；中文媒体称其为追踪者，L5）；发于 2026-10-30 06:04:41 北京（10-29 22:04:41 UTC）。早于帮助页更新约 18 小时。结账页原图为页面内嵌，只在其推文配图里可见 | 已找到（L6 线索） |
| A5.5 | OpenAI 官方账号 / 员工是否发过共用额度公告 | — | 以 fxtwitter 取得的帖：TokenGremlin（`a-fx-2116609772396920937.json`）、Mr_Salio（`a-fx-2116625475015528759.json`）均为第三方转述，写 “OpenAI quietly updated its help page” | 本组无 X 搜索能力，只能核实已知 ID；未发现 OpenAI 官方号或员工直接宣布共用额度的帖 | — | 不能证明“不存在”，只能写“本组未找到” | 未找到一手来源（官方 X 公告） |
| A5.6 | Chat 与 Work 合并测试的官方原文 | L1/L6 | 无公开页；仅订阅邮件（A1.4）与 BleepingComputer 邮件截图 | 版本二：“we’ll start testing a combined experience for Chat and Work tabs in ChatGPT with a small group of users.” 帮助页只写 “as we bring the agentic capabilities of Work into the main ChatGPT experience” | 无 | 帮助页没有“小范围测试”一句 | 部分支持（只有邮件，L6） |
| A5.7 | 两版邮件的差异 | L6 | HN 50324512（版本一）与 50326057/50324763（版本二） | 版本一首段：“In the coming weeks, you’ll be able to delegate work directly to ChatGPT … without switching to the Work tab.”；版本二首段：“In the coming weeks, we’ll start testing a combined experience for Chat and Work tabs … with a small group of users.” 版本一无 “Codex will remain a separate experience in the ChatGPT desktop app”（转贴摘录处至“Settings → Usage”之前全文可见，未含这句）。 | 无 | “A shared allowance for ChatGPT and Codex” 整段在两版里逐字相同（我逐字比对了转贴文本）：“As part of this unified experience, your use of ChatGPT and Codex will draw from the same allowance in the coming weeks. We’re preparing carefully for this transition to help prevent disruption to your workflows. We expect most users won’t need to wait for their usage to reset more often than they do today. Usage will depend on model and task, with more advanced models and complex tasks using more of your allowance. Unlimited Instant is included in your plan and will not draw from your usage.” 转贴者称两封来自不同账号（一封来自其第一个 Plus 账号） | 已找到（差异只在首段，额度段相同） |

## A6 与 Pro 帮助页的关系

| # | 说法 | 级 | 一手来源 URL | 对比结果 | 截图文件 | 条件 | 状态 |
|---|---|---|---|---|---|---|---|
| A6.1 | 现存 Pro 页是否已改写 Work and Codex allowance 的描述 | L1 | https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers | 本组渲染文本 `a-rendered-pro-tiers.txt`（`updatedAt`=1793293375.208，2026-10-30 01:02:55 北京）与 1030 存档 `docs/2610/1030/sources/b-pro-help-rendered.md` 做 diff（去掉首行注释）：正文**逐字相同**，只有页头相对时间从 “Updated: 2 days ago” 变为 “Updated: 3 days ago”，以及页尾“Related articles”里原来的 “API credits benefit for eligible ChatGPT subscriptions” 条目换成 “ChatGPT release notes”（相关文章推荐列表，随渲染变化）。`updatedAt` 与 1030 期完全一致，即页面内容自 10-30 起未改 | 无 | Pro 页仍写 “Pro 100 and Pro 500 now include more Work and Codex usage. All Pro 200 subscriptions now include the same Work and Codex allowance.”，没有提共用额度或 12 月 | 已找到（无差异） |
| A6.2 | 与 GPT-6 Pro 在 Chat 里的限制 | L1 | models 页 | “GPT-6 Pro has usage limits. A Pro subscription does not include unlimited use of the GPT-6 Pro model. ChatGPT Work and Codex have separate allowances.” | 未截（`a-rendered-gpt6-models.txt`） | 该页仍写 Pro 模型在 Chat 里有**单独**限额，且 “ChatGPT Work and Codex have separate allowances”。合并后这些 Pro 模型消息是否进共用池，官方未写 | 已找到（文档目前未与合并一致，可能是过渡期） |

## A7 热度与转述

热度（`a-heat.json`，取得于 2026-11-02 02:35 北京）：

| 项 | 数据 |
|---|---|
| HN 50320588 “ChatGPT and Codex usage limits merging” | 207 分 / 149 评，发帖 2026-11-01 05:49:07 北京（2026-10-31 21:49:07 UTC），链接指向帮助页 20001625 |
| HN 50321622 “Tell HN: ChatGPT and Codex usage will be combined” | 6 分 / 3 评，11-01 08:28:26 北京 |
| HN 50324512 “A simpler way to put ChatGPT to work (A shared allowance for ChatGPT and Codex)” | 1 分 / 2 评，11-01 19:13:45 北京（转贴邮件版本一） |
| HN 50326057 | 50320588 下的评论，转贴邮件版本二 |
| Reddit | 搜索接口返回登录页（old.reddit.com 要求登录），记失败，未取得 |
| 参考：X 帖 | Tibor Blaho 10-30 帖：2.1K 赞 / 769,310 浏览（fxtwitter 取得 11-02 02:4x 北京）；codexneo4 帖 10-31：1,321 赞 / 753,754 浏览；TokenGremlin 帖 11-01 03:14：1,602 赞 / 221,401 浏览 |

报道（代表性，逐字摘标题；发布时刻按页面所写，原时区括号内）：

| 发布方 | 标题 | URL | 发布时刻 | 存档 |
|---|---|---|---|---|
| BleepingComputer（L4） | “OpenAI says it's doubling Pro usage limits and bringing Work features to Chat” | https://www.bleepingcomputer.com/news/artificial-intelligence/openai-says-its-doubling-pro-usage-limits-and-bringing-work-features-to-chat/ | 页面写 “November 1, 2026 08:53 AM”，时区页面未写 | a-news-bleeping.* |
| Crypto Briefing（L5） | “OpenAI to merge ChatGPT and Codex token allowances in December 2026” | https://cryptobriefing.com/openai-unifies-chatgpt-codex-token-allowance/ | 页面写 “Sat 31 Oct 2026 · 16:34 EDT”，即 11-01 04:34 北京 | a-news-cryptobriefing.* |
| IT时代网（腾讯新闻转载，L5） | 标题页内未单列；首句 “IT时代网11月1日消息，OpenAI发布公告称，自12月起将Work的智能体能力引入ChatGPT主要体验” | https://news.qq.com/rain/a/20261101A03PH300 | 2026-11-01 08:49 | a-zh-qq-ithome.* |
| 爱范儿（腾讯新闻转载，L5） | “ChatGPT要和Codex共用额度，免费重置时代结束了” | https://news.qq.com/rain/a/20261101A05QJV00 | 2026-11-01 12:15 发布（北京，页面写“发布于广东”） | a-zh-qq-reset.*、a-rendered-ifanr-qq-rendered.txt |
| 新智元（网易号，L5） | “ChatGPT要和Codex抢额度了！奥特曼一声令下，12月Token全部合池” | https://m.163.com/news/article/LAKQNPMU0511ABV6.html | 2026-11-01 07:57（北京） | a-zh-163.* |
| 搜狐号（L5） | “ChatGPT与Codex额度合并：无限聊天时代结束？” | https://www.sohu.com/a/1095632433_122066678 | 2026-11-01 08:22 | a-zh-sohu.* |
| 新浪财经（L5） | “Tibo 28连更后，OpenAI合并额度池，计费聊天开始了” | https://finance.sina.com.cn/roll/2026-11-01/doc-inixeyiz3198259.shtml | 日期 2026-11-01（页内时刻未取出） | a-zh-sina-tibo.* |
| note（techno_news，L5，日文） | “12月からChatGPTとCodexの利用枠がひとつに　Plus・Pro・Businessが対象、Instantのチャットは枠を使わない” | https://note.com/techno_news/n/n18077c2e1678 | 2026-10-31 14:42（Bing 新闻列表所示，时区不明，未核） | a-news-note.* |

被屏蔽：MSN “OpenAI Likely to Make ChatGPT and Codex Share Usage Limits Starting in December”（Bing 列出 2026-10-30 01:21 GMT）正文需 JS，barking fetch 判 blocked，未取得内容，不引用。

## 可能的吠点（线索，须回到一手核实后才进正文）

1. 帮助页只写月份 “December”，没有日期、没有各档预算、没有折算表；邮件只写 “in the coming weeks”（A1、A2.2）。出处：help 20001625；HN 50324512。
2. 帮助页点名 Plus、Pro、Business；Free、Go、Enterprise、Edu 在该页没提（A1.2）。出处：help 20001625。
3. “Instant 不扣额度”在 Business 页被限定为 “Virtually unlimited … subject to safeguards against misuse”，且 “Automatic reasoning may use your reasoning allowance even while Instant remains selected”（A3.4）。出处：Business 页 12003714（images/05）。Plus/Pro 页没有同句，但 Plus/Pro 网页版没有 “Higher intelligence” 设置（A3.5）。
4. Work 与 Codex 官方页早已写共用同一额度（A2.3）；因此“合并”新增的是常规 Chat 里计量的部分，不是 Codex 与 Work 的合并。官方页没有把这一点写成一句；这是对照三处官方页的推断。出处：Codex 定价页、personal credits 页、help 20001625（“as we bring the agentic capabilities of Work into the main ChatGPT experience”）。
5. GPT-6 Pro 在 Chat 中有单独的消息限额，models 页仍写 “ChatGPT Work and Codex have separate allowances”；合并后这些限额与共用池的关系官方没写（A6.2）。HN 评论者 yurimo（https://news.ycombinator.com/item?id=50322773）与 0x_rs（https://news.ycombinator.com/item?id=50324129）担心 Pro 模型的“解耦计费”被取消，L6。
6. credits 适用的 “eligible features” 现有清单不含普通 Chat（A4.2）；credit→美元价格不在公开页（A4.3）。
7. “We expect most users won’t need to wait … more often than they do now”是预期，不是保证（A4.1）。HN 评论 lelandfe（https://news.ycombinator.com/item?id=50326849）：“With the implied, "...but some will"”（L6）。
8. 抓取到的官方页日期：帮助页本身写“Updated: yesterday”，绝对时间是 11-01 00:17:48 北京；Tibo 10-29 的帖还说 “it’s not rolling out just yet”，那是 Chat/Work 合并界面，不是共用额度（A5.3）。

HN 评论（L6）：HN 评论得分不公开（API 的 points 为 null），下列只记评论链接，不记得分。

- https://news.ycombinator.com/item?id=50324034 jwr：“your usage will be measured in tokens, which we do not define, nor give you a way to measure them.”（质疑 token 单位与预算未公布；与 A2.2 一致）
- https://news.ycombinator.com/item?id=50322773 yurimo（35 条子评论，评论树最大）：作为研究者，因 Pro 计划里 Pro 档模型在 chat 与 codex 的计费是解耦的而成为客户。
- https://news.ycombinator.com/item?id=50324129 0x_rs：Pro 模型每周消息数有限，若“depletes usage in the normal chat and codex”则是降级（假设性判断）。
- https://news.ycombinator.com/item?id=50322784 anonymous908213：平均聊天用量远小于 agentic 编码（自述）。
- https://news.ycombinator.com/item?id=50324156 dataking：“This is a help page, not an announcement.”
- https://news.ycombinator.com/item?id=50322944 brianjking：“Am I the only one who didn't realize these limits weren't already merged since the current Codex app was released?”（与 A2.3 官方页“Work 与 Codex 已共用”呼应）
- https://news.ycombinator.com/item?id=50325300 matheusmoreira：Instant 用 Sol（与 A3.3 的 models 页一致）；称 200 美元档额度不降（与 10-29 发行说明一致）。

## 夸大说法实例

| 发布方 | URL | 原句（逐字） | 发布时刻 | 与一手的出入 |
|---|---|---|---|---|
| 爱范儿（腾讯新闻转载） | https://news.qq.com/rain/a/20261101A05QJV00 | 标题“ChatGPT要和Codex共用额度，免费重置时代结束了”；文末“无论如何，CHatGPT 无限畅聊的时代要结束了。” | 2026-11-01 12:15 北京 | 同文第二段写“Instant 聊天除外，不扣这份共享额度”；帮助页写 “Instant chat will not draw from your usage.”。“免费重置”指 Tibo 此前的全局重置（同文提及），不是官方页的措辞。截图 images/12 |
| 搜狐号 | https://www.sohu.com/a/1095632433_122066678 | 标题“ChatGPT与Codex额度合并：无限聊天时代结束？”；“对于许多ChatGPT的重度用户来说，这一消息如同晴天霹雳。过去，ChatGPT的聊天额度几乎是无限的” | 2026-11-01 08:22 | 同文也写“Instant聊天模式仍然可以免费使用”；标题用问句 |
| 新智元（网易号） | https://m.163.com/news/article/LAKQNPMU0511ABV6.html | 标题“ChatGPT要和Codex抢额度了！奥特曼一声令下，12月Token全部合池” | 2026-11-01 07:57 北京 | 正文未见奥特曼发言或决定；一手页是帮助页，署名为 OpenAI 帮助中心，邮件署名 “The OpenAI team”。（我 grep 了存档正文，除标题与无关侧栏外无“奥特曼”相关句） |
| Crypto Briefing | https://cryptobriefing.com/openai-unifies-chatgpt-codex-token-allowance/ | “Come December, every prompt and every code generation will be drawing from the same well.” ；“A heavy afternoon of Codex work will now eat into the same allowance you’d otherwise spend asking ChatGPT to rewrite your emails.” | 2026-11-01 04:34 北京（10-31 16:34 EDT） | 全文 0 处提到 Instant 豁免；官方：Instant chat 不扣。另称 “shared credit balance … measured in credits per million tokens”，帮助页未写 credits 是计量单位（写的是 tokens） |
| BleepingComputer（L4） | 见 A7 | “There's another significant change coming to Pro subscriptions.” | 2026-11-01 08:53（时区页面未写） | 帮助页点名 Plus、Pro、Business；两位转贴邮件的用户之一是 Plus 账号（HN 50324763）。同文“OpenAI says it's doubling Pro usage limits”的“doubling”指 10-29 已发的变动，与合并无关 |
| luojiuyue（X） | https://x.com/luojiuyue/status/2115957220747923952 | “对于plus&go用户,chatgpt将彻底沦为出图工具吧!!” | 2026-10-30 08:01:28 北京（10-30 00:01:28 UTC） | 以“如果是真的话”开头，针对 Tibor Blaho 的结账页发现，早于帮助页；后来帮助页写 Instant 不扣额度，且 Go 未被点名。截图 images/11 |
| codexneo4（X） | https://x.com/codexneo4/status/2116365292284641662 | “OpenAI、ChatGPTを使い続けるメリットまで完全になくすらしいです。” | 2026-10-31 11:03:00 北京（03:03:00 UTC），753,754 浏览 | 帮助页尚未更新（早于 11-01 00:17），依据是结账页提示；未截图 |

## 扫描说法勘误

1. “2026 年 12 月起 Plus、Pro、Business 合并”：帮助页写 “Starting in December” 与三个套餐，**无年份**（年份来自页面日期推断），也无具体日；与扫描一致，但“2026 年 12 月”是推断。
2. “Instant chat 不扣”：与扫描一致；帮助页用 “Instant chat”，邮件用 “Unlimited Instant”。
3. “页面只显示相对更新时间”：正确，页头是 “Updated: yesterday”，但页面数据里有绝对时间 `updatedAt`（A1.1）。
4. Claude 已知情况里“12 月与 coming weeks 是否矛盾”：二者是不同文件里的不同措辞，非同一句话的矛盾；邮件两版“共用额度”段逐字相同。
5. 已知情况里写“邮件有两版，一版另写 ‘start testing a combined experience…’ 与 ‘Codex will remain a separate experience…’”：核实无误。版本一首段另有 “delegate work directly to ChatGPT … without switching to the Work tab” 一句，已知情况未写。
6. “本号此前 0929 Pro 额度下调、1030 撤回”：发行说明与 Pro 页印证 10-29 的 “The reduction announced in our September 29 email is canceled.”；本期是新机制，区分点见“可能的吠点”第 4 条。
7. BleepingComputer 把共用额度写成“coming to Pro subscriptions”，与帮助页范围不同（见夸大说法）。

## 假设与限制

- 帮助页首次发布时刻无法确定（Wayback 离线，`a-wayback-cdx-usage.json` 实为错误页）；只能引用 `updatedAt` 和中文报道（新浪财经称帮助中心“刚刚上线”）。
- 没有任何官方文字写明“新体系下 Plus/Pro/Business 的 token 额度具体是多少”；与现行 Work/Codex 额度的关系，官方没写。
- X 帖互动数字在 fxtwitter 取得时刻的快照，会继续变化。
