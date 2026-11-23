# 1122 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B 组 Sonnet 5.5（`claude-sonnet-5-5`），C 组 Haiku 5.5（`claude-haiku-5-5`）；用时（harness）A 28 分钟、B 27 分钟、C 10 分钟 + 补充轮 2 分钟，未催交。Claude 抽查：Lean 声明原句与署名改动（三份 diff、PR #154 提交说明）、Clay RH 页状态位与规则原句、Sutherland 页（Claude 北京 00:05 自取）、MiniMax 论文摘要与 Table 1、ARADI README、DRS 原句、Blender 提交 diff 与两条 issue 回复（Claude 北京 00:40 自取）、新智元与新周刊原句、OpenAI 状态 JSON（Claude 北京 00:42 自取）。

## 一、黎曼猜想证明获 Lean 机构认证（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：1700万行代码证黎曼猜想，百万奖还没发 | L1/L3 | ✅ | 声明“a Lean library comprising 17 million lines of code”；“证”= 两家机构认证证明正确（下行）；“百万奖还没发”：Clay 规则 PDF §2“$1 million allocated to each Problem”、RH 页状态“Active”、新闻页无条目。另： Lean 声明 `a-lean-blog.txt`：“We hereby certify that both the statement and the proof of the Riemann Hypothesis are correct.”；Clay 规则 `a-clay-rules.txt`、Clay RH 页 `a-clay-rh2.txt` 状态位“Active”（同站庞加莱猜想页同一位置为“Solved”，`a-clay-poincare.txt`）；Clay 新闻页 `a-clay-news.txt` 无 RH 条目 | “百万美元”：Clay 规则 PDF §2“$1 million allocated to each Problem”（`a-clay-rules-pdf.txt`）。“还没发”= 未颁奖，Clay 未就本事件表态。“Lean认证”指 Lean FRO 与 Mathlib Initiative 的声明 |
| AI 参与的黎曼猜想机器证明获 Lean 两家官方机构认证（省流） | L1/L3 | ✅ | 同上；`a-remark-pdf.txt`：“I believe I have, working with Claude, discovered a proof” | 两家机构见下行 |
| 百万美元奖还要在认可刊物发表后再等至少两年（省流） | L1 | ✅ | `a-clay-rules.txt`：“(i) the proposed solution must be published in a Qualifying Outlet (see §6), and (ii) at least two years must have passed since publication, and (iii) the proposed solution must have received general acceptance in the global mathematics community” | “认可刊物”= Qualifying Outlet（规则 PDF §6：合格出版渠道，含特别批准）；省流只写前两条，第三条进吠点② |
| 黎曼猜想是关于素数分布的百年难题 | L1 | ✅ | Clay RH 页 `a-clay-rh2.txt`：“The prime number theorem determines the average distribution of the primes. The Riemann hypothesis tells us about the deviation from the average. Formulated in Riemann's 1859 paper” | 1859 年至今 167 年，写“百年难题” |
| Lean 是让电脑逐步查证明的软件 | — | ✅ | 常识性解释；Xena `a-xena.txt` 与 Lean 声明对检查流程的描述 | 通俗说法，不改变结论 |
| 11月20日，Anthropic 员工 Alpöge 公布与 Claude 合作的黎曼猜想证明 | L2 | ✅ | `a-remark-pdf.txt` 第 6 行（见上）；员工身份：`a-alpoge-home.txt` 第 3、21 行；Lean 声明：“On November 20, 2026, Levent Alpöge released a proof of the Riemann Hypothesis as a Lean library” | remark.pdf SHA-256 与 1120 存档相同（A 组）；“Anthropic 员工”沿用 1120 期 ✅ 行 |
| 只给了约1700万行 Lean 代码 | L2/L3 | ✅ | `a-remark-pdf.txt` 第 82 行：“I decided to only present Lean.”；Lean 声明：“a Lean library comprising 17 million lines of code” | |
| 11月21日，Lean FRO 和 Mathlib Initiative 两家官方机构发声明，“认证”命题与证明都对 | L1/L3 | ✅ | `a-lean-blog.txt`：页面时间“2026-11-21 13:55”（HTML meta 与 RSS 为 UTC，即北京 21:55；仓库合并 14:26:03Z，北京 22:26）；“Joint statement by the Lean FRO and the Mathlib Initiative.”；“We hereby certify that both the statement and the proof of the Riemann Hypothesis are correct.” | 机构：Lean FRO“Since its formation in July 2023 as a non-profit organization … to improve Lean's critical systems”（`a-lean-fro-about.txt`），Mathlib Initiative 支持 Mathlib（`a-mathlib-init.txt`）。“官方机构”指负责 Lean 与 Mathlib 的两家组织。署名改动见“未进正文” |
| MIT 复核页显示，Lean 自带检查程序随后也查完整份、通过 | L2 | ✅ | `a-sutherland.txt`（last updated 2026-11-23 14:35 UTC，Claude 北京 00:05 自取同版）：“Lean default kernel (leanchecker --from-export, Lean v4.36.0) on the full export — accepts — exit 0 at 18:44:18 UTC on 22 November; 48 h 3 min on one thread” | 北京 11-23 02:44。1120 期时该项为 running（`a-sutherland-diff-1120-to-1122.txt`）。默认设置的 paranoid 内核与 lake comparator 仍 running，正文未写 |
| 对你：代码公开，复核页写自己复核约需 100GB 内存、70GB 硬盘 | L2 | ✅ | `a-sutherland.txt` 第 192 行：“With Lean v4.36.0 installed, nothing needs to be built. You need roughly 100 GB of RAM and 70 GB of disk.” | 页面“Check it yourself”一节；“代码公开”：Lean 库随 remark.pdf 公开发布（1120 期 ✅），导出文件公开下载（Sutherland 页）；48 h、82.6 GiB 是 Sutherland 一次运行的数字，正文未写（批注译注写了 48 小时 3 分钟，见“批注译注”） |
| 吠点①：认证说的是这份机器证明每步都查过、证的确是黎曼猜想 | L1/L3 | ✅ | `a-lean-blog.txt` 四条：“The statement of the Riemann Hypothesis is correct. This statement is part of Mathlib: RiemannHypothesis”；“The code proves the correct statement … exactly Mathlib's statement of the Riemann Hypothesis”；“The Lean proof is valid.” | 佐证（未进正文，L2/L6）：Lean Zulip 上 Loeffler“it makes a declaration about formalization, not about mathematics at large”（`a-zulip-excerpts.md`，自称不代表 FRO/MI）；Xena 文末：Lean 通过“does not mean that any corresponding pdf also contains a correct proof” |
| 吠点①：新思路是什么，声明说等数学界解释 | L1/L3 | ✅ | `a-lean-blog.txt` 第 48 行：“We look forward to all the activity in the mathematical community that will help explain the new ideas in this proof.” | |
| 吠点②：新周刊文章题为“悬赏百万美元的难题先被 AI 破解” | L5（自媒体转载） | ✅（只证引文存在；其主张即本条要纠正的说法） | 投资界转载 `a-zh-pedaily.txt` 第 39 行：“悬赏百万美元的难题先被AI破解，全世界的数学家都睡不着了”（更新于 2026-11-23 17:33；文末注来源微信公众号“新周刊”）；凤凰网同文 `a-zh-ifeng.txt` | 引号内按 lint 加了中英文空格；正文写“新周刊文章”，平台名不写 |
| 吠点②：克雷规定证明在认可刊物发表满两年、获普遍接受才考虑颁奖 | L1 | ✅ | `a-clay-rules.txt`（见上）：“Before CMI will consider a proposed solution, all three of the following conditions must be satisfied” | “才会考虑”= “Before CMI will consider a proposed solution” |
| 吠点②：官网仍未标“已解决” | L1 | ✅ | `a-clay-rh2.txt` 第 103 行状态位“Active”；`a-clay-poincare.txt` 第 103 行同一位置为“Solved” | Claude 核对两页同一面包屑下的状态位 |
| 小标题：Lean 机构认证黎曼猜想证明 | L1/L3 | ✅ | 同上 | |

