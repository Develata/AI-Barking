# 1005 C 组取证

结论：一手材料支持“两种计算候选材料”，不支持“已实验证明两种室温零磁矩半导体”或“已做出下一代内存”。YBaMnFeO₅ 是尚未制成的设计；KV[Cr(CN)₆] 的含水粉末在 1999 年已有磁性实验。室温磁有序、室温精确补偿、半导体带隙、自旋选择性、器件性能是不同待证事项。

取证窗口：北京时间 2026-10-06 09:13 起。逐次记录见 capture-log-c.md；原始记录 c-capture-records.jsonl。浏览器仅使用 `1005-c`。L1 表示发布者对自身工作的正式声明；论文为原作者/出版社一手研究记录（在本表按 L1 标注），均不等于本执行方已复现实验。社区均为 L6。

## 证据表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 博客发布两种计算候选，而非实验发现 | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | Both are predicted to have zero net magnetism yet still sort electrons by spin: one a new compound we designed, the other a material first made in 1999. | 15-c-summary.png | 开头；c-blog.txt 全文，署名 Geby Jaff；2026-10-04，官方未给时刻/时区 | 已找到 |
| C1 候选 1 的 2.35 eV、1.0/1.4 eV、420/490 K | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | about 420 K in the raw simulation, or about 490 K after calibrating the simulation against a known magnet | 16-c-designed.png | Candidate 1；带隙/窗口为 HSE06；磁序温度分别是原始模型/校准估计，非实验 | 已找到 |
| C1 Mn/Fe 棋盘排列约 950 K 失序，标准合成可能困难 | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | A scrambled crystal loses the spin sorting, so this design may be hard to make in its useful form. | 16-c-designed.png | Candidate 1；正文比较 950 K 与 900–1300 °C，不可混用温标；不能改成“不可能合成” | 已找到 |
| C1 候选 2 已有 1999/2008 先行工作 | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | Its zero net magnetism is not new: the chemists who made it designed the two metals’ magnetism to cancel. Even its spin sorting was already on paper. | 17-c-prior-work.png | Candidate 2；新意范围由作者限定为识别、窗口量化、稳健性测试；“As far as we found”不是穷尽文献证明 | 已找到 |
| C1 理想干晶体预测与含水粉末不能互换；带隙和自旋分选未测 | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | Neither the band gap nor the spin sorting has been measured yet. | 18-c-water.png | Candidate 2 末段；HSE06 与 PBE+U 对含水影响不一致；0.125 μB/f.u. 是实际粉末残余磁矩 | 已找到 |
| C1 bottom line 与 score 保留预测条件和下一步实验 | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | The next step is to make KV[Cr(CN)₆] again and measure its spin sorting directly. | 19-c-bottom-line.png | The bottom line；“in our calculations for perfect crystals”限定仍在 | 已找到 |
| C2 README 和 17 条 caveats 全文存档 | L1 | https://github.com/spicylemonade/compensated-magnet-ledger/blob/main/LEDGER.md | Room-temperature compensation was not computed for either material. | 20-c-readme-status.png；21-c-readme-caveats.png；22-c-caveats-general.png；23-c-caveats-kvcr.png；24-c-caveats-design.png | c-repo-readme.md、c-ledger.md；逐条原文另存 c-caveats-verbatim.md；0 K 理想共线自旋，不含可靠 SOC/轨道磁矩 | 已找到 |
| C2 一键检查器证明物理结论已经独立复现 | L1（源码核对） | https://github.com/spicylemonade/compensated-magnet-ledger/blob/main/tools/verify.py | python tools/verify.py | 无；源码存档 | README 报 58 pass/0 fail/3 not computable，日期 2026-10-04；本执行方未运行。源码 Y21/Y22/Y25 默认读取已有结果，不自行启动 Level 2；--repro 也解析已保存输出。检查通过不能推出实验验证 | 与说法不符 |
| C2 仓库有分级重跑材料 | L1 | https://github.com/spicylemonade/compensated-magnet-ledger/blob/main/reproduce/RESULTS.md | We did exactly this in a fresh cloud container for four sets of runs, and all 16 checked values match the originals | 无；README与RESULTS文本存档 | 摘句来自 README；c-rerun-results.md 是记录。Level 2 拟合/MC：417 K 对 414±4 K；有序模型 915/965/915 K。Level 3：QE 7.5 新 Modal 容器、同输入/赝势；作者自行重跑，不是外部团队复现；匹配依既定容差，不是全部逐位相同 | 已找到 |
| C2 约 750 任务、Modal、日期范围 | L1 | https://github.com/spicylemonade/compensated-magnet-ledger | The Track-L lane submitted about 750 jobs, most of them not part of this repository. | 21-c-readme-caveats.png | How this was produced：1–4 October 2026；QE 7.5、Modal cloud CPUs；仓库存 876 inputs（868 pw.x + 8 后处理），口径不同，不擅自合并 | 已找到 |
| C2 90+ agents、3 天 | L1 | https://x.com/ValsAI/status/2107204457738256749 | In 3 days, 90+ Opus 5.5 agents helped us uncover two room-temperature magnetic semiconductor candidates in simulations: YBaMnFeO₅ and KV[Cr(CN)₆]. | 无 | c-vals-oembed.json 官方公共嵌入英文；2026-10-06 04:21:07 北京（原 UTC 10-05 20:21:07）；自报数量/时长，未独立计数 | 已找到 |
| C2 仓库提交与修订可追溯 | L1 | https://github.com/spicylemonade/compensated-magnet-ledger/commits/main/ | Say YBaMnFeO5 may be hard to make rather than that it can't be made | 无 | c-repo-commits.json 共 11 条；所取最新 45551de5fac4e69de3e03c420b31abb335d73ef4；完整时间见下表 | 已找到 |
| C3 1999 实验论文和含水样品，376→365 K | L1（原论文） | https://pubs.acs.org/doi/10.1021/ja990946c | temperature of 3 decreases from 376 to 365 K upon repeated heating of the material to 400 K. | 无；PDF 整页渲染在 sources/c-holmes-page-1.png、c-holmes-page-2.png | Holmes & Girolami，JACS 121(23),5593–5594；ACS 正文挑战页失败；已从作者实验室官网取得原 PDF，见下文官方地址。第5593页样品 KVII[CrIII(CN)6]·2H2O·0.1KOTf，微晶粉末；第5594页磁性测量；1999-05-25 网刊日期，未给时刻 | 已找到 |
| C3 2008 杂化泛函出处；同自旋带边图的独立核对 | L1（论文摘要）；L1（仓库转述） | https://iopscience.iop.org/article/10.1088/0953-8984/20/33/335231 | functionals containing 35%, 65% and 100% admixtures of Fock exchange. | 无 | Middlemiss、Lawton、Wilson，J. Phys.: Condens. Matter 20,335231，2008-07-31；官方摘要已读。全文订阅墙，Fig.3a/Table3 未直接核实，不能把仓库的读图结论写成执行方独立证实 | 部分支持 |
| C3 2025 两种 LCM 候选都在室温以下失序 | L1（原论文） | https://arxiv.org/abs/2502.18136 | their Neel temperatures are below room temperature. | 无；sources/c-guo-page-3.png、c-guo-page-4.png | Guo et al.，Luttinger compensated bipolarized magnetic semiconductor，v1；第3页 Fig.4：Mn(CN)₂ 210 K、Co(CN)₂ 75 K，U=4 eV，经典 MC/Heisenberg；第4页结论。2025-02-25 20:01:29 北京（原12:01:29 UTC）；PDF 题头日期26日，与提交元数据分别保留 | 已找到 |
| C4 补偿亚铁磁、LCM 与 altermagnet 有不同对称性条件 | L1（APS 权威背景） | https://physics.aps.org/articles/v17/4 | two (or more) opposite-spin sublattices, which are not related by any crystal symmetry. | 无 | Igor Mazin，Altermagnetism Then and Now，2024-01-08，未给时刻；补偿亚铁磁子晶格不由晶体对称性关联；见下文释义，不据此裁判具体材料 | 已找到 |
| C4 altermagnet 的子晶格关联不是平移/反演 | L1（APS 权威背景） | https://physics.aps.org/articles/v17/4 | neither translation nor inversion | 无 | 原页讨论旋转等对称关联；不能把 LCM、altermagnet 和所有净矩为零的磁体当同义词 | 已找到 |
| C4 Luttinger 补偿的术语出处 | L1（APS 权威背景） | https://physics.aps.org/articles/v17/4 | Luttinger-compensated ferrimagnets | 无 | 原页：绝缘体/半金属每晶胞自旋磁矩整数约束，允许精确零；非相对论条件。c-aps-terms.txt 全文 | 已找到 |
| C5 Vals 身份 | L1 | https://www.vals.ai/about | The independent evaluator of artificial intelligence. | 无 | c-about.txt；这是公司对自身定位的描述，不表示此次材料预测已被独立第三方评测 | 已找到 |
| C5 Geby Jaff 与仓库、官方传播的关系 | L1 | https://www.vals.ai/blogs/room-temperature-magnetic-semiconductors | a public repository I created to document my research journey | 19-c-bottom-line.png | 博客署名 + spicylemonade 公开 GitHub 名称 Geby Jaff + ValsAI 官方帖互相支持。能确认作者项目由 Vals 正式传播；未确认雇佣职务、资助/组织归属，不称 Anthropic 官方材料研究 | 部分支持 |
| C6 HN 热度 | L6 | https://news.ycombinator.com/item?id=49970667 | 196 points | 无 | c-hn-page.txt：北京10-06 09:15:53，196分/152评论；09:13:56 API 为195分。动态快照，不替换扫描193分/146评论 | 已找到 |
| C6 X 官方帖热度 | L1（自身帖互动） | https://x.com/ValsAI/status/2107204457738256749 | 241718 次观看 | 无 | c-x-vals-main.json：北京10-06 09:25:20，82回复/297转发/2667赞/940书签/241718浏览；DOM 自动译为中文，英文原文另取官方 oEmbed | 已找到 |
| C6 X 转述热度 | L6 | https://x.com/Dr_Singularity/status/2107233044457218266 | AI agents just found two room temperature magnetic semiconductor candidates | 无 | c-singularity-oembed.json 原文保留 candidates；发布北京10-06 06:14:42（原10-05 22:14:42 UTC）；09:22:01抓取33回复/106转发/747赞/142书签/21861浏览 | 已找到 |
| C6 Reddit 热度 | L6 | https://www.reddit.com/r/accelerate/comments/1wyo3t1/ai_agents_discover_two_roomtemperature_magnetic/ | AI Agents Discover Two Room-Temperature Magnetic Semiconductors | 无 | c-reddit-accelerate.json：北京10-06 09:20:45，138分/16评论；部分折叠评论正文未展开，不声称完整讨论存档 | 已找到 |
| C6 AIHOT 条目/分数；AlphaSignal、Glosignal 转载 URL | L5 | https://aihot.news/all | — | 无 | 已查 AIHOT 首页及全部第1页；标题搜索输入未产生结果页，不等于全站没有。AlphaSignal/Glosignal 未定位可核实对应原条目；不编造分数/URL | 未找到一手来源 |
| C7 实际英文夸大标题漏掉候选限定 | L6 | https://www.reddit.com/r/accelerate/comments/1wyo3t1/ai_agents_discover_two_roomtemperature_magnetic/ | AI Agents Discover Two Room-Temperature Magnetic Semiconductors | 无 | EuphoricTomorrow2440；北京10-06 07:54:25.510（原10-05 23:54:25.510 +0000）；标题无 candidates、模拟、未实验，正文仅源链接；源文有限定 | 已找到 |
| C7 中文夸大实例 | L6 | — | — | 无 | 搜到含“候选”的中文转述线索，但没有经原站核实、确实省略限定的中文实例；不凑数 | 未找到一手来源 |
| C8 有依据的社区质疑 | L6 | https://news.ycombinator.com/item?id=49971175 | — | 无 | HN 用户 tedsanders 自称磁性材料博士；身份未核；质疑磁体分类科普过简和应用条件缺口。评论得分不可见/API为空，不能称已核实高赞。具体清单见下节 | 已找到 |

