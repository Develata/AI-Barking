# 1017 事实核验

逐条对照 `sources/` 存档。结果：✅ 已核实（L1–L3 原文支持）；⚠️ 有署名的报道或转述，已按分级署名；❌ 与原文不符（不得进正文）。

## 主帖一：OpenCode 漏了谁的 key？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenCode 北京时间18日凌晨称，前一晚遭遇与“大规模蒸馏攻击”有关的安全事件 | L1 | ✅ | https://x.com/opencode/status/2111519637049319818（`a-fx-2111519637049319818.raw.json`；初版 `a-fx-2111516673953935833.raw.json`；`../images/01`） | 初版北京 10-18 01:56:19（UTC 10-17 17:56:19），编辑版 02:08:06 补上后两句。“We had a security breach last night related to a large scale distillation attack”。“last night”未给时区；用户 10-17 23:18 北京已报告收到通知（`a-fx-2111476982580211851.raw.json`），故写“称前一晚” |
| 波及1.4%的 Console 用户（均关联外部服务商）（省流：OpenCode 称云端 Console 1.4%用户受影响） | L1 | ✅ | 同上 | “impacted 1.4% of OpenCode Console users … it was a subset of users who had external providers linked”；受影响者是关联者的子集。“云端”据 docs：Console 在其网关存凭据（见下）。第一轮反向核验 BLOCKER 1 后，省流不再写“密钥可能已暴露”（该说法只见于 L6 邮件转贴） |
| 据用户转贴的邮件，工作区存的 OpenAI、Anthropic 等密钥“可能已暴露” | L6 | ⚠️ | HN 评论 https://news.ycombinator.com/item?id=50133412（`a-hn-50133401.json`；`../images/03`）；Reddit r/opencode 邮件截图帖（`evidence-a.md` A4b） | “Third party provider keys (eg OpenAI, Anthropic) saved in your workspace may have been exposed.”两个独立转贴逐词一致；官方未公开邮件原文，正文署“据用户转贴” |
| Anthropic 员工称已撤销“认为受影响”的 Claude 密钥 | L2 | ✅ | https://x.com/lydiahallie/status/2111563956666110287（`a-fx-2111563956666110287.raw.json`；`../images/02`） | Lydia Hallie，北京 10-18 05:04:12。“have revoked Claude API keys we believe were affected”。身份据其账号简介“Claude Code @AnthropicAI @ClaudeDevs”；Anthropic 官方渠道未见说明（A6c）；正文为省字未写姓名 |
| 出事的是可选的云端 Console，它替用户存密钥 | L1 | ✅ | https://opencode.ai/v2/docs/console（`a-opencode-v2-console.txt` 85 行）；https://opencode.ai/v2/docs/console/providers/（`a-opencode-v2-console-providers.txt` 86 行；事发前 Wayback 10-07 同句 `a-wb-console-providers-1007.txt`） | “The OpenCode Console is an optional service”；“Console stores the credential and serves each provider through its gateway”；官方帖写“OpenCode Console users”。文档未写加密方式 |
| 其员工称只用本机版的密钥不经过公司 | L2 | ✅ | https://x.com/thdxr/status/2111572618340188393（`a-fx-2111572618340188393.raw.json`；`../images/05`） | “其”= OpenCode。dax（简介“building @opencode at @anomalyco”）北京 10-18 05:38:37 回复 Hallie：“note this is the OpenCode Console, not OpenCode itself, those keys never come to us”。文档只写本地 CLI 的 key 存在 `~/.local/share/opencode/auth.json`（`a-wb-docs-providers-1017.txt`），没写“不上传”，故署员工 |
| 已查通报与文档没给 Console 用户总数 | L1 | ✅ | 官方帖两版、邮件转贴、docs、首页、状态页（`evidence-a.md` A3） | 比例的分母类别是“OpenCode Console users”，未给的是人数或受影响账号数。否定只限已查材料：官方帖两版、邮件转贴、opencode.ai 首页与 docs、状态页、GitHub advisory、Trust Center；官方账号时间线未能枚举（A1d），故第二轮 SHOULD_FIX 7 后由“官方没公布”改为“已查通报与文档没给”。首页“used and trusted by over 16M developers every month”是整体口径（`a-opencode-home.txt`），不是 Console 用户数 |
| 也没给“蒸馏攻击”的调用量、模型或时段，邮件未提（省流：通报未给“蒸馏攻击”的依据） | L1 | ✅ | 官方帖两版；邮件转贴（A4、A5a） | 主语承上句“已查通报与文档”。官方帖只有“related to a large scale distillation attack”，并称“we cannot share more details at this point”；邮件转贴全文无“distillation”。读取截至北京 10-18 约 21:40。HN ipsum2 称没有公开证据（L6，`../images/04`） |

