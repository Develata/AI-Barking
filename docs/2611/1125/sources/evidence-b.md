# 1125 B 组取证：黎曼证明“砍掉 99.5%”——短版 grh-lean 与原版是什么关系，各家数字量的是哪一版（跟进 1122）

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，UTC+8；北京 2026-11-26 21:38 起取证，22:00 前写成。时间一律北京时间，括号保留原时区。级别按 `EDITORIAL.md` L1–L7；状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。数字多处不一致，**一律并列记录，不取舍，比值由 Claude 算**。抓取与截图逐条记录见 `capture-log-b.md`；截图在 `images/13–24`。

## 零、先读这一节（结论与对 Claude 假设的判定）

Claude 的初步假设拆成四句，逐句判定：

1. “99.5% 可能就是 grh-lean 紧凑版对原版的行数比”：**部分支持**。Sernau 本人在 Lean Zulip 写“from about 15 million lines to ~76,000”（B1-6）；量子位、HN 评论写 99.56%，分子是 76,149（cloc code 行），分母是 17,374,215（rh-shrink 与 rh-kernel-backbone 的**总行数**，含空行注释）。三处分子分母口径不同，见“一、数字对照表”。
2. “README 没说是从原版删出来还是另写，要看 ATTRIBUTION/NOTICE、提交历史与代码比对”：**README 其实写了，且不是“删”**。`grh/README.md` 与 `grh-compact/README.md` 原句：“This project is a shorter rewriting of Levent Alpöge's Lean proof … It follows that proof's method; many steps are proved by other arguments, and parts of it, mostly background material, were ported from there and then rewritten.”（B1-2、截图 14）；`NOTICE`：“src/Statements.lean and the names and types of the five theorems in src/Main.lean are taken from that proof unchanged; lakefile.toml is adapted from it.”（B1-4）；`ATTRIBUTION.md` **只**列 Mathlib 的改编，没有一行指向原版代码（B1-4）。提交历史只有 1 个提交（B1-5），无从看“删”与“写”的过程。**未做**与原版的代码比对（按派工单不下载原版；只比了 `Statements.lean`，见 B2-3）。
3. “若是另写，则吠点是：短版是另一份证明”：**与说法不符 / 说得过头**。Sernau 在 Zulip 原话：“Yes, it's based on the same proof. … Some stuff just needed entirely new arguments though, like getting rid of Vandiver.”（B1-7）；“a fair number of the arguments are new. For example: resolution of singularities is gone entirely; the product formula for a Gram determinant replaces the slope inequality and Hecke L-values replace the covolumes on Shimura varieties. Vandiver, patching, Hida theory, Gsp4, local Langlands, there were some big cuts.”（B1-8）；“we code golfed the heck out of it to get the 76k version and then rewrote for clarity to get the 300k one”（B1-9）。即：**同一方法、Claude 重写，部分步骤换了新论证**；既不是“删冗余”，也不是“另一条证明路线”。
4. “社区四套统计量的是原版，结论是删不了多少”：**部分与说法不符**。(a) rh-shrink 追踪的是 Lean 社区仓库 `main`，11-25 北京 11-26 06:24 起 `main` 被换成 grh-lean 的可读版，所以它的“2.0% left”是**换版**造成的，不是删除（B3-1）；(b) rh-kernel-backbone、rh-trim、rh-proof-trimmed 量的确是原版（Lean v4.33.1，约 1737 万行），删除比例各异：8%（rh-trim，从未零错构建）、约 14.1%（rh-proof-trimmed S4，**构建与内核复核通过**）、社区 `alpoge` 分支截至换版前 24.8%（rh-shrink 文本计数，有效代码行）；(c) VerifyTask 第一季量的是原版 39 个核心模块的标准化 token，**现行榜单 33.52%**，第二季已改到 grh-compact（B3-4）；(d) “删不了多少”只对 rh-trim 的“仅死代码”口径成立。

另有 5 条“扫描 / 中文稿口径”与一手来源对不上，见“十一、扫描说法勘误”。

## 一、数字对照表（各数字的分子、分母、版本、口径、出处；比值不算）

| 数字 | 对象 | 版本 / 基线 | 口径 | 出处（文件） |
|---|---|---|---|---|
| 17 million lines | Alpöge 原版 | Lean v4.33.1 | Lean FRO 与 Mathlib 联合声明原句 “a Lean library comprising 17 million lines of code” | `docs/2611/1122/sources/evidence-A.md` A1-2；本期另取 `b-lean-blog.txt`（B2-6 已 grep 无 “shorter / grh-lean”） |
| 175,187 文件 / 17,374,215 总行 / 14,621,937 代码行* | 原版 | rh-shrink 对 `40e56bab`“Mirror the submission exactly as published”的文本统计（*去空行、纯注释行、文件头 import） | 正则文本统计，不跑 Lean | `b-rh-shrink-earlier.json` 第 1 条 |
| 175,187 文件 / 17,374,215 raw lines / 1,063,284 声明 | 原版 | rh-kernel-backbone `v0.2.1` 内核导出 | README 表 “Upstream source tree” | `b-gh-thekeunpie-hash_rh-kernel-backbone-README.md`；截图 19 |
| 175,188 文件 / 17,374,242 行 | 原版 | rh-trim | README “Original 17,374,242 / 175,188” | `b-gh-keithadler_rh-trim-README.md` |
| 980.4M 字符 / 175,187 文件 | 原版 | VerifyTask | “Alpöge’s 980.4M-char RH proof” | `b-verifytask.txt` |
| about 15 million lines → ~76,000 | 原版 → 紧凑版 | Sernau 自述（Zulip，北京 11-26 05:26，消息有 EDITED 标记） | “reduce it by about two orders of magnitude” | `b-zulip-excerpts.md`；截图 24 |
| 76,149 code / 17,797 blank / 18,881 comment；95,030 非空行；112,827 总行；794 文件 | grh-compact/src | `72fead10`，cloc 与 `wc -l` 实测 | cloc 的 code 不含空行与注释行，含 import | `b-grh-lean-measure.txt` |
| 299,902 code / 432,527 非空行 / 477,124 总行；803 文件 | grh/src（可读版） | `72fead10`，同上 | 同上 | 同上 |
| 约 30 万 / 约 7.6 万行；43.3 万 / 9.5 万非空含注释；47.7 万 / 11.3 万总行；495+306 / 573+219 文件 | 两版 README 自述 | README Layout | 与实测一致 | `b-grh-readable-README.md`、`b-grh-compact-README.md`；截图 15 |
| 298,160 code* / 477,124 总行 / 803 文件 | 社区 `main` 换成可读版后的首个数据点 | rh-shrink（code* 另去 import 行） | | `b-rh-shrink-data.json` 第 1 条 |
| 296,425–296,610 code*、801 文件（会随社区继续提交变化） | 社区 `main` 现状 | rh-shrink，北京 21:45 | “98.0% down, originally 14.62M”“Files 801 down 99.5%, originally 175.19K” | 截图 16 |

要点：**同一个“99.5%”在 rh-shrink 页上还有第二个来源——“Files 801，↓99.5%”（文件数）**；“98.0%”才是它的代码行降幅。

