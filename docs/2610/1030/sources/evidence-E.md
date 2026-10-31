# 1030 期取证 E 组：速览（研究与安全）

执行方：Haiku 5.5（claude-haiku-5-5）。取证时刻：北京时间 2026-10-31 16:48–16:55。只存档、摘句，不截图（派工单规定）。时区：北京时间（UTC+8）；括号内为原时区。

存档文件均在 `docs/2610/1030/sources/`，文件名前缀 `e-`。`.html` 为原件，`.txt` 为文本，`.headers` 为保留的响应头（已去掉可能暴露抓取地点的头）。

## 速览一句事实（供 Claude 合并，≤40 字，限定词留在句中）

- E1：谷歌原型星在轨回传 Gemma 4 俳句，官方称数据中心级 TPU 在轨首个 AI 响应（45 字内，见下方“与已发过的重叠”）。
- E2：YC Labs 称 50 个 GPT-6.1 Sol 在 Minecraft 跑 3 小时，“从未真正繁荣”（引号内为译文，原句 "never really thrived"；发布时刻以页面 "October 2026" 为准）。
- E3：nesformer 称：NES CPU 编译进 Qwen3.5 结构权重，比真机慢 150–290 倍（仓库自述，权重未经训练）。
- E4：据 404 Media 报道，Proton CEO 取消几乎全公司“AI 复审面谈”（据录音转述，Proton 对取消未回应）。

## 与已发过的重叠

- E1 与 1002 期重叠：`daily-scan.md` 1002 行已写“4颗 TPU、每次约15分钟 Gemma 出自 NPR、谷歌两篇博文未写”。1030 的新事实是 10-30 博文的 “Hello World” 回传（俳句），不要重复 1002 的数字。本次核对的 Google 三篇页面（10-01 发射博文、10-30 回传博文、官方事实页）仍无这些数字，与 1002 结论一致。
- E2、E3、E4：`daily-scan.md` 的“已发过”与“近 3 天速览已收”里没有 YC 实验、nesformer 与 Proton 条目（grep 核对）。

