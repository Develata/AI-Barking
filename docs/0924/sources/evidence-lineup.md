# GPT-5.6 / GPT-6 产品线取证

取证日期：北京时间 2026-09-24，新增浏览器读取约 14:36–14:44。只记录事实、原始数值、来源和缺口，不裁定编辑假设。L1 为 OpenAI 官方材料；L3 为 Artificial Analysis（AA）对自身评测的材料。本文不把厂商定位、AA 的比较措辞视为独立证明。

优先读取了任务指定的五份存档：`openai-pricing-expanded-rows.json`、`aa-sol-luna-article.json`、`aa-sol-luna-current.json`、`aa-opus-article.json`、`openai-launch-browser.json`。随后通过官方网页检索及 OpenCLI 浏览器补查。新增页面的关键字段快照嵌入本文；没有另建存档、截图或修改其他文件。新增截图数为 0。

## 型号、价格与 AA 数据

价格单位均为美元 / 100 万 token，顺序为“未缓存输入 / 输出”，处理档位为 Standard，非 Batch、Flex 或 Fast。当前 Standard 价格可以同时处于促销期，不能把“Standard”理解成“促销前原价”。不包含工具调用费或地区附加费。

AA 单任务成本单位为美元 / Intelligence Index task，是按各评测权重汇总的任务成本，包含不同 token 类别的使用量与价格；不是固定输入输出配比的每百万 token 价格，也不是 Coding Agent Index 单任务成本。

| 型号 / API ID | 当前 Standard 短上下文：输入 / 输出 | 当前 Standard 长上下文：输入 / 输出 | AA Intelligence Index | AA 单任务成本 | AA 实际展示 effort | AA 版本 / 取证时点 | 来源、级别与状态 |
|---|---:|---:|---:|---:|---|---|---|
| GPT-5.6 Sol / `gpt-5.6-sol` | $4 / $20（促销） | $8 / $30（促销） | 47 | $1.99 | max | v4.3.2；09-24 | 价格 [P]、[S56]，L1；AA [A56]，L3；已找到 |
| GPT-5.6 Terra / `gpt-5.6-terra` | $2 / $12 | $4 / $18 | 42 | $1.40 | max | v4.3.2；09-24 | 价格 [P]、[T56]，L1；AA [A56]，L3；已找到 |
| GPT-5.6 Luna / `gpt-5.6-luna` | $0.20 / $1.20 | $0.40 / $1.80 | 37 | $0.18 | max | v4.3.2；09-24 | 价格 [P]、[L56]，L1；AA [AL]，L3；已找到 |
| GPT-6 Astra / `gpt-6-astra` | $10 / $50 | $20 / $75 | 53 | $3.26 | max | v4.3.2；09-24 | 价格 [P]、[AST]，L1；AA [A6]，L3；已找到 |
| GPT-6 Sol / `gpt-6-sol` | $2 / $10 | $4 / $15 | 48 | $1.06 | max | v4.3.2；09-24 | 价格 [P]，L1；AA [A6]，L3；已找到 |
| GPT-6 Luna / `gpt-6-luna` | $0.10 / $0.50 | $0.20 / $0.75 | 37 | $0.07 | max | v4.3.2；09-24 | 价格 [P]，L1；AA [AL]，L3；已找到 |
| GPT-5.6 Cyber / `gpt-5.6-cyber`（受限安全专用型号，补充列出） | $12.50 / $75 | 官方价格表为“—”；未找到长上下文价 | 未找到 | 未找到 | 未找到 AA 展示 | 未找到 | [P]、[CY]，L1；价格及型号已找到；AA 未找到一手来源 |

表内六个通用型号均采用 AA 页面实际显示的 max；未用其他 effort 补位。三个当前比较页均标明 v4.3.2，但不是六型号同时出现在同一张页面：Sol/Terra 使用 [A56] 同页，Astra/6 Sol 使用 [A6] 同页，新旧 Luna 使用 [AL] 同页。既有 [A6L] 存档也在同页显示 6 Sol=48、6 Luna=37 及 $1.06/$0.07，与本次读取一致。

