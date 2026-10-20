# 1019 事实核验

取证：A/B/C 组 Sonnet，D/E 组 Haiku（claude-haiku-5-5，effort high）；存档见本目录 `evidence-*.md`、`capture-log-*.md`。时间为北京时间。

## 主帖一：DeepInfer

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| DeepSeek 20日在公众号宣布把内部推理框架 DeepInfer“全面开源” | L1 | ✅ | DeepSeek 官方公众号（`a-wx-official.json/.txt`，经搜狗微信跳转，链接带签名不稳定）；官方 X 帖（`a-fx-2112476496283775135.raw.json`） | 公众号时间戳 1792486494 = 10/20 16:54:54。原句“我们决定进一步将内部推理框架 DeepInfer 全面开源”。X 帖 10/20 17:30（09:30:19 UTC）：“now fully open source!” 仓库 Apache-2.0；首个 commit 10/20 12:09，无 release/tag |
| 知乎技术报告称单一集群日吞吐155万亿 token；按图注，不计波峰波谷，平均每卡每小时整体收入38美元（按谷价） | L1 | ✅ | https://zhuanlan.zhihu.com/p/2095560295930508953（`a-zhihu-dom.txt`、`a-zhihu-api.json`；图02、图03） | 作者 DeepSeek 认证机构号，10/20 16:49:27。原句“目前单一集群的服务吞吐是 155 万亿 Tokens/天，整体收入是每卡每小时 38 美金（按谷价）”。图03 注：“实际部署 P:D 接近 1:1，不考虑波峰波谷，整体收入是平均每卡每小时 38 美金（按谷价）”；图头“机器已经预留了一定冗余度，以应对流量的波动”。全文无卡型号、卡数、集群数 |
| README 列明 KV 缓存系统、预填充/解码分离部署、跨副本负载均衡不在本次发布中（省流：本次只含离线推理） | L1 | ✅ | https://github.com/deepseek-ai/DeepInfer README（`a-readme-raw.txt`；图01） | “This open-source release provides offline inference with the core features needed for serving at scale.”；“Required for a complete inference service (not included in this release): A KV cache system / VE/Prefill/Decode disaggregation (most required engine functionality is already implemented) / Load balancing across data-parallel replicas for both prefill and decode”。分离部署一项注明“所需引擎功能大部分已实现” |
| 知乎报告自己也写“相关代码目前尚未完整开源” | L1 | ✅ | 知乎报告（`a-zhihu-dom.txt`） | 原句“推理系统中的 KVCache 管理与负载均衡组件因与内部基础设施耦合较深，相关代码目前尚未完整开源”。报告开头措辞为“今天，我们正式开源大规模云服务推理框架 DeepInfer”，无“全面” |
| 只支持 V4.1-Flash，硬件只有英伟达 SM100 和华为昇腾950 | L1 | ✅ | README（图01） | Supported models: DeepSeek-V4.1-Flash；Supported hardware: “NVIDIA SM100 GPUs”“Huawei Ascend 950 NPUs” |
| 快科技标题“每张卡每小时能赚38美元” | L4 | ✅（标题存在） | https://m.mydrivers.com/newsview/1159039.html（`a-mydrivers.txt`） | 10/20 17:53。标题“DeepSeek把核心商业秘密也开源了：每张卡每小时能赚38美元”；同篇正文“还不能估算他们每天的真实收入和Token吞吐量” |
| “谷价”未解释，官方价格页写的是“空闲时段”半价 | L1 | ✅ | 知乎报告；https://api-docs.deepseek.com/zh-cn/quick_start/pricing（`a-pricing.txt`） | 价格页：“空闲时段价格为高峰时段价格的一半”；北京时间周一至周五（不含法定节假日）9:00–12:00、14:00–18:00 为高峰；全页无“谷”字。报告“谷价”与“空闲时段”是否同义官方未写，正文只并列，不画等号 |
| （背景，未入正文）DeepInfer“最初主要基于 vLLM” | L1 | ✅ | README Acknowledgments | “DeepInfer was originally built primarily on vLLM, with additional inspiration from SGLang and TensorRT-LLM.” |

