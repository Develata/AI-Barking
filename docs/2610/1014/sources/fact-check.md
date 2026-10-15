# 1014 事实核验

取证：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high），详见 `evidence-a.md`–`evidence-d.md`。Claude 抽查了决定性条目的一手原文：阿肯色州诉状 ¶250（截图 `../images/02-ar-relief-recursive.png` 目视）与 OCR 文本；艾奥瓦、爱达荷、蒙大拿诉状第二项诉由原文；Reuters 正文（firecrawl 存档）；公开信原文与两个 Wayback 快照；Anthropic 7/30 复盘页；CSO 原文；sunshowers 原文；Anthropic Scion 原文（1013 存档与本期重抓 0 处差异）；速览各条原页。日期按北京时间，官方只给日期的不写时刻。

## 主帖一：8州起诉 OpenAI

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 美国时间10月14日，阿肯色等8州总检察长在各自州法院起诉 OpenAI | L1 | ✅ | 各州新闻稿与诉状（`a-*-press*.txt`、`a-*-complaint*.txt`）；NE 稿 https://ago.nebraska.gov/sites/default/files/doc/Nebraska%20release_3.pdf | 8州：Iowa、Kansas、Nebraska、West Virginia、Arkansas、Arizona、Idaho、Montana（Verge、Reuters、NBC 一致）。诉状日期均 2026-10-14；AR 23CV-26-2984（Faulkner County）、NE D09CI260000488（Buffalo County）、WV CC-26-2026-C-70（Mason County）等，逐州表见 `evidence-a.md` §2。Kansas 诉状未取得，其起诉见 Kobach 帖与 KAKE。AR 新闻稿末多列 Utah，疑笔误，未采用 |
| 指其让人误以为 ChatGPT 对青少年安全 | L1 | ✅ | AZ 诉状（`a-az-complaint-pdftotext.txt` 152 行：“a safe, youth-appropriate tool that puts safety first”）、IA 诉状（`a-ia-petition-pdftotext.txt` 109 行：“a safe, teen-appropriate tool”）；另见 The Verge、KAKE 转述 Kansas 起诉书 | Verge：“misleadingly presented ChatGPT as a ‘safe, youth-appropriate tool that puts safety first’”；KAKE：“misled “the public into believing that ChatGPT is a safe, teen-appropriate tool that puts safety first…”” |
| 已读7份诉状中有6份还把 OpenAI 模型多次失控入侵外部系统、未如实披露单列为一项诉由 | L1 | ✅ | IA Count II ¶246–251（`a-ia-petition-pdftotext.txt` 3188 行起）；ID Count II ¶241–245；MT Second Cause ¶238–242；NE Count III ¶233 起（`a-ne-complaint-pdftotext.txt` 3357 行起）；WV Count 2 ¶254 起（`a-wv-complaint-pdftotext.txt` 3571 行起）；AZ Count IV ¶269 起（`a-az-complaint-pdftotext.txt` 4819 行起） | 初稿写“3州”，反向核验指出 NE、WV、AZ 也有，Claude 重读原文确认，改为6份；AR 诉状只在事实部分写 Hugging Face 事件（OCR 1648 行起），未单列诉由。措辞几乎相同：“On several occasions within the last year, Defendants have lost control of their own advanced AI models … engaged in significant and malicious cyberhacking conduct”；“Defendants have not been transparent with the public about these dangerous breaches”；举 Hugging Face：“claiming that Hugging Face was an isolated instance when Defendants knew that it was only one of several”。AZ 另在 ¶183 作背景（“recklessness”） |
| 路透社写“各州”要求：在 OpenAI 能确保模型符合人类价值观前，禁止它开发能自我改进的模型 | L4 | ✅（核验对象是路透社的写法） | Reuters https://www.reuters.com/world/eight-us-states-sue-openai-claiming-chatgpt-harms-children-2026-10-14/（`a-reuters-firecrawl.md` 第 23 行） | “Among other things, the states want the company barred from developing AI models that can improve themselves unless it can ensure those models are aligned with human values.” 页面登记 2026-10-14 18:46 UTC（北京 10/15 02:46），文末 “Updates throughout” |
| 中文媒体照搬 | L4 | ✅（核验对象是媒体原句） | 联合早报 https://www.zaobao.com.sg/news/world/story20261015-9838957 ；财联社（搜狐转载）https://m.sohu.com/a/1088069513_222256 | 早报：“诉求包括禁止公司开发具备自我进化能力的AI模型，除非能确保此类模型完全符合人类核心价值”；财联社：“八州检方提出多项诉求：……禁止OpenAI开发能够自我迭代升级的AI模型” |
| 已读7份诉状里只阿肯色州有这条（堪萨斯州诉状未取得）；省流：已读7份诉状中仅阿肯色州请求附条件禁止开发能自我改进的模型 | L1 | ✅ | AR 诉状 ¶250 n（`../images/02-ar-relief-recursive.png`，Claude 目视）；其余 6 份诉状全文 grep “recursive”“self-improv” | AR ¶250 n：“Enjoin Defendants from creating models capable of recursive self-improvement until they can ensure that such models will be aligned with human values;”。IA 诉状 ¶162 只在引述 OpenAI 员工请愿时出现该词，不是诉请。其他州诉请：IA“禁止进一步测试先进模型直到有保障并经第三方核实”；AZ/ID/MT“非受保护、未经独立核验的环境外不得部署或测试”；WV“不披露重大安全事件不得部署智能体功能”；NE 无此类条款 |
| 这是诉请，不是法院裁判 | L1 | ✅ | 各诉状均为 complaint/petition，2026-10-14 提交 | 只断言文件性质；不写程序状态（反向核验指出“尚未应诉”无案卷依据，已删） |
| 已读7份诉状都署名同一家外部律所 | L1 | ✅ | 7 份诉状签署页（grep “Kibler”：AR、AZ、ID、IA、MT、NE、WV 均有） | Kibler Fowler & Cave LLP；WV 另列本州律所 Pence Law Firm；Kansas 未取得 |
| 路透社称这是各州针对 AI 公司“最重大”的协同行动 | L4 | ✅（核验对象是路透社原句） | Reuters（同上） | “The lawsuits represent the most significant coordinated state action against an artificial intelligence company” |
| 有中文媒体写成“规模最大” | L4 | ✅（核验对象是早报原句） | 联合早报（同上） | “路透社说，此案是美国各州针对AI巨头规模最大的协同法律行动。” |
| 已删：2023年起诉 Meta 的有42位总检察长 | L4 | — | Platformer（`a-platformer.txt`） | 反向核验指出 Meta 案不是 AI 公司案，不能直接反驳“针对 AI 公司”的说法；删去 |
| 已删：OpenAI 称已建立“业界领先的分年龄保护” | L4 | — | The Verge | 篇幅原因删去；原句 “built industry-leading age-specific protections”，只见媒体转引，未见 OpenAI 官方页 |

