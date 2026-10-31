# 1030 期 B 组取证：OpenAI 撤回 Pro 200 降额，只把降下去的翻倍（跟进 0929）

执行：Sonnet 5.5（claude-sonnet-5-5）。抓取时间北京时间 2026-10-31 16:50–17:15。时间一律北京时间（UTC+8），括号保留原时区。本机时区为美东（EDT），Chrome 截图中的帖文时间（如 12:56 PM）是 EDT。

存档文件（`docs/2610/1030/sources/`）：
- 帮助页渲染文本与页面数据：`b-pro-help-rendered.md`、`b-pro-help-article.json`（`updatedAt`）
- 发行说明渲染文本：`b-relnotes-rendered.md`；定价页渲染文本：`b-pricing-rendered.md`
- Wayback 快照：`b-wayback-20261026042106.{html,txt}`（改前）、`b-wayback-20261029170331.{html,txt}`（改后 36 秒）、`b-wayback-cdx.json`
- 帖文 JSON（fxtwitter）：`b-fx-2115850175868727621.json`（撤回帖）、`b-fx-2115851621137784832.json`（“Token Company”帖）、`b-fx-2104823812042940713.json`（0929 长帖）、`b-fx-2104951965184925941.json`（0929 倍数帖）
- 热度：`b-heat.json`、`b-hn-49919026.json`、`b-hn-50292524.json`
- 报道存档：`b-ifanr`、`b-sina`、`b-pedaily`、`b-netease-tingquan`（智东西转载）、`b-smzdm-*`、`b-bleeping`、`b-startupfortune`、`b-devto`、`b-nerdschalk`、`b-kingy`、`b-cellcog`、`b-ghacks-rendered.txt`、`b-devday-recap`（均 .html + .txt）

## 一、核心事实（一句话版）

OpenAI 帮助页与发行说明（L1）：9 月 29 日邮件宣布的 Pro 200 降额取消；原本额度较高者保留，额度较低者翻倍到与之持平；Pro 100 与 Pro 500 的 Work 与 Codex 额度各翻倍；Plus 不变；价格不变。撤回的首发载体是 OpenAI Codex 负责人 Tibo 的 X 帖（北京 10-30 00:56:07），帮助页数据 `updatedAt` 在其后约 7 分钟（北京 10-30 01:02:55）。OpenAI 官方页面不给任何绝对额度，也不给 Pro 100/200 的倍数；倍数只见于 9 月 29 日 Tibo 帖、邮件转引（HN 引文）和 DevDay 回顾页（只有 Pro 500 = Plus 的 25 倍）。

## 二、清单逐条（evidence 表）

