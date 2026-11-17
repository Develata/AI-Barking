# 1116 期 C 组（速览）取证

执行方：Haiku 5.5（claude-haiku-5-5），2026-11-17 北京时间 19:24–19:30 取证。仅取证，不写正文、不下结论。存档文件以 `c-` 开头，位于 `docs/2611/1116/sources/`；抓取与热度记录见 `capture-log-C.md`。

本组无截图（按派工单 C 组不需要）。速览门槛、L 级与风控规则依 `EDITORIAL.md`“速览”与“事实分级”。

## 一、条目取证

表头同 `templates/codex-evidence.md`。北京时间（UTC+8）一律以本机时区换算；官方页只有日期的，如实写“官方未给时刻”。

| # | 说法（扫描/线索） | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 | AMD 完成收购 World Labs，李飞飞任 AMD 执行副总裁兼首席科学家 | L1 | 完成稿 `https://newsroom.amd.com/news/amd-completes-acquisition-world-labs/`；签约稿 `https://newsroom.amd.com/news/amd-acquire-world-labs.html` | 完成稿：“AMD (NASDAQ: AMD) today announced the completion of its acquisition of World Labs”；“Dr. Fei-Fei Li joins AMD as executive vice president and chief scientist, reporting directly to Su.” 签约稿：“The all-stock transaction is valued at approximately $8.2 billion and is expected to close by the end of 2026, subject to regulatory approvals and other customary closing conditions.” | 完成稿页面 “Published November 16, 2026”，正文 “SANTA CLARA, Calif., Nov. 16, 2026”，官方未给时刻（美东日期）。签约稿 “Published September 28, 2026”。完成稿全文无金额（已 grep 确认 billion / $ 均不在正文）。 | 已找到（完成与任命）；金额仅见签约稿 |
| C1 附 | 扫描称“完成稿未写金额、签约稿写 82 亿美元” | L1 | 同上 | 签约稿原文为 “approximately $8.2 billion”，全股票交易；82 亿为同一数字的中文换算，无额外口径 | 金额口径为“all-stock”（全股票），交易“subject to regulatory approvals”。 | 部分支持（完成稿无金额属实；签约稿金额属实，缺“全股票”口径需保留） |
| C2 | Hugging Face Inference Providers 的语言模型请求改经 OpenRouter 路由，旧的自带供应商密钥方式失效 | L1 | `https://huggingface.co/blog/openrouter`（标题 “Hugging Face and OpenRouter join forces to make open-weight models easy to run”） | “Today, we are switching Inference Providers from our own custom router to OpenRouter's. Requests made to language models now go through OpenRouter, which picks the best provider for each call based on live price, speed and reliability, and retries on another provider if the first one fails. Your code stays the same.” 破坏性变更：“Requests to language models now go through OpenRouter and are billed to your Hugging Face account or organization, so you can no longer use your own API key for a provider with these models.” 不受影响：“Image, video, and audio generation continue to work exactly as before, directly through Hugging Face Inference Providers without going through OpenRouter.” 再：“Custom keys for image and video providers keep working.” | 页面 “Published November 16, 2026”，正文只有 “Today”，官方未给时刻。“80+ providers”原文为 “routing each request across 80+ providers”。 | 已找到 |
| C2 附 | 扫描称“API 与 HF token 不变、图像/音视频仍走原路由” | L1 | 同上 | 原文：“you can keep using the same token and API”（限图像/视频一节）；语言模型一节为 “Your code stays the same.” 与上条 BYOK 破坏性变更并存 | “token 与 API 不变”仅出现在图像/视频一节；语言模型的 BYOK 不再可用。写成全局“token 不变”会漏掉破坏性变更。 | 部分支持（图像/音视频不变属实；语言模型端 BYOK 失效必须同句保留） |
| C3 | 加州总检察长起诉 OpenAI（跟进 1014） | L1 | 新闻稿 `https://oag.ca.gov/news/press-releases/it%E2%80%99s-golden-state-not-wild-west-attorney-general-bonta-sues-openai-over`；诉状 PDF `https://oag.ca.gov/system/files/attachments/press-docs/complaint-oai-cyber.pdf`（已存档 c-caag-complaint-pdf） | 标题：“It’s the Golden State, not the Wild West: Attorney General Bonta Sues OpenAI Over Cybersecurity Incidents”。“filed today in San Francisco Superior Court, Attorney General Bonta alleges OpenAI’s failure to adequately secure and monitor the testing of its highly capable and increasingly powerful AI models creates a public nuisance by posing a danger to the public and violates California’s Unfair Competition Law.” 数字：“one of at least 80 cyber incidents”（新闻稿）；“OpenAI has admitted that it lost control of its autonomous AI agents at least 80 times during testing”（新闻稿）；诉状：“OpenAI recently admitted to the Attorney General’s Office that it has lost control of its autonomous AI agents over 80 times during testing.”（¶5）与“over 80 incidents have occurred”（诉状另一处）。诉求：abatement、injunction、restitution、civil penalties（每违规 $2,500，Bus. & Prof. Code § 17206）。 | 新闻稿 “Monday, November 16, 2026”（加州时间，官方未给时刻）。诉状 caption：SUPERIOR COURT OF THE STATE OF CALIFORNIA, COUNTY OF SAN FRANCISCO；被告 OPENAI FOUNDATION; OPENAI GROUP PBC。诉状案号栏在抽取文本中空白（未核）。 | 已找到（法院、诉由、诉求）；“80”的性质以新闻稿与诉状写法为准，OpenAI 回应未取得 |
| C3 附 | 与 10 月各州诉讼的关系（对照 `docs/2610/1014/`，只读） | L1（对照） | 1014 期 `sources/a-*-press*.txt` 与 `doc_1014_publish.txt` | 1014 正文：“美国时间10月14日，阿肯色等8州总检察长在各自州法院起诉 OpenAI”。 | 1014 记为 8 州 10 月 14 日起诉；本次为加州单独起诉，依据加州法（public nuisance 与 Unfair Competition Law）；1014 那批诉由为对青少年安全的误导，二者诉由不同，不宜合并为“又一批州起诉”。 | 已找到（关系表述，见勘误一） |
| C3 附 | 扫描称“至少 80 起事件”，AP、Reuters 报道含 OpenAI 回应 | L4 / 未取得 | 未取得 | 无 OpenAI 回应原句存档 | 未检索到 AP/Reuters 原文存档；OpenAI 回应未找到一手原句 | 未找到一手来源（OpenAI 回应） |
| C4 | Databricks AI Index：同成功率下两模型每任务成本差约 32 倍 | L1 | `https://www.databricks.com/blog/introducing-databricks-ai-index-state-enterprise-ai-tokenomics`（已存档 c-databricks-aiindex） | “In our coding benchmark, GPT-6 Luna and Claude Opus 5.5 both achieved an 89.1% task success rate. However, Luna averaged just $0.05 per task compared with $1.62 for Opus 5.5, a roughly 32x difference.” 任务来源：“Using tasks from our private codebase and withheld tests”；“real pull requests from Databricks' private codebase”。成本口径：“Model spending is estimated using publicly listed provider prices”。样本：“Aggregated, anonymized data from 10,000+ companies using Unity Gateway”（用量数据）；编码任务由 Databricks 工程师的真实工作流经 Unity Gateway 采集。数据区间：“The current dataset covers September 21 through November 7, 2026.” 免责：“Results reflect the workloads and models evaluated and may vary across organizations.” | 页面日期 “November 16, 2026”，官方未给时刻。档位（effort）全文未写；harness 仅写 “using Claude Code and Codex”，未逐行注明两模型各用哪个 harness。 | 已找到（数字与口径）；档位未找到 |
| C4 附 | 扫描称“样本限 Unity Gateway 客户” | L1 | 同上 | 用量数据来自 Unity Gateway；编码基准任务来自 Databricks 自家私有代码库与隐藏测试 | 说“限 Unity Gateway 客户”会把“自家私有代码任务”这一限定漏掉；基准的限定是 Databricks 私有代码库，不是客户样本 | 与说法不符（需改述，见勘误三） |
| C5 | 英伟达称 Vera Rubin 首次 MLPerf Endpoints 提交每瓦性能 10 倍，脚注称结果待 MLCommons 审核 | — | 未找到 SC26 原文 | 搜索未找到 2026-11 的 NVIDIA SC26 博客，也未找到 MLPerf Endpoints 成绩条目。相近的一手材料为 NVIDIA 2026-09-16 博客 `https://blogs.nvidia.com/blog/vera-rubin-nvl72-mlperf-inference`（MLPerf Inference v6.1 预览）：“the upcoming MLPerf Endpoints benchmark will bring standardized measurement to agentic inference workloads”；“Post-submission results, not yet verified by MLCommons, on GPT-OSS-120B and DLRMv3 show further performance gains.”（不是本条说法） | 9-16 博客的数字为吞吐量倍数（3.7x Qwen3-VL；2.5x DeepSeek-R1），不是每瓦 10 倍；未找到“10x per watt”“every benchmark”“TTFT P90”原句 | 未找到一手来源 |
| C6 | TensorWave 为 OpenAI 建 457MW 全 AMD 算力，首批 2027 上半年上线 | — | 未找到 | 搜索未找到 TensorWave 与 OpenAI 或 457 MW 的一手新闻稿。相关但不同的材料（搜索摘要，未存档）：TensorWave 2026-06-10 融资 $350M Series B（businesswire）；2026-07-24 采用 AMD Helios / MI455X（hpcwire）；与 Fermi 的 650MW 租约（datacenterdynamics，首期 222MW 自 2027 下半年起） | 未找到 457 MW、OpenAI、2027 上半年等字样 | 未找到一手来源 |
| C7 | 笔记应用 Mem 宣布关停 | L1 | `https://get.mem.ai/blog/mem-is-shutting-down`（已存档 c-mem-shutdown） | “Mem is shutting down on January 15, 2027.” “After a lot of thought, we’ve made the difficult decision to bring Mem to a close.” “December 18, 2026: notes become read-only. You won’t be able to create or edit notes, but you can still read and export your existing notes.” “January 15, 2027: Mem shuts down. You won’t be able to access Mem after this date. Please download your notes beforehand.” “You won’t be charged again. All charging has been paused.” 导出：“Start by saving a copy of your library. Our migration guide walks you through that download and moving to Apple Notes, Notion, Google Docs, Obsidian, OneNote, and other apps.” | 公告页日期 “Nov 16, 2026”，未给时刻。 关停原因：公告未写明原因；全文未出现 “AI” 作为关停理由（仅在产品回顾中提及 AI 功能）。 | 已找到（日期、导出、费用）；原因未写 |
| C8 | 中科院青藏高原所“万拓 One2N”把全球天气模型细化到 0.07° | — | 未找到 | 搜索只找到 ECMWF AI Weather Quest 参赛团队页 ONE2N（负责人田向军，单位列为中国科学院青藏高原研究所；搜索摘要，未存档）。未找到成果页（11-16）、arXiv 预印本（11-10）或“0.07°”“单卡约 4 周”“93.46%”原句。已存档的 `c-sinoxiv-one2n.txt` 与本条无关（2025 年 FourCastNet 文献列表）。 | 未找到“万拓”名称与分母定义 | 未找到一手来源 |

