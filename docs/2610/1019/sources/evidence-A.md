# 1019 期 A 组取证：DeepSeek 开源 DeepInfer（“全面开源”、README 不含项、155 万亿 token 与每卡每小时 38 美金）

执行方：claude-sonnet-5-5。时刻均为北京时间（UTC+8），括号保留原时区。本机为美东（EDT），读取日志以 `barking` 写入的北京时间为准。交卷时间已超 60 分钟，未完成项见末节。

## 1. 要点（先读）

1. “全面开源”是官方原话：DeepSeek 官方公众号文章写 “我们决定进一步将内部推理框架 DeepInfer 全面开源”，官方 X 帖写 “now fully open source!”。知乎技术报告同日同主体，却写 “今天，我们正式开源”，并明说 KVCache 管理与负载均衡组件“目前尚未完整开源”；README 写 “not included in this release”。官方措辞在三处不一致，这是核心吠点。
2. “155 万亿 Tokens/天”“每卡每小时 38 美金（按谷价）”确实在知乎原文（DeepSeek 已认证机构号）里，且只在正文出现一次；图 3 的脚注多了限定：“实际部署 P:D 接近 1:1，不考虑波峰波谷，整体收入是平均每卡每小时 38 美金（按谷价）”，图头写 “机器已经预留了一定冗余度，以应对流量的波动”。报告没有解释“谷价”，没有给卡型号、卡数与集群数。
3. 官方价格页用词是“空闲时段 / 高峰时段”，空闲价为高峰价的一半；全页不含“谷”字（见 A5）。报告里的“谷价”与价格页的“空闲时段价格”是否同一回事，**官方未写**，不推断。
4. 扫描说法勘误：知乎两节“统计数据”并非空表，是两张图（线上实测表、离线 Pareto 图），已截。

