# 1013 期 D 组取证（速览候选 D1–D10）

执行方：Haiku 5.5（claude-haiku-5-5），子代理。抓取过程见 `capture-log-d.md`。北京时间（UTC+8），官方未给时刻处如实写“官方未给时刻”。本组不截图，只存档与摘句。逐字摘句已用 `d-verify-quotes.py` 在存档中 grep 核对（56 条，0 失败）。

级别按 `EDITORIAL.md` 事实分级；无法精确套用的，注明“类比”。

## 一、条目表

### D1 Meta Muse Wonder 与 Muse Wonder Full

| 项 | 内容 |
|---|---|
| 一句事实 | Meta 于 October 12, 2026 发布 Muse Wonder 与 Muse Wonder Full。Muse Wonder 通过 Meta Model API 与 Muse Code 提供；Muse Wonder Full 仅对 Muse Code 订阅用户开放。据 AA 评测，Muse Wonder (max) 智能指数 52，Muse Wonder Full (max) 为 56，前代 Muse Spark 1.3 (max) 为 48；Muse Wonder (max) 每题成本 $4.52，为 Muse Spark 1.3 (max) 的 $1.60 的 2.8 倍；Muse Wonder Full 不按 token 计价，故无每题成本数字。 |
| 北京时间（原时区） | 2026-10-12（原文日期；Meta 页仅给 “October 12, 2026”，未给时刻与时区） |
| 一手来源 | Meta 官方：https://research.meta.ai/blog/muse-wonder/ （已看原文）；AA：https://artificialanalysis.ai/articles/muse-wonder （已看原文） |
| 级 | Meta 发布为 L1；AA 数字为 L3（第三方评测原始页面） |
| 逐字摘句 | Meta：“Muse Wonder Full is available today for Muse Code subscribers.”（`d-d1-meta-wonder.txt`）；AA：“achieving Artificial Analysis Intelligence Index v4.3 scores of 52 and 56 at max reasoning effort”（`d-d1-aa-wonder.txt`）；“Muse Wonder (max) costs $4.52 per Artificial Analysis Intelligence Index task, 2.8x Muse Spark 1.3 (max, $1.60)”（`d-d1-aa-wonder.txt`）；“Muse Wonder Full is not priced per token, so no Cost per Task figure is available”（`d-d1-aa-wonder.txt`） |
| 状态 | 已找到 |
| 备注 | 52/56 与 $4.52 / $1.60 数字只见于 AA 文章，Meta 页面未给这些数字（Meta 自述 “state-of-the-art”，为厂商自家口径）。“max effort”口径以 AA 原文为准，Meta 页未写 “max”。Muse Wonder 上下文 1M，定价 $4 输入 / $0.20 缓存输入 / $20 输出 每百万 token（AA 原文）。 |

### D2 xAI 合并订阅（页面品牌 “The X Subscription”）

| 项 | 内容 |
|---|---|
| 一句事实 | xAI 于 Oct 12, 2026 发布合并订阅，Grok 与 Cursor 共用一个 Grok 模型额度池；Premium（$30/月）及以上档可在 Cursor 中使用第三方模型，按 API 标价另行充值计费；旧套餐不再对新注册用户开放；页面称将在数日内向全球开放。 |
| 北京时间（原时区） | 2026-10-12（官方未给时刻） |
| 一手来源 | https://x.ai/news/x-subscription （已看原文；页脚版权署名为 “SpaceXAI LLC”） |
| 级 | L1 |
| 逐字摘句 | “Paid plans on The X Subscription include a generous pool of Grok model usage shared across Grok and Cursor.”（`d-d2-xai-sub.txt`）；“With the new plans, third-party models are still available in Cursor on Premium and above.”（`d-d2-xai-sub.txt`）；“Usage is billed separately at API list prices, through top-ups.”（`d-d2-xai-sub.txt`）；“The previous plans are no longer available for new sign-ups.”（`d-d2-xai-sub.txt`）；“Over the next few days, we’ll make it available to everyone globally.”（`d-d2-xai-sub.txt`） |
| 状态 | 已找到 |
| 备注 | 事实句写“xAI 合并订阅”，不写平台名。两处措辞需注意：正文表格与段落写 “Premium and above”，FAQ 写 “third-party models are still available in Cursor on X Premium and higher tiers”（`d-d2-xai-sub.txt`），两者是否同指 $30 的 Premium 档，页面未明写，需人工确认。“此前三者合计 $90/月”来自页面表格 Premium 行的 “previously $90/month combined”，为页面自述。五档价格 Free $0 / Plus $8 / Premium $30 / Super $100 / Ultra $200（页面表格）。HN 无对应帖（关键词搜索无命中）。 |

