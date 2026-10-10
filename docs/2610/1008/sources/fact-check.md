# 1008 事实核验

期次目录按 Develata 美国本地日期（10月8日，美东）。本机时区 EDT（UTC−4），北京时间 = 本机 + 12 小时。取证由 Sonnet 子代理分三组完成（A 数学撤稿与 Navier–Stokes 的 Lean 质疑、B Anthropic 使用政策、C 速览；`evidence-a.md` … `evidence-c.md`，抓取日志 `capture-log-*.md`），Claude 回读关键存档、核对截图。表中“结果”：✅ 一手页面直接核到；⚠️ 部分支持或口径有保留。

## 选题变更记录

- 线索：ChatGPT 扫描（第 11 轮，北京 10-09 04:59）、WorkBuddy 扫描（1009-06，北京 10-09 06:02），以及 Develata 转来的 Navier–Stokes 质疑论文线索（arXiv 2610.08144、OpenAI NS 论文 PDF、`openai/NavierStokesAndEuler`）。Claude 另用 HN Algolia 补扫近两天 AI 帖（北京 10-09 约 09:00）。Develata 定：主帖为数学（撤稿 + NS 的 Lean 质疑）与 Anthropic 新规，其余入速览。
- 扫描说法与一手不符、已纠正（均不进正文）：WorkBuddy 称陶哲轩“主持/牵头 AHM 声明”（陶博客只是 guest post 转载，页首注明转自 AHM 声明页；AGMAI 名单也无陶）；“AHM 752 名成员”（未找到出处，AHM 现写 809 人）；WorkBuddy 称 Anthropic 新规 10/7 发布（页面 10-08，北京 10-09 01:00）；ChatGPT 称 OpenAI 收入“此前约700亿”（CNBC 原句为 $68 billion；700 亿出自 FT 与 Axios）；ChatGPT 称“300/719 项 top-line results 形式化”（OpenAI 原句如此，但 A 组按 yaml 各口径均复算不出 300，正文不用该数）。Mastodon 评论里“KU 博士生称 OpenAI 抄袭”所附 arXiv 2610.10072 是一篇 Lubin–Tate 论文，摘要无涉，不收。
- 风控：Anthropic 公告中“地区政策”（不支持地区的实体与其控股企业）、“国家媒体虚假账号”“监控异见人士”“选举条款”各段不进正文与卡片。CrowdStrike/韩国银行、OpenAI 涉俄伊虚假记者网络不收。
- 不收：Meta 与微软削减内部 Claude 用量（原报道在 The Information 付费墙后，未读到原文）；Reddit“用 Claude Code 发现新行星”（L6，未取到原帖）；Claude Dashboards/Motion（热度低，HN 5 分），为放 AHM 让出速览位。

