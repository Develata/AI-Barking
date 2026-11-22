# 1121 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B 组 Sonnet 5.5（`claude-sonnet-5-5`），C 组 Haiku 5.5（`claude-haiku-5-5`）；用时（harness）A 23 分钟、B 22 分钟、C 8 分钟，未催交。Claude 抽查：C 组 Hy4 许可第 10.2 条、IPO 股数、Runway、MedChemBench、BDCP、Sutherland 原句（grep 存档逐字对上）；B 组 PDF 第 12 页“potentially impractical”、Vassilevska Williams 个人页（Claude 北京约 23:25 重取 `thoughts.txt`，与 B 组经 Exa 取得的 `b-vvw-thoughts.txt` 去空白后一致）；A 组 Together 整改条目（见 `README.md` 勘误：Claude 初核读漏“Checking all pull requests for credentials.”）。

## 一、AI 推理供应链提示词泄露（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI写代码漏了密钥，用户提示词被下载 | L1 | ✅ | Together 公告 `a-together.txt`：“an AI coding agent committed a credential that provided access to the private Hugging Face repository”；“An unauthorized party downloaded stored prompt samples after a credential for a private repository was exposed in a public code repository.” | “用户提示词”：公告称样本来自“a subset of prompts submitted to Together AI’s serverless inference service in June and July 2026”。陈述句只写 Together 自述，未写“全部” |
| Together 称自家 AI 编码助手把密钥传上公开仓库，部分用户提示词遭下载（省流） | L1 | ✅ | 同上 | “部分”= “a subset” |
| OpenRouter 撤下125个接口的“零留存”资格，含预防性撤下的（省流） | L1 | ✅ | OpenRouter 公告 `a-openrouter-announce.txt`（页面“11/22/2026”，官方未给时刻；官方 X 帖北京 11-22 09:32:38）：“Removed 125 endpoints from Zero Data Retention (ZDR) eligibility. The removals included providers flagged by researchers, plus additional endpoints that we removed as a precautionary measure.” | “接口”= endpoints（某提供商上的某个模型端点）；“零留存”= Zero Data Retention（ZDR）。官方 X 帖写“removed many endpoints”无数字，博客写 125 |
| Dragos 研究者往各家 AI 服务发了约200万条藏着假密钥和秘密链接的提示词 | L1（研究方自家发布） | ✅ | Dragos 报告 `a-dragos.txt`（datePublished 2026-11-19T13:00Z，北京 11-19 21:00；作者 HD Moore）：“Over a five-month period from July through November 2026, we submitted approximately two million prompts containing unique URLs and canary token credentials to widely used AI services and monitored what happened after inference.” | “假密钥”= canary token credentials；“秘密链接”= unique URLs |
| AI 中转平台 OpenRouter 11月22日称，据此清退3家提供商，撤下125个接口的“零留存”资格，含预防性撤下的 | L1 | ✅ | `a-openrouter-announce.txt`：“Earlier this week, security researchers reported …”（链接到 Dragos 报告）；“Off-boarded three providers from OpenRouter, based on the results of our investigations.”；“Removed 125 endpoints …” | “据此”：公告开头以 Dragos 报告为起因，清退写“based on the results of our investigations”。被清退的三家未点名 |
| 11月21日，提供商 Together 称，AI 编码助手往公开仓库提交了一把能读私有数据的密钥，撤回前已有人拿到，下载了6、7月存下的部分提示词 | L1 | ✅ | `a-together.txt`（“Published 11/21/2026”，署名 Vipul Ved Prakash，官方未给时刻与时区）：“During development of our public GitHub repository, xorl, an AI coding agent committed a credential that provided access to the private Hugging Face repository. Our automated scanning detected the exposed credential, and we reverted the commit.”“During the exposure window, an unauthorized party obtained the credential and used it to download the stored prompt samples.”；样本“primarily on June 24, July 4, July 8, and July 14, 2026” | “11月21日”照页面所标日期，不换算北京时间（反向核验第 11 条后由“前一天”改）。“密钥”= credential；暴露窗口长度公告未写 |
| 对你：Together 老账户要在隐私设置里关掉“提示词存储”，只管之后的请求，还会停用转发第三方的模型 | L1 | ✅ | 公告：“Making zero data retention (ZDR) the default setting for new accounts.”（只说新账户）；文档 `a-together-zdr-doc.txt`（北京 23:08 取）：“1. Go to the Privacy section of Organization Settings. 2. Turn off Prompt storage.”“Turning off prompt storage also turns off passthrough models, because passthrough requires retained prompts.”“ZDR applies only to requests sent after you enable it.” | “老账户”是公告（只改新账户）与文档（“ZDR is not enabled by default”）合读；文档另写此前数据可在设置里删除或发邮件 |
| 对你：OpenRouter 可设成只走零留存接口 | L1 | ✅ | `a-or-zdr-blog.txt`：“You can enforce ZDR across every request without touching your code, from your account’s privacy settings”；“you can restrict a request to eligible endpoints by setting provider.zdr to true” | 账户设置页需登录，未存档 |
| 吠点①：Dragos 写明，隐私开关只管请求的第一站 | L1（研究方） | ✅ | `a-dragos.txt`：“Keep in mind that a privacy flag only binds the first hop, and that AI agents will happily send whatever they can read, including credentials.” | “第一站”= first hop。原吠点①（The Hacker News 标题）按读者稿检查删去，摘句留在 `evidence-a.md` |
| 吠点①：它追踪约2000条转发路线，少数把测试图片发给了路线上没列出的公司 | L1（研究方） | ✅ | `a-dragos.txt`：“So far, we have traced about two thousand routes. Several hundred of those included a system that we couldn’t identify, and a handful sent our image to a company that wasn’t on the advertised path at all.” | “少数”= a handful；“测试图片”= 追踪用图片 URL（“we include an image URL in the request that points back to our servers”） |
| 吠点②：Together 称被下载的是“获授权”测试样本 | L1 | ✅ | `a-together.txt`：“As part of routine software testing, we collected prompt samples authorized for that use and stored them in a private Hugging Face repository.” | |
| 吠点②：其文档写明，没开零留存的账户，提示词默认会被存 | L1 | ✅ | `a-together-zdr-doc.txt`：“Unless your organization enables ZDR, Together stores the prompts you send and the responses models return, and may use them for product improvements.” | 文档页北京 23:08 取档 |
| 小标题：AI 助手漏密钥，提示词被下载 | L1 | ✅ | 同标题 | |

