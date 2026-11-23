# 1122 期 C 组（速览）取证

执行方：模型 claude-haiku-5-5（Haiku 5.5）。本组不截图，只存档与摘句。时间一律北京时间（UTC+8），官方页只给日期的如实写“官方未给时刻”。状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

## 一、逐条

### C1 智谱 GLM-5.4 发布（已找到，部分支持）

- 一手来源：Z.ai 博客 `https://z.ai/blog/glm-5.4`（页面标注 “2026-11-23 · Research”，官方未给时刻）；智谱微信公众号原文 `https://mp.weixin.qq.com/s/D-IZr-QaT1or_chminK8IQ`（页面元数据 publish_time 换算为 2026-11-23 17:40:42 +08:00；扫描称 17:42，以元数据为准）。
- 存档：`c-c1-wx-glm54.html` / `.txt`（微信，curl 取得）；`c-c1-zai-glm54.txt`（Z.ai，curl 被拦，改由 opencli 渲染取文本，html 为拦截页未采用）。
- 摘句（英文博客原文）：“GLM-5.4 is our first flagship model built on the Loop Transformer architecture, reaching intelligence levels comparable to 2T-3T parameter models.”（中文原文：“GLM-5.4是我们第一代采用Loop Transformer架构的旗舰模型。收获了效果，智能水平达到2-3T级模型。”）“2T-3T 级”为官方自述，正文未给出对应的基准分数表；文字中未见绝对分数，30B 对照只有“相对基线有明显提升”与“平均输出长度下降10%–14%”的相对描述（图表未存档）。
- 开放范围（原句）：中文“即日起，GLM-5.4向GLM Coding Plan全量用户开放……API将在完成安全评估后开放，模型权重计划以负责任的方式开源。” 英文 “API (Coming Soon)”；“Open Weights: We plan to open-source the model weights responsibly after completing necessary safety hardening.”
- 限定词：API 为 “Coming Soon”；开源为 “plan”；官方同时写明局限：“it tends to get caught in overly long ‘marathon thinking’ loops”（中文：“模型很容易进入‘雷霆大思考’模式”）。
- 热度（barking heat，`c-hn-50609685` 与 `c-heat.json`）：HN 652 分 / 含嵌套评论 238 条，发帖 2026-11-23 18:06 北京时间（10:06:43Z）；检索时 Algolia 记为 650 分 / 244 评，两个数字全部保留。
- 状态理由：开放范围与“2-3T 级”均为官方自述原句，可按 L1 直述；未见第三方评测。

### C2 OpenAI 211 分钟故障复盘（与说法不符）

- 扫描说法：2026-11-21 13:48–17:21 美西，ChatGPT 与 API 失败率峰值 75%，实例为“备用数据库”。
- 一手页面：`https://status.openai.com/incidents/n254wyd7nml7`，存档 `c-c2-openai-incident.html` / `.txt`。页面正文与时间戳均为 **2023 年**：“Nov 21, 2023, 05:09 PM”起，“Fri, Dec 1, 2023, 06:24 PM Resolved”。正文原句：“On November 21st, between 1:48 - 5:21 PT a large portion of requests to OpenAI failed. This peaked at a 75% failure rate on paid and free ChatGPT.”（页面未写年份，年份由页面时间戳给出；时间为 PT，页面未给出时区名之外的说明。）
- 2026 年对照：OpenAI 状态页 JSON `c-c2-openai-api-incidents.json`（25 条，创建时间 2026-10-23 至 2026-11-23）中**没有 2026-11-21 的事故**。该 JSON 中 11-19（gpt-6 Luna API）与 11-23（GPT-6 付费对话，minor）两条与之无关。
- 结论：扫描说法所依据的页面是 2023 年事故；2026 年 11-21 的“211 分钟、13:48–17:21 美西、峰值 75%”没有一手来源支持。不得用于速览或正文。此为推断：扫描可能误把旧页当成本期事件（L1 页面本身存在，但年份不符）。
- 热度：无 HN 条目记录。
- 对读者：无。

### C3 Claude 11-23 多模型错误率升高（已找到）

