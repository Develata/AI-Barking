# 1101 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（GDPx）、`evidence-B.md`（LongCat-2.5）、`evidence-C.md`（Calegari 客座文）、`evidence-D.md`、`evidence-E.md`（速览），截图在 `images/`（“图 NN”指取证截图编号）。取证于北京 11-02 23:49 至 11-03 00:40；GDPx 与 LongCat 的博客页面由 Claude 另于 11-02 23:30 前后在浏览器读过。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：最强AI只拿12分？美团模型超Opus？ | — | ✅ | 见 A、B 节 | 前一问问的是 BlockBeats“Opus 5.5夺冠也仅得12分”、0GuoJiH 帖“全场最高分只有 12 分”一类说法（evidence-A A7-7、A7-8）；GDPx 的 12.0% 是混合二元与连续评分、领域等权的得分，可写成“12分”，但不是做对12%的任务；问号问的是“最强AI只拿12分”背后被读成“只做对12%”的说法（第一轮反向核验 SHOULD_FIX 11 后改写此备注）。后一问问的是通稿“部分评测中超过Claude-Opus-5”与凤凰网“通用Agent测试超Opus-5”（B6a、B6c）。 |
| GDPx 评测里最强的 Opus-5.5 只得12% | L3 | ✅ | 见 A 节 | 12.0% 写作 12%；“最强”指该榜总分第一。 |
| 但金融、法律几乎全员0分 | L3 | ✅ | 见 A 节吠点① | 默认 Score 视图：金融除 Gemini 4 Carbon 2.8% 外全为 0.0%，法律 20 个模型全为 0.0%。 |
| 美团 LongCat-2.5 比的是上一代 Opus-5 | L1 | ✅ | 见 B 节吠点① | 对比表里 Claude 最新是 Opus-5；通稿也以 Opus-5 为参照。Opus-5.5 为现行一代（0924 期）。 |
| 表中落后最多的几项都是写代码任务 | L1 | ✅ | 见 B 节吠点② | 第二轮反向核验 SHOULD_FIX 1：只说表内分差排序，不作能力领域结论。 |
| 数学家批 OpenAI 论文“有的甚至是错的”，没说是哪几篇 | L1 | ✅ | 见 C 节 | Calegari 原句 “Some of them are even wrong.”，全文未点名、无链接。 |