| 编号 | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B1a | 撤回降额 | L1 | https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers | “The reduction announced in our September 29 email is canceled.” | `images/14-pro-help-existing-200.png` | 位于问答 “What happens to my existing Pro 200 subscription?” | 已找到 |
| B1b | 已有高额度者保留，低额度者翻倍到持平 | L1 | 同上 | “If you already had the higher Work and Codex allowance, you keep it. If you had the lower allowance, it has doubled to match.” | `images/14-pro-help-existing-200.png` | 帮助页没有写“有多少人属于哪一类”；也没写高/低各是 Plus 的几倍（见 B3） | 已找到 |
| B1c | 价格不变 | L1 | 同上 | “Your subscription price stays at $200/month.” “The monthly price remains $200.” | `images/14-pro-help-existing-200.png` | 前一句在“existing”问答，后一句在 “Can I subscribe to Pro 200 again?” | 已找到 |
| B1d | 页面更新时刻 | L1（页面数据） | 同上（`window.__reactRouterContext` 的 `article.updatedAt`） | `updatedAt` = 1793293375.208 | `images/13-pro-help-top.png`（页头 “Updated: 2 days ago”，Chrome 设为 en-US 的相对时间） | = UTC 2026-10-29 17:02:55 = 北京 2026-10-30 01:02:55。这是“最后一次更新”，不证明撤回文字首次出现的时刻；但 Wayback 在 UTC 17:03:31（36 秒后）的快照已含撤回文字且页头为 “Updated: 1 minute ago”，改前最近的 200 快照为 UTC 10-26 04:21:06（仍是旧文字）。0929 抓取时的 `updatedAt` 为 1790701467.746 = 北京 9-30 01:04:27 | 已找到 |
| B1e | 与 0929 版逐句对比 | L1 | 0929 存档 `docs/2609/0929/sources/e-pro-tiers-help.md` vs `b-pro-help-rendered.md` | 见下方“三、与 0929 页面的删改” | `images/13`–`20` | 另有 10-26 中间版（Wayback）可对比 | 已找到 |
| B2a | 撤回的首发公布（官方员工帖） | L2 | https://x.com/thsottiaux/status/2115850175868727621 | “We are doubling all usage limits for all Pro 100/200/500 tiers and Business Premium, effective today. We heard your kind feedback and won’t be reducing usage limits for Pro 200, so your Work and Codex usage will stay the same.” | `images/23-tibo-doubling-post.png`（X 官方嵌入渲染；嵌入卡片对长文截断，在 “continue offering” 后显示 “Show more”，全文见 fxtwitter JSON） | fxtwitter：created_at “Thu Oct 29 16:56:07 +0000 2026” = 北京 2026-10-30 00:56:07（EDT 12:56 PM，Oct 29）；开头 “Day 25/”；点赞 38,274、转发 2,643、回复 7,307、浏览 6,314,061（取于北京 10-31 约 17:00）。作者 Tibo（@thsottiaux），公开职业身份：OpenAI Codex 负责人（媒体称法：Codex and ChatGPT 负责人，Bleeping 称 “face of ChatGPT and Codex on X”） | 已找到 |
| B2b | 同晚第二帖（“重置公司”） | L2 | https://x.com/thsottiaux/status/2115851621137784832 | “Graduating from The Reset Company to The Token Company.” | 未截 | fxtwitter：Thu Oct 29 17:01:51 +0000 2026 = 北京 10-30 01:01:51；回复上一帖；点赞 8,651 | 已找到 |
| B2c | 发行说明条目 | L1 | https://help.openai.com/en/articles/6825453-chatgpt-release-notes | 标题 “More Work and Codex usage on Pro and Business Premium Plans”；“We’ve also canceled the Pro 200 reduction announced in our September 29 email. If you already had the higher Work and Codex allowance, you keep it. Starting October 29, Pro 200 subscribers on the lower allowance get twice their previous Work and Codex allowance, bringing them up to the same limit.” | `images/22-release-notes-top.png`（整段含两条日期标题）、`images/21-release-notes-1029.png` | 条目只写日期 “October 29, 2026”，无时刻、无时区；该条在页面上出现了两次（页面原样，内容相同）；发行说明页 `updatedAt` = 1793399524.18 = 北京 10-31 06:32:04（之后又改过别的条目） | 已找到 |
| B2d | OpenAI 官方 X 账号、OpenAI 新闻页是否另有公告 | — | — | 检索 WebSearch 未见 @OpenAI 或 openai.com/news 的专文；X 无法按账号检索，未做账号时间线逐条核 | — | 只表示本次检索没找到，不证明不存在；Claude 如需可在浏览器里看 @OpenAI、@OpenAINewsroom、@OpenAIDevs 的 10-29/30 时间线 | 未找到一手来源 |
| B2e | 与 10/29 截止的先后 | L1 + L2 | 0929 帮助页存档；撤回帖 | 0929 版：“you’re eligible to keep your previous included usage allowance through Oct 29, 2026 … After that date, your subscription will move to the lower included usage allowance.” 10-26 中间版：“Eligible customers can use the previous included allowance through October 29, 2026 while their Pro 200 subscription is active.” | `images/14` | 撤回帖北京 10-30 00:56（UTC 10-29 16:56，EDT 10-29 12:56 PM），在“through Oct 29”的日期内；邮件引文（HN 49919026，见 B6）写 “Starting October 30, 2026 …”，官方页都没写时区，所以“降额是否在 UTC/北京的 10-30 零点前撤回”只能按美国时区说“撤在生效日前一天”。是否有人在撤回前已被实际降过，官方未说（smzdm 文章也指出这点未讲清，L4） | 部分支持 |
| B3a | Pro 100、Pro 500 翻倍 | L1 | 同 B1 | “Pro 100 and Pro 500 now include twice their previous Work and Codex allowance. On Pro 200, only the lower allowance doubled.” | `images/16-pro-help-increased-allowance.png` | “previous”指改版（9-29）后的额度，不是一个月前的额度；官方页没给“previous”是多少 | 已找到 |
| B3b | Plus 不变 | L1 | 同 B1 | “Plus usage is unchanged.” | `images/16-pro-help-increased-allowance.png` | 官方没说 Plus 的绝对额度 | 已找到 |
| B3c | 周度 GPT-6 Pro 消息上限（聊天）是另一件事 | L1 | 同 B1 | “Pro 100 and Pro 500 keep their current weekly GPT-6 Pro message limits in Chat. On Pro 200, the lower weekly limit now matches the higher one. If you already had the higher limit, it stays the same. These message limits are separate from your Work and Codex allowance.” | `images/17-pro-help-weekly-pro-limit.png` | 翻倍只指 Work 与 Codex 额度；聊天里 GPT-6 Pro 的周消息上限在 Pro 100/500 不变，Pro 200 把低的拉到高的 | 已找到 |
| B3d | 帮助页是否给出各档相对倍数（结论：官方页无倍数） | L1 | 同 B1；定价页 https://chatgpt.com/pricing | 帮助页：“Pro 200 includes more usage than Pro 100. Pro 500 offers the highest included usage of the three plans.” 定价页：“Your choice of 3 usage tiers”“Expanded Codex usage”“Longer Codex and ChatGPT Work sessions” | `images/20-pro-help-plan-table.png` | 帮助页与公开定价页都只有“更多/最高”，没有“Pro 200 = Plus 的几倍”。定价页价格数字为动态元素，渲染文本里空缺，只存了文字版（`b-pricing-rendered.md`）；登录态才可见的内容未存 | 已找到 |
| B3e | Pro 500 = Plus 的 25 倍（改版时的官方说法） | L1 | https://openai.com/index/devday-2026-recap/ | “For those who want even more usage and speed, the new Pro 500 plan offers our highest usage allowance at 25 times the ChatGPT Plus allowance and includes access to Ultrafast.” | 未截（0929 期有 `docs/2609/0929/images/21-devday-recap-pro500.png`） | 唯一官方页上的倍数；翻倍后官方页没有更新这个数字，“50 倍”只是媒体推算 | 已找到 |
| B3f | Pro 100 = 5 倍、Pro 200 = 10 倍（新）、老用户暂留 20 倍（0929 员工帖） | L2 | https://x.com/thsottiaux/status/2104951965184925941 | “Plus = 1X / Pro 100 = 5X / Pro 200 = 10X … If you have an existing plan you will keep the 20X multiplier for a bit” | 0929 期 `images/18-tibo-pro-multiplier.png` | fxtwitter：Tue Sep 29 15:10:31 +0000 2026 = 北京 9-29 23:10:31。倍数不见于任何官方页 | 已找到 |
| B3g | 邮件（9-29）：“Starting October 30, 2026 … decrease from 20 times to 10 times the ChatGPT Plus allowance” | L6（HN 用户引文） | https://news.ycombinator.com/item?id=49919026 | “Starting October 30, 2026, your included usage in ChatGPT Work and Codex will decrease from 20 times to 10 times the ChatGPT Plus allowance. … the new Pro 500 tier offers our highest usage allowance, with 25 times the usage of Plus.” | 无 | 邮件原件我们没有，只有 HN 帖主引文（10-01 08:05 UTC 发帖）；nerdschalk 也转述“limits start on October 30, 2026”并称邮件含一次性 62,500 usage credits（$2,500，12-31 到期），均为二手 | 部分支持 |
| B4a | 额度翻倍不保证任务翻倍 | L1 | 同 B1 | “The amount of work you can complete still depends on the model, task, and settings. A larger allowance does not guarantee twice as many completed tasks.” | `images/16-pro-help-increased-allowance.png` | | 已找到 |
| B4b | 消耗倍率（Fast、Ultrafast） | L1 | 同 B1 | “Compared with Standard mode for the same model, Fast mode uses your included allowance at 2.5 times the rate, and Ultrafast for GPT-6 Astra and GPT-6.1 Sol uses it at 8 times the rate. These are allowance consumption rates, not guarantees of faster performance. When using purchased credits, Ultrafast is charged at 6× the Standard credit rate for the selected model.” | `images/19-pro-help-rates.png`；Ultrafast 另见 `images/18-pro-help-ultrafast.png` | 额度单位官方页未公开；用量取决于 “the model, the amount of context processed, reasoning, tool use, information retrieval, caching, and speed mode” | 已找到 |
| B4c | 绝对额度官方有没有给（结论：官方没给） | L1/L4 | 同 B1；Bleeping、gHacks、Startup Fortune | 帮助页全文没有任何绝对额度或 token/任务数；Bleeping：“OpenAI won't provide exact numerical usage limits for each tier”；gHacks：“OpenAI does not publish exact numerical usage limits for each tier, so subscribers cannot verify the doubling against a published figure.” | 无 | 没给；第三方两句是媒体陈述，不是官方 | 已找到 |
| B4d | Ultrafast 仅 Pro 500；买 credits 不解锁 | L1 | 同 B1 | “Among Pro plans, Ultrafast is available only on Pro 500.” “At launch, buying credits on Pro 100 or Pro 200 does not unlock Ultrafast.” | `images/18-pro-help-ultrafast.png` | | 已找到 |
| B5 | 新订阅 Pro 200 含增加后的额度 | L1 | 同 B1 | “Yes. New Pro 200 subscriptions are available from the pricing page and include the increased Work and Codex allowance. The monthly price remains $200.” | `images/14-pro-help-existing-200.png` | 对照 0929 版：“New subscriptions that aren’t eligible for grandfathering include a lower usage allowance than previously offered with Pro 200 to reflect our increasingly efficient models.” 这句话新版已删 | 已找到 |
| B5b | 取消再订阅 | L1 | 同 B1 | “If you cancel and later resubscribe, you’ll get the same Work and Codex allowance as other Pro 200 subscribers. If your subscription is active, you don’t need to cancel and resubscribe to get the updated allowance.” | `images/15-pro-help-resubscribe.png` | 0929 版有“上周内断档者重订可恢复旧额度”的条件，新版整段改写 | 已找到 |
| B6a | HN 热度 | L6 | https://news.ycombinator.com/item?id=50292524 “OpenAI: We are doubling all usage limits for all Pro 100/200/500 tiers” | 帖链撤回帖；北京 10-30 01:13（UTC 10-29 17:13:33） | 无 | 16 分、1 评（取于北京 10-31 17:05）；唯一评论 50292648：“OpenAI is such a mess atm … They finally, right at the end, flip-flopped on 100/200/500 plan usage.”（得分未公开） | 已找到 |
| B6b | HN 旧帖 | L6 | https://news.ycombinator.com/item?id=49919026 “OpenAI halved the allowance of the $200 plan to 10x” | 10-01 16:05（UTC 08:05）；23 分，9 条嵌套评论（取于北京 10-31 17:05） | 无 | 帖主引邮件原文（见 B3g）；评论 49920294：“Previously, you got a magical free money bonus on the $200 sub. Plus costs 20. Pro 5x costs 100, and gives 5x Plus usage. Pro 20x was 200 … Of course it had to end.”（无得分）；49965261：“How predictable will the new 10x practice be in practice, given the fact that usage differs drastically based on effort level, models and tasks?” | 已找到 |
| B6c | 其他 HN 条目 | L6 | 49901067 “Tell HN: OpenAI's New Pro 500 subscription, reduced Pro 200 usage”（3 分 2 评，09-30 05:42）；49907320 “ChatGPT Pro 20x now 10x for the same price”（1 分，09-30 19:23）；50302093 BleepingComputer 帖（2 分，10-30 18:34） | | 无 | 均低热度 | 已找到 |
| B6d | Reddit | — | — | `barking heat` 的 Reddit 搜索被 old.reddit.com 跳转到登录页，两次都不是 JSON | 无 | 不收登录态内容，记失败 | 未找到一手来源 |

