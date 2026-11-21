# 10. The open frontier

This page collects, from every report's "unread" and "what I could not check" sections, the declarations nobody has read yet, grouped by strand and ordered by how much of the argument depends on them. It ends with the open questions that reading alone has not settled.

Priority is the editor's ordering from the reports' own recommendations: **P1** = where the reports locate the non-classical weight of the argument, or a step everything downstream depends on; **P2** = substantive but with a clear classical model; **P3** = classical or bookkeeping, lower risk. Paths are relative to the archive's `src/`. `AV/` = `Development/AutomorphicArithmetic/AutomorphicVarieties/`, `GR/` = `Development/AutomorphicArithmetic/Foundations/GaloisRepresentations/`, `TPS/` = `Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/ThirdPlaceSeries/`, `DTC/` = `AV/ModularityArgument/ThresholdArguments/DenseThresholdCases/`, `IE/` = `Development/AbelianVarietyTheory/Proofs/InertiaEigenvalues/`.

## P1: read these first

Items that the reports marked as decisive and that were read since the first frontier pass have moved out of this list (see "Read since the first pass" below).

| # | Declaration(s) | File | Why it matters | Source |
|---|---|---|---|---|
| 1 | `KGLt.first_kernel_law_global_small`, `first_kernel_small_of_frame_twist` | `Formalism/LocalToGlobal/InertKernelTheory/InertCoprimeLemmas/FirstKernelSmallOfFrameTwist.lean:124` and callees | the claim that the **actual** global lift kernels, in Lubin–Tate coordinates at the CM point, have the form exp(−Σβ)·U with U integral: the bridge from automorphic theta lifts to the p-adic power series behind `Conclusion1`. Read only to line 130 | frontier-f-divisibility.md Task 2(d), Task 5 |
| 2 | `T.thetaLocalModels_of_splitUnramified_joints`, `Svd.jointsAt_all_std` (via `modelsAt_of_part`), `TwTie.KerAtMu`, `admissible_third_instance`, `admissible_fourth_instance` | `Proofs/SatakeParameters/TuplesHaveLocalModelsOfStandardColumnObjects.lean:11`; `SecondKernelInstances.lean:128` | that the global chain steps K_j equal the lattice-sum kernel expansion (`ExpansionEquality`), with stabilizers dividing DGam | frontier-f-divisibility.md Task 5 |
| 3 | `PfromAut` internals: `UpperTheory.V.Pa.pFromAut_of`, `chain_of`, `TaylorWilesObstructionBound`, `FirstOrderDualArithmeticBound`; the predicates `Point.NiceI` / `NiceE`, `IsThick`, `IsStable` | `AV/PatchingModules/ChebotarevFamilies/PFromAutOf.lean`; `AV/PatchingModules/AutomorphicDetection/`; `AV/WeilGroupCharacters/UniversalOrdinaryDeterminantRing.lean` | Taylor–Wiles patching for residually reducible pseudo-deformation rings, stated for all n and arbitrary residual data, "at or beyond the published frontier"; which residual hypotheses the predicates impose compared with Thorne 2015 and Allen–Newton–Thorne | frontier-g-deep-inputs.md §II.2, §II.4 |
| 4 | the linking argument `twoRuns_alt` (`DTC/TwoRuns.lean:80`), `seedPtOf_alt` (`DTC/PrSeedPtOf.lean:255`), `gdEnd_of_frame_linkedEnd_downCsdK_descent`; the theorem inhabiting `UpperTheory.V.W.HidaUAlt` (`DTC/HidaU.lean:12`), `arithmeticPointClassicalityDense` | `DTC/` = `AV/ModularityArgument/ThresholdArguments/DenseThresholdCases/` | why every ordinary type-I point lies on a component reached from a seed; where Hida theory for unitary groups is proved | frontier-g-deep-inputs.md §II.2 |
| 5 | the good-twist chain: `admAB_of_toric_parts` (`IE/IsAuxiliaryPlaceFamilyAtLevelMono.lean:355`), `residual_selmer_determinant_order_bound` (via `IE/ResidualSelmerBoundOfAuxiliaryPrimesOdd.lean`), `thmC_of_parts` (`IE/AddValLeOfNotMem.lean:87`), `PropE` / `propE_of_admissible`, `PtSel.selCore` (`IE/PsiAtPsiAt.lean`) | `IE/` = `Development/AbelianVarietyTheory/Proofs/InertiaEigenvalues/` | the proof of `TwistingCharacterExistence`: twisted Bloch–Kato Selmer groups can be made nearly trivial while twisted theta elements keep their valuation. "The most load-bearing and least standard step" on the Ш-link strand | frontier-e-ribet.md Q3–Q5 |
| 6 | "Theorem S" for theta kernels: `theorem_s_uniform` (`…/FrobeniusLiftNorms/OmegaOfTraceDuality.lean:31`), `gcCondition_of_twoSums`, `restTwoSums`; the jet / factorial inequality `EntryStar_of`, `gainFactorialInequality`, `excessGainBound`, `cyclicShiftLeast`, `HasseDerivProdVanish` | `Formalism/ThetaLogCoordinates/UniformThetaSubstitution/…`, `GrowthSums/…` | the p-adic heart of `Conclusion1`: where Σ_{odd e}(a/p^e − cap)⁺ is actually derived | frontier-f-divisibility.md Task 2, Task 5 |
| 7 | the `LinkS` provider below the top: `TheoremSSingle`'s provider, `loop_rows_closed_gap`, the diagonal e0 bound, `BesselId`'s provider; `CountAllPointsClass`, `VolSigIndep`, `MatK_at`, `InputSG` | `Formalism/LocalToGlobal/SiegelLatticeTheory/FlagStabilizerTensors/ThmSOwn.lean:968-1060`; `Development/AbelianVarietyTheory/Proofs/FlagMeasures/LoopRowsOfLargeGapDiscreteSeries.lean` | the near-orthogonality that makes the slope lower bound bite; without it the divisor inequality has no content | frontier-a-divisor.md Q4–Q6; frontier-f-divisibility.md Task 4 |
| 8 | Lefschetz local terms `UpperTheory.Gal.W2.FixLoc_holds` (inputs `CspDim`, `EigTop`, `LocSum`, `CorrChart`); `complex_count_regular_marked_points` (571 lines); the uniformisation `unifG_of`, `gUnif_of`, `complexModel_points_of_genericFiber_embedding` | `GR/SphericalCoordinates/`; `…/EtaleClopenDescent/ComplexCountRegularMarkedPoints.lean` | whether the Lefschetz local terms match the Weyl character, and whether the bijection between complex points and the group-theoretic double cosets (ker¹ included) is right | frontier-g-deep-inputs.md §I.4, "What to read next" |
| 9 | per-step Gamma-size bound: providers of `nKcb` / `CBHyp`; `adm3` / `L3main`; `slope_bounds` | `Formalism/ThetaSupremumBounds/StatementCB.lean:148`; `DivIneqExplicit6.lean:15`; `Formalism/SharpRestriction/Divisibility/LocalQuadraticParts.lean` (after 228); `Formalism/DivisorInequality/DivisorInequalityProofs/BaseLevelBoundsWithCovolumeHolds` | that log‖K_j‖ ≤ Σ ph(ν) − ν·logΩ holds with the full top entry a_t in every step j: the source of the m copies of a log a − a | frontier-a-divisor.md Q6 |
| 10 | `NumbersLink.lean:120-535` and the source of `NumFacts` (`H u` in `pkg_of_rowsK_alt`) | `Formalism/LocalToGlobal/Divisibility/NumbersLink.lean` | the residual bounds `sE`, `sδ` are the real numerical content of the divisor record; two other fields are tautologies by definition | frontier-a-divisor.md Q1b, Q6 |

