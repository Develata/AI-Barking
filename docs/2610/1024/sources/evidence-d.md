# 取证交回：D 组 速览（1024 期）

取证时刻：2026-10-25 23:25–23:30（北京时间，本机 EDT 时钟 +12 小时换算）。取证人：claude-haiku-5-5。范围：D1–D6 与扫描说法勘误，不截图（派工单）。存档在 `docs/2610/1024/sources/d-*`。原时区为官方页面所示，页面只给日期的，写“官方未给时刻”。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

| # | 条目 | 状态 |
|---|---|---|
| D1 | MiniMax-M3.1-Flash 权重上线 | 已找到（权重公开的 HF 仓库已核；参数量与 AA 状态的分项结论见下方 D1 节与勘误） |
| D2 | Johns Hopkins Bayview 员工上传患者信息 | 部分支持（仅见 Baltimore Sun 付费墙前导段；医院通知原文与 HHS/州通报未找到，未找到一手来源） |
| D3 | 音乐版权诉讼告 Anthropic | 已找到（Reservoir 诉状 PDF 已取：10/21 立案，853 首作品歌词；“约 3.4 亿美元”为两案诉状所列金额之和，须写明是指控）；Elvis 相关诉讼部分支持（10/23 起诉，274 首，媒体两家报道，诉状未取得） |
| D4 | Cerebras 万亿参数部署说明 | 已找到（厂商官方博客，厂商自述；无成本数字） |
| D5 | 英国 AI 生物安全实验室 | 已找到（gov.uk 官方新闻；现金与额度区分见下） |
| D6 | 备用：Claude 订阅额度 A/B 灰测 | 未找到一手来源（两次检索无 SemiAnalysis 原帖与 Anthropic 员工原帖，不展开） |

---

## D1 MiniMax-M3.1-Flash 权重上线【跟进1019】

- **一句事实**：MiniMax 10/19 称 M3.1 Flash“现已通过 MiniMax API、M Plan 与 MiniMax Code 提供”，并写“Model weights will be released later”；此后 Hugging Face 上出现 `MiniMaxAI/MiniMax-M3.1-Flash` 公开仓库（非门控，许可证为 MiniMax Community License），模型卡写“Approximately 428B total; 23B activated per token”。
- **状态**：已找到（仓库公开）；“428B 是 M3 规格”与说法不符（见下）。
- **级别**：L1（官方博客、官方 Hugging Face 仓库）；AA 页为 L3。
- **一手来源**：
  - 官方博客 https://www.minimax.io/blog/minimax-m31f （`d-d1-minimax-blog.html` / `.txt` / `.headers`）
  - HF 模型卡 https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash （`d-d1-hf-model.html` / `.txt` / `.headers`）
  - HF API 元数据 https://huggingface.co/api/models/MiniMaxAI/MiniMax-M3.1-Flash （`d-d1-hf-api.json` / `.headers`）
  - HF 提交记录 https://huggingface.co/api/models/MiniMaxAI/MiniMax-M3.1-Flash/commits/main （`d-d1-hf-commits.json` / `.headers`）
  - 许可证 https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash/raw/main/LICENSE （`d-d1-hf-license.txt`）
  - AA 模型页 https://artificialanalysis.ai/models/minimax-m3-1-flash （`d-d1-aa-model.html` / `.txt` / `.headers`）
- **发布时刻**：
  - 官方博客页日期 2026-10-19；官方未给时刻。
  - HF 元数据：`createdAt` 2026-10-17T06:37:50Z（北京 10/17 14:37）；`lastModified` 2026-10-23T03:37:20Z（北京 10/23 11:37）。提交记录只有一条，`Init MiniMaxAI/MiniMax-M3.1-Flash`，时间同 `lastModified`。
  - 公开时刻未核。HN 上最早指向该 HF 链接的提交为北京 10/23 09:09（社区提交，仅作参考），早于上述提交时间，两者不一致，未解释。
