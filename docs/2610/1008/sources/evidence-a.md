# 1008 期 A 组取证：OpenAI 数学撤稿（跟进 1006）+ Navier–Stokes 的 Lean 质疑论文

取证方：Sonnet 子代理（A 组）。时刻一律北京时间（UTC+8），括号保留原时区；官方页只有日期的写“官方未给时刻”。存档文件都在本目录（`a-` 前缀），截图在 `../images/`（01–14）。PDF、HTML 原件、图片只存本地，不入库。引文逐字、保留原语言。抓取过程与失败见 `capture-log-a.md`。

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。**核验与推断分开写**：“核验”指我在原件里逐字对过；“推断”会明说。

## 0. 先给结论（决定正文能否成立的几件事）

1. **三篇撤稿从未出现在形式化清单里**。1006 存档的 `m-oai-lean-formalization.yaml` 与 `openai/math` 初始提交 `adc7f12` 的 `lean/formalization.yaml` **字节相同**（`cmp` 通过）；三篇的论文目录名（见下）在旧、新两版 yaml 的 `sources`、`main_results` 与全文子串中都是 0 次命中（不靠关键词，按完整目录名比对；关键词 eightfold / Kuga / K3 / Hodge / abelian / Weil 在两版 yaml 里也都是 0 次）。三篇在 1006 的 `CONTENTS.md` 里属于第 032 号成果族，该族标题行没有 `([Lean](lean/docs/032.md))` 链接（新旧两版 CONTENTS 都没有 `lean/docs/032.md`）。脚本与输出：`a-yaml-diff.py`、`a-yaml-diff.out.txt`。
2. **300 / 719 的统计口径在仓库文件里复算不出来**（见 A2-4）。正文不要把 300 当成可核的篇数。
3. **质疑论文的 Example 3.1 与 3.3 的对比，对照原件基本属实**：NL 论文 (8.19) 确为 C^{m+4}（OpenAI 论文第 95 页），被引 Lean 定理确为 m+5（`SmoothFamilyTorusInverse.lean` 第 1059–1080 行，f9e8bc5），(10.19) 与 Lean 的 `exists_uniform_actual_pressure_flux_bound`（`R3/PressureFlux.lean` 第 576 行）两式与论文所引逐字一致。**但**：(a) 论文 Figure 3 的 ChatGPT 对话截图里的行号（978–999、988–997、336–354）与 f9e8bc5 对不上；(b) 作者自己配套的“更多错译清单”是 AI 生成、自注“未全人工核”；(c) 作者明说“不对 NL 证明的正确性作论断”。
4. **三篇撤稿的原因、范围，以及“撤回的是证明，不是命题为假”**，在三份 README 里写得明白（逐字见 A1-3）。
5. **陶哲轩没有“牵头/主持”AHM 声明**：陶博客 10/7 的帖子是 guest post 转载（页首注明）；AGMAI 成员名单里也没有陶哲轩。详见 A7。

## 1. 清单表

表头：说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态

### A1　撤稿：history.md、提交 3014888、三篇 README

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1-1 history.md 的撤稿、修订、形式化三段 | L1 | https://github.com/openai/math/blob/main/history.md ；存档 `a-history.md`（raw） | “## October 7, 2026”；“In “Algebraicity of Weil classes on split abelian eightfolds” a sign error invalidates a stabilization-trace cancellation argument and the construction used by two dependent papers. As a result, we have withdrawn the following three manuscripts:”；“We have revised 14 other manuscripts with proof repairs, corrected statements, clearer hypotheses and dependencies, and one correction to an obsolete citation.”；“Also, as a consequence of these fixes we updated 13 additional manuscripts to cite the revised editions of companion papers.”；“We have added an additional 6 formalizations and 5 other additions covering supporting results. This brings the total percentage of top-line results formalized to 300 / 719 = ~42%.” | 01、02 | 页内标题日期“October 7, 2026”（官方未给时刻，也没写时区）。与 README 的“Withdrawn on October 6, 2026”、提交时间不一致，见 A1-2、A1-3，不自行取舍。文件头一句：“For any withdrawn papers, their README files explain the gap and link to the retracted manuscript.” | 已找到 |
| A1-2 提交元数据 | L1 | `repos/openai/math/commits`（GitHub API）；存档 `a-math-commits.json`、`a-commit-3014888.summary.json`、`a-commit-3014888-name-status.txt` | 3014888：message “Update manuscripts and Lean formalizations”，作者 Dan Roberts，`2026-10-08T05:03:50Z`（= 北京 10/8 13:03:50；美西 10/7 22:03:50 PDT）。该提交相对 `adc7f12` 改动 1185 个路径（A 1135、D 31、M 19，`git diff --name-status`）；API 的 files 数组只给了 300 项（已截断，摘要里注明）。 | — | **派工单写“最新提交 3014888”不准**：main 的最新提交是合并提交 `fd4aeeb`（James R Lee，`2026-10-08T05:20:00Z` = 北京 13:20:00，“Merge pull request #1 from openai/codex/update-10-7”）；PR #1 由 dr-openai 于 `05:18:38Z`（北京 13:18:38）创建、已关闭（合并）。合并前后树相同（`git diff 3014888 fd4aeeb` 为空）。初始提交 `adc7f12` “Initial commit”作者标 Anonymous，`2026-10-06T21:58:50Z`（北京 10/7 05:58:50）。 | 已找到（“最新提交”一说：与说法不符，记入勘误） |
| A1-3 三篇 README 的撤稿说明 | L1 | 同仓库 `preprints/<目录>/README.md`@fd4aeeb；存档 `a-withdrawn-README-*.md`（三份） | Weil：“The proof contains a sign error in the stabilization-trace argument used to obtain negative double points. In the manuscript's signed double-point convention, write $I(f_1)=-m$ with $m>0$. The proof assigns each reverse stabilization trace sign $+1$ and therefore claims that inserting $m$ such traces makes the signed count zero.” / “Accounting for the opposite source orientations of the two branches of the standard cusp gives sign $-1$ for each reverse trace in this convention. The resulting count is therefore $$I_{\mathrm{new}}=I(f_1)-m=-2m\ne0.$$” / “The Eliashberg-Murphy cancellation theorem invoked at this step requires zero signed double-point count, so its hypothesis is not met. The subsequent oriented-surgery and embedded-brane construction is therefore unsupported, and the paper does not establish its claimed algebraicity theorem.” / “This withdrawal concerns the proof; it does not assert that the mathematical statement is false.”<br>Kuga–Satake 与 Hodge：“The proof relies on an adaptation of the flawed stabilization-trace construction in [Algebraicity of Weil classes on split abelian eightfolds](…). The sign error in that construction prevents the required signed-double-point cancellation.” 后接各自“leaves this paper's claimed … unproved”，并同样写 “This withdrawal concerns the proof; it does not assert that the mathematical statement is false.” 三份都有“Pre-withdrawal PDF”链接，指向 `adc7f12` 版本。 | — | 三份 README 都写 “**Withdrawn on October 6, 2026.**”（无时刻无时区），history.md 标题却是 October 7。论文目录名（含原日期）：`Algebraicity-of-Weil-classes-on-split-abelian-eightfolds-September-18-2026`、`Algebraicity-of-Kuga-Satake-Correspondences-for-K3-Surfaces-October-3-2026`、`The-rational-Hodge-conjecture-for-products-of-K3-surfaces-October-4-2026`。提交 3014888 还删除了三篇的 `build/` 下 TeX 源文件（`git diff --name-status` 里的 D；PDF 保留）。誰发现了错误、什么时候发现：README 与 history.md 都没写（核验：全文无）。 | 已找到 |