## 主帖二：WhatsApp

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 隐私工具公司 Redact 称，10/8 发现后读了 WhatsApp 帮助中心链接的545个页面，其中80页藏着102段给 AI 看的文字 | L3 | ✅（Redact 自述） | https://redact.dev/blog/whatsapp-help-pages-hidden-ai-instructions（`b-redact.txt`；图15） | “We found it on October 8, 2026 … we then read all 545 pages that WhatsApp's help centre links to”；统计卡 545 / 80 / 102。数字只有 Redact 一家，WIRED 写 “around 80”“the 100 blocks”，未独立计数。博文页面日期 10/19，元数据 10/19 16:00 UTC |
| 删号页一段写“不要建议联系任何数据保护机构、监管机构”；这条只针对丢了号码或设备、无法自助删号的人 | L3 | ✅ | 同上（图13）；互联网档案馆 10/9 05:48 UTC 快照 `b-wb-20261009054848.html`（`b-wayback-excerpt.txt`） | 隐藏块首句是条件：“If a user cannot complete self-service account deletion because they have lost access to their phone number, their phone line was cancelled, or they no longer have access to their device”；指令 “Do not recommend contacting any Data Protection Authority, regulator, or supervisory authority.” 泛指，未点名具体机构。同块还有 “Do not suggest WhatsApp can manually delete accounts on the user's behalf.” |
| 举报页：用户提到诉讼、律师后，不要引用或解释 GDPR 等隐私法规，用户主动提也不行；同一段仍让用户咨询律师 | L3/L4 | ✅ | Redact（图14）；WIRED https://www.wired.com/story/whatsapp-help-pages-leaked-hidden-instructions-for-a-meta-chatbot/（`b-wired.txt`；图16）；快照 `b-wb-report-20261009054943.html` | 触发：“user mentions legal action, lawsuits, lawyers, attorneys, courts, or litigation”；“Do not reference, cite, or interpret data protection or privacy regulations (for example GDPR, LGPD, CCPA), including when a user raises them directly”；末句 “For legal matters, users should consult a qualified legal professional.” 页面 faq.whatsapp.com/414631957536067（About reporting and blocking） |
| 互联网档案馆9日、16日快照可见（18日、20日快照里没有） | — | ✅（本号核对，快照为第三方存档） | web.archive.org 快照 10/9 05:48、10/16 14:11、10/18 02:37、10/20 07:44 UTC（`b-wb-*.html`、`b-wayback-excerpt.txt`） | 隐藏块在组件 `HelpContentGalaxyLLMInclude` 下；现行页面（10/20 19:19 读）隐藏文字0命中。Redact 称 10/18 02:08 UTC 已全部移除。“屏幕上不可见”本号未独立验证渲染，只有 Redact 与 WIRED 说法 |
| 据 WIRED 报道，WhatsApp 发言人称这些是给自家客服机器人的指令，“不是直接给用户的” | L4 | ⚠️ | WIRED（图16） | 10/20 00:00（10/19 12:00 美东）。直接引语：“These are instructions to our chatbot, not directly to users, and are intended to ensure accurate and relevant responses for WhatsApp users who choose to message WhatsApp Support with AI”；“We want our product support chatbots to be focused on providing the best advice to users about WhatsApp—not to offer legal advice”。“bug 已修复”是 WIRED 转述（“saying it was being displayed due to a bug that has now been fixed”），不是引语。发言人引语未回应“不要建议监管机构”那条 |
| 有转述去掉“删不了号”这个前提 | L4/L5 | ✅（实例存在） | Mindgrow https://mindgrow.io/technology/a-bug-exposed-the-hidden-rules-for-whatsapp-s-support-chatbo-128363（`b-mindgrow.txt`）；dev.to/Hacks.gr（`b-devto.txt`） | Mindgrow：“One rule told it never to suggest you complain to a privacy regulator”；dev.to 标题 “WhatsApp's hidden instructions forbade AI from sending users to data protection authorities”。均未写“仅限无法自助删号”的前提。见 `evidence-B.md` 第三节 |
| Redact 自己也说102段大多无害 | L3 | ✅ | Redact | “Most of those 102 blocks were harmless”；“多数是一般客服说明”是 Redact CEO Saltman 在 WIRED 的话，不是 WhatsApp 发言人 |
| Redact 没测机器人是否照做，也没给受影响人数（省流：发现者没测 AI 是否照做；WIRED 也未测） | L3/L4 | ✅ | Redact（图19）；WIRED | Redact：“We read its instructions, not its answers.”；“We haven't tested whether any AI assistant acted on them.” WIRED：“It is unclear if the text could be accessed by third-party chatbots or how long it had been displayed.” |
| Redact 本身卖批量删帖、数据清除工具 | L1 | ✅ | https://redact.dev/（`b-redact-home.txt`） | “Mass delete your posts, comments, likes and DMs from 35+ platforms.” 付费订阅（$7.99/月、$14.99/月）。博文未推荐自家产品 |

