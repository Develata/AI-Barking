# 1001 事实核验

标题（定稿）：“Argon解决幻觉？Kimi被点名？”——“解决幻觉”是社区说法（Reddit r/singularity 帖标题 “Gemini 4 Argon solved hallucinations.”，帖文正文写 “may have solved”），正文用 AA 原话拆开；“Kimi被点名”依据 OpenAI 报告 “individuals associated with Moonshot AI, the developer of Kimi”，带问号，正文吠点②写明归因只到“核心集群”。

正文：`../doc_1001_publish.txt`。取证：`evidence.md`（A–D，Codex gpt-6-astra，派工单 `.handoff/2026-10-01-1001-evidence.md`）。Claude 对照存档与官方原页逐条核对：Google 博文、AA 文章、Anthropic 研究页用 curl 独立抓取（本机时间 10/1 13:32，本机为 UTC-3，即北京 10/2 00:32），OpenAI 报告用 opencli 浏览器独立读取，Google 对比表官方原图（GIF）由 Claude 逐格目视对照，AA-Omniscience 指标定义页另行抓取（`e-aa-omniscience-definitions.md`），Reddit 帖文与媒体回应句在存档 JSON 中 grep 复核；PDF 页（Anthropic）与网页截图目视检查。按 EDITORIAL.md，L4 及以下只标 ⚠️。时间口径：OpenAI、Anthropic 官方页只标 “September 30, 2026”，无时刻与时区，正文照写“9月30日”“同日”，不换算北京时间。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题与吠点用语：Reddit 一帖标题写“解决了幻觉” | L6 | ⚠️ | r/singularity 帖 1wuj72j（`d-reddit-a.json`） | 帖子 1305 分 / 246 评，抓取北京 10/2 00:45；发帖北京 10/1 06:46:33。标题 “Gemini 4 Argon solved hallucinations.”，正文 “it looks like Google may have solved hallucinations”。正文只写“标题写”，不写成谷歌或 AA 的说法；热度数字未入正文。 |
| “Kimi被点名”：OpenAI 点名月之暗面（Moonshot AI） | L1 | ✅ | OpenAI 报告（`b-openai-extract.json`；`../images/22-openai-observed.png`） | “we attribute a core cluster of the activity to individuals associated with Moonshot AI, the developer of Kimi.” |
| 省流：谷歌宣布 Gemini-4-Argon，先向网络防御者推出 | L1 | ✅ | Google 博文（`a-google-extract.json`；`../images/02-argon-rollout-price.png`） | “announcing our new frontier model, Gemini 4 Argon, which is rolling out to a set of trusted cyber defenders through our Fairwind Program”；用“宣布”而非“发布”，因未对大众开放。 |
| 省流：OpenAI 称蒸馏行动核心集群涉及月之暗面相关人员 | L1 | ✅ | OpenAI 报告 | 对应 “attribute a core cluster … to individuals associated with Moonshot AI”；反向核验指出旧稿“点名月之暗面蒸馏模型”把相关人员压成公司行为，已改。报告写 “adversarial distillation”，没有展示这些输出实际用于训练 Kimi 的证据。 |
| 省流：Anthropic 估计美国74%的体力任务机器人已能做，多在受限环境 | L1 | ✅ | Anthropic 研究页与 PDF（`c-anthropic-extract.json`；`c-anthropic.pdf`；`../images/41-robots-key-findings.png`） | “robots can already perform 74% of physical tasks in the US”；Key findings “but mostly in limited settings”；官方方法为 Claude 评级的估计，故用“估计”。74% 含相关任务的能力迁移推断（见吠点①），“已能做”按研究的“被判为可做”理解。 |
| 北京时间10月1日凌晨，谷歌宣布 Gemini-4-Argon | L1 | ✅ | Google DeepMind 官方 X 帖 2105388084154056939（`a-official-x.json`） | 帖子时间 2026-09-30T20:03:30Z = 北京 10/1 04:03:30；Google 博文只标 “Sep 30, 2026”、`article:published_time`=2026-09-30，无时刻。扫描里 04:00、05:53 两说：04:00 与官方帖相符，05:53 未找到依据。X 显示文字被浏览器自动翻译，只取时间。 |
| 目前只向受信任者推出 | L1 + L3 | ✅ | Google 博文；AA 文章 | 博文：“rolling out to a set of trusted cyber defenders through our Fairwind Program”；结尾 “the initial cohort of cyber defenders and trusted testers”。AA：“currently being rolled out to selected users and is not publicly available”；AA 模型页多处标 “Not publicly available”（`a-model-extract.json`）。“受信任者”概括网络防御者与受信任测试者。 |
| 大众开放未给日期；小标题与结论“未向公众开放” | L1 | ✅ | Google 博文 | “before making Argon available to developers, enterprises, and consumers as soon as possible”；“starting with paid API customers and Google AI Ultra subscribers”；全文无日期。第二轮核验指出旧稿“没开放/暂未开放”未写开放对象，且与已向受信任者推出冲突，已改“未向公众开放”。 |
| 吠点①：AA 的知识题测试 AA-Omniscience | L3 | ✅ | AA 文章（`a-aa-extract.json`）；AA 指标定义页（`e-aa-omniscience-definitions.md`） | AA 页面：6,000 questions、42 个主题；幻觉率与准确率都是 AA-Omniscience 的指标。 |
| Argon（high 档）没答对的题里错答占15% | L3 | ✅ | AA 文章（`../images/14-aa-omniscience-details.png`、`../images/40-aa-omniscience-panels.png`） | “Gemini 4 Argon has a 15% hallucination rate”；AA 定义：幻觉率 = 错答 ÷（错答 + 部分答对 + 未作答），即没答对的题里错答占比，不是全部回答的错误率。图 40 Non-Hallucination Rate 面板 Argon 85%（=1−15%）居首。Argon 为 “high reasoning (the highest available)”。 |
| 为综合指数45分以上模型最低 | L3 | ✅ | AA 文章 | “the lowest of any model scoring 45+ on the Intelligence Index”；45 分门槛属于 Artificial Analysis Intelligence Index，不是 AA-Omniscience Index；“综合指数”指前者。 |
| 答对率仅50%，低于 Astra（max 档）的63% | L3 | ✅ | AA 文章；图 40 Accuracy 面板 | “scores 50%, a 5 point decrease from Gemini 3.1 Pro Preview, and 13 points below GPT-6 Astra (max, 63%)”；AA 定义：准确率 = 答对题数 ÷ 全部题目。Argon（high）对 Astra（max），均取各自列出的最高档；AA-Omniscience 总分 42，Astra 43、GPT-6.1 Sol 42。 |
| 更常答“不知道”，不等于更懂 | L3 + 判断 | ✅ | AA 文章 | AA 原话：“Argon is much more likely to acknowledge when it does not know an answer rather than guess incorrectly.”；“不等于更懂”是据准确率下降（比 Gemini 3.1 Pro Preview 低5点、比 Astra 低13点）作的编辑推论，不是 AA 的话，也不扩大为整体能力结论。 |
| 吠点②：限时首发价每百万 token 输入2美元、输出10美元 | L1 | ✅ | Google 博文（`../images/02-argon-rollout-price.png`） | “Argon will launch at an introductory price¹ of $2 per million input tokens and $10 per million output tokens, with cached input tokens priced at 95% off input token price.” “限时首发价”译 introductory price。 |
| 期满后4美元、20美元 | L1 + L3 | ✅ | Google 博文脚注 1（`../images/07-argon-price-footnote.png`）；AA 文章 | 脚注：“After the introductory period expires, the price of $4 per 1M input tokens and $20 per 1M output tokens will apply.” 具体日期 Google 未给；AA 称 “currently discounted 50% to $2/$10 for at least one month”，并写明 Google 未确认促销截止日。 |
| 吠点③：谷歌自家对比表19项（来源与测试条件不统一） | L1（厂商自评） | ✅ | Google 博文官方原图 `gemini-4-argon_table_blog.gif`（`a-table.gif`；`../images/08-argon-benchmark-table.png`）；方法页 PDF（`a-methodology.txt`；`../images/09-argon-methodology.png`） | 19 行 × 4 列（Argon / GPT-6 Astra / Claude Fable 5.1 / Claude Opus 5.5），逐行抄录见 `evidence.md` A5；表中没有 GPT-6.1 Sol、Claude Sonnet 5.5。方法页：“All the results for non-Gemini models are sourced from providers’ self reported numbers unless otherwise mentioned”；DeepSWE v1.1、Terminal-Bench 4.0 的 Argon 成绩为自测，Astra 等取自榜单或系统卡，PostTrainBench、LABBench2、GraphWalks 为各模型自测，OSWorld 2.0 另有 3 次取最大等条件；故括注“来源与测试条件不统一”。厂商自家对比，非独立评测。 |
| Argon 最高13项、并列1项、落后5项 | L1 | ✅ | 同上 | Claude 目视逐行复核官方原图，与 Codex 抄录一致，codex-reviewer 另独立重算相同。最高：Vals Index、AutomationBench、Vals Finance Agent v2、Harvey’s Legal Agent Benchmark、DeepSWE v1.1、Vibe Code Bench、LABBench 2、RiemannBench、GraphWalks×2、Agent’s Last Exam、Chartography、LVBench（13）；并列：CWE-bench v1（68.0 对 Astra 68.0）；落后：FrontierSWE v2、Terminal-bench 4.0、PostTrainBench、Terminal-Bench Science 0.1、OSWorld-2.0（5）。“最高”只在表内4个模型中比较，缺值不补零。 |
| 二：9月30日，OpenAI 披露套取隐藏推理的“蒸馏”行动 | L1 | ✅ | OpenAI 报告（`b-openai-extract.json`；`../images/21-openai-title.png`） | 页面日期 “September 30, 2026”，无时刻、时区；官方 X 帖未找到。“protected reasoning”译作“隐藏推理”（报告后文用 “hidden reasoning content”）；“adversarial distillation”译作“蒸馏”。已取存档的三家媒体中最早为 The Register，页面 21:36 UTC = 北京 10/1 05:36（搜索视图曾显示 22:36 UTC，两处不一致）；故“9月30日”在北京时间可能已是 10/1，正文不加“美国时间”以免断言时区。 |
| 7月24、25日1.6万次请求、4000多名用户 | L1 | ✅ | 同上（`../images/22-openai-observed.png`） | “high-volume spikes on July 24 and 25 consisting of 16,000 requests using a relevant extraction pattern from over 4,000 users”；原文是 16,000，不加“约”。 |
| 关联集群超1.5万名用户，至7月28日已阻断 | L1 | ✅ | 同上 | “related prompt-pattern activity across a cluster of more than 15,000 users, which we fully disrupted by July 28”；原文是 users，不改“账号”；“by July 28”译“至7月28日已”。 |
| 核心集群归因于月之暗面（Kimi）相关人员 | L1 | ✅ | 同上 | “we attribute a core cluster of the activity to individuals associated with Moonshot AI, the developer of Kimi.” |
| 吠点①：1.6万是尝试，不一定成功 | L1 | ✅ | OpenAI 报告脚注 1（`../images/24-openai-footnote.png`） | “These figures describe attempted, not necessarily successful, extractions.” CNBC、CyberScoop、The Register 本次可见正文均未写成“1.6万次成功”（`evidence.md` B6）。 |
| 吠点②：OpenAI 称不确定全部操作者是同一主体 | L1 | ✅ | 同上 | “It is unclear whether all operators we observed during the relevant time period originated from a single actor.” |
| 页面未给归因依据 | L1（缺失） | ✅ | OpenAI 报告全文通读 | 归因节只有两句，未列账号、网络或模型输出等依据；只限该页，不代表别处未披露；codex-reviewer 通读同样成立。 |
| CNBC、The Register 称月之暗面未立即回应 | L4 | ⚠️ | CNBC（`b-cnbc-extract.json`）；The Register（`b-register-extract.json`） | CNBC：“Moonshot did not immediately respond to CNBC’s requests for comment.” The Register：“reached out to Moonshot AI for comment and did not receive an immediate response.” 仅限各自截稿；CyberScoop 只写已联系置评。Moonshot/Kimi 官网、官方 X、研究博客、微博、公众号未找到本事件回应（检索边界见 `evidence.md` B5，不宣称完整检索）。 |
| 三：同日，Anthropic 估计机器人已能做美国74%的体力任务（约占全部工时的34%） | L1 | ✅ | Anthropic 研究页与 PDF（`../images/41-robots-key-findings.png`） | 页面 “Sep 30, 2026”。“Robots … can perform three-quarters of physical tasks in the US, making up 34% of working hours, but mostly in limited settings”；正文 “74% of physical work, or 34% of all work, can be done by robots in some circumstances”。 |
| 成本低于人工的任务仅占全部工时0.3% | L1 | ✅ | PDF 页 2、19（`../images/41-robots-key-findings.png`、`../images/42-robots-cost-70pct.png`） | Key findings：“Robots are cost-competitive for just 0.3% of job tasks.”；页 19：“Though robots can theoretically do tasks that add up to 34% of all work time, they are cost-competitive for just 0.3% today.” 任务按时间与就业加权，故分母是全部工时；定义：“A robot is cost-competitive when it can do the same task for cheaper than a human worker.” |
| 吠点①：74%按时间加权、不是岗位；任务内可做情形的时间合计至少一半才计入 | L1 | ✅ | PDF 页 6、7；附录页 6–7（`../images/43-robots-majority-rule.png`、`../images/45-robots-appendix-a4.png`；`c-appendix.txt`） | 页 6：“Task exposure is set by majority rule: the least structured environment in which robots can do at least half of a task’s examples, weighted by time.” 附录公式：exposure_t = max{e: Σ 1[exposure_it ≥ e]·w_it ≥ 0.5}，w_it 为情形权重（按时间）。官方未把 74% 写成岗位占比；74% 的分母是物理任务时间（34% 是全部工时）。第二轮、第三轮核验指出“一半情形”须按情形的时间权重理解（例：只能做的一种情形占 60% 时间也可计入），已改。 |
| 含相关任务的能力迁移推断，排除后约一半 | L1 | ✅ | 研究附录 A.4（`c-appendix.pdf` 页 7；`c-appendix.txt` 第 193–207 行；`../images/45-robots-appendix-a4.png`） | “Sometimes Claude can’t find a robot that does a task exactly as written, but counts the task as exposed when robots do related ones.” “Excluding ratings that rely on related robots decreases the share of exposed physical work from about three-quarters to a half. Though these cases require judging whether capabilities transfer across tasks, they show which tasks robots could likely do, or are close to doing, with today’s technology.” 第三轮核验（BLOCKER）发现旧稿遗漏；约一半是“排除这类评级后的暴露份额”，不是实测完成率。 |
| 吠点②：成本降约70%，才对10%的工时划算 | L1 | ✅ | PDF 页 19（`../images/42-robots-cost-70pct.png`） | “For robots to be cost-competitive for 10% of human work today, costs would need to decline about 70%.” Figure 8 图注：任务按时间与就业加权，故“工时”。 |
| 按年降3%、任务与工资不变推算，约需40年 | L1 | ✅ | PDF 页 19、20（`../images/42-robots-cost-70pct.png`、`../images/38-robots-cost-decline.png`）；页 2 Key findings（`../images/41-robots-key-findings.png`） | “At a 3% decline per year, that would take around 40 years. That said, it’s possible that new, more capable robots like humanoids or new manufacturing processes will drive down costs”；页 20：“We hold fixed tasks and wages and ask how soon historical rates of cost declines and capability gains would make robots cost-competitive.”；摘要 “If robot price declines follow past trends, it will take 40 years”；“roughly 3% per year since the 1990s”。是情景推算，不是就业预测；codex-reviewer 复算年降3%累计降70%约需 39.5 年。旧稿“属外推”改为写明固定条件。 |
| 吠点③：评级与成本估算由 Claude 联网完成 | L1 | ✅ | PDF 页 4–6、16、22；网页脚注 10（`c-anthropic-extract.json`；`../images/43-robots-majority-rule.png`、`../images/44-robots-cost-method.png`） | “we have Claude score task descriptions”；“Claude then scores exposure for these examples using web search”；页 16：“We prompt Claude to estimate costs using web search and the list of robots cited for each task.”；“rely on Claude to judge which machines meet this criterion”。作者同时写有稳健性检查与附录（`c-appendix.pdf`），不是“全靠自评”。 |
| 作者称成本为近似值 | L1 | ✅ | PDF 页 17（`../images/36-robots-cost-example.png`） | “these cost estimates are approximate”（Firms face adoption frictions … and these cost estimates are approximate）。 |
| 一句话：Argon 未向公众开放，归因只到核心集群，74%不等于岗位被取代 | — | ✅ | 见上各行 | 对应 Google “rolling out to … trusted cyber defenders”与 AA “not publicly available”、OpenAI “core cluster”、Anthropic 74% 为物理任务时间口径。 |
| 来源：谷歌、Artificial Analysis、Reddit、OpenAI、CNBC、The Register、Anthropic | — | ✅ | 见上各行 | Reddit 为 L6、CNBC 与 The Register 为 L4，正文均已用“标题写”“称”限定。 |

