# 1012 事实核验

期次目录按 Develata 美国本地日期（10月12日，美东）。本机时区 EDT（UTC−4），北京时间 = 本机 + 12 小时。取证分四组：A Karpathy 漏斗、B GPT-6.1 Sol 500k、C ArtCraft（Sonnet 5.5 子代理，effort high），D 速览（Haiku 5.5，effort high）；清单见 `evidence-a.md`–`evidence-d.md`，抓取日志见 `capture-log-*.md`。Claude 另自读：Karpathy 原帖与 5/19 加入 Anthropic 帖（opencli、fxtwitter）、Tibo 两帖与 10/10 回帖（opencli、fxtwitter）、OpenAI API 模型页（curl）、Codex 定价页 FAQ（`b-codex-pricing.txt`）、PhotoCraft README（raw.githubusercontent）与 GitHub releases / PR #372 / vectorcraft 提交 87cf1ff（gh api）、PhotoCraft 提交尾注计数（gh api，见下）、TechRadar（curl）、HN 49958850 / 49993722 / 49981449（Algolia），以及速览各条存档的逐字摘句。

## 选题变更记录

- 线索：ChatGPT 扫描（北京 10-12 11:06）与 WorkBuddy 扫描 1012-12（北京 12:00）。Claude 用 HN Algolia 与首页取热度（北京约 12:50），读关键一手后推荐 A Karpathy + B Sol 500k + C ArtCraft；Develata 选定。
- 扫描说法与一手不符、已纠正：
  - WorkBuddy 称 Karpathy 帖发于 10-11 20:20 UTC：那是 HN 提交时间；原帖首版 19:10:46 UTC、编辑版 19:12:37 UTC（北京 10/12 03:10 / 03:12，`evidence-a.md` A1）。WorkBuddy 的 HN 57/25 为旧快照（实测 338/513 → 360/525）；两份扫描的 AIHOT 热度 142 与 12160 均未在页面见到（实测热点榜 NO.01、热度 169）。WorkBuddy 把 Karpathy 原帖标 L1，应为 L2（个人署名）。
  - WorkBuddy 称 Vals 团队实验“没有自己的方法论页面、只能当二手”：Vals 博客 10/09《Do Agent Teams Pay Off?》与 vals.ai/methodology 均存在（`d-d3-*`）。
  - WorkBuddy 所称站长之家两篇、华军软件园、腾讯新闻报道 500k：B 组 8 次搜索未找到；实际中文首报为 IT之家（`evidence-b.md`）。
  - ChatGPT 扫描称陶哲轩《Math 2.0》HN 帖为主线：另有 10/08 提交的 Mastodon 帖（611/652），陶哲轩首发于北京 10/07 02:00（`evidence-d.md` D4）。
  - TechRadar 称 PhotoCraft 读不了 HEIC：其测试版本为 0.20（作者原话）；HEIC 读取的 PR #372 于 10/07 14:20 UTC 合并，v0.5.0 于 10/08 14:52 UTC 发布，早于 TechRadar 10/10 13:00 UTC 发文。TechRadar 未写测试日期，本期正文不用这条。
  - AIHOT / 卡兹克写 ArtCraft“逆向”Adobe、HN 一帖标题写“decompiled”：项目自述 clean-room（见主帖三）。
  - Nicolas Cage“Amazon 向 OpenAI 投了 500 亿美元”：Amazon 新闻稿（SEC 8-K Ex.99.1）为分期承诺（先 150 亿，满足条件后再 350 亿），速览不写这个数字。
- 风控：标题与封面不出现“政府”；Karpathy 帖里的零日、网络攻击字样不进标题、封面与卡片文字。三条主帖均不涉外国政府或点名中国公司。速览未收 Yandex（军事）、英国 AI 网安与 Google 芬兰机房（涉政府）。