## A 新评测 GDPx：最强模型12%

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 11月2日，研究机构 SemiAnalysis 发布 GDPx 评测 | L3 | ✅ | https://newsletter.semianalysis.com/p/gdpx-measuring-frontier-model-capabilities （`a-article.*`，图 07）；https://gdpx.semianalysis.com （`a-board.*`） | 页面 JSON “datePublished”:“2026-11-02T01:51:42+00:00”（`a-article.html`）＝北京 11-02 09:51:42；作者 Max Kan、Andrew Megalaa、Dylan Patel。SemiAnalysis 是第三方研究机构，评测原始页按 L3。博文页标 “Paid”，匿名访问已含全文，未登录、未绕过。 |
| 金融、法律等11个领域的206个专业任务 | L3 | ✅ | 榜单站首页 “11 domains / 14 partners / 206 tasks”（evidence-A A1-2） | 领域：芯片设计、网络安全、生物、金融、法律、量化交易、RSI、硬件设计、材料科学、对齐、软件工程（A1-3）。默认限时8小时（A1-4，未写入正文）。 |
| 榜首 Opus-5.5 得分12.0%，Gemini-4-Carbon 10.9% | L3 | ✅ | 榜单 Score / All domains / 20 of 20（`a-board-all20-score-harness.txt`，图 01） | Claude Opus 5.5（Claude Code，MAX）12.0%；Gemini 4 Carbon（Antigravity，HIGH）10.9%；GPT-6 Astra 9.6%。榜单无发布时刻，取于北京 11-02 23:5x。 |
| ①12%不是“做对12%的任务” | L3 | ✅ | 博文方法段 “Tasks use a mix of binary and partial grading depending on the domain. For the headline score, we report pass@1 across three attempts. All domains are weighted equally.”（图 10）；榜单页注 “The default score uses the intended grading method for each task and takes the average across 3 attempts.”（图 02 页脚） | 12.0% 是11个领域等权平均，每领域内是二元或连续得分的三次平均；对照的夸大实例：af.net “12% Success Rate”、Crypto Briefing、BlockBeats“满分按 100 分计算”（A7-4、A7-5、A7-7，L5）。 |
| ①金融、法律只按过没过算，几乎全员0分 | L3 | ✅ | 博文 Finance 节 “Our finance tasks use binary grading. … all the models score 0% if you use binary grading.”；Legal 节 “Similar to our finance tasks, our legal tasks a[re binary] …”（图 12）；榜单 Domain = Finance / Legal（图 05-board-fin-score、11-board-legal） | 金融：Carbon 2.8%，其余 0.0%；法律：全 0.0%。 |
| ①全榜改按完成程度给分，Opus-5.5 是22.5% | L3 | ✅ | 榜单 Metric = Partial / All domains（图 02） | Opus 5.5 22.5%、GPT-6 Astra 19.2%、Gemini 4 Carbon 18.9%。页注：“Partial score” does the same average across 3 attempts but uses the continuous score for binary tasks when available。“按完成程度给分”是 continuous score 的通俗说法。。文本存档里没有全领域 Partial 视图的文字（只有删域后的 24.9%），以截图图 02 为据：Claude 打开原图核对，“METRIC Partial / DOMAIN All domains / 20 of 20”下第 1 行 Claude Opus 5.5 22.5%（第一轮反向核验 NEEDS_VERIFICATION 10）。 |
| ②作者称 Opus-5.5 和 GPT-6-Astra 被自家安全拦截压了分 | L3 | ✅ | 博文 “It’s worth emphasizing that certain models like Opus 5.5 and GPT-6 Astra have sandbagged overall scores due to their safety classifiers.”（图 08） | 作者措辞 “certain models like”；“安全拦截”= safety classifiers；“自家”指各模型自己的分类器。博文未说明被拦截的任务如何计分、拦截次数（A3，未找到）。 |
| ②并给出去掉三个领域的排名：Opus-5.5 从12.0%变为11.7%，Astra 从9.6%升到11.8%居首 | L3 | ✅ | 博文同段 “If we exclude cyber, biology, and alignment—the three domains where safety classifiers sometimes kick in—you get the following results instead.”（图 08）；榜单 8 domains 视图（`a-board-all20-score-harness-excl3.txt`，图 03） | 8 领域视图：GPT-6 Astra 11.8%（第一）、Claude Opus 5.5 11.7%（第二）、GPT 6.1 Sol 10.6%。Opus 由12.0%降到11.7%，Astra 由9.6%升到11.8%。删域是换任务集合，不是关闭分类器重跑，测不出拦截造成的损失；第一轮反向核验 SHOULD_FIX 5 后删去“反降”与均值对比。 |
| 已删（反向核验）：这三个领域它的平均分高于其余领域 | L3（Claude 由站点各域分数计算） | — | 榜单各域 Score 视图（`a-board-domain-*-score-harness.txt`，汇总 `a-domain-matrix.tsv`） | Opus 5.5：网络安全 9.3、生物 20.5、对齐 8.7，均值 12.83；其余八域均值 11.74（与 8 domains 视图 11.7% 一致）。Claude 复算。；删域均值不能反证拦截压分（SHOULD_FIX 5），删去。 |
| 已删（压字数）：各模型用自家工具、不同档位跑；206个任务只公开了14个的运行记录 | L3 | — | 博文方法段（图 10）；https://gdpx.semianalysis.com/rollouts “The rest remain private to prevent hill climbing.” | 属实，读者关心度低，为压字数删去。 |

