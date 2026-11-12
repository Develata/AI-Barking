# 1111 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`（玛雅遗址与作者更正）、`evidence-b.md`（AWS 模拟器）、`evidence-c.md`（Grok 应用无需登录）、`evidence-d.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。D 组（claude-haiku-5-5）取证于北京 11-12 17:53–18:08（harness 计 14 分钟）。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI找到玛雅遗址？作者自己改口了 | L1 | ✅ | 见 A 节 | 问号指原帖与流行说法“AI 智能体找到（未测绘的）玛雅遗址”；“改口”= 作者更新段把 discovered 改为 rediscovered（“rediscovered it rather than discovered it”），标题改为 “(Re)discovered”。读者稿检查首选此条（原选定 #1，改选）。 |
| 省流：48个 AI 智能体“发现”的玛雅遗址，作者更正为重新发现 | L1 | ✅ | 见 A 节 | 引号表示原说法存疑；“rediscovered it rather than discovered it”。 |
| 省流：AWS 称模拟器代码全由 AI 写、没人审，规格和标准答案是人选的 | L1 | ✅ | 见 B 节 | “AWS 称”= 官方文作者署名 AWS Sr. Principal Engineer；“全由 AI 写”= “100% written by autonomous AI agents”；“没人审”= “Our team hasn’t written or reviewed any of the code”；“规格和标准答案是人选的”见 B 节。省流卡只取前半句“AWS 称模拟器代码全由 AI 写、没人审”。 |
| 省流：安全公司称3.5万个 Grok 做的应用不用登录就能打开 | L3 | ✅ | 见 C 节 | 3.5万 = 35,578 取整（报告标题段 “more than 35,000 apps built with Grok that anyone with the link could open”）；分母是仍在线的应用，正文给出。 |

