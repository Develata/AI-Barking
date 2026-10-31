# 1030 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`、`evidence-b.md`、`evidence-c.md`、`evidence-d.md`、`evidence-E.md`，截图在 `images/`。

## 省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 两天里，Claude 和 ChatGPT 的200美元档都改了额度规则 | L1 | ✅ | 见 A、B 节 | OpenAI 帮助页 updatedAt 北京 10-30 01:02:55；Anthropic 官方帖北京 10-31 01:29:07，前后约一天。“200美元档”＝Claude Ultra（原 Max 20x）与 ChatGPT Pro 200。 |
| Claude 对已验证的 Ultra 账号取消每5小时的用量上限，每周上限不变 | L1 | ✅ | 见 A 节 | “No five-hour limit for verified accounts”；官方帖 “Weekly limits remain unchanged”。 |
| OpenAI 撤回 Pro-200 降额，只有原先额度较低的用户翻倍 | L1 | ✅ | 见 B 节 | “The reduction announced in our September 29 email is canceled.”；“On Pro 200, only the lower allowance doubled.” |
| 苹果 Apple 智能12月面向中国用户推出，部分体验接入千问，不含 Siri AI | L1 | ✅ | 见 C 节 | 第二轮反向核验 NEEDS_VERIFICATION 7 后删“国行 iPhone”（购买地、使用地范围苹果未给）。“将于 12 月 iOS 27.3 正式发布时面向中国用户推出”；“在部分 Apple 智能体验中加入对千问的支持”；“本次更新不包含 Siri AI”。 |

