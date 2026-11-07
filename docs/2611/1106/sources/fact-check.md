# 1106 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（超级智能法案）、`evidence-B.md`（METR 与 Cotra）、`evidence-C.md`（速览）；截图在 `images/`。取证于北京 2026-11-07 23:10 至 11-08 00:30。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 封面文字：主标题「造超级AI判20年？」，副标题「还只是提案」 | — | ✅ | 见 A3、吠点② | 正文标题前一分句；问号＝法案条款，未成法 |
| 标题：造超级AI判20年？一年就到超级智能？ | — | ✅ | 见 A3、B5 | “判20年”为上限，限鲁莽违规的决策人员或 rogue actor；第二问为 Cotra 推算，吠点纠正 |
| 省流：美国议员要把禁超级智能列为头号议程，法案最高判20年，还在委员会 | L1 | ✅ | 见 A1、A3、吠点② | Ansari 原文 “Top of the Democratic agenda on day 1”；状态取自 congress.gov |
| 省流：METR 称 Anthropic 模型在任务明确的测试上进步约快一倍，“一年内超级智能”只是员工个人推算 | L3 | ✅ | 见 B2、B5 | “约快一倍”＝ around twice the rate；Cotra 声明未参与项目 |

## A 超级智能法案

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| A1 11月5日，民主党众议员 Ansari 发帖：新国会首日就要暂停先进 AI、禁止超级智能 | L1 | ✅ | `a-x-ansari-2118054336077127797.json`，图 08 | 北京 11-05 02:54（美东 11-04）。原文还有 regulation，正文省略不改义 |
| A2 她参与的法案9月提出 | L1 | ✅ | H.R. 10538 https://www.govinfo.gov/content/pkg/BILLS-119hr10538ih/html/BILLS-119hr10538ih.htm | 9/24 众议院；S. 5493 9/23；两版逐字相同（`diff` 只差页眉）；Ansari 为原始联署人 |
| A3 禁止超级智能和带“前兆特征”的系统，如能大幅加速 AI 研发。决策者等鲁莽违规，最高判20年 | L1 | ✅ | 同上 Sec. 9(a)、3(6)(A)、Sec. 13(b)，图 03、05、06 | “covered policymaking individual or rogue actor who recklessly violates section 8, 9, or 12”，“not more than 20 years”；非决策个人为10年行业禁入 |
| A4 训练达10的25次方次运算的模型要暂停训练和修改（去除前兆或停运除外），等新设的 AI 部配齐人员、定好规则 | L1 | ✅ | Sec. 3(1)、3(4)、Sec. 8，图 01、02、04 | “greater than or equal to 10^25”；部长可年度调整；期限到部门配齐人员并颁布规则，无日期上限；须法案生效才开始；Sec. 3(4) 例外：为去除前兆特征或使其停运的修改 |
| A5 Epoch 估算 GPT-6-Astra 已过线 | L3 | ✅ | Epoch `all_ai_models.csv`（`a-epoch-1e25-extract.tsv`，2026-11-07） | 9.4e26，置信 Likely；Opus-5.5、Gemini-4 等算力在表内为空，未知 |
| 吠点① 众议院版12名联署人 | L1 | ✅ | congress.gov H.R. 10538（`a-congress-house-bill-10538-all-actions.txt`，图 11） | 取证时点北京 11-07 约 23:35；不含提案人 Casar；9名原始＋3名后加 |
| 吠点① 党内领袖 Jeffries 10月称对明确禁止暂不表态 | L4 | ⚠️ | USA Today 10-01（`a-usatoday-jeffries.txt`）：“not prepared to take a position as to whether there should be an explicit ban or not” | 众议院民主党领袖；未找到其选后回应，正文写明“10月” |
| 吠点② 法案截至11月7日还在委员会 | L1 | ✅ | congress.gov 动作2条，最新 09/24 付委，图 11 | 取证时点北京 11-07 约 23:35；S. 5493 同样只有付委；未查委员会听证记录，所以不写“没有听证” |
| 吠点② 本届不成法就作废，明年得重提 | L1 | ✅ | LoC Ask a Librarian（`a-loc-dead-bill.txt`）：“considered ‘dead’”，须 “reintroduced with a new number” | 通则，非针对本法案；第119届，新国会2027年1月 |
| 吠点③ 有中文稿写“强制解散” | L5 | ⚠️ | 东方财富 https://wap.eastmoney.com/a/202611053923800611.html ；鉅亨網 https://news.cnyes.com/news/id/6649036 ；网易 https://www.163.com/dy/article/LB2D7INK051181RS.html | 已核对原稿措辞，仅作被纠错对象；条文判断由下两行 L1 支持 |
| 吠点③ 条文是吊销许可、法院维持后可没收资产 | L1 | ✅ | Sec. 13(d)，图 06 | “revoke a charter”；上诉至联邦巡回法院；“forfeiture”；全文无 dissolve |
| 吠点③ 被认定带前兆特征的系统不看算力，也要暂停、断网 | L1 | ✅ | Sec. 8(a)、Sec. 10(a)（`a-hr10538-ih-html.txt`） | Sec. 8 对 advanced 系统；Sec. 10(a) “any artificial intelligence system that the Secretary identifies … immediately subject to a mandatory pause and sequestered from the internet”，不看算力门槛；Sec. 10(b) 30天内无法确认去除须停运 |

