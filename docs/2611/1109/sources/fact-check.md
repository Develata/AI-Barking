# 1109 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`（Fable 5.2 价格、跑分与成本）、`evidence-b.md`（费马与黑洞）、`evidence-c.md`（“AI 泡沫”长文）、`evidence-d.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证于北京 11-10 15:28 至约 16:35；Claude 另于 16:40 前后对照存档抽查了下文标“抽查”的条目。

Claude 与第一条的当事模型同属 Anthropic，第二条引的也是 Anthropic 招股书；两条靠跨模型复核把关。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：Claude降价反贵一倍？ | L1/L3 | ✅ | 见 A 节 | 问号。“降价”= 每 token 单价从 $10/$50 降到 $8/$40；“贵一倍”= AA 综合指数最高档平均每题成本 Fable-5.2 $26.54 对 Fable-5.1 $12.99（2.04×）。只在最高档成立，正文吠点①写明中、低档反而便宜 |
| 标题：AI没泡沫？ | L1 | ✅ | 长文标题 “I'm Not Convinced The AI Bubble Exists” | 问号，指博文主张，本号不判断泡沫有无 |
| 省流：Claude 的开发商 Anthropic 发布 Fable-5.2，单价降20% | L1 | ✅ | 见 A 节 | 单价口径 |
| 省流：据第三方 AA 综合指数评测，最高档平均每题花费却翻倍 | L3 | ✅ | 见吠点① | AA Intelligence Index v5.0.1 的加权平均每题成本，max 档，与 Fable-5.1 max 比 2.04×。省流卡不放此句（视觉复核 BLOCKER） |
| 省流：“AI 没泡沫”热文引 Anthropic 二季度的2.08倍，招股书修订版里三季度已降到1.72倍 | L1 | ✅ | 见 C 节吠点① | 2.08 = 2026Q2 单季；1.72 = 2026Q3 单季。省流卡改摘事件事实，不放此句（视觉复核 BLOCKER） |

## A Fable-5.2：价格与成本

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间11月10日凌晨，Anthropic 发布 Claude-Fable-5.2 | L1 | ✅ | https://www.anthropic.com/claude-fable-and-mythos-5-2 （`a-release.*`）；官方社媒帖 | 发布页只写 November 9, 2026（美国日期）；HN 首帖北京 11-10 02:01:10，官方社媒帖北京 02:57:59 与 03:00:21（`evidence-a.md` A1） |
| 单价比5.1降20% | L1 | ✅ | 发布页（图 01b）；价格页 https://platform.claude.com/docs/en/about-claude/pricing （图 04） | 发布页 “$8 per million input tokens and $40 per million output tokens”，只说 “a lower price”；5.1 的 $10/$50 取自价格页 Fable 5.1 行。8/10、40/50 均为 −20%。Claude 抽查 |
| 官方称，早期测试中，编程测试 FrontierCode-2.0 上它开 High 档与 Opus-5.5 最高档几乎同分，每道题花费约一半 | L1 | ✅ | 发布页（图 03）；`a-release-chart-points.txt` | “In early testing on the FrontierCode 2.0 evaluation, Fable 5.2 at High effort scored about level with Opus 5.5 at Max effort, for roughly half the cost per task.”；45.3% 对 45.5%，$6.72 对 $12.82（52.4%）。系统卡：评测由 Cognition 跑，各模型用各自的 CLI，单次结果无不确定区间（图 11、12）。Claude 抽查图点 |
| 吠点①：有中文稿写“使用成本降低20%” | L4 | ⚠️ | 智通财经 https://www.zhitongcaijing.com/content/detail/1520763.html （`a-news-zhitong.*`），2026-11-10 06:00 | 原句 “使用成本则较5.1版本降低20%”，Claude 对存档逐字核对。核验对象是“该媒体确实这样写”，按分级 L4 标 ⚠️（反向核验第一轮 SHOULD_FIX 4） |
| 吠点①：其实降的是单价 | L1 | ✅ | 同上 | 发布页与价格页只给每 token 牌价 |
| 吠点①：据 Artificial Analysis 综合指数评测，最高档平均每题26.54美元，是 Fable-5.1 最高档的2倍 | L3 | ✅ | https://artificialanalysis.ai/models/claude-fable-5-2 （`a-aa-model.txt`，图 08）；Fable 5.1 max（`a-aa-fable51.txt`，图 09） | 指标：Artificial Analysis Intelligence Index v5.0.1（9 项评测）的 “Cost per Intelligence Index task”，按各评测任务数平均、按指数权重加权，是每题平均，不是总成本。档位：两者均为 max（AA 标 “Max, Default Fallback”）。$26.54 ÷ $12.99 = 2.04。A 组称原因是输出 token（每题加权平均 273,776 对 117,941，取自 AA 页面内嵌数据），文本存档里找不到这两个数，待核，正文未用（反向核验第二轮 NEEDS_VERIFICATION 7）。存档时刻 15:29–15:30，AA 数字会漂移。Claude 抽查图 08、09 |
| 吠点①：中、低档反而便宜约三成 | L3 | ✅ | AA 各档页（`a-aa-fable-medium.txt`、`a-aa-fable-low.txt`、`a-aa-fable51-medium.txt`、`a-aa-fable51-low.txt`） | 同一指标同档对比：medium $3.80 对 $5.56（0.68×），low $3.10 对 $4.47（0.69×）。Claude 抽查四个存档。high 档 1.20×、xhigh 1.37×，未写入正文 |

