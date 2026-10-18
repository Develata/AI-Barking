# 1017 期 D 组取证（速览，D1–D10）

执行方：claude-haiku-5-5（派工单 `.handoff/2026-10-17-1017-evidence.md` 的 D 组）。
时区：北京时间（UTC+8）；页面只给日期的写“官方未给时刻”。抓取时刻 2026-10-18 19:40–19:49（北京时间）。
存档前缀 `d-`；原件（html、headers）只存本地，文本（txt）入库。取证日志见 `capture-log-d.md`，摘句机械核对脚本数据见 `d-quotes.tsv`（36 条，35 条 grep 命中，1 条因原文换行断开，已改为断点前的完整片段，验证后命中）。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

## 一览

| # | 条目 | 级 | 状态 | 一手来源 |
|---|---|---|---|---|
| D1 | Lean 内置多个验证内核 | L2（Lean FRO 员工个人博客，署名） | 部分支持 | leodemoura.github.io 博客 |
| D2 | Amazon Bee 两款设备 | L1 | 已找到 | aboutamazon.com 官方新闻 |
| D3 | Apple 审核指南更新 | L1 | 已找到 | developer.apple.com/news |
| D4 | 腾讯混元 Hy-Memory 2.0 | L1（厂商自家评测） | 已找到（分数为厂商自测） | memory.hunyuan.tencent.com、GitHub |
| D5 | RTX Spark 两个起售价 | L1 | 已找到（两处价格不一致，全部记录） | nvidianews.nvidia.com、blogs.nvidia.com |
| D6 | Replit 与 Anthropic | L4（TechCrunch 报道，内含 CEO 引语） | 部分支持 | techcrunch.com |
| D7 | Google AI 湿实验 | L1（复用 1016 存档） | 已找到 | research.google（1016 E 组存档） |
| D8 | Waymo 多伦多 | L1（复用 1016 存档） | 已找到 | waymo.com（1016 D 组存档） |
| D9 | Elorian Glimpse | L1（厂商自家评测） | 已找到（自测，非第三方） | elorian.ai |
| D10 | Reflection Beam 的 AA 评测 | L3（AA 自身数据） | 已找到 | artificialanalysis.ai |

---

## D1 Lean 内置多个验证内核

- **一句事实**：Lean v4.35.0 带 `lake check` 命令；Lean 随附四个额外内核（nanoda、lean4lean、con-leche、con-ron），`lake check --paranoid` 可用全部内核复查；其中 con-leche 由 Claude 在 Joachim Breitner 的“heavy supervision”下实现并证明一致性。
- **级**：L2（署名员工个人博客；Lean FRO 相关）。
- **一手来源**：https://leodemoura.github.io/blog/2026-10-15-lean-now-ships-with-verified-kernels/ 。存档 `d-d1-lean-blog.html`、`d-d1-lean-blog.txt`。
- **发布时刻**：页面日期 2026-10-15（URL 与页面一致），官方未给时刻。
- **原句**：“Lean now ships with four additional kernels: nanoda …, lean4lean …, con-leche …, and con-ron …. You can recheck your development with all of them using the command lake check --paranoid.”；“It was implemented and proven consistent by Claude (Fable and Opus), under heavy supervision by Joachim Breitner at the Lean FRO.”；“We also expect the verification of lean4lean to be completed in the next few months, and then we will have three verified kernels in our distribution.”；“However, con-leche is not bulletproof …”。
- **限定词与要点**：只有 con-leche 有一致性证明（“proven consistent”）；con-ron 是 AI 把 con-leche 译成 Rust 后，用 Aeneas 证明 Rust 版细化 con-leche 的检查算法（原文：“prove that the Rust version refines con-leche's checking algorithm”）；lean4lean 的验证“expected … in the next few months”，即未完成。标题 “Verified Kernels” 比正文宽。
- **扫描说法勘误**：线索称“部分内核由 Claude 在研究者监督下协助实现”。原文是 con-leche 由 Claude **实现并证明**（“implemented and proven consistent by Claude”），且只指这一个内核；“协助”不准。
- **热度**：HN 50114758，152 分 / 50 评，2026-10-16 14:16（北京）。线索 “129/45” 为较早数字，见 `capture-log-d.md`。

