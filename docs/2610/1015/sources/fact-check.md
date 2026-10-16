# 1015 事实核验

取证：A、B、C 组 Sonnet 5.5 子代理，D、E 组 Haiku 5.5（均 effort high），详见 `evidence-a.md`–`evidence-e.md`。选题前 Claude 另派两个 Sonnet 核验代理复核扫描说法。Claude 抽查了决定性条目的一手原文：商务部公告与答记者问原页（Claude 本人抓取）、附件 1 PDF 文本（`a-annex1-pdftotext.txt`）、Meta 博客与 HF 许可文本（`b-hf-api.json`）、AA 内嵌数据（`b-aa-ranked-all.json`，Claude 重新筛选）、ARC Prize 结果页与 Astra 博客（截图 25、26、29 目视）、速览各条原页。日期按北京时间，官方只给日期的不写时刻。

## 主帖一：AI 技术出口要登记？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间10月16日16:00，商务部、科技部公告 | L1 | ✅ | https://www.mofcom.gov.cn/zcfb/dwmygl/art/2026/art_61446ab4beee4f63973d4d0ab3097648.html（`a-a-notice.txt`；`../images/01-notice-title-scope.png`） | 页面时间“2026-10-16 16:00”；“商务部 科技部公告2026年第48号” |
| 拟在技术出口禁限目录中增列 AI、生物医药、光伏三个领域40项技术，其中 AI 23项 | L1 | ✅ | 同上 | “拟修订《中国禁止出口限制出口技术目录》，增列部分人工智能、生物医药、光伏技术”；“涉及人工智能、生物医药、光伏三个领域40项技术。人工智能领域包括大模型、智能体、算力集群、信息处理相关23项技术”。正文“技术出口禁限目录”是《中国禁止出口限制出口技术目录》的简称 |
| 已开源或发表论文进入公共领域的技术不纳入 | L1 | ✅ | 公告（`a-a-notice.txt` 25 行）；附件 1 首页脚注 | “对已通过开源、发表学术论文等方式进入公共领域的技术，不纳入管理范围。”（第一轮反向核验 SHOULD_FIX 8 补入） |
| 征求意见到11月15日 | L1 | ✅ | 同上 | “公众可在2026年10月16日-11月15日期间……提出意见” |
| 所列技术出口前须登记，即日实行 | L1 | ✅ | 同上（`../images/02-notice-feedback-registration.png`） | “自本公告发布之日起，至《中国禁止出口限制出口技术目录》正式修订并实施之日止，技术出口经营者在出口本公告附件所列技术前，应当在……登记。已经开始但尚未完成的出口，在登记范围内。”登记情形限四类（转让所有权、授权境外使用、所有权人或实际控制人变为境外主体、商务部明确要求的其他情形），正文未展开 |
| 40项中拟禁止14项、拟限制26项（三个领域合计，征求意见稿）；正文：拟禁止的14项是三个领域合计 | L1 | ✅ | 答记者问 https://www.mofcom.gov.cn/zwgk/zcjd/art/2026/art_b11d5c1e9374421e93e85dc87eba6e38.html（`a-a-qa.txt`；`../images/04-qa-40-14-26-counts.png`） | “涉及拟增列的人工智能、生物医药、光伏三个领域40项关键技术。其中，拟禁止出口技术14项……拟限制出口技术26项” |
| 按附件逐条数，AI 23项中拟禁止6项、拟限制17项 | L1 | ✅（计数口径为推断，已在正文写“按附件逐条数”） | 附件 1（官方 `.wps`，`a-annex1.wps`；LibreOffice 转 `a-annex1.pdf`、`a-annex1-pdftotext.txt`；`../images/05`、`06`） | 附件表格 AI 部分只有 11 行；按行内编号的控制要点数：禁止 266401J 1 + 266402J 3 + 266501J 1 + 266502J 1 = 6；限制 266402X 2 + 266404X 1 + 266501X 2 + 266504X 2 + 266505X 4 + 266506X 2 + 266507X 4 = 17。同口径生物医药 3+2=5、光伏 5+7=12，禁止合计 14、限制 26、总计 40，与公告和答问全部吻合。Claude 逐行复核了 A 组表（`evidence-a.md` §2）的加总 |
| 答记者问称，目录正式实施后，用限制类大模型对外提供 API、SaaS、App 或网页交互等，不属于大模型技术出口管理范围；限制类 AI 技术在海外本地化部署则属于 | L1 | ✅ | 答记者问（`a-a-qa.txt`；`../images/03-qa-api-saas-local-deploy.png`） | 问：“《目录》正式实施后，人工智能大模型在海外本地化部署、向境外提供模型调用等各类‘出海’形式，是否属于技术出口管理范围？”答：“对于纳入限制类的人工智能技术，开展海外本地化部署，属于技术出口管理范围；如果部署在海外自有云等风险相对可控的情形下，可适用自动许可等便利措施。对于企业利用限制类大模型对外提供API调用、APP/网页交互、SaaS应用、本地端侧轻量化部署等情形，不属于大模型技术出口管理范围。”正文省略“端侧轻量化部署”与“海外自有云可适用自动许可”，以“等”承接 |
| 已删：大模型条目指标（重要数据训练；10^26 次浮点运算、2万亿参数、公开评估全球前10） | L1 | — | 附件 1 266504X（`a-annex1-pdftotext.txt` 137–163 行） | 篇幅原因删去；原文“具有以下任一特性的大模型的核心技术，包括权重参数、关键检查点、底层源代码、架构设计，以及训练优化、推理加速等技术：（1）计算量≥10^26次浮点运算；（2）参数量≥20,000 亿；（3）能力达到全球领先水平。” |