### A2　形式化覆盖

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A2-1 当前 yaml 存档 | L1 | `lean/formalization.yaml`@fd4aeeb；存档 `a-formalization-fd4aeeb.yaml`（79 KB） | 头注：“# Catalog of papers with a formalized main result. Paths are relative to lean/.”；`status:` 下 “scope: "Partial progress."”；`review:` 下 “status: unchecked”；`automation:` 下 “method: agent”。 | — | 现 `sources` 173 条（均不同目录）、`status.main_results` 200 条（192 个不同文件）。旧版（=1006 存档）162 条、185 条（180 个文件）。 | 已找到 |
| A2-2 与 1006 存档逐条比对 | L1 | 同上 + `m-oai-lean-formalization.yaml`（1006 存档）；脚本 `a-yaml-diff.py`，输出 `a-yaml-diff.out.txt` | 新增 `sources` 11 条（目录名）：A-Complete-Local-Domain-Without-a-Small-Cohen-Macaulay-Module；A-counterexample-to-Hadwigers-conjecture；A-stable-coordinate-that-is-not-a-coordinate-in-four-variables；Almost-everywhere-Fourier-convergence-in-L-log-L；An-explicit-power-saving-for-the-exact-discrete-Fourier-transform；Critical-honeycomb-chords-with-prescribed-boundary-endpoints；Exact-Derandomization-of-Logarithmic-Space-L-equals-RL-equals-BPL；Integral-points-on-character-varieties-of-curves；Simulating-One-Tape-Time-in-Two-Fifths-Power-Space；The-trace-cone-classifies-Razak-Jacelon-stabilizations；Uniform-Cartier-sections-for-Fano-type-contractions（日期后缀略）。删除 0 条；同目录标题变更 0 条。`main_results` 新增 11 条 comparator_config，删除 0 条。 | — | 1006 存档的 yaml、README、CONTENTS 与 `adc7f12` 的同名文件逐字节相同（`cmp`）。新增 11 条与 history.md “6 formalizations and 5 other additions” 数量吻合（11 = 6 + 5，推断：这是同一批）。 | 已找到 |
| A2-3 三篇撤稿是否曾在清单中 | L1 | 同上；`a-yaml-diff.out.txt` “withdrawn dirs” 一节 | 三个目录名在旧、新 yaml 的 `sources`：False/False；在 yaml 全文子串：False/False。关键词计数（旧/新）：eightfold 0/0，Kuga 0/0，K3 0/0，Hodge 0/0，abelian 0/0，Weil 0/0。CONTENTS：三个目录名都在旧版出现、新版不出现；第 032 号族 1006 标题为 “Hodge and Kuga–Satake results for all projective K3 surfaces.”，现改为 “The rational Hodge conjecture for CM abelian varieties.” | — | 结论：**三篇从未被形式化清单收录**（“形式化清单”= `lean/formalization.yaml`）。方法：按论文目录名精确比对，不依赖关键词。机器之心的话“被撤回和实质修订的内容，几乎都落在没有 Lean 形式化的部分”（A8 的新浪转载）对这三篇成立；对另外 14 篇修订我没有逐一核（未核验）。 | 已找到 |
| A2-4 “300 / 719 = ~42%” 的单位与计数关系 | L1 | history.md；README；yaml；CONTENTS | “300 / 719 = ~42%”；README 现句：“The repository has ~42% top-line results formalized.”（旧句：“Many, but not all, of the manuscripts have been formalized.”） | — | **我复算不出 300。** 719 = 722 − 3：README “The current catalogue contains 719 manuscripts”（旧 722），可信。分子：yaml `sources` 173；`main_results` 200 条（192 个不同文件）；CONTENTS 带 `[Lean]` 链接的成果族 242 / 372 族（旧 235），这些族内的手稿 475 篇（旧 467）。没有任何一种口径给出 300。机器之心称“300 篇手稿的主结果”“按单篇主结果统计”，是其解读，yaml 无法复核。“top-line results”的定义 OpenAI 没写。 | 部分支持（719 可核；300 无法由仓库文件复核） |
| A2-5 README 现行原句 | L1 | https://github.com/openai/math/blob/main/README.md ；存档 `a-math-README-fd4aeeb.md`、`m-oai-README.md`（1006 版） | “This collection includes results at different stages of verification. Not all have accompanying Lean formalizations.”；“Some of the unformalized results could have issues. We will endeavor to fix any such issues quickly.”（两版都有，未改）；“The current catalogue contains 719 manuscripts organized into 372 families.”；“Updates to the repo are described in the [history](history.md).” | — | 新旧 README 只差两处（见 A2-4）。 | 已找到 |

### A3　质疑论文 arXiv 2610.08144

存档：`a-ns-lean-critique-v1.pdf`（arXiv v1，25 页）、`a-ns-lean-critique-v1.txt`（`pdftotext -layout`）、`a-ns-arxiv-abs.html`。发表：arXiv 只有 v1，“Tue, 6 Oct 2026 10:58:01 UTC (1,080 KB)”= 北京 10/6 18:58:01；arXiv 备注“25 pages, 4 Figures”；类别 math.AP（分类页还列 cs.AI / math.LO，我未在 abs 页逐项核对，**未核验**）。HN 提交 `2026-10-07T15:24:38Z`（北京 10/7 23:24:38）。以下页码均为 v1 PDF 页码（与印刷页码一致）。

**版本提醒**：作者主页还挂着另一份同题 PDF（`http://www.damtp.cam.ac.uk/research/afha/anders/NavierStokes_LostInTranslation_Final.pdf`，31 页，PDF 创建时间 2026-10-05，存档 `a-damtp-ns-final.pdf/.txt`），多出 Appendix B；arXiv v1 里 Appendix B 的清单改为单独文件 `Navier-Stokes_Experiment.pdf`（7 页，存档 `a-damtp-ns-experiment.pdf/.txt`）。下表摘句除注明外都取自 arXiv v1。