## 取证表

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 存档文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| E1-1 原型星在轨跑通 Gemma 4：地面上行俳句提示，卫星回传诗 | L1（Google Research 官方博文，署 Travis Beals） | https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-satellite-response/ | "During a ground-station pass, we uplinked our first prompt to our onboard Gemma 4 open model, asking it to generate a haiku about being in space. As the satellite passed by Svalbard, Norway, the downlinked poem confirmed success:" | `e-e-suncatcher.html` / `.txt` | 官方页署 "Oct 30, 2026"，未给时刻；原型星 10-01 发射（官方 10-01 博文） | 已找到 |
| E1-2 “数据中心级 TPU 在轨的首个 AI 响应” | L1（同上） | 同上 | "This week, we reached our "Hello World" milestone: the first-ever AI response from a data center class TPU in orbit." | 同上 | 限定词："data center class TPU"、"in orbit"；原文没有比较或对照其他芯片 | 已找到 |
| E1-3 下一步计划 | L1（同上） | 同上 | "Next, we will continue with a series of tests to evaluate how TPUs perform under thermal stress and radiation exposure in orbit. Verifying our TPUs can reliably execute workloads in space paves the way for our next milestone of twin interconnected satellites in 2027." | 同上 | 下一步是热应力与辐射测试；2027 年为“两颗互联卫星”里程碑 | 已找到 |
| E1-4 本文数字：芯片数、功率、时长（1 kW / 20 分钟 / 4 颗 TPU，扫描称出自 TechCrunch 10-01 与 NPR） | — | 本文与 Google 10-01 发射博文、官方事实页均未找到上述数字（逐行 grep 核对） | 本文仅有 "Oct 30, 2026"、"Oct 1"、"2027" 与 "a few weeks"（"we've spent the last few weeks completing our commissioning checklist"）；事实页 https://blog.google/innovation-and-ai/models-and-research/google-research/google-project-suncatcher-facts/ 写 "Future designs of our satellites will each carry dozens of TPU chips while orbiting the Earth in clusters."（未来设计，非原型） | `e-e-suncatcher.txt`、`e-e-suncatcher-launch.txt`、`e-e-suncatcher-facts.txt` | TechCrunch 与 NPR 原文未取（不在派工范围内，未核） | 未找到一手来源（上述三个数字）。官方事实页的 "dozens" 只指未来设计，不能当作原型的 4 颗。与 1002 期的 NPR 转述数字（4 颗、约 15 分钟）相比，扫描又写成 20 分钟，两者都未由 Google 一手确认 |
| E1-5 官方社交账号（@sundarpichai）发布 | L1（Google 公司 CEO 官方账号） | https://api.fxtwitter.com/status/2116214866444943853 （原帖 x.com/sundarpichai/status/2116214866444943853） | "A ‘hello world’ milestone for Project Suncatcher 🚀 … received our first complete response back on earth - a haiku!" | `e-e-x-pichai.json` / `.headers` | 北京 10-31 01:05（UTC 10-30 17:05）；取档时点赞 21,565、转 2,129、浏览 2,376,876 | 已找到 |
| E2-1 50 个 GPT-6.1 Sol 智能体在生存模式 Minecraft 世界跑 3 小时，目标“建立繁荣文明”，约 5,900 条消息 | L1（YC Labs 官方页，署 Olam Labs 合作） | https://labs.ycombinator.com/projects/games/index.html | "50 GPT-6.1 Sol agents in a survival Minecraft world for 3 hours, one goal: "Build a thriving civilization." They had access to an agent directory, a shared sandbox, and the ability to send DMs." / "They sent about 5,900 messages." | `e-e-ycgames.html` / `.txt`（文本 111 KB，JS 渲染内容已在 `.txt` 中） | 页面只写 "October 2026"，没有具体日；限定：一个世界、一次运行（"The colony never really thrived."） | 已找到 |
| E2-2 Bed Wars：9 局、每局 45 分钟，10 个 Claude Opus 5.5 对 10 个 GPT 6.1 Sol，GPT 胜 6 局 | L1（同上） | 同上 | "We ran nine 45-minute games of Bed Wars. The teams were 10 Claude Opus 5.5 agents against 10 GPT 6.1 Sol agents with the instruction: "win the game." GPTs won our series 6–3, usually by breaking Claude's bed in the final minutes, while Claude played defensively and tried to win on kills." | 同上 | 指令为 "win the game"；页面未给每局杀敌数的总表（仅提到三场 Claude 杀敌领先） | 已找到 |
| E2-3 Mindustry：7 局，约 9.5 小时，GPT-6.1 Sol（Codex）对 Claude Opus 5.5（Claude Code），GPT 胜 6–1 | L1（同上） | 同上 | "The GPTs (GPT-6.1 Sol in Codex) won 6-1 against the Claudes (Claude Opus 5.5 in Claude Code) in the 7 games. The series ran for about 9.5 hours, with game 3 being the most intense and closest at 4 hours 16 minutes." | 同上 | 两队运行环境不同（Codex 对 Claude Code），页面未拆分模型与 harness 的影响；每队人数页面未给出明文（见勘误 #5） | 已找到 |
| E2-4 实验设置：两队在同样 API 下、无额外策略提示 | L1（同上） | 同上 | "Both teams have the exact same API to the game, without any special instructions given to them on how to play - neither team is given anything more than the other team." / "When launching it, we set the composition of the teams (which harness + model, and how many), then let it run!" | 同上 | 页面明确“how many”由运行者设定，但本系列的每队人数未在正文中给出 | 部分支持（每队人数：页面明文未给；页面文字隐含 Claude 队至少 37 席，见勘误 #5） |
| E2-5 Minecraft 开源 harness | L1（GitHub 仓库，项目方自有） | https://github.com/yc-software/minecraft-agent-harness | "The harness for this didn’t exist yet, so we built one for the experiment that we’ve open-sourced." / 仓库页：MIT license，1 commit | `e-e-ycharness.html` / `.txt` | 存档时 Stars 71（10-31 16:48 BJ 抓取）；secondary 报 63（theterminal，时刻不明），不取舍，两个数字都记 | 已找到 |
| E2-6 发布时刻 | L1（YC 官方 X 账号 @ycombinator） | https://api.fxtwitter.com/status/2115904914526548472 （原帖 x.com/ycombinator/status/2115904914526548472） | "In collaboration with @olam_labs, we ran a series of experiments letting agent swarms compete in Minecraft and Mindustry." / "We found civil bureaucracy, thousands of telegraphic A2A messages, and OpenAI models retrieving the game's source code to beat Claude." | `e-e-x-ycombinator.json` / `.headers` | 北京 10-30 04:33（UTC 10-29 20:33，原文即 UTC）；取档时点赞 1,921、转 199、浏览 349,469 | 已找到 |
| E3-1 “编译而非训练”：6502 CPU 与卡带 ROM 跑在 Qwen3.5 结构的 checkpoint 中，权重由编译器算出 | L1（作者自有仓库 README，dbreunig） | https://github.com/dbreunig/nesformer | "An NES CPU compiled into the weights of a stock Qwen3.5 model." / "Nobody trained these weights; a compiler computed every one." | `e-e-nesformer.html` / `.txt` | 仓库页未给发布日期；HN Show HN 北京 10-31 00:40（UTC 10-30 16:40） | 已找到 |
| E3-2 “约 1GB Qwen 权重” | L1（同上） | 同上 | "466.6M in bf16 (933 MB), of which 9.3M are nonzero, so the weights zip to 3.3 MB" / "Architecture: Qwen3_5ForCausalLM, with Qwen3.5-4B's DeltaNet head shapes (16 key heads, 32 value heads, dim 128) and MLP width (9,216)" / "Hidden size 768" | 同上 | 933 MB 为 bf16 未压缩体积；约 98% 参数为 0（9.3M / 466.6M）；压缩包 3.3 MB；结构只借用 Qwen3.5-4B 的头形状，隐藏维 768，不是 Qwen3.5-4B 的权重 | 部分支持（体积对，但“Qwen 权重”的性质要带限定） |
| E3-3 慢 150–290 倍（相对真机） | L1（同上） | 同上 | "Overall, nesformer runs 150–290× slower than the real console." / "It ran at about 1,650 tokens/s on one CPU core of the machine it was built on." | 同上 | 速度数字为作者机器、单 CPU 核、numba 引擎；真机需约 700,000 tokens/s（README 所写） | 已找到 |
| E3-4 测试规模“3000+ 条指令” | L1（同上） | 同上 | "The reference CPU (nesformer/cpu.py) matches nestest's golden log for 3,348 instructions, and the py65 emulator on 5.7 million instructions of game code." / "The weights, free-running, generate traces identical to the reference CPU's:" 表中 nestest 为 "identical, ending in the expected HALT" | 同上 | 3,348 条是参考 CPU 对 nestest 金标日志；权重端的逐条比对是 nestest 全程与两款游戏的帧与 trace；README 未给 3000+ 条指令的权重端逐条数 | 已找到（口径需写清：参考 CPU 对 nestest 的 3,348 条） |
| E3-5 支持范围：只支持部分 ROM | L1（同上） | 同上 | "NROM cartridges only (iNES mapper 0)." / "Bigger games carry a bank-switching chip (a mapper, such as MMC1 or MMC3) that swaps which part of the ROM the CPU sees." | 同上 | README 限制写 NROM 与 mapper 0；另 "Official opcodes only"、"No IRQs (APU frame IRQ, DMC)" | 已找到 |
| E3-6 “代码与权重由 Opus-5.5 生成” | L2（作者 dbreunig 的公开 HN 发言，署名） | https://news.ycombinator.com/item?id=50307402 （HN 评论，存档见 `e-hn-50306442.json`） | "First off, Opus 5.5 built working weights and code with only two prompts:" | `e-hn-50306442.json` | 北京 10-31 01:39（UTC 10-30 17:39）；README 未写生成者（README 无 Claude/Opus 字样） | 部分支持（作者 HN 自述，仓库内无此句） |
| E3-7 作者承认不是对所有卡带有效 | L2（同上，HN 评论 50306936） | https://news.ycombinator.com/item?id=50306936 | "Doesn't work for all cartridges (I didn't run Opus that long, but it could keep going). Works with Super Mario." | `e-hn-50306442.json` | 北京 10-31 01:10（UTC 10-30 17:10）；HN 评论原文，已存档 | 已找到（限定：作者自述） |
| E4-1 Proton CEO 取消“AI 复审面谈” | L4（404 Media 报道；原站 404media.co） | https://www.404media.co/protons-ceo-cancels-ai-re-interviews/ | "On Friday, Andy Yen, the CEO of Proton, cancelled a series of nearly company-wide ‘re-interviews’ that would have forced employees to explain how they are using AI" / "“We’ve decided to actually cancel the re-interviews,” Yen said." | `e-e-404-proton-cancel.html` / `.txt`；Verge 转述 `e-e-verge-proton.html` / `.txt`（页面署 "Posted Oct 30, 2026 at 7:55 PM UTC"，北京 10-31 03:55） | 页面署 "Oct 30, 2026 at 12:40 PM"（美东 EDT）= 北京 10-31 00:40；据 404 Media 所得 “around two hour all-hands” 录音与 6 名员工，非 Proton 公告 | 已找到（据 404 Media 报道） |
| E4-2 “还会继续推”与“沟通失败” | L4（同上，据录音转述） | 同上 | "there was a communication failure on my part" / "“Going forward, I'm still going to push,” he added." | 同上 | 同上 | 已找到（据 404 Media 录音转述） |
| E4-3 复审面谈的内容与时间（Proton 内部邮件；“全员”、“consequential”） | L4（404 Media 报道，邮件为 404 所见） | https://www.404media.co/proton-to-re-interview-staff-about-ai-to-see-who-gets-to-keep-their-job/ | "told staff on Monday it would be ‘re-interviewing’ staff across the company about how they use AI in their jobs, with the implication that the interviews may decide whether staffers have “continued employment,”" / "“These re-interviews are consequential,” a leaked email seen by 404 Media reads." | `e-e-404-proton-reinterview.html` / `.txt` | 页面署 "Oct 26, 2026 at 4:16 PM"（EDT）= 北京 10-27 04:16；正文在付费墙处截断（"This post is for paid members only"） | 已找到（正文截断） |
| E4-4 “11 月 4–11 日”面谈时间 | L4（TNW 转述，未经 404 可见页核到） | https://thenextweb.com/news/proton-ai-re-interviews-staff-andy-yen-404-media | "The interviews run from 4 to 11 November." / "Not every employee will be called in." | `e-e-tnw-proton.html` / `.txt`（TNW 转述，第 106 行） | 北京时间不适用（日期为 11 月 4–11 日，原文未写时区） | 部分支持（只见于 L4 二转；404 原稿付费墙后未见） |
| E4-5 “AI 写了 10 月 90% 代码”（CTO Bart Butler 在会议中说） | 据 404 Media（付费墙后）；TNW 与 madrobot.blog 转述 | 未找到一手来源（404 原稿付费墙后；录音由 404 持有） | TNW 转述："AI wrote 90% of Proton's code in October, chief technology officer Bart Butler said in a meeting. 404 Media obtained the audio." | `e-e-tnw-proton.html` / `.txt`（TNW 转述，原文 TNW 页可见） | 404 投资报道 10-29 稿可见部分未出现此数字，正文付费墙后 | 未找到一手来源 |
| E4-6 “This will be the last year that we hire engineers.”（Yen 在安全 Slack 频道的话） | L4（404 Media 原话，正文付费墙后） | 同 E4-1 与 E4-7 的 404 投资稿 | 404 投资稿可见部分的改写："Yen said this is the last year that Proton will hire engineers at all, and instead only hire people to build things with AI." 直接引语在付费墙后；HN 50295112 标题引用了原话 | `e-e-404-proton-reckoning.html` / `.txt` | 404 投资稿北京 10-29 22:21（美东 10:21 AM）；HN 50295112 北京 10-30 04:11（UTC 10-29 20:11） | 部分支持（改写已核，直接引语只见于付费墙后与 HN 标题） |
| E4-7 投资稿：Slack 中“bullshit and loser thinking”与“stop lovingly handcrafting all our code and use AI more” | L4（404 Media 投资稿） | https://www.404media.co/manifestos-fear-and-loser-thinking-inside-protons-ai-reckoning/ | "That’s bullshit and loser thinking. Of course it’s possible to be fast and secure," Yen wrote in the channel at the end of August. / "stop lovingly handcrafting all our code and use AI more." | `e-e-404-proton-reckoning.html` / `.txt` | 页面署 "Oct 29, 2026 at 10:21 AM"（EDT）= 北京 10-29 22:21 | 已找到（据 404 Media） |
| E4-8 Proton 官方回应 | — | 未找到公司公告；404 投资稿（10-29）引发言人回应，取消稿写 "Proton did not respond to a request for comment." | "In our determination to move fast, we have sometimes been too blunt or too slow in explaining ourselves to our people." （发言人，10-29 稿） / "Proton did not respond to a request for comment." （取消稿） | 同 E4-1、E4-7 | 发言人回应早于取消。Proton 在 Reddit 的回应（TNW 转述 TechRadar："not doing layoffs"）针对的是 10-26 复审稿的裁员传言，不涉及取消，且原帖未取到 | 未找到（对取消的官方公告）；发言人对投资稿的回应已找到 |

