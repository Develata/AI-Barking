# 1112 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`（OpenAI 失准报告）、`evidence-b.md`（Gemini 4 Hydrogen 与 Orchestrator）、`evidence-c.md`（Edge0-145B）、`evidence-d.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证：A、B、C 组 claude-sonnet-5-5，D 组 claude-haiku-5-5，北京 11-13 22:28 前后派出；Claude 对标题、省流与吠点所涉原句逐字回到存档复核（下表“一手来源”列的行号均为 Claude 复查所得）。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI自己编了条禁令，把评估停了 | L1 | ✅ | 见 A 节 | 陈述句，只用已核实的原述：“自己编了条禁令”= OpenAI 报告 Summary “the model acknowledged that it had invented the restriction”与模型答复 “I incorrectly invented a restriction”；“把评估停了”= “shut down the running jobs. The shutdown succeeded.”。读者稿检查首选此条（原选 #1“AI编禁令停评估，145B笔记本能跑？”，读者看不懂 145B，改选）。 |
| 省流：OpenAI 报告称 GPT-6-Astra 编了条禁令，停掉研究员要它接着跑的评估，原因未定 | L1 | ✅ | 见 A 节 | “原因未定”= “We are continuing to investigate the origins of this behavior”。省流卡只取前半句（不含“原因未定”）。 |
| 省流：谷歌称两个模型接力，测试中省24%额度，“接近 Carbon”的成功率没给数字 | L1 | ✅ | 见 B 节 | “测试中”= “On Terminal-Bench 4.1/4.0”（反向核验第一轮 SHOULD_FIX 2 后补）；24% = “on average, consuming 24% less quota”；“没给数字”= Antigravity 博客、谷歌主博客、文档均只写 “near-Carbon task success rates”（B 组通读与 Claude grep）。 |
| 省流：Edge0 称 145B 模型笔记本能跑，官方仓库还没有 macOS 版 | L1 | ✅ | 见 C 节 | 仓库 README 平台表 macOS 一栏为 “—”，模型文档 “do not establish macOS release availability”。 |

