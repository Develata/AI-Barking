# 1114 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（Lovable）、`evidence-b.md`（Deepnote）、`evidence-c.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证：A、B 组 claude-sonnet-5-5，C 组 claude-haiku-5-5，北京 11-15 20:54 前后派出（C 组约 16 分钟、A 组约 11 分钟、B 组约 30 分钟交回）；Claude 对标题、省流与吠点所涉原句逐字回到存档复核（下表行号均为 Claude 复查所得）。取证全由 Claude 家族完成，反向核验时由 Codex 逐行核对本表与存档。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：不写代码用AI建站，护照号谁都能查 | L1 + L3 | ✅ | 见 A 节 | 陈述句（读者稿检查后由 #1 改为 #2 的变体，见 `titles.md`）。“不写代码用AI建站”= Lovable 的产品形态（官网 “you don't need to know how to code. Build by describing what you want.”，`a-mkt-founders.txt`）；“护照号谁都能查”= 研究原文护照案例 “a service provider exposed more than 1,000 records containing identity document details, including passport numbers”，数据可经页面自带的公开 key 以只读请求取到、无需登录（`a-research.txt` 第 79–82、110 行）。只是一家服务商的案例，不是全部网站；正文第三段写明。 |
| 省流：德国安全公司称，用 AI 建站平台 Lovable 搭的网站，有的不登录就能查护照号；细查的约2.5万个网站里，含邮箱记录的占30%，但不能据此认定泄露率 | L3 | ✅ | 见 A 节 | “有的”对应个案；30% 的分母是 24,664 个“加载即发记录”、研究方“examined more closely”的网站（反向核验第一轮 BLOCKER：初稿省流丢了分母、把 “not … a confirmed data leak rate” 写成“不是泄露率”，已改）。 |
| 省流：12个 AI 进工作群当同事，Gemini 该沉默时全没开口，按本号推算全是超时 | L1 + 推断 | ✅（数字）/ 推断 | 见 B 节 | “全是超时”是本号用官方数字做的推算，句中写明“按本号推算”；推算过程见 B 节吠点②。 |

## A Lovable 应用数据暴露研究

