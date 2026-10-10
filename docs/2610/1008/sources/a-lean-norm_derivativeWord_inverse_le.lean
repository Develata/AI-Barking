-- 取自 openai/NavierStokesAndEuler @ f9e8bc5b38b6e212696e8a30e3e91517af887bbd :: NavierStokes/SmoothFamilyTorusInverse.lean 行 1058-1080（git show 原文，逐字；本行注释为存档说明）
omit [FiniteDimensional ℝ P] in
theorem norm_derivativeWord_inverse_le (d : Direction) {f : Source P}
    (hf : ContDiff ℝ ∞ f) (hp : Periodic f) (w : List Bool) (p : P) (Y : Plane) {C : ℝ}
    (hzero : ∀ x ∈ Icc (0 : ℝ) 1, ∀ y ∈ Icc (0 : ℝ) 1, ‖f (p, (x, y))‖ ≤ C)
    (hfirst : ∀ x ∈ Icc (0 : ℝ) 1, ∀ y ∈ Icc (0 : ℝ) 1,
      ‖SmoothFourierData.xJet (w.length + 5) (slice f p) (x, y)‖ ≤ C)
    (hsecond : ∀ x ∈ Icc (0 : ℝ) 1, ∀ y ∈ Icc (0 : ℝ) 1,
      ‖SmoothFourierData.xJet (w.length + 5)
        (SmoothFourierData.swapFunction (slice f p)) (x, y)‖ ≤ C) :
    ‖derivativeWord w (slice (inverse d f) p) Y‖ ≤
      ParametricTorusInverse.mixedLossConstant w.length * C := by
  have hbound := SmoothFourierData.coefficient_seminorm_bound (slice_smooth hf p) (hp p)
    (w.length + 1) hzero (by simpa only [Nat.add_assoc] using hfirst)
    (by simpa only [Nat.add_assoc] using hsecond)
  calc
    _ ≤ ((6 * ‖omega⁻¹‖) * ‖omega‖ ^ w.length) *
        coeffSeminorm (w.length + 1) (coefficient f p) :=
      inverse_derivativeWord_bound d
        (SmoothFourierData.rapid_coefficient (slice_smooth hf p) (hp p)) w Y
    _ ≤ ((6 * ‖omega⁻¹‖) * ‖omega‖ ^ w.length) *
        ((3 ^ ((w.length + 1) + 4) * C) * ∑' k : Frequency, (weight k ^ 4)⁻¹) :=
      mul_le_mul_of_nonneg_left hbound (by positivity)
    _ = _ := by simp only [ParametricTorusInverse.mixedLossConstant, Nat.add_assoc]; ring
