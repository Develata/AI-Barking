# 1031 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`、`evidence-b.md`、`evidence-c.md`、`evidence-d.md`、`evidence-e.md`，截图在 `images/`（“图 NN”指取证截图编号）。取证于北京 11-02 02:30–03:30。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：AI给鼠脑打针？ChatGPT额度合并 | — | ✅ | 见 B、A 节 | 问号问“AI 做脑部手术”的说法（CEO 首帖、IT 之家）；“打针”＝脑内注射；“额度合并”＝12月起 ChatGPT 与 Codex 共用额度。 |
| 一家公司 CEO 称，让 Claude 智能体给癫痫模型小鼠脑内注射基因疗法 | L2 | ✅ | 见 B 节 | CEO 自述（L2）；首次手术执行模型为 Sonnet-5（Claude）。 |
| 开颅由兽医做 | L2 | ✅ | 见 B 节 | 对比帖串原句（切口、开颅、缝合）。 |
| 每一步有兽医把关 | L2 | ✅ | 见 B 节 | 对比帖串 “supervised every step”；首帖 “gated every irreversible step”。第二轮反向核验 SHOULD_FIX 2 后补。 |
| 12月起，Plus、Pro、Business 的 ChatGPT 和 Codex 共用额度 | L1 | ✅ | 见 A 节 | 第一轮反向核验 SHOULD_FIX 6 后补回三种套餐名。 |
| Instant 聊天不扣 | L1 | ✅ | 见 A 节 | |
| DeepSeek 公开的“超级智能体”自称研究原型 | L1 | ✅ | 见 C 节 | |

## B AI 智能体给活鼠脑内打针

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Exin Therapeutics CEO 10月28日发帖称 | L2 | ✅ | Gabriel Ocana-Santero 海外社媒帖 status/2115442964574908902（`b-fx-2115442964574908902.json`，图 13） | created_at “Wed Oct 28 13:58:00 +0000 2026”＝北京 10-28 21:58（美西 10-28 上午）。CEO 与联合创始人身份见 WIRED（`b-wired.txt` 第80行）。 |
| 他们让 AI 智能体操控自制的电动手术台 | L2 | ✅ | 主帖；帖串 status/2115442965464150135 | 主帖 “access to a motorize surgery rig with cameras”；“this is a custom-made motorized surgery rig we built on arduino” |
| 给一只癫痫模型小鼠往脑内注射基因疗法，药和脑区都由它选 | L2 | ✅ | 主帖（图 13，后半句在截断之后，以 JSON 为准）；帖串 status/2115442966508474690 | “We gave it 19 gene therapy vectors in a spreadsheet, a mouse model of epilepsy … It picked the drug. It picked the brain region. It derived the coordinates from three papers. Then it drove the stereotaxic arms into the brain and injected 250 nanolitres”。整段在“CEO … 发帖称”之下。 |
| 切口、开颅和缝合由兽医做 | L2 | ✅ | 对比帖串 status/2116285917883437096（`b-fx-thread-2116285926121091382.json`，图 18 截断之后） | “A licensed veterinary surgeon still performed the incision, craniotomy and suturing, and supervised every step.” 首次手术帖串：“Watching our surgeon do the craniotomy, which the agent is not allowed to do yet for safety reasons”。WIRED 只写开颅由兽医做。麻醉由谁做未找到。 |
| 不可逆步骤都要兽医把关 | L2 | ✅ | 主帖（图 13、15） | “A licensed vet gated every irreversible step and held a physical kill switch to stop the AI agent.” 第二轮反向核验 SHOULD_FIX 2 后补。 |
| ①据 CEO 称，先试的 Opus-5.5 一发现涉及生物就拒绝 | L2 | ✅ | 帖串 status/2115442969964589227（图 14） | “Opus 5.5 refused immediately when detected bio.” WIRED 版本不同：称 Opus-5.5 拒绝时说 “something on my output is telling me I shouldn't do this”（L4，未写）。第一轮反向核验 BLOCKER 3 后补“据 CEO 称”。 |
| ①动手的是 Sonnet-5 | L2 | ✅ | 同上（截断之后，以 JSON 为准）；WIRED（图 20） | “Sonnet 5 ran the live surgery start to finish.”；WIRED “The team switched to using Claude Sonnet 5, which agreed to proceed.” |
| ①官方已标为旧版（Sonnet-5） | L1 | ✅ | https://platform.claude.com/docs/en/models/sonnet-5/overview （`b-sonnet5.*`，图 23） | 页面标 “Legacy”，“Although Claude Sonnet 5 is still available, you should consider migrating to …”。弃用页状态仍为 Active，最早退役 2027-06-30（未写）。北京 11-02 02:32 取。 |
| ②IT 之家标题写“让 AI 自己做脑部手术” | L4（夸大实例；核验对象是该媒体确实这样写） | ✅ | https://www.ithome.com/1/020/737.htm （`b-ithome.txt` 第68行，图 22；批注图 exin-ithome） | 完整标题“让 AI 自己做脑部手术，初创公司成功让智能体给小鼠治疗癫痫”，北京 11-01 15:15:13。 |
| ②同文转述该公司披露：开颅和缝合都是真人医生做的 | L4 | ✅ | IT 之家（`b-ithome.txt` 第90行） | “IT之家从该公司披露了解到，AI 只负责了注入药物的过程，开颅和缝合都是真人医生做的。”第二轮反向核验 BLOCKER 1：原“其实 AI 只管注药”把自述写成本号裁决，且注射不足以否定“手术”说法，改为转述 IT 之家同文；吠点落在标题的“自己”。 |
| ③CEO 称随后 Claude、GPT、Grok 各做一只，选的疗法、脑区都不同 | L2 | ✅ | 对比帖串 status/2116285912628052110、…915425620232（图 17） | 北京 10-31 05:47:34。“We gave Claude Sonnet 5, GPT-6.1 Sol and Grok 4.7 the same 19 gene therapy vectors … All three picked different therapies, different mechanisms and different parts of the brain.”；“N=1 per model, on three different animals”。Claude＝Sonnet-5，GPT＝GPT-6.1-Sol，Grok＝Grok-4.7。 |
| ③疗效未知 | L2 | ✅ | 对比帖串第5条（图 19，monitoring 句在截断之后）；WIRED | “we don't yet know which will actually reduce seizures … we'll be monitoring their behavior and brain activity over the coming weeks”；WIRED “They’re now waiting for the therapy to take effect before assessing its impact on seizures.” 第一轮反向核验 NEEDS_VERIFICATION 7 后删去“没有论文”。 |
| ③他说别当正式评测 | L2 | ✅ | 对比帖串第5条（图 19；批注图 exin-too-early） | “This is N=1 per model, on three different animals … Take this as a commentary on how the models reason and behave in a real laboratory, not as a proper benchmark.” |

