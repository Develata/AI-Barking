# 1110 来源与制作状态

期号1110（上期1109+1）；线索为 Develata 回贴的 HTML 选题扫描 1111-12（北京 11-11 12:02）。同时附来的 ChatGPT stateful rescan（.md）是 10-29 的旧一轮（读的是 1027 期时的仓库），Develata 确认放错，未用。Develata 要求再调研一轮后定题：Claude 现查 HN 40 小时内 60 分以上全部帖子，另派一个 Sonnet 子代理扫各家官方渠道与 AA，新发现 Gemini 人工审核（HN 717）与 Gemini-4-Carbon 全面开放（跟进 1028）。Develata 拍板：主帖 Gemini 人工审核、Opus-5.5“变笨”之争、Carbon 跟进；AlphaProof、Glasswing、Strands 等进速览。

派工单 `.handoff/2026-11-10-1110-evidence.md`（文件名日期是 `barking handoff` 按期号与美东算的，派工实际在北京 11-11 约 14:25；派工单正文的时区行已改为北京时间）。四组执行方：A、B、C 为 Claude Sonnet 5.5（各约 25–30 分钟），D 为 Claude Haiku 5.5（约 5 分钟）。取证与起草同属 Claude 家族，事实清单与存档的跨模型核对由反向核验（Codex）完成。

## 主帖

