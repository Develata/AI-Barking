# 1120 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B、C 组 Sonnet 5.5（`claude-sonnet-5-5`）；用时（harness）A 19 分钟、B 15 分钟、C 14 分钟。Claude 抽查：Alpöge 声明 SHA（与 1119 存档一致）、Sutherland 页（“two independently written kernels”、“"GRH" here means Dirichlet L-functions only”、“roughly 100 GB … about 50 GB of RAM”）、Lipponen README、Vakil 博文与更新、Masullo 稿第 6 页与第 522 页、Aaronson 三句、论文 v2 的 Tab. 8/Tab. 13/E.9/Fig. 1 图注/截断比例、NS 论文摘要与 certificate 句。存档均在本目录，`a-`/`b-`/`c-` 前缀。

## 一、两份广义黎曼猜想证明（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 几小时内，两份文稿称证明了广义黎曼猜想（省流） | L2 | ✅ | Alpöge `remark.pdf`（`docs/2611/1119/sources/d-rh-remark.txt`，SHA-256 与北京 11-21 18:15 重取一致）：“In fact, the method easily extends much further, resolving the so-called Generalized Riemann Hypothesis”；Vakil 博文 `a-vakil*.txt` 标题 “Hurried Announcement: Alessandro Masullo GRH” | 北京时间：Alpöge PDF 创建 11-20 20:52、HN 帖提交时间 22:16:21（`a-hn-50574968.json` created_at 2026-11-20T14:16:21Z；不是精确发布时刻），Vakil 博文 11-21 02:18:42；相隔约 4–5.5 小时（第 1 轮反向核验后由“同天”改，原为美国日期口径）。两份范围不同，见吠点③ |
| 借 Claude 写的那份过了多项机器检查，作者自述没吃透（省流；标题：机器验过，作者没吃透；小标题：AI 那份作者没吃透） | L2 | ✅ | `a-sutherland.txt` Summary：“The published Lean source compiles without errors, the five headline theorems depend only on the three standard axioms, every module has been replayed through the Lean kernel, two independently written kernels, nanoda and con-leche, accept the full exported proof, and lean4lean accepts every compiled module.”；`remark.pdf` 第 2 页：“before having mastered a massive neighbourhood around its mathematics”；Lipponen README：“It would not establish that anyone understands the argument” | “多项”而非“全部”：同页写 “The lake comparator run and the Lean-kernel replay of the export are still in progress”（北京 18:37 复取未变）。“没吃透”只指作者本人（“before having mastered a massive neighbourhood around its mathematics”）；第 1 轮反向核验 BLOCKER 后由“人没读懂”改 |
| 人写的那份没写完，多人借 AI 报了问题（省流） | L2 | ✅ | Masullo 稿 `a-masullo-excerpts.txt` 第 6 页：“only filling in some writing remains on several parts across this long paper”“The introduction itself is unfinished”；Vakil 更新：“Several people have contacted me to report problems in the proposed proof, identified with the assistance of different AI models.” | 问题内容 Vakil 未列；“报了问题”≠ 已确认错误（Vakil：“all issues need to be resolved to the satisfaction of the community”） |
| Anthropic 员工 Alpöge 称与自家 AI Claude 找到黎曼猜想证明，可推到广义版，只公开 Lean 代码 | L2 | ✅ | `remark.pdf` 第 1 页：“I believe I have, working with Claude, discovered a proof of the Riemann Hypothesis. In fact, the method easily extends much further, resolving the so-called Generalized Riemann Hypothesis”；第 2 页 “I decided to only present Lean”；主页 “hi from anthropic!” | 个人声明，非 Anthropic 官方；官方新闻页（北京 18:29）无 RH 条目，AP、Nature 写 “did not respond”（A10）。另公开了 4 页思路说明与时间线，“只公开 Lean 代码”指证明本身 |
| MIT 数学家 Sutherland 的复核页称，17万多个代码文件编译通过，Lean 自带和两个独立编写的检查程序都接受 | L2 | ✅ | `a-sutherland.txt`：“Andrew Sutherland (MIT), checks run with Claude Code”；“All 175,187 modules compile (Lean v4.33.1, Mathlib v4.33.1) pass zero errors”；“leanchecker kernel replay of every module pass 175,187 of 175,187”；“two independently written kernels, nanoda and con-leche, accept the full exported proof” | 页面 “last updated 2026-11-21 09:30 UTC”（北京 17:30）；lean4lean 另逐模块全过。Lipponen 在 LUMI 独立编译 175,187/175,187、`Main.olean` 哈希一致（`a-lipponen-readme.*`） |
| 几小时后，数学家 Vakil 公布 Masullo 的940页证明草稿，称“全靠人、没用 AI” | L2 | ✅ | Vakil 博文：“Alessandro Maria Masullo has a proposed proof of GRH … It is all human, with no AI … Now it seems best to put out the current draft.”；`a-masullo-pdf.pdf` pdfinfo Pages 940 | 博文北京 11-21 02:18:42（11-20 18:18 UTC）；Vakil 是 Stanford 数学家（博客为其 Math 216 课程站）；“全靠人、没用 AI”为意译 |
| 约8小时后他补记，多人借 AI 报告了问题 | L2 | ✅ | Vakil 博文：“Update about 8 hours later: … Several people have contacted me to report problems in the proposed proof, identified with the assistance of different AI models … (I make no claim as to the correctness of the proof strategy.)” | 更新段未标时刻；页面修改于北京 10:54:54（发布后约 8 小时 36 分） |
| 对你：两份都能免费下载；按复核页，自己复核 AI 那份最快也要约 50GB 内存 | L2 | ✅ | `a-sutherland-recheck.txt` “Check it yourself”：“You need roughly 100 GB of RAM and 70 GB of disk.”；“The fastest independent check is con-leche built from source … It needs about 50 GB of RAM and took under four minutes on 64 threads” | Lean 分卷 part01–14 在 alpo.ge 公开下载（1119 存档记录）；Masullo PDF 由 Vakil 博文直链下载（`a-masullo-pdf.pdf`）；导出文件压缩 7.7 GB |
| （正文已删，第 2 轮：L4 不进“对你”）据《科学美国人》报道，这个证明暂无即时应用 | L4 | ⚠️ | `a-sciam.txt`：“It doesn’t have any immediate real-world applications—it won’t upend global banking or make the internet run faster.” | 正文按 L4 写“据《科学美国人》报道” |
| 吠点①：新浪新闻标题称“AI 写出25万页证明” | L5（夸大实例） | ✅ | `a-sinacn-weibo.txt`：标题 “AI 写出 25 万页证明，陶哲轩：验证仍需时间” | 新浪新闻（sina.cn）北京 11-21 08:02；引号内按本号空格规则去掉数字两侧空格 |
| 吠点①：25万页是 Alpöge 脚注里提的未公开文稿，他自述还没掌握其中大片数学 | L2 | ✅ | `remark.pdf` 第 2 页：“It being obviously inappropriate to attempt to post some massive unreadable travesty of "slop"², I decided to only present Lean.”；脚注 2：“I of course amused myself, and hit 250, 000+ pages.”；“before having mastered a massive neighbourhood around its mathematics” | 25 万页未公开，无人见过 |
| 吠点②：检查程序只能保证代码里每一步推得通 | L2 | ✅ | `a-sutherland.txt`：“A passing check shows that the formal statements, as written in Lean against Mathlib's definitions, follow from the three standard axioms according to the kernels used.” | 三条：propext、Classical.choice、Quot.sound |
| 吠点②：复核者 Lipponen 写明这不等于有人读懂论证，另有检查仍在跑 | L2 | ✅ | Lipponen README：“It would not establish that anyone understands the argument”；Sutherland 页：“The lake comparator run and the Lean-kernel replay of the export are still in progress.” | Lipponen 为仓库作者（芬兰 LUMI 超算上的独立复核）；北京 18:37 复取 Sutherland 页仍为 running |
| 吠点③：两份“广义”不一样大：复核页写明 AI 那份只证了一类（狄利克雷 L 函数） | L2 | ✅ | `a-sutherland.txt`：“and four further theorems, including the analogous statement for all Dirichlet L-functions. "GRH" here means Dirichlet L-functions only.” | Alpöge 声明称方法“easily extends much further”；形式化的是 Dirichlet L 函数 |
| 吠点③：人写那份称范围更大，推出结论的定理却只有标题，脚注写“插入陈述” | L2 | ✅ | `a-masullo-excerpts.txt` 第 6 页：“(generality: all number fields, all unitary Hecke characters)”；第 522 页：“Theorem 13.3.1.2 (Divisor of L(s, χ)).” 后无正文，脚注 “1Correct statement. 2Insert statement. 3State and prove the Main Theorems, also explaining how to deduce them…”；第 523 页起为第 14 章 | 第 13 章 “Zeros of Hecke L-functions” 只有 521–522 两页；同章后文称 Theorem 13.3.1.2 断言非平凡零点在 1/2+iR 上。“插入陈述”为 “Insert statement” 的直译 |

