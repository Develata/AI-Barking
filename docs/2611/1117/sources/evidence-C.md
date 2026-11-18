# 1117 期 C 组（速览）取证

执行：Claude Haiku 5.5（`claude-haiku-5-5`）。时间为北京时间（本机 UTC+8）。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。存档文件前缀 `c-`，存在 `docs/2611/1117/sources/`。日志见 `capture-log-C.md`。

## 总表

| # | 条目 | 状态 | 一手来源 | 发布时刻（北京） | 级别 |
|---|---|---|---|---|---|
| C1 | Gemini 免费版 Tasks | 已找到 | Google 官方博客 | 2026-11-18 01:00（官方 `2026-11-17T17:00:00+00:00`） | L1 |
| C2 | Google Wallet 上线美国 iOS | 已找到 | Google 官方博客 | 2026-11-18 01:00（官方 `2026-11-17T17:00:00+00:00`） | L1 |
| C3 | FrontierHarness Eval v1.1 | 已找到（厂商自家评测） | Runta 官方博客 | 官方未给时刻（页面 `November 17, 2026`） | L1，厂商自家评测 |
| C4 | Google/UCL HelpBench | 已找到 | Google 官方博客 + arXiv 论文 | 2026-11-17 15:00（官方 `2026-11-17T07:00:00Z`） | L1 |
| C5 | OpenAI 医疗行政智能体 325 项任务 | 已找到 | OpenAI 官方博客 | 官方未给时刻（页面 `November 16, 2026`） | L1 |
| C6 | Isomorphic Labs 50 亿美元 C 轮 | 已找到（“临床前”一词部分支持） | PR Newswire 公司新闻稿 | 2026-11-17 20:00（官方 `Nov 17, 2026, 07:00 ET`） | L1 |
| C7 | MAI Image 2.7 图像编辑第一 | 已找到（限定：截至 11-16、AA v2.1、差 4 Elo） | Microsoft AI 官方博客；AA 原页 | 2026-11-18 02:16（官方 `2026-11-17T18:16:36Z`） | L1；AA 原页 L3 |
| C8 | MAI Cyber 2 Flash 漏洞任务 92.3% | 已找到（厂商自报；开放范围部分支持） | Microsoft AI 官方博客 | 2026-11-18 02:15（官方 `2026-11-17T18:15:34Z`） | L1，厂商自报 |

## C1 Gemini 免费版 Tasks（代打电话、代购与付款）

- 一句事实（原文）：Google says Tasks in Gemini is "rolling out free of charge for hundreds of millions of people across the U.S., Japan, Korea, Canada, Australia, and India, using our new Gemini 4 Hydrogen model."
- 状态：已找到。
- 一手来源：https://blog.google/innovation-and-ai/products/gemini-app/tasks-in-gemini-free-new-features/ （作者 Josh Woodward，Vice President of Labs, Gemini app, and AI Studio）
- 存档：`c-gem-tasks.html`、`c-gem-tasks.txt`、`c-gem-tasks.headers`
- 发布时刻：页面 `article:published_time` 为 `2026-11-17`；JSON-LD `datePublished` 为 `2026-11-17T17:00:00+00:00`，即北京 2026-11-18 01:00。页面可见日期 `Nov 17, 2026`。
- 级别：L1（Google 官方博客）。
- 限定词原句：
  - "We’re also rolling out a set of advanced features, starting in the U.S." （正文未写明这些高级功能属于哪一档套餐）
  - "You can also set a hard spending cap upfront, and Gemini is designed to stay within your budget."
  - "Gemini is designed to ask for your confirmation or pre-approval before clicking "Buy,"" （“designed to”）
  - "We’re working with American Express (coming soon), Mastercard, and Visa to keep your purchases secure with digital tokens on supported cards."
  - 脚注 1："Calling through Gemini is not currently available in Indiana, Louisiana, Minnesota, Montana, and Nebraska."
