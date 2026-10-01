# 0924 实际使用体验与数学证据

取证日期：2026-09-24（UTC）。只整理证据与第一轮编码，不作编辑结论。Arena、OpenRouter、Epoch、MathArena 对自己的评测或流量记录为 L3；官方发布与作者论文为 L1；X 帖为 L6。网页日期、模型发布日期、评测开始日期和本次抓取日期分别保留，不互相替代。

已核实的是页面/数据集实际返回的记录，不是本次独立复现的模型能力。X 仅为固定检索词便利样本；编码为 Codex 单一编码者第一轮结果，Claude 独立复编码尚未完成。

## A. Arena（L3）

读取 [Text 总榜](https://arena.ai/leaderboard/text)、[Text 的 Coding 分类](https://arena.ai/leaderboard/text/coding)、[WebDev 总榜](https://arena.ai/leaderboard/code)。Text 与 Coding 为页面当时选中的 Style Control；WebDev 为 Overall。Text 页面 8,146,274 票/402 模型，Coding 1,721,564 票/397 模型，WebDev 739,375 票/132 模型，另列 1 个 AutoEval。本表只取有真人票数的目标模型行，不把 AutoEval 当人类盲评。

“排名”为原始名次；rank spread 另列。区间来自页面分数的 ± 数值，区间端点是显示精度下的加减值。95% 口径依据 [Arena-Rank 官方方法](https://arena.ai/blog/arena-rank) 中 significance_level=0.05；不是用帖子计数算出的 Wilson 区间。

| 榜单 | 目标/精确型号 | 名次 / spread | 分数 [95% CI] | 票数 | Preliminary | 页面更新 |
|---|---|---|---|---|---|---|
| Text | Opus 5.5：未上榜 | — | — | — | 不适用 | 2026-09-13 |
| Text | gpt-6-astra-max | 24 / 5–51 | 1480 [1468, 1492] | 2693 | 未标注 | 2026-09-13 |
| Text | Sol：未上榜 | — | — | — | 不适用 | 2026-09-13 |
| Text | Luna：未上榜 | — | — | — | 不适用 | 2026-09-13 |
| Text | claude-fable-5.1-max | 5 / 1–15 | 1498 [1490, 1506] | 5783 | 未标注 | 2026-09-13 |
| Coding | Opus 5.5：未上榜 | — | — | — | 不适用 | 2026-09-13 |
| Coding | gpt-6-astra-max | 6 / 1–42 | 1543 [1520, 1566] | 645 | 未标注 | 2026-09-13 |
| Coding | Sol：未上榜 | — | — | — | 不适用 | 2026-09-13 |
| Coding | Luna：未上榜 | — | — | — | 不适用 | 2026-09-13 |
| Coding | claude-fable-5.1-max | 32 / 6–77 | 1519 [1501, 1537] | 1184 | 未标注 | 2026-09-13 |
| WebDev | claude-opus-5.5-max | 1 / 1–2 | 1818 [1797, 1839] | 1219 | 未标注 | 2026-09-23 |
| WebDev | gpt-6-astra-max | 2 / 1–2 | 1792 [1780, 1804] | 4325 | 未标注 | 2026-09-23 |
| WebDev | gpt-6-sol-max | 5 / 4–7 | 1686 [1669, 1703] | 1521 | 未标注 | 2026-09-23 |
| WebDev | Luna：未上榜 | — | — | — | 不适用 | 2026-09-23 |
| WebDev | claude-fable-5.1-max | 3 / 3–3 | 1755 [1744, 1766] | 4916 | 未标注 | 2026-09-23 |


“未上榜”仅指本次读取的对应榜单，不等于模型表现差。Text/Coding 的页面仍标 9 月 13 日，早于 9 月 22 日新模型发布；不能用这些榜单推断新模型的当前相对位置。WebDev 的 Opus 与 Astra 的 rank spread 均为 1–2；原始第一、第二名不代表统计上已区分。[排名方法说明](https://arena.ai/blog/ranking-method)

原始：`usage/arena-text.txt`、`arena-text-browser.json`、`arena-coding-loaded.json`（完整表格行）、`arena-code-all.json`。初次 `arena-coding.txt` / `arena-coding-browser.json` 未成功返回有效正文，未用于结果。结构化摘录：`usage/arena-extract.json`。截图：`30-arena-text.png`、`30-arena-webdev.png`，保留表头、页面日期、目标行；未用旧版本或其他 effort 补空缺。

## B. OpenRouter（L3）

[Rankings](https://openrouter.ai/rankings) 页面标示 **Usage data through Sep 23, 2026**。从该页面公开使用的只读接口取得 `view=week` 与 `view=month` 两个累计窗口；本次取得的是窗口合计，不是逐日序列。精确窗口起始时间/时区未由这两个响应给出，不能擅自写成某个确定的自然月或滚动 30 日。

以下是普通型号的 standard 流量，并单列 month 的 batch 流量；没有混入 `-pro` 型号。各列 token = total_prompt_tokens + total_completion_tokens = rankingMetricValue，未再次加 reasoning 字段。单位为 token，B=10^9，T=10^12。

| 型号 | week standard | month standard | month batch | month 两 variant 合计 |
|---|---|---|---|---|
| Opus 5.5 | 221,813,177,843 | 221,813,177,843 | 167,509,630 | 221,980,687,473 |
| Astra | 1,801,597,252,529 | 3,091,825,008,534 | 758,916,290 | 3,092,583,924,824 |
| Sol | 154,158,921,133 | 154,158,921,133 | 41,346,633 | 154,200,267,766 |
| Luna | 482,238,558,830 | 482,238,558,830 | 2,319,540,023 | 484,558,098,853 |
| Fable 5.1 | 481,759,766,581 | 1,380,835,913,495 | 1,256,813,627 | 1,382,092,727,122 |


数据端点：[week](https://openrouter.ai/api/frontend/v1/rankings/models?view=week)、[month](https://openrouter.ai/api/frontend/v1/rankings/models?view=month)。模型页：[Opus 5.5](https://openrouter.ai/anthropic/claude-opus-5.5)、[Astra](https://openrouter.ai/openai/gpt-6-astra)、[Sol](https://openrouter.ai/openai/gpt-6-sol)、[Luna](https://openrouter.ai/openai/gpt-6-luna)、[Fable 5.1](https://openrouter.ai/anthropic/claude-fable-5.1)。后四链接按公开 model slug 对应，未逐一抓取其模型页；数字来自排名接口。

**发布以来口径的限制**：目标型号均在本月发布；新发布的 Opus/Sol/Luna 在 week 与 month 中相同。因此 month 窗可作为“覆盖发布时期的累计流量”代理，但本次没有取得严格以各发布时刻为起点的逐日积分，也没有证明发布前流量为零。精确“发布以来累计”：未找到可独立核验的专用字段。不要把此推断改写为精确累计事实。permaslug 中的日期是标识符，例如 Opus 的 20260921 与模型页的 Released Sep 22 不一致，未用 slug 日期替代发布日期。

仅覆盖经 OpenRouter 路由且纳入其公开统计的流量，不能代表厂商官方 API、Claude/ChatGPT 订阅。页面说明私密请求不计入公开统计。token 量不是用户数、完成任务数、满意度或胜率；不同模型 tokenization、缓存、推理与使用场景不同，窗口内可用天数也不同。

原始 `or-week-public.json`、`or-month-public.json`；精确型号/variant 及数字见 `openrouter-extract.json`。截图 `31-openrouter.png` 是 Opus 模型页 Activity 的日柱图，显示 9 月 22–24 日；24 日为未完成日，截图右侧数值不是上述截至 23 日的窗口合计。截图仅辅助确认日粒度界面，合计证据以原始接口为准。

## C. X 使用体验抽样（L6）


固定时间窗：2026-09-22T16:00:00+00:00 至 2026-09-24T16:43:55.778041+00:00。截止点在第一条查询开始时冻结，所有查询共用，排除搜索期间新发但超过此点的帖子。日期检索附加 `since:2026-09-22 until:2026-09-25`，再按 created_at 精确过滤。


OpenCLI 1.8.7，已登录浏览器桥接，只读搜索；未登录、未读取 Cookie、未点赞/转发/关注/回复。每次请求 limit=100、超时 150 秒、调用之间暂停 4 秒；help 未给出可验证的硬上限，100 是本次操作上限，不声称抓完全部匹配结果。8 组检索均已尝试 live/top，第 8 组 top 返回 HTTP 429 后停止，未绕过限流。

| 检索词 | live 返回 | top 返回 |
|---|---|---|
| "Opus 5.5" Astra | 100 | 100 |
| "Opus 5.5" vs | 100 | 100 |
| "Opus 5.5" coding | 100 | 100 |
| "Opus 5.5" 体验 | 90 | 100 |
| "Opus 5.5" 用了 | 100 | 100 |
| "GPT-6 Sol" | 100 | 100 |
| "GPT-6 Luna" | 100 | 100 |
| Astra "Opus 5.5" 编程 | 24 | 未采集到：HTTP 429 |


原始返回 **1414** 条（含查询间重复）；按 tweet id 去重 **1122** 条；纳入 **310** 条、**285** 个不同账号；排除 **812** 条。原始 JSONL 的 query 保存每次命中的检索词、排序与采集批次。

| 互斥排除原因 | 条数 |
|---|---|
| outside_time_window | 32 |
| rumor_or_speculation_only | 7 |
| official_account | 5 |
| no_clear_firsthand_model_experience | 698 |
| duplicate_template_or_satire | 19 |
| advertising_promotion_giveaway | 43 |
| bot_reply | 8 |


`no_clear_firsthand_model_experience` 是保守合并类：纯新闻/转述跑分、观看别人的 demo、询问他人体验、只有抽象赞踩、仅标题/模型名单、版本不明确等，均没有足够的本人目标模型使用与具体体验证据。它不意味着其中每条都是假帖。所有排除 id、原始行号及原因在 `x_excluded.jsonl`；原始全文可按 id 回查。

编码过程：宽松词法预筛后逐帖语义审阅候选；另复查初筛未命中的高浏览量帖、日文及其他可辨识使用叙述，补回省略主语的日文等样本。此轮为单个模型编码者完成，并非人工调查、也不是两个独立编码者。纳入本人使用/自测/原始制作叙述；不检查未访问的视频来臆断胜负。不把“GPT-6”“Codex”自动当 Astra，不把“Sol”未指明代际的语句自动当 GPT-6 Sol。

`models` 为本任务五个目标型号中可辨识实际使用者的列表，其他型号保留在原文，不并入这五个型号。`pos/neg` 是帖内方向；质量好但速度/价格差等同时出现时记 mixed；仅展示成本或输出而无情感判断可记 na。明确总偏好即 prefer_opus/prefer_astra；各有擅长、取舍或接近记 mixed；同帖出现两者但没有直接比较记 na。task 取帖子体验的主任务；编程生成游戏/动画归 coding_agent，侧重成片或软件操作而未明确编程过程的归 other。原文短摘仅作为定位锚点，完整原文是编码依据。

简介字段在全部原始记录中均为空。已从 [Anthropic 官方页面](https://www.anthropic.com/webinars/claude-code-service-delivery) 核实 Boris Cherny 的身份，@bcherny 记 employee=true；其余 false 仅表示“本次未识别为员工”，均附 employee_status=unknown_bio_unavailable，不能理解为已核实非员工。员工帖按照协议纳入；移除已确认员工后，直接比较分布不变，因为该帖没有直接比较 Astra。

### 计数与 95% Wilson 区间

每一类按二项边际区间计算；不是三类联合置信区域。设 k 为该类条数，n 为该字段去掉 na 的条数，p=k/n，z=1.959963984540054，区间为

\[
\frac{p+z^2/(2n)\ \pm\ z\sqrt{p(1-p)/n+z^2/(4n^2)}}{1+z^2/n}
\]

表内每格为 k/n；百分比 [Wilson 95% 下界, 上界]。n=0 时区间不定义。统计脚本 `build_x_stats.py`、全精度结果 `x_stats.json` 可逐项复算。

| Opus vs Astra | 计数/比例/区间 |
|---|---|
| prefer_opus | 43/73；58.9% [47.4%, 69.5%] |
| prefer_astra | 12/73；16.4% [9.7%, 26.6%] |
| mixed | 18/73；24.7% [16.2%, 35.6%] |


此字段 na=237，未计入 n=73。

| 情感字段 | pos | neg | mixed | na |
|---|---|---|---|---|
| sentiment_opus55 | 159/197；80.7% [74.6%, 85.6%] | 6/197；3.0% [1.4%, 6.5%] | 32/197；16.2% [11.7%, 22.0%] | 113 |
| sentiment_sol | 22/88；25.0% [17.1%, 35.0%] | 54/88；61.4% [50.9%, 70.9%] | 12/88；13.6% [8.0%, 22.3%] | 222 |
| sentiment_luna | 23/54；42.6% [30.3%, 55.8%] | 24/54；44.4% [32.0%, 57.6%] | 7/54；13.0% [6.4%, 24.4%] | 256 |


按 task 分组的直接比较：

| task | prefer_opus | prefer_astra | mixed | na |
|---|---|---|---|---|
| coding_agent | 21/40；52.5% [37.5%, 67.1%] | 8/40；20.0% [10.5%, 34.8%] | 11/40；27.5% [16.1%, 42.8%] | 133 |
| writing | 0/2；0.0% [0.0%, 65.8%] | 1/2；50.0% [9.5%, 90.5%] | 1/2；50.0% [9.5%, 90.5%] | 9 |
| reasoning_math | 0/0；不定义 | 0/0；不定义 | 0/0；不定义 | 1 |
| chat | 1/1；100.0% [20.7%, 100.0%] | 0/1；0.0% [0.0%, 79.3%] | 0/1；0.0% [0.0%, 79.3%] | 3 |
| other | 2/6；33.3% [9.7%, 70.0%] | 2/6；33.3% [9.7%, 70.0%] | 2/6；33.3% [9.7%, 70.0%] | 16 |
| unspecified | 19/24；79.2% [59.5%, 90.8%] | 1/24；4.2% [0.7%, 20.2%] | 4/24；16.7% [6.7%, 35.9%] | 75 |


语言：{"en": 159, "other": 33, "zh": 118}；任务：{"chat": 4, "coding_agent": 173, "unspecified": 99, "writing": 11, "other": 22, "reasoning_math": 1}。


### 纳入帖中浏览量最高的五条

浏览量是检索返回时的快照；不作为统计权重，不等于独立读者或赞成票。

| 作者/原帖 | views | 内容概括（作者自述，未复现） |
|---|---|---|
| [@bcherny](https://x.com/i/status/2102543349102338309) | 1,843,217 | 自称用 Opus 5.5 与 Lean 形式化检查 Agent SDK，得到 16 个修复 PR；作者为 Anthropic 员工。 |
| [@Layton_Gott](https://x.com/i/status/2102586000752988176) | 584,159 | 称高强度使用后不满意 Sol、满意 Luna，对 Opus 5.5 的表现和额度给出正面反馈。 |
| [@ScarletKc](https://x.com/i/status/2102699699409228146) | 540,409 | 称刚试用 Opus 5.5，表达从 Astra 转向 Opus 的强烈个人偏好。 |
| [@PawelHuryn](https://x.com/i/status/2102672181305958543) | 528,516 | 本人在两个仓库、105 个植入 bug 上测试；报告 Sol 找到的 bug 较少，但 API 等价成本更低。 |
| [@qibiz_me](https://x.com/i/status/2102594977264234578) | 459,823 | 称让 Opus 5.5 操作 Synthesizer V Studio 2 Pro 完成请求后，认为其表现超过 Astra。 |


### 盲审、偏差与未验证事项

纳入集按 tweet id 升序，Python `random.Random(20260924).sample` 抽 20 条，保存 `x_blind20.jsonl`，只有 id/url/text。请独立编码时只读取该文件，不读取本报告分布、`x_coded.jsonl` 或决策表；之后才对齐 id 计算逐字段一致率/Cohen κ。本次没有代替 Claude 复编码，也未填任何一致率。

- 自选择与幸存偏差：愿意公开发体验的人不是随机用户；新发布两天内的兴奋、投诉、宣传均可能过量出现；无长期可靠性随访。
- 平台/语言偏差：只涵盖可被当前登录 X 会话检索到的帖，中英文检索占主导。补看日文不等于完成多语言均衡抽样。删除、私密、未收录或限流隐藏的帖子不可见。
- 检索词偏差：五个 Opus 相关词组加一个交叉编程词组，Sol/Luna 各一个独立词组；更容易找出 Opus 比较与编程内容，各模型情感分母不可当公平随机对照。
- 排序与截断偏差：热门排序放大高传播帖子；每批最多 100，13 批触及该数，未穷尽；末次 top 因 429 缺失。重复 id 去重不能消除算法曝光偏差。
- 依赖性：同一作者可贡献多帖，同一测试可能衍生不同帖；明显模板/重复会剔除，但未做作者聚类加权，不能假定帖子统计独立。
- 编码偏差：单一模型编码者，语义边界与主任务判断存在主观性；简介缺失导致员工标签不完整；只凭原文自述，没有验证使用日志、账单、付费关系或复现结果。词法预筛及保守复查仍可能漏收。
- **Wilson 只描述把当前条目视作独立二项观测时的区间，不覆盖以上选择/依赖/编码偏差，不能推断全体用户支持率、模型能力胜率，或宣称统计显著的总体优势。**

## D. 数学能力相关证据（L1 / L3）

### Epoch FrontierMath v2（L3）

来源：[Tiers 1–3 v2](https://epoch.ai/benchmarks/frontiermath-tiers-1-3-v2)、[Tier 4 v2](https://epoch.ai/benchmarks/frontiermath-tier-4-v2)、[官方数据导出说明](https://epoch.ai/benchmarks/use-this-data)。使用官方 CSV 的 mean_score、stderr、Started at；误差是 **一个标准误 SE，单位为百分点，不是 95% CI**。Started at 是评测开始时间，不是发榜日期。保留 CSV 的 effort 字面值，不以高档结果替换低档结果。

| 精确型号 × effort | 评测 | 得分 % ± SE百分点 | 评测开始 UTC | 原始文件 |
|---|---|---|---|---|
| claude-fable-5-1_max | Tiers 1–3 v2 | 90.2 ± 1.8 | 2026-09-01T18:38:24.000Z | epoch-frontiermath_tiers_1_3_v2.csv |
| gpt-6-astra_max | Tiers 1–3 v2 | 93.7 ± 1.4 | 2026-08-30T14:57:43.000Z | epoch-frontiermath_tiers_1_3_v2.csv |
| claude-opus-5_max | Tiers 1–3 v2 | 85.6 ± 2.1 | 2026-07-24T19:54:20.000Z | epoch-frontiermath_tiers_1_3_v2.csv |
| claude-fable-5-1_max | Tier 4 v2 | 87.8 ± 5.2 | 2026-09-01T18:38:24.000Z | epoch-frontiermath_tier_4_v2.csv |
| gpt-6-astra_none | Tier 4 v2 | 82.9 ± 5.9 | 2026-08-30T02:22:12.000Z | epoch-frontiermath_tier_4_v2.csv |
| gpt-6-astra_low | Tier 4 v2 | 87.8 ± 5.2 | 2026-08-30T02:22:12.000Z | epoch-frontiermath_tier_4_v2.csv |
| gpt-6-astra_medium | Tier 4 v2 | 97.6 ± 2.4 | 2026-08-30T02:22:12.000Z | epoch-frontiermath_tier_4_v2.csv |
| gpt-6-astra_high | Tier 4 v2 | 97.6 ± 2.4 | 2026-08-30T02:22:12.000Z | epoch-frontiermath_tier_4_v2.csv |
| gpt-6-astra_xhigh | Tier 4 v2 | 97.6 ± 2.4 | 2026-08-30T02:22:12.000Z | epoch-frontiermath_tier_4_v2.csv |
| gpt-6-astra_max | Tier 4 v2 | 97.6 ± 2.4 | 2026-08-30T02:22:12.000Z | epoch-frontiermath_tier_4_v2.csv |
| claude-opus-5_max | Tier 4 v2 | 73.2 ± 7.0 | 2026-07-24T19:54:20.000Z | epoch-frontiermath_tier_4_v2.csv |
| Claude Opus 5.5 | Tiers 1–3 v2 | 未找到 | 未找到 | 官方 CSV 无该型号 |
| Claude Opus 5.5 | Tier 4 v2 | 未找到 | 未找到 | 官方 CSV 无该型号 |


条件：v2 于 2026-06-12 引入，模型提交可执行 Python `answer()`；Tiers 1–3 用排除公开示例后的 285 道私有题，Tier 4 用 41 道；不与早期 v1 直接拼接。题集与执行协议不同于人类盲评和无工具数学聊天。Astra high/xhigh/max 的官方 CSV 已舍入为 0.976 与 stderr 0.024，未恢复或假造额外精度。结构化 `epoch-extract.json` 还保留官方 log viewer 链接，原始 CSV 可复核。截图 `32-frontiermath.png` 为 Tier 4 页面。

Epoch 页面利益冲突提示原文：

> FrontierMath was developed with funding from OpenAI, who has exclusive access to a subset of the benchmark.

该句来自两级榜页，明确 OpenAI 资助及对部分题目的独占访问。它不等于“访问全部评测题”，也不能推出是否训练污染。榜页另指向 [详细声明](https://epoch.ai/frontiermath/tiers-1-4/about#:~:text=Conflict%20of%20interest%20statement)；本次该详细页面未成功定位到完整声明段落，只保留上面已读到的提示原文。

### MathArena 及开放问题对照（L3）

来源：[MathArena Models](https://matharena.ai/models)。以下为其 **Expected Performance**，包含对未测竞赛的估计，不是把所有模型放在同一已完成题集上的原始正确率。

| 型号 | effort | Expected Performance | 日期/条件 |
|---|---|---|---|
| GPT-6 Astra | max | 88.0% ±1.7% | 评测日期未找到 |
| GPT-6 Astra | low | 82.4% ±3.1% | 同上 |
| Claude Fable 5.1 | high | 82.3% ±5.3% | 同上 |
| Claude Fable 5.1 | max | 81.6% ±4.5% | 同上 |
| Claude Fable 5.1 | low | 65.9% ±7.6% | 同上 |
| Claude Opus 5 | max | 73.7% ±3.6% | 同上 |
| Claude Opus 5.5 | — | 未上榜 | 不以 Opus 5 顶替 |


方法页说明：以每题等权聚合，各题实测采用多次运行均值，缺测项用 item-response 模型估计，区间通过模拟与重拟合获得；本次未确认此 ± 的置信水平，故不擅自标为 95% CI。页面的模型 Release Date 也没有当评测日期。原始 `matharena-models.txt`、`matharena.txt`。

另一个较难题集：[Epoch FrontierMath Erdős 论文](https://arxiv.org/html/2609.25050v1) 报告预发布 Astra 在 68 个开放问题中解决 2 个（约 3%），Fable 5.1 为 0/68；Opus 5/5.5 未在该对照中评测。每题一个尝试、标称 300 美元/72 小时 agent 条件；文中还披露 Astra 初期按 Sol 价计费，实际费用约高 1.8–2 倍，并讨论重计费对结果的影响。它是开放问题研究协议，不可同 Tier 4 的分数比较。原始官方 CSV `epoch-frontiermath_erdos.csv` 可作补充。

### 一手数学成果：型号归属与验证边界

| 一手来源 | 型号/条件 | 来源实际声称 | 本次核验边界 |
|---|---|---|---|
| [OpenAI Astra 发布页](https://openai.com/index/gpt-6-astra/)，2026-09-03 | GPT-6 Astra | 发布页给出数学基准及此前开放问题成果入口 | 厂商发布口径，不代替 Epoch 相同协议评测 |
| [Ten advances](https://openai.com/index/ten-advances-in-mathematics/)，2026-08-01；[253 页论文](https://cdn.openai.com/pdf/ten-proofs-oai.pdf)，8 月 6 日更新 | **内部 Astra 版本**，人类配合同一模型整理手稿；其后形式化 | 十项数学/理论计算机科学进展，包括球堆积上界、非 sofic 群、多色 Ramsey 等；提供 Lean 证书入口 | 论文封面署 OpenAI；成果与正确性责任由发布方陈述。本次未逐定理复证、未运行 Lean，不能升级为独立证明验收 |
| [a16z 原始访谈](https://www.youtube.com/watch?v=1JvyLGd2Sfs)，2026-09-08 | Mehtaab Sawhney、Mark Sellke 与 Lisha Li 对谈 | 原始视频说明确认双方研究者身份与球堆积、非 sofic 群讨论；16:20 章节为 Astra 十题，36:17 讨论 harness/model | 已读取原始视频说明/章节，原文保存在 a16z-video-expanded.json。没有用第三方逐字稿冒充原始作者声明；未完整听完访谈 |
| [Short proofs II, arXiv:2604.06609](https://arxiv.org/abs/2604.06609)，2026-04-08 | 未命名的 OpenAI 内部模型 | Sawhney、Sellke 等五位作者报告五个 Erdős 相关证明 | **不能仅凭作者身份归到公开 Astra**；本次核对作者、摘要、日期，未证明审计 |
| [GPT, the Counterexample Machine, arXiv:2608.29595](https://arxiv.org/abs/2608.29595)，2026-08-30 | Suvrit Sra；12 个月内使用 GPT Pro | 报告超过 15 个跨领域反例并附公开库 | **不是 Astra 专属成果集，也不是 Sawhney/Sellke 合著**；未逐例独立验证 |
| [Anthropic：Formalizing Fermat’s Last Theorem](https://www.anthropic.com/research/formalizing-fermats-last-theorem)，2026-09-04 | 通用内部研究模型，厂商称大致与 Fable 5.1 相当；Prove2Me/Claude Code 多 agent | 称 11 天、约 60 亿输出 token，1300 万行 Lean，最终使用 29,500 个中间定理；Lean 检查、标准三公理与 statement comparator，引用 Kevin Buzzard 审阅 | 是既有数学定理的形式化；继承 Mathlib/既有 FLT 项目工作。**不是 Opus 5.5 实测，也不能直接视作公开 Fable 5.1 的同设置结果**；本次未运行证明工程 |

此处同时列“未命名内部模型”“内部 Astra”“GPT Pro”“公开榜单精确型号”，不把不同系统合并。Opus 5.5 的上述公开数学评测缺位仍为缺位；没有据此给出数学强弱结论或正文建议。

## 交付与复核入口

`usage/x_raw.jsonl`、`x_coded.jsonl`、`x_excluded.jsonl`、`x_blind20.jsonl`；`protocol.json`、`search_ledger.jsonl`、`coding_decisions.tsv`、`build_x_stats.py`、`x_stats.json`；A/B/D 结构化摘录与网页/CSV 原始档案。`usage/README.md` 记录四张图的来源与使用限制。完整本次新增文件清单见 `usage/new-files.txt`；不修改原 images/README.md。

明确未完成/未验证：第 8 组 top 限流缺失；X 不穷尽、员工身份不完整、Claude 盲审一致率待算；OpenRouter 严格发布时刻累计未取得；Opus 5.5 数学数据未找到；MathArena 误差置信水平和评测日期未确认；未独立复现模型测试或数学证明。所有这些缺口保留在报告，不补造。
