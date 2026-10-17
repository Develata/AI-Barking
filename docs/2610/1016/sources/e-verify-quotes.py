# E 组摘句核对：每条摘句须能在对应存档 txt 中逐字检索到（UTF-8，精确子串匹配）
import sys, os
sys.stdout.reconfigure(encoding="utf-8")
base = os.path.dirname(os.path.abspath(__file__))
Q = [
 ("e-e1-quantum.txt", "Claude discovered novel designs that used both fewer qubits and fewer gates than the best previously published algorithms"),
 ("e-e1-quantum.txt", "one circuit in the family uses 12% more qubits but 17x fewer gates"),
 ("e-e1-quantum.txt", "After working for 53 hours with minimal human input (limited to the initial brief and occasional prompts to think more broadly)"),
 ("e-e1-quantum.txt", "Claude’s elliptic-curve design was the result of a single 53-hour run."),
 ("e-e1-quantum.txt", "took many research groups eight years"),
 ("e-e1-quantum.txt", "reduced the number of gates in the decoder by a factor of 12 relative to Google’s previous construction"),
 ("e-e1-quantum.txt", "About one run in 10,000 needs more than that and fails, which is well within the error rate the task allows."),
 ("e-e1-quantum.txt", "We found that Claude Mythos 5 could successfully work in a closed loop"),
 ("e-e1-quantum.txt", "Working largely on its own, Claude found designs that improve on the best published"),
 ("e-e1-quantum.txt", "in some cases by an order of magnitude or more"),
 ("e-e1-quantum.txt", "the number of qubits and gates it used"),
 ("e-e1-quantum.txt", "Oct 16, 2026"),
 ("e-e1-quantum.txt", "Claude was conservative by nature."),
 ("e-e1-quantum.txt", "it prompted Claude to write Lean proofs to verify its divsteps design"),
 ("e-e1-arxiv-abs.txt", "[Submitted on 15 Oct 2026]"),
 ("e-e1-arxiv-abs.txt", "Thu, 15 Oct 2026 17:59:20 UTC"),
 ("e-e1-arxiv-abs.txt", "In seven of the eight tasks, the discovered circuits significantly reduce both Toffoli and qubit counts relative to the state of the art."),
 ("e-e1-arxiv-abs.txt", "In the eighth, the search discovers a new reversible architecture for prime-field modular inversion."),
 ("e-e1-arxiv-pdf.txt", "Extended Data Table 1 Main results across eight compilation tasks."),
 ("e-e1-arxiv-pdf.txt", "where T counts Toffoli and CCZ gates equally"),
 ("e-e1-arxiv-pdf.txt", "using Claude Mythos 5 [28] to propose"),
 ("e-e1-arxiv-pdf.txt", "TABLE S3 Resource comparison for point addition."),
 ("e-e1-arxiv-pdf.txt", "Our reconstruction, the baseline below, uses 69.5 million Toffolis and"),
 ("e-e1-arxiv-pdf.txt", "The discovered specialization uses 843 qubits while reducing the Toffoli count to 20.5 million, a factor of"),
 ("e-e1-arxiv-pdf.txt", "Naïve reversible implementation; no prior reversible compilation exists in the literature."),
 ("e-e2-expectations.txt", "39% of US adults and 91% of staff expect that AI will be able to make a Nobel-level discovery by 2030."),
 ("e-e2-expectations.txt", "For example, 91% of staff expect AI to be able to make a Nobel-level discovery by 2030, compared to 39% of US adults."),
 ("e-e2-expectations.txt", "We asked when AI will be able to do each of eight tasks as well as a skilled professional, from writing routine emails to making a Nobel-level scientific discovery."),
 ("e-e2-expectations.txt", "By 2030, 39% of US adults expect AI to be able to make a Nobel-level discovery (panel C), compared with 91% of staff."),
 ("e-e2-expectations.txt", "In August 2026, we surveyed two groups:"),
 ("e-e2-expectations.txt", "Questions about the future were asked about 2027 for one half of respondents and about 2030 for the other half. We report the 2030 responses unless we say otherwise."),
 ("e-e2-expectations.txt", "The staff survey was an internal poll that ran from August 9 to 12 and drew 1,334 respondents from every part of the company. It is unweighted, and everyone was asked about 2030."),
 ("e-e2-expectations.txt", "Morning Consult fielded the survey of US adults between August 11 and 23 and reached 10,980 US adults."),
 ("e-e2-expectations.txt", "The largest change was a rise in the share of US adults expecting a Nobel-level discovery by 2030 to 44%, up from 39%."),
 ("e-e2-expectations.txt", "Oct 16, 2026"),
 ("e-e3-google-wetlab.txt", "October 16, 2026"),
 ("e-e3-google-wetlab.txt", "causing both target specificity (from 75.0% to 19.1%) and true, bead-corrected hit rate (from 10.2% to 3.7%) to plunge."),
 ("e-e3-google-wetlab.txt", "to recover specificity (72.4%) and surge ninefold in on-target hit rate (33.5%)."),
 ("e-e3-google-wetlab.txt", "Across three rounds on two protein targets with varying difficulty levels"),
 ("e-e3-google-wetlab.txt", "On myoglobin, as agents prioritized raw binding affinity in Round 2, they heavily enriched G-quadruplex"),
 ("e-e3-aptamer-pdf.txt", "on-target specificity dropped from 75.0%"),
 ("e-e3-aptamer-pdf.txt", "restoring on-target specificity to 72.4%"),
 ("e-e3-aptamer-pdf.txt", "in Round 1 to 19.1% in Round 2"),
 ("e-e3-aptamer-pdf.txt", "bind exclusively to the intended target without off-target cross-reactivity"),
 ("e-e3-aptamer-pdf.txt", "2026-10-12"),
 ("e-e4-starskirmish.txt", "Each LLM gets one hour to write a Protoss bot in C++, which is then evaluated against other LLMs' bots and established human written bots"),
 ("e-e4-starskirmish.txt", "Against Stardust, PurpleWave and BananaBrain, the best model, GPT-6 Astra, wins only 7% of its games."),
 ("e-e4-starskirmish.txt", "1 hour reasoning limits are a factor, but these matches are played on three 2 player maps. Real human written bot tournament conditions are significantly harder."),
 ("e-e4-starskirmish.txt", "GPT-6 Astra is the top scoring StarSkirmish Bench v0.2 LLM, a clear step ahead of GPT-6.1 Sol in second."),
 ("e-e4-starskirmish.txt", "Gemini 4 Argon, Claude Opus 5.5 and Claude Fable 5.1 are functionally tied behind them, all scoring between 37 and 39."),
 ("e-e4-starskirmish.txt", "Gemini 4 Argon runs at high, the highest reasoning effort available to it; every other model runs at max."),
 ("e-e4-starskirmish.txt", "Scores are average win rates (0–100) of each model's 5 runs against a roster of competitive human written bots."),
 ("e-e4-starskirmish.txt", "v0.2 runs each model in its lab's 1st party CLI harness at the highest reasoning setting it offers."),
 ("e-e4-starskirmish.txt", "The CLIs' web search and web fetch tools are turned off, and the agent has no other internet access."),
 ("e-e4-starskirmish.txt", "Gemini 4 Argon places 3rd, and Muse Wonder Full 7th."),
 ("e-e4-starskirmish.txt", "October 15, 2026"),
 ("e-e4-starskirmish.txt", "Heartbreak Ridge, Benzene and Destination"),
]
bad = 0
cache = {}
for fn, q in Q:
    if fn not in cache:
        with open(os.path.join(base, fn), encoding="utf-8") as f:
            cache[fn] = f.read()
    ok = q in cache[fn]
    if not ok: bad += 1
    print(("OK   " if ok else "MISS ") + fn + " | " + q[:70])
print("total %d, miss %d" % (len(Q), bad))
sys.exit(1 if bad else 0)
