# 1018 期 A 组取证：Anthropic 公共利益基金

执行方：claude-sonnet-5-5（Claude Code 子代理）。读取时刻 2026-10-19 17:08–17:35 北京时间（UTC+8；本机美东 EDT = 北京 − 12 小时）。全部时间换算为北京时间，括号内为原时区。存档文件均在 `docs/2610/1018/sources/a-*`，图片在 `docs/2610/1018/images/01–04`。所有引文逐字，未改动。

## 0. 先说结论（供 Claude 抽查；不是正文结论）

- 官方一手（L1）只有三处：公告页、官方 X 帖、公告里链接的 2 亿美元页。公告**没有**给出基准估值数值、基金治理（谁管、董事会构成）、股东批准的时间表与批准类别。
- 官方 X 帖（2026-10-19 02:07:51 北京时间，原文 18:07:51 UTC）的措辞与公告页有三处不同：未提“Subject to stockholder approval”；写作 “a new wholly owned entity dedicated entirely to the public benefit”；写作 “roughly 20% of Anthropic’s shares are committed to the public good”（公告页是 “will be dedicated” 且附 “plus any additional percentage …”）。见 A1。
- 5% 的美元价值：Anthropic 官方没有给。最近一次公开官方估值是 2026-05-28 的 Series H，后估值 965 亿 × 10 = 9650 亿美元（官方页）；该价乘 5% 约 480 亿美元，是中文/英文二手媒体的自行折算。“750–1000 亿美元”出自财联社（经网易号转载页），依据是“外界预期的上市后估值 1.5 万亿–2 万亿美元”，该预期区间对应路透社 10-15 与 9-29 的两篇报道（匿名消息人士/招股书传闻）。均非官方估值。
- 路透社原站能打开读正文（见下，有订阅弹层，未绕过）。路透社文中**没有** “stockholder approval” 字样；CNBC 与公告页有。

