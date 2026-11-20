# 1119 期 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区 UTC+8），取自 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json` 与命令输出。文件前缀 `a-`。未登录任何账户，未接受任何 Cookie 横幅，未提交表单，未发帖、点赞、评论、关注。`.headers` 由 `barking fetch` 去掉抓取地点与标识。截图用 `barking shot render`（headless Chrome，匿名，`BARKING_CHROME` 指向 Chrome 应用二进制，700 CSS px 宽、DPR 2，即 1400 px 宽）；本组没有使用 opencli 浏览器。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 22:11:57–22:11:59 | https://architectlabs.com/blog/frostbite ；https://architectlabs.com/ ；https://news.ycombinator.com/item?id=50572503 ；https://hn.algolia.com/api/v1/items/50572503 | `barking fetch`（`a-fetch-list-1.tsv`） | 全部 ok：`a-blog.*`、`a-home.*`、`a-hn.*`、`a-hnapi.html`；打开 txt 确认是正文（博客 169 行，含全部 Fig. 说明、Results、Notes） |
| 22:12 | https://architectlabs.com/_next/static/media/perf-per-watt-heldout.26hhprjnn8kyi.png ；…/workload-suite.0d2l3bzeh4gp9.png ；…/energy-breakdown.1vhkm_oxxzwwv.png | curl（`--max-time 60`） | 均 200：`a-fig-perf-per-watt-heldout.png`（1600×1968）、`a-fig-workload-suite.png`（1600×1401）、`a-fig-energy-breakdown.png`（1600×686）；用于读 Fig. 11/3/12 的标注值，不入库 |
| 22:12:36 | HN 搜索 “Frostbite”“Architect Labs”“interaction nets”；条目 50572503；Reddit 搜索 “Architect Labs Frostbite CPU”“Frostbite interaction nets CPU AI” | `barking heat docs/2611/1119 a` | HN 成功：`a-heat.json`、`a-hn-50572503.json`（294 分 / 200 评，含 193 条嵌套评论）；**Reddit 两次均失败**：重定向到 old.reddit.com 登录页，响应非 JSON，不登录、不重试 |
| 22:12–22:14 | Exa 检索（`web_search_exa`，共 6 次：英文二手稿、中文二手稿、Redwood、夸大标题、中文媒体域名限定） | Exa MCP | 只作线索与二手稿清单；被引页面均另行 `barking fetch` 存档后再核对原文（Exa 的发布时间字段只作线索，已标“未核”） |
| 22:13:42–22:13:46 | https://www.aroged.com/…crushed-conventional-cpus-in-virtual-tests/ ；https://www.notebookcheck.net/AMD-and-Intel-move-over-…1455225.0.html ；https://www.pic720.com/article/6b0022218e9f091ec34c6f0a ；https://www.ic.work/article/frostbite-4-59x-performance-per-watt-and-the-limits-of-pre-s ；https://explainx.ai/blog/architect-labs-frostbite-… ；https://xenospectrum.com/en/frostbite-cpu-design-energy-efficiency/ ；https://i10x.ai/news/architect-labs-frostbite-ai-designed-cpu ；https://architectlabs.com/blog/redwood ；https://architectlabs.com/blog ；HN 页 50562422、50557080 | `barking fetch`（`a-fetch-list-2.tsv`） | 除 xenospectrum 外全部 ok（`a-m-aroged`、`a-m-notebookcheck`、`a-m-pic720`、`a-m-icwork`、`a-m-explainx`、`a-m-i10x`、`a-redwood`、`a-blogidx`、`a-hn-50562422`、`a-hn-50557080`）；**xenospectrum：blocked，HTTP 403（Cloudflare 质询）**，未绕过，只有 Exa 摘录，未存档、未作为证据引用 |

被拒或失败：xenospectrum（403）、Reddit（登录重定向，两次）。未取得 Architect Labs 的 X / LinkedIn 帖（未尝试，不影响博客取证）。

## 截图