## 主帖二：美国最强开放模型？

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Meta 10月15日开放 Muse Spark Open 权重，总参数8160亿 | L1 | ✅ | https://research.meta.ai/blog/muse-spark-open（`b-meta-blog.txt`；`../images/13-meta-title-intro.png`） | 页面日期 October 15, 2026，官方未给时刻；“816B total parameters (42B active) and a 1M-token context window” |
| 称它是“迄今能力最强的美国开放权重模型” | L1 | ✅ | 同上 | 标题“The Most Capable US Open‑Weight Model to Date”；正文“the most capable US open-weight model to date” |
| 据 AA 评测，截至北京时间16日晚，它的综合指数39 | L3 | ✅ | https://artificialanalysis.ai/models/muse-spark-open（`b-aa-model.txt`；`../images/15-aa-muse-summary.png`） | Xhigh 档；页面显示 39，内嵌值 38.87；Intelligence Index v4.3.2；抓取 2026-10-16 19:39 |
| AA 收录的开放权重模型里排在它前面的9个都来自中国厂商 | L3 | ✅ | AA 页内嵌数据（`b-aa-ranked-all.json`，693 个有指数的模型） | Claude 筛选 open=true 且指数 >38.87：MiMo-V2.6-Pro 46.32（小米）、GLM-5.3 (Max) 44.78（Z AI）、Step 5 Preview 43.73（StepFun）、Kimi K3 (Max) 43.59、GLM-5.3-Flash 41.81、Ling 3.1 Flash 41.09（InclusionAI）、Qwen3.8 2.4T A95B 39.95、Qwen3.8-Flash-Next 39.82、DeepSeek V4.1 Flash (Max) 39.46，country 均为 cn；美国开放权重第二为 Nemotron 3.5 Super VL 26.49。口径：AA 抓取时刻的收录范围 |
| 已删：Meta 模型卡也列着 GLM-5.3 为45、Kimi-K3 为44 | L1 | — | HF 模型卡 https://huggingface.co/meta-models/Muse-Spark-Open-816B-A42B（`b-hf-page.txt` 525 行起）；评测方法 PDF（`b-meta-eval-pdf.txt`） | “Artificial Analysis Intelligence Index”一行：Muse 39 / Inkling 26 / Nemotron 3 Ultra 23 / Beam - / Mistral Large 4 38 / Qwen3.8-2.4T 40 / GLM 5.3 (max) 45 / Kimi K3 (max) 44；PDF：AA 分数取自 2026-10-07。博客里的基准表图没有这一行。第二轮 SHOULD_FIX 2：Muse 为 xhigh、GLM/Kimi 为 max，且是 10/7 的 AA v4.3 快照，与本期 10/16 v4.3.2 存档并列易误读，删去 |
| Meta 称两台 M5-Ultra Mac Studio 就能在桌面跑 | L1 | ✅ | Meta 博客第 22 行 | “Muse Spark Open is the first model in its class to run on your desktop using two Apple M5 Ultra Mac Studios with the latest MLX framework.” |
| 据 Apple 测试，4位量化后权重约 493GB，单台装不下 | L1 | ✅ | https://machinelearning.apple.com/research/mlx-distributed-inference（`b-apple.txt`） | “In a 4-bit quantized (NVFP4) format, the model memory footprint is lowered to about 493GB. Add the memory needed for intermediate computations during inference, and the model cannot fit on a single machine.” |
| 8千 token 上下文时两台每秒约16–25个 token | L1 | ✅ | 同上 Table 3 | 测试配置：M5 Ultra Mac Studio（单台最高512GB统一内存），MLX 分布式推理，4-bit NVFP4。Decode speed，8,192 上下文：2 M5 Pipeline 15.74、2 M5 Tensor 24.99（4 M5 Tensor 36.73）；65,536 上下文为 14.37 / 23.84 / 34.66（B 组用 opencli 切换标签读出） |
| 已删：许可门槛（关联方合计年收入超2000万美元提供模型服务须另签协议；须标注“Built with Muse Spark Open”） | L1 | — | HF 许可文本（`b-hf-api.json`） | 篇幅原因删去；第 3 条“the aggregate gross revenue of the Licensee and its affiliates exceeds twenty (20) million US dollars … must notify Meta and enter into a separate written agreement”；第 2 条署名要求 |

