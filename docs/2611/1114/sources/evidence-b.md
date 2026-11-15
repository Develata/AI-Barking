# 1114 期 B 组取证：Deepnote Agent Initiative Benchmark（主动性基准）

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。北京时间（UTC+8，括号保留原时区）。取证时间 2026-11-15 20:54–21:25。存档前缀 `b-`。截图在 `images/13–29`（见文末截图表）。
级别沿用 EDITORIAL 事实分级：官方页与官方仓库为 L1/L2，媒体转载为线索，社区评论为 L6。

读法：每行“状态”只用 已找到 / 部分支持 / 与说法不符 / 未找到一手来源。“推断”一栏明确标出的是我从官方数字做的算术，不是官方原句。

## 一、关键结论（先看这里）

1. **B4 超时 + 沉默题的计分规则找到了原文**：仓库评审提示 `src/benchmark/prompts/judge/evaluate_agent_run.txt` 第 36 行（8.）“Reaching the model-response budget adds no separate penalty.”，第 113 行“An agent stop caused by its configured model-response budget is graded exactly like finish_task. Judge its actual delivered messages or silence. Apply no timeout penalty and do not invent a finish_task event.”，第 35 行（7.）“With no delivered message, rate intervention-or-silence and overall. … Correct silence receives 10.” 即：超时的运行按“实际发出的消息或沉默”评分；沉默题上超时且没发消息 = 正确沉默（该项 10 分），不是扣分。
2. 官方榜单的 “Stayed quiet / Helped” 只统计“有没有消息送出”（图注原句 “Message delivery only. Whether a message was any good is scored separately.”），不区分“主动忍住”与“超时没来得及说”。
3. **Gemini 4 Argon “Stayed quiet 10/10”与 27 次超时的关系（推断，算术自洽）**：50 次运行里发出消息 23 次（全在帮忙题，因为 Helped 23/40），没发消息 = 17（帮忙题没发）+ 10（沉默题没发）= 27，正好等于官方写的“reached the 180-second limit before any message was sent: 27”。所以只要超时运行都没发消息（官方句意如此），Gemini 沉默题的 10 次“没发消息”全是触到 180 秒。Gemini 的“0 分次数 17”也正好等于帮忙题没发消息的 17 次，说明这 10 次沉默题没有被判 0。**逐题结果未公开**，所以“10 次都超时”是推断，不是直接核对。
3b. 同样的算术用在 Grok：没发消息 = 16 + 4 = 20，超时 18，差 2（官方没解释；仅记录）。
4. **仓库只含 8 个示例任务（task-01 至 task-08），不是 50 题全集，也没有 600 次运行的逐题结果、消息或评审理由**；`docs/results.md` 原句 “These eight selected examples are not a score for the full benchmark.” 所以榜单数字第三方无法从公开仓库复算。
5. 评审是哪两个模型：官方页与仓库均**未写**（仓库只要求两个不同的评审模型 ID）。
6. 中文侧：新智元稿（2026-11-14 17:16:28，网易号；新浪财经同日 17:16 转载）是主要传播件，标题“Fable 5.2贵17倍还输了！12 个AI进群当同事，OpenAI包揽前二”，含多处与原文不符的转述（见“夸大说法实例”）。HN 仅 2 分 1 评。创始人 X 帖 387,707 次浏览、683 赞。
7. 官方页自身有一处前后不一致：榜单表 Opus 5.5 与 Fable 5.2 的 Helped 都是 40/40，而正文写 “Opus 5.5 is the reverse: a step behind at helping”。

