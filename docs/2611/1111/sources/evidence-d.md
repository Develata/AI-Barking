# D 组证据：速览（1111 期）

取证时间：2026-11-12 17:53 起至约 18:10 北京时间。时间为北京时间（UTC+8）；原文只给日期的写“官方未给时刻”。无截图（D 组按派工不截图）。格式：说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态。状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

## D1 Gemini Decision 1 / Decisions API

一句事实：Google 于 2026-11-11 发布 Gemini Decisions API（模型 `gemini-decision-1`），单次前向给出概率与置信度；博客称单次调用的延迟为 150–250 ms 端到端（模型计算中位数 50–100 ms，限于最长 16K token 的上下文），定价为 $0.10 / 100 万输入 token，输出 token 计费为 0。
级：L1（Google 官方博客，署名 Min Choi、Felix Yu；AI Studio 文档与模型页为 L1）。
发布时刻：页面只标 “Nov 11, 2026”（官方未给时刻）。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| 150–250 ms 端到端；$0.10/百万输入 token，输出 0 | L1 | https://blog.google/innovation-and-ai/technology/developers-tools/gemini-decisions-api/ （存档 `d-gemini-blog-opencli.txt`） | "returning probabilities and confidence scores in 150-250 ms end-to-end latency at $0.10 per 1M input tokens with zero output tokens billed." | 博客正文。定价页 Decision 条目的价格数值未出现在页面文本中（条目仅列 “Standard”），故价格以博客为准 | 已找到（博客 L1；定价页数值未核） |
| 中位延迟 50–100 ms（模型计算），限于最长 16K token 的上下文，且 1–100 个问题之间几乎持平 | L1 | 同上 | "keeping median latency 50-100 ms of model compute (150-250 ms end-to-end), across multimodal inputs and contexts up to 16K tokens, remaining nearly flat from 1 to 100 questions per request." | 限定词：median（中位数）、“of model compute”、“contexts up to 16K tokens”、“1 to 100 questions per request”。输入上限为 “up to 256K tokens”（同文）；16K 以上的延迟博客未给 | 部分支持（延迟数字按限定词读，16K 以上未给数据） |
| 比 Gemini 3.5 Flash-Lite（Interactions API）便宜“nearly 5x” | L1 | 同上 | "With zero output tokens billed, Gemini Decision 1 with the Decisions API is nearly 5x cheaper across the shared benchmark suite than Gemini 3.5 Flash-Lite with the Interactions API." | 限定词 “across the shared benchmark suite” 必须保留；对比对象限定为该模型与该接口 | 已找到 |
| 基准上“leads across the suite, with the lowest calibration error” | L1 | 同上 | "On benchmarks spanning agent routing, reasoning, ranking, and multimodal understanding, Gemini Decision 1 leads across the suite, with the lowest calibration error." | 博客未给套件名、分数或对手；为 Google 自家基准 | 已找到（仅定性） |
| “9 项第一 / 92.2% / ECE 0.0250” | — | 已查 4 个 Google 一手页面：博客 HTML（`d-gemini-blog.html`）与正文（`d-gemini-blog-opencli.txt`）、AI Studio 决策文档（`d-gemini-decisions-docs-opencli.txt`）、模型页 https://ai.google.dev/gemini-api/docs/models/gemini-decision-1 （`d-gemini-decision-1-model-opencli.txt`）、定价页（`d-gemini-pricing-opencli.txt`） | 四处均无 92.2、0.0250、“9 项”或 ECE 数值；模型页只有概述与 “calibrated” 描述，无基准表 | 博客图片未逐张检视（HTML 中未发现相关 alt 文字） | 未找到一手来源 |
| API 限制：缓存、Batch、对话历史、工具调用 | L1 | AI Studio 文档 https://aistudio.google.com/docs/decisions-api （存档 `d-gemini-decisions-docs-opencli.txt`） | "The Decisions API doesn't support context caching, Batch API, or Flex or Priority inference. Repeated input across requests counts in full." / "Conversation history, thought, and tool-call blocks aren't supported as input blocks. Serialize multi-turn context as text." | 文档 Limitations 节；“Available on the Gemini Developer API (v1beta) with Gemini Decision 1 only.” | 已找到 |
| 文档概述（补充） | L1 | 同上 | "Yes/no probabilities always fall between 0.01 and 0.99." | 是非题的概率上限与下限，阈值需在此区间内设置 | 已找到 |

