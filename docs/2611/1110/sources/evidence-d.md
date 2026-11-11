# 1110 期取证：D 组（速览）

执行方：Claude Haiku 5.5（模型 ID `claude-haiku-5-5`）。时间为北京时间（UTC+8），官方原时区括注。存档与抓取记录见 `capture-log-D.md`。本组未截图，只存档（html 原件仅存本地，txt 入库）与逐字摘句。

级别按 EDITORIAL “事实分级”：L1 官方发布、官方文档、官方社交账号、官方博客；L3 第三方分析或评测的原始页面，仅限其自身数据。

## D1 DeepMind AlphaProof Code（Signal 形式化验证）

- 事实（照原文）：DeepMind 称“agents powered by Gemini Carbon did the proofs, generating over six million lines of Lean”；人类只需审核“just one percent of the codebase”（博文原句，指智能体审计后标出需要人工再看的部分）。论文称“Our proofs amount to about 6.5 million lines of Lean”。
- 一手来源：博文 `https://deepmind.google/blog/introducing-alphaproof-code/`；技术论文（页面即论文，Verso 版）`https://google-deepmind.github.io/code-verification`。存档：`d-d1-alphaproof-blog.html|.txt`、`d-d1-code-verification.html|.txt`。
- 发布时刻：博文 “November 10, 2026”（官方未给时刻）；论文页标注 2026-11-10（官方未给时刻）。
- 级别：L1（官方博客与官方论文页，DeepMind 自述）。
- 限定词原句：
  - 验证范围：“We describe the version implemented by libsignal v0.105.4, restricted to one-to-one sessions. (We leave group messaging as future work.)”
  - 对象：“the results concern our build of libsignal-cli …, run as a fresh process on a machine with the memory of §4.1, not Signal’s own applications, and some of the environment assumptions are argued informally.”
  - 博文自述：“Our work marks the first time AI has been used to formally verify a widely deployed cryptographic protocol at this depth.”（自述，未经独立核实）
  - 时长：“our verification process took just four weeks”
  - 论文表述：“Not covered … Quantum attackers, multi-device state, adversarial first messages …”（表格）
- 热度：HN 50439193，608 分、182 条嵌套评论（14:32 BJ 抓取；见 `d-hn-50439193.json`）。
- 备注：博文的“one percent”与论文的 6.5 百万行是同一工作的两处数字，口径不同，并列记录，不取舍。

## D2 VulnCheck 复盘 Glasswing 七个月（跟进 1007）

- 事实（照原文）：VulnCheck 称“Of Anthropic’s 37,966 reported findings, only 7,349 (19.4%) have been made public and just 217 have been marked fixed.”；“Roughly 1,654 findings (4.4%) have been marked as duplicates”（正文）。curl 一批：“Of the 34 reports published so far, 19 have been marked as not a vulnerability, 6 have been confirmed as low severity, and 9 are still awaiting assessment.”
- Anthropic 台账（截至 11/02）：“As of November 2, 2026, we’ve disclosed 5,087 vulnerabilities across 680 open source projects. To our knowledge, 629 of these have been patched.”；“94.2% true positives of 6,376”（标签，同图 6,009 confirmed valid）。
- 一手来源：VulnCheck `https://vulncheck.com/blog/anthropic-glasswing-november-update`（L3，第三方分析，仅限其自身统计）；Anthropic 台账 `https://red.anthropic.com/2026/cvd/`（L1）。存档：`d-d2-vulncheck.html|.txt`、`d-d2-anthropic-cvd.html|.txt`。
- 发布时刻：VulnCheck “November 9, 2026”（官方未给时刻）；台账数字“as of November 2, 2026”（官方未给时刻）。
- 限定词原句（台账，定义）：“the ‘true positive rate’ (the number of findings confirmed as valid, as a share of the number of findings manually reviewed) … This includes real bugs that we later discover have already been reported, and ‘won’t fix’ findings”；“Direct disclosure: Other vulnerabilities are disclosed to maintainers directly, without the same independent check.”
- 并列口径（不判谁对）：
  - 公开数量：VulnCheck 称 “only 7,349 (19.4%) have been made public”，台账称 “we’ve disclosed 5,087 vulnerabilities across 680 open source projects”；两个口径（made public 与 disclosed）未统一，并列。
  - 修复数量：VulnCheck 217（“Anthropic’s own public ledger lists just 217 fixed vulnerabilities”）对台账 “629 of these have been patched” 与 “Patched upstream … 629 findings”。
  - 重复数（VulnCheck 正文）：1,654；VulnCheck 表格行：1,670（由文本抽取推断，表格单元格需目视核实，未核实）。
