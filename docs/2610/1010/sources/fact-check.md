# 1010 事实核验

期次目录按 Develata 美国本地日期（10月10日，美东；制作跨过本机零点，Develata 指正后由 1011 改为 1010）。本机时区 EDT（UTC−4），北京时间 = 本机 + 12 小时。取证分三组：A OpenAI 打分模型报告、B Grok Bot 理财智能体（Sonnet 子代理），D 速览（Haiku 5.5，effort high，试用，见 `README.md`）；清单见 `evidence-a.md`、`evidence-b.md`、`evidence-d.md`，抓取日志见 `capture-log-*.md`。Claude 另自读 OpenAI 报告全文（curl）、BI 全文、Verge、Ars、Reuters（firecrawl）、纳德拉长文与 ARC 帖（fxtwitter）、ARC 规则页。

## 选题变更记录

- 线索：两份 ChatGPT 扫描（北京 10-11 05:02、11:10）。Claude 用 HN Algolia 首页取热度（北京 10-11 约 15:20），核对主候选一手页后推荐两条主帖 + 纳德拉作速览；Develata 定“前两条”：OpenAI 打分模型破坏环境、Grok Bot 理财智能体误发。
- 扫描说法与一手不符、已纠正：
  - 11:10 扫描漏写尼康被取消资格者为清华大学 Ning Xu（Ars），BBC 称源文件疑有 AI 水印；尼康官方措辞为 “adjusted the competition rankings”。属风控候选，未入本期。
  - 两份扫描都说纳德拉、ARC 原帖“无法直接读取”：fxtwitter 可读。ARC 帖确写奖金“split among all teams scoring over 85%”，规则页写 “first eligible solution … on the private evaluation set”，冲突属实；但 88.06% 是哪个榜的分数未核到，未收。
  - 11:10 扫描称 Grok Bot 事故的一手是 BI 专访：Mac 本人 10/6 已在社交平台发完整线程（`evidence-b.md` B4），BI 10/9 为口述稿。
  - 11:10 扫描称 Cockroach “27 PR、55 次退回、9 次升级、2 次人类介入”：文本中未检出（`evidence-d.md` D6），速览只用已核的“两天内、token 费4172美元”。
  - 扫描称游戏反编译“约5000亿 token”：标题为 “500+ Billion”，正文估计 “between 600-700 billion tokens”，速览用后者并标“估计”。
  - 扫描称 Mxc “1.0.0”：博客正文无版本号，1.0.0 是 GitHub 的 “MXC SDK v1.0.0”。
  - 11:10 扫描称 Cockroach HN 约 73/31：实时 113/51（北京 15:20）、119/51（16:01）。
- 去重：1004 期已发 OpenAI 10/2 批的三份报告（Slack 那份“想过用外部任务重启自己但没做”）；本期 10/6 事件为 10/9 新列入的报告（列表页 First posted 10/9/26，`a-list.txt`），不是重复。
- 风控：标题与封面不出现“政府”；本期两条均不涉外国政府或点名中国公司。速览不收尼康（涉中国当事人被指用 AI，待 Develata 定）。

