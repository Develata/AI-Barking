# 1005 A组抓取日志

所有时间为北京时间UTC+8。工作目录E:\gitclone\AI-Barking，固定session `1005-a`。无commit、push、删除，无修改1004及其他组文件。仅A组。完整逐次页面/API时刻在`a-fetch-log.jsonl`；截图实际区域在`a-shots-log.jsonl`。

## 页面采集

下列均为`opencli browser 1005-a open/eval`读取公开可见DOM、元数据与链接；落盘同名`a-*.json/.txt`。标题、正文逐一人工核读，不把HTTP成功当事实核验。

| 北京时间（2026-10-06） | URL | 文件前缀 | 结果 |
|---|---|---|---|
| 09:14:08 | https://diff.wikimedia.org/2026/10/05/openai-rogue-agent-activities-found-on-wikimedia-projects/ | a-wikimedia | 全文8768字符；明确署名，含published/modified |
| 09:14:11 | https://wikitech.wikimedia.org/wiki/Incidents/2026-05-13_wdqs | a-wdqs | 全文6894字符；Summary、时间线、结论完整 |
| 09:14:19 | https://collusion.wiki/ | a-collusion | 5203字符；正文/可见摘要 |
| 09:14:27 | https://transluce.org/agent-activity | a-transluce | 9595字符；正文 |
| 09:14:36 | https://rubyhack.ai/ | a-rubyhack | 3135字符；公开可见正文；未展开所有交互附录 |
| 09:14:43 | https://metr.org/blog/2026-08-26-openai-hugging-face-incident-investigation/ | a-metr | 178014字符；只摘开头背景、不深挖附录 |
| 09:14:59 | https://cryptobriefing.com/wikimedia-openai-rogue-bots-may-outage/ | a-cryptobriefing | 4424字符；核后不能认定确定夸大 |
| 09:15:18 | https://news.ycombinator.com/item?id=49968105 | a-hn | 52977字符；256分178评论，未登录 |
| 09:15:24 | https://aihot.news/daily/2026-10-05 | a-aihot | 857字符；该日报无目标条目 |
| 09:16:22 | https://openai.com/hugging-face-incident-and-misalignment/ | a-openai-timeline | 自动重定向中文，保留18448字符，不用于英文零命中结论 |
| 09:16:37 | https://openai.com/index/hugging-face-incident-and-the-road-ahead/ | a-openai-report | 自动中文，16944字符，保留版本 |
| 09:16:55 | https://alignment.openai.com/misalignment-reports/ | a-openai-alignment | 1789字符；12 reports、3 notices，DSEwiki条目 |
| 09:17:03 | https://www.theverge.com/news/1004929/wikipedia-openai-rogue-bots-wikimedia-foundation-outage | a-verge | 5369字符；含OpenAI新增回应 |
| 09:17:22 | https://aihot.news/daily/2026-10-06 | a-aihot-oct6 | 2237字符；发现目标条目链接 |
| 09:17:29 | https://www.ic.work/article/wikimedia-flags-openai-rogue-agents-behind-may-outage | a-icwork | 4658字符；原站夸大语句 |
| 09:17:36 | https://pollar.news/en/event/wikimedia-flags-rogue-openai-agents | a-pollar | 6021字符；AI-generated，因果限定丢失 |
| 09:18:08 | https://www.reuters.com/technology/wikipedia-operator-says-openais-rogue-agents-possibly-tied-data-service-2026-10-05/ | a-reuters | 5687字符；当前已有回应，无所要求旧句 |
| 09:18:17 | https://aihot.news/items/ncv6u97zqan3hgzng59yel19f | a-aihot-item | 3035字符；评分78 |
| 09:18:25 | https://openai.com/en-US/hugging-face-incident-and-misalignment/ | a-openai-timeline-en | 30710字符；最终规范英文URL，grep采用此版 |
| 09:18:38 | https://openai.com/en-US/index/hugging-face-incident-and-the-road-ahead/ | a-openai-report-en | 38123字符；最终规范英文URL，grep采用此版 |

## CSV与修订元数据

约09:14—09:15，curl原站 https://security.wikimedia.org/data/openai-wikimedia-edits-2026-10-04.csv ，普通Mozilla User-Agent，HTTP200、4795 bytes；确认是54条URL，不是challenge。存`a-wikimedia-edits.csv`原样。

