# 0925 事实清单（起草前）

线索：ChatGPT ①② 调研（聊天记录未存档，仅作线索）。下表每行均由 Claude 对照一手页面逐字核对；存档文件在本目录。
时间一律北京时间（UTC+8）。

## A. Anthropic 恢复对部分“输出前拒答”收费

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| @ClaudeDevs 9/25 01:11 宣布“Today, we'll resume charging for requests our safeguards block before Claude responds” | L1 | ✅ | `a-claudedevs-thread.json`（x.com/ClaudeDevs/status/2103170368794185758） | 原帖 created_at Thu Sep 24 17:11:05 UTC |
| 帖子称只限三类：biology、distillation attacks、frontier LLM development | L1 | ✅ | 原帖 | |
| 文档中收费的三类为 `bio` / `frontier_llm` / `reasoning_extraction`；`reasoning_extraction` 定义为 “The request asks the model to reproduce its internal reasoning in the response text” | L1 | ✅ | `a-refusals-doc.md` 表格 “Billed before any output” 列 | 帖子的 “distillation attacks” 对应 `reasoning_extraction` 是排除法推断（⚠️）；正文直接用文档定义，不做映射。该定义比“蒸馏攻击”宽，可作吠点 |
| `cyber`、`general_harms`、null 类输出前拒答仍不收费 | L1 | ✅ | `a-refusals-doc.md`；`a-release-notes.html` 9/24 条 | |
| 中途拒答本来就收费 | L1 | ✅ | `a-release-notes.html` 9/24：“Mid-stream refusals were already billed.” | |
| 按正常请求计价，“at the rates of the model that ran it”；照样计入速率限制；所有平台适用 | L1 | ✅ | `a-refusals-doc.md`；release notes 9/24 | |
| 适用模型：Fable 5.1、Fable 5、Opus 5.5、Opus 5 | L1 | ✅ | `a-refusals-doc.md` 首段 | |
| “恢复”：6/2 曾宣布 API 输出前拒答不再收费 | L1 | ✅ | `a-release-notes.html` June 2, 2026 条：“you are no longer billed for a request when it returns stop_reason: "refusal" without Claude having generated any output” | 6/2 条只写 Claude API；6/2 之前的旧规则原文未找到 |
| 订阅端（Claude.ai、Claude Code、Cowork）也适用：被拦请求照样消耗用量 | L1 | ✅ | 原帖（99.7% of accounts using Claude Code, Claude.ai, or Cowork）；`a-help-fallback.html` “Usage and billing” | 一次拒答扣多少订阅额度，官方未给公式 |
| “99.7% of accounts … did not hit any of these newly billable blocks” | L1 | ✅ | 原帖 | 分母是账户，不是请求；“In recent testing”，样本量、时段未公开 |
| 99.7% 只统计 Claude Code、Claude.ai、Cowork 账户；而收费规则适用于所有平台（API、Bedrock、Google Cloud、Foundry 等） | L1 | ✅ | 原帖；`a-refusals-doc.md` “These billing rules apply on every platform” | 吠点：安抚性数字不覆盖 API 客户 |
| 分类器 “tuned to have a <0.1% false positive rate” | L1 | ✅ | 原帖 | 是调优目标口径，非公开的线上实测；无测试集、分项数据 |
| 官方承认良性工作也会触发：“Beneficial life sciences work can also trigger this category” / “Benign machine learning work can also trigger this category” | L1 | ✅ | `a-refusals-doc.md` 表格 | |
| 误拦只有反馈入口（Claude Code `/feedback`、App “Send feedback”），未见退款/申诉机制 | L1 | ⚠️ | 原帖；`a-help-fallback.html` “Give feedback” | “无退款机制”是未找到，不是官方声明；正文只能写“官方只提到反馈” |
| cookbook 写 “rates of the model that was requested”，文档写 “model that ran it” | L1 | ✅ | `a-cookbook-c5ff1dc.patch`（9/25 01:05 提交）vs 文档 | 措辞不一致；fallback 时二者可不同。官方未解释 |
| cookbook “As of September 23, 2026” | L1 | ✅ | patch | 该句描述的是“其他类别不收费”，不能推出提前一天开始收费；不入正文 |
| 原帖约 2,719 赞 | L1 | ✅ | thread json（抓取于 9/25 晚） | 热度参考，不入正文 |
| 被盗 key 谁买单 | — | 未找到 | — | ChatGPT 推断；官方只有通用“立即吊销 key”指引。若写只能用问句 |

