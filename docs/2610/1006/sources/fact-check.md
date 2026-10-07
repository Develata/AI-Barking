# 1006 事实核验

期次目录按 Develata 美国本地日期（10月6日，美东）。本机时区 EDT（UTC−4），北京时间 = 本机 + 12 小时（Python 确认）。

## 选题变更记录

- 原定主帖：Claude 日记报警 + SemiAnalysis 订阅限额实测（取证见 `evidence-a.md`、`evidence-b.md`）。DeepSeek 融资因全部数字来自匿名信源（L4），降为速览。
- 北京时间10月7日上午，Develata 要求改一条为 OpenAI 数学发布（HN 565 分）。SemiAnalysis 移出本期（证据存档保留，见 `evidence-b.md`、`b-semianalysis.txt`、`b-figures/`），数学条为主帖第一。
- 数学条由 Claude 直接取证（`m-*` 文件），未经代理；反向核验时请逐行对照存档。

## 主帖一：OpenAI 数学手稿

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间10月7日清晨，OpenAI 在 GitHub 公开722篇数学手稿 | L1 | ✅ | github.com/openai/math（`m-oai-README.md`、`m-oai-CONTENTS.md`）；GitHub API（`m-github-commit.json`、`m-github-repo.json`）；博文 `m-openai-post.txt` | 唯一提交 `2026-10-06T21:58:50Z` = 北京 10-07 05:58（美东 10-06 17:58）；仓库创建 21:47:02Z。提交时间不等于仓库转公开时间；《科学美国人》称 “6 P.M. EDT” 公开（L4，= 北京 10-07 06:00）；两者都落在北京 10-07 清晨，正文只写到“清晨”。OpenAI 博文只写 “October 6, 2026”。CONTENTS 首行 “722 manuscripts covering 372 result families.” |
| 分为372组结果 | L1 | ✅ | `m-oai-README.md`、`m-oai-CONTENTS.md` | README：“722 manuscripts organized into 372 families. A family groups related papers, which may include a principal result, companion arguments, consequences, or alternative proofs.” CONTENTS 编号到 376（如 376 号 Navier–Stokes 通用计算），但 “**NNN.** ” 条目计数为 372，编号不连续。NYT 标题写 377（`m-` 未存档，仅搜索摘要），不在正文使用。 |
| 称由一个未发布的内部模型产出 | L1 | ✅ | README：“produced by an internal OpenAI model”；“using an unreleased internal OpenAI model” | 博文：“produced by an internal frontier model”；“working to responsibly release the model”。 |
| （已从正文删去，压字数；仅留在批注图 41）平均每个结果约用3小时 ChatGPT Pro 思考算力 | L1 | ✅ | README：“On average, each result used three hours of ChatGPT Pro thinking compute with that model.”；博文：“The average result used the equivalent compute of roughly three hours of ChatGPT Pro thinking.” | 以 ChatGPT Pro 思考时长折算的算力单位，用的是未发布模型；不是用户用公开模型可复现的时长。README 另写 “posed approximately 4,000 problems” 并经聚合与重要性筛选——不能算成“成功率”。截图 `../images/41-oai-readme-procedure-raw.png`。 |
| 其中包括拟黎曼猜想：OpenAI 称证明了所有狄利克雷 L 函数（含黎曼 ζ 函数）在实部大于7/8的半平面没有零点（s=1 的极点除外），并附 Lean 形式化 | L1 | ✅（OpenAI 声称；Lean 未由本号复跑） | CONTENTS 003：“Proves that every Dirichlet L-function, including ζ(s), is zero-free in Re s > 7/8, resolving the quasi-Riemann hypothesis.”；论文 PDF（`m-oai-qrh-7-8.pdf`，摘要 “with the principal pole at s = 1 allowed”）；`m-oai-lean-docs-003.md`；挑战文件 `m-oai-QuasiRiemannHypothesis.lean` | 反向核验后删去“最受关注”（关注度排名无一手依据），改为“其中包括”。“s=1 的极点除外”对应摘要 “with the principal pole at s = 1 allowed”（极点不是零点，写出以忠实范围）。Lean：`formalization.yaml` 收录该论文，Comparator 挑战文件陈述为 `(7/8 : ℝ) < s.re → riemannZeta s ≠ 0`，允许公理仅 propext、Quot.sound、Classical.choice；本号未编译验证。截图 `../images/43-oai-qrh-abstract-raw.png`（论文首页）。 |
| ①拟黎曼猜想不是黎曼猜想。对黎曼 ζ 函数，要把这条边界推到1/2才相当于黎曼猜想 | — | ✅（数学定义） | `m-oai-lean-docs-003.md`：“The quasi-Riemann hypothesis asks for a fixed zero-free half-plane Re s>θ with θ<1.”；Clay 研究所对黎曼猜想的表述限于 ζ | 黎曼猜想等价于 ζ 在 Re s > 1/2 无零点。拟黎曼猜想只要求存在某个 θ<1，7/8 是这次声称取得的具体界。对全部狄利克雷 L 函数推到 1/2 是广义黎曼猜想，故正文限定“对黎曼 ζ 函数”（反向核验 BLOCKER 2 采纳）。 |
| ②722是手稿数，不是“一夜攻克722个难题”：按 OpenAI 的分法是372组结果，手稿目录名里的日期跨9月10日至10月6日 | L1 | ✅ | `m-oai-CONTENTS.md` 中 preprints 目录名日期统计 | 目录名日期：9-10（2）、9-17、9-18、9-22（3）、9-23（177）、9-24（193）、9-25（90）、9-26（53）、9-27（51）、9-30（5）、10-1、10-2、10-3（6）、10-4（24）、10-5（112）、10-6（1）。“一夜攻克722个数学难题”出自 36氪标题（新浪转载 `m-sina-36kr.html`，2026-10-07 09:35），正文引作流行说法，不点名。 |
| ③形式化清单只列出主结论已形式化的162篇，审核状态字段写着“unchecked” | L1 | ✅（仓库自述） | `m-oai-lean-formalization.yaml`（注释 “Catalog of papers with a formalized main result.”；`sources` 162 条；`status.scope: "Partial progress."`；`automation.methods: agent`；`review.status: unchecked`） | schema v0.4（`m-formalization-schema-v0.4.json`）把 `review.status` 定义为审核状态（示例 “unchecked \| agent-reviewed \| self-assessed \| peer-reviewed \| author-verified \| other/freeform”），未规定等级排序；unchecked 表示未经审核，不表示编译失败。162 为论文数，`main_results` 185 个声明（反向核验复算一致）。 |
| 仓库自述，未形式化的结果“可能有问题” | L1 | ✅ | README：“Some of the unformalized results could have issues.” | 截图 `../images/40-oai-readme-verification-raw.png`。 |
| ④仓库写明，ζ 函数无零区域与 CM 阿贝尔簇霍奇猜想两项不在统一流程内，11/12版文稿还经人工编辑 | L1 | ✅ | README：“Exceptions to this fixed procedure include work on a zero-free region for the Riemann zeta function and proof of the Hodge Conjecture for CM abelian varieties. Additionally, the writeup for the Re(s) > 11/12 zero-free region for the Riemann zeta function was human edited for readability.” | 原文 “include”，例外可能不止两项；“无零区域的工作”具体指 7/8 还是 11/12 原文未说，正文照原文措辞，不替它指定。11/12 版论文 PDF 自注 “written with human assistance”（HN 评论线索，L6，未核 PDF）。 |
| ⑤OpenAI 咨询的 IAS 数学与 AI 顾问组称，其角色不代表评价这些结果的影响，也不代表认可 OpenAI 获取结果的过程 | L3 | ✅ | agmai.org/statement-oct6（`m-agmai-oct6.txt`，发布 2026-10-06T22:25:49Z = 北京 10-07 06:25） | “AGMAI’s advisory role should not be interpreted as a judgment of the impact of these results or an endorsement of the process by which OpenAI obtained them.”；“This release is the beginning, not the completion, of the process of human understanding”。OpenAI 博文称其咨询了该组（“Advisory Group on Mathematics and Artificial Intelligence at the Institute for Advanced Study”）。 |

