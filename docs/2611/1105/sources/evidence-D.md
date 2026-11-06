# 1105 期 D 组（速览）取证

取证时刻：2026-11-06 北京时间 19:33–19:35（抓取）；HN 热度取 2026-11-06 19:34（北京时间）。
级别按 `EDITORIAL.md` 事实分级：L1 官方、L3 第三方评测原始页、L4 媒体。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。
时刻：官方页只给日期时，写“官方未给时刻”；HN 时刻为条目提交时刻，不是发布时刻，已注明。

存档文件统一在 `docs/2611/1105/sources/` 下（`d-<name>.html` 原件、`.txt` 文本、`.headers`）；抓取日志 `d-fetch-log.jsonl`；热度 `d-heat.json`。本组没有截图。

---

## D1 Inkling 2 Small（Thinking Machines）

- 一句事实：Thinking Machines 发布 Inkling 2 Small，594B 总参、17B 激活；先以闭源权重在 Tinker 提供，“intend”在外部安全测试与红队完成后以 Apache 2.0 放出全部权重。
- 一手来源：https://thinkingmachines.ai/news/inkling-2-small/（L1）；存档 `d-d1-inkling-official.html` / `.txt`。
- 发布时刻：官方未给时刻（页面日期 Nov 5, 2026）。
- 级别：L1。
- 限定词原句：“Inkling 2 Small will initially be available through Tinker, with closed weights”；“We intend to make the full weights available under an Apache 2.0 license next week after the completion of external safety testing and red-teaming.”
- AA 第三方评测（L3）：https://artificialanalysis.ai/articles/thinking-machines-releases-inkling-2-small-highest-scoring-open-weights-model-outside-of-chinia-upon-weights-release-it ；存档 `d-d1-aa-article.html` / `.txt`；页面日期 November 5, 2026。原句：“Thinking Machines has released Inkling 2 Small, scoring 31 on the Artificial Analysis Intelligence Index - their plan to release its weights would make it the highest-scoring open weights model developed outside China based on current rankings”；“Inkling 2 Small is launching as a proprietary model”。口径：Artificial Analysis Intelligence Index v5.0.1，分数 31。
- 热度：HN 50386338，213 分 / 70 评，北京 11-06 08:00（提交时刻）；AA 文章 HN 50388824，2 分 / 1 评，11-06 14:18。
- 状态：部分支持（“下周放权重”是官方的“intend”，附前提；截至取证，权重未放出）。

## D2 METR 复核 Anthropic 风险报告（自动化 R&D 一节）

- 一句事实：METR 复核 Anthropic 2026 年 8 月风险报告的 R&D 一节，认为报告称“未过 2 倍加速线”的论证“overstated”，并认为该线在 7 月 15 日“plausible”已过；同时同意 7 月 15 日模型的直接灾难风险为“low”。
- 一手来源：https://metr.org/blog/2026-11-05-rd-section-anthropic-risk-report-august-2026-review/（L3，METR 复核）；存档 `d-d2-metr-review.html` / `.txt`。
- 发布时刻：官方未给时刻（页面日期 November 5, 2026）。
- 级别：L3（复核方原文）。Anthropic 原报告未在本组取证，其中的数字只据 METR 的引述。
- 限定词原句：“We believe the public risk report’s argument that the 2x Acceleration Threshold had not been crossed to be overstated, and believe it is plausible this threshold was crossed by July 15th.”；“Most relevant researchers at METR and Anthropic lean toward the threshold not having been crossed on July 15th, but we think it is plausible that it had been.”；“we agree that these risks were ‘low’ for models available at the time.”
- 1.97 的来源：“METR's note on this quote: the acceleration rate implied by [redacted]×/yr is 1.97. The 1.97 figure does not appear in the private report; this is our calculation …”。即 1.97 为 METR 自算，且出自未公开的私有版报告的数据组合。
- 总体加速倍数的各个数字（原文，分别记录，不合并）：Anthropic 复用报告数据约 1.5x，加入更多 benchmark 后 1.9x（“estimates an acceleration of 1.9x using the same set of models”）；METR 自己的 leading indicators 区间为 “between 1.3 and 2.8”。
- 热度：HN 查询 “METR Anthropic risk report” 与 “METR” 均无命中。
- 状态：部分支持。“overstated”“plausible”“多数倾向未过”“同意风险 low” 均有原文；扫描写的 “1.9x [1.5–2.8x]” 与原文不符，见勘误。

