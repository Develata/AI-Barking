[English](./README.md) ｜ 中文

<p align="center">
  <img src="https://github.com/Tencent/AngelSlim/blob/main/docs/source/assets/logos/angelslim_logo.png?raw=true" alt="AngelSlim" width="70%">
</p>

### A more accessible, comprehensive, and efficient toolkit for large model compression.

<p align="center">
  ✒️ <a href="https://arxiv.org/abs/2602.21233">TechnicalReport</a>&nbsp;&nbsp;|&nbsp;&nbsp;📖 <a href="https://angelslim.readthedocs.io/">Documentation</a>&nbsp;&nbsp;|&nbsp;&nbsp;🤗 <a href="https://huggingface.co/AngelSlim">Hugging Face</a>&nbsp;&nbsp;|&nbsp;&nbsp;🤖 <a href="https://modelscope.cn/organization/AngelSlim">ModelScope</a>
  <br>
  💬 <a href="https://github.com/Tencent/AngelSlim/blob/main/docs/source/assets/angel_slim_wechat.png?raw=true">WeChat</a>&nbsp;&nbsp;|&nbsp;&nbsp;🫨 <a href="https://discord.com/invite/dHVNeuNdFt">Discord</a>
</p>

<p align="center">
  <img src="https://huggingface.co/AngelSlim/Hy4-flash-GGUF/resolve/main/angelslim-hy4flash-edge-light.gif" alt="AngelSlim x Hy4-flash：把 224B 参数 Hy4-flash 模型，带到你桌边的设备上" width="100%">
</p>

# Hy4-flash GGUF

