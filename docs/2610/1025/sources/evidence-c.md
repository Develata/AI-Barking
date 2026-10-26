# 1025 期 C 组取证：速览（C1–C9）

取证时刻：2026-10-26 20:30–21:10（北京时间，UTC+8）。执行方模型 ID：claude-haiku-5-5。
存档：`sources/c-*`（html/pdf + txt + headers；抓取记录见 `c-fetch-log.jsonl`，逐次过程见 `capture-log-c.md`）。本组无截图。
HN/Reddit 热度：`c-heat-part1.json`（HN 首批 8 组）与 `c-heat.json`（HN 第二批 7 组）；时刻为北京时间。`barking heat` 只做 HN 与 Reddit 检索，不取 X。

## 一、逐条事实

| # | 一句事实（照原文措辞，保留限定词） | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 | IBM CEO Arvind Krishna 在 Q3 财报电话会上称，开放权重模型“anywhere from 25 percent to 10 percent of the token cost of the frontier models”，且“they’re now 90 percent as effective” | L4（The Register 报道电话会问答；IBM 原稿未取到） | https://www.theregister.com/systems/2026/10/22/ibm-ceo-says-open-weight-models-are-now-irresistibly-good-and-cheap-on-prem/5304640 （存档 `c-c-register.html/.txt`）；IBM 3Q26 prepared remarks https://www.ibm.com/downloads/documents/us-en/17f55bddaac4eb80 （存档 `c-c-ibm-q3-remarks.pdf/.txt`） | Register：“The CEO said open weight models currently deliver “anywhere from 25 percent to 10 percent of the token cost of the frontier models. That is very attractive given that they’re now 90 percent as effective.”” | Register 发布 2026-10-22 04:26:39Z（北京 10/22 12:26；JSON-LD）。“token cost”是 token 成本，不是总成本。“90 percent as effective”未具名基准。IBM 3Q26 prepared remarks 全文 grep 无 frontier / open weight / 90% 字样，该句出自问答，未取到电话会转录 | 未找到一手来源（IBM 原稿与转录未取到；L4 可用“据 The Register 报道”） |
| C1a | Register 原文另一处：Krishna 称 2024 年时差距是“70 to 20 or 30”，2025 年末为“70 to 50”（为假设性“broad-based”评测示例） | L4 | 同上 | “If you ran some assessment, some benchmark, and it was broad-based, not narrow, the top model might score a 70, and the open model might score a 20 or a 30.” / “In late 2025, we began to see that gap shrink, where maybe the gap was not 70 to 20 or 30, it was 70 to 50,” | 同上 | 部分支持（“90%”的依据是口头举例，不是具名评测） |
| C2 | 哈佛 Gary King 与 Steven Worthington 工作论文称，AI 文章审稿意见“dominate”人类审稿（rapidPeer 高质量人工审稿） | L1（作者页面与论文 PDF，非期刊同行评审） | 论文页 https://gking.harvard.edu/peerless/ （存档 `c-c-peerless-page.html/.txt`）；PDF https://gking.harvard.edu/files/peerless.pdf （存档 `c-c-peerless-pdf.pdf/.txt`） | 摘要：“We show that AI-generated article reviews are now better, by every measure we could construct, than reviews written by well-compensated human experts and judged to be of unusually high quality by journal editors and authors.” / 正文：“The corpus includes every eligible submission to RapidPeer, including 124 scholarly paper submissions and 134 human reviews (10 submissions received two independent reviews) collected between 2019 and 2026” / “For each submission, we generated a single AI review with Claude Opus 4.8 … at its maximum reasoning setting” / “A model from a different family, GPT-5.5 (OpenAI, 2026), served as the AI judge” / 编辑样本：“three former editors … to independently judge 10 randomly selected and blinded review pairs” / 结果：“It wins in 100% of the pairs overall when judged by another AI model (132 of the 132 decided pairs) and 87% overall when judged by human journal editors (Panel A).” | PDF 首页标 “October 25, 2026”；作者自述 rapidPeer “a service developed by Harvard University’s Institute for Quantitative Social Science”（King 为 IQSS 主任）。原文正文写 132/132 decided，图 1 写 “AI (134/134)”，两处不一致（见勘误） | 已找到 |
| C2a | 作者自述的局限（原句） | L1 | 同上 | “We did not measure the degree to which subsequent revisions improved the paper or the collective consequences of switching to AI reviewing.”（引言末段）；“our results do not speak to the second purpose, which is on what basis editors should make publication decisions”（第 4 节） | 论文未声明已同行评审；人工编辑样本仅 3 人 × 10 对 | 已找到 |
| C3 | r/opensource 版主发布新规：不收用 AI 或面向 AI 的项目，并设项目年限、贡献者人数、账号年龄等门槛 | L1 为版主原帖（r/opensource 版主以个人身份署名的版规帖）；Verge 为 L4 | https://www.reddit.com/r/opensource/comments/1xelcuf/your_tokens_arent_valuable_no_more_ai/ （存档 `c-reddit-opensource-1xelcuf.json`，opencli reddit read 取得）；The Verge https://www.theverge.com/ai-artificial-intelligence/1018217/reddit-opensource-ai-ban （存档 `c-c-verge.html`） | 原帖标题：“Your tokens aren't valuable: No more AI.”；规则原句：“Projects advertised here must be all of: 1 year old (not just age, but actively maintained), have multiple contributors (subjectively 3+), … not be intended for AI (such as MCPs), and have an existing community.”；“1 year account age minimum, with significant Reddit contributions.”；“If it's obvious you're using AI in your project, we don't want it. Tokens are cheap, and we don't care how many you spent to make your amazing project.”；“If you don't speak English, you need to find a different community. We cannot distinguish between AI-written and AI-translated.” | 原帖发布时刻：opencli 未返回 created_utc，只能按 HN/Verge 推定为 10/24 前后（未核）。Verge 标题原句：“r/opensource just banned AI, non-English speakers, and more”（Verge 发布 2026-10-25 13:25 UTC，北京 10/25 21:25）。取证时原帖仍在；返回字段未含 edited 标记，是否修改未核 | 已找到（原帖与 Verge 均在） |
| C3a | 社区规模 | L1（Reddit 官方页面数据，经 opencli subreddit-info） | https://www.reddit.com/r/opensource/ | Subscribers：394350（取证时刻） | 取证时刻 2026-10-26 约 20:45（北京） | 已找到 |
| C4 | Wiz 披露 Gheist 活动：自 2026-09-15 起跟踪，已回收 13,500+ 个被植入的分支，来自 850+ 个失陷账户 | L1（Wiz 官方博客） | https://www.wiz.io/blog/gheist-attempted-scaled-abuse-of-stolen-github-tokens-stumbles （存档 `c-c-wiz.html/.txt`） | “Since September 15, 2026, Wiz Research has been tracking Gheist: a new campaign performing mass abuse of compromised GitHub tokens.”；“We have recovered more than 13,500 planted branches from more than 850 compromised accounts, ranging from personal projects to enterprise organizations' private repositories.”；“A recent GitHub control has been blocking the exploitation in public repositories, but we've observed successful exploitation and follow-on activity in private repositories and GitHub enterprise environments.”；副标题：“has been significantly disrupted by recent GitHub security improvements.” | 发布 2026-10-25（页面）。“As of October 24”统计截止。“roughly ⅔ of implicated accounts are no longer available on GitHub.” 13,500 是植入分支数，原文未给窃密成功次数。“stumbles”只出现在标题里，正文没有这个词 | 已找到 |
| C4a | 攻击起始与 AI 关联 | L1 | 同上 | “Tool B uses a base36 suffix … within hours of its first run on September 28”；凭证搜索列表含 “31 token formats (including keys for AI providers such as Anthropic, Hugging Face, Groq and NVIDIA)” | 攻击起始：Wiz 跟踪自 9/15，Tool B 首次运行 9/28。与 AI 的关系仅限于窃取 AI 服务商的凭证，不是 AI 驱动 | 已找到（与 AI 无直接关系，需 Claude 决定收不收） |
| C5 | JetBrains 开源 ThinkRail，README 自称 “the agentic IDE that gets better every time you use it”，构建于 pi coding agent 之上，支持多个并发 pi 会话与子智能体 | L1（JetBrains 官方 GitHub 仓库） | https://github.com/JetBrains/thinkrail （存档 `c-c-thinkrail.html/.txt`）；API https://api.github.com/repos/JetBrains/thinkrail （`c-c-hf-thinkrail-api.json`） | README：“ThinkRail is the agentic IDE that gets better every time you use it — learning your project through living specs today, and building its own capabilities tomorrow.”；“We built a rich interface around the pi coding agent”；“Several concurrent pi sessions per workspace … delegate to subagents”；仓库描述：“Vibe code with pi in a lightweight, real IDE that customises itself around the way you work — The Vibe You Need” | 许可证 Apache-2.0（API spdx_id Apache-2.0）；仓库 created_at 2026-07-08T07:40:43Z（北京 15:40）；main 最早一次提交 2026-06-22T16:58:26Z（author date，“initial specs”，北京 06-23 00:58），早于仓库创建，说明仓库由既有历史迁入（未核）。Releases 页最近三条均为 nightly：v0.3.0-nightly.4 2026-10-22T21:38:15Z、.5 2026-10-23T07:51:45Z、.6 2026-10-25T06:49:25Z。未找到稳定版首发日期。Stars 2320（API，取证时刻） | 已找到（“多智能体开发环境”是派工单用语，README 原文用 agentic IDE / subagents，正文用语应以原文为准） |
| C6 | 南北阁（Nanbeige）开源 Nanbeige4.3-33B：Apache 2.0、33B、512K 上下文；模型卡自测 Terminal-Bench 4.0 23.8 对 Qwen3.8-27B 6.3，SWE-Bench Pro 60.3 对 61.7 落后 | L1（模型卡为作者自发布；成绩为作者自测） | https://huggingface.co/Nanbeige/Nanbeige4.3-33B （存档 `c-c-nanbeige.html/.txt`）；HF API https://huggingface.co/api/models/Nanbeige/Nanbeige4.3-33B （`c-c-nanbeige-api.json`） | 卡片：“Delivering state-of-the-art agentic performance among open models in the ~30B parameter class”；“We compare Nanbeige4.3-33B with recently released frontier models at around the 30B-parameter scale: Qwen3.8-27B, Muse-Glimmer-30B, and Gemma4-31B. All models are evaluated at their highest thinking effort. For each benchmark, we use the same inference hyperparameters and harness across models.”；表：Terminal-Bench 4.0 为 23.8 / 6.3 / 1.6 / 0.0（Nanbeige / Qwen3.8-27B / Muse-Glimmer-30B / Gemma4-31B；数值前的脚注编号 3 在原文本里粘连，已剥离）；SWE-Bench Pro 原表数值为 60.3 / 61.7 / 36.9 / 51.2（列位同上；表格文本里 60.3 前面的 “3” 是脚注编号粘连，取值以原表为准，见 `c-c-nanbeige.txt` 第 280 行）；“The model supports a context length of up to 524,288 tokens (512K).” | HF 元数据：license apache-2.0；参数 BF16 33,118,977,024（约 33.1B）；architectures LlamaForCausalLM；卡片未写 “dense”，稠密为依据架构推断（未核）。createdAt 2026-10-24T02:49:18Z（北京 10:49），lastModified 2026-10-25T11:11:17Z（北京 19:11）。卡片表注：Terminal-Bench 4 为 mini-swe-agent v2.4.6；Terminal-Bench 2.1 为 Terminus 2 | 已找到（成绩为自家评测） |
| C7 | Meta 新商业条款生效日推迟到 11/20（原定 10/30） | 无官方来源取到；L4/L5 检索结果仅报 10/30 | Meta 官方页未取到（meta.com 博客为 2026-01 旧文，无关；WebSearch 未返回官方条款页） | 检索得到的报道（截至 10/20）均称生效日为 10/30；未见推迟到 11/20 的材料。另有报道称争议条款已删除，“train”一词自美国英文版删除（L4：Nieman Lab、TechCrunch 10/17 转述，未核原文） | 未找到一手来源（推迟日期）。与 WorkBuddy“条文未改”相反的转述见勘误 | 未找到一手来源 |
| C8 | CapCut 官方称，其 ChatGPT 插件“coming soon”，将在 CapCutWorldLA 推出 | L1（CapCut 官方 X 账号 @capcutapp） | https://x.com/capcutapp/status/2114062937094823955 （fxtwitter JSON：`c-c-capcut-fx.json`，帖子卡片截图未做，C 组不截图） | “We’re excited to announce our new CapCut plugin is coming soon to @ChatGPT at CapCutWorldLA.”；“The plugin brings CapCut's video editing and generation tools alongside ChatGPT.” | 原帖 UTC 2026-10-24 18:34:16（北京 10/25 02:34）。取证时 views 1,400,944、likes 7,513、retweets 771、replies 827（fxtwitter，北京 10/26 20:33）。同站 CapCut 页面 https://www.capcut.com/tools/capcut-x-codex （`c-c-capcut-codex.html/.txt`）已有“Install the plugin in Codex”的安装说明，与“coming soon”的关系原文未说明 | 已找到（限定词 “coming soon” 必须保留） |
| C9 | 菲尔兹奖得主 Richard Borcherds（UC Berkeley）不再按正确性批改作业，改为按是否交作业评分 | L4（The Daily Californian 采访报道） | https://www.dailycal.org/news/campus/academics/ai-use-leads-uc-berkeley-math-professor-to-see-no-point-in-grading-homework/article_bdf1eae2-c08b-491f-b9f1-81a7cea177de.html （存档 `c-c-dailycal.html/.txt`） | “Richard Borcherds, a UC Berkeley professor and Fields medalist, no longer grades his students’ homework.”；“he felt there was “no point” in paying readers to grade for correctness in homework, as AI use is now unavoidable.”；“Borcherds said he currently grades on completion if a student hands in their homework. He admitted that it is “not a very good system,” but at the moment, he has no alternative.” | 文章 Oct 23, 2026（页面）；HTML 元数据 datePublished 2026-10-23T06:15:00-07:00（北京 10/23 21:15）。评分口径：按完成情况（on completion）。作者为学生报纸记者，引语为采访转述 | 已找到 |

