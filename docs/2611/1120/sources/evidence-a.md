# 1120 期 A 组取证：同一天两份 GRH 证明

执行方：Claude Sonnet 5.5（claude-sonnet-5-5）。取档时间为北京时间（UTC+8）：主批 2026-11-21 18:24–18:37；X 帖与 Zulip 为 18:27–18:33。原文摘句保持原语言。不判断数学对错。时间换算：页面给 UTC 或美东时间的，括号保留原时区。

文件命名：存档在 `sources/a-<name>.*`（`.html/.pdf/.json` 为原件，`.txt` 为转文本），截图在 `images/01–12`。抓取记录见 `capture-log-a.md`。

## 一、关键发现（先看这里）

1. **Sutherland 复核页 18:25 与 18:37 两次取档，与 09:30 UTC（北京 17:30）版逐字相同**：lake comparator 与 Lean 官方内核（默认、paranoid）仍是 `running`，没有新增 fail。页面写“checks run with Claude Code”；他在 Zulip 贴的摘要则写明由 “Fable 5.2” 编排计算（见 A4）。
2. **“GRH”范围两份不同**：Alpöge 的 Lean 定理是 Mathlib 的 `RiemannHypothesis` 加 Dirichlet L 函数（Sutherland 页原句 “"GRH" here means Dirichlet L-functions only.”）；Masullo 稿自称覆盖“all number fields, all unitary Hecke characters”（第 6 页），但第 13 章（推出零点位置的章）只有两页、主定理未写（见 A7）。Tao 博文写的是 “Generalized Riemann Hypothesis”，没有区分范围。
3. **Masullo 稿未完成的证据来自稿件本身**：第 6 页作者自述 “The introduction itself is unfinished”“aware of several glitches here and there”；“Return here” 共 30 处（27 处纯 “Return here.”，页码 199–483；另 3 处变体在 464、514、515 页）；第 522 页有 “Theorem ??”、脚注 “Insert statement.” “State and prove the Main Theorems…”。
4. **Vakil 博文的“8 小时后更新”**：博文发布 2026-11-20 18:18:42 UTC（北京 11-21 02:18:42），页面最后修改 2026-11-21 02:54:54 UTC（北京 10:54:54）。更新段写 “Update about 8 hours later”，页面元数据只能说明修改发生在发布后约 8 小时 36 分，不能直接定出更新的发布时刻。
5. **Elliot Glazer 的 “Fails an Astra audit”**：2026-11-20 18:59:25 UTC（北京 11-21 02:59:25）发在他自己 18:27:12 UTC 那条提到 Masullo 的推文之下，即 Vakil 博文发布 41 分钟后。这条帖子本身没有写出 “Masullo”，指向靠它是对前一条的回复（reply_to 字段）。
6. **“第 480 页 Theorem 11.1.2.1(2) 有反例”**：Reddit 无法匿名读取（登录墙/403），改在两处找到同一指向：HN 评论 50581927（schleck8，“theorem 11.1.2.1(2) on p. 480 onward”）与 Refine Technologies 自发布报告 Issue 1（明确写 “Theorem 11.1.2.1(2), p. 480”）。该定理在稿件第 480 页确实存在。均为 L6，不判断对错。
7. **扫描勘误成立**：Aaronson 三句确在 `scottaaronson.blog/?p=10344`，Tao 博文与评论里搜不到（见 A5/A6）。扫描说“Anthropic 全程未表态”：未找到一手表态，但也没有找到“拒绝评论”的一手来源，只有媒体写 “did not respond”（A10）。
8. 没有找到把 Aaronson 原话安在 Tao 头上的中英文标题（A11）；但有多篇中文标题把 Tao 写成“回应”并把“25 万页”写成事实（见夸大实例）。

## 二、逐条证据表

### A1　Alpöge 声明（remark.pdf）与主页

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| remark.pdf 仍为 4 页，内容与 1119 存档相同 | L2（署名员工自述，托管在其个人站） | https://alpo.ge/remark.pdf | 4 页；`SHA-256 3fc97784b71ddd44ff8aa0a177e99622a1c61b13cfa843a0989ee4a7a0cfeb8d`，与 `docs/2611/1119/sources/d-rh-remark.pdf` 完全一致 | 10 | 1119 存档取于 2026-11-21 00:39 BJT（响应头 Fri, 20 Nov 2026 16:39:10 GMT）；本组取于 11-21 18:25（10:25:01 GMT）。PDF 创建时间 2026-11-20 20:52:48 +08:00。pdfinfo：Pages 4。扫描写“5 页”有误 | 已找到 |
| 声明 RH 与 GRH，并预期 Extended RH | L2 | 同上，第 1 页 | “I believe I have, working with Claude, discovered a proof¹ of the Riemann Hypothesis. In fact, the method easily extends much further, resolving the so-called Generalized Riemann Hypothesis, and I expect, with more time, the Extended Riemann Hypothesis too. Claude and I have also formalized the arguments without assumptions.” | — | “I believe”“I expect”是作者的限定词；脚注 1 指向 `lean.tar.gz.part01…part14` | 已找到 |
| “只放 Lean”“没掌握”“25 万页” | L2 | 同上，第 2 页正文与脚注 2 | “in which I am presenting the world with a complete argument before having mastered a massive neighbourhood around its mathematics. It being obviously inappropriate to attempt to post some massive unreadable travesty of "slop"², I decided to only present Lean.” 脚注 2：“I of course amused myself, and hit 250, 000+ pages.” 同页脚注：“Claude cutely called it the GRH . . . "book" — I looked at Theorem 1 and it began by noting that because the reals are totally ordered if σ ≠ ½ then σ > ½ or σ < ½.” | 10 | 页码为 PDF 页 2；“250,000+ pages”是作者自述的未公开稿，不是已公开文件 | 已找到 |
| 作者对时间线自陈不精通 | L2 | 同上，第 3 页脚注 4 | “The parts in which Claude is the expert and not me I am sort of paraphrasing a "slop" summary, which might make zero sense, and I apologize, though the Lean should suffice to clarify everything. I'm just too inexpert in the relevant stuff at the moment” | — | 脚注 4 印在 PDF 第 3 页底 | 已找到 |
| 作者认为会被大幅简化 | L2 | 同上，第 1 页 | “My intuition insists it will be massively simplified, likely immediately.”“I intend to do my best to understand and impart understanding of this argument and its certain simplifications” | — | 预期，不是结果 | 已找到 |
| 主页 alpo.ge 是否新增说明 | L2 | https://alpo.ge/ | 主页 HTML 与 1119 存档 `d-rh-home.html` 逐字节相同（`cmp` 无差异），页面不含 Riemann、remark.pdf 字样 | — | 1119 取于 11-21 00:39 BJT；本组 11-21 18:25 BJT | 已找到（无新增） |

### A2　Sutherland 复核页

