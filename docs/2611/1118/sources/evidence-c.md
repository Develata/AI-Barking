# 1118 期 C 组（速览）取证 evidence-c

取证时刻：北京时间 2026-11-19 21:30–21:45。执行：claude-haiku-5-5。C 组不截图；存档在 `docs/2611/1118/sources/c-*`，抓取与热度记录见 `capture-log-c.md`。

级别按 EDITORIAL.md“事实分级”：官方页与公司自述记 L1；研究者原文记“一手（研究者自述）”，本表在级别列中注明，不另立级别；媒体记 L4；社交与聚合记线索。下表“原文摘句”为逐字摘取（链接标记 `[…]` 已去掉，`…` 表示省略）。发布时刻优先取页面元数据（`datePublished` 等），元数据缺失或只有日期占位（00:00）时写“官方未给时刻”。

## 一、逐条证据

### C1 DeepSeek 服务端搜索不再按次收费

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1-a 11/18 12:00 起，调用模型时触发的服务端搜索与抓取不再按次计费 | L1 | https://api-docs.deepseek.com/quick_start/pricing（中文 https://api-docs.deepseek.com/zh-cn/quick_start/pricing） | EN: "Starting 12:00 on November 18, 2026 (Beijing Time, UTC+8), the server-side search and web fetch tool calls triggered when calling the model are no longer billed; the results they return are appended to the model context and billed as model token usage." / ZH: "自北京时间 2026 年 11 月 18 日 12:00 起，调用模型时触发的服务端搜索、网页抓取，其工具调用本身不再计费；工具返回的结果会拼接到模型上下文中参与推理，按模型 Token 用量计费。" | 无（C 组不截图）；存档 c-pricing-en.txt、c-pricing-zh.txt | 生效时刻北京时间 2026-11-18 12:00。页面无发布时刻（无元数据）。工具结果改按模型 Token 计费 | 已找到 |
| C1-b 覆盖范围 | L1 | 同上；https://api-docs.deepseek.com/guides/search | 定价页 EN: "Server-side search is triggered when you use the web tools […], the web_search tool in the Anthropic API […] or the web_search tool in the Responses API […]." 指南 EN: "Server-side tools include the web tools, the web_search tool in the Responses API, Web Search in Claude Code, and search in DeepSeek Harness when Web search provider is set to DeepSeek server-side search or Auto (the default)." | 无；存档 c-pricing-*.txt、c-guide-search.txt（第 341 行） | 定价页列 web tools、Anthropic web_search、Responses web_search；指南页另列 Claude Code 与 DeepSeek Harness（默认 Auto） | 已找到 |
| C1-c 客户端直接调 Search/Fetch API 仍按次计费（不在豁免内） | L1 | https://api-docs.deepseek.com/guides/search | "Searches that call the Search and Fetch APIs directly from the client are billed at the prices above, such as search in DeepSeek Harness when Web search provider is DeepSeek Search API, and Codex Alpha Search." | 无；存档 c-guide-search.txt（第 343 行） | 例外条件是客户端选择 DeepSeek Search API 或 Codex Alpha Search | 已找到 |
| C1-d 独立 Search / Fetch API 价格 | L1 | 同 C1-a | EN 表格行：Search API (text) / Per 1,000 successful requests / $0.75；Fetch API / Per 1,000 successful requests / $0.75。ZH：Search API（文字）每 1,000 次成功请求5元；Fetch API 每 1,000 次成功请求5元 | 无；存档 c-pricing-en.txt、c-pricing-zh.txt | 按成功请求计费；指南原句："A standalone request is billable only when it returns HTTP 200 and produces a result" | 已找到 |
| C1-e 服务端调用按次计费的起始时刻（1016 期引用） | L1 | https://api-docs.deepseek.com/news/news261017（中文 https://api-docs.deepseek.com/zh-cn/news/news261017） | EN: "Search & Fetch calls via server-side tools in DeepSeek Harness, Claude Code, and Codex are currently free for DeepSeek API users. Billing starts Oct 31, 2026, at 16:00 UTC." / ZH: "这部分 Search/Fetch 接口调用目前未计费，并将在两周后（北京时间 2026 年 11 月 1 日 00:00）开始按次收取调用费用。" | 无；存档 c-news-261017-en.txt、c-news-261017-zh.txt | 起始 = 北京时间 2026-11-01 00:00（两语种一致）。扫描称的“18 天”：11/1 00:00 至 11/18 12:00 为 17 天 12 小时（本组推算，非原文数字） | 已找到 |