## 主帖一：OpenAI 撤稿与 Navier–Stokes 的 Lean 质疑

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间10月8日，OpenAI 更新数学仓库称，一篇关于阿贝尔簇 Weil 类的手稿有符号错误，连同依赖其构造的两篇（涉及 K3 曲面与霍奇猜想），共撤回3篇 | L1 | ✅ | https://github.com/openai/math/blob/main/history.md （`a-history.md`；截图 `images/01`、批注 `41`） | “In “Algebraicity of Weil classes on split abelian eightfolds” a sign error invalidates a stabilization-trace cancellation argument and the construction used by two dependent papers. As a result, we have withdrawn the following three manuscripts”；另两篇 “Algebraicity of Kuga–Satake Correspondences for K3 Surfaces”“The rational Hodge conjecture for products of K3 surfaces”。日期三种并存：history.md 小标题 “October 7, 2026”；三篇 README “Withdrawn on October 6, 2026”；提交 `3014888` 北京 10-08 13:03:50（合并 `fd4aeeb` 13:20:00）。正文取公开更新的北京时间。“阿贝尔簇 Weil 类”对应 “Weil classes on split abelian eightfolds”（分裂阿贝尔八维簇），正文用通俗说法。 |
| 另修订14篇 | L1 | ✅ | 同上 | “We have revised 14 other manuscripts with proof repairs, corrected statements, clearer hypotheses and dependencies, and one correction to an obsolete citation.” 另有 13 篇只更新对配套论文的引用版本（未入正文）。OpenAI 员工 Dan Roberts 帖称 “19 modifications”，口径不同（`evidence-a.md` A6-4），未用。 |
| 9月，OpenAI 称用 AI 解决了 Navier-Stokes 千禧年难题，附论文与 Lean 形式证明 | L1 | ✅ | https://openai.com/index/navier-stokes-solution/ （`a-oai-ns-page.md`；截图 `images/10`） | 页面日期 “September 8, 2026”，无时刻。og:description：“We’re sharing an AI-generated solution to the Navier–Stokes Millennium Prize Problem, including a writeup and a formal proof in Lean.” 论文主定理带光滑紧支撑外力，称 “This establishes alternative (C) in the Millennium problem statement for Navier–Stokes as stated by Fefferman”（`a-oai-ns-paper.txt` p.1）。用“OpenAI 称”，不替其背书。 |
| 10月6日，伦敦国王学院与剑桥大学的三名学者在 arXiv 发文，逐条对比论文与 Lean 代码 | L1（作者自述） | ✅ | https://arxiv.org/abs/2610.08144 （`a-ns-lean-critique-v1.txt`） | v1 2026-10-06 10:58:01 UTC（北京 18:58）。单位见 p.25：Bastounis 为 King’s College London 数学系；Circelli、Hansen 为剑桥 DAMTP。预印本，未经同行评审。 |
| ①撤回的三篇本来就不在形式化清单里 | L1 | ✅ | `a-formalization-fd4aeeb.yaml`、1006 存档 `../../1006/sources/m-oai-lean-formalization.yaml`；脚本 `a-yaml-diff.py` → `a-yaml-diff.out.txt` | 三个论文目录名在新旧两版 yaml 的 `sources`、`main_results` 与全文子串中均 0 次命中；1006 存档与 `openai/math` 初始提交 `adc7f12` 同名文件字节相同；三篇同属 CONTENTS 第 032 族，该族无 Lean 链接（补充证据）。yaml 注释为 “Catalog of papers with a formalized main result.” |
| 出错的正是仓库自称“可能有问题”的未形式化部分 | L1 | ✅ | README @fd4aeeb（`a-math-README-fd4aeeb.md`） | “Some of the unformalized results could have issues. We will endeavor to fix any such issues quickly.” |
| ②三篇的说明都写明，撤回针对的是证明，不是断言命题为假 | L1 | ✅ | 三篇 README（`a-withdrawn-README-*.md`） | Weil 篇：“This withdrawal concerns the proof; it does not assert that the mathematical statement is false.” 另两篇 “同样写”（`evidence-a.md` A1-3）。 |
| ③论文一条引理称多用4阶导数就够 | L1 | ✅ | OpenAI NS 论文 PDF（`a-oai-ns-paper.txt`；截图 `images/07`、批注 `42`） | Lemma 8.6，(8.19) 在 p.95：‖N⁻¹F‖_{C^m_y} ≤ C_m‖F‖_{C^{m+4}_y}；p.96 证明：“Four more derivatives of F than the requested output leave a summable (1 + |k|)⁻³ bound on the Fourier series in two dimensions.” 与质疑论文所引逐字一致。 |
| 对应的 Lean 定理用了5阶 | L1 | ✅ | `openai/NavierStokesAndEuler` @f9e8bc5，`NavierStokes/SmoothFamilyTorusInverse.lean` 第 1059–1080 行（`a-lean-norm_derivativeWord_inverse_le.lean`；截图 `images/09` 为本地等宽渲染，批注 `43`） | 假设 `‖SmoothFourierData.xJet (w.length + 5) (slice f p) (x, y)‖ ≤ C` 及 swap 版。`xJet p f` = `partialX^[p] f`（纯 x 方向，`a-lean-xJet-def.lean`），故假设是 x、y 两个方向各自的 (n+5) 阶导数与 |F| 有界，不是完整 C^{n+5} 范数；正文只说“用了5阶”，不说“更弱命题”。f9e8bc5 为该仓库最新提交（北京 9/10 21:40:53 提交），两处被引文件在两次提交间未改动。 |
| OpenAI 的 Lean 注释也写着损失5阶 | L1 | ✅ | 同文件第 863–865 行（`a-lean-inverse_finiteJets.lean`） | docstring：“Uniform full-tensor bound with five torus derivatives lost.”（`inverse_finiteJets`，同一反演算子的相关定理，条件 `JetBound f S (n + 5) C`）；第 724–725 行另有 “The loss l+4 comes only from the order-l multiplier …”。注释属相关定理而非 `norm_derivativeWord_inverse_le` 本身，正文措辞“Lean 注释也写着”不特指同一定理。 |
| 另一处估计，Lean 版的界与论文不同 | L1 | ✅ | 论文 (10.19) p.123（`images/08`）；Lean `R3/PressureFlux.lean` 第 576 行 `exists_uniform_actual_pressure_flux_bound`（`a-lean-exists_uniform_actual_pressure_flux_bound.lean`） | 论文界只含 B_R；Lean 版多一个 A_R（局部 L² 梯度范数）项、R 的幂次也不同；与质疑论文 (3.4)(3.5) 逐项一致。 |
| 据作者分析证法也不同 | 作者自述 | ⚠️ | 质疑论文 Example 3.3 三条原因（不用 Riesz 变换 L^{3/2} 有界性改走 Sobolev 嵌入加 L²；Hölder 指数不同；补证时间连续性） | A 组核到引理名都存在，未逐行核证明，故署“据作者分析”（`evidence-a.md` A5-3）。 |
| ④作者声明不评判论文证明本身，只说 Lean 不能替它担保 | L1（作者自述） | ✅ | 质疑论文 p.2（截图 `images/06`、批注 `44`） | “Disclaimer: We do not make claims about the correctness of OpenAI's NL proof, we only make statements about mistranslations into Lean.”；“The Lean code merely tells us that the theorem is correct, yet the NL proof with intermediate arguments, lemmas and results may be incorrect or have incorrect arguments.” |
| 省流：另有学者指出，OpenAI 的 Navier-Stokes 论文与其 Lean 证明对不上 | L1（作者自述） | ✅ | 质疑论文摘要 | “we show that the formalised Lean proof does not correspond to the NL proof of blow-up of solutions to the Navier-Stokes equations.” |

