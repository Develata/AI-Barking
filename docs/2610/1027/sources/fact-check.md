# 1027 期事实清单

## 省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Google 发布实时 AI 世界 Genie-4；据彭博社报道，两分钟是上限，研究员说跑满也不保证前后一致 | L1/L4 | ⚠️ | 见 A 节；a-blm-linea.txt | 彭博社（西语版 Bloomberg Línea）：“alcanzar el límite de dos minutos del modelo no significa necesariamente que una escena se mantenga completamente consistente”——“两分钟上限”与研究员发言同出一篇彭博稿（反向核验第6条）。 |
| Anthropic 实验里，研究者暗中让 AI 偏向某个选项，分析师的中间选择86%跟着走，最终建议的差别按论文主算法算不上显著 | L1 | ✅ | 见 B 节 | “中间选择”= four analytical choices；“暗中”= hidden system prompt，由研究者设定；“主算法”= registered primary, covariate-adjusted estimator（未调整估计单侧 p=0.037 显著，故必须保留此限定）。 |
| AA 综合指数换了考卷，Kimi 从55分变46分 | L3 | ✅ | 见 C 节 | 55 为 v4.3.2，46 为 v5。 |

## A Genie 4

A 组取证：Claude Sonnet 5.5（evidence-a.md）。Claude 已逐字抽查下列原句。取证方与执行方同家族，反向核验须逐行核对本节与存档。博文页面 “October 27, 2026”，官方未给时刻；Google 博客北京 10/28 00:00（16:00Z）；HN 帖北京 10/28 00:06。博文 JSON-LD dateModified 为北京 10/28 20:01，首发后改过，无旧版存档。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Google DeepMind 28日凌晨发布世界模型 Genie-4：一句话生成能实时走动的世界，还能随时改天气、加人 | L1 | ✅ | https://deepmind.google/blog/genie-4-a-new-generation-of-world-simulation/ ; a-blog.txt:498、516、520 | “Today, we are announcing Genie 4, our new world model”；“users use standard keyboard controls for navigation and camera control, and text prompts to steer the scene”；“you can use natural language to modify a simulation in real-time — like changing the weather, moving the camera, or introducing a new person or object”。日期：@GoogleDeepMind 首帖 2026-10-27T16:10:55Z（a-fx-2115114028338962672.json，北京 10/28 00:10）；HN 帖 16:06Z（a-heat.json，北京 00:06）；Google 博客 16:00Z 为取证记录，JSON-LD 未单独核。正文写“28日凌晨”（反向核验第5条）。 |
| 自带声音 | L1 | ✅ | a-blog.txt:578、580 | “Genie 4 is our first real-time world model to generate audio natively”；“Genie 4 introduces interactive dialogue that adapts on the fly”。“第一个”是 Google 自家模型范围内。 |
| 据 TechCrunch 报道，场景最长保持一致约两分钟 | L4 | ⚠️ | https://techcrunch.com/2026/10/27/ （a-tc.txt） | “It can maintain a consistent scene for up to two minutes, double its predecessor’s limit”。The Verge：“about two minutes long, up from one minute for the previous Genie 3 model”（a-verge.txt）。Google 书面材料无“两分钟”：博文 “over short sessions”，模型页 “The model can support a few minutes of continuous interaction, rather than extended sessions.”（a-model-page.txt:702；原先误引 Genie 3 页的 “currently … extended hours”，反向核验第10条已改）。“一分钟”对应 Project Genie 产品 “Limitations in generations to 60 seconds”（a-projgenie.txt）；Genie 3 官方页亦写 “a few minutes”。 |
| 目前只给少数受邀测试者 | L1 | ✅ | a-blog.txt；a-googleblog.txt | “Genie 4 has been released as a research preview to trusted testers, and is coming soon to Google AI Ultra subscribers in Project Genie and early access partners in the Gemini API”。未见日期与价格。 |
| 机器之心标题写成“游戏引擎已死？” | L4 | ⚠️ | https://news.pedaily.cn/202610/570971.shtml ; a-pedaily.txt；images/12-jqzx-engine-dead-headline.png | 投资界转载机器之心稿，标题“游戏引擎已死？谷歌Genie 4刚刚发布，实时操控整个世界”，2026-10-28 10:40+08。同文正文：“距离一个能够无限运行、完全符合现实物理规律的虚拟世界仍有明显距离”。仅核实转载页标题存在；按分级表 L4 标 ⚠️（反向核验第11条）。 |
| 两分钟是上限：据彭博社报道，Google 研究员 Hang Qi 说，跑满两分钟也不代表场景全程一致 | L4 | ⚠️ | Bloomberg 西语版 a-blm-linea.txt；财联社 a-cls.txt | “Hang Qi, ingeniero de investigación de Google DeepMind, afirmó que alcanzar el límite de dos minutos del modelo no significa necesariamente que una escena se mantenga completamente consistente en todo momento”；财联社：“即便模型能够运行至两分钟的上限，也不意味着场景在整个过程中都能保持完全一致”。Bloomberg 英文原文付费墙，未绕过；西语版为 Bloomberg Línea。 |
| Google 自己的页面没写两分钟，只说能连续玩“几分钟” | L1 | ✅ | a-model-page.txt；a-blog.txt；images/05、08 | 模型页 “The model can support a few minutes of continuous interaction, rather than extended sessions.”（a-model-page.txt:702）；博文 “visual details and object permanence can still shift or drift over time”；模型页局限另列复杂物理交互、可读文字、空间精度、多主体等。08 号图为 DOM 克隆白底截图（原页深色），文字未改。 |
| （已删，读者稿检查称不在乎、字数）官方演示视频注明“片段已缩短” | L1 | ✅ | a-model-page.txt；images/02 | “Results for illustrative purposes. Sequences shortened.” 博文称视频为 “recordings of real-time interactions”。未说明挑选标准。 |
| HN 热度 | — | ✅ | a-heat.json | 927/536（北京 10/28 20:30）。 |

