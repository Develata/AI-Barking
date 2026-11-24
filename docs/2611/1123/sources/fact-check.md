# 1123 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B 组 Sonnet 5.5（`claude-sonnet-5-5`），C 组 Haiku 5.5（`claude-haiku-5-5`）；用时（harness）C 12 分钟，B 15 分钟，A 15 分钟（A、B 被催交，A 截图另补一轮）。Claude 抽查：README 原句（GitHub API 自取）、论文摘要与附录 Q.1 原句（`a-paper-pdf.txt` 第 13、37、55、578、3798 行）、OpenAI 提交时刻（GitHub API 自取 e07100d3 = 2026-11-24T01:20:59Z）与 history.md 原句、NariQ 模型卡原句与分数表（HF raw 自取）、fork 硬件表 GeForce 行、掘金原句（截图目视）。

## 一、Claude 证 VP≠VNP（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：7万行AI证明，没有数学家查过 | L1 | ✅ | 7万行：vp-vnp 目录 72,932 行 Lean（A 组浅克隆统计）；“没有数学家查过”：README “no human mathematician has checked the proof” | 读者稿检查后由“Claude证代数版P≠NP？人类没查过”改来（见 `style-review.md`）；“7万行”为向下取整 |
| 省流：Claude 的开发商 Anthropic 公开署名 Claude 的论文，称证明了 VP≠VNP | L1 | ✅ | `a-paper-pdf.txt`：署名“CLAUDE / ANTHROPIC”，摘要“Hence VP ≠ VNP over every field”；README “*VP ≠ VNP over every field*, by Claude (Anthropic)” | |
| （俗称“代数版 P≠NP”） | L1/L3 | ✅ | 论文摘要“This is the algebraic analogue of the conjecture P ≠ NP”；Bürgisser 综述（arXiv:2406.06217）“can be seen as a version of Cook’s P ≠ NP conjecture” | “俗称”：中文媒体普遍写法（新智元、华尔街见闻等） |
| 论文和 Lean 代码都由 AI 写成，没有人类数学家查过 | L1 | ✅ | README：“Both were written by AI models, and no human mathematician has checked the proof.” | |
| VP≠VNP 是 Valiant 1979年提出的猜想 | L1/L3 | ✅ | Crossref `a-bg-valiant-crossref.json`：Valiant, “Completeness classes in algebra”, STOC '79；Bürgisser：“Valiant conjectured that the classes VP and VNP are different” | |
| 可看作 P≠NP 在“只用加法和乘法算多项式”时的版本 | L3 | ✅ | Bürgisser：“a version of Cook’s P ≠ NP conjecture, tailored to the framework of computations of polynomials”；论文摘要“Algebraic circuits must work for all inputs in the field” | 通俗说法：算术电路只用加法与乘法 |
| 11月24日凌晨，Anthropic 公开论文和约7.3万行 Lean 代码 | L1 | ✅ | 提交 918c8e27，2026-11-23T20:04:44Z = 北京 11-24 04:04:44；PR #51 合并北京 05:43:50；A 组浅克隆统计 vp-vnp 目录 324 个 .lean 文件、72,932 行（含空行与注释） | 统计方法见 `evidence-A.md` A1-4 |
| 称由内部 Claude 模型发现 | L1 | ✅ | README：“The result was discovered by an internal research Claude model.” | |
| 项目说明写，没有人类数学家查过证明，“这不解决 P vs NP” | L1 | ✅ | README：“This does not settle P vs NP.” | |
| OpenAI 随后也公开同题论文（提交时间晚约5小时） | L1 | ✅ | openai/math 提交 e07100d3 “Add VP != VNP manuscript and Lean.” 2026-11-24T01:20:59Z = 北京 09:20:59（比 04:04:44 晚 5 小时 16 分）；论文题“VP and VNP are different”，摘要“We prove that VP differs from VNP over every field.” | 两家下界形式不同（Anthropic m^{c(log m)^{1/3}}；OpenAI exp(Ω((log n)^{5/4}))，border 电路），n、m 含义不同，不比大小，正文不写。OpenAI 论文未引用 Anthropic 本文 |
| 称11月6日已得出同样结论 | L1 | ✅ | `a-oai-history.txt`：“*VP and VNP are different*. This result was obtained on November 6, 2026.” | OpenAI 自述，未见独立佐证 |
| 对你：论文写明，结果对“NP 难题有没有快速算法”什么也没说 | L1 | ✅ | 论文 p.4：“The results of this paper therefore say nothing about whether NP-complete problems have fast algorithms.” | |
| 吠点①：新智元稿题“Claude 证出「代数版 P≠NP」” | L5 | ✅（只证引文存在） | 网易号新智元 `a-zh-163-xzy`（北京 11-24 10:21:50）：标题“Claude证出「代数版P≠NP」！73000行Lean代码通过验证” | 凤凰网同题转载 12:19:52 |
| 吠点①：论文摘要写它是 P≠NP 的代数类比，但“不解决那个猜想” | L1 | ✅ | 摘要：“This is the algebraic analogue of the conjecture P ≠ NP, but it does not settle that conjecture.” | |
| 吠点①：结论也管不到电脑里只处理0和1的电路 | L1 | ✅ | 摘要：“Algebraic circuits must work for all inputs in the field and its extensions – not just 0 and 1 – thus our superpolynomial bound does not apply to Boolean circuits.” | 已知方向只有布尔分离 ⟹ 代数分离（Bürgisser），反向“no implication … is known”（论文 p.1） |
| 吠点②：华尔街见闻稿题“全篇 Lean 机器验证通过” | L5 | ✅（只证引文存在） | 网易号华尔街见闻 `a-zh-163-just-now`（北京 11-24 10:55:40）：标题“刚刚，Anthropic内部Claude模型独立攻克代数版P/NP问题，全篇Lean机器验证通过” | 其正文另写“没有经过人类数学家的人工审稿” |
| 吠点②：Lean 只保证每步推得通 | L1 | ✅ | README：“Lean checks the proofs, but no machine checks that this file says what is claimed.” | 通俗说法 |
| 吠点②：要证的命题本身有没有写对，论文附录写“只由 AI 读者判断过” | L1 | ✅ | 论文附录 Q.1（p.66）：“Whether the statements say what is meant was judged only by AI readers. No human mathematician has checked the development, the statement file or the proofs.” | |
| 小标题：VP≠VNP 证明，数学家没查过 | L1 | ✅ | README“no human mathematician has checked the proof” | 反向核验后由“Claude 证代数版 P≠NP，人没查”改来（原句缺“称”、“人”范围过宽） |