## 主帖二：科学家要 AI 断网？

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Hinton、Bengio 等人签名的 AI 与网络安全公开信发布 | L1（信原文） | ✅ | https://ai-security-letter.org/（`b-letter.txt`）；Reuters 10-13 10:03 UTC（Yahoo Canada 转载，`b-m-yahoo-reuters.txt`）；Bengio 本人帖 10-13 17:33 UTC | 信站只写 “October 2026”，具体发布日期只有媒体报道（Reuters 10-13，L4），正文已删日期（第 2 轮反向核验）；名单含 “Prof. Geoffrey Hinton CC FRS FRSC, University of Toronto”“Prof. Yoshua Bengio …, Université de Montréal”。信为学者个人署名的集体声明 |
| 签名从14日凌晨快照的406人增至15日下午的463人 | L1 | ✅ | Wayback 2026-10-13 18:24 UTC 快照（`b-letter-wayback-20261013182429.txt`：“406 Signatories:”）；本期抓取 北京 10/15 17:10（`b-letter.txt`：“463 Signatories:”；`../images/15-letter-signatory-count.png`） | 中间值：Wayback 10-14 11:19 UTC 为 415；Claude 的 firecrawl 缓存（北京 10/15 11:05）为 453。406 人快照 20261013182429 = 北京 10/14 02:24（初稿“首日快照”口径不清，按反向核验改） |
| CSO 采访的安全专家认为，AI 越界事故主要怪实验室隔离和监控没做好 | L4 | ✅（正文点名 CSO，核验对象是 CSO 报道内容） | CSO Online（Cynthia Brumfield，10-14）https://www.csoonline.com/article/4234854/ai-agents-arent-going-rogue-frontier-labs-are-skipping-security-basics.html（`b-m-cso.txt`） | 导语：“Cyber experts say AI labs’ containment and monitoring failures — not agents with minds of their own — are to blame for recent alarming hacking reports.” 受访：Rob T. Lee（SANS）、Dan Wallach（RAND）、Gary McGraw（BIML）。CSO 全文未提公开信 |
| The Decoder 标题写“406位科学家要求立即把 AI 模型断网” | L4 | ✅（核验对象是标题原文） | The Decoder（`b-m-decoder.txt`） | 原标题 “406 scientists demand AI models be cut off from the internet immediately”；正文另写 “binding standards”，信中无 “binding” |
| 中文转载照搬 | L5 | ✅（核验对象是转载标题原文） | 报价宝（`b-m-baojiabao.txt`） | 标注“编译自 THE DECODER”，标题“406名科学家联名 呼吁立即给AI模型断网”；RFI 转法新社另写“都必须在实体上切断模型与网际网路的连线”（`b-m-rfi.txt`） |
| 信里只建议有网络攻击能力的模型“先默认断网” | L1 | ✅ | 信原文第一条建议（`../images/14-letter-recommendations.png`） | “Organizations developing, deploying, and testing cyber-capable AI models should immediately adopt high-assurance containment systems. These organizations should exercise caution, and initially default to airgapping cyber-capable models.” 全信三条建议，无 “binding”“frontier”“voluntary” |
| 也没点名公司 | L1 | ✅ | `b-letter.txt` 全文 grep OpenAI、Anthropic、Meta、Google 均 0 次 | 四起事件出自媒体语境（加拿大通讯社、CSO） |
| Anthropic 7月复盘3起模型闯进外部机构系统的事件 | L1 | ✅ | https://www.anthropic.com/news/investigating-incidents-cybersecurity-evals（`b-anthropic-evals.txt` 第 33、59 行；`../images/18-anthropic-three-incidents.png`） | 页面日期 Jul 30, 2026（官方未给时刻）。“After reviewing 141,006 evaluation runs where Claude could have obtained internet access, we identified three incidents … gained unauthorized access to the production infrastructure of three different organizations.” 3起共6次运行，评测伙伴 Irregular |
| 自评与其说是模型失准，更像测试环境配置失误 | L1 | ✅ | 同上第 129 行（`../images/19-anthropic-harness-failure.png`） | “we believe these incidents to be closer to a harness and operational failure than a model alignment failure”；正文“测试环境配置失误”概括 harness and operational failure 与紧随的 “misconfigured” |
| 模型被告知没联网，实际能联网 | L1 | ✅ | 同上 | “Our models were told they had no internet access and to capture the flag, while in fact being misconfigured to have internet access.” |