## 2. 证据表

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1-1 | DeepInfer 仅提供离线推理 | L1 | https://github.com/deepseek-ai/DeepInfer（raw：https://raw.githubusercontent.com/deepseek-ai/DeepInfer/main/README.md，存 `a-readme-raw.txt`、`a-repo.html/.txt`） | “This open-source release provides offline inference with the core features needed for serving at scale.” | `images/01-readme-scope-not-included.png` | README 读取 2026-10-20 19:12（北京；截图约 20:35） | 已找到 |
| A1-2 | 只支持一个模型 | L1 | 同上 | “Supported models: - DeepSeek-V4.1-Flash” | 01 | | 已找到 |
| A1-3 | 硬件范围 | L1 | 同上 | “Supported hardware: - NVIDIA SM100 GPUs - Huawei Ascend 950 NPUs” | 01 | 构建前提另写：Linux、GCC >= 13.0、Python 3.12、CUDA 13（NVIDIA）、CANN 9.2（Ascend） | 已找到 |
| A1-4 | 完整推理服务所需组件未随本次发布 | L1 | 同上 | “Required for a complete inference service (not included in this release):” 其下三条：“A KV cache system”；“VE/Prefill/Decode disaggregation (most required engine functionality is already implemented)”；“Load balancing across data-parallel replicas for both prefill and decode” | 01 | 括号内 “most required engine functionality is already implemented” 是 README 原有限定 | 已找到 |
| A1-5 | 基于 vLLM | L1 | 同上 | “DeepInfer was originally built primarily on vLLM, with additional inspiration from SGLang and TensorRT-LLM. We thank their developers.” | 未截（见未完成项） | Acknowledgments 一节 | 已找到（无图） |
| A2-1 | 许可证 | L1 | https://api.github.com/repos/deepseek-ai/DeepInfer（存 `a-gh-repo.json`、`a-gh-license.json`） | README：“DeepInfer is released under the Apache License 2.0.”；API `license.spdx_id` = Apache-2.0 | — | | 已找到 |
| A2-2 | 时间线（只记不推断“公开时间”） | L1 | 同上 + `.../commits`（`a-gh-commits.json`） | 仓库 `created_at` 2026-10-16T07:47:23Z（北京 10-16 15:47:23）；首个提交 “initial commit” 2026-10-20T04:09:42Z（北京 12:09:42）；末个提交 “update Q&A EN doc” 2026-10-20T11:03:13Z（北京 19:03:13）；`pushed_at` 11:03:36Z | — | 提交共 3 条（读取 19:12）；提交时间可由作者设置，不等于公开时间 | 部分支持（官方未写首次公开时刻） |
| A2-3 | release / tag | L1 | `.../releases`、`.../tags`（`a-gh-releases.json`、`a-gh-tags.json`） | 两个接口均返回空数组 `[]` | — | 读取 19:56 前后 | 已找到（无 release、无 tag） |
| A2-4 | 星标 / fork | L1 | `a-gh-repo.json` | 读取约 19:56（北京）：stargazers_count 6002、forks_count 470、open_issues_count 50；此前 19:12 前后读取为 4478 / 345 以下（扫描值，未存档） | — | `updated_at` 2026-10-20T11:57:11Z；星标数随时间增长，以交稿时读取为准 | 已找到 |
| A2-5 | eval/README.md 内容 | L1 | https://raw.githubusercontent.com/deepseek-ai/DeepInfer/main/eval/README.md（`a-raw-eval-README.md`） | “Offline batch evaluation for AIME 2026, HLE, and GPQA Diamond, built on the DeepInfer offline batch API.”；“The reported metric is **Pass@1**.”；第 7 节 “Reference results”：“The following Pass@1 values are reference figures for the V4.1 model under the default configuration described above.” AIME 2026 99.9%、GPQA Diamond 93.4%、HLE-Full 42.6%；“These figures are therefore indicative rather than exact, and serve as a sanity check that a deployment is configured correctly.” | — | 是评测脚本与“参考值”，用于校验部署；README 未写由谁、何时、哪个 effort 跑出；AIME 每题 32 样本取均值，GPQA 每题 8 样本，HLE 单样本 | 已找到 |
| A3-1 | 官方公众号公告原文 | L1 | 公众号 DeepSeek 文章《DeepSeek 开源推理框架：走向真正普惠的未来》（经搜狗微信跳转到 mp.weixin.qq.com，带签名的临时链接，无稳定短链；正文存 `a-wx-official.txt`/`.json`） | “在此前开源一系列大模型、TileLang 编译工具及推理算子库的基础上，我们决定进一步将内部推理框架 DeepInfer 全面开源。”；“借助全面开源的模型、算子库与推理框架”；文内文末链接：“DeepSeek 知乎技术报告：https://zhuanlan.zhihu.com/p/2095560295930508953” | 未截 | 页面 `ct`=1792486494，即 2026-10-20 08:54:54 UTC = **北京 16:54:54**；页面显示 “2026年10月20日 04:54” 是本机时区，不是北京时间；账号名 “DeepSeek” | 已找到（“全面开源”是官方原话） |
| A3-2 | 官方 X 帖 | L1 | https://x.com/deepseek_ai/status/2112476496283775135（fxtwitter 存 `a-fx-2112476496283775135.raw.json`；2/4、3/4、4/4 同存） | 1/4：“Introducing DeepInfer, our in-house inference framework, now fully open source!”；2/4：“Inference code for future DeepSeek models and ongoing optimizations will land here too.”；4/4：“Open models. Open kernels. Open inference.” | 未截 | 2026-10-20 09:30:19 UTC = **北京 17:30:19**；读取时 9,305 赞 / 1,058 转 / 485 回复 / 约 41.4 万浏览 | 已找到 |
| A3-3 | api-docs 新闻页是否有公告 | L1 | https://api-docs.deepseek.com/news/news261017（10-17 模型发布新闻）；`/updates` 302 | 本次检索未发现 DeepInfer 公告条目；`a-news.html/.txt` 是文档首页（URL 猜错），**不是**任何说法的来源 | — | | 未找到一手来源（api-docs 新闻页未见） |
| A3-4 | 谁先把它写成“全面开源”的媒体 | L4/L5 | IT之家 https://www.ithome.com/1/015/429.htm（`a-ithome.*`）；新浪转 IT之家 17:05；观察者网经网易 17:43（`a-163-1.*`）；21财经 17:55 注 “来源：DeepSeek官方微信”（`a-21jingji.*`） | IT之家：“DeepSeek 今日官宣，……决定进一步将内部推理框架 DeepInfer 全面开源。”；观察者网引号内：“在此前开源一系列大模型、TileLang编译工具及推理算子库的基础上，我们决定进一步将内部推理框架DeepInfer全面开源。” | — | 媒体页显示时刻（北京）：新浪 17:05、网易 17:43、21财经 17:55；均晚于官方公众号 16:54 | 已找到 |
| A4-1 | 作者账号 | L1 | https://zhuanlan.zhihu.com/p/2095560295930508953（curl 403；opencli 浏览器读取；`a-zhihu-dom.txt`、`a-zhihu-api.json`） | 作者 “DeepSeek”，徽标 “已认证机构号”，`user_type` = organization；标题 “DeepSeek V4.1 Flash 大规模推理部署实践”；公众号文章链回此文（见 A3-1） | `images/02-...png` 不含作者栏 | | 已找到 |
| A4-2 | 发布时刻 | L1 | 同上 | `datePublished` = 2026-10-20T08:49:27Z；API `created` = 1792486167（同值）= **北京 16:49:27**；页面显示 “发布于 2026-10-20 04:49・北京” 是本机美东时区值，“北京”是 IP 属地标签 | — | 比公众号早约 5 分钟；`updated` 与 `created` 相同（读取时无修改记录） | 已找到 |
| A4-3 | 155 万亿 / 38 美金 原段 | L1 | 同上 | “此前，我们已开源用于开发高性能算子的 TileLang 以及一系列推理算子库。今天，我们正式开源大规模云服务推理框架 DeepInfer。 基于这一框架，我们已实现 DeepSeek V4.1 Flash 的高吞吐、低延迟大规模推理服务。目前单一集群的服务吞吐是 155 万亿 Tokens/天，整体收入是每卡每小时 38 美金（按谷价）。”；紧接下一段：“……同时，我们还开源了生产级 Token 编解码组件 deepseek-recipe……未来，DeepSeek 新模型的推理代码及后续性能优化也将在 DeepInfer 中持续开源。”；再下一段：“推理系统中的 KVCache 管理与负载均衡组件因与内部基础设施耦合较深，相关代码目前尚未完整开源。本文将从上述设计目标出发，介绍 DeepSeek V4.1 Flash 推理系统的整体架构与关键实现，以及这两部分的设计细节。” | `images/02-zhihu-155t-38usd.png`（含上文、数字段与“尚未完整开源”段） | 正文用词：“今天，我们正式开源”（无“全面”）；“单一集群”；“整体收入”（非“利润”“赚”）；“谷”字在正文只出现这一次 | 已找到 |
| A4-4 | 卡型号、卡数、集群数 | L1 | 同上 | 全文未给 GPU 型号、卡数、集群数；只写：“用常见的 8 卡机器做 Prefill，用常见的 64～128 卡超节点做 Decode”；并行策略 “Prefill: EP2～8/DP2～8”“Decode: EP64~128/DP64~128 (Wide-EP)”。线上表给每卡吞吐（见 A4-6） | 02、03 | 快科技也写：“DeepSeek没有公布他们具体有多少集群，也没有明确的显卡数量”（a-mydrivers.txt） | 已找到（未给） |
| A4-5 | “线上系统实际统计数据”“离线性能基准数据”两节是否为空 | L1 | 同上 | 两节各含一张图，不是空表；页面纯文本抽取只有小标题，所以文本抓取看起来为空（扫描误判的原因） | `images/03-zhihu-online-stats-table.png`；`images/04-zhihu-offline-pareto.png` | 图 URL（知乎 CDN，防盗链，直接 curl 得 HTML）：线上表 `https://pic1.zhimg.com/v2-bff36102d7ffd58ffca11ad64738012a_1440w.jpg`，离线图 `https://picx.zhimg.com/v2-c1c07ce013c2dc0b0a3864c566bad801_1440w.jpg` | 与说法不符（并非空表） |
| A4-6 | 线上表（图 3）内容与脚注 | L1 | 同上 | 标题 “实际线上运行数据”，副标题 “机器已经预留了一定冗余度，以应对流量的波动”。prefill：每卡吞吐（不含缓存命中）60K tps；每卡吞吐（含缓存命中）1.5M tps；平均输入缓存命中率 96%；首字时延（TTFT）平均 400 ms；TTFT P99 1000 ms；平均输入长度 77K tokens。decode：每卡吞吐 14K tps；吐字速度 230 tps（代码场景 300 tps）。kvcache 缓存：不活跃后保存时间 72 小时。脚注：“实际部署 P:D 接近 1:1，不考虑波峰波谷，整体收入是平均每卡每小时 38 美金（按谷价）” | 03 | 图中单位按原图写；图右下有知乎水印 “@DeepSeek”；脚注是 38 美金的唯一定义性文字 | 已找到 |
| A4-7 | 离线图（图 4）条件 | L1 | 同上 | Prefill：“Non-cached Input Throughput vs. Latency”，“EP4DP4 \| 384+32 experts \| context 131,072 \| Decoder Bounded Replay (140 tokens)”，点标 prefill_batch_size（tokens per DP rank）4,096 / 8,192 / 16,384 / 32,768 / 65,536。Decode：“Output Throughput vs. Interactivity”，“EP64DP64 \| 384+64 experts \| context 131,072 \| nextn=3 fixed \| per-token acceptance 75% → τ=3.25”，点标 decode_num_requests 8–256 | 04 | 与 README “Measured on 4 SM100 GPUs for prefill and 64 for decode — each within one scale-up domain” 并读（见 A7）；图未标卡型号 | 已找到 |
| A5-1 | 报告是否解释“谷价” | L1 | 知乎全文（`a-zhihu-dom.txt`、`a-zhihu-api.json`）与图 3 | 正文与图脚注都只写“（按谷价）”，全文无定义、无换算、无时段说明 | 02、03 | 搜索 “谷” 仅正文 1 处 + 图脚注 1 处 | 报告未解释 |
| A5-2 | 官方价格页的时段价 | L1 | https://api-docs.deepseek.com/zh-cn/quick_start/pricing（`a-pricing.*`）；英文页 https://api-docs.deepseek.com/quick_start/pricing（`a-pricing-en.*`） | 中文：“空闲时段价格为高峰时段价格的一半。北京时间周一至周五（不含中国法定节假日）9:00 - 12:00、14:00 - 18:00 为高峰时段；其余时段，包括周末及中国法定节假日全天均为空闲时段。”价格（deepseek-flash，元/百万 tokens）：输入缓存命中 空闲 0.02 / 高峰 0.04；输入缓存未命中 空闲 1 / 高峰 2；输出 空闲 4 / 高峰 8。deepseek-v4-pro：命中 0.15/0.30；未命中 4.5/9.0；输出 13.5/27.0。英文页：“Off-peak rates are half of the peak rates. Peak hours are 01:00 - 04:00 and 06:00 - 10:00 UTC, Monday through Friday, excluding Chinese public holidays. All other hours are off-peak, including weekends and Chinese public holidays in full.”（即北京 9:00–12:00、14:00–18:00）；英文页 “1M OUTPUT TOKENS” 一行：OFF-PEAK $0.6 / $1.98，PEAK $1.2 / $3.96（两列分别对应两个模型，列头以存档为准） | — | 价格页读取 19:12（北京）；价格页用词“空闲时段/高峰时段”，全页不含“谷”字；页面写 “deepseek-flash” 对应 “DeepSeek-V4.1-Flash-1017”，并发限制 2500 | 已找到（存在空闲/高峰分档；与“谷价”的对应关系官方未写） |
| A5-3 | HN 评论者把“谷价”译成 off-peak | L6 | HN 评论 50163494（schneehertz，2026-10-20 09:34:19 UTC）与 50164097（GreenWatermelon，10:36:21 UTC）；HN 不显示评论分数 | “the overall revenue is $38 per card per hour (at off-peak pricing)” | — | 评论者自己的译法，非官方；知乎热评 “波谷35美金/h”（115 赞，读取约 19:15 北京）与原文 38 不符，疑为笔误或记忆偏差 | 线索（L6，不可引用） |
| A6-1 | deepseek-recipe README | L1 | https://github.com/deepseek-ai/deepseek-recipe（`a-recipe-repo.*`、`a-recipe-readme-raw.txt`） | “…connect an inference backend to API services that support multiple formats. The caller must provide model inference, tool execution, and HTTP transport.”；表格行 “server-rs … An Axum API example with mock inference.”“server-py … A FastAPI example with mock inference.”；“Project code and public documentation are licensed under the MIT License.” | 未截 | 仓库 `created_at` 2026-09-10T05:36:48Z，`pushed_at` 2026-10-19T17:51:18Z（北京 10-20 01:51），共 4 个提交；读取时 1086 星、92 fork（API，19:5x） | 已找到 |
| A7-1 | README 两张 Pareto 图与测量条件 | L1 | `doc/images/prefill_pareto.png`、`doc/images/decode_pareto.png`（raw，存 `a-raw-doc-images-*.png`，已下载未截） | “Measured on 4 SM100 GPUs for prefill and 64 for decode — each within one scale-up domain. To reproduce the benchmarks, see run_sweep.sh.” | 原图已存本地，尚未打开核对、尚未入 `images/` | 知乎离线图（04）与 README 图是否同一批数据未核 | 部分支持 |
| A8-1 | 中文夸大标题：快科技 | L5 | https://m.mydrivers.com/newsview/1159039.html（`a-mydrivers.*`） | 标题：“DeepSeek把核心商业秘密也开源了：每张卡每小时能赚38美元”；正文：“今天他们又把核心的商业机密都公开了。”“每张卡每小时38美元的收入也证明了他们的收入。”“不过DeepSeek没有公布他们具体有多少集群，也没有明确的显卡数量，因此现在还不能估算他们每天的真实收入和Token吞吐量。” | 未截 | 页面时刻 2026年10月20日 17:53（北京）；作者署名 “宪瑞”；原文是“整体收入”，标题改成“赚”并加“核心商业秘密”，正文又写“还不能估算” | 已找到 |
| A8-2 | 中文“全面开源”标题（多为转述官方措辞） | L4/L5 | 新浪科技（转 IT之家）https://finance.sina.com.cn/tech/digi/2026-10-20/doc-inivwitm5770225.shtml；网易×2；80aj | “DeepSeek 宣布将内部推理框架 DeepInfer 全面开源，目标实现大模型普惠”（新浪 10-20 17:05）；“DeepSeek全面开源推理框架DeepInfer，兼容英伟达与华为昇腾”（80aj） | — | 这些标题与官方公众号措辞一致，不算夸大；只能说“全面”二字官方自己用了 | 已找到 |
| A8-3 | 英文夸大/限定对照 | L5 | https://runtimewire.com/article/deepseek-opens-deepinfer-inference-engine（`a-runtimewire.*`） | “DeepInfer targets large-scale serving and supports DeepSeek-V4.1-Flash, but its README says a complete inference service requires components the project does not include.”（这是限定而非夸大，供对照） | — | | 已找到 |
| A9-1 | HN 热度 | L5 | https://news.ycombinator.com/item?id=50163277（`a-hn-50163277.json`、`a-heat.json`） | 标题 “DeepSeek just open-sourced its inference engine”；读取时 551 分、含嵌套评论 168 条 | — | 读取时刻 2026-10-20 19:58:57（北京）；发帖 09:08:54 UTC（北京 17:08:54）；Claude 此前 19:xx 读到 237 分 / 79 评 | 已找到 |
| A9-2 | HN 有理有据的评论（L6，仅作线索） | L6 | 评论 50163850（_aavaa_）；50164217（rektlessness）；50164052（xrisk） | “Notably missing their kv cache system”；“Deepseek released just enough to stir the pot but not enough to cook your own from scratch. Wake me up when we can clone, point it at a dozen H100s we own, and get a working production service.”；“For comparison, rental prices seem to be ~$2.75/ per H-100-hour [1]. Meanwhile, Deepseek says \"38 USD per hour per card”. They haven’t specified which card as far as I can tell.” | — | HN 不公开评论分数；以上三条均对应官方自己写明的限定或未给项 | 已找到 |

