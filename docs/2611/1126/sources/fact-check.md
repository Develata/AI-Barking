# 1126 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间（UTC+8）。取证：A、B、C 三组均为 Antigravity（Gemini 模型家族）取证子代理执行。主执行方抽查（11-27 21:24–23:55）：核对 C 组凤凰网科技/CNA/Cogentic README/Kimi/Reddit/3DCodeFactory 原文、B 组 `/mcp/v1`（30 工具）及 `/mcp/v1self`（25 工具）JSON、`gwscli gmail --help` 输出（`b-gwscli-gmail-help.txt`）、Frank Chu 实测文（`b-frankchu-23-or-30.txt`）、配额定价文档、BlockBeats 截图，以及 A 组 Anthropic 发布文/Platform Refusal 文档/`@ClaudeDevs` 道歉原文（`a-claudedevs-apology.json`）/Sweeney 与 Edwin Arbus X 帖子 JSON 及全部裁切底图。

## 一、谷歌：Gmail 接口列表含发信（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：Gmail接口已列发信，默认没确认发不了 | L1 | ✅ | 见下 `/mcp/v1` 工具列表、`b-gmail-mcp-ref.txt`、`b-mcp-elicitation.txt:996–999,1050–1066` 与 `b-admin-apps-help.txt:1636–1641` | 主端点列有发信工具，外部代理走 MCP/CLI 默认须用户批准（管理员可设 `Allow` 或匹配已保存授权）；未测带鉴权实际发信 |
| 省流：Tech Stackups 称 Gmail MCP 不能发信，27日晚主端点列发信、备用无 | L4 / L1 | ⚠️ | `b-techstackups.txt:20–22`（被引用与核验的个人/媒体作者说法，“AI agent writer at Ritza”，⚠️）；`/mcp/v1` 30 工具（`b-gmailmcp-v1-tools.json`，✅）与 `/mcp/v1self` 25 工具（`b-gmailmcp-v1self-tools.json`，✅） | 含被核验作者说法且仅查到端点工具列表，未做带鉴权发信 |
| 11月26日 Tech Stackups 称 Gmail MCP 不能发信 | L4 | ⚠️ | `b-techstackups.txt:20–22` 标题“Google Will Make AI Agents Use Its Workspace MCP Servers. The Gmail One Can't Send Email” | 平台热度参考：HN `50653477`（`b-hn-50653477.json`，提交 2026-11-26T09:44:23Z = 北京 11-26 17:44:23，27 日晚采样 356 分） |
| 谷歌推 MCP 与 CLI，27日晚列9服务器 | L1 | ✅ | `b-ws-updates-blog.txt:184,234` 与 `b-ws-updates-feed.json:1`（发布 `2026-11-24T09:02:59-08:00` = 北京 11-25 01:02:59，博客 11-27 06:28:37 更新）：“making Workspace Model Context Protocol (MCP) servers and the official Workspace CLI generally available”；`b-mcp-server-tools.txt:1789`（页面标注 `Last updated 2026-11-27 UTC.`）：“nine generally available Google Workspace MCP servers” | `Last updated` 为网页 UTC 更新日期，不代表首次上线的精确北京时刻 |
| 明年2月1日起代理访 Workspace 须用其一、直连 REST 限标准档；27日晚查含发信等30工具，未测鉴权发信 | L1 | ✅ | `b-ws-updates-blog.txt:184`：“AI agents… must use Workspace MCP servers or the Workspace CLI to access Workspace data… Technical enforcement begins on February 1, 2027.”；`b-quota-usage-tiers.txt`（抽查）：“direct REST APIs… limited to Standard Tier quotas”；北京 11-27 21:14–21:19 实测 `/mcp/v1` `tools/list`（`b-gmailmcp-v1-tools.json`）与 `gwscli gmail --help`（`b-gwscli-gmail-help.txt`）各列 30 工具含 `send_message`/`reply`/`forward` | 明确注明仅查列表、未做带鉴权实际发信测试 |
| 对你：网页手机收发不变 | L1 | ✅ | `b-ai-tools-addendum.txt`（截图 17）仅补充《Google APIs Terms of Service》，管辖经 GWS APIs/MCP 接入的工具；网页与官方 App 手动操作不经 APIs | |
| 个人 MCP 标准档每天每项目每产品免5000次（搜会话计10次）；备用写草稿无发信 | L1 | ✅ | `b-configure-mcp-servers.txt`（Quick setup for personal use）；`b-quota-pricing.txt:886–891`（抽查）：“On the Standard Tier, GA Google Workspace MCP servers require no billing account and are capped at 5,000 no-cost requests per day, per project, per product.”；`b-gmail-quotas.txt:2700,2723`：“`list_drafts` and `search_threads` each count as 10 requests”；`b-gmailmcp-v1self-tools.json`（25 工具含 `create_draft`/`update_draft`，无发信/回复/转发） | 5000 次属项目标准档对 GA MCP 的配额 |
| 吠点①：26日 Suncly 测23工具（旧文写须合作方权限）；27日早 Frank Chu 无鉴权同轮测23/30交替；备用25工具无发信 | L1 | ✅ | Suncly `b-suncly-contradictions.txt:2–9`（具名实测存档，`readOn: 2026-11-26T11:03:29Z` = 北京 19:03:29）：`tools: 23, canSend: false` 及旧配额脚注“require partner access”；Frank Chu `b-frankchu-23-or-30.txt:77,111–112,147`（具名实测存档，探测于美山 Nov 26 ~16:40 MST = 北京 11-27 约 07:40，11 秒内无鉴权 30 次返回 23 项 18 次 / 30 项 12 次）；`/mcp/v1self` 25 工具 | 具名外部无鉴权观测＋本号 `/mcp/v1self` 列表观测，未推断部署切流时间 |
| 吠点②：代理发信默认须批准（CLI 终端确认，策略许存最多90天、管理员可放行），无确认仍败；非合作方实发待鉴权实测 | L1 | ✅ | `b-mcp-elicitation.txt:996–999,1050–1066`（外挂代理走 MCP/CLI 的审批规则）与 `b-admin-apps-help.txt:1636–1641`：默认 `Require user approval`；仅管理员策略允许保存时记住选择最多 90 天，管理员亦可设 `Allow` 或匹配已有批准；`b-workspace-cli.txt:2063–2075`：`gwscli` 在终端请求确认；未声明 form elicitation 的调用报 `MissingRequiredClientCapability` | 明确保留“非合作方能否实发待鉴权实测”，撤去排他性能力结论 |
| 吠点③：动察写“阉割收费版”，MCP 标准档日免5000次；选 Scaled 明年4月起 Gmail、日历、网盘 MCP 免额0、首笔成功起计且不享组织内部提额例外 | L4 / L1 | ⚠️ | `b-media-blockbeats.txt`（北京 11-26 19:06 标题：“……推阉割收费版！” ⚠️）；标准档见 `b-quota-pricing.txt:886–891`（✅）；Scaled Tier 见 `b-quota-pricing.txt:779–795,971`（✅）与 `b-gmail-quotas.txt:2351–2358`（HTTP 非 2xx 与 `isError: true` 不计费）；`b-quota-usage-tiers.txt:803–810`（组织内部使用可提 REST 限额例外，`This exception does not apply to MCP quotas.` ✅） | 注：`b-quota-pricing.txt:929–946` 另列 Gmail REST API 按 Light（7000 万）/ Medium（1000 万）/ Premium（100 万）三桶每账单账号日免额；为防混淆已不在正文单列其中一桶数字 |

