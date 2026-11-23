# Improved Practical Key Recovery on HAWK

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Post--Quantum%20Signature-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Key%20Recovery%2C%20Optimization-blue?style=flat-square" alt="Tags" />
</p>

**HAWK** (*ASIACRYPT 2022*) is a post-quantum lattice signature scheme based on the Module Lattice Isomorphism Problem ($`\mathsf{Module\text{-}LIP}`$) and a Round-3 candidate in the NIST Post-Quantum Cryptography Additional Digital Signature Standardization Process. Recently, researchers from Anthropic (*ePrint 2026/1593*) introduced a dimension-halving $`\tau`$-cocycle lattice attack that recovered the secret key of `HAWK-256` ($`n = 256`$) in $`3\text{ hours } 42\text{ minutes}`$ on a $`96`$-core server ($`\approx 1{,}278{,}720`$ CPU-core-seconds, with lattice sieve dimension $`d = 118`$), following which the scheme was withdrawn from the NIST standardization process.

We introduce an algebraic sublattice refinement and exact Cholesky preconditioning pipeline that reduces the estimated security of all HAWK parameter sets. Enforcing the public parity congruence $`V_\tau \equiv I_2 \pmod 2`$ before lattice reduction yields an index-$`2^{n-1}`$ sublattice $`\Lambda' \subset \Lambda_\tau`$ where $`\pm V_\tau`$ is the unique shortest vector pair. Across `HAWK-256`, `HAWK-512`, and `HAWK-1024`, this lowers the expected BKZ block size $`\beta`$ by $`\Delta\beta = 34, 49,`$ and $`81`$ ($`92 \to 58`$, $`205 \to 156`$, and $`432 \to 351`$), reducing their estimated $`\mathsf{AGPS20}`$ security by $`9.28`$, $`11.15`$, and $`23.74\text{ bits}`$. For `HAWK-256`, our implementation recovers an equivalent secret key on a single CPU core in $`30.9\text{ min}`$ ($`690\times`$ less CPU compute than the $`96`$-core baseline).

## Results

| Parameter Set | Prior Cost | Our Cost | Security Gain |
|---|---|---|---|
| `HAWK-256` | $`2^{50.83}`$ gates | $`2^{41.56}`$ gates | $`-9.28\text{ bits}`$ |
| `HAWK-512` | $`2^{80.79}`$ gates | $`2^{69.64}`$ gates | $`-11.15\text{ bits}`$ |
| `HAWK-1024` | $`2^{146.48}`$ gates | $`2^{122.73}`$ gates | $`-23.74\text{ bits}`$ |

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026hawk,
  title  = {Improved Practical Key Recovery on {HAWK}},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/HAWK}
}
```
