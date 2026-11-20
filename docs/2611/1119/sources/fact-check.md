# 1119 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B、C 组 Sonnet 5.5（`claude-sonnet-5-5`），D 组 Haiku 5.5（`claude-haiku-5-5`）；用时（harness）A 8 分钟、B 12 分钟、C 16 分钟、D 14 分钟。Claude 抽查：A 组 Fig. 11 与结果段、B 组博文与路透、C 组榜单与评分细则、D 组委员会附件与 424B4 的原句在存档中逐字命中。

## 一、Frostbite（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Architect Labs 称，AI 智能体从指令集做到版图，设计出 CPU 架构 Frostbite（标题：公司称AI设计CPU能效4.59倍） | L1 | ✅ | `a-blog.txt`：“Our AI systems autonomously iterated over the compiler, ISA, architecture, microarchitecture, RTL, verification, and final GDS.”；标题 “Introducing Frostbite: A new CPU architecture discovered, designed, and verified end-to-end by AI” | 公司自述；博客日期 November 18, 2026（L18），未给时刻；“电路版图”= final GDS；负载集、搜索/留出划分、对照口径写成“我们”决定，人工介入量未披露 |
| 它拿四个计算单元比面积相近的一颗顺序执行核 | L1 | ✅ | `a-blog.txt`：“We benchmark one Quad+L3 cluster against a single core of a modern superscalar (dual-issue, in-order) CPU, which we refer to as TCPU”；“This TCPU is 0.895 mm² at the same process.”；“benchmarking one Frostbite quad against the TCPU, since they have similar area” | quad = 2×2 tile + 4 个 L3 节点，0.792 mm²；对手为双发射顺序核。第 2 轮反向核验后由“四核簇”改为“四个计算单元”：原文是 tile，未称为核 |
| 25个自选小程序上每瓦性能平均4.59倍（标题：公司称……能效4.59倍；省流：AI 设计的 CPU 称每瓦性能是对手4.59倍） | L1 | ✅ | `a-blog.txt`：“On a set of 25 initially chosen workloads … we achieved a 4.59x performance-per-watt multiple”；“Frostbite reaches a geometric mean of 4.59x the TCPU's performance-per-watt” | 几何平均；“小程序”：FFT/radixsort/LU 按 SPLASH-2 缩小 64 倍（“64 times smaller”），Fig. 3 规模如 gemm 64×64×64；A 组用图内 25 个标注值复算 4.59、6.02、3.83 一致 |
| 对手没具名（省流；吠点①：对手只是没具名的顺序执行核） | L1 | ✅ | `a-blog.txt` Notes：“† For legal reasons, we cannot disclose the exact CPU.” | — |
| 对你：没有芯片可买，设计停在图纸，功耗是估算（标题：没造；省流：芯片没造） | L1 | ✅ | `a-blog.txt`：“500 MHz in the final GDS”；“These are, of course, still pre-silicon power numbers”；计划列表 “Deliver a software playground … hosted on FPGAs.”“Release the RTL and ISA for Frostbite.” | 全文未见流片、硅片或 FPGA 实测；未宣布量产或销售 |
| 吠点①：Notebookcheck 标题写“AMD 和 Intel 靠边站” | L5（夸大实例） | ✅ | `a-m-notebookcheck.txt` L61：“AMD and Intel, move over: AI designed an all-new CPU — and claims it is 4.6x more efficient” | 页面 Published 11/20/2026；其正文第一段写了 pre-silicon，标题没有；翻译为本号意译 |
| 吠点①：对手只是没具名的顺序执行核，跟乱序大核比要等“未来几个月” | L1 | ✅ | `a-blog.txt`：“In the coming months, we'll be benchmarking Frostbite mesh-like configurations against multi-core “TCPU” arrangements with large caches and larger, wider, out-of-order CPUs, over a larger set of industry benchmarks such as SPEC and PARSEC.” | “乱序大核”对应 larger, wider, out-of-order CPUs。第 1 轮反向核验 BLOCKER：原稿“博客没和两家比过”不成立——对手型号保密，可能就是 AMD 或 Intel 的产品；博客引言还提到 Intel Xeon、AMD Zen（L30），已改 |
| 吠点②：Architect Labs 自报每瓦性能是对手4.59倍，其中 AI 照着优化的10个程序6.02倍，没照着的15个3.83倍 | L1 | ✅ | `a-blog.txt`：“6.02x on the 10 search-set workloads, and 3.83x on the 15 held-out workloads it was never tuned on”；“A search set of workloads drove all architectural optimization, ISA design, RTL, and verification” | 留出 15 项里 5 项后加；最低 fib 1.15×，无一项低于 1×（Fig. 11） |

