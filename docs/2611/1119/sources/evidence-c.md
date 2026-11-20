# 1119 期 C 组取证：Contra 设计榜（Frontier Design Benchmark）

取证时刻：2026-11-20 北京时间 22:12–22:35（本机即 UTC+8）。执行方：Sonnet 5.5（claude-sonnet-5-5）。只做取证，不下结论。工具与失败项见 `capture-log-c.md`。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别按 `EDITORIAL.md` 事实分级。

## 先说结论（读法线索，非正文）

- 榜单上只有一个匿名（stealth）模型 Gestalt；HN 标题的“两个”不是官方说法。第二行是“Human designers · Recruited from Contra”，即 Contra 请设计师做的“golden”参考稿，不是模型。HN 上署名 gajus 的用户（kingy.ai 称其为 Contra 联合创始人，未独立证实）原话：“it is not two models. Its a single model (Gestalt). The other bar on the benchmark is a human created dataset.”（L6）
- 人类（Elo 1739、胜率 74%、CI 71–78）与 Claude Fable 5.2（Elo 1713、胜率 73%、CI 69–76）的**胜率区间重叠**；页面只给胜率的 95% CI，**没有给 Elo 的区间**。官方两两对战矩阵里人类对 Fable 5.2 为 60%（对战次数未给）。
- 方法页没有说评审设计师与做参考稿的设计师是否同一批（未找到）。
- 发布推文串的第一条（北京 11-19 04:00）写的是“15 个模型、3,591 票、人类第 1（赢 76%）”，24 分钟后才出 Gestalt 帖；现榜单 17 行、4,311 票、Gestalt 第 1。