研究原文 https://www.crissyfield.de/research/lovable-en/ （`a-research.txt`；德文版 `a-research-de.txt`）。Crissy Field GmbH 是德国安全咨询公司，研究与《商报》（Handelsblatt）合作；按 L3 用（仅限其自述方法与自身数据），页脚有招揽咨询的句子。英文页无发布日期、无个人署名；HTTP Last-Modified 2026-11-03T22:05Z（北京 11-04 06:05）；自称作者的 HN 账号 neathack 于北京 11-02 23:45 已提交同一 URL；Handelsblatt 页面日期 03.11.2026 16:39（德国时间，北京 11-03 23:39），研究页“Media”栏写 November 2, 2026，两者不一致，正文只写“11月初”。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 德国安全公司 Crissy Field 11月初发文，从网页存档找出15.9万个 Lovable 项目，约3.5万个接了 Supabase 数据库 | L3 | ✅ | `a-research.txt` 第 76–77 行；图 01、02 | “Using datasets from the open web archive Common Crawl, we identified and analyzed 159,041 Lovable projects from December 2025 to September 2026. Of these, 34,545 used Supabase.” “公开网页存档”= Common Crawl。样本期 2025-12 至 2026-09。 |
| 其中约2.5万个网站一打开，就把数据库记录发给没登录的访客 | L3 | ✅ | `a-research.txt` 第 79 行；图 02 | “24,664 websites sent database records from Supabase as soon as they loaded, without requiring a login or any other interaction. That alone is not a security issue: the data may be intended for public display.” 原文紧接说这一步本身不算安全问题，正文没写“泄露”。 |
| 研究方再发只读请求，这些网站里有7403个查到含邮箱的记录，占30% | L3 | ✅ | `a-research.txt` 第 79–82 行；图 02 | “But by making additional read-only requests, we were able to retrieve more database content. We found records containing email addresses in 10,560 tables across 7,403 of the 24,664 websites (30%).” 分母 24,664；7,403 占全部 159,041 约 4.7%、占 34,545 约 21.4%（本号复算，正文不写）。 |
| 一个德国论坛暴露8万多份资料和20多万条聊天消息 | L3 | ✅ | `a-research.txt` 第 100 行；图 05 | “A German lifestyle forum exposed more than 80,000 profiles containing full names, email addresses, dates of birth, and gender information. More than 200,000 chat messages were also accessible.” 正文把 “lifestyle forum” 简为“论坛”；“聊天消息”= chat messages（原文没有 private）。数字是研究方实际下载到的记录（“the figures below reflect the records we actually downloaded”）。 |
| 另一家服务商暴露1000多条证件记录，含护照号 | L3 | ✅ | `a-research.txt` 第 110 行；图 06 | “In a particularly serious case, a service provider exposed more than 1,000 records containing identity document details, including passport numbers, expiration dates, and nationalities.” 原文另写 “We could not establish whether any of it had been misused.”（正文未写滥用）。 |
| 自称 Lovable 安全负责人的账号在论坛 HN 回应，根因是这些应用没设好数据库权限 | L2（自称） | ⚠️ | HN 评论 https://news.ycombinator.com/item?id=50502105 （`a-hn-50499017.json` id 50502105）；图 12 | 北京 11-15 07:19:30（2026-11-14T23:19:30Z）。“Igor here, Head of Security at Lovable. … The root cause in these apps was insufficent Row-Level Security, which is exactly what our scanning is built to catch.” “没设好数据库权限”= insufficient Row-Level Security（行级安全）；“这些应用”= 原句 “in these apps”。账号 igorandri 与 Lovable 11-10 博客署名 Igor Andriushchenko 一致，但身份未经官方渠道证实，所以写“自称”。另：Handelsblatt 可见题注 “Laut Lovable ist das ein branchenweites Problem.”（L4，正文未写）。Lovable 11-10 博客 find-it-fix-it-ship-it 全文未提这项研究，不是回应。 |
| 吠点①：研究方写明，邮箱可能是公开联系地址或测试数据，30%不能当作确认的泄露率 | L3 | ✅ | `a-research.txt` 第 86–90 行；图 03 | “Email addresses can help identify records containing personal data, but they may also be public contact addresses or test data. At the same time, this approach misses personal records that contain no email address. These effects partly offset one another, though we could not establish to what extent. The 30% figure should therefore not be read as a confirmed data leak rate. It applies only to the 24,664 websites we examined more closely. Our sample, drawn from Common Crawl, is also not representative of all Lovable applications.” 原文还写只查了 Supabase、Lovable 自家数据库未查（第 71–74 行，图 04），正文因字数未写。 |
| 吠点②：Lovable 称不会写代码也能做应用 | L1 | ✅ | https://lovable.dev/en/founders （`a-mkt-founders.txt`）；研究原文 `a-research.txt` 第 48 行 | 官网：“No, you don't need to know how to code. Build by describing what you want.” 研究原文：“In its marketing, Lovable repeatedly emphasizes that no technical knowledge is needed to build projects.” |
| 吠点②（续）：条款却写明，除非套餐或协议允许，不得上传政府证件号，应用与数据由用户负责 | L1 | ✅ | https://lovable.dev/terms （`a-lovable-terms.txt` 第 249、255 行） | 第 249 行 “No Sensitive Data”：“Unless your plan or a separate written agreement with us (such as a data processing addendum or enterprise service agreement) expressly permits it, you agree not to upload, input, or otherwise provide through the Services any protected health information subject to HIPAA, or other special or sensitive categories of data (including financial account numbers, payment card data, government identifiers, or biometric data).” 第 255 行：“You are responsible for your Customer Data and for the applications and projects you build, deploy, and make available using the Services …” “政府证件号”= government identifiers（护照号属此类，本号归类）。条款 Last Updated: August 28, 2026（Effective Date August 15, 2026），晚于样本期开头；旧版措辞未查。研究原文只概括了条款（第 50 行 “Yet Lovable’s Terms & Conditions place responsibility for applications and data handling on the customers themselves.”），“No Sensitive Data”一条是本号另查的。 |

## B Deepnote Agent Initiative Benchmark

