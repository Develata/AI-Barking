# Running Hy4-flash on Apple silicon (Metal)

[English](#english) | [中文](#中文)

Covers `hy4-flash-mac-minimal.patch` on top of upstream `2cbca389f`: architecture support, Metal kernels and fusions, two correctness fixes (index-pool tail selection, MTP acceptance with a quantized K cache), and `hy_v4_next_dfly` draft support.

---

## English

### 0. Modes

| Mode | Decode | Vision |
|---|---|---|
| AR | ~36 tok/s | yes |
| MTP (`draft-mtp`, K=2) | 44-48 tok/s  | yes |
| DFly (`--dfly`, K=4) | 51.95 tok/s | no |

### 1. Build

```bash
git clone https://github.com/ggml-org/llama.cpp
cd llama.cpp
git checkout 2cbca389f
git apply hy4-flash-mac-minimal.patch

cmake -B build -DCMAKE_BUILD_TYPE=Release -DGGML_METAL=ON -DGGML_METAL_EMBED_LIBRARY=ON
cmake --build build -j --target llama-server llama-cli llama-bench
```

### 2. System setup

```bash
sudo sysctl iogpu.wired_limit_mb=114688      # 112 GiB, resets on reboot; weights are ~97.4 GiB
```

- Keep the GGUF on a local SSD (the ~59 GiB Engram table is lazily mapped).
- Run on AC power.
- Stop the server before `llama-bench` or any other GPU job.

### 3. Memory budget

| Component | Size |
|---|---|
| f16 KV cache + index pool | 5.7 GiB / 256k tokens |
| compute buffer | 2.4 GiB at `-ub 512`, 4.5 GiB at `-ub 1024` (256k pool) |
| MTP draft context | 0.9 GiB at `--spec-draft-n-ubatch 256` |
| vision projector | ~1.0 GiB |

| Configuration | Total | Result |
|---|---|---|
| 4 slots, 512k pool, `-ub 1024` | 113.3 GiB | fails (OOM) |
| 4 slots, 256k shared pool, `-ub 512` | 105.5 GiB | fine |
| 4 slots, 256k shared pool, `-ub 1024` | ~107.6 GiB | fine |

On `kIOGPUCommandBufferCallbackErrorOutOfMemory`, shrink the pool before `-ub`, then restart the server.

### 4. Speed changes

All kernel changes are bit-identical to the unfused result.

| Area | Change | Gain |
|---|---|---|
| Graph reuse | fix: the K-only iSWA input never reused its graph under unified KV + FA | decode +12% |
| Prefill | dense FA kernels for sparse layers when ubatch >= 32, n_kv <= 12288 | pp4096 +25% |
| Decode attention | MLA decode kernel: 1 threadgroup / 16 heads, shared K tiles; indexer reads pooled keys in place | 48k 31.8->34.9, 64k 32.1->34.6 tok/s |
| Fusions | HC gate, HC_PRE/POST+RMS_NORM, sigmoid+mul, MoE reduce (16 experts), ROPE+CONCAT, ADD+RMS_NORM+MUL | dispatches/token 2861->2384; 48k ->37.2, 64k ->37.1 tok/s |
| Short KV | 1 workgroup for sliding-window FA | +3% |
| Verify | sparse MLA kernels take multi-row blocks (MTP/DFly verify) | shortens verify proportionally |
| Verify (DFly) | 1 workgroup for a short decode batch | shortens DFly verify further |

~10.4 GB/token at ~550 GB/s is the floor (~19 of ~27 ms); the target's quantization is untouched.

### 5. Runtime switches (from the patch)

| Variable / flag | Value | Effect |
|---|---|---|
| `iogpu.wired_limit_mb` | `114688` | Metal working-set limit |
| `HY4_MLA_DISABLE` | any | fall back to generic vec kernels |
| `GGML_METAL_OP_PROFILE` | `1` | per-op GPU time (profiling) |
| `GGML_METAL_CB_TIMING` | `1` | per-command-buffer timing (profiling) |
| `GGML_METAL_N_CB`, `GGML_METAL_N_MAIN` | default | encoder threads; no gain measured |
| `--dfly FNAME` | path to a `hy_v4_next_dfly` GGUF | shorthand for `--model-draft FNAME --spec-type draft-dfly`; MTP block stays unloaded unless `draft-mtp` is also requested |

`--spec-type draft-mtp` and the `--spec-draft-*` flags, `--no-cache-idle-slots`, `LLAMA_GRAPH_REUSE_DISABLE` are upstream flags, not added by this patch. `--dfly` is.

`--dfly` also pins the target's `token_embd`/`output` to the first GPU/iGPU device (DFly needs them, every draft weight, and the K cache of the layers it reads on one device; `--n-gpu-layers all` otherwise leaves them on the host) — unless `-ot` already placed one of them, or `--n-gpu-layers 0` asks for CPU only.

### 6. Start the server

```bash
sudo sysctl iogpu.wired_limit_mb=114688

build/bin/llama-server \
  --model /path/to/A12B-Q4_K_M-00001-of-00002.gguf \
  --mmproj /path/to/mmproj-A12B-bf16.gguf \
  --alias hy4-flash-a12b --host 127.0.0.1 --port 8080 \
  --ctx-size 262144 --kv-unified --kv-unified-per-slot 262144 --parallel 2 \
  --n-gpu-layers all --flash-attn on \
  --cache-type-k f16 --cache-type-v f16 \
  --batch-size 2048 --ubatch-size 1024 \
  --cache-ram 8192 --ctx-checkpoints 32 \
  --slot-save-path /path/to/slots \
  --reasoning-format auto --jinja --metrics \
  --n-predict 8192 \
  --spec-type draft-mtp --spec-draft-n-max 2 --spec-draft-p-min 0 --spec-draft-n-ubatch 256
```

Drop the last line for plain AR.

| Flag | Value | Why |
|---|---|---|
| `--ctx-size` / `--kv-unified-per-slot` / `--parallel` | 262144 / 262144 / 2 | 2 slots share one 256k pool; 2 reserved slots OOM with the projector |
| `--ubatch-size` / `--batch-size` | 1024 / 2048 | prefill +16% over 512 |
| `--cache-type-k/-v` | f16 | q8_0 KV is slower here (one latent KV head, re-decoded per query head) |
| `--cache-ram` | 8192 | host prompt cache |
| `--n-predict` | 8192 | upper bound for an uncancelled stream |
| `--spec-draft-n-max` | 2 | extra verify rows aren't free (2 rows = 1.31x, 4 rows = 1.74x); measure K=0 alongside |
| `--spec-draft-n-ubatch` | 256 | keeps the draft compute buffer at ~0.9 GiB instead of 3.75 |

MTP verification is exact rejection sampling, so output matches plain decoding.

For DFly instead of MTP, replace the last line and drop `--mmproj`/`--slot-save-path` (DFly rejects both) and add `--fit off`:

```bash
build/bin/llama-server \
  --model /path/to/A12B-Q4_K_M-00001-of-00002.gguf \
  --alias hy4-flash-a12b --host 127.0.0.1 --port 8080 \
  --ctx-size 262144 --kv-unified --kv-unified-per-slot 262144 --parallel 2 \
  --n-gpu-layers all --flash-attn on \
  --cache-type-k f16 --cache-type-v f16 \
  --batch-size 2048 --ubatch-size 1024 \
  --cache-ram 8192 --ctx-checkpoints 32 \
  --reasoning-format auto --jinja --metrics \
  --n-predict 8192 \
  --fit off \
  --dfly /path/to/hy4-flash_dfly_Q8_0.gguf
```

DFly also needs: `--cache-reuse 0` (default), no `--swa-full` (default), no `--context-shift` (default), no `--backend-sampling`, no `--lora`, and `--ubatch-size` of at least `1 + K`. All satisfied by the defaults above; the server names whatever is missing and refuses to load rather than guess.

### 7. Measured results

#### 7.1 MTP (shipped draft block)

From the [AngelSlim/Hy4-flash-GGUF model card](https://huggingface.co/AngelSlim/Hy4-flash-GGUF), 128 GB Mac M5 Max, Metal:

| Format | Prefill tok/s (4K / 16K / 32K) | Decode tok/s (4K / 16K / 32K) |
|---|---|---|
| Q4_K_M | 629.8 / 448.1 / 418.6 | 48.13 / 45.18 / 44.15 |
| Q3_K | 743.2 / 563.0 / 520.6 | 50.34 / 49.47 / 47.37 |

#### 7.2 DFly (this patch)

128 GB Apple silicon, Metal, `Q4_K_M`, real agent prompt, K=4, B=8, greedy.

| Scenario | Decode | Accept rate | Mean accepted len |
|---|---|---|---|
| decode, ~81k context | 51.95 tok/s | 0.611 | 3.44 |

---

## 中文

### 0. 模式

| 模式 | decode | 视觉 |
|---|---|---|
| AR | 约 36 tok/s | 支持 |
| MTP（`draft-mtp`，K=2） | 44-48 tok/s | 支持 |
| DFly（`--dfly`，K=4） | 51.95 tok/s| 不支持 |

### 1. 编译

```bash
git clone https://github.com/ggml-org/llama.cpp
cd llama.cpp
git checkout 2cbca389f
git apply hy4-flash-mac-minimal.patch

cmake -B build -DCMAKE_BUILD_TYPE=Release -DGGML_METAL=ON -DGGML_METAL_EMBED_LIBRARY=ON
cmake --build build -j --target llama-server llama-cli llama-bench
```

### 2. 系统设置

```bash
sudo sysctl iogpu.wired_limit_mb=114688      # 112 GiB，重启失效；权重约 97.4 GiB
```

- GGUF 放本地 SSD（Engram 表约 59 GiB，惰性映射）。
- 接电源运行。
- 跑 `llama-bench` 等 GPU 任务前先停 server。

### 3. 显存预算

| 组成 | 大小 |
|---|---|
| f16 KV cache + 索引池 | 每 256k token 5.7 GiB |
| compute buffer | `-ub 512` 2.4 GiB，`-ub 1024` 4.5 GiB（256k 池） |
| MTP 草稿 context | `--spec-draft-n-ubatch 256` 时 0.9 GiB |
| 视觉投影 | 约 1.0 GiB |

| 配置 | 合计 | 结果 |
|---|---|---|
| 4 槽，512k 池，`-ub 1024` | 113.3 GiB | 失败（OOM） |
| 4 槽，256k 共享池，`-ub 512` | 105.5 GiB | 正常 |
| 4 槽，256k 共享池，`-ub 1024` | 约 107.6 GiB | 正常 |

报 `kIOGPUCommandBufferCallbackErrorOutOfMemory` 时先缩池再缩 `-ub`，然后重启 server。

### 4. 速度改动

所有 kernel 改动与非融合结果逐位相同。

| 方面 | 改动 | 收益 |
|---|---|---|
| 计算图复用 | 修复：K-only iSWA 输入在统一 KV + FA 下从不复用计算图 | decode +12% |
| prefill | ubatch ≥ 32、n_kv ≤ 12288 时稀疏层改走 dense FA kernel | pp4096 +25% |
| decode 注意力 | MLA decode kernel：1 threadgroup 管 16 头，共享 K tile；indexer 原地读池化键 | 48k 31.8→34.9，64k 32.1→34.6 tok/s |
| 融合 | HC gate、HC_PRE/POST+RMS_NORM、sigmoid+mul、16 专家 MoE reduce、ROPE+CONCAT、ADD+RMS_NORM+MUL | dispatch/token 2861→2384；48k →37.2，64k →37.1 tok/s |
| 短 KV | 滑窗 FA 用单 workgroup | +3% |
| 验证 | 稀疏 MLA kernel 支持多行一起验证（MTP/DFly） | 按行数等比缩短 |
| 验证（DFly） | 短 decode batch 单 workgroup | 进一步缩短 DFly 验证 |

每 token 约 10.4 GB、约 550 GB/s，下限约 19/27 ms；target 的量化没有改动。

### 5. 运行开关（来自 patch）

| 变量 / 参数 | 取值 | 作用 |
|---|---|---|
| `iogpu.wired_limit_mb` | `114688` | Metal 工作集上限 |
| `HY4_MLA_DISABLE` | 任意 | 退回通用 vec kernel |
| `GGML_METAL_OP_PROFILE` | `1` | 逐算子 GPU 耗时（调试） |
| `GGML_METAL_CB_TIMING` | `1` | 逐 command buffer 计时（调试） |
| `GGML_METAL_N_CB`、`GGML_METAL_N_MAIN` | 默认 | 编码线程数；未测出收益 |
| `--dfly FNAME` | `hy_v4_next_dfly` GGUF 路径 | 等价于 `--model-draft FNAME --spec-type draft-dfly`；没额外要 `draft-mtp` 时 MTP block 不加载 |

`--spec-type draft-mtp` 及 `--spec-draft-*`、`--no-cache-idle-slots`、`LLAMA_GRAPH_REUSE_DISABLE` 是上游参数，不是 patch 加的；`--dfly` 是 patch 加的。

`--dfly` 还会把目标模型的 `token_embd`/`output` 固定到第一个 GPU/iGPU 设备（DFly 要求它们、每个草稿权重、以及它读的那几层目标 K cache 都在同一设备；`--n-gpu-layers all` 默认不会挪这两个张量），除非 `-ot` 已经指定过，或 `--n-gpu-layers 0` 要求纯 CPU。

### 6. 启动服务

```bash
sudo sysctl iogpu.wired_limit_mb=114688

build/bin/llama-server \
  --model /path/to/A12B-Q4_K_M-00001-of-00002.gguf \
  --mmproj /path/to/mmproj-A12B-bf16.gguf \
  --alias hy4-flash-a12b --host 127.0.0.1 --port 8080 \
  --ctx-size 262144 --kv-unified --kv-unified-per-slot 262144 --parallel 2 \
  --n-gpu-layers all --flash-attn on \
  --cache-type-k f16 --cache-type-v f16 \
  --batch-size 2048 --ubatch-size 1024 \
  --cache-ram 8192 --ctx-checkpoints 32 \
  --slot-save-path /path/to/slots \
  --reasoning-format auto --jinja --metrics \
  --n-predict 8192 \
  --spec-type draft-mtp --spec-draft-n-max 2 --spec-draft-p-min 0 --spec-draft-n-ubatch 256
```

去掉最后一行即为普通 AR。

| 参数 | 取值 | 原因 |
|---|---|---|
| `--ctx-size` / `--kv-unified-per-slot` / `--parallel` | 262144 / 262144 / 2 | 两槽共用一个 256k 池；各自预留会连同视觉投影一起 OOM |
| `--ubatch-size` / `--batch-size` | 1024 / 2048 | 比 512 的 prefill 快 16% |
| `--cache-type-k/-v` | f16 | q8_0 KV 更慢（唯一 latent KV 头被每个 query 头重复解码） |
| `--cache-ram` | 8192 | 内存 prompt cache |
| `--n-predict` | 8192 | 未取消流的生成上界 |
| `--spec-draft-n-max` | 2 | 多出的验证行不是免费的（2 行 1.31 倍，4 行 1.74 倍），请和 K=0 对比 |
| `--spec-draft-n-ubatch` | 256 | 把草稿 compute buffer 压在约 0.9 GiB，而不是 3.75 |

MTP 验证是精确拒绝采样，输出与普通解码一致。

换成 DFly：改最后一行，去掉 `--mmproj`/`--slot-save-path`（DFly 都不接受），加 `--fit off`：

```bash
build/bin/llama-server \
  --model /path/to/A12B-Q4_K_M-00001-of-00002.gguf \
  --alias hy4-flash-a12b --host 127.0.0.1 --port 8080 \
  --ctx-size 262144 --kv-unified --kv-unified-per-slot 262144 --parallel 2 \
  --n-gpu-layers all --flash-attn on \
  --cache-type-k f16 --cache-type-v f16 \
  --batch-size 2048 --ubatch-size 1024 \
  --cache-ram 8192 --ctx-checkpoints 32 \
  --reasoning-format auto --jinja --metrics \
  --n-predict 8192 \
  --fit off \
  --dfly /path/to/hy4-flash_dfly_Q8_0.gguf
```

DFly 还要求：`--cache-reuse 0`（默认）、不开 `--swa-full`（默认）、不开 `--context-shift`（默认）、不开 `--backend-sampling`、不带 `--lora`、`--ubatch-size` 至少 `1 + K`。以上默认值全部满足；缺什么 server 会直接报出来，不会猜。

### 7. 实测结果

#### 7.1 MTP（自带草稿块）

来自 [AngelSlim/Hy4-flash-GGUF 模型主页](https://huggingface.co/AngelSlim/Hy4-flash-GGUF)，128 GB Mac M5 Max，Metal：

| 格式 | Prefill tok/s（4K / 16K / 32K） | Decode tok/s（4K / 16K / 32K） |
|---|---|---|
| Q4_K_M | 629.8 / 448.1 / 418.6 | 48.13 / 45.18 / 44.15 |
| Q3_K | 743.2 / 563.0 / 520.6 | 50.34 / 49.47 / 47.37 |

#### 7.2 DFly（本 patch）

128 GB Apple silicon，Metal，`Q4_K_M`，真实 agent prompt，K=4，B=8，greedy。

| 场景 | decode | 接受率 | 均接受长度 |
|---|---|---|---|
| decode，约 81k 上下文 | 51.95 tok/s | 0.611 | 3.44 |