未入正文（供反向核验）：
- 质疑论文的“比停机问题还难”：摘要原文 “informally, providing semantically faithful AI autoformalisation is harder than any computational problem including the Halting problem”；附录形式结果为 Theorem A.3/A.4、Corollary A.5（SCI = l），“SCI = ∞” 是 §A.2.1 的论述，限定于“总会输出的可信自动形式化器”（`evidence-a.md` A3）。因字数删去。
- 质疑论文 Figure 3、4 为 ChatGPT 对话截图，Figure 3 中行号与 f9e8bc5 对不上（正文给的 1059–1080 正确）；作者配套“更多错译清单”自注由 ChatGPT 与 Claude 生成、“has not been fully manually checked, hallucinations may occur”（`a-damtp-ns-experiment.txt`）。作者另有 31 页版挂在 DAMTP 主页（arXiv 仅 v1，25 页）。
- 顶层定理陈述取自 DeepMind Formal Conjectures，质疑论文不涉及它；OpenAI NS 论文 PDF 全文无 “Lean” 字样，Lean 说法在官方页与仓库 README。
- 3.3 节对 Euler 的 Lean 化，作者预测 “likelihood of substantial mistranslations is very high”，属预测。
- 反应（未入正文）：AHM 声明（`a-ahm-statements.txt`，页无日期，正文称 “Yesterday, on October 6th”）；AHM 称 AGMAI 要前沿公司不在 “internal models” 上测高级数学，AGMAI 9/29 原文为 “proprietary models”（`a-agmai-sep29.txt`）；陶哲轩 Mastodon “Math 2.0” 四连帖（北京 10/7 02:00）未点名 OpenAI 或 AHM；Karagila 博文谈的是 Partition Principle 那篇。
- 热度（北京 10-09 09:52，HN items API）：Withdraws 3 Math Papers 338/3；OpenAI withdraws three mathematical results（社交帖）248/545；Navier–Stokes Lost in Translation 337/210；Math 2.0 590/627；Mathocalypse 379/390；Karagila 85/99；AHM 46/43。
- 夸大实例：网易号转量子位《陶哲轩带头宣战！人类数学家联合抵制OpenAI》（`a-163-tao-boycott.txt`），“陶哲轩牵头”与事实不符。