## 二、Qwen 改写自己权重（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 论文称 Qwen 改自己参数后，做题准确率从57.2%升到63.8%（省流） | L1 | ✅ | `b-html.txt` 摘要：“its own edit makes it 6.6 points more accurate on problems it never saw”；§1：“6.6 points more accurate on 490 problems it never saw, in a paired test fixed in advance” | “points”为百分点；第二协议第 8 号会话的编辑 |
| 8轮各自从头改，5轮撤回（省流；小标题：8轮撤了5轮） | L1 | ✅ | Fig. 1 图注：“(each session itself starts from the base weights; Sec. E.9)”；E.9：“Every session starts from the base weights”“Sessions differ only in the seed of the model’s turns. The first eight sessions, all reported.”“In five the model…”；Tab. 13 会话 3–7 中 5 个 withdraw | 第二协议；会话 3、4、7 一个编辑都没留下，5、6 检查后撤回（均记为 withdraw） |
| arXiv 预印本称，开源模型 Qwen3.5-27B 读自己的参数（权重），不训练就改写 | L1 | ✅ | 摘要：“an open 27B model, given its weights, its graded answers and the ability to run itself, diagnoses its own failure and rewrites the weights behind it”；“with no gradients and no training” | arXiv 2611.21885 v2（北京 11-18 16:57）；无 v3（北京 18:25 查 404） |
| 在另留的490道数学题上（每题答8次）准确率从57.2%升到63.8% | L1 | ✅ | Tab. 13（`21-tab13-second-protocol.png`）：会话 8 “63.8 +6.6∗ [5.0, 8.1] <10⁻⁵”；表注 “490 problems no session saw with eight answers each at 16,384 tokens, against the same answers of the base model (57.2%)” | 每题 8 个答案，3,920 个答案的准确率。“另留”= 没有会话看到的留出集；论文 App. D.3 称其中仍有 8 道与练习题改述重复（“eight in other words”），去掉后第一协议仍显著（Tab. 7：482 题 57.3→58.3，p=0.045）；第 2 轮反向核验后由“没见过的”改 |
| （正文已删）论文未经同行评审 | L1 | ✅ | `b-abs.txt`：arXiv 页只有 Submission history v1/v2，未标会议或期刊（Comments 字段无录用信息） | 依 arXiv 预印本状态 |
| 对你：项目页没给改过的模型下载 | L1 | ✅ | 论文 HTML/PDF、作者项目页 `b-proj.*`、Hugging Face 论文页 `b-hf.txt` 均无代码、权重或会话记录链接 | 第 2 轮反向核验：“你用的 Qwen 不受影响”是推断，已删。第三方仓库 `DukeCaesar/models-rewrite-own-weights` README 自述尚未复现任何准确率（`b-gh-dc-readme.txt`），正文未写 |
| 对你：自我改进只测了数学题 | L1 | ✅ | §3.1/App. D：练习与评测均为 DeepScaleR 数学题；Tab. 12 下：“the 27B model’s repair was found and tested on text math problems only, and no session saw an image” | 图像题只用于评测“修复”，不是自我改进 |
| 吠点①：智客前沿标题称“8轮持续提升” | L5（夸大实例） | ✅ | `b-m-zicq.txt` 标题：“27B模型无梯度自改权重，8轮持续提升并跨任务修复”；摘要：“连续8次会话均未回退” | 页面只有日期 11-18 |
| 吠点①：论文写每轮都从原始权重重改，5轮撤回，提交的3次准确率为57.8%、61.3%、63.8%，原版57.2% | L1 | ✅ | E.9 与 Fig. 1 图注（同上）；Tab. 13：会话 1 “submit … 57.8 +0.5 [−0.9, 1.9] 0.25”，会话 2 “submit … 61.3 +4.0∗”，会话 8 “submit … 63.8 +6.6∗” | 摘要另写 “each edit improving on the last, and over eight consecutive sessions its accuracy only rises”：曲线是“当前在用模型”被后提交的编辑替换；第一协议唯一的编辑 +1.1（Tab. 8，p=0.038），正文未写 |
| 吠点②：论文写这次改动针对写到长度上限还没答完的答案，这类答案从33.6%降到26.1% | L1 | ✅ | §3.6/App. E.9：“Both edits aim at what the model diagnosed in itself, answers that run out of tokens before they end: fewer of them run out with the edit (33.6% of the base model’s answers against 26.1% for session 8 and 28.8% for session 2).” | 16,384 token 上限；Fig. 1c 例子中原模型把一句话重复 549 次 |
| 吠点②：例题里原模型把一句话重复了549次 | L1 | ✅ | Fig. 1(c) 图注：“the base model repeats one sentence until the limit, 549 times, and never answers.” | 同一道留出题、同一种子，第 8 号会话编辑前后；与 1102 期（参考文献 [25]）修的复读循环同类，正文未写 |
| （正文已删）1102期那篇修的也是写不完的复读 | L1 | ✅ | `daily-scan.md` 已发过 1102 行：“修好的是复读循环（93%答案写到长度上限→33%）”；本文参考文献 [25] 即 1102 期那篇博文（同一 URL） | 1102 存档 `docs/2611/1102/sources/a-blog.txt` |
| 吠点③：摘要称能把弄坏的副本修回原准确率“80%以上”，那是图像题（70.2%对84.3%） | L1 | ✅ | 摘要：“the model brings it back to over 80% of the original accuracy on image tasks it never saw”；Tab. 12：六榜合计 84.3 / 7.3 / 70.2（原/损坏/修复），“83% of the base model’s 84.3%” | 每榜前 100 题；六榜里三榜修复÷原低于 80%（自算，正文未写） |
| 吠点③：修法在文本数学题上找到，文本题修后33.0%，原来58.5% | L1 | ✅ | Tab. 12 下：“found and tested on text math problems only”；Fig. 26 图注：“The repaired copy solves 33.0%, about 56% of the base model’s 58.5%” | 200 道文本留出题；修法为超过中位数 15 倍的权重剪到 15 倍（App. E.3），正文未写 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Anthropic 员工等称借 AI 证出 NS 方程无外力也爆破，证书未公开 | L2 | ✅ | NS 论文（`c-ns-pdf.txt`，Alpöge、Buckmaster，79 页，“version of 20 November 2026”）摘要：“for which the solutions of the three-dimensional Navier–Stokes equations without force blow up in finite time. This disproves the statements (A) and (B) of the Millennium Prize problem as formulated by Fefferman, and hence proves the statements (C) and (D)”；“The certificate will be released soon” | 跟进1008（OpenAI 9 月的 NS 证明带外力）。Buckmaster 为纽约大学学者；作者自述用 Claude。Lean 形式化以 certificate 为前提，区间算术“not part of the formalization”（`c-ns-gh-readme.txt`）。Clay 规则要求发表满两年并获普遍接受（`c-clay-rules.txt`） |
| Inkling 2 Mini 以 Apache 2.0 许可开放权重 | L1 | ✅ | `c-inkling-official.txt`：“Inkling 2 Mini is released with open weights”；模型卡（`c-inkling-modelcard.txt`）许可 “Apache 2.0”；“leads every open-weights model with less than 100B total parameters on benchmarks such as DeepSWE v1.1 (48.2%) and Vals Index v2.2 (46.5%)”；“not the strongest small model for multi-turn chat or for instructions with many constraints” | 第 2 轮反向核验：领先说法缺“Evaluation setups vary by model.”限定，卡片改为只写许可与开放权重（交付前自查：官方写 open weights，“开源”改“开放权重”；26.4B 总参数未上卡）；另附 Model Acceptable Use Policy；官方未给时刻（页面 Nov 20）；HN 50581260 997 分 |
| 据 Mother Jones 报道，数百名谷歌员工请愿让学校能关掉搜索里的 AI | L4 | ⚠️ | `c-mj-google.txt`：“Hundreds of workers at Google are pushing senior leadership to retract a policy that would block schools who …”；请愿书原文（`c-petition.txt`）：“turn off AI features in Search without disrupting their ability to use search as a whole” | Mother Jones 北京 11-20 20:30；“1000+”是工会会员数，未用；谷歌 11-03 当晚已澄清 Search 可单独开启 |
| OpenAI：ChatGPT Space 逐步向18岁以上 Plus 用户开放 | L1 | ✅ | OpenAI 发布说明（`c-oa-relnotes.txt`）11-20 条目：Space “rolling out globally to Plus users ages 18 and older”；官方帐号帖：“Space is the new home for pages, slides, images, files, and uploads” | Pro、Business、Enterprise 9-29 已有；共享页对方须有 Space 权限，Free 打不开 |
| METR：其评测中 AI 有一半把握完成的任务，按人类用时约3.6个月翻倍 | L3 | ✅ | `c-metr-blog.txt`：“doubling roughly every 3.61 months (95% CI 2.08-4.94)”；“increasingly concerned that our easy-to-run, containerized evals could overestimate real-world capabilities” | 指标为 50% time horizon（AI 以 50% 成功率完成的任务，按人类完成用时计）；TH1.1 与 TH2 合并；可验证子集 1.65 个月；METR 另称其容器化评测可能高估真实能力（未上卡）；官方未给时刻（页面 11-20）；HN 仅 2 分，按 AIHOT 精选收录 |

