# 1125 期 C 组（速览）取证结果

执行：claude-haiku-5-5（effort high）。北京时间 11-26 21:38 起取证，目标 60 分钟内交回。
范围：派工单 C1–C9。无截图（C 组不需要）。存档、摘句、热度均在本目录 `c-*` 文件中；抓取过程见 `capture-log-c.md`。

约定：时间一律北京时间（UTC+8），括号保留原时区；官方只给日期的写“官方未给时刻”。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。热度为 `barking heat` 取得的 HN 数字（取得时刻 11-26 21:42–21:43），会随时间变化，只作快照。

本组可用条目 11 条（派工要求至少 7 条）：C1–C8 各一条，C9 三条（OpenAI 蛋白质、PrismML Ternary Bonsai、Verge 报道的 NYT Photoshop 提醒）。C9 三条的发布时刻大多早于派工单写的“北京 11-25 06:00 之后”窗口，见末尾“给 Claude 的提示”。

---

## C1 OpenAI 手稿称多项式 Hirsch 猜想不成立

- 一句事实（原文）：“We disprove the polynomial Hirsch conjecture. For every sufficiently large n, we construct a bounded simple rational polytope with exactly n facets whose graph diameter grows faster than every fixed power of n.”（PDF 摘要）
- 限定词原句：Theorem 1.1 为 “dimension n^{o(1)}, and s(n)-move diameter n^{ω(1)}”；仓库 README：“produced by an unnamed internal OpenAI model”，“In our view, these manuscripts are best understood as a form of raw research data, rather than polished academic papers.”，“Not all have accompanying Lean formalizations.”，“Some of the unformalized results could have issues.”
- 一手来源：`https://github.com/openai/math/blob/main/preprints/Superpolynomial-graph-diameter-of-simple-polytopes-November-24-2026/paper.pdf`（GitHub 页面）；PDF 原件 `https://raw.githubusercontent.com/openai/math/main/preprints/Superpolynomial-graph-diameter-of-simple-polytopes-November-24-2026/paper.pdf`；仓库说明 `https://github.com/openai/math`。
- 存档：`c-c1-openai-math-paper-raw.pdf`（不入库）、`c-c1-openai-math-paper-raw.txt`；`c-c1-openai-math-readme.txt`；`c-c1-openai-contents.txt`（CONTENTS.md，列全部手稿与家族）；`c-c1-openai-lean-formalization.txt`；`c-c1-openai-lean-doc380.txt`；`c-c1-openai-hirsch-lean.txt`。
- 发布时刻：PDF 与 CONTENTS 标注 November 24, 2026（官方未给时刻）；HN 提交 11-26 01:34（北京，UTC 11-25 17:34）。
- 级别：L1（OpenAI 官方仓库，自述“unnamed internal OpenAI model”）。
- 九篇核对：CONTENTS.md 中 11-24 日期的手稿共 9 篇（含本篇、Strongly polynomial linear programming、The matrix multiplication exponent is nine-fourths 等）。
- Lean 范围：家族 380 的 Lean 文档写 “A family of simple rational polytopes disproves the polynomial Hirsch conjecture”；Hirsch.lean 的注释写 “on bounded simple rational polytopes with connected graphs”。formalization.yaml 登记了本篇。
- 热度（HN item 50644980）：167 分，45 条嵌套评论（11-26 01:34 提交，取得时 167 分）。标题 “The polynomial Hirsch conjecture is false [pdf]”。
- 状态：已找到（“九篇”“Smale 第九问题”“n^{o(1)}”均属实；“Lean 只覆盖有界”部分支持）。

## C2 蚂蚁 Ling-3.2-air 开源，AA 分数与官方接近

