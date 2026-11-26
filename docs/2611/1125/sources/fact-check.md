# 1125 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B 组 Sonnet 5.5（`claude-sonnet-5-5`，effort high，A 约 28 分钟、B 约 26 分钟），C 组 Haiku 5.5（`claude-haiku-5-5`，effort high，约 9 分钟）。Claude 抽查（11-26 21:00–22:30）：Tibo 原帖（fxtwitter JSON 自取）、grh-lean README（gh api 自取）、子目录 README、Zulip 摘录、rh-shrink 数据与截图、Wayback 价格页三个快照、智东西与量子位稿题，逐字对过下表标“抽查”的行。

## 一、GPT-6.2 计费修正（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：额度更快用完？OpenAI：上周少扣了 | L2 | ✅ | Tibo 原帖“We made a billing mistake at launch of GPT-6.2 that incorrectly discounted GPT-6.2 Astra and Sol usage for most users. We’ve fixed this now, so the same work will use more of your plan’s usage allowance.”（`a-x-tibo-2125784147889393806.raw.json`，截图 01，抽查） | “上周”= 上线（11-17 美国日期）后被少扣的一周；“OpenAI：”署员工所在公司，正文写明是员工发帖。标题里没有待纠正的说法，所以不用“说法 + 实际”；读者稿检查后由候选 1 改为候选 2 |
| 省流：OpenAI 员工称 GPT-6.2 两款模型上线时少扣了额度 | L2 | ✅ | 同上（抽查） | “两款模型”= GPT-6.2 Astra 与 GPT-6.2 Sol；Tibo X 简介“Codex & ChatGPT @OpenAI”（A1-2） |
| 修正后比上周更快用完 | L2 | ✅ | “Comparing to last week, you will reach your limits 40% faster on GPT-6.2 Astra and 25% faster on Sol.”（抽查） | |
| 有媒体写成“砍近半” | L4 | ⚠️ | 智东西（凤凰网大风号）稿题“突发！ChatGPT额度爆砍接近一半？”（`a-cn-ifeng.txt`，截图 05，抽查） | 缩写自“爆砍接近一半” |
| 黎曼猜想机器证明出了7.6万行短版，是 Claude 重写的，不是从原版删出来的 | L2 | ✅ | 见第二节“紧凑版”“重写”两行；子目录 README“The Lean text and the documents in this project were written by Claude (AI).” | “不是从原版删出来的”= README“a shorter rewriting … many steps are proved by other arguments”；部分背景材料从原版搬来后重写（“ported from there and then rewritten”），所以不写“全新写成” |
| 11月26日上午，OpenAI 员工 Tibo 发帖称 | L2 | ✅ | `created_at` “Thu Nov 26 02:50:10 +0000 2026”= 北京 10:50:10（抽查） | |
| GPT-6.2 上线时计费出错，多数用户用 Astra、Sol 两款模型的额度被少扣 | L2 | ✅ | “billing mistake at launch … incorrectly discounted GPT-6.2 Astra and Sol usage for most users”（抽查） | 用户晒出的邮件写“Some GPT-6.2 Astra and Sol requests in Codex”（L7，未进正文） |
| 修正后，同样的活和上周比，Astra 快40%、Sol 快25%用到上限 | L2 | ✅ | “so the same work will use more of your plan’s usage allowance. Comparing to last week, …”（抽查） | |
| 他称两者仍应比上一代 GPT-6-Astra、GPT-6.1-Sol 耐用 | L2 | ✅ | “You should still see these go further in usage compared to 6 Astra and 6.1 Sol.”（抽查） | 带“should”，故写“仍应”；无数字 |
| 其他模型不受影响，过去少扣的不追缴 | L2 | ✅ | “Other models weren’t affected, and we won’t charge you for the difference on past usage.”（抽查） | 同帖末段补偿为额度重置，按规定不进正文 |
| 对你：剩余额度和重置时间在 ChatGPT 设置的“用量”页看 | L1 | ✅ | help 20001516“Check Settings → Usage for your remaining allowance and reset …”；20001625 同（`a-help-20001516.txt`、`a-help-20001625.txt`，Claude grep） | “用量”= Usage |
| 只用其他模型的不受影响 | L2 | ✅ | “Other models weren’t affected” | |
| 吠点①：智东西稿题“额度爆砍接近一半？” | L4 | ⚠️ | `a-cn-ifeng.txt`，标题“突发！ChatGPT额度爆砍接近一半？”（抽查）；同稿 163 网易号 | 摘标题后半；北京 11-26 11:04:36 |
| Tibo 原帖只说 Astra 快40%用到上限，没给额度绝对数 | L2 / L1 | ✅ | 原帖只有“40% faster”；官方帮助中心、套餐页、changelog、status 页均无数值说明，套餐页只有“not fixed message limits”的估计区间（A2-2、A3-1、A4-1） | |
| 按两种读法换算，上周能干10件活的额度，现在约干6到7件 | — | ✅ | 本号换算。读法一：额度扣得快1.4倍 → 10 ÷ 1.4 ≈ 7.1 件；读法二：用完时间缩短40% → 6 件 | 推算，非官方，“10件”是换算参照；读法一宝玉xp 微博同算（A6-2，L6），读法二见 Notebookcheck 中文版标题“时间最多缩短 40%”（A6-5）。Sol 按读法一约8件，正文只写 Astra |
| 吠点②：40%和25%是跟上周少扣时比 | L2 | ✅ | 原帖“Comparing to last week”+ 首句“incorrectly discounted”（抽查） | |
| Tibo 称仍应比上一代耐用，没给数字 | L2 | ✅ | 同“仍应比”行 | “上一代”= GPT-6 Astra 与 GPT-6.1 Sol；保留“应”（should） |
| OpenAI 套餐页给 Plus 写的“每5小时约能发几条”，11月18日和26日存档一样 | L1 | ✅ | 套餐页“The estimates below show local messages per five-hour period for Plus and Standard Business.”；GPT-6.2 Astra 6-50、GPT-6.2 Sol 17-180 在 Wayback 11-18 03:56 UTC、11-26 03:16 UTC 快照与 11-26 21:39 现页相同（`a-wayback-learn-pricing-20261118035643.txt`、`-20261126031654.txt`、`a-codex-pricing.txt`，Claude grep 三份）；中间 11-19、11-23、11-24、11-25 快照亦同（A4-2） | “每5小时约能发几条”是对估计表的意译，引号表示页面上的那张表，不是逐字引语；官方称这些区间“not fixed message limits”。反向核验第一轮：原稿“从6.2上线起没改过”由离散快照推出全时段，改为“11月18日和26日存档一样”。Wayback 只用于看页面以前写了什么 |

