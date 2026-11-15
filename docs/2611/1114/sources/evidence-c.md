# 1114 期 C 组（速览）取证

取证时刻：2026-11-15 20:54–21:10（北京时间）。存档：`barking fetch`（html、headers、txt，见 `c-c*-*.{html,headers,txt}`），日志见 `c-fetch-log.jsonl`；过程与失败见 `capture-log-c.md`。热度：`c-heat.json` 与 `c-hn-<id>.json`（HN 三条，抓取时刻 2026-11-15 20:55 北京）。截图：本组不需要。

级别与状态用词按派工单与 `EDITORIAL.md`“事实分级”。北京时间由原时区换算，官方只给日期的写“官方未给时刻”。

## 一、速览条目

| # | 一句事实（原文措辞） | 级 | 一手来源 | 存档 | 发布时刻 | 状态 |
|---|---|---|---|---|---|---|
| C1 | Google 称 /teamwork-LEAP “fully autonomously produced an end-to-end, machine-checked formalization of the Weil conjectures”，约 180 万行 Lean，28,800 次子代理调用 | L1 | https://research.google/blog/advancing-science-engineering-with-antigravitys-teamwork-feature/ | `c-c1-antigravity-teamwork.*` | 2026-11-13（官方未给时刻；署名 Vahab Mirrokni） | 已找到（厂商自述） |
| C2 | Project Zero 称自 2026-11-13 起披露政策为 “Full 90”（报告后 90 天披露），2027-02-02 起 “Full 60”，2028-01-03 起 “Full 30” | L1 | https://projectzero.google/2026/11/disclosure-policy.html | `c-c2-projectzero-disclosure.*` | 2026-11-13（官方未给时刻；署名 Natalie Silvanovich） | 已找到 |
| C3 | Google 称 Gemini 应用在美国所有用户推出 Google Wallet 连接，仅可读取、不能移动或投资资金 | L1 | https://blog.google/innovation-and-ai/products/gemini-app/google-wallet-gemini-app/ | `c-c3-gemini-wallet.*` | 2026-11-13T17:00:00Z = 2026-11-14 01:00（北京） | 已找到 |
| C4 | DeepSeek 更新日志（2026-11-13）：把联网搜索、文件输入、URL 读取、模型安全等能力经现有 API 参数开放给开发者，deepseek-flash 支持三套 API | L1 | https://api-docs.deepseek.com/updates （中文 https://api-docs.deepseek.com/zh-cn/updates） | `c-c4-deepseek-updates.*`、`c-c4-deepseek-updates-zh.*` | 2026-11-13（官方未给时刻） | 已找到 |
| C5 | EFF 观点文（Thorin Klosowski）：“They ignore every bystander you interact with and their inability to actually opt out.” | L1 形式，内容为倡议组织观点 | https://www.eff.org/deeplinks/2026/11/tech-companies-are-trying-shift-entire-concept-privacy-again | `c-c5-eff-privacy.*` | 2026-11-13 13:29:31 −08:00（PT）= 2026-11-14 05:29:31（北京） | 已找到（观点，写成“EFF 认为”） |
| C6 | Ars Technica 报道：OpenAI Foundation 董事 Paul Christiano 在多伦多讲座称 AI 事故可能是“在场多数人最可能的死因”（引语） | L4（Ars）；讲座与头衔 L1 | Ars：https://arstechnica.com/ai/2026/11/ai-is-the-most-likely-reason-youll-die-openai-board-member-warns/ ；讲座：https://hintonlectures.com/ ；任职：https://openai.com/index/paul-christiano-joins-openai-foundation-board/ | `c-c6-arstechnica-christiano.*`、`c-c6-hinton-home.*`（`c-c6-hinton-2026.*` 内容相同）、`c-c6-openai-board-announcement.*` | Ars：2026-11-13T17:25:45Z = 2026-11-14 01:25:45（北京）；讲座 Night 1：2026-11-09 18:00–19:30 美东 = 2026-11-10 07:00–08:30（北京） | 讲座与头衔已找到；引语未找到一手逐字记录 |

## 二、逐条要点

### C1 Google Antigravity /teamwork 科研成果

- 限定与人机原句（官方总述）：“All of this work is done in collaboration with human experts, and we are making these tools available externally.”
- “autonomously” 出现的句子（均为官方自述，未独立核验）：
  - Weil：“Notably, /teamwork-LEAP fully autonomously produced an end-to-end, machine-checked formalization of the Weil conjectures … This formalization spans about 1.8 million lines of Lean, produced across 28,800 subagent invocations.”
  - Courtade–Kumar：“prove information theory's 2013 Courtade–Kumar conjecture in full generality”；“The team formalized the entire proof in Lean 4, producing 50 million lines of code.”；“They also extended the proof from balanced to unbalanced functions without human input.”
  - 自适应数据分析：“a multi-agent harness running on Gemini 4 Argon independently proved and Lean-verified near-optimal sample bounds …”