## 主帖二：AI 让美国 GDP 涨五成？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 联创 Jack Clark 17日在 IMF 与世行年会称，在“类奇点”条件下，公司一个模型预测美国 GDP 可达44万亿美元 | L1 | ✅ | IMF 官方频道录像 https://www.youtube.com/watch?v=Rqt9kWoTJbc（Per Jacobsson Foundation Lecture，`b-b14-yt-captions-excerpt.txt` 约 00:23:50）；路透社转载 `b-b24-yahoo-reuters.txt`、`b-b25-thestar-reuters.txt` | 讲座曼谷 10-17 15:01 开始（北京 16:01），44 万亿一段约曼谷 15:25（推算）。字幕：“if you assume, as people at the labs do, that AI models will be able to autonomously complete a broad range of tasks … then under these singularity-esque conditions, one model predicts a vast economic transformation with US GDP potentially reaching 44 trillion”。自动字幕年份写“by 2040”，本机 Whisper 听成“by 2030”，路透社写 2030——待人工听；正文不引 Clark 口述的年份。路透社图注 10/16 是另一场研讨会 |
| IT 之家标题写“到2030年 AI 有望推动美国 GDP 增长近50%” | L4 | ⚠️ | IT之家（`b-b15-ithome.txt` 68 行；`../images/16`） | 北京 10-18 12:27:27。网页标题“Anthropic 联合创始人：到 2030 年 AI 有望推动美国 GDP 增长近 50%”（第二轮 SHOULD_FIX 9 后更正转录，原转录误加了姓名）。本行只核“IT 之家网页用了这个标题”这一文本事实，不为“增长近50%”背书；作为报道来源，evidence-b B8 按转载记 L5，两者分别记录 |
| Anthropic 模型里，2030年44.4万亿美元是三个情景中最高的“极端”情景，另两个为34.1万亿、36.3万亿（按2025年价格）（省流：Anthropic 模型里2030年美国 GDP 44.4万亿美元（按2025年价格）是最极端的情景） | L1 | ✅ | https://www.anthropic.com/institute/econ-scenarios（`b-b1-scenarios.txt` 160–176 行；`../images/13`） | 图题“US GDP in 2030, by scenario”；“Modest scenario +1.6% $34.1T GDP / Substantial scenario +8.3% $36.3T GDP / Extreme scenario +32.4% $44.4T GDP”，“*GDP is calculated at 2025 price levels”。第二轮 BLOCKER 1 后省流补“2030年”“按2025年价格”。Clark 口述的44万亿与此极端情景是否同一数字，Clark 未点明，属推断；正文分开陈述口述与模型情景，不写“就是”（`evidence-b.md` 第 7 行“已验证”一句已加更正注） |
| 论文写明情景“不是预测” | L1 | ✅ | 工作论文 https://www-cdn.anthropic.com/files/4zrzovbb/website/cf58f84d46a4a76bf5a5b039ac695fba6b80041c.pdf p.3（`b-b2-paper-pdf.txt`；`../images/14`） | “The scenarios are not predictions, and we attach no probabilities to them”。Korinek 等，The Anthropic Institute Working Paper No. 2026-02 |
| 按论文，无 AI 时 GDP 2030年也比2024年高约13% | L1 | ✅ | 同论文 Table 3（`b-b2-paper-pdf.txt` 1583 行；`../images/17`） | “GDP, index with 2024 = 100”：无 AI 112.7、modest 114.5、substantial 122.1、extreme 149.3。112.7 即“约13%”。论文另写无 AI 路径年增 2%（“GDP without AI therefore grows at g + n = 2 percent a year”） |
| “近50%”是拿2030年比现在，含没有 AI 也有的增长 | L1+推断 | ⚠️ | IT 之家标题（`b-b15-ithome.txt` 68 行）；路透社转载（`b-b25-thestar-reuters.txt` 399 行）；论文表 3 | 路透社写“compares to the current level of just over $30 trillion”，即 44 万亿对现在约 30 万亿（约 +47%）；任何拿 2030 年对现在的比较都包含基线增长，论文无 AI 路径 2030 年指数 112.7（2024=100）。媒体是否按这套情景与基期算出“近50%”无来源说明，属推断，故标 ⚠️（第三轮 SHOULD_FIX 1 后由“有一截与 AI 无关”改写） |
| 极端情景比无 AI 路径高32.4% | L1 | ✅ | 情景页；论文 p.3、Table 3 | “GDP, pct. above the no-AI path … 32.4”；p.3 “GDP 32 percent higher by 2030”；149.3/112.7 约 1.325 |