### Read since the first pass

| Item | Was | Now | Source |
|---|---|---|---|
| provider of `Conclusion1` | not found | found and proved via `FormalGroupSystem.Pf.stmt_proved`; mechanism read at skeleton level | frontier-f-divisibility.md |
| `length_heckeValueIdeal_le_determinant_bound` | unread | [proof read in full]: a Fitting-ideal / GMA argument | frontier-e-ribet.md §1 |
| `Cut.ShaBound.main` | statement only | [proof read in full]: formal | frontier-e-ribet.md §3 |
| `jump_bound_family_large` | unread | [proof read in full]: Hodge–Tate weight combinatorics | frontier-e-ribet.md §4 |
| `blochKatoTop_…_alt`, `UseLoc.main` | unread | [proof read in full]; `BlochKatoLocal` hypothesis discarded | frontier-e-ribet.md §6 |
| matching of Lefschetz fixed points with `dTrace`; holomorphic Lefschetz formula | unread | bijection and trace identity read; Lefschetz formula [proof outline read], proved algebraically | frontier-g-deep-inputs.md §I |
| modularity of type-I points | unread | top level read: Hida + patching (`PfromAut`) + seeds + linking | frontier-g-deep-inputs.md §II |
| `LemmaM` / `LemmaM'` | statements only | top-level proofs [proof outline read]; judged lower risk | frontier-g-deep-inputs.md §II.3 |
| Ferrero–Washington via Sinnott | skeleton names | statement and skeleton read; `sinnott` read in full | frontier-g-deep-inputs.md §III |

