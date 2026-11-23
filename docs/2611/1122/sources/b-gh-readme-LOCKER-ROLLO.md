# Algebraic Cryptanalysis of LOCKER and ROLLO

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Post--Quantum%20KEM-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Key%20Recovery%2C%20Algebraic%20MinRank-blue?style=flat-square" alt="Tags" />
</p>

**LOCKER** and its merged successor **ROLLO** (`ROLLO-I`, `ROLLO-II`, `ROLLO-III`) are Ideal Low-Rank Parity-Check ($`\mathsf{LRPC}`$) key encapsulation mechanisms over $`R = \mathbb{F}_{2^m}[X]/(P(X))`$ that advanced to Round 2 of the NIST Post-Quantum Cryptography Standardization Process as one of the $`17`$ public-key/KEM candidates. In CryptanalysisBench (*NeurIPS 2026*), evaluated models bypassed `LOCKER` only via a reference-implementation spec-versus-code defect (arXiv:2607.18538v2, p. 10). Once that implementation flaw is patched, solving the rank-metric decoding problem of full-strength `LOCKER-I` via prior methods (*EUROCRYPT 2020*, *ASIACRYPT 2020*, and 2026 rank-metric analyses) requires a huge matrix of $`42\text{ PB}`$ RAM and $`2^{58.2}`$ gates, with no full-parameter practical key recovery previously demonstrated.

We exploit the transposed partial Frobenius automorphism on $`R \cong \mathbb{F}_{2^n}[T]/(g(T))`$ to collapse the algebraic MinRank system without unfolding the extension field. This algebraic symmetry generates a full orbit of conjugate linear equations directly from a single passive ciphertext or public key.
On full-strength `LOCKER-I`, our Schur-complement reduction compresses the elimination matrix $`700\text{ million-fold}`$ from $`42\text{ PB}`$ to $`58\text{ MB}`$, recovering the shared secret in several seconds and breaking all $`9`$ parameter sets of 2017 `LOCKER`.
Furthermore, on the 2019 NIST Round-2 initial specification, our attack recovers an equivalent `ROLLO-I-128` private key in $`5.97\text{ s}`$ on a single CPU core.
For `ROLLO-III-128/192`, our public-key recovery attack costs $`2^{39.3}`$ and $`2^{57.6}`$ gates.

## Results

| Parameter Set | Claimed Sec. | Prior Best | Our Cost | Runtime | Memory |
|---|---|---|---|---|---|
| `LOCKER-I` | $`128`$-bit | $`2^{58.2}`$ gates | $`2^{45.6}`$ gates | $`1.15`$–$`2.15\text{ s}`$ | $`58\text{ MB}`$ |
| `LOCKER-II` | $`192`$-bit | $`2^{57.5}`$ gates | $`2^{41.0}`$ gates | $`0.48\text{ s}`$ | — |
| `LOCKER-III` / `VI` / `IX` | $`256`$-bit | $`2^{69.6}`$–$`2^{79.3}`$ gates | $`2^{58.8}`$–$`2^{60.5}`$ gates | — | $`1.65`$–$`2.35\text{ GB}`$ |
| `LOCKER-IV` / `VII` / `VIII` | $`128`$–$`192`$-bit | $`2^{60.8}`$–$`2^{66.2}`$ gates | $`2^{42.7}`$–$`2^{45.6}`$ gates | $`1.33`$–$`6.91\text{ s}`$ | — |
| `2019 ROLLO-I-128` | $`128`$-bit | $`2^{50.0}`$ gates | $`2^{31.3}`$ gates | $`5.97\text{ s}`$ | $`< 2\text{ MB}`$ |
| `2019 ROLLO-III-128` | $`128`$-bit | $`2^{49.4}`$ gates | $`2^{39.3}`$ gates | — | — |
| `2019 ROLLO-II-192` | $`192`$-bit | $`2^{78.7}`$ gates | $`2^{60.3}`$ gates | — | — |

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026lockerrollo,
  title  = {Algebraic Cryptanalysis of {LOCKER} and {ROLLO}},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/LOCKER-ROLLO}
}
```