未进正文（A 组，存档备查）：
- 声明署名改过：PR #143 合并（北京 11-21 22:26）时署“Lean and Mathlib communities, endorsed by the Lean FRO and the Mathlib Initiative”；PR #149（11-21 16:41Z）改为两家机构，PR #150（17:36Z）回退；PR #154（11-23 09:38Z，北京 17:38）再改为“Lean FRO and Mathlib Initiative”，提交说明“The Mathlib maintainers have changed the byline to avoid attribution to the Lean and Mathlib communities as a whole.”（`a-gh-leanblog-diff-*.diff`、`a-gh-leanblog-pr154.json`，Claude 读 diff 核对）。正文“Over the past day, the Lean and Mathlib communities have rallied”未改（Claude 北京 00:50 前后现取）。初稿曾作吠点，因字数删去，只记于此；“社区认证”在中文稿写出时与页面一致，不算写错。
- Tao 11-20 博文：“I imagine the digestion process will take some time”（`a-tao.txt`）。
- Quanta：Radziwiłł 称四页说明含“nonsensical statements”，发在声明之前。
- 声明第④条“Lean's official kernel”与 Sutherland 页当时状态的时间差：A 组推断，不写。
- GRH 只指 Dirichlet L 函数：1120 期已写。

## 二、MiniMax 密码分析（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| MiniMax 称 AI 破解多个密码方案，原文写明不影响在用标准（省流） | L1 | ✅ | `b-blog.txt`：“MIM produced the first practical breaks against multiple candidate algorithms that previously had no known practical attacks”；“This work focuses on academic designs, past competition candidates, and public challenge parameters, and does not affect deployed cryptographic standards.” | “在用标准”= deployed cryptographic standards |
| 11月20日，MiniMax 称，内部模型 MIM 对美国 NIST 往届密码竞赛的多个候选方案做出可实际运行的破解 | L1 | ✅ | `b-blog.html` JSON-LD datePublished 2026-11-20T06:41:45Z（北京 14:41:45）；“across historical NIST cryptographic standardization competitions, MIM produced the first practical breaks against multiple candidate algorithms”；未发布：“latest internal model”，测试资格需申请 | “未发布”= 内部模型，申请邮箱见 evidence-B §7；“可实际运行的破解”= practical breaks |
| 还在一项公开挑战中解出55道此前无公开解的题 | L1 | ✅ | `b-blog.txt`：“it recovered secret keys for 94 instances, including 55 previously unsolved hard instances”；论文 `b-pdf-tii.txt` 摘要：“achieving 55 first-blood recoveries” | 挑战为 TII McEliece Track 2；“首解”= 此前无公开解（first-blood）。94 个中另 39 个是已有解的实例再解 |
| 博客末尾写明，只涉及学术设计、往届候选和公开挑战，不影响已部署的密码标准 | L1 | ✅ | `b-blog.txt` Conclusion（见上） | 截图 14 |
| 11月23日，新智元稿题为“中国 AI 刷爆全球纪录！比 Claude 快四万倍” | L5（自媒体转载） | ✅（只证引文存在；其主张即本条要纠正的说法） | 新浪财经转载 `b-cn-sina.txt` 第 73 行（2026-11-23 07:02）；网易号同稿见截图 23（`b-cn-163-claude.txt` 第 251–253 行实为财闻稿，新智元标题只在第 354–356 行推荐区） | 原题后半“，多个密码学难题被破解”未引。引号内加了中英文空格。Develata 11-24 拍板：标题与封面不写“比 Claude 快”，正文可引 |
| 对你：网银、聊天加密用的现行标准不在这次破解之列 | L1 | ✅ | `b-blog.txt` Conclusion；B 组 B5：涉及方案均不在 FIPS 203/204/205（`b-nist-fips20*.txt`），博客对象中无 AES、RSA、TLS | MiniMax 博客原文没有 AES/RSA 字样（新智元“注”与 IT之家“官方特别说明”所列名单在英文博客中没有，见 evidence-B §5）；本句只说“现行标准不在攻击之列”，不列算法名 |
| 吠点①：新智元写“比 Anthropic 的多 GPU 集群快了43400倍” | L5 | ✅（只证引文存在） | `b-cn-sina.txt` 第 121 行：“它比 Anthropic 的多GPU集群快了 43400 倍！” | 引号内调整了空格 |
| 吠点①：MiniMax 博客也写多 GPU | L1 | ✅ | `b-blog.txt`：“a 6,180x to 43,400x speedup over both the CRYPTO 2026 record (9.20 hours) and Anthropic's prior multi-GPU cluster solver” | 截图 15 |
| 吠点①：但论文写明，仅一道题：Anthropic 研究员用32线程 CPU 跑了21.7小时，MiniMax 用64核服务器跑了1.80秒 | L1 | ✅ | `b-pdf-tii.txt` 摘要：“On TII-254 and TII-253, it finishes in 30.4 s and 1.80 s, achieving 1,896× and 43,400× wall-time speedups over the prior GPU-cluster record and the multi-core hold-out record by Anthropic.”；Table 1：“253 … 21.7 h [39] 694.4 th (32 threads) 1.80 s … 43,400× time”；正文：“Recovering TII-253 (214, 8, 9) took 21.7 h on 32 Xeon threads by Weis at Anthropic [39]” | 截图 24。基线硬件只据 MiniMax 论文，Weis 原文 PDF 被 Cloudflare 拦未读（evidence-B）。“一道题”= TII-253 一个实例；对 GPU 纪录（Vedenev，H200）同题为 28,000×，正文未写、批注译注写了。64 核：摘要“on a single 64-core CPU server”，表注“64 Xeon cores for Route A”。“1.80 秒”为墙钟时间（wall-time）。不对基线是否用 Claude 作任何断言 |
| 吠点②：ARADI 密码共16轮运算，新智元写 MIM“全面刷新了理论攻击纪录” | L1/L5 | ✅（引文存在） | 16 轮：README `b-gh-readme-ARADI.md`“ARADI is a 16-round low-latency block cipher”；`b-cn-sina.txt` 第 253 行：“直接将实用密钥恢复推进到了第 8 轮（耗时仅 2.4～16 秒），全面刷新了理论攻击纪录。” | |
| 吠点②：MiniMax 仓库写理论攻击到12轮、实用破解到8轮，完整16轮“仍未被破” | L1 | ✅ | `b-gh-readme-ARADI.md` 第 12 行：“This enables 256-bit master-key recovery across 5 to 12 rounds. On a single CPU core, we achieve practical master-key recovery up to 8 rounds … and improve all-key theoretical attacks on 10 to 12 rounds. Full 16-round ARADI remains unbroken.”；博客：“extended practical key recovery to 8 rounds … and set new theoretical records for 9 to 12 rounds” | 反向核验第 1 轮 SHOULD_FIX：理论纪录改进与 16 轮未破可同时成立；正文据此改为并列给出理论 12 轮、实用 8 轮、完整 16 轮未破，只纠正“全面” |
| 吠点③：博客列的 NIST 候选破解里，有几条是官方示例代码的漏洞，如 DRS 把 && 误写成 & | L1 | ✅ | `b-blog.txt` 第 100 行：“A verification bug in the official code—logical AND (&&) mistyped as bitwise AND (&)—had gone unnoticed in earlier reviews.”；同节 RQC、LAC、KCL、LEDAkem、ISAP 等条亦为“reference implementations”里的缺陷 | “几条”：DRS、RQC-128、LAC-128、KCL、LEDAkem、ISAP 等，均在博客第 1 节“Breaking NIST candidates”；“官方示例代码”= reference implementations / official code；“误写”只用于 DRS（mistyped），其余为采样泄漏、未初始化内存、种子截断等，统称“漏洞” |
| 吠点③：这些方案都已落选 | L1 | ✅ | NIST：`b-nist-pqc-r2.txt`、`b-nist-ir8309.txt`（ROLLO、RQC、LAC、LEDAcrypt 2020 年未进第三轮）；`b-nist-pqc-r1.txt`（DRS、KCL、QC-MDPC KEM 只在第一轮）；轻量密码只选 Ascon（`x-x-nist-lwc-sel.txt`“NIST Selects Ascon”，`x-x-nist-ir8454.txt` NIST IR 8454；ISAP 为决赛未选）；`b-nist-pqc-sel.txt` 已选算法中无上述方案 | Shamash、REMUS-N1、ForkAE 为轻量密码早期轮次候选，未选 |
| 小标题：MiniMax 破密码：在用标准不受影响 | L1 | ✅ | 同省流 | |

