# 1028 期事实清单

## 省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌 Carbon 评测登顶只领先1分，本周起将只先给少数人 | L1/L3 | ✅ | 见 A 节 | 登顶＝AA 指数 v5 第一（59 对 58）；少数人＝“a subset of cyber defenders”。 |
| 网页脚本藏“危险词”专拦 AI，是研究者推测 | L1 | ✅ | 见 B 节 | “could this be some form of anti-LLM obfuscation?” |
| Claude Code 状态栏占估算花费16.4%是一家公司的数，Anthropic 员工称上报数据的全体用户按 API 标价约2% | L1/L2 | ✅ | 见 C 节 | 16.4%＝Flamerix 一周估算花费份额；2%＝上周全体上报遥测用户、按 API 标价。“全体”指上报遥测的用户，正文 C 节写全。 |

## A Gemini 4 Carbon

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Google 29日凌晨发布 Gemini-4-Carbon | L1 | ✅ | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-carbon/ ；a-blog.txt；图 01 | 博文只有日期 Oct 28, 2026；Pichai 帖北京 10-29 01:01:48（17:01:48 UTC），HN 提交 01:01:23。“our strongest model with state-of-the-art cybersecurity and long horizon capabilities”。 |
| 在 Artificial Analysis 新版综合指数上以59分排第一 | L1/L3 | ✅ | a-blog.txt；a-aa-leaderboard.txt；a-aa-article.txt；图 08/09 | “ranks #1 on the recently updated v5.0 of Artificial Analysis Intelligence Index at 59”；AA 榜 Carbon (high) 59。 |
| 称本周起将先向部分网络安全防御者开放，没给大众开放日期 | L1 | ✅ | a-blog.txt；a-aa-article.txt；a-api-models-opencli.txt；图 02 | “本周起”= Starting this week（计划措辞，未见已开放的一手确认）。“Starting this week, we will make Carbon available to a subset of cyber defenders and will gradually phase access to additional trusted partners through our Fairwind Program … before making Carbon available to developers, businesses, and consumers via our product surfaces as soon as possible.” AA：“not yet publicly available”；Gemini API 模型页、价格页、Vertex 文档均无 Carbon（北京 10-29 取）。同段美国政府自愿预发布测试，正文不写（风控）。 |
| （正文已删）首发价每百万 token 输入2美元、输出10美元 | L1 | ✅ | a-blog.txt；图 02 | “same introductory price 1 as Argon at $2 per million input tokens and $10 per million output tokens”；缓存输入 95% off 未写进正文。 |
| 只领先1分，档位还不同：Carbon（high 档）59，Claude-Opus-5.5（max 档，带 fallback）58 | L3 | ✅ | a-aa-leaderboard.txt；图 09 | AA 榜：Gemini 4 Carbon (high) 59；Claude Opus 5.5 (max with fallback) 58；Opus 5.5 (xhigh with fallback) 与 GPT-6 Astra (max) 57（AA 页面，北京 10-29 取）。 |
| Artificial Analysis 称它编程和知识工作没领先 | L3 | ✅ | a-aa-article.txt；图 08 | “with the #1 score in 4 of the 9 Index evaluations … not leading in coding and knowledge work”。 |
| （正文已删）谷歌自家对比表17行两行没领先：OSWorld 上 Opus-5.5 87.2%，Carbon 81.5%——留作批注图 | L1 | ✅ | a-dm-models.txt；图 07 | deepmind.google/models/gemini 对比表 17 行（AA Index 至 SEC-bench Pro）；不领先：OSWorld-2.1 Offline subset（Opus 5.5 87.2%、Carbon 81.5%）、FrontierSWE 2（Astra 65.5%、Opus 62.3%、Carbon 61.2%）。厂商自家对比表。 |
| （正文已删，读者稿检查：读者不在乎）首发价没写截止日，期满涨到4美元、20美元；Argon 写明到11月30日 | L1 | ✅ | a-blog.txt；a-api-pricing-opencli.txt；图 02、11 | 脚注 1：“After the introductory period expires, the price of $4 per 1M input tokens and $20 per 1M output tokens will apply.” 无日期。Gemini API 价格页 Argon：“$2.00 through November 30, 2026”。AA：“50% launch discount, with no confirmed end date”。 |
| 网络安全成绩是关安全限制测的，三项里两项谷歌自己跑 | L1 | ✅ | a-blog.txt；a-methodology.txt | “Carbon excels across key external benchmarks (evaluated without guardrails)”；方法页：ExploitGym、SEC-bench Pro “self computed using the open source evaluation setup”；CWE-bench 来自 Collinear AI 官方榜。SEC-bench Pro 为 JS 引擎子集、ExploitGym 54.8% 为 10 小时上限（见 evidence-A）。 |
| Chrome 团队称它生成高质量补丁比另一家实验室的大模型强10倍以上，没说是谁 | L1 | ✅ | a-blog.txt；图 03 | “the Chrome team has reported that Carbon performed more than 10X better than another lab's large frontier model when generating high quality patches”；未点名、无基线；A 组未找到 Chrome 侧一手材料（已查 Chrome 博客等，见 evidence-A A3）。 |