## B 分析师实验

B 组取证：Claude Sonnet 5.5（evidence-b.md）。Claude 已逐字抽查研究页 b-page.txt 与论文 b-paper.txt 的下列原句。取证方与执行方同家族，反向核验须逐行核对本节与存档。研究页 Oct 27, 2026，官方未给时刻（JSON-LD dateModified 2026-10-27T19:17:23Z，北京 10/28 03:17）；研究页标题 “How AI agents shape experts’ judgment calls”，论文标题 “Who's Aligned to Whom?”。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 发布实验：50名职业分析师用 Claude Code 评估同一个农场项目 | L1 | ✅ | https://www.anthropic.com/research/agents-judgment-calls ; b-page.txt:41、77、111 | “we asked 50 professional analysts to do a realistic statistical analysis with Claude Code. They all had experience in causal inference”；“Out of 61 participants, 11 chose not to use the agent and are left out of the analysis”；“it was specifically about farming”。数据为合成数据（b-paper.txt “built on synthetic data”）。招募自预筛的付费专家平台。 |
| 研究者抛硬币决定 AI 偏向哪边，没告诉分析师 | L1 | ✅ | b-page.txt（“coin flip” 两处） | “Which way the agents would ‘lean’ would be decided by coin flip”；“which the analysts weren’t told about”；“We gave the agent its leans in hidden instructions (a system prompt)”。论文为32种配置的随机区组，研究页简称 coin flip。 |
| 实验的四个中间选择里，86%的情况下分析师选了 AI 偏向的那边；不受影响应约一半 | L1 | ✅ | b-page.txt:45、83 | “Across the four analytical choices, analysts went with their agent’s lean 86% of the time. If the agent’s input made no difference, we’d expect about 50%.” 分项 92%、82%、81%、90%。单位是“分析师×决策”的试次（论文表2 n=195，分母为推断）。实验共五个决策，第五个是最终建议。 |
| 86%算的是中间选择。最终“要不要推广”上，AI 偏向推广时54%的人建议推广，偏向反面时36% | L1 | ✅ | b-page.txt:85 | “When the agent leaned toward rolling out the program, 54% of analysts recommended it, compared with 36% when it leaned the other way. (A one-year extension counts as not rolling out.)” 54/36 只在研究页图2与此段。 |
| 论文把建议分三档打分再比，按其登记的主要算法，差距不算显著 | L1 | ✅ | b-paper.txt:360–364、442–446 | 检验对象是三档打分（“discontinue 0, extension 0.5, rollout 1”）的均值差 policy shift，不是54%/36%的二元比例差；正文已分开表述（反向核验第1条 BLOCKER）。 “Under the registered primary, covariate-adjusted estimator, the policy shift is not statistically significant (95% CI −6% to 40%, one-sided p = 0.069); the unadjusted estimate has one-sided p = 0.037.” 未调整估计显著，正文须保留“预先登记的主要算法”限定。注册时已有17人开始任务（evidence-b.md）。 |
| 分析师先表态、AI 再反对时，60%改了主意，AI 赞同时6%；两种情况合计只有38个案例，Anthropic 称估计不精确 | L1 | ✅ | b-page.txt:45、89 | “In 38 cases, an analyst had expressed a preference … before the agent expressed its lean. When the lean went against their choice, they switched 60% of the time, compared with 6% when it agreed”；“because this rests on only 38 cases, the estimate is imprecise, and the 95% confidence interval runs from 24 to 82 percentage points. We also can’t be sure that the difference is caused by the AI’s influence”。单位是案例（试次），不是人。 |
| （已删，字数）77%的人复述了 AI 的论点，只有15%注明来自 AI | L1 | ✅ | b-page.txt；b-paper.txt | “77% repeated their agent’s argument in at least one answer, sometimes word for word, and only 15% credited the agent anywhere.” 论文 “77% (37 of 48)”——分母48是答问卷者；“至少一个答案里”。 |
| 这次实验只测了一个任务、一个模型，AI 的偏向是特意加强的 | L1 | ✅ | b-page.txt:109、111 | “We deliberately strengthened the agents’ lean.”；“We studied one task, in one field, with one model … the agent ran on Claude Opus 4.8.” 研究页与论文均写 High effort。无指定倾向时 Opus 4.8 在123次中112次做出相同五个选择。 |
| （已删，字数）另有11人自己选择不用 AI，6人犯了让结果作废的严重错误；用 AI 的50人中3人 | L1 | ✅ | b-page.txt:103、105 | “Of the 11 participants who chose not to use the agent, six (55%) made serious methodological errors that invalidated their results, compared with three of the 50 (6%) who did use it.”；“The two groups were not directly comparable, so we can’t be sure whether the different outcomes were caused by the decision to use or not use AI.” 自选、非随机。 |
| （已删，字数）新智元标题称“86%顶尖专家关键决策被 AI 控制” | L4 | ⚠️ | https://eu.36kr.com/zh/p/4045003602007172 ; b-36kr-zh.txt；images/23-36kr-xinzhiyuan-headline-v2.png | 标题原文“现实版「思想钢印」，Anthropic惊悚实验曝光，86%顶尖专家关键决策被AI控制”，经36氪转载，2026-10-28 12:26（页面无时区，推断北京时间）。是媒体原句的存在性（✅ 指标题确实如此），不是事实背书。 |
| HN 热度：仅见2分/0评转帖 | — | ✅ | b-heat.json | 扫描所称162/150 查不到；id 50271917 提交于北京 10/28 19:33。热度主要在中文媒体（新智元、财联社快讯、XiaoHu.AI）。 |