- 一手来源：Anthropic 状态页（stspg 短链指向 `status.claude.com`）：事故 “Elevated errors for multiple models”，id `yzs0s90d8fm8`，impact major。存档 `c-c3-claude-incident.html` / `.txt`（状态页）与 `c-c3-claude-api-incidents.json`（API，25 条）。
- 时间（北京时间）：创建 2026-11-23 17:49:22（09:49:22 UTC）；标记 resolved 2026-11-23 19:07:41（11:07:41 UTC）。
- 受影响模型（identified 更新原句，09:56:38 UTC）：“elevated errors on requests to Claude Mythos 5.2, Claude Fable 5.2, Claude Opus 5.5, Claude Sonnet 5.5, Claude Haiku 5.5, and other Claude models”；受影响产品：claude.ai、Claude API、Claude Code、Claude Cowork（monitoring 更新的组件列表）。
- 原因：页面只写 “We have identified the cause … and are working on a fix”，未给根因。
- 时间不一致（须记录）：resolved 更新写“Impact occurred from 01:39 PT / 09:39 UTC to 01:56 PT / 09:56 UTC”，即影响窗口为北京 17:39–17:56，早于事故页创建时刻 17:49。两个时段都是页面原文，不自行取舍；如要写起止时间，两者都要注明出处。
- 状态：已找到。L1 直述。

### C4 OpenAI 首席科学家提议以困惑度设行业共同上限（部分支持；微软回应未找到）

- 一手来源：`https://openai.com/index/perplexity-and-intelligence/`，作者 Jakub Pachocki（OpenAI Chief Scientist），页面日期 “November 22, 2026”（官方未给时刻；HN 提交 2026-11-23 01:06 北京时间）。存档 `c-c4-openai-perplexity.html` / `.txt`。
- 提议原句：“I argue for establishing perplexity (or, equivalently, cross-entropy loss) as a metric to compare between model developers and inform shared safety standards.”
- 上限的性质（原句）：“I further propose common limits, based on perplexity and other measurements, that developers are committed to not crossing until our ability to understand and monitor AI behavior achieves a certain concrete milestone”；“I’d like an industry standard that specifies a first limit on development”。文中**没有给出具体阈值**，也没有时间表。
- 数值与限定（OpenAI 自报）：GPT-3.5 约 0.99、GPT-4 约 0.91、“losses below 0.8”、“log-perplexity ~0.76”，外推“~0.68-0.7 log-perplexity window”。脚注 3：“The primary held-out datasets we’ve been using since GPT-3.5 are sourced from our internal codebase, and those are the numbers reported here (for a fixed snapshot).” 正文另写：“Difficulty of comparison across labs. Different developers use different evaluation datasets and train on different data mixtures.”
- 发布前的顾虑（原句）：“Publishing our internal perplexity measurements took some consideration; it gives competitors a clearer picture of where we stand”。
- 微软回应：两次检索（WebSearch）未找到微软针对该文发布可比数据的一手材料，查到的只有微软 MAI 模型的第三方转述，与本事件无关。状态：未找到一手来源（扫描称“微软随后公布可比数据”未经证实）。
- 热度：HN 50601525，653 分 / 425 评（发帖 2026-11-23 01:06 北京时间）。

### C5 pequalnp.com：AI agent 协作攻 P vs NP，Lean 逐条检查（部分支持，本组唯一的站点自述）

