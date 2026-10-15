# 1014 期 D 组（速览）取证清单

取证时刻：2026-10-15，北京时间 17:05–17:30（抓取与热度时刻见 `capture-log-d.md`）。
范围：D1–D7 速览条目，不截图。存档文件均在 `docs/2610/1014/sources/`，以 `d-` 开头。
时区约定：官方页只给日期的，写“官方未给时刻”；HN 时刻为 `barking heat` 输出（已换算北京时间）；GitHub API 原始为 UTC，已换算。

| # | 条目 | 状态汇总 |
|---|---|---|
| D1 | REA | 已找到（官方仓库、FAQ、README 原句齐全；山寨仓库只记 URL 与 README 文字） |
| D2 | OpenCode 2.0 | 已找到 |
| D3 | Jev 1.14 | 部分支持（图像、长上下文确在 danger zone；bounding-box 不在；jev-preview 指向 1.14 原文未写） |
| D4 | Grok Voice 3.0 | 已找到（AA 数字、价格、grok-voice-latest 均有原文；“人类偏好与 τ-Voice 并非第一”由 AA 表验证成立） |
| D5 | OpenAI 退役 GPT-4o/4.1 快照 | 已找到 |
| D6 | Google Deep Review | 已找到 |
| D7 | Claude Code Desktop 预览窗口漏洞（SneakPeek） | 部分支持（时间线与修复版本为研究方自述；“研究方评分 8.7”与原文不符；Anthropic 一手公告与 CVE 未找到） |

---

## D1 REA（Reverse Engineer Anything）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 官方仓库 morluto/rea，2026-04-14 创建，MIT | L1 | https://github.com/morluto/rea （API：https://api.github.com/repos/morluto/rea） | GitHub API：`created_at: 2026-04-14T00:51:32Z`，`license: MIT License` | 北京时间 2026-04-14 08:51；API 原始 UTC | 已找到 |
| 星数 254,529、fork 63,948（2026-10-15 08:56 UTC） | L1 | 同上 | 抓取时（2026-10-15 09:10 UTC）：`stargazers_count 254747`，`forks_count 64002` | 数字随时间变动；本期快照以 09:10 UTC 为准，原记 08:56 UTC 数字已被更新 | 已找到（数字已更新，见勘误） |
| 官网 rea.tools：MCP 服务 + CLI，分析二进制、Electron、APK、网页 | L1 | https://rea.tools/ ；https://github.com/morluto/rea | README：“REA connects your agent to tools for inspecting native binaries, JavaScript and Electron apps, .NET assemblies, and websites.”；README 能力表含 “Android APKs” | 能力范围以 README 为准 | 已找到 |
| 官方安装 `npx rea-agents@latest setup` | L1 | https://rea.tools/ ；https://github.com/morluto/rea | rea.tools：“npx rea-agents@latest setup”；README：“npx rea-agents setup” | 两处写法略有差别（有无 `@latest`），均为官方 | 已找到 |
| FAQ：深度本机分析要用 Hopper/Ghidra/IDA 之一 | L1 | https://github.com/morluto/rea （README FAQ） | “Deep native analysis uses one of them. Static JavaScript and .NET inspection work without a native analysis engine.” | 限定词：仅原生分析需要；JavaScript、.NET 静态检查不需要 | 已找到 |
| FAQ：分析在本机，模型提供方数据政策适用于结果 | L1 | https://rea.tools/faq/ | “REA analyzes targets on your machine. Your agent receives the results, and its model provider's data policy applies to those results.” | 限定词原句：结果交给智能体后，适用模型提供方的数据政策 | 已找到 |
| 授权与合法用途 | L1 | https://github.com/morluto/rea （README “Disclaimer”）；https://github.com/morluto/rea/blob/main/DISCLAIMER.md | README：“REA is for lawful research, education, interoperability, software preservation, and analysis of software you own or are authorized to examine.”；DISCLAIMER：“You are responsible for obtaining any required authorization and for complying …”（另有“Local operation does not establish authorization”） | README 原文为 “lawful”，并以“you own or are authorized to examine”限定 | 已找到 |
| 近期 release rea-agents 6.1.0–6.5.0（10/9–10/14） | L1 | https://github.com/morluto/rea/releases （API：`/repos/morluto/rea/releases`） | 6.1.0 2026-10-09T01:08:45Z；6.2.0 10-09T16:51Z；6.3.0 10-09T23:56Z；6.4.0 10-11T03:58Z；6.5.0 10-14T12:32Z | 北京时间：6.1.0 10-09 09:08；6.2.0 10-10 00:51；6.3.0 10-10 07:56；6.4.0 10-11 11:58；6.5.0 10-14 20:32 | 已找到 |
| HN 805/337（2026-10-10，“窗口前”） | — | HN 条目 50028275：https://rea.tools/ | — | HN 10-10 08:37（北京时间，热度时刻 10-15 17:16 快照：805 分 / 337 评） | 已找到 |
| 山寨仓库 `rea-githu/REA` | 非官方 | https://github.com/rea-githu/REA （仅记 URL） | README 文字：“1. Download the x64 zip.”；仓库描述自称 “Official free download. Local zip.” | 创建 2026-10-13T14:40:18Z（北京时间 10-13 22:40），40 星。与官方无关，不是 REA 官方来源。**未下载、未解压、未运行任何文件**；README 文字存于 `d-scam-rea-githu-readme.md` | 与官方无关（仿冒，速览不得引用） |