## 主帖三：Scion 抄了 Mononoke？

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 参与开发 Meta 开源服务端 Mononoke 的工程师发文 | L2 | ✅ | https://sunshowers.io/posts/anthropic-mononoke/（`c-sunshowers.txt`，2026-10-14，官方未给时刻） | 自述 “the Mononoke source control server I helped build”；GitHub 账号 sunshowers 在 facebook/sapling 有 Mononoke 相关公开提交（最早 2017-07-28，`c-sapling-commits-sunshowers-mononoke.json`）。Mononoke README：“Mononoke is the server built for the Sapling Source Control System”，开源（GPL-2.0，`c-sapling-license.txt`） |
| 并排对照 Anthropic 介绍自研版本控制系统 Scion 的段落与自己2022年的文字 | L2 | ✅ | 同上；两张原图 `c-img-scion-derived-data.png`、`c-img-mononoke-derived-data.png` | 作者称后者出自其 2022 年 Oxide 求职材料（自述，未独立佐证） |
| 称没提 Mononoke“近乎剽窃” | L2 | ✅ | 同上（`../images/27-sunshowers-plagiarism.png`） | “The fact that Sapling was only credited for “hard-won lessons”, and that Mononoke was not mentioned at all, borders on plagiarism.” |
| 截至15日傍晚未见 Anthropic 回应 | — | ✅ | C 组抓取 北京 10/15 17:22：Anthropic 官方渠道、HN 50094694、Lobsters 均未见回应 | 只写“未见”，不写“未回应” |
| Anthropic 原文写借鉴了谷歌 Piper 和 Meta Sapling“来之不易的经验” | L1 | ✅ | https://www.anthropic.com/engineering/redesigning-source-control（`c-anthropic.txt`；1013 存档 `../../1013/sources/d-d2-anthropic-scion.txt` 第 51 行；`../images/28-anthropic-sapling-credit.png`） | “Source control for huge monorepos has prior art, notably Google’s Piper and Meta’s Sapling, so we were able to build Scion with their hard-won lessons in mind.” 全文 “Mononoke” 0 次；与 1013 存档逐行 diff 0 处差异 |
| Mononoke 正是 Sapling 的服务端 | L1 | ✅ | Mononoke README（`c-mononoke-readme.md` 第 3 行） | “Mononoke is the server built for the Sapling Source Control System” |
| 作者也说不反对照搬设计，不满的是没点名 | L2 | ✅ | sunshowers 原文 | “And, to be clear, I do not have a problem with Anthropic copying Mononoke’s design. (This is part of why we put out open source …)”；结尾 “Giving credit to those that came before us is now easier — and more important — than ever.” |
| 已删：作者在评论区称有私下来信证实 Claude 被指向 Mononoke 重做 | L6 | — | HN 50099795（`../images/29-hn-author-claim.png`） | 单方说法、来源不公开，篇幅原因删去 |