## B Fable-5.2：费马与黑洞

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 发布页还称，1000个智能体在少量人工干预下30小时完成费马大定理的形式化 | L1 | ✅ | 发布页（图 13、14） | “a team of 1,000 Fable 5.2 agents fully formalized the theorem in just 30 hours, with minimal human intervention”。30 小时 = 18 + 12 小时预算；claude.dev 写最终定理在第 28.7 小时完成（图 16），不矛盾 |
| （把已有证明翻成机器能核对的形式） | L1 | ✅ | 发布页 “Formalization is the process of translating a mathematical proof into a language that a computer can check step by step.”；9 月原文（图 19） | 定理 1995 年由 Wiles 证明；9 月原文称沿用 Darmon、Diamond、Taylor 的简化路线 |
| 没用数学参考 | L1 | ✅ | 发布页 | “used no mathematical references, only a list of intermediate theorem statements to score progress”（发布页自述，与吠点②并列） |
| 还解决了两个黑洞物理开放问题 | L1 | ✅ | 发布页（图 20） | “Claude Fable 5.2 solved two open problems in theoretical physics. Both concern black holes”（发布页自述，句首“发布页还称”覆盖） |
| 吠点②：官方技术博客写，用的是早期版本 | L1 | ✅ | https://claude.dev/blog/safely-scaling-multi-agent-teams-with-fable-5-2/ （`b-claudedev.txt`，图 15） | “a team of 1,000 agents running an early version of Claude Fable 5.2”；系统卡 “an earlier snapshot of Fable 5.2”、图例 “Claude Fable 5.2 (earlier version)”（图 24）。发布页图例只写 “Fable 5.2” |
| 吠点②：开跑前给了上次已证明的47条中间结论和证明思路 | L1 | ✅ | claude.dev（图 15） | “We gave the agents a list of 60 statements, a one-sentence sketch of the classical route, the definitions those statements use, and 47 proved lemmas from our earlier formalization”。“中间结论”= proved lemmas（引理）；“证明思路”= one-sentence sketch of the classical route。系统卡同义：“building on definitions and smaller proven results from an earlier formalization”。文中另有“47/60 条被判预算内不可行”，是另一个 47，正文未用。Claude 抽查 |
| 吠点②：第18小时人工接着跑 | L1 | ✅ | claude.dev 图 4 说明；系统卡；发布页图注（图 14、16、24） | claude.dev “we restarted the run from its last save and gave the agents 12 more hours”；发布页图注 “At the dashed line (18 hours), both teams resumed from their last save with 12 more hours.” 第 18 小时完成度 0.86。“人工”= 研究者重启运行，原文 “we restarted”。Claude 抽查 |
| 吠点②：新智元却写“第一次独立地彻底证明了费马大定理” | L4 | ⚠️ | 凤凰网转载新智元 https://tech.ifeng.com/c/8xugZ5rMQG3 （`b-ifeng-zuiqiang.txt`），11-10 06:41:33 | 原句 “在 30 小时内，第一次独立地彻底证明了费马大定理，还完成形式化验证！”，Claude 对存档逐字核对。核验对象是“该媒体确实这样写”，按分级标 ⚠️ |
| 吠点③：两篇论文发在自称不做同行评审的预印本站 | L1/L3 | ✅ | 发布页两个预印本链接指向 hexagonmath.org/2611.00175v1 与 2611.00174v1；Hexagon 首页（`b-hexagon-home.txt`，图 23） | Hexagon 首页 “Hexagon does not perform peer review.”（站方自述）。预印本本身没写“未经同行评审”。两篇日期 Nov 07, 2026。Claude 抽查 |
| 吠点③：署名只有 Claude | L1/L3 | ✅ | Hexagon 记录页与 PDF（`b-bh-174*`、`b-bh-175*`，图 21、22） | PDF 署名 “Claude (Anthropic)”；页面 “Contributed by Anthropic”；无具名人类作者。175 写由一位物理学家给提示并核对特例，174 写一位物理学家给简短指导并核对每个主要结果（正文未展开） |
| 吠点③：写的是“内部 Claude 研究模型”，没提 Fable-5.2 | L1/L3 | ✅ | 同上 | 175 “An internal Claude research model, prompted by a physicist, obtained the result presented here.”；174 “The main result was first obtained autonomously by an internal Claude research model.” 四个存档（两篇记录页、两篇 PDF 文本）grep “Fable” 均为 0，Claude 抽查。本号不推断该内部模型是不是 Fable-5.2 的早期版本。174 另写公开版 Opus 5.5 后来独立得到同一结果（为压字数，正文已删） |

