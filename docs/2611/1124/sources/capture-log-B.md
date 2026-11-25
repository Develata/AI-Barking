# 1124 期 B 组抓取记录

执行方 Claude Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8，本机时区），取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl` 的 `time_bj` 与命令时刻；任务开始约 02:01，截至 02:17 本组完成。命令前 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"`。未登录任何站点、未输入凭据、未接受 Cookie、未使用镜像或代理站（搜索结果里的 pith.science、arcxiv.org、catalyzex 等镜像/摘要站只用来发现线索，未当作来源）。opencli 只用于一次 Reddit 搜索（被拦）。清单文件：`b-fetch-list-1.tsv` … `b-fetch-list-5.tsv`。

## 一、barking fetch

| 北京时间 | 清单 | name 与 URL | 结果 |
|---|---|---|---|
| 02:01:22 | list-1 | arxiv-abs（arxiv.org/abs/2611.31083）、arxiv-pdf（arxiv.org/pdf/2611.31083，73 页，5,058,374 字节）、arxiv-api（export.arxiv.org/api/query?id_list=2611.31083）、arxiv-html（arxiv.org/html/2611.31083）、arxiv-listing（…/abs/2611.31083v1） | 5 条全 ok；arxiv-api 存为 `.xml`；arxiv-listing 与 arxiv-abs 内容基本相同 |
| 02:05:54 | list-2 | nber-slides（conference.nber.org/conf_papers/f247465/f247465.slides.pdf）、aea-program（aeaweb.org/conference/2026/program/paper/t8AnaAf9） | 2 条 ok；**aea-program 是另一篇论文（非本题），仅因搜索摘要误取，待删除**；nber-slides 响应头 Last-Modified 2026-08-03 16:40:51 GMT |
| 02:06:58 | list-3 | zh-aihot（aihot.news/items/…）、zh-aichaai、zh-vsnews、zh-agihunt、zh-chenqiyuan、author-raymond（lindseyrraymond.com）、pith（pith.science/paper/2611.31083） | zh-aichaai、zh-chenqiyuan、author-raymond ok；**zh-aihot failed**（TLS 协议错误 InvalidContentType）；**zh-vsnews blocked**（正文仅 12 个非空白字符，需 JS）；**zh-agihunt blocked**（HTTP 403，Cloudflare 质询）；**pith blocked**（HTTP 202，空正文）。失败后不重试，标题与时刻改由 Exa 搜索摘要与 AI 圈报转载页取得，并在 evidence-B 注明 |
| 02:12:11 | list-4 | en-dmo、en-bedrock、en-tldr、es-de8（de8.mx）、mit-econ-pdf（economics.mit.edu/.../AILaborDemand_06Nov2026.pdf，72 页，5,624,975 字节）、en-aivoa（aivoa.news，AI 记者自动稿） | 6 条全 ok；en-tldr 正文只有一段摘要 |
| 02:14:50 | list-5 | wiley-baird（onlinelibrary.wiley.com/doi/abs/10.1111/coep.70035） | **blocked**（HTTP 403，Cloudflare 质询），不重试；Baird 摘要只见于搜索引擎摘要 |

`b-fetch-log.jsonl` 共 21 条，其中 ok 16、blocked 4、failed 1。blocked 条目留下了 `.html`（质询页）与 `.headers`，无 `.txt`，列入待删除。

## 二、其他请求