1. Gemini 人工审核：[GitHub 文档](https://github.com/thisisme6598/gemini_files)、[HN](https://news.ycombinator.com/item?id=50429426)、[Gemini 帮助页 16836988](https://support.google.com/gemini/answer/16836988)、[帮助页 2026-01-14 快照](https://web.archive.org/web/20260114213656/https://support.google.com/gemini/answer/16836988)、[Gemini Apps Privacy Hub](https://support.google.com/gemini/answer/13594961)、[Personal Intelligence 资格页 16598406](https://support.google.com/gemini/answer/16598406)、[2023 年博文](https://blog.google/innovation-and-ai/products/our-responsible-approach-to-building-guardrails-for-generative-ai/)、[1 月发布博文](https://blog.google/innovation-and-ai/products/gemini-app/personal-intelligence/)、[Workspace Privacy Hub](https://knowledge.workspace.google.com/admin/generative-ai/generative-ai-in-google-workspace-privacy-hub)；夸大实例：[AI Weekly](https://aiweekly.co/fr/alerts/github-gemini-aspire-drive-et-gmail-vers-des-rviseurs-humains)、[Techpresso](https://techpresso.co/blog/google-2023-promise-no-human-reviewers-gmail-drive-gemini-contractors-can-read)、[cnBeta](https://www.cnbeta.com.tw/articles/tech/1585684.htm)。
   - 待核：Google 有无对媒体回应（报道与所查官方页均未见，官方社媒时间线未查）；Drive 帮助页“Gemini side panel … reviewers may read and annotate”一句未核。
   - 仓库首个提交的作者邮箱在 `a-gh-commits.json` 里已替换为 `[EMAIL-REDACTED]`；正文不点仓库作者。
2. Opus-5.5“变笨”之争：[Melaugh 原帖](https://x.com/shanemelaugh/status/2119192075505893695)（thread 共 4 条，存 fxtwitter JSON 与帖内原图）、[Theo 反驳帖](https://x.com/theo/status/2119238026136330313)、[HN 50422069](https://news.ycombinator.com/item?id=50422069)、[Claude thinking 文档](https://platform.claude.com/docs/en/build-with-claude/thinking)、[effort 文档](https://platform.claude.com/docs/en/build-with-claude/effort)、[Claude Code CHANGELOG](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)、[LiveNerf](https://github.com/ninjahawk/livenerf)。
   - 两人数据都是自述、原始日志未公开（L6）；Anthropic 回应只查了状态页、更新日志、GitHub issue 与 HN，官方社媒时间线未查。
   - CHANGELOG 2.1.316/2.1.317 有子代理 effort 改动，未判断与此有关，正文未写。Reddit、BridgeBench、linux.do 打不开。
3. Gemini-4-Carbon 全面开放（跟进 1028）：[发布博文](https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-4-carbon-now-available/)、[定价页](https://ai.google.dev/gemini-api/docs/pricing)、[Google MKA 博文](https://blog.google/innovation-and-ai/models-and-research/gemini-models/multimodal-knowledge-agent-benchmark/)、[DeepMind MKA 技术报告](https://storage.googleapis.com/deepmind-media/papers/mka_tech_paper.pdf)（PDF 只存网盘）、[AA MKA 榜单](https://artificialanalysis.ai/evaluations/mka)、[AA MKA 文](https://artificialanalysis.ai/articles/google-deepmind-multimodal-knowledge-agent)、[模型卡](https://storage.googleapis.com/deepmind-media/Model-Cards/Gemini-4-Carbon-Model-Card.pdf)。
   - “6X”与 AA 榜单约 5 倍的差异来源（缓存命中率、计价时点、档位）双方都未说明，并列记录。
   - 博文脚注 1（11/30 到期）折叠在“More Information”里，无头渲染展不开，只有文本存档、没有截图；正式配图改用定价页。
   - AA 数字为北京 11-11 15:00 前后快照，会漂移。

Claude 与第二条的当事模型同属 Anthropic，速览里另有三条 Anthropic 相关；这几条靠跨模型复核把关。

## 速览

- DeepMind AlphaProof Code：[博文](https://deepmind.google/blog/introducing-alphaproof-code/)、[论文页](https://google-deepmind.github.io/code-verification)。
- VulnCheck Glasswing 复盘（跟进 1007）：[VulnCheck](https://www.vulncheck.com/blog/anthropic-glasswing-november-update)、[Anthropic 台账](https://red.anthropic.com/2026/cvd/)、Anthropic 10/08 OSS Scanner 研究页（存于 `docs/2610/1008/sources/c-oss-scanner-research.txt`）。curl 那批 19/34 未能自行复数，署 VulnCheck。
- 亚马逊 Strands Decider：[Strands 博客](https://strandsagents.com/blog/amazon-saves-millions-replacing-sonnet-with-strands-decider/)。
- Anthropic 顺势学习：[对齐博客](https://alignment.anthropic.com/2026/homeopathic-learning/)、[arXiv 2611.10110](https://arxiv.org/abs/2611.10110)。
- OpenAI 思维链可监控性：[对齐博客](https://alignment.openai.com/cot-monitorability-model-lifecycle/)。
- Claude 白板、表格：[产品文章](https://claude.com/resources/articles/sheets-whiteboards-data)。
- Qwen4-Max 预览：[通义官方帖](https://x.com/Alibaba_Qwen/status/2120335993643684315)（官方博客、模型卡未找到）。
- 未收：OpenRouter MCP vs CLI（原文同一数字前后归属不同模型）、OpenRouter CSAM 筛查（无热度）、OpenRouter 并入 Stripe（8/19 旧闻）、澳大利亚“未能防止”罪（只是“考虑”，风控）、Anthropic 全档额度重置（Develata 发评论区）、Anthropic IPO 估值与 OpenAI 三季度营收（单一媒体源、财经线）、英伟达 CEO 风波（非 AI 主线）。

## 备注

- 批注底图 `carbon-pricing-raw.png` 由 `barking shot` 以 `pricing-raw.png` 之名裁出后改名（`ed-shots-log.jsonl` 记的是旧名），像素未改。
- `thinking-summary-raw.png` 是 Claude thinking 文档按 700 CSS px 宽重新渲染后裁出的“Summarized thinking”节（B 组原截图 2000 px 宽，放进卡片字太小）；`theo-table-raw.png` 是 Theo 图的下半部；`blog-promise-raw.png`、`melaugh-upfront-raw.png` 为同图两段拼接。
- B 组取证截图 13–24 用尽后溢出到 37–45；D 组原交付文件名 `evidence-D.md`、`capture-log-D.md` 已改为小写。
- `b-cc-changelog.txt`（1.19 MB，CHANGELOG 全文）超过 1 MB，不入库；入库的是其前 799 行 `b-cc-changelog-2.1.310-322.txt`（2.1.322 至 2.1.310，行号与原文件相同），全文可从 GitHub 上 `anthropics/claude-code` 的 CHANGELOG 历史版本取回。