存档：`d-rea-tools.html` / `d-rea-tools.txt`（官网首页）；`d-rea-faq.html` / `d-rea-faq.txt`（FAQ）；`d-rea-readme.md`（README 原文，raw）；`d-rea-disclaimer.md`；`d-gh-rea-api.html`（GitHub API JSON，原样存）；`d-gh-rea-releases.html`；`d-scam-rea-githu-readme.md`、`d-scam-rea-githu-api.json`。

---

## D2 OpenCode 2.0

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 后台服务默认启动，TUI/Web/桌面共用 | L1 | https://anoma.ly/notes/opencode-2-0/ | “In 2.0 this server now automatically runs as a single background process on your machine. When you open up a TUI, the web interface or the desktop app they're all talking to the same central process.” 前文：“this was not something it took advantage of by default and was mostly used by power users.” | 页面 meta `article:published_time` = 2026-10-12；官方页未给时刻（北京时间日期 10-12） | 已找到 |
| Code Mode | L1 | 同上 | “We've built a minimal JavaScript interpreter in the @opencode/codemode package.” | 同上 | 已找到 |
| 后台子智能体 | L1 | 同上 | “First, we've added background subagents. This allows the primary agent to kick off some work to another one while it continues.” | 同上 | 已找到 |
| HN 587/225 | — | HN 条目 50094994：https://anoma.ly/notes/opencode-2-0/ | — | 热度快照（10-15 17:16 北京时间）：598 分 / 232 评；投稿 10-15 04:48 | 已找到（数字已更新，见勘误） |

存档：`d-opencode2.html`；`d-opencode2.txt`（已清理为通顺文本，见日志）。

---

## D3 Jev 1.14（TypeSafe）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 图像与长上下文放在“danger zone”实验配置 | L1 | https://typesafe.ai/blog/jev-1-14 | TL;DR：“experimental features for our dreamers: images, a new type, and longer context. Enter the danger zone at your own risk.”；“share those dreamer-ready features under “danger zone” flags in our API headers.”；图像：`TypeSafe-Danger-Zone-Images: true`；长上下文：`TypeSafe-Danger-Zone-Long-Context: true` | 页面标注 Oct 14, 2026（官方未给时刻） | 已找到 |
| bounding-box 放在 danger zone | L1 | 同上 | 新类型 box：“It returns a single bounding box - no type errors allowed.”；“We made huge leaps and bounds on this one recently, so we're enabling this only for jev-1.14.0 (spoilers).”——该段无 danger-zone 标志 | 原文 box 只说“only for jev-1.14.0”，没有 danger-zone 头 | 与说法不符（box 不在 danger zone；只限 jev-1.14.0） |
| 可靠性警告 | L1 | 同上 | “But as a warning, we think there is still way too much jaggedness for us to vouch for these for production use cases!”；图像段：“all models (Jev included) perform worse on multimodality over pure text.” | 同上 | 已找到 |
| `jev-preview` 指向 1.14 | L1 | 同上 | “We're also releasing a new version of Jev under jev-preview ! Not meant to be flashy but instead be small, subtle improvements to everything .” | 原文未写 jev-preview 指向哪一版；只说发布“a new version”，并称 jev-latest 一周内更新为 jev-1.14.0 | 未找到一手来源（“指向 1.14”原文未写） |
| HN 281/90 | — | HN 条目 50095353：https://typesafe.ai/blog/jev-1-14 | — | 热度快照 10-15 17:16：283 分 / 91 评；投稿 10-15 05:11 | 已找到（数字已更新） |