## P2 by strand

### Analytic endgame and `core_gm` (pages 03, 04)

| Declaration(s) | File | Why it matters | Source |
|---|---|---|---|
| `shape_closed_link_n_of_green` (body) and the discharges of its ~50 `TopShapeAlt` fields; `TopG`, Green's theorem input | `…/ThirdPlaceSeries/TopShape.lean:33` | wires all `Link*` inputs into `topDesWin`; only the skeleton was read | core-gm.md Q6; analytic-core.md "Not read" |
| `GlobalTheory.Top.link_gl` → `famGLmin_of_realPlaceSecondLayer … .link8`; who supplies `hLoss`, `hSN`, `hFr` to `linkGLOBAL` | `Formalism/MainNotions/Wire` | how the concrete family meets `LinkGLOBAL` | core-gm.md Q6; frontier-b-links.md Q6 |
| `supplyOfAdm`, discharge of `hSup` | `ShortNorms/…` | how good pairs become host data | analytic-core.md "Not read" |
| rest of `Deduction.ch_balance` (`askExp`, `sum_cut`, `inner_eq`) | `DedekindLFunctions/LandauTheorems/ChBalance.lean:200` | the explicit-formula identity ASK − MAIN ↔ Fourier sum of G | core-gm.md Q6; frontier-a-divisor.md Q6 |
| `Ga_bound5`, `errX5at_le` | `ChainSlopeWindows/LevelOfCongruence.lean`, `PChain.lean:613` | arithmetic behind `each_unit_bound_statement` | core-gm.md Q6 |
| `height_bound_at_set`, `stepDiiiRecord`, `HtNums.hPn_le_hA`, `complex_height_cast` | `Formalism/DivisorInequality/DivisorInequalityProofs/EvenDegreeWeightBounds.lean:154` and others | final-pairing and composite-height bounds | frontier-a-divisor.md Q6; core-gm.md Q6 |
| `linkDIVt_of_rowsT`, `pkg_of_rowsK_alt` (body), `DivisionPackageColumns.ofTied`, `DivInputsDef3.toDivInputs0R`, `DivisionInputsWithoutNorm.ofRows`, `hOpen` in `wire4F_alt` | `Divisibility/…`; `…/ThirdPlaceSeries/Wire4F.lean:26` | providers of the divisor inputs | core-gm.md Q6; frontier-a-divisor.md Q6 |
| `losses_of_small_share_weight`, `loss_setting_family`, `joint_discriminant_eventually`, `loss_points_of_choices` | `LocalToGlobal/GlobalTheory/LossRequirementsOfSmallShares.lean`, `PowFourLtCeilThresholdPrime.lean:2039` | the LinkG → LossBounds path (lower order) | core-gm.md Q6; frontier-b-links.md Q6 |
| `D7` itself: `what₂.perCol`, `thetaP`, `Mt`, `MtLat`, `hc : HCParam`, `div.Bk`, `div.RR`, `besS` | | what the abstract records name concretely | frontier-a-divisor.md Q6 |
| `GlobalTheory.HondaCM3.cm3` (`DecoratedLatticeProofs/OneSubTwoOmegaSq.lean:77`), `cmLattice_good_model_unramified` | | supersingular reduction and good models of the CM curves | frontier-f-divisibility.md Task 5 |
| `sysTw_sites` (`LocalSystemsLowerBoundsSecondRealPlace.lean:48`), `FormalGroupSystem.Pf.stmtNumE`, `main_ind` | | global ≤ local gain comparison; accumulation over the chain | frontier-f-divisibility.md Task 5 |
| the `pdv` kernel conditions, `J.four_orbit_conditions_of_proof` (`AdmissibleParameterJoins/GandSab.lean:11`), `DGamSize`; `hypotheses_nonempty_of_unramified_exponents_alt` past line 57 | | non-served primes; the upward link of `Conclusion1` to the top theorem | frontier-f-divisibility.md Task 1, Task 5 |