## C AA v5【跟进1023】

C 组取证：Claude Sonnet 5.5（evidence-c.md）。Claude 已逐字抽查下列原句。取证方与执行方同家族，反向核验须逐行核对本节与存档。v5 公告与 Changelog 只写 “October 27, 2026 / 27 Oct 2026”，官方未给时刻；AA 官方社交帖北京 10/28 10:11:22（02:11:22Z）为最早可证宣布时刻。站点切换时刻无一手来源：1026 期 AA 存档（北京 10/27 22:52–23:01）仍为 v4.3.2。

本号旧数字的版本（C3）：1023 期“Kimi 55分、列第四”取自 v4.3.2（存档北京 10/24 19:52–20:18）；1026 期“Terra 最高档48、上一代42、Sol 中档48”取自 v4.3.2（存档北京 10/27 22:52–23:01）。两期提交均早于 v5 宣布（1023 约3.4天，1026 约8小时），属发布后口径过期，不是当时引错版本，不需要 CORRECTION.md；正文以【跟进】交代。1026 现值：Terra(max) 51、每任务 $0.96；Sol(medium) 46；Sol(high) 50；GPT-5.6 Terra 现为 N/A。“同分更便宜”的具体数字失效，论点换档位后近似成立，不写“1026 结论翻转”。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 评测机构 Artificial Analysis 发布综合指数第5版（换了多项测试，正文已删此半句；日号已删，第二轮第6条） | L3 | ✅ | https://artificialanalysis.ai/articles/artificial-analysis-intelligence-index-v5 ; c-aa-v5.txt；https://artificialanalysis.ai/changelog ; c-aa-changelog.txt | 新增 FrontierCode 2.0（15%）、Terminal-Bench-Science（15%）；HLE 换为 HLE-Diamond，Omniscience v1.1、CritPt v2 hard subset；删除 GDPval-AA v2.1、SciCode、AA-LCR v1.1；四大类各25%（v4.3.2 为30/20/20/30）。“28日”按北京时间：@ArtificialAnlys 宣布帖 2026-10-28T02:11:22Z（c-fx-2115265133467635877.json，北京 10:11）；页面日期为美国 27日。 |
| 总分六成来自题目或答案不公开的测试 | L3 | ✅ | c-aa-v5.txt | “With v5, 60% of the Index is now based on evaluations with private test sets or held-out answers.” v4.3.2 按方法页 Private 列加总约45%（推算，不上正文）。 |
| 本号1023期写过 Kimi-K3.1-Pro 在旧版指数得55分，新版得46分 | L3 | ✅ | docs/2610/1023/doc_1023_publish.txt:9；c-aa-m-kimi.txt；c-aa-v5.txt | 1023 原句“Artificial Analysis 综合指数55分、列第四”（v4.3.2）。v5：“Kimi K3.1 Pro scores 46 on the Artificial Analysis Intelligence Index”；模型页 #20/51。推理档位：AA 页未分档（单一设置）。 |
| （已删，字数）GPT-6.1-Terra 开最高档从48分变51分 | L3 | ✅ | c-v5-m-terra-max.txt；docs/2610/1026/sources/b-aa-article.txt | 1026 期写 48（v4.3.2，max）；v5 max 档 51。同模型同档跨版，仅陈述两版数字，不作相减比较。 |
| （已删，字数）蚂蚁 Ling-3.1-flash-1019 的官方介绍仍写旧版44分，新版29分 | L1/L3 | ✅ | https://huggingface.co/inclusionAI/Ling-3.1-flash-1019 ; c-hf-ling.txt；c-aa-m-ling.txt | 模型卡：“it scores 44 on the Artificial Analysis Intelligence Index v4.3.2, up from 41 for its predecessor”（卡上写明 v4.3.2，“仍写”指未更新为 v5）；AA：“Ling 3.1 Flash 1019 scores 29 on the Artificial Analysis Intelligence Index”。HF 现查北京 10/28 20:3x。 |
| AA 方法页写，各版分数不用于跨版本长期比较 | L3 | ✅ | https://artificialanalysis.ai/methodology/ （Progress Index 段）；c-aa-progress.txt | “Each version of the Intelligence Index is therefore measured on a different set of evaluations, and scores across versions are not intended for long-run comparison.” 出处是 Progress Index 方法页，v5 公告本身没有这句。 |
| 公告却说，开放模型最好的落后榜首12分，旧版只差3分 | L3 | ✅ | c-aa-v5.txt | “The best, Kimi K3.1 Pro (weights not yet released), scores 46, 12 points behind the leader, compared with a 3-point gap in v4.3.” 旧版“3分”是按 Kimi 承诺放权重后计算的。 |
| 这个“开放模型最好的”是 Kimi-K3.1-Pro，公告也注明它权重还没公开，却仍算进开放模型；AA 模型页把它归为闭源 | L3 | ✅ | c-aa-v5.txt；c-aa-m-kimi.txt | 公告“Kimi K3.1 Pro (weights not yet released)”；模型页 FAQ：“No, Kimi K3.1 Pro is proprietary. The model weights are not publicly available.” HF 现查（北京 10/28 20:31）无 K3.1 Pro 权重；1023 存档官方称最迟 11/4 公开。 |
| （已删，字数）已公开权重的模型里，最高是 DeepSeek-V4.1-Flash-1017 和 GLM-5.3，各32分 | L3 | ✅ | c-aa-v5.txt | “Of models with released weights, DeepSeek V4.1 Flash 1017 (max) and GLM-5.3 (max) score highest at 32”。均为 max 档。 |
| HN 热度 | — | ✅ | c-heat.json | 69/72（v5 帖，北京 10/28 11:06 提交）。 |