未进正文的相关核验：
- AGMAI 9/29 建议（`m-agmai-sep29.txt`）要求每个结果公开模型名、提示词、推理摘要、时间与成本，并说明失败尝试；《科学美国人》称 OpenAI 只公开平均算力等统计、无提示词（L4，`m-sciam.txt`）。本号未逐目录核实“无提示词”，故未写入正文。
- OpenAI 公开了10份推理摘要（README 表格）。
- 同日 Erdős 问题网站站长 Thomas Bloom（经陶哲轩博客客座转载）宣布暂停题目评论与解题声明，见速览。

## 主帖二：Claude 对话被报警

当事人为普通人：正文、卡片不出现姓名、住址、照片，不引威胁原句。存档 `a-wink.txt`、`a-gulfcoast.txt` 已替换为占位符；原始 HTML 不入库。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据佛州当地电视台 WINK News 等援引逮捕报告，9月26日，一名女子在 Anthropic 的 Claude 上写下要枪击当地警长办公室的话，次日又称得到了新枪 | L4 | ⚠️（媒体援引，逮捕报告原件未取得） | WINK News（`a-wink.txt`，datePublished 2026-09-30T16:22-04:00 = 北京 10-01 04:22）；WBBH/Gulf Coast News（`a-gulfcoast.txt`，2026-10-01T04:13Z） | WINK：“According to the arrest report, a user identified as [当事人] made a statement on Sept. 26 saying she was going to “shoot up” the Lee County Sheriff’s Office. Investigators say the same user made another statement the following day saying she had gotten a new gun.” WBBH 平台名写 “Anthropic's Claude AI platform”。A 组查过 Lee County Clerk 与警局官网，未取得报告原件（`evidence-a.md` A1）。 |
| 报道称她已被捕，被控书面暴力威胁罪 | L4 | ⚠️ | WINK：“charged with making a written threat of violence under Florida law”；“court date set for November” | 佛州法条 836.10 原文见 `evidence-a.md`；被控 ≠ 定罪。 |
| ①据 WINK 报道，“当日记用”是她事后的说法，由警长转述 | L4 | ⚠️ | WINK：“Sheriff Carmine Marceno told WINK News that [当事人] later said she uses AI like a “diary.”” | “later” = 事后（被找到之后）；正文未写“被捕后”，只写“事后”。 |
| ②据上述报道，逮捕报告称，Anthropic 的安全措施监测威胁内容，因情节严重升级到人工审核团队，由该团队报给执法部门 | L4 | ⚠️ | WINK：“The arrest report says Anthropic’s AI platform uses safety and security measures to monitor for key phrases and potentially threatening content. Investigators wrote that because of the severity of the statements, the information was escalated to a human review team, which then reported the statements to law enforcement.” | 截图 `../images/05-wink-human-review-raw.png`。WBBH 归因 “According to investigators”：“auto-flagged the messages and escalated them to a human review team, which then notified law enforcement”。WINK 称已联系 Anthropic 置评；Inc. 称 Anthropic 未即时回应（`evidence-a.md` A5）。 |
| ③Anthropic 隐私政策称，公司善意认为有合理必要时，可为防止对任何人或财产的严重伤害向执法部门披露个人数据 | L1 | ✅ | anthropic.com/legal/privacy（`a-anthropic-privacy.txt`） | “We may share personal data with government authorities, law enforcement, or other third parties where, based on the information available to us, we have a good-faith belief that disclosure is reasonably necessary to … (ii) prevent serious harm to any person or to property”。截图 `../images/06-anthropic-privacy-raw.png`。另有更窄的“死亡或严重身体伤害”措辞，出自政府请求政策与透明度报告（`evidence-a.md` A4），正文未混用。 |
| 消费级产品关闭“帮助改进模型”后，被安全分类器标记的对话仍可能用于安全用途 | L1 | ✅ | privacy.claude.com 文章 12109829（`a-claude-classifier.txt`，页面日期 2026-08-03） | 适用消费级产品（Free、Pro、Max 及这些账号的 Claude Code）。原文 “may still be used to improve our internal trust and safety models, detect harmful content, enforce our policies, or advance our safety research.” 截图 `../images/07-claude-classifier-raw.png`。 |
| 被起诉也不等于有罪 | — | ✅ | 常识；WINK 写 “court date set for November” | — |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Erdős 问题网站站长称，暂停新的题目评论与解题声明，因多为宣布 AI 证明 | L1 | ✅ | erdosproblems.com 论坛 blog:9，经陶哲轩博客客座转载（`m-bloom-erdos.txt`，2026-10-06T14:37:30Z） | 作者 Thomas Bloom（站长）。“I will freeze new problem comments and proof claims.”；“in practice the vast majority of the comments the site receives now are people announcing AI-generated proofs.” 原文称 hiatus，可能恢复。 |
| Claude 接入 Google 文档、表格和幻灯片，所有付费版公测 | L1 | ✅ | claude.com 文章（`d-claude-workspace.txt`） | “Claude for Google Workspace™ is now in public beta on all paid Claude plans.” 页面未给时刻；9to5Google 10-06 报道。 |
| Mistral 称 Mistral-Large-4 权重本月底发布，现为预览 | L1 | ✅ | mistral.ai/news/mistral-large-4（`d-mistral-blog.txt`） | “Today, we’re launching a public preview of Mistral Large 4.”；“Weights drop end of this month.” 2026-10-06，官方未给时刻。AA 当前标 Proprietary、智能指数38（`d-aa.txt`），未上图。 |
| 据彭博社报道，DeepSeek 接近获得至少800亿元新一轮融资 | L4 | ⚠️ | bloomberg.com 2026-10-06（`d-bloomberg.txt`，北京 10-06 13:00 发布） | “DeepSeek is close to securing at least 80 billion yuan ($12 billion) in its latest round of funding”；匿名知情人士；未完成；DeepSeek 未公告（`evidence-d.md` D3）。腾讯100亿/宁德50亿属首轮（TNW），不用。 |

