# 1127 期 A 组取证：小米汽车世界模型 2.0 的“全面 SOTA”口径

- 取证执行方：Sonnet 5.5（`claude-sonnet-5-5`）；取证时间：2026-11-28 23:32–23:50（北京时间，本机时区即 UTC+8）。
- 方法：论文 PDF 全文转文本（`a-arxiv-pdf.txt`，40 页）逐表核对，并把表格页渲染成图目视确认粗体/下划线（“最佳/次佳”标记）；项目页、官方发布稿、中文转载稿逐字摘句。
- 存档文件前缀均为 `a-`，见 `capture-log-a.md`。“级”按 `EDITORIAL.md` 事实分级；“已核实”仅指对论文、项目页、官方发布稿原文的核对，不含对小米方法本身的独立复现。
- **已验证 / 推断 / 未验证** 在各条“条件/口径”里分开标注：凡标“推断”者是我从表格算出来的，论文未直接写。

## 0. 先给结论（摘要）

1. **“全面 SOTA”只出现在发布稿，不在论文里。** 官方发布稿（小米技术，微博头条文章，2026-11-27 17:00）原句：“在多个主流基准测试中全面取得SOTA”。同一天的官方微博短帖只写“PAI-Bench-G 评测中……均位列报告所列模型第一”。论文摘要与正文没有“SOTA”自称，只对 nuScenes、TriWorldBench 自称 “competitive”（见 E-A3、E-A9、E-A10）。
2. **PAI-Bench-G 的“最高分”是 8 个子指标的未加权平均，小米在其中两列垫底。** 驾驶子集：Ours 78.0（综合分最高），但 Aesthetic Quality 46.6（7 个模型里最低，最高 HunyuanVideo1.5 48.8）、Imaging Quality 64.5（7 个里倒数第二，最高 LingBotWorld 69.1）。机器人子集 Aesthetic Quality 44.8（7 个里最低，最高 HunyuanVideo1.5 51.0）。扫描把 Imaging Quality 的 69.1 记给了 Hunyuan，**实为 LingBotWorld；Hunyuan 是 69.0**。
3. **PAI-Bench-G 评测用的是 Mid-train 模型，不是蒸馏后的 Post-train 模型**（论文 5.1 节原句）；闭环仿真用的是 Post-train 模型。
4. **碰撞率 0.4% 对 1.2%** 是在 1000 个 NAVSIM 测试场景、小米自建的世界模型仿真器里测的，对比对象是 ST-JEPA-E2E（引用为 “Anonymous, 2026, under review”），另一个对比 DrivoR-E2E 为 1.4%；同表 Ego Progress 小米 84.5 低于 ST-JEPA-E2E 的 84.7。不是实车或公开道路数据。
5. **LIBERO Plus：小米 75.0，同表 GAM 85.5**，项目页副标题自己写了 “GAM reports the highest average at 85.5%”；项目页和官方发布稿都没展示 Background（小米 71.0，GAM 94.3）与 Sensor（小米 71.4，GAM 95.3）两列，论文表里有。GAM 在论文里没有引用文献。
6. **论文对比的模型里没有 Veo、Sora、Seedance、Kling 等闭源商用视频模型**（论文全文 grep 零命中）；论文没有标注各模型开闭源，也没有说明为什么不比这些模型；TriWorldBench 一节论文自己写明 “We compare against methods with publicly available papers”。
7. **“因果”：无任何定量因果指标**。定性案例两个（落石避让、追尾，Fig. 7）；“causal”在论文里同时指“块间因果注意力的自回归生成”（式 (1)）和“物理因果数据”，不是因果推断意义上的度量。
8. **对读者**：论文与项目页都没有给出代码、权重或数据的公开链接；官方称落地场景是“合成数据生成”和“仿真测试”（云端研发），官方发布稿全文不含 HAD、车端、量产等字样。同日另发的车端 HAD 1.20（“因果链”“车端世界模型”）是另一件事，官方尚未说明二者的继承关系。

---

## 1. 证据表（A1–A8）

### A1 论文与项目页身份

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 论文 arXiv 编号与标题 | L1 | https://arxiv.org/abs/2611.01677 | `arXiv:2611.01677 (cs)`；`Title:Xiaomi Auto World Model 2.0: Causal World Models for Real-World Driving and Embodied Intelligence` | — | 编号正确。PDF 首页署名 “Xiaomi Auto World Model Team”；arXiv 列 23 位作者（Zhenxin Zhu 等，项目负责人 Bing Wang）；PDF 无单位栏，署名只写 Xiaomi Auto World Model Team | 已找到 |
| v1 提交时刻 | L1 | https://arxiv.org/abs/2611.01677（`a-arxiv-abs.txt`）；API `a-arxiv-api.xml` | `[v1] Sun, 1 Nov 2026 07:53:48 UTC (26,874 KB)`；API `<updated>2026-11-01T07:53:48Z</updated>` | — | = 2026-11-01 15:53:48 北京时间。仅 v1，无 v2。扫描所称“论文 11-01 15:53 提交”是北京时间，正确；与 11-27 官方发布不矛盾：论文提交日先于官方公告 26 天 | 已找到 |
| 论文摘要不含 SOTA 自称 | L1 | https://arxiv.org/pdf/2611.01677 | 摘要末句：“These applications demonstrate its ability to generate physically consistent future states and support world-state prediction, policy learning, and action generation for Physical AI.” | `09-midtrain-eval-note.png`（5.1 节） | 全文 grep “SOTA/state-of-the-art”：仅用于形容**基线**（“state-of-the-art end-to-end baseline”）及 NAVSIM 一句 “best leaderboard performance in the Trainval comparison setting”（见 A5） | 已找到 |
| 项目页 | L1 | https://xiaomiautoworldmodel.github.io/CausalWorldModel/ | 标题 “Xiaomi Auto World Model 2.0”；`Xiaomi Auto World Model Team / November 2026` | `12-project-page-libero-plus.png` | 服务器 `Last-Modified: Tue, 03 Nov 2026 03:57:16 GMT`（= 北京 11-03 11:57）；页内样式版本串 `v=202610281024`。即页面主体 11-27 之前已在线；我没有 11-27 之前的快照可比 | 已找到 |
| 官方发布稿（小米技术，微博头条文章） | L1 | https://weibo.com/ttarticle/p/show?id=2309405369910959407174 | 标题“小米汽车世界模型 2.0：为真实世界辅助驾驶与具身智能而生”；“小米技术 11-27 17:00 已编辑 阅读数：26万+” | `01-official-weibo-article-sota.png` | 17:00 北京时间；官方页“已编辑”，我存的是 11-28 版本。页面半数内容对非粉丝折叠，但 DOM 含全文，存档只存文章正文（评论区含网友昵称，未存） | 已找到 |
| 官方微博短帖（小米技术） | L1 | https://weibo.com/6486870325/RtkHBt2aH | 见 A2 | `08-official-weibo-post.png` | `created_at: Fri Nov 27 17:00:03 +0800 2026`；119 转发 / 155 评论 / 1121 赞（23:3x 读） | 已找到 |
| 雷军原帖 | L1（署名官方账号） | https://weibo.com/1749127163/RtsktkiH2 | “小米汽车世界模型2.0 已正式发布，感谢大家一直以来的关注和支持！相比5月份公布的进展，这次世界模型能力有了巨大进步。多视角视频生成、物理交互建模、三维场景重建与动作建模连接起来，为辅助驾驶和具身智能统一因果世界模型带来了新的范式。” | — | `created_at: Sat Nov 28 12:25:02 +0800 2026`；无 SOTA 字样；181 评论 / 52 转发 / 1274 赞（23:3x 读）。存档 `a-weibo-5370204134838420.json` | 已找到 |
| 前代（1.0）身份 | L1 | https://jointwm.github.io/ ；论文 Table 1 | 项目页底部 “Explore version 1.0 ↗”；论文 `Ours(1.0) (Zhou et al., 2026c)` | — | 1.0 在 2026-05 发布（官方稿“今年5月，小米汽车发布Xiaomi Auto World Model”）；1.0 当时的中文标题同样是“主流基准测试全面 SOTA”（同花顺 2026-05-26 11:43:03，`a-cn-10jqka-v1.txt`，L5，仅作措辞对照） | 已找到 |