未进正文（B 组，存档备查）：
- Blender 论文被改：提交 `640514fd`“fix Blender issue”（2026-11-23T16:11:40Z，北京 11-24 00:11:40）去掉“provably minimal”“Global-Minimum”字样并改第 3.3 节；两条 GitHub issue 北京 00:22–00:23 由协作者回复后关闭：#1“We’ve removed the “global minimum” wording since "minimum" is not the focus.”，#2“We’ve fixed the entry condition statement in Section 3.3.”（`b-gh-issue-replies.md`，Claude 自取）。
- 博客与仓库数字口径不同：Blender 博客“0.36 to 2.3 seconds on a single core”，论文与 README 为 8 线程 0.045–0.73 s；HAWK 23.8 bit 对 23.74；LOCKER“all parameter sets in 2 seconds”与 README 不全对应；CryptanalysisBench 博客“NeurIPS 2026、135 + 7”对论文 v2“136 + 6”、无 NeurIPS 字样。配置不同，不算矛盾，正文不写。
- “55 个首解后约 74 个仍无解”是 B 组按论文数字推算，未逐项核，且 Machinefi Labs 预印本另报新解，不写。
- 新智元稿称“Anthropic 工程师亲自下场，火速转发点赞”：内嵌社媒帖未核，不写。
- “央广网 11-23 发稿”：未找到。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 智谱 GLM-5.4 先向编程套餐用户开放，官方称智能达到2-3T级模型 | L1 | ✅ | 智谱公众号 `c-c1-wx-glm54.txt`（publish_time 在 `c-c1-wx-glm54.html` 元数据，北京 11-23 17:40:42）：“即日起，GLM-5.4向GLM Coding Plan全量用户开放……API将在完成安全评估后开放，模型权重计划以负责任的方式开源。”；“智能水平达到2-3T级模型”；Z.ai 博客 `c-c1-zai-glm54.txt`：“reaching intelligence levels comparable to 2T-3T parameter models”；“API (Coming Soon)” | “编程套餐”= GLM Coding Plan；“官方称”= 自述，正文无对应跑分表。HN 652 分 / 238 评 |
| 11月23日17:39至17:56，多款 Claude 模型报错率升高 | L1 | ✅ | `c-c3-claude-api-incidents.json`、`c-c3-claude-incident.txt`：“Elevated errors for multiple models”，created 09:49:22Z、resolved 11:07:41Z；identified 更新：“elevated errors on requests to Claude Mythos 5.2, Claude Fable 5.2, Claude Opus 5.5, Claude Sonnet 5.5, Claude Haiku 5.5, and other Claude models” | 时段取解决更新原句“Impact occurred from 01:39 PT / 09:39 UTC to 01:56 PT / 09:56 UTC.”（`c-c3-claude-incident.txt` 第 360 行，北京 17:39–17:56）；开单 17:49:22、标为解决 19:07:41 只记于此。反向核验第 1 轮 BLOCKER（初稿误用开单至结单时刻），已改。impact major |
| 付费版 ChatGPT 的 GPT-6 对话间歇报错，24日0:38宣布恢复 | L1 | ✅ | `c-c6b-openai-incidents-0042.json`（Claude 北京 00:42 取）：“Elevated errors with GPT-6 conversations on paid ChatGPT plans”，created 15:50:14Z，resolved 16:38:46Z；“intermittent errors affecting GPT-6 conversations on paid ChatGPT plans”；“All affected services are operating normally again.” | impact minor；resolved_at 16:38:46Z = 北京 24 日 00:38:46；首次通报 15:50:14Z（北京 23:50）不一定是故障起点，不写持续时长（反向核验第 1 轮） |
| OpenAI 首席科学家提议按困惑度等设上限、监控能力达标前不越过，未给数值 | L1 | ✅ | `c-c4-openai-perplexity.txt`（页面 November 22, 2026，官方未给时刻）：“I further propose common limits, based on perplexity and other measurements, that developers are committed to not crossing until our ability to understand and monitor AI behavior achieves a certain concrete milestone” | “监控能力达标前不越过”= “committed to not crossing until our ability to understand and monitor AI behavior achieves a certain concrete milestone”（反向核验第 2 轮补条件）；“未给数值”= 文中无上限阈值与时间表（C 组通读）；文中公开了 OpenAI 内部数值（内部代码库 held-out 集），不是上限。HN 653 / 425。“微软公布可比数据”未找到，不写 |
| AI 智能体集体攻 P vs NP：网站称0时已查738个引理，结论未定 | L1（站点自述） | ✅ | `c-c5-pnp-about.txt`：“A fleet of AI agents, run by Brilliant, works the hardest open problem in computer science in the open. Lean checks every claim.”；`c-c5-pnp-dashboard-0012.txt`（opencli，北京 00:12–00:14）：“738 of 1,306 lemmas checked”；“Verdict: open, being worked on now.” | 网站为 pequalnp.com；“0时”= 北京 11-24 00:12–00:14 快照；“已查”= checked（站点称由 Lean 检查）；“结论未定”= verdict open；“集体攻”= a fleet of AI agents |
| 主帖：Lean 机构认证黎曼猜想证明 / MiniMax 破密码：在用标准不受影响 | — | ✅ | 见上两节 | 速览前两条为主帖小标题 |

