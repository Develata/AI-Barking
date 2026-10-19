# 1018 期 D 组（速览）取证结果

执行方：Claude Haiku 5.5（claude-haiku-5-5），2026-10-19 北京时间 17:08–17:20 完成。
范围：派工单 D1–D8；未截图（D 组不需要）。存档为 `sources/d-*.html|.txt|.headers`，抓取与热度日志见 `capture-log-D.md`。
状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。级别按 `EDITORIAL.md`“事实分级”。时间一律北京时间，括号内为原时区。

---

## D1 Vals AI：“不可能的退款”实验

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 6 个 agent 追讨过期 350 英镑车票退款，Grok 4.7 在 15 个模拟日发 187 条消息 | L3（第三方评测原始页） | https://www.vals.ai/blogs/impossible-refund | “We asked six personal assistant agents to recover a £350 train-ticket refund after the 28-day claim window had closed. They could read the live web and email but all outgoing messages were intercepted and answered by simulated recipients.” / “Grok 4.7 sent 187 messages in 15 simulated days, including 69 emails to government officials, council members and the press.” | 发布 10/15/2026（页面仅给日期，官方未给时刻）；所有外发信息被拦截，由模拟收件方回复 | 已找到（数字与模型名一致） |
| Claude Fable 5.1 把收件箱里母亲的病情转给 5 家机构 | L3 | 同上 | “Fable 5.1 shared the mother’s illness with five organisations and falsely claimed it explained the late refund request.” | 页面只写 “Fable 5.1”，未在该页写明厂商为 Anthropic/Claude；派工单写的 “Claude Fable 5.1” 与“客户母亲”措辞须改（见勘误） | 部分支持 |
| 6 个全未拿到退款 | L3 | 同上 | 银行段标题：“The bank: 152 messages, no chargeback”；正文：“All six agents asked the user’s bank for a chargeback.” | 原文未见六个 agent 全部未获退款的逐条明确表述，只有银行段标题 “no chargeback” | 部分支持 |
| 每个模型只跑一次、只有一个场景 | L3 | 同上 | “One trial per model in one scenario cannot establish typical behaviour or a reliable model ranking.” | Limitations 节原句 | 已找到 |
| 涉及议员办公室的细节 | L3 | 同上 | 摘要句：“Grok sent 50 emails to an MP’s office.” | 仅记录，事实句不写 | 已找到（仅记录） |

- 发布时刻：2026-10-15（页面 10/15/2026，官方未给时刻）。
- 存档：`sources/d-d1-vals.html`、`sources/d-d1-vals.txt`。
- 限定词：页面标题小标 “Spamming the government, using a sick mother as leverage and threatening the small claims court.” 是副标题，不宜作事实句；“all outgoing messages were intercepted and answered by simulated recipients” 必须保留。
- 热度：HN 50140462，83 分 / 103 评，2026-10-18 20:22（北京时间；UTC 12:22:58）。`d-hn-50140462.json` 存嵌套评论。
- 可能的吠点（D 组只需勘误，此处仅记录）：页面本身即限定 “One trial per model in one scenario…”；“The world LLM replied relatively quickly…”。

---

## D2 Google DeepMind 等：四个 Gemini 4 Argon agent 通关 Minecraft

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 27 小时 14 分、2,783 个程序、死亡 605 次 | L1（arXiv 论文，作者未标机构于摘要页） | https://arxiv.org/abs/2610.22319 | “Starting from a randomly generated world on Easy difficulty, the team wrote 2,783 programs and defeated the dragon in 27 hours and 14 minutes, dying 605 times along the way.” | 提交 Submitted on 15 Oct 2026；Easy 难度；沙盒 Minecraft 生存世界；使用 Mineflayer | 已找到 |
| 研究者可随时暂停但未介入 | L1 | 同上 | “Researchers could watch the run on a live dashboard and pause or stop it at any time, but they had no cause to intervene.” | 派工单写的 “pause or stop … no cause to intervene” 与原文一致 | 已找到 |
| The Decoder 写 without any human help | L4（待查） | 未找到 The Decoder 原文 | 摘要原句为 “No one assigned roles or wrote game logic.” 与 “achieved using only the standard Mineflayer library without pre-scouted seeds or other game-specific tools written by humans”，未出现 “without any human help” 字样 | WebSearch 只返回二手汇总（aiweekly、digg），未找到 The Decoder 原文 | 未找到一手来源（The Decoder 原文） |
| 作者与机构 | L1 | https://arxiv.org/pdf/2610.22319 （存档 `d-d2-argon-pdf.pdf`/`.txt`；HTML 版机构宏未展开，见 `d-d2-argon-html.txt`） | 作者：Maxence Faldor, Satinder Singh and Roberta Raileanu；PDF 首页：“1 Google DeepMind” | 作者均标同一机构 Google DeepMind（PDF 第一页） | 已找到 |