## D3 Cursor 云端代理做在线应用（Build Live Apps）

- 一句事实：Cursor 发布 Build Live Apps，可在云端代理内构建 web 应用、配置数据库与登录并部署，并以链接分享；“This capability is now available on all paid plans with cloud agents enabled.”。
- 一手来源：https://cursor.com/changelog/build-live-apps（L1）；存档 `d-d3-cursor-changelog.html` / `.txt`。文档 https://cursor.com/docs/cloud-agent/apps（L1）；存档 `d-d3-cursor-docs-apps.html` / `.txt`。
- 发布时刻：官方未给时刻（页面日期 Nov 5, 2026）。
- 级别：L1。
- 限定词原句：“This capability is now available on all paid plans with cloud agents enabled.”；“Nothing goes live until the checks pass and you explicitly publish the app.”；“On individual plans, you can also share apps publicly.”；文档：“Public sharing is disabled by default for Teams and Enterprise.”
- 热度：HN 查询 “Cursor build live apps” 无相关命中（返回的 3 条与之无关）。
- 状态：已找到。

## D4 Meta Muse 语音与表情化身

- 一句事实：Meta 称 Muse 应用的语音与表情化身“rolling out now in the Muse app in the US to start”；“Voice conversations with your Muse are coming soon to AI Glasses and Muse Charm.”。
- 一手来源：https://about.fb.com/news/2026/11/introducing-muse-ai-voice-expressive-avatars/（L1）；存档 `d-d4-meta-about.html` / `.txt`。curl 直接取到正文，未用 opencli。
- 发布时刻：官方未给时刻（页面日期 November 5, 2026）。
- 级别：L1。
- 限定词原句：“Voice and expressive avatars are rolling out now in the Muse app in the US to start.”
- 扫描说法“看不到用户”：Meta 原文中没有此句，搜不到 “can't see” 或摄像头相关的原句。TechCrunch 同日报道未取到原页（搜索只返回二手转述，且其中关于 TechCrunch 的两处引用无法在原页核对）。该说法目前为未找到一手来源。
- 热度：HN 50383465，1 分 / 0 评，11-06 03:41，标题 “Give Your Muse a Voice”。
- 状态：部分支持（“美国先行”有原文；“看不到用户”未找到一手来源）。

## D5 Linum 2.1 万美元训练文生图（P-JiT）

- 一句事实：Linum 称其 P-JiT 文生图模型训练 660M 样本，“just $21K in compute”；算式为预训练 8,139 H100 小时加 276 H100 小时，按 $2.50 每 H100 小时计，合计 $21,038。
- 一手来源：https://www.linum.ai/field-notes/pjit-21k（L1，厂商）；存档 `d-d5-linum-fieldnote.html` / `.txt`。
- 发布时刻：官方未给时刻。
- 级别：L1。
- 限定词原句：“The whole release cost about $21K of compute”；“priced at $2.50 per H100-hour”；“Research release · Apache 2.0”。
- 费用口径：只列 H100 小时乘以作者假定的 $2.50 单价，原文未列人工等其他成本。
- 权重许可：Apache 2.0，页面写明 weights 与 code 分别在 Hugging Face 与 GitHub。
- 页面另写：“The Huggingface Model Card and Github Repo were written automatically by Claude Opus 5.5.”（厂商自述，L1）。
- 热度：HN 50368208，239 分 / 22 评，11-05 00:45，“Show HN: A decent text-to-image model trained for ~$20K (Apache 2.0 Weights)”。
- 状态：已找到（费用与许可证按厂商口径）。

## D6 Pangram Image 1.0