## D2 Amazon Bee 两款设备

- **一句事实**：Amazon 称 Bee Frontier 现以 $74.99 在 Amazon.com 上架（Onyx、Lavender、Sage 三色）；Bee Eclipse“将很快上架”，起价 $99.99（Silver、Midnight、Honey、Blush）。
- **级**：L1（Amazon 官方新闻）。
- **一手来源**：https://www.aboutamazon.com/news/devices/amazon-personal-ai-bee-frontier-bee-eclipse 。存档 `d-d2-amazon-bee.html`、`.txt`。
- **发布时刻**：页面无日期，官方未给日期与时刻。HN 投稿时刻（非官方）：2026-10-15 20:12（北京）。
- **原句**：“Bee Frontier [...] is available now for $74.99 in Onyx, Lavender, and Sage at Amazon.com. Bee Eclipse [...] will be available soon, starting at $99.99 in Silver, Midnight, Honey, and Blush.”
- **限定词**：Eclipse 未上架（“will be available soon”）。页面称“Bee Private Compute … where no one, including Amazon, can access your data”，属厂商自述，未独立核实。
- **GeekWire 补充（L4）**：https://www.geekwire.com/2026/amazons-new-bee-ai-devices-have-a-bystander-problem-says-former-aws-security-engineer/ 返回 **403（blocked）**，未取得正文，没有存档。仅搜索摘要（未核）显示：一位前 AWS 高级安全工程师在 LinkedIn 长文批评旁人同意机制，并引用汉堡数据保护机构对 Meta 眼镜的判断。此条只作线索，正文未读，不可引用原句。
- **热度**：HN 50102536（10 分 / 1 评，10-15 20:12）；HN 50118395（3 分 / 0 评，10-16 21:51）。

## D3 Apple 审核指南更新

- **一句事实**：Apple 10/16 更新 App Review Guidelines；其中 1.2 条新增说明：承载用户内容的应用须主动防范并移除非自愿亲密影像（含 AI 工具生成），否则可能违反开发者行为准则；2.5.19 为新增条款，澄清隐藏、休眠或未文档化功能的政策。
- **级**：L1（Apple Developer News）。
- **一手来源**：独立条目 https://developer.apple.com/news/?id=vvzx6cul（“Strengthening App Review Guidelines against nonconsensual intimate imagery and bait-and-switch apps”）；总条目 https://developer.apple.com/news/?id=92th7e5i（“Updated App Review Guidelines are now available”）。存档 `d-d3-apple-news-nonconsent.*`、`d-d3-apple-guidelines-news.*`，并存列表页 `d-d3-apple-news.*`。
- **发布时刻**：两条页面日期均为 October 16, 2026，官方未给时刻。
- **原句**：“1.2: Content moderation requirements now explicitly cover nonconsensual intimate imagery — including imagery generated using AI tools — as content that apps must proactively prevent and remove from their platforms. Developers who fail to implement adequate safeguards — or don't remedy violations after notice — risk violating the Developer Code of Conduct and being removed from the App Store.”；总条目中 “1.2: Clarified app content and app types that are subject to this guideline …”；“2.5.19: New guideline clarifies long-standing policy on hidden, dormant, or undocumented features, formerly covered in Guideline 2.3.1.”
- **限定词**：总条目把 1.2 写成“Clarified”（澄清适用范围），独立条目写 “now explicitly cover”；两者是同一更新的两面，不取舍。
- **热度**：HN 检索“Apple App Review nonconsensual”0 条。Reddit 检索失败（返回登录页）。

## D4 腾讯混元 Hy-Memory 2.0

