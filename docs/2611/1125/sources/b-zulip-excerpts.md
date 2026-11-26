# B 组 Zulip 摘录：Lean Zulip #Riemann Hypothesis > New proof in 300k lines

来源：Lean Zulip 公开（web-public）频道，匿名接口分页取全文（`b-zulip-fetch.py`，频道 658556，话题 “New proof in 300k lines”，共 403 条，最后一条北京 2026-11-26 21:42 前后；取于北京 11-26 21:50 前后）。完整 JSON 留在 scratchpad，不入库；只摘下列与本组有关的条目，原文逐字，时刻为北京时间。未存邮箱与头像字段。每条前的“被回复者”只写其被引用的原话，不收其个人信息。

链接：https://leanprover.zulipchat.com/#narrow/channel/658556-Riemann-Hypothesis/topic/New.20proof.20in.20300k.20lines

## 消息 643702366｜Luke Sernau｜北京 2026-11-26 05:26:26（UTC 11-25 21:26:26）

Hi all, over the past few days I've been sprinting with a few colleagues at Anthropic on a much shorter version of the proof. We've been able to reduce it by about two orders of magnitude, from about 15 million lines to ~76,000. Also removed a lot of complexity (removes the dependence on the weak form of Vandiver's conjecture, uses a simpler integrality argument, the sieve step is just Poisson summation, etc.). Just posted it here: https://github.com/lsernau/grh-lean

## 消息 643709436｜Kim Morrison｜北京 2026-11-26 06:06:51（UTC 11-25 22:06:51）

I'm getting cache ready for the new shorter proof now. I propose that we just move main over to a different branch, say alpoge, and just switch to this shorter proof on main.

## 消息 643710158｜Kim Morrison｜北京 2026-11-26 06:11:08（UTC 11-25 22:11:08）

I have a build of the 300k version, uploading oleans now. But it's very doable for anone.

## 消息 643710501｜Luke Sernau｜北京 2026-11-26 06:13:06（UTC 11-25 22:13:06）

Riccardo Brasca said:

I am running lake comparator --paranoid.

Heads up lean4lean OOMed for me at 125GB on grh/, let me know if it gives you trouble.
Arthur Paulino said:

Luke Sernau can you share your method(s)?

Yes, it's based on the same proof. I'm not able to say very much about methods, unfortunately, but I had some luck with unrolling definitions to get a long, flat proof, looking for simplifications there, and then rolling them back up again. That way I could simplify across boundaries. Some stuff just needed entirely new arguments though, like getting rid of Vandiver.

## 消息 643717520｜Luke Sernau｜北京 2026-11-26 06:57:21（UTC 11-25 22:57:21）

Jake Levinson said:

Luke Sernau Can you say a bit more about what else was removed from the proof? It initially sounded like it used a lot of big results (resolution of singularities?) that weren't in mathlib yet. 300,000 lines sounds pretty short for that, so does the proof somehow bypass all that stuff?

Yeah, a lot of the simplification was finding simpler proofs for sub-results. So a fair number of the arguments are new.
For example: resolution of singularities is gone entirely; the product formula for a Gram determinant replaces the slope inequality and Hecke L-values replace the covolumes on Shimura varieties. Vandiver, patching, Hida theory, Gsp4, local Langlands, there were some big cuts.

## 消息 643719237｜Kim Morrison｜北京 2026-11-26 07:08:23（UTC 11-25 23:08:23）

I've basically turned off the merge queue for the alpoge branch, to reuse those machines for v4.37.0-rc1. To be honest I'm not sure what the point of continuing with the old branch is.

## 消息 643719896｜Luke Sernau｜北京 2026-11-26 07:12:57（UTC 11-25 23:12:57）

Snir Broshi said:

Will {76k + formatter} still be of worse quality than 300k?
(I don't mean line count)

I would expect so. Basically we code golfed the heck out of it to get the 76k version and then rewrote for clarity to get the 300k one. So the 300k version is sort of what you're describing plus some additional readability edits.

## 消息 643724260｜Luke Sernau｜北京 2026-11-26 07:40:59（UTC 11-25 23:40:59）

@Anton Mellit  Yeah, we made some cursory effort to also include some points of interest, and finish the thought on some lemmas and things rather than using just the fragment the proof needed. The idea was to make it easier for folks who are trying to build a human-language version to have what you need. We certainly were not exhaustive, you can drop them if you don't want them!