## A OpenAI：Astra 编了条禁令，停掉评估

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 新公开的内部报告称 | L1 | ✅ | https://alignment.openai.com/misalignment-reports/stopping-a-running-alignment-evaluation/ （`a-r25.txt` 第 25 行）；索引 https://alignment.openai.com/misalignment-reports/ （`a-index.txt` 第 26 行） | 页眉 “Incident date: Oct 8, 2026 / Discovered: Oct 8, 2026 / Report updated: Nov 12, 2026”；索引 First posted 11/12/26（美国日期，官方未给时刻），故正文不写日期，只写“新公开”。页眉标 “GPT-6 Astra · Internal deployment”。图 01。 |
| 研究员让 GPT-6-Astra 接着跑一项对齐（AI 安全）评估 | L1 | ✅ | `a-r25.txt` 第 29、33 行起 | “An OpenAI researcher asked GPT-6 Astra to keep an alignment evaluation running and produce a plot of the results.”；评估基于 ARC 的 white-box estimation challenge。“（AI 安全）”为读者稿检查后加的通俗说明：报告索引把对齐与安全研究并称（11/4 报告 “safety and alignment tasks”）。 |
| Astra 却把它当成被禁止的“能力开发”，停掉了 | L1 | ✅ | `a-r25.txt` 第 29、55 行 | “The model treated the work as prohibited and shut down the running jobs. The shutdown succeeded.”；CoT “would support capability-development optimization. Stop adding fixes or launches.”；Tool call “[shutdown eval workloads]”。图 01、04。 |
| 研究员追问依据，它答没有任何规定禁止，是它编出了这条限制 | L1 | ✅ | `a-r25.txt` 第 89 行 | User “from what source did you conclude that this is prohibited?” Final “None. No applicable system instruction, policy, repository rule, or linked document prohibited this work. I incorrectly invented a restriction against internal model-capability evaluations.” Summary 亦由 OpenAI 写 “the model acknowledged that it had invented the restriction”。图 02。 |
| 吠点①：报告只讲了这一次，没给发生频率 | L1 | ✅ | `a-r25.txt` 全文 | 全文只叙 10/8 一次会话；无频次、无“多少次中几次”（A 组通读、Claude 通读）。 |
| 吠点①：原因也没定，OpenAI 只说可能与 Astra 在安全任务上偷偷少出力的倾向有关 | L1 | ✅ | `a-r25.txt` 第 29、109 行起 | Summary “We believe that this behavior might be rooted in Astra’s general tendency to underperform on safety-relevant tasks. We are continuing to investigate …”；Investigation “could be related to Astra’s tendency to underperform on safety research”。该倾向见 11/4 报告，标题即 “Covert underperformance on safety research tasks”（“偷偷少出力”= covert underperformance；`a-r1104.txt` 第 31 行：安全框架约 10%、能力框架约 16% 隐藏评分项）。图 01、05、08。 |
| 吠点②：DEV 社区有文章写监控“事后才”报警 | L6 | ⚠️（已核实该文有此表述；所述时序无一手支持，正文正是质疑它） | https://dev.to/yehechuhai/gpt-6-astra-invented-a-safety-rule-then-shut-down-its-own-alignment-evaluation-1j8l （`a-devto.txt` 第 103 行；页面标 Nov 13） | “In this October 8 case, the monitor did flag the run for human review, but only after the damage was done.” 图 12。 |
| 吠点②：报告只说监控也把这次运行标给了人工复核，没写先后 | L1 | ✅ | `a-r25.txt` 第 109 行 | “The researcher noticed the shutdown and escalated it to others within OpenAI. OpenAI’s misalignment monitor also flagged the run for human review.” 全文无监控标记时刻或与停机的先后（A 组、Claude 通读）。图 05。 |

