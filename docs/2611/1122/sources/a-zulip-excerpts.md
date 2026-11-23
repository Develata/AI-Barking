# Lean Zulip 讨论串摘录（A 组引用）

来源：leanprover.zulipchat.com 网页公开频道 general，话题 “Discussion: Alpöge Proves the Riemann Hypothesis”（匿名公开接口，无登录无凭据）；A 组北京 11-24 00:2x 取全文，交付只留被引用的消息（全文不入库）。时间为北京时间（接口 UTC 时间戳换算，本机 UTC+8）。Zulip 不给消息得分。

## [642234267] 2026-11-20T22:30:29+08:00 Floris van Doorn

It uses the Lean toolchain where all the kernel bugs that were mention in Leo's blog post were fixed. I don't recall a kernel bug after that (but there might be one). But as Riccardo says: we should be able to port the proof to a newer version of Lean and verify it there.

## [642306912] 2026-11-21T02:44:35+08:00 Kevin Buzzard

So several people are announcing that the code compiles; we still await a successful comparator run, which will check that (a) the code does not do anything nefarious like redefine the meaning of any of the symbols in the statement and (b) (assuming it's run in paranoid mode) the code compiles with several independent kernels (so the code is not just exploiting an as yet unknown bug in Lean's official kernel -- it would have to exploit bugs in several independent kernels, some of which have been formally verified).

## [642453321] 2026-11-21T17:38:21+08:00 Andrew Sutherland

Andrew Sutherland said : Harald Helfgott At this point I don't see any realistic scenario in which this proof does not imply the statement of RH in mathlib. But you should perhaps satisfy yourself that you agree with the formalization of the statement. There are a number of formalized equivalences of RH, e.g. https://github.com/kimihiro64/Robin1984 that might more easily satisfy you, and as long as we are in a belts-and-braces mode, I will check that we can also now verify forall n > 5040, sigma(n) < exp(gamma) * n * log(log n) (which seems pretty hard to screw up!), as Andy Booker suggested to me earlier today. I have now verified this equivalence using lake comparator and con-leche compare , see https://math.mit.edu/~drew/grh/ for details. I think this effectively rules out any potential holes in the definitions used in the mathlib formulation of RH. The comparator and paranoid runs on the full proof of RH are still in progress. They are unlikely to complete before tonight, more likely tomorrow, but I will update https://math.mit.edu/~drew/grh/ and post here when they finish. For those feeling impatient and looking for something to do :), it would be useful to have a parallel version of comparator that we trust as much as the single-threaded version...

## [642453558] 2026-11-21T17:40:29+08:00 Jack McCarthy

Kevin Buzzard said : my understanding of the comments above is that the answer is "yes" so we can declare that all the checks mentioned in the blog post have now been done and thus the lean community have checked the proof No I don't think this was correct. Nobody had yet completed lake comparator until as of 1 minute ago

## [642453964] 2026-11-21T17:44:44+08:00 Andrew Sutherland

I agree with @Jack McCarthy . I personally have no doubts at this point, but in all my public remarks I have tried to make it clear that the verification is still in progress and that for a result of this importance we should take the time to make sure it meets the gold standard of the Lean community.

## [642456684] 2026-11-21T18:09:10+08:00 Joachim Breitner

Kevin Buzzard schrieb : Aah -- perhaps Joachim is saying that we haven't verified it to the best of our abilities but that there are tests which are running which I don't mention in the blog post? I'm saying we have checked it to the best of our abilities: statement comparison, done two ways, one with verified code proof checks, done with many checkers, including two verified ones There are more variants we could run, but they don't add much value at this point. What we haven't done yet is “Run this one command that bundles a few checks without thinking about it” (it takes long because it's sequential and it did bundle slightly out of date tools). That one command doesn't add anything to anyone who knows what it's doing, as we did all the steps separately. Knowing and explaining that are extra steps and it would be great if “run comparator paranoid” just worked without further thinking. So we have achieved what comparator in paranoid mode can achieve. Whether you'd rather explain that or what for that too to finish is up to you :-)

## [642463018] 2026-11-21T19:07:52+08:00 Kim Morrison

(We've now heard from Levent and he's working on getting a licence out.)

## [642485818] 2026-11-21T22:28:25+08:00 Johan Commelin

Press release on the verification process: https://leanprover-community.github.io/blog/posts/riemann-hypothesis/ Thanks everyone for the rally!

## [642590778] 2026-11-22T10:02:20+08:00 Mirek Olšák

@Artie Khovanov To my knowledge, it has not been machine code. con-leche 's correctness depends on Lean compiler validity, and con-ron depends on Rust compiler validity & correct (automatic) Rust-Lean translation. Also, it is good to take into account that all the SW-verified kernels were built with significant AI assistance. I am unsure if anyone went full to machine code. On the other hand, verifying against machine code requires anyway trusting the CPU spec, which is often messy. And indeed, inconsistency of ZFC+universes is always an option, perhaps even wilder than the proof of Riemann hypotesis. But broadly I agree that it is hard to imagine any other plausible scenario than that the Riemann Hypothesis simply has been solved. edit: I missed con-agua , my bad