能改变含义的限定词：“across the shared benchmark suite”；“median”与“of model compute”；“up to 16K tokens”；“Repeated input across requests counts in full”。

热度（`barking heat`，2026-11-12 17:55 查）：HN 50456791「Gemini Decisions API」429 分 / 130 评，提交于 2026-11-12 02:32 北京时间（提交时间，不是官方发布时刻）。HN 50457853「Gemini Decision 1 is Free on Google AI Studio」15 分 / 1 评，2026-11-12 03:59，链接为 AI Studio 文档页。“Free” 未核实：定价页文字中的 free 为通用免费层描述，表中 gemini-decision-1 条目未显示免费或价格数值，因此标题中的“免费”未被一手页面证实。

扫描说法勘误（D1）：
- “9 项第一 / 92.2% / ECE 0.0250”→ 已查四个 Google 一手页面，均无出处（未找到一手来源）。
- 线索“仅在 ≤16K 上下文”→ 原文确为 “contexts up to 16K tokens” 的限定；按限定词读，属部分支持。16K 以上的延迟未给。
- HN 标题 “Free on Google AI Studio”→ 未核实。

## D2 Ring Drone Cam 与 Rover Cam

一句事实：Amazon 旗下 Ring 公告 Ring Drone Cam（$249.99，美国，官方称 2027-01-27 起发货）与 Ring Rover Cam（$999.99，美国，2026-12-16 起发货，可预购）；Drone Cam 脚注称未获 FCC 授权，获授权前不得销售或出租。
级：L1（Amazon 官方新闻稿；Ring 官方产品页）。
发布时刻：新闻稿 “Published: November 10, 2026”（官方未给时刻）。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| Drone Cam 249.99 美元，预计 2027-01-27 发货 | L1 | https://www.aboutamazon.com/news/devices/ring-drone-cam-rover-cam-security-robot | "The Ring Drone Cam will begin shipping to customers in the U.S. on January 27, 2027, for $249.99." | 限定词 “to customers in the U.S.”；原文是 “will begin shipping ... on”（定日），不是“预计” | 已找到（措辞：定日） |
| Rover Cam 999.99 美元，2026-12-16 发货 | L1 | 同上 | "The Ring Rover Cam will begin shipping to customers in the U.S. on December 16, 2026, for $999.99. Preorder Ring Rover Cam" | 限定词 “to customers in the U.S.”；可预购 | 已找到 |
| FCC 脚注：未授权，获授权前不能销售或出租 | L1 | 同上 | "** Ring Drone Cam has not been authorized as required by the rules of the Federal Communications Commission. This device is not, and may not be, offered for sale or lease, or sold or leased, until authorization is obtained." | 脚注标记 ** 在提取文本中未能对应到具体句子，但脚注正文指向 Drone Cam | 已找到 |
| 续航 8 分钟 | L1 | https://ring.com/products/drone-cam | FAQ: "Up to 8 minutes per flight. When the flight is over or the battery runs low, it flies back to its dock to recharge." 脚注 1: "Up to 8 minutes of flight time tested with a new, fully-charged battery under controlled conditions. Actual flight time will vary based on environment, usage, and battery condition." | 限定词 “tested ... under controlled conditions” 必须保留；新闻稿本身不含 8 分钟 | 已找到（带限定词） |
| （补充）产品页状态 | L1 | 同上 | "Enter your email below to be notified when this product is available." / "Does not ship to Canada." | 抓取时产品页文本仍显示到货提醒表单 | 已找到（页面快照） |

热度：HN 相关命中 12 条，分数 2–9。主要：50441878 Ring 新闻稿 9 分 / 2 评（2026-11-10 23:37）；50452395 Ars Technica 8 分 / 2 评（11-11 19:59）；50458375 Ring 产品页 3 分 / 3 评（11-12 04:41）。其余见 `d-heat.json`。

