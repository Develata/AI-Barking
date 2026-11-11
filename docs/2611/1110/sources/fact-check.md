# 1110 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`（Gemini 人工审核）、`evidence-b.md`（Opus-5.5“变笨”争议）、`evidence-c.md`（Gemini-4-Carbon 全面开放）、`evidence-d.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。D 组取证于北京 11-11 14:30 前后；Claude 另于 14:00–14:40 对照原页抽查了 Glasswing 台账、VulnCheck、AlphaProof 博文、Gemini 帮助页、Strands 博文、Carbon 博文与 AA MKA 文。

第二条的当事模型 Opus-5.5 与起草者 Claude 同属 Anthropic，速览里也有三条 Anthropic 相关；这几条靠跨模型复核把关。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：谷歌有人读你邮件？ | L1/L6 | ⚠️ | 见 A 节 | 问号。指 GitHub 文档与 HN 帖“routes … to human annotators”的说法（L6）；“读”= 人工审核员查看；谷歌帮助页确认关联后“a subset … reviewed by human reviewers”，含邮件与文件的摘要、摘录（L1）。只涉及个人账户且主动关联，正文吠点②写明 |
| 标题：Claude变笨了？ | L6 | ⚠️ | 见 B 节 | 问号。指 Melaugh 帖“is Opus 5.5 nerfed?”与“the nerf is real”的说法，本号不下结论。标题用 Claude 是读者稿检查后改的（“Opus”普通读者不认识）；说法只针对 Claude 的 Opus-5.5，省流写“Claude Opus-5.5” |
| 省流：Gemini 关联邮箱网盘后部分内容有人审，帮助页1月就写了 | L1 | ✅ | A1、A4 | “有人审”= 人工审核员查看关联应用数据的一部分；“1月”= Wayback 2026-01-14 21:36:56 UTC 快照（北京 01-15 05:36） |
| 省流：Claude Opus-5.5“变笨”之争，两边测的不是一回事 | L6 | ⚠️ | B1、B2 | Melaugh：可见摘要措辞、high 档首个工具调用前思考 token 中位数；Theo：每条回复平均推理 token、工具调用数 |
| 省流：Gemini-4-Carbon 向付费用户开放，2美元是体验价 | L1 | ✅ | C1、C2 | “付费用户”= 付费 API 开发者、企业与 Ultra 订阅；免费 API 层 “Not available”。原写“全面开放”，反向核验第一轮 SHOULD_FIX 2 指出放大范围，已改 |

## A 谷歌 Gemini：邮箱网盘有人工审核

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 匿名 GitHub 文档称 Gemini 把个人邮件、网盘送去人工标注 | L6 | ⚠️ | https://github.com/thisisme6598/gemini_files （`a-gh-readme-raw.txt`、`a-gh-commits.json`；图 11） | 标题 “Google Gemini routes personal Drive files and emails to human annotators”。仓库创建北京 11-10 02:01:19（11-09 18:01:19 UTC），作者匿名；11-10 20:17 加范围说明（只适用于个人账户） |
| Hacker News 上700多分 | L6 | ⚠️ | https://news.ycombinator.com/item?id=50429426 （`a-hn-50429426.json`、`a-heat.json`） | 717 分 / 147 评，北京 11-10 02:24:38 提交，抓取 11-11 14:34 |
| 谷歌帮助页写：个人账户关联邮箱、网盘后，部分数据由人工审核员（含外包）查看 | L1 | ✅ | https://support.google.com/gemini/answer/16836988 （`a-help-16836988.txt`；图 01、03） | “When connected to Gemini, a subset of Connected Apps data processed by Gemini is reviewed by human reviewers (including trained service providers)”。“外包”= trained service providers。“个人账户”见 A7：Personal Intelligence 只对个人 Google 账户开放（帮助页 16598406、Workspace Privacy Hub “No”） |
| 含摘要、摘录；邮件或文件很短时，摘要可能就是原件 | L1 | ✅ | 同上（图 01） | “Interactions, summaries, excerpts and inferences from your relevant emails and files. In some cases, for example, where the email or file is brief or particularly relevant to respond to the prompt, the summary may just be the email or file.” 原文还有“particularly relevant”一种情形，正文只举“很短”，用“时”不写“只有” |
| 吠点①：今年1月的网页存档里，帮助页就这么写 | L1（Wayback 快照） | ✅ | https://web.archive.org/web/20260114213656/https://support.google.com/gemini/answer/16836988 （`a-wb-help-20260114.txt`；图 12e） | 快照北京 01-15 05:36:56（UTC 01-14 21:36:56），已含 “a subset of Connected Apps data … reviewed by human reviewers”“the summary may just be the email or file”“Important: Please don't connect…”。快照只证明“不晚于此时存在”。与 10-19 快照比，该问答只多一条 Wallet 项。Claude 抽查 grep 确认 |
| 吠点①：10月16日改的是2023年旧博文，在“不会被人工审核”后补了说明 | L1 | ✅ | https://blog.google/innovation-and-ai/products/our-responsible-approach-to-building-guardrails-for-generative-ai/ （`a-blog2023.txt`；Wayback 10-12、10-17、11-10 快照 `a-wb-blog-*.txt`；图 06、07） | 原句 “If you choose to use the Workspace extensions in Bard, your content from Gmail, Docs and Drive is not seen by human reviewers …”（博文 2023-10-12）。后加 “(Update, October 16, 2026: … It can also be subject to human review in limited circumstances …)”，日期为谷歌自署、未给时刻；Wayback 夹逼：10-12 04:46 UTC 无、10-17 00:45 UTC 有。原句未删。AI Weekly 称 10/16 改写了帮助页、“deleted” 承诺，与存档不符（evidence-a 夸大实例 2） |
| 吠点②：只涉及主动关联应用的个人账户，工作、学校账户不在内 | L1 | ✅ | 帮助页 16598406（`a-help-16598406.txt`；图 12f）；Workspace Privacy Hub（`a-ws-privacy-hub.txt`；图 12、12d）；1 月发布博文（`a-blog-pi-launch.txt`） | 发布博文：“Connecting your apps is off by default: you choose to turn it on”。16598406：个人账户、18 岁以上，工作/学校账户不可用；Workspace Hub：“Is Personal Intelligence available for Google Workspace Business, Enterprise, or Education accounts? No.”，“Your content is not human reviewed”。另限 Keep Activity 开启、部分地区不可用，正文未写 |
| 吠点②：“整份文件被读走”的演示，是让 Gemini 打印自己读到的内容，不说明有人看过 | L6/L1 | ⚠️ | README（`a-gh-readme-raw.txt`）；帮助页 16836988 | README 第 4 步：“Print the exact text and metadata block that the retrieval tool returned and injected into your prompt …”，情形 4 “Full-File Ingestion … ingested directly into session context”。这是模型自述的会话内容，不是审核日志；README 无审核员看到某份文件的证据。帮助页审核的是 “a subset of Connected Apps data processed by Gemini” |