## 1. 逐条清单

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1 | 公告全文与日期；首段 5%、股东批准、董事会酌情、baseline | L1 | https://www.anthropic.com/news/public-benefit-fund（存档 `a-anthropic-pbf.html/.txt/.headers`）| “Today, we’re committing a portion of Anthropic’s shares to the Anthropic Public Benefit Fund, a new entity with a structure that increases the stake as the company grows. Subject to stockholder approval, 5% of Anthropic’s shares will be committed today, with an additional 2% added at the board’s discretion for every 50% increase in the company’s value above a set baseline.” | `01-pbf-announcement-lead.png`（标题、日期、首段） | 页面只显示 “Oct 18, 2026”（官方未给时刻）。页面元数据 JSON-LD：datePublished 2026-10-18T16:49:35.513Z（= 10-19 00:49:35 北京），dateModified 2026-10-18T18:05:05.000Z（= 10-19 02:05:05 北京）。官方 X 帖发于 18:07:51 UTC（见 A1b）。元数据时刻是页面自带，不是官方“发布时刻”声明；写时刻时建议写“官方页只给日期 10 月 18 日”，元数据只作内部备注 | 已找到 |
| A1b | 官方 X 帖 | L1 | https://x.com/AnthropicAI/status/2111881963657412903（存档 `a-fx-2111881963657412903.raw.json`，fxtwitter） | “We’re committing a portion of Anthropic’s shares to the Anthropic Public Benefit Fund, a new wholly owned entity dedicated entirely to the public benefit.” / “It will start off with 5% of our shares. For every 50% increase in the company’s value over a set baseline, we can contribute up to an extra 2%, subject to our board’s discretion.” / “In total, this means that roughly 20% of Anthropic’s shares are committed to the public good.” | 未截（X；按规则只存 JSON，正文不写平台名） | created_at “Sun Oct 18 18:07:51 +0000 2026” = 2026-10-19 02:07:51 北京时间；fxtwitter 读到 16,814 赞、4,638,258 浏览（17:20 前后读，会漂移）。**该帖未含 “stockholder approval”**；写作 “up to an extra 2%” | 已找到 |
| A1c | LLC 而非 501(c)(3)，不用于游说 | L1 | 同 A1 | “We’re establishing the Anthropic Public Benefit Fund as a wholly-owned LLC—rather than a 501(c)(3) or other format—to give adjustability between different forms of giving. None of the Anthropic equity held by the LLC will be used for lobbying or political giving, in the US or abroad.” | `02-pbf-llc-not-501c3.png`（含上方 2 亿美元段） | 限定词：只说“该 LLC 持有的 Anthropic 股权”不用于游说/政治捐款；没有说基金不能用于其他方向，也没有说谁控制该 LLC | 已找到 |
| A2 | “约 20%”的构成 | L1 | 同 A1 | “When Anthropic was founded, our co-founders pledged to donate 80% of their equity holdings, and since founding we have also operated an equity donation match program that allows employees to pledge equity which Anthropic matches. Approximately half of our employees participate in this program. When combined with today’s announcement, roughly 20% of Anthropic shares will be dedicated to the public good, plus any additional percentage that could be added if the company reaches specific valuation milestones.” | `03-pbf-20pct-and-governance.png`（含下方 “In the coming months” 段） | **叠加口径**：5% 基金 + 创始人承诺（80% 其持股）+ 员工配捐（约半数员工参与）。公告**没有拆分**各自占多少、也没给创始人合计持股比例；“约 15%”的拆法只见于 HN 评论（tedsanders，L6 推算，非官方）。“will be dedicated” 是将来时，基金部分尚待股东批准 | 已找到 |
| A3 | 基准估值与治理细则 | L1 | 同 A1 | “In the coming months, we will share more about the Anthropic Public Benefit Fund’s structure and governance, including the valuation milestones that may add to its stake. We will also report on our progress to date against our mission in an upcoming public benefit report.” | 同 03 图 | 公告页全文 grep：**未给 baseline 数值，未给谁管理基金、董事会构成**。路透社：“a baseline that has yet to be disclosed”（路透社原话，见 A7）。**扫描里“基金将保持独立性”（21 经济网）在公告里没有对应句**（公告页 grep “independen” 只命中相关阅读 “An update on independent evaluators”） | 未给（公告原文承诺“未来数月”披露） |
| A4 | 股东批准 | L1 + L4 | 同 A1；CNBC https://www.cnbc.com/2026/10/18/anthropic-ai-public-benefit-fund.html（存档 `a-cnbc.*`） | 公告页：“Subject to stockholder approval”。CNBC：“Anthropic said it plans to commit 5% of its shares to a new Public Benefit Fund, subject to stockholder approval.” | 01 图含该词 | **未找到**批准时间表、需哪类股东批准（普通股/优先股/特定多数）、是否已有批准动作。路透社文中无 “approval” 字样（grep 0）；The Verge 文中也无；官方 X 帖无。CNBC datePublished 2026-10-18T22:55:48Z（= 10-19 06:55 北京） | 部分支持（只有“需批准”，无时间表与类别；类别与时间表：未找到） |
| A5a | Anthropic 最新公开官方估值 | L1 | https://www.anthropic.com/news/series-h（存档 `a-series-h.*`）；TechCrunch https://techcrunch.com/2026/05/28/anthropic-raises-65-billion-nears-1t-valuation-ahead-of-ipo/（存档 `a-tc-series-h.*`，L4） | “Anthropic has raised $65 billion in Series H funding led by Altimeter Capital, Dragoneer, Greenoaks, and Sequoia Capital, valuing the company at $965 billion post-money.” | 无 | 官方页日期 May 28, 2026（页面元数据 2026-05-28T17:13:20Z = 05-29 01:13 北京）。是私募轮的“后估值”，不是上市估值，距公告约 4.7 个月。**本组没有找到更新的官方估值** | 已找到 |
| A5b | 上市估值预期（外界）| L4 | 路透 10-15 https://www.reuters.com/business/finance/investors-temper-anthropic-ipo-valuation-expectations-over-revenue-growth-2026-10-15/（存档 `a-reuters-valuation-1015.txt`，作者 Milana Vinn、Anirban Sen）；路透 9-29 https://www.reuters.com/business/anthropics-ipo-prospectus-sharpens-focus-ai-valuations-2026-09-29/（`a-reuters-prospectus-0929.txt`）| 10-15：“Some investors who attended Anthropic’s investor day on Wednesday in San Francisco discussed a valuation of around $1.5 trillion, three of the people said, a significant step down from the roughly $2 trillion valuation that investors had previously expected for the November share sale.” 9-29：“The AI lab, which could be valued at more than $2 trillion in its IPO” | 无 | 均为匿名消息人士/传闻，且 10-15 篇说明“some investors … discussed”；Anthropic 未置评、未公布上市价（“Anthropic did not immediately respond to requests for comment.”）。10-15 篇 datePublished 2026-10-15T21:00:26Z（= 10-16 05:00 北京）。9-29 篇 datePublished 2026-09-29T14:13:31Z（= 22:13 北京） | 已找到（作为 L4 传闻） |
| A5c | “750–1000 亿美元”出处 | L4（财联社，二手转载页）| https://www.163.com/dy/article/L9JGEV0005198CJN.html（网易号，页面标注 “来源: 财联社”，存档 `a-zh-163-qianyi.*`）| “若按外界预期的公司上市后估值1.5万亿-2万亿美元，这部分股份的价值为750-1000亿美元。” 标题：“Anthropic IPO前夕设立千亿公益基金：划出5%股份应对AI社会风险” | 无 | 页面标注时刻 2026-10-19 09:23:04（北京）。**这是转载页，不是 cls.cn 原站，原站未取**。假设：基金股份按“未来 IPO 估值”折算，且 5% 的分母是 IPO 时的股本；财联社该稿也未写“尚需股东批准”（grep 0）。另有：金融界/21 经济网按 9650 亿算 480 亿美元（约 3200 亿元）；新智元稿同时给出 480 亿与“真涨到2万亿那天，光这5%就值1000亿美元” | 已找到（出处），数字为媒体折算，非官方 |
| A6 | 2 亿美元劳动力承诺 | L1 | https://www.anthropic.com/news/economic-futures-research-fund-agenda（存档 `a-econ-futures.*`）| 公告里：“Anthropic has already committed $200 million to studying and addressing this issue”。链接页：“We’re committing $200 million to the fund to support ambitious external research on interventions to prepare society for the economic impacts of AI.” | 02 图含公告里该句 | 链接页日期 Jul 22, 2026（元数据 datePublished 2026-07-22T17:00Z = 07-23 01:00 北京；dateModified 2026-09-09）。**注意口径**：链接页是“外部研究资助基金”（research on interventions），公告里写 “studying and addressing”；不是直接发给受影响工人的钱 | 已找到 |
| A7a | 路透社原文 | L4 | https://www.reuters.com/business/anthropic-commit-5-stake-new-public-benefit-fund-2026-10-18/（存档 `a-reuters.txt`；`barking fetch` 返回 401，改用 opencli 浏览器读 DOM 文本）| “AI lab Anthropic, the developer of Claude, said on Sunday it will commit 5% of its shares to a new public benefit fund”；“The proposed fund would be established as a wholly owned limited liability company and would receive an additional 2% of Anthropic’s shares, at the board’s discretion, for every 50% increase in the company’s value above a baseline that has yet to be disclosed.”；“Elon Musk, who runs Anthropic rival xAI, on X said he recommended “giving the shares directly to the public.””；末尾更正注记：“Rewrites throughout and adds Musk comment in paragraph 6” | 未截（页面渲染出订阅弹层，不绕过） | 页面显示 “October 18, 2026 6:18 PM UTC”（= 10-19 02:18 北京），JSON-LD datePublished 2026-10-18T18:18:44.533Z，dateModified 2026-10-19T02:47:09.512Z（= 10-19 10:47 北京）。读取时页面有订阅弹层，我没有点击或绕过；正文是 DOM 里已加载的文本。作者署名 Abu Sultan，编辑与 Bengaluru 记者 Anusha Shah | 已找到 |
| A7b | The Verge 原文 | L4 | https://www.theverge.com/ai-artificial-intelligence/1013560/anthropic-is-committing-5-of-its-shares-to-a-public-benefit-fund（存档 `a-verge.*`）| 标题 “Anthropic is committing 5% of its shares to a ‘Public Benefit Fund’”；“It offered little information about how it would use the money, other than saying it wouldn’t fund lobbying or political donations.” | 未截 | “Posted Oct 18, 2026 at 7:56 PM UTC”（= 10-19 03:56 北京）；Terrence O’Brien。该篇很短，**标题与正文都没有 “stockholder approval”**，且 “other than saying it wouldn’t fund lobbying” 比公告的限定（仅限 LLC 持有的 Anthropic 股权）更宽 | 已找到 |
| A7c | HN 两帖 | L6 | https://news.ycombinator.com/item?id=50143755 ；https://news.ycombinator.com/item?id=50147479 | 见第 3 节 | 无 | 50143755 为官方公告页链接，57 分 / 47 评，2026-10-19 02:15:10 北京（10-18 18:15:10 UTC）。**50147479 不是第二个 Anthropic 官方帖，而是 FT 链接**（https://www.ft.com/content/3beab58d-4269-4722-b299-8fa5a93f5a52，7 分 / 2 评，10-19 09:58:56 北京），标题 “Anthropic commits 5% of shares to 'public benefit' fund”，已被标 dupe（评论 ChrisArchitect）。FT 付费墙，未取 | 已找到 |
| A7d | 马斯克评论 | L6（社交转述）；本帖为本人账号发言，原帖可链接，作“据外媒报道/马斯克称”级，正文勿写平台名 | https://x.com/elonmusk/status/2111914396490604547（存档 `a-fx-2111914396490604547.raw.json`）| “@AnthropicAI I recommend giving the shares directly to the public” | 未截 | created_at “Sun Oct 18 20:16:44 +0000 2026” = 2026-10-19 04:16:44 北京；回复对象 2111881963657412903（上面的官方帖）。fxtwitter 读到 32,528 赞、1,635,400 浏览（会漂移）。路透社已转述该句。**“This is what I will be doing” 这句只见于 StockTwits/Detroit News 等转述，我没有找到原帖（opencli 搜索 `from:elonmusk` 返回空），记为未找到一手来源** | 已找到（主句）；补充句未找到一手来源 |
| A8 | 夸大实例 | 见第 4 节 | | | `04-zh-headline-3200yi.png` | | 已找到 |
| A9 | 热度 | | `barking heat`（`a-heat.json`）| HN 50143755：57 分 / 47 评；50147479：7 分 / 2 评 | | 读取 2026-10-19 17:08 北京。Reddit（`opencli reddit search`，约 17:30 读）：r/singularity 帖 1x9a50t 1446 分 / 356 评，标题含 “pledging 20% of its shares to the public good”；r/Anthropic 帖 1x9a395 362 分 / 62 评（链接到 runtimewire.com，非官方）；r/accelerate 1x9a8je 138 分 / 40 评。`barking heat --reddit` 在上期被重定向登录页失败，本期没有用它，Reddit 数字未入 `a-heat.json` | 已找到 |