- 一手来源：`https://pequalnp.com/`（首页）、`/about`、`/live`、`/feed.atom`。存档 `c-c5-pequalnp.html/.txt`（静态 HTML，计数为空）、`c-c5-pnp-about.html/.txt`、`c-c5-pnp-live.html/.txt`、`c-c5-pnp-feed.bin`（Atom，未改名）。渲染后的首页文本：`c-c5-pnp-dashboard-0012.txt`（由 opencli 2026-11-24 00:12–00:14 北京时间输出逐字摘录；之后快照停在加载状态，未能重取）。
- 站点自述（About 页原句）：“A fleet of AI agents, run by Brilliant, works the hardest open problem in computer science in the open. Lean checks every claim.”；“Refuted bars count as much as checked ones.”
- 首页数字（2026-11-24 00:12–00:14 北京时间）：“7,223 agent sessions so far”；“316 agents working now”；“738 of 1,306 lemmas checked · hour 4 of day 2 · 33 in the last hour · last checked 3 min ago”；“Verdict: open, being worked on now.”。“Just in” 条：“Refuted by the checker: two of the fleet's own conjectures on gate counts, each by one example.”
- 限定词：feed 中 “Checked” 与 “Conjecture” 分开记录；Conjecture 的条目原句为 “Conjecture: seat-clb-1 proposed L-meta-739: …”，即未核。首页 legend 写明 “conjecture, unchecked”。“checked” 指引理，不是 P vs NP 本身：页面 verdict 仍为 “open”。
- 未取得：About 页中“How do I know ‘checked’ isn’t made up?”的答案全文（答案不在 About 页内，点击与页内链接均未展开，未找到直链）。原扫描提到的 “no result here is a proof unless it says checked” 在已存档文件中未找到，不能作为原句引用。
- 发起者署名（站点页脚原句）：“A project by Amichai & Lior”，制作平台 Brilliant。此处只写站点署名，不另收集个人信息。
- 热度：HN 50604314，“Agent swarm trying to solve P vs. NP”，160 分 / 147 评（发帖 2026-11-23 05:53 北京时间，21:53:45Z）。扫描时的 128 / 107 为旧快照。
- 状态：已找到（站点与 feed 为一手）；“Lean 检查”为站点自述，未独立核验 Lean 检查器的运行环境。

### C6 补充条目（不足 7 条，见下）

仅能取到以下两条可用的一手候选，其余条目经核查不在窗口内或不成立：

**C6a 斯坦福 AI100 2026 报告 “AI Goes Mainstream”（已找到，L1 机构报告，新闻性较弱）**
- 来源：`https://ai100.stanford.edu/sites/g/files/sbiybj36086/files/media/file/ai100_report_2026.pdf`（PDF，存本地不入库：`c-c6-stanford-ai100.pdf`，7.2 MB）；文本 `c-c6-stanford-ai100.txt`（376 KB，入库）。封面 “NOVEMBER 2026”，官方未给日期；HN 提交 2026-11-24 00:08 北京时间。
- 摘句（Key Takeaways 2，原句）：“It is already transforming the creative industries, mathematics, and software development, all fields that believed themselves beyond its reach.”
- 限定词：报告性质为学术综述，结论为其判断，不是新模型或事件发布。
- 热度：HN 50614164，1 分 / 0 评（检索时）。

**C6b OpenAI 状态页：11-23 付费 ChatGPT 上 GPT-6 对话间歇报错（minor，L1，已取得详情，影响范围仅限付费对话）**
- 来源：`https://status.openai.com/incidents/01M84HKHX73RF7HTVZQ2A9GFXP`（页面标题 “Elevated errors with GPT-6 conversations on paid ChatGPT plans”）。存档 `c-c6-openai-gpt6-incident-page.html`（486 KB HTML，本地保留）；状态 JSON `c-c2-openai-api-incidents.json`（同一事故，created 2026-11-23T15:50:14Z，resolved 空，impact minor）。
- 时刻（页面原文，时区按 API 时间戳校验为美东，见下）：“Mon, Nov 23, 2026, 10:50 AM” 开始调查，“10:58 AM” 确认原因，“11:12 AM” 进入 Monitoring。换算北京时间：11-23 23:50 开始，23:58 确认，11-24 00:12 进入监控；抓取时仍未标记 resolved。
- 时区校验：页面 10:50 AM 与 API 的 15:50:14Z 对应，故页面时间为美东时间（UTC-5，EST）。此为由 API 时间戳推出的推断，页面本身未写时区。
- 受影响范围（页面原文）：“1 affected component”，组件为 ChatGPT / Conversations。
- 原句：“We are investigating intermittent errors affecting GPT-6 conversations on paid ChatGPT plans.”（Investigating）；“We’ve identified the issue causing intermittent errors in GPT-6 conversations on paid ChatGPT plans. We are working on implementing a mitigation.”（Identified）；“We’ve applied a mitigation and are monitoring the affected services to confirm recovery.”（Monitoring）。
- 限定词：“intermittent”（间歇）、仅限 “paid ChatGPT plans”；页面未给根因、未给错误率、未提免费用户。
- 状态：部分支持（起止与范围为页面原文；根因与影响规模未公布）。新闻性弱，作备选。