- 一句事实：Pangram 发布 Pangram Image 1.0，称其在一个评测集上“achieves 99.96% accuracy”，该集为 100 张真实照片用 12 个前沿生图器生成的 1,200 张合成镜像图。
- 一手来源：https://www.pangram.com/blog/introducing-pangram-image-1-0（L1，厂商）；存档 `d-d6-pangram-new.html` / `.txt`。注：派工单给的 https://www.pangram.com/blog/pangram-image-1 返回 404，存档为 `d-d6-pangram.html`（404 页，留作失败记录）。
- 发布时刻：官方页日期 Nov 4, 2026，官方未给时刻。
- 级别：L1（厂商自测，不是独立第三方评测）。
- 限定词原句：“We tested our Pangram Image 1.0 model against several commercial and academic AI image detectors.”；“We created synthetic mirrors of 100 real photos using 12 frontier state the art AI image generators … for a total of 1,200 synthetic images. Our model achieves 99.96% accuracy on this evaluation set.”
- 热度：HN 50383294，113 分 / 54 评，11-06 03:27，“Pangram Image 1.0”（指向新 URL）。
- 状态：部分支持（数字与口径有原文，但为厂商自测，正文须写明口径）。

## D7 OpenClaw Multiplayer

- 一句事实：OpenClaw 博客称 Multiplayer “It’s free, it’s open source”；同文另称 “It’s MIT licensed”，但指的是 OpenClaw 本体。
- 一手来源：https://openclaw.ai/blog/shattering-silos-with-openclaw-multiplayer（L1，厂商）；存档 `d-d7-openclaw.html` / `.txt`。
- 发布时刻：官方未给时刻（页面日期 Nov 5, 2026）。
- 级别：L1。
- 限定词原句：“It’s free, it’s open source, and you’re welcome to make it your own.”（语境为 Multiplayer 这一新功能所在的产品）；“OpenClaw is developed by a nonprofit, open-source foundation. It’s MIT licensed, model-agnostic …”。
- 范围注意：原文 “MIT licensed” 的句子是对 OpenClaw 整体的描述，未逐句写明 Multiplayer 单独的许可证；正文若写 MIT，须写到 OpenClaw 整体。
- 热度：HN 查询 “OpenClaw Multiplayer” 无命中。
- 状态：部分支持（免费与开源有原文；MIT 对应范围需限定到 OpenClaw 整体）。

## D8 GPT-6 Cyber（OpenAI）

- 一句事实：OpenAI 发布 GPT-6 Cyber，“initially available through the API and Codex to a select group of approved Daybreak Red customers”，并称“We will progressively expand access to all Daybreak Red customers soon”。
- 一手来源：https://openai.com/index/introducing-gpt-6-cyber/（L1）；存档 `d-d8-openai-gpt6cyber.html` / `.txt`。curl 取到正文，未用 opencli。
- 发布时刻：官方未给时刻（页面日期 November 5, 2026）。
- 级别：L1。
- 限定词原句：“a select group of approved Daybreak Red customers”；页面另有两句：“Our researchers used GPT‑6 Cyber to develop 948 privilege escalation exploits in Linux”（导语）；“Developing exploits for 948 Linux kernel vulnerabilities. GPT‑6 Cyber reproduced 1,724 vulnerabilities and developed working exploits for 948”（正文）。ExploitGym 的数字为 “GPT‑6 Cyber scores 48.5%, up from 32.1% for GPT‑5.6 Cyber.”，口径为 OpenAI 自测。
- 与 1104 期“即将推出的网络安全模型”的关系：1104 期取证的 OpenAI 原文（`docs/2611/1104/sources/a-openai-paths-to-root.txt` 第 51 行）写 “powered by an upcoming cyber model”，未点名。D8 页面链接的正是同一篇 “a-thousand-fewer-paths-to-root”，并写明 948 个 Linux 提权 exploit 由 GPT‑6 Cyber 开发。因此可以推断 1104 所说的 “upcoming cyber model” 即 GPT‑6 Cyber。这一对应是据页内交叉链接的推断，OpenAI 未在 1104 原文中点名。
- 注意：948 个 Linux 提权是 1104 期主帖的同一事件，速览是否重复由 Claude 判断。
- 热度：HN 50381590，54 分 / 7 评，11-06 01:07，标题 “GPT‑6 Cyber: Advancing the frontier of aligned cybersecurity agents”。
- 状态：已找到（开放范围按原文为“select group”起步）。

