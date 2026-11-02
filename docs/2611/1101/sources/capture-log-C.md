# 1101 期 C 组抓取日志

执行方：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8；本机美东 EST，北京 = 本机 + 13 小时）。批量抓取的逐条结果见 `c-fetch-log.jsonl`；截图动作见 `c-shots-log.jsonl`；热度见 `c-heat.json`（HN 条目）与 `c-r-heat.json`（Reddit 搜索失败）。

## 批量存档（barking fetch，前缀 c，均为匿名 curl，无登录态）

| 时刻 | 名称 | URL | 工具 | 结果 |
|---|---|---|---|---|
| 11-02 23:49 | blog | https://terrytao.wordpress.com/2026/11/01/mathematics-has-changed-lets-act-like-it/ | barking fetch | ok（276 KB，正文 1360 行；含 30 条评论） |
| 11-02 23:49 | blog-comments-feed | …/feed/ | barking fetch | ok（评论 RSS，仅最近 10 条） |
| 11-02 23:50 | calegari-home | https://math.uchicago.edu/~fcale/ | barking fetch | ok |
| 11-02 23:50 | pp-100reactions | https://proofsandprompts.com/2026/10/08/100-reactions-to-100-solutions/ | barking fetch | ok |
| 11-02 23:50 | pp-accreditation | https://proofsandprompts.com/2026/11/01/a-new-system-of-accreditation/ | barking fetch | ok（旁证，未用） |
| 11-02 23:50 | leiden-zenodo / leiden-doi | https://zenodo.org/records/20302944 ；https://doi.org/10.5281/zenodo.20302944 | barking fetch | ok（两条内容相同） |
| 11-02 23:50 | leiden-pdf | https://zenodo.org/records/20302944/files/Leiden_Declaration_on_Artificial_Intelligence_and_Mathematics.pdf | barking fetch | ok（199 KB，11 页，531 行） |
| 11-02 23:50 | oai-history / oai-readme | raw.githubusercontent.com/openai/math/main/history.md、README.md | barking fetch | ok |
| 11-02 23:50 | szegedy-doc / hexagon | Szegedy 的 Google Docs 发布页；https://hexagonmath.org/ | barking fetch | ok（旁证，未用于取证条目） |
| 11-02 23:51 | numberphile-oembed | https://www.youtube.com/oembed?url=…Zh86i_azaQw&format=json | barking fetch | ok（只取标题，未看视频） |
| 11-02 23:51 | calegari-ai / calegari-researchai | https://math.uchicago.edu/~fcale/AI.html、researchAI.html | barking fetch | ok |
| 11-02 23:51 | tao-mastodon | https://terrytao.wordpress.com/mastodon-posts/ | barking fetch | ok（该页已停更，仅含侧栏评论列表） |
| 11-02 23:51 | calegari-blog | https://www.galoisrepresentations.org/ | barking fetch | ok（首页聚合） |
| 11-02 23:52 | news-unwire | https://unwire.hk/2026/11/02/openai-ai-maths-papers-frank-calegari/ai/ | barking fetch | ok |
| 11-02 23:52 | news-newsat | https://www.news.at/technik/mathematik-ki-gastbeitrag-terence-tao-calegari-openai | barking fetch | ok |
| 11-02 23:52 | news-drweb | https://www.drweb.de/mathematik-ki-calegari-bewertung/ | barking fetch | ok |
| 11-02 23:52 | news-hyperai | https://hyper.ai/en/stories/8066d6aa1d88a0ad9b66f31f56dcadc9 | barking fetch | ok |
| 11-02 23:52 | news-ecosistema | https://ecosistemastartup.com/openai-y-700-pruebas-como-cambian-las-matematicas/ | barking fetch | ok |
| 11-02 23:52 | news-fourweekmba | https://fourweekmba.com/ai-taos-blog-lists-6-proposals-for-maths-after-openais-proofs/ | barking fetch | **blocked（HTTP 403）**，按规则未再试 |
| 11-02 23:53 | cn-sina-lingxiu | https://www.sina.cn/weibo/detail/5352050670175663.html | barking fetch | ok |
| 11-02 23:53 | cn-cnyes | https://news.cnyes.com/news/id/6626630 | barking fetch | ok |
| 11-02 23:53 | cn-163-retract | https://www.163.com/dy/article/L8QIF4C30514R9P4.html | barking fetch | ok（网易号，未用） |
| 11-02 23:53 | ahm-tao | https://terrytao.wordpress.com/2026/10/07/ahm-statement-on-openais-october-6-release-of-mathematical-documents/ | barking fetch | ok |
| 11-02 23:54 | tao-home | https://terrytao.wordpress.com/ | barking fetch | ok（确认 11-01 后博客无其他回应帖） |
| 11-02 23:54 | pp-newsfeed | https://proofsandprompts.com/newsfeed/ | barking fetch | ok（页面含站点联系邮箱 proofsandprompts@gmail.com，是该站公开地址，不是抓取者信息） |
| 11-03 00:01 | ihes-duminil | https://www.ihes.fr/en/professeur/hugo-duminil-copin-2/ | barking fetch | ok |
| 11-03 00:01 | oai-contents | raw.githubusercontent.com/openai/math/main/CONTENTS.md | barking fetch | ok（641 KB，未超过 1 MB） |

