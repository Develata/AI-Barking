# 1119 期 C 组抓取日志（Contra 设计榜）

时刻均为北京时间（本机即 UTC+8）。机器日志：`c-fetch-log.jsonl`（`barking fetch`）、`c-shots-log.jsonl`（`barking shot`）、`c-heat.json`。
工具：`barking fetch`、`barking shot`（`BARKING_CHROME` 指向本机 Google Chrome）、`barking heat`、`opencli browser`（会话 `c-shot`，后台窗口；用于渲染后截图与 DOM 滚动）、curl（fxtwitter JSON、HN 用户页、官方图片原文件）、WebSearch 与 Exa 搜索（只用来发现二手稿，搜索摘要不作证据）、`dwebp`（官方 webp 转 png）。
未登录任何站点，未输入任何凭据，未接受 Cookie 提示；未读取浏览器 Cookie。opencli 打开的均为公开页面；截图里浏览器翻译插件的悬浮图标出现在页面右缘（x>1340），所有成品截图都已裁掉或未含。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 22:12:14 | https://contralabs.com/frontierdesign 、HN `50558559` 页、Algolia `items/50558559`、https://contralabs.com/ | barking fetch | ok，`c-board.*`、`c-hn.*`、`c-hnapi.json`、`c-contra-home.*` |
| 22:12:24 | …/frontierdesign/leaderboard、methodology、rubric、dataset、blog、gallery、vote；https://benchflow.ai/；https://contralabs.com/research/human-creativity-benchmark | barking fetch | 全部 ok：`c-leaderboard.*`、`c-methodology.*`、`c-rubric.*`、`c-dataset.*`、`c-blog.*`、`c-gallery.*`、`c-vote.*`、`c-benchflow.*`、`c-hcb.*`（HCB 页未用于核对） |
| 22:12:55 | barking heat：`--item 50558559` + HN 搜索 ×3 + Reddit 搜索 ×2 | barking heat | HN ok（`c-heat.json`、`c-hn-50558559.json`）；Reddit 两次被 `old.reddit.com` 登录页拦截（errors 已记入 `c-heat.json`），不绕过 |
| 22:15–22:20 | https://api.fxtwitter.com/status/2123144312494178479、…2123144316365492278、…2123150473465401719、…2123145763182301460 | curl（`--max-time 60`） | 均 200：`c-x-<id>.json`。仅含公开推文数据，无抓取者信息；作者头像等为 Contra 公开账号元数据 |
| 22:1x | https://hacker-news.firebaseio.com/v0/user/gajus.json 、user/cheptsov.json | curl | ok，仅用于查个人页“about”（gajus 写 “gajus.com [email omitted]”，未写所属公司；两份 JSON 未存档，只用来确认） |
| 22:20:47 | https://contralabs.com/research/frontier-design-benchmark；Sulat、Top5Apps、Pillitteri（英文）、PromptBlueprints、Die KI Woche、AI타임스、Kingy.ai、DigIntoAI 报道页 | barking fetch | 全部 ok：`c-research-blog.*`、`c-m-sulat.*`、`c-m-top5.*`、`c-m-pillitteri.*`、`c-m-promptblueprints.*`、`c-m-kiwoche.*`、`c-m-aitimes.*`、`c-m-kingy.*`、`c-m-digintoai.*` |
| 22:21:52 | http://web.archive.org/web/20261119110536/https://contralabs.com/frontierdesign/leaderboard | barking fetch | ok，`c-wb-lb-1119.*`（Wayback 快照，只用来看当时页面写了什么；快照时刻 2026-11-19 11:05:36 UTC = 北京 19:05:36）。`.headers` 已被工具截去抓取地点相关头 |
| 22:22 | 17173、TechMoon、AIPostHub、Digg、HN 用户页 gajus、Pillitteri 中文版；AGI Hunt | barking fetch | 前六项 ok（`c-m-17173.*`、`c-m-techmoon.*`、`c-m-aiposthub.*`、`c-m-digg.*`、`c-hn-user-gajus.*`、`c-m-pillitteri-zh.*`）；`agihunt.info` blocked（HTTP 403，Cloudflare 质询），未重试、未用 |
| 22:2x | https://frontierdesign.contralabs.com/frontierdesign/assets/blog/fdb-04-head-to-head.webp 、fdb-05-pareto-frontier、fdb-01-leaderboard、fdb-02-by-the-numbers、fdb-12-elo-bars（官方博客内图） | curl | ok，`c-fdb-*.webp`（只存本地，不入库）；`fdb-04` 用 `dwebp` 转 png 后读图；其余未逐张读 |
| 22:2x | `opencli reddit search …` 两次 | opencli | 失败：第一次 “web search pagination changed the query”，第二次 SECURITY_BLOCK（Reddit 403 network security）。不重试 |
| 22:15 | X 官方嵌入页 `platform.twitter.com/embed/Tweet.html?id=2123144312494178479`（匿名 headless Chrome） | barking shot render | 渲染为空白图（嵌入需要联网脚本，未渲出）。X 帖子卡片截图：**未取到**；文字与时间以 fxtwitter JSON 为准 |
| 22:xx | Reddit / LinkedIn（Contra 页链到的帖子 https://www.linkedin.com/posts/ben-huffman-b7b6a8102_…） | — | 未取（LinkedIn 帖需登录态风险，不取） |