- 未写明：付款上限是否对所有档生效；“高级功能”的套餐归属。
- 热度：HN 无相关主帖。Hacker News 查 "Gemini Tasks"、"Put Gemini to work" 两条，命中均与本功能无关（`c-hn-search-gemini-tasks.json`、`c-hn-search-put-gemini-to-work.json`）。Reddit 失败（见日志）。
- 扫描说法核对：“美日韩加澳印”与原文六地一致；“高级功能先美国”与原文 "starting in the U.S." 一致，但未写明套餐。

## C2 Google Wallet 上线美国 iOS

- 一句事实（原文）："Today, we’re bringing Google Wallet to iOS users across the U.S., ..."
- 状态：已找到。
- 一手来源：https://blog.google/products-and-platforms/platforms/google-pay/wallet-available-ios-united-states/ （作者 P.J. Linarducci，Google 消费支付 VP）
- 存档：`c-wallet-ios.html`、`c-wallet-ios.txt`、`c-wallet-ios.headers`
- 发布时刻：页面 `Nov 17, 2026`；JSON-LD `datePublished` `2026-11-17T17:00:00+00:00`，即北京 2026-11-18 01:00。
- 级别：L1。
- 限定词原句：
  - 标题 "Google Wallet is now available on iOS in the U.S."
  - "Please note that tap-to-pay functionality isn't supported on iOS at this time."
- 热度：HN 50534509 "Google Wallet is now available on iOS in the U.S."，2 分 / 0 评（提交北京 2026-11-18 01:05）。
- 扫描说法核对：“iOS 暂不支持 tap-to-pay”与原文一致。

## C3 FrontierHarness Eval v1.1（同一模型换 harness，成本排名大变）

- 一句事实（原文）："We partnered with Baseten to build the largest-scale harness leaderboard using 83B tokens across 27,280 agent runs."
- 状态：已找到。厂商自家评测：Runta 既是博客发布方，也运行了全部试验（"Each trial gets a new Runta runtime, which is deleted at the end."），不是独立第三方评测，正文需点明。
- 一手来源：https://runta.com/blog/introducing-frontierharness-eval-v1-1/ （作者 Shilin Zhu）
- 存档：`c-frontierharness.html`、`c-frontierharness.txt`、`c-frontierharness.headers`
- 发布时刻：官方未给时刻（页面 `<time datetime="2026-11-17">`，可见 `November 17, 2026`）。HN 提交时刻为北京 2026-11-17 22:53（UTC 14:53），那是 HN 提交时间，不是官方发布时刻。
- 级别：L1（厂商官方博客）+ 厂商自家评测。
- 规模与口径原句：
  - "V1.1 expands the benchmark to 22 harnesses × 5 models × 62 tasks × 4 attempts, 76× the size of v1.0."
  - "All costs are inference estimates at list prices."
  - "Cost per pass: the average cost per task divided by the pass rate."
- 限定词原句：
  - "64.5% of the list-price cost of the scored runs went to trials that did not pass: 52.7% to failures and 11.8% to timeouts."（分母是 scored runs 的标价成本，不是实际账单）
  - "Audits of the trial logs flagged 216 trials for reward hacking, and 163 of them were online lookups. These trials score 0."
  - "Codex CLI stayed first with GLM-5.3 in 99.6% of the draws. The other four winners are the most likely of several close harnesses: Oh My Pi with Muse (53%), ZCode with DeepSeek (47%), Pi with GLM Flash (42%) and Alma with Kimi-K3 (25%)."
  - "Open models, one provider. Closed models can give different results. The cache and the prices are those of Baseten, and another provider can change the cost ranks."
- 热度：
  - HN 主帖 50532191 "What is the best harness for each model?"：368 分 / 125 评（搜索接口，北京 2026-11-18 16:28 取得）；item 接口 367 分 / 124 嵌套评论。提交 2026-11-17 22:53 北京。
  - 社区评论（HN 无得分，API 不返回评论分）：Runta 创始人在 HN 自述 "The eval runs on our runtime, so weigh this accordingly."（item 50532247）；另一评论认为以 "steps per pass" 解释成本近乎同义反复，因为成本与 token 数强相关、而 token 数与步数相关（item 50533354）。链接：https://news.ycombinator.com/item?id=50532247 、https://news.ycombinator.com/item?id=50533354
- 扫描说法核对：见末尾“扫描说法勘误”。