## C “AI 没泡沫”博文

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 博文《我不信 AI 泡沫存在》 | L1（作者原文） | ✅ | https://matteoraso.github.io/im-not-convinced-the-ai-bubble-exists.html （`c-blog.txt`） | 原题 “I'm Not Convinced The AI Bubble Exists”，页面 “Posted on Sun 08 November 2026”。正文不点作者名 |
| 在 Hacker News 引来近千条评论 | L6（平台计数） | ⚠️ | https://news.ycombinator.com/item?id=50418975 ；计数取自 `c-heat.json`（北京 11-10 15:59:55 采样：550 分、944 评） | 采样时的平台计数，写成“近千条”。`c-hn-50418975.json` 是评论树（914 个节点），不作计数依据（反向核验第一轮 SHOULD_FIX 3）。发帖北京 11-09 07:44:36 |
| 作者自称不是泡沫专家 | L1 | ✅ | 博文 | “I'm not going to pretend that I'm an expert on financial bubbles” |
| 作者引 Anthropic 招股书称每花1美元算力进账2.08美元 | L1 | ✅ | 博文（图 26） | “Anthropic's S-1 filing says that they make $2.08 for every $1 of compute.” 作者原话 |
| 又称建数据中心是一次性支出，建完就骤降 | L1 | ✅ | 博文（图 25c、26） | “The reason why AI companies are spending so much money is to rollout data centres … These expenses are one-off purchases, not regular operating expenses. Once all the data centres are built, AI spending is going to plummet.” |
| 吠点①：2.08倍是 Anthropic 二季度营收除以算力与基础设施费用 | L1 | ✅ | 首版 S-1 MD&A 图 “Significant increase in revenue per dollar of compute”（图 29）；季度利润表（`c-s1-excerpts.txt`） | 图上 Q2'26 “2.08x”，图例 “In-period revenue / compute”；11,555 ÷ 5,558 = 2.079（百万美元）。单季口径。“算力与基础设施费用”= 利润表 “Compute and infrastructure” 整行 |
| 吠点①：11月2日的招股书修订版里，三季度已降到1.72倍 | L1 | ✅ | S-1/A 同图（图 30）；EDGAR 受理北京 11-02 22:24:24 | 图上 Q3'26 “1.72x”；正文 “This reduced revenue per dollar of compute compared to the preceding quarter.”；16,956 ÷ 9,863 = 1.719。“11月2日”美东与北京同日。Claude 目视核对图 30 |
| 吠点①：博文页面标11月8日，引的还是首版 | L1 | ✅ | 博文（`c-blog.html` 中 sec.gov 链接仅 1 个，指向 000162828026067168 即首版 S-1） | 页面只标 “Sun 08 November 2026”，时区未知，正文写“页面标11月8日”（反向核验第二轮 SHOULD_FIX 6）；全文没提三季度或修订版 |
| 吠点②：进账不等于赚到：人员、销售等没扣 | L1 | ✅ | 首版 S-1 利润表与 “Compute and infrastructure” 定义（图 31、34）；S-1/A 季度利润表（图 32） | 比值分母只有 Compute and infrastructure 一行；利润表另列 Personnel、Sales, marketing, and partnerships、Other operating and administrative。该行本身含研究与内部使用的算力，2026 年起含租赁成本与服务器折旧（图 34），正文未展开 |
| 吠点②：修订版显示其三季度经营亏损5.67亿美元 | L1 | ✅ | S-1/A 季度利润表（图 32）；MD&A “Our operating loss was $567 million for the three months ended September 30, 2026” | GAAP 口径；Q2 为经营盈利 9.74 亿。1102 期已核。Claude 抽查 |
| 吠点③：说是一次性，修订版却写：Anthropic 采购与租赁承诺约8954亿美元，几乎都是算力与基础设施，部分附解约权 | L1 | ✅ | S-1/A（图 33；`c-s1-excerpts.txt`） | “are approximately $895.4 billion, substantially all of which is related to compute and infrastructure”；构成为已签采购承诺与租赁协议（含短期承诺与尚未起租的租赁），其中约 2,138 亿附解约权或前提条件（图 33） |
| 吠点③：约一半预计2031至2045年才发生 | L1 | ✅ | S-1/A（`c-s1-excerpts.txt`） | “approximately $447.5 billion expected to be incurred before December 31, 2030, and substantially all of the remainder is expected to be incurred between 2031 and 2045”。895.4 − 447.5 = 447.9。“发生”= incurred。这是 Anthropic 一家的承诺，作者说的是全行业，正文写成“Anthropic 采购与租赁承诺” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Meta 发布小模型 Muse-Ember，输入每百万 token 0.1美元起 | L1 | ✅ | https://research.meta.ai/blog/muse-ember （`d-d1-meta-muse-ember.txt`），11-09（官方未给时刻） | “It costs $0.10 per million input tokens … and $0.50 per million output tokens”；脚注：超过 200K token 的提示输入 $0.30。“起”= 200K 以内的最低档。Meta 称其为 “our smallest” 模型。权重是否开放原文未说；成本对比（反向核验第一轮 BLOCKER 2）已删 |
| 谷歌发布 TPU-8i-Max，称算力、内存带宽均为 TPU-8i 的2倍 | L1 | ✅ | https://blog.google/innovation-and-ai/infrastructure-and-cloud/global-network/tpu-8i-max-and-8tx/ （`d-d2-*.txt`），北京 11-09 21:00 | “delivering double the compute, double the SRAM and double the HBM bandwidth of TPU 8i”；“带宽”= HBM 带宽。同文另发 TPU 8tx；未给价格与上市时间 |
| PSI 称用非公开 Anthropic 模型证出18年量子猜想，Lean 已接受 | L1 | ✅ | https://psi.inc/research/aaronson-ambainis/ （`d-d5-psi-*.txt`），11-06 | “running a non-public Anthropic model”；“Lean … accepted it”；猜想 “Open since 2008”。README：主猜想的 Lean 证明无占位、无新增公理；第四条（更强改进界）以未形式化的外部结果为前提；论文本身未经机器核对 |
| Asahi Linux 称部分开发中代码遭 AI 改写挪用，开始改用 GPL | L1 | ✅ | https://asahilinux.org/2026/11/progress-report-7-3/ （`d-d4-*.txt`），11-07 | “In some extreme cases, in-development feature code was being lifted straight out of our personal forks and published without attribution or lazily rewritten with LLM assistance”（项目方单方陈述，未点名）；“begin the process of transitioning all of our repositories to the GNU General Public License version 2 or later” |
| 据 EE Times 报道，DeepX 二代 AI 芯片流片，预计明年3月出样 | L4 | ⚠️ | https://www.eetimes.com/deepx-tapes-out-second-gen-chip-projects-6-7x-performance-gain-with-lpddr5x-pim/ （`d-d3-eetimes-opencli.txt`），11-09 | “DeepX has taped out its second-generation AI accelerator”；“samples of the M2 are expected in March 2027”。6.7 倍（仿真、加 LPDDR5x-PIM 跑 Llama3.1-8B、相对 DX-M2 单独）与总功耗 2.7 倍已从速览删去（反向核验第一轮 BLOCKER 1）。DeepX 官方新闻稿未找到 |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| half-cost：在 FrontierCode 2.0 的早期测试中，Fable 5.2 开 High 档的得分与 Opus 5.5 开 Max 档大致持平，每道题的花费约为一半 | L1 | ✅ | 发布页（图 03，`half-cost-raw.png`） | 原句见 A 节第 3 行 |
| aa-cost：Fable 5.2（最高档）AA 综合指数平均每题成本 26.54 美元；Fable 5.1（最高档）同一指标为 12.99 美元 | L3 | ✅ | AA 模型页（图 08、09，`aa-cost-raw.png`；`a-aa-model.txt`、`a-aa-fable51.txt`） | 两页拼接，指标名 “Cost per Intelligence Index task” |
| fermat-setup：1000 个智能体运行 Claude Fable 5.2 的早期版本；给了一句话的经典证明路线概要、这些陈述用到的定义，以及上次形式化中已证明的 47 条引理 | L1 | ✅ | claude.dev（图 15，`fermat-setup-raw.png`；`b-claudedev.txt`） | 原句见 B 节 |
| fermat-chart：第 18 小时 Fable 5.2 一队的完成度（已证明陈述的加权占比）为 0.86；在虚线处（第 18 小时）两队都从上次存档处继续，再给 12 小时 | L1 | ✅ | 发布页进度图与图注（图 14，`fermat-chart-raw.png`） | 纵轴 “Completion (0–1, weighted share of statements proved)”；图注 “At the dashed line (18 hours), both teams resumed from their last save with 12 more hours.” |
| blackhole-provenance：主要结果最先由一个内部 Claude 研究模型自主得到；Hexagon 不做同行评审 | L1/L3 | ✅ | Hexagon 174 页（图 22）与首页（图 23），`blackhole-provenance-raw.png` 三段拼接 | 原句见 B 节 |
| blog-208：这些支出是一次性购买，不是常规运营开支；他们每花 1 美元算力进账 2.08 美元 | L1 | ✅ | 博文（图 26，`blog-208-raw.png`） | 作者原话 |
| s1a-ratio：营收除以算力与基础设施费用，二季度 2.08 倍、三季度 1.72 倍；这使每美元算力带来的营收比上一季度下降 | L1 | ✅ | S-1/A 图与正文（图 30，`s1a-ratio-raw.png`） | “This reduced revenue per dollar of compute compared to the preceding quarter.” 主语为短期算力交付抵消了效率提升 |
| s1a-q3：截至 2026 年 9 月 30 日的三个月营收 16,956、算力与基础设施费用 9,863、经营亏损 567（百万美元） | L1 | ✅ | S-1/A 季度利润表（图 32，`s1a-q3-raw.png`；`c-s1-excerpts.txt` 末段补入全表） | 底图为表头、营收与算力两行、经营损益一行的拼接 |