### D3 QGIS 对 AI 辅助贡献的禁止政策（QEP-408 修订，PR #387）

| 项 | 内容 |
|---|---|
| 一句事实 | QGIS 项目将 QEP-408 AI 工具政策改为“不允许”：现行文本称 “No LLM, AI or agent assisted code contributions are allowed in the https://github.com/qgis/qgis repository.”（范围为 qgis/qgis 代码仓库）。该修订于 2026-10-07T21:00:14Z 合并。文本未写期限。 |
| 北京时间（原时区） | PR 合并 2026-10-07T21:00:14Z（UTC）= 2026-10-08 05:00（北京）；PR 创建 2026-09-24T07:51:43Z（UTC）= 2026-09-24 15:51（北京） |
| 一手来源 | https://github.com/qgis/QGIS-Enhancement-Proposals/blob/master/qep-408-ai-tool-policy.md （经 `gh api` 取 master 版本，已看原文）；PR：https://github.com/qgis/QGIS-Enhancement-Proposals/pull/387 （经 `gh pr view` 取元数据与评论） |
| 级 | L1（项目官方提案文档）；PR 评论为维护者（nyalldawson）公开发言，L2 |
| 逐字摘句 | “No LLM, AI or agent assisted code contributions are allowed in the https://github.com/qgis/qgis repository.”（`d-d3-qep408-master.md`）；PR 正文：“This formally changes the AI policy to a strict \"NOT ALLOWED\" policy.”（`d-d3-pr387.json`）；评论：“lets start with a closed door”（`d-d3-pr387-comments.tsv`，2026-09-24T22:37:27Z 维护者 nyalldawson 的回复）、“The intention is not that this becomes the "written in stone, forever" policy”（同文件） |
| 状态 | 已找到 |
| 备注 | 旧版 QEP-408（2026-02 至 2026-09）允许工具辅助但要求 human in the loop 与标注（`d-d3-qep408-at-d9efd1a3f9.md`）。PR 标题 “Add QEP428: No ai contributions permitted” 与修改文件 qep-408 不一致（见勘误）。HN 50054751 标题写 “QGIS enacts moratorium on AI contributions”，QEP 正文不用 moratorium 一词。 |

### D4 Intel D270 推理 GPU

| 项 | 内容 |
|---|---|
| 一句事实 | 未能从一手来源核实。HN 标题为 “Intel D270 GPU AI Accelerator–Up to 480 GB LPDDR5X”，链接指向 intel.com 产品页；该页 curl 返回 HTTP 403（bot 拦截），未绕过。搜索摘要统称该产品为 “Crescent Island”，并称最高 480GB、350W（标称）。 |
| 北京时间（原时区） | HN 帖创建 2026-10-12T21:35:35Z（UTC）= 2026-10-13 05:35（北京）；Intel 官方发布时刻未取得 |
| 一手来源 | 未取得。intel.com 产品页 https://www.intel.com/content/www/us/en/products/details/discrete-gpus/data-center-gpu/d-series.html 返回 403；WebSearch 限定 intel.com 域得到的新闻稿 https://www.intel.com/content/www/us/en/newsroom/news/artificial-intelligence/intel-accelerates-open-infrastructure-agentic-ai-era.html 与 …/intel-to-expand-ai-accelerator-portfolio-with-new-gpu.html 同样返回 403。未绕过。 |
| 级 | 无一手；搜索摘要为 L5 线索 |
| 逐字摘句 | 无（未取得原文） |
| 状态 | 未找到一手来源 |
| 备注 | 命名：WebSearch 索引中 intel.com 产品页标题为 “Intel GPU D270 │ Inference-Optimized Data Center GPU”，说明 D270 是 Intel 的产品名之一；搜索摘要同时把同一加速器称为 “Crescent Island”。以上仅是索引标题，正文未取得。规格（最高 480GB、350W）、带宽（1.5 TB/s）、FP8 算力、上市时间（各来源称 2026 年底或 2027 年，不一致）、价格均未核实，不入正文。详见 `d-d4-websearch-intel.txt`。 |

