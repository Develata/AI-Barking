# 1008 期 C 组（速览）取证清单

执行：C 组取证代理（Claude 系，Sonnet）。取证时间 2026-10-09 09:37–10:30 北京时间（本机 UTC 01:37 起；北京 = UTC+8）。基线 commit `35910be`。未 commit、未 push。

约定：
- 时间一律北京时间，括号内为原时区。官方页只有日期时写“官方未给时刻”。
- “建议措辞”是我给 Claude 的一句话候选（标了字数，按 EDITORIAL 最保守口径含空格与标点），不是定稿；英文标识符多，部分略超 40 字，需 Claude 压缩。图上措辞不出现 X/YouTube 等平台名。
- 级别按 EDITORIAL“事实分级”标注。arXiv 预印本不在表内，我按“作者/机构自行发布、未经同行评审”标 L1（作者自述），请 Claude 定级。
- 状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。
- 存档文件均在 `docs/2610/1008/sources/`；PDF、HTML 原件只存本地（不入库），文本可入库。超 1 MB 的文本文件见文末“体积提示”。

---

## C1　AI 购物 agent 按“推断财富”推荐更贵选项

论文：Aman Priyanshu、Supriti Vijay、Brian Jabarian、Niloofar Mireshghallah，“Et Tu, Brute? Economic Misalignment in Personal AI Agents”，arXiv:2609.24927。
- 一手来源：https://arxiv.org/abs/2609.24927 ；PDF v2 https://arxiv.org/pdf/2609.24927v2 （存 `c-arxiv-2609.24927v2.pdf` / `.txt`；v1 同存 `c-arxiv-2609.24927v1.pdf` / `.txt`；arXiv API 元数据 `c-arxiv-api.xml`；摘要页 `c-arxiv-abs.html`）。
- 时刻：v1 2026-09-21T17:22:44Z（= 北京 09-22 01:22:44）；v2 2026-09-25T18:15:11Z（= 北京 09-26 02:15:11）。arXiv 评论栏：“20 pages, 10 tables, 4 figures”。cs.AI。**未经同行评审**（论文首页 “Preprint.”）。
- 作者单位（p.1 逐字）：“1 Foundation AI, Cisco　2 Carnegie Mellon University”。Priyanshu、Vijay → Cisco Foundation AI；Jabarian、Mireshghallah → CMU。
- v1 → v2 差异：我用 diff 比对 pdftotext 输出，仅参考文献增补与编号变化（新增 Hadfield&Koh、Karten 等条目），正文数字、表格、结论无变化。
- 页码指 PDF 页（与论文页脚印刷页码一致）。核对脚本与输出：`c-c1-numcheck.py` / `c-c1-numcheck.txt`。

### C1-a　逐个数字对照（论文为准）

| 转述数字 | 论文位置 | 论文原文（逐字） | 与转述 |
|---|---|---|---|
| 13 个模型 / “13 agents” | 摘要 p.1；§4.2 p.6；附录 A.7 p.17 | 摘要：“325K experiments on 13 agents across three types of economic decisions”；§4.2：“We test a broad range of 13 models spanning 4 model families” | 一致。论文自己混用 agents / models（摘要说 agents，正文说 models） |
| 13 个模型完整清单 | §4.2 p.6；A.7 p.17 | GPT-5、GPT-5-mini、GPT-5-nano、GPT-5.5（OpenAI API）；Gemini 2.5 Flash、Gemini 3 Flash、Gemini 3.1 Flash Lite（Gemini API）；Claude Opus 4.8、Claude Sonnet 5、Claude Haiku 4.5（Anthropic API）；Qwen3.5-2B、Qwen3.5-9B、Qwen3.5-35B-A3B（本地 vLLM） | —— |
| 32.5 万次实验 | 摘要 p.1；结论 p.11 | “325K experiments” / “In 325K experiments across 13 models, 4 independently trained families, and 3 consumer domains” | 一致。**分解论文未明写**；我的推算（推断）：13 模型 × 3 领域 × 32 画像 × 13 个非对照条件 × 20 次（4 种意图 × 5 种动机，p.17）= 324,480 ≈ 325K。另 A.6 p.17：98.2% 的试验给出完整五条推荐，其余剔除 |
| 8 个模型 | 摘要 p.1；§5.1 p.6 | 摘要：“we find that 8 models systematically choose more expensive options for wealthier users when requests are identical”；§5.1：“8 of the 13 models evaluated recommend more expensive options to wealthier personas in every domain where trials pass the inventory gate. These effects survive Benjamini–Hochberg correction (q < 0.05; uncorrected p < 0.001), with mean Cohen’s d ranging from 0.26 to 0.85” | 一致。**论文正文未点名是哪 8 个**（Table 1 平均 d≥0.26 的模型有 11 个，不能据此还原） |
| Opus 4.8 机票 +198 美元 | §1 要点 p.3；§5.1 p.6；Table 1 p.10 | §5.1：“Claude Opus 4.8 exhibits the largest impact (d = 0.85, corresponding to differences of $198 for flights and $284 per month for insurance)”；Table 1：Claude Opus 4.8 Flights +198, Insurance +284, Grad schools +3,467, Mean d 0.85 | 一致。**含义**：Δ = “高金融画像”与“低金融画像”所推荐 5 项均价之差（p.5 式 (1)；A.8 p.17），各 16 个合成画像；Table 1 为 tool-full 条件（五个属性均可经 get_attributes 取得）、各意图（中性/最便宜/最舒适/价格上限）合并平均，并非“相对无背景基线”的涨幅 |
| Opus 4.8 保险 +284 美元/月 | 同上 | 同上 | 一致（月保费，单人、科罗拉多某邮编，p.6、Table 2） |
| 要“最便宜”时 Gemini 2.5 Flash +208 | §5.3 p.8；图 4 图注 p.8；§6 p.10 | §5.3：“if you ask for the cheapest flight, Gemini 2.5 Flash will average $336 for a flight cost for wealthy personas vs $128 for low-income personas - a disparity of $208”；图 4 图注：“Even when users explicitly request the cheapest option, Gemini 2.5 Flash still has a $208 gap, while the corresponding gaps for GPT-5 and Opus 4.8 are $21 and $20, respectively.” | 一致 |
| 同条件 GPT-5 +21、Opus 4.8 +20 | 图 4 图注 p.8 | 同上 | 一致。注意图 4 比的是 **GPT-5**（不是 GPT-5.5） |
| 屏蔽非财务属性使保险差距最多增加 40% | 摘要 p.1；§1 p.3；§5.4 p.9 | 摘要：“blocking financial attributes largely removes the disparity, but blocking other attributes leaves it unchanged and can increase it by up to 40% for insurance”；§1：“blocking access to employment information increased the insurance gap for GPT-5.5 by 40% (from 122 to 171/mo), Gemini 2.5 Flash by 13% (from 217 to 246/mo), and Claude Opus 4.8 by 12% (from 284 to 317/mo)” | 方向一致，但见下“论文内部不一致”与 Quartz 的 GPT-5 误写 |
| 屏蔽财务属性 → 差距几乎消失 | §1 p.3；§5.4 p.9；图 2 | “on flights, gaps ranging between +92 to +198 fell to between -1 and +17” | 一致 |
| 用户画像是否合成 | §1 p.2；§3.1 p.4；§4.1 p.6；A.2 p.14；§6 局限 p.11；C.2 p.20 | p.2：“we create a pool of 32 synthetic users created using a 2^5 design with binary factors”；p.11：“We use personas and mock inventories for our data — this study does not include any real users or fieldwork”；p.20：“All personas, emails, and inventories that we’ve used in our experimentation have been synthetically-generated. There were no human users and no personal information gathered.” | **合成**。所有画像统一取名 “Alex”（p.6）；库存是 200 项固定的模拟库存（机票 $91–$883，丹佛–芝加哥 1/10–1/24；保险 $85–$1,350/月；CS 博士项目净成本 −$20K 至 +$61K/年；Table 2 p.14） |