- 热度：HN 50447074，373 分、160 条嵌套评论（14:32 BJ；`d-hn-50447074.json`）。
- 未找到一手来源：“Anthropic 官方称 curl 那批 ‘fully model-generated, without human review or triage’”。台账页未见此句；仅二手报道（implicator.ai、AlphaSignal、Neoteo、SmartScope）转述。Anthropic 的 OSS Scanner 官方页未存档。
- curl 标签计数（GitHub 搜索，`label:vuln:NONE` 51、`vuln:LOW` 8、`vuln:ASSESS` 1；仓库全部 issue，未限定本批 Anthropic 报告，无法与 VulnCheck 的 19/6/9 对齐；数据见 `d-d2-curl-labels-vuln-none.json`）。

## D3 亚马逊用 Strands Decider 替换 Sonnet 4.6

- 事实（照原文）：“Two Amazon engineers swapped Claude Sonnet 4.6 for a decision model … Strands Decider … performed just as well at 98% less cost. This change will save millions of dollars of token costs in a year.”
- 成本（官方博客表）：Sonnet 4.6 每百万工单 “about $22,200”；Decider E4B “about $500”；“That is about 44 times cheaper than Sonnet, a 98% reduction.”
- 一手来源：`https://strandsagents.com/blog/amazon-saves-millions-replacing-sonnet-with-strands-decider/`（Strands 团队博客，页脚 “© 2026 Amazon Web Services, Inc.”）。存档：`d-d3-strands.html|.txt`。
- 发布时刻：“November 9, 2026”（官方未给时刻）。
- 级别：L1（开发团队官方博客，自述；厂商自家案例，非独立评测）。
- 限定词原句：
  - 测试集：“On a test set of tickets, the E4B Gemma 4 build … got clearly better results … The results showed E4B beating production Sonnet on a separate set of our tickets.”
  - 价格口径：“These are published list prices. They leave out the Lambda, which also runs for less time per ticket now.”
  - 适用条件：“This worked because the task is a closed choice from a known catalog, the volume is high, and the latency of the old path mattered. If your labels need explanation text, if the catalog changes weekly, or if you can’t fit the useful context into a few thousand tokens, a decision model probably can’t replace the LLM’s job.”
  - 对象：“nine month old model”（Sonnet 4.6）；“a decision model we launched a month ago”（Decider）。
  - 工单量：博文未给总量（“millions of dollars … in a year” 为预期句）。
- 热度：HN 50436640，702 分、246 条嵌套评论（14:32 BJ；`d-hn-50436640.json`）。

## D4 Anthropic “顺势学习”（稀释投毒数据反而增强）

- 事实（照原文）：Anthropic 对齐团队博文称 “poisoned training data with little effect on its own can create a hidden backdoor once clean, contrasting data is added”；实验在 “small open-weight models (Qwen3-8B and Gemma-3-4b-it) or a small MNIST network”。
- 数字（逐组）：
  - MNIST 小网（同一实验）：“With 500 poisoned examples alone, the model classifies only 10% of patched test digits 5, 6, 8 and 9 as 7. Adding clean data … increases this rate to 60%”。
  - Qwen3-8B 人格实验：“from 8% to 72% in Qwen3-8B conditional on the user name Michael”。
  - Qwen3-8B 涌现错位实验：“When 80% of the poisoned data is replaced with clean data, however, the misalignment rate under the inoculation prompt rises from 7% to 16% for Qwen3-8B”。
- 一手来源：博文 `https://alignment.anthropic.com/2026/homeopathic-learning/`；arXiv `https://arxiv.org/abs/2611.10110`（arXiv 页 “Submitted on 6 Nov 2026”）。存档：`d-d4-homeopathic.html|.txt`、`d-d4-arxiv.html|.txt`。
- 发布时刻：博文 “November 10, 2026”（官方未给时刻）；arXiv 提交 6 Nov 2026（arXiv 时刻未在页内给出，未核实）。
- 级别：L1（Anthropic 官方对齐博文；作者含 Anthropic Fellows 与 Truthful AI 等，文中标注 “Work done as part of the Anthropic Fellows Program, in collaboration with Truthful AI.”）。
- 限定词原句：“We do not study any production training pipeline.”
- 热度：未找到 HN 讨论（关键词 “Homeopathic Learning” 无命中，`d-heat.json` 记录）。