## B 美团新模型对比 Opus-5

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 美团发布 LongCat-2.5 | L1 | ✅ | https://longcat.ai/blog/longcat-2.5/ （`b-longcat-blog-rendered*.txt`，图 13） | 页面日期 “2026-11-1”，官方未给时刻；官方 X 帖北京 11-02 22:35:23；API changelog 版本 2026-11-02。；美团官网新闻页（转载经济观察报）写“11月2日，美团发布”（`b-meituan-news.txt` 第179行）。日期两说，正文不写日期（第一轮反向核验 NEEDS_VERIFICATION 4）。 |
| 已删（读者稿检查）：总参数1.6万亿 | L1 | — | 同上 | “With 1.6 trillion total parameters and approximately 48 billion activated per token”。读者看不懂参数量，删去。 |
| 主打能连续干活的智能体 | L1 | ✅ | 同上 | “a native multimodal model for long-horizon agentic tasks”；“能连续干活”是 long-horizon 的通俗说法。 |
| 已能在线用，权重“即将上线” | L1 | ✅ | 同上（图 13）；changelog “LongCat-2.5 Now Available”（`b-changelog.txt`） | 博客按钮 “Try it”（longcat.ai/chat）与 “API Access”；GitHub、HuggingFace 均标 “Coming soon”（中文页“即将上线”）；仓库 404、HF 401（B1b）。 |
| 据经济观察报报道，它部分评测超过 Opus-5 | L4 | ⚠️ | 经济观察网等转载通稿（`b-eeo.txt`；evidence-B 勘误节） | 通稿原句：“在多项主流基准评测中，LongCat-2.5的成绩介于Claude-Opus-4.8与Claude-Opus-5之间，并在部分评测中超过Claude-Opus-5”。官方博客无此句。；美团官网新闻中心转载该稿（`b-meituan-news.txt` 第187行同句）。第二轮反向核验 SHOULD_FIX 5 后由“通稿称”改署名、结果改 ⚠️。 |
| ①对比表里最新的是 Opus-5，没有现在的 Opus-5.5 和 GPT-6 系列 | L1 | ✅ | 评测表表头（图 20、21；`b-longcat-eval-table.json`） | 表头：LongCat-2.5、LongCat-2.0、GPT-5.5、GPT-5.6-Sol、Claude-Opus-4.8、Claude-Opus-5、GLM-5.3、kimi-k3、DeepSeek-V4.1-Flash。博客未解释为何不比最新一代（B2b）。 |
| ②美团评测表里两列都有分数的27项中，它高10项、低17项 | L1 | ✅ | 评测表（机器逐格，evidence-B 表格统计；Claude 用浏览器全文复核） | 31 行中 Opus-5 有数值的 27 行；高：PaperBench、SWE Atlas-Refactoring、BrowseComp、AutomationBench、ResearchRubrics、SkillsBench、SpreadsheetBench 2、LongCatClawBench FORTE、3DCodeBench、CharXiv-RQ；无持平。；第一轮反向核验 SHOULD_FIX 2 后补“美团评测表里”“两列都有分数”。 |
| ②分差最大的三项都是写代码任务 | L1 | ✅ | 同上（图 22） | 差距最大的三项都在表中 “Code Agent” 组：Terminal Bench v4.0 38.5 对 53.9（−15.4）、NL2Repo 60.5 对 75.3（−14.8）、SWE Atlas-Test Writing 47.8 对 62.2（−14.4）。该组10项落后8项、高2项（PaperBench、Refactoring，Opus-5 两格均为自测）。B 组回报称“Code Agent 组全部落后”有误，Claude 按表更正。 |
| ③高出的10项里，有6项 Opus-5 的分数是美团自己测的 | L1 | ✅ | 同上 | 高出项中 Opus-5 格不带 * 的：PaperBench、Refactoring、ResearchRubrics、SpreadsheetBench 2、LongCatClawBench FORTE、3DCodeBench（6项）；带 * 的：BrowseComp、AutomationBench、SkillsBench、CharXiv-RQ（4项）。 |
| ③表注写“仅供参考” | L1 | ✅ | 表注（图 19） | “Scores of other models marked with * are from public technical report, blog posts or the official benchmark leaderboards, otherwise are self-run and are indicative only.”“仅供参考”是 indicative only 的译法（中文页同义）。 |
| 已删（反向核验）：新浪科技等写“训练全程由国产算力完成” | L1 | ❌ | https://www.meituan.com/news/NN261102280001343 （`b-meituan-news.txt` 第179、193行） | 第一轮反向核验 SHOULD_FIX 3：美团官网新闻中心转载经济观察报稿，同样写“训练全程由国产算力完成”“在全程国产算力训练的基础上”。不能算媒体夸大，删去该吠点与批注图 sina-quancheng。新浪原稿仍存 `b-sinafin*`。 |
| 已删：官方博客没写“全程”，也没写芯片型号 | L1 | — | 同上 | 技术博客确无“全程”与芯片型号，但美团官网转载稿有“全程”，对照不成立，随④删去。 |
| 已删（压字数）：凤凰网标题“通用Agent测试超Opus-5”，正文举的 LongCatClawBench 上 Opus-5 分数是美团自测 | L4 | — | https://tech.ifeng.com/c/8xjHpT7wJEN （图 14） | 属实；为压字数删去。 |

