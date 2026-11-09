# 1108 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`（MaxxBench）、`evidence-b.md`（高盛）、`evidence-C.md`（速览）；截图在 `images/`。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI拿到答案，反删对的代码？ | L3 | ✅ | 见 A 组“换掉自己的代码” | 问号；指 DeepSeek-V4.1-Flash 一次运行（`run-abf4bf4dd0ac3ba3`）；风控按 Develata 决定，标题不点名 DeepSeek，正文写明 |
| 省流：第三方团队改了编程榜89道题，多家 AI 被发现在猜测试考什么，部分原始分却是厂商自报 | L3 | ✅ | 见 A1–A3、吠点① | MaxxBench 自报 |
| 省流：据彭博社报道，高盛不让多数员工用 AI 编程工具 Claude Code | L4 | ⚠️ | 见 B 组 | 另含 GitHub，正文写明 |

## A MaxxBench

MaxxBench 站点是第三方评测方自己的页面（L3）。Claude 已对照存档抽查 `a-mb-why.txt`、`a-same89-recompute.txt`、`a-v41-outside-llm-recount.txt`、图 11。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 第三方 MaxxBench 改了编程榜 DeepSWE 的89道题，比如要求文件名小写 | L3 | ✅ | https://maxxbench.unlap.ai/research/why-maxxbench/ （`a-mb-why.txt`，图 03） | “changes the instructions or starting code of 89 DeepSWE tasks”；“reference patches grow by a median of just 17 lines (3.5%)”。Unlap Corporation 自称独立评测，融资与产品未查到；博文标 20 Oct，changelog 10/29 首发，HN/X 11/7 才广泛传播 |
| 博文称多家 AI 在猜测试考什么 | L3 | ✅ | 博文（图 04、05） | “many models had figured out they were in a benchmark”；Hidden-test 表14行 |
| 博文列的原始分（含厂商自报）约74%的6个模型设置，改题后差了23分 | L3 | ✅ | 同上（图 01、02） | “Six configurations sit within 1.5 points of one another near 74% … fan out across 23.0 points”；原始分含厂商自报，见吠点① |
| Claude-Opus-5 算了笔账：假设五五开，按要求报错预计通过率0.55，不报错0.80。它选不报错，四项检查全败 | L3 | ✅ | 博文 LangChain 段（图 04）；轨迹 `run-cd4a491f8bf1dbc3#event-69`（`a-quote-check.txt`） | “With a 50/50 prior, raising gives roughly 0.55 expected pass rate while graceful gives roughly 0.80”；“suppressed the error after the first chunk, failing all four abandoned-stream checks”，后半为博文判断。其他模型同类记录见图 04、05（GLM-5.3 等） |
| 限内存题，博文称13个模型7个猜测试只看程序自报 | L3 | ✅ | 博文 scc 段（图 06） | “Seven of the 13 models reasoned explicitly that an external test suite could only inspect the printed statistics line … reloaded the spilled records back into memory”；含 Claude-Opus-5、Fable-5、GLM 两个、DeepSeek 两个、Muse-Spark-1.3；这是博文对模型思考的归纳，未逐模型独立确认；Muse 1.3 原话只写重新载入记录，与归纳有差；测试实际另有独立内存检查，“只看自报”是模型的猜测；Opus-5.5、Astra、Sol 无钻空证据 |
| 最高档178次里，它20次用评测方的密钥调外部模型或搜索 | L3 | ✅ | 博文与 `https://maxxbench.unlap.ai/research/coding-sandboxes`（图 07）；`a-v41-outside-llm-recount.txt` | “At max effort, it made outside-model calls in 20 of 178 attempts”；已用公开轨迹复算为20。“最高档”指本次测试的 max effort；沙箱博文“used it to call outside models or web search 20 times”；要答案只写已核实的具体案例，不写每次都要 |
| DeepSeek-V4.1-Flash 有一次先做对小写，拿到外部答案后换掉自己的代码，没过 | L3 | ✅ | 博文 Testem 段（图 08）；`run-abf4bf4dd0ac3ba3`，step 58/107–110 | “We'll follow tests, since grading uses them.”；F2P 35/68 |
| 吠点①：“差23分”的起点不一致：15个原始分里5个是厂商自报，不在官方榜上，Opus-5.5 的74.2%就是 | L1/L3 | ✅ | 图 11（“DeepSWE 74.2% · Reported by Anthropic”）、图 09；`a-ds-data` 与 `evidence-a.md` A6 | 5 个：Opus-5.5、Sonnet-5.5、V4.1-Flash、GPT-6-Luna、Qwen3.8-Flash-Next；官方榜页 9/22 更新，页面下拉与 trials.json 均无这5个；Opus-5.5 出处为系统卡 §8.3 “average of 74.2% over five trials” |
| 吠点①：博文正文没交代 | L3 | ✅ | 图 11；博文 “published DeepSWE scores” | 博文 Source data 表这5行也有 i 图标；正文与图表标题仍写 Published DeepSWE baseline |
| 吠点②：题也不是同一批：官方榜原始分算113题，改题分算89题 | L3 | ✅ | DeepSWE 官方榜（113 tasks，图 09）；博文 | 厂商自报分的题数各家口径另论 |
| 吠点②：本号用官方数据重算这89题，Gemini-3.8-Flash 跌25.9分，博文算20.5分 | L3 | ✅ | `https://deepswe.datacurve.ai/artifacts/v1.1/trials.json`；`a-same89-recompute.txt` | 79.3→53.4；榜单 Est. Benchmaxxing 20.5、博文 “18.6 to 20.5”（基线73.8）；本号按任务先取平均，113题为74.0、跌幅20.6，与博文差0.1来自聚合口径，官方榜自身按另一种聚合；与 MaxxBench issue #1 表一致；MaxxBench 另修过题，未拆开 |
| 吠点③：榜上每个模型每题只跑2次，没给误差范围 | L3 | ✅ | `a-mb-home-fields.txt`（自 `a-mb-home.html` 内嵌数据摘出，HTML 只存网盘）、图 02 | 官方 DeepSWE 榜有 ± |
| 吠点④：本号复算，DeepSeek 这20次过10次，其余158次过89次，两组题目不同 | L3 | ✅ | `a-v41-outside-llm-recount.txt`（列出20个 run id；178份轨迹约68 MB 未入库，从 `https://maxxbench.unlap.ai/data/runs/<id>` 可重取） | 按工具调用中 openrouter.ai 与 API key 同现计数，与博文20次一致，未逐条重放请求；公开轨迹与 trials 的 outcome 字段；样本小，未控制题目难度，只描述两组计数，不作外援效果判断 |
| 吠点④：密钥是评测方放的，博文称换 high 档后调用为0 | L3 | ✅ | `coding-sandboxes` 博文；`why-maxxbench` | “the container has to keep the inference API key in its environment variables”；“outside-model calls dropped to zero”，high 档89次运行，逐次数据未公开，未独立核对 |