## C4 Google/UCL HelpBench（AI 网络安全建议）

- 一句事实（原文，博客）："Our findings show that models return high-quality advice for many day-to-day cybersecurity questions—matching 82% of the advice provided by experts for the same questions. However, 1 in 10 responses contain inaccurate advice that poses a cybersecurity risk."
- 状态：已找到。两个数字口径需分开写（见下）。
- 一手来源：
  - 博客 https://blog.google/security/how-google-is-making-cybersecurity-advice-from-ai-more-helpful/
  - 论文 arXiv:2606.24819v1（"HelpBench: Assessing the Ability of LLMs to Provide Privacy, Safety, and Security Advice"，Meiklejohn（UCL）、Consolvo、Kelley、Matthews、Peddinti、Shelby、Simicich、Thomas（Google）；arXiv 标注 "23 Jun 2026"；博客称将于 NeurIPS 2026 发表）
- 存档：`c-helpbench.html`、`c-helpbench.txt`、`c-helpbench.headers`；`c-helpbench-abs.html|txt|headers`；`c-helpbench-paper.pdf`（PDF 仅本地）、`c-helpbench-paper.txt`
- 发布时刻：博客 JSON-LD `datePublished` `2026-11-17T07:00:00+00:00`，即北京 2026-11-17 15:00。
- 级别：L1（Google 官方博客与论文；与 UCL 合著）。博客同时说明 Google 把 HelpBench 用于 Gemini 4 Carbon 安全报告，并称 "Gemini 4 Carbon improves on Gemini 4 Argon and Gemini 3.1 Pro"，属厂商自家评测。
- 口径与限定（论文原句）：
  - 评分：“Each question is scored based on the total points earned by a model’s response divided by the total possible points”（博客）；论文 §4.1："the earned number of points divided by the maximum possible points"。即 82% 是 rubric 得分率，不是“与专家建议的一致率”。
  - 博客原句 "On average, models scored 82% on HelpBench, indicating generally high-quality advice."
  - 博客 "Earlier in the year, we evaluated HelpBench on 18 models from 7 different companies"（评测为今年早些时候，不含本期新模型）。
  - 论文 §5："18 models"，7 个模型家族（ChatGPT、Claude、Gemini、Grok、DeepSeek、GLM、Qwen）。
  - 论文 §5.2 "Across all models, 14% of responses scored less than 65% on our rubric—roughly 1 in 10 responses."
  - 自动评分器：论文报告其与人工评分 Pearson 0.85（factual 0.96，delivery 0.78），评分模型为 Gemini 2.5 Pro。
- 热度：HN 无相关主帖（"HelpBench" 查询只命中无关条目 50471281）。Reddit 失败。
- 扫描说法核对：见末尾。

## C5 OpenAI 医疗行政智能体（325 项任务 99% 临床安全）

- 一句事实（原文）："Overall, 99% of 325 reviewed tasks were rated safe."（前文："Clinical safety was assessed by evaluating whether inaccuracies, omissions, or actions in the model’s work could plausibly lead to patient harm."）
- 状态：已找到。
- 一手来源：https://openai.com/index/advancing-computer-use-for-healthcare-teams/（页面未给评测人员身份，只写 "Physicians rated helpfulness"）
- 存档：`c-openai-health.html`、`c-openai-health.txt`、`c-openai-health.headers`（curl 直取，HTTP 200，未走 403 路径）
- 发布时刻：官方未给时刻（页面 `November 16, 2026`；页内另有相关文章的 `dateTime`，不属本文，未采用）。
- 级别：L1（OpenAI 官方）。
- 限定词原句：
  - "We evaluated GPT‑6 Astra on tasks clinicians handle in their EHR inboxes. These tasks, shaped by physician input, require Astra to carry out multi-step workflows ..."（评测对象是 GPT-6 Astra，不是 GPT-6.2 Astra）
  - "Physicians rated helpfulness on a five-point scale, selecting “very helpful” or “extremely helpful” in 93% of 646 assessments."
  - 演示："To show how a computer-use agent can help a clinician and an authorization specialist work on the same MRI request, we built a demo using synthetic patient data."（合成数据仅用于演示）
  - 页面未说明 325 项任务的数据来源、评审人数与资质。
