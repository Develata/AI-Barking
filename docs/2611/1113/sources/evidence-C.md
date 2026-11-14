# 1113 期取证 C 组：速览（C1–C8）

执行方：Haiku 5.5（claude-haiku-5-5）。取证时间：2026-11-14 21:49–21:55（北京时间）。规则见派工单 `.handoff/2026-11-13-1113-evidence.md`。C 组不截图。付费墙未绕过，未登录，未用镜像或转载站。

时区约定：官方页只给日期的，写“官方未给时刻”。HN 热度时间为 `barking heat` 按北京时间显示。Fortune 原文 “12:05 PM ET” 换算为北京时间。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

---

## 一、逐条

### C1 OpenAI 推出 ChatGPT Wallet

- 一手来源：https://openai.com/index/introducing-chatgpt-wallet/（已存档 `c-c1-wallet.html` / `.txt` / `.headers`）
- 发布时刻：官方页日期 November 13, 2026（美东相关日期）；官方未给时刻。正文写 “begins rolling out today”。
- 级别：L1
- 原文摘句（逐字）：
  - “ChatGPT Wallet will initially be available through dots for ChatGPT Pro subscribers in the US.”
  - “It will initially be available to ChatGPT Pro subscribers aged 18 and older in the US with eligible US-issued cards.”
  - “Built in partnership with American Express, Mastercard, and Visa”
  - “approves it with a security check such as Face ID or a fingerprint scan.”
  - “without ChatGPT Wallet sharing their actual card details with the business or agent.”
- 能改变含义的限定词：initially；ChatGPT Pro；US；18 岁以上；eligible US-issued cards。
- 扫描说法核对：“首发仅美国、18 岁以上、Pro 订阅者的 dots 里可用，需美国发行的合格卡”——与官方一致（已找到）。“与 AmEx、Mastercard、Visa 合作”——一致。“每笔用 Face ID/指纹批准”——一致。“商家与智能体拿不到真实卡号”——一致（原文为 payment token 替代）。
- “上线初期不抽成”：官方文中没有费用或抽成表述（逐字搜索 fee 无命中）。**未找到一手来源**，该表述只能来自媒体采访，不能写成官方说法。
- 热度（`barking heat`）：HN 50484640，13 分 / 4 评，11-13 23:07（北京）。HN 50487510 为 Reuters 报道链接，9 分 / 1 评，11-14 02:17（北京）。Reuters 原文 HTTP 401，记为 blocked，未绕过，未存档正文。
- 状态：已找到（范围与合作方）；“不抽成”未找到一手来源。

### C2 LangChain C 轮 3 亿美元、估值 30 亿美元

- 一手来源：https://www.langchain.com/blog/series-c（已存档 `c-c2-langchain.html` / `.txt` / `.headers`）
- 发布时刻：页面日期 November 13, 2026（作者 Harrison Chase）；官方未给时刻。
- 级别：L1
- 原文摘句（逐字）：
  - 标题：“LangChain Raises $300M Series C at $3B Valuation”
  - “Today, we’re announcing that LangChain raised $300M at a $3B valuation to advance our mission to enable every company to own its intelligence. The round was led by existing investor CapitalG, with significant participation from new investors GIC and DFJ Growth and existing investors IVP and Sapphire Ventures.”
  - “More than 7,000 organizations already rely on LangSmith”
- 能改变含义的限定词：无估值前后口径限定（原文未写 pre/post-money）。
- 员工数 435→850（到 2027 年底，计划）：官方文中无此数字，也无“计划”表述。搜索 “435 850 employees” 只找到二手数据站，无一手来源。**未找到一手来源**。
- WSJ 报道：派工单未给 URL，未查，不绕付费墙。
- 热度：HN 50486088，4 分 / 0 评，11-14 00:44（北京）。
- 状态：融资额、估值、领投方已找到；员工数未找到一手来源。

### C3 Devin Teams 方案调整

- 一手来源：https://devin.ai/blog/devin-teams-plan（已存档 `c-c3-devin.html` / `.txt` / `.headers`）
- 发布时刻：页面日期 November 12, 2026（Cognition），官方未给时刻。
- 级别：L1
- 原文摘句（逐字）：
  - “We’re retiring quota seats on the Teams plan in 30 days.”（“in 30 days” 未写起算日，按发布日推算约 12 月 12 日，属推断）
  - “Teams plans are now limited to 100 seats, down from 200.”
  - “making free SWE-2 in Devin Desktop and CLI a permanent part of our Teams, Pro, and Max plans.”
  - “Every user gets 2 concurrent Free sessions and you can always pay for more.”
