# Handoff: 0924 实际使用体验取证（Arena + OpenRouter + X 抽样）

## 目标

为“AI 吠点”0924 期补一组**实际使用体验**证据，回答：Claude Opus 5.5 与 GPT-6 Astra（以及 GPT-6 Sol / Luna）在真实使用中口碑如何。Develata 原话：“benchmark 并不等于使用体验，可以找一种统计学上的实际使用体验”，并要求通过 X 找实际使用帖。

证据分两层：

1. 主证据（有统计量）：LMArena 人类盲评排名、OpenRouter 实际使用量。
2. 补充（口碑抽样）：X 上第一手使用帖，按固定协议抽样、编码、计数。结论只能表述为“抽样”，不是用户调查。

你只取证和整理数据，不写正文、不下编辑结论。

## 上下文

- 仓库：`E:\gitclone\AI-Barking`，分支 `main`，**尚无任何 commit**。
- 先读：`AGENTS.md`、`EDITORIAL.md`（重点“事实分级”）、`0924/doc_0924_publish.txt`（当前正文）、`0924/sources/fact-check.md`。
- 事实分级：LMArena、OpenRouter 对其自身数据为 L3；X 帖子为 L6（社交媒体转述），只能作口碑样本，不能作为能力事实。
- 已知：9 月 22 日（UTC）Anthropic 发布 Opus 5.5（16:31 UTC），OpenAI 发布 GPT-6 Sol / Luna（18:12 UTC）；GPT-6 Astra 于 9 月初发布。

## 起始基线（交接时 `git status --short`）

```
A  0924/doc_0924.md
AM AGENTS.md
?? .gitignore
?? 0924/doc_0924_publish.txt
?? 0924/images/
?? 0924/sources/
?? EDITORIAL.md
?? brand/
?? templates/
```

以上全部是需保留的输入，不得修改、移动或删除。

## 环境与工具

- Windows 11，PowerShell / Git Bash 均可。
- X 读取：用 OpenCLI（复用 Develata 已登录 X 的 Chrome 会话，经 OpenCLI 浏览器桥接）。已验证可用的命令（2026-09-24）：

  ```bash
  export OPENCLI_BROWSER_COMMAND_TIMEOUT=150
  opencli twitter search '"Opus 5.5"' --limit 5 -f yaml
  ```

  先跑 `opencli twitter search --help` 查看是否支持“最新/热门”排序与更大的 limit。默认超时 60s 会失败，必须设上面的环境变量。
- `twitter-cli`（`twitter search`）当前 404，不要用。
- 运行前需要 Develata 的 Chrome 已打开、已登录 X、OpenCLI 扩展已连接。若 OpenCLI 报未连接，停止 X 部分并在报告中说明，不要尝试登录或读取 Cookie。
- 网页：LMArena、OpenRouter 用浏览器或 `curl -s "https://r.jina.ai/<URL>"` 读取。

## 任务

### A. LMArena（L3）

- 找到当前 Text 总榜及 WebDev 榜（若有 Coding 分类也记录），记录 Claude Opus 5.5、GPT-6 Astra、GPT-6 Sol、GPT-6 Luna、Claude Fable 5.1 的：排名、分数、95% CI、票数、是否标注 preliminary、页面更新时间、URL。
- 某模型未上榜就写“未上榜”，不要用其他版本或 effort 顶替。
- 截图保存为 `0924/images/30-arena-*.png`（最多 2 张，截含上述模型的区域，保留表头与更新时间）。

### B. OpenRouter（L3）

- 记录上述模型在 OpenRouter 的模型页 / Rankings 页上发布以来的使用量（token 数或占比、时间粒度、截至时间）。
- 注明 OpenRouter 数据只覆盖经 OpenRouter 调用的流量，不代表各家官方 API 与订阅产品。
- 截图最多 1 张：`0924/images/31-openrouter.png`。

### C. X 口碑抽样（L6）

**时间窗**：2026-09-22 16:00 UTC 至采集时刻。

**检索词**（每个都跑；如支持，“最新”和“热门”各跑一次；每次尽量取到上限）：

- `"Opus 5.5" Astra`
- `"Opus 5.5" vs`
- `"Opus 5.5" coding`
- `"Opus 5.5" 体验`
- `"Opus 5.5" 用了`
- `"GPT-6 Sol"`
- `"GPT-6 Luna"`
- `Astra "Opus 5.5" 编程`

每次调用之间停顿几秒，不要高频请求。**只读**：不点赞、不转发、不关注、不回复。

**原始数据**：全部结果按 tweet id 去重，写入 `0924/sources/usage/x_raw.jsonl`（每行一条，保留 id、author、text、created_at、likes、views、url、query）。

**纳入标准**（全部满足）：发帖人声称自己用过所讨论的模型，并描述了体验或对比；在时间窗内。