- **一句事实**：腾讯混元 10/16 开源发布 Hy-Memory 2.0（模型 Hy-Memory-30B-A3B、SDK、MemHub 应用）；厂商报告同一模型在四套记忆系统测试中的 6 项基准等权平均为 55.49，高于 Opus5 的 54.62（厂商自测）。
- **级**：L1（官方；数字为厂商自家评测，非第三方）。
- **一手来源**：https://memory.hunyuan.tencent.com/（存档 `d-d4-hunyuan.html`、`.txt`）；GitHub https://github.com/Tencent-Hunyuan/Hy-Memory（存档 `d-d4-hy-memory-github.*`）；HF 模型页 https://huggingface.co/tencent/Hy-Memory-30B-A3B 与论文 https://arxiv.org/pdf/2610.22196 **未存档**（未抓取）。
- **发布时刻**：页面标注“开源发布 · 2026.10.16”，官方未给时刻。
- **原句**：“Hy-Memory-30B-A3B 得分 55.49，高于 Opus5 的 54.62、DeepSeek V4.1 Flash 的 53.64，以及 Qwen3-30B-A3B 的 44.37。”；“注 · 六项为 LoCoMo、LongMemEval、PersonaMem v2、Travel、Shopping、CMB，等权平均，数值越高越好。”
- **比较对象**：页面写 “Opus5”（闭源），即 Opus 5，不是 Opus 5.5。
- **核算**：页面表中 Hy-Memory 行六项数字的算术平均为 55.49，与页面吻合；Opus5 行六项平均约 54.63，与页面 54.62 差 0.01，属舍入（推断，非原文）。页面标题写“四套记忆系统平均”，原文说明为同一模型接入 Hy-Memory、Mem0、MemOS、EverMemOS 四套记忆系统测试、取六项基准平均。
- **限定词**：页面自注 “Hy-Memory、Mem0、MemOS 用于 RL 训练，EverMemOS 训练未见，用于检验泛化”；“记忆专项训练 47.66 → 55.49”（相对基座 HY3-A3B Base）。
- **许可证**：Apache-2.0。原文（GitHub LICENSE 原件，存档 `d-d4-license-raw.txt`）：“Hy-Memory is licensed under Apache-2.0.”
- **热度**：HN 检索“Hy-Memory”0 条。

## D5 RTX Spark 两个起售价

- **一句事实**：NVIDIA 10/16 新闻稿称 RTX Spark 笔记本“available globally today”，起价 $2,599；NVIDIA 博客（10/17）称起价 $2,399。
- **级**：L1（NVIDIA 官方）。
- **一手来源**：新闻稿 https://nvidianews.nvidia.com/news/nvidia-rtx-spark-pcs-available-today-a-new-beginning-for-windows-pcs（存档 `d-d5-nvidia-news.*`）；博客 https://blogs.nvidia.com/blog/local-ai-rtx-spark-windows-laptops-celebration/（存档 `d-d5-nvidia-blog.*`）。
- **发布时刻**：新闻稿 October 16, 2026；博客 October 17, 2026；均未给时刻。
- **原句**：新闻稿 “RTX Spark laptops are available globally today from ASUS, Dell, HP, Lenovo, Microsoft and MSI through their websites and major retailers, starting at $2,599 …. Pricing and configurations vary by region.”；博客 “Starting at $2,399, laptops from ASUS, Dell, HP, Lenovo, Microsoft and MSI bring powerful local AI …”。
- **限定词**：新闻稿说明价格与配置因地区而异；博客未说明配置与地区。两处价格不一致，全部记录，不取舍。
- **热度**：HN 无 NVIDIA 本稿直接命中；相关 HN：50119594（1 分，IgorsLab，10-16 23:25）、50139919（2 分，Surface Laptop Ultra 评测，10-18 18:59）。

## D6 Replit 与 Anthropic