## 二、B1 grh-lean 本身

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B1-1 | 仓库元数据 | L2（个人 GitHub 账号 `lsernau`；LICENSE 署 Anthropic） | https://github.com/lsernau/grh-lean ；API `gh api repos/lsernau/grh-lean` | description “A short Lean 4 proof of the Generalized Riemann Hypothesis”；created_at 2026-11-25T20:42:55Z（北京 11-26 04:42:55）；pushed_at 20:45:04Z（04:45:04）；星标 744、fork 78（北京 21:38），页面 21:48 渲染为 747 / 78；Apache-2.0；Issues 1 | 13 | 仓库在个人账号下，**未在 anthropic.com 新闻与研究页找到对应条目**（北京 21:59 取 `b-anthropic-news.txt`、`b-anthropic-research.txt`，grep 无 riemann / lean / grh / sernau）；HN 有人问“为何不挂在 Anthropic 账号”（B7） | 已找到 |
| B1-2 | README 关系句 | L2 | `b-grh-lean-README.md` 根 README；`b-grh-readable-README.md`；`b-grh-compact-README.md` | 根：“The proof is based on Levent Alpöge's, with Claude (https://alpo.ge/remark.pdf). This repository is a shorter version, produced in collaboration with my colleagues at Anthropic, and Claude.”；子目录：“The proof is Levent Alpöge's, with Claude. The Lean text and the documents in this project were written by Claude (AI).” 与 “This project is a shorter rewriting of Levent Alpöge's Lean proof … It follows that proof's method; many steps are proved by other arguments, and parts of it, mostly background material, were ported from there and then rewritten.” | 13、14 | “rewriting”是 README 自己的词 | 已找到 |
| B1-3 | 两版规模与状态 | L2 | 同上 | 根：“`grh/`: readable edition, about 300,000 lines.” “`grh-compact/`: the same five theorems, about 76,000 lines.” “The proof has passed machine checks only, listed in each folder's README. No mathematician has read it through, and no paper comes with it.” “Research artifact. Not maintained and not accepting contributions.” “A build takes one to two hours on 12 cores with 24 GB of memory (about 85 minutes more if the Mathlib cache is missing).” | 13 | 紧凑版 README 另写：“The proofs are written to be short, not to be easy to read.” | 已找到 |
| B1-4 | NOTICE / ATTRIBUTION | L2 | `b-grh-readable-NOTICE.txt`、`b-grh-compact-NOTICE.txt`、`b-grh-*-ATTRIBUTION.md` | NOTICE：“This product is a rewriting of Levent Alpöge's Lean proof (see README.md). src/Statements.lean and the names and types of the five theorems in src/Main.lean are taken from that proof unchanged; lakefile.toml is adapted from it.” ATTRIBUTION 首句：“Each declaration below adapts or closely follows, in whole or in part and with changes, a definition, statement or proof from Mathlib (Apache License, Version 2.0).” | — | ATTRIBUTION 表只有 4 列：文件 / 声明 / Mathlib 声明 / Mathlib 文件，**无任何一行对应原版文件**。紧凑版 NOTICE 另写：“src/Lib/General/LinearAlgebra/Base.lean restates about twenty lemma statements on matrices from Mathlib … for classical decidability.” **没有“复用原版多少代码”的数字** | 已找到 |
| B1-5 | 首个与最新提交 | L2 | `git clone --depth 50` 实测（`b-grh-lean-measure.txt`） | 仅 1 个提交：`72fead102f663d5c7a407964a13dc0cf9d977207`，author-date 2026-11-25T12:38:07-08:00（= UTC 20:38:07，北京 11-26 04:38:07），作者 Luke Sernau，信息 “Lean proof of GRH” | — | 无历史可对比；首提交早于建仓 4 分 48 秒 | 已找到 |
| B1-6 | Sernau 在 Lean Zulip 宣布 | L2（公开发言，署名可链接） | https://leanprover.zulipchat.com/#narrow/channel/658556-Riemann-Hypothesis/topic/New.20proof.20in.20300k.20lines/near/643702366 （`b-zulip-excerpts.md`） | “Hi all, over the past few days I've been sprinting with a few colleagues at Anthropic on a much shorter version of the proof. We've been able to reduce it by about two orders of magnitude, from about 15 million lines to ~76,000. Also removed a lot of complexity (removes the dependence on the weak form of Vandiver's conjecture, uses a simpler integrality argument, the sieve step is just Poisson summation, etc.). Just posted it here: https://github.com/lsernau/grh-lean” | 24 | 北京 11-26 05:26:26；消息标 EDITED（取到的是编辑后版本）；“about 15 million”，不是 17.37 million | 已找到 |
| B1-7 | 方法答复（问：能分享方法吗） | L2 | 同话题，消息 643710501 | “Yes, it's based on the same proof. I'm not able to say very much about methods, unfortunately, but I had some luck with unrolling definitions to get a long, flat proof, looking for simplifications there, and then rolling them back up again. That way I could simplify across boundaries. Some stuff just needed entirely new arguments though, like getting rid of Vandiver.” | — | 北京 06:13:06 | 已找到 |
| B1-8 | 新论证与大块删除 | L2 | 消息 643717520 | “a lot of the simplification was finding simpler proofs for sub-results. So a fair number of the arguments are new. For example: resolution of singularities is gone entirely; the product formula for a Gram determinant replaces the slope inequality and Hecke L-values replace the covolumes on Shimura varieties. Vandiver, patching, Hida theory, Gsp4, local Langlands, there were some big cuts.” | — | 北京 06:57:21 | 已找到 |
| B1-9 | 两版关系：紧凑版是“golf”，可读版是再重写 | L2 | 消息 643719896 | “Basically we code golfed the heck out of it to get the 76k version and then rewrote for clarity to get the 300k one. So the 300k version is sort of what you're describing plus some additional readability edits.” | — | 北京 07:12:57；与 HN 评论 gregdeon “the 76k line version is heavily golfed … a more sane version is around 300k lines”（B7）一致；两版 README 均称“some of its steps use other arguments”（可读版与紧凑版论证不完全相同） | 已找到 |
| B1-10 | 多出来的内容 | L2 | 消息 643724260；`grh/README.md` Checks | “we made some cursory effort to also include some points of interest, and finish the thought on some lemmas and things rather than using just the fragment the proof needed.” README：“Several hundred of the more than 20,000 declarations are not used by the five theorems: they state results for their own sake, or serve automation.” | 15 | 即短版并非“只含必要”；可读版 README 自认有未用声明 | 已找到 |
| B1-11 | 顶层定理 | L2 | `b-grh-readable-README.md` Theorems；`grh/src/Main.lean` | 五个：`riemannHypothesis`（Mathlib 的 `RiemannHypothesis`）、`riemannZeta_eq_zero_re`、`LFunction_ne_zero_of_half_lt_re`、`LFunction_eq_zero_re`、`LFunction_eq_zero_re_of_isPrimitive`。README：“The last three theorems say nothing about zeros with re s ≤ 0.” “All moduli N ≥ 1 and all characters are covered, primitive or not.” 两版 `src/Statements.lean` `diff` 无差异 | — | 陈述文件见 `b-grh-Statements.lean.txt` | 已找到 |
| B1-12 | `PrintAxioms` / 机器检查清单 | L2（作者自述）+ L3（MIT 独立复核，见 B2-5） | README Build / Checks；`scripts/PrintAxioms.lean`（`#print axioms` 五个定理） | 作者自述：“The first of its five results is `'riemannHypothesis' depends on axioms: [propext, Classical.choice, Quot.sound]`.”“An export of the five theorems … is accepted by the independent checkers nanoda, con-leche and con-ron.”（可读版）；“Every file in `src` compiles with the pinned Lean and Mathlib, without errors or `sorry`, and was replayed through the kernel of Lean.”（紧凑版）；“We ran Comparator v4.35.0 on this folder (Linux, with a stand-in for its sandbox and `ulimit -s unlimited`), and it accepted the proofs; allow 64 GB of memory and a few hours.”（两版） | 15 | **本次未在本机构建**（Lean 与 Mathlib 缓存超出时限），`#print axioms` 输出未自跑；只有作者自述与 MIT 页（B2-5） | 部分支持 |
| B1-13 | Luke Sernau 公开身份 | L2 | 他本人 Zulip 发言（B1-6）、README 与 LICENSE 署 “Copyright 2026 Anthropic, PBC”；Wikipedia（L5，仅作线索）称 “Anthropic researcher Luke Sernau” | 他自述 “colleagues at Anthropic”；README：“produced in collaboration with my colleagues at Anthropic, and Claude” | — | **他自己的话没有给职衔**；第三方页（Google Scholar / OpenReview / LinkedIn 的搜索摘要）仍写 Google DeepMind，未深究是否过期；不收更多个人信息 | 部分支持 |
| B1-14 | 实测规模 | 实测（本机） | `git clone --depth 50` + `find` / `wc -l` / `cloc`，见 `b-grh-lean-measure.txt` | 可读版 803 个 `.lean`（src）；总 477,124；非空 432,527；cloc code 299,902（Lib 177,492 + New 122,391 + 根 19）。紧凑版 794 个；总 112,827；非空 95,030；cloc code 76,149（Lib 51,280 + New 24,850 + 根 19） | — | README 与实测一致；含 `Challenge.lean`、`scripts` 则各多 2 个文件（805 / 796）；README 写“801 / 792 files”之说来自量子位等二手稿，README 本身写 495+306、573+219 | 已找到 |