扫描说法勘误要点：“国泰海通、界面、腾讯 10-28 仍写55分”——找到的55分稿都早于 v5 官宣，界面、腾讯 10-28 稿未找到，这一吠点不成立，正文不写；FrontierCode 2.0 表里是 Kimi K3（44.2→17.7）不是 K3.1 Pro；Ling config.json 262144 读数属实但不能推出“1M 不成立”，正文不写。

## 速览

D 组取证：Claude Haiku 5.5（evidence-D.md）。Claude 已逐字抽查下列原句与存档一致：药研六家与“mostly preclinical”、Survival 四档结果表、Ai2 93.5%→40.9% 与 Human-Only 约7点、LFM 许可证门槛、Replit “Pro & Enterprise”。取证方与执行方同家族，反向核验须逐行核对本节与存档。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 称与6家机构合作研发药物，多数仍在临床前阶段 | L1 | ✅ | https://www.anthropic.com/research/early-progress-drug-development ; d-d1-anthropic-drug.txt | “deep research partnerships with six organizations developing new medicines: BigHat Biosciences, Bristol Myers Squibb, Divergence Labs, Enveda, Generate Biomedicines, and The Column Group.”；“so far it is mostly preclinical—that is, for the most part, it isn’t yet at the stage where the drugs are being tested in humans.” 页面 Oct 27, 2026，官方未给时刻；HN 246/246（北京 10/28 04:50）。不上卡的口径：博文“roughly 10 times”对应 Generate 报告 day-1 12-fold、另一测定 8.4-fold；约35,000个是“antibody designs”，预印本为 23,600 scored / 57 nominated；BigHat 1,010 是变体数、1,232 是孔数，不是冲突；申报文书草稿是 Enveda 的。 |
| Survival LLM 开发者旧版自测22道应急题，两档 Mac 配置对21题 | L1 | ✅ | https://www.brangus.io/survival-llm ; d-d2-brangus-survival.txt | 两档 Mac 配置 = Mac 128GB+ 与 Mac 32–96GB（反向核验第7条：原句“高配”会把 Windows 32GB+ 的19/22读进去）。结果表：Mac 128GB+ Nemotron 3.5 Super 21/22；Mac 32–96GB Qwen3.8 27B 21/22；Windows 32GB+ Nemotron 3.5 Lightning 19/22；Mac/PC 16–31GB Granite 4.2 8B 14/22。“graded blind, in our tests on an earlier build of the library”——开发者自测、旧版资料库。首次须联网下载：“a 9 GB download for everything (19 GB once installed)”。0.3.3/0.3.4 为 October 27, 2026，官方未给时刻；HN 392/148（北京 10/28 03:55）。未找到“不能代替急救”类原句。标签“离线应急”。 |
| Axiom 称其团队指挥 AI 代理，24天用 Lean 形式化有限单群分类定理 | L1 | ✅ | https://axiommath.ai/research/the-enormous-theorem-formalized/ ; d-d3-axiom-blog.txt；https://github.com/AxiomMath/CFSG ; d-d3-axiom-cfsg-gh.txt | “Over 24 days, engineers and mathematicians at Axiom Math directed swarms of dozens of AI agents”；README “A sorry-free Lean4 … proof of the Classification of Finite Simple Groups (CFSG) … We primarily follow the first generation proof”；陈述取自 TauCeti；网站 43.1M 行、2.05M 声明；博文称与文献有34处偏差。博文 October 26, 2026；仓库创建北京 10/27 21:19，仅1个提交。未见独立构建或外部审计（D 组所查范围）。HN 218/68。 |
| Ai2 称一项实验中，提示里告不告知评审结论，模型对同一批配对的判断过半改变 | L1 | ✅ | https://allenai.org/blog/novelty-bench ; d-d6-ai2-novelty.txt；论文 https://arxiv.org/abs/2610.02022 ; d-d6-arxiv-abs.txt | “In one experiment, changing just a few words in a prompt shifts a model’s accuracy in identifying which of two ideas is more novel by more than 50 percentage points.” 具体：GPT-5.4、Human+Generated 配对、弱标签，93.5% → 40.9%；Human-Only 数据集同一改动约7个百分点。改动内容：“One prompt told the model that reviewers had found exactly one idea in each pair to be novel and asked it to identify which one”（d-d6-ai2-novelty.txt:127），论文摘要同：“telling the judge that reviewers found one idea novel and the other not”。原句“改几个词”取自博文导语，反向核验第9条认为会被读成同义改写，改为写明改的是评审结论。博文 October 27, 2026；arXiv v1 10/01、v3 10/26。无 HN 当期帖。 |
| Liquid AI 开放6款2B级模型，年收入千万美元以上企业商用不在许可内 | L1 | ✅ | https://huggingface.co/blog/LiquidAI/lfm3 ; d-d4-liquid-lfm3.txt；d-d4-lfm-license.txt | “six 2B-class models (text, vision, audio, omni, and decision)”；LFM3-1.7B 为1.7B。许可证 LFM Open License v1.0：“Threshold” = “annual revenue of 10 million United States dollars ($10,000,000) or more”；“Any Commercial Use … by a Legal Entity that exceeds the Threshold is not licensed under this Agreement.” 模型卡称因体积小不推荐复杂 Agent 编码。October 27, 2026；HN 147/57（北京 10/28 01:04）。 |
| Replit Pro 与企业版：非工程同事可让 Agent 改代码，工程师定上线 | L1 | ✅ | https://replit.com/blog/anyone-can-bring-ideas-to-company-codebase ; d-d5-replit-cloud.txt | “now available for all Pro & Enterprise customers”；“the agent opens a pull request (PR) in GitHub or a merge request (MR) in GitLab”；“engineering still decides what ships”。原文无“merge 由工程团队完成”字样，卡片写“上线与否由工程师定”。Oct 27, 2026，官方未给时刻；无 HN 当期帖。 |