未进正文（核过，备用）：12 月起 ChatGPT 与 Codex 共用额度的帮助页现多一段“A small group of users may see the usage changes before December”（A5-1；1031 期已发合并本身）；TechCrunch 11-25 稿讲的是 Chat 与 Work 合并，与计费修正无关（扫描张冠李戴）；凤凰网“营销方式”一句是记者接着用户推测下的判断（A6-1b）；用 API key 登录的 Codex 会话按 API 价另计、不扣套餐额度（套餐页，初稿“对你”，读者稿检查后撤下）。

## 二、黎曼证明短版（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| 11月26日凌晨，Anthropic 的 Luke Sernau 公开黎曼猜想机器证明的短版 | L2 | ✅ | grh-lean 建仓 2026-11-25T20:42:55Z = 北京 04:42:55、首提交 04:38:07（gh api，Claude 自取）；README“produced in collaboration with my colleagues at Anthropic”；Zulip“sprinting with a few colleagues at Anthropic”（`b-zulip-excerpts.md`，截图 24，抽查） | 仓库在个人账号 `lsernau`，非 Anthropic 官方账号；版权“Anthropic, PBC”。证明还覆盖狄利克雷 L 函数的广义黎曼猜想，正文从简 |
| 原版是 Alpöge 与 Claude 11月20日公布的约1700万行 Lean 代码 | L1 / L3 | ✅ | 1120 存档：remark.pdf CreationDate 北京 11-20 20:52；Lean FRO 与 Mathlib 声明“a Lean library comprising 17 million lines of code”（1122 存档） | Sernau 自述原版“about 15 million lines”（Zulip，批注图 zulip 可见）；第三方计数 17,374,215 总行。正文取官方声明 |
| Lean 是让电脑逐步检查证明的语言 | — | ✅ | 通俗解释；与 1120、1122 期写法一致 | |
| 短版沿用原版方法，他和同事带 Claude 重写 | L2 | ✅ | 子目录 README“This project is a shorter rewriting of Levent Alpöge's Lean proof … It follows that proof's method”；“The Lean text and the documents in this project were written by Claude (AI).”（两个子目录 README，Claude grep；截图 14） | |
| 紧凑版约7.6万行，可读版约30万行 | L2 | ✅ | 顶层 README“readable edition, about 300,000 lines”“about 76,000 lines”（抽查） | B 组 cloc 实测 code 行 299,902 / 76,149 |
| 只过了机器检查，没有数学家通读 | L2 | ✅ | 顶层 README“The proof has passed machine checks only … No mathematician has read it through, and no paper comes with it.”（抽查） | |
| MIT 的 Sutherland 复核称，两版都能编译，证明的命题与原版一致 | L3 | ✅ | `b-sutherland-shortgrh.txt`：“Both editions compile without errors”；“Statements.lean … its four definitions are textually identical to those in the original proof's Statements.lean”；comparator“Your solution is okay!” for both editions | 页面写明“checks run with Claude Code”，最后更新 北京 11-26 10:50；同页另称两版只依赖三条标准公理；是代码检查，不是数学审阅 |
| 对你：普通读者用不上；想看 AI 写的证明长什么样，可读版附了导读 | L2 | ✅ | “Start at `grh/GUIDE.md`”（抽查） | “AI 写的”= README“written by Claude (AI)” |
| 吠点①：量子位稿题“黎曼猜想证明代码暴砍了99.5%” | L4 | ⚠️ | 腾讯新闻转量子位，标题“刚刚，黎曼猜想证明代码暴砍了99.5%！只剩7.6万行”，北京 11-26 08:40（`b-cn-qq-995.txt`，截图 20，抽查） | 摘标题中段 |
| 这是 Claude 重写的新版对比原版，许多步骤换了论证 | L2 | ✅ | 子目录 README“a shorter rewriting … many steps are proved by other arguments”“written by Claude (AI)”（截图 14）；量子位正文的99.56%即紧凑版 76k 对原版 17.37M（B5-1）。口径保留：76,149 是紧凑版 cloc code 行，17,374,215 是原版总行（含空行注释），同口径总行比为 112,827 / 17,374,215，约少 99.35%；正文只引量子位的数，不自算缩减率（B 组 零-1“部分支持”） | 反向核验第一轮 BLOCKER：量子位正文自己写了“彻底的重写与重构”，原稿把“重写”当作对量子位的纠正，改为用社区删减的结果对照 |
| 社区在原版上删减了五天多，有效代码从约1462万行只删到约1100万行 | L3 | ✅ | `b-rh-shrink-earlier.json`（Claude 读）：首点 `40e56bab` 2026-11-20T09:36:09-05:00（北京 11-20 22:36）code 14,621,937；换版前末点 `756a7025`（PR #4761，2026-11-26T08:25:34+11:00 = 北京 11-26 05:25）code 11,000,328；降 24.77% | “五天多”= 约5天7小时；“有效代码”= rh-shrink 的 code*：去空行、纯注释行与 import 行。反向核验第二轮：原稿“只少约四分之一”只有相对数，补两端绝对数 |
| 吠点②：统计网站 rh-shrink 显示代码“只剩2.0%” | L3 | ✅ | 截图 16（北京 21:45）：“2.0% of the original code lines* left”（Claude 目视）；`b-rh-shrink-data.json` 换版后首点 `47fcde57` code 298,160（2.04%）、末点 `120962b4`（2026-11-26T13:22:57Z）code 296,425（2.03%），对原版 14,621,937（Claude 算） | 第三方文本统计，不跑 Lean |
| 是因为社区公共仓库11月26日整个换成了约30万行的可读版，不是删出来的 | L3 | ✅ | rh-shrink：“On Nov 25, 2026, main was replaced by the readable edition of the proof (grh-lean)”（`b-rh-shrink-home.txt`，抽查）；换版提交 `47fcde57` 2026-11-25T22:24:29Z = 北京 11-26 06:24；`b-rh-shrink-data.json` 换版后首点即 `47fcde57`（803 文件、477,124 总行，与可读版规模一致） | “社区公共仓库”= `leanprover-community/riemann-hypothesis` 的 main 分支；可读版约30万行（README） |