## A 玛雅遗址：作者更正为重新发现

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| AI 研究者 Pietro Miotti 博文称 | L1 | ✅ | https://pietromiotti.com/latest/maya-site/ （`a-blog.txt` 第 20 行）；作者站首页（`a-wb-home.txt` 第 10 行） | “Disclaimer: I'm not an archaeologist.”；首页 “I’m an AI researcher.” HN 原提交标题的 “Google SWE” 出自提交者，作者 X 简介自述在 Google 做研究，博文未写雇主，正文不写雇主。 |
| 让48个 AI 智能体逐张看约1200平方公里的公开激光雷达图像 | L1 | ✅ | 博文第 32、42 行 | “They turned the lidar strips (about 1,200 km² of jungle) into images, and 48 agents went through them one by one”；“There was no special model or classifier involved”。更新版写 public lidar，原版为 NASA G-LiHT 数据（图注 2013）。图 02。 |
| 找到一处墨西哥官方登记里没有的玛雅遗址，从起念到收到官方回信不到72小时 | L1 | ✅ | 博文第 12 行；INAH 回函图（图 04） | 博文：“found a Maya site in Mexico that wasn't in the official register”；INAH 回函：“sin que se identificara un sitio arqueológico registrado en la ubicación señalada”。 |
| 随后，Miotti 在文首更正：遗址已在2025年 Estrada-Belli 等人的论文里测绘过，智能体查文献时漏了，是“重新发现” | L1（作者自述） | ✅ | 博文第 10 行（更新段） | “Update (11 November): it turns out the site had already been mapped in Estrada-Belli et al. 2025. The agents missed it while going through the literature, so they rediscovered it rather than discovered it”。时间：原站只标 “11 November”（作者时区未知），更新上线时刻无法确定（Wayback 北京 11-11 23:56 快照尚无更新段；作者 X 更正帖北京 11-12 00:39:47），正文第一轮后改为“随后”，不写具体时刻（反向核验第一轮 6）。论文：Estrada-Belli、Canuto、Šprajc、Fernández-Diaz，Journal of Archaeological Science: Reports 66，105288（Crossref/OpenAlex 元数据）。论文页被 ScienceDirect 验证页拦，“该遗址就在论文第 14 页”只有作者自述与 HN 评论，未独立核实，正文只写作者说法。图 01。 |
| 官方回信只说登记系统里该位置没有已登记遗址 | L1 | ✅ | INAH 回函（图 04，作者博客公布） | “Se realizó la revisión correspondiente de la información proporcionada en el Sistema de Información Geográfica de esta Subdirección, sin que se identificara un sitio arqueológico registrado en la ubicación señalada en su informe.” 检索范围是该司地理信息系统（登记），不是文献。“不等于没人发现过”由作者更正本身印证（“it just wasn't in the register … yet”）。 |
| 更正前博文写“据目前所知没人报告过”，文末却已引了 Estrada-Belli 等人2025年的论文 | L1（Wayback 存档） | ✅ | https://web.archive.org/web/20261110201337/https://pietromiotti.com/latest/maya-site/ （`a-wb-blog-20261110.txt` 第 10、104 行；快照北京 11-11 04:13:37） | 原版引言：“my AI agents discovered a Maya site in Mexico that, as far as anyone can tell, no one had reported before.”；原版文末：“others used lidar to estimate how many people lived in the Maya lowlands (Estrada-Belli et al. 2025 [doi …105288])”。同一 DOI。更新后改为 “mapped settlements across the Maya lowlands, including this very site”。图 07、05。 |
| 截至12日傍晚，作者在社交平台的原帖浏览超130万次，更正帖约4000次 | L2 平台计数 | ✅（数字为平台公开计数的瞬时值） | 作者 X 帖（fxtwitter 公开 JSON：`a-x-2120219901814043087.json`、`a-x-2120557109410775430.json`），约北京 11-12 18:00 取 | 核验对象是平台显示的计数本身。原帖 views 1,320,891、likes 20,005（发于北京 11-11 02:19:51）；更正帖 views 4,093、likes 112（北京 11-12 00:39:47），文本 “Update: the site had already been discovered. It appears in a 2025 paper …”。平台名不进正文，写“社交平台”。两帖发布时长不同（原帖早约22小时），比较只说明更正传播远不及原帖。图 10、11。 |
| TechRadar 仍写“发现”（截至12日傍晚） | L4 页面表述 | ✅（核验对象是该页当时的表述，不证明考古事实） | https://www.techradar.com/ai-platforms-assistants/google-researcher-discovers-unmapped-maya-site-in-mexico-using-48-ai-agents-project-takes-less-than-72-hours-after-agents-scour-nasa-lidar-data （`a-techradar.txt`，北京 11-12 17:56 取） | 标题 “Google researcher discovers unmapped Maya site in Mexico using 48 AI agents …”；全文 grep “rediscover/correction/update” 无更正（唯一 “update” 命中是无关的 Fitbit 链接）。发于北京 11-11 23:00。图 12。 |
| （已删，读者稿检查后）“72小时”是从起念到收到墨西哥官方回信 | L1 | ✅ | 博文第 12 行 | “It took less than 72 hours to go from a random thought while traveling to an official reply from the Mexican government.” 小标题 18—21 October。博文没写智能体运行时长。 |
| （已删，读者稿检查后）博文没说48个智能体用的什么模型 | L1 | ✅ | 博文全文（`a-blog.txt`） | 只在头脑风暴处列 “a few agents (Gemini, ChatGPT, Claude and Grok)”，翻译建议出自 Claude、复原图出自 Gemini；48 个看图智能体所用模型未写（A 组通读、Claude grep “model” 仅见 “no special model or classifier”）。 |