## 其他取数

| 时刻 | 内容 | 工具 | 结果 |
|---|---|---|---|
| 11-02 23:51 | HN 条目 50328251 | barking heat --item | ok，355 分，447 条嵌套评论；`c-hn-50328251.json`、`c-heat.json` |
| 11-03 00:00 | Reddit 搜索（2 组）、HN 搜索 “Calegari Tao OpenAI math” | barking heat（前缀 c-r） | HN 命中 0 条；**Reddit 两次均被重定向到登录页，失败**，按规则不重试；`c-r-heat.json` |
| 11-03 00:02 | 陶哲轩 Mastodon 公开 API（accounts/lookup + statuses?limit=20） | curl --max-time 30 | ok；最新一条 2026-10-08；`c-tao-mastodon-statuses.json`（公开帖，无抓取者信息） |
| 11-02 23:5x–00:00 | 6 次 WebSearch（中英文，找报道与中文夸大标题） | WebSearch | 仅找到 unwire.hk 一篇中文报道；未找到大陆媒体报道 |

## 截图（opencli 浏览器优先；失败或版式异常退回 barking shot render）

| 时刻 | 文件 | 来源 URL | 工具与处理 |
|---|---|---|---|
| 11-02 23:5x | `25-guest-post-header.png`、`26-openai-critique.png`、`27-duminil-copin-quote.png`、`29-rush-leiden.png`、`30-recommendations-list.png`、`31-evaluate-ai-assisted.png`、`32-duminil-notes.png` | 陶哲轩博客原文页 | opencli browser（会话 c-shot，`--window background`），zoom 2，`--full-page --width 1400`，再用 barking shot cropbox 裁切。**页面处理**：该博客主题版式最小宽 760 CSS px，在 zoom 2 下右侧被截断，故先注入样式：隐藏侧栏（#secondary）、主栏宽 700 px；隐藏浏览器翻译插件浮标与 WordPress 底部操作栏。正文文字、顺序与排版未改。输出宽 1380 px（裁掉右侧滚动条）。25 的页头副标题 “…and other mat[hematics]” 被右侧裁断（非证据文字）。31 里 Cole 的因式分解公式与年份（1903、2003）是页面渲染的数学图片。 |
| 11-03 00:00 | `28-duminil-original-source.png` | proofsandprompts.com 反应汇编页 | opencli browser，zoom 2，覆盖 WordPress 栏布局（`.wp-block-columns` 改为单栏）以免窄栏；**为让 Duminil-Copin 那条出现在页首，把同页上他之前的几十条他人反应用样式隐藏（display:none）**，保留页头、标题、日期、“Posted on … last reactions added on 19 October 2026” 与主持人说明；该条与后续一条的分隔线均在图内。译文插件浮标已隐藏。该页 DOM 是 flex 栏，**原页是长页，这张图不是页面实际首屏**。 |
| 11-03 00:00 | `33-leiden-declaration.png` | Zenodo Leiden Declaration PDF | barking shot pdf（pdftoppm，144 dpi）渲第 1、4、5 页，barking shot stack 拼接：第 1 页标题、第 1 页说明与 DOI、第 4 页 “Recommendations for individual mathematicians” 与 01、第 5 页 02–06。拼接间隙与左侧细边为深蓝（stack 自动取页边色），非原文内容。Zenodo 记录页本身用 opencli zoom 2 渲染出错（页面平铺重复），未使用。 |
| 11-03 00:00 | `34-openai-readme-raw-data.png`、`35-openai-history-withdrawals.png` | github.com/openai/math/blob/main/README.md、history.md | opencli 在 GitHub 上 zoom 2 出现重复平铺，改用 barking shot render（headless Chrome，700 CSS px，DPR 2）+ cropbox。截图为匿名 headless，不含登录栏。35 的顶部有一条 GitHub 工具栏残边（约 6 px）。 |
| 11-03 00:00 | `36-tao-as-face-example.png` | 新浪转载的凤凰网科技微博 | barking shot render + cropbox；账号为媒体官方号，保留 |

截图逐张已打开目视确认文字清楚、范围正确（25–36 全部）。截图中无抓取者账号信息：opencli 页面未登录相关站点；`barking shot` 渲染为匿名；已隐藏的插件浮标不在图内。

## 假设与限制

- 本机 EST（UTC-5）；北京时间 = 本机 + 13 小时，已按此换算 `article:published_time` 等 UTC 时间。
- “小红书/抖音等平台名”规则只对正文与卡片文字生效，本取证文件中出现的是来源页面的原文。
- 为让 28 号图成立，隐藏了同页其他人的反应（只是视图，没有改页面内容）；这一处理在日志与 `28` 的说明中如实记录。
- 抓取者信息自查：对 `docs/2611/1101/` 下 c- 前缀文件 grep “develata/gmail.com/本机路径”，仅命中 `c-pp-newsfeed.*` 里该站公开联系邮箱（站点方信息，非抓取者）。

## 待删除文件

无（本组未调用 rm；中间截图在 `/tmp/barking-shot/26111101-c/` 与 `/tmp/claude-1000/…/scratchpad/`，不在仓库内）。
