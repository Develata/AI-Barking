# 抓取日志：D 组（1024 期）

取证人：claude-haiku-5-5。时间一律北京时间（UTC+8）。本机时钟为 EDT，文件 mtime 所示时刻 +12 小时即北京时间。

逐条记录用的工具：`barking fetch`（curl，见 `d-fetch-log.jsonl`）、`curl`（一次性请求）、WebSearch（检索摘要，不当作原文）、`barking heat`（HN Algolia 与 Reddit）。

## 1. 批量存档：`barking fetch docs/2610/1024 d --jobs 6`

- 开始时刻：2026-10-25 23:25:05（北京）；用时 2 秒；结果 ok 8、blocked 0、failed 0。
- 日志：`sources/d-fetch-log.jsonl`（同时写入 `d-*.headers`、`.html`、`.txt`）。

| name | URL | 结果 | 字节 | 备注 |
|---|---|---|---|---|
| d1-hf-model | https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash | 200 ok | 325,193 | txt 690 行 |
| d1-minimax-blog | https://www.minimax.io/blog/minimax-m31f | 200 ok | 80,666 | txt 175 行 |
| d1-aa-model | https://artificialanalysis.ai/models/minimax-m3-1-flash | 200 ok | 4,467,942 | html 超过 1 MB，只存本地；Date 头 13:13 GMT（北京 21:13，疑为缓存） |
| d2-baltimore | https://www.baltimoresun.com/2026/10/24/johns-hopkins-bayview-medical-center-data-breach/ | 200 ok | 166,280 | 付费墙前导段，正文不完整 |
| d3-hr-elvis | https://www.hollywoodreporter.com/music/music-industry-news/anthropic-elvis-presley-lawsuit-ai-1236739617/ | 200 ok | 605,664 | txt 844 行 |
| d4-cerebras | https://www.cerebras.ai/blog/how-we-deploy-trillion-parameter-frontier-models-on-cerebras | 200 ok | 296,664 | 并发数与 tokens/s 在图中，未入文字 |
| d5-gov-uk | https://www.gov.uk/government/news/introducing-the-ai-biosecurity-lab | 200 ok | 75,597 | |

注：`barking fetch` 的 `d1-*` 等名字会生成 `d-d1-*` 文件，即组前缀与名称叠加，属正常。

## 2. 一次性 curl

| 时刻（北京，约） | URL | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 23:25:47 | https://huggingface.co/api/models/MiniMaxAI/MiniMax-M3.1-Flash/commits/main | curl `--max-time 60` | 200，1 条提交“Init MiniMaxAI/MiniMax-M3.1-Flash”（2026-10-23T03:37:20Z） | `d-d1-hf-commits.json`、`.headers` |
| 约 23:26 | https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash/raw/main/LICENSE | curl `--max-time 60` | 200，MiniMax Community License 原文 | `d-d1-hf-license.txt` |
| 约 23:3x | https://www.courtlistener.com/api/rest/v4/search/?q=%22Anthropic%22%20Presley&type=r&order_by=dateFiled%20desc | curl `--max-time 60` | 200，14 条，最新 2026-06-17（Shakespeare v. Anthropic），无 Presley 案 | `d-d3-courtlistener-search.json`（索引可能滞后，未作“不存在”结论） |
| 约 23:3x | https://www.billboard.com/pro/elvis-presley-estate-sues-suno-anthropic/ | curl `--max-time 60` | 200，页面日期 10/24/2026，“filed Friday (Oct. 23)”，274 首，Authentic Brands Group 一方 | `d-d3-billboard.html`（只存本地，不入库） |

## 3. WebSearch 检索（摘要，非原文）

以下检索只用于定位线索，结论不以搜索摘要为一手来源；检索时刻约 23:2x–23:3x（北京）。

| 查询 | 结果 |
|---|---|
| “Johns Hopkins Bayview data breach 3,182 patients unapproved public AI tool notification” | 未返回 Bayview 事件，只有 2024 MOVEit 事件 |
| “Bayview Medical Center employee uploaded patient health information unapproved AI tool breach notice” | 未返回 Bayview 事件；返回 Loma Linda、安大略等其他事件 |
| “Anthropic lawsuit Elvis Presley music publishers 274 songs Claude training complaint” | 未返回 Elvis 诉讼；返回 Sony Music Publishing 与 Warner Chappell 于 8 月底起诉 Anthropic（搜索摘要） |
| “Reservoir Media sues Suno Anthropic October 2026 lyrics 853 songs statutory damages” | 未返回 Reservoir 报道；返回 Round Hill Music 8 月起诉 Suno 与 Anthropic 的案件（搜索摘要） |
| “SemiAnalysis Claude Max 20x A/B test 5-hour usage limit weekly limit 19%” | 未返回 SemiAnalysis 原帖与 19% 数据；返回 Anthropic 帮助中心页与二手汇总 |
| “Lydia Hallie Boris Cherny Claude Max usage limit A/B test SemiAnalysis response” | 未返回 A/B 测试原帖；Hallie 的说法只有二手汇总，未核 |

## 4. 热度：`barking heat docs/2610/1024 d`（北京 10/25 23:27:40）

- HN 查询结果（写入 `d-heat.json`）：
  - “MiniMax M3.1 Flash”：13 条命中，取前 13（主帖 517 分 / 220 评，10/20 00:39 BJ）。
  - “Johns Hopkins Bayview”：0 条。
  - “Elvis Presley Anthropic”：1 条（6 分 / 0 评，10/25 17:15 BJ）。
  - “AI Biosecurity Lab”：1 条（4 分 / 0 评，10/24 16:44 BJ）。
  - “Cerebras Kimi K3”、“Round Hill Anthropic”：0 条。