| 说法 | 级 | 一手来源 | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| A3-1 摘要 | L1（作者自述） | https://arxiv.org/abs/2610.08144 ，PDF p.1 | “The purpose of this article is to demonstrate why this process may offer no confidence in the original NL argument, owing to the various difficulties in performing the translation semantically faithfully.” “Hence, informally, providing semantically faithful AI autoformalisation is harder than any computational problem including the Halting problem (which has SCI = 1).” “These include OpenAI’s announced Navier-Stokes proof. In particular, we show that the formalised Lean proof does not correspond to the NL proof of blow-up of solutions to the Navier-Stokes equations.” | — | 注意措辞是“Lean 证明与 NL 证明不对应”，不是“证明是错的”。 | 已找到 |
| A3-2 免责句 | L1 | p.2 | “Disclaimer: We do not make claims about the correctness of OpenAI’s NL proof, we only make statements about mistranslations into Lean.” | 06 | 在引言第二页，与下一行同段。 | 已找到 |
| A3-3 “merely tells us” | L1 | p.2 | “The Lean code merely tells us that the theorem is correct, yet the NL proof with intermediate arguments, lemmas and results may be incorrect or have incorrect arguments.” 前一句：“Because of the phenomenon of mistranslations – as highlighted in this paper – the NL proof by OpenAI and other autoformalised Lean proofs should not prima facie be trusted without the same peer review process and scrutiny that other proofs are subjected to.” | 06 | “theorem is correct”这一让步值得保留：作者认可 Lean 层面的定理陈述成立。 | 已找到 |
| A3-4 Example 3.1 全文要点 | L1 | p.4–7 | “Example 3.1 (The NL paper claims a stronger inverse estimate than the cited Lean formalisation). Figure 3 concerns the estimate in Lemma 8.6, equation (8.19), of OpenAI’s Finite time blowup for Navier–Stokes. We compare this estimate with the cited Lean declarations at commit f9e8bc5 … The issue is a change in the order of the input derivatives required to bound the output derivatives.” (3.1) NL (8.19)：‖N⁻¹F‖_{C^m_y} ⩽ C_m‖F‖_{C^{m+4}_y}；(3.2) Lean：‖N⁻¹F‖_{C^m_y} ⩽ K_m‖F‖_{C^{m+5}_y}。“Summary: The Lean code produced by OpenAI often has stronger conditions on bounds on the derivatives of various functions than the corresponding results in the NL paper.” p.6：“The related Lean codes – on the other hand – all use five additional derivatives, not four.” 文件与行号：“File: NavierStokes/SmoothFamilyTorusInverse.lean. Declaration: norm derivativeWord inverse le, lines 1059–1080.” p.7：“This estimate is then strictly weaker than equation (8.19) due to the extra derivative on the right-hand side.” | 03 | 对原件：见 A4、A5（我核对后基本属实，并有细节限定）。 | 已找到 |
| A3-5 Figure 3 图注 | L1 | p.5 | “FIGURE 3. ChatGPT-6 (Astra Ultra) shares our concern that the NL argument in Lemma 8.6 in the announced proof of blow-up of Navier–Stokes has not been faithfully translated into Lean. In particular, there is a discrepancy between Lemma 8.6 and the cited Lean estimate: the latter uses one additional input derivative to control the same torus derivatives of the output. See Example 3.1 for details.” | 04 | **Figure 3 是一张 ChatGPT（“GPT-6 Astra Ultra”）对话截图**，不是论文作者手绘的图；对话里写的 “SmoothFamilyTorusInverse.lean, lines 978–999”“Lean, lines 988–997”“SmoothFourierData.lean, lines 336–354” **与 f9e8bc5 对不上**（f9e8bc5 中该定理在 1059–1080；`coefficient_seminorm_bound` 在 `SmoothFourierData.lean` 第 368 行；978–999 行是另一段 `nonbarPart` 相关代码）。论文正文的 1059–1080 是对的。原因未知（推断：对话时用的是另一份检出，**未核验**）。Figure 4（p.9）同为 ChatGPT 对话截图，引 OpenAI 论文 “p. 123”“pp. 122–123”，页码与原件相符。 | 已找到 |
| A3-6 Remark 3.2 | L1 | p.7 | “Remark 3.2 (Consistent mismatch between m + 4 and m + 5 derivatives). Similar estimates as (3.2) can be seen throughout OpenAI’s Lean code: for example, inverse finiteJets and mixedJet inverse bound each contain a similar m + 5 derivative requirement, rather than the expected m + 4 derivatives from the NL paper, as in (3.1).” “Consequently, the Lean results discussed in this section are weaker than (3.1) in the NL proof.” 脚注 5：“In fact, some such Lean results are actually incomparable with (3.1), due to the mismatch in the type of derivative.” | — | 作者自己承认部分结果与 (3.1) “incomparable”。 | 已找到 |
| A3-7 Example 3.3 对比式与三条原因 | L1 | p.8–10 | (3.4) NL (10.19)：|∫π w·∇χ_R| ⩽ (C_T/R)[(B_R+1)B_R^{1/2} + R^{−3/4}B_R^{3/4}]，for a.e. t∈(0,T)；(3.5) Lean：⩽ C_T[(B_R^{1/2}+1)(A_R/R + 1/R²) + R^{−7/4}B_R^{3/4}]，∀t∈(0,T)。“(1) The Lean proof does not rely on the boundedness of the Riesz transform as a map from L^{3/2} → L^{3/2}; it instead invokes Sobolev embedding to move to L² and uses the boundedness of the Riesz transform in L². The additional derivative involved in the Sobolev embedding is why (3.5) features A_R.” “(2) The Lean proof applies Hölder’s inequality with different exponents, which changes the bounds at the end.” “(3) In the Lean proof, the left hand integral in (3.4) is explicitly shown to be continuous in time. This ensures that the statement for almost every t ∈ (0, T) can be upgraded to a statement that avoids the ‘almost every’ qualifier.” 脚注 6：“the purpose of the lemma is to establish that w = 0 and hence the (equal to 0) bound on the right hand side of (3.4) is stricter than the (non-zero) bound on the right hand side of (3.5). However, this is only established after completing the entire proof of Lemma 10.5.” | 05 | 脚注 6 的意思：两个界在 w=0 时一个为 0、一个非零，且只有证完 Lemma 10.5 才知道 w=0（作者自己说明不好直接比较）。 | 已找到 |
| A3-8 Euler 的预测 | L6（作者预测，未逐条检查） | p.12 §3.3 | “It is beyond the scope of this paper to carefully examine the autoformalisation of the Euler proof in a similar way to what we have done in this paper with the Navier-Stokes case. However, based on some examinations of the autoformalisation of the Euler case – with both AI and manual tools – we think that the Euler proof will be another great case study for mistranslations in practice.” “Our prediction is based on preliminary examinations, but our assessment is that the likelihood of substantial mistranslations is very high.” | — | 作者自己写明未做 Euler 的逐条核对；这是预测，不是已发现的错译。 | 已找到 |
| A3-9 第 4 节与附录 A：SCI = ∞ 的层次 | L1（作者定理）；“SCI = ∞”为推论性论述 | p.13–23 | **Theorem A.3**（p.19）：“Fix k ⩾ 9. For any computable enumeration (p_e) of all polynomials with integer coefficients in k+1 variables, let B_e be as in (A.3). Then the set d := {e ∈ N : (∃n ∈ N) B_e(n)} is Σ⁰₂-complete. In particular, no AI can always determine whether the number n_e specified in Example 4.1 is well-defined, even with an oracle for the Halting problem.” **Theorem A.4**（p.21）：对 l ⩾ 2，d_l 是 Σ⁰_l-complete。**Corollary A.5**（p.22）：“SCI_A(Ξ_{d_l}) = l for every l ⩾ 2.” **§A.2.1**（p.22–23）：“Hence, the problem of answering the above question for all e ∈ N and l ⩾ 2 has SCI = ∞.”；其含义限定：“By ‘semantically faithful AI autoformalisation’ we mean a trustworthy AI autoformaliser that will always provide an output – as above. Also, by ‘any computational problem’ we mean any computational problem with finite SCI.” | — | **“SCI = ∞”没有挂在任何定理编号上**：Theorem A.3 是 Σ⁰₂-完备（k ⩾ 9），Theorem A.4 是 Σ⁰_l-完备，Corollary A.5 是 SCI = l，“SCI = ∞”是 §A.2.1 对“对一切 l 取并”的论述。“harder than the Halting problem”在引言与 §4 标为 informal（“informally”）；精确表述见 A.2.1 上面那句限定（针对“一个对任意输入总给出‘忠实译文或拒绝’的可信自动形式化器”，不是针对某个具体 Lean 翻译）。它与 Example 3.1/3.3 的错译是两条独立论证（推断，由上文结构读出）。 | 已找到 |
| A3-10 5.1 节 Meta 段及引文 | 作者转引（L6） | p.16、p.24–25 | “During the spring of 2026, Meta declared a new milestone in autoformalisation: they claimed to have translated 26 mathematics textbooks into Lean, forming a library with 45,000 verified Lean declarations and around 500,000 lines of Lean code [46]. These claims were, however, swiftly refuted by the Lean community in the discussion quoted below [32].” 引文 (i) Jack McCarthy：“…The reality is that 0% of the statements here are accurately formalized.” (ii) Brian Nugent：“…claiming “my AI autoformalized this AG textbook” in the state that’s its in is simply lying.” (iii) Christian Merten：“I would like to push back a bit on the point that this project has produced something useful on this front.” 引文号：[46] A. Rammal et al., Formalizing mathematics at scale, arXiv:2605.29955, 2026；[32] “Lean community. Atlas (Meta). Zulip discussion in the Autoformalization channel, 2026. Discussion thread. Accessed 18 September 2026.” 同段还写：“Anthropic’s recent autoformalisation of Fermat’s Last Theorem took 13 million lines of Lean code [10]”，[10] 为 K. Buzzard, Xena Project 博客，2026-09-04。 | — | **作者转引**：Zulip 帖子我没有取到（未核验）；“swiftly refuted”“simply lying”等是作者引述或评语。 | 已找到（作者转引，不是我核验） |
| A3-11 作者单位 | L1 | p.25 | “DEPARTMENT OF MATHEMATICS, KING’S COLLEGE LONDON”（A. Bastounis）；“DEPARTMENT OF APPLIED MATHEMATICS AND THEORETICAL PHYSICS, UNIVERSITY OF CAMBRIDGE”（F. Circelli；A. C. Hansen，通讯作者 a.hansen@damtp.cam.ac.uk） | — | 首页脚注 “*Corresponding author”。 | 已找到 |
| A3-12 作者的“更多错译清单”（配套文件） | L1 | `http://www.damtp.cam.ac.uk/research/afha/anders/Navier-Stokes_Experiment.pdf`；p.1 | “1.1. AI generated list (with some human input) of suggested mistranslations. … The twelve comparisons below are generated as a collaboration between ChatGPT and Claude, with some human input.” “Warning – Potential hallucinations: Since the list below is generated with the help of AI, and has not been fully manually checked, hallucinations may occur.” | — | arXiv v1 Remark 3.4 指向此文件（“click to view”）。DAMTP 31 页版里同样的话在 Appendix B.1。 | 已找到 |

