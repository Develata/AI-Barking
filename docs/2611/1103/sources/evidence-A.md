# 1103 期 A 组取证：腾讯 Hy4 称“最强开源”与 Kimi-K3.1-Pro 权重到期【跟进1023】

执行：Sonnet 5.5（claude-sonnet-5-5）。核查时刻均为北京时间（UTC+8），括号内为原时区。本组首次核查 20:46，Kimi 与 HF 复查 21:24（见 A6）。
截图在 `docs/2611/1103/images/`，A 组区间 01–12；数量超出 12 张，多出的几张用字母后缀（`03a`、`04a`、`05a`）挂在同主题的编号下，未占用 B 组的 13–24。

## 一眼结论（verified）

1. **“将于近期开源模型权重、发布模型系统卡片”这句话存在，出处是腾讯官网（tencent.com）简体/繁体中文新闻稿首段，不在腾讯新闻版里。** 腾讯官网英文稿写 “Tencent Hy plans to release model weights and system cards in the coming days”；混元 Blog（中、英）只写“我们计划开源模型权重……届时也会发布模型系统卡片”，没有时间词。四个版本时间词互不相同（近期 / in the coming days / 无）。
2. 截至 21:24（北京），HF `tencent`、GitHub `Tencent-Hunyuan`、ModelScope 均没有 Hy4 正式版；只有 8 月 27–28 日开源的 `Hy4-preview`（Apache-2.0）。“开源第一”的标题是在权重未放出时发的。
3. Kimi-K3.1-Pro：官方博客与官方 X 帖都写 “by Nov 4, 2026”，未给时刻与时区。21:24（北京）HF `moonshotai` 仍无 K3.1 系列，到期日在北京时间尚未过去。反面样本：什么值得买 11/4 08:56 一篇汇总标题写“今天正式开源全部权重”。
4. 官方自己的对比图里，Hy4 在 12 项基准全部低于 Claude Opus 5.5，与 Kimi K3.1 Pro 6 比 6（目视读数，见 A4）；混元 Blog 的“局限性”一节自称与顶尖闭源模型“仍然存在较大差距”。
5. 官方稿之间数字不一致：同一句“幻觉率”，腾讯官网稿写 12.95%→7.2%，混元 Blog 写 15.76%→5.45%；API 美元价三处各异。

