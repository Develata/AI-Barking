# 7. Named theorems: the consolidated review table

The archive states about 155 classical theorems as `Prop`s in the namespace `GlobalTheory.Published` and proves each of them inside the archive; four AI review batches have read 105 named statements to ask only whether each says what its name claims. This page merges their verdicts into one table, lists the named statements nobody has reviewed yet, and collects the notes on `True` placeholders and misnamed statements.

> **Caution on the verdicts (2026-11-21).** The verdicts below are AI readings and have not been checked against the sources. When two "faithful" rows were checked properly, against a quoted source and with every project definition read, one of them moved: Linnik's theorem is a close match, not an exact one (the archive bounds the least prime in the residue class, the classical statement the least prime in the progression past its first term). Read "faithful" as "probably, unverified". The two checked statements are in [`literature-links/`](../literature-links/).

> **Measured (2026-11-21).** Every row of the tables below was looked up in the compiled dependency graph of the five main theorems. Of the reviewed rows, 102 resolve to exactly one constant (by full name, or by a name suffix unique in the graph); one (`SiegelWeilUnitaryConvergent`) matches two constants and one (`SerreTate`) carries only a short label that could not be resolved. All 83 rows of the "not yet reviewed" tables resolve. Every resolved constant, 183 distinct declarations in all, is one the proof of RH depends on. None of the named statements is decoration. (The graph is the `dependency-graph-1` release.)

## How to read this page

A *verdict* answers one question: does the Lean statement say what its name claims? It says nothing about whether the proof is correct, and none of these statements is an assumption: each is proved in the archive (statements-review-3-geometry.md, introduction; skeleton.md §3).

| Verdict | Meaning (as used by the four reviews) |
|---|---|
| faithful | the statement is the named theorem, possibly in a stated setting (the qualifier in brackets, e.g. "tc" = only for totally complex fields, "U(p,q)", "CM") |
| special case | a genuine but narrower case of the named theorem (restricted fields, groups, coefficients or ranges) |
| weakened | asserts less than the named theorem. Every weakening found is in the "safe" direction: less is claimed, so nothing false is smuggled in |
| undetermined | the reviewer could not decide within budget, or there is no published analogue |

"Verified" in the reviews means the reviewer read the Lean definition and the project notions it uses, to the stated depth; that corresponds to **[statement read]** in this documentation. No proof was examined for any row.

The four reviews (all dated 2026-11-20, read-only on the source):

| Tag | File | Scope | Rows |
|---|---|---|---:|
| R1 | analysis/statements-review.md | `Statements.lean`, 18 `Published` statements over six areas, and the three load-bearing statements | 21 |
| R2 | analysis/statements-review-2-reptheory.md | representation theory and automorphic forms | 26 |
| R3 | analysis/statements-review-3-geometry.md | arithmetic geometry, p-adic Hodge theory, complex and Arakelov geometry, class field theory | 28 |
| R4 | analysis/statements-review-4.md | automorphic analysis, real and p-adic representation theory, CM and class fields, algebraic geometry | 30 |

The table was built by parsing the summary tables of the four files; verdicts and reasons are copied from them, with the verdict normalized to the four classes above and the reviewer's qualifier kept in brackets. Areas were assigned by the editor of this documentation for sorting only.

## Counts

| Area | faithful | special case | weakened | undetermined | total |
|---|---:|---:|---:|---:|---:|
| Analytic number theory | 2 | 3 | 0 | 0 | 5 |
| Class field theory and CM | 9 | 1 | 1 | 0 | 11 |
| Hermitian forms, algebraic groups and algebra | 8 | 0 | 0 | 0 | 8 |
| Galois cohomology and p-adic Hodge theory | 6 | 1 | 1 | 0 | 8 |
| Abelian schemes and moduli | 8 | 2 | 0 | 1 | 11 |
| Complex algebraic geometry | 6 | 2 | 2 | 1 | 11 |
| Arakelov geometry | 3 | 0 | 0 | 0 | 3 |
| p-adic and finite-field representation theory | 12 | 1 | 0 | 0 | 13 |
| Real representation theory | 7 | 2 | 0 | 0 | 9 |
| Automorphic forms | 2 | 14 | 0 | 0 | 16 |
| Theta correspondence and Siegel-Weil | 2 | 3 | 1 | 1 | 7 |
| Load-bearing (project-specific) | 0 | 0 | 2 | 1 | 3 |
| **All rows** | 65 | 29 | 7 | 4 | 105 |

The per-review counts agree with the totals each review reports: R1 faithful 11 (including "faithful for finite moduli only"), special case 5, weakened 3, undetermined 2; R2 12 / 13 / 1 / 0; R3 20 / 5 / 2 / 1; R4 22 / 6 / 1 / 1. No row in any review was judged vacuous.