## A Claude 改名 Ultra

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间31日凌晨，Anthropic 把 Claude 的 Max 20x 套餐改名 Ultra，网页订阅仍是200美元/月 | L1 | ✅ | Anthropic 官方海外社媒帖 status/2116220869479428500（`a-fxtweet.json`，图 07）；https://claude.com/pricing （`a-pricing.txt`，图 01） | 帖：“Max 20x → Ultra … Ultra is still $200. If you’re a subscriber, there’s nothing you need to do.” 发帖 UTC 10-30 17:29:07 = 北京 10-31 01:29:07；Ultra 帮助页 lastUpdatedDate 北京 01:11:53。 |
| 已验证的 Ultra 账号取消每5小时的用量上限 | L1 | ✅ | https://support.claude.com/en/articles/17728952-what-is-the-ultra-plan （`a-ultra-help.txt`，图 03、04；批注图 claude-ultra-help） | “Verified Ultra accounts don't have a five-hour limit. If your account isn't verified yet, a temporary limit applies.” 官方帖：“starting today with most existing subscribers”。 |
| 每周上限不变，是 Pro 套餐的10倍 | L1 | ✅ | 同上；定价页 “10x Pro usage every week*” | “The Ultra plan includes 10 times the Pro plan's usage every week.”；帖 “Weekly limits remain unchanged.” |
| ①已验证的 Ultra 账号只是不再受每5小时上限约束，每周上限照旧 | L1 | ✅ | 同上 | 同上两句。第一轮反向核验 BLOCKER 1 后补回“已验证的 Ultra 账号”。 |
| ①Fable 最多占一半（指每周上限） | L1 | ✅ | 定价页 “Fable included, up to half your weekly limit”；对比表 “50% of weekly limits*”（Max、Ultra）；Fable 帮助页 https://support.claude.com/en/articles/15424964-claude-fable-models-on-your-plan （`a-fable-plan.txt`） | “You can use up to 50% of your weekly usage limits on Fable models at no extra cost.” Ultra 帮助页写法为 “a separate weekly limit for Fable”。不是另给额度。 |
| ②旧名“20x”指每5小时上限是 Pro 的20倍 | L1（旧页，web.archive.org） | ✅ | Max 帮助页旧版 https://web.archive.org/web/20261024051700/https://support.claude.com/en/articles/11049741-what-is-the-max-plan （`a-wb-max-1024.txt`，图 05；批注图 claude-max-old） | “Max 20x includes 20 times the Pro plan's per-session usage allowance.”；“Your session-based usage limit will reset every five hours.” 快照 UTC 10-24 05:17，页面 lastUpdatedDate 2026-10-13。 |
| ②Anthropic 订阅产品负责人称每周上限设立时就是10倍，并为新用户误以为都是20倍道歉 | L2 | ✅ | Mike Overell 海外社媒帖（`a-fx-mboverell.json`，图 08 卡片在关键句前截断，全文以 JSON 为准） | “I lead product for Claude subscriptions.”；“When we added weekly limits to Max 20x, we set them to 10x Pro's (the 5-hour limit stayed at 20x). I get that some new users assumed 20x applied to both limits. We could have done better here, and I'm sorry.” UTC 10-30 17:31:27。官方页面未写旧 20x 的周倍数，此说仅员工帖。 |
| ②6月有人集体起诉，按周用量估算称只有 Pro 的6到8倍 | L1（法院公开案卷：起诉状） | ✅ | Kahn v. Anthropic PBC, N.D. Cal. 3:26-cv-05763，起诉状 https://storage.courtlistener.com/recap/gov.uscourts.cand.472161/gov.uscourts.cand.472161.1.0.pdf （`a-cl-doc1-pdf.txt`，图 12；批注图 kahn-complaint-page2）；案卷 `a-cl-docket.txt` | 2026-06-14 提起（docket “Filed on 6/14/2026”）。¶6 “delivering just six to eight times the usage of Pro”；¶60–63 以每周用量估算（`a-cl-doc1-pdf.txt` 888–893 行），驳回动议亦称 “estimated weekly hour limits”，正文写“按周用量估算”；为原告主张，非法院认定。Anthropic 驳回起诉动议（ECF 39，`a-cl-doc39-pdf.txt`）概括原告依据为 “estimated weekly hour limits”，并称 5x/20x 是 per session。已与 Smith 案合并。案卷 10-30 有一条无正文的 “Order on Motion to Dismiss”，内容不明，正文不写听审日期与诉讼进展；不写改名与诉讼有关（官方未说）。 |
| 译注：Ultra 套餐有一个覆盖所有模型的周限额，另有一个单独的 Fable 周限额 | L1 | ✅ | Ultra 帮助页（`a-ultra-help.txt` 第102行；批注图 claude-ultra-help） | “Ultra plans have a weekly usage limit that applies across all models, and a separate weekly limit for Fable.” |
| 译注：按会话计的用量限制每5小时重置一次 | L1（旧页，web.archive.org） | ✅ | `a-wb-max-1024.txt` 第116行；批注图 claude-max-old | “Your session-based usage limit will reset every five hours.” |
| 批注图标题：Kahn 诉 Anthropic 集体诉讼起诉状第2页 | L1 | ✅ | `a-cl-doc1-pdf.txt` 页头；图 12 | “Case 3:26-cv-05763 Document 1 Filed 06/14/26 Page 2 of 35”；案名 Kahn v. Anthropic PBC。 |
| ③Ultra 新账号先受临时限制，何时解除没说 | L1 | ✅ | Ultra 帮助页；官方帖 | “If your account isn't verified yet, a temporary limit applies. We remove it automatically once your account is verified”；帖 “Newer accounts keep a temporary limit until they’re verified … You'll get an email when it's removed.” 已读范围（帮助页、官方帖、Overell 帖）均无解除时间。 |
| ③结账验证手机号不算 | L1 | ✅ | Ultra 帮助页（图 04） | “Verifying your phone number at checkout is a separate step and doesn't remove this limit.” |

## B OpenAI 撤回 Pro-200 降额

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 前一天凌晨，OpenAI 撤回9月底宣布的 Pro-200 降额 | L1 | ✅ | https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers （`b-pro-help-rendered.md`、`b-pro-help-article.json`，图 13、14；批注图 openai-pro200）；Claude 北京 10-31 约 16:40 亦在浏览器读到同句 | “The reduction announced in our September 29 email is canceled.” 页面 updatedAt 北京 10-30 01:02:55（UTC 10-29 17:02:55）；员工 Tibo 帖北京 10-30 00:56:07。原降额：0929 期已核，老订户旧额度保留至 Oct 29, 2026。 |
| 译注：我们9月29日邮件中宣布的降额取消 | L1 | ✅ | `b-pro-help-rendered.md` 第65行；批注图 openai-pro200 | “The reduction announced in our September 29 email is canceled.” |
| Pro-100 和 Pro-500 的办公与编程（Work、Codex）额度翻倍 | L1 | ✅ | 同上（图 16；批注图 openai-doubled） | “Pro 100 and Pro 500 now include twice their previous Work and Codex allowance.” 聊天里 GPT-6 Pro 周消息上限，这两档不变（正文未写）。 |
| 已删：价格都不变 | L1 | — | 同上（图 14、20） | 第一轮反向核验 NEEDS_VERIFICATION 8：Pro 100、Pro 500 价格“不变”缺历史存档，正文删去；Pro 200 “Your subscription price stays at $200/month.” 有直接依据，未写进正文。 |
| 译注：额度更大，不保证完成的任务翻倍 | L1 | ✅ | 同上（`b-pro-help-rendered.md` 第84行；图 16；批注图 openai-doubled） | “A larger allowance does not guarantee twice as many completed tasks.”（第一轮反向核验 SHOULD_FIX 5 后补此行） |
| ④OpenAI 帮助页写明，Pro-200 只有原先额度低的翻倍，额度高的不变 | L1 | ✅ | 同上（图 14、16） | “On Pro 200, only the lower allowance doubled.”；“If you already had the higher Work and Codex allowance, you keep it. If you had the lower allowance, it has doubled to match.” 员工 Tibo 帖称 “doubling all usage limits for all Pro 100/200/500 tiers”，同帖又说 Pro 200 “usage will stay the same”（`b-fx-2115850175868727621.json`；正文已删此句以控长度）。夸大实例：投资界“突发，ChatGPT Pro所有档位额度，官宣翻倍”。 |