## 三、B2 与原版的关系

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B2-1 | 原版规模（以 1120 / 1122 存档为准） | L3 | `docs/2611/1122/sources/evidence-A.md` A1-2（Lean 博客声明）；`docs/2611/1120/sources/evidence-a.md`（MIT 复核页：175,187 模块、Lean v4.33.1、Mathlib v4.33.1）；本期补：rh-kernel-backbone、rh-shrink、rh-trim 数字（见“一”） | 声明：“a Lean library comprising 17 million lines of code” | 19、16 | 17,374,215 总行是第三方（rh-shrink、rh-kernel-backbone）的计数；“1737 万 / 1737.4 万”**不是官方数字**，官方写 17 million | 已找到 |
| B2-2 | grh-lean 是否复用原版代码 | L2 | B1-2、B1-4、B1-7 | “parts of it, mostly background material, were ported from there and then rewritten.”；“src/Statements.lean and the names and types of the five theorems in src/Main.lean are taken from that proof unchanged” | 14 | **复用比例无官方数字**。ATTRIBUTION 不含原版条目。未做逐文件代码比对（不下载原版） | 部分支持 |
| B2-3 | 陈述文件与原版一致 | 实测 + L3 | `b-statements-original-mirror.lean.txt`（取自 `homeowmorphism/riemann-hypothesis-original`，该库描述 “preserved exactly as published on 2026-11-20”）与 `b-grh-Statements.lean.txt` | 非空行 `diff` 无差异；另 MIT `shortgrh` 页：“Statements.lean is byte-for-byte the same file in the two editions, and its four definitions are textually identical to those in the original proof's Statements.lean.” | — | 社区 `alpoge` 分支上的 `Statements.lean` 只在头三行（`module` / `public import` / `@[expose]`）与 `import Mathlib` 不同，定义体相同（`b-statements-alpoge.txt`） | 已找到 |
| B2-4 | “99.5%”能否由 7.6 万对原版行数直接得出 | — | 见“一、数字对照表” | 两个原始数字：紧凑版 76,149（cloc code）/ 112,827（总行）；原版 17,374,215（总行）/ 14,621,937（rh-shrink code 行*）/ Sernau 自述“about 15 million” | — | 分子分母口径不同，**由 Claude 选口径后算**；量子位与 HN 评论的“99.56%”所用分母是 17.37M | 已找到 |
| B2-5 | 独立方对 grh-lean 的复核构建 | L3 | https://math.mit.edu/~drew/shortgrh/ （`b-sutherland-shortgrh.txt`） | “Andrew Sutherland (MIT), checks run with Claude Code. Page last updated 2026-11-26 02:50 UTC. All checks have finished.” “Both editions compile without errors, and in both the five theorems depend only on the axioms propext, Classical.choice and Quot.sound. Every module of both editions has been replayed through the Lean kernel.” “lake comparator --paranoid (Lean v4.35.0) reports “Your solution is okay!” for both editions. … For the compact edition it passed independently on two machines. For the readable edition it passed on the larger machine; lean4lean needs 332 GB of memory for that edition, more than the smaller machine has, so the run there was stopped without a verdict” | — | 页面最后更新 北京 11-26 10:50；与原版页（`https://math.mit.edu/~drew/grh/`，“Page last updated 2026-11-25 22:05 UTC”= 北京 11-26 06:05）并列，原版页新增一句 “A shorter Lean proof was published on 25 November 2026; the checks of that proof are recorded on a separate page”。**这是对“代码通过机器检查”的复核，不是对数学内容的审阅**。作者本人说明与 HN 评论 tito（riemann-at-home，18 个独立验证者）另有众包构建（L6，未核） | 已找到 |
| B2-6 | Lean FRO / Mathlib 声明是否已覆盖短版 | L3 | `b-lean-blog.txt`、`b-lean-fro.txt`、`b-mathlib-init.txt`（北京 21:40 取） | grep “shorter / grh-lean / short / sernau”均无命中（`b-alpoge-home.txt` 唯一命中是无关论文名） | — | 声明原文（1122 存档）只写 Alpöge 的 Riemann Hypothesis 与 17 million lines，**未提短版**；“hereby certify” 不延伸到 grh-lean 是**从沉默推出**，不是原文写明（推断） | 部分支持 |
| B2-7 | Lean 社区仓库的态度 | L3（Lean 社区官方仓库，维护者署名） | https://github.com/leanprover-community/riemann-hypothesis ；`b-lc-rh-readme-main.txt`；`b-gh-lc-rh-repo.json`；Zulip 消息 643709436 | 仓库 README：“This repo recently moved to a highly condensed version on November 25, 2026. Prior progress is preserved on the `alpoge` branch.” “This work is taking place on the `main` branch, which is based on a copy of the readable edition (`grh/`) of https://github.com/lsernau/grh-lean at commit 72fead102f663d5c7a407964a13dc0cf9d977207.” 换版提交 `47fcde57`（Kim Morrison，2026-11-25T22:24:29Z = 北京 11-26 06:24:29）：“Make grh/ of lsernau/grh-lean commit 72fead1 the root of this repository and drop grh-compact/, which stays on branch grh-lean. The file contents are unchanged.” Zulip（北京 06:06:51）：“I propose that we just move main over to a different branch, say alpoge, and just switch to this shorter proof on main.”；（07:08:23）“I've basically turned off the merge queue for the alpoge branch, to reuse those machines for v4.37.0-rc1. To be honest I'm not sure what the point of continuing with the old branch is.” | — | 社区 `main` 此后继续对可读版做 PR 清理（`batch/m1…m39-on-main`，北京 21:22 最后一条合并）。README 许可段：“The proof belongs to its author Levent Alpöge with Claude and was originally published without a license.” | 已找到 |

## 四、B3 四套（加一套）“删冗余”统计

每套分“统计对象 / Lean 版本 / 方法 / 核心数字原句 / 作者”。

### 4.1 rh-shrink（社区统计站）

| 项 | 内容 |
|---|---|
| URL | https://rh-shrink.github.io/ ；仓库 https://github.com/rh-shrink/rh-shrink.github.io（组织账号，2026-11-22T16:52:59Z 建 = 北京 11-23 00:52；描述 “Size of the Lean proof of the Riemann Hypothesis per commit (unofficial)”） |
| 统计对象 | **`leanprover-community/riemann-hypothesis` 的 `main` 逐提交**。原版是第一个数据点（`40e56bab`，北京 11-20 22:36）；11-25 换版后，同一曲线接上 grh-lean 可读版（页面标注 “Nov 25: main replaced by the readable edition of grh-lean; the earlier work goes on in the branch alpoge”）。**换版前后是两个不同的证明版本，在同一条曲线上** |
| 方法 | 页面 “How it's measured”：“Text search, not Lean. All numbers come from plain text search, with Python regular expressions, over the .lean files as stored in git (collect.py). Lean is never run and nothing is elaborated.” “Code lines*: total lines, without blank lines, lines that are only comments … and the imports at the top of each file”（截图 23） |
| 作者公开身份 | 页脚：“Vibe coded on commission from Moritz Roos.”（链到 GitHub 用户）；仓库属组织账号。不收更多信息 |
| 数字（`b-rh-shrink-data.json` + `b-rh-shrink-earlier.json`，北京 21:39 取；按提交顺序，日期为提交者本地时区，会交叉） | 原版 `40e56bab`：175,187 文件 / 17,374,215 总行 / 14,621,937 code*。**换版前**：①`9c9f665e`（PR #185，提交时间 +11:00 时区 11-23 18:20 = 北京 11-23 15:20）起 12,214,914 code*（−2,407,023，16.46%）；②`c82871e6`（PR #539，+11:00 时区 11-24 10:24 = 北京 11-24 07:24）12,108,510（−2,513,427，17.19%）；③最后一个换版前点 `756a7025`（PR #4761，+11:00 时区 11-26 08:25 = 北京 11-26 05:25）：58,947 文件 / 13,710,054 总行 / **11,000,328 code***（−3,621,609，**24.77%**；文件数降到 58,947 主要是 “clump” 并文件，不是 code 行降幅）。**换版后**首点 `47fcde57`：803 文件 / 477,124 总行 / **298,160 code***（占原版 2.04%）；页面现状（截图 16，北京 21:45）：296.61K code*，“98.0% down, originally 14.62M”；Files 801 “99.5% down, originally 175.19K”；Total lines 474.61K “97.3% down, originally 17.37M”；Theorems 17.44K “97.9% down, originally 818.76K”；Import statements 1.85K “99.8% down, originally 753.57K” |
| 扫描 / 中文稿数字核对 | “1462 万→1211 万、减 16.5%”：1462 万 = 原版 code* 14,621,937；**1211 万 = `c82871e6` 12,108,510；16.5% 却是 `9c9f665e` 12,214,914 的降幅**。两个数是两个提交，量子位稿把它们写在同一句（B5-3）。“240 万行”= 2,407,023（`9c9f665e`）；“251 万”= 2,513,427（`c82871e6`）。均为文本统计的 code 行，**不是 Lean 内核口径，也不是“删掉了无用证明”的证据** |
| 状态 | 已找到（多数字原句核到出处）；“两天”= 11-22 至 11-24 的粗略说法，页面无此句 |

