# Independent verification of Alpöge's Lean formalization of RH / GRH, run on LUMI

On 20 November 2026 Levent Alpöge announced a Lean proof of the Riemann Hypothesis and of GRH for
Dirichlet L-functions, found with Claude ([remark.pdf](https://alpo.ge/remark.pdf)). The proof is theirs.
This repository records one independent check of the published archive: we compile it in a sandbox and audit, from the
compiled environment, what the main theorems state and depend on.

## Status: compile and dependency audit complete (updated 2026-11-21 12:40 UTC)

**All 175,187 modules compile, with no Lean error**, and a walk of the compiled proof terms finds that the
five theorems of `Main.lean` depend on 1,478,243 constants and on no axiom other than `propext`,
`Classical.choice` and `Quot.sound`.

This is one independent confirmation among many. Andrew Sutherland (MIT) reported the first full compile;
since then nanoda, con-leche (verified), con-ron, lean4lean and several other kernels have accepted the
proof (https://math.mit.edu/~drew/grh/ and the "Discussion: Alpöge Proves the Riemann Hypothesis" thread on
the Lean Zulip). The community copy of the source, with a build cache, is
https://github.com/leanprover-community/riemann-hypothesis . Exact used/unused counts from the kernel terms
were published independently by James Shore (https://github.com/james-shore/rh-proof-dependencies); ours
agree with his exactly once the counting rule is matched: his README reports that, counting type and value
references plus each inductive's constructors (our rule), the five theorems use 1,478,243 constants and
`riemannHypothesis` alone 1,478,219, the same numbers as here; his larger total (1,484,729) follows
`lean4export` in also including 6,486 recursors that no term refers to. Both find 5,081 Mathlib modules. His
file count (146,292 files with a used declaration) is by source declaration; ours (146,437 modules
contributing a constant) also counts compiler-generated constants. His dataset is the more complete one
(edges labelled by kind, statement cones, duplicates, universes); use it first.

| Check | Result |
|---|---|
| Archive | 14 parts from alpo.ge, concatenated sha256 `9631b4f020199e580ba86178cb56c6e9b09f690c2369a80dc4334aa93f49c7fc` |
| Static scan of all 175,187 files | no `axiom`, `sorry`, `admit` or `native_decide` in the source ([results/static-scan.txt](results/static-scan.txt)) |
| Compile all 175,187 modules (Lean v4.33.1, Mathlib tag `v4.33.1`), each `lean` process in a no-network sandbox | **done: 175,187 of 175,187, 0 failures** ([results/build-summary.json](results/build-summary.json), per-module record in [results/build-results.tsv.gz](results/build-results.tsv.gz)) |
| Cross-check against another build | `Main.olean` sha256 `9146dfb4148e24d2252e04c50742f3e49dd126dfcc5006d10bf5f873ddf796ad` and `Statements.olean` sha256 `f8e0aed3c2d69390b6dd3bb24e13aa25e043a02ca42f021e3f6e4ec8b57fd6b1`: **identical to the hashes Andrew Sutherland published** for his build |
| Axioms of the five theorems in `Main.lean`, from the compiled environment | **`propext`, `Classical.choice`, `Quot.sound` only**; 1,478,243 constants reached, none missing |
| Which modules and declarations the five theorems depend on | **146,437 of 175,187 project modules (83.6%)**; per-declaration table in [results/per-declaration-dependencies.tsv](results/per-declaration-dependencies.tsv) |
| Kernel replay (`leanchecker`) of every module | **not run by us** (started and cancelled after 6 minutes: others had done it). |
| Second, independent kernel | not run by us. Accepted by nanoda, con-leche, con-agua, con-ron, lean4lean, Pestka, nanoclo, tenet and Gruszka, per reports on the Lean Zulip |

How the build went: the first 94% (164,776 modules) took 48 minutes on 8 LUMI-C nodes. The remaining 6% are
the modules deep in the import graph. They first failed on a machine limit and then on memory, for the reasons
in the notes below, and took several restarts and about three more hours of wall time, most of it spent
finding the right settings. 718 compile attempts were killed or failed for those infrastructure reasons and
were retried; no attempt failed with a Lean error. In total the build night used about 3,500 core-hours, of which
the compile itself needed roughly 900.

### Notes for anyone else building this

**`vm.max_map_count`.** Lean maps every imported file separately, and for module-system libraries that is
five files per module (`.olean`, `.olean.server`, `.olean.private`, `.ir`, `.ir.sig`). `import Mathlib`
alone costs about 52,000 memory mappings. With the traditional Linux default of 65,530 per process, any
module with more than roughly 12,000 transitive project imports fails with
`failed to read file '….olean'` although the file is fine: 8,974 of the 175,187 modules, including `Main`.
With root, `sysctl vm.max_map_count=1048576` fixes it. Without (LUMI), [`rhshim.c`](rhshim.c) is
preloaded: it makes Lean read small files instead of mapping them (42,000 of Mathlib's files are under
4 KB) and lets Lean's own read fallback allocate. It changes where bytes are held, not what is loaded.

**Memory.** A module near the top of the tree needs about 5 to 6 GB of private memory just to import its
dependencies, against under 1 GB for a shallow one. Scheduling by core count alone runs a 256 GB node out
of memory once the build reaches the deep modules; `dbuild.py` weights modules by import-closure size and
kills and retries the largest jobs before the node does.

**Olean hashes and paths.** Some oleans embed the absolute path of their source file. The same module
compiled under two spellings of one directory gave different bytes; with Lake's spelling our output was
byte-identical to Lake's. Such modules are comparable only between builds at the same absolute path.
Modules that embed no path, `Main` and `Statements` among them, are byte-identical across machines.

**Wall time is the import chain, not cores.** The build is a few hundred CPU-hours in total; the import
graph is 203 levels deep, and the last 68 levels hold the 10,000 expensive modules.

## How much of the archive the proof uses

Measured from the compiled proof terms (`verify/Audit.lean` imports only `Lean`, loads the built
environment and follows every constant in types and values):

- `riemannHypothesis` depends on **146,437 of the 175,187 project modules (83.6%)**, 1,478,219 constants and
  5,081 Mathlib modules. 28,750 project modules are imported by `Main` but not used by the proof.
- Used / total by library: Development 119,090 / 141,928; Formalism 14,927 / 18,254; GroundTheory
  6,578 / 8,507; ReciprocityTheory 3,458 / 3,845; CompanionTheory 1,882 / 2,027; PositivityTheory 343 / 449.
- 292 declarations were walked individually (the five theorems of `Main.lean`, the 243
  `UpperTheory.LiteratureTheorems.*`, and the main internal steps): every one depends on `propext`,
  `Classical.choice` and `Quot.sound` only, and all 243 literature theorems are reached by the proof of RH.

**What the statement depends on.** Following only types and definitions (not proofs) from the statement of
`riemannHypothesis` reaches 4,981 constants: 4,183 in Mathlib, 792 in Lean core, 6 in Batteries and **none
in the project**. The other four theorems each add exactly two project constants, their own statement
definitions in `Statements.lean`. Knowing what was proved requires reading nothing the archive wrote.

**Shape.** The five theorems together depend on 1,478,243 constants joined by 116 million edges: 1,246,109
theorems and 214,238 definitions; 1,265,279 of the constants are the project's, 203,036 Mathlib's. The
longest dependency chain is 462 constants. 473,251 project theorems (44%) use no other project theorem.
The most referenced project constants are bundled-data structures (`Formalism.BasicNotions.D7`, used by
156,891 constants; the `Host` family, about 120,000 each). Usage by directory is in
[`results/usage-by-directory.tsv`](results/usage-by-directory.tsv), all figures in
[`results/graph-stats.json`](results/graph-stats.json), and the graph itself, with type and value
dependencies separated, in the
[dependency-graph-1 release](https://github.com/aapolipponen/rh-lean-check-lumi/releases/tag/dependency-graph-1).

Per-declaration counts are in [`results/per-declaration-dependencies.tsv`](results/per-declaration-dependencies.tsv).
This is a dependency count; it does not say which parts are new mathematics.

## Documentation of the proof's structure

[`docs/`](docs/README.md) is a reader's guide to the archive: what is proved and how everything reduces to one
statement, a top-down map of the argument, where the exponent 1/2 comes from, how the Galois
representations and the cohomology bound are obtained, a table of 105 named classical theorems inside the
archive with a verdict on whether each formal statement matches its name, the anatomy of the archive, and a
prioritised list of what nobody has read. It was assembled from read-only reviews of the source by AI
agents ([`docs/reports/`](docs/reports/)); every claim carries a marker saying how much was actually read.
It is not a referee report and does not vouch for the mathematics.

## What this does and does not establish

A full pass would establish that the published Lean files compile with the pinned toolchain, that the five theorems in
`Main.lean` have the stated types, that they rest on no axioms beyond Lean's standard three, and that the
Lean kernel accepts every declaration when replayed.

It would not establish that anyone understands the argument, and it trusts: the Lean 4.33.1 kernel and
toolchain binaries, Mathlib's definitions (in particular `RiemannHypothesis`, `riemannZeta` and
`DirichletCharacter.LFunction`), and the four definitions in `src/Statements.lean`, which a human should
read. Mathlib's compiled files came from the official build cache and were replayed in the kernel here.

## Method

The archive is untrusted code: Lean files can run arbitrary programs while they compile. Every Lean process
ran in a Singularity container (bubblewrap where available) with no network, a read-only view of its inputs and one private scratch
directory.

1. **Scan** (`scan.py`): import graph of all 175,187 files and a text audit. No `axiom`, `sorry`, `admit`
   or `native_decide` in the source. Import depth 203; every file is reachable from `Main`.
2. **Build** (`dbuild.py`): a coordinator hands modules to one worker per node, longest dependency chain
   first. Each module is compiled with the same `lean` invocation Lake uses (taken from `lake build -v`),
   including the lakefile's options. Outputs are written to a temporary name and renamed.
3. **Audit** (`verify/Audit.lean`): a program that imports only `Lean`, loads the compiled environment
   without running any project code, and walks every constant the five theorems depend on.
4. **Replay** (`dbuild.py --mode replay`): `leanchecker` on each module. Implemented and tested on a
   sample, but not run over the whole tree (see the table above).

Everything needed to repeat this is in this repository; see [RUNBOOK.md](RUNBOOK.md). Raw outputs
(per-module results, olean hashes, audit output, replay results, Slurm logs) are added to `results/` as
each stage finishes.

## Acknowledgements

Computations were run on the LUMI supercomputer, owned by the EuroHPC Joint Undertaking and hosted by
CSC – IT Center for Science (Finland) and the LUMI consortium. We thank CSC for the computing resources and
for their quick support.

Run by Aapo Lipponen.
