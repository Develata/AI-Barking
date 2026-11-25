# 1124 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B、C 组均为 Sonnet 5.5（`claude-sonnet-5-5`，effort high），用时（harness）A 44 分钟、B 21 分钟、C 21 分钟；Claude 抽查（11-26 02:25–02:45）逐字对过下表标“抽查”的行。引号内中文稿原句为方便排版在中英文之间补了空格，未改字。

## 一、MAGI-2（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：多家媒体称视频AI全球第一，换榜排第7 | L4 / L3 | ✅ | 说法：新浪财经稿题“AA榜全球第一”（`a-cn-sina.txt`）；投资界、网易科技、上证报等首句“在 AA 榜单中登顶全球第一”（`evidence-A.md` A6）。实际：AA 文生视频带音频榜（文生页默认视图）MAGI-2 第7、Elo 1150（`a-aa-text-to-video-audio.table.txt`，02:03–02:05 读） | “视频AI”指 MAGI-2；第7 与第8、9名同为 1150，Range 4–11；AA 标“Provisional result”。交付前须重读四张榜 |
| 省流：视频模型 MAGI-2 在 AA 榜单的“图片生成视频”两张榜暂列第一，多家中文媒体写成“登顶全球第一” | L4 | ⚠️ | 新浪财经“在Artificial Analysis（AA）榜单中登顶全球第一”（`a-cn-sina.txt`，抽查）；投资界 13:18、网易科技 14:30 | 爱范儿、智东西、ZPedia 正文限定了图生视频；通稿原件未取得。⚠️ 表示：被纠正说法的原句已核对，媒体稿不作事实来源 |
| （省流）“图片生成视频”两张榜暂列第一 | L3 | ✅ | AA I2V（带音频）第1，1163；I2V 无音频第1，1153；T2V 无音频第2，1144；T2V 带音频第7，1150（四个 `a-aa-*.table.txt`，抽查） | “图片生成视频”= Image to Video；AA 另有“视频编辑”页，未计入 |
| 11月25日凌晨，Sand.ai 发布视频模型 MAGI-2 | L1 | ✅ | 官方发布帖 created_at “Tue Nov 24 18:09:28 +0000 2026” = 北京 11-25 02:09（`a-x-magiai-hq-2125290721003680155.json`，抽查） | 官方博客标 November 24, 2026、未给时刻 |
| 第三方评测 Artificial Analysis（AA）让真人盲选视频来排名 | L3 | ✅ | AA 页：“Rankings are based on blind votes from our validated private evaluators”；页头“by Elo from pairwise human preference votes”（截图 01、02） | |
| 文字生成、图片生成视频各有带音频、不带音频两张榜 | L3 | ✅ | AA-Video-T2V / T2V-Silent / I2V / I2V-Silent v2.1；页内 With Audio / No Audio 切换 | |
| 11月26日凌晨，MAGI-2 在图片生成视频两张榜第一，文字生成视频不带音频榜第二、带音频榜第七 | L3 | ✅ | 同第3行；02:03–02:05 读，02:27 复读 I2V 未变 | 扫描（11-25 18:02）读到 I2V 1165 对 1161，属漂移 |
| AA 标注它刚上榜，名次会变 | L3 | ✅ | AA 页 MAGI-2 旁提示“Provisional result: this model was added to the leaderboard recently. Score and rank may change as we collect and verify more votes.”（`evidence-A.md` A1；`a-aa-image-to-video-audio.dom.html` 含“Provisional result: this model was added to the leaderboard recently.”，Claude grep） | |
| 对你：官网列出 VidMuse 可试用，发布帖写了 Higgsfield | L1 | ✅ | sand.ai 首页“Try on VidMuse [https://vidmuse.ai/en/magi-2] Official Partner”（`a-sand-home.txt`，抽查）；官方发布帖“exclusively on @higgsfield”“Now available worldwide on our launch partner Higgsfield’s platform and via the Higgsfield API.”（抽查） | 两处官方口径不同（发布帖写 exclusively），正文并列两处入口、不写“只能”；反向核验第一轮 BLOCKER 1。实际注册与生成未试。17173 转述个人可经 LibTV 体验（L4/L5，未进正文） |
| 官方接口 1080p 每秒0.08美元，暂不支持充值 | L1 | ✅ | `a-sand-platform-model-pricing.txt`：“0.08 USD/s”“Top-up is not supported for the time being”（抽查） | 1080p；AA 页标 $4.80/min，一致 |
| 吠点①：新浪财经稿题写“AA 榜全球第一” | L4 | ⚠️ | `a-cn-sina.txt` 标题“Sand.ai发布MAGI-2：AA榜全球第一，价格仅为Seedance2.5 1/10”（抽查） | 原句已核对；补空格 |
| 它在文字生成视频带音频榜暂列第7 | L3 | ✅ | `a-aa-text-to-video-audio.table.txt`“7 \| 4-11 \| MAGI-2 \| 1150 ±13”（抽查）；取消“隐藏研究预览”仍第7 | 同分并列见标题行 |
| 吠点②：图片生成视频带音频榜上，MAGI-2 1163分，第二名 Seedance-2.5 1158分 | L3 | ✅ | 截图 01：MAGI-2 1163 ±14，Dreamina Seedance 2.5 1158 ±14（抽查） | |
| 误差都是正负14分 | L3 | ✅ | 同上，两者均“±14”，页面释义“95% interval” | |
| AA 页名次区间都是1到3 | L3 | ✅ | 截图 01 Range 列两者均“1–3” | 正文只照录 Range 列数值，不解释其概率含义：Range 定义只见 A 组转录，提示原文未存档（反向核验两轮 NEEDS_VERIFICATION） |

未进正文（核过，备用）：价格“仅为Seedance2.5 1/10”——人民币“0.5元/秒”只见媒体稿，火山方舟 Seedance 2.5 1080p、输入不含视频、16:9、5 秒“折合价格（元/秒）3.74”（`a-volc-ark-pricing.txt`）；参数 480B（博客）与 400B（官方帖、Higgsfield）不一致；权重未公开（HF、GitHub 官方组织 11-26 02:27 只有 Preview 114B），官方“We will follow up soon with open-source releases”；Higgsfield 自家盲测 MAGI-2 1146 对 Seedance 2.5 1219（独家合作方）。

## 二、Copilot 论文（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 省流：一篇论文估计，因合同审查提前用上 Copilot 的大企业少招约40%初级软件工程师 | L1 | ✅ | `b-arxiv-pdf.txt` p.3“among compliers, quarterly hiring of junior software engineers falls by roughly 40%”；§6.2.3“reduces quarterly junior software engineering hiring by approximately 40% (−0.513 log points)”（截图 23，抽查） | “部分大企业”= compliers；junior = Revelio 级别 1–2 |
| 范围是少约72%到多约1% | L1 | ✅ | Table A.14 Panel C 第(3)列“[−1.262, 0.008]”，注“95% percentile firm-block bootstrap interval”（截图 18，抽查） | exp(x)−1 = −71.7%、+0.8%（本号换算，论文只给对数点） |
| 11月24日凌晨，MIT 与微软研究院的两位作者发布预印本 | L1 | ✅ | arXiv v1“Mon, 23 Nov 2026 17:33:22 UTC”= 北京 11-24 01:33；PDF 首页单位 MIT、Microsoft Research（抽查） | 同行评审状态论文未写 |
| 用 GitHub 记录配上招聘数据看5414家大企业 | L1 | ✅ | 摘要“5,414 multinational firms”；p.3“We link GitHub’s administrative records on coding assistant adoption and contract reviews to employment data from Revelio Labs, which tracks firm-level headcount, hiring, and separations”（抽查） | Revelio 数据主要来自领英资料与招聘帖（B 组） |
| 在既定的 GitHub 合同审查时买 Copilot，比另走采购省事 | L1 | ✅ | p.2：“Firms faced a lower organizational cost of adopting GitHub Copilot during these existing GitHub reviews relative to initiating a separate procurement process. Because these schedules were established before GitHub Copilot existed…”（抽查） | 反向核验第一轮改：初稿“最省事”超出原文比较 |
| 论文估计，因此提前用上的企业一年后每季度技术岗招聘比不用时少约29% | L1 | ✅ | p.19“the fourth-quarter Wald estimate is −0.340 log points, or a 29% decline in quarterly technical hiring”（截图 15，抽查）；摘要“among review-induced adopters” | “比不用时”= 相对反事实 |
| 初级软件工程师少约40% | L1 | ✅ | 同省流行 | |
| 离职没有明显增加 | L1 | ✅ | 摘要“with no effect on separations”；p.18 区间“rules out separation increases larger than roughly 12%” | |
| 对你：样本是平均8000多人的大企业 | L1 | ✅ | p.9“employs 8,284 workers globally”（抽查） | |
| 数据到2025年6月 | L1 | ✅ | “from January 2021 through June 2025”（抽查） | |
| 作者写，小公司可能反应不同 | L1 | ✅ | p.33“Smaller or younger firms facing fewer coordination constraints may reorganize production more rapidly, or respond to lower software development costs by expanding output and technical hiring more elastically.”（抽查） | |
| 吠点①：爱查 AI 稿题“Copilot 使初级工程师招聘降约40%” | L5 | ⚠️ | `b-zh-aichaai.txt`（抽查），11-25 14:30 | 原句已核对，引作被纠正说法；补空格 |
| 这只算提前用上那批企业 | L1 | ✅ | p.17“Scaling these reduced form estimates by the first stage yields the implied effect … among compliers”（截图 15） | Wald = 简约式 ÷ 第一阶段 |
| 范围少约72%到多约1% | L1 | ✅ | 同省流行 | 本号换算 |
| 论文另算的合同审查效应，是初级软件岗每季度少招约一成，误差范围不含零 | L1 | ✅ | Table A.14 Panel A 第(3)列 Junior“Contract review (Q4) −0.108**（0.052）”（截图 18，抽查） | 与 40% 同口径（季度、初级软件工程师），简约式；exp(−0.108)−1 = −10.2%（本号换算） |
| 吠点②：摘要里的29%范围是少约61%到多约9% | L1 | ✅ | p.19“95% firm-block bootstrap confidence interval of [−0.948, 0.088] log points”（截图 15，抽查） | exp(x)−1 = −61.2%、+9.2%（本号换算） |
| 更稳的是月度数：到了审查日的企业一年后，比还没到的当月少招7.6%技术岗 | L1 | ✅ | p.17“By month twelve, technical hiring is 7.6% (−0.079 log points)…”；Table 1 −0.079**（SE 0.038）（截图 15） | 月度简约式（意向处理），对象是全部到审查点的样本企业。同口径的季度简约式为 −0.073（SE 0.051）、不显著（附表 A.9，`b-arxiv-pdf.txt` 2338 行），所以正文写明“按月”，不写成“同一估计不放大就显著”（反向核验第一轮 BLOCKER 2）。“更稳”= 标准误更小、在5%水平显著 |
| 吠点③：样本企业2022到2024年技术岗招聘总共少了29% | L1 | ✅ | §6.5“mean quarterly technical hiring across sample firms fell by 29%, or 16.1 hires per firm-quarter (from 55.3 to 39.2)”（截图 25，抽查） | 与摘要 29% 是两个数 |
| 每家每季度从约55人降到约39人 | L1 | ✅ | 同上 | “人”= hires |
| 论文按三种假设，估计 Copilot 占其中10%、19%或51% | L1 | ✅ | §6.5 三种反事实：10%、19%、51%（截图 25，抽查） | 三个是条件性点估计，不是上下限（反向核验第一轮改） |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| MAGI-2：只在图生视频榜第一 | — | ✅ | 主帖一 | 主帖条目 |
| Copilot 论文：软件新人少招四成？ | — | ✅ | 主帖二 | 主帖条目 |
| Tidal 称12月9日起拒收并下架完全 AI 生成的曲目，AI 辅助的仍可上 | L1 | ✅ | Tidal 官方账号长文“Starting December 9, any wholly AI-generated track that's submitted to Tidal will be rejected and removed from our platform.”“Tracks that were assisted by AI tools, but not the sole outputs, will be allowed”（`c-c4-fxtwitter-tidal-article.json`，抽查） | 北京 11-24 22:22；HN 334 / 324 |
| OpenAI 预印本称矩阵乘法指数在任意域上恰为9/4，形式化只覆盖无除法电路 | L1 | ✅ | `c-c1-pdf.txt`：“Theorem 1.1. Over every field F, the exponent of square matrix multiplication is ωF = 9/4”（抽查）；Lean 覆盖范围见 `evidence-C.md` C1 | 仓库提交北京 11-25 08:04；HN 904 / 509；仓库自称“Some of the unformalized results could have issues” |
| 高德开源 ABot-World 小版，称单卡 RTX-5090 每秒21帧 | L1 | ✅ | `c-c6-ghreadme.txt`：“Iris (5B) streams video at 21 FPS on a single RTX 5090”；“Iris, Iris-BiT, and six interactive examples are now open”；仓库 LICENSE Apache-2.0（C 组） | 小版 = Iris 5B；30B 版 Halo 仅云端；“WBench 第一”未收 |
| 腾讯开源 Hy-MT3 翻译模型，支持语种由38种增至102种 | L1 | ✅ | `c-c9-hf-30b.txt`：“expands language coverage from 38 languages in Hy-MT2 to 102 languages”（抽查）；“2026.11.24 We open-sourced …” | Apache-2.0 |
| Mantic 称200道数值题自测，预测智能体得分约同中档 Fable-5.2 | L1 | ✅ | `c-c5-tml.txt`：“It is roughly as accurate as Fable 5.2 (medium) but 15× cheaper per forecast”（抽查） | “200 held-out numerical questions from Metaculus, Polymarket, Kalshi and our platform”，指标 mean log score；15 倍成本只有相对数，未进速览（反向核验第一轮 BLOCKER 3） |

## 批注译注

| 卡片 | 译注 | 级 | 结果 | 依据 |
|---|---|---|---|---|
| aa-i2v | 第1名 MAGI-2，1163（误差±14），名次区间1–3；第2名 Dreamina Seedance 2.5，1158（±14），名次区间1–3。 | L3 | ✅ | `a-aa-image-to-video-audio.table.txt` 第6–7行“1 \| 1-3 \| MAGI-2 \| 1163 ±14 95% interval 1149 to 1177”“2 \| 1-3 \| Dreamina Seedance 2.5 \| 1158 ±14”；截图 01 |
| aa-t2v ① | 第7名 MAGI-2，1150（误差±13），名次区间4–11。 | L3 | ✅ | `a-aa-text-to-video-audio.table.txt` 第12行 |
| aa-t2v ② | 第8名 FLUX 3 Max、第9名 HeyGen Video 1，同为1150。 | L3 | ✅ | 截图 02 第8、9行 |
| paper-29 ① | 合同审查后第12个月：月度技术岗招聘人数（对数）−0.079个对数点，在5%水平显著。 | L1 | ✅ | Table 1 第(1)列“Log”“−0.079**（0.038）”，注“** p < 0.05”；p.17“technical hiring is 7.6% (−0.079 log points)” |
| paper-29 ② | （对因合同审查而采用 Copilot 的企业）第四季度的 Wald 估计为 −0.340 个对数点，即季度技术岗招聘下降29%，95% 企业分块自助法置信区间为 [−0.948, 0.088] 个对数点。 | L1 | ✅ | p.19 原句“95% firm-block bootstrap confidence interval”（截图 15 高亮）；对象“implied effect … among compliers”（同段上句） |
| paper-junior ① | 初级软件工程师季度招聘人数（对数），简约式（合同审查后第4季度）：−0.108个对数点，在5%水平显著。 | L1 | ✅ | Table A.14 Panel A 第(3)列；表头“Log quarterly hires”；注“Panel A reports the reduced-form effect of contract review in log points”“** p < 0.05” |
| paper-junior ② | 初级软件工程师季度招聘人数（对数），Wald 估计（对因合同审查而采用 Copilot 的企业）：−0.513个对数点，95% 企业分块百分位自助法区间 [−1.262, 0.008] 个对数点。 | L1 | ✅ | Table A.14 Panel C 第(3)列；注“Panel C reports the Wald estimate in log points”“the bracketed interval is the 95% percentile firm-block bootstrap interval” |
| paper-decline ① | 2022年至2024年（2024年为数据中最后一个完整年份），样本企业平均每季度技术岗招聘下降29%，即每家每季度少16.1人（从55.3降到39.2）。 | L1 | ✅ | §6.5“Between 2022 and 2024, our last full calendar year in the data, … fell by 29%, or 16.1 hires per firm-quarter (from 55.3 to 39.2)” |
| paper-decline ② | （按简约式）每家每季度少招1.7人，占总体下降的10%。 | L1 | ✅ | §6.5 第1种“Reduced form counterfactual … 1.7 forgone hires per firm-quarter or 10%” |
| paper-decline ③ | （更强的假设：所有采用企业的招聘下降比例与变化路径都和提前采用者相同）估计每家每季度少招8.1人，占总体下降的51%。 | L1 | ✅ | §6.5 第3种“all adopters experience the same proportional hiring decline and path as compliers … 8.1 hires per firm-quarter, or 51%” |

标题栏“Raymond & Dillon 论文”：作者 Lindsey Raymond、Eleanor Wiske Dillon（arXiv abs 页，抽查）。

## 卡片日期

`cards.toml` 的 `date` 写 11月26日（发布日）。

## 反向核验与视觉复核

第一轮反向核验（Codex gpt-6-astra，effort medium，read-only，北京 11-26 02:50–03:05，`barking review --reverse --run`，重点 19 个文本文件）：3 BLOCKER、9 SHOULD_FIX、1 NEEDS_VERIFICATION。逐条处理：

1. BLOCKER“普通用户要在 Higgsfield 上用”成排他说法：采纳。sand.ai 首页另列 VidMuse（`a-sand-home.txt` 已核）。改为“可在 Sand.ai 官网列出的 VidMuse 或 Higgsfield 平台试用”。
2. BLOCKER 季度 29% 与月度 7.6% 对照，暗示“同一估计不放大就显著”：采纳（季度简约式 −0.073 不显著，已核附表 A.9）。②改写明“按月”；同口径的季度显著数（初级软件岗简约式 −0.108**）移到①，与 40% 同口径。
3. BLOCKER 速览 Mantic 丢自测范围、只有相对成本：采纳。改为“自测200道预测题”，删“便宜15倍”。
4. SHOULD_FIX 临时结果未进正文：采纳。事实段加“AA 标注它刚上榜，名次会变”，省流与吠点①用“暂列”。
5. SHOULD_FIX 5分差无绝对数：采纳，吠点②写1163对1158。
6. SHOULD_FIX 数据来源只写 GitHub、“最省事”超出原文：采纳，改“GitHub 记录配上招聘数据”“比另走采购省事”。
7. SHOULD_FIX“只能解释10%到51%”像上下限：采纳，改“按三种假设，估计占其中10%、19%或51%”，“本来就”改“总共”。
8. SHOULD_FIX 速览 Hy4-flash 条件被压缩：采纳。40 字内放不下 M4 Pro、IQ2、SSD、Strata 分支，撤下该条，换高德 ABot-World（速览仍 7 条）。
9. SHOULD_FIX 译注漏对数单位：采纳，三条译注补“招聘人数（对数）”“个对数点”。
10. SHOULD_FIX 清单缺译注节：采纳，补上节“批注译注”。
11. SHOULD_FIX `images/README.md` 缺失：送审时尚未写成，现已补。
12. SHOULD_FIX L4/L5 原句标 ✅：采纳，改 ⚠️ 并注明“原句已核对，作被纠正说法引用”。
13. NEEDS_VERIFICATION Range 提示原句：采纳，清单改为不加引号的转述并注明出处为 A 组转录。

视觉复核（Codex gpt-6-astra，`barking review --visual --run`，北京 03:08–03:15，18 张图）：1 BLOCKER、6 SHOULD_FIX，全部采纳。
1. BLOCKER 省流卡两条都带了吠点（“只在…第一”“范围却是…”）：正文省流改为先写事实（“在 AA 榜单的‘图片生成视频’两张榜暂列第一，多家中文媒体写成‘登顶全球第一’”；“一篇论文估计，因合同审查提前用上 Copilot 的大企业少招约40%…，范围是…”），省流卡只摘事实部分。
2. aa-t2v 译注“第8、9名同为1150”无对应高亮：拆成两条，第8、9行另加高亮。
3. 两条 Wald 译注未写估计对象：补“对因合同审查而采用 Copilot 的企业”。
4. 两条区间未写方法：补“企业分块自助法置信区间”“企业分块百分位自助法区间”。
5. paper-decline ① 括号指代不清：改“2024年为数据中最后一个完整年份”。
6. paper-decline ③ 假设写弱、未高亮：译注补“下降比例与变化路径都相同”，高亮扩到假设句。
7. 速览高德条无自报属性：改“高德开源 ABot-World 小版，称单卡 RTX-5090 每秒21帧”。
改后重渲 6 张，Claude 逐张目视确认 01、06、11。AA 四张榜北京 03:07 复读前五名摘要，与 02:03 一致。

第二轮反向核验（短审，Codex gpt-6-astra，北京 03:08–03:20，送审稿为视觉复核修改前的版本）：PASS，无 BLOCKER；8 SHOULD_FIX、1 NEEDS_VERIFICATION。
1. 高德测速缺自报限定：视觉复核时已改“称”。
2. “放大/不放大”混淆估计对象、7.6%缺基线：采纳。①改“这只算提前用上那批企业”“论文另算的合同审查效应”；②补“比还没到的”（p.17“relative to firms that have not yet reached their scheduled review”）。
3. 相对数缺同口径绝对量：不采纳。论文没有给初级软件岗的同口径绝对人数；技术岗整体的 55→51/39 是另一口径，不能当 40% 的基线。正文改为写清估计对象与区间。
4. 小标题“初级岗”扩大范围：采纳，改“软件新人少招四成？”（问号保留，纠正在吠点）。
5. Mantic 口径：采纳，改“200道数值题自测，预测智能体得分约同中档 Fable-5.2”。
6. “对你”入口归属与 1080p：采纳。
7. README 未同步：采纳，两份 README 已改。
8. `evidence-A.md` 第14、111行“互相包含”“统计上不可分”：采纳，取证记录改为“区间重叠”。
9. Range 定义：采纳，正文改为只照录 Range 列数值（“AA 页名次区间都是1到3”）。
改后 lint 0 错误 0 警告，重渲 5 张。