- **一句事实**：TechCrunch 报道，Replit CEO Amjad Masad 在 Disrupt 上称公司已不再与 Anthropic 合作（“we just felt like they were a better partner for now”，指 OpenAI）；其后 Masad 对 TechCrunch 称用户仍可选择 Claude；一位知情人士否认合作已终止。
- **级**：L4（TechCrunch 报道；CEO 引语本身按署名 L2 处理）。
- **一手来源**：https://techcrunch.com/2026/10/17/replit-is-no-longer-working-with-anthropic-ceo-says/（存档 `d-d6-techcrunch-replit.html`、`.txt`）。
- **发布时刻**：文中注明 “originally published on October 16 at 2:22 pm PT”（北京时间 10-17 05:22），之后更新；标题也被更新为 “Replit makes OpenAI its default over Anthropic, CEO says”。正文另写 “said Thursday onstage”，与发布日期不一致，未核。
- **原句**：“Our users can choose Anthropic's Claude or other supported models…OpenAI is now our preferred partner for powering Replit Agent, based on performance, economics, and the strength of our partnership.”（Masad 对 TechCrunch 的陈述）；“customers can still access Anthropic models on the platform, but that OpenAI's ChatGPT is now the default”（TechCrunch 转述）；“One person familiar … said the partnership was still in place.”
- **Replit 官方材料**：文档 `docs.replit.com/features/agent/model-selector`（存档 `d-d6-replit-docs-model-selector.*`）列出 Claude Fable 5.1、Opus 5.5 等模型，可侧证 Claude 仍可选，但页面不是对此事的原句；9 月下旬博客 `replit.com/blog/free-the-models`（存档 `d-d6-replit-free-models-blog.*`）称 Claude Fable 5 / 5.1 运行于 Repl 中；8 月 `replit.com/blog/replit-introduces-free-mode`（存档 `d-d6-replit-free-mode-blog.*`）宣布与 OpenAI 的合作。**未找到** Replit 官方帖子确认“切换默认”或“仍可选 Claude”的原句。
- **热度**：HN 50129206（2 分 / 1 评，10-17 16:56）与 50125160（2 分 / 0 评，10-17 05:44），两条都指向 `…/2026/10/16/…` 旧 URL；Reddit 检索失败（登录页）。

## D7 Google AI 湿实验（复用 1016 存档）

- **一句事实**：Google Research 10/16 博客称，agents 在湿实验中“discovering single-digit nanomolar protein binders for cancer targets”（针对癌症靶标发现单位数纳摩尔结合蛋白）；其中 FAP 一例，原文称“在几次实验运行内”得到单位数纳摩尔亲和力的结合物。
- **级**：L1。
- **一手来源**：https://research.google/blog/grounding-ai-agents-in-biological-reality-building-blocks-for-autonomous-wet-labs/ 。存档复用 `docs/2610/1016/sources/e-e3-google-wetlab.html`、`.txt`（未重抓）。
- **发布时刻**：页面 October 16, 2026，官方未给时刻。
- **原句**：“ultimately discovering single-digit nanomolar protein binders for cancer targets”；“In just a few experimental runs, the top designs yielded binders with single-digit nanomolar affinity”；“Agents need real experimental readouts … cannot replace empirical validation.”
- **限定词**：原文限定为“for cancer targets”（靶标范围）与“in the wet lab”（湿实验）。原文**没有**“非临床”字样；派工单提示的“非临床”是推断，不能当原文引用。
- **热度**：HN 检索“Google Research wet lab agents binders”0 条。

## D8 Waymo 多伦多（复用 1016 存档）

- **一句事实**：Waymo 10/16 宣布进入多伦多，目标“welcoming our first riders in 2028”（2028 年迎接首批乘客），眼下“begin driving manually”（以人工驾驶起步的高精地图测绘）。
- **级**：L1。
- **一手来源**：https://waymo.com/blog/2026/10/waymo-in-toronto/ 。存档复用 `docs/2610/1016/sources/d-waymo-toronto.html`、`.txt`（未重抓）。
- **发布时刻**：页面 October 16, 2026，官方未给时刻。
- **原句**：“…with the goal of welcoming our first riders in 2028.”；“In the coming days, you'll start to see Waymo vehicles on the streets of Toronto as we begin driving manually. We are taking a rigorous, phased, safety-first approach starting with high-definition mapping of local street networks and testing with trained autonomous specialists behind the wheel.”
- **限定词**：2028 是“goal”（目标）；当前是“driving manually”、高精地图测绘，并有受过训练的自动驾驶专员在驾驶位上测试。
- **热度**：HN 50118981“Waymo in Toronto”，326 分 / 443 评，2026-10-16 22:44（北京）；Reddit 检索失败。