页面 `https://math.mit.edu/~drew/grh/`，存档 `a-sutherland.html/.txt`（取 11-21 18:25:00 BJT，响应头 Date 10:25:00 GMT）与 `a-sutherland-recheck.*`（18:37 BJT）。页内原句：“Andrew Sutherland (MIT), checks run with Claude Code. Page last updated 2026-11-21 09:30 UTC.”（= 北京 11-21 17:30）。两次取档与 09:30 UTC 版 `diff` 无差异，**无新增、无新 fail**。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 175,187 个模块编译 0 错误 | L3（第三方复核者自述其运行；页内可核日志） | 同页 | “All 175,187 modules compile (Lean v4.33.1, Mathlib v4.33.1) — pass — zero errors; finished 16:43:21 UTC” | 01 | 16:43:21 UTC = 北京 11-21 00:43 | 已找到 |
| 五个定理只依赖三条标准公理 | L3 | 同页 + `a-sutherland-audit-log.txt` | “#print axioms for the five theorems in Main.lean — pass — propext, Classical.choice, Quot.sound only (audit.log)”；audit.log 末五行均为 “depends on axioms: [propext, Classical.choice, Quot.sound]” | 01 | audit.log 另列环境中共 15 个公理，“all in Init”（含 `sorryAx`、`Lean.ofReduceBool`，均在 Lean 核心，五个定理不依赖） | 已找到 |
| 顶层命题就是 Mathlib 的 RH 定义 | L3 | 同页 “What was verified” | “The theorem riemannHypothesis : RiemannHypothesis in the submission's Main.lean, where RiemannHypothesis is Mathlib's own definition (Mathlib/NumberTheory/LSeries/RiemannZeta.lean)”；定义 `∀ (s : ℂ) (_ : riemannZeta s = 0) (_ : ¬∃ n : ℕ, s = -2 * (n + 1)) (_ : s ≠ 1), s.re = 1 / 2` | 02 | 截图里代码块被页面自身的横向滚动截断右端（页面如此，非裁切所致） | 已找到 |
| GRH 仅指 Dirichlet L 函数 | L3 | 同页 | “and four further theorems, including the analogous statement for all Dirichlet L-functions. "GRH" here means Dirichlet L-functions only.” | 02 | 五个定理：`riemannHypothesis`、`riemannZeta_eq_zero_re`、`LFunction_ne_zero_of_half_lt_re`、`LFunction_eq_zero_re`、`LFunction_eq_zero_re_of_isPrimitive`（audit.log） | 已找到 |
| 逐模块内核重放 | L3 | 同页 | “leanchecker kernel replay of every module — pass — 175,187 of 175,187; zero rejections; finished about 18:17 UTC” | 01 | 18:17 UTC = 北京 11-21 02:17 | 已找到 |
| 两个独立内核接受 | L3 | 同页 + 日志 | nanoda：“exit 0 at 19:05:45 UTC; 27 min 42 s on 80 threads”（`a-sutherland-nanoda-log.txt` 末行 “Exit status: 0”，Elapsed 27:42.28）。con-leche master（cd2eceab）：“1,468,418 declarations accepted in --verified mode; exit 0 at 21:19:04 UTC; 3 min 15 s on 64 threads”（日志 “done … 196.7s”） | 01 | 两者各取页面给出的 UTC：nanoda 北京 11-21 03:05:45，con-leche 05:19:04 | 已找到 |
| con-leche --compare 确认五个定理 | L3 | 同页 + `a-sutherland-con-leche-compare-log.txt` | “all five theorems "justified by the solution"; 51,692 challenge declarations accepted; exit 0 at 21:19:12 UTC” | 01 | 分支 `compare` @5dc36dc1；Breitner 在 Zulip 说该 PR 已合入 master（北京 09:04） | 已找到 |
| lean4lean 通过 | L3 | 同页 | “175,187 of 175,187; finished about 21:55 UTC. 175,182 passed with default settings. Five hit lean4lean's default step limit ("deterministic timeout") on decide +kernel computations and passed when rerun with the limit raised” | 01 | 5 个需调高步数上限才通过 | 已找到 |
| 仍在跑的项 | L3 | 同页 | “The lake comparator run and the Lean-kernel replay of the export are still in progress.” 表内：“Lean default kernel and Lean paranoid kernel on the export — running — single-threaded; no verdict yet”；“lake comparator (Lean v4.36.0) end to end — running — statement comparison and axiom check already passed in one run; kernels in progress” | 01 | 18:25 与 18:37 两次取档均未变；Sutherland 在 Zulip 北京 17:38 写 “comparator and paranoid runs on the full proof of RH are still in progress. They are unlikely to complete before tonight, more likely tomorrow” | 已找到 |
| 旧版 con-leche/con-ron “declined” 不是拒绝 | L3 | 同页 | “con-leche and con-ron as bundled with Lean v4.36.0 — declined — exit 2, not a rejection: those older binaries do not support one reflexive inductive type in the proof. con-leche master, above, does.” | — | Zulip 里 sbp（北京 17:52）称自行从源码构建 con-ron（de06320f）跑通，需调大栈（gist 链接，本组未取，L6） | 已找到 |
| 页面声明的信任边界 | L3 | 同页 “Caveats” | “A passing check shows that the formal statements, as written in Lean against Mathlib's definitions, follow from the three standard axioms according to the kernels used. It relies on those kernels and on Mathlib's definitions being what they are intended to be.”；另：“The build ran on cloud machines without a sandbox”“We have not ourselves compared its sha256 with an export made from a sandboxed lake build.” | — | 以上为页面自述的限定 | 已找到 |
| 另合并 Robin1984 推出 Robin、Lagarias 不等式 | L3 | 同页 | “robin_inequality and lagarias_inequality compile; #print axioms — pass — propext, Classical.choice, Quot.sound only”；con-leche master 与 --compare 接受；“lake comparator (Lean v4.36.0) on the combined export — running” | — | Sutherland Zulip 北京 17:38：已用 lake comparator 与 con-leche compare 验证此等价；“I think this effectively rules out any potential holes in the definitions used in the mathlib formulation of RH.”（他自己的判断，L2/L6） | 已找到 |

### A3　Lipponen 独立复核与 AI 导读

