# 1016 事实核验

取证：A、B、C 组 Sonnet 5.5 子代理，D、E 组 Haiku 5.5（均 effort high），详见 `evidence-a.md`–`evidence-e.md`。选题前 Claude 本人抓取并读过 DeepSeek 更新日志与搜索指南（中英文）、英文价格页、Anthropic 三篇研究原文、The Verge RTX Spark 实测与 Mercor 页（存档在 scratchpad，结论与各组存档一致）。起草时 Claude 回读了下列决定性条目的存档原句与截图：DeepSeek 搜索指南计费节（`../images/02`）、新闻页官方对比表（`../images/05`）、Claude Code 接入页（`../images/08`）、Anthropic ECDSA 原文（`../images/13`）、Valsorda 宣布领奖页（`../images/21`、`22`）、Anthropic 用工研究图 1、图 2（`../images/25`、`27`）、IT 之家标题（`c-news-ithome.txt` 1 行）。

时间：本机美东 EDT（UTC-4），北京时间 = 本机 + 12 小时。

## 主帖一：DeepSeek 搜索要收钱？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| DeepSeek 10月17日把 deepseek-flash 升级为 V4.1-Flash-1017 | L1 | ✅ | https://api-docs.deepseek.com/zh-cn/updates/（`a-updates-zh.txt` 45 行；`../images/01`） | 更新日志“时间: 2026-10-17”，官方未给时刻；官方 X 帖北京时间 10-17 16:35:54（`a-x-deepseek-ai.raw.json`）。“DeepSeek V4.1 Flash 已升级至 DeepSeek-V4.1-Flash-1017，官方网页端与 APP 模型也已同步更新至新版本。API 调用方式与价格保持不变，仍使用 deepseek-flash 即可访问最新模型。” |
| 并开放权重 | L1 | ✅ | https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017（`a-hf-card.txt` 377 行；`a-hf-api.json`） | “This repository and the model weights are licensed under the MIT License”；API 标签 license:mit，safetensors 合计约 763B 参数（OpenRouter 另写 552B，口径不同，正文不写参数量） |
| API 价格不变 | L1 | ✅ | 同第一行 | “API 调用方式与价格保持不变” |
| 同日开放自研搜索 API，Search、Fetch 各按每千次成功请求5元计费 | L1 | ✅ | https://api-docs.deepseek.com/zh-cn/guides/search/（`a-search-zh.txt` 299–305 行；`../images/02`）；新闻页 https://api-docs.deepseek.com/zh-cn/news/news261017（`a-news261017-zh.txt` 101 行） | 表“Search 每 1,000 次成功请求 5元”“Fetch 每 1,000 次成功请求 5元”；英文页 $0.75；“只有返回 HTTP 200 且产生结果的请求才会计费……请求失败、搜索无结果或超时不计费”；各 5 QPS（`../images/03`） |
| 在 DeepSeek Harness、Claude Code、Codex 里用 DeepSeek API 时，服务端工具已在调用搜索，目前不计费，北京时间11月1日起按次收（省流：Claude Code 等客户端经 DeepSeek API 的服务端搜索11月起也收费） | L1 | ✅ | 搜索指南（`a-search-zh.txt` 305 行；`../images/02`）；价格页（`a-pricing-zh.txt`；`../images/10`）；新闻页 103 行（`../images/06`） | “请注意，当前用户在 DeepSeek Harness/Claude Code/Codex 中使用 DeepSeek API 时，已经会通过 Server 端工具调用访问到 Search/Fetch 接口，这部分 Search/Fetch 接口调用目前未计费，并将于北京时间 2026 年 11 月 1 日 00:00 起按次收取调用费用。”更新日志正文未写这句（新闻页写“两周后”）。Codex 接入页：内置联网搜索仅 deepseek-v4-pro 可用，一键脚本默认 `web_search = "disabled"`（`a-codex-zh.txt` 101、301 行；`../images/09`），故正文只点名 Claude Code 的触发方式 |
| 据文档，Claude Code 里搜不搜由模型判断 | L1 | ✅ | https://api-docs.deepseek.com/zh-cn/quick_start/agent_integrations/claude_code（`a-cc-zh.txt` 153 行；`../images/08`） | “如果模型判断您的问题需要通过搜索功能才能满足，模型会调用 Web Search 工具……因此会产生额外的模型 Token 费用。搜索请求本身还会按 Search API 计费规则计费。”页面未写默认开关与能否关闭（A4-a） |
| 官方对比表里的美国模型是 GPT-5.6-Sol 和 Opus-5 | L1 | ✅ | 新闻页官方表格图（`a-img-v4.1_261017_benchmark_table_cn.png`；`../images/05`）；HF 模型卡同表 | 表头：DeepSeek V4.1-Flash-1017、DeepSeek V4.1-Flash、Kimi K3、GLM 5.3、GPT 5.6-Sol、Claude Opus 5。更新日志页只有 DeepSeek 自家一列 |
| 不是9月下旬发布的 GPT-6-Sol、Opus-5.5 | L1 | ✅ | 本号 0924 期（`../../../2609/0924/sources/fact-check.md` 9、14 行） | 0924 期已核：Opus 5.5 发布（“比 Opus 5 降 20%”）、GPT-6-Sol/Luna 发布（“比 GPT-5.6 促销价便宜 50%”）。Ultramarine 页对比用的就是“GPT-6 Sol (max)”“Claude Opus 5.5 (max)”（`d-ultramarine-blog.txt` 787 行），说明两者已公开可测 |
| 11项里 Opus-5 仍有6项最高 | L1 | ✅ | 同官方表格图 | Claude 逐行读图：粗体最高——Opus 5：HLE 56.3、Terminal-Bench 4.0 51.8、ProgramBench 37.0、NL2Repo 75.3、Chartography 84.0、BabyVision 94.1，共 6 行；1017：DeepSWE 77.0、Automation-Bench 60.5、ALE 35.4、Apex Bench 72.1，共 4 行；GPT 5.6-Sol：ZeroBench 53.0，1 行。与 A 组读数一致 |
| 表为官方公布 | L1 | ✅ | 新闻页官方表格图与脚注 | 表出自 DeepSeek 新闻页；脚注“我们在对 Terminal-Bench 4.0 进行的测试中使用 Claude Code 框架，DeepSWE 使用 Mini-SWE-V2 框架”。第一轮反向核验 NEEDS_VERIFICATION 10：“均为自测”不能证明竞品各列都由 DeepSeek 重测，改为“官方公布”。AA、OpenRouter 页未见标明 1017 版本的独立成绩（A6） |
| 已删：9月列19项、这次列11项 | L1 | — | 更新日志两段（`../images/01`、`04`） | 新闻页对比表另有竞品列，“只比自家”说法不成立后删去 |