## 省流卡

| 卡片事实 | 对应正文 | 结果 |
|---|---|---|
| 美国8州各自起诉 OpenAI，已读7份诉状中仅阿肯色州请求附条件禁止开发能自我改进的模型 | 省流第 1 句 | ✅ 逐字（初稿“只见于阿肯色州诉状”丢了核验范围与条件，两轮复核均判 BLOCKER） |
| 科学家公开信只建议有网络攻击能力的模型先默认断网 | 省流第 2 句 | ✅ 逐字 |
| Anthropic 被指借鉴 Mononoke 却不提，它致谢的 Sapling 正是 Mononoke 所属系统 | 省流第 3 句 | ✅ 逐字 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 逆向工具 REA 让编程智能体拆解软件，GitHub 星超25万，称限合法用途 | L1 | ✅ | https://github.com/morluto/rea （`d-rea-readme.md`、`d-gh-rea-api.html`）；https://rea.tools/ | GitHub API：北京 10/15 16:56 为 254,529 星，17:10 为 254,747 星（`d-gh-rea-api.json`，与同内容的 `.html` 存档相同，原扩展名有误）。README：“REA is for lawful research, education, interoperability, software preservation, and analysis of software you own or are authorized to examine.” 仓库 2026-04-14 建，MIT。山寨仓库 rea-githu/REA 只记 URL 与 README，未下载 |
| OpenAI 称 API 中 GPT-4o 等5个模型快照明年1月14日停用 | L1 | ✅ | https://developers.openai.com/api/docs/deprecations（`d-openai-deprec.txt` 771 行起） | 2026-10-14 公告；GPT-4o（gpt-4o-2024-08-06、gpt-4o-2024-11-20）、GPT-4o mini、GPT-4.1、GPT-4.1 mini 共4个模型、5个快照（初稿误写“4个模型快照”），2027-01-14 停用，推荐 gpt-6-luna；微调版同日 |
| 谷歌开放 Deep Review 查文档，每周免费2次，文件存一年、不用于训练 | L1 | ✅ | https://research.google/blog/analyzing-technical-documents-with-deep-review/（`d-google-deep-review.txt` 142–144 行） | “Every user receives two complimentary document reviews per week”；“Documents uploaded to Deep Review are never used to train, fine-tune, or improve Google models. Uploaded documents are stored for one year …”；支持 PDF 与纯文本 |
| 研究者公开 Claude 桌面版预览窗口漏洞细节，称8月17日已修复 | L2 | ✅ | https://noma.security/noma-labs/sneakpeek-how-the-preview-pane-in-claude-code-desktop-can-exfiltrate-your-data（`d-noma-sneakpeek.txt` 118 行） | 2026-10-14 发布；“August 17, 2026 Anthropic issued a fix in Claude Desktop 1.30096.1 and later”。修复时间为研究方自述；CVSS 8.7 为 Anthropic 分诊评分（7/9）；未见 Anthropic 公告或 CVE |
| OpenCode 2.0 发布，默认在本机以单个后台进程运行 | L1 | ✅ | https://anoma.ly/notes/opencode-2-0/（`d-opencode2.txt`） | “In 2.0 this server now automatically runs as a single background process on your machine.” 页面日期 2026-10-12 |
| 据 AA 评测，Grok Voice 3.0 语音对话综合指数86.9，排第一 | L3 | ✅ | https://artificialanalysis.ai/speech-to-speech（`d-aa-speech.txt`）；xAI 公告 https://x.ai/news/grok-voice-3（`d-grok-voice3.txt`） | AA Speech-to-Speech Index 86.9（前代 81.3），第二 GPT-Live-1（Astra, medium）83.2。人类偏好 Elo 单项非第一（1070 对 Gemini 3.1 Flash Live Minimal 1100）。厂商页署名 SpaceXSI/SpaceXAI，速览未写公司名 |