## 已核实但未写入正文（备选，可用于更正或跟进）

| 事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| DeepSWE v1.1 的77.9%是谷歌自测 | L1 | ✅ | Google 方法页 PDF（`a-methodology.pdf`、`a-methodology.txt`；`../images/09-argon-methodology.png`） | “DeepSWE v1.1 results for Gemini 4 Argon are self computed, using a mini-swe agent harness. GPT-6 Astra results are reported from the official public leaderboard, Fable 5.1 and Opus 5.5 results are taken from their respective system cards.” 博文称 DeepSWE v1.1 “new state of the art (77.9%)”；Terminal-Bench 4.0、Terminal-Bench Science 0.1、OSWorld 2.0 等的 Argon 成绩也是自测。定稿因篇幅删去，正文以“谷歌自家对比表”点明厂商自评。 |
| 如 FrontierSWE v2 55.0%对 Astra 65.5% | L1 | ✅ | 同官方对比表（`../images/08-argon-benchmark-table.png`） | 表内数值：Argon 55.0、Astra 65.5、Fable 5.1 56.3、Opus 5.5 62.3。定稿因篇幅删去，以补入“来源与测试条件不统一”的限定。 |
| OpenAI 称未破解加密、未入侵数据库 | L1 | ✅ | OpenAI 报告开篇（`../images/21-openai-title.png`） | “The operators did not break our encryption, compromise a database, or gain direct access to stored user conversations.” 另写手法是复制一段对话里的加密推理，让另一段对话里的模型解密并转录。定稿因篇幅删去。 |
| 约一半体力任务只能在专为机器人搭建的环境里做（E1 49.8%） | L1 | ✅ | Figure 2（`../images/33-robots-figure2.png`） | 物理任务份额：E0 26.3%、E1 49.8%、E2 22.0%、E3 1.9%；E1 为 “Purpose-built robotic work environment”；正文 “Robots can do half of physical tasks in purpose-built environments but not in wider settings (E1).” 分母是物理任务，不是那 74%。；第三轮改稿为腾字数给能力迁移吠点而删去。 |
| 马路等非结构化环境仅约2% | L1 | ✅ | Figure 2 / 页 7 | E3 1.9%；正文 “Today’s robots do only 2% of physical tasks in unstructured environments (E3).” 定稿因篇幅删去。 |
| 对受信任防御者和谷歌内部团队“不带网络护栏” | L1 | ✅ | Google 博文（`../images/05-argon-cyber.png`） | “For trusted defenders and our own internal teams at Google, we’ll be releasing Argon without cyber guardrails”；限这两类对象。 |
| 内存节省300 TiB 是 “once rolled out”，Zircon 80万行 C/C++→Rust 仍在审计 | L1 | ✅ | Google 博文（`../images/03-argon-internal-work.png`） | 扫描里写成“已省下”“已迁移”，与原文不符。 |
| Terminal-bench 4.0：Google 表中 Opus 5.5 为66.4%，AA 为60% | L1 / L3 | ✅ | 对比表；AA 文章（`../images/13-aa-agent-benchmarks.png`） | 两处数值冲突，来源条件不同，未取舍（`evidence.md` A10）。AA 另写 Argon 57%，低于 Sonnet 5.5（max）64%、Opus 5.5（max）60%、Astra 59%。 |
| 成本：对 Astra 约六成，对 GPT-6.1 Sol（max）2.7倍 | L3 | ✅ | AA 文章 | $1.99 对 $3.26 与 $0.72；折扣结束后 $3.98（约 Astra 的1.2倍）；Argon 每任务输出约 62k token，Astra 约 27k。 |
| AA 编码 agent 对比：Antigravity CLI + Argon 64 对 Codex + GPT-6.1 Sol（xhigh）63 | L3 | ✅ | AA 对比页（`../images/17-aa-agents-table.png`） | 成本约 $5.84 对 $1.04 每任务，耗时 34.5 对 15.5 分钟；是 API 成本，不含环境启动与评审时间。 |
| Bloomberg：谷歌内部对 Argon 编码等表现有疑虑 | L4 | ⚠️ | Bloomberg 导语（`a-bloomberg-extract.json`） | 仅核到付费墙外导语（9/30 19:52 UTC 发布、21:05 UTC 更新）；员工原话、谷歌否认、盘后涨幅未核到，不入正文。 |
| OpenAI 官方页写明合作方托管部署还需同等保护 | L1 | ✅ | OpenAI 报告（`../images/23-openai-partners.png`） | “Partner-hosted deployments need the same protections as first-party services”。研究者 9 月更新另记 9/27 Azure 缓解、9/28 原提取无法复现（`b-update.txt`，Claude 已核到时间线），不能只引旧状态。 |
| The Register 标题称 “Chinese model stole its special IP” | L4 | ⚠️ | The Register（`../images/27-register-title.png`） | 讽刺口吻、归属于 OpenAI 的指控；比官方 “attempted / core cluster” 更肯定，未写“1.6万次成功”。 |
| 打包工例子：机器人组合超200万美元、约14人年、每人年约4.5万对人工4.9万、约省2,500美元 | L1 | ✅ | PDF 页 17 正文、页 18 Figure 7（`../images/36-robots-cost-example.png`、`../images/37-robots-figure7.png`） | 近似估算；robotaxi 的 $7,000 是比司机贵的差额。 |
| 另一篇 Anthropic 机器人研究：低层直接操控完整任务成功率 0–5.5% | L1 | ✅ | anthropic.com/research/claude-plays-robotics（7/9） | MuJoCo/LIBERO 控制实验，不是就业暴露研究的口径，不可混用。 |
| Anthropic 官方 X 帖、OpenAI 官方 X 帖时间戳 | — | 未找到 | — | 扫描称北京 10/1 00:09、02:21，均无法核实，正文不用。 |

