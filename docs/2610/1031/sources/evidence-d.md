# 1031 期 D 组取证：速览一

执行方：claude-haiku-5-5（D 组取证，不截图）。取证时刻：北京 2026-11-02 02:32–02:37（本机 EST 11-01 13:32–13:37）。派工单：`.handoff/2026-10-31-1031-evidence.md` D 组。

状态用词：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。逐字摘句保留原文措辞与引号。存档：`sources/d-*.txt`（正文文本）、`sources/d-*.html`（原件，仅本地）、`sources/d-*.headers`、`sources/d-hn-*.json`、`sources/d-heat.json`、`sources/d-fetch-log.jsonl`。

## 各条一句事实（速览用，待 Claude 定稿）

- **D1（HackerTube）**，L2：Karpathy 称约两小时做出 HackerTube，视频取自 HN 评论推荐，共 6,454 个。
  - 注：原文主题为 "YouTube" 克隆。YouTube 受平台名规则约束不入正文与卡片，本句以“视频”代称，措辞待 Claude 定；“约两小时”为作者说法，见勘误 1。
- **D2（JS 框架成本）**，L2：Nolan Lawson 称 Opus 5.5 为 12 个框架各做同一应用，按 API 标价中位 $2.67–$4.97。
  - 注：“按 API 标价”保留；作者称并非实际支出（Max 订阅）。
- **D3（Leo de Moura）**，L2：de Moura 称预计未来几年，人们会更关注规格说明、模型与软件应有的性质。
- **D4（Ternus）**，L4：据 WIRED 报道，苹果 CEO Ternus 称“不为 AI 而推出 AI”，同时表示会用 AI 改进产品。
  - 注：引号内为译文，原句 "We don't want to ship AI for AI's sake."；“会用 AI”对应 "We’re absolutely going to leverage AI to do that."