- 能改变含义的限定词：“2 concurrent Free sessions”（免费会话并发上限）。
- 扫描说法核对：“(still subject to usage limits)”——**原文未见该括注**，与说法不符。“permanent”一句已找到。
- 热度：HN 搜索 “Devin Teams” 0 条。
- 状态：部分支持（席位 200→100、30 天退役、SWE-2 永久已找到；括注未找到）。

### C4 Claude Code Projects 更新

- 一手来源：
  - 官方文章：https://claude.com/resources/articles/projects-redesigned（已存档 `c-c4-claude-projects.html` / `.txt` / `.headers`）
  - 官方文档：https://code.claude.com/docs/en/claude-projects（已存档 `c-c4-cc-projects-docs.html` / `.txt`）。扫描给的 `/docs/en/projects` 返回 404，已记录。
  - 官方 changelog：https://code.claude.com/docs/en/changelog（已存档）
- 发布时刻：文章页面 metadata（`article:published_time`、`datePublished`）为 **2026-09-17**，页眉亦为 “September 17, 2026”。正文写 “Starting today, updated projects are available in beta …”。**扫描称 “11-13 22:51 UTC 前后”，与官方页日期不符**。官方 X 账号未查（派工单未给 URL）。
- 级别：L1
- 原文摘句（逐字）：
  - 文章：“Starting today, updated projects are available in beta to select Claude Pro and Max subscribers who use cloud sessions in Claude Code and don’t have any existing projects on the web or desktop.”
  - 文章：“Over the coming week, we'll expand access to more Claude Code users on those plans. Updated projects across all of Claude and Team and Enterprise plans come after that.”
  - 文章：“Claude also checks in and follows through on work.”
  - 文档：“Projects are in public beta on Pro, Max, and Ultra plans and rolling out gradually.”
  - 文档：“Projects aren’t offered on Team or Enterprise plans yet.”
  - 文档：“The enforced limit is 500 new threads per day across your projects.”
  - 文档：“You can’t share a project or its threads with another user, and thread transcripts don’t have the share option other cloud sessions have.”
- 扫描说法核对：
  - 协调器主动汇报进度：部分支持。文章写 “Claude also checks in”，并说可调 “how often it checks in”，没有“主动汇报进度”原词。
  - /btw 与 /fork：**未找到**。文章与文档均未提及。changelog 中 /btw、/fork 为既有功能的修复条目，未与 Projects 更新关联。
  - 单线程只读分享：**与说法不符**。文档明确写 thread transcripts 没有分享选项，项目及线程不能与他人分享。
  - 套餐：扫描称 “仍是 Pro/Max 公测”。文档写 Pro、Max 和 Ultra；Team 与 Enterprise “yet” 未开放。应以文档为准，补上 Ultra。
  - 线程上限 500 还是 200：官方写的是**每日新建线程上限 500**（enforced limit 500 new threads per day）。未见 200。并发数无固定数字。
- 热度：HN 50492141，92 分 / 86 评，11-14 08:28（北京）。HN 链接指向上述官方文章（claude.com）。
- 状态：部分支持（公测范围、线程上限已找到；日期与“只读分享”与扫描不符；/btw、/fork 未找到）。

### C5 Google Antigravity 上线 Dynamic Workflows

- 一手来源：
  - 文档：https://antigravity.google/docs/dynamic-workflows（已存档 `c-c5-antigravity.html` / `.txt` / `.headers`）
  - 迁移页：https://antigravity.google/docs/migration/workflows-to-skills/（已存档 `c-c5-workflows-to-skills.*`）
  - 变更日志：https://antigravity.google/docs/changelog（已存档 `c-c5-antigravity-changelog.*`）
- 发布时刻：变更日志标注 Antigravity App **v2.32.0**，标签 Latest，日期 **November 13, 2026**，官方未给时刻。
- 级别：L1
- 原文摘句（逐字）：
  - 文档：“Plan availability: Dynamic Workflows are available on Google AI Pro and Google AI Ultra plans across the Google Antigravity App and the Antigravity CLI.”
  - 变更日志：“This release adds Dynamic Workflows, letting agents orchestrate parallel subagents and multi-step tasks in Python using the runtime bundled with the app.”
  - 迁移页：“Workflows are deprecated, and a new workflows system is on the way. Your current workflows keep working for now.”
  - 迁移页：“The new workflows system is Dynamic Workflows”
  - 迁移页：“The /migrate-workflows command is available only in the Antigravity App 2.12.0 and later, not in the Antigravity CLI or the Antigravity IDE.”
