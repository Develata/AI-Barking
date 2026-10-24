# 1023 期 A 组取证：Kimi K3.1 Pro 网络能力取舍

取证时刻：2026-10-24 19:52–21:05（北京时间，UTC+8）。执行方：Sonnet 5.5（claude-sonnet-5-5）。所有存档在 `docs/2610/1023/sources/a-*`，截图在 `docs/2610/1023/images/01–04`。HTML 原件只存本地。

## 先说结论（取证层面）

- 博客 xdev-bench-hard 图表比扫描摘要多出三项：上一代 Kimi K3 = 33.3、DeepSeek-V4.1-Flash = 85.7（高于 K3.1 Pro 的 81.0）、GPT-5.6 Sol = 100（带星号“Third-party evaluation”，harness 为 codex）。90.5→81.0 是同一管线“去不去网络数据”的对照，不是相对上一代下降。
- 同一张表各行 harness 不同（kimi-code / claude-code / codex），平均成功率另列：90.5 版 71.4，K3.1 Pro 64.5。
- 截至 2026-10-24 约 20:00（北京时间）Hugging Face moonshotai 组织页与 API 列表里没有 K3.1 Pro 权重（最新为 Kimi-K3，lastModified 2026-09-02）。
- AA“第 4”是按模型去重（同一模型的不同 effort 档只算一个）；AA 总榜表格按行排，Kimi 在第 5 行（前面有 Opus 5.5 max 58、Sonnet 5.5 max 56、Opus 5.5 xhigh 56、Muse Wonder Full 56）。不是并列。
- AA 文章写 Cost per Task $1.98；模型页与总榜写 $2.11。两处都在 AA 原站。

