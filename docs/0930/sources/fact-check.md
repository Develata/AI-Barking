# 0930 事实核验

正文：`../doc_0930_publish.txt`。取证：`evidence.md`（A–D，Codex gpt-6-astra，派工单 `.handoff/2026-09-30-0930-evidence.md`）。Claude 对下列条目逐字对照了存档原文或截图；AA 文章另由 Claude 用 curl 独立抓取核对（北京时间 9/30 23:4x）。按 EDITORIAL.md，L4 及以下只标 ⚠️。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间30日凌晨，OpenAI 发布 GPT-6.1-Sol | L1 | ✅ | OpenAI X 2104986129686741046（`../../0929/sources/e-x-openai-latest.json`）；developers changelog “Sep 29 … gpt-6.1-sol Released”（`a-changelog.md`） | X 帖 2026-09-29 17:26:16 UTC = 北京 9/30 01:26；发布页正文不显示自身日期；changelog 日期无时区。HN 帖 17:06:45 UTC。 |
| 升级7天前的 6-Sol | L1 + L3 | ✅ | 发布页 “an upgrade to GPT‑6 Sol”；GPT-6 Sol 发布 X 帖 2026-09-22T18:12:13Z（evidence A3）；AA 标题 “replaces GPT-6 Sol after just 7 days” | 9/22 18:12 UTC → 9/29 17:26 UTC 约 7 天。 |
| 主打“接近 Astra 的智能，五分之一的价格” | L1 | ✅ | 发布页标题 “Near-Astra intelligence for a fifth of the price”；OpenAI X 同句 | 意译。 |
| 标准价每百万 token 输入2美元、输出10美元，与 6-Sol 相同，仅缓存输入降价 | L1 | ✅ | developers.openai.com 价格页与模型页（`../images/08`、`../images/09`；`a-doc-pricing.md`、`a-sol-model.md`、`a-sol61-model.md`） | 6 Sol：$2 / $0.20 / $10；6.1 Sol：$2 / $0.10 / $10（输入 / 缓存输入 / 输出）。AA 原文：“Pricing matches GPT-6 Sol at $2/$10 … except that the cache read discount rises from 90% to 95%.” 发布页：“50% less than GPT‑6 Sol’s cached input pricing”。0924 存档的 6 Sol 价格与现价一致；中间是否调过价无完整日志。 |
| “五分之一”比的是 Astra 单价，6-Sol 时就如此 | L1 | ✅ | 价格页：gpt-6-astra $10 / $50 | 发布页原文：“at one-fifth of Astra’s standard input and output token prices”。$2/$10 ÷ $10/$50 = 1/5，6 Sol 同价故同比。 |
| 据第三方 AA 评测，max 档综合分只比 Astra 低1分 | L3 | ✅ | AA 文章；AA 模型页（`a-aa-sol-release.md`；`../images/22`、`25`） | Intelligence Index v4.3.2：GPT-6.1 Sol（max）52，GPT-6 Astra（max）53。 |
| 每题成本0.72对3.26美元 | L3 | ✅ | AA 文章 | “At max effort, GPT-6.1 Sol costs less than a quarter of GPT-6 Astra per Intelligence Index task ($0.72 vs $3.26).” 为 AA 指数的每道题平均成本，不是跑完整个指数的总价。 |
| 输出 token 比 6-Sol 多10%–30% | L3 | ✅ | AA 文章 | “uses ~10-30% more output tokens than GPT-6 Sol across effort levels”。 |
| AA 编码榜上，xhigh 档反比 max 档高3分 | L3 | ✅ | AA 文章；`../images/23` | “We observed the xhigh effort setting to outperform the max effort setting by 3 points.” Coding Agent Index v1.5：xhigh 63，max 60。只是这次观测，不是规律。 |
| Anthropic 称，开放权重的 GLM-5.3 防护太弱：模拟测试中，简单手法绕过率达64%–100% | L1（厂商研究） | ✅ | anthropic.com/research/glm-5-3-and-the-spread-of-advanced-cyber-capabilities（`b-anthropic.md`） | “We find that attackers can bypass GLM-5.3’s safeguards between 64% and 100% of the time with simple techniques in our simulated tests.” 核心论断另有 “GLM-5.3 will likely give malicious actors access to capabilities that will allow them to find and exploit cyber vulnerabilities without meaningful restrictions.” 开放权重见 Hugging Face zai-org/GLM-5.3（自定义许可证，`b-hf-license.md`）。页面日期 Sep 29, 2026，无时区，正文不写日期。 |
| 省流：Anthropic 点名智谱 GLM-5.3 防护太弱 | L1（厂商研究） | ✅ | 报告首段（`b-anthropic.md`） | “GLM-5.3 is unlike other frontier models in that it has been released without meaningful safeguards to limit misuse.” 报告另写 GLM-5.3 有内置安全措施、明显有害请求常会拒绝，故用“太弱”而非“没有”。小标题“100%不是攻击成功率”：100% 为模拟环境中尝试连接目标的比例，代码未执行（见下行）。 |
| 这是模型收到恶意指令后“尝试连接攻击目标”的比例，代码没执行；去掉拒答机制才到100% | L1 | ✅ | 报告 Figure 5（`../images/12`） | 图题：“Share of 50 episodes per cell in which the model tried to connect to a remote target after receiving a harmful cyber-attack order (simulated environment; nothing was executed)”；Abliterated 列注 “refusals removed”。脚注：“No model-generated code is ever executed in this simulation”。报告摘要称 “attackers can bypass GLM-5.3’s safeguards between 64% and 100% of the time … in our simulated tests”，标题“绕过”取自此。 |
| 直接提为0% | L1 | ✅ | Figure 5 | “Unmodified model, bare order”：GLM-5.3 0%。 |
| 端到端写攻击程序：GLM-5.3 成功50/410次，关掉防护的 Mythos-Preview 为56/410次 | L1 | ✅ | 报告正文与 Figure 2（`../images/11`） | “GLM-5.3 develops end-to-end exploits in 50 of 410 attempts. Claude Mythos Preview did so at a similar rate—in 56 of 410 attempts.” 图注：“two Claude models run with safeguards disabled (Opus 4.6, Mythos Preview)”。ExploitBench 为 V8 已知漏洞，隔离沙箱、无联网。 |
| 小号 Flash 版打已知漏洞：8小时、API 估价20.4美元 | L1 | ✅ | 报告正文（`../images/14`） | “GLM-5.3-Flash (a smaller, less capable version of GLM-5.3) to develop an exploit for a known vulnerability … CVE-2026-11645 … 20 minutes of human attention, plus eight hours of work … At Zhipu’s API prices, this effort would have cost $20.40.” |
| 挖出浏览器未知漏洞的是完整版 | L1 | ✅ | 同上 | “a researcher used GLM-5.3 on a sandboxed machine with a local Linux build … Over the course of a day (and with limited human attention), GLM-5.3 found several previously unknown vulnerabilities in the browser’s JavaScript engine”。已向维护方披露。 |
| 报告出自对手 | — | ✅（事实） | 报告署名 Anthropic；GLM-5.3 为智谱模型 | 报告未声明利益冲突（检索正文无相关段落）。 |
| 伪装请求下 Claude 均为0%，关防护的两款 Opus 为4%、10% | L1 | ✅ | Figure 5 | False cover story 列：Claude Opus 4.8 “0% / 4% with safeguards disabled”，Claude Opus 5 “0% / 10% with safeguards disabled”，Claude Mythos 5 0%（无关防护数值）。图注：“every tested Claude model stays at zero under API safeguards”。第一稿写“0%靠的是 API 防护，关掉后为4%–10%”，反向核验指出把两款模型泛化为 Claude 整体、且“靠的是”强于证据，已改。 |
| 预填、改权重对用户不可行 | L1 | ✅ | Figure 5 锁形格；B5 正文 | “Not applicable — the Claude API does not accept prefilled reasoning”；“Not applicable — Claude’s weights are not released”。 |
| 美国时间29日，特朗普签行政令，要求联邦行政部门依法改用 Super Intelligence（SI）；旧文件不必改 | L1 | ✅ | whitehouse.gov 行政令（`c-order.md`；`../images/17`、`18`） | 页面日期 September 29, 2026。第 2(a) 节：“executive departments and agencies … shall use ‘Super Intelligence’ and ‘SI’ in place of ‘Artificial Intelligence’ and ‘AI’ in official correspondence, public communications, websites, reports, policy documents, and other non-statutory documents”，前提 “To the maximum extent permitted by law”。第 2(b) 节：“Nothing in this section requires the alteration of previously issued regulations, Presidential actions, contracts, grants, or other historical documents.” |
| 并“不承认”AI 一词 | L1 | ✅ | 第 1 节 | “will not acknowledge the usage of ‘Artificial Intelligence’ and ‘AI’ in any applicable setting”。 |
| SI 的定义直接引用美国法典里 AI 的定义；换名暂不换范围 | L1 | ✅ | 第 3(a)、3(b) 节；15 U.S.C. 9401(3)（`c-law-text.md`） | “the terms ‘Super Intelligence’ and ‘SI’ mean the technologies and systems encompassed by the term ‘artificial intelligence’ as defined in section 9401(3) of title 15”；“暂”对应 3(a) “unless and until superseded” 与 3(b) 60 天内提出立法建议。 |
| 不等于超级智能已实现 | — | 编辑判断 | 同上 | 依据：定义范围与现行法定 AI 相同，文件未作技术能力认定。 |
| 白宫同日的说明里，“AI 行动计划”要“赢得 SI 竞赛” | L1 | ✅ | whitehouse.gov Fact Sheet（`c-fact-sheet.md`；`../images/19`） | “In July 2025, the White House released America’s AI Action Plan, identifying more than 90 Federal actions to win the SI race …”。正文为中文意译。Fact Sheet 与行政令同为 9/29。 |
| 截至10月1日凌晨，ai.gov 标题仍是“AI.Gov” | L1 | ✅ | ai.gov（`c-ai-gov-counts.json`；`../images/20`、`27`） | 抓取于北京时间 2026-10-01 00:01；页面 title “AI.Gov \| President Trump's AI Strategy and Action Plan”，可见正文 “artificial intelligence” 11 次、“Super Intelligence” 0 次。行政令未规定网站改名期限，正文不写“违令”。 |
| SI 本是国际单位制的缩写 | L1 | ✅ | NIST SP 330 “The International System of Units (SI)”（`c-sp330.pdf`；`../images/21`） | 常识，正文未列入来源。 |