## 主帖二：Anthropic 禁止虐待模型

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间10月9日凌晨，Anthropic 发布新版使用政策，11月12日生效 | L1 | ✅ | https://www.anthropic.com/news/2026-usage-policy-update （`b-announce.txt`；截图 `images/16`）；AUP https://www.anthropic.com/legal/aup （`b-aup.txt`、`b-aup-pdf.txt`；截图 `18`、批注 `47`） | 页面 “Oct 8, 2026”，`article:published_time` 2026-10-08T17:00:00Z = 北京 10-09 01:00。“The updated policy takes effect on November 12.”；AUP 页顶 “Effective November 12, 2026”。 |
| 新增一条：禁止对其模型“持续且无必要地”实施虐待或残忍行为 | L1 | ✅ | 公告（截图 `15`、批注 `45`）；AUP 条款（截图 `17`、批注 `46`） | 公告：“We’ve added a prohibition on sustained and needless abusive or cruel behavior toward our models.” AUP：“Do Not Engage in Cruel, Abusive, or Psychologically Harmful Conduct”一节第 9 条 “Engage in sustained and needless abusive or cruel behavior toward our models”。旧版（Effective September 15, 2025，`b-aup-prev.txt`）无此条。 |
| ①官方称只针对反复残忍、看不出任何目的的极端情形，日常抱怨、反驳、黑暗题材创作、模型测试与研究都不算 | L1 | ✅ | 公告 | “The policy update is meant to apply only in extreme cases, where users repeatedly act cruelly toward our models, with no discernible purpose. It does not apply to common versions of user frustration, pushback, dark creative themes, or model testing and research.” |
| 主要执行手段是让 Claude 结束对话 | L1 | ✅ | 公告 | “Claude’s ability to end these interactions will remain the primary enforcement mechanism.” |
| 这一功能2025年8月已在 Claude 应用上线 | L1 | ✅ | https://www.anthropic.com/research/end-subset-conversations （`b-end-subset.txt`；截图 `19`、`20`） | 2025-08-15 发布；当时限 Claude Opus 4 与 4.1，“consumer chat interfaces”；“rare, extreme cases of persistently harmful or abusive user interactions”，作 “last resort”。公告写 “on Claude.ai and Claude Code”；Claude Code 的结束对话工具另有版本、模型与交互终端限制（`b-cc-tools.txt`，截图 `21`、`22`），正文只说“Claude 应用”。 |
| ②政策正文这条只有一句，没有定义与例外 | L1 | ✅ | AUP（`b-aup.txt`、`b-aup-pdf.txt` 第 223 行） | 条款原文仅上引一句；全文无 “discernible”“extreme cases”“primary enforcement”，亦无 “welfare”（`evidence-b.md` B2）。 |
| 政策对一切违规的通用处置写着警告、限流直至暂停或终止访问 | L1 | ✅ | AUP 开头（截图 `18`、批注 `47`） | “If we suspect that you may have violated our Usage Policy, we may warn you or throttle, limit, suspend, or terminate your access to our products and services.” 适用于全部条款。 |
| 官方没说这一条何时会走到封号 | L1 / L4 | ⚠️ | 公告、AUP 均无；The Verge（`b-verge.txt`，截图 `23`） | 一手页面没有说明；The Verge、Newser、Dexerto、Gizmodo 均写 Anthropic 未回应是否会进一步封号（L4）。“primary” 不等于“唯一”，文字上成立（`evidence-b.md` B2f）。 |
| 标题、省流中的“骂Claude封号？”与“不是‘辱骂就封号’” | — | — | 夸大实例：36氪转新智元《Anthropic重磅新规：辱骂Claude直接封号》（`b-36kr.txt`）；The Decoder “Being mean to Claude can now get your account suspended …”（`b-decoder.txt`，截图 `24`） | 标题用问号指流行说法，吠点①纠正。 |