## 3. 可能的吠点（均须回到一手来源再用）

1. 官方三处措辞不一致：公众号与 X 写“全面开源 / fully open source”，知乎报告写“正式开源”并承认 KVCache 管理与负载均衡“尚未完整开源”，README 写 “offline inference” 与 “not included in this release”（A3、A4-3、A1-4）。
2. README 只支持 DeepSeek-V4.1-Flash 一个模型，硬件限 “NVIDIA SM100 GPUs”“Huawei Ascend 950 NPUs”（A1）。
3. 38 美金是“整体收入”不是利润，是“平均”值，“不考虑波峰波谷”，“按谷价”，“机器已经预留了一定冗余度”（A4-6）；报告未给卡型号、卡数、集群数（A4-4）。
4. 155 万亿是“单一集群”的“服务吞吐”（Tokens/天），不是全站总量，也未说明是否含缓存命中的输入 token（图 3 同时列了含/不含缓存命中的每卡吞吐 1.5M / 60K tps，平均缓存命中率 96%）。这一条是对原文的口径提示，不是推断。
5. README 性能图 “Measured on 4 SM100 GPUs for prefill and 64 for decode”，而线上描述为 “Prefill: EP2～8/DP2～8”“Decode: EP64~128”（不同配置，勿混比）。
6. eval/README 的 AIME 2026 99.9%、GPQA Diamond 93.4%、HLE-Full 42.6% 是“参考值，用于校验部署”，且没写 effort、日期与运行方式（A2-5）。
7. README 致谢写 “originally built primarily on vLLM”。

