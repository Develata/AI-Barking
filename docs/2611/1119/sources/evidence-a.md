# 1119 期 A 组取证：Frostbite（Architect Labs）“能效 4.59 倍”

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时刻均为北京时间（UTC+8），括号保留原时区。主存档：`sources/a-blog.html|.txt|.headers`（博客，2026-11-20 22:11:58 北京取）。博客页内图表原图另存 `sources/a-fig-perf-per-watt-heldout.png`（Fig. 11）、`a-fig-workload-suite.png`（Fig. 3）、`a-fig-energy-breakdown.png`（Fig. 12），均取自 architectlabs.com 的 `/_next/static/media/`，只存本地不入库。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。“推算”是本组用博客自己的数字做的算术，不是原文。

## 一、逐条证据表

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1 | 博客标题与发布日期 | L1 | https://architectlabs.com/blog/frostbite | 标题：“Introducing Frostbite: A new CPU architecture discovered, designed, and verified end-to-end by AI”；页面标 “Research November 18, 2026”。 | `01-frostbite-top-abstract.png` | 官方只给日期，未给时刻（HTML 无 datePublished；博客列表页 `a-blogidx.txt` 同样只有日期）。时区未标。线索：第三方 LinkedIn 帖 Exa 元数据 2026-11-18 21:17 UTC（北京 11-19 05:17）、Snape Report 2026-11-18 21:30 UTC，均为聚合元数据未核；HN 最早提交 2026-11-19 15:33 北京（`50557080`）。故博客最晚 11-19 05:17 北京前已上线（推算，弱证据） | 已找到 |
| A1 | 4.59x 头条句 | L1 | 同上 | “On a set of 25 initially chosen workloads, including 15 held out until the final evaluation, we achieved a 4.59x performance-per-watt multiple when compared to a modern, silicon-proven superscalar (dual-issue, in-order) CPU core.” | `01-frostbite-top-abstract.png` | 指标是 performance-per-watt；对照是“一颗核” | 已找到 |
| A1 | 结果段全文（8.77 / 4.59 / 6.02 / 3.83） | L1 | 同上 | “Across all the workloads, we average an 8.77x performance-per-area, and across all 25 workloads, search set and held out together, Frostbite reaches a geometric mean of 4.59x the TCPU's performance-per-watt: 6.02x on the 10 search-set workloads, and 3.83x on the 15 held-out workloads it was never tuned on. As expected, the architecture does most strongly on highly parallelizable tasks, but it also outperforms the TCPU on branching, dynamic code.” | `02-tcpu-quad-comparison.png` | 8.77x 只写 “average”，未写是算术还是几何平均；4.59/6.02/3.83 为几何平均。图内标注值复算（见第二节）：6.02、3.83、4.59 均复现 | 已找到 |
| A2 | 对照核是谁 | L1 | 同上 | “We benchmark one Quad+L3 cluster against a single core of a modern superscalar (dual-issue, in-order) CPU, which we refer to as TCPU (“Traditional CPU”). This TCPU is 0.895 mm² at the same process.†”；脚注：“† For legal reasons, we cannot disclose the exact CPU.” | `02-tcpu-quad-comparison.png` | 未点名型号；未给 TCPU 主频；“same process” 指 7 nm；Fig. 11 标题与图注称 “TCPU's best build”，正文未解释“best build”含义。博客称它是 “silicon-proven”（对照核有流片实绩）。TCPU 面积是否含缓存未写 | 已找到（型号：未披露） |
| A2 | 比的是“一个四核簇对一颗核” | L1 | 同上 | “For a fair comparison, we chose to first measure “single-core” performance by benchmarking one Frostbite quad against the TCPU, since they have similar area.” 面积：“Each Frostbite quad (a two-by-two grid of tiles) paired with four L3 nodes is 0.792 mm².” | `02-tcpu-quad-comparison.png` | 0.792 mm²（quad+4 个 L3 节点）对 0.895 mm²（单核，7 nm）。Frostbite quad = 2×2 = 4 个 tile。Fig. 11 图内标题写 “Whole-chip performance per watt against TCPU's best build”，图注写 “one Frostbite quad”，措辞不一致 | 已找到 |
| A3 | 4.59 / 6.02 / 3.83 / 8.77 | L1 | 同上 Fig. 11、Fig. 3 | Fig. 11 图注：“Performance-per-watt of one Frostbite quad relative to the TCPU's best build, across 25 workloads: the 10 in the search set, and the 15 in the held-out eval set. Dashed lines mark the geometric mean of the eval set, of all 25, and of the search set.”；图内图例 “eval set 3.83× / all 4.59× / search set 6.02×”，“TCPU = 1×” | `03-fig11-perf-per-watt.png`、`04-fig3-workload-split.png` | 25 项全部标注值见第二节；最低 fib 1.15×，最高 conv2d 23.12×（留出集）；没有任何一项低于 1×。8.77x perf/area 没有对应图表 | 已找到 |
| A3 | 搜索集 10 项与留出集 15 项的划分 | L1 | 同上 Fig. 3 | Fig. 3 图注：“All 25 workloads, grouped by how each program exposes its work, and split into the search set the agents tuned on and the held-out eval set, with the size each was run at.”；正文：“Later, we added five more practical workloads to the held-out set: a Kalman filter, PCA, finite element analysis, an LU solve, and LZ4 decompression.” | `04-fig3-workload-split.png` | 留出的 15 项里有 5 项（kalman、pca、fea、lu solve、lz4）是后加的；博客称 “including the five practical workloads added later”。博客未写这 5 项何时加入、加入时设计是否已冻结 | 已找到 |
| A4 | 流片前、只到 GDS | L1 | 同上 | “We synthesized Frostbite on a 7 nm process, reaching 800 MHz timing at synthesis and 500 MHz in the final GDS.”；“These are, of course, still pre-silicon power numbers, and over multiple tapeout loops, ASL models can be correlated to true post-silicon, production power numbers.”；“Arcmodel, however, is not physically aware, so its power and performance estimates at first diverge from the true numbers of the timing-converged, synthesis-clean RTL, in the worst case by almost 250%.”；“...until the RTL and Arcmodel agree to within 5%.” | `07-presilicon-alignment.png`、`02-tcpu-quad-comparison.png` | 全文未见硅片实测、流片、MPW 或 FPGA 实测；未写使用的工艺库/EDA 工具；功耗是流片前数字，经 Arcmodel 与 RTL 对齐到 5% 以内（对齐的是两套流片前估算）。TCPU 的功耗数字来源（实测还是同流程估算）博客未写 | 已找到（硅片实测：未找到） |
| A5 | AI 自主做了什么 | L1 | 同上 | “Our AI systems autonomously iterated over the compiler, ISA, architecture, microarchitecture, RTL, verification, and final GDS.”；“Frostbite was designed with zero architectural priors.”；“This is a fully autonomous pipeline.”（指 Arcmodel 对 RTL 的功耗重校准）；“This behavior was never specified by any human.”（指 tile 间偷任务规则）；“Throughout the entire development of Frostbite, our AI iterates on the Frostbite ISA, Arcmodel, Design, and Verification tightly together” | `05-ai-autonomy-claims.png`、`07-presilicon-alignment.png` | 均为公司自述；博客未给人工介入量化（几位工程师、多长时间、人工改了什么） | 已找到 |
| A5 | 人做了什么（博客里的“我们”） | L1 | 同上 | “To begin architectural exploration, we had to establish the set of workloads Frostbite should do exceedingly well in. We decided to look at different ways a program could expose its work...”；“Under each category, we created a search set and a held-out eval set”；“we designed a new in-house specification language, ASL, and wrote Frostbite's system model “Arcmodel” in it.”；“we chose to first measure “single-core” performance by benchmarking one Frostbite quad against the TCPU” | `06-human-choices-and-lean.png` | “we”指公司（人）还是公司的系统，原文未区分；负载集选择、搜索/留出划分、ASL 语言、对照口径均写成“我们”。Redwood 前作写明 “from a high-level specification written by two human architects”“with no human intervention below the specification”（https://architectlabs.com/blog/redwood，2026-08-27，`a-redwood.txt`）；Frostbite 博客没有对应的这类说明 | 部分支持（人工介入量：未找到） |
| A5 | Lean / 形式化说法与规模 | L1 | 同上 | “ASL also translates natively to Lean, which we use for all theorem proving when checking AI outputs across RTL, UVM, C, SVA, and more.”；“By the end, our Frostbite ASL lowered down to 800k+ lines of Lean.”；Fig. 4 图注：“The Lean contracts of all 31 blocks.” | `06-human-choices-and-lean.png` | “800k+ lines of Lean” 是 ASL 降级生成的行数，不是人写的证明量；博客未写哪些性质被证明、证明是否全部通过、覆盖哪些块的哪些性质；ASL 细节 “We will discuss ASL in more detail in a future post.” | 已找到（证明范围：未找到） |
| A5 | 开源承诺与仓库 | L1 | 同上 | 计划列表末项：“Release the RTL and ISA for Frostbite.”；“We plan to continue scaling development on Frostbite, including but not limited to:” | `08c-next-plans-spec-parsec.png` | 是“计划”，无日期、无仓库链接（全文唯一 GitHub 链接是 SPLASH-2 基准）；编译器（Vine/Ivy）未列入开源；Vine 语言官网 vine.dev 为第三方 | 已找到（仓库链接：未找到；只有计划） |
| A6 | 后续计划与局限 | L1 | 同上 | “In the coming months, we'll be benchmarking Frostbite mesh-like configurations against multi-core “TCPU” arrangements with large caches and larger, wider, out-of-order CPUs, over a larger set of industry benchmarks such as SPEC and PARSEC.”；“Scale swarms to bring Frostbite to higher performance-per-watt, and benchmark it on a much larger set of workloads against a broader set of CPUs.”；“Deliver a software playground for anybody to run a program on Frostbite, hosted on FPGAs.”；“Build a preliminary operating system capable of managing hardware resources, multiple programs, virtualization, and more.” | `08c-next-plans-spec-parsec.png` | 全部无日期；“coming months” 仅指对比测试；乱序核与多核对比尚未做。博客未列“局限”小节；局限只能从上面各句反推（只对比顺序核、25 项自选小负载、流片前） | 已找到 |
| A6 | 负载规模与缩放 | L1 | 同上 | “The kernels for FFT, radixsort, and LU follow the implementation in the SPLASH-2 benchmark, although 64 times smaller, because at regular sizes they are not suitable for exploratory work.” | `06-human-choices-and-lean.png` | 25 项负载的规模见 Fig. 3（如 fib n = 20、quicksort n = 4096、gemm 64×64×64）；“LU” 在 Fig. 3 中是 “lu solve”（后加留出项），Fig. 11 里没有单列的 SPLASH-2 LU | 已找到 |
| A6 | 能耗构成只有 20 项 | L1 | 同上 Fig. 12 | “Share of each chip's energy by function, excluding clock distribution, for the TCPU, one Frostbite quad, and the full four-by-four Frostbite. Mean over 20 workloads.”；图内：“The clock distribution is 49% of TCPU's energy, 41% of Frostbite quad's, 42% of Frostbite's.” | `09-fig12-energy-20-workloads.png` | 能耗构成取 20 项负载均值，与 4.59x 的 25 项不同；博客未说明缺哪 5 项。时钟树占 TCPU 49%、quad 41%，构成图已把它剔除 | 已找到 |
| A7 | 夸大标题实例（见第四节） | L5 | 见第四节 | 见第四节 | `10-aroged-crushed-headline.png`、`11-notebookcheck-move-over-headline.png` | 二手稿，只作“夸大实例”，不作事实来源 | 已找到 |
| A8 | HN 热度 | L6 | https://news.ycombinator.com/item?id=50572503 | 标题 “Frostbite: A new CPU architecture discovered, designed and verified by AI” | `12-hn-thread-title-points.png` | 2026-11-20 22:12 北京：294 分 / 200 评（`a-heat.json`）；22:17 渲染时 301 分 / 207 评；提交 2026-11-20 10:01 UTC（北京 18:01）。同一博客另有两次提交：`50557080`（北京 11-19 15:33，7 分 / 2 评，标题 “...E2E by AI”）、`50562422`（北京 11-20 00:40，2 分 / 0 评，标题 “...designed, and verified by AI (4x less power)”） | 已找到 |
| A8 | Reddit | — | — | — | — | `barking heat --reddit` 两次均被重定向到 old.reddit.com 登录页，未登录、未重试；Reddit 主帖：未取得 | 未找到一手来源 |