## 热度（`barking heat`，取档 2026-10-31 16:52 BJ）

| 条目 | HN 条目 | 分数 | 评论（取档时） | 北京时刻 | 备注 |
|---|---|---|---|---|---|
| E2 YC 实验 | 50296486 | 402 | 嵌套评论存档 152 条（HN 对象的 descendants 未存） | 10-30 05:40（UTC 10-29 21:40） | 扫描称 165 评，未核，两个数字都记 |
| E3 nesformer | 50306442 | 247 | 121 | 10-31 00:40（UTC 10-30 16:40） | 扫描称 242 分 120 评，属另一取档时点 |
| E4 Proton 取消稿 | 50307183 | 273 | 227 | 10-31 01:24（UTC 10-30 17:24） | 同一 404 文章的 10-27 稿 HN 50251062：109 分、48 评，10-27 05:13 |
| E4 Yen 引语 | 50295112 | 53 | 40（嵌套存档） | 10-30 04:11（UTC 10-29 20:11） | 扫描称 53 分 40 评，一致 |
| E1 Suncatcher | 50306870 | 8 | 0 | 10-31 01:06 | 另有 HN 50307222（Pichai 帖链接）6 分 1 评，10-31 01:27；HN 50313051 同一链接重复提交 2 分 0 评，10-31 11:54 |
| Reddit（三个查询） | — | 失败 | — | — | `old.reddit.com` 返回登录页（非 JSON），三个查询均失败，记录见 `capture-log-E.md`，未重试 |

