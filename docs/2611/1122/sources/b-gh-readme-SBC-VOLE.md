# Single-Signature Key Recovery on the $`\mathsf{SBC}_{\mathsf{VOLE}}`$ Signature Scheme

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-Post--Quantum%20Signature-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Key%20Recovery-blue?style=flat-square" alt="Tags" />
</p>

Post-quantum signature schemes in the VOLE-in-the-Head ($`\mathsf{VOLEitH}`$) paradigm combine compact public keys with fast, symmetric-key-based signing and verification. Introduced at *ASIACRYPT 2025*, $`\mathsf{SBC}_{\mathsf{VOLE}}`$ and its implicit MinMax commitment variant $`\mathsf{SBC}_{\mathsf{VOLE}}^{\mathsf{M}}`$ instantiate $`\mathsf{VOLEitH}`$ over the Subfield Bilinear Collision ($`\mathsf{SBC}`$) problem. At the claimed $`128`$-bit $`\mathsf{EUF\text{-}CMA}`$ security level, both schemes feature a $`48`$-byte public key, reduce signing time by $`90\%`$ compared to their *CRYPTO 2024* $`\mathsf{MPCitH}`$ predecessor ($`\approx 2.2\text{ ms}`$), and compress signatures to as little as $`2{,}658\text{ bytes}`$. Prior cryptanalysis of the underlying $`\mathsf{SBC}`$ problem required at least $`2^{128}`$ operations (2026).

We expose a structural flaw in the protocol's signature-compression optimization and in its $`\mathsf{HVZK}`$ simulation proof. The verifier can publicly reconstruct punctured $`\mathsf{VOLE}`$ evaluations $`\vec{F}_x(\Delta^*)`$ and $`\vec{F}_y(\Delta^*)`$ and use them to cancel pairwise quadratic secret terms in $`(\vec{x}, \vec{y})`$ over $`\mathbb{F}_2`$. This reduces to $`257`$ linear equations over $`\mathbb{F}_2`$ in the $`256`$-bit secret key, enabling full secret-key recovery from $`1`$ passive signature in $`0.06\text{ s}`$ across all proposed parameter sets.

## Results

| Method | Target | Result Type | Signatures | Time Complexity |
|---|---|---|---|---|
| Prior $`\mathsf{SBC}`$ Algebraic Analysis (2026) | $`\mathsf{SBC}`$ problem | Key recovery | $`0`$ | $`\ge 2^{128.00}`$ ops |
| Prior MinMax Analysis (2025) | $`\mathsf{SBC}_{\mathsf{VOLE}}^{\mathsf{M}}`$ | Key-only forgery | $`0`$ | $`< 2^{128.00}`$ ops |
| Linearized $`B`$-Coefficient Attack | $`\mathsf{SBC}_{\mathsf{VOLE}}, \mathsf{SBC}_{\mathsf{VOLE}}^{\mathsf{M}}`$ | Key recovery | $`1`$ | $`2^{25.24}`$ bit ops |
| Witness-Free Simulator Distinguisher | $`\mathsf{SBC}_{\mathsf{VOLE}}, \mathsf{SBC}_{\mathsf{VOLE}}^{\mathsf{M}}`$ | $`\mathsf{HVZK}`$ distinguisher | $`1`$ | $`2^{25.24}`$ bit ops |

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026sbcvole,
  title  = {Single-Signature Key Recovery on the {$\mathsf{SBC}_{\mathsf{VOLE}}$} Signature Scheme},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/SBC-VOLE}
}
```
