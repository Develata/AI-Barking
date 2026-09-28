[Artificial Analysis](/)

[Premium](/pricing?source=nav&from=%2Fevaluations%2Fterminalbench-4-0)

[Log in](/login)

[All evaluations](/evaluations)

# Terminal-Bench 4.0 Benchmark Leaderboard

[Intelligence Index](/evaluations?attribute=intelligence-index)[Agentic](/evaluations?skill=agentic)[Tool Use](/evaluations?skill=tool-use)[Coding](/evaluations?skill=coding)[Reasoning](/evaluations?skill=reasoning)[Instruction Following](/evaluations?skill=instruction-following)

A harder 66-task benchmark of complex terminal work across software, machine learning, science, operations, security, hardware, and media, with recalibrated compute and time allowances and improved instructions, environments, and verifiers.

Terminal-Bench 4.0 is a harder agentic terminal benchmark from the [Laude Institute](https://www.laude.org/ "https://www.laude.org/"), Stanford University researchers, and open-source contributors. Its 66 tasks span software, machine learning, science, operations, security, hardware, and media.

The v4.0 release recalibrates compute and time allowances, improves the fairness of instructions, environments, and verifiers, and removes eight tasks that were saturated, refusal-prone, publicly solved, or affected by unresolved quality issues.

Each task has its own set of tests. The agent works through the task in a terminal, and the task passes only if every test passes.

We run all 66 Terminal-Bench 4.0 tasks with the mini-swe-agent harness and report pass@1 averaged over three repeats per task.

All evaluations are conducted independently by Artificial Analysis. More information can be found in our [Terminal-Bench 4.0 methodology](/methodology/intelligence-benchmarking#terminal-bench-v4-0 "/methodology/intelligence-benchmarking#terminal-bench-v4-0").

#### Publication

[View on arXiv](https://arxiv.org/abs/2601.11868)

Terminal-Bench: Benchmarking Agents on Hard, Realistic Tasks in Command Line Interfaces

Mike A. Merrill, Alexander G. Shaw, Nicholas Carlini, Boxuan Li, Harsh Raj, Ivan Bercovich, Lin Shi, Jeong Yeon Shin, Thomas Walshe, E. Kelly Buchanan, Junhong Shen, Guanghao Ye, Haowei Lin, Jason Poulos, Maoyu Wang, Marianna Nezhurina, Jenia Jitsev, Di Lu, Orfeas Menis Mastromichalakis, Zhiwei Xu, and .

AI agents may soon become capable of autonomously completing valuable, long-horizon tasks in diverse domains. Current benchmarks either do not measure real-world tasks, or are not sufficiently difficult to meaningfully measure frontier models. To this end, we present Terminal-Bench 2.0: a carefully curated hard benchmark composed of 89 tasks in computer terminal environments inspired by problems from real workflows. Each task features a unique environment, human-written solution, and comprehensive tests for verification. We show that frontier models and agents score less than 65% on the benchmark and conduct an error analysis to identify areas for model and agent improvement. We publish the dataset and evaluation harness to assist developers and researchers in future work at https://www.tbench.ai/.

[

![2601.11868](/img/logos/arxiv.svg)2601.11868

](https://arxiv.org/abs/2601.11868 "https://arxiv.org/abs/2601.11868")[

![Terminal-Bench 4.0](/img/logos/tbench.svg)Terminal-Bench 4.0

](https://www.tbench.ai/leaderboard/terminal-bench/4.0 "https://www.tbench.ai/leaderboard/terminal-bench/4.0")[

![laude-institute/terminal-bench](/img/logos/github.svg)laude-institute/terminal-bench

](https://github.com/laude-institute/terminal-bench "https://github.com/laude-institute/terminal-bench")

#### Terminal-Bench 4.0

Claude Sonnet 5.5 (Adaptive Reasoning, Max Effort, Default Fallback) scores the highest on Terminal-Bench 4.0 with a score of 63.6%, followed by Claude Opus 5.5 (Adaptive Reasoning, Max Effort, Default Fallback) with a score of 59.6% and Claude Opus 5.5 (Adaptive Reasoning, Xhigh Effort, Default Fallback) with a score of 59.6%

## Score

### Terminal-Bench 4.0: Score

Benchmark developed by the Laude Institute, Stanford, and open-source contributors · Independently benchmarked by Artificial Analysis

### Terminal-Bench 4.0: Score vs. Cost per Task

Terminal-Bench 4.0 score vs. average cost per task (USD) · Lower is better

Most attractive quadrant

Pareto line

Average cost per task in the evaluation. Costs are split by input, cache hit, cache write, reasoning, and answer token pricing where canonical token counts are available.

## Token Usage

### Terminal-Bench 4.0: Output Tokens per Task

Output tokens used to run one task, broken down by reasoning and answer tokens

The average number of answer and reasoning tokens produced per benchmark task in this evaluation.

## Cost

### Terminal-Bench 4.0: Cost per Task

Average cost per task (USD), broken down by input, cache hit, cache write, reasoning, and answer tokens

Average cost per task in the evaluation. Costs are split by input, cache hit, cache write, reasoning, and answer token pricing where canonical token counts are available.

## Speed

### Terminal-Bench 4.0: Time per Task

Weighted average decode time (minutes) per task; excludes TTFT and overhead time · Lower is better

The weighted average time (seconds) per evaluation task. This is calculated by dividing output tokens per task by output speed, weighted by the relative weights of each benchmark in the evaluation.

## Score vs. Release Date

### Terminal-Bench 4.0: Score vs. Release Date

Most attractive region

## Example Tasks

## Explore Evaluations

[

![Artificial Analysis Intelligence Index v4.3.2](/img/logo-icon.svg)Artificial Analysis Intelligence Index v4.3.2

A composite benchmark aggregating ten challenging evaluations to provide a holistic measure of AI capabilities across mathematics, science, coding, and reasoning.

](/evaluations/artificial-analysis-intelligence-index)[

![Artificial Analysis Openness Index](/img/logo-icon.svg)Artificial Analysis Openness Index

A composite measure providing an industry standard to communicate model openness for users and developers.

](/evaluations/artificial-analysis-openness-index)[

![AA-Briefcase v1.1: Agentic Knowledge Work Benchmark](/img/logo-icon.svg)AA-Briefcase v1.1: Agentic Knowledge Work Benchmark

A private evaluation developed by Artificial Analysis for frontier agentic capability in long-horizon knowledge work, testing agents on realistic business workflows that require deliverables such as spreadsheets, presentations, and memos.

](/evaluations/aa-briefcase)[

![GDPval-AA v2.1 Leaderboard](/img/logo-icon.svg)GDPval-AA v2.1 Leaderboard

GDPval-AA v2.1 is Artificial Analysis' evaluation framework for OpenAI's GDPval dataset. It tests AI models on real-world tasks across 44 occupations and 9 major industries. Models are given shell access and web browsing capabilities in an agentic loop via Stirrup to solve tasks, with Elo ratings derived from blind pairwise comparisons.

](/evaluations/gdpval-aa)[

![APEX-Agents-AA Benchmark Leaderboard](/_next/image?url=%2Fimg%2Flogos%2Fmercor.png&w=32&q=75)APEX-Agents-AA Benchmark Leaderboard

Artificial Analysis' implementation of the APEX-Agents benchmark, testing AI agents on long-horizon, cross-application tasks in professional-services environments with realistic application tooling.

](/evaluations/apex-agents-aa)[

![AA-AnalystAgent Benchmark Leaderboard](/img/logo-icon.svg)AA-AnalystAgent Benchmark Leaderboard

Artificial Analysis' data analysis benchmark, testing AI agents on their ability to work with spreadsheets and documents to answer quantitative questions a Business Analyst or Data Analyst would face day-to-day.

](/evaluations/aa-analyst-agent)[

![AutomationBench-AA: Agentic SaaS Workflow Benchmark](/img/logos/zapier.svg)AutomationBench-AA: Agentic SaaS Workflow Benchmark

A benchmark measuring agentic task completion across simulated SaaS application environments, scoring the share of each task's objectives completed without guardrail violations.

](/evaluations/automationbench-aa)[

![Harvey LAB-AA Benchmark Leaderboard](/img/logos/harvey.svg)Harvey LAB-AA Benchmark Leaderboard

Artificial Analysis' implementation of Harvey's Legal Agent Benchmark (LAB), testing AI agents on real-world legal work from Harvey's dataset of 120 private tasks spanning 24 legal practice areas. The agent reads case documents in a sandbox and produces legal deliverables (e.g., memos, disclosure schedules, deposition summaries), graded criterion-by-criterion by a single LLM rubric judge.

](/evaluations/harvey-lab-aa)[

![GDP.pdf Benchmark Leaderboard](/_next/image?url=%2Fimg%2Flogos%2Fsurge.jpg&w=32&q=75)GDP.pdf Benchmark Leaderboard

Artificial Analysis' implementation of Surge AI's GDP.pdf benchmark, testing whether language models can reason over long, real-world professional documents and satisfy detailed task-specific criteria.

](/evaluations/gdp-pdf)[

![EnterpriseOps-Gym-AA Benchmark Leaderboard](/img/logos/servicenow.svg)EnterpriseOps-Gym-AA Benchmark Leaderboard

Artificial Analysis' independent implementation of ServiceNow's EnterpriseOps-Gym, an agentic benchmark testing whether LLM agents can complete stateful, multi-step enterprise workflows across eight business domains via live tool use, graded on the final state of the underlying databases.

](/evaluations/enterprise-ops-gym-aa)[

![Terminal-Bench 4.0 Benchmark Leaderboard](/img/logos/tbench.svg)Terminal-Bench 4.0 Benchmark Leaderboard

A harder 66-task benchmark of complex terminal work across software, machine learning, science, operations, security, hardware, and media, with recalibrated compute and time allowances and improved instructions, environments, and verifiers.

](/evaluations/terminalbench-4-0)[

![Terminal-Bench-Science 0.1 Benchmark Leaderboard](/img/logos/tbench_science.svg)Terminal-Bench-Science 0.1 Benchmark Leaderboard

A 70-task benchmark of research workflows authored and reviewed by domain experts across the life, physical, earth, mathematical, and engineering sciences, each completed in a terminal and checked by its own set of tests.

](/evaluations/terminal-bench-science)[

![Artificial Analysis Long Context Reasoning Benchmark Leaderboard](/img/logo-icon.svg)Artificial Analysis Long Context Reasoning Benchmark Leaderboard

A challenging benchmark measuring language models' ability to extract, reason about, and synthesize information from long-form documents ranging from 10k to 100k tokens (measured using the cl100k\_base tokenizer).

](/evaluations/artificial-analysis-long-context-reasoning)[

![AA-Omniscience: Knowledge and Hallucination Benchmark](/img/logo-icon.svg)AA-Omniscience: Knowledge and Hallucination Benchmark

A benchmark measuring factual recall and hallucination across various economically relevant domains.

](/evaluations/omniscience)[

![SciCode Benchmark Leaderboard](/img/logos/scicode.svg)SciCode Benchmark Leaderboard

A scientist-curated coding benchmark featuring 288 test set subproblems from 80 laboratory problems across 16 scientific disciplines.

](/evaluations/scicode)[

![Humanity's Last Exam Benchmark Leaderboard](/img/logos/hle.svg)Humanity's Last Exam Benchmark Leaderboard

A frontier-level benchmark with 2,500 expert-vetted questions across mathematics, sciences, and humanities, designed to be the final closed-ended academic evaluation.

](/evaluations/humanitys-last-exam)[

![CritPt Benchmark Leaderboard](/_next/image?url=%2Fimg%2Flogos%2Fcritpt.png&w=32&q=75)CritPt Benchmark Leaderboard

A benchmark designed to test LLMs on research-level physics reasoning tasks, featuring 71 composite research challenges.

](/evaluations/critpt)[

GPQA Diamond Benchmark Leaderboard

The most challenging 198 questions from GPQA, where PhD experts achieve 65% accuracy but skilled non-experts only reach 34% despite web access.

](/evaluations/gpqa-diamond)[

![ITBench-AA Benchmark Leaderboard](/img/logos/ibm.svg)ITBench-AA Benchmark Leaderboard

Artificial Analysis' implementation of IBM's ITBench benchmark, testing AI agents on Kubernetes incident root-cause analysis from offline incident snapshots. The agent inspects alerts, events, traces, and topology and identifies the contributing-factor entities (deployments, pods, namespaces, network policies, etc.) responsible for the failure.

](/evaluations/itbench-aa)[

![MMMU-Pro Benchmark Leaderboard](/_next/image?url=%2Fimg%2Flogos%2Fmmmu.png&w=32&q=75)MMMU-Pro Benchmark Leaderboard

An enhanced MMMU benchmark that eliminates shortcuts and guessing strategies to more rigorously test multimodal models across 30 academic disciplines.

](/evaluations/mmmu-pro)[

![IFBench Benchmark Leaderboard](/img/logos/ai2.svg)IFBench Benchmark Leaderboard

A benchmark evaluating precise instruction-following generalization on 58 diverse, verifiable out-of-domain constraints that test models' ability to follow specific output requirements.

](/evaluations/ifbench)[

![Medical Long Context Reasoning (MLCR-AA)](/img/logos/wisedocs.svg)Medical Long Context Reasoning (MLCR-AA)

An open benchmark from Wisedocs measuring how well models reason over long, fragmented medical records, performing the multi-document synthesis claims professionals rely on when reviewing insurance and healthcare cases.

](/evaluations/mlcr-aa)[

![𝜏³-Banking Benchmark Leaderboard](/img/logos/sierra.svg)𝜏³-Banking Benchmark Leaderboard

A fintech customer-support benchmark from the 𝜏-Knowledge framework that tests whether agents can navigate a large unstructured knowledge base and execute multi-step tool calls to resolve realistic banking workflows.

](/evaluations/tau3-banking)[

![Terminal-Bench 2.1 Benchmark Leaderboard](/img/logos/tbench.svg)Terminal-Bench 2.1 Benchmark Leaderboard

A verified refresh of Terminal-Bench 2.0 — 89 curated tasks across software engineering, system administration, data processing, model training, and security, with environment and instruction fixes so scores reflect agent capability rather than environment gaps.

](/evaluations/terminalbench-2-1)[

![Terminal-Bench Hard Benchmark Leaderboard](/img/logos/tbench.svg)Terminal-Bench Hard Benchmark Leaderboard

An agentic benchmark evaluating AI capabilities in terminal environments through software engineering, system administration, and data processing tasks.

](/evaluations/terminalbench-hard)[

![𝜏²-Bench Telecom Benchmark Leaderboard](/img/logos/sierra.svg)𝜏²-Bench Telecom Benchmark Leaderboard

A dual-control conversational AI benchmark simulating technical support scenarios where both agent and user must coordinate actions to resolve telecom service issues.

](/evaluations/tau2-bench)[

![MMLU-Pro Benchmark Leaderboard](/_next/image?url=%2Fimg%2Flogos%2Ftiger_lab.jpg&w=32&q=75)MMLU-Pro Benchmark Leaderboard

An enhanced version of MMLU with 12,000 graduate-level questions across 14 subject areas, featuring ten answer options and deeper reasoning requirements.

](/evaluations/mmlu-pro)[

![LiveCodeBench Benchmark Leaderboard](/img/logos/livecodebench.svg)LiveCodeBench Benchmark Leaderboard

A contamination-free coding benchmark that continuously harvests fresh competitive programming problems from LeetCode, AtCoder, and CodeForces, evaluating code generation, self-repair, and execution.

](/evaluations/livecodebench)[

![MATH-500 Benchmark Leaderboard](/img/logos/openai.svg)MATH-500 Benchmark Leaderboard

A 500-problem subset from the MATH dataset, featuring competition-level mathematics across six domains including algebra, geometry, and number theory.

](/evaluations/math-500)[

![AIME 2025 Benchmark Leaderboard](/img/logos/aops.svg)AIME 2025 Benchmark Leaderboard

All 30 problems from the 2025 American Invitational Mathematics Examination, testing olympiad-level mathematical reasoning with integer answers from 000-999.

](/evaluations/aime-2025)[

![Global-MMLU-Lite Benchmark Leaderboard](/img/logos/cohere.svg)Global-MMLU-Lite Benchmark Leaderboard

A lightweight, multilingual version of MMLU, designed to evaluate knowledge and reasoning skills across a diverse range of languages and cultural contexts.

](/evaluations/global-mmlu-lite)

本页面提供中文版本