## 二、Signal（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 加密聊天应用 Signal 称，为防手机系统 AI 读聊天（省流、标题：iOS-27 禁截屏） | L1 | ✅ | `b-blog.txt`：“Meredith Whittaker on 20 Nov 2026”；“we are restricting the latest AI agents built into mobile operating systems from potentially capturing and storing your Signal data” | 博文只有日期；路透 17:02 首发 |
| 在 iOS-27 上禁止应用内截屏，Android 上检测到 AI 助手读屏才开“屏幕安全” | L1 | ✅ | `b-blog.txt`：“we are blocking such capture by preventing screenshots in the app on iOS 27 and when AI assistants attempt to capture screen data on Android”；脚注 2：“We automatically enable the “Screen security” setting in Signal Android when this type of access is detected.” | iOS 设置名 “Block Siri AI”，说明 “Prevent Siri AI from scanning Signal content. This will also block you from taking screenshots in the app and App Switcher.”（博文内嵌图） |
| 据路透，默认开启 | L4 | ⚠️ | `b-reuters-live.txt` L48：“the service is introducing new settings blocking screenshot access on people's phones and switching those restrictions on by default” | 博文本身未写“默认”；TNW 也写 iOS “on by default”（L4） |
| 对你：开关在 Signal“设置-隐私”，关掉可截屏（省流：保护能关） | L1 | ✅ | `b-blog.txt`：“People who want the ability to take screenshots can disable this protection within the Signal app’s settings.”；`b-gh-ios-commit-9256c9583.json` L15：GitHub 提交 “Add 'Block Siri AI' setting to Settings > Privacy”；支持页（Android）“Settings … > Privacy > Screen Security” | iOS 路径来自 Signal 官方代码提交与博文截图，支持页尚未更新；官方未给生效版本 |
| 对你：苹果称中国大陆没有 Siri AI | L1 | ✅ | `b-apple-restrict.txt` L3271：“Apple Intelligence and Siri AI are not available in China mainland.” | 苹果支持页（iOS 27 版） |
| 吠点①：The Verge 标题称 Signal“阻止 Gemini 和 Siri AI 读屏” | L4（夸大实例） | ✅ | `b-verge.txt`：标题 “Signal is blocking Gemini and Siri AI from reading your screen” | 20:30 发布；其正文写了可恢复截屏、“incomplete” |
| 吠点①：Signal 自称只拦截屏，读手机已存内容拦不住 | L1 | ✅ | `b-blog.txt`：“Our remedy is also incomplete – … restricting screenshots does not address other more sneaky ways AI agents could potentially access application data, such as retrieving content already present on a user’s device via the filesystem.” | 原文为 “could potentially”，未说已发生 |
| 吠点①：Mac 上目前也做不到 | L1 | ✅ | `b-blog.txt`：“This is also the reason we are currently unable to implement these protections on macOS.”；L57：“we are also not blocking Siri AI on macOS. This is because we cannot reliably block screenshots on macOS.” | 脚注另建议 macOS 用户在系统设置 Siri 的 Excluded Apps 里加入 Signal |
| （读者稿检查后删去的吠点②）Android 只防“部分 AI 助手” | L1 | ✅ | 博文内嵌 Android 设置图：“This will also prevent some AI assistants such as Google Gemini from scanning Signal content.”（`images/15-signal-settings-ios-android.png`） | — |
| （同上，已删）Signal 称覆盖不了所有设备 | L1 | ✅ | `b-blog.txt` L57：“in addition to not being able to cover all Android devices” | 脚注 2：多家厂商的系统上无法可靠检测 |
| 省流：也挡不全 | L1 | ✅ | 同上（文件系统、macOS、部分 Android 设备） | — |

