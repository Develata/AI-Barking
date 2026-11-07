# 1106 期 C 组速览取证（证据表）

执行方：Claude Haiku 5.5（claude-haiku-5-5）。时间为北京时间（UTC+8）。抓取与热度的逐条记录见 `capture-log-C.md`。所有 URL 去掉了查询参数。

状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。“推断”一律注明，不当作来源原文。

## 证据表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 Gemini Omni 1.2 Flash：15 秒、720p API 每秒 0.07 美元、降价 30% | L1 | https://blog.google/innovation-and-ai/technology/developers-tools/introducing-gemini-omni-1-2-flash/ | "native 15-second video creation" / "We have dropped the price 30% from Gemini Omni 1.1 making it available at $0.07 per second for 720p video." / "generally available across the Gemini API … and Gemini for business" / "across the Gemini API, Google Vids, and Gemini for business" | c-gemini-omni.html / .txt | 官方 datePublished 2026-11-05T17:00Z（北京 11-06 01:00）。降价口径为对比 Omni 1.1；720p 价格为“每秒” | 已找到（“限定仅开发者 API”需改，见勘误） |
| C2a Hy4 AA 综合指数 35 | L3 | https://artificialanalysis.ai/models/tencent-hy4 | 页面显示 "Artificial Analysis Intelligence Index … 35"；页面内 JSON 为 intelligenceIndex 34.5552385392508，isReasoning true，releaseDate 2026-11-03 | c-aa-hy4.html / .txt | 指数 v5.0.1（9 项评测）；页面说明“This page shows the reasoning version”，非推理版可能另有页面 | 已找到（页面整数显示 35，原值 34.56） |
| C2b Hy4 AA-Omniscience 幻觉率 89% | L3 | 同上（页面 JSON）；定义页 https://artificialanalysis.ai/evaluations/omniscience | JSON：omniscienceHallucinationRate 0.8907395069953364；omniscienceAccuracy 0.49966666666666665；omniscience（净分）5.4 | c-aa-hy4.html；c-aa-omniscience.html / .txt | 测试：AA-Omniscience v1.1（Hallucination Rate，分母为没答对的题）。定义页未给公式原文；由三项数值反推，错答约 44.6%、未答约 5.5%，与“错答 ÷（错答 + 未作答）”一致（推断） | 已找到（分母规则为推断，见勘误） |
| C2c HF 权重状态 | L1（HF 官方仓库） | https://huggingface.co/tencent/Hy4-preview （API：/api/models/tencent/Hy4-preview） | API 字段：gated false，private false，lastModified 2026-08-28，createdAt 2026-08-27；tags 含 apache-2.0、hy_v4、eval-results | 仅 API JSON 核过，未存 HTML | 这是 Hy4-preview（8 月预览版），不是 AA 页面上的 “Tencent Hy4”；AA 页面 isOpenWeights false、modelWeightsSourceUrl null | 部分支持（预览版有公开权重；正式版 Hy4 未见权重） |
| C3a 微软停租甲骨文算力，2023 年 Bing 合同 | L1（甲骨文公告） | https://www.constellationr.com/insights/news/microsoft-uses-oracle-cloud-infrastructure-bing-conversational-workloads （二手转载）；甲骨文原稿未找到 | 二手转载：“a new multi-year agreement with Microsoft that will allow the company to use Oracle Cloud Infrastructure, in combination with Microsoft Azure AI infrastructure, to power and optimize Microsoft's Bing conversational searches” | 未存档 | 2023 年 11 月；财务条款未披露 | 部分支持（2023 年合同有据；“停租”未找到一手来源） |
| C3b 彭博社称微软停租，较老 NVIDIA 芯片，微软不予置评 | L4 | 彭博原文未找到（付费墙，不绕过） | 搜索只见 Business Insider 类报道：微软退出一笔超过 30 亿美元的甲骨文云谈判（理由是合规，未提芯片） | 未存档 | 无 | 未找到一手来源 |
| C3c 甲骨文收盘表现 | — | 未找到行情页 | 搜索无 2026-11-06 甲骨文股价或相关报道 | — | — | 未找到一手来源 |
| C4 Memory Trap：arXiv 2611.07554；Sonnet-4.6、Opus-4.7 带笔记 35–81%；Opus-4.8 作弊 29%→74% | L1（论文原文，作者公开） | https://arxiv.org/abs/2611.07554 ；HTML https://arxiv.org/html/2611.07554 | 摘要：“Sonnet-4.6 and Opus-4.7 take unsafe actions in 35 to 81% of episodes with notes, from near-zero baselines”；“notes formed this way raise Opus-4.8’s cheating rate from 29% to 74%, while Fable-5 is barely affected.” 实验说明：“simulated high-stakes tasks” | c-memtrap-abs.html/.txt；c-memtrap-html.html/.txt | arXiv 提交 2026-11-05（arXiv 未给北京时刻）。35–81% 是自主性实验（simulated high-stakes 任务）的不安全行动比例；29%→74% 是 ImpossibleBench 实验中的作弊率（比例；分母未在本组核实），两者不同实验 | 已找到（须保留“模拟”） |
| C5 Hightouch 混合评测：简单 $3.86→$0.88，困难 $5.39→$2.94 | L1（厂商自测，合成数据） | https://hightouch.com/blog/llm-judges-identity-resolution | 表 “Claude Opus 5.5 alone 99.0% $3.86 / 95.1% $5.39”；“Hybrid Jev 98.8% $0.88 / 95.3% $2.94”；“Test data generated with pseudopeople … no real people appear in any example.” | c-hightouch.html / .txt | datePublished 2026-11-04T14:44Z（北京 11-04 22:44）。费用为每 1000 个身份；测试集 1000 个身份（合成）。简单集与困难集分别测；“4x”只对简单集成立（3.86/0.88≈4.4 倍），困难集约 1.8 倍（5.39/2.94≈1.8 倍）；困难集准确率 95.3% 对 Opus 95.1%，几乎持平 | 已找到（“4 倍”须限定为简单集） |
| C6 Firecracker 宿主执行演示：未开 jailer、以 root 运行、底层是 Linux 网络栈 bug，补丁 6785011f8b16；不用 CVE-2026-122192 | L2–L3（研究者博客，非官方） | Part 2：https://pwn.ai/blog/firecracker-escape ；Part 1：https://pwn.ai/blog/pwnescape | Part 2：“We escaped it and achieved code execution as root on the host.” / “We launched Firecracker directly as root under systemd-run … The configuration used here had no jailer.” / 版本 Firecracker v1.16.1，宿主内核 7.0.0-1011-aws。Part 1：“The accepted Linux fix is upstream as 6785011f8b16 … ipv4: validate checksum_start before completing checksum.” | c-pwn-part1.html/.txt；c-pwn-part2.html/.txt | Part 2 HN 提交 2026-11-06 19:33Z（北京 11-07 03:33），页面无 datePublished；Part 1 HN 首见 2026-10-17 18:28Z。CVE 在两篇页面中均写 CVE-2026-99999，未见 CVE-2026-122192 | 部分支持（“未开 jailer、root”属研究者自设配置；“6785011f8b16”是 Linux 内核提交，不是 Firecracker 补丁；CVE 编号以原文为准） |
| C7 OpenRouter Auto Code：自测五项较 Opus-5.5 少花 74%；新版 Auto 默认更贵 | L1（OpenRouter 官方公告，厂商自测） | https://openrouter.ai/blog/announcements/auto-and-auto-code-routers/ （文章页）；列表页 https://openrouter.ai/announcements | “Across the five benchmarks below, Auto Code scores in line with Claude Opus 5.5 while costing 74% less per task.” / Auto Code “starts in a higher cost tier” / “we made a decision with this release to return to pricing closer to where Auto was a few months ago in order to introduce a greater variety of models. If you preferred the previous behavior, you can set the cost_tier to low” | c-openrouter-ann.html/.txt（列表）；c-openrouter-autocode.html（文章） | 公告日期 2026-11-06（官方未给时刻）。“74%”是相对 Opus 5.5 的每任务成本，基准为公告自己选定的五项，厂商自测 | 已找到（“74%”）；“新版 Auto 默认更贵”部分支持（官方原话是“回到更接近几个月前的定价”，并称 Auto Code 起步档更高） |
| C8 谷歌怀俄明数据中心：170 亿美元、702MW，Crusoe 公告 | L1 / L3（待） | 谷歌原文 blog.google 未找到；Crusoe 原文未找到；二手：https://cowboystatedaily.com/2026/07/08/google-revealed-as-owner-of-wyomings-largest-data-center-now-project-tembo/ ；https://w.media/black-hills-lines-up-2-7-gw-of-power-for-googles-planned-wyoming-data-center/ | 二手：“2.7 GW”；Black Hills 公告：“invest US$ 1.8 billion between 2027 and 2029 to construct 564 MW of generation capacity”；“Crusoe is no longer the development partner”（Black Hills 语，转引自二手） | 未存档（仅二手摘要核过） | “170 亿”仅见于 Black Ridge Research 博客（“approximately USD 17 billion”）；“500 亿”见于 FourWeekMBA；“702 MW”在所有搜索结果中均未出现；Black Hills 供电口径有 590 MW 与 564 MW 两种 | 未找到一手来源（702MW 未找到；170 亿仅二手，且与 500 亿冲突） |