## D9 Elorian Glimpse

- **一句事实**：Elorian 10/16 以“Research preview”身份发布 Glimpse，通过 API 与聊天界面提供，输入 $0.60、输出 $3.00（每百万 token）；其自家对比表中，相对 GPT-6 Astra，12 项中 4 项领先、1 项持平、6 项落后、1 项 Astra 无数据。
- **级**：L1（厂商自家评测，非独立第三方，正文需点明）。
- **一手来源**：https://elorian.ai/blog/introducing-glimpse （存档 `d-d9-elorian-glimpse.html`、`.txt`）。
- **发布时刻**：页面 “Elorian · 16 October 2026”，官方未给时刻。
- **原句**：“Research preview”；“Glimpse is available through API and our chat UI today.”；“Input tokens cost $0.60 per million and output tokens $3.00 per million: both rates are 94% lower than GPT-6 Astra …”；“Glimpse exceeds Astra on four evaluations …, and matches it on embodied reasoning.”
- **价格口径**：较 Astra（$10 / $50）低 94%；较 Opus 5.5、GPT-6.1 Sol 的 85%、70% 亦写于页面；较 Gemini 3.8 Flash“introductory rates”低 20%（Gemini 的是引入期价，不是常规价）。
- **对比表（Astra 列，共 12 项）**：
  - 领先 4：WGO Bench（42.70 vs 39.42）、CharXiv-R（92.90 vs 92.10）、UGC-VideoCap（82.66 vs 80.32）、FSC-147（83.36 vs 82.02）。
  - 持平 1：ERQA（83.75 vs 83.75）。
  - 落后 6：BabyVision（95.36 vs 97.68）、HSS（46.36 vs 53.6）、DREAM-1K（76.04 vs 79.05）、TOMATO（93.60 vs 96.02）、Video-MME v2（56.68 vs 64.06）、CountQA（82.66 vs 86.42）。
  - 无 Astra 数据 1：E.T. Bench（Glimpse 63.55；Sol 59.79；Gemini 63.45）。
  - 同表对 Sol（12 项）：领先 7、落后 5。
- **限定词**：“Unless otherwise noted, all model results use the highest thinking effort and native agent harness”；“HSS includes paper-reported results”（HSS 含论文报告值，不全是本次复现）；Video-MME v2 的 Sol 列用 max 档，其余视频评测 max 档结果不可得。
- **热度**：HN 50128667（2 分 / 0 评，10-17 15:04）。

## D10 Reflection Beam 的 AA 评测

- **一句事实**：Artificial Analysis 10/16 称 Reflection 的 Beam 在 Intelligence Index（v4.3，10 项）得 30 分，为美国开放权重模型第二，仅次于 Meta Muse Spark Open（39）；AA 称 Beam 为开放权重、Apache 2.0 许可，权重已在 Hugging Face 公开。
- **级**：L3（AA 自身数据，仅限其自测结果）。
- **一手来源**：文章 https://artificialanalysis.ai/articles/reflection-beam （存档 `d-d10-aa-reflection-beam.*`）；模型页 https://artificialanalysis.ai/models/reflection-beam （存档 `d-d10-aa-model.*`，html 约 4.3 MB）。
- **发布时刻**：文章与模型页均为 October 16, 2026（模型页 “Beam 1.0 was released on October 16, 2026.”），官方未给时刻。
- **原句**：“Beam scores 30, behind only Meta's Muse Spark Open (39) and ahead of yesterday's NVIDIA Nemotron 3.5 Super VL (26) release.”；“Proprietary models still set the frontier, with GPT-6 Astra (max) scoring 53 on the same budget (~27k).”；“Running in Mirror, Reflection's open-source CLI harness, Beam scores 28 on the Artificial Analysis Coding Agent Index.”；“Beam 1.0 is released under the Apache 2.0 license.”；模型页“Model weights” 链接指向 `huggingface.co/reflection/Beam-1.0-501B-A23B`。
- **条件与口径**：Intelligence Index v4.3（文中原词）；GPT-6 Astra 为 max 档、同预算约 27k 输出 token；Beam 约 26k 输出 token。原文未见 Beam 的 effort 档位，记为“未见”。Coding 分数在 Mirror harness 下测得。
- **权重**：AA 称已公开（HF 页面未抓取，只据 AA 页面）。
- **热度**：HN 50120173（1 分 / 0 评，10-17 00:08，AA 模型页）；发布相关 HN 条目：50118213（12 分 / 1 评，Reflection 博客，10-16 21:34）、50118552（19 分 / 2 评，HF 博客，10-16 22:05）、50118549（6 分 / 1 评，HF 组织页，10-16 22:05）、50118782（10 分，10-16 22:28）、50119226（6 分，10-16 23:01）。

