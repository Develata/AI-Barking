# 1117 期 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区），取自 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json`。文件前缀 `a-`。未登录任何账户，未提交表单，未发帖、点赞、评论、关注。浏览器：`barking shot` 的 headless Chrome（匿名，700 CSS px、DPR 2，`BARKING_CHROME` 指向 Chrome 应用二进制）；opencli 扩展未连接，未使用。存档的 `.headers` 已由 `barking fetch` 去掉抓取地点与标识。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 16:27:07 | https://deploymentsafety.openai.com/gpt-6-2-astra-sol ；https://developers.openai.com/api/docs/models/gpt-6.2-astra ；…/models/gpt-6-astra ；https://openai.com/index/introducing-gpt-6-2-astra-sol/ ；https://www.vals.ai/models/openai_gpt-6.2-astra ；https://artificialanalysis.ai/models/gpt-6-2-astra | `barking fetch`（`a-fetch-list-1.tsv`） | 全部 ok：`a-syscard.*`（5594 行）、`a-model-new.*`、`a-model-old.*`、`a-blog.*`（458 行，博客居然未被 403）、`a-vals.*`、`a-aa.*`；逐个打开 txt，是正文 |
| 16:27–16:28 | Figure 35、Figure 36 原图 https://deploymentsafety.openai.com/data/eval-sets/gpt-6-2-astra-sol/assets/images/image1.png 与 image52.png | curl（`--max-time 60`） | 成功，`a-fig-image1.png`、`a-fig-image52.png`（各 2048×1242；不入库） |
| 16:32:08 | https://deploymentsafety.openai.com/gpt-6-2-astra-sol/gpt-6-2-astra-sol.pdf | `barking fetch`（`a-fetch-list-2.tsv`） | ok，9.5 MB，119 页；`a-syscard-pdf.pdf/.txt`（PDF 不入库） |
| 16:36:50 | thenewstack.io（`m-thenewstack`） | `barking fetch`（`a-fetch-list-3.tsv`） | **blocked**：HTTP 403，Cloudflare 质询。未绕过；以检索引擎摘录为线索（见 evidence-A） |
| 16:36:50 | thecapitoljournal.com、bytechap.com、tanhow.com、news.qq.com、trongreen.com、venturebeat.com、implicator.ai（cost-per-task）、paddo.dev | 同上 | 全部 ok（`a-m-*`）；已打开 txt 看过是正文 |
| 16:36:50 | https://news.ycombinator.com/item?id=50535230 ；https://hn.algolia.com/api/v1/items/50535230 | 同上 | ok（`a-hn-page.*`、`a-hn-algolia.json`） |
| 16:36:50 | https://artificialanalysis.ai/leaderboards/models | 同上 | ok（`a-aa-leaderboard.*`） |
| 16:37:00 | HN `--item 50535230` | `barking heat docs/2611/1117 a` | 1248 分 / 773 评（含嵌套），写入 `a-heat.json`、`a-hn-50535230.json` |
| 16:37:00 | Reddit 搜索「GPT-6.2 Astra」「GPT-6.2 system card Blocker Deception」 | `barking heat --reddit` | **失败**：重定向到 `old.reddit.com/login`，响应非 JSON（`a-heat.json` 的 errors）。不登录，不重试 |
| 16:37:35 | https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex ；…/20001354-gpt-6-and-other-models-in-chatgpt ；…/6825453-chatgpt-release-notes ；https://community.openai.com/t/gpt-6-2-astra-and-sol-are-now-available/1417531 | `barking fetch`（`a-fetch-list-4.tsv`） | 前三条 ok（`a-help-work-codex.*`、`a-help-models.*`、`a-release-notes.*`）；论坛帖 **blocked**（正文过短，需 JS 渲染），只留 `a-community.html`（页面元数据里有发布时间 2026-11-17T18:00:37Z） |
| 16:40 | https://openai.com/zh-Hans-CN/index/introducing-gpt-6-2-astra-sol/ | `barking fetch`（`a-fetch-list-5.tsv`） | ok（`a-blog-zh.*`），取中文措辞：“漏报”“隐瞒任务障碍” |
| 16:45:26 | https://developers.openai.com/codex/pricing ；https://help.openai.com/en/articles/11165333-chatgpt-enterprise-and-edu-models-and-limits | `barking fetch`（`a-fetch-list-6.tsv`） | 均 ok（`a-codex-pricing.*`、`a-help-ent-edu.*`）；Codex 定价页第 2110 行起有 “Feature availability” 一段 |
| 16:3x–16:4x | Exa 检索（`web_search_exa`，共 5 次：英文二手稿、The New Stack 原文、中文二手稿、英文社区与 Reddit 说法、开放范围与帮助中心） | Exa MCP | 仅作线索与二手稿清单；The New Stack 原文、implicator.ai 的 early-testers 稿、Alignment Incident Database 与 Unite.AI 仅见摘录，**未存档** |