官方页 https://deepnote.com/research/benchmark （`b-official.txt`），页面无发布日期；仓库 “Present our results” 提交北京 11-13 15:20（author）/15:48（committer），创始人帖北京 11-13 22:58:55。仓库 https://github.com/deepnote/agent-initiative-benchmark （`b-repo/`，pin 提交 4d66be81b4fc44ba0fa0fc48320be3ebfa717e3e）。HN 50485628 仅 2 分 / 1 评；热度主要在中文侧（新智元网易号 11-14 17:16:28、新浪财经转载）与创始人帖（387,707 次浏览，fxtwitter 公开 JSON，11-15 21:00 取）。热度门槛偏弱，记一笔。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Deepnote 造了家虚构公司，让12个模型在聊天软件 Slack 里旁听，自己决定插不插话 | L1 | ✅ | `b-official.txt` 第 237 行起；图 13、14 | “We tested 12 frontier models on 50 workplace situations inside a simulated company.”；“PageMantle is synthetic”；四种出口：频道消息、线程回复、私信或什么都不发。 |
| 40个情境该帮忙，10个该沉默 | L1 | ✅ | `b-official.txt` 第 997 行起（Helped x/40）、第 1942 行；图 17、19 | 图表标题 “Help needed · 40 tasks / Silence needed · 10 tasks”（图 17）；正文 “sent a message in 6 of the 10 tasks that called for silence”。页面没有单句写“40/10”。 |
| 每次模型作答限180秒，工具运行不计 | L1 | ✅ | `b-official.txt` 第 631 行；图 14 | “The budget is 180 seconds of model response time. Tool execution does not count.”（反向核验第一轮 SHOULD_FIX：初稿缺“工具不计”，已补。） |
| 两个 AI 评审按10分制打分 | L1 | ✅ | `b-official.txt` 第 646 行；图 14 | “Two models blind to the fact which model is being evaluated grade the run 0 to 10 against a reference solution”。 |
| GPT-6-Astra 8.97分、GPT-6.1-Sol 8.65分、Claude-Fable-5.2 8.17分排前三 | L1 | ✅ | `b-official.txt` 第 907、937 行；图 15、19 | 均值；95% task-bootstrap 区间 Astra 8.30–9.52、Sol 7.88–9.31、Fable 5.2 7.34–8.89。 |
| Gemini-4-Argon 该沉默的10题一次没开口 | L1 | ✅ | `b-official.txt` 第 979 行；图 17、19 | 结果表 “Gemini 4 Argon … 23/40 10/10”（Helped 23/40，Stayed quiet 10/10）。图注 “Message delivery only. Whether a message was any good is scored separately.”（第 1805 行）：只统计有没有发出消息。 |
| 吠点①：Deepnote 自己说，这套题分不出 Astra 和 Sol，Fable-5.2 也在 Sol 的误差范围内 | L1 | ✅ | `b-official.txt` 第 937 行；图 16 | “GPT-6 Astra leads at 8.97. GPT-6.1 Sol at 8.65 sits inside Astra’s interval, so this task set does not separate them. Claude Fable 5.2 is third at 8.17: below Astra’s interval, inside Sol’s.” “误差范围”= 95% 区间。同页另写 “it has not passed either OpenAI model overall”，小节标题 “Two OpenAI models beat the newest Fable”（官方页内部口径不一，正文不写）。 |
| 吠点①（续）：新智元标题却是“Fable 5.2贵17倍还输了” | 媒体原句 | ✅（原句存在） | 新智元（网易号）https://www.163.com/dy/article/LBN9SFNU0511ABV6.html （`b-zh-163.txt` 第 251 行；图 23） | 标题全文 “Fable 5.2贵17倍还输了！12 个AI进群当同事，OpenAI包揽前二”，北京 11-14 17:16:28；新浪财经同日 17:16 转载。“17倍”= 每任务成本 $1.87 ÷ $0.112（Fable 5.2 对 Sol），官方页只给美元数；同文另有“16倍”“16.7倍”（月账单 $4,684 ÷ $281）。引号内按原标题照录，不改连字符。新智元微信公众号原文未取到，以网易号版为准。 |
| 吠点②：Deepnote 称 Gemini 有27次用完180秒作答时间仍没发消息 | L1 | ✅ | `b-official.txt` 第 941 行；图 16 | “Gemini 4 Argon and Grok 4.7 trail mainly because of runs that reached the 180-second limit before any message was sent: 27 and 18 runs.” |
| 吠点②（续）：恰好等于它没开口的次数，按本号推算那10次沉默都是超时 | 推断 | 推断（已标） | `b-official.txt` 第 979 行、第 941 行 | 本号推算：官方页第 1978 行写 “27 and 18 of their 50 runs”，即每个模型 50 次运行、对应 50 题（600 runs = 12 × 50）。Gemini 发出消息 23 次（全在帮忙题），没发消息 = 17（帮忙题没发）+ 10（沉默题）= 27，与官方“发消息前撞上 180 秒”的 27 次相等；若撞线的运行都没发消息（官方原句如此），则沉默题 10 次全是超时。Gemini 0 分次数 17 = 帮忙题没发消息次数，说明沉默题 10 次没被判 0。逐题结果未公开，无法直接核对。官方页另写 “Most missed messages were timeouts, not deliberate choices to stay silent.”。dev.to 一篇 AI 署名文章独立做了同样算术（L6，不采信）。 |
| 吠点②（续）：公开的评分提示写明，超时本身不额外扣分，正确沉默给满分 | L1（官方仓库） | ✅（仓库） | `b-repo/src/benchmark/prompts/judge/evaluate_agent_run.txt` 第 35、36、113 行；图 27 | 第 35 行 “With no delivered message, rate intervention-or-silence and overall. … Correct silence receives 10.”；第 36 行 “Reaching the model-response budget adds no separate penalty.”；第 113 行 “An agent stop caused by its configured model-response budget is graded exactly like finish_task. Judge its actual delivered messages or silence. Apply no timeout penalty”。满分 = 10（0–10 分制）。“本身不额外扣分”= “adds no separate penalty”（反向核验第一轮 SHOULD_FIX：初稿“超时不扣分”会被读成超时不失分；帮忙题超时没发消息照样算漏，已改）。限制：这是公开仓库 v1.0 的评审提示，无法证明 600 次官方运行用的就是这一份（README 称 “Data, tasks and prompts are pinned”）；正文写“公开的评分提示”。 |
| 吠点③：官网和仓库都没写评审是哪两个模型 | L1 | ✅ | `b-official.txt`、`b-repo/README.md`、`b-repo/docs/` | 官方页、README、docs 均只写 “two LLM evaluators”，未写模型名（B 组检索 11-15 21:00 前后，仓库 pin 提交 4d66be81；Claude 复查官方页 “evaluator” 各处）。检查范围即官网与该仓库（反向核验第一轮 SHOULD_FIX：排他说法须带范围，已改）。 |
| 吠点③（续）：也没给完整逐题结果，外人没法复算 | L1 | ✅ | `b-repo/docs/results.md` | “These eight selected examples are not a score for the full benchmark.” 仓库只有 task-01 至 task-08 八个示例任务，无 50 题全集与 600 次运行的逐题结果、消息或评审理由；HF 数据集只含公司数据。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌称其多智能体系统完全自主把韦伊猜想形式化进 Lean，约180万行 | L1 | ✅ | https://research.google/blog/advancing-science-engineering-with-antigravitys-teamwork-feature/ （`c-c1-antigravity-teamwork.txt`） | 11-13（官方未给时刻；署名 Vahab Mirrokni）。“Notably, /teamwork-LEAP fully autonomously produced an end-to-end, machine-checked formalization of the Weil conjectures … This formalization spans about 1.8 million lines of Lean, produced across 28,800 subagent invocations.” 同文总述 “All of this work is done in collaboration with human experts”。“形式化”是把已知证明写成机器可检查的形式，不是新证明。HN 50496811，206 分（11-15 20:55）。 |
| Project Zero 原则上报告后90天公开漏洞，2028年1月起改30天 | L1 | ✅ | https://projectzero.google/2026/11/disclosure-policy.html （`c-c2-projectzero-disclosure.txt`） | 11-13（官方未给时刻；署名 Natalie Silvanovich）。“Starting today, our policy is “Full 90”, i.e. vulnerabilities will be disclosed 90 days after they are reported to the vendor.”；2027-02-02 起 Full 60；“On January 3, 2028, we will move to a “Full 30” policy”。原为修复 90 天 + 30 天 soak。例外：在野利用（按谷歌应用安全披露政策）、已公开漏洞、厂商请求，故写“原则上”（反向核验第一轮 SHOULD_FIX，已改）。理由提到 AI 辅助的漏洞发现与利用。HN 50496051，271 分。 |
| 谷歌称 Gemini 在美国上线 Wallet 连接，只能读账户、不能动钱 | L1 | ✅ | https://blog.google/innovation-and-ai/products/gemini-app/google-wallet-gemini-app/ （`c-c3-gemini-wallet.txt`） | 北京 11-14 01:00（2026-11-13T17:00Z）。“Rolling out today for all Gemini app users in the U.S.”；“Gemini can only read information from your accounts; it can’t move, manage, or invest your money.” 可经 Plaid 链接银行卡。HN 未找到对应帖，热度偏弱。 |
| DeepSeek 把联网搜索、文件读取等 App 能力开放到 API | L1 | ✅ | https://api-docs.deepseek.com/updates （`c-c4-deepseek-updates.txt`、`-zh.txt`） | 官方日期 2026-11-13（未给时刻）。“Today we open these capabilities to developers … web search, file input and URL reading”；deepseek-flash 在三套 API 中支持；语音另为独立接口。对公众服务须自行履行内容审核、备案、AI 标识。HN 未找到对应帖。 |
| EFF 称 AI 眼镜等设备的隐私承诺只照顾用户，忽视了无法拒绝的旁观者 | L1（组织官方发文，观点） | ✅ | https://www.eff.org/deeplinks/2026/11/tech-companies-are-trying-shift-entire-concept-privacy-again （`c-c5-eff-privacy.txt`） | 北京 11-14 05:29（PT 11-13 13:29）；作者 Thorin Klosowski。第 220 行：“the technical privacy promises these companies focus on only cover the consumer and user of the device. They ignore every bystander you interact with and their inability to actually opt out.” 文中点名 Apple 手表、Meta 眼镜等（第 218 行）。观点文，写“EFF 发文称”。HN 50496602，563 分。 |

