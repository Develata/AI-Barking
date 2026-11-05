# 1104 期 A 组抓取日志

执行：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8）；本机时钟为美东 EST，北京 = 本机 + 13 小时。批量抓取的逐条结果在 `a-fetch-log.jsonl`，截图动作在 `a-shots-log.jsonl`，热度在 `a-heat.json`。清单：`a-fetch-list-1.tsv`。截图辅助脚本：`a-shot-section.py`。

## 一、抓取

| 时刻 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 17:19 | `opencli doctor` | opencli | 扩展已连接 |
| 17:20 | https://openai.com/index/a-thousand-fewer-paths-to-root/ | opencli browser（innerText 与 outerHTML） | 成功。存 `a-openai-paths-to-root.txt`、`a-openai-paths-to-root.html`（555 KB，本地）。页面是浅色，未登录 |
| 17:21 | 同一页，headless Chrome（`barking shot render`） | barking | 渲染出空白页（挑战页），丢弃；之后所有 openai.com 截图改用 opencli |
| 17:22 | https://openai.com/news/rss.xml | opencli | 成功；摘出该条 item 存 `a-openai-news-rss-excerpt.xml`（pubDate Wed, 04 Nov 2026 08:00:00 GMT）。完整 RSS 约 3.7 MB，没有存 |
| 17:24 | `barking heat`：HN 条目 50372929 与搜索 “a thousand fewer paths to root” | barking | 成功，`a-heat.json`、`a-hn-50372929.json`（Algolia：318 分在条目里，搜索命中 320 分 / 228 评）；Reddit 搜索被登录墙拦下，失败，未绕过 |
| 17:27 | 中文与英文报道批量存档（新浪两篇、IT之家、量子位、蓝点网、Cryptika、ShiftDelete） | `barking fetch` | 7 条 ok（量子位、蓝点网首次连接失败，自动重试成功），存 `a-sina-patch`、`a-sina-121w`、`a-ithome`、`a-qbitai`、`a-landian`、`a-cryptika`、`a-shiftdelete` 的 html/headers/txt |
| 17:29 | OpenAI 官方 X 时间线 | `opencli twitter tweets OpenAI`；`api.fxtwitter.com/status/<id>`（三条） | 成功，存 `a-openai-x-thread.json`、`a-fx-2118119451401048497.json`、`a-fx-2118119461345739018.json`、`a-fx-2118119463161905362.json`（公开内容，不含抓取者信息）。首条 2026-11-04 23:13:24 UTC = 北京 11/5 07:13:24 |
| 17:30 | Wayback CDX | curl | 成功，`a-wayback-cdx.json`；`archive.org/wayback/available` 返回 429，未重试。只用来看页面何时被抓取，不用于正文 |
| 17:31 | lore.kernel.org 搜索（`f:openai.com`、`f:kylebot@openai.com`、`b:"Patch the Planet"`）与两个线程页 | curl 先返回 403；改 opencli 读取 Atom 搜索结果与线程页 | 成功。存 `a-lore-kylebot-list.json`（主题 + 链接，122 条）、`a-lore-maintainer-replies-excerpt.txt`（只存维护者回复，不存补丁正文与差异块） |
| 17:33 | WebSearch 找内核社区与媒体反应（英文、中文各 1–2 次） | 搜索引擎 | 英文查询没有命中 OpenAI 原文的二手反应；中文查询命中新浪、IT之家、量子位等；没有找到 LWN / LKML 对 948 数字本身的回应 |

没有抓：央广网、界面、财联社、腾讯新闻（搜索结果里没出现它们的具体稿件）；Reddit（登录墙）。

## 二、截图（`docs/2611/1104/images/`，2 倍像素比，宽度均不超过 1400 px）

方法：opencli 打开页面，`document.documentElement.style.zoom='2'`，隐藏页头，把目标元素滚到视口顶部，用 `--width 2400`（即 1200 CSS 像素，文章栏在中间，左侧悬浮目录不压正文）截视口，再用 `barking shot cropbox` 只裁出文章栏（x 620–1760 的 1140 px 宽）。文章页浅色。取图后逐张目视确认。