- 热度：HN 主帖 50535898 "Advancing computer-use agents for healthcare teams"，47 分 / 30 评（提交北京 2026-11-18 02:26）。同一 URL 另有 HN 50519201，3 分 / 0 评（2026-11-17 00:11，重复提交）。
- 社区评论（有理有据的质疑，线索 L6）：一评论称 "This clearly wasn't written or reviewed by a physician."，并质疑演示中 MRI 指征（item 50539758）；另一评论指出演示视频中有人工编辑步骤，即 AI 只是起草、人工审阅（item 50538020）。链接：https://news.ycombinator.com/item?id=50539758 、https://news.ycombinator.com/item?id=50538020
- 扫描说法核对：见末尾。

## C6 Isomorphic Labs 50 亿美元 C 轮

- 一句事实（原文）："Isomorphic Labs ... today announced a $5 Billion Series C financing round."
- 状态：已找到（融资事实）；“临床前”一词部分支持（见末尾）。
- 一手来源：PR Newswire 公司新闻稿 https://www.prnewswire.com/news-releases/isomorphic-labs-announces-5-billion-series-c-funding-302942062.html（该 URL 为页面 canonical；页内侧栏另有同题链接 302942093，未核差异）
- 存档：`c-isomorphic.html`、`c-isomorphic.txt`、`c-isomorphic.headers`
- 发布时刻：页面 "Nov 17, 2026, 07:00 ET"；页面 `<meta name="date">` `2026-11-17T12:00:00Z`，即北京 2026-11-17 20:00。（handoff 写的 "11-17 12:17 GMT" 与页面不符，见勘误。）
- 级别：L1（公司新闻稿，经 PR Newswire 分发）。
- 原句要点：
  - "The round was co-led by Greenoaks and Sequoia Capital, alongside Andreessen Horowitz (a16z) and several other new investors, with continued significant investment from Alphabet."
  - 参与方另含 "Thrive Capital, MGX, GV, CapitalG, Temasek and the UK Sovereign AI Fund."
  - 时点与临床："This momentum is yielding positive preclinical results, both for internal programs in oncology and immunology and with partners, setting the stage for clinical entry."；另 "driving our first therapeutic programs towards clinical trials"。
  - 估值：新闻稿全文未出现 valuation 或估值字样（全文检索），即“未披露估值”可写。
- 热度：未找到 PR 本身的 HN 主帖。HN 50537890 "Isomorphic Labs $5B Series C"（链接指向 CNBC），12 分 / 2 评（北京 2026-11-18 04:42）；属媒体转载，不是一手。
- 扫描说法核对：“未披露估值”与新闻稿一致；“项目处于临床前”见勘误。

## C7 MAI Image 2.7 图像编辑排行

- 一句事实（原文）："MAI Image 2.7 takes the #1 spot on Artificial Analysis image editing, outperforming GPT Image 2.6 Sunburst (max) at just one-fifth of the cost."
- 状态：已找到，但排名的限定词必须带上（见下）。
- 一手来源：Microsoft AI 博客 https://microsoft.ai/news/introducing-mai-image-2-7/
- 存档：`c-mai-image27.html`、`c-mai-image27.txt`、`c-mai-image27.headers`
- 第三方原页（L3，AA 页面仅限其自身数据）：https://artificialanalysis.ai/image/leaderboard/editing（北京 2026-11-18 16:28 取得）：`c-aa-image-edit-b.html`、`c-aa-image-edit-b.txt`、`c-aa-image-edit-b.headers`。另有错误 URL（`/text-to-image/leaderboard`，404）产物 `c-aa-image-edit.*`，已记入待删除清单。
- 发布时刻：页面可见 `Nov 17, 2026`；JSON-LD `datePublished` `2026-11-17T18:16:36+00:00`，即北京 2026-11-18 02:16（页面 `<time>` 为 `2026-11-17T13:16:36-05:00`）。
- 级别：Microsoft 官方 L1；AA 原页 L3。
- 限定词原句（脚注，Microsoft 页）：
  - "As of November 16, 2026, MAI Image 2.7 ranks #1 on AA-Image-Editing v2.1, scoring 1,215 Elo across 5,478 votes, ahead of GPT Image 2.6 Sunburst (max) by 4 Elo. Category rankings are based on Artificial Analysis’s editing-action evaluations."
  - "On Arena, MAI Image 2.7 is a top-2 lab across image generation and single image editing ..."（Arena 上是前二，不是第一）
  - 价格："Available in public preview on Microsoft Foundry, MAI Image 2.7 costs ~$35 per 1,000 images while Flash costs ~$10 per 1,000 images."
  - 页脚 "Results reflect specific model configurations and evaluations. Performance, speed, and cost comparisons vary by workload and methodology."