## 批注译注（`images/cards.toml` 的 gloss，措辞以此为准）

| 图 | 英文原句 | 译注 |
|---|---|---|
| 43 | We prove that all finite-order Hecke L-functions over Q(√−3) and all Dirichlet L-functions are zero-free in the half-plane ℜs > 7/8, with the principal pole at s = 1 allowed. | 我们证明：Q(√−3) 上所有有限阶 Hecke L 函数，以及所有狄利克雷 L 函数，在半平面 Re s > 7/8 内没有零点（s = 1 处的主极点除外）。 |
| 43 | In particular, the Riemann zeta function is zero-free in this half-plane, proving the quasi-Riemann hypothesis. | 特别地，黎曼 ζ 函数在这个半平面内没有零点，从而证明拟黎曼猜想。 |
| 40 | This collection includes results at different stages of verification. Not all have accompanying Lean formalizations. | 这批成果处于不同的验证阶段，并非都附有 Lean 形式化。 |
| 40 | Some of the unformalized results could have issues. | 部分未形式化的结果可能有问题。 |
| 41 | On average, each result used three hours of ChatGPT Pro thinking compute with that model. Over the course of the evaluation, the model was posed approximately 4,000 problems. | 平均每个结果用了该模型约3小时 ChatGPT Pro 思考算力。评测期间共向模型提了约4000道题。 |
| 41 | Exceptions to this fixed procedure include work on a zero-free region for the Riemann zeta function and proof of the Hodge Conjecture for CM abelian varieties. | 这一固定流程的例外包括：黎曼 ζ 函数无零区域的工作，以及 CM 阿贝尔簇霍奇猜想的证明。 |
| 41 | Additionally, the writeup for the Re(s) > 11/12 zero-free region for the Riemann zeta function was human edited for readability. | 此外，黎曼 ζ 函数 Re(s) > 11/12 无零区域的文稿经过人工编辑，以便阅读。 |
| 05 | The arrest report says Anthropic’s AI platform uses safety and security measures to monitor for key phrases and potentially threatening content. | 逮捕报告称，Anthropic 的 AI 平台使用安全措施，监测关键词句和潜在的威胁内容。 |
| 05 | Investigators wrote that because of the severity of the statements, the information was escalated to a human review team, which then reported the statements to law enforcement. | 调查人员写道，由于言论情节严重，信息被升级到人工审核团队，再由该团队报给执法部门。 |
| 06 | We may share personal data with government authorities, law enforcement, or other third parties where, based on the information available to us, we have a good-faith belief that disclosure is reasonably necessary to | 如果根据现有信息，我们善意地认为披露是合理必要的，我们可能向政府机构、执法部门或其他第三方提供个人数据，以便： |
| 06 | (ii) prevent serious harm to any person or to property; | （二）防止对任何人或财产造成严重伤害； |
| 07 | This article is about our consumer products such as Claude Free, Pro, Max | 本文针对消费级产品，如 Claude Free、Pro、Max |
| 07 | If you decide to turn off the model training setting, we will not use any new chats and coding sessions you have with Claude for future model training. | 如果你关闭模型训练设置，我们不会把你与 Claude 的任何新聊天和编程会话用于未来的模型训练。 |
| 07 | If our safety classifiers flag your conversations, they may still be used to improve our internal trust and safety models, detect harmful content, enforce our policies, or advance our safety research. | 如果我们的安全分类器标记了你的对话，这些对话仍可能用于改进内部信任与安全模型、检测有害内容、执行政策或推进安全研究。 |