**C6c ARTEX（已按协调者指示不再跟进，未收入候选）**
- 此前取得的 404 Media 存档 `c-c6-404-artex.html` / `.txt` 保留作记录。涉及美方机构点名“中国黑客”，按风控不纳入速览。

### C6 其余检索结果（均不收）

- 不在窗口内（北京 11-22 18:00 之后）：Google Gemini 4 Neon（11-18）、Genie 4（11-18）、Gemini 4 Carbon（11-10）、Gemini 4 Hydrogen（11-12）、AlphaProof Code（11-10）、Microsoft MAI Code 2 与 Code 2 Flash（11-17）、Waymo 高速 robotaxi 暂停（11-17，WebSearch 转述）、Inkling 2 Mini（11-20，Hugging Face 博客，且已在 1120 速览）、HF “harness adapters”（11-20）。
- 非 AI 或不收：Jensen Huang 复职 Nvidia CEO（风控，不收）；“First Claim of Artificial Super Intelligence”（自发白皮书，未核，HN 2 分，不收）；“Buckle Up the AI Bubble Is Popping”（评论文章，L6 性质，不收）；“Yes, 17M lines of Lean is a lot”、“Masullo on the Riemann Hypothesis”、“Treemap of the Riemann Hypothesis Lean Proof”（A 组）。
- 未能访问：AIHOT（`https://aihot.news/`）。诊断：`http://aihot.news/` 返回 302，跳转到 `block.charter-prod.hosted.cujo.io/warn.html`（网络过滤器拦截页）；`https://` 在 curl 中无响应、在 opencli 中报 `ERR_SSL_PROTOCOL_ERROR`。判断为本机网络层拦截，不是站点故障。按规则不绕过，11-23、11-24 精选未浏览，需 Develata 决定是否换网络或授权其他路径；Qwen 博客（`qwen.ai/blog`，正文过短，疑似 JS 渲染，未用 opencli 重取）；Meta AI 博客（HTTP 400）；xAI 新闻（HTTP 403）。因此 AIHOT 的 11-23、11-24 精选未能浏览，C6 的覆盖面有缺口。

## 二、扫描说法勘误

1. C2：2026-11-21 OpenAI 211 分钟故障、13:48–17:21 美西、峰值 75% 的说法，一手页面（`n254wyd7nml7`）是 2023 年 11 月 21 日事故（PT 1:48–5:21），2026 年状态 JSON 中无对应事故。原说法与页面时间不符，**与说法不符**。
2. C2：“中文稿把只读副本写成‘备用数据库’”——页面原文为 “read replicas”，“备用”一词在原文中未出现（与 2023 年事故相关，不适用于本期）。
3. C1：微信发布时刻为 17:40:42（扫描称 17:42）。
4. C1：扫描称“30B 实验模型上有对比分数”；正文文字中只见相对提升与输出长度下降 10%–14%，未见绝对分数（图表未存档）。
5. C3：事故页内两个影响时段不一致（创建 17:49 与影响起点 17:39 北京时间），须按原文注明。
6. C4：扫描称“微软随后公布可比数据”，未找到一手来源。
7. C5：扫描称首页“三次只取到标题区”，本组已用 opencli 取得计数（7,223 sessions / 316 agents / 738 of 1,306 lemmas）。扫描“128 分 / 107 评”为旧快照，现为 160 / 147。
8. C5：“no result here is a proof unless it says checked”一句在已存档的页面与 feed 中未找到，不得引用。

## 三、对读者

- 无可直接用于速览的“读者能做什么”类事实；C1 的 API 与开源为“即将”与“计划”，不可写成已开放。