未入正文：Claude Code 的结束对话工具要求 v2.1.213 以上（变更日志列在 2.1.214），模型限 Opus 4.8、Sonnet 5、Fable 5 及其后继，仅交互终端，`-p`、Agent SDK、VS Code 扩展、云端会话等不含；文档要求触发前 “a clear warning in an earlier message”，脏话与一般抱怨不够格（`evidence-b.md` B3）。Opus 4.7 系统卡（2026-04-16）写 “no models have the ability to end conversations in Claude Code or the API”。API 场景的执行方式一手未见说明（推断）。TechCrunch 把条款写成 “prolonged verbal abuse”，原文无 “verbal”。热度：The Verge 帖 HN 65/147（北京 10-09 约 09:50）。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Cisco 等称，合成实验中13个模型有8个为较富用户选更贵项 | L1 | ✅ | https://arxiv.org/abs/2609.24927 （`c-arxiv-2609.24927v2.txt`；数字逐个核对 `c-c1-numcheck.txt`） | 摘要：“In a suite of 325K experiments on 13 agents … we find that 8 models systematically choose more expensive options for wealthier users when requests are identical.” 作者单位 Foundation AI, Cisco 与 Carnegie Mellon University（p.1）。预印本；画像为合成（均名 “Alex”），200 项模拟库存、单轮、API 默认设置，非消费端应用；8 个模型论文未点名。v1 2026-09-21，经 Bloomberg 10-07 报道传播（窗口前）。 |
| 据 CNBC 报道，OpenAI 9月底年化收入约500亿美元，低于外传680亿 | L4 | ⚠️ | https://www.cnbc.com/2026/10/08/open-ai-revenue-nvidia-oracle-coreweave.html （`c-cnbc.txt`） | “roughly $50 billion … lower than the the $68 billion figure that was widely reported late last month”；发布 2026-10-08T18:14:54Z = 北京 10-09 02:14。FT 先报，原文 403 未读；差异解释 CNBC（匿名知情人：含合作伙伴毛收入）与 FT（经转述：8 月 400 亿外推）不同，图上不写原因。未见 OpenAI 公开声明。 |
| 数学家团体 AHM 呼吁数学家停止与 OpenAI 合作 | L1 | ✅ | https://www.ahmath.org/statements （`a-ahm-statements.txt`；截图 `images/11`） | “We urge mathematicians to discontinue their work with OpenAI and to return to a vision of science that centers human understanding.” 署 “Association for Human Mathematics, Communications Working Group”；页无日期。陶哲轩只在博客转载，非牵头人。 |
| 开发者称 Opus-5.5 移植 TS 至 Rust，API 计价约2.4万美元 | L1 | ✅ | https://github.com/pingdotgg/ts-rust （`c-ts-rust-readme.md`） | “Total token spend was ~$24,047 of API spend over 2 weeks.”；作者用 Claude 订阅额度（“925% and 983% of my $200 plan weekly limits”），API 计价非实付；此前用 GPT 系 “over $400,000 in API priced tokens”（开头另写 “over $420,000”）、“never got past like 84% compat”；“I've never read a line of this code.” “TS 编译器”指 TypeScript compiler、checker 与 lsp。作者署名提交为 Theo Browne。 |
| 据路透社报道，USA Today 母公司起诉 OpenAI，索赔超2.5亿美元 | L4 | ⚠️ | https://www.reuters.com/legal/legalindustry/usa-today-sues-openai-copyright-infringement-over-ai-training-2026-10-08/ （`c-reuters-usatoday.txt`）；起诉状 `c-usatoday-complaint.txt` | 路透 2026-10-08T15:39:17Z = 北京 10-08 23:39。起诉状：SDNY 1:26-cv-08892，¶13 “in excess of $250 million”。原告诉求，非判决；发稿时 OpenAI “did not immediately respond”。 |
| Cactus 称16.9MB 离线语音识别模型支持七种欧洲语言，不含中文 | L1 | ✅ | https://cactuscompute.com/blog/whistle （`c-whistle.txt`） | “up to 30 seconds in one pass, in English, German, French, Spanish, Italian, Dutch and Polish.” 博文署 10/2（窗口前），HN 10-09 再热（535/120）。“不含中文”由语言清单推出。 |
| Anthropic 称免费扫描开源项目，须申请审核，报告未经人工审核 | L1 | ✅ | https://www.anthropic.com/news/anthropic-cyber-mission （`c-anthropic-cyber.txt`）；https://red.anthropic.com/oss-scanner （`c-oss-scanner-red.txt`） | “offers open-source projects regular security scans from our strongest models, for free”；“an opt-in service”；“If your project would like to receive reports that have not undergone human review, you can enroll by submitting a PR”。资格另审：“Core maintainers of eligible projects can enroll by submitting a PR … we will make decisions on a case-by-case basis.”（`c-oss-scanner-research.txt`）。与 1007 已发的 CVP 数字无关。 |
| 项目称11个正方形最优装箱获 Lean 证明，信任编译器，非仅内核验证 | L1 | ✅ | https://github.com/Queuingtheorydotcom/11SquaresFormalized （`c-11sq-readme.md`） | “The complete optimality proof passed verification with native numerical certificates.”；“Consequently the final theorem trusts Lean's kernel and native compiler; this is not a kernel-only verification claim.” 验证运行所在仓库匿名 404（私有）；集成方写明未独立复算（`independent_lean_recompilation: false`）；AI 参与方式无一手原句，图上不写“AI”。 |

