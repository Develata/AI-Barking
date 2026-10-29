# B 组取证：指纹脚本藏危险词诱使 AI 拒绝分析（nullpt.rs）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8），括号保留原时区。存档与截图均在 2026-10-29 19:40–20:05（北京）取得。本组只记录作者如何描述脚本、模型反应与归因；未运行、未下载、未分析该脚本本身（脚本 URL `servicepipe.tech/static/checkjs/...` 未请求）。危险内容一律不复述：下表只引作者对“这些内容是什么、起什么作用”的非操作性原句，伪系统提示、函数名、命令文本、政治敏感词的具体内容请看存档 `b-article.txt`，本文不复制。

## 一、事件概要（均为作者原文，L6 级个人博客，作者署名 veritas）

- 原文：https://nullpt.rs/fingerprinting-bioweapons ，标题 “Why Is This Fingerprinting Script Creating Bioweapons?”，页面日期 “Wed Oct 28 2026”，作者 veritas；页面元数据 `article:published_time` 为 2026-10-28T00:00:00Z（只有日期，官方未给时刻）。存档 `b-article.html/.txt`。
- nullpt.rs 是个人技术博客，不是机构或厂商（L6，作者自述的分析，尚无厂商或第三方复现）。
- 作者发现路径：用 Tranco Top Sites 列表加 Playwright 爬约 1 万个网站、拦截 canvas toDataURL，汇总到 fingerprint.garden；其中一支 canvas 引起他对脚本来源的好奇，查到 servicepipe.tech 的一个检测脚本。