## 二、B1–B8 逐条

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B1a | 12 个模型、50 个职场情境 | L1 | https://deepnote.com/research/benchmark | “We tested 12 frontier models on 50 workplace situations inside a simulated company.” | 13-page-title.png | 页面无发布日期 | 已找到 |
| B1b | 40 帮 / 10 默 | L1 | 同上 | 图表标题 “Help needed · 40 tasks”、“Silence needed · 10 tasks”；正文 “Silence is a moat … The best models still spoke in 2 of 10 situations that called for silence” | 17-silence-chart.png | 页面没有单独写“40/10”一句，由图表标题给出 | 已找到 |
| B1c | 180 秒，只计模型响应时间 | L1 | 同上 | “The budget is 180 seconds of model response time. Tool execution does not count.”；“the same 180-second budget.” | 14-method-run.png | 表脚注：“Time is awaited model response, tool execution excluded.” 仓库 `agent_runner.py` DEFAULT_LIMITS `"agent_model_seconds": 180`，各 task config 的 limits 也是 180 | 已找到 |
| B1d | 超时即停；帮忙题没发消息算漏 | L1 | 同上 | “A run that reaches the limit stops immediately. If help was needed and no message went out, the run is scored as a miss.” | 14-method-run.png | 页面只写了帮忙题；沉默题见 B4 | 已找到 |
| B1e | 两个 LLM 评审，盲评 | L1 | 同上 | “Two models blind to the fact which model is being evaluated grade the run 0 to 10 against a reference solution, with the company data and the full action log.”；“The score is the mean of two LLM evaluators, 0 to 10, averaged over all 50.” | 14-method-run.png | **评审是哪两个模型：未写**（页面、README、docs 均无） | 已找到（评审身份未写） |
| B1f | high effort | L1 | 同上 | “Every model elaborated on the same 50 situations at high effort.”（原文 elaborated，似为 ran 之误）；“One effort level. Every model ran at high.” | 25-quick-findings.png、26-limitations.png | 仅这一个档位 | 已找到 |
| B1g | 600 runs | L1 | 同上 | “Across 600 runs, OpenAI leads on score and value, Anthropic on speed, and models find it harder to stay quiet than to speak up.” | — | 12×50=600。页面没有明说“每题每模型只跑一次”或“重复运行”；仓库里的 attempts 只是中断后续跑与技术错误重试（`execution.py` start/retryable），没有重复采样机制。“只跑一次”为推断；置信区间只重抽任务（“task-bootstrap”），不含同一题多次运行的波动 | 部分支持（600 属实；“各跑一遍”为推断） |
| B1h | 发布日期 | L1/L2 | 页面无日期；GitHub API https://api.github.com/repos/deepnote/agent-initiative-benchmark/commits | 提交 “Present our results”：author 2026-11-13T07:20:02Z，committer 07:48:01Z；“Update citation instructions” 2026-11-13T06:25:41Z；“Update README.md” 2026-11-12T14:13:24Z；“Add benchmark source code” author 2026-11-08T17:46:06Z / committer 2026-11-10T11:34:24Z | — | 北京时间：结果提交 2026-11-13 15:20:02（author）/ 15:48:01（committer）；创始人 X 帖 2026-11-13 22:58:55；HN 2026-11-14 00:12:10。官网页响应头 Date 为 Fri, 13 Nov 2026 15:29:50 GMT（= 11-13 23:29:50 北京），是我取页时的响应时间（疑为缓存渲染时间），**不是发布时间**。仓库可能在发布前已公开（11-12 的提交早于 11-13），未能确认 | 部分支持（官方未给页面发布时刻；综合证据为 2026-11-13，北京时间 15:48 之后至 22:58 之前） |
| B2a | Astra 8.97，Sol 8.65 落在 Astra 区间内，“does not separate them” | L1 | 同上 | “GPT-6 Astra leads at 8.97. GPT-6.1 Sol at 8.65 sits inside Astra’s interval, so this task set does not separate them.” | 15-leaderboard-intervals.png、16-not-separate-text.png | Astra 95% 区间 8.30–9.52（“95% task-bootstrap interval … where the mean would land if the task set were redrawn”） | 已找到 |
| B2b | Fable 5.2 第三，8.17 | L1 | 同上 | “Claude Fable 5.2 is third at 8.17: below Astra’s interval, inside Sol’s. It leads Decide tasks, but it has not passed either OpenAI model overall.” | 16-not-separate-text.png | Fable 5.2 区间 7.34–8.89；Sol 7.88–9.31。8.17 低于 Astra 区间下界 8.30，但 Fable 5.2 的区间与 Astra 的区间（8.30–9.52）相互重叠，也包含 Sol 的均值 8.65。同页小节标题却写 “Two OpenAI models beat the newest Fable” | 已找到 |
| B2c | 全部区间（原文逐项） | L1 | 同上 | Astra 8.30–9.52；Sol 7.88–9.31；Fable 5.2 7.34–8.89；Opus 5.5 7.14–8.46；Fable 5.1 6.66–8.24；Sonnet 5.5 5.70–7.53；Luna 5.40–7.31；DeepSeek V4.1 Flash 5.41–7.23；Mistral Large 4 5.10–7.03；Haiku 5.5 5.16–6.88；Gemini 4 Argon 4.41–6.76；Grok 4.7 3.39–5.65 | 15 | 同上 | 已找到 |
| B3a | 9/10 对 6/10 | L1 | 同上 | “When help was needed, a message went out 9 times out of 10. When silence was right, a message was held back only 6 times out of 10.”（快速发现第 2 条）；正文 “Agents almost always speak up when help is needed: a message went out 9 times out of 10. Nearly every miss was a run that reached the time limit before sending anything.” “Holding back is where they slip. When silence was right, models stayed quiet only 6 times out of 10. The best held back 8 times in 10. The weakest spoke more often than not.” | 17、18、25 | 图注 “Message delivery only. Whether a message was any good is scored separately.” 复算（推断）：Helped 合计 434/480 = 90.4%，Stayed quiet 合计 75/120 = 62.5%，与“9/10、6/10”吻合；去掉 Gemini 后 65/110 = 59% | 已找到 |
| B3b | 12 个模型整表 | L1 | 同上 | 见第三节整理表 | 19-results-table.png（整表，含列头与脚注） | 页面无独立 Gemini 卡片的静态展开（卡片在交互面板里），表格行即其逐字数据 | 已找到 |
| B4a | 27 与 18 次超时 | L1 | 同上 | “Gemini 4 Argon and Grok 4.7 trail mainly because of runs that reached the 180-second limit before any message was sent: 27 and 18 runs.” | 16-not-separate-text.png | 另见 “Gemini 4 Argon’s median run reached the 180-second limit.”、中位 3 min 0 s；Grok 中位 2 min 41 s、P90 3 min 0 s；“Astra and Sol handled that trade-off best: neither reached the limit more than once in 50 runs.” | 已找到 |
| B4b | 超时 + 沉默题的计分 | L2 | https://github.com/deepnote/agent-initiative-benchmark/blob/4d66be81b4fc44ba0fa0fc48320be3ebfa717e3e/src/benchmark/prompts/judge/evaluate_agent_run.txt | 第 35 行：“7. With no delivered message, rate intervention-or-silence and overall. Set the other three categories to null. Correct silence receives 10.” 第 36 行：“8. Technical failures and invalid tasks are errors with null scores, not quality zeros. Reaching the model-response budget adds no separate penalty.” 第 113 行：“An agent stop caused by its configured model-response budget is graded exactly like finish_task. Judge its actual delivered messages or silence. Apply no timeout penalty and do not invent a finish_task event.” 另 `src/benchmark/runtime/judge_runner.py` 第 1635–1637 行把 `outcome == "timeout"` 且 `stop_reason == "model_budget"` 的运行标为 eligible（可被评分）；`runtime/agent_runner.py` 第 706–710 行：超过预算的那次响应“its calls were not executed”（越线那次响应里的发消息调用不会执行） | 27-repo-judge-prompt-silence-timeout.png | 这是公开仓库 v1.0 的评审提示（pin 到提交 4d66be81…）；**无法证明 600 次官方运行用的就是这一份**（README 称 “Data, tasks and prompts are pinned”）。页面方法段只写了帮忙题超时算漏，没写沉默题超时 | 已找到（仓库规则）；官方运行是否同版未证实 |
| B4c | Gemini 沉默题 10 次是否都超时 | — | 无逐题结果 | 仓库 `docs/results.md`：“These eight selected examples are not a score for the full benchmark.”；仓库树无 results 目录；HF 数据集页只描述公司数据 | — | 逐题结果、消息与评审理由均未公开。推断见“一、3” | 未找到一手来源（逐题结果未公开）；算术推断自洽 |
| B5a | 成本表 | L1 | 同上 | Cost/task：Astra $0.693、Sol $0.112、Fable 5.2 $1.87、Opus 5.5 $1.28、Fable 5.1 $3.02、Sonnet 5.5 $0.420、Luna $0.0070、DeepSeek $0.026、Mistral $0.082、Haiku $0.075、Gemini $0.403、Grok $0.425 | 19、21 | “Cost is recorded model spend per task in USD.” 另有 “Recorded costs are historical observations, not a live price quote.” | 已找到 |
| B5b | 430× | L1 | 同上 | “GPT-6 Luna ($0.007) and Claude Fable 5.1 ($3.02) are 430× apart in price and 1.1 points apart on a 10-point score.”（快速发现 1）；“Prices span 430×, from Luna at $0.007 per task to Fable 5.1 at $3.02, and those two are 1.1 points apart.” | 25、21 | 分子 Fable 5.1 $3.02，分母 Luna $0.007；两者分数 7.50 对 6.38，区间 6.66–8.24 与 5.40–7.31 重叠 | 已找到 |
| B5c | 月账单 | L1 | 同上 | “One investigation per Slack thread in PageMantle’s live month, about 2,500 threads at a 70-person company, comes to $281 on Sol 6.1 and $4,684 on Fable 5.2.”；表 “A month of Slack”：Luna $18、DeepSeek $66、Haiku $188、Mistral $204、Sol $281、Gemini $1,008、Sonnet $1,049、Grok $1,061、Astra $1,732、Opus $3,196、Fable 5.2 $4,684、Fable 5.1 $7,540 | 21 | 官方原句另有 “Sol scores 0.32 below Astra at one-sixth of the price.”“Sol 6.1 … at under a tenth of Opus’s price”。**官方页自己没有写“17 倍/16 倍”这个倍数**；它给的是各自美元数 | 已找到 |
| B5d | 中文稿的倍数 | L5（线索） | https://www.163.com/dy/article/LBN9SFNU0511ABV6.html | “排第三的Anthropic Fable 5.2，8.17分，单个任务1.87美元。 贵了17倍，分还低了。”；“Sol是281美元，Fable 5.2是4684美元。 同样的活儿，Fable 5.2贵16倍。”；“单任务花费是Sol的17倍、月账单是Sol的16.7倍”；“Fable 5.1每单3.02美元，GPT-6 Luna每单0.007美元，价格差了430倍，得分只差1.1分”；“花430倍的钱，只多11%的分数！” | 23、29 | 17 倍分子 Fable 5.2 单任务 $1.87、分母 Sol $0.112；16/16.7 倍分子 $4,684、分母 $281（同一篇里 17、16、16.7 并存）。“只多11%”：官方是 1.1 分 /10 分制 | 部分支持（见勘误） |
| B6a | 仓库 | L2 | https://github.com/deepnote/agent-initiative-benchmark | README “Agent Initiative Benchmark v1.0 — Evaluating agents that help without being asked.” | — | 4 个提交；1 star；许可证：代码、文档、任务 Apache-2.0；PageMantle 数据集及其清单 CC BY 4.0；含评审提示（`src/benchmark/prompts/judge/`）；含 8 个示例任务（每类 1 个）；不含逐题结果；README “Judge models assess the decision to intervene, evidence, message quality and audience. Data, tasks and prompts are pinned. Tool calls, messages and both verdicts stay available in the CLI.”（指你自己跑的结果） | 已找到 |
| B6b | 数据集 | L2 | https://huggingface.co/datasets/deepnote/pagemantle-company-dataset | “PageMantle is a fictional company that sells software for building and hosting websites.” “Size: 5.97 GB compressed, 10.11 GB extracted, 101,337 files.” License cc-by-4.0 | — | Dataset Viewer 已禁用；DOI 10.5281/zenodo.24005613；快照 2026-09-30。数据集是公司数据，不含评测结果 | 已找到 |
| B7a | HN | L6 | https://news.ycombinator.com/item?id=50485628 | “Benchmark for Proactive Agents”，2 分，1 评；发帖 2026-11-14 00:12:10（北京）（= 2026-11-13T16:12:10Z） | — | 唯一评论（2026-11-14 00:19:49 北京）：“was pretty cool to see the price difference between Sol (and Luna) and Fable 5.2 (430x is kinda crazy).” 官方 430× 是 Luna 对 Fable 5.1，不是 Fable 5.2（读者误读实例）。`barking heat`（`b-heat.json`）HN 查询 “agent initiative benchmark” 命中 0 | 已找到 |
| B7b | Reddit | — | — | 搜索失败：Reddit 返回登录页（`b-heat.json` errors）；opencli reddit 被 “network security” 403 拦截 | — | 未取到，记失败 | 未找到一手来源（未取到） |
| B7c | 新智元原稿 | L5 | https://www.163.com/dy/article/LBN9SFNU0511ABV6.html | 页面显示 “2026-11-14 17:16:28　来源: 新智元”，北京；文末参考资料列出创始人 X 帖与官方页；编辑：所罗门 | 23-zh-163-title-date.png | 阅读数页面不可见；跟贴数：我 20:59 取到的存档显示 0，21:15 浏览器所见为 2。**新智元微信公众号原文未取到**（搜索未返回 mp.weixin.qq.com），此处以网易号版为准 | 已找到（网易号版） |
| B7d | 新浪财经转载 | L5 | https://finance.sina.com.cn/stock/t/2026-11-14/doc-iniyszsz2786927.shtml | “2026年11月14日 17:16”，“（来源：新智元）”，正文与网易号版一致 | — | 阅读数不可见 | 已找到 |
| B7e | 同花顺转载 | — | — | 扫描称有同花顺转载 | — | 我两次搜索（含 同花顺 关键词）均未找到同花顺转载页；也没找到量子位、机器之心、36氪的报道 | 未找到一手来源 |
| B7f | 创始人 X 帖 | L1 | https://x.com/jakubjurovych/status/2121256499552915586 | “6.1 Sol and Astra from @OpenAIDevs beat Anthropic's latest Fable 5.2, with Sol crushing it at both performance and cost ($0.11 per task vs $1.87).” “The most expensive models aren't the best: Fable 5.1 is 430x more expensive than GPT-6 Luna, with similar performance.” “Most models try to help even if they're not needed. Interestingly, in these failure cases, they spend more time thinking, and the unnecessary work takes about 80% more time and money than if they stayed quiet.” | 24-x-founder-post.png（只含帖子卡片，页面显示的是浏览器本地时区 10:58 PM Nov 13, 2026） | fxtwitter 公开 JSON（`b-x-2121256499552915586.json`，取于 2026-11-15 21:00）：发布 Fri Nov 13 14:58:55 +0000 2026（= 2026-11-13 22:58:55 北京）；likes 683、retweets 110、replies 132、quotes 83、bookmarks 671、views 387,707；无 community note。创始人身份：该帖作者 “Jakub Jurovych”（官方博客称其为 CEO，来自搜索摘要，未单独核）。132 条回复未采集 | 已找到 |
| B7g | 其他转发 | L5/L6 | https://x.com/chetaslua/status/2121259512414032320 | “GPT-6 Luna is 430x cheaper than Claude Fable 5.1 as an always-on Slack agent , and only 1.1 points behind” | — | 2026-11-13 23:10:53（北京）；94 赞、27,460 次浏览。把“价格差 430×”写成“便宜 430×”，量级同官方 | 已找到 |
| B8 | 夸大实例 | — | 见第五节 | — | 23、28、29 | — | 部分找到 |

