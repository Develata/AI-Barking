# 0928 事实核验

取证：Codex（gpt-6-astra，medium），见 `evidence.md`、`capture-log.md`、`validation.md`；派工 `.handoff/2026-09-28-0928-evidence.md`。抽查：Claude 对照存档逐字核对下表每一行（2026-09-28，美国当地日期；北京时间已是 09-29）。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 9月28日 Anthropic 发布 Sonnet-5.5 | L1 | ✅ | anthropic.com/claude-sonnet-5-5 页首 “September 28, 2026”；`a-sonnet.md` | 原文日期无时刻、时区，正文照录。HN 首帖北京时间 09-29 01:58（17:58 UTC） |
| 与 Sonnet-5 同价，每百万 token 输入2美元、输出10美元 | L1 | ✅ | “priced the same as Sonnet 5 at $2 per million input tokens, $10 per million output tokens, and $0.20 per million tokens for cache reads” | 缓存读 $0.20 正文略 |
| 官方表 Terminal-Bench-4.0：Sonnet-5.5 70.6%，Opus-5.5 66.4%（Max 对 Xhigh 档） | L1 | ✅ | 发布页评测表；图 01；系统卡 pp.112–113（`a-system-card.txt` 约 L3141 起） | 均用 Claude Code --bare；Sonnet-5.5 为 Max 档，Opus-5.5 为 Xhigh 档（脚注1：其最高分；Max 档为64.8%）。两者都开 safeguards，fallback 涉及 trials 分别为1.5%、10%。AA 自测（mini-swe-agent）为 63.6% 对 59.6%，Sonnet-5.5 仍领先，正文未引。Sonnet-5 为10.3%，条件未交代，正文未引 |
| 官方同页称需持续判断的复杂开放任务 Opus-5.5 仍明显更强 | L1 | ✅ | “Opus 5.5 remains clearly stronger at complex, open-ended work requiring sustained judgment.” | 官方措辞含 “in our own testing, and in that of external testers” |
| FrontierCode：Max 档46.2% 低于 Xhigh 档52.1%；官方举例称多调代码审查致超时或改出任务范围 | L1 | ✅ | 评测表；脚注2 “more often ran Claude Code’s code-review skill … led to a timeout or to extra edits beyond the task’s scope” | 原因解释只来自 Cognition 检查的两个案例（“in two cases Cognition examined”），正文写“举例”，不写成全部分差的原因 |
| 9月28日作者公布剑桥 CASP 的22人合著论文 | L1（论文）/ L2（日期） | ✅ | casp.ac/reports/intelligence-explosion；PDF 物理第2页（印刷第1页）署名22人（`b-authors-transcription.md`） | PDF 只标 September 2026，网页无日级日期。日期依据是第一作者 Alan Chan 的 X 帖（北京 09-28 22:26:53，“In a new paper”，`b-author-x.json`），Guardian/Axios 北京 09-28 23:00 佐证；不代表 CASP 网页当天首次上线，故正文写“作者公布” |
| 作者含 Hinton、Bengio、OpenAI 的 Pachocki、Anthropic 的 Jack Clark | L1 | ✅ | PDF 物理第2页署名栏：Pachocki — OpenAI；Clark — Anthropic；图 03、03b | 是论文作者，不是背书签名 |
| 论文称 AI 有望几年内自动化大部分 AI 研发 | L1 | ✅ | 摘要：“AI systems are on track to automate most AI R&D work within a few years, and possibly all of it.” | |
| 初步证据显示多年进展可能压缩到数月以内 | L1 | ✅ | 摘要：“could AI progress radically accelerate … where years of advances are compressed into months or less? Preliminary evidence suggests that it could.” | “数月以内”对应 “months or less” |
| “数百万研究员”的前提：专家级研发能力、运行成本与今天相当；一家前沿开发者的算力才可能撑起相当于至少数百万顶尖研究员的 AI | L1 | ✅ | 印刷 pp.3–4：“Once AI systems reach expert-level AI R&D capabilities at runtime costs comparable to those of today’s systems, the compute available to a single frontier developer today could sustain an AI workforce equivalent to at least millions of top human researchers”；图 05 | |
| 目前增益“尚未达到”阈值，新系统“很可能正接近” | L1 | ✅ | 印刷 p.5：“Productivity gains from AI R&D automation have not yet reached the threshold needed to trigger an intelligence explosion, but gains from newer systems are likely approaching that threshold”；图 04 | |
| 论文注明观点不一定代表作者所在机构 | L1 | ✅ | PDF 物理第2页（印刷第1页）脚注：“The views presented in this paper are the authors’ and do not necessarily represent the views of the organizations with which they are affiliated.”；图 03b | |
| 9月27日小米复盘 MiMo-V2.6 反复发起相同工具调用 | L1 | ✅ | mimo.xiaomi.com 博客，页首“2026 年 9 月 27 日”；图 06 | 正文照录页面标注日期；博客无时刻、时区，无法唯一换算北京时间；官方 X 帖北京 09-28 00:45 |
| 官方称奖励只看答对，原规则单轮超32次调用才罚，其下的低效调用被训练放大 | L1 | ✅ | 官方 X：“when rewards only track final-answer correctness, inefficient behaviors along the way go unnoticed — and get amplified as training scales. Concretely, our flooding penalty only kicked in at >32 tool calls per turn”；博客 L203：“如果某一轮的工具调用超过 32 次，则对该条 rollout 执行 early stop，并将其 reward 直接置为 0” | 博客称 RL step 0 已有少量高并发调用，阈值过松是“推测”；故正文写“放大”，不写训练从无到有造出复读 |
| 修复版9月25日上线 API，模型名不变 | L1 | ✅ | 博客：“最新模型已于 9 月 25 日 06:00（UTC+8）后在 API 开放平台上线”；X：“names unchanged” | 北京时间 |
| 开头只写复读率“超过0.05%”；分场景看 OpenCode 上 Flash 达1.02% | L1 | ✅ | 博客开头 “Response 级别的复读率超过 0.05%”；分 harness 表 OpenCode Flash 1.02% / Pro 0.54%；图 06 | 英文版明写表格是按 harness 展开（“The table below breaks down the rates … across different agent harnesses”），不构成矛盾；吠点只指出开头的单一数字掩盖了场景差异 |
| 只算单轮内完全相同的调用，官方称是下界 | L1 | ✅ | “若两个调用的工具名相同，且参数经 JSON 规范化后完全一致，则将其视为重复”；“可观测复读的下界” | 不含跨轮、近似重复、code-mode exec 内部调用 |
| “成本仅4%”是9万美元修复对比231万美元重启训练估算 | L1 | ✅ | 博客：“需要重启 20 步 MixRL 训练，这个代价是巨大的（约 231 万美金）”“最终全程训练成本约为 9 万美金，仅为上一个方案 MixRL 训练成本的 4%”；图 08 | X 帖写 “roughly 4% of the cost of a full mixRL retrain”；博客具体口径为重启20步。“不是训练模型只要9万”是防误读的解释，未找到实际误读实例（evidence E-C） |

