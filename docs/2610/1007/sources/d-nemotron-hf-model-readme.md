---
library_name: transformers
license: other
license_name: openmdw-1.1
license_link: >-
  https://openmdw.ai/license/1-1/
pipeline_tag: text-generation
language:
- en
tags:
- nvidia
- pytorch
- nemotron-3
- competitive-programming
- code-generation
track_downloads: true
---

# NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4

<div align="center" style="line-height: 1;">
<a href="https://arxiv.org/abs/2609.02849" target="_blank" style="margin: 2px;">
    <img alt="Paper" src="https://img.shields.io/badge/📝Paper-Read Now!-536af5?color=76B900&logoColor=white" style="display: inline-block; vertical-align: middle;"/>
</a>
<a href="https://github.com/NVIDIA-NeMo/Skills" target="_blank" style="margin: 2px;">
    <img alt="NeMo Skills" src="https://img.shields.io/badge/🛠️NeMo_Skills-Inference_%26_Eval_Recipes-76B900?logoColor=white" style="display: inline-block; vertical-align: middle;"/>
</a>
</div>
<div align="center" style="line-height: 1;">
  <a href="https://developer.nvidia.com/nemotron" target="_blank" style="margin: 2px;">
    <img alt="Homepage" src="https://img.shields.io/badge/🏠Nemotron Developer Page-Learn More Here!-536af5?color=76B900&logoColor=white" style="display: inline-block; vertical-align: middle;"/>
  </a>
<a href="https://discord.gg/9xpKQtVvrk" target="_blank" style="margin: 2px;">
    <img alt="Discord" src="https://img.shields.io/badge/Discord-NVIDIA%20AI%20Developer-7289da?logo=discord&logoColor=white&color=7289da" style="display: inline-block; vertical-align: middle;"/>
  </a>
</div>

<div style="text-align: center; line-height: 1;">
  <a href="https://openmdw.ai/license/1-1/" style="margin: 2px;">
    <img alt="License" src="https://img.shields.io/badge/License-OpenMDW--1.1-f5de53" style="display: inline-block; vertical-align: middle;"/>
  </a>
</div>

## Model Summary