- **限定词与原句**：
  - 官方（10/19）：“M3.1 Flash is now available through the MiniMax API, M Plan, and MiniMax Code. Model weights will be released later.”
  - HF 模型卡：“Parameters: Approximately 428B total; 23B activated per token”；“Context window: Up to 1,000,000 tokens”。卡中另写“Building on MiniMax-M3, M3.1-Flash improves task performance…”
  - HF 元数据：`gated: false`，`private: false`；`cardData.license: other`，`license_name: minimax-community`。
  - 许可证（LICENSE 原文）：“to deal in the Software for non-commercial purposes”；商业使用须“Built with MiniMax M3.1-Flash”署名；“if such products and services generate more than 20 million US dollars … in yearly revenue”须事先书面授权，否则只需一次性通知。许可证不是开源许可证，不能写成“开源”。
  - AA 页：“Proprietary model”；FAQ：“No, MiniMax-M3.1-Flash is proprietary. The model weights are not publicly available.”；“MiniMax has not disclosed the model size or parameter count.”（AA 页与 HF 公开状态相左，见勘误）
- **参数量核对**：
  - HF 模型卡写 Approximately 428B total，23B activated；卡中“What’s New”称 M3.1-Flash 保留 M3 的 MoE 架构，并非说 428B 属于 M3。
  - HF `safetensors` 元数据合计 220,490,854,016 个元素，分 dtype：F32 46,339,200；BF16 3,323,222,528；F8_E4M3 10,560,208,896；U8 206,561,083,392。U8 占绝大部分，与量化打包存储有关，原因未核；两者不能直接比较。
- **热度**（`barking heat`，北京 10/25 23:27）：HN“MiniMax M3.1 Flash”主帖 517 分 / 220 评（10/20 00:39 BJ，官方博客链接）；HN 指向 HF 仓库的提交 2 分 / 0 评（10/23 09:09 BJ 与 10/23 20:41 BJ 各一条）；Reddit 3 次请求失败（登录页重定向，见 capture log）。
- **勘误**：1019 速览线索称“428B 是 M3 规格（推断，未读模型卡）”，与 M3.1-Flash 模型卡原文不符，应改为“模型卡写 Approximately 428B total”。

---

## D2 Johns Hopkins Bayview 员工上传患者信息至未获批公共 AI 工具

- **一句事实**：Baltimore Sun 报道，Johns Hopkins Bayview 正在通知 3,182 名患者，原因是一名员工把个人健康信息上传到一个未获批的公共 AI 工具。
- **状态**：部分支持。文章为付费墙前导段，仅有一句；医院通知原文、HHS 或州检察长通报均未找到，故一手来源未找到。
- **级别**：L4（权威媒体，单一来源）。
- **一手来源**：Baltimore Sun https://www.baltimoresun.com/2026/10/24/johns-hopkins-bayview-medical-center-data-breach/ （`d-d2-baltimore.html` / `.txt` / `.headers`）；医院通知原文：未找到。
- **发布时刻**：PUBLISHED October 24, 2026 at 2:46 PM EDT（北京 10/25 02:46）；UPDATED 2:57 PM EDT（北京 10/25 02:57）。
- **限定词原句**：“Johns Hopkins Bayview is notifying 3,182 patients after an employee uploaded personal health information to an unapproved public AI tool.”（归档文字即此句，其余正文付费墙）
- **未核**：
  - “9/21 发现”“涉姓名、病历号、用药等”“不含 SSN”“WMAR”：未获原文，未核，不可写入速览。
  - 检索子代理称的 AI 工具名：未披露（按扫描说法未核）。
- **检索记录**：两次 WebSearch（“Johns Hopkins Bayview … 3,182 patients …”、“Bayview Medical Center employee … breach notice”）未返回 Bayview 事件，结果为 2024 年 Johns Hopkins MOVEit 事件、Loma Linda 等其他事件。
- **热度**：HN 检索“Johns Hopkins Bayview”0 条；“Johns Hopkins AI patient data”0 条。

---

## D3 两起音乐版权诉讼告 Anthropic

- **一句事实（Elvis 相关）**：Billboard 报道，Elvis Presley 相关权利方（Authentic Brands Group 旗下实体等）于 10/23（周五）起诉 Anthropic，指控其在训练 Claude 时使用了该歌手 274 首作品；此为指控，尚未经法院认定。
- **状态**：部分支持（两家媒体一致，诉状未取得）。
- **级别**：L4（Billboard、The Hollywood Reporter，均为权威媒体；转引 Rolling Stone 的诉状原句）。
- **一手来源**：
  - Billboard https://www.billboard.com/pro/elvis-presley-estate-sues-suno-anthropic/ （`d-d3-billboard.txt` 可入库；HTML 原件 `d-d3-billboard.html` 1,031,744 字节，只存本地）
  - THR https://www.hollywoodreporter.com/music/music-industry-news/anthropic-elvis-presley-lawsuit-ai-1236739617/ （`d-d3-hr-elvis.html` / `.txt` / `.headers`）
  - 诉状与 CourtListener 记录：未找到（CourtListener 搜索 API 返回的“Anthropic”相关最新案件到 2026-06-17，无 Presley 案，见 capture log）。