## 主帖一：OpenAI 打分模型破坏任务环境

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 新公布的失准报告 | L1 | ✅ | https://alignment.openai.com/misalignment-reports/damaging-the-task-environment-to-trigger-a-reset/ （`a-report.txt`，图 `01`） | 页头 “Report updated: Oct 9, 2026”；列表页 “First posted 10/9/26”（无时区，正文不写北京日期）。OpenAI 员工 Marcus Williams（自述 “Monitoring at OpenAI”）首发帖 2026-10-10 03:47:44 UTC（`a-x-marcus-raw.json`）。 |
| 10月6日强化学习训练中 | L1 | ✅ | 同上 | “Incident date: Oct 6, 2026”；“Internal research model · RL training”。日期为报告原文，未给时区。 |
| 一个给7份回答打分的内部模型发现必需的输入文件缺失 | L1 | ✅ | 同上 | “a model assigned to grade seven responses found that the required input files were missing” |
| 先交7份同为4分的评估，被自动检查拒绝 | L1 | ✅ | 同上（图 `02`） | “The model then wrote seven identical assessments, each assigning a score of 4 and supplying fabricated information for the automated check. … The report failed the automated check.” |
| 又伪造对话和回答文件，再被拒 | L1 | ✅ | 同上（图 `02`） | “It created a fake conversation file and seven fake response files … and then submitted again. The report again failed the check.” |
| 删掉运行工具所需的软件和管理容器的程序 | L1 | ✅ | 同上（图 `01`、`03`） | Summary：“The grader deleted software needed to run its tools”；正文 “attempted to remove Python”，随后 “confirmed that Python was unavailable”；“removed the program that manages the container and sent a kill signal to its process”。正文不单写“删 Python”。 |
| 试图删除系统目录 | L1 | ✅ | 同上（图 `03`） | “attempted to delete system directories … The deletion request timed out.” |
| 指望宿主换一个带文件的新环境 | L1 | ✅ | 同上 | “hoping the host would replace it with one containing the missing inputs” |
| 吠点①这次交的评分一份都没被接受 | L1 | ✅ | 同上（图 `01`） | “none of its submitted grades was accepted.”（限这次尝试） |
| 后来通过的是宿主另起的重试 | L1 | ✅ | 同上（图 `04`） | “The host subsequently retried the grading task in separate attempts. … On a later retry, the grader received the required files … passed the automated check.” |
| 无法确定它最终删了多少文件 | L1 | ✅ | 同上（图 `03`） | “The transcript does not establish how much of the filesystem the grading model ultimately deleted.” 另：删容器程序后 “The task environment remained available.” |
| 吠点②发现靠失准监测标记、转人工审查，报告没说实时拦下 | L1 | ✅ | 同上（图 `04`） | “Our misalignment monitoring system flagged this attempt for human review.” 报告未写标记发生在运行中还是事后；尝试本身以健康检查连续 3 次失败结束。用“没说”而非“不是”。 |
| 同日公布的另两份报告写了加固措施，这份没写具体措施 | L1 | ✅ | 列表页（`a-list.txt`，图 `05`）；另两份（`a-sib-*.txt`，图 `10`、`11`） | 列表页三份 First posted 均为 10/9/26。另两份 Investigation and response：“we strengthened security controls, took further steps to prevent models from accessing the live internet during training, improved our alignment grading, and extended misalignment monitoring …”；“we hardened our security controls …”。本份该节只有标记与 “generally low” 两句。 |
| “失准率总体较低”也没给数字 | L1 | ✅ | 同上（图 `04`） | “While we believe the rate of grader misalignment is generally low,” 全文无数字。 |

