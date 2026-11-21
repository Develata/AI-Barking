# The Lean formalization and the paper

This folder contains a Lean 4 formalization (Lean v4.33.1, with Mathlib) of the results of the paper *Finite-time
blow-up for the Navier–Stokes equations* by L. Alpöge and T. Buckmaster (`../paper/finite-time-blow-up-navier-stokes.pdf`). The
formalization came first: a first draft of the paper was obtained from the Lean proof, and the paper was then
rewritten, with standard results cited in place of proofs and with simpler arguments in several places. The
proofs here therefore do not follow the proofs of the paper. What the two have in common is their statements.
The theorems proved here are those of the paper, once the passage between the two forms is made by standard
facts, which are recalled below. In each case the formal statement and the paper's differ in what they write,
not in mathematical content.

## How the package is organized

The statement files at the top of this folder (`Challenge.lean`, `CertificateChallenge.lean`,
`ExtendedCertificateChallenge.lean`, `CoerciveChallenge.lean`, `ExtendedCoerciveChallenge.lean`,
`CellLayerChallenge.lean` with `CellLayerTheorems.lean`) give the definitions and state the theorems, with `sorry`
in place of their proofs. `Solution.lean` proves these theorems from the library in `Breakdown/`. Each file
`comparator*.json` pairs a statement file with `Solution.lean`, names the theorems whose statements must agree,
and permits only the axioms `propext`, `Quot.sound` and `Classical.choice`. `NOTICE` says in words what is
proved. The files whose names begin with `Fractional` concern equations with fractional dissipation and are not
used by the paper.

## Theorem 1.1

`Challenge.lean` states `breakdown_on_space` and `breakdown_on_torus`. Their hypothesis,
`PeriodicSimilarityProfileExists`, is the existence of a profile in the sense of the paper: smooth functions
$W$ and $q$ of $(s,y)$ that solve the self-similar Navier–Stokes equations, with $W$ divergence-free and not
identically zero, and $W(s+P,y)=R\,W(s,R^{-1}y)$ for a period $P>0$ and a linear isometry $R$. The formal
hypothesis asks for more bounds than display (1.3) of the paper, namely
$|\nabla^jW(s,y)|\le C(1+|y|)^{-1-j}$ for $j=0,1,2$ and $|\nabla q(s,y)|\le C(1+|y|)^{-3}$. The bounds on
$\nabla W$ and $\nabla^2W$ follow from the bound on $W$ by an interior estimate (Remark 1.2 of the paper), and the
bound on $\nabla q$ is a standard consequence of the bounds on $W$, since the equation determines the pressure
gradient of a profile.

The conclusions are `BreakdownOnSpace` and `BreakdownOnTorus`. For every viscosity $\nu>0$, the first gives a
smooth, compactly supported, divergence-free datum on $\mathbb R^3$ for which the Navier–Stokes equations have no
solution that is smooth on $[0,\infty)\times\mathbb R^3$ and has bounded energy. The second gives a smooth
divergence-free datum of period one in each coordinate for which they have no solution that is smooth on
$[0,\infty)\times\mathbb R^3$ and whose velocity keeps this periodicity, whatever the pressure. Theorem 1.1 of
the paper is stated for viscosity one and for the solutions that it constructs: they are smooth up to a finite
time, at which they blow up at the rate $(T-t)^{-1/4}$ and converge to the profile in self-similar variables.

The passage from the paper's form to the formal one uses three standard facts. First, the scaling
$(u,p)\mapsto(\nu u(\nu t,x),\nu^2p(\nu t,x))$ carries the equations with viscosity one to those with viscosity
$\nu$. Second, on $\mathbb R^3$ a smooth solution of finite energy with a compactly supported datum is
determined by its datum and stays bounded in $H^1$ on every interval on which it is smooth (Tao, Anal. PDE 6
(2013), Corollaries 11.1 and 4.3 and Theorem 5.4). A global smooth solution of bounded energy would therefore
coincide with the solution of Theorem 1.1, which blows up. Third, on the torus the pressure of a smooth solution
with periodic velocity is periodic up to a term $x\cdot a(t)+C(t)$ (Tao, Lemma 4.1(ii)). A Galilean change of
frame that leaves the datum unchanged makes it periodic (Tao, (33)), and a smooth periodic solution with periodic
pressure is determined by its datum (Tao, Theorem 5.1). The paper does not write out this passage; the formal
proof contains it. Conversely, the rate and the convergence to the profile, which Theorem 1.1 gives, are not
part of the formal statements.