- 能改变含义的限定词：“for now”（旧工作流仍可用，只是已弃用）。“AI Pro and AI Ultra plans”。
- 扫描说法核对：“仅 Google AI Pro/Ultra”——一致。“App 与 CLI”——一致。“旧 Markdown 工作流弃用”——一致（原文为 deprecated，仍可运行）。**“无独立 API”**：原文未见相关说明，**未找到一手来源**，不得写成官方说法。
- 热度：HN 搜索 “Antigravity Dynamic Workflows” 0 条。
- 状态：已找到（套餐、App 与 CLI、弃用、发布日期）；“无独立 API”未找到。

### C6 Sierra 发布 Poppy 1.0，Shopify 为全部商户启用

- 一手来源：
  - Sierra 博文：https://sierra.ai/blog/releasing-poppy（已存档 `c-c6-poppy.*`）
  - 规范站：https://personalagentprotocol.org/docs/path-to-1（已存档 `c-c6-poppy-path-to-1.*`）；首页 `c-c6-poppy-spec.*`
- 发布时刻：Sierra 博文日期 November 13, 2026，官方未给时刻。
- 级别：L1（Sierra 为协议共同发起方；Shopify 的覆盖情况是 Sierra 博文中的陈述，未找到 Shopify 自己的公告）
- 原文摘句（逐字）：
  - “Shopify has enabled Poppy for every merchant on its platform.”
  - “every Shopify store now automatically publishes a Poppy file, so merchants don’t need to do any extra implementation.”
  - “Version 1.0 of Poppy reflects what design partners have learned over the past five weeks …”
  - “bringing the total to more than 100”（设计伙伴）
  - “Multiple design partners are now going live with Poppy, including Mercury, Synchrony, and Turkish Airlines.”
  - 规范站：“Docs version: Draft 1.0”；“Build against the 1.0 draft [/docs/1.0/spec], the latest version.”
- 能改变含义的限定词：Sierra 博文称 “Version 1.0”，未写 draft；规范站仍标 “Draft 1.0”，首页另有 “Version 0.4 is the 1.0 release candidate”（首页存档于 `c-c6-poppy-spec.txt`）。两处表述不一致，全部记录，不自行取舍。
- 扫描说法核对：“设计伙伴超 100 家”——一致。“实装 3 家”——**与说法不符**。原文只点名三家，没有给出“3 家”这一计数。“标 draft 1.0”——规范站一致；Sierra 博文无 draft。“Shopify 为全部商户启用”——一致。
- 热度：HN 搜索 “Sierra Poppy” 与 “Poppy Personal Agent Protocol” 均 0 条。
- 状态：部分支持（版本命名两处不一致，实装数计数错误）。

### C7 四家非营利组织起诉五家 AI 实验室，要求禁止开发更强模型

- 一手来源：Public Citizen 新闻稿 https://www.citizen.org/news/coalition-of-nonprofits-files-landmark-lawsuit-to-stop-development-of-advanced-ai-systems-at-anthropic-google-meta-openai-and-spacexsi-that-proliferate-bioweapons-and-cyberattacks/（已存档 `c-c7-citizen.html` / `.txt` / `.headers`）
- 发布时刻：新闻稿日期 November 12, 2026，正文写 “Today … filed a lawsuit”。官方未给时刻。
- 级别：L1（原告方自身发布的新闻稿，属“直述”；法院与诉状信息未经独立核实）
- 原文摘句（逐字）：
  - “Today, Irreplaceable Education Fund, Public Citizen, Heat Initiative, and The Alliance for Secure AI filed a lawsuit … seeking to freeze the development of increasingly advanced AI models at Anthropic, Google, Meta, OpenAI, and SpaceXSI that can escape human control, including fueling the proliferation of bioweapons and cyberattacks.”
  - “The lawsuit, filed in Superior Court of California, County of San Francisco, alleges …”
  - “The plaintiffs are invoking California’s Unfair Competition Law …”
  - “Rather than wait for a mass casualty event to occur, the plaintiffs believe California law enables them to seek a freeze on the development of the most dangerous AI systems now.”
- 能改变含义的限定词：“seek a freeze”；“filed in Superior Court of California, County of San Francisco”。
- 扫描说法核对：
  - 原告为四家：**一致**（新闻稿列出四家机构；标题写 “Coalition of Nonprofits”）。
  - 五被告：**一致**。第五被告在新闻稿中写作 **“SpaceXSI”**，原文未展开为 SpaceX Super Intelligence 或 xAI。扫描的展开写法未得到核实，按原文写 SpaceXSI。
  - “诉求即刻禁令、不索赔”：新闻稿未写“即刻”或“不索赔”。诉状链接 slug 含 “complaint-for-injunctive-relief”，可支持禁令类诉求，但这是链接文字，不是正文。**“不索赔”未找到**。
  - 法院裁定：搜索未找到任何裁定。**未查到**。
  - Just Security 文章：搜索未找到。**未找到**。
  - 诉状页（platkinllp.com）：派工单所指的诉状页未直接抓取，未绕反爬。