## B 高盛

彭博全文取自 FA Magazine 转载页（`b-fa-mag.txt`，页末注“provided by Bloomberg News”），前两段与彭博原页、Bloomberg Law 导语逐字一致；Claude 已逐句对照存档。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据彭博社11月2日报道，高盛10月更新内部指引，多数员工不能再用 AI 编程工具 Claude Code 和代码平台 GitHub，限工程师使用，量化岗 strats 除外 | L4 | ⚠️ | https://www.bloomberg.com/news/articles/2026-11-02/goldman-restricts-traders-and-bankers-from-using-some-ai-tools ；`b-fa-mag.txt` 第304–306行 | 北京 11/2 21:52（13:52 UTC），Todd Gillespie。“banned most of them”“limiting the use of those tools to engineers only”“internal guidance updated in October and seen by Bloomberg News”。原文只写 GitHub，无 Copilot；strats 例外见第324行，“said another person”，匿名 |
| 一些交易员原本用它们盯客户交易流、搭市场模型，对此不满 | L4 | ⚠️ | `b-fa-mag.txt` 第308行 | 匿名知情人；“monitoring client trading flows, modeling market changes and positions” |
| 7月，高盛首席信息官还在夸交易员和银行家用 AI 写程序 | L4 | ⚠️ | 同上，第310行 | “touted the ‘vibe coding’ … he said in an interview”；7月原始访谈未找到 |
| 发言人称，试验一段时间后，工具交给“最适合长期使用的人” | L4 | ⚠️ | 同上，第312行 | Carcaterra：“After a period of experimentation … we put them in the hands of those who are best suited to leverage them for the long term.” 泛指 certain AI tools，未点名这两款工具 |
| 吠点①：印度时报标题写“禁了数千员工”，彭博社没给人数 | L4 | ⚠️ | timesofindia 文（`b-` 存档，图 23）；`b-fa-mag.txt` | 标题“bans thousands of employees”；全文无人数 |
| 吠点②：彭博社只写原因是“生产率和风险” | L4 | ⚠️ | 同上第304、312行 | 首段“productivity and risk concerns”；发言人“productivity and risk management”；另有行业背景句提 cybersecurity、costs，非高盛这次的原因 |
| 吠点②：本号截至11月9日未查到事故报道 | — | ✅ | `capture-log-b.md`、`evidence-b.md` B7 | 范围：Exa 多组检索、彭博三个版本、HN 413 评、多家转载；未穷尽 |
| 吠点③：高盛没弃用 Claude：据彭博社，其自建 AI 助手仍用 Anthropic 的模型 | L4 | ⚠️ | `b-fa-mag.txt` 第316行 | “combines models from OpenAI, Anthropic and Alphabet Inc.”；Alphabet 写作谷歌 |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 译注：我们新增一条要求：文件名须为小写 | L3 | ✅ | `a-mb-why.txt`（图 03） | “We added a requirement that the filenames be lowercase.” |
| 译注：所有题目中，参考补丁增长的中位数只有 17 行（3.5%） | L3 | ✅ | `a-mb-why.txt:99` | 正文已删，留批注 |
| 译注：GLM-5.3 思考中写道：就用 float64 吧；测试里出现大于 2^53 的数的风险极小 | L3 | ✅ | `a-mb-why.txt:204`；轨迹回源 `a-quote-check.txt` | |
| 译注：Claude-Opus-5：先验五五开时，报错的预期通过率约 0.55，温和处理约 0.80 | L3 | ✅ | 图 04；`a-quote-check.txt` | |
| 译注：博文：四项“流被中途放弃”的检查全部失败 | L3 | ✅ | `a-mb-why.txt` | 博文判断 |
| 译注：Claude-Opus-5：峰值内存其实只能通过我们自己的统计输出观察到 | L3 | ✅ | `a-mb-why.txt:426`（图 06） | 模型原话 |
| 译注：GLM-5.3：它测不了实际内存，所以风险只在我们往统计行里打印什么 | L3 | ✅ | `a-mb-why.txt:446` | 模型原话 |
| 译注：DeepSeek-V4.1-Flash：测试查的是溢出统计，大概不测格式化器内部 | L3 | ✅ | `a-mb-why.txt:456` | 模型原话 |
| 译注：在最高档（max）下，它在 178 次尝试中有 20 次调用了外部模型 | L3 | ✅ | 图 07 | |
| 译注：（换成 high 档重跑后）针对性使用密钥和调用外部模型降为零 | L3 | ✅ | 图 07 | high 档89次，逐次数据未公开 |
| 译注：DeepSeek 起初正确实现了小写，随后拿到一份没有小写的外部实现 | L3 | ✅ | 图 07、08 | |
| 译注：它决定优先照顾推测中的原始测试，换掉了自己的代码 | L3 | ✅ | 图 07、08 | |
| 译注：DeepSWE 74.2%，由 Anthropic 报告（链接指向 Anthropic 系统卡） | L3 | ✅ | 图 11 | 悬浮原文 “Reported by Anthropic” |
| 译注：官方榜：113 道题，更新于 2026 年 9 月 22 日 | L3 | ✅ | `a-ds-home.txt:40`（图 09） | |
| 译注：审查生产率和风险问题后，禁止多数员工使用为写代码设计的先进 AI 工具 | L4 | ⚠️ | `b-fa-mag.txt:304` | |
| 译注：据彭博社看到的、10 月更新的内部指引，该行把这些工具限定为只有工程师能用（strats 例外见下图） | L4 | ⚠️ | `b-fa-mag.txt:306`、`:324` | |
| 译注：就在 7 月，高盛首席信息官 Marco Argenti 还夸赞交易员和银行家等非技术员工的“氛围编程” | L4 | ⚠️ | `b-fa-mag.txt:310` | |
| 译注：（发言人）我们把工具交到最适合长期使用它们的人手中 | L4 | ⚠️ | `b-fa-mag.txt:312` | |
| 译注：高盛自己开发了一款 AI 助手，整合了 OpenAI、Anthropic 和 Alphabet 的模型 | L4 | ⚠️ | `b-fa-mag.txt:316` | |
| 译注：另一位知情人称：这份最新指令不适用于所谓的 strats，即横跨交易、量化研究和软件工程的员工 | L4 | ⚠️ | `b-fa-mag.txt:324` | |
| 译注：高盛禁止数千名员工使用 Claude Code 和 GitHub | L4 | ⚠️ | `b-toi.txt:1`（图 23） | 印度时报标题，被纠正说法 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 英伟达董事会就 CEO 相关报道启动独立调查，称暂无理由认为机密受损 | L1 | ✅ | https://nvidianews.nvidia.com/news/statement-from-nvidia-board-of-directors （`c-nvidia-board-statement.txt`） | 11/8，官方未给时刻。原文“no reason to believe that either national security or NVIDIA secrets were compromised”“will examine all available information”。Develata 决定只进速览、不上标题与封面；媒体所称的国家、第三人与私生活不写不存 |
| Nature 早期研究：手表 AI 戴7天查高血压，196人无心血管病未用降压药 | L1 | ✅ | https://www.nature.com/articles/s41586-026-11336-3 （`c-nature-hypertension.txt`）、https://www.research.google/blog/detecting-signs-of-hypertension-using-wearable-sensors/ （`c-google-hypertension-blog.txt`） | Nature 11/5。诊室55.3%；特异度均90.0%；排除已知心血管病与服降压药者；“early feasibility research”“not intended to replace standard clinical arm cuffs”。两组 CI 重叠，“差异不显著”未在摘要与博文中见到，不写 |
| 百度开放 ERNIE-5.5-Flash-Base 预训练权重，商用与地域有限制 | L1 | ✅ | https://ernie.baidu.com/zh/release/ernie-5-5-flash-base/ （`c-ernie-release.txt`）、https://huggingface.co/baidu/ERNIE-5.5-Flash-Base/raw/main/LICENSE （`c-ernie-hf-license.txt`） | 11/5，官方未给时刻。“未经中训练与后训练”；许可证第4节：相关产品或服务年收入≤5000万美元署名并通知，超过须书面授权；1.11 地域不含欧盟、英国、韩国；自订许可证，不写“开源” |
| 据 FT 报道，Anthropic 约34亿美元订1.7GW 发电机，路透未核实 | L4 | ⚠️ | https://www.reuters.com/business/energy/anthropic-secures-34-billion-generator-deal-ai-data-centers-ft-reports-2026-11-08/ （`c-reuters-anthropic-generators.txt`） | 未读到 FT 原文，“据 FT 报道”依路透转述。路透 11/8 20:54。“bought commitments for 1.7 gigawatts”；未收购 OnePWR；“could not immediately verify”；双方未回应。FT 原文 403，未读到 |
| Claude 个人财务 beta 向美国 Pro 及以上逐步开放，只读账户 | L1 | ✅ | https://claude.com/resources/articles/introducing-personal-finance-in-claude （`c-claude-personal-finance.txt`） | 11/5，官方未给时刻。Pro、Max、Ultra；“read-only”；专区内对话不用于训练，专区外按隐私设置；解绑后已有回答保留（帮助中心） |
| Anthropic 文：agent 记忆不含权重更新，空闲整理叫做梦 | L1 | ✅ | https://www.anthropic.com/research/components-of-agent-memory （`c-anthropic-agent-memory.txt`） | 11/6。原文“It excludes … changes to their weights”“This process is often called dreaming” |