- 一句事实（模型卡原文）：“Ling-3.2-air … Its AutomationBench-AA score rises from 15.7% to 61%, … Overall, Ling-3.2-air scores 23 on the Artificial Analysis Intelligence Index v5.0.1.”
- 规格与许可（模型卡与 HF 元数据）：“Parameter Scale Total 124B, Activated 5.5B”；HF tags 含 `license:mit`。
- AA 页原文：“Ling-3.2-air scores 23 on the Artificial Analysis Intelligence Index, placing it well above average among comparable models (median: 5).” 页面指数名为 “Artificial Analysis Intelligence Index v5.0.2”。
- 一手来源：`https://huggingface.co/inclusionAI/Ling-3.2-air`；HF API `https://huggingface.co/api/models/inclusionAI/Ling-3.2-air`；AA 页 `https://artificialanalysis.ai/models/ling-3-2-air`。
- 存档：`c-c2-ling-hf.html` / `.txt`、`c-c2-ling-hf-api.json`、`c-c2-ling-aa.html`（约 0.997 MB，接近 1 MB 线）/ `.txt`。
- 发布时刻：HF createdAt 2026-11-23 03:55:12 UTC（北京 11-23 11:55）；lastModified 2026-11-24 08:31:31 UTC（北京 11-24 16:31）。
- 级别：模型卡与 AutomationBench 数字为厂商自报（L1）；AA 页为第三方评测（L3，仅限其自身数据）。
- 测试条件：AA 页 Ling-3.2-air 的 effort 档在渲染文本中未显示（其他模型标 max/xhigh）；模型卡写指数版本 v5.0.1，AA 页为 v5.0.2，二者不同。
- 未核：扫描称的 “AA 复现 AutomationBench-AA 60.8%” 在 AA 静态 HTML 与 opencli 渲染文本中均未找到（分项图表数据未以文字呈现）。
- 热度：HN 关键词 “Ling-3.2-air” 无命中。
- 状态：部分支持（规格、MIT、指数 23 已找到；60.8% 未找到；AA 与官方卡版本不一致属实）。

## C3 JetBrains Mellum2.1 Instruct：不带思考的编程模型

- 一句事实（模型卡原文）：“Mellum2.1 Instruct answers directly, without a chain of thought.”；“On our repository-level validation tasks, it generates about half as many tokens as Mellum2.1 Thinking.”
- 分数（模型卡表，“All values are percentages … All values are self-reported by JetBrains.”）：LiveCodeBench v6，Instruct 53.7 / Thinking 82.0（同表 Mellum2 为 40.4 / 70.5）。
- 方法限定（模型卡）：“Agentic benchmarks use the same open-source agent harness (Pi v0.73.1, shell and file tools) for every model”；非 agentic 用贪心解码，AIME 为 2025+2026 各 30 题合并。
- 规格与许可：“Number of Parameters: 12B total, 2.5B active”；“License: Apache 2.0”。
- 一手来源：`https://huggingface.co/JetBrains/Mellum2.1-12B-A2.5B-Instruct`（JetBrains 官方组织页，L1）；HF API `https://huggingface.co/api/models/JetBrains/Mellum2.1-12B-A2.5B-Instruct`。
- 存档：`c-c3-mellum-hf.html` / `.txt`、`c-c3-mellum-hf-api.json`；JetBrains 研究博客索引 `c-c3-jetbrains-blog.txt`（索引中未见本模型发布文）。
- 发布时刻：HF createdAt 2026-11-13 16:43:56 UTC（北京 11-14 00:43）；lastModified 2026-11-19 15:19:46 UTC（北京 11-19 23:19）。官方博文未找到，故无官方发布时刻。
- 热度：HN 关键词 “Mellum2” 无命中。
- 状态：部分支持（12B/2.5B、Apache 2.0、53.7/82.0 已找到；“45%”未找到，原文是 “about half”；“全部厂商自测单一 harness”需改写，原文只说 agentic 基准用同一 harness，且全部数值为厂商自报）。

## C4 OpenAI Private Inference 进入小范围测试