- Navier–Stokes（修正扫描说法）：原文为“It also independently reproduces an end-to-end formalization of a recent proof of finite-time blowup for the 3D Navier–Stokes equations with smooth forcing”。对象是**某近期证明的形式化**，不是 Google 自己证明了 Navier–Stokes，也不是复现证明本身。
- 可用范围：“We have also opened applications giving 10,000 academic researchers six months of free access to our highest usage tier.”（“applications”即申请；官方未写是否限定机构或国家，表单未核。）
- 热度：HN 50496811，206 分，嵌套评论 40 条，创建于 2026-11-14 21:20（北京）。

### C2 Google Project Zero 缩短披露期限

- 旧政策：“Since 2021, Project Zero has followed a “90 + 30” vulnerability disclosure policy, allowing vendors up to 90 days to remediate a vulnerability plus an extra 30 days “soak time” for patch rollout.”
- 新政策：“Starting today, our policy is “Full 90”, i.e. vulnerabilities will be disclosed 90 days after they are reported to the vendor.”；“On February 2, 2027, we will move to a “Full 60” policy where vulnerabilities are disclosed after 60 days.”；“On January 3, 2028, we will move to a “Full 30” policy where vulnerabilities are disclosed after 30 days.”
- AI 理由：“The advent of AI-assisted vulnerability discovery and exploitation has changed the threat landscape such that the vulnerability remediation timeline encouraged by our disclosure policy is clearly insufficient.”
- 例外（限定句）：“We are also removing most ‘special cases’ from our disclosure policy, including grace period and variant policy.”；例外条款为 ITW 漏洞（按谷歌应用安全披露政策处理）、公开的漏洞、厂商请求。
- 热度：HN 50496051，271 分，嵌套评论 149 条，创建于 2026-11-14 19:13（北京）。

### C3 Gemini 接入 Google Wallet

- 范围：“Rolling out today for all Gemini app users in the U.S., you can connect Google Wallet for responses informed by your spending habits.”
- 银行账户：“You also have the option to securely link your financial accounts from your credit cards and banks to Wallet via Plaid to unlock deeper insights.”
- 只读限定：“If you choose to connect your Wallet (under Connected Apps in Personal Intelligence settings), Gemini can only read information from your accounts; it can’t move, manage, or invest your money.”
- 平台：“Google Wallet in Gemini is rolling out across web and mobile, for Gemini app users on both Android and iOS.”
- 年龄或账户类型限制：官方未写。
- 热度：未找到 HN 对应条目（见 capture-log-c.md；Algolia 两轮检索）。barking heat 未写入（见下方“工具问题”）。

### C4 DeepSeek 开放 App 能力 API

- 官方原句：“Today we open these capabilities to developers: enable the corresponding parameters in the existing APIs to get conversation features such as web search, file input and URL reading, with the tool calls and context assembly handled by the service.”
- 支持范围：“The deepseek-flash model supports the following in the Messages, Responses and Chat Completions APIs:”，列出联网搜索（非思考模式）、深度思考搜索、文件输入、URL 读取、模型安全。
- 语音：官方写的是“these underlying capabilities and the voice capabilities are available as standalone APIs, including Search and Fetch, OCR, ASR and TTS”。语音以独立接口提供，**不在** deepseek-flash 三套 API 的支持列表内。
- 合规义务：“If you provide a service to the public, enable this parameter and fulfill obligations such as content safety review, large model registration and AI labeling.”
- 计费：本条目未写新价格，只指向 Models & Pricing（未写）。
- 媒体说法“1:1 复刻 DeepSeek App”（澎湃）：WebSearch 两轮未找到该稿，也未见 2026-11 相关报道。**未找到一手来源**；官方原文没有“复刻”的说法。

### C5 EFF 观点文

- 核心论点（原句，EFF 观点）：“They ignore every bystander you interact with and their inability to actually opt out.”
- 另一句：“This is not reasonable, nor how consent works in any practical sense.”
- 事实性陈述（可作为 L1 事实，须标“EFF 认为”）：“Meta and Google’s smart glasses can record anything the owner is looking at or use artificial intelligence to describe it.”
- 热度：HN 50496602，563 分，嵌套评论 184 条，创建于 2026-11-14 20:45（北京）。

### C6 Christiano 讲座与头衔

