# 1127 期 C 组（速览）取证

执行：Claude Haiku 5.5（模型 ID `claude-haiku-5-5`）。取证时刻：北京时间 11-28 23:32–23:35。未截图（C 组不需要）。未绕过验证码、付费墙或登录。存档与热度见 `capture-log-c.md`。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别见 `EDITORIAL.md`“事实分级”。

## 汇总表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 Google DeepCrossAttention 训练快 3 倍 | L1 | https://research.google/blog/supercharging-transformer-residual-connections/ ；https://arxiv.org/abs/2502.06785 | “obtains the same model quality up to 3x faster while adding a negligible number of parameters.”（arXiv 摘要） | 无 | 语言模型实验（LM1B、C4）；“up to”是上限；摘要语境为 “for a given training time”；博客时刻未给 | 已找到 |
| C2 Nielsen 撤下神经网络教材 | L1（作者本人声明） | https://neuralnetworksanddeeplearning.com/ | “I don't wish to help (even in a very small way) train people to contribute to this work, and so I've taken the book down.” | 无 | 原站署名 “November 2026”，无具体日；撤下日期：未注明 | 已找到 |
| C3 GPT-6.2 Astra 提出素描本作者 | L2/L3（作者本人历史学家的博客，Substack 署名） | https://generativehistory.substack.com/p/gpt-62-astra-solves-a-historical | “It did so extremely quickly (in just 22 minutes), using publicly available primary and secondary sources.” ；“probably made the travel drawings.”（模型输出原句，作者引用） | 无 | 北京时间 11-24 18:01；作者为测试方与核验方；模型版本为“发布前”测试；Ultra effort + goal mode | 部分支持（见下：“probably”与作者自我保留） |
| C4 DeepMind 质谱技能包 | L1 | https://deepmind.google/blog/interpreting-mass-spectra-with-google-antigravity/ | “accuracy remained below 4% across all three models” （无互联网、无技能）；“identified and verified the exact molecule in around 97% of attempts”（加技能） | 无 | 100 个化合物，取自 MoNA 中 Enamine REAL MS/MS 集，结构也见 PubChem；基准谱皆“available in public reference libraries”；每谱 3 次尝试，超 1 小时计失败；官方博客仅标日期 | 已找到（自测、非独立） |
| C4 免费开放 | L1 | 同上 | “offering 10,000 scientists the highest usage tier in Google Antigravity free of charge for 6 months (onboarded on a rolling basis), subject to eligibility, capacity, and selection. Applications are now open” | 无 | 申请制，按资格、容量与选择；滚动接纳 | 已找到 |
| C5 OS-Omni 跨平台基准 | L1（论文）/ L3 | https://arxiv.org/abs/2611.34232 ；HTML v1 https://arxiv.org/html/2611.34232v1 | “On v1, the strongest evaluated agent, Claude Opus 5.5, reaches 68.5% average success, trailing the 77.3% human reference by 8.8 points”（摘要）；正文：“Claude Opus 5.5 obtains the best Macro-Avg. success rate in Table 1 [...] 68.5%” | 无 | 提交 2026-11-25 19:55:12 北京（11:55:12 UTC）；v1 共 300 任务：六平台各 48 + 跨平台 12；68.5% 是宏平均（七列等权），微平均下 GPT-6-Astra 67.0%、Claude Opus 5.5 66.7%（正文原句） | 已找到（口径需写明宏平均） |
| C6 蚂蚁灵波 LingBot-Map 2.0 / LingBot-Navi | L1（官方页、HF）；量子位 L5 | https://technology.robbyant.com/lingbot-map-v2 ；https://technology.robbyant.com/lingbot-navi ；https://huggingface.co/robbyant/lingbot-navi | 官方页题名：“LingBot-Map 2.0 Reconsolidatable Streaming 3D Reconstruction”；“LingBot-Navi ... Scene‑aware Navigation with Spatial Memory”；HF：“License: apache-2.0”，“This repository contains the LingBot-Navi model weights.” | 无 | 官方页未标日期；量子位 2026-11-27 15:01:20 北京；HF Map-V2 集合为相对时间“updated 2 days ago”，未取精确时刻；Map 2.0 的许可证未核 | 部分支持（发布与开放入口已核；性能数字仅来自量子位，未核技术报告） |
| C7 Google Humanity's First Exam | L1 | https://blog.google/innovation-and-ai/models-and-research/google-deepmind/humanitys-first-exam/ | “The dataset consists of 544 elementary text and image prompts across 20 foundational capabilities.”；“Join our three-week Kaggle competition through December 15 and submit everyday questions that today’s models still get wrong.” | 无 | 博客标日期 Nov 24, 2026，官方未给时刻；Kaggle 竞赛至 12-15（原文未给时刻与时区） | 已找到 |
| C8 OpenAI 反诉苹果 | L4（CNBC、卫报、BBC 转述法庭文件；路透社原文未能打开） | https://www.cnbc.com/2026/11/27/openai-countersues-apple-for-stealing-trade-secrets-ft-reports.html ；https://www.theguardian.com/technology/2026/nov/27/openai-countersues-apple-trade-secrets ；https://www.bbc.com/news/articles/cm15ekqvlezeo | CNBC：“The ChatGPT maker filed counterclaims in California federal court on Friday”；OpenAI：“Given the timing, it now appears that stealing those documents may have been his reason for joining OpenAI in the first place.”；Apple：“OpenAI's counterclaims are a transparent attempt to distract from the substance of Apple's lawsuit,” | 无 | 反诉于 2026-11-27（周五，美国时间）提交加州联邦法院；CNBC 发布 11-28 02:56 北京（1:56 PM EST）；卫报 11-28 07:28 北京（6:28 PM EST，更新至 19:55 EST）；Apple 起诉为 7 月 10 日（CNBC、卫报） | 部分支持（事件与双方原句已由三家媒体核实；路透社原文与法院文件未取得） |

