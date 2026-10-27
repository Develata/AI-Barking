# A 组取证：OpenAI 与 Anthropic 同日称推翻 ETH（1026 期，跟进 1006/1008）

执行方 claude-sonnet-5-5。取证时段北京时间 2026-10-27 22:53–23:50。时间一律北京时间（UTC+8），括号保留原时区；官方只给日期时写“官方未给时刻”。本机美东（EDT），北京 = 本机 + 12 小时。数学题：只记录谁在何时声称了什么、形式化覆盖了什么、谁在审，不判断证明对错。

存档在 `docs/2610/1026/sources/`，前缀 `a-`（html、pdf、txt、headers、json、fetch-log、shots-log、heat）；截图在 `docs/2610/1026/images/01–12`。HTML、PDF、截图只存本地（不入库）。

## 0. 先看这几条（与派工单“已知情况”相比的新发现）

1. **先后：OpenAI 先，Anthropic 晚约 5.5 至 7.5 小时；WorkBuddy 说 Anthropic“更早公开”是错的。** OpenAI：GitHub 仓库提交 `42907c9` 2026-10-27 04:52:18（20:52:18Z，10-26），PR #3 合并 05:18:03；博文 HN 提交 05:35:28、Ryan Williams 开帖 05:35:29、Dan Roberts 开帖 05:36:57。Anthropic：GitHub 提交 `16a5ce4` 11:07:21（03:07:21Z，10-27），Hexagon 记录发布 13:12:41（05:12:41Z）。Anthropic 论文自己引用了 OpenAI 的预印本，并写“Independently and concurrently, an internal model at OpenAI claims to have proven a similar result”。两家各自都说核心发现在 10 月 16 日。我没找到 Anthropic 官网新闻页、X 帖上的 ETH 公告（官网新闻页 22:5x 抓取时无该条）。
2. **两篇论文的时间界是同一个：** OpenAI 定理 1.1 `(n+m)^{O_k(1)} 2^{O_k(n log log n/√log n)}`（稀疏 m≤Dn 时 `2^{O(n/√log n)}`）；Anthropic 主定理 `2^{O(min{k² n log log n/√log n, km/√log m})}`（脚注：稀疏输入 `2^{O(kn/√log n)}`）。**但 OpenAI 的 Lean 只证了更弱的 `(log log n)²` 版：** `lean/docs/378.md` 写 `(L+1)^{O_k(1)}2^{O_k(n(log log n)²/√log n)}`，并写明 “the October 25 manuscript gives the sharper time bound”。Anthropic 的 Lean 主定理陈述用 `k² n log log n/√log n` 与 `km/√log m` 的最小值。
3. **Anthropic 论文有大量限定词，博文/媒体摘要多半没带：** “purely asymptotic”（m=5n 时要 n > 10^10000 才优于穷举；只按 n 的界要 n > 10^(10^8)）；Lean 开发“written by AI models and reviewed by other instances of the same models; at the time of writing no person is known to have checked it”；“no machine checks that this file says what the paper says”。OpenAI 论文我没找到“实用性”或“人类参与”说明（仓库 README 称 “an unnamed internal OpenAI model”）。
4. **AGMAI 不是“OpenAI 自己组建的”：** AGMAI About 原句 “This group operates independently of any AI company and members do not accept payment for this work”，同页还写 “This group came together after OpenAI approached some of its members about establishing an external advisory board. In agreement with OpenAI, they decided to instead create an independent group and invite the others to join.” OpenAI 9 月 21 日自家文章同样写 “independent mathematics advisory group”“The group will operate independently from OpenAI … Its members will not be paid by OpenAI”，并写该组 “hosted at the Institute for Advanced Study”（IAS Nelson Center 项目页确认，PI 为 De Lellis 与 Witten）。但 Scientific American 10-27 的报道把它写成 “the expert panel OpenAI organized in response to widespread criticism from mathematicians”——这是媒体写歪的实例。1006 期提到的“IAS 顾问组”即 AGMAI。成员共 9 人，含 Witten（WorkBuddy 对，Claude 只读到 5 位）。
5. **Impagliazzo–Paturi–Williams 的公开信（Google 文档）现在列有 “5. Anthropic: Refutation of ETH”**，即该组已把 Anthropic 的论文列为资源；信的原话 “Our involvement should not be taken as certification of the claim”。信是可变文档，无时间戳；我 23:3x 抓到的版本含 Anthropic 一行，ethdiscussion.blog 上 10-26 的帖子版本不含这一行。

## 1. 事实清单

级别按 EDITORIAL 事实分级。专家本人的公开发言（X、博客评论、公开信）不在 L1–L7 现成条目里，暂按 L6 记，是否升级由 Claude 定。状态列只用四个词，附加说明在“条件/口径/时区”列。

