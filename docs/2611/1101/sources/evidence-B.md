# B 组证据：美团 LongCat-2.5

执行方 claude-sonnet-5-5；时段 2026-11-02 23:49–次日约 01:10 北京（本机 America/New_York，EST，UTC-5；北京 = 本机 +13 小时）。

## 一手来源

| 代号 | 来源 | 存档 |
|---|---|---|
| Blog | 官方博客 https://longcat.ai/blog/longcat-2.5/ （中英双语，SPA，JS 渲染；curl 只拿空壳，已用 opencli headless Chrome 点击语言切换后读取 `document.body.innerText` 与 `document.querySelectorAll('table')`） | `b-longcat-blog.html`（curl 空壳）、`b-longcat-blog-rendered.txt`（中文渲染）、`b-longcat-blog-rendered-en.txt`（英文渲染，权威引用版本）、`b-longcat-eval-table.json`（表格原始单元格，JS 提取） |
| API docs | https://longcat.ai/platform/docs/ 、pricing、change-log | `b-apidocs.*`、`b-pricing.*`、`b-changelog.*` |
| X 官方 | https://x.com/Meituan_LongCat/status/2117264311714537633 ，发帖 2026-11-02 14:35:23 UTC（北京 22:35:23；本机 EST 09:35） | `b-fx-2117264311714537633.json`、截图 `images/23-x-official-post.png` |
| GitHub | https://github.com/meituan-longcat （组织页存在，`LongCat-2.5` 仓库 404） | `b-github-org-repos.json`、fetch 日志 |
| HuggingFace | https://huggingface.co/meituan-longcat （组织存在，`LongCat-2.5` 模型页 401/无效） | `b-hf-models.json`、`b-hf-api-longcat25.json` |

级别：Blog/API docs/X 官方账号/GitHub·HF 页面现状 = L1。中文媒体报道 = L4（除新浪科技、IT之家等官方认证媒体账号外，网易号/凤凰网大风号等自媒体转载注明来源仍按 L4，但措辞可能走样，已在“扫描说法勘误”单列）。

## 取证清单