## [642647909] 2026-11-22T19:00:08+08:00 Shirin Shafaie

I am relatively new to Lean and mathlib, so I'm a bit nervous to post this comment but am I alone in finding it strange for Lean FRO and the Mathlib Initiative to jump to endorse a statement on this largely LLM-generated proof before having a credible understanding of what the proof actually means in natural human language? I understand that the formal proof has been verified so I'm not questioning the significance of that but when did establishing "that" something is true become more important than understanding "why"? Ever since joining this wonderful community (via a graduate course on Lean formalisation I'm taking at the Fields Institute and via my first PR on a small TODO proof in mathlib), I've been under the impression that people here cared deeply about the human understanding of mathematics, not just the mechanical verification of proofs. Am I alone in feeling a bit disappointed and discouraged by the way such an extraordinary mathematical claim is being celebrated so readily when all we got from Claude/Alpoge was 17 million lines of LLM-generated Lean code and 250k pages of AI-generated draft with no comprehensible mathematical explanation of WHY the RH is true? It just feels like the frontier AI companies are in a race to dump increasingly enormous formalised proofs for the sake of prestige and funding and less for the sake of human mathematical understanding and that's what I find sad and worrying. Lastly, I was thinking of starting a new topic to discuss this question but again as a new member I wasn't sure if this should be under this thread or deserves to have a new topic and I couldnt find another discussion thread on this particular question.

## [642648569] 2026-11-22T19:06:33+08:00 David Loeffler

Shirin Shafaie said : I am relatively new to Lean and mathlib, so I'm a bit nervous to post this comment but am I alone in finding it strange for Lean FRO and the Mathlib Initiative to jump to endorse a statement on this largely LLM-generated proof before having a credible understanding of what the proof actually means in natural human language? This is just my opinion – I don't speak for MI or the Lean FRO here – but why would this be the Lean community's role? This community consists of experts in formal verification, and there is (by now) no credible doubt that this proof has been formally verified. That is what the MI / FRO announcement says: it makes a declaration about formalization, not about mathematics at large. Speaking as a number theorist as well as a Lean person, I really really hope that soon we can reach a collective understanding of why the proof works, and I am currently trying all I can to work that out, as are many, many other mathematicians (some of them much smarter than I). But if and when that succeeds, it will not be the Lean FRO or Mathlib Initaitive's role to make a statement about it.

## [642648880] 2026-11-22T19:09:58+08:00 Kim Morrison

Exactly: there are a lot of sleepless number theorists trying to understand this all right now, and here in the Lean community we are trying to do the bit we can do (very well!): provide the single bit of information "the proof is correct".

## [642271468] 2026-11-21T00:36:56+08:00 Malvin Gattinger

Does it depend on elan or lake from a month ago or older? Because the proof is using an old lean (and thus lake?) version (v4.33.1 from 21 August).

## [642386485] 2026-11-21T09:07:55+08:00 Kim Morrison

To date, the experience of the experts is that it's not actually that effective to try to refactor an "AI dump" Lean proof continuously into something nice, and often it's more practical to just take a good look at it and rewrite. But today is sufficiently exciting that we probably should try this anyway! It's clear that the Lean proof is stylistically terrible right now. It's running on an old version of Lean and Mathlib. It's probably (although I haven't seen this quantified) redoing things already done in Mathlib (and likely Tau Ceti too). It probably has dead code that isn't used by the main result. It probably has internal duplication. And it almost certainly has extremely inefficient proofs, and places where intermediate lemmas can be useful extracted. What logistical questions do we need to sort out to have a community adoption of the repository? (e.g. Jack McCarthy's fork, perhaps even moved to leanprover-community/) It seems the main question is actually providing resources to get builds done, and oleans published. This is somewhat expensive, but not inordinately so. Shall we just do it? I happy to assist setting up CI, providing a mechanism for people to register external

## [642456189] 2026-11-21T18:04:23+08:00 Kim Morrison

I'm not sure if it unnecessarily duplicating effort, but I've now turned on CI at https://github.com/leanprover-community/riemann-hypothesis , and there are instructions for registering your own machine as a CI runner at #AI authored projects > Running CI for the riemann hypothesis .

## [642456477] 2026-11-21T18:07:09+08:00 Michael Stoll

Going back to the observation that Levent(+Claude)'s Lean code looks so low-quality: this surprises me a bit. I get Claude to write quite decent Lean code these days, and I assume the model that was used is quite a bit more advanced than what I have access to. I suppose that "high quality Lean code" just was not a priority here.

## [642458394] 2026-11-21T18:22:59+08:00 Timothy Chow（仅摘与本稿性质相关的一句；原帖还对稿件作者有评语，不存）

回应 Bogdan Grechuk 关于 philarchive.org/rec/NIEIOT-2 “claiming that the proof is wrong. Is it a serious claim?”：“No, it is not a serious claim.”