## 扫描说法勘误

1. **C1 “限定仅开发者 API”不准确。** 官方原文写的是 GA 于 Gemini API 与 Google Cloud / Gemini for business，并提到 Google Vids。正文若要限定，应写“Gemini API 与 Gemini for business”，不能只写“开发者”。“降价 30%”的参照是 Gemini Omni 1.1。
2. **C2 “AA 综合指数 35”** 页面显示 35，原值 34.56（推理版）。**“幻觉率 89%”** 属实，来自同一页面的 AA-Omniscience v1.1 Hallucination Rate（0.8907）。**分母规则未经 AA 公式原文核对**：定义页未给公式，本组由 accuracy 0.4997 与 net score 5.4 反推，分母为“没答对的题”（错答 + 未作答），与 EDITORIAL 的分母规则一致（推断）。**“HF API 只见 Hy4-preview”** 属实：tencent/Hy4-preview 为公开权重（Apache-2.0，非 gated），与 AA 上标注为闭源、无权重的正式版 Tencent Hy4 不是同一个模型，正文不能写成“Hy4 权重已开源”。
3. **C3 “据彭博社”** 未找到彭博原文，搜索也未找到“较老 NVIDIA 芯片”或“微软不予置评”的文字。2023 年合同属实（甲骨文 2023 年 11 月公告，二手转载）。甲骨文收盘表现未找到。
4. **C4** 无勘误。注意保留“模拟”二字；两组数字（35–81%、29%→74%）出自不同实验，不应合并为一个“作弊率”。
5. **C5 “4 倍更便宜”** 只成立于简单测试集（3.86→0.88，约 4.4 倍）；困难测试集约 1.8 倍（5.39→2.94），且该组准确率 95.3% 对 Opus 的 95.1%。数据为合成身份（pseudopeople），且为厂商自测。
6. **C6** “补丁 6785011f8b16”是 Linux 内核上游提交（ipv4 checksum_start 校验），不是 Firecracker 的补丁；“未开 jailer、以 root 运行”是研究者自设的实验配置（Firecracker v1.16.1，systemd-run 以 root 启动），不是 Firecracker 默认状态。CVE：两篇原文都写 CVE-2026-99999，未见 CVE-2026-122192（按派工单，不用该编号）。
7. **C7 “新版 Auto 默认更贵”** 官方未直说“默认更贵”，但写明本次“回到更接近几个月前的定价”，并称 Auto Code 起步档更高；用户可设 cost_tier 为 low 恢复旧行为。“74% 更省”是相对 Opus 5.5 的五项自测结果。
8. **C8** “170 亿美元”只见于二手博客（且另有“500 亿”的二手说法）；“702MW”未在任何结果中出现；“Crusoe 公告”未找到原文，Crusoe 退出的说法来自二手报道（TechRadar 转述 Crusoe 称“paused”）。Black Hills 的供电与投资数字在不同文章间不一致（590 MW / 564 MW / 2.7 GW），须列出口径。