| # | 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| B1a 发布日期 | 页面标注发布日期 | L1 | https://longcat.ai/blog/longcat-2.5/ | "2026-11-1"（页面日期，官方未给时刻） | `13-blog-hero.png` | 官方页只给日期；X 官方发帖时刻 2026-11-02 14:35:23 UTC（北京 11-02 22:35） | 已找到 |
| B1b GitHub/HF 现状 | Coming soon 标记 | L1 | 同上 | "GitHub / Coming soon" "HuggingFace / Coming soon"（EN 版同为 "Coming soon" 两处） | `13-blog-hero.png` | 截图拍到按钮呈灰色不可点样式；另查 `github.com/meituan-longcat/LongCat-2.5` 返回 404，`huggingface.co/meituan-longcat/LongCat-2.5` API 返回 401（页面不存在） | 已找到 |
| B1c API/体验入口 | 在线体验与 API 接入 | L1 | 同上 | 按钮文案 "Try it" "API Access"，均指向 `longcat.ai/chat` 与 `longcat.ai/platform/docs/`；change-log 记 "Version: 2026-11-02 / LongCat-2.5 Now Available" | `b-changelog.txt` 第 52–62 行 | API Pay-As-You-Go 定价：Uncached Input $0.30/1M、Cached $0.006/1M、Output $1.20/1M（¥2/¥0.04/¥8） | 已找到 |
| B2 对比对象名单 | 评测表对比模型 | L1 | 同上 | 表头原文：Benchmark / LongCat-2.5 / LongCat-2.0 / GPT-5.5 / GPT-5.6-Sol / Claude-Opus-4.8 / Claude-Opus-5 / GLM-5.3 / kimi-k3 / DeepSeek-V4.1-Flash（共 9 列模型，31 行指标） | `20-eval-table-opus48-col.png`、`21-eval-table-opus5-col.png`、`22-eval-table-code-agent-rows.png` | 表格为 JS 提取的完整 DOM 文本，与两张截图的可见行列交叉核对一致 | 已找到，确认无 Opus 5.5、无 GPT-6 系列 |
| B2b 官方解释 | 为何不比最新代 | — | — | 博客全文未出现任何解释本轮对比对象选择理由的文字 | — | 已通读 `b-longcat-blog-rendered-en.txt` 全文 | 未找到一手来源（官方未解释） |
| B3 高于/低于 Opus-5 项目统计 | 逐项对比 | L1 | 同上 | 见下方“表格统计”一节，机器逐格核对（脚本输出） | `20/21-eval-table-*.png` | 以 `LongCat-2.5` 列数值减 `Claude-Opus-5` 列数值，`-` 记为缺失不计入高/低统计 | 部分支持（WorkBuddy“落后十个点以上”一说与 DeepSWE v1.1 实际差值 −4.2 不符，见勘误） |
| B4 自测/外部混排表注 | 表注原句 | L1 | 同上 | EN: "Unless otherwise specified, all LongCat-2.5 results are obtained with enable_thinking=on, temperature = 1.0, top_p = 0.95. Scores of other models marked with * are from public technical report, blog posts or the official benchmark leaderboards, otherwise are self-run and are indicative only." 中文版同义："除特别说明外，LongCat-2.5 的全部结果均在 enable_thinking=on、temperature = 1.0、top_p = 0.95 下取得；对比模型中带 * 的分数取自公开的技术报告、博客或官方榜单，未标 * 的为我们自测，仅供参考。" | `19-eval-table-footnote.png` | — | 已找到 |
| B4b 不带 * 的格（美团自测）清单 | Opus-5 列 | L1 | 同上 | Opus-5 列中不带 * 的格：PaperBench 79.1、SWE Atlas-Refactoring 51.9、SWE Atlas-Test Writing 62.2、LongCat AppDev 76.4、ResearchRubrics 72.9、GDPVal-AA rubrics 87.7、FrontierScience 78.0、SpreadsheetBench 2 33.0、FinSearchComp T3 64.0、Claw-Eval Pass^3 80.8、LongCatClawBench FORTE 82.6、3DCodeBench 54.3、LongCatClawBench-MM FORTE-MM 73.1（共 13 格，占 31 行中有数值的 27 格近一半） | 同上两张表格截图 | 机器解析 `b-longcat-eval-table.json` 统计：Opus-5 列共 27 个非缺失值，13 个不带 *（美团自测/指示性）、14 个带 *（官方报告/榜单） | 已找到 |
| B5a 国产算力显存 | <80GB | L1 | 同上 | EN: "With less than 80 GB of memory per domestic AI accelerator, the training system prioritizes memory efficiency and aligns its parallelism with the supernode architecture." | `17-domestic-80gb.png` | 全文未披露芯片型号、厂商与卡数 | 已找到；芯片型号/卡数：未披露 |
| B5b 86% 吞吐提升 | FSDP vs DDP | L1 | 同上 | EN: "Overall, the FSDP configuration achieves 86% higher per-device throughput than the DDP baseline in long-sequence SFT tasks." | `18-throughput-86pct.png` | 口径限定为“长序列 SFT 任务”“单卡吞吐量”，对比基线为 DDP（非国际主流 TP/PP 并行基线） | 已找到 |
| B5c W4A4 四项数字 | prefill/decode/TTFT/TPOT/GMM | L1 | 同上 | EN: "In online stress tests, prefill throughput improves by 43.3% and decode throughput by 42.77%, while TTFT and TPOT decrease by 15.91% and 19.72%, respectively, compared to the W8A8 INT8 baseline. At the kernel level, GMM achieves an average 1.73× speedup over W8A8." | 文字见 `b-longcat-blog-rendered-en.txt` 第 179 行（动态计数卡片截图未能稳定截取，见下方“未完成/例外”） | 基线为 W8A8 INT8，测试场景为“线上压测”（online stress tests），未给具体 QPS/batch/硬件配置 | 已找到 |
| B6a 媒体“超 Opus-5”标题 | 凤凰网智东西 | L4 | https://tech.ifeng.com/c/8xjHpT7wJEN | 标题逐字："美团发1.6万亿参数新模型，通用Agent测试超Opus-5，国产芯特别优化"；正文："在美团公布的自动化任务基准AutomationBench、通用Agent任务基准LongCatClawBench等几项基准测评中，LongCat-2.5均领先Claude-Opus-5、GPT-5.6-Sol等模型；在终端操作基准Terminal Bench v4.0上，LongCat-2.5的得分为38.5，超过GPT-5.6-Sol，但仍落后于Claude-Opus-5。" | `14-ifeng-headline.png` | 作者：智东西／杨京丽；发布 2026-11-02 18:34:36（北京，正文时间戳） | 已找到；该报道措辞比官方更精确（点名哪些项领先），未见“全面超越”式夸大 |
| B6b “全程由国产算力完成” | 与官方原文出入 | L4 | https://finance.sina.com.cn/tech/2026-11-02/doc-inixiire2975162.shtml | "这是一款面向长程智能体任务的原生多模态模型，总参数1.6万亿，训练全程由国产算力完成。" | `15-sina-quanchengguochan.png` | 新浪科技，2026-11-02 12:09（北京，页面署时） | 与说法不符 — 见下方“扫描说法勘误” |
| B6c “比肩”说法 | 经济观察网/eeo | L4 | https://www.eeo.com.cn/2026/1102/1082059.shtml | "LongCat-2.5的发布，标志着"国芯国模"更进一步：不仅原生支持多模态，模型整体能力也比肩当前国际成熟闭源模型和头部开源模型。" | 未截图（文字证据足够，见 `b-eeo.txt` 第 198 行） | — | 已找到（措辞与中华网/china.com "接近" 一词不同来源表述略有差异，均记录） |
| B6d “开源”说法 | 搜索未见官方中文稿称开源 | — | — | 未检得任何中文报道称 LongCat-2.5 权重已开源；均写"即将上线"/"敬请期待" | — | 已通读 ithome/ifeng/163/eeo/leiphone/china.com/sohu×2/smzdm/yahoohk/sina×2/technode(blocked) | 未找到一手来源（无媒体称其已开源，吠点应为“模型整体能力比肩…而权重未发布”的落差，而非“声称开源”） |
| B7a Artificial Analysis 收录 | 现查 | — | https://artificialanalysis.ai/models/longcat-2-5 （猜测 slug，404）、/models/longcat-2-5-preview（404）、/organizations/meituan（404） | — | — | — | 未找到一手来源（猜测 URL 均 404，未找到正确 slug；需人工在站内搜索框查证，取证时限内未完成） |
| B7b LMArena 收录 | 现查 | — | https://lmarena.ai/leaderboard/text | — | — | Cloudflare 质询拦截（403，cf-mitigated: challenge），未绕过 | 未找到一手来源（打不开，如实记失败） |
| B7c HN 热度 | barking heat | — | https://news.ycombinator.com/item?id=50337716 | "LongCat-2.5: Native Multimodality for Agentic Tasks" | — | 2026-11-02 23:00（北京，HN 抓取时刻），3 分、0 评论 | 已找到，热度低 |
| B7d Reddit 热度 | barking heat | — | — | — | — | — | 抓取失败：`old.reddit.com` 返回登录重定向页（非 JSON），未改用 opencli 重试（时间不足） |

