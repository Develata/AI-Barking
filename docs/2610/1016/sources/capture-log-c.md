# 1016 期 C 组抓取日志

执行：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8）；本机时区美东，北京时间 = 本机 + 12 小时。工具：`barking fetch`（批量存档，结果记 `c-fetch-log.jsonl`）、`barking shot`（headless Chrome 渲染与裁切，记 `c-shots-log.jsonl`；按任务补充说明未等 opencli）、`barking heat`（`c-heat.json`）、curl（HN Algolia 公开 API、fxtwitter 公开 JSON、arXiv 未用）、WebSearch 与 Exa 搜索（只作线索，不存结果页）。脚本类辅助（Pillow 切片看图、静默行吸附）放在会话临时目录，不入库。

## 存档（网页/文本）

| 时刻 | URL | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 20:49:11 | https://www.anthropic.com/research/employment-at-anthropic | barking fetch | 200 ok | `c-anth-report.html/.txt/.headers` |
| 20:49:11 | https://www.mercor.com/research/labor-index/benchmark-map/ | barking fetch | 200 ok | `c-mercor-map.*` |
| 20:49:11 | https://www.mercor.com/research/labor-index/ | barking fetch | 200 ok，跳转后正文与上一条相同（同一页，仅 nonce 不同），重复，仅备查 | `c-mercor-index.*` |
| 20:50:13 | https://www-cdn.anthropic.com/72628f1863c53432195db83279252dfd2cb53158/Employment%20at%20Anthropic.pdf | barking fetch | 200 ok，2.0 MB，转文本 682 行；无附录 | `c-anth-pdf.pdf/.txt/.headers`（PDF 本地，不入库） |
| 约 20:54 | https://www.mercor.com/research/labor-index/benchmark-map/benchmarks/ | barking fetch | 200 ok | `c-mercor-benchmarks.*` |
| 约 20:54 | https://www.mercor.com/research/labor-index/benchmark-map/methodology/ | barking fetch | 200 ok | `c-mercor-method.*` |
| 21:07:15 | https://www.anthropic.com/research/expectations-for-the-future | barking fetch | 200 ok | `c-anth-summary.*` |
| 21:07:15 | https://www.indiatoday.in/jobs/story/anthropic-is-hiring-fewer-juniors-says-ai-now-does-6-times-more-work-than-humans-tchc-3019422-2026-10-17 | barking fetch | 200 ok | `c-news-indiatoday.*` |
| 21:07:15 | https://www.newsbytesapp.com/news/business/anthropic-says-ai-agents-now-work-6-times-human-hours/tldr | barking fetch | **blocked（403）**；未绕过，仅用检索摘录 | `c-news-newsbytes.html/.headers`（492 字节，无正文） |
| 21:07:15 | https://aiweekly.co/alerts/anthropic-junior-technical-hire-share-fell-from-27-to-9 | barking fetch | 200 ok | `c-news-aiweekly.*` |
| 21:07:15 | https://the420.in/anthropic-cuts-junior-hiring-ai-agents-work-hours/ | barking fetch | 200 ok（`.txt` 多为分享链接，未作引用） | `c-news-the420.*` |
| 21:07:15 | https://explainx.ai/blog/anthropic-employment-junior-hiring-agents-2026 | barking fetch | 200 ok | `c-news-explainx.*` |
| 21:07:15 | https://runtimewire.com/article/mercor-benchmark-map-us-economy | barking fetch | 200 ok（正文被截短，摘要可读） | `c-news-runtimewire.*` |
| 21:08:22 | https://www.ithome.com/1/014/195.htm | barking fetch | 200 ok | `c-news-ithome.*` |
| 21:08:22 | https://finance.sina.com.cn/stock/t/2026-10-17/doc-inivnrax8238598.shtml | barking fetch | 200 ok | `c-news-sina.*` |
| 21:08:22 | https://tech.ifeng.com/c/8xHwSo3IDfh | barking fetch | 200 ok | `c-news-ifeng.*` |
| 约 21:10 | https://api.fxtwitter.com/status/2110557079027265653（Mercor 官方帖） | curl（--max-time 60） | ok，公开 JSON，只含官方账号与帖子，无抓取者信息 | `c-mercor-tweet.json` |
| 约 21:06 | https://hn.algolia.com/api/v1/items/50121179（Mercor 帖评论树） | curl | ok（272 KB） | `c-hn-mercor-comments.json` |
| 约 21:06 | https://hn.algolia.com/api/v1/items/50127555（Anthropic 报告帖评论树） | curl | ok | `c-hn-anth-comments.json` |
| 约 20:50 | HN Algolia 搜索（employment-at-anthropic、Anthropic hiring 等） | curl | 报告帖仅 1 个：50127555 | 无 |

`c-list.tsv`、`c-list2.tsv`、`c-list3.tsv` 为 fetch 清单（c-list2 的 `news-newsbytes` 对应上面 403 条；c-list 里 `mercor-index` 是重复项）。

