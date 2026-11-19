# 1118 期 B 组取证：Gemini 4 Neon 的价格前提

执行方：Claude Sonnet 5.5（子代理）。时刻均为北京时间（UTC+8），括号保留原时区。存档均在本目录，文件名前缀 `b-`。图片在 `../images/`，编号 13–24（25 为超区间补图）。级别按 `EDITORIAL.md` L1–L7。未登录任何站点，未输入凭据。

## 0. 一眼结论（取证事实，不是正文结论）

- 谷歌博客只写“32% lower cost per task than Carbon”，没写按 Carbon 哪个价、哪个档；博客页内与方法学 PDF（6 页）都没有该 32% 的脚注或推导（逐页文本检索“cost / price / Carbon / %”无相关句）。
- AA 文章标题与摘要写“~17% lower Cost per Task”，该数是 Neon (xhigh) $6.44 对 Carbon (high) $7.76，按 Carbon 标准价 $4/$20。同一篇开头段还写：高档位下 “Neon matches Carbon (high) at less than half the Cost per Task using Carbon’s standard (non-discounted) pricing”。AA 模型页内嵌图表数据给出 Neon (high) $3.74，对 Carbon (high) $7.76。
- 定价页：Carbon “$2.00 through November 30, 2026. $4.00 starting December 1, 2026.”（输出 $10.00 / $20.00）；Neon 只有一个价 $3.00 / $15.00，无截止日。AA 的 Carbon 每任务成本一律按标准价，站内没有“按折扣价”的 Carbon 每任务成本（见 B3）。
- “32%”“17%”“less than half”“60% of the cost”四个数来源、档位、基线各不相同，下面逐条登记，不取舍。