## A ChatGPT 和 Codex 共用额度

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 帮助页11月1日更新 | L1 | ✅ | https://help.openai.com/en/articles/20001625-understanding-chatgpt-and-codex-usage-limits （`a-help-usage-rendered.txt`、`a-help-usage-article.json`，图 01） | 页面数据 updatedAt 北京 11-01 00:17:48（UTC 10-31 16:17:48）；页头只显示相对时间。首次发布时刻未找到。第三方 Tibor Blaho 北京 10-30 06:04 已在结账页见到同类文字（L6，未写）。 |
| 12月起，Plus、Pro 和 Business 套餐的额度由 ChatGPT 和 Codex 共用 | L1 | ✅ | 同上（图 02） | “Starting in December, Plus, Pro, and Business plans’ usage allowance will be shared across ChatGPT and Codex as we bring the agentic capabilities of Work into the main ChatGPT experience.” 只写月份，无日期与年份。用户转贴邮件写 “in the coming weeks”（L6，未写）。 |
| 按 token 计 | L1 | ✅ | 同上（图 03） | “Your usage for ChatGPT and Codex will be measured in tokens …” |
| Instant 聊天不扣 | L1 | ✅ | 同上（图 03） | “Instant chat will not draw from your usage.” Instant 是模型选择器里的档位，选 GPT-6 时由 GPT-6 Sol 驱动（GPT-6 模型页，图 06）。 |
| ①合并后每档额度多少，帮助页没写 | L1 | ✅ | help 20001625（北京 11-02 取） | 该页无各档合并后的预算。现行 Work/Codex 机制另有公开表（Codex 定价页 “credits per 1M tokens”，`a-rendered-codex-pricing-rendered.txt` 第218、231行），不是合并后的预算。第一轮反向核验 SHOULD_FIX 8 后改为“合并后……帮助页没写”。 |
| ②Business 帮助页写明 Instant 是“几乎不限”，仍防滥用；选着 Instant，自动推理也可能扣推理额度 | L1 | ✅ | https://help.openai.com/en/articles/12003714-chatgpt-business-models-and-limits （图 05） | “Virtually unlimited access applies to Instant chat and remains subject to safeguards against misuse. … Automatic reasoning may use your reasoning allowance even while Instant remains selected.” 该页 updatedAt 北京 11-01 07:15:31，该段何时加入未找到。 |
| ③免费版、Go、企业版、教育版，帮助页都没提 | L1 | ✅ | help 20001625 | 全文只点名 Plus、Pro、Business；未出现 Free、Go、Enterprise、Edu。 |
| 已删：爱范儿稿文末称“无限畅聊的时代要结束了”，开头却写 Instant 不扣额度 | L4 | — | https://news.qq.com/rain/a/20261101A05QJV00 （`a-rendered-ifanr-qq-rendered.txt` 第36、137行） | 属实，为压字数删去（读者稿检查称不在乎）。 |

