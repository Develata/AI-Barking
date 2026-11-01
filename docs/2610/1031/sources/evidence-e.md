# 1031 期 E 组取证：速览二（evidence-e）

执行方：claude-haiku-5-5（Haiku 5.5）。取证时刻：2026-11-02 02:32–02:35 北京（本机 11-01 13:32–13:35 EST）。
不截图（D/E 组）。文本存档与摘句见各条；抓取与失败记录见 `capture-log-e.md`。

## 汇总表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| E1 Northflank 安全事件 | L1 | https://status.northflank.com/history/1 （当前页 https://status.northflank.com/） | "Users have reported unauthorised use of AI API keys supplied to the Northflank platform." | 无 | 北京时间，见下文各时刻 | 已找到 |
| E2 Wiz “Black October” | L3（Wiz 自家研究，非独立第三方） | https://www.wiz.io/blog/black-october-for-ai-developer-platforms | "At least nine AI Developer Platforms have disclosed incidents over the course of the month" | 无 | 页面日期 October 30, 2026（官方未给时刻） | 已找到（名单与调用数）；“超 10 万美元”部分支持 |
| E3 商汤 SenseNova-Loop-9B-Preview | L1（模型仓库与模型卡）；“商汤”归属未在页面中确认 | https://github.com/OpenSenseNova/SenseNova-Loop ； https://huggingface.co/sensenova/SenseNova-Loop-9B-Preview | "~9B total parameters; ~14.6B inference parameters"；"License: Apache License 2.0" | 无 | 发布标注 "[2026.10.30]"（官方未给时刻） | 部分支持 |
| E4 ChatGPT 分享链接漏洞（已修复） | L3（研究方自述；OpenAI 侧原文未找到） | https://www.parameter.ai/labs/we-found-a-gap-at-openai | "OpenAI fixed the issue on October 26 and classified the report as P1." | 无 | 修复日期仅有日期（官方未给时刻）；文章作者日期 Oct 28, 2026 | 部分支持 |
| E5 Grok 文本水印 | L1（xAI 官方 API 文档）；L4 IT之家转述 | https://docs.x.ai/developers/advanced-api-usage/watermarking | "Starting November 7, 2026, responses from these models include a watermark. It's on by default, and you can opt out per model" | 无 | 日期无时刻；范围限 API 文本生成端点 | 部分支持 |

## E1 Northflank 安全事件

- 事实：Northflank 状态页自 10-30 起以 "Security incident" 记录事件；官方称有用户报告 AI API 密钥被未授权使用，建议所有用户轮换密钥，调查仍在进行，尚无进一步确认结论。
- 一手来源：https://status.northflank.com/history/1 （存档 `e-northflank-history.html` / `.txt`）；当前页 https://status.northflank.com/ （存档 `e-northflank-status.html` / `.txt`）。
- 发布时刻（北京时间，原时区 UTC）：
  - 首条 "Investigating" 10-30 21:40（13:40 UTC）；
  - 10-31 07:38 北京（10-30 23:38 UTC）那条 "Users have reported unauthorised use…"；
  - 末条 11-01 07:31 北京（10-31 23:31 UTC，RabbitMQ 密钥轮换可用）。取证时仍为 "Identified · Major outage"。
  - 状态页事件记录另列出 10-30 19:43 北京（11:43 UTC）"Wiz threat alert"。
- 级别：L1（官方状态页）。
- 原句（逐字）：
  - "Users have reported unauthorised use of AI API keys supplied to the Northflank platform."
  - "Our investigation is ongoing. We have no further confirmed findings to share at this time."
  - "assume all environment variables have been compromised and rotate all secrets and credentials immediately"
  - "Review your AI provider usage and billing logs for unexpected activity, and report any suspicious usage to Northflank support and the relevant provider."
- 能改变含义的限定词：“Users have reported”（用户报告）、“no further confirmed findings”（无进一步确认结论）、“assume … compromised”（预防性假设，非确认泄露）。官方归档文本中未出现 "breach" 一词；"Breached" 只见于 HN 标题。
- 热度（`barking heat`）：HN 50305025 "Northflank Breached"，477 分、360 评，10-30 23:10 北京（`e-hn-50305025.json`）。另 HN 50304345 "Northflank PaaS: Security Incident - Disable/rotate all keys" 3 分 1 评；HN 50309881 2 分 1 评。Reddit 失败（见 capture-log）。
- 状态：已找到。

## E2 Wiz “Black October”

