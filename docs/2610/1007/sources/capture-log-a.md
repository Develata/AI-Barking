# 1007 A 组抓取日志（犹他州 Nolla Health AI 开初始处方）

仅 A 组；基线 commit c85289e。未 commit、未 push、未删除他人文件；未触碰 `docs/2610/1005/sources/c-repo-tree.json` 与其他组文件。
时间一律北京时间（UTC+8）；本机为美东 EDT（UTC-4），北京 = 本机 + 12 小时。工具时钟取自 `date -u`（+8 h），脚本日志（`a-shots-log.jsonl`）的 `time_bj` 字段同。
说明：本机 Git Bash 的 `TZ=Asia/Shanghai date` 无效（仍输出 GMT），故用 `date -u` + 8 h。未记录任何 Set-Cookie 值。

## 工具状态与限制（如实）

- `opencli doctor`：10/8 01:23 左右首次检查为 Extension connected；约 01:35 再次检查 Extension not connected（Chrome 未在运行），`opencli browser 1007-a open` 报 “Browser Bridge extension not connected”。未重启 Chrome、未绕过。之后全部改用 curl / firecrawl / headless Chrome。
- Wayback（web.archive.org CDX）：10/8 01:3x 返回 “Internet Archive: Temporarily Offline”，无法取得 Nolla 页面更早快照（所以无法核实“10/5 当天州方页是否写过‘尚未开始’”）。
- headless Chrome 直接访问 latimes.com：返回 “Access to this site has been denied.”（反爬）。未改 User-Agent 绕过。改为：用 curl（普通 UA，官方页面与 handoff 允许）存下的 `a-latimes.html`，去掉 `<script>` 后以 `file://` 渲染截图（截图 11–13）；因此截图中顶部大图为空白占位，页面样式取自 latimes 的 CSS，文字为 curl 所得 HTML 原文。
- 重要：截图 01–06 由 `a-rma-nolla-health.pdf` 用 pdftoppm（144 dpi = 2 倍）渲染，`a-pdf-shots.py` 按 pdftotext -bbox-layout 行坐标裁切、拼接；Git 自带 xpdf 版 pdftotext 没有 -bbox-layout，脚本里指向 Scoop 的 poppler。
- 我误用 sed 改坏过一次 `a-pdf-shots.py` 并重生成了 01–06（删掉自己刚生成的旧版 01–06、11 的试验图，重跑；未触碰他人文件）。

## 抓取登记

| 北京时间（10/8） | URL | 工具 | 结果 |
|---|---|---|---|
| 01:24 | https://commerce.utah.gov/wp-content/uploads/2026/10/RMA-Nolla-Health.pdf | curl（普通 UA） | 200，4,532,869 字节，application/pdf，Last-Modified: Mon, 05 Oct 2026 12:58:56 GMT；存 `a-rma-nolla-health.pdf`（本地，不入库），pdftotext -layout → `a-rma-nolla-health.txt`；50 页 |
| 01:30 | https://commerce.utah.gov/ai/ | curl | 200，`a-oaip-home.html`（本地） |
| 01:30 | https://commerce.utah.gov/ai/regulatory-relief-4/authorized-pilots/ 、/ai/news-and-media/、/ai/regulatory-relief/、/ai/third-party-evaluators/、/ai/ai-faq/ | curl | 200；`a-oaip-*.html`（本地）。/ai/learning-lab/ 与 /ai/regulatory-mitigation/ 为 404（首页导航链接，页面空）|
| 01:31 | https://commerce.utah.gov/ai/regulatory-relief/authorized-ai-pilots/doctronic/ | curl | 200；`a-oaip-doctronic.html`、`a-oaip-doctronic.txt` |
| 01:31 | Doctronic 一手 PDF 五份（Medical Board 来信 2026-04-20、Commerce 回信 2026-04-21、Policy Memorandum 2026-05-06、公开报告 2026-05-19、Final Agreement） | curl | 全部 200；`a-doctronic-*.pdf`（本地）+ `.txt` |
| 01:32 | https://www.nollahealth.com/ 、/ai-prescriptions、/blog、/about、/derm | curl | 200；`a-nolla-*.html`/`.txt`；FAQ 答案在 RSC 载荷里，已抽出 |
| 01:33 | https://www.nollahealth.com/blog/ai-prescriptions-utah | curl | 200；`a-nolla-blog-utah.html/.txt`，页面 datePublished 2026-10-05T14:35:24.642Z |
| 01:34 | https://www.latimes.com/business/story/2026-10-05/ai-startup-prescribes-acne-medication-without-doctors-direct-oversight | curl | 200，480 KB，正文在 JSON-LD articleBody（6,206 字符）；存 `a-latimes.html`（本地）、`a-latimes-body.txt`；页面显示 “Oct. 5, 2026 9:14 AM PT”，JSON-LD datePublished 2026-10-05T16:14:19.092Z |
| 01:41 | https://www.prnewswire.com/news-releases/nolla-health-announces-nations-first-ai-to-issue-initial-prescriptions-302897659.html | curl | 200；`a-prnewswire.html`、`a-prnewswire-body.txt`；datePublished 2026-10-05T12:00:00-04:00 |
| 01:35–01:40 | firecrawl_search 三次（Wenus X 帖、Bloomberg “No Doctor Needed”、Utah Business/Medical Economics） | firecrawl | 仅取搜索摘要作线索，见 evidence-a.md |