## 主帖二：NSA 曲线种子的身世

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 北京时间17日凌晨，Anthropic 称 Claude 不到半小时找出 P-256 等5条 NIST 椭圆曲线种子的来历 | L1 | ✅ | https://www.anthropic.com/research/ecdsa-curve-origins（`b-anthropic.txt` 47 行；`../images/13`） | datePublished 2026-10-16T17:02Z = 北京 10-17 01:02。“In under half an hour, Claude discovered the origins of the NIST P-192, P-224, P-256, P-384 and P-521 seeds.”模型名原文只写“an internal research model”；员工提问时“without pointing at the clue” |
| 1997–1998年用时间戳初始化的随机数生成器的输出 | L1 | ✅ | 同上；Valsorda https://words.filippo.io/seeds-solved/（`b-solved.txt` 9、21 行） | Anthropic 写“the C library’s lrand48 random-number generator, seeded with a Unix timestamp”；Valsorda 写“the output of drand48 seeded with the time, which was indeed in 1997–98”。函数名两处不同，正文不写函数名。B 组在本机重跑原文代码，五条素数曲线种子逐字复现（`b-repro-seeds.out.txt`），P-256 = 1997-08-16 01:52:34 第 1842 个候选 |
| 这些种子由美国国家安全局提供，当年没附说明 | L1 | ✅ | Anthropic 原文 31 行 | “they were provided by the NSA with no explanation or justification” |
| Anthropic 写明这不算攻破或新弱点 | L1 | ✅ | Anthropic 原文 53 行（`../images/14`） | “This is not a cryptographic break, new attack, or weakness in the ECDSA curves.” |
| 悬赏发起人、密码学家 Valsorda 称，即便放宽到任选当年时间戳和前16个素数阶候选，余地也仅约30比特，不够埋“只有设计者能用”的后门 | L2 | ✅ | Valsorda 宣布领奖页（`b-solved.txt` 61 行；`../images/22`） | “Even if the NSA had been free to pick any timestamp in 1997–98 and any of the first 16 prime-order candidates (which generously accounts for the unexplained rejections), they would have had barely 30 bits of freedom. Not enough to intentionally target a hypothetical small class of weak curves, not enough to embed any kind of NOBUS backdoor.”“只有设计者能用”对应原文 NOBUS（nobody but us）；“当年”指 1997–98 年。第一轮反向核验 BLOCKER 3 后补回假设与后门类型。悬赏由 Anthropic 团队（10 人署名）于 2026-10-15 01:07:51 UTC 申领（“claimed the full bounty”），奖金捐给 ISRG、三倍后 36,864 美元（11 行），Anthropic 原文写“will be donated”，到账未验证；悬赏出资人 Amir Omidi 也在 Anthropic 署名名单中 |
| Valsorda 称可用种子约是公布数的两倍，跳过原因找不到合适解释 | L2 | ✅ | `b-solved.txt` 27 行（`../images/21`） | “This leaves us with roughly twice as many valid seeds as were published. We explored a number of explanations for this last bit, and found no good fit.”Anthropic 原文对跳过候选写的是“the number of skipped candidates is small”，并举 P-256 第 876 个候选（95 行；`../images/15`）。两种说法都记录 |
| 四条曲线的基点也仍未解释 | L1 | ✅ | Anthropic 原文 91 行 | “The base points of P-224, P-256, P-384 and P-521 don’t fit this recipe; we weren’t able to explain how they were selected.” |
| 已删：B-163 种子未还原 | L1 | — | Anthropic 原文“B-163 … remains unexplained” | Valsorda 页 P.S. 写 Anthropic 团队也弄清了 B 曲线种子，两处说法有出入，正文删去 |

