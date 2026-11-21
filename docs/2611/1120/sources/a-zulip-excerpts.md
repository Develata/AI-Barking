# Lean Zulip 讨论串摘录（A 组引用的 6 条）

来源：leanprover.zulipchat.com 网页公开频道 general，话题 “Discussion: Alpöge Proves the Riemann Hypothesis” 与 Masullo 话题；A 组北京 11-21 18:27 经网页公开版匿名接口取全文，交付时只保留被引用的消息（全文不入库，公开频道可在原站查看）。时间为北京时间。

## a-zulip-alpoge.txt

[642224678] 2026-11-20T21:58:06+08:00 Jack McCarthy
Well folks, it's finally happened

Uploaded less than an hour ago to Levent Alpöge's page, with no formal announcement yet. Alpöge claims a full, unconditional Lean proof of the (generalized) Riemann hypothesis. I'm compiling it now on two separate devservers to verify the claim.

https://alpo.ge/remark.pdf [https://alpo.ge/remark.pdf]

## a-zulip-alpoge.txt

[642277830] 2026-11-21T00:55:59+08:00 Andrew Sutherland
For the record I verified 9631b4f020199e580ba86178cb56c6e9b09f690c2369a80dc4334aa93f49c7fc using 8x192 core machines.  It took just over an hour.  The summary below is from Fable 5.2 which orchestrated the computation:

details

This is Lean’s own elaborator and kernel accepting every declaration. The independent re-checks (below) are still running.
What the audit shows

Statements are Mathlib’s. RiemannHypothesis and riemannZeta come from Mathlib.NumberTheory.LSeries.RiemannZeta; DirichletCharacter.LFunction from Mathlib.NumberTheory.LSeries.DirichletContinuation. The printed definition is ∀ s, riemannZeta s = 0 → (¬∃ n, s = -2 * (↑n + 1)) → s ≠ 1 → s.re = 1 / 2, and riemannHypothesis : RiemannHypothesis type-checks against it.
No axioms outside Lean core. The environment (186,200 modules, 2,701,132 constants, 2,084,256 theorems) contains 15 axioms in total, all declared in Lean’s Init. Neither Mathlib nor the submission declares any.
Mathlib is untampered. All 8,322 Mathlib oleans and 378 dependency oleans on the build machine are byte-identical to a clean download of the official v4.33.1 cache.
Source is the published one. Archive sha256 9631b4f0…c7fc, the same hash Deicyde/riemann-hypothesis records; their tree is byte-identical to ours apart from the files they added (Challenge.lean and the config).
Build outputs match lake. A 77-module sample is byte-identical to a real lake build. The manifest of all 175,187 project oleans has sha256 375cbee2…1d25; Main.olean is 9146dfb4…96ad.

Caveats

The build ran outside any sandbox on throwaway machines, and I called lean directly rather than through lake build (with lake-equivalent inputs, as the byte-identity sample shows).
“GRH” here means Dirichlet L-functions only, as stated in Statements.lean.
The first build attempt (wrong module root) was discarded entirely; nothing from it is in these results.

Still running

Kernel re-check of every module (leanchecker): restarted at 11:49 AM EST with larger batches after the first configuration proved far too slow. No modules reported yet, so I have no rate.
Comparator export: 7.4 GB written after 6.5 minutes, about 19 MB/s, single-threaded. I don’t know the final size, so I still can’t give a Comparator ETA. Both Comparator v4.33.0 and the file-based export are running on the 768 GB machine; the parallel kernels start when the file is complete.

I have not published anything. The evidence files are in /home/claude/grh/evidence/ on claude-box, and the local copy of all 175,187 oleans is complete.

## a-zulip-alpoge.txt

[642305635] 2026-11-21T02:39:29+08:00 Kevin Buzzard
I’m not sure if it belongs in this discussion, but the following post just made by Ravi Vakil seems relevant

This is not relevant to Lean.

With my moderator hat on (and speaking as someone who has already had to delete tens of messages in the last few hours) let me remind people that posts which are not about Lean are off-topic for this Zulip.

## a-zulip-alpoge.txt

[642355040] 2026-11-21T06:06:33+08:00 Szymon Teżewski
Ok, so here are all the accepted runs of independent checkers on the RH export (solution.ndjson, sha256 5ea74b91…244e) that I was able to make today:



Checker
Repo @ commit
Verdict
Time
Threads
Peak RAM




con-leche --compare
leanprover/con-leche @ 5dc36dc
all 5 theorems justified by the solution
3:35
64
48 GB


con-leche
leanprover/con-leche @ cd2ecea
accepted
3:24
64
45 GB


con-agua
leanprover/con-agua @ fa516c1
accepted
14:34
32
130 GB


Pestka
jasisz/pestka @ 48857ad
accepted
10:22
64
35 GB


nanoclo
sgraf812/nanoclo @ b8cefd0
accepted
33:43
32
91 GB


tenet
keithadler/tenet @ 5046488
accepted
45:50
32
186 GB



Have a good night ;)

## a-zulip-alpoge.txt

[642458394] 2026-11-21T18:22:59+08:00 Timothy Chow
Bogdan Grechuk said [#narrow/channel/113488-general/topic/Discussion.3A.20Alp.C3.B6ge.20Proves.20the.20Riemann.20Hypothesis/near/642444328]:

I get an e-mail from a colleague with link https://philarchive.org/rec/NIEIOT-2 [https://philarchive.org/rec/NIEIOT-2] claiming that the proof is wrong. Is it a serious claim?

No, it is not a serious claim. Jenny Lorraine Nielsen is a crank, as you can quickly ascertain for yourself if you do a little investigation. She has made numerous spurious claims of having solved major mathematical problems herself, while attacking every AI-generated proof in sight with no substantive backing.

## a-zulip-masullo.txt

[642326773] 2026-11-21T04:05:57+08:00 Kevin Buzzard
I have objections to using this particular forum as a place to deconstruct someone's proof in public. This is not about Lean. Furthermore the author of the purported proof did not ask for it to be disassembled in this public way. Mathoverflow does not allow this kind of content and I don't think we should either. I suggest you take it to Reddit.