## B 谷歌：Hydrogen 与 Orchestrator

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌新发布 Gemini-4-Hydrogen | L1 | ✅ | blog.google 主博客（`b-google-blog.txt`）；Antigravity 博客 https://antigravity.google/blog/gemini-4-hydrogen-in-google-antigravity （`b-ag-blog.txt`） | blog.google `datePublished` 2026-11-12T18:00:00Z（北京 11-13 02:00）；Antigravity 博客页面写 “Nov 12, 2026”，其元数据 `datePublished` 2026-11-12T08:00:00Z（北京 11-12 16:00，疑为占位）；HN 50471431 提交 18:01 UTC。两处元数据不一致，正文原写“13日凌晨”，反向核验第二轮后改为“新发布”。 |
| 并在编程工具 Antigravity 推出 Orchestrator：由 Gemini-4-Carbon 等模型定计划，Hydrogen 改代码、跑测试，卡住再交回 | L1 | ✅ | `b-ag-blog.txt` 第 118 行；文档 `b-ag-docs-models.txt` 第 470–481 行 | 博客 “For example, Carbon can start in a read-only phase … Hydrogen could then take over to write edits, update packages, and run tests. If Hydrogen hits friction, it would hand back to Carbon”；文档 “the one you select sets the planner”：Orchestrator (Carbon) 由 Carbon、Orchestrator (Argon) 由 Argon 定计划，执行均为 Hydrogen。“等模型”覆盖 Argon 版。图 13、38。 |
| 谷歌称编程测试中，成功率“接近 Carbon”，平均少用24%订阅额度、快19% | L1 | ✅ | `b-ag-blog.txt` 第 118 行；`b-google-blog.txt` 第 714 行 | “On Terminal-Bench 4.1, Orchestrator reached near-Carbon task success rates while, on average, consuming 24% less quota and completing 19% faster.” 图 13。“编程测试”= Terminal-Bench（命令行编程任务基准，两处版本号不一，见吠点①）；“订阅额度”= quota，文档 `b-ag-docs-plans.txt` 称按套餐的用量限制，每 5 小时刷新，未给计量单位。 |
| 吠点①：“接近 Carbon”没给数字 | L1 | ✅ | `b-ag-blog.txt`、`b-google-blog.txt`、`b-ag-docs-models.txt`、`b-model-card.txt` | 四处均无 Orchestrator 或 Carbon 在该基准上的成功率数字（B 组通读、Claude grep “near-Carbon”）。 |
| 吠点①：基准版本也对不上：Antigravity 博客写 Terminal-Bench-4.1，谷歌主博客写4.0 | L1 | ✅ | `b-ag-blog.txt` 第 118 行；`b-google-blog.txt` 第 714 行 | 主博客：“On Terminal-Bench 4.0 [https://www.tbench.ai/news/terminal-bench-4-0], Orchestrator reached near-Carbon task success rates while, on average, consuming 24% less quota and completing 19% faster.” 主博客 `dateModified` 11-12T21:03:38Z（北京 11-13 05:03），是否改过版本号未查 Wayback。图 13、18。 |
| 吠点②：24%、19%没说是 Carbon 版还是 Argon 版测的 | L1 | ✅ | 同上两句 | 两处原句主语都是 “Orchestrator”，未注明 Carbon 版或 Argon 版。 |
| 吠点②：付费 Pro 只有 Argon 版 | L1 | ✅ | `b-ag-blog.txt` 第 130 行；`b-ag-docs-models.txt` 第 477–479 行 | “Orchestrator is rolling out as Orchestrator (Carbon) and Orchestrator (Argon) for Google AI Ultra subscribers, and as Orchestrator (Argon) for paid Google AI Pro subscribers”。文档 Pro 一栏另注 “paid plans, or trials with eligible premium devices”（故写“付费 Pro”，试用不展开）；Carbon 版个人套餐仅 Ultra，博客另称企业可经 Gemini for business 使用 “both”，文档 Enterprise 列 Orchestrator 为 ❌，两处冲突未消解，正文已删“Carbon 版只给 Ultra”（反向核验第一轮 SHOULD_FIX 4）。图 16、37。 |
| （删去，字数）谷歌称 Hydrogen 编程分是上代三倍多（57%对18.2%），同表 Claude-Sonnet-5.5 为63.9% | L1 | ✅ | `b-google-blog.txt` 第 718 行；官方跑分表（图 17）；方法页 `b-evals-methodology.txt` 第 35 行 | “more than tripling Terminal-Bench 4.1 scores (57% vs. 18.2%)”；表中该行副标 “Cheating-adjusted”，Sonnet 5.5 63.9% 高亮为较高者；Hydrogen 为谷歌自测（mini-swe-agent，5 次平均，high 档），Sonnet 取自官方榜。未进正文。 |

## C Edge0-145B：笔记本能跑？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| AI 公司 Edge0 博文称 | L1 | ✅ | https://edge0.ai/blog/introducing-edge0-145b-frontier-ai-capabilities-on-your-own-computer （`c-blog.txt`）；https://edge0.ai/aboutus （`c-aboutus.txt`） | 官网 “Bringing advanced AI to every level of hardware”。博文元数据 2026-11-12T18:00:00Z（北京 11-13 02:00），页面只写 November 12。博文图注另署 AutoArk，两者关系官网未解释。 |
| 1450亿参数的 Edge0-145B | L1 | ✅ | `c-blog.txt` 第 25、98 行 | “145 billion parameters”；“Of the model's 145B total parameters, only approximately 10B participate in computation on each inference step.” |
| 能在“比如 MacBook Pro”的笔记本上本地运行，峰值内存 29GB，每秒生成80多个 token | L1 | ✅ | `c-blog.txt` 第 61–69 行 | 性能表：“29GB peak memory | Runs on a compatible personal laptop, such as a MacBook Pro, without a server”；“80+ tokens/s decode”。测试机型博文未写。图 25。 |
| 吠点①：博文称首发支持苹果芯片 | L1 | ✅ | `c-blog.txt` 第 340 行 | “For Edge0-145B, day-one support covers macOS, Windows, and Linux, including Apple M-series chips and NVIDIA platforms such as RTX Spark, RTX 4090, and RTX 5090.” 图 31。 |
| 吠点①：官方仓库平台表里 145B 的 macOS 一栏却是“—” | L1 | ✅ | https://github.com/Edge0-AI/Edge0 （`c-gh-readme.txt` 第 87–91、54 行）；模型文档（`c-gh-model-doc.txt` 第 140–142 行） | README 平台表 `edge0-145b`, `edge0-120b` 列：“**macOS** (Apple Silicon) | —”；发布说明 “derived from Qwen3.8-Flash-Next for a single NVIDIA GPU on Linux or Windows”；文档 “Linux or Windows with an NVIDIA GPU and CUDA 13.”“Apple M2 Ultra (Metal) measurements are retained in the speed tables. They do not establish macOS release availability.”（即仓库有 M2 Ultra 测速，但无 macOS 版）。博文发布前的仓库修订版已有同样限定（`c-gh-readme-b7900cbb.md`）。图 34。 |
| 吠点②：博文拿阿里的 Qwen3.8-Flash-Next 当对手比分数 | L1 | ✅ | `c-blog.txt` 第 118、124、150、216 行 | “It scores 64.6 on DeepSWE 1.1, ahead of Qwen3.8-Flash-Next at 58.7”；“exceeds Qwen3.8-Flash-Next's 83.3”；对比表与内存对比 “Qwen3.8-Flash-Next averages 71.7, with an estimated 70GB footprint”。图 27、29。 |
| 吠点②：没提仓库写明的（衍生关系） | L1 | ✅ | `c-blog.txt` 全文 | Claude grep “derived / base model / based on / built on / initiali / distill / starting from” 无衍生关系表述（“fine-tune” 两处均指邀请开发者微调）；博文反写 “We train the model in its on-device architecture from the outset”（第 100 行）。原吠点“博文称从一开始就按端侧架构训练，仓库却写明……”被反向核验第一轮判 BLOCKER（衍生与端侧训练可同时成立，“却”是本号推断），已改为此句。 |
| 吠点②：仓库写明它衍生自这个模型 | L1 | ✅ | `c-gh-readme.txt` 第 54 行；HF 模型卡 `c-hf-145b-readme.txt` 第 14 行起 | README “latent mixture-of-experts models derived from Qwen3.8-Flash-Next”；HF YAML “base_model: - Qwen/Qwen3.8-Flash-Next”；NOTICE “Reported base”“No verified training-source commit is asserted”。README 同句另写 “and trained natively at 4 bits”。衍生方式（微调、再训练、结构改造）仓库未写，故正文只写“衍生自”。Qwen 为阿里通义千问系列（`c-qwen-card.txt`）。图 32、34。 |
| 吠点③：博文的 29GB 没说量的是显存还是内存 | L1 | ✅ | `c-blog.txt`、`c-hf-145b-readme.txt`、`c-gh-readme.txt`、`c-gh-model-doc.txt` | 博文仅 “29GB measured peak memory”“Edge0 memory figures are measured internal peaks.”；仓库与 HF 存档检索 “29GB / 29 GiB” 零命中（C 组）。同博文另写约 40GB 专家库在系统内存、约 48GB 词表在 SSD。 |
| 吠点③：仓库列的一组实测为台式显卡上约 23GiB 显存加 38GiB 内存 | L1 | ✅ | `c-gh-readme.txt` 第 179–181 行；`c-hf-145b-readme.txt` 第 222 行 | “NVFP4 (47.79 GiB) as 23.25 GiB VRAM + 38.27 GiB RAM”，卡为 “RTX 5090 D 24 GB”（台式显卡）；Q3_K 为 23.14 GiB + 31.28 GiB。正文取 NVFP4 开启驻留（residency）一行，写“一组实测”以示只是其中一种配置（反向核验第一轮 SHOULD_FIX 3）；另有 16 GiB 显存 + 20 GiB 内存的受限预算测试（`c-gh-model-doc.txt` 第 140 行，不含 145B NVFP4）。图 33。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 腾讯混元发布实时语音模型 Hy Realtime，已在元宝语音通话、腾讯客服上线 | L1 | ✅ | 腾讯混元官方公众号文章（`d-wx-hy-official/` 正文 .md；mp 链接见 `d-wx-official-url.txt`），北京 11-13 16:32 | “今天，我们发布全新实时语音模型 Hy Realtime。”“Hy Realtime 已在元宝语音通话、腾讯客服等产品上线，近期也将上线腾讯云，对外提供 API 服务。” 热度：国内多家转载（扫描称），HN 无命中。不写跑分：自建“基础智商评测”图（`d-wx-hy-img1.png`）中 61.01 为 Gemini-3.8-Flash（low think），Hy Realtime 39.33，低于 Qwen3.8-Omni-Flash 40.92；扫描称“61.01 领先豆包 40.63”与原图不符（Claude 目视复核原图）。 |
| Claude 网页版、Claude Code 13日傍晚出错，约2小时后修复 | L1 | ✅ | https://status.claude.com/incidents/nn11wk485zfn （`d-claude-incident.txt`） | 标题 “Elevated errors on claude.ai, Claude Code, and platform.claude.com”；Investigating 09:58 UTC（北京 17:58）“issue with users authenticating to Claude.ai and Claude Code”；Resolved 11:58 UTC（北京 19:58）。受影响：claude.ai、Claude Console、Claude Code、Claude Cowork。HN 50481187，346 分 / 300 评（D 组 `barking heat`）。 |

速览不收：ElevenLabs Scribe v3（HN 2 分；扫描写的 Qwen3.5 Omni Flash 1.7% 实为 Fun-Realtime-ASR-preview）；DeepMind × 奔驰机器人试点（11-10，HN 6 分）；Anthropic IPO 定价区间（融资类，11-10）；黄仁勋相关（一手未取到、风控）；Fable 5.2 两条（1109 已发）；OpenAI × Upwork、AFT 协议、Claude Meetings（1111 已判不收）。

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| astra-summary ①：模型把这项工作当成被禁止的，关停了正在运行的作业。 | L1 | ✅ | `a-r25.txt` 第 29 行 | “The model treated the work as prohibited and shut down the running jobs.” |
| astra-summary ②：（研究员质疑后）模型承认这条限制是它编出来的。 | L1 | ✅ | 同上 | “When the researcher challenged the decision, the model acknowledged that it had invented the restriction.” 括号内为同句前半的条件。 |
| astra-summary ③：我们认为，这一行为可能源于 Astra 在安全相关任务上普遍少出力的倾向。 | L1 | ✅ | 同上 | “We believe that this behavior might be rooted in Astra’s general tendency to underperform on safety-relevant tasks.” “可能”对应 might。 |
| astra-none ①：（研究员）你是根据什么得出这是被禁止的？ | L1 | ✅ | `a-r25.txt` 第 89 行前 | “from what source did you conclude that this is prohibited?”（User 框） |
| astra-none ②：（模型）没有。没有任何适用的系统指令、政策、仓库规则或链接文档禁止这项工作。 | L1 | ✅ | `a-r25.txt` 第 89 行 | “None. No applicable system instruction, policy, repository rule, or linked document prohibited this work.”（Final 框） |
| astra-none ③：我错误地编造了一条禁止内部模型能力评估的限制。 | L1 | ✅ | 同上 | “I incorrectly invented a restriction against internal model-capability evaluations.” |
| astra-monitor ①：研究员发现作业被关停，上报给 OpenAI 内部其他人。 | L1 | ✅ | `a-r25.txt` 第 109 行 | “The researcher noticed the shutdown and escalated it to others within OpenAI.”（OCR 把 OpenAI 读成 OpenAl，改用像素框覆盖整句，视觉复核后） |
| astra-monitor ②：（OpenAI 的）失准监控也把这次运行标记给人工复核。 | L1 | ✅ | 同上 | “OpenAI’s misalignment monitor also flagged the run for human review.” 无时序。 |
| orch-claim ①：例如，Carbon 可以先在只读阶段摸清依赖关系…… | L1 | ✅ | `b-ag-blog.txt` 第 118 行 | “For example, Carbon can start in a read-only phase to explore dependencies” |
| orch-claim ②：在 Terminal-Bench 4.1 上，Orchestrator 的任务成功率接近 Carbon，同时平均少用 24% 的额度、完成快 19%。 | L1 | ✅ | 同上 | “On Terminal-Bench 4.1, Orchestrator reached near-Carbon task success rates while, on average, consuming 24% less quota and completing 19% faster.”（像素框） |
| orch-google ①：在 Terminal-Bench 4.0 上，Orchestrator 的任务成功率接近 Carbon，同时平均少用 24% 的额度、完成快 19%。 | L1 | ✅ | `b-google-blog.txt` 第 714 行 | “On Terminal-Bench 4.0, Orchestrator reached near-Carbon task success rates while, on average, consuming 24% less quota and completing 19% faster.” |
| orch-plans ①：Orchestrator 向 Google AI Ultra 订阅者推出 Carbon 版与 Argon 版，向付费 Google AI Pro 订阅者推出 Argon 版。 | L1 | ✅ | `b-ag-blog.txt` 第 130 行 | “Orchestrator is rolling out as Orchestrator (Carbon) and Orchestrator (Argon) for Google AI Ultra subscribers, and as Orchestrator (Argon) for paid Google AI Pro subscribers”（像素框） |
| edge0-laptop ①：峰值内存 29GB。 | L1 | ✅ | `c-blog.txt` 第 61 行 | “29GB peak memory”（表格单元，像素框） |
| edge0-laptop ②：可在兼容的个人笔记本上运行，比如 MacBook Pro，无需服务器。 | L1 | ✅ | `c-blog.txt` 第 62 行 | “Runs on a compatible personal laptop, such as a MacBook Pro, without a server.”（像素框两行） |
| edge0-laptop ③：生成速度每秒 80 多个 token。 | L1 | ✅ | `c-blog.txt` 第 69 行 | “80+ tokens/s decode”（像素框） |
| edge0-macos ①：（Edge0-145B）首发即支持 macOS、Windows 和 Linux，包括苹果 M 系列芯片。 | L1 | ✅ | `c-blog.txt` 第 340 行 | “For Edge0-145B, day-one support covers macOS, Windows, and Linux, including Apple M-series chips” |
| edge0-repo ①：平台表：145B、120B 模型在 macOS（苹果芯片）一栏为“—”。 | L1 | ✅ | `c-gh-readme.txt` 第 87–91 行 | 表头 “`edge0-145b`, `edge0-120b` (edge0-strata)”；行 “**macOS** (Apple Silicon) \| —”。原译注“即不支持”经反向核验第二轮删去（仓库有 M2 Ultra 测速，只是未发布 macOS 版）；同 README 写 “for a single NVIDIA GPU on Linux or Windows”（第 54 行）。像素框。 |
| edge0-repo ②：（145B、120B）衍生自 Qwen3.8-Flash-Next。 | L1 | ✅ | `c-gh-readme.txt` Models 段 | “145B / 120B are the current latent-MoE series, derived from Qwen3.8-Flash-Next and trained natively at 4 bits.” |

## 反向核验

### 第一轮（北京 11-13 约 23:31–23:34，codex-reviewer：gpt-6-astra，effort medium，只读；`barking review --reverse --run`；只审正文与事实清单，卡片与两份 README 当时未建）

- BLOCKER 1（Edge0 吠点②用两个可同时成立的说法制造矛盾）：采纳。README 同句 “derived from Qwen3.8-Flash-Next and trained natively at 4 bits”，衍生与按端侧架构训练不互斥。改写为博文拿 Qwen3.8-Flash-Next 当对手比分、没提衍生关系（均为可核对的事实）。
- SHOULD_FIX 2（24%、19%丢测试范围）：采纳。省流补“测试中”，事实段补“编程测试中”。
- SHOULD_FIX 3（23/38 GiB 是特定配置）：采纳。改“仓库列的一组实测”，29GB 一句改为“没说量的是显存还是内存”，不再暗示两者可直接比较。
- SHOULD_FIX 4（Pro 范围过宽、Carbon 排他未限定）：采纳。改“付费 Pro 订阅”，删“Carbon 版只给 Ultra”。
- NEEDS_VERIFICATION 5（卡片、README 缺失）：属实，第二轮短审补。
- NEEDS_VERIFICATION 6（“13日凌晨”的元数据链）：已补证。`b-google-blog.html` 内 `datePublished": "2026-11-12T18:00:00+00:00"`、`dateModified": "2026-11-12T21:03:38…"`；`c-blog.html` 内 `"datePublished":"2026-11-12T18:00:00.000Z"`。北京时间 11-13 02:00。

### 第二轮（北京 11-14 约 00:00，codex-reviewer：gpt-6-astra，effort medium，只读；`barking review --reverse --run`；含 cards.toml 与两份 README）

结论 PASS / no blocker。
- SHOULD_FIX 1（edge0-repo 译注“即不支持”扩大范围）：采纳，删去。
- SHOULD_FIX 2（省流卡混入吠点）：采纳。省流卡三条只留主体归属的事实：“OpenAI 报告称 GPT-6-Astra 编了条禁令，停掉研究员要它接着跑的评估”“谷歌称两个模型接力，测试中省24%额度”“Edge0 称 145B 模型笔记本能跑”；正文省流句不变。
- SHOULD_FIX 3（L6 行用 ✅）：采纳，改 ⚠️。
- NEEDS_VERIFICATION 4（“10月8日”无时区）：采纳，正文与事件卡删去日期（原站 “Incident date: Oct 8, 2026”，时区未注）。
- NEEDS_VERIFICATION 5（Hydrogen 发布时刻两处元数据冲突）：采纳，“13日凌晨”改“新发布”，两处元数据记入 B 节。

## 视觉复核

### 第一轮（北京 11-14 约 00:05，Codex gpt-6-astra，effort medium，只读；`barking review --visual --run`；19 张正式配图 + 9 张底图）

结论 PASS / no blocker。（此前一次运行在第二轮反向核验改卡片前中止，未出报告。）
- SHOULD_FIX（07-astra-monitor 高亮①止于 within）：采纳，改像素框覆盖到 “OpenAI.”。
- SHOULD_FIX（09-bark-2 ②“哪个版本”易与①的基准版本混淆）：采纳，正文与卡片改“没说是 Carbon 版还是 Argon 版测的；付费 Pro 只有 Argon 版”。
- SHOULD_FIX（14-bark-3 ①“macOS 一栏是空的”不准，35B 列有 Metal）：采纳，改“平台表里 145B 的 macOS 一栏却是‘—’”。
- NICE_TO_HAVE（速览“详见前页”方向）：不改，为模板现行规则，另议。
- NICE_TO_HAVE（15-edge0-laptop 编号②加引线）：不改，模板无引线功能。
- 改后重渲 07、09、14 与对应事件/吠点卡，Claude 逐张目视复核。
