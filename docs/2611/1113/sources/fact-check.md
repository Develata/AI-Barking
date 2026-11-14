# 1113 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（哥德尔笔记）、`evidence-B.md`（Anthropic Anvil）、`evidence-C.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证：A、B 组 claude-sonnet-5-5，C 组 claude-haiku-5-5，北京 11-14 21:50 前后派出（C 组约 6 分钟交回）；Claude 对标题、省流与吠点所涉原句逐字回到存档复核。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI读哥德尔笔记，50条定理是错的？ | L4 | ⚠️ | 见 A 节 | 问句，问的是《卫报》报道的说法（von Plato 口述“About 50 were false”）；“AI”指 Claude。读者稿检查后由双标题改为单钩子（双标题读者以为“关防护”与读笔记有关），见 `titles.md`、`style-review.md`。 |
| 省流：据《卫报》报道，Claude 从哥德尔笔记里列出663条定理，约50条是错的，它拿到的是学者的转写稿 | L4 | ⚠️ | 见 A 节 | 署名“据《卫报》报道”。 |
| 省流：Anthropic 称用 AI 攻击自家防护，找漏洞胜过人类，但攻击用的 Claude 关了自己的防护 | L1 | ✅ | 见 B 节 | “outperforms human red-teamers against our bioweapons, cybersecurity, and distillation safeguards”；“frontier internal versions of Claude with safeguards disabled”。 |