### A2 官方“全面 SOTA”原话与中文转载

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| “全面 SOTA”原句 | L1 | 小米技术微博头条文章（A1） | “本次工作公开了技术报告及项目主页。模型支持 1—7路摄像头视角，训练语料规模达到 1,260万段视频片段，并展示了从辅助驾驶、机器人场景生成，到闭环仿真与World Action Model的多项结果，在多个主流基准测试中全面取得SOTA，并已在小米汽车合成数据、仿真测试等核心场景完成业务落地应用。” | `01-official-weibo-article-sota.png` | 17:00 北京。原句里“全面”没有限定“在所列对比方法中”；后文的具体句才有限定（见下行） | 已找到 |
| 官方具体限定句 | L1 | 同上 | “在PAI-Bench-G评测中，世界模型2.0的驾驶视频生成质量评分为 78.0，机器人视频生成质量评分为 77.1，均为项目主页所列对比方法中的最高分。”；“在感知理解任务上，世界模型2.0同样获得对比方法中最佳的指标结果。相比于前SOTA方法VGGT-World，世界模型2.0在不同的数据序列上均取得最优结果” | — | 限定语是“项目主页所列对比方法”，项目页的对比方法比论文表少 2 个（见 A3 勘误） | 已找到 |
| 官方微博短帖措辞 | L1 | https://weibo.com/6486870325/RtkHBt2aH | “在 PAI-Bench-G 评测中，自动驾驶和机器人两个场景的视频生成质量综合得分分别为78.0和77.1，均位列报告所列模型第一。” | `08-official-weibo-post.png` | 短帖没有“全面 SOTA”，只有“报告所列模型第一” | 已找到 |
| IT之家（转载） | L4 | https://www.ithome.com/1/033/614.htm | 标题“小米汽车世界模型 2.0 发布，已在实际业务场景中落地”；导语段“据介绍，模型支持 1—7 路摄像头视角，训练语料规模达到 1,260 万段视频片段，并展示了从辅助驾驶、机器人场景生成，到闭环仿真与 World Action Model 的多项结果，在多个主流基准测试中全面取得 SOTA，并已在小米汽车合成数据、仿真测试等核心场景完成业务落地应用。” | `07-ithome-zh-sota-report.png` | 页面时间 2026/11/27 17:13:11（北京），作者“沁沧”；页面为暗色主题；正文同句前有“官方称……” | 已找到 |
| 环球网（经新浪财经） | L4 | https://finance.sina.com.cn/roll/2026-11-28/doc-inkahkve7331920.shtml | 标题“小米汽车世界模型2.0发布：打通生成与决策，赋能辅助驾驶与具身智能”；“并在多个主流基准测试中取得 SOTA”（去掉“全面”）；“面向辅助驾驶与具身智能构建统一因果世界模型范式” | — | 2026-11-28 13:03 北京；作者署“青云” | 已找到 |
| 搜狐号（自媒体，仅作夸大实例） | L5 | https://www.sohu.com/a/1107623848_122066676 | 标题“小米汽车世界模型2.0发布：颠覆辅助驾驶与智能决策的未来！”；“使得模型能够在辅助驾驶、机器人场景生成、闭环仿真及World Action Model等多个领域取得SOTA（State of the Art）成绩” | — | 2026-11-28 14:17，署名“车迷老司机”；正文首句写“2025年11月28日”（年份写错）。不得引用，只作转述放大实例 | 已找到 |
| 新浪财闻微博转帖 | L6 | https://www.sina.cn/weibo/detail/5370245171646940.html | “【#小米汽车世界模型2.0正式发布#】……在多个主流基准测试中全面取得SOTA，并已在小米汽车合成数据、仿真测试等核心场景完成业务落地应用。” | — | 财闻官方微博，11-28 12:43 北京；页面含网友评论，未用作证据 | 已找到 |

### A3 PAI-Bench-G 表格逐列（论文 Table 4、Table 5，PDF 第 16 页）

截图：`02-pai-bench-g-av.png`（Table 4）、`03-pai-bench-g-robot.png`（Table 5）。表题原文：“Table 4 Quality evaluation on the Autonomous Vehicle subset of PAI-Bench-G. AV-Quality denotes the average score; SC, BC, MS, AQ, IQ, OC, IS, and IB denote Subject Consistency, Background Consistency, Motion Smoothness, Aesthetic Quality, Imaging Quality, Overall Consistency, Image-to-Video Subject Consistency, and Image-to-Video Background Consistency, respectively. Bold and underlined values denote the best and second-best results per column.”（Table 5 “using the same metrics as Tab. 4. Robot-Quality denotes the average score.”）

**Table 4（驾驶子集，118 个视频）**，数值逐列照抄，粗体最高者标 **★**：

| 模型 | AV-Quality | SC | BC | MS | AQ | IQ | OC | IS | IB |
|---|---|---|---|---|---|---|---|---|---|
| Wan2.2-5B | 76.1 | 90.4 | 93.7 | 97.1 | 47.5 | 67.4 | 21.8 | 95.4 | 95.8 |
| Wan2.1-14B | 77.2 | 91.7 | 94.3 | 98.4 | 48.1 | 68.8 | 21.7 | 97.0 | 97.3 |
| Wan2.2-14B | 77.0 | 91.4 | 94.1 | 98.6 | 47.3 | 68.1 | 21.9 | 97.2 | 97.5 |
| DreamXWorld | 76.1 | 91.9（次佳） | 93.7 | 97.3 | 47.5 | 64.4 | 21.7 | 95.7 | 96.7 |
| HunyuanVideo1.5 | 77.4（次佳） | 91.5 | 92.5 | 99.3（次佳） | **48.8★** | 69.0（次佳） | **22.3★** | 97.9（次佳） | 98.1（次佳） |
| LingBotWorld | 77.3 | 91.2 | 94.4（次佳） | 98.8 | 48.3（次佳） | **69.1★** | 21.8 | 97.3 | 97.8 |
| **Ours** | **78.0★** | **96.2★** | **96.5★** | **99.5★** | 46.6 | 64.5 | 22.2（次佳） | **99.0★** | **99.2★** |