仓库 `https://github.com/aapolipponen/rh-lean-check-lumi`，存档 `a-lipponen-readme-raw.txt`、`a-lipponen-docs-readme.txt`、`a-lipponen-07.txt`、`a-lipponen-10.txt`，提交记录 `a-lipponen-commits.json`（GitHub API，取 18:25）。**最近提交** `82d91dbb` 2026-11-20T23:56:59Z（北京 11-21 07:56:59）“Dependency audit: riemannHypothesis uses 146,437 of 175,187 project modules; 292 declarations…”；README 状态行仍写 “updated 2026-11-20 22:30 UTC”（北京 11-21 06:30），表格里 “Statement and axioms of the five theorems … running”，但正文已含依赖统计，即状态行早于正文更新。18:37 复取 README 无变化。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| LUMI 上编译 175,187/175,187、0 个 Lean 错误 | L3（第三方复核者自述） | README | “All 175,187 modules compile, with no Lean error. Main.lean finished at 22:21 UTC on 20 November 2026.” | 12 | 22:21 UTC = 北京 11-21 06:21；“718 compile attempts were killed or failed for those infrastructure reasons and were retried; no attempt failed with a Lean error.” | 已找到 |
| `Main.olean`/`Statements.olean` 哈希与 Sutherland 一致 | L3 | README 表 | “Main.olean sha256 9146dfb4148e24d2252e04c50742f3e49dd126dfcc5006d10bf5f873ddf796ad and Statements.olean sha256 f8e0aed3c2d69390b6dd3bb24e13aa25e043a02ca42f021e3f6e4ec8b57fd6b1: identical to the hashes Andrew Sutherland published for his build” | — | 与 Sutherland 页 Provenance 栏的 Main.olean 哈希 9146dfb4… 相同 | 已找到 |
| 静态扫描无 axiom/sorry/admit/native_decide | L3 | README 表 | “no axiom, sorry, admit or native_decide in the source (results/static-scan.txt)” | 12 | 扫的是源码文本；Sutherland audit.log 环境里的 `Lean.ofReduceBool` 在 Lean 核心 Init 内 | 已找到 |
| 没读/没验证的范围写明 | L3 | README 表 | “Kernel replay (leanchecker) of every module — not run by us. Done by Sutherland”“Second, independent kernel — not run by us.” | — | 本仓库不重复内核重放 | 已找到 |
| `riemannHypothesis` 依赖 146,437 个项目模块（83.6%） | L3 | README | “riemannHypothesis depends on 146,437 of the 175,187 project modules (83.6%), 1,478,219 constants and 5,081 Mathlib modules. 28,750 project modules are imported by Main but not used by the proof.” 另：“This is a dependency count; it does not say which parts are new mathematics.” | — | 292 个声明逐个走过，“every one depends on propext, Classical.choice and Quot.sound only” | 已找到 |
| 不证明有人读懂 | L3 | README “What this does and does not establish” | “It would not establish that anyone understands the argument, and it trusts: the Lean 4.33.1 kernel and toolchain binaries, Mathlib's definitions (in particular RiemannHypothesis, riemannZeta and DirichletCharacter.LFunction), and the four definitions in src/Statements.lean, which a human should read.” | 03 | 该段在“full pass 才能 establish”的条件句之后 | 已找到 |
| AI 导读不是审稿报告 | L3 | README 与 docs/README | README：“It was assembled from read-only reviews of the source by AI agents (docs/reports/); every claim carries a marker saying how much was actually read. It is not a referee report and does not vouch for the mathematics.” docs/README：“The proof is Alpöge's and Claude's. Nothing here should be read as saying that the mathematics has been verified by a person” | 03 | docs/README 另写 “No human-readable proof exists beyond a four-page announcement.” | 已找到 |
| `True` 占位定义共 8 个，含 `UnusedSlot72` | L3 | docs/07-named-theorems.md | “GlobalTheory.Published.UnusedSlot72 : Prop := True … appears in literature.json, so the count of about 155 Published statements includes at least one placeholder. Harmless, but it should not be counted as a literature input.”；“Eight definitions are Prop := True … This is inside the upper theory, not in a Published statement, and does not affect soundness of the final theorems; but those names should not be taken at face value.” | 04 | 截图为 GitHub raw 的 Markdown 源码视图（渲染页过长）；“does not affect soundness”是 AI 阅读者的结论（R4 §3），不是复核结论；“Limitation: the regex misses …No second search was run.” | 已找到 |
| 105 条命名定理“无一被判 vacuous” | L3 | docs/07 | “No row in any review was judged vacuous.”；判定仅回答 “does the Lean statement say what its name claims”，“says nothing about whether the proof is correct”；65 faithful / 29 special case / 7 weakened / 4 undetermined | — | 同页：155 条 Published 中另有 56 条连“名实相符”都未审（docs/README 原句 “56 of the 155 named Published statements have not been reviewed even for whether they state what their names claim”） | 已找到 |
| 无人读过的清单（P1） | L3 | docs/10-open-frontier.md；docs/README | docs/README：“Not read by anyone, as far as these reports know. Most of the argument below the top levels. In particular: the bridge from the automorphic theta-lift kernels to p-adic power series behind the key divisibility statement; the Taylor–Wiles patching theorem and linking argument behind the modularity input of the window bound; the proof that twisted Selmer groups can be made nearly trivial; the local terms of the trace comparison for the Galois representations.” docs/10 P1 第 1 行：“KGLt.first_kernel_law_global_small … Read only to line 130” | — | 该清单出自 AI 阅读报告 | 已找到 |

### A4　Lean Zulip 讨论串

直接访问 `https://leanprover.zulipchat.com/#narrow/…` 返回的是 JS 壳页（“Public view of Lean”）。改用该站 Web 公开版自己使用的匿名接口（`/json/messages`，narrow 含 `channels: web-public`，无登录、无凭据），按页取串全文：`a-zulip-alpoge.json/.txt`（讨论串 “Discussion: Alpöge Proves the Riemann Hypothesis”，465 条，北京 2026-11-20 21:58:06 至 11-21 18:25:04）与 `a-zulip-masullo.json/.txt`（“Formalising Masullo's proof of GRH”，106 条，11-21 03:16:18 起）。仅存发言人显示名、时间与正文，已不含邮箱与头像。`leanprover-community.github.io/archive/` 首页可打开但滞后，未使用。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 版主把讨论限定在 Lean 方面 | L2（署名版主发言） | Zulip 消息 642305635（Kevin Buzzard，北京 11-21 02:39:29） | “This is not relevant to Lean. With my moderator hat on (and speaking as someone who has already had to delete tens of messages in the last few hours) let me remind people that posts which are not about Lean are off-topic for this Zulip.” | — | 另：Rémy Degenne（01:12）“keep this discussion on topic. No meme, no random pictures. We deleted a few messages already.”；Riccardo Brasca（01:58）“please don’t start parallel discussions”；Michael Rothgang（03:30）“AI-generated comments are disallowed here.”；Kim Morrison（09:46）要求引用 AI 文本须标明。Tao 博文的 “UPDATE” 与此对应 | 已找到 |
| 开串时间与首帖 | L2 | 消息 642224678（北京 11-20 21:58:06） | “Alpöge claims a full, unconditional Lean proof of the (generalized) Riemann hypothesis. I'm compiling it now on two separate devservers to verify the claim.” | — | 首帖称文件“Uploaded less than an hour ago”；PDF 创建 20:52:48 BJT | 已找到 |
| 有无人报告 Lean 层面的问题 | L2/L6 | 全串关键词扫描（kernel bug/soundness/reject/vacuous/native_decide/sorry 等；**未逐条通读 465 条**） | 未发现任何人报告“被内核拒绝”或“用到公理漏洞”。有人问“利用旧内核漏洞”，Floris van Doorn（22:30）答：“It uses the Lean toolchain where all the kernel bugs that were mention in Leo's blog post … were fixed. I don't recall a kernel bug after that (but there might be one).”；Joachim Breitner（11-21 05:10）“Multiple people have built and checked the development with multiple checker, and it all looks good.”；“some other checkers didn't get through (neither is a rejection)”（Szymon Teżewski 05:02，nano-lean 解析失败、gruszka 卡在两个声明；09:56 他称 gruszka 修复后 4.5 分钟接受） | — | Thomas Murrills（09:51）：去掉未用导入时发现 4,660 处“死代码”例子，推测是 AI 用来“check its work”（L6） | 部分支持（只做了关键词扫描） |
| 多个独立内核接受 | L2 | 消息 642355040（Szymon Teżewski，北京 06:06） | 表列 con-leche --compare（3:35，64 线程）、con-leche @cd2ecea（3:24）、con-agua（14:34）、Pestka（10:22）、nanoclo（33:43）、tenet（45:50）均 accepted，导出 sha256 5ea74b91…244e | — | Elliot Arledge（06:06）另报从零构建 + nanoda 通过，1,484,729 个声明；Sutherland 页用的是 nanoda 与 con-leche | 已找到 |
| Sutherland 用 AI 编排 | L2 | 消息 642277830（Andrew Sutherland，北京 00:55:59） | “The summary below is from Fable 5.2 which orchestrated the computation” | — | 页面写 “checks run with Claude Code” | 已找到 |
| Masullo 串的版主态度 | L2 | Masullo 串消息 642326773（Kevin Buzzard，北京 04:05:57） | “I have objections to using this particular forum as a place to deconstruct someone's proof in public. This is not about Lean. Furthermore the author of the purported proof did not ask for it to be disassembled in this public way. Mathoverflow does not allow this kind of content and I don't think we should either. I suggest you take it to Reddit.” | — | Floris van Doorn（04:13）引 Vakil 原句 “I hope people quickly check it, formalize, fill any small gaps” 反驳 | 已找到 |

### A5　Tao 博文