## 未入正文的已知出入

- 发布页的“1/5”“1/7”“$5.47 对 $23.80”是 OpenAI 自测的每任务成本（DeepSWE、OSWorld 2.0 offline、Terminal-Bench Science），与标题的 token 单价是两种口径；篇幅所限未写。
- DevDay 中文回顾页此前写 Sol 价格为 Astra 的“四分之一”，本次抓取时已改为“五分之一”（`a-devday-zh-comparison.md`）。
- 6.1 Sol 暂未进 ChatGPT 普通对话（“not yet available in Chat”），篇幅所限删去。
- CAISI（NIST）9/17 的评估称 GLM-5.3 是“迄今网络能力最强的开放权重模型”、落后美国前沿约四个月；其 ExploitBench 为 41 题、16 分制、三次取最佳，得分 61.1%（9.8/16），与 Anthropic 的 50/410 评分方式不同，不能并列；另一扫描称“Anthropic 与 CAISI 数字打架”，比的是 ExploitBench 与 SEC-Bench Pro 两个不同基准，不成立。
- 行政令起源：9/22 联大讲话的白宫节选有 “hereinafter officially called ‘Super Intelligence’”；“artificial makes intelligence fake”整句与国务院邮件只见 Euronews（L4）。
- America.gov 同日上线相关内容本期按 Develata 决定不写。
- 热度（抓取时）：GPT-6.1 Sol HN 1028 分 / 893 评论；Anthropic 报告 HN 237 / 225，r/LocalLLaMA 395 票 / 179 评论。