## 四、待 Develata 决定

- C6c（ARTEX）：协调者已指示不再跟进，不收。
- C6b（OpenAI 11-23 GPT-6 付费对话报错）：新闻性弱，建议只作备选。

## 五、补充轮（协调者 11-24 追加，20 分钟内）

1. AIHOT：2–3 条补充未完成（见 C6 中“未能访问”的诊断）。
2. C6b：已取得 OpenAI 事故详情（见 C6b）。
3. 致谢名单：已另存删去致谢段与 @handle 的副本 `c-c6-stanford-ai100-redacted.txt`（删去 Acknowledgements 段；图片署名的两个 @handle 替换为 “@[handle已删]”；正文其他人名未动）。原文件 `c-c6-stanford-ai100.txt` 列入待删除清单。
4. 最终可用候选：强 C1、C3、C4、C5；弱 C6a（AI100）、C6b（GPT-6 付费对话报错）。强候选仍不足 7 条，AIHOT 被网络拦截未能补齐。C6c 按指示不跟。

## 六、文件清单（本组新增，均在 `docs/2611/1122/sources/`）

- 文本存档（可入库）：`c-c1-wx-glm54.txt`、`c-c1-zai-glm54.txt`、`c-c2-openai-incident.txt`、`c-c3-claude-incident.txt`、`c-c4-openai-perplexity.txt`、`c-c5-pequalnp.txt`、`c-c5-pnp-about.txt`、`c-c5-pnp-live.txt`、`c-c5-pnp-dashboard-0012.txt`、`c-c5-pnp-dashboard-opencli.json`（渲染快照加载失败的空壳，**待删除候选**，见下）、`c-c6-hn-front.txt`、`c-c6-hn-p2.txt`、`c-c6-hn-newest.txt`、`c-c6-openai-news.txt`、`c-c6-anthropic-news.txt`、`c-c6-deepmind-blog.txt`、`c-c6-google-ai-blog.txt`、`c-c6-mistral-news.txt`、`c-c6-hf-blog.txt`、`c-c6-nvidia-blog.txt`、`c-c6-gemini4-neon.txt`、`c-c6-genie4.txt`、`c-c6-gemini4-carbon.txt`、`c-c6-gemini4-hydrogen.txt`、`c-c6-alphaproof-code.txt`、`c-c6-ms-ai-news.txt`、`c-c6-stanford-ai100.txt`、`c-c6-404-artex.txt`、`c-c6-hn-…`等、`c-c6-ms-mai-code2.*`（与 c-c6-ms-ai-news 内容相同，重复）
- 结构化文件（可入库）：`c-c2-openai-api-incidents.json`、`c-c3-claude-api-incidents.json`、`c-heat.json`、`c-hn-50609685.json`、`c-hn-50601525.json`、`c-hn-50604314.json`、`c-hn-extra.json`、`c-fetch-log.jsonl`、`c-fetch-list-1..9.tsv`
- 本地不入库（PDF/HTML 原件，存本地）：`c-c6-stanford-ai100.pdf`（7.2 MB）及各 `.html` 原件、`.headers`。

## 七、待删除文件

- `docs/2611/1122/sources/c-c6-openai-gpt6-incident.json`（实为 Next.js 错误页 HTML，无用）。
- `docs/2611/1122/sources/c-c6-stanford-ai100.txt`（协调者指示：原文件含致谢名单个人 handle，已由 `c-c6-stanford-ai100-redacted.txt` 替代，原文件列为待删除）。
- `docs/2611/1122/sources/c-c5-pnp-dashboard-opencli.json`（当前内容为加载中的空壳，已被 `c-c5-pnp-dashboard-0012.txt` 取代）。
- `docs/2611/1122/sources/c-c6-ms-mai-code2.*`（与 `c-c6-ms-ai-news.*` 内容完全相同，重复抓取；待 Claude 确认后删除）。
- 其他 `c-fetch-list-*.tsv` 为清单记录，保留，不删。
- 保留本地不删：`c-c6-stanford-ai100.pdf`、`c-c6-openai-gpt6-incident-page.html`。