扫描说法勘误（D2）：
- “预计 2027-01-27 发货”、“预计 2026-12-16”→ 原文是 “will begin shipping ... on”（定日，且限定美国），措辞应照原文。
- “续航 8 分钟”出自产品页脚注，原文带 “under controlled conditions” 限定，且新闻稿未写。
- FCC 脚注属实。

## D3 AlphaGenome 2

一句事实：DeepMind 发布 AlphaGenome 2，用单细胞数据训练（博客称 “more than 100 million individual cell profiles across 68 tissues”）；博客称在 TenK10K 上候选因果变异识别 30%（原模型 16%）、方向性基因效应 43%（23%）；科研 API 仅限非商用研究，商用为私有预览；官方称未经临床或诊断验证。
级：L1（DeepMind 官方博客）；Nature 新闻为 L4（权威媒体，署名 Ewen Callaway）。
发布时刻：博客 “November 11, 2026”（官方未给时刻）；Nature “11 November 2026”（官方未给时刻；页面元数据 publicationDate=2026-11-11）。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| 候选因果变异回收 30%（前代 16%）、方向 43%（23%） | L1 | https://deepmind.google/blog/alphagenome-2-deciphering-how-cells-read-dna-to-better-understand-disease/ | "On the TenK10K dataset, a large single-cell map of 28 primary immune cell types, AlphaGenome 2 reliably identified nearly twice as many causal variants (30% vs. 16%) and directional gene effects (43% vs. 23%) as the original AlphaGenome fine-tuned on single-cell data and other specialized tools." | 对照组是 “the original AlphaGenome fine-tuned on single-cell data and other specialized tools”，不只是前代；数据集 TenK10K（28 种原代免疫细胞类型） | 部分支持（对照组需写全） |
| 1.07 亿单细胞、68 种组织 | L1 / L4 | 博客 同上；Nature https://www.nature.com/articles/d41586-026-03673-0 | 博客: "Trained on more than 100 million individual cell profiles across 68 tissues"；博客另有 "including over 107 million individual cell profiles"；Nature: "trained ... on gene activity and other measurements made in more than 100 million individual cells, across 68 tissues" | 两处数字并存（100 百万以上 / 1.07 亿）；均为原文，应全部记录 | 已找到（口径并存） |
| 相比上一代 SOTA 多约一倍的基因表达变异 | L1 | 博客 同上 | "AlphaGenome 2 identifies nearly twice as many variants that change gene expression in cells, compared to our previous state-of-the-art model." | 比较对象是“previous state-of-the-art model” | 已找到 |
| 科研 API、非临床 | L1 | 博客 同上 | "AlphaGenome 2 is available today for non-commercial research via the AlphaGenome API, and in private preview for commercial research." / "Please note that AlphaGenome and AlphaGenome 2 are intended for research use only and have not been validated or approved for clinical or diagnostic purposes." | 非商用可用；商用仅私有预览 | 已找到 |
| 模型形态（补充） | L4 | Nature 同上 | "a co-author of a preprint describing the model"（Gayoso 语） | 模型描述为 preprint | 已找到（限定：preprint） |
| 物种与细胞类型数（补充） | L4 | Nature 同上 | "The model also isn't limited to making predictions in the 637 types of human and 311 mouse cells it was trained with." | 另一口径：637 种人类、311 种小鼠细胞类型 | 已找到（口径与 68 组织不同，并列记录） |

存档说明：Nature 页面在注册墙处截断，正文仅前半段（lede 与 IBD 背景、训练数据段）可读；本次未取得全文。

热度：HN 50454306「AlphaGenome 2」13 分 / 0 评（2026-11-11 23:04）。Nature 页面未查到 HN 命中。

扫描说法勘误（D3）：
- “前代 16%”→ 对照组是 “the original AlphaGenome fine-tuned on single-cell data and other specialized tools”，必须写全。
- “1.07 亿”→ 博客两处分别写 “more than 100 million” 与 “over 107 million”，都是原文。

## D4 OpenAI × Upwork 自由职业研究