## 批注译注

| 卡片 | 原文 | 译注要点 | 备注 |
|---|---|---|---|
| 05-sutherland-summary | Summary 段两句 | 两个独立编写的内核 nanoda、con-leche 接受完整导出的证明；lake comparator 与 Lean 内核重放导出文件仍在进行 | `a-sutherland.txt`；页面 last updated 09:30 UTC |
| 06-sutherland-grh | “"GRH" here means Dirichlet L-functions only.” | 这里的 GRH 只指狄利克雷 L 函数 | 下划线 “Dirichlet L-functions only”；底图只取该段（代码块右端被页面横向滚动截断，视觉复核后不再入图） |
| 07-vakil-update | 博文三句 | 全靠人没用 AI；（约8小时后补记）多人借不同 AI 模型报告这份拟议证明的问题；不担保证明思路正确 | “约8小时后补记”为本号所加的位置说明，原文 “Update about 8 hours later” |
| 08-masullo-p522 | 第 522 页定理 13.3.1.2 标题与脚注 2、3 | 定理 13.3.1.2（L(s, χ) 的除子）；插入陈述；陈述并证明主定理 | 像素框标注；底图为同页 13.3 节标题至页末的连续截图（视觉复核后由上下拼接改）；脚注 3 只高亮 “State and prove the Main Theorems” |
| 11-qwen-sessions | E.9 两句 | 每一轮都从原始权重开始；8轮里有5轮撤回 | 原文 “In five the model withdraws” 前文为 “the first eight sessions” |
| 12-qwen-abstract | 摘要两句 | 自己的编辑让它在没见过的题上准确率高6.6个百分点；在没见过的图像任务上恢复到原准确率的80%以上 | “points”译为百分点 |
| 13-qwen-repair | 表 12 下注 | 27B 模型的修法只在文本数学题上找到和测试过 | 原句 “the 27B model’s repair was found and tested on text math problems only” |