## 二、Fig. 11 全 25 项能效（图内标注值，相对 TCPU = 1×）

读法：图内每根柱末尾印有数值（非目视读数）；“集”为图例深蓝 = search set（调优），浅蓝 = eval set（留出）。取自 `sources/a-fig-perf-per-watt-heldout.png`，与 `images/03-fig11-perf-per-watt.png` 一致。

| 序 | 负载（规模） | 集 | 能效倍数 |
|---|---|---|---|
| 1 | conv2d（2×64×64 → 2, 3×3） | 留出 | 23.12× |
| 2 | mergesort（1024 words） | 搜索 | 13.75× |
| 3 | gemm（64×64×64） | 搜索 | 11.46× |
| 4 | biquad（2048 samples, 4 sections） | 搜索 | 7.84× |
| 5 | binomial（1024 steps） | 搜索 | 7.80× |
| 6 | json（4096 chars） | 搜索 | 7.59× |
| 7 | lcs（512×512） | 留出 | 6.35× |
| 8 | kdtree（1024 points, 64 queries, k = 4） | 留出 | 6.19× |
| 9 | kalman（state estimation） | 留出（后加） | 5.87× |
| 10 | nqueens（n = 10） | 搜索 | 5.66× |
| 11 | pca | 留出（后加） | 5.31× |
| 12 | huffman（bit-serial decode） | 留出 | 4.93× |
| 13 | chacha20（16384 B） | 搜索 | 4.82× |
| 14 | fea（finite element analysis） | 留出（后加） | 4.50× |
| 15 | radixsort（n = 16384） | 留出 | 3.96× |
| 16 | sha256（16384 B） | 搜索 | 3.92× |
| 17 | spmv（512×512, density 0.1） | 搜索 | 3.35× |
| 18 | randomforest（10 trees, 256 samples） | 留出 | 3.07× |
| 19 | lu solve（dense linear system） | 留出（后加） | 2.46× |
| 20 | lz4（block decompression） | 留出（后加） | 2.44× |
| 21 | fft（1024 points） | 搜索 | 2.39× |
| 22 | emeans †（n = 256, d = 20, k = 16, T = 1, B = 8） | 留出 | 2.32× |
| 23 | bfs（1024 nodes, degree 8） | 留出 | 2.20× |
| 24 | quicksort（n = 4096） | 留出 | 2.09× |
| 25 | fib（n = 20） | 留出 | 1.15× |