## A 哥德尔笔记

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据《卫报》报道，逻辑学家 von Plato 自2017年研究哥德尔的16本笔记，共1009页，大多用早已过时的德文速记写成，每周只能读懂约一页 | L4 | ⚠️ | 《卫报》Alex Bellos https://www.theguardian.com/science/2026/nov/13/kurt-godel-notebooks-ai-helps-unlock-hidden-discoveries-man-who-transformed-maths （`a-guardian.txt`；图 01、02） | 北京 11-14 00:38 发布（页眉 Fri 13 Nov 2026 11.38 EST），02:38 修改。“Since 2017, the Finnish logician Jan von Plato has led a project at Helsinki University to understand the contents of 16 notebooks … 1,009 pages … Much of it is written in a German shorthand, Gabelsberger, that was obsolete even when Gödel was using it.”“Progress in the mathematical understanding of the text was painfully slow – about a page a week”。页数口径各处不一：赫尔辛基项目页 “some twelve hundred shorthand pages”、AITP 摘要 “some 1700 pages”（evidence-A 第 2 节），正文按《卫报》并署名。 |
| 今年9月，他把自己转写的稿子和自己的6本旧著交给 Claude | L4 | ⚠️ | `a-guardian.txt` 第 266、288 行；图 03 | “until September, when von Plato enlisted the help of the AI model Claude”；“showing Claude his transcriptions of the workbooks, as well as six books he had previously written on Gödel and other secondary scientific literature.”（另有二手文献，正文从略）。650 页解读、选择公理一段因读者稿检查与字数删去（选择公理段原文：Claude 回答 “yes, almost”，von Plato 称 “just one step away”，重建的证明经他放进 Coq 通过；图 04）。 |
| Claude 列出哥德尔写过证明或思路的定理，共663条 | L4 | ⚠️ | `a-guardian.txt` 第 322 行；图 05 | “asked the AI model to extract all theorems in the workbooks for which Gödel either had a proof, had sketched a proof or had an idea for a proof. Claude found 663.” “思路”= idea for a proof；“证明”含 sketched a proof。 |
| 他说放进证明检查软件 Coq 后，约400条照原样成立，约200条要改写或补条件，约50条是错的 | L4 | ⚠️ | `a-guardian.txt` 第 322 行；图 05 | “When these results were put into Coq: ‘About 400 were verified as stated, about 200 were repaired, meaning that perhaps the statement of the theorem had to be made more precise, or some assumption had to be added. About 50 were false’”。“改写或补条件”= made more precise / assumption added。400+200+50=650，余约 13 条《卫报》未交代（维也纳演讲机器字幕称 55 条错、5–8 条在做，未核音频，不用）。谁放进 Coq 原文未写（被动 “were put into Coq”），正文同样不写主语。 |
| （同句）哥德尔自己也在页边批过“错！” | L4 | ⚠️ | `a-guardian.txt` 第 322 行；图 05 | “and this is no wonder. Gödel himself, when rereading his notes, would occasionally add a marginal remark: falsch! [incorrect] or schlecht! [bad].” “错！”译 falsch!（原文 “occasionally”，正文“批过”不含频率）。 |
| 吠点①：据《卫报》，Claude 拿到的是 von Plato 转写好的稿子；36氪转载的新智元文章，标题却是“Claude 破译哥德尔天书” | L4 / 媒体原句 | ⚠️ | `a-guardian.txt` 第 286、288 行；新智元文（36氪转载）https://www.36kr.com/p/4069178330764161 （`a-36kr.txt` 第 1、135 行，图 11，页面标 11-14 14:09） | 输入为 “his transcriptions of the workbooks”；《卫报》另引 von Plato 自学速记、自称 “I am one of only six people who can read it in the world”。AITP 2026 幻灯片 “Machine reading harder by orders of magnitude than standard handwriting”（图 08，未提 Claude）。反向核验第一轮 BLOCKER：原写“Claude 没读速记”，是由“给了转写稿”推出的排他断言，已改为只陈述《卫报》写的输入，并署名“据《卫报》”，结果由 ✅ 改 ⚠️。36氪转载页标题“Claude破译哥德尔天书，80年笔记震撼数学界”（微信原文 curl 只取到验证页，未核原标题是否相同，正文因此写“36氪转载的新智元文章”），正文按本号空格规范写作“Claude 破译哥德尔天书”。 |
| 吠点②：该文还写 Claude“挖出440多条全新定理”，而据《卫报》，663条里含只有思路的，约50条是错的 | 媒体原句 / L4 | ⚠️ | `a-36kr.txt` 第 151 行；`a-guardian.txt` 第 322 行 | 36氪转载页原句：“几天内，Claude就读完上千页笔记，挖出440 多条全新定理！”（正文按空格规范写“440多条”）。440 与《卫报》任何数字都对不上（evidence-A 第 4 节）。663 的定义含 “had an idea for a proof”；“约50条是错的”= “About 50 were false”。《卫报》第三段记者叙述 “revealing Gödel had proved hundreds of theorems that no one knew about” 也是同类概括，正文不写。 |
| 吠点③：这些数字来自《卫报》转述 von Plato 的说法，本号截至14日没找到论文或公开的检查文件 | — | ✅ | evidence-A 第 A2、A4 行 | A 组查过：arXiv（0 条）、GitHub（Gödel workbooks Coq，0 条）、赫尔辛基大学新闻、ERC CORDIS（GODELIANA 787758、HERITAGE 101198605）、Anthropic 官网，均无此事的论文、新闻稿或 Coq 文件；员工社媒帖无搜索接口，未完成检索（evidence-A 第 38 行），正文只说论文与检查文件。更早的一手是 von Plato 10-19 维也纳大学演讲（预告 PDF `a-vcs-lecture.pdf`、字幕 `a-yt-lecture.txt`，图 07），数字只见于机器字幕、未核音频，第二轮 SHOULD_FIX 2 后改为“这些数字来自《卫报》转述 von Plato 的说法”，不再写“只来自”。“检查文件”= Coq 形式化文件。 |