## 逐条一句事实与限定词

**C1 Google DeepCrossAttention**
- 一句事实：Google 研究院称，DeepCrossAttention（DCA）让模型以“同等质量”快至多 3 倍（语言模型实验），且只增加极少参数。
- 限定词原句：“up to three times faster with a negligible increase in parameters”（博客）；“the same model quality up to 3x faster”（摘要）。
- 核对：博客日期 November 23, 2026（官方未给时刻）。论文 arXiv:2502.06785 提交于 2025-02-10 18:58:52 UTC，即北京 2025-02-11 02:58:52；v2 2025-07-24 03:32:20 北京。扫描称“2025-02-10 提交”仅 UTC 日期正确。博客还写“reducing the number of parameters needed by up to 30-40%”，与“up to 3x”同为上限表述。
- 热度：HN 条目 50672565（“Google's DeepCrossAttention achieves comparable model quality up to 3x faster”），507 分，嵌套评论 178 条；提交于北京 11-28 07:06:29。
- 未核：论文正文是否把“3x”定义为训练时间或步数（只取了摘要与博客）。

**C2 Michael Nielsen 撤下教材**
- 一句事实：Michael Nielsen 在其《Neural Networks and Deep Learning》原站发表声明，称因不愿帮助培养参与前沿实验室工作的人，已下架该书。
- 限定词原句：“I don't wish to help (even in a very small way) train people to contribute to this work, and so I've taken the book down.”
- 核对：原站署 “November 2026”，无具体日期，撤下日期记为“未注明”。服务器 Last-Modified 为 2026-11-27 16:45:15 GMT（北京 11-28 00:45:15），仅为线索（L5 级别的推断），不作为撤下时刻。原站未说明镜像或其他渠道。
- 热度：HN 50670757（“Michael Nielsen took down his book on neural nets”），614 分，嵌套评论 404 条；提交于北京 11-28 03:31:01。扫描称“437 评”，与嵌套计数 404 不同，口径未明，以 heat 计数为准。