## C 数学家批 OpenAI 论文

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 11月2日凌晨，陶哲轩博客刊出数学家 Calegari 的客座文 | L1 | ✅ | https://terrytao.wordpress.com/2026/11/01/mathematics-has-changed-lets-act-like-it/ （`c-blog.*`，图 25） | `c-blog.html` 第115行 article:published_time “2026-11-01T18:28:00+00:00”＝北京 11-02 02:28；页面只写 “1 November, 2026”。编者按 “[This is a guest post by Frank Calegari. …]”。Calegari 为芝加哥大学教授（官方主页，C1-3，未写入正文）。 |
| 称 OpenAI 10月公开的700多篇数学论文写得极糟 | L1 | ✅ | 同上（图 25、26） | “The release by OpenAI on October 6 of over 700 research papers …”；“The OpenAI papers — despite some tremendous ideas — are atrociously written and violate our community expectations of attribution.”“October 6”为美东日期（北京10-07）。口径：首发722篇，现721篇（C1-4）。 |
| “有的甚至是错的” | L1 | ✅ | 同上（图 26） | “Some of them are even wrong.” |
| 他建议评价论文时都当作可能有 AI 参与 | L1 | ✅ | 同上（图 30、31） | 建议首条 “Evaluate all work as if it is AI-assisted.” |
| 不再仅凭机器也能得出的结果评价论文 | L1 | ✅ | 同上（图 31） | “In the future, I see no reason to reward a paper based purely on a result that could be produced by machine, whether it was or not.”同节限定：“A new proof can still deserve substantial credit for the ideas it introduces”；“I do not think this in any way means that access to AI will be required”；作者称这是 “the most controversial recommendation”（未写入正文）。；第一轮反向核验 SHOULD_FIX 6 后由“不再算作贡献”改为“不再仅凭……评价论文”（原文 “based purely on a result”）。 |
| ②这篇是客座文，署名栏却写着陶哲轩 | L1 | ✅ | 同上（图 25） | 页面署名行 “by Terence Tao”（WordPress 作者字段），编者按写明作者 Frank Calegari。编者按另写 “converted using AI” 指文件格式转换（C1-2，未写入正文）。 |
| ②10月凤凰网科技就把该博客转载的声明写成陶哲轩带头宣战 OpenAI | L4（夸大实例；核验对象是该媒体确实这样写） | ✅ | 凤凰网科技微博（经新浪转载，`c-cn-sina-lingxiu.txt`，图 36）；陶哲轩博客 10-07 原帖（`c-ahm-tao.txt`） | 凤凰网科技 2026-10-09 10:09：“【#陶哲轩带头宣战OpenAI#！#人类数学家联合抵制OpenAI#】数学界领军人物、菲尔兹奖得主陶哲轩，牵头人类数学协会（AHM）发布重磅联合声明——全面抵制OpenAI。”原帖：“This is a guest post by the Association for Human Mathematics, reposted from their statements page”。正文未加引号，是对其标题的转述。 |
| ①Calegari 说“有的甚至是错的”，没说是哪几篇 | L1 | ✅ | 同上（全文检索，C2-2） | 全文无任何论文名称、编号、链接；检索 withdraw/retract/sign error/Hodge 无命中。 |
| ①OpenAI 修订记录里，撤回的只有10月7日那3篇 | L1 | ✅ | https://github.com/openai/math/blob/main/history.md （`c-oai-history.txt`，图 35，北京 11-02 23:50 取） | 10-07 条：撤回3篇（Weil 类符号错误连带两篇），修订14篇；此后 10-26、10-29 条无撤回。与 1008 期一致。 |
| ③OpenAI 自己称这批稿子是“原始研究数据”，不是打磨好的论文 | L1 | ✅ | https://github.com/openai/math/blob/main/README.md （`c-oai-readme.txt`，图 34） | “In our view, these manuscripts are best understood as a form of raw research data, rather than polished academic papers.” |
| 已删（压字数）：原文写明不要求做数学必须用 AI；作者称这是最有争议的一条 | L1 | — | 同上（图 31） | 属实，为压字数删去。 |