- **发布时刻**：Billboard 页面日期 10/24/2026；THR 页面 “October 24, 2026 7:22pm”（时区未标）；诉讼“filed Friday (Oct. 23)”，法院立案时刻未核。
- **限定词与原句**：
  - Billboard：“The new lawsuit, filed Friday (Oct. 23) and first reported by Billboard, claims that Anthropic, the tech behemoth behind Claude, infringed 274 of the rock legend’s copyrighted songs, including famed hits like ‘Can’t Help Falling in Love,’ ‘A Little Less Conversation’ and ‘Don’t Be Cruel.’”
  - Billboard：“The case was filed by a unit of Authentic Brands Group”
  - THR：“The suit was filed Friday and claims the company infringed 274 songs of Presley’s”
  - 诉状原句（经 Rolling Stone 转引，THR 引）：“‘Claude’ has created infringing output of copyrighted material of virtually every song from every catalog available on the internet, including the Works”
  - 诉讼请求：THR 引 Rolling Stone 称寻求“$150,000 per infringed song”（每首上限，指控性质）。
- **被告**：Billboard 正文只写 Anthropic 为被告；URL slug 含 “suno”，正文中 Suno 只出现在 2024 年三大唱片公司起诉的背景段落中。Elvis 一案是否也起诉 Suno，未核，不写。
- **未核**：Anthropic 对本案的回应：未见。
- **Reservoir 诉讼（一手诉状已取）**：
  - 立案：Filed 10/21/26，N.D. Cal.，Case 5:26-cv-12308；原告 Reservoir Media Management, Inc. 与 Reservoir Media Management (UK) Limited（`d-d3-reservoir-anthropic-complaint.pdf` / `.txt`，诉状首页与页脚可见）。媒体报道称“Wednesday (October 21)”（MBW，`d-d3-mbw-reservoir.txt`）。
  - 作品：诉状附件 Exhibit A 列 853 首作品，主张 Reservoir 对其美国版权拥有 100% 或独占控制。原句：“A list of 853 copyrighted musical compositions in which Reservoir …”（诉状第 2 页）。指控 Anthropic 以这些作品的歌词训练 Claude，并指 Claude 在提示下生成歌词复制与衍生作品（指控）。
  - 请求金额（诉状所列，属指控）：版权一项“up to $150,000 per Work willfully infringed”，诉状算出 $127,950,000.00；DMCA §1203 两项分别为 $2,132,500.00 与 $21,325,000.00。诉状请求合计约 $151.4 百万。
  - Suno 另案（MBW 报道，未取诉状）：1,050 首，诉状算出 $157.5 百万，DMCA 两项 $2.625 百万与 $26.25 百万，报道称三项合计“just over $186 million”。
  - “约 3.4 亿美元”：是两案诉状所列金额之和（约 $151.4 百万 + 约 $186 百万，约 $338 百万），是原告请求的金额，不是法院判定或法定上限的判定；写作时须写明为指控。
  - 未找到：Anthropic 对诉状的回应（搜索未见）。
  - 检索另见 Round Hill Music 于 8/17 起诉 Suno 与 Anthropic，与本案是不同原告、不同日期，不混用。
- **热度**：HN“Anthropic Sued for Using Elvis Presley Music to Train AI Models”6 分 / 0 评（10/25 17:15 BJ，THR 链接）。

---

## D4 Cerebras 万亿参数模型部署说明

