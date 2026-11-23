# Practical Collision Cryptanalysis of Waterfall

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Hash%20Function-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Collision%2C%20Differential%20Cryptanalysis-blue?style=flat-square" alt="Tags" />
</p>

**Waterfall** is a dedicated hash function family ($`224, 256, 384,`$ and $`512`$-bit digests) submitted to Round 1 of the NIST SHA-3 Cryptographic Hash Algorithm Competition in 2008. Since its 2008 NIST submission, the only public cryptanalysis in the literature estimated a theoretical collision cost of roughly $`\mathcal{O}(2^{70})`$ operations, and all five frontier models evaluated in CryptanalysisBench (*NeurIPS 2026*) failed to break full-strength Waterfall.

We present the first practical collision attacks on the Waterfall hash family across all four NIST security levels (`Waterfall-224/256/384/512`). Across all $`2^{32} - 1`$ non-zero input differences to Waterfall's $`32`$-bit non-linear $`\mathsf{Xbox}`$ S-box, we discover two distinguished differences ($`\beta = \mathtt{0x00f9e70e}`$ and $`\beta' = \mathtt{0x003f7638}`$) with zero top byte ($`\beta_3 = \mathtt{0x00}`$) for which the $`4`$-step differential fixed-point set $`T_\beta`$ collapses to an exact $`13`$-dimensional $`\mathbb{F}_2`$-affine coset.
Coupling the affine coset with polynomial annihilators and active block-counter modification yields efficient full-state collisions, which run within 1 second.


## Results

| Method | Target | Message Length | Time / Complexity | Success Rate |
|---|---|---|---|---|
| Prior Differential Estimate (2008) | Waterfall family | — | $`\approx 2^{70}`$ ops | — |
| Dual-Excursion Annihilator (`solve5`) | Waterfall-256 | $`2{,}816\text{ B}`$ | $`45.0\text{ ms}`$ | $`90\%`$ |
| Quiet-Counter Coset Scan (`attack7`) | Waterfall-256 | $`576\text{ B}`$ | $`171.0\text{ ms}`$ | $`100\%`$ |
| Quiet-Counter Projection (`ucol9`) | Waterfall-224/256/384/512 | $`576\text{ B}`$ | $`3.8\text{ ms}`$ | $`100\%`$ |
| Immediate Counter Steering (`ucol8`) | Waterfall-224/256/384/512 | $`512\text{ B}`$ | $`3.5\text{ ms}`$ | $`100\%`$ |
| Pre-Steered Minimal Trail (`ucol7`) | Waterfall-224/256/384/512 | $`448\text{ B}`$ | $`52.1\text{ ms}`$ | $`100\%`$ |

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026waterfall,
  title  = {Practical Collision Cryptanalysis of {Waterfall}},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/Waterfall}
}
```
