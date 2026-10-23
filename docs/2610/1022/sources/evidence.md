# 1022 证据汇总

用户确定主线顺序：Claude 软件安全证明、软银紧急灾害警报暂停、Canva Craft。两份扫描是线索，不作为既有核验。取证日2026-10-23，期号1022为用户指定。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 证明规模239入口、约1.8亿行 | L1 | https://www.anthropic.com/research/formal-verification | 239 entry points across 123 different OSS-Fuzz libraries. / about 180 million lines of Rocq | images/01-formal-raw.png | 单线程静态、固定内存预算、RISC-V/Linux模型；修改后代码。原文是厂商研究报告，本号未复跑。 | 已找到 |
| 修代码再证明有奖励作弊可能性 | L1 | 同上 | Note this leaves open the door to reward hacking. | 无；preflight-formal.txt脚注6 | 可能性，不是已确认作弊。惩罚修改、奖励输入改变少、记录审计补丁。 | 已找到 |
| 软银暂停三品牌紧急警报 | L1 | https://www.softbank.jp/corp/news/info/2026/20261023_01/ | 緊急速報メールの配信を停止しています。 | images/02-softbank-raw.png | PHP紧急维护；地震/海啸/灾害避难；JST06:47即北京时间05:47；预计26日完成。 | 已找到 |
| Canva官方称模型第四、实验室第二 | L1 | https://www.canva.com/newsroom/news/canva-craft/ | It debuted as the fourth most advanced image generation model on DesignArena | 无；b-web-reader-archive.txt | 第四为模型，第二为实验室；独立榜单当前顺序另有内部截图核对，但正文保持官方归属。 | 已找到 |
| Craft从零构建 | L1 | https://community.canva.dev/t/connect-api-canva-craft-is-now-available-for-image-generation/8999 | Craft was built from scratch by Canva Research | images/03-canva-raw.png | 官方开发者社区；API预览信息见完整存档。 | 已找到 |

## 热度与去重

`preflight-heat.json`为北京时间19:20:46实际HN采样：Claude原文对应50202217为357分/211评论；Canva官方社区帖50200002为12分/1评论，另有The Verge报道帖50203240。PHP讨论热度不能直接视为软银新进展的独立热度；软银以当日官方现实服务影响作为跟进依据。未把用户扫描中的342/199等旧值写为本次值。

本地daily-scan已发到1021。PHP原始漏洞已发，本期只跟进新增服务暂停；Claude软件证明、Craft和三条补充速览未重复近三期已有同一事实。未以缺少微信公众号完整列表推定平台绝无未登记内容。

## 输入纠偏和未采用线索

- HTML把WSJ报道标为L1，实际应为L4；未选该条，不据此确认员工泄密或“公司赶走审计者”的动机。
- 就业政策原文确有“鼓励企业探索”，速览保留该限定；不采纳“一个目标都没有”及“首份”的排他断言。
- MiniMax许可已经存档，但未选主帖或速览，不把“超过年营收门槛”说成“每2000万美元要批一次”。
- Meta承诺支出、Anthropic递表传闻未完成一手核对，不作成品事实。

形式化研究报道状态：needs_human_mathematical_review（若要独立接受研究证明）。当前仅完成新闻事实与条件转述核验，非形式化证明复现；无对完整证明/补丁的接受结论。