### Ribet-style link (page 04)

| Declaration(s) | Why it matters | Source |
|---|---|---|
| `TopBridge.twistedLatticeCohomology_theoremTOP` (`IE/TwistedLatticeCohomologyTheoremTOP.lean:67`), `TableLineDegreeTwo.lean` (`TopCore`, `PositionHyps`), `unramifiedEnlargement_cut_cocycles_blochKato`, `exists_exactLocalConditionData_at_place` | the integral local theorem behind "cut cocycles are coboundaries at ℓ" | frontier-e-ribet.md Q5 |
| `LocalGlobalSelmer.main`, `length_cocycles_le_coboundaries_add_selmer`, `length_cutCoboundaryModule_le_unitDelta`, `localCocycles_aboveEll_inCut_unitDelta`, `Cut.Peel.main`, `CutTrivIne.main`, `UnramHom.length_le_classNumber`, `inertiaCocycleBlock_length_le_of_quadraticRelation` | the ρ-block, χ-block and tame pieces of `CutCoreUnit.main` | frontier-e-ribet.md Q5; frontier-b-links.md Q6 |
| `isUnit_sub_charValues_of_cyclotomic_generator`, `exists_inertia_cyclotomic_generates_units_mod`, `SplittingAlgebra.main`, `exists_block_action_of_decomposition` | the "five objects" set-up | frontier-e-ribet.md Q5 |
| `starInequality_rowsAD_of_global_theorems` in full (`IE/StarInequalityRowsADOfGlobalTheorems.lean`), `nonPlacePart_localBound_le_middleRange`, `adm_at_inert_of_range` | which core lemma is used at medium ℓ with non-unit root differences | frontier-e-ribet.md §5, Q5 |
| `SumRepresentations.ConclusionG6`, `NormChoiceFacts.rhoBar`; `jumpLemma_a_of_admissible` | sources of `NoChiOff`, of residual irreducibility, and of the Galois content of `ConclusionJa` | frontier-e-ribet.md Q5 |
| `UpperTheory.N.at23_family_alt`, `starSF_of_prints`, `NLink.starG_of_thmB` | the ℓ = 2, 3, small ℓ and ℓ = q, d cases | frontier-b-links.md Q6; core-gm.md Q6 |
| `admittedStatements_of_localBoundsConclusion_alt`, `N.linkN_of`, `export_link_of_family` | assembly of `LinkN` | frontier-b-links.md Q6; core-gm.md Q6 |
| where `LocalFinitenessK` is used | listed in `NLink.Hypotheses₂` but not located on the lines read | frontier-b-links.md Q4 |

### Galois representations (page 05)