## 原始材料、方法与限制

- 1999 作者实验室官网 PDF：https://girolami-group.chemistry.illinois.edu/publications/publications/J.%20Am.%20Chem.%20Soc.%201999%2C%20121%2C%205593.pdf 。作者 Stephen M. Holmes、Gregory S. Girolami；标题 Sol-Gel Synthesis of KVII[CrIII(CN)6]·2H2O: A Crystalline Molecule-Based Magnet with a Magnetic Ordering Temperature above 100 °C。原文第5594页饱和磁化强度为5 K、40 kG下0.7 kG cm³ mol⁻¹；约0.125 μB/f.u.是单位换算/后续引用口径，不冒充该页逐字印出的数字。论文图注的杂质写法与正文略异，未据此重建化学计量。实际粉末不等于计算的理想无水晶体。
- 2008 标题：A solid-state hybrid density functional theory study of Prussian blue analogues and related chlorides at pressure。c-iop.txt、c-middlemiss-doi.json 支持书目和方法；仓库 caveat 11 所称 Fig.3a 同自旋带边、Table3 4.01 eV、20%交换外推2.1 eV，均未从订阅全文独立复核。另一篇 Kabalan 2008 不是这篇杂化泛函论文，不能替代。
- 2025 官方全文：https://arxiv.org/html/2502.18136v1 ，PDF https://arxiv.org/pdf/2502.18136 。v2 猜测地址无文档，保留失败文件。v1原文、PDF及关键页都已检查。
- 补偿亚铁磁的相反自旋子晶格可无晶体对称关联；Luttinger整数约束在相应电子结构与近似下使自旋磁矩为零；altermagnet 则由特定晶体对称联系相反自旋子晶格。APS给出了这些区别。博客将LC放在宽泛“antiferromagnet”科普框架，仓库采用 compensated ferrimagnet/LCM 更细称谓；本组只交术语背景，不自行裁定分类错误。
- `c-verify-source.py` 是下载的仓库源码，没有运行。`python tools/verify.py` 的 Y21、Y22、Y25 读取已有 JSON；Level 2 应另运行 README 所列拟合/Monte Carlo脚本。`--repro` 读 `reproduce/results/`；没有调用 QE 重跑。作者自报61项分成52项原始输出重算、3项可由附带脚本重跑、3项分析文件、3项文献值，不能将一键命令概括成61项独立物理复现。
- 独立新容器重跑仍是同一项目发布者的记录。SSSP替换赝势测试属于敏感性检验，不是同输入逐位一致的复现。含水HSE06旧计算在12个交换循环停止，新算41循环收敛：带隙2.14、空穴窗口2.31、电子窗口1.40 eV；旧窗口2.43/1.42 eV不再当现行结果。
- 社交存档只采公开帖子/评论字段，未保存抓取者侧栏、头像、账号。X DOM 中自动翻译文本不是原文；逐字英文只用公开 `publish.twitter.com/oembed`，该接口长帖末尾截断，不冒充全文。两条关键候选句均在截断前。