## 一、事实行（C1–C6）

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1-1 榜单第 1 为 Gestalt（stealth）：Elo 1931、胜率 90%、CI 86–92、420 票 | L3 | https://contralabs.com/frontierdesign/leaderboard | “Gestalt (stealth)#1 · ELO 1931WIN 90% · 95% CI 86–92OVERALL #1”；表格行 “01 Gestalt Gestalt · Elo 1931 / 90% / 86–92 / 420” | 26-lb-table.png、27-elo-chart.png | 取于 11-20 22:1x；页头“Updated Nov 2026 · 27 Contra jobs”（仅在 HTML/txt 中；截图因隐藏吸顶栏未含）；列名为 Win rate / 95% CI / Votes | 已找到 |
| C1-2 第 2 为“Human designers”（Recruited from Contra）：Elo 1739、74%、CI 71–78、571 票 | L3 | 同上 | “Human designersRECRUITED FROM CONTRA#2 · ELO 1739WIN 74% · 95% CI 71–78OVERALL #2”；脚注 “Human designers, recruited from Contra, are ranked with the models.” | 26-lb-table.png | 同上；Wayback 11-19 19:05 快照该行标签为 “Human golden / Contra designers”，脚注为 “The human golden is ranked with the models.”（标签后来被改，数值相同） | 已找到 |
| C1-3 第 3 Claude Fable 5.2：1713、73%、CI 69–76、683 票 | L3 | 同上 | “Claude Fable 5.2#3 · ELO 1713WIN 73% · 95% CI 69–76OVERALL #3” | 26-lb-table.png | 同上 | 已找到 |
| C1-4 全榜 17 行（逐字） | L3 | 同上 | #4 Claude Opus 5.5 1691 / 73% / 69–76 / 527；#5 GPT-6.1 Sol 1668 / 70% / 66–74 / 549；#6 Claude Sonnet 5.5 1667 / 69% / 65–73 / 440；#7 Kimi K3.1 Pro 1663 / 71% / 66–74 / 519；#8 GPT-6 Astra 1638 / 66% / 61–70 / 449；#9 Qwen3.8 Max 1434 / 39% / 35–44 / 428；#10 Kimi K3 1415 / 40% / 35–44 / 425；#11 GLM-5.3 Prime 1403 / 41% / 37–45 / 464；#12 Gemini 4 Carbon 1366 / 36% / 32–40 / 546；#13 Muse Spark 1.3 1335 / 31% / 27–35 / 503；#14 Muse Wonder 1249 / 22% / 18–25 / 486；#15 Grok 4.7 1247 / 23% / 19–26 / 559；#16 Gemini 4 Argon 1221 / 20% / 17–24 / 547；#17 DeepSeek V4 Pro 1121 / 12% / 9–15 / 506 | 26-lb-table.png（全 17 行含列名与脚注）、27-elo-chart.png | 17 行里 1 行 stealth、1 行人类、15 行模型（与线索“共 17 行”一致）。Votes 列合计 8,622 = 2 × 4,311（每票计入两个参赛者）。榜上未见 Gemini 4 Neon、GPT-6.2 | 已找到 |
| C1-5 CI 是谁的区间 | L3 / 推算 | 同上 | 列头 “Win rate / 95% CI / Votes”；Elo 列无 CI | 26-lb-table.png | **CI 跟在胜率后面，是胜率的 95% 区间；页面未给 Elo 的区间。** 推算（推断，未经官方确认）：对 Gestalt 378/423、人类 425/574 做 Wilson 区间得 86.1–91.95、70.3–77.5，与页面 86–92、71–78 吻合，即按二项比例算，未体现按评审人或题目的聚类 | 已找到（口径）；Elo 区间：未找到一手来源 |
| C1-6 样本规模 | L3 | 同上；https://contralabs.com/frontierdesign | “Ranked by Elo from Contra’s designer panel: 4,311 blind votes by 35 professional designers on 33 briefs.”；首页 “33 design briefs / 68 contributing designers / 4,311 blind votes / 35 rubric dimensions” | 26-lb-table.png（脚注） | 首页 68 “contributing designers” 与 35 位评审不同口径，页面未解释；页头 “27 Contra jobs”、画廊 “Gallery[12]” 又是另外两个数 | 已找到 |
| C2-1 Elo 怎么算、投票怎么投 | L1 | https://contralabs.com/frontierdesign/methodology | “So far we have recruited 35 designers, casting 4,311 votes over five rounds. Each designer is given pairs of designs on the same prompt, answers a general preference question, and optionally explains their choice, covering color, typography, layout, usability and prompt adherence. We turn the votes into a rating per model with a Bradley-Terry model with Elo scores.” | 28-method-golden-elo.png | 日期：方法页无日期 | 已找到 |
| C2-2 WIN% 是什么 | L1 | https://contralabs.com/frontierdesign/blog | “Gestalt (stealth) has the highest Elo, 1,928, and wins 89% of its votes (378 of 423).”；排行榜页 Blind review：“Reviewers see two outputs for the same brief side by side and pick the better one, without seeing which model made which. The sides are shuffled for every pair.” | 31-blog-gestalt-results.png | WIN% = 该参赛者在其出场的成对投票里赢的比例（对所有对手混合；对手不同会影响数值，推断）。官方未给公式页 | 部分支持（定义为从示例推出，无专页） |
| C2-3 投票人是谁；是否与参赛人类重叠 | L1 | https://contralabs.com/frontierdesign/blog；…/methodology | “judged by professional designers in the Contra Labs network”；“So far we have recruited 35 designers”；做参考稿：“we worked with professional designers on Contra to create human golden outputs for the tasks.” | 30-blog-voters.png；28-method-golden-elo.png | **评审设计师与做参考稿设计师是否同一批：页面未写（未找到）。** 博客 “And who evaluates the design? The voters behind these Elo scores were whoever showed up. Some of them design for a living; most of them don’t.” 是在批评**其他**基准的投票人（该段后面接 “Today we announce …”），并非描述本榜 | 部分支持（重叠与否：未找到一手来源） |
| C2-4 人类参考稿怎么产出 | L1 | https://contralabs.com/frontierdesign/blog | “Designers were first asked to create a moodboard of 3 design directions. We reviewed their moodboards and gave feedback. They then built the site based on the moodboard directions with tools of their choice. After that, our team gave feedback and the designers iterated on the design.”（方法页版本无最后一轮：“They then built the site based on the moodboard with tools of their choice.”） | 28-method-golden-elo.png（方法页版）；博客版见 `c-blog.txt` | 对照：模型 “BenchFlow runs an AI agent on each task in a sandbox and records its output, cost and full trajectory. We used OpenRouter as the model provider. Wherever we could, we ran each model in its own lab’s agent harness.” 工具自选的人类 vs 各家自带 harness 的模型，流程不同 | 已找到 |
| C2-5 模型的提示/档位/每题样本数 | L1 | 同上 | 页面只写 “BenchFlow runs an AI agent on each task in a sandbox …”；未写提示词、effort 档位、是否单次生成或择优 | — | 每题样本数：未给。画廊每题 “14/15 outputs”（见 C3-4） | 未找到一手来源（提示/档位/择优规则） |
| C2-6 任务集 | L1 | https://contralabs.com/frontierdesign/methodology | “We included 33 tasks as an initial set in Frontier Design. … We based 27 of them on real design jobs posted on Contra, rewritten for fictional clients.”；“we also included 6 design tasks elicited from startup partners.” | 29-method-taskset.png | 排行榜页头 “27 Contra jobs”，脚注 “33 briefs”；博客 “One of the 19 human golden designs does.”（人类参考稿共 19 份，少于 33 题） | 已找到 |
| C3-1 Gestalt 站内描述（全部） | L1 | https://contralabs.com/frontierdesign/blog | “We are excited to preview Gestalt on Frontier Design Bench, a high-quality design model currently in stealth. Professional designers evaluated Gestalt on our design task set in the same blind voting configuration, and Gestalt topped the leaderboard at 1,928 Elo, ahead of all leading frontier models at a solid margin.”；“Gestalt was also the first model rated above the human golden designs. Based on our Slop Detector, Gestalt outputs are significantly less AI-looking, almost on par with human golden designs.”；“We are working with the team to continuously evaluate its quality and provide feedback.” | 33-blog-gestalt-preview.png | 博客 1,928 / 89%（378/423）、Fable 5.2 为 1,714；实时榜 1931 / 90% / 420、Fable 5.2 为 1713，同一站两套数字（博客为发布时快照，推断）。未写开发者、发布日期、价格、试用方式 | 已找到 |
| C3-2 “two stealth models”是否官方说法 | L1/L6 | 官方 X 与站内页（见下） | 官方 X 首帖 20:00 UTC 说的是 “We evaluated 15 frontier AI models … Human designers ranked #1”；24 分钟后 “There is a new model "Gestalt" that is beating everything, including the human golden designs” | 25-hn-title.png；36-hn-gajus-clarify.png | 站内、X 均未出现“两个 stealth 模型”；该说法见于 HN 提交者自拟标题。榜单曾在后撤下另一匿名模型：未找到证据（Wayback 11-19 19:05 快照与现榜 17 行一致） | 与说法不符（“两个匿名模型”） |
| C3-3 Wayback 早期快照 | L5（快照） | http://web.archive.org/web/20261119110536/https://contralabs.com/frontierdesign/leaderboard | 同现榜数值；“Human golden#2 · ELO 1739”；脚注 “The human golden is ranked with the models.” | 无（存档 `c-wb-lb-1119.*`） | 仅用于查看“当时页面写了什么”：快照时刻 2026-11-19 11:05:36 UTC = 北京 19:05:36 | 已找到 |
| C3-4 Gestalt 的输出是否可看 | L1 | https://contralabs.com/frontierdesign/gallery | 画廊 “All 12”，12 个条目全是 “Human design · …”，每条 “14/15 outputs”；全页文本无 Gestalt | — | HN 评论 cronin101：“none of the gallery examples show the new model's output”（L6）；画廊本身的核对：已做（gallery 全文无 “Gestalt”） | 已找到 |
| C4-1 上线时间（X） | L1 | https://x.com/contraben/status/2123144312494178479 | “The AI industry has a benchmarking problem. We're letting random people on the internet decide which models are best even for capabilities like design... So we built something different. Introducing The Frontier Design Benchmark from @contralabs” | 未截（见 capture-log：X 嵌入渲染为空白） | 来源 fxtwitter JSON：created_at “Wed Nov 18 20:00:24 +0000 2026” = **北京 2026-11-19 04:00:24**；该账号简介 “building @contra and @contralabs”，Contra Labs 页顶 “Read on X” 即链到该帖 | 已找到 |
| C4-2 X 串第 2 条（15 模型/3,591 票/人类第 1） | L1 | https://x.com/contraben/status/2123144316365492278 | “We evaluated 15 frontier AI models with 35 professional designers casting 3,591 votes / The results? / Human designers ranked #1, winning 76% of their comparisons. / At the frontier, @claudeai's Fable 5.2 led the pack, followed by @OpenAI's GPT Astra, and @Kimi_Moonshot models.” | 未截 | 20:00:25 UTC = 北京 11-19 04:00:25；3,591 票 / 35 位 / 15 个模型，与现榜 4,311 票 / 17 行不同（推断：发布后榜单更新，官方未写缘由） | 已找到 |
| C4-3 Contra 官方号 | L1 | https://x.com/contra/status/2123145763182301460 | “AI is getting good at design. Designers are still better. / The Frontier Design Benchmark from @contralabs is live. Tell us how you spot AI design to win a MacBook Pro.” | 未截 | 20:06:10 UTC = 北京 11-19 04:06:10 | 已找到 |
| C4-4 Gestalt 帖 | L1 | https://x.com/contraben/status/2123150473465401719 | “Breaking🚨: There is a new model "Gestalt" that is beating everything, including the human golden designs 👀” | 未截 | 20:24:53 UTC = 北京 11-19 04:24:53；该帖 591,654 次浏览（fxtwitter，取于 11-20 22:1x） | 已找到 |
| C4-5 研究页发布日期 | L1 | https://contralabs.com/research/frontier-design-benchmark | “Benchmark·November 18, 2026·9 min read”；“Cite as Contra Labs (2026). Frontier Design: Evaluating AI on Real World Design Tasks. Contra Labs Research, November 18, 2026.” | — | 官方未给时刻；此页写 “34 design dimensions”，站内另一版（/frontierdesign/blog、方法页）写 “35” | 已找到 |
| C4-6 Contra Labs 与 BenchFlow 是什么 | L1 | https://contralabs.com/ ；https://benchflow.ai/ | Contra Labs：“A frontier human data and evaluation lab making AI better for creativity.”；“We build the eval layer for creative AI, helping AI tools … understand quality the way expert creatives do.” BenchFlow：“BenchFlow builds the environments AI agents learn in / A frontier environment lab for AI agents.” | — | 榜单题目来自 Contra 的接单平台（“real client briefs from Contra’s marketplace”）；人类参考稿与评审都取自 Contra 的设计师网络；博客结尾 “We at Contra Labs continue to evaluate frontier models and provide high-quality data to improve model design capabilities. If you are interested in real-world tasks, golden designs from professional designers, pairwise preference, and designer critiques, book a call with our partnerships team.”（Contra 向模型方出售数据/评测，原句见此）。Contra 与 Gestalt 开发方关系：官方只写 “working with the team”；HN 署名 gajus（kingy.ai 称联合创始人，L5）称 “it is not Contra model” | 已找到 |
| C4-7 数据集/投票是否公开 | L1 | https://contralabs.com/frontierdesign/dataset ；…/vote | 数据集页：“The dataset isn’t public. For research or model evaluation, email us.”；投票页：“Community votes don’t affect the benchmark leaderboard.”，需 “Sign in with Contra” | 34-dataset-access.png | 榜单、画廊、方法、Rubric、博客页无需登录可看 | 已找到 |
| 官方两两对战矩阵（本主帖“区间重叠”的直接证据） | L1 | https://frontierdesign.contralabs.com/frontierdesign/assets/blog/fdb-04-head-to-head.webp （博客内图，标题 “Top-group models won 90% of votes against the second group”） | 行 Human golden：对 Gestalt 29、Fable 5.2 60、Opus 5.5 72、GPT-6.1 Sol 63、Sonnet 5.5 69、Kimi K3.1 Pro 68、GPT-6 Astra 66；行 Gestalt：对 Human golden 71、Fable 5.2 78、Opus 5.5 81 | 37-h2h-matrix.png（官方图原文件按比例缩到 1393×1400，非页面截图：该图在页面里懒加载，匿名渲染截图为空白，故用官方图原文件，见 capture-log） | 单元格为行方对列方的胜率（%），**各格对战次数未给**；博客另写 “When designers saw a Gestalt site next to a golden design, they picked Gestalt 20 times out of 28.”（71%，与矩阵 71 吻合） | 已找到 |
| Contra 自己的 rubric 与人类排名 | L1 | https://contralabs.com/frontierdesign/blog ；https://contralabs.com/frontierdesign/rubric | “They score 73.6 on the rubric, sixth of 16, behind the two GPT models (78.7 and 79.4) and the three Claude models (79.3 to 83.8).”；“Designers saw the golden designs with those mistakes and still chose them in 74% of votes (425 of 574). The rubric counts the kind of mistakes all designers should avoid, but it doesn’t measure “taste”, which is what the designers were voting on.”；Rubric 页：Claude Fable 5.2 84、Claude Opus 5.5 84、Gestalt 83、…、“Designer reference74” | 32-blog-golden-rubric.png | 博客 425/574 与排行榜页 74%×571 不是同一份计数（榜单页 571 票），官方未解释（推断为快照不同） | 已找到 |

