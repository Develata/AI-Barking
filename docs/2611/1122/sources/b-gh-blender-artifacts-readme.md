# Reproducible Cryptanalysis Artifacts for Blender (NIST SHA-3 Round 1)

This directory contains self-contained C11 and Python 3 verification suites, live collision and partial-preimage solvers, and precomputed witnesses for all cryptanalytic results in [../paper/main.pdf](../paper/main.pdf), verified directly against the NIST SHA-3 Round-1 C reference implementation (`reference/core.c` with fixed 32-bit/64-bit `<stdint.h>` word widths).

## Environment and Dependencies

- **Supported Platforms & Architecture**:
  - **Operating Systems**: 64-bit Linux and macOS (POSIX-compliant environments).
  - **Architectures**: Little-endian 64-bit `x86_64` and `arm64` (`aarch64`).
  - **Word-Width & Endianness Notes**: In `reference/api.h` and `reference/core.h`, `Uint32`, `Uint64`, and `DataLength` are bound to fixed-width `<stdint.h>` types (`uint32_t` and `uint64_t`) so that `reference/core.c` executes exact 32-bit and 64-bit arithmetic on 64-bit LP64 hosts (avoiding the 2008 64-bit `unsigned long` width bug). Because `reference/core.c` aliases 32-bit and 64-bit fields inside its `Mint` union during finalization, a little-endian host (`x86_64` or `arm64`) is required. The 64-bit SFS and probability routines use compiler-native `__uint128_t` (supported by 64-bit GCC and Clang); `carry_healing/sfs_closed_form.c` uses optional ARMv8 carryless multiplication (`vmull_p64`) when `__ARM_FEATURE_CRYPTO` is enabled and automatically falls back to a portable 64-step C loop on `x86_64` and baseline `arm64`.
- **C Compiler & Threading**:
  - Any standard C11 compiler (`cc`, `gcc`, or `clang`) supporting `-std=c11`, `-O3`, POSIX threads (`-pthread`), C11 atomics (`<stdatomic.h>`), and `<math.h>` (`-lm`).
  - Zero external C libraries, OpenSSL/GMP headers, or SAT/SMT libraries are required.
- **Python Environment**:
  - `Python 3.8+` (`python3`) using the Python standard library only (`itertools`, `json`, `math`, `os`, `random`, `struct`, `subprocess`, `sys`, `tempfile`).
  - Zero third-party Python packages and zero external SAT/SMT solvers are required.
- **RAM & CPU Footprint**:
  - **`./run.sh --demo`**: $`< 15\text{ MB}`$ peak RAM (`cycle_swap_solver` allocates $`\approx 1\text{ MB}`$ per worker thread across 8 threads; `sfs_closed_form`, `mc_healing_density`, and `byte_lattice_solver` each use $`< 5\text{ MB}`$). Runs in $`\approx 5`$–$`10\text{ s}`$ on an 8-core CPU.
  - **`./run.sh --full`**: $`\approx 300\text{ MB}`$ peak RAM (`snf_herding_solver` in 32-bit mode uses $`\approx 25\text{ MB}`$ for the 8-anchor reverse-reachable Sink-DAG and $`\approx 280\text{ MB}`$ for the 4-list generalized birthday tree over $`2^{22}`$-entry leaf lists). Runs in $`\approx 3`$–$`5\text{ min}`$ on 8 threads.
  - **Standalone 64-bit partial-preimage solver (`snf_herding_solver 64 ...`)**: $`\approx 2.2\text{ GB}`$ peak RAM (streaming 8-list 3-level generalized birthday tree over $`2^{24}`$-entry leaf lists; at least 4 GB of available RAM is recommended).
  - **Multi-Threading & Scratch Hygiene**: All parallel C solvers use POSIX `pthread` worker pools (`THREADS=8` by default in `run.sh`, clamped safely to internal worker array bounds). `run.sh` compiles all binaries and writes temporary outputs exclusively inside a `mktemp -d` directory removed automatically via an `EXIT` trap, leaving the source tree completely clean.

## Quick Start / Usage

Ensure `run.sh` is executable (`chmod +x run.sh`) and run from inside `artifacts/` (on first run, `run.sh` automatically invokes `./fetch_third_party.sh --ensure` to download and SHA-256-verify the external NIST/EACirc reference files into `reference/`, printing a terminal notice; subsequent runs use the local files offline):