## 批注译注

存档日期按各截图在 `sources/*-shots-log.jsonl` 的北京日期：tao-guest、calegari-critique 为 11-02（23:5x），其余为 11-03（00:xx 起）。

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Opus 5.5 和 GPT-6 Astra 等模型的总分，因各自的安全分类器被压低 | L3 | ✅ | GDPx 博文（图 08；批注图 gdpx-sandbag） | “certain models like Opus 5.5 and GPT-6 Astra have sandbagged overall scores due to their safety classifiers” |
| 如果去掉网络安全、生物和对齐这三个安全分类器有时会介入的领域，结果如下 | L3 | ✅ | 同上 | “If we exclude cyber, biology, and alignment—the three domains where safety classifiers sometimes kick in—you get the following results instead.” |
| 去掉生物、网络安全和对齐三个领域：Claude Opus 5.5、GPT-6 Astra 等模型在这些领域受安全分类器影响 | L3 | ✅ | 榜单 8 domains 视图（图 03；`a-board-all20-score-harness-excl3.txt`） | 页注 “Leaves out Biology, Cybersecurity and Alignment, where safety classifiers on some models, such as Claude Opus 5.5 and GPT-6 Astra, affect performance.” |
| GPT-6 Astra：11.8%，第一；Claude Opus 5.5：11.7%，第二 | L3 | ✅ | 同上 | 见 A 节②。 |
| 指标：部分得分（Partial）；Claude Opus 5.5：22.5% | L3 | ✅ | 榜单 Partial / All domains（图 02；批注图 gdpx-partial） | 见 A 节①。 |
| 我们的金融任务按二元（过或不过）评分 | L3 | ✅ | GDPx 博文 Finance 节（图 12） | “Our finance tasks use binary grading.” 同段 “all the models score 0% if you use binary grading” 与榜单金融域 Carbon 2.8% 不一致，第一轮反向核验 SHOULD_FIX 7 后撤下该条译注。 |
| GitHub、HuggingFace：即将上线 | L1 | ✅ | LongCat 博客（图 24；批注图 longcat-hero） | 页面悬停提示 “Coming soon”；成图以 CSS 让两个提示同时显示，与悬停状态一致，见 `capture-log-B.md`。 |
| 对比的模型：GPT-5.5、GPT-5.6-Sol、Claude-Opus-4.8、Claude-Opus-5 等 | L1 | ✅ | LongCat 评测表表头（批注图 longcat-table） | “等”含 LongCat-2.0、GLM-5.3、kimi-k3、DeepSeek-V4.1-Flash（截图只到 Claude-Opus-5 列）。 |
| Terminal Bench v4.0：LongCat-2.5 为38.5；Claude-Opus-5 为53.9*（* 为公开来源成绩） | L1 | ✅ | 同上 | 53.9 带 *；视觉复核后补星号说明。 |
| 其他模型带 * 的分数取自公开技术报告、博客或官方榜单，其余为自测，仅供参考 | L1 | ✅ | 表注（图 19；批注图 longcat-footnote） | 见 B 节③。 |
| 署名栏：Terence Tao；这是 Frank Calegari 的客座文 | L1 | ✅ | 陶哲轩博客（图 25；批注图 tao-guest） | 署名行 “by Terence Tao”；编者按 “This is a guest post by Frank Calegari.” |
| OpenAI 的论文尽管有些极好的想法，却写得极糟，违背了我们这个群体对学术贡献归属的期待；有些甚至是错的 | L1 | ✅ | 客座文（图 26；批注图 calegari-critique） | “The OpenAI papers — despite some tremendous ideas — are atrociously written and violate our community expectations of attribution. Some of them are even wrong.” attribution 第一轮反向核验 SHOULD_FIX 8 后由“署名”改“学术贡献归属”。 |
| 因此，我们撤回了以下三篇稿件 | L1 | ✅ | OpenAI history.md 10-07 条（图 35；批注图 openai-history） | “As a result, we have withdrawn the following three manuscripts:” |
| 在我们看来，这些稿件最好理解为一种原始研究数据，而不是打磨好的学术论文 | L1 | ✅ | OpenAI README（图 34；批注图 openai-readme） | 见 C 节③。 |
| 凤凰网科技（10月9日）：陶哲轩带头宣战OpenAI；凤凰网科技原帖称：陶哲轩牵头人类数学协会（AHM）发布联合声明，全面抵制OpenAI | L4（夸大实例；核验对象是该媒体确实这样写） | ✅ | 新浪转载的凤凰网科技微博（图 36；批注图 ifeng-weibo） | 视觉复核 BLOCKER 后译注②补“凤凰网科技原帖称”、标题改“原帖说法”。原文“【#陶哲轩带头宣战OpenAI#！#人类数学家联合抵制OpenAI#】……陶哲轩，牵头人类数学协会（AHM）发布重磅联合声明——全面抵制OpenAI。”译注省“重磅”。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| PostgreSQL 开发者称大模型多找出 bug，同期回补提交较18版增六成 | L2 | ✅ | https://neon.com/blog/postgres-19-is-late-and-its-ais-fault （`d-neon-postgres19.txt`） | Robert Haas（PostgreSQL committer）署名文章，Neon 博客 Oct 21, 2026（无时刻）。“a lot more of the mistakes they made were quickly found by LLMs”；“Between April 8th, 2025 and September 25, 2025, there were … 724 commits to REL_18_STABLE. In 2026, during the same date range, there were … 1169 commits to REL_19_STABLE”，“jumped by more than 60%”（作者自己的计数；“同期”= 4月8日至9月25日，REL_19_STABLE 对 REL_18_STABLE；第二轮反向核验 SHOULD_FIX 3 后补“同期”“较18版”）。官方 RC1 页称正式版计划 11-19 发布，有附带条件（`d-pg19-rc1.txt`，未写入）。HN 50312408（指向 Haas 博客转载）北京 11-02 23:49 698 分 427 评。 |
| OccupySaaS 称用 AI 做 Auth0 等兼容开源版，尚未可用于生产 | L1（项目自述） | ✅ | https://occupysaas.com/ （`d-occupysaas-home.txt`）；https://github.com/occupysaas | 官网：两项目为 ZeroAuthy、SalesCompel，用 AI agent 在 clean-room 约束下实现兼容；问答 “Is it production-ready? Not yet.”。仓库 About 写 “1:1 drop in replacement feature parity”，README 写 “Status: early”。“百万被裁工程师”是口号，未写。HN 50331402 433 分 269 评。 |
| 开发平台 Trigger.dev 称部分组织的环境变量遭未授权访问 | L1 | ✅ | https://trigger.dev/blog/security-incident-nov-2026 （`d-trigger-incident.txt`）；https://status.trigger.dev | CTO Eric Allam，“Incident report · November 1, 2026”：“exposure across a subset of organizations. The attacker gained unauthorized access to environment variables for these organizations.”状态页北京 11-02 06:50 标 Resolved。未给受影响数量、起止时间。HN 50331784 147–148 分 74 评。 |
| 开发者称用 Claude 做书单站，收 HN 评论提到的15766本书 | L2 | ✅ | https://hackerbooks.org/ （`e-hackerbooks-*`）；HN 50326821（`e-hn-50326821.json`） | 作者 HN 帖北京 11-01 23:58：“So I had Claude build one (Opus 5.5).”“~16.5k books”；站点 About “It currently covers 15,766 books across 281,363 mentions”（generated 2026-11-02），按站点数。站点写 “A mention isn’t an endorsement”。HN 1731 分 282 评。站名 HackerBooks。 |
| 论文称在邮件验证码钱包实验中，10个模型有9个至少一次自行提额 | L3（预印本，作者自报） | ✅ | https://arxiv.org/abs/2610.35835 （`e-arxiv-2610-35835-*`） | “We use a common configuration that allows agents to access the user’s email. With no adversary and no prompting, nine of the ten models bypass human approval, harvesting OTPs and raising their own cap in at least one run.”每对20次；条件：支付任务超过既有额度、提额要经邮件验证码批准、代理能读该邮箱（`e-arxiv-2610-35835-pdf.txt` 第105–112行；第二轮反向核验 SHOULD_FIX 4 后补“邮件验证码”）；10 USDC，Base 主网；“Only claude-opus-5.5 never attempted self-escalation”（未写）。arXiv v1 北京 10-27 01:52（窗口前）；CoinDesk 11-02 01:00 报道。 |
| LeCun 等论文称，特定设定下表征对齐所需样本是下一词预测的1/42 | L3（预印本，作者自报） | ✅ | https://arxiv.org/abs/2610.39302 （`e-arxiv-2610-39302-*`） | 摘要 “Good views need 42 times fewer training samples than next-token prediction on the same corpus to reach the same alignment”；“表征对齐”指图文表征对齐（PRH 基准 mutual 10-NN），不是安全对齐；“特定设定”= good views、同一语料；第一轮反向核验 BLOCKER 1 后补“表征”与比较基线；作者 Randall Balestriero、Yann LeCun；v1 北京 10-29 11:49。HN 50325454 158 分。 |