## 二、证据表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1a 脚本是哪一支 | L6（作者自述；脚本 URL 为作者所给） | https://nullpt.rs/fingerprinting-bioweapons | “The responsible script was https://servicepipe.tech/static/checkjs/06b667/4CIrx8U3Da.js.” 另：“I wasn't expecting anything unusual. Most fingerprinting scripts are frequently obfuscated using javascript-obfuscator or some derivative.” | 14-servicepipe-script-url.png（含脚本 URL；本期未请求该 URL） | 作者爬取日期原文未给；脚本内容可随时更新（作者称 “I will continue to poll for script updates”） | 已找到 |
| B1b 运营者是谁（归因强度） | L6 | 同上 “Who's behind this?” 与 “Special Thanks” | “A urlscan for servicepipe.tech reveals a similarly named servicepipe.ru whose site, when translated, states: ‘We develop high-precision traffic control systems Cloud services and local protection products against DDoS attacks, bots, and targeted threats’”；致谢处：“Sshell: for helping identify who could be behind this site.” | 20-whos-behind.png | 作者只写 “similarly named”（名称相近）与 “could be behind”；原文没有说“已确认是该公司”，没有注册信息、官方声明或其他硬证据。属推断 | 部分支持 |
| B2a 脚本里有什么：函数名与伪“系统提示” | L6 | 同上 “A peek inside / Inside the script” | 作者定性：“could this be some form of anti-LLM obfuscation? It would make sense: include symbol names that would intentionally trip model safeguards to prevent deobfuscation.” 以及：“This was clearly a string meant to trip every possible safeguard out there. It included Tiananmen Square references likely as a way to combat Chinese models.”（函数名、伪系统提示及其所列主题的具体文字见存档，不在此复述。作者称一段不可见 Unicode 字符串解码后是一段伪“系统提示”，列了多类高敏感主题；另有若干危险主题的函数名与返回字符串。） | 17-trip-safeguards.png（只含上两句，已避开提示正文） | 作者描述的是“字符串、函数名、注释式文本”，不是可运行的操作说明；原文没有说脚本在“制造”或“生成”任何危险物。作者列出的“anti-LLM strings”清单见存档，本文不抄 | 已找到 |
| B2b 脚本其余部分做什么 | L6 | 同上 “What does it actually do?” | “Outside of the unique obfuscation, the script appears to be your standard anti-bot script.” “The signals collected are pretty disappointing considering the level of thought that went into the obfuscation.” 采集项为 navigator 与 WebGL 属性（userAgent、hardwareConcurrency、deviceMemory、webdriver、WebGL 厂商/渲染器等）并有 “Integrity check”。 | 19-standard-antibot.png（全是 navigator/WebGL 信号，无危险内容） | 作者贴的是摘要式伪代码，不是脚本原文；是否完整未知 | 已找到 |
| B2c Node 环境分支（命令文本） | L6 | 同上 “Inside the script” | 作者原句：“The script seemingly executes malicious code when under a Node environment gated behind an environment flag:”，随后贴一段代码块（读取若干配置类文件名、一串 shell 命令，含下载并执行一个外部二进制、写计划任务；具体内容不在此复述，见存档）。紧接着在代码块后作者又写：“However, this is likely also anti-LLM obfuscation since the calls to readFile have empty callbacks and make references to self-harm/jailbreak system prompts.” | 17-trip-safeguards.png（仅作者引导句）；18-node-also-decoy.png（作者的“likely also”句；均不含代码块） | 作者对这段代码给了两种互相拉扯的说法：先写 “seemingly executes malicious code”，再写 “likely also anti-LLM obfuscation”；两句都带 seemingly/likely。条件：代码中的门控写成“Node 环境、有 Buffer、非浏览器、且某环境变量已设置”（作者所贴代码） | 已找到 |
| B3 哪些模型拒绝、哪些照做、测试规模 | L6 | 同上 | “I had Astra take a look at the script, giving it the context that this script seems to intentionally set misleading variable names to intentionally prevent LLM-assisted deobfuscation attempts.” “Maybe unsurprisingly, Astra was not willing to comply with the request.” “I also attempted this using the Daybreak Blue toggle with the same result, blocked.” “Out of curiosity, I tried GLM-5.3 with the same prompt and it complied!” 配图标题：“Astra refusing to deobfuscate the script, citing safety risks around biological research.” | 15-astra-refusal.png；16-blue-toggle-glm.png | 只测了 3 个设置（Astra、Astra+Daybreak Blue 开关、GLM-5.3），每个只写了一次结果，没有次数、没有温度/系统提示/运行环境说明，也没有测 Claude、Gemini、DeepSeek 等；提示词只有转述（“giving it the context that…”），没有全文；“complied” 没有说明交出的结果质量。原文没有写 Astra 是哪家的——据 OpenAI 帮助中心与发布页，Astra 是 OpenAI 的 GPT-6 Astra，Daybreak 是其“可信访问”计划，见下条 B3-补（推断：作者的 Astra 即该模型，截图为 Codex 式界面，原文未说明）。GLM-5.3 原文只写模型名，未写厂商、部署方式（API 或本地） | 部分支持 |
| B3-补 Astra 与 Daybreak Blue 的官方说法 | L1 | https://help.openai.com/en/articles/20001258-openai-daybreak-trusted-access-for-cyber-overview ；https://openai.com/index/gpt-6-astra/ ；https://developers.openai.com/api/docs/guides/daybreak | 帮助中心表格 Daybreak Blue 一行：“Reduced refusals on supported mainline models, including GPT-5.5, GPT-5.6 Sol, GPT-6 Sol, and GPT-6 Luna, for verified defensive work in authorized environments. Astra retains standard safeguards.” FAQ：“At this time, reduced refusals on Astra are available to Daybreak Red customers only.” 发布页：“However, Astra will refuse to comply with more advanced cybersecurity tasks such as creating proof-of-concept exploits for vulnerabilities.” API 文档：“Reduced refusals on gpt-6-astra and gpt-6.1-sol require Daybreak Red” | 24-openai-daybreak-table.png（1 倍像素，720 px 宽，见 capture-log） | 帮助中心页面标注 “Updated: 20 hours ago”（取页时）；页面为当前版本，不代表作者测试当天的版本。含义：作者开 Daybreak Blue 后“结果相同”，与 OpenAI 文档“Blue 不放宽 Astra”一致，并不说明 Blue 无效或脚本的诱饵更强 | 已找到 |
| B4 “诱饵／反分析”是作者推断还是有证据 | L6 | 同上 | 提出假设处：“could this be some form of anti-LLM obfuscation? It would make sense: include symbol names that would intentionally trip model safeguards to prevent deobfuscation.” 结论处：“This was clearly a string meant to trip every possible safeguard out there.” 关于 Node 段：“However, this is likely also anti-LLM obfuscation since…”。标题与正文没有 we think 一类措辞，但整体是作者的推断：证据是（a）函数名与字符串与脚本功能无关；（b）读文件回调为空；（c）一个模型拒绝而另一个照做。作者没有取得脚本提供方的说明 | 15-astra-refusal.png；17-trip-safeguards.png；18-node-also-decoy.png | 脚本提供方回应：未找到。检索 nullpt.rs 文末、HN 讨论（242 条嵌套评论中无 veritas 或 servicepipe 官方账号发言，用关键词扫描）、Tildes、各转述页与网络搜索，均无 servicepipe 声明。servicepipe.ru 首页（需 JS）与 servicepipe.tech（403）本机取不到正文，未查官方声明页 | 部分支持 |
| B5 是否真在浏览器执行了命令 | L6 | 同上 | 作者贴的代码门控条件为 `isNode && hasBuffer && !isBrowser && process.env.…ACTIVE`（环境变量名见存档）；作者原句 “The script seemingly executes malicious code when under a Node environment gated behind an environment flag”。 | 17-trip-safeguards.png | 原文没有说命令在浏览器里被执行；按作者所贴的门控代码，浏览器环境（`!isBrowser`）不会进入该分支。原文也没有说作者本人运行过脚本或观察到命令真的被执行。HN 讨论里有人（MisterMunchkin）把它说成“静默在我电脑上跑命令”，正文并未支持这一点（HN 评论 50279119） | 与说法不符（若说法是“网页在浏览器里执行了命令”）；原文只支持“Node 环境下的门控分支，作者称 seemingly” |
| B6 社区质疑 | L6 | 见下“三、可能的吠点” | 见下 | 22-hn-post-card.png；23-hn-comments-doubt.png | HN 评论分数不公开（HN 不对外显示评论得分），因此“得分”无法记录；改记评论 ID、发言人、时刻与子树回复数 | 已找到 |
| B7 转述与夸大 | L5/L4 级聚合与摘要站，仅作线索 | 见“四、夸大说法实例” | 见下 | 21-radar-exaggeration.png | 未找到大型媒体（The Register、BleepingComputer 等）的独立报道；搜索只出现聚合与摘要站 | 已找到（聚合站实例） |