## C DeepSeek 超级智能体原型

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 10月31日，DeepSeek 在 GitHub 公开 dsh-superagent | L1 | ✅ | https://github.com/deepseek-ai/dsh-superagent-prototype （GitHub API `c-*` JSON、`c-git-log.txt`，图 35） | 仓库创建北京 10-31 17:32:46，唯一提交作者时间 20:05:39，作者 “DeepSeek Harness Team”。 |
| 它盯着一个仓库 | L1 | ✅ | README（`c-readme*`，图 25） | “dsh-superagent is a proactive coding agent for one GitHub repository … It listens to GitHub and, optionally, Feishu.” |
| 被允许的人派活，就在独立会话里干完并回复 | L1 | ✅ | README；docs/000（图 36） | “When someone allowed asks the agent for work, it opens a work item, works on it in its own agent session and directory, and answers in the original conversation, for example with a pull request.” 第一轮反向核验 SHOULD_FIX 4 后恢复“被允许的人”。 |
| ①说明文档第一句：仅供参考的研究原型，不是 DeepSeek 官方产品 | L1 | ✅ | README（图 25；批注图 ds-readme） | “Research prototype for reference only, not an official DeepSeek product.”“第一句”指 README 首段引文框首句。 |
| ②公开原型对没人明确要求的活，默认只记不做，切 auto 模式并匹配规则才做 | L1 | ✅ | docs/002-authorization.md（`c-doc002.txt` 第98、101行，图 27；批注图 ds-accept-mode）；`c-src-policy.ts.txt` 第62–64行 | 表 `observe` (default)：“Inferred candidates are assessed; a positive assessment is recorded as ignore marked "would accept", and nothing is accepted”；`auto`：“A positive assessment of an inferred candidate accepts it”；“Explicit requests work in every mode.”；“A proactive accept requires at least one matching rule.” 默认启用 copy、visual 两条规则。第一轮反向核验 SHOULD_FIX 5 后限定为“没人明确要求的活”、“匹配规则”。 |
| ③新浪上一篇自媒体稿标题写“这 AI 牛马，没人 @ 都自己干活” | L5（夸大实例；核验对象是该稿确实这样写） | ✅ | https://k.sina.cn/article_5953740931_162dee083067043yzs.html （`c-sina-niuma.txt`，图 31；批注图 sina-niuma） | 自媒体“AI信息Gap”，新浪托管，页面 11.01 09:35（未标时区）。该稿第25行称这是 DeepSeek Harness 内部用户群里的实例。 |
| ③正文才交代这是 DeepSeek 内部用户群里的实例 | L5 | ✅ | 同上（`c-sina-niuma.txt` 第25行；批注图 sina-niuma） | “DeepSeek Harness 的内部用户群里，就潜伏着这么一位 AI 牛马。”DeepSeek 原文同样把“无人 @ 主动修复”写成内部实例（`c-wechat-original.txt` 第30、34行），README 称公开版与内部版不同。第二轮反向核验 SHOULD_FIX 3：原“同文后段又写得切 auto 模式”把内部实例与公开原型两种对象当成自相矛盾，改为指出标题把内部实例的行为安在开源原型上。 |
| ④部署文档写明，它以运营者的系统账号、完全访问权限运行，能读其主目录 | L1 | ✅ | docs/006-deployment.md（`c-doc006.txt` 第19行，图 28；批注图 ds-deploy） | “Both presets run with `danger-full-access` as the operator's OS user … A worker can execute programs by path and read the operator's home directory.” “它”＝两种预设（worker 与 router）。 |
| 已删：新浪稿称它“7x24 小时在线”，DeepSeek 原文没有这个说法 | — | ❌ | `c-wechat-original.txt` 第44行；`c-article.txt` 第31行 | 第一轮反向核验 BLOCKER 1：文章写“持久：始终在线”/“Persistent: It is always online”，含义相同，原结论不成立，删去。 |

