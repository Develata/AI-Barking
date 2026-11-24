# Build and validate NariQ on gfx1100

Use a Linux ROCm 7.2.4 development environment with access to the AMD
GPU. In a container, pass through /dev/kfd and the appropriate /dev/dri
render device. Build from the patched NariQ tree.

```sh
CC=/opt/rocm/bin/amdclang CXX=/opt/rocm/bin/amdclang++ \
cmake -G Ninja -S . -B build-hip -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_HIP_COMPILER=/opt/rocm/lib/llvm/bin/clang \
  -DGGML_HIP=ON -DAMDGPU_TARGETS=gfx1100 \
  -DLLAMA_BUILD_TESTS=ON -DLLAMA_USE_PREBUILT_UI=OFF
cmake --build build-hip --parallel 6 --target llama-server test-trellis test-trellis-model
```

Obtain Qwen3.8-27B-NariQ-3.0bpw.gguf from the public Nari Labs model card
and record its SHA-256. Start the text-only server.

```sh
MODEL=/path/to/Qwen3.8-27B-NariQ-3.0bpw.gguf
build-hip/bin/llama-server -m "$MODEL" -ngl 99 --fit off \
  --host 127.0.0.1 --port 18281 -c 262144 -np 1 -fa on \
  -ctk q8_0 -ctv q8_0 -b 1024 -ub 1024
```

For the other cache runs, restart with q8_0/q4_0 and q4_0/q4_0. Submit
identical token IDs via /completion with cache_prompt=false, temperature=0
and n_predict=16. Record the actual prompt and generated token counts,
timings, and VRAM after completion. The companion benchmark script generates
the same prompt and prints those measurements.

To run the synthetic tensor test, use `build-hip/bin/test-trellis -v ROCm0`.

## Numerical parity procedure

Use the same coded weights for the comparison. Build the F32 reference with
nariq/decode_reference.py, passing the ggml-base shared library from this fork
and --outtype f32. Allow disk space for roughly 110 GB of reference weights
and use CPU inference for the F32 reference.

```sh
PROBE=build-hip/bin/test-trellis-model
"$PROBE" --model /path/to/reference-f32.gguf --output ref \
  --gpu-layers 0 --ubatch 8 --predict 8
for ubatch in 8 1; do
  "$PROBE" --model "$MODEL" --output "hip-$ubatch" --force ref.json \
    --gpu-layers 99 --ubatch "$ubatch" --predict 8
  python3 nariq/compare_logits.py ref "hip-$ubatch" \
    --max-kl 0.001 --min-top1 0.98 --output "hip-$ubatch-report.json"
done
```

This procedure uses the upstream comparison tool with max KL capped at 0.001. The larger
CUDA qualification samples 510 positions. Record mean KL, top-1
agreement, and greedy-token equality. Keep CUDA regression qualification
separate and validate it on CUDA hardware using the patched tree.

Upstream qualification record
https://github.com/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/cuda-qualification.md

## Numerical checks on the RX 7900 XTX

The initial HIP bulk path converted transformed activations to FP16 before GEMM.
In the synthetic TRL1 test, the first reported mismatch at a batch of 35 was
-10.3191 on HIP versus -10.3261 on CPU. Keeping both GEMM inputs in FP32
makes the default path pass TRL1–6 with the original tolerance. The documented
CTest subset passed 7/7. The Python suite passed 20 tests and skipped two.

The tests below use the CPU backend with the same original GGUF and the same
forced continuation. Each comparison covers 55 positions. All runs matched
every top-1 token and the entire greedy continuation.

| Weight format | HIP microbatch | Mean KL | Max KL |
| --- | ---: | ---: | ---: |
| 3.0 bpw | 1 | 6.51e-9 | 5.99e-8 |
| 3.0 bpw | 32 | 4.21e-6 | 4.82e-5 |
| 2.5 bpw | 1 | 4.46e-9 | 4.56e-8 |
| 2.5 bpw | 32 | 5.32e-6 | 5.74e-5 |
| 2.0 bpw | 1 | 2.08e-8 | 5.82e-7 |
| 2.0 bpw | 32 | 1.25e-5 | 3.80e-4 |

## Performance on the RX 7900 XTX

All measurements use 3.0 bpw, ROCm 7.2.4, Q8_0/Q8_0 KV, and batch and
microbatch sizes of 1024.

| Measurement | Initial HIP code | FP32 GEMM |
| --- | ---: | ---: |
| llama-bench pp2048 (2 runs) | 218.97 tok/s | 352.55 tok/s |
| llama-bench tg32 (2 runs) | 6.19 tok/s | 6.32 tok/s |
| Server prefill through 65,536 tokens | 205.26 tok/s | 312.09 tok/s |
| Server prefill, 261,632 uncached tokens | 165.82 tok/s | 228.99 tok/s |
| Server decode after full prefill (5 tokens) | 5.18 tok/s | 4.65 tok/s |
| VRAM after request | 20.35 GiB | 20.51 GiB |

On this host the short prefill improved 61%, and the long prefill improved
38%, saving roughly 7 minutes 15 seconds. The GPU still does more attention
work per token late in a long prompt. The five-token decode samples are
too short to establish a decode regression.

## CUDA build check

The patched tree built and linked `ggml-cuda` with the bundled libtrellis
using CUDA 12.4, GCC 13, and `sm_86`. Preprocessing the two modified CUDA
translation units with the same flags showed no change to the CUDA Trellis
implementation. In `ggml-cuda.cu`, the only differences were shifted source
line numbers in diagnostics. A CUDA runtime regression check still needs an
NVIDIA GPU.

## Independent decoded-F32 comparison

The upstream converter materialized F32 weights from each original coded
GGUF. A CPU-only reference harness evaluated the prompt and forced
continuation in one batch with mmap prefetch disabled to limit memory and
disk pressure. The HIP side used the original GGUF and microbatches of
1 and 32. Each comparison used 55 full-vocabulary logit rows. All greedy
continuations matched. The comparator required max KL <= 0.001 and
top-1 >= 98%.

| Weight (bpw) | HIP microbatch | Mean KL | Max KL | Top-1 |
| --- | ---: | ---: | ---: | ---: |
| 3.0 | 1 | 5.85e-09 | 3.8e-08 | 55/55 |
| 3.0 | 32 | 4.21e-06 | 4.87e-05 | 55/55 |
| 2.5 | 1 | 4.41e-09 | 4.39e-08 | 55/55 |
| 2.5 | 32 | 5.33e-06 | 5.74e-05 | 55/55 |
| 2.0 | 1 | 2.82e-08 | 8.54e-07 | 55/55 |
| 2.0 | 32 | 1.25e-05 | 0.000379 | 55/55 |

This test covers a shorter sample than the upstream CUDA qualification
(55 versus 510 positions).