### A1 OpenAI 博文

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1a 博文标题与日期 | L1 | https://openai.com/index/building-scientific-understanding-with-academic-communities/ | “Working with academic communities to build scientific understanding”；“October 26, 2026”；“Research Publication”；作者 “OpenAI” | 01 | 官方未给时刻。可推：GitHub 提交 20:52Z、PR 合并 21:18Z、HN 提交 21:35:28Z（10-27 05:35:28 北京）、Williams 帖 21:35:29Z。页面 HN 标题为 “Three new discoveries in mathematics, computer science, and physics”，页面 `<title>` 即上面这条 | 已找到 |
| A1b 三项成果与“原始研究数据”定位 | L1 | 同上 | “We are announcing three new discoveries in mathematics, computer science, and physics about harmonic analysis, complexity theory, and quantum field theory.”；“In our view, these manuscripts are best understood as a form of raw research data.” | 01 | 外部专家被称 “shepherds”；三条路线：数学“release the AI output”，CS“form an external group … while making the AI output publicly available”，物理“form an external group … and defer to them on the release of materials” | 已找到 |
| A1c 四条原则（不签 NDA、不要学术署名、无偿） | L1 | 同上 | “Participants will not be asked to sign nondisclosure agreements”；“We do not seek academic credit for the discoveries produced by our internal models.”；“Participation is voluntary and unpaid. Working-group members participate independently, and their conclusions are their own. They do not speak on behalf of OpenAI, and OpenAI does not speak on their behalf.” | 02 | 指的是各“外部合作组”，不是 AGMAI | 已找到 |
| A1d 指定数学家造成信息不对称（OpenAI 自己的说法） | L1 | 同上 | “On the other hand, choosing initial shepherds necessarily shapes who gets involved first and creates asymmetry of information. We welcome feedback on how to broaden participation and make this process more useful.” | 02 | 只写“选 shepherd 造成信息不对称”，未点名 AGMAI；AGMAI 10-25 声明是对这一做法的批评（见 A8） | 已找到 |
| A1e ETH 一节：原话、验证、谁在审 | L1 | 同上 | “Surprisingly, our system has recently proved that the ETH is false, a result that was subsequently verified in Lean.”；“This work will be verified and shepherded by a group led by Russell Impagliazzo (UC San Diego), Ramamohan Paturi (UC San Diego), and Ryan Williams (MIT).”；“Finding such algorithms, and turning theoretical advances into practical improvements, will require significant further research.”；“We have agreed to the group’s requests that all raw data of the results be made public … and that OpenAI commit to funding a conference” | 03 | “verified in Lean”是 OpenAI 自述；“will be verified”指外部组尚未验完 | 已找到 |
| A1f ETH 发现时间 | L1 | 同上 | “Our AI found its natural language disproof for the ETH on October 16th, 2026 and then subsequently we formalized the result in Lean on October 17th, 2026” | 03 | 无时区 | 已找到 |
| A1g 局部光滑/Kakeya 一节：原话与时间 | L1 | 同上 | “Our models produced a proof of the local smoothing conjecture in every dimension and therefore also resolve the Kakeya conjecture and Stein’s restriction conjecture in all dimensions. The new proof is based on a novel auxiliary partial differential equation.”；“We obtained our AI output on October 18th, 2026, which was formalized in Lean the following day” | 04 | 数学一路为“release the AI output”，没有设外部合作组；博文称“we reached out to an expert to discuss forming a collaboration … Following their recommendation, we decided to release the AI output publicly, following the same strategy for this result as our October 6th, 2026 release”（该专家未具名） | 已找到 |
| A1h 物理（Yang–Mills/QCD）一节：为何未发、谁在审、时间 | L1 | 同上 | “The physics collaboration … the group has decided to hold off on releasing the AI-produced artifacts and instead work on writing it up themselves.”；“David Gross will lead a group including Michael Douglas, Arthur Jaffe, Igor Klebanov, and Nikita Nekrasov”；“Our models produced a natural language proof for Yang–Mills existence on September 9th, 2026, and then subsequently produced a proof for QCD on September 18th, 2026. Given the length and complexity of the work, we postponed release until the Lean formalization completed. We finished our formalization of Yang–Mills’ existence on October 16th, 2026”；“As a subproblem, this work also includes a proof of the existence of pure Yang–Mills theory, which is part of the Quantum Yang–Mills Theory Clay Mathematics Institute Millennium Prize problem” | 未截（页面 y≈9800 与 11260–11600 可补截） | 页面只称“existence”，没有写 mass gap；“暂缓发布”原因在博文里是“该组决定”，没有写“因为 Lean 未完成”（Lean 完成是 10-16，博文称之后才形成合作） | 已找到 |
| A1i 额外资助 | L1 | 同上 | “our recently announced $5 million contribution to the Simons Foundation” | 未截 | 与 ETH 会议资助是两件事 | 已找到 |

### A2 OpenAI ETH 预印本

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A2a 摘要与署名 | L1 | https://github.com/openai/math/blob/main/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf（存档 `a-oa-eth-paper.pdf/.txt`，raw.githubusercontent 取） | “THE EXPONENTIAL TIME HYPOTHESIS IS FALSE”，作者栏 “OPENAI”；“We give a deterministic algorithm for exactly counting the satisfying assignments of an arbitrary k-CNF formula. For every fixed k ⩾ 2, on a formula with n variables and m clauses, our algorithm uses poly_k(n, m) space and runs in time (n + m)^{O_k(1)} 2^{n/(log n)^{1/2+o(1)}}. Our algorithm refutes the Exponential Time Hypothesis (ETH), which postulates that deciding 3-SAT on n variables requires 2^{Ω(n)} time in the worst case.”；“Date: 17 October 2026, revised 25 October 2026.” | 05 | 固定 k（k ⩾ 2），非任意 CNF；署名为机构而非个人；论文**未见**人类参与或实用性说明（grep “human”“practical”“asymptotic”“in practice”均无本文声明；仅在背景段提到他人的实验） | 已找到 |
| A2b 定理 1.1 与两个界 | L1 | 同上 | “Theorem 1.1. For each fixed k ⩾ 2, there exists a deterministic algorithm that, given a k-CNF F on n ⩾ 20 declared variables with m clauses, computes #F … in (n + m)^{O_k(1)} 2^{O_k(n log log n/√log n)} bit operations and (n + m)^{O_k(1)} bits of space. For each fixed D > 0, its running time on inputs with m ⩽ Dn is (n + m)^{O_k,D(1)} 2^{O_k,D(n/√log n)}.”；“The rational interpolation argument in Section 6.4 gives the weaker exponent O_k(ρ(n)), where ρ(n) = n(log log n)²/√log n” | 05 | 摘要里的 n/(log n)^{1/2+o(1)} 与两个界都相容；定理给的是更强的 n log log n/√log n，正文第 6.4 节的方法只给 (log log n)² 版，附录 D 证两个界 | 已找到 |
| A2c 只对固定宽度 k-SAT，不推出一般 NP 问题次指数 | L1 | 同上 | “Note that runtime bounds of the form 2^{o(n)} do not transfer under general polynomial-time reductions (2^{n^{o(1)}} would be needed for this), so our result does not imply that general NP problems admit subexponential time algorithms.” | 未截（PDF 第 2 页下部） | 同页另给：图着色、独立集、完美匹配的次指数计数 | 已找到 |