## 取证表

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| D1-1 | 作者用 Claude 做出以 HN 推荐视频为内容的站点，约 6,454 个视频、150 个主题 | L2（作者个人博客署名发言；与机构的关系未核） | https://karpathy.bearblog.dev/hacker-tube/ | "In total, we get 6,454 YouTube videos that have been recommended by a few people on HN from 2007-2026 across 150 topics." | 无（D 组不截图） | 2007–2026 年 HN 评论；数字为作者自报，未独立核验；hackertube.org 站点本身未访问 | 已找到 |
| D1-2 | 视频来自 HN 历史评论推荐 | L2 | 同上 | "narrowed everything down to 40K comments recommending a YouTube video" | 无 | 作者说明：先由 Opus 拉取 HN 评论，再用 Sonnet 在 Workflow 中过滤 | 已找到 |
| D1-3 | 作者称约两小时做成 | L2 | 同上 | "This takes ~2 hours now. I can breathe again. Thank you Opus 5.5 <3." | 无 | 原文只写 "This takes ~2 hours now"，未明写“做成用时”；同文另写 "I worked on this in between Claude's actual work"，用时口径不清 | 部分支持 |
| D1-4 | 模型与配置（扫描未收） | L2 | 同上 | "I use Opus 5.5 on Medium effort as my default." | 无 | 默认 Medium effort；另有 Sonnet 参与筛选（见 D1-2） | 已找到 |
| D1-时刻 | 博文日期 | L2 | 同上 | "31 Oct, 2026" | 无 | 官方未给时刻；正文称 "this morning (Saturday)" | 已找到 |
| D2-1 | Opus 5.5 以 high effort 为 12 种框架各实现同一 Pokémon 应用 | L2（作者个人博客） | https://nolanlawson.com/2026/10/28/the-ai-cost-of-javascript-frameworks/ | "I simply gave the spec to Opus 5.5 with high effort and told it to use the latest version of the given framework." | 无 | 表格共 12 行；每框架取 3–5 次中位数 | 已找到 |
| D2-2 | 中位成本：Hono+htmx $2.67、Next.js $3.14、SvelteKit $4.28、Angular $4.97 | L2 | 同上 | 表格行 "Hono+htmx $2.67 (2.23–3.19)"、"Next.js $3.14 (2.58–4.13)"、"SvelteKit $4.28 (3.59–4.65)"、"Angular $4.97 (3.19–6.82)"；表注 "Median of 3-5 runs." | 无 | 计价口径：原文 "this quotes the API prices directly from Claude Code and doesn't actually represent how much money I spent."；作者用 Max 订阅："I’m on a Max plan, so this was just part of my overall $100/month." | 已找到 |
| D2-3 | 各框架区间大幅重叠，不宜只看中位数排序 | L2 | 同上 | "React Router’s median is roughly $1 worse than that of Hono+htmx, but the cheapest version of React Router ($2.82) is still cheaper than the most expensive version of Hono+htmx ($3.19)." | 无 | 作者原话，随后称 "some reliable patterns do emerge" | 已找到 |
| D2-4 | Angular 失败次数（扫描未收） | L2 | 同上 | "The exception was Angular, which failed to create a working app twice, so I took 3 iterations instead." | 无 | 两次未产出可用应用（作者称 SSR 版拒绝非 localhost 访问，判为未遵守 spec，未重跑）；Angular 取 3 次中位 | 已找到 |
| D3-1 | Leo de Moura 访谈中谈 AI 与形式化验证，重心放在规格说明 | L2（本人署名访谈；页面称其为 AWS VP 与 Lean FRO 首席架构师） | https://thedataexchange.media/leo-de-moura-lean-aws | "But we expect, in the next few months and years, people will focus way more on the specifications, on the models, on the properties you want the software to have." | 无 | 文字稿为 "a polished and edited transcript"（页面原句），逐字为编辑稿，非录音逐字 | 已找到 |
| D3-2 | 扫描称"证明对象是规格说明" | L2 | 同上 | "Now, with formal verification, you can start with the properties. You can, for example, write the properties down and ask the AI to implement a piece of code that satisfies these properties, and prove it to me." / "And AI now is fully capable of doing that." | 无 | 原文无"证明对象是规格说明"这一表述，属转述；原意是“从性质出发，让 AI 写满足性质的代码并证明” | 部分支持 |
| D3-时刻 | 访谈发布时刻 | L2（访谈页；发布方为 The Data Exchange / Gradient Flow） | 同上 | 页面文字日期 "October 29, 2026October 21, 2026"（两日期并列）；HTML meta `article:published_time` 与 JSON-LD `datePublished` 均为 "2026-10-29T11:00:24+00:00" | 无 | 换算为北京 2026-10-29 19:00:24；页面另一日期 "October 21, 2026" 不在 meta 中，义未明，留待 Claude 判断 | 已找到 |
| D4-1 | 苹果 CEO John Ternus 称"不为 AI 而加 AI" | L4（WIRED 专访，据页面署名 Steven Levy） | https://www.wired.com/story/i-spent-30-minutes-with-apples-ceo-now-i-know-why-john-ternus-got-the-job/ | "We don't want to ship AI for AI's sake." | 无 | Ternus 本人原话，经 WIRED 转述；为引语，非 WIRED 评论 | 已找到 |
| D4-2 | 扫描称"产品价值看用户体验" | L4 | 同上 | "What matters is, did we build something that they really love?" | 无 | "用户体验"为转述，原文无该词；同段紧接 "We’re absolutely going to leverage AI to do that." | 部分支持 |
| D4-时刻 | 专访发表时刻 | L4 | 同上 | 页面显示 "Oct 30, 2026 11:00 AM"；JSON-LD `datePublished` "2026-10-30T11:00:00.000-04:00" | 无 | 按 -04:00 换算为北京 2026-10-30 23:00 | 已找到 |

## 扫描说法勘误