## 热度（抓取北京 10/2 00:45–00:57，非扫描时点）

Argon：HN 1579 分 / 1047 评；r/GeminiAI 1538 / 220、1349 / 236；r/singularity “solved hallucinations” 1305 / 246；DeepMind 官方帖 780.9 万查看。蒸馏：HN 6 分 / 0 评、4 / 1；r/accelerate 71 / 49；Techmeme 聚合簇 12 条目（1 官方 + 11 媒体）。机器人：r/artificial 5 / 3；Yahoo Finance 跟进（北京 10/1 19:59）；HN、X 未定位到相关帖。机器人一条社交热度偏低，靠媒体与聚合站跟进过门槛；若需替换，备选为 SynthID Bio。

## 反向核验

第一轮（`codex-reviewer`，gpt-6-astra，medium，WSL 只读，约 5 分钟）：只读检查规范、逐句正文、核验材料与官方原图；2 项 BLOCKER、4 项 SHOULD_FIX、1 项 NICE_TO_HAVE，另有 1 项在线复核缺口。

| # | 发现 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER：省流“OpenAI 点名月之暗面蒸馏模型”把“相关人员”压成公司行为 | 采纳（对照 OpenAI 原文成立） | 省流改为“OpenAI 称蒸馏行动核心集群涉及月之暗面相关人员”；本表对应行改写 |
| 2 | BLOCKER：AA 的15%与50%缺测试范围与分母，易读成日常错误率 | 采纳；Claude 另抓 AA 定义页核实（`e-aa-omniscience-definitions.md`） | 正文改为“AA 的知识题测试 AA-Omniscience：没答对的题里错答占15%…答对率仅50%”，配图说明补公式 |
| 3 | SHOULD_FIX：准确率比较遗漏推理档位 | 采纳 | 正文标“Argon（high 档）”“Astra（max 档）”；本表注明各取最高档 |
| 4 | SHOULD_FIX：0.3% 与 10% 的分母是全部工时，封面并排两个百分比易暗示同一分母 | 采纳（页 19 原文成立） | 正文改“成本低于人工的任务仅占全部工时0.3%”“10%的工时”；封面重生成，牌上写“体力任务能做74%”“仅0.3%工时划算” |
| 5 | SHOULD_FIX：“已能完成”遗漏多数规则判定门槛 | 采纳（页 6 原文成立） | 正文改“估计…能做”“能做至少一半情形即计入”；新增配图 43 |
| 6 | SHOULD_FIX：配图说明三处不准确（灰底含并列；over/more than/by；Figure 8 图注无 Claude） | 采纳 | `images/README.md` 已改 |
| 7 | NICE_TO_HAVE：“未公开”“入门价”“恢复”措辞 | 部分采纳 | “没开放/暂未开放”“限时首发价”“期满后4美元、20美元”；“未向公众开放”因字数用“开放未给日期”表达 |
| — | NEEDS_VERIFICATION：CNBC 原页、Google 主站页本轮未能在线打开 | 已有本地存档支撑 | CNBC“未立即回应”句由 Claude 在存档 `b-cnbc-extract.json` 中 grep 到原句；Google 主站页由 Claude 直接 curl 读取 |