### A3 OpenAI 的 Lean 范围文件

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A3a 378.md（ETH）覆盖范围与时间界 | L1 | https://github.com/openai/math/blob/main/lean/docs/378.md（存档 `a-oa-lean378.*`） | “For fixed k≥2, Lean proves that a deterministic machine exactly counts satisfying assignments of a canonically encoded k-CNF on n variables in time (L+1)^{O_k(1)}2^{O_k(n(log log n)²/√log n)}, where L is the encoding’s bit length. At every step, the machine has visited at most (L+1)^{O_k(1)} tape cells in total.”；“For m clauses, the October 25 manuscript gives the sharper time bound (n+m)^{O_k(1)}2^{O_k(n log log n/√log n)}. When m≤Dn for fixed D>0, this improves to (n+m)^{O_{k,D}(1)}2^{O_{k,D}(n/√log n)}.” | 06 | **与预印本不一致处：Lean 证的是 (log log n)² 版，论文主定理是 log log n 版；稀疏 m≤Dn 的 n/√log n 版也只在论文里。**界以输入比特长度 L 计，论文以 (n+m) 计。无 axiom 说明 | 部分支持 |
| A3b 提交时刻 | L1 | GitHub API `repos/openai/math/commits`（存档 `a-gh-*.json`） | 提交 `42907c9` “Update math/cs papers, formalizations, and verification data” 作者 Dan Roberts；PR #3 合并 `7ef5c0c` | — | 04:52:18（20:52:18Z）与 05:18:03（21:18:03Z），均 10-27 北京 | 已找到 |
| A3c Lean 陈述里有 sorry 吗 | L1 | https://github.com/openai/math/blob/main/lean/ComparatorChallenges/SubexponentialSAT.lean（存档 `a-oa-lean-sat.txt`） | `theorem exact_count_and_sat (k : ℕ) (hk : 2 ≤ k) : ∃ (machine : …) … := by sorry` | 未截 | 这是 Comparator 的“挑战陈述”文件，sorry 是约定做法；证明在 `lean/OAI/Algorithms/SubexponentialSAT/…`（`formalization.yaml` 指向 `SqrtLogSpace/Final.lean` 的 `exact_count_and_sat`）。**我没读完那几百个证明文件**，OpenAI 文档里**未找到**对“无 sorry、仅标准公理”的明确声明（README 只写 “Not all have accompanying Lean formalizations”“Some of the unformalized results could have issues”）。`formalization.yaml` 文末 `review: status: unchecked`，`automation: methods: - method: agent` | 部分支持 |
| A3d 079.md（局部光滑）范围 | L1 | https://github.com/openai/math/blob/main/lean/docs/079.md（存档 `a-oa-lean079.*`） | “For every dimension n≥2 and every 2≤p<∞, the formalization bounds the L^p(R^n×[1,2]) norm of the Euclidean half-wave by a constant times the W^{s,p}(R^n) norm of its Schwartz initial data.”；“The formalization also gives uniform Bochner–Riesz multiplier bounds, spherical extension estimates, and Stein square-function bounds in their strict ranges. The Kakeya maximal operator satisfies an L^n bound with loss δ^{-ε} for every ε>0 at tube radii 0<δ≤1. Every subset of R^n containing a unit segment in each direction has Hausdorff dimension n.” | 07 | 同一份 Lean 同时挂两篇论文（Oct 25 的 Euclidean 论文与 Sep 24 的三维临界论文） | 已找到 |
| A3e 形式化覆盖哪几项成果 | L1 | https://github.com/openai/math/blob/main/history.md（存档 `a-oa-history.*`）；博文 | 10-26 条目：“We have added two new manuscripts with accompanying Lean formalizations … Local smoothing for the Euclidean wave equation … The Exponential Time Hypothesis is False”；另“The rational Hodge conjecture for CM abelian varieties. A Lean formalization was added.” | — | **Yang–Mills/QCD 的形式化不在该仓库**；博文称其 Lean 10-16 完成、材料未发 | 已找到 |

### A4 局部光滑/Kakeya/Stein 范围措辞

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A4 博文与 GitHub/论文的措辞不同 | L1 | 博文；https://github.com/openai/math/blob/main/history.md；`a-oa-ls-paper.txt`（论文摘要） | 博文：“resolve the Kakeya conjecture and Stein’s restriction conjecture in all dimensions”。history.md：“It also yields the Kakeya maximal and dimension conjectures and Stein’s spherical restriction estimates throughout the interior of the conjectured range.” 论文摘要：“We obtain Fourier restriction estimates for the sphere, the compact paraboloid, and the truncated cone throughout the interior of the conjectured range … We also prove the Kakeya and Nikodym maximal conjectures; in particular, every Euclidean Kakeya set has full Hausdorff dimension.” 079.md：“Stein square-function bounds in their strict ranges” | 04、07 | **措辞有差异：博文写“Stein 限制猜想在所有维度被解决”，论文摘要与 history.md 只写“猜想区间的内部（interior）”的限制估计；Kakeya 一侧（极大函数、Hausdorff 维数）三处一致。** WorkBuddy 的说法成立 | 已找到 |