未收：D7 OpenAI×Oracle（13万、9.5万是 Oracle 自家员工用户，圈内题）；D8 Apple M6（硬件新品，同 1026 判断；“每个核心带神经加速器”在新闻稿中，扫描说错，“2 纳米”未找到）。

## 批注译注（图上额外事实）

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 40：模型能支持几分钟的连续互动，而不是长时间的会话 | L1 | ✅ | a-model-page.txt:702 | “The model can support a few minutes of continuous interaction, rather than extended sessions.” 底图为 DOM 克隆白底截图。 |
| 41：它能让场景保持一致最长两分钟，是上一代上限的两倍 | L4 | ⚠️ | a-tc.txt | TechCrunch 原句 “It can maintain a consistent scene for up to two minutes, double its predecessor’s limit”；图上标题与页脚标明 TechCrunch。 |
| 42：我们已开始以研究预览形式向受邀测试者推出 Genie 4 | L1 | ✅ | a-blog.txt | “We have started rolling Genie 4 out as a research preview for trusted testers — gathering feedback before we release it to consumers and developers, starting with …”（译注只译 quote 高亮的前半句，后半开放计划不摘，第二轮第3条）。 |
| 43：四个分析选择合计，分析师有86%的情况选了代理偏向的那边；如果代理的意见毫无影响，预期约为50% | L1 | ✅ | b-page.txt:45 | 原句见 B 节。 |
| 44：代理偏向推广时，54%的分析师建议推广；偏向另一边时为36% | L1 | ✅ | b-page.txt:85 | 原句见 B 节。 |
| 45（移入备用：手机上字太小，视觉复核）：按已登记的主要（协变量调整）估计，最终建议的偏移在统计上不显著 | L1 | ✅ | b-paper.txt:444–446 | 底图同屏可见“(95% CI −6% to 40%, one-sided p = 0.069); the unadjusted estimate has one-sided p = 0.037”。 |
| 46：代理意见与其原选择相反时，他们60%改了选择；一致时为6%；差距为54个百分点，但只基于38个案例，估计不精确 | L1 | ✅ | b-page.txt:89 | 原句见 B 节。 |
| 47：我们有意加强了代理的偏向；我们只研究了一个任务、一个领域、一个模型 | L1 | ✅ | b-page.txt:109、111 | 原句见 B 节。 |
| 48：（开放权重模型中）最好的 Kimi K3.1 Pro（权重尚未发布）得46分，落后第一名12分；v4.3 时差距为3分 | L3 | ✅ | c-aa-v5.txt:104 | “Open weights models fall further behind in v5. The best, Kimi K3.1 Pro (weights not yet released), scores 46, 12 points behind the leader, compared with a 3-point gap in v4.3.” 括号“开放权重模型中”取自前一句。 |
| 49：不同版本的分数不打算用于跨版本的长期比较 | L3 | ✅ | c-aa-progress.txt；https://artificialanalysis.ai/methodology/progress-index | “scores across versions are not intended for long-run comparison” |