## 取证清单逐条

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1-1 | 腾讯新闻版官方稿：标题与发布时间 | L1 | https://news.qq.com/rain/a/20261104A074Z600 | 标题“Hy4正式发布，跻身最强开源模型之列”；署“腾讯 2026-11-04 12:52 发布于广东 腾讯官方账号”；首句“刚刚，腾讯混元发布最新一代大语言模型 Hy4。随着模型能力的再次全面跃升，Hy 首次跻身最强开源模型之列。” | `01-tx-news-top.png` | 12:52 北京时间。WorkBuddy 扫描称的 14:06 与此不符，也晚于 网易智能 13:25:31 的转载。正文无任何“权重”“系统卡”字样（对 HTML 与 txt 全文检索，0 处） | 已找到 |
| A1-2 | 腾讯官网简体中文稿 | L1 | https://www.tencent.com/zh-cn/tencent-launches-hy4-joining-the-ranks-of-the-worlds-leading-open-source-ai-models/ | 标题“腾讯混元发布Hy4，跻身最强开源模型之列”，日期“2026年11月04日”；首段“11月4日，腾讯混元发布最新一代大语言模型 Hy4。随着模型能力的再次全面跃升，Hy 首次跻身最强开源模型之列。Hy4 将于近期开源模型权重，发布模型系统卡片，并积极推进更负责任的开源，重点关注网络和生物安全等风险。” | `05a-tencentcom-zhcn-lead.png` | 官方只给日期，未给时刻。繁体版 `zh-hk` 同句（“Hy4將於近期開源模型權重，發布模型系統卡片”）。headless Chrome 渲染该 zh-cn 地址被送到英文版，故中文页用 opencli 截 | 已找到 |
| A1-3 | 腾讯官网英文稿 | L1 | https://www.tencent.com/tencent-launches-hy4-joining-the-ranks-of-the-worlds-leading-open-source-ai-models/ | 标题“Tencent Launches Hy4, Joining the Ranks of the World’s Leading Open-Source AI Models”，“November 4, 2026”；“Tencent Hy plans to release model weights and system cards in the coming days, reinforcing its commitment to responsible open-source AI development, with a focus on cybersecurity and biological safety.” | `05-tencentcom-en-lead.png` | 只给日期。中文“近期”在英文稿里变成 “in the coming days”（更具体）。小标题 “Delivering Greater Cost Performance, Hy4 Flash Recently Released” 与正文 “Hy4 Flash is scheduled for release soon” 自相矛盾（中文稿小标题“Hy4 Flash近期上线”与正文“也将于近期发布”同样不一致） | 已找到 |
| A1-4 | 混元 Blog 中文版 | L1 | https://hunyuan.tencent.com/research/hy4?langVersion=zh | 页面日期“2026年11月4日”；标题“Hy4 正式发布”；“今天，我们发布 Hy4 正式版。随着模型能力的再次全面跃升，Hy 首次跻身最强开源模型之列。” | `03a-hy-blog-weights-limits.png`（含权重句） | 博客列表页把该文标为“2026-11-03”，详情页写 11月4日，二者差一天，原因不明（可能是草稿日期），只记。页面是 JS 渲染，`barking fetch` 判 blocked，改用 opencli 抽取并存 `a-hy-blog-hy4.md/.html` | 已找到 |
| A1-5 | 混元 Blog 英文版（hy.tencent.ai） | L1 | https://hy.tencent.ai/research/hy4?langVersion=en | “Today, we’re releasing Hy4, another major leap forward that places Hy among the strongest open-source models for the first time.” | 无 | “November 4, 2026”，只给日期。HN 提交的就是这个地址 | 已找到 |
| A1-6 | 微信公众号“腾讯技术工程”“腾讯混元”原文 | — | 未取得 | 搜狗微信搜索能搜到十条转述，点开均遇微信验证码页（未绕过）。某聚合摘要标“来源：公众号：腾讯混元 \| 发布时间：11-04 12:52 北京时间”（二手，L5） | 无 | 公众号原文**未找到一手来源**。但 IT之家 2026/11/4 12:59:42 已引“官方表示……Hy4 将于近期开源模型权重，发布模型系统卡片”，说明该句在发布当时的通稿里就有（推断） | 未找到一手来源 |
| A1-7 | 各版本差异 | — | 见上 | 见 A2 差异表 | — | — | 已找到 |
| A2-1 | “开源”的依据：Hy4 权重是否已放出 | L1 | 同上 + 下方 A3 | 见 A2 表；无任何版本说“已开源”权重 | `03-tx-news-responsible.png`、`03a-…`、`05-…`、`05a-…` | 全部是将来时：“将于近期”“plans to … in the coming days”“我们计划”。**没有具体日期** | 部分支持（“开源”指计划，非已发生） |
| A2-2 | 近期开源权重这句在哪个版本 | L1 | tencent.com 简/繁中文稿首段；英文稿；Blog 倒数第三段 | “我们计划开源模型权重，并已联合 vLLM/SGLang 社区与 Inferact Inc. 提供开源生态支持。我们届时也会发布模型系统卡片，包括更全面的安全评估等细节。”（Blog 中文）；“We plan to open-source Hy4’s model weights and have partnered with the vLLM (Inferact Inc.) and SGLang communities to provide ecosystem support. When we release the weights, we will also publish a comprehensive system card detailing our safety evaluations and additional technical details.”（Blog 英文） | `03a-hy-blog-weights-limits.png` | 腾讯新闻版把该句整句删掉，只保留“更多细节可点击阅读原文或前往腾讯混元 Blog” | 已找到 |
| A3-1 | HF `tencent` 组织无 Hy4 正式版 | L1 | https://huggingface.co/models?sort=created&author=tencent ；API `https://huggingface.co/api/models?author=tencent&sort=createdAt&direction=-1` | 最新创建：`tencent/Hy-Nomas-Diffusers` 2026-10-24T05:45:20Z，之后 WeVisOne-VL-4B/2B（10-22）、Hy-SimulMT 系列（10-22）、Hy-Nomas（10-21）。组织页 Collections 里仍是 “Hy4 preview”（`tencent/Hy4-preview`、`Hy4-preview-FP8`）。按 lastModified 排序最新为 Youtu-Parsing-Omni（10-30T09:15Z） | `09-hf-tencent-models-recent.png`（页面“Models 176 / Sort: Recently created”） | 20:46 与 21:24 两次 API 一致；`api/models/tencent/Hy4`、`…/Hy4-FP8` 返回 401（HF 对不存在或私有仓库的统一响应）。注意 HF 的 createdAt 是仓库创建时间，可能早于公开时间（推断：`moonshotai/Kimi-K3` createdAt 06-13，而官方博客 07-16、GitHub 仓库 07-27，疑为先建后公开），所以另用 lastModified 排序与直接探测各复核一次，结论相同 | 已找到（无 Hy4 正式版） |
| A3-2 | HF `Tencent-Hunyuan` 组织 | L1 | `https://huggingface.co/api/models?author=Tencent-Hunyuan&sort=createdAt&direction=-1` | 最新 2024-09-10 `IP-Adapter` | 无 | 近两年无新仓库 | 已找到 |
| A3-3 | GitHub `Tencent-Hunyuan` | L1 | `https://api.github.com/orgs/Tencent-Hunyuan/repos?sort=created&direction=desc` | 最新创建 `HyperLogic` 2026-10-27T11:05:36Z；`Hy4-preview` 创建 2026-08-27T09:27:34Z（409 星）；`api.github.com/repos/Tencent-Hunyuan/Hy4` 404 | 无 | 21:24 复查一致 | 已找到（无 Hy4 正式版） |
| A3-4 | ModelScope | L1 | https://modelscope.cn/api/v1/models/Tencent-Hunyuan/Hy4-preview （200）；`…/Hy4`（404 “获取模型失败”）、`…/Hy4-FP8`（404） | — | 无 | 另探 cnb.cool/ai-models/tencent/Hy4 404；ai.gitcode.com/tencent_hunyuan/Hy4 返回 200 但是 SPA 壳页，不能据此判断，**未验证** | 已找到（无 Hy4 正式版，GitCode 未验证） |
| A3-5 | Hy4-preview 许可证与时间 | L1 | https://huggingface.co/tencent/Hy4-preview （LICENSE 原文已存） | 模型卡头 “License: apache-2.0”；LICENSE 首行 “Tencent Hy4 preview is licensed under the Apache-2.0.” | `08-hf-hy4-preview-head.png` | HF 创建 2026-08-27T08:52:40Z（北京 16:52），lastModified 2026-08-28T14:58:32Z；下载 11,399（模型卡“last month”）、点赞 530。GitHub 许可证检测显示 NOASSERTION（LICENSE 带腾讯抬头后附 Apache 全文），不是不同许可证。官方 8/28 稿 “Hy4 preview is now available as an open-source model”：preview 是权重先放出再发稿，正式版是先发稿后放权重，次序相反 | 已找到 |
| A4-1 | 金融众测原段 | L1 | 混元 Blog 中文；tencent.com 中/英文稿；腾讯新闻版 | Blog：“我们联合腾讯金融科技 FiT 开展了 213 位专家的金融众测，其中 153 位来自外部的头部金融机构，且七成拥有五年以上工作经验。基于 579 个真实金融任务的众测显示，Hy4 综合表现位居开源模型第一（Hy4：3.04/4；DeepSeek-V4.1-Flash-1017：2.97/4；Kimi K3.1 Pro：2.68/4；GLM-5.3：2.68/4）。” 英文稿：“In a rigorous blind evaluation conducted by the Tencent Hy team in partnership with Tencent Financial Technology, Hy4 emerged as the top-performing open-source model.” | `04a-hy-blog-finance.png`、`06-tencentcom-en-finance.png`、`04-tx-news-finance.png` | 组织者：腾讯混元团队 + 腾讯金融科技（FiT），腾讯自家；满分 4；对照只有 3 个模型；Hy4 与 DeepSeek 差 0.07。题目、评分细则、盲评如何设计均未公开。腾讯新闻版只写“213 位专家”“579 个真实金融任务”，没有分数 | 已找到 |
| A4-2 | 官方对比图（12 项基准） | L1 | 腾讯新闻版与混元 Blog 同图 | 图例：Hy4、Hy4 preview、GLM 5.3、DeepSeek-V4.1-Flash-1017、Kimi K3.1 Pro、Claude Opus 5.5、GPT 6.1 Sol；脚注 “Benchmarks marked with* were only evaluated for the first time upon release.” | `02-tx-news-bench-chart.png`（整图含脚注） | 目视读数见下“对比图读数”，**需 Claude 复核**。图无 GLM 的 OSWorld 2.1 一栏。(internal) 标记的 3 项为腾讯内部基准 | 已找到 |
| A4-3 | 其他“第一”“领先” | L1 | 同上 | “WorkBuddy 综合评测：平均效果 83.18 分，通过率 96.15%，12 个场景里，Hy4 在 9 个场景位列国内模型第一。”（Blog）；TenureTrail 内部评测“位居开源模型第一”（Blog，英文 “ranks first among open-weight models”）；元宝内部评测 65.25→72.53；“幻觉率”见 A5 | 无 | 全是腾讯自测、自家产品或合作方内部评测；WorkBuddy 的“国内第一”是腾讯自己的产品 | 已找到 |
| A4-4 | AA / LMArena 有无 Hy4 | L3 | https://artificialanalysis.ai/leaderboards/models | 页面里 “Hy3-preview”“Hy3” 有，“Hy4” 0 处；`/models/hy4`、`/models/tencent-hy4` 返回 404 | 无（存 `a-aa-leaderboard.html`，仅本地） | 21:03 取；WorkBuddy 扫描“18:00 无 Hy4”复核仍成立。LMArena 被 Cloudflare 验证页拦下，未绕过，**未验证** | 已找到（AA 无 Hy4）；未找到一手来源（LMArena 被拦） |
| A5-1 | 网络进攻能力“未展现突出风险” | L1 | 腾讯新闻版；Blog；tencent.com | 腾讯新闻版：“结合内部专家审计和自动评测，我们发现 Hy4 的网络进攻能力未展现突出风险，尚未发现其能够显著降低实施生物危害行为的门槛。”；Blog：“Hy4 在加密算法的逆向求解、对抗二进制混淆、漏洞利用等进攻性能力上未展现出突出风险。”；英文稿 “The assessments found no significant concerns regarding Hy4’s offensive cybersecurity capabilities in the areas evaluated.” | `03-tx-news-responsible.png`、`07-tencentcom-en-cyber.png` | 审计方是“来自腾讯玄武和科恩实验室的内部安全专家”（均为腾讯自家实验室）；英文稿限定 “in the areas evaluated”，中文稿没有这个限定 | 已找到 |
| A5-2 | 腾讯说过过滤/限制网络安全数据 | L1 | Blog 中文；英文 Blog；tencent.com | “在 Hy4 训练阶段，我们仅纳入与 PoC 生成、漏洞修复相关的防守性网络安全数据，同时剔除漏洞利用、逆向、渗透等攻击性训练内容，力求避免潜在进攻性能力的提升。”；英文 Blog “we limited cybersecurity data to defensive material … We filtered out offensive content such as exploit development, reverse engineering, and penetration testing”；英文新闻稿却写 “The model incorporates reverse engineering and penetration testing for analysis purposes. However, offensive materials related to exploitation are explicitly excluded from the training dataset.” | `03a-hy-blog-weights-limits.png`、`07-tencentcom-en-cyber.png` | 同一件事两种口径：Blog 说“剔除……逆向、渗透”，英文新闻稿说模型“纳入逆向与渗透测试（用于分析目的）”，只排除“漏洞利用相关的攻击性材料”。腾讯新闻版没有这段。Reuters 的 “excluded material on building hacking tools, breaking into systems and testing systems for weaknesses” 对应 Blog 版 | 已找到 |
| A5-3 | 路透社 | L4 | https://www.reuters.com/legal/litigation/tencent-rolls-out-ai-model-trained-with-limited-cybersecurity-data-amid-hacking-2026-11-04/ | 标题 “Tencent rolls out AI model trained with limited cybersecurity data amid hacking fears”；“Tencent said it restricted the model's cybersecurity training to defensive uses, such as identifying software flaws and helping fix them. It excluded material on building hacking tools, breaking into systems and testing systems for weaknesses.”；“Tencent said it plans to release its model weights later along with a report describing its safety testing.” | 无（headless 被设备验证拦；opencli 打开后弹订阅框，不截） | 2026-11-04 5:34 AM EST = 10:34 UTC = 18:34 北京；Nichiket Sunil、Eduardo Baptista。curl 返回 401。正文在 opencli 首次渲染时即在页面里，随后出现订阅弹窗；未登录、未绕过，仅摘 L4 线索。路透写 “later” 无时间词 | 已找到（L4） |
| A5-4 | 参数、上下文 | L1 | 同上 | 正式版稿（腾讯新闻、tencent.com、Blog 中英）**均未写** 770B、49B、1M；这些数字在 Hy4-preview 模型卡（“770B total parameters, of which 49B are activated per token”“Context Length 1M”）与 8/28 preview 稿。财联社（科创板日报）写“Hy4系列则达到了770B”，署“据《科创板日报》记者了解” | 无 | WorkBuddy 扫描把 preview 参数写成 Hy4 正式版：正式版参数官方**未找到一手来源** | 未找到一手来源（正式版）；preview 已找到 |
| A5-5 | 视觉、电脑操作 | L1 | 腾讯新闻版 | “Hy4 首次引入视觉能力”；“Hy4 也首次支持电脑操作（Computer Use），可以结合图形界面与命令行，处理工作和生活中的各类任务。” | 无 | 官方自述，无独立评测 | 已找到 |
| A5-6 | 价格 | L1 | Blog 中文；tencent.com 英文稿；Blog 英文 | 中文：输入（命中缓存）0.3 元、输入 6 元、输出 18 元（每百万 Tokens）；tencent.com 英文稿 “approximately US$0.90 per million input tokens, US$2.70 per million output tokens and US$0.045 per million cached input tokens”；Blog 英文 “$0.042 / $0.834 / $2.501” | `03-tx-news-responsible.png`（含人民币价目表） | 三处美元数各异，同一价目表的汇率换算：0.9 与 0.834 差 8%。“性能显著超过 Hy4 preview 的同时定价维持一致”；TokenHub 至 2026-12-31 按刊例价 5 折；WorkBuddy/CodeBuddy 11/4–11/15 “一定额度的免费使用” | 已找到 |
| A5-7 | 幻觉率 | L1 | tencent.com 中英文稿 vs Blog 中英文 | tencent.com：“在自建测试中，Hy4 的幻觉率从 Preview 版的 12.95% 降至 7.2%，猜测导致的错误率从 23.1% 降至 5.6%。”；Blog：“在自建测试中，Hy4 的幻觉率从 Hy4 preview 的 15.76% 降至 5.45%，猜测导致的错误率从 23.1% 降至 5.6%。” | `04a-hy-blog-finance.png`、`06-tencentcom-en-finance.png` | **同一指标两组数，全部记录，不取舍**。自建测试，分母与测试集未公开。财联社、什么值得买引 12.95%→7.2% | 已找到 |
| A6-1 | HF `moonshotai` 无 K3.1 | L1 | https://huggingface.co/models?sort=created&author=moonshotai ；API 同前 | 最新 `moonshotai/Kimi-K3`（createdAt 2026-06-13T06:42:57Z）；其后 Kimi-K2.7-Code、K2.6、K2.5 等；页面“Models 19”。`api/models/moonshotai/Kimi-K3.1-Pro`、`…/Kimi-K3.1` 均 401 | `10-hf-moonshotai-models-recent.png` | 核查时刻：20:46（API）、21:12（页面截图）、21:24 与 21:28（API 复查，交回前最后一次）。GitHub `MoonshotAI` 最新创建 `TCSExam`（2026-11-03，与 K3.1 无关），无 K3.1 仓库；官方博客索引页最新仍是 2026-10-24 的 K3.1 Pro；官方 X 账号（经 opencli 读公开时间线）最新帖 2026-10-30 06:00 UTC | 已找到（核查时刻尚无权重） |
| A6-2 | 官方“by Nov 4” | L1 | https://www.kimi.com/en/blog/kimi-k3-1-pro | 摘要段：“Kimi K3.1 Pro and Kimi K3.1 Pro Highspeed are available today on Kimi.com, Kimi Work, Kimi Code, and the Kimi API. The full model weights will be released by Nov 4.”；Availability 节：“Full model weights will be released by Nov 4, 2026.” | `11-kimi-blog-availability.png`（Availability 节；摘要段是滚动动画页，没截出清晰图，原句见 txt） | 博客日期 2026-10-24（索引页）。**没有时刻与时区**。“by Nov 4” 若按北京 11/4 24:00 结束，则 21:24 尚有约 2.5 小时；按美东 11/4 24:00 = 北京 11/5 13:00；按太平洋 = 北京 11/5 16:00。不得写“跳票”“食言” | 已找到 |
| A6-3 | 官方 X 帖 | L1 | https://x.com/Kimi_Moonshot/status/2113680701770068067 （fxtwitter JSON 已存） | “Introducing Kimi K3.1 Pro: our most capable 3T-class open-weight model. … Open weights by Nov 4, 2026.” | 无（X 平台，若用只截帖子卡片） | 2026-10-23 17:15:24 UTC = 10-24 01:15 北京。帖里直接称 “open-weight model”，同时权重要到 11/4 才发 | 已找到 |
| A6-4 | 官方公众号 / 其他延期公告 | — | — | 未能读取 Kimi 公众号；官方 X 公开时间线 10/30 之后无新帖，未见延期或发布公告 | 无 | 只覆盖 X、博客索引、HF、GitHub；公众号与 Kimi 开放平台公告**未验证** | 未找到一手来源 |
| A6-5 | 反面样本：“今天正式开源全部权重” | L5 | https://post.m.smzdm.com/p/awwde3w2 （桌面版 https://post.smzdm.com/p/awwde3w2/） | 小标题“月之暗面 Kimi K3.1 Pro 今天正式开源全部权重”；正文“11 月 4 日按计划开源全部权重”；出处“汇总 今日头条《AI 每日新闻简报》（据月之暗面官方公告）” | `12-smzdm-kimi-today.png` | 什么值得买用户投稿，标注时间 2026-11-04 08:56:35（站点时间，应为北京）；标题把“计划”写成“今天已开源”，写稿时与 21:24 都没有权重。作者账号名未入图 | 已找到 |
| A7-1 | 澎湃 | L4 | https://www.thepaper.cn/newsDetail_forward_34403267 | 标题“国产大模型再上新：混元Hy4正式发布，腾讯称首次进入最强开源模型梯队”；“Hy4将于近期开源模型权重，发布模型系统卡片，并积极推进更负责任的开源，重点关注网络和生物安全等风险。” | 无 | 2026-11-04 14:27，记者范佳来；转述腾讯，写“腾讯称” | 已找到 |
| A7-2 | 网易智能 | L4 | https://www.163.com/tech/article/LAT4MDOJ00098IEO.html | 标题“腾讯混元发布 Hy4，首次进入最强开源模型梯队”；同上句 | 无 | 2026-11-04 13:25:31 | 已找到 |
| A7-3 | IT之家 | L4 | https://www.ithome.com/1/022/035.htm | 标题“腾讯混元 Hy4 正式版发布，号称首次跻身最强开源模型之列、为真实生产力而生”；“官方表示，随着模型能力的再次全面跃升，Hy 首次跻身最强开源模型之列。Hy4 将于近期开源模型权重，发布模型系统卡片……”；“官方计划开源模型权重，并已联合 vLLM/SGLang 社区以及 Inferact Inc. 提供开源生态支持。” | 无 | 2026/11/4 12:59:42，距腾讯新闻版 12:52 仅 7 分钟；标题用“号称” | 已找到 |
| A7-4 | 21 财经（转财联社/科创板日报） | L4 | https://m.21jingji.com/article/20261104/herald/84ac28807e27df0ba14b1f2cb2a9274e.html | 标题“腾讯史上最强开源模型，来了”；“开源方面，Hy4 将于近期开源模型权重，发布模型系统卡片……”；“《科创板日报》记者也注意到，Hy4 在代码编辑、自动化、电脑操作等多项基准测试中，实现了对DeepSeek-V4.1-Flash等模型的超越，但距离GPT-6.1 Sol、Claude Opus 5.5等模型仍有一定差距。” | 无 | 2026-11-04 19:56；标题读起来像已发生，正文是“将于近期” | 已找到 |
| A7-5 | 新京报 | L4 | https://www.bjnews.com.cn/detail/1793776487129942.html | 标题“腾讯混元发布Hy4，首次引入视觉理解能力”；全文未提“开源”“权重” | 无 | 2026-11-04 15:59，记者韦英姿 | 已找到 |
| A7-6 | 把 Hy4 写成“已开源”的稿子 | — | — | 在抽查的 5 篇与搜狗微信 10 条摘要里**没有找到**写“权重已发布”的 Hy4 稿；所有转述都带将来时。仅有标题层面容易误读：“腾讯史上最强开源模型，来了”（21 财经）、“腾讯混元Hy4正式发布!首次跻身最强开源模型行列”（微信转述，摘要写“权重将于近期开源”） | 无 | 搜狗微信摘要之一写“随正式版发布的还有系统卡片，提到网络与生物安全风险”（第 10 条，正文未取得，微信验证码拦截），若属实则是写错：系统卡片官方说的是将来发布。仅作线索 | 未找到一手来源（无“已开源”实例，仅标题级夸大） |
| A7-7 | 热度 | L5 | HN 50362291 https://news.ycombinator.com/item?id=50362291 | 提交标题“Tencent Hy4”，指向 hy.tencent.ai 英文 Blog | 无 | 7 分、2 评；创建 2026-11-04T07:05Z = 15:05 北京；两条评论：一条推测多模态投入是否对标 Opus/Astra 或蒸馏 Astra（无依据，臆测），一条补充 WorkBuddy 在中国流行。HN 搜索 “Kimi K3.1 Pro” 自 10/31 起 0 条。Reddit 搜索被登录墙拦（403/login），**未取得** | 已找到 |