### A5 Anthropic README 与 paper.pdf

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A5a 结论与时间界 | L1 | https://github.com/anthropics/formal-math/tree/main/eth（README 存档 `a-an-readme.*`；paper `a-an-paper.pdf/.txt`） | README：“A deterministic algorithm solves 3-SAT on n variables in time 2^o(n), which refutes the Exponential Time Hypothesis (ETH). The algorithm in fact solves the harder problem #3-SAT, counting the satisfying assignments, in time 2^O(n log log n / √log n).” 论文定理 1：“2^{O(min{k²n log log n/√log n, km/√log m})}” | 08、10 | README 写 k 固定 3 的特例；论文主定理含 k² 与 m 项 | 已找到 |
| A5b “purely asymptotic” | L1 | 同上 | README：“The result is purely asymptotic and says nothing about solving formulas in practice.” 论文：“Our result is purely asymptotic: for arbitrary 3-CNF formulas with m = 5n clauses, the bound we prove does not improve on the 2^n of exhaustive search unless n > 10^{10 000}, and the bound in n alone, which holds for any number of clauses, does not unless n > 10^{10^8} … Our general approach seems unlikely to yield any general improvement unless n > 10^7, however it is instantiated. Brute force enumeration is already impractical for n = 100, so the improvement of our result does not appear in practical settings.” | 08、10 | WorkBuddy 的“m=5n 时 n>10^10000”对；论文另有“只按 n 的界要 n>10^(10^8)”（页面上标渲染为 10^{10^8}，附录 D 末尾亦写 10^{10^8}）这一条扫描没有 | 已找到 |
| A5c “no machine checks…” | L1 | README | “Lean checks the proofs, but no machine checks that this file says what the paper says; the file is kept short so that a reader can check this.” | 09 | 指 `MainResults.lean`（只含定义、无证明，只依赖 Mathlib） | 已找到 |
| A5d Lean 检查范围与人类审阅 | L1 | 论文附录 E（第 40 页）；README | 论文：“The Lean development was written by AI models and reviewed by other instances of the same models; at the time of writing no person is known to have checked it; its constant is not explicit.”；“they rest on no axiom beyond Lean’s three standard ones (propext, Classical.choice, Quot.sound). They are accepted by Lean’s kernel, by Lean’s replay checker leanchecker … and, through Comparator, by the independently implemented kernel nanoda”；“All 1,050 Lean modules”。README：两处 “declaration uses `sorry`” 均来自 `Challenge.lean`，“The build took about 8 minutes on 24 virtual CPUs” | 09、10 | 论文附录 E 说明 Lean 陈述与论文印刷陈述的差异（单步计数、k≥1、n≥4、m≥2 等）；我核对了 `Challenge.lean`（1938 字节）为陈述文件 | 已找到 |
| A5e 发现时间、token、人类输入 | L1 | 论文第 1 页末“Provenance” | “The core algorithm was found on October 16 by an internal Claude research model with 14M output tokens and no human input. Anthropic researchers worked with Claude to improve the presentation, resulting in this paper. Independently and concurrently, an internal model at OpenAI claims to have proven a similar result [Ope26].” | 10 | “no human input”只限核心算法发现；论文呈现由 Anthropic 研究员与 Claude 改写；论文落款 “Date: October 26, 2026”；README：“The result was discovered by an internal research Claude model”；署名 “CLAUDE ANTHROPIC” | 已找到 |
| A5f “800 多篇” | L1 | 论文第 2 页 | “In a literature search carried out by Claude, we found more than 800 papers that prove at least one result of their own only under ETH or under a hypothesis that implies it, such as SETH or Gap-ETH; we did not count results that are also proved under a hypothesis that we do not refute, such as Orthogonal Vectors.” | 10 | 数字来自 Claude 自己的文献检索，非人工清点；同页“Theorem 1 does not refute the Orthogonal Vectors hypothesis” | 已找到 |
| A5g 与 P vs NP 的关系 | L1 | 论文第 1 页 | “However, our result does not bear on the P vs NP question directly, nor does it yield any practical algorithms for NP-complete problems.” | 未截 | OpenAI 论文同样写 2^{o(n)} 不随一般多项式归约传递（A2c） | 已找到 |
| A5h 其他：Hexagon 页面元数据 | L3（第三方预印本站，其自身数据） | https://hexagonmath.org/2610.00577v1（存档 `a-hexagon-abs.*`、`a-hexagon-eth.pdf`） | 页面：“Contributed by Anthropic”；“Version 1 / Oct 27, 2026 / CC BY 4.0”；标签 “Primarily AI-generated text”“Human understanding: some parts”；历史：“Submitted by Jelani Nelson on behalf of Anthropic … Initial deposit … Oct 27, 2026”；HTML 元数据 `datePublished` 2026-10-27T05:12:41.548Z | 未截 | 13:12:41 北京；Hexagon 是新预印本站（Hexagon Mathematics Foundation）；提交者是页面公开署名的人 | 已找到 |

### A6 先后与相互提及