## 1. 取证表

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1-a Neon 排名第三、61 分、xhigh | L1（厂商自述引第三方） | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-neon/ （存档 `b-blog.html/.txt`） | “Neon ranks ahead of comparable models and #3 overall with a score of 61 on the Artificial Analysis Intelligence Index at its xhigh thinking level, landing just behind larger, slower models.” | 13-google-rank-price.png | 61 分是 xhigh 档；博客发布 2026-11-18T21:00:00+00:00 = 北京 11-19 05:00（页面元数据 datePublished；dateModified 2026-11-18T22:59:17Z = 北京 11-19 06:59）；页面显示日期 “Nov 18, 2026” | 已找到 |
| B1-b 定价 | L1 | 同上 | “The model is priced at $3 per million input tokens ($0.15 per million cached input tokens, a 95% discount) and $15 per million output tokens, with a 1 million-token context window.” | 13 | 该句无计价日期、无档位 | 已找到 |
| B1-c “32% lower cost per task than Carbon” | L1 | 同上 | “At its default high thinking level, it pairs a high Artificial Analysis Intelligence Index score with a 32% lower cost per task than Carbon.”（前一句：“Neon delivers high performance at a lower cost per task across third-party benchmarks.”） | 14-google-32pct.png | 句内未说明 Carbon 取哪个档、哪个价（标准/促销）；该段无脚注；方法学 PDF 无成本口径。段内同时有 “roughly 50% to 80% lower cost per task than top competing frontier models” 与 “costing 36%+ less per task than competing models at their highest setting” | 已找到（说法本身）；32% 的口径 未找到一手来源 |
| B1-d Coding Agent Index 第一（58.6） | L1 | 同上 | “ranking #1 overall on the Artificial Analysis Coding Agent Index (58.6), with a score of 65.2% on Terminal-Bench 4.1 and 46.2% (at its default high setting) on FrontierCode 2.0.” | 16-google-coding-index.png | 同一指数 AA 文章写 59（“Neon (xhigh) scores 59 … 3 points ahead of Carbon (high, 55)”）；58.6 与 59 疑为取整，不当吠点；AA 文章写的是 Antigravity CLI 环境、xhigh 档 | 已找到 |
| B1-e 可用范围 | L1 | 同上 | “Gemini 4 Neon is rolling out today in the Gemini API in Google AI Studio, Gemini for business (via API and model dropdown), for paid Google AI Pro and Ultra subscribers in Google Antigravity, and for Pro and Ultra subscribers in the Gemini app.” 另：“For developers: Build with the Gemini API paid tier in Google AI Studio and Android Studio, if you’re a paid Google AI Pro or Ultra subscriber, you can try Neon in Google Antigravity and all Pro and Ultra subscribers can try it in AI Studio.” | 13 | | 已找到 |
| B1-f 谷歌自己的成本图 | L1 | 同上 | 图题 “AA Index v5.0.1”，横轴 “Cost per task ($)”，注 “Methodology: deepmind.google/models/evals-methodology/gemini-4-neon” | 15-google-cost-chart.png | 图上 Carbon 最高点横坐标约 $7.8、Neon High 约 $3.7、Neon Xhigh 约 $6.4（目测，与 AA 的 $7.76/$3.74/$6.44 吻合）。即谷歌自己的图用的也是 Carbon 的标准价点位 | 已找到 |
| B1-g 谷歌页内伙伴语录 “60% of the cost” | L2（第三方企业人员公开引语，经谷歌博客转载） | 同 B1-a，页面 HTML 的 quote 区（`b-blog.html`；文本抽取版不含，屏幕上桌面宽度可见） | “In our early evaluations, it matched or exceeded Gemini 4 Carbon on overall quality at 3x the speed and 60% of the cost.”——Thai Tran，AI Product Lead, Glean | 25-google-glean-quote-60pct.png（超区间） | 这是 Glean 一家的“early evaluations”，不是谷歌的测量；窄视口下卡片文字被截断，桌面宽度（1400 CSS px）完整可见；未给任务、档位、Carbon 价格口径 | 已找到 |
| B2-a AA 文章：标题、摘要、开头段 | L3 | https://artificialanalysis.ai/articles/gemini-4-neon-intelligence-gain （存档 `b-aa.html/.txt`） | 标题 “Google continues to accelerate its release cadence with Gemini 4 Neon, delivering a small Intelligence gain over Gemini 4 Carbon at a ~17% lower Cost per Task”；“Neon (xhigh) scores 61 on the Artificial Analysis Intelligence Index, 1 point ahead of Gemini 4 Carbon (high) and 3rd overall, behind GPT-6.2 Astra (max) and Claude Fable 5.2 (max). At high effort, Neon matches Carbon (high) at less than half the Cost per Task using Carbon’s standard (non-discounted) pricing.” | 17-aa-opening.png | 页面只给日期 “November 18, 2026”（元数据 2026-11-18），官方未给时刻 | 已找到 |
| B2-b AA：$6.44 / $7.76 / 17% / 15% | L3 | 同上 | “It costs $6.44 per task, ~17% less than Carbon ($7.76 at non-discounted pricing), and takes ~21 minutes per task, ~40% less than Carbon (~36 minutes). Neon uses ~15% more output tokens per task, so the lower cost comes purely from its 25% lower token prices ($3/$15 vs $4/$20 per 1M input/output tokens), and the lower time from its higher output speed (~110 vs ~57 output tokens per second)” | 18-aa-cost-17pct.png | $6.44 是 Neon **xhigh**，$7.76 是 Carbon **high**；“~15% more output tokens”与 AA 模型页的总输出 token 数不一致，见 B3 | 已找到 |
| B2-c AA：Pricing 段 | L3 | 同上 | “Pricing: $3/$15 per 1M input/output tokens, 50% above Gemini 4 Carbon’s currently discounted price of $2/$10 (which ends on November 30), and 25% below its standard $4/$20. Cached input tokens receive the same 95% discount ($0.15)” 同段上文：“Availability: Gemini 4 Neon is publicly available at launch through Google’s API and the Gemini Enterprise Agent Platform. Pro and Ultra subscribers can also access it in Antigravity and the Gemini app” | 19-aa-pricing.png | AA 本人在 Pricing 段写明 Carbon 正在折扣；但 17% 与成本图均用标准价 | 已找到 |
| B2-d AA：Coding Agent / GDP.pdf | L3 | 同上 | “Neon (xhigh) scores 59 on the Artificial Analysis Coding Agent Index, 3 points ahead of Carbon (high, 55). … Gemini 4 Neon in Antigravity has an average Cost per Task of $6.86, compared to $12.61 for Claude Opus 5.5 (max) and $15.47 for Claude Fable 5.2 (xhigh).”“In GDP.pdf, Gemini 4 Neon (high) scores 33.4%, 5 points above Carbon (high, 28.4%) and in line with the highest scores measured, while Neon (xhigh) scores 28.0%.” | 21-aa-coding-gdp.png | xhigh 的 GDP.pdf 比 high 低 5.4 分（33.4 对 28.0），AA 原文已写；Coding Agent 的成本是 Antigravity 环境下 | 已找到 |
| B2-e AA 文章图表 | L3 | 同上；图原件 `b-aa-fig1.png`–`b-aa-fig4.png`（AA CDN 原图，宽 1600，仅存档） | 图 1 上图 “Cost per Intelligence Index Task / Weighted average cost (USD) per Intelligence Index task · Segmented by token type · Lower is better”，柱：Gemini 4 Neon (high) $3.74、Neon (xhigh) $6.44、Carbon (high) $7.76、Hydrogen (xhigh) $3.16、GPT-6.2 Astra (max) $4.00、GPT-6.2 Sol (max) $1.01 | 20-aa-cost-charts.png（成本柱图 + 智能指数对成本散点，含图例；时间图在 `b-aa-fig2.png`，未入截图区） | 图里 Carbon 只有一个标签 “Gemini 4 Carbon (high)”，没有折扣价版本 | 已找到 |
| B3-a AA 模型页：Neon | L3 | https://artificialanalysis.ai/models/gemini-4-neon （`b-aa-neon.html/.txt`；渲染 `aa-neon-r`） | “Gemini 4 Neon scores 61 (xhigh), 60 (high), 55 (medium), and 44 (low)”；“283M output tokens (xhigh), 123M output tokens (high), 62.9M output tokens (medium), and 28.3M output tokens (low)”；“Default effort high”；摘要卡 Cost “#79 / 95”、“$6.44”，Verbosity “#93 / 95”、“283M”；页内图表数据（HTML 内嵌 JSON，逐值读出）Neon (xhigh) $6.444838、(high) $3.742750、(medium) $1.904556、(low) $0.745228 | 22-aa-model-pages-neon-carbon.png | 摘要卡以 xhigh 为准（“Gemini 4 Neon (Xhigh) is amongst the leading models …”）；“Intelligence #6 / 96”（模型页按档位计，文章称 “3rd overall”，口径不同：页面把每个档位当一个条目，见勘误） | 已找到 |
| B3-b AA 模型页：Carbon | L3 | https://artificialanalysis.ai/models/gemini-4-carbon （`b-aa-carbon.html/.txt`） | “Gemini 4 Carbon scores 60 (high), 58 (medium), and 54 (low)”；“205M output tokens (high), 124M output tokens (medium), and 83.9M output tokens (low)”；“Default effort medium”；价格 “In $4.00 Out $20.00 Cache discount 95%”；“$7.76”，Cost “#84 / 95”，Verbosity “#90 / 95”；页内图表数据：Carbon (high) $7.7555、(medium) $6.5858、(low) $7.1866 | 22 | 价格与成本一律标准价；全页（含 FAQ）无 “discount/promo/November 30” 字样（检索确认，仅 “Cache discount”） | 已找到 |
| B3-c AA 榜单页 | L3 | https://artificialanalysis.ai/leaderboards/models （`b-aa-leaderboards.txt`） | 榜单行：Fable 5.2 Max 65 / $26.54；GPT-6.2 Astra Max 62 / $4.00 / 13.8 分钟；Gemini 4 Neon Xhigh 61 / $6.44 / 21.5；Gemini 4 Carbon High 60 / $7.76 / 36.6；Claude Opus 5.5 Max 59 / $10.04；GPT-6.2 Sol Max 57 / $1.01 / 8.9 | （未截） | 默认视图；静态页，仅含列出的行 | 已找到 |
| B3-d “AA 官网未建 Neon 独立页” | — | https://artificialanalysis.ai/models/gemini-4-neon | 页面存在，标题 “Gemini 4 Neon: Intelligence, Performance & Price Analysis”，“Released November 2026”；AA 文章顶部有 “See model page” | 22 | | 与说法不符（扫描有误） |
| B3-e 本号推算（不是任何来源的数字） | — | — | 无 | — | 全部由上面 AA 数字做除法：见第 4 节“说法 vs 证据” | 推算，非来源 |
| B4 价格页 | L1 | https://ai.google.dev/gemini-api/docs/pricing （静态抓取为空壳，改用 opencli 渲染：`b-pricing-rendered.txt`（账号标识已替换；DOM 存档因含登录账号信息已清空）；截图用匿名 headless Chrome） | Carbon：“Input price | Not available | $2.00 through November 30, 2026. $4.00 starting December 1, 2026.”“Output price (including thinking tokens) … $10.00 through November 30, 2026. $20.00 starting December 1, 2026.”“Context caching price … $0.10 through November 30, 2026. $0.20 starting December 1, 2026.” Neon：“Input price | Not available | $3.00”“Output price … $15.00”“Context caching price … $0.15”；“Gemini 4 Neon is not available on the Free tier. Every token the model processes is counted and billed, including thinking tokens and internal tool system instructions and function declarations when using grounding.” | 23-pricing-two-models.png（深色底） | 价格页是 Standard 标签页；Argon 同样 $2/$10 至 11-30；Hydrogen $1.50/$7.50 且 Free Tier “Free of charge” | 已找到 |
| B5 二手稿 | L4/L5 | 见第 3 节 | 见第 3 节 | 24-newsbytes-60pct-headline.png | | 已找到 |
| B6 热度与社区 | L6 | HN `50552424`、`50554374`（`b-heat.json`、`b-hn-*.json`） | 见第 5 节 | — | Reddit 被登录页拦截 | 部分支持（HN 有，Reddit 无） |

