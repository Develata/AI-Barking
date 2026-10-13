# 1012 事实核验

取证：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high），详见 `evidence-a.md`–`evidence-d.md`。Claude 抽查了决定性条目的一手页面（Codex 源码 `features/src/lib.rs`、issue #48195、Anthropic 原文、Vals 模型页、arXiv、xAI、AA）。日期按北京时间。

## 主帖一：Codex 关窗还在跑

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 开发者9月26日在 GitHub 报告：0.157.0 起，打开交互式 Codex 就会启动后台进程，关掉所有窗口仍在跑 | L6（用户报告，页面一手） | ✅ | https://github.com/openai/codex/issues/48195 | createdAt 2026-09-25T20:04:30Z = 北京 9/26 04:04。“Since 0.157.0, just opening an interactive `codex` session: … keeps running after every Codex window is closed (reparented to PID 1) until `codex app-server daemon stop` or a reboot.” 以“开发者报告”署名。默认开启自 #47179（随 0.157.0）。 |
| 它复制约374MB 程序包运行，每小时自查升级 | L6 / L1 源码 | ✅ | 同上；`update_loop.rs` | “~374 MB”为报告者测量；源码 `INITIAL_UPDATE_DELAY = 5*60 s`、`DEFAULT_UPDATE_INTERVAL_MINUTES = 60`（evidence-a A5.1）。 |
| 暂无 OpenAI 成员回复 | L1（GitHub API） | ✅ | issue 评论 JSON | 16 条评论 authorAssociation 全为 NONE（北京 10/13 19:26、20:04 两次）。etraut-openai 在相邻 #41188 回复过更新器设置，未在本 issue。 |
| 可用 codex features disable daemon_auto_start 关掉 | L1 源码 | ✅ | `cli/src/main.rs` 第 1011–1012 行；issue 正文 workaround | “With the flag off, a new TUI session no longer spawns a daemon.”；`--no-daemon` 也可（cli.rs 第 87–89 行）。 |
| 0.157.0 更新说明提了一句，但没说常驻、复制和自动更新 | L1 | ✅ | https://github.com/openai/codex/releases/tag/rust-v0.157.0 | “Enabled automatic background-server startup for eligible interactive sessions, with recovery choices when server settings are incompatible.” |
| 官网配置文档也查不到开关 | L1 | ✅ | https://developers.openai.com/codex/config-reference | 全文搜 daemon/auto_start/no-daemon 0 命中（北京 10/13 19:34）。 |
| 后开的会话沿用首个会话的环境变量（如代理），官方说明写明“不提供隔离” | L1 | ✅ | `codex-rs/app-server-daemon/README.md`（commit 90f00bd） | “Shared clients use the environment inherited when the daemon started … per-client environment isolation is not provided.” 代理例子来自 issue 正文（HTTPS_PROXY）。 |
| 本号机器上旧版本也没被清掉，已攒5个版本、约2GB（体感、样本小） | 本号观察 | ✅ | `m-local-codex-observation.md` | WSL 内 0.156.1–0.159.3 共 5 个副本、`du` 2.0G。源码无清理旧 release 的逻辑（A5.6），#52067 另报 2.8 GB。 |