---

## 扫描说法勘误（D 组）

1. **D1**：线索“部分内核由 Claude 在研究者监督下协助实现”不准。原文：con-leche 由 Claude（Fable 与 Opus）实现并证明一致性，Breitner 监督。标题“Verified Kernels”比正文宽：只有 con-leche 有一致性证明，con-ron 为 Aeneas 证明的 Rust 版细化，lean4lean 的验证待完成。HN 数字已变为 152 分 / 50 评（线索写 129/45）。
2. **D2**：线索“Frontier 74.99、Eclipse 99.99 美元”数字正确，但发售状态不同：Frontier“available now”，Eclipse“will be available soon”。GeekWire 批评文章未能取得（403）。
3. **D3**：线索的 1.2 与 2.5.19 两点正确。“1.2 要求…防范并移除非自愿私密图像（含 AI 生成）”的原句在独立条目 vvzx6cul；总条目写的是 1.2 “Clarified app content and app types”。
4. **D4**：“比较对象是否 Opus 5 而非 5.5”：是 Opus 5（54.62）。55.49 为六项基准等权平均（同一模型接入四套记忆系统）；数字为厂商自测。许可证 Apache-2.0。
5. **D5**：日期核实：新闻稿 10/16，博客 10/17（均无时刻）。两处价格 $2,599 与 $2,399 均为原文，不一致，全部记录。
6. **D6**：“用户仍可选 Claude”属实，但是 CEO 对 TechCrunch 的说法，知情人士否认合作终止，Replit 官方未见确认原句。TechCrunch 标题在更新后由“no longer working with Anthropic”改为“makes OpenAI its default”。HN 链接用的是旧 URL。
7. **D7**：“非临床”不是原文限定词，原文限定为“cancer targets”与“in the wet lab”。
8. **D8**：2028 是“goal”，人工驾驶测绘是“begin driving manually”，不是已开始的乘客服务。
9. **D9**：“与 GPT-6-Astra 有输有赢”属实：自家表中对 Astra 4 领先、1 持平、6 落后、1 无数据。所有结果为厂商自测。
10. **D10**：1005 期线索“权重本月稍后发布”已过期：AA 页面称 10/16 发布，权重开放，Apache 2.0；HF 页面未抓取。AA 的 Intelligence Index 版本为 v4.3（文中原词），检索摘要中的 v4.3.2 未在原文中找到，不采用。

## 未完成与失败

- GeekWire（D2 补充）：403，blocked，未存档正文；搜索摘要仅作线索。
- Reddit 热度（D3、D6、D8 等）：`barking heat` 返回登录页，无法解析，未取数据。
- HF 模型页（D4、D10）与论文 arXiv（D4）：未抓取。
- Waymo / Google 的 HN 热度已取，Reddit 失败。
- firecrawl 额度已用完，全程未使用；网页均用 `barking fetch`，搜索用 WebSearch。
- 未截图（D 组按派工单不截图）。
