# rh-proof-dependencies

A study of which declarations the Lean 4 proof of the Riemann Hypothesis and GRH for Dirichlet L-functions
([remark.pdf](https://alpo.ge/remark.pdf)) uses. Sourced from the kernel proof terms of the five theorems in
`src/Main.lean` (`riemannHypothesis`, `riemannZeta_eq_zero_re`, `LFunction_ne_zero_of_half_lt_re`,
`LFunction_eq_zero_re`, `LFunction_eq_zero_re_of_isPrimitive`).

Dataset v0.2.1. The list of used declarations comes from Andrew Sutherland's kernel export of the proof. Declarations,
files and lines come from Lean's own declaration ranges in a full build of the proof.

"Used" means referenced, directly or through other constants, by the kernel proof terms. Some unused declarations may
still be needed to compile (notation, tactics, `simp` lemmas and instances that leave no trace in the term).

## Summary

| | Used by the proof | Total | % used |
|---|---|---|---|
| Project declarations | 804,912 | 1,061,400 | 75.8 |
| Project files with a used declaration | 146,287 | 175,187 | 83.5 |
| Lines of Lean source inside used declarations | 10,737,728 | 17,374,215 | 61.8 |

Kernel constants used by the proof:

| Origin | Used | Declared there | % used |
|---|---|---|---|
| Project | 1,270,184 | | |
| Mathlib | 204,488 | 569,189 | 35.9 |
| Lean core | 9,796 | 217,233 | 4.5 |
| Batteries | 258 | 10,138 | 2.5 |
| Aesop | 3 | 6,504 | 0.05 |
| **Total** | **1,484,729** | | |

Categorization of the Lean source code in the proof:

| | Lines | % |
|---|---|---|
| Inside used declarations | 10,737,728 | 61.8 |
| Inside unused declarations | 2,692,541 | 15.5 |
| Inside duplicate copies of theorems (same name and statement as a theorem in another file) | 1,294 | <0.1 |
| Outside any declaration (blank lines, `import`, `namespace`, `open`, `variable`, …) | 3,922,942 | 22.6 |
| Inside commands with no kernel constant (`example`, notation, macros, `run_cmd`) | 19,710 | 0.1 |
| **Total** | **17,374,215** | **100** |

- Theorems that repeat the statement of another used theorem account for at most 606,362 more lines (5.6%).
- Removing the hypotheses that the proofs never use frees at most 5,659 more lines (0.1%), or 27,255 (0.3%) if unused
  `let`/`have` values go too.
- The five theorems share almost everything. Each one alone uses at least 804,889 of the 804,912 used declarations.

## Results

### Kernel constants

Kernel constants are the individual definitions and theorems that Lean's kernel checks. The summary counts them by
origin. A constant is used when one of the five theorems depends on it, directly or through other constants.

- The export holds 1,484,733 constants in 1,468,418 declaration records (an inductive block is one record). The four
  extra constants (`Nat.lor`, `Nat.xor`, `String.mk`, `String.mk._proof_1`) are there only because the export also
  includes some kernel primitives.
- The export has 160,315,873 direct references between constants.
- The closure follows `lean4export`'s rules, so an inductive type comes with its whole block (constructors and
  recursors) and literals bring their types. Counting only type and value references, plus each inductive's
  constructors, the five theorems use 1,478,243 constants, or 1,478,219 for `riemannHypothesis` alone (1,265,256 of
  them the project's). The difference is exactly 6,486 recursors that no term refers to.
- The used Mathlib constants come from 5,081 of Mathlib's 8,173 modules that declare a constant.
- Loading Main's compiled environment from three independently built sets of oleans (ours, proofsandreasons.io and
  Jack McCarthy's) gives byte-identical references and declaration ranges, and the same closure, 1,484,729 constants,
  as the kernel export.

### Project declarations

A declaration is a top-level constant with its own declaration range in the compiled proof, such as a `theorem`,
`def`, `instance` or `structure` a person wrote. Lean also generates helper constants (`_proof_N`, `match_N`, `eq_N`,
`rec`, `mk`, …), and each one is counted with the declaration that produced it. Its lines are the whole source command,
including its docstring and attributes.

- 804,887 declarations are used by each of the five theorems.
- The 1,884 definitions that notation, macro and elab commands generate are needed to compile and never appear in proof
  terms. They are not counted as declarations.
- 839 unused declarations generated an auxiliary lemma that Lean shares with a used declaration
  (`data/aux-only-decls.tsv`).

### Lines

Lines are counted with `wc -l` over the 175,187 source files.

Of the lines outside any declaration, 1,752,637 are blank, 909,822 are `import`, 522,750 are
`namespace`/`section`/`end`, 467,582 are `open`, 131,653 are `variable`/`universe`/`include`/`omit`, 81,402 are
`set_option`/`attribute`/`export`, and 57,096 are other commands.

### By directory

Each column gives used out of total. The last column compares lines inside used declarations with all lines in the
directory.

| Directory | Files with a used declaration / files | Used declarations / declarations | % | Lines in used declarations / lines |
|---|---|---|---|---|
| `src/Development` | 119,001 / 141,928 | 544,156 / 692,952 | 78.5 | 7,659,708 / 12,232,663 |
| `src/Formalism` | 14,898 / 18,254 | 167,705 / 240,149 | 69.8 | 1,900,674 / 3,200,748 |
| `src/GroundTheory` | 6,558 / 8,507 | 48,530 / 75,101 | 64.6 | 600,712 / 1,119,138 |
| `src/ReciprocityTheory` | 3,450 / 3,845 | 30,735 / 36,425 | 84.4 | 394,315 / 563,016 |
| `src/CompanionTheory` | 1,881 / 2,027 | 10,561 / 11,768 | 89.7 | 143,098 / 194,218 |
| `src/PositivityTheory` | 341 / 449 | 2,981 / 4,744 | 62.8 | 32,327 / 55,130 |
| `src/Lemmata` | 155 / 167 | 231 / 248 | 93.1 | 6,871 / 9,232 |
| `src/Statements.lean` | 1 / 1 | 4 / 4 | 100.0 | 11 / 16 |
| `src/Supplements` | 1 / 3 | 4 / 4 | 100.0 | 7 / 22 |
| `src/Main.lean` | 1 / 1 | 5 / 5 | 100.0 | 5 / 12 |
| `src/BuildAid` | 0 / 5 | 0 / 0 |  | 0 / 20 |
| **Total** | **146,287 / 175,187** | **804,912 / 1,061,400** | **75.8** | **10,737,728 / 17,374,215** |

The next level of subdirectories is in `data/directories.tsv`.

### Specific declarations and directories

- `src/GroundTheory/ArithmeticGeometry`: 2,335 of 3,770 declarations used (61.9%). SchemeCohomology 1,675 / 2,089;
  AbelianVarieties 626 / 1,277; EtaleCompletion 24 / 46; ToricVarieties 7 / 115; ModularCurves 3 / 222;
  IwasawaModules 0 / 16; SelmerPairing 0 / 5.
- `src/Development/AbelianVarietyTheory`: 337,436 of 457,030 declarations used (73.8%). Its subdirectories are in
  `data/dirs-level3.tsv`.
- Mathlib `AlgebraicGeometry`: 6,820 of 10,573 constants used (64.5%), from 125 of its 139 modules.
- `GeneralHyps`, `GeneralHypU`, `GeneralHypV` (`GlobalTheory.Fc.S0.*`): not used. No constant with these names, or under
  them, is in the export.
- `UpperTheory.Top.plugAll`, `UpperTheory.Lit.everyE₂`: not used. Neither is in the export.
- Each theorem alone uses 1,484,697 to 1,484,710 constants and 804,889 to 804,899 declarations (`data/per-theorem.tsv`).
- Why a declaration is used: `tools/why_path.py why-parents.parquet NAME` prints one shortest chain from a theorem.
  For example, `Formalism.UG.P.conv_finDosetSupp` is at depth 14, through `LFunction_ne_zero_of_half_lt_re` → `allChi` →
  `UpperTheory.FinalTheorems.everyCharacter` → `everyCharacter_of_row` → `UpperTheory.One.vwin_of_row` → … →
  `Formalism.UG.Final.HeckeConvMem` → `Formalism.UG.P.hecke_convolution_mem_holds` → `Formalism.UG.P.conv_finDosetSupp`.
- `tools/structure/query.py` answers more path questions: what a declaration uses, what uses it, and what it is used
  through ([docs/query.md](docs/query.md)).

### Text-based maps compared with the kernel

These counts are declaration names, with `_private` prefixes dropped. Each map was regenerated with its authors' own
scripts on the same source, then compared with the kernel closure of the same starting theorem.

| Map | Starting theorem | Names in map | Names in kernel closure | Used, missing from map | … indexed, not reached | … not indexed | In map, not used |
|---|---|---|---|---|---|---|---|
| [cadamcat](https://github.com/cadamcat/riemann-hypothesis-exposition) `595de571` | `riemannHypothesis` | 697,937 | 801,180 | 117,946 | 115,207 | 2,739 | 14,685 |
| [jae-63](https://github.com/jae-63/rh-alpoge-blueprint) `0434143d`, no depth limit | `UpperTheory.FinalTheorems.everyCharacter` | 713,043 | 801,175 | 103,581 | 100,492 | 3,089 | 15,447 |

- Missing declarations are reached through dot notation, instances, notation and tactics. Most of the not-indexed ones
  are instances (2,241 and 2,591).
- 18 and 2 names in the maps match no declaration.
- The per-name lists are in `data/text-maps/`.
- These comparisons use the text-derived declaration names of dataset v0.1.3, since text tools can't produce the
  names Lean generates for 1,269 unnamed instances.

### Dominator tree

The graph has 886,185 nodes and 9,232,563 edges, with one root above the five theorems. The nodes are the 804,912 used
declarations, 81,106 shared auxiliary lemmas used by a declaration other than the one that generated them, and 167
project constants with no declaration row. A dominates B when every path from the root to B passes through A, so B is
used only through A. Sizes are given in nodes and in lines inside declarations.

| Root | Immediate dominator | Nodes | Lines | % of lines |
|---|---|---|---|---|
| `allChi` | (root) | 886,156 | 10,737,526 | 100.0 |
| `UpperTheory.FinalTheorems.everyCharacter` | `allChi` | 886,155 | 10,737,525 | 100.0 |
| `UpperTheory.FinalTheorems.everyCharacter_of_row` | `…everyCharacter` | 670,692 | 8,308,005 | 77.4 |
| `UpperTheory.LiteratureTheorems.every_character_of_literature_of_vWin` | `…everyCharacter_of_row` | 587,596 | 7,172,414 | 66.8 |
| `UpperTheory.Lit.every_character_of_theta_integrals_local_matching` | `…every_character_of_literature_of_vWin` | 366,417 | 4,245,419 | 39.5 |
| `UpperTheory.Lit.nonempty_hyps_definite_side_of_closed_bound` | `…every_character_of_theta_integrals_local_matching` | 305,572 | 3,576,707 | 33.3 |
| `GlobalTheory.Fc.S0.StageU.hypotheses_nonempty_of_unramified_exponents_alt` | `…nonempty_hyps_definite_side_of_closed_bound` | 102,963 | 1,217,085 | 11.3 |
| `UpperTheory.One.vwin_of_row` | `…everyCharacter_of_row` | 61,730 | 914,524 | 8.5 |
| `SelfIntersectionVolumeEstimates.topCovolSlim2_use_alt_alt` | `…every_character_of_literature_of_vWin` | 58,036 | 824,935 | 7.7 |
| `UpperTheory.Gal.Int.galoisLocalGlobalStrong_recParameter` | `…everyCharacter` | 64,176 | 689,213 | 6.4 |
| `UpperTheory.Gal.Int.Geo.geoOne_holds` | `UpperTheory.Gal.Int.galoisRepresentations_of_thetaLift` | 54,856 | 568,278 | 5.3 |

- `allChi` dominates all but 29 nodes. Declarations used by only one theorem: 0, 1, 1, 12 and 4, in the order above.
- `everyCharacter` is the immediate dominator of 53,864 nodes, which several branches share.
- `data/spine.tsv` and `data/top-subtrees.tsv` give file and line for each root and the 200 largest subtrees. The
  whole tree is the `dominators` asset.
- [Explorer](https://james-shore.github.io/rh-proof-dependencies/explorer.html): treemaps of directories and dominator
  subtrees.

### Statements labelled as published results

Owned lines become unused once those proofs are removed. Used lines are everything those proofs use. Percentages are of
the 10,737,728 lines inside used declarations.

| Label | Labelled in | Proofs | Owned lines | Used lines |
|---|---|---|---|---|
| Published result (151 statements) | [keithadler/rh-trim `docs/LITERATURE.md`](https://github.com/keithadler/rh-trim/blob/4339aac/docs/LITERATURE.md) `4339aac` | 285 | 884,308 (8.2%) | 2,961,601 (27.6%) |
| Known theorem | [jae-63/rh-alpoge-blueprint `BLUEPRINT.md` §3](https://github.com/jae-63/rh-alpoge-blueprint/blob/0434143d/BLUEPRINT.md) `0434143d` | 5 | 617,556 (5.8%) | 1,714,595 (16.0%) |
| Crux | the same | 11 | 4,959,529 (46.2%) | 7,906,144 (73.6%) |

The labels come from those repositories and haven't been checked by a mathematician.

### Duplicate statements and definitions

| | Declarations | Groups | Redundant lines (% of used lines) |
|---|---|---|---|
| Theorems with identical statements | 167,248 | 69,708 | 606,362 (5.6%) |
| Definitions identical in type and value | 6,694 | 2,599 | 10,240 (0.1%) |
| Theorems with the same statement up to renaming of constants | 179,919 | 74,542 | 677,963 (6.3%) |
| Definitions identical up to renaming of constants | 14,705 | 5,939 | 49,310 (0.5%) |
| Restating a Mathlib or core constant, exactly or up to renaming | 171 theorems, 12 definitions | | 648 |
| … of which the Mathlib or core constant is not used by the proof | 26 | | 98 |

Redundant lines are what is left after keeping the cheapest member of each group and everything its proof reaches. These
are upper bounds, since the kept member must be visible and importable where the others are used. The Mathlib rows
compare against all of Mathlib at `0df444a3` (`data/mathlib-restatements.tsv`).

### Unused hypotheses

This counts hypotheses of project `def`s and `theorem`s. A hypothesis can be removed when it is a proposition that
neither the rest of the statement nor the proof uses. Passing it on to another removable hypothesis doesn't count as a
use.

| | Count |
|---|---|
| Binders | 10,624,622 in 1,188,277 constants |
| Binders used nowhere | 121,033 (102,204 propositions) |
| Removable positions | 101,663 in 67,238 constants |
| Constants that leave the closure when every removable position is removed | 1,412 of 1,484,729 (0.1%) |
| … when unused `let`/`have` values are also removed | 4,472 (0.3%) |
| Lines inside declarations that leave the closure | 5,659 (0.1%), or 27,255 (0.3%) with unused `let`/`have` values removed |

### Universe levels

| | Largest level | Constants at that level |
|---|---|---|
| Project constants | `Type 2` (`Sort 3`) | 711 |
| All constants, as instantiated in the proof | `Type 6` (`Sort 7`) | 2, both in Lean core |

The highest levels come from Mathlib's cardinal and ordinal lemmas and from core's encoding of structural recursion.
These are the levels that occur in the elaborated terms, not a set-theoretic bound.

## Changes

| | v0.1.3 | v0.2 | v0.2.1 |
|---|---|---|---|
| Declarations | 1,061,531 | 1,061,400 | 1,061,400 |
| Used | 804,911 | 804,912 | 804,912 |
| Unused | 256,620 | 256,488 | 256,488 |
| Files with a used declaration | 146,290 | 146,292 | 146,287 |
| Lines inside used declarations | 10,737,754 | 10,737,690 | 10,737,728 |
| Redundant lines in theorems with identical statements | 606,568 | 606,437 | 606,362 |

v0.2.1 takes file and line for names declared in several modules from the copy the environment holds, the one
imported last.

## Data

The large tables are release assets of tag `v0.2.1`. [docs/dataset.md](docs/dataset.md) describes every column, and
`SHA256SUMS` has their checksums.

| Asset | Rows | Files and sizes | Contents |
|---|---|---|---|
| `constants` | 1,484,733 | `constants.csv.gz` 61.9 MB; `constants.parquet` 64.5 MB | every kernel constant in the export, with origin, module, declaration, file and line, sizes, depth and theorem mask |
| `edges` | 160,315,873 | `edges.csv.gz` 1.31 GB; `edges.parquet` 399.1 MB | direct references between constants (`type`, `value`, `lit`, `quot`) |
| `decls` | 1,063,284 | `decls.csv.gz` 23.0 MB; `decls.parquet` 23.9 MB | every declaration, with its source command, compiled range, constants, used or not, and depth |
| `directories` | 338 | `directories.csv.gz` 7 KB; `directories.parquet` 10 KB | used fraction per directory, levels 1–2 |
| `why-parents` | 1,410,923 | `why-parents.csv.gz` 38.3 MB; `why-parents.parquet` 64.3 MB | one shortest-path parent per declaration-level node, with the witnessing reference |
| `keep-list` | 804,912 | `keep-list.tsv.gz` 14.8 MB | the used declarations with their command spans and compiled ranges |
| `dominators` | 886,186 | `dominators.tsv.gz` 42.5 MB | the dominator tree, with immediate dominator and subtree sums per node |
| `universes` | 1,478,243 | `universes.tsv.gz` 10.6 MB | per constant, the largest universe level in its instantiated type and value |
| `cone-constants` | 1,484,729 | `cone-constants.txt.gz` 8.9 MB | names of the kernel constants in the five theorems' closure, one per line |
| `cone-decls` | 804,912 | `cone-decls.txt.gz` 5.8 MB | names of the used project declarations, one per line |
| `decl_edges` | 20,372,294 | `decl_edges.csv.gz` 103.3 MB; `decl_edges.parquet` 70.0 MB | the project declaration graph over the whole environment, used or not |

Small tables are in `data/`.

| File | Contents |
|---|---|
| `directories.tsv` | used fraction per directory, levels 1–2 |
| `dirs-level3.tsv` | level 3 under `src/Development/AbelianVarietyTheory` and `src/GroundTheory/ArithmeticGeometry` |
| `lines.tsv` | the line breakdown above |
| `per-theorem.tsv` | closure of each of the five theorems |
| `aux-only-decls.tsv` | the 839 declarations whose only used constant is a shared auxiliary lemma |
| `text-maps/` | text maps vs kernel, per name, with summary counts |
| `spine.tsv`, `top-subtrees.tsv`, `dir-dominators.tsv` | the dominator tree's spine, its 200 largest subtrees, and per directory |
| `statement-cone-*.tsv` | declarations and Mathlib constants that the statements, and the definitions they unfold to, mention |
| `mathlib-used.tsv.gz`, `mathlib-areas.tsv` | Mathlib constants used directly by project declarations, by constant and by area |
| `unused-hypotheses.txt` | the unused-hypothesis counts above |
| `duplicates-statements.tsv.gz`, `duplicates-definitions.tsv`, `duplicates-renaming.tsv.gz`, `duplicates-mathlib.tsv` | every duplicate group with its members, and the Mathlib or core twins the proof uses |
| `mathlib-restatements.tsv` | declarations that restate a constant anywhere in Mathlib or core, with the twin's module and whether the proof uses it |
| `literature.tsv` | each labelled statement, its proofs and what they dominate |
| `inputs/` | the labelled names taken from keithadler/rh-trim and jae-63 |
| `universes-by-sort.tsv`, `universes-witness.tsv`, `universes-pairs-ge2.tsv.gz` | constants by largest universe level; one chain to the maximum; every instantiation at `Sort 3` or above, or with a universe argument of 2 or more |
| `removable-hypotheses-*.tsv.gz`, `relevance-dropped-*.tsv` | removable positions per constant; declarations that leave the closure (`binders`; `deadlet` also drops unused `let`/`have` values) |

## Method

[docs/method.md](docs/method.md) has the details and limits.

- Parse the kernel export; take the closure of the five theorems over type and value references, literal
  dependencies and inductive blocks (the rules `lean4export` itself uses).
- Origin: a constant is Mathlib, core or a dependency when Lean 4.33.1 with Mathlib's prebuilt cache declares it;
  otherwise it is project.
- Fold each project constant into the source declaration that produced it, using a text index of the source.
  A declaration is used when one of its own constants is; auxiliary lemmas that Lean shares between declarations don't
  count for their generator.
- Dominators: Lengauer–Tarjan on the declaration graph, checked edge by edge.
- Unused hypotheses: a least fixed point over the export's proof terms, then the closure again over relevant
  references only.

## Reproduce

This needs Linux with Docker, git and curl, about 120 GB of disk and 250 GB of RAM.

```
tools/reproduce.sh ./work
OLEANS=/path/to/build/lib/lean tools/reproduce-compiled.sh ./work
```

`tools/reproduce.sh` downloads the inputs and checks their sha256, then runs every step in containers with no network.
No code from the proof runs; the export and the source are only read as data. It writes dataset v0.1.3 (file and line
from a text index) to `./work/out/v0.1.3`, the universe levels, and the analyses (`tools/analyses.sh`). A run from a
clean clone on 2026-11-21 produced all ten v0.1.3 table files byte for byte identical to the v0.1 release.

`tools/reproduce-compiled.sh` adds dataset v0.2.1 and its analyses. It needs a full build of the proof. Loading the
compiled files runs the proof's code, so every step runs in a container with no network, no capabilities and read-only
mounts. A run against our own build on 2026-11-21 produced every v0.2.1 table file (`constants`, `decls`,
`directories`, `why-parents`, `decl_edges`, the keep list, both name lists and the directory and auxiliary-only tables
in `data/`) byte for byte identical to the release.
A run of `tools/analyses.sh` on v0.2.1 on the same day reproduced the dominator, statement-cone, unused-hypothesis and
duplicate tables in `data/` and `docs/explorer.html` byte for byte, and every computed column of `data/literature.tsv`.
The decompressed `dominators.tsv.gz` is identical too (sha256 in [docs/dataset.md](docs/dataset.md)).

`tools/test.sh` runs the tests.

## Inputs

| Input | Identity |
|---|---|
| Kernel export | A. Sutherland's `solution.ndjson.zst` from [math.mit.edu/~drew/grh](https://math.mit.edu/~drew/grh/), sha256 `61a8938ad5f5e57397c1c0e3b954916af546d9004ade01c3555744a681b05381`; unpacked `5ea74b912147c5c93cc2b6d70cc727cf908df2f62ac6a20480a3fa02f6be244e`. `lean4export` v4.33.0 on Lean 4.33.1 |
| Source | [leanprover-community/riemann-hypothesis](https://github.com/leanprover-community/riemann-hypothesis) at `8b83d5e7b319ebc7b3be6e59ce3411ba2b09d116`, identical to the archive from [alpo.ge](https://alpo.ge) (sha256 `9631b4f020199e580ba86178cb56c6e9b09f690c2369a80dc4334aa93f49c7fc`) in every proof file |
| Compiled `.olean` files | Our own build of the source (Lean 4.33.1, 175,187 of 175,187 modules, `Main.olean` identical to Sutherland's), for dataset v0.2.1. Cross-checked against [proofsandreasons.io](https://proofsandreasons.io/rh-oleans.html) `alp-oleans-357309b5dd12` and [Jack McCarthy's build](https://huggingface.co/datasets/JackMcCarthy/riemann-hypothesis-build) |
| Lean, Mathlib | Lean v4.33.1 (`819816b2`), Mathlib `0df444a360eaa60ab8c11dca51a86af692955474` |
| Text maps | cadamcat/riemann-hypothesis-exposition `595de571`, jae-63/rh-alpoge-blueprint `0434143d` |
| Counts cited | nanoda's 1,484,729 from [Infatoshi/rh-verification-receipts](https://github.com/Infatoshi/rh-verification-receipts); con-leche's 1,468,418 records from the export page |

`tools/decl_index.py` adapts declaration matching from [keithadler/rh-trim](https://github.com/keithadler/rh-trim)
`tools/reach.py` (Apache-2.0).

## Coming soon

- A reduced tree that compiles, with kernel and statement checks.

## Licence

Public domain under CC0 1.0 ([LICENSE](LICENSE)), code and data alike. Three files adapted or taken from Apache-2.0
projects stay under the Apache License 2.0 ([NOTICE](NOTICE)). The proof's source is not included.