仅存档、不进正文（风控，Develata 11-22 拍板）：Novita 通告、Dragos 报告中涉及中国机构的段落、Ars Technica 更新段、HN “Tell HN: OpenRouter prompts to Minimax may have been read”——摘句见 `evidence-a.md` 第五节。截图 `images/05-dragos-intro-findings.png` 只截到第二条发现；`02-or-provider-announcements.png` 含“Novita: Privacy notice regarding MiniMax requests on Novita”链接文字，不进正式配图。

## 二、3SUM 论文（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| “Claude 推翻 3SUM 猜想”的论文10月就发了（省流） | L3 | ✅ | arXiv `b-arxiv-abs.txt`：“[v1] Mon, 5 Oct 2026 17:44:29 UTC”（北京 10-06 01:44:29），只有 v1；摘要“This refutes the 3SUM and APSP hypotheses”；第 1 页：“Claude … discovered the algorithm that refutes the 3SUM, APSP, and Exact Triangle hypotheses.” | 引号内是转发的说法；论文作者署名、学者自述，级别按 L3（作者对自家论文的陈述） |
| 3SUM 问 n 个数里有没有三个加起来为0，课上教的解法约要 n² 步 | L3 | ✅ | 论文 `b-arxiv-pdf.txt` 第 2 页：“3SUM: Given n numbers, decide whether three of them sum to 0. … the O(n2)-time algorithm taught in algorithms classes has only been sped up by polylogarithmic factors” | |
| 11月22日，多家中文网站转发 Claude 发现算法推翻 3SUM 猜想的消息 | L5（夸大实例） | ✅ | 搜狐号“财法观天下”`b-sohu.txt`（页面 11-22 04:27）；新浪微博页 IT 时代网 `b-sinacn-claude.txt`（11-22 04:32）标题“Claude 独立发现新算法，打破 3SUM 速度极限猜想”；AI万址、爱查AI 等聚合站（`b-aiwz.txt`、`b-aichaai.txt`）；源头为海外社媒 Rohan Paul 帖（北京 11-22 02:47:26，`b-fx-2124213113583174076.json`） | 不加引号，不当原话；页面时刻未标时区，按北京理解 |
| 论文首页写明算法由 Claude 发现、作者理解并简化 | L3 | ✅ | PDF 第 1 页（摘要后两段斜体）：“Claude, an AI model developed by Anthropic, discovered the algorithm … The authors then worked to understand, simplify, strengthen, and extend the algorithm” | 作者：Josh Alman（哥伦比亚大学）、Virginia Vassilevska Williams（MIT） |
| 论文称，Anthropic 员工让内部研究模型研究密码学问题，它在一次无人插手的会话里做出了这个算法 | L3 | ✅ | PDF 第 63 页：“An Anthropic employee used an internal research model to investigate open problems in the theory of cryptography. … Claude was tasked with verifying and improving the constructions, but instead developed this algorithm, first for the average case, then for the worst case. The session used 16M output tokens with no human input.” | “无人插手”= no human input，只指那次会话；作者此后理解、简化、加强与扩展（第 1 页） |
| 对你：论文和 Lean 证明代码都免费公开；论文没写程序实现 | L1/L3 | ✅ | arXiv 免费；Lean 仓库 `anthropics/formal-math/3sum-apsp`（Apache-2.0，提交 2026-10-06T04:00:29Z）；B 组对 PDF 全文 grep implementation、timing、practical 类词：无实现、无计时（`evidence-b.md` B4） | 范围限于论文本身（反向核验第 11 条后由“没给可运行的实现”改）；Lean 仓库是形式化证明，不是可调用的算法程序 |
| 吠点①：搜狐号文章写“11月22日……研究人员发表论文” | L5（转载原文存在） | ✅ | `b-sohu.txt`（页面 11-22 04:27）：“11月22日，哥伦比亚大学与麻省理工学院的研究人员发表论文指出，AI系统Claude发现的新算法成功打破了……” | ✅ 只证该站这样写过（反向核验第 9 条）；原吠点①“完全由 Claude 自主完成”按反向核验 BLOCKER 2 改掉：论文写那次会话确实无人输入 |
| 吠点①：论文北京时间10月6日就上了 arXiv，只有这一版 | L3 | ✅ | `b-arxiv-abs.txt`：“[v1] Mon, 5 Oct 2026 17:44:29 UTC (93 KB)”，版本历史只有 v1（北京 11-22 23 时取）；Lean 提交 2026-10-06T04:00:29Z | UTC 日期 10-05 |
| 吠点②：“推翻”是指数上的：一定范围内的整数输入，从2次方降到1.9992次方 | L3 | ✅ | 摘要：“deterministically solve 3SUM on n integers of polynomial size in O(n^{1.9992}) time”；教科书 O(n²) | “一定范围内的整数”= integers of polynomial size（反向核验第 6 条后补）；实数输入另有随机算法界，未写 |
| 吠点②：论文自己写“可能不实用”，常数“巨大” | L3 | ✅ | PDF 第 12 页：“The new algorithms are algebraic and potentially impractical in their current form: the constants hidden in the O(·) are enormous, and the exponents can likely be improved.” | |
| 吠点③：作者之一 Vassilevska Williams 称内部模型“不必要” | L2 类（学者署名公开发言） | ✅ | 个人页 `https://people.csail.mit.edu/virgi/thoughts.txt`（`b-vvw-thoughts.txt`；Claude 重取一致）：“I claim that the internal model was unnecessary. We could have (even quickly!) proven the result without it.” | 页面无日期，首句“A week has passed since Josh and I posted our write-up on the arXiv” |
| 吠点③：她把学生2023年的笔记和两篇论文交给公开版的两个 Claude 模型，连续两条提示、每个只跑一次就各自做出 | L2 类 | ✅ | 同页：“one of my PhD students, Shyan Akmal … I have had his untouched notes since March 2023”；“we were 2 simple prompts to Claude Opus 5.5 or Fable 5.1 away from refuting 3SUM and APSP”；提示词 1 附学生笔记、Coppersmith 1982 年论文与 van den Brand 等人的论文；两条提示各模型均成功（19 分 44 秒 / 27 分 1 秒；30 分 44 秒 / 约 45 分）；“after the first 2 prompts we are actually done due to the known reductions”；“I only ran a single run on Fable 5.1 and Opus 5.5 each.” | “两个模型”= Fable 5.1、Opus 5.5；另跑了可选的第 3 条提示；单次、自述（反向核验第 7 条后改写） |
| 吠点③：不给笔记，每个模型试三次都没成 | L2 类 | ✅ | 同页：“Neither Fable 5.1 nor Opus 5.5 could refute the conjectures without the hint of our approach. I ran three tries each.” | 不给笔记时仍给了两篇论文 |
| 小标题：3SUM 论文：10月就发了 | L3 | ✅ | 同省流 | |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 腾讯 Hy4 开放权重；年营收超5000万美元的模型托管商对外服务须另获许可 | L1 | ✅ | Hugging Face `tencent/Hy4` 模型卡 `c-c-hy4-hf-card.txt`（public；lastModified 北京 11-20 11:05，官方未给发布时刻）：“Hy4 is released under the Tencent Hy License.”；许可 `c-c-hy4-license.txt` 10.2：“if You operate a MaaS business, You must obtain a separate written license … (a) if Your aggregate worldwide gross revenue exceeds US$50 million … in any consecutive 12-month period …; or (b) … more than 100 million monthly active users”；10.3：仅内部使用、不向第三方提供时 10.2 不适用 | 跟进 1103。“对外服务”对应 10.3 内部使用豁免（反向核验第 5 条后补）；(b) 月活条件未写 |
| Anthropic IPO 交割：含超额配售共11.27亿股，每股83美元 | L1 | ✅ | `c-c-anthropic-ipo-close.txt`（Published Nov 20, 2026）：“the closing of its initial public offering of an aggregate of 1,127,000,000 shares of its Class A common stock at a public offering price of $83.00 per share, including the full exercise by the underwriters of their option to purchase an additional 147,000,000 shares” | 1118 速览的 9.8 亿股是定价口径，不混用 |
| 数学家 Sutherland：黎曼猜想 Lean 证明重建，导出文件逐字节一致 | L2 类（MIT 数学家署名页） | ✅ | `c-c-sutherland-grh.txt`：“Andrew Sutherland (MIT), checks run with Claude Code”；“Page last updated 2026-11-22 14:10 UTC.”；“An independent sandboxed lake build of the archive reproduces the export byte for byte.” | Lean 内核整份回放与 lake comparator 仍在跑；主体由“MIT 复核页”改为署名数学家（反向核验第 8 条）；跟进 1120 |
| OpenAI 自家药化基准：GPT-6.2-Astra xhigh 得84.5% | L1 | ✅ | `c-c-openai-medchembench.txt`（November 20, 2026）：“At the xhigh reasoning setting, GPT‑6.2 Astra earned 84.5% of the available credit” | 84.5% 是可得分数的比例，不是答对题数比例；原站另写“These results do not establish whether models can discover new drugs” |
| Runway 自报：10月合并的 PR 中38%由云端智能体驱动，6月为0.1% | L1（自报） | ✅ | `c-c-runway-software-factory.txt`（November 20, 2026）：“Cloud agents drove 38% of the pull requests we merged in October, up from 0.1% in June.” | 分母是合并的 PR 数量（反向核验第 8 条后由“代码改动”改）；图注称是下限 |

