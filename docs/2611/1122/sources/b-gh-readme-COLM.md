# Two-Query Nonce-Respecting Forgeries on $`\mathsf{COLM}_0`$

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./artifacts/"><img src="https://img.shields.io/badge/Artifacts-Code-brightgreen?style=flat-square" alt="Artifacts" /></a>
  <img src="https://img.shields.io/badge/Category-AEAD-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Ciphertext%20Forgery-blue?style=flat-square" alt="Tags" />
</p>

Authenticated encryption schemes designed for *defense in depth* aim to preserve confidentiality and integrity even under operational pitfalls such as nonce misuse. $`\mathsf{COLM}`$ was selected into the CAESAR Final Portfolio (second choice for Use Case 3: *Defense in Depth*) and specified in the *Journal of Cryptology* (2024) with two recommended modes: the primary single-tag mode $`\mathsf{COLM}_0`$ and the intermediate-tag mode $`\mathsf{COLM}_{127}`$, claiming $`64`$-bit integrity for both. In the standard nonce-respecting, encryption-only ($`\mathsf{INT\text{-}CTXT}`$) setting, $`\mathsf{COLM}`$ had no known attacks faster than the $`2^{64}`$ birthday bound.

By combining cross-IV ciphertext splicing with decryption checksum telescoping, we present the first practical forgery attack on $`\mathsf{COLM}_0`$ in the standard nonce-respecting model. This breaks its claimed $`64`$-bit integrity security with just $`2`$ encryption queries and no decryption oracle access.

## Results

| Method | Target | Nonce | Oracle Model | Queries | Blocks | Success Prob. |
|---|---|---|---|---|---|---|
| Birthday Bound (2017) | $`\mathsf{COLM}_0, \mathsf{COLM}_{127}`$ | Unique | Encryption only | $`2^{64}`$ | $`2^{65}`$ | $`\Theta(1)`$ |
| Difference-Fade Splice | $`\mathsf{COLM}_0`$ | Unique | Encryption only | $`2`$ | $`262`$ | $`1`$ |
| State-Transfer Splice | $`\mathsf{COLM}_0`$ | Unique | Encryption only | $`2`$ | $`260`$ | $`1`$ |

*Note:* Prior below-birthday forgery analyses (*ToSC* 2017) rely on Release of Unverified Plaintext ($`\mathsf{INT\text{-}RUP}`$) decryption queries, assuming the decryption oracle returns full unverified plaintext blocks from rejected ciphertexts directly to the adversary. Because this $`\mathsf{INT\text{-}RUP}`$ setting lies outside $`\mathsf{COLM}_0`$'s standard security model, those results are omitted from the table above.

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{minimax2026colm,
  title  = {Two-Query Nonce-Respecting Forgeries on {$\mathsf{COLM}_0$}},
  author = {MiniMax},
  year   = {2026},
  url    = {https://github.com/MiniMax-AI/Research/tree/main/Cryptography/COLM}
}
```
