# RiemannMasullo

A Lean 4 formalization of Alessandro Maria Masullo's *The Music of the Primes*, built in the
open by people and their coding agents.

**[Progress map](https://claude.ai/artifact/CtQyX8qfYAF822bgHJHHE3)**: every section of the
paper, what is stated in Lean, what is open as an issue, and what is proved, with the
dependencies between them. It is regenerated as proofs land.

## Get involved

Hundreds of proofs are open, and each one is a single Lean statement that is already written.
You need no special access and no knowledge of the whole paper. Give your coding agent (Codex,
Claude Code or any agent that can run shell commands) this instruction:

> Help formalize a mathematics paper in Lean 4. Fork and clone the repository with
> `gh repo fork mccorvie/riemann-masullo --clone`, run `lake exe cache get` in it, then read
> `AGENTS.md` and `prompts/prover.md`. Choose one open issue labelled `proof` that has no `claimed` or
> `stuck` label, comment `Claiming` on it, and replace that one `sorry` with a proof, without
> changing the statement. Run `lake build` and `scripts/lint.sh`. Then open a pull request
> against `main` whose description says `Closes #<issue>`.

Pick from the [open, unclaimed proofs](https://github.com/mccorvie/riemann-masullo/issues?q=is%3Aissue+is%3Aopen+label%3Aproof+-label%3Aclaimed+-label%3Astuck).
Each issue shows the Lean statement, where it is, and the passage of the paper it comes from.
(A fork is needed because GitHub only lets collaborators push branches to this repository;
your pull request comes from your copy.) Doing it by hand works the same way, and
[`CONTRIBUTING.md`](CONTRIBUTING.md) has the details.
A proof that is too big can be split into smaller lemmas, and finding that a statement is wrong
is a result too. Every pull request is built and checked automatically; a proof that compiles
and leaves the statement unchanged gets merged.


An attempt to autoformalize, in Lean 4 + Mathlib, the manuscript

> Alessandro Maria Masullo, *The Music of the Primes* (940 pp.)

The paper is in [`paper/`](paper/) (PDF plus a `pdftotext -layout` extraction for search/grep).

## Status

**Active: formalizing the prerequisites.** The appendices and first chapters are being stated in
Lean section by section, and each statement's proof is an open issue that anyone can take. See
the roadmap below.

**Concerns about the main theorem.** Separately from this project, concerns have been raised
about gaps in the proof of the paper's main theorem, the trace formula of Theorem 12.5.1.1. They
come from readings of the paper discussed in the Lean Zulip thread
[Formalising Masullo's proof of GRH](https://leanprover.zulipchat.com/#narrow/channel/113488-general/topic/Formalising.20Masullo.27s.20proof.20of.20GRH/with/642314625),
where the details have not been posted. They did not come from this formalization, which has not
reached that part of the paper and has not hit a roadblock. The draft is also unfinished by its
author's own account: Chapter 13, which should deduce GRH from the trace formula, does not yet
state its main theorem.

**What the formalization has found.** Small errors and gaps met along the way are logged in
[`ERRATA.md`](ERRATA.md). So far they are all in the appendices' and early chapters' accounts of
standard material, and none touches the trace formula.

The prerequisites (adeles and ideles, Haar measures, locally convex and nuclear spaces,
Schwartz–Bruhat functions) are standard mathematics worth having in Lean whatever becomes of the
main theorem, so that work continues.

## Layout

The library mirrors the paper's table of contents. Each chapter or appendix has an index file,
and its sections go in a directory of the same name (`RiemannMasullo/Ch02_AdelicTheta/Sec1.lean`):

| File | Paper |
|---|---|
| `RiemannMasullo/Ch02_AdelicTheta.lean` | Ch. 2 Adelic θ-functions |
| `RiemannMasullo/Ch03_IdelicZeta.lean` | Ch. 3 Idelic Zeta Functions |
| `RiemannMasullo/Ch04_PeriodicSchauderBases.lean` | Ch. 4 Periodic Schauder Bases on modulated groups |
| `RiemannMasullo/Ch05_ThetaZSpaces.lean` | Ch. 5 Spaces of adelic θ-functions and Idelic Z-functions |
| `RiemannMasullo/Ch06_LocalGlobal.lean` | Ch. 6 Local-Global structures |
| `RiemannMasullo/Ch07_SchauderBasesZeta.lean` | Ch. 7 Schauder Bases of Idelic Zeta functions |
| `RiemannMasullo/Ch08_PeriodicOrbits.lean` | Ch. 8 Periodic Orbits and Fixed Points |
| `RiemannMasullo/Ch09_HardySobolev.lean` | Ch. 9 Hardy-Sobolev topologies |
| `RiemannMasullo/Ch10_TraceSpectral.lean` | Ch. 10 Trace formula: Spectral Side |
| `RiemannMasullo/Ch11_ShellStructure.lean` | Ch. 11 The Shell Structure |
| `RiemannMasullo/Ch12_TraceGeometric.lean` | Ch. 12 Trace formula: Geometric Side |
| `RiemannMasullo/Ch13_ZerosHecke.lean` | Ch. 13 Zeros of Hecke L-functions |
| `RiemannMasullo/Ch14_Complements.lean` | Ch. 14 Complements |
| `RiemannMasullo/AppA_AlgebraicNT.lean` | App. A Algebraic Number Theory |
| `RiemannMasullo/AppB_AnalyticNT.lean` | App. B Analytic Number Theory |
| `RiemannMasullo/AppC_LCTVS.lean` | App. C Locally convex TVS |
| `RiemannMasullo/AppD_SchwartzBruhat.lean` | App. D Schwartz-Bruhat functions |
| `RiemannMasullo/AppE_TopologicalStacks.lean` | App. E Topological stacks |

(Chapter 1 is the informal introduction and has no file.)

Third-party formalizations we build on are vendored under
[`RiemannMasullo/External/`](RiemannMasullo/External/README.md); nothing is vendored yet.

## Roadmap

The order follows the paper's own cross-references. Two streams run in parallel and meet at the
trace formula:

| Wave | Paper | Status |
|---|---|---|
| 1 | Appendices A.1–A.3, A.5, C, D; then Chapters 2–3 (adelic θ-functions, idelic zeta functions) and, independently, Chapters 4 and 9 (periodic Schauder bases, Hardy–Sobolev spaces) | in progress: [section issues](https://github.com/mccorvie/riemann-masullo/issues?q=is%3Aissue+label%3Asection), [proof issues](https://github.com/mccorvie/riemann-masullo/issues?q=is%3Aissue+is%3Aopen+label%3Aproof+-label%3Aclaimed) |
| 2 | Chapters 5–7, with Appendix B.1–B.6 | in progress: Appendix B.1–B.3 stated; the rest seeded as section issues |
| 3 | Chapters 10–12: the trace formula | not yet seeded |
| later | Chapters 8 and 14, Appendix E, A.4, B.7 | only when something needs them |

An issue labelled `blocked` is waiting for the sections it builds on. The appendices are mostly
standard mathematics, and several pieces already exist in other Lean projects (see
[issue #3](https://github.com/mccorvie/riemann-masullo/issues/3)); the chapters are where the
paper's own argument is tested.

## Contributing

Work is organized with issues and pull requests: scouts find existing Lean, digesters turn paper
sections into statements, provers close one `sorry` at a time, and an integrator reviews and
merges. The workflow and rules are in [`AGENTS.md`](AGENTS.md), the role prompts in
[`prompts/`](prompts/README.md).

## Building

```sh
lake exe cache get   # download prebuilt Mathlib
lake build
```

Pinned to Lean `v4.36.0` / Mathlib `v4.36.0`.

## Comparator

The target statements are checked with [Comparator](https://github.com/leanprover/comparator)
(`lake comparator`, built into Lean from 4.35), following the setup for Alpoge's formalization at
<https://github.com/Deicyde/riemann-hypothesis>.

| File | What it is |
|---|---|
| [`Statements.lean`](Statements.lean) | definitions of the target statements; imports only Mathlib |
| [`Challenge.lean`](Challenge.lean) | the theorems, with `sorry` |
| [`Main.lean`](Main.lean) | the same theorems, to be proved from `RiemannMasullo` |
| [`comparator.json`](comparator.json) | the Comparator config |

| Theorem | Statement |
|---|---|
| `riemannHypothesis` | Mathlib's `RiemannHypothesis` |
| `riemannZeta_eq_zero_re`, `LFunction_ne_zero_of_half_lt_re`, `LFunction_eq_zero_re`, `LFunction_eq_zero_re_of_isPrimitive` | RH and GRH for Dirichlet L-functions, identical to the Alpoge challenge |
| `dedekindZeta_zeros_re` | GRH for the Dedekind zeta function of every number field |
| `heckeLFunction_zeros_re` | GRH for `L(s, χ)`, every number field, every unitary Hecke character |

```sh
lake comparator              # needs bubblewrap; add --paranoid for the extra kernels
```

It currently rejects with `Illegal axiom detected: 'sorryAx'`, as it should until `Main.lean` has
no `sorry`.

The paper's own main theorem (Theorem 13.3.1.2) is a placeholder in the current draft, so the last
two statements were reconstructed from the introduction and have not been reviewed.

## License

The Lean code and everything else written for this project is released under the
[Apache License 2.0](LICENSE), the license of Mathlib. The manuscript in [`paper/`](paper/README.md)
belongs to its author and is not covered by it. Code vendored under `RiemannMasullo/External/`
keeps its own license and notices.