## D9 11 月 5 日美股 AI 股下跌（只记收盘数字与报道方）

- 一句事实（L4，据媒体报道）：CNN 称“the Nasdaq slumped 5.52% and closed in correction on Thursday”，“Nvidia shares (NVDA) sank 9.3% Thursday”。
- CNN：https://www.cnn.com/2026/11/05/investing/tech-heavy-nasdaq-plunges ；存档 `d-d9-cnn.html` / `.txt`。CNN 原文未给出具体时刻。
- Guardian：https://www.theguardian.com/business/2026/nov/05/nasdaq-heading-for-worst-day-in-18-months-as-wall-street-sustains-widespread-losses ；存档 `d-d9-guardian.html` / `.txt`。页面 “First published on Thu 5 Nov 2026 12.31 EST”，即北京时间 2026-11-06 01:31。原文：“slumping 5.5% on Thursday”；“The S&P 500 was down 4.3%.”；“Nvidia … down 9.2%”。
- 同一指标不同来源：Nvidia 跌幅 CNN 9.3%、Guardian 9.2%；纳指 CNN 5.52%、Guardian “5.5%”。全部记录。
- AMD −17.3%：CNN 与 Guardian 两篇原文中均无 AMD 相关数字，未找到。
- AP：未取到（见抓取日志），搜索也未找到 2026 年 11 月 5 日的 AP 报道。
- 不入事实句的因果说法（原文，仅记录以便核对）：CNN “fears that Democrats’ midterm gains could put the brakes on the AI buildout”；Guardian 标题 “Nasdaq sees worst day in 19 months as investors bet on AI slowdown after Democrats’ election triumph”。这些属于因果归因，依派工单规定不进事实句。
- 热度：HN 50390352，3 分 / 5 评，11-06 17:55（指向 Guardian 文）。
- 状态：已找到（CNN、Guardian 的收盘数字，L4）；AMD 数字未找到；AP 未找到一手来源。

---

## 扫描说法勘误

1. D2：扫描写 “加速 1.9x [1.5–2.8x]”。原文中没有 “1.5–2.8x” 这一区间。1.5x 是 Anthropic 复用报告数据的结果，1.9x 是加入更多 benchmark 后的结果，1.3–2.8 是 METR 自己分析给出的区间。三者应分开写。
2. D6：扫描给的 URL https://www.pangram.com/blog/pangram-image-1 返回 404。正确地址为 https://www.pangram.com/blog/introducing-pangram-image-1-0。数字 99.96% 有原文，但它是厂商自测，正文须写明口径。
3. D4：扫描写的 “看不到用户” 在 Meta 原文中不存在；TechCrunch 原页未取到，不能写成“据 TechCrunch”。
4. D9：扫描写的 “AMD −17.3%” 在 CNN、Guardian 中未找到。AP 未取到，扫描中 AP 的说法无法核对。
5. D9：Nvidia 跌幅 9.3%（CNN）与 9.2%（Guardian）不一致，两个数字都记录。
6. D1：扫描的 “下周 Apache 2.0 放权重” 是官方的 “intend”，前提是外部安全测试与红队完成，且原文同时写明当前为 closed weights。应保留这些限定词。
7. D5：扫描的 “费用只算 GPU” 与原文基本一致，但原文是 “$21K of compute”，成本由 H100 小时乘以作者假定的 $2.50 单价算出，不另列人工等其他成本。
8. D8：1104 期的 “即将推出的网络安全模型” 在本组的判断为 GPT‑6 Cyber，已于 11 月 5 日以限定范围发布（见 D8）。
9. D7：免费与开源的原句有，MIT 对应范围是 OpenClaw 整体，正文须写清。

## 未收录或无法核对的条目

- D9 中 AP 的报道：apnews.com 直接访问与搜索都未取到，不再重试。
- D4 中 TechCrunch 同日报道：搜索只有二手转述，未取到原页。