## 批注译注

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 昨晚，一个 AI 智能体给小鼠做了脑部手术 | L2 | ✅ | CEO 首帖（`b-fx-2115442964574908902.json`） | “Last night an AI agent performed brain surgery on a mouse.” |
| 我们给它一张列着19种基因治疗载体的表格、一只癫痫模型小鼠、一台带摄像头的电动手术台，目标只有一个：减少癫痫发作 | L2 | ✅ | 同上 | “We gave it 19 gene therapy vectors in a spreadsheet, a mouse model of epilepsy, access to a motorize surgery rig with cameras and one goal: reduce seizures.” |
| Opus 5.5 一检测到涉及生物，就立刻拒绝 | L2 | ✅ | 帖串（图 14） | “Opus 5.5 refused immediately when detected bio.” |
| Opus 5 在水和方格纸上从头到尾跑完演练，到真手术开始时停下 | L2 | ✅ | 同上 | “Opus 5 ran the entire dry run end to end on water and graph paper, then stopped at the start of the live surgery.” |
| 标题：让 AI 自己做脑部手术，初创公司成功让智能体给小鼠治疗癫痫；同文：AI 只负责注药，开颅和缝合都是真人医生做的 | L4 | ✅ | IT 之家（`b-ithome.txt` 第68、90行） | 见 B 节②。 |
| 说实话，现在下结论太早 | L2 | ✅ | 对比帖串第5条 | “Honestly, too early to say.” |
| 每个模型只做了一只，三只不同的动物，设备问题也略有不同；请把它当作模型在真实实验室里怎么推理、怎么做事的观察，不是正式评测 | L2 | ✅ | 同上 | “This is N=1 per model, on three different animals, with slightly different equipment problems. Take this as a commentary on how the models reason and behave in a real laboratory, not as a proper benchmark.” |
| 12月起，Plus、Pro 和 Business 套餐的用量额度由 ChatGPT 和 Codex 共用 | L1 | ✅ | help 20001625（图 02） | 见 A 节。 |
| 用掉多少额度，取决于你选的模型和交给它的任务 | L1 | ✅ | 同上（图 03） | “How much of your allowance you use will depend on the model you choose and the task you ask it to do.” |
| Instant 聊天不扣这份额度 | L1 | ✅ | 同上 | “Instant chat will not draw from your usage.” |
| Instant 聊天几乎不限，仍受防滥用措施约束；即使选着 Instant，自动推理也可能用掉你的推理额度 | L1 | ✅ | Business 页（图 05） | 见 A 节②。 |
| 仅供参考的研究原型，不是 DeepSeek 官方产品；可能仍有 bug；所以与我们的内部版本、与文章所写的实现都不完全相同 | L1 | ✅ | README（`c-readme*`，图 25） | “It may still contain bugs”；“so it is not identical to our internal version or the implementation described in”。 |
| 观察模式（默认）：评估推断出的活；评估通过的也只记作“本来会接”，这些推断出的活一律不接；先用观察模式，看过记录的决定后再切到 auto | L1 | ✅ | docs/002（`c-doc002.txt` 第98、101行） | “Start in observe and switch to auto after reviewing the recorded Decisions.” |
| 标题：刚刚，DeepSeek 开源「超级智能体」！这 AI 牛马，没人 @ 都自己干活；正文：DeepSeek Harness 的内部用户群里，就潜伏着这么一位 AI 牛马 | L5 | ✅ | 新浪托管稿（`c-sina-niuma.txt` 第1、25行） | 见 C 节③。 |
| 两种预设都以完全访问权限、用运营者自己的系统账号运行；执行者能按路径运行程序、读取运营者的主目录；要隔离，就用只放智能体凭据的专用系统账号 | L1 | ✅ | docs/006（`c-doc006.txt` 第19行） | “A worker can execute programs by path and read the operator's home directory. For isolation, use a dedicated OS account containing only the agent's credentials.” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Karpathy 称用 Claude 做网站，收 HN 网友推荐的6454个视频 | L2 | ✅ | https://karpathy.bearblog.dev/hacker-tube/ （`d-karpathy-hackertube-post.txt`） | “I took a hold of my Opus 5.5 and gave it the task”；“In total, we get 6,454 YouTube videos that have been recommended by a few people on HN from 2007-2026 across 150 topics.” 数字为作者自报。扫描“约两小时做成”不写：原文只有 “This takes ~2 hours now.”，所指不明。博文 10-31，官方未给时刻；HN 50319510 1878 分（北京 11-02 02:32）。站名平台词不入卡片。 第一轮反向核验 SHOULD_FIX 10 后加“称”（作者自报）。 卡片标签 HackerTube＝站名（博文标题 “HackerTube”，站点 hackertube.org，`d-karpathy-hackertube-post.txt` 第13行）。 |
| Wiz 称10月至少9家 AI 开发平台披露安全事件，多数仍在分析 | L3（厂商自家研究） | ✅ | https://www.wiz.io/blog/black-october-for-ai-developer-platforms （`e-wiz-black-october.txt`） | “At least nine AI Developer Platforms have disclosed incidents over the course of the month”；“Currently, most of these security incidents are still being analyzed”。名单含 Northflank（其状态页称 “Users have reported unauthorised use of AI API keys”，`e-northflank-history.txt`）。页面日期 10-30，官方未给时刻。 |
| SenseNova 开源 Loop-9B 预览版：约9B 参数，推理约14.6B | L1 | ✅ | https://github.com/OpenSenseNova/SenseNova-Loop （`e-sensenova-gh.txt`）；https://huggingface.co/sensenova/SenseNova-Loop-9B-Preview （`e-sensenova-hf.txt`） | “~9B total parameters; ~14.6B inference parameters”；Apache-2.0；“[2026.10.30] We release SenseNova-Loop-9B-Preview”。HF 页参数显示 “10B params”。“商汤”归属两页均未写，正文用组织名 SenseNova。对比只在图 `e-sensenova-benchmark.png`：8项中6项高于 Qwen3.5-27B，Terminal Bench 2.1（40.45 对 47.19）与 τ³-Banking（18.56 对 19.59）低于；RaccoonPPTBench 图注为内部自研。正文未写对比。 |
| Grok 文档称11月7日起，所列模型的 API 文本输出默认加隐形水印 | L1 | ✅ | https://docs.x.ai/developers/advanced-api-usage/watermarking （`e-xai-docs-watermarking.txt`） | “Starting November 7, 2026, responses from these models include a watermark. It's on by default, and you can opt out per model”；“The watermark is not visible.” 范围为 API 文本生成端点，未提网页与 App。页面署名 SpaceXSI，正文不写公司名。 第405行限 “supported Grok models”；第514行 “Some models may be locked on for your team”，故正文不写可关闭。第一轮反向核验 BLOCKER 2 后改为“所列模型”、删去“可按模型关闭”。 |
| 安全研究方称 OpenAI 10月26日已修复 ChatGPT 分享链接漏洞 | L3（研究方自述） | ✅ | https://www.parameter.ai/labs/we-found-a-gap-at-openai （`e-parameter-gap.txt`） | “OpenAI fixed the issue on October 26 and classified the report as P1.” OpenAI 自身公告未找到；文中未见野外利用。文章 10-28。HN 50314699 357 分。 |
| 据 WIRED 报道，苹果 CEO 称不为 AI 而推出 AI，但会用 AI | L4 | ⚠️ | https://www.wired.com/story/i-spent-30-minutes-with-apples-ceo-now-i-know-why-john-ternus-got-the-job/ （`d-wired-ternus.txt`） | “We don't want to ship AI for AI's sake.”；同段 “We’re absolutely going to leverage AI to do that.” 北京 10-30 23:00（-04:00 11:00）。 |

