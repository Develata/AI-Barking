# Kernel-Grounded Minimal Backbone Specification of the Lean 4.33.1 RH / GRH Formalization

> **Target Formalization**: `leanprover-community/riemann-hypothesis` (Lean `v4.33.1`, Mathlib `0df444a360eaa60ab8c11dca51a86af692955474`, 0 `sorry`, 0 user `axiom`)  
> **Kernel Export Ground Truth**: Andrew Sutherland & James Shore `v0.2.1` Lean kernel export (`cone-constants.txt.gz`, `decls.csv.gz`, `dominators.tsv.gz`, `why-parents.csv.gz`, `directories.csv.gz`; standard axioms `{propext, Classical.choice, Quot.sound}` only)  
> **Machine-Readable Summary**: [data/kernel_backbone_summary.json](data/kernel_backbone_summary.json) · **Citation Auditor**: [scripts/audit_spec_citations.py](scripts/audit_spec_citations.py) · **NDJSON Statement Auditor**: [scripts/audit_ndjson_statement.py](scripts/audit_ndjson_statement.py)  
> **Companion Reports**: [rosetta_stone.md](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/rosetta_stone.md) · [rh_proof_explained.md](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/rh_proof_explained.md) · [meta_audit_verdict.md](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/meta_audit_verdict.md)

## Epistemic Labeling Convention

Every substantive claim in this specification carries one of three explicit epistemic tags, following the kernel-auditing convention established in the Lean community discussion of this proof:

- **`[Exact]`** — Mechanically computed from the `v0.2.1` Lean kernel export (`cone-constants.txt.gz`, `decls.csv.gz`, `dominators.tsv.gz`, `why-parents.csv.gz`, `directories.csv.gz`) or directly measured by the Lean `v4.33.1` compiler (`Kernel.isDefEq`, `Meta.isDefEq`, `collectAxioms`, `-DwarningAsError=true`).
- **`[Verified]`** — Read directly from the pinned Lean `v4.33.1` `.lean` source files and cross-checked to satisfy `in_cone = true` in `decls.csv.gz` (i.e., actually reachable inside the kernel proof terms of `Main.lean`'s five headline theorems).
- **`[Reading]`** — Informal mathematical synthesis or structural interpretation connecting the verified Lean signatures to classical number theory and automorphic representation theory.

> **Why this specification exists (`[Reading]`)**: A static reading of the 17.37-million-line source tree easily mistakes abandoned drafts, unreferenced `def : Prop` wrappers, and parallel corollaries for active proof steps. Every active declaration cited outside the fenced Dead-Code Self-Audit in Section 7 is verified `in_cone = true` by [`scripts/audit_spec_citations.py`](scripts/audit_spec_citations.py) with **zero dead citations**.

---

## 1. Global Kernel Accounting & Bloat Waterfall `[Exact]`

Joining the raw repository file inventory against the `v0.2.1` Lean kernel export decomposes the 17.37 million source lines into exact active vs. non-active buckets (`directories.csv.gz`, `decls.csv.gz`):

| Layer / Bucket | Files | Declarations | Lines | Share of Raw Lines | Notes |
| :--- | ---: | ---: | ---: | ---: | :--- |
| **Total upstream source tree** | 175,187 | 1,063,284 | 17,374,215 | 100.0% | `175,187 / 175,187` in import closure |
| Non-declaration boilerplate (imports, `open`, `namespace`, blanks, comments) | — | — | 3,941,735 | 22.7% | `raw_lines - total_decl_lines` |
| **All parsed declarations (`decls.csv.gz`)** | 175,003 | 1,063,284 | 13,432,480 | 77.3% | 184 files contain 0 declarations (import aggregators & `BuildAid`) |
| Wholly dead files (`used_decls = 0`) | 28,900 | — | 1,754,237 | 10.1% | 28,716 files with decls + 184 decl-free files outside cone |
| Unused declarations inside mixed active files | — | — | 940,515 | 5.4% | Total unused decls: `258,372` (`24.3%` of all decls; `2,694,752` decl lines) |
| **Active Lean kernel cone (`in_cone = true`)** | **146,287** | **804,912** | **10,737,728** | **61.8%** | `1,484,729` kernel constants (`cone-constants.txt.gz`) |
| ↳ Single-declaration active files (`n_decls = 1, used = 1`) | 63,899 | 63,899 | — | — | Of these, `48,922` files are `≤ 15` decl lines (`213,750` decl lines; thin re-export / wrapper fragmentation) |
| ↳ Shared across **all 5** `Main.lean` theorems (`thm_mask = 31`) | — | **804,887** | **10,737,528** | **99.997% of cone** | Routed through one root theorem: `allChi` |
| ↳ Theorem-specific delta across the 5 `Main.lean` theorems | — | **25** | **200** | **0.002% of cone** | `riemannHypothesis` (1L), `rhOfAll` (63L), `LFunction_eq_zero_re` (101L), `LFunction_eq_zero_re_of_isPrimitive` (28L), wrappers (7L) |

### Top-Level Directory Breakdown (`directories.csv.gz`) `[Exact]`

| Directory | Total Files | Active Files | Raw Lines | Kernel-Used Decl Lines | Active Decls / Total Decls | Decl Usage |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| `src/Development` | 141,928 | 119,001 | 12,232,663 | 7,659,708 | 544,156 / 692,952 | 78.53% |
| `src/Formalism` | 18,254 | 14,898 | 3,200,748 | 1,900,674 | 167,705 / 240,149 | 69.83% |
| `src/GroundTheory` | 8,507 | 6,558 | 1,119,138 | 600,712 | 48,530 / 75,101 | 64.62% |
| `src/ReciprocityTheory` | 3,845 | 3,450 | 563,016 | 394,315 | 30,735 / 36,425 | 84.38% |
| `src/CompanionTheory` | 2,027 | 1,881 | 194,218 | 143,098 | 10,561 / 11,768 | 89.74% |
| `src/PositivityTheory` | 449 | 341 | 55,130 | 32,327 | 2,981 / 4,744 | 62.84% |
| `src/Lemmata` | 167 | 155 | 9,232 | 6,871 | 231 / 248 | 93.15% |
| `src/Supplements` + `Main.lean` + `Statements.lean` | 5 | 3 | 50 | 23 | 13 / 13 | 100.00% |
| `src/BuildAid` | 5 | 0 | 20 | 0 | 0 / 0 | 0.00% |

### Measured Structural Bloat Families Inside the Active Cone `[Exact]`

Even inside the `10.74M`-line active kernel cone, three major machine-generation artifacts account for substantial line inflation:

1. **Extreme single-declaration file fragmentation**: `63,899` active files (`43.7%` of all active files) contain exactly one declaration, and `48,922` of those are `≤ 15` declaration lines (`213,750` declaration lines total, inflated by repeated `import` / `open` / `namespace` headers).
2. **`pp.explicit` typeclass-elaboration dumps in `CapelliIdentities`**: `793 / 880` files (`2,884 / 3,213` declarations, `106,255 / 111,362` declaration lines) are active in the kernel cone. Directly rewriting five 100%-kernel-used files under Lean `v4.33.1` (`-DwarningAsError=true`, `Kernel.isDefEq = true`, `Meta.isDefEq = true`, unchanged imports and axioms) eliminates **12,486 lines (-95.3%)**:
   - `FixPiIff.lean` (`UpperTheory.Reformulated.B.Gr34.mem_UC`, depth 20): `8,029 → 45` lines (`-99.4%`, [PR #60](https://github.com/leanprover-community/riemann-hypothesis/pull/60), **merged** at `173225e84`).
   - `FreeModuleQuotient.lean` (`UpperTheory.Reformulated.B.FreeModuleQuotient.freeN_quot`, depth 19): `2,516 → 84` lines (`-96.7%`, `68 → 0` warnings, [PR #65](https://github.com/leanprover-community/riemann-hypothesis/pull/65), **merged** at `f1f6d6542`).
   - `ContDiffHerm.lean` (`UpperTheory.Reformulated.B.Qj20.aa_closed` + 18 defs/lemmas, depth 25): `1,727 → 211` lines (`-87.8%`, `20 → 0` warnings, [PR #65](https://github.com/leanprover-community/riemann-hypothesis/pull/65), **merged** at `f1f6d6542`).
   - `SquareOfApproximation.lean` (`UpperTheory.Reformulated.B.SquareOfApproximation.main` + 4 defs/lemmas, depth 14): `658 → 235` lines (`-64.3%`, `17 → 0` warnings, [PR #65](https://github.com/leanprover-community/riemann-hypothesis/pull/65), **merged** at `f1f6d6542`).
   - `LimitOfFinite.lean` (`UpperTheory.Reformulated.B.LimitOfFinite.exists_limit_of_finite`, depth 21): `172 → 61` lines (`-64.5%`, [PR #65](https://github.com/leanprover-community/riemann-hypothesis/pull/65), **merged** at `f1f6d6542`).
3. **Near-duplicate 3-copy directory clones in `UnitaryGroupMassFormula`**: The three parallel directories previously profiled statically (`448` files total) have sharply asymmetric kernel usage:
   - `StabilizerVolume`: `145 / 155` files active (`15,186 / 15,895` decl lines = **95.5%** active — primary copy).
   - `StabilizerOrbitVolume`: `53 / 147` files active (`5,407 / 15,600` decl lines = **34.7%** active).
   - `WeilStabilizerVolume`: `4 / 146` files active (`135 / 14,668` decl lines = **0.9%** active — **99.1% dead**).
4. **Legacy `backward.*` transparency overrides inside the active cone**: Four 100%-kernel-used (`89 / 89` declarations `in_cone = true`) algebraic-geometry files accounted for **26 of 33** repository-wide `backward.defeqAttrib.useBackward` hits (`78.8%`) and **40** `backward.isDefEq.respectTransparency` hits (`66` directives total). Removing all 66 directives cleanly (`-DwarningAsError=true` with `0` warnings, exact `.olean` `Kernel.isDefEq` / `Meta.isDefEq` equivalence, `{propext, Classical.choice, Quot.sound}` only, and `5/5` direct `dist=1` importers verified) was submitted in [PR #71](https://github.com/leanprover-community/riemann-hypothesis/pull/71) (`ea66ac29e`). PR #71 itself was closed without merge; its patch was adapted into the merged batch [PR #115](https://github.com/leanprover-community/riemann-hypothesis/pull/115) as commit `da6765973f` (`CofilteredSchemeLimitExistsLocally.lean` `40 → 0`, `CoequifiberedDiagramMap.lean` `10 → 0`, `RelObjIsoHomSubtype.lean` `13 → 0`, and `ExistsEtaleNeighbourhoodFinitePieceIsolatedPoint.lean` `3 → 0`), and this tracker measured repository-wide `backward.*` options at `3,000` on `main@14f383e5e` and `2,934` on `main@291b7446e` (Lean `v4.36.0` tree, since replaced on `main` on 2026-11-25).

---

## 2. The Single-Chokepoint Kernel Dominator Spine `[Exact]` + `[Verified]`

In `dominators.tsv.gz`, the entire 10.74M-line proof collapses onto a **single depth-1 chokepoint** `allChi : UpperTheory.EveryCharacter` (`src/Supplements/Kernels/MainTheorems.lean:7`), which proves nonvanishing of every Dirichlet $L$-function on $\operatorname{Re}(s) > 1/2$ (`∀ N [NeZero N] (χ : DirichletCharacter ℂ N) s, 1/2 < s.re → (χ ≠ 1 ∨ s ≠ 1) → LFunction χ s ≠ 0`). All five theorems in `src/Main.lean` (`riemannHypothesis`, `riemannZeta_eq_zero_re`, `LFunction_ne_zero_of_half_lt_re`, `LFunction_eq_zero_re`, `LFunction_eq_zero_re_of_isPrimitive`) follow from `allChi` in 25 declarations / 200 lines total via functional-equation reflection (`Formalism.VW.rh_of_every`, `Formalism.VW.critLine`, `Claim.strip`) `[Exact]` `[Verified]`.

Under `allChi`, the proof splits cleanly at `UpperTheory.FinalTheorems.everyCharacter` (`MainConclusion.lean:10`) into **two top-level branches**—the Galois local-global engine (`689K` exclusive lines) and the row-conditional RH engine `everyCharacter_of_row` (`8.31M` exclusive lines), which itself forks into **three major pillars** (plus `1.74M` lines of shared infrastructure dominated by `everyCharacter` because multiple branches invoke it):

```text
(root: 5 Main.lean theorems)
 └─ [d=1] allChi (MainTheorems.lean:7)                                   10,737,526 L (100.0% of shared cone)
     └─ [d=2] UpperTheory.FinalTheorems.everyCharacter (MainConclusion.lean:10)  10,737,525 L
         ├─ [d=3] UpperTheory.FinalTheorems.everyCharacter_of_row (MainConclusion.lean:7)  8,308,005 L (77.4%)
         │   ├─ [d=4] UpperTheory.LiteratureTheorems.every_character_of_literature_of_vWin  7,172,414 L (66.8%)
         │   │   ├─ [d=5] UpperTheory.Lit.every_character_of_theta_integrals_local_matching  4,245,419 L (39.5%)
         │   │   │   ├─ [d=6] UpperTheory.Lit.nonempty_hyps_definite_side_of_closed_bound    3,576,707 L (33.3%)
         │   │   │   │   ├─ [d=7] GlobalTheory.Fc.S0.StageU.hypotheses_nonempty_of_unramified_exponents_alt  1,217,085 L
         │   │   │   │   │   ├─ [d=8] GlobalTheory.Fc.AllAtMemberStage0.allRows_of_open0_alt                   453,684 L
         │   │   │   │   │   │   └─ [d=9] GlobalTheory.LocalRows.lg1_sLp_sO_hTF → CarrierGap.offDiag           192,549 L
         │   │   │   │   │   ├─ [d=9] GlobalTheory.Fc.StageOneU.member_bundles_large_unramified_exponents_alt  285,027 L
         │   │   │   │   │   └─ [d=8] UpperTheory.GalC.final_alt_alt (LinkN upgrade)                           224,129 L
         │   │   │   │   ├─ [d=7] GlobalTheory.Fc.S0.ontoX_and_lamLow_of_unitaryGroupHypotheses'               303,151 L
         │   │   │   │   └─ [d=7] GlobalTheory.T.open0_of_unitary_group_hypotheses (→ miStarRow_alt_alt)       173,136 L
         │   │   │   └─ [d=6] UpperTheory.Top.every_character_of_green_v_win (TopPlug.lean:16)    194,989 L (1.8%)
         │   │   └─ [d=5] SelfIntersectionVolumeEstimates.topCovolSlim2_use_alt_alt               824,935 L (7.7%)
         │   │       ├─ [d=6] PositivityTheory.HtPt.Ah.deep_slim_use → good_tower_volume_theorem  488,348 L
         │   │       └─ [d=6] SelfIntersectionVolumeEstimates.topCovolG_alt_alt                   198,583 L
         │   └─ [d=4] UpperTheory.One.vwin_of_row (VwinOfRow.lean:4)                               914,524 L (8.5%)
         │       └─ [d=6] v_window_of_automorphy_csd_ss_mono_odd → vwin_of_hypotheses_H_E_alt     814,900 L
         │           ├─ katoTwoW_of_printsH (144,491 L) · sp4In0_alt (112,445 L) · unitary_three_automorphy_dense (63,996 L)
         │           └─ pFromAut (47,885 L) · lemmaK_litN_alt (38,854 L) · slopeAll (33,479 L) · HV.Core.of_four_alt (21,124 L)
         └─ [d=3] UpperTheory.Gal.Int.galoisLocalGlobalStrong_recParameter (RowSrc.lean:6)        689,213 L (6.4%)
             ├─ [d=4] galoisRepresentations_of_thetaLift → Geo.geoOne_holds → geometric_realization_of_weight_one (648,197 L)
             └─ [d=4] UpperTheory.TP.K.tLiftK (definite unitary theta lift, 40,938 L)
```

*(Note on line accounting `[Exact]`: each node's `sub_lines` in `dominators.tsv.gz` counts lines strictly dominated by that node—declarations reachable ONLY through it. Shared libraries invoked by multiple sibling branches, such as `GlobalTheory.T.zarhinGeometric_of_semistableReduction` (28.5K L), `GlobalTheory.deRham_potentiallySemistable` (21.7K L), `GlobalTheory.T.jacquetRallisUnitMatch_all_ranks` (19.0K L), and `UpperTheory.Zero.LLC.recTExists` (Local Langlands for $\mathrm{GL}_n$), sit directly under their lowest common dominator.)*

### Exact Top Dominator Metrics Table `[Exact]`

| Depth | Qualified Declaration (`in_cone = true`) | Source File & Line | Exclusive Subtree Lines | Exclusive Subtree Decls |
| :-: | :--- | :--- | ---: | ---: |
| 1 | `allChi` | `src/Supplements/Kernels/MainTheorems.lean:7` | 10,737,526 | 886,156 |
| 2 | `UpperTheory.FinalTheorems.everyCharacter` | `src/Development/AutomorphicArithmetic/CharacterBoxes/MainConclusion.lean:10` | 10,737,525 | 886,155 |
| 3 | `UpperTheory.FinalTheorems.everyCharacter_of_row` | `src/Development/AutomorphicArithmetic/CharacterBoxes/MainConclusion.lean:7` | 8,308,005 | 670,692 |
| 4 | `UpperTheory.LiteratureTheorems.every_character_of_literature_of_vWin` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/EveryCharacterOfLiteratureOfVWin.lean:772` | 7,172,414 | 587,596 |
| 5 | `UpperTheory.Lit.every_character_of_theta_integrals_local_matching` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/SiegelEisensteinSecondTermHypotheses.lean:69` | 4,245,419 | 366,417 |
| 6 | `UpperTheory.Lit.nonempty_hyps_definite_side_of_closed_bound` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/SiegelEisensteinSecondTermHypotheses.lean:44` | 3,576,707 | 305,572 |
| 7 | `GlobalTheory.Fc.S0.StageU.hypotheses_nonempty_of_unramified_exponents_alt` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/HypothesesNonemptyOfUnramifiedExponents.lean:41` | 1,217,085 | 102,963 |
| 4 | `UpperTheory.One.vwin_of_row` | `src/Development/AutomorphicArithmetic/AutomorphicVarieties/ModularityArgument/ThresholdArguments/DenseThresholdCases/VwinOfRow.lean:4` | 914,524 | 61,730 |
| 5 | `SelfIntersectionVolumeEstimates.topCovolSlim2_use_alt_alt` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/TopCovolSlim2Use.lean:16` | 824,935 | 58,036 |
| 3 | `UpperTheory.Gal.Int.galoisLocalGlobalStrong_recParameter` | `src/Development/AutomorphicArithmetic/Foundations/GaloisRepresentations/IntegralStructures/RowSrc.lean:6` | 689,213 | 64,176 |
| 6 | `PositivityTheory.HtPt.Ah.deep_slim_use` | `src/Development/WeilBound/CoreStatements/AutomorphicKernels/MoriwakiBigness/HilbertSamuelApplications/AhSlimU.lean:9` | 488,348 | 33,095 |
| 8 | `GlobalTheory.Fc.AllAtMemberStage0.allRows_of_open0_alt` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/LinkROfIsLinked.lean:50` | 453,684 | 39,552 |
| 7 | `GlobalTheory.Fc.S0.ontoX_and_lamLow_of_unitaryGroupHypotheses'` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/SurjectiveAndLowWeightOfUnramifiedExponents.lean:16` | 303,151 | 23,724 |
| 9 | `GlobalTheory.Fc.StageOneU.member_bundles_large_unramified_exponents_alt` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/MemberBundlesLargeUnramifiedExponents.lean:34` | 285,027 | 23,338 |
| 8 | `UpperTheory.GalC.final_alt_alt` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/CFinal.lean:19` | 224,129 | 18,273 |
| 6 | `UpperTheory.Top.every_character_of_green_v_win` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/TopPlug.lean:16` | 194,989 | 15,648 |
| 9 | `GlobalTheory.LocalRows.lg1_sLp_sO_hTF` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/AtChoice.lean:27` | 192,549 | 16,599 |
| 7 | `GlobalTheory.T.open0_of_unitary_group_hypotheses` | `src/Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/OpenOrbitOfUnramifiedExponents.lean:75` | 173,136 | 14,608 |

---

## 3. Pillar 1 — Analytic Gateway, Arakelov/Bost Ensemble Slope Inequality, Behrend-Sphere Host Tower, & Good-Tower Volume Estimates

### 3.1 From Windowed von Mangoldt Bounds to `EveryCharacter` `[Verified]`
- **Actual active bridge**: `UpperTheory.Top.every_character_of_green_v_win` (`TopPlug.lean:16`, depth 6) calls `UpperTheory.Top.zero_free_all_of_green_v_win` (`TopPlug.lean:12`, depth 7) and lifts primitive-character nonvanishing to all characters via `UpperTheory.Top.imprimitive`.
- **Weighted von Mangoldt sums**: The kernel-active definitions are `Formalism.Sz`, `Formalism.Gz` (`StatementOC.lean:11,13`), `Formalism.HC.An.G`, `Formalism.HC.An.gChi` (`Analytic.lean:14,23`), and the cyclic character-group weighted sum `Formalism.HC.Level.Gm` (`ExplicitFormulaErrorOfHostConditions.lean:38`).
- **Landau-type window-to-zero-free criterion**: `Formalism.HC.level_of_tiedWin` (`TopWin.lean:68`, depth 11) → `zeroFreeOn_of_tiedWin` → `zeroFreeOn_of_tiedWin_cyc` → `UpperTheory.Top.zeroFreeAll_of_topG` (`ZeroFreeAllOfTopG.lean:26`) deduces `Formalism.HC.ZeroFreeAll` from upper bounds $\Lambda.\mathrm{Gm}(x) \le x^{1/2 + (e_D+e_L+7)\varepsilon}$ on multiplicative windows $[Y, Y^{3/2}]$ (`FormBAt`).
- **Pontryagin sum & $\sqrt{x}$ size-window balance**: `Formalism.LFunctionChainParameters.characterSumData.{ASK, MAIN, De, colour_sum, inner_eq}` (`FirstPrimeNeSecondPrime.lean:84–93`, `StronglyIrreducible.lean:171,655`) connect inert prime-power sums to $G_\psi(a/w)$, and `Formalism.LFunctionChainParameters.ThickSetMeanBounds.proposition_size_bound` (`LipschitzWeightAndMinimum.lean:507`, depth 13) closes $G_x \le x^{1/2+(C+3)\varepsilon}$ under `SizeWindows` / `RestSize` (`SChain.lean:53,114`), balancing the chain length at $x^{1/2-\varepsilon} \le m \le x^{1/2+\varepsilon/2}$ and $xm \le k \le n^2 x^{1/2-\varepsilon}$ `[Reading]`.

### 3.2 Bost / Harder–Narasimhan Slope Inequalities (`Formalism.HN`) `[Verified]`
All four active theorems are exported in `src/Formalism/HarderNarasimhan/ShiftInvariance.lean` (`399/475` decls active in `HarderNarasimhan/` `[Exact]`):
1. `Formalism.HN.slopeIneqMean` (`ShiftInvariance.lean:54`, proved by `Formalism.HN.slope_inequality_mean`, depth 15): place-averaged Bost slope inequality $\hat{\mu}_{\min}(A) \le \hat{\mu}_{\max}(B) + h(\varphi)$ over totally complex number fields $K$ (used in `GL4/MuMaxLeMon.lean` and grid height bounds).
2. `Formalism.HN.sizeIneq` (`ShiftInvariance.lean:57`, depth 19): multiplicative covolume/size-ratio form $\mathrm{leastSize}(B)/\mathrm{largestSize}(A) \le (\prod_{v\mid\infty}\|\varphi\|_v)^{1/n_{\mathrm{pl}}(K)}$ (used in `GlobalTheory.RationalCount.separated` → `count_le` → `autBallCount` to count rational lattice points).
3. `Formalism.HN.theoremFSharp` (`ShiftInvariance.lean:60`, proved by `Formalism.HN.frame_theorem_sharp`, depth 20) & `Formalism.HN.theoremF` (`ShiftInvariance.lean:61`, depth 19): **frame-ensemble Bost slope inequalities** bounding the average target slope $\hat{\mu}(\mathrm{cc})$ from below using near-Parseval Hilbert–Schmidt and Cauchy–Schwarz trace-ratio errors $(e, \eta)$ on the padded archimedean frame matrices (`FrameLA1`). Consumed directly by `sharp_flat_lattice_theorem_holds` / `flat_lattice_theorem_holds` (`JointDModelAppliesUnitLattice.lean`) to prove `LinkGLOBAL` → `ChainTied` from `Formalism.MainNotions.{LinkG_at, LinkDIV_at, LinkS_at}` (`GlobalTheoremInputs.lean:77,122,145`) `[Verified]`.

### 3.3 Behrend-Sphere Thick Sets & Non-Uniform Bernoulli Orbit Selection `[Verified]` + `[Exact]`
Kernel inspection overturns a major static-analysis trap: the standalone directory `src/Formalism/BehrendSets/` has **`0 / 89` declarations in the kernel cone** (`[Exact]`, see Section 7). Instead, Behrend's construction enters the live proof through **two 100%-active mechanisms**:
1. **Direct Mathlib Behrend-sphere construction (`Formalism.FrobeniusOrbits`, `348 / 443` decls active `[Exact]`)**:
   - `Formalism.FrobeniusOrbits.hostThickExists` (`ThickExists.lean:181`, depth 14) invokes Mathlib's `Behrend.exists_large_sphere` and base-$4Ld$ sphere projection (`Formalism.FrobeniusOrbits.ThickExists.noRel_image`, `L85`) to prove `HostThickExists` with explicit constant $C = 32$: for all $n \in \mathbb{N}$ and $L \ge 2$, $\exists T \subseteq \mathbb{N}$ with $\forall s \in T,\ 4Ls < n$, `NoRel T L` (banning nontrivial 3-term linear relations $j(s_2-s_1) = i(s_3-s_1)$ for $0 < |i|,|j| \le L, i \neq j$), and $|T| \ge n\exp(-32\sqrt{\log n\cdot\log L})/L$.
   - `Formalism.FrobeniusOrbits.Final.host_field` (`HostCorollaries.lean:40`, depth 18) simultaneously proves all 19 algebraic/Galois/ramification properties of the explicit 2-adic CM host field $\mathbb{Q}(\sqrt{-q}, \sqrt{-d}, \zeta_{2^{e+2}}^+)$ (`hostOrdinary`, `hostUnimodular`, `hostEta`, `hostDiscr_of`).
   - `GlobalTheory.HTW.WindowTwo` (`Alternates.lean:14`) and `GlobalTheory.noThree_of_windowTwo` (`CyclicWordNoThreeMarkedOfAtMostTwoInWindow.lean:10`) convert `NoRel` into $\le 2$ orbit collisions per window.
2. **Non-uniform coset-biased Bernoulli selection (`src/Development/AutomorphicArithmetic/ThickFrames/`, `69 / 69` decls active `[Exact]`)**:
   - `UpperTheory.BernoulliSumsFrobeniusOrbits.thickFrame2` (`ThickFrame2Large.lean:29`, depth 13) wraps `hostThickExists` to supply `H.T2` (`|T_2| \ge x^{-\varepsilon} n`).
   - `UpperTheory.BernoulliSumsFrobeniusOrbits.select` (`select.lean:70`, depth 16) generalizes uniform Bernoulli selection to non-uniform per-element probabilities $0 \le p(a) \le \delta$ controlling every coset count simultaneously via a second-moment bound, and `topsChoiceH` (`topsChoiceH.lean:30`, depth 12) builds the primary character set `Host.TT` with `BadOK` and `Shares` balance, feeding `hostOfFieldOf` → `UpperTheory.Top.hostOfField` → `supplyOfAdm` → `supplyChi` → `zero_free_all_of_green_v_win`.

### 3.4 Good-Tower Existence & Arakelov Covolume Bounds (`824,935` exclusive lines) `[Verified]`
`SelfIntersectionVolumeEstimates.topCovolSlim2_use_alt_alt` (`TopCovolSlim2Use.lean:16`, depth 5, `58,036` decls `[Exact]`) supplies the Hodge-bundle covolume lower bound `GlobalTheory.CovTop.CovolumeBoundOfX` directly to `every_character_of_theta_integrals_local_matching`. Its dominant branch runs through:
- `PositivityTheory.HtPt.Ah.deep_slim_use` (`AhSlimU.lean:9`, depth 6, `488,348` L) → `GlobalTheory.CompanionTheory.Ar.Plug.deepStR_of_use` (`DeepStROfUse.lean:12`, depth 7) → `good_tower_volume_theorem_bis` (`GoodTowerVolumeTheoremBis.lean:9`, depth 8) → `GlobalTheory.T.final_theorem_of_inputs_right` / `goodTower_volume_of_reduced` (`FinalTheoremOfInputsRight.lean:28`, depth 10) → `GlobalTheory.T.good_tower_volume_theorem` / `GlobalTheory.CompanionTheory.Ar.Top.goodTower_and_volume` (`GoodTowerVolumeTheorem.lean:36`, depth 12, `297,554` L), establishing both `Top.GoodTowerExistsX` (root towers of unitary Shimura models with good origin) and `Top.VolumeTheoremX` (proportionality, arithmetic volume identity, and mean-slope height bounds) from Moriwaki bigness, Hilbert–Samuel, Fargues Hodge–Tate, Zarhin's trick, and resolution of singularities `[Reading]`.

---

## 4. Pillar 2 — Definite-Side Unitary Automorphy, Iterated Theta Lifts, Rallis/Siegel–Weil, Poincaré Series, & Galois Realizations

### 4.1 Definite-Side Assembly (`3,576,707` exclusive lines) `[Verified]`
`UpperTheory.Lit.every_character_of_theta_integrals_local_matching` (`SiegelEisensteinSecondTermHypotheses.lean:69`, depth 5) consumes `ThetaLocalInputs`, `ThetaIntegralsAgree`, `LocalMatching`, `CovolumeBoundOfX`, `RecTExists`, and `VWin`, and calls `UpperTheory.Lit.nonempty_hyps_definite_side_of_closed_bound` (`L44`, depth 6) to build `Nonempty (GlobalTheory.Fc.HypsAlt S0.NL DefiniteSideCondition)` via three major engines:
1. **`GlobalTheory.Fc.S0.StageU.hypotheses_nonempty_of_unramified_exponents_alt`** (`HypothesesNonemptyOfUnramifiedExponents.lean:41`, depth 7, `1,217,085` L):
   - `GlobalTheory.Fc.AllAtMemberStage0.allRows_of_open0_alt` (`LinkROfIsLinked.lean:50`, depth 8, `453,684` L) assembles all 26 row conditions of `AllRowsAlt`, including `GlobalTheory.LocalRows.lg1_sLp_sO_hTF` (`AtChoice.lean:27`, depth 9, `192,549` L), which proves loop-$L^p$, off-diagonal Theorem S (`GlobalTheory.CarrierGap.offDiag`, `OffK0Closed.lean:673`, depth 12, `183,705` L), and torsion-freeness directly from the convergent unitary Siegel–Weil formula.
   - `GlobalTheory.Fc.StageOneU.member_bundles_large_unramified_exponents_alt` (`MemberBundlesLargeUnramifiedExponents.lean:34`, depth 9, `285,027` L) supplies `SiegelMassRatioCM`, `MemberE`, and linked Weil-block bundles `ExBL`.
   - `UpperTheory.GalC.final_alt_alt` (`CFinal.lean:19`, depth 8, `224,129` L) upgrades the primes-$\{2,3\}$ norm bound into the full family norm-link predicate `Closed 𝒞 (fun Fm _ => LinkN Fm)` using local Tate duality, local Euler characteristic, and depth-zero support.
2. **`GlobalTheory.Fc.S0.ontoX_and_lamLow_of_unitaryGroupHypotheses'`** (`SurjectiveAndLowWeightOfUnramifiedExponents.lean:16`, depth 7, `303,151` L): pairs `GlobalTheory.BigOn.ontoX_of_hypotheses_alt_alt` (`OntoXOfPrints.lean:28`, depth 8, `298,150` L — surjectivity onto `X` from Clozel cohomologicality, definite-unitary base change, and Bhatt–Scholze crystalline lattices) with `lamLow_of_hypotheses_alt_alt` (low-weight lower bounds from Siegel–Weil and formal degrees).
3. **`GlobalTheory.T.open0_of_unitary_group_hypotheses`** (`OpenOrbitOfUnramifiedExponents.lean:75`, depth 7, `173,136` L): establishes all 12 linked open-orbit parameter predicates, including multiplicity injection via `GlobalTheory.miStarRow_alt_alt` (`MainTheoremPublishedInputsThird.lean:22`, depth 10, `151,285` L).

### 4.2 Iterated Theta Chain, Rallis Inner Product, Active Siegel–Weil / Genus Mass, & $U(p,q)$ Poincaré Decay `[Verified]`
- **Iterated theta chain**: Discrete column chain `Formalism.BasicNotions.ColumnChain.col` (`DataKernels.lean:70`) and continuous alternating Bessel↔Fourier–Jacobi inductive chain `GlobalTheory.Unf.Chain` (`IsThetaFam.lean:149`), running over definite/indefinite CM Hermitian spaces `Formalism.BasicNotions.Spaces` (`DataSpaces.lean:77`) of rank $N(j)=k+j(t+1)$.
- **Rallis inner product & Bessel periods**: `UpperTheory.Lit.Rallis.RallisIPProof.rallis_of_dm` (`RallisDM.lean:24`, depth 7) derives `RallisInnerProductConvergent` from the doubling method and convergent unitary Siegel–Weil via seesaw Fubini; `GlobalTheory.T.rallis_inner_product_big_theta_lift` (`BigThetaNormEqRallisInnerProduct.lean:47`) evaluates squared big-theta norms as products of Hecke $L$-values; `Formalism.MainNotions.BesselId` (`GlobalTheoremInputs.lean:65`) and `PI.BesselMapUniq.{IsCWCont, IsBesselMapSm, psiGen}` (`BesselMapUniq.lean:22–38`) connect algebraic and analytic Bessel periods.
- **Active Siegel–Weil & `GL5` genus-mass theorems (`in_cone = true`)**:
  - `GlobalTheory.CompanionTheory.SW.siegelWeilUnitaryConvergent` (`CompanionTheory/.../AdelicAnalysis/Imports.lean:10`, depth 7) proves the convergent unitary Siegel–Weil formula (`2n < M`), feeding `GlobalTheory.VolRatioFromSiegelWeil.volumeRatio` (`UnitaryHyperplaneVolumeRatioSiegelWeil.lean:10`, depth 8) and `GlobalTheory.LocalRows.lg1_sLp_sO_hTF`.
  - `PositivityTheory.SiegelWeilFormula.Fin.siegelWeilStd` (`SiegelWeilStd.lean:8`, depth 7) establishes `ThetaIntegralsAgree ∧ LocalMatching`.
  - Under `GL5` (`GenusMassVolumes`, i.e., *Global Lemma 5*, unrelated to $\mathrm{GL}_5$ `[Verified]`): `Formalism.L2.GlobalTheoryTheory.GenusMassVolumes.genus_class_mass` (`L10`) → `classMass_of_D7` (`L12`) → `volume_mass_of_genus_rank` (`PacketStatementOfIndependence.lean:22`, depth 11) + `GlobalTheory.genusMassAt_two` (`GenusMassAtTwo.lean:7`, depth 8) identify definite quotient volumes with Hermitian genus masses (base case rank 2, inducted via `volumeRatio`).
- **Projected $U(p,q)$ Poincaré series & matrix-coefficient decay**: `Formalism.L2.SG.PoincareInHH.PoincareSetting.Pu` (`PoincareInHH.lean:208`) maps `Cu X u hK →ₗ[ℂ] HHX X` (discrete-series $L^2$ projection), while the unconditional 0-hypothesis theorem `GlobalTheory.wuDecayOfGap` (`WuDecayOfGap.lean:7`, depth 8) establishes weighted $L^1$ Archimedean Whittaker–Bessel decay under `GapHyp D` ($r \ge 1801$, $b_{j+1}-b_j \ge 16 r^{19}$), feeding both `nonempty_hyps_of_allRows_alt` and `CarrierGap.offDiag` → `LinkS_at`.

### 4.3 Integral Galois Representations & Weight-One Geometric Realization (`689,213` exclusive lines) `[Verified]`
`UpperTheory.Gal.Int.galoisLocalGlobalStrong_recParameter : UpperTheory.GalStar.Row0b recT` (`RowSrc.lean:6`, depth 3) is the single Galois-representation theorem plugged into `everyCharacter_of_row` (simultaneously feeding `OpenLiteratureStatements` and `vwin_of_row`). It is proved by applying `UpperTheory.Gal.Int.galoisRepresentations_of_thetaLift` (`GaloisRepresentationsOfThetaLift.lean:6`, depth 4, `648,197` L) to the definite unitary theta-lifting theorem `UpperTheory.TP.K.tLiftK` (`TLiftK.lean:4`, depth 4, `40,938` L), backed by:
- `UpperTheory.Gal.Int.Geo.geoOne_holds` (`WeightOneGeometricRealization.lean:11`, depth 5, `568,278` L) → `UpperTheory.Gal.Int.Geo.geometric_realization_of_weight_one` (`GeometricRealizationOfWeightOne.lean:25`, depth 6, `538,436` L), synthesizing `steps_holds` (`L20`, `305,636` L: Eichler–Shimura, crystalline comparison, Hodge–Tate jump steps) and `kodaira_spencer_of_weight_one` (`L22`, `190,439` L: Kottwitz–Shimura / Kodaira–Spencer data from the holomorphic Lefschetz fixed-point formula for finite étale correspondences).
- Paired with `UpperTheory.Zero.LLC.recTExists` (`RecTExists.lean:20`, depth 4: local Langlands correspondence for $\mathrm{GL}_n(E_w)$ satisfying `LawsT`), these discharge all three fields of `UpperTheory.LiteratureTheorems.OpenLiteratureStatements` (`EveryCharacterOfLiteratureOfVWin.lean:177`) with zero user axioms `[Verified]`.

---

## 5. Pillar 3 — Weak Vandiver Window Bound (`VWin`), $U(3)$ Patching, Kato Euler System, & Ferrero–Washington (`914,524` exclusive lines)

### 5.1 Target & Downstream Hook `[Verified]`
- **Target**: `UpperTheory.Top.VWin` (`ExistsPageN.lean:49`) asserts for every primitive character $\psi$ and $0 < \varepsilon < 1/52$ the existence of multiplicative windows $[Y, Y^2] \subseteq [X^{1/J}, X]$ on which every cyclotomic compositum $E$ (`IsChiField ψ (e x) q d E`) satisfies `VVle E ⌊x^401⌋₊ √(exp(x^ε))`: finiteness `shaLen (ld p) e ≠ ⊤` for $1 \le e < \lfloor x^{401} \rfloor$ and $\sum_{\ell \in S}\log\ell \sum_{1 \le e < \lfloor x^{401} \rfloor}\mathrm{shaLen}(E,\ell,e).\mathrm{toNat} \le \sqrt{e^{x^\varepsilon}}$, where `Formalism.GaloisCohomology.shaLen` (`GaloisCohomology/Statement.lean:78`) is the $\mathbb{Z}_\ell$-length of $\Sha^1(G_{E,S_\ell}, (\mathbb{Q}_\ell/\mathbb{Z}_\ell)(-e))$.
- **Downstream consumption**: `UpperTheory.Top.divinN_of_hypV` (`DivinNOfHypV.lean:117`, depth 10) converts `HypV` + `LinkN` + `SmallLevelRankEq` into `DIVinN` (`etaSum L j ≤ D0 * C * x^ε * (B0 + Nc)`), sealing the arithmetic cohomology error inside `zero_free_all_of_green_v_win`.

### 5.2 Orchestration Spine & Six Core Engines Under `vwin_of_hypotheses_H_E_alt` `[Verified]`
`UpperTheory.One.vwin_of_row` (`VwinOfRow.lean:4`, depth 4) → `UpperTheory.One.Join.automorphic_window_of_inputs` (`TopMainZj.lean:71`) → `UpperTheory.V.HV.DescentVanishingReductions.v_window_of_automorphy_csd_ss_mono_odd` (`WindowBoundOfLiteratureLevel.lean:30`, `903,896` L) → `vwin_of_hypotheses_H_E_alt` (`L25`, `814,900` L) fans out into six verified engines:

1. **Kato Bloch–Kato Selmer Vanishing (`144,491` L)**: `UpperTheory.V.Z.Kato.WeilSphericalRecords.katoTwoW_of_printsH` (`KatoTwoWOfPublishedH.lean:48`) proves simultaneous Bloch–Kato Selmer vanishing for a weight-$(2e+1)$ modular representation $W$ and its quadratic twist $W \otimes \theta$, supplying `CountI`.
2. **4D Symplectic ($GSp_4$) Automorphy & Potential Modularity (`112,445` L)**: `UpperTheory.V.HV.DescentVanishingReductions.sp4In0_alt` (`Sp4In0.lean:17`) proves `SymplecticFourImageDense` (combining adjoint lifts, Ferrero–Washington, and potential modularity), supplying `CountII` and `CountIII`.
3. **Dense $U(3)$ Automorphic Seeds (`63,996` L)**: `UpperTheory.V.Z.Sym.W.unitary_three_automorphy_dense_of_prints` (`UnitaryThreeInputsOfLiterature.lean:30`) propagates symmetric-square/adjoint lifts from Eisenstein seeds to produce open-irreducible 3D automorphic seeds of weights $(m, 0, -m)$, supplying `CountI` and `CountV`.
4. **Taylor–Wiles–Kisin Patching (`47,885` L)**: `UpperTheory.V.Pa.pFromAut` (`PfromAut.lean:9`) establishes the universal ordinary determinant-ring patching properties `P1core J ∧ P3core J`, driving `ModUAlt` (`HVCore.lean:23`) for `CountI`, `CountII`, `CountIII`, and `CountV`.
5. **Effective $K$-Theoretic / $H^2$ Baseline (`38,854` L)**: `UpperTheory.V.ZK.lemmaK_litN_alt` (`KFinal.lean:11`) proves `∃ CK, LemmaK CK` (coarse exponential upper bound on $|H^2(G_{L,S_\ell}, \mathbb{Z}_\ell(e+1))|$) via `UpperTheory.V.LDataCardLeFinrank.vFormula` (`VFormula.lean:16`, depth 16: Poitou–Tate `VLen + len H^0 = shaLen + ∑ len H^0_v`) and `PositivityTheory.QuaternionShaBounds.sSha` (`OrderOfOneAddPrimePowZmodPrimePow.lean:490`).
6. **Satake Newton-Polygon Bounds (`33,479` L) & Final Window Assembly (`21,124` L)**: `UpperTheory.V.Z.Int.slopeAll` (`SlopeAll.lean:11`) bounds $w$-adic slopes of Satake parameters to count CM eigensystems; `UpperTheory.V.HV.Core.of_four_alt` (`HVCore.lean:18`, depth 10) merges `LemmaK`, `UpperTheory.V.FW.ferreroWashington` (`FerreroWashington.lean:104`, depth 11; 170/170 decls active `[Exact]`), Stickelberger/odd-residue bounds (`UpperTheory.V.Odd.thmBcFree_of`, `UpperTheory.V.X.residual_bernoulli_nonvanishing`, `propZ`), and `CountI`–`CountV` into `VWin`.

### 5.3 How the Live Kernel Does Kummer & Iwasawa Descent Without $\Lambda = \mathbb{Z}_p[[T]]$ `[Verified]` + `[Reading]`
A striking kernel-cone finding is that `GroundTheory/ArithmeticGeometry/IwasawaModules/` (`0/16` decls active) is **never used**: the live proof avoids formal power-series structure theory over $\mathbb{Z}_p[[T]]$ entirely. Instead:
- **Direct Kummer injection**: `UpperTheory.V.Kum.resInfInj` (`ResInfInj.lean:58`), `resInfOnto` (`ResInfOnto.lean:102`), and `reflect` (`Reflect.lean:44`) inject `Sha D e` directly into equivariant homomorphisms on the cyclotomic tower (feeding `QuaternionShaBounds.sSha` and `FW.reflOdd`).
- **Matrix-determinant Iwasawa descent**: `UpperTheory.V.Two.{fwRk, twoIn, fixedIn}` (`IsAlgClosureRat.lean:37,133,139`) convert `FW.ferreroWashington` directly into a finite $\mathbb{Z}_\ell$-generator bound `Tower.IsTower l L R` on the unramified Iwasawa module `XP l L`. Using unipotence of the topological generator $\gamma$ (`Kum.dinfNil`, `PropA.xUnip`) and exact layer descent (`Tower.step0`, `ExistsDescends.lean:26`), `PropA.exists_centres_norm` (`ExistsCentresLevel.lean:57`) and `aBound_of_descends` / `aBound_of_isTower` (`ABoundOfDescends.lean:36`, `ABoundOfIsTower.lean:7`) lift $\gamma^{\ell^j}$ to an $R \times R$ matrix over $\mathbb{Z}_\ell$ and factor its determinant over $\overline{\mathbb{Q}}_\ell$, showing `shaInv` can spike at most at $R$ exceptional $\ell$-adic centers—which `UpperTheory.V.Win.thmBab_of_lemmaK` (`ThmBabOfLemmaK.lean:9`) dodges to obtain `VWin`!

---

## 6. Selected Exact Kernel Witness Chains (`why-parents.csv.gz`) `[Exact]`

Every chain below is extracted verbatim from `why-parents.csv.gz` (full 36-landmark chains stored in [data/kernel_backbone_summary.json](data/kernel_backbone_summary.json)):

| Landmark Theorem (`in_cone = true`) | Depth | Shortest Kernel Witness Chain to `Main.lean` (`LFunction_ne_zero_of_half_lt_re`) |
| :--- | :-: | :--- |
| `UpperTheory.Top.every_character_of_green_v_win` | 6 | `every_character_of_green_v_win → every_character_of_theta_integrals_local_matching → every_character_of_literature_of_vWin → everyCharacter_of_row → everyCharacter → allChi → LFunction_ne_zero_of_half_lt_re` |
| `Formalism.HC.level_of_tiedWin` | 11 | `level_of_tiedWin → zeroFreeOn_of_tiedWin → zeroFreeOn_of_tiedWin_cyc → zeroFreeAll_of_topG → zero_free_all_of_green_v_win → every_character_of_green_v_win → …` |
| `Formalism.LFunctionChainParameters.ThickSetMeanBounds.proposition_size_bound` | 13 | `proposition_size_bound → ch_core → core_gm → windowInequality_of_artinInputs → level_gm_le_on_family_scales → zeroFreeAll_of_topG → …` |
| `Formalism.FrobeniusOrbits.hostThickExists` | 14 | `hostThickExists → thickFrame2 → hof_thick → hostOfFieldOf → hostOfField → supplyOfAdm → supplyChi → zero_free_all_of_green_v_win → …` |
| `UpperTheory.BernoulliSumsFrobeniusOrbits.select` | 16 | `select → topsGroup → topsFrame → topsBase → topsChoiceH → hostOfFieldOf → hostOfField → supplyOfAdm → …` |
| `Formalism.HN.frame_theorem_sharp` | 20 | `frame_theorem_sharp → theoremFSharp → sharp_flat_lattice_theorem_holds → ramification_condition_holds → joint_a_ramified_holds → frame_at_of → frameF_of → linkGLOBAL7_items → …` |
| `GlobalTheory.CompanionTheory.Ar.Top.goodTower_and_volume` | 12 | `goodTower_and_volume → good_tower_volume_theorem → goodTower_volume_of_reduced → final_theorem_of_inputs_right → good_tower_volume_theorem_bis → deepStR_of_use → deep_slim_use → topCovolSlim2_use_alt_alt → …` |
| `GlobalTheory.wuDecayOfGap` | 8 | `wuDecayOfGap → hypotheses_nonempty_of_unramified_exponents_alt → nonempty_hyps_definite_side_of_closed_bound → every_character_of_theta_integrals_local_matching → …` |
| `UpperTheory.Lit.Rallis.RallisIPProof.rallis_of_dm` | 7 | `rallis_of_dm → rallis_inner_product_convergent_of_literature → published_results_first → every_character_of_literature_of_vWin → …` |
| `GlobalTheory.CompanionTheory.SW.siegelWeilUnitaryConvergent` | 7 | `siegelWeilUnitaryConvergent → siegelWeilU_unitary_convergent_of_literature → published_results_eigen → every_character_of_literature_of_vWin → …` |
| `Formalism.L2.GlobalTheoryTheory.GenusMassVolumes.volume_mass_of_genus_rank` | 11 | `volume_mass_of_genus_rank → famGLmin_of_realPlaceSecondLayer → link_gl → shape_closed_link_n_of_green → zero_free_all_of_green_v_win → …` |
| `UpperTheory.V.FW.ferreroWashington` | 11 | `ferreroWashington → of_four_alt → vwin_of_hypotheses_H_E_alt → v_window_of_automorphy_csd_ss_mono_odd → v_window_of_literal_automorphy_n → v_window_of_depth_zero_residual_bc_cm → automorphic_window_of_inputs → vwin_of_row → …` |
| `UpperTheory.V.LDataCardLeFinrank.vFormula` | 16 | `vFormula → vFormula_of_classicalTheorems → VLen_le_shaLen_add → sFinal → lemmaK → lemmaK_of_litN_alt → lemmaK_litN_alt → vwin_of_hypotheses_H_E_alt → …` |
| `UpperTheory.Top.divinN_of_hypV` | 10 | `divinN_of_hypV → topDesWin → shape_closed_link_n_of_green → zero_free_all_of_green_v_win → …` |
| `UpperTheory.Gal.Int.galoisLocalGlobalStrong_recParameter` | 3 | `galoisLocalGlobalStrong_recParameter → everyCharacter → allChi → LFunction_ne_zero_of_half_lt_re` |
| `UpperTheory.Zero.LLC.recTExists` | 4 | `recTExists → everyCharacter_of_row → everyCharacter → allChi → LFunction_ne_zero_of_half_lt_re` |

---

## 7. Dead-Code Traps & Self-Audit of Earlier Static Reports `[Exact]`

Before the `v0.2.1` kernel export was joined against our static DuckDB index, our earlier static-only Round 2 reports (`rosetta_stone.md` and `rh_proof_explained.md`) cited **24 declarations** that exist in the import closure yet have `in_cone = false` in the kernel export. Running [`scripts/audit_spec_citations.py`](scripts/audit_spec_citations.py) caught every single one; all 24 have now been corrected across both companion reports and are transparently catalogued below inside an audit exclusion fence so future readers do not fall into the same static-analysis traps:

<!-- audit:allow-dead-start -->

| Dead / Unused Citation (`in_cone = false`) | Why It Misled Static Analysis | Actual Kernel-Active Replacement (`in_cone = true`) |
| :--- | :--- | :--- |
| `Formalism.BehrendSets.BehrendBound`, `Formalism.BehrendSets.TranslateMean`, `Formalism.BehrendSets.Final.select` (`0/89` decls active in `BehrendSets/`) | Standalone earlier Behrend formalization inside the import closure, completely bypassed by the final proof. | `Formalism.FrobeniusOrbits.hostThickExists` (calls Mathlib's `Behrend.exists_large_sphere` directly, $C=32$) + `UpperTheory.BernoulliSumsFrobeniusOrbits.{thickFrame2, select, topsChoiceH}` (`ThickFrames/`, `69/69` active). |
| `UpperTheory.Top.plugAll`, `UpperTheory.Top.plugWin` (`EveryCharacter.lean:8,16`) | Early wiring of `Hyps ... DRS` superseded by `HypsAlt ... DefiniteSideCondition`. | `UpperTheory.Top.every_character_of_green_v_win` & `zero_free_all_of_green_v_win` (`TopPlug.lean:12,16`). |
| `Formalism.Gchi`, `Formalism.Schi` (`StatementOC.lean:15,20`) | Part of an unused real-character lower-bound formulation (`OCStep`/`OCAvg`). Only `Formalism.Sz` & `Formalism.Gz` are active in `StatementOC.lean`. | `Formalism.HC.An.G`, `Formalism.HC.An.gChi` (`Analytic.lean:14,23`), `Formalism.HC.Level.Gm` (`ExplicitFormulaErrorOfHostConditions.lean:38`). |
| `Formalism.LFunctionChainParameters.characterSumData.AskMainBalance`, `colour_balance` | Unused top-level balance wrapper; its constituents `ASK`, `MAIN`, `De`, `ColourSum`, `OrthoRam` are active. | `Formalism.LFunctionChainParameters.characterSumData.{ASK, MAIN, De, colour_sum, inner_eq}` (`FirstPrimeNeSecondPrime.lean`, `StronglyIrreducible.lean`). |
| `Formalism.LFunctionChainParameters.LChain.SizesTextAt` (`SChain.lean:207`) | Unused text-parameter bundle wrapper inside `SChain.lean`. | `Formalism.LFunctionChainParameters.LChain.{SizeWindowsSplit, RestSize}` (`SChain.lean:53,114`) + `SizeWindows` / `proposition_size_bound`. |
| `Formalism.RankGapBounds.WeightedMeanData.Data.MeanBoundSecondForm` (`L187`) | Unused top-level `def : Prop` wrapper; its inner fields (`Data`, `HidaSetting`, `DIV`, `SlopeIneq`) are active. | `Formalism.RankGapBounds.WeightedMeanData.Data.{key_identity, SlopeIneq, HidaSetting, DIV}` (`MeanBoundSecondForm.lean`, `WeightedMeanData.lean:170`). |
| `Formalism.HN.slopeIneq` (`ShiftInvariance.lean:43`) | Abstract Euclidean form unused at top level; the active proof consumes the totally-complex number-field forms directly. | `Formalism.HN.{slopeIneqMean, sizeIneq, theoremF, theoremFSharp}` (`ShiftInvariance.lean:54,57,60,61`). |
| `Formalism.GradedWeylDivisibility.bidetSpan` (`StatementB.lean:15`) | Unused bideterminant-span predicate; the active graded-Weyl path works via `ClosedEqNabla` / `WeylEqNablaQ` and SSYT peeling. | `Formalism.GradedWeylDivisibility.{fockQ, ClosedEqNabla, WeylEqNablaQ, leadInterlaced, twoColumn, sSYTPeel}` (`StatementB.lean`, `FinalC.lean`). |
| `PI.BesselMapUniq.BesselMapUniq` (`BesselMapUniq.lean:50`) | Unused `def : Prop` wrapper; its constituent definitions (`ellA`, `psiGen`, `IsCWCont`, `IsBesselMapSm`) are active. | `PI.BesselMapUniq.{IsCWCont, IsBesselMapSm, psiGen}` + `CompanionTheory/AutomorphicTheory/LocalBesselUniqueness/`. |
| `GlobalTheory.Published.SiegelWeilAnisotropicHermitian`, `Formalism.L2.GlobalTheoryTheory.GenusMassVolumes.WeilSiegelCM`, `packet_statement_of_independence` | Unused older Siegel–Weil / Kudla-packet wrappers (`GL5/Statement.lean` & `PacketStatementOfIndependence.lean:38`). | `GlobalTheory.CompanionTheory.SW.siegelWeilUnitaryConvergent`, `PositivityTheory.SiegelWeilFormula.Fin.siegelWeilStd`, `GlobalTheory.VolRatioFromSiegelWeil.volumeRatio`, `GenusMassVolumes.{genus_class_mass, classMass_of_D7, volume_mass_of_genus_rank}`, `GlobalTheory.genusMassAt_two`. |
| `GlobalTheory.T.seesaw_identity_config_a_fourth` | Unused seesaw specialization; general seesaw laws are active. | `GlobalTheory.T.seeSaw_latticeFramePins_finiteIntegralLaws`, `GlobalTheory.T.seeSawIteratedIntegralLaw_at_twist_of_factorisation`, `UpperTheory.Lit.Rallis.RallisIPProof.rallis_of_dm`. |
| `Formalism.L2.SG.Pf.wuDecay_of_gap`, `Formalism.L2.SG.ThmSOwn.linkS_selH` | Conditional / superseded assembly theorems in `WuDecayAll.lean:1058` and `ThmSOwn.lean:923`. | `GlobalTheory.wuDecayOfGap` (`WuDecayOfGap.lean:7`), `GlobalTheory.CarrierGap.offDiag` (`OffK0Closed.lean:673`), `Formalism.L2.SG.ThmSOwn.{TheoremSSelected, size_numbers_of_hypotheses, hsNormSq_diag}`. |
| `Formalism.HermitianLattices.FiltrationBoundaryCriterion.PfM.coord`, `Formalism.HermitianLattices.ZeroFreeRegionGL2.weighted_von_mangoldt_sum_identity` | Unused helper lemmas inside `GL3MeanSquares/HookFormulaTorsion/`. | `Formalism.HermitianLattices.GlobalBoundRefinements.shortIntervalLowerBound`, `Formalism.HermitianLattices.ZeroFreeRegionGL2.nonvanishing_of_hypothesis`, `Formalism.HC.Frame.hostThickOrbit`. |
| `UpperTheory.V.Kum.shaLen_le_of_cl` (`MinusLenLeOfCl.lean:129`) | Unused error-term class-group bound (`39/256` dead decls in `KummerTheory/`). | `UpperTheory.V.Kum.{resInfInj, resInfOnto, reflect}` + `PositivityTheory.QuaternionShaBounds.sSha` (`OrderOfOneAddPrimePowZmodPrimePow.lean:490`). |
| `GroundTheory.Arith.Iwasawa.Lambda` (`0/16` decls active in `IwasawaModules/`) | Formal power-series ring $\mathbb{Z}_p[[T]]$ never enters the kernel cone. | Direct matrix-determinant descent on `Tower.XP`: `UpperTheory.V.Two.{fwRk, twoIn, fixedIn}`, `Tower.step0`, `PropA.{xUnip, exists_centres_norm, aBound_of_descends}`, `Win.thmBab_of_lemmaK`. |
| `UpperTheory.V.SmallClassNumbers.Bound`, `Large` (`0/23` decls active in `SmallClassNumbers/`) | Standalone downstream corollary over $\mathbb{Q}(\sqrt{-3})$ (already removed from upstream `main`), never feeding `Main.lean`. | Core `VWin` spine (`UpperTheory.One.vwin_of_row` → `UpperTheory.V.HV.Core.of_four_alt` → `UpperTheory.Top.divinN_of_hypV`). |

<!-- audit:allow-dead-end -->

---

## 8. Independent External-Kernel Verification & TCB / Metaprogramming Audit `[Exact]` + `[Verified]`

### 8.1 Multi-Kernel & Arithmetic-Consequence Verification (MIT page last updated 2026-11-25 22:05 UTC) `[Exact]`

Unless a row says otherwise, entries are taken from Andrew Sutherland's [MIT verification page](https://math.mit.edu/~drew/grh/) for the original proof on Lean `v4.33.1` / Mathlib `v4.33.1` (`solution.ndjson` = `61,570,595,788` B, `sha256 5ea74b91…244e`; `challenge.ndjson.zst` about `47 MB`, unpacked `350,862,970` B, `sha256 bd484ac9…b2b9`). The shorter proof now on upstream `main` is checked on a [separate MIT page](https://math.mit.edu/~drew/shortgrh/).

| Checker / Audit | Verdict | Scope & Measured Detail |
| :--- | :--- | :--- |
| Sandboxed `lake build Main` + export | **PASS (identical)** | Zero errors (offline, read-only source); exported `61.57 GB` proof matches published `sha256 5ea74b91…244e` byte for byte (`175,186/175,187` `.olean`s identical; 1 non-deterministic module) |
| `#print axioms` & Mathlib integrity | **PASS** | Only `{propext, Classical.choice, Quot.sound}` on all 5 `Main.lean` theorems; all 15 environment axioms reside in `Init`; all `8,322` Mathlib + `378` dependency `.olean`s match official cache byte for byte |
| `leanchecker` (Lean kernel replay) | **PASS** | `175,187 / 175,187` modules replayed, 0 rejections |
| `lean4lean` (`67f63b0c`, Lean `v4.33.1`) | **PASS** | `175,187 / 175,187` compiled modules, checked module by module on the `.olean` files rather than the export (`175,182` at default limits + `5` `decide +kernel` modules with raised step limit) |
| `nanoda` (independent Rust kernel) | **PASS** | Full `61.57 GB` export accepted (`exit 0`, 27m 42s on 80 threads) |
| `con-leche` (`master`, `cd2eceab`, `--verified`) | **PASS** | `1,468,418` declarations of the full export accepted (3m 15s on 64 threads) |
| `con-leche --compare` (branch `compare`, `5dc36dc1`) | **PASS** | all five theorems "justified by the solution"; `51,692` challenge declarations accepted |
| Lean default and paranoid kernels (`leanchecker --from-export`, `leanchecker-paranoid --from-export`, Lean `v4.36.0`) | **PASS** | full export accepted (`exit 0`; single-threaded, 48 h 3 min and 72 h 20 min with default settings) |
| Comparator `v4.33.0` and `lake comparator` (Lean `v4.36.0`) | **PASS** | statement comparison and axiom check pass; both report "Your solution is okay!" |
| `con-ron --compare` (Rust port, `master`, `3f22699b`) | **PASS** | `1,468,418` solution + `51,692` challenge decls accepted in `--verified` mode (`exit 0`, 7m 06s on 64 threads) |
| **Zeta-free arithmetic check** (`kimihiro64/Robin1984`, `5ce18bf3`) | **PASS** | Combining the proof with an independent formalization of Robin-type criteria proves `robin_inequality` ($\forall n > 5040,\ \sigma(n) < e^\gamma n \log\log n$), `lagarias_inequality` ($\forall n > 0,\ \sigma(n) \le H_n + e^{H_n}\log H_n$) and `cyclotomic_corrected_ideal_robin`, each with `{propext, Classical.choice, Quot.sound}` only; the combined `61,801,392,638`-byte export (`1,475,109` declarations, `sha256 396f55dc…54a5`) is accepted by `con-leche`, `con-leche --compare`, `con-ron --compare` and `nanoda` (29 min 9 s on 64 threads). The first two inequalities were first checked against the earlier commit `acab1a3` with the same outcome |
| **Elaborated arithmetic statements** ([`explicit-statements.out`](https://math.mit.edu/~drew/grh/explicit-statements.out)) | Recorded | Elaborated with `pp.all` against Mathlib `0df444a3` alone (none of the proof loaded): Robin's statement names **21** constants directly and its meaning depends on **2,050** declarations; Lagarias's names **27** and depends on **1,801** (the difference is mainly the definition of Euler's constant) |
| **Lean-free raw NDJSON audit** ([`scripts/audit_ndjson_statement.py`](scripts/audit_ndjson_statement.py)) | **PASS** | Independent stdlib-only Python parse of MIT's `challenge.ndjson.zst` (`51,692` top-level records incl. `46` `opaque`, `6,495,712` lines, `sha256 bd484ac9…b2b9`, `0` unsafe, `0` missing): extracts all 5 raw kernel statement ASTs and confirms `{Classical.choice, Quot.sound, propext}` only (`0` `sorryAx`) across the full statement-type closures — `riemannHypothesis` & `riemannZeta_eq_zero_re` (`32` direct / `4,889` def-closure / `52,527` full), `LFunction_ne_zero_of_half_lt_re` (`34` / `5,126` / `52,889`), `LFunction_eq_zero_re` (`29` / `5,119` / `52,869`), `LFunction_eq_zero_re_of_isPrimitive` (`30` / `5,236` / `52,992`) |
| **`formal-conjectures` statements** ([PR #88](https://github.com/leanprover-community/riemann-hypothesis/pull/88) by `@Deicyde`, merged 2026-11-23) | Reported in PR | The PR adds a second Comparator setup whose challenge copies the RH and GRH statements of `google-deepmind/formal-conjectures` (commit `d8d031d4`). In a PR comment (2026-11-22 13:22 UTC), `@ishaan2006` reported `con-leche --compare` (branch `compare`, `274f0138`) on a `61,570,614,651`-byte export (`sha256 17719b54…`) accepting `RiemannHypothesis.riemannHypothesis` and `GRH.generalized_riemann_hypothesis` |

### 8.1b Upstream `main` After `v0.2.1` `[Exact]`

PR states and times below were read from the GitHub API on 2026-11-26; file and line counts are this tracker's own measurements in [`data/upstream_frontier.json`](data/upstream_frontier.json).

- 2026-11-22: [PR #60](https://github.com/leanprover-community/riemann-hypothesis/pull/60) and [PR #65](https://github.com/leanprover-community/riemann-hypothesis/pull/65) merged; [PR #66](https://github.com/leanprover-community/riemann-hypothesis/pull/66) (`14f383e5e`) moved `main` to Lean `v4.36.0` (measured `149,396` files); [PR #115](https://github.com/leanprover-community/riemann-hypothesis/pull/115) (17:50 UTC) and [PR #150](https://github.com/leanprover-community/riemann-hypothesis/pull/150) (21:50 UTC) merged the reviewed Lean `v4.36.0` cleanup batches (measured `149,397` files and `15,168,601` lines at `main@51d9b1a28`).
- 2026-11-23: [PR #185](https://github.com/leanprover-community/riemann-hypothesis/pull/185) ("Integrate proof cleanup and bump to Lean 4.37.0-rc1", `@kim-em`) and [PR #134](https://github.com/leanprover-community/riemann-hypothesis/pull/134) (`@Vilin97`) merged.
- 2026-11-24: this tracker maintainer's [PR #109](https://github.com/leanprover-community/riemann-hypothesis/pull/109), [PR #110](https://github.com/leanprover-community/riemann-hypothesis/pull/110) and [PR #224](https://github.com/leanprover-community/riemann-hypothesis/pull/224) were closed without merge.
- 2026-11-25: `main` switched to a tree based on the shorter `lsernau/grh-lean@72fead10` proof; the original tree stays on the `alpoge` branch. The latest measurement is `802` files and `475,141` lines at `main@4249e7b98` (Lean `v4.37.0-rc1`, 2026-11-26 06:10 UTC).

### 8.2 Trusted Computing Base (TCB) & Metaprogramming Audit `[Exact]` + `[Verified]`

Cross-checking upstream [`verification/static-scan.txt`](https://github.com/leanprover-community/riemann-hypothesis/blob/alpoge/verification/static-scan.txt) (across all `175,187` original source files; kept on the `alpoge` branch) against our line-by-line metaprogramming audit in [`meta_audit_verdict.md`](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/meta_audit_verdict.md) confirms **zero TCB bypass paths**:

- **Zero risky declarations or compiler-trust escapes (`[Exact]`)**: `0` `axiom`, `0` `sorry`, `0` `admit`, `0` `native_decide` / `ofReduceBool` / `trustCompiler`, `0` `@[implemented_by]` / `@[extern]`, `0` `unsafe`, `0` `opaque`, `0` `set_option debug.*`, `0` `skipKernelTC`, `0` `initialize`, and `0` non-local `notation`.
- **Line-by-line verification of every non-zero metaprogramming hit (`[Verified]`)**:
  - `CutCptCopy.lean` (`addDecl` ×2, `partial def` ×1, `elab` ×1): clones already-proved `Formalism.FourierDecayAlt.*` theorems into compact-support (`...Cpt`) variants by calling `addDecl (.defnDecl ...)` / `addDecl (.thmDecl ...)`, which routes strictly through C++ `lean_add_decl` kernel checking and explicitly rejects `.opaqueInfo`.
  - `BuildAid/D1..D5.lean` (`run_cmd` / `modifyEnv` ×5): five 4-line files that only append strings to `LibrarySuggestions.nameDenyListExt`; they register `0` kernel declarations (`0/0` in cone) and cannot touch the kernel constant table.
  - `allowUnsafeReducibility` (37 hits) & `hygiene` (31 hits): all 37 `allowUnsafeReducibility` directives only attach `attribute [local irreducible]` (elaborator transparency); all 31 `hygiene` options are `set_option hygiene false in` scoped to local notations or tactic macros.
- **Named classical-theorem cross-reference**: For community cross-checking of the named statements under `GlobalTheory.Published` and `UpperTheory.LiteratureTheorems`, see upstream [Issue #39](https://github.com/leanprover-community/riemann-hypothesis/issues/39).

---

## 9. Reproducibility `[Exact]`

To re-verify that every active qualified Lean declaration cited in this specification, in [rosetta_stone.md](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/rosetta_stone.md), and in [rh_proof_explained.md](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/rh_proof_explained.md) appears in the `v0.2.1` Lean kernel export with **0 dead citations**:

```sh
python scripts/audit_spec_citations.py \
  --cone path/to/cone-constants.txt.gz \
  --decls path/to/decls.csv.gz \
  SPEC.md README.md
```

To independently inspect the five raw kernel headline statements and verify their SHA-256, safety flags, and axiom closures directly from MIT's `challenge.ndjson.zst` (no Lean toolchain or third-party Python packages required):

```sh
python scripts/audit_ndjson_statement.py \
  --expected-sha256 bd484ac9886a96f26b1d9a7a59ebd45b7a70d102c015d4f31f9302bbdd6ab2b9 \
  path/to/challenge.ndjson.zst
```