长上下文价格按 [P] 展开的 Standard 表直接抄录。5.6 三个通用型号的模型页明确写明：输入超过 272K token 时，整笔请求按 2 倍输入价、1.5 倍输出价计费。GPT-6 的短/长分栏价格已直接核到；本轮未取得其分界阈值的独立原文摘录，不把 5.6 的阈值自动套给 GPT-6。

### GPT-5.6 型号范围、首发价与优惠记录

- [F56] 首发稿及 [C] 2026-07-09 条目列出三个通用型号：Sol、Terra、Luna。`gpt-5.6` 是指向 Sol 的别名，不另算一种模型。[C] 2026-08-07 条目及 [CY] 另列 GPT-5.6 Cyber；因此本报告把“全部型号”中的安全专用型号也补列，未把它混入三档通用模型对比。
- [F56] 首发 API 价格：Sol 输入 $5 / 输出 $30；Terra $2.50 / $15；Luna $1 / $6。该首发稿所读段落未列长上下文原价，故三者历史长上下文原价均记为“未找到”，不把当前规则的乘算值冒充当时原始价表。
- [C] 2026-07-30 条目记载 Terra 降价 20%、Luna 降价 80%；现价分别为 $2/$12 与 $0.20/$1.20。该条目未给促销结束日期。Terra 的“限时促销”原文及期限：未找到；Luna 的具体促销期限：未找到。[F6] 后来将 Sol 和 Luna 的对照基准合称为 GPT-5.6 promotional pricing，保留这一原文口径，不自行统一为永久价或限时价。
- [C] 2026-08-21 条目及 [S56] 记载 Sol 优惠价 $4/$20，输入降幅 20%、输出降幅 33%；优惠至少持续至 2026-11-21。“至少持续至”不是已确定的结束日，期满恢复价格：未找到。
- 上述 07-09、07-30、08-21、11-21 均为官方页面的日期标签。精确生效时刻及其原始时区未找到，不能据此补写北京时间零点；北京时间的精确起止日期边界未核定。

### OpenAI 定位原话（各摘录保持原文）

- GPT-5.6 Sol：[S56] “Flagship model for complex professional work”。同页将其与早期 GPT-5 系列无后缀的型号档位作近似对应。
- GPT-5.6 Terra：[T56] “GPT-5.6 model that balances intelligence and cost”。同页将其与早期 GPT-5 系列 mini 档位作近似对应。
- GPT-5.6 Luna：[L56] “GPT-5.6 model optimized for cost-sensitive workloads”。同页将其与早期 GPT-5 系列 nano 档位作近似对应。
- GPT-6 Astra：[AST] “Our most capable model, built for the hardest end-to-end work”。
- GPT-5.6 Cyber：[CY] “Our most advanced cybersecurity model for authorized vulnerability research and security testing.” 访问受 Daybreak 审批及配置限制。

以上早期 GPT-5 档位说明不等于官方声明 GPT-6 的逐档对应关系。

### GPT-6 Astra 发布日期

[C] 的 2026-09-03 条目记录发布 GPT-6 Astra；[FAST] 发布稿说明先向有限组织开放，再于随后数日扩大至 ChatGPT 用户及 API 等渠道。这里的日期是官方发布标签，并不表示全部用户当天都已可用。浏览器访问发布稿时重定向至中文页面，未取得 `time`、发布日期 meta 或 JSON-LD 时间戳；精确 UTC 发布时刻、换算后的北京时间日期：未找到。不能把 09-03 无条件标成北京时间发布日期。

## 假设逐条检验所需事实

本表只给事实与缺口；“≈”“强得不多”“变相涨价”没有在任务中给出量化判定标准，本文不新增标准、不下结论。