`https://terrytao.wordpress.com/2026/11/20/unforced-euler-singularity-unforced-navier-stokes-singularity-and-the-generalized-riemann-hypothesis/`，存档 `a-tao.html/.txt`。元数据：`article:published_time` 2026-11-20T17:19:00+00:00（北京 11-21 01:19:00），`modified_time` 2026-11-20T19:13:39+00:00（北京 11-21 03:13:39）。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Tao 对 RH 的原话 | L2（署名学者个人博客） | 同上 | “Levent Alpöge, using an internal AI model, has generated an (enormous) Lean-only proof of the Generalized Riemann Hypothesis. This Lean Zulip thread is probably the best place to follow the efforts to verify the proof and extract the strategy of proof (which, from very initial analysis, appears to use the full power of modern developments on the Langlands program and arithmetic geometry). (UPDATE: the moderators of the Lean thread are now focusing the discussion on the Lean-related aspects of the proof only.) I imagine the digestion process will take some time, though in this case at least we have a human author to communicate with to assist with this task.” | 08 | 写 “internal AI model”，没有写 Claude 或 Anthropic；写 “Generalized Riemann Hypothesis”，没有提 Dirichlet 范围；同帖另两项（Euler、Navier–Stokes 爆破）归 C 组 | 已找到 |
| Tao 评论区回复（RH 相关） | L2 | 同页评论 #699598（Terence Tao，“20 November, 2026 at 11:06 am”） | “The conventional wisdom has been that purely analytic methods are not sufficient to establish RH … It is very early days but it seems like the AI-generated proof uses all of these inputs. There has also been the wildcard of “F_1” approaches, as proposed by Alain Connes; the new Masullo manuscript apparently goes down this route to some extent, but it will take some time to digest that manuscript also. [EDIT: on closer inspection, the Masullo manuscript is closer to Deninger’s strategy towards RH than Connes’, although the two approaches have some similarities (though neither is close to being concretely fleshed out).]” | — | 评论时刻为博客站点时区，按首条评论 9:34 am 早于文章 17:19 UTC 推断为 UTC-8（**推断**，换算北京 11-21 03:06） | 已找到 |
| 评论 #699631（Tao）答“AI 能否同样解决某等式蕴含问题”时列出的理由 | L2 | 同页 | “The RH proof uses a century of past literature on related topics.”“We don’t know how much compute in aggregate has been spent on both problems” | — | 与本主帖仅旁涉 | 已找到 |

### A6　Aaronson 博文与评论 #35

`https://scottaaronson.blog/?p=10344`，存档 `a-aaronson.html/.txt`。页面写 “posted on Friday, November 20th, 2026 at 12:29 pm”；元数据 `published_time` 2026-11-20T18:29:03+00:00（北京 11-21 02:29:03，可推出页面时间为 UTC-6），`modified_time` 2026-11-20T22:20:32+00:00（北京 06:20:32）。共 73 条回复。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 标题与置顶更新 | L2 | 同上 | 标题 “The stochastic parrots have apparently proved the Riemann Hypothesis”。置顶 “[Update: According to “Poe’s Law,” there is no irony or sarcasm on the Internet so obvious that it won’t fly over countless people’s heads. In line with that, lots of people on social media are now saying variants of “how dare Scott Aaronson call LLMs ‘stochastic parrots,’ … ROFL.]” | 09 | 置顶更新讲的是标题的反讽被读歪，不是证明的状态 | 已找到 |
| 句 1 | L2 | 同上 | “Mathematicians of course won’t be satisfied with 17 million lines of Lean. They’ll be satisfied when, and only when, they’re able to explain and distill and reconstruct the ideas for themselves. And that will probably take years, since human mathematicians move only at biological speeds.” | 09 | 全文有此句，Tao 博文无 | 已找到 |
| 句 2 | L2 | 同上 | “Many will of course question whether the proof announced today will stand. They’ll point out that, despite how heavily the world is now “lean-ing” on it, the Lean formal verification system still has issues, from kernel bugs (which have, in the past, allowed proofs of 0=1) to uncertain soundness of the underlying type theory. In my view, these issues can and should urgently be fixed!” | 09 | 这是他转述“许多人会提出的质疑”，随后他说 “I see zero indications that that’s happening right now” | 已找到 |
| 句 3 | L2 | 同上 | “I note that, so far, not one of the AI results with Lean certificates has needed to be retracted (while several without Lean certificates have needed to be). So I’m not 100% sure, but my bet is that the proof will stand.” | 09 | 个人押注，带 “I’m not 100% sure” | 已找到 |
| Aaronson 对来源的写法 | L2 | 同上 | “I woke up this morning to the news that Levent Alpöge says that he’s gotten Anthropic models to prove the Generalized Riemann Hypothesis, producing 17.4 million (!!) lines of Lean code.”（链到 `github.com/CocoSgt/riemann-hypothesis-lean`） | 09 | “Anthropic models”与 Alpöge 原话 “working with Claude”、Tao 的 “internal AI model” 措辞不同 | 已找到 |
| 评论 #35：27 个 “Return here”，页码 199–483 | L6（署名 “Science Markets”，评论自披露 “I run science.markets, which has a market on whether this proof holds”） | 同页评论 #35（“November 20th, 2026 at 6:12 pm”，页面时区 UTC-6 → 北京 11-21 08:12） | “There are 27 footnotes that say only “Return here” – his own markers for loose ends – spread across pages 199 to 483. Two more specific ones read “complete proof” and “after the Proposition is updated.”” | 07 | **本组在 PDF 上核**：纯 “Return here.” 脚注 27 处，页码 199–483（199、200、234、236、250、294、295×2、414×2、415、416、417、426、435×2、436、437、438、446、462、475×2、480、482×2、483）；另 3 处变体：464 页 “Return here and complete proof.”，514 页 “Extend Definition 9.3.3.1 … Return here and change the choice of smooth (λ, ε)-cutoff into a smoothening …”，515 页 “Return here after the Proposition is updated.”，合计 30 处（`pdftotext -layout`，已对分行写法 `Return\s+here` 复核，仍为 30）。评论称“两处具体变体”，实际是三处 | 已找到（计数相符；变体数 2 vs 3 有出入） |
| 评论 #35：第 13 章两页、不陈述定理 | L6 | 同上 | “Chapter 13, which is supposed to deduce GRH from the trace formula, is two pages and states no theorem.” | 11 | **本组核**：第 13 章在 PDF 第 521–522 页（目录写 13.1–13.3 起于 521–522，第 14 章起于 523）。第 521 页是章名与小节目录，第 522 页正文：13.1、13.2 两个小节只有标题没有正文；13.3.1 重述 “Theorem 13.3.1.1 (Trace Formula)”；“Upon combining Theorem 13.3.1.1 and Theorem ??”；“Theorem 13.3.1.2 (Divisor of L(s, χ))” 之后无内容，脚注 “Correct statement.” “Insert statement.” “State and prove the Main Theorems, also explaining how to deduce them by approximation φ → e^{−s|log(t)|} as Re(s) > 1.” 因此“没有关于零点位置的主定理”成立，但“states no theorem”字面不准——重述了第 12 章的迹公式 | 部分支持 |
| 评论 #35：方法几乎不共享（词频） | L6 | 同上 | “Search the 940 pages for those and you find: “Eichler-Shimura” 0 times, “CM field” 0, “Hilbert-Samuel” 0, “Moriwaki” 0.” | — | 本组只核了这四个词在 pdftotext 中均为 0 次；“trace formula 203 次”等其余词频未复核（统计口径不同，我数得 “trace formula” 164、“explicit formula(s)” 61） | 部分支持 |
| 评论 #35：McCorvie 勘误“11 条” | L6 | 同页 | “Ryan McCorvie started a Lean formalization today and his errata file already lists eleven statements that are false as written, all in the appendices’ recalls of standard material.” | — | 本组 18:31 取档 `a-mccorvie-errata.md`：“False as stated” 表现有 F1–F18 共 18 行（另有 “Imprecise, inconsistent or wrongly justified” 等表）；文件自述 “Most entries were first reported by an AI agent … No entry has been reviewed by a specialist.” 数字随时间在涨，评论发于 08:12 北京 | 部分支持 |