## 反向核验

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-21 18:50–18:54，按提示词与报告文件时间；审的是读者稿检查前的初稿与事实清单，卡片尚无批注图、配图说明未写；审查中正文被改，报告引用的行号为当时版本）

1. BLOCKER 标题、省流、小标题“人没读懂”把“检查不能证明有人理解”写成全称判断——采纳：改为“作者没吃透”“作者自述没吃透”（只指作者本人，依据 remark.pdf 第 2 页）。
2. BLOCKER METR 条目丢了 50% 成功率与按人类用时的口径——采纳：改为“METR：其评测中 AI 有一半把握完成的任务，按人类用时约3.6个月翻倍”。
3. SHOULD_FIX “普通电脑跑不了复核”是编辑推断——采纳：删去，改“按复核页，自己复核最快也要约 50GB 内存”。
4. SHOULD_FIX “同天”按美国日期、未交代时区——采纳：改“几小时内”“几小时后”（北京 20:52/22:16 至次日 02:18）。
5. SHOULD_FIX 删去 “unitary” 扩大了 Masullo 声称范围——部分采纳：正文已不写 Hecke 特征，改“人写那份称范围更大”（酉 Hecke 特征覆盖狄利克雷特征，范围更大成立）；清单备注保留 “all unitary Hecke characters” 原句。
6. SHOULD_FIX 准确率分母——采纳：加“（每题答8次）”。
7. SHOULD_FIX 三次提升只给差值、第一次不显著——采纳：改给三次准确率 57.8%、61.3%、63.8% 与原版 57.2%，不再写差值。
8. SHOULD_FIX “immediate”误译为“直接”——采纳：改“暂无即时应用”。
9. SHOULD_FIX Inkling 缺“总参数”、评测设置不一——采纳“总参数”；设置差异以“官方称”“部分测试”限定，清单备注写明。
10. NEEDS_VERIFICATION “未经同行评审”——采纳：正文删去。
11. NEEDS_VERIFICATION “作者没公开代码和改后的权重”超出检查范围——采纳：改“项目页没给改过的模型下载”。
12. NEEDS_VERIFICATION 配图说明缺失——已补 `images/README.md` 与批注图，第 2 轮送审。