每列最高者：AV-Quality Ours；SC Ours；BC Ours；MS Ours；**AQ HunyuanVideo1.5 48.8（Ours 46.6，7 个里最低）**；**IQ LingBotWorld 69.1（Ours 64.5，7 个里倒数第二，仅高于 DreamXWorld 64.4）**；**OC HunyuanVideo1.5 22.3（Ours 22.2，次佳）**；IS Ours；IB Ours。Ours 在 8 个子指标里 5 列最高、1 列次佳、2 列垫底或近垫底。

**Table 5（机器人子集，174 个视频）**：

| 模型 | Robot-Quality | SC | BC | MS | AQ | IQ | OC | IS | IB |
|---|---|---|---|---|---|---|---|---|---|
| Wan2.2-5B | 74.3 | 89.0 | 91.0 | 98.0 | 46.1 | 67.8 | 19.2 | 90.3 | 92.8 |
| Wan2.1-14B | 75.2 | 87.9 | 90.3 | 98.5 | 48.8 | **71.7★** | 19.0 | 91.5 | 93.7 |
| Wan2.2-14B | 75.4 | 89.4 | 90.6 | 98.4 | 48.1 | 70.6（次佳） | 19.4 | 92.3 | 94.3 |
| DreamXWorld | 74.8 | 90.3 | 92.2 | 97.3 | 45.6 | 66.2 | 18.9 | 92.4 | 95.8 |
| HunyuanVideo1.5 | 76.9 | 89.0 | 90.6 | 99.2（次佳） | **51.0★** | 70.1 | 19.6（次佳） | **97.9★** | 97.8 |
| LingBotWorld | 77.0（次佳） | 91.8（次佳） | 93.2（次佳） | 99.0 | 50.0（次佳） | 66.8 | **19.7★** | 97.8（次佳） | 98.0（次佳） |
| **Ours** | **77.1★** | **95.4★** | **95.7★** | **99.5★** | 44.8 | 67.2 | 19.0 | 96.1 | **98.8★** |

每列最高者：Robot-Quality Ours（77.1，LingBotWorld 77.0，差 0.1）；SC、BC、MS、IB 为 Ours；**AQ HunyuanVideo1.5 51.0（Ours 44.8，7 个里最低）**；**IQ Wan2.1-14B 71.7（Ours 67.2，第 5）**；**OC LingBotWorld 19.7（Ours 19.0，与 Wan2.1-14B 并列第 5）**；IS HunyuanVideo1.5 97.9（Ours 96.1，第 3）。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 驾驶 78.0 对 Hunyuan 77.4 领先 | L1 | https://arxiv.org/pdf/2611.01677 p.16 | “Our model achieves the highest Quality Score in both domains across all evaluated models, with scores of 78.0 for autonomous driving and 77.1 for robotics.” | 02 | 数值属实；Hunyuan 77.4 为次佳；LingBot 77.3 | 已找到 |
| 驾驶 AQ 46.6（Hunyuan 48.8）落后 | L1 | 同上 Table 4 | AQ 列：`HunyuanVideo1.5 … 48.8`（粗体）、`Ours … 46.6` | 02 | 属实；Ours 为该列最低 | 已找到 |
| 驾驶 IQ 64.5（扫描称 Hunyuan 69.1）落后 | L1 | 同上 Table 4 | IQ 列：`LingBotWorld … 69.1`（粗体）、`HunyuanVideo1.5 … 69.0`（下划线）、`Ours … 64.5` | 02 | 小米 64.5 属实且落后；**扫描把 69.1 归给 Hunyuan 有误，69.1 是 LingBotWorld，Hunyuan 是 69.0** | 部分支持 |
| 机器人 77.1 对 LingBot 77.0 | L1 | 同上 Table 5 | `LingBotWorld … 77.0`（下划线）、`Ours … 77.1`（粗体） | 03 | 属实；0.1 的差距，无方差/重复运行信息 | 已找到 |
| “评分”口径 | L1/推断 | 同上 | 表注：“AV-Quality denotes the average score” | 02 | 推断（我按 8 列均值核算，与表值吻合到四舍五入：Ours 驾驶 77.96→78.0，机器人 77.06→77.1；Hunyuan 驾驶 77.42→77.4）：综合分是 8 个子指标的**未加权**平均，OC 等小数值子项与 SC 等 90+ 子项等权 | 部分支持（推断） |
| 用哪个模型评测 | L1 | https://arxiv.org/pdf/2611.01677 p.16 | “Using this shared setting, we evaluate the model in the Mid-train stage from three complementary perspectives: general video generation, scene perception, and camera-pose control.” | `09-midtrain-eval-note.png` | 同页前一段 Table 3 用“final distilled two-step results”。即 PAI-Bench-G 等生成质量评测是 Mid-train 模型；闭环仿真 5.2 节用“our model which from the Post-train stage”。论文没有给 Post-train 模型的 PAI-Bench-G 分数 | 已找到 |
| 论文自己如何概括 | L1 | 同上 p.17 | “…while also achieving the best Background Consistency, Motion Smoothness, and Image-to-Video Background Consistency in both domains.” | — | 论文正文只点名了小米最高的列（SC、BC、MS、IB），没提 AQ、IQ | 已找到 |

### A4 对比集合

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| 各主表对比方法清单 | L1 | https://arxiv.org/pdf/2611.01677 | 见下表 | 02/03/04/05/10/11 | “开源/闭源”论文**没有标注**；下表“开闭源”列是我依公开信息的推断，仅供参考，未逐个核证 | 已找到（清单）；开闭源属推断 |
| 是否含 Veo、Sora、Seedance、Cosmos 等 | L1 | 同上 | 全文 grep：Veo、Sora、Seedance、Kling、Genie、Runway、proprietary、closed-source、commercial 均 0 命中；Cosmos 在论文里出现于：Table 6 深度预测基线（Cosmos-4B、Cosmos-12B）；数据处理一句 “We then render the aligned annotations with the Cosmos-Transfer tools (Ren et al., 2025)”；致谢 “We are deeply grateful to the Cosmos team (Agarwal et al., 2026) for their helpful advice in data processing and model deployment”；及参考文献（Cosmos、Cosmos 3、Cosmos-drive-dreams）（`grep -n -i cosmos`，共 6 行）。作为跑分对比对象只有 Table 6 一处 | — | Cosmos 是 NVIDIA 的开放权重世界模型（推断，非论文所述）；没有任何闭源商用视频模型 | 已找到 |
| 论文自述比较范围 | L1 | 同上 p.21（TriWorldBench） | “We compare against methods with publicly available papers, and report their numbers from the official leaderboard snapshot dated Sep. 30, 2026.” | `11-nuscenes-triworldbench-tables.png` | 这是论文唯一一处写明范围的地方，只限 TriWorldBench；NAVSIM 一节写 “Among the methods listed with NAVSIM Trainval training data”（p.28）；Table 11 表注 “External values are source-reported; dashes indicate unreported or pending submetrics”；Table 9 表题 “the metrics where our method ranks first or second among the compared methods”（只列小米排前二的指标，19 个子指标只列 6 个） | 已找到 |

