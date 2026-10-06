# C 组：Known caveats 逐条原文转录

来源：https://github.com/spicylemonade/compensated-magnet-ledger/blob/45551de5fac4e69de3e03c420b31abb335d73ef4/LEDGER.md

作者：Geby Jaff / spicylemonade；数据和文字 CC BY 4.0。下文1–17及全部子项从本次 c-ledger.md 原样抽出，不含编辑删改；取证摘要的中文解释不替代原文。

## Known caveats and method issues

Most of these come from the agents' own adversarial reviews and cross-checks. They are listed so that nobody has to rediscover them.

**Both materials**
1. **Ideal crystals at 0 K.** Every DFT number is for a perfectly ordered crystal at 0 K. Finite-temperature spin disorder was tested only for YBaMnFeO₅ (spin-flip snapshots, Y17–Y18).
   - The zero net spin moment is a 0 K result. At finite temperature two inequivalent sublattices generally lose order at different rates, so a small net magnetisation can appear; compensation is generally exact only at T = 0 (Şaşıoğlu, PRB 79, 100406 (2009)). Room-temperature compensation was not computed for either material.
2. **DFT+U is a choice.** U values are literature-typical, not computed from first principles. Attempts to compute U by linear response (`hp.x`) never produced a result. That is why the U scans (K09–K11, Y05–Y07) and the independent HSE06 checks are part of the ledger.
3. **HSE06 windows come from the self-consistent k-grid, not a dense grid.** For KV[Cr(CN)₆] the 4×4×4 grid includes Γ, X and L. Band extrema off the grid could make windows slightly smaller.
4. **HSE06 energy differences between states of different magnetic symmetry can carry exact-exchange k-set offsets.** Gaps and windows are not affected. The KV[Cr(CN)₆] FM–LCM pair (K18) has the same crystal symmetry.
5. **Spin–orbit numbers were not used.** Earlier spin–orbit runs began from antiparallel non-collinear starting moments, which can trigger a known QE sign issue, so they are marked unverified. Any net moment from spin–orbit coupling or orbital moments is therefore not quantified here. Luttinger counting protects only the spin moment, not orbital moments.

**KV[Cr(CN)₆]**

6. **The real material is not the ideal crystal.** The only sample ever reported (1999) is a hydrated powder, never reproduced. Its saturation moment is 0.125 μB/f.u. instead of 0, and every missing [Cr(CN)₆] unit adds 3 μB. Its ordering temperature, 376 K, fell to 365 K after heat treatment (dehydration suspected). No band-gap, optical, transport or spin-resolved measurement of it has been reported, so the gap and the spin windows are predictions only.
7. **Water: the two methods disagree.**
   - PBE+U says the dihydrate's hole window drops from 2.02 to 0.93 eV (K21).
   - HSE06 says 2.64 → 2.31 eV (K19b; with matched settings, 2.68 → 2.31). The electron window barely moves (1.44 → 1.40 eV, K20b), and both band edges stay in the same spin channel.
   - The agents judged HSE06 more reliable, because PBE+U misplaces the water levels. That is a judgement, not a measurement.
   - The agents' HSE06 hydrate run (F) stopped after 12 exact-exchange cycles, before the loop converged, and the old run index wrongly marked it converged. The same input has now been run to convergence (F2: 41 cycles, converged `!!` energy). Its first 12 cycles reproduce the agents' run digit for digit. Converged values: gap 2.14 eV, windows 2.31 / 1.40 eV (the stopped run gave 2.02 eV and 2.43 / 1.42 eV; the earlier estimate for the hole window was 2.35 eV, range 2.25–2.45).