## 2. 发布时刻

| 项 | 时刻（北京）| 原时区 | 备注 |
|---|---|---|---|
| 谷歌博客 | 2026-11-19 05:00 | 2026-11-18T21:00:00+00:00（datePublished） | 页面显示 “Nov 18, 2026”；最后修改 2026-11-18T22:59:17Z = 北京 11-19 06:59 |
| AA 文章 | 官方未给时刻 | “November 18, 2026” | 元数据仅日期 |
| AA 模型页 | 官方未给时刻 | “Released November 18, 2026”（FAQ） | |
| 定价页 | 官方未给时刻 | — | 2026-11-19 21:3x 北京时取到 |
| HN 50552424（Gemini 4 Neon，链谷歌博客） | 2026-11-19 05:34 | 2026-11-18T21:34:07Z | 取得 21:34 北京：895 分 / 704 评 |
| HN 50554374（micjm 博客） | 2026-11-19 08:29 | | 238 分 / 73 评 |

## 3. 二手稿与夸大实例（B5）

均只作线索（L4/L5）。“写歪”指把谷歌、AA 或伙伴的相对数改成与来源不同的说法，或不带档位与基线。

| 站点 / 发布时刻 | URL | 原句（逐字） | 与一手来源的差别 |
|---|---|---|---|
| NewsBytes（印度媒体），2026-11-19 11:19（页面未标时区；时区未核） | https://www.newsbytesapp.com/news/science/google-launches-gemini-4-neon-ai-model-check-features/story （curl 403；opencli 渲染取文，`b-m-newsbytes-opencli.txt`） | 标题 “Google launches Gemini 4 Neon: It's 3x faster, 60% cheaper”；正文 “The new model is designed to deliver advanced reasoning at a faster pace and lower cost.” | 谷歌博客页里出现的是 Glean 一家的 “60% of the cost”，“60% of the cost” 即便宜 40%，不是 “60% cheaper”；且归在 Neon 名下，不署 Glean，不写 “early evaluations”。博客正文自己的数是 32% |
| 36氪，页面元数据 2026-11-19 21:42 +08:00（疑为抓取时刻，未核） | https://www.36kr.com/p/4075845199402119 | “Neon 速度是 Carbon 的 3 倍，成本只有 Carbon 的 60%。”（另一处把同一句当引语：「Neon 在整体质量上追平甚至超过了 Carbon，速度快 3 倍，成本降低 40%。」） | 同一句 Glean 语录被写成 Neon 的事实，未署 Glean、未写 early evaluations |
| ETtoday（台湾），2026-11-19 09:09 +08:00 | https://ai.ettoday.net/news/3274652 | 标题 “Google推出Gemini 4 Neon！主打高效AI推理、程式開發能力　成本最高降低60%”；“Google 表示，在早期內部評估中，Neon 的整體品質已能媲美 Gemini 4 Carbon，但執行速度提高 3 倍，成本則降低 40%。” | 标题 “最高降低60%” 与正文 “降低40%” 自相矛盾；“Google 表示/早期內部評估” 是把 Glean 的外部评估写成谷歌内部评估 |
| 链捕手 ChainCatcher，2026-11-19T00:07:48Z = 北京 08:07 | https://www.chaincatcher.com/article/2306120 | 标题 “谷歌 Gemini 4 Neon 发布：速度 3 倍、成本 60%，世界模型 Genie 4 同步开放” | 标题不署来源，不写基线 |
| Crypto Briefing，2026-11-18T21:49:22Z = 北京 11-19 05:49 | https://cryptobriefing.com/google-deepmind-gemini-4-neon-launch/ | “According to a widely shared post on X, it also costs about 17% less per task.”“There, Neon’s cost per task comes in approximately 32% below Carbon’s.”“the figures range from about 17% to approximately 32% depending on the comparison.” | 把 AA 的 17%（xhigh 对 Carbon high 标准价）与谷歌的 32%（default high，口径未说明）合并成“看比较对象”的区间，没有写 AA 同篇的 “less than half”，也没提 Carbon 促销价 |
| 对照（写得准的）：The Decoder，2026-11-19T11:18:18Z = 北京 19:18 | https://the-decoder.com/googles-latest-ai-model-is-faster-cheaper-and-smarter-than-its-flagship-predecessor-from-just-weeks-ago/ | “That is 25 percent below Carbon's regular price of four and twenty dollars, but 50 percent above Carbon's promotional price”；“per-task costs for Neon at xhigh come in at $6.44, roughly 17 percent below Carbon's undiscounted figure of $7.76”；“At the second-highest level, "high," Neon matches Carbon's intelligence score while cutting per-task costs in half.” | 不是夸大；标题 “faster, cheaper and smarter” 仍不带条件 |
| 对照：heise，2026-11-19T13:50:00+01:00 = 北京 20:50 | https://www.heise.de/en/news/Gemini-4-Neon-takes-on-Google-s-flagship-model-11530558.html | “Neon is therefore currently 50% more expensive per token, but will be 25% cheaper than Carbon after that.”“Both cost comparisons use Carbon's regular prices, starting Dec. 1.” | 准确 |
| 对照：Neowin，2026-11-18 20:02 EST = 北京 11-19 09:02 | https://www.neowin.net/news/google-launches-gemini-4-neon-claims-its-three-times-faster-than-carbon/ （opencli 取文，`b-m-neowin-opencli.txt`；curl 被 Cloudflare 拦） | “Google also claims that Neon completes tasks three times faster at 60% of the cost of Carbon.” | 把 Glean 语录归给 Google；但同文写明 Carbon 现正半价 |
| 其他已存档（未逐条评）：itdaily（`b-m-itdaily`）、basic-tutorials（`b-m-basic`，写 “At the standard "High" thinking level, Neon costs 32 percent less per task than Carbon.” 并注 “Here are the figures Google provides”）、163（`b-m-163`，AA 17%）、mashdigi（`b-m-mashdigi`，标题 “任務成本大砍四成”，正文归 Glean） | | | |