## B Anthropic Anvil

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 称，自建的 AI 红队 Anvil 专门扮演攻击者，找自家 Claude 防护的漏洞，即“越狱” | L1 | ✅ | https://safeguards.anthropic.com/2026/automated-red-teaming/ （`b-post.txt`，图 13） | 页面日期 November 12, 2026（`<time datetime="2026-11-12">`，官方未给时刻；北京日期按 11月12日或13日均可能；反向核验第一轮 SHOULD_FIX 6 后正文删去日期）。“we built Anvil, an automated red-teaming system, which outperforms human red-teamers against our bioweapons, cybersecurity, and distillation safeguards.” 23/24 与 24–48 小时早在 RSP 更新页 10-19 条已公开（`b-rsp-updates.txt`，图 30），11-12 为详细版；正文不写。 |
| 8道生物武器题分别对3个版本的防护，共24组。据 Anthropic 自测，Anvil 有23组找到比人类更严重的越狱，1组打平 | L1 | ✅ | `b-post.txt` 第 56 行；图 17 | “Anvil finds a substantially better jailbreak than the bounty in 23 out of 24 of the question-classifier pairs, and ties the human solution on the remaining single instance at the maximum score.” 24 组 = 8 道常设赏金题 × 3 代分类器（赏金 2026-04-20 至 09-11）。严重度为模型打分（“model-graded severity score … validated with an Anthropic biologist”），正文不展开。 |
| 用时24到48小时 | L1 | ✅ | `b-post.txt` 第 34、56 行 | “finds these jailbreaks in 24–48 hours rather than weeks to months”。口径：Anvil 为墙钟时间（并行多会话），人类 430 小时为七周多累计投入工时，两者不同口径；正文写“累计投入”并加“据 Anthropic 自测”（反向核验第一轮 SHOULD_FIX 3）。胜负按模型打分、Anthropic 生物学家校验。 |
| 人类悬赏测试者跨7周多累计投入430小时，8道题只在1道上找到高危越狱 | L1 | ✅ | `b-post.txt` 第 24 行；图 14 | “red-teamers found a high-severity jailbreak on only 1 of 8 questions, and a weaker jailbreak on 3 others, despite 430 hours of dedicated red-teaming over more than seven weeks.” |
| （已删，读者稿检查后为字数删去）Anthropic 称，外部攻击者现在很难造出同样强的系统，但 Anvil 随底层模型变强而变强 | L1 | ✅ | `b-post.txt` 第 16 行；图 13 | “It would be very difficult for an external attacker to build a system as strong as Anvil today … But Anvil’s performance improves with the strength of the model it runs on, suggesting that attackers could soon build similar systems on models with weaker safeguards.” |
| 吠点①：发起攻击的是关掉自身防护的内部 Claude；Anthropic 自测，给它打开防护，97%的攻击会话会被拦下，多数在第一条消息就被拦 | L1 | ✅ | `b-post.txt` 第 16、168 行；图 13、22 | “It relies on frontier internal versions of Claude with safeguards disabled”；“we also measured the aggregate block rate across cyber, anti-distillation and bio, and found that the respective safeguards would block 97% of 1,453 red-teaming sessions, 85% of them at the attacker’s first message.” Fig 11 图注：“With safeguards on (black) … the attacker model is blocked at its first request on every task.”（仅网络任务）。“85% of them” 指 1,453 个会话的 85% 还是被拦的 97% 中的 85%，原文有歧义；两种读法都过半，正文写“多数”。“它”= 发起攻击的 Claude。不写“分类器精确分数是关键”：原文称 “internal visibility is not required to find jailbreaks on many tasks”（二值反馈 Cybertasks 107 对完整 114，仅在图内）。 |
| 吠点②：智东西标题称“只用人类1/235的请求量，就找到严重漏洞”，比的只是双方首次套出“实质性”隐藏思考的提问次数，套出的是一道编程题的思考 | L1 / 媒体原句 | ✅ | `b-post.txt` 第 34、138 行；智东西网易号 https://www.163.com/dy/article/LBKGPTI4051180F7.html （`b-zhidx163.txt` 第 251 行，北京 11-13 15:19） | 官方：“Anvil first leaks the model’s hidden CoT reasoning, on a coding problem, after 435 requests, whereas the expert red-teamers took around 102,200 requests to reach their first substantial leak.” 435/102,200≈1/235，官方文未写此比值。“实质性”= 内部泄露量表至少 70/100（“An attack counts as substantial only if it scores at least 70/100 across graders”），反向核验第一轮 SHOULD_FIX 4 后补引号。整体：人类一次战役约 204 万次请求（“During one red-teaming campaign”），Anvil 约 649 会话、约 13.2 万次。属反蒸馏防护（防止套出模型推理），与生物、网络防护不同。 |
| 吠点③：人类专家在网络安全题上报了11组成功，Anthropic 称其中9组的攻击代码是防护更弱的其他 AI 写或运行的，按 Anvil 的标准不算 | L1 | ✅ | `b-post.txt` 第 88 行；图 18 | “they reported successful breaks on 11 task-model pairs … in 9 of 11 of these instances, a separate helper model with weaker or no cyber safeguards wrote the malicious attack code or executed it. This does not meet the stricter bar we set for Anvil where the safeguarded model must write and execute every step itself.” 专家会话是人加 AI 代理协作。扫描说“人类 0 个”不准：对 Opus 5.5 的 13 个里有 1 个完整利用（第 26 行）。耗时数字官方前后不一（摘要 0.4–4.4、正文 0.5–3.5、图 5 0.39–4.4 小时），正文不写。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Wallet 首发经 dots，限美国18岁以上 Pro 用户、持美国合格卡 | L1 | ✅ | https://openai.com/index/introducing-chatgpt-wallet/ （`c-c1-wallet.txt`） | 页面日期 November 13, 2026（官方未给时刻），“begins rolling out today”；“It will initially be available to ChatGPT Pro subscribers aged 18 and older in the US with eligible US-issued cards.”（Claude grep 存档命中）；“须持美国合格卡”= eligible US-issued cards（反向核验第一轮 SHOULD_FIX 5 后补）；首发入口 dots（“initially be available through dots”）第二轮 SHOULD_FIX 4 后补进；标签为 ChatGPT，句子省去重复的“ChatGPT”以压进 40 字。与 AmEx、Mastercard、Visa 合作。扫描的“上线初期不抽成”官方文未见（C 组 grep fee 无命中），不收。热度：HN 50484640，13 分 / 4 评，11-13 23:07。 |
| 四家非营利组织称已起诉五家 AI 公司，要求冻结更先进模型开发 | L1 | ✅ | Public Citizen 新闻稿 https://www.citizen.org/news/coalition-of-nonprofits-files-landmark-lawsuit-to-stop-development-of-advanced-ai-systems-at-anthropic-google-meta-openai-and-spacexsi-that-proliferate-bioweapons-and-cyberattacks/ （`c-c7-citizen.txt`） | 原告方新闻稿，日期 November 12, 2026（官方未给时刻）：“Today, Irreplaceable Education Fund, Public Citizen, Heat Initiative, and The Alliance for Secure AI filed a lawsuit … seeking to freeze the development of increasingly advanced AI models at Anthropic, Google, Meta, OpenAI, and SpaceXSI”；法院 “Superior Court of California, County of San Francisco”；依据加州《不正当竞争法》。“称”= 原告自述，法院与诉状正文未独立核实；截至 C 组取证未见法院裁定。第五被告按原文写 SpaceXSI，图上只写“五家 AI 公司”。另一起 9/18 联邦反垄断集体诉讼（“Pacing the Frontier”）不是本案。热度：HN 0 条；扫描称中文侧 11-13 集中转写（未核）。 |
| 据 Fortune 报道，OpenAI 解散独立机器人团队，研究员并入主研究团队 | L4 | ⚠️ | https://fortune.com/2026/11/13/openai-disbands-robotics-team-as-its-general-models-start-running-robots/ （`c-c8-fortune.txt`） | 北京 11-14 01:05（原文 Nov 13, 12:05 PM ET）。“OpenAI has disbanded its stand-alone robotics team …”“The company confirmed the reorganization when asked about it by Fortune.”“reassigned its robotics research team to the company's main research team, reporting to Chief Research Officer Mark Chen, while the applied robotics and hardware teams have been reassigned to the company's larger Applied organization”（Claude grep 存档命中）。L4 措辞“据 Fortune 报道”。74.8% 对 51.0% 为“an Nvidia study … simulated manipulation benchmark”，图上不写。热度：HN 50487184，172 分 / 72 评，11-14 01:55。 |