## 主帖三：AI 公司少招新人？

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Anthropic 发布内部用工研究：技术岗新招聘中初级占比从27%降到9% | L1 | ✅ | https://www.anthropic.com/research/employment-at-anthropic（`c-anth-report.txt` 59 行；`../images/25`） | datePublished 2026-10-16T16:26Z = 北京 10-17 00:26。“The proportion of junior technical hires fell even faster, from 27% to 9%.”图 1 Hires 线起点约 2025-05（读图），故正文不写起始时间；初级员工占比 21%→14% 为“since early 2025” |
| 技术招聘绝对数仍在增长 | L1 | ✅ | 同上 63 行（`../images/26`） | “As the company expands, technical hiring has continued growing in absolute terms” |
| 降的是招聘占比，不能推出裁员（省流） | L1 | ✅ | 同上 | 原文只报占比、技术招聘绝对数仍增长，未报裁员；本句只说明指标不能推出裁员，不断言没有裁员（第一轮 SHOULD_FIX 4 后改写） |
| IT 之家标题写“AI 工时已是人类6倍” | L4 | ⚠️ | https://www.ithome.com/1/014/195.htm（`c-news-ithome.txt` 1 行） | 标题“Anthropic 报告：AI 工时已是人类 6 倍，初级程序员岗位骤减，人均雇员创收是 Meta / 谷歌同期的 2 至 4 倍”；凤凰网转载同题（`c-news-ifeng.txt`）。已核的是标题文字本身；6 倍的含义另据研究原文核验（下一行）。原文为“by August the agents worked 6 times as many hours as their human handlers” |
| 这是8月数据，只算员工发起的交互式会话，分子是 agent 运行时长（不含超30分钟的等待），分母按人头乘每周40小时算 | L1 | ✅ | 图 2 注释（`c-anth-report.txt` 77 行；`../images/27`） | “We limit to employee-directed interactive sessions and pool subagents. Working time concept is agent runtime, including inference and active tool use, but excluding any wait longer than 30 minutes. Human work hours are calculated as headcount and assumed 40 hours per week.” |
| 已删：这类会话约80%时间在等人回复 | L1 | — | 同上 113 行（`../images/31`） | “In interactive, multi-turn sessions… Claude sessions amount to around 80% idly waiting for human input.”80% 按会话总历时计，6 倍按运行时长（已排除超过30分钟的等待）计，口径不同，并置会误导（第一轮 BLOCKER 2），删去 |
| 已删：效率对比是192名用 agent 的工程师对74名没用的，原文承认用不用不是随机的 | L1 | — | 同上 85、87 行（`../images/29`） | “Note however that tool adoption is not random”；“We follow 192 engineers over time as they adopt coding agents, compared to 74 engineers who do not adopt agents during the period studied.”篇幅原因删去 |