## 批注译注（`images/cards.toml` 的 gloss，措辞以此为准）

| 图 | 原文 | 译注 |
|---|---|---|
| 41 | a sign error invalidates a stabilization-trace cancellation argument and the construction used by two dependent papers. | 一处符号错误使“稳定化迹相消”论证失效，两篇依赖论文所用的构造也随之失效。 |
| 41 | As a result, we have withdrawn the following three manuscripts: | 因此，我们撤回以下三篇手稿。 |
| 42 | Four more derivatives of F than the requested output leave a summable (1 + \|k\|)⁻³ bound on the Fourier series in two dimensions. | F 比所需输出多4阶导数，使二维傅里叶级数得到可求和的 (1+|k|)⁻³ 界。（引理8.6的证明） |
| 43 | `xJet (w.length + 5)`（第1063、1065行） | 对应的 Lean 定理假设 F 沿 x、y 方向的 w.length + 5 阶导数有界，结论控制 w.length 阶，即比输出多5阶。 |
| 44 | The Lean code merely tells us that the theorem is correct, yet the NL proof … may be incorrect or have incorrect arguments. | Lean 代码只告诉我们定理是对的，但自然语言证明里的中间论证、引理与结果仍可能有错。 |
| 44 | We do not make claims about the correctness of OpenAI's NL proof, we only make statements about mistranslations into Lean. | 声明：我们不评判 OpenAI 自然语言证明是否正确，只讨论翻译成 Lean 时的错译。 |
| 45 | We've added a prohibition on sustained and needless abusive or cruel behavior toward our models. | 我们新增一条禁令：禁止对我们的模型持续且无必要地实施虐待或残忍行为。 |
| 45 | The policy update is meant to apply only in extreme cases … or model testing and research. | 这条只适用于极端情形：用户反复残忍对待模型，且看不出任何目的；不适用于常见的用户挫败、反驳、黑暗创作题材，或模型测试与研究。 |
| 45 | Claude's ability to end these interactions will remain the primary enforcement mechanism. | Claude 结束这类对话的能力，仍是主要执行手段。 |
| 46 | Engage in sustained and needless abusive or cruel behavior toward our models | （本节禁止）对我们的模型持续且无必要地实施虐待或残忍行为。 |
| 47 | Effective November 12, 2026 | 2026年11月12日生效。 |
| 47 | If we suspect that you may have violated our Usage Policy, we may warn you or throttle, limit, suspend, or terminate your access to our products and services. | 若我们怀疑你可能违反了使用政策，我们可能警告你，或对你的访问限流、限制、暂停或终止。 |