## A2 各版本对“权重/系统卡”的措辞对照

| 版本 | 权重 | 系统卡 | 时间词 |
|---|---|---|---|
| 腾讯新闻版（12:52，腾讯官方账号） | **无此句** | 无 | — |
| tencent.com 简/繁中文稿（日期 11月4日） | “Hy4 将于近期开源模型权重” | “发布模型系统卡片” | 近期 |
| tencent.com 英文稿（Nov 4） | “plans to release model weights” | “and system cards” | “in the coming days” |
| 混元 Blog 中文（11月4日；列表页 11-03） | “我们计划开源模型权重，并已联合 vLLM/SGLang 社区与 Inferact Inc. 提供开源生态支持” | “我们届时也会发布模型系统卡片，包括更全面的安全评估等细节” | 无 |
| 混元 Blog 英文（Nov 4） | “We plan to open-source Hy4’s model weights” | “When we release the weights, we will also publish a comprehensive system card” | 无 |
| 路透（18:34）转述 | “plans to release its model weights later” | “a report describing its safety testing” | later |

## 对比图读数（目视，供 Claude 复核；图见 `02-tx-news-bench-chart.png`）

Hy4 / Kimi K3.1 Pro / Claude Opus 5.5 / GPT 6.1 Sol（GLM 与 DeepSeek 略）：