存档：`d-jev114.html`；`d-jev114.txt`。

---

## D4 Grok Voice 3.0（xAI 官网 + Artificial Analysis）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| AA Speech-to-Speech Index 86.9（前代 81.3） | L3（AA 原始页）/ L1（xAI 页） | AA：https://artificialanalysis.ai/speech-to-speech ；xAI：https://x.ai/news/grok-voice-3 | xAI：“Grok Voice 3.0 ranks #1 on the Speech-to-Speech Index, with a score of 86.9”；AA 表：Grok Voice 3.0 **86.9**，Grok Voice 2.0 **81.3** | AA 页快照 Date 2026-10-15 09:10 GMT；xAI 页注明 “Source: Artificial Analysis, October 13, 2026”（北京时间 10-13 不适用，为 AA 数据日期） | 已找到 |
| 测试名与分母 | L3 | AA 页 | AA 定义：“Equal-weighted score across Speech Reasoning (Big Bench Audio), Agentic Performance (𝜏-Voice), Arena Preference (Arena Score), and Task Success Rate. Models must have results for all four components to receive an index score.” | 人类偏好项：“Preference Elo is converted into expected preference against an 800-Elo baseline … The rating is frozen when a model first becomes eligible for publication”；𝜏-Voice 的 GPT-Live-1 (Astra, medium) 注明 “based on 1 trial”。**分母（样本数）未在页面读到**，只见 “Charts use average performance across 3 trials where available” | 已找到（分母未核） |
| 价格 $0.08/分钟 | L1 | https://x.ai/news/grok-voice-3 | “available today in the Voice Agent API at $0.08 / min ( $4.80 / hr ) , the same price as Grok Voice 2.0.” | 官方未给时刻（页面日期 Oct 14, 2026）。AA 表对 Grok 3.0 的 price input 列为 “-”，即 AA 未列 | 已找到 |
| `grok-voice-latest` 已指向新版 | L1 | 同上 | “Agents using grok-voice-latest have already been switched over automatically, with no changes needed.” | 同上 | 已找到 |
| 人类偏好项与 τ-Voice 并非第一（WorkBuddy 说） | L3 | AA 页 | Arena Preference Elo：Grok Voice 3.0 **1070**，而 Gemini 3.1 Flash Live Minimal **1100**、Gemini 3.8 Live **1088** 更高；τ-Voice（Agentic Performance）：Grok Voice 3.0 **71.6%**，GPT-Live-1 (Astra, medium) **74.5%**（注明 based on 1 trial） | xAI 页的 τ-Voice 写法为 “comparable with GPT-Live-1 (Astra)”，未称第一 | 已找到（AA 表支持“并非第一”；Astra 的 τ-Voice 只有 1 次试验） |
| 语音推理 98.5% | L1 / L3 | xAI 页：“Big Bench Audio 98.5%”；AA 表 Speech Reasoning 为 “99%” | AA 正文：“Grok Voice 3.0 (98.5%)” | 同一指标两种写法：表格四舍五入为 99%，AA 正文与 xAI 页为 98.5% | 已找到（同源精度差） |

存档：`d-grok-voice3.html`；`d-grok-voice3.txt`；`d-aa-speech.html`；`d-aa-speech.txt`（AA 页，抓取时间 2026-10-15 09:10 GMT 前后，页面是动态榜单）。

注：xAI 页脚为 “© 2026 SpaceXAI LLC”，页头为 “SpaceXSI”；AA 表中的供应商名为 “SpaceXAI”。见勘误。

---

## D5 OpenAI 退役 GPT-4o / GPT-4.1 快照