## 事实清单

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1-a | 发布与渠道 | L1 | https://www.kimi.ai/blog/kimi-k3-1-pro（存档 a-blog.html/.txt） | “Kimi K3.1 Pro and Kimi K3.1 Pro Highspeed are available today on Kimi.ai, Kimi Work, Kimi Code, and the Kimi API. The full model weights will be released by Nov 4.” 另：“Full model weights will be released by Nov 4, 2026.”；“Kimi Enterprise administrators can enable K3.1 Pro for their organization; access is off by default at launch.” | 无（正文文字已含在 01） | 官方博客页面没有发布时刻。旁证：HN 提交 2026-10-24 00:59（北京时间；2026-10-23T16:59:08Z）；TNW 2026-10-23T18:19:34Z（= 10-24 02:19 北京）；图片路径含 `20261023`。官方 X 帖未取（本组未扩展来源）。“by Nov 4”是承诺上限，未给时刻与时区 | 已找到（发布时刻：官方未给） |
| A1-b | 取证时 HF 上有无 K3.1 Pro 权重 | L1 | https://huggingface.co/moonshotai ；https://huggingface.co/api/models?author=moonshotai（存档 a-hf-org.html/.txt、a-hf-api.json） | 列表最新项为 `moonshotai/Kimi-K3`（lastModified 2026-09-02T02:22:41Z）；没有 K3.1 条目。直接查 `api/models/moonshotai/Kimi-K3.1-Pro`、`Kimi-K3.1`、`Kimi-K3.1-Pro-Highspeed` 均返回 401（HF 对不存在的仓库返回 401） | 无 | 取证时刻 2026-10-24 19:52（北京时间）；之后可能变化 | 已找到（未发布） |
| A1-c | AA 对权重状态 | L3 | https://artificialanalysis.ai/articles/kimi-k3-1-pro-launch-benchmarks | “Weights and license: Weights currently not published, Moonshot is expected to release the weights soon after launch” ；模型页顶部标 “Proprietary model” | 无 | AA 页面 2026-10-23 | 已找到 |
| A2-a | 90.5%→81.0% 比较对象 | L1 | 同 A1-a | “On xdev-bench-hard, the checkpoint without these risk-reduction measures — no cyber-related data removal at mid-training and no cyber-related RL environment filtering — solves 90.5% of problems at pass@3; the final K3.1 Pro reaches 81.0%. The comparison shows that data removal meaningfully reduces cyber capability relative to what the same training pipeline would otherwise produce.” | 01-kimi-cyber-raw.png | 对照对象是同一管线、不做剔除的检查点（博客图表行名 “Kimi model w/o cyber risk-reduction”），不是上一代 | 已找到 |
| A2-b | pass@3、子集、任务构成 | L1 | 同 A1-a | “We report pass@3 over a random subset of runs, with no hard timeouts or output token limits.” “xdev-bench-hard, our in-house benchmark of expert-curated, source-code-based vulnerability discovery and exploitation tasks … Most tasks faithfully reproduce recently disclosed vulnerabilities in software such as the Linux kernel, Node.js, JavaScriptCore, Exim, and VirGL, with mitigations enabled.” | 01 | 自家基准，未公开任务数；博客未说明“random subset of runs”的具体含义。值 90.5/81.0/85.7/57.1/33.3/100 都是 1/21 的整数倍（推断任务约 21 个，未经官方证实，L7 级推断，不入正文） | 已找到（任务数：未找到） |
| A2-c | “仍强于上一代”及上一代分数 | L1 | 同 A1-a；图表 https://statics.kimi.ai/kimi-blogs/kimi-k3-1/20261023/xdev-bench-hard-results.webp | “At the same time, K3.1 Pro improves substantially over its predecessor, indicating that stronger general coding and long-horizon debugging capabilities generalize to cyber tasks even after targeted data removal. K3.1 Pro's cyber capability remains behind that of frontier proprietary models that have been publicly available for at least three months.” 图表：Kimi K3 = 33.3（平均成功率 15.9），K3.1 Pro = 81.0（64.5） | 01、02-kimi-xdev-chart-raw.png | 博客文字没写上一代分数，只在图表里给；“predecessor”= Kimi K3 | 已找到 |
| A2-d | 图表全貌（扫描摘要没提） | L1 | 同 A2-c | 图表行：GPT-5.6 Sol*（codex）100 / 平均成功率 77.8；Kimi model w/o cyber risk-reduction（kimi-code）90.5 / 71.4；DeepSeek-V4.1-Flash（claude-code）85.7 / 57.1；Kimi K3.1 Pro（kimi-code）81.0 / 64.5；GLM-5.3（claude-code）57.1 / 34.9；Kimi K3（kimi-code）33.3 / 15.9。脚注 “* Third-party evaluation” | 02 | 各行 harness 不同；GPT-5.6 Sol 是第三方评测，其余为 Kimi 自家跑；DeepSeek 与 GLM 用 claude-code，Kimi 用 kimi-code | 已找到 |
| A2-e | 开放权重原则 | L1 | 同 A1-a | “First, we aim not to release open weights that would extend the absolute frontier of cyber capability. K3.1 Pro's offensive cyber capability is, by design, below what our pipeline can produce, and remains behind that of frontier proprietary models. Second, we intend to progressively raise the cyber capability available in open weights.” | 01 | 厂商自述 | 已找到 |
| A2-f | 剔除了什么（范围） | L1 | 同 A1-a | “For the open-weight release of K3.1 Pro, we identified and removed cyber-related data from both training stages, while retaining the general-purpose coding tasks needed for model usefulness.” “In preliminary experiments on internal models, we found that without data removal at either the mid-training or the RL stage, downstream cyber capability would increase substantially.” | 01 | 措辞是“for the open-weight release”。博客没说明 API / app 上线的版本是不是同一检查点（API 已上线而权重未发布）。博客未说明“cyber-related”的界定，也未说明能否经微调恢复（Implicator 亦指出） | 已找到（API 版是否同一检查点：未找到） |
| A3 | ExploitBench / ExploitGym 分数与对照 | L1 | 同 A1-a（页内数据，Cybersecurity benchmarks 表；该表由画布渲染，截图时空白，未能截取；数值取自页面源码内嵌 JSON，存于 a-blog.html） | 表：ExploitBench — K3.1 Pro 74.7；Claude Fable 5.1 83.0；GPT-6 Astra 100；Claude Opus 5.5 91.0；DeepSeek-V4.1-Flash-1017 无；GLM-5.3 54.4。ExploitGym — 20.7；30.4；42.4；34.5；无；15.0。xdev-bench-hard (Internal) — 81.0；Fable、Astra、Opus 无；DeepSeek 85.7；GLM 57.1。脚注：“ExploitBench … We report the pass@5 max capability score: for each environment, the best run across 5 seeds, scored by the number of capability flags reached out of 16, where achieving arbitrary code execution counts as achieving all 16 capabilities. The GPT-6 Astra score is taken from https://openai.com/index/gpt-6-astra/ ; the Claude Opus 5.5 and Claude Mythos 5.1 scores are taken from https://www.anthropic.com/claude-opus-5-5-system-card ; the GLM-5.3 score is taken from https://z.ai/blog/glm-5.3” “ExploitGym. Kimi K3.1 Pro is evaluated with the KimiCode harness under a 6-hour wall-clock budget per task.” | 无（待补：表为画布渲染；`02` 区间为本组图，表未截到） | 对照模型分数全部引自他家报告，不是 Kimi 重测；脚注写的是 “Claude Mythos 5.1”，表头列名是 “Claude Fable 5.1”，二者名称不一致（疑为表头与脚注笔误，待 Claude 核）。ExploitBench 用官方 harness、300 轮预算、pass@5（5 个种子取最优）；xdev 用 pass@3，口径不同。K3.1 Pro 在 ExploitBench / ExploitGym 上均低于三家闭源（及差 GPT-6 Astra 25.3 / 21.7 分）。DeepSeek 的 ExploitBench/ExploitGym 栏为空 | 已找到（未截图） |
| A4-a | AA 指数 55 与名次 | L3 | https://artificialanalysis.ai/articles/kimi-k3-1-pro-launch-benchmarks ；https://artificialanalysis.ai/models/kimi-k3-1-pro ；https://artificialanalysis.ai/leaderboards/models | 文章标题：“Kimi K3.1 Pro launches at 4th on the Artificial Analysis Intelligence Index scoring 55”；“It trails only Claude Opus 5.5 (max with fallback, 58), Claude Sonnet 5.5 (max with fallback, 56), and Muse Wonder Full (max, 56)”；模型页 “#4 / 215”“55” | 03-aa-index-raw.png | 文章与模型页按模型去重（Opus 5.5 的 xhigh 档 56 不另计）。总榜表按行排：Opus 5.5 max 58、Sonnet 5.5 max 56、Opus 5.5 xhigh 56、Muse Wonder Full 56、Kimi K3.1 Pro 55 → 在表里是第 5 行；不是并列。榜上“215”与“709 models”“250+”等分母出现在同一页的不同位置，分母口径未说明 | 已找到 |
| A4-b | Terminal-Bench 4.0 67% | L3 | 同文章；https://artificialanalysis.ai/evaluations/terminalbench-4-0 | 文章：“Kimi K3.1 Pro also achieves the highest score of any model on Terminal-Bench 4.0. K3.1 Pro scores 67%, up from 13% for Kimi K3 and ahead of Claude Sonnet 5.5 (64%).”；评测页：“Kimi K3.1 Pro scores the highest on Terminal-Bench 4.0 with a score of 67.2%, followed by Claude Sonnet 5.5 (Max, Default Fallback) with a score of 63.6% and Claude Opus 5.5 (Max, Default Fallback) with a score of 59.6%”；“We run all 66 Terminal-Bench 4.0 tasks with the mini-swe-agent harness and report pass@1 averaged over three repeats per task.” | 04-aa-tbench-raw.png | AA 用 mini-swe-agent harness：K3.1 Pro 67.2、Opus 5.5 59.6。Kimi 博客自报用 Claude-Code harness：K3.1 Pro 65.7、Opus 5.5 66.4（“taken as the maximum value reported in either their blog posts or the official … leaderboard”）。同一基准两套数字，Opus 的位置相反（同一指标不同来源，全部记录） | 已找到 |
| A4-c | AutomationBench-AA 73% | L3 | 同文章 | “AutomationBench-AA, which tests agentic SaaS workflows, rises from 58% to 73%.” | 无 | 是 K3→K3.1 Pro 的变化；文章未列此项的对照模型排名。Kimi 博客脚注：“GDPval-AA v2.1, AA-Briefcase v1.1 and AutomationBench-AA scores are cited from https://artificialanalysis.ai/” | 已找到 |
| A4-d | 每任务成本两处数字 | L3 | 文章与模型页 | 文章：“Cost per Task roughly equal to Kimi K3 at $1.98 per Task”；模型页与总榜：“$2.11 Cost per Intelligence Index task”（榜表 $2.11）。文章：“55K output tokens per Intelligence Index task”；模型页：“150M Output tokens from Intelligence Index” | 无 | 两处都在 AA 原站，数字不同，原因页面未解释（文章当天首发，模型页可能后续更新；推断，未证实）。Implicator 引 $2.11 | 已找到（不一致，全部记录） |
| A5-a | 参数规模 | L1/L3 | 官方博客；AA 文章 | 官方：“our most capable 3T-class open-weight model to date, with a 1-million-token context window”；AA：“Size: 2.8T total, 104B active parameters” | 无 | 官方只写“3T-class”，没给总/激活参数；2.8T/104B 只见于 AA（L3，来源未标）。Reuters 写 “3-trillion-parameter-class” | 部分支持（2.8T/104B 官方未给） |
| A5-b | API 价格 | L1 | https://platform.kimi.ai/docs/pricing/chat（存档 a-kimi-pricing.html；价格表由页面脚本渲染，.txt 里没有，取自 HTML 内嵌数据） | kimi-k3.1-pro：Input $2.00 / MTok，Output $10.00 / MTok，Cached Input $0.20 / MTok，5m writes $2.00，1h writes $4.00；kimi-k3.1-pro-highspeed：$4.00 / $20.00 / $0.20 / $4.00 / $8.00；kimi-k3：$3.00 / $15.00 / $0.30 | 无 | 官方博客称 Highspeed 为 “2-3× the speed”；AA：“$2/$10/$0.20 per 1M input/output/cache hit tokens, 33% lower than Kimi K3 on all three rates” | 已找到 |
| A6 | AA Cyber Index 64% | L3 | https://artificialanalysis.ai/evaluations/cyber-index（404） | AA 文章、模型页、总榜、评测列表（a-aa-*.txt）均无 “cyber” 字样 | 无 | 取证时刻 2026-10-24 约 20:20（北京时间） | 未找到一手来源 |
| A7 | 媒体框架 | L4 | 见下节“夸大说法实例”。Reuters 原站：https://www.reuters.com/technology/moonshot-curbs-hacking-ability-new-kimi-model-fears-grow-over-ai-powered-2026-10-24/ curl 返回 401 | — | — | Reuters 原站打不开，记失败；正文取自 Rappler 的 Reuters 转载（二手，仅作 L5 线索，且不用于定级） | 未找到一手来源（Reuters 原站）；转载见下 |