- AA 原页（北京 2026-11-18 16:28 取得；响应头 Date 为 15:05 北京时间，疑为缓存，数据以取得时页面为准）：
  - MAI-Image-2.7：Elo 1215，95% 区间 1201 至 1229，5,478 样本，Released Nov 2026，Pricing $34.82 /1k imgs。
  - GPT Image 2.6 Sunburst (max)：Elo 1211，95% 区间 1198 至 1224，6,554 样本，Released Nov 2026，Pricing $210.72 /1k imgs。
  - 页面 "Rankings are based on blind votes from our validated private evaluators."
- 推断（inferred，非原文）：两者 95% 区间重叠（1201–1229 与 1198–1224），4 Elo 的领先在统计上不能区分；AA 列价比值约为 34.82 / 210.72 ≈ 0.17，与原文 "one-fifth" 不完全一致，二者口径未核。
- Oct 27 首发：页面相关报道列表 "MAI Image 2.7 debuts above GPT Image 2.5 Flare on Artificial Analysis and #3 on Arena"，日期 Oct 27, 2026（相关报道链接 https://microsoft.ai/news/mai-image-2-7-debut/，未存档原页，只据列表标题与日期）。
- 热度：HN 主帖 50536193 "MAI Image 2.7: Frontier quality at a fraction of the cost"，3 分 / 0 评（北京 2026-11-18 02:44 提交）。
- 扫描说法核对：见末尾。

## C8 MAI Cyber 2 Flash（备用）

- 一句事实（原文）："On CyberGym‑E2E, a benchmark measuring real-world vulnerability discovery, MAI Cyber 2 (92.3%) beats GPT 6 Astra (91.5%) at less than 2% of the cost per task."
- 状态：已找到（厂商自报）。
- 一手来源：https://microsoft.ai/news/introducing-mai-cyber-2-flash/
- 存档：`c-mai-cyber2flash.html`、`c-mai-cyber2flash.txt`、`c-mai-cyber2flash.headers`
- 发布时刻：JSON-LD `datePublished` `2026-11-17T18:15:34+00:00`，即北京 2026-11-18 02:15（页面 `<time>` `2026-11-17T13:15:34-05:00`）。
- 级别：L1（微软官方，自测结果，非独立评测）。
- 限定词原句：
  - 对比图文字："MAI Cyber 2 Flash achieved 92.3% task success versus 91.5% for GPT‑6 Astra, using half the reasoning budget"（正文写作 "MAI Cyber 2"，对比图文字写作 "MAI Cyber 2 Flash"，两处不完全一致）。
  - 基准剔除："we found that 92 of the benchmark’s 905 tasks are based on the same open source projects and vulnerability reports as some of our training data. The benchmark’s reference solutions were never used in training. To be conservative, we excluded these 92 tasks for every model and report CyberGym‑E2E results on the remaining 813 tasks."
  - 成本口径："Measured on CyberGym‑E2E ... priced at list price ... Reflects offline evaluation cost, not customer spend"
  - 开放范围："MAI Cyber 2 Flash is coming exclusively to Microsoft security products, including Codename MDASH, where it will automatically replace the existing model in the MAI profile for eligible customers starting mid-December."
  - GPT-6 Cyber 未比："GPT‑6 Cyber is currently available only to select customers ... We plan to share comparative results in the coming weeks"