## 二、Claude：部分限能与反蒸馏（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 省流：51CTO 称研发时 Claude 降智，官方称两新模型限部分研发，斯威尼称被拦 | L4 / L1 / L2 | ⚠️ | 51CTO 标题“拿Claude给模型提能力，直接原地降智！”（`a-cn-51cto.txt:41`，⚠️）；发布文 `a-anthropic-fable-mythos.txt:497`（✅，针对 Fable-5.2 与 Mythos-5.2）；斯威尼帖 `a-x-tweets.json:142`（✅） | 明确 51CTO 模型研发场景、Anthropic 两款型号与斯威尼自述各为独立主张 |
| 11月25日 MagicPath CEO 称 Claude 破坏科研，斯威尼称集合论被当蒸馏拦 | L2 | ✅ | Pietro Schirano（`@skirano`）帖 `a-x-tweets.json:64,70`（个人简介确认 MagicPath CEO，发帖 `2026-11-24T23:28:56Z` = 北京 11-25 07:28:56）；Tim Sweeney（`@TimSweeneyEpic`）帖 `a-x-tweets.json:136,142`（个人简介确认 Epic CEO，发帖 `2026-11-25T08:53:18Z` = 北京 11-25 16:53:18，“blocked some set theory work I was doing, accusing me of distillation”） | 二人均实名公开自述 |
| 官方称 Fable-5.2/Mythos-5.2 训练限部分研发（算子架构等，约 Opus-5.5），推理不改该能力，少数主题回 Opus-5 | L1 | ✅ | 发布文 `a-anthropic-fable-mythos.txt:497`（抽查）：“during the model’s training, we took steps to limit advances in the specific research capabilities… including accelerator kernel optimization and improving large language model architectures, Fable 5.2 and Claude Mythos 5.2 perform at about the level of Opus 5.5… We do not alter these capabilities as the model is being served to users (that is, at inference time)… include transparent fallbacks to Opus 5 for a small set of topics” | 对象明确为 Fable-5.2/Mythos-5.2，推理不改指“这些研究能力”；约 Opus-5.5 为官方自述 |
| 对你：官方称未影响代理式编程与传统机器学习研究；网页与 Claude Code 回退有提示且可关切换；API 撞推理提取或竞争类未输出即拒也计费 | L1 | ✅ | 发布文 `a-anthropic-fable-mythos.txt:497`：“we ensured that the training changes did not affect related capabilities like agentic coding, hardware development, and traditional AI or machine learning research”；`a-support-switched-models.txt:52,62–66,82–100`（Fable 对话自动切换可关且有提示）；`a-platform-refusals-fallback.txt:190,192,230`（`bio`/`frontier_llm`/`reasoning_extraction` 未输出前拒绝按运行模型费率计费） | 训练未影响范围由 Anthropic 声明；另见吠点②分类器拦截 |
| 吠点①：51CTO 写“全给阉割”，动区27日称“尚未回應指控”；员工 Arbus 回 Schirano 称6月 Fable-5 降级已修，官号曾就隐性措施道歉 | L4 / L2 / L1 | ⚠️ | `a-cn-51cto.txt:45` 导语（⚠️）；`a-cn-blocktempo.txt:331`（北京 11-27 17:02）：“Anthropic 尚未回應指控”（⚠️）；`@edwinarbus` `a-x-tweets.json:87–88`（员工身份见 `name`/`description` 与官方回复语境，北京 11-26 09:03 回复 `@skirano`：Fable-5 头两天曾降低 AI 研究回答能力，是个错误并很快修好）；`@ClaudeDevs` 道歉原帖 `a-claudedevs-apology.json:4–5`（`Thu Jun 11 05:56:30 +0000 2026` = 北京 **6 月 11 日 13:56:30**，“We went with invisible safeguards… that was the wrong tradeoff”） | 明确分开 Edwin 回复 Schirano 的事件说明与 `@ClaudeDevs` 6 月针对隐性保障的公开道歉 |
| 吠点②：斯威尼26日称仅批反蒸馏、肯定 Claude；文档写正常机器学习可触竞争拦截，所查公开资料无该案认定 | L2 / L1 | ✅ | `@TimSweeneyEpic` `a-x-tweets.json:160`（北京 11-26 02:44:39，“The anti-distillation feature is garbage. Claude’s leading edge models are quite excellent overall.”）；`a-platform-refusals-fallback.txt:190`（`frontier_llm`：“Benign machine learning work can also trigger this category.”）；本次所查公开存档未见 Anthropic 对斯威尼个案的认定 | 严格限定于本次所查公开资料范围 |
| 吠点③：API 自动回退须配置，推理提取无推荐回退、未输出即拒也计费；Artificial Analysis 榜标默认回退，所查方法页未写回退计分 | L1 / L3 | ✅ | `a-platform-refusals-fallback.txt:224,230,244,258–262`（服务端默认回退须设 `fallbacks: "default"` 与 beta header，另有 `:308` 显式回退列表；`reasoning_extraction`：“This category has no recommended fallback model”且 `Billed before any output: Yes`）；`a-aa-models.txt:3354`（`Claude Fable 5.2 (Max, Default Fallback)` 65 分）；`a-aa-methodology.txt`、`a-aa-methodology-root.txt` 检索 `Default Fallback` 为 0 条 | 客观标明基于所查两份 AA 方法页 |