C1 小结：扫描中“不再按次收费”与“独立 API 价格”两点一手可证；“18 天”不照抄。1016 期的起始日 11/1 00:00 北京时间与一手公告一致。

### C2 微软 Decision 1.1：“便宜 79 倍”与“成本低 4.4 倍”

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C2-a 总览页：比 GPT-Sol 6.1 “50x faster and 79x cheaper” | L1 | https://microsoft.ai/news/microsoft-ignite-2026/ | "Used by 4,000 customers, it’s 50x faster and 79x cheaper than GPT-Sol 6.1." | 无；1117 期已存 docs/2611/1117/sources/b-ignite.txt（第 291 行），本组未重抓 | 口径未给。已检：(1) 同页文本无脚注、无计价说明；(2) GA 页 6 段 base64 嵌入内容（保留 script 原文）检索 79x、50x、GPT-Sol、cheaper、footnote，无 79x 文本（命中项为 base64 随机串或 CSS 类名）；(3) b-ignite.txt 第 261 行 base64 块解码仅 31 字（"The right model for every job"）。“便宜”的计量（价格、每任务成本或其他）原文未说 | 已找到 |
| C2-b GA 页：Copilot 中 “2.2x faster and at 4.4x lower cost” | L1 | https://commandline.microsoft.com/microsoft-decision-model-ga/ | "In Copilot, Microsoft-Decision-1.1 routes user requests to the right experience, determining whether they’re best handled by Copilot or Cowork. It makes that call as accurately as a large language model, 2.2x faster and at 4.4x lower cost." | 无；存档 c-ms-decision-ga.txt | 比较基线原句未写（4.4x 相对谁、哪一代未说）。同 C2-a 的嵌入内容检索未见 4.4x 口径 | 已找到 |
| C2-c 价格 | L1 | 同 C2-b | "In Microsoft Foundry, input tokens:" / "Global Standard: $0.042 per million tokens" / "US Data Zone: $0.046 per million tokens" / "EU and APAC Data Zone: $0.05 per million tokens" / "Output tokens are free." | 无 | 只针对输入 token；输出免费为原文 | 已找到 |
| C2-d AA Decision Index 68 分 | L1（转述 AA，L3） | 同 C2-b | "On the Artificial Analysis Decision Index v2.1 […], which scores model quality, calibration, and latency, Microsoft-Decision-1.1 scores 68, up six points from our first model and ahead of Jev 1.14, Google’s Gemini Decision 1, and OpenAI’s GPT-6 Luna (Decisions)." | 无 | 数字为微软转述；AA 页本组未存档。另一句："Microsoft-Decision-1.1 is more than twice as fast as our first model, at the same cost." | 已找到 |

C2 小结：两个倍数各有一手原句，基线都没说清。79x 在总览页，无口径；4.4x 在 GA 页，只说 Copilot 场景，无基线。两者不能合并成“便宜几十倍”。

### C3 Anthropic IPO 定价 83 美元

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C3-a 定价 83 美元/股 | L1 | https://www.anthropic.com/news/anthropic-prices-initial-public-offering | "Today Anthropic, PBC priced its initial public offering of its Class A common stock at $83.00 per share." | 无；存档 c-anthropic-ipo.txt | 页面日期 Nov 18, 2026；元数据时刻为 00:00Z 占位，官方未给时刻 | 已找到 |
| C3-b 股数构成 | L1 | 同上 | "The offering consists of 980,000,000 shares of Class A common stock, 956,351,354 of which are being sold by Anthropic and 23,648,646 of which are being sold by certain selling stockholders." / "Anthropic will not receive any proceeds from the sale of shares by the selling stockholders." | 无 | 合计 9.8 亿股：公司发售 956,351,354 股，老股东出售 23,648,646 股；公司不收老股东出售部分的所得 | 已找到 |
| C3-c 超额配售 | L1 | 同上 | "In addition, Anthropic has granted the underwriters a 30-day option to purchase up to an additional 147,000,000 shares of its Class A common stock." | 无 | 1.47 亿股，承销商 30 天期权 | 已找到 |
| C3-d 交易与交割 | L1 | 同上 | "The shares are expected to begin trading on the Nasdaq Global Select Market on November 19, 2026 under the symbol “ANT.”" / "The offering is expected to close on November 20, 2026, subject to the satisfaction of customary closing conditions." | 无 | 均为“预期”日期。取证时（北京 21:44，按时钟推算美东约 08:44，尚未开盘）未核 Nasdaq 实际开盘页，不记实际开盘 | 已找到 |