### A7　Vakil 公布与 Masullo 稿

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Vakil 公布、“all human, with no AI”、Conrad 看了几个月 | L2（署名学者；Conrad 部分是 Vakil 转述） | https://math216.wordpress.com/2026/11/20/hurried-announcement-alessandro-masullo-grh/ ，存档 `a-vakil.html/.txt` | “Alessandro Maria Masullo has a proposed proof of GRH, and Brian Conrad has been looking closely at it for months and gave extensive feedback which is being incorporated, and it is so far looking good to him. It is all human, with no AI.” | 05 | 发布 2026-11-20T18:18:42+00:00（北京 11-21 02:18:42）；“looking good to him”是 Vakil 转述 Conrad 的看法，不是 Conrad 本人发言；Quanta 的引文省去了 “to him”（见 A11） | 已找到 |
| 8 小时后更新 | L2 | 同页 | “Update about 8 hours later: I’m out of the meetings, and now catching up! Several people have contacted me to report problems in the proposed proof, identified with the assistance of different AI models. (Thanks to several mathematicians in particular who were first to let me know.) As is usual, all issues need to be resolved to the satisfaction of the community before this proof would be accepted as true. (I make no claim as to the correctness of the proof strategy.)” | 05 | 页面 `dateModified` 2026-11-21T02:54:54+00:00（北京 10:54:54）；更新段本身未标具体时刻（官方未给时刻）。“problems … identified with the assistance of different AI models”是 Vakil 转述他人来信，未给问题的具体内容 | 已找到 |
| 稿件：940 页、创建时间、标题 | L1（作者稿件，经 Vakil 站点发布） | https://math216.wordpress.com/wp-content/uploads/2026/11/the-music-of-the-primes.pdf ，`a-masullo-pdf.pdf` | pdfinfo：Pages 940；Creator “LaTeX with hyperref”；CreationDate Sat Nov 21 02:13:46 2026 +08；`SHA-256 e039d3f6cf0a4b14f03c7edb85097752dd65c0a2fe543c76fe992be234f97bf5`。第 1 页：“The Music of the Primes — Alessandro Maria Masullo”，无摘要；PDF 元数据 Title/Author 为空 | — | 本稿将来可能更新，哈希用于比对；`.txt` 4.1 MB（>1 MB，见回报） | 已找到 |
| 致谢里的开始时间、Conrad、Stanford | L1 | 稿件第 6 页 | “This work started off when the author was nearing the end of his first year as a Graduate Student at Stanford under Brian Conrad. It took nearly 12 years to assemble. Brian Conrad and Ravi Vakil were in the process of making arrangements for me to return to Stanford for a year to finish the polishing of the paper.” | 06 | Quanta 称其 LinkedIn 显示为 Stanford postdoctoral scholar（L4 转述）；TechCrunch 写 “Stanford postdoc”；稿件自述只说“曾是 Stanford 研究生”，未写现职 | 已找到 |
| 作者自述未完成 | L1 | 稿件第 6 页 | “The math is complete, and only filling in some writing remains on several parts across this long paper.”“The introduction itself is unfinished and will be soon replaced by a proper one”“Brian Conrad and I are aware of several glitches here and there that I was in the process of addressing on the way to completing the typewriting. The ideas are however all here, and many of the chapters are already fairly stable.” | 06 | “math is complete”是作者主张，不是复核结论 | 已找到 |
| 适用范围：Hecke L 函数 | L1 | 稿件第 6 页；第 522 页 | 第 6 页：“The excerpts from the letter to Sarnak explain the overall strategy (generality: all number fields, all unitary Hecke characters).”第 522 页（第 13 章开头）：“In this Chapter, we show how to deduce information about the zeros of Hecke L-functions of an arbitrary number field F, from the Trace Formula proved in Theorem 12.5.1.1.” | 06、11 | 稿件全文不含 “Generalized Riemann” 字样，只有缩写 “GRH”（第 16、30、31、35、40、52 页，如 “1.4.8. GRH is a topological phenomenon.”）。结论定理在第 13 章未写（见 A6） | 已找到 |
| 导言与相关章节状态 | L1 | 目录与导言 | 目录：1.6 “How this work was conducted, and why” 的小节 1.6.1、1.6.2 在第 33 页只有标题无正文；多处 “Theorem ??”（第 33、69、88、178、234、522、528、533 等页含 “??”） | — | 以 pdftotext 计 | 已找到 |
| “Return here” 总数与页码 | L1 | 稿件各页脚注 | 见 A6 | 07 | 共 30 处（27 纯 + 3 变体），见 A6 | 已找到 |

### A8　Masullo 稿被报告的问题（全部 L2 或 L6；不判断数学对错）

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Elliot Glazer：Masullo 稿“没过 Astra 审计” | L2（署名公开发言，经 fxtwitter） | https://x.com/ElliotGlazer/status/2123853740574945473 ，`a-fx-2123853740574945473.json` | “Followup: Fails an Astra audit. Come on what were these guys doing the past few months🤦‍♂️” | — | 2026-11-20 18:59:25 UTC（北京 11-21 02:59:25）；`replying_to_status` = 2123845630124847322（他 18:27:12 UTC 的帖：“Btw Alessandro Masullo proposed a proof of GRH several months ago that has been under review by Brian Conrad "... and it is so far looking good." [Vakil 博文链接]”，`a-fx-2123845630124847322.json`）。本帖自身没有写出 Masullo 或 Astra 的审计方式 | 已找到（指向靠回复链） |
| AcerFur（自述前 OpenAI 数学雇员）：“GPT quickly finds some errors” | L6 | https://x.com/AcerFur/status/2123853069599547786 ，`a-fx-2123853069599547786.json` | “GPT quickly finds some errors, but idk how irreparable it is”（引用 18:26:40 UTC 帖 “Someone just shared this with me lol” + Vakil 博文链接） | — | 2026-11-20 18:56:45 UTC；HN 评论 50580084（markasoftware）转述；“前 OpenAI 员工”身份取自 HN 评论，本组未核 | 已找到 |
| teapawt.org 页面 | L6（第三方 X 镜像，非 X） | https://teapawt.org/ElliotGlazer | 个人页列出上述两帖：“Followup: Fails an Astra audit …” 的页面时刻显示 “Nov 20, 2026 · 6:59 PM UTC” | — | 本组存的是 fxtwitter 的 JSON；teapawt 页只用于确认帖子存在，未存档（页面里含他人帖子） | 已找到 |
| Reddit 上 “Theorem 11.1.2.1(2) 有反例”的 Astra 审计摘录 | L6 | Reddit（`/r/math/comments/1y1b6l8/…`，Zulip 里 Snir Broshi 北京 04:09 给的短链） | 无法读取：匿名访问被 Reddit 登录墙/403 挡回（`barking heat` 4 次搜索、短链、old.reddit.com、`.json` 均失败） | — | 不登录、不绕过 | 未找到一手来源 |
| 同一指向的替代来源 1：HN 评论 | L6 | https://news.ycombinator.com/item?id=50581927 （schleck8，2026-11-20T20:55:03Z=北京 11-21 04:55） | “But the latter seems to include several flaws too, most notably in theorem 11.1.2.1(2) on p. 480 onward.” | — | 评论没有给出反例内容；上级评论 50581192（demibabs）给了 `philpapers.org/archive/NIEIOM.pdf`，称 “This write up is claiming his proposed proof is wrong.”（该链接本组打不开，见 A9） | 已找到 |
| 同一指向的替代来源 2：Refine 自发布报告 | L6（公司自发布，自述 “We used the Refine harness and all frontier models to prepare this report”） | https://github.com/Refine-Technologies-Inc/music-of-primes-report-by-refine ，`a-refine-tex.txt`（report.tex） | Issue 1：“Chapter 11's truncated prime-shell filtration provably loses the primes”，位置 “Theorem 11.1.2.1(2), p. 480; Proposition 11.2.0.1(2), p. 484; §11.2.2; used on pp. 495–496 and 519–520”；总述 “A detailed audit identifies ten substantive gaps and errors … in its present form, the manuscript does not establish the claimed results.” | — | 仓库两次提交 2026-11-20T21:12:42Z、21:20:43Z（北京 11-21 05:12、05:20）；Zulip Brent Yee 北京 06:38 贴出同一仓库与 “Error on page 444…” | 已找到 |
| 该定理编号在稿件 480 页附近存在 | L1 | 稿件第 480 页 | “Theorem 11.1.2.1 (Existence of S-local carvings on the 0-line).”(1)(2) 两条，(2) 含 “Let i, i′ ∈ I_S be distinct, with S_i ⊂ S_i′ and such that there exists a prime number p such that no place v ∈ S_i divides p, and at least one place in S_i′ does. Then we have the inclusions of closed linear subspaces:” | — | 该编号另见第 495 页被引用；仅确认编号与位置，不判对错 | 已找到 |
| 公开可见的核心争议：Theorem 12.5.1.1（迹公式） | L6 | Zulip Masullo 串，Tim Gehrunger（北京 03:32） | “I had a quick go at having an LLM look at Masullo's proof, and it seems to think that the Main Theorem (12.5.1.1 in the notes) is incorrect and has a pretty straight forward counterexample.”；03:42：“The correction of one of the errors reduces the statement of the main theorem of the article to a classical formula; at least that is what Astra and Fable seem to agree on.” | — | 未贴细节。McCorvie 仓库 README（北京 18:31）：“Concerns about the main theorem … the details have not been posted. They did not come from this formalization”；该仓库 06:41 的旧版状态为 “On hold”，18:31 版为 “Active: formalizing the prerequisites” | 已找到 |
| McCorvie ERRATA.md | L6（AI 阅读 + 整合代理，未经专家） | https://github.com/mccorvie/riemann-masullo ，`a-mccorvie-errata.txt` | 开头：“Most entries were first reported by an AI agent … No entry has been reviewed by a specialist.”“Everything below so far is in the appendices and early chapters that recall standard material, and none of it touches the trace formula.” | — | “False as stated” 表 F1–F18（取档 18:31）；条目随仓库提交更新（最新提交 2026-11-21T10:31:22Z，北京 18:31） | 已找到 |