一句事实：OpenAI 与 Upwork 研究显示，在 63 个匹配子类别中，43 个（68.3%）满足“ChatGPT 使用强度低而 Upwork 份额升，或使用强度高而份额降”；其余 20 个相反。官方称 ChatGPT 使用本身不能解释自由职业需求变化，并称该图 “does not identify a causal effect”。
级：L1（OpenAI 官方页面与 PDF 报告）。
发布时刻：网页 “November 10, 2026”（官方未给时刻）；PDF 标 “November 2026”。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| 63 个类别中 43 个呈“ChatGPT 用得多、Upwork 份额降”，20 个相反 | L1 | https://openai.com/index/ai-effect-on-freelance-work/ ；全文 PDF https://cdn.openai.com/pdf/openai-upwork-full-report.pdf （存档 `d-openai-upwork-full-pdf.txt`） | 网页: "Many activities that were relatively common in ChatGPT lost share on Upwork, while less common activities gained share. Of the 63 work categories analyzed, 43 followed this pattern." / "The remaining 20 work categories showed the reverse." PDF: "Of the 63 subcategories, 43 (68.3 percent) either had lower ChatGPT intensity and gained Upwork marketplace share or had higher intensity and lost share." | 网页的 “lost share ... gained share” 两个方向都写了；“用得多、份额降”只是两个方向中的一个。PDF 另有 “Six categories (9.5 percent) combined higher ChatGPT intensity with gains”；“the remaining 14 categories (22.2 percent) combined lower ChatGPT intensity with losses” | 已找到（43 = 两方向合计，定义以 PDF 为准） |
| 有效类别数 14.6→21.2 | L1 | PDF 同上；网页同上 | PDF: "the effective number of task clusters increased from 14.6 to 21.2—approximately 45 percent." 网页: "45% more diverse" | 指标是 AI 相关岗位的任务簇有效数（effective number of task clusters），不是“类别数”；时段 2023 年初至 2025 年末 | 已找到（需写全指标名） |
| 非因果、非岗位数（官方限定） | L1 | PDF 同上 | "The figure does not measure aggregate Marketplace growth or identify a causal effect of ChatGPT use on Upwork demand." / "neither represents the labor market as a whole." / 网页: "ChatGPT use alone does not explain changes in freelance demand." | 原文没有 “非岗位数” 一词；最接近的是 “does not measure aggregate Marketplace growth” | 部分支持（“非因果”属实；“非岗位数”为改写） |
| 时间窗口（补充） | L1 | 网页 同上；PDF 图注 同上 | 网页: "ChatGPT shares cover Jan 2025–Jun 2026; Upwork shares are H1 2026. Changes in Upwork share are H1 2025 to H1 2026, in percentage points." 图注（PDF）: 使用强度取 ChatGPT 的 H2 2025，Upwork 基线取 H1 2023，变化取 H1 2023 至 H2 2025 | 两段是不同图表的窗口（网页图表与 63 点散点图），不构成冲突，两者都需写清各自所指 | 已找到 |

热度：HN 查询「OpenAI Upwork freelance」0 命中，未再换词查询。

扫描说法勘误（D4）：
- “非岗位数”→ 原文是 “does not measure aggregate Marketplace growth”，不是“非岗位数”。
- “有效类别数”→ 原文指标是 AI 相关岗位的任务簇有效数（effective number of task clusters）。
- 扫描线索把 43 个概括为“用得多、份额降”一个方向；原文（网页与 PDF）写的是两个方向合计。

## D5 AFT / UFT 与 Anthropic 学校 AI 协议