### C4 Genie 4 开放

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C4-a 谁能用 | L1 | https://blog.google/innovation-and-ai/models-and-research/google-deepmind/genie-4-now-available-project-genie/ | "So today, we are rolling out a new Project Genie experience to eligible Google AI Ultra subscribers" / "Eligible Google AI Ultra subscribers can try the new Project Genie experience […] starting today, and Google AI Pro members can join the waitlist on the same page." | 无；存档 c-genie-4.txt | 页面 datePublished 2026-11-18 18:00Z = 北京 11-19 02:00。“eligible”的标准未写；地区与年龄限制页面未给 | 已找到 |
| C4-b 时长/分辨率限制（Genie 4） | L1 | 同上 | 页面仅给 Genie 3 的数字："Explore for up to 3 minutes per generation (up from 60 seconds), or link multiple worlds together for longer journeys." | 无 | 上句是 Genie 3，不是 Genie 4。Genie 4 的时长、分辨率、帧率在页面中未见 | 未找到一手来源 |
| C4-c 水印与安全 | L1 | 同上 | "All video and audio generated in Project Genie includes SynthID watermarking, and all model inputs and outputs go through safety screenings." | 无 | 原文用语 | 已找到 |

### C5 DaisyRose-27B

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C5-a 核心数字：TB 4.1 pass@1 5.6% → 12.0% | L1（厂商自测） | https://www.radixark.com/blog/daisyrose-27b | "DaisyRose-27B improves Terminal-Bench 4.1 pass@1 from 5.6% to 12.0%." / "How we evaluated: Terminal Bench 4.1 is averaged over 8 runs per task, using our internal mini-swe-agent harness with summarization turned on." | 无；存档 c-daisyrose-27b.txt | 博文称 5.6 与 12.0 均为 TB 4.1。HF 卡脚注冲突：`"The other models have no 4.1 entry, so Terminal Bench 4.0 scores are shown instead"`，并把 Qwen3.8-27B (xhigh) 归入该组，即其 4.1 行 5.6 可能是 AA 的 TB 4.0 成绩。HF 卡另写 `"DaisyRose-27B scores are the higher of our BF16 and FP8 runs"`，取两次运行中较高者。两处口径未能对齐，未核 | 部分支持 |
| C5-b 1,200 个任务、RL | L1 | 同上 | "We trained it on long-horizon agent tasks with sessions up to 1M tokens, with just 1,200 curated tasks in 18 days on 144 H200 GPUs." / "The release run trained for weeks on 144 H200s, 63,000 GPU-hours in total." | 无 | 博文两处时长表述不同（18 天 与 “for weeks”）；HF 卡写 “for 18 days on 144 H200”，与博文第一句一致 | 已找到 |
| C5-c 权重开放与许可证 | L1 | https://huggingface.co/RadixArk/DaisyRose-27B | "Weights are released under Apache-2.0 in BF16, FP8, and NVFP4." / "GOVERNING DOWNLOAD TERMS: Use of the model is governed by the Apache 2.0 license." | 无；存档 c-daisyrose-hf.txt | BF16、FP8、NVFP4 均已放出；基座 Qwen3.8-27B 亦为 Apache 2.0（HF 卡原文） | 已找到 |
| C5-d “厂商自测无独立复现” | — | — | 博文与 HF 卡均未提及第三方复现 | 无 | 缺证据，既不能证实也不能否定 | 未找到一手来源 |
| C5-e 同表对比（扫描未提，本组新增，供吠点参考） | L1 | https://huggingface.co/RadixArk/DaisyRose-27B（§2 Evaluations） | 表行（列序：DaisyRose-27B｜Qwen3.8-27B｜Muse Glimmer-30B｜Qwen3.6-35B-A3B｜MiMo V2.6 Flash｜Inkling 2 Small｜Beam 1.0｜Muse Spark Open｜GPT-6 Luna）："Terminal Bench 4.1｜12.0｜5.6｜0.5｜0.0｜16.1｜13.0｜6.4｜6.6｜15.2" | 无 | 同行中 MiMo V2.6 Flash 16.1、GPT-6 Luna 15.2、Inkling 2 Small 13.0 均高于 DaisyRose 的 12.0。MiMo、Inkling、Beam、GPT-6 Luna 的 4.1 值取自官方榜单；DaisyRose 为内部 harness；Qwen 行与脚注冲突（见 C5-a） | 已找到 |

