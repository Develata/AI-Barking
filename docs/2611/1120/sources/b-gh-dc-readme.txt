# Language Models Can Rewrite Their Own Weights — reproduction

Independent implementation of [arXiv:2611.21885v2](https://arxiv.org/html/2611.21885).
Both protocol controllers and evaluation workflows are implemented; numerical replication is not yet verified.
See [REPRODUCTION_STATUS.md](REPRODUCTION_STATUS.md) for completed and remaining work.
No benchmark accuracy claim has been reproduced yet. The [paper-to-code map](docs/PROTOCOL_MAP.md) lists implementation coverage and validation limits.

The implementation separates immutable baseline weights, validated edits, and
paired evaluation. In repair experiments the baseline accessor returns damaged
weights. The editor must run on clean weights, while every requested experiment
runs on the damaged baseline with or without its current edits.

```bash
python -m pip install -e '.[test,inference]'
python -m pytest -q
```

Optional inference dependencies are declared separately (`.[inference]`). Full
27B experiments need a suitable GPU and model checkpoints. Numerical unit tests
run on CPU. Tested library versions are recorded in
[`configs/verified-environment.json`](configs/verified-environment.json). The downloaded article in `reference/` is source material, not author
code. Stable hashing and Monte Carlo seeds in this implementation are explicit
reconstruction choices because their exact originals are not published.

Implemented execution interfaces live in `backend.py` and `forward.py`.
`TransformersBackend` supports real generation and hidden-state reads, restoring
clean editor weights after every requested experiment, including failed ones.
The test suite creates a tiny random transformer locally; it downloads no model.
A persistent code worker and session CLI for both protocols are available.

First-protocol sessions can now run against an existing Hugging Face safetensors
checkpoint and a prepared practice JSONL (`id`, `question`, `answer`, all strings):

```bash
self-rewrite session --checkpoint /path/to/Qwen3.5-27B \
  --practice /path/to/practice.jsonl --work runs/self-27b --short
# Resume after a framework interruption (Python variables restart; files persist):
self-rewrite session --checkpoint /path/to/Qwen3.5-27B \
  --practice /path/to/practice.jsonl --work runs/self-27b --short --resume
```

For repair, create a separate corruption overlay with `self-rewrite damage`,
then pass its safetensors file as `--damage` to the session command. The damage
record must remain outside the editor's work folder. `self-rewrite export`
writes a separate checkpoint from baseline + damage + saved edits. Use each
subcommand's `--help` for arguments. The original checkpoint is never overwritten.

The CPU worker exposes numerical libraries and mediated weight/artifact access.
Python variables persist until a timeout or restart. It refuses gradient,
optimizer, network, model-loading, direct filesystem and process APIs. These are
protocol restrictions. On Linux, the worker additionally installs a seccomp filter
after trusted library initialization. It denies new filesystem opens, sockets,
processes/threads, and process-inspection syscalls; it accesses data only through
an existing pipe and host-validated capabilities. Startup fails if sealing fails.
Worker messages use a data-only JSON/tensor format, never pickle deserialization
on the host. Tests verify denied file/network/process access directly. This is
process isolation, not a virtual machine or a proof against all native/kernel exploits.

Session logs and state live in `events.jsonl`, `state.json`, `forward.jsonl`,
`census.jsonl`, `gen_*.jsonl`, and `read_*.pt`. Saved edit tensors live in `edited/`.
Tests with a scripted editor verify orchestration; they do not demonstrate that
a language model discovers a successful edit.

Data preparation and evaluation:

```bash
self-rewrite import-deepscaler --source data/raw/deepscaler.json --output data/deepscaler.jsonl
self-rewrite prepare --pool data/deepscaler.jsonl --output data/splits --seed 0 \
  --exclude math500=benchmarks/math500.jsonl \
  --exclude aime2024=benchmarks/aime2024.jsonl \
  --exclude aime2025=benchmarks/aime2025.jsonl \
  --exclude hmmt2025=benchmarks/hmmt2025.jsonl \
  --exclude amc2023=benchmarks/amc2023.jsonl \
  --exclude olympiadbench=benchmarks/olympiadbench.jsonl \
  --exclude gpqa-diamond=benchmarks/gpqa-diamond.jsonl
self-rewrite evaluate --checkpoint /path/to/Qwen3.5-27B \
  --edits runs/self-27b/edited --problems data/splits/confirmatory.jsonl \
  --practice data/splits/practice.jsonl --output runs/evaluation --samples 8
```

Benchmark input files use the same canonical JSONL schema. Preparation requires
all seven exclusion sources and records source hashes and selected IDs. Matching
normalizes whitespace, Unicode, and case; it does **not** detect paraphrases.
`--manual-exclusions` accepts a JSON list of pool IDs from an external semantic
overlap audit. Until that audit is done, complete benchmark decontamination is
unverified. The paper's original split IDs were not released: seeded splitting
here is a reconstruction, and filtering held-out positions 500–999 may produce
a count other than 490. Within-pool repeats are recorded; evaluation refuses
practice overlap even when that means a further cleaned exploratory subset.

Evaluation writes `plan.json` before sampling, hashes model/checkpoint assets and
edits, saves each completed pair atomically, and writes `report.json` only after
all pairs finish. Repeat the same command to resume. Changing the plan or weights
requires a new output directory. Multiple samples are tested with a one-sided
problem-level sign-flip test (100,000 draws); single-sample runs use the paired
sign test. Fractions in reports are on a 0–1 scale. No intermediate significance
results are emitted. Serial execution and restart-related resampling of an
incomplete pair may differ from the paper's vLLM generation.

Reported rules can be replayed independently of discovery with `self-rewrite
replay --help`: `repair` supports all/named/early ablations, `direction` requires
a practice-derived direction tensor, and `time-bias` implements the reported
+0.5 shift across 48 linear-attention biases. The explicit Figure 12 repair list
skips layer 4's MLP. Replays require an empty edit store. They are not evidence
that an editor model independently discovers those rules. The original measured
direction vectors are unavailable, so direction replay is a recipe reconstruction.


Second-protocol sessions use the prepared editor-visible and reserved splits:

```bash
self-rewrite session --checkpoint /path/to/Qwen3.5-27B --protocol 2 \
  --practice data/splits/visible.jsonl --reserved data/splits/reserved.jsonl \
  --work runs/protocol2-seed0 --turn-seed 0
```

Each session starts from base weights. Use a new work directory and turn seeds
0–7 for eight sessions; do not compose submitted edits. Python supports
`save_candidate(label)`, `list_candidates()`, and `restore_candidate(label)`.
A generate request can compare up to four saved candidates using
`"weights":"both","candidates":["first","second"]`. Its 128-answer limit counts
all candidates plus the shared baseline. Candidate labels are immutable.

The reserved check uses 256 reserved problems with two samples, outside the
session's forward budget, followed by a SUBMIT/WITHDRAW decision with no further
code execution. Its full answers are saved for audit in `reserved_check.json`,
which the worker cannot access; only aggregate feedback reaches the editor.
Completed checks survive restart without resampling. A malformed final decision
is treated as withdrawal (an explicit reconstruction choice). This flow has
been tested using a scripted editor and real tiny-model forward calls, not yet
with Qwen discovering its own edit.

Actual Qwen integration verification:

```bash
python scripts/download_qwen.py
python scripts/smoke_qwen.py
```

The downloader records the resolved official revision. The smoke script runs
Qwen3.5-2B on GPU with short synthetic text/image prompts, reads both linear- and
full-attention layers, applies one tensor edit and verifies clean-weight restoration.
Its JSON report is `runs/qwen2b-smoke/report.json`. Short-budget answers are not
paper benchmark results. The installed Transformers backend currently uses the
correct but slower reference kernels when causal-conv1d/flash-linear-attention
are absent. Tensor helpers exclude auxiliary MTP weights unused by inference.

Image evaluation accepts canonical JSONL records with the additional fields
`image` (local path), `benchmark`, and `grading`. Relative image paths resolve
against the manifest directory. Supported benchmark labels are `mathvista`,
`mathverse`, `mmstar`, `hallusionbench`, `pope`, and `screenspot-v2`. Graders are
`image-math` (Math-Verify), `choice` (one letter; comma-separated reference labels allow equivalent options), `choice-set` (all listed letters required), `yesno`, and `point`.
Image answers must close thinking; the final boxed or explicitly stated answer
is graded. No judge model is used. These explicit extraction rules are a
reconstruction; exact original image graders were not released.

For `point`, `answer` is a JSON string containing normalized `bbox:[x1,y1,x2,y2]`
and `coordinate_scale:[sx,sy]`. The prompt requests one coordinate pair in that
scale and the grader checks containment after normalization. The target box is
never placed in the model's prompt.

`self-rewrite prepare-images --source mathvista=... --source mathverse=...`
(with all six sources, `--output` and optional `--seed`) shuffles each benchmark,
draws 300, and selects its first 100. It records selected IDs and image hashes.
Use `self-rewrite evaluate --samples 1` with the resulting `images.jsonl` to
compare damage with repair; the report contains both pooled and per-benchmark
paired sign tests. Image content hashes are part of the precommitted evaluation
plan. Original-format adapters and pinned downloads are available for all six datasets.
Run `python scripts/download_images.py` followed by `python scripts/import_images.py`
(with `.[data]` installed). Canonical manifests do not establish original split equivalence.

The full session schedule is in `configs/paper-sessions.json` (nine first-protocol
sessions plus eight second-protocol sessions). Execute one unchanged scheduled
job at a time, using independent base checkpoints:

```bash
self-rewrite run-plan --plan configs/paper-sessions.json \
  --job protocol2-27B-session1 --checkpoints checkpoints \
  --data data/splits --output runs/paper
self-rewrite history --output runs/paper
```

Repair jobs create the specified damaged row block outside editor work. History
uses submission decisions alone: a submission replaces the deployed edit, and a
withdrawal retains the previous model. It never picks winners using held-out
scores. Resumes check model and practice/reserved data hashes, reconcile committed
forward passes, and disclose interrupted actions before continuing.

`python scripts/smoke_session.py` exercises a real Qwen editor under shortened
limits and synthetic practice. The verified run completed without an edit or
valid tool action; that outcome is retained in its transcript. This is not a
paper-budget run or evidence of successful self-improvement.

`self-rewrite import-benchmark` now converts source-format text/image exports;
see [data sources and conversion details](docs/DATA_SOURCES.md). This reduces
manual JSONL preparation but does not verify original dataset/split equivalence.

An optional serving backend is available with `--backend vllm`. Its generation
and weight-restoration contract is tested, but an installed vLLM runtime has not
been validated in this environment. See [vLLM implementation and validation
status](docs/VLLM.md) before using it. The default remains the verified
Transformers path; neither backend choice silently falls back to the other.

Staged second-protocol evaluation is available through `evaluate-staged`, with
shared baseline completions, two screening runs, case retests, and a precommitted
optional second look. See [evaluation procedures](docs/EVALUATION.md). `evaluate`
also accepts `--baseline-cache` for exact baseline reuse across submitted edits.
Explicit overlap-reporting flags support the paper's audited repeated-question
comparisons without silently substituting an easier or smaller evaluation set.


`audit-overlap` retrieves reviewable near-duplicate question pairs; it does not
automatically discard problems. `reviewed-exclusions` exports IDs only after
all retrieved pairs have explicit decisions and rationales. This supplements
exact matching but cannot prove complete semantic decontamination.
