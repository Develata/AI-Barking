# 1126 来源与制作状态

期号 1126（上期 1125 + 1）；线索以 Develata 回贴的 WorkBuddy HTML 选题扫描 `1127-18`（北京 11-27 18:02）为主，并现查 HN 近 36 小时高分帖（补入 Google Workspace MCP 主帖）；同时附来的 ChatGPT rescan 邮件是 11-19 13:16 的旧件（与 1124/1125 同份），未使用。Develata 北京 11-27 约 21:08 拍板主帖 1 + 2；经读者稿检查与一轮、二轮反向核验修订，将 Google Workspace MCP 调为第一条主帖、Anthropic Fable-5.2 调为第二条主帖，并以「Gmail接口已列发信，默认没确认发不了」为主标题。MiniMax/Dragos 延用 1121 期风控不收，SUICE V0.1、韩国 KFTC 及含 FDA 审批的 Neuralink 速览条目亦未收入正文。

派工单 `.handoff/2026-11-26-1126-evidence.md`。派工前 `opencli doctor`（北京 21:06）：Browser Bridge 已连接。A、B、C 三组均由 Antigravity（Gemini 模型家族）取证子代理并行完成。

## 扫描勘误

- **B 组（各时点观测与口径勘误）**：Tech Stackups 称“Gmail MCP 不能发邮件”；各时点观测记录为——Wayback `20261126211519`、Suncly `contradictions.json`（北京 11-26 19:03:29，主端点 23 工具、旧文档写须合作方权限）、Frank Chu `b-frankchu-23-or-30.txt`（探测于美山 Nov 26 ~16:40 MST ≈ 北京 11-27 07:40，非鉴权 30 次返回 23 项 18 次 / 30 项 12 次）、本号北京 11-27 21:14–21:19 实测 `/mcp/v1` `tools/list` 与 `gwscli gmail --help`（均列含 `send_message`/`reply`/`forward` 的 30 个工具；未测带鉴权实际发信）。上述观测仅反映对应时刻无鉴权列表结果，不推断服务端具体切换时间或部署机制。无弹窗备用端点 `/mcp/v1self`（25 工具）仍无发信、回复和转发；主端点发信默认须确认（策略允许时最多存 90 天）。另有：GA 服务器总览页为 **9 个**（含 People API）；标准档超限仅限流不扣费，选 Scaled 档后 2027 年 4 月 1 日起对 Gmail/日历/网盘 MCP 首笔成功请求计费（非 2xx 与 `isError: true` 不计）。
- **A 组（时间、范围与回应勘误）**：Pietro Schirano（`@skirano`）发帖为北京 `11-25 07:28:56`（非 11-24），未见公开 `DoNotSabotage` 仓库；Tim Sweeney 首帖为北京 `11-25 16:53:18`（非 11-26），并在 `11-26 02:44:39` 澄清仅批反蒸馏；Anthropic 员工 `@edwinarbus` 已于 `11-26 09:03` 与 `11:50` 回复 Schirano，澄清仅 6 月 Fable-5 头两天有服务期降级（`@ClaudeDevs` 已于 `06-11` 道歉），官方称 Fable-5.2/Mythos-5.2 训练期限制的是**包括**算子优化与改进大模型架构在内的部分研发能力（`including`，非封闭两项）；“90%–99%”出自 10-12 报告在自有平台上估算的推理 token 提取量降幅。
- **C 组（旧闻、口径与链接勘误）**：Yandex `AliceAI-Foundation-80B-A3B-Base` 早在 `2026-09-21` 已开源（11-25 仅补交 arXiv 论文），不收；`3DCodeFactory` 公开 Qwen SFT 训练集为 **6,414** 条，**5,045** 条经验证轨迹为论文统计语料；派工单 C7 GitHub commit hash 漏截 `7966c50049`，已用完整 hash `4a7bc8ab0bc770e71a259b7966c500496b99010a` 核对；C4“8.4GB 峰值”出自 Android Authority 引述的另一用户旧报告；SAP 彭博原稿付费墙未抓取，正文撤去具体裁员人数。

## 主帖

