# 1023 来源与制作状态

期号1023（上期1022+1）；线索为 Develata 回贴的 ChatGPT 扫描（北京时间10/24 17:20）与 WorkBuddy 扫描（1024-18，18:05），均只作线索。取证日期2026-10-24。

## 主帖

1. Kimi-K3.1-Pro：[Kimi 官方博客](https://www.kimi.ai/blog/kimi-k3-1-pro)（L1，未给时刻）；[Artificial Analysis 文章](https://artificialanalysis.ai/articles/kimi-k3-1-pro-launch-benchmarks)（L3，10/23）；[Hugging Face moonshotai](https://huggingface.co/moonshotai)（10/24 19:52 无 K3.1 Pro 权重）。Reuters 原站401，未用。证据 `evidence-a.md`。
2. Block Dessa：[官方页](https://block.xyz/dessa)（L1，无日期；Wayback 最早快照北京10/24 03:08）；Dessa 负责人公开帖（L2，X 平台，fxtwitter 公开 JSON）。证据 `evidence-b.md`。
3. Anthropic × 英国 AI 安全研究所：[Anthropic 研究页](https://www.anthropic.com/research/alignment-assessment-ukaisi)（L1，10/23）；[AISI 对齐博客](https://www.aisi.gov.uk/blog/investigating-alignment-failures-in-aisis-incident) 与技术报告 PDF（L1，10/23）；[AISI 8/4 事故报告](https://www.aisi.gov.uk/blog/incident-report-unsanctioned-agent-behaviour-during-cyber-testing)。证据 `evidence-c.md`。与1014所用 Anthropic 7/30 三起事件不同。

## 速览

- Claude Duet 安全说明【跟进1021】：[claude.com](https://claude.com/resources/articles/claude-duet-safety-privacy-and-security)，10/23。
- Google 结核筛查：[Google Research](https://research.google/blog/bringing-foundational-models-to-the-front-lines-of-tb-screening/)，10/23。
- Brotli CVE-2026-111497【跟进1021】：[CVE 记录](https://www.cve.org/CVERecord?id=CVE-2026-111497)、[OpenAI Patch the Planet](https://patchtheplanet.org/brotli)。
- 备用未用：[Railway 初步报告](https://blog.railway.com/p/preliminary-report-october-security-incident)。

证据 `evidence-d.md`。

## 输入与取证

派工单 `.handoff/2026-10-23-1023-evidence.md`（文件名日期由工具按期号生成）。A/B/C 组 Sonnet 5.5、D 组 Haiku 5.5，各组日志 `capture-log-a..d.md`、`*-fetch-log.jsonl`。草稿 `draft-body.md`、`draft-roundup.md` 为中间稿。原件 HTML/PDF/图片不入 Git，发布后按流程存 OpenList。

## 待办与限制

- Kimi 权重“截至10月24日晚尚无”须在发布前重查 Hugging Face。
- 封面：Develata 加权限规则后由 Codex CLI 生成，选择与核对见 images/README.md。
- 未复跑任何模型或评测；Kimi、Block、Anthropic 数字均为各自自报或自测。