## 主帖一：Karpathy 的“5000人”漏斗

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Karpathy 10月12日发帖 | L2 | ✅ | https://x.com/karpathy/status/2109361546505966046 （`a-x-karpathy-raw.json`、`a-x-karpathy.txt`，图 `01`–`03`） | 首版 2026-10-11 19:10:46 UTC、编辑版 19:12:37 UTC = 北京 10/12 03:10 / 03:12；两版只差一个重复段，数字与 “Napkin math” 位置不变（`evidence-a.md` A1）。引用其 2026-04-09 旧帖。 |
| 约60亿人几乎没接触过大模型 | L2 | ✅ | 同上 | “Napkin math somewhere around 6B people (~75% of the population) have barely come in contact with LLMs at all.” |
| 10亿–20亿人偶尔用免费聊天产品 | L2 | ✅ | 同上 | “Around 1-2B (~20%) are casual and infrequent users of free-tier ChatGPT-like products.” |
| 约2000万人在数学和代码上专业用前沿模型 | L2 | ✅ | 同上 | “Now we get to professional use of frontier-grade LLMs in math&code. Somewhere around 20M people (0.2%) …” |
| 约5000人能在内部用前沿级系统 | L2 | ✅ | 同上 | “the ~5,000 people (~0.00006%) with access to frontier-grade systems internally.”（初稿写“实验室内部”，原帖无“实验室”，按反向核验改为“内部”。） |
| 省流：Karpathy 粗估约5000人能在内部用前沿系统 | L2 | ✅ | 同上 | “粗估”见下一行的口径说明。 |
| 指使用权而非“懂 AI”；5000指内部使用权 | — | ✅ | 同上 | 原帖用词为 “with access to”，全帖未说这 5000 人“懂 AI”。属本号判断，依据为原文措辞。 |
| “粗估”是他写在第一层的，四层人数都没给来源 | L2 | ✅ | 同上 | “Napkin math” 全帖只出现一次，在第一层（6B）开头；其余三层用 “Around”“Somewhere around”“~”；全帖未给任何数字的来源（`evidence-a.md` A1–A2）。省流与卡片写“粗估约5000人”：原帖未把 napkin math 单独标在第四层，但 5000 同样只有 “~” 且无来源；正文吠点已写明标注位置。 |
| 虎嗅标题写成“全球见过 AI 真实能力的，只有5000人”，没提估算 | L5（作为夸大实例引用） | ✅ | https://www.huxiu.com/article/4896682.html （`a-m-huxiu.txt`） | 标题原文“Karpathy：全球见过AI真实能力的，只有5000人”（转载自公众号“夕小瑶科技说”，北京 10/12 09:18）；正文另写“这些数字是Karpathy自己的粗略估算”，标题没有。正文为满足 lint 在 AI 两侧加了空格。 |
| 他本人5月宣布加入 Anthropic | L2 | ✅ | https://x.com/karpathy/status/2056753169888334312 （`a-x-karpathy-anthropic-join-raw.json`；Claude 另用 fxtwitter 复核） | “Personal update: I've joined Anthropic.” 2026-05-19 15:05:42 UTC。原帖未说他是否属于那 5000 人，正文不写。职责（预训练团队）只有 L4，未用。 |
| 第四层“数千智能体连续数周协作”只是描述，未给可查证据 | L2 | ✅ | 原帖 | “It looks like swarms of thousands of agents collaborating over weeks on software mega projects” —— 原帖只有描述，未给数据、链接或可查材料；原帖另说 “The external world has seen the preview.”（初稿“外界无法核实”过强，按反向核验改。） |

未入正文：OpenAI 称 ChatGPT 周活 “more than 1.2 billion”（10/05、10/07 官方页，`a-openai-gpt6-for-everyone.txt`）、Google 称 AI Overviews 月活 “over 2.5 billion” 等口径各异，不能相加，不用作吠点（`evidence-a.md` A4）。Karpathy 引的 “722 manuscripts” 是 1006 期初始数，OpenAI 10/07 撤回 3 篇后仓库现写 719（`a-openai-math-README.md`、`a-openai-math-history.md`），字数所限未写。