未进正文（A 组，存档备查）：
- 作者方 CI（Anthropic 自家 Actions）北京 06:05:02 日志 “nanoda kernel accepts the solution”“Your solution is okay!”；非第三方复现。
- PR #51 由同一账号开启并合并，评审评论“lgtm”与一次 APPROVED，是代码评审不是证明核验。
- 下界是拟多项式 2^{Ω((log m)^{4/3})}，论文称“there is still plenty of room to tighten the bounds”；m₀ 依赖特征 p；特征 2 下置换式即行列式，不适用。
- 此前最好下界：论文“no lower bound of larger order than m² log m was known for PERM_m”，Bürgisser“nobody knows how to prove a lower bound of order n² log n”，措辞不同，未裁决。
- anthropic.com 新闻、研究页与 claude.com 博客无条目（北京 23:05）；formal-math 根 README 项目表无 vp-vnp 行。
- 署名学者公开评价：只找到 Viola 博客转述“was announced”；Fortnow–Gasarch 帖早于本事件。
- 夸大实例另有 emergent.sh“Independent validation by human mathematicians”（与 README 相反）、36Kr 英文站“has been proven”。

## 二、NariQ（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 省流：NariQ 把 27B 模型压进约 10GB | L1 | ✅ | 模型卡标题“NariQ: Qwen3.8-27B in a 10 GB file”；3.0bpw 文件 10,123,626,592 字节 | 不含 0.93GB 视觉投影 |
| 但只能用作者改版的 llama.cpp 跑 | L1 | ✅ | 模型卡：“Run them on our llama.cpp fork (required).”；“Stock llama.cpp, Ollama and LM Studio can't load the new tensor types yet” | |
| 11月20日，Nari Labs 发布阿里 Qwen3.8-27B 模型的压缩版 | L1 | ✅ | 模型卡“(Nov 20, 2026) We are releasing NariQ v1 GGUFs of Qwen3.8-27B at 3, 2.5 and 2 bits per weight.”；HF 仓库创建北京 11-20 09:27:15 | |
| 3比特档文件约 10.1GB，原版约 54GB | L1 | ✅ | 分数表“NariQ 3-bit (10.1 GB)”“BF16 (53.8 GB)” | 论文写原版 55.6GB（含视觉与 MTP），正文用模型卡数 |
| 称在长时间编程代理、数学和看图任务上与原版持平 | L1 | ✅ | “The 3-bit and 2.5-bit files match BF16 on long-horizon agent tasks, math and image understanding.” | 看图 = image understanding（RealWorldQA） |
| 成绩都是作者自测 | L1 | ✅ | 模型卡图注“Every point was measured by us with the same code.”；Unsloth 对照点读自其图表 | |
| 对你：这次放出的是要自己部署的模型文件 | L1 | ✅ | 模型卡：“We are releasing NariQ v1 GGUFs … Run them on our llama.cpp fork (required).” | 反向核验指出原句“只在网页或 App 里聊天的用不上”无来源支持（fork 自带聊天网页与兼容 API），已改 |
| AMD、Intel 显卡暂不支持 | L1 | ✅ | 模型卡：“AMD and Intel GPUs are not supported yet”；fork“AMD, Intel GPUs: No NariQ kernels.” | 外部 AMD 移植 PR #3 未合并 |
| Mac 只在一台 M3-Pro 上用3比特档跑过，每秒约3.8个 token | L1 | ✅ | 模型卡：“on an M3 Pro (18 GB) decode is about 3.8 tok/s with the 3.0bpw file … Other Macs are untested.”；Metal 文档“Only one machine has run this” | |
| 吠点①：掘金速递写“可直接对接 llama.cpp 等主流推理框架” | L5 | ✅（只证引文存在） | juejin.cn/post/7711225699185410102（北京 11-24 08:31:20）第6条 | 截图 `20-juejin-nariq-item.png` |
| 吠点①：模型卡写须用作者改版，原版 llama.cpp、Ollama、LM Studio 都加载不了 | L1 | ✅ | 同省流第二句 | |
| 吠点②：掘金同一篇写“消费级显卡甚至部分笔记本”能跑 | L5 | ✅ | 同篇：“压到10GB意味着可在消费级显卡甚至部分笔记本上本地运行” | |
| 吠点②：模型卡写 GeForce 显卡“尚未测试” | L1 | ✅ | “GeForce cards have not been tested yet; tested GPUs are RTX PRO 6000, H100, L4 and A10.” | |
| 吠点②：作者改版说明里只记一张 RTX-5060-Ti 跑过，标注尚未完成资格验证 | L1 | ✅ | fork README 硬件表：“NVIDIA GeForce: One card run so far, not qualified: RTX 5060 Ti 16 GB, 2.5bpw with the vision projector, about 38 tokens/s decode on Windows (512 tokens, short prompt).” | 与模型卡“尚未测试”不一致，原样并列；fork 该段北京 11-20 16:46，模型卡该段最后改于 17:09 |
| 小标题：NariQ：27B 压进 10GB | L1 | ✅ | 同上 | |