### C1-b　论文自身的限定词（p.11 §6 “Limitations and external validity”，逐字）

- “(1) We measure recommendation price and composition but not user effectiveness. Outside of explicit preference scenarios, we cannot measure whether these higher-price recommendations decrease user utility: higher-income users might prefer them.”
- “(2) … small sample sizes (we use a no-context baseline of 182-214 samples per domain) … the low-income insurance effect is consistent with zero.”
- “(3) Ours is an incomplete grid: we omit 5 of the 39 model x domain cells due to some models hallucinating the inventory or prices”（Table 1 脚注 ‡：Gemini 3 Flash、3.1 Flash Lite、2.5 Flash 的部分领域因库存校验失败过半被省略）
- “(4) … single-turn interactions with a consistently neutral system prompt, and we don’t investigate multi-turn conversations, explicit anti-profiling system prompts, or long-term memory”；“we only consider a binary wealth variable, which could exaggerate the purity of the signal compared to a true income distribution”
- 模型调用方式（A.7 p.17）：“GPT, Claude, and Gemini run with their default api-settings”，即**经 API + 论文自搭的 MCP 工具环境**，不是 ChatGPT / Claude 消费者应用（Quartz 标题写 “AI chatbots like Claude and ChatGPT”，范围比论文宽）。
- “更贵”不等于有害（p.10 §6）：“We emphasize that our setup is not designed to answer whether the agent is actually helping or hurting their user … this question is genuinely ambiguous”；只有“用户明说要最便宜而 agent 给了更贵的”才明确违背意图。
- 数字上限类提示不会被推翻：p.8–9 “an explicit numerical price limit sharply constrains the gap across models, bringing it close to zero for most capable models. Gemini 2.5 Flash is the notable exception”。

### C1-c　论文内部不一致 / 与转述不符（记录，不取舍）

1. p.3（§1）：GPT-5.5 封锁 employment 后保险差距 “from 122 to 171/mo”（+40%）；p.9（§5.4）：“The strongest amplification effects are found in GPT-5.5, with a 40% increase to a $151 insurance disparity when we block the employment or demographic attributes.” 附录 Table 7 p.18：GPT-5.5 tool_full +122，blocked_employment +171，blocked_demographics +131；**GPT-5.5 一行没有 151**（151 只出现在 Sonnet 5 tool_full 与 Qwen3.5-35B 的 employment 格）。→ §5.4 的 “$151 … or demographic” 与 Table 7 不符（疑似笔误）；以 Table 7 为准的“40%”是 $122→$171，仅 employment。
2. “up to 40%”并非 Table 7 全表最大：Gemini 3.1 Flash Lite 封锁 health 179→326（+82%）、Gemini 3 Flash 332→508（+53%）、Qwen3.5-2B 17→63（+271%）。但这些格要么在 Table 1 被标 ‡（不可靠）、要么模型近零效应（d=0.03）。在 Table 1 标为可靠且非近零的模型中，40%（GPT-5.5）是最大。这是我对论文表格的复算（`c-c1-numcheck.txt`），论文本身没有这样限定。
3. **Quartz 转述错误**：Quartz（2026-10-07T11:33Z = 北京 10-07 19:33）写 “Blocking employment information increased the insurance gap for GPT-5 by 40%”；论文是 **GPT-5.5**（GPT-5 的同一格是 191→210，约 +10%）。
4. Fast Company：派工单所列“Quartz/Fast Company 转述”，我只搜到 Quartz、Inc.com、Digiko 等，**未找到 Fast Company 的对应报道**。
5. Bloomberg newsletter（URL 见 Quartz 引文：`https://www.bloomberg.com/news/newsletters/2026-10-07/study-claude-chatgpt-ai-bots-offer-different-shopping-prices-based-on-wealth`）付费墙，未读；只记 Quartz 转引：Priyanshu 对 Bloomberg 说 “We asked a simple question: if we hand all of that to our assistant and ask it to shop for us, will it use that knowledge against us, the way a seller might?”；以及 “According to Bloomberg, OpenAI said the version of ChatGPT evaluated in the study differs from the one powering its consumer shopping experience. Neither Anthropic nor Google responded to Bloomberg’s requests for comment.”（转引，L5）。

### C1-d　记录行

| 说法 | 级 | 一手来源 URL | 原文摘句 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 13 个 agent/模型，32.5 万次实验，其中 8 个模型对富裕用户系统性推荐更贵 | L1（作者预印本，未经评审） | https://arxiv.org/abs/2609.24927 | 见上表 | 合成画像、模拟库存、单轮、API 默认设置；v2 北京 09-26 02:15 | 已找到 |
| Claude Opus 4.8 差距最大：机票 +198 美元、保险 +284 美元/月 | 同上 | 同上 Table 1 p.10 | “Claude Opus 4.8 … $198 for flights and $284 per month for insurance” | 差距 = 高/低金融画像所推荐 5 项均价之差；tool-full；意图合并平均 | 已找到 |
| 要求“最便宜”时 Gemini 2.5 Flash +208、GPT-5 +21、Opus 4.8 +20 | 同上 | 同上 图 4 图注 p.8 | 见上表 | 仅机票；数值上限（“Under $200”）几乎抹平差距 | 已找到 |
| 屏蔽非财务属性可使保险差距最多增加 40% | 同上 | 同上 摘要 p.1、p.3 | 见上表 | 对象是 GPT-5.5（employment 122→171）；论文 §5.4 写 $151 与 Table 7 不符；Quartz 误写成 GPT-5 | 部分支持（数字有内部不一致） |
| 作者单位 Cisco Foundation AI 与 CMU | L1 | 同上 p.1 | “Foundation AI, Cisco”“Carnegie Mellon University” | —— | 已找到 |
| 用户画像是合成的 | L1 | 同上 p.11、p.20 | “this study does not include any real users or fieldwork” | —— | 已找到 |
| Quartz/Fast Company 转述 | L4/L5 | https://qz.com/ai-chatbots-claude-chatgpt-wealth-pricing-study-100726 | 见上 | Quartz 存 `c-quartz.html` / `.txt`；Fast Company 未找到 | 部分支持（Quartz 找到，Fast Company 未找到；Quartz 有一处模型名误写） |

**建议措辞（C1，39 字）**：预印本称合成画像测试中，Opus-4.8 机票推荐均价富裕比低收入高198美元
（保留限定词：预印本、合成画像、相对低收入画像。其余数字另放备选：保险 +284 美元/月；要“最便宜”时 Gemini-2.5-Flash 仍差 208 美元。）

---

## C2　ts-rust：LLM 把 TypeScript 编译器移植到 Rust