## C 苹果国行 Apple 智能接入千问

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 苹果北京时间31日凌晨宣布 | L1 | ✅ | https://www.apple.com.cn/newsroom/2026/10/apple-intelligence-features-are-coming-to-users-in-china/ （`c-apple-newsroom-release.txt`；RSS `c-apple-newsroom-rss.xml`） | 页面正文只写“2026 年 10 月 31 日”；RSS（Atom）条目 updated 2026-10-30T20:59:25Z = 北京 10-31 04:59:25（是更新时间字段，不是 pubDate）；IT之家北京 10-31 05:02 已报道（L4），故“凌晨宣布”成立。Claude 北京 10-31 亦用 curl 读到正文。 |
| Apple 智能首批功能11月4日随 iOS 开发者测试版开放测试 | L1 | ✅ | 同上（图 25；批注图 apple-intro） | “这一系列 Apple 智能功能将自 11 月 4 日起随最新的 iOS 开发者 Beta 版开放测试”。 |
| 12月随 iOS-27.3 正式版面向中国用户推出 | L1 | ✅ | 同上 | “并将于 12 月 iOS 27.3 正式发布时面向中国用户推出”。 |
| Siri、写作工具和视觉智能的部分体验接入阿里千问 | L1 | ✅ | 同上 | “Apple 与阿里巴巴达成合作，在部分 Apple 智能体验中加入对千问的支持，让中国用户能直接在 Siri、写作工具以及视觉智能中使用阿里巴巴的千问”。 |
| ①苹果原文写明，这次不含 Siri AI，也不含今年开发者大会（WWDC 2026）公布的其他下一代功能 | L1 | ✅ | 同上（图 27；批注图 apple-siri-qwen） | “本次更新不包含 Siri AI 及 Apple 在 WWDC 2026 上宣布的其他下一代 Apple 智能功能。” |
| ②新浪转载的文汇报标题写“千问全面接入” | L4（媒体标题，核的是标题文字） | ⚠️ | https://k.sina.cn/article_5953740931_162dee083067043w24.html （`c-sina-quanmian-jieru.txt` 第12、21行：标题与“来源：文汇报”，北京 10-31 09:35；图 32） | 标题“阿里巴巴与苹果达成合作，千问全面接入，国行版苹果智能，真的要落地了！” |
| ②苹果原文是部分体验支持千问，Siri 处理特定请求时能接入 | L1 | ✅ | 苹果新闻稿（图 25、27） | “在部分 Apple 智能体验中加入对千问的支持”；“在处理特定请求时，Siri 能够接入千问”。另：“Apple 智能由 Apple 构建的生成式模型驱动”（正文未写）。 |
| ③正式版要等12月，苹果没给具体日期 | L1 | ✅ | 苹果新闻稿 | 只写“12 月 iOS 27.3 正式发布时”，无日期（已读：新闻稿、Apple 智能中国页）。11月4日为开发者 Beta。库克10-15曾说“10月底”测试版（上海证券报，L4，未进正文）。 |
| 译注：支持机型 iPhone 16 及后续、iPhone Duo、15 Pro、15 Pro Max、Air | L1 | ✅ | `c-apple-newsroom-release.txt` 第220行；批注图 apple-devices | 脚注1原句见下行。 |
| ④首批只支持 iPhone-16 及以后、15-Pro 系列和 iPhone-Air 等机型，普通款 iPhone-15 用不上 | L1 | ✅ | 苹果新闻稿脚注1（图 28；批注图 apple-devices） | “支持的设备包括：iPhone 16 及后续机型、iPhone Duo、iPhone 15 Pro、iPhone 15 Pro Max 和 iPhone Air”。“等”含 Duo；iPhone 15、15 Plus 不在列表内。“首批只支持”：脚注写的是首批核心功能随 iOS 27.3 提供的设备；另需 Siri 与设备语言设为英文和中文（简体），与 Apple 智能页“或”的写法不一致（未进正文）。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌称原型卫星上的 TPU 在轨跑通 Gemma 4，回传一首关于太空的俳句 | L1 | ✅ | https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-satellite-response/ （`e-e-suncatcher.txt`） | “we uplinked our first prompt to our onboard Gemma 4 open model, asking it to generate a haiku about being in space … the downlinked poem confirmed success”；“the first-ever AI response from a data center class TPU in orbit”。页面 Oct 30, 2026，官方未给时刻。跟进 1002（入轨）；1 kW、20 分钟、4 颗 TPU 均不在谷歌三篇页面，不用。 |
| Reddit 称10月搜索引荐周均少逾100万，将在美国以外部分市场测试必须登录 | L1 | ✅ | Q3 股东信 https://s203.q4cdn.com/380862485/files/doc_financials/2026/q3/v2/Q3-26-Shareholder-Letter.pdf （`d-d1-shareholder-pdf.txt`） | “During October, we have seen that search traffic has dropped by more than 1 million referrals on average per week.”；“In Q4 we’ll begin fully testing this in select markets outside the U.S.”（this＝logged-in-only）。信中未点名谷歌或 AI Mode；归因谷歌只见 The Verge 转述 CEO（L4，未写）。 |
| OpenAI 计划在数月内遵守网站拒绝 AI 训练的共同信号 | L1 | ✅ | https://openai.com/index/helping-websites-express-ai-training-preferences/ （`d-d2-openai.txt`） | “OpenAI will honor a common signal in robots.txt … for foundational generative AI training.”；“We aim to implement the signal in the coming months”（aim，目标而非承诺，故写“计划”）。只管训练；谷歌2027年、微软2027-02-01 起（L1，未写）；Anthropic 仅员工 IETF 邮件称“已开始实施”。 |
| OpenAI 称第三季度就迫在眉睫的暴力威胁向执法机构转介超200次 | L1 | ✅ | https://openai.com/index/community-safety-report-and-new-protections-for-teens/ （`d-d3-openai.txt`） | “we made over 200 law enforcement referrals in Q3 2026”；“bringing imminent threats of violence to the attention of law enforcement”。是否人工审核原文未说。 |
| 开发者把 NES 游戏机处理器编译进模型权重，权重未经训练 | L1（作者自有仓库） | ✅ | https://github.com/dbreunig/nesformer （`e-e-nesformer.txt`） | “An NES CPU compiled into the weights of a stock Qwen3.5 model.”；“Nobody trained these weights; a compiler computed every one.” “处理器”＝CPU。速度“150–290× slower”是作者机器单核、numba 引擎自测，第一轮反向核验 BLOCKER 2 后删去。HN 50306442。 |
| 据 404 Media 报道，Proton CEO 取消几乎全员 AI 复审面谈 | L4 | ⚠️ | https://www.404media.co/protons-ceo-cancels-ai-re-interviews/ （`e-e-404-proton-cancel.txt`） | “Andy Yen, the CEO of Proton, cancelled a series of nearly company-wide ‘re-interviews’ that would have forced employees to explain how they are using AI”。美东 10-30 12:40 = 北京 10-31 00:40；据录音与员工，非 Proton 公告。“几乎全员”＝nearly company-wide。 |