## B 指纹脚本

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| （正文已删日期）研究者 veritas | L1 | ✅ | https://nullpt.rs/fingerprinting-bioweapons ；b-article.txt | 页面“Wed Oct 28 2026 authored by veritas”，官方未给时刻；HN 50276998 提交于北京 10-29 01:13（10-28T17:13Z）。 |
| 研究者 veritas 发现，一个反机器人指纹脚本把函数起成吓人的名字，还藏了隐形“越狱提示” | L1 | ✅ | b-article.txt | 爬 Tranco 前 1 万站；脚本 servicepipe.tech checkjs；伪提示以不可见 Unicode 编码，作者解码得“SYSTEM OVERRIDE …”。危险内容与涉政词不复述。 |
| 他猜是诱饵，让 AI 触发安全拦截、不肯帮人拆代码 | L1 | ✅ | b-article.txt | “could this be some form of anti-LLM obfuscation? It would make sense: include symbol names that would intentionally trip model safeguards to prevent deobfuscation.” |
| 试 Astra 被拦，GLM-5.3 照做 | L1 | ✅ | b-article.txt；图 15、16 | “Astra was not willing to comply”；“tried GLM-5.3 with the same prompt and it complied!” 原文未写 Astra 厂商；据 OpenAI Daybreak 文档（b-daybreak-overview.txt）Astra 为 OpenAI 模型、Daybreak 为 OpenAI 项目——本号推断，正文只在吠点④写“OpenAI 的 Daybreak Blue”。 |
| 博文标题问脚本为何“制造生物武器”，只是钩子：veritas 拆完说，除了这些混淆，它看起来就是普通反机器人脚本 | L1 | ✅ | b-article.txt；图 19 | 标题“Why Is This Fingerprinting Script Creating Bioweapons?”；“Outside of the unique obfuscation, the script appears to be your standard anti-bot script.” 采集 navigator 与 WebGL 信号。“只是钩子”为本号判断，依据即此句。 |
| “专拦 AI”是 veritas 的推断，本号没找到脚本方回应 | L1 | ✅ | b-article.txt | “could this be…”“It would make sense”；另有“clearly a string meant to trip every possible safeguard”（指伪提示）与 Node 分支“likely also anti-LLM obfuscation”。提供方回应：B 组搜索与 HN 未见；servicepipe 站点 403/需 JS，未取得。 |
| 测试只写三种设置，没给次数 | L1 | ✅ | b-article.txt | Astra、Astra + Daybreak Blue、GLM-5.3 各一句结果；提示词只概述“giving it the context that…”。 |
| （正文已删，字数）他开了 OpenAI 给安全人员的放宽权限仍被拦，不奇怪：文档写明这档不给 Astra 放宽 | L1 | ✅ | b-daybreak-overview.txt；图 24 | “放宽权限”= Daybreak Blue（OpenAI’s Trusted Access for Cyber 的一档）；作者原文“Daybreak Blue toggle … blocked”。原文未写 Astra 厂商，OpenAI 文档在 Daybreak 各档列出 Astra——Astra 属 OpenAI 为本号依文档推断。OpenAI 帮助中心：Daybreak Blue “Reduced refusals on supported mainline models … Astra retains standard safeguards.” Astra 减少拒绝只在 Daybreak Red。另：Daybreak 是网络安全项目，拦截提示写的是生物研究（图 15）。 |