## 二、C5 夸大/误读实例（原站 URL、原句逐字、时刻、发布方）

| # | 发布方 / 时刻 | URL | 原句 | 与官方出入 |
|---|---|---|---|---|
| 1 | HN 提交者 cheptsov，2026-11-19 11:03:20 UTC（北京 19:03:20） | https://news.ycombinator.com/item?id=50558559 | “Two stealth models just appeared on the leaderboard, both ahead of Fable 5.2” | 榜单只有 Gestalt 一个 stealth；另一个是人类参考稿（gajus 评论更正，2026-11-19 14:41:00 UTC = 北京 22:41:00，见 `c-hn-50558559.json`） |
| 2 | 17173（来源标“快科技”），页面写 2026-11-19 22:32:00（时区未注，按本站默认北京时间） | https://news.17173.com/content/11192026/223200729.shtml | 标题 “神秘大模型Gestalt杀穿AI排行榜：断层式性能领先 最强Fabe 5.2都不够看”；正文 “正以Stealth（隐身）的模式在OpenRouter提供调用”；“全球用户公认最强的Fable 5.2得分是1715分，Gestalt直接干到了1928分” | Contra 官方页面未说 Gestalt 在 OpenRouter 上可调用（官方只写 “currently in stealth”，未给接入方式）；Fable 5.2 官方为 1713（榜）/ 1,714（博客），不是 1715；“杀穿”“最强都不够看”官方无此措辞，官方同时写人类参考稿 74% 胜率、设计师仍偏好人类（blog）。截图 35 |
| 3 | AI타임스（aitimes.com），页面 “입력 2026.11.20 17:30”（韩国时间，= 北京 16:30） | https://www.aitimes.com/news/articleView.html?idxno=217630 | 标题 “정체불명 모델 '게슈탈트', 디자인 평가서 인간 전문가 제치고 1위” | 事实基本对应榜单；标题省略“人类参考稿 1739 仍排在所有具名模型之上”与 “28 次中 20 次” 的小样本（同文正文写 20/28 “약 71%”） |
| 4 | Sulat（news.sulat.com），页面 19 Nov 2026（无时刻） | https://news.sulat.com/2026/11/19/contra-frontier-design-benchmark/ | 标题 “Contra Labs’ Frontier Design Benchmark puts stealth model Gestalt ahead of human designers” | 与榜单一致，但“ahead of human designers”要读成：Elo 领先人类参考稿 192 分，对人类参考稿 20/28 |
| 5 | Pasquale Pillitteri（pasqualepillitteri.it），19/11/2026 | https://pasqualepillitteri.it/en/news/35144/stealth-model-gestalt-beats-fable-5-2-frontier-design-benchmark | “Stealth model Gestalt beats Claude Fable 5.2 on the Frontier Design Benchmark” | 数值对；同文自己写了局限（运营方自报、28 次小样本）。该站中文版 https://pasqualepillitteri.it/zh/news/35150/… 标题 “隐身模型 Gestalt 在 Frontier Design Benchmark 击败 Fable 5.2” |