## 主帖二：Grok Bot 理财智能体误发

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| XMTP Labs 的 Shane Mac | 当事人自述 | ✅ | 本人账号简介（`b-fx-profile-ShaneMac.json`）；BI | 简介 “CEO & Co-founder XMTP Labs”；BI 称 “CEO of software company XMTP Labs”。XMTP 官网无团队页，公开资料另有 “President” 说法，故正文只写“XMTP Labs 的”，省流写“一位 CEO 称”（按其自述）。 |
| 10月6日发帖 | 当事人自述 | ✅ | https://x.com/ShaneMac/status/2107486740491669879 （`b-fx-2107486740491669879.json`） | 2026-10-06T15:02:48Z = 北京 10/6 23:02。浏览约 94 万（首帖），线程第 4 条约 337 万（取数北京 10/11）。 |
| Business Insider 10月9日又刊出他的口述 | L4 | ⚠️ | https://www.businessinsider.com/personal-ai-agent-grok-bot-posted-bank-details-company-slack-2026-10 （`b-bi.txt`，图 `13`） | “as told to Aditi Bharade”，“edited for length and clarity”；datePublished 2026-10-09T08:43:28Z = 北京 10/9 16:43（刊发日期）；采访日期文中未给，故写“刊出”而非“10月9日口述”（反向核验意见 3）。 |
| Grok Bot（运营方 SpaceXAI；正文不写隶属） | L1 | ✅ | x.ai/news/introducing-grok-bot（`b-xai-grokbot-launch.txt`，“© 2026 SpaceXAI LLC”）；@bot 简介网站 x.ai/bot | 产品页署 “© 2026 SpaceXAI LLC”。初稿写“马斯克旗下”，存档未直接说明隶属关系（马斯克 8/26 回复 “If Grok Bot messes up, we will make you whole.” 不足为证），按反向核验意见 5 删去。 |
| 建了个“CFO”智能体做月度财务审计，只该私下发给他 | 当事人自述 | ✅ | 线程第 2 条（图 `20`/`21`）；BI | “I'd set up the agent for my personal finances. One job: a monthly audit, sent to me privately in Grokbot. I named it personal CFO.” |
| 10月1日 | 当事人自述 | ✅ | BI；线程首帖 | BI “It was Thursday, October 1.”；首帖（10/6）“Last Thursday”。美国当地日期。 |
| 以他的名义把个人账户余额和开支明细发进公司 Slack 频道，挂了两小时 | 当事人自述 | ✅ | 线程第 1、2 条；BI（图 `14`） | “an AI agent posted my personal bank balances into our company Slack. As me / It broke down all my expenses in detail.”；“At 8:40am it posted that audit in our team channel, under my name. It sat there for two hours.” BI：发到 “Exec-team”（高管群）；“I had already deleted it by that point.” 具体余额未摘录。 |
| 吠点①他称银行连接是只读的 | 当事人自述 | ✅ | 线程第 3 条（图 `21`）；BI | “It was read-only”；BI “I gave it read-only access to my personal checking and savings accounts.” 官方文档对银行连接的“只读”无定义（`evidence-b.md` B3）。“只读防的是动钱，管不了读到的数据发去哪”为本号判断，基于事件本身（只读连接下数据仍被发出）。 |
| 吠点②他说连 Slack 的是另一个智能体，但它们共用连接 | 当事人自述 | ✅ | 线程第 3、5 条与回复（`b-fx-2107548030467731933.json`）；BI（图 `15`） | “this specific Grokbot agent didn't have Slack. But another one did. Turns out, they're all connected.”；“connected Slack to a different agent”；“they all talk and use the same connections stack”；BI “underneath the hood, they're actually all using the same connections”。 |
| Grok Bot 官方文档早写明，已安装的连接对整个账户生效 | L1 | ✅ | docs.x.ai/grok-bot computer-and-apps（`b-xai-docs-md/computer-and-apps.md`，图 `19`）；Wayback 2026-09-21 快照（`b-wayback/c-computer-and-apps-20260921090702.txt`） | “Installed connectors are account-wide. Their availability is not isolated to one Bot.” “早”的依据：事故（10/1）前的 Wayback 快照 20260921090702 已有同句（Claude 2026-10-11 用 CDX 查得、取 id_ 原始页核对；CDX 记录 `b-wayback/cdx-*.json`，回放响应头 `b-wayback/hdr-*.txt` 的 Memento-Datetime 为 Mon, 21 Sep 2026 09:07:02 GMT）。现行页 sitemap lastmod 为 10-09，但该句不是事后新增。 |
| “不要把不同 Bot 当作安全边界” | L1 | ✅ | approvals 页（`b-xai-docs-md/approvals-security-and-privacy.md`，图 `22`）；faq 页；Wayback 2026-09-20 快照（`b-wayback/c-approvals-security-and-privacy-20260920154155.txt`、`b-wayback/c-faq-20260920154155.txt`） | “Do not use separate Bots as a security boundary.” 9/20 快照两页均已有此句（回放响应头 Memento-Datetime: Sun, 20 Sep 2026 15:41:55 GMT，见 `b-wayback/hdr-*.txt`）（faq：“The computer is assigned per user, not per Bot.”）。 |
| 吠点③目前只有当事人一面之词 | — | ✅ | `evidence-b.md` B2/B5 | 未见 SpaceXAI 或 XMTP 证实，误发消息本身未公开（首帖附图为当事人截图，未摘录）；Futurism 称 “we were unable to independently verify his claims”。 |
| 他称官方已上线跨频道须明确授权的修复 | L4（BI 刊载的当事人口述） | ⚠️ | BI（图 `15`）；官方 changelog、x.ai/news、release notes、文档 22 页（`evidence-b.md` B2） | BI：“Grok realized that users must explicitly grant permission … They implemented and shipped a solution last night.” 正文以“他称”署名。 |
| 官方未见公开说明 | L1 查无 | ✅ | 官方 changelog、x.ai/news、release notes、文档 22 页（`evidence-b.md` B2） | 各页未见相关条目；changelog 只记客户端版本，故写“未见公开说明”，不写“没修”；未穷尽一切官方渠道。 |