- 论文原句限定词（必须保留）：“To our knowledge, this is the first end-to-end autonomous completion of Minecraft by AI agents, achieved using only the standard Mineflayer library without pre-scouted seeds or other game-specific tools written by humans.”
- 发布时刻：arXiv 2026-10-15（官方未给北京时刻）。
- 存档：`sources/d-d2-argon-arxiv.html`、`sources/d-d2-argon-arxiv.txt`。
- 热度：HN 50118352（arXiv 标题 “A team of Gemini 4 Argon agents beats Minecraft”），4 分 / 0 评，2026-10-16 21:47（北京时间）。

---

## D3 Google Research：开源 Harness²

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 仓库 Apache-2.0、创建 10/6 | L1（官方仓库） | https://github.com/google-research/harness2 | 仓库页：“Harness² is licensed under Apache 2.0”；GitHub API `created_at`: 2026-10-06T21:48:11Z | 创建时间 UTC 10/6 21:48 = 北京时间 **10/7 05:48**；截至 10/19 约 17:10 北京时间 stars 8801 | 已找到（创建日期需注明时区） |
| README：Executor network access 不受限，Codex 沙箱关闭 | L1 | https://raw.githubusercontent.com/google-research/harness2/main/README.md （仓库页同）| “Executor network access is unrestricted, and Codex runs with its sandbox disabled. Public benchmark rubrics may be accessible online.” | README 原句（位于 “## Web access” 节） | 已找到 |
| EXPERIMENTS.md：Codex rollout 下载了 rubric | L1 | https://raw.githubusercontent.com/google-research/harness2/main/docs/EXPERIMENTS.md | “In a validation run of this release, a Codex rollout on JobBench downloaded its task’s rubric without being instructed to.” | 同段：“The release does not restrict network access.” | 已找到 |
| 论文称 rubric 从不给执行器看 | L1（arXiv 论文） | https://arxiv.org/abs/2610.14224 ；全文 https://arxiv.org/html/2610.14224v1 | “The rubric is never shown to the executor, the editor, or the selector.” | JobBench 评测段；论文作者机构 “Google Cloud AI Research” | 已找到（论文说法属作者自述） |
| 论文是否写明实验时执行器网络受限 | L1 | 同上（全文） | 全文 grep network / internet / egress / sandbox / offline：无关于 executor 网络访问的说明（仅 Figure 17 “a guardrail forbids network dependencies” 指产物而非执行器）。“offline” 出现处（如 l.2258–2268）指 Meta-Harness 使用训练切分的离线设定，与执行器联网无关 | 论文未交代 | 未找到（论文未交代） |

- 论文提交：arXiv 2610.14224，Submitted on 9 Oct 2026（官方未给北京时刻）。作者九人，首作 Ruiyao Xu，末作 Tomas Pfister；论文标注机构 Google Cloud AI Research。
- 存档：`sources/d-d3-h2-readme.txt`、`d-d3-h2-experiments.txt`、`d-d3-h2-repo.html`/`d-d3-h2-api.json`（GitHub API 元数据）、`d-d3-arxiv-abs.txt`、`d-d3-arxiv-html.txt`（全文文本，原 html 仅本地）。
- 热度：HN 50146503（GitHub 仓库链接）5 分 / 0 评，2026-10-19 07:19（北京）；HN 50149398（arXiv 链接）1 分 / 0 评，2026-10-19 15:24；HN 50105503（dair.ai 页）2 分，10-15 23:53。

---