## 未进正文

- Meta Muse 读取 iMessage 争议（D 组）：已取证；Develata 决定本期不发——原始事件是 9/19 的，不算最新，授权链也要等更多实际进展。留作后续跟进，材料见 `evidence.md` D1–D9。
- Axios 正文 “will create an ‘intelligence explosion’”：该句位于订阅墙后，公开可见导语用的是 “could rapidly speed up”，不作为夸大实例。
- AA 独立测得 Sonnet-5.5 智能指数 56、Terminal-Bench-4.0 63.6%（A7）：篇幅原因未引。

## 反向核验

codex-reviewer（WSL Codex，gpt-6-astra，medium，只读，2026-09-28）初判不通过：2 项 BLOCKER、4 项 SHOULD_FIX、1 项 NICE_TO_HAVE。evidence.md 37 行引文逐行在存档中找到。Claude 逐条对照存档复核后全部采纳：

| # | 级别 | 问题 | 处理 |
|---|---|---|---|
| 1 | BLOCKER | “不罚中途低效动作”省略了原有“单轮超过32次才罚”的规则；结论“复读是练出来的”把放大写成起源 | ✅ 采纳。事实段改为“原规则单轮超32次调用才罚，其下的低效调用被训练放大”；小标题改“RL 放大‘复读’”；结论改“复读被训练放大” |
| 2 | BLOCKER | “两者关系未说明”过度：英文版明写表格按 harness 展开 | ✅ 采纳（`c-mimo.html` 含该句）。改为“开头只写……分场景看 OpenCode 上 Flash 达1.02%” |
| 3 | SHOULD_FIX | Sonnet 对比未交代档位；两者都开 safeguards | ✅ 采纳（系统卡核实）。正文补“（Max 对 Xhigh 档）”，小标题改“单榜分数超 Opus”；封面副标题由“只赢一个榜”改为“单榜领先” |
| 4 | SHOULD_FIX | FrontierCode 两个案例被写成全部分差的原因；“越界”易误读为权限越界 | ✅ 采纳。改“官方举例称……改出任务范围” |
| 5 | SHOULD_FIX | 日期混用页面标注、作者宣布与北京时间 | ✅ 采纳。CASP 改“作者公布”，本表日期行分列依据；Anthropic、小米照录页面标注日期（沿用 0927 做法），在本表注明无法唯一换算 |
| 6 | SHOULD_FIX | “撑起至少数百万顶尖研究员”与“可能正在接近”改变了原文确定程度 | ✅ 采纳。改“才可能撑起相当于至少数百万顶尖研究员的 AI”“很可能正接近” |
| 7 | NICE_TO_HAVE | PDF 物理页码与印刷页码混用 | ✅ 采纳。统一为“PDF 物理第2页（印刷第1页）” |

审查边界：审查方未能查看 PNG；截图内容由 Claude 目视核对（图 01–08、03b）。