### A4　OpenAI NS 原论文

存档：`a-oai-ns-paper.pdf`（167 页，2.9 MB，PDF 创建时间 2026-09-08）、`a-oai-ns-paper.txt`。来源 https://cdn.openai.com/pdf/32d9f210-8b73-45e0-91bc-82a30aef8a9a/navier-stokes.pdf 。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| A4-1 首页：标题、署名、摘要、主定理 | L1 | PDF p.1 | 标题 “FINITE TIME BLOWUP FOR NAVIER–STOKES”，署名 “OPENAI”（无个人作者，页内无日期）。“Abstract. For every positive viscosity, we construct a solution of the three-dimensional incompressible Navier–Stokes equations that starts from rest and develops unbounded velocity in finite time while maintaining uniformly bounded kinetic energy.” “Theorem 1.1. For every ν > 0 there exist a force f ∈ C_c^∞(R³ × (0, ∞); R³), a compact set K ⊂ R³, and smooth velocity and pressure fields u, p on R³ × [0, 1) satisfying …” “This establishes alternative (C) in the Millennium problem statement for Navier–Stokes as stated by Fefferman in [13].” | — | 带光滑、紧支撑的**外力**（官方页也写 “The fluid has a smooth force applied to it”）。PDF 元数据创建时间 2026-09-08 15:06:26（时区乱码）。 | 已找到 |
| A4-2 Lemma 8.6 / (8.19) 与 “Four more derivatives” | L1 | PDF p.95–96 | p.95：“Lemma 8.6. Let N = v_t · ∂_y on T², with v_t as in (6.2), and let F be a smooth function of zero Haar mean. … satisfies the displayed norm bound for every integer m ≥ 0:” (8.19) ‖N⁻¹F‖_{C^m_y} ≤ C_m‖F‖_{C^{m+4}_y}。p.96 证明首段：“The divisor bound (6.7) gives |v_t · k|⁻¹ ≤ C(1 + |k|). Four more derivatives of F than the requested output leave a summable (1 + |k|)⁻³ bound on the Fourier series in two dimensions. This proves (8.19), smoothness, and uniqueness after fixing the zero Fourier coefficient. The multiplier commutes with every slow coefficient derivative and preserves reality.” | 07 | 质疑论文把这段当作 “source paper, page 96” 的引文，**与原件逐字一致**；“Equation (8.19) … p. 95”也对。 | 已找到 |
| A4-3 (10.19) 压力通量界 | L1 | PDF p.123（推导起于 p.122） | “Consequently |∫π w · ∇χ_R| ≤ C_T R⁻¹[(B_R + 1)B_R^{1/2} + R^{−3/4}B_R^{3/4}]. (10.19)”；p.122：“B_R = ‖φ_R⁴ w‖₆”，“We use the standard L^p boundedness of Riesz transforms for 1 < p < ∞; see [20].”（[20] = Stein 1970，与质疑论文的 [53] 同一本）；“The first sum has L^{3/2} norm at most C_T(B_R + 1)”；(10.19) 属于 Lemma 10.5 的证明；“for almost every t ∈ (0, T)” 在 p.123 (10.19) 上方。 | 08 | 与质疑论文 (3.4) 一致。 | 已找到 |
| A4-4 论文怎么描述 Lean | L1 | `a-oai-ns-paper.txt` 全文检索 | 全文 grep “lean”“formali[sz]”：**0 处命中**。 | — | 论文 PDF 本身没有提到 Lean 或形式化；相关表述在官方页、Lean 仓库 README、`formalization.yaml`（见 A5、A6）。 | 未找到一手来源（针对“论文里的 Lean 描述”这一问） |

### A5　OpenAI NS Lean 仓库 @ f9e8bc5

仓库 https://github.com/openai/NavierStokesAndEuler ；描述 “Lean certificates accompanying Navier-Stokes and Euler results”；创建 `2026-09-08T10:53:38Z`；最后推送 `2026-09-10T15:14:13Z`（API）；Apache-2.0。GitHub 已关闭 Issues（`has_issues: false`，`open_issues_count: 0`；issues API 返回空；pulls API 404），无 Discussions。存档：`a-nsrepo-info.json`、`a-nsrepo-commits.json`、`a-nsrepo-compare.summary.json`、`a-nsrepo-8937a8f-to-f9e8bc5-name-status.txt`、`a-nsrepo-README-*.md`、`a-nsrepo-formalization-f9e8bc5.yaml`、`a-lean-*.lean`（片段）。