## 2. 官方公告页与官方 X 帖措辞对照（内部备注；写正文时建议以公告页为准，因为 X 帖是摘要）

| 要点 | 公告页 | 官方 X 帖 |
|---|---|---|
| 股东批准 | “Subject to stockholder approval” | 未提 |
| 实体描述 | “a new entity” / “wholly-owned LLC” | “a new wholly owned entity dedicated entirely to the public benefit” |
| 追加 2% | “an additional 2% added at the board’s discretion for every 50% increase in the company’s value above a set baseline” | “we can contribute up to an extra 2%, subject to our board’s discretion” |
| 约 20% | “When combined with today’s announcement, roughly 20% of Anthropic shares will be dedicated to the public good, plus any additional percentage …” | “In total, this means that roughly 20% of Anthropic’s shares are committed to the public good.” |

## 3. 可能的吠点（原文里有、线索摘要没提或说反的条件；每条注出处）

1. 5% 需股东批准——“Subject to stockholder approval”（公告页首段）。官方 X 帖、路透社正文、The Verge 都没有这个限定；The Verge 标题是 “is committing”。批准类别与时间表公告未给。
2. 追加的 2% 不是自动的：公告写 “added at the board’s discretion”；且基准（baseline）与“里程碑”都未披露，“In the coming months, we will share more about … including the valuation milestones”（公告页末段）。基准为何、是否是复利式的“每 50%”公告未说明。
3. 基金是 “wholly-owned LLC—rather than a 501(c)(3)”，理由写的是 “adjustability between different forms of giving”（公告页）。公告没有写谁控制该 LLC、有无独立董事、能否改变用途。（HN kennywinker 的质疑见下。）
4. 不用于游说的承诺只覆盖 “the Anthropic equity held by the LLC”（公告页），不是基金所有支出；“in the US or abroad” 是公告原话。
5. “roughly 20%” 是叠加口径（5% + 创始人 80% 承诺 + 员工配捐），“plus any additional percentage that could be added if the company reaches specific valuation milestones”（公告页）；公告没有拆分各自占比。
6. 示例用途只是 “we might explore large donations to traditional nonprofits or direct distributions to specific populations”（公告页）——“might explore”，不是承诺；劳动力部分，公告里的 2 亿美元在链接页是“外部研究资助”（Jul 22, 2026 页面）。
7. 公告写 “We are instead directing that equity to the public benefit.”，对比 “It is common practice for tech companies to offer escalating compensation awards to their executives.”（公告页倒数第 3 段）——这是公司自己的论述。HN 上有人（andsoitis）拿 Dario Amodei 2025 年薪酬报道对比，**我没有核那条数字，只作线索**。
8. 5% 的美元价值没有官方数；最近官方估值 9650 亿美元（2026-05-28 Series H），IPO 估值预期在路透社匿名消息里从约 2 万亿降到约 1.5 万亿（10-15）。