要点（图内数值，可直接核对）：
- 没有任何一项低于 1×；最低是 fib 1.15×（博客正文把 “recursive Fibonacci” 归入 “irregular, branching programs” 例子，Fig. 3 归入 Recursive 类）；其次 quicksort 2.09×、bfs 2.20×、emeans 2.32×。最高是留出集的 conv2d 23.12×。
- Fig. 11 的 `emeans †` 带 † 号，页面上的唯一脚注是 “† For legal reasons, we cannot disclose the exact CPU.”（挂在 TCPU 上）；图内 emeans 的 † 在博客页面上没有单独说明（图为位图，本页文字与 HTML alt 中均无解释）。**含义未知，不要推断**。
- 复算（推算，用图内标注值）：搜索集 10 项几何平均 6.02、留出集 15 项 3.83、全部 25 项 4.59，与博客一致。
- 推算：全 25 项中 13 项高于 4.59×，12 项低于；25 项中位数 4.82×；留出集中位数 3.96×（radixsort）；去掉 conv2d 后留出集几何平均约 3.37×、25 项约 4.29×。后加的 5 项（kalman、pca、fea、lu solve、lz4）几何平均约 3.85×，其余 10 项留出约 3.82×，二者几乎一致。以上都是本组计算，不是博客原文。