- 事实：Wiz 10-30 博客称，10 月内至少九家 AI 开发平台披露事件，并以两把被盗 AI 密钥为例，记录 Bedrock 调用：约 80 万次（约 90% 成功）与约 170 万次（超过 100 万次成功）。
- 一手来源：https://www.wiz.io/blog/black-october-for-ai-developer-platforms （存档 `e-wiz-black-october.html` / `.txt`）。
- 发布时刻：页面日期 October 30, 2026，官方未给时刻；HN 投稿 50311844 为 10-31 08:11 北京。
- 级别：L3（Wiz 为安全厂商，此文为其自家研究，不是独立第三方；级别判定待 Claude 定）。
- 名单（原文顺序，日期为原文括号内）：Fireworks AI (Oct 6)、Andromeda (Oct 9)、Zeabur (Oct 14)、Lightning AI (Oct 15)、OpenCode (Oct 16–17)、Railway (Oct 20)、Warp (Oct 23)、CommandCode (Oct 28)、Northflank (Oct 30)。
- 原句（逐字）：
  - Case A: "the actor made approximately 800k Bedrock calls, and about 90% of them succeeded."
  - Case B: "that single Level 3 address made approximately 1.7 million Bedrock calls, with over a million successful."
  - 金额: "At list price, these two cases alone likely ran up over $100k in usage."
  - 归因: "we are not able to definitively establish the relationship between these reselling organizations and either the initial compromise or the “flood” token abuse documented above"
  - 归因（另一句）: "whose compromise we can attribute to one of these vendor breaches"
  - 总体: "Currently, most of these security incidents are still being analyzed, and detailed technical information is not available."
- 口径：800k、1.7M 是调用数（calls），不是成功数；成功数见上句。金额是按标价（list price）估算，原文用 "likely"。
- 能改变含义的限定词：“approximately”、“likely”、“at list price”、“most … still being analyzed”。
- 热度：HN 50311844 "Black October: At least nine AI developer platforms breached this month"，6 分、0 评，10-31 08:11 北京。其余 HN 条目（如 50326775 "Show HN: Black October"，1 分，11-01 23:52 北京）只作参考，不计为本条热度。Reddit 失败。
- 状态：已找到（名单、调用数）；“超 10 万美元”为部分支持（原文 "likely"，扫描删去了）。

## E3 SenseNova-Loop-9B-Preview

- 事实：GitHub 仓库与 Hugging Face 模型卡列出 SenseNova-Loop-9B-Preview，README 写 "~9B total parameters; ~14.6B inference parameters"，许可证 Apache-2.0；README 发布标注 "[2026.10.30] We release SenseNova-Loop-9B-Preview"。
- 一手来源：https://github.com/OpenSenseNova/SenseNova-Loop （`e-sensenova-gh.html` / `.txt`）；https://huggingface.co/sensenova/SenseNova-Loop-9B-Preview （`e-sensenova-hf.html` / `.txt`）。
- 发布时刻：官方仅给日期 2026.10.30，未给时刻。
- 级别：L1（模型仓库与模型卡）。“商汤”归属：两页面均未出现 "SenseTime" 或“商汤”字样；组织名为 OpenSenseNova（GitHub）与 sensenova（HF）。一次 WebSearch 未找到商汤官方发布原文，仅得到 IT之家 2026-07 关于商汤 SenseNova-Vision 的报道（L4 线索，非本条证据）。归属需 Claude 另行确认。
- 参数口径：README 为 "~9B total parameters; ~14.6B inference parameters"；HF 页面 Safetensors 参数量显示 "10B params"。两个数字都记录，不取舍。
- 许可证：README "License: Apache License 2.0"；HF 元数据 "License: apache-2.0"。
- 对比分数：README 有 "Key Benchmarks" 小节，但分数以图片 `assets/loop9b-benchmark-results.png` 呈现，图中对比对象与数字未读（未下载，见 capture-log 待决事项）。归档文本中未找到 "27B" 字样，即“媲美 27B”未在页面文本中找到。
- 技术报告：HF 页面写 "Technical report / TBD"，并有 "Publication details to be announced"。扫描称“技术报告 TBD”属实，但仅见于 HF 页面。
- 限定词：README 称 "a looped multimodal model"，技术细节以 "Preview" 标注。
- 热度：HN 搜索 "SenseNova-Loop" 0 条（"SenseNova" 命中 HN 50322032，与本模型无关）；Reddit 失败。GitHub 截取时 67 stars、5 forks；HF 截取时 Like 137。
- 状态：部分支持（参数口径、许可证、发布日期已找到；对比对象、27B、技术报告的“TBD”中，TBD 已找到，27B 与对比分数未找到文本来源）。

## E4 ChatGPT 分享链接漏洞（已修复）

- 事实：研究方 Parameter 的文章称，其于 10 月发现 ChatGPT 公开分享链接相关的一处漏洞，研究方称 OpenAI 于 10 月 26 日修复，并将报告定为 P1。（不复述技术细节。）
- 一手来源：https://www.parameter.ai/labs/we-found-a-gap-at-openai （`e-parameter-gap.html` / `.txt`）。
- 发布时刻：文章页面作者日期 "Oct 28, 2026"；修复日期 "October 26, 2026"（仅日期，官方未给时刻）。
- 级别：L3（研究方自述）。OpenAI 方面的修复说法据研究方；一次 WebSearch 未找到 OpenAI 自身公告。
- 原句（逐字，修复相关）：
  - "OpenAI fixed the issue on October 26 and classified the report as P1."
  - 文中时间线 10-26 条目记录研究方所称的 OpenAI 确认与“已解决”结论（只记结论，技术细节不转录）。