### 4.2 rh-kernel-backbone

| 项 | 内容 |
|---|---|
| URL | https://github.com/thekeunpie-hash/rh-kernel-backbone （2026-11-22T02:07:04Z 建 = 北京 10:07；0 星标；站点 https://thekeunpie-hash.github.io/rh-kernel-backbone/ ，README 另称 riemannhypothesis.dev） |
| 统计对象 | **原版**（README：“baseline `v0.2.1` on Lean `v4.33.1`; since 2026-11-25 upstream `main` is based on the shorter `lsernau/grh-lean@72fead10` proof, with the original tree kept on the `alpoge` branch”）；数据来自 Andrew Sutherland 与 James Shore 的 `v0.2.1` Lean 内核导出 |
| 方法 | **用 Lean 内核导出**（`in_cone` = 在五个定理的内核依赖闭包内），README 标 “[Exact]”（机械计算）、“[Verified]”、“[Reading]”（作者的数学解读）三档；不是纯文本统计 |
| 核心数字原句（README / SPEC，截图 19） | “175,187 files · 17,374,215 raw lines”；“Active Lean kernel cone (`in_cone = true`): 146,287 files · 10,737,728 decl lines; 804,912 active decls (75.7% of decls, 61.8% of raw lines)”；“Wholly dead files (`used_decls = 0`): 28,900 files · 1,754,237 decl lines — Plus 940,515 dead decl lines inside mixed active files”；SPEC 第 30 行：“Total unused decls: `258,372` (`24.3%` of all decls; `2,694,752` decl lines)”；“99.997% of the cone, routed through one root chokepoint: `allChi` (`src/Supplements/Kernels/MainTheorems.lean:7`)” |
| 扫描数字核对 | 1737.4 万 = 17,374,215 总行 ✓；**1073.7 万 = 活跃内核闭包的 decl 行（不是“被主证明用到的总行”，是声明行）**；**269.4 万 = 1,754,237 + 940,515 = 2,694,752，是“未用声明的 decl 行”**，与“1737.4 万”不同口径（decl 行总数是 13,432,480，不是 17,374,215）；扫描写“约 24.1 万条重复定理”：**README、SPEC、Pages 全文均无此数**（见 B3-6） |
| 作者 | GitHub 用户 `thekeunpie-hash`，README 与站点都没有职业身份；README 注明 `claude` 为贡献者之一（贡献者栏：thekeunpie-hash、github-actions[bot]、claude） |
| 同库还有 | VerifyTask 抓取脚本与 `data/verifytask_board.json`（北京 11-26 21:36 快照，793,241 字节，已存 `b-backbone-verifytask_board.json`；本组另存的 API 直取件 `b-verifytask-board.json` 为 1,148,179 字节，**超 1 MB，见回报**） |

### 4.3 VerifyTask（`riemann.verifytask.cn`，Bohrium Playground 看板）

| 项 | 内容 |
|---|---|
| URL | https://riemann.verifytask.cn （curl 200，`b-verifytask.html/.txt`；**浏览器与 headless Chrome 均打不开页面样式与数据，见 capture-log**）；API `https://riemann.verifytask.cn/api/board`（`b-verifytask-board.json`，北京 21:41 取；1,148,179 字节，超 1 MB） |
| 统计对象 | 第一季：把**原版**拆成 39 个“无假设核心模块”（Core 1–37、40–41，占 158,055 / 175,187 去重文件，页面写 92.23% 字符），参与者提交更短的**等价 Lean 代码**，评分用“anti-golf 标准化 token”，在线用 Lean 4.34.1 与 4.33.1 + Comparator（0 sorry、标准公理）校验。页面现状：“Anthropic released a compressed Lean 4 RH formalization (lsernau/grh-lean, 7.04 MB = 7.1% Mathlib). All Playground Season 1 community attempts and leaderboard records below are frozen as a historical archive.”；“SEASON 2 LIVE · grh-compact”（793 可编辑文件、7.04 MB、1.62M token；基线 0.0% 缩减） |
| 方法 | 页面 Methodology：“Model × Harness charts aggregate self-declared CLI tags involving human-in-the-loop guidance and unequal retry budgets (descriptive telemetry, not a blinded benchmark).” 编译校验是真的跑，评分是 token 计数 |
| 核心数字（API，北京 21:41） | `progress.core`：baseline_tokens 611,228,972 → current_best_tokens 406,329,566，**saved_tokens_pct 33.52**；saved_chars_pct 32.53；42 / 48 个核心已解（core_solved）。**单模块**：core36 “Pure Family Decomposition of CM Period S…” 1,023,975 → 188 token（99.98%）、core40 1,031,987 → 207（99.98%）、core44 99.73%、core33 99.72%、core7 99.45%、core29 99.17%（`challenges[].token_saved_pct`） |
| 扫描数字核对 | “整体压缩率 8.4%、5048 万、955 次通过、301 次有效突破”：**不是现行数**，是北京 **11-24 凌晨–上午**的快照。依据：rh-kernel-backbone 存的看板时间序列里，第 955 个点（提交运行数）在 11-23T20:54Z（北京 11-24 04:54）时 best_pct 8.16；8.33–8.34 出现于 11-23T21:22Z–22:46Z（北京 11-24 05:22–06:46）；到第 1000 个点（11-23T23:53Z = 北京 11-24 07:53）已 17.73；11-25T17:33Z（北京 11-26 01:33）32.45；现状 33.52。截至第 955 点 kept 点数为 304（量子位写“301 次有效突破”，近但不等）。量子位原文写“截至今天早上 8 点半”（11-24 发布），时间序列对不上 8:30（那时已超 17%）——**时刻与 8.4% 有出入**，原因未明（推断：文中“早上”指较早的快照） |
| HN 佐证 | HN 评论 w-m（北京 11-26 06:52）：“the community-driven compression with a hundred contributors and thousands of pull requests sits at a mere 33% reduction. … https://riemann.verifytask.cn/”，与 33.52% 一致（L6） |
| 作者公开身份 | 页面链到 @osgood001、GitHub `Osgood001/riemann-hypothesis`；不收更多 |
| 状态 | 部分支持（33.52% 与 99.9% 单模块已核；8.4% 当时数无法原页复核，仅有量子位转述与时间序列近似） |

### 4.4 rh-trim（keithadler）

