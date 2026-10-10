-- 取自 openai/NavierStokesAndEuler @ f9e8bc5b38b6e212696e8a30e3e91517af887bbd :: NavierStokes/SmoothFourierData.lean 行 68-68（git show 原文，逐字；本行注释为存档说明）
noncomputable def xJet (p : ℕ) (f : Plane → ℂ) : Plane → ℂ := partialX^[p] f
