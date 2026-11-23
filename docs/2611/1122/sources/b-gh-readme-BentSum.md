# BentSum

<p align="center">
  <a href="./paper/main.pdf"><img src="https://img.shields.io/badge/Paper%20Report-PDF-b31b1b?style=flat-square" alt="Paper Report" /></a>
  <a href="./lean/"><img src="https://img.shields.io/badge/Formalization-Lean%204-brightgreen?style=flat-square" alt="Formalization" /></a>
  <img src="https://img.shields.io/badge/Category-Boolean%20Functions-yellow?style=flat-square" alt="Category" />
  <img src="https://img.shields.io/badge/Tags-Bent%20Functions%2C%20Formal%20Proof-blue?style=flat-square" alt="Tags" />
</p>

Counterexamples to two conjectures of Tokareva on bent functions:

- the **bent-sum conjecture**: for even $`n`$, every Boolean function on $`\mathbb{F}_2^n`$ of
  algebraic degree at most $`n/2`$ is a sum of two bent functions;
- the **derivative conjecture**: for even $`n`$, every balanced Boolean function of algebraic
  degree at most $`n/2-1`$ with a nonzero period is a derivative of a bent
  function.

The bent-sum conjecture is known to hold for $`n\le 6`$, so $`n=8`$ is the
smallest dimension in which it fails.

## Results

1. **Weight congruence (Theorem 3.2).** If $`f`$ is a sum of two bent functions
   on $`\mathbb{F}_2^n`$ and $`\mathrm{supp}(f)`$ lies in an affine subspace
   of codimension $`k < n/2`$, then $`2^{k+1}`$ divides $`\mathrm{wt}(f)`$.
   Corollary 3.4 specializes this to cylinder functions.
2. **Counterexamples to the bent-sum conjecture (Theorem 4.2).** For every even
   $`n=2m\ge 8`$, the function
   $`f_n=x_1\cdots x_{m-2}\thinspace q_3(x_{m-1},\dots,x_{m+4})`$ has degree $`m`$
   and is not a sum of two bent functions. In dimension $`8`$ this is
   $`f_8=x_1x_2(x_3x_4+x_5x_6+x_7x_8)`$, whose weight $`28`$ is not divisible
   by $`8`$.
3. **Counterexamples to the derivative conjecture (Theorem 4.7).** For every even
   $`n\ge 10`$, the function $`h_n=x_2+f_{n-2}`$ is balanced, has degree $`n/2-1`$ and
   a nonzero period, but is not a derivative of a bent function. The proof uses
   Shaporenko's reduction of derivatives to bent sums (Lemma 4.6), which turns
   the counterexamples of item 2 into explicit ones.

The paper documents in its *Use of AI* section the large-scale automated
reasoning that produced the results.

## Lean formalization

Needs elan; the toolchain is `leanprover/lean4:v4.35.0-rc3` and Mathlib is
pinned to `v4.35.0-rc3`.

```bash
cd lean
lake exe cache get   # first time only
lake build
```

The main results are `BentSum.IsBentSum.two_pow_dvd_wt` (the weight
congruence), `BentSum.not_bentSumConjecture` and
`BentSum.not_derivativeConjecture`. The proofs contain no `sorry` and no
`native_decide`; the only axioms they use are `propext`, `Classical.choice`
and `Quot.sound`. [`lean/README.md`](./lean/README.md) describes how to print
the axioms locally, the correspondence with the paper, and the few points
where the formal statements are weaker.

## Contact and Citation

While we have verified the main technical claims, the manuscripts may still contain minor bugs or typos and are not yet intended as polished academic papers. We welcome further discussions from the cryptography community, respect concurrent work in this space, and are open to collaborations. You can reach out to us by email at [cyber@minimax.io](mailto:cyber@minimax.io) or directly contact the core contributors.

```bibtex
@misc{feng2026counterexampletokarevasconjecturesums,
  title         = {A counterexample to {Tokareva}'s conjecture on sums of two bent functions},
  author        = {Yansong Feng and Hao Jiang and Guancheng Li and Tianyi Li and Lili Tang},
  year          = {2026},
  eprint        = {2610.32239},
  archivePrefix = {arXiv},
  primaryClass  = {math.CO},
  url           = {https://arxiv.org/abs/2610.32239}
}
```