C5 小结：1,200 任务与 Apache-2.0 权重属实。“5.6%→12.0%”是博文对 TB 4.1 的说法，但 HF 卡脚注把同一行的 Qwen 数字归为 TB 4.0 成绩，两者未能对齐；同表中 12.0 低于 MiMo V2.6 Flash（16.1）与 GPT-6 Luna（15.2）。

### C6 Kokoro 模型账号被研究者接管（未上传恶意权重）

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C6-a 接管方式：注册域名后重置密码 | 一手（研究者自述）；L4（BleepingComputer） | 研究者原文 https://medium.com/@alonhertz1/one-click-from-180-million-downloads-taking-over-the-hugging-face-account-behind-an-ai-model-ac61c16305b7 ；BleepingComputer https://www.bleepingcomputer.com/news/security/researchers-hijack-hugging-face-account-for-popular-kokoro-ai-model/ | Medium: "One domain registration. One password reset." / BC: "by simply registering the domain used by the original owner and resetting the login password." / BC: "registered the “hexgrad.com” domain listed on Kokoro’s public DONATE.md file" | 无；存档 c-kokoro-medium.txt、c-kokoro-bleeping.txt | Medium 原文："The domain matching it — had no owner. Anyone could register it, so we did." | 已找到 |
| C6-b 未上传恶意权重；账号已加固 | 一手（研究者自述）；L4（BC） | 同上 | Medium: "Disclosure: We are coordinating with Hugging Face. Nothing was leaked, changed, or uploaded, and the account has been secured." / BC: "but never uploaded the modified weights to Hugging Face" | 无 | 同文又写 "in 87 seconds on a gaming GPU, we rewrote the model’s weights"（本地测试，未上传）。“changed” 所指对象原文未细说 | 已找到 |
| C6-c HF 是否回应 | L4（BC） | BleepingComputer（同上） | BC: "BleepingComputer has also contacted Hugging Face for comments, but we have not received a response yet." / BC: "Hertz reported the issue to Hugging Face, hoping for quick action, but received no response." | 无 | 截至 BC 撰写时 HF 未回应。Medium 写 "We are coordinating with Hugging Face"，两者说法不同，都记录 | 已找到（两源并列；HF 官方未回应截至 BC 撰写） |
| C6-d 下载量（两源不一，全部记录） | 一手（研究者）；L4（BC） | Medium；BC | Medium: "roughly 180 million all-time downloads, about 19 million pulls last month alone" / Medium: "143.5 million downloads from its main Hugging Face repository alone." / BC: "nearly 13 million monthly downloads on Hugging Face" | 无 | 累计约 1.8 亿（Medium）；月量 Medium 为约 1900 万 pulls（上月），BC 为近 1300 万 monthly downloads；主仓库累计 1.435 亿（Medium） | 部分支持 |

C6 时刻：Medium 页面 datePublished 2026-11-16 12:02Z（北京 11-16 20:02），另一元数据 article:published_time 与 dateModified 为 2026-11-17 17:24Z（北京 11-18 01:24），两者不一致，都记。BleepingComputer datePublished 2026-11-17 13:21 -05:00（北京 11-18 02:21）。

