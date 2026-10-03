# C组事实清单（1002）

仅取证；状态评价的是该条的证据覆盖，不把媒体提升为L1。逐字摘录按页面渲染文字，换行合并不改变字词。截图DPR2，宽上限按任务指定模板的CSS像素口径（最大1400 CSS px对应2800物理px，常用1100 CSS px对应2200物理px；部分正文裁切700CSS对应1400物理px）。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 博文标题、日期、开源与leader原话 | L1 | https://blog.cloudflare.com/clef-decision-models/ | Introducing Clef: our open-source decision models, and new RL fine-tuning platform / Clef is currently the leader when evaluated against the Jev Decision Index / We’re fully open-sourcing these models on Hugging Face under an Apache 2.0 license for you to run locally and experiment with yourselves. | 16-clef-title.png；17-clef-release.png | 北京2026-10-01 23:34:02.111（原2026-10-01T15:34:02.111Z）；更新北京10-02 00:15:44.524（原10-01T16:15:44.524Z）。官方自述，不是独立认证。HTML与browser-tables均存档。 | 已找到 |
| C2 全部质量对比表、运行方 | L1 | https://blog.cloudflare.com/clef-decision-models/ ; https://clef-evals.workers-ai-mle.workers.dev/ | We shortlisted some evaluations below that are important for decision-making as defined by the Jev Decision Index and scored some of the more popular models on the market for it. / All non-Cloudflare results, the benchmark suite and the scoring formula come from the original Jev Decision Index on Hugging Face, maintained by multimodalart and contributors. | 18-clef-benchmark-table.png；19-clef-latency-table.png；27-clef-methodology.png | 官方页面截图：10行×6列质量表全截，另4行×3列workflow全截；逐项抄录下附。重要限定：博客that we ran不能推断六款全部由CF重新运行；live demo明确非CF数据沿用社区快照9/28。版本0.2.1。 | 已找到 |
| C3 延迟全表及测试条件 | L1/L3 | https://blog.cloudflare.com/clef-decision-models/ ; https://clef-evals.workers-ai-mle.workers.dev/#methodology ; https://multimodalart-jev-decision-index.static.hf.space/methodology.html | Across the 43 eval benchmarks that we ran, our Clef models beat the decision models on latency (except for Laya which is very fast but trades off quality in the benchmarks above): / Their latency was measured on the authors' own serving stack, not the upstream reference hardware (1 x NVIDIA RTX PRO 6000), and is not directly comparable. / Jev is TypeSafe's hosted API: a network round-trip from our lab with one sequential HTTPS client on the same latency sample, not comparable to on-card latency. | 19-clef-latency-table.png；27-clef-methodology.png | Jev 524.1ms含HTTPS网络，上游单进程单请求、750条分层样本+10条不计时warm-up；开源复现RTX PRO6000。Clef 209.3/38.8ms自家serving stack，具体GPU/并发/是否含网络未找到一手说明。模型卡单H200仅Usage测试环境，不能移作延迟条件；Register41/85GB+单并发64k也是运行容量条件。 | 部分支持 |
| C4 社区榜归属、版本、数据量、排名 | L3 | https://huggingface.co/spaces/multimodalart/jev-decision-index ; https://multimodalart-jev-decision-index.static.hf.space/index.html | Benchmarks and news on various repros of TypeSafe's Jev / 70 open reproductions of TypeSafe Jev's "Decision Model" (zero-shot classification) on the same benchmark suite, requesting 120K decisions per model across 43 benchmarks. / Every model faces the same 120,340 requests. / Not affiliated with TypeSafe AI | 20-jev-space.png；21-jev-index.png | 作者multimodalart；Decision Index0.2.1；页面updated2026-09-28（官方未给时刻）；HF API最后修改北京10-02 09:22:48（01:22:48Z）。该上游榜未见Clef，Jev57.9第1。HTML meta description仍132,422，正文/方法120,340；两处皆存，不擅选一个消除差异。图20为Space README归属，图21为应用标题；榜单数据全文已存，但完整榜表截图多次CDP超时/拼块重复，缺合格配图；失败原图移存sources/c-failed-jev-ranking.png，不能用于发布。 | 已找到 |
| C5 权重、代码、数据、许可证与基座 | L1 | https://huggingface.co/Cloudflare/clef ; https://huggingface.co/Cloudflare/clef-flash | Clef is a 27B multimodal model that turns a state and a schema of typed questions into decisions. / There is no free-form text generation and no output parsing. / Released under the Apache-2.0 license, following the base model / This training leverages our own internal synthetic datasets permutating field orders, prompts, and schema structures. | 22-clef-license.png；23-clef-flash-card.png；28-clef-model-card.png | Clef基座Qwen3.8-27B，flash Qwen3.5-9B。公开safetensors权重、joint head、joint_schema_model.py推理/批处理、tokenizer与配置；仓库文件清单未见训练脚本/训练数据/完整评测脚本。两份LICENSE实际文本均Apache2.0；无额外模型用途限制见本次所查文件（不作法律解释）。有评测结果表与社区评测链接不等于公开训练过程。API sha与精确文件清单下附。 | 已找到 |
| C6 TypeSafe Jev定义与自述 | L1 | https://typesafe.ai/blog/introducing-system-one-models-and-jev | System One Model: a new class of frontier models built to make fast, structured decisions that software can use directly. / Think of Jev as a frontier-intelligence function call: unstructured state in, typed probabilistic decisions out. / Input tokens: $0.042 / MTok ($42 per billion tokens). / Output tokens: FREE (too cheap to meter). | 29-jev-pricing.png | 页面Sep15,2026，官方未给时刻；作者Diogo Almeida。介绍的是System One Model品类，不能把CF的decision model叫法当逐字标题。原页API early access；未找到开放权重链接或明确承诺。自述70–500ms、40–200倍在System One任务限定内；发布页称West Coast laptops测服务。 | 已找到 |
| C7 Workers AI与Jev价格 | L1 | https://developers.cloudflare.com/workers-ai/models/clef/ ; https://developers.cloudflare.com/workers-ai/models/clef-flash/ ; https://developers.cloudflare.com/workers-ai/platform/pricing/ ; https://typesafe.ai/blog/introducing-system-one-models-and-jev | $0.24 per M input tokens / $0.09 per M input tokens / Input tokens: $0.042 / MTok ($42 per billion tokens). | 24-clef-pricing.png；25-clef-flash-pricing.png；29-jev-pricing.png | 顺序Clef/flash/Jev，均每百万输入token美元。CF两卡未标发布/更新时间，抓取北京10-02 21:22:49–52，不是定价生效日；Jev发布日9/15无时刻。CF未列单独output价不等于自行补写免费；Jev明确FREE。 | 已找到 |
| C8 Cloudflare live demo可用性与一致性 | L1 | https://clef-evals.workers-ai-mle.workers.dev/ | Self-reported. Clef and Clef-flash were run by Cloudflare, the models' authors, and have not been reproduced by the upstream board. / Community results from Decision Index 0.2.1, snapshot 28 September 2026. Unofficial; not affiliated with the upstream maintainers or TypeSafe. Built 1 October 2026. | 26-clef-demo.png；27-clef-methodology.png | 成功加载榜单、图表、Benchmarks、Methodology；中间一次ERR网络错误已记录。榜Clef61.21第1，Jev57.91第2，flash57.07第5；两Clef36/38指标，未跑项计0。显示209/38.8与博客209.3/38.8按舍入相容；博文leader是Clef而非flash。非CF结果从上游导入。 | 已找到 |
| C9 The Register原文与署名消息 | L4 | https://www.theregister.com/ai-and-ml/2026/10/01/cloudflare-tries-to-outplay-jev-with-open-weight-clef-models/5300649 | To be fair to the competition, Cloudflare self-reported its own scores against the benchmark, and they have yet to be reproduced for ranking on the official Decision Index. / While described as “open source” in the announcement, Cloudflare AI Platform group product manager Michelle Chen confirmed to The Register that its training datasets aren’t public. | 无 | 标题Cloudflare tries to outplay Jev with open-weight Clef models；Brandon Vigliarolo。datetime/meta 2026-10-01T20:39:49Z=北京10-02 04:39:49；HTML显示21:39 UTC=北京05:39，browser显示20:39 UTC=北京04:39，内部不一致，均保留。价格与硬件原句下附；媒体采访仍L4。 | 部分支持 |
| C10 HN/Reddit热度与质疑 | L6 | https://news.ycombinator.com/item?id=49923692 ; https://news.ycombinator.com/item?id=49717558 ; https://www.reddit.com/r/LocalLLaMA/comments/1wv4zzi/clef_open_weights_decision_model_by_cloudflare/ | Pricing is $0.24/million input tokens which is ~6x compared to Jev. Clef-flash is at $0.09 which is way more competitive. | 无 | 北京10-02 21:22:53附近HN Clef549分195评；Jev1989分520评。Clef Reddit21:39:04为372票109评；Jev解释贴472/369不是首发帖。3条HN原评论附后，points=null，不能造得分或标高赞。X适配器Failed to fetch未读到；Reddit原帖回退成功。 | 部分支持 |
| C11 实际夸大样本 | L5 | https://ai-blog.cloud/tool/cloudflare-clef-open-source-decision-model/ ; https://aistify.com/cloudflare-clef-clef-flash-open-decision-models/ ; https://news.lavx.hu/zh-Hans/article/cloudflare-fa-bu-clef-jue-ce-mo-xing-ji-qiang-hua-xue-xi-wei-tiao-ping-tai | 27B 多模态模型只输出概率不写字，官方称在 43 项评测里比同类决策模型更快，同时上线自己的强化学习微调服务。 | 无 | AI潮汐10/02未给时刻；导语省except Laya但正文有限定，不能说整篇全面碾压。未找到指定“GPT级39ms”“全面碾压”“完全开源含训练数据”的可靠中英实际原文。3篇已抽查正文见d-c-extra.md，不凑数量。 | 未找到一手来源 |

