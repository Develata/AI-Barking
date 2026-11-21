# A reader's guide to the Alpöge–Claude Lean archive for RH and Dirichlet GRH

These documents describe a 175,187-file Lean 4 archive, published by Levent Alpöge on 2026-11-20, whose five final theorems state the Riemann Hypothesis and the Generalized Riemann Hypothesis for Dirichlet L-functions. They say how the archive is organised, what it rests on, what its intermediate statements say according to AI reviewers who read the source, and which parts nobody has read; they are not a referee report and do not vouch for the mathematics.

> ## Status and limits (read this first)
>
> **Established by machine** (as of the evening of 2026-11-20 UTC; see [09](09-building-and-checking.md)):
> - The archive compiles with Lean v4.33.1 and Mathlib v4.33.1. Reported by Andrew Sutherland (full compile, 16:43 UTC); Deicyde's CI found the source identical to the published tarball and its challenge file compiling against Mathlib; the independent LUMI run compiled all 175,187 modules with no Lean errors (finished 22:21 UTC), and its `Main.olean` and `Statements.olean` have the same SHA-256 hashes Sutherland published.
> - The five theorems in `Main.lean` use only Lean's three standard axioms (`propext`, `Classical.choice`, `Quot.sound`). Reported by Sutherland. No `sorry`, `axiom`, `admit` or `native_decide` occurs in the source (static scans by the independent run and by Deicyde).
> - The Lean kernel accepts every module on replay (`leanchecker`, Sutherland), and the independent kernel nanoda accepts the export (Sutherland). con-leche, a formally verified kernel, accepts it in its current version (Szymon Teżewski, Lean Zulip); an older bundled version had declined over an unsupported inductive type, which was not a rejection.
>
> **A reading of source by AI reviewers.** Everything in these documents about *content* (what a statement says, how a proof works, which classical theorem it resembles, how strong an input is) comes from read-only reviews of the Lean source by AI agents. Every such claim carries a status marker or a pointer to the report it comes from. These readings can be wrong. They have not been checked by a human specialist.
>
> **Not read by anyone, as far as these reports know.** Most of the argument below the top levels. In particular: the bridge from the automorphic theta-lift kernels to p-adic power series behind the key divisibility statement; the Taylor–Wiles patching theorem and linking argument behind the modularity input of the window bound; the proof that twisted Selmer groups can be made nearly trivial; the local terms of the trace comparison for the Galois representations. In addition, 56 of the 155 named `Published` statements have not been reviewed even for whether they state what their names claim. The full list is [10-open-frontier.md](10-open-frontier.md). No human-readable proof exists beyond a four-page announcement.
>
> **The proof is Alpöge's and Claude's.** Nothing here should be read as saying that the mathematics has been verified by a person, or that the authors of these documents vouch for the argument.

## What this is, and who wrote it

On 2026-11-20 Levent Alpöge (Anthropic) published a Lean 4 archive (175,187 files, about 17.4 million lines, Lean v4.33.1 and Mathlib tag v4.33.1) with five theorems in `Main.lean`: the Riemann Hypothesis in Mathlib's formulation and GRH for Dirichlet L-functions. He credits Claude with much of the argument. The only prose is a four-page announcement, https://alpo.ge/remark.pdf.

These documents are by an independent checker, Aapo Lipponen, who built the archive on the LUMI supercomputer. They were edited from a set of review reports (listed below) produced by AI agents that read the archive's source read-only, plus the build records and a dated web briefing. They describe the artifact. Other people's verification results are credited on page [09](09-building-and-checking.md): Andrew Sutherland (compile, axiom audit, kernel replay, nanoda), Szymon Teżewski (con-leche), Deicyde (CI checks and static scan), and the rohanarun replay campaign.

## How to read these documents

- **A number theorist who knows no Lean:** read [01](01-what-is-proved.md) (what is proved and the reduction to one statement), then [02](02-proof-map.md) (the whole argument on one page), then the strand pages [03](03-analytic-endgame.md) to [06](06-window-bound.md) in any order, then [10](10-open-frontier.md). Each page defines its technical terms in a table near the top.
- **A Lean user who knows little number theory:** read [01](01-what-is-proved.md), [08](08-archive-anatomy.md) (size, style, reachability) and [09](09-building-and-checking.md) (building and checking), then [07](07-named-theorems.md) (are the named classical theorems stated faithfully?). The [glossary](glossary.md) explains the archive's internal names.
- **Status markers.** Every claim about content carries one of: **[statement read]** (the Lean statement was read), **[proof outline read]** (the proof term or its `have`/`obtain` skeleton was read), **[proof read in full]**, **[inferred from names]** (from declaration names, imports or the shape of a statement), **[unread]**; or a pointer such as "(core-gm.md Q2)". These are never stronger than what the source report says. Where a report says "impression", these documents say so too.