### A9　公开反驳稿

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Nielsen 反驳稿 | L6 | https://philarchive.org/rec/NIEIOT-2 ；同一记录 https://philpapers.org/rec/NIEIOT-2 ；PDF https://philpapers.org/archive/NIEIOM.pdf（HN 评论 50581192 所给） | 三个地址 18:31 BJT 均返回 403 + Cloudflare 质询（`cf-mitigated: challenge`），存档的是质询页（`a-nielsen-*.html`），无正文 | — | 不绕过质询。旁证：Zulip 北京 18:22 Timothy Chow 答 Bogdan Grechuk 的提问 “Is it a serious claim?”：“No, it is not a serious claim.”（Zulip 消息 642458394；他对作者的其余评语本组不记录） | 未找到一手来源 |
| Islam（Zenodo） | L6 | https://zenodo.org/records/24404855 ，`a-islam-zenodo.html/.txt` | 标题 “The Bit in the Definition: Two Derivations of the Riemann Hypothesis in One Day, Their Tautology, and the Public Record They Land In”；“Published November 20, 2026”，Version 1.0.0；作者 “Islam, Mohammad F.”。摘要：“Each premise is a definition, which a law of logic proved in our file classifies as strictly stronger than the value: both derivations are tautologies of their premises, and the standard axioms carry nothing.” 另：“We read both against the one-bit normal form of the Riemann Hypothesis, public in our git repository since October 2026 … and filed on Zenodo on 16 October 2026.” | — | 作者自述其“一位范式”早于两份证明；页面未给作者职业身份（有 ORCID 链接）；自发布，未经评审；摘要点名 “Alpöge decides the bit on the prime side of Weil's explicit formula, Masullo on the zero side” | 已找到 |

### A10　Anthropic 有无官方表态

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Anthropic 官方新闻页 | L1 | https://www.anthropic.com/news ，`a-anthropic-news.txt`（取 18:29 BJT） | 最新条目为 Nov 20 “Anthropic announces closing of initial public offering …”（及 Nov 18 定价、Nov 10 roadshow）；无 Riemann、Alpöge 条目 | — | 只看了新闻页列表，未逐一打开其他栏目（Research 等） | 未找到一手来源 |
| Anthropic 官方社媒 | L6（X 镜像 teapawt） | https://teapawt.org/AnthropicAI 与 `/claudeai`、`/__alpoge__`、`/DarioAmodei`（取 18:30 BJT，未存档页面） | @AnthropicAI 最新为 Nov 20 16:57 UTC “Anthropic’s co-founders on the story so far”，之前为 IPO 相关；@claudeai 最新 Nov 19；@DarioAmodei 最新 Oct 21；@__alpoge__ 个人页最新自发帖为 Nov 11 | — | 镜像可能滞后或不完整，只能说明“镜像里未见”，不能证明官方没发 | 未找到一手来源 |
| 媒体记 Anthropic 未回应 | L4 | AP/Boston Globe：https://www.bostonglobe.com/2026/11/21/nation/anthropic-worker-solve-math-problems-ai-riemann-hypothesis/ （`a-bostonglobe.txt`）；Nature：https://www.nature.com/articles/d41586-026-03812-7 | AP：“Anthropic did not respond to an email from The Associated Press requesting comment on Alpöge’s claim.”Nature：“Anthropic did not respond immediately to Nature’s request for comment.” | — | 均为媒体对“未回应”的记述，不是 Anthropic 发言人引语；没有找到任何发言人引语 | 已找到（媒体记述） |
| Alpöge 自述与 Anthropic 领导层磋商 | L2 | remark.pdf 第 2 页 | “I messaged Dario, "Hypothetically, if I were to have . . . ", eventually asking about my ability to announce. We went and figured it out and the fact is that there was a constraint to respect … Our leadership and the few trusted mathematicians I consulted hold extremely similar views on the right things to do in these circumstances” | 10 | 这是员工自述，不是公司表态 | 已找到 |

