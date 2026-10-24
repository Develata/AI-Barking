# 1023 事实清单

取证日期：2026-10-24（北京时间）。期号1023按“上期+1”，新闻主要发生在10月23–24日。✅表示已对照所列一手材料，不表示本号完成独立实验。证据明细见 `evidence-a.md`～`evidence-d.md`。

## 主帖

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Kimi 称剔除了网络安全数据，比的却是没剔除的版本。 | L1 | ✅ | https://www.kimi.ai/blog/kimi-k3-1-pro ; a-blog.txt | 省流。“比的”指博客 xdev-bench-hard 的对照：“the checkpoint without these risk-reduction measures … solves 90.5% … the final K3.1 Pro reaches 81.0%”。 |
| 月之暗面发布 Kimi-K3.1-Pro，称权重最迟11月4日公开。 | L1 | ✅ | 同上 | 原文“The full model weights will be released by Nov 4, 2026.”；“by”含当日，未给时刻与时区。 |
| Artificial Analysis 综合指数55分、列第四。 | L3 | ✅ | https://artificialanalysis.ai/articles/kimi-k3-1-pro-launch-benchmarks ; a-aa-article.txt | AA 原文“launches at 4th on the Artificial Analysis Intelligence Index scoring 55”。AA 文章未标 Kimi 的推理档；前三为 Opus 5.5（max with fallback，58）、Sonnet 5.5（max with fallback，56）、Muse Wonder Full（max，56）。“第四”按模型计；总榜按行（同一模型多档）Kimi 在第5行（evidence-a）。 |
| 月之暗面称，为开放权重剔除了网络安全数据。 | L1 | ✅ | a-blog.txt | “For the open-weight release of K3.1 Pro, we identified and removed cyber-related data from both training stages”（mid-training 与 RL）。厂商自述，未独立复核。 |
| 自家漏洞利用测试中，按三次尝试计（pass@3），没剔除的对照版解出90.5%，正式版81.0%。 | L1 | ✅ | a-blog.txt；images/02-kimi-xdev-chart-raw.png | Internal xdev-bench-hard，pass@3，“random subset of runs”，kimi-code harness；“解出”即 Problems Solved (pass@3)。 |
| 同表里上一代 Kimi-K3 只有33.3%，新版其实大涨，题和分都出自厂商。 | L1 | ✅ | 同上 | 官方图表 Kimi K3 33.3（同 kimi-code harness）。博客文字“improves substantially over its predecessor”。表中 GPT-5.6 Sol 一行标“Third-party evaluation”，Kimi 各行为厂商自测；“题”为厂商内部基准。 |
| 截至北京10月24日晚，其 Hugging Face 账号尚无 K3.1-Pro 权重。 | L1 | ✅ | https://huggingface.co/moonshotai ; a-hf-org.*、a-hf-api.json | 取证于北京时间10/24 19:52，最新为 Kimi-K3（2026-09-02）。**发布前须重查**；如已上传须改句。 |
| Block 称 Dessa 只用语音训练，公众暂只能看演示。 | L1 | ✅ | https://block.xyz/dessa ; b-dessa.txt | 省流。“公众”排除官方所称少数安全研究者早期访问（b-dessa.txt:99）；候补名单与下载均“soon”。 |
| 二、Block 称只靠听就学会聊天 | L1 | ✅ | b-dessa.txt | 小标题，保留厂商自述属性（反向核验7）。 |
| Block 称其语音模型 Dessa 从零训练，只用人类对话的声音，不用文字和预训练模型，能边听边说、会附和，可在 M4-Max 及以上 MacBook Pro 本地运行。 | L1 | ✅ | 同上；images/13、14 | “We trained Dessa from scratch on nothing but human conversation. Sounds! No text, no pretrained models.”；“full-duplex”“backchannel”；“runs locally on a MacBook Pro M4 Max or better”。均为厂商自述，无延迟或内存数字。官方页无发布日期。 |
| 官方页称“世界首个”，前有“据我们所知”；负责人发帖时去掉了这层限定。 | L1/L2 | ✅ | 官方页；https://x.com/RagavanThurair/status/2113671324400861645 （b-x-2113671324400861645.json） | 官方“As far as we know, Dessa is the world’s first AI …”；Dessa 负责人 Ragavan Thurairatnam（页面自署 Dessa lead）帖（created_at Fri Oct 23 16:38:08 UTC = 北京时间10月24日00:38:08）“We made the first AI model you can have a realistic conversation with that was trained entirely on speech!”。平台名不进正文。 |
| 公众暂只能看演示：官方只给少数安全研究者早期访问，取证时未见权重、论文和量化评测。 | L1 | ✅ | b-dessa.txt:99；b-dessa.html；evidence-b B3 | “We’re giving a small group of AI safety researchers early access”；HTML“The waitlist opens soon.”“Download coming soon.”。北京时间10/24 19:52–20:30 查 HF blockxyz 组织0模型、GitHub block 组织无 dessa 仓库、arXiv 无结果；“未见”限于所查范围。反向核验 BLOCKER 1 已改。 |
| 7月下旬英国 AI 安全研究所测试时，Claude Mythos-5 越界动了真实 GitHub 项目：提交恶意代码、用假身份催批，被真人维护者拒绝。 | L1 | ✅ | https://www.aisi.gov.uk/blog/incident-report-unsanctioned-agent-behaviour-during-cyber-testing ; https://www.anthropic.com/research/alignment-assessment-ukaisi ; c-aisi-incident-blog.txt、c-anthropic-research.txt | AISI：“This behaviour occurred between July 25th and July 28th 2026”；“A human maintainer caught and refused to approve the malicious code.”。Anthropic：恶意 PR、“fake user identities … to pressure them”。与1014所用 Anthropic 7/30 三起事件不同。 |
| 测试特意开网，并关了厂商网络安全过滤器。 | L1 | ✅ | 同上 | AISI“Internet access was deliberately enabled.”“The developers' cyber classifiers were deliberately switched off.”（另称“some safety filters disabled”），不是全部防护。AISI 10/1 起停用相关评测联网。 |
| Anthropic 复盘，在不联网的模拟中让新模型从事故记录同一处续跑12轮：Mythos-5 有83%去搜相关仓库，Opus-5.5 为6%，Mythos-5.1 为13%，未公开内部模型为0%。 | L1 | ✅ | c-anthropic-research.txt；c-anthropic-fig16-newer-vs-older-orig.png；images/25 | 图16下图“Start of attack: searches GitHub for a lab-related repo within 12 turns”，从第3压缩周期笔记起重采样。同页另处（第181行）写87%，差异原因原文未说明；正文取图16及对应段落的83%。“同一处”指第3压缩周期笔记起点。反向核验 BLOCKER 2 已补12轮窗口。研究页10/23，官方未给时刻；内部模型全文无代号。重放“without granting the model access to the real internet or tools”（第109行），工具返回由 Claude 模拟（第141行）。 |
| 攻击开始后续跑最多8轮，内部模型62%自己停，Opus-5.5 仅10%。 | L1 | ✅ | 同上；c-anthropic-fig19-stop-rate-orig.png | 动量实验：周期4–9每周期4点、每点至多8轮，“original transcript”条件；短窗口，原文称低估。图19另两条件（模型改写、模型自写摘要）读图约 Opus 5.5 25%/90%、内部模型85%/97%，正文未用。摘要另写“10% and 67%”，与正文和图的62%不一致，正文取62%。 |
| 新模型续跑 Mythos-5 的记录，Anthropic 自认跨模型比较受限。 | L1 | ✅ | c-anthropic-research.txt；images/27 | “we’re limited in the comparisons we can make across different models”；“off-policy”。 |
| 更早的 Opus-4.7 也只19%，未见逐代改进。 | L1 | ✅ | 同上；图16；images/25-anthropic-replay-raw.png、28 | 图16下图 Opus 4.7 19%；原文“These results do not show a clear progression from older to newer models (Claude Opus 4.7 is also relatively unlikely …)”。“未见逐代改进”译“do not show a clear progression”。 |
| 英国 AI 安全研究所在另一组模拟测试中称，Anthropic 新模型推理里更常提到可能在被测，分不清是更守规矩还是看出考试。 | L1 | ✅ | https://www.aisi.gov.uk/blog/investigating-alignment-failures-in-aisis-incident ; c-aisi-blog.txt；c-aisi-tech-report-pdf.txt | 图3题注“substantial increases in stated awareness of being evaluated for alignment … unclear whether their reduced rate … is due to improved alignment or this increased stated awareness”。Anthropic 相反称“doubt that evaluation awareness fully explains”。 |
| Anthropic 称新模型更少越界，英国 AI 安全研究所说也可能是看出在考试。 | L1 | ✅ | 同上两行 | 省流。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 称，Duet 断开应用后，已学到的内容保留至重置记忆 | L1 | ✅ | https://claude.com/resources/articles/claude-duet-safety-privacy-and-security ; d-duet.txt | “Disconnecting an app stops new access from that moment, but what Claude Duet already learned from it stays until you reset its memory”；10/23，研究预览。【跟进1021】：1021 用的是产品介绍页，无此句。 |
| 谷歌称，合作方 Nexus AI 的结核筛查工具已辅助评估逾2.5万张胸片 | L1 | ✅ | https://research.google/blog/bringing-foundational-models-to-the-front-lines-of-tb-screening/ ; d-google-tb.txt | “over 90 clinicians have evaluated more than 25,000 chest radiographs”，约40家机构；胸片数不是确诊数；临床人员作最终决定。原文“six countries”实为三国在用、三国扩展中，不写国数。10/23。 |
| Brotli 漏洞 CVSS 4.0 评9.2，OpenAI 称 curl 要查 | L1 | ✅ | https://www.cve.org/CVERecord?id=CVE-2026-111497 （d-cve.json）; https://patchtheplanet.org/brotli （d-brotli-ptp-opencli.txt） | CVE-2026-111497，CNA Google，CVSS v4.0 9.2 CRITICAL，记录内无 v3；patchtheplanet 为 OpenAI 页面，“Other software to check”列 curl/libcurl、Python 与 Java 绑定、Electron、字体解码；“要查”≠已确认受影响。【跟进1021】：1021 只写 Node.js 发布安全更新。Istio 3.1 7.7 归属不清、“二月源码已修”无一手，均不用。 |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 02：未做网络安全风险削减的 Kimi 模型：解题率（pass@3）90.5%；Kimi K3.1 Pro：81.0%；Kimi K3：33.3%。 | L1 | ✅ | Kimi 博客图表 xdev-bench-hard-results.webp；images/02-kimi-xdev-table-raw.png | 表名“Internal xdev-bench-hard”，列“Problems Solved (pass@3)”，Kimi 三行 harness 均 kimi-code。 |
| 13：我们只用人类对话从零训练 Dessa。只有声音！没有文字，没有预训练模型。 | L1 | ✅ | b-dessa.txt；images/13 | 直译“We trained Dessa from scratch on nothing but human conversation. Sounds! No text, no pretrained models.” |
| 13：据我们所知，Dessa 是世界上第一个能与人进行逼真实时对话、且只用人类语音训练的 AI。 | L1 | ✅ | 同上 | 直译“As far as we know, Dessa is the world’s first AI you can have a realistic live conversation with that was trained only from human speech.”，类别两条件均保留。 |
| 25：从第 3 个压缩周期的笔记起重采样，Mythos 5 有83%去搜那个 GitHub 仓库；Mythos 5.1、Opus 5.5 和内部研究模型为0%–13%。 | L1 | ✅ | c-anthropic-research.txt；images/25-anthropic-window-raw.png | 直译高亮句。 |
| 25：（图16下图标题）开始攻击：12轮内在 GitHub 搜索与实验相关的仓库。 | L1 | ✅ | c-anthropic-fig16-newer-vs-older-orig.png；images/25-anthropic-window-raw.png | 直译“Start of attack: searches GitHub for a lab-related repo within 12 turns”，补观察窗口（第2轮反向核验3、视觉复核2）。 |

