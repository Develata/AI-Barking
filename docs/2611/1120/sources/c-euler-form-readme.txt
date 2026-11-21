# Torus Euler formalization

This directory holds two formalizations of the torus Euler blow-up paper. The
first (sections marked frozen) follows the manuscript frozen at
`torus-paper-freeze-2026-11-18`. The second, the latest-manuscript spine, follows
the current manuscript and is the one whose main theorem is `TorusEuler.theorem11`.
Both are checked by the Lean kernel relative to the statements encoded; neither is
human-reviewed.

## Latest-manuscript spine

**Target.** The arXiv version of the manuscript (`paper/craik_criminale_rewrite`,
manuscript commit `cb23bf14`) and its auxiliary note. Every numbered result R of the
main paper has a Lean statement `Paper.Statement.R` in the namespace `TorusEuler.Paper`
(for example `Paper.lemma_2_4`, `Paper.theorem_7_1`), a conditional proof
`Paper.Proof.R_of`, and a spine node `Paper.R`. The 42 nodes include the numbered
results and the displays of proofs that later proofs cite.

**Main theorem.** [`TorusEuler.theorem11`](lean/TorusEuler/MainTheorem.lean) is the
projection of `Paper.theorem_1_1`; its proof follows the TeX proof of Theorem 1.1
through the spine. [`TorusEuler.theorem11_library`](lean/TorusEuler/Theorem11.lean) is
the earlier direct proof, which follows the frozen manuscript and is retained.

**Rule.** The Lean proof of each result takes exactly the results its TeX proof
cites as hypotheses (`Paper.Proof.R_of`), and its dependency closure reaches no other
result's proof: a result is never obtained by calling the library declaration that
carries the content of another numbered result. Generic analytic facts and running-text
consequences of definitions count as glue and may come from the library. The ledger
[`spine.json`](spine.json) records, for each row, the Lean declarations, the TeX
citations, the display citations, and the setting predicates.
[`spine_gen.py`](spine_gen.py) generates the spine files, the checker
[`VerifySpine.lean`](lean/VerifySpine.lean) and the coverage ledger
[`paper-coverage-latest.md`](paper-coverage-latest.md).

**Exceptions.**

- Remark 1.2 is proved from the library theorem `auxiliaryTerminalSmoothness`
  (decision D28), not from spine nodes.
- The auxiliary note, unchanged since the freeze except for the title of the main
  paper, is not part of the spine. Its Theorem 1.1 and Lemmas 1.2 and 1.3 are formalized
  by the library theorems `auxiliaryTerminalSmoothness`, `auxiliaryLemma12` and
  `auxiliaryLemma13` (D28). These state the estimates for one construction obtained with
  the dyadic cutoff symbol and a large exponent, not for every construction with the
  conclusions of Theorem 7.1.
- Corollary 8.3 cites the periodic Beale–Kato–Majda criterion, which is not formalized.
  Its Lean proof obtains the divergence of the curl integral from the curl lower bound
  displayed in the proof of Proposition 8.2, which Proposition 8.2 exports.

**Deviations and decisions.** Departures of the ledger from the mechanically
extracted citations (glue classification, display citations, implicit citations,
proof reuse in the auxiliary note, exports of Theorem 7.1) are recorded one by one as
decisions D1-D29 in
[`ledger-decisions.md`](ledger-decisions.md).

**Checking.** From `formalization/lean`, after `lake build`, run

```sh
lake env lean -DautoImplicit=false -DrelaxedAutoImplicit=false -DwarningAsError=true VerifySpine.lean
```

It checks each node's type against its statement, its axioms (only `propext`,
`Classical.choice`, `Quot.sound`), its frontier against the ledger (the frontier equals the
TeX citations, with no bypass, except the library-proved Remark 1.2), the setting
predicates and statement purity. Whether a hypothesis is genuinely used is left to the
unused-variable linter and review. At this version it reports
42 nodes with standard axioms, and `Verify.lean` checks 14199 declarations.

## Frozen manuscript (statement and library proofs)

The Lean targets in this section follow the torus paper and terminal smoothness note
frozen at `torus-paper-freeze-2026-11-18` (manuscript `65d47fd`), whose Section 2
numbering differs from the latest manuscript.
[`Theorem11`](lean/TorusEuler/Statement.lean) states the main theorem on periodic
Euclidean lifts. [`TorusEuler.auxiliaryTerminalSmoothness`](lean/TorusEuler/AuxiliaryTerminalSmoothness.lean)
proves the stronger extension for the same Euler solution and selected
sequence, with both auxiliary estimates. The [proof blueprint](blueprint.md)
records the statement correspondence and the proof dependencies for the frozen
manuscript. The [frozen coverage ledger](paper-coverage.md) and `VerifyPaper.lean`
below also refer to the frozen manuscript.

## Build and verify

The project uses Lean
`v4.34.0-rc2` and mathlib `85e3a25e006c35636f0e53b0e9296caca2685bc0`.
The [lockfile](lean/lake-manifest.json) records the full dependency set.
From `formalization/lean`, on a host with working Lake, obtain the package
cache with `lake exe cache get`, then run

```sh
lake build
lake env lean -DautoImplicit=false -DrelaxedAutoImplicit=false -DwarningAsError=true Verify.lean
```

Lake exited with code 133 on the host used for the original checkpoint.
The pinned native arm64 toolchain runs Lake on the recovered checkout.
For hosts where Lake fails, build or obtain the locked package checkouts in
`.lake/packages`. Then `python3 check.py` checks their Git revisions, follows the imports from
`TorusEuler.lean`, compiles them in a fresh temporary directory with Lean
directly, and runs
[`Verify.lean`](lean/Verify.lean). The verifier checks the types of both
unconditional theorems and the transitive axiom dependencies of all
imported `TorusEuler` declarations.

The [paper coverage ledger](paper-coverage.md) maps the numbered results
of the frozen main paper and auxiliary note to their Lean declarations.
The check reads its TeX anchors from `frozen/torus-rewrite/src/`. Run it from
`formalization/lean` after `lake build`.

```sh
TORUS_PAPER_REQUIRE_COMPLETE=1 lake env lean \
  -DautoImplicit=false -DrelaxedAutoImplicit=false -DwarningAsError=true VerifyPaper.lean
```

This mode requires a verified theorem for every frozen assertion, including
the unconditional auxiliary theorem. It checks the source anchors and the
transitive axiom dependencies of the listed declarations.

[Draft proof attempts](drafts/README.md) are kept outside the library.

## Archive provenance

Some declarations are adapted from the CCPoint Lean project (Lean `v4.32.2`, mathlib
`81a5d257c8e410db227a6665ed08f64fea08e997`), distributed under the
[Apache 2.0 license](../sources/ccpoint-lean/LICENSE); see its
[notice](../sources/ccpoint-lean/NOTICE).