## 二、扫描说法勘误（以原文为准）

1. **C3 的“此前已有 6 个州起诉”与 1014 记录不符。** 派工单线索写“此前已有 6 个州起诉”，但 1014 期正文与存档（`docs/2610/1014/`）记为 10 月 14 日“阿肯色等8州”各自起诉；“6”在 1014 正文里是“已读7份诉状中有6份”的另一个计数。应以 1014 记录为准，写“此前 8 州 10 月 14 日起诉”或只写“与 10 月 14 日的 8 州起诉分开”，待 Claude 定稿时决定。
2. **C3 的“80”性质。** 新闻稿写 “one of at least 80 cyber incidents”，诉状写 “over 80 times”、“over 80 incidents”；两者限定词不同（at least / over），须逐字保留。“OpenAI admitted”是加州司法部的表述，OpenAI 自己的回应未找到。
3. **C4 的“样本限 Unity Gateway 客户”不完整。** 官方原文为：用量数据来自 “10,000+ companies using Unity Gateway”；编码基准的任务来自 “Databricks' private codebase and hidden tests”。两者应分开写。
4. **C1 金额口径。** 签约稿为 “approximately $8.2 billion”，且是 “all-stock”；完成稿无金额。扫描写的 82 亿成立，但缺“全股票”。
5. **C2 的 token/API 不变范围。** “token 与 API 不变”仅限图像/视频；语言模型改经 OpenRouter 后，自带供应商密钥不可用（破坏性变更）。
6. **C5、C6、C8 扫描数字均未在一手来源中找到。** 扫描的 SC26 博客、TensorWave 457 MW OpenAI 新闻稿、CAS 成果页与 arXiv 预印本，搜索均未命中。
7. **C7 热度。** 扫描称 HN “60 分”，本次抓取为 64 分 / 102 评（2026-11-17 19:25 北京时间）。