## 可能的吠点

1. “室温磁有序”并不保证“室温净磁矩恰好为零”：仓库明确两种材料都没算室温补偿，只有0 K理想共线自旋结果（LEDGER caveats 1、5）。
2. 研究代理漏掉2008先行工作，后来外部读者检查促使收窄创新声明；这不是从零发现一种此前未知化合物（README corrections、caveat 11）。
3. 新设计的关键负面结果是有用原子排列难以在标准合成中得到；950 K本身也来自暂定模型，部分模型变体达1210 K，不能写成绝对无法合成（caveats 12–14）。
4. 热稳定性凸包距离曾从+2.6改到+13.7 meV/atom，因迟完成竞争相改变比较；“检查器通过”不抹除这些修订（README corrections、caveat 17）。
5. 含水HSE06曾被错误标成收敛，后重跑改了窗口；PBE+U仍给出显著更弱空穴窗口，方法分歧尚在（caveat 7）。
6. 金属载流子会重、有空穴极化子，不能由大自旋窗口推出硅级输运或现成内存器件（caveat 9）。
7. 只公开LCM搜索分支；未达门槛的其他分支及代理文献笔记未包含，无法从本仓库核实整个90+代理搜索全过程（README How this was produced）。

## 社区质疑（仅线索 L6）

