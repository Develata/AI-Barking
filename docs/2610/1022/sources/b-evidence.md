# Canva Craft 取证

采集：2026-10-23 19:21–19:24 北京时间。仅取证，未实测生成质量。官网下载遭 Cloudflare 验证，未绕过；网页阅读工具成功取得正文，原始工具结果见 `b-web-reader-archive.txt`。`preflight-canva.html` 不是正文。官方社区 fetch HTML 成功但自动纯文本仅标题，完整文字以网页阅读存档及截图核对。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 图片内文字支持55语言 | L1 | https://community.canva.dev/t/connect-api-canva-craft-is-now-available-for-image-generation/8999 | Prompts in any language, text in the image in 55 languages. | images/03-canva-raw.png | 官方开发者公告；是文字生成支持范围，非55语言质量实测 | 已找到 |
| 最高2K | L1 | 同上 | Craft renders at up to 2K | images/03-canva-raw.png | 厂商能力表述，未实测 | 已找到 |
| Craft模型第4，Canva实验室第2 | L1 / L3 | https://www.canva.com/newsroom/news/canva-craft/ ; https://www.designarena.ai/leaderboard/image | —（榜单视觉数据） | sources/b-arena-view.png | 独立榜单本次显示前三OpenAI模型1407/1382/1363，Craft1335，第5Reve1321；可支持模型第4及按模型所属实验室去重的第2，不能改写成模型第2 | 已找到 |
| Magic Layers把生成图片转换为可编辑设计元素 | L1 | https://www.canva.com/newsroom/news/canva-craft/ | transform generated images into fully editable design elements | 无，官网验证页不交付 | 官网明确由Magic Layers转换；不支持模型原生直接输出可编辑图层的说法 | 已找到 |
| Craft MCP尚未上线 | L1 | https://www.canva.com/newsroom/news/canva-craft/ | Canva MCP (Coming Soon) | 无 | 官网上一段笼统说available，具体段落明确Coming Soon，应保留该限制 | 已找到 |

## 可能的吠点与待核

- 模型第4与实验室第2不能混写；独立榜单本次1335与外部报道1341不同，动态时点不同，不取二手值替代现场值。
- 图层由Magic Layers转换；未实测编辑、文字质量，也未检查55种语言的完整清单。
- 官网称两阶段为先规划构图后渲染。未验证这是唯一架构或独家能力，不能写“首次”。
- 官网自报Portrait、Landscape第1；独立榜单本次只显示默认Abstract第2，未逐项复核另外两分类。
- 社区API公告称preview，可能发生破坏性变化；还留有编辑确认占位文本，API细节不宜仅凭这一帖作扩展结论。
- 社区讨论未采样；不编造热度或评论。

## 截图验收

`images/03-canva-raw.png` 来自官方开发者社区真实网页，700 CSS px / DPR2，1400 px宽，仅裁切，保留标题、Canva Staff署名、首段、What's new全部条目。原公告截图遇验证页，因此改用同公司官方开发者页面，未重绘。

`sources/b-arena-view.png` 为独立榜单现场辅助存档：含主榜前12名与Show 71 More以及部分Abstract榜，属于折叠列表的局部视图，不满足完整benchmark表格发布截图要求，不能直接当正式配图。足以核对本次前5名顺序与分数，未声称复核全部83个模型。

原文档案 `b-web-reader-archive.txt` 包括官网完整阅读结果、社区公告和一次旧榜单URL读取错误。`b-leaderboard.html/.txt` 是旧参数URL的页面壳，实际排名以 /leaderboard/image 现场截图为据。