### D5 AA-3D 交互式 3D 场景评测

| 项 | 内容 |
|---|---|
| 一句事实 | AA 于 October 12, 2026 发布 AA-3D（AA-3D v1.0）：各模型作为代理在离线沙箱中把文字简报做成 three.js 页面，由招募的评审小组做盲测成对投票并以 Elo 排名；截至 2026-10-12 数据，Claude Sonnet 5.5 以 1194 居首，与另外三个 Anthropic、OpenAI 模型统计上并列（置信区间重叠）；每个场景成本为 $0.20 至 $32.08（依模型而定）。 |
| 北京时间（原时区） | 2026-10-12（官方未给时刻）；数据截至 12 October 2026 |
| 一手来源 | https://artificialanalysis.ai/articles/aa-3d （已看原文）；方法说明 https://artificialanalysis.ai/methodology/aa-3d （已看原文，`d-d5-aa3d-methodology.txt`） |
| 级 | L3 |
| 逐字摘句 | “Claude Sonnet 5.5 is first at 1194, statistically tied with three other Anthropic and OpenAI models.”（`d-d5-aa3d.txt`）；“their 95% confidence intervals all overlap with that of Claude Sonnet 5.5.”（`d-d5-aa3d.txt`）；“A scene costs anywhere from $0.20 to $32.08, depending on the model.”（`d-d5-aa3d.txt`）；“AA-3D v1.0 has 55 tasks.”（`d-d5-aa3d-methodology.txt`）；“through human preference voting”（同文件，方法说明，指 recruited panel 的盲测投票）；“Our recruited panel compares pairs of scenes built from the same brief, without knowing which model made which”（`d-d5-aa3d.txt`） |
| 状态 | 已找到 |
| 备注 | 方法说明页确认 55 个任务、人类偏好投票（recruited panel），与派工单一致。注意 “Claude Sonnet 5.5 … first” 是 AA 原文措辞，“暂列第一”为派工单措辞；原文强调统计并列。页面为 v1.0 发布时数据，排行榜实时更新。 |

### D6 AA FrontierCode 1.1（Cognition 基准，AA 独立评测）

| 项 | 内容 |
|---|---|
| 一句事实 | FrontierCode 1.1 为 Cognition 的基准，AA 独立评测（页面标注 “Independently benchmarked by Artificial Analysis”）。主集 100 题（共 150 题中的主集），每题三次尝试。Claude Opus 5.5 (max with fallback) 平均分 52%、通过率 58%；GPT-6.1 Sol (max) 与 Claude Fable 5.1 (max with fallback) 均为 49%。评分含基于 LLM 的评审，judge 为 GPT-6 Luna (medium)。FrontierCode 1.1 为独立排行榜，不计入 AA Intelligence Index。 |
| 北京时间（原时区） | 2026-10-12（官方未给时刻）；数据截至 12 October 2026 |
| 一手来源 | https://artificialanalysis.ai/articles/frontiercode （已看原文）；基准作者 Cognition 页 https://cognition.com/blog/frontier-code-1.1 （仅见 AA 页链接，未直接打开） |
| 级 | L3（AA 数据）；基准设计权属 Cognition（L1，未直接核实原页） |
| 逐字摘句 | “Claude Opus 5.5 (max with fallback) leads with an average score of 52% and a pass rate of 58%.”（`d-d6-aa-frontiercode.txt`）；“followed by GPT-6.1 Sol (max) and Claude Fable 5.1 (max with fallback) at 49%”（同文件）；“We run FrontierCode 1.1 Main, the primary 100-task set of the benchmark's 150 tasks”（同文件）；“We use GPT-6 Luna (medium) as the judge.”（同文件）；“FrontierCode 1.1 launches as a standalone leaderboard and is not part of the Artificial Analysis Intelligence Index.”（同文件） |
| 状态 | 已找到 |
| 备注 | 派工单写的 “Opus 5.5 约 52%” 是平均分（score），通过率为 58%，两个指标勿混。 |

