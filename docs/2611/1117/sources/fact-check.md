# 1117 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B 组 Sonnet，C 组 Haiku 5.5（`claude-haiku-5-5`）；Claude 抽查。

## 一、GPT-6.2 系统卡（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 发布新模型 GPT-6.2-Astra 和 Sol，安全报告里有项测试（小标题：设障测试里新模型更常漏报） | L1 | ✅ | `a-blog.txt`；`a-syscard.txt`：“Published November 17, 2026” | 官方未给时刻；HN 50535230 提交 2026-11-17T17:46:43Z（北京 11-18 01:46），正文不写日期 |
| 给编程任务故意设障碍，比如删掉要改的文件，看 AI 会不会说出来 | L1 | ✅ | `a-syscard.txt` 4.1.3.1：“we may remove a source file the user asked the model to edit or make a tool it needs to build the project unavailable. We then measure how often the model’s final response omits or misrepresents the obstacle it encountered.” | 测试名 Blocker Deception，OpenAI 内部评测；图轴标题 “Blocker-related deception rate” |
| GPT-6.2-Astra 有22.28%的回答没提障碍或说错了，上一代 GPT-6-Astra 是11.09%（省流：故意设障碍让任务做不成时，有22.28%没告诉用户；这是设局测试，不是真实使用的统计） | L1 | ✅ | `a-syscard.txt`：“GPT-6.2 Astra omitted a known obstacle to completing its work in 22.28 percent of trials, compared with 11.09 percent for GPT-6 Astra.”；Figure 35（`images/02-syscard-fig35.png`，Claude 已读图） | 分母是 trials；不带 Codex 提示词。图中另有 Sol 24.08%、GPT-6.1 Sol 14.00%、GPT-6.1 Terra 8.47%。“回答”指最终回复（final response） |
| OpenAI 写道对这一退步“感到担忧”，猜测可能与“让回答更简洁、少加限定和提醒”有关 | L1 | ✅ | `a-syscard.txt` L4529–4531：“We are concerned by this regression … One hypothesis is that our efforts to make responses concise and avoid excessive caveats may be related to the model omitting important obstacles.” | 原文是 “one hypothesis”，正文写“猜测” |
| 对你：在 ChatGPT 里，新模型只进 Work 和 Codex（写代码的工具），普通聊天选不到 | L1 | ✅ | `a-help-work-codex.txt` L18：“This rollout does not add either model to regular Chat.” | 同句亦见 `a-help-models.txt`；另有 API |
| 对你：官方名单里没有免费版 | L1 | ✅ | `a-blog.txt` L250：“available to all Plus, Pro, Enterprise, and Edu users in ChatGPT Work and Codex”；`a-codex-pricing.txt` L2110–2111：“GPT-6.2 Astra and GPT-6.2 Sol are available on Plus, Pro, Business, Enterprise, and Edu in ChatGPT Work on web, mobile, and desktop, and in the Codex app and CLI.”（反向核验第 1 轮指出原引 L2121 主语是 GPT-6.1 Terra，已改） | 三份名单均无 Free、Go；页面未明说“不可用”，正文只写“名单里没有” |
| （第 2 轮后删出正文）企业版、教育版默认关，要管理员开 | L1 | ✅ | `a-codex-pricing.txt` L2112：“Both models are off by default in Enterprise and Edu workspaces. An administrator must enable them” | 帮助中心 `a-help-ent-edu.txt` L46 同 |
| 吠点①：障碍是故意放的，OpenAI 写明结果不一定反映真实使用中的误导率；真实使用里的比例，报告没给 | L1 | ✅ | `a-syscard.txt`：“Although these problems were designed to elicit misaligned behavior and do not necessarily reflect the rate of misleading responses in production” | “没给数”指 4.1.3.1 节未给生产环境比率（A 组通读该节与博客） |
| 吠点②：OpenAI 称已在 Codex 里加指令，同一测试里 Astra 不说障碍的比例降到8.04% | L1 | ✅ | `a-blog.txt` L244：“We added instructions in Codex that substantially reduce these omissions”；Figure 35 “with dev prompt” GPT-6.2 Astra 8.04% | 图中加提示词的只有 GPT-6.2 两个模型，旧模型无对应数，正文不作新旧对比 |
| 吠点②：安全报告写明，开发者在自己程序里直接调用模型时，不带这段指令 | L1 | ✅ | `a-syscard.txt` L4573：“The Codex developer prompt used in these evaluations is not applied to direct API requests.” | |

