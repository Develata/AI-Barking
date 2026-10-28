# 1027 期 C 组取证：AA 智能指数 v5 换版（Kimi 55→46）【跟进1023】

执行：Claude Sonnet 5.5（claude-sonnet-5-5）。取证窗口：2026-10-28 20:29–21:10 北京时间（本机 EDT，北京时间 = 本机 + 12 h；fetch 日志里已是 +08:00）。所有 AA 页面均匿名、未登录。

页面写“Updated”“v5.0”等版本标签的，已在表中注明；未写版本的不推断。AA 分数一律写成 “指数版本 / 测试名 / 档位”。

## 〇、核心结论（先给 C2/C3）

1. **v5 公开宣布时刻**：AA 官方 X 帖 2026-10-28T02:11:22Z = **北京 2026-10-28 10:11:22**（美东 10-27 22:11:22）。AA 文章与 Changelog 条目只写日期 “October 27, 2026 / 27 Oct 2026”，官方未给时刻。HN 最早帖 2026-10-28T03:06:22Z（北京 11:06:22）。
2. **本号 1023 期的 “55 分、第四”**：取自 **v4.3.2**。AA 存档 `docs/2610/1023/sources/a-aa-*.txt` 抓取于北京 2026-10-24 19:52–20:18；`a-aa-model.txt` 写 “Artificial Analysis Intelligence Index v4.3.2”，AA 文章 10-23 的图标题写 “Intelligence Index v4.3.2 incorporates 10 evaluations”。1023 期成品提交（`faf13be`）为北京 2026-10-25 00:43，早于 v5 宣布约 3.4 天。**当时引用的就是当时的现行版，不是引了旧版；现在它已过期。**
3. **本号 1026 期的 “Terra 最高档 48、上一代 42、Sol 中档 48”**：同样取自 **v4.3.2**。AA 存档 `docs/2610/1026/sources/b-aa-*.txt` 抓取于北京 2026-10-27 22:52–23:01；`b-aa-article.txt` 写 “Breakdown of the individual evaluations in the Artificial Analysis Intelligence Index v4.3.2”，`b-aa-method-intel.txt` 写 “Intelligence Index v4.3.2”。1026 期成品提交（`192127e`）为北京 2026-10-28 01:56，早于 v5 宣布约 8 h 15 min。**同样是“当时的现行版”。**
4. **旧数字在 v5 下的现值**（AA 现在页面，抓取于北京 10-28 20:34–20:39）：Terra 最高档 48→51；Sol 中档 48→46；GPT-5.6 Terra（“上一代”）42→**N/A**（模型页写 “This model is deprecated”，Intelligence 卡片 N/A）。**具体数字全部失效，但相对关系换档位后仍近似成立**：1026 的“Sol 中档与 Terra 最高档同为 48 分”在 v5 下不再同分（Sol medium 46 < Terra max 51），但 **Sol high 50（$0.59）与 Terra max 51（$0.96）只差 1 分、成本约六成**，“同分档更便宜的 Sol 档”这一论点换到 high 档基本还成立。成本同变：Terra（max）每任务 $0.40→$0.96，Sol（medium）$0.21→$0.36，Sol（high）$0.32→$0.59。不宜写成“1026 的结论翻转”。
5. **AA 自己对跨版本比较的说法**：v5 公告与方法页未写“新旧分数不可比”；公告自己写 “compared with a 3-point gap in v4.3”。但 AA Progress Index v1 方法页写 “Each version of the Intelligence Index is therefore measured on a different set of evaluations, and scores across versions are not intended for long-run comparison.”（原句，见 C1-6）。
6. 同一批模型 v4.3.2→v5 的变化（见第四节）并非全体下降：Astra +4、Sol(max) +3、Terra(max) +3，而 Kimi −9、DeepSeek −13、GLM −13、Ling −15。

---

## 一、逐条取证（按 templates/codex-evidence.md 第 4 条格式）

级别：AA 自身页面 = L3；Moonshot/inclusionAI/Cognition 自家页面 = L1；HN 评论、媒体 = L4–L6，仅线索。