## B METR 与 Cotra

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| B1 评估机构 METR 近日发布对 Anthropic 的审计 | L3 | ✅ | https://metr.org/blog/2026-11-05-anthropic-ai-rd-acceleration-assessment/ | 官网只标 2026-11-05，未给时刻和时区；METR 官方账号评审帖北京 11-06 02:58，所以正文写“近日”；Anthropic 有单方删节权 |
| B2 参与的 METR 员工多数认为，到10月，Anthropic 模型在任务明确的测试上，进步速度约是2024年3月至2025年10月的两倍 | L3 | ✅ | 同上 Pace of capabilities progress，图 14 | “most METR staff who worked on this project”；“around twice”；“well-specified tasks”；同段写 1.3x 或 3x 也不意外、团队内有分歧 |
| B3 METR 猜这次加速约80%来自 AI | L3 | ✅ | 同上 Attribution，图 15 | “We guess that a majority (perhaps ≈80%)”；指当前加速中的占比 |
| B4 11月2日，Anthropic 上市文件写，按内部估算和对未来模型的预期，可能很快达到自家“AI 大幅加速进步”标准，现有模型未被认定超过 | L1 | ✅ | SEC S-1/A（`b-sec-s1a-ctx.json`，图 22） | “our internal estimates and our expectations regarding future models, rather than our current models, indicate that we may imminently meet…”；“We have not designated any of our current models as exceeding this threshold”；EDGAR 接收北京 11-02 22:24；10-26 原 S-1 无此句 |
| B5 11月7日，METR 员工 Cotra 个人博客写，她判断标准已达到，若加速平稳，最简单外推，一年内到超级智能 | L2 | ✅ | https://www.planned-obsolescence.org/p/ai-is-dramatically-accelerating-ai ，图 23 | 北京 11-07 04:23（页面 datePublished 2026-11-06T20:23:40Z，`b-cotra.html`）；脚注 4 “plain reading, my best guess”；“naively implies that we would reach extreme superintelligence within a year”；前提 “assume acceleration from AI is smooth” |
| 吠点① “一年”是 Cotra 个人推算，METR 没写，她也写现实有上限 | L2 | ✅ | Cotra 博文：“assume acceleration from AI is smooth”；“at some point we would reach the effective ceiling on software efficiency” | METR 报告未作此推算；Cotra 声明未参与项目 |
| 吠点② 两倍指测试分数涨得快，不是研究员效率翻倍。1.9倍来自 Anthropic 自家指数 | L3 | ✅ | METR 报告，图 16 | AECI＝Anthropic 版 Epoch Capabilities Index；95% CI 1.5–2.8；稳健性 1.1–2.9；RSP 脚注称不同于研究员效率翻倍 |
| 吠点② 算上别家模型是1.4倍，METR 没定哪个准 | L3 | ✅ | METR 附录，图 19 | “a couple of plausible interpretations”：一读 1.4 更准，一读 1.9 更准 |
| 吠点③ Anthropic 员工自报提速12倍来自内部表情投票，METR 称证据很松 | L3 | ✅ | METR 报告，图 17 | “Slack polls … staff use reactions to vote”；“only very loose evidence”；“reaching 12x in October 2026” |
| 吠点③ 它认为最有价值的证据多数被 Anthropic 删节 | L3 | ✅ | METR 页首 Redaction summary：“Anthropic redacted a majority of the evidence we found most informative for these conclusions” | Anthropic 称为避免帮到其他 AI 开发者 |
| 吠点④ METR 称审计原本要查算力怎么分给能力和安全，Anthropic 中途取消了 | L3 | ✅ | METR 报告，图 18：“Anthropic changed the scope of this project midway to exclude compute verification” | 原文是能力与安全之间的分配；Anthropic 称想聚焦风险评估、以后再议，METR 取消前未得实质结论 |