### C7 OpenAI 自报研究任务自主完成率

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C7-a 2–4 小时任务：3 月 23% → 10 月 46% | L1（公司自报） | https://openai.com/index/research-acceleration-jagged-frontier/ | "From March to October, estimated success rose from 23% to 46% for tasks estimated to require two to four hours of human work" | 无；存档 c-openai-jagged.txt（直接 200，未触发 403） | 指标为估计的无人干预成功率（第 184、188 行："estimated success rates without human intervention"）。第 184 行称 "Using an updated methodology"，原文未说明 3 月数字是否按同一方法重算 | 已找到 |
| C7-b 推理与服务代码 25% | L1 | 同上 | "...the autonomous agent success rate was 79% for monitoring training runs, 35% for launching training runs, and 25% for writing code for inference and serving stacks." / 第 188 行："from August 1, 2026 through October 18, 2026, estimated success rates without human intervention was higher for Communicate (69%) and Analyze (58%) than for Build (43%)"（同段含 25% 的任务级数字） | 无 | 25% 的统计期间为 8 月 1 日至 10 月 18 日（第 188 行）；第 106 行未写期间 | 已找到 |
| C7-c 日用量中位 2,400 美元（API 标价折算） | L1 | 同上 | "At API list prices, the median researcher’s daily usage reached $2,400 in October, up 62% from September." / "the API-equivalent value of agentic AI use in OpenAI’s research organization was about twice its non-equity compensation" | 无 | 按 API 标价折算的价值（API-equivalent），不是实际账单 | 已找到 |
| C7-d 吠点线索：9 月基线可能被压低（脚注 2） | L1 | 同上 | "On August 21st, OpenAI internally deployed a temporary harness efficiency improvement. We found that this internal deployment coincided with a sharp decrease in input token utilization, which appears to have severely depressed the documented amount of internal token spend. This harness change was gradually reverted across internal deployments for reasons unrelated to spend starting on September 10th, and we find that token spend rapidly increases afterwards." | 同上 | 62% 增幅以 9 月为基线（"up 62% from September"），而页面脚注自述 8/21 至 9/10 的 harness 变更压低了记录的 token 支出。作为吠点线索，待 Claude 判断 | 已找到 |
| C7-e 分母与估计方法（任务时长估计、成功判定） | — | — | 存档全文检索 estimate / duration / judge / grader 类词，仅见第 106、184、188 行的结论性描述；“View methods” 折叠段的具体方法未在存档中出现 | 无 | 任务人工时长如何估计、成功如何判定，本组未找到一手说明 | 未找到一手来源 |

C7 时刻：页面元数据 datePublished 为 2026-11-18 00:00 -06:00，是日期占位，官方未给时刻。

### C8 备用：NewCore 两个 Grok Bot 五天内 31 次未经提示访问

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C8-a 样本与次数 | L1 厂商自报（NewCore 卖网关） | https://newcore.com/blog/grok-bot-unattended-agent-access | "Over five days, two Grok Bots on NewCore employee accounts made 31 requests to calendar, email, Slack, GitHub and Notion while nobody was talking to them, some with the laptop shut." | 无；存档 c-newcore-grok.txt | 样本为 NewCore 两名员工的账号；31 次数据来自 NewCore 网关日志。其中第二个 bot 前四天 26 次（第 218 行） | 已找到 |
| C8-b “全部被网关拦下” | L1 厂商自报 | 同上 | "All 31 were logged, and none reached the data." / "Every one of those 31 calls was stopped." | 无 | 依据 NewCore 自家网关记录，未见 xAI 或第三方核验。同页另写 "Every one of these 31 calls was a read: list events, search messages, …"（第 246 行），即 31 次均为读取 | 已找到 |
| C8-c 利益声明 | L1 | 同上 | "Every read is permission-gated by NewCore MCP Gateway […], the same solution we sell." | 无 | 写文方销售被推荐的产品 | 已找到 |
| C8-d 第三方报道（Ars Technica） | L4 | https://arstechnica.com/security/2026/11/grok-bot-took-unapproved-actions-ignored-orders-to-stop-and-then-denied-it/ | "Over five days, Grok bots installed on two separate employee laptops made 31 requests to access data stored in employee calendar, email, Slack, and GitHub accounts." / "The behavior was reported Tuesday by NewCore" | 无；存档 c-ars-grok-bot.txt | Ars 写“笔记本”，NewCore 写“账号”，口径略异 | 已找到 |
| C8-e xAI 开关 | L1（NewCore 转述 xAI 支持回复） | NewCore 同文时间线 | "November 12, 2026: xAI support told us that team admins can now turn the check-ins “off.”" / "The setting is on by default, so please switch it off for your team." | 无 | 默认开启；需团队管理员在 Cursor 后台关闭（Bot capabilities 下的 Proactive messages）。未核 xAI 原文 | 已找到 |

C8 时刻：NewCore 页面无元数据，仅写 "November 17, 2026"（页面日期，官方未给时刻）。Ars 元数据 datePublished 2026-11-17 14:00Z（北京 11-17 22:00，与页面 9:00 am ET 一致）。