```bash
# Optional: explicitly pre-fetch / verify third-party reference sources
./fetch_third_party.sh

# Fast end-to-end verification (~5-10 s, < 15 MB RAM):
#   [1/5] Compiles reference/core.c + blender_cli.c and checks NIST SHA-3 KATs
#   [2/5] Verifies precomputed & live 16,000 B standard-IV collisions, t | m law across
#         all 15 variants, Parity Obstruction Theorem, out-degree 2.000, and 20 multicollisions
#   [3/5] Verifies 80,000 zero-retry O(1) SFS collisions across Blender-224/256/384/512,
#         stored 24 B / 36 B / 40 B witnesses, 200k-state healing density, and a live 24 B solve
#   [4/5] Verifies 2-adic SNF subgroup ranks, C_inv = 0 on core.c, and full preimage bounds
#   [5/5] Verifies Fixed-IV 32-bit (0xdeadbeef, 0x00000000) and 64-bit (0^64) preimage witnesses
./run.sh --demo

# Extended benchmark & live herding suite (~3-5 min, ~300 MB RAM):
#   Adds 10^7-state parallel Monte Carlo carry-healing density audit across 4 ensembles,
#   5-seed cold-start sweeps for 24 B Standard-IV and 40 B Identical-Prefix ("Dear Bob, wire $")
#   collisions, and a live Fixed-IV 32-bit SNF partial-preimage solve (0xcafebabe) from scratch
./run.sh --full

# Optional compiler and thread-count overrides:
CC=gcc THREADS=4 ./run.sh --demo
```

### Standalone CLI and Solver Usage

To run individual tools manually (compiling into a temporary directory so no binaries remain in `artifacts/`):

```bash
TMP=$(mktemp -d)

# 1. Reference CLI (run ./fetch_third_party.sh --ensure first if reference/core.c is not yet fetched)
./fetch_third_party.sh --ensure
cc -std=c11 -O2 -o "$TMP/blender_cli" reference/blender_cli.c reference/core.c
"$TMP/blender_cli" -l 256 -p "616263"
"$TMP/blender_cli" -l 256 -f subspace_collision/witnesses/cycle_swap_m1.bin

# 2. Subspace Cycle-Swap: Standard-IV Shifted Subspace Cycle-Swap Solver & Theory Verifier
cc -std=c11 -O3 -pthread -o "$TMP/cycle_swap_solver" subspace_collision/cycle_swap_solver.c
"$TMP/cycle_swap_solver" 8 7 "$TMP/m1.bin" "$TMP/m2.bin" 0x0
python3 subspace_collision/verify_subspace_theory.py subspace_collision/witnesses/cycle_swap_uv.txt "$TMP/blender_cli"

# 3. Carry Healing: All-Variant O(1) SFS, Carry-Healing Density & Pure-C 24 B / 40 B Solver
cc -std=c11 -O3 -o "$TMP/sfs_closed_form" carry_healing/sfs_closed_form.c
"$TMP/sfs_closed_form"

cc -std=c11 -O3 -pthread -o "$TMP/mc_healing_density" carry_healing/mc_healing_density.c -lm
"$TMP/mc_healing_density" --demo -t 8          # or --full -t 8 for 10^7 states/ensemble

cc -std=c11 -O3 -pthread -o "$TMP/byte_lattice_solver" carry_healing/byte_lattice_solver.c
"$TMP/byte_lattice_solver"                     # Fast live 24 B Standard-IV solve
"$TMP/byte_lattice_solver" --solve-cp40 -t 8   # Live 40 B Identical-Prefix ("Dear Bob, wire $") solve

# 4. Multi-Shifted SNF Steering: 2-Adic SNF Subgroup Auditor & Fixed-IV 32/64-Bit Herding Solver
python3 snf_steering/snf_subgroup_audit.py snf_steering/witnesses/snf_audit_vectors.json "$TMP/blender_cli"

cc -std=c11 -O3 -pthread -o "$TMP/snf_herding_solver" snf_steering/snf_herding_solver.c
"$TMP/snf_herding_solver" 32 0xdeadbeef "$TMP/herd32.bin" 8             # ~300 MB RAM
"$TMP/snf_herding_solver" 64 0x0000000000000000 "$TMP/herd64.bin" 8     # ~2.2 GB RAM

rm -rf "$TMP"
```

## Directory Layout

- `run.sh`: Automated end-to-end build and verification harness (`--demo` and `--full` modes; auto-invokes `fetch_third_party.sh --ensure` when third-party reference files are absent).
- `fetch_third_party.sh`: Downloads and SHA-256-verifies the official NIST SHA-3 Round-1 Blender C reference (applying deterministic `<stdint.h>` LP64 width bindings) and EACirc C++ archival reference into gitignored `reference/` paths.
- `reference/`:
  - `blender_cli.c`: Tracked command-line driver linking against `core.c`, supporting hex strings (`-p`), raw binary files (`-f`), semi-free-start initial states (`--sfs-iv`), and specification-table `Blender-384` initialization (`--spec-384`) across all digest sizes (`224/256/384/512`).
  - `api.h`, `core.h`, `core.c` *(fetched on demand)*: NIST SHA-3 Round-1 C reference implementation (2008) with fixed 32-bit/64-bit `<stdint.h>` integer types (`uint32_t`, `uint64_t`) for 64-bit LP64 portability.
  - `Blender_sha3.h`, `Blender_sha3.cpp` *(fetched on demand)*: Comment-preserved archival C++ reference submission.