## B Opus-5.5：11月3日“变笨”之争

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 开发者 Shane Melaugh 让 Gemini 分析自己1400次 Claude Code 编程记录 | L6 | ⚠️ | https://x.com/shanemelaugh/status/2119192075505893695 （`b-melaugh-main.json`；图 13） | 北京 11-08 06:15:38（UTC 11-07 22:15:38）；“I just had Gemini 4 Argon analyze 1,400 Claude Code sessions”。约 59.8 万浏览（14:3x 抓取）。个人开发者，非 Anthropic 员工；原始日志未公开。平台名不写进正文 |
| 称按 UTC 计11月3日起 Opus-5.5 在 high 档动手前想得少了，中位数从84降到54个 token | L6 | ⚠️ | thread 第 2 条 status 2119192078773260496 及其图（`b-melaugh-2119192078773260496.json`；图 15） | 图标题 ““High” effort now plans less upfront”，副标 “Before vs. after November 3 · effort = high”；“Before Nov 3 84 … After Nov 3 54 … Median thinking tokens before first tool call”。“动手前”= 首个工具调用前；样本前 14,560 回合、后 14,871 回合。图 1 写“Shift observed Nov 3 · ~18:00 UTC”（北京 11-04 约 02:00）、“Daily UTC averages”，作者按 UTC 日期分组，正文写“按 UTC 计11月3日”（反向核验第一轮 SHOULD_FIX 3）。Claude 已直接看图 14、15 核对数字与页脚 |
| Theo 用自己45.8万条回复反驳：变的像是给人看的思考摘要 | L6 | ⚠️ | https://x.com/theo/status/2119238026136330313 （`b-theo-2119238026136330313.json`；图 17、18） | 北京 11-08 09:18:13（UTC 01:18:13）。“Anthropic appears to be rolling out a change to the reasoning summarizer on a per-account basis.”（“appears”= 推断，正文写“像是”）。图注 “458k Opus 5.5 responses”，他自己 6 个账号的 Claude Code 流量；“回复”= responses |
| 平均推理 token 反而从369升到413 | L6 | ⚠️ | Theo 帖内图（图 18，深色底） | “Reasoning tokens per response (avg)” 369→413；同图工具调用/回复 1.03→1.06、摘要长度中位数 325→440 字符；图注 “not evidence that the model got worse” |
| 吠点①：Melaugh 图表页脚写着“可见摘要变了，不能证明模型参数变了” | L6 | ⚠️ | thread 第 1 条图（图 14） | 原句 “Visible summaries show a behavior shift, not proof of changed model weights.”；“参数”= weights（读者稿检查称“权重”看不懂）。初稿写“模型本身”，反向核验第二轮 SHOULD_FIX 2 指出权重不变不等于表现不变、不构成矛盾，已改回参数并补下一条 |
| 吠点①：修 bug 对比只有一例，自注“不是对照测试” | L6 | ⚠️ | thread 第 3 条图（图 16，`b-melaugh-2119192081251906001.json`） | 图题 “One bug. Two effort levels.”，“Same codebase · same day · audio playback bug”，页脚 “Single debugging comparison · Nov 7, 2026”“One case, not a controlled benchmark.” 比较的是同一天 high 与 xhigh 两档，不是 11/3 前后。Claude 直接看图核对 |
| 吠点①：帖子结尾却说“变笨是真的” | L6 | ⚠️ | thread 第 4 条 status 2119192083361599643（`b-melaugh-2119192083361599643.json`） | “so yeah, you're not imagining things. the nerf is real.” 北京 11-08 06:15:40 |
| 吠点①：官方文档说，用户看到的思考是另一模型写的摘要 | L1 | ✅ | https://platform.claude.com/docs/en/build-with-claude/thinking （`b-thinking-overview.txt`；Wayback 09-23、10-28、11-02、11-03 快照 `b-wb-thinking-*.txt`；图 19、22） | “what you see is never the raw chain of thought: the text in a thinking block is a summary of Claude’s reasoning”；“Summarization is processed by a different model from the one you target in your requests.” 11-02 与 11-03 21:02 UTC 快照逐字相同，非 11/3 新加。未写是哪个模型、摘要方式会不会变 |
| 吠点②：Melaugh 和 Theo 指标、样本不同，都只给图、没附原始日志 | L6 | ⚠️ | B1、B2 | Melaugh：自己 1,400 个会话、摘要措辞与首个工具调用前思考 token；Theo：自己 6 个账号 45.8 万条回复、平均推理 token。两人都只发图，未公开原始日志 |
| （已删）吠点②：截至11日下午未见 Anthropic 回应 | L1 | — | 状态页（`b-status-incidents.json`）、API 发布说明、Claude Code CHANGELOG 2.1.316–2.1.322（`b-cc-changelog-2.1.310-322.txt`）、GitHub issue 搜索（`b-gh-issues-search.json`）、HN 两帖 526 条评论 | 查到北京 11-11 15:00 前后。官方 X 时间线未能查（无列表接口）。反向核验第一轮 SHOULD_FIX 8 指出排他判断超出检索范围，正文已删此句。CHANGELOG 无摘要器条目；2.1.316/2.1.317 有子代理 effort 改动，未判断与此有关，正文不写 |
| 热度：Hacker News 讨论帖816分（未进正文） | L6 | — | HN 50422069（`b-hn-50422069.json`） | 816 分 / 511 评，北京 11-09 15:04 提交 |