| Declaration(s) | Why it matters | Source |
|---|---|---|
| `UpperTheory.Gal.Spc.eigEx`, `clsMove`, `Frame` fields, `frameEx`, `vecEx_holds`, `wtOf_holds`, `expOf_holds` (`GR/ScalarCommutation/`, `PrimeLevelLocalization/PrimeProj.lean:23`, `GR/ProjectiveAutomorphicHecke/CharConjHolds.lean`) | how a definite form gives an eigenvector on the definite inner forms of the HT group; why `F.j` is surjective | frontier-c-galois.md Q7 |
| `GlobalTheory.ZM.SiegelArithFamily.nonempty_sFam`, `GlobalTheory.thetaUnifAV_closed`, `abelianSchemeLaws_all`, `pGenSm`, `pPowSm`; whether the `SiegelToroidalIntegral` import is hypothesis-free | bottom of the moduli construction; smoothness and projectivity | frontier-c-galois.md Q1, Q7 |
| `DegFin`; the ~15 `…Statement` inputs to `UpperTheory.Et.proper_base_change`; `etale_base_change_of_duality`; `Seam.crysStep_holds` | foundations of the archive's own étale cohomology and comparison theorems | frontier-c-galois.md Q1, Q7 |
| the Hodge–Tate comparison `HTSpecGood`: `CrystallineSemistableComparison.HT.hodgeTate_of_etale_side`, `Onto.htBij_of` (`Development/WeilBound/CoreStatements/PadicHodgeComparison/`, 538 files) | Faltings-style Hodge–Tate decomposition for good reduction, used to pass from Hodge to étale cohomology | frontier-g-deep-inputs.md §I.3 |
| `UpperTheory.Cb.exists_chart_lift_of_finite_cover` (`…/EtaleClopenDescent/ExistsChartLiftOfFiniteCover.lean:199`) | the core local computation of the algebraic Atiyah–Bott formula | frontier-g-deep-inputs.md §I.2 |
| `exists_heckeCorrespondence_on_integral_shimuraModel`, `exists_pIsogeny_kernel_eq_subgroupScheme`, body of `ordGen`, `esPt_holds`, `denseGen`, `frEx` | Eichler–Shimura internals | frontier-c-galois.md Q2, Q7 |
| `seedFinOdd_of`, `divAll_holds`, `gapLac_of` (`ZetaCharacterDivisibility/`) and the ~40 `P*.lean` lemmas | cutting the a-dimensional piece out of dimension dd | frontier-c-galois.md Q7; theta-chain.md Q6 |
| `congUWG_of`, `combine_holds`, `famEx_of`, `ladic_weight_at_embedding_over_place`, `wtLacEx_holds` | ℓ-adic congruences | frontier-c-galois.md Q7; theta-chain.md Q6 |
| `closed_of_cover` (body), `exists_trace_limit`, `crysDRU_of_linApprox`, `JointApprox` | closedness of the crystalline locus in the limit | frontier-c-galois.md Q7 |
| `localGlobalCompatAtIwahoriFixed_of_unlinked` (R2), `coreAll_of_core`, `toCoreK`, `twoUpOK_of_k`, `Row.galoisLocalGlobal_of_localGlobalCompatibility`, `joinUp_holds`, `joinGL_holds` | tame-case compatibility; extension from scope `.csd` to all CM fields | frontier-c-galois.md Q7; theta-chain.md Q6 |
| `tLift_of_k`, `nonVan_k`, `datum_k`, `liftAut_k`; check that `TLift_k` ≡ `TLiftK` | the stable-range non-vanishing of the theta lift | theta-chain.md Q6 |

### Window bound (page 06)