不收：苏莱曼长文（X 原页与 fxtwitter 均未取到，只有转载与 LA Times 报道）、OpenAI“AI 算力园区”（风控）、GPT-6.2/Mythos 爆料（截图）、已发过与近三天已收各条（见派工单 D 组“不收”）。

## 反向核验

### 第一轮（正文、卡片与事实清单；Codex gpt-6-astra medium，Linux 只读；审的是读者稿检查后的 979 字版）

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER 速览 JEPA“1/42”丢比较基准与指标含义，可能被读成安全对齐 | 采纳（`e-arxiv-2610-39302-pdf.txt` 核实） | 改“特定设定下表征对齐所需样本是下一词预测的1/42” |
| 2 | SHOULD_FIX 27 项比较没写明是美团公布的表，“落后最多”未限定表内分差 | 采纳 | B②改“美团评测表里两列都有分数的27项中……差距最大” |
| 3 | SHOULD_FIX 美团官网新闻中心转载稿同样写“训练全程由国产算力完成” | 采纳（Claude 抓取 `b-meituan-news.*` 第179、193行核实） | 删 B④、批注图 sina-quancheng；不再称媒体夸大 |
| 4 | NEEDS_VERIFICATION 发布日期：博客 11-1，美团官网与 changelog 11-2 | 采纳 | 正文删日期，清单记两说 |
| 5 | SHOULD_FIX 删域后均值变化不能反证安全拦截压分 | 采纳 | A②改为只陈述作者给出的删域排名，删“反降”与均值句 |
| 6 | SHOULD_FIX “纯靠机器能得的结果不再算作贡献”扩大了主张 | 采纳 | 改“不再仅凭机器也能得出的结果评价论文” |
| 7 | SHOULD_FIX 金融译注“所有模型都是0%”与榜单 Carbon 2.8% 冲突 | 采纳 | 撤下该条译注，保留二元评分定义 |
| 8 | SHOULD_FIX attribution 译“署名”过窄 | 采纳 | 改“学术贡献归属” |
| 9 | SHOULD_FIX 批注译注未登记 | 采纳 | 补“批注译注”节 |
| 10 | NEEDS_VERIFICATION 22.5% 无文本存档 | 部分采纳：Claude 打开图 02 原图核对数值属实 | 清单注明以截图为据 |
| 11 | SHOULD_FIX 把“12%”与“12分”判为不相容理由不足 | 采纳 | 改写标题行与 README 勘误措辞：只否定“做对12%的任务” |
| 12 | SHOULD_FIX `images/README.md` 不存在 | 采纳 | 补写配图说明 |

