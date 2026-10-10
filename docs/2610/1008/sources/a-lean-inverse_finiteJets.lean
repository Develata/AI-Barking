-- 取自 openai/NavierStokesAndEuler @ f9e8bc5b38b6e212696e8a30e3e91517af887bbd :: NavierStokes/SmoothFamilyTorusInverse.lean 行 863-875（git show 原文，逐字；本行注释为存档说明）
/-- Uniform full-tensor bound with five torus derivatives lost. The
constant is chosen before the source, parameter set, or input bound. -/
theorem inverse_finiteJets (d : Direction) (n : ℕ) :
    ∃ K : ℝ, 0 ≤ K ∧ ∀ (f : Source P) (S : Set P) (C : ℝ),
      ContDiff ℝ ∞ f → Periodic f → 0 ≤ C →
      JetBound f S (n + 5) C → JetBound (inverse d f) S n (K * C) := by
  obtain ⟨K, hK, hb⟩ := applyMultiplier_finiteJets (P := P) n 1
  refine ⟨K * (6 * ‖omega⁻¹‖), by positivity, ?_⟩
  intro f S C hf hp hC h
  have hh := hb (multiplier d) (6 * ‖omega⁻¹‖) (by positivity)
    (fun k => by simpa only [pow_one] using norm_multiplier_le d k)
    f S C hf hp hC (by simpa only [Nat.add_assoc] using h)
  exact hh