风控：原文解码出的伪提示含涉政敏感词（作者称“likely as a way to combat Chinese models”），正文、卡片、译注不提；图 17 含 Tiananmen 字样，只作备用图、不上平台。

## C Claude Code 后台摘要

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Flamerix 公司称，Claude Code 工作中的后台 agent 约每30秒把整段对话发给模型写状态 | L1 | ✅ | c-gist-README.md | Gist “by Flamerix AI”：“Every ~30 seconds, for every background agent … sends that agent's entire transcript … ‘describe your most recent action in 3-5 words’”。“正在干活的”取员工回复限定（“only applies to background agents that are actively working”），Gist 原文写 every background agent。Gist API created_at 北京 10-27 06:27，最早修订北京 10-29 02:43。 |
| 他们一周里这项占估算花费16.4% | L1 | ✅ | c-gist-README.md；图 26/27-v2 | 表头“week before the hook”；“share of all Claude Code spend 16.4 %”“share of all API requests 19.7 %”；样本规模未给。 |
| 一位自称在 Claude Code 工作的 Anthropic 员工称 | L2（身份自述） | ⚠️ | c-hn-edwinarbus.json；c-fx-user-edwinarbus.json | HN 50284699，北京 10-29 12:52（04:52Z）：“I work on Claude Code”，留 anthropic.com 邮箱；公开简介“Head of Product & Community Comms, @claudeai”。未见 Anthropic 官方页署名，正文用“自称”。 |
| 上周上报数据的全体用户里，这项按 API 标价约占2% | L2 | ✅ | c-hn-edwinarbus.json | “Last week, across Claude Code users who send us telemetry, agent_summary was about 2% of cost at API list prices and just under 4% of requests. In the terminal, where nearly all of these calls happen, it was about 6% of cost.” “上报数据的用户”= users who send us telemetry；“命令行里”= In the terminal 子集。 |
| 分母不同：16.4%只是一家公司一周的账，自称员工的回复者认为它像主要跑后台 agent 的团队 | L2 | ✅ | 同上 | “Your 16.4% sounds like it was for a team running mostly background agents, and doesn't seem like a typical session.” “谁也推翻不了谁”为本号判断。 |
| 16.4%是工具估算，不是账单 | L1 | ✅ | c-gist-README.md | “Spend here means Claude Code's own per-request estimate (`cost_usd`), not an invoice: on a subscription, what you lose is rate-limit headroom.” |
| Flamerix 标题说“烧掉15–20%的 token”，测的却是花费和请求 | L1 | ✅ | c-gist-README.md | 标题“Claude Code burns ~15–20% of your tokens on a 3-word status line”；表中只有 spend 与 API requests 份额（另有“input tokens per output token ≈ 9,000 : 1”，非份额）。 |
| 此人还称2.1.311版起修了最费钱的情况，本号在更新日志里没找到 | L2/L1 | ✅ | c-hn-edwinarbus.json；c-changelog.txt | “We just rolled out a fix for the most expensive case, summaries that miss the prompt cache (2.1.311+), and right now we're working on a much cheaper mode…”。CHANGELOG 2.1.311–2.1.312 节未找到明确对应后台 agent 摘要缓存未命中修复的条目（2.1.311 有一条无关的 `/effort` prompt cache miss 提示修复），全文无“agent_summary”（已读范围：c-changelog.txt 全文，北京 10-29 取）。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| GitHub 称 Copilot 记忆付费版正式可用，“做梦”整理仍是自愿预览 | L1 | ✅ | https://github.blog/changelog/2026-10-28-github-intelligence-puts-your-code-context-and-history-to-work/ ；d-d1-copilot-changelog.txt | “GitHub Copilot Memory … is now generally available on paid Copilot plans.”“Copilot dreaming, now in public preview”“Dreaming is opt-in.” 官方只给日期 10/28/2026。 |
| OpenRouter 称开源模型自测首字延迟中位数降17%，千次请求贵6% | L1 | ✅ | https://openrouter.ai/blog/announcements/how-we-rebuilt-provider-routing/ ；d-d2-openrouter-routing.txt | 一周、2400 万+请求切换实验：“TTFT p50 1,007 ms → 837 ms 16.9% lower”；“Cost per K requests $1.25 → $1.32 6.0% higher”。厂商自测；实验对象为“a representative sample of open source models”。首字延迟为 p50。 |
| GitHub 称9月超四成拉取请求由开发者用 agent 起头，3月为17% | L1 | ✅ | https://github.blog/news-insights/octoverse/octoverse-commits-pushes-reach-new-highs-in-2026-as-developers-build-with-agents/ ；d-d4-octoverse.txt | “As of September, developers start more than 40% of pull requests with an agent. In March, it was 17%.” |
| Stack Overflow 9月新提问接口查得969条，去年同月5072条 | L1 | ✅ | https://api.stackexchange.com/2.3/questions?site=stackoverflow&fromdate=1788220800&todate=1790812800&filter=total ；d-se-count-2026-09.json、d-se-count-2025-09.json | Stack Exchange 官方 API，UTC 整月；Claude 北京 10-29 复查同为 969 与 5072。WorkBuddy 扫描写 961/5044，不采用。月份按 UTC；计数为查询时接口返回的数量（已删除问题是否计入未核），正文写“接口查得”；当月历史新增总数未核。HN 50282454。 |
| MicroZig 项目准则禁用大模型，连谈论是否用了 AI 也不许 | L1 | ✅ | github.com/ZigEmbeddedGroup/microzig 行为准则；d-d6-microzig-coc-main.txt | “Strict No LLM / No AI Policy … takes effect October 28th, 2026.”“No talking about use of chatbot/LLM services.” 提交 #1103 北京 10-29 08:13。 |