## 省流卡

| 卡片文字 | 对应正文 |
|---|---|
| DeepSeek 搜索接口按次收费，Claude Code 等客户端经 DeepSeek API 的服务端搜索11月起也收费 | 省流第 1 句 |
| Anthropic 称 Claude 不到半小时找出 NIST 曲线种子的来历 | 省流第 2 句前半；原文“In under half an hour”（视觉复核后删去卡上的“不是破解”，并补“不到”） |
| Anthropic 技术岗新招聘中初级占比从27%降到9% | 省流第 3 句前半（视觉复核后删去卡上的判断尾句，正文省流保留） |

## 批注截图译注

| 图 | 标题 | 原句（高亮） | 译注 | 级 | 结果 | 来源 |
|---|---|---|---|---|---|---|
| 41 | DeepSeek 搜索 API 指南 · 计费 | “请注意，当前用户在 DeepSeek Harness/Claude Code/Codex 中使用 DeepSeek API 时……起按次收取调用费用。”（rects 整句三行） | 在 DeepSeek Harness、Claude Code、Codex 中使用 DeepSeek API 时经服务端工具调用的 Search/Fetch，目前不计费，北京时间11月1日00:00起按次收费 | L1 | ✅ | `a-search-zh.txt` 305 行；截图存档 2026-10-17（`capture-log-a.md`） |
| 42 | DeepSeek 接入文档 · Claude Code 的 Web Search | “如果模型判断您的问题需要通过搜索功能才能满足，模型会调用 Web Search 工具，并通过 DeepSeek 提供的 API 进行搜索。” | 在 Claude Code 里，模型判断问题需要搜索时会调用 Web Search 工具，经 DeepSeek 的 API 搜索 | L1 | ✅ | `a-cc-zh.txt` 153 行（“并通过 DeepSeek 提供的 API 进行搜索”） |
| 43 | DeepSeek 新闻页 · 官方对比表 | 表头“GPT 5.6-Sol”“Claude Opus 5”（rects） | 对比表里的 GPT-5.6-Sol 与 Claude Opus-5 两列 | L1 | ✅ | 官方表格图；“美国模型”“粗体最高”见主帖一各行 |
| 44 | Anthropic 研究 · Claude 找到种子 | “In under half an hour, Claude discovered the origins of the NIST P-192, P-224, P-256, P-384 and P-521 seeds.” | 不到半小时，Claude 找出了 NIST P-192、P-224、P-256、P-384、P-521 五条曲线种子的来历 | L1 | ✅ | `b-anthropic.txt` 47 行 |
| 45 | Filippo Valsorda · 种子悬赏已被申领 | “Even if the NSA had been free to pick any timestamp in 1997–98 and any of the first 16 prime-order candidates (which generously accounts for the unexplained rejections), they would have had barely 30 bits of freedom.”（rects 四行）；“not enough to embed any kind of NOBUS backdoor.” | 即便当年可任选1997–98年的时间戳和前16个素数阶候选（已宽松计入没解释的跳过），也只有约30比特的选择余地；不够埋入任何“只有设计者能用”（NOBUS）的后门 | L2 | ✅ | `b-solved.txt` 11（“claimed the full bounty”）、61 行 |
| 46 | Filippo Valsorda · 没对上的跳过 | “This leaves us with roughly twice as many valid seeds as were published.”；“We explored a number of explanations for this last bit, and found no good fit.” | 这样算下来，可用种子约是公布数的两倍；这一点我们试了多种解释，没找到合适的 | L2 | ✅ | `b-solved.txt` 27 行 |
| 47 | Anthropic 用工研究 · 图 1 | “The proportion of junior technical hires fell even faster, from 27% to 9%.” | 技术岗新招聘中初级的占比降得更快，从27%降到9% | L1 | ✅ | `c-anth-report.txt` 59 行 |
| 48 | Anthropic 用工研究 · 图 2 注释 | “We limit to employee-directed interactive sessions and pool subagents.”；“Working time concept is agent runtime, including inference and active tool use, but excluding any wait longer than 30 minutes.”；“Human work hours are calculated as headcount and assumed 40 hours per week.” | 只统计员工发起的交互式会话，子代理合并计入；工时指 agent 运行时长，含推理与主动调用工具，不含超过30分钟的等待；人的工时按人头计算，并假设每周40小时 | L1 | ✅ | `c-anth-report.txt` 77 行 |