## 主帖三：Argon 考了97.5%？

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 北京时间16日凌晨，ARC Prize 公布 Gemini-4-Argon 的 ARC-AGI-3 成绩 | L3 | ✅ | ARC Prize 帖（fxtwitter JSON，`c-*`；`../images/32-arcprize-tweet.png`） | 2026-10-15 16:22:30 UTC = 北京 10/16 00:22。结果页标的 Oct 13, 2026 是模型日期 |
| ARC-AGI-3 成绩（均为低推理档）：用厂商设计的上下文管理得97.5%，换成统一接口的标准框架只有49.8% | L3 | ✅ | https://arcprize.org/results/google-gemini-4-argon（`c-arc-gemini4-argon.txt` 129 行；`../images/25-argon-summary.png`、`26-argon-verified-table.png`） | 两者均为 low 推理档：“49.8% with the Standard harness at low reasoning and 97.5% with the Provider Adapter harness at low reasoning”。定义见政策页（`c-arc-policy.txt` 243–245 行）：Standard “minimal, provider-neutral interface”；Provider Adapter “provider-designed context-management features, such as preserving opaque reasoning state between requests and using compaction” |
| “仅次于 GPT-6-Astra 的99.9%”比的是两家各自的适配成绩，且推理档不同 | L3 | ✅ | ARC Prize 帖第 2 条；Astra 结果页（`c-arc-gpt6-astra.txt` 127 行） | Astra 99.9% 为 Provider Adapter、high 档、$18,817；Argon 97.5% 为 Provider Adapter、low 档 |
| 已删：ARC Prize 称标准框架提供各家同条件的比较 | L3 | — | Astra 博客（`c-arc-astra-blog.txt` 243 行；`../images/29-arc-astra-two-harnesses.png`） | “Our Standard harness for ARC-AGI-3 asks how models compare under the same minimal, provider-neutral interface. … The shared interface also gives us a consistent, apples-to-apples comparison across providers.” 同段还有 “We believe a future AGI should be able to solve ARC-AGI-3 under these conditions.” |
| Astra 在标准框架下最好为62.7%（最高推理档） | L3 | ✅ | 同上 164、172 行；Astra 结果页 127 行 | max 档、$26,098 |
| 49.8%那次因接口故障，55道题只跑完42道，未跑完的按已得部分分计入；省流：标准框架未测完、得49.8% | L3 | ✅ | Argon 结果页 129 行 | “API issues prevented complete v3 evaluation with the standard harness. Runs completed on 42 of 55 tasks at low reasoning … Reported scores include all 55 tasks, retaining partial credit earned on incomplete runs.” |

## 省流卡