## 三、速览（C 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 谷歌：Gmail 接口列表含发信 | — | ✅ | 正文小节标题，逐字 | 主帖 1 |
| Claude：部分限能与反蒸馏 | — | ✅ | 正文小节标题，逐字 | 主帖 2 |
| 据凤凰网科技报道，DeepSeek 灰度显示本轮估算价，个人聊天仍免费 | L4 | ⚠️ | `c-c1-ifeng-orig.txt:75–87`（北京 11-27 18:09，灰度显示本轮对话估算成本及“个人日常聊天服务目前仍然免费……卡片显著位置仍标注‘免费’字样”）；`c-c1-deepseek-pricing.txt` 核对按 V4.1-Flash API 价估算 | 无官方收费公告 |
| 小红书开源 3D 框架，公开6414条 SFT 记录，论文5045条经验证轨迹 | L1 | ✅ | `c-c8-3dcodefactory-hf.txt:194–196`（抽查）：“publicly release the Qwen training dataset containing 6,414 SFT records… The paper reports results for a corpus of 5,045 verified trajectories (5.52B tokens).” | 补齐“SFT 记录”与“经验证”单位口径（card 受 40 字限制省略 Qwen 一词，完整描述为 Qwen SFT 记录）；Apache 2.0 |
| 谷歌支持称已修复 Gemini 网页版 Chrome 高内存，该回复未说明原因 | L1 / L2 | ✅ | `c-c4-reddit-opencli.txt:1`（`GoogleHelpCommunity` Official Google Support，抽查）与 `c-c4-reddit-meta.json`（`ts` 字段北京时间 11-25 01:15）：“This has now been resolved… high memory usage in Chrome”；原回复与 `c-c4-androidauthority.txt:43` 均显示该回复未说明原因 | 元数据时戳字段为 `ts` |
| 据亚洲新闻台援引彭博报道，SAP 考虑裁员重组；SAP 回应亚洲新闻台称未定案 | L4 | ⚠️ | `c-c5-cna.txt:1921,1927–1929,1969`（北京 11-27 00:33，抽查）：“considering cutting at least 10,000 jobs… Bloomberg News reported”；SAP 回应亚洲新闻台：“no decisions have been made regarding any further measures” | 两半分开署源；彭博原稿付费墙未抓取，已撤裁员数字 |
| Cogentic 用 Gemini-4 生成整数分解论文已公开，非谷歌官方产品 | L1 | ✅ | `c-c7-github-readme-ok.txt:28–32,51`（`c-c7-github-commit.json:13` 提交 `2026-11-25T10:21:27Z` = 北京 18:21，抽查）：“entirely obtained by Cogentic… using Gemini 4 as the base model.”“This is not an official Google product.” | 未审数学证明 |
| Kimi 科研首期开放理、工及交叉申请，入选最高15万元非现金，12月8日截 | L1 | ✅ | `c-c8-kimi-science.txt:91,105,131,135`（抽查）与 `c-c8-kimi-news.txt:96–104`：自然科学、工程学及相关交叉学科科研人员与团队，入选最高 15 万元 Kimi API 额度（非现金），首期 12 月 8 日截止 | 补齐“首期”“入选”“元”及并列学科范围 |
| 据 IT之家报道，腾讯混元发 Hy3D-3.6，供专业 3D 创作 | L4 | ⚠️ | `c-c8-hy3d-ifeng.txt:65,79`（凤凰网转 IT之家北京 11-27 11:46）：“面向专业 3D 创作…已经上线腾讯云 Token Hub” | 补齐“3D”产品范围；官网需登录、IT 之家原页待核，Token Hub 上线实况留在备注 |

