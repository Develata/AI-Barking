# 1012 期 A 组抓取日志（Karpathy“能力认知漏斗”）

时间为北京时间（UTC+8）；本机美东（EDT）= 北京 − 12 小时。时刻取自命令发出时的 `date -u` 读数与脚本记录（`a-browser-records.jsonl`），为近似值。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。

工具：curl（浏览器 UA，匿名）；fxtwitter 公开 JSON（`api.fxtwitter.com`）；Playwright Core + 本机 Chrome（匿名、全新临时 profile、不登录、不带 Cookie、UA 去掉 Headless 标记、en-US；脚本 `a-xpage.mjs`、`a-fetch.mjs`、`a-links.mjs`，`a-browser.mjs`/`a-xshot.mjs` 改写自 1010 同名脚本，本期未用于出图）；HN Algolia 与 Firebase API（`a-heat.py`）；WebSearch、Exa（web_search_exa）、firecrawl_search（仅 2 次，额度告急，未用 scrape）。未使用 opencli / agent-reach，未登录任何站点，未运行读取浏览器 Cookie 的工具。

## 1. 抓取

| 约时刻 | URL | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 13:10 | https://api.fxtwitter.com/status/2109361546505966046 | curl | 200；created_at “Sun Oct 11 19:12:37 +0000 2026”，views 2,816,511、likes 27,510、retweets 3,508、replies 1,230、bookmarks 19,089（05:10Z） | `a-x-karpathy-raw.json`、`a-x-karpathy.txt` |
| 13:10 | https://api.fxtwitter.com/status/2042334451611693415 | curl | 200（4 月旧帖，与主帖 `quote` 字段文本一致） | `a-x-karpathy-april-raw.json`、`a-x-karpathy-april.txt` |
| 13:10 | https://api.fxtwitter.com/karpathy | curl | 200；`description` 为空（X 简介空白） | `a-x-karpathy-profile.json` |
| 13:10 | https://eurekalabs.ai 、https://karpathy.ai 、https://karpathy.bearblog.dev | curl | 全部 200 | `a-eurekalabs.*`、`a-karpathyai.*`、`a-karpathyblog.*`（`a-html2txt.py` 生成 txt，复制自 1010，未改） |
| 13:11 | https://www.bloomberg.com/news/articles/2026-05-19/openai-founding-member-andrej-karpathy-takes-role-at-anthropic | curl | **403（Bloomberg 反机器人页）**，不硬绕；仅 /tmp 试验文件，已删，无存档 | 无 |
| 13:11 | https://api.fxtwitter.com/status/2056753169888334312 | curl | 200（Karpathy 5/19 “I’ve joined Anthropic” 帖） | `a-x-karpathy-anthropic-join-raw.json` |
| 13:11 | https://www.axios.com/2026/05/19/anthropic-openai-karpathy-andrej-claude | curl | **403**（反爬壳，5,575 B）；已删除该试验文件，改引搜索摘要 | 无 |
| 13:12:54 | https://platform.twitter.com/embed/Tweet.html?id=2109361546505966046 | Playwright，`a-xshot.mjs`（匿名） | 成功但**长帖被截断**（“…barely come Show more”），不合格；试验图已删（`01-karpathy-funnel-embed-test.png`） | 无（记录在 `a-browser-records.jsonl` 首行） |
| 13:14 | https://x.com/karpathy/status/2109361546505966046 | Playwright 默认 UA | `ERR_HTTP_RESPONSE_CODE_FAILURE`（默认 HeadlessChrome UA 被拒）；换去掉 Headless 标记的 UA 后匿名页可渲染完整长帖 | 试验图在 scratchpad（不入库） |
| 13:15:08 → 13:16:14 | 同上（`?lang=en`） | Playwright，`a-xpage.mjs`（匿名，UA 覆盖，en-US，tz Asia/Shanghai，DPR 2） | 成功，三次调整边距后定稿（用 `OVERWRITE=1` 覆盖了自己刚生成的试验图，非他人文件）：图 01、02、03 | `images/01-karpathy-funnel-top.png`、`02-karpathy-funnel-layers34.png`、`03-karpathy-funnel-drivers.png` |
| 13:16:46 | https://openai.com/index/hugging-face-incident-and-the-road-ahead/ | Playwright，`a-fetch.mjs` | 200（curl 此前被 403）；38 KB 文本 | `a-openai-hf-incident-road-ahead.txt` |
| 13:16:57 | https://openai.com/hugging-face-incident-and-misalignment/ | 同上 | 200；6 KB，时间线折叠（正文只含汇总；完整时间线见 0926 期存档 `docs/2609/0926/sources/a-openai-third-parties-en.md`） | `a-openai-third-party-impact.txt` |
| 13:17:34 | https://huggingface.co/blog/agent-intrusion-technical-timeline | 同上 | 200 | `a-hf-blog-agent-intrusion.txt` |
| 13:18–13:20 | openai.com/index/the-work-now-within-reach/ 、/gpt-6-for-everyone/ 、/new-chatgpt-ads-format-and-measurement/ | 同上 | 全部 200 | `a-openai-work-within-reach.txt`、`a-openai-gpt6-for-everyone.txt`、`a-openai-ads-format.txt` |
| 13:20:08 | https://blog.google/innovation-and-ai/sundar-pichai-io-2026/ | 同上 | 200 | `a-google-io-keynote.txt` |
| 13:20 | https://www.sec.gov/Archives/edgar/data/0001652044/000165204426000066/googexhibit991q22026.htm | curl（SEC 要求带联系方式 UA，写占位邮箱） | 200，371 KB | `a-alphabet-q2-2026-ex991.html/.txt/.headers` |
| 13:20 | https://techcrunch.com/2026/08/11/googles-gemini-app-surges-to-one-billion-users/ | curl | 200 | `a-tc-gemini-1b.html/.txt` |
| 13:20 | https://api.fxtwitter.com/status/2087233951031009665 | curl | 200（@NewsFromGoogle 8/11 帖，TechCrunch 称 Pichai 经 X 宣布） | `a-x-gemini-1b-raw.json` |
| 13:21 | https://www.engadget.com/2271985/openai-dev-day-live-blog-chatgpt-news/ | curl | 200 | `a-engadget-devday.html/.txt` |
| 13:21–13:22 | https://s21.q4cdn.com/399680738/files/doc_financials/{2025/q3,2025/q4,2026/q1,2026/q2}/META-…-Earnings-Call-Transcript.pdf | curl + pdftotext | 全部 200（Meta IR 官方 PDF） | `a-meta-q3-2025-call`、`a-meta-q4-2025-call`、`a-meta-q1-2026-call`、`a-meta-q2-2026-call` 各 `.pdf/.txt`（PDF 不入库） |
| 13:22 | 豆包：新浪财经 2026-06-23 报道 | curl | 200；仅日均 token 数，无用户数 | `a-m-sina-doubao-tokens.html/.txt` |
| 13:21–13:24 | 媒体 14 篇（见 evidence-a.md A5） | curl；FourWeekMBA、AGI Hunt 改用 Playwright | 12 篇 200；FourWeekMBA、AGI Hunt（agihunt.info）**403 反爬**，其 403 壳文本已删，FourWeekMBA 仅引 firecrawl 搜索摘要（未存档页面）；CellCog 首次 curl 超时（`000`），Playwright 重取成功 | `a-m-*.html/.txt/.headers` |
| 13:23–13:26 | 虎嗅、金融界、信鸽中文、ic.work、知澜资讯、哲见 WISE AI、ChainCatcher 中文 | curl | 全部 200 | `a-m-huxiu`、`a-m-jrj`、`a-m-sigai`、`a-m-icwork`、`a-m-pic720`、`a-m-spke`、`a-m-chaincatcher-zh` |
| 13:25 | https://hn.algolia.com/api/v1/items/50047422 、Firebase item、topstories | curl | 200；Algolia 359/360 分（读数时刻不同），Firebase descendants 522→525，首页第 9 | `a-hn-50047422.json`、`a-hn-50047422-fb.json`、`a-hn-topstories.json`、`a-hn-50047422-comments-selected.txt`（我从 Algolia 树中摘选的 13 条评论） |
| 13:26 | https://old.reddit.com/search?… | Playwright 匿名 | 200 但是**登录墙**（“accounts are required to access old Reddit”），不登录，页面文本已删 | 无 |
| 13:27 | https://www.reddit.com/r/singularity/search/?q=karpathy funnel… | Playwright 匿名 | 200，列出 2 帖 | `a-reddit-search.txt`（覆盖 old.reddit 试验文件） |
| 13:27–13:28 | r/singularity、r/OpenAI、r/LocalLLaMA、r/ClaudeAI、r/accelerate、r/ChatGPT、r/artificial 各一次站内搜索 | Playwright 匿名，`a-links.mjs` | 仅 r/singularity（2 帖）、r/accelerate（1 帖）有相关帖；其余版块无；全站搜索返回无关结果 | 结果写进 evidence-a.md；未单独存档 |
| 13:29 | 三个 Reddit 帖页面 | Playwright，`a-fetch.mjs` | 200；匿名页不显示评论得分 | `a-reddit-singularity-breaks-down.txt`、`a-reddit-singularity-gap.txt`、`a-reddit-accelerate-gap.txt` |
| 13:30 | https://aihot.news/ 、/hot | Playwright 匿名（curl 之前遇 bot 挑战页） | 200，渲染成功；热点榜 10/12 13:25 更新 | `a-aihot-home.txt`、`a-aihot-hot.txt` |
| 13:31 | https://x.com/karpathy | Playwright 匿名 | 200；简介空白、Location “Earth”、4.3M Followers | `a-x-karpathy-profile-page.txt` |
| 13:31–13:32 | https://raw.githubusercontent.com/openai/math/main/{README,history}.md、api.github.com/repos/openai/math/commits | curl | 200 | `a-openai-math-README.md`、`a-openai-math-history.md`、`a-openai-math-commits.json` |
| 13:32:46 | HN Algolia/Firebase 热度脚本 | python，`a-heat.py` | 200；360 分 / 525 评 / 首页第 9 | `a-heat.json` |
| 13:33:21 | https://api.fxtwitter.com/status/2109361546505966046（复读） | curl | 200；views 2,903,347、likes 28,074、retweets 3,581、replies 1,254、bookmarks 19,497、quotes 1,232（05:33Z） | `a-x-karpathy-raw-2.json` |
| 13:36 | https://x.com/karpathy/status/2109361546505966046?lang=en | Playwright，`a-fetch.mjs`（匿名） | 200；程序比对：渲染页文本（空白归一化后）包含 fxtwitter `text` 全文，逐字一致；页面另含第三方公开回复（未据以立论） | `a-x-karpathy-page.txt` |
| 13:41–13:46 | https://cdn.syndication.twimg.com/tweet-result?id=2109361546505966046&token=1 、…?id=2109361082217402758 ；https://api.fxtwitter.com/status/2109361082217402758 ；https://x.com/karpathy/status/2109361082217402758?lang=en | curl；Playwright（`a-fetch.mjs`，匿名） | 全部 200。syndication 的 `edit_control` 给出首版 ID 2109361082217402758（snowflake 19:10:46.691Z）与编辑版 ID（19:12:37.386Z）；fxtwitter 对首版只返回 278 字截断文本；旧 ID 的匿名页面仍渲染首版全文，与编辑版逐行 diff 仅差一个重复段落 | `a-x-karpathy-syndication.json`、`a-x-karpathy-original-raw.json`、`a-x-karpathy-original-syndication.json`、`a-x-karpathy-original-page.txt` |
| 13:47 | https://sozai.app/transcript/conversation-mark-zuckerberg/ | curl | 200（SozAI 转录站，Zuckerberg 访谈 “right about a billion … monthly active”；访谈日期未知） | `a-m-sozai-zuck.html/.txt` |

