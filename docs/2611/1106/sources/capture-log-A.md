# 1106 期 A 组抓取日志

执行：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8）；本机时钟为美东 EST，北京 = 本机 + 13 小时。全组约 2026-11-07 23:10–00:10 北京。批量抓取逐条结果见 `a-fetch-log.jsonl`，截图动作见 `a-shots-log.jsonl`，热度见 `a-heat.json`，清单文件 `a-fetch-list-1.tsv … a-fetch-list-12.tsv`。浏览器（opencli 会话 `a-cg`、`a-cg2`、`a-x`）均为匿名读取，没有登录、没有输入凭据、没有发帖或点赞；congress.gov 页面右上角的 “Sign In” 是站点自带链接，已裁掉。

## 一、批量存档（barking fetch）

| 时刻 | 清单 | 结果 |
|---|---|---|
| 23:11 | 1：govinfo H.R. 10538（html/pdf/txt）、S. 5493（html/pdf）、congress.gov 三页、Casar 新闻列表页 | govinfo 与 Casar 页 ok；congress.gov 三页 403 Cloudflare 质询（blocked，其 `.html` 为质询页，无 `.txt`） |
| 23:11 | 2：govinfo BILLSTATUS XML 两份、govinfo 详情页、Casar 首页、Sanders 新闻列表 | BILLSTATUS ok（`a-hr10538-billstatus.xml`、`a-s5493-billstatus.xml`，updateDate 2026-10-14）；Sanders 403（Akamai，blocked） |
| 23:15 | 3：CNBC、FourWeekMBA、TechTimes、Quartz、RedState、BigGo、The Standard、Casar 新闻稿、Reuters 转载（Claims Journal） | Quartz 403（改浏览器读，`a-qz-sanders-opencli.txt`）；其余 ok。BigGo、FourWeekMBA、RedState、TechTimes 只作线索，未引 |
| 23:16 | 4：LoC 失效法案页、congress.gov 词汇表、Senate 词汇表、宪法第 20 修正案页、Tom’s Hardware（Ansari）、Yahoo、Guardian 实时稿、Axios、USA Today、ABC、CNA、State of AI | 词汇表、宪法页、Axios 403（改浏览器读）；其余 ok |
| 23:18 | 5：AP 三篇、Tom’s Hardware 数据中心公投、Tom’s Hardware Sanders 20 年、Epoch 两页 | AP 三篇 403（AP 主稿改浏览器读，`a-ap-dems-win-opencli.txt`，另两篇未读）；其余 ok |
| 23:19 | 6：datacentersontheballot.com | 静态页只有骨架（JS 渲染，还含 “Simulated returns” 占位字样），改浏览器读实时数字 |
| 23:2x | 7：Sanders 三份 PDF（One-Pager、Bill、Section-by-Section） | 全部 403，blocked；浏览器直开 PDF 也返回 “Access Denied”，未绕过 |
| 23:22 | 8（清单被 `printf` 的 `%` 截断，实取 9 条）：NBC、PBS/AP、Scripps、MIRI、ControlAI、Cybernews、网易、鉅亨網、香港 01 | Cybernews 403（改浏览器读）；香港 01 404（failed，URL 编码问题，未重试，不引）；其余 ok |
| 23:2x | 9：凤凰网、东方财富、The Next Web、The Hill、星岛 | 凤凰网 404（failed，未引，Musk 帖改取 fxtwitter 原帖）；东方财富静态页只有正文骨架，改浏览器读（`a-eastmoney-pc-opencli.txt`）；其余 ok |
| 23:24 | 10：Reuters 两篇、Guardian 两篇、CNBC、BBC、Euronext（Reuters 转载） | 全部 ok（Reuters 页 200 且为正文）；grep 全部无 superintell/Ansari/“pause on advanced” |
| 23:36 | 11：Politico、Guardian（renew calls）、IAPP、Sanders 9-03 新闻稿 | Politico 403（改浏览器读，`a-politico-earthquake-opencli.txt`）；Sanders 稿 403 / Access Denied（浏览器亦然）；其余 ok |
| 23:38 | 12：ITIF 声明、Fox News | ok |