## 主帖二：电视盒里的 AI 蠕虫

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 10月12日发文 | L1 | ✅ | https://www.anthropic.com/research/self-replicating-ai | 页面标 Oct 12, 2026；元数据 2026-10-12T19:30Z = 北京 10/13 03:30（正文按页面日期写）。 |
| 把一个260亿参数模型拆到15台约55美元的电视盒上，每秒约0.39个 token | L1 | ✅ | 同上 | “We run Gemma4-26B-A4B, distributed over 15 inexpensive off-brand Android TV boxes (~$55 each) that we bought and networked together, at 0.39 tokens per second.” |
| 小模型或许“几个月后”就能在这类设备上自我复制 | L1 | ✅ | 同上 | “small AI models may be months away from being capable enough to self-replicate on commercial internet-of-things (IoT) devices, like smart ovens and off-brand Android TV boxes.” |
| 量子位标题写“实测AI全自主复制进智能家电” | L4 | ✅（标题原样） | https://www.qbitai.com/2026/10/502920.html | 标题“Claude团队又搞事：实测AI全自主复制进智能家电”，北京 10/13 14:19。 |
| 电视盒只做了推理演示 | L1 | ✅ | 同 Anthropic | 演示为 Gemma4-26B-A4B 分布式推理，16 token 回答；原文未称电视盒被感染。 |
| 复制在8台云上模拟安卓节点里做，端口不设认证 | L1 | ✅ | 同上 | “a firewalled VPC of 8 GCP nodes running redroid, an Android 15 emulator”；“there’s no authentication over the ADB port”。 |
| 原文称这是现实成功率的“上限”，也提醒真实设备未必更难攻破 | L1 | ✅ | 同上 | “which makes this an upper bound on real-world success.”；“real IoT security might not be dramatically more difficult than the trivial step in our evaluation.” |
| 测试的开放模型中，只有7000多亿参数的 GLM-5.2 跑通全流程，且跑在外部服务器上 | L1 | ✅ | 同上 | “Only GLM-5.2 out of our set of tested open-weight models is capable of full end-to-end replication”；原文图标 744B，HF 元数据 753.3B，均在“7000多亿”内；“The worm’s model runs on a separate inference server”。 |
| 9B、26B 小模型一台都没复制成 | L1 | ✅ | 图 07 | Qwen3.5-9B “0 of 8 nodes serving in 5/5 runs”；Gemma4-26B-A4B “0 of 8 nodes serving in 6/6 runs”。 |
| “年底小模型达标”是趋势外推的粗估 | L1 | ✅ | 同上 | “the trend line puts a small model at that level by the end of 2026 … this is only a rough estimate”。 |

## 主帖三：GLM-Legal 的“可靠”

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 智谱10月12日发布法律模型 GLM-Legal，目前需申请试用 | L1 | ✅ | https://z.ai/blog/glm-legal ；https://z.ai/consultation/legal | 页面“2026-10-12”；“Join the GLM-Legal Beta … Apply for API access”。Vals 标“open-weight”，HF/ModelScope/GitHub 未见权重（正文未写）。 |
| 据 Vals 评测，它在 Harvey 法律智能体测试中全过率19.17%，79个模型中排第7，每题成本0.15美元、前十最低 | L3 | ✅ | https://www.vals.ai/models/zai_glm-legal ；https://www.vals.ai/benchmarks/hlab | “#7 of 79 on Harvey’s Legal Agent Benchmark (19.17%) at $0.15 per test, the cheapest model in the top 10.” |
| 法律智能体测试全过率19.2%，官方称这证实“可靠” | L1 | ✅ | z.ai 官方页 | “GLM-Legal scores 19.2% … Under Harvey's strict all-pass grading, this confirms reliable end-to-end task execution without intermediate degradation.” |
| 约八成任务没有全部达标，评分的是两个 AI 模型 | L3 | ✅ | Vals hlab 方法段 | 100% − 19.17% = 80.83%；“A task passes only if 100% of its criteria pass”；“The two judges were GPT 5.5 and Claude Sonnet 4.6.” |
| 官方对比图只放5个模型，榜上排它前面的6个都没放 | L1 / L3 | ✅ | 官方图；Vals hlab 榜 | 图：GLM-Legal、GLM-5.3-Flash、Kimi K3、Claude Fable 5、GPT-6 Astra；Vals 第 1–6：Muse Spark 1.2、1.3 Max、1.3、Wonder Full、Muse Spark 1.1、Gemini 4 Argon。 |
| Vals 美国法检索测试中它得41.83%，低于自家基座的45.19% | L3 | ✅ | https://www.vals.ai/benchmarks/legal_research | Legal Research Bench（“Agentic US legal research”，all-pass）：GLM 5.3 Flash 45.19%（第17）、GLM Legal 41.83%（第24）；官方页称基于 GLM-5.3-Flash。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据 AA 评测，Muse Wonder 指数52，每题成本为前代2.8倍 | L3 | ✅ | https://artificialanalysis.ai/articles/muse-wonder | “scores of 52 and 56 at max reasoning effort”；“$4.52 per … task, 2.8x Muse Spark 1.3 (max, $1.60)”。指数为 AA 智能指数 v4.3、max 档。 |
| xAI 合并订阅：Grok 与 Cursor 共用额度，第三方模型另计费 | L1 | ✅ | https://x.ai/news/x-subscription | “a generous pool of Grok model usage shared across Grok and Cursor”；“third-party models are still available in Cursor on Premium and above. Usage is billed separately at API list prices, through top-ups.” 10/12。 |
| 开源地图软件 QGIS 新规：核心代码仓库不接受 AI 辅助的代码贡献 | L1 | ✅ | https://github.com/qgis/QGIS-Enhancement-Proposals/blob/master/qep-408-ai-tool-policy.md | “No LLM, AI or agent assisted code contributions are allowed in the https://github.com/qgis/qgis repository.” 合并北京 10/08 05:00（窗口前），HN 10/12 376/403。 |
| 论文称可在模型权重里藏入加密能力，仅秘密触发时运行；实验为小任务 | L3 | ✅ | https://arxiv.org/abs/2610.13928 | “embed encrypted neural networks within host model weights, decrypting and executing them only upon receiving a secret trigger”；“Our proof-of-concept hides only a small binary reaction-feasibility classifier”。arXiv 北京 10/10 01:55。 |
| 据 AA 评测，AI 做3D场景，Sonnet-5.5 居首但与另三款统计并列 | L3 | ✅ | https://artificialanalysis.ai/articles/aa-3d | “Claude Sonnet 5.5 is first at 1194, statistically tied with three other Anthropic and OpenAI models.” |
| Epic CEO 称公司正大幅转向 AI 加速开发 | L2 | ✅ | Tim Sweeney 海外社媒帖 status/2109705826688549304（`d-d8-sweeney-fx.json`） | “We have a huge pivot underway here at Epic, moving developers around to retool for maximum AI acceleration.” 北京 10/13 02:00。迪士尼合作游戏推迟只经 Ars/gamesindustry 转述彭博社，未收。 |