速览不收：Claude Code Projects 更新（官方文章日期 2026-09-17，HN 帖 11-14 指向旧文；扫描的“单线程只读分享”与文档相反，文档写线程不能分享；/btw、/fork 未见）；LangChain C 轮（融资类）；Devin Teams 席位上限 200→100（定价细节、HN 0 条）；Antigravity Dynamic Workflows（11-13 App v2.32.0，HN 0 条，圈内工具）；Sierra Poppy 1.0（HN 0 条）；Blackwell 芯片指纹报告（风控）；Muse Ember、Astral 收购、Worse Together、文心 5.5 Flash Preview（旧闻重推）；英伟达担保与 Haas（转述链或非 AI 主线）；微软 365 故障。

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| godel-title：（哥德尔）留下了1009页写得密密麻麻的文字 | — | ✅ | 底图 `images/godel-title-raw.png`；原句：“left 1,009 pages of densely written text” | 《卫报》报道 · 标题与导语。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| godel-title：据称其中有数百条已证明的定理 | — | ✅ | 底图 `images/godel-title-raw.png`；原句：“which is now said to contain hundreds of proved theorems” | 《卫报》报道 · 标题与导语。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| godel-transcriptions：（von Plato）自学了 Gabelsberger 速记 | — | ✅ | 底图 `images/godel-transcriptions-raw.png`；原句：“taught himself Gabelsberger shorthand” | 《卫报》报道 · 交给 Claude 的材料。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| godel-transcriptions：给 Claude 看他对这些笔记的转写稿，以及他此前写的六本书 | — | ✅ | 底图 `images/godel-transcriptions-raw.png`；原句：“showing Claude his transcriptions of the workbooks, as well as six books he had previously written” | 《卫报》报道 · 交给 Claude 的材料。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| godel-663：（哥德尔）有证明、写过证明梗概或有证明思路的（定理）。Claude 找出了663条。 | — | ✅ | 底图 `images/godel-663-raw.png`；原句：“either had a proof, had sketched a proof or had an idea for a proof. Claude found 663.” | 《卫报》报道 · 663 条与 Coq 检查。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| godel-663：（von Plato：）约400条照原样通过验证，约200条经过修补 | — | ✅ | 底图 `images/godel-663-raw.png`；原句：“About 400 were verified as stated, about 200 were repaired” | 《卫报》报道 · 663 条与 Coq 检查。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| godel-663：约50条是错的，这不奇怪。 | — | ✅ | 底图 `images/godel-663-raw.png`；原句：“About 50 were false, and this is no wonder.” | 《卫报》报道 · 663 条与 Coq 检查。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| kr-title：原文标题：“Claude破译哥德尔天书，80年笔记震撼数学界” | — | ✅ | 底图 `images/kr-title-raw.png`；原句：“Claude破译哥德尔天书，80年笔记震撼数学界”（`a-36kr.txt` 第 135 行） | 新智元文章（36氪转载）。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| kr-title：原文：“几天内，Claude就读完上千页笔记，挖出440 多条全新定理！” | — | ✅ | 底图 `images/kr-title-raw.png`；原句：“几天内，Claude就读完上千页笔记，挖出440 多条全新定理！”（`a-36kr.txt` 第 151 行） | 新智元文章（36氪转载）。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-disabled：（Anvil）在我们的生物武器、网络安全和防蒸馏防护上，胜过人类红队测试者。 | — | ✅ | 底图 `images/anvil-disabled-raw.png`；原句：“which outperforms human red-teamers against our bioweapons, cybersecurity, and distillation safeguards.” | Anthropic Safeguards 博客 · 开头。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-disabled：它依靠关闭了防护的内部前沿版 Claude 来找出最好的越狱方法 | — | ✅ | 底图 `images/anvil-disabled-raw.png`；原句：“It relies on frontier internal versions of Claude with safeguards disabled to find the best jailbreaks” | Anthropic Safeguards 博客 · 开头。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-bio：（越狱严重度用）模型打分的严重度分数（衡量） | — | ✅ | 底图 `images/anvil-bio-raw.png`；原句：“model-graded severity score” | Anthropic Safeguards 博客 · 生物武器题。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-bio：（Anvil 找到明显更好的越狱）在24组“题目×分类器”中有23组，剩下1组与人类的方案同为满分打平。 | — | ✅ | 底图 `images/anvil-bio-raw.png`；原句：“23 out of 24 of the question-classifier pairs, and ties the human solution on the remaining single instance at the maximum score.” | Anthropic Safeguards 博客 · 生物武器题。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-human：尽管在七周多里专门投入了430小时，红队测试者只在8道题中的1道上找到高危越狱，另有3道找到较弱的越狱。 | — | ✅ | 底图 `images/anvil-human-raw.png`；原句：“red-teamers found a high-severity jailbreak on only 1 of 8 questions, and a weaker jailbreak on 3 others, despite 430 hours of dedicated red-teaming over more than seven weeks.” | Anthropic Safeguards 博客 · 人类基线。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-ablation：给攻击方模型打开防护后，绝大多数攻击请求被我们直接拦下 | — | ✅ | 底图 `images/anvil-ablation-raw.png`；原句：“When safeguards are enabled on the attacker model, we block the great majority of attacker requests outright” | Anthropic Safeguards 博客 · 给攻击方打开防护。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-ablation：相应的防护会拦下1453个红队会话中的97%，其中85%在攻击方的第一条消息就被拦。 | — | ✅ | 底图 `images/anvil-ablation-raw.png`；原句：“the respective safeguards would block 97% of 1,453 red-teaming sessions, 85% of them at the attacker’s first message.” | Anthropic Safeguards 博客 · 给攻击方打开防护。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-distill：一次攻击在内部泄露量表上、经多位评分者评定得分至少70/100，才算“实质性”泄露。 | — | ✅ | 底图 `images/anvil-distill-raw.png`；原句：“An attack counts as substantial only if it scores at least 70/100 across graders on an internal leakage rubric.” | Anthropic Safeguards 博客 · 套出隐藏思考。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-distill：（Anvil 首次套出模型的隐藏思考）是在一道编程题上、第435次请求之后，而人类专家用了约102,200次请求才达到首次实质性泄露。 | — | ✅ | 底图 `images/anvil-distill-raw.png`；原句：“on a coding problem, after 435 requests, whereas the expert red-teamers took around 102,200 requests to reach their first substantial leak.” | Anthropic Safeguards 博客 · 套出隐藏思考。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-cyber：他们（人类专家）报告在11组“任务×模型”上成功突破 | — | ✅ | 底图 `images/anvil-cyber-raw.png`；原句：“they reported successful breaks on 11 task-model pairs” | Anthropic Safeguards 博客 · 网络安全题。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-cyber：11例中有9例，恶意攻击代码是由另一个网络安全防护更弱或没有防护的辅助模型写出或运行的。 | — | ✅ | 底图 `images/anvil-cyber-raw.png`；原句：“in 9 of 11 of these instances, a separate helper model with weaker or no cyber safeguards wrote the malicious attack code or executed it.” | Anthropic Safeguards 博客 · 网络安全题。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |
| anvil-cyber：这达不到我们给 Anvil 定的更严标准 | — | ✅ | 底图 `images/anvil-cyber-raw.png`；原句：“This does not meet the stricter bar we set for Anvil” | Anthropic Safeguards 博客 · 网络安全题。译注为直译，限定词照搬；括号内为本号补出的省略主语。 |

