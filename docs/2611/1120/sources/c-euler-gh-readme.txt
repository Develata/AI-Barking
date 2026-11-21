# Finite-Time Singularity Formation for the Three-Dimensional Euler Equations

Levent Alpöge, Tristan Buckmaster and Matei P. Coiculescu

This repository contains the manuscript, its auxiliary note on terminal smoothness, and a Lean 4
formalization of the main theorem.

## Manuscript

[`manuscript/paper.pdf`](manuscript/paper.pdf) is the version submitted to arXiv.
[`manuscript/source/`](manuscript/source) is the arXiv source package: the TeX, the figures, and the
auxiliary note in [`anc/auxiliary/`](manuscript/source/anc/auxiliary) (TeX and PDF).

## Lean formalization

[`formalization/`](formalization) is a Lean 4 formalization by the authors of the arXiv version of
the manuscript, in one-to-one correspondence with its proof. Every numbered result, and every
display that the proof of a later result cites, is a Lean theorem `TorusEuler.Paper.<result>`
whose proof takes exactly the results cited by its TeX proof as hypotheses and uses no other
result's proof. The ledger [`spine.json`](formalization/spine.json) records these citations, and
[`VerifySpine.lean`](formalization/lean/VerifySpine.lean) checks the correspondence;
[`paper-coverage-latest.md`](formalization/paper-coverage-latest.md) lists the results with their
Lean names. `TorusEuler.Paper.Statement.theorem_1_1` states Theorem 1.1,
`TorusEuler.Paper.theorem_1_1` proves it, and `TorusEuler.theorem11` in
[`MainTheorem.lean`](formalization/lean/TorusEuler/MainTheorem.lean) is its existence part. These
theorems depend only on the axioms `propext`, `Classical.choice` and `Quot.sound`, and the
sources contain no `sorry`. The project uses Lean `v4.34.0-rc2` and the mathlib revision fixed in
[`lake-manifest.json`](formalization/lean/lake-manifest.json); build and check instructions are in
[`formalization/README.md`](formalization/README.md).

Exceptions to the correspondence:

- The auxiliary note, unchanged since 18 November 2026 except for the title of the main paper,
  is formalized by the theorems `auxiliaryTerminalSmoothness`, `auxiliaryLemma12` and
  `auxiliaryLemma13`, which are not part of the correspondence. Remark 1.2 is
  proved from `auxiliaryTerminalSmoothness`.
- Corollary 8.3 cites the periodic Beale–Kato–Majda criterion, which is not formalized. Its Lean
  proof obtains the divergence of the curl integral from the curl lower bound in the proof of
  Proposition 8.2.

Where the ledger departs from the literal citations of the TeX (implicit citations, displays used
by later proofs, generic facts treated as library results), the decision and its justification
are listed in [`ledger-decisions.md`](formalization/ledger-decisions.md).

The formalization also contains an earlier direct proof of Theorem 1.1, `TorusEuler.theorem11_library`
in [`Theorem11.lean`](formalization/lean/TorusEuler/Theorem11.lean), which follows the frozen version
of the manuscript of 18 November 2026 (below). `VerifyPaper.lean` checks its correspondence with the
numbered results of that version ([`paper-coverage.md`](formalization/paper-coverage.md)) against a
copy of its TeX in [`formalization/frozen/`](formalization/frozen).

In this copy, the notes on the development workflow were removed from `formalization/README.md`,
and comments give the path of adapted files in [`sources/ccpoint-lean/`](sources/ccpoint-lean). Some
declarations are adapted from an earlier Lean development distributed under the Apache License 2.0;
its [license](sources/ccpoint-lean/LICENSE) and [notice](sources/ccpoint-lean/NOTICE) are included.

## Frozen manuscript

[`frozen-manuscript/`](frozen-manuscript) is the manuscript and the auxiliary note as frozen on
18 November 2026, the version that `TorusEuler.theorem11_library` follows:
[`paper.pdf`](frozen-manuscript/paper.pdf),
[`auxiliary/terminal_smoothness.pdf`](frozen-manuscript/auxiliary/terminal_smoothness.pdf) and the
TeX source. For packaging, the figure paths in the TeX point to
[`frozen-manuscript/figures/`](frozen-manuscript/figures), which holds the figures of the arXiv
package; the scripts that generate them are the same for both versions. The arXiv version differs
from the frozen one by a revision of Section 2 (scaled components, the reference angle and a
separate coupling lemma, which changes the numbering in Section 2) and by edits to the
introduction.