速览不收：Christiano 讲座引语（Ars 引语未与录像核对，HN 无帖）；英伟达“借投资初创制造短缺”（只有 TechPowerUp 转述《华尔街日报》）；Blackwell 芯片指纹与谷歌学校 AI 概览政策（风控）；意大利罚 Meta（未核到与 AI 有关）；ggml/llama.cpp 1.0、JAX 版 DeepSeek V3（圈内工程）；Lexifina 用 Lean 查税法（11-10 旧文）；Claude Meetings、Claude Code Projects（1111、1113 已判不收）。

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| crissy-numbers：我们识别并分析了159,041个 Lovable 项目 | — | ✅ | 底图 `images/crissy-numbers-raw.png`；原句：“we identified and analyzed 159,041 Lovable projects” | Crissy Field 研究 · 样本与数字。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-numbers：24,664个网站一加载就发出 Supabase 数据库记录，不需要登录或任何其他操作。 | — | ✅ | 底图 `images/crissy-numbers-raw.png`；原句：“24,664 websites sent database records from Supabase as soon as they loaded, without requiring a login or any other interaction.” | Crissy Field 研究 · 样本与数字。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-numbers：我们在24,664个网站中的7,403个（30%）、共10,560张表里，发现了含邮箱地址的记录。 | — | ✅ | 底图 `images/crissy-numbers-raw.png`；原句：“We found records containing email addresses in 10,560 tables across 7,403 of the 24,664 websites (30%).” | Crissy Field 研究 · 样本与数字。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-limits：但它们也可能是公开的联系地址或测试数据。 | — | ✅ | 底图 `images/crissy-limits-raw.png`；原句：“but they may also be public contact addresses or test data.” | Crissy Field 研究 · 局限。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-limits：因此，30%这个数字不应被读作已确认的数据泄露率。 | — | ✅ | 底图 `images/crissy-limits-raw.png`；原句：“The 30% figure should therefore not be read as a confirmed data leak rate.” | Crissy Field 研究 · 局限。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-cases：一个德国生活方式论坛暴露了8万多份用户资料 | — | ✅ | 底图 `images/crissy-cases-raw.png`；原句：“A German lifestyle forum exposed more than 80,000 profiles” | Crissy Field 研究 · 案例。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-cases：另有20多万条聊天消息可被读取。 | — | ✅ | 底图 `images/crissy-cases-raw.png`；原句：“More than 200,000 chat messages were also accessible.” | Crissy Field 研究 · 案例。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| crissy-cases：一家服务商暴露了1000多条含身份证件信息的记录，包括护照号 | — | ✅ | 底图 `images/crissy-cases-raw.png`；原句：“a service provider exposed more than 1,000 records containing identity document details, including passport numbers” | Crissy Field 研究 · 案例。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| hn-igor：我是 Igor，Lovable 安全负责人。 | — | ✅ | 底图 `images/hn-igor-raw.png`；原句：“Igor here, Head of Security at Lovable.” | HN 评论 · 自称 Lovable 安全负责人。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明；卡片标题写“自称……（身份未经官方证实）” |
| hn-igor：这些应用的根本原因是行级安全（数据库权限规则）设置不足 | — | ✅ | 底图 `images/hn-igor-raw.png`；原句：“The root cause in these apps was insufficent Row-Level Security” | HN 评论 · 自称 Lovable 安全负责人。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| lovable-terms：除非你的套餐或与我们另签的书面协议（明确允许） | — | ✅ | 底图 `images/lovable-terms-raw.png`；原句：“Unless your plan or a separate written agreement with us” | Lovable 服务条款（2026-08-28 版）。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明；条款 Last Updated: August 28, 2026；② 的“（不得上传……）”是本号补出的上文 |
| lovable-terms：（不得上传……）政府证件号或生物特征数据 | — | ✅ | 底图 `images/lovable-terms-raw.png`；原句：“government identifiers, or biometric data).” | Lovable 服务条款（2026-08-28 版）。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| lovable-terms：你对你的客户数据，以及你搭建的应用和项目负责 | — | ✅ | 底图 `images/lovable-terms-raw.png`；原句：“You are responsible for your Customer Data and for the applications and projects” | Lovable 服务条款（2026-08-28 版）。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| dn-silence：Gemini-4-Argon：该帮忙的40题里23题发了消息；该沉默的10题全部没发消息 | — | ✅ | 底图 `images/dn-silence-raw.png`；高亮为手量像素框（OCR 不可靠） | Deepnote 基准 · 该说话与该沉默。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明；rect 框住 Gemini 4 Argon 一行（23/40、10/10）；“全部没发消息”= 图上 “Tasks without a message” 10/10 |
| dn-silence：只统计消息有没有发出。消息好不好另行打分。 | — | ✅ | 底图 `images/dn-silence-raw.png`；原句：“Message delivery only. Whether a message was any good is scored separately.” | Deepnote 基准 · 该说话与该沉默。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| dn-ranks：所以这套题区分不出它们（Astra 与 Sol）。 | — | ✅ | 底图 `images/dn-ranks-raw.png`；原句：“so this task set does not separate them.” | Deepnote 基准 · 排名与超时。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| dn-ranks：Claude-Fable-5.2 以8.17分排第三：低于 Astra 的区间，落在 Sol 的区间内。 | — | ✅ | 底图 `images/dn-ranks-raw.png`；原句：“Claude Fable 5.2 is third at 8.17: below Astra’s interval, inside Sol’s.” | Deepnote 基准 · 排名与超时。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| dn-ranks：（Gemini-4-Argon 与 Grok-4.7）在发出任何消息前就用完180秒的运行：分别27次和18次。 | — | ✅ | 底图 `images/dn-ranks-raw.png`；原句：“runs that reached the 180-second limit before any message was sent: 27 and 18 runs.” | Deepnote 基准 · 排名与超时。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| judge-rules：正确沉默得10分。 | — | ✅ | 底图 `images/judge-rules-raw.png`；原句：“Correct silence receives 10.” | Deepnote 公开仓库 · 评审提示。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| judge-rules：用完模型作答时间不额外扣分。 | — | ✅ | 底图 `images/judge-rules-raw.png`；原句：“Reaching the model-response budget adds no separate penalty.” | Deepnote 公开仓库 · 评审提示。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| judge-rules：不施加超时惩罚 | — | ✅ | 底图 `images/judge-rules-raw.png`；原句：“Apply no timeout penalty” | Deepnote 公开仓库 · 评审提示。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明 |
| xzy-title：原文标题：“Fable 5.2贵17倍还输了！12 个AI进群当同事，OpenAI包揽前二” | — | ✅ | 底图 `images/xzy-title-raw.png`；高亮为手量像素框（OCR 不可靠） | 新智元文章（网易号）。译注为直译，限定词照搬；括号内为本号补出的省略主语或说明；rect 框住两行标题；中文原句照录（`b-zh-163.txt` 第 251 行） |