## The argument on one screen

```
 RH, Dirichlet GRH (Main.lean)
   ▲ functional equations, primitive characters: one page              [01]
 EveryCharacter: L(s,χ) ≠ 0 for Re s > 1/2
   ▲
 ZeroFreeAll ◄── localized Landau theorem: one-sided bound on windows ⇒ zero-free  [03]
   ▲
 Gm(x) ≤ x^{1/2+9ε} on windows   (core_gm)                              [04]
   ▲ m ≈ √x "lifts"; error x/m + m balanced at m ≈ √x ⇒ exponent 1/2
   ├── good auxiliary primes q, d: zero density + mean square (classical)  [03]
   ├── slope inequality (Bost) for Hermitian lattices; near-orthogonality (LinkS)
   ├── divisibility of the lift chain at inert p (Conclusion1):
   │     Dwork-type analysis of theta kernels at supersingular CM points   [04]
   └── each lift's congruence module ≤ Selmer + class number + Σ len Ш¹(ℚ_ℓ/ℤ_ℓ(−e))
         ▲ Fitting-ideal / GMA argument; twisted Selmer groups made small   [04]
         ▲
       VWin: Σ_ℓ log ℓ Σ_{e<x^401} len Ш¹ ≤ exp(x^ε/2) for abelian CM fields  [06]
         minus part: Euler systems │ plus part: U(3) congruences + modularity
         ℓ = 2 and bad ℓ: Ferrero–Washington, pigeonhole, Soulé-type bound
         ▲
       Row0b: Galois representations for cuspidal π on GL_a over CM fields  [05]
         Kuga–Sato cohomology of a Harris–Taylor-type Shimura variety,
         ℓ-adic congruences, pseudo-characters; theta lifts for even rank
```

## Contents

| Page | What it covers |
|---|---|
| [01-what-is-proved.md](01-what-is-proved.md) | the five theorems, the four definitions in `Statements.lean`, the reduction to `UpperTheory.EveryCharacter`, the three inputs |
| [02-proof-map.md](02-proof-map.md) | the whole argument top down, one row per step, with declarations, files, classical analogues and status |
| [03-analytic-endgame.md](03-analytic-endgame.md) | good prime pairs, `winPairs`, the function Gm, the localized Landau theorem; constants 401, ε < 1/52, residues mod 8 and 16, la = 3/2 |
| [04-chain-and-one-half.md](04-chain-and-one-half.md) | `core_gm`, the chain of m ≈ √x lifts, the divisor inequality, the divisibility `Conclusion1`, why 1/2 appears, how Ш enters each lift |
| [05-galois-representations.md](05-galois-representations.md) | `Row0b`, its weak local–global compatibility, and its construction |
| [06-window-bound.md](06-window-bound.md) | `VWin`: statement, non-vacuity, and its proof by a three-way split |
| [07-named-theorems.md](07-named-theorems.md) | consolidated verdicts on 105 named statements, the unreviewed list, `True` placeholders and misnamed statements |
| [08-archive-anatomy.md](08-archive-anatomy.md) | size, libraries, file shapes, style statistics, duplicates, absence of comments, static reachability |
| [09-building-and-checking.md](09-building-and-checking.md) | how to build, the memory-mapping limit, memory per module, olean path dependence, the independent run, public verification status |
| [10-open-frontier.md](10-open-frontier.md) | prioritised list of unread declarations by strand, and open questions |
| [glossary.md](glossary.md) | the archive's internal names |

## Corrections to earlier reports

Readers may have seen earlier versions of these claims. Each correction is repeated where it applies.