备选（未用）：斯坦福 AI100 2026 报告（L1，新闻性弱，`c-c6-stanford-ai100-redacted.txt`）。不收：OpenAI“211 分钟故障”（扫描所据页面是 2023-11-21 事故，Claude 核页面时间戳确认，❌）；ARTEX（美方机构点名“中国黑客”，风控）；Jensen Huang 复职（风控）。

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 我们在此认证：黎曼猜想的命题与证明都是正确的。（lean-certify） | L1/L3 | ✅ | `a-lean-blog.txt`：“We hereby certify that both the statement and the proof of the Riemann Hypothesis are correct.” | 直译 |
| 我们期待数学界的各种工作，帮助解释这份证明里的新想法。（lean-closing） | L1/L3 | ✅ | `a-lean-blog.txt` 第 48 行 | 直译 |
| Lean 默认内核检查整份导出证明：接受；北京时间11月23日 02:44:18 结束，单线程 48 小时 3 分钟。（mit-kernel ①） | L2 | ✅ | `a-sutherland.txt` 第 35 行：“accepts — exit 0 at 18:44:18 UTC on 22 November; 48 h 3 min on one thread” | UTC 18:44:18 换北京 11-23 02:44:18 |
| lake comparator（Lean v4.36.0）端到端比对：仍在运行。（mit-kernel ②） | L2 | ✅ | `a-sutherland.txt` 第 45 行：“lake comparator (Lean v4.36.0) end to end — running” | 页面 last updated 2026-11-23 14:35 UTC |
| （须满足）自发表起至少已过两年。（clay-rules） | L1 | ✅ | `a-clay-rules.txt` 第 113 行：“at least two years must have passed since publication” | |
| 这项工作针对学术设计、往届竞赛候选方案和公开挑战参数，不影响已部署的密码标准。（minimax-conclusion） | L1 | ✅ | `b-blog.txt` Conclusion | 直译 |
| 比 CRYPTO 2026 的纪录（9.20 小时）和 Anthropic 此前的多 GPU 集群求解器都快 6,180 到 43,400 倍。（minimax-gpu-claim） | L1（MiniMax 自报） | ✅ | `b-blog.txt`：“a 6,180x to 43,400x speedup over both the CRYPTO 2026 record (9.20 hours) and Anthropic's prior multi-GPU cluster solver.” | 博客原话；与其论文描述不一致，见吠点① |
| TII-253：此前纪录 21.7 小时（文献 [39]，32 线程），本文 1.80 秒（64 核），按用时快 43,400 倍。（tii-table ①） | L1（MiniMax 自报） | ✅ | `b-pdf-tii.txt` 第 121 行（Table 1）；表注“64 Xeon cores for Route A”（底图 `tii-table-raw.png` = `27-tii-table-key-rows.png`，由 `24-tii-paper-table1.png` 裁出表头、两行 TII-253 与表注，像素未改）；正文“took 21.7 h on 32 Xeon threads by Weis at Anthropic [39]” | 基线硬件只据 MiniMax 论文 |
| 同一题对比此前 GPU 纪录（文献 [38]，1 块 H200 加 48 线程，14.0 小时）：快 28,000 倍（CPU 对 GPU）。（tii-table ②） | L1（MiniMax 自报） | ✅ | `b-pdf-tii.txt` 第 122 行：“253 … 14.0 h [38] 1×H200 + 48 threads 1.80 s … 28,000× time (CPU vs. GPU)” | |
| 在单个 CPU 核上，用 4 至 90,112 个选择明文、不超过 16 秒，对最多 8 轮实现了实用的主密钥恢复。（aradi-unbroken ①） | L1 | ✅ | `b-gh-readme-ARADI.md` 第 12 行：“On a single CPU core, we achieve practical master-key recovery up to 8 rounds in ≤16 s using 4 to 90,112 chosen plaintexts.” | 反向核验第 1 轮补“选择明文” |
| 完整 16 轮的 ARADI 仍未被破。（aradi-unbroken ②） | L1 | ✅ | 同上：“Full 16-round ARADI remains unbroken.” | |