## 三、12 模型整表（照官方页原文数字；来源 https://deepnote.com/research/benchmark 的 Results table，截图 19-results-table.png）

| # | 模型 | Score | 10/10 次数 | 0/10 次数 | Helped | Stayed silent | 超时次数 | Time | Cost/task | Tool calls |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | GPT-6 Astra | 8.97 | 26 | 2 | 40/40 | 8/10 | 页面未列（“neither reached the limit more than once in 50 runs”，Astra 与 Sol 合说） | 54 s | $0.693 | 19.3 |
| 2 | GPT-6.1 Sol | 8.65 | 22 | 3 | 39/40 | 8/10 | 同上 | 54 s | $0.112 | 18.0 |
| 3 | Claude Fable 5.2 | 8.17 | 18 | 4 | 40/40 | 6/10 | 未列 | 1 min 0 s | $1.87 | 20.9 |
| 4 | Claude Opus 5.5 | 7.84 | 14 | 2 | 40/40 | 8/10 | 未列 | 29 s | $1.28 | 10.0 |
| 5 | Claude Fable 5.1 | 7.50 | 11 | 3 | 40/40 | 6/10 | 未列 | 51 s | $3.02 | 14.5 |
| 6 | Claude Sonnet 5.5 | 6.65 | 11 | 7 | 37/40 | 6/10 | 未列 | 15 s | $0.420 | 8.9 |
| 7 | GPT-6 Luna | 6.38 | 9 | 9 | 38/40 | 4/10 | 未列 | 50 s | $0.0070 | 13.7 |
| 8 | DeepSeek V4.1 Flash | 6.35 | 7 | 7 | 38/40 | 4/10 | 未列 | 1 min 26 s | $0.026 | 21.1 |
| 9 | Mistral Large 4 | 6.09 | 6 | 9 | 37/40 | 4/10 | 未列 | 1 min 25 s | $0.082 | 22.5 |
| 10 | Claude Haiku 5.5 | 6.05 | 5 | 6 | 38/40 | 5/10 | 未列 | 28 s | $0.075 | 12.1 |
| 11 | Gemini 4 Argon | 5.60 | 11 | 17 | 23/40 | 10/10 | 27 | 2 min 44 s | $0.403 | 34.5 |
| 12 | Grok 4.7 | 4.51 | 8 | 20 | 24/40 | 6/10 | 18 | 2 min 25 s | $0.425 | 35.5 |