**C3 GPT-6.2 Astra 素描本作者**
- 一句事实：历史学家 Mark Humphries 在 Substack 撰文称，测试中 GPT-6.2 Astra 用约 22 分钟，推断出一本 1808 年匿名素描本“probably”的作者为 Chabot 子爵，作者随后以档案材料核验。
- 限定词原句：“It did so extremely quickly (in just 22 minutes), using publicly available primary and secondary sources.”；“probably made the travel drawings.”；作者随后称日记“an anonymous diary attributed to Chabot”。
- 限定与核对：
  - 测试为“发布前”（“before it was released”）；设置为 Ultra effort 与 goal mode；作者先给了模型自己的文章与两名候选人（文章被作者称为“misleading”）。
  - 日记等材料由作者向档案馆索取并核验，核验方即提出假说的作者本人，非独立核验。
  - 另一对照：Fable 5.2 在约 35 分钟得出同一结论，但没有找到 Chabot 日记。
  - 时刻：Substack post_date 2026-11-24 10:01:24 UTC，即北京 11-24 18:01:24。
- 热度：HN 50643158（“GPT 6.2 Astra Solves a Historical Mystery”），485 分，嵌套评论 308 条；提交于北京 11-25 23:16:53。
- 与扫描一致：“约 22 分钟”（实为 22 分 1 秒）；“另一模型的对照结果”（Fable 5.2，约 35 分钟）。

**C4 DeepMind 质谱技能包**
- 一句事实：Google DeepMind 博客称，在 100 个已有公共参考谱的化合物上，无互联网与无技能时三款模型准确率低于 4%，加入 Mass Spec Skills 后约 97% 次尝试能识别并核验出确切分子；并向 1 万名科研人员提供 6 个月最高用量档（需资格审核与名额）。
- 限定词原句：“accuracy remained below 4% across all three models”；“identified and verified the exact molecule in around 97% of attempts”；“subject to eligibility, capacity, and selection”。
- 口径：样本是 MoNA 中 Enamine REAL MS/MS 集的 100 个化合物（结构也在 PubChem），即“库内已有参考谱”；每谱 3 次尝试；超过 1 小时计失败；模型为 Gemini 3.8 Flash、Gemini 4 Argon、Gemini 4 Carbon；由 Google 自测。博客示例（图 2）是库内无高置信匹配的情形，与 97% 基准不同集合。
- 时刻：博客 November 25, 2026（官方未给时刻）。HN 50647716：提交于北京 11-26 05:21:00，3 分，0 条评论。

**C5 OS-Omni**
- 一句事实：OS-Omni（arXiv:2611.34232，NeurIPS 2026 投稿）v1 含 300 个跨六平台任务，论文报告最强智能体 Claude Opus 5.5 宏平均成功率 68.5%，人类参考为 77.3%。
- 限定词原句：“reaches 68.5% average success, trailing the 77.3% human reference by 8.8 points”；正文 “Macro-Avg.”。
- 口径：68.5% 是宏平均（七列等权，含跨平台列）；微平均（每任务等权）下 GPT-6-Astra 为 67.0%、Claude Opus 5.5 为 66.7%。
- 时刻：arXiv v1 提交于 2026-11-25 11:55:12 UTC，即北京 19:55:12。
- 未核：扫描称 “HF 日榜 11-27 第 2”，本轮未取 HF 日榜，未核。

**C6 蚂蚁灵波 LingBot-Map 2.0 / LingBot-Navi**
- 一句事实：蚂蚁灵波（Robbyant）官方页列出 LingBot-Map 2.0（流式三维重建）与 LingBot-Navi（带空间记忆的导航模型）的技术报告、代码与 Hugging Face 入口；HF 上 LingBot-Navi 模型卡标注 apache-2.0 许可。
- 限定词：官方页未标发布日期；量子位文中的性能数字（如 GOAT-Bench 成功率 56.09%）未在官方页或技术报告中核对，不写入速览。
- 时刻：量子位 2026-11-27 15:01:20 北京。
- 对照：量子位称“两款「空间记忆」原生基模”，官方页与之一致。