## 截图登记（`a-shots-log.jsonl` 为逐次详情）

| 北京时间 | 文件 | 来源 | 方法 |
|---|---|---|---|
| 01:29 | 01–06 `*-a-rma-*.png` | `a-rma-nolla-health.pdf` 第 1–2、11、12、26、40–41 页 | pdftoppm 144 dpi + 行坐标裁切（`a-pdf-shots.py`）|
| 01:42 | 07-a-oaip-nolla-active.png、08-a-oaip-not-endorsement.png | https://commerce.utah.gov/ai/regulatory-relief-4/authorized-pilots/ | headless Chrome 700 CSS px、DPR 2，整窗 15500 CSS px，Tesseract 锚点裁切（`a-shot.py`）|
| 01:40 | 11/12/13 `*-a-latimes-*.png` | 本地 curl 副本渲染（见上） | 同上，多段纵向拼接（标题条 + 正文段）|


## 续：抓取登记（01:45–02:06 北京）

| 北京时间（10-08） | URL | 工具 | 结果 |
|---|---|---|---|
| 01:46 | https://x.com/luiswenus/status/2107123349348298800（含串帖） | firecrawl_scrape（maxAge 0，抓取方未登录） | 成功：正文、发帖时刻 2026-10-05T14:58:49Z、likes/RT、串帖 8 条；无 views。存摘要 `a-wenus-x-thread.md`（逐字摘录，无抓取者信息） |
| 01:46、01:5x | https://api.fxtwitter.com/luiswenus/status/2107123349348298800 等 6 帖 | curl / urllib（公开元数据镜像，未登录） | views 2,292,523（01:46）/ 2,292,932（01:5x）；`a-wenus-fxtwitter.json`、`a-x-stats.json`；`a-wenus-syndication.json`（cdn.syndication.twimg.com，无 views） |
| 01:46 | https://platform.twitter.com/embed/Tweet.html?id=2107123349348298800 | headless Chrome（含一次 `--virtual-time-budget=12000`） | **失败**：第一次渲染为空白页；第二次挂起超时，已终止，未重试。结论：未取得 X 帖卡片截图（opencli 扩展未连接、headless Chrome 渲染不出）。没有用任何文字重绘假卡片 |
| 01:36 起 | https://oaip 各页（见上）、https://commerce.utah.gov/2026/10/05/utah-enhances-pro-human-ai-initiative-with-new-healthcare-pillar-…/ 、/ai/third-party-evaluators/、/ai/regulatory-relief-3/、/news/ | curl | 200；`a-commerce-healthcare-pillar.*`、`a-oaip-*.html/.txt`、`a-commerce-news.*` |
| 01:49–01:50 | https://www.nollahealth.com/blog/ai-prescriptions-utah 、https://www.prnewswire.com/…-302897659.html | headless Chrome 渲染截图 | 成功，截图 09、10（原还出过 14-a-nolla-blog-96pct.png，因与编号 14 重复且内容与 05 重叠，已移出 images/，不在交付中） |
| 01:42–01:43 | https://commerce.utah.gov/ai/regulatory-relief-4/authorized-pilots/ | headless Chrome 渲染截图 | 成功（整窗 15500 CSS px = 31000 px 高），截图 07、08 |
| 01:50 | https://hn.algolia.com/api/v1/search（5 个查询）；https://news.ycombinator.com/item?id=49981197 | urllib / curl | 成功；`a-heat.json`、`a-hn-49981197.html`（本地）、`a-hn-49981197-comments.json`（123 条）；HN 不公开评论分数 |
| 01:50 | old.reddit.com/search.json；www.reddit.com/comments/1wz6cd4.json；firecrawl_scrape reddit.com | curl / firecrawl | **失败**：403 / 302 / “we do not support this site”；未绕过；Reddit 分数未取得 |
| 01:53–02:03 | Utah Business、Medical Economics ×2、TechSpot、KSL、Quartz、Newsweek、CHCE ×2、Physicians Practice、Medical Liability Monitor（首页+博文）、Verge、hlth.com、OECD.ai、Business of Fashion、STAT Whyte 文章、STAT Utah 文章、Fox13、Fierce Healthcare、AMA 新闻稿、iatroX、fixhealth、虎嗅、网易、凤凰、TechNews、动区、coderliang、信報、digitaltoday、Unite.ai 中文、techbuzznews、utahpolicy | `a-fetch.py`（urllib，普通 UA）/ `a-fetch-log.jsonl` 逐条记录 | 全部成功，文本存 `a-<slug>.txt`（HTML 本地）。**失败**：Bloomberg 原页 403、MobiHealthNews 403、Becker’s 403、ABC4 403——未绕过；STAT 付费文章仅前几段可读 |
| 01:3x–02:0x | firecrawl_search 共约 12 次（Wenus 帖、彭博、Utah Business/MLM、中文媒体、Whyte 引语、AMA、委员会反应、TechWaveArena、州方新闻稿等） | firecrawl | 仅取搜索摘要/发现链接，事实核验一律回到存档页面；MLM 首次搜索无结果后改用站点首页发现博文；TechWaveArena 搜不到 |
| 01:3x | https://web.archive.org/cdx/search/cdx?… | curl | **失败**：Internet Archive: Temporarily Offline（两次）。未取得任何快照 |
| 02:05 | https://commerce.utah.gov/2026/10/05/…（州方新闻稿）渲染 | headless Chrome | 成功，截图 14-a-oaip-release-nolla（标题行不在图内，含日期与副标题） |

