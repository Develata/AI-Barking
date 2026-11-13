# 1112 期 B 组抓取与截图日志

执行：Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（UTC+8，本机时区），2026-11-13。存档前缀 `b-`；机器可读日志 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`。清单 `b-list1.tsv`–`b-list6.tsv`；脚本 `b-opencli-dump.sh`（opencli 渲染后写 innerText）。所有页面匿名取得；opencli 浏览器未登录任何账号，未存 X 等社交平台页面，无需抹去抓取者信息（已 grep，见文末）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 22:29 | antigravity.google/blog/gemini-4-hydrogen-in-google-antigravity | `barking fetch`（list1） | failed（TLS 读取响应解码失败）；改 curl（`--max-time 60`）200，180,566 字节，存 `b-ag-blog.html/.headers`，`barking html2txt` 转 `.txt`（该条不在 fetch-log 里） |
| 22:29 | blog.google 主发布博客；HN 50471431、50472260 页 | `barking fetch`（list1） | 3 ok（`b-google-blog`、`b-hn-50471431`、`b-hn-50472260`） |
| 22:29 | 模型卡 PDF；tbench.ai 新闻与首页/leaderboard；antigravity docs/models、changelog、swarm、pricing；Carbon 博客；deepmind.google/models/gemini/hydrogen/ | `barking fetch`（list2） | 9 ok；`aistudio-whatsnew` blocked（正文过短，JS 渲染）；`dm-hydrogen` 404 failed（URL 猜错，不重试）。`tbench-leaderboard` 与 `tbench-home` 内容相同（站点单页） |
| 22:31 | deepmind.google/models/evals-methodology/gemini-4-hydrogen 与 deepmind.com 变体 | `barking fetch`（list3） | 两条 ok，返回同一份 PDF（267,631 字节，4 页） |
| 22:31 | antigravity docs/plans；ai.google.dev 定价与模型页；Argon 博客（antigravity 与 blog.google）、Carbon 博客（blog.google）；docs/boost；AA 方法页、AA 榜页、AA Hydrogen 页 | `barking fetch`（list4） | 8 ok；`ai-pricing`、`ai-models-h` blocked（正文过短） |
| 22:32 | AA Hydrogen high/medium/low、Argon、Carbon、Gemini 3.8 Flash 模型页；AA terminalbench-4-0 评测页；AA 指数页 | `barking fetch`（list5） | 8 ok（AA 页服务端渲染，文本含榜行；图表数据在 JS 中未取到） |
| 22:33 | Google 博客内官方图：`gemini-4-hydrogen_benchmarks_table.gif`、`_aa-index_score-time_graph.gif`、`_aa-index_graph.gif`、`_t-bench_graph.gif`（storage.googleapis.com） | curl | 4 个 200；存 `b-gimg-table.gif`、`b-gimg-aa-index_score-time_graph.gif`、`b-gimg-aa-index_graph.gif`、`b-gimg-t-bench_graph.gif`；sips 转 PNG 只作查看（在草稿目录） |
| 22:34 | tbench.ai/leaderboard/terminal-bench/4.1 与 `/4.0`（`/4.0` 跳到首页，默认 4.1） | opencli（`b-opencli-dump.sh`） | ok，`b-tbench-lb-41.txt`（42 行榜）；`b-tbench-lb-40.txt` 内容与 4.1 相同，**不是 4.0 榜**（无用，列入待删除） |
| 22:36 | hub.harborframework.com/…/terminal-bench/latest?tab=leaderboard&leaderboard=4-0-0 | opencli | 页面一直 “Loading dataset details…”，失败，`b-harbor-lb-40.txt` 无内容（待删除）；tbench.ai 下拉切 4.0 在后台与前台窗口都被拒（“Native input requires a visible browser page”），JS 事件派发也没展开，**不再重试**，4.0 榜未取得 |
| 22:37 | `barking heat`：条目 50471431、50472260；HN 查询 “Gemini 4 Hydrogen”“Antigravity Orchestrator”；Reddit 两条 | `barking heat` | HN 成功（`b-heat.json`、`b-hn-50471431.json`、`b-hn-50472260.json`）；Reddit 两次被重定向到 old.reddit.com 登录页（响应不是 JSON），不登录、不重试，记在 `b-heat.json` errors |
| 22:42 | ai.google.dev/gemini-api/docs/pricing；aistudio.google.com/docs/whats-new-gemini-4-hydrogen | opencli（`b-opencli-dump.sh`） | ok：`b-ai-pricing-oc.txt`（定价页表格内容未出现在文本）；`b-aistudio-whatsnew-oc.txt` 含价格行 |
| 23:06 | WebSearch ×4（英文、中文） | WebSearch | 找到 InfoWorld、TNW、iGeeksBlog、T2 Online、Fello AI、MLQ、The New Stack、BigGo；中文关键词只返回发布前传闻稿 |
| 23:08 | 上述 7 篇报道 | `barking fetch`（list6） | 6 ok；The New Stack 403（Cloudflare 质询，blocked），不绕过，未存正文 |

## 截图

方法：`barking shot render`（匿名 headless Chrome，`BARKING_CHROME` 指向 Chrome 应用二进制，`--lang=en-US --accept-lang=en-US`，否则页面按本机 zh-CN 渲成中文）→ `cropbox`/`stack`；PDF 页用 `barking shot pdf`（144 dpi）；Terminal-Bench 榜用 opencli `screenshot`（zoom 1.25、`--width 1400 --height 3800`）再 `cropbox`。日志 `b-shots-log.jsonl`。裁切边界落在文字行之间。2 倍像素比（PDF 页为 144 dpi，等同 2 倍；opencli 图为 1 倍 + zoom 1.25，原因：700 px 宽版式会把榜右侧列挤出页面）。

| 图 | 来源与时刻 | 方式 | 备注 |
|---|---|---|---|
| `13-ag-orchestrator-24pct.png` | Antigravity 博客，22:51 渲染 | cropbox | 含小标题、三段正文，24%/19% 句在最后；浅色 |
| `14-ag-title-date.png` | 同上 | cropbox | 页顶公告条、标题、作者、日期 “Nov 12, 2026”、“3 min read” |
| `15-ag-hydrogen-numbers.png` | 同上 | cropbox | 51/50/54/57%/18.2%/“tripiling” 一段 |
| `16-ag-availability.png` | 同上 | cropbox | “Getting started” 可用范围段 |
| `17-google-benchmarks-table.png` | blog.google 主博客，22:53 渲染 | cropbox | 页内官方跑分表（原图 GIF 末帧完整）含脚注 |
| `18-google-orchestrator-tb40.png` | 同上 | cropbox | “On Terminal-Bench 4.0” 句 |
| `19-google-price-aa-availability.png` | 同上 | cropbox | 可用范围、AA 51 对 50、价格 |
| `20-google-tb41-3x-flash.png` | 同上 | cropbox | “more than tripling Terminal-Bench 4.1 scores (57% vs. 18.2%)” 段 |
| `21-aa-leaderboard-rows.png` | AA 榜页，22:55 渲染（页面取数 22:31） | stack 4 段（表头 + Carbon high + Hydrogen xhigh + Hydrogen high 与 Argon high 两行） | 段间浅灰间隔条是工具生成；宽 2200 px（1100 CSS px × 2） |
| `22-tbench-41-leaderboard.png` | tbench.ai 首页（4.1 榜），22:50 opencli | cropbox | **深色页面**；42 行全表含表头与脚注；Tokens/Cost 列在 1400 px 宽外，没入图（文本存档 `b-tbench-lb-41.txt` 完整）；已裁掉浏览器翻译插件图标 |
| `23-methodology-tbench-harness.png` | 方法 PDF 第 2 页，23:00 | pdf + cropbox | 含 “Methodology” 与三条 Agentic Coding 说明 |
| `24-modelcard-mcc-argon.png` | 模型卡 PDF 第 9 页 | pdf + cropbox | “Frontier Safety Assessment” 段 |
| `37-ag-docs-model-matrix.png` | antigravity.google/docs/models，23:02 渲染 | cropbox | **深色页面**（按系统外观渲染）；5 行（Carbon、Argon、Hydrogen、Orchestrator 两行） |
| `38-ag-docs-orchestrator-table.png` | 同上 | cropbox | **深色页面**；Orchestrator 小节与两张表、下段说明 |

**编号假设**：B 组区间 13–24 共 12 个号，已用满 13–24；多出的两张按 1111 期先例顺延到 37、38（避开 C 组 25–36）。已逐张打开目视确认：13、14、15、16、17、18、19、20、21、22、23、24、37、38 均文字清楚、范围正确，无账号信息。

没有截图的候选：配额定义段（plans 文档，若要图可补 39）、HN 评论、官方 Terminal-Bench 4.0 与 AA 图（原 GIF 在 `b-gimg-*.gif`，1920 px 宽）、模型卡 “adapted from Carbon” 段（PDF 第 2 页）、InfoWorld/TNW 等报道标题。

## 备注

- `b-ag-docs-changelog.txt` 565,778 字节（< 1 MB），其 `.html` 1,337,149 字节（> 1 MB，HTML 原件不入库）。其余入库文本均小于 1 MB。
- opencli 会话 `b-tb`、`b-tb2`、`b-pr` 为后台页面，未登录；用完未手动关闭。
- 泄露自查：见回报。