| 待检验的假设 / 问题 | 已取得的事实、原文或原始数值 | 级别、来源与取证状态 |
|---|---|---|
| GPT-5.6 有 Sol / Terra / Luna 三档，是否确有 Terra | 官方首发稿列出 Sol、Terra、Luna；Terra 有独立模型页和价格。另有 Cyber 安全专用型号；`gpt-5.6` 为 Sol 别名。 | L1，[F56]、[T56]、[S56]、[CY]；已找到 |
| GPT-6 有 Astra / Sol / Luna 三档 | 当前官方模型指南直接列出三者；[P] Standard 价格表均有对应行。 | L1，[G6]、[P]；已找到 |
| 6 Astra ≈ 旧 Sol 档 | AA max、v4.3.2：Astra 53/$3.26，5.6 Sol 47/$1.99。短上下文输入/输出：$10/$50 对 $4/$20（现促销价），或 $5/$30（Sol 首发价）。官方定位原话见上。AA 的 Astra 文章将 5.6 Sol 称作 predecessor，并报告相差 6 点。 | L1，[P]、[F56]、[AST]、[S56]；L3，[A6]、[A56]、[AAST]；数值及比较对象已找到；官方“等能力档位”声明未找到 |
| 6 Sol ≈ 旧 Terra 档 | AA max、v4.3.2：6 Sol 48/$1.06，5.6 Terra 42/$1.40，5.6 Sol 47/$1.99。短上下文价格：6 Sol $2/$10，5.6 Terra $2/$12。官方发布稿的价格对照行是 5.6 Sol → 6 Sol。 | L1，[P]、[F6]；L3，[A6]、[A56]；已找到；官方 Terra → 6 Sol 的逐档对应声明未找到 |
| 6 Luna ≈ 旧 Luna 档 | AA 同页、max、v4.3.2：两者均为 37；单任务成本分别 $0.07、$0.18。官方发布稿价格行是 5.6 Luna → 6 Luna。 | L1，[F6]；L3，[AL]；已找到 |
| 除 Luna 外属于“换名变相涨价” | 当前及历史价格如上。OpenAI [F6] 按 Sol→Sol、Luna→Luna 展示价格；[AAST] 则拿 Astra 与 5.6 Sol 比较，并记录每 token 价格为当时 Sol 现价的 2.5 倍。两种材料的比较对象不同。 | L1，[P]、[F56]、[F6]；L3，[AAST]；事实已找到；能力等价、换名意图的官方说明未找到 |
| 6 Sol 比 6 Luna 强得不多 | AA 同页存档显示 max、v4.3.2 指数 48 和 37，单任务成本 $1.06 和 $0.07。原始子项包括 AutomationBench-AA 62%/53%、Terminal-Bench 4.0 44%/13%、HLE 48%/39%、AA-LCR v1.1 84%/83%。这些为 Intelligence Index 页面的子项。 | L3，[A6L]；已找到；“强得不多”的统一判定阈值未找到 |
| AA “level with GPT-5.6” 的比较对象 | [ASL] 标题含 “halving cost relative to GPT-5.6 Sol and Luna” 及 “Intelligence Index and Coding Agent Index scores remain level with GPT-5.6”。正文分别列 Sol 的 $1.06 对 5.6 Sol 的 $1.99，Luna 的 $0.07 对 5.6 Luna 的 $0.18，均为 max。该段未用 Terra 作对象。 | L3，[ASL]；已找到；这里记录文章列明的对象，不把 level 改写成所有指标完全相等 |
| OpenAI 是否给官方对应关系 | [F6] 价格表明确列 GPT‑5.6 Sol → GPT‑6 Sol、GPT‑5.6 Luna → GPT‑6 Luna。该稿还分别比较前后代；Astra 被列为最高能力选择。 | L1，[F6]；价格对照关系已找到；涵盖所有旧档位的三行一对一迁移表未找到 |
| OpenAI 是否给迁移建议 | [G6] 的 Migration quickstart 允许将 model 设置为 Astra、Sol 或 Luna；建议在支持时保留有效 effort，Astra 不支持 none。指南按任务推理需求、延迟、成本选模型。 | L1，[G6]；一般迁移建议已找到；“所有 Sol 用户应迁移到 Astra”或“Terra 用户应迁移到 6 Sol”的定向要求未找到 |