### 社区质疑线索（L6，仅线索；HN 评论没有公开分数，Algolia 与 HN 页面都不显示评论分，我无法给得分；Reddit 为读取时数字）

- “仍由公司全资控制”类：HN kennywinker https://news.ycombinator.com/item?id=50144886 ——“this sounds like it's going to an LLC that is owned and controlled by anthropic. Afaik there's no formal constraint preventing them from spending 100% of this money on cool sports cars for anthropic execs. Even if it was committed to being spent on the "public good", that's the public good as decided by anthropic, not the public.”
- 回应：HN tedsanders https://news.ycombinator.com/item?id=50145029 ——“If they spend the money on cars for execs, I assume they'd open themselves up to lawsuits, hurt their image, and damage employee morale.”（推测，非来源）
- 比例设计：HN jfckenfishf https://news.ycombinator.com/item?id=50144655 ——“A 2% for every 50% increase is just insulting. It might have made sense when they were valued at <100B. At current 2T valuation ...”（观点；其“2T”估值是传闻口径）
- 估值挂钩：HN herunan https://news.ycombinator.com/item?id=50144924 ——“It feels disingenuous when a company attaches to market valuation the amount of public good they do.”
- 折算：HN tedsanders https://news.ycombinator.com/item?id=50144795 ——“5% of $1.5T is $75B (and 7% of $2.25T is $156B). When added to the ~15% already pledged by founders and employees, that's roughly $300B …”（作者自己的算术与假设估值，非官方）
- 公关/形象：HN novaRom https://news.ycombinator.com/item?id=50144482；CNBC 引用 IPO 顾问 Lise Buyer（Class V Group 合伙人）：“beneficial from an image perspective”（CNBC 页，L4 引述；见 `a-cnbc.txt` 行 255）。
- 要求公众有发言权：HN verdverm https://news.ycombinator.com/item?id=50147801（50147479 帖下）。
- Reddit r/singularity 帖 1x9a50t：一条 15 分评论（bartnet）称 “at whose discretion and to whose benefit the funds will go will be a source of contention”，并拿它与一项把 AI 公司股票收归公有的议案做比（评论里的链接来源未核）。仅线索。