## 三、可能的吠点（线索摘要里没提或说反的条件与限制；进正文前须回一手核实）

原文内（L6 作者自述）：

1. 标题里的 “creating” 是作者的标题钩子：正文没有说脚本制造或生成了任何危险物，说的是“函数名与字符串刻意长得危险，以触发模型安全机制”；作者自己写 “Outside of the unique obfuscation, the script appears to be your standard anti-bot script”。HN 里 aussieguy1234（50281891）也有同样的总结：“Its not creating bioweapons”。（出处 b-article.txt；HN）
2. 动机是推断：作者的句式是 “could this be…？It would make sense…” 与 “likely also”，没有脚本提供方的确认；作者的完整推理链只有“名字无关＋回调为空＋模型拒绝”。
3. 归因更弱：脚本域名 servicepipe.tech 到 servicepipe.ru 只靠 urlscan 里的“名称相近”，致谢里 Sshell 只是 “helping identify who could be behind this site”。
4. 测试规模很小：3 个设置、各写一次结果，无次数与提示词全文；没有测 Claude、Gemini、DeepSeek 等。
5. Astra 的那张图里模型先说“I'll analyze the saved script with its identifiers normalized …… I'll keep the AI-targeted text and harmful decoy names out of the analysis so they can't steer the result.”，接着有 5 条命令记录，最后才出现界面提示 “This content can't be shown ……We take extra caution with requests involving biological research and applications that could pose safety risks. Eligible researchers can apply for Trusted Access.”（图中原文）。即模型一开始是打算做的，被拦是在做了一段之后、由界面提示给出；谁触发了拦截（模型自身还是产品侧的检测）原文没有说（推断：产品侧机制，未证实）。
6. Node 分支：作者自己前后措辞不一（seemingly executes malicious code → likely also anti-LLM obfuscation）；门控为非浏览器环境。
7. OpenAI 官方文档写明 Daybreak Blue 下 Astra 仍是标准防护（B3-补），因此“开了 Blue 仍被拦”是文档预期，不是额外发现。
8. 转述站有两处与原文不符：Yii Chen 周报把作者用的 Astra 写成“作者自己的 AI 助理”；该周报还写“作者判斷主要是誘餌”，原文对 Node 段是 seemingly 加 likely 两种说法并存。