## 版本、冲突与未核验项

- [ASL] 为 2026-09-22 文章，图解文字标注 Intelligence Index v4.3；[A6L] 及本次三张比较页标注 v4.3.2。本文没有把文章的 v4.3 标签改写为 v4.3.2。其正文列出的四项单任务成本与当前表相同，但“相同数值”不证明评测版本相同。该存档的正文未给出四个模型全部指数整数值；文章图片中的整套历史指数数值本轮未重新核读，记为“未找到”。
- [AAST] 2026-09-09 文章写 Astra max=53、较 5.6 Sol 高 6 点，并列 Astra max 单任务成本 $3.26。这些数值与当前比较页相应字段一致；本轮不据此倒推该历史文章必为 v4.3.2。
- [AAST] 历史正文还记录 Astra AutomationBench-AA 为 69%，本次 [A6] 为 68%；两者均保留，不覆盖旧值或宣称同一测试条件。该子项不用于主表的档位判定。
- `aa-opus-article.json` 关于 Astra 在 Terminal-Bench 4.0 的比较使用 xhigh，而其他段落有 Astra max 的 token 用量。本文没有把 xhigh 子项替换进六型号 max 指数表。
- [F6] 将新旧 Luna 的整体价格变化标成 50% cheaper，同时列出输出价 $1.20→$0.50；本文保留原始价格，不把输入和输出两列统称为精确减半。
- [G6]、[F6]、[FAST] 已检查，但“未找到官方对应”仅表示本轮所查材料没有相关明确声明，不是证明所有官方资料都不存在。
- 未使用媒体、聚合页或社区用户评论为原价、优惠期限或迁移关系背书；未运行任何自有模型测试。

## 一手来源索引

所有链接均指向原始页面；对应旧存档仅作读取时点的证据，当前值以本次读取为准。

- [P] L1，OpenAI API Pricing：<https://developers.openai.com/api/docs/pricing>。本次浏览器选中 Standard 并展开首个 All models；六个通用型号短/长价均在同表。旧存档：`openai-pricing-expanded-rows.json`（该旧摘录不含 Terra、Astra 行）。
- [S56] L1，GPT-5.6 Sol：<https://developers.openai.com/api/docs/models/gpt-5.6-sol>。
- [T56] L1，GPT-5.6 Terra：<https://developers.openai.com/api/docs/models/gpt-5.6-terra>。
- [L56] L1，GPT-5.6 Luna：<https://developers.openai.com/api/docs/models/gpt-5.6-luna>。
- [CY] L1，GPT-5.6 Cyber：<https://developers.openai.com/api/docs/models/gpt-5.6-cyber>。
- [AST] L1，GPT-6 Astra：<https://developers.openai.com/api/docs/models/gpt-6-astra>。
- [C] L1，OpenAI API Changelog：<https://developers.openai.com/api/docs/changelog>，定位 2026-07-09、07-30、08-07、08-21、09-03 条目。
- [F56] L1，GPT-5.6 首发稿：<https://openai.com/index/gpt-5-6/>，定位 Availability and pricing 的三型号价格。
- [FAST] L1，GPT-6 Astra 发布稿：<https://openai.com/index/gpt-6-astra/>。浏览器自动跳转 <https://openai.com/zh-Hans-CN/index/gpt-6-astra/>；未获取到精确发布时间戳。
- [F6] L1，GPT-6 Sol/Luna 发布稿：<https://openai.com/index/introducing-gpt-6-sol-and-luna/>。复用 `openai-launch-browser.json`，定位 GPT‑6 API pricing。
- [G6] L1，当前 GPT-6 使用与迁移指南：<https://developers.openai.com/api/docs/guides/latest-model>，定位 Introduction、Migration quickstart。
- [A56] L3，5.6 Sol/Terra 同页比较：<https://artificialanalysis.ai/models/comparisons/gpt-5-6-sol-vs-gpt-5-6-terra>。本次浏览器 2026-09-24 14:40:59 北京时间读取。
- [A6] L3，6 Sol/Astra 同页比较：<https://artificialanalysis.ai/models/comparisons/gpt-6-sol-vs-gpt-6-astra>。本次浏览器及网页读取，v4.3.2。
- [AL] L3，新旧 Luna 同页比较：<https://artificialanalysis.ai/models/comparisons/gpt-6-luna-vs-gpt-5-6-luna>。本次浏览器 2026-09-24 14:41:32 北京时间读取。
- [A6L] L3，6 Luna/Sol 同页比较：<https://artificialanalysis.ai/models/comparisons/gpt-6-luna-vs-gpt-6-sol>。复用 `aa-sol-luna-current.json`，v4.3.2；存档本身未带抓取时间字段，不能将文件修改时间当作评测时间。
- [ASL] L3，AA 的 Sol/Luna 文章：<https://artificialanalysis.ai/articles/gpt-6-sol-and-luna-push-the-cost-efficiency-frontier>。复用 `aa-sol-luna-article.json`，文章日期标签 2026-09-22，图解标注 v4.3。
- [AAST] L3，AA 的 Astra 文章：<https://artificialanalysis.ai/articles/benchmarking-gpt-6-astra>。本次打开，文章日期标签 2026-09-09。
- [AOP] L3，AA 的 Opus 文章：<https://artificialanalysis.ai/articles/claude-opus-5-5>。复用 `aa-opus-article.json`，仅用于检查 effort，不作为六型号主表的数值来源。