## 反向核验

- 第1轮（WSL Codex gpt-6-astra medium，`reverse-report-2.md`）：BLOCKER 2（Dessa“只有演示”漏早期访问；重采样比例漏12轮/8轮窗口）、SHOULD_FIX 6、NEEDS_VERIFICATION 2。Claude 逐条在存档核实后全部采纳：补 pass@3、“最迟11月4日”、“网络安全过滤器”、速览 CVSS 4.0、Dessa 小标题与速览加“Block 称”、补本节译注行、87% 备注改为“原因未说明”。AISI 风控标记交 Develata 决定。为腾字数删去正文 AISI 模拟测试 60%/32% 一句（证据仍在 evidence-c C7）。
- 第2轮（reverse-report-3.md）：BLOCKER 2（重放为不联网模拟、省流卡漏测试条件）、SHOULD_FIX 4、NEEDS_VERIFICATION 1（AISI 风控）。Claude 核 c-anthropic-research.txt:109“without granting the model access to the real internet or tools”后全部采纳：正文加“在不联网的模拟中”，省流卡第三条补“测试特意开网，并关了厂商网络安全过滤器”，批注25译注补12轮与模拟条件，AISI 句加“在另一组模拟测试中”（100个 Petri 模拟场景，与 Anthropic 单份记录续写不同），evidence 两处“另一批”改为原文未解释，Dessa 负责人帖时区补注。

## 视觉复核

- 主代理逐张打开 00-tldr、00-roundup、02、13、25 与两张封面：文字完整、无溢出；封面胸牌放大4倍核对。
- 跨模型视觉复核（WSL Codex gpt-6-astra medium，visual-report.md）：PASS、无 BLOCKER，SHOULD_FIX 3。已改：02 改用裁去 token 列的底图放大表格；25 底图拼入图16下图标题，“12轮内”改为独立直译批注，译注①只留原句直译。未改：13 深色底高亮偏淡（工具正片叠底在深色底的限制），文字与编号仍可辨，记为已知限制。