- 一句事实（Altman 原文，X）：“We’re now testing Private Inference, our latest offering as part of OpenAI Private Intelligence, with a small group of customers.”
- 限定词原句：“Reach out to @shekar4x if interested!”（原帖，仅为联系方式，不收）；“(Also, ZDR with Private Safety Processing enables automated safety reviews without giving OpenAI personnel access to the underlying content.)”
- 官方页原文：“Private Inference (Alpha)”；“Private Inference combines confidential computing with strict controls designed to protect your prompts and responses while the model processes them.”；图示标签 “Confidential computing, strict verifiable controls” 与 “API Preview”。
- 一手来源：`https://openai.com/business/private-intelligence`（官方页，L1）；Altman 帖 `https://x.com/sama/status/2125638790593274259`（fxtwitter JSON 存档，L2 署名）。
- 存档：`c-c4-openai-private.html` / `.txt`（官方页，正文无发布日期）；`c-c4-altman-fx.json` / `.headers`。
- 发布时刻：Altman 帖 Wed Nov 25 17:12:34 +0000（北京 11-26 01:12）。官方页未给日期时刻。
- 热度：Altman 帖取得时 likes 8975、retweets 477、replies 1012、views 1,204,735；HN item 50645072 为 3 分、0 评论（11-26 01:43）。
- 未找到：页面与 Altman 帖中没有 “independently verified”（全文 grep 无）；没有价格、支持模型或区域的文字。
- 状态：已找到（“testing … with a small group of customers” 与 “Alpha / API Preview” 属实）。注意：扫描说的“企业试点”原文没有 “enterprise”，应写“小范围客户测试”。扫描的措辞 “strict controls” 漏了原文 “strict verifiable controls”（图示）。

## C5 Amazon 与 Meta 谈 Muse 接入亚马逊商城（据 Fortune 报道）

- 一句事实（Fortune 原文）：“Amazon and Meta Platforms are in ongoing discussions over a potential deal to connect Meta’s popular Muse AI agent to the e-commerce giant’s online store and end a blockade of the shopping bot imposed by Amazon two months ago.”
- 限定词原句：“according to people familiar with the matter”；“Discussions are fluid, and it’s unclear if or how soon an agreement will be struck.”；“Meta and Amazon declined to comment.”；“Any deal between Amazon and Meta is expected to require sign-off from the CEOs of both companies.”
- 用户数原句：“As of October, more than 10 million people in the U.S. and Canada had set up the app, making it Meta’s biggest consumer AI hit to date.”（扫描的“1000 万用户完成设置”漏掉“截至 10 月”“美国和加拿大”两个限定）。
- 一手来源：Fortune 报道 `https://fortune.com/2026/11/25/exclusive-amazon-meta-talks-deal-muse-ai-shopping-agent-blockade/`（L4，署名 Sebastian Herrera）。两家公司均未公开置评，故无 L1 材料。
- 存档：`c-c5-fortune-muse.html` / `.txt`（另有重复存档 `c-c5-fortune-check.*`，见待删除清单）。
- 发布时刻：Fortune 页面 “November 25, 2026, 3:01 AM ET”（北京 11-25 16:01）。
- TechCrunch：未找到 11-25 原文（派工单指定的转载源）。仅在 digitaltoday 等二手稿中见 “TechCrunch and Fortune reported” 的提及；TechCrunch 搜索页存档 `c-c5-techcrunch-muse.*` 无有效正文（待删除）。
- 热度：HN 关键词 “Muse Amazon Meta” 无相关命中。
- 状态：已找到（Fortune L4，“据报道”写法）。扫描说的“TechCrunch 转载”未找到。

## C6 Anthropic 把 Messages API 从 Python 迁到 Rust