### C1 v5 公告

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1-1 v5 是“最大的一次升级”，60% 题集私有 | L3 | https://artificialanalysis.ai/articles/artificial-analysis-intelligence-index-v5 （存档 `c-aa-v5.html/.txt`） | “v5 is our biggest upgrade since v4 launched in January. With v5, 60% of the Index is now based on evaluations with private test sets or held-out answers.” | `25-aa-v5-intro-lists.png` | 页面日期 “October 27, 2026”；“private test sets or held-out answers”是 AA 的合并口径，不等于“60% 的题完全不公开”（见 C1-4） | 已找到 |
| C1-2 新增 2、升级 3、保留 4、删除 3 | L3 | 同上 | “New to the Intelligence Index: ➤ FrontierCode 2.0 … ➤ Terminal-Bench-Science: 70 agentic scientific research tasks”；“Upgraded: ➤ HLE-Diamond (replacing Humanity's Last Exam) … ➤ AA-Omniscience v1.1 … ➤ CritPt v2: a private, 40-question hard subset”；“Staying in the Index: AA-Briefcase, AutomationBench-AA, GDP.pdf and Terminal-Bench 4.0”；“Removed from the Index: GDPval-AA v2.1, SciCode, and AA-LCR v1.1” | `25-aa-v5-intro-lists.png` | Changelog 写 “GDPval-AA v2”“AA-LCR”（无小数版本），公告写 “GDPval-AA v2.1”“AA-LCR v1.1”；方法页写 “GDPval-AA v2.1”。同一页内版本写法不一，不影响结论 | 已找到 |
| C1-3 四大类各 25%；各项权重 | L3 | 同上；方法页 https://artificialanalysis.ai/methodology/intelligence-benchmarking （存档 `c-aa-method.txt`，页首 “Artificial Analysis Intelligence Index v5.0”） | “Knowledge Work (25%): AA-Briefcase (15%), AutomationBench-AA (5%), and GDP.pdf (5%)”；“Coding (25%): FrontierCode 2.0 (15%) and Terminal-Bench 4.0 (10%)”；“Science (25%): Terminal-Bench-Science (15%) and CritPt v2 hard subset (10%)”；“General (25%): AA-Omniscience v1.1 15% (10% accuracy, 5% non-hallucination) and HLE-Diamond (10%)” | `26-aa-v5-weights-results.png`、`29-aa-v5-method-table.png` | v4.3.2 为 Agents 30% / Coding 20% / Scientific Reasoning 20% / General 30%（`docs/2610/1026/sources/b-aa-method-intel.txt`：“Agents (30%), Coding (20%), Scientific Reasoning (20%) and General (30%)”）。**扫描说“四大类各 25%”对 v5 成立；旧版不是** | 已找到 |
| C1-4 私有题占比 60% 的算法 | L3 | 方法页 `c-aa-method.html`（表头 “Tool Usage / Private”） | 表中 Private 列为 ✓ 的项：AA-Briefcase 15%、AutomationBench-AA 5%、FrontierCode 2.0 15%、CritPt v2 (Hard Subset) 10%、AA-Omniscience v1.1 15%，合计 **60%**；Private 列为 ✗：GDP.pdf 5%、Terminal-Bench 4.0 10%、Terminal-Bench-Science 15%、HLE-Diamond 10%（合计 40%） | `29-aa-v5-method-table.png` | 合计为推算（按页面 Private 列加总），与公告“60%”吻合。v4.3.2 同法加总：AA-Briefcase 15 + AutomationBench 5 + AA-Omniscience 15 + CritPt 10 = **45%**（`b-aa-method-intel.txt` 表；推算）。所以“首次过半私有”成立（45→60），但不是“0→60”。方法页表格在窄屏下横向滚动，Private 列常被截掉 | 已找到 |
| C1-5 为什么改（官方理由） | L3 | 公告 | “The Artificial Analysis Intelligence Index is our composite measure of model capabilities across tasks reflecting real-world usage today.”；“v5 leaves significant room for progress across all nine evaluations, and no single model leads them all.”；FrontierCode：“the tasks stay private to avoid contamination”；AA-Omniscience：“saturated and public set questions removed”。没有单独写“为什么要换”的一段 | 25、26 | AA 未写“因为旧版被刷分”或类似理由；换版理由只能从各项改动描述读出（防污染、去饱和、更难）。社区对动机的猜测见“可能的吠点” | 部分支持（理由为推读，非官方明言） |
| C1-6 是否明说新旧分数不可比 | L3 | 公告：无此句；Progress Index 方法页 https://artificialanalysis.ai/methodology/progress-index （存档 `c-aa-progress.txt` 第 42 行） | 公告原句相反地做了跨版本比较：“The best, Kimi K3.1 Pro (weights not yet released), scores 46, 12 points behind the leader, compared with a 3-point gap in v4.3.” Progress Index 页：“Each version of the Intelligence Index is therefore measured on a different set of evaluations, and scores across versions are not intended for long-run comparison.” | `28-aa-v5-openweights-gap.png`（公告句）；Progress Index 句未截 | “not intended for long-run comparison” 在 Progress Index 方法页，不在 v5 公告页。“3-point gap in v4.3”写的是 v4.3，1023 期存档的版本是 v4.3.2（同一大版本的补丁号） | 部分支持（v5 公告未明说不可比；AA 在姊妹产品页明说跨版本不适合长期比较） |
| C1-7 任务输出 token 中位数 35k→59k | L3 | 公告 | “Every model we measured generates more output tokens per Intelligence Index task than in v4.3, with the median rising by two thirds, from 35k to 59k.” | 未截（文字存档足够） | 原句 “median rising by two thirds” | 已找到 |
| C1-8 档位差距加大 | L3 | 公告 | “Effort settings matter more in v5 than in v4.3. Claude Opus 5.5 scores 35 at low effort and 58 at max, at 10x the cost per Intelligence Index task ($0.98 vs $10.04).” | 未截 | — | 已找到 |
| C1-9 AA 发帖时刻 | L1（AA 官方账号） | https://x.com/ArtificialAnlys/status/2115265133467635877 （fxtwitter JSON 存 `c-fx-2115265133467635877.json`） | 首帖 “Announcing Artificial Analysis Intelligence Index v5, featuring 2 new benchmarks and 3 upgrades. For the first time, the majority of the Index is built on private datasets” | 未截 | `created_at` Wed Oct 28 02:11:22 +0000 2026 = 北京 10-28 10:11:22；4280 赞 / 943k 浏览（20:4x 取） | 已找到 |

### C2 上线时刻

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C2-1 Changelog 条目 | L3 | https://artificialanalysis.ai/changelog （存档 `c-aa-changelog.html/.txt`） | “27 Oct 2026 … 🔔 New article published … Announcing Artificial Analysis Intelligence Index v5”；其下 “Artificial Analysis Intelligence Index methodology / Artificial Analysis Intelligence Index v5.0” | `30-aa-changelog-v5-terra.png` | 只有日期，**官方未给时刻**；日期应为美国日期（推断：X 帖是 UTC 10-28 02:11，即美东 10-27 22:11） | 部分支持（只有日期） |
| C2-2 页面 meta | L3 | `c-aa-v5.html` | `article:published_time` content="2026-10-27"；JSON-LD `"datePublished":"2026-10-27"`、`"dateModified":"2026-10-27"` | — | 仅日期 | 部分支持（只有日期） |
| C2-3 AA 官方 X 帖 | L1 | 见 C1-9 | `created_at` 2026-10-28T02:11:22Z | — | **北京 10-28 10:11:22**；转述站 AlphaSignal JSON-LD `datePublished` 2026-10-28T02:11:22.000Z 与之一致（L5，仅旁证，`c-alphasignal-v5.*`） | 已找到 |
| C2-4 HN | L6 | https://news.ycombinator.com/item?id=50268868 （`c-hn-50268868.json`） | 标题 “Artificial Analysis Intelligence Index v5” | — | 提交 2026-10-28T03:06:22Z = 北京 11:06:22；69 分 / 72 评（`barking heat`，取于北京 20:33）。另一帖 id=50269651 “Artificial Analysis Intelligence Index v5.0”（指向方法页）：北京 13:29（05:29Z），38 分 / 5 评 | 已找到 |
| C2-5 上线的**最早**时间 | — | — | 无任何一手来源给出“网站数据切到 v5”的时刻 | — | 上界：不晚于 X 帖 10-28 10:11:22 北京；下界：1026 存档（北京 10-27 22:52–23:01，页面仍为 v4.3.2）。两者之间的 11 小时内站点何时切换，**未找到一手来源**。第三方 Cognition X 帖（FrontierCode 2.0 发布）2026-10-27T17:24:17Z = 北京 10-28 01:24:17，AA 公告写 “Launched earlier today” | 未找到一手来源（精确切换时刻） |

### C3 本号旧数字的版本（读 1023 / 1026 存档，只读）

| 文件（相对仓库根） | 抓取时刻（北京）/ HTTP Date | 存档里写的版本 | 内容 | 与 v5 宣布（北京 10-28 10:11:22）比 |
|---|---|---|---|---|
| `docs/2610/1023/sources/a-aa-article.txt`（AA 文章 10-23） | `a-fetch-log.jsonl`：2026-10-24 19:52:49；响应头 Date 2026-10-24 09:05:03 GMT | 图标题 “Intelligence Index v4.3.2 incorporates 10 evaluations”（文章图，读自 `31` 截图；txt 为图中文字之外的正文） | “Kimi K3.1 Pro launches at 4th on the Artificial Analysis Intelligence Index scoring 55”；“the gap between the leading open weights and proprietary models on the Intelligence Index will narrow from 12 points to 3.” | 早 3 天 14 小时 |
| `docs/2610/1023/sources/a-aa-model.txt` | 2026-10-24 19:52:52；Date 09:05:01 GMT | “Artificial Analysis Intelligence Index v4.3.2 incorporates 10 evaluations: …” | Kimi K3.1 Pro 模型页 | 同上 |
| `docs/2610/1023/sources/a-aa-leaderboard.txt` | 2026-10-24 20:18:11；Date 09:05:01 GMT | 榜单表头无版本号；表内 Kimi K3.1 Pro **55**（行 5）、Opus 5.5 (max with fallback) 58、Astra max 53、Sol max 52、DeepSeek V4.1 Flash 1017 max 45、GLM-5.3 max 45、Ling 3.1 Flash 1019 44、Haiku 5.5 max 43 | v4.3.2 榜单 | 同上 |
| `docs/2610/1023/sources/a-aa-evals.txt`、`a-aa-tbench.txt` | 2026-10-24 20:18:11 / 20:18:15 | “Artificial Analysis Intelligence Index v4.3.2” | — | 同上 |
| `docs/2610/1026/sources/b-aa-article.txt`（AA 文章 10-26 Terra） | `b-fetch-log.jsonl`：2026-10-27 22:52:39；Date 2026-10-27 14:05:15 GMT | “Breakdown of the individual evaluations in the Artificial Analysis Intelligence Index v4.3.2” | “At max effort, GPT-6.1 Terra gains 6 points in the Artificial Analysis Intelligence Index over GPT-5.6 Terra, at less than a third of the cost per task ($0.40 vs $1.40).” | 早 11 小时 19 分 |
| `docs/2610/1026/sources/b-aa-model.txt`（Terra max 模型页） | 2026-10-27 22:52:40；Date 13:29:11 GMT | 页内 “Intelligence Index v4.3.2” | “#25 / 221”，分数 **48**，$0.40，130M | 同上 |
| `docs/2610/1026/sources/b-aa-m-sol-medium.txt`、`b-aa-m-sol-high.txt`、`b-aa-m-61sol.txt` | 2026-10-27 23:01:46 / 22:59:26；Date 13:29 GMT | v4.3.2 | Sol medium **48**（$0.21）、Sol high 50（$0.32）、Sol max 52（$0.72） | 同上 |
| `docs/2610/1026/sources/b-aa-m-56terra*.txt` | 2026-10-27 22:59:23–23:01:43 | v4.3.2 | GPT-5.6 Terra：max **42**（$1.40）、xhigh 38、high 34、medium 30、low 27、non-reasoning 21 | 同上 |
| `docs/2610/1026/sources/b-aa-method-intel.txt` | 2026-10-27 22:52:41；Date 13:29:21 GMT | 页首 “Artificial Analysis Intelligence Index v4.3.2” | 10 项评测、Agents 30% … | 同上 |

注：
- HTTP `Date` 响应头比 fetch-log 的北京时刻早 1.5–3.5 小时（AA 站点有 CDN 缓存，Date 为缓存页生成时间）。两种时刻都早于 v5 宣布；结论不受影响。
- 本号成品提交时刻（`git log`，EDT→北京）：1023 `faf13be` 2026-10-24T12:43:07-04:00 = 北京 10-25 00:43:07；1026 `192127e` 2026-10-27T13:56:12-04:00 = 北京 10-28 01:56:12。提交时刻不等于 Develata 发出时刻，但后者不可能更晚于提交之后太久（AGENTS.md：“已发布”后提交）；两者均早于 v5 宣布。
- **判断（推断，非独立核实 Develata 发布时刻）：两期正文引用 AA 时，AA 的现行版本都是 v4.3.2，不属于“当时就引了旧版”。口径过期发生在发布之后。**

### C4 v5 当前分数（AA 榜单与模型页，抓取于北京 2026-10-28 20:29–20:39）

排行榜 `https://artificialanalysis.ai/leaderboards/models`（`c-aa-leaderboard.html/.txt`）与公告图（`27-aa-v5-score-chart.png`，图标题 “Artificial Analysis Intelligence Index v5.0 incorporates 9 evaluations”）。每个模型一行一档。括号内为 AA 的档位标签，原样抄录。

| 模型（档位） | v5 分数 | 来源 | 本号存档里 v4.3.2 同模型同档分数 | 差 |
|---|---|---|---|---|
| Claude Opus 5.5 (max with fallback) | 58 | 榜单 / 模型页 `c-v5-m-opus` | 58（`1023 a-aa-leaderboard`；文章 “max with fallback, 58”） | 0 |
| Claude Opus 5.5 (xhigh with fallback) | 57 | 榜单 | 未存档（1023 榜单第 3 行 56） | +1（按 1023 榜单行） |
| GPT-6 Astra (max) | 57 | 榜单 / 模型页 `c-v5-m-astra` | 53 | +4 |
| GPT-6 Astra (xhigh) / (high) | 55 / 55 | 榜单 | — | — |
| GPT-6.1 Sol (max) | 55 | 榜单 / `c-v5-m-61sol`（$1.27，64M token） | 52（$0.72，67M） | +3 |
| GPT-6.1 Sol (xhigh) | 52（$0.76） | 榜单 | — | — |
| GPT-6.1 Sol (high) | 50（$0.59，25M） | `c-v5-m-sol-high` | 50（$0.32，25M） | 0 |
| **GPT-6.1 Sol (medium)** | **46**（$0.36，15M） | `c-v5-m-sol-medium` | **48**（$0.21，15M） | **−2** |
| GPT-6.1 Sol (low) | 35（$0.16） | 榜单 | — | — |
| Claude Sonnet 5.5 (max with fallback) | 54 | 榜单 | 56 | −2 |
| Muse Wonder Full (max) | 50 | 榜单 | 56 | −6 |
| Gemini 4 Argon (high) | 50 | 榜单 / `c-v5-m-argon` | 53 | −3 |
| Claude Fable 5.1 (max with fallback) | 49（$13.04） | 榜单 | 1023 榜单第 7 行 53 | −4 |
| **Kimi K3.1 Pro** | **46**（$3.68，110M token） | 榜单 / `c-aa-m-kimi` | **55**（$2.11；文章 “$1.98 per Task”，55K token/任务） | **−9** |
| Claude Haiku 5.5 (max) | 36（$0.41） | 榜单 / `c-v5-m-haiku` | 43（$0.21） | −7 |
| DeepSeek V4.1 Flash 1017 (max) | 32（$0.60） | 榜单 / `c-v5-m-deepseek` | 45（$0.26） | −13 |
| GLM-5.3 (max) | 32（$5.01） | 榜单 / `c-v5-m-glm` | 45（$2.01） | −13 |
| Ling 3.1 Flash 1019 | 29（$3.19，240M token） | 榜单 / `c-aa-m-ling` | 44（$1.49）（1023 榜单；Ling 官方卡也写 44） | −15 |
| **GPT-6.1 Terra (max)** | **51**（$0.96，99M） | `c-v5-m-terra-max`（榜单同） | **48**（$0.40，130M；`1026 b-aa-model.txt`） | **+3** |
| GPT-6.1 Terra (xhigh) | 45（$0.40，35M） | `c-v5-m-terra-xhigh` | 45（$0.19，41M） | 0 |
| GPT-6.1 Terra (high) | 42（$0.26，21M） | `c-v5-m-terra-high` | 44（$0.13，23M） | −2 |
| GPT-6.1 Terra (medium) | 38（$0.17，14M） | `c-v5-m-terra-medium` | 41（$0.10，14M） | −3 |
| GPT-6.1 Terra (low) | 29（$0.07，7.2M） | `c-v5-m-terra-low` | 36（$0.05，7.9M） | −7 |
| GPT-6.1 Terra (non-reasoning) | 30（$0.23，10M） | `c-v5-m-terra-nr` | 33（$0.11，8.3M） | −3 |
| **GPT-5.6 Terra (max)（上一代）** | **N/A**（Intelligence 卡片 N/A；页面 “This model is deprecated. We only continue performance benchmarking for the default 10k input token workload.”） | `c-v5-m-56terra` 及各档 | **42**（$1.40，120M） | v5 无分 |
| GPT-6 Luna (max) | 28 | 榜单 | `1026 b-aa-article`：38（“10 points above GPT-6 Luna” 推算） | −10（推算） |

- 公告原文对应句：“Claude Opus 5.5 (max) leads at 58, ahead of GPT-6 Astra (max, 57) and GPT-6.1 Sol (max, 55): Anthropic and OpenAI hold the top five models. Gemini 4 Argon (high) is next at 50”；“Cost per task … GPT-6.1 Terra (max) is the cheapest at $0.96 per Intelligence Index task and Claude Fable 5.1 (max) the most expensive at $13.04”（`26-aa-v5-weights-results.png`）。
- 扫描称 “Terra 51（旧 48）”：成立（max 档）。“Haiku 5.5 36”“DeepSeek/GLM 32”“Gemini 4 Argon (high) 50”：成立。
- 注意 “Anthropic and OpenAI hold the top five models” 在榜单行（含档位）上成立；按每个模型取一条，公告图 27 中第 6 位是 Gemini 4 Argon（50）、与 Muse Wonder Full 并列。
- 1026 期正文用词核对：“开最高推理档的 Terra 得48分”——v4.3.2 的 max 档；“上一代最高档42分”——GPT-5.6 Terra（max）v4.3.2；“GPT-6.1-Sol 开中档也得48分”——v4.3.2 的 medium 档。三者的 v5 对应值见上表（51 / N/A / 46）。
- 页面把 v5 数据直接盖在旧 URL 上，Changelog 的历史条目同样被改写：`c-aa-changelog.txt` 的 26 Oct 条目现在写 “GPT-6.1 Terra (Max) Intelligence Index: 51”“(Low) 29”“(Medium) 38”“(High) 42”“(Xhigh) 45”“(Non-reasoning) 30”——而 1026 期取证（v4.3.2）该条目对应 48/36/41/44/45/33（见 `1026 b-aa-m-*.txt` 的模型页分数；Changelog 当时的文本未存档，故“Changelog 被改写”为推断：值与 v5 一致、与 1026 存档的 v4.3.2 不一致）。 `30-aa-changelog-v5-terra.png`。

### C5 Kimi K3.1 Pro

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C5-1 v5 分数 46；“权重尚未发布” | L3 | 公告 `c-aa-v5.txt`；模型页 https://artificialanalysis.ai/models/kimi-k3-1-pro （`c-aa-m-kimi`） | 公告：“The best, Kimi K3.1 Pro (weights not yet released), scores 46, 12 points behind the leader”；模型页：“Kimi K3.1 Pro scores 46 on the Artificial Analysis Intelligence Index, placing it above average among comparable models (median: 41).” | `28`（公告句与图）、`32-aa-kimi-model-v5.png` | 指数 v5.0，档位：模型页为单一推理版（“This page shows the reasoning version of this model.”）；榜单标签 “Kimi K3.1 Pro”（无档位括号）。模型页卡片 “#20 / 51”，是同类 51 个条目中的位次，不是全球名次 | 已找到 |
| C5-2 排名 | L3 | 同上 | v4.3.2（1023 文章）“launches at 4th”；v5 公告图（`27`）按每模型一条，Kimi 为第 9 条（58, 57, 55, 54, 51, 50, 50, 49, 46）；榜单行（含各档位）第 21 行；模型页 “#20 / 51” | `27`、`32` | “第四”只对 v4.3.2 成立；v5 公告没有写 Kimi 的名次 | 已找到（v5 下不再是第四） |
| C5-3 是否标注权重未发布 | L3 | 公告；模型页 | 公告：“(weights not yet released)”在公告全文出现 4 次；模型页头部标 “Proprietary model”，页面 FAQ：“No, Kimi K3.1 Pro is proprietary. The model weights are not publicly available.”；公告图 `28` 按 “Proprietary / Open Weights” 图例，Kimi 的柱是黑色（Proprietary） | `28`、`32` | **AA 内部不一致**：公告文字把 Kimi 算作“top open weights model”并据此算 12 分差距，但同一公告的图和模型页把它归为 Proprietary | 已找到 |
| C5-4 Kimi 官方何时放权重 | L1 | https://www.kimi.ai/blog/kimi-k3-1-pro （`1023/sources/a-blog.txt` 第 90、148 行） | “The full model weights will be released by Nov 4.”；“Full model weights will be released by Nov 4, 2026.” | — | 1023 存档，北京 2026-10-24 19:52 取 | 已找到 |
| C5-5 HF 现查 | L1 | https://huggingface.co/api/models?author=moonshotai&sort=lastModified&direction=-1&limit=15 （`c-hf-moonshot-api.json`，北京 2026-10-28 20:31） | 列表最新为 `moonshotai/Kimi-K3`（lastModified 2026-09-02）；无 Kimi-K3.1 条目。直连 `…/api/models/moonshotai/Kimi-K3.1-Pro`、`Kimi-K3.1`、`Kimi-K3.1-Pro-Base` 均 HTTP 401（HF 对不存在或非公开仓库一律 401） | — | **截至北京 10-28 20:31，HF 上无 K3.1 Pro 权重**（匿名 API 可见范围内）；未验证 Kimi 官网是否另有下载渠道 | 已找到（尚未放） |

### C6 “开放权重最高分”“开源与闭源差距”

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| C6-1 AA 自己写的差距 | L3 | 公告 | “The open weights gap widens from 3 points to 12: Kimi K3.1 Pro (weights not yet released) is the top open weights model at 46.”；“Open weights models fall further behind in v5. The best, Kimi K3.1 Pro (weights not yet released), scores 46, 12 points behind the leader, compared with a 3-point gap in v4.3.” | `26`、`28` | **12 分 = 58（Opus 5.5 max）− 46（Kimi，尚未放权重）**；“3-point gap in v4.3” 也是按 Kimi 55 与 Opus 58 算的（1023 AA 文章写“narrow from 12 points to 3”，条件是权重发布后）。v5 的 12 把未发布权重的 Kimi 算作开放权重；v4.3.2 的 12 用的是已放权重的 MiMo-V2.6-Pro（见 C6-5） | 已找到 |
| C6-2 已放权重的最高分 | L3 | 公告 | “Among released weights, DeepSeek V4.1 Flash 1017 (max) and GLM-5.3 (max) lead at 32 - 26 points behind the leader and below Claude Haiku 5.5 (max, 36)” | `26`、`28` | 已放权重：32，落后 26 分 | 已找到 |
| C6-3 有开放权重榜 | L3 | 榜单页 / 公告图 `28` | 图标题 “Artificial Analysis Intelligence Index by Open Weights / Proprietary”，图例 “Proprietary / Open Weights / Open Weights (Commercial Use Restricted)” | `28` | 榜单页有 “Open Weights / Proprietary” 筛选 | 已找到 |
| C6-4 差距在哪些评测最大 | L3 | 公告 | “The gap is widest on HLE-Diamond and Terminal-Bench-Science, where Kimi K3.1 Pro scores 37% and 34%, compared with 68% for Claude Opus 5.5 (max) and 63% for GPT-6 Astra (max).” | `28` | 反例：Kimi K3.1 Pro 仍是 Terminal-Bench 4.0 最高（67%）（公告 “Kimi K3.1 Pro: Terminal-Bench 4.0 (67%)”） | 已找到 |
| C6-5 “差距从 3 分变 12 分”的措辞 | L3 | 公告；1023 存档榜单 `1023/sources/a-aa-leaderboard.txt` 第 5527–5533 行（FAQ 区） | 公告：“The open weights gap widens from 3 points to 12”。1023 期 AA 文章（v4.3.2）：“Once the weights for K3.1 Pro are released it will be the highest-scoring open weights model and the gap between the leading open weights and proprietary models on the Intelligence Index will narrow from 12 points to 3.”；同日榜单 FAQ：“MiMo-V2.6-Pro is the highest-ranked open weights model with an Intelligence Index score of 46.”、“The top open weights models … 1. MiMo-V2.6-Pro (46), 2. DeepSeek V4.1 Flash 1017 (Max) (45), 3. GLM-5.3 (Max) (45).” | `31`（文章句与图） | **两个“12”是同一个数字、不同参照**：v4.3.2 的 12 = Opus 5.5 58 − MiMo-V2.6-Pro 46（已放权重）；v5 的 12 = Opus 5.5 58 − Kimi K3.1 Pro 46（**尚未放权重**）。v4.3.2 的 “3” = 58 − Kimi 55（权重放出后才成立）。v5 下 MiMo-V2.6-Pro 为 30（榜单），已放权重最高为 DeepSeek/GLM 32，与 Opus 差 26。以上差值为按页面数字推算，AA 未写算式 | 已找到 |

### C7 Ling-3.1-flash-1019

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C7-1 官方卡引用的 AA 版本与分数 | L1 | https://huggingface.co/inclusionAI/Ling-3.1-flash-1019 （`c-hf-ling.html/.txt`；API `c-hf-ling-api.json`） | “Overall, it scores 44 on the Artificial Analysis Intelligence Index v4.3.2, up from 41 for its predecessor.” | `33-hf-ling-card-v432.png`（含图中 “Intelligence Index v4.3.2 incorporates 10 evaluations”） | 版本为 **v4.3.2**，不是 v5；卡片的图也是 v4.3.2 图。仓库创建 2026-10-27T05:46:51Z（北京 13:46），README 最后修改 06:51:42Z（北京 14:51），权重上传 10:45–10:46Z（北京 18:45–18:46）；均早于 v5 宣布（北京 10-28 10:11） | 已找到 |
| C7-2 AA 模型页 v5 分数 | L3 | https://artificialanalysis.ai/models/ling-3-1-flash-1019 （`c-aa-m-ling`） | 页面：“Ling 3.1 Flash 1019 scores 29 on the Artificial Analysis Intelligence Index, placing it well above average among comparable models (median: 23).”；“#4 / 16”；头部 “Open weights model” | `34-aa-ling-model-v5.png` | 指数 v5.0；页面有 “Updated” 标；成本 $3.19、输出 240M token（同类 #16/16）；Output speed N/A。同一页面写 “has a 1M tokens context window” | 已找到 |
| C7-3 “41→44”变成 v5 的“29” | L3 | 1023 存档榜单 `1023/sources/a-aa-leaderboard.txt`；v5 榜单 | v4.3.2（10-24 取）：Ling 3.1 Flash 1019 = 44，Ling 3.1 Flash = 41；v5 榜单：Ling 3.1 Flash 1019 = 29，**Ling 3.1 Flash 一行为 “--”（无分）** | `27`（公告图含 “Ling 3.1 Flash 1019 29”） | 前代 41 在 v5 没有分数，无法给出 v5 下的“上一代→本代”提升 | 已找到 |
| C7-4 许可、规模 | L1 | HF 卡 | “License: mit”（页面元数据）；“with 560B total parameters, 25B active parameters per token, and a 1M-token context window” | `33` | 规模句在卡片正文 | 已找到 |
| C7-5 config.json 的 max_position_embeddings 与 rope_scaling | L1 | https://huggingface.co/inclusionAI/Ling-3.1-flash-1019/raw/main/config.json （`c-hf-ling-config.html`，HTTP 200，6636 字节，取于北京 2026-10-28 20:29） | `"max_position_embeddings": 262144,` … `"rope_scaling": null,` `"rope_theta": 6000000,` | `35-hf-ling-config-maxpos.png`（两段拼接：上段含 max_position_embeddings，下段含 rope_scaling；中间省略的行是 num_nextn_predict_layers 等无关项） | **扫描说法（262144）读数属实**。262144 = 256K，与卡片 “1M-token context window” 数字不一致。前代 `inclusionAI/Ling-3.1-flash` 的 config.json 同样 `max_position_embeddings: 262144`、`rope_scaling: null`（curl 现取，北京 20:3x，未存档）。**这不能直接推出“1M 不成立”**：该模型是 hybrid-linear（63 KDA + 9 Gated MLA，卡片表），config 字段名与有效上下文的关系未核；卡片未写 262144 与 1M 之间如何衔接（如 YaRN 配置、推理引擎参数）。只能写“config 与卡片数字不一致，官方未解释” | 已找到（读数）；“1M 是否可用”未找到一手来源 |
| C7-6 “官方新闻稿”称 41→44 | — | 未找到 | 本次只核对了 HF 模型卡；蚂蚁/inclusionAI 的新闻稿未检索 | — | 扫描称新闻稿同口径，未核 | 未找到一手来源 |

### C8 FrontierCode 2.0

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C8-1 Cognition 博文分数 1.1 vs 2.0 | L1 | https://cognition.ai/blog/frontier-code-2.0 （`c-cognition-fc2.html/.txt`；图表为客户端渲染，txt 含数据表） | 页面“FrontierCode 1.1 vs. FrontierCode 2.0”：Claude Opus 5.5 54.6%→45.5%；GPT-6 Astra 53.3%→42.8%；Grok 4.7 47.6%→28.6%；Gemini 4 Argon 47.5%→26.3%；Muse Wonder 1.0 47.0%→24.3%；SWE-2 50.0%→23.9%；DeepSeek V4.1 Flash 43.9%→19.3%；Kimi K3 44.2%→17.7%。脚注：“Each model is shown at its best reasoning effort.” | `36-cognition-fc11-vs-20.png`（1× 像素；1.1 标签页） | 页面日期 “2026.10.27”；Cognition X 帖 2026-10-27T17:24:17Z = 北京 10-28 01:24:17（`c-fx-2115132488775524793.json`）。表里是 **Kimi K3**，不是 K3.1 Pro；表没有 Terra / Sol / Haiku | 已找到 |
| C8-2 是否声明与 1.1 不可比 | L1 | 同上 | 未找到“不可比”字样；博文只写 “Compared with benchmarks like DeepSWE v1.1 and FrontierCode 1.1, FrontierCode 2.0 separates models more clearly across tiers of capability. The benchmark is far from saturated: the top-performing model, Opus 5.5, scores only 45%.”；并写任务更大 “reference solutions modify 67% more files and 89% more lines of code on average, in codebases that are 25% larger” 且 “we loosened our scope criteria” | 未截 | 页面自述 2.0 提高了标准（“Higher mergeability standards”）、放宽了 scope 判定，两者方向相反；没有“分数不可比”的明文 | 未找到一手来源（不可比声明） |
| C8-3 任务数 | L1/L3 | Cognition 博文：未写数字；AA https://artificialanalysis.ai/evaluations/frontiercode-2-0 （`c-aa-fc2-eval`） | AA：“a private agentic coding benchmark of 132 tasks in real open-source repositories”；“We run all 132 tasks of FrontierCode 2.0. The tasks are private to avoid contamination.” | — | Cognition 博文文本中未出现 “132”（grep） | 部分支持（任务数仅 AA 写） |
| C8-4 AA 复跑 | L3 | 公告；评测页 | 公告：“We run every task three times in Stirrup, our agentic harness. Claude Opus 5.5 (max) leads at 46%, followed by GPT-6 Astra (max), GPT-6.1 Terra (max) and Claude Sonnet 5.5 (max), all at 42%. DeepSeek V4.1 Flash 1017 (max) is the highest-scoring open weights model at 28%, 18 points behind the leader.”；评测页：“Claude Opus 5.5 (Max, Default Fallback) scores the highest on FrontierCode 2.0 with a score of 46.4%, followed by Claude Opus 5.5 (Xhigh, Default Fallback) with a score of 43.8% and GPT-6 Astra (Max) with a score of 42.4%”；“We use GPT-6 Luna (medium) as the LLM judge.” | 无（公告 FrontierCode 段落与图未截；文字存档 `c-aa-v5.txt`） | AA 与 Cognition 的 Opus 5.5 数不同：AA 46.4%（Stirrup，3 次均值，500 轮上限），Cognition 45.5%（“best reasoning effort”，未写 harness）。AA 用 OpenAI 的 GPT-6 Luna 做 LLM 评审（HN 有人质疑） | 已找到 |
| C8-5 Kimi 在 FrontierCode 2.0 的 AA 分 | L3 | AA 公告图（FrontierCode 2.0: Score） | 柱状图按横轴次序读出 Kimi K3.1 Pro 25.5%（夹在 Gemini 4 Argon 25.7% 与 GPT-6 Luna 25.2% 之间） | 无（该图未单独截） | 读自图，不是正文文字，**推算** | 部分支持 |

### C9 中文媒体 / 研报仍引旧口径

只找到“发布于 v5 宣布之前、至今仍挂在网上的 55 分稿”，**没有找到发布于 v5 宣布之后、仍写 55 分的稿**。国泰海通研报的转载页发布时刻均早于 v5。界面新闻、腾讯新闻 10-28 当日写 55 分的稿：**未找到**（Brave 搜索只返回 10-24 至 10-27 的稿；`opencli brave search`，搜索摘要不当来源）。

| 稿件 | 发布方 / 时刻（北京） | 原站 URL | 原句逐字 | 存档 | 状态 |
|---|---|---|---|---|---|
| 国泰海通：Kimi K3.1 Pro发布 看好开源模型生态繁荣带动AI应用加速落地（智通财经稿，同花顺转载） | 同花顺页面时间 2026-10-28 06:48:11，来源标注 智通财经；网易订阅转载 06:49:05 | http://news.10jqka.com.cn/20261028/c681159862.shtml ；https://www.163.com/dy/article/LAAD7FNI05198UNI.html | “Artificial Analysis智能指数中，K3.1Pro以55分位列全球第四，较K3提升11分，距第一名Claude Opus5.5仅3分；权重开放后将成为全球评分最高的开放权重模型，开源与闭源前沿的差距由12分收窄至3分；完整模型权重将于11月4日前发布。” | `c-gtht-10jqka.*`、`c-gtht-163.*` | 已找到（页面时间 06:48:11 **早于 AA 官方 X 帖（10:11:22）3 小时 23 分**；但站点何时切到 v5 未知（C2-5：窗口为北京 10-27 23:01–10-28 10:11），所以**不能推出“早于站点换版”**，该稿写作时 AA 网站显示的是否已是 v5 无法判定；它不是“v5 官宣之后仍写旧口径”；至北京 10-28 20:3x 页面未更正） |
| 国泰海通同稿，英为财情（cn.investing.com） | — | https://cn.investing.com/news/stock-market-news/article-3631311 | 被 Cloudflare 质询拦截（403） | 无 | 失败（blocked，已记 `c-fetch-log.jsonl`） |
| Kimi K3.1 Pro更新背后：AI大模型能力演进趋势解读 | 什么值得买（AIGC 文章），2026-10-27 18:46:23（JSON-LD） | https://post.smzdm.com/p/a26x9zkp/ | “在第三方评测机构 Artificial Analysis 的智能指数上，这款新模型拿到 55 分、位列全球第四，与榜首 Claude Opus 5.5 的分差只剩 3 分。” | `c-smzdm-kimi-trend.*` | 已找到（发布于 v5 之前；L5） |
| 实测 Kimi K3.1 Pro，国模终于继续卷旗舰 | 腾讯新闻（页面注“2026-10-24 10:04发布于广东爱范儿官方账号”，即爱范儿稿） | https://news.qq.com/rain/a/20261024A04UAK00 | “在 Artificial Analysis 「智能指数」直接跃升到全球第四，开源第一。”（页面原文型号写作 “K31. Pro”） | `c-qq-kimi.*` | 已找到（发布于 v5 之前；L4/L5） |
| Kimi K3.1 Pro综合评分超GPT 6 Astra … | CNMO，页面时间 2026-10-27 17:32（正文引韩媒） | https://ai.cnmo.com/news/821217.html | “根据Artificial Analysis（AA）的评测，该模型在Intelligence Index中获得55分，排名第四” | `c-cnmo-kimi.*` | 已找到（发布于 v5 之前） |

英文端对 v5 的“把事件写歪”的实例见“夸大说法实例”节。

### C10 HN 热度

`barking heat docs/2610/1027 c --item 50268868 --item 50269651 --hn "Artificial Analysis Intelligence Index v5" …`，取于北京 2026-10-28 20:33。Reddit 搜索两次均被重定向到登录页（`old.reddit.com/login`），未登录，记入 `c-heat.json` 的 errors；Reddit：**未取到**。

| 帖 | 分数 / 评论 | 提交（北京） | 存档 |
|---|---|---|---|
| https://news.ycombinator.com/item?id=50268868 “Artificial Analysis Intelligence Index v5”（指向公告） | 69 分 / 72 评（含嵌套） | 10-28 11:06:22 | `c-hn-50268868.json` |
| https://news.ycombinator.com/item?id=50269651 “Artificial Analysis Intelligence Index v5.0”（指向方法页） | 38 分 / 5 评 | 10-28 13:29 | `c-hn-50269651.json` |

（派工单写 “66 分 / 72 评”；现查 69 分，增长所致。）

---

## 二、扫描说法勘误

| 扫描说法 | 核对结果 |
|---|---|
| v5 加入 FrontierCode 2.0 与 Terminal-Bench-Science | 与原文相符。 |
| “HLE 换成 HLE-Diamond（或升级 HLE / Omniscience / CritPt）” | 原文：HLE-Diamond **替换** HLE；AA-Omniscience → v1.1；CritPt → v2 hard subset。三项都是“Upgraded”，不是新增。 |
| 删除 GDPval-AA、SciCode、AA-LCR | 相符：“Removed from the Index: GDPval-AA v2.1, SciCode, and AA-LCR v1.1”。 |
| 四大类各 25% | 相符（Knowledge Work / Coding / Science / General，各 25%）。旧版 v4.3.2 是 30/20/20/30，分类名也不同（Agents / Scientific Reasoning）。 |
| “60% 权重用私有或保留答案的题集” | 原文：“60% of the Index is now based on evaluations with private test sets or held-out answers.” 方法页 Private 列加总也是 60%。扫描说法正确；v4.3.2 同法加总为 45%（推算），故不是“首次有私有题”，是“首次过半”。 |
| 任务中位输出 token 35k→59k | 相符（“median rising by two thirds, from 35k to 59k”）。 |
| 分数：Opus 5.5 Max 58、Astra Max 57、Sol Max 55、Argon (high) 50、Kimi 46、DeepSeek 与 GLM 32、Haiku 36、Terra Max 51 | 全部相符（见 C4）。“Kimi（权重尚未发布）”相符。 |
| “Terra 51（旧 48）” | 相符（48 来自 1026 存档 v4.3.2）。 |
| “开放与闭源差距从 3 分变为 12 分” | 与公告相符；但 v5 的“12”以未发布权重的 Kimi K3.1 Pro（46）为基准，v4.3.2 的“12”以已放权重的 MiMo-V2.6-Pro（46）为基准，“3”是权重放出后才成立的预测；AA 自己的模型页与图又把 Kimi 标 Proprietary（C5-3、C6-1、C6-5）。 |
| Ling：卡片称 v4.3.2 “41→44”，AA 现按 v5 显示 29 | 相符。卡片原句 “scores 44 on the Artificial Analysis Intelligence Index v4.3.2, up from 41 for its predecessor”；前代 Ling 3.1 Flash 在 v5 榜单为 “--”。 |
| Ling “2026-10-27 放出权重（MIT，560B 总参 / 25B 激活，称 1M 上下文）” | HF 仓库 2026-10-27T05:46:51Z 创建，权重文件 10:45–10:46Z 上传；license mit；560B / 25B / “1M-token context window”。相符（“放出”以仓库可下载为准）。 |
| 第三方称 config.json 的 max_position_embeddings = 262144 | **读数属实**（本次直接取 config.json，`rope_scaling: null`）；前代同值。但“262144 ⇒ 1M 不成立”的推论未被一手来源证实。 |
| FrontierCode 2.0：Opus 5.5 45.5%（1.1 版 54.6%）、Astra 42.8%、Kimi K3 44.2%→17.7% | 相符，但 **Kimi K3（非 K3.1 Pro）**。 |
| “AA 在 132 题上复跑三次 Opus 5.5 46%、Astra/Terra/Sonnet 5.5 并列 42%” | 公告相符：“132 tasks”（评测页与方法页），“three times”，Opus 46%，Astra/Terra/Sonnet 5.5 均 42%（评测页精确值 Opus 46.4%、Astra 42.4%、Terra 42.3%、Sonnet 41.7%）。“132” 在 AA，不在 Cognition 博文文本里。 |
| 国泰海通、界面新闻、腾讯新闻 10-28 仍写“55 分、全球第四、差距收窄至 3 分” | 国泰海通：**页面存在**（同花顺 10-28 06:48:11），但发布早于 v5 宣布 3 小时 23 分，不是 v5 之后仍写旧口径。界面新闻、腾讯新闻 10-28 稿：**未找到**。 |
| HN “Artificial Analysis Intelligence Index v5” 66 分 / 72 评 | 现查 69 分 / 72 评。 |
| 两份扫描称 AA 公告“权重尚未发布” | 相符。 |
| 扫描称“v5 上线 10-27” | AA 页面日期 10-27（美国日期）；X 发帖时刻为北京 10-28 10:11:22。 |

---

## 三、可能的吠点（原文里有、但线索摘要没提或说反的条件，每条一句）

1. **旧分与新分不能直接相减**：AA 自己写 “Each version of the Intelligence Index is therefore measured on a different set of evaluations, and scores across versions are not intended for long-run comparison.”（Progress Index 方法页，`c-aa-progress.txt` 第 42 行）；同一批模型 v4.3.2→v5 有升有降（Astra +4、Sol(max) +3、Terra(max) +3、Opus 0、Kimi −9、DeepSeek −13、GLM −13、Ling −15）。
2. **“55→46”是同一个模型在两套题上的分数，不是模型退步**：Kimi K3.1 Pro 没有改动；变化来自指数换了 5 项评测与权重（见 C1）。公告图 `28` 中 Kimi 仍是 Terminal-Bench 4.0 最高（67%）。
3. **AA 给 Kimi 的“开放权重”口径前后不一**：公告文字称它为 “the top open weights model at 46”，同页图例与模型页却把它归为 Proprietary，FAQ 写 “No, Kimi K3.1 Pro is proprietary.”（C5-3）。“开放权重差距 3 分→12 分”：v4.3.2 的 “12” 以已放权重的 MiMo-V2.6-Pro（46）为基准，v5 的 “12” 以未放权重的 Kimi K3.1 Pro（46）为基准，同一个数字换了参照模型（C6-5）；“3” 是权重放出后才成立的预测。
4. **v5 的“私有题”不等于可复现**：60% 的项目私有或保留答案；AA 评分里 FrontierCode 2.0、AA-Briefcase 等 LLM 评审项用的是 AA 自己选定的评审（FrontierCode：“We use GPT-6 Luna (medium) as the LLM judge”；AA-Briefcase 的评审名单见 `c-aa-method.txt` 第 442 行）。
5. **v5 的测试更贵更重**：模型输出 token 中位数 35k→59k；效果：Terra (max) 每任务 $0.40→$0.96，Kimi $1.98→$3.68，Opus 5.5 (max) $10.04。旧文章引用的每任务成本在 v5 下全部失效。
6. **v5 之后，上一代 GPT-5.6 Terra 在 AA 没有分数**（N/A，已标 deprecated），所以 1026 期“上一代最高档 42 分”无法再用 AA 原页复核，只能引 v4.3.2 存档。
7. **1026 期“Terra 最高档 = Sol 中档（48=48）”的具体数字在 v5 下不成立，相对关系换档后近似成立**：Terra max 51（$0.96）、Sol medium 46（$0.36）、Sol high 50（$0.59）（C4）。同分对比失效；“Sol high 比 Terra max 低 1 分、约六成成本”仍支持“Terra 最高档不划算”。
8. **Ling 的 “44” 是 v4.3.2 自报，在 v5 下 AA 实测为 29**；前代 Ling 3.1 Flash 在 v5 无分，所以“比前代提升”无法在 v5 下重现。config.json 的 262144 与卡片 “1M” 不一致（C7-5）。
9. **FrontierCode 2.0 分数比 1.1 低 17–27 个百分点**，Cognition 自述是提高了标准（更多文件、更多 blocking criteria），与 AA 把它放进 v5 15% 权重叠加；Kimi K3 17.7% 对应的是 K3，不是 K3.1 Pro。
10. **AA 把 v5 数据直接盖在旧 URL 与旧 Changelog 条目上**：26 Oct 的 Terra 条目现显示 51/29/38/42/45/30（v5），不是当日的 48/36/41/44/45/33（推断，见 C4 末尾）。

### 社区质疑线索（L6，仅线索，进正文前必须回到一手来源）

| 评论 | 得分 | 链接 | 内容 / 与一手来源核对 |
|---|---|---|---|
| wyre：逐项对比 v5.0 与 v4.3 的权重 | HN 评论分未显示（API 不提供） | https://news.ycombinator.com/item?id=50270166 | 列出 v4.3 的 Agents 30/Coding 20/General 30/Scientific Reasoning 20 与 v5 的 25×4，结论 “gained Terminal-Bench Science and FrontierCode, dropped SciCode, GDPval-AA and AA-LCR. Lowered weight of GDP.pdf.” 与 `b-aa-method-intel.txt`、v5 方法页核对：相符（GDP.pdf 10%→5%）。 |
| mirekrusin：逐模型分差 | 同上 | https://news.ycombinator.com/item?id=50270428 | “Kimi K3.1 Pro: −9 … GPT-6 Astra: +4 … Claude Opus 5.5: 0 …” 质问是否与 OpenAI 有关联。本次核对：Kimi −9、Astra +4 与 AA 存档相符；“关联”为猜测，无证据。 |
| rbehrends：Coding Index 单看更合理；Terminal-Bench-Science 拖累小模型 | 同上 | https://news.ycombinator.com/item?id=50270361 | “The introduction of Terminal-Bench Science 0.1 in particular hurts their aggregate scores”；AA 公告确认 “most open weights models we benchmarked score under 10%” 于该项。 |
| hobofan：评测题量少（编码约 200、科学约 150） | 同上 | https://news.ycombinator.com/item?id=50270060 | 方法页：FrontierCode 132×3 次、Terminal-Bench-Science 70×3、CritPt 40×5、Terminal-Bench 4.0 66×3；题量确实不大，“约 200 / 150”是 HN 作者的概数。 |
| chvid：为何用闭源模型做评审 | 同上 | https://news.ycombinator.com/item?id=50270115 | AA 评测页：“We use GPT-6 Luna (medium) as the LLM judge.” 相符。 |
| peri-cl：新指数悄悄让若干开放权重模型“N/A” | 同上 | https://news.ycombinator.com/item?id=50271289 | 核对：`https://artificialanalysis.ai/models/glm-5-3-flash`（`c-aa-m-glmflash`）Intelligence 卡片 N/A；`…/qwen3-8-flash-next`（`c-aa-m-qwenflash`）页面同为 N/A。**属实**（北京 10-28 20:3x 取）；公告未提这些模型。 |
| thereitgoes456（已被同串反驳） | 同上 | https://news.ycombinator.com/item?id=50269895 | 称 AA 直接采用 Cognition 分数、用 Devin 跑开放模型；shelfchair 指出 AA 用自家 Stirrup 跑 FrontierCode 2.0（评测页原文确认），原作者已承认。**不可用。** |
| irthomasthomas：新增私有源后不可复现 | 同上 | https://news.ycombinator.com/item?id=50271561 | “includes two private sources, data which we can't reproduce”（公告确认 60% 私有；“two”为其概数，方法页 Private 列有 5 项）。其后半段对 Anthropic 的指控与本题无关，不引。 |

## 四、夸大说法实例（把事件写歪的标题或文章）

找到 2 篇较典型的英文端实例，中文端暂未找到。

| 稿件 | 发布方 / 时刻 | 原站 URL | 原句逐字 | 问题 |
|---|---|---|---|---|
| “Artificial Analysis's New A.I. Index Penalizes Open-Weight Models” | Trending Topics，2026-10-28T06:33:37Z（北京 14:33:37） | https://www.trendingtopics.eu/artificial-analysis-index-v5-open-weight-models/ （`c-trendingtopics-v5.*`） | 标题；正文：“Because the index now relies mostly on private test data, open-weight models, especially from China, have slid sharply.” | “Penalizes”（惩罚）是定性判断；“Because … private test data” 是因果断言，AA 公告并未把分数下降归因于私有数据（HN 评论认为 Terminal-Bench-Science 等也有影响）。同文 “Kimi K3.1 Pro (Moonshot AI): Falls from 55 to 46 points” 把 v4.3.2 与 v5 分数直接相减。 |
| “Artificial Analysis Launches Intelligence Index v5 With Majority Private Datasets, OpenAI Models Gain, Muse Wonder, Kimi Drop” | OfficeChai，2026-10-28T04:55:27Z（北京 12:55:27） | https://officechai.com/ai/artificial-analysis-launches-intelligence-index-v5-with-majority-private-datasets-openai-models-gain-muse-wonder-kimi-drop/ （`c-officechai-v5.*`） | 标题 “… OpenAI Models Gain, Muse Wonder, Kimi Drop” | 标题用 “Gain / Drop” 描述跨版本分数变化；AA 公告本身没有写某模型“下降”，只写名次与差距。 |
| 对照（准确转述） | TechApple（繁体），2026-10-28T14:35:48+08:00 | https://techapple.com/archives/64755 | “與領先者的差距由上一版 3 分擴大至 12 分。Kimi K3.1 Pro（權重未釋出）以 46 …” | 忠实转述 AA 句式，未写“下降”。 |

中文端“55→46 骤降”的夸大标题：**未找到**（Brave 搜索只返回 AA 公告转述：新浪微博转述 `https://www.sina.cn/weibo/detail/5359081984363731.html` 北京 10-28 19:49，只写 “开源与闭源模型差距拉大”，未写骤降）。

## 五、待办 / 本组未覆盖

- C7-6 蚂蚁官方新闻稿未检索。
- Kimi K3.1 Pro 的 v5 FrontierCode 2.0 分数 25.5% 只读自图（C8-5）。
- v5 切换的精确上线时刻（C2-5）无一手来源；Wayback 未查。
- Reddit 热度：被登录页拦截，未取。