## 二、单次请求与浏览器读取

| 时刻 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 23:12–23:13 | congress.gov H.R. 10538 的 /cosponsors、/all-actions、/committees，S. 5493 的 /cosponsors、/all-actions | opencli browser（`a-cg`，`--window background`，`eval document.body.innerText`） | 成功。`a-congress-hr10538-cosponsors.txt`、`a-congress-house-bill-10538-all-actions.txt`、`…-committees.txt`、`a-congress-senate-bill-5493-cosponsors.txt`、`…-all-actions.txt`（H.R. 12 名联署，Actions 2；S. 0 名联署，Actions 1） |
| 23:13 | `opencli browser … eval` 读 AP、LoC、宪法页等被 403 的页面 | opencli | 见上；文件名 `a-*-opencli.txt` |
| 23:14 | fxtwitter：`api.fxtwitter.com/status/<id>`：Ansari 2118054336077127797、Casar 2118414798782341488、Sanders 2118131101562454038、Musk 2118248670915404057、Ansari 回帖 2118387329694117986、Coin Bureau 2118131721145389562、Decrypt 2118304821245534676（只读其引用） | curl（`--max-time 60`） | 成功。存档 `a-x-<id>.json`（Decrypt 那条只读其引用，未存 JSON）。均为公开官方账号或媒体账号，无抓取者信息 |
| 23:2x | Epoch：`all_ai_models.csv`（6.9 MB）、`large_scale_ai_models.csv`（1.1 MB） | curl（`--max-time 120`） | 成功；CSV 放会话临时目录，**不入库**；入库的是抽取表 `a-epoch-1e25-extract.tsv`（109 行）。抽取脚本在会话临时目录（`epoch.py`、`epoch2.py`、`epoch3.py`），未入库 |
| 23:37 | `barking heat`：HN “Ban Artificial Superintelligence Act”“Ansari superintelligence”“Sanders superintelligence ban”（自 2026-09-01）；Reddit 同题 | barking | HN 成功（主帖 49553463：62 分 / 83 评，09-04 01:20 北京；11 月无相关帖）；Reddit 被重定向到登录页，失败，已记入 `a-heat.json` errors。HN 评论 `a-hn-49553463.json`（Algolia） |
| 23:3x | datacentersontheballot.com 实时数字 | opencli browser | 成功，页面标 “UPDATED NOV 7, 4:58 AM ET” = 2026-11-07 17:58 北京；摘录 `a-dcotb-opencli.txt` |
| 全程 | Exa 搜索（web_search_exa）与内置 WebSearch | 搜索 | 仅用于发现 URL 与读摘要；Axios 与 Sanders 官网两处的引语来自搜索摘录，已在证据表标明 |

## 三、截图（`docs/2611/1106/images/`，A 组 01–12）

方法与限制：
- 01–07：govinfo PDF 用 `barking shot pdf` 渲页（144 dpi，1224×1584），再用 `barking shot stack` 按行号裁切并跨页拼接（x 200–1020，只裁切不改像素；每页行高 52 px，按 `150 + 52×(行号−1) − 14` 取边界，裁切边界落在行间）。宽 820 px，是 PDF 144 dpi 的原生像素（按 72 dpi 页面计为 2 倍）。每张图保留节标题与行号，跨页处用 24 px 白色间隔，页码行已裁去。官方页脚 “VerDate Sep 11 2014 15:58 Sep 26, 2026” 未入镜。
- 08–10：X 官方嵌入页（`platform.twitter.com/embed/Tweet.html?id=<id>`）在 opencli 浏览器里匿名渲染，`document.documentElement.style.zoom='2'` 后截图（1400×1400），再 `cropbox` 裁出卡片本身（1100 px 宽，2 倍像素版式）。卡片里的 “Follow” 是嵌入页自带按钮，不是抓取者账号信息；右侧浏览器翻译插件的悬浮图标在裁切框外。Sanders 卡片后台截图超时（`Page.captureScreenshot timed out`），改 `--window foreground` 成功。
- 11：congress.gov 联署页，opencli 前台截图 1400 px 宽（**DPR 1**，不是 2 倍；页面本身 1200 px 版心），裁掉顶栏（含 Sign In）与站点页脚，保留标题、概要框（Sponsor、Committees、Latest Action、Tracker）、各标签页计数（Cosponsors (12)）与联署名单全部 12 行。
- 12：鉅亨網页面（opencli 前台截图 1400 px，DPR 1），裁出标题、日期与网站自动生成的“AI新聞摘要”框。页面上有网站广告与导航，均未入镜。