未进正文（B 组，存档备查）：
- 原吠点③（2.5比特档五项有四项低于原版，如 AIME 96.5% 对 97.7%）读者稿检查后删去：差距小、读者不在乎；数字仍 ✅（现行分数表）。
- DeepSWE 列 11-22 由一轮改为三轮平均，3-bit 与 2.5-bit 高低互换（一轮 41.6/40.7，三轮 41.0/41.9）；论文 v1 与 fork README 仍是一轮数。
- 同卡（RTX PRO 6000）4K 上下文 NariQ 3.0bpw 88.9 tok/s，Unsloth UD-IQ3_XXS 94.7。
- mobdome 自称 RTX 4070 实测 18–22 tok/s、需 Python 环境，与官方不符，疑似虚构（推断，未证实）。
- 上游 llama.cpp 无 NariQ 相关 issue/PR；HF 页面自动生成的 llama.cpp、LM Studio 按钮按模型卡现在不能用。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI、思科开源 Shirudo：Linux 上自测不重启拦下攻击，预发布 | L1 | ✅ | openai.com/index/introducing-shirudo：“In a controlled test, a Codex-generated shield blocked a Java4Shell exploit attempt against a running Elasticsearch service without requiring a restart.”；README：“shirudo is pre-release.”；思科新闻稿北京 11-24 05:20 | 受控测试、厂商自测（“自测”= OpenAI 与思科自己的测试）；Linux 运行时；README“pre-release”，生产需带补丁内核与 TPM 2.0；“攻击”= exploit attempt |
| Claude 插件页：卸载带连接器的插件不会断开连接器，须另行断开 | L1 | ✅ | support.claude.com 17740565：“Removing a plugin that packages skills and a connector doesn't disconnect its connector, so you'll need to disconnect it separately.” | Free/Pro/Max/Ultra 已开放，Team 11-30 起 |
| Anthropic：Claude Ship 代码审查正式可用，另两个代理为测试版 | L1 | ✅ | claude.com/resources/articles/introducing-claude-ship：“Code Review, which is already generally available, and two new agents in beta: PR Steward and Claude Approvals.”；“available on Claude Team and Enterprise plans for repositories on GitHub” | 页面仅日期 11-23；“可用”= generally available；PR Steward 与 Claude Approvals 为 beta；ZDR、HIPAA、客户自管密钥的组织不可用 |
| Anthropic 访谈58家美国客户（非随机）：2/3自报因 AI 少招人 | L1 | ✅ | anthropic.com/research/working-with-ai：“66 interviews with 58 of Anthropic's enterprise customers”；“People at two-thirds of the companies we spoke to reported reduced hiring due to AI”；“only two had seen AI automation fully eliminate a position and lead to someone losing their job” | 2/3 指受访公司、受访者自报（“reported reduced hiring due to AI”）；样本为美国客户、原文“not a random or systematically representative sample”；另“仅两家因自动化替岗致失业”第 2 轮反向核验后删去（与解雇拒用 AI 员工的五家口径易混） |
| 英国 AISI：同一 AI 监控器换攻击生成法，成功攻击漏报0.2%到23% | L1 | ✅ | aisi.gov.uk（Nov 23, 2026）：“the same monitor missed 0.2% to 23% of successful attacks depending on the ‘attack policy’ used to generate them” | 单一监控器、单一代理；分母为成功攻击 |
| a16z 测购物 AI 代理：允许游客结账的网店，81%的运行走到表单末尾 | L1 | ✅ | a16z.com/welcome-to-the-b2a-economy-how-agents-buy：“agents reached the end of the checkout form in 81% of runs….on stores that offer guest checkout” | 不是付款完成率；北京 11-23 22:55 |
| DeepMind 智能体改进量子算法，称所需资源可能仍超首批纠错量子机 | L1 | ✅ | deepmind.google 博客（北京 11-24 01:00）：“Running these algorithms in scientific simulations would still demand more computational resources than the first error-corrected quantum computers are likely to provide” | 智能体为 Gemini 4 团队 |