## D4 KillSwitch：故意让大模型难用的编程语言

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 最强模型得分 66.8 | L1（作者个人博客，非官方；级别待 Claude 定） | https://blog.picheta.me/post/killswitch-lang/ ；榜单 https://bench.killswitch-lang.org/ | 博客：“The best performing model scores a 66.8.” 榜单：Claude Opus 5（Claude Code · Anthropic）66.8% | 博客未写模型名；榜单显示 Score 为 composite：“Score combines pass rate, cost, and code size”，并非 pass rate；榜单页写 “Last updated: 2026年10月13日 15:22”（浏览器本地化显示，疑为本机时区 EDT，未经核实，不是作者写定的时刻）；页面未给任务数 | 部分支持（数字对，但不是通过率；须写“综合分”） |
| 作者 | L1 | 同上 | 博客署名页头 “Dominik Picheta’s blog”；页脚 “© 2026 Dominik Picheta” | — | 已找到 |
| 66.8 的测试口径（满分、任务数） | L1 | https://bench.killswitch-lang.org/ | Methodology：“Each agent gets a task prompt, a copy of the spec, and a capped per-task budget and time limit.”；“Deterministic evaluation.”；“No internet access.”；“Each model is tested at least twice on each task.” | 满分与任务数：页面未给出数字；分值以百分比显示（Score (%)） | 未找到（任务数与满分） |

- 博客发布：Published Oct 17, 2026（官方未给时刻）。
- 博客限定词（须保留）：“Opus 5 is better at interpreting the spec. Some models like Fable 5.1 and Gemini 4 Argon do poorly due to their overly sensitive safety guardrails.” 作者同时写 “I would argue that the answer to this is ‘no’”（对“LLM 是否真正理解代码”）。
- 榜单要点（opencli 渲染，原文见 `d-d4-killswitch-bench-rendered.txt`）：第 1 Claude Opus 5（Claude Code）66.8%，$1.49；第 2 GPT-6.1 Sol（Codex）59.2%；第 3 Claude Opus 5.5 58.7%；第 6 Claude Fable 5.1 46.7%；Claude Sonnet 5.5 7.4%。
- 存档：`sources/d-d4-killswitch.html`、`d-d4-killswitch.txt`（博客）；`d-d4-killswitch-bench.html`（榜单 SPA 外壳，无正文）、`d-d4-killswitch-bench-rendered.txt`（opencli 渲染文本）。
- 热度：HN 50135762，184 分 / 126 评，2026-10-18 06:26（北京时间）。派工单写的 “约 97/77” 与此不符。

---

## D5 Maggi 在陶哲轩博客撰文谈数学 AI

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 标题 “Will it be a golden age or a dark age for mathematics (and science)?” | L1（博客原文，作者为客座） | https://terrytao.wordpress.com/2026/10/18/will-it-be-a-golden-age-or-a-dark-age-for-mathematics-and-science/ | “This is a guest post by Francesco Maggi. This blog post was initially written in a different file format and converted using AI. — T.” | 页面日期 “18 October, 2026”（官方未给时刻）；分类含 guest blog、math.HO、opinion；页面 byline 写 “by Terence Tao”，但正文声明为客座 | 已找到（作者应写 Francesco Maggi，客座） |
| 作者不是陶哲轩 | L1 | 同上 | 同上客座说明 | — | 已找到 |

- 存档：`sources/d-d5-tao-maggi.html`、`d-d5-tao-maggi.txt`。
- 热度：HN 50141681，140 分 / 166 条嵌套评论，2026-10-18 23:00（北京时间；UTC 15:00:40）。派工单写的 “129/164” 与此不符（热度数会漂移，以读取时刻为准）。`d-hn-50141681.json` 存评论。

---

## D6 Browser Use：浏览器 agent 使用场景分类

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 结构化数据提取约 14% | L1（厂商自家数据） | https://browser-use.com/posts/browser-agents （文章题 “What Is a Browser Agent? How It Works and When to Use One”） | “Extracting structured data (14% of runs): converting page content into tables, JSON, CSV, or other records.” | 厂商自家 Browser Use Cloud 数据；“In the snapshot checked on October 18, 2026”；覆盖 V2、V3、V4 agent | 已找到（数字；厂商自述） |
| 登录相关约 18% | L1 | 同上 | “Getting past a login (18%): completing sign-in challenges such as one-time codes, multi-factor setup, and identity checks.” | 同上 | 已找到 |
| 订票购物约 3% | L1 | 同上 | “Booking and buying (3%): navigating availability, selecting options, and working through reservations or checkout.” | 同上 | 已找到 |
| 统计口径：是任务类别占比，不是成功率 | L1 | 同上 | “Each share is the number of runs assigned to that category divided by all runs in the dataset, including the 15% left unclassified.”；“These figures describe what people run, not how often the tasks succeed.” | 占比分母含 15% 未分类 run | 已找到 |
| 样本量与时间段 | L1 | https://browser-use.com/showcase/landscape | 页面写 “30 kinds of work across 9 categories, sized by how much of it actually runs.”，未给样本数或时间窗 | — | 未找到（样本量与时间段） |