| 条目 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 公告 | L1 | https://developers.openai.com/api/docs/deprecations | “On October 14, 2026, we notified developers using certain GPT-4 series models of their deprecation and removal from the API on January 14, 2027.” | 公告日 2026-10-14（官方未给时刻）；停用日 2027-01-14 | 已找到 |
| GPT-4o（gpt-4o-2024-08-06，gpt-4o-2024-11-20） | L1 | 同上 | 停用日 January 14, 2027；推荐替代 **gpt-6-luna** | — | 已找到 |
| GPT-4o mini（gpt-4o-mini-2024-07-18） | L1 | 同上 | 停用日 January 14, 2027；推荐替代 **gpt-6-luna** | — | 已找到 |
| GPT-4.1（gpt-4.1-2025-04-14） | L1 | 同上 | 停用日 January 14, 2027；推荐替代 **gpt-6-luna** | — | 已找到 |
| GPT-4.1 mini（gpt-4.1-mini-2025-04-14） | L1 | 同上 | 停用日 January 14, 2027；推荐替代 **gpt-6-luna** | — | 已找到 |
| 微调版 | L1 | 同上 | “Fine-tuned versions of these models follow the same shutdown schedule as their underlying base models.” | 微调版随基础模型同日 | 已找到 |

HN 检索（“OpenAI deprecate GPT-4o API”“GPT-4.1 deprecation”）：0 条命中。

存档：`d-openai-deprec.html`；`d-openai-deprec.txt`。页面另列有 2026-10-01 公告的 GPT-5.3-Codex/5.1/5.4-Nano 等（2027-04-01 停用）及 2026-10-23 的 gpt-4.1-nano 等条目，不在本条范围内。

---

## D6 Google Deep Review

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 免费每周 2 次 | L1 | https://research.google/blog/analyzing-technical-documents-with-deep-review/ | “Every user receives two complimentary document reviews per week. Users with high-volume workflows can connect their Gemini API key for higher usage limits …” | 官方页日期 October 14, 2026（官方未给时刻） | 已找到 |
| 格式：PDF 与纯文本 | L1 | 同上 | “Deep Review will initially support PDFs and plain text as input formats, with support for multi-file inputs coming soon.” | 同上 | 已找到 |
| 不训练、保存一年 | L1 | 同上 | “Documents uploaded to Deep Review are never used to train, fine-tune, or improve Google models. Uploaded documents are stored for one year for the sole purpose of display to the user in the Deep Review UI.” | 同上 | 已找到 |
| 前身在四个会议处理 16,000 多份自愿提交稿件，与正式评审分开 | L1 | 同上 | “… to provide private, pre-submission feedback to authors of more than 16,000 manuscripts who opted in.”；“This feedback was completely decoupled from the official conference review process.”；会议为 STOC 2026、ICML 2026、NeurIPS 2026、ICLR 2027 | 限定词：自愿加入（opted in）；按“PAT”试点，非正式评审 | 已找到 |

HN 3 分 / 0 评（投稿 10-15 03:43 北京时间，热度快照 17:16）。

存档：`d-google-deep-review.html`；`d-google-deep-review.txt`。

---

## D7 Claude Code Desktop 预览窗口漏洞（Noma Labs “SneakPeek”）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|
| 研究方披露：7/4 报告、7/24 首个补丁被绕过、8/17 最终修复 | L2（研究方自述，非 Anthropic 一手，待 Claude 定级） | https://noma.security/noma-labs/sneakpeek-how-the-preview-pane-in-claude-code-desktop-can-exfiltrate-your-data | 时间线：“July, 4, 2026 Initial Report …”；“July 24, 2026 Patch and Retest: Anthropic fixed the vulnerability and requested a retest.”；“July 24, 2026 Bypass Reported: … blob: URL scheme Web Workers bypass the request filter.”；“August 3, 2026 Bypass Confirmed: Anthropic confirmed the fix is incomplete …”；“August 17, 2026 Anthropic issued a fix in Claude Desktop 1.30096.1 and later, delivered via auto-update.” | 时间线只写日期，无时刻；修复版本与时间线均为 Noma 的自述。Anthropic 确认绕过的 8/3 来自 Noma 的转述 | 已找到（Noma 自述；Anthropic 一手未找到，见下） |
| 8.7 的评分方 | — | 同上 | “Anthropic scored Noma Labs' discovery as CVSS 8.7 (High) and treated the impact as equivalent to code execution on the host.”；“They triaged the initial report, scored it CVSS 8.7 (High) using CVSS v4.0, …” | 8.7 是 **Anthropic** 的分诊评分（7/9），不是研究方评分 | 与说法不符（见勘误） |
| 修复版本 Claude Desktop 1.30096.1 起 | 研究方自述 | 同上；第三方：https://generalanalysis.com/guides/claude-desktop-sneakpeek-response （仅线索） | “Anthropic issued a fix in Claude Desktop 1.30096.1 and later” | 版本号来自研究方；Anthropic 发布说明未查到 | 未找到一手来源（Anthropic 侧） |
| 有无 Anthropic 一手公告或 CVE | — | 搜索（firecrawl 一次）：仅见 Dark Reading、generalanalysis 等二手；Noma 原文：“October, 2026 GitHub Advisory and CVE pending publish” | — | 截至抓取，未找到 Anthropic 官方公告或 CVE 编号 | 未找到一手来源 |
| 攻击前提 | 限定词 | 同上 | “a victim clones a public repo and asks Claude to review a file. Claude reads the file, its preview pane renders it automatically, and the embedded malicious payload executes.”；“Because Claude's preview pane loads files above 256KB via file:// URLs …” | 限定词：需受害者克隆公开仓库并让 Claude 审阅文件；研究方测试条件 | 已找到 |