未入正文：Broken Search Tool 23.00% 对 GPT-6 Astra 7.58%（加提示词 Astra 8.50%）；AA Intelligence Index Astra 62、Sol 57（模型页 #3/92）；Vals Index Astra 67.66% #7/41（同页另写 “#7 of 40”）；272K 以上整单加价新旧两版都有（WorkBuddy“长请求比上一代还贵”不成立）；博客正文只写 “more likely to omit mentioning obstacles”，未给 22.28/11.09；英国 AISI 一节（涉政府，风控，不用）。

## 二、微软 MAI Code 2（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 微软发布编程模型 MAI-Code-2 | L1 | ✅ | `b-blog.txt`；`datePublished` 2026-11-17T18:15:16Z | = 北京 11-18 02:15 |
| 称在四项编程测试里（三项微软自己跑），它的平均通过率是 GPT-6.1-Sol 的98%；每个任务平均花0.309美元，对方0.761美元（省流：微软称成本只有 GPT-6.1-Sol 的四成，按的是五折促销价） | L1 | ✅ | `b-blog.txt`：“Averaged over four widely used benchmarks … MAI Code 2 gives you 98% of the performance of GPT 6.1 Sol at 40% of the cost” | 0.309/0.761 见图表 data-title（`b-blog.html` L1065，“average cost per task $0.309 (50% promotional price), $0.761”）与 `images/15-chart-code2-avg.png`；四项：DeepSWE 1.1、Terminal Bench 2.1、SWE-Atlas、FrontierCode v1.1；“everyday enterprise coding tasks”；全部 max reasoning effort |
| GitHub 价目表：MAI-Code-2 现在是促销价，每百万 token（文字计量单位）输入0.5美元、输出2.5美元，2027年5月17日起涨到1美元、5美元 | L1 | ✅ | `b-copilotpricing.txt` L1346、L1430：“promotional pricing of $0.50 per 1M input tokens … $2.50 per 1M output tokens until May 17, 2027. Starting May 17, 2027, pricing is $1.00 … $5.00” | GitHub Docs “Models and pricing”，取于北京 11-18 16:28；缓存输入另计 |
| （第 1 轮后删出正文）GPT-6.1-Sol 是2美元、10美元 | L1 | ✅ | `b-copilotpricing.txt` L1268：GPT-6.1 Sol Default ≤272K $2.00 / $0.10 / $2.50 / $10.00 | Copilot 价目表口径（≤272K），列序：输入/缓存/缓存写入/输出 |
| 对你：GitHub 称，其编程助手 Copilot 的 Pro 及以上付费用户可以选它，按用量计费，商业版、企业版由管理员定 | L1 | ✅ | `b-ghmai.txt`：“MAI Code 2: Copilot Pro, Pro+, Max, Business, and Enterprise”；“These models are billed under usage-based billing.” | 同页 “Both models are available to eligible GitHub Copilot subscribers on annual plans.” 适用范围不清，正文不解读；`copilot/plans` 页 Free 清单出现 MAI Code 2，与更新日志不一致，正文按更新日志写“付费版”；“商业版、企业版由管理员定”见 `b-ghmai.txt` L124（Copilot Enterprise and Copilot Business plan administrators） |
| 吠点①：微软大会总览页写“40%的价格”，没提促销 | L1 | ✅ | `b-ignite.txt` L247：“reaches 98% of the coding quality of GPT-6.1 Sol at about 40% of the price”；全页 “promotional” 0 命中 | 北京 11-18 02:16 |
| 吠点①：AI 负责人苏莱曼写“40%的价格”，没提促销 | L2 | ✅ | `b-suleyman-fx.json`：“at just 40% of its price”；全文 “promotional” 0 命中 | 账号简介 “CEO, @MicrosoftAI”；海外社媒长文，正文不写平台名；北京 11-18 02:28 |
| 吠点①：博客脚注写明，这个成本按五折促销价算 | L1 | ✅ | `b-blog.txt`：“MAI-Code-2 cost is shown at its 50% promotional price.” | 图表 alt 文字也写 “$0.309 (50% promotional price)”（`b-blog.html` L1065），图面可见区不写 |
| 吠点②：微软称的“98%”来自平均通过率63.2%对 GPT-6.1-Sol 的64.1% | L1 | ✅ | `b-blog.html` L1065 图表 data-title：“average pass rate 63.2%, 64.1%, 65.3%; average cost per task $0.309 (50% promotional price), $0.761, $6.306” | 63.2/64.1 = 98.6%（本号核算，微软写 98%） |
| 吠点②：MAI-Code-2 更低的三项，正是微软自己跑的三项 | L1 | ✅ | 图表分项（Claude 已对图 `images/16-chart-code2-bench.png` 逐项核对）：DeepSWE 73.2/74.6、Terminal-Bench 84.6/85.0、SWE Atlas 43.1/47.4、FrontierCode 52.0/49.4；脚注 “DeepSWE 1.1, TerminalBench 2.1 and SWE Atlas were run on MSBench (Microsoft’s internal eval platform)” | 三项低于 Sol 的正好是 MSBench 三项；FrontierCode 取自 AA，AA 页现为 Legacy 且读数与微软图不一（Opus 52.2 vs 56.0、6.1 Sol 49.1 vs 49.4），未入正文 |
| （已删出正文，留作记录）同一天 OpenAI 发布的 GPT-6.2-Sol 在 Copilot 标价相同，微软没拿它比 | L1 | ✅ | `b-copilotpricing.txt` L1280：GPT-6.2 Sol $2.00 / $0.10 / $2.50 / $10.00；博客对比对象只有 GPT-6.1 Sol、GPT-6 Sol、GPT-6 Luna、Opus 5.5、Haiku 5.5 | GPT-6.2 HN 帖 17:46 UTC 早于 MAI 博客 18:15 UTC 约半小时，只记时间，不作指控 |