## 三、与 0929 页面的删改（`e-pro-tiers-help.md` → 当前渲染页）

逐句对比（0929 版在左，当前在右；另有 10-26 的中间版见 Wayback `b-wayback-20261026042106.txt`）：

1. 页头引言：旧 “…a new $500/month plan that includes Astra Ultrafast. Pro 200 is also available for new subscriptions again. New subscriptions that aren’t eligible for grandfathering include a lower usage allowance.” → 新 “…a new $500/month plan that includes GPT-6 Astra Ultrafast and GPT-6.1 Sol Ultrafast. Pro 200 is also available for new subscriptions again. Pro 100 and Pro 500 now include more Work and Codex usage. All Pro 200 subscriptions now include the same Work and Codex allowance.”（“GPT-6.1 Sol Ultrafast”在 10-26 版已出现。）
2. “Can I subscribe to Pro 200 again?”：删 “New subscriptions that aren’t eligible for grandfathering include a lower usage allowance than previously offered with Pro 200 to reflect our increasingly efficient models.”，改 “…and include the increased Work and Codex allowance.”
3. “What happens to my existing Pro 200 subscription?”：整段删（原：“you’re eligible to keep your previous included usage allowance through Oct 29, 2026 … After that date, your subscription will move to the lower included usage allowance … Only users affected by this change will receive an email”），改为撤回声明三句（B1a–c）。
4. 删问答 “Does keeping my current allowance give me Pro 500 features?”，换成 “Does the updated Pro 200 allowance include Ultrafast?”（答案同样是 No，Ultrafast 只在 Pro 500）。
5. “What if I cancel and subscribe again?”：删老用户“断档七天内重订可保旧额度”的条件，改为“重订得到与其他 Pro 200 订户相同的额度；有效订阅无需取消重订”。
6. 新增 “Increased Work and Codex allowance”（含 “A larger allowance does not guarantee twice as many completed tasks.”）与 “Does my weekly GPT-6 Pro message limit change?”。
7. “Which Pro plans include Ultrafast?”：新增 “8× the Standard rate … 6× the Standard credit rate”。
8. 新增 “Why can similar tasks use different amounts of my included allowance?”（含 Fast 2.5 倍、Ultrafast 8 倍）。
9. 相关文章链接有变（新增 “API credits benefit for eligible ChatGPT subscriptions”）。
10. 10-26 中间版曾写明资格窗口：“Grandfathering applies if you had an active Pro 200 subscription at any point from September 22, 2026 through 10 a.m. Pacific Time on September 29, 2026.” 0929 版只写 “at the eligibility cutoff or during the seven days before it”；现行版两者都不再出现。