## C Gemini-4-Carbon：全面开放

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌11日宣布，Carbon 向付费开发者和企业开放 | L1 | ✅ | https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-4-carbon-now-available/ （`c-blog.txt`；图 25、36）；Sundar Pichai 帖（fxtwitter JSON） | 博文页只写 Nov 10, 2026；Sundar 帖北京 11-11 00:04:43（UTC 11-10 16:04:43），“凌晨”按此。“available via the API in Google AI Studio and Gemini for business for all paying developers and enterprises”。两周前已通过 Fairwind Program 给部分网络安全团队（1028 期已发） |
| 每百万 token 输入2美元、输出10美元 | L1 | ✅ | 博文；定价页 https://ai.google.dev/gemini-api/docs/pricing （`c-api-pricing-rendered.txt`；图 26） | “at the same introductory price as Gemini 4 Argon ($2 per million input tokens and $10 per million output tokens)”；免费层 “Not available” |
| 普通用户限 Ultra 订阅 | L1 | ✅ | 博文（图 36）；订阅页（`c-gemini-subs.txt`） | “Try Carbon starting today in the Gemini app and AI Mode in Search if you're a Google AI Ultra subscriber.” 官方未写 AI Pro 不可用，订阅页 Pro 栏只列 Argon、Ultra 栏列 Carbon |
| 谷歌称，在与第三方 AA 合作的测试 MKA 上，Carbon 排第二，每题花费不到第一名的六分之一 | L1 | ✅ | 博文（图 27） | “we partnered with Artificial Analysis to launch the new Multimodal Knowledge Agent (MKA) benchmark … Carbon ranks #2 while being more than 6X lower in average cost per task compared to the top performing model.” “MKA”链接指向 AA 榜单页 artificialanalysis.ai/evaluations/mka。“more than 6X lower” 译作“不到六分之一”（反向核验第二轮 SHOULD_FIX 4：“低6倍”中文有歧义） |
| 吠点①：2美元是体验价，12月1日起输入、输出价都翻倍 | L1 | ✅ | 博文脚注 1（`c-blog.txt`，未截到图：折叠在 More Information）；定价页（图 26）；changelog（`c-api-changelog-rendered.txt`） | 脚注：“After the introductory period expires on November 30, 2026, the price of $4 per 1M input tokens and $20 per 1M output tokens will apply. Applies to the Carbon model intended for public access only.” 定价页：“$2.00 through November 30, 2026. $4.00 starting December 1, 2026.” 1028 时脚注没有到期日（`c-blog-1028.txt`） |
| （已删）吠点①后半：有中文稿标题写“价格仅 Fable 1/4”，届时是一半——读者稿检查后为省字数删去，记录保留 | L5（夸大实例，只引标题原句） | ⚠️ | https://k.sina.cn/article_5952915705_162d248f906703t0x8.html （`c-zh-sina.txt`），页面标 11.11 06:14 | 自媒体“AI信息Gap”发在新浪看点，标题“刚刚，Gemini 4 Carbon 终于开放！价格仅 Fable 1/4，Pro 会员却没份”；正文写的是“尝鲜价”，也提了到期。初稿写成“新浪新闻”，抽查发现是自媒体，已改“有中文稿”。Fable-5.2 单价 $8/$40（1109 期已核）：体验价 $2/$10 为 1/4，12/1 起 $4/$20 为 1/2 |
| 吠点②：“六分之一”与谷歌自家论文一致 | L1 | ✅ | DeepMind MKA 技术报告 https://storage.googleapis.com/deepmind-media/papers/mka_tech_paper.pdf （`c-mka-paper.txt` 第 11 页；图 32） | “At provider list prices, this corresponds to a 6.26× higher API cost ($35.28 vs. $5.63 per task)”，表 4 “list-price API cost (with prompt caching)”。论文由 Google MKA 博文链出，Carbon 博文未直接链论文，所以写“一致”不写“出自” |
| 吠点②：博文链接的 AA 榜单上约为五分之一，AA 是 Fable 最高档对 Carbon high 档，论文未标档位 | L3 | ✅ | https://artificialanalysis.ai/evaluations/mka （`c-aa-mka-board.*`；图 28）；AA 文 `c-aa-mka.txt`（图 34） | 榜单柱图：Fable-5.2 (Max) $31.98、Carbon (High) $6.29，≈5.08 倍；AA 文 “Claude Fable 5.2 averages $31.97 per task, over 5× Gemini 4 Carbon ($6.29)”（同站差 0.01）。AA 按标准价 $4/$20 计（页面载荷），与论文同为挂牌价，差异来源未说明，并列记录。档位：AA 榜 Fable-5.2 为 Max（带 fallback）、Carbon 为 High；论文未标档位（Claude 北京 11-11 17:00 前后对 `c-mka-paper.txt` 全文检索 effort、xhigh、max、(high)、budget 等，只见 “identical harness budgets”，无推理档位设置），口径未对齐（反向核验第一轮 SHOULD_FIX 5；正文把 2.2 个百分点明确归于论文得分） |
| （已删）吠点②：论文里第一名 Fable-5.2 只高2.2个百分点，差距不显著——反向核验第三轮后为补档位腾字数删去，批注图 19 仍有此句 | L1 | ✅ | MKA 技术报告第 11 页（图 32） | “for a statistically non-significant +2.22 pp Climb-WSR difference”，95% CI [−0.94, +5.49]；前三名 “statistically indistinguishable”。AA 榜：Fable-5.2 (max) 57%、Carbon (high) 54%（取整） |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| DeepMind 称用 AI 四周形式化验证 Signal 加密库的一对一会话 | L1 | ✅ | https://deepmind.google/blog/introducing-alphaproof-code/ 与论文页 https://google-deepmind.github.io/code-verification （`d-d1-alphaproof-blog.txt`、`d-d1-code-verification.txt`），11-10（官方未给时刻） | 博文 “our verification process took just four weeks”；论文 “the version implemented by libsignal v0.105.4, restricted to one-to-one sessions. (We leave group messaging as future work.)”，结果针对其自建的 libsignal-cli，“not Signal’s own applications”。“加密库”= libsignal，不是 Signal 应用。两个“first time”为 DeepMind 自述。行数口径：博文 “over six million lines of Lean”，论文 “about 6.5 million lines of Lean”——扫描称中文侧“650 万行”是误写，与论文不符；速览不写行数 |
| VulnCheck：至9日 curl 标34份 Claude 报告中19份非漏洞 | L3 | ✅ | https://www.vulncheck.com/blog/anthropic-glasswing-november-update （`d-d2-vulncheck.txt`），11-09（官方未给时刻） | “Of the 34 reports published so far, 19 have been marked as not a vulnerability, 6 have been confirmed as low severity, and 9 are still awaiting assessment.” 计数是 VulnCheck 对 curl 仓库标签的统计（L3，仅其自身统计），“so far”指其 11-09 发文时。Claude 北京 11-11 15:00 前后用 GitHub 搜索查 curl 仓库 11-01 以来带 vuln 标签的 issue 共 57 个，但无法只凭标签分出哪些属于 Anthropic 这批，未能复数；正文署 VulnCheck，不写成本号统计。“Claude 扫描报告”= Anthropic OSS Scanner 报告，研究页称扫描由 “our strongest models” 完成；“标”= curl 维护者打的 vuln:NONE 标签（反向核验第二轮 SHOULD_FIX 6）。“截至 11-09”的时点 40 字内放不下，未进速览图。这批来自 Anthropic 的 OSS Scanner：Anthropic 10/08 研究页原句 “fully model-generated, without human review or triage”（`docs/2610/1008/sources/c-oss-scanner-research.txt` 第39行；D 组勘误第2条称“未找到一手来源”有误）。Anthropic 台账 94.2% 是 “true positives of 6,376”（外部安全公司复核过的）且含重复与 won't fix，与 curl 这批不是同一管线，速览不并列 |
| 亚马逊工程师称工单分类用自家小模型替 Sonnet-4.6，按挂牌价估算省98% | L1 | ✅ | https://strandsagents.com/blog/amazon-saves-millions-replacing-sonnet-with-strands-decider/ （`d-d3-strands.txt`），11-09（官方未给时刻） | Strands 团队（AWS）博客，作者为两名亚马逊工程师：“performed just as well at 98% less cost”；每百万工单 Sonnet “about $22,200”、Decider E4B “about $500”；“These are published list prices. They leave out the Lambda”。98% 是按挂牌费率与实际用量估算的每百万工单成本，不是两模型单价比（反向核验第一轮 SHOULD_FIX 6，已加“估算”）。“自家小模型”= Strands Decider E4B（Gemma 4 E4B 底座、约19 GiB 显存）。“save millions … in a year”未给工单量，速览不写 |
| Anthropic 研究：小模型上，触发特征只在投毒数据里时，掺干净数据可成后门 | L1 | ✅ | https://alignment.anthropic.com/2026/homeopathic-learning/ 与 arXiv 2611.10110 （`d-d4-homeopathic.txt`、`d-d4-arxiv.txt`），博文 11-10，arXiv 北京 11-07 02:28 提交（UTC 11-06 18:28:50） | “poisoned training data with little effect on its own can create a hidden backdoor once clean, contrasting data is added”；实验在 “small open-weight models (Qwen3-8B and Gemma-3-4b-it) or a small MNIST network”；“We do not study any production training pipeline.” “This effect requires that the poison data have an identifying “trigger” feature that is absent from the clean data.”；“In the German numbers experiment, removing the trigger from all examples eliminates the effect.” 反向核验第一轮 BLOCKER 1 指出原写法漏了触发特征这一前提，第一轮补“带触发特征的”，第二轮 BLOCKER 1 指出还须写明干净数据不含该特征、结果写“可成”而非“反成”，已改。Anthropic Fellows 与 Truthful AI 合作 |
| OpenAI 称 GPT-6-Astra 思维链比前代更难监控 | L1 | ✅ | https://alignment.openai.com/cot-monitorability-model-lifecycle/ （`d-d5-openai-cot.txt`），11-10（官方未给时刻；Last-Modified 北京 11-11 06:14） | “with GPT-6 Astra we observed degradation in CoT monitorability for non-adversarial settings …, greater ability to perform CoT monitor evasion under adversarial testing … compared to prior models”；“models are becoming harder to monitor through their CoT”。两个代理指标 “imperfect proxies”；附录 B 称可控性 “a noisy proxy” |
| Anthropic 向付费版推出 Claude 白板、表格测试版，企业版默认关 | L1 | ✅ | https://claude.com/resources/articles/sheets-whiteboards-data （`d-d8-claude-sheets.txt`），11-10（官方未给时刻） | “launching Claude Whiteboards and Claude Sheets in beta”；“These are available on all paid plans.” “On Enterprise plans, Claude Whiteboards and Sheets are off by default”（反向核验第一轮 SHOULD_FIX 7，已补）；Team 版默认开；Dashboards 同日更名 Claude Data 转正 |
| 通义称 Qwen4-Max 开放预览，可在 Qwen APP 等提前试用 | L1 | ✅ | 通义官方账号帖（`d-d7-qwen4max-fxtwitter-raw.json`），北京 11-11 10:01:09（UTC 02:01:09） | “Qwen4-Max is now in preview on Alibaba’s Token Plan. You can also get early access to the new model on QwenWork, Qoder, Qwen APP, and QwenStudio.” 帖内无跑分、价格、权重；官方博客与模型卡未找到，Token Plan 定价页未提 Qwen4-Max。平台名不写进速览 |