对比方法清单（逐表）：

| 表 | 对比方法（论文原写法） | 开闭源（推断） |
|---|---|---|
| Table 1（能力对比，非跑分） | LingBotWorld、DreamXWorld、CausalWM、HorizonDrive、OmniDreams、X-World、Ours(1.0) | 均学术/公司论文模型 |
| Table 4/5（PAI-Bench-G） | Wan2.2-5B、Wan2.1-14B、Wan2.2-14B、DreamXWorld、HunyuanVideo1.5、LingBotWorld | 均开放权重（Wan、Hunyuan 公开发布；LingBot、DreamXWorld 未核） |
| Table 6（深度） | Cosmos-4B、Cosmos-12B、Copy Last (VGGT)、Aether、DINO-Foresight、VGGT-World | 学术/开放权重 |
| Table 7（相机控制） | CogVideoX-I2V、Hunyuan 1.5、LingBot World、FantasyWorld-1.0、DreamXWorld | 学术/开放权重 |
| Table 8（nuScenes） | Vista、DriveDreamer、MagicDrive-V2；Self-Forcing、Self-Forcing++、LongLive、Causal Forcing++ | 学术模型 |
| Table 9（TriWorldBench） | DreamDojo、Fysiverse-Video、WoVR_Plus、BWM、CausalWM | 排行榜上“有公开论文”者 |
| Table 10（闭环训练） | DrivoR-E2E、ST-JEPA-E2E | ST-JEPA 引用为 “Anonymous. … Submitted to The Fifteenth International Conference on Learning Representations, 2026. under review.” |
| Table 11（NAVSIM） | Epona、DriveLaW、DriveVLA-W0、DriveFine、R2SE、iPad、SparseDriveV2、Centaur、DriveSuprim、DA-WAM、DrivoR、Drive-JEPA、ChainFlow-VLA、FPD-Drive、DriveZero；另有小米自己的 Base | 均 Trainval 103k 训练 |
| Table 12（LIBERO Plus） | Fast-WAM、Fast-WAM-Joint、4D-WAM、GAM | **GAM 在正文与参考文献中无引用**（grep `GAM` 仅命中表格一行） |
| Table 13/14（重建） | EmerNeRF、3DGS、PVG、DeformableGS、LGM、GS-LRM、Depth Anything 3、AnySplat、STORM、DGGT、WorldRec 1.0 | WorldRec 1.0 是小米自己的前代 |

另：小米模型从 Wan2.2-5B 初始化——论文 3.1 节 “Our Causal World Model adopts the architecture, weights, and VAE encoder of Wan2.2-5B (Wan et al., 2025)”；Table 4/5 里 Wan2.2-5B 是 76.1/74.3，Ours 是 78.0/77.1（同一基准、小米加料后的差）。

### A5 LIBERO Plus 与碰撞率及数字原句

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| LIBERO Plus 75.0%，GAM 85.5，4D-WAM 71.0 | L1 | https://arxiv.org/pdf/2611.01677 p.29 Table 12 | 表题 “Table 12 LIBERO Plus Benchmark results (% success rate). Avg: weighted average.” | `04-libero-plus-table12.png`、`12-project-page-libero-plus.png` | 数值全部属实（见下表）。论文正文只写 “It surpasses Fast-WAM … and Fast-WAM-Joint …”，未提 GAM、4D-WAM；GAM 与 4D-WAM 的 “Embodied PT.” 是 Yes，小米是 No | 已找到 |
| 项目页的 LIBERO 摘要 | L1 | 项目页 Results → World Action Model | “75.0% weighted average. GAM reports the highest average at 85.5%.” | 12 | 项目页表只有 Avg./Layout/Camera/Robot/Language/Light 六列；论文表的 **Background、Sensor 两列项目页与官方稿都没放** | 已找到 |
| 官方稿的 LIBERO 句 | L1 | 小米技术头条文章 | “在LIBERO Plus机器人策略评测中，项目报告的加权平均成功率为 75.0%。其中，机器人相关任务成功率为74.7%，语言相关任务为95.3%，光照相关任务为97.5%。” | — | 官方稿引的三列（Robot 74.7、Language 95.3、Light 97.5）恰好都是小米在该列最高的列；没提 GAM 85.5，也没提 Camera 39.6、Background 71.0、Sensor 71.4 | 已找到 |
| 碰撞率 0.4% 对 1.2% | L1 | https://arxiv.org/pdf/2611.01677 p.25 Table 10 | 表题 “Table 10 Test Results of Closed-Loop Training Strategy and End-to-End Baseline Methods. CR denotes collision rate, SCR denotes success rate …” | `05-collision-table10.png` | 逐行：DrivoR-E2E CR 1.4 / SCR 80.3 / OR 0.0 / TTC 0.504 / EP 81.0 / HDR 99.96；ST-JEPA-E2E 1.2 / 95.2 / 0.0 / 0.614 / **84.7（EP 最高）** / 99.93；E2E-WM-RL (Ours) **0.4 / 96.3 / 0.0 / 0.740** / 84.5 / **99.99**。小米方案 = 在小米世界模型仿真器里用 PPO 对基线做在线 RL 后训练的策略（论文 5.3.1），推断：“小米 vs 对比方案”可能实为“后训练后 vs 后训练前（的同类基线）”（论文同时引了 DrivoR 与 ST-JEPA 两个基线，未写明小米方案究竟基于哪一个）；“Based on a state-of-the-art end-to-end driving policy (Kirby et al., 2026; Anonymous, 2026)”，基线选择未写清楚 | 已找到 |
| 碰撞率测试条件 | L1 | 同上 p.26 | “During the closed-loop testing phase, 1000 representative scenes are selected as evaluation cases.”；“we adopt the causal inference world model to construct closed-loop simulation training scenarios using the training and validation scene data provided by NavSim, and build closed-loop simulation test scenarios based on the test-set scenes from NavSim.” | — | 训练与测试用同一个（小米自建）仿真器，场景来自 NAVSIM 的 trainval/test 划分；论文未报告重复次数、置信区间；无实车数据。推断：若 CR 为场景占比，0.4% = 约 4/1000，1.2% = 约 12/1000（论文未写分子分母） | 已找到（口径）；4/12 个为推断 |
| 官方稿对碰撞率的措辞 | L1 | 小米技术头条文章 | “在1,000个闭环仿真场景中，E2E-WM-RL方案的碰撞率为 0.4%，相比ST-JEPA-E2E的1.2%大大降低；场景完成率达到 96.3%，高于对比方案的95.2%，取得更优结果。” | — | 官方稿写了“闭环仿真场景”，没写“仿真器就是小米世界模型”，也没提 EP 84.5 低于 84.7 | 已找到 |
| 论文对该表的自评 | L1 | 同上 p.26–27 | “…delivers substantial improvements in collision rate, scenario completion rate, and TTC … while retaining competitive performance on the driving efficiency metric EP.” | — | 论文自己也只说 EP “competitive” | 已找到 |
| “20 万 clips” | L1 | 小米技术头条文章与短帖 | 文章：“合成数据生成：已交付超过20万 clips 高质量合成数据，直接用于感知模型/感知基模训练，提升车辆在危险场景下的识别能力。”；短帖：“交付超20万clips高质量合成数据” | 08 | 论文与项目页都没有这个数字（grep “200,000/200K/20万” 0 命中）；只出现在官方发布稿/短帖，没有质量或使用效果数据；“clip”长度论文自己的预处理为 93 帧/16Hz ≈ 5.8 s | 已找到（官方稿）；论文中未找到 |
| “1260 万段视频” | L1 | 同上；论文 2.5 节 | 论文：“our training corpus contains 12.6M training clips in total”；官方稿：“训练语料规模达到 1,260万段视频片段” | — | 是 clips（预处理后的片段）不是原始视频条数。Table 2 分项：General Full 3.6M（HQ 0.5M 是其子集）、Driving 5.4M、Robot 3.6M；另有 Engine 37K、LVM 29K、Pub. Media 12K 合计约 78K。3.6+5.4+3.6=12.6M，与再加 78K 的口径差 0.08M，论文未交代是否含 78K（推断，小） | 已找到 |
| “1–7 路摄像头” | L1 | 同上 | 官方稿：“模型支持 1—7路摄像头视角”；论文 Table 1 “1–7V” | — | 论文 Table 2/4.1：Mid/Post-train “1-7V”；Post-train 按场景分别出权重（“scenario-specific weights (e.g., autonomous-driving 7V and embodied 3V)”，4.1 节）——即“共用一个架构和预训练底座”，部署权重按场景分开 | 已找到 |