## 主帖二：Sol 默认上下文 500k

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 员工 Tibo 10月12日称 | L2 | ✅ | https://x.com/thsottiaux/status/2109430343346581527 （`b-fx-2109430343346581527.final.json`，图 `13`、`24`、`43-tibo-default-raw.png`） | Thibault Sottiaux（OpenAI）。2026-10-11 23:45:59 UTC = 北京 10/12 07:45。帖首 “Day 7/”。 |
| GPT-6.1-Sol 在 Codex 与 ChatGPT Work 中的默认上下文由 272k 提到 500k | L2 | ✅ | 同上 | “We have increased default context length for GPT-6.1 Sol to 500k when used through Codex & Work, up from 272k or an ~ 80% increase.” 官方产品名为 ChatGPT Work（`evidence-b.md` B3）。省流写“Codex 等处”。 |
| 最高 1M | L2 | ✅ | 同上 | “You can still override it if you want to be higher our lower, with maximum context length of GPT-6.1 Sol of 1M.” API 模型页写 “1,050,000 context window”；openai/codex 仓库内置目录写 `max_context_window: 872000`（改动前客户端兜底值）。正文照 Tibo 原话写 1M。 |
| 用订阅额度时不收其长上下文费 | L2 | ✅ | https://x.com/thsottiaux/status/2109430894604026039 （`b-fx-2109430894604026039.final.json`，图 `14`、`44-tibo-no-fee-raw.png`） | 自回复 23:48:11 UTC = 北京 07:48：“With this change, we are not charging long context fees for GPT-6.1 Sol when drawing usage from the subscription.” 写“不收”不写“取消”：Tibo 7 月对 GPT-5.6 Sol 已说过订阅上不额外收（`evidence-b.md` 关键发现 3），6.1 Sol 此前是否收无官方陈述。 |
| 只限订阅额度；API 文档照旧：输入超过 272K token 的请求，整次按2倍输入、1.5倍输出计价 | L1 | ✅ | https://developers.openai.com/api/docs/models/gpt-6.1-sol （`b-model.txt`；Claude curl 复核；图 `16`、`45-api-long-context-raw.png`） | “Prompts with more than 272K input tokens are priced at 2x input and cache rates and 1.5x output for the full request.” 缓存价同样 2 倍，正文省略“与缓存”（不改变方向），卡片译注保留。Wayback 10/11 12:10Z 快照与现行页逐行无差异（B2）。 |
| 省流：API 输入超 272K 仍整次加价 | L1 | ✅ | 同上 | 即上一行的 >272K 倍率（初稿“API 照旧加价”漏了门槛，反向核验 BLOCKER，已改）。 |
| 不收附加费不等于不多耗额度：Codex 定价页写明，保留更多上下文的长会话每条消息耗得明显更多 | L1 | ✅ | https://learn.chatgpt.com/docs/pricing （`b-codex-pricing.txt`，图 `18`、`46-codex-usage-raw.png`） | FAQ “What are the usage limits for my plan?”：“… larger projects, long-running tasks, or extended sessions that require the agent to hold more context will use significantly more per message.” 原 developers.openai.com/codex/pricing 308 重定向至此。 |
| （已删，未入正文）10月10日 Tibo 说开到 1M 只会更耗额度 | L2 | ✅ | https://x.com/thsottiaux/status/2108778939057664357 （`b-fx-2108778939057664357.json`；Claude fxtwitter 复核） | 2026-10-10 04:37:32 UTC = 北京 10/10 12:37，回复他人：“Why do you think we picked 300k for context length for Codex. We ran the numbers and it’s better. We can run it at 1M too but it would just cost more of the usage. Good defaults are important.” 他说 300k，当时默认为 272k；正文不引这个数字。 |

未入正文：截至取证（北京 10/12 13:50），Codex changelog、定价页、模型页与 help.openai.com 均未见 500k 或“订阅不收长上下文费”的条文（`evidence-b.md` 关键发现 1）；Enterprise 按 token 计费的费率卡仍列 >272K 倍率，积分与 Enterprise 计费下是否收未见说明。