## B AWS 模拟器：智能体写、没人审

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 亚马逊云 AWS 宣布开源 AWS Emulator，在自己电脑上模仿 AWS 云服务，供程序员测试 | L1 | ✅ | 官方文 https://www.aboutamazon.com/news/aws/how-autonomous-ai-agents-built-aws-emulator （`b-official-post.txt` 第 85 行）；README（`b-gh-readme-raw.txt` 第 7、140 行） | “Today, we released AWS Emulator, a local emulator of AWS services that you can run on your machine to build and test your AWS applications.” 官方文页面 JSON-LD 2026-11-11 17:25:18Z（北京 11-12 01:25）；GitHub release v1.0.322 更早（北京 11-11 08:32），仓库公开时刻未验证，故正文第一轮后不写开源时刻（反向核验第一轮 7）。许可 CC0-1.0，`vendor/` 目录为 Apache-2.0。图 13、20。 |
| 工程师 Matthew Goldberg 撰文称，代码“完全由自主智能体编写，团队没写也没审过任何一行” | L1 | ✅ | 官方文（`b-official-post.txt` 第 85 行） | “The emulator was written entirely by autonomous agents. Our team hasn’t written or reviewed any of the code.” 署名 Sr. Principal Engineer。图 13。 |
| （热度，未进正文）仓库帖在 Hacker News 上800多分 | 热度（未进正文） | — | https://news.ycombinator.com/item?id=50456631 （`b-hn-50456631.json`、`b-heat.json`） | 北京 11-12 17:58 取 847 分 / 235 评（Claude 17:40 现查 842 分）。 |
| （已删，读者稿检查后）早期版本的智能体还自己造了个 Python 解释器 | L1 | ✅ | 官方文（`b-official-post.txt` 第 137 行） | “My personal favorite was from an earlier version, when they decided to build their own Python interpreter to support AWS Lambda functions. We’ve since given them guidance to reuse existing …” 是早期版本，不是公开仓库现版本。图 17。 |
| 规格和标准答案是人选的：照 AWS 公开的接口说明写，测试拿真 AWS 当标准答案 | L1 | ✅ | 官方文（`b-official-post.txt` 第 95、119 行） | “the AWS API Models are our spec and the actual AWS cloud is our reference implementation.”；“Agents devise these tests from publicly released AWS API Models and verify each one against real AWS before submitting.” “人定的”= 作者团队选定规格与参照物、写最初提示词（Kiro 三步提示词）、设计三队分工与门禁（B2-2、B2-5）。图 14、15、18。 |
| 没人审，但合并前有智能体审查 | L1 | ✅ | 官方文（`b-official-post.txt` 第 123 行） | “This gate obviously blocks changes that don’t compile. But it also includes agents that review against coding standards, verify the code is secure, and ensure that the change did not regress fidelity.” 即“没审”指没有人工审查，审查由智能体做。图 16。 |
| AWS 每版都公布与真 AWS 是否一致的测试结果，本号合计11日那版约4.3万项通过约56% | L1 数据 / 本号统计 | ✅ | https://github.com/aws/aws-emulator/releases/download/v1.0.322/scores.json.gz （sha256 eca6d69b…08cd，B 组与 Claude 各自下载，哈希一致） | 核验状态：原始文件由 B 组（Sonnet）与 Claude 各自下载重算，结果一致，二者同属 Claude 系；跨模型复核者（Codex）无网络取不到原件，只核到汇总存档 `b-fidelity-scores-v1.0.322-summary.json` 与算术（56.46%）。Claude 重算：`units` 共 42,937 项，pass 24,243、fail 18,614、skipped 80，通过率 56.46%。文档：“A fidelity score is, for each service, the number of fidelity tests that the emulator passes out of the total for that service”“We publish fidelity scores with each release”（`b-doc-fidelity.txt` 第 38、58 行）。官方只按服务给“k of n”，不给总数；总数是本号合计，故正文写“按本号合计”。把 unit 当作一项测试是推断（官方文第 119 行称 “We have tens of thousands of fidelity tests across all of AWS”，量级一致），正文用“项”。scoredAt 2026-11-11T00:09:58Z，对应提交 7753983。“11日那版”= v1.0.322（GitHub release 2026-11-11T00:32:13Z，北京 11-11 08:32）；文档要求同一测试版本才可比较、分数是快照，故正文点明版本（反向核验第一轮 3）。文档另称控制面最接近 AWS、数据面与 IaC 支持因服务而异，未实现的操作返回 HTTP 501。 |
| README 写明别用于正式生产 | L1 | ✅ | README（`b-gh-readme-raw.txt` 第 7 行） | “Don’t use it for production workloads or to test security.” 图 21。 |

## C Grok 应用：无需登录即可打开

