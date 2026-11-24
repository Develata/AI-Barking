# CUDA qualification: b12431-nariv0

Engineering checks on Modal of the release binaries, built from `9fc1bfa` (`30f794c` in `build-info.json`: the same
tree before a commit-message rewording), with the launchers' pinned model files. The GPU checks ran on the same
binaries under the release's earlier name (`b12431-nari2`). The two `bin` archives were later repacked with new
license files only (MIT for the NariQ additions, exllamav3 credit); every binary and `build-info.json` is unchanged.

## Build

- Linux: `git archive` of `30f794c` built as in `nariq/Dockerfile` on a Modal CPU container
  (`nvidia/cuda:12.8.2-devel-ubuntu22.04@sha256:b212f940…`, `cmake==3.31.6`; CUDA 12.8.93, GCC 11.4.0; native code for
  8.6, 8.9, 9.0 and 12.0a, PTX for 5.0-9.0). Its 3,531 source-file hashes in `build-info.json` equal the commit's files; the other 70 entries are the pinned chat page.
- Windows: the `NariQ Windows artifacts` workflow at `30f794c`
  ([run 44435062044](https://github.com/nari-labs/nari-llama.cpp/actions/runs/44435062044)), MSVC 2022 with CUDA 12.8;
  CPU-checked on the runner, never run on a Windows GPU.

| File | Bytes | SHA256 |
| --- | ---: | --- |
| `nari-llama.cpp-b12431-nariv0-bin-linux-cuda-12.8-x64.tar.gz` | 257,432,658 | `1bf65e866d46323594ffacaf0de95120677e954cdd80a91a4f1b9c91a7e5836a` |
| `cudart-nari-llama.cpp-b12431-nariv0-linux-cuda-12.8-x64.tar.gz` | 594,266,099 | `ec215628695711b8b80e232403165f0eb1d140cf7a28d804fe085b1eae5cddcc` |
| `nari-llama.cpp-b12431-nariv0-bin-win-cuda-12.8-x64.zip` | 224,585,591 | `9e52858b7f067c635da54a42981affbf237abeefec93602c2b825fb7136001c1` |
| `cudart-nari-llama.cpp-b12431-nariv0-win-cuda-12.8-x64.zip` | 563,452,046 | `fe20366827d357c00797eebb58244dddab7fd9a348d70090c3871004c320f38d` |
| `run.sh` | 7,917 | `9eebff519f522f07cccc06fa8c780ca28b449e5478e489b7aa0cefd5f69dbc30` |
| `run.ps1` | 6,676 | `fce57fe42d2e3361ed18ce4c4e11f7339d40be25e2655d17d8b75915fe60a8d8` |

## Results

| Check | GPUs | What it covers | Result |
| --- | --- | --- | --- |
| F32 runtime parity (`20261119T012811Z`) | B200 | Coded prefill (`-ub 512`) and decode (`-ub 1`) against an F32 decode of the same coded GGUF, 510 positions; gate: mean KL <= 0.001, top-1 >= 98% | Pass, all three files |
| Token parity (`fork-qual-20261119T012742Z`) | RTX PRO 6000, H100, L4 | Greedy tokens against the previous build, all three files, 4K-64K input, 51 requests; decode speed | 51/51 identical; speed within 0.5% |
| Serve matrix (`serve-matrix-20261119T012915Z`) | RTX PRO 6000, H100, L4, A10G | All files and the projector via `run.sh` and the model-card command: chat, thinking, tool calls, images, a needle at up to 248,694 tokens, eight sessions | Pass on every GPU and file |
| Launch rehearsal (`rehearsal-20261119T033256Z`) | RTX PRO 6000 | The staged release in plain `ubuntu:22.04`: `bash run.sh 3.0 / 2.5 / 2.0`, no flags, every download hashed | Pass |
| Container image (`image-check-20261119T023705Z`) | RTX PRO 6000 | The release image workflow's image: entry point, health check, serving checks, a 32,583-token needle. The published image was then built by the same workflow from the release's own archives ([run 44543313079](https://github.com/nari-labs/nari-llama.cpp/actions/runs/44543313079)) | Pass |

F32 runtime parity (mean KL / top-1):

| File | Coded prefill | Coded decode |
| --- | ---: | ---: |
| 3.0bpw (`1eaad934…`) | 0.000168 / 100.0% | 0.000056 / 100.0% |
| 2.5bpw (`2445372e…`) | 0.000122 / 99.8% | 0.000101 / 99.8% |
| 2.0bpw (`3a9e4955…`) | 0.000237 / 99.6% | 0.000607 / 99.4% |

The nine logit files equal the previous build's byte for byte. References come from `nariq/decode_reference.py`;
this is a same-weight runtime check on a short sample, not a quality measurement.

Decode on the RTX PRO 6000, one session, tokens/s (512 greedy tokens, median of three, BF16 KV; previous build in parentheses):

| File | 4,096 | 16,384 | 65,536 input tokens |
| --- | ---: | ---: | ---: |
| 3.0bpw | 89.50 (89.29) | 85.21 (85.01) | 71.78 (71.61) |
| 2.5bpw | 99.98 (99.86) | 94.72 (94.50) | 78.35 (78.27) |
| 2.0bpw | 112.91 (112.72) | 106.23 (105.67) | 85.64 (85.81) |

The model card and release notes quote a separate measurement on the same GPU type (88.9 / 84.5 / 70.7,
99.2 / 93.8 / 77.5, 111.7 / 105.1 / 85.0), within 1.5% of this one.

## BF16 references

A BF16 decode of the same coded weights misses the KL limit, so it is not the gate: coded prefill against it gives
mean KL 0.0016 / 0.0044 / 0.0216 (3.0 / 2.5 / 2.0), while the BF16 reference itself differs between prefill and decode
by 0.0014 / 0.0016 / 0.0076. That is BF16 rounding, not the coded kernels, which the F32 check isolates.

## Not covered

- No Windows GPU has run the Windows archives; they are experimental.
- No GeForce card has been tested.
- Driver minimums (Linux 570, Windows 572) are documented, not tested; the checks used drivers 610 and 620.
- Metal is covered in [README-metal.md](README-metal.md).