中文标题里“便宜 17%”（163：“任务成本降17%”）来自 AA，写的是 xhigh 对 Carbon high 标准价，标题未写档位与价格口径。

## 4. 可能的吠点（说法 vs 证据；线索，写进正文前须再核）

1. 说法（谷歌博客）：“At its default high thinking level, it pairs a high Artificial Analysis Intelligence Index score with a 32% lower cost per task than Carbon.” 证据：博客与方法学 PDF 未给 32% 的分子分母、Carbon 档位与价格口径。AA 的实测数字（L3，标准价）：Neon (high) 每任务 $3.74，指数 60；Carbon (high) $7.76，指数 60；Carbon (medium)（AA 标注 Carbon 的默认档）$6.59，指数 58。用这些数除出来是 51.8% 与 43.2%，都不是 32%（本号推算）。32% 不能由公开数字复现。
2. 说法（AA 标题）：“~17% lower Cost per Task”。证据（AA 同文与模型页）：17% 比较的是 Neon **xhigh**（61 分、$6.44）与 Carbon **high**（60 分、$7.76），Carbon 取标准价 $4/$20；Pricing 段写明 Carbon 当前 “discounted price of $2/$10 (which ends on November 30)”。价格页确认 Carbon $2.00/$10.00 至 2026-11-30，之后 $4.00/$20.00。
3. 本号推算（不是来源数字；假设同样的 token 用量，Carbon 的输入、缓存、输出价格全部恰为标准价的一半，故每任务成本恰好减半）：促销期内 Carbon (high) 每任务约 $7.76÷2 = $3.88；对比 Neon (high) $3.74（低约 3.6%）、Neon (xhigh) $6.44（高约 66%）；Carbon (medium) $6.59÷2 ≈ $3.29（Neon (high) $3.74 反而高约 14%）。AA 站内没有按折扣价的 Carbon 每任务成本，也没有 Neon (high) 的文字摘要，上述需自行换算，不能写成 AA 的数。
4. 说法（谷歌）：Neon “#3 overall with a score of 61 … at its xhigh thinking level”；默认档是 high。证据：AA 模型页 Neon (high) 60 分，xhigh 61 分；GDP.pdf 子项 high 33.4%、xhigh 28.0%（xhigh 更低，AA 原文与 The Decoder 均提到）；xhigh 输出 283M token 对 high 123M（AA 模型页）。即“#3/61 分”与“32% 更便宜”分别属于 xhigh 与 high 两个档位，谷歌用同一段话先后引用。
5. 说法（AA 文）：“Neon uses ~15% more output tokens per task”。证据（AA 模型页）：Neon (xhigh) 283M 对 Carbon (high) 205M 输出 token（总量，相差约 38%，本号推算）；Neon (high) 123M 对 Carbon (high) 205M（少约 40%，推算）。两处口径（每任务加权 vs 总量）不同，AA 未解释，不取舍；若用“成本全来自单价”的说法，需注意 AA 文章与模型页两处 token 数并列。
6. 说法（Glean 语录，谷歌博客页内）：“3x the speed and 60% of the cost”。证据：来源是 Glean 的 “early evaluations”，不是谷歌，不是 AA；AA 测得时间 21.5 对 36.6 分钟（xhigh 对 high，约 41% 更短）、成本 $6.44 对 $7.76（17%）。60%、3 倍、17%、32% 四个数不可相互验证。
7. AA 模型页的 Neon 摘要写 “somewhat expensive when comparing to other models of similar price. It's also notably slow per task and very verbose”，Cost “#79 / 95”，Verbosity “#93 / 95”（Neon xhigh，AA 模型页）；Carbon “#84 / 95”、“#90 / 95”。同一页，对 Carbon 也写 “somewhat expensive … notably slow … very verbose”。
8. 榜单页（AA，默认视图）：GPT-6.2 Astra (max) 62 分 $4.00；GPT-6.2 Sol (max) 57 分 $1.01；Neon (xhigh) 61 分 $6.44（均标准价、各自档位）。“便宜”需指明对谁。