## 主帖三：Meta 删了“训练”？

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Meta 新商业条款10月30日生效 | L1 | ✅ | https://www.facebook.com/legal/commercial_terms_preview?locale=en_US（`c-meta-ct-en_US-dom.txt`） | “These updates will go into effect on October 30, 2026.”；GB 版“30 October 2026” |
| 授权其爬取用户商品或服务关联网站 | L1 | ✅ | 同上；`../images/25`、`../images/26` | `c-meta-ct-en_US-dom.txt` 11 行：条款适用于“for a business or commercial purpose”，“regardless of the entity type”；19 行：爬取“the domains, web pages, or URLs (and any content available through them) that are associated with your products or services, or that are made available through your or users' use of Meta Company Products and services”。第三轮 SHOULD_FIX 3 后由“企业账户关联网站”改写（“business accounts”是 Nieman Lab 的概括）。现行商业条款（2024-03-06 生效）无此段（C4c） |
| 据 The Verge 报道，含“训练”一句已删 | L4 | ⚠️ | https://www.theverge.com/tech/1013409/meta-removes-language-that-may-have-allowed-it-to-scrape-news-sites（`c-verge.txt`） | 北京 10-17 23:04（15:04 UTC）。副题“Language that experts say allowed the scraping of news sites to train its AI has been removed from Meta’s terms.”；正文“the following language no longer appears on the commercial terms page” |
| Meta 发言人称整合后的爬取条款只适用于广告和商业产品，“已在条款中澄清” | L4 | ⚠️ | 同上 | 发言人 Daniel Roberts 对 The Verge：“the crawling terms we consolidated only applied to our ads and business products, and we’ve clarified that in the terms themselves.”主语是“整合后的爬取条款”，不是 Meta 全部爬取（第一轮 SHOULD_FIX 4）；Meta 爬虫页写 Meta-ExternalAgent 用于“training foundation AI models”（`c-meta-crawlers-en.txt`） |
| 没删干净：北京时间18日晚看预览页，英国英语、法语、简体中文版仍写着“研究和训练”（省流：Meta 新商业条款预览美国英语版删了“训练”，英国英语、法语、简体中文版仍在） | L1 | ✅ | `?locale=en_US`（`c-meta-ct-en_US-dom.txt`；`../images/25`）；`?locale=en_GB`（`c-meta-ct-en_GB-dom.txt`；`../images/26`）；`?locale=fr_FR`（`c-meta-ct-prev-fr_FR.txt`）；`?locale=zh_CN`（`c-meta-ct-prev-zh_CN.txt`） | 页面为 `commercial_terms_preview`（10月30日生效的新版预览），北京 10-18 19:41–19:56 读取，20:33 复读不变；匿名浏览器。en_US 全页 grep “train” 0 处；en_GB：“Meta may use any content from such domains, web pages or URLs to provide, develop, improve, research, train and maintain the integrity of”；fr_FR：“rechercher, entraîner”；zh_CN：“以便提供、开发、改进、研究和训练”。只查了这四个语言版，故不写“只删了美国版”。第二轮 SHOULD_FIX 2 后省流写明“新商业条款预览”；现行自助广告条款仍含“research and training”，见下行。旧版 Wayback 10-06 美国版含 train（`../images/27`） |
| 现行自助广告条款（2025年8月生效）已写明广告主下单即授权 Meta 爬取其网站用于“研究和训练”等 | L1 | ✅ | https://www.facebook.com/legal/self_service_ads_terms?locale=en_US（`c-meta-ssat-current-en_US.txt`；`../images/28`） | “6. By placing an Order, you grant us the right to access, index, cache, analyze or crawl the content available through your domains, webpages or URLs, or any portion thereof, for developing and improving advertising and other business-related products and related uses, research and training, and maintaining the integrity of the Meta Company Products.”“Effective Date: August 22, 2025” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| Waymo 称目标2028年在多伦多迎来首批乘客，眼下先人工驾驶测绘 | L1 | ✅ | https://waymo.com/blog/2026/10/waymo-in-toronto/（复用 `../../1016/sources/d-waymo-toronto.txt`） | 页面 2026-10-16，官方未给时刻。“with the goal of welcoming our first riders in 2028”；“as we begin driving manually … starting with high-definition mapping of local street networks and testing with trained autonomous specialists behind the wheel”。HN 50118981 326/443。速览标签“Waymo” |
| 苹果审核指南写明：含用户内容的应用须主动防范、移除非自愿私密影像，含 AI 生成 | L1 | ✅ | https://developer.apple.com/news/?id=vvzx6cul（`d-d3-apple-news-nonconsent.txt`）；总条目 https://developer.apple.com/news/?id=92th7e5i（`d-d3-apple-guidelines-news.txt`） | 两条均 2026-10-16，官方未给时刻。“1.2: Content moderation requirements now explicitly cover nonconsensual intimate imagery — including imagery generated using AI tools — as content that apps must proactively prevent and remove from their platforms.”总条目写 1.2 为“Clarified”，故不写“新规” |
| 亚马逊称 Bee Frontier 把对话整理成摘要，美国售74.99美元 | L1 | ✅ | https://www.aboutamazon.com/news/devices/amazon-personal-ai-bee-frontier-bee-eclipse（`d-d2-amazon-bee.txt` 83–85 行） | 页面未给日期；HN 首帖北京 10-15 20:12。“Bee Frontier … is available now for $74.99 … at Amazon.com.”（第二轮 SHOULD_FIX 10 后补“美国”）“With the purchase of Bee, all users receive conversation summaries, recaps, and 14-day memory at no additional cost.”待办（suggested to-dos）属 Bee Pro（月费 11.99 美元），第一轮 BLOCKER 2 后删去 |
| 腾讯开源 Hy-Memory-2.0，含记忆模型、框架与 MemHub 应用三层 | L1 | ✅ | https://memory.hunyuan.tencent.com/（`d-d4-hunyuan.txt` 1–90 行）；https://github.com/Tencent-Hunyuan/Hy-Memory（`d-d4-hy-memory-github.txt`，许可证 `d-d4-license-raw.txt`） | 页面标题“Hy-Memory 2.0 — 模型、框架、应用三层全开源”，“Tencent Hunyuan · 开源发布 · 2026.10.16”，未给时刻；三层为 Hy-Memory-30B-A3B（模型）、Hy-Memory（框架，“通过 SDK 接入”）、MemHub（应用）。Apache-2.0。第三轮 SHOULD_FIX 2：自测分数（55.49% 对 Opus 5 的 54.62%）的子样本与只换记忆写入模型等条件速览容不下，删去分数比较 |
| Lean 作者称，一个新内核由 Claude 在研究者密切监督下实现 | L2 | ✅ | https://leodemoura.github.io/blog/2026-10-15-lean-now-ships-with-verified-kernels/（`d-d1-lean-blog.txt` 12–13 行） | 2026-10-15，未给时刻；作者 Leonardo de Moura（Lean 作者）。“Con-leche is the first kernel for Lean written in Lean and proven consistent. It was implemented and proven consistent by Claude (Fable and Opus), under heavy supervision by Joachim Breitner at the Lean FRO.”一致性证明以模型所用集合论的一致性为前提，速览容不下，故删去“并证明一致性”（第一轮 BLOCKER 3）。HN 50114758 152/50 |