平衡的二手稿（可作为核对对照，不是夸大）：Top5Apps（https://top5apps.ai/news/what-is-gestalt-stealth-ai-design-model-contra-frontier-design-benchmark-vs-human-designers/ ，19 Nov 2026）写出“首帖与官方号说设计师更强、Gestalt 帖晚 24 分钟”，并自算 20/28 的 95% 区间约 53%–85%（我们用 Wilson 复算得 52.9%–84.7%，吻合）。

**未找到**：把本榜写成“两个神秘模型都超过 Fable 5.2”的中文媒体原文（中文侧只找到 17173 一条单模型的夸大稿）。“AI 设计已超人类”“人类设计师打败 Claude”类标题：未找到原站实例。

## 三、可能的吠点（说法 vs 证据，线索，进正文前须回一手来源核实）

1. **说法**：HN 标题 “Two stealth models … both ahead of Fable 5.2”。**证据**：榜单 17 行里只有 1 行 stealth（Gestalt）；第 2 行是 Contra 招来的人类设计师做的参考稿（榜单页 “Human designers / RECRUITED FROM CONTRA”）。（C3-2）
2. **说法**：“第二名是人类，赢了 Claude”。**证据**：Elo 1739 vs 1713，差 26；胜率 74%（CI 71–78）vs 73%（CI 69–76）重叠；页面不给 Elo 区间；官方两两矩阵里人类对 Fable 5.2 为 60%、对战次数未给。第 2 名是人类、人类排在所有具名模型之上，是榜单页自己的数。（C1、37-h2h-matrix.png）
3. **说法**：Gestalt “第一个超过人类”。**证据**：Elo 1931 vs 1739；直接对战 20/28 = 71%（官方写 “picked Gestalt 20 times out of 28”；Wilson 95% 区间约 53%–85%，为复算，推断）；Gestalt 只有 420 票，是 17 行里最少的；Contra 自己的 rubric 里 Gestalt 83 低于 Fable 5.2 与 Opus 5.5（均 84）；Gestalt 的输出不在公开画廊，开发方、接入方式、成本均未披露（“We are working with the team”）。（C3）
4. **说法**：“人类 vs AI”是同一条件的比。**证据**：人类参考稿先交 3 份 moodboard、Contra 团队反馈、自选工具、再按反馈迭代（博客原句）；模型由 BenchFlow 在沙箱里跑，各家自带 harness，提示、档位、是否择优均未写。（C2-4、C2-5）
5. **说法**：评审是“专业设计师盲评”，所以公正。**证据**：35 位评审与 4,311 票出自 Contra 的设计师网络，参考稿设计师也来自该网络，两批人是否重叠页面未写；首页又写 “68 contributing designers”；Contra 是接单平台、向模型方出售评测与数据（博客结尾原句）；榜单、数据集由其自报，数据集不公开（“By request”），社区投票不计入榜单。（C2-3、C4-6、C4-7）
6. **说法**：榜单覆盖 33 道真实委托。**证据**：榜单页头 “27 Contra jobs”、脚注 “33 briefs”、画廊 12 个条目；博客写人类参考稿共 19 份（“One of the 19 human golden designs does”）——人类并非对 33 题都有参考稿，数值之间的对应关系官方未写。（C2-6、C3-4）
7. **说法**：榜单前后一致。**证据**：发布 X 首帖（北京 11-19 04:00）写 15 个模型、3,591 票、人类第 1（76%）；24 分钟后 Gestalt 帖；博客写 Gestalt 1,928 / 89%（378/423）、Fable 5.2 1,714，现榜 1931 / 90%（420 票）、1713；排行榜页标签由 “Human golden” 改为 “Human designers · Recruited from Contra”（Wayback 11-19 19:05 快照对比）。（C4-2、C1-2、C3-3）

