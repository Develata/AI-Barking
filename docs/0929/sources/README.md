# 0929 来源入口与限制

本页供读者回到原始来源。页面存档与截图为北京时间 2026-09-30 凌晨抓取；期次目录按制作当日美国当地日期命名为 `0929`。派工文件为 `.handoff/2026-09-29-0929-evidence.md`；DevDay 部分（E 组）在北京时间 9/30 01:00 之后补抓，见 `e-capture-log.md`。选题线索来自本地扫描 `.handoff/2026-09-30-0930-scan.md`、`.handoff/2026-09-29-0930-scan-v2.md`，以及两份在对话中回贴、未单独存档的 ChatGPT 扫描（北京时间 9/30 00:17）。

| 内容 | 来源 |
|---|---|
| dots 发布、套餐与额度 | [OpenAI：Introducing dots](https://openai.com/index/introducing-dots/)、[OpenAI X 帖](https://x.com/OpenAI/status/2104984504133918973) |
| dots 安全机制（Auto-review、沙箱） | [OpenAI：How we build safety, security, and privacy into dots](https://openai.com/index/how-we-build-safety-security-and-privacy-into-dots/) |
| Tibo：dots 不占额度 | [Tibo X 帖](https://x.com/thsottiaux/status/2104981170685616361) |
| Tibo：Pro 200 重开与用量改算 | [9月29日 14:41 帖](https://x.com/thsottiaux/status/2104823812042940713)、[23:10 倍数帖](https://x.com/thsottiaux/status/2104951965184925941) |
| Muse 代卖键盘、买家上门（当事人原帖与附图） | [Matt Robb 9月27日 X 帖](https://x.com/MattRobbt/status/2104090601411293303) |
| Allow Always 更新 | [Matt Robb 9月29日 X 帖](https://x.com/MattRobbt/status/2104798102037074212) |
| Meta 员工回应 | [David Singleton X 回复](https://x.com/dps/status/2104805474268783059) |
| Muse 权限选项 | [Meta 帮助中心：Muse 权限](https://www.meta.com/help/artificial-intelligence/1385290430137537/) |
| “发给5个人”等采访内容 | [Guardian](https://www.theguardian.com/technology/2026/sep/28/metas-ai-agent-muse-home-address) |
| GPT-6-Astra 模拟越权测试 | [UK AISI 博客](https://www.aisi.gov.uk/blog/gpt-6-astra-performs-unsanctioned-supply-chain-attacks-in-simulations)、[技术报告 PDF](https://cdn.prod.website-files.com/663bd486c5e4c81588db7a1d/6aba83e3772048bdd24df3d8_AISI_GPT-6_Astra_Technical_Report.pdf) |
| GPT-6.1-Astra 不发布 | [WSJ](https://www.wsj.com/tech/ai/openai-chatgpt-model-release-cancel-safety-5a2f9f42)、[CBS](https://www.cbsnews.com/news/openai-halts-gpt-astra-safety-concerns/)、[CNBC](https://www.cnbc.com/2026/09/28/openai-abandons-plan-to-release-upcoming-model-as-safety-concerns-escalate.html) |

## 阅读限制

- dots 与 Pro 额度部分：“不占额度”“1/5/10 倍”均为员工 Tibo 的个人发言（L2）；官方 dots 页写明只有对话不计入额度，dots 在 Codex 或 ChatGPT Work 开的任务照常计入。Pro 新倍数的官方定价/帮助页检索结果见 `e-capture-log.md`。
- AISI 测试关闭了 Astra 的网安分类器，也没有 dots 的 Auto-review 等产品防护，不能直接当作 dots 的越权率。
- Robb 的帖子与附图是当事人自述，Muse 的聊天截图是模型对自己行为的复述，都不是 Meta 后台日志；Singleton 的回复是员工个人发言。
- Robb 更新帖称 $600 报价、$700 底价，原帖附图却显示商品标价 CA$15、成交 $10，两者对不上，正文不写价格。
- AISI 的 29.2% 为 100 个场景、每场景 5 次运行下的比例，全程模拟且关闭了 Astra 的网安分类器；44% 与 4/49 来自 10 个高越权场景子集，不能与 29.2% 按同一分母比较。
- GPT-6.1-Astra 不发布只见媒体报道与媒体所获声明；截至北京时间 9/30 01:00，未找到 OpenAI 官方原文。
- 取证清单（[evidence.md](evidence.md)）、抓取日志（[capture-log.md](capture-log.md)）与[事实核验](fact-check.md)均保留为工作档案；配图说明见[这里](../images/README.md)。`*.py`、`capture.ps1` 为取证时使用的抓取脚本。

发现影响正文的错误时，在本期增加 `CORRECTION.md`，保留更正原因与来源。