| 原站评论 | 作者/得分（抓取时） | 内容与边界 |
|---|---|---|
| https://news.ycombinator.com/item?id=49971175 | tedsanders；得分未显示/API null | 自称相关博士，质疑开头二分法省略抗磁/顺磁，以及从计算属性到可用材料仍需许多条件；身份未核，不当专家定论。 |
| https://news.ycombinator.com/item?id=49971436 | rsfern；得分未显示/API null | 原评论曾混淆超导，后自改半导体；不能截其更正前误读当科学反驳。其有限温度疑问可回到caveat 1核实。 |
| https://news.ycombinator.com/item?id=49971423 | contemporary343；得分未显示/API null | 将成果类比本科高年级/研究生初级DFT工作，属于评价，不证明计算无效。 |
| https://news.ycombinator.com/item?id=49971223 | otterley；得分未显示/API null | 指第二种材料已有发现历史；1999出处已另行核实。 |
| https://www.reddit.com/r/accelerate/comments/1wyo3t1/comment/pe4fnfc/ | aKaizuh；37分 | 原文：Candidates.；纠正标题遗漏限定。 |
| https://www.reddit.com/r/accelerate/comments/1wyo3t1/comment/pe4fw9h/ | Mrp1Plays；56分 | 原文：Note this isn't room temperature super conductors；澄清半导体与超导体，不是对研究方法的反驳。 |