### 社区评论线索（L6，仅线索；HN 页面不显示评论得分，评论只记链接与作者）

- gajus（自述 “we didn’t expect anyone to beat it”；kingy.ai 称其为 Contra 联合创始人，HN 个人页未写所属，**身份未独立证实**）：https://news.ycombinator.com/item?id=50560739 “To clarify, it is not two models. Its a single model (Gestalt). The other bar on the benchmark is a human created dataset. … And no, it is not Contra model.”（截图 36）
- cronin101（质疑图库里没有 Gestalt 作品）：https://news.ycombinator.com/item?id=50560221 “Saying this when none of the gallery examples show the new model's output is the ultimate "trust me, bro" move.”（本组核对：画廊全文无 Gestalt，见 C3-4）
- akutlay（Kimi 的 slop 分高、专业评审分也高）：https://news.ycombinator.com/item?id=50560305；danielrearden 回复 https://news.ycombinator.com/item?id=50561501 “If slop score is mostly “looks like AI”, Kimi scoring high there while human designers rate it well isn’t that weird.”（对照：Rubric 页 Kimi K3.1 Pro slop 66、critique 73、Elo 1663）
- gorgmah（找不到 OpenRouter 上的 Gestalt）：https://news.ycombinator.com/item?id=50560673（对照 17173 的 OpenRouter 说法，官方未证实）
- 另有 petesergeant “clever bit of content marketing”（https://news.ycombinator.com/item?id=50560727）等，属观点，未采。

