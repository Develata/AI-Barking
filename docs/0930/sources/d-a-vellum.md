URL: https://website.vellum.ai/blog/gpt-6-1-sol-benchmarks-explained
Fetched BJT: 2026-09-30T23:55:39.357512+08:00
HTTP: 200
Title: GPT-6.1 Sol Benchmarks Explained | Vellum Blog

GPT-6.1 Sol Benchmarks Explained | Vellum Blog
Pricing
Community
Use Cases
Blog
Careers
Log in
Get Started
GPT-6.1 Sol Benchmarks Explained
Every GPT-6.1 Sol benchmark explained, and compared head to head with GPT-6 Sol, GPT-6 Astra, and Claude Opus 5.5.
Written by
Nicolas Zeeb
Contents
Quick Comparison: Key Specs and Pricing at a Glance
1. Real-World Software Engineering: DeepSWE v1.1
2. Long-Horizon Computer Use: OSWorld 2.0
3. Professional Document Analysis: GDP.pdf
4. Multi-Step Business Workflows: AutomationBench 1.0.6
5. Scientific Research and Terminal Simulation: Terminal-Bench Science 0.1
6. Factual Reliability and Hallucination Reduction
7. Safety, Tool Transparency, and Alignment
8. Latency and Token Speed: The Upcoming Ultrafast Tier
Key Takeaways
Route Dynamically Across Frontier Models in Vellum
Just seven days after releasing
GPT-6 Sol and Luna
alongside a 50% price cut, OpenAI has dropped
GPT-6.1 Sol
.
Announced on September 29, 2026 at
DevDay 2026
, the pitch is straightforward:
near-Astra intelligence for a fifth of the price
. Standard API pricing remains fixed at
$2.00 per million input tokens
and
$10.00 per million output tokens
, while cached input has been slashed by another 50% to
$0.10 per million tokens
(a 95% discount off standard input).
Where GPT-6 Sol established a viable mid-tier workhorse, GPT-6.1 Sol attacks the justification for paying flagship rates at all. On DeepSWE v1.1, it matches GPT-6 Astra's top-end accuracy while undercutting task execution costs by roughly 80%. On OSWorld 2.0 offline evaluations, it lands within 2.1 percentage points of Astra while running at one-seventh the cost per completed task. Across professional document processing on GDP.pdf and workflow orchestration on AutomationBench, it outpaces Claude Opus 5.5 at a fraction of Anthropic's task pricing.
Below is every benchmark from OpenAI's announcement, translated into practical performance terms and compared directly against GPT-6 Astra, GPT-6 Sol, Claude Sonnet 5.5, and Claude Opus 5.5.
Quick Comparison: Key Specs and Pricing at a Glance
GPT-6.1 Sol sits in OpenAI's mid-tier pricing band but shifts the frontier efficiency frontier. It is available immediately in the OpenAI API under the model identifier
gpt-6.1-sol
, as well as inside ChatGPT Work and Codex for Plus, Pro, Business, Enterprise, and Edu tiers.
<table style="width:100%;border-collapse:collapse;margin:24px 0;font-family:-apple-system,BlinkMacSystemFont,sans-serif;font-size:13px;border:1px solid #e5e3dc;border-radius:8px;overflow:hidden;">
<thead>
<tr style="background:#fafaf7;border-bottom:1px solid #e5e3dc;text-align:left;">
<th style="padding:12px 16px;color:#1a1a1a;font-weight:600;">Model</th>
<th style="padding:12px 16px;color:#1a1a1a;font-weight:600;">Input / 1M</th>
<th style="padding:12px 16px;color:#1a1a1a;font-weight:600;">Cached Input / 1M</th>
<th style="padding:12px 16px;color:#1a1a1a;font-weight:600;">Output / 1M</th>
<th style="padding:12px 16px;color:#1a1a1a;font-weight:600;">Context Window</th>
<th style="padding:12px 16px;color:#1a1a1a;font-weight:600;">Availability</th>
</tr>
</thead>
<tbody>
<tr style="border-bottom:1px solid #eee;background:#fff;">
<td style="padding:12px 16px;font-weight:600;color:#1a1a1a;">GPT-6.1 Sol</td>
<td style="padding:12px 16px;color:#333;">$2.00</td>
<td style="padding:12px 16px;color:#10a37f;font-weight:600;">$0.10 (-50%)</td>
<td style="padding:12px 16px;color:#333;">$10.00</td>
<td style="padding:12px 16px;color:#333;">1,050,000</td>
<td style="padding:12px 16px;color:#333;">API, ChatGPT Work, Codex</td>
</tr>
<tr style="border-bottom:1px solid #eee;background:#fafaf7;">
<td style="padding:12px 16px;color:#555;">GPT-6 Sol</td>
<td style="padding:12px 16px;color:#555;">$2.00</td>
<td style="padding:12px 16px;color:#555;">$0.20</td>
<td style="padding:12px 16px;color:#555;">$10.00</td>
<td style="padding:12px 16px;color:#555;">1,050,000</td>
<td style="padding:12px 16px;color:#555;">API, ChatGPT Work, Codex</td>
</tr>
<tr style="border-bottom:1px solid #eee;background:#fff;">
<td style="padding:12px 16px;color:#555;">GPT-6 Astra</td>
<td style="padding:12px 16px;color:#555;">$10.00</td>
<td style="padding:12px 16px;color:#555;">$1.00</td>
<td style="padding:12px 16px;color:#555;">$50.00</td>
<td style="padding:12px 16px;color:#555;">1,050,000</td>
<td style="padding:12px 16px;color:#555;">API, Pro, Enterprise</td>
</tr>
<tr style="border-bottom:1px solid #eee;background:#fafaf7;">
<td style="padding:12px 16px;color:#555;">Claude Sonnet 5.5</td>
<td style="padding:12px 16px;color:#555;">$2.00</td>
<td style="padding:12px 16px;color:#555;">$0.20</td>
<td style="padding:12px 16px;color:#555;">$10.00</td>
<td style="padding:12px 16px;color:#555;">1,000,000</td>
<td style="padding:12px 16px;color:#555;">API, Claude Work, Claude Code</td>
</tr>
<tr style="background:#fff;">
<td style="padding:12px 16px;color:#555;">Claude Opus 5.5</td>
<td style="padding:12px 16px;color:#555;">$5.00</td>
<td style="padding:12px 16px;color:#555;">$0.50</td>
<td style="padding:12px 16px;color:#555;">$25.00</td>
<td style="padding:12px 16px;color:#555;">1,000,000</td>
<td style="padding:12px 16px;color:#555;">API, Claude Pro, Claude Code</td>
</tr>
</tbody>
</table>
A critical operational change is prompt caching: at $0.10 per million tokens, caching long developer codebases, API documentation, or complex multi-document context is now essentially free. In multi-turn agent loops where an agent reads a 200k-token repository on every step, caching cuts per-step input overhead from $0.40 to $0.02.
1. Real-World Software Engineering: DeepSWE v1.1
On
DeepSWE v1.1
, which evaluates autonomous agents on resolving real, complex software engineering tasks in full repositories, GPT-6.1 Sol achieves its headline result:
matching GPT-6 Astra's peak accuracy of ~74.8% while cutting the cost per task by 80%
.
Compared to original GPT-6 Sol, which topped out at
68.8%
at max reasoning effort ($2.60/task), GPT-6.1 Sol achieves
75.2%
at higher reasoning settings, gaining 6.4 percentage points over its predecessor at a lower reasoning effort and cost.
As reported by
The New Stack
, GPT-6.1 Sol also moves ahead of Anthropic's newly launched
Claude Sonnet 5.5
(71.0% on DeepSWE) while operating in the identical $2/$10 pricing band.
<div style="font-family:-apple-system,BlinkMacSystemFont,sans-serif;background:#fafaf7;border:1px solid #e5e3dc;border-radius:8px;padding:20px;margin:24px 0;">
<div style="font-size:14px;font-weight:600;color:#1a1a1a;margin-bottom:4px;">DeepSWE v1.1</div>
<div style="font-size:12px;color:#666;margin-bottom:16px;">Pass rate, % (higher is better)</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6.1 Sol (high)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:100%;background:#1a1a1a;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;font-weight:600;color:#1a1a1a;margin-left:8px;">75.2</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Astra (high)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:99.5%;background:#888888;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">74.8</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">Claude Sonnet 5.5</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:94.4%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">71.0</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">Claude Fable 5 (xhigh)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:93.0%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">69.9</div>
</div>
<div style="display:flex;align-items:center;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Sol (max)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:91.5%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">68.8</div>
</div>
<div style="font-size:11px;color:#888;margin-top:14px;">Source: OpenAI, Introducing GPT-6.1 Sol announcement and official scatter plot; The New Stack. GPT-6.1 Sol matches Astra at roughly $1.50/task vs ~$7.70/task for Astra.</div>
</div>
What makes this chart striking is the efficiency curve. In OpenAI's published scatter plot, GPT-6.1 Sol's trajectory rises steeply between $0.50 and $1.50 per task, establishing a plateau around 72% to 75%. GPT-6 Astra covers that same 68% to 75% band but requires between $2.00 and $7.70 per task. For development teams running autonomous test-and-repair loops in CI, switching from Astra to 6.1 Sol reduces spend by four-fifths with zero drop in bug resolution rates.
2. Long-Horizon Computer Use: OSWorld 2.0
On the
OSWorld 2.0 offline benchmark
, which measures how effectively an AI agent navigates real desktop operating systems, handles web forms, switches between apps, and resolves multi-step GUI workflows, GPT-6.1 Sol delivers an unexpected jump.
At maximum reasoning effort, GPT-6.1 Sol scores
71.4%
, outperforming GPT-6 Sol (
64.4%
) by
7.0 percentage points
while reducing compute cost by more than half.
Critically, GPT-6.1 Sol comes within
2.1 percentage points of GPT-6 Astra's flagship score of 73.5%
, while costing approximately
one-seventh as much per completed task
($1.30 vs $9.30).
<div style="font-family:-apple-system,BlinkMacSystemFont,sans-serif;background:#fafaf7;border:1px solid #e5e3dc;border-radius:8px;padding:20px;margin:24px 0;">
<div style="font-size:14px;font-weight:600;color:#1a1a1a;margin-bottom:4px;">OSWorld 2.0 (Offline Set, Partial Reward)</div>
<div style="font-size:12px;color:#666;margin-bottom:16px;">Score, % (higher is better)</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Astra (max)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:100%;background:#888888;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">73.5</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6.1 Sol (max)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:97.1%;background:#1a1a1a;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;font-weight:600;color:#1a1a1a;margin-left:8px;">71.4</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Sol (max)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:87.6%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">64.4</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">Claude Opus 5 (medium)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:82.0%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">60.3</div>
</div>
<div style="display:flex;align-items:center;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Sol (xhigh)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:82.3%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">60.5</div>
</div>
<div style="font-size:11px;color:#888;margin-top:14px;">Source: OpenAI, Introducing GPT-6.1 Sol announcement; OSWorld 2.0 v2026.08.08 offline release evaluation.</div>
</div>
For autonomous desktop agents, cost per step is the primary barrier to deployment. High-effort computer use requires processing multiple full-screen screenshot frames, OCR overlays, and DOM trees. By cutting token generation costs and reasoning loop overhead, GPT-6.1 Sol makes high-frequency GUI automation viable in standard production pipelines.
3. Professional Document Analysis: GDP.pdf
On
GDP.pdf
, a benchmark developed by Surge AI evaluating how accurately models answer questions from complex professional documents (financial statements, legal filings, medical records, multi-column tables, and dense charts), GPT-6.1 Sol outperforms Anthropic's flagship.
GPT-6.1 Sol achieves
32.0%
at higher reasoning settings, beating
Claude Opus 5.5
with fallbacks (
28.8%
) while cutting task cost by more than
50%
.
Simultaneously, GPT-6.1 Sol approaches GPT-6 Astra's state-of-the-art score of
32.2%
at roughly one-fifth of the cost per task ($0.38 vs $1.95).
<div style="font-family:-apple-system,BlinkMacSystemFont,sans-serif;background:#fafaf7;border:1px solid #e5e3dc;border-radius:8px;padding:20px;margin:24px 0;">
<div style="font-size:14px;font-weight:600;color:#1a1a1a;margin-bottom:4px;">GDP.pdf (Complex Document Extraction)</div>
<div style="font-size:12px;color:#666;margin-bottom:16px;">Accuracy, % (higher is better)</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Astra</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:100%;background:#888888;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">32.2</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6.1 Sol</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:99.4%;background:#1a1a1a;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;font-weight:600;color:#1a1a1a;margin-left:8px;">32.0</div>
</div>
<div style="display:flex;align-items:center;margin-bottom:10px;">
<div style="width:160px;font-size:13px;color:#333;">Claude Opus 5.5 (fallbacks)</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:89.4%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">28.8</div>
</div>
<div style="display:flex;align-items:center;">
<div style="width:160px;font-size:13px;color:#333;">GPT-6 Sol</div>
<div style="flex:1;background:#eee;border-radius:3px;height:22px;position:relative;">
<div style="width:87.0%;background:#b8b8b8;height:100%;border-radius:3px;"></div>
</div>
<div style="width:50px;text-align:right;font-size:13px;color:#333;margin-left:8px;">28.0</div>
</div>
<div style="font-size:11px;color:#888;margin-top:14px;">Source: OpenAI announcement; Surge AI GDP.pdf evaluation. GPT-6.1 Sol costs ~$0.38/task vs $0.80-$1.55 for Opus 5.5 and ~$1.95 for Astra.</div>
</div>
For enterprise document pipelines in legal, compliance, and wealth management, this closes the gap where teams previously felt forced to pay $10/$50 rates to avoid hallucinated numbers in fine print.
4. Multi-Step Business Workflows: AutomationBench 1.0.6
On
AutomationBench 1.0.6
, which evaluates autonomous agents executing end-to-end business workflows across 47 real SaaS tools (sales, marketing, customer operations, finance, and HR), GPT-6.1 Sol demonstrates sharp execution discipline.
At medium reasoning effort, GPT-6.1 Sol scores
35.4%
(scaling to ~
36.0%
at higher settings). This is
2.2 percentage points above Claude Opus 5.5
at medium reasoning effort, while costing approximately
one-third as much per task
($0.30 vs $0.92).
Compared to GPT-6 Sol at medium effort, this represents a
4.8 percentage point increase
.
While Anthropic's
Claude Sonnet 5.5
posted an impressive
44.7%
on AutomationBench on September 28, it incurred an average cost of
$1.14 per task
. GPT-6.1 Sol achieves its 36.0% score at
$0.30 per task
—nearly four times cheaper per task execution.
5. Scientific Research and Terminal Simulation: Terminal-Bench Science 0.1
On
Terminal-Bench Science 0.1
, which tests agents conducting scientific research workflows via bash and command-line tools (data exploration, physical simulations, differential equation fitting, and formal theorem proving), GPT-6.1 Sol more than doubles its predecessor.
At maximum reasoning effort:
GPT-6.1 Sol scores
more than 2x GPT-6 Sol
.
Task execution cost averages
$5.47 per task
.
Claude Opus 5.5 averages
$23.21 per task
on the same harness.
GPT-6 Astra averages
$23.80 per task
, reaching the benchmark ceiling at
68.1%
.
While GPT-6 Astra remains the recommended model for cutting-edge mathematical proofs and novel laboratory analysis, GPT-6.1 Sol provides over 75% cost savings for day-to-day scientific computing and quantitative backtesting.
6. Factual Reliability and Hallucination Reduction
A frequent criticism of fast mid-tier models is subtle hallucination when operating under lower reasoning budgets. In GPT-6.1 Sol, OpenAI focused on factuality curves under low effort.
On evaluation sets built from de-identified user-flagged historical errors:
At
low reasoning effort
, the proportion of responses containing a factual error dropped from
11.4% (GPT-6 Sol)
to
7.7% (GPT-6.1 Sol)
—a
32% reduction in hallucinations
.
Across all reasoning tiers, GPT-6.1 Sol's error rate remains within
1.9 percentage points of GPT-6 Astra
, while consuming less than one-fifth the compute budget.
7. Safety, Tool Transparency, and Alignment
OpenAI's accompanying system card addendum highlights noticeable gains in tool honesty and boundary adherence:
Broken Tool Disclosure:
In tests where search or API integrations were artificially degraded, GPT-6 Sol previously failed to notify users in 4.9% of runs (often guessing or hallucinating synthetic responses). GPT-6.1 Sol reduces this failure rate to
2.1%
(matching GPT-6 Astra's 1.5%).
Jailbreak and Guardrail Adherence:
In evaluations assessing attempts to bypass automated safety reviewers or exceed authorized execution scopes, GPT-6.1 Sol recorded
zero bypass attempts
, matching the clean records of Astra and Sol.
Instruction Following:
Significant improvements were noted in honoring negative constraints (e.g., "do not modify files outside
/src
" or "never expose raw API tokens").
8. Latency and Token Speed: The Upcoming Ultrafast Tier
Alongside standard generation speeds, OpenAI announced
GPT-6.1 Sol Ultrafast
, rolling out to Codex in the coming days.
According to OpenAI's DevDay announcements, Ultrafast delivers
up to 8x faster token generation
compared to standard generation speeds. For interactive developer workflows—such as tab completion, live diff suggestions, and multi-file code search—latency has historically been a bottleneck. Pairing near-Astra reasoning with 8x generation speed positions 6.1 Sol directly against high-throughput developer tools.
Key Takeaways
Astra-level coding at 20% cost:
Matching 75.2% on DeepSWE v1.1 eliminates the price premium previously required for automated repo maintenance.
Computer use reaches parity:
At 71.4% on OSWorld 2.0 (vs Astra's 73.5%), GPT-6.1 Sol is within striking distance of the frontier while cutting execution costs by 85%.
Cached inputs at $0.10/1M:
The 50% discount on cached tokens dramatically favors architectures that maintain persistent, dense context across requests.
Document extraction beats Opus 5.5:
Scoring 32.0% on GDP.pdf gives teams a viable replacement for heavy legal and financial parsing workflows.
Not yet in general ChatGPT Chat:
GPT-6.1 Sol is currently restricted to ChatGPT Work, Codex, and the API (
gpt-6.1-sol
).
Route Dynamically Across Frontier Models in Vellum
Frontier model releases no longer happen on quarterly cycles; they land weekly. Committing your production software to a single provider or static model endpoint guarantees you are either overpaying for intelligence or falling behind state-of-the-art benchmarks.
With Vellum's model-routing and prompt-engineering platform, you can test GPT-6.1 Sol against GPT-6 Astra, Claude Sonnet 5.5, and Claude Opus 5.5 in parallel across your actual production datasets. Configure fallback logic, evaluate cost-latency trade-offs, and swap models in one click without touching client code.
Build on Vellum →
Last Updated
Sep 29, 2026
Share Post
Last updated:
Sep 29, 2026
Back to all posts
What Are Plugins? A Practical Guide
Guides
|
September 29, 2026
How to Reconcile Lump-Sum Shopify Payouts in QuickBooks with AI
LLM basics
|
September 29, 2026
Official OpenAI Dots Breakdown (2026)
AI agents for your boring ops tasks. Describe what you want and your agent starts working.
Product
Pricing
Use Cases
Changelog
Status
Resources
Documentation
Blog
Community
Help Center
Company
About
Careers
Privacy
Terms
©
2026
Vellum AI, Inc. All rights reserved.