## 反向核验

### 第一轮（正文与事实清单；Codex gpt-6-astra medium，Linux 只读；审的是读者稿检查修改前的版本）

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER DeepSeek 吠点“仓库和文章里都没有 7x24 的说法”被原文反驳：文章写“持久：始终在线” | 采纳（`c-wechat-original.txt` 第44行核实） | 删去该吠点；③改为新浪稿标题与同文后段的自相矛盾，②补“没人 @ 也主动修 bug 的是内部实例” |
| 2 | BLOCKER Grok 水印省掉“所列模型”与部分模型锁定不能关 | 采纳（`e-xai-docs-watermarking.txt` 第405、514行核实） | 速览改“所列模型的 API 文本输出默认加隐形水印”，删“可按模型关闭” |
| 3 | BLOCKER Exin 的 L2 自述在省流、吠点①③脱离署名 | 采纳 | 省流“一家公司 CEO 称”，①“据 CEO 称”，③“CEO 称随后……” |
| 4 | SHOULD_FIX 删掉了“被允许的人” | 采纳 | 恢复“被允许的人派活” |
| 5 | SHOULD_FIX 观察模式扩大到全部任务、“自己写规则”写成普遍前提 | 采纳（`c-doc002.txt` 第101行、`c-src-policy.ts.txt` 第62–64行核实） | ②限定“没人明确要求的活”，“匹配规则才做” |
| 6 | SHOULD_FIX 省流丢套餐范围 | 采纳 | 省流补 Plus、Pro、Business |
| 7 | NEEDS_VERIFICATION “没有论文”否定证据不足 | 采纳 | 删去 |
| 8 | SHOULD_FIX “怎么换算，OpenAI 没公布”缺新旧机制与范围 | 采纳 | 改“合并后每档额度多少，帮助页没写” |
| 9 | SHOULD_FIX 批注译注未逐条入清单 | 采纳 | 新增“批注译注”节 |
| 10 | SHOULD_FIX 速览 Karpathy 未署自述、框架成本缺小样本条件 | 采纳 | Karpathy 加“称”；框架成本条目压不进40字，撤出速览 |
| 11 | SHOULD_FIX L4 标 ✅ | 采纳 | 苹果条改 ⚠️；IT 之家、新浪、爱范儿为夸大实例，备注写明核验对象是“确实这样写” |
| 12 | SHOULD_FIX 配图说明的取证日期与批注图来源日期不一致 | 采纳 | 配图说明分写新闻日期与取证时段 |