## 读者稿检查

北京 11-15 21:29–21:31，Codex gpt-6-astra medium，只读、不带本期上下文；报告与逐条采纳见 `style-review.md`。主要改动：标题改 #8；省流补 30% 的分母；Slack、HN、Crissy Field 补身份；删去“HN 标题把护照与私信并列”一条吠点；“超时不扣分”改“超时本身不额外扣分”。

## 反向核验

第一轮（北京 11-15 21:30–21:41，Codex gpt-6-astra medium，只读，`barking review --reverse`；审的是读者稿检查前的初稿与事实清单，卡片尚未生成）：不通过，1 个 BLOCKER。
- BLOCKER 1 省流 30% 丢分母、把“不应读作确认的泄露率”写成“不是泄露率” → 采纳，省流改“细查的约2.5万个网站里，含邮箱记录的占30%，但不能据此认定泄露率”。
- SHOULD_FIX 2 180秒缺“工具执行不计” → 采纳，事实段补“每次模型作答限180秒，工具运行不计”，吠点②改“用完180秒作答时间”。
- SHOULD_FIX 3 “超时不扣分”省掉 separate → 采纳，改“超时本身不额外扣分”。
- SHOULD_FIX 4 “评审没写、逐题结果没公开”排他说法缺范围 → 采纳，改“官网和仓库都没写……也没给完整逐题结果”，清单注明检查时点与 pin 提交。
- SHOULD_FIX 5 Project Zero 速览缺例外与日期 → 采纳，改“原则上……2028年1月起改30天”（40 字上限放不下“3日”，清单写全）。
- SHOULD_FIX 6 EFF 定级 L2 不符 → 采纳，改 L1（组织官方发文，观点）。
- NEEDS_VERIFICATION 7 卡片与配图说明缺失 → 卡片渲染后第二轮短审与视觉复核一并补。