## 四、批注译注与图题来源

| 卡片 / 字段 | 内容 | 级 | 结果 | 依据（含存档时间） |
|---|---|---|---|---|
| `ws-mcp-deadline` 标题/来源 | `Google Workspace 官方公告 · 强制期与标准档` / `googleblog.com 原文截图（2026-11-27 存档）` | L1 | ✅ | `b-ws-updates-blog.txt`、`b-ws-updates-feed.json:1`（2026-11-27 截图存档，公告正文 Nov 24） |
| `ws-mcp-deadline` ① | AI 代理或类似工具必须使用 Workspace MCP 服务器或官方命令行工具访问 Workspace 数据，不能直接调用 Workspace REST API；技术强制于 2027 年 2 月 1 日开始。 | L1 | ✅ | `b-ws-updates-blog.txt:184` |
| `ws-mcp-deadline` ② | 目前超过 99% 的活跃开发者在标准档内，将继续使用这些接口，无需为此次变化额外付费。 | L1 | ✅ | `b-ws-updates-blog.txt:186` |
| `mcp-v1-v1self` 标题/来源 | `Google Workspace MCP 工具总览（27日 UTC 更新）` / `developers.google.com 原文截图（2026-11-27 存档）` | L1 | ✅ | `b-mcp-server-tools.txt:1789`（`Last updated 2026-11-27 UTC.`） |
| `mcp-v1-v1self` ① | 总计 9 个正式可用（GA）的 Google Workspace MCP 服务器。 | L1 | ✅ | `b-mcp-server-tools.txt`（只框 `all nine generally available Google Workspace MCP servers`） |
| `mcp-v1-v1self` ② | Gmail 官方 MCP：主端点 `/mcp/v1` 列 30 个工具，备用端点 `/mcp/v1self` 列 25 个工具。 | L1 | ✅ | `b-mcp-server-tools.txt`（拆成 `Gmail`、`30 tools`、`25 tools` 三处独立高亮框） |
| `mcp-elicitation` 标题/来源 | `Google MCP 人工确认弹窗示例（send_message）` / `developers.google.com 原文截图（2026-11-27 存档）` | L1 | ✅ | 官方文档页面内嵌示例图 `mcp-elicitation-raw.png`（2026-11-27 截图 27 裁拼） |
| `mcp-elicitation` ① | 允许 Cymbal 代理对这位收件人使用 `send_message` 发信吗？ | L1 | ✅ | 底图 `mcp-elicitation-raw.png`（截图 27）像素可见 `Allow Cymbal Agent to use send_message for this recipient?`，高亮已避开图标且盖全两行 |
| `mcp-elicitation` ② | 下拉持久选项：始终允许向这位收件人发信、始终允许发邮件（管理员策略与 90 天上限另见帮助文档）。 | L1 | ✅ | 底图 `mcp-elicitation-raw.png` 可见菜单文字 `Always allow for this recipient`、`Always allow sending emails`；策略与 90 天另见 `b-admin-apps-help.txt:1636–1641` |
| `mcp-scaled-billing` 标题/来源 | `Google Workspace API 与 MCP 计费档对比` / `developers.google.com 原文截图（2026-11-27 存档）` | L1 | ✅ | `b-quota-pricing.txt:886–891,971`（2026-11-27 截图存档） |
| `mcp-scaled-billing` ① | 每日免费额度仅适用于 REST API；标准档下正式可用的 Workspace MCP 无需账单账号，每天每项目每产品限 5,000 次免费请求；项目切到 Scaled 档则取消免费 MCP 额度，Gmail、日历、网盘 MCP 从第 1 次请求起计费，每日免费额度为 0。 | L1 | ✅ | `b-quota-pricing.txt:886–891`（严格直译所框 `Important:` 整段） |
| `mcp-scaled-billing` ② | 2027 年 4 月 1 日起，Scaled 档的 Gmail、日历、网盘 MCP 用量没有每日免费额度，从第 1 次请求起计费（非 2xx 及返回 `isError: true` 的失败请求不计费，另见 Gmail 配额文档）。 | L1 | ✅ | `b-quota-pricing.txt:971` 严格直译＋`b-gmail-quotas.txt:2351–2358`（明标另见） |
| `fable-training-limit` 标题/来源 | `Anthropic 发布文 · 训练限能与推理不改这些能力` / `anthropic.com 原文截图（2026-11-27 存档）` | L1 | ✅ | `a-anthropic-fable-mythos.txt:497`（2026-11-27 截图存档；`kernel` 统译“内核”） |
| `fable-training-limit` ①–③ | ①训练期限制特定加速研发能力（加速器内核优化等，约 Opus-5.5）；②推理阶段不改动这些能力；③Fable-5.2 与 Mythos-5.2 对特定非标准加速器上的内核开发等少数主题明面回退 Opus-5。 | L1 | ✅ | `a-anthropic-fable-mythos.txt:497` |
| `edwinarbus-reply` 标题/来源/①② | `Anthropic 员工 Edwin Arbus 的公开回复（11月26日）` / `Edwin Arbus 公开帖截图（海外社媒 2026-11-27 存档）` / ①不降级服务期回答（Fable-5 上线头两天曾那样做，是个错误并很快修好）；②部分主题可能会回退（有提示，是我写的！）。 | L2 | ✅ | `a-x-tweets.json:87–88,106`（`2026-11-27` 截图存档，`user.name`/`description` 与回复上下文确认员工身份） |
| `sweeney-clarify` 标题/来源/①② | `Epic CEO 斯威尼首帖与次日凌晨澄清` / `Tim Sweeney 公开帖截图（海外社媒 2026-11-27 存档）` / ①反蒸馏挡住集合论工作；②仅批反蒸馏、肯定 Claude 顶尖模型。 | L2 | ✅ | `a-x-tweets.json:136,142,160`（`2026-11-27` 截图存档，`description` 确认 Epic Games CEO） |
| `refusal-billing` 标题/来源/①② | `Anthropic API 拒绝分类与零输出计费规则` / `platform.claude.com 原文截图（2026-11-27 存档）` / ①正常 ML 可触竞争模型类；②触发三类零输出前拒绝按运行模型费率计费。 | L1 | ✅ | `a-platform-refusals-fallback.txt:190,230`（2026-11-27 截图存档，② 同时框出类别句与费率句） |