## 三、Contra 设计榜（C 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 接单平台 Contra 的设计榜让设计师盲选同一需求的两份网站 | L3 | ✅ | `c-leaderboard.txt` L259：“Reviewers see two outputs for the same brief side by side and pick the better one, without seeing which model made which.”；“Ranked by Elo from Contra’s designer panel: 4,311 blind votes by 35 professional designers on 33 briefs.”；`c-board.txt` L35/L41：“real client briefs from Contra’s marketplace”；`c-blog.txt`：“When designers saw a Gestalt site next to a golden design” | 与 BenchFlow 合办；正文省略 BenchFlow |
| HN 论坛帖称“两个隐身模型都排在 Claude 的 Fable-5.2 前面”（省流：设计榜热帖称“两个隐身模型”胜 Claude） | L6（说法） | ✅ | `c-hn.txt` / `images/25-hn-title.png`：“Two stealth models just appeared on the leaderboard, both ahead of Fable 5.2” | 145 分 / 46 评（截图时）；提交者自拟标题 |
| 榜上隐身模型只有第一的 Gestalt，第二是 Contra 请设计师做的参考作品（省流：第二个是人类设计师） | L3 | ✅ | `c-board.txt`：“Gestalt (stealth) #1 · ELO 1931”；“Human designers RECRUITED FROM CONTRA #2 · ELO 1739”；`c-blog.txt`：“we worked with professional designers on Contra to create human golden outputs for the tasks” | 17 行中仅 Gestalt 标 stealth；Fable 5.2 第 3（1713） |
| 对你：榜单免费看 | L1 | ✅ | 榜单、方法、图库匿名可取（C 组存档）；`c-vote.txt` L41：“Community votes don’t affect the benchmark leaderboard.” | — |
| 对你：官方没说榜首 Gestalt 是谁家的、怎么用 | L1 | ✅ | `c-blog.txt`：“a high-quality design model currently in stealth”；“We are working with the team to continuously evaluate its quality” | 未写开发方、价格、接入方式；17173 称可在 OpenRouter 调用，官方未证实 |
| 吠点①：据 Contra，Gestalt 胜人类参考作品，是设计师28次二选一选它20次 | L3 | ✅ | `c-blog.txt` / `images/31-blog-gestalt-results.png`：“When designers saw a Gestalt site next to a golden design, they picked Gestalt 20 times out of 28.” | 官方矩阵 Gestalt 对人类 71%；Gestalt 共 420 票，为 17 行最少 |
| 吠点①：Contra 的评分细则给它83分，比 Fable-5.2 低1分 | L3 | ✅ | `c-rubric.txt` L76：“Claude Fable 5.284Claude Opus 5.584Gestalt83” | 细则页（Rubric）为 critique 分；博客写 Claude 三款 79.3–83.8、人类参考稿 73.6 |
| 吠点②：人类参考作品按 Contra 反馈改过 | L1 | ✅ | `c-blog.txt`：“We reviewed their moodboards and gave feedback. … After that, our team gave feedback and the designers iterated on the design.” | 模型侧：“BenchFlow runs an AI agent on each task in a sandbox … we ran each model in its own lab’s agent harness.” |
| 吠点②：AI 的提示词方法页没写 | L3 | ✅ | `c-methodology.txt`：模型侧只有 “BenchFlow runs an AI agent on each task in a sandbox … we ran each model in its own lab’s agent harness”，全页未写提示词（C 组 C2；第 2 轮复核者在线复查方法页一致） | “没公开”的范围：方法页、博客、数据集页 |

## 速览