LIBERO Plus 表（Table 12）逐列，最高者标 ★（粗体，PDF 第 29 页）：

| 方法 | Embodied PT. | Layout | Camera | Robot | Language | Light | Background | Sensor | Avg |
|---|---|---|---|---|---|---|---|---|---|
| Fast-WAM | No | 61.4 | 16.5 | 44.5 | 67.0 | 79.9 | 52.6 | 38.2 | 49.9 |
| Fast-WAM-Joint | No | 74.8 | 27.9 | 60.9 | 89.3 | 86.7 | 49.3 | 51.1 | 62.2 |
| 4D-WAM | Yes | 79.2（次佳） | 45.2（次佳） | 64.3 | 90.6（次佳） | 94.3 | 57.7 | 69.1 | 71.0 |
| GAM | Yes | 79.1 | **83.1★** | 70.0（次佳） | 84.8 | 97.2（次佳） | **94.3★** | **95.3★** | **85.5★** |
| **Ours** | No | **81.5★** | 39.6 | **74.7★** | **95.3★** | **97.5★** | 71.0（次佳） | 71.4（次佳） | 75.0（次佳） |

NAVSIM（Table 11）：Base + causal world model PDMS 95.3；论文原句（p.28）“Among the methods listed with NAVSIM Trainval training data, Base + causal world model achieves the highest PDMS of 95.3, exceeding the next-best reported Trainval results of 94.8. This result establishes the best leaderboard performance in the Trainval comparison setting on NAVSIM v1.1 navtest.” 同表小米自己的 **Base 为 94.9**（高于 94.8），小米方案相对 Base 的增益是 +0.4（NC 不变 99.1，TTC 97.0→96.6 下降）；NC 最高 Centaur 99.5、TTC 最高 Centaur 98.0、DAC 最高 DriveZero 99.2，小米只在 EP（93.6）与 PDMS 最高（截图 `10-navsim-table11.png`）。无多次运行方差。

### A6 因果性证据

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径 | 状态 |
|---|---|---|---|---|---|---|
| “因果”的定义 | L1 | https://arxiv.org/pdf/2611.01677 p.3、p.4、式 (1)、p.14 | 目标：“Goal 1. Model and generate the causal evolution in the real world.”；式 (1) `u_θ(x_{1:T} | …) = ∏ u_θ(x_i | x_{1:i-1}, w_{1:i}, p_{1:i}, I_0, c)`；“we apply a causal attention mask, so that x_i can attend only to the historical chunks x_{1:i-1}” | — | 论文里“causal”至少三个含义：(1) 自回归/块间因果注意力（时间方向）；(2) “physical causal data”（模拟器、视频生成器、公开媒体做的碰撞、落石等稀有事件数据，共约 78K 条）；(3) 闭环里的“causal reasoning world model”措辞。没有给出因果性（干预/反事实）的形式定义或评测 | 已找到 |
| 因果的定量证据 | L1 | 同上 5.1.1 节 p.18 | “Fig. 7 presents two physically causal events designed to examine whether the model can preserve real-world physical consistency and cause-and-effect relationships during scene evolution.” | `06-causal-qualitative-fig7.png` | 只有两个定性例子（Environmental Hazard Response 落石避让、Vehicle Collision Dynamics 追尾），每例 9 秒、挑 3 个视角、6 帧；没有任何定量“因果指标”；论文无 Limitations/Conclusion 小节（grep 无）。另一处相关数字：5.2.3 节“Pearson correlation is r = −0.88 for hazardous lane change and r = −0.84 for late deceleration”，是六个策略版本在仿真与实车测试排序的相关，不是因果指标（符号为负的原因论文未解释） | 已找到 |
| 项目页/官方稿的“因果” | L1 | 小米技术头条文章 | “……为辅助驾驶和具身智能统一因果世界模型带来新的范式。”；“世界模型2.0的核心是统一因果DiT架构。” | — | 官方稿的“因果 DiT”指前一条的 (1)，与“统一因果世界模型范式”的字面含义并不一样 | 已找到 |

### A7 夸大实例与热度

夸大/放大的中文实例见 A2 表；补充两处“从表到稿”的放大（均有原文）：