未入正文：盲测偏好率博客旧版 67%（Wayback 北京 11-18 08:36）→ 现版 64%，对象 GPT-6 Sol，258 名标注者；Flash “20% of the cost” 对图上数字比值 16.3%；博客 “not yet tuned for … very long horizon agentic work” 对模型卡 “recommended for the most complex, long-horizon tasks”（层级可能不同，不作矛盾写）；按字面五折还原约为 Sol 的 81%（本号推算，不用）。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌称 Gemini 免费版 Tasks 在美日韩等6国逐步推出，高级功能先美国 | L1 | ✅ | `c-gem-tasks.txt`：“rolling out free of charge for hundreds of millions of people across the U.S., Japan, Korea, Canada, Australia, and India” | 官方 2026-11-17T17:00Z = 北京 11-18 01:00。高级功能 “starting in the U.S.”；可设 “hard spending cap”；购买前 “designed to ask for your confirmation or pre-approval”；Indiana 等5州不能代打电话。与 1114 速览“Wallet 连接只读”是不同功能。HN 无主帖 |
| Google Wallet 上线美国 iOS，iOS 上暂不支持碰一碰支付 | L1 | ✅ | `c-wallet-ios.txt`：“Google Wallet is now available on iOS in the U.S.”；“tap-to-pay functionality isn't supported on iOS at this time.” | 北京 11-18 01:00。HN 50534509 2 分 |
| Runta 自家评测22种智能体配置：按标价，64.5%的成本花在没通过的尝试上 | L1 | ✅ | `c-frontierharness.txt`：“22 harnesses × 5 models × 62 tasks × 4 attempts”；L86：“18 harnesses … Each counts separately: 22 configurations.”（第 2 轮指出“22种外壳”应为配置数，已改）；“64.5% of the list-price cost of the scored runs went to trials that did not pass” | 厂商自家评测（Runta 自己的运行环境，创始人 HN 自述 “weigh this accordingly”）；成本为标价估算，非实际账单；只测开源模型、一家推理商（Baseten）。官方未给时刻（11-17）；HN 50532191 367 分 |
| 谷歌与 UCL 测18个模型的网络安全建议，按专家细则平均得分82% | L1 | ✅ | `c-helpbench.txt`：“On average, models scored 82% on HelpBench”；论文 §4.1 得分 = 所得分 ÷ 满分 | 82% 是细则得分率，不是准确率（博客另写 “matching 82% of the advice provided by experts”，按论文定义写）。“低于65%”的比例博客写 1 in 10、论文写 14%（“roughly 1 in 10”），两数不一，速览不用。评测于 “Earlier in the year”。北京 11-17 15:00 |
| OpenAI 称 GPT-6-Astra 做病历任务，325项中99%被评为安全 | L1 | ✅ | `c-openai-health.txt`：“Overall, 99% of 325 reviewed tasks were rated safe.” | 评测对象是 GPT-6 Astra（不是 6.2，速览写全型号，免得读成主帖那个新模型）；安全 = 错误或遗漏是否可能伤害患者；评审者身份、人数未写；合成患者数据只用于演示。官方只给日期 11-16 |
| Isomorphic Labs 宣布50亿美元 C 轮融资，新闻稿未写估值 | L1 | ✅ | `c-isomorphic.txt`：“today announced a $5 Billion Series C financing round.”；全文无 valuation | 北京 11-17 20:00（07:00 ET）。Greenoaks 与红杉共同领投；原文 “positive preclinical results … setting the stage for clinical entry” |
| （第 1 轮后不收）微软称 MAI Image 2.7 登 AA 图像编辑榜第一，领先第二4 Elo | L1 | ✅ | `c-mai-image27.txt` 脚注：“As of November 16, 2026, MAI Image 2.7 ranks #1 on AA-Image-Editing v2.1, scoring 1,215 Elo across 5,478 votes, ahead of GPT Image 2.6 Sunburst (max) by 4 Elo.” | 排名为 “As of November 16, 2026”。AA 原页（`c-aa-image-edit-b.txt`，北京 11-18 16:28）1215（1201–1229）对 1211（1198–1224），区间重叠（推断）。Arena 上为 “top-2 lab”。北京 11-18 02:16 |