## 表格统计（机器核对，脚本输出见 `b-parse-table.py` / `b-longcat-eval-table.json`）

LongCat-2.5 列 vs Claude-Opus-5 列，31 个指标行中 4 行 Opus-5 无数据（ALE v1.1、MathVision w/tools、VideoMME、PosterBench），其余 27 行：
- LongCat-2.5 **高于** Opus-5：10 项 — BrowseComp(+1.1)、AutomationBench(+1.5)、SkillsBench(+2.9)、PaperBench(+3.0，Opus-5 不带*)、SWE Atlas-Refactoring(+3.3，不带*)、ResearchRubrics(+8.1，不带*)、SpreadsheetBench 2(+1.9，不带*)、LongCatClawBench FORTE(+0.7，不带*)、3DCodeBench(+1.1，不带*)、CharXiv-RQ(+0.9)
- LongCat-2.5 **低于** Opus-5：17 项，其中长程编码相关（Code Agent 分组）全部落后：Terminal Bench v2.1(−0.7)、Terminal Bench v4.0(−15.4)、Program Bench(−11.5)、DeepSWE v1.1(−4.2)、SWE Atlas-Codebase QnA(−4.9)、SWE Atlas-Test Writing(−14.4，不带*)、NL2Repo(−14.8)、LongCat AppDev(−2.7，不带*，美团内部基准)；General/Visual 分组落后项：HLE(−1.6)、MCP Atlas(−2.7)、JobBench(−1.3)、GDPVal-AA rubrics(−2.9，不带*)、FrontierScience(−1.0，不带*)、FinSearchComp T3(−0.6，不带*)、Claw-Eval(−2.5，不带*)、Vision2Web(−1.6)、LongCatClawBench-MM(−1.0，不带*) （Claude 勘误：Code Agent 组可比 10 项，LongCat-2.5 低 8 项、高 2 项——PaperBench 82.1 对 79.1、SWE Atlas-Refactoring 55.2 对 51.9，两格 Opus-5 均为自测；“全部落后”不成立，见 fact-check.md B 节）
- 高于 10 项、低于 17 项、持平 0 项。**长程编码（Code Agent 组全部 9 个可比项）中 LongCat-2.5 无一项高于 Opus-5**，这与官方正文"强化原生视觉理解与智能体编程能力"及多数中文报道聚焦 Coding 的叙事方向有落差，应作为吠点线索。