## 批注译注

| 文中事实（译注） | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 昨晚我们发生了一起与大规模蒸馏攻击有关的安全事件，波及1.4%的 OpenCode Console 用户 | L1 | ✅ | 官方帖（`a-fx-2111519637049319818.raw.json`；`../images/41-opencode-post-raw.png`） | 直译“We had a security breach last night related to a large scale distillation attack that impacted 1.4% of OpenCode Console users”；存档 10-18（`capture-log-a.md`） |
| 受影响的是关联了外部服务商的部分用户（原帖“Show more”后为 linked） | L1 | ✅ | 同上 JSON（`evidence-a.md` A1a） | 截图卡片被“Show more”截断在“external providers”，JSON 全文为“it was a subset of users who had external providers linked” |
| 已撤销我们认为受影响的 Claude API 密钥 | L2 | ✅ | Hallie 帖（`a-fx-2111563956666110287.raw.json`；`../images/42-hallie-revoked-raw.png`） | 直译“have revoked Claude API keys we believe were affected.” |
| 注意，出事的是 OpenCode Console，不是 OpenCode 本身；那些密钥从不经过我们 | L2 | ✅ | dax 帖（`a-fx-2111572618340188393.raw.json`；`../images/43-dax-console-raw.png`） | 直译“note this is the OpenCode Console, not OpenCode itself, those keys never come to us” |
| 温和、显著情景：比无 AI 路径高1.6%、8.3%，分别为34.1万亿、36.3万亿美元 | L1 | ✅ | 情景页（`b-b1-scenarios.txt` 160–176 行；`../images/44-gdp-scenarios-raw.png`） | “Modest scenario +1.6% $34.1T GDP”“Substantial scenario +8.3% $36.3T GDP”；“比无 AI 路径”据论文 p.3 “higher than without AI”、表 3 “pct. above the no-AI path” |
| 极端情景：比无 AI 路径高32.4%，44.4万亿美元 | L1 | ✅ | 同上 | “Extreme scenario +32.4% $44.4T GDP” |
| GDP 按2025年价格计算 | L1 | ✅ | 同上 | “*GDP is calculated at 2025 price levels” |
| 这些情景不是预测，我们也不给它们赋概率 | L1 | ✅ | 论文 p.3（`b-b2-paper-pdf.txt` 102 行；`../images/45-not-predictions-raw.png`） | 直译“The scenarios are not predictions, and we attach no probabilities to them” |
| GDP 比无 AI 路径高：极端情景32.4%；GDP 指数（2024=100）：无 AI 112.7，极端情景149.3 | L1 | ✅ | 论文表 3（`b-b2-paper-pdf.txt` 1583 行；`../images/46-table3-raw.png`） | “GDP, pct. above the no-AI path … 32.4”；“GDP, index with 2024 = 100 … 112.7 … 149.3”；第二轮视觉复核后两行合为一条批注 |
| Meta 可使用来自这些网域、网页或网址的任何内容，用于提供、开发、改进、研究、训练根据本商业条款及其他适用补充条款提供的 Meta 旗下产品、服务和功能，并维护其完整性 | L1 | ✅ | en_GB 预览（`c-meta-ct-en_GB-dom.txt` 19 行；`../images/47-meta-gb-train-raw.png`） | 直译“Meta may use any content from such domains, web pages or URLs to provide, develop, improve, research, train and maintain the integrity of, the Meta Company Products, services and features provided under these Commercial Terms and any other applicable supplemental terms.”；第二轮 SHOULD_FIX 4 后补全范围限定；中文版同句见 `c-meta-ct-prev-zh_CN.txt` 19 行 |
| 以便提供、开发、改进、研究和维护广告与商业产品 | L1 | ✅ | en_US 预览（`c-meta-ct-en_US-dom.txt` 19 行；`../images/48-meta-us-crawl-raw.png`） | 直译“in order to provide, develop, improve, research, and maintain advertising and business products”；第二轮 SHOULD_FIX 5 后删去括号里的比对判断 |
| 下单即授权我们访问、索引、缓存、分析或爬取您网域、网页或网址上的内容 | L1 | ✅ | 现行自助广告条款（`c-meta-ssat-current-en_US.txt` 45 行；`../images/49-meta-ssat-raw.png`） | 直译“By placing an Order, you grant us the right to access, index, cache, analyze or crawl the content available through your domains, webpages or URLs” |
| 用途包括研究和训练 | L1 | ✅ | 同上 | “… for developing and improving advertising and other business-related products and related uses, research and training, and maintaining the integrity of the Meta Company Products.” |