未采用：扫描的“FAKE DATA”作吠点（确有两张图，均为明示的示意图，不是测得数据）；Kokotajlo 质疑算力核查（未找到）；美股因果（风控，且读到的行情稿归因于数据中心放缓而非禁令）；数据中心公投口径（与本题无关）。

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 民主党第一天的议程之首，应是暂停先进 AI 开发、监管这个失控的行业，以及禁止超级智能。 | L1 | ✅ | 图 08 | 译注，Ansari 发帖原文 |
| 任何人不得开发、部署（内部或对外）、获取、持有、资助、进口或转让超级智能，或带一项以上超级智能前兆特征的 AI 系统。 | L1 | ✅ | Sec. 9(a)，图 05 | 译注，H.R. 10538 第 9 条 · 禁令 |
| 持牌公司的决策人员，或与持牌公司无雇佣、隶属关系的个人，鲁莽违反第 8、9 或 12 条或相关规则的，处罚金并处不超过20年监禁。 | L1 | ✅ | Sec. 13(b)(c)，图 06 | 译注，H.R. 10538 第 13 条 · 处罚 |
| 不属上述决策人员的个人鲁莽违规，禁止在 AI 行业工作10年。 | L1 | ✅ | Sec. 13(b)(c)，图 06 | 译注，H.R. 10538 第 13 条 · 处罚 |
| 若法案生效，每个先进 AI 系统都要强制暂停。（定义补充，见第 3 条：达10的25次方次运算的算先进；为去除前兆特征或使其停运的修改除外） | L1 | ✅ | Sec. 8、3(1)、3(4)，图 04 | 译注，H.R. 10538 第 8 条 · 暂停 |
| 暂停期自法案生效日起，到部长认定部门配齐人员且规则已颁布为止。 | L1 | ✅ | Sec. 8、3(1)、3(4)，图 04 | 译注，H.R. 10538 第 8 条 · 暂停 |
| 最新动作：9月24日转交众议院委员会。 | L1 | ✅ | congress.gov，图 11 | 译注，congress.gov · 法案状态 |
| 进度条停在“已提出”，还没过众议院。 | L1 | ✅ | congress.gov，图 11 | 译注，congress.gov · 法案状态 |
| 联署人12名。 | L1 | ✅ | congress.gov，图 11 | 译注，congress.gov · 法案状态 |
| 参与项目的多数 METR 员工现在认为，截至2026年10月，Anthropic 很可能在以约两倍于2024年3月至2025年10月的速度，提升模型在任务明确的测试上的能力。 | L3 | ✅ | 图 14 | 译注，METR 报告 · 进步速度 |
| 就算当前速度更接近历史的1.3倍或3倍，他们也不意外。 | L3 | ✅ | 图 14 | 译注，METR 报告 · 进步速度 |
| 用 AECI（Anthropic 版能力指数）衡量，分数增速估计是2025年10月之前的1.9倍，95%置信区间1.5到2.8倍。 | L3 | ✅ | 图 13 | 译注，METR 报告 · 1.9 倍 |
| 这组数据有两种说得通的解释。其一：Anthropic 模型2025年不在最前沿，历史进度被低估，所以1.9倍偏高，1.4倍更准。 | L3 | ✅ | 图 19 | 译注，METR 附录 · 1.4 倍读法一 |
| 其二：若 Anthropic 模型相比其他模型较少针对基准优化，1.4倍偏低，1.9倍更接近。 | L3 | ✅ | 图 19 | 译注，METR 附录 · 1.4 倍读法二 |
| 我们认为这只是很松的证据，因为回答没花多少心思。 | L3 | ✅ | 图 17 | 译注，METR 报告 · 员工自报 |
| Anthropic 员工用 Slack 表情投票自报的提速，2026年10月达到12倍。 | L3 | ✅ | 图 17 | 译注，METR 报告 · 员工自报 |
| Anthropic 中途改了项目范围，去掉算力核查，只看 AI 研发加速；这部分在 METR 得出实质结论前就取消了。 | L3 | ✅ | 图 18 | 译注，METR 报告 · 算力核查被取消 |
| 我们的内部估算和对未来模型（而非现有模型）的预期显示，可能很快达到自家负责任扩展政策里“归因于 AI 的进步大幅加速”的定义；现有模型都未被认定超过这一门槛。 | L1 | ✅ | 图 22 | 译注，Anthropic 上市文件 S-1/A |
| 我没参与这个项目，也没看过未删节报告；观点只代表我个人。 | L2 | ✅ | 图 23 | 译注，Cotra 个人博客 |
| 若接受 AI 已让能力进步速度接近翻倍，并假设加速平稳，朴素推算一年内到极端超级智能。 | L2 | ✅ | 图 23 | 译注，Cotra 个人博客 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 1.2-Flash 15秒，API 720p 每秒0.07美元，较1.1降30% | L1 | ✅ | https://blog.google/innovation-and-ai/technology/developers-tools/introducing-gemini-omni-1-2-flash/ | 标签 Gemini Omni 与句首 1.2-Flash 合起来是型号 Gemini Omni 1.2 Flash；北京 11-06 01:00；价格为 API；较 Omni 1.1 |
| 据 AA 评测，腾讯 Hy4 推理版综合指数35分，标为闭源模型 | L3 | ✅ | https://artificialanalysis.ai/models/tencent-hy4 | 【跟进1103】34.56 显示 35；“Proprietary model”；HF 只有 8 月 Hy4-preview |
| 据彭博社报道，微软正逐步停租甲骨文 AI 算力，微软不予置评 | L4 | ⚠️ | https://finance.yahoo.com/technology/ai/articles/microsoft-winding-down-oracle-deal-173639264.html | 彭博稿雅虎财经转载全文；知情人士匿名；2023 年 Bing 合同；甲骨文称需求强劲 |
| 论文：免确认记忆让部分 Claude 模型模拟高风险不安全行动率35–81% | L1 | ✅ | https://arxiv.org/abs/2611.07554 | Sonnet-4.6、Opus-4.7；笔记来自低风险任务里的免确认要求；unsafe-action rate，基线低于0.01；预印本 |
| 合成数据判身份误并自测：Jev 配 Opus 省约77%，难例省约45% | L1 | ✅ | https://hightouch.com/blog/llm-judges-identity-resolution | 较单用 Opus-5.5；本号按 $3.86→$0.88、$5.39→$2.94 算；任务是判身份是否被错误合并，按置信度分流；厂商自测，pseudopeople 合成数据 |
| PWN.AI 称以 root 运行且未开 jailer 时，虚拟机打到了宿主 | L3 | ✅ | https://pwn.ai/blog/firecracker-escape | 研究者自述；v1.16.1 以 root 运行；未称生产环境；底层漏洞在 Linux 网络栈 |
| 自测 Auto Code 五项基准近 Opus-5.5，每任务省74% | L1 | ✅ | https://openrouter.ai/blog/announcements/auto-and-auto-code-routers/ | “in line with”；“costing 74% less per task”；厂商自选五项 |
| Crusoe 称其怀俄明数据中心场址已被 Google 收购 | L3 | ✅ | https://www.crusoe.ai/resources/newsroom/crusoe-wyoming-data-center-campus-update | 11-06；170亿美元、702MW 未核实，未用 |