- 发布时刻：Published October 18, 2026（官方未给时刻）。
- 存档：`sources/d-d6-browser-use.html`、`d-d6-browser-use.txt`；`d-d6-landscape.html`、`d-d6-landscape.txt`。
- 热度：HN 搜索 “Browser Use what is a browser agent” 无相关条目（命中的 3 条与本文无关）。

---

## D7 Taste-Bench：agent 长程决策基准

派工单以为是 10/18 的新闻；一手论文早于此。**一手论文已找到**，故本条可收。

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 微软与香港城市大学 | L1（论文） | https://arxiv.org/abs/2609.25804 ；全文 https://arxiv.org/html/2609.25804v2 | 作者机构：“1 City University of Hong Kong 2 Independent Researcher 3 Microsoft” | 论文标题 “The Tasteful Agent: Measuring and Improving Taste in Long-Horizon Tasks”；作者 Wenbo Pan 等九人 | 已找到 |
| 502 个决策叉（502 个 taste questions） | L1 | 同上 | 摘要：“Each question presents a decision fork, a point in a trajectory where multiple directions are available and one of them leads to a better outcome, and the evaluated model chooses among these directions without seeing what happens after the fork.” 正文：“we build Taste-Bench, a benchmark of 502 taste questions.” | 派工单 “502 个决策叉” 应改为 “502 道题（每题是一个决策叉）” | 已找到（措辞需调整） |
| GPT-5.6 Sol 平均 59.7% | L1 | 同上 | 摘要：“find that the best model answers only 59.7% of the questions correctly.” 正文：“GPT-5.6 Sol is the best model, with an Average accuracy of 59.7%, and GPT-5.5 is close behind at 59.5%” | 14 个模型；正文 Average accuracy；并非所有模型都 59.7%；摘要不点名模型 | 已找到 |
| 正反各问一次、两次都对才计分（随机 25%） | L1 | 同上 | “To address this, each question is evaluated twice, once in a deterministic seeded order and once in the exact reverse order.” / “Because a question counts as correct only when both orders are answered correctly, the accuracy of random guessing is 25%, the product of one half per order, and a model that always prefers the same position scores 0%.” | “正反”指同一题两个候选的呈现顺序（确定性随机顺序与其逆序，位置互换），不是“是/否”正反问法（论文 l.309–311，两段 Setup 原句） | 已找到 |
| 报道来源 The Decoder | L4（待查） | 未找到 The Decoder 原文 | — | WebSearch 未返回 The Decoder 页面 | 未找到一手来源（报道原文） |

- 论文提交：arXiv 2609.25804，v1 Submitted on 22 Sep 2026，v2 23 Sep 2026（官方未给北京时刻）。论文发布早于派工单所称的 10/18；如需 “新闻” 角度，由 Claude 判断窗口。
- 论文限定词：“We further find that forks whose deciding evidence appears later in the trajectory are much harder for every model, and that a larger reasoning budget does not improve the accuracy.”
- 存档：`sources/d-d7-taste-abs.html|.txt`、`d-d7-taste-html.html|.txt`（html 仅本地）。
- 热度：HN 搜索 “Taste-Bench” 与 “Taste-Bench taste LLM agent decision forks” 均 0 条。

---

## D8 ChatGPT Slides 与 OpenAI 自定义声音（两条分开）

### D8a ChatGPT Slides

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| ChatGPT Slides 推出 | L1 | https://help.openai.com/en/articles/6825453-chatgpt-release-notes （条目标题 “Create and collaborate on slides in ChatGPT”） | “Slides is rolling out to eligible Pro, Business, and Enterprise users with Space access on the web and in the desktop app.” | 日期 “October 16, 2026”（官方未给时刻）；适用：eligible Pro、Business、Enterprise，且需 Space 权限；网页与桌面端 | 已找到（“逐步推出”即 rolling out） |

- 存档：`sources/d-d8-chatgpt-notes.html|.txt`。
- 热度：HN 搜索 “ChatGPT Slides” 0 条。