## D5 OpenAI：思维链可监控性随模型变强而下降

- 事实（照原文）：OpenAI 称 “with GPT-6 Astra we observed degradation in CoT monitorability for non-adversarial settings …, greater ability to perform CoT monitor evasion under adversarial testing …, and significantly greater no-CoT capability … compared to prior models.”；结论句 “These results are sobering: as we scale toward ever-more intelligent systems, models are becoming harder to monitor through their CoT.”
- 一手来源：`https://alignment.openai.com/cot-monitorability-model-lifecycle/`（OpenAI 对齐研究博客）。存档：`d-d5-openai-cot.html|.txt`（抓取时 HTTP 200，未被 403）。
- 发布时刻：“November 10, 2026”（官方未给时刻）；响应头 Last-Modified 为 2026-11-10 22:14:09 GMT（即 11-11 06:14 BJ）。
- 级别：L1（官方研究博客）。
- 局限原句（两处）：
  - 代理指标：“both of which are imperfect proxies for decreased CoT monitorability”（指无 CoT 能力与 CoT 可控性两者）。
  - 噪声：“the CoT controllability evaluation can be noisy and is an imperfect proxy, which does not always correspond with more monitor evasion (Appendix B)”；附录 B：“We believe CoT controllability is a noisy proxy for adversarial monitorability.”（只说可控性有噪声。）
- 热度：未找到 HN 讨论（关键词无命中）。

## D6 OpenRouter：默认 46 个工具的 MCP 比 CLI 贵约 17 倍

- 事实（照原文）：“On Sonnet 5.5, the default MCP server used 42 times the CLI’s input tokens and cost 17 times as much, $127 against $7.50 per 1,000 tasks.”（表：CLI 1,421 输入 token、$0.00756/任务；MCP 46 工具 59,417 输入 token、$0.1274/任务）。
- 一手来源：`https://openrouter.ai/blog/insights/mcp-vs-cli-vs-skills/`（OpenRouter 官方博客，厂商自测）。存档：`d-d6-openrouter.html|.txt`。
- 发布时刻：页面日期 11/11/2026（官方未给时刻）；测试时刻 “We ran each combination three times on October 20, 2026, and show medians.”
- 级别：L1（官方博客，厂商自测；非独立评测）。
- 限定词原句：
  - 局限：“We tested one server on one task, so run mcp_cost.py on yours before you decide.”
  - 口径：“Haiku 5.5 counts a prompt exactly as Sonnet 5.5 does, but it took extra turns in some runs, so we show Sonnet’s steadier tokens and billed cost.”
- 内部不一致（需留意）：正文 TL;DR 写 “The GitHub MCP server’s 46 default tools add 19,436 tokens to every request on Claude Sonnet 5.5”；正文脚本输出与后段 “The same 46 tools cost 19,436 tokens on Haiku 5.5 but 8,903 on GPT-6 Luna” 把 19,436 归为 Haiku 5.5。同一篇文章对该数字的模型归属不一致，未核实哪处正确（脚本输出在 `anthropic/claude-haiku-5.5` 行）。
- 热度：未找到 HN 讨论（关键词 “OpenRouter MCP CLI skills” 无命中）。

## D7 通义 Qwen4-Max 预览

