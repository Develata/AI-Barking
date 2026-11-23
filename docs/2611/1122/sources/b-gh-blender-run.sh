#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"
MODE="${1:---demo}"
case "$MODE" in
    --demo|--full) ;;
    -h|--help)
        echo "Usage: ./run.sh [--demo | --full]"
        exit 0
        ;;
    *)
        echo "Usage: ./run.sh [--demo | --full]" >&2
        exit 1
        ;;
esac

./fetch_third_party.sh --ensure

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

CC="${CC:-cc}"
THREADS="${THREADS:-8}"

echo "======================================================================"
echo "Blender Hash Family Cryptanalysis Verification Suite"
echo "Target: Blender (NIST SHA-3 Round 1 Submission, 2008)"
echo "Mode: $MODE (CC=$CC, THREADS=$THREADS)"
echo "======================================================================"

# 1. Build NIST SHA-3 Round-1 C reference CLI
echo "[1/5] Compiling NIST SHA-3 Round-1 C reference (reference/core.c)..."
"$CC" -std=c11 -O2 -o "$TMP/blender_cli" reference/blender_cli.c reference/core.c

# Verify official NIST KATs on empty string and "abc" for Blender-256
d_empty_256=$("$TMP/blender_cli" -l 256 -p "")
[ "$d_empty_256" = "98fef3418262e83ebb21461e147cbba650f8c7b9fb5357f43e4d3ea944cbf6a5" ] || { echo "FAIL: KAT 256 empty"; exit 1; }
d_abc_256=$("$TMP/blender_cli" -l 256 -p "616263")
[ "$d_abc_256" = "fe255765c2d114ae120e959d5dad38757f1b6ae1427e5d56295702ca867a39d0" ] || { echo "FAIL: KAT 256 abc"; exit 1; }
echo "  PASS  Official NIST C reference compiled & verified against NIST KATs."

# 2. Shifted Clean Subspace Collapse & Overlapping Cycle-Swap Collisions
echo ""
echo "[2/5] Shifted Clean Subspace Collapse, Parity Obstruction & Cycle-Swap Solver..."
"$CC" -std=c11 -O3 -pthread -o "$TMP/cycle_swap_solver" subspace_collision/cycle_swap_solver.c

cmp -s subspace_collision/witnesses/cycle_swap_m1.bin subspace_collision/witnesses/cycle_swap_m2.bin && { echo "FAIL: identical cycle-swap witnesses"; exit 1; }
r1_d1=$("$TMP/blender_cli" -l 256 -f subspace_collision/witnesses/cycle_swap_m1.bin)
r1_d2=$("$TMP/blender_cli" -l 256 -f subspace_collision/witnesses/cycle_swap_m2.bin)
[ "$r1_d1" = "$r1_d2" ] || { echo "FAIL: cycle-swap witness digest mismatch"; exit 1; }
echo "  PASS  Precomputed subspace cycle-swap standard-IV collision verified on core.c: ${r1_d1:0:32}..."

# Live single-stage overlapping cycle-swap solve from standard IV
"$TMP/cycle_swap_solver" "$THREADS" 7 "$TMP/live_cs_m1.bin" "$TMP/live_cs_m2.bin" > "$TMP/cs_live.log"
cmp -s "$TMP/live_cs_m1.bin" "$TMP/live_cs_m2.bin" && { echo "FAIL: live cycle-swap identical"; exit 1; }
lr1_d1=$("$TMP/blender_cli" -l 256 -f "$TMP/live_cs_m1.bin")
lr1_d2=$("$TMP/blender_cli" -l 256 -f "$TMP/live_cs_m2.bin")
[ "$lr1_d1" = "$lr1_d2" ] || { echo "FAIL: live cycle-swap mismatch"; exit 1; }
grep "Solved in" "$TMP/cs_live.log" | sed 's/^/  /'
echo "  PASS  Live subspace cycle-swap collision verified by official NIST C reference: ${lr1_d1:0:32}..."

python3 subspace_collision/verify_subspace_theory.py subspace_collision/witnesses/cycle_swap_uv.txt "$TMP/blender_cli" | sed 's/^/  /'

# 3. 4-Variant Zero-Retry Closed-Form SFS, Monte Carlo Density & Pure-C 24B/40B Byte-Lattice Solver
echo ""
echo "[3/5] Carry Healing: All-Variant Closed-Form SFS, Healing Density & Byte-Lattice Solver..."
"$CC" -std=c11 -O3 -o "$TMP/sfs_closed_form" carry_healing/sfs_closed_form.c
"$CC" -std=c11 -O3 -pthread -o "$TMP/mc_healing_density" carry_healing/mc_healing_density.c -lm
"$CC" -std=c11 -O3 -pthread -o "$TMP/byte_lattice_solver" carry_healing/byte_lattice_solver.c