## 主帖三：AI 造“免费 Photoshop”

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| ArtCraft 做了对标 Photoshop、Premiere 等7款开源应用 | L1 | ✅ | https://getartcraft.com/apps ；各仓库 README（`evidence-c.md` C1） | 对标 Adobe 的 7 个仓库：Photoshop、Illustrator、Premiere Pro、Lightroom、Acrobat、After Effects、InDesign；官网现写 “Twelve apps”（另 5 款对标微软、Autodesk、Avid），10/07 时为 seven。本行只认证 7 个对标仓库的存在与对标对象；“用 AI 开发”见下一行（开发者自述，L2）。 |
| 开发者称用 Opus-5.5 编写；省流：ArtCraft 开发者称用 AI 做了7款对标 Adobe 的软件 | L2 | ✅ | HN 评论 49934844（`c-hn-comment-49934844.json`，图 `26`） | 作者 echelon：“I'm currently using Opus 5.5 to create …”（全文见存档）；GitHub 同名账号 bio “Founder: ArtCraft, Storyteller.ai, FakeYou”，HN 与 GitHub 为同一人属推断。正文不点开发者网名或姓名（身份对应为推断）。佐证：PhotoCraft AGENTS.md 首行 “guide for AI agents and contributors”，提交尾注见下。初稿直述“用 AI 智能体做了”并记 L1，按反向核验改为开发者自述（L2）。 |
| PhotoCraft 约4.9万星 | L1 | ✅ | https://github.com/storytold/photocraft （GitHub API） | 48,754（北京约 13:00）→ 48,967（13:54）。仓库创建 2026-09-30T13:37:48Z。 |
| 截至10月12日，多数提交注明 Claude 为共同作者 | L1 | ✅ | git 历史（`c-gh-commit-stats.txt`；Claude 逐提交存档 `m-photocraft-commits.tsv`） | `m-photocraft-commits.tsv` 为 gh api 逐提交清单（sha、提交时间、是否含 “Co-Authored-By: Claude” 尾注），2026-10-12T10:14:55Z 取：共 1665 个、含尾注 1073 个（64%）；按提交时间截至 05:20Z 为 1614 / 1045，与 C 组统计一致。逐提交尾注原文行见 `m-photocraft-trailers.tsv`（每个 sha 对应的 “Co-Authored-By:” 行，可离线重算标记本身）。初稿写精确计数，反向核验指出缺可重算存档，正文改为“多数”。尾注只表示声明共同署名，不代表代码占比（正文用“注明”）。贡献者 225 人。 |
| PhotoCraft 说明里写着：早期 alpha，“还不能替代 Photoshop 做日常专业工作”；省流：PhotoCraft 自称还不能替代 Photoshop 做日常专业工作 | L1 | ✅ | PhotoCraft README（raw.githubusercontent 复核；`c-readme-photocraft.md`；图 `29`、`48-photocraft-status-raw.png`） | “Status: PhotoCraft is in early alpha … it is not yet a Photoshop replacement for daily professional work.” 徽章 “status: early alpha”。 |
| 网传它“反编译了 Adobe” | L6 | ✅（作为网传引用） | Bluesky 帖 https://bsky.app/profile/jamesomalley.co.uk/post/3mxbl36nkms2q （`c-bsky-thread.json`，图 `31`） | 2026-10-07 08:54:11 UTC，约 4,400 赞：“Someone has decompiled the Adobe suite, rebuilt it in Rust and released it as open source.” HN 49993722 转载此帖为标题；HN 评论 fotcorn：“no, it's not decompiled from real Photoshop”。另有媒体写 reverse-engineered（`evidence-c.md` C10）。原帖含粗口，不作配图。 |
| 项目自称 clean-room 实现，只依公开规格和观察到的行为 | L1 | ✅ | PhotoCraft README（图 `25`、`47-photocraft-cleanroom-raw.png`） | “an open-source, clean-room reimplementation of Adobe Photoshop, rebuilt in pure Rust.”；“Clean-room: implemented from public specs and observed behaviour only. No proprietary code, shaders or assets.” vectorcraft 10/08 提交 87cf1ff 把智能体规则收紧为不运行、不观察任何 Adobe 软件（gh api 复核）；PhotoCraft 的 `docs/target-app-parity.md` 写检查过本机 Photoshop 安装包（未启动）。正文未展开，不作法律判断。 |
| 本号未见 Adobe 回应 | — | ⚠️（有限检索） | `evidence-c.md` C8；`capture-log-c.md` | 检索范围：Adobe 新闻室首页（未做站内搜索）、各媒体全文与搜索；PetaPixel 写 “has asked Adobe for comment”。只代表本次检索未见，不认证 Adobe 没有回应。 |
| 不判断是否合规 | — | （编辑立场） | — | 本号不作法律判断；不是事实陈述。初稿“合规与否未定，Adobe 未公开回应”过强，按反向核验改。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 陶哲轩称数学进入“证明充裕”时代，但前沿多数问题仍未解决 | L2 | ✅ | https://teorth.github.io/tao-web/slides/math-2-0-caltech-2026.pdf （`d-d4-tao-slides.txt`）；Mastodon 首发 https://mathstodon.xyz/@tao/117395269325940185 | “we are now entering an era of proof abundance in mathematics”；“Despite the impression given by press releases and social media, most of the problems at the current frontier of mathematics remain unsolved, even with massive computational expenditure.” 首发北京 10/07 02:00（窗口前）；HN 611/652。个人署名讲稿，按 1009 期先例记 L2。 |
| 据 Vals 网页应用评测，团队成本为单智能体1.8–5.1倍，仅一组显著提升 | L3 | ✅ | https://www.vals.ai/blogs/multi-agent-vibe-code-bench （`d-d3-vals-multi-agent.txt`） | 按反向核验补“网页应用”范围（Vibe Code Bench：50 个完整网页应用；作者提醒不外推到其他工作）。“团队”指智能体团队。页面日期 10/09/2026：“Teams cost 1.8 to 5.1 times as much as single agents, and only one difference in score was statistically significant.”；四组 = GPT 6 Sol 与 Claude Opus 5.5 × medium / max；唯一显著的是 Sol medium（+7.3 分，p = 0.005）；“Each of the eight setups ran once.” Vibe Code Bench 50 个应用。 |
| Sakana 称模型自评自训两轮，三项合成任务对原版胜率69.9%，评审为模型 | L1 | ✅ | https://pub.sakana.ai/mass/ （`d-d2-mass-page.txt`）；arXiv 2610.12176 | “On the three synthetic test tasks, the win rate against the base rises from 53.9% after one round to 69.9% after two rounds”；“only a self-evaluation signal (whose correctness is unknown) guides the overall learning loop”；LoRA 微调自身输出；“we separately ask GPT-5.5 and Claude Opus 4.8 to assess the completed work.” 基座 Qwen3.6-27B。arXiv 提交北京 10/08 23:44（窗口前）。 |
| 据 Variety 报道，演员凯奇称没签亚马逊的 AI 授权书，大概不再与其合作 | L4 | ⚠️ | https://variety.com/2026/tv/news/nicolas-cage-ai-waiver-amazon-spider-noir-1236907564/ （`d-d7-variety-us-orig.txt`） | “I didn’t sign the waiver to let him use AI on ‘Spider Noir.’ I’m probably not going to be working with that studio again, but that’s okay.”（纽约动漫展，10/10 周六）；Variety 北京 10/11 00:54。“大概”对应 probably not；“与其”指 that studio（Amazon 开发了该剧）。500 亿说法未用（见选题变更记录）。 |