截图存档日期均为 2026-10-17（北京时间 10-17 晚至 10-18 凌晨，见各组 `capture-log-*.md`）。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 据 The Verge 报道，实测本地模型做同一份简报两次耗电10%和60% | L4 | ⚠️ | https://www.theverge.com/tech/1012748/nvidia-rtx-spark-windows-arm-laptops-asus-hp-lenovo-impressions（`d-rtx-verge.txt` 399–401 行） | 北京 10-16 23:00；记者 Sean Hollister 用 Asus ProArt P16（RTX Spark，99.9Wh 电池）跑 Hermes + Qwen3.8-27B：“It took 10 minutes, and 10 percent of my entire 99.9 watt-hour battery”；云端 Claude（Opus 5.5）用时一半、结果更好；重装后“took even longer and used 60 percent of my battery”。单人实测；速览标签“RTX Spark”指测试机型 |
| 论文称 Claude 设计量子电路7/8项估算比特与 Toffoli 门低于基线 | L1 | ✅ | arXiv 2610.22367（`e-e1-arxiv-abs.txt` 35 行）；https://www.anthropic.com/research/designing-efficient-quantum-circuits（`e-e1-quantum.txt`） | “In seven of the eight tasks, the discovered circuits significantly reduce both Toffoli and qubit counts relative to the state of the art.”速览标签“量子”。“基线”：摘要写 relative to the state of the art，但 Hermitian DQI 解码器一行为作者自建的朴素实现（“Naïve reversible implementation; no prior reversible compilation exists in the literature.”，`e-e1-arxiv-pdf.txt` 1046–1060 行），故写“论文称……低于基线”（第二轮 BLOCKER 1）。“估算”指论文的资源数均为未来容错机器上的估算，非实机测量（视觉复核 SHOULD_FIX 后补入）。“此前最佳”= relative to the state of the art；第八项点加法为比特多 12%、门数少 17 倍；均为未来容错机器资源估算 |
| Mercor 按29个基准估算，美国约90%工时花在几乎没被覆盖的任务上 | L1 | ✅ | https://www.mercor.com/research/labor-index/benchmark-map/（`c-mercor-map.txt` 82、124 行；`../images/33`） | “Our findings suggest that 90% of American working hours are spent on tasks with little or no coverage from existing benchmarks.”；“we start with a list of 29 benchmarks”；限制节称结果 provisional、名单不全会低估覆盖；29 个中 4 个是 Mercor 自家 APEX 基准 |
| AI 限时1小时写星际机器人，榜首模型对三款顶级人写机器人平均胜7% | L3 | ✅ | https://starskirmish.com/bench/v0-2/（`e-e4-starskirmish.txt` 14、43 行） | “Each LLM gets one hour to write a Protoss bot in C++”；“Against Stardust, PurpleWave and BananaBrain, the best model, GPT-6 Astra, wins only 7% of its games.”速览标签“StarSkirmish”（v0.2）；分数为每个模型 5 次生成的机器人的平均胜率（26 行“Scores are average win rates (0–100) of each model's 5 runs”）；对局在三张双人地图上，页面另称真实人写机器人比赛条件更难 |
| Ultramarine-1-Flash 自称开放模型，权重仍标“即将发布” | L1 | ✅ | https://www.paperinstruments.com/ultramarine（`d-ultramarine.txt` 14 行） | 标题“An open model”；“Model weights coming soon”；技术报告页 10-14 |

## 反向核验

### 第一轮（codex gpt-6-astra medium，WSL，`barking review --reverse`）

3 BLOCKER、6 SHOULD_FIX、3 NEEDS_VERIFICATION，Claude 逐条核实后全部采纳或处理：