**Duplicates and overlaps.**
- `JacquetShalika` (R1 #11) and `JacquetShalikaRigidity` (R2 #23) are two `Published` constants, but R2 #23 says it was "already reviewed in batch 1, row 11". Both verdicts are kept and flagged in the table. They agree (special case, totally complex fields).
- R1 #12 reviewed `DeligneRibet` "(+ `SiegelKlingen`)" in the file `SiegelKlingenDeligneRibet.lean`. The separate constant `SiegelKlingenDeligneRibet` in literature.json has no row of its own and is listed as unreviewed below.
- Overlaps of content, not of name: `IrreducibleAdmissibleGL` (R4 #8) is a conjunct of `BernsteinZelevinskyFiniteness` (R1 #10); `SiegelEisensteinContinuation` (R4 #12) is the first conjunct of `Published.DoublingMethod` (R2 #18 notes); `RegularAlgebraicPurity` (R4 #23) overlaps `ClozelPurity` and `ClozelCohomological` (R2 #3, #4); `IdealInvariantsOneOrbitSt` (R4 #30) is derived from `ComplexMultiplicationJSt` (R4 #22); `HermitianHassePrinciple` (R3 #25), `HermitianLocalGlobal` (R4 #19) and `LandherrSt` (R4 #20) are three forms of Landherr's theorem.
- Statements proved in the same file, so not external inputs: `ThetaRationalInvariance`, `HermitianLocalGlobal`, `RadfordShufflePolynomial` (R4 §3).
- `SerreTate` (R1 #14) is not among the `GlobalTheory.Published` constants of literature.json; `ResolutionCharZero` and `PrincipalisationCharZero` (R3 #14, #15) are literature.json constants in `GlobalTheory.CompanionTheory.Hypotheses₂`, not in `Published`.

## The table

Sorted by area, then by name. "R1 #11" means row 11 of the R1 summary table.

| Area | Statement | Verdict | One-line reason (from the review) | Review |
|---|---|---|---|---|
| Analytic number theory | `DeligneRibet` (+ `SiegelKlingen`) | special case | ℓ-integrality of c-smoothed partial zeta values at 1−k on narrow ray classes mod ℓ^n (no tame part of the conductor) | R1 #12 |
| Analytic number theory | `HeckeLEntireAndNonzeroAtOne` | faithful | Hecke: L(s,χ) is entire for unitary χ ≠ \|·\|^{it}, and L(1,χ) ≠ 0 for χ ≠ 1 | R3 #27 |
| Analytic number theory | `HeckeTateFunctionalEquation` | special case | CM fields, unitary algebraic Hecke characters with nonzero ∞-type; Λ entire with explicit ε (Gauss sums); `ExpAt` not traced | R1 #3 |
| Analytic number theory | `Linnik` | close match (checked; was "faithful") | ∃ C,L>0 with least prime ≡ a mod q ≤ C q^L for all coprime a, q ≥ 1 | R1 #1 |
| Analytic number theory | `PartialZetaFunctionalEquation` | special case | Hecke FE for partial zeta of wide ideal classes, totally real fields only; `IsPartialZeta` requires analytic continuation, so not vacuous | R1 #2 |
| Class field theory and CM | `ChebotarevDegreeOne` | weakened (qualitative) | Each g ∈ Gal(L/F) is the Frobenius at some unramified degree-1 prime outside any finite S. Infinitude, no density | R3 #23 |
| Class field theory and CM | `ComplexMultiplicationJSt` | faithful | H = K(j(𝔞)) is the Hilbert class field, Artin isomorphism Cl(K) ≅ Gal(H/K), and art(𝔟)·j(𝔞) = j(𝔟⁻¹𝔞) | R4 #22 |
| Class field theory and CM | `EllipticMainTheoremCM` | faithful | Shimura's main theorem of CM for analytic ℂ/𝔞 ≅ E(ℂ), stated against any `IsGlobalArtinMap` | R1 #7 |
| Class field theory and CM | `GlobalArtinReciprocity` | faithful (strong form) | Open surjection from ideles to G^ab with kernel closure(E^×·connected comp.), inertia, Frobenius, decomposition compatibilities | R1 #4 |
| Class field theory and CM | `HasseNormSt` | special case | Hasse norm theorem for quadratic extensions | R3 #24 |
| Class field theory and CM | `HeckeLAdicCharacter` | faithful | for an algebraic Hecke character (∞-type n on totally positive E^×) there is a unique continuous ℓ-adic character given by Weil's formula through a global Artin map | R4 #24 |
| Class field theory and CM | `HilbertClassFieldJ` | faithful | K(j(𝒪_K)) is the Hilbert class field, degree h_K, j(𝔞) separate classes, conjugates = all j(𝔞) | R1 #6 |
| Class field theory and CM | `IdealInvariantsOneOrbitSt` | faithful (derived in-tree from R4 #22) | j(𝔞) is algebraic over K and its Aut(ℂ/K)-orbit is {j(𝔟)} | R4 #30 |
| Class field theory and CM | `QuadraticClassFieldTheory` | faithful (quadratic) | Norm-residue symbol multiplicative, Hasse norm theorem, idelic exact sequence onto ±1, product formula, norm functoriality | R3 #26 |
| Class field theory and CM | `RayClassFieldSt` | faithful (finite moduli only) | For each nonzero integral 𝔪 an abelian L/K with Gal ≅ ray class group mod 𝔪, unramified outside 𝔪, Frobenius compatible | R1 #5 |
| Class field theory and CM | `WeberRayFieldSt` | faithful | K imaginary quadratic, disc < −4: K(j, Weber values of E[n]) is abelian and is the ray class field mod n (characterised by Frobenius splitting) | R4 #21 |
| Hermitian forms, algebraic groups and algebra | `GLHoweDuality` | faithful | (GL_n, GL_n′) and (U_n, U_n′) duality on polynomials on n′×n matrices; the pieces are irreducible and pairwise distinct | R2 #9 |
| Hermitian forms, algebraic groups and algebra | `HermitianHassePrinciple` | faithful | Landherr: nondegenerate hermitian forms for a quadratic E/F that are locally equivalent everywhere are globally equivalent | R3 #25 |
| Hermitian forms, algebraic groups and algebra | `HermitianLocalGlobal` | faithful (proved in-tree) | local classification of hermitian forms by det mod norms (char ≠ 2) plus Landherr's Hasse principle over CM fields | R4 #19 |
| Hermitian forms, algebraic groups and algebra | `InducedSimpleLowestAlcove` | faithful | char p: H⁰(λ) is simple as a GL_N-module when ⟨λ+ρ, α^∨⟩ ≤ p for all positive α (closure of the lowest alcove) | R4 #29 |
| Hermitian forms, algebraic groups and algebra | `LandherrSt` | faithful (CM) | same signatures at every τ and det ratio a global norm ⇒ isometric | R4 #20 |
| Hermitian forms, algebraic groups and algebra | `RadfordShufflePolynomial` | faithful (proved in-tree) | char 0: the shuffle algebra is the polynomial algebra on Lyndon words | R4 #28 |
| Hermitian forms, algebraic groups and algebra | `StrongApproxSU` | faithful | SU(J), J hermitian nondegenerate over CM E/F, M ≥ 2: G(F)·G_S is dense in G(𝔸) when G_S is noncompact | R2 #24 |
| Hermitian forms, algebraic groups and algebra | `WeylBranching` | faithful | GL_{n+1}(ℂ) ↓ GL_n(ℂ) and ↓ U(n): multiplicity-free, constituents = interlacing weights | R4 #16 |
| Galois cohomology and p-adic Hodge theory | `BhattScholzeCrystallineLattices` | faithful | Same theorem as `BhattScholzeIntegral`, for O_L-lattices (finite coefficient extension) | R3 #2 |
| Galois cohomology and p-adic Hodge theory | `BhattScholzeIntegral` | faithful | Crystalline ℤ_ℓ-lattices of G_K are realised by prismatic F-crystals on (O_K)_Δ (the site is encoded honestly), and the étale realisation is fully faithful | R3 #1 |
| Galois cohomology and p-adic Hodge theory | `BlochKatoLocal` | faithful | dim H¹_f formula, rational orthogonality of H¹_f under cup product, integral version. H¹_f = ker(H¹ → H¹(V⊗B_cris)) | R3 #5 |
| Galois cohomology and p-adic Hodge theory | `DeRhamPotSemistable` | faithful | de Rham ⇒ potentially semistable (Berger). B_st = B_cris[X] with Kummer-cocycle Galois action | R3 #6 |
| Galois cohomology and p-adic Hodge theory | `FontaineBcris` | faithful | Fontaine's structure theorems for the explicitly built B_cris (A_cris = p-adic completion of the PD envelope), for every admissible choice of embeddings | R3 #4 |
| Galois cohomology and p-adic Hodge theory | `HeckeLAdicCharacterCrystalline` | faithful | The ℓ-adic character of an algebraic Hecke character is crystalline at every w \| ℓ where χ is unramified | R3 #28 |
| Galois cohomology and p-adic Hodge theory | `PoitouTateNormalised` | special case | nine-term sequence, Ш duality, H^{≥3}=0; requires ℓ odd or F totally complex (the usual simplification) | R1 #18 |
| Galois cohomology and p-adic Hodge theory | `PrismaticDieudonneHodge` | weakened | Only ∃ a functor from p-divisible groups over O_C to BKF modules with a Hodge filtration and two normalisations. No equivalence, no comparison with T_pG | R3 #3 |
| Abelian schemes and moduli | `AbelianDeRham` | faithful (definitional caveat, see below) | H¹_dR locally free of rank 2g, ∧²H¹ ≅ H², Hodge filtration of rank g, base change, crystalline rigidity, polarisation pairing | R3 #9 |
| Abelian schemes and moduli | `AbelianLifting` | faithful | The LiftObj ∧ LiftHom half of `GrothendieckDeformation` (R3 #8). The name suggests more than the content: a lift must already exist | R3 #10 |
| Abelian schemes and moduli | `DualAbelianScheme` | undetermined | claims the dual exists as an abelian *scheme* over an arbitrary base S, with no projectivity or noetherian hypothesis | R1 #13 |
| Abelian schemes and moduli | `FormalAlgebraisation` | faithful | Grothendieck existence theorem: full faithfulness, and algebraisation of a formal proper scheme with an ample line bundle | R3 #21 |
| Abelian schemes and moduli | `GrothendieckDeformation` | faithful (definitional caveat, see below) | `GrothendieckMessing` (R3 #7) without "p nilpotent": the abelian-scheme form (Mazur–Messing), any nilpotent PD ideal | R3 #8 |
| Abelian schemes and moduli | `GrothendieckMessing` | faithful (definitional caveat, see below) | Lifts of abelian schemes over nilpotent PD thickenings (p nilpotent) ↔ admissible Hodge filtrations, plus lifting of homomorphisms. H¹_dR/H¹_cris is the project's "categorical cohomology" | R3 #7 |
| Abelian schemes and moduli | `SemiAbelianHomExtension` | faithful | Faltings–Chai I.2.7: homomorphisms of semi-abelian schemes over a dense open of a normal noetherian base extend uniquely | R4 #25 |
| Abelian schemes and moduli | `SemistableReductionSt` | special case | Grothendieck semistable reduction for polarised abelian varieties over p-adic fields only | R3 #11 |
| Abelian schemes and moduli | `SerreTate` (= `HomLift ∧ Lift`) | faithful | Serre–Tate deformation theorem (p nilpotent, I nilpotent), incl. lifting of a relatively ample bundle | R1 #14 |
| Abelian schemes and moduli | `SiegelToroidalIntegral` | faithful (Faltings–Chai) | for g ≥ 1, odd n ≥ 3: smooth projective toroidal compactification of A_{g,n} over ℤ[ζ_n,1/n] with semi-abelian extension, charts, ω^N globally generated, ω^{Nk}(−D) ample | R4 #13 |
| Abelian schemes and moduli | `ToroidalCompactification` | special case | Unitary PEL Shimura varieties over a CM field, characteristic 0 only: a smooth projective normal compactification with SNC boundary, semi-abelian extension and log Kodaira–Spencer iso. No integral model | R3 #12 |
| Complex algebraic geometry | `ChowFormExists` | faithful (statement level) | The Chow form of a geometrically irreducible projective variety exists and is unique up to a scalar (char 0) | R3 #17 |
| Complex algebraic geometry | `GoodChernWeil` | undetermined | Mumford-proportionality-shaped (good metrics, flag push-forward). Key predicates not traced | R3 #22 |
| Complex algebraic geometry | `KawamataViehweg` | faithful | H^q(X, K_X⊗N)=0, q>0, X smooth projective over alg. closed char-0 k, N nef and big | R1 #15 |
| Complex algebraic geometry | `L2ExtensionLogCanonical` | special case | Ohsawa–Takegoshi/Demailly L² extension with constant 34/δ, but only from a finite reduced point set (common zeros of n sections) | R3 #16 |
| Complex algebraic geometry | `NadelVanishingAndNonKltDivisor` | weakened | vanishing for *some* ideal sheaf whose support is the non-klt locus, not the multiplier ideal | R1 #16 |
| Complex algebraic geometry | `NefNumerics` | faithful | Kleiman nonnegativity; for nef N, big ⇔ N^D > 0; h⁰(mL) = L^D m^D/D! + O(m^{D−1}) for nef L | R3 #19 |
| Complex algebraic geometry | `PointSeparationPair` | weakened (mild) | Angehrn–Siu/Kollár-type criterion, but the conclusion is one-sided: some section vanishes at exactly one of x₁, x₂ (an "or"), not separation in both orders | R4 #27 |
| Complex algebraic geometry | `Principalisation` | faithful | Hironaka principalisation for a nonzero ideal on a smooth projective variety over a char-0 field, using smooth-centre blow-ups | R3 #13 |
| Complex algebraic geometry | `PrincipalisationCharZero` (used as `principalisation_char_zero`) | faithful | Principalisation on smooth, separated, quasi-compact X with an SNC boundary E carried along | R3 #15 |
| Complex algebraic geometry | `ResolutionCharZero` (used as `resolution_char_zero`) | faithful | Hironaka resolution (strong form) for reduced, separated, finite-type schemes over a char-0 field | R3 #14 |
| Complex algebraic geometry | `SerreHolomorphicSections` | special case | GAGA for H⁰ of line bundles on smooth projective complex varieties | R3 #20 |
| Arakelov geometry | `ArithHilbertSamuel` | faithful | log #{small sections of mL+N} = (a/(D+1)!) m^{D+1} + o(m^{D+1}) for nef hermitian L with self-intersection a | R1 #17 |
| Arakelov geometry | `CycleHeights` | faithful | Bost–Gillet–Soulé heights: existence (axiomatised by vertical/point/recursion clauses), the recursion formula, the projection formula | R3 #18 |
| Arakelov geometry | `VolNef` | faithful (depth 1) | arithmetic volume of a nef hermitian line bundle = its arithmetic self-intersection (with the (D+1)! normalisation) | R4 #26 |
| p-adic and finite-field representation theory | `BernsteinZelevinskyFiniteness` | faithful | irreducible smooth ⇒ admissible; Jacquet functor and parabolic induction preserve finite length; induction commutes with contragredient | R1 #10 |
| p-adic and finite-field representation theory | `BorelCasselman` | faithful | V^I → J_B(V)^{T(𝒪)} is bijective for admissible V | R2 #21 |
| p-adic and finite-field representation theory | `CuspidalMirabolic` | faithful | GL_{n+1}(𝔽_q): a cuspidal irreducible exists, and every cuspidal irreducible restricts to the mirabolic as ind_N^P ψ (Gelfand–Kazhdan/Bernstein–Zelevinsky) | R4 #3 |
| p-adic and finite-field representation theory | `DepthZeroInduction` | faithful | if the cuspidal support has depth zero, so does the representation | R2 #26 |
| p-adic and finite-field representation theory | `GenericUnitaryDualGL` | faithful (one direction) | char-0 p-adic GL_N: generic unitary irreducible ⇒ product of unitary discrete series δ_i and complementary pairs δν^{±s}, 0<s<1/2 (Tadić/Bernstein) | R4 #7 |
| p-adic and finite-field representation theory | `GreenRegularCharacters` | special case | Green's cuspidal characters of GL_d(𝔽_q), only for q = p prime | R2 #14 |
| p-adic and finite-field representation theory | `IrreducibleAdmissibleGL` | faithful | smooth irreducible ⇒ admissible for GL_n over a nonarchimedean local field (Jacquet/Bernstein) | R4 #8 |
| p-adic and finite-field representation theory | `LanglandsExhaustionGL` | faithful | every irreducible smooth rep of GL_n(k), char k = 0, has a Langlands multisegment | R2 #11 |
| p-adic and finite-field representation theory | `LanglandsMultisegments` | faithful | the multisegment is unique, and L(𝔪+𝔫) is a subquotient of L(𝔪)×L(𝔫) | R2 #12 |
| p-adic and finite-field representation theory | `LocalConverseGL` | faithful | Henniart-type n×(n−1) converse theorem for cuspidals of GL_{m+1}, char-0 local field; ε defined by JPSS functional equation at s=1/2 | R1 #9 |
| p-adic and finite-field representation theory | `MoyPrasadJacquetIso` | faithful | V^{K₁·N(𝒪)} → J_N(V)^{M∩K₁} is bijective for admissible V (depth zero, standard vertex) | R2 #13 |
| p-adic and finite-field representation theory | `StoneVonNeumann` | faithful | p-adic (char ≠ 2) Heisenberg group: smooth irreducible rep with central char ψ exists and is unique | R1 #8 |
| p-adic and finite-field representation theory | `ZelevinskyPolynomialRing` | faithful | Groth(GL) ≅ ℤ[segments] as rings, with Z(Δ) ↦ X_Δ | R2 #22 |
| Real representation theory | `CasselmanWallach` | special case | automatic extension and closed range for (𝔤,K)-maps from an irreducible **unitary** rep of U(p,q) into smooth moderate-growth Fréchet reps | R2 #20 |
| Real representation theory | `DiscreteSeriesClassification` | faithful (for U(p,q) | Harish-Chandra parametrisation by (ε, λ), infinitesimal character λ, lowest K-type λ+ρ_n−ρ_c of multiplicity one, and exhaustion | R2 #8 |
| Real representation theory | `DiscreteSeriesExists` | faithful (U(p,q) | for every HC parameter and every signature pattern a discrete series with Blattner-minimal lowest K-type exists | R4 #18 |
| Real representation theory | `DixmierMalliavinFactorisation` | faithful | for a closed linear group G ⊆ GL_n(ℂ), every f ∈ C_c^∞(G) is a finite sum g_i ∗ h_i with supp g_i ⊆ V and supp h_i ⊆ supp f; `conv` not traced | R4 #4 |
| Real representation theory | `HolomorphicDiscreteSeries` | faithful (U(p,q) | for HC parameter in the holomorphic chamber: K-types = lowest + ℕ-span of 𝔭⁺ roots, 𝔭⁻ kills the lowest K-type, K-finite vectors = U(𝔭⁺)·lowest K-type | R4 #17 |
| Real representation theory | `HoweDualityReal` | special case | only Θ(**trivial** rep of U(X)): finite length plus a unique irreducible quotient, at the archimedean places | R2 #10 |
| Real representation theory | `KFiniteVectorsUnitary` | faithful (U(p,q) | Harish-Chandra: irreducible unitary ⇒ admissible, K-finite vectors analytic, dense, 𝔤-stable and algebraically irreducible under 𝔤 | R4 #9 |
| Real representation theory | `MetaplecticRepresentation` | faithful | real Weil representation: a double cover of Sp_{2n}(ℝ), unitary on L², preserving 𝒮, intertwining Schrödinger, −1 on the kernel, Gaussian coefficient formula | R4 #10 |
| Real representation theory | `SchurOrthogonalitySquareIntegrable` | faithful | unimodular lc group, irreducible unitary with one nonzero L² coefficient ⇒ all coefficients L², orthogonality with formal degree d>0 | R4 #11 |
| Automorphic forms | `BaseChangeDefiniteUnitaryGroup` | special case | same as `BaseChangeIsobaricTransfer` (R2 #1), but the local component is the full induced product, which must be irreducible at split places; derived from it (`definiteN_of`) | R2 #2 |
| Automorphic forms | `BaseChangeIsobaricTransfer` | special case | weak base change from the *definite* U(n) (form = identity) to GL_n over CM E, with E/E⁺ unramified at every finite place; the image is an isobaric sum of csd Speh-type discrete pieces, with local compatibility at split places, unramified inert places and ∞ | R2 #1 |
| Automorphic forms | `ClozelCohomological` | special case (tc) | regular algebraic cuspidal ⇒ (𝔤,K)-cohomology ≠ 0 with some algebraic coefficient system; one direction of Clozel 3.14 | R2 #4 |
| Automorphic forms | `ClozelPurity` | faithful (tc) | algebraic cuspidal π ⇒ ∃ w₀ with infChar(τ∘c) = w₀ − infChar(τ); regularity is not even assumed | R2 #3 |
| Automorphic forms | `CuspidalMultiplicityOne` | special case (tc) | isomorphic `CuspSub`s are equal as subspaces | R2 #5 |
| Automorphic forms | `CuspidalSpectrumGL` | special case (tc) | L²_cusp is a Hilbert sum of irreducibles with finite multiplicities; every irreducible closed piece is the closure of some `CuspSub`; cusp forms are bounded | R2 #6 |
| Automorphic forms | `CuspRapidDecay` | special case (U(V), CM) | smooth cusp forms of uniform moderate growth decay faster than any power of the height on G(F)\G(𝔸) | R4 #1 |
| Automorphic forms | `CuspTraceClass` | special case (U(V), CM) | R_cusp(f) is trace class for f ∈ C_c^∞(G(𝔸)) (smooth at ∞, locally constant at finite places); trace class = finite sum of products of Hilbert–Schmidt operators | R4 #2 |
| Automorphic forms | `FlathFactorisation` | special case (U(J), CM) | every irreducible unitary rep of U(J)(𝔸) is a restricted tensor product of local irreducible unitaries; finite factors smooth-irreducible admissible, spherical line = ℂξ_v a.e. | R4 #5 |
| Automorphic forms | `FlathTensorProduct` | special case (U(J), CM) | `FlathFactorisation` (R4 #5) without the admissibility/sphericity clauses (strictly weaker than #5) | R4 #6 |
| Automorphic forms | `GelfandKazhdanDual` | special case (tc) | global form: a nondegenerate pairing between π and π∘(g ↦ ᵗg⁻¹), i.e. π̃ ≅ π^θ | R2 #25 |
| Automorphic forms | `HarishChandraFinite` | special case | Harish-Chandra finiteness (level, K∞-type, eigenvalues of the Gelfand central elements) for unitary groups over CM fields | R2 #7 |
| Automorphic forms | `JacquetShalika` | special case | strong multiplicity one for isobaric sums + JS bound q^{-1/2}<\|α\|<q^{1/2}; totally complex fields only | R1 #11 (also reviewed: R2 #23) |
| Automorphic forms | `JacquetShalikaRigidity` | special case (tc) | already reviewed in batch 1, row 11 | R2 #23 (also reviewed: R1 #11) |
| Automorphic forms | `RegularAlgebraicPurity` | special case (CM) | regular algebraic cuspidal π on GL_a over CM E: purity of the infinity type and cohomological | R4 #23 |
| Automorphic forms | `SiegelEisensteinContinuation` | faithful (unitary Siegel parabolic) | Siegel Eisenstein series on U(V⊕−V) from flat sections continue meromorphically; at every s₀ a finite-order pole with locally uniform moderate growth | R4 #12 |
| Theta correspondence and Siegel-Weil | `DoublingMethodLocalZetaIntegrals` | weakened (/ misnamed) | contains no local zeta integrals; it is the global doubling basic identity plus continuation, and the pole clause of `Published.DoublingMethod` is replaced by `True` | R2 #18 |
| Theta correspondence and Siegel-Weil | `RallisInnerProductConvergent` | special case | Rallis inner product formula (global doubling-zeta form) in Weil's convergent range, assuming earlier lifts vanish | R2 #19 |
| Theta correspondence and Siegel-Weil | `SiegelWeilUnitaryConvergent` | special case | Siegel–Weil at s₀ in Weil's convergent range rk A < rk B ≤ 2 rk A, for Fock (Gaussian-type) vectors only | R2 #17 |
| Theta correspondence and Siegel-Weil | `ThetaRationalInvariance` | undetermined (proved in-tree) | invariance of a project-built adelic theta action `WeilO.Om` under rational elements up to determinant characters; project-specific, and proved inside the archive | R4 #15 |
| Theta correspondence and Siegel-Weil | `WeilRepresentationExists` | faithful (depth 2) | Kudla-type existence of a global Weil representation of U(X)×U(W) attached to the splitting characters χ_X, χ_W | R2 #15 |
| Theta correspondence and Siegel-Weil | `WeilSiegelFormulaUnitary` | faithful | Weil's convergence criterion, then Siegel–Weil in the absolutely convergent range 2·rk A < rk B | R2 #16 |
| Theta correspondence and Siegel-Weil | `WeilThetaConvergence` | special case (unitary dual pairs) | ∫\|θ(g,Φ)\| over U(V)(F)\U(V)(𝔸) is finite when V is anisotropic or n + witt(V) < dim V | R4 #14 |
| Load-bearing (project-specific) | `IsRecT` / `RecTExists` | weakened | a "partial LLC": pinned down only on "Good" representations; elsewhere only dimension, irreducibility, twist and \|central char\| constraints | R1 #21 |
| Load-bearing (project-specific) | `UpperTheory.GalStar.Row0b recT` | weakened | Galois reps for RACSDC π on CM fields, but local–global compatibility away from ℓ only in narrow cases | R1 #20 |
| Load-bearing (project-specific) | `UpperTheory.Top.VWin` | undetermined (no published analogue) | custom bound on Σ_ℓ log ℓ · Σ_e length Ш^1(ℚ_ℓ/ℤ_ℓ(−e)) over large abelian fields; many magic constants; quick vacuity checks pass | R1 #19 |

## Caveats that apply to several rows

- **Project-made cohomology.** `GrothendieckMessing`, `GrothendieckDeformation` and `AbelianDeRham` (R3 #7–#9) are stated with the archive's own "categorical" crystalline and de Rham cohomology: first cohomology of a functor on a category of nilpotent PD thickenings. The definitions are not stubs (rank 2g is asserted), but whether they agree with Berthelot's crystalline cohomology or algebraic de Rham cohomology was not checked. The three statements are faithful in form and their content rests on that identification. (R3 §2)
- **Period-ring packages.** `BhattScholzeIntegral`, `BhattScholzeCrystallineLattices`, `BlochKatoLocal`, `DeRhamPotSemistable` and `HeckeLAdicCharacterCrystalline` (R3 #1, #2, #5, #6, #28) quantify over period-ring data with `IsStd` / `LocTied`. R3 did not find a construction and noted they would be vacuous without one. vacuity-check.md later found `GlobalTheory.MemLoc.exists_locPrime_tied` (`Development/AbelianVarietyTheory/Proofs/IwasawaDecomposition/ExistsLocalPrimeAinf.lean:281`), which proves such data exist. (R3 §2; vacuity-check.md Q3)
- **Restricted settings.** The recurring pattern is "faithful, often restricted to the special case the argument needs": CM, totally real or totally complex fields, unitary groups, characteristic 0, ℓ odd (R1 §4). In the table, 14 of the 16 rows in the automorphic-forms area are special cases, most of them restricted to totally complex or CM fields or to unitary groups.
- **`uses O`.** In literature.json, `uses_O = True` can mean only that an `_of_literature` term is assembled from other `_of_literature` terms; for example `bhatt_scholze_crystalline_lattices_of_literature O := bhattScholze_coeff (bhatt_scholze_integral_of_literature O)` (R3, introduction). 23 of the 155 `Published` constants have `uses_O = True` (literature.md; out/literature.json).

## `True` placeholders and misnamed statements

From R2 (side finding) and R4 §3, which ran one whole-tree search `grep -rn --include=*.lean -E 'Prop := True\b|: True :=|∧ True\b|→ True\b'` (81 hits).

**Placeholders that are literally `True`.**
- `GlobalTheory.Published.UnusedSlot72 : Prop := True` (`Development/AutomorphicArithmetic/ThetaCorrespondence/ThetaPairLifts/LocalTypeDictionary/DiscreteSpectrumResidualClassification.lean:7`) appears in literature.json, so the count of about 155 `Published` statements includes at least one placeholder. Harmless, but it should not be counted as a literature input. (R2)
- Eight definitions are `Prop := True`: `UnusedSlot72`; `UpperTheory.GalS.UnusedSlot55`, `UpperTheory.GalS.NoCondition`, `UpperTheory.One.M.TrivialCondition`; `TrivialRankCondition`; three copies of `trivialClassCondition`. `NoCondition` and `UnusedSlot55` are used as *conclusions*, for example `noLgc` / `noLgc_alt` "prove" `NoCondition D`. A name like `noLgc` suggests an emptied-out local–global compatibility condition. This is inside the upper theory, not in a `Published` statement, and does not affect soundness of the final theorems; but those names should not be taken at face value. `TrivialRankCondition` is used as the input of `Row.of_noE`, the strong direction. (R4 §3)
- Six theorems take `True` in place of a `Published` hypothesis (a removed slot, so less is assumed): `hasDSComp`, `copyFinDS`, `specTrace` (`Development/AbelianVarietyTheory/Proofs/DivisorProofs/HasDSComp.lean`), `archTrace` (`IrrUnitaryTraceNeZeroCopyFinDSX.lean:21`), `dm_of_two` (`DmOfHyp.lean:24`), `true_of_artin_reciprocity_of_recTExists` (`ThirdPlaceSeries/GlobalArtinReciprocityTrivial.lean:15`). (R4 §3)
- Limitation: the regex misses `Prop :=` with `True` on the next line, `True` as a structure-field type, and `fun _ => True` predicates. No second search was run. (R4 §3)

**Misnamed or thinner than the name.**
- `DoublingMethodLocalZetaIntegrals` (R2 #18): contains no local zeta integrals; it is the global doubling identity plus continuation, and the pole clause of its sibling `Published.DoublingMethod` is replaced by `True`. Weakened and misnamed.
- `PrismaticDieudonneHodge` (R3 #3): only the existence of a functor with a Hodge filtration and two normalisations; no equivalence, no comparison with T_pG.
- `AbelianLifting` (R3 #10): the Grothendieck–Messing lifting criterion, not a statement that abelian varieties lift; a lift must already exist.
- `ChebotarevDegreeOne` (R3 #23): qualitative (infinitely many degree-one primes with a given Frobenius), no density.
- `PointSeparationPair` (R4 #27): one-sided separation ("or"), not surjectivity onto both points.
- `NadelVanishingAndNonKltDivisor` (R1 #16): vanishing for *some* ideal sheaf with the right support, not the multiplier ideal.
- Outside `Published`: the file `Formalism/SharpRestriction/GlobalSchurBounds/QuadraticLogDerivCongr.lean` contains no arithmetic congruence; its namesake is a Lean `congr` lemma (frontier-b-links.md Q3). `galoisRepresentations_of_thetaLift` overstates the role of the theta lift (theta-chain.md Q1). `published_hypotheses_from_patching_alt` contains no patching (frontier-d-window.md Q1).
- Also outside `Published`: `Cut.ResidualSelmer.main` is a characteristic-0 non-degeneracy lemma, not a residual Selmer statement (frontier-e-ribet.md §1); the namespace `WeilJacquetShalikaLaw` contains no Jacquet–Shalika identity, and `positivity_condition_holds` proves Dwork's integrality lemma (frontier-f-divisibility.md Task 2).

**Named inputs that do no work.** Two literature hypotheses are taken and then discarded in a closing proof: `BlochKatoLocal` in `blochKatoTop_of_localTateDuality_eulerChar_alt` (`intro _ hdual heul …`), and `GreenbergWiles` in `residual_bound_of_parts`. This is harmless for validity, but those named inputs are not used there. (frontier-e-ribet.md §6, Q4) Together with the `True` slots above and the `uses O` caveat, this means a statement's presence in a literature bundle does not show that its content is used.

## Named statements not yet reviewed

### `GlobalTheory.Published` constants in literature.json without a review row (56)

From out/literature.json minus the 105 reviewed names. "proved in" abbreviates the assembly file: ECLV = `EveryCharacterOfLiteratureOfVWin.lean`, TPSI = `TemperedProductOfSquareIntegrable.lean`. `uses O` is the literature.json flag (see the caveat above).

| Statement (`GlobalTheory.Published.…`) | `_of_literature` theorem(s) | uses `O` | proved in | note |
|---|---|---|---|---|
| `AbMulNSt` | `ab_mul_n_st_of_literature` | no | ECLV |  |
| `AlgebraicCharacterWD` | `algebraic_character_wd_of_literature`, `algebraic_character_wd_of_open` | yes | ECLV, TPSI |  |
| `ArCalc` | `ar_calc_of_literature` | no | ECLV |  |
| `ArDegree` | `ar_degree_of_literature` | no | ECLV |  |
| `AutomorphicFormsFiniteGL` | `automorphic_forms_finite_gl_of_literature`, `automorphic_forms_finite_gl_of_open` | no | ECLV, TPSI |  |
| `BernsteinUnitaryProductIrreducible` | `bernstein_unitary_product_irreducible_of_literature`, `bernstein_unitary_product_irreducible_of_open` | no | ECLV, TPSI |  |
| `BrauerDegreeZero` | `brauer_degree_zero_of_literature`, `brauer_degree_zero_of_open` | no | ECLV, TPSI |  |
| `ChevalleyUnits` | `chevalley_units_of_literature`, `chevalley_units_of_open` | no | ECLV, TPSI |  |
| `CMCurveGoodOutsidePrimeSt` | `cm_curve_good_outside_prime_st_of_literature` | no | ECLV |  |
| `CuspBounded` | `cusp_bounded_of_literature` | no | ECLV |  |
| `CuspDecayBound` | `cusp_decay_bound_of_literature` | no | ECLV |  |
| `CuspDecaySobolev` | `cusp_decay_sobolev_of_literature` | no | ECLV |  |
| `CuspFormsBoundedGL` | `cusp_forms_bounded_gl_of_literature`, `cusp_forms_bounded_gl_of_open` | no | ECLV, TPSI |  |
| `DepthZeroJacquetInjective` | `depth_zero_jacquet_injective_of_literature` | no | ECLV |  |
| `DepthZeroSupport` | `depth_zero_support_of_literature` | no | ECLV |  |
| `DiscreteSeriesCover` | `discrete_series_cover_of_literature` | no | ECLV |  |
| `DiscreteSeriesCoverExists` | `discrete_series_cover_exists_of_literature` | no | ECLV |  |
| `DiscreteSeriesHermitianUnique` | `discrete_series_hermitian_unique_of_literature` | no | ECLV |  |
| `DiscreteSeriesLowestKType` | `discrete_series_lowest_k_type_of_literature` | no | ECLV |  |
| `DiscreteSeriesUpqExists` | `discrete_series_upq_exists_of_literature` | no | ECLV |  |
| `EssSquareIntegrableSegments` | `ess_square_integrable_segments_of_literature`, `ess_square_integrable_segments_of_open` | no | ECLV, TPSI |  |
| `FormalDegreeCompactDual` | `formal_degree_compact_dual_of_literature` | no | ECLV |  |
| `GenericUnlinkedGL` | `generic_unlinked_gl_of_literature`, `generic_unlinked_gl_of_open` | no | ECLV, TPSI |  |
| `GeometricLemma` | `bernstein_zelevinsky_geometric_of_open`, `geometric_restriction_of_literature` | no | ECLV, TPSI |  |
| `GeometricLemmaFiltration` | `geometric_restriction_filtration_of_literature` | no | ECLV |  |
| `GKGlobalisation` | `gk_globalisation_of_literature` | no | ECLV |  |
| `GreenCuspidal` | `green_cuspidal_of_literature` | no | ECLV |  |
| `GreenSeriesSt` | `green_series_st_of_literature` | no | ECLV |  |
| `GrothendieckRingPolynomial` | `grothendieck_ring_polynomial_of_literature`, `grothendieck_ring_polynomial_of_open` | no | ECLV, TPSI |  |
| `HarishChandraAnalyticity` | `harish_chandra_analyticity_of_literature` | no | ECLV |  |
| `HarishChandraModules` | `harish_chandra_modules_of_literature` | no | ECLV |  |
| `HeckeLAdicCharacterAbove` | `hecke_l_adic_character_above_of_literature`, `hecke_ladic_character_above_of_open` | yes | ECLV, TPSI |  |
| `HeckeLAdicCharacterDeRham` | `hecke_l_adic_character_de_rham_of_literature`, `hecke_ladic_character_de_rham_of_open` | no | ECLV, TPSI |  |
| `HeckeLAdicParameterAboveAlt` | `hecke_l_adic_parameter_above_of_literature` | yes | ECLV |  |
| `HeckeLPartialContinuation` | `hecke_l_partial_continuation_of_literature` | no | ECLV |  |
| `KudlaSplittingSplitUnitary` | `kudla_splitting_split_unitary_of_literature` | no | ECLV |  |
| `principalPolarisationOneSection` | `principal_polarisation_one_section_of_literature` | no | ECLV |  |
| `RayCharacterArtin` | `ray_character_artin_of_literature` | yes | ECLV |  |
| `RealMetaplecticRepresentation` | `real_metaplectic_representation_of_literature` | no | ECLV |  |
| `SegmentRepresentation` | `segment_representation_of_literature`, `segment_representation_of_open` | no | ECLV, TPSI |  |
| `SemiAbelianLieSeminegative` | `semi_abelian_lie_seminegative_of_literature` | no | ECLV |  |
| `SiegelKlingenDeligneRibet` | `siegel_klingen_deligne_ribet_of_literature`, `siegel_klingen_deligne_ribet_of_open` | yes | ECLV, TPSI | R1 #12 read `DeligneRibet` in the file `SiegelKlingenDeligneRibet.lean`; whether that covers this constant is not stated |
| `SiegelSetUnitary` | `siegel_set_unitary_of_literature` | no | ECLV |  |
| `SiegelWeilUnitarySchrodinger` | `siegel_weil_unitary_schrodinger_of_literature` | no | ECLV |  |
| `SquareIntegrableSegment` | `square_integrable_segment_of_literature`, `square_integrable_segment_of_open` | yes | ECLV, TPSI |  |
| `StandardCosocleGL` | `standard_cosocle_gl_of_literature`, `standard_cosocle_gl_of_open` | no | ECLV, TPSI |  |
| `StandardFunctorsFiniteLength` | `standard_functors_finite_length_of_literature`, `standard_functors_finite_length_of_open` | no | ECLV, TPSI |  |
| `TemperedProductOfSquareIntegrable` | `tempered_product_of_square_integrable_of_literature`, `tempered_product_of_square_integrable_of_open` | yes | ECLV, TPSI | also the name of an assembly file |
| `ToroidalForFanKS` | `toroidal_for_fan_ks_of_literature` | no | ECLV |  |
| `TotallyDisjointProductIrreducible` | `totally_disjoint_product_irreducible_of_literature`, `totally_disjoint_product_irreducible_of_open` | no | ECLV, TPSI |  |
| `UnitaryIrreducibleAdmissible` | `unitary_irreducible_admissible_of_literature` | no | ECLV |  |
| `UnlinkedProductIrreducible` | `unlinked_product_irreducible_of_literature` | yes | ECLV |  |
| `UnramifiedDoublingGL` | `unramified_doubling_gl_of_literature` | no | ECLV |  |
| `UnusedSlot72` | `siegel_eisenstein_simple_pole` | no | ECLV | defined as `True` (R2 side finding) |
| `VolGe` | `vol_ge_of_literature` | yes | ECLV |  |
| `WeilRepresentationUnitaryPair` | `weil_representation_unitary_pair_of_literature` | no | ECLV |  |

### Other statement constants in literature.json (27)

literature.json has 182 distinct statement constants; 27 are outside `GlobalTheory.Published`. Most are local and global Galois-cohomology inputs (finiteness, Euler characteristic, Tate duality, Greenberg–Wiles) and project bundles.

| Statement constant | `_of_literature` theorem(s) | uses `O` | reviewed? |
|---|---|---|---|
| `Formalism.GaloisCohomology.LocalEulerCharK` | `local_euler_char_k_of_literature`, `local_euler_char_k_of_open` | no | no |
| `Formalism.GaloisCohomology.LocalFinitenessK` | `local_finiteness_k_of_literature`, `local_finiteness_k_of_open` | no | no |
| `Formalism.GaloisCohomology.LocalTateDualityK` | `local_tate_duality_k_of_literature`, `local_tate_duality_k_of_open` | no | no |
| `GlobalTheory.CompanionTheory.Ar.GR.ZarhinStatement` | `zarhin_statement` | yes | no |
| `GlobalTheory.CompanionTheory.Ar.HT.FarguesExact` | `fargues_exact_of_literature` | no | no |
| `GlobalTheory.CompanionTheory.Ar.Pub.NefRR` | `nef_rr_of_literature` | no | no |
| `GlobalTheory.CompanionTheory.Hypotheses₂.PoincareLelongInv` | `poincare_lelong_inv_of_literature` | no | no |
| `GlobalTheory.CompanionTheory.Hypotheses₂.PrincipalisationCharZero` | `principalisation_char_zero_of_literature` | no | yes, R3 #15 |
| `GlobalTheory.CompanionTheory.Hypotheses₂.ResolutionCharZero` | `resolution_char_zero_of_literature` | no | yes, R3 #14 |
| `GlobalTheory.HeightCounting.HtCard` | `ht_card_of_literature` | no | no |
| `GlobalTheory.Mem.An.LogDerOne` | `logDerivOne` | no | no |
| `GlobalTheory.Mem.LiteralAutomorphy.JoinOut` | `join_out_of_literature` | yes | no |
| `GlobalTheory.ThK.Dbl.EisBL` | `eis_bl_of_literature` | no | no |
| `GlobalTheory.ThK.Dbl.SiegelWeilStd` | `siegel_weil_std_of_literature` | no | no |
| `GlobalTheory.TwSys.Cheb` | `cheb_of_literature` | no | indirectly: R3 #23 says `ChebotarevDegreeOne` is proved equivalent to `TwSys.Cheb` |
| `HeckeCharacterZeroLocus.SiegelWeilU.SiegelWeilUnitaryConvergent` | `siegelWeilU_unitary_convergent_of_literature` | no | no (R2 #17 reviewed the `Published` homonym only) |
| `ReciprocityTheory.Gal.GlobalEulerChar` | `global_euler_char_of_literature`, `global_euler_char_of_open` | no | no |
| `ReciprocityTheory.Gal.GlobalFiniteness` | `global_finiteness_of_literature`, `global_finiteness_of_open` | no | no |
| `ReciprocityTheory.Gal.GreenbergWiles` | `greenberg_wiles_of_literature`, `greenberg_wiles_of_open` | yes | no |
| `ReciprocityTheory.Gal.LocalEulerChar` | `localEulerChar_alt_alt`, `localEulerChar_alt` | no | no |
| `ReciprocityTheory.Gal.LocalFiniteness` | `local_finiteness_of_literature`, `local_finiteness_of_open` | no | no |
| `ReciprocityTheory.Gal.LocalTateDuality` | `local_tate_duality_of_literature`, `local_tate_duality_of_open` | no | no |
| `ReciprocityTheory.Gal.UnramifiedOrthogonality` | `unramified_orthogonality_of_literature`, `unramified_orthogonality_of_open` | no | no |
| `UpperTheory.GalS.CarUAlt` | `carU_above_l_of_literature`, `carU_local_global_of_literature` | yes | no |
| `UpperTheory.GalS.CuspUAlt` | `cusp_u_of_literature` | yes | no |
| `UpperTheory.GalS.PairUAlt` | `pair_u_of_literature` | yes | no |
| `UpperTheory.Zero.LLC.RecTExists` | `local_langlands_gl_recT_exists_of_literature`, `local_langlands_recT_exists_of_literature` | yes | yes, R1 #21 (as `IsRecT`/`RecTExists`) |

## Sources

- analysis/statements-review.md (R1), analysis/statements-review-2-reptheory.md (R2), analysis/statements-review-3-geometry.md (R3), analysis/statements-review-4.md (R4)
- out/literature.json and rhcheck/results/literature.md (the list of named statements and the `uses O` flag)
- analysis/vacuity-check.md (Q3, existence of `LocTied` data)
- analysis/frontier-b-links.md (Q3), analysis/theta-chain.md (Q1), analysis/frontier-d-window.md (Q1), analysis/frontier-e-ribet.md (§1, §6), analysis/frontier-f-divisibility.md (Task 2), for the misnamed items outside `Published` and the discarded hypotheses