中位 / P90 时间（官方原文）：Astra 53 s/1 min 31 s；Sol 46 s/1 min 45 s；Fable 5.2 52 s/1 min 43 s；Opus 23 s/48 s；Fable 5.1 46 s/1 min 19 s；Sonnet 14 s/27 s；Luna 37 s/1 min 29 s；DeepSeek 1 min 14 s/2 min 45 s；Mistral 1 min 17 s/2 min 17 s；Haiku 25 s/45 s；Gemini 3 min 0 s/3 min 0 s；Grok 2 min 41 s/3 min 0 s。

推断（算术，非官方原句）：
- 没发消息的运行数 = (40 − Helped) + Stayed quiet：Gemini 17 + 10 = 27（= 官方超时数 27）；Grok 16 + 6 = 22（官方超时 18，差 4）。
- 0 分次数与“帮忙题没发”对照：Gemini 17 = 40 − 23；Astra 2 = 沉默题插话 2（8/10）+ 帮忙题漏 0；Sol 3 = 1 + 2；Fable 5.2 4 = 0 + 4；Opus 2 = 0 + 2；Sonnet 7 = 3 + 4；Mistral 9 = 3 + 6；Grok 20 = 16 + 4。个别模型对不上（Fable 5.1 为 3 而非 4；Luna 9 对 2 + 6 = 8；DeepSeek 7 对 8；Haiku 6 对 7），说明“沉默题插话”并非一律判 0，也不是官方逐项说明。Gemini 的 11 个 10 分里，若沉默题正确沉默都得 10，则至多 1 个来自帮忙题。