补注：anvil-ablation 第 2 条“其中85%”照原文 “85% of them” 直译，保留原文的歧义（正文写“多数”）；kr-title 两条为中文原句，非翻译。

## 读者稿检查

报告见 `style-review.md`（Codex gpt-6-astra，medium，只读，无仓库上下文）。采纳：标题由双钩子改为单钩子（读者以为“关防护”与读笔记相关，“红队”看不懂）；删选择公理一段（读者在此想划走；也删去 Hamkins 吠点，它评的是选择公理重建）；新吠点②纠正“挖出440多条全新定理”；红队、越狱、3代防护、“它”补白话；补“哥德尔自己也在页边批过‘错！’”回答“错算谁的”。不采纳：固定开场与 Slogan（品牌固定）；“补假设”“Coq 文件”太专业（已改“补条件”“检查文件”）。

## 反向核验

第一轮（北京 11-14 约 23:00，Codex gpt-6-astra medium，只读，`barking review --reverse`；审的是读者稿检查前的版本，问题逐条映射到新稿）：
- BLOCKER 1“Claude 没读速记”排他断言 → 采纳，吠点①改为“据《卫报》，Claude 拿到的是 von Plato 转写好的稿子”，清单改 ⚠️。
- SHOULD_FIX 2 Coq 结果口述限定 → 采纳，补“他说”。
- SHOULD_FIX 3 计时口径与自家评分 → 采纳，补“据 Anthropic 自测”“跨7周多累计投入”。
- SHOULD_FIX 4 “实质性泄露”门槛 → 采纳，补“实质性”。
- SHOULD_FIX 5 Wallet 合格卡 → 采纳，速览改写；dots 入口字数放不下，记在清单。
- SHOULD_FIX 6 页面日期换算北京时间 → 采纳，正文删“11月12日”。
- SHOULD_FIX 7 README 与清单不一致 → 采纳，README 改“正文采用《卫报》1009页并署名”，清单改“员工帖未完成检索”。
- NEEDS_VERIFICATION 8 卡片缺失 → 卡片渲染后第二轮短审与视觉复核一并补。