| 说法 | 级 | 来源 | 原文摘句 / 核对 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| A5-1 README 原句 | L1 | README.md @f9e8bc5 | “This repository contains Lean 4 formalizations of the results presented in “[Finite time blowup for Navier–Stokes](…navier-stokes.pdf)” and “[Finite time blowup for the Euler equation](…euler.pdf)” by OpenAI.” “Whole space R³: There exist smooth initial data and forcing for which no global smooth solution with uniformly bounded kinetic energy exists.” | — | 质疑论文所引那句与原文一致（质疑论文省去了链接，中括号注为作者所加）。8937a8f 版 README 没有三条链接，f9e8bc5 加了。 | 已找到 |
| A5-2 `norm_derivativeWord_inverse_le` | L1 | `NavierStokes/SmoothFamilyTorusInverse.lean` 第 1059–1080 行；存档 `a-lean-norm_derivativeWord_inverse_le.lean` | 假设含 `‖SmoothFourierData.xJet (w.length + 5) (slice f p) (x, y)‖ ≤ C` 与 swap 版；结论 `‖derivativeWord w (slice (inverse d f) p) Y‖ ≤ ParametricTorusInverse.mixedLossConstant w.length * C`；证明里 `∑' k : Frequency, (weight k ^ 4)⁻¹`。 | 09（本地等宽渲染，见 capture-log） | **核验**：行号 1059–1080、声明名、代码与质疑论文所示一致。`xJet p f` 的定义是 `partialX^[p] f`（`SmoothFourierData.lean` 第 68 行）——即纯 x 方向 p 阶导；故该定理的假设是 |F|、∂ₓ^{n+5}F、∂ᵧ^{n+5}F 各自有界 C（n = `w.length`），**不是**完整的 C^{n+5} 范数；质疑论文先写成 ∂^{m+5}F/∂y_i^{m+5} ⩽ C（准确），再改写成 ‖F‖_{C^{m+5}}（是它的充分条件，推断）。同文件第 863–865 行 `inverse_finiteJets` 的 docstring：“Uniform full-tensor bound with five torus derivatives lost.”，条件 `JetBound f S (n + 5) C`；第 724–725 行：“The loss l+4 comes only from the order-l multiplier and the summable two-dimensional lattice majorant; it does not grow with jet order.”——OpenAI 自己的注释也表明 Lean 版损失 5 阶（l=1 的乘子，4+1）。 | 已找到 |
| A5-3 `exists_uniform_actual_pressure_flux_bound` | L1 | `NavierStokes/R3/PressureFlux.lean` 第 576–598 行；存档 `a-lean-exists_uniform_actual_pressure_flux_bound.lean` | 结论：`∃ CP : ℝ, 0 ≤ CP ∧ ∀ R : ℝ, 1 ≤ R → ∀ t ∈ Ioo 0 T, |∫ x, (p - q) (t, x) * fderiv ℝ (ComparisonCutoffs.weight R) x ((u - v) (t, x))| ≤ CP * ((cutoffL6 … ^ (1 / 2 : ℝ) + 1) * (dissipationRoot … / R + 1 / R ^ 2) + R ^ (-(7 / 4 : ℝ)) * cutoffL6 … ^ (3 / 4 : ℝ))` | — | **核验**：文件、行号 576、式 (3.5) 与质疑论文逐项一致（(3.5) 的 A_R = `dissipationRoot`，B_R = `cutoffL6`）。论文所点的引理名（`lpNorm_six_rieszTest_le`、`riesz_rTest_bound`、`smooth_eLpNorm_six_toReal_le`、`cutoff_interpolation_twelve_fifths`、`norm_weighted_tensorDiff_le`、`actual_flux_eq_canonicalCutoffFlux`、`multiplier_r_eq_weight_deriv` 等）都存在于 f9e8bc5。**未核验**：三条“原因”里 (2) Hölder 指数、(3) 时间连续性的逐行证明；以及脚注 7 “R3StressPressureEstimate.lean … does not appear to be used” 的说法。 | 部分支持（陈述一致；证明差异未逐行核） |
| A5-4 两个被引文件在两次提交之间是否改动 | L1 | `git diff --name-status 8937a8f f9e8bc5` | 两个文件都不在改动清单里。 | — | HN 9/11 帖 “OpenAI changed Navier-Stokes press release and Lean4 code on GH”（49661928）指的改动与这两个文件无关；两次提交共改 188 个文件（173 新增、15 修改，另有 `README.md`、`formalization.yaml` 在清单内）。f9e8bc5 文件数 2669，8937a8f 为 2496。 | 已找到 |
| A5-5 顶层定理的 Lean 陈述 | L1 | `ComparatorChallenges/NavierStokes.lean`（挑战陈述）与 `NavierStokes/ComparatorSolution.lean`；存档 `a-lean-comparator-challenge-theorems.lean`、`a-lean-ComparatorSolution.lean` | `theorem navier_stokes_breakdown_R3 (nu : ℝ) (hnu : nu > 0) : ∃ (u₀ : ℝ³ → ℝ³) (f : ℝ³ → ℝ → ℝ³), InitialVelocityConditionDecay u₀ ∧ ForceConditionDecay f ∧ ¬ (∃ v p, NavierStokesExistenceAndSmoothnessRn nu u₀ f v p)`；周期版同形。挑战文件头注：“Standalone comparator copied from: https://github.com/google-deepmind/formal-conjectures/blob/8bf45ed70d48b2b2a501de9c00b26bfa38c573ee/FormalConjectures/Millenium/NavierStokes.lean”。`ComparatorChallenges/README.md`：“Thank you to the Formal Conjectures authors for their Lean formalization of the Navier–Stokes problem statement …, which we adapted for these Comparator challenges.” | — | 顶层定理陈述取自 DeepMind Formal Conjectures（经 OpenAI 改写命名空间/导入），不是 OpenAI 模型自己翻译的；质疑论文谈的是中间引理，没有质疑这一陈述。注意：挑战文件里的两条定理带 `sorry`，那是 Comparator 的“挑战”约定，解答在 `ComparatorSolution.lean`。 | 已找到 |
| A5-6 仓库自己对形式化状态的说法 | L1 | `formalization.yaml` @f9e8bc5 | “scope: "Full formalization of main results."”；“sorry_count: 0”；axioms “propext”“Classical.choice”“Quot.sound”；`automation: method: "agent", models: "GPT-6 Astra", framework: "Codex"`；`review: status: "self-assessed"`。 | — | “自评估”，不是第三方评审。一致地，OpenAI 的官方页写 “Lean formalization and verification took an additional 17 hours via GPT‑6 Astra.” | 已找到 |
| A5-7 两次提交内容 | L1 | `a-nsrepo-commits.json`、`git log` | 8937a8f：作者 Boris Alexeev，作者/提交时间 `2026-09-08T10:57:25Z`（北京 18:57:25），message “.”；f9e8bc5：同作者，作者时间 `2026-09-10T11:51:24Z`（北京 19:51:24）、提交时间 `2026-09-10T13:40:53Z`（北京 21:40:53），message “.”。二者相差：新增 173 个文件，修改 15 个（含 `ComparatorR3Theorem.lean`、`ComparatorTheorem.lean`、`README.md`、`formalization.yaml`）。 | — | 派工单把 f9e8bc5 的时间写为 `13:40:53Z`：那是**提交时间**；作者时间是 11:51:24Z。 | 已找到 |
| A5-8 仓库有无 issue/讨论回应该论文 | — | GitHub API | Issues 关闭，无 Discussions。 | — | 未见仓库内回应。 | 未找到一手来源 |

### A6　OpenAI 官方 NS 页与回应

存档：`a-oai-ns-page.md`、`a-oai-ns-page.html`（firecrawl 取得；curl 与首次 headless Chrome 遇 403/挑战页，见日志）。

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| A6-1 发布时间、标题 | L1 | https://openai.com/index/navier-stokes-solution/ | 标题 “On the Navier–Stokes Millennium Prize Problem”；日期 “September 8, 2026”；og:description：“We’re sharing an AI-generated solution to the Navier–Stokes Millennium Prize Problem, including a writeup and a formal proof in Lean.” | 10 | 官方未给时刻。HN 提交 `2026-09-08T17:13Z`（来自派工单，我未重取）。 | 已找到 |
| A6-2 关于 Lean 的原句 | L1 | 同上 | “We’re sharing both a writeup of the proof and a formalization in Lean.” “Our system produced an analytical proof and a Lean formalization that an initially smooth fluid at rest can develop a singularity in a finite time. The fluid has a smooth force applied to it, and its energy remains finite through the entire dynamics, from rest to the formation of the singularity.” “The agents arrived at their resolution on Saturday, September 5, about 88 hours after the first agents were launched. Lean formalization and verification took an additional 17 hours via GPT‑6 Astra.” “After the completion of our full project and Lean verification (on September 6th) …” | 10（前两句） | 页面没有逐引理对照 NL 证明与 Lean 的说法。 | 已找到 |
| A6-3 更新/更正说明 | L1 | 同上，脚注 2 | “Update — September 10, 2026: We have updated “Concurrent work” with findings from our investigation into whether user inputs could have influenced this result.” | — | 唯一的更新说明，与 Lean 无关。页面未写 Lean 方面的更正。 | 已找到 |
| A6-4 OpenAI 或作者对 2610.08144 的公开回应 | — | 官方页、GitHub、HN/搜索 | 未找到。Dan Roberts（OpenAI）10/8 的 X 帖只谈 math 仓库：“We've updated our GitHub math repo with 6 new Lean formalizations, 19 modifications, and 3 withdrawals. The repo now has ~42% top-line results formalized. We will continue to update the repo with new formalizations and with any errata we notice.”（`article:published_time` `2026-10-08T05:20:44Z` = 北京 13:20:44；存档 `a-x-danintheory.html`） | — | 搜索：firecrawl 网页搜索两次（英中文），HN Algolia。**搜不到不等于没有**。该帖说“19 modifications”，history.md 说修订 14 篇 + 13 篇改引用，数字口径不同，两处都记。 | 未找到一手来源 |