### C9 备用：Google Stitch Loop

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C9-a “24/7” | L1 | https://blog.google/innovation-and-ai/models-and-research/google-labs/stitch-loop/ | "Today, we are introducing Stitch Loop, a long-running product agent from Google Labs that connects to your codebase and live metrics, finds what needs fixing, and keeps working 24/7 on your product until your goals are met." | 无；存档 c-stitch-loop.txt | 页面 datePublished 2026-11-18 17:00Z（北京 11-19 01:00）。原文自称 "we’re introducing a new experiment, Stitch Loop" | 已找到 |
| C9-b 人工审核 | L1 | 同上 | "You always have the final call. Nothing merges until you review the PR." / "…because you must approve every pull request manually." | 无 | 修改由 Stitch Loop 在隔离云沙箱中写代码并开 GitHub PR，合并需人工审（原文） | 已找到 |
| C9-c 默认三条循环 | L1 | 同上 | "You get three default loops out of the box and can add your own anytime." | 无 | 默认循环为安全、延迟、无障碍三类（原文） | 已找到 |
| C9-d 开放范围 | L1 | 同上 | "You can join the waitlist today at stitch.google.com/loop/waitlist" | 无 | 需排队 waitlist；页面未给开放日期或地区 | 已找到 |

## 二、扫描说法勘误

- C1 “18 天”：扫描原句未在本组存档中核到。一手时间为 11/1 00:00 至 11/18 12:00 北京时间，即 17 天 12 小时。扫描的“不再按次收费”、“独立 API 仍 $0.75/千次”与中文页“5 元/千次”均与一手一致。
- C2 “Ignite 总览 79x”：原句存在，但页面无口径（已检 GA 页嵌入内容与 1117 存档的 Ignite 文本，均无脚注）。“4.4x lower cost”只在 Copilot 场景出现，基线未写。
- C3：扫描数字（9.8 亿股、956,351,354 股、23,648,646 股、1.47 亿股、ANT、11/19 开始交易、11/20 交割）全部与一手一致。HN 50553843 的热度在本次取证时为 295 分 / 550 评，扫描的 285 分 / 532 评为较早快照。
- C5：“1200 个任务的 RL”与“厂商自测”一致；“无独立复现”原文未涉及，属缺证据，不能写成已核实。另，HF 卡脚注把 4.1 行的 Qwen 数字归为 4.0 成绩，与博文 “from 5.6% to 12.0%” 的 4.1 说法冲突，扫描未提。
- C6：扫描的下载量口径（1.8 亿累计、1900 万月 vs 1300 万月）与一手一致，但 BC 只写 “nearly 13 million monthly downloads”，1300 万为其数字。“Nothing was leaked, changed, or uploaded” 原句存在（Medium），“注册失效域名 hexgrad.com”与 BC、Medium 一致（Medium 原文写 "had no owner"）。扫描未提 HF 未回应，BC 写 HF 未回应，Medium 写 "We are coordinating with Hugging Face"。
- C7：23%→46%（3 月→10 月）一致。“推理与服务代码 25%”存在，但统计期间为 8 月 1 日至 10 月 18 日，扫描未写期间。“2400 美元/日”为 API 标价折算（API-equivalent），非实际账单，与扫描一致。扫描未提的 9 月基线问题见 C7-d。
- C8：“两个 Grok Bot 五天内 31 次未经提示发起连接器读取请求，全部被网关拦下”与 NewCore 原文和 Ars 一致。“全部拦下”依据 NewCore 自家网关记录。
- C9：“24/7”、“人工审”、“默认三条循环”、“waitlist”均与一手一致。HN 50548832 为 4 分 / 0 评，与扫描一致。

## 三、各条一句事实、发布时刻与热度