1. BLOCKER 省流与图 41 删掉了“使用 DeepSeek API 时”，把收费扩大到客户端搜索整体 → 补回。
2. BLOCKER 6 倍（运行时长、排除超过30分钟等待）与 80%（会话总历时）口径不同却并置 → 删去 80% 句。
3. BLOCKER “不够埋后门”丢了 Valsorda 的假设与 NOBUS 限定 → 补回“即便放宽到任选当年时间戳和前16个候选”“只有设计者能用”。
4. SHOULD_FIX “不是裁员”超出证据 → 改为“降的是招聘占比，不能推出裁员”。
5. SHOULD_FIX 量子速览缺指标与基线 → 改为“比特数、Toffoli 门数都比此前最佳少”。
6. SHOULD_FIX StarSkirmish“最强的”无范围 → 改为“榜首模型……平均胜7%”，清单补 5 次生成平均口径。
7. SHOULD_FIX 图 45 标题“已被领走” → 改为“已被申领”。
8. SHOULD_FIX 批注与清单失同步 → 重写译注表，图 42 译注收窄到所引句。
9. SHOULD_FIX L4 标 ✅ → 改 ⚠️。
10. NEEDS_VERIFICATION “成绩均为自测” → 改为“表为官方公布”。
11. NEEDS_VERIFICATION 配图说明与 offsite 未就绪 → `images/README.md` 已写；`offsite.tsv` 发布后由 `barking offsite` 生成。
12. NEEDS_VERIFICATION 正文与卡片出现 NSA/美国国家安全局 → 平台风控提示，交 Develata 决定。

篇幅：补回限定后超 1000 字，另删三、吠点②（192 对 74 非随机）。

### 第二轮（同上，附第一轮处理清单）

1 BLOCKER、4 SHOULD_FIX、1 NICE_TO_HAVE、1 NEEDS_VERIFICATION：

1. BLOCKER 量子速览“比此前最佳少”：8 项中 Hermitian DQI 解码器一项的基线是作者自建朴素实现 → 改为“论文称……低于基线”。
2. SHOULD_FIX 正文“前16个候选”缺“素数阶”（P-256 是原始第 1842 个候选，不在前 16 个原始候选里）→ 补“素数阶”。
3. SHOULD_FIX `images/README.md` 图 41、42、45 的可选图注仍是第一轮前的宽泛写法 → 改为与译注一致。
4. SHOULD_FIX 正文 6 倍口径缺“不含超30分钟的等待”与“8月” → 补入。
5. NICE_TO_HAVE `evidence-a.md` A5 把 DeepSeek 官方 HF 模型卡标 L3 → 本清单按 L1（官方仓库材料），取证记录不改，以本清单为准。
6. NEEDS_VERIFICATION NSA/美国国家安全局风控 → 同第一轮第 12 条，交 Develata。

## 视觉复核（codex gpt-6-astra medium，`barking review --visual`，20 张图）

1 BLOCKER、8 SHOULD_FIX，处理如下：

1. BLOCKER 省流卡第二、三条带判断尾句（“不是破解”“不能推出裁员”），违反“图上不放吠点” → 卡上删去，正文省流保留。
2. SHOULD_FIX 省流卡“半小时”漏“不到” → 正文省流与卡片同改“不到半小时”。
3. SHOULD_FIX 图 42 译注超出高亮 → quote 延伸到“并通过 DeepSeek 提供的 API 进行搜索。”
4. SHOULD_FIX 图 43 译注加了高亮外的内容 → 译注只留两列模型名。
5. SHOULD_FIX 图 45 高亮缺前面的假设 → 改用 rects 高亮整句 Even if 条件（OCR 对“1997–98”置信度 36）。
6. SHOULD_FIX 图 48 缺分子定义 → 加第 2 条译注（Working time … 30 minutes）。
7. SHOULD_FIX 速览第 5 条缺“估算” → 补入（后经第二轮再改为“低于基线”）。
8. SHOULD_FIX 速览第 2 条含 NSA → 交 Develata（同上）。
9. SHOULD_FIX 图 43 表格脚注、图 48 图表在手机上字小 → 关键条件已写进中文译注（图 48 第 2、3 条），不另裁图。

### 第三轮短审（codex gpt-6-astra medium，视觉 + 文字，限前两轮改动）

PASS，无 BLOCKER：省流卡、速览第 5 条、图 42、43、45、48 与正文补改均已落实，未见新错位或口径混用。NSA 题材的风控取舍仍待 Develata 决定。

风控决定：Develata 终审决定保留标题、封面与速览中的“NSA”（2026-10-17）。