| 文件 | 来源 | 内容 | 目视确认 |
|---|---|---|---|
| `01-bill-sec3-advanced-ai.png` | govinfo PDF 第 4–5 页 | Sec. 3 抬头、(1)(A)(B)：advanced AI system、10^25、年度调整（含 5 页首两行） | 已逐张打开，文字清楚 |
| `02-bill-sec3-asi-pause-def.png` | 第 5–6 页 | Sec. 3(2) 超级智能两项、(3)、(4) mandatory pause（含 6 页首三行） | 已确认 |
| `03-bill-sec3-precursors.png` | 第 6–7 页 | Sec. 3(6)(A)–(F) 前兆特征 | 已确认 |
| `04-bill-sec8-pause.png` | 第 9–10 页 | Sec. 8 全节：(a)(b)(c) | 已确认 |
| `05-bill-sec9-prohibitions.png` | 第 10–11 页 | Sec. 9(a)(b)(c)(d)(1)(2)(A) | 已确认（9(d)(2)(B) 在第 11 页末尾之后未入镜；`9(d)(2)(B) withhold a pre-deployment approval` 见证据表文本） |
| `06-bill-sec13-penalties.png` | 第 14–15 页 | Sec. 13(a)(b)(c)(1)：定义、20 年、10 年禁入 | 已确认（(c)(2)(3)、(d) 在后续页，文字见证据表） |
| `07-bill-sec15-international.png` | 第 18–19 页 | Sec. 15(a)(b) 与 Sec. 16 | 已确认 |
| `08-ansari-post.png` | X 嵌入页 | Ansari 原帖卡片，1:54 PM · Nov 4, 2026，23.8K 赞（11-07 23:3x 北京） | 已确认 |
| `09-casar-reply.png` | X 嵌入页 | Casar “Yes!” 及被引 Ansari 帖，1:47 PM · Nov 5, 2026 | 已确认 |
| `10-sanders-post.png` | X 嵌入页 | Sanders 帖，6:59 PM · Nov 4, 2026，19.4K 赞 | 已确认 |
| `11-congress-hr10538-status.png` | congress.gov | H.R. 10538 概要框与 12 名联署人名单 | 已确认 |
| `12-headline-cnyes-company-death.png` | 鉅亨網 | 标题、日期、网站 AI 摘要 | 已确认 |

没有截：Sec. 12（文字在证据表 A1-6）、Reuters/Guardian 行情段落、Epoch 数据表。

## 四、自查

- 对 `docs/2611/1106/` 的 A 组文本存档（`a-*.txt/.json/.xml/.tsv/.jsonl`）grep 抓取者信息（本机用户名、`C:\Users`、`Develata`）：无命中。
- X 相关存档仅为公开官方账号与公开媒体账号的 fxtwitter JSON；未存任何登录态页面；congress.gov 等页面匿名读取。
- 文本存档里单个超过 1 MB 的：无（`.html` 原件有 5 个超过 1 MB，不入库：`a-abc-jeffries.html`、`a-guardian-live-1106.html`、`a-tomshardware-ansari.html`、`a-tomshw-datacenter-45.html`、`a-tomshw-sanders-20yr.html`）。
- 未 commit、未 push、未调用 `rm`。