## 二、各条热度（barking heat，北京时间）

| # | 平台与主帖 | 分数 / 评论 | 时刻（北京） | 来源文件 |
|---|---|---|---|---|
| C1 | HN 50238088，“IBM CEO says open weight models are now good and cheap on-prem”（指向 Register 同一篇） | 317 / 220 | 10/26 09:47（提交） | `c-heat-part1.json` |
| C1 | HN 50191944（早期重复提交） | 16 / 7 | 10/22 14:33 | `c-heat-part1.json` |
| C2 | HN 50208384，gking.harvard.edu/peerless/ | 2 / 0 | 10/23 21:05 | `c-heat-part1.json` |
| C2 | HN 50210159，PDF 版 | 4 / 0 | 10/23 22:57 | `c-heat-part1.json` |
| C3 | HN 50231722，“R/open source bans all solo projects and projects using LLMs”（指向原帖） | 88 / 111 | 10/25 22:05 | `c-heat.json` |
| C3 | HN 50231744，The Verge 链接 | 3 / 3 | 10/25 22:06 | `c-heat.json` |
| C3 | Reddit 原帖 1xelcuf（opencli reddit search 返回） | 原帖 score 返回 0（字段未公开，未核），comments 2,580 | 取证时 | `c-reddit-opensource-1xelcuf.json` |
| C4 | HN 50235010，Gheist 原文链接 | 47 / 0 | 10/26 03:10 | `c-heat.json` |
| C5 | HN 50233294，“JetBrains Thinkrail”（指向仓库） | 221 / 164 | 10/26 00:27 | `c-heat-part1.json` |
| C5 | HN 50219186，“ThinkRail from JetBrains”（指向 thinkrail.ai，已 curl 失败，000） | 57 / 83 | 10/24 16:08 | `c-heat-part1.json` |
| C6 | HN 50231099，HF 模型页 | 3 / 0 | 10/25 21:01 | `c-heat-part1.json` |
| C7 | HN 未找到相关讨论 | — | — | — |
| C8 | HN 未找到（CapCut 关键词 0 条） | — | — | `c-heat.json` |
| C9 | HN 50233304，Daily Cal 原文 | 221 / 320 | 10/26 00:28 | `c-heat.json` |