1. Google Workspace 强制 MCP 与 Gmail 发信列表更新（B 组，`evidence-b.md`、`capture-log-b.md`）：
   - [Workspace Updates 公告](https://workspaceupdates.googleblog.com/2026/11/mcp-servers-cli-standardized-usage-tiers-now-available.html)（正文标 `November 24, 2026`，北京 11-25 01:02:59 发布、11-27 06:28:37 更新）；[Workspace AI Tools Addendum](https://workspace.google.com/terms/ai-tools)；[MCP 工具总览](https://developers.google.com/workspace/guides/mcp-server-tools)（`Last updated 2026-11-27 UTC.`）；[Gmail MCP 参考](https://developers.google.com/workspace/gmail/api/reference/mcp)及 `send_message` 页；[Form Elicitation 指南](https://developers.google.com/workspace/guides/mcp-elicitation)；[Workspace CLI 指南](https://developers.google.com/workspace/guides/workspace-cli)及实测输出 `b-gwscli-gmail-help.txt`；[用量档位与计费页](https://developers.google.com/workspace/guides/quota-pricing)、[Usage Tiers](https://developers.google.com/workspace/guides/quota-usage-tiers)、[Gmail Quotas](https://developers.google.com/workspace/gmail/api/guides/quotas)。
   - 反转与夸大实例：Wayback `20261126211519` 快照；Suncly `contradictions.json`；[Frank Chu dev.to 23/30 实测](https://dev.to/frankchu/23-or-30-tools-googles-gmail-mcp-cant-decide-59j3)（存档 `b-frankchu-23-or-30.txt`）；[TechStackups](https://techstackups.com/articles/google-workspace-ai-agents-mcp/)；[动察 / BlockBeats](https://www.theblockbeats.info/flash/379465)。
   - 热度：HN item `50653477`（11-26 17:44:23 提交，27 日晚采样 356–359 分 / 307–308 评）。
2. Anthropic Fable-5.2 限能与反蒸馏争议（A 组，`evidence-a.md`、`capture-log-a.md`）：
   - [Fable-5.2 发布文](https://www.anthropic.com/claude-fable-and-mythos-5-2)（2026-11-09）；[Fable-5.2 系统卡](https://www.anthropic.com/document/claude-fable-5-2-mythos-5-2-system-card)（§1.5）；[反蒸馏报告](https://www.anthropic.com/news/distillation-defenses)（2026-10-12）；[Platform 拒绝与回退文档](https://platform.claude.com/docs/en/build-with-claude/refusals-and-fallback)；[Help Center 自动切换说明](https://support.claude.com/en/articles/15363606-why-claude-switched-models-in-your-conversation-with-fable-models)；[Fable-5 道歉原帖](https://x.com/ClaudeDevs/status/2064949876463645026)（`a-claudedevs-apology.json`）及 [Decrypt 报道](https://decrypt.co/370831/anthropic-apologizes-claude-fable-5-secret-censorship)。
   - 当事方原帖（海外社媒公开 JSON `a-x-tweets.json`）：`@skirano`（`2125371115979641188`）、`@TimSweeneyEpic`（`2125513144353411115` / `2125661964617871830`）、`@edwinarbus`（`2125757350040711397` / `2125799392481571135`）。备用线索：`@RealAstropulse`（`2126119408997466389`，未进正文）。
   - 第三方榜与夸大实例：[Artificial Analysis 榜单](https://artificialanalysis.ai/leaderboards/models)；[51CTO](https://www.51cto.com/article/862320.html)；[动区](https://www.blocktempo.com/claude-accused-sabotaging-ml-research-anthropic-documents-frontier-llm-limits/)；[IT之家](https://www.ithome.com/1/032/822.htm)。
   - 热度：HN item `50664325`（11-27 16:28:08 提交，148–152 分 / 97–98 评）；Reddit `1y65dvn`（133 分 / 60 评）、`1y6s6ze`（49 分 / 20 评）。

## 速览（C 组，`evidence-c.md`、`capture-log-c.md`）

- DeepSeek 灰度卡片：[凤凰网科技](https://original.ifeng.com/c/8yOj7tdntji)、[IT之家](https://www.ithome.com/1/033/157.htm)、[DeepSeek 定价页](https://api-docs.deepseek.com/zh-cn/quick_start/pricing/)。
- 小红书 3DCodeFactory：[Hugging Face 数据集](https://huggingface.co/datasets/3DCodeFactory/3DCodeFactory)（公开 6,414 条 Qwen SFT 记录）、[GitHub](https://github.com/3DCodeFactory/3DCodeFactory)、[arXiv 2611.28783](https://arxiv.org/abs/2611.28783)（论文统计 5,045 条经验证轨迹）。
- Gemini 内存修复：[Reddit 原帖](https://www.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage/)与 [Android Authority](https://www.androidauthority.com/google-gemini-chrome-ram-hogging-issue-fixed-3739606/)。
- SAP 裁员考虑：[CNA](https://www.channelnewsasia.com/business/sap-job-cuts-ai-restructuring-6585781)。
- Cogentic 整数分解论文：[GitHub 目录](https://github.com/google-research/google-research/tree/4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/integer_factorization)。
- Kimi for Science：[官网新闻](https://www.kimi.com/news/kimi-for-science-program)。
- 腾讯混元 Hy3D-3.6：[IT之家原稿](https://www.ithome.com/1/033/282.htm)与[凤凰网转载](https://tech.ifeng.com/c/8yOKHwwsCrm)。
- 未收：C2 Neuralink（含 FDA 监管许可）、C3 韩国 KFTC 与 C8e 英国 FCA（风控）、C6 Yandex（9 月旧闻）、C8b Kimi 游戏方案（已核备用）。

## 待删除与不入库

- 仓库内待删除废图与失败抓取空壳将在终审校验前一次删除（含 A 组旧版截图 3 张与 6 个拦截空壳、B 组废图 2 张、C 组失败抓取与未用尝试文件 29 个）。全部 `.html`、`.pdf` 原件与 `images/` 下图片不入库，随 `sources/offsite.tsv` 上传 OpenList。