（主帖条目逐字取自小标题；其余条目见下表）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 美众院：截至5月11日，对华 H200、MI325X 获批约88万颗，均未出货 | L1 | ✅ | `d-scribd-1-disclosure.txt` L74：“Covered integrated circuits are the following products: NVIDIA’s H200 and AMD’s MI325X.”；“as of that date, BIS received applications to sell 3,444,160 covered integrated circuits to PRC companies”；“BIS approved … 879,868 covered integrated circuits to PRC companies across 21 license applications, as of May 11, 2026.”；`d-scribd-2-disclosure.txt`：“H200 License and Equivalent Application Status as of May 11, 2026”，21 行均为 “Approved, not shipped” | 众院外委会 11-19 公布（通稿北京 11-20 03:18）；芯片为 NVIDIA H200、AMD MI325X；“约88万颗”= 879,868；“均未出货”指 21 份获批许可，为 5 月 11 日附表口径；Nvidia 11-19 称已停止向中国发 H200（CNBC 转引）。风控：标题、封面不碰本条 |
| Anthropic 招股书：创始人捐出个人股权80%的承诺自愿、可撤销 | L1 | ✅ | `d-edgar-424b4-excerpt.txt` L15：“our co-founders have each pledged to dedicate 80% of their personal equity in Anthropic to philanthropic causes. These commitments are voluntary, revocable expressions of each co-founder’s philanthropic intent and do not constitute legally binding obligations.” | 424B4 申报日 11-19；1018 期只收过“承诺捐出80%”。TechCrunch 称七位创始人售股约 19.5 亿美元（未上卡） |
| Anthropic：Claude 解的黑洞一题，证明一半沿用 OpenAI 思路 | L1 | ✅ | `d-anthropic-blackhole.txt` L89：“One half of the final proof, showing that the ceiling can’t be lowered, follows the strategy of the OpenAI proof.” | 跟进1109；指模块化自举（最轻黑洞上限）一题；研究页 11-17；HN 50563148 518 分 |
| 美法官：若如原告所称仅凭 AI 人脸比对申请逮捕，警探暂不获豁免 | L1 | ✅ | `d-osorio-ruling-pdf.txt` L49：“more than an uncorroborated conclusion by AI that the individual in question has…”；L501：“the only tool was uncorroborated AI facial recognition”；L396：“defeat qualified immunity at this stage of the action” | 东肯塔基联邦地区法院 11-09 驳回被告动议；该阶段假定原告主张属实（“若如原告所称”），未认定责任；“豁免”指合格豁免；路透 11-10 报道 |
| 英国 DeepMind 员工工会称谷歌拒绝自愿承认，已申请法定承认 | L2 | ✅ | `d-gdmunion.txt` L161：“UTAW and Unite, voluntarily. Google declined and offered talks through ACAS”；L173：“We didn't reach an agreement, so we applied for statutory recognition through the Central Arbitration Committee (CAC).” | 工会自述；站上无人数；HN 50557885 123 分 |

## 批注译注

| 卡片 | 原文 | 译注要点 | 备注 |
|---|---|---|---|
| 05-frostbite-tcpu | 结果段三句 + Notes 脚注 | 四核簇对单核、双发射顺序执行；6.02/3.83；法律原因不透露 | “含 L3 缓存”对应 Quad+L3 |
| 06-frostbite-fig11 | Fig. 11 图例 | 留出 3.83、全部 4.59、调优 6.02；TCPU 记 1 倍 | 像素框标注 |
| 07-frostbite-next | 后续计划段 | 未来几个月对比多核与乱序 CPU、SPEC/PARSEC | — |
| 08-notebookcheck-title | 标题 | 意译“靠边站” | 像素框标注 |
| 11-signal-scope | 范围句 | iOS 27；Android 限于能检测到的版本 | 像素框标注 |
| 12-signal-incomplete | 可关闭句 + 不完整句 | 可在设置里关；文件系统途径挡不住 | 第 2 条像素框 |
| 13-verge-title | 标题 | — | 深色页面原样 |
| 16-hn-two-stealth | HN 标题 | — | — |
| 17-contra-board | 榜单前三行 | 第 2 名人类设计师 1739；第 3 名 Fable 5.2 1713 | 像素框标注 |
| 18-contra-28 | 博客 20/28 句 | — | — |

## 反向核验

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-20 约 22:30–22:39；审的是读者稿检查前的正文与卡片，配图说明尚未写；审查中正文被改，报告已注明）