## 三、热度（HN，参考）

- C2：HN 50520212 “Hugging Face and OpenRouter join forces…”，4 分 / 0 评，提交 2026-11-17 01:29（北京时间）。
- C4：HN 50520896 “Databricks AI Index”，2 分 / 1 评，提交 2026-11-17 02:19（北京时间）。
- C7：HN 50526537 “Mem Is Shutting Down”，64 分 / 102 评，提交 2026-11-17 10:13（北京时间，取自 HN 官方 JSON）。
- C1、C3、C5、C6、C8：HN 搜索无相关条目（C6 命中条目与 TensorWave 无关）。
- Reddit：仅对 C7 查询，返回登录页重定向，记为失败，未重试。其余条目未查 Reddit。

## 四、风控提示（给 Claude 的参考，非本组结论）

- C3 为美国州政府起诉 OpenAI。`EDITORIAL.md` 速览风控：“涉外国政府与政治”的题材不放进速览，确需收录先问 Develata。1014 期已收录过 8 州起诉，此次属同一题材的跟进，是否收录由 Develata 决定。
- C5、C6、C8 因无一手来源，不宜进速览。
- C6、C8 的“未找到”可能是搜索工具覆盖不足，而非来源不存在，定稿前可再由 Claude 复核。

## 五、假设

- 北京时间以本机时区（UTC+8）为准；官方页只给日期的，时刻写“官方未给时刻”。
- HN 时间由 unix 时间戳经本机时区换算。
- C3 的“一手”以加州司法部新闻稿与诉状为准，未检索 OpenAI 官方回应。
- 搜索工具返回的摘要未存档为原文，仅作线索（L5 级别，不据此定稿）。