## 卡片日期

`cards.toml` 的 `date` 写 `11月28日`（预计发布日；交付时间为北京 11-27 深夜）。

## 反向核验与视觉复核

### 第一轮反向核验（Codex `gpt-6.2-sol` medium，read-only；北京 11-27 22:50–22:57）

5 BLOCKER、9 SHOULD_FIX、2 NEEDS_VERIFICATION：全部采纳落地（去掉“限两项”封闭清单与“误封”断语，将 Gmail 调为主帖一，补 `@ClaudeDevs` 道歉原帖与 `b-gwscli-gmail-help.txt`，去 `@RealAstropulse`，速览换 `3DCodeFactory`）。

### 第二轮视觉复核与反向核验（Codex `gpt-6.2-sol` medium，read-only；北京 11-27 23:40–23:46）

- 视觉 5 BLOCKER、反向 3 BLOCKER 及对应 SHOULD_FIX 全部落实：重定 20 字全 L1 主标题「Gmail接口已列发信，默认没确认发不了」，重出 B 组第 2 版封面（去绿灯对勾改清单卡与半开防护罩小锁确认键），删省流「实为」，区分 `3DCodeFactory` 两数，撤 SAP 裁员数字，补抓 Frank Chu `b-frankchu-23-or-30.txt`，修复 `06`/`07`/`14` 高亮坐标及 `6-11`、`ts` 笔误。

