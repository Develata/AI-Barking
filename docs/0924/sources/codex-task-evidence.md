# Codex 取证任务：0924 重做

按 `templates/codex-evidence.md` 执行，期次目录 `0924/`。先读 `AGENTS.md`、`EDITORIAL.md`（事实分级）、`0924/images/README.md`（上一轮已确认的项目，不必重复取证）。

已有快照在 `0924/sources/*.yml`，能用则直接引用；不足时再打开网页。新截图从 `10-` 开始编号，不得覆盖已有图片。

需要取证的说法：

1. OpenAI 官方对 Sol / Luna 相对 GPT-5.6 降价的原话（是否写“促销价”、是否写 50%），以及官方价格表中新旧逐项单价（输入、输出）。线索：https://openai.com/index/introducing-gpt-6-sol-and-luna/ ，https://developers.openai.com/api/docs/pricing
2. Anthropic 对 Opus 5.5 定位的原话：是否称其为 leading / most capable 模型；与 Fable 5.1 的关系（“大多数工作达到 Fable 5.1 水平”的原文）；Fable 5.1 是否仍在售、是否仍为更高档。线索：https://www.anthropic.com/claude-opus-5-5
3. Banked Reset 两家各自的原话：适用用户、如何使用、有无到期时间。线索：Anthropic 发布页；https://community.openai.com/t/announcing-gpt-6-sol-and-gpt-6-luna-in-the-api-codex-and-chatgpt/1399925/4
4. 两家发布的准确时间（带时区）。
5. Artificial Analysis 关于 Opus 5.5 的完整文章：Intelligence Index 分数（Opus 5.5、Astra）及 effort/fallback 条件；AA 自测的 Terminal-Bench 4.0 分数（线索称 Opus 5.5 与 Astra 打平于 59.6%）。同时记录 Anthropic 发布页评测表中 Terminal-Bench 4.0 的数字及其条件（Opus xhigh / Astra high？harness？）。两者并列，不取舍。线索：AIHOT 条目 https://aihot.news/items/cmucyny580521roni2aiyh9xj ，AA 官网文章页。
6. AA 对 Sol / Luna 的评测：Intelligence Index 与前代是否持平、单任务成本是否减半、是否有评测退步（哪几项）。线索：AA 的 X 账号 @ArtificialAnlys 与官网。
7. Opus 5.5 系统卡中“安全演习约半数运行出现有害行为”的原文、测试设定与 Anthropic 的解释。线索：https://aihot.news/items/cmud52j4i003droralpe8a53b ，Anthropic system card。

截图只取第 5、7 条中能直接展示冲突或原文的区域（最多 3 张）。

输出：`0924/sources/evidence.md`（表格 + “可能的吠点”列表）。最后一条消息只给 evidence.md 的内容摘要（≤300 字）和新截图文件名。不修改 `0924/doc_0924.md`、`0924/doc_0924_publish.txt`、`0924/images/README.md`。