## 四、可能的吠点（原文里有、线索摘要没提或说反的条件；L1 除另注）

1. “Stayed quiet 10/10”不等于“最会克制”：官方图注 “Message delivery only”，且 Gemini 的 27 次“没发消息”与官方超时数一致，沉默题的 10 次几乎必然是超时（推断，见一、3）。官方自己也说 “Most missed messages were timeouts, not deliberate choices to stay silent.”（页面 “Running out the clock”）。
2. 超时在沉默题上不扣分（仓库评审提示 7、8、113 行），与页面方法段只写帮忙题的“算漏”并不矛盾，但读者会以为“超时一律是失败”；而在沉默题里超时恰好得到满分。方向因题型而异。
3. 超时的计时口径：180 秒只计“模型响应时间”，工具执行不计（页面 + 表脚注）；表里的 Time 也是响应时间，不是墙钟时间。越线那次响应里的发消息调用不会执行（`agent_runner.py` 第 706–710 行）。
4. 区间重叠比标题更宽：Fable 5.2 的区间（7.34–8.89）与 Astra（8.30–9.52）和 Sol（7.88–9.31）都重叠，页面写 “below Astra’s interval, inside Sol’s” 是指均值点相对区间；同页小节标题 “Two OpenAI models beat the newest Fable”与正文 “has not passed either OpenAI model overall” 口径不一。Luna 对 Fable 5.1（430×价差、1.1 分）区间也重叠。
5. 区间只重抽“任务”（task-bootstrap），且每题每模型似乎只跑一次（推断）；沉默题只有 10 道，Stayed quiet 的 4/10、6/10、8/10 每一格都是 10 次里的计数，粒度很粗。
6. 只有 50 题（页面 Limitations：“A 50-task panel. Deepnote will release a larger sample of tasks in the next version.”）；每类 5–8 题（“Each type has 5 to 8 tasks, so read these as tendencies to test on your own work, not fixed rankings.”）；沉默题 10 题。
7. 其他限制（原文）：“Fixed snapshot … teammates do not reply.”；“One effort level. Every model ran at high.”；“The agent replies only through Slack.”；方法段 “This version compares models, not harnesses.”；“Deepnote is now building a harness benchmark.”
8. 评审：两个 LLM，身份未写；评审以“参考解 + 公司数据 + 完整动作日志”打分，参考解只是作者意图（评审提示：“The private reference … is not the only acceptable solution.”）。评审与被测模型是否同厂商，未披露（未验证）。
9. 运行方即出题方自述：数据、任务、评审提示由 Deepnote 自建；公开仓库只有 8 个示例任务，600 次运行的逐题结果不可复核（`docs/results.md`）；公司本身是合成数据（页面 “PageMantle is synthetic”）。Deepnote 自述“正在做一个 agent 工作区”（“Deepnote is building a workspace where agents and people work on data together, which is why the question matters to us.”），页面末尾有 “Request a benchmark” 招揽。
10. 官方页内部不一致：表里 Opus 5.5 与 Fable 5.2 的 Helped 同为 40/40，正文却写 Opus “a step behind at helping”；快速发现 5 写 Gemini/Grok “failed to reach an answer in ~half the tasks that required assistance”（23/40 = 57.5%、24/40 = 60% 发了消息）。
11. “Fable 5.2 贵”的口径：成本是“每任务记录的模型花费”，含 high effort 的思考与多工具调用；官方说 “Recorded costs are historical observations, not a live price quote.”；月账单是假设“每个线程一次调查、约 2,500 线程”的外推（“We did not test a router.”同理，路由建议未测试）。
12. 社区（L6，仅线索）：HN 仅 1 条评论，把 430× 误记成 Sol/Luna 对 Fable 5.2（链接见 B7a）。一篇 dev.to 文章（作者声明是 AI agent，标签 #botwrotethis，https://dev.to/quiet_context/a-model-leaderboard-ranked-by-average-score-the-bill-per-perfect-answer-ranks-it-differently-2odi ，2026-11-14）独立做了同样的 27 = 17 + 10 算术，并指出“超时与沉默决定在 Stayed quiet 列里看起来一样”；这条是 AI 写的二手分析，只作线索。daily.dev 聚合页显示 37 条评论、2.4K 展示（聚合页，评论内容未取到，不采信）。Reddit 未取到。