## 反向核验与视觉复核

第 1 轮反向核验（Codex gpt-6-astra，effort high，只读；北京 00:53:51 读取的初稿，即读者稿检查前的版本）：1 BLOCKER、11 SHOULD_FIX、1 NICE_TO_HAVE、1 NEEDS_VERIFICATION。处理：
1. BLOCKER Claude 故障时段：采纳，速览改为状态页写明的影响窗口 17:39–17:56。
2. ARADI 对比指标不同：采纳，吠点②改为并列理论 12 轮、实用 8 轮、完整 16 轮未破，只纠正“全面”。
3. 43,400 倍缺另一端：采纳，补 MiniMax 64 核 1.80 秒；表格译注补“64 核”。
4. 克雷“发表”须为合格渠道：采纳，省流与吠点改“认可刊物”。
5. “笔误”泛化：采纳，统称“漏洞”，只对 DRS 写“误写”。
6. ChatGPT 48 分钟：采纳，速览改为“24日0:38宣布恢复”，不写时长。
7. 译注事实未入清单：采纳，补“批注译注”一节，删去“未写入”的过时说法。
8. ARADI 译注漏“选择明文”：采纳。
9. 媒体引文等级：采纳，自媒体转载定 L5，结果注“只证引文存在”。
10. 轻量密码落选缺存档：采纳，补存 NIST“Selects Ascon”公告与 IR 8454 页（`x-x-nist-*`）。
11. 译注用 UTC：采纳，改北京时间。
12. 来源指针：采纳（Alpöge 员工身份、JSON-LD 与微信元数据在 html、网易存档行号）。
13. “美国 NIST”涉外国政府机构：交 Develata 终审判断；正文是技术竞赛名，不涉政府点名指控。