| 文件 | 来源 | 内容 |
|---|---|---|
| `01-oa-hero.png` | openai.com 原文 | 日期、类别、标题、副标题（948、$1.2 million） |
| `02-oa-intro-upcoming-model.png` | 同上 | 导语四段（2,308 / 1,724 / 948 exploits；“upcoming cyber model”；“human researchers reviewed”） |
| `03-oa-summary-findings.png` | 同上 | 摘要 Findings 全部要点（948 working LPE exploits、$1,213,430、$1,279、16x–55x、kernelCTF、at least 10 novel） |
| `03a-oa-summary-reflections.png` | 同上 | 摘要 Reflections 前三条（over 100 patched or staging、~300 discussed、>5x、GPT-5.6 Sol） |
| `04-oa-cost-upper-bound.png` | 同上 | Cost analysis 整节（upper bound 原句） |
| `04a-oa-funnel-chart.png` | 同上 | 官方漏斗图整张：图标题、全部层级、总额 $1,200,000、图例脚注 “Schematic; widths not to scale” |
| `05-oa-validation-1724.png` | 同上 | 2,308 / $316,549、Vulnerability validation 前半、验证系统图、1724 / ~74.7% 段 |
| `07-oa-validation-breakdown-chart.png` | 同上 | 未验证部分占比饼图（图标题、图例、6 项百分比）与前一段文字 |
| `07a-oa-validation-cost-fp-tp.png` | 同上 | $73,330 与真/假阳性成本相近一句 |
| `06-oa-exploit-1169-948.png` | 同上 | Exploitability assessment：313/1,238、291/1,169、948 distinct patches、$359 |
| `06a-oa-exploit-cost-success-fail.png` | 同上 | 各阶段成本、$558 vs $271、失败 $3,922/69 与成功 $360/1169 |
| `08-oa-maintainers-61-43-299.png` | 同上 | 61 / 43 / 299 与 10 天、Takeaway |
| `09-oa-stronger-models-166-948.png` | 同上 | 7 月 166 个 / $513,690；10 月 948 个 / $1,213,430 |
| `09a-oa-novel-techniques.png` | 同上 | We sampled 166 exploits…10 new techniques；细节不公开原句 |
| `10-oa-wrong-0day-200-20.png` | 同上 | Using the wrong 0-day 整节 |
| `11-oa-patch-generation-reports.png` | 同上 | Patch generation：已发数百份报告和补丁、其余经 Patch the Planet |
| `12-hn-main-and-author-reply.png` | news.ycombinator.com 条目 50372929 | 标题与点数栏（321 points，229 comments，提交人 dangelosaurus）+ basilikum 的提问 + ky1ebot1337 的“按补丁数计”答复（两段拼接；浅色；HN 用户名是公开的，抓取者未登录；已用样式隐去浏览器翻译插件注入的中文，并隐去横向滚动条以外的浮标，右侧悬浮图标已裁掉） |
| `12a-qbitai-948-patch-suggestions.png` | 量子位 | 标题与时间（2026-11-05 09:54:44）、“948个补丁建议”“大约20个”“10天…50年”几段（三段拼接，中间跳过一张电影剧照式配图，未纳入）；署名“Jay”为该站公开署名，头像为默认占位 |

编号说明：A 组区间 01–12，字母后缀（03a、04a、06a、07a、09a、12a）为同一编号下的拆分或补充。`12a` 是夸大说法实例，不是原文依据。

没有截图的项：新浪两篇转载页（页面是微博转载、标题在网页标题里，页面可见文字无标题）、IT之家、蓝点网（只存档与摘句）；官方 X 帖（只存 JSON；量子位页里嵌的 X 帖截图是转载，不用）；lore.kernel.org 线程（只存文字，不放补丁正文）；pipeline 示意图（宽 1485 px，超过 1400 上限，且文字已在正文里有）。

## 三、使用到的假设与限制

- 假设：opencli 浏览器里没有登录 openai.com、HN；HN 页面顶栏显示 “login”，说明未登录。
- openai.com 页面里 “Stage costs”“Findings” 等内容均取自 innerText，图表（饼图、漏斗图）以截图为准。
- 时区：OpenAI 页只有日期；其余时刻见 `evidence-A.md` A1。
- lore.kernel.org 邮件中的个人姓名是邮件 From 头里公开的职业身份，只记录其发给公开列表的补丁与维护者回复，不收集其他个人信息；这些线程与 948 的对应关系页面没有写，未作推断。
- 不下载、不存任何利用代码或 PoC；lore 线程只存回复文字，补丁正文没有存。
- 没有运行会读取本机浏览器 Cookie 的工具；opencli 只在匿名页面上操作，没有登录、输入凭据、发帖、点赞。

## 四、过程中的小事

- 第一次裁 `01-oa-hero.png` 时下边缘切到“Listen to article”那一行的中间，已换成更靠上的框重新裁（旧版移到工作目录外的临时文件夹，不在仓库内）。第一版 `12-hn-main-and-author-reply.png` 宽 1670 px，超过 1400，且带有浏览器翻译插件的中文，已按窄布局与隐去译文重截（旧版也在仓库外）；`a-shots-log.jsonl` 里保留了两次的记录。