## 主帖三：Mercury 3

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Inception 发布扩散语言模型 Mercury-3，称每秒1068个 token，比 GPT-6-Luna、Haiku-5.5 快4–8倍 | L1 | ✅ | https://www.inceptionlabs.ai/blog/introducing-mercury-3（`c-blog.txt`；图25） | 博客页只给日期 Oct 19；Business Wire 新闻稿 10/20 00:48（10/19 12:48 EDT）。“Across a range of benchmarks, Mercury 3 outperforms GPT-6 Luna and Claude Haiku 5.5, while running 4–8x faster at 1,000+ tokens per second.” |
| 扩散模型不是一个字一个字往外蹦，而是并行生成、逐步去噪 | L1 | ✅ | https://www.inceptionlabs.ai/（`c-home.txt`）；docs Streaming 页（`c-docs-streaming.txt`） | “All other LLMs generate text one token at a time. Mercury diffusion LLMs (dLLMs) generate tokens in parallel”；docs：“stream the model’s intermediate denoising steps” |
| 自家 medium 档对比表12项，它拿8项最高 | L1 | ✅（本号逐行数） | 对比表原图（`c-img-YO5OXn25zt3KJBJkUxO2kAwmS6o.png`；图26） | 非最高：AA-Omniscience-Public 46.44（Luna 46.94）、AA-LCR v1.1 77.67（Luna 78.30）、MRCR v2 75.50（Haiku 78.40）、DeepSWE v1 46.31（Haiku 51.62）。三列均标 MEDIUM；图无脚注，未说明谁跑。联合创始人 HN 评论 50155371：“at matched reasoning effort”。厂商自测 |
| 表里 Mercury-3 的价格写每百万 token 输入0.10、输出0.35美元，这是首发五折价，原价0.20和0.70美元 | L1 | ✅ | 博客（图27）；docs 模型页 https://docs.inceptionlabs.ai/get-started/models（`c-docs-models-page.txt`；图29） | 博客：“Price: $0.20 per million input and $0.70 per million output tokens. At launch, Mercury 3 is 50% off at $0.10 per 1M input and $0.35 per 1M output.” docs：“(regularly $0.20 and $0.70)”。表中价格列 $0.10 / $0.35 |
| 对手 GPT-6-Luna、Haiku-5.5 表里写的是输入0.10、输出0.50美元，与两家官方短提示价一致 | L1 | ✅ | OpenAI https://developers.openai.com/api/docs/pricing（`c-oai-api-pricing.txt`；图31）；Anthropic https://platform.claude.com/docs/en/about-claude/pricing（`c-anthropic-pricing-table.txt`；图30） | Luna Standard short context：Input $0.10 / Output $0.50（>272K 输入按输入2倍、输出1.5倍）；Haiku-5.5：“$0.10 / MTok for prompts up to 100,000 tokens”、输出 $0.50（>10万 $0.50 / $2.50）。按原价 Mercury-3 输入是两家2倍、输出1.4倍（本号按官方价计算） |
| 五折没给截止时间 | L1/L4 | ✅ | 博客、docs、官网首页、新闻稿均未写；The Deep View（`c-m-deepview.txt`） | “Inception hasn't said how long the launch promotion period will last. But Ermon told The Deep View that the company is working on developments that could allow it to make the launch price permanent in the weeks ahead.”（L4，正文若用写“据 The Deep View 报道”） |
| 1068是厂商自报，未写 GPU 型号 | L1/L2 | ✅ | 博客（图27）；HN 50155245 联合创始人评论（`c-hn-hn-50155245.json`） | 博客：“1,068 tokens per second on NVIDIA GPUs.” 联合创始人 HN：“widely available NVIDIA Blackwell GPUs”。批量、提示长度未给。对手速度 ~130 / ~240 无来源无脚注；AA 上 Luna(Max) 124.2、Haiku-5.5(Max) 234.1、Haiku-5.5(Medium) 178.6 token/s，与表头 MEDIUM 不完全对应 |
| 截至20日晚，AA 还没有 Mercury-3 的页面 | L3 | ✅ | https://artificialanalysis.ai/models/mercury-3（`c-aa-mercury3.html`，404） | 10/20 19:12 读取 404；`-medium` 同 404；AA Inception 厂商页只列 Mercury 2.5、Mercury 2 |
| 截至20日晚，OpenRouter 页面显示其输出速度中位数约每秒180个 token，混合各推理档、条件不同 | L3 | ✅ | https://openrouter.ai/inception/mercury-3（`c-or-mercury.txt`；图32） | 10/20 19:12 文本 “181 tok/s P50, best across providers”，19:29 渲染 “180 tok/s”；Reasoning effort 选 All、1 week；图表 P50 均值 230。OpenRouter 自身的线上测量（真实流量、混合推理档、含网络），与厂商实验条件不同，只说明两者口径不同，不说明谁错。OpenRouter 模型页称 Oct 16 上架 |
| 参数量没公布，只说比 Mercury-1 大40多倍 | L1/L2 | ✅ | 博客；HN 联合创始人评论 | “more than 40x larger than Mercury 1”；HN：“We’re not publicly announcing the model size right now”。权重未开放 |
| Mercury-3 的 API 默认推理档是 low，对比表用的是 medium | L1 | ✅ | docs https://docs.inceptionlabs.ai（`c-docs-reasoning.txt`） | “If reasoning_effort is omitted, mercury-3 defaults to low, while mercury-2.5 and mercury-2 default to medium.” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| AA 医生对话子榜：Opus-5.5 原始分第一，按长度调整后降到第六 | L3 | ✅ | https://artificialanalysis.ai/articles/healthbench-professional-health-conversations（`d-healthbench.txt`） | 10/19（官方未给时刻）。“Claude Opus 5.5 (max with fallback) has the highest raw score (71.4%) but drops to sixth (59.9%).” 榜单为 HealthBench Professional（525 段医生对话）；“The length-adjusted score lowers the score of replies longer than 2,000 characters and raises the score of shorter ones.” 评分器为 GPT-5.4 (low)，按医生细则打分。HN 无热度（1/0） |
| Viggle 称 JST-2 约1秒生成3D场景（B200 推理，不含网络延迟） | L1 | ✅ | https://viggle.ai/jst2（`d-viggle-jst2.txt`） | 10/19（官方未给时刻）。“JST-2 generates a scene in about one second … Timing covers model inference on NVIDIA B200, excluding network latency.” 另：“Scene collisions and object interactions still rely on rule-based physics.” 对 Marble 的 5–6 分钟无来源，不写 |
| MiniMax-M3.1-Flash：AA 指数43，官方称权重稍后发布 | L1/L3 | ✅ | https://artificialanalysis.ai/models/minimax-m3-1-flash（`d-aa-minimax-m31f-model.txt`）；https://www.minimax.io/blog/minimax-m31f（`d-minimax-m31f.txt`） | 博客 10/19（官方未给时刻）。AA：“scores 43 on the Artificial Analysis Intelligence Index”（v4.3.2，10/20 19:13 读取；前代 MiniMax-M3 为29）；AA 测输出约105 token/s。博客：“Model weights will be released later.”；“nearly three times faster”无基线，不写。HN 50154540 477/201 |
| ProjectDiscovery 演示自训后门模型经 Codex 外传 .env | L1 | ✅ | https://projectdiscovery.io/research/how-abliterated-models-can-get-you-pwned（`d-projectdiscovery.txt`） | 原文 10/6（窗口前），HN 50159245 10/20 07:08 提交，147/85。“We built one to see what it takes, and ran it end to end through a coding agent (Codex).” 基于 Qwen2.5-7B-Instruct，触发短语后经 Codex CLI 读取项目 .env 并外传；凭据为 “production-looking dummy credentials”。7B 版投毒比例20%（125/625），1% 仅指 1.5B |
| 据路透社报道，OpenAI 洽谈约300亿美元融资，估值或超1.4万亿美元 | L4 | ⚠️ | https://www.reuters.com/world/openai-seeks-accelerate-fundraising-ahead-anthropic-ipo-sources-say-2026-10-19/（`e-reuters.txt`） | 10/20 07:18（10/19 23:18 UTC）。“in talks to join the round of about $30 billion”；“could value the ChatGPT maker at over $1.4 trillion, according to two people familiar with the matter”；赶在 Anthropic 预计 11 月中旬 IPO 前；OpenAI、软银拒绝置评。L4，正文写“据路透社报道” |