## 5. 热度与社区质疑（L6 线索）

`barking heat`（北京 11-19 21:34）：HN `50552424` “Gemini 4 Neon” 895 分 / 704 评（提交北京 11-19 05:34）；HN `50554374` “Gemini 4 Neon looks like an Opus 5.5 competitor”（micjm.com）238 分 / 73 评（提交 08:29）；HN `50552067`（AI Studio 文档链接）22 分、`50552279`（企业平台文档）13 分。Reddit 搜索被登录页拦截（`b-heat.json` 记 errors）；不绕过。**HN 评论的得分不在 Algolia JSON 与页面 HTML 中公开，下列评论均无法给出得分**，只给链接与作者。

- https://news.ycombinator.com/item?id=50555676 （aenis，北京 11-19 11:23）：“…first runs are in, and Neon is far below Carbon in terms of quality, but indeed much faster - ~4x faster, and 2x cheaper per task.” 个人跑分，样本不明。
- https://news.ycombinator.com/item?id=50556234 （svnt，11-19 12:58，回复上条）：“I am seeing comparable outcomes in some technical document processing, but the cost is not lower for me.”
- https://news.ycombinator.com/item?id=50552769 （svnt，05:57）：引 Glean 语录 “3x the speed and 60% of the cost”。
- https://news.ycombinator.com/item?id=50552543 （maplet，05:43）：“On Artificial Analysis Intelligence, it scores comparably to GPT-6.2 Sol but is significantly more expensive.”
- https://news.ycombinator.com/item?id=50552548 （onlyrealcuzzo，05:44）：“It's about 2x more expensive than Sol 6.2, but significantly better and faster.”（与 AA 数字：Sol (max) $1.01 对 Neon (xhigh) $6.44，未核该评论的口径。）
- https://news.ycombinator.com/item?id=50552645 （adithyareddy，05:48）：列价 “Gemini 4 Neon - second fastest - $3/$15 … Gemini 4 Argon - … $2/$10 (till Nov 30), $4/$20 (> Nov 30)”。
- https://news.ycombinator.com/item?id=50552739 （jeffbee，05:54）：“That looks to me like OpenAI equals or beats everything Google offers at half the price.”

