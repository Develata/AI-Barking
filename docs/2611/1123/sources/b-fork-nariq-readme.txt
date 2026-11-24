# NariQ: packaging, conversion and checks

Maintainer notes for this fork. Users start at the [top-level README](../README.md). `TAG` names the release
(`b12431-nariv0`). Releases, the container image and the model files are published by hand: no workflow creates a
release, and the image workflow pushes only when dispatched with `push=true`.

## Build and test

```sh
cmake -S . -B build-cpu -DCMAKE_BUILD_TYPE=Release -DGGML_CUDA=OFF -DGGML_METAL=OFF
cmake --build build-cpu -j 4
python3 -m pip install -e ./gguf-py
python3 -m unittest discover -s nariq/tests -v
ctest --test-dir build-cpu --output-on-failure \
  -R 'test-(trellis|trellis-metadata|gguf|quantize-fns|chat|grammar-parser|sampling)$'
```

CUDA builds add `-DGGML_CUDA=ON` (compute capability 8.0+; `-DCMAKE_CUDA_ARCHITECTURES` narrows the set). libtrellis is
vendored under `ggml/src/ggml-cuda/libtrellis`. Metal: [README-metal.md](README-metal.md).

## Convert a NariQ checkpoint

`convert_hf_to_gguf.py` detects `quantization_config.quant_method = native_trellis` and hands off to
`conversion/trellis.py`. The input must already hold NariQ codes and scales; the converter does not quantize BF16
weights.

```sh
python convert_hf_to_gguf.py /path/to/native-trellis-checkpoint --outfile model.gguf --outtype bf16 --no-mtp
```

`bf16` applies to the uncoded tensors; coded tensors keep their widths. The vision projector is the base model's BF16
projector. Format details: [FORMAT.md](FORMAT.md).

## Release archives

```sh
bash nariq/build_linux.sh nariq-dist                     # CUDA 12.8 toolchain, CMake 3.28+, Ninja, Python 3
docker buildx build -f nariq/Dockerfile --target artifacts \
  --build-arg NARIQ_SOURCE_REVISION="$(git rev-parse HEAD)" --output type=local,dest=nariq-dist .   # pinned container
```

Each run gives a program archive (with `build-info.json`: revision, source hashes, build settings) and a CUDA runtime
archive, plus `SHA256SUMS`. `CUDA_ARCHS` narrows a development build; never release one. The build fetches the chat
page pinned in `ui.json` and fails if its hash does not match.

Windows archives come from the `NariQ Windows artifacts` workflow (MSVC and CUDA 12.8, CPU tests only; hosted runners
have no GPU).

To stage a release (nothing is uploaded):

```sh
bash nariq/stage_release.sh linux-artifacts [windows-artifacts]   # -> nariq-release-stage/TAG
```

`stage_release.sh` runs `assemble_page.py` (also `assemble.sh page`), which checks that each program/runtime pair comes from one clean revision whose
source hashes match, and writes one `SHA256SUMS` over the archives, `run.sh` and `run.ps1`. It adds the release page
text from [RELEASE_NOTES.md](RELEASE_NOTES.md). For local tests, `TRELLIS_RELEASE_URL` and `TRELLIS_MODEL_URL` point
the launchers at a local server.

`run.sh` (Linux x86-64) and `run.ps1` (Windows) pin the four model files by sha256, fit the context to free GPU memory
and cap CPU threads in small containers, since coded embeddings run on the CPU.

## CI

`NariQ CPU and Metal checks` runs on pushes and pull requests (Linux and macOS CPU, Metal shader build). The Linux and
Windows CUDA workflows run on relevant pull requests or by hand and only store artifacts. The upstream publishing
workflows are removed, so no push can publish an upstream-style release.

Release gates and results: [cuda-qualification.md](cuda-qualification.md).