**排除**（记录排除原因）：官方账号（@AnthropicAI、@claudeai、@OpenAI 及其产品号）；纯转发跑分或新闻；爆料/传闻；广告、带货、抽奖；明显机器人或重复内容；无评论的转发。能从简介判断为两家公司员工的，纳入但标 `employee=true`。

**编码**（写入 `0924/sources/usage/x_coded.jsonl`，每条纳入帖一行）：

| 字段 | 取值 |
|---|---|
| `models` | 帖中实际使用的模型列表 |
| `verdict_opus_vs_astra` | `prefer_opus` / `prefer_astra` / `mixed` / `na`（未直接比较两者） |
| `sentiment_opus55` | `pos` / `neg` / `mixed` / `na` |
| `sentiment_sol` / `sentiment_luna` | 同上 |
| `task` | `coding_agent` / `writing` / `reasoning_math` / `chat` / `other` / `unspecified` |
| `lang` | `zh` / `en` / `other` |
| `employee` | true / false |
| `evidence_quote` | 支撑编码的原文短摘（逐字） |

**盲审样本**：用固定随机种子 `20260924` 从纳入帖中抽 20 条，只把 id、url、text 写入 `0924/sources/usage/x_blind20.jsonl`（**不含**你的编码），供 Claude 独立复编码以计算一致率。

**统计**（写入报告）：

- 原始条数、去重后条数、纳入条数、各排除原因计数。
- `verdict_opus_vs_astra` 在 `na` 以外各类的计数、比例及 95% Wilson 区间（写出公式与 n）。
- 各模型 sentiment 分布，同上格式；按 `task` 分组的 Opus vs Astra 分布。
- 浏览量最高的 5 条纳入帖（url、views、一句话内容概括）。
- 偏差说明：自选择、平台与语言偏差、检索词偏差、热门排序偏差。

## 范围与约束

- 只可新建：`0924/sources/usage/` 下的文件、`0924/images/30-*.png`、`0924/images/31-*.png`、`0924/images/32-*.png`，以及报告文件 `0924/sources/usage-evidence.md`。
- 不得修改任何已有文件（含正文、`images/README.md`、`fact-check.md`）。
- 不下编辑结论，不写正文措辞建议；数据缺失如实写“未找到 / 未上榜 / 未采集到”。
- 不编造、不补全帖子内容；引文逐字。

## 授权

仅在工作区内创建上述文件。不 commit、不 push、不做任何删除或破坏性操作。不在 X 上做任何写操作。

## 验收

1. `0924/sources/usage-evidence.md` 存在，含 A、B、C、D 四节及偏差说明。
2. `x_raw.jsonl` 行数 = 报告中“去重后条数”；`x_coded.jsonl` 行数 = “纳入条数”；`x_blind20.jsonl` 恰 20 行（纳入不足 20 时等于纳入条数）且不含编码字段。
3. 报告中每个 Wilson 区间可由给出的 n 与计数复算。
4. 验证命令（Git Bash）：

   ```bash
   cd /e/gitclone/AI-Barking/0924/sources/usage
   wc -l x_raw.jsonl x_coded.jsonl x_blind20.jsonl
   python -c "import json;[json.loads(l) for f in ['x_raw.jsonl','x_coded.jsonl','x_blind20.jsonl'] for l in open(f,encoding='utf-8')];print('jsonl ok')"
   grep -c verdict x_blind20.jsonl || true   # 应为 0
   git -C /e/gitclone/AI-Barking status --short
   ```

## 回报

给 Develata 一段可直接粘贴回 Claude 的报告：新建文件列表；以上验证命令的真实输出；A/B/C 的关键数字摘要（≤300 字）；遇到的问题、做过的假设、未完成项。

## 追加任务 D：数学能力对比（L1/L3）

正文现写“数学上，GPT 明显更强”（标注为体感）。请取证，不下结论：

1. Epoch AI FrontierMath（Tier 1–3 与 Tier 4，v2）：GPT-6 Astra、Claude Opus 5.5、Claude Fable 5.1、Claude Opus 5 的分数、误差、effort、评测日期。页面为 JS 渲染，用浏览器读取数据或 Data Explorer；截图 `0924/images/32-frontiermath.png`（最多 1 张）。记录 Epoch 的利益冲突声明原文（OpenAI 资助、独占访问部分题目）。
2. MathArena 等其他数学评测中上述模型的分数（若有）。
3. OpenAI 关于 Astra 数学成果的一手来源（发布页、论文、作者声明），如 Sawhney、Sellke 的开放问题结果、arXiv 2608.29595；Anthropic 方面同类一手来源（若有）。
4. 写入 `0924/sources/usage-evidence.md` 的 D 节，按“型号 × 评测 × 分数 × 条件 × 来源”列表。