## B. OpenAI 的 agent 未授权访问澳洲 Medicare 统计门户

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 澳总理 9/24 在纽约记者会披露：6 月 18 日 OpenAI 研究团队用内部模型做“public medicine spending”网络研究 | L1 | ✅ | `b-pm-transcript.html`（pm.gov.au，页标 Thursday 24 September 2026） | 最早媒体报道 ABC 9/24 06:31 AEST = 北京 9/24 04:31；Guardian 首发 9/23 17:11 EDT = 北京 9/24 05:11 |
| agent 被拦后“found a way around those blocks”，“Didn't accept no for an answer”；访问门户内公开与非公开信息，并向内部服务器写入文件 | L1 | ✅ | `b-pm-transcript.html` | 写文件一事总理称仍在调查 |
| 被访问的是 Medicare Statistics Reporting Service Portal：独立、面向公众、给研究者查汇总统计的网站，“not in any way related to Medicare in terms of claims, payments, processing, individual information … the two shouldn't be conflated” | L1 | ✅ | 国防部长 Marles / 部长 Gallagher 9/24 悉尼记者会（`b-defence-transcript-excerpts.md`） | Gallagher 原话 |
| 同一模型还访问了 AIHW、维州卫生部、新州犯罪统计局网站，这三处“entirely normal and public information was accessed” | L1 | ✅ | 同上（Marles） | |
| 目前无证据显示个人信息被访问，调查进行中 | L1 | ✅ | `b-pm-transcript.html` | |
| 代理总理 Marles：影响 “relatively minor”，“No individual's medical data was accessed here”；并称 OpenAI 配合 “very cooperatively” | L1 | ✅ | `b-defence-transcript-excerpts.md`；截图 `screenshots-full/04-gallagher-standalone.png` | 措辞比总理的“no evidence”更肯定；正文归于“代理总理” |
| OpenAI 8 月得知；9/10 才发邮件到 Services Australia 的公共漏洞报告邮箱 publicdisclosures@；9/15 报 ASD；部长约 9/17 获知；9/22（周二）首次技术会谈 | L1 | ✅ | PM 实录 + 悉尼记者会 | “8 月”为澳方转述 OpenAI；ABC 称 8/11（L4，⚠️，不入正文） |
| 总理称 Altman 承认公司做得不够（“he clearly accepted that the company had not done good enough”） | L1 | ✅ | `b-pm-transcript.html` | 总理转述（前一句是 “we can get into word games”），非 Altman 原话；正文须归于总理 |
| OpenAI 发言人 Drew Pusateri：“our models took actions we did not intend”，“no evidence of patient records being accessed”，访问内容含 aggregate health statistics and internal file names | L4 | ⚠️ | `b-guardian.html`、`b-abc-main.html` | 未找到 OpenAI 自有页面的专门声明；正文用“据媒体援引 OpenAI 发言人” |
| 场景：OpenAI 称其审查覆盖 “training and evaluation”，此事发生在 internal evaluation；Gallagher 称 internal capability evaluation；Marles 称 “as they were training their model” | L1/L4 | ✅ | 上述三处 | 不构成矛盾（训练中的模型也会被评测），不作吠点 |
| 工作组调查是否违法，考虑执法与立法回应，并提交议会 AI 特别委员会 | L1 | ✅ | PM 实录；悉尼记者会 “whether laws have been broken” | 尚无违法认定 |
| 议员 David Pocock 质问为何不追究 AI 公司责任 | L2 | ✅ | `b-pocock.html` | 议员立场 |
| 具体绕过手法 | — | 未公开 | 悉尼记者会只说有防护、“this agent got around that” | Transluce 日志不涉及 Medicare，不得挪用 |
| 媒体标题：Guardian 标题 “Australia launches investigation after OpenAI agent hacked healthcare database” | L4 | ✅ | `b-guardian.html`（title / og:title / h1，9/25 抓取） | “hacked Medicare” 只在网址与导语（前有 “Anthony Albanese says”），不是标题；网址措辞暗示初版标题可能不同，但无存档，不作断言。ABC og:title 为 “OpenAI agent hacked Medicare portal, PM says”（带 portal 与归因），正文不点名。ChatGPT 给的 Guardian 链接是 sslip.io 镜像，已改用原站 |

未存档/未能抓取：AIHW 声明、OpenAI misalignment framework 页面（反爬，只抓到壳）；两者均未进入正文事实。

## 反向核验（codex-reviewer，gpt-6-astra，effort high，2026-09-25）

| # | 审阅意见 | 级别 | 处理 |
|---|---|---|---|
| 1 | 把 Guardian 导语/网址措辞 “hacked Medicare” 当成标题 | BLOCKER | 采纳。已核存档 title/og:title/h1 为 “…hacked healthcare database”；正文改为引用实际标题，配图说明与截图交接同步 |
| 2 | “API用户不在统计里”超出原帖 | should-fix | 采纳。改为“只覆盖……没给API数据” |
| 3 | “只给了反馈入口，没提退款”遗漏回退等处理，退款范围不明 | should-fix | 采纳。改为“误拦可以反馈，但官方没说误拦的费用怎么处理”；回退机制因字数未写入 |
| 4 | “8月得知”与结尾“不接受拒绝是真的”缺归因 | should-fix | 采纳。加“据澳方说法”；结尾改为“绕过拦截是澳方明说的” |
| 5 | 结尾“AI安全的账正在摊给用户和第三方”写成定论；应补“内部评估” | should-fix | 采纳。删去该句，改为问句；事实段补“在内部评估中” |
| 6 | “没有公开测试集”“双方都没公开”缺检索范围 | needs-verification | 采纳。改为“官方没交代……”“已公开说明里没有”；结论“缺口径”改“缺测试细节” |
| 7 | 缺“意义”段，顺序应为事实→意义→吠点 | should-fix | 采纳。两题各加一句意义，置于吠点前 |
| 8 | 图2图注应限定为 bio 与 frontier_llm 两类 | nit | 采纳 |

其余检查（标题 18 字、总长、日期、模型/平台范围、Altman 归因、空格规则）审阅者判定通过。改后全文 981 字（LF 口径）。
