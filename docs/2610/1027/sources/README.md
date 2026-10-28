# 1027 来源与制作状态

期号1027（上期1026+1）；线索为 Develata 回贴的 ChatGPT 扫描（运行键 2026-10-28T10:00:00Z，北京18:03）与 WorkBuddy 扫描（1028-18），只作线索；HN 热度由 Claude 北京10/28晚现查。Develata 10/28 定主帖：Genie 4 第一，Anthropic 分析师实验第二，AA 智能指数 v5【跟进1023】第三；速览由 Claude 从候选中选。

## 主帖

1. Genie 4：[DeepMind 博文](https://deepmind.google/blog/genie-4-a-new-generation-of-world-simulation/)（L1，页面 October 27，官方未给时刻）；[模型页](https://deepmind.google/models/genie/)（局限 “a few minutes of continuous interaction”、演示 “Sequences shortened”）；[Google 博客](https://blog.google/innovation-and-ai/models-and-research/google-deepmind/introducing-genie-4/)；Genie 3 官方页与 Project Genie 页（对照）；[TechCrunch](https://techcrunch.com/2026/10/27/)、The Verge（“两分钟”，L4）；Bloomberg 西语版 Bloomberg Línea 与财联社（Hang Qi 发言，L4；Bloomberg 英文原文付费墙未绕过）；[投资界转载机器之心](https://news.pedaily.cn/202610/570971.shtml)（夸大标题实例）。
2. Anthropic 分析师实验：[研究页](https://www.anthropic.com/research/agents-judgment-calls)（L1，“How AI agents shape experts’ judgment calls”）；[论文 PDF](https://www-cdn.anthropic.com/519c17a5d23a794e8b6d50a854048d0b61e0d73a.pdf)（“Who's Aligned to Whom?”，表2、注册估计）；[36氪转载新智元](https://eu.36kr.com/zh/p/4045003602007172)（夸大标题实例）。
3. AA v5【跟进1023】：[v5 公告](https://artificialanalysis.ai/articles/artificial-analysis-intelligence-index-v5)（L3）；[Changelog](https://artificialanalysis.ai/changelog)；Progress Index 方法页（跨版不作长期比较）；Kimi K3.1 Pro、Ling 3.1 Flash 1019 模型页；[Ling 模型卡](https://huggingface.co/inclusionAI/Ling-3.1-flash-1019)（v4.3.2 44分）；[Cognition FrontierCode 2.0](https://cognition.ai/blog/frontier-code-2.0)。本号1023、1026 期所引 AA 数字均为 v4.3.2，取证时 v5 尚未上线，见 `fact-check.md` C 节。

## 速览

- 药研：[Anthropic 研究页](https://www.anthropic.com/research/early-progress-drug-development)（L1）；Generate、Divergence、BigHat 报告（L3）。
- 离线应急：[Survival LLM](https://www.brangus.io/survival-llm)（L1，开发者自测）。
- 单群分类：[Axiom 博文](https://axiommath.ai/research/the-enormous-theorem-formalized/)、[AxiomMath/CFSG](https://github.com/AxiomMath/CFSG)（L1）。
- 新颖性：[Ai2 博文](https://allenai.org/blog/novelty-bench)（L1）；[arXiv 2610.02022](https://arxiv.org/abs/2610.02022)。
- LFM3：[Liquid AI HF 博客](https://huggingface.co/blog/LiquidAI/lfm3)与 LICENSE（L1）。
- Replit：[博文](https://replit.com/blog/anyone-can-bring-ideas-to-company-codebase)（L1）。
- 未收：OpenAI×Oracle（Oracle 自家员工用户数，圈内题）、Apple M6（硬件新品）。不收：GSA 政府采购折扣（风控）、Roblox 爬虫（风控、归因未闭合）、Wan 诉 OpenAI（患者死亡、诉讼指控）、Discord 公开消息进搜索（原文未提 AI）。证据 `evidence-D.md`。

## 输入与取证

派工单 `.handoff/2026-10-27-1027-evidence.md`（文件名日期为工具按本机美东日期生成，内容为1027期）。A/B/C 组 Claude Sonnet 5.5（约27、36、约36分钟），D 组 Claude Haiku 5.5（约22分钟），均未触发60分钟催交。取证方与执行方（Claude）同家族，反向核验须逐行核对事实清单与存档。

## 扫描说法勘误（已核原文）

- Genie 4 “两分钟”：Google 书面材料只写 “a few minutes”“short sessions”，“两分钟”出自媒体对发布会的转述。
- 分析师实验：研究页标题不是 “Who's Aligned to Whom?”（那是论文标题）；HN 162分查不到，只有2分转帖；0.24–0.82 是改口率差值的区间。
- AA v5：WorkBuddy 称中文媒体 v5 后仍写55分，找到的55分稿均早于 v5 官宣；FrontierCode 2.0 表里是 Kimi K3 而非 K3.1 Pro。
- 速览：药研 1010/1232 不冲突（变体数与孔数）；申报文书是 Enveda 的；Survival LLM 21/22 只是高配两档；M6 新闻稿写了“每个核心带神经加速器”。

## 待办与限制

- Genie 4 博文首发后改过（dateModified 北京10/28 20:01），无旧版存档；无技术报告。
- AA 站点切换到 v5 的精确时刻无一手来源（Wayback 未查）。
- 分析师实验 OSF 预注册页未核；Reddit 各题检索均被登录页拦截。
- Axiom CFSG 未构建、未审计。
