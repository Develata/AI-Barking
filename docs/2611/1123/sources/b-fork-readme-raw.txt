# nari-llama.cpp

The Nari Labs fork of [llama.cpp](https://github.com/ggml-org/llama.cpp) that runs the NariQ trellis-coded
[Qwen3.8-27B GGUFs](https://huggingface.co/nari-labs/Qwen3.8-27B-NariQ-GGUF) at 3.0, 2.5 and 2.0 bits per weight.
Stock llama.cpp, Ollama and LM Studio cannot load these files yet. Based on upstream b12431 (`a946eb5`).

(Nov 20, 2026) First release, [`b12431-nariv0`](https://github.com/nari-labs/nari-llama.cpp/releases/tag/b12431-nariv0):
NariQ v1 GGUFs of Qwen3.8-27B at 3.0, 2.5 and 2.0 bits per weight.

## Performance

| Qwen3.8-27B | Size | DeepSWE solved | DeepSWE score | Terminal-Bench 2.1 | AIME 25+26 | RealWorldQA | MGSM |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| BF16 | 53.8 GB | 44 | 83.0 | 62.6 | 97.7 | 82.4 | 94.0 |
| NariQ 3.0bpw | 10.1 GB | 47 | 84.9 | 63.8 | 98.3 | 82.4 | 93.1 |
| NariQ 2.5bpw | 8.4 GB | 46 | 84.5 | 60.2 | 96.5 | 81.3 | 91.8 |
| NariQ 2.0bpw | 6.8 GB | 33 | 75.7 | 52.4 | 86.0 | 80.9 | 90.6 |

DeepSWE v1.1: tasks fully solved of 113, and the mean share of each task's new tests that pass (%), one attempt per
task. Terminal-Bench 2.1: mean test reward over 82 tasks × 3 attempts (%). Both use Claude Code driving the model.
AIME, RealWorldQA and MGSM: % correct. Details are in the [technical report](https://arxiv.org/abs/2611.26157) and on the
[model card](https://huggingface.co/nari-labs/Qwen3.8-27B-NariQ-GGUF).

Decode on one RTX PRO 6000, one session, at 4K / 64K tokens of context: 88.9 / 70.7 (3.0bpw), 99.2 / 77.5 (2.5bpw) and
111.7 / 85.0 (2.0bpw) tokens/s.

## Quick start (Linux x86-64, NVIDIA GPU)

```sh
curl -fsSLO https://github.com/nari-labs/nari-llama.cpp/releases/download/b12431-nariv0/run.sh
bash run.sh 3.0   # or 2.5, 2.0
```

`run.sh` downloads the binaries and model once, checks every sha256, fits the context to free GPU memory and serves a
chat page and an OpenAI-compatible API at http://127.0.0.1:8080. Options: `--port N`, `--no-vision`, `--docker`,
`--download-only`; arguments after `--` go to `llama-server` (`bash run.sh --help`). Files go to
`~/.cache/nari-llama.cpp`, about 13 GB with the 3.0bpw file. With the `hf` CLI installed (`pip install -U
huggingface_hub`), the model files download through it, in parallel and often much faster than one curl stream, and
are still checked against their sha256; `TRELLIS_NO_HF=1` turns this off.

It needs an NVIDIA driver of 570 or newer, glibc 2.34 or newer, libstdc++ with `GLIBCXX_3.4.30` and libgomp (Ubuntu
22.04 or newer, Debian 12 or newer). On RHEL 9, Rocky 9 or Amazon Linux 2023, use `bash run.sh 3.0 --docker`.

A 24 GB GPU fits about 135K / 160K / 184K tokens of context with the 3.0 / 2.5 / 2.0bpw file and the vision projector;
the full 262K context needs about 30 / 28 / 27 GB.

### Windows (experimental)

Download `run.ps1` from the [release](https://github.com/nari-labs/nari-llama.cpp/releases/tag/b12431-nariv0) and
run it in PowerShell (NVIDIA driver 572 or newer):

```powershell
powershell -ExecutionPolicy Bypass -File run.ps1 -Budget 3.0   # or 2.5, 2.0
```

It downloads the native Windows build and the model, checks every sha256 and serves the chat page and API at
http://127.0.0.1:8080. Manual setup: [README-windows.md](nariq/README-windows.md).

On Windows, close to 1.5 GB of GPU memory can go to the desktop, which shortens the fitted context. For example, an
RTX 5060 Ti 16 GB fits about 47K tokens of context with the 2.5bpw file and the vision projector.

The Windows binaries are not code-signed yet, so Windows 11 blocks them when Smart App Control is on. To run them, turn
it off in Windows Security > App & browser control > Smart App Control settings (on many Windows 11 versions it cannot
be turned back on without resetting Windows), or use the WSL2 workaround below.

<details>
<summary>Workaround: keep Smart App Control on and run the Linux build in WSL2</summary>

Set up a distribution once with `wsl --install` (Ubuntu 22.04 or newer), open it, and run the Linux quick start:

```sh
curl -fsSLO https://github.com/nari-labs/nari-llama.cpp/releases/download/b12431-nariv0/run.sh
bash run.sh 3.0   # or 2.5, 2.0
```

Files go to `~/.cache/nari-llama.cpp` inside WSL, and the server is reachable from Windows at http://127.0.0.1:8080.
In one test it decoded about 10% slower than the native build. From the next release, `run.ps1 -Wsl` does this from
PowerShell.

</details>

### Manual setup

```sh
mkdir nari-llama.cpp
tar -xzf nari-llama.cpp-b12431-nariv0-bin-linux-cuda-12.8-x64.tar.gz --strip-components=1 -C nari-llama.cpp
tar -xzf cudart-nari-llama.cpp-b12431-nariv0-linux-cuda-12.8-x64.tar.gz --strip-components=1 -C nari-llama.cpp
hf download nari-labs/Qwen3.8-27B-NariQ-GGUF \
    Qwen3.8-27B-NariQ-3.0bpw.gguf Qwen3.8-27B-NariQ-mmproj-BF16.gguf --local-dir .   # pip install -U huggingface_hub
./nari-llama.cpp/llama-server -m Qwen3.8-27B-NariQ-3.0bpw.gguf --mmproj Qwen3.8-27B-NariQ-mmproj-BF16.gguf \
    --fit-target 2560 --port 8080 --temp 1.0 --top-p 0.95 --top-k 20 --min-p 0
```

The archives are on the [release page](https://github.com/nari-labs/nari-llama.cpp/releases/tag/b12431-nariv0).
Without `-c`, the context is fitted to free GPU memory (`--fit-target 2560` keeps room for the vision projector); add
`-c N` to cap it. Keep `--port 8080`: this version otherwise listens on 9931. The sampling flags are the thinking-mode
settings the scores used.

### Docker

With the NVIDIA Container Toolkit, from the folder that holds the GGUFs:

```sh
docker run --rm --gpus all -p 127.0.0.1:8080:8080 -v "$PWD:/models:ro" ghcr.io/nari-labs/nari-llama.cpp:b12431-nariv0 \
    -m /models/Qwen3.8-27B-NariQ-3.0bpw.gguf --mmproj /models/Qwen3.8-27B-NariQ-mmproj-BF16.gguf \
    --fit-target 2560 --min-p 0 --host 0.0.0.0 --port 8080
```

## Hardware

| Hardware | Status |
| --- | --- |
| NVIDIA, compute capability 8.0+, Linux | Tested on RTX PRO 6000, H100, L4 and A10G. Native code for 8.6, 8.9, 9.0 and 12.0; other cards compile PTX on first start (about 50 s). |
| NVIDIA, Windows | Experimental. Run on a GeForce RTX 5060 Ti (Windows 11), not qualified yet. Blocked by Smart App Control when it is on (not code-signed yet). |
| NVIDIA GeForce | One card run so far, not qualified: RTX 5060 Ti 16 GB, 2.5bpw with the vision projector, about 38 tokens/s decode on Windows (512 tokens, short prompt). |
| NVIDIA, Linux ARM64 (GH200, DGX Spark) | No release binaries; build from source, untested. |
| Apple Silicon | Experimental, build from source, about 3.8 (3.0bpw) to 4.9 (2.0bpw) tokens/s on an M3 Pro ([Metal notes](nariq/README-metal.md)). |
| CPU | Every coded operation runs on the CPU, slowly for a 27B model. |
| AMD, Intel GPUs | No NariQ kernels. |

Kernel tuning has focused on the RTX PRO 6000; on other GPUs NariQ files are slower than comparable stock GGUFs for
now. Coded embedding lookups run on the CPU. LoRA adapters on NariQ weights are rejected.

## Build from source

Needs CMake 3.18 or newer, a C++17 compiler and, for CUDA, the CUDA toolkit (the releases use 12.8).

On Apple Silicon, run `bash nariq/run-mac.sh 2.0` from the checkout to verify the weights, build the Metal server and start local chat. See the [Mac quick start](nariq/README-metal.md) for prerequisites and options.

```sh
git clone --branch b12431-nariv0 https://github.com/nari-labs/nari-llama.cpp && cd nari-llama.cpp
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DGGML_CUDA=ON    # or -DGGML_CUDA=OFF for CPU only
cmake --build build -j 4
./build/bin/llama-server -m model.gguf --mmproj projector.gguf --port 8080
```

## Contributing

We welcome open-source contributions that speed up NariQ inference on more hardware: faster kernels for GeForce,
Ampere, Ada and Hopper cards, Apple Silicon, AMD and Intel GPUs, and the CPU path. Report bugs or results on your
hardware in [Issues](https://github.com/nari-labs/nari-llama.cpp/issues), and open pull requests against
`nariq/b12431`; a change should keep the [CUDA qualification](nariq/cuda-qualification.md) parity checks passing and
report decode and prefill tokens/s before and after.

## More

- [Tensor format](nariq/FORMAT.md)
- [Packaging, conversion and checks](nariq/README.md)
- [CUDA qualification record](nariq/cuda-qualification.md)
- [Metal](nariq/README-metal.md)

General llama.cpp documentation is in [docs](docs/) and [upstream](https://github.com/ggml-org/llama.cpp), which does
not endorse this fork.

## Contact us

For technical consulting, compute sponsorships, or partnership inquiries, please contact founders@narilabs.com.

## License

llama.cpp and the NariQ additions are under the [MIT license](LICENSE). The vendored libtrellis keeps its
[Apache-2.0 license](ggml/src/ggml-cuda/libtrellis/LICENSE). The coded layout follows
[exllamav3](https://github.com/turboderp-org/exllamav3) (MIT); see [NOTICE](nariq/NOTICE).
