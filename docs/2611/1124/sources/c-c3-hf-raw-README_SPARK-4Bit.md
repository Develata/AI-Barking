# Suggestions for running Hy4-flash on NVIDIA Spark

[English](#english) · [中文](#中文)

## English

We recommend the **SPARK-4Bit GGUF** build of Hy4-flash for NVIDIA DGX Spark. Apply `hy4-flash-spark.patch` before building llama.cpp. This single patch includes Hy4-flash architecture support, our CUDA optimizations, and MTP support. Follow the build steps below, then use the runtime settings and server command provided here.

### Build

Place `hy4-flash-spark.patch` beside the directory you will clone. The patch applies to the pinned upstream commit below.

```bash
git clone https://github.com/ggml-org/llama.cpp.git llama.cpp-hy4-flash
cd llama.cpp-hy4-flash
git checkout 2cbca389f515df455412dafa07e2b1359cc72a6d

git apply --check ../hy4-flash-spark.patch
git apply ../hy4-flash-spark.patch

export PATH=/usr/local/cuda-13.0/bin:$PATH
export CUDACXX=/usr/local/cuda-13.0/bin/nvcc

cmake -S . -B build-cuda \
  -DGGML_CUDA=ON \
  -DGGML_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_CUDA_ARCHITECTURES=121a \
  -DLLAMA_OPENSSL=OFF \
  -DLLAMA_BUILD_UI=OFF \
  -DLLAMA_USE_PREBUILT_UI=OFF

cmake --build build-cuda --config Release \
  --target llama-server llama-bench --parallel 8
```

Use `121a` for Spark / GB10 (CC 12.1). Adjust the CUDA installation path for your system. Compile on the target host architecture. `LLAMA_OPENSSL=OFF` avoids the OpenSSL dependency for this local-model setup; enable it if HTTPS support is required.

### Optimizations

Our patch further optimizes Hy4-flash for faster inference on local and edge devices through SPARK-4Bit matrix multiplication tuning, gate/up and HC fusion, Q8 activation reuse, and concurrent MoE execution. Local A/B validation gives indicative gains of approximately **Prefill +30% / Decode +20%**, compared with the version containing only architecture support. Actual gains depend on hardware and workload and are not Spark-specific measurements.

### Runtime switches

The commands below use the recommended settings in this table. A switch takes effect only when its tensor types and shapes match the corresponding optimization.

| Environment variable | Setting | Meaning |
|---|---|---|
| `GGML_SCHED_BATCH_HOST_INPUTS` | **0 for Linux, 2 for Windows** | Batch host-input copies and synchronize once per group. `0`: off; `1`: external INPUT tensors; `2`: also include CPU-produced intermediates. |
| `GGML_CUDA_FUSE_HC_GATE` | `1` | Fuse HC gate operations to reduce kernel launches. |
| `GGML_CUDA_FUSE_HC_CONSUMER` | `1` | Extend HC gate fusion into compatible HC pre/post consumers. |
| `GGML_CUDA_REUSE_Q8` | `1` | Reuse quantized Q8 activations across compatible MMVQ operations. |
| `GGML_CUDA_SPARK_MMQ_FUSION` | `1` | Fuse compatible SPARK-4Bit expert gate/up matrix multiplications. |
| `GGML_CUDA_SPARK_MMQ_COMPACT_GRID` | `1` | Use compact grids for eligible SPARK-4Bit MMQ operations. |
| `GGML_CUDA_SPARK_MMQ_TILE` | `64` | Set the tile-width hint for SPARK-4Bit expert MMQ. |
| `GGML_CUDA_SPARK_MMQ_TILE_FUSED` | `64` | Set the tile-width hint for fused SPARK-4Bit expert MMQ. |
| `GGML_CUDA_MMQ_PREC` | `q4` | Request 4-bit activations for eligible SPARK-4Bit MMQ paths. |
| `GGML_CUDA_CPY_F32_VEC` | `1` | Vectorize eligible F32 layout copies. |
| `GGML_CUDA_SPARK_MMVQ_2W` | `1` | Use two warps for eligible single-token SPARK-4Bit expert MMVQ with K=1536. |
| `GGML_CUDA_MOE_CONCURRENT` | `1` | Run eligible shared and routed expert branches concurrently. |
| `GGML_CUDA_FA_576_WHOLE_TILES` | `0` | Keep the optional whole-tile Flash Attention path for head size 576 disabled. |

For `GGML_SCHED_BATCH_HOST_INPUTS`, we recommend starting with `0` on Linux and `2` on Windows. Performance may vary by device and workload.

### Start the server

Recommended server configuration: context 196608, one slot, batch/ubatch 2048/1024, four CPU threads, F16 KV cache, Flash Attention, MTP, PLE on CPU, and token embeddings on CUDA0. Replace the model and projector paths. For text-only serving, omit `--mmproj` and `--mmproj-offload`.

**Linux**

```bash
export CUDA_VISIBLE_DEVICES=0
export GGML_SCHED_BATCH_HOST_INPUTS=0
export GGML_CUDA_FUSE_HC_GATE=1
export GGML_CUDA_FUSE_HC_CONSUMER=1
export GGML_CUDA_REUSE_Q8=1
export GGML_CUDA_SPARK_MMQ_FUSION=1
export GGML_CUDA_SPARK_MMQ_COMPACT_GRID=1
export GGML_CUDA_SPARK_MMQ_TILE=64
export GGML_CUDA_SPARK_MMQ_TILE_FUSED=64
export GGML_CUDA_MMQ_PREC=q4
export GGML_CUDA_CPY_F32_VEC=1
export GGML_CUDA_SPARK_MMVQ_2W=1
export GGML_CUDA_MOE_CONCURRENT=1
export GGML_CUDA_FA_576_WHOLE_TILES=0

MODEL=/local-ssd/Hy4-flash-SPARK-4Bit.gguf
MMPROJ=/local-ssd/mmproj-Hy4-flash-bf16.gguf

./build-cuda/bin/llama-server \
  --model "$MODEL" \
  --mmproj "$MMPROJ" --mmproj-offload \
  --alias hy4-flash \
  --host 127.0.0.1 --port 8080 \
  --ctx-size 196608 --parallel 1 \
  --n-gpu-layers 999 --split-mode none \
  --override-tensor '^per_layer_token_embd\.weight$=CPU,^token_embd\.weight$=CUDA0' \
  --load-mode mmap --lazy-mode on --no-host \
  --fit off --flash-attn on \
  --batch-size 2048 --ubatch-size 1024 \
  --threads 4 --threads-batch 4 \
  --cache-type-k f16 --cache-type-v f16 \
  --spec-type draft-mtp --metrics --jinja
```

**Windows / PowerShell**

Use a Windows build of the same patched source; point `SERVER_BIN` to its executable.

```powershell
$env:CUDA_VISIBLE_DEVICES = '0'
$env:GGML_SCHED_BATCH_HOST_INPUTS = '2'
$env:GGML_CUDA_FUSE_HC_GATE = '1'
$env:GGML_CUDA_FUSE_HC_CONSUMER = '1'
$env:GGML_CUDA_REUSE_Q8 = '1'
$env:GGML_CUDA_SPARK_MMQ_FUSION = '1'
$env:GGML_CUDA_SPARK_MMQ_COMPACT_GRID = '1'
$env:GGML_CUDA_SPARK_MMQ_TILE = '64'
$env:GGML_CUDA_SPARK_MMQ_TILE_FUSED = '64'
$env:GGML_CUDA_MMQ_PREC = 'q4'
$env:GGML_CUDA_CPY_F32_VEC = '1'
$env:GGML_CUDA_SPARK_MMVQ_2W = '1'
$env:GGML_CUDA_MOE_CONCURRENT = '1'
$env:GGML_CUDA_FA_576_WHOLE_TILES = '0'

$SERVER_BIN = '.\build-cuda\bin\Release\llama-server.exe'
$MODEL = 'D:\Hy4-flash-SPARK-4Bit.gguf'
$MMPROJ = 'D:\mmproj-Hy4-flash-bf16.gguf'

& $SERVER_BIN `
  --model $MODEL `
  --mmproj $MMPROJ --mmproj-offload `
  --alias hy4-flash `
  --host 127.0.0.1 --port 8080 `
  --ctx-size 196608 --parallel 1 `
  --n-gpu-layers 999 --split-mode none `
  --override-tensor '^per_layer_token_embd\.weight$=CPU,^token_embd\.weight$=CUDA0' `
  --load-mode mmap --lazy-mode on --no-host `
  --fit off --flash-attn on `
  --batch-size 2048 --ubatch-size 1024 `
  --threads 4 --threads-batch 4 `
  --cache-type-k f16 --cache-type-v f16 `
  --spec-type draft-mtp --metrics --jinja
```

Keep the GGUF on a local SSD. The SPARK-4Bit GGUF relies on mmap/lazy access to CPU PLE rows; placing PLE on CPU does not add physical memory on Spark. The recommended configuration uses a 196608-token context; check memory usage on your device and lower it if necessary.

### Enable DFly

The same `hy4-flash-spark.patch` also enables DFly for text generation.

1. Save the separate draft GGUF on your local SSD. Keep `--model` pointed at your main Hy4-flash GGUF. [Download `hy4-flash_dfly_Q8_0.gguf`](https://huggingface.co/AngelSlim/Hy4-flash-GGUF/resolve/main/hy4-flash_dfly_Q8_0.gguf)
2. For text-only DFly serving, remove the `--mmproj` and `--mmproj-offload` line from the server command. Keep `--parallel 1`, `--fit off`, and the F16 KV cache. Replace the final `--spec-type draft-mtp --metrics --jinja` line with the block for your shell and adjust the draft file path.

**Linux**

```bash
  --no-mmproj --no-context-shift --cache-reuse 0 \
  --spec-type draft-dfly \
  --spec-draft-model "/local-ssd/hy4-flash_dfly_Q8_0.gguf" \
  --spec-draft-ngl 999 --spec-draft-n-max 4 --spec-draft-n-min 0 \
  --spec-draft-p-min 0 --spec-draft-sampling greedy --metrics --jinja
```

**Windows / PowerShell**

```powershell
  --no-mmproj --no-context-shift --cache-reuse 0 `
  --spec-type draft-dfly `
  --spec-draft-model 'D:\hy4-flash_dfly_Q8_0.gguf' `
  --spec-draft-ngl 999 --spec-draft-n-max 4 --spec-draft-n-min 0 `
  --spec-draft-p-min 0 --spec-draft-sampling greedy --metrics --jinja
```

## 中文

### 在 NVIDIA Spark 上运行 Hy4-flash 的建议

建议在 **NVIDIA DGX Spark** 上使用 Hy4-flash 的 **SPARK-4Bit GGUF** 版本。编译 llama.cpp 前先应用 `hy4-flash-spark.patch`。这一份补丁已合并 Hy4-flash 架构支持与 CUDA 加速，并保留 MTP。具体编译步骤、运行开关和完整服务启动命令如下。

### 编译

将 `hy4-flash-spark.patch` 放在待克隆目录的同级位置。补丁对应下面这个固定的上游 commit。

```bash
git clone https://github.com/ggml-org/llama.cpp.git llama.cpp-hy4-flash
cd llama.cpp-hy4-flash
git checkout 2cbca389f515df455412dafa07e2b1359cc72a6d

git apply --check ../hy4-flash-spark.patch
git apply ../hy4-flash-spark.patch

export PATH=/usr/local/cuda-13.0/bin:$PATH
export CUDACXX=/usr/local/cuda-13.0/bin/nvcc

cmake -S . -B build-cuda \
  -DGGML_CUDA=ON \
  -DGGML_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_CUDA_ARCHITECTURES=121a \
  -DLLAMA_OPENSSL=OFF \
  -DLLAMA_BUILD_UI=OFF \
  -DLLAMA_USE_PREBUILT_UI=OFF

cmake --build build-cuda --config Release \
  --target llama-server llama-bench --parallel 8
```

Spark / GB10（CC 12.1）使用 `121a`；CUDA 路径按本机安装位置修改，并在目标主机架构上编译。这里用 `LLAMA_OPENSSL=OFF` 简化本地模型运行的依赖，需要 HTTPS 时可改为 `ON`。

### 加速效果

我们的 patch 进一步优化了 SPARK-4Bit 矩阵乘法、gate/up 与 HC 融合、Q8 activation 复用以及 MoE 并发，让端侧设备上的 Hy4-flash 推理更快。相对仅包含架构支持的版本，本地 A/B 验证的参考增益约为 **Prefill +30% / Decode +20%**。实际收益随硬件和负载变化，不代表 Spark 实机测速。

### 运行开关

下表为后续命令使用的推荐配置；开关还需要匹配张量类型、形状等条件才会实际触发。

| 环境变量 | 设置 | 含义 |
|---|---|---|
| `GGML_SCHED_BATCH_HOST_INPUTS` | **0 for Linux, 2 for Windows** | 成组复制 host 输入并统一同步；`0` 关闭，`1` 仅外部 INPUT 张量，`2` 额外包含 CPU 中间结果。 |
| `GGML_CUDA_FUSE_HC_GATE` | `1` | 融合 HC gate 算子，减少 kernel 启动。 |
| `GGML_CUDA_FUSE_HC_CONSUMER` | `1` | 将 HC gate 与符合条件的 HC pre/post 消费算子进一步融合。 |
| `GGML_CUDA_REUSE_Q8` | `1` | 在符合条件的 MMVQ 运算间复用 Q8 activation，减少重复量化。 |
| `GGML_CUDA_SPARK_MMQ_FUSION` | `1` | 融合符合条件的 SPARK-4Bit 专家 gate/up 矩阵乘法。 |
| `GGML_CUDA_SPARK_MMQ_COMPACT_GRID` | `1` | 为符合条件的 SPARK-4Bit MMQ 使用紧凑 grid，减少无效工作。 |
| `GGML_CUDA_SPARK_MMQ_TILE` | `64` | 设置 SPARK-4Bit 专家 MMQ 的 tile 宽度提示值。 |
| `GGML_CUDA_SPARK_MMQ_TILE_FUSED` | `64` | 设置融合 SPARK-4Bit 专家 MMQ 的 tile 宽度提示值。 |
| `GGML_CUDA_MMQ_PREC` | `q4` | 对支持的 SPARK-4Bit MMQ 路径使用 4-bit activation。 |
| `GGML_CUDA_CPY_F32_VEC` | `1` | 向量化符合条件的 F32 布局拷贝。 |
| `GGML_CUDA_SPARK_MMVQ_2W` | `1` | 对单 token、K=1536 等条件匹配的 SPARK-4Bit 专家 MMVQ 使用两个 warp。 |
| `GGML_CUDA_MOE_CONCURRENT` | `1` | 并发执行符合条件的共享专家与路由专家分支。 |
| `GGML_CUDA_FA_576_WHOLE_TILES` | `0` | 保持关闭 head size=576 的可选 whole-tile Flash Attention 路径。 |

`GGML_SCHED_BATCH_HOST_INPUTS` 推荐在 Linux 上设为 `0`，在 Windows 上设为 `2`。实际性能随设备和负载变化。

### 启动服务

推荐的服务配置：context=196608、单 slot、batch/ubatch=2048/1024、CPU 线程数=4、F16 KV cache、Flash Attention、MTP、PLE 放 CPU、token embedding 放 CUDA0。请替换模型和 projector 路径。纯文本服务可去掉 `--mmproj` 与 `--mmproj-offload`。

**Linux**

```bash
export CUDA_VISIBLE_DEVICES=0
export GGML_SCHED_BATCH_HOST_INPUTS=0
export GGML_CUDA_FUSE_HC_GATE=1
export GGML_CUDA_FUSE_HC_CONSUMER=1
export GGML_CUDA_REUSE_Q8=1
export GGML_CUDA_SPARK_MMQ_FUSION=1
export GGML_CUDA_SPARK_MMQ_COMPACT_GRID=1
export GGML_CUDA_SPARK_MMQ_TILE=64
export GGML_CUDA_SPARK_MMQ_TILE_FUSED=64
export GGML_CUDA_MMQ_PREC=q4
export GGML_CUDA_CPY_F32_VEC=1
export GGML_CUDA_SPARK_MMVQ_2W=1
export GGML_CUDA_MOE_CONCURRENT=1
export GGML_CUDA_FA_576_WHOLE_TILES=0

MODEL=/local-ssd/Hy4-flash-SPARK-4Bit.gguf
MMPROJ=/local-ssd/mmproj-Hy4-flash-bf16.gguf

./build-cuda/bin/llama-server \
  --model "$MODEL" \
  --mmproj "$MMPROJ" --mmproj-offload \
  --alias hy4-flash \
  --host 127.0.0.1 --port 8080 \
  --ctx-size 196608 --parallel 1 \
  --n-gpu-layers 999 --split-mode none \
  --override-tensor '^per_layer_token_embd\.weight$=CPU,^token_embd\.weight$=CUDA0' \
  --load-mode mmap --lazy-mode on --no-host \
  --fit off --flash-attn on \
  --batch-size 2048 --ubatch-size 1024 \
  --threads 4 --threads-batch 4 \
  --cache-type-k f16 --cache-type-v f16 \
  --spec-type draft-mtp --metrics --jinja
```

**Windows / PowerShell**

使用相同补丁源码的 Windows 构建，将 `SERVER_BIN` 指向对应可执行文件。

```powershell
$env:CUDA_VISIBLE_DEVICES = '0'
$env:GGML_SCHED_BATCH_HOST_INPUTS = '2'
$env:GGML_CUDA_FUSE_HC_GATE = '1'
$env:GGML_CUDA_FUSE_HC_CONSUMER = '1'
$env:GGML_CUDA_REUSE_Q8 = '1'
$env:GGML_CUDA_SPARK_MMQ_FUSION = '1'
$env:GGML_CUDA_SPARK_MMQ_COMPACT_GRID = '1'
$env:GGML_CUDA_SPARK_MMQ_TILE = '64'
$env:GGML_CUDA_SPARK_MMQ_TILE_FUSED = '64'
$env:GGML_CUDA_MMQ_PREC = 'q4'
$env:GGML_CUDA_CPY_F32_VEC = '1'
$env:GGML_CUDA_SPARK_MMVQ_2W = '1'
$env:GGML_CUDA_MOE_CONCURRENT = '1'
$env:GGML_CUDA_FA_576_WHOLE_TILES = '0'

$SERVER_BIN = '.\build-cuda\bin\Release\llama-server.exe'
$MODEL = 'D:\Hy4-flash-SPARK-4Bit.gguf'
$MMPROJ = 'D:\mmproj-Hy4-flash-bf16.gguf'

& $SERVER_BIN `
  --model $MODEL `
  --mmproj $MMPROJ --mmproj-offload `
  --alias hy4-flash `
  --host 127.0.0.1 --port 8080 `
  --ctx-size 196608 --parallel 1 `
  --n-gpu-layers 999 --split-mode none `
  --override-tensor '^per_layer_token_embd\.weight$=CPU,^token_embd\.weight$=CUDA0' `
  --load-mode mmap --lazy-mode on --no-host `
  --fit off --flash-attn on `
  --batch-size 2048 --ubatch-size 1024 `
  --threads 4 --threads-batch 4 `
  --cache-type-k f16 --cache-type-v f16 `
  --spec-type draft-mtp --metrics --jinja
```

GGUF 建议放在本地 SSD。SPARK-4Bit GGUF 依赖 CPU PLE 行的 mmap/lazy 访问；Spark 上将 PLE 放 CPU 不会增加物理内存。推荐配置使用 196608-token context，请根据设备的实际内存占用按需调小。

### 启用 DFly

同一份 `hy4-flash-spark.patch` 也支持用 DFly 做文本生成。

1. 将独立草稿权重保存到本地 SSD。`--model` 仍指向 Hy4-flash 主模型 GGUF。[下载 `hy4-flash_dfly_Q8_0.gguf`](https://huggingface.co/AngelSlim/Hy4-flash-GGUF/resolve/main/hy4-flash_dfly_Q8_0.gguf)
2. 启动纯文本 DFly 服务时，删除启动命令中包含 `--mmproj` 和 `--mmproj-offload` 的整行，保留 `--parallel 1`、`--fit off` 和 F16 KV cache。用对应平台的参数块替换最后一行 `--spec-type draft-mtp --metrics --jinja`，并填入实际草稿文件路径。

**Linux**

```bash
  --no-mmproj --no-context-shift --cache-reuse 0 \
  --spec-type draft-dfly \
  --spec-draft-model "/local-ssd/hy4-flash_dfly_Q8_0.gguf" \
  --spec-draft-ngl 999 --spec-draft-n-max 4 --spec-draft-n-min 0 \
  --spec-draft-p-min 0 --spec-draft-sampling greedy --metrics --jinja
```

**Windows / PowerShell**

```powershell
  --no-mmproj --no-context-shift --cache-reuse 0 `
  --spec-type draft-dfly `
  --spec-draft-model 'D:\hy4-flash_dfly_Q8_0.gguf' `
  --spec-draft-ngl 999 --spec-draft-n-max 4 --spec-draft-n-min 0 `
  --spec-draft-p-min 0 --spec-draft-sampling greedy --metrics --jinja
```