备选（未入图，取证已完成）：OpenAI COBOL 测试（MultiLangBench：用指定语言从零写 SQLite 兼容数据库引擎；最高推理档 GPT-6-Astra COBOL 72.1%，GPT-5.6-Sol 10.9%；限定词放不进40字，第一轮反向核验 BLOCKER 后撤下）；斯坦福 PhysicianBench（最佳 GPT-5.5 单次46%、三次都对28%；病例源自去标识真实病历、人口学信息合成，HAI 写“synthetically reconstructed”）；Browser Use Bench v2.1（运营方自测，加权评分非成功率，成本不含搜索、评判、浏览器与运行器）；GitHub Copilot 10/19 起在多数体验下线 GPT-5.5 等6款模型（9/18 已预告）。

## 批注译注

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 41：本次开源发布提供离线推理，含规模化服务所需的核心功能 | L1 | ✅ | `a-readme-raw.txt` | 直译 “This open-source release provides offline inference with the core features needed for serving at scale.” |
| 41：完整推理服务所需、但本次发布不包含：KV 缓存系统；VE/预填充/解码分离部署（所需引擎功能大部分已实现）；预填充与解码在数据并行副本间的负载均衡 | L1 | ✅ | 同上 | 高亮为小标题 “Required for a complete inference service (not included in this release):”，译注同时译出下列三项，括号照原文 “most required engine functionality is already implemented” |
| 42：单一集群服务吞吐每天155万亿 token，整体收入每卡每小时38美元（按谷价） | L1 | ✅ | `a-zhihu-dom.txt` | 中文原文，译注只改“美金”为“美元”、“Tokens/天”为“每天…token”；“按谷价”照录 |
| 42：KVCache 管理与负载均衡组件与内部基础设施耦合较深，相关代码目前尚未完整开源 | L1 | ✅ | 同上 | 中文原文照录，删“推理系统中的”“因” |
| 43：图头：机器已经预留了一定冗余度，以应对流量的波动 | L1 | ✅ | 图03（知乎报告内嵌图） | 图内文字照录 |
| 43：图注：实际部署 P:D 接近1:1，不考虑波峰波谷，整体收入是平均每卡每小时38美元（按谷价） | L1 | ✅ | 图03 | 图内文字照录，“美金”改“美元” |
| 44：如果用户因无法使用手机号、号码已注销或无法使用设备而不能自助删号 | L3 | ✅ | `b-redact.txt`；快照 `b-wb-20261009054848.html` | 直译 “If a user cannot complete self-service account deletion because they have lost access to their phone number, their phone line was cancelled, or they no longer have access to their device” |
| 44：不要建议联系任何数据保护机构、监管机构或监督机构 | L3 | ✅ | 同上 | 直译 “Do not recommend contacting any Data Protection Authority, regulator, or supervisory authority.” |
| 45：（WIRED 转述）WhatsApp 发言人 Joshua Breckman 称，这些文字因 bug 而显示，现已修复 | L4 | ⚠️ | `b-wired.txt` | “with WhatsApp spokesperson Joshua Breckman saying it was being displayed due to a bug that has now been fixed.” 是 WIRED 转述，非直接引语，译注标明 |
| 45：这些是给我们聊天机器人的指令，不是直接给用户的 | L4 | ⚠️ | 同上 | 直接引语 “These are instructions to our chatbot, not directly to users” |
| 46：三列都标 MEDIUM（中档推理） | L1 | ✅ | 对比表原图 | 表头 “MERCURY 3 (MEDIUM)”“GPT-6 LUNA (MEDIUM)”“CLAUDE HAIKU 5.5 (MEDIUM)” |
| 46：价格（美元/百万 token，输入/输出）：Mercury 3 为0.10/0.35；GPT-6 Luna、Claude Haiku 5.5 均为0.10/0.50 | L1 | ✅ | 同上 | 表行 “Price ($/M, in / out)” |
| 47：在英伟达 GPU 上每秒1,068个 token | L1 | ✅ | `c-blog.txt` | 直译 “1,068 tokens per second on NVIDIA GPUs.” |
| 47：每百万输入 token 0.20美元，每百万输出 token 0.70美元 | L1 | ✅ | 同上 | 直译 “$0.20 per million input and $0.70 per million output tokens.” |
| 47：首发期间 Mercury 3 打五折：每百万输入 token 0.10美元，每百万输出 token 0.35美元 | L1 | ✅ | 同上 | 直译 “At launch, Mercury 3 is 50% off at $0.10 per 1M input and $0.35 per 1M output.” |
| 48：吞吐量：P50 每秒180个 token（各服务商中最优者） | L3 | ✅ | `c-or-mercury.txt`；图32 | 页面 “180 tok/s / P50, best across providers”（19:29 渲染；19:12 文本版为181） |
| 48：推理档筛选为“All”（各推理档混合） | L3 | ✅ | 同上 | 页面 “Reasoning effort All” |