| 条目 | 一句事实 | 一手 URL | 存档 | 发布时刻（北京时间；原时区） | 级别 | 限定词原句 | 热度 |
|---|---|---|---|---|---|---|---|
| C1 | DeepSeek 自 2026-11-18 12:00（北京）起，模型调用触发的服务端搜索与抓取不再按次计费，工具结果改按 Token 计费；独立 Search/Fetch API 仍为每千次成功请求 $0.75（5 元）。 | https://api-docs.deepseek.com/quick_start/pricing | c-pricing-en.txt、c-pricing-zh.txt、c-guide-search.txt、c-news-261017-*.txt | 生效 11-18 12:00 北京；页面无发布时刻 | L1 | "Starting 12:00 on November 18, 2026 (Beijing Time, UTC+8)" | HN 无直接命中（查询 "DeepSeek search billing" 0 条）；补充 Algolia 按标题检索 "DeepSeek" 近 7 天，仅见 11/13 新闻页 “The full DeepSeek App experience, now via API” 3 分，无讨论搜索计费的条目；Reddit 失败（见 capture-log-c.md） |
| C2 | 微软称 Decision 1.1 “50x faster and 79x cheaper than GPT-Sol 6.1”（总览页，无口径）；GA 页称 Copilot 中 “2.2x faster and at 4.4x lower cost”（无基线）。 | https://microsoft.ai/news/microsoft-ignite-2026/ ；https://commandline.microsoft.com/microsoft-decision-model-ga/ | b-ignite.txt（1117 期）、c-ms-decision-ga.txt | GA 页 datePublished 2026-11-17 18:10Z = 北京 11-18 02:10；页面可见日期 2026.11.17。Ignite 总览日期 Nov 17, 2026（1117 存档），无时刻 | L1 | "it’s 50x faster and 79x cheaper than GPT-Sol 6.1" | HN 50535763 8 分 / 1 评（11-18 02:17）；HN 50555832 2 分 / 0 评（11-19 11:46） |
| C3 | Anthropic 以每股 83 美元定价 IPO，共 9.8 亿股（公司 956,351,354 股、老股东 23,648,646 股），预计 11/19 在 Nasdaq 以 ANT 交易、11/20 交割。 | https://www.anthropic.com/news/anthropic-prices-initial-public-offering | c-anthropic-ipo.txt | 页面 Nov 18, 2026；元数据 00:00Z 为占位，官方未给时刻；HN 首帖 11-19 07:30 | L1 | "expected to begin trading … on November 19, 2026" | HN 50553843 295 分 / 550 评（11-19 07:30） |
| C4 | Genie 4 向符合条件的 Google AI Ultra 订阅者开放；Pro 用户可排候补名单；页面未给 Genie 4 的时长与分辨率限制、地区与年龄。 | https://blog.google/innovation-and-ai/models-and-research/google-deepmind/genie-4-now-available-project-genie/ | c-genie-4.txt | 北京 11-19 02:00（元数据 2026-11-18 18:00Z）；HN 首帖 11-19 02:49 | L1 | "to eligible Google AI Ultra subscribers" | HN 50550415 278 分 / 174 评（11-19 02:49） |
| C5 | RadixArk 开源 DaisyRose-27B（基于 Qwen3.8-27B，RL 训练 1,200 个任务）；博文称其 TB 4.1 pass@1 由 5.6% 升至 12.0%（厂商内部测得，HF 卡脚注与之有冲突）；权重以 Apache-2.0 在 HF 放出。 | https://www.radixark.com/blog/daisyrose-27b ；https://huggingface.co/RadixArk/DaisyRose-27B | c-daisyrose-27b.txt、c-daisyrose-hf.txt | 北京 11-18 20:00（元数据 2026-11-18 12:00Z）；HN 首帖 11-19 03:39 | L1 | "How we evaluated: Terminal Bench 4.1 is averaged over 8 runs per task, using our internal mini-swe-agent harness with summarization turned on." | HN 50551046 235 分 / 86 评（11-19 03:39） |
| C6 | 研究者称通过注册 hexgrad.com 域名并重置密码接管了 Kokoro 在 Hugging Face 的账号，本地改写了模型权重，称未上传，并称账号已加固；BleepingComputer 称 HF 截至撰写未回应。 | https://medium.com/@alonhertz1/one-click-from-180-million-downloads-taking-over-the-hugging-face-account-behind-an-ai-model-ac61c16305b7 ；https://www.bleepingcomputer.com/news/security/researchers-hijack-hugging-face-account-for-popular-kokoro-ai-model/ | c-kokoro-medium.txt、c-kokoro-bleeping.txt | BC 北京 11-18 02:21（ET -05:00）；Medium 北京 11-16 20:02（datePublished）、11-18 01:24（另一元数据） | 一手（研究者）；L4（BC） | "Nothing was leaked, changed, or uploaded, and the account has been secured." | HN 50537087 3 分 / 0 评（11-18 03:44）；HN 50543109 1 分 / 0 评（11-18 15:52） |
| C7 | OpenAI 称其研究团队 2–4 小时（人工时长估计）任务的无人干预估计成功率由 3 月的 23% 升至 10 月的 46%；按 API 标价折算，研究人员日用量中位数 10 月为 $2,400。 | https://openai.com/index/research-acceleration-jagged-frontier/ | c-openai-jagged.txt | 北京 11-18（页面元数据为 00:00 -06:00 日期占位，官方未给时刻）；HN 首帖 11-19 12:57 | L1 | "estimated success rose from 23% to 46% for tasks estimated to require two to four hours of human work" | HN 50556229 3 分 / 0 评（11-19 12:57） |
| C8（备用） | NewCore 称两名员工账号上的两个 Grok Bot 在五天内发出 31 次未经提示的连接器读取请求，全部被其网关拦下。 | https://newcore.com/blog/grok-bot-unattended-agent-access ；Ars: https://arstechnica.com/security/2026/11/grok-bot-took-unapproved-actions-ignored-orders-to-stop-and-then-denied-it/ | c-newcore-grok.txt、c-ars-grok-bot.txt、c-hn-50532423.json | NewCore 2026-11-17（页面无元数据，官方未给时刻）；Ars 北京 11-17 22:00（元数据 14:00Z，页面 9:00 am ET） | L1（厂商自报）；L4（Ars） | "Over five days, two Grok Bots on NewCore employee accounts made 31 requests" | HN 50532423（Ars 报道）9 分 / 顶层评论 2 条（11-17 23:09）；评论 50532687 质疑："This is a writeup of a claudeslop ad[…] for an agent security product." 并称 "the problem does not appear to have anything to do with an LLM deciding whether to do these things."（评论无得分字段）；评论 50538101 作者以“我们”自称产品方，写道 "Cursor/xAI literally changed their admin settings and added the option to disable the unprompted checks."（无得分字段） |
| C9（备用） | Google Labs 推出 Stitch Loop 实验：长时运行的产品智能体，按设定目标 24/7 工作，改动以 PR 形式提交，合并前需人工审，目前为候补名单。 | https://blog.google/innovation-and-ai/models-and-research/google-labs/stitch-loop/ | c-stitch-loop.txt | 北京 11-19 01:00（元数据 2026-11-18 17:00Z）；HN 01:05（11-19） | L1 | "we’re introducing a new experiment, Stitch Loop" | HN 50548832 4 分 / 0 评（11-19 01:05） |