### A11　媒体与夸大实例

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Scientific American 报道 | L4 | https://www.scientificamerican.com/article/anthropic-mathematician-claims-to-prove-riemann-hypothesis-most-famous-problem-in-all-of-math/ （`a-sciam.*`） | 标题 “Anthropic mathematician claims to prove Riemann hypothesis, most famous problem in all of math”（Joseph Howlett）；“Alpöge says his proof confirms the “Generalized Riemann Hypothesis,”” | — | `datePublished` 2026-11-20T09:00:00-05:00（北京 11-20 22:00），`dateModified` 2026-11-20T23:26:31Z（北京 11-21 07:26）；正文已更新为含 Sutherland 编译结果。标题带 “claims” | 已找到 |
| Nature 报道 | L4 | https://www.nature.com/articles/d41586-026-03812-7 （`a-nature.*`） | 标题 “Mathematician says he and AI solved Riemann hypothesis — the field’s biggest outstanding challenge”（Davide Castelvecchi）；“Andrew Granville … says that Alpöge should have taken the time to prepare a clearer write-up before making the announcement”；“What he released is garbled,” Granville says | — | 日期 2026-11-20（无时刻）；付费墙后半段未取 | 已找到 |
| Quanta 实时更新页 | L4 | https://www.quantamagazine.org/updates/transformation/ （`a-quanta-updates.*`，取 18:28 BJT） | “Several independent efforts have now successfully compiled Alpöge’s proof. Andrew Sutherland … was the first”；Radziwiłł：Alpöge 的四页稿含 “nonsensical statements”；“(Since a proof of Riemann that he was working on several years ago did not, in the end, hold up, many number theorists are withholding judgement pending further scrutiny.)”；“This is a breaking news story, Quanta will publish further updates if and when independent mathematicians verify the Lean code.” | — | 页面是持续更新的条目集合，条目时刻见页面内各更新（本组未逐条取时刻）。引 Vakil 时写 “…and it is so far looking good.”，省去了 Vakil 原文的 “to him” | 已找到 |
| TechCrunch：标题把两件事并列 | L4 | https://techcrunch.com/2026/11/20/ai-claimed-a-solution-to-the-167-year-old-riemann-hypothesis-and-a-human-did-too/ （`a-techcrunch.*`） | 标题 “AI claimed a solution to the 167-year-old Riemann hypothesis — and a human did, too”（Tim Fernholz）；正文 “Ravi Vakil, a Stanford professor and the president of the American Mathematics Association”“a proof of the General Riemann hypothesis written by Stanford postdoc Alessandro Masullo”“Without being expressed in Lean, it is too early to say whether the proof checks out” | — | 发布 2026-11-20T19:32:08Z（北京 11-21 03:32:08），修改 22:51:18Z（北京 06:51）；“American Mathematics Association” 与 Vakil 博文自述的 American Mathematical Society 不符；文章未写 Vakil 的 8 小时后更新 | 已找到 |
| 夸大实例（中文）1 | L5 | https://finance.sina.com.cn/tech/roll/2026-11-21/doc-inizmwan6831231.shtml （新浪科技，`a-sina.*`） | 标题 “刚刚，Claude 攻克黎曼猜想？Anthropic 上市第二天，167 年难题或将告破！”；正文 “他和 Claude 一起找到了黎曼猜想的证明，还顺带解决了范围更大的广义黎曼猜想。这份证明已经用 Lean 写好并公开，但还没有经过专家验证。” | — | 发布 2026-11-21T06:09:36+08:00。标题以 “攻克” 作谓语并带问号；正文把 Alpöge 的 “I believe … resolving the so-called GRH” 写成“顺带解决”，且写“没有经过专家验证”，与同日 Sutherland 页（已编译、内核接受）不一致 | 已找到 |
| 夸大实例（中文）2 | L5 | https://www.sina.cn/weibo/detail/5367601423190555.html （新浪新闻，`a-sinacn-weibo.*`） | 标题 “AI 写出 25 万页证明，陶哲轩：验证仍需时间” | — | 发布 2026-11-21 08:02；“25 万页”是 Alpöge 自述“amused myself, and hit 250,000+ pages” 的未公开稿，标题当成 AI 写出的“证明”并与 Tao 并列；Tao 博文里无“25 万页”（只在评论中有他人转引） | 已找到 |
| 夸大实例（中文）3 | L5 | https://www.163.com/dy/article/LC8C5H7T0511CSAO.html（网易订阅，转爱范儿早报，`a-163-tao.*`）、https://www.ifanr.com/1690759 （`a-ifanr.*`） | 标题 “曝 Claude 证明黎曼猜想，陶哲轩回应”；正文 “菲尔兹奖得主陶哲轩也于 11 月 20 日在个人博客提及此事” | — | 爱范儿发布 2026-11-21 08:25:08（+08:00），网易 08:23:27；Tao 博文并无“回应”传闻，只是顺带记了一项；标题的 “证明” 谓语无 “宣称” | 已找到 |
| 夸大实例（中文）4 | L5 | https://news.qq.com/rain/a/20261121A047KJ00 （腾讯新闻，`a-qq-0121.*`） | 标题 “‘数学界第一神山’被攻克？Anthropic数学家宣称证明黎曼猜想，给出1700万行Lean源码，AI证明文件超过25万页” | — | 发布 2026-11-21T09:16:16+08:00；标题带 “宣称”，但 “AI证明文件超过25万页” 写成既成事实；正文：“Elliot Glazer初步检查发现，Lean文件中形式化的数学命题确实是广义黎曼猜想，并估计Lean自身存在漏洞的概率‘远低于1%’”（本组未在 X 上找到这条原帖，Glazer 个人页里只见其他帖，故此句 **未核实**） | 已找到 |
| 把 Aaronson 原话安在 Tao 头上的标题 | — | 查过上述中文稿、WebSearch 两次（中英文关键词，含 “Mathematicians of course won't be satisfied with 17 million lines of Lean”） | 未找到 | — | 检索面有限（搜索引擎摘要加 10 篇存档），不能证明不存在；扫描里的误署出自 WorkBuddy 扫描文本，未见媒体复现 | 未找到一手来源 |

### A12　热度（`barking heat`，取 18:30:45 BJT；见 `a-heat.json`、`a-hn-<id>.json`）

| 帖 | 标题 | 分数 | 评论（含嵌套） | 提交（UTC → 北京） |
|---|---|---|---|---|
| 50574968 | Levent Alpöge claims proof of the Generalized Riemann Hypothesis [pdf] | 2647 | 1810 | 2026-11-20 14:16:21 → 22:16:21 |
| 50579568 | Hurried Announcement: Alessandro Masullo GRH | 2082 | 926 | 2026-11-20 18:33:14 → 11-21 02:33:14 |
| 50578358 | Unforced Navier-Stokes Singularity and the Generalized Riemann Hypothesis（Tao 博文） | 447 | 180 | 2026-11-20 17:21:36 → 11-21 01:21:36 |
| 50581084 | The stochastic parrots have apparently proved the Riemann Hypothesis | 296 | 246 | 2026-11-20 20:03:07 → 11-21 04:03:07 |

Reddit：`r/math` 等四次搜索与短链均被登录墙/403 挡回，未取得数据（`a-heat.json` 的 errors）。

HN 评论的分数在 Algolia 数据里为 null，无法按得分排序；下面按回复数（reply count）挑，仅作线索（L6）：

- 50581412（Aurornis，Masullo 帖，约 150 条回复）：认为 12 年工作者在 Anthropic 公告后“hurried announcement … is the only way to claim a tie to the discovery”（动机推测，无证据）。
- 50580084（markasoftware，Masullo 帖）：转述 AcerFur 推文称稿件有问题（见 A8），“Maybe repairable”。
- 50583510（swingboy，Tao 帖）：“how can we tell when the Lean build “passes” that it is actually formally verifying the right thing and there wasn’t just a “1+1=2” making it pass?”——对应的回答在 Zulip：Floris van Doorn（23:01 BJT）“The challenge should be import Mathlib theorem riemannHypothesis : RiemannHypothesis := sorry”，Sutherland 页的 comparator/con-leche --compare 即在做此事。
- 50579624（AndrewKemendo，Tao 帖）：提醒该帖是 Tao 自己写，而不是转贴他人（对应“把别人的话安到 Tao 头上”的风险）。

## 三、可能的吠点（说法 vs 证据；仅线索，进正文前须回一手来源）