## 视觉复核

第 1 轮：WSL Codex（gpt-6-astra medium，只读），14 张图（省流卡、速览图、6 张批注图及其底图）。结论 PASS / no blocker，3 条 SHOULD_FIX，均采纳：
- 06：译注漏了“依据现有信息、善意认为披露合理必要”的条件 → 扩大第 1 条高亮与译注到条件从句。
- 43：中文译注只写“这个半平面”，范围 Re s > 7/8 只在英文里 → 新增第 1 条，高亮并翻译摘要首句（手量坐标）。
- 07：译注缺“关闭训练设置”的语境 → 新增关闭训练设置一句。
复核方确认：无高亮串句、编号配色对应、省流卡保留归因、速览 L4 条保留“据彭博社报道”、无禁用平台名。重渲后 Claude 逐张自查。

## 反向核验

第 1 轮：codex-reviewer（WSL Codex gpt-6-astra medium，只读，联网），对照正文、cards.toml、本表与 `sources/` 存档。2 BLOCKER、6 SHOULD_FIX、1 NEEDS_VERIFICATION，Claude 逐条对照存档后全部采纳：
1. BLOCKER 正文 1022 字超限 → 删去算力句（留在批注图 41）、精简省流与日记①，现 991 字。
2. BLOCKER “所有狄利克雷 L 函数”之后写“相当于把7/8推到1/2”会变成广义黎曼猜想 → ①改为“对黎曼 ζ 函数，要把这条边界推到1/2才相当于黎曼猜想”；事实段改“OpenAI 称证明了……（s=1 的极点除外）”。
3. 隐私条款丢了“善意认为有合理必要”与“消费级产品”，“仍可”宜作“仍可能” → 正文③补齐；卡片 06、07 已在视觉复核轮补齐。
4. 日记链的被捕句缺句内归因，“买了新枪”原文是 had gotten → 改“报道称她已被捕”“得到了新枪”；省流把“据当地媒体援引逮捕报告”前置；省流卡同步。
5. “最受关注”无一手依据 → 正文与省流卡改“其中包括”。
6. “手稿标注日期”实为目录名日期 → 改“手稿目录名里的日期”。
7. AGMAI “judgment of the impact” 是“评价”不是“认可” → ⑤改“不代表评价这些结果的影响，也不代表认可 OpenAI 获取结果的过程”。
8. schema 未规定 unchecked 为“最低一级” → 本表删去该说法，正文改“审核状态字段”。
9. 提交时间 ≠ 转公开时间、博文未存档 → 存档 GitHub API 响应与博文正文，时间精度只写到“清晨”（提交 05:58 与《科学美国人》所说 6 P.M. EDT 均为北京 10-07 清晨）。
复核方另确认：README 的 “include” 未被偷换成穷尽列举；速览四条与本表逐字一致；译注无实质错译；公开文本无当事人姓名。未运行 Lean/Comparator。