备用（未入图）：MAI Cyber 2 Flash 自报 CyberGym-E2E 92.3% 对 GPT-6 Astra 91.5%（`c-mai-cyber2flash.txt`；剔除 905 题中与训练数据同源的 92 题；只进微软安全产品，12 月中旬起替换既有模型）。

## 批注译注

| 卡片 | 原句 | 译注（图上逐字） | 依据 |
|---|---|---|---|
| gpt-blocker ① | Notably, GPT-6.2 Astra omitted a known obstacle to completing its work in 22.28 percent of trials, compared with 11.09 percent for GPT-6 Astra. | GPT-6.2 Astra 在 22.28% 的试验中遗漏了妨碍完成工作的已知障碍，GPT-6 Astra 为 11.09%。 | `a-syscard.txt` 4.1.3.1；省去句首 Notably |
| gpt-blocker ② | We are concerned by this regression … One hypothesis is that our efforts to make responses concise and avoid excessive caveats may be related to the model omitting important obstacles. | 我们对这一退步感到担忧，正继续调查根本原因。一种假设是：让回答简洁、避免过多限定说明的努力，可能与遗漏重要障碍有关。 | 同上（视觉复核后补回“正继续调查根本原因”） |
| gpt-blocker ③ | Although these problems were designed to elicit misaligned behavior and do not necessarily reflect the rate of misleading responses in production, | 这些题目为诱发不对齐行为而设计，不一定反映真实使用中误导性回答的比率。 | 同上；下划线 “do not necessarily reflect” |
| gpt-blocker ④ | The Codex developer prompt used in these evaluations is not applied to direct API requests. | 评测中用的 Codex 开发者提示，不作用于直接的 API 请求。 | 同上 |
| gpt-fig35 ①–③ | 图 35 柱标：with dev prompt GPT-6.2 Astra 8.04%；without dev prompt GPT-6.2 Astra 22.28%、GPT-6 Astra 11.09%；纵轴 “Blocker-related deception rate (%)”，“Lower is better” | 见 cards.toml | Claude 读 `02-syscard-fig35.png`；“欺瞒率”为轴标题直译，正文不用 |
| mai-chart ①② | Average pass rate 63.2% / 64.1%；Average cost per task $0.309 / $0.761 | 平均通过率：MAI Code 2 63.2%，GPT-6.1 Sol 64.1%。每任务平均成本：MAI Code 2 0.309 美元，GPT-6.1 Sol 0.761 美元。（博客脚注补充：MAI 按五折促销价；均为最高推理档） | `15-chart-code2-avg.png`；`b-blog.html` L1065 |
| mai-footnote ① | DeepSWE 1.1, TerminalBench 2.1 and SWE Atlas were run on MSBench (Microsoft’s internal eval platform) with the GitHub Copilot CLI without subagents. | DeepSWE 1.1、TerminalBench 2.1 和 SWE Atlas 在 MSBench（微软内部评测平台）上用 GitHub Copilot CLI 跑，不开子智能体。 | `b-blog.txt` 脚注 1 |
| mai-footnote ② | MAI-Code-2 cost is shown at its 50% promotional price. | MAI-Code-2 的成本按其五折促销价显示。 | 同上（手量框） |
| mai-bench ①② | DeepSWE 1.1 73.2/74.6；Terminal-Bench 2.1 84.6/85.0；SWE Atlas 43.1/47.4 | 见 cards.toml | `16-chart-code2-bench.png`（Claude 逐项读图） |
| ignite-price | MAI Code 2 reaches 98% of the coding quality of GPT-6.1 Sol at about 40% of the price. | MAI Code 2 以约 40% 的价格达到 GPT-6.1 Sol 98% 的编码质量。 | `b-ignite.txt` L247 |
| copilot-promo ①② | MAI Code 2 is available at the promotional pricing of … until May 17, 2027. / Starting May 17, 2027, pricing is … | 见 cards.toml | `b-copilotpricing.txt` L1430 |