## 三、可能的吠点（线索摘要没提或说反的条件；写成“说法 vs 证据”）

1. 对照核：说法“AI 设计的 CPU 能效 4.59 倍”vs 证据——对照是一颗面积 0.895 mm²、型号“因法律原因不披露”的双发射顺序核（A2，博客脚注）；博客自己说乱序核与多核对比“in the coming months”才做（A6）。
2. 比的是四核簇对单核：说法“Frostbite 对 CPU 核”vs 证据——一个 quad（2×2 tile）+4 个 L3 节点共 0.792 mm²，对一个 0.895 mm² 的单核；博客称按“面积相近”比（A2）。博客同时给 perf/area 8.77×（只写 “average”）。推算：若 perf/area 8.77× 成立，则同面积基础上绝对性能约 8.77×0.792/0.895≈7.76×（HN 评论 `50573371` 也做了同样算术），对应功耗约为 TCPU 的 7.76/4.59≈1.7 倍（推算，忽略平均口径不同，仅作量级）。博客未给任何绝对性能或绝对功耗数字（无 MHz 下的绝对运行时间，无 mW）。
3. 调优题 vs 留出题：说法“4.59×”vs 证据——搜索集（调优用）10 项 6.02×，留出集 15 项 3.83×（A3）；留出集 15 项里有 5 项是后加的实用负载（kalman、pca、fea、lu solve、lz4）（A3）；负载集由公司自选，25 项都是小规模（FFT/radixsort/LU 缩小到 SPLASH-2 常规规模的 1/64）。留出集第一名 conv2d 23.12× 自己拉高均值（推算：去掉后约 3.37×）。
4. 流片前：说法“造出芯片”vs 证据——全文只到综合与最终 GDS，500 MHz 是 GDS 时序，功耗是 “still pre-silicon power numbers”，对齐的是 Arcmodel 与 RTL 两套流片前估算（A4）。没有硅片、没有 FPGA 实测（博客把 FPGA 游乐场列为计划）。
5. “AI 自主”：说法“AI 自己造出 CPU”vs 证据——博客称 “autonomously iterated”“zero architectural priors”，同时负载集、搜索/留出划分、ASL 规格语言、对照口径均由“我们”决定（A5）；人工介入量未披露；Redwood 前作写明人类架构师写规格、公司人员“drive the design end to end through tapeout”（https://architectlabs.com/blog/redwood，`a-redwood.txt`）。
6. Lean/形式化：“800k+ lines of Lean” 是规格语言 ASL 降级生成的行数；证明了什么、是否全通过未写（A5）。
7. 最弱的项：留出集 fib 仅 1.15×；博客正文说 “it also outperforms the TCPU on branching, dynamic code”，与 Fig. 11 一致（所有项 >1×），但 fib、quicksort、bfs 等分支/递归类远低于几何平均。
8. 能耗构成图是 20 项均值，4.59× 是 25 项（A6）；Fig. 11 图内标题 “Whole-chip” 与图注/正文 “one Frostbite quad” 口径措辞不一致（A2）。
9. 开源：“Release the RTL and ISA for Frostbite” 是计划，无日期无仓库（A5）；因此 4.59× 无法被外部复现。