第 2 轮（同一提示词，北京约 01:00 完成；读到的正文已是读者稿检查后版本，事实清单仍是第 1 轮前的旧版）：1 BLOCKER、6 SHOULD_FIX、1 NEEDS_VERIFICATION。BLOCKER（Claude 时段）与 SHOULD_FIX 2、3、6、7、8 与第 1 轮重合，已随第 1 轮处理同步进清单。新增：
- SHOULD_FIX 4（困惑度提案丢了“监控能力达标前”条件）：采纳，速览改为“提议按困惑度等设上限、监控能力达标前不越过，未给数值”。
- SHOULD_FIX 5（L4 不应标 ✅）：部分采纳。转载稿改定 L5；“X 写了某句”这一事实由存档证明，沿用 1120、1121 期写法标 ✅ 并注“只证引文存在”，其所述主张即本期纠正对象。

视觉复核（Codex gpt-6-astra，effort high，24 张图）：PASS，无 BLOCKER；6 条 SHOULD_FIX 全部采纳：ARADI ① 高亮扩到“On a single CPU core”至“chosen plaintexts”、译注补明文数量与 16 秒；ARADI ② 高亮扩到“Full”；MIT ② 补标下一行“end”；TII 表改用只含表头、两行 TII-253 与表注的裁切底图（字放大约三成，② 编号贴近目标行），① 加标表注“64 Xeon cores for Route A”。改后 Claude 逐张目视复查。