未收：Better Data Center Project 估算 2028 年 Anthropic 用电约87%来自天然气（反向核验 BLOCKER 3：40 字内放不下“美国已签约数据中心”与“边际电厂”两个限定，删去；存档 `c-c-bdcp-byop.txt`）、MiniMax MIM 密码分析（官方一手未找到）、Maynard 谈 Alpöge 证明（IT之家所引采访出处未找到）、Gemini 代打电话（Google 页 11-17，在窗口前）、Browser Use 1.0、特斯拉 FSD v15（“25x”无基线，速览写不清限定）。

## 批注译注

| 图 | 原句 | 译注 | 备注 |
|---|---|---|---|
| together-what ① | “As part of routine software testing, we collected prompt samples authorized for that use” | （作为例行软件测试的一部分）我们收集了获授权用于该用途的提示词样本。 | rects |
| together-what ② | “an AI coding agent committed a credential that provided access to the private Hugging Face repository.” | （开发公开仓库 xorl 时）一个 AI 编码助手提交了一份凭证，可访问那个私有的 Hugging Face 仓库。 | rects |
| together-what ③ | “an unauthorized party obtained the credential and used it to download the stored prompt samples.” | 在暴露期间，一个未经授权的人拿到了这份凭证，用它下载了存着的提示词样本。 | rects |
| together-privacy ①–④ | “Turn off Prompt storage.”；“Turning off prompt storage also turns off passthrough models”；“ZDR applies only to requests sent after you enable it.”；“Unless your organization enables ZDR, Together stores the prompts you send and the responses models return” | 见 cards.toml | 深色底；替换原 together-zdr（反向核验第 4 条：删去“ZDR is not enabled by default”单句） |
| or-actions ①② | “Off-boarded three providers …”；“Removed 125 endpoints … precautionary measure.” | 见 cards.toml | 深色底，rects |
| dragos-first-hop ①② | “a privacy flag only binds the first hop”；“verify that your privacy settings actually change where your requests go.” | 见 cards.toml | |
| 3sum-claude、3sum-method、3sum-impractical | 见二节 | 见 cards.toml | 3sum-claude 为第 1 页标题与摘要后两段拼接（视觉复核：整页太小） |
| vvw-claim ①–④ | 见二节 | 见 cards.toml | 560 CSS px 重渲后拼接四段（视觉复核：原图字小） |
| sohu ①② | “11月22日，哥伦比亚大学与麻省理工学院的研究人员发表论文指出”；“整个发现过程完全由Claude自主完成” | 原文照录 | rects |