一句事实：AFT 与 UFT 宣布与 Anthropic 达成 “legally binding agreement”，把 AI 安全与隐私标准在美国 K-12 学校中变为可执行；协议文本（National Academy for AI Instruction 的 “AI Safety & Privacy Standard” MOA，Version 1.0，September 2026）自称 “binding Memorandum of Agreement”，签署方含 Anthropic PBC；附录 C 称 Anthropic 的承诺 “currently apply to Claude for Teachers”，并且对学生用户的要求不适用。
级：L1（AFT 官方新闻稿，AFT 自述其约定；协议文本为签署文件，L1）。
发布时刻：AFT 新闻稿 “Tuesday, November 10, 2026”（官方未给时刻）。签署页 DocuSign 证书：信封发出 10/21/2026 7:28 PM；AFT 一方签名 10/22/2026 6:36 AM（证书未标时区，按原样记录）。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| “legally binding”，在“thousands of U.S. K-12 schools”中可执行 | L1 | https://www.aft.org/press-release/aft-uft-and-anthropic-sign-binding-ai-safety-and-privacy-agreement-schools ；协议 https://www.aft.org/sites/default/files/media/documents/2026/Final_Signed_Anthropic_Agreeemnt_102226.pdf （存档 `d-aft-agreement-pdf.txt`） | 新闻稿: "announced a legally binding agreement with Anthropic that makes AI safety, privacy and transparency standards enforceable in thousands of U.S. K-12 schools." 协议: "This is a binding Memorandum of Agreement between the National Academy for AI Instruction ... and any AI Provider ..." / "Each principle below is a binding commitment." / "To the extent permitted by applicable law, the Academy may enforce the commitments" | “legally binding”与“thousands of schools”是 AFT 的表述；协议自述 binding，并写明 Academy 可执行。“thousands”未核实 | 已找到（AFT 表述；协议自述 binding） |
| 自动适用于 Anthropic 与使用 Claude for Teachers 的学区现有协议 | L1 | 新闻稿 同上；协议 同上 | 新闻稿: "automatically applies to Anthropic's existing agreements with school districts using Claude for Teachers" 协议: "If requested, the AI Provider agrees to make the substantive protection of the Standard available to EDU Customers within 90 days of the Effective Date through a clear, simple, and expeditious process." / "Once the substantive protections of the Standard are incorporated into an EDU Customer's agreement, those protections will be contractually enforceable by the EDU Customer." 附录 C: "Anthropic's commitments under these Standards currently apply to Claude for Teachers." | 协议的机制是 “If requested”（应要求）后并入学区协议，而非“自动”；附录 C 限于 Claude for Teachers，学生用户要求不适用。AFT 的 “automatically applies to ... existing agreements” 未在协议中找到对应条款；两者张力未解决 | 已找到（AFT 表述）；与协议机制存在未解决张力 |
| Anthropic 已签署 | L1 | 协议 署名页与 DocuSign 证书 | 署名页: "Legal name: Anthropic, PBC"；Anthropic PBC 签名人为其 Head of Beneficial Deployments（证书所示） | 生效: "These principles are effective from the date signed" | 已找到 |
| （时间线旁证）Anthropic 此前未签 | — | WebSearch 摘要（引 EdWeek 2026-09 报道，未存档正文） | 摘要称 Anthropic 尚未签署 | 早于 10-21 签署，已被协议签署记录取代 | 仅作时间线，不引用 |

热度：HN 查询「AFT Anthropic binding agreement schools」0 命中。EdWeek 的 2026-09 报道（WebSearch 线索）讨论的是微软与 AFT 的协议，不是 Anthropic 的 11 月签署；未找到针对本次协议的 EdWeek 11 月报道。

扫描说法勘误（D5）：
- 线索“automatically”→ 见上表：AFT 新闻稿有此表述，协议机制是 “If requested”。二者未解决，正文宜两边都写明，或以协议为准写“应要求并入”。
- 适用范围→ 附录 C：Claude for Teachers，不适用于学生用户。
- “legally binding”→ AFT 表述；协议自述 binding，可执行性以协议与 AFT 为准，未另核实。

## D6 台积电董事会批准投资 Anthropic、OpenAI