Claroc 报告按 L3（第三方研究方自述）用，正文一律写“Claroc 报告称”。报告页只给日期 2026-11-10（官方未给时刻）；Ars Technica 北京 11-11 05:12 已引用。HN 热度弱（三帖 5–11 分、0 评），靠 Ars Technica、The Decoder、THN、Cybernews 等十余家报道。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 安全公司 Claroc 报告称，调查时仍在线的37,571个用 Grok 生成的应用中，35,578个（94.7%）不用登录就能打开 | L3 | ✅ | https://blog.claroc.io/ghostshares-grok-build-apps/ （`c-report.txt` 第 60、124 行） | “We tried 43,551 Grok app addresses. 37,571 were still online, and 35,578 of those (94.7%) opened for anyone, with no sign-in.” 方法段另写子域名枚举得 43,601 个名称（第 216 行），与导语 43,551 差 50、报告未解释，故正文不写尝试总数。分母是“仍在线”的应用，不是全部 Grok 应用；样本只含被动 DNS 里看得见的应用。图 25、26。 |
| 含 SpaceXSI 员工做的24个（报告后已下架） | L3 | ✅ | 报告第 62、66 行 | 导语 “24 of the open apps were built by SpaceXSI and X staff”；第 66 行 “Its own staff still had 24 apps open to anyone, and SpaceXSI took them down after our report. It has not done the same for the apps other Grok users and organizations left open.”（X 已并入 SpaceXSI，平台名不进正文）。“报告后已下架”只指这 24 个，其他用户的应用未下架（反向核验第一轮 2）。 |
| （已删，视觉复核与第二轮后）Grok 母公司 SpaceXSI | L4 | ⚠️（公司关系据报道，未存一手公告；正文已删） | Ars Technica（`c-ars.txt` 第 210 行）；Claroc 报告标题 “Grok Apps Exposed xAI’s Business Secrets” | Ars：“SpaceXSI (the name of the now-merged SpaceX and xAI)”。Grok 是 xAI 的产品，xAI 并入 SpaceXSI。反向核验提到 SpaceX 官网有收购公告（https://new.spacex.com/updates），本期未存档。 |
| （未进正文）其中一个是列了594个客户的销售区域图 | L3 | ✅ | 报告第 62、86 行 | “a sales territory map of 594 accounts”；“a sales territory map of 594 strategic accounts and their pipeline”。594 是这一张区域图（图 2 “North Central Territory Mapper”）的账户数，不是 SpaceXSI 全部客户。图 36（报告自带打码）。 |
| Claroc 用自建应用测试称，Grok 写的登录代码先把来人加进团队再核对，没受邀的账号也能改删记录 | L3 | ✅ | 报告第 158 行 | “The code Grok wrote adds whoever signs in to the team before it checks whether they are on it, so the check always passes. A Google account that was not the owner’s and had never been invited signed in, saw every customer, changed one record and deleted another.” 报告图 6 是“Illustrative, simplified from the code Grok wrote for us”，不是原始代码。图 28。 |
| “能打开”指访问首页没被拒绝，Claroc 每个只访问一次 | L3 | ✅ | 报告第 124、216 行 | “Only 1,993 (5.3%) answered with HTTP 401, the error an app set to Only you or Invited users returns.”；“We loaded the first page of each app once, from the public internet and signed in to nothing”。“拒绝访问”= HTTP 401。图 26、30。 |
| 能改删只在它自建的两个应用上验证过 | L3 | ✅ | 报告第 166 行 | “We proved this only on our own two apps, and did not sign in to, write to or test anyone else’s.” |
| 可打开的应用中约2%含高危内容，是 Claroc 的估算 | L3 | ✅ | 报告第 126 行 | “by our estimate roughly 2% held material serious enough to rate high or critical.” 分级标准与样本量未公布。同段 “About 1 in 10 held sensitive material” 与方法段 “roughly 650 to 800 apps with private data”（第 218 行）口径关系报告未解释，第一轮后正文删去“约十分之一”（反向核验第一轮 11）。 |
| （已删，读者稿检查后）SpaceXSI 称员工账号默认仅自己可见，只见 Claroc 转述 | L3 转述 | ⚠️（只见报告转述，已删） | 报告第 208 行；Ars Technica（`c-ars.txt` 第 268 行） | “It said employee accounts are set to Only you by default”，出处是 SpaceXSI 回答报道该研究的记者（报告转述），原始回应未找到。Ars 里唯一的厂商回应来自 Cursor 代表（“apps are private by default …”），措辞不同。Claroc 反驳：“In the paid account we used … Anyone with the link was already selected.” 图 29。 |
| 它本身卖 AI 应用安全服务 | L3 | ✅ | 报告 “Where Claroc fits” 一节（`c-report.txt` 第 234–240 行） | “Claroc secures the apps people build with AI while they are built and after they go live”，文末有服务介绍与预约入口。The Decoder（`c-thedecoder.txt` 第 126 行，L4）也写 “It sells security solutions for AI-built apps.”，仅作旁证。图 32。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌推打分模型 Gemini-Decision-1，输出 token 不计费 | L1 | ✅ | https://blog.google/innovation-and-ai/technology/developers-tools/gemini-decisions-api/ （`d-gemini-blog-opencli.txt`），11-11（官方未给时刻） | “Rather than generating answer text token by token, Gemini Decision 1 scores your candidate options in a single forward pass … at $0.10 per 1M input tokens with zero output tokens billed.” “打分模型”= 对候选选项给概率与置信度、不逐 token 生成。中文稿流传的“9 项第一 / 92.2% / ECE 0.0250”在博客、AI Studio 文档、模型页、定价页均未找到（D 组查四页），不收。延迟 50–100 ms 是中位数、限 ≤16K 上下文，未进速览。HN 50456791 429 分 / 130 评（11-12 17:55）。 |
| Ring 室内无人机美国定价249.99美元，官方称获 FCC 授权前不得销售 | L1 | ✅ | https://www.aboutamazon.com/news/devices/ring-drone-cam-rover-cam-security-robot （`d-ring-amazon.txt`），11-10（官方未给时刻） | “The Ring Drone Cam will begin shipping to customers in the U.S. on January 27, 2027, for $249.99.”；脚注 “Ring Drone Cam has not been authorized as required by the rules of the Federal Communications Commission. This device is not, and may not be, offered for sale or lease, or sold or leased, until authorization is obtained.” “室内无人机”= 新闻稿 “Ring Drone Cam is a flying indoor security camera”（第 77 行）。“美国定价”= 美国 2027-01-27 起发货的售价（第 165 行），尚未开售（反向核验第一轮 10）。风控：涉美国联邦监管机构 FCC，属外国政府相关表述，交 Develata 终审决定（反向核验第一轮 12）。单次续航 8 分钟为产品页脚注 “under controlled conditions”，未进速览。HN 热度低（最高 9 分），多家媒体报道。 |
| AlphaGenome-2 预测 DNA 变异对人体特定细胞类型的影响 | L1 | ✅ | https://deepmind.google/blog/alphagenome-2-deciphering-how-cells-read-dna-to-better-understand-disease/ （`d-alphagenome2-blog.txt` 第 498 行），11-11（官方未给时刻） | “AlphaGenome 2, our next-generation model that predicts how DNA sequences and their variations impact specific human cell types”（第二轮后改为“人体特定细胞类型”，保留 human 与 types）。第一轮前写的“识别因果变异30%，对照16%”撤下：30%/16% 限 TenK10K 数据集，据官方论文是 90% 精确率下的召回率，16% 对应 AG1-sc，40 字放不下这些限定（反向核验第一轮 1）。官方称仅供研究、未经临床或诊断验证。Nature 新闻 11-11 报道（L4）。HN 13 分。 |
| 台积电批准投 Anthropic（按 IPO 价）、OpenAI 各至多5亿美元 | L1 | ✅ | 台积电 SEC Form 6-K https://www.sec.gov/Archives/edgar/data/0001046179/000104617926000797/tsm-boardx202611xx.htm （`d-tsmc-sec-6k-board-opencli.txt` 第 95 行），11-10 董事会（官方未给时刻） | “Approved the investments in (1) Anthropic, PBC at its per share initial public offering price in an amount not to exceed US$500 million to acquire Anthropic's common shares; and (2) OpenAI Group PBC in an amount not to exceed US$500 million to acquire OpenAI's preferred stock.” “批准”不等于交割；Anthropic 部分按其 IPO 每股价格（第二轮后写进速览）。“至多”= not to exceed。台积电官方中文决议 PDF（第五项）亦一致（复核者独立检索，本期未存档）。另一项不超过 5 亿美元的半导体生态投资公司决议与两家无关，不合并。路透（401）、台积电新闻稿（403）未取得正文。 |