## 反向核验

第 1 轮（北京 11-22 23:34–23:52）：Codex（gpt-6-astra，effort medium，只读，`barking review --reverse`）；未通过，3 个 BLOCKER。取证方为 Claude 家族（Sonnet、Haiku），本轮同时核对事实清单与存档。

1. BLOCKER：Together 对你一句漏了“关闭提示词存储会同时关闭转发型模型、只对之后的请求生效”。采纳，正文与事件卡补上；批注图改为 together-privacy，标出这两句。
2. BLOCKER：数学吠点①拿“员工布置任务”反驳“完全由 Claude 自主完成”，但论文同时写那次会话无人输入，两者不矛盾。采纳，吠点①改为搜狐号把发表日写成“11月22日”（论文 10-06 已上 arXiv，只有 v1）；事实段补“它在一次无人插手的会话里做出了这个算法”。
3. BLOCKER：能源速览丢了“美国已签约数据中心”范围，主张者写成“机构”。采纳，40 字内放不下两个限定，删去该条（速览剩 7 条，含 2 条主帖）。
4. SHOULD_FIX：批注卡单列“ZDR is not enabled by default”与公告“新账户默认开启”冲突。采纳，换底图，去掉该句。
5. SHOULD_FIX：Hy4 许可漏了 10.3 内部使用豁免。采纳，改“对外服务须另获许可”。
6. SHOULD_FIX：1.9992 漏输入条件。采纳，补“一定范围内的整数输入”。
7. SHOULD_FIX：复现实验压掉“连续两条提示”和“每个模型各三次”。采纳。
8. SHOULD_FIX：“MIT 复核页”提升了背书主体；“代码改动”易读成代码量。采纳，改“数学家 Sutherland”“合并的 PR”。
9. SHOULD_FIX：L4/L5 行标 ✅ 与分级表冲突；部分译注未承接。部分采纳：媒体行备注写明 ✅ 只证“该站这样写过”；新增事实行承接“一次无人插手的会话”；译注逐条原句见“批注译注”节与 cards.toml。L4/L5 夸大实例沿用往期写法（级别列写明“夸大实例/转载原文存在”）。
10. SHOULD_FIX：images/README.md 缺失、sources/README.md 无来源入口。images/README.md 在复核开始后写成（复核时未见）；sources/README.md 已补主帖与速览来源。
11. NEEDS_VERIFICATION：“前一天”无时区；“没给可运行的实现”否定范围过宽。采纳，改“11月21日”与“论文没写程序实现”。

