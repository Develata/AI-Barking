# 1024 来源与制作状态

期号1024（上期1023+1）；线索为 Develata 回贴的 ChatGPT 扫描（北京时间10/25 17:04）与 WorkBuddy 扫描（1025-18），以及 Claude 的检索子代理，均只作线索。取证日期2026-10-25。

## 主帖

1. Kimi 推广说明风波【跟进1023】：爆料者公开发言（L6）与未经独立验证的附图内容（L7），海外社媒，fxtwitter 公开 JSON 与 opencli 时间线；[VentureBeat](https://venturebeat.com/technology/moonshots-kimi-k3-1-pro-flirts-with-open-weights-leadership-amid-paid-influencer-scandal-allegations)（L4）；[Pekingnology](https://www.pekingnology.com/p/kimis-global-momentand-a-very-chinese)（背景）；[Kimi 官方博客](https://www.kimi.ai/blog/kimi-k3-1-pro)（L1）；[Artificial Analysis Kimi K3.1 Pro](https://artificialanalysis.ai/models/kimi-k3-1-pro)、[GPT-6 Astra](https://artificialanalysis.ai/models/gpt-6-astra)（L3）；Reddit r/kimi 3D 帖（L6）；[Arena Agent 榜](https://arena.ai/leaderboard/agent)（备选，未用）。证据 `evidence-a.md`。
2. Apollo 与 Apricitas：[Apollo Daily Spark](https://www.apollo.com/wealth/insights-news/insights/daily-spark/no-signs-of-ai-payoff-outside-the-tech-sector)（L1，10/24，未给时刻）；[Ramp AI Index](https://ramp.com/data/ai-index)（L1，数据截至2026年9月）；[Apricitas Economics](https://www.apricitas.io/p/ai-and-the-fall-of-the-creative-class)（9/27）。证据 `evidence-b.md`。
3. METR 审计【跟进1020】：[METR](https://metr.org/blog/2026-10-22-red-teaming-openai-agent-monitoring/)（L1，10/22）；[OpenAI 训练环境安全](https://openai.com/index/securing-our-frontier-model-training-environment/)（L1，页面10/23）；The Decoder 等“6–25%”转述（L4，夸大实例）。与0926（9/20 DNS 事件）、1020（两起编码事故）区分。证据 `evidence-c.md`。

## 速览

- MiniMax-M3.1-Flash 权重【跟进1019】：[Hugging Face](https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash)。
- Johns Hopkins Bayview：[Baltimore Sun](https://www.baltimoresun.com/2026/10/24/johns-hopkins-bayview-medical-center-data-breach/)（付费墙，导语）。
- Reservoir 诉 Anthropic：诉状 N.D. Cal. 5:26-cv-12308（`d-d3-reservoir-anthropic-complaint.txt`）。
- Elvis 相关版权方诉讼：[Billboard](https://www.billboard.com/pro/elvis-presley-estate-sues-suno-anthropic/)、[The Hollywood Reporter](https://www.hollywoodreporter.com/music/music-industry-news/anthropic-elvis-presley-lawsuit-ai-1236739617/)。
- 英国 AI 生物安全实验室：[GOV.UK](https://www.gov.uk/government/news/introducing-the-ai-biosecurity-lab)，10/24（涉外国政府，Develata 同意收录）。
- 未用：Cerebras（工程细节，备用）；Claude 额度 A/B（未找到原帖）。

证据 `evidence-d.md`。

## 输入与取证

派工单 `.handoff/2026-10-25-1024-evidence.md`。A/B/C 组 Sonnet 5.5、D 组 Haiku 5.5（模型 ID 自报 claude-haiku-5-5，用时约25分钟），四组都因 API 连接中断（ECONNRESET）停过一次，经续跑完成。各组日志 `capture-log-a..d.md`、`*-fetch-log.jsonl`。草稿 `draft-body.md`、`draft-roundup.md` 为中间稿。原件 HTML/PDF/图片不入 Git，发布后按流程存 OpenList。

## 待办与限制

- Kimi 官方回应须在发布前重查。
- 未复跑任何模型或评测；AA、Arena、Ramp 数字均为各自平台数据。