## 读者稿检查

见 `style-review.md`（Codex gpt-6-astra，medium，只读沙箱）。采纳：换标题为“游戏引擎已死？谷歌AI世界撑两分钟？”，省流与 B、C 段去术语补主语，删“片段已缩短”一条。

## 反向核验

### 第一轮（正文与事实清单）

复核方：WSL Codex gpt-6-astra（medium，只读，联网），`barking review --reverse`，北京 10/28 晚。取证方为 Claude 系（Sonnet、Haiku），复核方为 GPT，按提示词第2项逐行核对清单与存档。结论：1 BLOCKER、10 SHOULD_FIX、2 NEEDS_VERIFICATION。Claude 逐条核实，全部成立：

1. BLOCKER 采纳：54%/36% 是二元比例（延期计为不推广，b-page.txt:85），p=0.069 检验的是三档打分均值差（b-paper.txt:360–364）。吠点①改为两项分开写：“论文把建议分三档打分再比，按事先定好的主要算法，差距不算显著”。
2. SHOULD_FIX 采纳：标题已不含86%；小标题改“分析师的选择86%跟着 AI？”，正文“86%的情况下”。
3. SHOULD_FIX 采纳：补“AI 赞同时6%；两种情况合计只有38个案例”。
4. SHOULD_FIX 采纳：C 吠点②补“公告也注明它权重还没公开，却仍算进开放模型”。
5. SHOULD_FIX 采纳：Genie 日期改“28日凌晨”（@GoogleDeepMind 首帖与 HN 帖均为北京 10/28 00:0x，存档 a-fx-2115114028338962672.json、a-heat.json）。
6. SHOULD_FIX 采纳：省流点名彭博社。
7. SHOULD_FIX 采纳：Survival 速览改“开发者旧版自测……两档 Mac 配置对21题”。
8. SHOULD_FIX 采纳：Replit 速览补“Pro 与企业版”。
9. SHOULD_FIX 采纳：Ai2 速览改为“提示里告不告知评审结论”；“弱标签”限定字数放不下，保留“一项实验中”，弱标签写在本表备注。
10. SHOULD_FIX 采纳：模型页原句改为 a-model-page.txt:702 的真实原句（原误引 Genie 3 页）。
11. SHOULD_FIX 采纳：两条媒体标题行改 L4 ⚠️。
12. NEEDS_VERIFICATION：卡片与配图说明当时未做，第二轮短审补。
13. NEEDS_VERIFICATION 部分采纳：Anthropic 研究页北京日期无法闭合（页面只写 Oct 27，最早外部痕迹北京 10/28 02:47），正文删去日号；AA 的28日有宣布帖存档 c-fx-2115265133467635877.json（北京 10/28 10:11），保留。