## C2/C3 官方表格完整抄录

来源c-cloudflare.html三张table，解析结果c-benchmark-transcription.json；官方页面截图18、19，未删列/行，原页无另列表下注脚，紧邻解释段保留。

### 主质量表（10行×6款）

| Benchmark | Clef | Clef-flash | Jev | DiffusionGemma Jev | Kev 9B | Laya |
|---|---|---|---|---|---|---|
| BFCL · case exact | 98.47 | 98.76 | 95.75 | 96.52 | 94.51 | 38.13 |
| ToolRet · nDCG@10 | 69.19 | 66.43 | 65.28 | 61.21 | 64.26 | 12.69 |
| API-Bank · accuracy | 91.93 | 93.11 | 88.19 | 83.66 | 56.30 | 11.41 |
| Home appliances · case exact | 82.95 | 97.73 | 52.27 | 42.05 | 25.00 | 0.00 |
| When2Call · accuracy | 72.37 | 65.58 | 80.97 | 75.44 | 49.62 | 11.94 |
| BANKING77 · macro-F1 | 94.20 | 90.93 | 79.74 | 74.28 | 84.83 | 14.29 |
| CLINC150+OOS · macro-F1 | 97.43 | 66.77 | 89.27 | 83.49 | 79.03 | 3.19 |
| BRIGHT · nDCG@10 | 45.91 | 39.26 | 47.52 | 42.94 | 38.53 | 19.90 |
| Amazon ESCI · macro-F1 | 57.48 | 57.39 | 55.21 | 53.37 | 49.22 | 24.40 |
| PhishNChips · accuracy | 79.60 | 75.05 | 62.55 | 85.35 | 50.75 | 50.15 |