**C7 Google Humanity's First Exam**
- 一句事实：Google DeepMind 博客宣布在 Kaggle 发布 HFE 基准，含 544 道基础文本与图像题（20 项基础能力），并举办至 12 月 15 日的三周竞赛，征集模型仍答错的日常问题。
- 限定词原句：“544 elementary text and image prompts across 20 foundational capabilities”；“submit everyday questions that today’s models still get wrong”。
- 时刻：博客 Nov 24, 2026（官方未给时刻）。竞赛截止 “through December 15”（未给时刻与时区）。
- 热度：HN 关键词查询（“Humanity's First Exam”）命中 2 条：50633316（3 分，0 评，11-25 03:34）、50640241（Kaggle 页，1 分，1 评，11-25 17:18）。热度低。

**C8 OpenAI 反诉苹果**
- 一句事实：OpenAI 于 11-27（周五）在加州联邦法院对苹果提出反诉，指苹果通过再雇用前员工获取其机密信息，并阻碍硬件供应商为 OpenAI 设备生产；苹果回应称这是“分散注意力”的做法。
- 限定词原句：“The ChatGPT maker filed counterclaims in California federal court on Friday”（CNBC）；“transparent attempt to distract from the substance of Apple's lawsuit”（苹果）。
- 未取得：路透社原文（401，验证码页，不绕过）；法院文件（未尝试 CourtListener，不作为依据）。
- 热度：HN 关键词查询命中 4 条，均为 4–8 分、0 评：50672949（FT，8 分，11-28 08:06）、50671976（CNBC，4 分，11-28 05:44）、50678061（卫报，4 分，11-28 22:27）、50677365（BBC，4 分，11-28 21:24）。
- 扫描称“据路透社报道”：Reuters 无法打开，改以 CNBC、卫报、BBC 的同一事件报道为 L4 依据；速览若要写“据路透社报道”，需 Develata 另行提供原文。

## 扫描说法勘误

- C1：扫描称“up to 3x faster”——原文一致，但它是“同等质量”的上限（摘要 “same model quality up to 3x faster”）。HN 标题写成 “comparable model quality”，与论文用词不同，不要照抄。论文“2025-02-10 提交”是 UTC 日期，北京为 02-11 02:58。热度数字（506/175）为扫描时刻，本轮为 507/178。
- C2：扫描称“撤下日期”未给；原站只写 “November 2026”。扫描称 HN “437 评”，嵌套评论计数为 404，口径不同。
- C3：扫描称“约 22 分钟”正确（22 分 1 秒）；“归属对象”原文为 “probably”，并非定论，作者随后自己也用 “attributed to Chabot”。“另一模型”为 Fable 5.2（约 35 分钟，未找到日记）。
- C4：扫描的“100 个库内已有参考谱的化合物”正确；“不足 4%”仅指无互联网、无技能组；“约 97%”为加技能组，且以库内已有谱为样本（博客称 “available in public reference libraries”）。
- C5：扫描称 68.5% 为宏平均，正确；“Claude Opus-5.5”正文写作 “Claude Opus 5.5”。微平均下 GPT-6-Astra 67.0% 高于 Claude 66.7%，速览若写“最强”需注明口径。
- C6：扫描称“LingBot-Map 2.0 / LingBot-Navi”，官方页名称一致；量子位日期 11-27 正确。
- C7：扫描的 “544 道题”“20 项能力”“12-15 截止”均与原文一致；原文为 “elementary” 与 “everyday” 题，速览勿写成“难题”。
- C8：扫描称 “反诉”正确（11-27 周五，加州联邦法院）；“据路透社报道”本轮未能核对路透社原文；CNBC、卫报、BBC 均为转述。

## 失败与未取得

- Reuters 原文（C8）：HTTP 401，验证码页，未绕过。
- Reddit 热度（`barking heat --reddit`）：2 次搜索被重定向到登录页，失败，已记入 `c-kw-heat.json`。
- C6 性能数字（量子位）与 C5 HF 日榜：未核。
- C7 博客时刻、C4 博客时刻、C1 博客时刻：官方只给日期，未给时刻。

## 对读者

本组不要求。C 组的“对读者”不做。