## 4. 夸大说法实例

| 序 | 发布方 / 时间 | URL | 标题或原句（逐字） | 与一手来源的差异 |
|---|---|---|---|---|
| 1 | 新智元（凤凰网转载页）2026-10-19 08:24:11 北京 | https://tech.ifeng.com/c/8xLUfYk3HXK（存档 `a-zh-ifeng-3200.*`）；网易转载 https://www.163.com/dy/article/L9JCCUTJ0511ABV6.html；投资界 https://news.pedaily.cn/202610/570434.shtml（10-19 10:40） | 标题：“Anthropic官宣：要捐3200亿分给普通人”；首句：“近500亿美元，Claude要撒钱了！”；“按5月那轮9650亿美元的估值，这5%值480亿美元（约3200亿）。” | 公告没有“捐”“分给普通人”：写的是承诺股份给一家 Anthropic 全资 LLC，用途 “might explore … direct distributions to specific populations”，且需股东批准；3200 亿元是媒体用 5 月估值与汇率折算。同稿自己后文写“基金发放对象仅为特定人群”（pedaily 摘要）。图：`04-zh-headline-3200yi.png` |
| 2 | 金融界（21 经济网转载）2026-10-19 09:39 | https://www.21jingji.com/article/20261019/herald/dd51ecc353272a4c0f998378cc268c47.html（存档 `a-zh-21jingji-3200.*`）| 标题：“刚刚，Anthropic官宣将拿出3200亿“分给普通人”，马斯克：建议把这些股份直接给公众”；“公司强调，基金将保持独立性，专注于AI带来的社会经济影响议题。” | 标题同上；“强调基金将保持独立性”在公告页无对应句；正文有“该计划尚需获得股东投票批准”。 |
| 3 | 财联社（网易号转载页，标注来源财联社）2026-10-19 09:23:04 | https://www.163.com/dy/article/L9JGEV0005198CJN.html（存档 `a-zh-163-qianyi.*`）| 标题：“Anthropic IPO前夕设立千亿公益基金：划出5%股份应对AI社会风险”；“若按外界预期的公司上市后估值1.5万亿-2万亿美元，这部分股份的价值为750-1000亿美元。” | 标题把假设估值下的折算写成“千亿公益基金”；“设立”与“划出”忽略“尚需股东批准”（正文也未提）；“IPO前夕”是媒体对时间点的描述（CNBC 引 Lise Buyer 说法与之相近，路透社写 “planning one of the biggest IPOs”），官方未说基金与 IPO 的关系。 |
| 4 | Reddit r/singularity，用户帖，约 10-19 02:14 北京 | https://www.reddit.com/r/singularity/comments/1x9a50t/anthropic_creates_a_public_benefit_fund_pledging/ | “Anthropic creates a public benefit fund, pledging 20% of its shares to the public good, with another 2% potentially added for every 50% increase in its valuation” | 基金本身是 5%；“20%” 是叠加口径（创始人承诺与员工配捐）；英文社区实例，L6 |
| 5 | KuCoin 快讯（转载/改写）2026-10-18T19:52:53Z（= 10-19 03:52 北京）| https://www.kucoin.com/news/flash/anthropic-pledges-48-billion-in-stock-to-public-benefit-fund-ahead-of-ipo（存档 `a-en-kucoin-48b.*`）| 标题 “Anthropic Pledges $48 Billion in Stock to Public Benefit Fund Ahead of IPO”；“The plan needs stockholder approval weeks before Anthropic’s expected November listing.” | 标题的 $48B 是站方用 $965B 估值折算，非 Anthropic 数字；正文倒是写了需批准。二手/聚合站（L5），仅作标题实例 |