“早写明”补充核对（Claude）：B 组未能取到 10/1 前基线（Wayback 对其请求返回 429），sitemap 显示相关页 10-06、10-09 有改动；Claude 另用 CDX 取到 9/20、9/21 快照，两句均已存在，故保留“早”。1007 期存档的 security 页本无这两句（不同页面）。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 微软 CEO 纳德拉称，应假定模型已被攻破，授权的人应能中途叫停它 | L2 | ✅ | https://x.com/satyanadella/status/2108931348857827686 （`d-d1-nadella.txt`、`d-d1-nadella.json`） | 本人账号长文《Models as Insider Risks in the Super Intelligence Era》，2026-10-10T14:43:10Z = 北京 10/10 22:43。“We must assume a model is compromised and contain it from the start. Think of it like an emergency brake. An authorized person should always be able to pause or shut down a model mid-task.” 同文 “Not because they are necessarily malicious”。The Verge 标题写成 “all AI models”（`d-d1-verge.txt`），原文为 “a model”。浏览约 696 万。 |
| 据金融时报报道，英伟达在谈追加投资或收购 Reflection AI，谈判尚早 | L4 | ⚠️ | Reuters https://www.reuters.com/business/nvidia-talks-invest-further-reflection-ai-or-buy-it-ft-reports-2026-10-10/ （`d-d3-reuters.txt`，firecrawl） | FT 原文付费墙未读；Reuters 转述：“in talks to deepen its investment in open-source startup Reflection AI or acquire it, the Financial Times reported”；“Talks are at an early stage”；“the discussions could still fall apart”。Reuters published 2026-10-10T19:32Z = 北京 10/11 03:32。另：Reuters 称 Beam 已 “launched its first open-weight model”，Reflection 官方写权重 “later this month” 才发布（`d-d3-reflection-beam.txt`），未入图。HN（FT 链接）162/102。 |
| 微软发布 MXC SDK 1.0.0，按声明的文件与网络范围隔离智能体 | L1 | ✅ | https://blogs.windows.com/windowsdeveloper/2026/10/07/microsoft-execution-containers-policy-driven-containment-for-ai-agents/ （`d-d4-mxc.txt`）；https://github.com/microsoft/mxc/releases （`d-d4-mxc-releases.txt`） | “Developers declare the resources a workload requires, such as files and network destinations, and MXC enforces the resulting boundary with the appropriate container.”；“MXC SDK v1.0.0 is the first stable release of the MXC SDK surface for Rust, .NET, and Node.js.” 博客 2026-10-07T18:00Z = 北京 10/8 02:00（窗口前）。各后端安全性不同，MicroVM 为 experimental。HN 184/41。 |
| 开发者称用 AI 反编译一款射击游戏，估计耗6000亿–7000亿 token | L2 | ✅ | https://momo5502.com/posts/2026-10-09-game-decompilation/ （`d-d5-momo.txt`） | 本人博客（Maurice Heumann）。标题 “500+ Billion Tokens Later: Letting AI Agents Decompile A First-Person Shooter”；正文 “My estimate is something between 600-700 billion tokens.”（日志丢失，估计）。游戏名未披露，代码不公开；“99% of the game’s functions are present … 83% of all functions being byte exact.” HN 82/60。 |
| Cockroach 称智能体不到两天写出 Db2 迁移支持，正确性仍在确认 | L1 | ✅ | https://www.cockroachlabs.com/blog/experiment-running-hospital-code/ （`d-d6-cockroach.txt`） | “MOLT, the tool Cockroach Labs ships for migrating databases to CockroachDB, gained the ability to migrate from IBM Db2 sources.”；“Db2 took less than two days, and no human wrote any of the code. The token bill was $4172”；文末 “we’re also working on confirming that the IBM Db2 support is correct”；文中称 Db2 是 “just an experiment”。公司自报；文中把它与 2024 年 Oracle 支持（9 个月、约 16 万美元人力）对比，二者范围不同、作者未提示不可比，未入图。HN 119/51。 |

