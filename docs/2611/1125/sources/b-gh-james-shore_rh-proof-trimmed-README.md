# rh-proof-trimmed

The Lean 4 proof of RH and GRH for Dirichlet L-functions ([remark.pdf](https://alpo.ge/remark.pdf)), made smaller by
mechanical steps only.

- The five theorems in `src/Main.lean` and their statements are unchanged
- No proof is written by hand or by a model
- Each stage is one commit on top of
  [leanprover-community/riemann-hypothesis](https://github.com/leanprover-community/riemann-hypothesis) main
- S1 is on main at `7da4577e` and S2 on main at `bdda2f0e`
- Each stage has a receipt in `cleaning/<stage>/`
- S1 is [PR #55](https://github.com/leanprover-community/riemann-hypothesis/pull/55) on leanprover-community/riemann-hypothesis
- S2 is [PR #67](https://github.com/leanprover-community/riemann-hypothesis/pull/67) on leanprover-community/riemann-hypothesis

## Stages

| Stage | What | Files | Lines | Declarations | Compiles | Kernel-accepted | Same statements | Term diff |
|---|---|---|---|---|---|---|---|---|
| Base | the source at `7da4577e` | 164,463 | 16,588,671 | 1,024,669 | yes (leanprover-community's CI) | | | |
| S1 | remove declarations the proof doesn't use | 150,632 | 14,473,033 | 858,404 | yes | yes | yes | 0 statements changed |
| S2 | remove syntax Lean's own linters flag as unused | 150,632 | 14,453,805 | 858,404 | yes | yes | yes | 0 statements changed |
| S3 | keep one copy of each duplicate theorem or definition, on Lean 4.37 only, see below | | | | | | | |
| S4 | remove import lines each file doesn't need, on branch [`s4-on-s1`](https://github.com/james-shore/rh-proof-trimmed/tree/s4-on-s1) | 150,632 | 14,252,250 | 858,404 | yes | yes | yes | 0 statements changed |

- Compiles means Lean 4.33.1 builds every module
- Kernel-accepted means an independent kernel (nanoda) accepts the exported proof
- Same statements means con-leche `--compare` accepts the export against the published challenge
- Term diff compares each statement in the exported proof with the original archive's
- Files and lines exclude `Challenge.lean`

## Lean 4.37

- S2 is ported to Lean 4.37 as [PR #243](https://github.com/leanprover-community/riemann-hypothesis/pull/243) on
  leanprover-community/riemann-hypothesis, built on [#185](https://github.com/leanprover-community/riemann-hypothesis/pull/185),
  now merged. Its lists are at tag [`s2-pr-185-7`](https://github.com/james-shore/rh-proof-trimmed/tree/s2-pr-185-7)
- On main before #221 it compiles and verifies on Lean 4.37. It compiles on
  [#134](https://github.com/leanprover-community/riemann-hypothesis/pull/134) under Lean 4.36, and cuts the warnings from the
  three linters by 94%
- S3 is [PR #295](https://github.com/leanprover-community/riemann-hypothesis/pull/295) on leanprover-community/riemann-hypothesis,
  built on [#185](https://github.com/leanprover-community/riemann-hypothesis/pull/185), now merged. Its lists are at tag
  [`s3-pr-185-5`](https://github.com/james-shore/rh-proof-trimmed/tree/s3-pr-185-5)
- It compiles on Lean 4.37 on main after #221, and compiles and verifies on main before #221. A version of it compiles
  and verifies on #134 under Lean 4.36

## Base

- `7da4577e` differs from the original archive (`8b83d5e7`) in 10,737 files under `src/`
- 10,724 modules are deleted, and 13 files have only their import lines changed
- leanprover-community's CI built the same `src/` in run 44945214129 (173,179 jobs, exit 0)
- S1 on the earlier base `8f558a88` (`d25d2b3c…`) is tag `s1-on-8f558a88`
- S2 is on main at `bdda2f0e`, merged in before S2. It adds PR #57's edits to 5 files that S1 doesn't touch

## Regenerate a stage

```
python3 cleaning/transform/transform.py BASE/src DEST --keep keep.tsv --level L2g \
  --lib-namespaces cleaning/transform/mathlib-namespaces.txt.gz [--statement-cone CONE.tsv --edits LIST.tsv …]
```

- BASE is this repository at `7da4577e`
- The last line printed is the tree fingerprint, which must match the receipt
- Edit lists need `--statement-cone`, the table `data/statement-cone-with_definitions.tsv` from
  [rh-proof-dependencies](https://github.com/james-shore/rh-proof-dependencies). No edit can touch it
- The edit format is in `cleaning/transform/EDIT-LISTS.md`

## Layout

- `src/` is the source after the latest stage
- `cleaning/` has the transform, the keep list, the edit lists and the receipts
- Everything else is the base, unchanged. Its README is `README-base.md`

## Inputs

- Base `7da4577eb40dd74b09218d111114ca43bc2ad226`
- Lean v4.33.1 and Mathlib `0df444a360eaa60ab8c11dca51a86af692955474`
- Used declarations from [rh-proof-dependencies](https://github.com/james-shore/rh-proof-dependencies)
- `cleaning/transform/` follows [keithadler/rh-trim](https://github.com/keithadler/rh-trim) `tools/prune.py`
  (Apache-2.0)

## Licence

[LICENSE.md](LICENSE.md)
