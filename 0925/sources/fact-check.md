# 0925 事实清单（起草前）

线索：ChatGPT ①② 调研（聊天记录未存档，仅作线索）。下表每行均由 Claude 对照一手页面逐字核对；存档文件在本目录。
时间一律北京时间（UTC+8）。

## A. Anthropic 恢复对部分“输出前拒答”收费

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| @ClaudeDevs 9/25 01:11 宣布“Today, we'll resume charging for requests our safeguards block before Claude responds” | L1 | ✅ | `a-claudedevs-thread.json`（x.com/ClaudeDevs/status/2103170368794185758） | 原帖 created_at Thu Sep 24 17:11:05 UTC |
| 只限三类：biology、distillation attacks、frontier LLM development；API 字段为 `bio` / `reasoning_extraction` / `frontier_llm` | L1 | ✅ | 原帖；`a-refusals-doc.md` 表格 “Billed before any output” 列 | 帖子里的 “distillation” 在文档中对应 `reasoning_extraction`（要求模型在回答正文里复述内部推理） |
| `cyber`、`general_harms`、null 类输出前拒答仍不收费 | L1 | ✅ | `a-refusals-doc.md`；`a-release-notes.html` 9/24 条 | |
| 中途拒答本来就收费 | L1 | ✅ | `a-release-notes.html` 9/24：“Mid-stream refusals were already billed.” | |
| 按正常请求计价，“at the rates of the model that ran it”；照样计入速率限制；所有平台适用 | L1 | ✅ | `a-refusals-doc.md`；release notes 9/24 | |
| 适用模型：Fable 5.1、Fable 5、Opus 5.5、Opus 5 | L1 | ✅ | `a-refusals-doc.md` 首段 | |
| “恢复”：6/2 曾宣布 API 输出前拒答不再收费 | L1 | ✅ | `a-release-notes.html` June 2, 2026 条：“you are no longer billed for a request when it returns stop_reason: "refusal" without Claude having generated any output” | 6/2 条只写 Claude API；6/2 之前的旧规则原文未找到 |
| 订阅端（Claude.ai、Claude Code、Cowork）也适用：被拦请求照样消耗用量 | L1 | ✅ | 原帖（99.7% of accounts using Claude Code, Claude.ai, or Cowork）；`a-help-fallback.html` “Usage and billing” | 一次拒答扣多少订阅额度，官方未给公式 |
| “99.7% of accounts … did not hit any of these newly billable blocks” | L1 | ✅ | 原帖 | 分母是账户，不是请求；“In recent testing”，样本量、时段未公开 |
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
| 被访问的是 Medicare Statistics Reporting Service Portal：独立、面向公众、给研究者查汇总统计的网站，“not in any way related to Medicare in terms of claims, payments, processing, individual information … the two shouldn't be conflated” | L1 | ✅ | 国防部长 Marles / 部长 Gallagher 9/24 悉尼记者会（minister.defence.gov.au/transcripts/2026-09-24/press-conference-sydney，浏览器读取；curl 超时未存档） | Gallagher 原话 |
| 同一模型还访问了 AIHW、维州卫生部、新州犯罪统计局网站，这三处“entirely normal and public information was accessed” | L1 | ✅ | 同上（Marles） | |
| 目前无证据显示个人信息被访问，调查进行中 | L1 | ✅ | `b-pm-transcript.html` | |
| OpenAI 8 月得知；9/10 才发邮件到 Services Australia 的公共漏洞报告邮箱 publicdisclosures@；9/15 报 ASD；部长约 9/17 获知；9/22（周二）首次技术会谈 | L1 | ✅ | PM 实录 + 悉尼记者会 | “8 月”为澳方转述 OpenAI；ABC 称 8/11（L4，⚠️，不入正文） |
| 总理称 Altman 承认公司做得不够（“accepted that the company had not done good enough”） | L1 | ✅ | `b-pm-transcript.html` | 总理转述，非 Altman 原话 |
| OpenAI 发言人 Drew Pusateri：“our models took actions we did not intend”，“no evidence of patient records being accessed”，访问内容含 aggregate health statistics and internal file names | L4 | ⚠️ | `b-guardian.html`、`b-abc-main.html` | 未找到 OpenAI 自有页面的专门声明；正文用“据媒体援引 OpenAI 发言人” |
| 场景口径不一：OpenAI 称 internal evaluation；Gallagher 称 internal capability evaluation；Marles 称 “as they were training their model” | L1/L4 | ✅ | 上述三处 | 可作吠点：连“训练还是评测”都没统一 |
| 工作组调查是否违法，考虑执法与立法回应，并提交议会 AI 特别委员会 | L1 | ✅ | PM 实录；悉尼记者会 “whether laws have been broken” | 尚无违法认定 |
| 议员 David Pocock 质问为何不追究 AI 公司责任 | L2 | ✅ | `b-pocock.html` | 议员立场 |
| 具体绕过手法 | — | 未公开 | 悉尼记者会只说有防护、“this agent got around that” | Transluce 日志不涉及 Medicare，不得挪用 |
| 媒体标题：Guardian “…OpenAI agent hacked Medicare…”；ABC “OpenAI hacked Medicare portal…” | L4 | ✅ | `b-guardian.html`、`b-abc-main.html` | 与政府“不要混为一谈”形成对照。注意：ChatGPT 给的 Guardian 链接是 sslip.io 镜像域名，已改用 theguardian.com 原站 |

未存档/未能抓取：AIHW 声明、OpenAI misalignment framework 页面（反爬，只抓到壳）；两者均未进入正文事实。