## 新增浏览器关键字段快照

以下为本次页面读取的关键字段转录，非重绘截图、非新增评测；保留列序及标签。用于让本文在不修改其他文件的前提下留下可复查记录。

```text
URL: https://developers.openai.com/api/docs/pricing
captured_at: 2026-09-24T06:42:12.372Z
selected: Standard
Columns:
  Short context: Input / Cached input / Cache writes / Output
  Long context:  Input / Cached input / Cache writes / Output
gpt-6-astra: 10.00 / 1.00 / 12.50 / 50.00 ; 20.00 / 2.00 / 25.00 / 75.00
gpt-6-sol: 2.00 / 0.20 / 2.50 / 10.00 ; 4.00 / 0.40 / 5.00 / 15.00
gpt-6-luna: 0.10 / 0.01 / 0.125 / 0.50 ; 0.20 / 0.02 / 0.25 / 0.75
gpt-5.6-sol: 4.00 / 0.40 / 5.00 / 20.00 ; 8.00 / 0.80 / 10.00 / 30.00
gpt-5.6-terra: 2.00 / 0.20 / 2.50 / 12.00 ; 4.00 / 0.40 / 5.00 / 18.00
gpt-5.6-luna: 0.20 / 0.02 / 0.25 / 1.20 ; 0.40 / 0.04 / 0.50 / 1.80

URL: https://artificialanalysis.ai/models/comparisons/gpt-5-6-sol-vs-gpt-5-6-terra
captured_at: 2026-09-24T06:40:59.240Z
Columns: GPT-5.6 Sol (max) / GPT-5.6 Terra (max)
Intelligence Index: 47 / 42
Cost per Task: $1.99 / $1.40
Version: Intelligence Index v4.3.2

URL: https://artificialanalysis.ai/models/comparisons/gpt-6-sol-vs-gpt-6-astra
Columns: GPT-6 Sol (max) / GPT-6 Astra (max)
Intelligence Index: 48 / 53
Cost per Task: $1.06 / $3.26
Version: Intelligence Index v4.3.2

URL: https://artificialanalysis.ai/models/comparisons/gpt-6-luna-vs-gpt-5-6-luna
captured_at: 2026-09-24T06:41:32.149Z
Columns: GPT-6 Luna (max) / GPT-5.6 Luna (max)
Intelligence Index: 37 / 37
Cost per Task: $0.07 / $0.18
Version: Intelligence Index v4.3.2
```