| 卡片事实 | 对应正文 | 结果 |
|---|---|---|
| 两部门拟把23项 AI 技术列入出口禁限目录；即日生效的是出口前登记 | 省流第 1 句（拼接两段，因版面省去“仍在征求意见”，“拟”保留） | ✅ 逐段一致 |
| Muse Spark Open 号称美国最强开放权重模型；截至16日，AA 指数领先它的9个开放权重模型都来自中国 | 省流第 2 句 | ✅ 逐字。“号称”指 Meta 官方自称（标签为 Meta）；“领先它的”=AA 综合指数高于它（初稿“比它强的”为第一轮 SHOULD_FIX 5）；“截至16日”为第二轮 NEEDS_VERIFICATION 7：复核时 AA 同类排名已变为 #9/118，本期结论按 10/16 19:39 存档 |
| 谷歌 Argon 在 ARC-AGI-3 低推理档用厂商上下文管理得97.5%，标准框架未测完、得49.8% | 省流第 3 句 | ✅ 逐字（第一轮 BLOCKER 2 标未测完；视觉复核 SHOULD_FIX 补“低推理档”） |

## 批注截图译注

| 译注 | 原句 | 来源 | 结果 |
|---|---|---|---|
| 41-1 目录正式实施后，限制类 AI 技术在海外本地化部署，属于技术出口管理范围 | 问“《目录》正式实施后……”；答“对于纳入限制类的人工智能技术，开展海外本地化部署，属于技术出口管理范围” | `a-a-qa.txt`；底图 `41-mofcom-api-raw.png`（A 组截图 03，2026-10-16） | ✅（第一轮 BLOCKER 1 后补“目录正式实施后”、改回“属于……管理范围”） |
| 41-2 目录正式实施后，用限制类大模型对外提供 API、App/网页交互、SaaS、端侧轻量化部署等，不属于大模型技术出口管理范围 | “对于企业利用限制类大模型对外提供API调用、APP/网页交互、SaaS应用、本地端侧轻量化部署等情形，不属于大模型技术出口管理范围” | 同上 | ✅ |
| 42-1 此次公开征求意见拟增列40项关键技术，涉及 AI、生物医药、光伏三个领域 | “此次公开征求意见涉及拟增列的人工智能、生物医药、光伏三个领域40项关键技术” | `a-a-qa.txt`；底图 A 组截图 04 | ✅ |
| 42-2 三个领域40项中，拟禁止出口14项 | “其中，拟禁止出口技术14项” | 同上 | ✅ |
| 43-1 迄今能力最强的美国开放权重模型（“美国”加强调） | “the most capable US open-weight model to date” | `b-meta-blog.txt`；底图 B 组截图 13 | ✅ |
| 44-1 ARC-AGI-3 上，标准框架得49.8%，厂商适配框架得97.5%，均为低推理档 | “On v3, it reaches 49.8% with the Standard harness at low reasoning and 97.5% with the Provider Adapter harness at low reasoning.” | `c-arc-gemini4-argon.txt` 129 行；底图 C 组截图 25 | ✅（OCR 认不准 “Adapter.”，高亮按 Tesseract 词框手量 rects，Claude 目视确认落点） |
| 44-2 接口问题导致标准框架没能测完；低推理档55道题跑完42道 | “API issues prevented complete v3 evaluation with the standard harness. Runs completed on 42 of 55 tasks at low reasoning” | 同上 | ✅ |
| 44-3 公布的分数按全部55道题计，未跑完的保留已得部分分 | “Reported scores include all 55 tasks, retaining partial credit earned on incomplete runs.” | 同上 | ✅ |
| 45-1 我们认为未来的 AGI 应能在上述标准框架条件下解出 ARC-AGI-3；统一接口也让各家之间可以同条件比较 | “We believe a future AGI should be able to solve ARC-AGI-3 under these conditions. The shared interface also gives us a consistent, apples-to-apples comparison across providers.” | `c-arc-astra-blog.txt` 243 行；底图 C 组截图 29 | ✅（“上述标准框架条件”补足原文 these conditions 的指代，见同段首句） |
| 45-2 用厂商适配框架，Astra 在 ARC-AGI-3 半私有集的最好成绩从62.7%升到99.9%（前者为最高推理档，后者为高推理档） | “With the Provider Adapter harness, Astra's best observed score on ARC-AGI-3 Semi-Private increased from 62.7% to 99.9%.” | `c-arc-astra-blog.txt` 247 行 | ✅（62.7% 为标准框架 max 档，99.9% 为适配框架 high 档，见 172、176 行） |