- 一句事实（官方博文原文）：“This allowed us to ship an API on Rust that uses about 70% less CPU per request and cuts median time to first token by about 20% without ever taking the service offline or freezing feature development.”
- 条件（图注原文，正文未重复）：“CPU and memory used per request, and median time to first token … CPU is one region, memory is all of production, and time to first token is customers’ Claude Code requests over a shorter period.”
- 测试与切换原文：“Our Rust API passed about 98% of its tests in April, yet only about a quarter of inputs matched on production traffic.”；“Changes merged to our API’s Rust code in each two-week period, from April until public traffic moved to Rust in September 2026.”
- 一手来源：`https://www.anthropic.com/research/migrating-api-to-rust`（官方博文，L1，Team 署名）。
- 存档：`c-c6-anthropic-rust.html` / `.txt`。
- 发布时刻：页面 “Published Nov 25, 2026”（官方未给时刻）。
- 热度：HN item 50646357，30 分、9 条嵌套评论（11-26 03:33 北京提交）。
- 未找到：全文没有 “99.6%”；没有“内存降 80%”（图中有内存曲线，文字未给数字）；“9 月两周 Claude Code 请求”原文为 “over a shorter period”，没有“两周”。
- 状态：已找到（70%、20%、98%、约四分之一、9 月切换均属实，附图注条件）。扫描中的“内存降 80%”与“9 月两周”与原文不符或未找到。

## C7 纽约州禁售面向儿童的 AI 聊天玩具（风控：美国州政府立法，由 Develata 决定是否收录）

- 一句事实（州长新闻稿原文）：“Governor Kathy Hochul today signed a bill establishing a five-year pause on AI chatbot toys.”；“S.9408A/A.11144B will place a five-year moratorium on the manufacturing, distribution and sale of toys in New York with embedded or integrated AI chatbots”。
- 生效与罚则（州长稿与法案原文一致）：“Since the new law takes effect 90 days after signing”；“Violators face civil penalties up to $15,000 per day for each violation that will be enforced by the Attorney General.”（法案 §1802：“civil penalties of up to fifteen thousand dollars per day for each violation”）；法案 §2：“take effect on the ninetieth day after it shall have become a law and shall expire and be deemed repealed 5 years after such effective date.”
- 年龄：法案正文未写年龄。“children’s toy”援引 15 U.S.C. §2057c(g)（法案 §1800(3)）。州长稿中的 “ages 3-12” 是消费者组织抽样测试的对象，不是法律年龄界定。扫描的“12 岁及以下”因此未在法案中核实。
- 一手来源：州长新闻稿 `https://www.governor.ny.gov/news/consumers-begin-their-holiday-shopping-season-governor-hochul-signs-five-year-ban-ai-chatbot`（派工单与 HN 提交的 URL；`governor.ny.gov` 直接抓取返回 403 Cloudflare 质询，改用 opencli 浏览器打开取得正文，L1）；参议院法案页 `https://www.nysenate.gov/legislation/bills/2025/S9408`（opencli 取得，状态 “SIGNED BY GOVERNOR”）；法案 PDF `https://legislation.nysenate.gov/pdf/bills/2025/S9408A`；众议院法案页 `https://nyassembly.gov/leg/?default_fld=&leg_video=&bn=A11144B&term=2025&Summary=Y&Text=Y`。
- 存档：`c-c7-ny-governor-opencli.txt`（州长稿正文）、`c-c7-nysenate-s9408a-opencli.txt`（参议院页）、`c-c7-nysenate-s9408a-pdf.pdf` / `.txt`（法案原文）、`c-c7-nyassembly-a11144b.html` / `.txt`。
- 发布时刻：州长稿日期 “NOVEMBER 24, 2026”（官方未给时刻）。
- 热度：HN item 50635246，38 分、3 条嵌套评论（11-25 05:49 北京提交）。
- 状态：已找到（五年、90 天、每日 1.5 万美元、签署均属实）；年龄“12 岁及以下”未找到，需按法案援引的 15 U.S.C. §2057c(g) 另核。

## C8 Scott Aaronson 博文：“HOLY SHIT … The magnitude of what’s now happening”