- **一句事实**：Cerebras 官方博客（10/22）以“开源模型 Kimi K3（约 3 万亿参数）”为例，说明把 93 层摊到 59 片 WSE-3T 晶圆（35 + 24）、采用 4-bit 权重与 8-bit KV cache、按 256k 平均请求长度估算，目标为 1000–2000+ tokens/s；文中为厂商示例配置。
- **状态**：已找到（厂商官方页，厂商自述；文中无成本数字）。
- **级别**：L1（厂商官方博客，厂商自述）。
- **一手来源**：https://www.cerebras.ai/blog/how-we-deploy-trillion-parameter-frontier-models-on-cerebras （`d-d4-cerebras.html` / `.txt` / `.headers`）
- **发布时刻**：页面日期 Oct 22 2026；官方未给时刻。
- **限定词与原句**：
  - “We’ll use the open source model Kimi K3, with ~3 trillion parameters, as an illustrative example.”（示例）
  - “…to run multi-trillion parameter models at 1000-2000+ tokens per second.”
  - “This gives us a total wafer count of 35 + 24 = 59 wafers.”
  - “A 256k assumed average request size”（假设值）
  - “4-bit weight quantization”；“An 8-bit KV cache”
  - 页末：“Performance comparisons are based on third-party benchmarking or internal testing. Observed inference speed improvements versus GPU-based systems may vary depending on workload, configuration, date and models being tested.”
- **未核**：并发请求数与 tokens/s 以图呈现，图片未入库，文字中未读到具体数字（图为 `d-d4-cerebras.html` 内的图片，未单独存档）。
- **热度**（`barking heat`，北京 10/25 23:28）：HN“Cerebras Deploys Trillion-Parameter AI Models”（本页链接）264 分 / 88 评（提交于 10/25 09:33 BJ）。同链接另一提交 2 分 / 0 评（10/23 06:41 BJ）。扫描线索称“251/83”，已过时。

---

## D5 英国 AI 生物安全实验室（AI Biosecurity Lab）

- **一句事实**：英国官方公告称，AI Biosecurity Lab 由“£100 million of government funding”与“up to £200 million in company commitments from OpenAI and Anthropic”支持，官方定位为“the world’s first dedicated AI biosecurity organisation”。
- **状态**：已找到（官方页）。
- **级别**：L1（官方新闻页）。
- **一手来源**：https://www.gov.uk/government/news/introducing-the-ai-biosecurity-lab （`d-d5-gov-uk.html` / `.txt` / `.headers`）
- **发布时刻**：Published 24 October 2026；官方未给时刻。
- **限定词与原句**：
  - 标题导语：“Backed by up to £300mn of government funding and company commitments”
  - “Backed by £100 million of government funding, and up to £200 million in company commitments from OpenAI and Anthropic”
  - 现金与额度的区分（原句）：“OpenAI will provide £100 million in compute and model credits and trusted access to its most capable relevant models through the Rosalind Biodefense programme.”；“Anthropic will provide up to £100 million in model credits, trusted model access and dedicated technical support.”
  - 注意：政府部分是现金（100 million of government funding）；两家公司是算力、模型额度与访问权，OpenAI 原句无“up to”，Anthropic 有“up to”。
  - 页面另写：“The AI Biosecurity Lab will be the world’s first dedicated AI biosecurity organisation.”；“The Government intends for AIBL to be supported by both public and philanthropic funding”（计划中）。
- **热度**：HN“UK to invest £100M in 'world first' AI Biosecurity Lab”（BBC 链接）4 分 / 0 评（10/24 16:44 BJ）。

---

## D6 备用：Claude 订阅额度 A/B 灰测

- **状态**：未找到一手来源。
- **检索记录**：两次 WebSearch（“SemiAnalysis Claude Max 20x A/B test 5-hour usage limit weekly limit 19%”、“Lydia Hallie Boris Cherny Claude Max usage limit A/B test SemiAnalysis response”）未返回 SemiAnalysis 原帖、Anthropic 员工原帖，也未找到 19% / −20% 数据。搜索结果只有 Anthropic 帮助中心页与二手汇总。
- **不展开**，按派工单不记“已找到”。凤凰网转述页 https://tech.ifeng.com/c/8xSNopjD8lu 未打开。

---

## 扫描说法勘误