## 四、可能的吠点（L1 为官方原文；其余为线索，进正文前须回核）

1. “翻倍”并不对所有 Pro 200 用户成立：官方写 “On Pro 200, only the lower allowance doubled.” 原来高额度的老订户只是保住额度，没有增加（帮助页 B1b、B3a）。Tibo 帖本身前后矛盾：“doubling all usage limits for all Pro 100/200/500 tiers” 与 “your Work and Codex usage will stay the same”（fxtwitter JSON；爱范儿报道称科技博主 Chubby 在评论区就此提问，L4）。
2. 倍数没有官方出处：官方页只有“Pro 500 = Plus 的 25 倍”（DevDay 回顾页，改版时）；Pro 100 = 5 倍、Pro 200 新 = 10 倍/老 = 20 倍来自 9-29 Tibo 帖与邮件引文；翻倍后的 10/20/50 倍是媒体（智东西等）按“翻倍”推算，帮助页没有这些数字（B3）。
3. 绝对额度仍然不公开：官方页不给 token、任务数或美元额度，所以“翻倍”无法由用户核对（Bleeping、gHacks、Startup Fortune 都这么说，L4；帮助页本身确实没有，L1）。
4. “额度翻倍不保证任务翻倍”：官方自己写明（B4a）；Fast 模式 2.5 倍、Ultrafast 8 倍的消耗倍率也写在同页（B4b）。
5. 聊天里的 GPT-6 Pro 周消息上限不随之翻倍：Pro 100/500 “keep their current weekly GPT-6 Pro message limits”（B3c）。
6. Plus 不变：官方写 “Plus usage is unchanged.” 爱范儿与智东西都提到 Plus 用户不满（L4）。
7. 先降后撤的整体是回到原点：对 Pro 200 来说，9-29 之后新订阅者拿到的是 10 倍口径（Tibo 帖），现在补到与老用户相同的 20 倍口径；这个“20 倍”只有 Tibo 的 9-29 帖与邮件引文支持，官方页不写（推断）。Tibo 9-29 自己写过 “it will net out at half the dollar in API spend compared to the old Pro $200 plan”（fxtwitter），智东西据此总结为“先减半再翻倍”。
8. 帮助页的撤回是“删除了资格窗口与 10-29 截止日期”之外还把此前的原因说明也删了：0929 版写降额是 “to reflect our increasingly efficient models”，新版整句删除，没有给出撤回理由；理由只见于 Tibo 帖（“We heard your kind feedback”）（B5）。
9. 撤回前是否有人已被降过：官方页没有说；smzdm 文章称两则帖文“都没讲清楚中间那半个月限额是否真的收回过”（L4，`b-smzdm-merge.txt`）。
10. 一次性 usage credits：nerdschalk 称 9-29 邮件向老 Pro 200 订户送 62,500 usage credits（$2,500，12-31 到期）；撤回后官方页没有再提这一项（L5 转引，未核）。
11. 同日有人称结账页小字出现 “Starting in December, plan usage will have a shared limit across all of ChatGPT and Codex”（smzdm 文章与微博/知乎转述，L4，登录态页面，**未存档、未核**）；公开帮助页、定价页、发行说明里 grep 不到 “shared limit” 字样。仅作线索，不进正文。
12. dots 的扩展额度只是 “for now”：Tibo 帖 “continue offering extended limits for dots for now, and usage of your dot will still continue to be separate from your default usage.”（fxtwitter；gHacks 指出没说期限，L4）。