复核通过项（审查原文）：官方表 19 行重算为 13 独占最高 + 1 并列 + 5 落后；5 项落后与 55.0%／65.5% 吻合；Reddit“标题写”准确；AA 15%、50%、63%、综合指数 ≥45 限定均有原文；OpenAI 请求数、用户数、核心集群、相关人员、尝试未必成功均有出处；机器人数字 74%、34%、49.8%、1.9%、70%、3%、40 年均有出处；“页面未给归因依据”经通读成立。

第二轮（`codex-reviewer`，gpt-6-astra，medium，WSL 只读，约 4.5 分钟；附两张封面与图 42、43）：1 项 BLOCKER、6 项 SHOULD_FIX、1 项 NICE_TO_HAVE、1 项在线复核缺口。

| # | 发现 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER：两张封面把标着 OpenAI、Kimi 的杯子用吸管连通并挂上“1.6万次尝试”，扩大了归因（OpenAI 只把一个核心集群归于月之暗面相关人员，没有把全部请求归给他们） | 采纳（对照 OpenAI 原文成立） | 封面第 5 组重画：只有一个“OpenAI”杯子，吸管通向一群无面剪影，标签“1.6万次尝试，不一定成功”，只把其中一小撮圈出并写“核心集群：月之暗面相关人员”；不再有 Kimi 杯子与吸管直连。`images/README.md` 撤回旧“不把归因画成定论”的验收结论并重写 |
| 2 | SHOULD_FIX：“没开放/暂未开放”缺开放对象，与已向受信任者推出冲突 | 采纳 | 小标题与结论改“未向公众开放” |
| 3 | SHOULD_FIX：封面“体力任务能做74%”与“仅0.3%工时划算”并置，限定仍不完整；省流缺“美国”“特定环境” | 采纳 | 封面删去 74% 数字牌，改“能做，未必划算”，价签写“全部工时仅0.3%划算”；省流改“估计美国74%的体力任务机器人已能做，多在受限环境” |
| 4 | SHOULD_FIX：多数规则的“一半情形”须按时间加权 | 采纳（页 6 原文成立） | 正文改“按时间加权，任务里能做至少一半情形才计入” |
| 5 | SHOULD_FIX：13／1／5 计数正确，但各项来源与测试条件不统一，正文未交代 | 采纳（方法页成立） | 正文括注“来源与测试条件不统一”，为腾字数删去 FrontierSWE 例子；图 08 说明写明自测与榜单、系统卡的混合 |
| 6 | SHOULD_FIX：图 42 说明“0.3%对应34%”含混，段落定位不准 | 采纳 | 说明改为“倒数第二段：34%与0.3%均以全部工时为分母，分别衡量能力与成本竞争力；最后一段才是70%与40年”；另增图 44（页 16）作“Claude 联网估算”的锚点 |
| 7 | SHOULD_FIX：封面文字超出 EDITORIAL “主标题另加至多一行关键数字或副标题” | 部分采纳 | 该句管标题区；本仓库历期封面（如 0930 有 8 处）均带主体标注，规范不改。已删去风险最大的数字标签（74% 牌）与第二个杯子标签，其余标注保持逐字可核 |
| 8 | NICE_TO_HAVE：40 年情景应写明固定任务与工资 | 采纳（附录页 20 成立） | 正文改“按年降3%、任务与工资不变推算，约需40年” |
| 9 | NEEDS_VERIFICATION：CNBC、The Register 原页与 Google X 本轮未能在线复读 | 区分记录 | 这三项以本地存档为准：CNBC、The Register 的“未立即回应”句在 `b-cnbc-extract.json`、`b-register-extract.json` 中逐字存在（Claude grep 复核），官方 X 时间在 `a-official-x.json`；不把访问失败当作事实错误，也不声称本轮在线重验成功 |