（未找到：IT 之家、腾讯新闻、金融界正文中以“千亿美元”为标题的原页——扫描称的 IT 之家稿我只取到新浪转载页 `a-zh-sina-1`，其内容为中性；“千亿”标题实例只找到上面第 3 条。）

## 5. 扫描说法勘误

- “HN `50147479`（第二个 Anthropic 公告帖）”：**不符**。50147479 是 FT 文章链接帖（7 分 / 2 评，已标 dupe），不是官方公告帖。
- “WorkBuddy：中文财经端（财联社、IT之家、金融界、腾讯新闻）以‘千亿美元’‘IPO 前夕’做标题”：部分支持。我只验证了财联社（经网易号转载）的标题含“千亿”“IPO前夕”；金融界/21 经济网与新智元的标题用的是“3200亿”（人民币），不是“千亿美元”；IT 之家（新浪转载页）标题是“拟设立‘公共利益’基金，初始注入 5% 股份”，不含“千亿”；腾讯新闻我没有取到。“按 1.5–2 万亿估值折算 750–1000 亿美元”：已找到，出自财联社。
- “马斯克评论被搬运”：属实——路透社转述、财联社、新智元等。原帖已用 fxtwitter 存档。“This is what I will be doing”：**未找到一手来源**。
- “路透社 ChatGPT 扫描称 10/18 18:18 UTC”：属实（页面显示 6:18 PM UTC，元数据 18:18:44Z）；该稿之后被 “Rewrites throughout and adds Musk comment” 改写，现行文本 dateModified 为 10-19 02:47Z。
- “公告页只给日期 Oct 18, 2026”：属实（官方未给时刻）。页面元数据 datePublished 16:49:35Z、dateModified 18:05:05Z；官方 X 帖 18:07:51Z。
- “roughly 20%”：公告页措辞 “roughly 20% of Anthropic shares will be dedicated to the public good, plus any additional percentage …” 与官方 X 帖 “roughly 20% of Anthropic’s shares are committed to the public good” 不完全一致，见第 2 节。
- “2 亿美元已承诺”：公告写 “committed $200 million to studying and addressing this issue”；链接页（Jul 22, 2026）是外部研究基金，不要写成发给工人的钱。

## 6. 未完成 / 未找到

- 批准时间表与批准类别：未找到。
- baseline 数值、基金治理：公告未给（原文承诺未来数月披露）。
- Musk “This is what I will be doing” 原帖：未找到一手来源。
- FT 原文（50147479 链接）：付费墙，未取。
- 财联社 cls.cn 原站稿：未取，只有网易号转载页。
- Forbes、BeInCrypto、cn.investing：Forbes 403，BeInCrypto 与 cn.investing（fetch）Cloudflare blocked；cn.investing 页我用 opencli 浏览器读到文本（`a-zh-zhitong-3614097.txt`，页面署名 “Woofun AI”，不是智通财经原站），没有用作取证依据。