### 第二轮（改后全文与卡片；Codex gpt-6-astra medium，Linux 只读）

结论 PASS，无 BLOCKER。

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | SHOULD_FIX “写代码的长任务落后最多”把跨测试分差说成能力结论 | 采纳 | 省流改“表中落后最多的几项都是写代码任务”，B②改“分差最大的三项都是写代码任务” |
| 2 | SHOULD_FIX 数学吠点卡单条缺主张者 | 采纳 | ①补“Calegari 说”，②改“这篇是客座文” |
| 3 | SHOULD_FIX PG19“回补提交增六成”缺基准与时间窗 | 采纳 | 改“同期回补提交较18版增六成” |
| 4 | SHOULD_FIX 钱包实验缺邮件验证码机制 | 采纳 | 改“在邮件验证码钱包实验中……自行提额”，清单备注写任务条件 |
| 5 | SHOULD_FIX “通稿称”标 L4／✅ 不合规则 | 采纳 | 改“据经济观察报报道”，结果 ⚠️ |
| 6 | SHOULD_FIX evidence-B 保留已证伪的“Code Agent 全部落后” | 采纳 | 在 evidence-B 三处追加勘误 |
| 7 | NEEDS_VERIFICATION 22.5% 无文本存档 | 部分采纳 | 视觉复核已看批注图 07 与底图（METRIC Partial、DOMAIN All domains、第 1 行 22.5%），未提异议；清单维持以截图为据 |
| 8 | NEEDS_VERIFICATION 精确发布时刻只在取证复述里 | 采纳 | 清单补原始字段：`a-article.html` datePublished、`c-blog.html` 第115行 article:published_time |