### 社区质疑（仅线索，L6；HN 不公开评论分数，Algolia 评论 `points` 为 null，故无法给得分）

HN 主帖 https://news.ycombinator.com/item?id=50572503 的评论（存档 `a-hn-50572503.json`；22:12 北京共 194 节点）。链接格式 `https://news.ycombinator.com/item?id=<id>`：
- `50572930`：“their baseline is a in-order CPU, instead of a modern out-of-order design, and they're testing a small amount of programs.”；并称思路与 WaveScalar 的数据流架构相近（评论者自述，未核）。
- `50573371`：按博客数字算 8.77× perf/area 且面积 88%，则绝对性能约 7.76×；“won't say which single-core processor ‘for legal reasons’”（算术可核，见第三节第 2 点）。
- `50573236`（反方）：“They're claiming better performance per watt, not better absolute performance, and out-of-order does worse on that metric than the superscalar in-order design”——说明用顺序核作对照有其 perf/W 逻辑。
- `50574465`：称慢核的 perf/W 本就好；推断对照核面积与 Cortex-A720（较密工艺）相当；顺序核“对基准选择很敏感”，FFT、LU 等规则循环负载占优，编译器类分支多的负载未测（评论者推断，未核）。
- `50574502`：“I don't see any indication that they got their chips fabricated”；设计者自选 ISA 时易在某些指标上取胜。
- `50573533`：对照核属“microcontroller core class”，合成基准相关性低（评论者主观判断）。
另：楼主在 `50574533` 评论称评论区偏离主题，应关注交互网（interaction nets）本身。