### 工作流表（4行×3款）

| Workflow | Clef | Clef-flash | Jev |
|---|---|---|---|
| Invoice processing | 64.7 | 57.1 | 61.8 |
| Customer service | 76.3 | 77 | 76.0 |
| Security incidents | 62.9 | 61.7 | 61.7 |
| Agent trace observability | 68.5 | 69.8 | 71.6 |

### 延迟表（2行×6款；ms）

| Benchmark | Clef | Clef-flash | Jev | DiffusionGemma Jev | Kev-9B | Laya |
|---|---|---|---|---|---|---|
| Median latency  · ms | 209.3 | 38.8 | 524.1 | 84.4 | 51.4 | 5.8 |
| p95 latency · ms | 238.6 | 122.4 | 536.0 | 211.2 | 187.9 | 222.5 |

## C5 文件与许可证证据

- Cloudflare/clef；sha 2f3de3dd85f379784083b0814d997ab627200f0c；lastModified 2026-10-01T15:23:46.000Z（UTC；北京+8小时）。文件：.gitattributes, LICENSE, README.md, chat_template.jinja, config.json, generation_config.json, joint_head.safetensors, joint_head_config.json, joint_schema_model.py, model-00001-of-00012.safetensors, model-00002-of-00012.safetensors, model-00003-of-00012.safetensors, model-00004-of-00012.safetensors, model-00005-of-00012.safetensors, model-00006-of-00012.safetensors, model-00007-of-00012.safetensors, model-00008-of-00012.safetensors, model-00009-of-00012.safetensors, model-00010-of-00012.safetensors, model-00011-of-00012.safetensors, model-00012-of-00012.safetensors, model.safetensors.index.json, processor_config.json, tokenizer.json, tokenizer_config.json
- Cloudflare/clef-flash；sha 17f0b0ad64efb65d273590632833508766b2aae6；lastModified 2026-10-01T15:23:49.000Z（UTC；北京+8小时）。文件：.gitattributes, LICENSE, README.md, chat_template.jinja, config.json, generation_config.json, joint_head.safetensors, joint_head_config.json, joint_schema_model.py, model-00001-of-00004.safetensors, model-00002-of-00004.safetensors, model-00003-of-00004.safetensors, model-00004-of-00004.safetensors, model.safetensors.index.json, processor_config.json, tokenizer.json, tokenizer_config.json