## 扫描说法勘误

1. E1 “1 kW / 20 分钟 / 4 颗 TPU”：Google 本文、10-01 发射博文与官方事实页都没有这些数字。事实页只说未来设计 "dozens of TPU chips"。扫描说的 TechCrunch 与 NPR 未核，不能进速览。
2. E1 “数据中心级 TPU 在轨的首个 AI 响应”：与原文一致，但原句的限定词是 "data center class TPU" 与 "in orbit"，速览里要保留。
3. E2 “50 个 GPT-6.1-Sol”：官方写作 "GPT-6.1 Sol"（无连字符）。其余数字（3 小时、约 5,900 条消息、Bed Wars 9 局 6:3、Mindustry 7 局 6:1）与原文一致。
4. E2 HN 评论数：扫描 165 评，存档时嵌套评论 152 条，两数不同时刻，未取舍。
5. E2 每队人数：Herald（10-31 报道，`biz.heraldcorp.com/article/10921351`）称 Mindustry “Each side deployed 50 AI agents”。搜索摘要里有一段 YC 官方 X 线程的后续推文（经镜像站摘录，未能单独存档、未核原帖）说 “We placed 50 GPT-6.1 Sols against 50 Claude Opus 5.5s in PvP mode”。YC 页面明文未给每队人数，但页面文字隐含下界：Game 1 有 “Claude #37”（seat-37），Game 6 有 “the 28 Claudes who answered one focus-fire proposal with +1”，所以 Claude 队至少 37 席。结论：每队 50 人只有媒体与镜像摘录支持，状态“部分支持”，不得写成已核。
6. E2 X 原帖把源码检索与“击败 Claude”连在一起（"...OpenAI models retrieving the game's source code to beat Claude."）。原页只写 "When games got intense and it was ambiguous, GPT agents would fetch the public code..."，没有因果关系。速览只能引原页，不能引 X 的因果句。
7. E3 “代码与权重由 Opus-5.5 生成”：README 没有写，出处是作者 HN 评论（50307402）。速览若要写，须标 “作者在 HN 上称”。
8. E3 “约 1GB Qwen 权重”：933 MB 为 bf16 未压缩体积，约 98% 参数为 0，压缩包 3.3 MB；结构借用 Qwen3.5-4B 的头形状，隐藏维 768。速览写 "约 1GB" 时要带 "未压缩" 或 "bf16" 之类限定，或改写为 "参数 98% 为零"。
9. E3 “慢 150–290 倍”：速度数字是作者机器、单核、numba 引擎的数据，README 原句 "150–290× slower than the real console"。
10. E4 “全员”：取消稿写 "nearly company-wide"（几乎全公司）。10-26 邮件稿写 "across the company"，但扫描说的是取消。取消的范围应写 “几乎全公司”。
11. E4 “11/4–11”：只见于 TNW 转述（"The interviews run from 4 to 11 November."），404 可见页未核到。
12. E4 发布时刻：扫描写 “取消面谈稿 10-30、内部调查稿 10-29”。北京时间分别为 10-31 00:40（美东 10-30 12:40 PM）与 10-29 22:21（美东 10-29 10:21 AM）。日期应按哪一时区写，须在正文定。
13. E4 “CTO 称 10 月 90% 代码由 AI 写（未定义）”：仅见于付费墙后的 404 稿与 TNW、madrobot 转述，未找到一手来源。速览不能写成 “CTO 称”，只能写 “据 404 Media 报道”，并标未核。
14. E4 “This will be the last year that we hire engineers”：404 可见正文只有改写，直接引语在付费墙后（HN 50295112 标题引用）。
15. E4 Proton 官方回应：未找到公司对取消的公告；发言人对投资稿的回应已找到（见 E4-8）。

## 假设与限制

- 404 Media 两篇（10-26 复审邮件稿、10-29 投资稿）正文在付费墙处截断。存档的是可见部分，状态按可见部分记。未绕过付费墙。
- E4-4 与 E4-5 的数字来自二转稿，只写 L4，未存档。
- E2 与 E3 的“可能的吠点”与社区高赞评论按派工单（D/E 组只需勘误与一句事实）未做展开。
- Reddit 热度三个查询失败（`old.reddit.com` 登录页）。未改用其他域名，未重试。
- Yen 在 Reddit 上的回复未存档（404 稿提及）。
- 隐私与留存：`e-e-nesformer.html` 与 `e-e-ycharness.html` 含 GitHub 页面内嵌的 `uploadToken`（匿名页，`currentUser:null`），这是页面原文的一部分。HTML 原件不入库（AGENTS.md 规定），只留 `.txt`、`.headers`、`.json`；Claude 合并时不要提交 `.html`。
- 404 页面的时刻（如 "Oct 30, 2026 at 12:40 PM"）按美东 EDT 解读，与 Exa 元数据 `2026-10-30T16:40:39Z` 一致（EDT+4）。这是推断，不是页面明示的时区。