## 截图

| 时刻 | 页面 | 工具 | 结果 | 文件 |
|---|---|---|---|---|
| 20:52:07 | https://www.anthropic.com/research/employment-at-anthropic | `barking shot render`（headless Chrome，700 CSS px，DPR 2，高 11000 → 1400×22000），页面浅色底 | 成功；用 Pillow 切片目视找到各图位置 | 中间图 `anth`（临时目录，不入库） |
| 20:56:15 | 同上 | `barking shot cropbox` | 第一版 `25-fig1-junior-share.png`（5370–6490，首行贴边）、`26-tech-share-absolute-growth.png`（末行 “…to 90 in” 截断段落）、`27-fig2-agent-hours.png`（标题贴边）。我逐张看图后发现，用普通 `rm` 加精确路径删除这三个我自己刚生成的试验文件并重裁：25（5345–6510）、26（6525–7235）、27（9050–10580）。`c-shots-log.jsonl` 里仍保留前三条已删除版本的记录 | 定稿 `25-fig1-junior-share.png`、`26-tech-share-absolute-growth.png`、`27-fig2-agent-hours.png` |
| 约 20:57 | 同页 | `barking shot cropbox` | 成功 | `28-fig3-productivity.png`、`29-adoption-not-random.png`、`30-fig4-adoption-effects.png`、`31-idle-waiting.png` |
| 约 21:00 | https://www.mercor.com/research/labor-index/benchmark-map/ | `barking shot render`（高 6000 → 1400×12000） | 底部固定的 Cookie 横幅压住了 “Limitations” 末段；**未点击任何 Cookie 选项**，改用更高视口重渲 | 中间图 `mercor`（临时目录，只用于看图） |
| 约 21:03 | 同上 | `barking shot render`（高 8000 → 1400×16000） | 成功，横幅落在页面下方、不在裁切范围内 | 中间图 `mercor2` |
| 约 21:03 | 同上 | `barking shot cropbox` | 成功 | `32-mercor-top-stats.png`、`33-mercor-90pct-working-hours.png`、`34-mercor-limitations.png` |

| 约 21:12 | https://www.anthropic.com/research/expectations-for-the-future | `barking shot render`（高 3500 → 1400×7000）后 `stack`（标题与日期段 + 正文段，略去中间的页首装饰图） | 成功 | `35-anthropic-summary-six-times-agents.png` |

所有截图均浅色底（Anthropic 为暖白底，Mercor 为白底）；已逐张打开目视确认文字清楚、范围正确（25–35 均打开看过）。图中无登录栏、浏览器插件图标或抓取者信息。Mercor 页是交互式地图，截图是默认状态（“Benchmark Index”、“5 prompts”）。

## 热度与社区

| 时刻 | 请求 | 结果 |
|---|---|---|
| 21:05:36 | `barking heat … --hn "employment-at-anthropic" --hn "How employment is changing at Anthropic" --hn "mercor.com/research/labor-index" --hn "benchmark map of the US economy" --reddit "How employment is changing at Anthropic" --reddit "mercor benchmark map"` | HN：Anthropic 报告帖 50127555 8 分/3 评（10-17 11:13 北京提交）；Mercor 帖 50121179 536 分/454 评（10-17 01:08 北京提交）；Reddit 两次搜索被登录墙拦下（`old.reddit.com/login`），记入 `c-heat.json` errors |
| 约 21:06 | 对两个 HN 帖评论树做关键词扫描 | 只摘与方法或口径相关的评论，见 evidence-c.md；HN 评论分数不公开，未能按得分取 |

## 检索（线索，不存档结果页）

- WebSearch（standard/extended）：中英文关于报告与 Mercor 的检索各数次；Exa 检索两次（英/中，起始日期 2026-10-15/16）。线索来自：blockchain.news、aiweekly.co、fourweekmba.com、explainx.ai、priag.com、newsbytesapp.com、indiatoday.in、IT之家/新浪/凤凰网、runtimewire.com。
- **firecrawl_search 返回 402（额度耗尽），未使用 firecrawl。**
- 没有登录任何站点，没有点击 Cookie 横幅，没有发帖或互动，没有读取浏览器 Cookie，没有使用镜像或代理域名，没有 web.archive.org 快照。opencli 未启用（按任务说明）。

## 自查

- 对 `docs/2610/1016/` 的 `c-` 文件与图片 grep 过本机用户名、路径、邮箱等，无命中。
- `c-hn-mercor-comments.json` 含 HN 公开用户名（评论作者），属公开信息；fxtwitter JSON 只含 Mercor 官方账号。
- 单个文件 >1 MB：仅 `c-anth-pdf.pdf`（2.0 MB，本地不入库）；`c-*.html` 里最大 `c-news-indiatoday.html` 732 KB。HTML、PDF 原件按规则不入库。
