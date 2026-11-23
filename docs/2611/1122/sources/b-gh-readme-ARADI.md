# Integral Cryptanalysis of ARADI

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Block%20Cipher-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Key%20Recovery%2C%20Integral%20Cryptanalysis-blue?style=flat-square" alt="Tags" />
</p>

$`\mathsf{ARADI}`$ is a $`16`$-round low-latency block cipher published by the U.S. National Security Agency (NSA) in 2024 for inline memory encryption. Unlike standard academic designs, the specification provides neither formal security bounds nor selection criteria for its linear-layer rotation offsets (noting only that they were chosen via a *"limited search"*). Recent quasidifferential analysis (*ToSC* 2026) presented high-round differential trails on $`\mathsf{ARADI}`$ which hold only on weak-key classes. Meanwhile, published attacks on $`5`$ to $`7`$ rounds still required $`2^{42}`$ to $`2^{72}`$ evaluations for partial round keys (*LATINCRYPT* 2025), leaving practical all-key recovery of the $`256`$-bit master key open.

We uncover algebraic degeneracies in $`\mathsf{ARADI}`$'s linear layers and key schedule, unified under the $`\mathsf{OPTIC}`$ (*Orthogonal Pullback and Ternary Integral Cryptanalysis*) framework. This enables $`256`$-bit master-key recovery across $`5`$ to $`12`$ rounds. On a single CPU core, we achieve practical master-key recovery up to $`8`$ rounds in $`\le 16\text{ s}`$ using $`4`$ to $`90{,}112`$ chosen plaintexts. We also present the first dedicated $`9`$-round attack ($`2^{54.77}`$ butterflies) and improve all-key theoretical attacks on $`10`$ to $`12`$ rounds. Full $`16`$-round $`\mathsf{ARADI}`$ remains unbroken.

## Results

| Method | Rounds | Target | Data ($`\mathsf{CP}`$) | Time / Complexity | Memory |
|---|---|---|---|---|---|
| Truncated Differential (*LATINCRYPT* 2025) | $`5`$ | $`K^{(5)}`$ | $`20`$ | $`2^{42}`$ ops | $`O(1)`$ |
| Zero-Sum + $`\mathsf{FWHT}`$ (2025) | $`6`$ | $`2`$ subkeys | $`1{,}536`$ | $`< 0.1\text{ s}`$ | $`4\text{ KB}`$ |
| Neural Distinguisher (*LATINCRYPT* 2025) | $`7`$ | $`K^{(7)}`$ | $`2^{32}`$ | $`2^{72}`$ evals | — |
| Zero-Sum + $`\mathsf{FWHT}`$ (2025) | $`10`$ | Full MK | $`2^{118}`$ | $`2^{118}`$ enc. | $`2^{48}\text{ bits}`$ |
| Zero-Sum + $`\mathsf{FWHT}`$ (2025) | $`11`$ | Full MK | $`2^{119}`$ | $`2^{147.76}`$ pck. | $`2^{107.1}\text{ bits}`$ |
| Zero-Sum + $`\mathsf{FWHT}`$ (2025) | $`12`$ | Full MK | $`2^{126.81}`$ | $`2^{151.80}`$ pck. | $`2^{123.5}\text{ bits}`$ |
| Quasidifferential (*ToSC* 2026)<sup>\*</sup> | $`12`$ | Full MK | $`2^{108.68}`$ | $`2^{196.68}`$ enc. | $`2^{36}\text{ blocks}`$ |
| Quasidifferential (*ToSC* 2026)<sup>\*</sup> | $`14`$ | Full MK | $`2^{120}`$ | $`2^{208}`$ enc. | $`2^{36}\text{ blocks}`$ |
| $`\mathsf{OPTIC}`$ | $`5`$ | Full MK | $`4`$ | $`16.0\text{ s}`$ | $`< 10\text{ MB}`$ |
| $`\mathsf{OPTIC}`$ | $`6`$ | Full MK | $`48`$ | $`2.4\text{ s}`$ | $`< 10\text{ MB}`$ |
| $`\mathsf{OPTIC}`$ | $`7`$ | Full MK | $`3{,}072`$ | $`3.5\text{ s}`$ | $`< 5\text{ MB}`$ |
| $`\mathsf{OPTIC}`$ | $`8`$ | Full MK | $`90{,}112`$ | $`6.0\text{ s}`$ | $`< 20\text{ MB}`$ |
| $`\mathsf{OPTIC}`$ | $`9`$ | Full MK | $`2^{22.32}`$ | $`2^{54.77}`$ butterflies ($`\approx 2^{45.33}`$ enc.) | $`4.7\text{ MB}`$ |
| $`\mathsf{OPTIC}`$ | $`10`$ | Full MK | $`2^{116.00}`$ | $`2^{116.00}`$ enc. | $`2^{109.9}\text{ bits}`$ |
| $`\mathsf{OPTIC}`$ | $`11`$ | Full MK | $`2^{118.81}`$ | $`2^{147.17}`$ pck. | $`2^{119.5}\text{ bits}`$ |
| $`\mathsf{OPTIC}`$ | $`12`$ | Full MK | $`2^{126.58}`$ | $`2^{148.62}`$ pck. | $`2^{153.8}\text{ bits}`$ |

*Note:* <sup>\*</sup>Quasidifferential (*ToSC* 2026) attacks hold only on weak-key classes (fractions $`2^{-38}`$ and $`2^{-39}`$ of the key space at $`12`$ and $`14`$ rounds); all other results in the table apply in the all-key setting.

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026aradi,
  title  = {Integral Cryptanalysis of {ARADI}},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/ARADI}
}
```