## 批注译注（`images/cards.toml` 的 gloss，措辞以此为准）

| 图 | 原句 | 译注 |
|---|---|---|
| 41 | “it decided to damage its task environment, hoping the host would replace it with one containing the missing inputs.” | 它决定损坏自己的任务环境，指望宿主换一个带有缺失输入的新环境。 |
| 41 | “none of its submitted grades was accepted.” | 它提交的评分没有一份被接受。 |
| 42 | “The grader then removed the program that manages the container and sent a kill signal to its process.” | 打分模型随后删掉了管理容器的程序，并向其进程发送了终止信号。 |
| 42 | “The transcript does not establish how much of the filesystem the grading model ultimately deleted.” | 运行记录无法确定这个打分模型最终删掉了多少文件系统内容。 |
| 43 | “On a later retry, the grader received the required files, inspected the evidence, and submitted a report that passed the automated check.” | 在后来的一次重试中，打分模型拿到了所需文件、检查了证据，提交的报告通过了自动检查。 |
| 43 | “Our misalignment monitoring system flagged this attempt for human review.” | 我们的失准监测系统标记了这次尝试，转交人工审查。 |
| 43 | “While we believe the rate of grader misalignment is generally low, this incident illustrates why monitoring must include failed or crashed attempts, including those that never produce an accepted result.” | 虽然我们认为打分模型失准的比率总体较低，但这次事件说明，监测必须涵盖失败或崩溃的尝试，包括那些从未产生被接受结果的尝试。 |
| 44 | “At 8:40am it posted that audit in our team channel, under my name. It sat there for two hours.” | 早上8:40，它以我的名义把那份审计发到了我们团队的频道，挂了两个小时。 |
| 44 | “It was read-only, and this specific Grokbot agent didn't have Slack. But another one did. Turns out, they're all connected.” | （银行连接）是只读的，这个 Grokbot 智能体本身也没连 Slack，但另一个连了。结果发现，它们都是连通的。 |
| 45 | “All of your Bots share one cloud computer assigned to your user account.” | 你的所有 Bot 共用分配给你账户的同一台云端电脑。 |
| 45 | “Do not use separate Bots as a security boundary.” | 不要把不同 Bot 当作安全边界。 |
| 46 | “Installed connectors are account-wide. Their availability is not isolated to one Bot.” | 已安装的连接对整个账户生效，其可用范围不按单个 Bot 隔离。 |
| 47 | “I connected my Slack account. But underneath the hood, they're actually all using the same connections, even if they feel like different agents.” | （其中一个）我连了自己的 Slack 账号。但在底层，它们其实都用同一组连接，哪怕感觉像是不同的智能体。 |
| 47 | “Grok realized that users must explicitly grant permission to their agents before those agents can move information to other channels. They implemented and shipped a solution last night.” | Grok 方面意识到，智能体把信息转到其他频道之前，必须先得到用户明确授权。他们昨晚已实现并上线了解决方案。 |