- `subspace_collision/` (Section 3 of [../paper/main.pdf](../paper/main.pdf) — Shifted Clean Subspace Collapse & Commuting Closed Walks):
  - `cycle_swap_solver.c`: Multi-threaded single-stage overlapping ancestor cycle-swap collision solver from the standard `Blender-256` IV ($`0.045`$–$`0.73\text{ s}`$ solve time, $`2^{22.6}`$ mean DFS nodes).
  - `verify_subspace_theory.py`: Verifies the general $`t \mid m`$ subspace-collapse divisibility criterion across all 15 Blender parameterizations, the 8-phase Parity Obstruction Theorem, exact supercritical mean out-degree $`2.000`$ across `Blender-256/384/512`, and $`\binom{6}{3} = 20`$ equal-length multicolliding messages on `reference/core.c`.
  - `witnesses/`: Precomputed $`16{,}000`$-byte standard-IV collision pair (`cycle_swap_m1.bin`, `cycle_swap_m2.bin`) and `(entry, prefix, U, V)` multicollision specification (`cycle_swap_uv.txt`).
- `carry_healing/` (Section 4 of [../paper/main.pdf](../paper/main.pdf) — Two-Step Silent Carry Injection, Healing Rigidity & Pure-C Byte-Lattice Inversion):
  - `sfs_closed_form.c`: Deterministic zero-retry $`O(1)`$ closed-form semi-free-start (SFS) collision generator across all four official NIST variants (`Blender-224` and `Blender-256` at $`113\text{ B}`$; `Blender-384` and `Blender-512` at $`229\text{ B}`$), verifying $`80{,}000 / 80{,}000`$ pairs against all 8 NIST KATs and full compression/finalization.
  - `mc_healing_density.c`: Exact single-adder carry-flip density check ($`N_1 / 2^{64} \approx 7/48`$), independent bit-by-bit DFS completeness cross-check on $`10{,}000`$ states, and multi-threaded Monte Carlo audit verifying unordered state validity density $`\approx 2^{-69.56}`$ on `Blender-224` ($`\ll 2^{112}`$ birthday bound) and $`\approx 2^{-69.58}`$ on `Blender-256`.
  - `byte_lattice_solver.c`: Pure-C 4-round lane-invariant + 2-round $`r = 8`$ byte-lattice nibble-contraction solver ($`\approx 0.11`$–$`0.29\text{ ms}`$ per 2-round inversion) producing $`24`$-byte ($`6`$-word) Standard-IV collisions and $`40`$-byte identical-prefix (`"Dear Bob, wire $"`) collisions on `Blender-256`.
  - `witnesses/collisions.json`: Precomputed $`24`$-byte Standard-IV and $`36`$-byte / $`40`$-byte identical-prefix (`"Dear Bob, wire $"`) colliding message pairs verified on `reference/core.c`.
- `snf_steering/` (Section 5 of [../paper/main.pdf](../paper/main.pdf) — Multi-Shifted Closed-Walk Steering & Preimage Recovery):
  - `snf_subgroup_audit.py`: Exact 2-adic Smith Normal Form ($`\mathsf{SNF}`$) subgroup auditor over $`(\mathbb{Z}/2^{32}\mathbb{Z})^8`$ confirming invariant factors $`(1,1,2,2,4,4,8,8)`$ (index $`2^{12}`$) in the unshifted subspace $`\mathcal{C}_0`$ versus $`(1,1,2,2,2,2,2,2)`$ (index $`2^6`$, saturating $`\mathcal{H}_{\mathrm{par}}`$) across multi-shifted subspaces $`\mathcal{C}_{a_j}`$, $`\mathbb{F}_2`$ non-linearity vs. modular exactness, zero finalization inversion cost ($`C_{\mathrm{inv}} = 0`$) on `reference/core.c`, and full preimage complexity evaluation (`Blender-256/384/512` in $`2^{35.93} / 2^{66.34} / 2^{67.64}`$ vs. prior ISC 2009 bounds $`2^{136.00} / 2^{200.58} / 2^{265.00}`$).
  - `snf_herding_solver.c`: Multi-threaded pure-C 8-anchor shifted Sink-DAG + joint $`(\Delta C_1, \Delta\mathsf{Sum})`$ generalized birthday $`k`$-tree solver for Fixed-IV 32-bit and 64-bit partial-digest preimages on `Blender-256`.
  - `witnesses/`: Precomputed Fixed-IV 32-bit (`herded_32_deadbeef.bin`, `herded_32_zero.bin`) and 64-bit (`herded_64_zero.bin`, two full 32-bit digest words simultaneously zero) preimage witnesses verified on `reference/core.c`, plus `snf_audit_vectors.json`.

## External Reference Sources (Fetched on Demand)

Third-party reference implementations are not redistributed in this repository; `./fetch_third_party.sh` (called automatically by `./run.sh` when missing) fetches and SHA-256-verifies them locally into gitignored paths under `reference/`:

- `reference/{core.c,core.h,api.h}` — Extracted from the NIST SHA-3 Round 1 `Blender.zip` submission archive (Colin Bradbury, 2008; mirrored at `ethz-spylab/cryptanalysis-benchmark` @ `d243a0a0`, MIT).
- `reference/{Blender_sha3.cpp,Blender_sha3.h}` — Fetched from `crocs-muni/eacirc-streams` @ `b92d96ad` (MIT).