## 可能的吠点（原文里有、线索摘要没提或说反的条件）

1. 博客图表里 DeepSeek-V4.1-Flash（85.7）高于 K3.1 Pro（81.0），GPT-5.6 Sol（第三方评测）为 100——“81.0%”不是同类最高，也不是厂商最低。出处：xdev-bench-hard 图表（02）。
2. 同一图表各行 harness 不同（kimi-code / claude-code / codex），比较并非同一条件。出处：同上。
3. xdev-bench-hard 是自家基准、未公开任务与评分细节，值是 1/21 的整数倍（推断约 21 题）；“pass@3 over a random subset of runs”表述含糊。出处：博客 Cybersecurity 段。
4. 去数据的措辞是 “For the open-weight release”，权重尚未发布（by Nov 4），API/应用已上线的版本是否同一检查点，博客未说明。出处：博客 Cybersecurity 与 Availability 段。
5. 博客自述 K3.1 Pro 相对 K3 “improves substantially”（图表 33.3→81.0），且 “we intend to progressively raise the cyber capability available in open weights”：下一步计划提高而不是维持。出处：博客。
6. 博客本身给出结论“stronger general coding … generalize to cyber tasks even after targeted data removal”，即去数据只削掉一部分。出处：博客。
7. ExploitBench / ExploitGym 对照分数全部引自他家报告，不是 Kimi 重测；脚注把 Anthropic 对照写成 “Claude Mythos 5.1”，表头写 Claude Fable 5.1（名称不一致，待核）。出处：博客 Cybersecurity benchmarks 脚注与表。
8. 两份厂商文档幻觉数字不同：博客 Hallu Bench Hard 8.3%（K3 23.4%），API 快速上手页 6.85%（K3 23.35%，另一测试集）。与网络安全话题无关，仅作厂商文档一致性线索。出处：a-blog.txt、a-quickstart.txt（Implicator 亦指出）。
9. Kimi 自报 Terminal-Bench 4.0 65.7%（落后 Opus 5.5 的 66.4%），AA 实测 67.2%（Opus 5.5 为 59.6%）：同一基准名次相反，harness 不同。
10. HN 社区质疑（仅 L6 线索，Algolia 不提供评论得分；下列均在帖 https://news.ycombinator.com/item?id=50212065，得分 946/评论 553 以上；提交于 2026-10-24 00:59 北京时间）：
    - https://news.ycombinator.com/item?id=50213204 “how much ‘cybersecurity data’ is just standard software knowledge that one can derive cyber security exploits from?”（大意：界定模糊）
    - https://news.ycombinator.com/item?id=50213031 权重发布后微调即可补回网络能力吗（大意）；对应官方并未答复该问题（Implicator 亦指出博客未论及）
    - https://news.ycombinator.com/item?id=50212365 “So they admit that they could do more, but they won't?”（对 “below what our pipeline can produce” 的解读；回复 50212851 引原文说明是对照实验）
    - 另有评论称“与美国模型一样被削弱”（50212318）、“是否为避免权重发布被监管”（50213165，称有传闻，无来源）——只记录，不采信。