`a-csv-audit.mjs`成功执行。依次请求en.wikipedia.org、test.wikipedia.org、test2.wikipedia.org、www.mediawiki.org、commons.wikimedia.org、simple.wikipedia.org、incubator.wikimedia.org、meta.wikimedia.org、bg.wikipedia.org的`/w/api.php?action=query&prop=revisions`，rvprop仅ids|timestamp，按该域revid批量查询。每次完整URL、北京时间见`a-fetch-log.jsonl`。54/54成功，原始API JSON共9份，派生统计`a-csv-statistics.json`。没有抓修订载荷、IP或用户资料。必要查询参数保留。

CSV截图未完成；后补HEAD确认原站Content-Type为text/csv；尝试由1005-a进入原站CSV的view-source视图，OpenCLI明确拒绝该scheme（只允许http/https）。未绕过限制、未让浏览器下载到范围外目录，不生成本地表伪装原站截图。A2明确部分支持。HEAD响应未落盘，日志不保存响应Cookie或客户端IP。

## 截图

原站页面，CSS宽700、deviceScaleFactor2，截图没有重绘、改字、改数字；浏览器窗口切换只用1005-a。所有实际截图文件都在01–09区间。

| 文件 | 结果 |
|---|---|
| 01-a-wikimedia-opening.png | 已目视核对：标题、署名、日期、其他wiki限定与前两段；1400×1384 |
| 02-a-wikimedia-no-compromise.png | 不合格：缩放/裁切异常，右侧截断；保留失败记录，不可发布 |
| 03-a-wikimedia-summary.png | 已目视核对：调查段、未发现攻破/协调句、In summary及三条完整列表；1400×2540 |
| 04-a-wdqs-summary.png | 已目视核对：Summary表全部行列、Impact、Aggressive scrapers段；1400×1300 |
| 05-a-wdqs-timeline.png | 已目视核对：Timeline、UTC说明、首尾及中间条目；1400×2580 |
| 06-a-wikimedia-context.png | 不合格：opencli scroll/screenshot备用抓取缩放、裁切异常；保留勿用 |
| 07-a-wdqs-scrapers.png | 已目视核对：爬虫段、原站图表及图例、1/128采样与漏抓scraper段；1400×2300 |

04右侧有浏览器翻译浮标，未挡关键数值；未修改原图移除。截图坐标、精确时间见`a-shots-log.jsonl`。03替代失败02的证据区域，01+03覆盖A1要求。

## 社交采集

- HN原帖09:15:18快照：256分178评论；约09:25再次进入同帖，从公开DOM只提取所选评论正文、永久链接与可见score（没有）。写`a-hn-selected.json`。不据页面排序推定高赞。
- Reddit原站 https://www.reddit.com/r/neoliberal/comments/1wyjlak/wikipedia_operator_says_openais_rogue_agents/ ，约09:25进入；09:26:21.895抽取post标题、score、comment-count和选中公开评论的text/score/permalink，写`a-reddit-selected.json`。73分48评论；评论41/24/5分等已记。页面标题被浏览器翻译插件改成双语，但所存评论是英文。未保存全页导航、头像、登录者名字。
- X约09:26，`OPENCLI_BROWSER_COMMAND_TIMEOUT=150`，1005-a搜索：`from:OpenAI (Wikimedia OR Wikipedia OR Etherpad) since:2026-10-05 until:2026-10-07`，Latest。页面明确“没有结果”，articles空数组。只读帖子卡片/空状态；未存抓取者资料；结果不等于未发布。链接身份查询参数用于搜索，不保留src追踪参数作来源。
- 已通过agent-reach doctor看路由，Reddit/X没有实时验证的active_backend但OpenCLI桥已连。采用用户要求的固定browser session，不使用另建session的适配器。

## 搜索发现与原站回溯

工具web搜索只用于找链接，最终事实核验采用原站DOM/官方API。约09:13—09:17执行3批共10条query：