## 四、热度（barking heat，取于 11-20 22:12 北京）

- HN `50558559`：145 分 / 46 评（页面显示）；Algolia 嵌套评论 37 条；提交 2026-11-19 11:03:20 UTC（北京 19:03:20）。Claude 21:00 前后记为 142 分 / 18 评。存档 `c-heat.json`、`c-hn-50558559.json`、`c-hn.*`。
- Reddit：`old.reddit.com` 搜索被登录页拦截（`c-heat.json` errors）；`opencli reddit search` 报 SECURITY_BLOCK（403 network security）。未绕过，Reddit 主帖：未取到。
- X：@contraben Gestalt 帖 2,470 赞 / 188 转 / 591,654 浏览；首帖 2,172 赞 / 474,481 浏览（fxtwitter，22:1x）。

## 五、扫描说法勘误

- WorkBuddy 06 扫描“两个匿名模型空降榜单、均超 Fable 5.2”：**与原文不符**。只有 Gestalt 一个 stealth；第二名“Human designers”是 Contra 招募的设计师的参考稿（官方榜单页）。该说法源自 HN 提交者自拟标题。
- Claude 线索“95% CI 跟在 WIN% 后面须核实”：**已核实是胜率的 95% 区间**，Elo 无区间（C1-5）。
- 线索“HN 142 分 / 18 评”：22:12 为 145 分 / 46 评（页面显示）。
- 线索“榜单上线日期未核”：官方 X 首帖北京 11-19 04:00:24；研究页写 November 18, 2026（无时刻）。
- 线索“人类设计师一行是怎么参赛的”：已找到——Contra 请设计师做“golden”参考稿，经 moodboard、团队反馈、迭代（C2-4）；但排行榜页标签已从 “Human golden” 改成 “Human designers · Recruited from Contra”。
- 线索“评审与参赛设计师是否同一批”：**未找到**。

## 六、对读者（榜单与图库能否公开查看、数据集是否公开）

- 榜单、方法、Rubric、博客与作品图库无需登录即可看：https://contralabs.com/frontierdesign （页面可匿名获取，本组存档）。图库共 12 个条目，均为 “Human design · …”，每条 14 或 15 份输出；Gestalt 的输出不在图库里。
- 想投票需 “Sign in with Contra”；“Community votes don’t affect the benchmark leaderboard.”（https://contralabs.com/frontierdesign/vote）
- 数据集不公开：“The dataset isn’t public. For research or model evaluation, email us.”（https://contralabs.com/frontierdesign/dataset），内容含任务、人类参考稿、模型输出、评审偏好与可选理由、rubric、slop 列表。
- Gestalt 无公开使用方式：官方只写 “currently in stealth”，未给开发方、价格、接入或发布日期（博客“Gestalt (Stealth)”一节）；17173 称其在 OpenRouter 可调用，官方未证实。
- 对多数读者：这是一家做设计师评测与数据的公司自己的榜单，榜上的 Claude Fable 5.2 等具名模型读者可自行使用，但 Gestalt 不能；榜单结果不影响读者的现有使用（无开关、无价格变化）。