各批注图页脚存档日期 2026-10-16：A、B、C 组抓取日志（`capture-log-a.md`、`-b`、`-c`）记录的北京时间抓取日期。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Anthropic 补脚注点名 Mononoke，原作者称署名已足够 | L1（补脚注）＋L2（作者评价） | ✅ | https://www.anthropic.com/engineering/redesigning-source-control（`d-d1-anthropic.txt` 27、143 行）；https://sunshowers.io/posts/anthropic-mononoke-2/（`d-d1-sunshowers.txt` 23 行） | “Updated October 15 to include more detail on the projects that pioneered many of the techniques we use in Scion.” 脚注 2：“Specifically, the server component of Sapling, Mononoke. It pioneered many of the techniques we use in Scion, in particular the Bonsai data model.” 作者更新：“sufficient attribution has been added to Anthropic’s blog post. I stand by my posts as of the time I wrote them.” 【跟进 1014】：1014 写“截至15日傍晚未见 Anthropic 回应”，此后员工于北京 10/16 01:59 在 HN 回复（HN 50107559） |
| 据 IT之家报道，WorkBuddy 国内版另送500积分，须22日24点前领取 | L4 | ⚠️ | https://www.ithome.com/1/013/848.htm（`d-d2-ithome.txt` 84 行） | “面向国内版全量用户额外赠送 500 积分，均可在 10 月 22 日 24 点前进入‘用量管理’页面手动领取，积分有效期为一个月”；腾讯官方公告原文未找到。另：10/8–14 使用 DeepSeek 消耗的积分全额返还，7 个工作日内到账 |
| Claude API 中 Opus-5.5 快速模式结束研究预览，价格为标准2倍 | L1 | ✅ | https://platform.claude.com/docs/en/build-with-claude/fast-mode（`d-d3-fastmode.txt` 92、158 行）；定价页（`d-d3-pricing.txt`） | “Fast mode for Claude Opus 5.5 doesn't need the fast-mode-2026-02-01 beta header or an access request.”；ClaudeDevs 帖“no longer in research preview”；$8/$40 对标准 $4/$20。Opus 5、Opus 4.8 仍为研究预览。范围：ClaudeDevs 帖“in the Claude Platform”，另称 Claude Code 中的快速模式不变（`d-d3-tweet.json`）；价格为 API 每百万 token 价（第一轮 SHOULD_FIX 7 补“API”） |
| OpenAI 新公开4份6月内部模型越权报告，含一模型擅改外部机构内部系统配置 | L1 | ✅ | https://alignment.openai.com/misalignment-reports/（`d-d4-index.txt`）；仪表盘报告（`d-d4-dashboard.txt` 33 行） | 4 份 First posted 10/15/26，Incident date 6/16–6/26；“accessed the organization’s internal systems and changed their configuration”，该组织为“an international research organization”。均为内部研究模型 |
| 已删：据 RuntimeWire 报道，被指含 Claude Code 文本，未经核实 | L4 | — | https://runtimewire.com/article/tencent-workbuddy-claude-code-internal-text-allegation（`d-d5-runtimewire.txt` 7–18 行） | 速览标签为 WorkBuddy；“Tencent's WorkBuddy is facing an allegation that its international desktop app reused internal text from Anthropic's Claude Code”；“The reported matches have not been independently verified.” 正文“未经核实”对应 not independently verified（40 字上限内无法写“独立”）；指控源自中文博客报告（首发未找到，转载 `d-d5-0539edu.txt`），称 224 段相同；RuntimeWire 另称腾讯文档写明兼容 Claude 插件。未下载、未比对任何软件包或泄露源码（Develata 决定） |
| 西蒙斯基金会、Anthropic、OpenAI 承诺共投1500万美元资金与资源 | L1 | ✅ | https://www.simonsfoundation.org/news/simons-foundation-launches-15m-effort-to-help-mathematicians-navigate-ai-era/（`e-simons.txt`） | “the Simons Foundation, Anthropic and OpenAI will commit $15 million in combined funding and resources”；未拆分各方份额 |

## 反向核验