被拒或失败：The New Stack（403 Cloudflare）、Reddit（登录重定向）、OpenAI 论坛帖（正文过短）。

## 截图

均匿名、已打开确认文字清楚；`images/` 中 13–19 号及 `b-tmp-*` 是 B 组的文件，不是本组。

| 时刻 | 文件 | 来源页 | 说明 |
|---|---|---|---|
| 16:33:47 | `01-syscard-blocker-text.png`（1224×1340） | 系统卡官方 PDF 第 90 页（印刷页码 89），`barking shot pdf`，144 dpi | 4.1.3 概述、4.1.3.1 全节（含 “concerned by this regression … concise … caveats” 与 “designed to elicit … production” 两段，及 Codex 提示词一段）；浅色底；宽度低于 1400，不是 2 倍像素比的网页截图 |
| 16:33:47 | `02-syscard-fig35.png`（1224×580） | 同 PDF 第 91 页 | Figure 35 图（含图题 “Blocker Deception”、with/without dev prompt 图例、纵轴与全部柱值）与图注 |
| 16:33:47 | `03-syscard-broken-search-fig36.png`（1224×1144） | 同 PDF 第 91 页下半 + 第 92 页上半（竖向拼接两段，间隔 24 px） | 4.1.3.2 全节正文 + Figure 36 图与图注 |
| 16:33:46 | `04-model-new-price.png`（1400×2444） | https://developers.openai.com/api/docs/models/gpt-6.2-astra，headless render，DPR 2 | 两段拼接：页头（名称、Price $8·$40）与 Pricing 小节（四行价格、272K 句、Batch/Fast/Ultrafast 句）；浅色底 |
| 16:33:46 | `05-model-old-price.png`（1400×2404） | https://developers.openai.com/api/docs/models/gpt-6-astra | 与 04 同版式同拼法：页头（Price $10·$50）与 Pricing 小节（含 272K 句） |
| 16:47 | `12b-codex-pricing-availability.png`（1400×344） | https://developers.openai.com/codex/pricing，headless render 高 9000 CSS px，再 `crop` 按锚点 “Feature availability” 至 “workspace model settings” | 官方可用范围一段：Plus、Pro、Business、Enterprise、Edu；Enterprise 与 Edu 默认关闭；浅色底，含小标题 |
| 16:48 | `12d-community-availability-full.png`（1400×1520） | https://community.openai.com/t/gpt-6-2-astra-and-sol-are-now-available/1417531，render 高 1500 CSS px + `--virtual-time-budget=10000` | 论坛公告帖首屏：页头（未登录：Sign Up / Log In）、标题、发帖人（OpenAI 论坛版主账号，被引的发布者，非抓取者）、“Starting today … Plus, Pro, Enterprise, and Edu users”、Ultrafast 与 API 两句；含一张视频封面 |
| 16:39:18 | `07b-vals-astra-rank7-full.png`（1400×4110） | https://www.vals.ai/models/openai_gpt-6.2-astra | 页首描述、基本信息、67.66% / $10.68 / 52 min 16 s 卡片、全部榜单行（含 7 / 41）、Updates 栏（含 “#7 of 40”“max reasoning effort”）；浅色底 |
| 16:36:23 | `09-aa-astra-62.png`（1400×3369） | https://artificialanalysis.ai/models/gpt-6-2-astra | 两段拼接：卡片与摘要（#3 / 92、62、$4.00 per task、Max）与 Model Comparison 柱图（Fable 5.2 (max) 65 等；图例 “No publicly available API”）；浅色底；页面底部另有站点的“切换到中文”横幅，位于所截两段之外 |
| 16:38:50 | `10-tanhow-hide-doubled.png`（1400×1600） | https://tanhow.com/news/ffbd12e5-2a89-4789-85da-544f60c0e075 | 最接近的中文标题（未找到明确夸大实例）：标签、标题“隐瞒…概率翻倍”、来源标题、摘要、署名行、“AI 翻译” 声明与首两段；浅色底；站点浮动按钮与底栏已裁出 |
| 16:39:04 | `11-capitol-twice-as-likely.png`（1400×1330） | https://thecapitoljournal.com/openais-own-tests-show-gpt-6-2-astra-is-twice-as-likely-to-hide-coding-failures/ | 最接近的英文标题（未找到明确夸大实例）：站名、标题、署名日期、首两段；用 `--virtual-time-budget` 等动画结束后渲染（不带时首屏有淡入而发灰）；站头为深色底 |
| 16:39:09 | `12-hn-title.png`（1400×240） | https://news.ycombinator.com/item?id=50535230 | HN 标题 “GPT-6.2 Astra and Sol”、1250 points、774 comments、14 hours ago；匿名，右上角为 “login” |