## 反向核验

### 第一轮（codex gpt-6-astra medium，WSL，`barking review --reverse`）

结论：未通过，3 BLOCKER、6 SHOULD_FIX、2 NEEDS_VERIFICATION。Claude 逐条对存档核实，全部成立，处理如下：

1. BLOCKER 省流把 L6 邮件转贴（“密钥可能已暴露”）写成 L1 事实：省流与省流卡改为“OpenCode 称云端 Console 1.4%用户受影响”，邮件内容只留在事实段并署“据用户转贴”。
2. BLOCKER Bee 待办需 Bee Pro（月费 11.99 美元）：速览删“和待办”。
3. BLOCKER Lean 删了人类监督、一致性证明有前提：改为“Lean 作者称，一个新内核由 Claude 在研究者密切监督下实现”，删“并证明一致性”。
4. SHOULD_FIX 发言人主语是“整合后的爬取条款”：正文补上。
5. SHOULD_FIX “没给分母”“没证据”超出范围：改为“官方没公布 Console 用户总数”“截至18日晚也没披露……”；省流卡去掉该判断。
6. SHOULD_FIX Hy-Memory 均分口径：速览改“记忆评测自测”，清单补四套系统×六项。
7. SHOULD_FIX RTX Spark 价格缺地区、配置条件：速览容不下限定词，整条删去。
8. SHOULD_FIX 情景金额缺不变价：正文补“（按2025年价格）”。
9. SHOULD_FIX IT 之家行定级：清单注明只核标题文字。
10. NEEDS_VERIFICATION Clark 的44万亿与极端情景对应关系：正文与省流改为分开陈述（“Anthropic 模型里，2030年44.4万亿美元是……”），不写“就是”；清单标推断。
11. NEEDS_VERIFICATION 配图与来源交付未齐：已补 `images/README.md`、`sources/README.md`、批注截图与封面。