### 第一轮（codex gpt-6-astra medium，WSL，`barking review --reverse`）

4 BLOCKER、5 SHOULD_FIX、1 NEEDS_VERIFICATION，Claude 逐条回原文核对，全部采纳：

1. BLOCKER 商务部译注删去“目录正式实施后”并把“不属于……管理范围”写成“不算技术出口”——已改译注 41-1、41-2；正文②补“正式”。
2. BLOCKER ARC 省流只写“标准框架下只有49.8%”，未标未测完；“同条件”后接不同推理档的 Astra 62.7%——省流与省流卡改“标准框架未测完、得49.8%”；事实段注“均为低推理档”，吠点①补“且推理档不同”“（最高推理档）”，删去“ARC Prize 称标准框架提供各家同条件的比较”。
3. BLOCKER Mac 运行条件缺失——补“M5 Ultra”“4位量化”，速度保留“8千 token 上下文”。
4. BLOCKER WorkBuddy 条目的 L4 资格与“国际版”范围——RuntimeWire 自述主要来源为社交帖、首发博客未找到；Develata 决定撤下该条。
5. SHOULD_FIX “比它强的9个”超出指数排名——改“AA 综合指数领先它的9个”。
6. SHOULD_FIX 西蒙斯“共投”丢未来时——改“承诺共投”。
7. SHOULD_FIX 快速模式缺 API 范围——改“Claude API 中”。
8. SHOULD_FIX 公共领域排除——正文事实段补“已开源或发表论文进入公共领域的技术不纳入”。
9. SHOULD_FIX 卡片与译注缺事实行——补“省流卡”“批注截图译注”两节。
10. NEEDS_VERIFICATION `images/README.md`、`sources/README.md`、`offsite.tsv` 尚缺——已补前两份；`offsite.tsv` 按 publish.md 在“已发布”后由 `barking offsite` 生成。

### 第二轮（同上，撤下 WorkBuddy 前开跑）与视觉复核（codex gpt-6-astra medium，`barking review --visual`）

第二轮：1 BLOCKER、5 SHOULD_FIX、1 NEEDS_VERIFICATION。视觉复核：1 BLOCKER、4 SHOULD_FIX。处理：

1. BLOCKER（两份报告同一条）WorkBuddy 条目来源资格与“国际版”范围——Develata 决定撤下。
2. SHOULD_FIX Meta 模型卡数字为不同推理档、10/7 旧快照——正文删去该句。
3. SHOULD_FIX 45-2 比较条件——译注补“前者为最高推理档，后者为高推理档”；45-1 “这种条件”改“上述标准框架条件”。
4. SHOULD_FIX Scion 条目定级——改 L1＋L2，“致谢”改“署名”。
5. SHOULD_FIX 42 号图标题“拟限制26项”无事实行——补入主帖一第 5 行事实列；42-1 译注补“公开征求意见”“拟增列”。
6. SHOULD_FIX `evidence-c.md` 称“页面没有说明未完成的题如何计分”与所引原句矛盾——以原句为准：已说明全部55题计入并保留部分得分，逐题计分过程未核；本清单采用原句。
7. NEEDS_VERIFICATION AA 排名时点——正文与省流加“截至（北京时间）16日（晚）”。
8. SHOULD_FIX（视觉）省流卡第 3 条补“低推理档”——正文省流同步。
9. SHOULD_FIX（视觉）44、45 深色底图高亮后小字对比度下降——未改：`barking card` 的高亮为正片叠底，暂无针对深色底的选项；请 Develata 发布前在手机上看一眼，看不清可改用 26 号表格图（白字表格）或保留原截图作正式配图。

## 视觉复核

见上节第二轮与视觉复核记录；Claude 逐张目视 00-tldr、00-roundup、41–45：高亮落点、编号与译注对应正确，43 的下划线在 US 下，44 按手量 rects 落在三句上。

### 第三轮（未完成）

第二轮后改动（省流“号称”“截至16日”、省流卡“低推理档”、41/42/45 译注、Scion“署名”、删 Meta 模型卡数字）送第三轮反向核验，Codex 运行 20 分钟超时、未产出报告（`barking review` 退出码 124），这些改动**未经跨模型复核**；Develata 已于此后发布。改动均为收窄口径或补限定词，Claude 已逐条回原文核对。