## 4. 夸大说法实例

- 快科技（A8-1）：“DeepSeek把核心商业秘密也开源了：每张卡每小时能赚38美元”，2026-10-20 17:53（北京），https://m.mydrivers.com/newsview/1159039.html；同篇正文自己写了“还不能估算他们每天的真实收入和Token吞吐量”。
- “全面开源”类标题（新浪、网易、80aj）与官方措辞一致，不属于夸大；其中未带限定的转述（不提 KVCache 与负载均衡未开）可作对照。
- 英文：RuntimeWire 的标题 “DeepSeek opens its inference engine, leaving service plumbing to developers” 已带限定；未找到把 38 美元写成“利润”的英文标题（未穷尽搜索）。
- 未找到：中文“每卡赚 38 美元”的“利润”字样标题（只在快科技见“赚”“收入”）。

## 5. 扫描说法勘误

- “两节统计表在公开正文中无内容”：错。是两张图（线上实测表、离线 Pareto 图），文本抽取看不到，已截 03、04。
- “公众号原文称全面开源”：对，官方公众号（北京 16:54:54）原话含“全面开源”；WorkBuddy 的“16:54”正确，页面上显示的 04:54 是本机美东时间。
- “知乎页面显示 04:49 北京”：是本机时区，实际 16:49:27（北京）。
- WorkBuddy 把“谷价”理解为低谷电价：报告未解释，价格页没有“谷”字；不采信，也不采信 HN 的 off-peak 译法。
- “GitHub 创建 2026-10-16T07:47:23Z”属实，但首个提交是 2026-10-20T04:09:42Z（北京 12:09:42），早于公告 4 小时 40 分；仓库创建时间不等于公开时间。
- 星标数：19:12 前后 4478，20:2x 已 6002，随时间增长。

## 6. 未完成项（超时，已按优先级收口）

- A1 另一张截图：Acknowledgments 的 vLLM 句（文字已摘，无图）。
- A7：README 两张 Pareto 图原图只下载、未目视核对、未入 `images/`（本地：`sources/a-raw-doc-images-*.png`，PNG 不入库）。
- A8：快科技标题页未截图（候选图 04 号位已被知乎离线图占用，快科技截图如需要请用 05–12 号位，我未做）。
- 公众号文章与官方 X 帖未截图（文字与 JSON 已存）。
- A9：只做了 HN；Reddit 与中文社区（知乎评论、LINUX DO 被 Cloudflare 拦截）未系统取，知乎热评只摘 2 条记于 A5-3。
- 英文价格页的两列模型名与价格对应关系我只摘了数字行，未逐列核；以 `a-pricing-en.txt` 为准。