### A7　反应

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件/时刻 | 状态 |
|---|---|---|---|---|---|---|
| A7-1 AHM 声明 | L1（AHM 自己的页面） | https://www.ahmath.org/statements ；存档 `a-ahm-statements.html/.txt` | 标题 “AHM Statement on OpenAI’s October 6 Release of Mathematical Documents”；“Releasing over 700 files at once is not a demonstration of scholarship, but a demonstration of power. We urge mathematicians to discontinue their work with OpenAI and to return to a vision of science that centers human understanding.” 署名 “Association for Human Mathematics Communications Working Group”。 | 11 | **页面没有日期**；正文写 “Yesterday, on October 6th, 2026”，故写于 10/7（时区未知）。HN 首次提交 `2026-10-07T21:40:41Z`（北京 10/8 05:40:41）。AHM members 页：“The Association for Human Mathematics has 809 members, 532 of whom belong to our AI-free caucus.”（抓取时刻北京 10/9 约 09:50） | 已找到 |
| A7-2 AGMAI 原文有无“不应在内部模型上测高级数学问题” | L1 | https://agmai.org/general-sep29/ （“September 29, 2026”）；存档 `a-agmai-sep29.html/.txt`、`a-agmai-home.*` | AGMAI 原文首段：“At present, some frontier AI labs are testing advanced mathematical problems on proprietary models that remain inaccessible to the broader scientific community. … However, ideally, they would not do so. We want to state clearly from the start: we do not endorse this practice, and we ask them to stop testing advanced mathematical problems on proprietary models.” AHM 的转述：“opened their initial advisory statement by saying that frontier AI corporations should not test advanced mathematical problems on internal models.” | 14 | 实质一致，措辞不同：AGMAI 写 “proprietary models”，AHM 转述为 “internal models”；AGMAI 同时写了这是“practical context”下的建议（“Our recommendations are formulated with this practical context in mind”）。AGMAI 10/6 声明（首页）：“AGMAI’s advisory role should not be interpreted as a judgment of the impact of these results or an endorsement of the process by which OpenAI obtained them.”；成员页：9 人（Charles、De Lellis、Gowers、Hairer、Srivastava、Tillmann、Vakil、Witten、Wood），**无陶哲轩**。“This group came together after OpenAI approached some of its members about establishing an external advisory board. In agreement with OpenAI, they decided to instead create an independent group and invite the others to join.” | 部分支持（AHM 转述与 AGMAI 原文意思相合，非逐字） |
| A7-3 陶哲轩 Mastodon 四连帖 | L1（本人账号） | https://mathstodon.xyz/@tao/117395267721642920 … /117395269325940185（API：`a-tao-mastodon-4of4.json`、`a-tao-mastodon-context.json`） | 1/4 “In traditional mathematics (or "Math 1.0"), a breakthrough proof of a long-standing open conjecture generates a large amount of subsequent activity and excitement in the field.” 4/4 “"Math 1.0" placed a premium on being the first to solve an open problem, even if the solution was not initially well understood. Now that this goal has been optimized to the point of unsustainability, "Math 2.0" will need to decenter the role of raw problem solving and value mathematical progress more holistically … I believe that AI can contribute positively in all of these directions as well; but it will require more imagination and ambition than the "Math 1.0" mindset of simply pointing one's favorite AI agent at some set of open problems and asking for a solution.” | — | 时刻：1/4 `2026-10-06T18:00:27Z`、2/4 `18:00:33Z`、3/4 `18:00:44Z`、4/4 `18:00:51Z`（北京 10/7 02:00:27–02:00:51）。**四帖没有点名 OpenAI 或 AHM，也没有号召抵制**，并写明“AI can contribute positively”。4/4 的互动数（抓取时）：回复 17、转发 45、点赞 148。 | 已找到 |
| A7-4 陶博客是否转载 AHM 声明 | L1（陶博客页面） | https://terrytao.wordpress.com/2026/10/07/ahm-statement-on-openais-october-6-release-of-mathematical-documents/ ；存档 `a-tao-blog-ahm.html/.txt` | 页首方括号注：“[This is a guest post by the Association for Human Mathematics, reposted from their statements page. This blog post was initially written in a different file format and converted using AI. — T.]”。页面分类 “guest blog, opinion”，作者栏 “by Terence Tao”（站点作者字段）。 | 13 | 页面标日期 “7 October, 2026”；`article:published_time` `2026-10-08T00:02:51Z`（北京 10/8 08:02:51）。**转载属实，但是 guest post，由 AHM 署名；不是陶写的，也不是陶“牵头”**。陶博客 10/8 起另有帖子，我未逐篇核对是否另表态（未核验）。另：陶博客 9/22 转载过 Martin Hairer 的 “Why I agreed to join AGMAI”（guest post，Hairer 为 AGMAI 成员）。 | 已找到（“转载”）；“陶哲轩主持/发布”：与说法不符 |
| A7-5 “752 名成员”出处 | — | AHM members 页（现行）；Wayback | 现行页面写 “809 members, 532 of whom belong to our AI-free caucus” 并写 “We update our public list approximately once per week.” | — | 没有找到 752 的出处；Wayback 查询失败（429 / 站点离线），无法查历史值。**可能是更早一次的页面数字**（推断，未核验）。 | 未找到一手来源 |
| A7-6 Karagila 博文 | L1（博主本人） | https://karagila.org/2026/openai-pp/ ；存档 `a-karagila.html/.txt`、`a-karagila-feed.xml` | “No, if this was an academic paper submitted to a journal, it should be issued a desk rejection for the quality.” “But it is the equivalent of a Denial of Service, should we want to take it seriously: stop everything else and figure this one out.” “So, no, I will not be sending Sam Altman a bottle of whisky anytime soon …” | 12 | 页面写 “Oct 08 2026, 10:25”；RSS `pubDate` “Thu, 08 Oct 2026 10:25:37 +0100” → 时区 UTC+1，= 09:25:37Z = **北京 10/8 17:25:37**。**他谈的是 Partition Principle 那篇**（“I took a brief look at the preprint released by OpenAI (not at the Lean code …)”），既不是 NS 也不是三篇撤稿；原文也写 “I am not entirely against the use of AI.” | 已找到 |
| A7-7 Aaronson “The Mathocalypse” | L1（博主本人） | https://scottaaronson.blog/?p=10169 ；存档 `a-aaronson.html/.txt` | “Or at least, we’re pretty sure that it’s a proof! There’s a Lean certificate, as there are for some of the other 372 breakthrough results (not all of them). But it also appears that no human has understood just about any of these proofs yet; the race to do so has just started.”（该句针对 Unique Games 一篇）；同文：“among the 372 huge results released yesterday by OpenAI, on the recommendation of its advisory group of Timothy Gowers, Edward Witten, and other distinguished mathematicians” | — | `article:published_time` `2026-10-07T18:57:26Z`（页面 “October 7th, 2026 at 1:57 pm”，北京 10/8 02:57:26）。**发表于撤稿之前**（撤稿合并 10/8 05:20Z）。他说发布“on the recommendation of” AGMAI，而 AGMAI 自己写 “should not be interpreted as … an endorsement of the process”（两处并存，记录）。 | 已找到 |

### A8　热度（复取）

HN 通过 Algolia：items API 于 2026-10-09T01:52:15Z（北京 09:52:15）取，计数为评论树内条数；search API 于 01:47:59Z 取，`num_comments` 与 items 树可能差几条（已删/死评论）。存档 `a-heat.json`、`a-hn-<id>.json`（含评论树）、`a-heat.py`。

| 帖 | id | 分 | 评论（search / items 树） | 创建（UTC → 北京） |
|---|---|---|---|---|
| OpenAI Withdraws 3 Math Papers（url=history.md） | 50003107 | 338 | 3 / 3 | 10/8 08:08:37Z → 16:08:37 |
| OpenAI withdraws three mathematical results（X 帖，Dan Roberts） | 50002650 | 248 | 545 / 508 | 10/8 07:05:40Z → 15:05:40 |
| OpenAI withdraws three of their recent manuscripts（PR files） | 50003100 | 4 | 1 / 1 | 10/8 08:07:01Z → 16:07:01 |
| Navier–Stokes Lost in Translation（arXiv） | 49994145 | 337 | 210 / 203 | 10/7 15:24:38Z → 23:24:38 |
| “Math 2.0” will need to value mathematical progress more holistically（陶） | 50002008 | 590 | 627 / 628 | 10/8 05:14:14Z → 13:14:14 |
| The Mathocalypse | 49997718 | 379 | 390 / 377 | 10/7 19:33:40Z → 10/8 03:33:40 |
| OpenAI, the Partition Principle, and Mathematics（Karagila，后一次提交） | 50013902 | 85 | 99 / 99 | 10/8 23:29:43Z → 10/9 07:29:43 |
| 同上（前一次提交） | 50004120 | 2 | 0 / 0 | 10/8 10:34:52Z → 18:34:52 |
| AHM Statement …（后一次提交） | 50003677 | 46 | 43 / 43 | 10/8 09:29:48Z → 17:29:48 |
| Association for Human Mathematics's Statement …（前一次提交） | 49999159 | 13 | 1 / 1 | 10/7 21:40:41Z → 10/8 05:40:41 |
| Responsible Release of AI-Generated Mathematics（AGMAI 9/29） | 49903713 | 123 | 224 / 215 | 9/30 02:36:12Z → 10:36:12 |
| OpenAI’s Navier-Stokes release included a Lean 4 formal proof（J. D. Cook） | 49650326 | 180 | 179 / 177 | 9/10 21:22:59Z → 9/11 05:22:59 |
| OpenAI changed Navier-Stokes press release and Lean4 code on GH | 49661928 | 9 | 2 / 2 | 9/11 17:19:29Z → 9/12 01:19:29 |