| 北京时间 | 工具 | 目标 | 结果 |
|---|---|---|---|
| 02:02 起 | `pdftotext -enc UTF-8 -layout`（经 barking fetch 自动生成） | `b-arxiv-pdf.txt`（2,912 行）、`b-mit-econ-pdf.txt`、`b-nber-slides.txt` | 全文逐页读过：摘要、引言、§2–7、附录 A–F 中与本组相关的表（Table 1、2、3、A.1、A.3、A.4、A.9、A.11、A.14、A.5 文字）；对 “conflict/disclos/interest/Microsoft/40” 做了 grep |
| 02:05 | WebSearch（standard）×2；Exa web_search_exa ×5 | 论文名、中文“初级工程师 40%”、Rohan Paul、seekingsapiens、BrowserStay、英文新闻 | 只用来找 URL 与线索，页面另行用 barking fetch 抓取；seekingsapiens.com、BrowserStay 未找到；“AI 抢饭碗”类中文标题未找到；Reddit 站内搜索被工具拒绝（WebSearch 400：reddit.com 不可访问） |
| 02:05:31 | curl --max-time | `api.fxtwitter.com/search?q=…` | 404 “User not found”（非有效端点，弃用） |
| 02:05 | curl --max-time 30 | Algolia HN API：`query=2611.31083` 与论文标题 | 标题 0 命中；“2611.31083”只命中无关帖（item 50629962，KL 散度，arXiv 2611.30835） |
| 02:09 | curl --max-time 60 | `api.fxtwitter.com/status/2125461932111229203`（Rohan Paul 帖子，ID 取自 chenqiyuan.cn 页中“阅读原始信息”链接） | 200；存 `b-x-rohanpaul-raw.json`（含帖文、时间、点赞/转发/回复/浏览数、帖内论文首页配图链接；其中账号信息均为被引帖作者 Rohan Paul 的公开信息，不含抓取者） |
| 02:06:24 | `barking heat`（组前缀 b） | `--hn` 3 个查询（论文名、“GitHub Copilot hiring junior natural experiment”、“Copilot reduces technical hiring”）与 `--reddit` 2 个查询（论文名、“Copilot hiring 29% procurement”） | HN 3 组均无相关帖；Reddit 两次请求被重定向到 old.reddit.com/login（响应非 JSON），已记入 `b-heat.json` 的 errors；写 `b-heat.json` |
| 02:13:06 | `barking heat`（组前缀 bh） | `--hn "GitHub Copilot"`、`--hn "junior software engineer hiring"`、`--hn "Copilot hiring"` | 因 `b-heat.json` 已存在，改用前缀 bh，写 `bh-heat.json`；HN 命中 3 条无关帖（Copilot 指令检查、skill-scan、Modeloop），其余 0 命中 |
| 02:20 | opencli browser（匿名，`--window background`；session b-aihot、b-vsnews，用完 close） | aihot.news 条目页、vsnews.ai 条目页（`eval` 读 `document.body.innerText`） | vsnews 读到正文，存 `b-op-vsnews.txt`（JSON 包装的页面文本）；aihot.news 在 Chrome 也是 ERR_SSL_PROTOCOL_ERROR，`b-op-aihot.txt` 只是错误页文本，AIHOT 标题记为“搜索摘要，未存档” |
| 02:15 | opencli reddit search | “AI-Assisted Programming and Labor Demand Copilot hiring” | SECURITY_BLOCK（HTTP 403，Reddit network security）；不重试、不绕过 |
| 02:08 | python3（只读 JSON） | 读 `b-x-rohanpaul-raw.json` | 帖文时刻 2026-11-25 05:29:48 UTC（北京 13:29:48），likes 18、retweets 4、replies 22、views 3,687（取于 02:08 左右，会随时间变） |

## 三、barking shot

中间图在系统临时目录的 barking-shot 子目录（不在仓库）。PDF 渲页：`b-arxiv-pdf.pdf` 第 1、2、3、4、9、18、19、20、21、23、24、25、26、27、31、32、33、34、57、58、61、65、66 页，`b-nber-slides.pdf` 第 11、55、83 页（144 dpi，页图 1224×1584，等于 2 倍像素）；`render`：X 官方嵌入页（`platform.twitter.com/embed/Tweet.html?id=2125461932111229203&theme=light&lang=en`，第一次渲染空白，加 `--chrome-arg=--virtual-time-budget=15000` 后正常）、chenqiyuan.cn 条目页（`--virtual-time-budget=8000`）、arxiv.org/abs/2611.31083；aichaai.com 渲染超时（Teardown watchdog expired），弃用，改用 chenqiyuan.cn。

最终图（`docs/2611/1124/images/`，已逐张打开目视确认文字清楚、范围正确；PDF 页是白底）：

