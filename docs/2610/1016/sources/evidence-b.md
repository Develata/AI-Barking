# 1016 期 B 组取证：Claude 还原 NIST 椭圆曲线种子

执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时刻一律北京时间（UTC+8），括号保留原时区；本机 EDT = 北京 − 12 小时。存档在 `sources/b-*`，日志见 `capture-log-b.md`。核查时点：北京 2026-10-17 20:48–21:45。

标注：**已验证**＝我亲手对照存档或亲手运行；**推断**＝由已验证事实得出；**未验证**＝未能核实。

## 一、事实清单

### B1 半小时、lrand48、时间戳、1842

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 发布日期 | L1 | https://www.anthropic.com/research/ecdsa-curve-origins | 页面日期 “Oct 16, 2026”；`<time dateTime="2026-10-16T17:02:00.000Z">` | 19 | 17:02Z = 北京 10-17 01:02；归属 “Frontier Red Team” | 已找到 |
| “半小时” | L1 | 同上 | “In under half an hour, Claude discovered the origins of the NIST P-192, P-224, P-256, P-384 and P-521 seeds.” | 13 | 范围是 5 条**素数域**曲线的种子；原文没有写这 30 分钟里的算力、工具、是否多次尝试；二元曲线种子与基点是同文另述，耗时未写（见“How it was found”“Claude found some base points too”） | 已找到（原句）；“半小时解开全部谜题”部分支持 |
| 人工介入与起点 | L1 | 同上 | “Several Anthropic employees had unsuccessfully chased the seeds following the tip about it being a hash of a humorous phrase. That turned out to be wrong. In October 2026, a curious Anthropic employee asked an internal research model if it could figure out how those seeds were generated without pointing at the clue.” | 13 | 此前有多名员工人工尝试失败；本次只描述为“一名员工问了内部研究模型”，**未写**提示词全文、会话轮数、是否有人中途给提示 | 已找到 |
| 模型名 | — | 同上 | 原文只写 “an internal research model” 与 “Claude” | 13 | **原文不给模型名或版本** | 未找到一手来源 |
| 方法 | L1 | 同上 | “Claude wrote a scanner that slid along each published seed one bit at a time … The scan covered 34 combinations of generator and output slices.” “… solved with the Lenstra–Lenstra–Lovász (LLL) basis reduction algorithm.” | — | 文字在存档 `b-anthropic.txt` | 已找到 |
| lrand48 + Unix 时间戳 | L1 | 同上 | “None of them were the results of any hash algorithm at all, but rather the output of the C library’s lrand48 random-number generator, seeded with a Unix timestamp!” | 13 | “hash algorithm”指种子的来源；种子之后仍按 ANSI X9.62 经 SHA-1 推出曲线系数（见 Valsorda：“taking an arbitrary seed, hashing it with SHA-1”） | 已找到 |
| 时间窗 | L1 | 同上 | “every srand48 initialization was a Unix timestamp representing a date between May 1997 and April 1998.” | 13 | 十条素数曲线；原文表为 UTC | 已找到 |
| P-256 是第 1842 个候选 | L1 | 同上 | “For example, P-256 required 1842 iterations to output the standardized seed.” 表：`P-256 871696354 1997-08-16 01:52:34 1842 C49D360886E704936A6678E1139D26B7819F7E90` | 13 | 1997-08-16 01:52:34 UTC = 北京 1997-08-16 09:52:34；candidate 是 20 字节一块 | 已找到 |
| “1/5810” | L1 | 同上 | “four of the ten seeds have a 0 in the top bit for all 20 words. That would happen by chance with probability 1/5810.” 另见 Claude 内部思考引文：“All top bits clear—20 bits, chance 1e-6.” | — | 我验证（`b-prob-check.py`）：1/5810 = 10 个种子中**至少 4 个**各自“5 个字最高位全 0”（每个概率 2^-5）的二项概率；而“指定 4 个种子共 20 个字全 0”是 2^-20 ≈ 1e-6。原文两处口径不同，都在原文里 | 已找到 |

### B2 复现（已验证）