## 2. 截图（均为 DPR 2，页面均为浅色底；04、05 为超出清单的备用背景图）

| 文件 | 内容 | 方法与边界 | 目视核对 |
|---|---|---|---|
| `01-karpathy-funnel-top.png`（1176×674 px） | 帖子卡片顶部：头像与署名、开头句、第一层（“Napkin math…”）与第二层 | X 官方嵌入页把长帖截断（“Show more”），故改用匿名 x.com 帖子页，按 article 元素分段截；本段从 article 顶部到“Now we get to professional”行之前，边界落在行间 | 已打开：文字清楚，范围正确，头像顶部完整 |
| `02-karpathy-funnel-layers34.png`（1176×760 px） | 第三层（含“Building apps, copying apps, translating apps, decompiling apps from binaries…”）与第四层（含 “OpenAI-HF incident”“722 manuscripts”） | 同上；从“Now we get to professional”行到“The funnel is driven”行之前 | 已打开：文字清楚，范围正确 |
| `03-karpathy-funnel-drivers.png`（1176×1192 px） | “The funnel is driven…” 三个驱动因素、结尾“So this is the weirdness of the moment…”、引用的 4 月旧帖卡片、“Last edited 3:12 AM · Oct 12, 2026 · 2.8M Views”、回复/转发/赞/书签行 | 同上；从“The funnel is driven”行到 article 底部 | 已打开：文字清楚，范围正确；卡片内时间按本脚本设定的 tz Asia/Shanghai 显示为北京时间（3:12 AM · Oct 12）；引用卡内为 “Show more” 截断样式（X 页面本身如此） |
| `04-openai-math-withdrawn.png`（1800×2786 px） | github.com/openai/math 的 history.md 渲染页：October 7, 2026 撤回 3 篇、修订 14 篇、“300 / 719 = ~42%” | `a-browser.mjs`（匿名；GitHub 首两次返回 5xx 页面，等待后第三次成功）；从 “History” 标题到 “We have added an additional…” 段末 | 已打开：文字清楚；含 GitHub 页头（“Sign in”），为匿名页 |
| `05-openai-1p2b-weekly.png`（2000×≈1996 px） | openai.com/index/gpt-6-for-everyone/ 页首：日期 October 7, 2026、标题、首段 “…more than 1.2 billion people who use ChatGPT each week.” | `a-browser.mjs`；首次截图落在段中被我否掉并覆盖重截 | 已打开：文字清楚；英雄图未加载导致大块留白与 “Loading…” 作者栏（页面懒加载状态），未改动像素 |