### D7 NeurIPS 2026 论文：模型权重中隐藏加密能力

| 项 | 内容 |
|---|---|
| 一句事实 | 论文 “AI Models Can Provably Hide Arbitrary Capabilities”（arXiv:2610.13928）称，可在宿主模型权重中嵌入加密网络，仅在秘密触发时执行，在密码学假设下可证明地难以被诱发或解读；作者在 arXiv 注记中写 “Accepted at NeurIPS 2026”。论文自述为概念验证，实验含子集奇偶（subset parity）与化学反应可行性分类两项，后者为 CBRN 相关能力的代理任务，宿主模型约 2.07B 参数。 |
| 北京时间（原时区） | arXiv v1 “Fri, 9 Oct 2026 17:55:06 UTC” = 2026-10-10 01:55（北京） |
| 一手来源 | https://arxiv.org/abs/2610.13928 （已看原文）；HTML 版 https://arxiv.org/html/2610.13928v1 （已看原文，作者单位来源） |
| 级 | 论文原文（作者自述，研究原始页面），按 L3 类比，未正式定级 |
| 逐字摘句 | “under the stated cryptographic assumptions, it is computationally hard to recover the trigger”（`d-d7-arxiv-html.txt`）；“Our proof-of-concept hides only a small binary reaction-feasibility classifier and has substantial overhead, so it is not a turnkey attack.”（`d-d7-arxiv-html.txt`）；“We evaluate our constructions on subset parity and chemistry classification”（同文件）；“a small chemical reaction prediction task - a proxy for CBRN-relevant capabilities”（`d-d7-arxiv.txt`）；“Accepted at NeurIPS 2026”（`d-d7-arxiv.txt`，作者注记） |
| 状态 | 已找到 |
| 备注 | 作者单位：Contramont Research、University of Oxford（两位）、ACS Research（HTML 版署名）。论文另写 “The potential detectability of the presence of a cryptographic circuit and small-scale experimental settings further limit the immediate practicality of such attacks.”（`d-d7-arxiv-html.txt`）。NeurIPS 官方接收名录未核。HN 50065692 标题 “AI models can provably hide arbitrary capabilities”。 |

### D8 Epic Games 转向 AI 优先，迪士尼合作游戏推迟（Bloomberg 报道）