| 说法 | 级 | 来源 | 结果 | 状态 |
|---|---|---|---|---|
| 原文代码可复现 P-256 种子 | L1 + 本机运行 | 原文两个 `<pre>` 代码块（从存档 `b-anthropic.html` 原样提取，不改逻辑）→ `b-repro-seeds.py`、`b-repro-basepoints.py` | 本机 Python 3.15.0，`python -I` 运行，退出码 0。种子脚本输出与原文所附输出逐 token 一致（70/70）；基点脚本 54/54 一致，6 条全部 `matches published Gx: True`。P-256 行：`P-256 1997-08-16 01:52:34 1842 C49D360886E704936A6678E1139D26B7819F7E90`；`unix("1997-08-16 01:52:34")` = 871696354 | 已找到 |
| 与 NIST 文本核对 | L1 | FIPS 186-2 PDF 文本（`b-fips1862.txt`）的 `s=` 行 vs 脚本输出（`b-check-vs-fips.py`，输出 `b-check-vs-fips.out.txt`） | 10 条中 9 条逐字一致：P-192、P-224、P-256、P-384、P-521、B-233、B-283、B-409、B-571。B-163 脚本不产出（与原文“remains unexplained”一致）。我没有再核 SHA-1(seed)→b 这一步 | 已找到 |
| 第三方独立复现 | L2/L1（Valsorda 自述） | https://words.filippo.io/seeds-solved/ | “I reimplemented the algorithm in Go to verify the claim.” Go 源码已存档（`b-filippo-go.txt`），其输出列表与原文一致（含 `srand48(871696354) [1997-08-16 01:52:34 +0000 UTC]: accepted 1842nd seed c49d[...]: P-256`）。**我没有运行这份 Go 代码** | 部分支持（已读其自述与输出，未自行运行） |

### B3 悬赏与背景

| 说法 | 级 | 来源 | 摘句 | 截图 | 口径 | 状态 |
|---|---|---|---|---|---|---|
| 悬赏金额与条件 | L2（Valsorda 本人，署名博客） | https://words.filippo.io/seeds-bounty/（2023-10-05） | “I’m announcing a $8,192 $12,288 (12 Ki$) bounty for cracking these five hashes, tripled to $36,864 if the recipient chooses to donate it to a 501(c)(3) charity of their choice.”“Half the bounty ($6,144) will pay out to the first submission of at least one pre-seed, and the other half will pay out to the first submission of all five pre-seeds.”“The bounty expires if the seeds become publicly known …”“The bounty will pay out to the first person(s) to email the pre-seeds for the five prime-order NIST curves to seeds@filippo.io.” | — | 悬赏要的是哈希的“原像”（pre-seed）；页面自述“I am guaranteeing the full amount of the bounty personally” | 已找到 |
| 已宣布获奖、捐赠 | L2 | https://words.filippo.io/seeds-solved/（16 Oct 2026） | “John Wood, Henry de Valence, Ava Howell, Zack Voase, Lucy Sweet, Amir Omidi, Paul Crowley, Iain McGinniss, Nicholas Carlini, and Steve Weis from Anthropic solved the riddle and on October 15th, 2026, at 01:07:51 UTC, claimed the full bounty I announced three years ago. They chose to donate the prize to ISRG, the non-profit that operates Let’s Encrypt, tripling its value to $36,864 (36 Ki$).” | 20 | 申领时刻 2026-10-15 01:07:51 UTC = **北京 10-15 09:07:51**，早于 Anthropic 发文（北京 10-17 01:02）约 40 小时。Valsorda 页内嵌 Bluesky 帖称 “Also, we raised $36 Ki for ISRG”（OCR 读出，已看到文字行）。ISRG 侧的到账凭证**未找到** | 已找到（宣布）；付款到账 未验证 |
| Anthropic 对奖金的说法 | L1 | Anthropic 原文 | “The $36,864 for finding the seeds will be donated to the Internet Security Research Group.” | 18 | 原文用将来时 “will be donated” | 已找到 |
| Solinas 原话与出处 | L1（Bernstein–Lange 论文附录 C 的邮件原文） | https://cr.yp.to/papers/safecurves-20240809.pdf 附录 C，第 64–65 页 | 2014-09-16：“We built all the seeds via hashing (SHA-1, I think) from the ASCII representation of a humorous message.” 2015-06-17：“The message was along the lines of "Give Bob and Jerry a raise" or "Bob and Jerry rule" or something like that. It was Bob Reiter who actually wrote the code …”“… sitting on a hard drive near the Ark of the Covenant.” | 24 | Anthropic 署 “Jerry Solinas, 2015”，与 2015-06-17 邮件一致；“Jerry died in 2023”（Anthropic）、“passed away in early 2023”（Valsorda）。Steve Weis 的页面把这些邮件列在 “Update August 2024” 下 | 已找到 |
| NIST FIPS 186-2 的 SEED 字段 | L1 | https://csrc.nist.gov/files/pubs/fips/186-2/final/docs/fips186-2.pdf（PDF 第 34 页，印刷页码 31） | `Curve P-256 … s = c49d3608 86e70493 6a6678e1 139d26b7 819f7e90`；§1.4：“Pseudo-random curves are those whose coefficients are generated from the output of a seeded cryptographic hash. If the seed value is given along with the coefficients, it can be verified easily that the coefficients were indeed generated by that method.” | 23 | 该 PDF 是 NIST 存档版，封面写 “FIPS Publication 186-2 (dated January 27, 2000), was superseded on October 5, 2001”。文中**没有**写种子由 NSA 提供（我 grep 过）；“NSA 提供”来自 Anthropic、Weis、Valsorda 等 | 已找到（SEED 字段）；“NSA 提供”的 NIST 侧一手文本 未找到 |