速览未收：GPT-6.2 普通 Chat（与 1117 系统卡同线）、Figma Agent（开放范围原文矛盾）、Google 搜索接 Wallet（未找到 11 月官方公告）。

## 卡片日期

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 卡片日期“11月25日” | — | ✅ | 计划发布日（北京 11-25） | 与 1122 用法一致 |
| vp-readme、oai-history、vp-abstract、vp-ai-readers 存档日期“2026-11-25” | — | ✅ | `capture-log-A.md` 第六节：A 组截图于北京 11-25 01:12–01:35 补做（`a-shots-log.jsonl`） | |
| juejin-claims、nariq-limits、fork-geforce 存档日期“2026-11-24” | — | ✅ | `capture-log-B.md`：B 组截图于北京 11-24 23:05–23:09 | |

## 批注译注

| 图 | 原文 | 译注 |
|---|---|---|
| vp-readme ① | This does not settle P vs NP. | 这不解决 P vs NP 问题。 |
| vp-readme ② | Both were written by AI models, and no human mathematician has checked the proof. | 论文和 Lean 代码都由 AI 模型写成，没有人类数学家检查过这份证明。（“Both”指上句的论文与 Lean 形式化） |
| vp-ai-readers | Whether the statements say what is meant was judged only by AI readers. No human mathematician has checked the development, the statement file or the proofs. | 命题是否表达了本意，只由 AI 读者判断过。没有人类数学家检查过这套代码、命题文件或证明。 |
| juejin-claims ①② | 中文原文 | 原文照录 |
| nariq-limits ① | Stock llama.cpp, Ollama and LM Studio can't load the new tensor types yet | 原版 llama.cpp、Ollama 和 LM Studio 目前还加载不了这些新的张量类型。 |
| nariq-limits ② | GeForce cards have not been tested yet | GeForce 显卡尚未测试。 |
| nariq-limits ③ | on an M3 Pro (18 GB) decode is about 3.8 tok/s with the 3.0bpw file | 在一台 M3 Pro（18GB）上，用3比特档每秒约生成3.8个 token。 |
| fork-geforce | One card run so far, not qualified: RTX 5060 Ti 16 GB, 2.5bpw with the vision projector, about 38 tokens/s decode on Windows (512 tokens, short prompt). | NVIDIA GeForce：目前只跑过一张卡，尚未完成资格验证：RTX-5060-Ti 16GB，2.5比特档带视觉投影模块，Windows 上每秒约38个 token（512个 token、短提示）。 |
| oai-history | This result was obtained on November 6, 2026. | （VP 与 VNP 不同）这一结果于2026年11月6日得出。 |
| vp-abstract ① | This is the algebraic analogue of the conjecture P ≠ NP, but it does not settle that conjecture. | 这是 P≠NP 猜想的代数类比，但它没有解决那个猜想。 |
| vp-abstract ② | thus our superpolynomial bound does not apply to Boolean circuits. | 因此我们的超多项式下界不适用于布尔电路（只处理0和1的电路）。 |