### 第三轮视觉复核与反向核验（Codex `gpt-6.2-sol` medium，read-only；北京 11-28 00:00–00:10）

- **视觉复核二**（2 BLOCKER、12 SHOULD_FIX、1 NICE_TO_HAVE；封面零 BLOCKER）：
  1. BLOCKER 1（`04-bark-1` 漏掉 `Premium` 桶）：按反向核验三 BLOCKER #2 统一采用最小改法——正文与卡片删去 REST 100 万次数字，只保留三项 Scaled MCP 规则；Light/Medium/Premium 三桶数字完整留存清单备注。
  2. BLOCKER 2（`02-roundup` 写成“6414 条训练集”）：采纳，改为 `公开6414条 SFT 记录，论文5045条经验证轨迹`。
  3. SHOULD_FIX：采纳 `02-roundup` Kimi 补“元”“首期”“入选”与并列范围、`06` 拆成 `Gmail`/`30 tools`/`25 tools` 三框、`07`/`08` 译注直译与“另见”标注、`10-bark-2` 拆开 Edwin 与官号道歉、`11` 统译“内核”、`12`/`13` 页脚补出处主体。保留项说明：封面与 `14-refusal-billing` 深底色均已核对清楚、且受制于页面原主题及模板渲染机制（下划线位置同模板默认，未重取原图或改模板）。
- **反向核验三**（3 BLOCKER、9 SHOULD_FIX、1 NICE_TO_HAVE、3 NEEDS_VERIFICATION）：
  1. BLOCKER 1（Gmail 工具列表 vs 实际发信未测、审批管理员例外）：采纳，事件卡补写`未测鉴权发信`，吠点②写全管理员放行与`非合作方实发待鉴权实测`。
  2. BLOCKER 2（REST 100 万缺桶与“不豁免自用”口径）：采纳，正文删除 REST 100 万裸数字并明确“不享组织内部提额例外”。
  3. BLOCKER 3（Claude「对你」写成“训练不涉”）：采纳，改为`官方称未影响代理式编程与传统机器学习研究`。
  4. SHOULD_FIX 4–12 与 NICE_TO_HAVE：全面落实——补 `Workspace` 与 `27日 UTC 更新` 口径、补“无鉴权同轮”与移除封面“旧/新”措辞、省流补“研发时”与两新模型、`Arbus` 与官号道歉分述、限制 AA 方法页表述、拆开清单来源层级（区分 ritza 作者引文、HN 采样与个人无鉴权实测）、补齐 SAP 回应亚洲新闻台归因、补全所有精确行号与 8 张批注登记，并更新 `images/README.md` 手选封面顺序与第 7 图对应段落。