### 第二轮（codex gpt-6-astra medium，WSL，`barking review --reverse`）

结论：未通过，1 BLOCKER、11 SHOULD_FIX、1 NEEDS_VERIFICATION。Claude 逐条核实，全部成立，处理如下：

1. BLOCKER 省流 GDP 缺时间与价格口径：省流与省流卡改为“Anthropic 模型里2030年美国 GDP 44.4万亿美元（按2025年价格）是最极端的情景”。
2. Meta 省流泛称“条款”：改为“Meta 新商业条款预览美国英语版……英国英语、法语、简体中文版仍在”。
3. Hy-Memory 误记“无单位”：清单更正为单位 %；速览改“记忆评测自测均分55.49超 Opus-5”。
4. Meta 英国英语版译注截掉范围限定：引用与译注补全到“supplemental terms”。
5. 美国英语版译注里的比对判断：删去括号。
6. 译注未进“文中事实”列：新增“批注译注”一节，逐条列出。
7. OpenCode 否定判断超出已查范围：正文改为“已查通报与文档没给……也没给……”，省流改“通报未给‘蒸馏攻击’的依据”。
8. `evidence-b.md` 第 7 行旧“已验证”：加更正注，保留原记录。
9. IT 之家标题转录误加姓名：已更正，并分记标题文本与报道层级。
10. Bee 售价范围：速览补“美国”。
11. 42 号图页脚被挤掉：底图裁去点赞与回复栏后重渲。
12. NEEDS_VERIFICATION Clark 口述年份：仍待人工听辨，正文不引口述年份。
13. `offsite.tsv` 缺失：按 `docs/workflow/publish.md` 在发布后 `barking offsite --upload` 时生成并上传，当前尚未上传。

### 视觉复核（codex gpt-6-astra medium，WSL，`barking review --visual`）

结论：PASS，无 BLOCKER。处理：省流卡 GDP、Meta 两条同上补限定；48 号图删括号；45 号图底图改为只裁论文第 3 页第二段，放大关键句；46 号图两行高亮合为一条批注，编号不再错位；44 号图译注①补译 1.6%、8.3% 及“比无 AI 路径高”；省流卡第一条换行随文字改动重排。

### 第三轮（codex gpt-6-astra medium，WSL，`barking review --reverse`）

结论：PASS，无 BLOCKER；3 SHOULD_FIX、1 NEEDS_VERIFICATION、1 NICE_TO_HAVE。Claude 核实后处理：

1. GDP 吠点②“有一截与 AI 无关”把推断写成确定：改为“‘近50%’是拿2030年比现在，含没有 AI 也会有的增长”，清单标 ⚠️、L1+推断。
2. 腾讯速览缺子样本条件：删去分数比较，改为“腾讯开源 Hy-Memory-2.0，含记忆模型、框架与 MemHub 应用三层”。
3. Meta 适用范围写成“企业账户”：改为“用户商品或服务关联的网站”。
4. NEEDS_VERIFICATION Clark 口述年份：仍待 Develata 人工听辨（录像约 23:32–24:03），正文不引。
5. NICE_TO_HAVE 配图说明把 42、45 号底图记为副本：已更正为裁切图。