web.archive.org：仅上面一条快照，用途如上。

## 截图

用 opencli 后台窗口打开页面，在页面里把浏览器缩放设为 1.4（宽 1400 px 的视口得到 1000 CSS px 的桌面版式，才能看到榜单表里的 95% CI 与 Votes 两列；700 CSS px 的手机版式这两列被隐藏），隐藏 position 为 fixed/sticky 的浮层（顶栏、底部导航、提示条；页面内容不改动），把目标元素滚到视口上沿后 `opencli browser screenshot --width 1400 --height <H>`，再用 `barking shot cropbox/stack` 只裁切不改像素。脚本：`c-shot.sh`。**有效像素比约 1.4，不是 2**（2 倍要用 700 CSS px 版式，那样 CI/Votes 列缺失）；截图宽度均不超过 1400 px。中间图在 `/private/tmp/claude-501/c-int/`（会话临时目录）。逐张已目视确认文字清楚、边界落在文字行之间。

| 文件 | 来源页 | 内容 | 说明 |
|---|---|---|---|
| 25-hn-title.png | HN 帖子页（barking shot render，匿名 headless Chrome，DPR 2） | 页顶栏、标题 “Two stealth models just appeared on the leaderboard, both ahead of Fable 5.2”、“145 points by cheptsov … 46 comments” | 1400×260；右上 “login” 是匿名态，不含抓取者账号；“1 day ago” 为页面相对时间 |
| 26-lb-table.png | 榜单页 | 17 行完整表（列名 Win rate / 95% CI / Votes）、柱形、脚注 “Ranked by Elo … 4,311 blind votes by 35 … 33 briefs. Human designers, recruited from Contra, are ranked with the models.” | 984×1882；柱形的灰色块是页面原样（懒加载未着色） |
| 27-elo-chart.png | 榜单页 | “Elo rating from blind head-to-head votes” 柱图，含 17 个参赛者 Elo 标签与图标、横轴名称 | 984×1000 |
| 28-method-golden-elo.png | 方法页 | Model Rollouts & Human Golden Designs 两段；Evaluation Metrics；Expert Preference & Elo Rating（35 位、4,311 票、Bradley-Terry、Spearman 0.95） | 1060×1115 |
| 29-method-taskset.png | 方法页 | Design Taskset：33 个任务、27 个来自 Contra 真实委托、6 个来自创业公司；含页面内 “Most briefs include a logo and brand images” 图 | 1060×1380 |
| 30-blog-voters.png | 博客页 | “And who evaluates the design? The voters behind these Elo scores were whoever showed up. …” 与 “Today we announce …” 段（上接 Pairwise preference 段） | 1060×495 |
| 31-blog-gestalt-results.png | 博客页 | Benchmark Results 两段文字（Gestalt 1,928 / 89%（378/423）、Fable 5.2 1,714、20/28；两组分层）。**其上方的排行榜配图在页面里懒加载，匿名渲染为空白，故只截文字段落** | 1060×345 |
| 32-blog-golden-rubric.png | 博客页 | “The design rubric ranks the models …” 段（73.6、sixth of 16、425 of 574） | 1060×270 |
| 33-blog-gestalt-preview.png | 博客页 | 小节标题 “New Frontier Design Model Preview: Gestalt (Stealth)” 与首段，以及 “Gestalt was also the first model rated above the human golden designs …” 段（两段拼接，原页两段之间是懒加载空白图，已跳过；拼接间隔 24 px 白条） | 1060×349 |
| 34-dataset-access.png | 数据集页 | “Access By request / The dataset isn’t public …”；Dataset content 列表 | 1100×525 |
| 35-17173-exaggerated-headline.png | 17173 报道页 | 标题、作者与时间、以及 “正以Stealth（隐身）的模式在OpenRouter提供调用” 段（两段拼接；上段下沿带一条正文配图的碎边，页面在第二次渲染时版式略有变化，导语栏未出现） | 930×574；用 opencli 浏览器（缩放 1.3）截图 |
| 36-hn-gajus-clarify.png | HN 帖子页 | gajus 评论 “To clarify, it is not two models …”（含作者、“23 hours ago”） | 1400×200 |
| 37-h2h-matrix.png | Contra 官方博客内图 `fdb-04-head-to-head.webp`（官方图片原文件，非页面截图） | “Top-group models won 90% of votes against the second group” 两两对战胜率矩阵，含行列名与全部格子 | 1393×1400（原图 2000×2009，用 `sips -Z 1400` 等比缩小，未裁切；**编号超出 25–36 区间**：区间已用完）。该图在页面中懒加载，匿名渲染为空白，故直接用站内原图 |