不收（D 组已取证）：OpenRouter MCP vs CLI——原文同一个 19,436 token 在摘要里归 Sonnet-5.5、在脚本输出与后文里归 Haiku-5.5，自相矛盾，且只测一个服务器、一个任务。

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| help-review：连接 Gemini 后，Gemini 处理的关联应用数据中，有一部分由人工审核员（包括受过培训的服务商）审核 | L1 | ✅ | 帮助页 16836988（图 01，`help-review-raw.png`） | 原句 “When connected to Gemini, a subset of Connected Apps data processed by Gemini is reviewed by human reviewers (including trained service providers)” |
| help-review：如果你的应用里有不想让审核员看到的机密信息，请不要把它们关联到 Gemini 应用 | L1 | ✅ | 同上 | “Please don't connect your apps to Gemini Apps if they contain confidential information that you wouldn’t want a reviewer to see.” |
| help-review：在某些情况下，例如邮件或文件很短，或与回答提问特别相关时，摘要可能就是邮件或文件本身 | L1 | ✅ | 同上 | “In some cases, for example, where the email or file is brief or particularly relevant to respond to the prompt, the summary may just be the email or file.” |
| jan-snapshot：连接 Gemini 后，Gemini 处理的关联应用数据中，有一部分由人工审核员（包括受过培训的服务商）审核 | L1（Wayback） | ✅ | 2026-01-14 21:36:56 UTC 快照（图 12e，`jan-snapshot-raw.png`；`a-wb-help-20260114.txt`） | 快照只证明不晚于此时存在；页脚写“2026-01-14 UTC” |
| blog-promise：（2023 年原文）如果你在 Bard 里使用 Workspace 扩展，你在 Gmail、文档和云端硬盘里的内容不会被人工审核员看到 | L1 | ✅ | 2023 博文（图 06，`blog-promise-raw.png`） | 原句另有“used by Bard to show you ads, or used to train the Bard model”，批注只高亮人工审核半句 |
| blog-promise：2026 年 10 月 16 日更新 | L1 | ✅ | 同上 | “(Update, October 16, 2026:” 日期为谷歌自署 |
| blog-promise：（更新说明）在有限情况下，这些数据也可能接受人工审核 | L1 | ✅ | 同上 | “It can also be subject to human review in limited circumstances.” 前句主语为 “that same subset of data”（用户开启新功能后被处理的 Workspace 数据），译注写“这些数据” |
| melaugh-footer：图中标注：变化出现在 11 月 3 日约 18:00（UTC），即北京时间 4 日凌晨约 2 点 | L6 | ⚠️ | Melaugh 图 1（图 14，`melaugh-footer-raw.png`） | 图上 “Shift observed Nov 3 · ~18:00 UTC”；北京时间为本号换算 |
| melaugh-footer：可见的摘要显示出行为变化，但这不能证明模型权重（参数）变了 | L6 | ⚠️ | 同上 | “Visible summaries show a behavior shift, not proof of changed model weights.” |
| melaugh-upfront：比较的是 high 档，11 月 3 日前后 | L6 | ⚠️ | Melaugh 图 2（图 15，`melaugh-upfront-raw.png`） | “Before vs. after November 3 · effort = high” |
| melaugh-upfront：首次调用工具前的思考 token 中位数：11 月 3 日前 84，之后 54 | L6 | ⚠️ | 同上 | “Before Nov 3 84 … After Nov 3 54 … Median thinking tokens before first tool call” |
| theo-work：每条回复的平均推理 token：切换前 369，切换后 413 | L6 | ⚠️ | Theo 图（图 18 表格与页脚拼接，`theo-work-raw.png`，深色底） | “Reasoning tokens per response (avg) 369 413”，列名 “Before switch / After switch” |
| thinking-summary：你收到的思考文字是 Claude 完整思考过程的摘要，而不是原始思维链 | L1 | ✅ | thinking 文档（`thinking-summary-raw.png`，按 700 px 宽重新渲染；`b-thinking-overview.txt`） | “When display is "summarized", the thinking text you receive is a summary of Claude’s full thinking process rather than the raw chain of thought.” 译注省去 display 设置条件（Claude Code 摘要显示即此档） |
| thinking-summary：摘要由另一个模型生成，不是你请求里调用的那个模型 | L1 | ✅ | 同上 | “Summarization is processed by a different model from the one you target in your requests.” |
| carbon-pricing：Gemini 4 Carbon 不提供免费档 | L1 | ✅ | 定价页（图 26，`carbon-pricing-raw.png`） | “Gemini 4 Carbon is not available on the Free tier.” |
| carbon-pricing：输入价：到 2026 年 11 月 30 日为每百万 token 2 美元，12 月 1 日起 4 美元 | L1 | ✅ | 同上 | “$2.00 through November 30, 2026. $4.00 starting December 1, 2026.”（Standard、Paid Tier 栏） |
| carbon-pricing：输出价（含思考 token）：到 11 月 30 日为 10 美元，12 月 1 日起 20 美元 | L1 | ✅ | 同上 | “Output price (including thinking tokens) … $10.00 through November 30, 2026. $20.00 starting December 1, 2026.” |
| blog-6x：Carbon 排第二，平均每题成本不到第一名的六分之一（原文“低 6 倍以上”） | L1 | ✅ | 发布博文（图 27，`blog-6x-raw.png`） | “Carbon ranks #2 while being more than 6X lower in average cost per task compared to the top performing model.” |
| aa-cost：Claude Fable 5.2（Max 档）：每题 31.98 美元 | L3 | ✅ | AA MKA 榜单（图 28，`aa-cost-raw.png`） | 柱顶标 “$31.98”；同站文章写 $31.97 |
| aa-cost：Gemini 4 Carbon（High 档）：每题 6.29 美元 | L3 | ✅ | 同上 | 柱顶标 “$6.29”（视觉复核要求删去本号计算的倍数） |
| mka-cost：按挂牌价（含提示缓存）计，Fable 5.2 的 API 成本约为 Carbon 的 6.26 倍（每题 35.28 美元对 5.63 美元） | L1 | ✅ | MKA 技术报告第 11 页（图 32，`mka-cost-raw.png`） | “At provider list prices, this corresponds to a 6.26× higher API cost ($35.28 vs. $5.63 per task)”；表 4 题 “list-price API cost (with prompt caching)” |
| mka-cost：Climb-WSR 得分高 2.22 个百分点，统计上不显著 | L1 | ✅ | 同上 | “for a statistically non-significant +2.22 pp Climb-WSR difference” |

## 反向核验

### 第一轮（北京 11-11 约 15:40，codex-reviewer：gpt-6-astra，effort medium，只读）

审的是读者稿检查之前的一版正文（标题“真人看你Gmail？Opus变笨了？”）与事实清单；卡片与两份 README 尚未生成（NEEDS_VERIFICATION 10），留待第二轮。Claude 逐条核对存档后：

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | BLOCKER | 顺势学习速览漏了“投毒数据须带干净数据没有的触发特征”这一前提 | 采纳。`d-d4-homeopathic.txt` 摘要句 “This effect requires … ‘trigger’ feature …” 属实；速览改为“带触发特征的投毒数据掺入干净数据反成后门” |
| 2 | SHOULD_FIX | “全面开放”放大开放范围 | 采纳。省流与小标题改“向付费用户开放” |
| 3 | SHOULD_FIX | “11月3日”是作者按 UTC 分组的日期，北京时间已是 4 日凌晨 | 采纳。正文改“按 UTC 计11月3日起” |
| 4 | SHOULD_FIX | L5/L6 行标 ✅ 违反分级规则 | 采纳。L5、L6（含混合级）行一律改 ⚠️；“已找到”只表示原帖确有此说，不表示其测量为真 |
| 5 | SHOULD_FIX | 6 倍、约 5 倍、2.2 个百分点三者口径与档位未说明 | 部分采纳。正文把 2.2 个百分点明确归于“论文里……得分”；AA 档位（Fable Max、Carbon High）与论文未标档位写入 C 节备注，正文无篇幅再展开 |
| 6 | SHOULD_FIX | “挂牌价省98%”易读成单价降 98% | 采纳。改“按挂牌价估算省98%” |
| 7 | SHOULD_FIX | Claude 白板、表格速览漏了企业版默认关闭 | 采纳。改“向付费版推出……测试版，企业版默认关” |
| 8 | SHOULD_FIX | “截至11日下午未见 Anthropic 回应”超出实际检索范围（官方社媒时间线未查） | 采纳。正文删去此句 |
| 9 | SHOULD_FIX | arXiv 提交日未换算北京时间 | 采纳。改北京 11-07 02:28（UTC 11-06 18:28:50），仅涉及内部记录 |
| 10 | NEEDS_VERIFICATION | cards.toml 与两份 README 缺失 | 第二轮补审 |
| 11 | NEEDS_VERIFICATION | 84→54、45.8 万、369→413 与页脚只有转录，原图未独立复核 | Claude 已直接看图 14、15、18 核对：页脚 “Visible summaries show a behavior shift, not proof of changed model weights.”、“Before Nov 3 84 … After Nov 3 54”、“Reasoning tokens per response (avg) 369 413”、“458k Opus 5.5 responses” 均与转录一致；视觉复核时再由复核方看图 |


### 第二轮（北京 11-11 约 16:20，codex-reviewer：gpt-6-astra，effort medium，只读）

审的是第一轮返修、读者稿改写后，卡片与两份 README 已生成的版本；批注译注一节此时尚未写入（第 5 条）。

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | BLOCKER | 顺势学习速览只补了“带触发特征”，没写干净数据不含该特征；“反成后门”过于确定 | 采纳。改“触发特征只在投毒数据里时，掺干净数据可成后门” |
| 2 | SHOULD_FIX | “不能证明模型本身变了”把“权重”扩大成“模型本身”，与“变笨是真的”不构成矛盾 | 采纳。改回“不能证明模型参数变了”，并补 Melaugh 唯一的能力对比只是同一天两档修一个 bug、自注“不是对照测试”（图 16，Claude 直接看图） |
| 3 | SHOULD_FIX | 谷歌 6 倍与 AA 约 5 倍未注明口径未对齐 | 采纳。吠点②加“口径未对齐” |
| 4 | SHOULD_FIX | “高 6.26 倍”“低6倍”中文有歧义 | 采纳。译注改“约为 Carbon 的 6.26 倍”，正文改“不到第一名的六分之一”“约为五分之一” |
| 5 | SHOULD_FIX | 两条批注译注无对应事实行 | 已在“批注译注”一节补齐（含“输出价含思考 token”） |
| 6 | SHOULD_FIX | curl 速览丢了时点与判定主体 | 部分采纳。“非漏洞”改“标非漏洞”；“截至 11-09”在 40 字内放不下，记入速览行备注 |
| 7 | NEEDS_VERIFICATION | 84→54、369→413、458k、页脚与 AA $31.98 只有转录 | Claude 已直接看图 14、15、16、18、28 核对；视觉复核（Codex 看图）未报数字不符 |

## 视觉复核

北京 11-11 约 16:15，Codex（gpt-6-astra，effort medium，只读，`barking review --visual`），附 32 张图（省流、速览、事件卡、吠点卡、封面与 11 张批注图及底图）。结论 PASS，无 BLOCKER。Claude 处理：

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX | Theo 图黑底小字，高亮后页脚难读 | 采纳。改为表格与页脚两段拼接的放大底图 `theo-work-raw.png`，只高亮推理 token 一行，页脚不高亮 |
| 2 | SHOULD_FIX | Theo 图标题“变的是摘要”比正文“变的像是”更确定 | 采纳。改“摘要与推理用量” |
| 3 | SHOULD_FIX | aa-cost 译注“约为它的 5.1 倍”是本号计算 | 采纳。删去 |
| 4 | SHOULD_FIX | mka 译注“高 6.26 倍”易读成 7.26 倍 | 采纳。改“约为 Carbon 的 6.26 倍” |
| 5 | SHOULD_FIX | jan-snapshot 译注前缀“1 月的快照里已有同一句”不是翻译 | 采纳。译注只译原句，日期由标题与来源行承担 |
| 6 | SHOULD_FIX | 海外社媒截图来源只写泛称 | 采纳。作者与发帖日期补在图题（“Melaugh 11/8 帖”“Theo 11/8 反驳帖”，北京时间）；页脚放不下，保持“海外社媒原文截图（存档日期）” |
| 7 | NICE_TO_HAVE | mka 译注②“只高”带评价语气 | 采纳。改“高” |

改后重渲 10、11、12、17、18、19 号与速览、吠点卡；Claude 第三轮返修后另做一张缩略拼图逐张复查版式（见下）。

### 第三轮（北京 11-11 约 16:45，codex-reviewer：gpt-6-astra，effort medium，只读）

结论 PASS，无 BLOCKER。Claude 处理：

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX | blog-6x 译注仍是“低 6 倍以上” | 采纳。改“不到第一名的六分之一（原文“低 6 倍以上”）” |
| 2 | SHOULD_FIX | curl 速览丢了截至日期 | 采纳。改“VulnCheck：至9日 curl 标34份 Claude 报告中19份非漏洞”（34 份都已打标签，含 9 份 ASSESS） |
| 3 | SHOULD_FIX | Melaugh 的数字由 Gemini 分析得出，正文未写 | 采纳。改“让 Gemini 分析自己1400次……” |
| 4 | SHOULD_FIX | 吠点卡②“两边”脱离正文无对象，“都没公开”超出检索范围 | 采纳。改“Melaugh 和 Theo 测的指标、样本不同，都只给图、没附原始日志” |
| 5 | SHOULD_FIX | 跨来源比较未公开已知档位 | 采纳。补“AA 是 Fable 最高档对 Carbon high 档，论文未标档位”；为腾字数删去“论文里第一名只高2.2个百分点”（批注图 19 仍有） |
| 6 | NEEDS_VERIFICATION | 社媒图与 AA 柱顶数字只核到转录 | 同第二轮第 7 条：Claude 已直接看图 14、15、16、18、28；AA 柱顶 $31.98 与同站文章 $31.97 并列记录 |

省流卡 Carbon 一条因版面改为一行“Carbon 向付费开发者和企业开放”（逐字取自事实段）。改后重渲省流、速览、事件卡与吠点卡，lint 0 个错误。

### 第三轮之后的改动（未经跨模型复核）

第三轮 PASS 后 Claude 又改了：吠点卡②（点名 Melaugh 和 Theo）、Carbon 吠点②（补档位、删 2.2 个百分点）、curl 速览（“至9日”“标”“Claude 报告”）、Theo 批注图换底图 `theo-work-raw.png`；终审前自查再改两处：吠点①“能力对比只有一个 bug”改“修 bug 对比只有一例”（Melaugh 图 2 另有 11/3 前后的“Script & edit crashes +47%”指数，未定义何为崩溃，不能说能力对比只有一例）；省流补回“Claude”。这些改动由 Claude 对照存档自查，未再送 Codex，交付时已告知 Develata。curl 速览里“Claude 报告”是据 OSS Scanner 研究页 “our strongest models” 的推断；“34 份都已打标签”据 VulnCheck 的 19/6/9 三类相加为 34，未逐条复数。


终审前复查：北京 11-11 约 17:10 重渲 13 张后，Claude 把 02、03、04、08、09、10、11、14、15、17、18、19 缩成 4×3 拼图逐张看版式（01、05、12 另行打开），未见溢出、压叠或高亮错位；文字一致性由 lint 逐字比对。