- 讲座（官方讲座页 hintonlectures.com，archive 2026 版）：Hinton Lectures 2026，11 月 9–11 日，多伦多 John W. H. Bassett Theatre。Night 1 “AI alignment: What’s working and what’s not”，“Monday, November 9”，6:00–7:30 P.M. Eastern；讲员 Paul Christiano，由 Geoffrey Hinton 介绍。录像在 YouTube（未抓取）。
- 头衔（官方讲座页）：“Executive Director and Founder, Alignment Research Center”。
- 任职（OpenAI 官方公告，2026-09-09）：“We’re announcing the appointment of Paul Christiano to the OpenAI Foundation Board. He will be a non-voting observer on the OpenAI Group PBC Board.” 另加入 Safety and Security Committee。公告拼写为 “Paul Christiano”。
- Ars 引语（未核到一手原话）：“I think for most people in this room, the most likely reason they will die is because of an AI accident.”；“no ‘quantitative model’ that can ‘put a number’ on the likelihood”；“something like a coin flip of an AI takeover”；“meaningful risk” of one happening in the next six to 12 months。这些只来自 Ars 的转引，讲座录像或字幕未核对，标**未找到一手逐字记录**。
- 写法建议：正文可写“据 Ars Technica 报道，Christiano 在讲座中称……”；“OpenAI 基金会董事”的身份可以由 OpenAI 公告直述。拼写用 Christiano，不用 Cristiano。
- 热度：未找到 HN 对应条目（见 capture-log-c.md）。

## 三、扫描说法勘误

1. **C1 Navier–Stokes**：扫描写“爆破是‘reproduces’已有证明”。原文是 reproduces an end-to-end **formalization** of a recent proof，对象是形式化，不是证明本身，也不是 Google 自己证明的。
2. **C1 人机分工**：扫描写“Courtade–Kumar 猜想被证明并形式化”，原文属实，并补充：Courtade–Kumar 的“extended the proof from balanced to unbalanced functions without human input”（去平衡情形的推广由系统完成，官方原句）。Weil 句为“fully autonomously”，总述为“in collaboration with human experts”，两者并存。
3. **C2**：扫描所列日期、三档政策与 AI 理由均与原文一致。补充：作者 Natalie Silvanovich；官方未给时刻。
4. **C3**：扫描所列句子全部属实。“2026-11-13T17:00Z”属实，北京时间为 2026-11-14 01:00；页面标签只写“Nov 13, 2026”。
5. **C4**：扫描写“语音”开放给开发者，需限定：语音是独立 ASR/TTS 接口，不在 deepseek-flash 三套 API 的列表内。扫描写“澎湃称 1:1 复刻 DeepSeek App”——未找到该稿，官方原文没有此说法。
6. **C5**：扫描未写时刻。北京时间为 2026-11-14 05:29:31（页面 PT 2026-11-13 13:29:31），页面可见日期为 November 13, 2026。
7. **C6**：
   - Ars 正文拼作 “Paul Cristiano”，官方为 “Paul Christiano”（扫描说 Ars 拼作 Cristiano，属实）。
   - Ars 页面标题为 “OpenAI board member worries an AI ‘takeover’ of society could be months away”，URL slug 含 “most-likely-reason-youll-die”，扫描引用的是 URL，正文用引语即可。
   - BetaKit 等二手报道称 Christiano 是“周二、第二场”讲座，与官方页面的 Night 1（周一，11 月 9 日）冲突，以官方为准。
   - 扫描称其“现任头衔（OpenAI 基金会董事等）”：OpenAI 公告 2026-09-09 确认为 Foundation Board 成员（非投票观察员于 Group PBC 董事会），官方讲座页写的头衔是 ARC Executive Director and Founder。
8. **热度数字变化**：扫描写的 HN 分数（205、267、560）与抓取时刻不同，当前为 206、271、563；评论数以 barking 解析的嵌套评论计：40、149、184。

## 四、工具问题（供 Claude 收尾）

- `barking heat` 第一次调用 `--item` 时误写了 `50496...`（占位符，非 HN id），报错后未写入任何文件；第二次成功，写入 `c-heat.json` 与 `c-hn-{50496811,50496051,50496602}.json`。
- 之后再用 `--hn "…"` 查询 C3、C4、C6 的 HN 条目时，因 `c-heat.json` 已存在，工具拒绝覆盖（“改用别的组前缀，或先删除”）。本组没有 `rm` 权限，未删除，也没有改用别的组前缀。C3、C4、C6 的 HN 热度因此未写入 `barking heat` 记录，改用 Algolia 检索（见 capture-log-c.md），结论为未找到对应条目。