三张图分段后，02、03 不含署名头；01 含。每张只含帖子卡片（article），不含左侧导航、登录/注册栏、侧栏、其他回复。页面为匿名未登录渲染，卡片内无抓取者信息。图 01–03 间无重叠、无遗漏行（01 终于“abstract.”，02 起于“- Now we get to professional”，02 终于“mathematics for a while.”，03 起于“The funnel is driven”）。

## 3. 失败或未取得

- Bloomberg（403）、Axios（403）：未取得 Karpathy 加入 Anthropic 的这两家原文，改用 Karpathy 本人 5/19 帖（fxtwitter）与 Wikipedia 文本交叉。
- FourWeekMBA（403，Exa 抓取也失败）、AGI Hunt（403）：只有 firecrawl/Exa 搜索摘要，不作逐字引用依据。
- Reddit 评论得分：匿名页不显示；HN 评论得分不公开（Algolia/Firebase 均无）。
- 豆包/字节：未找到官方公布的月活或日活数字。
- OpenAI 官方 DevDay 页面/录像：未取得“1.2B 周活”在 9/29 DevDay 的 OpenAI 官方文字稿，只有 Engadget 实时博客（L4）；改用 OpenAI 10/5、10/7 官方页面的同一数字（L1）。
- Ethan Mollick 对该帖的转引：只见于 AIHOT 摘要，未能找到其原帖，未核实。
- AIHOT 条目详情页的 URL：未找到（页面链接未被脚本捕获），只记录热点榜与首页显示。
- 帖子编辑历史：X 页面本身只显示 “Last edited”，但 syndication 接口给出 edit_control，旧 ID 页面保留首版，已取得并比对（见 evidence-a.md A1）；这属于推断链：旧 ID 页面显示的是首版全文，是依据页面行为，不是 X 官方文档。

## 4. 假设

- 所有时刻按 UTC 原值换算北京时间（+8）；Karpathy 帖首版 19:10:46Z = 北京 10/12 03:10:46 = 美东 10/11 15:10:46，编辑版 19:12:37Z = 北京 03:12:37 = 美东 15:12:37。
- 把 fxtwitter JSON 当作 X 公开数据的读数（views 等为取数时刻值，随时间增长）。
- 匿名 Chrome 渲染的 x.com 页面（en-US）显示的文字与 fxtwitter 的 `text` 字段逐字一致（已对照；见 evidence-a.md A1）。