- 一句事实（博文原文，署名学者自述）：“I now feel in my bones, for the first time ever, that a proof of P ≠ NP is not far.”
- 对传闻的态度（原文）：“yes, I’m virtually certain, from multiple reliable sources, that the AI labs are now sitting on one or more bombshells in that area. However, until I either see details or hear directly from someone at a lab, I feel much, much less confident about the specific nature of those bombshells. I do not wish to be responsible for spreading unverified rumors.”；“some of those algorithms will break currently deployed cryptosystems”。
- 对 AI 证明的判断（原文）：“my working assumption by this point is that any proof that comes directly from OpenAI or Anthropic is correct barring information to the contrary.”
- 一手来源：`https://scottaaronson.blog/?p=10379`（L2，博主署名）。
- 存档：`c-c8-aaronson.html` / `.txt`；`c-c8-aaronson-feed.xml`（博客总 RSS，含 pubDate）。
- 发布时刻：博客页 “Wednesday, November 25th, 2026 at 1:01 pm”（原文未标时区；RSS pubDate Wed, 25 Nov 2026 19:01:13 +0000，与页面 1:01 pm 相符，故页面为 UTC-6）：北京 11-26 03:01。
- 热度：HN item 50646108，1700 分、1518 条嵌套评论（11-26 03:11 北京提交；取得时）。
- 状态：已找到（引文属实）。扫描说的“拒绝为加密学传闻背书”只有一半：原文确认“virtually certain”有 bombshell，并说“some of those algorithms will break currently deployed cryptosystems”，只是拒绝“unverified rumors”的具体细节。扫描说的“P≠NP 置信度回到 96%”在全文中不存在（与说法不符/未找到）。
- 注：派工单“不收”清单中写着“Aaronson 博文”，与 C8 表格本身矛盾。本文件按 C8 收录，请 Claude 确认。

## C9 自行补充（三条）

### C9a OpenAI：GPT-6 模型展示蛋白质设计能力

- 一句事实（官方页原文）：“In lab tests with Onava, proteins based on GPT‑6.2’s designs showed evidence of binding, or attachment, across five targets.”
- 限定词原句：“Specialized models helped supply sequences and select candidates for testing.”；“An internal research model matched or outperformed specialized models on many protein design tasks.”；论文题 “GPT‑6 as a Universal Foundation Model for Protein Biology: Beyond Specialized Models”。
- 一手来源：`https://openai.com/index/gpt-6-models-demonstrate-intrinsic-protein-design-capabilities/`（L1）。
- 存档：`c-c9-openai-protein.html` / `.txt`（另有重复 `c-c9-openai-protein-x.*`，见待删除）。
- 发布时刻：页面标注 “November 24, 2026”（官方未给时刻）；HN 提交 11-25 05:02（北京）。
- 热度：HN item 50634658，354 分、97 条评论（11-25 05:02 北京提交）。
- 状态：已找到。“为什么值得普通读者关注”：这是 OpenAI 对外宣称模型具备蛋白质设计能力；限定词要留在句中（有湿实验但只有 5 个靶点；专用模型仍参与选序列与筛选）。

### C9b PrismML：Ternary Bonsai 2.5 27B

- 一句事实（官方稿原文）：“Today, we’re introducing Ternary Bonsai 2.5 27B, an upgrade to Ternary Bonsai 2 27B with substantially stronger agentic coding, multi-turn tool use, and multi-step task execution.”；“Based on Qwen3.8 27B”。
- 限定词：“We evaluated Ternary Bonsai 2.5 27B against both Ternary Bonsai 2 27B and the full-precision Qwen 3.8 27B base model, as reported in the full whitepaper”（厂商自测，对照组为自家上一版与基座）。
- 一手来源：`https://prismml.com/news/bonsai-2-5-27b`（L1，公司官网）。
- 存档：`c-c9-prismml-bonsai.html` / `.txt`。
- 发布时刻：页面 “November 24, 2026”（官方未给时刻）；HN 提交 11-25 04:08（北京）。
- 热度：HN item 50633922，536 分、176 条评论。
- 状态：已找到（厂商自报）。

### C9c 《纽约时报》提醒摄影师暂勿使用新版 Photoshop（据 The Verge）

