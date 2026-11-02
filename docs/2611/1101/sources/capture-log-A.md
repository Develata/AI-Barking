# 1101 期 A 组抓取日志（SemiAnalysis GDPx）

执行方 Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间；本机为美东（EST），北京 = 本机 + 13 小时。开始 11-02 23:49，交回约 11-03 00:25。

## 网页与文本存档

| 北京时间 | URL | 工具 | 结果 / 失败原因 |
|---|---|---|---|
| 23:49 | https://newsletter.semianalysis.com/p/gdpx-measuring-frontier-model-capabilities | `barking fetch`（curl） | 200，`a-article.html|.txt|.headers`。匿名响应含全文，页尾另有 “This post is for paid subscribers” 条；未登录、未使用 Cookie |
| 23:49 | https://gdpx.semianalysis.com/ | `barking fetch` | 200，`a-board.html|.txt`；SSR 只含默认 13 模型的 Score 视图，需浏览器渲染才能看全 |
| 23:52–00:15 | https://gdpx.semianalysis.com/ | `opencli browser --session a-shot`（本机 Chrome，匿名读取） | 成功。用页面内 JS 点击切换 Metric / Domain / Models / harness，读 `#leaderboard` 的 innerText：`a-board-all20-score.txt`、`a-board-all20-score-harness.txt`、`a-board-all20-score-harness-excl3.txt`、`a-board-all20-partial-excl3-harness.txt`、11 个域的 `a-board-domain-<域>-score-harness.txt`、金融/法律/网安/生物/对齐的 `a-board-domain-<域>-partial-harness.txt`；矩阵与均值 `a-domain-matrix.tsv`（由前述文本计算） |
| 23:58 | https://gdpx.semianalysis.com/domains/cybersecurity | opencli | 成功，仅读域页文本（无榜单，已确认），未存档为文件 |
| 00:03 | https://gdpx.semianalysis.com/domains 、/rollouts | opencli | 成功，`a-site-domains.txt`、`a-site-rollouts.txt` |
| 00:06 | HN 条目 50333000、50333552；HN 搜索 “GDPx SemiAnalysis”“gdpx.semianalysis.com” | `barking heat` | 成功，`a-heat.json`、`a-hn-50333000.json`、`a-hn-50333552.json`；3 分 0 评 / 1 分 0 评 |
| 00:06 | Reddit 搜索 “GDPx SemiAnalysis benchmark” | `barking heat`；`opencli reddit search` | **失败**：old.reddit.com 返回登录页（LOGIN_WALL，403）；同一来源失败两次，不再重试，Reddit 热度未取得 |
| 00:08 | X 帖 2117075900575477855（SemiAnalysis 首帖）、2117075908301393972（5/7）、2117113121102045347（Pichai）、2117092550842683671（Dylan Patel）、2117150366332801536（0GuoJiH）、2117142297934757916（Mollick）、2117120675064238490（Fleet 员工）、2117189669037441181（distributionat） | `curl https://api.fxtwitter.com/status/<id>` | 成功，`a-fx-<id>.json`（均为公开作者数据；未含抓取者信息） |
| 00:08 | X 搜索 “GDPx”“Pichai GDPx”等、SemiAnalysis_ 与 sundarpichai 的近期帖 | `opencli twitter search / tweets / thread`（本机已登录的浏览器会话，只读） | 成功，仅用来**定位**帖子 ID，未存这些搜索结果文件（结果里含登录态下的排序与互动数，不入库）；全部引用以 fxtwitter 公开 JSON 为准 |
| 00:10 | https://af.net/realtime/semianalysis-launches-gdpx-ai-benchmark-as-claude-opus-5-5-leads-with-12-success-rate/ ；https://www.yingzheng.com/article/gdpx-benchmark-frontier-models-zero-finance-legal ；https://cryptobriefing.com/semianalysis-gdpx-ai-benchmark-frontier-models/ ；https://explainx.ai/blog/semianalysis-gdpx-benchmark-opus-5-5-12-percent-agents-fail-2026 ；https://www.unite.ai/semianalysis-launches-gdpx-benchmark-spanning-11-agentic-domains/ | `barking fetch` | 全部 200 ok，`a-media-afnet|yingzheng|cryptobriefing|explainx|unite.*`（explainx、unite 的稿子写对了 sandbagged 与部分分，未列入夸大实例） |
| 00:12 | https://www.theblockbeats.info/flash/374959 ；https://aiera.com.cn/asi-baodian.html | `barking fetch` | 200 ok，`a-media-blockbeats.*`、`a-media-aiera-baodian.*`（后者是聚合页，只作线索） |
| 00:14 | BlockBeats 同一 URL | `barking shot render`（headless） | **失败**：403 “您的访问可能对网站造成威胁，已被拦截”；opencli 本机 Chrome 打开也报 “unexpectedly closed the connection”。不绕过，**无截图**，存档文本来自 curl 那一次 200 |
| 00:15 | 报道检索 | `opencli google search`、`opencli brave search`（匿名搜索页），WebSearch | WebSearch 无该事件的索引（返回无关结果）；opencli 搜索有结果，仅用于找 URL |