第二轮（北京 11-15 约 00:10，同一命令；误用第一轮提示词文件并追加了“第二轮短审”说明段，审查者读的是磁盘上的现稿与 cards.toml）：PASS，无 BLOCKER。
- SHOULD_FIX 1 吠点②后半无来源 → 采纳，补“据《卫报》”。
- SHOULD_FIX 2 “数字只来自……口述”排他 → 采纳，改“这些数字来自《卫报》转述 von Plato 的说法”（维也纳演讲字幕也有这组数，未核音频）。
- SHOULD_FIX 3 README 残留“Claude 没读速记” → 采纳，README 改写；evidence-A 第 12 行追加复核更正。
- SHOULD_FIX 4 Wallet 首发入口 dots → 采纳，速览改“Wallet 首发经 dots，限美国18岁以上 Pro 用户、持美国合格卡”。
- SHOULD_FIX 5 “obsolete”译成“早已没人用” → 采纳，改“早已过时”。
- NICE_TO_HAVE 6 批注行原句栏填了坐标 → 采纳，改填中文原句与行号。

## 视觉复核

北京 11-15 约 00:40，Codex gpt-6-astra medium，只读，`barking review --visual`，附 28 张图（两张封面、16 张上传图与 10 张底图）：PASS，无 BLOCKER。
- SHOULD_FIX 16-anvil-cyber 高亮②停在 “executed it.”，译注含后半句 → 采纳，拆成两条 note（②到 “executed it.”，③ “This does not meet the stricter bar we set for Anvil”）。
- SHOULD_FIX 15-anvil-distill 译注①“被各评分者打到至少70/100”像逐人门槛 → 采纳，改“经多位评分者评定得分至少70/100”（原文 “across graders”）。
- SHOULD_FIX 06-godel-transcriptions 高亮停在 “on”、译注含“关于哥德尔” → 采纳，quote 收到 “previously written”（Gödel 一词 OCR 读不出 ö），译注删“关于哥德尔”。
- SHOULD_FIX 11、12、14 原文偏小 → 采纳，底图重裁：11 去掉大标题只留作者日期与开头两段（1080 px 高），12 只留生物段文字（830 px），14 只留消融段文字（725 px），图表留在取证截图 17、22。
- NICE_TO_HAVE 封面与标题“50条”省了“约” → 不采纳：标题是问句、正文与卡片都写“约50条”；改标题须重出封面，不值。
重渲后 Claude 打开 11、16 核对高亮与译注对应。