| Declaration(s) | Why it matters | Source |
|---|---|---|
| `Alg.M.main_holds` (`AV/CommutativeAlgebra/GradedModulesLength/MainCotSpan.lean:32`), `Alg.M.mprime_holds` (`Mprime.lean:38`), `Alg.lemmaP` and helpers | the graded-ring criterion "Selmer ≤ congruence depth"; top level read, judged lower risk than patching | frontier-g-deep-inputs.md §II.3; frontier-d-window.md Q6 |
| `exists_dat`, `exists_ring`, `exists_model`, `model_upper`, `vlen_le`, `twoPt_shape` (`AV/TheoremsMuLinked/GaloisRepresentationTypes/OneChar.lean`) | construction of the graded deformation ring | frontier-d-window.md Q6 |
| `UpperTheory.V.Z.J.rigIUE_alt`, `UpperTheory.V.Z.Win.RR.rigII_a_alt`, `UpperTheory.V.Th.adjoint_lifting_of_cohomology`, `rigSp4_alt` | rigidity inputs | frontier-d-window.md Q6 |
| `numberU3U_of_adm2A_imp_adm2B`, `CongruenceNumberUnitaryThree`, `numberSp4_n_alt`; where NN ≠ 0 and the log\|NN\| bound are proved | the integer annihilator's size | frontier-d-window.md Q6 |
| `UpperTheory.V.Th.sp4Main_alt` (`ConductorAtMapInjectiveCoeff.lean:131`), `main_big` | GSp4 cases II, III | frontier-d-window.md Q6 |
| `UpperTheory.V.FW.memZ` (`ToKOf.lean`), `reps`, `reflOdd`, `ambCyc`, `GaloisLibrary/StickelbergerTheorem/`; `FW.struct`, `FW.algK` | remaining bodies of the in-archive Ferrero–Washington (skeleton judged faithful) | frontier-g-deep-inputs.md §III; frontier-d-window.md Q6 |
| `quadStep`, `baseRk`, `abRk`, `partA`, `partB`, `PropA.aBound_of_isTower` | uniform tower rank (where q ≡ 3 mod 8, d ≡ 7 mod 16 enter) and the pigeonhole | frontier-d-window.md Q4, Q6 |
| `lemmaK_of_steps`, `KTheoryStability/*`, `bStarOdd` | Soulé-type bound | frontier-d-window.md Q6 |
| `prim_case`, `lev_case`, `vChar`, `ML.recip` (`OverIff.lean:297`), `kolStep`, `stickBound` components | minus part Euler-system internals | frontier-d-window.md Q6 |
| `published_hypotheses_from_patching_alt` consumers, `thmBab_of_lemmaK`, all `Count*` proofs, `X.zOne`, `X.zQuad`, `gdOfHost`, `countIV`, `countVs`, `sumIn`, `sumOut`, `thmHV`, `vwin_of_joint` | assembly and the final comparison with exp(x^ε/2) | frontier-d-window.md Q6; theta-chain.md Q6 |
| the 5 modules with "MainConj" in a declaration name | whether a main conjecture is stated somewhere | frontier-d-window.md Q2 |

## P3: classical or bookkeeping

- **Analytic**: bodies of `meanSqK`, `bad_share`, `box_of_S`, `kappaC`, `stage1_prim`, `count_a`; internals of the `CountZ*` lemmas; `landau_dichotomy`, `landau_nearest` (analytic-core.md "Not read").
- **Theta chain on the window side**: `kerSide_chainS` in full, `NoExcludedParameterOfLiftParameters` (theta-chain.md Q6).
- **Vacuity**: one unbroken chain from `Row0b.above` to a concrete bridge on the VWin path; whether `polG_of_printsH_of_rT` is on that path (vacuity-check.md Q3).
- **Statement-level layers** left by the statements reviews: `ExpAt` and the `HChar` Euler factor, `RayClassGroup` / `IsFrob`, `NineTermExact` / `ShaDual` / `GS`, `IsArithHeight` (R1 §4); `IsUWeilSplit`, `IsFockVec`, `IsEisFamily`, `ZetaValueAt`, `IsDiscrete` / `AscEmb`, `IsKTypeOf`, `Acts` / `ActsD` (R2 §3); `IsChowForm`, `IsSemiAbelian`, `IsBlowupSeq`, `WeightedAtlas`, `IsDR`, the `GoodChernWeil` predicates, a `Moduli` object for `ToroidalCompactification` (R3 §4); `conv`, `ShuffleAlgebra`, `HasChart` / `IsPolFan`, `mirabSub` (R4 §4).
- **Unreviewed named statements**: 56 `Published` constants and 24 other literature constants without a direct review row (one of them, `TwSys.Cheb`, is covered indirectly) (page [07](07-named-theorems.md)).

## Open questions