## 反向核验

### 第一轮（正文与事实清单；Codex gpt-6-astra medium，WSL 只读）

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER 吠点卡①③漏“已验证的 Ultra 账号”“Ultra 新账号” | 采纳 | ①改“已验证的 Ultra 账号只是不再受每5小时上限约束”，③改“Ultra 新账号” |
| 2 | BLOCKER NES 速度数字丢了作者单核自测条件 | 采纳 | 速览删速度数字，改“开发者把 NES 游戏机处理器编译进模型权重，权重未经训练” |
| 3 | SHOULD_FIX “称将在数月内”丢了 aim | 采纳 | 改“计划在数月内” |
| 4 | SHOULD_FIX “普通用户要等12月”超出来源（开发者测试版可免费注册） | 采纳 | 改“正式版要等12月” |
| 5 | SHOULD_FIX 译注“不保证任务翻倍”无清单行；B 节存档名笼统 | 采纳 | 补行；存档改为 `b-pro-help-rendered.md` 等实名 |
| 6 | SHOULD_FIX 两份 README 缺失 | 采纳 | 补 `images/README.md`、`sources/README.md`（送审时尚在制作） |
| 7 | SHOULD_FIX L4 标 ✅ 违反分级 | 采纳 | 改 ⚠️，正文已写“有媒体标题写” |
| 8 | NEEDS_VERIFICATION Pro 100/500 “价格不变”缺历史依据 | 采纳 | 正文删“价格都不变” |
| 9 | NEEDS_VERIFICATION 执法转介涉外国执法机构，请 Develata 定风控 | 部分采纳 | 公司自报的执法转介数据（原文只写 law enforcement，未限定国家），不属“外国政府与政治”或“点名中国公司”两类；Develata 10/31 终审定：保留 |

