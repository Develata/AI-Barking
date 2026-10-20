# 1019 期 E 组取证：速览（产品与行业）

执行方：claude-haiku-5-5（Haiku 5.5）。取证时刻：2026-10-20 北京时间 19:13–19:18（本机 07:13–07:18 EDT）。
范围：派工单 E1–E4 与“扫描说法勘误”。按派工不截图，只存档（html + txt）与逐字摘句。
文件前缀 `e-` 位于本目录；抓取与热度日志见 `capture-log-E.md`。

## 一览

| # | 说法（线索） | 级 | 一手来源 | 发布时刻 | 状态 |
|---|---|---|---|---|---|
| E1 | 据路透社报道 OpenAI 加速融资 | L4 | 路透社原文（无 OpenAI 官方确认） | 10/20 07:18（北京）／10/19 23:18:45 UTC | 已找到 |
| E2 | Browser Use Bench v2.1 | L1（运营方自测） | browser-use.com 基准页 | 页面标注 10/17，官方未给时刻 | 已找到 |
| E3 | GitHub Copilot 下线旧模型 | L1 | GitHub changelog | 10/20 05:27（北京）／10/19 14:27 PDT | 已找到 |
| E4 | OpenAI 低资源编程语言评测（COBOL） | L1（自测） | openai.com 研究页 | 页面标注 10/19，官方未给时刻 | 已找到 |

---

## E1 据路透社报道 OpenAI 加速融资

- **一句事实**：据路透社报道（引两名知情人士），OpenAI 正争取在 Anthropic 预计 11 月中旬公开上市前，完成一轮约 300 亿美元的融资，估值可能超过 1.4 万亿美元；SoftBank、MGX 等现有投资方“洽谈”加入该轮。
- **一手来源**：路透社原文 https://www.reuters.com/world/openai-seeks-accelerate-fundraising-ahead-anthropic-ipo-sources-say-2026-10-19/ （URL 已去除查询参数）。
  - 存档：`e-reuters.html`（opencli 浏览器取得，本地保存，不入库）、`e-reuters.txt`（入库，已删去记者个人联系链接）、`e-reuters-extract.json`（opencli extract 原始输出）。
  - curl 直取返回 401，存档为 `e-reuters-openai.html` / `.headers`，记为失败（见 capture-log-E）。
- **发布时刻**：`article:published_time` 2026-10-19T23:18:45.809Z（UTC）= **2026-10-20 07:18 北京时间**；稿件 dateline 为 “NEW YORK, Oct 19”；页面最后修改 2026-10-20T09:01:34Z（北京 17:01）。
- **级别**：L4（权威媒体，正文写“据路透社报道”；匿名来源，未经 OpenAI 证实）。
- **限定词原句**：
  - “according to two people familiar with the matter”
  - “SoftBank Group … MGX and other existing backers of OpenAI are in talks to join the round of about $30 billion, one of the people said.”
  - “could value the ChatGPT maker at over $1.4 trillion”
  - “OpenAI and SoftBank declined to comment. Anthropic and MGX did not immediately respond to requests for comment.”
  - 上市时点原句：“rival Anthropic's expected public market debut in mid-November”
- **状态**：已找到（L4，以路透社原文为核对对象）。“about $30 billion”“could value … over $1.4 trillion”“two people familiar with the matter”与线索一致；“洽谈中”与“could”的语气作为限定词保留在上面，不是与说法不符。OpenAI 官方确认未找到（未专门搜索其官方渠道；路透社称 OpenAI 与软银拒绝置评）。
- **热度**：HN 50159462，3 分 / 1 评，10/20 07:39 北京时间；同题 HN 无其他命中。Reddit 搜索 4 次被重定向到登录页，记失败（见 `e-heat.json` 的 errors）。

## E2 Browser Use Bench v2.1