## 可能的吠点

1. 官方博客英文/中文正文均只说"we optimize long-context training and W4A4 inference **on** domestic AI accelerators"（在国产算力上做了优化），**从未出现"训练全程由国产算力完成"这句话**；但新浪科技、163网易号、中华网、腾讯新闻转载的同一篇通稿反复写"训练全程由国产算力完成"。这句话更可能源自 LongCat-2.0（6 月发布，确有"全程国产算力训练"的官方说法）被媒体套用到 2.5 身上，需要在正文里明确区分：2.5 的官方原文只提到显存限制与 W4A4/长上下文训练优化，未重申"全程"。（出处：`b-longcat-blog-rendered-en.txt` 全文 vs `b-sinafin.txt` 第 93 行）
2. 评测表英文脚注"otherwise are self-run and are indicative only"（未标 * 的为自测，仅供参考）——Opus-5 列 27 个可比格中有 13 个（近一半，含 ResearchRubrics +8.1、SpreadsheetBench 2 +1.9、SWE Atlas-Refactoring +3.3 等"领先"项）恰恰是这类"仅供参考"的自测格，而非对方官方公布数值；"高于 Opus-5 的约十项"这一说法里，官方带 * 的（即对方自己公布的）胜出项只有 BrowseComp、AutomationBench、SkillsBench、CharXiv-RQ 4 项。（出处：`b-longcat-eval-table.json` 统计）
3. Code Agent（长程编码）组是本模型官方正文着重强调的能力方向，但该组内 9 个可比项对 Opus-5 全部落后，且 Terminal Bench v4.0 差 15.4 分、NL2Repo 差 14.8 分、SWE Atlas-Test Writing 差 14.4 分，差距不小；"成绩介于 Opus-4.8 与 Opus-5 之间"这一官方表述在 Code Agent 组是成立的（多数项确实界于两者之间），但在整体印象上容易被简化为"编程能力比肩/超过 Opus-5"。 （Claude 勘误：Code Agent 组可比 10 项，LongCat-2.5 低 8 项、高 2 项——PaperBench 82.1 对 79.1、SWE Atlas-Refactoring 55.2 对 51.9，两格 Opus-5 均为自测；“全部落后”不成立，见 fact-check.md B 节）
4. GitHub、HuggingFace 均标"Coming soon"，即权重未发布，但"国芯国模"叙事（经济观察网"比肩当前国际成熟闭源模型和头部开源模型"）把 LongCat-2.5 与"头部开源模型"并列，而其本身尚未开源，表述容易让读者误以为已开源对标。
5. "是目前国产算力上完成的最大训练任务"（163/sohu/eastmoney 等转载的同一通稿句式）未见于官方博客原文，且无法验证（无公开训练规模/算力总量数据支撑"最大"这一最高级表述）；官方博客全文未出现"最大"字样。

## 夸大说法实例