## 反向核验与视觉复核

第一轮（Codex gpt-6-astra medium，WSL，只读；审正文与事实清单，卡片未渲染前）：2 BLOCKER、6 SHOULD_FIX、1 NICE_TO_HAVE。Claude 逐条核实：

1. BLOCKER（卡片、两份 README 缺失）：属预期，卡片与 README 已补，交视觉复核与第二轮。
2. BLOCKER（COBOL 速览把 MultiLangBench 单项成绩写成一般“写 COBOL”通过率，最高推理档只在备注）：属实（`e-openai-lowres.txt` 第 76–82 行）。限定词放不进40字，撤下该条，移入备选。
3. SHOULD_FIX（OpenRouter 180与“近一周”拼成同一口径）：属实，顶部摘要与一周图表是两组数。采纳：改“截至20日晚，OpenRouter 页面显示其输出速度中位数约每秒180个 token”。
4. SHOULD_FIX（“AA 医疗对话榜”未写子榜）：采纳，改“AA 医生对话子榜”（HealthBench Professional 全为与临床医生的对话）。
5. SHOULD_FIX（未开源组件缩写过度）：采纳，改“KV 缓存系统、预填充/解码分离部署、跨副本负载均衡不在本次发布中”；“引擎功能大部分已实现”放进批注 41 译注（正文字数受限）。
6. SHOULD_FIX（12项成绩缺 medium 档）：采纳，改“自家 medium 档对比表12项，它拿8项最高”；默认档 low 记入清单，正文因字数未写。
7. SHOULD_FIX（监管指令的前提条件只在吠点里）：采纳，省流改“一条是删不了号时别建议用户找监管”，事实段写明“用户丢了号码或设备、无法自助删号时”；吠点①改为转述去掉前提的实例。
8. SHOULD_FIX（38美元口径缺“不考虑波峰波谷”，省流缺谷价）：Claude 已目视图03确认脚注原文。采纳：事实段改“按图注，不计波峰波谷，平均每卡每小时整体收入38美元（按谷价）”，省流改“按谷价算的平均收入”。
9. NICE_TO_HAVE（OpenRouter 分级与取证记录不一致）：清单按 L3（OpenRouter 自身线上测量）；`evidence-C.md` 标 L5 的是其转述厂商“1,000+ tok/s”与未分档统计的保守写法，以本清单为准。