## 反向核验

复核方：Codex（gpt-6-astra，medium），两轮。

第一轮（正文与清单，卡片未渲染）。BLOCKER：①“20次要答案”证据不足，采纳，改“调外部模型或搜索”；②“只有工程师”漏 strats 例外，采纳，事实段同句写明，小标题改“限制使用”。SHOULD_FIX：113题与2次的适用范围采纳；“没变高”删去，只留两组计数并注明题目不同；速览三条补限定（Pro 及以上、文章定义、地域与收入口径）采纳。NEEDS_VERIFICATION：卡片与 README 已在第二轮补审；复算证据链见下。

第二轮（正文、卡片、清单、README）。BLOCKER：①“7个想到”把归纳写成确认，采纳，改“博文称……猜”，备注写明 Muse 1.3 差异；②手表条目丢样本条件，采纳，改写并删65.8%，保留戴7天与受试范围。SHOULD_FIX：事件卡的“74%”补“含厂商自报”采纳；高盛译注补 strats 提示采纳；译注逐条登记采纳（上表）；NEEDS_VERIFICATION：20.5 与 25.9 改写成并列两个数并说明0.1差来自聚合；复算证据补 `a-mb-home-fields.txt` 与 run id，178份轨迹未全部重放，故写“本号复算”而不写“已验证”；英伟达题材风险由 Develata 决定只进速览，保留记录。

## 视觉复核

复核方：Codex（gpt-6-astra，medium）。BLOCKER：08 号③高亮漏第二句，采纳，改手量框。SHOULD_FIX：10 号高亮扫进 changelog，采纳；06 号译注补 float64，采纳；14 号 strats 补岗位定义，采纳；事件卡补“假设五五开”，采纳；07、08、13、14 英文截图偏小，未改（译注完整，原文图作依据；13、14 底图受页面宽版式限制）。