## 五、夸大说法实例（把“翻倍”写成所有 Pro 200 用户都翻倍或其他变形）

| 标题/原句（逐字） | 站点与 URL | 发布时间 | 发布方 | 说明 |
|---|---|---|---|---|
| “突发，ChatGPT Pro所有档位额度，官宣翻倍” | 投资界 https://news.pedaily.cn/202610/571121.shtml | 2026-10-30 11:50（北京，页面 datePublished；页面“更新于 11:46”） | 投资界（pedaily.cn） | 正文：“降额不降了，所有 Pro 档位的额度，直接翻倍。” 同文又写“Pro 100、Pro 200、Pro 500 三档……所有用量上限翻倍”。与官方“Pro 200 只有低额度者翻倍”不符 |
| “刚刚，ChatGPT Pro 额度官宣翻倍，但免费重置可能要没了” | 爱范儿 https://www.ifanr.com/1686830 | 2026-10-30 08:29（北京） | 爱范儿 | 标题泛称 Pro 翻倍；正文后半说明“对于原本享有较高额度的 Pro 200 老用户，此次调整主要是取消原定削减”。标题 vs 正文不一致，可作为“写歪标题”的例子，正文本身基本正确；同文称“免费重置可能要没了”是记者推测（Tibo 帖没这么说） |
| “刚刚，ChatGPT Pro 额度官宣翻倍，但免费重置可能要没了” | 新浪科技转载 https://finance.sina.com.cn/tech/roll/2026-10-30/doc-iniwyfvy5559225.shtml | 2026-10-30 08:29:45（北京，页面 meta create_at，与爱范儿原稿同一时刻）；页面另有注释 “published at 2026-10-30 16:35:38”（疑为缓存生成时刻） | 新浪 | 同上 |
| “OpenAI Doubles ChatGPT Pro Work and Codex Allowances Across Pro Tiers” | DEV Community（scalevise.com 转载）https://dev.to/alifar/openai-doubles-chatgpt-pro-work-and-codex-allowances-across-pro-tiers-2fj0 | 2026-10-29T20:03:32Z = 北京 10-30 04:03 | 作者 Ali Farhat（页面署名） | “Across Pro Tiers” 暗示所有档翻倍；正文引了帮助页 “For Pro 200, the lower allowance doubled”，标题没带 |
| “主打一个听劝，OpenAI把Pro额度翻倍了” | 智东西（经网易号转载）https://www.163.com/dy/article/LAGN5LRJ051180F7.html | 网易页标 2026-10-30 17:38:18（北京）；智东西原站未检到 | 智东西（网易号） | 标题泛称“Pro额度翻倍”。正文：“这次额度翻倍，本质上是一次‘先减半再翻倍’的营销策略，在推出新订阅档位的时候将额度减半，再通过翻倍保持原有额度。”（这是文章作者的判断，L4，不是官方说法）；正文同时写了 Pro 200 老用户“继续保留原有20倍额度” |
| “OpenAI Doubles ChatGPT Pro and Business Premium Usage Limits After Dropping a Planned Pro 200 Cut” | gHacks https://www.ghacks.net/2026/10/30/openai-doubles-chatgpt-pro-and-business-premium-usage-limits-after-dropping-a-planned-pro-200-cut/ | 2026-10-30（作者 Arthur Kay；页面未给时刻） | gHacks | 标题基本准确（点明 Pro 200 是被撤销的降额）；可作“写对”的对照。正文却写：“Pro 200 subscribers who were preparing to switch plans because of the announced cut keep their previous Work and Codex usage, now doubled.”——“now doubled”可读成原本高额度的老用户也翻倍了，与官方“只有低额度者翻倍”不符（措辞含糊，读法待 Claude 判断） |
| “OpenAI doubles ChatGPT Pro usage limits after backlash over planned 50% cut” | BleepingComputer https://www.bleepingcomputer.com/news/artificial-intelligence/openai-doubles-chatgpt-pro-usage-limits-after-backlash-over-planned-50-percent-cut/ | October 29, 2026 07:35 PM（页面未标时区） | BleepingComputer（Mayank Parmar） | 标题“Pro usage limits doubled”；正文引了 Tibo 原话，没有区分 Pro 200 的两类用户 |