未进正文（核过，备用）：“240 万行 / 1462→1211 万 / 16.5% / 8.4%”等出自量子位 11-24 19:08 稿，比 grh-lean 早两天，量的是原版的社区删改；16.5% 与 1211 万来自两个不同提交（B3-1）；VerifyTask 现行 33.52%（11-26 21:41），8.4% 是 11-24 的旧快照；rh-shrink“Files 801，↓99.5%”是文件数降幅；“99.5%”的分母有 15M（Sernau）与 17.37M（第三方总行数）两种；anyGeometry 质疑在原版，grh-lean 两版无此名（B6）；Lean FRO / Mathlib 的认证声明没有提短版（B2-6）；构建需 12 核、24GB 内存、一到两小时（初稿“对你”，读者稿检查后撤下）。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注 |
|---|---|---|---|---|
| OpenAI：GPT-6.2 少扣了额度 | — | ✅ | 正文小节标题，逐字 | 主帖 |
| 黎曼证明短版：7.6万行，是重写 | — | ✅ | 正文小节标题，逐字 | 主帖 |
| 学者 Aaronson 撰文称，首次打心底觉得 P≠NP 的证明不远了 | L2 | ✅ | `c-c8-aaronson.txt`：“I now feel in my bones, for the first time ever, that a proof of P ≠ NP is not far.”（博文北京 11-26 03:01） | 同文称“virtually certain”实验室握有重磅成果，但拒绝传播未证实细节；HN 1700 分 |
| OpenAI 内部模型写的手稿称推翻多项式 Hirsch 猜想 | L1 | ✅ | `c-c1-openai-math-paper-raw.txt` 摘要“We disprove the polynomial Hirsch conjecture.”；仓库 README“produced by an unnamed internal OpenAI model”“raw research data, rather than polished academic papers” | 11-24 日期，官方未给时刻；HN 167 分 |
| Anthropic 称把 API 从 Python 迁到 Rust，没停过服务 | L1 | ✅ | `c-c6-anthropic-rust.txt`：“This allowed us to ship an API on Rust that uses about 70% less CPU per request and cuts median time to first token by about 20% without ever taking the service offline or freezing feature development.”；“from April until public traffic moved to Rust in September 2026” | 官方 11-25，未给时刻。原稿“单区域每请求 CPU 约少七成”只有相对数（反向核验两轮），改写为不含比例的事实。内存：本地正文文本未包含，反向核验第二轮实时读到官方图15替代文字称每请求内存降逾80%（图注：内存为全部生产环境），中文稿“内存降80%”有官方出处 |
| （未收）OpenAI 称 GPT-6.2 设计的蛋白在5个靶点的实验中显示结合迹象 | L1 | ✅ | `c-c9-openai-protein.txt`：“In lab tests with Onava, proteins based on GPT‑6.2’s designs showed evidence of binding, or attachment, across five targets.” | 同页“Specialized models helped supply sequences and select candidates for testing.”；官方 11-24 |
| OpenAI 称一小批客户在测 Private Inference（Alpha） | L2 / L1 | ✅ | Altman 帖“We’re now testing Private Inference … with a small group of customers.”（`c-c4-altman-fx.json`，北京 11-26 01:12）；官方页“Private Inference (Alpha)”（`c-c4-openai-private.txt`） | 官方页无“independently verified” |
| 据《财富》报道，亚马逊与 Meta 在谈让 Muse 购物代理接入亚马逊 | L4 | ⚠️ | `c-c5-fortune-muse.txt`：“Amazon and Meta Platforms are in ongoing discussions over a potential deal to connect Meta’s popular Muse AI agent to the e-commerce giant’s online store”，“according to people familiar with the matter”，两家拒绝置评 | 北京 11-25 16:01；L4 以“据报道”措辞 |
| （未收）据 The Verge 报道，纽约时报要供稿者别用 Photoshop 2027 | L4 | ⚠️ | `c-c9-verge-nyt-photoshop.txt`：“The New York Times is asking freelance photographers not to use the newest version of Adobe Photoshop because features in the 2027 update use AI ‘in ways that may not be apparent to users.’”；邮件原句“we are requiring that all contributing photographers avoid using Adobe Photoshop 2027 (release 28.…” | 北京 11-25 00:09；邮件由 The Verge 看到，原件未公开 |

未收：纽约时报与 Photoshop（反向核验与视觉复核第二轮：“供稿者”扩大了“contributing photographers”，且漏了“为时报执行任务时”，补全后超40字，撤下）；OpenAI 蛋白质设计（反向核验第一轮指出“GPT-6.2 设计的蛋白”漏了专用模型配序列与筛选，补全后超40字，撤下）；纽约州禁售 AI 聊天玩具（风控：外国政府立法，留给 Develata 定）；JetBrains Mellum2.1（官方博文未找到，“45%”与原文不符）；蚂蚁 Ling-3.2-air（AA 复现数未找到）。

## 批注译注

| 卡片 | 译注 | 级 | 结果 | 依据 |
|---|---|---|---|---|
| tibo-post ① | GPT-6.2 上线时我们犯了一个计费错误，多数用户的 GPT-6.2 Astra 与 Sol 用量被错误地打了折。 | L2 | ✅ | “We made a billing mistake at launch of GPT-6.2 that incorrectly discounted GPT-6.2 Astra and Sol usage for most users.” |
| tibo-post ② | 和上周相比，你用 GPT-6.2 Astra 触到额度上限会快40%，用 Sol 快25%。 | L2 | ✅ | “Comparing to last week, you will reach your limits 40% faster on GPT-6.2 Astra and 25% faster on Sol.” |
| tibo-post ③ | 与 GPT-6 Astra、GPT-6.1 Sol 相比，这两个模型的用量应该仍然更耐用。 | L2 | ✅ | “You should still see these go further in usage compared to 6 Astra and 6.1 Sol.” |
| plus-estimates ① | 这些估计不是固定的消息上限。 | L1 | ✅ | “These estimates are not fixed message limits”（视觉复核后底图改为只含说明段与表格的 `03b-pricing-plus-table.png`） |
| plus-estimates ② | GPT-6.2 Astra：Plus 与标准 Business 每5小时本地消息估计 6–50 条。 | L1 | ✅ | 表行“GPT-6.2 Astra 6-50 6-50”；表头说明“local messages per five-hour period for Plus and Standard Business” |
| plus-estimates ③ | GPT-6 Astra：同口径 5–45 条。 | L1 | ✅ | 表行“GPT-6 Astra 5-45 5-45” |
| rewriting ① | 本项目的 Lean 文本与文档由 Claude（AI）撰写。 | L2 | ✅ | “The Lean text and the documents in this project were written by Claude (AI).”（视觉复核后底图换成 760 CSS px 宽重渲的 `14b-grh-rewriting-narrow.png`；两个像素框只框这一句，不含同行前句“The proof is Levent Alpöge's, with Claude.”） |
| rewriting ② | 它沿用那份证明的方法；许多步骤用别的论证来证 | L2 | ✅ | “It follows that proof's method; many steps are proved by other arguments” |
| rh-shrink ① | 原始代码行（不含 import）还剩2.0%。 | L3 | ✅ | “2.0% of the original code lines* left”“* without imports” |
| rh-shrink ② | 11月25日（UTC，即北京11月26日）：主分支换成 grh-lean 的可读版；此前的删减工作在 alpoge 分支继续。 | L3 | ✅ | “Nov 25: main replaced by the readable edition of grh-lean; the earlier work goes on in the branch alpoge”；页面日期为 UTC，北京为 11-26 06:24 |
| zulip ① | 从约1500万行减到约7.6万行 | L2 | ✅ | “from about 15 million lines to ~76,000” |
| zulip ② | 去掉了对弱形式 Vandiver 猜想的依赖 | L2 | ✅ | “removes the dependence on the weak form of Vandiver's conjecture” |

## 卡片日期

`cards.toml` 的 `date` 写 11月27日（预计发布日；交付时间为北京 11-26 深夜，由 Develata 定发布日）。

## 反向核验与视觉复核

### 第一轮反向核验（Codex `gpt-6-astra` medium，read-only；北京 11-26 约 21:55–22:10；审的是读者稿检查之前的初稿）

2 BLOCKER、8 SHOULD_FIX、2 NEEDS_VERIFICATION。逐条判断：

1. BLOCKER 标题“媒体称GPT额度砍近半，原帖比的是上周”没纠正“砍近半”：采纳（已因读者稿检查改为“额度更快用完？OpenAI：上周少扣了”，标题不再含待纠正说法）。
2. BLOCKER 量子位正文已写“彻底的重写与重构”，把“重写”当作对它的纠正不成立：采纳。吠点①改为“这是 Claude 重写的新版对比原版，许多步骤换了论证；社区在原版上删减了五天多，有效代码只少约四分之一”，不再暗示量子位说的是删冗余。
3. “自上线起没改过”由离散快照推出：采纳，改为“11月18日和26日存档一样”。
4. “想自己验”的条件不全：已随读者稿检查撤下该句。
5. “他称仍比上一代耐用”丢了“应”、代词不能单独上卡：采纳，改为“Tibo 称仍应比上一代耐用”，吠点①“原帖”补“Tibo”。
6. 行数口径与“换成短版”：部分采纳。吠点②改为“整个换成了约30万行的可读版”；正文“约1700万行”与“约7.6万行”各取官方或作者自己的说法，没有在正文里算缩减率，不改。
7. 蛋白质速览漏了专用模型配序列与筛选：采纳，补全后超40字，撤下该条。
8. Photoshop 速览扩大范围：采纳，改为“纽约时报要供稿者别用 Photoshop 2027”。
9. 相对数没有绝对数：部分采纳。吠点①改用换算参照“上周能干10件活的额度，现在约干6到7件”（EDITORIAL“对比”允许的换算参照）；40%/25% 是 OpenAI 自己的说法，在事实段照录，吠点①同时写明“没给额度绝对数”。速览 Rust 条“CPU 约少七成”照原文“about 70% less CPU per request”，附“单区域”，不改。
10. L4 行标 ✅：采纳，媒体稿题三行改 ⚠️（备注里注明原句已核对）。
11. 卡片未提供：第二轮补审。
12. “2.0%”只有目视：采纳，补 `b-rh-shrink-data.json` 计算（换版后 2.04%、末点 2.03%）。

### 第二轮反向核验（同模型同档，北京 11-26 约 22:10–22:20；审改后正文、卡片配置、清单）

PASS，无 BLOCKER；5 SHOULD_FIX、1 NEEDS_VERIFICATION。逐条判断：

1. Photoshop 速览“供稿者”扩大范围、漏“为时报执行任务时”：采纳，补全超40字，撤下该条（速览改为 7 条）。
2. 行数口径混用：采纳到清单与来源说明（同口径总行比约少99.35%）；正文不自算缩减率，只引量子位原数并说明是重写版对比原版，不改正文。
3. 只有相对数：采纳。吠点①改为“有效代码从约1462万行只删到约1100万行”；Rust 速览删去“CPU 约少七成”，改为“把 API 从 Python 迁到 Rust，没停过服务”。
4. 译注 UTC 日期：采纳，改为“11月25日（UTC，即北京11月26日）”，原文日期照译。
5. “内存降80%原文没有”：采纳，清单与来源说明改为“本地正文文本未含；官方图15替代文字称逾80%”。
6. Kim Morrison 引语未定位到原始存档：采纳，删去该引语，改指 `b-rh-shrink-data.json` 换版点。

### 视觉复核（Codex `gpt-6-astra` medium，16 张图，北京 11-26 约 22:10–22:20）

PASS，无 BLOCKER。

- `02-roundup` 第8条“供稿者”：同上，已撤下。
- `06-plus-estimates` 字小：采纳，底图改为只含说明段与表格的裁切 `03b-pricing-plus-table.png`，像素框随之平移。
- `09-rewriting` 字小、留白多：采纳，按 760 CSS px 宽重渲 README（`14b-grh-rewriting-narrow.png`），像素框重定。
- `11-zulip` 字小（NICE）：不改，截图本身宽度所限。
- 1700万与1500万并见（NICE）：采纳，批注图标题加“（作者自述）”。

改后重渲 6 张，Claude 逐张打开核对：高亮落在目标句上，没扫进相邻句，译注与原句对应。封面未送视觉复核，按 `templates/codex-cover.md` 第 4 步由 Claude 逐字与放大核对（见 `images/README.md`）。
