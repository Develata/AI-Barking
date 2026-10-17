# 1016 来源入口

正文：`../doc_1016_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（DeepSeek 1017 与搜索 API）、`evidence-b.md`（NIST 曲线种子）、`evidence-c.md`（Anthropic 用工研究与 Mercor）、`evidence-d.md`（速览：产品与硬件）、`evidence-e.md`（速览：研究与评测）；抓取日志：`capture-log-*.md`、`*-fetch-log.jsonl`、`*-shots-log.jsonl`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D、E 组 Haiku 5.5（均 effort high），派工单 60 分钟目标。耗时：D 约 13 分钟、E 约 19 分钟、A 约 21 分钟、C 约 45 分钟、B 约 70 分钟（60 分钟时催交，交部分）。opencli 扩展未连接，各组截图走 `barking shot`。firecrawl 额度耗尽（402），各组改用 curl、WebSearch 与 Exa。Haiku 第七次试用（D、E 两组）：摘句均能在存档 grep 到；E 组自查纠正“8 项全部双降”为“8 项中 7 项”，并指出 StarSkirmish 第一名是 GPT-6-Astra 而非 Argon；D 组指出 Waymo 2028 是“目标”、人工驾驶是“即将开始”。

选题：一份 ChatGPT 扫描（北京 10/17 17:07）与一份 WorkBuddy 扫描（18:00）。Claude 选题前自己抓取一手页面核对决定性说法，Develata 选 DeepSeek + ECDSA 种子 + Anthropic 用工研究。扫描勘误：WorkBuddy 称“53 小时对八年是媒体错位嫁接”，实为 Anthropic 原文自己的对比；WorkBuddy 称 DeepSeek 基准“全线上调”，只对两表都有的项成立；ChatGPT 把 The Verge RTX Spark 实测记为 10/17（实为北京 10/16 23:00）。Claude 起草时先写“DeepSeek 只跟自家 9 月版比”，A 组找到新闻页有竞品对比表后改写。风控排除：Eiliko 陪伴机器人（深圳厂商被美方记者指控）、Anthropic 向议员提交 AI 安全法案草案（美国立法政治、草案原件未取得）。

## 一手来源

- DeepSeek 更新日志：https://api-docs.deepseek.com/zh-cn/updates/ ；新闻页：https://api-docs.deepseek.com/zh-cn/news/news261017 ；搜索指南：https://api-docs.deepseek.com/zh-cn/guides/search/ （英文 https://api-docs.deepseek.com/guides/search/ ）；价格页：https://api-docs.deepseek.com/zh-cn/quick_start/pricing ；Claude Code 接入：https://api-docs.deepseek.com/zh-cn/quick_start/agent_integrations/claude_code ；Codex 接入：https://api-docs.deepseek.com/zh-cn/quick_start/agent_integrations/codex ；HF 模型卡：https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017
- Anthropic ECDSA 种子：https://www.anthropic.com/research/ecdsa-curve-origins ；Filippo Valsorda 领奖公告：https://words.filippo.io/seeds-solved/ ；悬赏原帖：https://words.filippo.io/seeds-bounty/
- Anthropic 用工研究：https://www.anthropic.com/research/employment-at-anthropic ；IT 之家报道：https://www.ithome.com/1/014/195.htm
- 本号 0924 期（GPT-6-Sol、Opus-5.5 发布）：`../../../2609/0924/sources/fact-check.md`
- 速览：The Verge RTX Spark 实测 https://www.theverge.com/tech/1012748/nvidia-rtx-spark-windows-arm-laptops-asus-hp-lenovo-impressions ；Anthropic 量子电路 https://www.anthropic.com/research/designing-efficient-quantum-circuits 、arXiv https://arxiv.org/abs/2610.22367 ；Mercor https://www.mercor.com/research/labor-index/benchmark-map/ ；StarSkirmish https://starskirmish.com/bench/v0-2/ ；Ultramarine https://www.paperinstruments.com/ultramarine