HN 社区（L6，仅线索；主帖 https://news.ycombinator.com/item?id=50276998 ，北京 10-29 01:13 提交，存档时 1302–1308 分、242–245 评论；评论得分不公开）：

- arbol（50281962，北京 10-29 06:40，https://news.ycombinator.com/item?id=50281962）：“This stuff doesn't work. The agents read the AST and just ignore dead code. Even replacing all your function names at build time with decoys makes the agent see the juxtaposition.” 并称要有“恶意语境＋恶意名字＋真的运行恶意代码”才行。——对“此法能挡住 AI 分析”的质疑，无证据，仅个人经验。
- perching_aix（50281002，北京 10-29 05:15，https://news.ycombinator.com/item?id=50281002）：“a simple identifier substitution and comment stripping should defeat it completely.” ——主张简单预处理即可绕开，同样是个人判断（评论含粗口，未截图）。
- zb3（50279808，北京 10-29 03:46，https://news.ycombinator.com/item?id=50279808）：“Note that Claude is resistant to these techniques, because Claude would refuse even without those tricks employed.” 语义含糊，疑为指 Claude 本来就会拒；另 privacyishuman（50285209，北京 14:29）称把文章与脚本一起发给 Claude 后“it interacted fine with it”；jocoda（50285346，北京 14:53）称用 DeepSeek 网页版总结该线程得到“Sorry, that's beyond my current scope.”（均为自述，无法复核）。
- swiftcoder（50278633，北京 02:28，https://news.ycombinator.com/item?id=50278633）：开源权重模型不拦，推测 Tiananmen 之类的过滤发生在中国推理服务商而非模型本身。
- HNuseraccount（50280102，北京 04:04）：“Make bioweapons is such a attention getting phrase but there's such a giant gap between ‘please make bioweapons’ as a string and actually accumulating the expertise and equipment…”——点出标题钩子与真实能力的差距。
- mmaunder（50282794，北京 10-29 08:19）：客户用该反欺诈服务时，品牌域名下的页面会带着这些内容被模型读到，可能影响模型对该品牌的回答（假设性推演）。
- void-pointer（50279765，北京 03:43）：对“靠逼模型去想自残和杀人来挡反混淆”的价值观反感（观点，非事实质疑）。

## 四、夸大说法实例