“话痨”：评论区无专门质疑 verbosity 的高质量评论；AA 页 “#93 / 95” 是一手来源。

## 6. 对读者（已核实事实，每条注出处）

- 可用位置（谷歌博客，B1-e）：Gemini API（Google AI Studio）、Gemini for business、Google Antigravity（付费 Google AI Pro 与 Ultra 订阅者）、Gemini 应用（Pro 与 Ultra 订阅者）。AI Studio 文档（opencli 渲染 `b-aistudio-new-opencli.txt`）：“Gemini 4 Neon is available on the paid tier of the Gemini API (across both generateContent and the Interactions API) and with Google AI Pro and Ultra plans in Google AI Studio.”
- 免费层：不可用。价格页：“Gemini 4 Neon is not available on the Free tier.”；AI Studio 文档：“Neon is available on the paid tier only (not on the free tier).”（同页 Hydrogen 免费层 “Free of charge”，Carbon 与 Argon 免费层 “Not available”。）
- API 价格：输入 $3.00、输出 $15.00、缓存输入 $0.15（每百万 token，美元；价格页 Standard）；默认思考档 high（AI Studio 文档：“high (default)”；AA 模型页：“Default effort high”）；“Using xhigh increases latency and token use.”（AI Studio 文档）；所有 token 含思考 token 计费（价格页）。
- Carbon 促销：$2.00/$10.00（缓存 $0.10）截至 2026-11-30，2026-12-01 起 $4.00/$20.00（缓存 $0.20）（价格页）。Neon 价格页未写任何截止日期或促销。Argon 同样 $2/$10 至 11-30。
- 谁受影响：无 Pro/Ultra 订阅且不用付费 API 的人，在上述渠道里用不到 Neon（据上面的可用范围；不含第三方转售渠道，未核）。
- 第三方渠道：OpenRouter 列有 Neon（micjm 博客称其 127 tokens/s；未存档 OpenRouter 页，L6）。