### B4 未解释的部分（原文）

| 说法 | 级 | 来源 | 摘句 | 截图 | 口径 | 状态 |
|---|---|---|---|---|---|---|
| 876 号候选 | L1 | Anthropic 原文 “Open questions” | “For P-256, the published seed is candidate 1842, but candidate 876 of the same run also gives a prime-order curve of the right size and with parameters chosen the same way as on the published curve. P-224’s run passed over three such curves, and the two runs behind the ANSI X9.62 example curves passed over seven more (see figure). We do not know what criteria may have been used, or whether this was an unintentional artifact.” | 15、16 | 原文表（截图 16）里 256-bit 的 876 号 Twist 列为 ✗（2^99.9），1842 号为 ✓（2^120.2）；表头标 Twist 攻击公开年份 “2000–08”。原文没有就此下结论，我也不推断 | 已找到 |
| 时间戳搜索的可能性 | L1 | 同上 | “It is still conceivable that these curves were selected to have a specific property in principle by searching over timestamps to the random number generator. However, you would only need to do that if you were going to disclose the timestamp as some evidence that the seed was innocuously generated. Nothing in the numbers shows how many runs there were.” | 17 | 同段前文：“we do not believe it is plausible that weakened curves were selected” | 已找到 |
| B-163 | L1 | 同上 | “There is also one seed we were unable to recover. The fifth binary seed, B-163, fits nothing Claude has tried and remains unexplained.” | 17 | 与我的复现一致（脚本没有 B-163） | 已找到 |
| 四条曲线基点 | L1 | 同上 | “The base points of P-224, P-256, P-384 and P-521 don’t fit this recipe; we weren’t able to explain how they were selected.”“Finally, we don’t know how the base points for P-224, P-256, P-384, and P-521 were generated. The same goes for the NIST binary curves.” | 17 | 已解释的基点只有 6 条：P-192 与 5 条 ANSI X9.62 示例曲线（FreeLIP） | 已找到 |
| 二元曲线读法 | L1 | 同上 | “The four binary-curve initializations read as February 1984. With the timestamps doubled, they map to April 17, 1998, two weeks after the P-224 run. So, the program may have halved the clock.” | — | 代码注释写 “srand48(time / 2), if that reading is right”，属推测 | 已找到 |
| 无“无后门”保证的限定 | L1 | 同上 | “This is not a cryptographic break, new attack, or weakness in the ECDSA curves. In fact, this makes us more confident the curves were chosen without any backdoors or intentional weaknesses.”“These findings make that much less plausible.” | 14 | 原文是“更有信心”“much less plausible”，不是“证明没有后门” | 已找到 |
| 无需行动 | L1 | 同上 | “There is no action to be taken or things to do differently. Most researchers trusted the curves already.” | 19 | | 已找到 |

## 二、B5 密码学界反应

核查范围：Valsorda 博客与内嵌帖；Anthropic 作者本人发言；HN、Lobsters 帖；Matthew Green、Bernstein、Schneier 的博客首页。HN 的 API 不返回评论分数，下面不写分数，只写评论 ID 与链接（`https://news.ycombinator.com/item?id=<ID>`）；是否被高赞无法核实。