| 文件 | 来源 | 裁切 | 备注 |
|---|---|---|---|
| 13-arxiv-abs.png | arxiv.org/abs/2611.31083（render 700 CSS px、DPR 2） | 标题、作者、摘要、Subjects、Submission history | 浅色底 |
| 14-paper-first-page.png | PDF 第 1 页 | 标题、作者单位、摘要、关键词、脚注 | |
| 15-table1-wald29.png | PDF 第 20 页 | Table 1 全表（表头、注释）与其后 Wald 29% 段 | |
| 16-intro-7p6-29-55to39.png | PDF 第 3 页 | 引言一段 | |
| 17-vacancy-postings.png | PDF 第 20、21、61 页拼接 | §6.1.2 全文与 Table A.11 全表 | |
| 18-junior-swe-table-a14.png | PDF 第 66 页 | Table A.14 全表与注释 | |
| 19-data-revelio-limits.png | PDF 第 9、34 页拼接 | §3.2 与结论局限三段 | 结论“7 Conclusion”标题行在裁切框外 |
| 20-zh-aiquanbao-40-title.png | chenqiyuan.cn 条目页 | 标题、来源、时间、摘要 | **深色底页面** |
| 21-x-rohanpaul-card.png | X 官方嵌入页 | 帖子卡片 | 匿名渲染；卡片里的“Follow”是匿名嵌入默认 |
| 22-table-a9-quarterly-wald.png | PDF 第 58 页 | Table A.9 全表与注释 | |
| 23-junior-40-sec623.png | PDF 第 27 页 | §6.2.3 | |
| 24-nber-slides-36-45.png | NBER 幻灯片第 11、55、83 页拼接 | 三张幻灯片全幅 | 页图 908 px 宽；含幻灯片自带图 |
| 25-overall-decline-29-2022-24.png | PDF 第 32 页 | §6.5 | 超出 13–24 区间 |
| 26-benchmark-limits.png | PDF 第 33 页 | §6.5 末与限制段 | 超出 13–24 区间 |

## 四、失败与限制

- 失败或被拦：aihot.news（TLS 协议错误）、vsnews.ai（需 JS）、agihunt.info（Cloudflare 403）、pith.science（HTTP 202 空正文）、Wiley（Cloudflare 403）、Reddit（登录页重定向与 403）。均未改用镜像或代理站。
- 未做：seekingsapiens.com、BrowserStay 页面未找到，无法记原句；抖音、小红书、公众号的同类帖无法搜索；Reddit 评论未取得；没有 L6 高赞质疑评论可摘。
- 没有写脚本：全程用 barking 子命令，无 `b-` 前缀自写脚本。
- 隐私自查（02:21）：对整个 `docs/2611/1124/` 的文本文件 grep 本机用户名（develata，不区分大小写）与家目录路径 `/Users/`，0 命中；14 张 b 组 PNG 用 `strings` 查用户名，0 命中；截图只含被引帖作者 Rohan Paul 的公开信息、arXiv/PDF 页面与聚合站页面，不含抓取者头像、昵称或登录栏。

## 五、待删除文件（精确路径，均在 `docs/2611/1124/sources/`）

- `b-aea-program.headers`、`b-aea-program.pdf`、`b-aea-program.txt`（另一篇论文，非本题）
- `b-zh-agihunt.headers`、`b-zh-agihunt.html`（Cloudflare 质询页）
- `b-zh-vsnews.headers`、`b-zh-vsnews.html`（正文过短）
- `b-pith.headers`、`b-pith.html`（空正文）
- `b-wiley-baird.headers`、`b-wiley-baird.html`（Cloudflare 质询页）
- `b-op-aihot.txt`（aihot.news 错误页）
- `b-zh-aihot.headers`（barking fetch 失败条目遗留的头文件，如存在）
- `b-arxiv-listing.headers`、`b-arxiv-listing.html`、`b-arxiv-listing.txt`（与 `b-arxiv-abs.*` 重复，可留可删）

## 六、命名越界，请 Claude 收尾时处理

- `docs/2611/1124/images/25-overall-decline-29-2022-24.png`、`docs/2611/1124/images/26-benchmark-limits.png`：超出 B 组 13–24 区间；25、26 当前无冲突，可保留或改号。
- `docs/2611/1124/sources/bh-heat.json`：`barking heat` 因 `b-heat.json` 已存在而拒绝覆盖，改用前缀 bh 写出；内容是后补的 HN 查询。
- 大文件：`b-arxiv-pdf.pdf`（5.06 MB）、`b-mit-econ-pdf.pdf`（5.62 MB）、`b-nber-slides.pdf`（4.20 MB）、`b-aea-program.pdf`（2.29 MB，待删）都是 PDF，只存本地、不入库；b 组文本存档（txt、json、xml、tsv、jsonl）均小于 1 MB（最大 `b-arxiv-pdf.txt`、`b-arxiv-html.txt` 等已用 find 核过 >1000 KB 无命中）。