- 注意：搜索时出现另一起案件（四名消费者在北加州联邦法院起诉 Anthropic、OpenAI、SpaceXAI、Google 的反垄断集体诉讼，2026-09-18 立案，涉 “Pacing the Frontier”）。这与本条不是同一案件，不可混用。
- 热度：HN 搜索 “Public Citizen lawsuit AI” 0 条。
- 状态：部分支持（起诉、原告、被告名单、法院已找到；“不索赔”未找到；裁定与 Just Security 未找到）。

### C8 OpenAI 解散独立机器人团队

- 一手来源：Fortune 报道 https://fortune.com/2026/11/13/openai-disbands-robotics-team-as-its-general-models-start-running-robots/（已存档 `c-c8-fortune.html` / `.txt` / `.headers`）。Fortune 为 L4 媒体，OpenAI 发言人向 Fortune 确认。
- 发布时刻：原文 “November 13, 2026, 12:05 PM ET”（美东时间，11 月为 EST，UTC−5），折合北京时间 **2026-11-14 01:05**。
- 级别：L4（据 Fortune 报道；公司发言人向 Fortune 确认）
- 原文摘句（逐字）：
  - “OpenAI has disbanded its stand-alone robotics team and reassigned the researchers working there to other parts of the company.”
  - “The company confirmed the reorganization when asked about it by Fortune.”
  - “OpenAI said it has reassigned its robotics research team to the company's main research team, reporting to Chief Research Officer Mark Chen, while the applied robotics and hardware teams have been reassigned to the company's larger Applied organization, which reports to OpenAI President Greg Brockman.”
  - 对 74.8% 与 51.0%：“In an Nvidia study, an Astra-powered agent achieved a 74.8% success rate on a simulated manipulation benchmark—where the model commands a simulated robot in a digital simulation, as opposed to a real robot—compared with 51.0% reported by the best specialized robotic models on this leaderboard.”
- 能改变含义的限定词：“In an Nvidia study”（非 OpenAI 自测）；“simulated”（仿真，非真机）；“reported by the best specialized robotic models on this leaderboard”（对比对象为该榜单上的最佳专用机器人模型）。
- 热度：HN 50487184，172 分 / 72 评，11-14 01:55（北京）。与扫描一致。
- 状态：已找到。

---

## 二、扫描说法勘误（汇总）

1. C1：“上线初期不抽成”未找到一手来源，删去或改为“媒体报道”。
2. C2：“员工 435→850（到 2027 年底，计划）”未找到一手来源。
3. C3：“(still subject to usage limits)”原文无此括注。
4. C4：
   - 发布日期：官方文章 datePublished 为 2026-09-17，扫描称 11-13 22:51 UTC，与官方不符。
   - 文档路径：`/docs/en/projects` 为 404，正确为 `/docs/en/claude-projects`。
   - 套餐：文档为 Pro、Max、Ultra 公测；Team、Enterprise 未开放。
   - “单线程只读分享”与官方相反（不能分享）。
   - 线程上限为每日 500 个新线程，未见 200。
   - /btw、/fork 未与 Projects 更新关联。
5. C5：“无独立 API”未找到。发布日期可补：App v2.32.0，2026-11-13（官方变更日志）。
6. C6：“实装 3 家”为点名三家，不是计数。“draft 1.0”：规范站为 Draft 1.0，Sierra 博文为 Version 1.0，两处不一致。
7. C7：第五被告原文为 “SpaceXSI”，未展开。“不索赔”未找到。搜索结果中的 “SpaceXAI” 与 “Pacing the Frontier” 属另一起联邦反垄断诉讼，不是本条。
8. C8：无误。北京时间为 2026-11-14 01:05。

---

## 三、未完成或受限项

- Reuters（C1 热度旁证）：HTTP 401，blocked，未绕过。
- WSJ（C2）：派工单未给 URL，未查。
- 官方 X 账号（C4 “11-13 22:51 UTC” 的发布来源）：未查，派工单未给 URL。
- Platkin 诉状页（C7）：未直接抓取，未绕反爬。
- Reddit：`barking heat --reddit` 对 “OpenAI robotics team” 返回登录页（非 JSON），记为失败，已写入 `c-heat.json` 的 errors。其余条目未查 Reddit。

## 四、文件

- 本文件：`/Users/develata/gitclone/AI-Barking/docs/2611/1113/sources/evidence-C.md`
- 抓取与操作日志：`/Users/develata/gitclone/AI-Barking/docs/2611/1113/sources/capture-log-C.md`
- 存档（c- 前缀）：`/Users/develata/gitclone/AI-Barking/docs/2611/1113/sources/`（含 `c-fetch-log.jsonl`、`c-heat.json`）