两份LICENSE从原站raw路径HTTP200存于c-clef-license-text.html、c-flash-license-text.html（扩展名是通用抓取器默认，内容为许可证纯文本），同时有blob页面浏览器存档。未下载巨型权重，文件存在性依据HF API列表；未运行模型或复现训练/评测。

## C10 HN质疑原句（仅L6）

- https://news.ycombinator.com/item?id=49924252；作者ssiddharth；得分：API points=null（未公开，非0）；原文：

> Pricing is $0.24/million input tokens which is ~6x compared to Jev. Clef-flash is at $0.09 which is way more competitive.

- https://news.ycombinator.com/item?id=49924502；作者buildbuildbuild；得分：API points=null（未公开，非0）；原文：

> Open weights, not open source.
> The weights have permissive licensing, but the data and training pipeline are not published to reproduce them from their proprietary Qwen starting points. Weights are not "source."

- https://news.ycombinator.com/item?id=49928781；作者pdlug；得分：API points=null（未公开，非0）；原文：

> I love what Cloudflare is doing generally so I was excited to try Clef in my evals on a real task vs Jev: should an agent's knowledge-base write go to human review?
> Quality: close (recall 0.98 vs 1.00)
> Hosted p50: Clef ~850ms, Jev ~110ms
> Clef-flash: over-escalates
> Data + script:
> https://github.com/nicia-ai/admission-decision-eval

## C9 补充摘句

> For those that would prefer not to pay the token cost (Clef costs $0.24 per million tokens - nearly six times the price of Jev at $0.042/M), Clef can also be downloaded from Hugging Face, and is open weight under the same Apache-2.0 terms as Qwen.

> As for whether your hardware can run it, Chen told us that Clef-flash will run on any GPU with at least 41 GB of VRAM, while Clef requires 85 GB of VRAM on a GPU for it to function.

> “This is assuming single concurrency and a 64k context window,” Chen added.

## 可能的吠点

- Clef61.21高于Jev57.91；flash57.07低于Jev，自家榜首不属于38.8ms款（c-demo-browser.json）。
- 主表When2Call/BRIGHT均Jev高于两Clef；CLINC150+OOS中flash66.77低于Jev89.27，不能称全面领先。
- 博文声称beat the decision models on latency但表中完整Clef209.3慢于DiffusionGemma84.4和Kev51.4，例外不只Laya；应区分完整与flash。
- CF demo明确其他模型沿用社区数据；不能把that we ran扩展成全部模型同机同环境重跑。
- Jev HTTPS网络往返与开源模型片上延迟不可直接等同；Clef具体延迟硬件和网络边界仍缺。
- 模型权重/推理代码Apache2.0与可完整复现训练是两件具体事实；训练数据未公开的明示来自Register采访，文件清单未见数据只能作本次范围内观察。
- 社区页meta132,422与正文120,340并存，未找到页面对该差异的直接说明，不擅自解释成相同指标。

## 夸大说法实例

本组未找到用户指定强夸大例；AI潮汐摘要省略Laya例外是较弱候选，正文有反向限定。详见d-c-extra.md。

## 扫描说法勘误

- “Jev Decision Index 是 Cloudflare 自家基准”：与本次原站存档不符；Space作者multimodalart，上游声明不隶属TypeSafe，CF自家demo另站。
- “对比表数字都是 Cloudflare 自跑”：需要进一步限定。Clef两款自跑已证实，但live demo明确所有非CF模型数据直接沿用上游。
- “38.8ms是完整Clef/所有维度领先”：与两张表不符；完整Clef209.3ms，flash38.8ms。

## 未覆盖/失败与假设

不将模型卡单H200使用验证当benchmark条件；不下载权重、不收费调用推理。X/Reddit适配器失败均记录，未修改适配器或登录；Reddit公开原帖补抓成功。未找到Jev Reddit发布首帖，说明贴不混称首发。普通HTTP一轮TLS EOF后原站浏览器/重试成功。部分手工截图日志以文件mtime补记并标记。
