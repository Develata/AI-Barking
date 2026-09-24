# 0924 Fact Check

对象：`../doc_0924_publish.txt`（2026-09-23 重写版）。取证：Codex（`evidence.md`，行号 E#）。抽查：Claude 对照本目录存档逐字核对，标注“抽查”。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Opus 5.5 发布于 9 月 23 日凌晨（北京时间） | L1 | ✅ | [@claudeai](https://x.com/claudeai/status/2102435511222890900) | UTC 09-22 16:31:01 = 北京 09-23 00:31。E4a |
| 不到两小时后 OpenAI 发布 Sol / Luna | L1 | ✅ | [@OpenAI](https://x.com/OpenAI/status/2102460975790137662) | UTC 18:12:13 = 北京 02:12，相隔 1 小时 41 分。E4b |
| Opus 5.5 输入 $4、输出 $20，比 Opus 5 降 20% | L1 | ✅ 抽查 | [Anthropic 发布页](https://www.anthropic.com/claude-opus-5-5) | `anthropic-page.yml` L83 |
| 典型工作负载总成本低约 40% | L1 | ✅ 抽查 | 同上 | 原文 “at default settings … 40% less than Opus 5 on typical workloads” |
| 大多数工作达到 Fable 5.1 水平 | L1 | ✅ 抽查 | 同上 | L51；厂商概括，非全面等价 |
| Fable 5.1 仍在售，单价为 Opus 5.5 的 2.5 倍 | L1 | ✅ | [Fable 页](https://www.anthropic.com/claude/fable) | $10/$50 vs $4/$20。Fable 页仍称其 “most capable generally available model”（抽查），与 Opus 页 “new leading model” 并存，正文未对两者排位下结论。E2c/E2d |
| Sol $2/$10、Luna $0.10/$0.50 | L1 | ✅ | [OpenAI 价格页](https://developers.openai.com/api/docs/pricing) | Standard 短上下文。E1b/E1c |
| 比 GPT-5.6 促销价便宜 50% | L1 | ✅ 抽查 | [OpenAI 发布文](https://openai.com/index/introducing-gpt-6-sol-and-luna/) | 官方概括值。逐项算术：Luna 输出降约 58.3%，正文未展开 |
| AA：Sol/Luna 综合智能分与前代基本持平，单任务成本约减半 | L3 | ✅ | [AA 文章](https://artificialanalysis.ai/articles/gpt-6-sol-and-luna-push-the-cost-efficiency-frontier) | 原文 “remain level with GPT-5.6”“Halves Cost per Task”；按展示值 Sol −46.7%、Luna −61.1%。Luna 编码退步未写入正文。E6a–E6d |
| OpenAI 称要最好结果仍选 Astra | L1 | ✅ 抽查 | OpenAI 发布文 | “GPT‑6 Astra continues to be our best model across the board. Choose it when you want the best results” |
| Claude：Pro/Max/Team 获赠可保存的重置，10 月 22 日前使用 | L1 | ✅ 抽查 | [@ClaudeDevs](https://x.com/ClaudeDevs/status/2102438803013333469)、[帮助页](https://support.claude.com/en/articles/17007452-what-is-a-limit-reset) | 原帖未写截止时刻与时区。E3a/E3b/E3g |
| GPT：Plus/Pro/Business 获赠 Banked Reset，到期看账户 | L1 | ✅ | [社区第 4 楼](https://community.openai.com/t/announcing-gpt-6-sol-and-gpt-6-luna-in-the-api-codex-and-chatgpt/1399925/4)、[帮助页](https://help.openai.com/en/articles/20001498-how-banked-codex-resets-work) | 本次统一截止日未找到一手来源，故正文不写日期。E3d–E3f |
| AA 智能指数 Opus 5.5 58、Astra 53 | L3 | ✅ 抽查 | [AA 文章](https://artificialanalysis.ai/articles/claude-opus-5-5)、AA 首页截图 | Opus max with fallback；53 来自首页截图 `04-aa.png`。E5a |
| LiveBench Fable 83.4 / Opus 83.2 / Astra 82.2 | L3 | ✅ | [LiveBench](https://livebench.ai/) | 2026-06-25 版，Overall，三者 Max Effort。上一轮已核 |
| Anthropic 表 Terminal-Bench 4.0：Opus 66.4%、Astra 57.9% | L1 | ✅ | Anthropic 发布页脚注、系统卡 pp.177–178 | Opus xhigh + Claude Code；Astra high 为转引 OpenAI（OpenAI 报告的 max 略低，故取最高值）。E5c/E5d |
| AA 自测 Terminal-Bench 4.0 两者均 59.6% | L3 | ✅ 抽查 | [AA 文章](https://artificialanalysis.ai/articles/claude-opus-5-5)、[AA 方法页](https://artificialanalysis.ai/evaluations/terminalbench-4-0) | 同一 mini-swe-agent 框架；Astra xhigh。E5b |
| Anthropic 正文称 Terminal-Bench 4.0 与 Astra 打平、成本约四成 | L1 | ✅ 抽查 | Anthropic 发布页 | `anthropic-page.yml` L253 “matches Astra for about 40% of the cost” |
| Anthropic 称跑分差距已不太能代表真实差距 | L1 | ✅ 抽查 | Anthropic 发布页 | L90 “benchmark margins have become a less reliable guide to real-world differences” |

## 本期未采用的线索

- 系统卡“安全演习约半数有害”（E7a–E7c）：已核到原文，但必须连同“关闭防护、模拟环境、逐级诱导、按 investigation 统计”一起讲，篇幅不适合本期。候选独立选题。
- AIHOT 摘要“Arena 上线 Sol/Luna……预示新一代模型发布在即”：摘要逻辑有误，未采用。

## 反向核验（Codex，只读，2026-09-23）

采纳：补“默认设置”“标准价（短上下文）”“截止时刻看账户”“Codex 额度的 Banked Reset”；LiveBench 改为榜内分差表述；“差在测法”弱化为“测法不同、不能单因归因”；删“最有意思的是”；结论改为“水平相近，按 Anthropic 测算更省钱”，不把厂商成本说法写成通用事实。

未采纳：“数字超过 1–3 个”——性能小节每段仍只保留 1–3 个关键数字，三段对比本身就是本期吠点；“所以——”保留为账号固定节奏。

## 2026-09-24 增补：GPT 产品线“名字没变，座位变了”

取证：`evidence-lineup.md`（Codex）。AA 数值均为 Intelligence Index v4.3.2、max；价格为 Standard 短上下文。

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| “比 GPT-5.6 同名型号的促销价再便宜 50%” | L1 | ✅ | OpenAI 发布文 | 官方价格对照为 5.6 Sol→6 Sol、5.6 Luna→6 Luna |
| GPT-5.6 为 Sol / Terra / Luna 三档 | L1 | ✅ | [GPT-5.6 首发稿](https://openai.com/index/gpt-5-6/)、Terra 模型页 | 另有受限的 GPT-5.6 Cyber，非通用档，正文不计 |
| GPT-6 为 Astra / Sol / Luna | L1 | ✅ | [GPT-6 指南](https://developers.openai.com/api/docs/guides/latest-model)、价格页 | |
| 6 Sol 价格落在原 Terra 位置（Terra $2/$12） | L1 | ✅ | [价格页](https://developers.openai.com/api/docs/pricing) | 6 Sol $2/$10。Terra 现价为 07-30 降价后价格，Terra 首发 $2.50/$15 |
| AA 智能分 6 Sol 48、5.6 Sol 47 | L3 | ✅ | AA 比较页 [A6]、[A56] | 两个数值不在同一张页面，但同为 v4.3.2、max |
| Astra $10/$50，为 5.6 Sol 现价 2.5 倍 | L1 | ✅ | 价格页 | 5.6 Sol 现价 $4/$20 为优惠价（至少持续至 11-21）；对首发价 $5/$30 为 2 倍 / 1.67 倍 |
| Luna 智能分持平，单任务成本降约六成 | L3 | ✅ | [AA Luna 比较页](https://artificialanalysis.ai/models/comparisons/gpt-6-luna-vs-gpt-5-6-luna) | 37 对 37；$0.18→$0.07，−61% |

Develata 原始假设中未写入正文的两条（数据不支持）：

- “6 Sol 比 6 Luna 强不多”：AA 48 对 37，Terminal-Bench 4.0 44% 对 13%；6 Sol 离 Astra（53）更近。
- “除 Luna 外都是变相涨价”：中档不成立，6 Sol 比 5.6 Terra 分高（48 对 42）、单任务成本低（$1.06 对 $1.40）；只有顶档成立，Astra 单价为旧旗舰现价 2.5 倍，但分数也高 6 分。

反向核验（Codex，只读，2026-09-24）：采纳“Terra 现价”（避免误读为首发价）与“Astra 分数高 6 分”（避免暗示同能力换名涨价）。未采纳将“降价 50% 没说错”改为“约五成或更多”——Luna 输出降幅 58% 大于 50%，官方说法不构成夸大，“没说错”成立。

## 2026-09-24 增补：“实际用起来呢？”

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 聊天、写代码 Opus 5.5 更顺手；同档订阅 Claude 能干的活更多；数学上 GPT 明显更强 | 体感 | — | Develata 及朋友的使用体验 | 正文已标“体感，样本小，仅供参考”。Develata 原话为“GPT 数学几乎碾压”“Claude Lean 断档优势”，正文未用“碾压”“断档”。数学对比的数据核验见 `.handoff/2026-09-24-usage-evidence.md` 任务 D，未完成 |
| 本月初 Anthropic 用接近 Fable 5.1 的内部模型，11 天完成费马大定理完整 Lean 形式化 | L1 | ✅ | [Anthropic Research](https://www.anthropic.com/research/formalizing-fermats-last-theorem) | 2026-09-04；原文 “a general-purpose internal research model roughly comparable to Claude Fable 5.1”；13M 行 Lean、仅用 Lean 三条标准公理；建立在 Buzzard 的 FLT 项目、flt-regular、Mathlib 及 Prove2Me 之上。**不是 Opus 5.5 完成的**。WebFetch 摘要读取，未存档原页 |

线索（L5，未核实）：搜索摘要称 Epoch FrontierMath Tier 4 (v2) 上 Fable 5.1 约 87.8%、Opus 5 约 73.2%、Astra 约 98%；Sawhney 与 Sellke 用 OpenAI 模型推进球堆积等开放问题。Epoch 页面注明 FrontierMath 由 OpenAI 资助且 OpenAI 可独占访问部分题目（已读原页，2026-09-24）。

## 2026-09-24 增补：实际使用证据（替换上节体感为主的写法）

取证：`usage-evidence.md`（Codex，交接运行）。Claude 验收：验证命令复跑一致（1122 / 310 / 20，jsonl ok，盲审文件无编码字段）；原有文件修改时间未变。

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| X 9 月 22–24 日 310 条第一手帖；直接比较 73 条：43 偏 Opus、12 偏 Astra、18 各有胜负 | L6 抽样 | ✅（数据层面） | `usage/x_coded.jsonl`、`x_stats.json` | 时间窗 09-22 16:00 UTC 至 09-24 16:43 UTC；每检索上限 100，第 8 组 top 遇 429；单一编码者。Wilson 95%：偏 Opus 58.9% [47.4, 69.5]。**Claude 盲审复编码 20 条**：verdict 20/20 一致（其中直接比较 5 条）、Opus 情感 18/20、Sol 20/20、Luna 20/20 |
| Arena WebDev：Opus 5.5 第一、Astra 第二，区间重叠 | L3 | ✅ | [Arena WebDev](https://arena.ai/leaderboard/code)，`images/30-arena-webdev.png` | 页面 09-23；Opus max 1818 [1797, 1839]、1219 票；Astra max 1792 [1780, 1804]、4325 票；两者 rank spread 均 1–2 |
| FrontierMath Tier 4 v2：Astra 97.6%、Fable 5.1 87.8%，Opus 5.5 未测 | L3 | ✅ | Epoch 官方 CSV `usage/epoch-frontiermath_tier_4_v2.csv`，`images/32-frontiermath.png` | Astra medium–max 均 97.6 ± 2.4（SE），Fable 5.1 max 87.8 ± 5.2（SE）；误差为标准误非 CI。Tiers 1–3：Astra 93.7、Fable 5.1 90.2 |
| FrontierMath 由 OpenAI 资助，部分题目 OpenAI 独占访问 | L3 | ✅ | Epoch 榜页原文 | Claude 亲读："FrontierMath was developed with funding from OpenAI, who has exclusive access to a subset of the benchmark." 不能推出训练污染 |
| 体感：写代码 Opus 5.5 更顺手，同档订阅更耐用；数学 GPT 强 | 体感 | — | Develata 及朋友 | 与 X 抽样、FrontierMath 方向一致；正文以数据为主，体感置后 |

删减：LiveBench 段、“Fable 5.1 单价是它的 2.5 倍”、“Anthropic 也承认跑分差距……”三句为控制篇幅删除（事实本身仍 ✅）。

X 抽样附带观察（未写入正文）：GPT-6 Sol 情感 88 条中负面 54 条（61.4% [50.9, 70.9]）；有帖子独立提出“6 Sol 只是 5.6 Terra 的升级版”（id 2102957703815717091），与本期“座位”一节方向一致，属 L6 观点。

修订：FrontierMath 句改为与 Claude 最高分对比——截图 `32-frontiermath.png` 显示 Claude Fable 5 (max) 90.2% ± 4.6 为 Claude 系最高，高于 Fable 5.1 的 87.8%，避免挑低分对照。`32-frontiermath.png` 右下角有 Cookie 弹窗，上传前建议裁掉。

## 反向核验（codex-reviewer，gpt-6-astra xhigh，只读，2026-09-24）

8 条发现，Claude 逐条对照数据核实后：

| # | 发现 | 核实 | 处理 |
|---|---|---|---|
| 1 | X 抽样 mixed 类不成立（#265 未比较；#81 是“接近”） | 成立。Claude 复查全部 73 条比较帖：明确误编 3 条（#213 比的是 Sol→na；#265→na；#224 “smoked astra”→prefer_opus），另有约 8 条存疑（#59、#80、#92、#119、#183、#223、#271、#305）。修正后约 43–44 / 12 / 15，分母约 70 | 正文改为约数：“约 70 条里，过半偏 Opus，不到两成偏 Astra，其余觉得各有取舍或差不多”；方向对编码误差稳健 |
| 2 | 结论把便利样本与其他 Claude 型号成绩外推 | 成立 | 结论改为“抽样口碑偏它；数学上，已测的 Claude 都不如 Astra” |
| 3 | “与 Astra 打平，成本约四成”省略 effort | **成立且重要**：该说法对应 Opus medium 57.6%（$2.94）对 Astra high 57.9%（$7.21），与表格的 Opus xhigh 66.4% 不是同一配置，并非厂商自相矛盾（`anthropic-page.yml` L299–323） | 改为“默认强度的 Opus 与高强度 Astra 打平，成本约四成” |
| 4 | Terminal-Bench 两组省略 effort 差异 | 成立 | 正文改为“两组框架和推理强度都不同”。封面副标题“换个测法”中的“测法”已涵盖设置差异，未改 |
| 5 | “9 月 22–24 日”用了 UTC | 成立：北京时间覆盖 23 日 78 条、24 日 175 条、25 日凌晨 57 条 | 改为“发布后两天” |
| 6 | FLT 发布日写成完成日；不能据此建立对 GPT 的 Lean 优势 | 成立：完成于 8 月 18 日（`usage/anthropic-flt.txt` L49），9 月 4 日公布 | 改为“Anthropic 本月初公布……11 天完成”；引导语改为“Claude 的数学高光在 Lean”，不作比较 |
| 7 | 体感漏“样本小”，本表却称已标注 | 成立（压缩篇幅时误删） | 已补“（样本小）” |
| 8 | Luna“单任务成本降约六成”未注明 AA 口径 | 成立 | 改为“AA 测的智能分持平，测试任务成本降约六成” |

附带更正：此前内部沟通称“Anthropic 同一页面有两个口径、自相矛盾”不准确，见第 3 条。

## 存档清理（2026-09-24，经 Develata 授权）

删除 138 个中间文件（约 23MB）：系统卡 PDF（原文见 [官方 PDF](https://www-cdn.anthropic.com/fc1b44717c85dc068bc6ba5024219938094694bd/Claude%20Opus%205.5%20System%20Card.pdf)，可检索文本 `opus-5-5-system-card.txt` 与截图 11、12 保留）；调试元数据；`usage/` 下逐次 X 检索原始返回（已合并入 `x_raw.jsonl`）、编码草稿 `x_review.txt`、浏览器与网络调试记录、与本期无关的 Epoch 评测 CSV（仅保留 FrontierMath Tier 4 v2、Tiers 1–3 v2、Erdős）。`evidence.md`、`usage/new-files.txt` 中对这些文件的引用以此为准。

## 1000 字版（2026-09-24，平台上限）

压缩到标题+正文共 987 字（含空格换行）。只删减、不新增事实：删去额度重置细节行、Arena 一句、“Anthropic 正文另一口径”一句、X 抽样“其余各有取舍”分句；“标准短上下文价”简为“标准价”；FLT 句改为“其内部模型”（仍非 Opus 5.5）；“部分题目它独占访问”改为“部分题只有它能看”（指 OpenAI 对部分题目的独占访问）。以上各项依据见前文各表。未再跑 codex-reviewer（无新增事实）。

标题改为“Opus 5.5 完爆 Astra？”（18 字），满足标题 20 字硬上限；原标题 24 字。封面图上的标题不受此限，保持不变。
