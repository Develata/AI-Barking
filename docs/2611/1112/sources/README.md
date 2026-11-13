# 1112 来源与制作状态

期号1112（上期1111+1）；线索为 Develata 回贴的两份扫描：HTML 选题扫描 1113-18（北京 11-13 18:02）与 ChatGPT stateful rescan（北京 11-12 01:12，`AI吠点-邮件-2026-11-12-0129.md`，其主候选玛雅、Ring、AlphaGenome 2 等已在 1111 期发出或收入速览）。Claude 对照“已发过”与“近 3 天速览已收”去重、现查 HN 热度后列候选：扫描的 02“Fable 5.2 降价反贵”与 06“Fable 与 Mythos 同一模型”是 1109 期主帖，不算新题。Develata 选定 OpenAI 失准报告 + Gemini Hydrogen/Orchestrator + Edge0-145B 三条主帖。

派工单 `.handoff/2026-11-12-1112-evidence.md`（文件名日期是 `barking handoff` 按期号与美东算的；派工实际在北京 11-13 约 22:28，派工单正文的时区行已改为北京时间）。四组执行方：A、B、C 为 Claude Sonnet 5.5（`claude-sonnet-5-5`，harness 计约 15、57、16 分钟），D 为 Claude Haiku 5.5（`claude-haiku-5-5`，15 分钟）。取证全由 Claude 系代理完成，反向核验由 Codex（gpt-6-astra）核对事实清单与存档。

## 主帖

1. OpenAI 失准报告：[第 25 份“Stopping a running alignment evaluation”](https://alignment.openai.com/misalignment-reports/stopping-a-running-alignment-evaluation/)、[报告索引](https://alignment.openai.com/misalignment-reports/)、[11/4“Covert underperformance on safety research tasks”](https://alignment.openai.com/misalignment-reports/covert-underperformance-on-safety-research-tasks/)、[11/10 集群越权报告](https://alignment.openai.com/misalignment-reports/using-peers-work-to-gain-unauthorized-access-to-openai-research-infrastructure/)、[DEV 社区文章](https://dev.to/yehechuhai/gpt-6-astra-invented-a-safety-rule-then-shut-down-its-own-alignment-evaluation-1j8l)、[HN 50376313](https://news.ycombinator.com/item?id=50376313)（11/4 报告，342 分）。
   - 第 25 份报告在 HN 上没有帖（Algolia 按 URL 查，北京 11-13 22:10）；热度靠 AIHOT 收录与 TokenPost、DEV、ChainCatcher 等当日成稿，AIHOT 只作线索。
   - 扫描说“虚构限制是模型自己的说法、不是 OpenAI 的结论”“详情页两轮超时”都不对：OpenAI Summary 自己写了模型承认编出限制；curl 2 秒内取到。
   - 11/10 集群越权报告（7/19 ExploitGym 智能体在研究集群 CI 中以 root 执行命令）与 11/4 报告的 10%/16% 都没进正文（字数），已存档。
   - The Decoder（11/5）把 11/4 报告写成“已把原因追到训练数据、去掉就好了”，原报告写未能查明确切原因；第一版吠点③收过这条，因字数删去。
2. Gemini 4 Hydrogen 与 Orchestrator：[Antigravity 博客](https://antigravity.google/blog/gemini-4-hydrogen-in-google-antigravity)、谷歌主博客（`b-google-blog.txt`，Hydrogen 发布文）、[Antigravity 模型文档](https://antigravity.google/docs/models#orchestrator)、[方法页](https://deepmind.google/models/evals-methodology/gemini-4-hydrogen)、AA 榜、[HN 50471431](https://news.ycombinator.com/item?id=50471431) 与 [50472260](https://news.ycombinator.com/item?id=50472260)。
   - 同一句话两处版本号不同：Antigravity 博客 Terminal-Bench 4.1、谷歌主博客 4.0；主博客 `dateModified` 晚于发布约 3 小时，是否改过版本号未查 Wayback。
   - 谷歌跑分表里 Terminal-Bench 4.1 一行，Claude Sonnet 5.5 63.9% 高于 Hydrogen 57.0%（谷歌自测，cheating-adjusted）；因字数未进正文。
   - 扫描的“CursorBench 54.3%”未找到一手来源；“官方 AA 51 vs 实测 54”是把 high 与 xhigh 两档并列，二者不矛盾。
   - 文档 Enterprise 列 Orchestrator 为 ❌，博客写企业可用 “both”，冲突未消解，正文不写企业。
3. Edge0-145B：[Edge0 博文](https://edge0.ai/blog/introducing-edge0-145b-frontier-ai-capabilities-on-your-own-computer)、[GitHub 仓库](https://github.com/Edge0-AI/Edge0)、[HF 模型卡 Edge0-145B-NVFP4](https://huggingface.co/Edge0/Edge0-145B-NVFP4)、[Qwen3.8-Flash-Next 模型卡](https://huggingface.co/Qwen/Qwen3.8-Flash-Next)、[HN 50475713](https://news.ycombinator.com/item?id=50475713)。
   - 扫描标题“苹果本跑不了”与博文相反（博文点名 MacBook Pro、M 系列首日支持），与仓库相符（平台表 macOS 一栏“—”，文档称 M2 Ultra 测速不代表 macOS 已发布）；正文只写两者的出入。
   - “衍生自 Qwen3.8-Flash-Next”出自 README、模型文档、HF `base_model` 与 NOTICE；衍生方式未写。“不是新训练”未找到一手来源，正文不写。
   - 29GB 在仓库与 HF 存档里零命中；博文另一处写 28GB；仓库 NVFP4 实测 23.25 GiB 显存 + 38.27 GiB 内存（RTX 5090 D 24 GB）。博文的 80+ tok/s、约 120W 没写机型。
   - 博文与仓库对 120B 的两项分数、模型元数据的激活参数（6.6B 对 10.12B）不一致，记在 `evidence-c.md`，未进正文。
   - 第三方实测只有 HN 4 条自述（L6），Reddit 被登录页拦。

## 速览

- 腾讯混元 Hy Realtime：腾讯混元官方公众号（北京 11-13 16:32，`d-wx-hy-official/`；mp 链接见 `d-wx-official-url.txt`）。扫描写的“自建基础智商测试 61.01 领先豆包 40.63”与原图不符：图中 61.01 是 Gemini-3.8-Flash（low think），Hy Realtime 39.33（Claude 看原图核对）。速览只写上线事实。
- Claude 服务出错：[状态页](https://status.claude.com/incidents/nn11wk485zfn)，HN 50481187（346 分）。
- 未收：ElevenLabs Scribe v3（HN 2 分；扫描把第 1 名写成 Qwen3.5 Omni Flash，实为 Fun-Realtime-ASR-preview）、DeepMind × 奔驰机器人试点（HN 6 分）、Anthropic IPO 定价区间（融资类）、黄仁勋相关（一手未取到、风控）、Microsoft Lazarus、NVIDIA GR00T 2、Meta Muse Ember（窗口外）、ChinaTalk 报道（风控）。

## 备注

- B 组截图 13–24 用尽后溢出到 37–38。
- 各组回报列出的待删文件（重复抓取、质询页、404 页、作废截图与搜狗中间件）在交付前统一删除；fetch-log 里仍留有这些条目的记录。