1. **D1 参数量**：1019 速览线索“safetensors 合计约 220B；部分中文稿写的 428B 是 M3 规格（推断，未读模型卡）”与原文不符。M3.1-Flash 模型卡写“Approximately 428B total; 23B activated per token”。220B 是 safetensors 元素合计（U8 占绝大部分，原因未核），不是官方参数量。
2. **D1 公开状态**：扫描说的“HF 公开、非门控、lastModified 10/23”属实（已核）。但“10/23”是 HF 元数据的 lastModified，并非官方宣布的上线时刻；官方 10/19 写“稍后发布”。HF 元数据 createdAt 为 10/17。HN 最早指向 HF 的提交为 10/23 09:09 BJ，与提交记录时刻不一致，公开时刻未核。
3. **D1 AA 页**：AA 页仍写“Proprietary model”与“not publicly available”，与 HF 公开状态相左。AA 页响应头 Date 为 GMT 10/25 13:13（北京 21:13），可能是缓存，抓取时刻 23:25 BJ。
4. **D2**：Baltimore Sun 报道仅前导段可见；“9/21 发现”“不含 SSN”“WMAR”未核；医院一手通知未找到。WMAR 与 hopkinsmedicine.org 的检索均未返回 Bayview AI 事件；`hopkinsmedicine.org/data-attack` 返回 HTTP 403，只能见到 MOVEit 旧通知（与 AI 无关）。
5. **D3 Elvis 诉讼**：扫描称“两起音乐版权诉讼”；已核 Elvis 一起（10/23，274 首）。原告为 Authentic Brands Group 旗下实体等，媒体写“owners of Elvis Presley's music”。
6. **D3 Reservoir 诉讼**：扫描线索大体属实。10/21 立案（N.D. Cal. 5:26-cv-12308）；Anthropic 诉状列 853 首作品歌词；Suno 另案 1,050 首（MBW 报道，未取诉状）。“约 3.4 亿美元”是两案诉状请求金额之和的推算，不是法定上限或判定，须写明为指控。
7. **D4 热度**：扫描线索 HN 251 分 / 83 评，已更新为 264 分 / 88 评（10/25 23:28 BJ）。
8. **D5 数字**：扫描线索“OpenAI、Anthropic 各承诺最多 1 亿英镑”不准确。gov.uk 原文：OpenAI 为“£100 million in compute and model credits”（无 up to），Anthropic 为“up to £100 million in model credits”。政府部分为现金 1 亿英镑。
9. **D6**：扫描线索“SemiAnalysis 称两个同价 Max 20x 账号，一个 5 小时窗口 +19%、周总额 -20%”未找到原帖，不得写入速览。

---

## 本组文件清单（均在 `docs/2610/1024/sources/`）

- 文本存档（可入库）：`evidence-d.md`、`capture-log-d.md`、`d-d1-minimax-blog.txt`、`d-d1-hf-model.txt`、`d-d1-hf-api.json`、`d-d1-hf-commits.json`、`d-d1-hf-license.txt`、`d-d1-aa-model.txt`、`d-d2-baltimore.txt`、`d-d3-hr-elvis.txt`、`d-d3-billboard.txt`、`d-d3-mbw-reservoir.txt`、`d-d3-reservoir-anthropic-complaint.txt`（80 KB）、`d-d4-cerebras.txt`、`d-d5-gov-uk.txt`、`d-heat.json`、`d-extra-heat.json`、`d-fetch-log.jsonl`、`d-d3-courtlistener-search.json`，以及各条 `.headers`（`d-d1-hf-commits.headers` 已重写为四行精简格式，去掉了代理与请求标识）。
- 本地保存、不入库（HTML/PDF 原件）：`d-d1-minimax-blog.html`、`d-d1-hf-model.html`（325 KB）、`d-d1-aa-model.html`（4,467,942 字节，超过 1 MB）、`d-d2-baltimore.html`（166 KB）、`d-d3-hr-elvis.html`（606 KB）、`d-d3-billboard.html`（1,031,744 字节，按十进制 1 MB 计超过 1 MB）、`d-d3-mbw-reservoir.html`、`d-d3-reservoir-anthropic-complaint.pdf`（260 KB）、`d-d4-cerebras.html`、`d-d5-gov-uk.html`。`d-d2-hopkins-dataattack.html` 为 403 页面，无正文。
- 待删除候选（本组自生成的试验文件，按协调者规则不删，待 Claude 统一处理）：无。本组未生成任何需要删除的废弃文件；`d-extra-heat.json` 为第二次热度查询（组前缀 `d-extra`，因 `d-heat.json` 已存在而拒绝覆盖），属有效记录，保留。