1. BLOCKER 吠点①“博客没和这两家比过”——采纳。对手型号保密，可能就是 AMD 或 Intel 的产品；博客引言提到 Intel Xeon、AMD Zen（`a-blog.txt` L30）。改为“对手只是没具名的顺序执行核，跟乱序大核比，博客称‘未来几个月’才做”。
2. BLOCKER 对你“Gestalt……也用不了”——采纳（该句是读者稿检查后新改的）。改为“官方没说榜首 Gestalt 是谁家的、怎么用”。
3. BLOCKER 吠点② 单独上卡时丢了主张者与指标——采纳。改为“公司自报的4.59倍里，设计时 AI 照着优化的10个程序是6.02倍，没照着的15个是3.83倍”；“每瓦性能”在事件卡与省流里，吠点卡已有“4.59倍”锚点。
4. BLOCKER 标题相对数没有署名——采纳。标题改为“AI设计CPU称能效4.59倍，没造出来”（“称”标出主张，主体是事实段里的 Architect Labs），封面 A 组按新标题重出。
5. SHOULD_FIX Signal 省流、速览丢 iOS 27——采纳：省流、小标题改为 iOS-27。
6. SHOULD_FIX Gemini 速览条件不全——采纳：改为“逐步推出……限美国、英文、苹果芯片与付费档”；18 岁以上个人账号、macOS 15+ 放清单备注（40 字放不下）。
7. SHOULD_FIX 芯片种类与截止日——采纳：改为“截至5月11日，对华 H200、MI325X 获批约88万颗，均未出货”，删去申请数。
8. SHOULD_FIX 法院程序阶段——采纳：改为“若如原告所称仅凭 AI 人脸比对申请逮捕”。
9. SHOULD_FIX “没公开”范围过宽——采纳：Contra 吠点②改为“AI 的提示词方法页没写”。
10. SHOULD_FIX “默认开启”L4 标 ✅——采纳，改 ⚠️，正文保留“据路透”。媒体与论坛标题行属“说法存在已核实”，不代表所述事件为真，已在级别栏写 L4/L5/L6。
11. SHOULD_FIX 证据定位——采纳：盲选原句改指 `c-leaderboard.txt` L259，投票限定指 `c-vote.txt` L41。
12. SHOULD_FIX 配图说明缺失、正文与卡片不同步——已补 `images/README.md`，卡片按定稿重渲。
13. NEEDS_VERIFICATION 对华芯片条目风控——保留，交付时提示 Develata。

### 第 2 轮（Codex `gpt-6-astra` medium，北京 11-20 约 22:48–23:00；审的是第 1 轮修改后、视觉复核修改前的稿）

1. BLOCKER 标题“AI设计CPU称能效4.59倍”仍没点名主张者——交 Develata 定，选“公司称AI设计CPU能效4.59倍，没造”，封面按此重出（A3）。
2. BLOCKER 吠点② 单独看缺公司名、指标、基线——采纳：改为“Architect Labs 自报每瓦性能是对手4.59倍，其中 AI 照着优化的10个程序6.02倍，没照着的15个3.83倍”。
3. SHOULD_FIX “四核簇”把 tile 说成核——采纳：改为“四个计算单元”，批注译注同步。
4. SHOULD_FIX Gemini 速览缺 macOS 15+、18 岁以上个人账号等条件——采纳：40 字放不下，删去该条（速览 8 条，含主帖 3 条）。
5. SHOULD_FIX Notebookcheck 译注“高4.6倍”——视觉复核时已改为“为对手的 4.6 倍”。
6. SHOULD_FIX Contra 评测数据应为 L3——采纳：相关行改 L3；吠点① 加“据 Contra”。
7. NICE_TO_HAVE 证据定位——采纳：marketplace 原句改指 `c-board.txt`，iOS 路径补 JSON 存档，提示词一行改指方法页。
8. NEEDS_VERIFICATION 对华芯片条风控——保留，交付时提示 Develata。

## 视觉复核

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-20 约 22:50；20 张图：两张封面、全部卡片与 10 张批注底图）

1. BLOCKER 第三条主帖的“热帖称两个隐身模型、实际只有一个”上了省流卡、速览与事件卡，被认为是纠偏、只能上吠点卡——不采纳。EDITORIAL 规定的是“吠点在图上只出现一次”；这条主帖的事件本身就是热帖说法与榜单原文的出入（“说法 + 实际”，与 1118 期省流卡“AA 的比价按 Carbon 原价”同类），吠点卡上的两条（28 次对比、评分细则、参考稿修改与提示词）没有重复它。交 Develata 终审时提示。
2. SHOULD_FIX 事件卡“平均4.59倍”未写几何平均——不采纳：正文字数已满，“平均”对普通读者不改变含义，几何平均写在清单与批注图 06。
3. SHOULD_FIX 吠点卡②缺指标名——采纳：改为“公司自报的每瓦性能4.59倍里”。
4. SHOULD_FIX Notebookcheck 译注“能效高4.6倍”易读成多 460%——采纳：改为“称能效为对手的 4.6 倍”。
5. SHOULD_FIX “Mac 上也做不到”缺“目前”——采纳：改为“Mac 上目前也做不到”。
6. SHOULD_FIX 图 11 原图字小——不采纳：图是完整原图，关键的三条几何平均已在译注；单独放大会多一张图。
7. NICE_TO_HAVE 吠点卡 Fable–5.2 在连接号后断行——渲染器自动换行，不改。
