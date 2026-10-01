URL: https://madrobot.blog/2026/09/29/anthropic-glm-5-3-zai-cyber-exploits-safeguards-open-weight/
Fetched BJT: 2026-09-30T23:58:56.137144+08:00
HTTP: 200
Title: Anthropic: China’s GLM-5.3 Can Build Cyber Exploits | MadRobot

Anthropic: China’s GLM-5.3 Can Build Cyber Exploits | MadRobot
Skip to main content
Menu
MAD
Robot
The latest AI news, every day
Follow
Get MadRobot every day
RSS feed
In Feedly, Inoreader or any reader
Search for:
Anthropic
OpenAI
Google
Meta
Microsoft
Apple
SpaceXAI
More Labs
Chips
Robotics
Policy & Safety
Business
Guides
More Labs
Policy & Safety
Anthropic
Anthropic says a Chinese AI model anyone can download can now build working hacks on its own
Vikram Singh
Sep 29 2026 - 19:42 UTC
A redacted screenshot of an exploit page GLM-5.3 built in Anthropic’s sandboxed test, reading an SSH private key from the test machine. Image: Anthropic
The kind of AI hacking power Anthropic has kept locked away is now free for anyone to download, the company warned on Tuesday. In
a report from its Frontier Red Team
, Anthropic says GLM-5.3, the latest open-weight model from China’s Zhipu AI (known outside China as Z.ai), can build working cyber exploits almost as well as Claude Mythos Preview, and that its safety guardrails can be bypassed or removed with simple tricks.
As capable as Mythos, without the lock
Anthropic released Mythos Preview five months ago only to vetted defenders through Project Glasswing, because it was the first model that could build end-to-end exploits on its own. Trusted defenders have since used it to find more than 10,000 vulnerabilities in critical software, and security firms such as
Palo Alto Networks
now point it at their customers’ systems.
In Anthropic’s tests, GLM-5.3 comes close. On ExploitBench, which asks models to exploit known bugs in Chrome’s V8 engine, GLM-5.3 built a working exploit in 50 of 410 attempts, against 56 for Mythos Preview. On Anthropic’s own binary exploitation test it managed a full takeover in 4% of trials, against 6% for Mythos. Earlier models, including GLM-5.2 and Claude Opus 4.6, managed none.
In one session, a researcher gave GLM-5.3 a sandboxed Linux build of a popular web browser. Within a day, and with little human attention, the model found several previously unknown flaws and chained them into a webpage that reads files from a visitor’s computer (the screenshot above shows it taking an SSH private key in Anthropic’s test). Anthropic says it has reported the flaws to the browser’s maintainer. In another test, the smaller GLM-5.3-Flash turned two public Chrome bugs into a working exploit chain in eight hours, a job that would have cost $20.40 at Zhipu’s API prices.
That lines up with the US government’s own testing. NIST’s Center for AI Standards and Innovation
called GLM-5.3
“the most cyber-capable open-weight model released to date” on September 17, and put it about four months behind the best US models.
“My job is to cause deaths quietly”
The bigger problem, Anthropic says, is how easily GLM-5.3’s safeguards come off. Asked outright to attack critical systems in a simulated test, it refused every time. But telling it that it was a red-team agent on an exercise got it to go ahead 64% of the time, pre-filling its reasoning pushed that to 92%, and an “abliterated” copy, with its refusals edited out of the model’s weights, complied every time.
Because the weights are public, anyone can do that edit. Anthropic says its team did it for about $4,400 of computing, and several developers posted abliterated versions within days of the release. The edit cut GLM-5.3’s refusal rate on harmful-request benchmarks from above 90% to as little as 2%, with almost no loss of capability.
Reasoning from an abliterated copy of GLM-5.3 in Anthropic’s simulated test. Image: Anthropic
The same tricks didn’t work on Claude in Anthropic’s tests: its safeguards blocked the deceptive prompts, and because Claude’s weights aren’t public, its reasoning can’t be pre-filled through the API or its refusals edited out.
A competitor’s warning
Anthropic is hardly a neutral party. It sells access to Mythos through its trusted programmes, and it has long pushed for tighter controls on AI and chips going to China. Its report ends by calling for more defenders to get frontier models and for governments to safety-test capable models, including GLM-5.3’s successors. Z.ai hasn’t commented on the report.
Anthropic also concedes that the same abilities help defenders, and many developers use Z.ai’s cheaper GLM models, which are
available free through NVIDIA’s API
, for everyday coding work.
Why it matters
Until now, the most dangerous AI hacking abilities sat behind company gates, where access could be vetted and misuse spotted. If Anthropic’s findings hold, that gate is gone: a model almost as capable as Mythos can be downloaded and stripped of its refusals for a few thousand dollars. Anthropic expects state and criminal hackers to use it, which leaves defenders racing to patch first.
Sources:
Anthropic Frontier Red Team, “GLM-5.3 and the spread of advanced cyber capabilities” (September 29, 2026)
,
NIST CAISI assessment of GLM-5.3 (September 17, 2026)
.
Latest More Labs news
DeepSeek just gave Huawei’s chips the software they were missing to take on Nvidia
Sanjit Patel
Sep 30 2026
China now wants its top AI engineers’ families to ask permission before they leave the country
Sanjit Patel
Sep 28 2026
Alibaba unveils what it calls China’s most powerful AI chip – and plans a 10 trillion-parameter model to catch the U.S.
Ross Nicholson
Sep 23 2026
More More Labs news
Follow MadRobot for daily AI news:
RSS feed
Poll
The biggest AI companies have signed a voluntary pact with no penalties. Should they be left to police themselves?
Yes, they understand the technology best
For now, with laws to follow later
No, they need binding laws with penalties
Not sure
View results
Submit
|
Read the story
Trending
OpenAI wants an AI agent working for you around the clock, and everything else it announced at DevDay
Sep 29 2026
Barack Obama is telling Democrats who want to be president that their AI plans aren’t good enough
Sep 30 2026
Ted Cruz just blocked mandatory AI safety rules in the Senate, for now
Sep 30 2026
Updated
The Bank of England warns AI stocks could fall much harder than they did in July
Sep 30 2026
Google DeepMind can now hide a watermark inside AI-designed proteins, and it survives in the real molecule
Sep 30 2026
MAD
Robot
The latest AI news, every day.
Topics
Anthropic
OpenAI
Google
Meta
Chips
Robotics
Policy & Safety
Business
Guides
Analysis
MadRobot
About
Submit a tip
RSS
Photo credits
Article photography is sourced from Wikimedia Commons under Creative Commons and public domain licenses. Credits appear on each article.
© 2026 MadRobot. All rights reserved.
About
·
Contact
·
Editorial policy
·
Privacy
·
Terms