| 基准 | Hy4 | Kimi K3.1 Pro | Claude Opus 5.5 | GPT 6.1 Sol |
|---|---|---|---|---|
| ProgramBench* | 30.0 | 24.5 | 56.5 | 46.0 |
| FrontierCode 2.0* | 31.0 | 25.5 | 46.4 | 40.6 |
| 3DArena-Modeling (internal) | 37.1 | 36.1 | 43.4 | 41.8 |
| Terminal Bench 4.0 | 44.4 | 66.8 | 66.4 | 55.5 |
| Agents' Last Exam v1.1* | 41.0 | 43.4 | 43.4 | 40.2 |
| AutomationBench v1.0.6 | 56.2 | 65.6 | 58.8 | 51.4 |
| E-Bench 2.0 (internal) | 70.0 | 50.0 | 80.0 | 65.3 |
| Hy-BrowseComp 2.0 (internal) | 59.9 | 59.3 | 60.7 | 61.7 |
| PostTrainBench v1.2* | 35.2 | 36.2 | 44.2 | 38.2 |
| Terminal Bench Science 0.1 | 19.8 | 41.7 | 56.2 | 60.9 |
| TenureTrail (internal)* | 53.6 | 51.3 | 59.0 | 57.6 |
| OSWorld 2.1 | 78.9 | 79.6 | 81.8 | 75.2 |