## 夸大说法实例

未找到把 81.0% 写成“能力下降”“失去攻击能力”的英文或中文稿，已读的几篇报道大体忠实于博客：

- Reuters（经 Rappler 转载，页面标 2026-10-24 13:42 PHT = 13:42 北京时间，为转载页发布时间，非 Reuters 原发时刻）：https://www.rappler.com/technology/moonshot-curbs-hacking-ability-kimi-model/ 标题 “Moonshot curbs hacking ability in new Kimi model as fears grow over AI-powered cyberattacks”（标题带“担忧加剧”框架，正文给出 81.0%/90.5% 并写 “although it still outperformed its predecessor. The results have not been independently verified.”）。正文还写 Moonshot “said on Saturday, October 24”（官方博客与 HN 提交时间折算为北京时间 10 月 24 日凌晨，美国时间 10 月 23 日）。正文无“能力下降”夸大。
- The Next Web，2026-10-23T18:19:34Z（= 10-24 02:19 北京）：https://thenextweb.com/news/moonshot-kimi-k3-1-pro-open-weight-cyber 标题 “Moonshot releases Kimi K3.1 Pro and says it cut its hacking skills”；正文 “The released model solved 81%, still far above K3's 33.3% …”；写出 DeepSeek V4.1-Flash 85.7%。忠实。
- Implicator.ai，2026-10-23 17:37 PST（= 10-24 08:37 北京）：https://www.implicator.ai/kimi-k3-1-pro-hacking-cut-unpublished-test/ 标题 “Kimi K3.1 Pro Hacking Cut Rests on Unpublished Moonshot Test”：质疑方向，写出 DeepSeek 85.7、GPT-5.6 Sol 100、K3 33.3。
- StratNews Global：https://stratnewsglobal.com/technology/moonshot-kimi-model-hacking-capabilities-ai-cyberattacks/ 首句 “has reduced the offensive cyber capabilities of its latest artificial intelligence model”；“amid growing concerns that increasingly autonomous AI systems could make sophisticated cyberattacks easier and cheaper to execute”（沿用 Reuters 框架）。
- 新浪（微博转载）：https://www.sina.cn/weibo/detail/5357509605070112.html 标题 “Kimi 砍掉攻防语料？官方：本地可审计，安全能力会泛化回来”：只写“砍掉的是训练侧的攻防语料和 RL 环境，不是产品里一问网络安全就完全一片空白”，是澄清而非夸大。
- 值得注意的措辞风险：Reuters、TNW 标题都用 “curbs hacking ability” / “cut its hacking skills”；单读标题会读成“能力被砍掉”，实际 81.0 对 90.5 是相对假想对照的下降，且比 K3 高很多。