### 第二轮（卡片、速览、译注与配图说明）

复核方：WSL Codex gpt-6-astra（medium，只读，联网），`barking review --reverse --run`。结论：PASS / no blocker；5 SHOULD_FIX、1 NEEDS_VERIFICATION，Claude 核实后全部采纳：

1. “事先定好/预先登记”强于原文：论文 3.6 节写明注册前已有17人开始任务（b-paper.txt:405–410）。正文改“按其登记的主要算法”，译注改“已登记的”。
2. Ai2 准确率依赖弱标签：速览改用不依赖标签的结果“模型对同一批配对的判断过半改变”（“the model changed its verdict on more than half of the same pairs”，d-d6-ai2-novelty.txt:127）。
3. 42 译注截掉首批对象：改为只译高亮的前半句。
4. 吠点③缺主语：正文与吠点卡补“这次实验”。
5. 配图计数：改为小红书/抖音16张、公众号13张。
6. AA“28日发布”未闭合：正文删去日号；宣布帖时刻（北京 10/28 10:11）留在本表备注。

## 视觉复核

复核方：WSL Codex gpt-6-astra（medium，只读），`barking review --visual --run`，27 张附图（含封面、卡片、批注图与底图）。结论：PASS / no blocker；5 SHOULD_FIX、1 NICE_TO_HAVE。复核时用的是第二轮反向核验修改前的图。Claude 处理：

1. 42 译注超出高亮：第二轮反向核验已把译注收窄为高亮的前半句，成立且已改。
2. 46 高亮未含条件句：采纳，quote 扩为 “When the lean went against their choice, they switched 60% of the time, compared with 6% when it agreed with their choice”。
3. 49 译注未写 across versions：采纳，改“不同版本的分数不打算用于跨版本的长期比较”。
4. 省流卡 Anthropic 条缺实验背景：采纳，正文与省流卡改“实验的四个中间选择里”。
5. 45、48 文字过小：采纳。45 移入备用图。48 部分采纳：试裁成只含文字段后，公告正文是全宽段落，字号不变、卡片大半留白，反而丢了 Kimi 按闭源着色的图表，故保留原图；图表标签小，读者以关键句高亮与译注为准。
6. NICE_TO_HAVE：吠点卡3 “Kimi-K3.1-Pro”断行，不改（模板自动换行，不影响文字）。
