# Metal (experimental)

NariQ coded linear layers (`GGML_OP_TRELLIS`) run on Apple GPUs with the weights kept compressed, for decode and
prefill, including banded output heads. Coded embedding lookups run on the CPU. There is no macOS package; build from
source.

## Mac quick start

From the source checkout on Apple Silicon, with CMake, Python 3 and the Xcode command line tools installed:

```sh
bash nariq/run-mac.sh 2.0
```

The launcher checks the release checksum, prepares the pinned Web UI and builds the Metal server in `build-metal`. It checks both the current and older model filenames under `~/.cache/nari-llama.cpp/models` (`TRELLIS_HOME` overrides the cache root). If the selected weights are missing, it uses the Hugging Face CLI (`hf`) to download them. Use `hf auth login` if the repository requires access.

Open [local chat](http://127.0.0.1:8080) after the server is ready. For the 2.0 budget, use model ID `Qwen3.8-27B-NariQ-2.0bpw` at the [API base](http://127.0.0.1:8080/v1). Stop the foreground server with Ctrl-C.

Use `2.5` or `3.0` for the other released weights, `--model /path/to/model.gguf` to point to a verified copy of the selected budget, or `--download-only` to verify or fetch the weights without building. The launcher asks llama-server to fit the context to available memory. Pass server options after `--`, for example `bash nariq/run-mac.sh 2.0 -- -c 8192 --port 8081`.

## Manual build

The coded products need Apple GPU family 7 or newer. Other devices and unsupported layouts fall back to the CPU.

```sh
python3 nariq/prepare_ui.py
cmake -S . -B build-metal -DCMAKE_BUILD_TYPE=Release \
  -DGGML_METAL=ON -DGGML_CUDA=OFF -DLLAMA_BUILD_TESTS=ON
cmake --build build-metal --target llama-server test-trellis -j 4
./build-metal/bin/llama-server -m /path/to/model.gguf -ngl 99 -c 8192 --jinja --port 8080
```

Check that unified memory holds the model before loading it; use fewer GPU layers (`-ngl`) if it does not. Add `--mmproj /path/to/projector.gguf` for vision. The default build embeds the shader source and compiles it at start; `-DGGML_METAL_EMBED_LIBRARY=OFF` builds a `default.metallib` that must stay next to the executables.

## Checks

```sh
ctest --test-dir build-metal --output-on-failure -R trellis
MTL_SHADER_VALIDATION=1 ./build-metal/bin/test-trellis MTL0
GGML_METAL_SHARED_BUFFERS_DISABLE=1 ./build-metal/bin/test-trellis MTL0
```

`test-trellis MTL0` compares Metal results with independently decoded CPU products (tolerance
`2e-4 * (1 + |reference|)`) across all bit widths, transforms, permutations, input shapes and mixed CPU/Metal graphs.
CI compiles the shaders on hosted macOS runners, which have no usable GPU: a build check only.

To check full-model startup and decode before a release, dispatch `NariQ CPU and Metal checks` with `metal_gpu` enabled and `metal_model` set to the local path of the released 2.0bpw GGUF on the self-hosted Metal runner. The check verifies the model checksum, fits it to GPU memory and uses `llama-bench` to run prompt processing and decode on Metal.

## Tested configuration and limits

Only one machine has run this: an M3 Pro with 18 GB of unified memory, macOS 14.6, Xcode 16.2.

- The device test above passes, with shader validation and with private buffers.
- For all three budgets, short full-model captures on CPU and Metal agree: identical greedy choices at every captured position, maximum full-vocabulary KL below 1e-8.
- Decode runs at about 3.8 (3.0bpw) to 4.9 (2.0bpw) tokens/s (prompt about 27 tokens/s), well below the 15-22
  tokens/s that the M3 Pro's 150 GB/s memory bandwidth allows for these files; too slow for long agent sessions.
- Not qualified: vision, long context, concurrent requests and other Macs.