速览不收：OpenAI×Upwork（“非因果”限定压不进 40 字，HN 0 命中）；AFT/UFT×Anthropic（新闻稿 “automatically applies” 与协议 “If requested” 机制有张力，HN 0 命中）；Anthropic “Worse Together”（HN 3 分）。

## 批注译注

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| （原站标注 11 月 11 日更新）原来这处遗址已在 Estrada-Belli 等人 2025 年的论文中测绘过。 | L1 | ✅ | `a-blog.txt` 第 10 行；图 01 | “Update (11 November): it turns out the site had already been mapped in Estrada-Belli et al. 2025.” 日期为原站标注，作者时区未知。 |
| 智能体查文献时漏掉了它，所以是重新发现，而不是发现。 | L1 | ✅ | 同上 | “The agents missed it while going through the literature, so they rediscovered it rather than discovered it”（视觉复核后高亮扩到整句） |
| 从旅途中一个念头到收到墨西哥官方回复，用了不到 72 小时。 | L1 | ✅ | `a-blog.txt` 第 12 行 | “It took less than 72 hours to go from a random thought while traveling to an official reply from the Mexican government.” |
| （更正前原文）据目前所知，此前没人报告过这处遗址。 | L1（Wayback） | ✅ | `a-wb-blog-20261110.txt` 第 10 行；图 07 | “as far as anyone can tell, no one had reported before.”（视觉复核后高亮含限定语） |
| （更正前文末）另有研究者用激光雷达估算玛雅低地生活过多少人（Estrada-Belli 等，2025）。 | L1（Wayback） | ✅ | `a-wb-blog-20261110.txt` 第 104 行 | “others used lidar to estimate how many people lived in the Maya lowlands (Estrada-Belli et al. 2025)”。第一轮后删去译注里追加的“引的正是……那篇论文”（反向核验第一轮 4）；同一 DOI 的对照见 A 节。 |
| 模拟器完全由自主智能体编写。我们团队没写、也没审过任何一行代码。 | L1 | ✅ | `b-official-post.txt` 第 85 行；图 13 | “The emulator was written entirely by autonomous agents. Our team hasn’t written or reviewed any of the code.” |
| 门禁里还有智能体按编码标准审查、核查代码安全，并确认改动没让保真度倒退。 | L1 | ✅ | `b-official-post.txt` 第 123 行；图 16 | “it also includes agents that review against coding standards, verify the code is secure, and ensure that the change did not regress fidelity.” |
| 官方称每天修掉数千个问题，有数百个智能体同时在改代码。 | L1 | ✅ | `b-official-post.txt` 第 123 行；图 16 | “We’ve seen thousands of issues fixed every day as hundreds of agents concurrently work on the codebase.” 导语（第 63 行）另写 “Thousands of autonomous AI agents wrote every line”，与“数百个”并存，译注不混写（反向核验第一轮 4）。 |
| AWS Emulator 用于开发和测试。不要用于生产环境，也不要用来测试安全性。 | L1 | ✅ | `b-gh-readme-raw.txt` 第 7 行；图 21 | “AWS Emulator is for building and testing. Don’t use it for production workloads or to test security.” |
| 在测试时仍在线的 37,571 个 Grok 应用中，35,578 个（94.7%）任何人都能打开。 | L3 | ✅ | `c-report.txt` 第 124 行；图 26 | “Of the 37,571 Grok apps still online when we tried them, 35,578 (94.7%) opened for anyone.” |
| 只有 1,993 个（5.3%）返回 HTTP 401，即拒绝访问。 | L3 | ✅ | `c-report.txt` 第 124 行 | “Only 1,993 (5.3%) answered with HTTP 401, the error an app set to Only you or Invited users returns.” 35,578 + 1,993 = 37,571。 |
| Grok 写的代码先把登录者加进团队，再检查他在不在团队里，所以检查永远通过。 | L3 | ✅ | `c-report.txt` 第 158 行；图 28 | “The code Grok wrote adds whoever signs in to the team before it checks whether they are on it, so the check always passes.” 只在 Claroc 自建应用上测（第 166 行）。 |
| 图 6 是示意图，由 Grok 为 Claroc 写的代码简化而来，不是原始代码。 | L3 | ✅ | `c-report.txt` 第 162 行；图 28 | “Figure 6. Illustrative, simplified from the code Grok wrote for us.” “不是原始代码”是 “Illustrative, simplified” 的直译说明。 |