## 反向核验与视觉复核

反向核验第 1 轮（Codex gpt-6-astra，effort high，只读；北京 11-25 01:20–01:26；读的是读者稿检查改稿过程中的版本）：4 项 BLOCKER、7 项 SHOULD_FIX、2 项 NEEDS_VERIFICATION。Claude 逐条核对存档后处理：

1. BLOCKER AISI 漏报率丢分母：采纳，速览改为“成功攻击漏报0.2%到23%”（`c-c-aisi-monitor.txt`“missed 0.2% to 23% of successful attacks”）。
2. BLOCKER 小标题“Claude 证代数版 P≠NP，人没查”缺“称”、“人”范围过宽：采纳，改为“VP≠VNP 证明，数学家没查过”。
3. BLOCKER“只在网页或 App 里聊天的用不上”无来源支持：采纳，核对 `b-fork-readme-raw.txt` 确有聊天网页与兼容 API；改为“这次放出的是要自己部署的模型文件”。
4. BLOCKER Claude Ship 开放范围：采纳，速览改为“Team 与企业版可用代码审查”；GitHub 仓库与 ZDR 等限制留在本清单。
5. SHOULD_FIX 就业调查缺自报：采纳，补“自报”；美国客户、非随机样本放不下 40 字，留在本清单。
6. SHOULD_FIX“未合格”译法：采纳，改为“尚未完成资格验证”，译注补“带视觉投影模块”。
7. SHOULD_FIX 对你句硬件条件：采纳，Mac 补“用3比特档”，24GB 补“作者称”；“显示器共用显存时须降低上下文”未写（字数）。
8. SHOULD_FIX Shirudo 受控测试：采纳，补“受控测试中”；Linux 范围未写（字数）。
9. SHOULD_FIX 配图说明未同步：审查时 README 正在更新；已核对正式配图表 01–13 与 `publish/` 一致，公众号跳过编号改为 03、04、09、10。
10. SHOULD_FIX 吠点②“同一篇”主张者不明：采纳，改为“掘金同一篇”。
11. SHOULD_FIX 卡片日期无清单行：采纳，补“卡片日期”一节。
12. NEEDS_VERIFICATION“约5小时后公开”是提交时间：采纳，改为“按提交时间，约5小时后”。两仓库均为公开仓库，提交与合并时即可见（PR #51 合并北京 05:43:50，OpenAI PR #5 合并 09:30:38，按合并算约3小时47分）；正文只写按提交时间。
13. NEEDS_VERIFICATION AISI 为英国政府机构，涉速览风控：未删，交终审由 Develata 决定（往期 1023 期主帖、多期速览收过 AISI 研究条目，未见受限记录；本条为技术研究，不涉政治）。