HN 3 分 / 0 评（投稿 10-15 13:02 北京时间，热度快照 17:16）。

存档：`d-noma-sneakpeek.html`；`d-noma-sneakpeek.txt`。

---

## 扫描说法勘误

1. **D7 CVSS 8.7 的评分方**：扫描线索（“CVSS v4.0 8.7 为研究方评分”）与原文不符。Noma 原文写 “Anthropic scored … CVSS 8.7 (High) using CVSS v4.0”，是 Anthropic 7/9 的分诊评分。
2. **D3 bounding-box 不在 danger zone**：线索说图像、bounding-box、长上下文都在 danger zone。原文只有图像（`TypeSafe-Danger-Zone-Images`）与长上下文（`TypeSafe-Danger-Zone-Long-Context`）有 flag；box 段写的是 “enabling this only for jev-1.14.0”，无 danger-zone 头。
3. **D3 jev-preview 指向 1.14**：线索称原句可证。原文只写 “a new version of Jev under jev-preview”，未写指向 1.14.0。
4. **D4 供应商名**：xAI 页页头为 “SpaceXSI”，页脚为 “© 2026 SpaceXAI LLC”，AA 表的供应商为 “SpaceXAI”。正文若写“xAI”，需由 Claude 判断（扫描与线索用的是 xAI）。
5. **D4 “人类偏好与 τ-Voice 并非第一”**：成立，但依据 AA 表（Gemini 3.1 Flash Live Minimal Elo 1100 > 1070；GPT-Live-1 Astra τ-Voice 74.5% > 71.6%，后者仅 1 次试验）。xAI 页自己没有称第一。
6. **D4 语音推理 98.5% 对 99%**：AA 表格取整为 99%，AA 正文与 xAI 页为 98.5%，属同源精度差，不是矛盾。
7. **D1 星数与 fork**：线索 254,529 / 63,948（08:56 UTC）；抓取 09:10 UTC 为 254,747 / 64,002。数字已动，正文若引用须带时刻。
8. **D1 安装命令写法**：rea.tools 为 `npx rea-agents@latest setup`，README 为 `npx rea-agents setup`，均为官方。
9. **D1 山寨仓库**：`rea-githu/REA` 的 README 含 “Download the x64 zip.”，描述自称 “Official free download”，创建于 2026-10-13 北京时间 22:40。与官方仓库无关，速览不得引用。
10. **D2、D1 热度数字**：HN 数字已动（D2 587/225 → 598/232；D3 281/90 → 283/91；D1 805/337 不变）。
11. **D5、D6：无不符**。D5 另有 2026-10-23 与 2027-04-01 的其他退役条目，不在本条范围。

---

## 未取证或未完成

- Reddit：三条查询（REA、OpenCode 2.0、Jev）均被重定向到登录页，`barking heat` 未取到数据。
- D5 的 HN 热度：0 条命中（不是抓取失败）。
- D4 AA 的样本数（分母）未在页面读到。
- D7 的 Anthropic 一手公告、CVE 编号：未找到。
- “可能的吠点”与高赞评论：D 组不要求，未做。