第 2 轮短审（北京 11-23 00 时前后，同一配置）：PASS，无 BLOCKER。
1. SHOULD_FIX：事件卡与正文事实句漏“含预防性撤下的”。采纳。
2. SHOULD_FIX：Sutherland 速览“逐字节一致”漏比较对象（导出文件一致；编译模块 175,186/175,187 相同）。采纳，改“沙箱重建，导出文件逐字节一致”。
3. SHOULD_FIX：吠点③成功组每个模型只跑一次的样本数未写。采纳，补“每个只跑一次”。
4. NICE_TO_HAVE：配图说明 04 行吠点条数。采纳。

## 视觉复核

第 1 轮（北京 11-22 23:52 前后）：Codex（gpt-6-astra，effort medium，只读，`barking review --visual`，26 张图）；PASS，无 BLOCKER。

- SHOULD_FIX：05-together-what 高亮①多涂了“and”、漏了“As part of routine software testing,”。采纳，像素框改为从行首到“use”。
- SHOULD_FIX：11 号论文第 1 页整页缩入，英文太小。采纳，改为标题与摘要后两段拼接（3sum-claude）。
- SHOULD_FIX：14 号 Vassilevska Williams 页等宽字偏小。采纳，按 560 CSS px 重渲、拼接四段（vvw-claim）。
- NICE_TO_HAVE：“10月就发了”带纠偏语气，建议省流与速览改“10月已公开”。不采纳：省流按规范可带本条核心吠点判断，小标题沿用。