读数结果：Hy4 低于 Claude Opus 5.5 的 12/12；高于 Kimi K3.1 Pro 的 6/12（ProgramBench、FrontierCode、3DArena、E-Bench、BrowseComp、TenureTrail），低于的 6/12；高于 GPT 6.1 Sol 的 4/12（Agents' Last Exam、AutomationBench、E-Bench、OSWorld）。图上带 (internal) 与 * 的几项是腾讯内部或首次评测。

## 可能的吠点

1. 标题与导语说“跻身最强开源模型之列”，权重尚未放出；官方四个版本对放出时间的说法各不相同（近期 / in the coming days / 无）。Blog 英文却同时写 Hy4 “ranks first among open-weight models”（金融众测与 TenureTrail 两处）——“open-weight 第一”而权重还没发。出处：A1、A2 表。
2. “开源模型第一”的依据只有腾讯自己与腾讯金融科技组织的众测：满分 4，对照 3 个模型，Hy4 3.04 对 DeepSeek-V4.1-Flash-1017 2.97；评分细则与测试集没公开。出处：Blog 中文“金融与办公场景”节。
3. 官方自己的对比图里，Hy4 12 项全部低于 Claude Opus 5.5、与 Kimi K3.1 Pro 6 比 6；Blog“局限性和未来展望”写“与顶尖闭源模型仍然存在较大差距”，腾讯新闻版没有这一节。
4. “未展现突出风险”是腾讯玄武、科恩两家自家实验室的内部审计；英文稿限定 “in the areas evaluated”，中文版无此限定。出处：Blog、tencent.com 英文稿。
5. 官方稿之间互相打架：幻觉率 12.95%→7.2% 与 15.76%→5.45%；网络安全数据处理，Blog 说剔除了逆向、渗透内容，英文新闻稿说模型纳入了逆向与渗透测试（用于分析）；美元价格三套数字；Hy4 Flash 小标题写 “Recently Released”，正文却是 “scheduled for release soon”。
6. 参数 770B/49B 与 1M 上下文是 Hy4 preview 的数据，Hy4 正式版官方稿未写；财联社写 770B 是“据科创板日报记者了解”。
7. 腾讯新闻版把“将于近期开源模型权重、发布模型系统卡片”整句删掉，只留“更多细节可点击阅读原文”；读者只看该版会以为“最强开源”已成事实。
8. 路透标题用 “trained with limited cybersecurity data”，官方原话是“仅纳入防守性网络安全数据”并说“剔除”攻击性内容；两种措辞、两种语气，不是同一个说法。
9. HN 社区：7 分 2 评，英文侧基本无人讨论，没有有理有据的质疑可摘（仅一条臆测性的“是否蒸馏自 Astra”，无依据）。