Hy4-flash 的五个 GGUF 版本：[tencent/Hy4-flash](https://huggingface.co/tencent/Hy4-flash)

| 文件                               | 模型大小       | bpw（整体） | bpw（不含 Engram 表） |
| -------------------------------- | ---------: | ------: | ---------------: |
| `Hy4-flash-Q4_K_M.gguf`          | 156.79 GiB | 6.05    | 5.27             |
| `Hy4-flash-SPARK-4Bit.gguf`      | 151.3 GiB  | 5.84    | 4.78             |
| `Hy4-flash-SPARK-4Bit-Fast.gguf` | 148.6 GiB  | 5.74    | 4.63             |
| `Hy4-flash-Q3_K.gguf`            | 144.70 GiB | 5.58    | 4.62             |
| `Hy4-flash-IQ2-AH.gguf`          | 125.73 GiB | 4.85    | 3.60             |

![Hy4-flash GGUF 各版本评测结果](https://huggingface.co/AngelSlim/Hy4-flash-GGUF/resolve/main/hy4-flash-gguf-benchmark.png)

---

## 1. 这是什么

`Hy4-flash-Q4_K_M.gguf` —— 标准 4 bit 版本。路由专家沿用 llama.cpp 的 Q4_K_M 规则：`gate`/`up` 为
Q4_K，`down` 按 llama.cpp 的固定规则在 19 层用 Q6_K、其余 22 层用 Q4_K。多数非专家权重为 Q8_0。**没有显存压力就用这个。**

`Hy4-flash-SPARK-4Bit.gguf` —— SPARK-4Bit 版本（NVIDIA 的 4 bit 浮点格式）。**适用于 Spark 设备。**

`Hy4-flash-SPARK-4Bit-Fast.gguf` —— SPARK-4Bit 的变体，比 SPARK-4Bit 小 2.7 GiB。**同样适用于 Spark 设备。**

`Hy4-flash-Q3_K.gguf` —— 专家层 Q3_K/Q4_K 混合。路由专家的 `gate`/`up` 在按量化敏感性选出的
19 层保留 Q4_K，其余 22 层用 Q3_K；`down` 为 Q4_K。路由专家以外的张量与 Q4_K_M 相同。

`Hy4-flash-IQ2-AH.gguf` —— 五个版本中体积最小。路由专家的 `gate`/`up` 用 2.0625 bpw（IQ2_XXS），`down` 用 Q4_K；多数非专家权重保留 BF16，而不是 Q8_0。

**Strata** 支持：见 PR [#7120](https://github.com/Niko1221/Strata/pull/7120)。

## 2. SPEED

### 2.1 llama.cpp

测试于真实workbuddy场景和数据。

**Mac M5 Max 128GB**

| 格式     | Prefill tok/s (4K / 16K / 32K) | Decode tok/s (4K / 16K / 32K) |
| ------ | ----------------------------: | ----------------------------: |
| Q4_K_M | 629.8 / 448.1 / 418.6         | 48.13 / 45.18 / 44.15         |
| Q3_K   | 743.2 / 563.0 / 520.6         | 50.34 / 49.47 / 47.37         |

对于 code 生成场景，Q3_K 可达 70 tok/s。

**RTX Spark 128GB**

| format          | Prefill tok/s (4K / 16K / 32K) | Decode tok/s (4K / 16K / 32K) |
| --------------- | ----------------------------: | ----------------------------: |
| SPARK-4Bit-Fast | 799 / 717 / 707               | 31.95 / 33.14 / 33.39         |
| SPARK-4Bit      | 765 / 686 / 666               | 25.80 / 28.16 / 30.3          |

对于 code 生成场景，SPARK-4Bit-Fast 可达 40 tok/s。

### 2.2 Strata

**NVIDIA CUDA — 16 GiB 显存预算**

完整版 Q4_K_M_MTP_Q8_0 模型，输入 4K / 输出 512 token，上下文上限 33,024 token；每个场景测试一次。

| 场景 | Decode tok/s（关闭投机解码） | Decode tok/s（DFly Q8_0） |
| ---- | --------------------------: | ------------------------: |
| 正文 | 36.8 | 38.7 |
| 代码 | 37.5 | 47.6 |

**Mac mini M4 Pro 64GB**

IQ2 模型，MTP k=2，8K 上下文：Prefill **125–127 tok/s**，Decode **17.3 tok/s**
（生成 256 token，取四个提示词测试中位数的几何平均值）。

以上数据与 llama.cpp 的测试条件不同。

## 3. 如何使用

### 3.1 llama.cpp

**通用运行**
相关支持在 PR [#32517](https://github.com/ggml-org/llama.cpp/pull/32517) 中,还没有合并,需要拉取 PR 分支:

```sh
git clone https://github.com/ggml-org/llama.cpp
cd llama.cpp
git fetch origin pull/32517/head:hy4-flash
git checkout hy4-flash
```
编译

```sh
cmake -B build                # NVIDIA 显卡加 -DGGML_CUDA=ON;macOS 默认启用 Metal
cmake --build build --config Release -j
```

终端对话:
```sh
./build/bin/llama-cli -m Hy4-flash-Q4_K_M-00001-of-00002.gguf -c 8192 -ngl 99 --jinja
```

OpenAI 兼容服务(http://localhost:8080):

```sh
./build/bin/llama-server -m Hy4-flash-Q4_K_M-00001-of-00002.gguf -c 8192 -ngl 99 --jinja --port 8080
```

图片输入(需要 mmproj 文件):
```sh
./build/bin/llama-mtmd-cli -m Hy4-flash-Q4_K_M-00001-of-00002.gguf --mmproj mmproj-Hy4-flash-bf16.gguf \
    --image photo.png -p "描述这张图片。" -c 8192 -ngl 99 --jinja
```


**Mac M5 Max 128GB 优化版本**

见 [README_MAC-Metal.md](https://huggingface.co/AngelSlim/Hy4-flash-GGUF/blob/main/README_MAC-Metal.md)

**RTX Spark 128GB 优化版本**

见[README_SPARK-4Bit.md](https://huggingface.co/AngelSlim/Hy4-flash-GGUF/blob/main/README_SPARK-4Bit.md)

### 3.2 Strata

NVIDIA CUDA 和 Apple Silicon Metal 支持见 PR
[#7120](https://github.com/Niko1221/Strata/pull/7120)，先拉取 PR 分支：

```sh
git clone https://github.com/Niko1221/Strata
cd Strata
git fetch origin pull/7120/head:hy4-flash-gguf
git checkout hy4-flash-gguf
```

按照 [CUDA 指南](https://github.com/sjl623/Strata/blob/777267388d3d3316fccee91b521c5a6c60e782ef/docs/hy4-flash/README.zh-CN.md)
或 [Metal 指南](https://github.com/sjl623/Strata/blob/777267388d3d3316fccee91b521c5a6c60e782ef/docs/hy4-flash/README_MAC-Metal.zh-CN.md)
编译引擎并安装依赖。两个 GGUF 分片需放在同一目录。

**NVIDIA CUDA** — 使用 Q4_K_M_MTP_Q8_0，支持 CPU 专家卸载和 MTP / DFly 投机解码：

```sh
MODEL=/path/to/Hy4-flash-Q4_K_M_MTP_Q8_0-00001-of-00002.gguf CTX=32768 \
  ./tools/hy4_flash/launch-hy4-flash.sh --no-tui
```

**Apple Silicon Metal** — 使用 IQ2，支持从 SSD 按需加载专家和内置 MTP：

```sh
MODEL=/path/to/Hy4-flash-IQ2_AH-00001-of-00002.gguf \
  STRATA_BIN="$PWD/build-metal/strata" KV=fp16 CTX=32768 MTP=on DRAFT_TOKENS=2 \
  ./tools/hy4_flash/launch-hy4-flash.sh --no-tui
```

OpenAI 兼容 API 地址为 `http://localhost:8080/v1`。
图片输入需将 `mmproj-Hy4-flash-bf16.gguf` 放在模型目录，并添加 `--vision on`。