## 反向核验

### 第一轮（北京 11-12 约 18:45，codex-reviewer：gpt-6-astra，effort medium，只读；`barking review --reverse --run`）

结论：2 BLOCKER、8 SHOULD_FIX、2 NEEDS_VERIFICATION，Claude 逐条核实后处理：

1. BLOCKER AlphaGenome“30%对16%”缺数据集、指标（90% 精确率下的召回率）与基线：采纳，速览改为不带数字的“预测 DNA 变异对特定细胞的影响”。
2. BLOCKER 24 个员工应用已下架却并入“仍在线”：采纳，正文与事件卡改为“调查时仍在线……（报告后已下架）”。
3. 56% 缺版本：采纳，改为“本号合计11日那版”（v1.0.322）。
4. 两条译注混入编辑判断：采纳，删去“引的正是……那篇论文”与“导语写的是数千个智能体”。
5. 批注译注未登记：采纳，补“批注译注”一节，逐条列出。
6. 博文更正时刻是推断：采纳，正文改为“随后”，译注注明“原站标注 11 月 11 日”。
7. 用新闻稿时间代替开源时间：采纳，改为“发文宣布开源”，去掉时刻。
8. 分级与 ✅ 混用：采纳，拆出“Grok 母公司”一行（L4 ⚠️），TechRadar 与平台计数行注明核验对象，转述行改 ⚠️，L3/L4 合并行拆开。
9. 两份 README 不存在：审查开始时尚未写成，现已补齐，第二轮复核。
10. Ring 价格缺美国范围：采纳，改为“美国定价”。
11. “约十分之一”与“650–800”口径未解释：采纳，正文删去“约十分之一”，只留“约2%高危”。
12. FCC 风控：Ring 一条涉美国联邦监管机构，事实有官方脚注；交 Develata 终审决定是否保留。