## 夸大说法实例

- 什么值得买用户稿，标题“月之暗面 Kimi K3.1 Pro 今天正式开源全部权重”，2026-11-04 08:56:35，发布方为站内用户投稿（汇总自今日头条简报）；正文“11 月 4 日按计划开源全部权重”。https://post.m.smzdm.com/p/awwde3w2 。21:24 时 HF 仍无该权重。截图 `12-smzdm-kimi-today.png`。
- 21 财经转财联社稿，标题“腾讯史上最强开源模型，来了”，2026-11-04 19:56；正文“Hy4 将于近期开源模型权重”。仅标题级，未写权重已发布。
- 把 Hy4 写成“权重已发布”的稿子：**没有找到**。

## 扫描说法勘误

1. WorkBuddy 扫描“11/4 14:06 起经‘腾讯技术工程’、腾讯新闻发布”：与存档不符。腾讯新闻版 12:52；网易智能转载 13:25:31；IT之家 12:59:42。
2. 两份扫描称“官方稿写了‘将于近期开源模型权重、发布模型系统卡片’”：成立，但只在腾讯官网中文稿、通稿转述与英文稿（措辞为 in the coming days）里；腾讯新闻版没有；Blog 无时间词。Claude 在腾讯新闻版没找到是对的。
3. WorkBuddy 扫描“770B 总参数 / 49B 激活、1M 上下文”挂在 Hy4 正式版名下：官方正式版稿没写，这些是 Hy4 preview 的数据。
4. 价格“输入 6 元、输出 18 元、缓存命中 0.3 元 / 百万 token”：与官方一致。
5. 众测分数“Hy4 3.04 / Kimi 2.68 / GLM 2.68 / DeepSeek 2.97”：与官方一致，满分 4（Blog 写明 /4），组织者是腾讯混元团队与腾讯金融科技（FiT）。
6. “AA 截至 18:00 无 Hy4 分数”：21:03 复核仍无（AA 有 Hy3、Hy3-preview）。LMArena 被验证页拦，未验证。
7. Claude 的 HF 现查结论成立：`tencent` 无 Hy4 正式版；`moonshotai` 无 K3.1 Pro（21:24 复核）。
8. 什么值得买“今天正式开源全部权重”：存档确认，见上。
9. HN 50362291 为 7 分 2 评：成立。
10. Kimi 官方博客原句：页面里有两处。摘要段 “The full model weights will be released by Nov 4.”（无年份），Availability 节 “Full model weights will be released by Nov 4, 2026.”（有年份）。派工单写成带年份的整句，只对应后一处。


## Claude 勘误（2026-11-04）

- A4-2 备注“(internal) 标记的 3 项”有误：Claude 看图核对为 4 项（3DArena-Modeling、E-Bench 2.0、Hy-BrowseComp 2.0、TenureTrail），以本条与 fact-check.md 为准（反向核验第2轮第6条）。