### 第 2 轮（Codex `gpt-6-astra` medium，北京 11-21 18:59–19:03，按文件时间；审的是第 1 轮修改后的正文、卡片与配图说明，封面节当时未写）

结论 PASS、无 BLOCKER，7 项 SHOULD_FIX：
1. “没见过的490道题”忽略论文自己说的 8 道改述重复——采纳：改“另留的490道”，清单补 Tab. 7 去重结果。
2. “你用的 Qwen 不受影响”是推断——采纳：删去。
3. 第一帖“对你”用了 L4 ⚠️ 的《科学美国人》——采纳：改为“两份都能免费下载；按复核页，自己复核 AI 那份最快也要约 50GB 内存”。
4. Inkling 领先说法缺“评测设置不一”——采纳：速览改为“Inkling 2 Mini 以 Apache 2.0 开源，总参数26.4B”，删比较。
5. “80%以上”只有相对数、省流 6.6 缺基线——采纳：吠点③加“（70.2%对84.3%）”，省流改“准确率从57.2%升到63.8%”。
6. 译注混入判断、漏“拟议”——采纳（与视觉复核同）。
7. HN 时间指向错存档——采纳：改指 `a-hn-50574968.json`，标为提交时间。

## 视觉复核

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-21 19:01–19:03，按文件时间；附图 22 张：两张封面、全部卡片与 7 张批注底图）

