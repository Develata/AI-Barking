# 1027 期 B 组抓取记录

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；本机时区美东（EDT，`date` 显示 08:5x 时北京为 20:5x），北京 = 本机 + 12 小时。机器记录另见 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`。本组开工约 20:29，交回约 21:0x（北京）。

## 一、网页与 PDF 存档（`barking fetch`，前缀 b）

| 北京时间 | name | URL | 结果 |
|---|---|---|---|
| 20:29 | page | https://www.anthropic.com/research/agents-judgment-calls | ok（200，正文完整；JSON-LD 日期取自 `b-page.html`） |
| 20:29 | paper | https://www-cdn.anthropic.com/519c17a5d23a794e8b6d50a854048d0b61e0d73a.pdf | ok（200，约 0.88 MB PDF，pdftotext 1349 行，只存本地不入库） |
| 20:37 | unite-en、unite-zh | https://www.unite.ai/anthropic-finds-analysts-defer-to-ai-agents-in-judgment-study/ 及 /zh-cn/ 版 | ok（对照用，未收为夸大例） |
| 20:37 | 36kr | https://eu.36kr.com/en/p/4045003602007172 | ok（新智元稿的英文版） |
| 20:37 | xiaohu | https://xiaohu.ai/article/anthropic-agents-judgment-calls/ | ok |
| 20:37 | fox | https://www.foxnews.com/live-news/ai-news-anthropic-openai-october-28 | ok（直播博客页，只用其中一条） |
| 20:37 | sina | https://finance.sina.com.cn/7x24/2026-10-28/doc-iniwtaun3602225.shtml | ok（快讯，txt 仅 26 行） |
| 20:37 | eastmoney | https://wap.eastmoney.com/a/202610283910639473.html | ok（财联社署名） |
| 20:37 | explainx、superpower、blockchain | https://explainx.ai/blog/…、https://superpowerdaily.com/posts/…、https://blockchain.news/news/anthropic-study-ai-agents-steer-expert-judgment | ok（只存档备查，未精读，未收为实例） |
| 20:3x | 36kr-zh | https://eu.36kr.com/zh/p/4045003602007172 | ok（中文原版） |

清单文件放在会话临时目录，未入库（内容就是上表的 name 与 URL）。

## 二、其他请求

| 北京时间 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 20:30 | `barking heat docs/2610/1027 b`（HN 5 组查询；Reddit 3 组） | barking | HN 命中 1 条（id 50271917，2 分 / 0 评）；Reddit 3 个请求都被重定向到登录页，记入 `b-heat.json` 的 errors，未登录、未绕 |
| 20:3x | https://hn.algolia.com/api/v1/search_by_date（标题关键词、URL 哈希、“Aligned to Whom”、“Anthropic” 等 10 余次） | curl | 200；只有 id 50271917 一条相关，没有 162 分的条目 |
| 20:3x | `opencli twitter search "agents-judgment-calls" / "from:AlexTamkin …" / "from:haileyjoren …" / "from:AnthropicAI …"` | opencli（浏览器已登录会话，只读；结果只在内存里看，未存档） | 前几次成功；随后 `HTTP 429 SearchTimeline … rate-limited`（会话配额），`from:amacinho`、`from:HaileyJoren` 未取到，之后不再重试 |
| 20:3x | https://api.fxtwitter.com/status/2115194783404835175、/2115281633494630424 | curl | 200（找到 Tamkin 开篇帖 id） |
| 20:3x | https://api.fxtwitter.com/status/2115190541688398184 ；https://api.fxtwitter.com/2/thread/2115190541688398184 | curl | 200，存 `b-fx-tamkin-2115190541688398184.json`、`b-fx-tamkin-thread.json`（作者官方公开字段，不含抓取者信息）；开篇帖时刻 2026-10-27T21:14:58Z |
| 20:3x | `opencli reddit search "Anthropic analysts agent lean 86% judgment calls"` | opencli | 失败（返回非 JSON，疑为登录页） |
| 20:3x | `opencli google search`（3 条）、`opencli brave search`（1 条） | opencli | 成功，只用于找报道线索，搜索摘要未当来源；引用的都来自存档页 |
| 20:3x | WebSearch（中、英文各 1 次） | WebSearch | 索引过旧，无相关结果，未使用 |
| 20:4x | https://osf.io/t3q59 | 未访问 | 论文写预注册在 osf.io/t3q59；我没有去核 OSF 页（时间与范围限制），预注册说法只来自论文自述 |

## 三、截图（`images/13`–`24`）

渲染工具：研究页正文用 `barking shot render`（headless Chrome，700 CSS px，DPR 2）；该方式下页头标题与题图不渲染（页首大片空白），所以页头改用 opencli 浏览器截图（`--window background`，1400 宽，页面按桌面版式显示，约 1.2× 而非 2×）。36氪页面与 X 嵌入页同样用 opencli。论文用 `barking shot pdf`（144 dpi，整页）。

| 文件 | 来源 | 内容 | 方法 |
|---|---|---|---|
| 13-anthropic-header.png | 研究页 | 标题、“Read the paper”、题图、作者、Published Oct 27, 2026、Team Alignment 与首段（1160×2120） | opencli 截图（top2）+ cropbox；裁掉左侧导航与右侧翻译插件图标、滚动条 |
| 14-anthropic-summary.png | 研究页 | 署名到 77%/15% 一段，含 86%/50%、60%/6% 概述（1320×1850） | render + cropbox |
| 15-anthropic-design.png | 研究页 | “Study design” 整节：61 人、11 人退出、50 人、四个决策、隐藏提示、Opus 4.8 High（1320×2990） | render + cropbox |
| 16-anthropic-results-fig2.png | 研究页 | “Results” 前两段（92/82/81/90、86%、54% vs 36%、82% vs 61%）与图 2 全图及图注（1320×1940） | render + cropbox |
| 17-anthropic-switch.png | 研究页 | 38 个试次、60% vs 6%、54 个百分点、95% CI 24–82 及自选警示（1320×470） | render + cropbox |
| 18-anthropic-77-15.png | 研究页 | 77% / 15% 段、“none suspected”、图 4 全图与图注（1320×1480） | render + cropbox |
| 19-anthropic-limitations.png | 研究页 | “Our study’s limitations” 四条全文（1320×1400） | render + cropbox |
| 20-anthropic-caveat.png | 研究页 | “We don’t know how much this tells us about the real world”（112/123 次）与“not a simple cautionary tale”两段，含“human reviewer is only an independent check”句（1320×910） | render + cropbox |
| 21-paper-abstract.png | 论文 PDF p.1 | 题目、作者栏（含论文公开的作者邮箱）与摘要（1224×1584） | barking shot pdf + cropbox |
| 22-paper-registration-table2.png | 论文 PDF p.9 + p.10 上部 | §3.6 预注册、表 2（含 H2 调整与未调整行）、§4.1 “not statistically significant” 段（1224×2384；p.9 下方有一段空白） | pdf 渲页 + stack |
| 23-36kr-xinzhiyuan-headline-v2.png | 36氪欧洲站（新智元稿） | 文章标题块与 “86%的关键分析决策……”“Alex Imas”引用及其嵌入的帖子图（827×1402，约 1× 像素） | opencli 截图（滚动两段）+ stack；页面右上日期显示 “2026-10-28 00:26” 是浏览器按本机 EDT 渲染，对应北京 12:26，静态文本为 12:26；页面是浅色底，标题块为蓝绿色底；裁掉了翻译插件图标与滚动条；嵌入的 Alex Imas 帖子是文内图片（含其头像与 handle，公开人物） |
| 24-tamkin-thread-limits.png | X 嵌入（platform.twitter.com/embed） | Alex Tamkin 线程：“human in the loop might provide less assurance” 与 “There are lots of limitations here …”（1100×1090） | opencli 截图 + cropbox；匿名嵌入，只含 Tamkin 的帖子；未含任何抓取者账号信息；右侧翻译插件图标与滚动条已裁掉 |

逐张目视确认：13、14、15、16、17、18、19、20、21、22、23-v2、24 均已打开，文字清楚、范围正确。

## 四、待删除文件（试验与废弃图，精确路径）

- `E:\gitclone\AI-Barking\docs\2610\1027\images\13-anthropic-head.png`（headless 渲染，页头标题与题图空白，废弃）
- `E:\gitclone\AI-Barking\docs\2610\1027\images\13-anthropic-title.png`（底边切掉了 “call.” 一行，已由 13-anthropic-header.png 取代）
- `E:\gitclone\AI-Barking\docs\2610\1027\images\23-36kr-xinzhiyuan-headline.png`（headless 渲染，标题被右侧截断，已由 23-36kr-xinzhiyuan-headline-v2.png 取代）
- 临时目录文件（库外，不用管）：`C:\Users\QQ\AppData\Local\Temp\b-work\`、`C:\Users\QQ\AppData\Local\Temp\barking-shot\26101027-b\`

## 五、失败与偏差

- Reddit：被重定向到登录页，未取到；HN 条目只有 2 分 / 0 评。
- X：`opencli twitter search` 遇会话 429 限流，Herdağdelen、Joren 的原创帖与 Anthropic 官方账号帖未能完整核查（见 evidence B8e、B8f）。
- OSF 预注册页未核。
- headless Chrome 对 anthropic.com 研究页的页头与对 X 嵌入页渲染为空白（原因未查），改用 opencli。
- 研究页 Figure 1（实验设计示意图）与 Figure 3（所有权分布图）在截图 14、18 外没有单独截图；Figure 3 在 18 之前的区段，未单独收。
- 存档与截图里的第三方公开作者信息：论文作者栏有作者公开的邮箱（论文 PDF 自带），截图 21、23 含之；Alex Imas 的 X 头像与 handle 出现在 23（文内转载的公开帖）。均非抓取者信息。
- 交付前自查：对 `docs/2610/1027/` 中本组 `b-*` 文件 grep 本机用户名与账号名，无命中（PDF 为二进制，另用 pdftotext 文本查过）。“QQ” 在新浪与 36氪页面的命中是网页分享按钮文字，不是抓取者信息。