| # | Earlier claim | Correction | Where |
|---|---|---|---|
| 1 | The Galois representations are "built from theta lifts" (skeleton.md) | They come from étale cohomology of a Harris–Taylor-type Shimura variety, congruences and pseudo-characters; the theta lift only reduces even rank to odd rank (theta-chain.md; frontier-c-galois.md) | [01](01-what-is-proved.md), [05](05-galois-representations.md) |
| 2 | The auxiliary-prime existence proof `pair_share` might carry RH-strength content (vacuity-check.md) | It is classical: zero density plus mean square. The weight is in `core_gm` / `CoreAtH` and what feeds them (analytic-core.md) | [03](03-analytic-endgame.md) |
| 3 | `QuadraticLogDerivCongr.lean` might hold an arithmetic congruence or bootstrap (core-gm.md) | It contains no arithmetic congruence; its namesake is a Lean `congr` lemma (frontier-b-links.md) | [04](04-chain-and-one-half.md) |
| 4 | `CoreAtH` is where the Ш bound must enter (analytic-core.md, inferred) | Ш does not appear in `core_gm` or `CoreAtH`; it enters where `ChainTied` is built (core-gm.md) | [03](03-analytic-endgame.md) |
| 5 | Where the divisibility `Conclusion1` is proved was not found (frontier-a-divisor.md) | It is proved, via a local p-adic theorem about theta kernels (frontier-f-divisibility.md) | [04](04-chain-and-one-half.md) |
| 6 | The congruence-to-Ш step is "a Ribet-style lattice argument" (frontier-b-links.md) | Its core inequality is a Fitting-ideal argument on a generalized matrix algebra, not Ribet's lattice construction (frontier-e-ribet.md) | [04](04-chain-and-one-half.md) |
| 7 | The trace-comparison step for the Galois representations is unread (frontier-c-galois.md) | It was read at the bijection level: a simple trace formula over ℂ with an algebraic Lefschetz formula (frontier-g-deep-inputs.md) | [05](05-galois-representations.md) |
| 8 | No construction of `LocTied` period data was found (statements-review-3-geometry.md) | `exists_locPrime_tied` proves it exists (vacuity-check.md) | [05](05-galois-representations.md), [07](07-named-theorems.md) |
| 9 | Whether window pairs exist was not verified (statements-review.md) | `QH.pairEx` proves they do (vacuity-check.md) | [03](03-analytic-endgame.md), [06](06-window-bound.md) |
| 10 | The window bound's names suggest patching (theta-chain.md) | `published_hypotheses_from_patching_alt` contains none (frontier-d-window.md); genuine Taylor–Wiles patching does occur, in `PfromAut` (frontier-g-deep-inputs.md) | [06](06-window-bound.md) |
| 11 | "Theorem S" | Two different statements carry the name: `TheoremSSelected` (feeds `LinkS`) and `ThmSUniform` (theta kernels) (frontier-f-divisibility.md) | [04](04-chain-and-one-half.md) |

## Reports this set was built from

All dated 2026-11-20 and produced read-only, without compiling or running archive code. Paths are relative to the working directory of the checking project.

| Report | Subject |
|---|---|
| rhcheck/results/skeleton.md | top-level skeleton (with correction 1) |
| rhcheck/results/structure.md | import-graph structure |
| rhcheck/results/literature.md, out/literature.json | the named literature statements |
| analysis/archive-stats.md | statistics |
| analysis/static-reachability.md | name-level reachability |
| analysis/statements-review.md, statements-review-2-reptheory.md, statements-review-3-geometry.md, statements-review-4.md | verdicts on 105 named statements |
| analysis/vacuity-check.md | non-vacuity of the three inputs |
| analysis/analytic-core.md | good pairs and the Landau-window deduction |
| analysis/core-gm.md | `core_gm`, `CoreAtH`, `ChainTied` |
| analysis/theta-chain.md | the theta-lift strand behind `Row0b` |
| analysis/frontier-a-divisor.md | the divisor inequality |
| analysis/frontier-b-links.md | the `Link*` inputs |
| analysis/frontier-c-galois.md | the geometric source of the Galois representations |
| analysis/frontier-d-window.md | the proof of `VWin` |
| analysis/frontier-e-ribet.md | the congruence inequality and its inputs |
| analysis/frontier-f-divisibility.md | where `Conclusion1` is proved |
| analysis/frontier-g-deep-inputs.md | the trace comparison, residually Eisenstein modularity, Ferrero–Washington |
| analysis/web-briefing.md | public verification status, about 21:15 UTC |
| rhcheck/README.md, rhcheck/RUNBOOK.md | the independent build on LUMI |

## Sources

All the reports listed above.

## Note added after assembly

- **con-leche.** The two reports are not in conflict. The status page at math.mit.edu/~drew/grh/ (last updated 19:45 UTC on 2026-11-20) recorded that the con-leche bundled with Lean 4.36 declined the export because of an unsupported reflexive inductive type. On the Lean Zulip, Joachim Breitner noted that this restriction was dropped in September 2026 and the bundled copy was outdated; Szymon Teżewski then reported (about 20:40 to 20:58 UTC) that the current con-leche accepts the proof in verified mode. No con-ron or lean4lean verdict and no complete Comparator run had been reported by 21:42 UTC.
- **Other builders.** Simon DeDeo reported a successful compile on the Lean Zulip at about 18:41 UTC and shared compiled files. Andrew Sutherland's compile, axiom audit, per-module `leanchecker` replay and nanoda pass are described in 09-building-and-checking.md.
- **The independent LUMI build** described here finished with all 175,187 modules compiled and no Lean error; see the repository README.