8. **Defect windows are "in-cell".** Aligning defect cells to the host bands (done by the original collectors, values in `runs/*/results/*.json`) gives smaller numbers in some cases. Examples: antisite hole window 0.23 eV in-cell vs 0.13 eV aligned; PBE+U water-filled-vacancy electron window 0.47 eV in-cell vs 0.07 eV aligned.
   - The 65-atom water-filled-vacancy cell was only partly relaxed. Its PBE+U relaxation (`I_pbeu_vacancy_with_water/KVCr_VAC_w1_relax`) stopped after 3 geometry steps on a time limit, with a residual total force of 0.035 Ry/Bohr against a 0.001 target, and the PBE+U and HSE06 band calculations (K22–K24) used that geometry. The net moment (K22) is fixed by composition; the windows could shift somewhat on full relaxation.
9. **Carriers will be heavy.** The relevant bands are 0.4–0.6 eV wide and a doped hole self-traps (polaron), so do not expect silicon-like transport. No conductivity, optical-gap or spin-resolved measurement exists for this compound.
10. **At room temperature the order is far from perfect.** The sublattice order is about 0.6 at 300 K (T_C = 376 K), so the ideal spin polarisation would be reduced.
11. **The physics is not new in general.** Compensated ferrimagnets are known to have spin-split bands. What is new is identifying this room-temperature compound as one and putting numbers on its band edges. The closest prior work is Schart et al., Inorg. Chem. 63, 22856 (2024), on Cr[Cr(CN)₆]; its windows are small and bipolar (K27–K28).
    - **Prior work on KV[Cr(CN)₆] itself.** It was designed as a compensated ferrimagnet (Holmes & Girolami 1999 expected a saturation moment of zero). Kabalan, Matar, Desplanches, Létard & Zakhour, Chem. Phys. 352, 85 (2008), arXiv:0802.3619, computed an antiparallel state with zero net moment that is an insulator "with a gap opening of ∼1 eV". They used LDA (ASW method) for the crystal; the B3LYP hybrid functional only for two-metal molecular models, to get the exchange constant. Their only figure for the zero-moment state is a total density of states whose two spin channels are mirror images. They report no per-spin gaps, no band-edge spin and no spin splitting.
    - **Middlemiss, Lawton & Wilson, J. Phys.: Condens. Matter 20, 335231 (2008)** is the closest prior study, and the agents' literature search missed it. It computed the same crystal with hybrid functionals (CRYSTAL03, Gaussian basis sets; 35%, 65% and 100% exact exchange), mainly to get exchange constants and ordering temperatures under pressure.
      - Table 3: cell moment −0.003 μB; gap 4.01 eV at 35% exact exchange (2.1 eV when extrapolated to B3LYP's 20%); valence edge 87% V, conduction edge 51% Cr and 45% cyanide.
      - Fig. 3a plots the 35% density of states spin by spin. Both band edges are in the same spin channel (V's), and reading the figure gives windows of about 3.1 eV for holes and 1.5 eV for electrons.
      - The text discusses only the atomic make-up of the edges; it does not comment on their spin, on spin splitting or on spintronics.
      - Independent agreement with this ledger, from a different code: their extrapolated B3LYP gap (2.1 eV) matches our HSE06 gap (2.09 eV, K14), and the windows read from their figure are close to ours (2.64 and 1.57 eV, K16–K17).
    - **What this ledger adds** beyond those papers: pointing out that the compound is a Luttinger-compensated semiconductor with spin-sorted band edges (spin-group identification, K29); explicit spin windows with PBE+U across U and with HSE06 (K14–K17); a second pseudopotential family (tier R); and the water and vacancy tests (K19–K24). The same-spin edges themselves were already visible, unremarked, in Middlemiss et al.'s Fig. 3a.
    - **Context.** The 2025 paper that predicted the cyanide LCM semiconductors Mn(CN)₂ and Co(CN)₂ (Guo et al., arXiv:2502.18136) found Néel temperatures of 210 K and 75 K and wrote that future work needs LCMs with "the transition temperature above the room temperature". Their band edges also sit in opposite spin channels ("bipolarized"); KV[Cr(CN)₆]'s sit in the same one.

**YBaMnFeO₅**

12. **It has never been made.** The key result is negative: the rock-salt Mn/Fe order needed for the effect is predicted to disorder at about 950 K (Y25–Y26). That is below the temperatures at which cations move quickly during standard synthesis.
    - Every chemically similar compound whose B-site arrangement has been determined (GdBaMnFeO₅, NdBaMnFeO₅₊δ, YBaMnCoO₅) is B-site disordered. SmBaMnFeO₅₊δ has been made, but its B-site order is not reported.
    - A single nearest-neighbour Mn/Fe swap collapses the opposite-spin gap (Y19–Y20).
13. **The freeze-out temperature is an analogy.** The ~1150 K below which B-site exchange is taken to freeze comes from a different compound, YBaCuFeO₅ (Morin et al., Nat. Commun. 7, 13758 (2016)), computed with the same protocol.
    - That compound has Jahn–Teller physics that YBaMnFeO₅ lacks.
    - The highest cluster-expansion variant (1210 K) reaches into the synthesis window.
14. **The cluster expansion is provisional.** Weighted leave-one-out error is 39 meV/f.u.
    - A charge-transfer-free variant predicts a different, non-rock-salt ground state, which has not been checked by DFT. If it held, rock-salt order would not be the ground state either.
    - The raw inputs and outputs of its 96 DFT runs, each arrangement's relaxation, and the YBaCuFeO₅ benchmark runs are in `runs/J_cation_order_cluster_expansion_raw/`. `python tools/crosscheck_raw.py` confirms that 183 of the 184 energies recorded in `cation_order_cluster_expansion/results_*/` match those outputs exactly. One saved output (YBMFO_STRp_C, configuration r0) stops just before its final energy, so that single value cannot be re-derived.
    - Two arrangements (MIX_B, MIXap_A) reached the 120-step limit of their relaxation with residual total forces of 0.011–0.016 Ry/Bohr. The agents' parser recorded them as converged, and their single points use those geometries. A third arrangement (SQS_F) has no completed runs and carries no weight in the fit.
15. **The HSE06 YBaMnFeO₅ runs used a 75 Ry cutoff on the 110 Ry geometry.** An earlier HSE06 result (gap 2.32 eV, windows 1.10/1.25 eV) was on a superseded 75 Ry geometry; it is replaced by Y10–Y13.
16. **The Néel temperature is a model estimate.**
    - 417 K (raw classical Monte Carlo) and about 490 K (calibrated on YFeO₃) come from a fitted Heisenberg model.
    - At U = 6 eV the raw value drops to about 320–340 K.
    - PBE+U gets the ground state of the twin compound YBaMn₂O₅ wrong at U ≥ 2 eV. It keeps the right ground state for YBaMnFeO₅ at all U, and HSE06 agrees (Y09).
17. **Hull and phonons.**
    - **The hull distance is +13.7 meV/atom, not +2.6 (corrected while building this ledger).** The agents' value (Y24) used the 28 competing phases that had finished when they computed it. `tools/crosscheck_raw.py` re-reads all 33 completed competing phases from their raw outputs: two that finished later, BaFe₂O₄ and Ba₆Y₂Fe₄O₁₅, lower the hull, giving +13.7 meV/atom (Y24b). Both numbers are recomputed from the raw files in `runs/K_hull_competing_phases_raw/`.
    - One more known phase, Ba₂Fe₂O₅, never finished (its job timed out after 22 hours). Missing competitors can only raise the distance.
    - For scale, the median distance above the hull for known inorganic crystals in a large DFT survey is about 15 meV/atom (Sun et al., Sci. Adv. 2, e1600225 (2016)). So +13.7 meV/atom does not rule the compound out, but "at the edge of stability" overstated it.
    - Single-cycle vc-relax energies should be re-relaxed once before precise reuse.
    - Phonons are real at every exactly computed q-point (Γ, Z, X, Y, M). Fourier-interpolated branches between them show imaginary values in the 1×1×2 supercell, which the agents attributed to interpolation; larger supercells were not run (`analysis_data/phonon110_summary.json`).