候选配图里“数据规模段（33/68/4311）”：首页 “33 design briefs / 68 contributing designers / 4,311 blind votes” 的滚动动画页在匿名渲染下为空白，**未截**；该数字见 `c-board.txt`，4,311 / 35 / 33 已在 26-lb-table.png 的脚注里。榜单页头 “Updated Nov 2026 · 27 Contra jobs” 因吸顶栏被隐藏同样未入图，见 `c-leaderboard.txt`。X 帖子卡片：未取到（见上）。

废弃/待删除：
- `docs/2611/1119/images/35-hn-gajus-clarify.png` 原件（裁切带半行 “reply”）已由我移出到 `/private/tmp/claude-501/c-int/trash/35-hn-gajus-clarify.png`（用 mv，未用 rm；库内已无此文件），新版为 `36-hn-gajus-clarify.png`，编号 35 改给 17173 实例。
- `docs/2611/1119/sources/c-shot.sh`、`c-fetch-list-1.tsv`–`c-fetch-list-5.tsv` 为本组工作脚本与清单，可保留。

## 不确定或有偏差之处

- HN 评论得分：页面与 Algolia 均不含评论得分，评论只记链接与作者。
- Gestalt “420 票最少”：来自榜单页 Votes 列的直接比较（425 为次少）；每票计入两个参赛者（列合计 8,622 = 2 × 4,311），并非各自独立的“对手数”。
- 博客与实时榜的数字不一致（1,928/89%/378 of 423/Fable 1,714 对 1931/90%/420/1713；人类 425 of 574 对 571 票）：官方未解释，我不取舍。
- 研究页（/research/frontier-design-benchmark）与站内博客（/frontierdesign/blog）正文基本相同，但研究页写 “34 design dimensions”、站内写 “35”；站内首页写 “68 contributing designers”。
- 官方两两矩阵（37）里行 “Qwen3.8 Max Prime”“GPT-6 Astra”与榜单页名称（“Qwen3.8 Max”）略有出入，Rubric 页还有 “GPT 6 Astra Ultra”，未核对差异。
- gajus 的身份：HN 个人页只写 “gajus.com”，未写公司；“Contra 联合创始人”来自 kingy.ai（L5）。未独立证实。
- 页面渲染可能随时间变化；各页取得时刻见上表（北京时间 11-20 22:12–22:35）。