编号区间 01–12：01、02、03、04、05、07b、09、10、11、12、12b、12d 为成品（已逐张打开目视确认：01、02、03、04、05、09、10、11、12、12b、12d 看了成品图或其原始裁切源，07b 看了页首与页尾两处）；06、07、08、12c 是试错版（06 与 08 把摘要段末行切在字中、07 少了页面末行、12c 页头被切到半截），已由 09、07b、12d 取代，列入待删除。没有出“博客可用范围段”：openai.com 与 help.openai.com 在 headless Chrome 里渲染为空白，改用 Codex 定价页与论坛公告帖（见下）。

## 试错与限制

- 系统卡网页：`render` 高视口（高度 8000 及以上 CSS px，另有 30000 与 1700 两次）输出空白，1600 CSS px 在加 `--virtual-time-budget=15000` 时可渲染但永远从页首开始（整份卡是长页，4.1.3 不在首屏）；带锚点 `#sec-4-1-3-1` 且高 1700 的那次为空白，因高度同时变了，不能归咎于锚点。改用官方 PDF 渲染第 90–92 页。中间图在系统临时目录 `barking-shot/26111117-a/` 下。
- 发布博客与帮助中心页面：`render`（含 `--virtual-time-budget`）输出几乎空白（28–118 KB，我只打开了 `blog0` 一张确认为空白，`blogv3000`、`blogv7000`、`help`、`help2` 按文件大小判断为空白），推断是站点对 headless 浏览器不绘制内容；曾尝试加自定义 User-Agent 绕过，该操作被权限系统拒绝，已放弃，**未绕过任何检测**。曾用本地存档 HTML 经 `file://` 渲染，版式崩坏（轮播未展开），不可用，未采用。
- `sips` 对 6 万像素高的 PNG 裁切给出空白，不可靠；排查系统卡长页时改用 `barking shot cropbox --from`。
- 内置浏览器面板（Claude_Browser）打开博客 URL 时被重定向到 `…/zh-Hans-CN/…`，页面可读，但该工具的截图不落盘，所以不能用于出成品图。

## 隐私自查

对 `docs/2611/1117/sources/a-*` 与 `docs/2611/1117/images/` 本组文件 grep 本机用户名、家目录路径、邮箱：存档中无命中（日志里是仓库相对路径）。截图均为公开页面的匿名渲染；10 号图的“探How”页面右下角有站点浮动按钮，已裁掉；12 号图为匿名视图。HN、论坛评论中出现的用户名是被引者的公开 HN 用户名（jasondavies、jrflo、chriddyp、mlmonkey），不是抓取者。

## 待删除文件（精确路径，由 Claude 收尾时一次 rm）

- `docs/2611/1117/images/06-aa-astra-62.png`
- `docs/2611/1117/images/07-vals-astra-rank7.png`
- `docs/2611/1117/images/08-aa-astra-62.png`
- `docs/2611/1117/images/a-tmp-0.png`
- `docs/2611/1117/images/a-tmp-20000.png`
- `docs/2611/1117/images/a-tmp-40000.png`
- `docs/2611/1117/images/a-tmp-56000.png`
- `docs/2611/1117/images/a-tmp-bd4-0.png`
- `docs/2611/1117/images/a-tmp-bd4-3000.png`
- `docs/2611/1117/images/a-tmp-bd4-6000.png`
- `docs/2611/1117/images/12c-community-availability.png`
- `docs/2611/1117/images/a-tmp-blog0.png`
- 系统临时目录里的中间图 `barking-shot/26111117-a/`（`$TMPDIR` 下，不在仓库内，可不理）