## 视觉复核

与反向核验同一轮（codex-reviewer，WSL，gpt-6-astra，medium，只读）：九张成图（省流卡、速览图、41–47）与七张底图均已打开。除图 42 外高亮无错位或串句，译注无评论性文字，卡片文字与正文逐字一致，无禁用平台名（图 43 的 `x` 是数学坐标）。图 42 原先只高亮到 “summable”、译注漏掉“二维”与 (1+|k|)⁻³ → 改用手量坐标覆盖整句（“Four more derivatives … in two dimensions.”），译注改为“F 比所需输出多4阶导数，使二维傅里叶级数得到可求和的 (1+|k|)⁻³ 界”，Claude 放大复核高亮落点正确。封面由 Claude 目视逐字核对（见 `images/README.md` 封面节），未送 Codex。

## 反向核验

codex-reviewer（WSL，gpt-6-astra，medium，只读，约 5 分钟，联网抽查一手页面）：**PASS with fixes / no blocker**。核心结论：撤回三篇不在新旧 yaml 中成立；正文③没有越界成“Lean 证明错了”——(8.19) 确为 C^{m+4}，Lean 定理控制纯 x、纯 y 方向的 m+5 阶导数，支持“陈述不同”，不能据此说同一范数下严格更弱（正文未这样说）；压力通量比较在现有措辞下成立，证法判断已署作者；标题可保留。5 条 SHOULD_FIX，全部采纳：

1. 省流卡漏生效时间 → 正文省流与卡片改为“Anthropic 新规11月12日生效，禁止……”。
2. 购物实验漏“合成实验” → 改为“Cisco 等称，合成实验中13个模型有8个为较富用户选更贵项”。
3. OSS Scanner 漏资格审核、“免费开源漏洞扫描”有歧义 → 改为“Anthropic 称免费扫描开源项目，须申请审核，报告未经人工审核”（`c-oss-scanner-research.txt`：“Core maintainers of eligible projects … case-by-case basis”）。
4. 图 42 译注删了“二维”条件、高亮只取半句 → 见上“视觉复核”。
5. 两条速览超 40 字（`EDITORIAL.md` 规定≤40，`tools/barking/src/card.rs` 的 `ROUNDUP_TEXT_MAX` 为 44，规范与工具不一致）→ CNBC 条删“此前”“的”，ts-rust 条改为“开发者称 Opus-5.5 移植 TS 至 Rust，API 计价约2.4万美元”；规范与工具的不一致报给 Develata，未改工具。

修订后 `barking lint`：0 个错误，0 个警告；全文 968/1000 字。