- 事实（照原文，@Alibaba_Qwen 帖）：“Qwen4-Max is now in preview on Alibaba’s Token Plan. You can also get early access to the new model on QwenWork, Qoder, Qwen APP, and QwenStudio.”
- 一手来源：X 帖 `https://x.com/Alibaba_Qwen/status/2120335993643684315`（fxtwitter JSON 存 `d-d7-qwen4max-fxtwitter-raw.json`；抓取时间 BJ 14:3x）。
- 发布时刻：2026-11-11 02:01:09 UTC（北京 11-11 10:01:09）。
- 级别：L1（官方社交账号；X 平台名不写进正文，仅本档案记录）。
- 帖内无基准、价格、权重信息（与扫描一致）。帖内 “Token Plan: http://qwencloud.com/pricing/token-plan”；定价页存档 `d-d7-token-plan.html|.txt`，页面未提 Qwen4-Max（逐行检索 “qwen4”/“max” 无命中）。
- 未找到一手来源：Qwen 官方博客或模型卡未找到。WebSearch 只返回 Qwen3.8-Max-Preview（7 月 19 日、Token Plan 独占）的报道，与 Qwen4-Max 不是同一名称，未采信。
- 热度：HN 50449130 “Qwen4-Max is now in Preview”，24 分、1 条评论（HN 发布 10:33:25 BJ；`d-hn-50449130-raw.json`）；X 帖 14:3x 抓取：浏览 510,843、赞 7,224、转发 1,075（fxtwitter 字段）。

## D8 Claude Whiteboards、Sheets、Data

- 事实（照原文）：“Today we’re launching Claude Whiteboards and Claude Sheets in beta, and Claude Dashboards … becomes generally available as Claude Data.”；适用计划：“These are available on all paid plans.”
- 管理员原句：“On Team plans, all three are on by default. On Enterprise plans, Claude Whiteboards and Sheets are off by default, and Claude Data turns on by default on November 19, 2026.”
- 一手来源：`https://claude.com/resources/articles/sheets-whiteboards-data`（Anthropic 官方产品更新）。存档：`d-d8-claude-sheets.html|.txt`。
- 发布时刻：“November 10, 2026”（官方未给时刻）。
- 级别：L1。
- 限定词原句：Claude Data 需要 “a data connector, such as a warehouse or CRM”；Sheets “For now, sheets have simple spreadsheet functionality, but we are working on adding more.”
- 热度：HN 50443716，170 分、137 条嵌套评论（14:32 BJ；`d-hn-50443716.json`）。

## 扫描说法勘误

以下是两份扫描回贴的说法与原文不符或需修正之处。已对照原文核实的一致项未列。

1. D5：扫描称“两个代理指标作者自认有噪声（附录 B）”。原文只说 CoT controllability（可控性）“noisy”（附录 B：“a noisy proxy”）；无 CoT 能力只被称为 “imperfect proxies”，未说有噪声。
2. D2：扫描称 curl 批报告“官方称 ‘fully model-generated, without human review or triage’”。台账与 VulnCheck 页未见该句，仅二手报道转述，未找到一手来源。
3. D2：扫描的“1,654（正文）/1,670（表格）”，正文 1,654 已核实；表格 1,670 由文本抽取推断，需目视核实。
4. D2：台账 “629 patched” 与 VulnCheck 的 “217 fixed” 并列记录（口径未统一）。扫描未提 629，若正文用“217”须注明口径。
5. D6：扫描 “46 个工具 MCP 比 CLI 贵约 17 倍（每千次任务）”成立，但仅限 Sonnet 5.5 与“一个服务器、一个任务”；原文 TL;DR 把 19,436 归到 Sonnet 5.5，正文脚本与后段归到 Haiku 5.5，二者相矛盾。
6. D7：扫描称 “通义 Qwen4-Max 预览”、X 帖 2026-11-11 02:01:09 UTC，时间与原文一致（已核实）；但 Qwen4-Max 名称未在其他一手页找到，WebSearch 只返回 Qwen3.8-Max-Preview 的旧报道，名称需再核。
7. D1：扫描称“650 万行”（论文 6.5 百万行）与“600 万行”（博文 “over six million lines”）两处并存，并列即可；扫描称 HN 606 分，抓取时为 608 分（14:32）。
8. D3：扫描称 HN 701 分，抓取时为 702 分；“performed just as well”是博文自述，测试集是另一组工单，原文并未说 98% 是同一测试集的结果（“98% less cost” 指成本）。
9. D4：扫描称 “10%→60% 等数字分属不同实验”，核实正确（MNIST 小网一组）。
10. D8：扫描称 HN 168 分，抓取时为 170 分；其余一致。
11. D4 与 D5：扫描未提 arXiv 提交日（6 Nov）与博文日（10 Nov）的差异，两者都已记录，正文不需要用 arXiv 日期。

## 未找到一手来源

- D2：“Anthropic 官方称 ‘fully model-generated, without human review or triage’”。
- D7：Qwen4-Max 的官方博客或模型卡。

## 待删除文件

无。