反向核验第 2 轮（Codex gpt-6-astra，effort high，只读；北京 11-25 约 01:50–02:05）：1 项 BLOCKER、4 项 SHOULD_FIX、2 项 NICE_TO_HAVE、1 项 NEEDS_VERIFICATION；第 1 轮 13 项中 9 项确认已修。处理：

1. BLOCKER Claude Ship 仍缺 GitHub 与 ZDR 等组织限制：采纳审查者给的退路，删去套餐可用性断言，速览改为“Anthropic：Claude Ship 代码审查正式可用，另两个代理为测试版”，限制留在本清单。
2. SHOULD_FIX 就业“2家”口径：采纳，删去第二个数字，改为“Anthropic 访谈58家美国客户（非随机）：2/3自报因 AI 少招人”。
3. SHOULD_FIX 24GB 上下文缺“显示器共用显存须降低”条件：采纳，删去该句。
4. SHOULD_FIX Shirudo 缺 Linux 与厂商自测：采纳，改为“OpenAI、思科开源 Shirudo：Linux 上自测不重启拦下攻击，预发布”。
5. SHOULD_FIX“按提交时间……公开”混用口径：采纳，改为“OpenAI 随后也公开同题论文（提交时间晚约5小时）”；“随后”另有两仓库 PR 合并先后支持（Anthropic 北京 05:43:50，OpenAI 09:30:38）。
6. NICE_TO_HAVE 截图时段写错：采纳，改为 01:12–01:35。
7. NICE_TO_HAVE 配图说明残留“尚未合格”：采纳，已同步。
8. NEEDS_VERIFICATION AISI 风控：仍交终审，由 Develata 决定。

视觉复核（Codex gpt-6-astra，effort medium，只读，22 张图；北京 11-25 约 01:45）：PASS / no blocker；3 项 SHOULD_FIX。处理：速览 Shirudo 补“自测”与 Claude Ship 开放范围两条，与反向核验第 2 轮一并改（见上）；`08-vp-ai-readers.png` 关键句太小：采纳，底图改为 Q.1 段首行加末两行拼接（`barking shot stack`，1000×122），关键句放大，段首保留以示出处。