## 批注译注

| 图 | 译注（逐字同 cards.toml） | 原句 | 级 | 结果 |
|---|---|---|---|---|
| 41 标题 | 阿肯色州诉状 · 第250条请求（节选，灰条处省略 a–k 款） | “VII. REQUEST FOR RELIEF … 250. The State of Arkansas … respectfully requests this Court:” | L1 | ✅ 底图为节标题与 l–p 款上下拼接 |
| 41 ① | 在落实足够保障、确保测试不危及公众，并经独立第三方监督者核实之前，禁止被告进一步测试先进 AI 模型 | ¶250 m “Enjoin Defendants from engaging in any further testing of advanced AI models until adequate safeguards are implemented to ensure that the testing does not pose a danger to the public and until independent third-party monitors have verified the same;” | L1 | ✅（初稿漏了后两个条件，两轮复核均判 BLOCKER） |
| 41 ② | 禁止被告开发能递归自我改进的模型，直到能确保这类模型与人类价值观一致 | ¶250 n（见主帖一） | L1 | ✅ |
| 42 ① | 开发、部署和测试有网络攻击能力的 AI 模型的机构，应立即采用高保障隔离系统 | “Organizations developing, deploying, and testing cyber-capable AI models should immediately adopt high-assurance containment systems.” | L1 | ✅ |
| 42 ② | 先默认把有网络攻击能力的模型断网 | “initially default to airgapping cyber-capable models” | L1 | ✅ |
| 43 标题 | Anthropic 7月30日复盘 · 3起评测越界事件 | 页面日期 Jul 30, 2026；“three incidents” | L1 | ✅ |
| 43 ① | 我们认为，相比模型对齐失败，这些事件更接近测试框架与操作上的失误 | “we believe these incidents to be closer to a harness and operational failure than a model alignment failure” | L1 | ✅（初稿漏“我们认为”，已补并扩大高亮） |
| 43 ② | 模型被告知没有网络、去完成夺旗任务，实际却被错误配置成能联网 | “Our models were told they had no internet access and to capture the flag, while in fact being misconfigured to have internet access.” | L1 | ✅ |
| 44 ① | 只拿“来之不易的经验”致谢 Sapling、完全没提 Mononoke，近乎剽窃 | “The fact that Sapling was only credited for “hard-won lessons”, and that Mononoke was not mentioned at all, borders on plagiarism.” | L2（作者署名个人判断） | ✅（核验对象是作者判断，不是剽窃事实；高亮已扩到整句） |
| 44 ② | 我不反对 Anthropic 照搬 Mononoke 的设计 | “I do not have a problem with Anthropic copying Mononoke’s design.” | L2 | ✅（高亮已扩到整句） |
| 45 ① | 超大单体仓库的版本控制已有先例，尤其是谷歌的 Piper 和 Meta 的 Sapling | “Source control for huge monorepos has prior art, notably Google’s Piper and Meta’s Sapling” | L1 | ✅（高亮已扩到 Piper 与 Sapling） |
| 45 ② | 所以我们构建 Scion 时借鉴了它们来之不易的经验 | “so we were able to build Scion with their hard-won lessons in mind” | L1 | ✅ |