| 说法 | 级 | 来源 | 原句（逐字） | 与论文的差异 |
|---|---|---|---|---|
| “WorldRec 2.0 … PSNR 25.81” | L1 官方稿 / L4 CNMO | 官方稿：“WorldRec2.0 在公开的 Waymo 数据集上，全面领先此前 SOTA 方法 WorldRec1.0与DGGT，PSNR（衡量重建精度的核心指标，越高越好）达到25.81。”；CNMO：“WorldRec 2.0在公开的Waymo数据集上全面领先此前方法，PSNR达到25.81。”（`a-cn-cnmo.txt`，2026-11-28 12:33） | 论文 Table 13：25.81 是 **Dyn. PSNR**（动态物体区域）；整图 Full PSNR 是 29.88；“此前 SOTA 方法”含小米自己的 WorldRec 1.0（25.01/28.48） |
| “感知理解任务上……最佳” | L1 官方稿 | 官方稿：“在感知理解任务上，世界模型2.0同样获得对比方法中最佳的指标结果。” | 论文 5.1.2：深度预测（Table 6）确有对比且最优（AbsRel 0.058 对 VGGT-World 0.065；δ1 96.3 对 94.0）；**指代分割（RefSeg）只报小米自己的 IoU（t 时刻 0.6726，t+30 衰减到 0.1705），没有任何对比方法** |
| 相机控制（Table 7） | — | — | 论文 Table 7：Ours 86.35 / 70.08 / 平均 78.22，对 FantasyWorld-1.0 的 81.45 / 44.28 / 62.87；两个评测集都最高，**属实**（这是“全面领先”里最扎实的一项） |

其他列（逐列核对，非官方稿的主推项）：

- nuScenes（Table 8，`11-nuscenes-triworldbench-tables.png`）：短视频 Ours 1 路 FID 12.09 / FVD 135.13，Vista 6.90 / 89.40 更好，MagicDrive-V2 FVD 94.84 更好；6 路 19.62 / 171.73。长视频 Ours 1 路 FID 25.57（Causal Forcing++ 25.05 更好）、FVD 125.74（最佳，Causal Forcing++ 126.86）；6 路 58.90 / 203.76。论文原句 “our model achieves competitive performance among short-video generation methods … with the best FVD and the second-best FID”。**项目页的长视频表漏掉了 Self-Forcing++（28.84 / 147.57）**。
- TriWorldBench（Table 9）：TWB-Score Ours 64.79，低于 CausalWM 66.04、BWM 65.54、WoVR_Plus 65.39，排 6 个里第 4；Traj. Acc. Ours 47.25 最高；论文原句 “our approach achieves competitive overall performance without any additional tricks or test-time heuristics, and attains the best Trajectory Accuracy among all compared methods.” 项目页（Embodied generation 面板）只列 4 行（DreamDojo、WoVR Plus、CausalWM、Ours），**漏掉 BWM 65.54 与 Fysiverse-Video 63.62**，副标题 “TWB score; ours leads trajectory accuracy.”（`a-project.txt`）。官方稿未提 TriWorldBench。
- 效率（Table 3）：GLA 注意力层 8.412 ms 对全 softmax 60.14 ms（7.15×）；FID 37.24（VTA）→ 38.08（GLA），+2.3%；单卡 H20、7 视角、100 个输出 latent 帧、蒸馏后两步；吞吐 1.148 latent frames/s。论文 Table 1 对 Ours(2.0) 的 Real-time 标 ✓；3.4 节自己写 “it still cannot meet the requirements of real-time streaming inference”（指 Post-train 之前），4.2 节部署在 8 卡并行（7 卡分视角 + 1 卡 VAE 编码）。社区分析把 1.148 latent 帧/秒按 4 倍时间下采样折成约 4.6 帧/秒（`a-cn-smzdm-gta.txt`，L5，推断，论文未写 4 倍，也未给“实时”的帧率阈值）。

热度（`barking heat` 与补充）：

| 平台 | 结果 | 存档 |
|---|---|---|
| Hacker News | 无：`barking heat --hn "Xiaomi Auto World Model"` 命中 0 条，但该命令只回看到 2026-11-21（论文 11-01 提交，窗口没覆盖）；所以另用 Algolia 查 “Xiaomi world model”“xiaomiautoworldmodel”“2611.01677”“Xiaomi Auto World Model 2.0”（`created_at_i>1793000000` = 2026-10-26 起，覆盖论文提交以来）均无相关故事（唯一一条是无关旧评论） | `a-heat.json`、`a-hn-algolia.json` |
| Reddit | `barking heat` 的 Reddit 搜索被重定向到登录页失败；改用 opencli 浏览器查公开页：r/SelfDrivingCars “Introducing Xiaomi Auto World Model 2.0”，11 分 / 3 评论，发布 2026-11-27 18:37:13 UTC（= 11-28 02:37 北京），链接指向项目页；3 条评论为 12.6M clips 与 Tesla/Waymo 数据量的闲聊（得分 3、5、1），无对基准或口径的质疑；r/WeiboTech 转载帖 1 票 / 0 评论；r/neuralworlds 一帖未取分数 | `a-reddit-1y70l0u.json` |
| 微博 | 小米技术短帖 119 转 / 155 评 / 1121 赞；雷军帖 52 转 / 181 评 / 1274 赞；文章阅读数“26万+”（23:3x 读，北京）。文章评论区有网友问“上车吗”“算力”“是否和 Tesla FSD v15 类似”，均为个人意见，不作证据 | `a-weibo-*.json`、`a-weibo-article.txt` |

结论：社区层面没有任何高分英文评论对口径提出质疑；质疑线索全部来自本人对论文表格的核对（L1 原文），不是社区。

## 对读者（A8；主执行方起草“对你”句只用本节）