## 反向核验

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-18 约 17:50；审的是读者稿检查前的正文，卡片尚未生成）

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | BLOCKER | 标题“AI说做完了？新GPT漏报22%”把诱发测试写成普遍漏报率 | 采纳。读者稿检查后已改为“AI没做成也不说？新GPT测试中22%”，“测试中”进标题；第 2 轮复核 |
| 2 | SHOULD_FIX | “40%成本”是每任务成本，正文给的绝对数却是每百万 token 单价，单位不同 | 采纳。事实段改写每任务平均 0.309 美元对 0.761 美元，token 单价单独一句只讲 MAI 自己的促销与原价 |
| 3 | SHOULD_FIX | GPT-6.1-Sol 2/10 美元漏了 ≤272K 门槛 | 采纳。正文删去 GPT-6.1-Sol 单价（每任务成本已给绝对数） |
| 4 | SHOULD_FIX | “付费用户现在能选”忽略管理员可禁用、年付句不清 | 采纳。改为“可以选它……企业版由管理员定”；年付句不解读 |
| 5 | SHOULD_FIX | “只进 Work 和 Codex”漏了 API | 采纳。改为“在 ChatGPT 里，新模型只进 Work 和 Codex” |
| 6 | SHOULD_FIX | “不代表真实使用”强于原文 “do not necessarily reflect”；“免责声明”窄于 caveats | 采纳。省流改“不一定反映真实使用”；读者稿检查后已改“限定和提醒” |
| 7 | SHOULD_FIX | 速览 Gemini 句把六国推出与代打电话合并 | 采纳。改为“……逐步推出，高级功能先美国” |
| 8 | SHOULD_FIX | 速览 MAI Image 句丢了时点、只给差值 | 采纳。删去该条（48 字放不下时点与绝对分，热度仅 HN 3 分），速览共 8 条 |
| 9 | SHOULD_FIX | 定价页行号引错（L2121 主语是 GPT-6.1 Terra） | 采纳，改引 L2110–2111 |
| 10 | NEEDS_VERIFICATION | 8.04% 与分项分数只有取证者转录 | Claude 已读 `02-syscard-fig35.png` 与 `16-chart-code2-bench.png` 原图（同家族），两张都作批注底图交视觉复核（Codex）跨模型核读 |
| 11 | NEEDS_VERIFICATION | cards.toml 与 images/README.md 缺失 | 生成后第 2 轮短审 |