一句事实：台积电 2026-11-10 董事会决议，批准 “in an amount not to exceed US$500 million” 投资 Anthropic, PBC（按其 IPO 每股价格购买普通股）与 OpenAI Group PBC（购买优先股）；另批准不超过 5 亿美元设立半导体生态投资公司，该项不限于两家 AI 公司。
级：L1（台积电向 SEC 提交的 Form 6-K，2026-11-10 签署，决议正文可核）。
发布时刻：决议日 2026-11-10（台北时间，与北京时间同为 UTC+8；官方未给时刻）；HN 上 TSMC 官方新闻稿链接的提交时间为 2026-11-10 22:59 北京时间。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| 董事会批准各至多 5 亿美元投资 Anthropic、OpenAI | L1 | https://www.sec.gov/Archives/edgar/data/0001046179/000104617926000797/tsm-boardx202611xx.htm （全文存档 `d-tsmc-sec-6k-board-opencli.txt`；同名 `.html` 为 403 页面） | "Approved the investments in (1) Anthropic, PBC at its per share initial public offering price in an amount not to exceed US$500 million to acquire Anthropic's common shares; and (2) OpenAI Group PBC in an amount not to exceed US$500 million to acquire OpenAI's preferred stock." | 限定：“not to exceed”为上限；“approved”为董事会批准，不等于交割；Anthropic 部分按其 IPO 每股价格；OpenAI 部分为优先股 | 已找到 |
| 另一项：不超过 5 亿美元设立投资公司 | L1 | 同上 | "Approved the investment of no more than US$500 million in the establishment of an investment company for the purpose of making strategic investments in the semiconductor industry and related ecosystem." | 与 Anthropic、OpenAI 无关，是另一项决议，不得合并 | 已找到 |
| 路透报道（各至多 5 亿美元） | L4 | https://www.reuters.com/world/china/tsmc-invest-up-1-billion-anthropic-openai-2026-11-10/ | 未取得正文：fetch 返回 401 Unauthorized（存档为 headers 与 html 原件，无 txt） | 与 6-K 数字一致，但本次未引用路透正文 | 未取得正文（仅线索） |
| 台积电新闻稿 | L1 | https://pr.tsmc.com/english/news/3352 | fetch 返回 403 Cloudflare 质询，未取得正文 | 6-K 与此稿内容同源，本次以 6-K 为准 | 未取得正文 |

热度：HN 50441326「TSMC approves investments of up to $500M each in Anthropic and OpenAI」3 分 / 1 评，2026-11-10 22:59 北京时间。

扫描说法勘误（D6）：未发现与线索冲突之处。补充：线索“各 5 亿美元”应写作“各不超过 5 亿美元”，并注明 Anthropic 部分按 IPO 价格。

## D7 Anthropic “Worse Together”：多用户多智能体研究

一句事实：Anthropic 对齐研究博客（2026-11-11）与 arXiv 2610.00583（v1 于 2026-09-30 18:49:25 UTC 提交，北京时间 2026-10-01 02:49:25）报告，在四个模拟环境中，每用户各派一个智能体的团队，群体结果都比单一协调者差；API key 环境中，点对点团队达到最优结果的 30%（Opus 5）与 12%（Sonnet 5），协调者为 64% 与 32%。
级：L1（Anthropic 官方研究博客，作者含 Anthropic Fellows 与 Anthropic 员工；arXiv 论文）。
发布时刻：博客 “November 11, 2026”（官方未给时刻）；arXiv v1 2026-09-30 18:49:25 UTC（北京时间 2026-10-01 02:49:25）。

| 说法 | 级 | 一手来源 | 原文摘句（逐字） | 条件 / 口径 | 状态 |
|---|---|---|---|---|---|
| 模拟场景 | L1 | 博客 https://alignment.anthropic.com/2026/worse-together/ ；arXiv https://arxiv.org/abs/2610.00583 （HTML 版 https://arxiv.org/html/2610.00583v1） | 博客: "we built four environments with simulated users and everyday tasks" / "In these simulated settings, we also saw problematic behaviors" ；论文: "The API key and clinic environments use a simulated harness built on Inspect" | “simulated”是研究条件，必须保留 | 已找到 |
| 团队只拿协调者 47%（Opus 5）、38%（Sonnet 5） | L1 | 博客 同上 | 博客: "peer-to-peer teams reach 47% and 38% of the coordinator's performance for Claude Opus 5 and Claude Sonnet 5, respectively (30% and 12% of the optimal outcome, against 64% and 32% for the coordinator)." 论文: "the peer-to-peer team reaches 30% of the optimum for Opus 5 and 12% for Sonnet 5, against 64% and 32% for their coordinators" | 博客原文即此句（已找到）。47% 与 38% 是 30/64 与 12/32 的比值，本次未在论文文本中检索到这两个比值，故正文宜直接引 30%、64%，12%、32%（均为“最优结果的比例”） | 已找到（博客原文；论文原数为 30/64、12/32） |
| 每环境 25 个情景 | L1 | arXiv 摘要 https://arxiv.org/abs/2610.00583 ；论文 HTML 同上 | 摘要: "across five frontier models and 77 scenarios in four environments" ；论文: "For each environment, we constructed between 3 and 25 scenarios." ；摘要: "comprising 74 scenarios" (MAMUBench) | 25 个只见于 API key 与个人助理环境；全部为 77 个情景、四环境合计 | 与说法不符（修正） |
| 四个环境中团队都比协调者差 | L1 | 同上 | 摘要: "Teams deliver worse group outcomes than the coordinator in every environment" ；博客: "In all four environments, group outcomes were worse when each user delegated to their own agent rather than when one agent served everyone." | 研究设计的对照是“协调者”（coordinator）；“群体结果”是研究的评分口径 | 已找到 |
| 基准开源 | L1 | https://github.com/safety-research/MAMUBench （存档 `d-mamubench-github.txt`） | 博客: "We are releasing three of the environments as a benchmark, MAMUBench" ；摘要: "We will release the API key, clinic, and personal assistant environments as MAMUBench, comprising 74 scenarios" | 博客与摘要的“三个环境”一致 | 已找到 |