- 一手来源：https://github.com/pingdotgg/ts-rust （README 存 `c-ts-rust-readme.md`；API 元数据 `c-ts-rust-repo.json`；README 历史版本 `c-ts-rust-readme-<sha>.md`；README 改动记录 `c-ts-rust-readme-commits.json`；最近 100 次提交 `c-ts-rust-commits-latest100.json`；发布 `c-ts-rust-releases.json`；npm 元数据 `c-npm-tsc-rs.json`）。
- 取回时刻：2026-10-09 09:38 北京（01:38Z），README 对应提交 `fbb9847e`（2026-10-08T14:07:54Z）。仓库仍在持续推送（取回时 pushed_at 01:37:59Z），以上为取回时快照。
- 仓库属组织账号 `pingdotgg`（owner type Organization）。README 不署名，第一人称（“I wanted to see…”）。贡献者：`t3dotgg` 6035 次、`Strate` 2 次；提交署名 “Theo Browne”（`6751787+t3dotgg@users.noreply.github.com`）。→ “作者是 Theo（t3.gg）”有提交署名佐证（L1），但 README 本身未写名字。

### 时刻
- GitHub 仓库创建：2026-10-07T05:17:07Z（= 北京 10-07 13:17:07）。API 不给“何时公开”，故“公开时刻”只能说**不晚于**下列最早的公开动作：首个 GitHub release `v0.1.0-preview.2` 2026-10-07T05:20:16Z（北京 13:20:16）；稳定版 `v0.1.0` 2026-10-07T07:07:32Z（北京 15:07:32）。npm：`0.0.1` 2026-10-03T21:30Z；`0.1.0-preview.2` 2026-10-07T03:59:50Z（早于仓库创建，说明 npm 包先于 GitHub 仓库发布）；`0.1.0` 2026-10-07T07:10:37Z。
- 仓库提交历史共约 6041 次（`Link` 头），最早可见 README 提交 2026-06-22（历史是导入的，不等于公开时间）。

### README 逐字（当前版）与核对