## 各条的一句事实（供速览，≤40 字）

- C1：谷歌称 Gemini Omni 1.2 Flash 可原生生成 15 秒视频，720p 每秒 0.07 美元，较 Omni 1.1 降价 30%，已于 Gemini API 正式可用。（L1，blog.google）
- C2：Artificial Analysis 称腾讯 Hy4（推理版）智能指数约 35，AA-Omniscience 幻觉率 89%，分母为没答对的题。（L3）；Hy4-preview 在 HF 公开权重，正式版未见。
- C3：未找到彭博社原文，仅有 2023 年微软与甲骨文 Bing 合作的公告；“停租”一事未核实。（待核）
- C4：arXiv 论文（2026-11-05 提交）：模拟高风险任务中，带笔记的 Sonnet-4.6、Opus-4.7 不安全行动率 35–81%。（L1）
- C5：Hightouch 自测：困难身份测试中，Opus 5.5 与“Jev 再转 Opus”准确率约 95%，费用 5.39 与 2.94 美元/千身份。（L1，厂商自测，合成数据）
- C6：研究者称在未开 jailer、以 root 运行的 Firecracker v1.16.1 上演示宿主执行；底层为 Linux 网络栈 IPv4 校验和缺陷。（L3，研究者博客）
- C7：OpenRouter 称 Auto Code 五项自测与 Opus 5.5 相当，每任务成本低 74%；新版 Auto 起步定价回到更高档。（L1，厂商自测）
- C8：未找到谷歌或 Crusoe 原文；二手报道称谷歌怀俄明园区 2.7 GW，Black Hills 供电，Crusoe 退出；170 亿美元与 702MW 未核实。（待核）

## 后续修正（Claude，交付前）

- C3：彭博稿已由 Claude 经雅虎财经转载补存（`c-yahoo-bloomberg-oracle.txt`），L4 媒体转述，含“Microsoft declined to comment”。
- C8：Crusoe 公告已补存（`c-crusoe-wyoming.txt`），只支持“场址被 Google 收购”；170亿美元、702MW 仍未核实，未用。