"$TMP/sfs_closed_form" | grep -E "PASS|Zero-Retry|Blender-" | sed 's/^/  /'

python3 - "$TMP/blender_cli" carry_healing/witnesses/collisions.json <<'PY'
import json, subprocess, sys
cli, path = sys.argv[1], sys.argv[2]
with open(path, encoding="utf-8") as fp:
    records = json.load(fp)
for r in records:
    b1, b2 = bytes.fromhex(r["m1_hex"]), bytes.fromhex(r["m2_hex"])
    assert len(b1) == r["bytes"] and len(b2) == r["bytes"] and b1 != b2
    if "prefix_ascii" in r:
        p = r["prefix_ascii"].encode()
        assert b1.startswith(p) and b2.startswith(p)
    d1 = subprocess.check_output([cli, "-l", str(r["variant"]), "-p", r["m1_hex"]]).decode().strip()
    d2 = subprocess.check_output([cli, "-l", str(r["variant"]), "-p", r["m2_hex"]]).decode().strip()
    assert d1 == d2 == r["digest_hex"]
    print(f"  PASS  Verified {r['id']} ({r['bytes']} B) on official core.c: {d1[:32]}...")
PY

if [ "$MODE" = "--full" ]; then
    "$TMP/mc_healing_density" --full -t "$THREADS" | grep -E "Ensemble|Pr\[state admits" | sed 's/^/  /'
    "$TMP/byte_lattice_solver" --full -t "$THREADS" | sed 's/^/  /'
else
    "$TMP/mc_healing_density" --demo -t "$THREADS" | grep -E "Ensemble|Pr\[state admits" | sed 's/^/  /'
    "$TMP/byte_lattice_solver" | sed 's/^/  /'
fi

# 4. Multi-Shifted SNF Modular-Sum Steering: 2-Adic SNF Subgroup Hierarchy & Fixed-IV Partial Preimages
echo ""
echo "[4/5] Multi-Shifted SNF Modular-Sum Steering: 2-Adic SNF Hierarchy, C_inv = 0 & Preimage Bounds..."
"$CC" -std=c11 -O3 -pthread -o "$TMP/snf_herding_solver" snf_steering/snf_herding_solver.c

python3 snf_steering/snf_subgroup_audit.py snf_steering/witnesses/snf_audit_vectors.json "$TMP/blender_cli" | sed 's/^/  /'

echo ""
echo "[5/5] Verifying Fixed-IV 32-Bit & 64-Bit Partial-Digest Preimage Witnesses on Official core.c..."
d_dead=$("$TMP/blender_cli" -l 256 -f snf_steering/witnesses/herded_32_deadbeef.bin)
[ "${d_dead:0:8}" = "deadbeef" ] || { echo "FAIL: 32-bit deadbeef witness"; exit 1; }
echo "  PASS  Fixed-IV 32-bit witness (0xdeadbeef): $d_dead"

d_z32=$("$TMP/blender_cli" -l 256 -f snf_steering/witnesses/herded_32_zero.bin)
[ "${d_z32:0:8}" = "00000000" ] || { echo "FAIL: 32-bit zero witness"; exit 1; }
echo "  PASS  Fixed-IV 32-bit witness (0x00000000): $d_z32"

d_z64=$("$TMP/blender_cli" -l 256 -f snf_steering/witnesses/herded_64_zero.bin)
[ "${d_z64:0:16}" = "0000000000000000" ] || { echo "FAIL: 64-bit zero witness"; exit 1; }
echo "  PASS  Fixed-IV 64-bit witness (0^64, two full 32-bit words == 0): $d_z64"

if [ "$MODE" = "--full" ]; then
    echo "  Running live Fixed-IV 32-bit SNF herding solve from scratch..."
    "$TMP/snf_herding_solver" 32 0xcafebabe "$TMP/live_herd32.bin" "$THREADS"
    d_live32=$("$TMP/blender_cli" -l 256 -f "$TMP/live_herd32.bin")
    [ "${d_live32:0:8}" = "cafebabe" ] || { echo "FAIL: live 32-bit herding"; exit 1; }
    echo "  PASS  Live Fixed-IV 32-bit SNF herding verified on official core.c: $d_live32"
fi

echo ""
echo "======================================================================"
echo "ALL VERIFICATIONS PASSED AGAINST OFFICIAL NIST C REFERENCE (core.c)"
echo "======================================================================"