没有找到可确认得分、且直接针对本案U值/带隙误差/altermagnet分类的专业高赞论证；相关技术限制来自仓库一手caveats，不借社区评论包装成权威审稿。

## 夸大说法实例

已核实英文实例为上述 Reddit 标题（候选限定缺失）；原链接的源博客明确保留预测。Dr_Singularity 的正文虽然说“discovery”，仍明确写 candidates，故不列作已经证实的无条件发现谣言。未找到可核实中文夸大实例。搜索所得 Weibo/AlphaLab 线索标题反而保留候选，不把中文传播一概写歪；未取得其完整原站证据，不作为正式取证行。

## 发布时刻与提交时间

| 对象 | 北京时间 | 原始时间依据 |
|---|---|---|
| Vals 博客 | 无法换算：只给2026-10-04日期 | 页面10/04/2026，meta article:published_time=2026-10-04，无时区/时刻 |
| ValsAI 官方 X | 2026-10-06 04:21:07 | DOM time datetime=2026-10-05T20:21:07Z |
| HN提交 | 2026-10-06 05:00:21 | Algolia created_at=2026-10-05T21:00:21Z |
| Dr_Singularity X | 2026-10-06 06:14:42 | 2026-10-05T22:14:42Z |
| Reddit主帖 | 2026-10-06 07:54:25.510 | 2026-10-05T23:54:25.510000+0000 |
| 仓库最早可见提交 e6b5405 | 2026-10-05 04:18:12 | committer 2026-10-04T20:18:12Z；不是仓库创建时刻证明 |
| 合成措辞修订 1f3bfc0 | 2026-10-05 05:02:10 | 2026-10-04T21:02:10Z |
| 收窄2008创新声明 943979b | 2026-10-05 06:54:08 | 2026-10-04T22:54:08Z |
| 作者称读完2008全文 071c13e | 2026-10-05 07:15:51 | 2026-10-04T23:15:51Z；不等于本执行方读到全文 |
| 本次最新提交 45551de | 2026-10-05 10:35:53 | 2026-10-05T02:35:53Z，含水HSE收敛修订 |

## 扫描说法勘误

- 90+/3天：博客全文未见这两个数，现已在 ValsAI 官方 X 原文找到；750/Modal来自仓库，不能把三者都说成博客正文数字，也不能说本组独立审计了计算账单。
- 候选1“难以合成”：官方措辞为 may be hard，支持可能困难；不支持不可能。提交日志明确记录删去 can't be made。
- 候选2“1999/2008”：1999原论文已取得；2008出处、摘要已取得，关键图结论仍只获作者转述。材料并非2026首次合成。
- HN扫描193/146与本次195 API、196/152页面是不同时间快照，均保留来源时间，不挑较大数写成固定热度。

## 假设与未验事项

本组只作来源取证和源码阅读，不启动科学计算、不做新证明、不代替同行评审。将同名GitHub公开资料、博客自述和互链用于确认项目作者；未推断员工职级/资金关系。作者“only sample”“never reproduced”“as far as we found”是其检索结论，本组没有做穷尽文献检索。所有“未找到”仅限本次范围。新增文件完整清单及SHA-256见 c-file-manifest.tsv；验证与失败见 capture-log-c.md。