没找到：标题明说“所有 Pro 200 用户额度翻倍”的字句，除上面的泛称；中文侧智东西“先减半再翻倍”在网易号转载找到，原站 zhidx.com 未检到（WebSearch 与 bing 都没有给出 zhidx.com 链接）。

## 六、扫描说法勘误（WorkBuddy / 派工单“已知情况”）

| 扫描说法 | 与存档对照 |
|---|---|
| “OpenAI 原定 10-30 把 Pro 200 额度从 20× 降到 10×，当天凌晨撤回” | 部分相符：降额的日期与倍数来自 9-29 邮件（HN 引文，L6）与 Tibo 帖，官方页写 “through Oct 29, 2026 … After that date”。撤回帖在美东 10-29 12:56 PM = UTC 10-29 16:56 = 北京 10-30 00:56；“当天凌晨”按北京时间成立，按美东是前一天中午。 |
| “反而翻倍” | 对 Pro 100、Pro 500 与 Pro 200 低额度者成立；对 Pro 200 原高额度者不成立（保留）。官方页原句见 B1b、B3a。 |
| “智东西 10-30 写‘先减半再翻倍’” | 找到：网易号转载稿（来源标注智东西，北京 10-30 17:38:18），原句见第五节；是作者的判断，不是官方说法；原站未检到。 |
| “HN 49919026 ‘OpenAI halved the allowance of the $200 plan to 10x’（10-01，23 分）” | 相符：23 分，发帖时间 2026-10-01T08:05:19Z（北京 10-01 16:05）；评论数 HN 搜索口径 9 条嵌套。 |
| “Claude 浏览器读到页面 ‘Updated: 2 days ago’ 与上列原句” | 逐句相符；页头的“Updated”随浏览器语言本地化，渲染文本在 en-US 下为 “2 days ago”，zh 下为“前天”；页面数据 `updatedAt` = 北京 10-30 01:02:55。 |
| 派工单：“Ultrafast 仅 Pro 500” | 相符，并补充：官方写明 “At launch, buying credits on Pro 100 or Pro 200 does not unlock Ultrafast.” |
| 派工单：“0929 期已核……老订户旧额度保留到 Oct 29, 2026，之后同样下调” | 相符（0929 存档）；该段现已整段删除。 |
| 本组未得到：OpenAI 官方 X 账号的公告、邮件原文、zhidx.com 原站、Reddit 热度 | 见第二节 B2d、B6d。 |