| 事实 | 级 | 来源 | 状态 |
|---|---|---|---|
| 论文、项目页、官方稿均未给出代码、模型权重、数据的公开链接。项目页外链只有 `arxiv.org/pdf/2611.01677` 与前代 `jointwm.github.io`；论文全文 grep release/GitHub/weights 无“将开源/已开源”的表述（GitHub 链接仅出现在引用 NVIDIA flashdreams、TriWorldBench 的参考文献里）；训练数据含自采 MiData，论文未说是否公开 | L1（缺失性结论，由本次存档核对） | `a-project.html`、`a-arxiv-pdf.txt` | 已找到（未开源，非“官方声明不开源”；官方未表态） |
| 已落地场景是“合成数据生成”和“仿真测试”：“合成数据生成：已交付超过20万 clips 高质量合成数据，直接用于感知模型/感知基模训练，提升车辆在危险场景下的识别能力。仿真测试：构建闭环仿真环境，优化测试效率，完备测试规范，可在仿真中复现真实事故并进行定向优化。” | L1 | 小米技术头条文章 | 已找到 |
| 官方发布稿全文不含“HAD”“车端”“量产”（`grep -c` 0）。论文 4.2 节部署是服务端推理（8 卡并行）；5.2 节 “system-level deployment of our model server … consumed by Xiaomi’s autonomous driving stack for closed-loop testing” | L1 | 小米技术头条文章；论文 | 已找到 |
| 同日（2026-11-27）小米汽车官方微博：“小米HAD 最新版本 Xiaomi HyperOS 1.20，引入「因果链」和「车端世界模型」， 像人一样思考，处理更准确。 能记忆过去20秒，预测未来6秒，更“聪明”地判断路况； 绕行临停更高效，风险预测更精准，更可靠地应对问题。” | L1（小米汽车官方账号，发布 2026-11-27 09:36:24 北京） | https://weibo.com/7871239944/RthNwxnFj（`a-weibo-5369799307955797.json`） | 已找到 |
| 同场推送范围：“小米澎程全系即日开启全量推送小米HAD最新版本；第一代SU7 Pro/Max、 SU7 Ultra，新一代SU7全系/YU7全系将陆续推送，以官方通知为准。”（新浪科技微博 2026-11-27 09:24:59 北京） | L4（转述广州车展发布会；官方账号上述帖未含推送范围，范围的 L1 原帖未取到） | https://weibo.com/1642634100/RthITs4Aq（`a-weibo-5369796436690818.json`） | 部分支持 |
| 车端“因果链回看 20 秒、车端世界模型预测 6 秒”与云端 2.0 的继承关系：官方发布稿与论文都没有说明。第三方解读（“蒸馏上车”）只是评论者推断 | L5（什么值得买转述，标 “42号车库”） | `a-cn-smzdm-gta.txt` | 未找到一手来源 |
| 普通车主：2.0 本体没有面向车主的开关或设置；不受影响（官方称落地场景在研发链路）。HAD 1.20 是否可用、何时推送以官方通知为准 | L1（落地场景）+ L4（HAD 推送） | 同上 | 已找到 |

---

## 2. 可能的吠点（说法 vs 证据，来源为论文原文，L1，除特别标注）

1. **说法**：“在多个主流基准测试中全面取得SOTA”（官方稿，11-27 17:00）。**证据**：论文自评在 nuScenes 与 TriWorldBench 用 “competitive”；TriWorldBench TWB-Score 小米 64.79，低于 CausalWM 66.04、BWM 65.54、WoVR_Plus 65.39（6 个里第 4）；nuScenes 短视频 FID/FVD 小米 12.09/135.13，Vista 6.90/89.40（Table 8、9，`11-…tables.png`）。
2. **说法**：PAI-Bench-G 驾驶 78.0、机器人 77.1“均为项目主页所列对比方法中的最高分”。**证据**：这是 8 个子指标的未加权平均（推断，已核算）；驾驶 Aesthetic Quality 小米 46.6（7 个模型里最低，最高 48.8）、Imaging Quality 64.5（倒数第二，最高 69.1）；机器人 Aesthetic Quality 44.8（最低，最高 51.0）；机器人总分领先 LingBotWorld 仅 0.1（77.1 对 77.0），论文无重复运行或置信区间；项目页对比方法比论文表少两个（Wan2.1-14B 77.2、DreamXWorld 76.1，都低于小米）。
3. **说法**：PAI-Bench-G 的 78.0/77.1 作为“世界模型 2.0”的成绩。**证据**：评测的是 Mid-train 模型，不是蒸馏后的 Post-train 模型（论文 5.1 节原句）；闭环仿真与部署用蒸馏后的版本；论文没给蒸馏模型的 PAI-Bench-G 分数。
4. **说法**：碰撞率“0.4%，相比 ST-JEPA-E2E 的 1.2% 大大降低”。**证据**：1000 个 NAVSIM 测试场景，在小米自建的世界模型仿真器里测，训练也用同一仿真器；对比的 ST-JEPA-E2E 引用为 “Anonymous … under review”；Ego Progress 小米 84.5 < ST-JEPA 84.7；没有实车或公开道路数据；论文未给重复次数或区间（Table 10）。
5. **说法**：LIBERO Plus 75.0%（官方稿只列 Robot 74.7、Language 95.3、Light 97.5）。**证据**：同表 GAM 85.5、4D-WAM 71.0；小米 Camera 39.6（GAM 83.1）、Background 71.0（GAM 94.3）、Sensor 71.4（GAM 95.3）；GAM 无引用；GAM、4D-WAM 做过 embodied 预训练，小米没有（Table 12）。项目页把 Background、Sensor 两列去掉，保留 GAM 85.5 的副标题。
6. **说法**：NAVSIM “best leaderboard performance in the Trainval comparison setting”，PDMS 95.3 对“next-best 94.8”。**证据**：小米自己的 Base 是 94.9；加世界模型条件的增益是 +0.4（94.9→95.3）；NC、TTC、DAC 单列都不是最高（Centaur 99.5/98.0，DriveZero 99.2）；“Trainval comparison setting”是论文自己圈出的比较集合；无方差（Table 11）。
7. **说法**：“汽车与机器人共用一套大脑”（扫描措辞；官方稿写“同一套主干网络可适配不同视角配置和控制任务”）。**证据**：Post-train 按场景分别出权重（“scenario-specific weights (e.g., autonomous-driving 7V and embodied 3V)”），共用的是架构与预训练底座。
8. **说法**：“统一因果世界模型”。**证据**：无因果性度量；定性两例（Fig. 7）；“causal”主要指块间因果注意力自回归与“物理因果数据”（约 78K 条）；论文没有 Limitations 小节。
9. **说法**：Table 1 对 Ours(2.0) 标 Real-time ✓、官方稿称“流式生成”。**证据**：Table 3：单卡 H20、7 视角、蒸馏后两步 1.148 latent frames/s；部署用 8 卡并行；论文 3.4 节写 Post-train 之前 “still cannot meet the requirements of real-time streaming inference”；论文未给“实时”的帧率阈值（社区折算 ≈4.6 fps 属推断，L5）。
10. **说法**：“WorldRec2.0……PSNR达到25.81 ……全面领先此前 SOTA 方法”。**证据**：25.81 是动态区域 PSNR；全图 29.88；“此前 SOTA”包含小米自己的 WorldRec 1.0（Table 13）。
11. **说法**：“在感知理解任务上……最佳”。**证据**：只有深度预测有对比（Table 6）；指代分割只报小米自己的数字（5.1.2 节），在 t+30 降到 IoU 0.1705。
12. **项目页对论文表的删减**（项目页与论文对照，均存档）：PAI-Bench-G 只显示综合分，不显示 AQ、IQ 等子列，且少 Wan2.1-14B、DreamXWorld 两个对比模型；LIBERO Plus 少 Background、Sensor 两列；TriWorldBench 少 BWM、Fysiverse-Video 两行；nuScenes 长视频少 Self-Forcing++ 一行。核实方法：在 11-28 的项目页上，对英文与中文两种界面的 `document.documentElement.innerHTML` 逐一搜索 Wan2.1-14B、BWM、Fysiverse、Self-Forcing++、Background、Sensor，全部 0 命中（DreamXWorld 在项目页只出现于能力对比表和相机控制图，不在 PAI-Bench-G 图里）；并存档并搜索了 `site-data.js`、`i18n-zh.js`、`results-charts.js`（`a-project-*.bin`），同样 0 命中。
13. 社区线索（L6，未用作结论）：smzdm 转述知乎分析者黄浴称“论文没有报告真机实验”（`a-cn-smzdm.txt`）——我独立核对：论文确实没有真机/实车实验结果，只有“实车测试排序与仿真排序的 Pearson 相关”（5.2.3 节，6 个策略版本，r=-0.88/-0.84）。