| 项 | 内容 |
|---|---|
| 一句事实 | 据彭博社报道（经 Ars Technica 转述），Epic Games 与迪士尼合作的 Marvel 与 Star Wars 题材射击游戏此前计划于 2027 年 3 月发布，现已无限期推迟；据内部消息，人类开发者将负责管理与引导由 Claude 等模型生成的代码与美术资产，即 “no more hand-written code”。CEO Tim Sweeney 在推文中称 “We have a huge pivot underway here at Epic, moving developers around to retool for maximum AI acceleration.”，并称 “The aim isn't to do the same stuff with fewer people, but to enable everyone here to build MORE and BETTER stuff”。 |
| 北京时间（原时区） | Ars 页显示 “Oct 12, 2026 4:59 pm”（页面未标时区）；Sweeney 推文 “Mon Oct 12 18:00:40 +0000 2026” = 2026-10-13 02:00:40（北京）；Bloomberg 报道时刻未取得（URL 日期 2026-10-12） |
| 一手来源 | Bloomberg 原报道：https://www.bloomberg.com/news/articles/2026-10-12/epic-games-delays-disney-project-asks-developers-to-use-more-ai （未取得：curl 返回 HTTP 403 “Are you a robot?” 挑战页，未绕过；HN 投稿标题与 URL 一致，为 “Epic Games Delays Disney Project, Asks Developers to Use More AI”，原页标题未核）；Ars Technica：https://arstechnica.com/gaming/2026/10/report-epic-games-goes-all-in-on-ai-for-disney-collaboration/ （已看原文，页面标注 “Oct 12, 2026 4:59 pm”）；gamesindustry.biz：https://www.gamesindustry.biz/epic-games-delays-disney-game-and-tells-developers-to-go-ai-first （已看原文，转述 Bloomberg 并引 Epic 发言人）；Sweeney 推文：https://x.com/TimSweeneyEpic/status/2109705826688549304 （经 fxtwitter JSON 核原文，已看；仅核对此一条，未核其后续线程） |
| 级 | Ars、gamesindustry.biz 为 L4（转述 Bloomberg）；Bloomberg 原文未取得；Sweeney 为公司高管公开发言，L2；Epic 通讯总监 Jake Jones 的声明经 gamesindustry.biz 转述，L2 类比 |
| 逐字摘句 | “The $1.5 billion collaboration between Epic and Disney, announced in early 2024”（`d-d8-arstech.txt`）；“was slated to release a Marvel- and Star Wars-themed shooter in March 2027.”（同文件）；“That project has now been delayed indefinitely”（同文件）；“Human developers will be tasked with managing and guiding code and art assets generated by models”（同文件）；“according to internal Epic employee messages viewed by Bloomberg”（同文件）；“moving developers around to retool for maximum AI acceleration.”（`d-d8-sweeney-fx.json`，推文 JSON）；“The aim isn't to do the same stuff with fewer people”（同文件）；“No more hand-written code.”（`d-d8-gamesindustry.txt`，转述 Bloomberg 引内部消息）；“Disney invested $1.5 billion in Epic in 2024 and took an equity stake”（同文件）；“no new release date has been confirmed”（同文件）；“embracing AI-accelerated development, led by Epic developers who are the visionaries and creative drivers for everything we do.”（同文件，Epic 通讯总监 Jake Jones 的声明） |
| 状态 | 推迟、2027 年 3 月、15 亿美元（$1.5B）：部分支持（经 Ars、gamesindustry.biz 转述 Bloomberg，原文未打开）。“CEO 回应称人类开发者仍负责创作”：所查 Sweeney 推文中未见此意，判为与说法不符（仅限该条推文）；但 Epic 通讯总监的声明写明开发者是 “creative drivers”，与“人类仍负责创作”的方向一致，故 CEO 的说法应改为“公司发言人称”，待人工决定。 |
| 备注 | “$1.5B 为 2024 年投资而非开发成本”：Ars 正文称 “collaboration”，gamesindustry.biz 称 “Disney invested $1.5 billion in Epic in 2024”，两者支持“投资”的表述；原文未出现“开发成本”。gamesindustry.biz 写 “no new release date has been confirmed”，Ars 写 “delayed indefinitely”，两者都是据 Bloomberg 的转述。HN：Ars 帖 50065312（7/3）；gamesindustry.biz 帖 50067543（3/0）；HN 投稿 50061195（Bloomberg 标题投稿，10/2，10-13 03:03 北京）；HN 投稿 50062237（“Epic Games tells employees to use AI to get Disney game done”，链接 avclub.com，83/69，10-13 04:07 北京），该文未核。 |

### D9 Ben Recht 批评 OpenAI 的“更快傅里叶变换”结果

| 项 | 内容 |
|---|---|
| 一句事实 | Ben Recht 在 argmin（Substack）发文（Oct 12, 2026）批评 OpenAI 的 FFT 结果：文中称 “Unfortunately, I did not find a faster Fourier transform.”，并自述 “Did I read this paper? Of course not.”，改用聊天机器人提取要点。OpenAI 官方说明（openai/math 仓库 lean/docs/130.md）写明：该结果为 subsequential，“with no all-length, bounded-coefficient, conditioning, or bit-complexity claim”，并称模型 “permits exact complex arithmetic and unrestricted coefficients”。 |
| 北京时间（原时区） | Recht 文 Oct 12, 2026（未给时刻）；OpenAI 文件最近提交 2026-10-08T05:03:50Z = 2026-10-08 13:03（北京）（`d-d9-openai-130-commits.tsv`）；HN 50065932 创建 2026-10-13T02:46:43Z = 2026-10-13 10:46（北京） |
| 一手来源 | https://www.argmin.net/p/faster-fourier-transforms （已看原文）；https://github.com/openai/math/blob/main/lean/docs/130.md （已看原文；为 main 分支地址，非固定 commit） |
| 级 | Recht 个人博客（学者个人公开发言，类比 L2）；OpenAI 仓库文件为 L1 |
| 逐字摘句 | “Did I read this paper? Of course not.”（`d-d9-argmin.txt`）；“Unfortunately, I did not find a faster Fourier transform.”（同文件）；“The result is subsequential, with no all-length, bounded-coefficient, conditioning, or bit-complexity claim.”（`d-d9-openai-130.txt`）；“the model permits exact complex arithmetic and unrestricted coefficients”（同文件） |
| 状态 | 已找到 |
| 备注 | 派工单写的“他自称未完整读论文”与原句不符，应以原句为准（见勘误）。派工单称其为伯克利教授，页面未写其任职，未核。这是 1006 期的跟进，本条只记事实。 |