1. `Wikimedia OpenAI rogue agents October 5 2026 Reuters TechCrunch Verge`；`"Wikimedia links OpenAI’s rogue bots to May outage"`。
2. `site:reuters.com "Wikipedia operator" "2026-10-05"`；`site:techcrunch.com/2026/10/05 Wikimedia OpenAI`；`site:404media.co Wikimedia OpenAI October 5 2026`；`"OpenAI" "维基百科" "10月6日" "2026"`。
3. `"Wikipedia operator says" "reuters.com" "October 5, 2026"`；`site:404media.co "Wikimedia" "OpenAI"`；`site:openai.com "Wikimedia" "2026"`；`site:theverge.com/news/1004929`。

Reuters/404限定搜索各2次后未再扩搜。搜索结果中的转载/代理链接不拿来取证。Reuters原站URL由Reddit公开帖外链定位。

web直接open The Verge失败；open Reddit成功；click Reuters外链失败。随后opencli在原站成功取得The Verge和Reuters正文。TechCrunch/404本事件同日报道未找到，不能写“没有报道”。

## 失败与边界

1. 最初OpenAI页面自动中文；随后从/en-US/入口重新读英文，保留两版并明确使用英文grep。
2. 首轮`a-shots.mjs` Page.captureScreenshot超时115秒，后有02/06缩放裁切异常。尝试停止本组卡住截图进程被自动审批拒绝，返回`approval required by policy, but AskForApproval is set to Never`；未换手段强杀。该进程后来自行超时退出。
3. `a-capture.mjs`首次尝试CDP Runtime.evaluate，被桥接层拒绝：`CDP method not permitted: Runtime.evaluate`。未绕过方法限制，改用已允许的opencli只读eval取得坐标；CDP仅做获准的Emulation及截图。单次截图脚本加30秒期限，重新导航原站后成功。
4. 重拍02时wx拒绝覆盖EEXIST；未覆盖/删除，03完整覆盖必要证据。
5. “curl HN+Python解析+读取Reuters”组合命令被自动审批在启动前拒绝，原因同上；未落盘该命令的输出。后改用范围更小的公开浏览器DOM读取。
6. `a-build-report.py`汇总脚本被自动审批拒绝，未执行，未换入口运行。本次两份Markdown由明确范围的文件编辑直接写入。脚本里设想的manifest/验证结果不算实际完成；保留该脚本作为尝试记录，不交付不存在的manifest。
7. 早期rg在PowerShell下把glob放进文件路径，报路径语法错误；改为目录配-g。读取两份尚未落盘文件时曾报不存在，等抓取完成后读取，不用空文件作证据。
8. Reuters未即时回复旧句未取到原站历史版本；Reddit旧转述不替代它。CSV截图缺失。

未登录、输入凭据、接受Cookie、发布互动；未用镜像/代理域名，未绕付费墙。所有本轮写入仅1005本组路径。

## 隐私及工作区检查

对1005文字检索Develata/QQ/auth_token/ct0/access_token/refresh_token/sessionid并查看命中：本组只命中公开网站facebook-domain-verification中的qq子串、公开DseWiki路径OpenAIOct07中的ct0子串，均非抓取者信息；无本组抓取者头像/handle/会话凭据。CSV账号是原始公开页面路径，依派工保留。没有修改其他组数据。

初始及收尾HEAD均为 `ac064e681343b7ce6401448a250b1ab2f01d4c63`；git status相对开工仅多1005目录。原有EDITORIAL.md、daily-scan.md修改，以及1004、派工单的未跟踪状态均保留。未提交、推送、删除。

环境检查：opencli doctor的daemon/扩展/连通性正常，OpenCLI1.8.7。Agent Reach check-update报告1.5.0已最新。最后实际只读验证：32份a-*.json全部通过ConvertFrom-Json；7个PNG读取IHDR均宽1400；62份源文件（新增本清单前）无单文件超过1MB。清单见a-files.md；不得以未执行的汇总脚本内容替代验证。

收尾北京2026-10-06 09:36:23再次实际核验：A组sources63、images7，合计70；两份JSONL逐行可解析；本组无>1MB文件；a-*.json/.txt检索auth_token/access_token/refresh_token/sessionid/Develata无命中。原CSV SHA-256：`300511bb7b90a7b2a5bfae4c7d80061b16617f16e4242204cb5c264cc32cc9c6`。此时其他组已并发新增1005材料，以上数量与验证仅指本组；没有把整个1005目录的变化归为本组。