### 第二轮（北京 11-12 约 18:34，codex-reviewer：gpt-6-astra，effort medium，只读）

结论：PASS / no blocker；4 SHOULD_FIX、2 NEEDS_VERIFICATION，全部处理：

1. “Grok 母公司”只有 L4：采纳，正文与卡片删去，只留“SpaceXSI 员工”（视觉复核同判 BLOCKER）。
2. 台积电漏 Anthropic 按 IPO 价：采纳，速览改为“台积电批准投 Anthropic（按 IPO 价）、OpenAI 各至多5亿美元”。
3. AlphaGenome“特定细胞”丢了 human 与 types：采纳，改为“人体特定细胞类型”。
4. 吠点卡“这篇论文”无指代：采纳，改为“Estrada-Belli 等人2025年的论文”。
5. 56% 原始数据未跨模型重算：如实记入 B 节该行（Codex 取不到原件；同家族两次重算一致）。
6. FCC 与“墨西哥官方回信”：交 Develata 终审。后者是作者与墨西哥文物机构的登记往来，不涉政治指控。

## 视觉复核

### 第一轮（北京 11-12 约 18:32，gpt-6-astra，effort medium，只读；24 张图）

1 BLOCKER、5 SHOULD_FIX、2 NICE_TO_HAVE：

1. BLOCKER 事件卡“Grok 母公司”证据只到 L4：采纳，已删。
2. 05 号图第②条高亮比译注窄：采纳，quote 扩到 “The agents missed it …”。
3. 06 号图第①条高亮漏了 “as far as anyone can tell”：采纳，已扩。
4. 吠点卡①“这篇论文”无指代：采纳（同第二轮 4）。
5. 吠点卡②浏览量缺采样时刻：采纳，改为“截至12日傍晚，作者在社交平台的原帖……”。
6. 吠点卡（Grok）②约2%缺分母：采纳，改为“可打开的应用中约2%……”。
7. NICE_TO_HAVE 15 号图流程图字小：不改，原文段与译注已能读懂；流程图是报告自带示意。
8. NICE_TO_HAVE 横版封面标题偏小：不改，居中 1:1 裁切安全区内完整可读。

