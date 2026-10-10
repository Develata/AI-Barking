# Eleven-square packing in Lean

This repository assembles the completed foundations and the available partial
formalizations of the optimal eleven-square packing. **Global optimality is
still unfinished.** Six explicit `sorry` sites record the remaining obligations.
A build that accepts those sites checks the surrounding code but does not prove
the final optimality theorem. See [MISSING.md](MISSING.md).

The target side length is the exact real number

\[
T = \frac{6u+4}{1+2u-u^2},
\]

where `u` is the unique root in `(9/25,37/100)` of

\[
5u^8-10u^7-2u^6+14u^5+12u^4-6u^3+2u^2+2u-1=0.
\]

The construction attains approximately `3.8770835900228141773`. The model allows
arbitrary orientations, legal boundary contact, and disjoint open interiors.

## Entry points

| File | Purpose |
| --- | --- |
| `ElevenSquare/Foundations.lean` | Geometry, the exact endpoint, attaining construction, closed-cell cover, and finite case reduction. |
| `ElevenSquare/Progress.lean` | Completed baseline groups, case1000 through two steps, and selected global capture helpers. |
| `ElevenSquare/Pending/` | Public interfaces and the later proof stages, with explicit remaining dependencies. |
| `ElevenSquare/Tasks/` | Returned certificate data, generic checkers, analytic lemmas, and concrete partial proofs. |
| `ElevenSquare/Optimality.lean` | Final unconditional theorem statements; their proofs currently inherit the listed admissions. |
| `ElevenSquare/Verification.lean` | Axiom queries for completed milestones and unfinished public targets. |

## Verification

Install Git and Lean's `elan` launcher. The project pins Lean `v4.10.0-rc2` and
mathlib revision `3fef63ff3bda38478ba4364ff03999f0246745a2`.
Keep `lake-manifest.json`; do not update dependencies while reproducing this
snapshot.

Set up the public dependencies, compile the main dependency chain serially,
and inspect its target axioms with one command:

```sh
python3 scripts/verify.py --setup
```

For every included source module, including progress outside the main chain:

```sh
python3 scripts/verify.py --setup --all
```

Once dependencies are installed, omit `--setup`. Accepted unchanged modules
can be resumed using the script's source/object/dependency fingerprints. Add
`--fresh` to rebuild every selected local module. These are substantial exact
certificate checks and can take a long time. They use ordinary Lean checking;
no packing search or external algebra system is required.

A source-only check, requiring only Python3, is:

```sh
python3 scripts/check_sources.py
```

`--plan` on the verifier prints the compilation order without running Lean.
Build logs and objects remain in ignored `.verification/` and `.lake/` folders.
The normal Lake entry point is also available via `lake build`.

The verifier distinguishes clean milestones, which may use only `propext`,
`Classical.choice`, and `Quot.sound`, from the explicit unfinished targets.
Success with the current six admissions is **partial assembly success**, not a
proof of optimality. Closing those admissions requires a fresh final audit.

## Assembly provenance

The source incorporates the baseline partial return, the checked prior-support
continuation, the local-packet return, and the global partial handoff. It also
preserves the previously integrated fixes to the overlay and D4 bridge from the
earlier current-work return. Older unreferenced speculative modules are omitted;
all delivered Lean modules and their local source dependencies are preserved.

`verification/source-inventory.json` records exact source hashes and which files
match supplied compiler inventories. `verification/imported-audits.json` retains
only mathematical declaration names and their reported axiom sets. It is
historical evidence, not a fresh combined compiler replay. In particular, a
reported inherited admission may have been removed by another merged return.

The assembly was checked for a complete local import closure, exact admission
inventory, matching returned source hashes, and personal information. The small
final composition is checked separately against the existing shared interfaces.
The entire large numerical certificate collection is supplied for reproducible
replay rather than claimed freshly rebuilt during packaging.

Only portable source, pinned public dependency metadata, mathematical audit
summaries, and fresh documentation are distributed. Original handoff archives,
conversation records, machine diagnostics, private project identifiers, and
historical machine-specific logs are omitted.
