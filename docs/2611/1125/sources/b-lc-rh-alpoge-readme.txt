# riemann-hypothesis

[![checks](https://github.com/leanprover-community/riemann-hypothesis/actions/workflows/checks.yml/badge.svg)](https://github.com/leanprover-community/riemann-hypothesis/actions/workflows/checks.yml)

A community-led effort to clean up and maintain a Lean formalization of the 2026 proof of the Riemann Hypothesis and of GRH for Dirichlet L-functions.

The purpose of this repo is to create an artifact that is:

- **accessible**: we want to improve build performance and modularity to enable researchers to feasibly engage with the proof even without access to massive amounts of compute.
- **readable**: we want to improve code quality while preserving the "logic" of the proof, aspirationally to Mathlib levels. This includes lemma organization, formatting and idiomatic style, eliminating linter warnings, slop-free documentation, and more.
- **integrated with the community**: we would like to add bibliographic references to prior work where appropriate, and rely on existing formalization projects when possible.

Please note: we are **not** looking for:

- complete overhauls that follow a substantially different proof path
- claims about your own machine's builds / comparator run
- adding files which are not necessary for the Riemann Hypothesis proof

This work is taking place on the `main` branch. The `original` branch preserves the first publicly-released version of the proof retrieved on 2026-11-20 from <https://alpo.ge/remark.pdf>.
If you want to open the original proof without compiling it, use
[riemann-hypothesis-original](https://github.com/homeowmorphism/riemann-hypothesis-original).
It keeps the published Lean source unchanged and provides a prebuilt cache for it.

## Theorem Statements

`src/Main.lean` proves the following theorems:

| Theorem | Statement |
|---|---|
| `riemannHypothesis` | Mathlib's `RiemannHypothesis` |
| `riemannZeta_eq_zero_re` | the same statement as `riemannHypothesis`, spelled out: every zero of ζ other than −2, −4, −6, … has Re s = 1/2 |
| `LFunction_ne_zero_of_half_lt_re` | L(χ, s) ≠ 0 for Re s > 1/2, for every modulus and every Dirichlet character, except the trivial character at s = 1 |
| `LFunction_eq_zero_re` | every zero of L(χ, s) with 0 < Re s < 1 has Re s = 1/2 |
| `LFunction_eq_zero_re_of_isPrimitive` | the special case of `LFunction_eq_zero_re` for primitive characters |

[`src/Challenge.lean`](src/Challenge.lean) states the same five with `sorry` in place of the
proofs, in a file that imports only Mathlib. Comparator checks that `Main.lean` proves exactly
those statements, that it uses no axioms other than `propext`, `Quot.sound` and
`Classical.choice`, and that Lean's kernel and nanoda both accept the proof.

## Cache

A full build takes hours, so you should use the cache:

```bash
lake exe cache get
lake build --no-build
```

A full build makes Lean check every proof on your machine. The cache downloads files that were
compiled elsewhere, so you can read and edit the project within minutes, but Lean does not
re-check them.

## Checking the proof yourself

Take a fresh clone on a Linux machine with
[bubblewrap](https://github.com/containers/bubblewrap) installed and run:

```bash
MATHLIB_CACHE_GET_URL=https://cache.mathlib.org/mathlib4 lake exe mathlib/cache get
lake comparator
```

The first command fetches Mathlib's compiled files and nothing of ours. Don't use
`lake exe cache get` here, because that also downloads our prebuilt copy of the submission. The
second reads `comparator.json`, builds `Challenge` and `Main` in a sandbox with no network, and
makes the checks described under [Theorem Statements](#theorem-statements).

Compiling the submission runs its code, so you may want to do it in a VM or your own sandbox. To do that, fetch Mathlib's cache as above, then run the check in three steps:

```bash
export LEAN_ABORT_ON_PANIC=1        # as Comparator does
export LAKE_ARTIFACT_CACHE=false    # compile everything; read nothing from a Lake cache

# in a clone where the submission has not been compiled
lake build Challenge
lake env leanexport Challenge -- $(cat export-targets.txt) > challenge.export

# wherever you build the submission
lake build Main
lake env leanexport Main -- $(cat export-targets.txt) > solution.export

# from an empty directory, with the same Lean release and comparator.json from the first clone
lake comparator --config comparator.json \
  --challenge-from-export challenge.export --solution-from-export solution.export
```

## CI

To lend a Linux machine with plenty of memory and disk, see
[CI-RUNNERS.md](verification/CI-RUNNERS.md). The CI scripts are in [`scripts/`](scripts/) and
[`tools/ci_native/`](tools/ci_native/README.md).

## License

The submission belongs to its author and was published without a license. It is mirrored here
so that it can be checked. Commit `40e56bab` is the unmodified copy.

The declarations in `comparator/` that are copied from
[formal-conjectures](https://github.com/google-deepmind/formal-conjectures) are copyright The Formal
Conjectures Authors, under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
The files that hold them carry the notice.