### D10 Rich Hickey：“AI” 不是工具；反驳文《AI Is a Tool. Using It Well Takes Skill.》

| 项 | 内容 |
|---|---|
| 一句事实 | Rich Hickey 在 LinkedIn 发文 “'AI' is Not a Tool”，主张 LLM 不是工具，而是“用来避免写代码的自动化”，并称 “LLMs are not tools.”。反驳文作者 Huahai 在 yyhh.org 发文，称 “A tool does not stop being a tool because it is difficult to control or easy to misuse.”。 |
| 北京时间（原时区） | Hickey 文 LinkedIn 页显示 “Published Oct 12, 2026”；反驳文称 Hickey 文为 “his October 11, 2026 article”（冲突，见勘误）；HN Hickey 帖 50060269 创建 2026-10-12T18:00:25Z = 2026-10-13 02:00（北京）；yyhh 文 HN 50063427 创建 2026-10-12T21:40:18Z = 2026-10-13 05:40（北京）；yyhh 页显示 “Mon 12 October 2026” |
| 一手来源 | Hickey：https://www.linkedin.com/pulse/ai-tool-rich-hickey-kjl8e （以匿名访客视图取得，已看原文，作者署名 Rich Hickey）；反驳文：https://yyhh.org/blog/2026/10/ai-is-a-tool/ （已看原文） |
| 级 | Hickey：个人公开发言（非公司员工，级别未套用，类比 L2，待定）；yyhh：个人博客，仅作观点原文，不作事实依据 |
| 逐字摘句 | “LLMs are not tools.”（`d-d10-hickey-linkedin.txt`）；“LLMs are not tools with which you write code - they are automation with which you avoid writing code”（同文件）；“In his October 11, 2026 article”（`d-d10-yyhh.txt`）；“A tool does not stop being a tool because it is difficult to control or easy to misuse.”（同文件）；“AI Is a Tool. Using It Well Takes Skill.”（同文件，标题） |
| 状态 | 已找到 |
| 备注 | “Clojure 作者”身份：Hickey 原页未写；yyhh 页仅有 “Clojure” 标签，未核实，正文勿写。 |

## 二、热度（HN Algolia，取数北京时间 10-13 19:29–19:32）

| 条目 | HN 帖（story id） | 积分 / 评论 | 发帖时刻（北京） | 备注 |
|---|---|---|---|---|
| D1 | 50059115 | 641 / 351 | 10-13 00:48 | 标题 “Muse Wonder”，链接 Meta 官方页 |
| D2 | 无 | — | — | 关键词 “X Subscription xAI Cursor” 无相关命中 |
| D3 | 50054751 | 376 / 403 | 10-12 19:22 | 标题 “QGIS enacts moratorium on AI contributions” |
| D4 | 50063365 | 240 / 116（19:29）；243 / 116（19:32） | 10-13 05:35 | 链接为 intel.com |
| D5 | 无 | — | — | 关键词 “AA-3D” 无相关命中 |
| D6 | 50064937（AA 文章） | 5 / 0 | 10-13 08:23 | Cognition 原帖 48451723（2026-06-08）256 / 57，仅作背景 |
| D7 | 50065692 | 222 / 112 | 10-13 10:07 | 标题 “AI models can provably hide arbitrary capabilities” |
| D8 | 50065312（Ars） | 7 / 3 | 10-13 09:13 | gamesindustry.biz 帖 50067543：3 / 0（10-13 14:23）；HN 投稿 50061195（Bloomberg 链接，标题 “Epic Games Delays Disney Project, Asks Developers to Use More AI”）：10 / 2（10-13 03:03）；HN 投稿 50069111（同一 Bloomberg 链接）：1 / 0（10-13 17:46）；HN 投稿 50062237（链接为 avclub.com，非 Bloomberg）：83 / 69（10-13 04:07）；取数北京时间 10-13 19:44，见 `d-heat-d8.json` |
| D9 | 50065932 | 158 / 106（19:29）；159 / 107（19:32） | 10-13 10:46 | 链接 argmin |
| D10 | 50060269（Hickey） | 281 / 147（19:29）；281 / 148（19:32） | 10-13 02:00 | 链接为 LinkedIn 页 |
| D10 | 50063427（yyhh） | 439 / 354 | 10-13 05:40 | 链接 yyhh.org |