## 批注译注（`images/cards.toml` 的 gloss，措辞以此为准）

| 图 | 原句 | 译注 |
|---|---|---|
| 41 | Napkin math somewhere around 6B people (~75% of the population) have barely come in contact with LLMs at all. | 粗略估算，大约60亿人（约占人口75%）几乎没接触过大模型。 |
| 42 | And finally we get to the ~5,000 people (~0.00006%) with access to frontier-grade systems internally. | 最后是约5000人（约0.00006%），能在内部用上前沿级系统。 |
| 42 | It looks like swarms of thousands of agents collaborating over weeks on software mega projects | 看起来像是成群的数千个智能体，连续数周协作做超大型软件项目 |
| 43 | We have increased default context length for GPT-6.1 Sol to 500k when used through Codex & Work, up from 272k | 我们把 GPT-6.1 Sol 在 Codex 与 Work 中使用时的默认上下文长度提到 500k，此前为 272k |
| 44 | With this change, we are not charging long context fees for GPT-6.1 Sol when drawing usage from the subscription. | 随这次调整，用订阅额度时，我们不收 GPT-6.1 Sol 的长上下文费。 |
| 45 | Prompts with more than 272K input tokens are priced at 2x input and cache rates and 1.5x output for the full request. | 输入超过 272K token 的请求，整次请求按2倍输入与缓存价、1.5倍输出价计费。 |
| 46 | extended sessions that require the agent to hold more context will use significantly more per message. | 需要智能体保留更多上下文的长会话，每条消息的消耗会明显更多。 |
| 47 | an open-source, clean-room reimplementation of Adobe Photoshop, rebuilt in pure Rust. | 一个开源、以“净室”方式重新实现的 Adobe Photoshop，用纯 Rust 重写。 |
| 48 | PhotoCraft is in early alpha | PhotoCraft 处于早期 alpha 阶段 |
| 48 | it is not yet a Photoshop replacement for daily professional work. | 它还不能替代 Photoshop 做日常专业工作。 |