复核通过项（审查原文）：AA 定义（15% 分母、50%/63% 准确率、high/max）；价格与脚注；官方表独立重算 13/1/5 与 FrontierSWE 55.0/65.5；OpenAI 请求量、用户量、尝试性质、“至7月28日已阻断”、页面未给归因依据；Reddit“标题写”；日期照录；机器人数字 34%、0.3%、49.8%、70%、3%、约 39.5 年；“Claude 联网”有直接支持；11 张正式配图均有正文锚点；标题 18 字符、全文 992 字符。

第三轮（`codex-reviewer`，gpt-6-astra，medium，WSL 只读，约 18 分钟；附两张封面与图 43、44）：1 项 BLOCKER、3 项 SHOULD_FIX、1 项 NICE_TO_HAVE、1 项在线复核缺口。

| # | 发现 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER：“74%已能做”遗漏附录 A.4 的能力迁移假设——Claude 在找不到对应机器人时按相关任务判为暴露，排除后体力工作暴露份额从约四分之三降到约一半 | 采纳（Claude 对照 `c-appendix.txt` 第 193–207 行与附录页 7 成立） | 正文吠点①加“含相关任务的能力迁移推断，排除后约一半”；新增配图 45；为腾字数删去“约一半体力任务只能在专用环境里做”（改入备选，配图 33 转备用） |
| 2 | SHOULD_FIX：封面“全部工时仅0.3%划算”缺美国与研究估计的限定 | 采纳 | 封面第 8 组删去价签与全部数字，只留“能做，未必划算” |
| 3 | SHOULD_FIX：“至少一半情形”仍有按数量计数的歧义 | 采纳（附录公式的情形权重 w 成立） | 正文改“任务内可做情形的时间合计至少一半才计入” |
| 4 | SHOULD_FIX：封面文字量不符 EDITORIAL 字面“主标题另加至多一行关键数字或副标题” | 不改规范，如实记录 | 历期封面（如 0930）均带多处主体标注，规范未写明漫画内标注的边界；本期已删去数字标签并保持标注逐字可核。这是规范解释问题，留给 Develata 决定是否在 EDITORIAL 里写明 |
| 5 | NICE_TO_HAVE：配图说明称两版均为“全部工时仅0.3%划算”，竖版实际无“仅” | 不成立 | Claude 放大核对：竖版为“全部工时仅/0.3%划算”，横版为“全部工时/仅0.3%划算”，两版都有“仅”；该价签随第 8 组删去 |
| 6 | NEEDS_VERIFICATION：Google 页、方法页、官方 X、CNBC、The Register 本轮无法在线复读 | 区分记录 | 以本地存档为准，不声称本轮在线重验；发布前可在能访问的环境复读 |

复核通过项（审查原文）：AA 分母与档位、Argon 开放与价格、19 项表重算 13/1/5、OpenAI 请求量与用户量与归因范围、封面 OpenAI 面板（单杯、只圈部分剪影、明确“OpenAI称”）无把全部请求归给 Kimi 的画面结论、12 张正式配图均有正文锚点；标题 18 字符、全文 996 字符。

第四步（封面第 8 组）由 Claude 逐字放大核对，未再送审：第 8 组只是在已通过的第 7 组上删去数字价签。