- 热度：HN 主帖 50536171 "MAI Cyber 2 Flash"，4 分 / 0 评（北京 2026-11-18 02:43 提交）。
- 扫描说法核对：见末尾。

## 找不到 / 未找到

- Reddit：所有 Reddit 查询（FrontierHarness、HelpBench）均被重定向到 old.reddit.com 登录页，返回非 JSON，未取到任何热度数据。同一来源失败两次，未再重试。
- HN 无相关主帖：C1（Gemini Tasks）、C4（HelpBench）、C6 的 PR 本身。
- AA 图像编辑榜：正确地址 `/image/leaderboard/editing` 已取到；`/text-to-image/leaderboard` 为 404（误用 URL）。
- C7 的 Oct 27 首发原页未存档，只据 Microsoft 页面的相关报道列表标题与日期。
- C4 标准差 "as high as 12%"：博客原文有，论文正文只指向 Figure 6，图内数字未读，记为未核（图）。

## 扫描说法勘误

1. C3 “14:53 UTC 前后”（handoff 线索）：该时刻是 HN 提交时刻（北京 2026-11-17 22:53），Runta 页面只给日期 November 17, 2026，官方未给时刻。
2. C3 “五个‘最优’里四个在重抽样下不稳定”：部分支持。原文是 Codex CLI 在 99.6% 的重抽样里仍第一，其余四家分别为 53%、47%、42%、25%，原文用语是 "the most likely of several close harnesses"，并未写“不稳定”。
3. C3 “HN 50532191 367 分”：取得时 item 接口为 367 分，搜索接口为 368 分，以取得时值为准（2026-11-18 16:28）。
4. C4 “约一成回答低于65%”：博客为 "1 in 10 model responses scored less than 65%"；论文正文为 "14% of responses scored less than 65%—roughly 1 in 10"。两个数字都要保留，不能只取其一。
5. C4 “82% 是细则评分不是准确率”：论文定义正确（rubric 得分率）。但博客开头用的是 "matching 82% of the advice provided by experts"，读者会理解为与专家一致的比例，正文应按论文定义写。
6. C4 “标准差最高 12%”：博客原文 "as high as 12%" 有；论文正文未出现，仅指向 Figure 6，未核（图内）。
7. C5 “646 份评审 93% 有帮助”：原文为 "very helpful" 或 "extremely helpful"（五级量表最高两档）。“演示用合成患者资料”：仅演示使用，评测任务的数据来源页面未写。
8. C6 时刻 “11-17 12:17 GMT”：与页面不符。页面为 "Nov 17, 2026, 07:00 ET"（UTC 12:00，北京 11-17 20:00）。
9. C6 “项目处于临床前”：部分支持。原文是 "positive preclinical results ... setting the stage for clinical entry" 与 "driving our first therapeutic programs towards clinical trials"；未写“项目处于临床前”这一现状判断。
10. C7 “只领先 4 Elo”：脚注原文一致。但 AA 页给出的 95% 区间（1201–1229 与 1198–1224）重叠，属推断（见 C7）。“one-fifth of the cost”与 AA 列价比值（约 0.17）不完全一致，口径未核。
11. C7 “10-27 已首发”：相关报道列表有 Oct 27, 2026 的同系列报道标题，原页未存档。
12. C8 “疑似训练污染”：原文为 "based on the same open source projects and vulnerability reports as some of our training data"，并未用“污染”或“疑似”；“为保守起见”排除 92 个任务。
13. C8 “对比旧 GPT-6 Astra”：原文为 "GPT 6 Astra"，未用“旧”字样。
14. C8 “12 月中旬向合格客户开放”：部分支持。原文为 "coming exclusively to Microsoft security products ... automatically replace the existing model in the MAI profile for eligible customers starting mid-December"，即在微软安全产品内替换既有模型，不是对外 API 开放。
15. C8 “正文 MAI Cyber 2 与对比图 MAI Cyber 2 Flash”：同一页面两种写法，正文写作时以正文为准并标注。
16. C1 “免费版 Tasks……高级功能先美国”：与原文一致；但原文未写高级功能的套餐归属，也未写付款上限是否对所有档生效。
17. C2 扫描说法：全部一致。