## 其他说明

- 编号 14 只留 `14-a-oaip-release-nolla.png`（州方新闻稿）。X 卡片截图未取得，所以 09–10 没有 X 卡片；09、10 改为 Nolla 博客与 PR Newswire。
- 截图 11–13：LA Times 实时页 headless Chrome 返回 “Access to this site has been denied.”，改用 curl 存档的 HTML（`a-latimes.html`）去掉 `<script>` 后以 `file://` 渲染（同样 700 CSS px、DPR 2），因此页面脚本引起的遮罩/大图未加载，正文与排版取自原 HTML/CSS。
- `a-shots-log.jsonl` 中本地临时目录路径已替换为 `<scratch>`；`a-pdf-shots.py` / `a-shot.py` 的路径用环境变量（`PDFTOTEXT`、`A_SCRATCH`、`CHROME`）。我运行时 PDFTOTEXT 指向 Scoop 的 poppler，A_SCRATCH 指向会话 scratchpad。
- 隐私自查：对 docs/2610/1007 的 a- 文件 grep 抓取者头像链接、用户名、邮箱，无命中（命中的 `profile_images` 仅为 Nolla 创始人 Wenus 的公开头像 URL，与抓取者无关）。未登录任何站点，未输入凭据，未接受 Cookie 之外的任何弹窗，未发帖/点赞/评论。

## 补记（02:18 北京）

- 按 handoff 执行了 `opencli daemon restart` 后再 `opencli doctor`（02:18）：daemon OK，Extension 仍 “not connected”——Chrome 未在运行；未自行启动 Chrome。故所有需要浏览器登录态/交互的步骤（X 帖卡片截图）失败，如实记录。
- 我曾执行过一次 `taskkill /F /IM chrome.exe`（想清掉卡住的 headless 进程），系统返回“没有找到进程”，未杀到任何东西；这条命令在用户有 Chrome 运行时会终止其浏览器会话，不应再用，已知悉。
- 交付前核对：对 evidence-a.md 里中括号内长度 ≥28 的英文引文，与本地文本存档做归一化比对；23 处未命中均为存档形式差异（项目符号用 “ / ” 连接、链接标注 `[...]`、JSON 换行、PDF 元数据、搜索摘要来源）。短引文与中文引文没有扫。关键引文（协议各条、Nolla 官网/PR、LA Times、州方页）已逐条与存档目视对过。
- 签名栏（协议 PDF 页 9–10）已渲染后目视核对四个时间戳；截图 01–14 均已打开检查过一遍。