## 视觉复核

Codex（gpt-6-astra，medium，只读）看 24 张图（成图与底图）：1 BLOCKER、6 SHOULD_FIX、1 NICE_TO_HAVE。

1. BLOCKER（省流卡第二条“修订版里已降到1.72倍”读成同期修订，且把吠点放上省流卡）：采纳。省流卡两条改摘事实段；正文省流写明“二季度”“三季度”。
2. SHOULD_FIX（省流卡第一条“每道题的花费却翻倍”缺范围）：采纳，省流卡删去此句；正文省流补“AA 综合指数”“平均”。
3. SHOULD_FIX（TPU“带宽”泛化）：采纳，改“内存带宽”。
4. SHOULD_FIX（事件卡漏“早期测试”“少量人工干预”）：采纳，正文与事件卡同步补入。
5. SHOULD_FIX（费马进度图译注“完成度”无口径）：采纳，补“已证明陈述的加权占比”。
6. SHOULD_FIX（黑洞批注图字小，建议拆两张）：不采纳。底图宽度由原页 2030 px 决定，拆成两张仍按同一宽度缩放，字号不变；关键句已高亮并配译注。交付时向 Develata 说明。
7. SHOULD_FIX（吠点卡②第③条编号紧贴开引号）：已解决，该条改写后不再以引号开头。
8. NICE_TO_HAVE（小标题“引的是旧数”是判断）：采纳，改为“热文与招股书”。