热度：HN 50463856「Worse Together: How Performance Breaks Down in Multi-User Multi-Agent Teams」3 分 / 0 评，2026-11-12 14:48 北京时间。热度低。

扫描说法勘误（D7）：
- “每环境 25 个”→ 与说法不符。原文为 77 个情景、四环境合计；每环境 3–25 个。
- “团队只拿 47% / 38%”→ 博客原文即此句，但它是 30/64、12/32 的比值；论文直接报的是最优结果的 30%、12%，对照 64%、32%。

## 扫描说法勘误汇总（D 组）

1. D1：“9 项第一 / 92.2% / ECE 0.0250”→ 已查四个 Google 一手页面，均无出处（未找到一手来源）。
2. D1：“仅在 ≤16K 上下文”→ 原文为 “contexts up to 16K tokens” 的限定，中位数，属部分支持；16K 以上未给。
3. D1：HN 标题 “Free on Google AI Studio”→ 未核实。
4. D2：“预计 2027-01-27 发货”→ 原文 “will begin shipping ... on January 27, 2027”（定日，美国）。
5. D3：“前代 16%”→ 对照组为 “the original AlphaGenome fine-tuned on single-cell data and other specialized tools”。
6. D4：“非岗位数”→ 原文为 “does not measure aggregate Marketplace growth”；43 个为两方向合计，线索只写了一个方向。
7. D5：“automatically applies”→ AFT 新闻稿有此表述；协议机制为 “If requested” 后并入，二者未解决，见 D5。适用范围为 Claude for Teachers。
8. D7：“每环境 25 个情景”→ 77 个情景、四环境合计，每环境 3–25 个。
9. D7：“47% / 38%”→ 博客原文即此句，但为 30/64、12/32 的比值；论文直接报 30%、12%（对照 64%、32%）。

## 存档说明

- 文本存档（可入库）：`d-*.txt`、`d-*.headers`、`d-*.json`、`d-fetch-log.jsonl`、`d-heat.json`、`d-hn-50456791.json`、`d-tsmc-sec-6k-board-opencli.txt`、`d-gemini-*-opencli.txt`。`d-worse-together-arxiv-html.txt` 为 298 KB，未超过 1 MB。
- HTML 与 PDF 原件（`d-*.html`、`d-*.pdf`）按规则只存本地，待 Claude 传网盘；其中 arXiv HTML 原件约 3.2 MB。
- 已做的隐私处理：存档中的邮箱地址替换为“[邮箱已隐去]”；协议 PDF 文本中的签名证书 IP 地址已删除（替换为占位）；Gemini 文档与定价页的账号邮箱行已从 opencli 存档中删除。HTML 中的个人邮箱也已替换（`d-aft-press.html`、`d-worse-together-arxiv-html.html`）；其余 HTML 中仅剩机构通用地址（Springer 版权、example.com、SEC opendata）。
- sha256 说明：对 `d-aft-*`、`d-aft-press`、`d-worse-together*` 的 `.txt` 与 `.html` 做了邮箱替换，文件内容变化，`d-fetch-log.jsonl` 中这些文件的 sha256 与现文件不再一致。
