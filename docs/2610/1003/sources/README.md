# 1003 来源入口与限制

本页供读者回到原始来源。页面存档与截图为北京时间 2026-10-03 抓取；期次目录按制作当日美国当地日期命名为 `1003`。派工文件为 `.handoff/2026-10-03-1003-evidence.md`。选题线索来自三份在对话中回贴、未单独存档的扫描（北京时间 10/3 11:01、17:01 两份 ChatGPT，18:00 一份 CodeBuddy）；其中未被采用的说法见 [fact-check.md](fact-check.md) 开头。

| 内容 | 来源 |
|---|---|
| Meta 与数学家合作的 6 篇论文、合作原则、并行工作致谢 | [Meta AI Research：Solving Open Research Problems Together](https://research.meta.ai/blog/solving-open-research-problems-together)、[Meta 官方 X 帖](https://x.com/AIatMeta/status/2106099776035152231) |
| 6 篇论文原件 | [椭球拟合阈值](https://ai.meta.com/research/publications/the-strict-threshold-for-gaussian-ellipsoid-fitting/)、[双调和 NLS 爆破](https://ai.meta.com/research/publications/finite-time-blow-up-of-radial-negative-energy-solutions-for-the-mass-critical-biharmonic-nonlinear-schrodinger-equation/)、[半阿贝尔群](https://ai.meta.com/research/publications/semiabelian-groups-need-not-be-monomial/)、[α-环松弛](https://ai.meta.com/research/publications/tightness-of-the-cycle-based-relaxation-for-completed-length-three-alpha-cycles/)、[弦两点函数与高度函数](https://ai.meta.com/research/publications/string-two-point-function-height-function-on-a-curve/)、[可解演化代数](https://ai.meta.com/research/publications/on-solvable-evolution-algebras-and-a-conjecture-by-garcia-martinez-and-perez-rodriguez/) |
| 并行独立工作 | [Misiakiewicz–Wen](https://arxiv.org/abs/2608.10184)、[de la Cerda–Potechin–Tulsiani–Xu](https://arxiv.org/abs/2608.12415)、[Koehler–Sohn](https://arxiv.org/abs/2608.27372)、[Hu–Wen](https://arxiv.org/abs/2609.25023)、[Nilradical 结果页](https://nilradical.ai/results/kourovka-21-68/) |
| PDE 篇所答的 2015 年开放问题 | [arXiv 1503.01741](https://arxiv.org/abs/1503.01741) |
| DAYJOB 论文、主结果、判分规则、评委模型 | [arXiv 2610.01306](https://arxiv.org/abs/2610.01306)（[HTML](https://arxiv.org/html/2610.01306v1)）、[评测代码仓库](https://github.com/surge-ai/dayjob) |
| DAYJOB 博客（9/23、9/26，耗时 19.6 / 21.6 小时） | [Surge AI：DAYJOB: Can Agents Survive a 9 to 5?](https://surgehq.ai/blog/dayjob) |
| DAYJOB 数据卡 | [Healthcare](https://huggingface.co/datasets/surgeai/DAYJOB-healthcare)、[Finance](https://huggingface.co/datasets/surgeai/DAYJOB-finance) |
| Tavus Griffin 研究页（方法、26/54、可用性） | [Tavus：Griffin](https://www.tavus.io/griffin)、[Tavus 官方 X 帖](https://x.com/tavus/status/2105704169009246248) |
| NVIDIA VideoFDB 榜单与方法 | [VideoFDB 项目页](https://research.nvidia.com/labs/amri/projects/video-fdb/)、[论文 arXiv 2605.30256](https://arxiv.org/abs/2605.30256) |
| 未入正文：Tavus 热度报道 | [Protos](https://protos.com/tavus-call-bot-sparks-ai-scam-psychosis-fears/) |

## 阅读限制

- Meta：博文只标 October 2, 2026，未给时刻与时区，正文因此不写日号；Meta 官方 X 帖为北京 10/3 03:11:30。“Five present answers”哪五篇，官方未列名。博文写“After completing our work”得知他人工作，论文 1、6 写“While completing”，正文合写为“完成期间或之后”。并行工作日期以 arXiv v1 与 Nilradical 结果页、固定 commit 为准；commit 时间不证明首次公开时间；本号不判断优先权。算术物理一篇未列 Review by，不能据此推成无人审阅。
- DAYJOB：Surge 博客 9/23、9/26 已公开基准，论文 v1 于北京 10/1 16:39:10 提交，属“窗口前”。耗时为估计值：论文 13.6 / 16.6 小时，博客 19.6 / 21.6 小时，差异原因两处均未说明。通过率为每题 5 次、剔除运行错误后按题平均。金融判分标准中位数 57.5 的统计范围在论文（50 道公开题）与数据卡（80 题全集）之间不一致。评委为 Claude Opus 4.8；论文未报告本研究中评委与人工的一致率。只公开 50 道医疗题与 50/80 道金融题。
- Tavus：研究页标 October 1st, 2026，未给时区；官方 X 帖北京 10/2 00:59:30。研究为 Tavus 自做，未披露招募平台名称、原始问卷、置信区间或统计检验。X 帖浏览量（抓取时约 1983 万）为瞬间值；帖下 Community Note 为提议、征求评分状态，未正式显示。Protos 报道把 26/54 误写为“of 26 people, 48%”。
- VideoFDB：Griffin 行只见于当前项目页，已存论文 v1 未含该行；评分为语言模型按量表打分（cross-judge agreement 77–89% within 1 point），与 Tavus 的 48% 不是同一指标。Generation 表中 Griffin 3.83、真人 3.92，Affect 子项 Griffin 4.40 高于真人 4.14，正文未写。
- 图灵 1950 原文（[Mind](https://academic.oup.com/mind/article/LIX/236/433/986238)）抓取时受验证页阻挡，正文未引。
- 取证清单（[evidence.md](evidence.md)）、抓取日志（[capture-log.md](capture-log.md)）与[事实核验](fact-check.md)均保留为工作档案；配图说明见[这里](../images/README.md)。`*.py` 为取证时使用的抓取与整理脚本。
- X、Reddit 档案只保存公开帖子的文本、时间与链接，不含抓取者信息；档案里出现的第三方用户名来自公开帖文。

发现影响正文的错误时，在本期增加 `CORRECTION.md`，保留更正原因与来源。
