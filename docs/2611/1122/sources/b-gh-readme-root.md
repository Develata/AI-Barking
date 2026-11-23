<div align="center">
  <img width="60%" src="figures/logo.svg" alt="MiniMax">
</div>
<hr>

In this repository, we plan to share research projects and findings from MiniMax internal models.

## Cryptography

The [`Cryptography`](./Cryptography/) directory contains results in cryptography and cryptanalysis, along with papers and supporting artifacts. These results were primarily discovered by our internal model.

- **Released on November 20, 2026**: Cryptanalysis Results.

  > These findings are released as part of our auto-cryptanalysis project by the MiniMax Cyber Team: **Lili Tang**<sup>\*</sup>, **Hao Jiang**<sup>\*</sup>, **Yansong Feng**<sup>\*</sup>, **Tianyi Li**<sup>†</sup>, and **Guancheng Li**<sup>†</sup> (<sup>\*</sup>Core Contributors; <sup>†</sup>Project Leads). We hope these results offer useful insights into automated cryptanalysis and help strengthen the security of cryptographic schemes. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io).

  | Target | Category | Key Result | Artifacts |
  | :--- | :--- | :--- | :--- |
  | [**COLM**](./Cryptography/COLM/) | AEAD | Nonce-respecting ciphertext forgery on CAESAR winner $`\mathsf{COLM}_0`$ with $`2`$ encryption queries | [Paper](./Cryptography/COLM/paper/main.pdf) · [Code](./Cryptography/COLM/artifacts/) |
  | [**TII McEliece**](./Cryptography/TII-McEliece/) | Code-Based PKE | $`94`$ Track-2 private keys recovered on TII McEliece challenges ($`\lambda_{\text{TII}} \in [83, 254]`$), including $`55`$ first-blood solutions and up to $`43{,}400\times`$ speedups | [Paper](./Cryptography/TII-McEliece/paper/main.pdf) · [Code](./Cryptography/TII-McEliece/artifacts/) |
  | [**ARADI**](./Cryptography/ARADI/) | Block Cipher | Key recovery up to $`8`$ rounds ($`\le 16\text{ s}`$), $`2^{54.8}`$ $`9`$-round attack, and improved $`10`$–$`12`$-round all-key integral attacks on NSA block cipher ARADI | [Paper](./Cryptography/ARADI/paper/main.pdf) · [Code](./Cryptography/ARADI/artifacts/) |
  | [**SBC-VOLE**](./Cryptography/SBC-VOLE/) | Post-Quantum Signature | Passive $`1`$-signature secret-key recovery in $`0.06\text{ s}`$ across all $`\mathsf{SBC}_{\mathsf{VOLE}}`$ and $`\mathsf{SBC}_{\mathsf{VOLE}}^{\mathsf{M}}`$ parameter sets | [Paper](./Cryptography/SBC-VOLE/paper/main.pdf) · [Code](./Cryptography/SBC-VOLE/artifacts/) |
  | [**HAWK**](./Cryptography/HAWK/) | Post-Quantum Signature | Secret-key recovery on `HAWK-256` in $`30.9\text{ min}`$ ($`690\times`$ speedup) and $`9.3`$–$`23.7`$-bit security reductions across `HAWK-256/512/1024` | [Paper](./Cryptography/HAWK/paper/main.pdf) · [Code](./Cryptography/HAWK/artifacts/) |
  | [**Blender**](./Cryptography/Blender/) | Hash Function | Collisions on `Blender-256` ($`0.73\text{ s}`$) and $`2^{35.9}`$ preimage attack | [Paper](./Cryptography/Blender/paper/main.pdf) · [Code](./Cryptography/Blender/artifacts/) |
  | [**LOCKER & ROLLO**](./Cryptography/LOCKER-ROLLO/) | Post-Quantum KEM | Passive ciphertext recovery breaking all $`9`$ `LOCKER` sets (`LOCKER-I` in $`1.15\text{ s}`$) and private-key recovery on `2019 ROLLO-I-128` ($`5.97\text{ s}`$) | [Paper](./Cryptography/LOCKER-ROLLO/paper/main.pdf) · [Code](./Cryptography/LOCKER-ROLLO/artifacts/) |
  | [**Waterfall**](./Cryptography/Waterfall/) | Hash Function | Practical collisions on `Waterfall-224/256/384/512` within 1 second | [Paper](./Cryptography/Waterfall/paper/main.pdf) · [Code](./Cryptography/Waterfall/artifacts/) |

- **Released on October 23, 2026**: Tokareva's Conjectures. We construct explicit counterexamples disproving Tokareva's bent-sum conjecture (even $`n \ge 8`$) and derivative conjecture (even $`n \ge 10`$) on Boolean bent functions ([BentSum](./Cryptography/BentSum/), [arXiv:2610.32239](https://arxiv.org/abs/2610.32239)), with all proofs formally verified in [Lean 4](./Cryptography/BentSum/lean/).

## License

This repository is licensed under the Apache License, Version 2.0; see [`LICENSE`](LICENSE).