## 批注译注（`images/cards.toml` 的 gloss，措辞以此为准）

| 图 | 原文 | 译注 |
|---|---|---|
| 41 | keeps running after every Codex window is closed | 关掉所有 Codex 窗口后仍在运行 |
| 41 | checks for updates 5 minutes after start and then hourly | 启动5分钟后检查更新，之后每小时一次 |
| 42 | Enabled automatic background-server startup for eligible interactive sessions | 为符合条件的交互会话启用后台服务自动启动 |
| 43 | is experimental and its lifecycle contract may change | 仍属实验性，其生命周期约定可能变化 |
| 43 | per-client environment isolation is not provided. | 不提供按客户端的环境隔离。 |
| 44 | which makes this an upper bound on real-world success. | 因此这是现实成功率的上限。 |
| 44 | so real IoT security might not be dramatically more difficult than the trivial step in our evaluation. | 所以真实物联网设备的防护，可能并不比我们评测里这一步难多少。 |
| 45 | this confirms reliable end-to-end task execution without intermediate degradation. | 这证实了可靠的端到端任务执行，中途没有退化。 |
| 46 | GLM 5.3 Flash 45.19%（第17）/ GLM Legal 41.83%（第24） | 基座 GLM-5.3-Flash：45.19%，第17名 / GLM-Legal：41.83%，第24名 |

## 视觉复核

未做跨模型视觉复核：Develata 要求尽快发布（2026-10-13），Claude 逐张目视检查了全部卡片与封面（含封面放大核字、看板娘形体）。

## 反向核验（发布后完成）

codex-reviewer（WSL Codex gpt-6-astra，medium，约 15 分钟）在 Develata 发布之后才返回，原文存 `m-reverse-review.md`。结论 1 项 BLOCKER、7 项 SHOULD_FIX，Claude 判断如下（发布稿不改，按纠错规范在评论区补充，见 `../CORRECTION.md`）：

| # | 意见 | 判断 |
|---|---|---|
| 1 BLOCKER | 省流“自我复制只在8台模拟设备上做成”与正文②易被读成模型复制了自身权重；实际蠕虫模型在外部服务器，节点上部署的是 Llama3.1-8B 载荷，下一份蠕虫由实验框架启动 | 采纳。评论区更正 |
| 2 | “关掉”只禁止后续自动启动，不停止已在运行的进程 | 采纳。评论区补充 `codex app-server daemon stop` |
| 3 | 丢了“符合条件的交互会话”与“连接同一后台服务的会话”限定 | 采纳，并入更正 |
| 4 | AA 两条速览丢了“最高推理档”等测试条件 | 采纳，记录；速览图不重发 |
| 5 | xAI 速览未说明仅新付费订阅、第三方模型限 Premium 及以上 | 采纳，记录 |
| 6 | Anthropic 页面元数据为北京 10/13 03:30，正文写 10月12日 | 采纳，记录（页面标注日期为 Oct 12） |
| 7 | “未必更难攻破”丢了程度限定（“可能没有难很多”） | 采纳，记录 |
| 8 | “暂无 OpenAI 成员回复”应为“未见可识别的官方回复” | 采纳，记录 |