## 7. 扫描说法勘误

- “AA 官网未建 Neon 独立页”：与事实不符。https://artificialanalysis.ai/models/gemini-4-neon 存在（B3-d）。
- “Carbon high 每任务 $4.90”：未找到。AA 文章与 AA 模型页均为 $7.76（标准价）。检索了 `b-aa*`、`b-blog`、`b-m-*`，仅在榜单页另一模型行见 “14.90”，与 Carbon 无关。
- “verbosity 95 个模型排第 93”：属实，AA 模型页 Neon “Verbosity #93 / 95”，输出 283M token；注意该名次是 Neon (xhigh)；同一指标 Carbon (high) 是 #90 / 95（205M token），两者相差 3 个名次。
- “博文无计价日期脚注”：属实。博客正文无计价日期、无档位脚注（全文检索 “as of / footnote / standard / discount”，只有 “a 95% discount” 一句）；方法学 PDF 无价格与成本字样。
- “X 上实测 Neon 20 分钟 $1.6 vs Sol 9 分钟 $0.53”：未找到一手来源（未去 X 检索；AA 榜单页 Sol (max) 为 $1.01、8.9 分钟，Neon (xhigh) 21.5 分钟，量纲不同）。
- “Carbon $2/$10 促销至 11-30、12-1 起 $4/$20”：属实（价格页原文）。
- “AA 写便宜约 17%”：属实，但 AA 同一篇开头段另写 “less than half the Cost per Task” （Neon high 对 Carbon high），线索摘要未提。
- “3rd overall”（谷歌与 AA 文章）与 AA 模型页 “Intelligence #6 / 96”：口径不同，页面按档位条目计名次；文章列的前两名是 Astra (max)、Fable 5.2 (max)。已记，不当吠点。
- 谷歌的 Coding Agent Index 58.6 对 AA 的 59：取整问题，不当吠点（沿用线索摘要）。

## 8. 未完成与限制

- Reddit：被登录页拦截，无数据。
- HN 评论得分：不可得。
- AA Coding Agent 页（`b-aa-agents.*`）静态页未含 Neon 条目，未取到。
- OpenRouter、Gemini 应用/Antigravity 套餐页未存档；“对读者”里的第三方渠道一条为 L6。
- Glean 语录的屏幕截图（25）依赖桌面宽度渲染；窄屏页面该卡片文字会被截断，不是页面被改。