### 第二轮（改后全文；Codex gpt-6-astra medium，Linux 只读）

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER “其实 AI 只管注药”把自述写成本号裁决，注射不足以反驳“手术” | 采纳 | 吠点②改为 IT 之家标题“让 AI 自己做脑部手术”与同文转述的“开颅和缝合都是真人医生做的” |
| 2 | SHOULD_FIX 删掉了“兽医把关每个不可逆步骤” | 采纳 | 事实段补“不可逆步骤都要兽医把关”，省流补“每一步有兽医把关” |
| 3 | SHOULD_FIX 新浪稿两段对象不同（内部实例与公开原型），不是自相矛盾 | 采纳 | 吠点③改为“正文才交代这是 DeepSeek 内部用户群里的实例”，批注图改框标题与该句 |
| 4 | NICE_TO_HAVE 速览标签 HackerTube 未入文中事实列 | 采纳 | 该行备注写明站名 |

## 视觉复核

### 第一轮（Codex gpt-6-astra medium，Linux 只读；全部 30 张图）

| # | 意见 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER ds-accept-mode 译注“什么都不接”扩大到全部任务、漏“评估通过”条件 | 采纳 | 译注改“评估推断出的活；评估通过的也只记作‘本来会接’，这些推断出的活一律不接” |
| 2 | SHOULD_FIX exin-ithome 译注未完整对应高亮 | 采纳 | 译注改为完整标题与“开颅和缝合都是真人医生做的” |
| 3 | SHOULD_FIX sina-niuma 高亮含整条标题，译注只对应后半 | 采纳 | 高亮框缩到标题后半；其后第二轮反向核验改吠点③，批注图重做为“完整标题＋内部用户群句”，高亮与译注逐一对应（Claude 目视核对） |