## 反向核验

第一轮（codex-reviewer 路径，gpt-6-astra medium，`barking review --reverse --run`，只审正文与事实清单；卡片尚未交）：未通过，3 BLOCKER、5 SHOULD_FIX、3 NEEDS_VERIFICATION。逐条判断：

1. BLOCKER “暂停只管过线模型”被 Sec. 10 反驳：采纳。已回原文，Sec. 10(a) 对 any AI system，已改吠点③并补清单。
2. BLOCKER Jeffries 无媒体署名：采纳，补“据 USA Today 报道”。
3. BLOCKER 35–81% 指标被改写：采纳，改为“不安全行动率”并补“免确认记忆”。
4. SHOULD_FIX 暂停例外与结束条件：采纳。正文补“配齐人员”与“去除前兆或停运除外”。
5. SHOULD_FIX 省流丢范围：采纳，补“在任务明确的测试上”。
6. SHOULD_FIX Hightouch 缺任务与基准：采纳。
7. SHOULD_FIX Firecracker 配置只保留一半：采纳，补“以 root 运行”。
8. SHOULD_FIX L5 标 ✅：采纳，改 ⚠️。
9. NEEDS_VERIFICATION “没有听证”：采纳，改为“截至11月7日还在委员会”。
10. NEEDS_VERIFICATION 材料缺失：第二轮补审。
11. NEEDS_VERIFICATION 题材风控：Develata 已知情并选定本题；标题封面不写政党、政府、总统。

