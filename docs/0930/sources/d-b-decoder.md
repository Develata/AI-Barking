URL: https://the-decoder.com/anthropic-says-zhipus-open-weight-glm-5-3-nearly-matches-claude-mythos-preview-at-building-exploits/
Fetched BJT: 2026-09-30T23:58:56.136363+08:00
HTTP: 200
Title: Anthropic says Zhipu's open-weight GLM-5.3 nearly matches Claude Mythos Preview at building exploits

Anthropic says Zhipu's open-weight GLM-5.3 nearly matches Claude Mythos Preview at building exploits
Ad
Skip to content
Log In
Subscribe
DE
Switch to German
Primary Menu
Log In
Subscribe
DE
Switch to German
Primary Menu
Sign In
Register
Subscribe Now
The Decoder
Opens discord in a new tab
Opens LinkedIn in a new tab
Plus
AI and society
Copy the url to clipboard
Share this article
Go to comment section
Anthropic says Zhipu's open-weight GLM-5.3 nearly matches Claude Mythos Preview at building exploits
Maximilian Schreiner
View the LinkedIn Profile of Maximilian Schreiner
Sep 30, 2026
Nano Banana Pro prompted by THE DECODER
Zhipu's open-weight model GLM-5.3 comes close to Claude Mythos Preview at exploit development, according to an Anthropic analysis. Anthropic says the model's safeguards can be bypassed with simple methods. The company also has its own reasons to sound the alarm.
Five months after Anthropic unveiled Claude Mythos Preview, its Frontier Red Team says the model's signature capability has reached the competition. In a new
analysis
, the team looks at GLM-5.3 from Zhipu AI, which operates as Z.ai outside China. According to Anthropic, GLM-5.3 can build complete cyber exploits on its own, just like Mythos Preview. Unlike every other model with comparable skills, though, it shipped without effective safeguards.
Anthropic deliberately held Mythos Preview back, giving access only to select defenders through
Project Glasswing
so they could get a head start. The company says those defenders have since found more than 10,000 vulnerabilities in critical software. OpenAI is taking a similar approach with
Daybreak
. GLM-5.3, on the other hand, is available for anyone to download.
Frontier-level exploits now cost about as much as lunch
ExploitBench
measures how well models exploit known bugs in Chrome's V8 engine. On that benchmark, GLM-5.3 built a working exploit in 50 of 410 attempts, while Mythos Preview managed 56. Anthropic also runs an internal binary exploitation benchmark based on open-source projects from Google's OSS-Fuzz. There, GLM-5.3 took full control of the target program in 4 percent of tasks, compared to 6 percent for Mythos Preview. Older models like GLM-5.2 and Claude Opus 4.6 failed both tests, and Kimi K3 and DeepSeek V4.1-Flash barely got off zero.
Anthropic's numbers put GLM-5.3 close to Mythos Preview at exploit development, with safeguards that are easy to bypass. | Image: Anthropic
Anthropic also paired GLM-5.3 with a human expert. Within a single day and with little human attention, the model found several previously unknown vulnerabilities in the JavaScript engine of a widely used browser. It then chained them into a web page that can read any file on a visitor's computer, and in the test it pulled a private SSH key. Anthropic says it reported the vulnerabilities to the browser's developers. Other findings in drivers and device firmware are still under review.
Anthropic used the smaller GLM-5.3-Flash to test how quickly a freshly disclosed vulnerability can be turned into a working attack. The model combined a recently disclosed Chrome bug with another known vulnerability and, with little guidance, built a reliable attack out of them. The attack even got around an extra security feature built into the processor. The whole job took 20 minutes of human attention and eight hours of model time, which would have cost $20.40 at Zhipu's API prices.
The US agency CAISI reached similar conclusions in its
own assessment
. It calls GLM-5.3 the most cyber-capable open-weight model to date and puts it about four months behind the best US models. That comparison comes with caveats. CAISI tested the US models with their cyber safeguards turned off, and the top tier includes models that only vetted users can access.
Open weights make safeguards easy to remove
In an Anthropic simulation, GLM-5.3 refused openly malicious attack commands. When the same request was dressed up as a red-team exercise, the model tried to connect to the target system in 64 percent of runs. With prefilled reasoning steps, that number rose to 92 percent. After
abliteration
, a technique that strips refusal behavior out of open weights, it reached 100 percent. The simulation doesn't execute any code, so it can't show whether an attack would actually have succeeded. Protected Claude models stayed at zero.
Anthropic's team says this was its first time using abliteration. The process took about 2,200 GPU hours at a cost of roughly $4,400, and Anthropic estimates an experienced team could do it for around $1,200. The refusal rate for harmful requests fell from over 90 percent to between 2 and 12 percent, while scores on science and cyber tests barely moved. According to Anthropic, several developers had already released unlocked versions within days of the model's launch.
Anthropic concludes that state and non-state actors will likely use models like GLM-5.3 to cause real harm. It points to
its own reports
and those from other US labs documenting attackers who already use AI. Governments should test capable models, the company argues, and defenders need tools at least as good as the ones their adversaries have.
Anthropic's warning also serves its business
The analysis isn't entirely selfless. Anthropic doesn't release its model weights, and the report presents exactly that as a key security advantage. A cheap Chinese open-weight model that's close to the frontier is also a direct competitor.
The report's conclusions fit Anthropic's business, too. The company wants to bring Claude's cyber capabilities to more defenders, and vetted users already have access to Claude Mythos 5.1. Its call for government testing of GLM-5.3's successors also invites suspicion of regulatory capture, where rules end up mainly protecting established players.
Still, this isn't just self-interest. CAISI's independent assessment backs up the capability numbers, and unlocked versions of the model are already out there.
The UK's AI Security Institute recently
found
that open models have narrowed their lag in cyber capabilities from six to ten months down to four to seven months. According to the institute, open models are also much cheaper to run, and their safeguards are largely ineffective. AISI warned of a persistent and irreversible misuse risk but also pointed to real benefits like private hosting, customization, and lower costs.
AISI saw that lag as a window for defenders to prepare. At the time, it was still unclear whether open models would also catch up to the leap Mythos Preview represented. The measurements from Anthropic and CAISI suggest GLM-5.3 is a first answer to that question.
AI News Without the Hype – Curated by Humans
Subscribe to THE DECODER for ad-free reading, a weekly AI newsletter, our exclusive "AI Radar" frontier report six times a year, full archive access, and access to our comment section.
Subscribe now
Read on for the full picture.
Subscribe for hype-free coverage.
Full access to every article on THE DECODER
No ads
Join the comments and community discussions
A weekly AI news recap via mail
6x/year: "AI Radar" — deep dives on the AI topics that matter most
Daily AI news, always up to date
Our full ten-year archive
Covered by a team with 10+ years in AI
Subscribe to The Decoder
Ad
Top
Stories
Top AI experts badly underestimated how fast the field is moving, study finds
AI agents do more of the work in model development, but humans still make the decisions
AI access makes people almost entirely unwilling to say "I don't know," study finds
Nvidia's SoL-Pi system cuts coding agent token usage nearly in half by optimizing the harness
Former Ukrainian Defense Minister Fedorov pitches a private-sector robot army
Don't Miss
What Matters
Stay in the loop on AI. Clear, useful, no fluff.
Your email address
Subscribe to our Newsletter
Most
Popular
Open-source tool pxpipe hides text in PNGs to cut Claude Code and Fable 5 token costs up to 70%
Anthropic says it cut 80 percent of Claude Code's system prompt because Fable 5 models "want a smaller system prompt"
Claude Code and Fable 5 ported the 2003 PC game Command & Conquer to native iOS in "a few hours"
Anthropic developer shares prompting tips for Fable 5 that focus on finding your own blind spots first
GPT and Claude failed Bridgewater's finance tests because the right answers were never public
Baidu's "Unlimited OCR" processes dozens of document pages in one pass by treating memory like human forgetting
AI Community
& Insights
The Decoder
Follow The Decoder for AI news, background stories and expert analyses.
Opens LinkedIn in a new tab
Opens Discord in a new tab
Ad
BETA-TEST
×
Start new search
×
|
Send
wpDiscuz
Insert
Newsletter - THE DECODER
Stay in the loop on AI. Clear, useful, no fluff.
Your email address
Subscribe to our Newsletter
Opens discord in a new tab
Opens discord in a new tab
Information
About
Advertise
Publish with us
Login
Topics
AI and society
AI in practice
AI research
Frontier Radar
Short News
Legal
Privacy Policy
Privacy Manager
Imprint
Terms and Conditions
Cancellation & Refund
THE DECODER by
DEEP CONTENT by heise
| All rights reserved 2026
BETA-TEST
×
Start new search
×
|
Send
wpDiscuz
Insert