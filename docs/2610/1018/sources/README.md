# 1018 来源入口

正文：`../doc_1018_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（Anthropic 公共利益基金）、`evidence-b.md`（DeepSeek-V4.1-Flash-1017 第三方评测）、`evidence-c.md`（HybridAgent 论文）、`evidence-D.md`（速览）；抓取日志：`capture-log-a.md`、`capture-log-b.md`、`capture-log-c.md`、`capture-log-D.md`；派工单：`../../../../.handoff/2026-10-18-1018-evidence.md`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high），派工单 60 分钟目标。耗时（harness 计时）：D 约 13 分钟、C 约 14 分钟、A 约 24 分钟、B 约 38 分钟，均未用到 60 分钟催交。本期起取网页默认用 curl（`barking fetch`）与 opencli，不用 firecrawl（Develata 2026-10-19 决定）。

选题：一份 ChatGPT 扫描（北京 10/19 11:10）与一份 WorkBuddy 扫描（12:00）。两份都有 Anthropic 公共利益基金与 DeepSeek Arena 成绩；WorkBuddy 首推的 Harness²，Claude 核 README 属实，但 HN 仅 5 分，且论文写明评分标准从不给执行器看，“沙箱关闭”是发布版默认配置的提醒，因此降入速览；第三条由 Develata 改选 HybridAgent（ChatGPT 扫描首推，HN 约 382 分）。DeepSeek 1017 版发布 1016 已发，本期按“跟进”规则只收新出现的第三方评测（AA、Arena；LiveBench 只记入核验表，未进正文）。

## 一手来源

- Anthropic 公告：https://www.anthropic.com/news/public-benefit-fund ；官方帖：https://x.com/AnthropicAI/status/2111881963657412903 ；Series H 估值：https://www.anthropic.com/news/series-h ；新智元稿（凤凰网转载）见 `a-zh-ifeng-3200.txt`。
- Arena WebDev 榜：https://arena.ai/leaderboard/code/webdev （开放源码筛选 `?license=open-source`）；Arena 官方帖：https://x.com/arena/status/2111894091168682086 ；Artificial Analysis：https://artificialanalysis.ai/models/deepseek-v4-1-flash-1017 、https://artificialanalysis.ai/leaderboards/models ；LiveBench：https://livebench.ai/ 。
- HybridAgent：https://arxiv.org/abs/2610.18861 （全文 https://arxiv.org/pdf/2610.18861v1 ）；仓库：https://github.com/togethercomputer/HybridAgent ；一作帖：https://x.com/Jiaru_Zou/status/2110400256647340476 。
- 速览：Vals https://www.vals.ai/blogs/impossible-refund ；KillSwitch https://blog.picheta.me/post/killswitch-lang/ 、https://bench.killswitch-lang.org/ ；DeepMind Minecraft https://arxiv.org/abs/2610.22319 ；Harness² https://github.com/google-research/harness2 （`docs/EXPERIMENTS.md`）；Maggi https://terrytao.wordpress.com/2026/10/18/will-it-be-a-golden-age-or-a-dark-age-for-mathematics-and-science/ 。

## 待核验与未完成

- 榜单数字随时刻漂移：Arena 帖写 1670，北京 10-19 17:10 读为 1664；正文写“截至19日”。
- AA 旧版 V4.1 Flash 的数字前后不一致（页面现值 39 / $0.49，AA 9 月帖写 40 / $0.27），原因未找到，正文不做新旧版对比。
- 股东批准的时间表与股东类别、基金治理与 baseline 估值：官方未给。马斯克“This is what I will be doing”只见二手转述，未用。
- AIHOT 本期无法访问（TLS 错误），扫描所称 AIHOT 分数未核。
- HybridAgent 论文的 10.4% 未给计算公式，C 组用表 1/2/8 试算未能复现；论文数字未做独立复现。
- B 组 45、46 号批注底图第一次裁切（全宽）因字太小作废重裁，`b-shots-log.jsonl` 里保留两次记录。
- 未进正文：Harness²（降入速览）、ChatGPT Slides（需 Space 权限、热度低）、Taste-Bench（9/22 旧论文）、Browser Use 场景统计（无热度）、ROSS 判决与德国 Google 案与微软英国数据中心（风控）。