Reddit：**无法取得分数与评论**（www.reddit.com 与 old.reddit.com：curl 403、firecrawl “不支持该站点”、内置浏览器拒绝访问）。仅从 firecrawl 搜索摘要得到链接与标题（作线索）：r/math “Navier-Stokes lost in translation (Why Lean [..] does not guarantee correct natural language proofs)”（https://www.reddit.com/r/math/comments/1x02ein/…、https://www.reddit.com/r/mathematics/comments/1x0anob/…）；r/mathematics “Errors and withdrawals of 3 OAI results on algebraic geometry”（https://www.reddit.com/r/mathematics/comments/1x0n92w/errors_and_withdrawals_of_3_oai_results_on/）；r/OpenAI “Fields Medalist Terence Tao reposts statement from …”（https://www.reddit.com/r/OpenAI/comments/1x0gqmb/…，r/ChatGPT 同题 1x0gr48）；r/aiwars 1x12u55；r/NoStupidQuestions 1x0ix8h。摘要里出现的一句 “(it has been withdrawn)” 是否指 NS 论文，**未核验**（NS 论文没有被撤回）。

中文媒体（均为二手，L5；用于“标题与时刻”）：
- 机器之心（经新浪）《刚刚，OpenAI撤回了三篇AI数学论文，竟是因为符号写错了》，北京 2026-10-08 18:03:56；https://k.sina.com.cn/article_5953190046_162d6789e06703tv3g.html ；存档 `a-sina-withdraw.html/.txt`。写了“撤回3篇、修订14篇、13篇更新引用”“撤回的是证明，并不意味着这些数学命题本身是错的”。其中“按单篇主结果统计 300 篇”是其解读。
- CNMO（经搜狐）《OpenAI撤回3项数学研究成果！数学仓库首次修正》，北京 2026-10-08 17:44；`a-sohu-cnmo.txt`。
- 投资界（news.pedaily.cn）转载同稿（`a-pedaily.html`）；winzheng 《OpenAI又惹毛数学家》页（`a-winzheng.html`，内容未逐条核）。
- 未找到专门报道 2610.08144 的中文媒体（两次网页搜索：英文 + 中文关键词；**搜不到不等于没有**）。

### A9　夸大实例

| 夸大类型 | 原站 URL | 原句（逐字） | 时间 | 发布方 | 有无写“3 篇撤回”“作者不评判自然语言证明”“Lean 编译通过” |
|---|---|---|---|---|---|
| “陶哲轩反 AI / 抵制 OpenAI / 牵头” | https://www.163.com/dy/article/L8PLN2CK0511DSSR.html （网易号转量子位稿；存档 `a-163-tao-boycott.html/.txt`） | 标题《陶哲轩带头宣战！人类数学家联合抵制OpenAI》。“数学界领军人物、菲尔兹奖得主陶哲轩，牵头人类数学协会（AHM）发布重磅联合声明——全球数学界集体行动，全面抵制OpenAI。”“这次由陶哲轩领衔，带头向商业巨头开火的机构，叫做人类数学协会” | 网易页 `ptime` 北京 2026-10-09 08:34:38（原站量子位时刻未取到） | 量子位 / 网易号；署名 “Jay发自 凹非寺 量子位” | 正文引用了 AHM 声明的节选，引用的参考链接是陶博客转载页，但写成陶牵头；**未写**“三篇撤回”；未写 NS 的 Lean 质疑。同稿还写“用内部模型一口气刷了8000道题”“命中率在5%左右”（OpenAI README 写 “approximately 4,000 problems”）、“OpenAI为了展现合规姿态，曾拉着普林斯顿高等研究院设立了…（AGMAI）”（与 AGMAI 自述不符，见 A7-2）。 |
| “陶哲轩说这不是好事” | YouTube 视频标题（仅标题，未取视频） | “OpenAI 一夜攻克722个数学难题？陶哲轩却说：这不是好事！”（`https://www.youtube.com/watch?v=OCln8nZ3uDA`，firecrawl 搜索摘要） | 未取 | 未核 | 陶的四连帖没有点名 OpenAI（A7-3）；其余**未核验**。 |
| “NS 的 Lean 证明是假的 / 证明被推翻” | 未找到文章；HN 评论中有误读（见 A10：ComplexSystems、balaclava9） | 49995534：“So these authors seem to be claiming that OpenAI has not really proven Navier-Stokes at all.”；50011433：“The statement written in Lean, is not actually a correct description of the Navier-Stokes problem. It's some other easier statement.” | 49995534：`2026-10-07T16:59:04Z`；50011433：`2026-10-08T20:10:32Z` | HN 读者 | 论文本身写了免责与 “the theorem is correct”（A3-2、A3-3）；balaclava9 的说法与 Lean 顶层陈述取自 Formal Conjectures（A5-5）不符。这是读者误读，不是媒体标题，仅作线索。 |
| “722 篇全部翻车 / 造假” | **未找到**（搜了英中文标题；只找到“首份勘误”类标题） | — | — | — | — |

### A10　HN 上有理有据的质疑与反驳（线索 L6，进正文前须回到一手来源）

HN 不公开评论分数，Algolia 也不提供（以下“分数”均不可得，仅给评论链接 `https://news.ycombinator.com/item?id=<id>`，按其子树规模排序）。

关于质疑论文（帖 49994145）：
- 49995023 buzzy_hacker、49995650 nicf、49995191 jrflo：都指出论文只在质疑“NL 证明与 Lean 证明是否对应”，没有质疑 Lean 证明本身有效。与 A3-2、A3-3 相合。
- 49996834 infogulch、49997951 latent-person、50003650 latent-person、49995587 fasterik：关键在于 Lean 顶层定理陈述是否等价于 Clay 官方陈述；陈述来自 DeepMind Formal Conjectures，不是 LLM 翻译；还提到独立内核 nanoda 复核。我核对了第一部分（A5-5：挑战文件头注）；“nanoda 复核”见 `formalization.yaml` 与 `ComparatorChallenges/NavierStokes.json`（`"enable_nanoda": true`），**我没有运行**，只读到配置。
- 49995871 dooglius、49997221 latent-person：m+4 与 m+5 更像是形式化过程中对证明的调整，而不是“自然语言歧义”；缺一次“从 Lean 反推回 NL”的对齐。这与 A5-2 的 docstring（Lean 版有意写成 l+4 乘子损失）相容，但“有意”是推断。
- 50002042 aidenn0：“This happens all the time with human researchers.”（人类论文里 NL 引理为假而弱化版成立的情形）。
- 49995010 empath75：自己用 Claude 形式化一篇 CS 论文，发现原文有多处错误，得到的形式化与原文的演算“不完全是同一个”。
- 49995106 j2kun：AI 生成证明的报道应称为 “claims”，AI 公司不应在同行评审之外。
- 误读：50011433、50011523 balaclava9（把“Lean 引理比 NL 引理弱”理解为“Lean 陈述的是更简单的问题”）。
- SCI = ∞ 论证是否切题：**HN 里没有找到针对 SCI/Halting 论证的高质量评论**（我用关键词 SCI/Halting/Hilbert/Rice 扫了全树）。我自己的读法（推断）：A.2.1 论证的是“一个总要输出（忠实译文或拒绝）的可信形式化器”的不可能性，与 Example 3.1/3.3 的具体错译在逻辑上独立。