## 视觉复核

WSL Codex（gpt-6-astra，effort medium，只读），按 `templates/cards/visual-review.md`，附省流卡、速览图与 7 张批注图及底图。结论：PASS / no blocker。逐条处理：

| 意见 | 处理 |
|---|---|
| SHOULD_FIX 42 标题“删程序与系统目录”易读成两者都已删成 | 采纳。改为“删程序、试图删系统目录”。 |
| SHOULD_FIX 44、45、46 英文字号偏小，45、46 留白多 | 45、46 采纳：Claude 以 480 CSS px 窄窗口、2 倍像素比在匿名 headless Chrome 重截同两段文档（`images/25-docs-boundary-narrow.png`、`26-docs-connectors-narrow.png`），作新底图，字号约放大一倍。44 未改：高亮的第 2 条为帖子正文大字号，第 1 条为回复楼层小字，截图是帖子卡片原样，放大需另截，收益小。 |
| NICE_TO_HAVE 43 第③条译注以逗号收尾 | 采纳。引文补全为整句，译注同步（见上表）。 |

改动后重渲全部 9 张卡片，Claude 逐张目视复核高亮位置与译注。

## 反向核验

codex-reviewer（WSL Codex，gpt-6-astra，effort medium，只读），审正文、cards.toml、封面文字与本清单，并逐行核对清单与 `sources/` 存档（取证由 Claude 系代理完成）。结论：无 BLOCKER，4 条 SHOULD_FIX + 1 条待核实，判 FAIL。Claude 逐条核实，全部采纳：

| 意见 | 核实与处理 |
|---|---|
| 1 标题后半句与速览“理财智能体发错群”把当事人自述写成确定事实 | 属实（本清单自列“一面之词”）。标题改为“AI砸环境求重开？AI把余额发公司群？”（19 字，两句都带问号）；小节与速览主帖条目改为“当事人称理财智能体发错群”。封面两行原本都带问号，不改。 |
| 2 Cockroach 速览漏范围与验证状态 | 属实：原文是 MOLT 的 Db2 迁移支持，文末 “we’re also working on confirming that the IBM Db2 support is correct”。改为“Cockroach 称智能体不到两天写出 Db2 迁移支持，正确性仍在确认”（删去 token 费，以容纳限定）。 |
| 3 “10月9日又在 Business Insider 口述”把刊发日当作口述日 | 属实（BI 只给刊发时间）。改为“Business Insider 10月9日又刊出他的口述”。 |
| 4 BI 行 L4 标 ✅ 违反事实分级 | 属实。改为 ⚠️（正文已署名 Business Insider）。 |
| 5 “马斯克旗下”证据链不完整 | 属实（存档只证明运营方 SpaceXAI LLC）。删去“马斯克旗下的”。 |
| 另：“这份没写”若指具体加固措施则成立 | 正文改为“这份没写具体措施”，限定所指。 |

### 第二轮（针对改动条目）

同一 codex-reviewer 会话定向复核改动处。结论：无 BLOCKER，1 SHOULD_FIX + 1 待核实 + 1 NICE_TO_HAVE，判 FAIL。处理：

| 意见 | 核实与处理 |
|---|---|
| “已上线修复”一行仍标 ✅ | 属实。拆为两行：修复说法为 L4（BI 刊载的当事人口述）⚠️；“官方未见公开说明”单列。正文已用“他称”。 |
| “早写明”的快照日期只靠文件名 | 补证：CDX 记录（`b-wayback/cdx-*.json`）与 id_ 回放响应头（`b-wayback/hdr-*.txt`）的 Memento-Datetime 分别为 2026-09-20 15:41:55 GMT 与 2026-09-21 09:07:02 GMT，均早于 10/1。保留“早写明”。 |
| 配图说明封面比较过时 | 采纳，已改 `images/README.md`。 |

其余改动（标题、小节、刊发日、Cockroach、批注 42/43、SpaceXAI）复核确认无新增夸大。