## 五、夸大说法实例

| 发布方 | URL | 原句（逐字） | 发布时间（北京） | 与原文出入 |
|---|---|---|---|---|
| 新智元（网易号） | https://www.163.com/dy/article/LBN9SFNU0511ABV6.html | 标题“Fable 5.2贵17倍还输了！12 个AI进群当同事，OpenAI包揽前二” | 2026-11-14 17:16:28 | 官方：Fable 5.2 的区间（7.34–8.89）包含 Sol 的均值 8.65，页面自己写 “inside Sol’s … has not passed either OpenAI model overall”；“还输了”与 Sol 的差异在区间内。“贵17倍”= $1.87 对 $0.112（官方给的是美元数，倍数是新智元算的） |
| 同上 | 同上 | “花430倍的钱，只多11%的分数！” | 同上 | 官方 “430× apart in price and 1.1 points apart on a 10-point score”：1.1 分是 10 分制的 11%，不是“多 11% 的分数”（7.50 对 6.38 相对多约 17.6%，此为复算，非新倍数）；且 Luna 与 Fable 5.1 区间重叠 |
| 同上 | 同上 | “同一个模型插错嘴时，平均比闭嘴多花1.8倍的时间和金钱。” | 同上 | 官方：“it cost about 1.8× as much as staying quiet, and took about 1.8× as long”；创始人帖：“about 80% more time and money”。“多花1.8倍”字面是 2.8 倍，应为“是 1.8 倍/多 80%” |
| 同上 | 同上 | “帮忙这件事，所有模型都挺积极的，需要出手的40个任务里9成都能搞定。” | 同上 | 官方 “a message went out 9 times out of 10”，图注 “Message delivery only. Whether a message was any good is scored separately.”；“搞定”把“发了消息”写成“做对了” |
| 同上 | 同上 | “Gemini在50个任务里有27个超时没交卷，Grok有18个。” | 同上 | 官方 “reached the 180-second limit before any message was sent: 27 and 18 runs”；Gemini 的 27 里包含 10 次沉默题（推断），那 10 次不是“没交卷”失败；“3分钟上限”指模型响应时间，工具执行不计 |
| 同上 | 同上 | “12个模型各跑一遍，每次3分钟上限，两个盲评模型独立打0到10分，600次完整运行” | 同上 | “各跑一遍”官方未明写；“3分钟”漏了“只计模型响应时间” |
| 同上 | 同上 | “GPT-6 Astra … Sol …6.1是这场考试的最大赢家。 … 96%的性能，六分之一的价格” | 同上 | 官方 “Sol scores 0.32 below Astra at one-sixth of the price”；8.65/8.97 ≈ 96%（复算）；区间重叠写对了 |
| 同上 | 同上 | “Fable 5.2排第三看着不错，但单任务花费是Sol的17倍、月账单是Sol的16.7倍。”（同文另一处“贵16倍”） | 同上 | 同一篇里 17、16.7、16 三个倍数并存；官方只给美元数（$1.87/$0.112；$4,684/$281） |
| 新浪财经（转载新智元） | https://finance.sina.com.cn/stock/t/2026-11-14/doc-iniyszsz2786927.shtml | 同上标题与正文 | 2026-11-14 17:16 | 同上 |
| 创始人 X 帖 | https://x.com/jakubjurovych/status/2121256499552915586 | “6.1 Sol and Astra from @OpenAIDevs beat Anthropic's latest Fable 5.2, with Sol crushing it at both performance and cost” | 2026-11-13 22:58:55 | 区间重叠，官方正文 “has not passed either OpenAI model overall” |
| 创始人 X 帖 | 同上 | “Fable 5.1 is 430x more expensive than GPT-6 Luna, with similar performance.” | 同上 | 7.50 对 6.38，差 1.1 分，区间重叠；“similar”是帖子措辞 |
| @chetaslua | https://x.com/chetaslua/status/2121259512414032320 | “GPT-6 Luna is 430x cheaper than Claude Fable 5.1 as an always-on Slack agent , and only 1.1 points behind” | 2026-11-13 23:10:53 | “always-on Slack agent”是月账单外推场景 |
| 英文 HN 评论 | https://news.ycombinator.com/item?id=50485628（评论 50485725） | “(430x is kinda crazy)” 写在 Sol/Luna 对 Fable 5.2 的语境里 | 2026-11-14 00:19:49 | 430× 实指 Luna 对 Fable 5.1 |
| “OpenAI 包揽前二”“贵 17 倍还输了”“Gemini 最懂沉默” | — | 前两种见上；**“Gemini 最懂沉默”这种说法我没有找到实例** | — | 若有人这样写，要回到“Message delivery only + 27 次超时”（见一） |