1. **Which chain-length definition is used.** `mAt := evenCeil (sqrt (x·x^{−ε/8}))` lives in a module with 0 of 38 statically reachable declarations; other instantiations (m = 2, m = 2^α) are also unreachable. `RestSize` forces x^{1/2−ε} ≤ m ≤ x^{1/2+ε/2} whatever the definition, but where m is instantiated on the compiled path is not known. (theta-chain.md Q3; core-gm.md Q1)
2. **Does the project's home-made cohomology agree with the standard one?** Three statements use a "categorical" crystalline / de Rham H¹ over nilpotent PD thickenings (R3 §2). The Galois representations use the archive's own ℓ-adic étale cohomology on Mathlib's big étale site, with proper base change proved in the archive, and Hodge cohomology defined as algebraic coherent cohomology (frontier-c-galois.md Q1; frontier-g-deep-inputs.md §I.2). Neither identification with the standard theory has been checked.
3. **Are the actual theta-lift kernels of the p-adic form `Conclusion1` needs?** `Conclusion1` is proved from a local theorem about kernels exp(−Σβ)·U; the global-to-local bridge `first_kernel_law_global_small` is the unread step. (frontier-f-divisibility.md Task 5)
4. **How strong is the modularity input?** "All type-I points are automorphic" is derived via patching (`PfromAut`) stated for all n and arbitrary residual pseudo-representations, more generally than the published residually reducible modularity theorems. Whether the `Nice` / `thick` / `stable` predicates encode the needed residual hypotheses is open. (frontier-g-deep-inputs.md §II.4)
5. **Is `PropGK` new mathematics?** It bounds Vandiver-type eigenspaces by a single nonzero integer of controlled size; the reviewer calls it not known classically in this generality. (frontier-d-window.md Q3, Q5)
6. **Can twisted Selmer groups be killed as `TwistingCharacterExistence` asserts?** For rank r ≥ 100 at every twisted ℓ, it is "the most conjecture-shaped statement" on the Ш-link strand; its auxiliary-prime proof is read only at skeleton level. (frontier-e-ribet.md Q4)
7. **What is behind `TheoremSSelected` (`LinkS`) and the diagonal term?** Its provider points to Poincaré-series estimates, point counting and mass formulas; whether the diagonal e0 bound and `BesselId`'s a ≠ 0 rest on a seesaw to a Bessel period is open. (frontier-f-divisibility.md Task 4)
8. **Does anything downstream need more than `IsRecT`, or `away.tame0`?** The consumers read use only the abelian case; this was not an exhaustive search. (vacuity-check.md Q4, Q5)
9. **Is `DualAbelianScheme` over an arbitrary base beyond the literature?** The standard references give an algebraic space in general and a scheme under extra hypotheses. (statements-review.md row 13)
10. **Hidden `True` placeholders.** The regex search misses `True` on the next line, `True`-typed fields and `fun _ => True` predicates; no second search was run. (statements-review-4.md §3)
11. **Exact dependency set.** Static reachability is an over-approximation; the compiled-term audit of what the five theorems actually use is pending in the inputs. (static-reachability.md; rhcheck/README.md)
12. **Sign and normalization conventions** (arithmetic versus geometric Artin map, Gaussian normalization, `matRep` duality, `bcParam`) were not checked. (R1 #7; R2 §3; R4 §4)
13. **con-leche.** Public reports disagree on whether it accepted or declined the export. (web-briefing.md §A)

## Sources

Every report's unread section: analysis/analytic-core.md ("Not read"), analysis/core-gm.md (Q6), analysis/frontier-a-divisor.md (Q6), analysis/frontier-b-links.md (Q6), analysis/frontier-c-galois.md (Q7), analysis/frontier-d-window.md (Q6), analysis/frontier-e-ribet.md (Q5), analysis/frontier-f-divisibility.md (Task 5), analysis/frontier-g-deep-inputs.md ("What remains unread", "What to read next"), analysis/theta-chain.md (Q6), analysis/vacuity-check.md (Q3–Q5), analysis/statements-review.md (§4), analysis/statements-review-2-reptheory.md (§3), analysis/statements-review-3-geometry.md (§4), analysis/statements-review-4.md (§3, §4); plus analysis/static-reachability.md, analysis/web-briefing.md and rhcheck/README.md for questions 11 and 13.