均匿名，已逐张打开确认文字清楚、范围正确。页面均为浅色底（Aroged 为浅灰底）。所有图 1400 px 宽（700 CSS px、DPR 2）。博客整页先用 `render` 以 `--height 17000` 渲出 `blog2`（1400×34000），再用 `cropbox`/`stack` 裁切，裁切边界落在文字行或图表框之间。

| 文件 | 来源页 | 工具与说明 |
|---|---|---|
| `01-frostbite-top-abstract.png`（1400×1870） | 博客 | cropbox；页顶标题卡（含 “Research November 18, 2026”）+ Abstract 全三段（含 4.59x 头条句与 “autonomously iterated” 句）；页顶深蓝封面图只露出一条边 |
| `02-tcpu-quad-comparison.png`（1400×1318） | 博客 | stack 三段：Results 首段（7 nm、800/500 MHz、0.792 mm²、TCPU 0.895 mm²、†）；“We benchmarked on a wide set…” 与 “For a fair comparison…” 两段（8.77x、4.59x、6.02x、3.83x）；Notes 脚注 “For legal reasons, we cannot disclose the exact CPU.” |
| `03-fig11-perf-per-watt.png`（1400×1730） | 博客 Fig. 11 | cropbox；整张图（标题、坐标轴、25 行全部标注值、虚线几何均值与图例）+ 图注，截全 |
| `04-fig3-workload-split.png`（1400×1440） | 博客 Fig. 3 | cropbox；含 “Later, we added five more practical workloads…” 一句、整张分组图与图注 |
| `05-ai-autonomy-claims.png`（1400×808） | 博客 | stack 三段：Abstract 第三段 “Our AI systems autonomously iterated…”；“Frostbite was designed with zero architectural priors.”；“This behavior was never specified by any human.” |
| `06-human-choices-and-lean.png`（1400×2034） | 博客 | stack 两段：“To begin architectural exploration, we had to establish the set of workloads…” 全段；“Formal contracts and system modeling” 节正文（ASL 自研、Lean、“800k+ lines of Lean”），节标题被裁掉 |
| `07-presilicon-alignment.png`（1400×820） | 博客 “RTL and alignment” | cropbox；三段（“almost 250%”、“fully autonomous pipeline… within 5%”、“still pre-silicon power numbers…”），节标题被裁掉 |
| `08c-next-plans-spec-parsec.png`（1400×1904） | 博客 | stack 两段：Fig. 12 之后的段落（含 “In the coming months… SPEC and PARSEC”）；“What comes next” 节全文与四条计划（含 “Release the RTL and ISA for Frostbite.”） |
| `09-fig12-energy-20-workloads.png`（1400×700） | 博客 Fig. 12 | cropbox；整张图含 “Mean over 20 workloads” 与时钟占比说明、图注 |
| `10-aroged-crushed-headline.png`（1400×1230） | Aroged | cropbox；面包屑、标题、日期、首段（“completely crushed the best processor of our time”）；页内图片未加载出的占位（俄文 alt 文字）；底边露出下一张图的上沿 |
| `11-notebookcheck-move-over-headline.png`（1400×960） | Notebookcheck | cropbox；标题、摘要、署名与日期、正文第一段（含 “pre-silicon simulations”）；已避开顶栏与广告位 |
| `12-hn-thread-title-points.png`（1400×105） | Hacker News | cropbox；只裁帖子标题与分数行（301 分 / 207 评，22:17 北京渲染读数），不含评论；含楼主 HN 用户名（公开站内名） |

## 待删除文件（本组留下的试验/废弃文件，请 Claude 收尾时一次删除）

- `/Users/develata/gitclone/AI-Barking/docs/2611/1119/images/08-next-plans-spec-parsec.png`（第一条 PARSEC 行被切掉半行，已由 08c 取代）
- `/Users/develata/gitclone/AI-Barking/docs/2611/1119/images/08b-next-plans-spec-parsec.png`（误生成的单段裁切，已由 08c 取代）
- `/Users/develata/gitclone/AI-Barking/docs/2611/1119/images/26-lb-table.png` 与 `27-elo-chart.png` 不是本组文件（C 组），勿删。

另外 `sources/a-shots-log.jsonl` 里有对应 08、08b 的记录，保留即可。