| 项 | 内容 |
|---|---|
| URL | https://github.com/keithadler/rh-trim （2026-11-20T21:58:17Z 建 = 北京 11-21 05:58；12 星标，北京 11-26 取） |
| 统计对象 | **原版**（“175,188 files and 17.4 million lines”），README 称“project wrapped up 2026-11-23 21:00 UTC” |
| 方法 | 在云机（AWS Spot）上**真构建原版**；用 Lean 导出（61.6 GB）做独立内核复核（Tenet C#、nanoda、tenet-jvm 接受 1,468,414 个声明；Lean 自带内核、paranoid、tenet-hs “stopped unfinished”）；删“死代码”的循环：删一批、编译、读错误、放回、再删。**不是纯文本统计** |
| 核心数字原句（截图 18） | “The smallest proof: stopped, unfinished”；“SMALLER ~8% in lines, not confirmed”；“LINES 15.95M was 17.37M”；“BEST ROUND 17 errors round 66 · it never reached 0”；“TIME 38h 27m 71 rounds”；“The cut source never built with 0 errors, so the ~8% is an upper bound on what dead-code removal gives, not a result.”；“Mostly foundations. The final argument needs only **6.6%** of the proof. … One step alone owns **38%**, and is likely where the new idea lives.”；“99.35% of the proof sits under a single statement, `allChi`.”；“It repeats itself: about 96,000 theorems restate another theorem word for word.”；表：Original 17,374,242 / 175,188；Cut version（never built clean）15,954,961 / 172,500 |
| 扫描数字核对 | 6.6% ✓；约 8% ✓（上界，未确认）；“没有精简版零错误构建通过” ✓（**只对 rh-trim 自己的删减版成立**，rh-proof-trimmed 与社区 PR 的删减版是构建通过的）；**“allChi 一步占 38%”与说法不符**：README 写“99.35% … under a single statement, `allChi`”；“One step alone owns 38%”的那一步 README **没有点名**（只说“likely where the new idea lives”）；38% 与 `allChi` 是两个不同的数 |
| 作者 | GitHub 用户 keithadler；README 注 “Tenet (C#) (mine)”，无职业身份；贡献者栏含 `claude` |
| 状态 | 已找到 |

### 4.5 rh-proof-trimmed（james-shore，扫描没提）

| 项 | 内容 |
|---|---|
| URL | https://github.com/james-shore/rh-proof-trimmed （2026-11-21T20:43:58Z 建 = 北京 11-22 04:43；3 星标） |
| 统计对象 | **原版**（Lean v4.33.1，Mathlib `0df444a3…`）在 `leanprover-community/riemann-hypothesis` `main` 的某提交上的机械删减；“The five theorems in `src/Main.lean` and their statements are unchanged. No proof is written by hand or by a model.” |
| 方法 | 每阶段一个提交 + 收据；用 Lean 构建、nanoda 内核、con-leche `--compare`；“Term diff compares each statement … with the original archive's”（截图 17） |
| 表（README，北京 21:51 渲染） | Base `7da4577e`：164,463 文件 / 16,588,671 行 / 1,024,669 声明；S1 删未用声明：150,632 / 14,473,033 / 858,404（编译 yes、内核 yes、陈述同 yes）；S2 删 linter 标未用语法：150,632 / 14,453,805；S3 去重（只在 Lean 4.37，表中空白）；S4 删多余 import 行：150,632 / **14,252,250** / 858,404（均 yes）。“Files and lines exclude `Challenge.lean`” |
| 作者 | GitHub 用户 james-shore（rh-kernel-backbone 称其与 Sutherland 提供内核导出，L6）；不收更多 |
| 状态 | 已找到 |

### 4.6 扫描逐数核对汇总（B3 要求“逐一找原句，找不到写未找到”）

| 扫描数字 | 找到的出处与口径 | 状态 |
|---|---|---|
| 240 万 | rh-shrink 数据，原版 code* 14,621,937 − `9c9f665e` 12,214,914 = 2,407,023（`b-rh-shrink-earlier.json`）；中文稿“两天砍掉 240 万行”为量子位 11-24 稿的标题 | 已找到 |
| 16.5% | 同上 2,407,023 / 14,621,937；与“1211 万”不是同一点 | 已找到（与扫描配对说法不符） |
| 8.4% | VerifyTask 11-24 凌晨–上午快照（见 4.3）；现行 33.52% | 部分支持 |
| 99%（单模块） | VerifyTask core36 / core40 99.98%、core44 99.73%、core33 99.72%、core7 99.45%、core29 99.17% | 已找到 |
| 1073.7 万 | rh-kernel-backbone 活跃内核闭包 10,737,728 decl 行 | 已找到 |
| 269.4 万 | rh-kernel-backbone SPEC：未用声明 2,694,752 decl 行（= 1,754,237 + 940,515） | 已找到 |
| 约 24.1 万条重复定理 | rh-kernel-backbone README、SPEC、Pages 无；rh-trim 写“about 96,000 theorems restate another theorem word for word”；rh-shrink 原版 “Repeated theorem statements” 85,992（`b-rh-shrink-earlier.json` 第 1 条 repeats）。**24.1 万只在量子位 11-24 稿里出现，其归因（rh-kernel-backbone）本期未找到原页依据** | 未找到一手来源 |
| 6.6% | rh-trim README | 已找到 |
| 38% | rh-trim README“One step alone owns 38%”；**未写“allChi 占 38%”** | 与说法不符 |
| 8% | rh-trim README “~8%”（上界、未确认） | 已找到 |
| 5048 万 / 955 次 / 301 次 | 量子位 11-24 稿原句；VerifyTask 时间序列近似（4.3） | 部分支持 |
| 25 万页“是玩笑” | Alpöge `remark.pdf` 脚注 2：“I of course amused myself, and hit 250, 000+ pages.”（`b-alpoge-remark.txt`）；**原话没有“joke / 玩笑”**，是“amused myself”；量子位 11-24 稿写“25 万页是他开的一个玩笑”是转述（L4/L5）；HN gus_massa 评论（L6）称“The 'joke' was pretending that the Lean version printed in A4 p…” | 部分支持 |

## 五、B4 时间线（北京时间；括号保留原时区）

| 节点 | 北京时间 | 来源 |
|---|---|---|
| Alpöge 在 alpo.ge 放 `remark.pdf` 与 Lean 压缩包 | 11-20（1120 存档：PDF CreationDate Fri Nov 20 20:52:48 +08） | `docs/2611/1120/sources`；`docs/2611/1122/sources/capture-log-A.md` |
| 社区镜像库首个提交 “Mirror the submission exactly as published” | 11-20 22:36:09（UTC 14:36:09；提交者本地 -05:00 09:36:09） | `b-rh-shrink-earlier.json`；仓库建于 11-20T14:47:49Z = 22:47:49 |
| Lean FRO 与 Mathlib Initiative 联合声明 | 11-21 21:55（UTC 13:55） | 1122 存档 A1-1 |
| MIT Sutherland 原版页最后更新 | 11-26 06:05（UTC 11-25 22:05） | `b-sutherland.txt` |
| grh-lean 首提交 | 11-26 04:38:07（PST 11-25 12:38:07） | B1-5 |
| grh-lean 建仓 / 首推送 | 04:42:55 / 04:45:04（UTC 11-25 20:42:55 / 20:45:04） | B1-1 |
| Sernau Zulip 宣布 | 05:26:26（UTC 21:26:26） | B1-6 |
| HN 帖提交（nb-t） | 05:48:10（UTC 21:48:10） | `b-heat.json`、`b-hn-50648050.json` |
| Kim Morrison 提议 `main` 换版 | 06:06:51 | B2-7 |
| 社区仓库 `main` 换成可读版（`47fcde57`） | 06:24:29（UTC 22:24:29） | B2-7 |
| MIT `shortgrh` 页最后更新 | 10:50（UTC 02:50） | B2-5 |
| 腾讯新闻转量子位“暴砍 99.5%” | 08:40 | B5-1 |
| 腾讯新闻夕小瑶科技说 | 10:45 | B5-2 |
| 头条号“被压缩 99.5% 这一说法” | 14:52 | B5-4 |
| 腾讯新闻“楼宇风运录”“240 万”稿 | 18:45 | B5-3 |
| 量子位“两天砍掉 240 万行”原稿（36氪） | **11-24 19:08**（早于 grh-lean 两天） | B5-3 |
| 腾讯研究院转载 anyGeometry 澄清长文 | **11-25 17:11**（早于 grh-lean 上线 11 小时） | B6 |

“8 天”“五天删完”：**“8 天”未找到出处**（本期所取各中文稿、Zulip、README 均无此句）。“五天”：夕小瑶稿小标题“上百人删了五天”（B5-2）；量子位 11-26 稿“几天前”；YouTube 视频摘要写“五天前公布”（L6）；11-20 至 11-25 起算为 5 天。11-20 至 11-26 之间没有任何节点恰好对应 8 天。**状态：8 天——未找到一手来源。**

## 六、B5 夸大与纠错实例（中文稿）

| # | 说法 | 发布方 / 时间 | URL（存档） | 原句（逐字） | 截图 | 备注 | 状态 |
|---|---|---|---|---|---|---|---|
| B5-1 | “暴砍 99.5%” | 量子位（腾讯新闻转，2026-11-26 08:40 发布于北京） | https://news.qq.com/rain/a/20261126A051B600 （`b-cn-qq-995.html/.txt`）；同稿 IT之家 https://www.ithome.com/1/032/611.htm（`b-cn-ithome-995`，标题“黎曼猜想 AI 证明代码暴砍 99.5%：只剩 7.6 万行，又是 Anthropic”） | 标题“刚刚，黎曼猜想证明代码暴砍了99.5%！只剩7.6万行”；“代码量暴砍99.56%。”；“它对Alpöge的原版证明进行了彻底的重写与重构，并一口气放出了两个版本。”；引 Sernau 原话译文“从约1500万行减少到了约76,000行”；“目前这份证明仅通过了机器验证，尚未有数学家完整通读，也没有配套的数学论文。”；“可读版 … 分布在801个文件中”“紧凑版 … 分布在792个文件中”（实测 803 / 794，README 写 495+306 / 573+219） | 20 | 标题 99.5%、正文 99.56%（分母 17.4M）、引文 1500 万，三处并存；**写的是“重写与重构”，同时标题用“砍”**；如实写了“只过机器检查” | 已找到 |
| B5-2 | “上百人删了五天” / 重写 | 夕小瑶科技说（腾讯新闻，11-26 10:45） | https://news.qq.com/rain/a/20261126A06QF300 （`b-cn-qq-rewritten`） | 标题“黎曼猜想那份1740万行的证明，被砍掉了99.5%！”；“Luke Sernau 没去碰原来那17万个文件，而是带着 Claude 和几位 Anthropic 同事，把证明从头重写了一遍。整体思路沿用 Alpöge 的方法，但很多步骤换成了别的论证，从原版搬来的背景材料也都重写过。”；“可到11月25日，Lean 社区公共仓库里的代码，也才比原版少了大约24%。” | — | 标题夸大、正文基本如实（24% 与 rh-shrink 换版前 24.77% 一致） | 已找到 |
| B5-3 | “两天砍掉 240 万行” | 量子位 11-24 19:08（36氪）：https://www.36kr.com/p/4083605367761538 （`b-cn-36kr-240w`）；转载腾讯新闻“楼宇风运录”11-26 18:45：https://news.qq.com/rain/a/20261126A0AWHF00 （`b-cn-qq-240w`）；新浪 `cj.sina.com.cn`、中华网（`b-cn-china-240w`，11-25） | 标题“黎曼猜想证明两天砍掉240万行，全球接力压缩，马斯克在线围观”；36氪正文：“这份证明，包含了1737万行Lean 4代码，分成175187个文件。”“去掉空行、注释和引用声明，有效代码已经从最初的1462万行掉到了1211万行，少了16.5%。”“发现原来的1737.4万行里，真正被主证明用到的其实只有1073.7万行，刚过六成。还有269.4万行根本是主证明完全没用上的死代码，以及大约24.1万条定理是重复写的。”“截至今天早上8点半，榜单已经记录了955次通过校验的运行和301次有效突破，……整体压缩率达到8.4%。”“虽然他后来声明，25万页是他开的一个玩笑。”；“楼宇风运录”稿：“代码从约1462万行降到约1211万行，减少约251万行，降幅约16.5%。”“减少约251万行，并不是从原始的1737万行里直接删掉这么多，两个数字不能混为一谈。” | 21（取标题与口径段落；稿件日期与账号在图中） | **本期所有“240 万 / 1462→1211 / 16.5% / 1073.7 / 269.4 / 24.1 / 955 / 301 / 5048 / 8.4%”的最早中文出处是量子位 11-24 19:08 稿**（早于 grh-lean 两天）；“楼宇风运录”稿 11-26 18:45 自己写了口径提醒，但仍把 1211 万、251 万、16.5% 并排（三个数来自两个提交，见 4.1）。**这批数字与 grh-lean 无关** | 已找到 |
| B5-4 | “被压缩 99.5%”这一说法（头条号个人文章） | 今日头条“浩盛水流年”，2026-11-26 14:52；图“疑似AI生成” | https://www.toutiao.com/article/7712368986859635261/ （`b-cn-toutiao-correction.txt`，opencli 取） | “被压缩的不是数学证明本身，而是它的Lean形式化代码。”；“最终论证只占整个代码库的6.6%，其余绝大部分是在构建证明所需的工具 …… 7.6万行紧凑版所做的，是通过彻底重构移除脚手架”；“精简版延续的是同一条被验证过的逻辑脉络”；“Lean社区对原版证明的独立核验已经确认了其正确性” | — | **把 rh-trim 的 6.6%（原版“最终论证占比”）当作紧凑版“移除脚手架”的依据**，混了两个口径；“独立核验已确认正确”指**原版**，对短版的复核是 MIT shortgrh 页（B2-5），文中未提；也写了“精简版 README 坦率地写道仅通过机器检查” | 已找到 |
| B5-5 | “三个数字被混在一起”纠错稿（今日头条 11-25 14:46） | — | — | — | — | **未找到**：本期搜索（Yahoo 经 opencli）未命中该标题；已存的“楼宇风运录”稿（B5-3，11-26 18:45）有类似口径提醒，但时间与发布方都对不上扫描所述。**不要把图 21 标成那篇稿** | 未找到一手来源 |
| B5-6 | 澎湃 / 量子位 11-26 07:14 稿 | — | — | — | — | **未找到**：找到量子位 11-26 08:40 稿（腾讯新闻，B5-1）；澎湃号本期只找到 11-24/25 的转述文章（`thepaper.cn/newsDetail_forward_34547514` 等，未存档）。07:14 时刻无对应页面 | 未找到一手来源 |
| B5-7 | 有无稿件把 grh-lean 写成“删掉冗余后的原证明”或“数学家已审” | — | `b-cn-toutiao-correction.txt`（B5-4）；`b-cn-geekfan-995`（极客公园，标题“黎曼猜想AI证明代码暴砍99.5%只剩7.6万行，又是Anthropic”） | 极客公园：“大规模重写精简”“整套证明逻辑不变、同样通过多工具交叉验证”；头条号：“移除脚手架”“同一条被验证过的逻辑脉络” | — | 未见“数学家已审”的说法；“只过机器检查”在量子位与夕小瑶稿里都写了。“移除脚手架 / 逻辑不变”的写法见于头条号与极客公园 | 已找到 |

## 七、B6 anyGeometry 质疑（次要）

| # | 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B6-1 | 质疑者与原文 | L6（个人 X 帖与预印本页；PhilArchive 页被 Cloudflare 拦截，未绕过） | X：https://x.com/QualiaQuanta/status/2124984931063673166 （fxtwitter 公开 JSON，只存帖文，`b-nielsen-x-excerpt.txt`）；预印本 https://philarchive.org/rec/NIEIOT-2 （`blocked`） | 帖文：“Minimal Mathlib example with zero `sorry` and no custom axioms: `anyGeometry` is `True`, `height` is assigned `targetFormula`, and `h_eq` is `rfl`. … `def anyGeometry : ∀ (_ : Nat), Prop := fun _ => True`”（作者署名 Jenny Lorraine **Nielsen**）；搜索摘要：“Alpöge’s claimed proof of the Riemann Hypothesis is invalid. The Lean code does not construct an independent geometric height and then check it against the analytic side.” | — | 帖时间 UTC 11-23 21:54:22 = **北京 11-24 05:54:22**；dev.to（Frank Chu，“Posted on Nov 22”）称她“same day … on PhilArchive”发表，故预印本约在 11-21/22。**姓是 Nielsen，不是 Nelson**；“堪萨斯大学博士生”只在腾讯研究院转载稿（B6-2）出现，PhilArchive 页未打开，**未核** | 部分支持 |
| B6-2 | 澄清长文 | L4/L5（腾讯新闻转载腾讯研究院，署“王盛颐 上海期智研究院研究员”） | https://news.qq.com/rain/a/20261125A0C27Z00 （`b-cn-qq-clarify`，2026-11-25 17:11 发布于北京） | “anyGeometry 这个定义在Levent Alpöge的源码中确实存在，而且在 24 个文件中被实际调用。但是，……这根本不算什么大事，也绝不可能推翻证明的有效性。”“真正的问题在于，anyGeometry 质疑错了地方。” | — | **早于 grh-lean 上线**，所指是原版 | 已找到 |
| B6-3 | 定义位置与文件数（原版） | 实测（GitHub 代码搜索） | `gh api search/code`，`q=anyGeometry repo:homeowmorphism/riemann-hypothesis-original`（该库为原版逐字镜像）；`b-gh-code-anygeometry-orig.json` | 定义：`def anyGeometry : GeoP := fun _ _ _ => True`，在 `src/Development/AbelianVarietyTheory/Definitions/AutomorphicMembership/OverIx.lean`；上一行为 `def geoOf (Q : …) : GeoP := fun D J X => (∃ c : ℕ, 0 < c ∧ GlobalTheory.ShFam.IsGeoOf (Q D) J c X) ∧ GlobalTheory.ShFam.Pad.HasTrueSurf X`；**代码搜索 total_count 23 个文件（含定义文件）**（incomplete_results false） | — | dev.to 称“in 24 files”（`git grep`，11-20 克隆），腾讯稿也写 24：**23 vs 24 并列记录**，差 1 未查明（可能含 `Challenge` 之类或索引差异） | 已找到 |
| B6-4 | 该定义在 grh-lean 里是否存在 | 实测 | 命令：`grep -rIil anygeometry . --exclude-dir=.git`（在 grh-lean 克隆根目录，大小写不敏感） | **无输出（0 个文件）**，两版均无 | — | 即扫描“代码位置指向 grh-lean”**与说法不符**：质疑（11-21/24）早于 grh-lean（11-26 04:38）上线，所指为原版；短版里已无该名字 | 与说法不符 |
| B6-5 | 谁对 | — | — | 只记事实，不判 | — | MIT 页对原版与 grh-lean 的复核只覆盖“编译、公理、陈述比对、内核重放”，**不涉及“该定义是否令几何论证空洞”这一语义问题**（页面无此句） | — |

## 八、B7 热度与质疑

`barking heat`（`b-heat.json`，北京 21:42）：HN item 50648050 “Much shorter version of GRH Lean verification from Anthropic”，**489 分 / 281 评**（条目 JSON 内嵌 259 条评论；页面渲染 21:51 显示 489 / 282），提交 北京 11-26 05:48:10；Show HN 50654144 “A textbook on the Lean proof of GRH in lsernau/grh-lean, for students”，1 分 / 0 评，19:30（只作线索）。**Reddit 两组搜索均被重定向到登录页（`old.reddit.com/login`），按规则未登录，失败**，未取 r/math、r/Lean、r/programming 热度。

有理有据的评论（L6，仅作线索，进正文前须回一手来源）：

| # | 评论 | 得分 | 链接 | 所指 |
|---|---|---|---|---|
| 1 | gregdeon：“It's worth noting that the 76k line version is heavily "golfed" (e.g., cramming the same content into fewer lines), which heavily hurts readability for the sake of a smaller line count. A more sane version is around 300k lines. … The proof also still contains a few unused intermediate results.” | 评论分数 HN 不公开 | https://news.ycombinator.com/item?id=50650263 | 行数能否说明冗余；与 Sernau 的 Zulip 原话一致（B1-9） |
| 2 | w-m：“the community-driven compression with a hundred contributors and thousands of pull requests sits at a mere 33% reduction. Why this huge discrepancy?” | 同上 | https://news.ycombinator.com/item?id=50648779 | 对比 VerifyTask 33%（B3-3）与 99.56%：**基线不同**（见 B3 与零节）；jychang 回复称那可能是错的看板，并说他本人倾向“从零重新形式化”而不是缩原版 |
| 3 | lelandfe：“If this were “from Anthropic” surely it would be hosted on an Anthropic account?” | 同上 | https://news.ycombinator.com/item?id=50648275 | 谁写的短版、官方性（B1-1：仓库在个人账号，LICENSE 署 Anthropic） |
| 4 | delegate / keyle：不懂 Lean，问人能否通读并确认其正确（“How realistic is it for a human to read this code, 'grok' it and verify that it is correct?”） | 同上 | https://news.ycombinator.com/item?id=50648814 、…50648959 | 短版是否被人审过：README 自称只过机器检查（B1-3） |
| 5 | bananaflag：“From ~17M lines to just ~76k lines, a 99.56% reduction.” | 同上 | https://news.ycombinator.com/item?id=50648590 | 99.56% 的最早出处之一（分母 17M；Sernau 自述分母是 15M） |

“短版是否独立审过”：除 MIT 页外，Zulip 中 Riccardo Brasca 在 06:13 前自跑 `lake comparator --paranoid`（lean4lean 在可读版上 125GB 内存不足，见 `b-zulip-excerpts.md` 消息 643710501 引文），Sernau 答“Works for me on con-leche and con-ron, but lean4lean OOMs”（消息 643721404，未摘录入表）。

## 九、可能的吠点（说法 vs 证据）

1. **“99.5%”的分母**：说法（量子位标题）“暴砍99.5%”；证据：Sernau 原话“from about 15 million lines to ~76,000”（分母 15M，B1-6）；量子位正文“99.56%”用 17.37M；76,149 是 cloc code 行，17,374,215 是总行（B2-4 / 一）。**同一句里分子是“code 行”、分母是“总行”** 是中文稿与 HN 评论的共性。
2. **“砍”不是“删”**：说法“砍掉 / 暴砍”；证据：README “a shorter rewriting … follows that proof's method; many steps are proved by other arguments”（B1-2）；Sernau “resolution of singularities is gone entirely; … Vandiver, patching, Hida theory, Gsp4, local Langlands, there were some big cuts”（B1-8）。即**换了部分论证**，不是把冗余代码删掉；也就**无法用“原版 99.5% 是废话”解读**。
3. **紧凑版 ≠ 可读版**：76k 行是“code golfed”（B1-9），README：“The proofs are written to be short, not to be easy to read.”；可读版 300k 行（299,902 code / 477,124 总行）才是给人读的；两版证明“some of its steps use other arguments”，不是同一份证明的两种排版。
4. **只过机器检查**：说法“短版可信”；证据：README Status：“The proof has passed machine checks only … No mathematician has read it through, and no paper comes with it.”；MIT shortgrh 页确认两版编译、公理、内核重放、陈述比对全过（B2-5），但页面**不涉及数学审读**；Lean FRO / Mathlib 声明（原版）未延伸到短版（B2-6，推断自沉默）。
5. **仓库在个人账号**：`lsernau/grh-lean` 非 Anthropic 官方账号；Anthropic 新闻 / 研究页无对应条目（B1-1）。
6. **“社区删冗余”统计：换了版本再比**：rh-shrink 曲线在 11-26 06:24 从 ~75%（换版前 24.77% 降幅，原版口径 11,000,328 / 14,621,937）跳到 2.0%（可读版 298,160），**跳变是换版，不是删除**（B3-1，截图 16）；VerifyTask 33.52% 是原版 39 个核心模块的 token 口径（B3-3）；rh-trim 的 ~8% 是“仅删死代码且从未零错构建”的上界（B3-4）；rh-proof-trimmed 的删减（S4 后 14.25M / Base 16.59M）**构建与内核通过**（B3-5）。**把这几个百分比并排比较是跨口径**。
7. **文件数降幅也是“99.5%”**：rh-shrink 页“Files 801，99.5% down”（175,187 → 801）（截图 16）——中文稿“99.5%”的另一可能来源，扫描未提。
8. **短版含未用结果**：README “Several hundred of the more than 20,000 declarations are not used by the five theorems”；Sernau “finish the thought on some lemmas … rather than using just the fragment the proof needed”（B1-10）。“短 = 去掉了不必要的”不准确。
9. **“25 万页是玩笑”**：原文是脚注 “I of course amused myself, and hit 250, 000+ pages.”，未写 joke（B3 汇总）。
10. **anyGeometry**：质疑与澄清均早于 grh-lean；原版 23（代码搜索）/ 24（dev.to）个文件含该名，grh-lean 两版 0 个文件含该名（B6）。**不要把 grh-lean 写成“修了 anyGeometry”**（无证据显示因果，只是名字已不存在）。

## 十、夸大说法实例

见 B5-1（量子位 / IT之家 / 腾讯新闻 11-26 标题“暴砍 99.5%”）、B5-2（夕小瑶标题“被砍掉了 99.5%”）、B5-3（量子位 11-24 标题“两天砍掉 240 万行”）、B5-4（头条号“通过彻底重构移除脚手架”并混用 6.6%）。英文侧：HN 帖标题 “Much shorter version of GRH Lean verification from Anthropic”（中性）；HN 评论 “a 99.56% reduction”；OfficeChai 标题 “Anthropic Researchers Use Claude To Shrink Riemann Hypothesis Proof From 17.4 Million Lines Of Lean To 76,000”（`b-cn-officechai`，11-26，L5）：“That is a 99.56% cut, and the new proof is roughly 228 times smaller.”——“Shrink”也把重写写成了压缩。

## 十一、扫描说法勘误

1. **“99.5% 是把三个数字揉成一句”**：与说法不符。99.5% / 99.56% 的分子都是 grh-compact（76k）、分母是原版（15M 或 17.37M）；“三个数字”（240 万 / 8.4% / 99.5%）是三个不同口径，在 11-24 量子位稿与 11-26 量子位稿里各自出现，**不是同一句里的错误**，但互相无法比较（零 / 一 节）。
2. **“今日头条‘三个数字被混在一起’纠错稿 11-25 14:46”**：未找到；发布时刻也无法核。
3. **“澎湃 / 量子位稿北京 11-26 07:14”**：量子位稿在腾讯新闻上是 11-26 **08:40**；07:14 未找到。
4. **“8 天”**：未找到。
5. **“rh-kernel-backbone 称 1737.4 万行里被主证明用到 1073.7 万行、269.4 万行死代码”**：数字都对，但 1073.7 万与 269.4 万是 **decl 行**（声明行，总数 13,432,480），不是 1737.4 万里的行；两数之和加其余部分不等于 1737.4 万（B3-2）。“约 24.1 万条重复定理”：该站无此数，**未找到**。
6. **“rh-trim 称 allChi 一步占 38%”**：与说法不符。README 写“99.35% … under a single statement, allChi”，“One step alone owns 38%”的步骤未点名（B3-4）。
7. **“没有精简版零错误构建通过”**：只对 rh-trim 成立。rh-proof-trimmed 的 S1/S2/S4 与社区 `main`（PR 合并，CI 构建）的删减版都是编译通过的；扫描写成笼统句不准（B3-4/B3-5）。
8. **“rh-shrink 去空行、注释与引用声明后有效代码约 1462 万→约 1211 万行、减 16.5%（Claude 按这两个数算是约 17.2%）”**：Claude 的算术是对的；16.5% 是另一个提交（12,214,914）的降幅，1211 万是 12,108,510 的值，二者在中文稿里并排（B3-1、B5-3）。另：**rh-shrink 现在显示的已不是这组数**（换版后“2.0% left”）。
9. **“VerifyTask 整体压缩率 8.4%、39 核心模块、5048 万、955 次、301 次”**：不是现行数，是 11-24 凌晨–上午的快照；现行（北京 11-26 21:41）33.52%；核心已解 42 / 48；第一季已冻结，第二季改到 grh-compact（B3-3）。
10. **“rh-kernel-backbone 量的应是原版”**：对（README 写 baseline `v0.2.1`、Lean v4.33.1），而且它自己写明 “since 2026-11-25 upstream main is based on the shorter … proof”（B3-2）。
11. **“扫描：Jenny Lorraine Nelson；代码位置指向 grh-lean”**：姓是 **Nielsen**（她本人帖与论文署名）；位置在**原版**（`…/AutomorphicMembership/OverIx.lean`），grh-lean 无此名（B6）。
12. **“25 万页是 Alpöge 自认的玩笑（1120 期已记 25 万页出处是作者脚注）”**：脚注原话是 “amused myself”，无 “joke”（B3 汇总）。
13. **“README 只写 based on Alpöge's … a shorter version，没说是从原版删出来还是另写”**（Claude 假设一节）：子目录 README 明写“shorter rewriting”“follows that proof's method; many steps are proved by other arguments”（B1-2）。
14. **“HN 487 分 / 274 评”**：北京 21:42 为 489 分 / 281 评（`barking heat`），21:51 页面 489 / 282；数字随时间涨。
15. **“github 星标 741”**：北京 21:38 为 744，21:48 渲染 747。

## 十二、对读者

| 项 | 已核实事实 | 出处 |
|---|---|---|
| 想自己验 | grh-lean 两个子目录各自是 Lake 项目；需 Lean v4.35.0 与 Mathlib 提交 56a00585；“A build takes one to two hours on 12 cores with 24 GB of memory (about 85 minutes more if the Mathlib cache is missing)”；命令 `lake exe cache get`、`lake build`、`lake env lean scripts/PrintAxioms.lean`；Comparator “allow 64 GB of memory and a few hours” | `b-grh-lean-README.md`；`b-grh-readable-README.md` |
| 想读 | 可读版从 `grh/GUIDE.md` 起读；有 `CONTENTS.md`（每个文件的标题）、`INDEX.md`（结果与出处）、`REFERENCES.md`；**没有配套论文，没有数学家通读** | 同上；B1-3 |
| 受不受影响 | 不影响此前对原版的结论：Lean 官方对原版的认证（1122 期已发）与 MIT 对原版的复核（B2-5）都不因短版改变；短版的复核另有 MIT shortgrh 页 | B2-5、B2-7 |
| 许可 | grh-lean 为 Apache 2.0，“Copyright 2026 Anthropic, PBC”；社区仓库 README：原版“was originally published without a license”，社区仓库现也用 Apache 2.0 | B1-1、B2-7 |
| 社区仓库现状 | `leanprover-community/riemann-hypothesis` 的 `main` 是可读版；`alpoge` 分支留着换版前的原版清理成果；`original` 分支留着原版；另有 `homeowmorphism/riemann-hypothesis-original` 逐字镜像与预构建缓存 | B2-7；`b-lc-rh-readme-main.txt` |
| 用不到 | 与普通读者的订阅、价格、额度无关 | — |

## 十三、未完成与失败

- B1-12：`PrintAxioms` 与 `lake build` 未在本机跑（需 Lean 工具链与数小时），只给作者自述 + MIT 复核页。
- B3-3：VerifyTask 页面浏览器打不开（`ERR_CONNECTION_CLOSED`），headless 渲染无样式无数据；17 号位（VerifyTask 榜单）**未截**，改放 rh-proof-trimmed 阶段表（见“十四、截图清单”）。
- B7：Reddit 被登录墙拦，未取。
- B5-5、B5-6、“8 天”、“24.1 万重复定理”：未找到。
- PhilArchive 页（质疑者论文）被 Cloudflare 拦，未绕过；知乎问题页 403。
- 未做 grh-lean 与原版代码比对（按派工单不下载原版；仅比了 `Statements.lean`）。

## 十四、截图清单（`images/`，均已逐张打开目视确认；深色底页面多数）

| 文件 | 内容 | 取法 |
|---|---|---|
| 13-grh-lean-repo.png | grh-lean 仓库首页（About 描述 + 文件列表 + README 全文，含 Two editions / Build / Status / License） | headless Chrome（dark 主题，1000 CSS px，DPR 2）；已裁去仓库所有者头像与提交栏；About 块被裁窄（“About / A short Lean 4 proof of the Generalized Riemann Hypothesis”左侧顶格） |
| 14-grh-rewriting-para.png | `grh/README.md`：“shorter rewriting … follows that proof's method” 段 | 同上，blob 页，裁 README 正文 |
| 15-grh-checks-layout.png | `grh/README.md` Checks 与 Layout（机器检查清单、行数、文件数） | 同上 |
| 16-rh-shrink.png | rh-shrink 页首：2.0% / 296.61K / Files 99.5% / “Nov 25: main replaced by the readable edition…” 曲线 | opencli 浏览器，zoom 1.5（约 DPR 1.5，非 2），已隐藏浏览器翻译插件浮层 `immersive-translate-popup` |
| 17-trimmed-stages.png | rh-proof-trimmed 阶段表（Base→S1/S2/S4，编译 / 内核 / 陈述） | headless Chrome，1400 CSS px，DPR 2；**占用原计划“VerifyTask 榜单”的号位** |
| 18-rh-trim-conclusion.png | rh-trim 顶部状态与“Three questions, three answers” | headless Chrome |
| 19-backbone-numbers.png | rh-kernel-backbone README 表（175,187 files、10,737,728 decl lines 等） | headless Chrome，1400 CSS px |
| 20-cn-qbitai-995.png | 腾讯新闻转量子位“暴砍 99.5%”标题与前文，含 Sernau 原话译文“约1500万行→约76,000行” | headless Chrome，700 CSS px |
| 21-cn-correction-240.png | “楼宇风运录”稿标题与“1462→1211、16.5%、不能混为一谈”两段（三段拼接，间隔 20 px，暗底） | headless Chrome，700 CSS px（高窗口以避开悬浮按钮） |
| 22-hn-thread.png | HN 帖标题与分数、bananaflag“99.56%”评论、gregdeon“golfed”评论（三段拼接） | headless Chrome，900 CSS px |
| 23-rh-shrink-method.png | rh-shrink “How it's measured”（文本检索，不跑 Lean，换版说明） | headless Chrome，1000 CSS px |
| 24-zulip-sernau.png | Lean Zulip：Sernau 宣布（“about 15 million lines to ~76,000”） | opencli 浏览器，zoom 2；已裁去头像（只留话题标题条与消息正文） |

未用于正式配图的备用：全页渲染中间图在系统临时目录 `barking-shot/26111125-b/`（待删除，见回报）。