读者稿检查（`style-review.md`）同轮返回：读者看不懂“5小时窗口”、套餐名和 Work/Codex，据此把“窗口”改为“每5小时的用量上限”，补“Claude 的……套餐”“办公与编程（Work、Codex）”“今年开发者大会（WWDC 2026）”，删去“取消的是……不是……”句式。

### 第二轮（改稿后正文、卡片与清单；同上）

结论 PASS、无 BLOCKER。

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX “6到8倍”未交代每周估算口径 | 采纳 | 改“按周用量估算称只有 Pro 的6到8倍” |
| 2 | SHOULD_FIX “200美元/月”漏“仅网页订阅” | 采纳 | 事实段改“网页订阅仍是200美元/月”（`a-ultra-help.txt` 第80行 “This price is for web subscriptions only.”） |
| 3 | SHOULD_FIX “Siri 只在……才接入”加强成排他 | 采纳 | 正文与吠点卡改“Siri 处理特定请求时能接入”，译注恢复原句语气 |
| 4 | SHOULD_FIX RSS 字段是 updated 不是 pubDate | 采纳 | 内部改记；“凌晨”另有 IT之家 05:02 报道佐证，保留 |
| 5 | SHOULD_FIX L4 媒体未点名 | 采纳 | 改“新浪转载的文汇报标题写” |
| 6 | SHOULD_FIX 译注与图中文字缺清单行 | 采纳 | 补 Fable 周限额、会话5小时重置、起诉状页头、9月29日邮件、iPhone Duo 机型各行 |
| 7 | NEEDS_VERIFICATION “国行 iPhone”超出苹果原文 | 采纳 | 省流改“苹果 Apple 智能12月面向中国用户推出”，小标题改“苹果：Apple 智能入华接千问” |
| 8 | NEEDS_VERIFICATION 执法转介风控待 Develata；内部“美国”无依据 | 采纳 | 内部删“美国”；Develata 10/31 定：保留 |

为放进吠点卡，另把吠点①③②的措辞压短（“Fable 最多占一半”“结账验证手机号不算”“6月有人集体起诉”），事实不变。

## 视觉复核

复核方：Codex gpt-6-astra medium（WSL 只读），附图24张（两张封面、省流卡、速览、两张事件卡、两张吠点卡、八张批注图及底图）。结论 PASS、无 BLOCKER。

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX 速览 Reddit “部分海外市场”易被读成中国以外 | 采纳 | 改“将在美国以外部分市场测试必须登录”（同时“每周平均”压为“周均”以守40字） |
| 2 | SHOULD_FIX Ultra 帮助页与起诉状批注图原文太小 | 采纳 | Ultra 页改为用量限制段与“已验证账号”段拼接（claude-ultra-help）；起诉状改为页头加第15–24行（kahn-complaint-page2），同时去掉了含原告姓名的首段 |
| 3 | NICE_TO_HAVE 事件卡“Pro-500”断行 | 不采纳 | 卡片模板按字宽自动折行，不为单词改模板；不影响事实 |

改后 Claude 逐张打开 05、07 两张核对高亮与行号。