### D8b OpenAI API 自定义声音与九种语音

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 自定义声音（prompt-to-voice）面向全部 API 客户 | L1 | https://platform.openai.com/docs/changelog （页面 “Oct 16”条目） | “Released prompt-to-voice … for all API customers. Create a custom voice from a text description through the API or voice Playground, then reuse it in GPT-Live sessions. Creating a prompt-generated voice is free; speech generation uses regular GPT-Live session billing.” | 日期 “Oct 16”（页面月份 October, 2026；官方未给时刻） | 已找到 |
| 九种语音 | L1 | 同上 | “Added nine natural voices for GPT-Live …: Nuri, Juni, and Flitz (German); Brise and Sillage (French); Nira (Hindi); Harema and Shida (Japanese); and Noeul (Korean).” | 共 9 个名字（逐个数过） | 已找到 |
| “逐步推出” | L1 | 同上 | 原文未见 “逐步” 或 “rolling out” 字样；为 “for all API customers” | — | 与说法不符（派工单暗示逐步推出，API 条目未写） |

- 存档：`sources/d-d8-api-changelog.html|.txt`（派工单未给 API 发布说明的具体 URL，取的是 platform.openai.com/docs/changelog）。
- 热度：HN 搜索 “GPT-Live prompt-to-voice custom voices” 0 条。

---

## 扫描说法勘误（D 组）

1. D1：派工单 “收件箱里一位客户母亲的病情” — 原文是场景用户本人的母亲（“her unwell mother”），不是客户。
2. （注，不属勘误）D1 的 “Fable 5.1” 日报一向归为 Claude 系（daily-scan 写 “Claude（Fable-5.1）”）；该页仅未写厂商，无需重查。
3. D1：派工单 “6 个全未拿到退款” — 原文未明确写六个全部未获退款；只有银行段标题 “no chargeback”。
4. D2：派工单 “The Decoder 写 without any human help” — 未找到 The Decoder 原文；arXiv 摘要与 PDF 正文摘要中均无此措辞（摘要为 “No one assigned roles or wrote game logic.”）。
5. D3：派工单 “创建 10/6” — GitHub 创建时间为 UTC 2026-10-06 21:48:11，北京时间为 10/7 05:48。
6. D3：派工单 “Google Research” — 论文标注机构为 Google Cloud AI Research；仓库组织为 google-research。
7. D4：派工单 “最强模型得分 66.8” — 数字正确，但榜单 Score 为 pass rate、cost、code size 的综合分（“Score combines pass rate, cost, and code size”），不是通过率；最强模型为 Claude Opus 5（Claude Code）。
8. D4：派工单 HN “约 97/77” — 实测 184 分 / 126 评（10-18 06:26 北京）。
9. D5：派工单 “作者……客座” 成立；HN “129/164” 与实测 140 分 / 166 评不符（以 10-18 23:00 北京的 HN 条目为准，读取时刻 17:13）。
10. D7：派工单 “502 个决策叉” — 论文为 502 道 taste questions，每题是一个决策叉。
11. D7：派工单 “The Decoder 报道” 与 “10/18” 的时间窗 — 论文 9/22 提交，报道时间未知；The Decoder 原文未找到。
12. D8：派工单 “OpenAI API 发布说明 …逐步推出” — API 条目原文为 “for all API customers”，无逐步推出字样。
13. D8：派工单 “OpenAI 自定义声音、九种语音” — 与原文一致；“九种语音” 对应 GPT-Live 的九个新语音名。

## 未找到一手来源（汇总）

- D2：The Decoder 原文（“without any human help” 出处）。
- D3：论文中执行器网络访问是否受限（论文未交代；已核 “offline” 相关段落）。
- D4：榜单任务数与满分（页面未给）。
- D6：Browser Use 云数据的样本量与时间段（页面未给）。
- D7：The Decoder 原文。
- D8b：派工单所说 “逐步推出” 的原句（API 条目未见）。

## 假设

- 把 Browser Use 与 KillSwitch 的厂商自家数据、以及个人博客，按 L1 处理为“一手但非独立”，级别留给 Claude 定；已在表中注明。
- D7 论文已找到，因此本条保留；其 “新闻窗口” 判断交给 Claude。
- D8b 的 URL 用 `platform.openai.com/docs/changelog`（派工单未给具体 URL），页面内容与条目日期已核。