扫描说法的另一个来源 vibecafe（https://vibecafe.ai/news/cmwfp2xmc000108jpeqf6z16v ，AI 播报稿，发布时间页面只写“2 天前”）写 “多数模型在不需要插话时依然爱强行帮忙，白白多花八成时间与费用”“经常直接超时失败”，基本沿用官方，未见明显偏差；casrai.org 聚合页（https://casrai.org/news/deepnote-agent-initiative-benchmark-pagemantle-release-2026）为二手综述，内容与官方一致，不采信为来源。

## 六、扫描说法勘误

| 扫描/Claude 初稿说法 | 以原文为准 |
|---|---|
| 页面日期 11-13 发布 | 页面无日期；仓库结果提交 2026-11-13 15:20–15:48（北京），创始人帖 11-13 22:58；见 B1h |
| 40 个需帮忙、10 个应沉默 | 页面无单句；由图表标题 “Help needed · 40 tasks / Silence needed · 10 tasks” 给出 |
| “Across 600 runs” = 每题每模型一次 | 页面只写 600 runs 和 12 模型 × 50 情境；“一次”为推断 |
| 评审是两个 LLM | 属实；评审是哪两个模型 **未写** |
| 超时在沉默题怎么算（Claude 缺的前提） | 仓库评审提示：超时按实际消息或沉默评分，“Apply no timeout penalty”，“Correct silence receives 10” |
| Gemini “Stayed quiet 10/10” 卡片 | 表中有该数据（19-results-table.png）；官方没有静态“Gemini 卡片”页面文本，卡片在交互面板（页面文本里的 “Helped 23/40 Stayed quiet 10/10” 来自表） |
| 成本：“Sol $0.112、Fable 5.2 $1.87、Fable 5.1 $3.02、Luna $0.007” | 属实（Luna 表内写 $0.0070） |
| “贵17倍/16倍/430倍”多个倍数 | 官方只写 430×（Luna 对 Fable 5.1）；17/16/16.7 是新智元自算 |
| 新智元 11-14 17:16 发稿、同花顺/网易转载 | 新智元网易号版 17:16:28、新浪财经转载 17:16 属实；同花顺转载未找到 |
| 创始人 X 帖被广泛转引 | 帖子本身 387,707 次浏览；转引证据：新智元参考资料、vibecafe“原推讨论”链接、@chetaslua 转述 |
| HN 2 分 / 1 评（11-13） | 属实，发帖 2026-11-14 00:12:10 北京（= 11-13 16:12:10Z）；评论 1 条 |
| 仓库与数据集地址 | 属实（GitHub、Hugging Face 均可开） |