1. BLOCKER 08 底图上下拼接却标“原图未改”——采纳：改为同页 13.3 节标题至页末的连续截图。
2. BLOCKER 08 译注①“标题之后没有内容”“即零点分布”是本号判断——采纳：译注只留“定理 13.3.1.2（L(s, χ) 的除子）”，判断留在吠点卡。
3. SHOULD_FIX 08 译注③高亮整条脚注却省略后半——采纳：高亮缩到 “State and prove the Main Theorems”，译注去掉省略号。
4. SHOULD_FIX 省流卡“冒出两份证明”缺“称”——采纳：改“两份文稿称证明了广义黎曼猜想”。
5. SHOULD_FIX 07 译注“这份证明”丢“proposed”——采纳：改“这份拟议证明”。
6. SHOULD_FIX 06 代码块右端被截断——采纳：底图只取最后一段（含 "GRH" 一句），高亮含 GRH。
7. NICE_TO_HAVE 11、13 表格字小——未改：截图保留表注与上下文，读者可放大；吠点卡已写出关键数字。

Claude 复看：08、06、01 重渲后逐张打开，高亮位置、译注与正文一致。

### 交付前自查（北京 11-21 19:10 前后）

- Sutherland 复核页北京 19:11:35 重取，转文本与 `a-sutherland.txt` 无差异（lake comparator 与导出重放仍为 running），标题“机器验过”、吠点②“另有检查仍在跑”与 05 卡不变。
- 速览 Inkling 条：官方原句为 “released with open weights”，模型卡许可 Apache 2.0，“开源”改“开放权重”，删去未上卡的参数；02 重渲。
- 第 2 轮之后新增、未经跨模型复核的句子：省流首句“几小时内，两份文稿称证明了广义黎曼猜想”、对你“两份都能免费下载”、“另留的490道”、“（70.2%对84.3%）”、Inkling 条；均已对照本清单所列存档。
- 重渲后的 01–13 卡与两张封面逐张打开核对。