## 批注译注（图上额外事实）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 首发优惠期结束后，按每百万输入 token 4美元、输出 token 20美元计价（图 40 译注③） | L1 | ✅ | a-blog.txt | 脚注 1 原句；未给截止日（Argon 写到 2026-11-30，见 a-api-pricing-opencli.txt）。 |
| ……之后尽快通过我们的产品向开发者、企业和消费者开放（图 40 译注②） | L1 | ✅ | a-blog.txt | “before making Carbon available to developers, businesses, and consumers via our product surfaces as soon as possible”。 |
| 正向选定用户逐步推出，尚未公开开放（图 41 译注①） | L3 | ✅ | a-aa-article.txt | “is being rolled out to selected users and is not yet publicly available”。 |
| 关掉前一周：占 Claude Code 全部花费的16.4%（主对话15.7%）（图 47 译注①） | L1 | ✅ | c-gist-README.md | 表头 “week before the hook”（装上关闭摘要的补丁前一周）；“16.4 % (main conversation: 15.7 %)”。 |
| 上报遥测的 Claude Code 用户中 agent_summary 请求数不到4%（图 48 译注②） | L2 | ✅ | c-hn-edwinarbus.json | “just under 4% of requests”。 |
| Carbon 在其他方面进步很大，但在编程和知识工作上没有领先（图 41 译注②） | L3 | ✅ | a-aa-article.txt | AA 的判断：“Carbon makes substantial gains elsewhere but is not leading in coding and knowledge work”。 |
| Carbon 在主要外部基准上表现出色（测试时未开安全限制）（图 42 译注） | L1 | ✅ | a-blog.txt | Google 自评：“Carbon excels across key external benchmarks (evaluated without guardrails)”。 |
| 不是账单：订阅用户损失的是额度余量（图 47 译注③） | L1 | ✅ | c-gist-README.md | Flamerix 的判断：“not an invoice: on a subscription, what you lose is rate-limit headroom.” |
| 此内容无法显示（作者截图中 Astra 的拦截提示）（图 44 译注③） | L1 | ✅ | 图 15；b-article.txt | 作者图注 “Astra refusing to deobfuscate the script, citing safety risks around biological research.” |