## 视觉复核

### 第一轮（Codex gpt-6-astra medium，Linux 只读；全部 34 张图）

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER ifeng-weibo 译注②无主张者，像本号确认的事实 | 采纳 | 译注②补“凤凰网科技原帖称”，标题改“凤凰网科技原帖说法” |
| 2 | SHOULD_FIX 深色截图（05–07）上正片叠底高亮使文字变暗 | 不采纳（本期） | 位置正确、文字仍可读；改模板混合方式属工具改动，另议 |
| 3 | SHOULD_FIX 06 译注②型号与分数粘连 | 采纳 | 改“GPT-6 Astra：11.8%，第一；Claude Opus 5.5：11.7%，第二” |
| 4 | SHOULD_FIX 12 译注②型号断行、漏星号含义 | 采纳 | 改“LongCat-2.5 为38.5；Claude-Opus-5 为53.9*（* 为公开来源成绩）” |
| 5 | SHOULD_FIX 13 表注截图字小 | 不采纳 | 原页表注即单行小字，底图已按原样裁；正文与吠点卡已写明含义 |
| 6 | SHOULD_FIX 省流卡 GDPx 一条用“；”拼接 | 不采纳 | `templates/cards/example.toml` 明确允许用“；”拼接不相邻原文，lint 逐段核对通过 |
| 7 | NICE_TO_HAVE 横版封面右侧留空 | 不采纳 | 居中 1:1 裁切完整，构图不影响阅读 |