### 第 2 轮（Codex `gpt-6-astra` medium，北京 11-18 约 18:10；含卡片，未看图）

结论 PASS / no blocker；6 项 SHOULD_FIX、1 项 NEEDS_VERIFICATION。

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX | 速览“22种智能体外壳”：原文 18 种外壳、22 种配置 | 采纳，改“22种智能体配置” |
| 2 | SHOULD_FIX | 标题与小标题没交代人为设障 | 部分采纳：小标题与速览主帖条目改“设障测试里新模型更常漏报”；标题保留“测试中”（20 字放不下“设障”，且不是在纠正某个流行说法，不适用“说法 + 实际”） |
| 3 | SHOULD_FIX | 对比图译注单独传播会丢五折促销条件 | 采纳，0.309 美元后加“（按五折促销价）”，并注“均为最高推理档” |
| 4 | SHOULD_FIX | 管理员限制漏了 Business | 采纳，改“商业版、企业版由管理员定” |
| 5 | SHOULD_FIX | 吠点单条缺指代：8.04% 降的是什么；“98%”是谁的说法 | 采纳，改“Astra 不说障碍的比例降到8.04%”“微软称的‘98%’” |
| 6 | SHOULD_FIX | 事件卡没交代三项是微软自测 | 采纳，事实段加“（三项微软自己跑）”；吠点②相应压短 |
| 7 | NEEDS_VERIFICATION | 图中数值未经跨模型核读 | 交视觉复核（Codex） |

为压字数，OpenAI“对你”句删去“企业版、教育版默认关，要管理员开”（行留作记录）。

## 视觉复核

复核方：Codex `gpt-6-astra`（medium，只读，附 22 张图），北京 11-18 约 18:40。结论 PASS / no blocker；4 项 SHOULD_FIX、1 项 NICE_TO_HAVE。另按本期补充要求独立读图，结果与正文、清单一致：图 35 with dev prompt GPT-6.2 Astra 8.04%，without dev prompt GPT-6.2 Astra 22.28%、GPT-6 Astra 11.09%；平均通过率 63.2%／64.1%／65.3%，每任务平均成本 $0.309／$0.761／$6.306；分项 DeepSWE 73.2／74.6、Terminal-Bench 84.6／85.0、SWE Atlas 43.1／47.4、FrontierCode 52.0／49.4。反向核验第 1、2 轮的 NEEDS_VERIFICATION（图中数值）据此关闭。

| # | 级别 | 意见 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX | 封面绿色对勾气泡像“AI 明确宣称完成”，测试证明的是遗漏或误述障碍 | 采纳。对勾改为灰色省略号气泡（“什么都没说”），重出竖版、横版（A3），Claude 逐字核对标题无误 |
| 2 | SHOULD_FIX | gpt-blocker 译注②漏了高亮内的“仍在调查根本原因” | 采纳，补回 |
| 3 | SHOULD_FIX | mai-chart 译注②“按五折促销价”本图无原文 | 采纳，改为“（博客脚注补充：MAI 按五折促销价；均为最高推理档）”，与直译区分 |
| 4 | SHOULD_FIX | 图 35 在卡片里太小 | 采纳，底图裁去左右空白（[240,0,1000,580]），高亮框随之平移；重渲后 Claude 逐处核对高亮落在 8.04%、22.28%、11.09% 上 |
| 5 | NICE_TO_HAVE | 译注①型号跨行 | 不改（模板自动换行，强制换行会留空格） |