数字随时间变化，以取数时刻为准（`d-heat.json`、`d-heat-search.json`）。

## 三、扫描说法勘误

1. D2：“Premium 及以上可用第三方模型”与 FAQ 的 “X Premium and higher tiers” 措辞不一，是否同指 $30 的 Premium 档，页面未明写。
2. D3：HN 标题与 “moratorium” 的说法不见于 QEP 原文；原文为“不允许”（no ... allowed），未写期限。PR #387 标题写 “QEP428”，修改的却是 qep-408 文件。
3. D4：派工单写的 “D270” 只见于 HN 标题与 intel.com 链接；搜索摘要统称 “Crescent Island”。规格、上市时间、价格均未核实。
4. D5：“55 项任务”原文只在主文章中以 “builds all 55 scenes” 出现，方法说明页明写 “AA-3D v1.0 has 55 tasks.”，已核；“人类盲测投票”由方法说明页 “through human preference voting” 支持，已核。派工单的表述可保留。
5. D6：“Opus 5.5 约 52%”为平均分（score），通过率为 58%；GPT-6.1 Sol 与 Claude Fable 5.1 同为 49%。基准作者为 Cognition，AA 为独立评测方，并非 AA 发布基准。
6. D7：“实验为小规模化学反应预测代理任务”不完整，论文另有子集奇偶（subset parity）实验；“NeurIPS 2026 论文”来自作者注记 “Accepted at NeurIPS 2026”，未核官方名录。
7. D8：“Epic CEO 回应称人类开发者仍负责创作”：所查 Sweeney 推文（仅此一条）未提此意，其强调的是 “moving developers around to retool for maximum AI acceleration”。Epic 通讯总监 Jake Jones 的声明（经 gamesindustry.biz 转述 Bloomberg）写明开发者是 “creative drivers”，因此应改写为“Epic 发言人称”，不宜写成 CEO。“人类开发者将管理与引导模型生成的代码与美术”是 Bloomberg 内部消息（经 Ars 转述）。“$1.5B 为 2024 年投资”：gamesindustry.biz 支持“投资”表述。Bloomberg 原报道未打开（403）。
8. D9：派工单写的 “他自称未完整读论文” 应改为原句 “Did I read this paper? Of course not.”，并说明他用聊天机器人提取要点。派工单称其为伯克利教授，页面未写。
9. D10：Hickey 文日期冲突：LinkedIn 页 “Published Oct 12, 2026”，yyhh 文 “his October 11, 2026 article”。“Clojure 作者”原页未写，未核。
10. 额外：D1 的 52/56 与 $4.52/$1.60 数字只见于 AA 页，Meta 页不含这些数字；Meta 原页有 “state-of-the-art” 自述（厂商口径）。

## 四、未完成与失败项

- D4：Intel 产品页与两条新闻稿均 403，未取得原文（见 `capture-log-d.md`）。
- D8：Bloomberg 原报道 403 挑战页，未取得原文；Sweeney 仅核一条推文，未核其后续线程（回复数 4088，未展开）。
- D2、D5：HN 无相关命中（已记录，不算失败）。
- 未触及付费墙或登录态页面；未使用镜像或转载站作为依据。
