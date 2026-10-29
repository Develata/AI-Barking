# 1028 来源与制作状态

期号1028（上期1027+1）；线索为 Develata 回贴的 ChatGPT 扫描（运行键 2026-10-29T10:00:00Z，北京18:06）与 WorkBuddy 扫描（1029-18），只作线索；HN 热度由 Claude 北京10/29晚现查（Carbon 950/761、指纹脚本 1293/241、Claude Code 413/153）。Develata 10/29 按推荐定主帖：Gemini 4 Carbon → 指纹脚本反 AI 诱饵 → Claude Code 后台摘要费用。

## 主帖

1. Gemini 4 Carbon：[Google 博文](https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-carbon/)（L1，页面 Oct 28, 2026，官方未给时刻；Pichai 帖北京10/29 01:01）；[DeepMind 模型页对比表](https://deepmind.google/models/gemini/)与[评测方法页](https://deepmind.google/models/evals-methodology/gemini-4-carbon)（L1）；[Artificial Analysis 文章与榜单](https://artificialanalysis.ai/leaderboards/models)（L3）；[Gemini API 价格页](https://ai.google.dev/gemini-api/docs/pricing)（Argon 首发价到 2026-11-30）。
2. 指纹脚本：[veritas 博文](https://nullpt.rs/fingerprinting-bioweapons)（L1，作者本人，页面 Wed Oct 28 2026）；[OpenAI Daybreak 说明](https://help.openai.com/en/articles/20001258-openai-daybreak-trusted-access-for-cyber-overview)（L1，Blue 档 Astra 保留标准限制；正文终稿删去该吠点，留作依据）；[HN 50276998](https://news.ycombinator.com/item?id=50276998)。
3. Claude Code 后台摘要：[Flamerix Gist](https://gist.github.com/flamerix-ai/c3905705cc4205e5de3011fa2e74efce)（L1，公司自述；API created_at 北京10/27 06:27，最早修订北京10/29 02:43）；[HN 50284255](https://news.ycombinator.com/item?id=50284255) 中 edwinarbus 回复（L2，身份仅自述＋公开简介，北京10/29 12:52）；[Claude Code CHANGELOG](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)（L1，2.1.311 节未见对应条目；全文 `c-changelog.txt` 1.1 MB 超入库上限，存网盘，入库节选 `c-changelog-excerpt.txt` 为 2.1.312–2.1.309 与 2.1.232 两段）；[issue #86879](https://github.com/anthropics/claude-code/issues/86879)（外部用户，未用）。

## 速览

- Copilot：[GitHub Changelog](https://github.blog/changelog/2026-10-28-github-intelligence-puts-your-code-context-and-history-to-work/)（L1）。
- OpenRouter：[路由改版博文](https://openrouter.ai/blog/announcements/how-we-rebuilt-provider-routing/)（L1，厂商自测）。
- Octoverse：[GitHub 博文](https://github.blog/news-insights/octoverse/octoverse-commits-pushes-reach-new-highs-in-2026-as-developers-build-with-agents/)（L1）。
- 提问量：[Stack Exchange API 2026-09](https://api.stackexchange.com/2.3/questions?site=stackoverflow&fromdate=1788220800&todate=1790812800&filter=total)、[2025-09](https://api.stackexchange.com/2.3/questions?site=stackoverflow&fromdate=1756684800&todate=1759276800&filter=total)（L1，UTC 整月）；[HN 50282454](https://news.ycombinator.com/item?id=50282454)。
- MicroZig：[行为准则](https://github.com/ZigEmbeddedGroup/microzig)（L1，提交 #1103/#1104）。
- 未收：Particle-1（厂商自测、热度低、40字内写不下限定）、Google Connected Apps（D 组未核完）、NVIDIA Nemotron 转写（参数口径不一）、Qwenputer（备选主帖）。不收：PostgreSQL 19 延期（热度不足、窗口前）、“Claude 找到62个 Chrome CVE”（热度不足）、OpenAI agent 经 CPU 缓存传话（1024已发）、以色列政府部门试点（风控）、OpenRouter CSAM 筛查（敏感）、苹果 OLED MacBook Pro（与 AI 关系弱）、Alphabet Q3（IR 原文未开）。

## 输入与取证

派工单 `.handoff/2026-10-28-1028-evidence.md`（文件名日期为工具按本机美东日期生成，内容为1028期）。A/B/C 组 Claude Sonnet 5.5（约35、25、35分钟），D 组 Claude Haiku 5.5（60分钟催交后交回）。A 组一次用 Cookie 罐加浏览器 UA 追 ai.google.dev 跳转，被权限分类器以“Bot Check Circumvention”拦下，改用 opencli 正常会话取文本；那两个 HTML（a-api-pricing.html、a-api-models.html）不作证据，收尾删除。

## 扫描说法勘误（已核原文）

- Carbon：ChatGPT 称 AA 的 Carbon 页打不开，现可打开；WorkBuddy“OSWorld 落后”成立（Opus-5.5 87.2% 对 81.5%）；36氪“只允许国家级安全防御者”原文为 “a subset of cyber defenders”。
- 指纹脚本：“Astra 拒答／GLM 照做”属实，但原文无次数与提示词全文。
- Claude Code：ChatGPT 称修复在 2.1.311+ 只有员工一句话，CHANGELOG 无对应条目；“挂牌价”是员工 2% 的口径，Gist 的 16.4% 是 `cost_usd` 估算。
- 提问量：WorkBuddy 的 961/947/5044 与 API 现查 969/963/5072 不同，采用现查值。

## 风控

- 指纹脚本原文解码出的伪提示含涉政敏感词，正文、卡片、译注不提；图 17 只存档。
- Carbon 博文“美国政府自愿预发布测试”一句不进正文、卡片；图 02 只作备用，正式图 40 裁去首两行。
- 标题与封面不写“生物武器”“政府”。

## 待办与限制

- Carbon 实际开放情况只有“本周起”的计划措辞与 AA “not yet publicly available”，未见防御者已拿到访问的一手确认。
- 员工身份未经 Anthropic 官方页面确认；Gist 首发时刻与修订历史不一致，原因未查明。
- Reddit 各题检索均被登录页拦截。