| 谁 | 内容（原话） | 链接 | 级 |
|---|---|---|---|
| Filippo Valsorda（悬赏发起人） | “This week’s findings completely rule out the technique described by Bernstein and Lange. Even if the NSA had been free to pick any timestamp in 1997–98 and any of the first 16 prime-order candidates … they would have had barely 30 bits of freedom. Not enough to intentionally target a hypothetical small class of weak curves, not enough to embed any kind of NOBUS backdoor.”；“this nicely completes the “rehabilitation arc” of NIST elliptic curve cryptography, which I would now personally recommend over Curve25519/Ed25519/X25519.”（后者是他个人意见） | https://words.filippo.io/seeds-solved/ （截图 22） | L2 |
| Valsorda（同页） | “Turns out memory can play tricks on everyone”；“This leaves us with roughly twice as many valid seeds as were published. We explored a number of explanations for this last bit, and found no good fit.”；另有第二条未文档化标准 “If the bit length of the order n … is more than the bit length of the base field p … the seed is rejected.” | 同上（截图 21） | L2 |
| Valsorda（HN 回复） | “even if we assume they tried all timestamps of 1997 and selected arbitrarily among the first 16 viable candidate of any timestamp, that’s just ~30 bits of freedom.” | HN 评论 50125469 | L2 |
| Steve Weis（Anthropic 作者，也是 2023 年研究文章作者） | HN：“13 years of tinfoil hats for srand48(time(NULL)”；X 帖 “Here's a video explaining it”（北京 10-17 02:51:09，220 赞、22285 浏览，视频即原文页内 5:08 的 “Where the NIST seeds came from”） | HN 50122897；X 帖 id 2111168083784245588（`b-fx-sweis.json`） | L2 |
| tptacek（HN） | “This is devastating news... for an entire generation of NIST cryptography conspiracy theories.” | HN 50123044 | L6 |
| Lobsters（Valsorda 帖，score 85、14 评） | 评论分：FiloSottile 24（“P-256 is everywhere, including in hardware …”）；syncsynchalt 14（DES S-box 先例）；orib 13；YogurtGuy 10 | https://lobste.rs/s/ddtqtn（`b-lobsters-ddtqtn.json`） | L6 |
| Matthew Green、Bernstein、Schneier | 截至北京 10-17 21:0x，Green 博客最新文章 2026-10-11（与此无关）；Bernstein 博客最新条目 2026-08-14；Schneier 首页无相关条目。Bluesky/Mastodon 未能检索（Bluesky 公共接口 403），**未找到**他们的公开评论 | blog.cryptographyengineering.com、blog.cr.yp.to、schneier.com（仅首页检查，未存档） | — |

### HN 质疑（仅作线索，L6；进正文前须回一手核实）

| 评论 | 要点 | 链接 |
|---|---|---|
| adastra22 | 认为“makes us more confident”过强：“Imagine that 1 in a thousand, or one in a million keys, selected at random, are vulnerable to some withheld attack. This approach lets you vary the timestamp or number of iterations until a weak key is found, then have plausible deniability afterward.”；后续回复：“the "much" in "much less plausible" is not warranted, I think.” | HN 50123184；50128574 |
| fidotron | “While this confirms the method of production it doesn't say anything about their selection … you cannot see those candidates that were generated and discarded.” | HN 50123213 |
| bayarearefugee | 时间戳横跨近一年，不是“现在”的时间；时间戳意义不明 | HN 50123242 |
| slowin | 普通用户用 Claude 做同样的事会被拦截（“we will get blocked”）；同串多人回复称未被拦（Retr0id：“I worked on this specific task myself, using Claude, and I was not blocked (although I did not succeed!)”） | HN 50123075；50123270 |
| connorboyle / FiloSottile（反驳） | 回应：约 2^30 量级的选择空间，不足以命中占比 1/2^N 的弱曲线（connorboyle 的推理；Valsorda 给出 “~30 bits of freedom”） | HN 50125441；50125469 |

## 三、B6 夸大说法实例

| # | 站点 / 发布方 | URL | 原句（逐字） | 发布时间 | 与原文的差别 |
|---|---|---|---|---|---|
| 1 | CoinDesk | https://www.coindesk.com/tech/2026/10/17/wow-it-s-a-real-discovery-claude-cracks-a-25-year-nsa-number-mystery | 标题 “‘Wow, it’s a real discovery’: Claude cracks a 25-year NSA number mystery”；正文 “An Anthropic AI model has cracked one of cryptography’s longest-running whodunits”；“The tinfoil theory was that the NSA had a backdoor to every system that used it.” | 页面 “Oct 17, 2026, 2:39 a.m. EDT” = 北京 10-17 14:39 | “cracks”用于“来源还原”，原文明说 “not a cryptographic break”；正文同时写了 “questions remain”，标题没有 |
| 2 | Bitcoin.com 中文 | https://news.bitcoin.com/zh/featured/claude-ai-po-jie-nsa-25-nian-mi-ma-zhong-zi-zhi-mi/ | 标题 “Claude AI 破解NSA 25年密码种子之谜”；“Anthropic公司的“克劳德”（Claude）人工智能（AI）在不到30分钟内破解了这一谜题” | 页面 “2026年10月16日 GMT-4 20:30” = 北京 10-17 08:30 | “破解”；“30 分钟”只覆盖 5 条素数曲线的种子，B-163 与四条曲线基点未解（同篇后文有写） |
| 3 | ic.work | https://www.ic.work/article/claude-solves-ecdsa-curve-origins-nist-seeds | 标题 “Anthropic破解NIST曲线种子：三十年NSA后门疑云，竟源于1997年一行粗糙时间戳”；“Anthropic 逆向出的代码逻辑彻底封死了后门假说。” | 页面 “2026年10月17日”；搜索元数据 10-16 19:58Z = 北京 10-17 03:58（未验证） | 原文是 “much less plausible”“more confident”，且写了 “conceivable … searching over timestamps”；**与原文不符之处**：其表中 P-192 为 863951711 / 1997-05-18 10:35:11（原文 863966734 / 14:45:34），P-224、P-384、P-521 的时间戳也都与原文表不同，只有 P-256 一致；“LLL 在三十步内”原文没有；“70 比特”出自 Bernstein–Lange（经 Valsorda 转述），不是 Anthropic |
| 4 | Investing.com（英文） | https://www.investing.com/news/economy-news/anthropic-says-claude-solved-25year-cryptography-mystery-in-30-minutes-4954741 | 标题 “Anthropic says Claude solved 25-year cryptography mystery in 30 minutes” | 搜索元数据 10-16 18:53:20Z = 北京 10-17 02:53（**存档被 Cloudflare 拦截，标题与时间只来自搜索引擎摘要，未验证**） | “solved”；“30 minutes” 范围同上 |
| 5 | Investing.com 香港（Yahoo 财经转载） | https://hk.finance.yahoo.com/news/anthropic%E7%A8%B1claude%E5%9C%A830%E5%88%86%E9%90%98%E5%85%A7%E7%A0%B4%E8%A7%A3%E4%BA%8625%E5%B9%B4%E5%AF%86%E7%A2%BC%E5%AD%B8%E8%AC%8E%E9%A1%8C-195320665.html | 标题 “Anthropic稱Claude在30分鐘內破解了25年密碼學謎題” | 搜索元数据 10-16 19:53:20Z（与英文版元数据相差 1 小时，未验证） | 同上；未存档 |
| 对照（相对忠实） | IT之家 / 凤凰网转载 | https://www.ithome.com/1/014/191.htm ；https://tech.ifeng.com/c/8xHv8awKi2q | 标题 “不到 30 分钟揭开困扰密码学界 26 年的 NSA 参数来源之谜”；正文 “Anthropic 强调，这不是破解 ECDSA，也没有发现新的攻击方式” | IT之家 “10 月 17 日消息”；凤凰网 2026年10月17日 06:54:28 | 标题的“26 年”是该报道自己的算法（2000 年标准至今），原文未写“26 年”（原文内思考文字写 “25-year-old mystery”）；正文写明未解部分 |

没有找到把它直接写成“ECDSA 被破解”“NSA 后门实锤/洗白”的标题（检索：Exa、WebSearch、HN Algolia、Lobsters；Firecrawl 额度已用尽，未能用）。上表 1–5 条的标题只用了“破解/cracks/solved”，没有写成“ECDSA 被破解”或“后门实锤”；最接近的是第 3 条正文的“彻底封死了后门假说”。

## 四、可能的吠点（原文有、线索摘要没提或说反）

1. 原文把种子来源说成 “None of them were the results of any hash algorithm at all”，但曲线系数仍由 SHA-1(种子) 推出（Valsorda：“taking an arbitrary seed, hashing it with SHA-1”）；“不是哈希”指的是种子本身（Anthropic 原文；Valsorda 页）。
2. “半小时”只覆盖 5 条素数域曲线的种子；B-163 种子与四条曲线的基点“remains unexplained”（Anthropic，Open questions）。
3. 模型名与会话条件原文不给：“an internal research model”（Anthropic，Claude found the seeds）。此前已有多名员工人工尝试（同节）。
4. 两处都明说“可想象仍有选择”：“It is still conceivable that these curves were selected to have a specific property in principle by searching over timestamps”；“Nothing in the numbers shows how many runs there were.”（Anthropic，Open questions）。
5. 跳过的候选数量：Anthropic 称 “the number of skipped candidates is small”；Valsorda 称 “roughly twice as many valid seeds as were published … found no good fit”。同一批数据，口径措辞不同（两者均为一手）。
6. Valsorda 另列一条 Anthropic 文中没有的未文档化条件：n 的比特长度大于 p 则拒绝（seeds-solved）。
7. 申领悬赏早于发文：Valsorda 页写申领时刻为 2026-10-15 01:07:51 UTC，Anthropic 发文 10-16 17:02Z。
8. 作者中的 Amir Omidi 同时出现在 Valsorda 的悬赏出资人名单里，Steve Weis 是 2023 年研究文章作者（两页的署名名单；奖金捐给 ISRG）。
9. 二元曲线的“时间减半”是猜测：“So, the program may have halved the clock.”（代码注释 “if that reading is right”）。
10. 种子“可由时间戳复现”不等于“当年选择过程已知”：基点、跳过的候选、B-163 都未解；“更有信心”是 Anthropic 的判断（Valsorda 的“个人推荐 NIST 曲线胜过 Curve25519”是他个人意见，不是 Anthropic 的）。
11. Bernstein–Lange 的“70 比特”论点（转述自 Valsorda，原论文我没读到）被 Valsorda 称作 “completely rule out”；Bernstein 本人是否回应：未找到。
12. 1/5810 的口径见 B1。
13. 对 Anthropic 页面里嵌入的 5 分 08 秒视频，我未观看；只记标题 “Where the NIST seeds came from”。

## 五、扫描说法勘误

- 已知情况中的原文摘句，我逐条核对存档，与原文一致：datePublished 17:02Z；Solinas 2015 引文；“Jerry died in 2023”；“In under half an hour …”；1842；“not a cryptographic break”；1997-05 至 1998-04；二元曲线 1984-02 与“halved the clock”；FreeLIP 六条基点；“candidate 876”；“conceivable … searching over timestamps”；B-163；致谢 Filippo Valsorda。
- 补充：原文“时间戳表”（P-192、P-224、P-256、P-384、P-521 的 t 与 n）与基点表是页内图，不在文字存档里，已截图（13）。
- 模型名：已知情况让核；原文**不给**（“an internal research model”）。
- 热度：已知 HN “903 分/335 评”；我 10-17 20:56:52 取得 912 分/338 评（`b-heat.json`，帖子 50122800，提交 2026-10-16 18:55:59Z = 北京 10-17 02:55:59）。Valsorda 帖 HN 50122920：52 分/8 评；Lobsters 85 分/14 评。Reddit 检索失败（返回登录页）；Techmeme 页（`b-techmeme.txt`）里列有 r/crypto 帖，未能取数。
- “Jerry 的口令”写法差异：Anthropic 与 Solinas 邮件为 “Give Bob and Jerry a raise”；Valsorda 悬赏页另引 “Jerry deserves a raise.”（Steve Weis 页第二手转述）。
- 函数名：Anthropic 写 `lrand48`（代码 `lrand48() >> 23`），Valsorda 写 `drand48`（其 Go 源码注释：“The NIST seeds are consecutive 20-byte blocks of (int)(drand48() * 256)”）。两处原话都保留，不取舍；推断：两者都取 48 位状态的最高 8 位（x>>40）。
- Valsorda 的 “passed away in early 2023” 与 Anthropic 的 “died in 2023” 一致。

## 六、复核提示

- 图 16 的表是 Anthropic 页内图的 HTML 渲染，截图保留了图例、表头、脚注。
- 图 24 显示 Solinas 的公开机构邮箱（Bernstein–Lange 论文附录原样）。若不想在卡片或备用图里露出，可从该图的 “Date:” 行开始重裁（重裁后失去 From 行的归属）。

## 七、未完成项

- Koblitz–Menezes《A Riddle Wrapped in an Enigma》全文（eprint PDF 403）：Valsorda 引用的 §3.1 与 Bernstein–Lange “70 bits” 的原论文出处未核。
- Reddit r/crypto 帖（登录页）、Bluesky/Mastodon 帖（接口拒绝）：只看到 Valsorda 页内嵌的 Bluesky 卡片文字，未取数。
- Matthew Green、Bernstein、Schneier：只查了博客首页，未查其社交账号。
- Valsorda 的 Go 实现未运行；Anthropic 页内 5 分 08 秒视频未看；ISRG 侧到账凭证未找到。
- Investing.com 三条报道存档被拦，标题与时间仅来自搜索摘要。