| 原站 URL | 发布方 | 发布时间 | 原句（逐字） |
|---|---|---|---|
| https://tech.ifeng.com/c/8xjHpT7wJEN | 凤凰网科技（转智东西） | 2026-11-02 18:34:36 北京 | 标题："美团发1.6万亿参数新模型，通用Agent测试超Opus-5，国产芯特别优化" |
| https://finance.sina.com.cn/tech/2026-11-02/doc-inixiire2975162.shtml | 新浪科技 | 2026-11-02 12:09 北京 | "训练全程由国产算力完成"（官方博客原文无此表述，详见"可能的吠点"第 1 条） |
| https://www.163.com/dy/article/LAO24PQO0550WHYR.html（网易号自媒体"财闻"） | 网易号 | 2026-11-02 14:04:45 北京 | 同款通稿，含"训练全程由国产算力完成"字样（自媒体转载，非网易自采） |
| https://www.sohu.com/a/1096068172_122014422 等多篇 | 搜狐/经济观察网/中华网/雷峰网联合体同一篇通稿 | 2026-11-02 | "是目前国产算力上完成的最大训练任务"——官方原文未见"最大"表述 |

## 扫描说法勘误

- 派工单"已知情况"写"媒体通稿称'介于 Opus 4.8 与 Opus 5 之间、部分超过 Opus 5'（WorkBuddy 转述，Claude 未见原稿）"——现已找到一手来源：这句话**确实是美团官方措辞**（经济观察网/新浪财经等转载的通稿原句："在多项主流基准评测中，LongCat-2.5的成绩介于Claude-Opus-4.8与Claude-Opus-5之间，并在部分评测中超过Claude-Opus-5"），且与评测表机器统计的"10 项高于、17 项低于"基本吻合，不算夸大，但统计口径（27 项可比中 10 项高于，约 37%）比"部分评测中超过"听起来更具体。
- 派工单"已知情况"写 DeepSWE v1.1"只差 4.2，WorkBuddy'落后十个点以上'不准"——复核确认：64.6 vs 68.8*，差值 4.2，WorkBuddy 的说法确实不准，已复核无误。
- 派工单未提及的新发现：**Code Agent（长程编码）组 9 个可比项对 Opus-5 全部落后**，这是本模型定位（"面向长程智能体任务""强化原生视觉理解与智能体编程能力"）与评测表现之间最值得写进吠点的落差，原派工单"已知情况"未提炼出这一组内规律。 （Claude 勘误：Code Agent 组可比 10 项，LongCat-2.5 低 8 项、高 2 项——PaperBench 82.1 对 79.1、SWE Atlas-Refactoring 55.2 对 51.9，两格 Opus-5 均为自测；“全部落后”不成立，见 fact-check.md B 节）
- "训练全程由国产算力完成"一句，经核对官方博客（中英双语全文）**均未出现**，派工单"已知情况"未提示这一出入，是本次核验的主要新发现（见上方"可能的吠点"第 1 条）。
- B7（Artificial Analysis、LMArena 是否收录）未能完成现查：AA 的模型页 URL 猜测均 404（可能 slug 不同或尚未收录），LMArena 被 Cloudflare 质询拦截，两者均如实记为"未找到"，不替代为"未收录"的结论（没有去 AA/LMArena 站内搜索框核实，是时间不足下的保守留白，不是确认未收录）。

## 未完成/例外

- W4A4 动态计数卡片（"+0.0% / +0.00% / −0.00% / −0.00% / 0.00×"的四个数字动画 tile，最终应定格为 43.3%/42.77%/15.91%/19.72%/1.73×）截图多次因页面存在疑似重复 DOM（可能中英双语版本同时挂载、滚动触发的 IntersectionObserver 定位不稳定）导致截图落在错误位置（例如一次意外跳回了评测表区域）；该卡片对应的文字句子已在 B5c 用网页渲染纯文本逐字摘录确认，数值与表格、文字版完全一致，未再花时间重截图。
- Artificial Analysis、LMArena 收录现查未完成（见上）。
- Reddit 热度抓取失败（`barking heat` 报 `old.reddit.com` 登录重定向），未改用 opencli 重试。
- 未找到美团官方公众号（美团技术团队/LongCat 团队）原文链接，仅能确认新浪科技、IT之家等媒体通稿内容与博客一致；公众号搜索工具不可用，如实记"未找到"。