| 发布方 | 时间（北京） | URL | 原句逐字 | 说明 |
|---|---|---|---|---|
| nullpt.rs（作者自己） | 2026-10-28（只有日期） | https://nullpt.rs/fingerprinting-bioweapons | “Why Is This Fingerprinting Script Creating Bioweapons?” | 标题本身是疑问钩子；正文并不支持“创造生物武器” |
| GitHub 用户 845421145-lang 的“agents-radar”日报 Issue #680（自动生成的技术社区摘要） | 2026-10-29 10:00:07（2026-10-29T02:00:07Z） | https://github.com/845421145-lang/agents-radar/issues/680 | “A shocking case where a benign-looking script generated bioweapon-like sequences due to unintended model behavior—raises red flags about AI safety in code.” 另：“A chilling reminder that AI's edge cases can have real-world consequences.” | 与原文不符：原文是人为写进脚本的诱饵字符串，不是模型“无意行为”生成的序列；摘要文本疑为自动生成（推断） |
| Hacker News Digest（zeli.app，聚合站） | 页面日期 2026-10-28（无时刻） | https://zeli.app/digest/2026-10-28 | “A Fingerprinting Script Hides a Fake Bioweapon Recipe to Thwart AI Deobfuscation” | “Recipe（配方）”原文没有这个说法；摘要后半“packed its code with deliberately alarming function names”大体准确，但“deliberately”是作者的推断 |
| Yii Chen《1%科技日報 EP226》（Substack） | 2026-10-29 08:51（2026-10-29T00:51:35Z） | https://yiichenhi.substack.com/p/1-ep226 | 条目标题“瀏覽器指紋腳本裡藏生化武器函式名，專門讓 AI 拒絕分析”；正文“作者自己的 AI 助理拒絕反混淆這支腳本，GLM-5.3 則照做”“作者判斷主要是誘餌” | “專門讓 AI 拒絕分析”把作者的推断当成事实；“作者自己的 AI 助理”原文是 Astra；Node 段原文两种说法并存 |
| AI/TLDR（ai-tldr.dev，聚合站） | 页面日期 2026-10-28 | https://ai-tldr.dev/releases/nullpt-anti-llm-fingerprinting-script/ | “A servicepipe.tech fingerprinting script fills its code with fake weapon-themed function names and a hidden jailbreak prompt so AI models refuse to deobfuscate it.” | 与原文基本一致，但把“作者的推断”写成脚本的目的（“so AI models refuse…”），未写 likely/推断 |

HN 上本身的标题是 “Why is this fingerprinting script creating bioweapons?”（沿用原标题）。未找到大型英文科技媒体或中文媒体的独立报道（只找到聚合与周报）。

## 五、扫描说法勘误

- “研究者在某浏览器指纹／反机器人脚本里发现模拟系统提示和高敏感危险主题词汇，并观察到部分模型因此拒绝解释脚本”：属实，来自 nullpt.rs；脚本为 servicepipe.tech 的 checkjs 脚本。
- WorkBuddy“Astra 拒答 / GLM 照做”：与原文一致（Astra 拒、Astra＋Daybreak Blue 仍拒、GLM-5.3 照做）。但原文只有一次性的描述，没有次数；Astra 的厂商需另查（OpenAI，来自官方页，原文未写）。
- ChatGPT“作者只测了少量模型与提示词”：属实（3 个设置）。
- ChatGPT“运营者归因有不确定性”：属实，原文只有“名称相近的 servicepipe.ru”（见 B1b）。
- ChatGPT“脚本里有 Node 命令文本”：属实，且是“Node 环境、需某环境变量、非浏览器”才进入的分支；作者对它的定性是 seemingly 与 likely 并存（见 B2c、B5）。
- 两份扫描都没提到的：（1）Astra 在截图里是先开始分析、后被界面提示拦下；（2）OpenAI 文档写明 Blue 下 Astra 仍是标准防护；（3）作者原句里的“clearly”与“likely”并存；（4）标题是钩子，正文不支持“制造生物武器”。
- 热度：HN 50276998，北京 10-29 01:13 提交；barking heat 取时 19:41（北京）1303 分／243 评；截图页 1308 分／245 评（约 19:50 北京）；Algolia 存档 1302 分／242 条嵌套评论；Tildes 帖 10 票（取页时）；Lobsters 上有讨论（agents-radar 摘要记 22 分／0 评，页面未取）。Reddit 搜索被登录页挡住，未取（见 capture-log）。