| 说法 | 级 | 一手来源 | 原文/数据 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A6a OpenAI 各时刻 | L1 | GitHub API；博文；fxtwitter（`a-fx-*.json`）；HN Algolia（`a-hn-item-*.json`） | 仓库提交 04:52:18；PR 合并 05:18:03；Williams 开帖 05:35:29 “An internal model at OpenAI has produced a claimed refutation of ETH!”；HN 博文帖 05:35:28；Dan Roberts（OpenAI）开帖 05:36:57；HN ETH 论文帖 05:59:15 | — | 博文页只给日期（10-26） | 已找到 |
| A6b Anthropic 各时刻 | L1/L3 | GitHub API；Hexagon | GitHub `16a5ce4` “Add eth: paper and Lean 4 formalization of “A 2^o(n) Time Algorithm for 3-SAT” (#31)” 11:07:21；Hexagon 13:12:41；HN 帖 “A 2^O(n) Time Algorithm for 3-SAT” 15:48:48（3 分/0 评） | — | 该仓库首条提交 2026-08-06；上一条 eth 以外的项目提交是 10-06（3sum-apsp） | 已找到 |
| A6c Anthropic 官网/X 公告 | — | https://www.anthropic.com/news（存档 `a-anthropic-news.*`）；WebSearch 两次 | 新闻页无 ETH 条目；搜索无 Anthropic 官方 X 帖 | — | X 无法在不登录时搜索，**未检索到不等于不存在** | 未找到一手来源 |
| A6d 双方是否提到对方 | L1 | 两篇论文 | Anthropic 论文引用 `[Ope26] OpenAI. The Exponential Time Hypothesis is false. Preprint, October 26, 2026.` 并写 “Independently and concurrently, an internal model at OpenAI claims to have proven a similar result”。OpenAI 博文、论文、仓库 README、history.md 全文**均未出现 “Anthropic” “Claude”** | — | Anthropic 论文落款日期 10-26，GitHub 提交 10-27；其引用 OpenAI 说明稿子在 OpenAI 发布后才定稿 | 已找到 |
| A6e 谁先发现 | L1 | 博文；Anthropic 论文 | OpenAI：“Our AI found its natural language disproof for the ETH on October 16th, 2026”；Anthropic：“The core algorithm was found on October 16 by an internal Claude research model” | 03、10 | 只记两家各自说法，同为 10-16，无时刻、无时区；不推断 | 已找到 |
| A6f 第三方点出“两家” | L4 | Scientific American 2026-10-27（Howlett、Hall）https://www.scientificamerican.com/article/dueling-proofs-from-openai-and-anthropic-astound-scientists/（存档 `a-media-sciam.*`） | “Mere hours after OpenAI’s announcement, Anthropic posted its own independent result disproving ETH”；“Two of these results coincide with closely related work involving OpenAI’s competitor, Anthropic.” | 未截 | “独立”“几小时后”是 SciAm 的说法；本文档按提交时刻是约 5.5–7.5 小时 | 已找到 |
| A6g 该组公开信已列 Anthropic | L6（专家公开信） | https://docs.google.com/document/d/1adB0IhJgLzsmbfCgoLrtoJTjnlPf74UnyjMo35P5JHo/edit（存档 `a-eth-group-letter.txt`，export?format=txt） | “Proposed arrangements for community participation … 5. Anthropic: Refutation of ETH” | 未截 | 文档可改、无时间戳；ethdiscussion.blog 的 10-26 帖不含此行 | 已找到 |

### A7 两家算法是否同一路线、哪个界更强

| 说法 | 级 | 一手来源 | 原文 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A7a 时间界 | L1 | 两篇论文 | 见 0 节第 2 条。主定理的 n 指数相同：n log log n/√log n | 05、10 | 技术路线：OpenAI 附录 D 称 “by tensor contraction”，Anthropic README 与论文关键词含 “tensor networks, rank”。**两份原文都没有互相比较的句子**（Anthropic 只写 “a similar result”）；“同一路线”是我对关键词的推断，未核实证明细节 | 部分支持 |
| A7b 具名专家比较 | — | — | 未找到具名专家比较两家算法的公开文字（Aaronson 博客评论区的 Jonathan Baxter 评论 #6 讨论 OpenAI 的 Lean 版弱于论文版，非具名专家） | — | — | 未找到一手来源 |

### A8 AGMAI

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A8a About（9-22）独立性 | L1（AGMAI 自己的站，L3 级别由 Claude 定） | https://agmai.org/ （全站存档 `a-agmai-home.*`；各声明都在首页锚点，没有独立 URL，猜测的子页 404 已记失败） | “This group operates independently of any AI company and members do not accept payment for this work. We will publish our recommendations to AI companies on this website.”；“Although we will give advice, we do not have decision making power at any AI company” | 11 | 日期 “September 22, 2026”，无时刻 | 已找到 |
| A8b 与 OpenAI 的关系 | L1 | 同上；https://openai.com/index/advisory-group-on-mathematics-and-ai/（存档 `a-oa-advisory.*`） | AGMAI：“This group came together after OpenAI approached some of its members about establishing an external advisory board. In agreement with OpenAI, they decided to instead create an independent group and invite the others to join.” OpenAI（September 21, 2026）：“we’re working with mathematicians who have established an independent mathematics advisory group”；“The group will operate independently from OpenAI … Its members will not be paid by OpenAI”；“hosted at the Institute for Advanced Study” | 11 | 召集：OpenAI 先接触其中部分成员，成员把“外部顾问委员会”改成独立小组；付费：双方都写成员不受 OpenAI 付费 | 已找到 |
| A8c 完整成员名单 | L1 | agmai.org About；IAS 页 https://www.ias.edu/nccr/projects#agmai（opencli 取，存档 `a-ias-nccr-agmai.txt`；curl 被 Cloudflare 403） | François Charles (ENS-PSL)；Camillo De Lellis (IAS, GSSI)；Timothy Gowers (Collège de France, Cambridge)；Martin Hairer (EPFL, Imperial College London)；Nikhil Srivastava (Berkeley, Simons Institute)；Ulrike Tillmann (Oxford)；Ravi Vakil (Stanford)；Edward Witten (IAS)；Melanie Matchett Wood (Harvard)。IAS 页：项目 “FY26 Spring Awards”，Principal Investigators Camillo De Lellis 与 Edward Witten | 11 | 9 人；IAS 页写 Tillmann 为 “(Oxford, INI)” | 已找到 |
| A8d 9-29 建议 | L1 | https://agmai.org/general-sep29/（存档 `a-agmai-general.*`；PDF 链接 `…/recommendations.pdf` 未存） | “we do not endorse this practice, and we ask them to stop testing advanced mathematical problems on proprietary models.”；“informed by over six hundred survey responses”；“The development of human understanding must remain organic and community led. It should not be directed by AI labs, even when the labs have produced the results.”；“We strongly recommend that AI labs refrain from treating the release of mathematical results as marketing vehicles” | 未截 | 日期 “September 29, 2026” | 已找到 |
| A8e 10-6 声明 | L1 | agmai.org 首页 | “AGMAI’s advisory role should not be interpreted as a judgment of the impact of these results or an endorsement of the process by which OpenAI obtained them.”；“We have discussed them with OpenAI and appreciate the company’s willingness to engage.” | 未截 | 日期 “October 6, 2026.” | 已找到 |
| A8f 10-25 声明 | L1 | 同上 | “In some past releases an AI company has designated mathematicians to receive the output on a problem, often before anyone else.”；“we recommend that AI companies do not adopt this practice for future releases”；五条理由：Delayed or unequal access / Denied agency / Biased selection / Borrowed reputation / It is unnecessary；“This should in no way be construed as criticism of the mathematicians who have taken on such a role” | 12 | 日期 “October 25, 2026”；Tao 的博客同日转载为 guest post（https://terrytao.wordpress.com/ ，存档 `a-tao-blog-home.html`，标 “by Terence Tao | 23 comments”，注明由 AGMAI 来稿、原为别种格式经 AI 转换） | 已找到 |
| A8g 10-26 声明 | L1 | 同上 | “We call on OpenAI to make public without delay the natural-language and Lean outputs behind its announced existence proofs for Yang–Mills theory, quantum chromodynamics, and a completion of the Standard Model.”；“We have great respect for the scientists asked by OpenAI to lead this work.”；“OpenAI have said they delayed release until the Lean formalization was complete. By their own account this is now complete, so there is no scientific reason for delaying the release any further.” | 12 | 日期 “October 26, 2026”，无时刻；HN 链接帖 10-27 08:51 北京（1 分/0 评）；声明点名的是 Yang–Mills/QCD，没有就 ETH 或局部光滑表态 | 已找到 |
| A8h 声明与 OpenAI 博文的关系 | L4 | SciAm（同 A6f） | “OpenAI’s approach to two of the three results also directly violates a clarifying statement AGMAI published just a day prior, by designating specific experts to shepherd the results into the public sphere.” | 未截 | “violates”是 SciAm 的定性，AGMAI 声明本身用词是 “recommend … do not adopt”；SciAm 同文把 AGMAI 写成 “the expert panel OpenAI organized”（与 A8a/A8b 不符，见夸大实例） | 部分支持 |

### A9 ETH 与 P≠NP；具名复杂度学者

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A9a Ryan Williams（MIT，OpenAI 点名的审查组成员）| L6（本人 X 帖） | https://x.com/rrwilliams/status/2114833319431209352 及串（fxtwitter 存档 `a-fx-williams-*.json`） | 10-26 21:35Z：“An internal model at OpenAI has produced a claimed refutation of ETH! Russell Impagliazzo, Mohan Paturi, and I have been asked to help organize an open and independent examination of this claim.”；“We’ve signed no NDAs and will receive no compensation”；10-27 01:28Z（09:28 北京）串：“If true, their claim implies a variety of other NP problems now all have faster algorithms as well. Since 3SAT is NP complete, this should make every computer scientist put at least a little more weight on the possibility of P = NP.”；“one of the first things I wanted OpenAI to check is that no existing cryptosystems are compromised by the result. Apparently they are safe...” | 未截 | 条件句“If true”，**没有说已读完证明**；“claimed refutation”为其用词 | 已找到 |
| A9b Impagliazzo–Paturi–Williams 公开信 | L6 | 上述 Google 文档；https://ethdiscussion.blog/2026/10/26/eth-refutation-examination-and-discussion/ | “Our involvement should not be taken as certification of the claim. A result of this significance requires careful, independent scrutiny and an understanding that others can reconstruct, question, and build upon.”；“If the claim is correct, it will change an important part of theoretical computer science.” | 未截 | 信的发件人署名为三人；blog 页作者栏为 “rpaturi…”（账号名，不抄） | 已找到 |
| A9c Scott Aaronson（UT Austin） | L6 | https://scottaaronson.blog/?p=10239 （存档 `a-aaronson-post.html`） | 博文（Oct 26）：“my students just came to my office to tell me that an OpenAI model has now refuted the Exponential Time Hypothesis, by giving an algorithm to solve 3SAT in an amount of time that scales roughly like exp(n/√log(n)).”；评论 #4（署名 “Scott”，“October 26th, 2026 at 6:16 pm”，时区页面未写）：“Given this news, I’ll up my personal probability of P=NP from 3% to 5%.”；#29：“No, I have zero news about breaking of cryptographic codes.” | 未截 | 评论的作者身份是博客评论署名，非独立核验；他转述学生的话，没说自己读过证明 | 部分支持 |
| A9d Lance Fortnow、Impagliazzo/Paturi 个人博客 | — | https://blog.computationalcomplexity.org/ （存档 `a-fortnow-blog.html`，23:0x 取） | 首页 10 月帖子标题无 ETH 条目（如 “Reading Messy Proofs”“Complexity and Lean”） | — | 未检索 Impagliazzo、Paturi 个人页 | 未找到一手来源 |
| A9e ETH 与 P≠NP 的关系（论文/博文口径） | L1 | OpenAI 博文；Anthropic 论文 | 博文：ETH “a problem deeply related to the P vs. NP problem”；Anthropic：“our result does not bear on the P vs NP question directly” | 03、10 | ETH 为真 ⇒ P≠NP；ETH 为假不推出 P=NP（HN 评论里 imadr、jibalt 的说法与此一致，仅 L6） | 已找到 |

### A10 局部光滑与 Yang–Mills 的外部专家反应

| 说法 | 级 | 来源 | 内容 | 状态 |
|---|---|---|---|---|
| A10a Terence Tao | L6 | https://terrytao.wordpress.com/ 首页（`a-tao-blog-home.html`），mathstodon 页（`a-tao-masto.html`，仅 JS 壳，无内容） | 首页未见就这三项成果发表评论的帖；有 AGMAI 10-25 声明转载与 “oai-math-revised” 社区修订仓库帖（10-24），均为先前议题 | 未找到一手来源 |
| A10b 物理组成员（Gross、Douglas、Jaffe、Klebanov、Nekrasov） | — | OpenAI 博文 | 博文只称他们将“shepherd”；所有联络走 YANGMILLS4D@gmail.com（博文公开邮箱，非个人邮箱）；没找到成员本人公开表态 | 未找到一手来源 |
| A10c Roland Bauerschmidt（NYU）关于 Balaban 路线 | L4（SciAm 引语） | SciAm 同上 | “He basically did it … But I think there’s no one who understands these papers and can tell whether they’re correct or not.” | 部分支持（这是评论 Balaban 论文，不是评论 OpenAI 的证明） |
| A10d AGMAI 对 Yang–Mills 的表态 | L1 | 见 A8g | 呼吁公开，不评证明对错 | 已找到 |
| A10e Anthropic 一侧与 Yang–Mills | L4 | SciAm | “Working with Anthropic employee Levent Alpöge, Ethan Sussman, a mathematician at Northwestern University, created and publicized a public GitHub repository to do just that [finish Balaban’s program]. … OpenAI’s claim—with no publicly available proof—appears to have beaten them to the finish line.” | 部分支持（只来自 SciAm，未回溯该仓库） |

### A11 HN 热度（`barking heat`，22:59 北京取，`a-heat.json`）

| 条目 | 分/评 | 提交时刻 | 链接 |
|---|---|---|---|
| “Three new discoveries in mathematics, computer science, and physics”（OpenAI 博文） | 1038 / 575 | 2026-10-27 05:35:28（21:35:28Z） | https://news.ycombinator.com/item?id=50251344 |
| “The Exponential Time Hypothesis Is False [pdf]”（OpenAI 论文） | 717 / 258 | 05:59:15（21:59:15Z） | https://news.ycombinator.com/item?id=50251661 |
| “A 2^O(n) Time Algorithm for 3-SAT”（Anthropic GitHub） | 3 / 0 | 15:48:48（07:48:48Z） | https://news.ycombinator.com/item?id=50255798 |
| “A 2 O(n) TIME ALGORITHM FOR 3-SAT [pdf]”（Hexagon PDF） | 1 / 0 | 19:21 | https://news.ycombinator.com/item?id=50257454 |
| “Request for Immediate Release of Yang–Mills AI Output”（AGMAI） | 1 / 0 | 08:51 | https://news.ycombinator.com/item?id=50253373 |

派工单旧数字（1011/556、691/256）是更早的快照。**HN 不显示他人评论得分**，下列评论按有理有据与具名度挑选，无得分；均 L6，只作线索：

- bsubs，ETH 帖，https://news.ycombinator.com/item?id=50253117 ：“SAT-solving expert here. No immediate applications (the results in the paper are “galactic”). The main question is whether some ideas here will snowball into other ideas…”（自称 SAT 求解专家，无法核实身份）
- mswphd，ETH 帖，https://news.ycombinator.com/item?id=50253240 ：多项式时间归约会放大实例规模，所以 3SAT 的 2^{o(n)} 不自动推出其他 NP 问题次指数（与论文自己那句 A2c 一致）
- jibalt，ETH 帖，https://news.ycombinator.com/item?id=50254961 与 …54685：ETH 为假并不使 P=NP 更可信地成立，逻辑方向只在 ETH 为真时才推出 P≠NP
- stillpointlab，博文帖，https://news.ycombinator.com/item?id=50254230 ：“Scientists working on similar or adjacent problems that are not included in these kinds of private releases might feel they are not getting a fair chance.”（物理组 5 人私下先行）
- JonChesterfield，ETH 帖，https://news.ycombinator.com/item?id=50253174 ：称 Lean 构造里“小规模”部分用查表、状态数“vastly more states than … the observable universe”，并转引“answer from openai”。**转述来源不明，我未核实，建议不用**

## 2. 可能的吠点（原文里有、线索摘要没提或说反的条件与限制）

1. OpenAI 的 Lean 只证了更弱的 (log log n)² 版，更强的 log log n 版与稀疏 n/√log n 版只在论文里（378.md；A3a）。
2. OpenAI 的 ETH 结果只对“固定宽度 k-CNF”，且论文自己写不推出一般 NP 问题次指数（OpenAI 论文第 2 页；A2c）。
3. Anthropic 论文：m=5n 时 n > 10^10000，只按 n 的界要 n > 10^(10^8)，“says nothing about solving formulas in practice”（A5b）。OpenAI 论文未见同类声明。
4. Anthropic 的 Lean：“no person is known to have checked it”，由同模型实例审阅，陈述文件是否忠实“no machine can check”（附录 E；README；A5c–A5d）。
5. “verified in Lean” 是 OpenAI 自述；外部组尚在验证，公开信明说 “should not be taken as certification”（A1e、A9b）。`formalization.yaml` 的 `review: status: unchecked`（A3c）。
6. 两家都说核心发现在 10-16；OpenAI 论文落款 17 October、revised 25 October；Anthropic 论文落款 26 October 且引用了 OpenAI 的预印本，因此 Anthropic 版本至少有一次在 OpenAI 发布后的定稿（A5e、A6d）。
7. 博文把 Stein 限制猜想写成“所有维度被解决”，论文摘要与 history.md 只写“猜想区间内部”（A4）。
8. 物理（Yang–Mills/QCD）：博文只称“existence”，不含 mass gap；材料未公开，博文里也没有可供外人核对的论文或 Lean；AGMAI 呼吁立即公开（A1h、A8g）。
9. 数学（局部光滑）一路没有外部合作组（只咨询了一位未具名专家），靠 Lean 与社区自发审查；OpenAI 自己承认指定 shepherd “creates asymmetry of information”（A1d）。
10. OpenAI 仓库 README 目录现为 “721 manuscripts organized into 373 families”（1006 期记 722），`history.md` 写 “300 / 719 = ~42%” 顶线结果已形式化；不是新闻点，但数字口径变了。

## 3. 夸大/写歪说法实例（找到 4 条，均非严重夸大，但都属“漏掉 Anthropic”或“写歪 AGMAI”）

| 说法 | 原站 URL | 原句逐字 | 时间 | 发布方 | 备注 |
|---|---|---|---|---|---|
| 把 AGMAI 写成 OpenAI 组织的 | https://www.scientificamerican.com/article/dueling-proofs-from-openai-and-anthropic-astound-scientists/ | “the Advisory Group on Mathematics and Artificial Intelligence (AGMAI), the expert panel OpenAI organized in response to widespread criticism from mathematicians” | 2026-10-27（官方未给时刻） | Scientific American（Howlett、Hall） | 与 AGMAI About 和 OpenAI 9-21 文章的“independent”不符；同文正确写了 “Mere hours after OpenAI’s announcement, Anthropic posted its own independent result” |
| 标题直写“AI 已推翻”，只写 OpenAI，未提 Anthropic | https://www.mindstudio.ai/blog/exponential-time-hypothesis-disproven | 标题 “AI Just Disproved the Exponential Time Hypothesis. Here’s Why It Matters”；副标题 “OpenAI says its AI disproved a 27-year-old computer science belief, the exponential time hypothesis, in 10 days.” | 2026-10-27 | MindStudio（Luis Chavez-Mattos 编辑） | 正文有“Not yet [confirmed]”的限定；“in 10 days”是媒体用 10-16 到 10-26 推出来的，OpenAI 博文只给了发现与形式化日期；正文把 “Greg Brockman noted that P still probably does not equal NP” 写成 OpenAI 说法，我**未找到该句的一手来源** |
| “OpenAI 推翻 ETH”，只写 OpenAI | https://thenextweb.com/news/openai-exponential-time-hypothesis-false-kakeya-yang-mills | 标题 “OpenAI says its AI model has disproved the Exponential Time Hypothesis”；“Hundreds of computer science papers rest on a conjecture that an internal OpenAI model now claims to have refuted” | 2026-10-26 22:30（页面 “October 26, 2026 - 10:30 pm”，时区未写） | The Next Web | 措辞到位（“claims”“says”），写作时 Anthropic 尚未发；**不是误导，只用作“漏掉 Anthropic 的时间点”对照**。同文把 Williams 的公开信写对了 |
| 局部光滑写成“证明了…并解决 Kakeya、Stein” | https://cryptobriefing.com/openai-proves-local-smoothing-conjecture/ | 页面标题 “OpenAI says its models proved the local smoothing conjecture in every dimension”；“According to OpenAI, the result also resolves the Kakeya conjecture and Stein’s restriction conjecture across all dimensions.” | 2026-10-26 | Crypto Briefing | 照抄博文的“all dimensions”，没带论文摘要里的“interior of the conjectured range” |

没找到的：中文媒体的“AI 证明 P=NP”“数学界已确认”“解决千禧年难题”类标题（WebSearch 中文查询无命中，未逐家检索国内平台）。FourWeekMBA、XenoSpectrum 被站点拒绝（403），未取得，不引用。

## 4. 扫描说法勘误

| 扫描说法 | 原文 | 结论 |
|---|---|---|
| WorkBuddy：Anthropic“更早公开” | A6a/A6b | **错**：OpenAI 先（见 0.1），Anthropic 晚约 5.5–7.5 小时 |
| WorkBuddy：AGMAI 是“OpenAI 自己组建的顾问组” | A8a/A8b | **错/过简**：独立；OpenAI 先接触，成员改建为独立小组；OpenAI 自家文章也写独立、不付费 |
| Claude：AGMAI 成员只读到 5 位 | A8c | 共 9 位，含 Tillmann、Vakil、Witten、Wood；WorkBuddy “含 Witten”对 |
| ChatGPT：378.md 时间界 (L+1)^{O_k(1)}·2^{O_k(n(log log n)²/√log n)} 且与论文不同 | A3a | 对（原文并写 “the October 25 manuscript gives the sharper time bound”） |
| WorkBuddy：博文“解决所有维度 Kakeya 与 Stein 限制猜想”，GitHub 收窄为“猜想区间内部” | A4 | 对（博文原句 “in all dimensions”；history.md 与论文摘要 “interior of the conjectured range”） |
| WorkBuddy：论文写“m=5n 时 n>10^10000”、“800 多篇”、发现于 10-16、14M token、无人类输入 | A5b/A5e/A5f | 全部在原文：10^{10 000}；“more than 800 papers”；October 16；“14M output tokens and no human input”。补充限定：“no human input”只指核心算法发现 |
| 扫描：OpenAI 内部发现时间 ETH 10-16、局部光滑 10-18、Yang–Mills 09-09 | A1f/A1g/A1h | 对。补：ETH Lean 10-17，局部光滑 Lean 10-19，QCD 自然语言 09-18，Yang–Mills Lean 10-16 |
| 扫描：“OpenAI 承认指定数学家造成信息不对称” | A1d | 对，原句 “choosing initial shepherds necessarily shapes who gets involved first and creates asymmetry of information” |
| 扫描：物理材料由外部合作组暂缓发布 | A1h | 对，原句 “the group has decided to hold off on releasing the AI-produced artifacts” |
| HN 1011/556 与 691/256 | A11 | 当前 1038/575 与 717/258 |
| Anthropic README “Hexagon link to appear” | A5h | 已上线：https://hexagonmath.org/2610.00577v1，2026-10-27 05:12:41Z（13:12:41 北京） |
| 仓库提交 03:07:21Z（北京 11:07） | A6b | 对，与 GitHub API 一致 |
| Claude：1006 的“IAS 顾问组”是否即 AGMAI | A8b/A8c | 是。AGMAI 首页页眉链接 IAS Nelson Center 项目页；OpenAI 9-21 文章写 “hosted at the Institute for Advanced Study” |

## 5. 未完成或有限项

- A9：Impagliazzo、Paturi 个人页与 Fortnow 博客未见 ETH 帖；Aaronson 评论的作者身份只据博客署名。
- A10：Tao、物理组成员、Hong Wang/Zahl/Guth 等对局部光滑的公开反应均没找到。
- A12：国内/中文平台未逐家查。
- A3c：OpenAI 的 Lean 证明目录有数百个文件，我只读了 Challenge 陈述与 yaml，没有 grep 全库 sorry/axiom，也没有本地编译。
- 截图 12 张已满（A 组区间 01–12）；没有截的候选：OpenAI 博文物理一节（A1h）、HN 两帖卡片、Williams 帖、Aaronson 评论、公开信。
- 排错：存档目录里有 4 个因猜 URL 而得到 404 页的文件（`a-agmai-sep22/oct6/oct25/oct26.*`），无用，列在待删除。