## 反向核验（codex-reviewer，gpt-6-astra high，针对第一稿）

结论：无 blocker。

| # | 发现 | 判断 | 处理 |
|---|---|---|---|
| 1 | SHOULD-FIX：GLM ④ 把两款 Opus 的 4%、10% 泛化为 Claude 整体的“4%–10%”，“0%靠的是 API 防护”强于证据；“无从测”应限定为用户不可行 | 采纳 | 改为“伪装请求下 Claude 均为0%，关防护的两款 Opus 为4%、10%；预填、改权重对用户不可行”；图注同步 |
| 2 | SHOULD-FIX：CAISI ExploitBench 的 16 是评分满分，不是题数（41 题、16 分制） | 采纳 | 已核 `b-nist.md`，本表附注与配图说明改正 |
| 3 | SHOULD-FIX：行政令的“依法”前提与旧文件豁免只在内部材料 | 采纳 | 正文加“依法”“旧文件不必改”；保留“不承认”作为吠点①的铺垫 |
| 4 | NIT：20.4 美元是按智谱 API 价估算，不是实付 | 采纳 | 改为“API 估价20.4美元” |
| 5 | NIT：ai.gov 图注日期应为 10/1 凌晨 | 采纳 | 已改 |

复核通过项：标题“100%绕过？”（报告原句 64%–100% bypass、in simulated tests）；“五分之一不是新降价”（限普通输入/输出单价）；发布时间与“7天”；AA 各数字；Figure 2/5 口径与两个漏洞案例的区分；SI 定义、Fact Sheet 句子与国际单位制。