- **一句事实**：Browser Use 发布基准 v2.1（180 项真实网站长任务，多模型同一套 harness）：GPT-6.1 Sol medium 得分 79.6 居首；DeepSeek V4.1 Flash 1017 max 得分 75.0，单任务模型成本 22¢；Claude Opus 5.5 high 得分 75.0，成本 $2.35；Browser Use 自家 agent（BU fast）得分 72.9，成本 11¢。
- **一手来源**：https://browser-use.com/benchmarks/agents 。存档 `e-browseruse-agents.html`（本地）、`e-browseruse-agents.txt`（入库）。
- **发布时刻**：页面标题标注 “October 17, 2026”；页面元数据 dateModified 也只到日期。**官方未给时刻**。
- **级别**：L1（运营方自己的基准页）。注意：运营方同时卖自家 agent，且得分由其自己的 harness 测得，正文需点明“自家评测，非独立第三方”。
- **限定词原句**：
  - “Mean weighted rubric score per task, 0-100.”（分数是加权任务得分，不是成功率）
  - “Estimated model cost per task in USD. Excludes search, judge, browser and runner costs.”（成本是模型估算，不含搜索、评判、浏览器与运行器费用）
  - “† Ran on different CI runners from the rest of the lineup, so its time is not comparable.”（DeepSeek 1017 各行的耗时有此 † 标注）
  - “180 long tasks on live websites, the same for every model, run in the open-source BrowserCode harness (Browser Use's own agent runs in ours).”
  - “Runner and encrypted tasks: github.com/browser-use/benchmark”（部分任务加密，页面给出公开仓库）
- **状态**：已找到。数字与页面逐条一致（见 txt 表格）。
- **热度**：HN 无相关命中（“Browser Use Bench” 检索只得无关帖）；Reddit 失败。

## E3 GitHub Copilot 下线旧模型

- **一句事实**：GitHub 称，截至 2026-10-19，多数 GitHub Copilot 体验中已下线 GPT-5.5、GPT-5.4、GPT-5.4 mini、GPT-5 mini、Gemini 3.7 Flash、Grok 4.5；官方建议替代为 GPT-6.1 Sol（对应 GPT-5.5、GPT-5.4）、GPT-6 Luna（对应 GPT-5.4 mini、GPT-5 mini）、Gemini 3.8 Flash（对应 Gemini 3.7 Flash）、Grok 4.7（对应 Grok 4.5）。
- **一手来源**：https://github.blog/changelog/2026-10-19-github-copilot-models-deprecated-on-october-19-2026/ 。存档 `e-copilot-changelog.html`（本地）、`e-copilot-changelog.txt`（入库）。
- **发布时刻**：页面 datePublished 2026-10-19T14:27:10-07:00（PDT）= 2026-10-19 21:27 UTC = **2026-10-20 05:27 北京时间**。正文写 “As of today, October 19, 2026”。
- **级别**：L1（GitHub 官方 changelog）。
- **限定词原句**：
  - “we have deprecated the following models across most GitHub Copilot experiences (including Copilot Chat, inline edits, ask and agent modes, and code completions).”
  - “These deprecations were announced on September 18, 2026.”
  - “Copilot Enterprise administrators may need to enable access to alternative models through their model policies in Copilot settings.”
  - “No action is required to remove the deprecated models.”
- **状态**：已找到（六个模型名与适用范围与线索一致）。
- **另见**：页面“Related Posts”列出 10/15 公告 “Upcoming deprecation of selected GitHub Copilot models in mid-November”，这是另一批、11 月中旬的下线，本组未存档其内容。
- **热度**：HN 50119093（即上述 10/15 的 11 月公告），2 分 / 0 评，10/16 22:52 北京时间；10/19 本条目在 HN 无命中。Reddit 失败。

## E4 OpenAI 低资源编程语言评测（COBOL）

- **一句事实**：OpenAI 称，其自建 MultiLangBench（用 20 种语言从头实现 SQLite 兼容数据库）中，GPT-6 Astra 在最高推理档平均通过率 87.4%（GPT-5.6 Sol 为 26.8%），COBOL 一项从 10.9% 提到 72.1%；另一套 COBOL 迁移评测中，GPT-6.1 Sol 在低推理档准确率 86.6%（GPT-5.6 Sol 为 5.4%），较高推理档达 97.9%。
- **一手来源**：https://openai.com/index/low-resource-programming-languages/ 。存档 `e-openai-lowres.html`（本地）、`e-openai-lowres.txt`（入库）。另存 MultiLangBench 仓库页 `e-multilangbench-repo.html` / `.txt`（https://github.com/openai/MultiLangBench）。
- **发布时刻**：页面标注 “October 19, 2026”；页面无时刻元数据。**官方未给时刻**。
- **级别**：L1（OpenAI 官方研究页；评测为厂商自测，非独立第三方，正文需点明）。
- **限定词原句**：
  - “we developed two evaluations”
  - “At maximum reasoning effort, GPT‑6 Astra achieves an average test pass rate of 87.4% across the 20 languages”
  - “At higher reasoning effort, accuracy reaches 97.9%, nearly saturating the evaluation.”（COBOL 迁移评测）
  - “GPT‑6 Luna … approximately 78% lower estimated API cost.”（成本为估算）
  - “we evaluate our models on hidden tests that require byte-for-byte parity on observable outputs.”（迁移评测用隐藏测试）
  - “applying it to production software still requires expert review and testing against the system's business rules and dependencies.”
  - “We are releasing the MultiLangBench evaluation setup and test suite on GitHub”（仅 MultiLangBench 公开）
- **状态**：已找到。页面上所有数字都找到了；线索漏写模型名与推理档，已记入上面的限定词与勘误。
- **热度**：HN 上同一官方页有 4 条提交：50159639（4 分 / 0 评，10/20 08:03）、50159090（2 分，06:48）、50162354（2 分，15:06）、50162135（1 分，14:32），均 0 评。Reddit 失败。

---

## 扫描说法勘误

1. **E1（措辞修正，状态不变）**：扫描称“约 300 亿美元”“超 1.4 万亿美元”。原文为 “in talks … round of about $30 billion”（洽谈中）与 “could value … over $1.4 trillion”（可能），正文应写成“洽谈中的约 300 亿美元一轮、估值可能超过 1.4 万亿美元”。扫描未写路透社“OpenAI 与软银拒绝置评”。扫描“10/19 23:18 UTC”无误，北京时间为 10/20 07:18。
2. **E2**：扫描的 22¢、75.0、79.6、72.9 等数字无误；“分数为加权任务得分非成功率”与原文一致。扫描漏了三点限定：成本不含搜索、评判、浏览器与运行器费用（页面原句）；DeepSeek 1017 的耗时不可比（† 标注）；自家 agent 在其自建 harness 中运行。
3. **E3**：扫描列出的六个模型、“多数 Copilot 体验”、10/19 日期均与原文一致，无勘误。补充：替代模型映射见上文；10/15 另有一批 11 月中旬下线公告。
4. **E4**：
   - 扫描“COBOL 10.9%→72.1%”漏了模型与档位：是 GPT-6 Astra 在最高推理档、对比 GPT-5.6 Sol，且来自 20 语言的 MultiLangBench。
   - 扫描“迁移评测 97.9%”漏了模型与档位：是 GPT-6.1 Sol 在较高推理档，属 COBOL 迁移评测，不是 MultiLangBench。
   - 扫描“用未公开测试集”限定的是“迁移评测 97.9%”，与原文 “hidden tests” 一致，无勘误。注意不要与 MultiLangBench 合并：MultiLangBench 的测试套件已在 GitHub 公开（Apache-2.0），该公开仓库中未见 COBOL 迁移内容。
   - 扫描“自建”与原文 “we developed two evaluations” 一致。
   - 待核、非勘误：MultiLangBench 仓库 README 称测试由 SQLite 3.40.1 的 TCL 测试转换而来（51k 中仅 24k 可转换），并建议默认只用 00/01/02 开头的 272 项；博客未注明其数字用的是哪一子集。博客称随仓库发布“每语言结果”，仓库根目录未见结果文件（只逐一查了 README，未逐文件核）。

## 未完成与失败

- 路透社 curl 直取 401（已记失败）；opencli 浏览器取得全文。登录态核过：`e-reuters.html` 页头为 “Sign In” 与 “Subscribe”，无账户或订阅者标识，属匿名态可见，无付费墙提示。未登录、未点击、未越过付费墙。
- Reddit 搜索 4 次全部重定向到登录页，热度只有 HN。
- 未查 OpenAI、软银官方渠道对 E1 的确认（未专门搜索）。
- E2 的 HN 检索（“Browser Use Bench”“browser-use.com”）均无相关命中。
- 未截图（派工单 D/E 组规定不截图）。