## 三、扫描说法勘误

1. **C1 时间**：派工单写 Register 为 10/22 05:26 UTC；Register 页面 JSON-LD 为 2026-10-22T04:26:39Z（北京 12:26）。以原页面为准。
2. **C1 热度**：派工单写 HN 299 分 / 205 评（10/26 01:47Z）；取证时刻 HN 为 317 分 / 220 评（北京 10/26 09:47 提交）。
3. **C1 数字口径**：“10%–25%”的原句是 “25 percent to 10 percent of the token cost”，指 token 成本，不是总成本；“90 percent”是 “as effective”，未具名基准。Register 文中另有一句 “90 percent of the performance at ten percent of the price”，那是 Register 的导语式引语（pull quote），不是 Krishna 原句，不能当作原话。
4. **C1 来源**：IBM 3Q26 prepared remarks 中没有这句话；问答段的转录未取到，所以 IBM 一手原话未找到。
5. **C2 日期**：WorkBuddy 称“10/22 前后”；论文 PDF 首页标 “October 25, 2026”，但 HN 最早提交为 10/23 21:05（北京）。两个日期不一致，可能存在更早版本，未核。
6. **C2 分母**：正文写 “132 of the 132 decided pairs”，图 1 写 “AI (134/134)”，同一原文内不一致。WorkBuddy 的 “132 组全选 AI” 对应正文。
7. **C2 同行评审**：WorkBuddy 称“未同行评审”。原文未声明同行评审状态，也未声明未经同行评审；只能说原文未写。
8. **C2 其余核对**：124 篇稿件、134 份人工审稿、Claude Opus 4.8（最大推理档）、GPT-5.5 作裁判、三名前编辑 30 次盲评选 AI 26 次（26/30）、rapidPeer 由 IQSS 开发，均与原文一致。
9. **C3 规则条数**：派工单写“四条规则”。原帖实际列出五条门槛（项目年限与贡献者、账号年龄、明显用 AI 即不收、非英语者不收、临时封禁与“永久不欢迎”）。
10. **C3 媒体**：Kotaku、Help Net Security 10/26 报道未找到（检索未返回）。The Verge 报道存在（2026-10-25 13:25 UTC）。
11. **C4 “stumbles”**：该词只出现在标题，正文无此词。正文的说法是 “significantly disrupted by recent GitHub security improvements”。
12. **C4 AI 相关性**：攻击本身与 AI 无直接关系，只是凭证搜索列表含 AI 服务商的 key，需 Claude 决定收不收。
13. **C5 热度**：派工单写 HN 约 210 分 / 157 评；取证时 HN 为 221 分 / 164 评（北京 10/26 00:27）。
14. **C7 “条文未改、只是生效日推后”**：与 L4/L5 转述相反（转述称争议条款已删除，“train”一词在美国英文版已删）。推迟到 11/20 未找到任何材料，最新报道仍写 10/30。两点都未经 Meta 原文核对。
15. **C8 日期**：派工单写 10/24；X 帖 UTC 时间为 10/24 18:34，北京为 10/25 02:34。
16. **C9 热度**：派工单写 HN 约 217 分 / 319 评；取证时为 221 分 / 320 评（北京 10/26 00:28）。评分口径是“按完成情况”（on completion），不是“改成按什么评分”的其他标准。

## 四、未找到 / 失败项

- C1：IBM 电话会转录未取到（搜索未返回 Q3 2026 转录）。
- C3：Kotaku、Help Net Security 报道未找到；原帖 created_utc 未取到。
- C7：Meta 官方条款页与公告未取到；推迟到 11/20 未找到。
- C5：thinkrail.ai（curl 000，失败），JetBrains 官方博客未核。
- C6：未核 “dense” 的卡片文字（只有架构推断）。
- Nieman Lab 页面 curl 失败（000），未入库。

## 五、假设

- 北京时间 = UTC+8；HN 与 Reddit 的时刻取自 `barking heat` 输出的 `time_bj` 字段。
- “一手来源”对 C5、C6 指仓库或模型页面（作者自发布），对 C3 指版主原帖，对 C8 指官方账号。
- C3 的 score 0 视为未公开字段，不当作真实分数。