- 野外利用：文中未找到“野外利用”（in the wild）或真实用户受害的原句；文章描述为研究方在测试账号中复现。
- 赏金：文中写 "awarded a total bounty of $5,000"（如需要）。
- 热度：HN 50314699 "Do the math: How we took over ChatGPT accounts"，357 分、176 评，10-31 18:30 北京（`e-hn-50314699.json`）。扫描称“334 分 165 评”，取证时数字已更新，以取证为准。
- 状态：部分支持（修复日期为研究方自述；OpenAI 原文未找到）。

## E5 Grok 文本水印

- 事实：xAI 官方 API 文档称，自 2026 年 11 月 7 日起，列出的 Grok 文本模型的 API 文本输出包含隐形水印，默认开启；团队管理员可按模型关闭。
- 一手来源：https://docs.x.ai/developers/advanced-api-usage/watermarking ，页面标题 "Output Watermarking | SpaceXSI Docs"（`e-xai-docs-watermarking.html` / `.txt`）。
- 发布时刻：官方文档仅给 "Starting November 7, 2026"，未给时刻，页面无发布日期。IT之家报道时间 2026/11/1 14:11:02（北京，`e-ithome-grok-721.txt`）。
- 级别：L1（xAI 官方 API 文档）。IT之家为 L4 转述（`e-ithome-grok-721.txt`）。
- 范围（官方原句）：
  - "SpaceXSI watermarks text model output to comply with the EU AI Act."
  - "The watermark is enabled on every SpaceXSI API endpoint, including the global endpoint, https://api.x.ai and any regional endpoint"
  - "Watermarking applies to outputs from every text generation API, including the Responses API and the Chat Completions API."
  - 列出的模型：grok-4.7、grok-4.6、grok-4.5、grok-4.3、grok-4.20-reasoning、grok-4.20-non-reasoning、grok-4.20-multi-agent。
- 不可见性："The watermark is not visible." 检测为统计性 ："Detection is statistical."
- 限定词：范围为 API 文本生成端点（"every SpaceXSI API endpoint"）；官方页未提及 Grok 网页或 App 的文本，不能写“Grok 全面加水印”。“Starting November 7, 2026” 为计划生效日期（页面对此有 "Starting"）。
- 公司名：官方页面写 "SpaceXSI"，IT之家写 "SpaceXAI"。正文统一用词待 Claude 定。
- 冲突的旧表态：X 上 @grok 账号 2026-08-15 07:19:15 UTC（北京 15:19）发文称 "No. Grok applies no text watermarking of any kind."（`e-grok-x-no-watermark-fx.json`，经 api.fxtwitter.com 取）。该帖早于本次文档，取证时未找到其撤回或更新的说明。
- 热度：HN 搜 "Grok watermark" 与 "Grok text watermark" 均 0 条；Reddit 失败。
- 状态：部分支持（11-07 起 API 文本水印属实；范围限于 API，消费者端未见于官方文档）。

## 扫描说法勘误

1. E1：扫描称“官方要求客户审计并轮换密钥”。官方原文为请求式措辞（"Please rotate … immediately"、"Review your AI provider usage and billing logs"）。“有用户报告 AI API 密钥被盗用”：官方原文是 "unauthorised use"（未授权使用），"被盗用" 为扫描用语。“仍在处理”属实。
2. E2：扫描称“按标价折算超 10 万美元”。原文为 "likely ran up over $100k"（"likely" 被删去）。扫描称“约 80 万与 170 万次 Bedrock API 请求”属实，但按原文是调用数（calls），其中成功约 90%（第一案）与超过 100 万次（第二案）。
3. E3：扫描称“~9B 总参数、~14.6B 推理参数、Apache-2.0”属实，但 HF 页面显示 "10B params"，需一并记录。“媲美 27B”在已存档文本中未找到。“技术报告 TBD”属实（HF 页面）。“对比分数只在一张 PNG 里”属实（`assets/loop9b-benchmark-results.png`，未读图）。“商汤”归属未在页面中确认。
4. E4：扫描称 HN “334 分 165 评”，取证时为 357 分、176 评（10-31 18:30 北京）。“研究方称 10-26 已修复”属实，但修复说法的来源仅为研究方；OpenAI 原文未找到。
5. E5：扫描称“Grok 将于 11-07 起给文本加水印”需限定为“xAI API 的文本输出”，官方文档未写消费者产品。“IT之家 11-01 14:11”属实（2026/11/1 14:11:02）。另有 2026-08-15 的 @grok 旧表态称无文本水印，未在扫描中提及。

## 无法取到的一手材料（写明未找到）

- x.ai/news、x.ai/legal/grok-faq、x.ai/legal/brand-guidelines：HTTP 403，未取到正文（见 capture-log）。
- Reddit 搜索全部失败（重定向登录页）。
- OpenAI 对 E4 修复的自身公告：未找到。
- SenseNova 基准图与“媲美 27B”的原文来源：未找到文本来源。