## 七、截图清单（`docs/2610/1030/images/`，均 1240 px 宽、2 倍像素，已目视确认）

| 文件 | 内容 | 备注 |
|---|---|---|
| `13-pro-help-top.png` | 帮助页标题、副标题、“Updated: 2 days ago”、引言 | 头部含面包屑；浏览器设为 en-US，来源 URL 未入图 |
| `14-pro-help-existing-200.png` | “Can I subscribe to Pro 200 again?”、“What happens to my existing Pro 200 subscription?”、“Does the updated Pro 200 allowance include Ultrafast?” | 核心依据图 |
| `15-pro-help-resubscribe.png` | “What if I cancel and subscribe again?” 及相邻两段 | |
| `16-pro-help-increased-allowance.png` | “Usage and limits”标题与 “Increased Work and Codex allowance” 全段（含 does not guarantee 句） | 核心依据图 |
| `17-pro-help-weekly-pro-limit.png` | “Does my weekly GPT-6 Pro message limit change?” | |
| `18-pro-help-ultrafast.png` | “Which Pro plans include Ultrafast?” 含 8×、6× | |
| `19-pro-help-rates.png` | “Why can similar tasks use different amounts of my included allowance?” 含 2.5 倍 | |
| `20-pro-help-plan-table.png` | “Pro plans” 方案表与 “See the pricing page” | 右侧有页面悬浮的客服按钮一角（页面原样） |
| `21-release-notes-1029.png` | 发行说明 “October 29, 2026” 条目 | |
| `22-release-notes-top.png` | 发行说明页头（含 “Updated: 11 hours ago”）、10-30 条目与 10-29 条目 | 同样有客服按钮一角 |
| `23-tibo-doubling-post.png` | Tibo 撤回帖（X 官方嵌入渲染，匿名） | 长文被截断（“Show more”）；时间为 EDT；头像为占位灰色 |

编号 24（0929 旧页面对应段）本次未生成：现成的旧图在 `docs/2609/0929/images/20-pro-tiers-help.png`（0929 期帮助页）、`17-tibo-pro-half.png`、`18-tibo-pro-multiplier.png`；10-26 与 10-29 的 Wayback 快照文字版见上。需要 Wayback 渲染图时由 Claude 另取。