1. **“AI 证明了黎曼猜想” vs 谁说的、说了什么**：说法来源是一名 Anthropic 员工自己的声明，原句是 “I believe I have … discovered a proof”（remark.pdf 第 1 页），没有 Anthropic 官方表态（A10）。内核层面：Sutherland 页写 5 个定理依赖 3 条标准公理、多个独立内核接受；**未验证**：lake comparator 全程与 Lean 官方内核重放仍在跑（页面 `running`）。
2. **“没人读懂”不是比喻**：作者自述 “before having mastered a massive neighbourhood around its mathematics”（第 2 页）；Lipponen 的 AI 导读自称 “not a referee report”，列出“not read by anyone”的中间步骤（docs/README）；README 原句 “It would not establish that anyone understands the argument”。Aaronson：“Mathematicians … They’ll be satisfied when, and only when, they’re able to explain and distill and reconstruct the ideas for themselves.”
3. **两份 “GRH” 不是同一个范围**：Alpöge 的定理是 Mathlib 的 RH + Dirichlet L 函数（Sutherland：“"GRH" here means Dirichlet L-functions only.”）；Masullo 声称 “all number fields, all unitary Hecke characters”（稿件第 6 页），但推出零点位置的第 13 章只有两页、主定理未写。“Generalized Riemann Hypothesis” 在 Tao 博文、SciAm 里没有区分。
4. **“人工那份”也不是“人没用 AI”的反面**：Vakil 原句 “It is all human, with no AI” 指作者写稿；8 小时后的更新是 “problems … identified with the assistance of different AI models”，即问题由 AI 辅助挑出，Vakil 未给具体问题，且 “I make no claim as to the correctness of the proof strategy”。
5. **“Conrad 看过”的限定**：Vakil 原句是 “so far looking good to him”（转述 Conrad，且 “so far”），稿件自己说 “Brian Conrad and I are aware of several glitches here and there”。Quanta 引文丢了 “to him”。
6. **稿件未完成的证据是它自己写的**：30 处 “Return here”（27 纯 + 3 变体）、“Theorem ??”、第 13 章只有标题、第 6 页 “The introduction itself is unfinished”。
7. **“人先写、AI 先挑错”的时间线**：Vakil 博文 02:18 BJT；Glazer “Fails an Astra audit” 02:59 BJT（41 分钟后）；Zulip Masullo 串里 LLM 读稿的发言从 03:32 BJT 起；Vakil 更新段写在约 8 小时后。AI 指出的问题细节未公开贴出（McCorvie README：“the details have not been posted”），已列出的勘误（F1–F18）主要在附录里重述标准材料的写法，文件自称与迹公式无关（L6）。
8. **“25 万页”**：只是 Alpöge 脚注里自述 “amused myself, and hit 250,000+ pages” 的未公开稿，没有任何人见过；中文标题常写成既成事实（A11）。
9. **Alpöge 自己指出的时间疑点**：“There is an obvious cynical view, to do with the timing, but it is false”（第 2 页）；Nature 引 Granville：“What he released is garbled”，并把时机与 IPO 后股价联系（Granville 的归因，非 Anthropic 回应）。
10. **Lean 里“看起来对”的方式**：Sutherland 页自述未沙盒构建，“We have not ourselves compared its sha256 with an export made from a sandboxed lake build.”；Lipponen 在沙盒内独立构建，`Main.olean` 哈希一致。Lipponen 标明 8 个 `Prop := True` 占位，称不影响终定理的可靠性（AI 阅读者的判断）。
11. **Zulip 对 AI 文本的管控**：版主明文 “AI-generated comments are disallowed here”，要求引用 AI 输出须标明（A4）。这是社区对“AI 读 AI”的态度，不是证明的状态。

## 四、夸大说法实例

见 A11 表（中文 4 组、英文 TechCrunch/Quanta 各 1 处细节）。核心：TechCrunch 把 “American Mathematical Society” 写成 “American Mathematics Association”；新浪新闻标题把“25 万页”当成 AI 写出的证明；新浪科技标题“攻克”；爱范儿/网易标题“Tao 回应”。**没有找到**把 Aaronson 三句写成 Tao 原话的标题。

## 五、扫描说法勘误

| 扫描说法 | 本组核对 |
|---|---|
| remark.pdf “5 页” | 4 页（pdfinfo）；SHA-256 3fc97784…eb8d，与 1119 存档一致 |
| “没有公开的公理依赖清单、没有第三方 no sorry 结论” | 已过时：Sutherland 页 audit.log 列出 5 个定理的公理依赖；Lipponen 静态扫描无 sorry/axiom/admit/native_decide（A2、A3） |
| 把 “Mathematicians of course won't be satisfied with 17 million lines of Lean”、“kernel bugs … 0=1”、“my bet is that the proof will stand” 记为 Tao | 实为 Aaronson（`scottaaronson.blog/?p=10344`），Tao 博文与评论中没有（A5、A6） |
| Tao 原话 | 原话是 “using an internal AI model” 与 “Generalized Riemann Hypothesis”；未写 Claude，也未写 Dirichlet（A5） |
| “Anthropic 全程未表态” | 官方新闻页（18:29）与 X 镜像（18:30）里未见；媒体写 “did not respond”（AP、Nature）。无发言人引语。**不能**写成“Anthropic 拒绝评论”（A10） |
| “评论 #35：27 个 Return here、第 13 章两页不陈述定理、勘误 11 条” | 27 处与页码范围成立，实有 30 处含变体；第 13 章两页成立，但重述了迹公式；勘误数已涨到 F1–F18（A6） |
| ChatGPT 调查：Nielsen 反驳稿 | 三个地址均 Cloudflare 403，读不到正文（A9） |
| ChatGPT 调查：Islam | Zenodo 页存在（11-20 发布，自发布）（A9） |
| ChatGPT 调查：Glazer 说证明没过 Astra 审计，指 Masullo | 帖子存在，需靠回复链才知所指（A8） |
| ChatGPT 调查：Reddit 摘录 480 页 Theorem 11.1.2.1(2) 有反例 | Reddit 读不到；HN 与 Refine 自发布报告有相同指向（A8） |
| ChatGPT 调查：“Lean 4.33.1 缺少后续安全修复” | 本组在 Zulip 里只见 Floris van Doorn 的回答（该工具链已含 Leo 博客所列内核漏洞的修复，“I don't recall a kernel bug after that (but there might be one)”）；没有找到该说法的原帖，**未核实** |
| 派工单里 Claude 北京 18:00 前后的 HN 数字 | 本组 18:30 复取：50574968 2647/1810，50579568 2082（扫描 2071）/926，50578358 447（扫描 445）/180，50581084 296（扫描 292）/246（A12） |
| 扫描：Masullo 稿“27 个 Return here”（评论 #35） | 本组独立计数 27 纯 + 3 变体 = 30（A6） |
| 扫描：Masullo 现职 | 稿件致谢写“曾是 Stanford 研究生”；TechCrunch 写 postdoc、Quanta 引 LinkedIn 称 postdoctoral scholar，稿件本身未写现职（A7） |

## 六、对读者

- 现在读者**能验证什么、不能验证什么**：Lean 源码文件与导出（7.7 GB 压缩，61 GB 解压）公开，Sutherland 页给出 `lake comparator` 与 `con-leche` 命令，要求约 100 GB 内存、70 GB 磁盘（Sutherland 页 “Check it yourself”）；con-leche 从源码构建约两分钟，约 50 GB 内存、64 线程不到四分钟。普通读者做不到。
- **不受影响**：日常使用、密码学与互联网安全——SciAm：“It doesn’t have any immediate real-world applications—it won’t upend global banking or make the internet run faster.”
- **奖金**：本组未取克雷研究所规则原页（`claymath.org`），不写发表与审查期要求；中文稿（凤凰网、什么值得买）称需期刊发表并经至少两年审查，属 L5，**未核实**。
- **Masullo 稿**：任何人可下载稿件；稿件第 6 页自述未完成，Vakil 写 “all issues need to be resolved to the satisfaction of the community before this proof would be accepted as true”。
- **Alpöge 的 Lean 在做什么、没做什么**：README “It would not establish that anyone understands the argument”；Sutherland 页：“A passing check shows that the formal statements, as written in Lean against Mathlib's definitions, follow from the three standard axioms according to the kernels used.”
- 无“开关或设置”“价格”“开放范围”一类面向普通用户的操作项。

## 七、未完成或受限

- Reddit 全部读不到（登录墙/403），未取 r/math 帖。
- Nielsen（philarchive/philpapers）Cloudflare 质询，未取正文。
- A8 的 X 帖未截图（01–12 号码用完；Glazer、AcerFur 帖有 fxtwitter JSON，如需卡片再补）。
- Zulip 465 条只做了关键词扫描，未通读。
- Quanta 实时更新页各条时刻未逐条取。
- 本组未取：Sutherland 页上 `lean4lean-raised-fuel.log` 等其余日志、克雷研究所规则页、Axiom 的 `DedekindZetaZeroFree`（Zulip 里 Junyan Xu 提到，非本主帖必需）。