| 说法 | README 位置/原句 | 状态 |
|---|---|---|
| 成本总数 | 开头：“It [cost over $420,000](#how-did-this-go) in tokens to do it, but you could probably have done it for ~$20k (see below)” | 已找到。**README 自身不一致**：开头 $420,000，正文 “over $400,000 in API priced tokens with GPT-5.6 Sol and GPT 6 Astra” 另加 “~$24,047 of API spend”（见下），未说明差额如何构成 |
| 40 万美元 | “In total I did **over $400,000 in API priced tokens with GPT-5.6 Sol and GPT 6 Astra**. They wrote over 1.3m lines of Rust over multiple months of /goal loops and never got past like 84% compat.” | 已找到。限定词：**“API priced”（按 API 标价折算，不是实付现金）**、OpenAI 模型、数月 /goal 循环、“like 84% compat” |
| 10 小时 v0 | “When I saw how little my Claude Code limits were burning, I figured it'd be fun to throw Opus 5.5 at this. It had a working v0 in 10 hours.” | 已找到 |
| 2 周 / 约 2.4 万美元 | “Total token spend was **~$24,047 of API spend over 2 weeks**. I was using my Claude accounts, and it worked out to somewhere between **925% and 983% of my $200 plan weekly limits**.” | 已找到。限定词：API 标价；实际走订阅账号额度 |
| 从零开始 | “Opus 5.5 started from scratch. It got further than Astra in 1/10th the time.” | 已找到（作者自述） |
| 从未读过代码 | “Also worth mentioning: I've never read a line of this code.” | 已找到 |
| 100% 兼容 | “This is an early release. It has 100% compatibility in every real world project we have tested. It should work as a drop in replacement for the vast majority of apps. See [Known problems].” | 已找到，但**该句 10-07 23:23Z 才加入**（提交 `26f69b80`：此前写 “It is not yet a full replacement for `tsc` in every project”；`26f69b80` 的措辞是 “… in every project, but it has 100% compatibility in every real world project we have tested”；次分钟 `e8993f51` 删去“not a full replacement”半句）。限定词：“real world project we have tested” |
| 兼容性依据 | Status：“TanStack Query core and Hono check with diagnostics identical to Go's. All 181,711 ported Go tests pass.” “On 120 open-source repos, the command-line output differs from Go's only in the problems below and where Go's own output changes from run to run.” | 已找到。**“100%”与“differs … only in the problems below”并存**，Known problems 有 4 条 |
| Known problems（摘要） | ① 某些 monorepo 工作区包源文件经 node_modules 和直接导入两条路径可达时，tsc-rs 可能对更多文件写输出并报 TS6059；② `tsc -b` 中项目 A 导入项目 B 的输出而无 project reference 时，tsc-rs 在少数情形读到旧的或缺失的输出（TS2305/TS2307）；③ 编辑器内存在长编辑会话中缓慢增长（约每 1,000 次编辑 20 MiB；最长测过 2,190 次）；④ `tsc-rs --version` 打印所移植的 TS 版本（7.1.0-dev）而非 npm 版本 | 已找到 |
| 基准 | “On 60 open-source projects, type checking takes about half of Go's time (geometric mean).”（6 个应用表：tsc-rs 几何平均 11.4×对 tsc 6；`bun check` 20.9×） | 已找到（作者自测；Apple M4 Pro；`tsc-rs` 0.1.0） |
| 平台 | “Platforms: Linux x64 (static, any distribution) and macOS arm64. Windows and Linux arm64 are not available yet.” | 已找到 |
| 作者身份 | 见上：org `pingdotgg`、提交署名 Theo Browne / t3dotgg | 部分支持（README 未署名，署名在提交里） |

README 版本演进（`c-ts-rust-readme-*.md`）：
- `adb41aac`（10-06 22:48Z）：“This is a preview. It is not yet a full replacement for `tsc`.”
- `934345c4`（10-07 06:22Z）：“This is an early release. It is not yet a full replacement for `tsc` in every project.”
- `c1e5e5cd`（10-07 08:21Z）起出现 “$420,000”“$400,000”“84% compat”“$24,047”“925% and 983%”“I've never read a line of this code”。
- `26f69b80`（10-07 23:23Z）/ `e8993f51`（23:24Z）：加入并改为 “100% compatibility in every real world project we have tested”。
- `9b9c101c`（10-08 01:14Z）、`fbb9847e`（10-08 14:07Z）：更新 Known problems。

**建议措辞（C2，41 字）**：作者自述：LLM把TypeScript编译器移植到Rust，API标价超40万美元
（备选：作者自述 Opus-5.5 约两周、按 API 标价约 2.4 万美元，从零写出可用版本。注意“100% 兼容”不要单独上图：它出现在“早期版本”“已知问题”语境中。）

---

## C3　11 个正方形最优装箱的 Lean 证明

- 一手来源：https://github.com/Queuingtheorydotcom/11SquaresFormalized （README `c-11sq-readme.md`；`docs/VERIFICATION_20261006.md` → `c-11sq-verif.md`；`ACKNOWLEDGEMENTS.md`、`PROVENANCE.md`、`MISSING.md`、`AGENTS.md`、`PC_RESUME.md`、`docs/PUBLICATION.md` 同目录 `c-11sq-*.md`；`verification/completed-run-20261006/{summary,independent-review,provenance}.json` → `c-11sq-run-*.json`；仓库元数据 `c-11sq-repo.json`、提交 `c-11sq-commits.json`、精简目录树 `c-11sq-tree.json`）。取回 2026-10-09 09:4x 北京。
- 时刻：仓库创建 2026-09-30T15:34:33Z（北京 09-30 23:34）；集成提交 `a1718f7d`“Integrate verified optimality proof and credit project contributors” 2026-10-06T05:17:05Z（北京 10-06 13:17:05）；最近推送 10-06T05:23:46Z（北京 13:23）。提交署名为通用身份 “Square Packing Contributors”。上游“计算机辅助证明”仓库 `Queuingtheorydotcom/11SquaresOptimal` 创建于 2026-09-29T02:52:57Z（北京 09-29 10:52）。HN 帖 2026-10-07T14:10:55Z（北京 10-07 22:10）。

### 逐字核对

| 说法 | 原句（README，除注明外） | 状态 |
|---|---|---|
| 7,920 个本地 Lean 模块、零 admission | “The completed EvolvingPrograms verification run accepted all **7,920 local Lean modules**, and its final audit reports **zero admissions**.” | 已找到（项目自述）。**该验证运行在 `EvolvingPrograms/11SquaresEvolving`，仓库与 Actions 页对匿名访问均 404（私有）**；`VERIFICATION_20261006.md`：“Repository permissions may be required to view the original Actions run and its downloadable artifacts.” 第三方登记册 jlevy/squares 亦写 “That run is private and recorded as reported, so it sets no rung”。仓库内留有可公开的摘要证据（`summary.json` 等） |
| native_decide | “Selected expensive, exact numerical certificate checks use `native_decide`. Geometry, checker soundness, and proof assembly retain ordinary Lean proofs. Consequently the final theorem trusts **Lean's kernel and native compiler**; this is not a kernel-only verification claim.” | 已找到 |
| 边长 | “The construction attains approximately `3.8770835900228141773`.” 其中 T = (6u+4)/(1+2u−u²)，u 为 (9/25, 37/100) 内 5u⁸−10u⁷−2u⁶+14u⁵+12u⁴−6u³+2u²+2u−1=0 的唯一根 | 已找到 |
| 状态标记 | `summary.json`：“status”: “OPTIMALITY_PROVED_WITH_NATIVE_CERTIFICATES”、“trust_model”: “lean_kernel_and_native_compiler”、“checked_modules”: 7920、“explicit_admissions”: 0、“audited_theorem_targets”: 2234、“native_certificate_axioms”: 13308 | 已找到 |
| 本次集成未独立复算 | `independent-review.json`：“independent_lean_recompilation”: false、“compiled_object_bytes_independently_rehashed”: false；VERIFICATION 文：“It audits the successful run's evidence rather than claiming a second independent compilation.” “The successful run reused validated receipts; its elapsed duration is not a cold full-build benchmark” | 已找到（限定词，重要） |
| 同行评审 | 上游 `11SquaresOptimal/README.md`：“This is a computational certificate argument … It is not a completed Lean/formal proof, and publication here does not constitute independent peer review.”（对上游计算机证明）。11SquaresFormalized 未提评审 | 已找到 |
| HN 标题 “AI-assisted” | HN 帖 49993121 标题 “AI-assisted proof of optimal packing for 11 squares”，发帖人 bluepeter | 标题是**发帖人所加**，仓库未写 |

### AI 参与方式（要求项）——**仓库自身未写明**
- 我 grep 了 11SquaresFormalized（README、ACKNOWLEDGEMENTS、PROVENANCE、MISSING、AGENTS、PC_RESUME、SIMPLIFICATION_HANDOFF、PUBLICATION、VERIFICATION）与 11SquaresOptimal（README、PROOF、PUBLICATION、REPRODUCING、paper/README）的全部 Markdown：**没有出现任何模型名（Astra、Claude、GPT、Codex）**。仅有间接痕迹：`AGENTS.md` 存在（“Formalization rules … The user-approved exception is native_decide …”）、分支名 `codex/native-numerical-certificates-20261004`、`codex/simplification-unverified-20261003`（`PC_RESUME.md`）。仓库致谢列的是人和项目：EvolvingPrograms/@ctjlewis（验证基础设施）、@wand125、Benjamin Gurevitch（T03）、@Julian-JJ（HPC 集群）、@Guzhou0806、@Queuingtheorydotcom；Walter Trump（装箱）。
- 第三方说法（L5/L6，仅线索）：jlevy/squares 登记册（https://jlevy.github.io/squares/cases/11.html，存 `c-jlevy-11.html` / `.txt`）T-060 条目写：“Queuingtheorydotcom, 11SquaresOptimal, Astra-assisted work building on Squares Project and Kleddamag.” “One adversarial AI review is retained and mapped, by GPT-6 Astra at max reasoning; it is not a human oversight record.” 注明该证明“announced as Astra-assisted work”（登记册转述，原始公告未找到）。Startup Fortune（页面署 Oct 6, 2026 8:46 PM，时区未给）与 aiweekly 称“仓库/公告把 OpenAI 的 Astra 与 Anthropic 的 Claude 列为直接贡献者”，**与仓库现有文本不符**（仓库没有这两个名字）；Reddit r/mathematics、r/singularity 标题写 “using Astra and Claude”（Reddit 对 curl 返回 403、firecrawl 不支持，未读正文）。HN 评论 fwip：“The readme also appears to be entirely LLM-written.”（L6 线索）。
- 结论：**“谁、用什么模型做了什么”无一手原句**；状态“部分支持”。
- 第三方对 Lean 构建的复核：登记册在 T-060 条目写 “V5 needs … a complete build of the 11SquaresFormalized proof … The source's own run is private and recorded as reported”——即截至登记册快照，没有第三方公开复跑；同时登记册对原证明做了“复现实现 + 数学审计”（V3/C3，非 Lean 层）。

### 此前已知结果（来源：jlevy/squares 登记册，L5；Friedman 页取回失败）
- 上界（即已知最优装箱，边长 ≈ 3.877084）：Walter Trump，1979（登记册 T-011：“Trump's 1979 packing is exactly valid, so s(11)≤3.877083590022814…”）。
- 下界链：Stromquist 2+4/√5 ≈ 3.7889（1984 备忘，2003 发表）；Levy 3.81（2026-09-04，T-018）；Kleddamag 严格 s(11) > 31/8 = 3.875（T-037，2026-09-29）；Wang 与 Li s(11) > 3875000000/999999999（T-061，2026-09-30）。
- 所以“最优性”此前是**未证开放问题**（装箱已知，下界与之差 0.0001 量级）；Queuingtheorydotcom 的计算机辅助证明（2026-09-29，T-060）给出等式，Lean 化于 10-06。

**建议措辞（C3，34 字）**：项目称11个正方形最优装箱获Lean证明，信任编译器，验证运行未公开
（更稳的写法：项目称，11 个单位正方形最小装箱边长的 Lean 形式化已通过自家验证；该运行未公开、证明依赖 Lean 编译器。“AI 辅助”不要写：一手无原句。）

---

## C4　OpenAI 年化收入口径

- 一手来源（L4）：CNBC，Ashley Capoot 与 Kate Rooney，“Nvidia, Oracle, CoreWeave and other AI stocks sink on OpenAI revenue report”，https://www.cnbc.com/2026/10/08/open-ai-revenue-nvidia-oracle-coreweave.html 。存 `c-cnbc.html`（本地）、`c-cnbc.txt`。
- 时刻：datePublished 2026-10-08T18:14:54Z = 北京 10-09 02:14:54 = 美东 10-08 14:14:54（与 WorkBuddy 的 “14:14 EDT” 一致）；dateModified 21:19:14Z（北京 10-09 05:19）。
- CNBC 逐字（正文）：
  - “OpenAI told investors that it hit roughly $50 billion in annualized revenue at the end of September, CNBC confirmed, lower than the the $68 billion figure that was widely reported late last month. A person familiar with the matter said the $68 billion figure included gross revenue from OpenAI's partners, which helps investors make a more direct comparison with its chief rival, Anthropic.”
  - “The Financial Times was first to report the $50 billion figure.”
  - “OpenAI shared an update about its finances in an investor presentation, said the person, who asked not to be named … In addition to the $50 billion in annualized revenue, OpenAI touted 77% total run rate growth during its third quarter, as well as 107% run rate growth for its enterprise business during the same period”
  - “Nvidia shares fell 3%, Oracle shares fell nearly 6% and CoreWeave shares slipped nearly 8% on Thursday.”
  - “In August, Anthropic told investors that its annualized revenue run rate hit $65 billion at the end of July.”（对照项）

### 680 还是 700：**以 CNBC 原句为准 = 680 亿（$68 billion）**
- CNBC：$68 billion（“widely reported late last month”）。
- “700 亿”来自别家：FT 页标题副题（搜索摘要，付费墙，未读正文）：“far short of the $70bn reported by the FT and other media outlets late last month”；Yahoo Finance 转述 FT：“well below the $70 billion outlets previously estimated”；Reuters 转 Axios（2026-09-29）标题 “OpenAI's annualized recurring revenue nears $70 billion”。→ ChatGPT 扫描的“700 亿”对应 FT/Axios，**不是 CNBC**；二者并存，按“同一指标不同来源不一致”全部记录。
- **对差异的解释，两家说法不同**：CNBC（匿名知情人）：68 含合作伙伴毛收入（gross revenue from OpenAI's partners）。FT（经 Yahoo 转述，L5）：投资者把 8 月约 400 亿的估计加上 OpenAI 所称 70% 增长，得出 700 亿，且口径与 Anthropic 不同（“OpenAI and Anthropic calculate their annualized revenue differently”）。HN 评论里贴了 FT gift link（item 50008187）——付费墙不绕，**未使用**。
- OpenAI 官方回应：CNBC 称 OpenAI “told investors”，来源是匿名知情人；**未找到 OpenAI 公开声明**（OpenAI 官网、新闻稿、署名员工）。FT 摘要称 9 月的 70 亿口径 “a number the company did not deny”（FT，转引）。

| 说法 | 级 | 来源 | 原句 | 状态 |
|---|---|---|---|---|
| OpenAI 告知投资者 9 月底年化收入约 500 亿美元 | L4 | CNBC（上） | “roughly $50 billion in annualized revenue at the end of September, CNBC confirmed” | 已找到（媒体报道，匿名知情人；非官方发布） |
| 此前外传约 680 亿（CNBC 口径） | L4 | CNBC | “the $68 billion figure that was widely reported late last month” | 已找到；FT/Axios 写约 700 亿 |
| 差异因是否计入合作伙伴毛收入 | L4（匿名） | CNBC | “A person familiar … said the $68 billion figure included gross revenue from OpenAI's partners” | 部分支持（仅一位匿名人士；FT 的解释不同） |
| FT 首发 | L4 | CNBC | “The Financial Times was first to report the $50 billion figure.” | 已找到 |
| OpenAI 官方回应 | —— | —— | —— | 未找到一手来源 |

**建议措辞（C4，39 字）**：据CNBC，OpenAI称九月底年化收入约500亿美元，低于此前外传的680亿
（注意：“OpenAI称”在 CNBC 里是“OpenAI 告知投资者，CNBC 证实”；如不放心，可改“据 CNBC 报道，OpenAI 告知投资者……”，字数会超。口径差异另放，不上图。）

---

## C5　Claude Dashboards 与 Motion

- 一手来源（L1）：https://claude.com/resources/articles/dashboards-and-motion 。存 `c-claude-dash.html`（本地）、`c-claude-dash.txt`。取回 2026-10-09 北京（curl 200）。
- 时刻：页面 `Date October 8, 2026`，meta `article:published_time` 仅 “2026-10-08”，**官方未给时刻**。HN 提交（非官方）10-09 03:16 北京。
- 标题：“Build live dashboards and animate explainers with Claude”。
- 逐字：
  - 副标题：“Claude Dashboards and Claude Motion are now in beta. Docs, Slides, and Design are out of beta and on every Claude plan, including Free.”
  - “Claude Dashboards connects to a data platform like BigQuery, Databricks or Snowflake, or a CRM tool like Salesforce, and is in beta on paid plans. Claude Motion is in beta on Team and Enterprise. Claude Docs, Slides, and Design are out of beta and available on every plan, including Free.”
  - 数据源（Dashboards）：“Connect your company’s data platform, such as Amazon Redshift, BigQuery, ClickHouse, Databricks, or Snowflake … It works with your other connectors too. For example, you can ask for a dashboard of your Salesforce opportunities.”
  - 可送往的分析工具：“Amplitude, Grafana, Hex, Mixpanel, Omni, Perplexity, PostHog, or Sigma … with Looker, monday.com, and Tableau coming soon.”
  - Motion 不是视频生成：“It doesn’t use a video generation model, so there’s no generated footage and no AI-generated people.” “Claude writes code that animates your text, charts, shapes, and images … then download it as an MP4 file.”
  - Design 独立站：“Claude Design started at its own URL, claude.ai/design. … we’re folding the standalone site into Claude. It stays at claude.ai/design until December 14, so you have time to move over.” “Chats with Claude and comments on your projects stay in the standalone version and won't be available after it closes. Public links to standalone projects stop working then too.”
  - Enterprise：“For Enterprise admins: Dashboards and Motion are off by default, and you can turn them on in Organization settings > Artifacts. Docs, Slides, and Design turn on by default October 15, or you can turn them on today.”
  - 使用量：“people have made more than 45 million docs, decks, and designs in Claude”。
- 与扫描一致处：Dashboards beta 适用付费计划 ✓；Motion 仅 Team/Enterprise beta ✓、不是视频生成 ✓；Docs/Slides/Design 结束 beta 且含 Free ✓；Design 独立站 12/14 关闭 ✓（页面写 “stays … until December 14”，“closes” 由上下文）。**页面未列出“付费计划”具体是哪几档**（只写 “paid plans”）——不要补。

| 说法 | 级 | 状态 |
|---|---|---|
| Dashboards：付费计划 beta；数据源含 BigQuery、Databricks、Snowflake、Salesforce 等 | L1 | 已找到 |
| Motion：仅 Team 与 Enterprise beta；非视频生成模型 | L1 | 已找到 |
| Docs、Slides、Design 退出 beta，所有计划含 Free | L1 | 已找到 |
| Claude Design 独立站 12 月 14 日前保留 | L1 | 已找到 |

**建议措辞（C5，46 字）**：Claude称Dashboards付费版公测、Motion仅限Team与Enterprise（压到 40 字内可去掉“称”并省“仅限”）
（备选单句：Claude称Docs、Slides、Design退出公测，所有计划含免费版可用。）

---

## C6　Anthropic Cyber Mission / OSS Scanner

- 一手来源（L1）：
  - https://www.anthropic.com/news/anthropic-cyber-mission （存 `c-anthropic-cyber.html` / `.txt`）；页面 “Oct 8, 2026”。meta：`article:published_time` 2026-10-08T09:04:16.632Z（北京 10-08 17:04）、`article:modified_time` 2026-10-08T19:00:45Z（北京 10-09 03:00）。两个时刻差 10 小时，**以哪个为发布时刻不能确定**（姊妹研究页 meta published 19:00:00Z）。
  - OSS Scanner 研究页 https://www.anthropic.com/research/launching-opt-in-vuln-finding-service-for-open-source （`c-oss-scanner-research.html` / `.txt`；published 2026-10-08T19:00:00Z = 北京 10-09 03:00；modified 19:01:09Z）。
  - OSS Scanner FAQ https://red.anthropic.com/oss-scanner （`c-oss-scanner-red.html` / `.txt`；页面无时间戳）。
- 逐字：
  - 总述：“Today we’re launching the Anthropic Cyber Mission, a long-term commitment to securing the systems everyone depends on.” “We’re starting with two areas: Critical infrastructure … Open-source software …”
  - OSS Scanner 免费：“Enrolled projects receive periodic scans from our most capable models, free of charge.”
  - 不经人工审核：“The reports are model-generated and sent without human review. That means maintainers receive them faster, but it also means that some will contain inaccuracies, such as a wrong severity rating. We expect a true-positive rate above 90%, and will work to improve the true positive rate and fix quality over time.”
  - **“>90%” 是预期，不是测得**。研究页另有实测：“we asked the expert penetration testers who review our CVD findings to check 97 critical and high-severity vulnerabilities from the scanner across 48 projects. Of these, 85 (88%) met the bar for our CVD process. Of the remaining 12, 11 were real but duplicated known issues or other findings from the scan, and only one was invalid”。
  - 需申请 / 自愿：“OSS Scanner … an opt-in service”；研究页：“Core maintainers of eligible projects can enroll by submitting a PR to this GitHub repo [anthropics/oss-scanner] … Projects are eligible based on a similar set of criteria that OSS-Fuzz uses: briefly, projects should have a ‘critical impact on infrastructure and user security’ and we will make decisions on a case-by-case basis.” FAQ：“we will manually validate you are a core maintainer before enrolling each project.” 并写：“This service is built for projects that are already able to keep up with verified high/critical vulnerability reports.”
  - 披露政策（FAQ）：“We will not place any form of 90-day coordinated disclosure period on these unvalidated findings.”
  - 关键基础设施方案（CIDP）：“Its founding partners are Accenture, Booz Allen, CrowdStrike, Deloitte, Dragos, Hitachi, Insane Cyber, Nozomi Networks, Palo Alto Networks, PwC, and Rockwell Automation.”（11 家）；“Our first step is to work with a small cohort of providers”；“Several partners are currently working with Claude to fix vulnerabilities and help customers do the same.” 登记兴趣表单，非开放注册。
  - 其它数字（本页专有，**勿与 1007 期已发 CVP 的 12.9 万 / 5,500 混**）：研究页 “We have discovered over 29,000 candidate vulnerabilities, but have only been able to manually review and triage approximately 6,000”；“we have sent nearly 5,000 reports directly to maintainers after they asked to receive everything we had—even if it wasn’t validated”。
  - 与 1007 的关系：本页写 “Earlier this week, we merged Project Glasswing into our expanded Cyber Verification Program”，指向 1007 期已发的 CVP 公告（10/6）；本页是新的 Cyber Mission。
- 与扫描一致处：免费 ✓、需申请（实为 opt-in + 提 PR + 人工核实维护者身份 + 逐案审批）✓、报告不经人工审核 ✓、“>90%”为预期 ✓、合作方名单 ✓。

| 说法 | 级 | 状态 |
|---|---|---|
| OSS Scanner 对开源项目免费，需自愿加入（核心维护者提 PR，逐案审批） | L1 | 已找到 |
| 报告为模型生成、发送前无人工审核 | L1 | 已找到 |
| 预期真阳性率 >90% | L1 | 已找到（“We expect”，是预期）；实测 88%（85/97）另列 |
| CIDP 创始合作方 11 家 | L1 | 已找到 |

**建议措辞（C6，39 字）**：Anthropic称OSS Scanner免费、项目自愿加入，报告不经人工审核
（题材提示：不出现“政府”。主体只写“关键基础设施”“开源”。若放预期值：“预期真阳性率超 90%”需带“预期”。）

---

## C7　Whistle：16.9 MB 离线语音识别

- 一手来源（L1，厂商自述）：https://cactuscompute.com/blog/whistle （存 `c-whistle.html`（本地）、`c-whistle.txt`）；Hugging Face 模型卡 https://huggingface.co/Cactus-Compute/whistle （`c-whistle-hf.json`、`c-whistle-hf-readme.md`）。
- 时刻：博文署 “October 2, 2026”（meta published_time “2026-10-02”，官方未给时刻），作者 Jakub Mroz、Henry Ndubuaku；HF 模型创建 2026-09-30T17:13:26Z（北京 10-01 01:13）、最后修改 10-02T07:27:29Z（北京 15:27）。许可：Apache-2.0（HF 卡元数据）。HN：10-03 首次有人提交（低分），10-08 的热帖提交于 2026-10-08T16:59:39Z（北京 10-09 00:59）。
- 逐字：
  - 副题：“An open speech recognition model that runs on the same CPU engine as Needle. It transcribes seven languages, reaches the first token in 11 ms, and loads beside Needle so one binary turns a clip straight into tool calls.”
  - 语言与时长：“Transcription. 16 kHz mono audio, up to 30 seconds in one pass, in English, German, French, Spanish, Italian, Dutch and Polish. The language is detected unless you name it.” 网页演示：“Up to 30 seconds in English, German, French, Spanish, Italian, Dutch or Polish. The first press downloads the 16.9 MB model, and audio never leaves your device.” → **30 秒是模型单次处理上限**（不只是网页演示）；**无中文**（七种语言清单）。
  - 11 ms 的条件：“Time to first token … Whistle 11.1 ms; Whisper base 73.2 ms; Moonshine tiny v2 22.8 ms … Ten seconds of audio on an Apple M4 Pro CPU … Whistle's tracks the clip: 5.9 ms at 5 seconds, 11.1 ms at 10, 36.3 ms at 30.” HF 卡：“10 s of audio on an Apple M4 Pro … Whistle's C++ engine at 5 beams … Precision: Whistle 2 to 4 bit, Whisper fp32 in memory on the CPU, Moonshine int8.”
  - 精度对比的限定词：“Whistle's are measured over 86,174 utterances. Whisper's and Moonshine's are the figures their authors published, from the multilingual checkpoints rather than the English-only ones.”（对手数字不是同批复测）“Whistle is ahead on LibriSpeech test-clean and test-other, on SPGISpeech, on Earnings-22 and on the FLEURS average. Whisper base is ahead on TED-LIUM, on AMI and on the MLS average, at 145.3 MB against 16.9.”（厂商自报）
  - 体积：16.9 MB（Whisper base 145.3 MB；Moonshine tiny v2 41.9 MB）。
  - 无训练数据泄漏的自证：“No test audio appears in Whistle's training or validation data, verified by comparing audio checksums and speaker IDs across every reported test set.”
- 扫描对照：10/2 发布 ✓；7 种语言（英德法西意荷波）✓；无中文 ✓；“网页演示约 30 秒”实为模型上限；11 ms 为 10 秒音频、M4 Pro CPU、5 beams 下的首 token 时间 ✓。

| 说法 | 级 | 状态 |
|---|---|---|
| 单文件 16.9 MB，CPU 运行 | L1 | 已找到 |
| 支持英、德、法、西、意、荷、波七种语言，单次最长 30 秒 | L1 | 已找到 |
| 首 token 11 ms | L1（厂商自测） | 已找到（10 秒音频、Apple M4 Pro CPU；30 秒音频为 36.3 ms） |
| 与 Whisper-base、Moonshine-tiny-v2 对比 | L1（厂商自测，对手为论文公布值） | 已找到 |

**建议措辞（C7，33 字）**：Cactus称16.9MB语音识别模型仅支持英德法西意荷波七种语言
（备选：Cactus称，16.9MB语音识别模型在M4 Pro的CPU上处理10秒音频，首token约11毫秒（自测）。）

---

## C8　USA Today 起诉 OpenAI

- 一手来源：
  - 起诉状（法院文件，L1 原始文书）：DocumentCloud https://www.documentcloud.org/documents/28731453-usa-today-v-openai/ → PDF https://s3.documentcloud.org/documents/28731453/usa-today-v-openai.pdf （存 `c-usatoday-complaint.pdf`（本地，4.2 MB）、`c-usatoday-complaint.txt`、`c-usatoday-complaint-dc.html`）；路透链到同一文件的 Thomson Reuters 副本 `tmsnrt.rs/4hMudzE` → `fingfx.thomsonreuters.com/gfx/legaldocs/movarymwypa/USA TODAY OPENAI COPYRIGHT lawsuit.pdf`（301 重定向，200 application/pdf，**未另存**，内容未比对）。**能公开取得，已存档。**
  - Reuters（L4）：https://www.reuters.com/legal/legalindustry/usa-today-sues-openai-copyright-infringement-over-ai-training-2026-10-08/ ，Blake Brittain。curl 返回 401，用 firecrawl 取回（返回完整正文，无付费墙提示），正文存 `c-reuters-usatoday.txt`。meta published 2026-10-08T15:39:17.669Z = **北京 10-08 23:39:17**（EDT 11:39；与 WorkBuddy 的“15:39 UTC”一致，注意换算后是 10-08 晚而非 10-09），modified 15:39:44Z。
- 诉状要点（页码为 PDF 页）：
  - 首页：“Case 1:26-cv-08892 Document 1 Filed 10/08/26 Page 1 of 79”，UNITED STATES DISTRICT COURT SOUTHERN DISTRICT OF NEW YORK；原告 14 家（USA TODAY Co., Inc. 等，“who are all owned by USA TODAY Co., Inc.”），被告 7 家（OpenAI Foundation；OpenAI GP, LLC；OAI International, Inc.；OpenAI OpCo, LLC；OpenAI Global, LLC；OAI Corporation；OpenAI Group PBC）；“JURY TRIAL DEMANDED”；代理律所 Rothwell, Figg, Ernst & Manbeck, P.C.。
  - ¶13（p.5）：“In this lawsuit, the USA TODAY Plaintiffs seek damages in excess of $250 million. Upon information and belief, OpenAI’s models have copied hundreds of thousands of articles and other materials from the USA TODAY Publications. The law provides that the USA TODAY Plaintiffs may recover up to $150,000 for each willful copyright infringement, plus up to $25,000 per violation for OpenAI’s stripping of copyright management information.”
  - 诉由：Count I 版权侵权（17 U.S.C. § 501，p.74）；Count II 替代侵权（Vicarious Copyright Infringement，p.76）；Count III DMCA 去除版权管理信息（p.76）。
  - 诉讼请求（p.78，Prayer for Relief）：法定损害赔偿、补偿性赔偿、返还等；声明侵权；永久禁令；“Ordering destruction under 17 U.S.C. § 503(b) of all GPT or other LLM models and training sets that incorporate the USA TODAY Plaintiffs’ content”；费用与律师费。
- 路透逐字：“USA Today Co (TDAY.N) and several newspapers it owns sued OpenAI in Manhattan federal court on Thursday for allegedly infringing their copyrights by using their content to train its large language models.” “Spokespeople for OpenAI did not immediately respond to a request for comment on the complaint.” “USA Today requested damages ‘in excess of $250 million’ and a court order blocking OpenAI's alleged infringement in its Thursday complaint.” “The case is USA Today Co v. OpenAI Foundation, US District Court for the Southern District of New York, No. 1:26-cv-08892”
- 限定词：这是**原告指控**；“上亿美元”是“in excess of”的索赔额，“$150,000/次”是法定最高额（不是已主张的数额）；OpenAI 回应：路透发稿时“did not immediately respond”，**其后是否回应未找到**。The Verge（线索，L4）同日跟进，称 “asks for damages of more than $250 million”，链接同一起诉状。

| 说法 | 级 | 状态 |
|---|---|---|
| USA Today Co. 及旗下多家报纸在纽约南区联邦法院起诉 OpenAI（案号 1:26-cv-08892） | L1（法院文书）+ L4 | 已找到 |
| 索赔“超过 2.5 亿美元” | L1 / L4 | 已找到（“in excess of $250 million”，原告主张） |
| 指控复制“数十万篇”文章 | L1 | 已找到（“Upon information and belief … hundreds of thousands”） |
| 提交时刻 15:39 UTC | L4（路透发稿） | 已找到：路透 meta 15:39:17Z；诉状本身只有日期 10/08/26，**未给时刻** |
| 能否取得起诉状 | —— | 已取得（79 页） |

**建议措辞（C8，40 字）**：据路透社，USA Today母公司起诉OpenAI，索赔超2.5亿美元，暂无回应
（“暂无回应”是路透发稿时的状态，需带“据路透社”；也可去掉这半句。母公司为“USA Today Co”，另有 13 家子公司/关联报业为共同原告。）

---

## HN 热度（Algolia，C 组统一取回）

取回时刻：2026-10-09 约 09:55 北京（01:55Z）；脚本 `c-heat.py` → `c-heat.json`（各条所有匹配帖）。分数/评论数为取回当时。

| 条 | HN 帖（item） | 分 / 评论 | 提交时刻（北京） | 标题 / 链接 |
|---|---|---|---|---|
| C1 | 49797999 | 1 / 0 | 09-22 16:07 | Et Tu, Brute? Economic Misalignment in Personal AI Agents（arxiv.org/abs/2609.24927） |
| C2 | 50000676 | 109 / 207 | 10-08 08:46 | Port of the TypeScript compiler, checker and lsp to Rust, by LLM（github.com/pingdotgg/ts-rust；TypeScript 团队成员 DanRosenwasser 在帖下留言称“3 of these ports have popped up in the last week”） |
| C2 | 49994958 | 6 / 1 | 10-08 00:20 | ts-rust: An experimental Rust port of the TypeScript 7 compiler (tsc) |
| C3 | 49993121 | 117 / 54 | 10-07 22:10 | AI-assisted proof of optimal packing for 11 squares（github.com/Queuingtheorydotcom/11SquaresFormalized） |
| C4 | 50008187 | 351 / 246 | 10-09 00:45 | OpenAI annualised revenues $20B less than previously signalled（url 指向 cnbc.com/2026/10/08/…；标题沿用 FT） |
| C4 | 50011505 | 6 / 1 | 10-09 04:15 | 同标题，url 指向 FT |
| C5 | 50010631 | 5 / 0 | 10-09 03:16 | Build live dashboards and animate explainers with Claude（claude.com/resources/articles/dashboards-and-motion） |
| C5 | 50010623 | 4 / 1 | 10-09 03:16 | Anthropic launches dashboard, animation tools for Claude（channelnewsasia.com） |
| C6 | 50010605 | 6 / 0 | 10-09 03:15 | The Anthropic Cyber Mission |
| C6 | 50013453 | 3 / 1 | 10-09 06:41 | OSS Scanner by Anthropic（red.anthropic.com/oss-scanner） |
| C7 | 50008427 | 535 / 120 | 10-09 00:59 | Whistle: Speech to Text in 16.9 MB（cactuscompute.com/blog/whistle；扫描写 513/117，现 535/120） |
| C7 | 49942095 / 49938769 | 5 / 1；2 / 0 | 10-03 | 同文先前提交，低分 |
| C8 | 50009239 | 11 / 0 | 10-09 01:49 | USA Today sues OpenAI for copyright infringement over AI training（reuters.com） |

另存评论树：C2 `c-hn-50000676.json`、C3 `c-hn-49993121.json`、C4 `c-hn-50008187.json`、C7 `c-hn-50008427.json`。
有理有据的社区质疑只作线索（L6）：C7 评论 skolos 称在自家 Echo Show 场景里 170 条消息 Whistle 约 70 条识别正确，对比 Qwen ASR 1.7B 为 168（用户个例，评论者随后称自行调整过 Whistle；未核）；C4 评论 underyx 认为 FT“先报高数再报低数”（观点）；C3 评论 fwip 称 README 疑似全由 LLM 写成。

---

## 扫描说法勘误（C 组）

1. **C1**：①“Quartz/Fast Company 转述”——Fast Company 未找到；找到的是 Quartz（2026-10-07T11:33Z）、Inc.com、Digiko 等。②Quartz 把“封锁 employment 后保险差距 +40%”写成 **GPT-5**；论文是 **GPT-5.5**（122→171）。③Quartz 标题/导语说“AI chatbots like Claude and ChatGPT”，论文是经 API 的默认设置模型 + 自搭 MCP 环境，并非消费者应用；OpenAI 经 Bloomberg 称被测版本与其消费端购物体验不同（Quartz 转引）。④论文 §5.4 p.9 的 “$151 … or demographic” 与 Table 7 不符（论文内部不一致，见 C1-c）。⑤扫描里的“13 个 agent”：论文摘要同样写 13 agents，正文写 13 models。⑥其余数字（198、284、208、21、20、40%、13、8、32.5 万）均能在论文中逐一找到，与转述一致；“198/284”是高/低金融画像推荐均价之差（合成画像、tool-full、意图合并平均），不是“相对基线的涨幅”。
2. **C2**：①README 开头写成本“over $420,000”，正文写“over $400,000 in API priced tokens”，两处并存，扫描只引了后者。②“100% compatibility in every real world project we have tested”这句**是 10-07 23:23Z 才加入**的（此前 README 写“not yet a full replacement for tsc in every project”），且同一 README 的 Known problems 有 4 条。③“never got past like 84% compat”“working v0 in 10 hours”“~$24,047 … 2 weeks”“925% and 983%”“I've never read a line of this code”均逐字存在。④作者：README 不署名；组织 pingdotgg，提交署名 Theo Browne，6035/6037 次提交为 t3dotgg——扫描称 Theo 有提交署名佐证。
3. **C3**：README 里 “7,920 local Lean modules”“zero admissions”“native_decide”“this is not a kernel-only verification claim”“3.8770835900228141773”逐字存在。扫描未提醒的限定词：①验证运行在私有仓库，匿名访问 404；②集成方明写未做独立复算（`independent_lean_recompilation: false`）；③仓库未写明 AI 参与；“AI-assisted”是 HN 发帖人的标题，“Astra / Claude 为直接贡献者”见于 Startup Fortune、aiweekly，**与仓库文本不符**；jlevy/squares 登记册称该证明“Astra-assisted”（转述）。
4. **C4**：①“约 500 亿对约 680 亿”——CNBC 原句确为 $50 billion 对 $68 billion；②ChatGPT 扫描写的“700 亿”不是 CNBC 的数，是 FT / Axios 的 $70bn；③“10/8 14:14 EDT”✓（18:14:54Z）；④“FT 首发”✓（CNBC 原句）；⑤“差异因是否计入合作伙伴毛收入”仅是 CNBC 匿名知情人之说，FT（经 Yahoo 转述）给的是另一种解释（投资者把 400 亿加 70% 外推得 700 亿、口径与 Anthropic 不同），二者未调和；⑥OpenAI 官方回应未找到。
5. **C5**：扫描说法均与页面一致；补充：页面写 “paid plans”，未列具体档位；Enterprise 默认关闭 Dashboards/Motion，Docs/Slides/Design 10 月 15 日默认开启。
6. **C6**：扫描说法一致；补充：“>90%”为 “We expect”，实测为 85/97（88%）通过 CVD 标准（研究页）；“需申请”实为 opt-in + 提 PR + 人工核实维护者身份 + 逐案审批；新闻页两个时间戳相差 10 小时（09:04Z / 19:00:45Z）。
7. **C7**：扫描“HN 10/8 513/117”现为 535/120（提交 10-09 00:59 北京）；“网页演示约 30 秒上限”其实是**模型单次 30 秒上限**，演示同限；“11 ms”为 10 秒音频、M4 Pro CPU。
8. **C8**：扫描说法一致（“15:39 UTC”即路透发稿时间，北京 10-08 23:39；“超 2.5 亿”“纽约联邦法院”均对）。起诉状已取得；原告实为 USA Today Co., Inc. 及其 13 家关联公司（共 14 家）。

## 失败/未能取得

- FT 原文：curl 403（付费墙/拦截），未绕过；仅用搜索摘要中的公开片段（标记为摘要）。Reuters/Axios 9/29 原文：curl 401，仅标题级线索。Bloomberg newsletter：付费墙，未试。
- Reddit（r/mathematics、r/singularity）：curl 403，firecrawl 不支持；未能读到 C3 相关帖的正文。
- `EvolvingPrograms/11SquaresEvolving` 仓库与 Actions 运行页：匿名 404。
- Friedman 的 Packing Center 页（`erich-friedman.github.io/packing/squinsqu/`）：curl 只回 154 字节，不含正文，未再追；改用 jlevy/squares 登记册（L5）。
- 派工单提到的 “Fast Company”：未找到。
- Anthropic 新闻页发布时刻无法确定（见 C6）。
- 无法确认 ts-rust 仓库何时转为公开（API 不给）。

## 体积提示

单个文本文件均 < 1 MB（`c-usatoday-complaint.txt` 124 KB、`c-arxiv-2609.24927v2.txt` 约 100 KB 等）。按规则不入库的本地原件：各 `.pdf`（`c-arxiv-*.pdf` 约 1.8 MB ×2、`c-usatoday-complaint.pdf` 4.2 MB）与 `.html`（`c-cnbc.html` 约 0.8 MB 等）。`c-hn-*.json` 评论树最大约 100 KB。