1. D1-3（"作者称借 Claude 约两小时做成"）：原文只有 "This takes ~2 hours now."，无"做成用时"的明写，且作者同时称是在 Claude 实际工作之间做的。正文若写，需限定为"作者称约两小时"。
2. D1 HN 数字：扫描称"1846 分 477 评"（Claude 现查），`barking heat` 于北京 11-02 02:32 取得 **1878 分、454 条嵌套评论**。时刻不同；"评论数"两边口径不同（heat 数的是嵌套评论树，HN 的 descendants 字段本次未取），本档不取舍，留待 Claude 按统一口径重取。
3. D1 口径遗漏：作者称筛选用 Sonnet（"ran Sonnet over them in a Workflow"），effort 为 Medium（"Medium effort as my default"）。扫描未收。
4. D2 Angular：扫描未提 Angular 两次失败未计入，只取 3 次中位（原文 "failed to create a working app twice, so I took 3 iterations instead"）。
5. D2 口径：扫描称"按 API 标价估算、作者实际用订阅"与原文相符；原文更具体，"doesn't actually represent how much money I spent"，并注明 Max 套餐。
6. D2 区间：各框架区间大幅重叠（如 Next.js 2.58–4.13），扫描只给中位数。
7. D3 用语："证明对象是规格说明"不是访谈原话，属扫描的转述。
8. D4 用语："产品价值看用户体验"中的"用户体验"为转述；原文是 "What matters is, did we build something that they really love?"。扫描漏掉同段 "We’re absolutely going to leverage AI to do that."，与"不为 AI 而加 AI"并存。
9. D3 HN 数字与扫描相符（208 分、52 条）。D2 HN 数字：扫描 211 分、145 条；heat 现为 217 分、145 条。
10. D4 HN 热度：扫描未提；本次 HN 无该文投稿（见下表）。

## 热度（北京时间）

| 条目 | HN 条目 | 分数 | 评论 | 提交时刻（HN created_at，UTC → 北京） | 扫描原数 |
|---|---|---|---|---|---|
| D1 HackerTube | 50319510 | 1878 | 454（嵌套） | 2026-10-31T19:56:53Z → 11-01 03:56:53 | 1846 分 477 评（Claude 现查） |
| D2 JS 框架成本 | 50323088 | 217 | 145（嵌套） | 2026-11-01T06:06:19Z → 11-01 14:06:19 | 211 分 145 评 |
| D3 Leo de Moura | 50290652 | 208 | 52（嵌套） | 2026-10-29T15:29:48Z → 10-29 23:29:48 | 208 分 52 评 |
| D4 Ternus（WIRED） | 无投稿 | — | — | 北京 11-02 02:36 检索：以 WIRED 文章 URL 检索 0 命中；以 "Ternus" 按时间检索最近 20 条，最晚为 10-19 的 Time 文，无该文 | — |

热度来源：D1–D3 为 `barking heat` 的 HN 条目接口（`d-hn-*.json`，汇总于 `d-heat.json`）；D4 为 HN Algolia 检索的两次一次性请求（`d-hn-search-ternus.json`、`d-hn-search-ternus-wired.json`）。HN 的 created_at 为投稿时刻，不是原文发布时刻。

## 发布时刻汇总

- D1：2026-10-31（官方未给时刻；博文页 "31 Oct, 2026"）。
- D2：2026-10-28（官方未给时刻；博文页 "Posted October 28, 2026"）。
- D3：2026-10-29 19:00:24 北京（meta/JSON-LD `2026-10-29T11:00:24+00:00`）；页面另有 "October 21, 2026"，义未明。
- D4：2026-10-30 23:00 北京（WIRED JSON-LD `2026-10-30T11:00:00-04:00`；页面显示 "Oct 30, 2026 11:00 AM"）。

## 未找到 / 未核

- 本组 4 条均有一手来源，无“未找到一手来源”条目。
- D1 作者所说"约两小时"的用时口径：原文不足以定论（勘误 1）。
- D1 hackertube.org 站点与 HN 社区质疑：未访问（D 组不需要）。
- D4 付费墙：正文可经普通请求读取，但未做浏览器目视核验，是否对一般访客显示付费弹窗待核。

## 备注（给 Claude）

- `sources/d-wired-ternus.html`（1.45 MB）与另三个 `.html` 原件只存本地，不入库，请记入 `offsite.tsv`。
- `d-*.txt`、`d-hn-*.json` 均小于 1 MB。
- `sources/d-heat-items-prev.json` 不存在（曾短暂改名，已恢复为 `d-heat.json`，内容未变）。