## 四、夸大说法实例（把事件写歪的标题/文章；二手稿，L5）

| # | 站点 URL | 原句（逐字） | 发布时间 | 发布方 | 截图 | 备注 |
|---|---|---|---|---|---|---|
| 1 | https://www.aroged.com/ai-created-a-completely-new-processor-architecture-without-crutches-it-crushed-conventional-cpus-in-virtual-tests/ | 标题：“AI created a completely new processor architecture without “crutches” — it crushed conventional CPUs in virtual tests”；正文：“In benchmarks, the development completely crushed the best processor of our time, the name of which remained secret for legal reasons.” | 页面标 November 19, 2026（Exa 元数据 2026-11-19 17:49 UTC，北京 11-20 01:49，未核）；页面未给时刻 | Aroged | `10-aroged-crushed-headline.png` | 把“对一颗未点名的双发射顺序核”写成“碾压 conventional CPUs”“best processor of our time”（对照核博客称 “modern, silicon-proven superscalar (dual-issue, in-order)”，并非当今最强）。存档 `a-m-aroged.*` |
| 2 | https://www.notebookcheck.net/AMD-and-Intel-move-over-AI-designed-an-all-new-CPU-and-claims-it-is-4-6x-more-efficient.1455225.0.html | 标题：“AMD and Intel, move over: AI designed an all-new CPU — and claims it is 4.6x more efficient”（正文第一段含 “However, the results come from pre-silicon simulations rather than a physical processor.”） | “Published 11/20/2026”（页面）；Exa 元数据 2026-11-20 12:08 UTC（北京 20:08，未核） | Notebookcheck（作者署名 Andrew Sozinov） | `11-notebookcheck-move-over-headline.png` | 标题点名 AMD、Intel，博客对照是未点名的顺序核、从未与 AMD/Intel 比较；正文有流片前限定，但标题没有。存档 `a-m-notebookcheck.*` |
| 3 | https://news.ycombinator.com/item?id=50562422 | HN 标题：“Frostbite: A new CPU architecture designed, and verified by AI (4x less power)” | 北京 2026-11-20 00:40 | HN 用户提交（2 分 / 0 评） | 无（存档 `a-hn-50562422.*`） | “4x less power” 不是博客说法：博客是 perf/W 4.59×，不是功耗降到 1/4；推算按博客数字功耗约为 TCPU 的 1.7 倍（见第三节第 2 点）。仅作措辞偏差实例，影响小 |
| 4（偏正面但无害） | https://www.pic720.com/article/6b0022218e9f091ec34c6f0a | 标题：“AI自主设计全新CPU Frostbite，能效比提升4.59倍” | 页面标 2026-11-20（Exa 元数据 10:01 UTC，未核） | 知澜资讯网 | 无 | 基本照译博客，标题未写对照核与流片前；非明显夸大，供中文侧参照。存档 `a-m-pic720.*` |

没有找到的：中文大陆主流媒体（IT之家、36氪、机器之心等）对 Frostbite 的报道，Exa 限定这些域名检索无结果（只返回博客与二手站），故不列。

