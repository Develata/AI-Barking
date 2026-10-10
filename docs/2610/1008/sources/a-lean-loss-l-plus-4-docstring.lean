-- 取自 openai/NavierStokesAndEuler @ f9e8bc5b38b6e212696e8a30e3e91517af887bbd :: NavierStokes/SmoothFamilyTorusInverse.lean 行 724-726（git show 原文，逐字；本行注释为存档说明）
/-- The loss l+4 comes only from the order-l multiplier and the summable
two-dimensional lattice majorant; it does not grow with jet order. -/
theorem norm_jet_applyMultiplier_le (n l : ℕ) {m : Frequency → ℂ}