- 一句事实（The Verge 原文）：“The New York Times is asking freelance photographers not to use the newest version of Adobe Photoshop because features in the 2027 update use AI ‘in ways that may not be apparent to users.’”
- 限定词：“In an email seen by The Verge and posted on Threads”（邮件内容由 The Verge 看到，Threads 转贴，属 L6 转述链）；邮件称这些功能的使用“can result in metadata identifying photographs as AI-generated”。
- 一手来源：The Verge `https://www.theverge.com/ai-artificial-intelligence/1041227/the-new-york-times-warns-photographers-not-to-update-photoshop-over-ai-features`（L4）。NYT 原邮件未取得（无公开原文）。
- 存档：`c-c9-verge-nyt-photoshop.html` / `.txt`。
- 发布时刻：The Verge 页面 “Nov 24, 2026, 4:09 PM UTC”（北京 11-25 00:09）。
- 热度：HN item 50647673，466 分、388 条评论（11-26 05:18 北京提交）；同一链接另有 50642177（5 分）与 50632517（5 分）两个早期提交。
- 状态：部分支持。标题“warns … not to update”与正文“not to use the newest version”措辞不同，正文为准。

---

## 扫描说法勘误（独立复核结果）

| 扫描说法 | 核验结果 | 依据 |
|---|---|---|
| C1 “494 分 / 356 评（11-25 13:33 UTC）”为 Hirsch 帖 | 与说法不符：该数字属于 Google Cogentic 帖（HN 50642086，495 分 / 359 条，北京 11-25 21:33 提交）；Hirsch 帖（50644980）为 167 分 / 45 条，北京 11-26 01:34 提交 | `c-hn-50642086.json`、`c-hn-50644980.json` |
| C1 “同批九篇”“配套 LP 论文称回答 Smale 第九问题”“Theorem 1.1 维度为 n^o(1)” | 已核实 | `c-c1-openai-contents.txt`（11-24 九篇；家族 380）；PDF Theorem 1.1 |
| C1 “Lean 只覆盖有界简单有理多面体” | 部分支持：家族 380 文档与 Hirsch.lean 注释写法不同（前者无 “bounded”，后者有） | `c-c1-openai-lean-doc380.txt`、`c-c1-openai-hirsch-lean.txt` |
| C2 “124B 总参 / 5.5B 激活、MIT” | 已核实 | `c-c2-ling-hf.txt`、HF 元数据 |
| C2 “AA 复现 AutomationBench-AA 60.8%（官方 61%）” | 60.8% 未找到；官方卡 61% 属实（自报） | `c-c2-ling-hf.txt`；`c-c2-ling-aa.txt` 与 opencli 渲染文本 |
| C2 “AA 指数版本与官方卡不同” | 已核实：卡写 v5.0.1，AA 页为 v5.0.2 | 同上 |
| C3 “12B/2.5B 激活、Apache 2.0” | 已核实 | `c-c3-mellum-hf.txt` |
| C3 “token 比自家 Thinking 版少 45%” | 与说法不符：原文 “about half as many tokens”，且只指 repository-level validation tasks | 同上 |
| C3 “LiveCodeBench v6 82.0→53.7” | 数字已核实（Thinking 82.0，Instruct 53.7），但 “→” 是两种模式对比，不是掉分前后 | 同上 |
| C3 “全部厂商自测单一 harness” | 部分支持：只有 agentic 基准用同一 harness（Pi v0.73.1）；全部数值为厂商自报 | 同上 |
| C4 “官网标 Alpha、API Preview” | 已核实 | `c-c4-openai-private.txt` |
| C4 “confidential computing with strict controls” | 部分支持：图示原文为 “strict verifiable controls”；正文为 “strict controls” | 同上 |
| C4 “independently verified” | 未找到（全文无） | 同上；`c-c4-altman-fx.json` |
| C4 “企业试点” | 措辞不符：原文 “a small group of customers”，无 “enterprise” | `c-c4-altman-fx.json` |
| C5 “《财富》援引知情人士、两家拒绝置评” | 已核实 | `c-c5-fortune-muse.txt` |
| C5 “TechCrunch 转载” | 未找到 11-25 原文 | 见 C5 |
| C5 “1000 万用户完成设置” | 部分支持：原文为截至 10 月、美国与加拿大、“set up the app” | `c-c5-fortune-muse.txt` |
| C6 “约 70% CPU、首 token 中位降约 20%” | 已核实（图注给出 CPU 限于一个区域、延迟限于 Claude Code 请求） | `c-c6-anthropic-rust.txt` |
| C6 “CPU 数字来自单一区域、延迟来自 9 月两周 Claude Code 请求” | 区域部分已核实；“两周”与“9 月”的组合未核实，原文为 “over a shorter period”（9 月切换另有原句） | 同上 |
| C6 “4 月测试过约 98%，只匹配约 25%” | 已核实（原文 “inputs matched on production traffic”） | 同上 |
| C6 “内存降 80%” | 未找到：文字中无此数字，图中为曲线 | 同上 |
| C6 “99.6%” | 未找到 | 同上 |
| C7 “S.9408A / A.11144B、州长 11-24 签署、90 天后生效、五年、每日最高 1.5 万美元” | 已核实 | `c-c7-ny-governor-opencli.txt`、`c-c7-nysenate-s9408a-pdf.txt` |
| C7 “12 岁及以下” | 未找到于法案，法案援引 15 U.S.C. §2057c(g)，未核 | 同上 |
| C7 “HN 38 分” | 已核实 | `c-hn-50635246.json`（取得时） |
| C8 “P≠NP 置信度回到 96%” | 与说法不符（全文无该数字） | `c-c8-aaronson.txt` |
| C8 “拒绝为加密学传闻背书” | 部分支持：原文 “virtually certain … bombshells”，并称部分算法 “will break currently deployed cryptosystems”，只拒绝未经证实的细节 | 同上 |
| C8 “HN 1700 分 / 1534 评” | 分数已核实；评论数取得时为 1518（随时间变化） | `c-hn-50646108.json` |
| C9 “HN 354”（OpenAI 蛋白质） | 已核实（50634658，354 分，97 条） | `c-kw-heat.json` |
| C9 “Photoshop HN 461” | 已核实，取得时为 466 分（随时间变化） | `c-kw-heat.json` |
| C9 “PrismML HN 536” | 已核实（536 分，176 条） | `c-kw-heat.json` |