## 读者稿检查

Codex（gpt-6-astra，medium）读者稿检查报告见 `style-review.md`，采纳记录在其末尾。

## 反向核验

### 第一轮（Codex gpt-6-astra，medium，只审正文与事实清单；卡片尚未生成）

1. BLOCKER（DeepX 速览丢掉“加 PIM、Llama3.1-8B、相对 DX-M2 单独”的条件）：采纳。核 `d-d3-eetimes-opencli.txt` 原句属实。速览改为只报流片与出样时间，倍数删去。
2. BLOCKER（Meta 成本比较丢掉 “estimates at proposed pricing”）：采纳。速览改为官方价格，成本比较删去。
3. SHOULD_FIX（944 评论的存档指向错误）：采纳。依据改指 `c-heat.json`，正文改为“近千条评论”。
4. SHOULD_FIX（L4、L6 标 ✅ 不合分级）：采纳。智通财经、新智元两条与 HN 计数改标 ⚠️，备注写明核验对象。
5. SHOULD_FIX（`evidence-d.md` 把 “2.7× the power consumption overall” 写成“多 2.7 倍”）：采纳，已改取证记录。
6. NEEDS_VERIFICATION（卡片、两份 README 尚不存在）：卡片已生成，第二轮补审。

### 第二轮（Codex gpt-6-astra，medium，含卡片、两份 README）

PASS，无 BLOCKER。

1. SHOULD_FIX（省流“每道题”把平均写成逐题）：采纳，改“AA 综合指数评测，最高档平均每题”。
2. SHOULD_FIX（8954 亿漏条件）：采纳，改为“采购与租赁承诺……几乎都是算力与基础设施，部分附解约权”。
3. SHOULD_FIX（TPU 带宽）：同视觉复核 3。
4. SHOULD_FIX（季度利润表摘录没有表格数据）：采纳，`c-s1-excerpts.txt` 末尾补入 S-1/A L17308–17485 全段。
5. SHOULD_FIX（批注译注没有进“文中事实”列）：采纳，新增“批注译注”一节。
6. SHOULD_FIX（博文日期时区未知）：采纳，正文改“博文页面标11月8日”。
7. NEEDS_VERIFICATION（输出 token 数与成本归因无文本存档）：采纳为待核，A 节备注已改，正文与卡片未用。