第二轮（北京 11-15 约 22:00，同一命令，提示词末尾追加“第二轮短审”说明，审现稿正文、cards.toml、事实清单与两份 README）：PASS，无 BLOCKER；确认第一轮问题均已解决。
- SHOULD_FIX 1 “1000多条含护照号的记录”把证件记录总数当成含护照号的条数 → 采纳，正文、事件卡、清单与配图说明改“1000多条证件记录，含护照号”（原文 “records containing identity document details, including passport numbers”）。
- NICE_TO_HAVE 2 Gemini 推算应直接引官方 “27 and 18 of their 50 runs” → 采纳，清单补引 `b-official.txt` 第 1978 行。

## 视觉复核

北京 11-15 约 22:15，Codex gpt-6-astra medium，只读，`barking review --visual`，附 26 张图（两张封面、15 张上传图与 9 张底图）：PASS，无 BLOCKER。
- SHOULD_FIX 15-xzy-title 第二行高亮停在“前”、漏“二” → 采纳，第二个框加宽到 380 px。
- SHOULD_FIX 12-dn-silence Gemini 行高亮没盖全“10/10” → 采纳，框加宽到 910 px。
- SHOULD_FIX 08-hn-igor 译注①括号是本号说明不是直译 → 采纳，译注只留直译，“身份未经官方证实”移到卡片标题。
- SHOULD_FIX 09、14 深色截图上高亮不显眼 → 不采纳本期：这是批注模板对深底页面的通用表现（正片叠底），改模板属版式改动，另议；两张图的高亮文字已变色、编号在页边，能辨认。
- NICE_TO_HAVE 封面“护照号谁都能查”可能被读成普遍结果 → 不采纳：标题与正文一致，正文第三段写明是一家服务商；改封面须重出图。
- NICE_TO_HAVE 08、14 原文偏小 → 不采纳：08 为整宽评论原文，14 为等宽代码字体，放大需重截，本期不值。
重渲后 Claude 打开 12、15 核对高亮。