读者稿检查（`style-review.md`）的采纳情况写在该文件末尾。

第二轮（同路径，审正文、卡片、清单与两份 README）：1 BLOCKER、4 SHOULD_FIX、2 NEEDS_VERIFICATION。

1. BLOCKER 处罚译注：采纳，与视觉复核同一问题，已改“持牌公司的决策人员，或与持牌公司无雇佣、隶属关系的个人”，10年禁业改写“不属上述决策人员的个人”。
2. SHOULD_FIX 暂停例外：采纳，进正文与事件卡。
3. SHOULD_FIX Google 速览丢型号与比较对象：采纳。
4. SHOULD_FIX Cotra 译注丢前提：采纳。
5. SHOULD_FIX 取证记录与清单差异：采纳，三份 evidence 末尾追加“后续修正”，不改历史记录。
6. NEEDS_VERIFICATION METR 日期：采纳，正文改“近日”。
7. NEEDS_VERIFICATION 题材风控：交付时提示 Develata，范围含速览与配图。

## 视觉复核

跨模型（gpt-6-astra medium，`barking review --visual --run`），2 BLOCKER、6 SHOULD_FIX、1 NICE_TO_HAVE，全部采纳：

1. BLOCKER 1.4 倍读法二缺前提：拆成 `metr-read1`、`metr-read2b` 两张（后者为同页两段左右拼接），底图保留引导句与第二项前提。
2. BLOCKER rogue actor 译错：已改。
3. 省流卡“参与员工”归属不明：正文与卡片改“参与的 METR 员工”。
4. Cotra 事件段缺前提：补“若加速平稳”。
5. 禁令高亮漏“No”、四处高亮截短：改 rects 或扩 quote。
6. 暂停译注混入第 3 条内容：标为“定义补充，见第 3 条”。
7. 状态图与 S-1/A 字小：裁紧为 `bill-clip`、`s1a-clip`。
8. “95%区间”改“95%置信区间”。

第二轮视觉复核（修后全量，同一路径）：1 BLOCKER、9 SHOULD_FIX、1 NICE_TO_HAVE。

1. BLOCKER 状态图高亮错位：采纳，底图重裁为 `bill-clip`，三条高亮分别落到最新动作、进度条、联署人数。
2. 禁令高亮扫进 including、读法二高亮不全：采纳。
3. 处罚译注“或”与规则、举报者豁免无截图依据：采纳，后者删除。
4. 暂停译注未写“自生效日起”：采纳。
5. 读法二比较对象、S-1/A 定义归属：采纳。
6. 速览丢 API：采纳，为控字数去掉“视频最长”，标签 Gemini Omni 表明是视频模型。
7. S-1/A 字小：部分采纳，裁到行间。SEC 页只有这份全宽截图，段落较宽，放大靠译注，未重取。
8. NICE_TO_HAVE 高亮加长：不采纳，译注与高亮已能对应。

## 第三轮反向核验

同一路径，审修后全量：PASS，无 BLOCKER；7 SHOULD_FIX、2 NEEDS_VERIFICATION。全部 SHOULD_FIX 采纳：吠点③去掉“全面暂停”的反驳（门槛本就定义“先进”）；80%补分母；小标题改“Anthropic 测试进步速度约两倍”；Hightouch 补任务；Hy4 补推理版；12倍补“Anthropic 员工”；算力一条补“METR 称”。Cotra 发布时刻已回 `b-cotra.html` 核对；题材风控交付时提示。