关于撤稿（帖 50002650）：
- 50003453 AlanYx：只有一部分稿子有 Lean；即使有，形式化也可能“语义上偏离”。
- 50003464 autuni：OpenAI 先发后补 Lean（引原博客：“We will update the repository with more formalizations as we obtain them.”）；devy 在其下猜测错误是在写 Lean 阶段发现的——**无依据**（history.md、README 都没写谁发现的、怎么发现的）。
- 50007128 chubot：撤回的三篇是不是没有 Lean 的？我核对：是（A2-3，三篇所在第 032 族无 Lean 链接）。
- 50005841 ijustlovemath：Lean 能编译但陈述的不是想要的东西。
- 50004733 hmate9：“3 mistakes (so far) out of ~400 is still a pretty good hit rate”（分母没有来源；README 的分母是 722 篇 / 372 族）。

## 2. 可能的吠点（原文里有、线索摘要没提或说反的条件与限制）

1. 撤稿只涉及三篇，且均无 Lean；history.md 与 README 都写“撤回的是证明，不是说命题为假”（A1-3）。**不是 722 篇翻车。**
2. history.md 与 README 对撤稿日期写法不同（10/7 与 10/6），最新提交是合并提交 fd4aeeb，不是 3014888（A1）。
3. “300 / 719”口径无法由仓库复核（A2-4）；`formalization.yaml` 的审查状态是 `unchecked`（OpenAI math 仓库）和 `self-assessed`（NS 仓库）。
4. 质疑论文的 Figure 3、Figure 4 是 ChatGPT 对话截图；作者的“更多错译清单”是 ChatGPT + Claude 生成，自注“未全人工核”；Figure 3 的行号与 f9e8bc5 不符（A3-5、A3-12）。
5. 质疑论文的“Lean 比 NL 弱”是对**中间引理**的比较；顶层定理陈述来自 Formal Conjectures，论文不碰（A5-5）。作者自己写“the theorem is correct”（A3-3）。
6. Example 3.1 里 Lean 的假设是纯 x/y 方向 (n+5) 阶导加 |F| ≤ C，不是 C^{n+5} 全范数；论文脚注 5 承认有些 Lean 结果与 (3.1) “incomparable”（A3-6、A5-2）。
7. OpenAI 论文 PDF 本身不提 Lean（A4-4）；Lean 的说法来自官方页（“writeup … and a formalization in Lean”）与 README。官方页写 Lean “verification” 用了 17 小时 GPT‑6 Astra。
8. OpenAI NS 论文证的是**带光滑外力**的爆破（官方页与 Theorem 1.1 都写明），不是无外力初值问题（A4-1）。
9. AHM 声明与 AGMAI 的立场不同：AGMAI 10/6 写“不应被解读为对结果影响的判断或对过程的背书”，并写与 OpenAI 的讨论“constructive”；AHM 是另一个组织（成员页：“Members commit to not working with commercial AI companies …”）。陶博客只是转载 AHM（A7）。
10. Karagila 的文章讲的是 Partition Principle 那篇，且他“没有看 Lean 代码”（A7-6）。

## 3. 夸大说法实例

见 A9 表。找到的唯一中文成稿级夸大是网易号转量子位《陶哲轩带头宣战！人类数学家联合抵制OpenAI》。“722 篇全部造假”“NS 证明被推翻”的标题我**没有找到**。

## 4. 扫描说法勘误

| 扫描/已知说法 | 核对结果 |
|---|---|
| “最新提交 `3014888`” | main 的最新提交是 `fd4aeeb`（合并 PR #1，北京 10/8 13:20:00）；3014888 是 PR 分支上的提交（北京 10/8 13:03:50）。 |
| “GitHub 显示 Oct 8, 2026” | 提交作者时间 UTC 10/8 05:03:50（美西 10/7 22:03）。history.md 标题写 “October 7, 2026”，三篇 README 写 “Withdrawn on October 6, 2026”。三个日期都存在。 |
| “三篇撤稿不在形式化清单” | 属实，方法已写（A2-3）。 |
| “300 / 719 = ~42%”与 yaml 条目数 | 复算不出（A2-4）。1006 yaml 旧版 162 篇 `sources`，新版 173。 |
| “f9e8bc5 = 2026-09-10T13:40:53Z” | 那是**提交时间**；作者时间 11:51:24Z。 |
| “Lean 仓库只有两次提交，论文引的就是当前最新版” | 属实（`git log`）；`pushed_at` 为 2026-09-10T15:14:13Z。 |
| “WorkBuddy：陶哲轩转载全文/主持发布” | “转载”属实（guest post，页首有 “— T.” 注）；“主持发布/牵头”无依据（A7-4）。 |
| “752 名成员” | 未找到出处；AHM 现写 809（532 AI-free）。 |
| “AGMAI 初始声明说前沿 AI 公司不应在内部模型上测高级数学问题”（经 AHM 转述） | AGMAI 9/29 原文是 “we ask them to stop testing advanced mathematical problems on proprietary models”，且前有 “ideally, they would not do so”。 |
| “Example 3.1：Lean 用 m+5” | 属实（第 1059–1080 行，xJet (w.length+5)）；细节：是纯导数假设（A5-2）。 |
| “Example 3.3：Lean 版多一个 A_R 项、证法不同” | 陈述式 (3.5) 与 Lean 第 576–598 行一致；证法差异的三条理由中，涉及的引理名都存在，逐行证明未核（A5-3）。 |
| “SCI = ∞ 是作者的定理” | 不是某个定理编号：A.3（Σ⁰₂-complete，k ⩾ 9）、A.4（Σ⁰_l-complete）、Cor A.5（SCI = l）；“SCI = ∞”是 §A.2.1 的论述，且限定于“总会输出的可信自动形式化器”（A3-9）。 |
| “质疑论文 v1 25 页、math.AP / cs.AI / math.LO” | 25 页、v1 2026-10-06 10:58:01 UTC 属实；arXiv 备注页给的主类别为 math.AP；cs.AI / math.LO 未在 abs 页逐项核对（未核验）。 |
| “HN 9/11 帖称 OpenAI changed Navier-Stokes press release and Lean4 code” | 帖存在（9 分 2 评论）；官方页的唯一更新说明是 9/10 关于 “Concurrent work”（与 Lean 无关）；Lean 仓库第二次提交确实比第一次多 173 个文件、改 15 个，但被质疑论文所引两个文件未变。 |
| Karagila “时区未知” | RSS：+0100，北京 10/8 17:25:37；他谈的是 Partition Principle 论文。 |
| Aaronson “no human has understood” | 原句限定在“just about any of these proofs”，并写 “Lean certificate … (not all of them)”；发表于撤稿之前。 |

## 5. 状态非“已找到”的条目清单

- A1-2 “最新提交 3014888”：**与说法不符**（实为合并提交 fd4aeeb）。
- A2-4 “300 / 719”：**部分支持**（719 可核；300 不可复核）。
- A3-5 Figure 3：已找到，但**对话截图的行号与 f9e8bc5 不符**（标注在条件栏，不改状态）。
- A4-4 OpenAI 论文里对 Lean 的描述：**未找到一手来源**（论文全文无 Lean 字样）。
- A5-3 压力通量陈述一致、证法差异未逐行核：**部分支持**。
- A5-8 仓库内回应：**未找到一手来源**（issues 关闭）。
- A6-4 OpenAI/作者对 2610.08144 的回应：**未找到一手来源**。
- A7-2 AGMAI 与 AHM 转述：**部分支持**（实质相合，措辞不同）。
- A7-4 “陶哲轩主持/发布”：**与说法不符**（转载属实）。
- A7-5 “752 名成员”：**未找到一手来源**。
- A8 Reddit 分数/评论：**未取得**（站点拒绝）。
- A8 中文媒体对 NS Lean 质疑的报道：**未找到**。
- A9 “722 篇全部造假”“NS 证明被推翻”的媒体标题：**未找到**。
- A10 HN 评论分数：**不可得**（HN 不公开）。

## 6. 我做的假设

1. “1006 存档”=`openai/math` 初始提交 `adc7f12`（三个文件字节相同已核）。
2. 北京时间 = UTC+8；来源只给美西/美东时间的，按 PDT=UTC-7、EDT=UTC-4 换算（commit 的 `-07:00`/`-04:00` 偏移来自 git 自身）。
3. 把“撤稿三篇所属成果族无 Lean 链接”当作“CONTENTS 层面也没有 Lean”的补充证据，不代替 yaml 比对。