## 视觉复核

第一轮：WSL Codex（gpt-6-astra，medium，只读），附 18 张图（省流卡、速览图、8 张批注图及底图），北京 10/12 约 17:50。结论 PASS / no blocker，3 条 SHOULD_FIX 全部采纳：

- `41` 底图末行 “abstract.” 被截：用 `m-shot.mjs` 重截，止于第三层开头的上一行间（PAD 1 px）。
- `44` 底图无作者栏：重截含 “Tibo @thsottiaux” 的完整自回复卡片。
- `45` 计费条件字号过小：以 480 CSS px 窄窗重截（字号相对放大），OCR 对 “272K” 置信度低，改用 Tesseract 词框量出的 rects。
- NICE_TO_HAVE（型号与数值跨行断开）：未改；省流卡改稿后 OpenAI 条已无跨行型号问题之外的影响。

其后省流卡因正文修订重渲（见下）。

## 反向核验

第一轮：codex-reviewer（WSL Codex gpt-6-astra，medium，只读，带检索），北京 10/12 约 18:00，结论 FAIL。逐条处理：

1. BLOCKER，“API 照旧加价”漏掉 >272K 门槛：采纳。正文省流改“API 输入超 272K 仍整次加价”，吠点①补 “token”；省流卡删去该半句（版面放不下完整条件，宁缺不写半截），API 门槛见批注图 `45`。
2. SHOULD_FIX，Qwen 速览丢了样本选择口径：采纳；第二轮仍嫌概率定义与署名不足，40 字内写不全，删去该条（存档仍在 `d-d5-*`）。
3. SHOULD_FIX，Vals 速览缺任务范围：采纳，补“网页应用”，删“四组”。
4. SHOULD_FIX，“实验室内部”原文无“实验室”：采纳，改“内部”（正文、省流卡、核验表）。
5. SHOULD_FIX，封面副标题“Karpathy粗估”把“见过真AI”算到他名下：采纳，两张封面副标题改为“Karpathy：内部使用者”（图像编辑，见 `images/README.md`）。
6. SHOULD_FIX，“外界无法核实”过强：采纳，改“只是描述，未给可查证据”。
7. SHOULD_FIX，“用 AI 做了7款”应为开发者自述：采纳，正文与省流改“开发者称用 AI/Opus-5.5”，记 L2；省流卡后半句明确主语为 PhotoCraft。
8. SHOULD_FIX，Adobe 回应与合规状态过度认证：采纳，改“本号未见 Adobe 回应，不判断是否合规”，核验表注明检索范围。
9. NEEDS_VERIFICATION，1614/1045 无可重算存档：采纳，补 `m-photocraft-commits.tsv`（逐提交 sha、时间、尾注标记），按 05:20Z 截点重算得 1614/1045；正文改为“多数提交”。

第二轮：同一 codex-reviewer（WSL Codex gpt-6-astra，medium，只读，带检索），北京 10/12 约 18:58，结论 PASS / no blocker。残留处理：

- SHOULD_FIX，Qwen 概率口径与署名：删去该速览条。
- SHOULD_FIX，核验表旧 L1 行与 L2 行冲突：删去旧“省流：开源项目 ArtCraft 用 AI 做了……”行，L1 行只认证 7 个对标仓库，AI 开发归 L2 行。正文不点开发者网名（HN 与 GitHub 身份对应为推断）。
- SHOULD_FIX，Adobe 混合行整体打勾：拆为“本号未见 Adobe 回应”（⚠️ 有限检索）与“不判断是否合规”（编辑立场）两行。
- NEEDS_VERIFICATION，尾注标记无法离线重算：补 `m-photocraft-trailers.tsv`（每个 sha 的 Co-Authored-By 原文行）。
- NICE_TO_HAVE，动态统计无时点：正文补“截至10月12日”。