## 七、截图表（`images/`，B 组区间 13–24，另加 25–29；全部 opencli 浏览器或 `barking shot` 裁切，未改像素）

| 文件 | 内容 | 备注 |
|---|---|---|
| 13-page-title.png | 官方页标题区 | 1.3 倍缩放，1400 px 宽；已裁掉页面外导航以下内容 |
| 14-method-run.png | “One run, start to finish” 六格（180 秒、超时即停、两个 LLM 评审） | 1.3 倍 |
| 15-leaderboard-intervals.png | “Result recap” 榜单图，含区间条、图例、轴 | 1.3 倍 |
| 16-not-separate-text.png | “does not separate them”、27 与 18 次超时、“zero column” 段 | 1.3 倍 |
| 17-silence-chart.png | “When to speak and when to stay silent” 双栏图 + 图注 “Message delivery only” | 1.3 倍 |
| 18-silence-text.png | “9 times out of 10 … 6 times out of 10”、1.8× 等段 | 1.3 倍 |
| 19-results-table.png | 完整结果表（12 行、全部列、脚注） | **1.0 倍（1400 px 宽、1x）**：表在 1.3 倍时右侧列被截，所以用 1x |
| 20-timeouts-three-ways.png | “Three ways to lose points” 前两项（Running out the clock；Stepping into a settled matter） | 1.3 倍 |
| 21-cost-text.png | “The most expensive models are not the best ones” 成本段（Cost / Time / Tool calls / Always on） | 1.0 倍 |
| 22-repo-judge-prompt.png | 仓库评审提示片段 | **已被 27 取代**：第 7 项开头被裁掉半行，建议删除 |
| 23-zh-163-title-date.png | 新智元（网易号）标题、时间、来源 + 首句 | 暗色拼接底色（两段宽度不同） |
| 24-x-founder-post.png | 创始人 X 帖卡片 | X 官方嵌入页，匿名；只含卡片，视频缩略图在卡片内 |
| 25-quick-findings.png | “Quick findings” 六格 | 额外编号；顶部有一条细刻度条残边 |
| 26-limitations.png | “Limitations” 段 | 额外编号 |
| 27-repo-judge-prompt-silence-timeout.png | 仓库 `evaluate_agent_run.txt` 第 35–36 行与第 113 行（raw 视图，深色底） | 额外编号；GitHub raw 视图没有行号，行号见 B4b |
| 28-zh-163-1p8x.png | 新智元稿“多花1.8倍”段 | 额外编号 |
| 29-zh-163-16x-430x.png | 新智元稿“贵16倍”“430倍”段 | 额外编号 |

我没有截的：榜单“Score vs cost”散点图（页面有，图为交互式）、“月账单”表、示例任务（Ashland）、任务类型表（页面文本已存档）。