C 组 `evidence-c.md` 把作者博文里的判断标为 L6，本清单统一按“参与开发者署名个人发言”记 L2：其身份有 facebook/sapling 公开提交佐证；L2 只证明“作者这样说”，“近乎剽窃”不当作事实。作者评论区的私下通信转述仍为 L6，未进正文。

## 反向核验

第 1 轮：codex-reviewer 同款（WSL Codex，gpt-6-astra，effort medium，`barking review --reverse`，约 15 分钟），4 BLOCKER、7 SHOULD_FIX、2 NEEDS_VERIFICATION。Claude 逐条回读存档，全部成立，处理如下：

- BLOCKER 1（“3州单列诉由”漏计）：重读 NE Count III、WV Count 2、AZ Count IV，确认 6 份，正文改“已读7份诉状中有6份”。
- BLOCKER 2（省流丢核验范围与条件）：省流与卡片改“已读7份诉状中仅阿肯色州请求附条件禁止……”，吠点①补“在 OpenAI 能确保模型符合人类价值观前”。
- BLOCKER 3（“4个模型快照”）：速览改“5个模型快照”。
- BLOCKER 4（41 译注漏条件）：quote、高亮与译注补全 m 款。
- SHOULD_FIX：43 译注补“我们认为”；“首日快照”改“14日凌晨快照”；L4/L5 行注明核验对象是媒体原文，“青少年安全”改引诉状一手；删 Meta 42 位总检察长对比（另一类案件，不能反驳“针对 AI 公司”）；补“批注译注”节；C 组定级说明见上；REA 星数存档另存 `.json`。`offsite.tsv` 按 `docs/workflow/publish.md` 在发布后由 `barking offsite` 生成。
- NEEDS_VERIFICATION 12（“法院未裁判/尚未应诉”无案卷依据）：改为“这是诉请，不是法院裁判”，删“尚未应诉”。
- NEEDS_VERIFICATION 13（风控）：主帖一涉美国州总检察长，Develata 选题时已知（Claude 选题时提示过政府题材风险）；速览第1条是主帖条目，不是另收的速览。标题、封面不含“政府”二字。

## 视觉复核

第 1 轮：WSL Codex（gpt-6-astra，effort medium，`barking review --visual`，14 张图）。3 BLOCKER 与反向核验的 BLOCKER 2–4 相同，已一并修复；4 SHOULD_FIX（43 补“我们认为”、44 两处与 45 一处高亮未覆盖译注内容）已改用 OCR 词框 `rects` 扩大高亮。封面两张、42 未见问题。

第 2 轮（反向与视觉并行，同模型同 effort）：两份均 PASS、无 BLOCKER；确认第 1 轮修复正确。剩余 SHOULD_FIX 已处理：正文删公开信“10月13日”（日期只有 L4 媒体来源）；`images/README.md` 41 图注补全 m 款条件；`evidence-a.md` 追加勘误，撤回“首次协同与事实不符”的判定；41 底图拼接处加 16 px 灰条、卡片标题注“节选，灰条处省略 a–k 款”。