谨慎的二手稿（可作参照，不是夸大）：explainx（https://explainx.ai/blog/architect-labs-frostbite-ai-designed-cpu-interaction-nets-4-59x-perf-per-watt-claimed-vs-verified-2026，Nov 19）、ic.work 中文（`a-m-icwork.*`，2026-11-20）、xenospectrum 英文版（403，未存档，仅有 Exa 摘录，含 15 项留出表与“中位数 3.96×”）。

## 五、扫描说法勘误

- ChatGPT 扫描“对照核名称未披露”：**属实**（脚注 “For legal reasons, we cannot disclose the exact CPU.”）。
- “RTL/ISA 承诺未来开源、尚未提供”：**属实**，仅为 “plan to…Release the RTL and ISA for Frostbite.” 一项计划，无日期、无仓库链接；编译器未写。
- “800k 行 Lean 合约”：**部分支持**——原文 “our Frostbite ASL lowered down to 800k+ lines of Lean”，是 ASL 降级产物；Fig. 4 为 “Lean contracts of all 31 blocks”；证明范围未写。
- “公司计划日后做 SPEC/PARSEC 与多核比较”：**属实**（“In the coming months...SPEC and PARSEC”，同时点名“larger, wider, out-of-order CPUs”）。
- 派工单已知情况核对：博客日期 November 18, 2026 无时刻——一致；4.59/6.02/3.83/8.77、0.792、0.895、800/500 MHz、7 nm——均与原文一致；“HN 206 分/136 评”是 Claude 21:00 前后的读数，本组 22:12 读 294/200、22:17 读 301/207，ChatGPT 的 65 分是更早时刻，均属热度随时间上升，不冲突。
- 派工单写“对负载含 SPLASH-2 的 FFT、radixsort、LU”：**部分**——正文称 SPLASH-2 的 FFT、radixsort、LU 内核缩小到 1/64；Fig. 3/11 中对应项为 fft、radixsort，LU 在图中只有后加的 “lu solve”。
- 派工单的“比的是四核簇”：**属实但需限定**——quad = 2×2 tile（4 个 tile）加 4 个 L3 节点；博客并未称 tile 为“核”，博客自己叫 “single-core” 对比。
- 派工单“25 项自选负载”：**属实**（“25 initially chosen workloads”）。
- “流片前”：**属实**；“只到 7nm 最终版图（GDS）”：**属实**（“final GDS”，500 MHz）；是否有 DRC/LVS 或 sign-off：博客未写。
- 派工单把 6.02 称“调优用的 10 项”、3.83 称“从没调过的 15 项”：**与原文一致**；但补充：15 项中 5 项为后加。

## 六、对读者（可买/可用/可复现）

- 有没有可买或可用的芯片：无。博客称成果是 7 nm 综合与最终 GDS，未写流片、未宣布制造或商用（A4）；Notebookcheck 也写 “has not announced a manufactured Frostbite chip or commercial availability”（二手，L5）。
- 何时流片/开源：官方只写计划，无日期：“Release the RTL and ISA for Frostbite.”（A5）；“Deliver a software playground for anybody to run a program on Frostbite, hosted on FPGAs.”（A6）。已知情况里的 “multiple tapeout loops” 是对未来关联真实硅片功耗的泛指，不是流片排期（A4）。
- 谁不受影响：所有人——没有可买产品；手机、电脑里的 CPU 不会因此改变。
- 谁可以验证：目前无人（RTL、ISA、编译器、测试脚本均未公开）。博客未给仓库。
- 博客“Partner with us”只面向 AI 实验室、芯片公司、OEM、国防承包商与 ASIC 设计合作方（页尾文字，`a-blog.txt`），非普通用户。

## 七、未完成与未核

- 未核：Architect Labs 官方 X/LinkedIn 帖是否另有说明；X 与 LinkedIn 帖未抓取。
- 未取得 Reddit 主帖（登录重定向）。
- 未存档 xenospectrum（403）；未存档 Tendrils 文章、Vine 官网。
- 未打开 Redwood 白皮书 PDF（只取博客页）。
- HN 评论“得分”不可得，仅给评论链接与摘句。