## Theorem 7.1 and Corollary 7.2

`ExtendedCertificateChallenge.lean` defines a certificate (`IsExtendedCertificate`) and states
`extended_certificate_orbit`: a certificate gives `WeakPeriodicSimilarityProfileExists`.
`CertificateChallenge.lean` states the same for an earlier definition (`certificate_orbit`). A weak profile in the
sense of `Challenge.lean` is a field $W$ that is bounded in $L^6$ and has its gradient bounded in $L^2$,
uniformly in $s$, with second derivatives square integrable over a period. It solves the equation of a profile in
the sense of distributions, tested against divergence-free fields, is periodic up to a linear isometry, and is not
identically zero. There are two differences with the paper.

The first is in the definitions. The formal certificate asks at least as much as the definition of the paper
(Subsections 7.1 to 7.3). Among other things, it asks that the approximate profiles be continuously
differentiable in all variables with a weak second derivative that satisfies a third decay bound, that the
residual be one explicit field, and that the fields $e_k$ be orthonormal with weights. There are thirteen such differences. This is a comparison of two
definitions, made by reading them, and not a formal statement.

The second is in the conclusions, and here the formal statement leaves out a step that the paper takes.
Theorem 7.1 states that the solution found near the approximate profile is a smooth profile, with the identity as
its isometry, with explicit bounds on its distance to the approximate profile, and with the symmetries that the
certificate prescribes. The formal statement stops at a weak profile: it records neither that the solution is
close to the candidate nor that it is smooth. The paper proves the smoothness in Proposition 7.11, by standard
regularity theory for the Navier–Stokes equations: the solution satisfies the integral formulation of the
equations, it is smooth by the regularity theory of Koch, Nadirashvili, Seregin and Šverák, and it has the decay
of a profile by the theorem of Chae and Wolf. In the formalization this step is not part of any statement
file. The library proves that a weak profile is a profile (`Breakdown.FarField.periodicLerayProfileExists_of_weak`,
in `Breakdown/CertTorus/Main.lean`) and uses this for the torus. On $\mathbb R^3$ it derives `BreakdownOnSpace`
directly from a weak profile (`breakdown_on_space_of_weak` in `Challenge.lean`, proved in `Breakdown/Weak/`),
without passing through a smooth profile. Both routes end at the conclusions of Corollary 7.2, in the form
described above for Theorem 1.1.

## Appendix B

`CellLayerChallenge.lean` with `CellLayerTheorems.lean` states that data of seven kinds which satisfy the condition
`CellSystemCondition` (fifteen conditions on finitely many functions and numbers) give `BreakdownOnSpace` and
`BreakdownOnTorus` (`breakdownOnSpace_of_cell_system_condition`, `breakdownOnTorus_of_cell_system_condition`,
`breakdownOnSpaceAndTorus_of_cell_system_condition`). The proof shows that the data contain a certificate in the
formal sense, then obtains a weak profile and the two conclusions as above. Appendix B of the paper describes the
form of the data and the conditions. It proves Proposition B.14 (data of this form that satisfy the conditions are
a certificate in the sense of Section 7), except for two statements whose proofs are only in the formalization:
Proposition B.1, the inequality for a multiplier (`PieceMultiplierDensityWeakHolds` in
`ExtendedCoerciveChallenge.lean`, proved in `Breakdown/Cert/ExtendedCoerciveMain.lean`), and Proposition B.4, the
inequalities for one cell (`IsCellFormBound` in `Breakdown/CertCell/CellCalculusStatements.lean`). The paper
names no Lean declaration.

The formalization contains no data. The certificate whose hash the paper gives consists of data of the form of
Appendix B. That these data satisfy the conditions is verified by a rigorous computation, involving interval
arithmetic, which is not part of the formalization.

## Where the proofs differ

The formalization follows an earlier version of the argument, and the present paper differs from it in many
places. Two examples. In Section 7 the paper proves Proposition 7.10 with the Schauder–Tychonoff fixed point theorem, applied
to the parameter and the quadratic term together in a weak topology. The formalization uses a contraction for
each value of the parameter and a fixed point theorem for a box in $\mathbb R^k$
(`Breakdown/Cert/FixedPointBox.lean`). Throughout, the paper cites standard results that the formalization proves
from Mathlib, among them the smoothing estimates of the heat flow, the local theory of the equations, the
equivalence of the differential and the integral formulations, and the passage from blow-up to the statements
above.