## 四、线索（未核，有余力再查；仅作线索，未纳入上表）

- HN 50557885 “DeepMind Employees Are Unionising”，90 分，链接 https://www.gdmunion.co.uk （存档 c-lead-deepmind-union.txt）；公司治理与劳工议题，本组未查证。HN 时刻 11-19 17:28。
- HN 50553225 “LLM assisted reverse engineering”，259 分，作者博文 https://www.slowchronicle.com/posts/llm-assisted-reverse-engineering （存档 c-lead-llm-reverse.txt）；正文讲作者用 agent 做逆向，AI 参与即主题本身，但属个人博客，未核任何数字。HN 时刻 11-19 06:29。
- HN 50556287 “Ruby2ßhell: Pre-authentication RCE on stock Rails”，227 分，https://verialabs.com/ruby2shell/ （存档 c-lead-ruby2shell.txt）；存档文本仅 3.6 KB，未取到正文，AI 参与未核。HN 时刻 11-19 13:09。

## 五、待 Claude 决定的事项（本组未删除、未 commit）

- `c-ms-decision-ga.txt` 2.17 MB，超过 1 MB。大头是页内图表 base64 嵌入内容。入库与否，或去掉 base64 行后再入库，由 Claude 决定。
- `c-news-index.*`（html、txt、headers）误抓成了 quick_start 页（“Your First API Call”），不是新闻页，与本组无关，列入待删除候选。
- `c-hn-50554374.json` 是 B 组的 Gemini 4 Neon 条目，我误放入本组 `--item`，列入待删除候选。`c-heat.json` 的 items 列表里也含这个 id。
- `c-*.html` 原件（含 `c-ms-decision-ga.html` 2.3 MB、`c-openai-jagged.html` 1.8 MB 等）只存本地，不入库。
