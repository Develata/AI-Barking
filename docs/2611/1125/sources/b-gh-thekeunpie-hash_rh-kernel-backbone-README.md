# Kernel-Grounded Minimal Backbone Specification (`leanprover-community/riemann-hypothesis`)

> **Web edition and upstream tracker** (independent; not an official upstream site): [riemannhypothesis.dev](https://riemannhypothesis.dev/) (GitHub Pages: `thekeunpie-hash/rh-kernel-backbone`)  
> **Full Specification**: [SPEC.md](SPEC.md) · **Live Tracker JSON (Hourly CI)**: [data/upstream_frontier.json](data/upstream_frontier.json) · **Machine-Readable Kernel Summary**: [data/kernel_backbone_summary.json](data/kernel_backbone_summary.json) · **Citation Auditor**: [scripts/audit_spec_citations.py](scripts/audit_spec_citations.py) · **NDJSON Statement Auditor**: [scripts/audit_ndjson_statement.py](scripts/audit_ndjson_statement.py)

A compact, kernel-audited technical map **and auto-updating hourly upstream progress tracker** for Levent Alpöge's formalization of the Riemann Hypothesis and Dirichlet $L$-function GRH ([`leanprover-community/riemann-hypothesis`](https://github.com/leanprover-community/riemann-hypothesis), baseline `v0.2.1` on Lean `v4.33.1`; since 2026-11-25 upstream `main` is based on the shorter `lsernau/grh-lean@72fead10` proof, with the original tree kept on the `alpoge` branch), grounded directly in Andrew Sutherland and James Shore's `v0.2.1` Lean kernel export (`cone-constants.txt.gz`, `decls.csv.gz`, `dominators.tsv.gz`, `why-parents.csv.gz`, `directories.csv.gz`).

## Epistemic Protocol

Every substantive claim in [SPEC.md](SPEC.md) is fenced with one of three explicit labels:

- **`[Exact]`** — Mechanically computed from the `v0.2.1` Lean kernel export or directly verified by Lean `v4.33.1` (`Kernel.isDefEq`, `Meta.isDefEq`, `collectAxioms`, `-DwarningAsError=true`).
- **`[Verified]`** — Read directly from the pinned Lean `v4.33.1` source and confirmed `in_cone = true` in `decls.csv.gz`.
- **`[Reading]`** — Informal mathematical synthesis connecting verified Lean signatures to number theory and automorphic forms.

Every active qualified Lean declaration cited outside the fenced Dead-Code Self-Audit in Section 7 of [SPEC.md](SPEC.md) is mechanically checked against `decls.csv.gz` and `cone-constants.txt.gz` with **0 dead citations**.

## Key Kernel-Export Findings `[Exact]`

| Metric | Value | Notes |
| :--- | ---: | :--- |
| Upstream source tree | 175,187 files · 17,374,215 raw lines | 1,063,284 parsed declarations (13,432,480 decl lines) |
| Active Lean kernel cone (`in_cone = true`) | **146,287 files · 10,737,728 decl lines** | **804,912 active decls** (75.7% of decls, 61.8% of raw lines); `1,484,729` kernel constants |
| Wholly dead files (`used_decls = 0`) | 28,900 files · 1,754,237 decl lines | Plus 940,515 dead decl lines inside mixed active files |
| Shared across all 5 `Main.lean` theorems | **804,887 decls · 10,737,528 decl lines** | **99.997% of the cone**, routed through one root chokepoint: `allChi` (`src/Supplements/Kernels/MainTheorems.lean:7`) |
| Single-declaration active files | 63,899 files (43.7% of active files) | 48,922 of those are `≤ 15` decl lines (`213,750` decl lines total) |
| Axioms | `{propext, Classical.choice, Quot.sound}` | Verified by Andrew Sutherland's [MIT audit](https://math.mit.edu/~drew/grh/audit.log) |

### Three Static-Analysis Traps Resolved by the Kernel Cone

1. **Behrend sets (`Formalism/BehrendSets/` is `0/89` dead `[Exact]`)**: The standalone `src/Formalism/BehrendSets/` directory never enters the kernel proof terms. The live proof constructs Behrend-free thick sets directly from Mathlib's `Behrend.exists_large_sphere` inside `Formalism.FrobeniusOrbits.hostThickExists` (`ThickExists.lean:181`, constant $C = 32$, depth 14) and performs non-uniform coset-biased Bernoulli selection in `src/Development/AutomorphicArithmetic/ThickFrames/` (`69/69` decls active: `thickFrame2`, `select`, `topsChoiceH`) `[Verified]`.
2. **Iwasawa descent without $\mathbb{Z}_p[[T]]$ (`IwasawaModules/` is `0/16` dead `[Exact]`)**: Formal power-series ring theory over $\mathbb{Z}_p[[T]]$ is unused in the cone. Instead, `UpperTheory.V.FW.ferreroWashington` (`170/170` decls active) gives a finite $\mathbb{Z}_\ell$-generator bound `Tower.IsTower l L R` on the unramified Iwasawa module, and `UpperTheory.V.PropA.{exists_centres_norm, aBound_of_descends}` lifts $\gamma^{\ell^j}$ to an $R \times R$ matrix over $\mathbb{Z}_\ell$ and factors its determinant over $\overline{\mathbb{Q}}_\ell$ to dodge at most $R$ exceptional $\ell$-adic centers (`Win.thmBab_of_lemmaK`) `[Verified]`.
3. **Standalone corollaries & clone asymmetry**: `SmallClassNumbers` (`0/23` decls active) is outside the 5-theorem kernel cone (and removed upstream). In `UnitaryGroupMassFormula`, the three near-duplicate directories have starkly asymmetric kernel usage: `StabilizerVolume` is **95.5%** active (`15,186 / 15,895` L), `StabilizerOrbitVolume` is **34.7%** active (`5,407 / 15,600` L), and `WeilStabilizerVolume` is **0.9%** active (`135 / 14,668` L — 99.1% dead) `[Exact]`.

## Upstream Simplification PRs by This Tracker's Maintainer

States below were read from the GitHub API on 2026-11-26. Line counts are the PR diffs; build and equivalence checks are as reported in each PR description. None of the touched files exist on the current `main` (the 2026-11-25 switch to the shorter proof replaced the tree).

- [PR #60](https://github.com/leanprover-community/riemann-hypothesis/pull/60) — **merged** 2026-11-22 (`173225e84`): `CapelliIdentities/FixPiIff.lean`, `+20 / -8,004` lines.
- [PR #65](https://github.com/leanprover-community/riemann-hypothesis/pull/65) — **merged** 2026-11-22 (`f1f6d6542`): 4 `CapelliIdentities` files (`FreeModuleQuotient.lean`, `ContDiffHerm.lean`, `SquareOfApproximation.lean`, `LimitOfFinite.lean`), `+142 / -4,624` lines.
- [PR #71](https://github.com/leanprover-community/riemann-hypothesis/pull/71) — closed without merge on 2026-11-22; its patch (66 `backward.*` options removed across 4 scheme files) was adapted into the merged batch [PR #115](https://github.com/leanprover-community/riemann-hypothesis/pull/115) as commit `da6765973f` ("Adapted from #71").
- [PR #109](https://github.com/leanprover-community/riemann-hypothesis/pull/109), [PR #110](https://github.com/leanprover-community/riemann-hypothesis/pull/110), [PR #224](https://github.com/leanprover-community/riemann-hypothesis/pull/224) — closed without merge on 2026-11-24 (Lean `v4.36.0` tree: restoring `#68`'s shared GL finiteness proof, simplifying the `cpR` / `std_cpR` signatures, and removing the last 5 `backward.defeqAttrib.useBackward` directives).

## Live Upstream `main` Auto-Tracker (Hourly CI) `[Exact]`

A scheduled GitHub Actions workflow ([`.github/workflows/update-tracker.yml`](.github/workflows/update-tracker.yml)) runs [`scripts/update_upstream_tracker.py`](scripts/update_upstream_tracker.py) against live `leanprover-community/riemann-hypothesis:main`, recording `.lean` file counts, lines, imports, declarations, remaining `backward.*` / `maxHeartbeats` options and the status of a watched PR list into [`data/upstream_frontier.json`](data/upstream_frontier.json) when upstream metrics change. Its latest measurement (`main@4249e7b98`, Lean `v4.37.0-rc1`, measured 2026-11-26 06:10 UTC) is `802` files and `475,141` lines, against `175,187` files in `v0.2.1`.

## Independent Verification & TCB Audit Summary

- **Checks on the original proof (`v0.2.1` tree)**: Andrew Sutherland's [MIT verification page](https://math.mit.edu/~drew/grh/) (last updated 2026-11-25 22:05 UTC) records that `leanchecker` replayed `175,187 / 175,187` modules; `nanoda` and `con-leche` (`master`, `1,468,418` declarations) accepted the full `61,570,595,788`-byte export; `lean4lean` accepted all `175,187` compiled modules module by module; the Lean default and paranoid kernels (Lean `v4.36.0`) accepted the full export; the `--compare` modes of `con-leche` and `con-ron` confirmed all five `Main.lean` theorems against the challenge; and both Comparator `v4.33.0` and `lake comparator` (Lean `v4.36.0`) reported "Your solution is okay!". The checks of the shorter proof now on `main` are on a [separate MIT page](https://math.mit.edu/~drew/shortgrh/).
- **formal-conjectures statements ([PR #88](https://github.com/leanprover-community/riemann-hypothesis/pull/88), merged 2026-11-23, by `@Deicyde`)**: adds a second Comparator setup whose challenge copies the RH and GRH statements of `google-deepmind/formal-conjectures` (commit `d8d031d4`). In a PR comment, `@ishaan2006` reported `con-leche --compare` (branch `compare`, `274f0138`) accepting `RiemannHypothesis.riemannHypothesis` and `GRH.generalized_riemann_hypothesis` on a `61,570,614,651`-byte export (`sha256 17719b54…`).
- **Arithmetic consequences (`kimihiro64/Robin1984`, commit `5ce18bf3`)**: per the same MIT page, combining the proof with Robin1984 gives `robin_inequality`, `lagarias_inequality` and `cyclotomic_corrected_ideal_robin` with axioms `{propext, Classical.choice, Quot.sound}` only; the combined export (`1,475,109` declarations) is accepted by `con-leche`, `con-ron --compare` and `nanoda`. Elaborated against Mathlib `0df444a3` alone ([`explicit-statements.out`](https://math.mit.edu/~drew/grh/explicit-statements.out)), Robin's statement names `21` constants directly (`2,050`-declaration closure) and Lagarias's names `27` (`1,801`).
- **Lean-free raw NDJSON audit ([`scripts/audit_ndjson_statement.py`](scripts/audit_ndjson_statement.py))**: parses MIT's `challenge.ndjson.zst` (unpacked `sha256 bd484ac9…b2b9`, `51,692` top-level records) with Python's standard library alone, and reports all 5 raw kernel statement ASTs with `{Classical.choice, Quot.sound, propext}` only (`0` `sorryAx`) across their statement-type closures.
- **Risky-construct scan**: upstream [`verification/static-scan.txt`](https://github.com/leanprover-community/riemann-hypothesis/blob/alpoge/verification/static-scan.txt) (kept on the `alpoge` branch) counts 0 `axiom`, 0 `sorry`, 0 `native_decide`/`ofReduceBool`/`trustCompiler`, 0 `implemented_by`/`extern`, 0 `unsafe`, 0 `opaque` and 0 `skipKernelTC` across the 175,187 original files; our line-by-line review of the remaining metaprogramming hits is in [`meta_audit_verdict.md`](https://github.com/thekeunpie-hash/rh-lean-dissection/blob/main/reports/meta_audit_verdict.md). See Section 8 of [SPEC.md](SPEC.md).

## Reproduce the Audits

Download `cone-constants.txt.gz` and `decls.csv.gz` from James Shore's `v0.2.1` kernel export, then run:

```sh
python scripts/audit_spec_citations.py \
  --cone path/to/cone-constants.txt.gz \
  --decls path/to/decls.csv.gz \
  SPEC.md README.md
```

Expected result: `[PASS]` with `dead=0` on both files.

To inspect the five raw kernel headline statements and verify their SHA-256 and axiom closures directly from MIT's `challenge.ndjson.zst` without Lean:

```sh
python scripts/audit_ndjson_statement.py \
  --expected-sha256 bd484ac9886a96f26b1d9a7a59ebd45b7a70d102c015d4f31f9302bbdd6ab2b9 \
  path/to/challenge.ndjson.zst
```

## Repository Layout

| Path | Description |
| :--- | :--- |
| [SPEC.md](SPEC.md) | Full kernel-grounded Minimal Backbone Specification (dominator spine, 3 pillars, 16-row witness table, 24-item dead-code self-audit, and Section 8 multi-kernel / TCB audit) |
| [docs/](docs/) | Source of the GitHub Pages site [riemannhypothesis.dev](https://riemannhypothesis.dev/) |
| [data/kernel_backbone_summary.json](data/kernel_backbone_summary.json) | Machine-readable global accounting, directory/subtree usage, top dominators, and 36 exact `why-parents` chains |
| [data/upstream_frontier.json](data/upstream_frontier.json) | Auto-updated live upstream `main` metrics (`files`, `lines`, `imports`, `declarations`, `backward.*`, `maxHeartbeats`) and watched PR states |
| [scripts/update_upstream_tracker.py](scripts/update_upstream_tracker.py) | CI tracker script measuring live `leanprover-community/riemann-hypothesis:main` and PR queue status (powered by [`.github/workflows/update-tracker.yml`](.github/workflows/update-tracker.yml)) |
| [scripts/fetch_verifytask.py](scripts/fetch_verifytask.py) | Zero-dependency fetcher for the public Bohrium Playground board: per-core best compression, the `main`-tier Grand Challenge, the board's model x harness ranking, repository coverage, and the Playground timeline, written to [`data/verifytask_board.json`](data/verifytask_board.json) by the same workflow |
| [skills/bohrium-rh-camp/SKILL.md](skills/bohrium-rh-camp/SKILL.md) | Camp workflow for one core pillar: read the board, build the Lean proof, optionally run it on Bohrium compute with the `bohr` CLI, and submit through the Playground CLI; every authenticated step is skipped when no owner credential is present |
| [scripts/audit_spec_citations.py](scripts/audit_spec_citations.py) | Zero-dependency Python auditor checking every active qualified Lean identifier against `cone-constants.txt.gz` and `decls.csv.gz` |
| [scripts/audit_ndjson_statement.py](scripts/audit_ndjson_statement.py) | Zero-dependency Lean-free Python auditor inspecting raw `lean4export 3.1.0` NDJSON statements, safety flags, and axiom closures |
| [LICENSE](LICENSE) | MIT License |

For the broader Round 1–2 static dissection, metaprogramming soundness audit, standalone Lean harnesses, and Korean archives, see [`thekeunpie-hash/rh-lean-dissection`](https://github.com/thekeunpie-hash/rh-lean-dissection).