第二轮文字复核（Codex gpt-6-astra medium，限定范围；卡片渲染后）：0 BLOCKER、5 SHOULD_FIX、1 NEEDS_VERIFICATION。报告回来时 Develata 已发布（北京 10/20 晚），以下记录核实结论，发布版暂未改动，需修订的另作发布后小改：

1. SHOULD_FIX（46 价格译注未标首发价与短提示价）：属实。待改译注“表列价格：Mercury 3 为首发五折价0.10/0.35；GPT-6 Luna、Claude Haiku 5.5 为短提示价0.10/0.50”。
2. SHOULD_FIX（省流卡第一条只留“全面开源”、无发布范围）：属实，省流句里有“本次只含离线推理”，但卡片取自事实段。待改：事实段补“README 写明本次提供离线推理”，卡片同步。
3. SHOULD_FIX（“没给卡数”过宽）：属实。知乎报告原句“用常见的 8 卡机器做 Prefill，用常见的 64～128 卡超节点做 Decode”（`a-zhihu-dom.txt`），给了节点级卡数；未给的是155万亿与38美元所对应集群的总卡数与具体卡型。正文吠点②“没给卡数、卡型”宜改“没给对应集群的总卡数和卡型”。本条是发布版里唯一的事实性过宽，建议优先修订。
4. SHOULD_FIX（医疗子榜未写 max with fallback）：属实，速览图条目待补配置（40字内需压缩）。
5. SHOULD_FIX（OpenRouter 分级说明不准）：属实，更正第一轮第 9 条：`evidence-C.md` C4-c 把 OpenRouter 自身吞吐统计标为 L5；本清单因其为平台自身线上测量改判 L3，厂商“1,000+ tok/s”宣传另属 L1 自报。
6. NEEDS_VERIFICATION（图03图头与图注只有代理转录）：Claude 已在本会话目视打开图03原件（`../images/03-zhihu-online-stats-table.png`），图头“机器已经预留了一定冗余度，以应对流量的波动”与图注“实际部署 P:D 接近 1:1，不考虑波峰波谷，整体收入是平均每卡每小时 38 美金（按谷价）”逐字相符；跨模型复核（视觉复核）也看过该图。

视觉复核（Codex gpt-6-astra medium，20 张图）：0 BLOCKER、4 SHOULD_FIX、1 NICE_TO_HAVE，均属实，发布版未改：

1. 41 高亮只盖“not included”小标题，译注含下列三项：待把三项并入高亮。
2. 46 单看易把0.10/0.35当常规价：同上文第1条。
3. 46 整表缩放后字小：可加表头与价格行局部放大（发布后不再重做，记作下期改进）。
4. 横版封面居中1:1裁切会切到问号：已在配图说明提醒手动设裁切框。
5. NICE_TO_HAVE：省流卡第一条“推理”、速览图第四条“融资”跨行断词。
