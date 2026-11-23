# Collision and Preimage Cryptanalysis of Blender

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Hash%20Function-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Collision%2C%20Preimage-blue?style=flat-square" alt="Tags" />
</p>

**Blender** is a cryptographic hash function family submitted to Round 1 of the NIST SHA-3 Competition in 2008. In CryptanalysisBench (*NeurIPS 2026*), evaluated models bypassed Blender only by exploiting a 64-bit `unsigned long` width bug in its C reference implementation. Prior literature (2008–2009) achieved no full-round standard-IV collision on `Blender-256` below the $`2^{128}`$ birthday bound and required at least $`2^{107.32}`$ words for preimages (ePrint 2009/006).

We analyze the exact coupling between Blender's word-level state recurrence and carry propagation to obtain the first practical full-round collision attacks on the specification. On `Blender-256`, our attack compresses the $`258`$-bit core state into a reachable $`38`$-bit invariant subspace, producing full-round collisions on the official implementation in $`0.045`$–$`0.73\text{ s}`$ ($`8`$ threads). Across the full family, we obtain practical semi-free-start collisions for all four NIST variants (`Blender-224/256/384/512`) and reduce full preimage recovery to $`2^{35.93}`$, $`2^{66.34}`$, and $`2^{67.64}`$ operations on `Blender-256/384/512`.

## Results

| Method | Target | Attack Type | Message Length | Time / Complexity |
|---|---|---|---|---|
| Prior Collision Analysis (2008–2009) | Blender-256 | Full-state multicollision stage | $`O(n)\text{ w}`$ | $`2^{144.0}`$ ops |
| Prior Preimage Analysis (2009) | Blender-256 | Preimage | $`2^{107.3}\text{ w}`$ / $`1{,}281\text{ w}`$ | $`\ge 2^{107.32}`$ / $`2^{136.00}`$ ops |
| Shifted Subspace Cycle-Swap | Blender-256 | Full-round collision | $`16{,}000\text{ B}`$ | $`0.73\text{ s}`$ |
| Carry-Healing Inversion | Blender-256 | Full-round collision | $`24\text{ B}`$ | $`34.6\text{ s}`$ |
| Carry-Healing Identical-Prefix | Blender-256 | Identical-prefix collision | $`36\text{ B}`$ / $`40\text{ B}`$ | — / $`145.5\text{ s}`$ |
| Closed-Form SFS | Blender-224/256/384/512 | Semi-free-start collision | $`113\text{ B}`$ / $`229\text{ B}`$ | $`0.006`$–$`0.054\text{ }\mu\text{s}`$ |
| Multi-Shifted $`\mathsf{SNF}`$ Herding | Blender-256 / 384 / 512 | Full preimage | $`2^{27.46}`$ / $`2^{38.91}`$ / $`2^{39.25}\text{ w}`$ | $`2^{35.93}`$ / $`2^{66.34}`$ / $`2^{67.64}`$ ops |

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026blender,
  title  = {Collision and Preimage Cryptanalysis of {Blender}},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/Blender}
}
```