| | |
|:---|:---|
| **Total Parameters** | 550B (55B active) |
| **Architecture** | Based on Nemotron-3-Ultra |
| **Context Length** | Up to 262,144 tokens |
| **Best For** | Competitive programming, algorithmic problem solving, code-reasoning research and benchmarking, test-time-compute research |
| **Reasoning Mode** | Structured Explanation → Confidence → Answer response format with step-by-step reasoning before the final C++ solution |
| **License** | [OpenMDW License Agreement, version 1.1](https://raw.githubusercontent.com/OpenMDW/OpenMDW/refs/heads/main/1.1/LICENSE.OpenMDW-1.1) |
| **Release Date** | Hugging Face: 09/03/2026 via [model page](https://huggingface.co/nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4) |

## Quick Start

> This checkpoint is a competitive-programming specialist model, not a general-purpose chat or agent model. It supports commercial and non-commercial applications, including research and evaluation contexts such as competitive programming benchmarks, code-reasoning research, and test-time-compute studies (for example, GenCorrect-style iterative refinement). See [Use Case](#use-case) below.

## Model Overview

**Model Developer:** NVIDIA Corporation

**Model Development:** Fine-tuned from [NVIDIA-Nemotron-3-Ultra-550B-A55B](https://huggingface.co/nvidia/NVIDIA-Nemotron-3-Ultra-550B-A55B-BF16)

### What is Nemotron?

NVIDIA Nemotron™ is a family of open models with open weights, training data, and recipes, delivering leading efficiency and accuracy for building specialized AI agents.

## Description

Nemotron-Labs-3-Competitive-Coding is a competitive-programming specialist model based on Nemotron-3-Ultra, fine-tuned for one epoch on 477,642 synthetic reasoning traces distilled from GLM-5.2 across 22,000 curated problems spanning 16 regional and international competitive-programming contest families. Selected as the SFT teacher for its higher accuracy and roughly 30% shorter generations compared to a DeepSeek-V4-Flash-trained variant, GLM-5.2 distillation yields a model that, combined at inference time with GenCorrect — an iterative closed-loop test-time compute strategy that generates diverse candidate solutions, incorporates evaluator feedback, and refines subsequent generations under a fixed submission budget — was evaluated live and prospectively on the IOI 2026 problem set under official contest time, internet-access, and submission constraints, scoring 535.4 out of 600 and surpassing both the gold-medal threshold (361.12) and the top human contestant's score (498.27), making it the first AI system reported to outscore the highest-scoring human contestant on an IOI problem set.

This model is ready for commercial or non-commercial use.

## License/Terms of Use

**Governing Download Terms:** Use of this model is governed by the [OpenMDW License Agreement, version 1.1](https://raw.githubusercontent.com/OpenMDW/OpenMDW/refs/heads/main/1.1/LICENSE.OpenMDW-1.1) (OpenMDW-1.1).

### Benchmarks

| Benchmark | Nemotron-Labs-3-Competitive-Coding |
| :--- | :---: |
| IOI 2025 — with GenCorrect (5 rounds) | 502.0 / 600 |
| ICPC 2025 — with GenCorrect (5 rounds) | 9.6 / 12 problems solved |
| LiveCodeBench Pro — Pass@1 | 74.5% |
| IOI 2026 — live, prospective, competition-specific run | 535.4 / 600 (Gold; exceeds gold threshold of 361.12 and top human score of 498.27) |

All results are from the source paper, *Post-Training Language Models for Gold-Medal Performance in Coding Competitions* (NVIDIA, arXiv:2609.02849). IOI Score@1/Score@200 and GenCorrect results are averaged over multiple independent runs; see the paper for full methodology. IOI 2025 was used as a development benchmark; IOI 2026 results are from a single prospective live run conducted under official IOI time, internet-access, and submission constraints before problems were publicly released, and were not part of the official IOI rankings.

### Deployment Geography: Global

### Use Case

Nemotron-Labs-3-Competitive-Coding is intended for researchers and developers evaluating or advancing frontier code-reasoning capability, particularly on competitive programming and algorithmic problem solving where a solution must satisfy strict correctness, efficiency, and hidden test-case constraints. It is suited to benchmarking and research on long-horizon reasoning, agentic code generation, and test-time compute strategies such as GenCorrect-style iterative refinement, rather than general-purpose chat, instruction-following, or production coding-assistant deployment. Use in safety-critical or real-time production systems requires further evaluation and safeguards appropriate to the application.

### Release Date

Hugging Face: 09/03/2026 via [model page](https://huggingface.co/nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4)

## Reference(s)

* [Post-Training Language Models for Gold-Medal Performance in Coding Competitions (arXiv:2609.02849)](https://arxiv.org/abs/2609.02849)
* [NVIDIA Nemotron 3 model family on Hugging Face](https://huggingface.co/collections/nvidia/nvidia-nemotron-v3)
* [NeMo Skills — inference and evaluation recipes](https://github.com/NVIDIA-NeMo/Skills)

## Model Architecture

- **Architecture Type:** Other
- **Architecture Description:** Mamba2-Transformer Hybrid Latent Mixture of Experts (LatentMoE) with Multi-Token Prediction (MTP)
- **Network Architecture:** Other
- **Network Architecture Description:** Nemotron Hybrid LatentMoE
- **Base Model:** Nemotron-3-Ultra-550B-A55B
- **Number of model parameters:** 550B Total / 55B Active

This model inherits its architecture unchanged from Nemotron-3-Ultra. See the [Nemotron-3-Ultra Technical Report](https://research.nvidia.com/labs/nemotron/files/NVIDIA-Nemotron-3-Ultra-Technical-Report.pdf) for architecture details.

## Input

Input Type(s): Text

Input Format(s): String

Input Parameters: One-Dimensional (1D)

Other Properties Related to Input: Maximum context length up to 262,144 tokens

## Output

Output Type(s): Text

Output Format: String

Output Parameters: One-Dimensional (1D)

Other Properties Related to Output: Maximum context length up to 262,144 tokens

This model is optimized for NVIDIA GPU-accelerated systems. Its deployment uses NVIDIA GPUs and CUDA-based software libraries to accelerate inference.

## Training Methodology

Nemotron-Labs-3-Competitive-Coding is initialized from the RLVR-teacher checkpoint of Nemotron-3-Ultra-550B-A55B and fine-tuned with supervised fine-tuning (SFT) only; no additional reinforcement learning or distillation stage was applied to this checkpoint.

**Supervised Fine-Tuning:**

* Teacher model: GLM-5.2
* Training data: 477,642 synthetic reasoning traces distilled from GLM-5.2 across a curated corpus of 22,000 competitive-programming problems from 16 regional and international contest families, with more generations allocated to harder problems and self-improvement traces where the teacher refines a previously generated solution.
* Epochs: 1
* Optimizer: AdamW, global batch size 64, peak learning rate 1.5 × 10⁻⁵, cosine schedule with 0.1 warmup ratio
* Maximum packed sequence length: 262,144 tokens
* Parallelism: Tensor parallel 8
* Hardware: 128 × NVIDIA GB300-288GB GPUs

**Test-time compute (GenCorrect):** At inference, Nemotron-Labs-3-Competitive-Coding is paired with GenCorrect, an iterative closed-loop test-time compute strategy that generates diverse candidate solutions, selects a representative subset via token-shingle diversity clustering, submits them for evaluator feedback, and conditions subsequent rounds on accumulated per-subtask scores and complementary reference solutions.

**Quantization:** Post-training quantized to NVFP4 using the NVIDIA Model Optimizer NVFP4 recipe, calibrated on 1,000 sequences of 32,768 tokens sampled from the SFT mixture, for increased inference throughput during live competition deployment.

More details on data curation, training configuration, and the GenCorrect algorithm can be found in the source paper: [Post-Training Language Models for Gold-Medal Performance in Coding Competitions (arXiv:2609.02849)](https://arxiv.org/abs/2609.02849).

## Training, Testing, and Evaluation Datasets

### Training Dataset

Data Modality: Text

Text Training Data Size: 477,642 samples across 22,000 problems.

Data Collection Method by dataset: Hybrid — curated contest problems and synthetically generated reasoning traces.

Labeling Method by dataset: Synthetic — generated by GLM-5.2.

Properties (Quantity, Dataset Descriptions, Sensor(s)):
Competitive-programming problems from 16 regional and international contest families, paired with synthetic reasoning traces and code solutions. The mixture includes self-improvement traces in which the teacher refines previous solutions, with more generations allocated to harder problems.

The dataset contains primarily English-language problem statements and reasoning, alongside programming-language source code.

### Testing Dataset

Data Collection Method by dataset: Hybrid — curated contest problems and automated benchmark processing.

Labeling Method by dataset: Hybrid — contest-provided test cases and scoring criteria, with automated solution evaluation.

Properties (Quantity, Dataset Descriptions, Sensor(s)):
Competitive-programming benchmarks for assessing code-generation quality. IOI 2025 was used as a development benchmark.

### Evaluation Dataset

Benchmark Score: See the Benchmarks section for results on IOI 2025, ICPC 2025, LiveCodeBench Pro, and IOI 2026.

Data Collection Method by dataset: Hybrid — curated contest problems and automated benchmark processing.

Labeling Method by dataset: Hybrid — contest-provided test cases and scoring criteria, with automated solution evaluation.

Properties (Quantity, Dataset Descriptions, Sensor(s)):
Competitive-programming benchmarks assessing solution correctness and algorithmic efficiency. IOI 2025 also served as a development benchmark. IOI 2026 was evaluated in a single prospective live run under official contest constraints, outside the official rankings.

## Software Integration

Runtime Engine(s): vLLM

Supported Hardware Microarchitecture Compatibility:

- NVIDIA Blackwell

Supported Operating System(s): Linux

Before integrating this model into an AI system, developers should evaluate it using data representative of the intended use case. Apply the V-model methodology with iterative unit and system testing to validate technical and functional requirements, address deployment risks, and assess safety and ethical requirements.

## Model Version(s)

* v1.0 - GA (2026-09-03)

To integrate this model into an AI system, deploy it with vLLM using the configuration in [Deployment (NVFP4)](#deployment-nvfp4), then submit text prompts through the interface shown in [API Client](#api-client).

### Deployment (NVFP4)

The live IOI 2026 competition run used NVFP4 post-training quantization for inference throughput. The evaluated configuration used an FP8 KV cache, prefix caching disabled, and MTP set to 5, achieving 736.8 tokens/s/GPU at 52.8% IOI 2025 Score@1 (vs. 199.1 tokens/s/GPU at 59.4% for the unquantized BF16 baseline). This trade-off was chosen to enable the large candidate batches required by GenCorrect within the competition time window.

**Recommended container:** `vllm/vllm-openai:v0.22.0`

```shell
export MODEL_CKPT=PATH/TO/NVFP4/CHECKPOINT
```

**Example single-node NVFP4 competition deployment (4×GB300, TP4):**

```shell
docker run -d --name nemotron-labs-3-competitive-coding-vllm \
  --gpus all \
  --ipc=host \
  --network=host \
  --shm-size=16g \
  --ulimit memlock=-1 \
  --ulimit stack=67108864 \
  -v $MODEL_CKPT:/model:ro \
  -e VLLM_WORKER_MULTIPROC_METHOD=spawn \
  -e SAFETENSORS_FAST_GPU=1 \
  -e NVIDIA_TF32_OVERRIDE=1 \
  -e VLLM_USE_FLASHINFER_MOE_FP8=1 \
  -e VLLM_USE_FLASHINFER_MOE_FP4=1 \
  -e VLLM_FLASHINFER_ALLREDUCE_BACKEND=trtllm \
  -e VLLM_DISABLED_KERNELS=FlashInferFP8ScaledMMLinearKernel \
  -e VLLM_FLASHINFER_MOE_BACKEND=throughput \
  -e VLLM_LOGGING_LEVEL=INFO \
  vllm/vllm-openai:v0.22.0 \
  /model \
  --host 0.0.0.0 \
  --port 8000 \
  --served-model-name nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4 \
  --trust-remote-code \
  --tensor-parallel-size 4 \
  --distributed-executor-backend mp \
  --dtype auto \
  --kv-cache-dtype fp8 \
  --block-size 64 \
  --no-enable-flashinfer-autotune \
  --max-model-len 262144 \
  --gpu-memory-utilization 0.90 \
  --max-num-seqs 32 \
  --max-num-batched-tokens 32768 \
  --enable-chunked-prefill \
  --no-enable-prefix-caching \
  --reasoning-parser nemotron_v3 \
  --mamba-ssm-cache-dtype float32 \
  --mamba-backend flashinfer \
  --enable-expert-parallel \
  --speculative-config '{"method":"nemotron_h_mtp","num_speculative_tokens":5,"max_model_len":262144}' \
  --model-loader-extra-config '{"enable_multithread_load":true,"num_threads":96}'
```


### API Client

```py
from openai import OpenAI
client = OpenAI(base_url="http://localhost:8000/v1", api_key="EMPTY")
MODEL = "nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4"

response = client.chat.completions.create(
    model=MODEL,
    messages=[{"role": "user", "content": PROBLEM_STATEMENT_PROMPT}],
    max_tokens=32000,
    temperature=1.0,
    top_p=0.95,
)
print(response.choices[0].message.content)
```

## Inference

Acceleration Engine: vLLM

Test Hardware (GPU Architecture, Model):
- NVIDIA Blackwell - GB300

## Ethical Considerations

NVIDIA believes Trustworthy AI is a shared responsibility and we have established policies and practices to enable development for a wide array of AI applications. When downloaded or used in accordance with our terms of service, developers should work with their internal model team to ensure this model meets requirements for the relevant industry and use case and addresses unforeseen product misuse.

We advise against circumvention of any provided safety guardrails contained in the Model without a substantially similar guardrail appropriate for your use case. For more details: [Safety](https://huggingface.co/nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4/blob/main/model-cards/Safety.md) and [Explainability](https://huggingface.co/nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4/blob/main/model-cards/Explainability.md) Subcards.

For more detailed information on ethical considerations for this model, please see the Model Card++ [Bias](https://huggingface.co/nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4/blob/main/model-cards/Bias.md), and [Privacy](https://huggingface.co/nvidia/NVIDIA-Nemotron-Labs-3-Competitive-Coding-550B-A55B-NVFP4/blob/main/model-cards/Privacy.md) Subcards.

Please report model quality, risk, security vulnerabilities or NVIDIA AI Concerns [here](https://www.nvidia.com/en-us/support/submit-security-vulnerability/).

## Citation

If you find this model or the accompanying pipeline useful, please cite:

```bibtex
@article{ficek2026posttraining,
  title={Post-Training Language Models for Gold-Medal Performance in Coding Competitions},
  author={Ficek, Aleksander and Narenthiran, Sean and Samadi, Mehrzad and Majumdar, Somshubra and Ginsburg, Boris},
  journal={arXiv preprint arXiv:2609.02849},
  year={2026}
}
```