## 3. 夸大说法实例（原站 URL、原句逐字、时间、发布方）

| 实例 | URL | 原句 | 发布时间（北京） | 发布方 | 性质 |
|---|---|---|---|---|---|
| 官方原句 | https://weibo.com/ttarticle/p/show?id=2309405369910959407174 | “在多个主流基准测试中全面取得SOTA” | 2026-11-27 17:00 | 小米技术（官方） | 原话，非转述 |
| IT之家 | https://www.ithome.com/1/033/614.htm | 标题“小米汽车世界模型 2.0 发布，已在实际业务场景中落地”；“在多个主流基准测试中全面取得 SOTA” | 2026-11-27 17:13:11 | IT之家 | 转述官方，保留“全面” |
| 环球网（新浪财经） | https://finance.sina.com.cn/roll/2026-11-28/doc-inkahkve7331920.shtml | “并在多个主流基准测试中取得 SOTA”；“面向辅助驾驶与具身智能构建统一因果世界模型范式” | 2026-11-28 13:03 | 环球网 | 去掉“全面”，保留统一因果 |
| 搜狐号“车迷老司机” | https://www.sohu.com/a/1107623848_122066676 | 标题“小米汽车世界模型2.0发布：颠覆辅助驾驶与智能决策的未来！”；“使得模型能够在辅助驾驶、机器人场景生成、闭环仿真及World Action Model等多个领域取得SOTA（State of the Art）成绩” | 2026-11-28 14:17 | 搜狐自媒体 | 把“多项结果、多个基准”扩成“多个领域取得 SOTA”；标题“颠覆”；正文写成“2025年11月28日” |
| 新浪财闻微博 | https://www.sina.cn/weibo/detail/5370245171646940.html | “在多个主流基准测试中全面取得SOTA” | 2026-11-28 12:43 | 财闻官方微博 | 原样转述 |
| CNMO | https://smartcar.cnmo.com/news/823484.html | 标题“雷军：小米汽车世界模型2.0正式发布 多项能力明显提升”；“碰撞率仅0.4%，远低于对比方案的1.2%”；“WorldRec 2.0在公开的Waymo数据集上全面领先此前方法，PSNR达到25.81” | 2026-11-28 12:33 | CNMO 科技 | 未带“仿真”“动态区域 PSNR”限定 |
| 前代同款标题（对照） | https://news.10jqka.com.cn/20260526/c676971274.shtml | “小米汽车发布世界模型全新框架：重建 + 生成一体化，主流基准测试全面 SOTA” | 2026-05-26 11:43:03 | 同花顺 | 1.0 发布时已用同一措辞（L5，仅对照） |

英文媒体：本次未单独存档（Hacker News 无讨论，Reddit 仅 11 分）。

## 4. 扫描说法勘误

| 扫描说法 | 核对结果 |
|---|---|
| 论文 `arXiv 2611.01677`、“11-01 15:53 提交”、11-27 发布 | 编号正确；v1 提交为 2026-11-01 07:53:48 UTC = 15:53:48 北京时间，仅 v1。论文 11-01 提交（arXiv 公开页的上线时刻比提交晚，未核），11-27 是官方发布稿（17:00）。扫描未说清这一点易被读成“11-27 才公开论文” |
| 项目页 `https://xiaomiautoworldmodel.github.io/CausalWorldModel/` | 正确；服务器 Last-Modified 为 2026-11-03 11:57:16 北京时间 |
| 1–7 路摄像头；训练语料 1260 万段视频 | 官方稿写“1,260万段视频片段”，论文写 “12.6M training clips”；是 clips |
| “已交付 20 万 clips 合成数据” | 官方稿与短帖有，论文与项目页没有 |
| “11-28 雷军发文” | 属实（微博 2026-11-28 12:25:02 +0800），帖内无 SOTA、无具体数字 |
| 驾驶 AV-Quality 78.0 对 Hunyuan 77.4 | 属实 |
| “Aesthetic Quality 46.6（Hunyuan 48.8）” | 属实 |
| “Imaging Quality 64.5（Hunyuan 69.1）” | **有误**：Hunyuan 69.0，69.1 是 LingBotWorld（该列最高）；小米 64.5 属实 |
| 机器人 77.1 对 LingBot World 77.0 | 属实 |
| 对比方法为开源模型，无闭源模型 | 无 Veo/Sora/Seedance/Kling 等闭源模型（论文全文 grep 零命中）；论文未标注开闭源；Cosmos 只在深度表出现；论文自己只在 TriWorldBench 一节写明范围（“methods with publicly available papers”） |
| LIBERO Plus 75.0%，GAM 85.5，4D-WAM 71.0 | 属实；表有 7 个子列，官方稿/项目页展示的列不全 |
| “碰撞率 0.4% 对 1.2%” | 属实，对比对象 ST-JEPA-E2E；另有 DrivoR-E2E 1.4%；仿真器内、1000 个 NAVSIM 测试场景 |
| “汽车与机器人共用一套大脑” | 共用架构/预训练底座；部署权重按场景分开（4.1 节） |
| “‘因果’只有两个定性案例（追尾、落石避让），无因果性度量” | 属实 |
| 官方“全面 SOTA”原话 | 官方头条文章原句见 A2；论文摘要/正文无此自称 |

## 5. 还没核实、需要主执行方知道的限制

- HAD 1.20 的推送范围（澎程全系、SU7/YU7 陆续）我只取到新浪科技转述（L4）；小米汽车官方账号的原帖（09:36:24）只含功能描述（已取到，L1）。
- 论文 PDF 第 3 页 Table 1 未截图（它是能力对比，不是跑分）；Table 3（效率）、Table 6（深度）、Table 7（相机控制）、Table 13/14（重建）未截图，数值已在上文逐字抄录，需要时可用 `a-arxiv-pdf.pdf` 第 15、19、20、31、32 页补截。
- “开闭源”列是推断，论文没标；若正文要写“只和开源模型比”，须逐个模型核对许可证，否则写成“论文对比的模型里没有 Veo、Sora、Seedance 等闭源商用视频模型”。
- 仿真器内的 0.4% 没有分子分母；“约 4/1000”是推断。
