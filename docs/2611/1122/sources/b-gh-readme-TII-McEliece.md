# Primal-Shortened ISD Key Recovery on Binary Goppa Codes

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Code--Based%20PKE-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Key%20Recovery%2C%20Shortened%20ISD-blue?style=flat-square" alt="Tags" />
</p>

Based on binary Goppa codes, the McEliece public-key cryptosystem has stood for nearly five decades, and its two underlying hard problems, syndrome decoding and structural key recovery, form the mathematical foundation of the NIST post-quantum finalist Classic McEliece.
To benchmark concrete key-recovery security margins across code parameters, the Technology Innovation Institute (TII) established the public **TII McEliece Challenges**, comprising $`206`$ Track-2 public-key recovery instances ($`\lambda_{\text{TII}} \in [22, 255]`$). Recent algebraic attacks, including shortened-dual methods (*CRYPTO 2026*) and primal hold-out solvers (Anthropic), had solved $`41`$ instances with $`\lambda_{\text{TII}} \ge 83`$, requiring hours to days on large CPU or GPU clusters for the hardest cases.

We propose a new primal-shortened Information-Set Decoding ($`\mathsf{ISD}`$) key-recovery algorithm for binary Goppa codes. Across the official Track-2 instances ($`\lambda_{\text{TII}} \in [83, 254]`$), our method recovers $`94`$ private keys in total, solving $`55`$ previously unsolved instances. On previously solved challenges, it improves upon the fastest existing records, including those from *CRYPTO 2026* and Anthropic, with wall-time speedups of up to $`43{,}400\times`$. NIST Classic McEliece remains unaffected.

## Results

| Challenge | Parameters $`(n, m, t, k)`$ | Prior Wall Time | Our Wall Time | Our Compute | Method | Result |
|---|---|---|---|---|---|---|
| `TII-83` | $`(253, 8, 5, 213)`$ | $`24.3\text{ s}`$ | $`1.58\text{ s}`$ | $`0.028\text{ core-h}`$ | Shortened ISD | $`15.4\times`$ speedup |
| `TII-125` | $`(507, 9, 7, 444)`$ | $`9.47\text{ h}`$ | $`5.90\text{ s}`$ | $`0.105\text{ core-h}`$ | Shortened ISD | $`5{,}778\times`$ speedup |
| `TII-166` | $`(500, 9, 7, 437)`$ | $`9.20\text{ h}`$ | $`5.36\text{ s}`$ | $`0.095\text{ core-h}`$ | Shortened ISD | $`6{,}180\times`$ speedup |
| `TII-210` | $`(491, 9, 7, 428)`$ | $`4.31\text{ h}`$ | $`6.04\text{ s}`$ | $`0.107\text{ core-h}`$ | Shortened ISD | $`2{,}568\times`$ speedup |
| `TII-223` | $`(492, 9, 9, 411)`$ | $`47.75\text{ h}`$ | $`39.9\text{ s}`$ | $`0.709\text{ core-h}`$ | Shortened ISD | $`4{,}308\times`$ speedup |
| `TII-252` | $`(1008, 10, 11, 898)`$ | $`375\text{ s}`$ | $`90.0\text{ s}`$ | $`0.025\text{ core-h}`$ | Frobenius HOVER | $`4.17\times`$ speedup |
| `TII-253` | $`(214, 8, 9, 142)`$ | $`21.7\text{ h}`$ | $`1.80\text{ s}`$ | $`0.032\text{ core-h}`$ | Shortened ISD | $`43{,}400\times`$ speedup |
| `TII-254` | $`(223, 8, 12, 127)`$ | $`16.0\text{ h}`$ | $`30.4\text{ s}`$ | $`0.540\text{ core-h}`$ | Shortened ISD | $`1{,}896\times`$ speedup |
| `TII-165` / `232` / `245` | $`mt \in [104, 112]`$ | *Unsolved* | $`81.2\text{ s}`$–$`41.8\text{ min}`$ | $`1.44`$–$`44.62\text{ core-h}`$ | Shortened ISD | First-Blood |
| `TII-228` / `TII-250` | $`mt \in \{126, 120\}`$ | *Unsolved* | $`57.1\text{ min}`$ / $`9.20\text{ h}`$ | $`2{,}463`$ / $`66{,}722\text{ core-h}`$ | Shortened ISD | First-Blood |

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026tiimceliece,
  title  = {Primal-Shortened {ISD} Key Recovery on Binary {Goppa} Codes},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/TII-McEliece}
}
```