## 扫描说法勘误

- “Terminal-Bench 4.0 67%”：AA 为 67.2%（mini-swe-agent，AA 实测）；Kimi 博客自报 65.7%（Claude-Code harness）。两份扫描只用其一，需标明口径。
- “AA 每任务成本 $1.98 与模型页 $2.11 不一致”：属实，两处都在 AA 原站，已全部记录。
- “名次 #4/215，总榜按并列为第 5”：不是并列。文章/模型页 #4 是按模型去重；总榜表按行排，Opus 5.5 的 max 与 xhigh 两档都在表里，Kimi 排第 5 行，其前有 Muse Wonder Full 56、Opus 5.5 xhigh 56。
- “90.5→81.0 是相对假想对照”：属实，且图表给出 K3 33.3、DeepSeek-V4.1-Flash 85.7、GPT-5.6 Sol 100（第三方评测），扫描摘要没提。
- “2.8T / 104B”：官方只写 “3T-class”，2.8T/104B 只见于 AA（来源未标）。
- “API $2/$10”：属实（platform.kimi.ai 价格表）；Highspeed 为 $4/$20，另有缓存命中 $0.20、5m/1h 写缓存 $2/$4。
- “AA Cyber Index 64%”：AA 原站无此页（404）、AA 页面搜索无 “cyber”；未找到一手来源。
- 扫描说 Reuters 页 401：属实；Reuters 正文只能经 Rappler 转载看到（仅作 L5 线索）。
- HN 热度：主帖 id 50212065 为 947 分/566 评（`barking heat` 20:24 北京时间），Algolia 条目 946 分/553 评；另帖 50213337（AA 55）119/40、50212246（重复提交）60/2。
- 发布日期：官方未给时刻；HN 提交时间折北京时间 10-24 00:59，AA 文章日期 “October 23, 2026”；Reuters 称 “Saturday, October 24”。内部核验表按北京时间写 10-24。

## 未完成项

- `02` 区间原定的“Cybersecurity benchmarks 表”截图未得（表为画布渲染，截图空白）；表中数值只来自页面源码内嵌 JSON，未有图。
- 官方 X 帖/发布时刻未取（超出本组 60 分钟与范围限制）。
- `02-kimi-xdev-chart-raw.png` 是官方图片文件（statics.kimi.ai）由 2400 px 宽等比缩到 1400 px（lanczos），不是页面截图；未改任何数字。