- Reddit 查询失败 3 次（“MiniMax M3.1 Flash”“AI Biosecurity Lab”“Anthropic Elvis lawsuit”）：`old.reddit.com/search.json` 被重定向到登录页（HTTP 200 text/html，约 335 KB），非 JSON。已记入 errors，未绕过。

## 5. 热度：`barking heat docs/2610/1024 d-extra`（北京 10/25 23:28:11）

- 因 `d-heat.json` 已存在，`barking heat` 拒绝覆盖，故改用组前缀 `d-extra`，结果写入 `d-extra-heat.json`。
- HN 查询：
  - “trillion parameter frontier models on Cerebras”：2 条命中（264 分 / 88 评，10/25 09:33 BJ；2 分 / 0 评，10/23 06:41 BJ）。
  - “Cerebras Kimi K3”、“Johns Hopkins AI patient data”：0 条。
  - “Anthropic Elvis Presley”：1 条（6 分 / 0 评）。
  - “Apollo zero payoff”：1 条（2 分 / 1 评，10/25 21:56 BJ；属 B 组，仅记录，不纳入 D 组）。
- Reddit 未运行。
- 失败：0 个请求失败。

## 5b. 第二轮补充（北京 10/25 23:2x–23:33）

| 时刻（北京，约） | 工具与 URL | 结果 | 存档 |
|---|---|---|---|
| 23:25 前后 | `curl -D` 取 HF 提交记录（commits/main） | 200，一条 Init 提交（2026-10-23T03:37:20Z）。原始 `-D` 输出含代理与请求标识，已按四行格式重写 `d-d1-hf-commits.headers`，原始内容未保留 | `d-d1-hf-commits.json`、`.headers` |
| 约 23:2x | WebSearch ×4（WMAR Bayview；Elvis 被告；SemiAnalysis/Hallie 第二次；Reservoir） | 见下 | 摘要，不作一手来源 |
| 23:32:06 | `barking fetch docs/2610/1024 d` 清单 `d3-mbw-reservoir` | 200 ok，91,973 字节，366 行 | `d-d3-mbw-reservoir.html` / `.txt` / `.headers` |
| 23:32:06 | `barking fetch` 清单 `d2-hopkins-dataattack`（https://www.hopkinsmedicine.org/data-attack） | 403 blocked，无 txt | `d-d2-hopkins-dataattack.html`（5,492 字节，403 页） |
| 23:32:17 | `barking fetch` 清单 `d3-reservoir-anthropic-complaint`（MBW 托管的诉状 PDF） | 200 ok，260,367 字节，PDF 1,620 行 txt | `d-d3-reservoir-anthropic-complaint.pdf` / `.txt` / `.headers` |
| 约 23:33 | `barking html2txt` Billboard HTML → txt | 884 行、27,463 字节；“filed Friday (Oct. 23)”、274 首、Authentic Brands 等句已核 | `d-d3-billboard.txt` |

WebSearch 第二轮结果：
- 第 1 次（WMAR 与 Bayview AI）：未返回 Bayview 事件。
- 第 2 次（Elvis 估计被告 Suno/Anthropic）：未返回 2026 年 10 月的 Elvis 案，只有 2016、2025 年旧案，属无关结果。
- 第 3 次（Reservoir，限定 digitalmusicnews.com 与 musicbusinessworldwide.com）：返回 MBW 报道，已由 fetch 取得正文与诉状。
- 第 4 次（hopkinsmedicine.org Bayview 通知）：只返回 MOVEit 旧通知与 AI scribe 页面，无 Bayview AI 事件。

## 6. 失败与未取到

- Baltimore Sun：付费墙，只见前导段。
- Bayview 医院通知、HHS 或州检察长通报：未找到（搜索两次，无结果）。
- WMAR 报道：未找到 URL，未取。
- Reservoir 诉讼：已取得 MBW 报道与诉状 PDF（见 5b）。Anthropic 对诉状的回应：未找到。
- Hopkins 官网 `data-attack` 页：403，未取到正文。
- Elvis 诉状原文与法院立案记录：未取到（CourtListener 索引无记录，Billboard 与 THR 只给二手转述）。
- SemiAnalysis 原帖与 Anthropic 员工原帖：未找到（D6）。
- 凤凰网转述页 https://tech.ifeng.com/c/8xSNopjD8lu：未打开（D6 不展开）。
- Cerebras 并发数与 tokens/s 图表：在图中，未入文字，未截图（D 组不截图）。

## 7. 本组文件与待处理

- 本组文件均在 `docs/2610/1024/sources/` 下，文件名以 `d-` 开头，外加 `evidence-d.md`、`capture-log-d.md`。
- 按协调者规则，取证过程中未调用 `rm`。本组没有生成需要删除的废弃文件；`d-extra-heat.json` 为有效记录（第二次热度查询）。
- 体积：超过 1 MB 的文件有 `d-d1-aa-model.html`（4,467,942 字节）与 `d-d3-billboard.html`（1,031,744 字节，按十进制 1 MB 计超过）。两者都是 HTML 原件，只存本地，不入库；`d-d3-reservoir-anthropic-complaint.pdf`（260 KB）同样只存本地。
- 待删除文件：无。本组没有需要删除的试验或废弃文件。`d-d1-hf-commits.headers` 为重写后的保留文件。