## 反向核验

### 第一轮（正文与事实清单）

复核方：Codex gpt-6-astra（medium，WSL 只读），审的是读者稿检查之前的版本；以下逐条按当前正文判断。

1. BLOCKER “强10倍”丢了“生成高质量补丁”条件，且引号像原话：采纳，改为“Chrome 团队称它生成高质量补丁比……强10倍以上”，去引号。
2. BLOCKER 2% 被扩大为全体用户、丢了 API 标价口径：采纳。省流与 C 节均写“上报数据的全体用户”“按 API 标价”；省流 16.4% 补“估算花费”。原 C①“一家公司一周对全体用户”已在读者稿修改中删去。
3. BLOCKER 卡片、两份 README、速览与译注行缺失：属实，第一轮按流程只审正文；卡片做完后第二轮短审。
4. SHOULD_FIX “目前只给”把本周计划写成已开放：采纳，改“本周起先向部分网络安全防御者开放”，省流“本周起只先给少数人”。
5. SHOULD_FIX 漏 fallback：采纳，补“max 档，带 fallback”，不解释其机制。
6. SHOULD_FIX “就是”加强了“appears to be”：采纳，改“除了这些混淆，它看起来就是普通反机器人脚本”。
7. SHOULD_FIX 清单对更新日志的否定过宽：采纳，清单收窄；正文改“没找到这项摘要修复”。

### 第二轮（卡片、速览、译注与配图说明）

复核方：Codex gpt-6-astra（medium，WSL 只读）。

1. BLOCKER 省流卡 C 摘掉“Flamerix 公司称”：采纳，恢复主张者；为让三条放进版面，A 只留到“排第一”，B 只留到“吓人的名字”（开放范围与越狱提示见事件卡）。
2. SHOULD_FIX 吠点卡主张者不完整：采纳，正文与卡同步改为“Artificial Analysis 称”“veritas 拆完说”“veritas 的推断”“这位自称员工认为”“他还称”。
3. SHOULD_FIX 开放状态未写明计划：采纳，正文“称本周起将先向……开放”，省流“本周起将只先给少数人”。封面副标题“只先给少数人”无时间词，保留，交付时请 Develata 判断。
4. SHOULD_FIX OpenRouter 漏开源模型样本：采纳，速览改“开源模型实验”，清单补原句。
5. SHOULD_FIX 三处译注未登记：采纳，“批注译注”一节补三行，注明 AA 判断、Google 自评、Flamerix 判断。
6. SHOULD_FIX 公众号张数：采纳，改“正文插图11张，另有横版封面1张”。
7. NEEDS_VERIFICATION 提问量口径：采纳，速览改“接口查得”，清单写明是查询时接口返回数、删除口径与当月历史新增总数未核。
- 复核未看的“This content can't be shown”：Claude 已开图 15 目视核对，原句在 Astra 拦截框内。

## 视觉复核

复核方：Codex gpt-6-astra（medium，WSL 只读），28 张附图。结论 PASS，无 BLOCKER。

1. SHOULD_FIX 速览 OpenRouter 漏 p50 与厂商自测：采纳，改“OpenRouter 称开源模型自测首字延迟中位数降17%，千次请求贵6%”。
2. SHOULD_FIX 吠点卡 3 主张者依赖事件卡：采纳，①改“自称员工的回复者认为”，③改“此人还称”（正文同步）。
3. SHOULD_FIX 深色截图高亮压暗浅色字：不采纳本期，属 annot.html 模板对深色底的通用问题，另议；文字仍可读。
4. SHOULD_FIX 45 窄截无上下文：采纳，底图改为图 15 上部（作者给 Astra 的请求背景）叠图 16 三句结果。
5. NICE_TO_HAVE 封面副标题缺“将”：交 Develata 决定，见 images/README.md 封面节。