---

## 给 Claude 的提示（非事实，供选题）

1. C9 时间窗：派工单要求“北京 11-25 06:00 之后”。C9a（官方页 11-24；HN 11-25 05:02）、C9b（官方页 11-24；HN 11-25 04:08）、C9c（The Verge 11-25 00:09）都早于该时刻。daily-scan 的规则是时间窗只用来排优先级、不作排除条件，但请 Claude 确认是否收。C1、C2、C3、C6 也依赖官方日期而无时刻，照上条处理。
2. C7 为风控条目：州长与参议院一手来源齐全，但属美国州政府立法，按派工单标为风控，由 Develata 决定。
3. C8 与派工单“不收”清单矛盾（见 C8 注），请确认。
4. C3 的“官方博文”未找到，只有 HF 模型卡（JetBrains 官方组织页，L1）。
5. C2 的 AA 分项数字（AutomationBench-AA 60.8%）未找到；若正文要引用，需另找 AA 分项页或由 Claude 决定删去。
6. 可能的吠点（L6 线索，未核）：Ling-3.2-air 的 AA 指数版本与官方卡不一致（v5.0.1 对 v5.0.2）；Mellum2.1 的数字全部为厂商自报；Anthropic 的 CPU 与延迟数字各有不同的区域和流量口径；C8 的“拒绝背书”与原文并不完全一致。
7. 对读者：C7 的读者影响（若收录）：法案规定“禁止在纽约州制造、分销、持有待售、出售”含 AI 聊天功能的儿童玩具，90 天后生效，五年后失效；法案未写适用年龄。