## 截图（`docs/2611/1101/images/`，全部已逐张打开目视确认）

| 文件 | 来源 / 方法 | 备注 |
|---|---|---|
| 01-board-score-all20.png | gdpx 榜单，Score、All domains、20 of 20、harness 显示；opencli 本机 Chrome，documentElement zoom 2、`--width 1800 --full-page`（版式 900 CSS px），`barking shot cropbox` 裁出榜单卡 | 深色底页面；右侧滚动条与浏览器翻译图标已裁掉；1720×3100 |
| 02-board-partial-score.png | 同上，Metric = Partial score，含两段页脚说明 | 1720×3200 |
| 03-board-excl-safety-domains.png | 同上，Domain = “Exclude domains affected by safety classifiers”（8 domains） | 含站点说明 “Leaves out Biology, Cybersecurity and Alignment…”；1720×3100 |
| 04-board-cybersecurity.png | Domain = Cybersecurity | 1720×3260 |
| 05-board-fin-score.png | Domain = Finance，Score | 1720×3270 |
| 06-board-fin-partial.png | Domain = Finance，Partial | 1720×3365 |
| 07-art-12pct-chart.png | 博文页：标题/署名 + “even the best model scores just 12%” + 配图；`barking shot render`（headless，760 CSS px，DPR 2）+ `stack` | 匿名页，右上为 Subscribe / Sign in；页面标 “14 HOURS AGO · PAID”（相对时间，绝对时间见 evidence）；1520×2260 |
| 08-art-sandbagged-excl-chart.png | 博文页：sandbagged 段 + 剔除三域配图 | 1520×1740 |
| 09-media-yingzheng-title-0pt.png | 赢政天下稿件标题与前三段；`barking shot render` + `cropbox` | 1520×1190；浅色底 |
| 10-art-qa-gaming-harness-passat1.png | 博文页：QA、gaming、first try、时限、native harness、pass@1、等权；opencli 本机 Chrome，`body.style.zoom=2`，视口截图 1520 宽；为截图隐藏了页面左侧目录条与固定按钮（只改显示，未改正文） | 1500×2260 |
| 11-board-legal.png | Domain = Legal，Score | 1660×3270（右边框被裁掉一点，以避开浏览器翻译图标） |
| 12-art-finance-binary-partial.png | 博文页：Finance 一节两段（binary / partial 说明） | 同 10 的方法；1500×520 |

未使用的编号：无（01–12 全部使用）。X 帖子（Pichai、SemiAnalysis 首帖、0GuoJiH 的夸大帖）**没有截图**：编号已满，且 X 嵌入卡片折叠长文；需要时请从 `a-fx-*.json` 取文字，或另行用 X 嵌入页截卡片。

## 中间与废弃文件（请 Claude 收尾时一次删除）

- `docs/2611/1101/images/05-board-finance-score.png`（裁错：缺卡片上半）
- `docs/2611/1101/images/06-board-finance-partial.png`（同上）
- `docs/2611/1101/images/07-art-headline-12pct.png`（切在社交按钮行中间）
- `docs/2611/1101/images/11-board-legal-score.png`（裁错）

## 自写脚本（站点特定，放 sources/）

`a-gdpx-dump.sh`（读榜单文本）、`a-gdpx-domains.py`（页面内 JS 点击切换）、`a-gdpx-shot.py`（切换视图并截图）、`a-art-shot.sh`（长文段落截图）。

## 假设与未做

- 假设 `barking heat` 输出的 HN 时刻为北京时间（该命令头部标 +08:00）。
- 未用浏览器翻译插件，截图里的翻译图标已裁掉；浏览器可能登着维护者账号，但 gdpx 站与 SemiAnalysis 博文页均为匿名可见，截图不含账号信息（07、08 图里的 “Sign in / Subscribe” 是匿名状态）。
- 不用镜像站；paywall 仅记录页面状态，未登录、未订阅。
- 没做：博文 2/7–4/7 线程帖存档；中文社区（微博、知乎、微信）搜索；Radar 图（Compare 面板）截图。

## 补记

- `a-gdpx-shot.py` 里导入 `a-gdpx-domains.py` 的路径是相对仓库根目录的（`docs/2611/1101/sources/a-gdpx-domains.py`），需在仓库根目录运行；`a-gdpx-dump.sh`、`a-art-shot.sh` 不依赖当前目录。
