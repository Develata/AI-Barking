# 1101 期 D 组取证：速览一（PG19 延期、OccupySaaS、Trigger.dev 安全事件）

执行方：Haiku 5.5（claude-haiku-5-5）。取证时间：2026-11-02 23:4x–23:5x（北京时间）。不截图（D 组按派工单不需要）。存档、抓取与热度的记录见 `capture-log-D.md`。

## 结论先行

- **PG19**：官方 RC1 页（L1）给出正式版计划日 **2026-11-19**（附“除非发现需要延期或另一个 RC 的问题”）；官方路线图页抓取版仍写 “planned for October 2026”，与 RC1 页冲突，以较新的 RC1 页为准（inferred）。“延期”一说来自委员 Robert Haas 的个人文章（L2）：“most likely won’t be released until November”。文章的提交数是作者自己统计的，原文自身有一处矛盾（见下）。
- **OccupySaaS**：官网问答写 “Not yet”，即未到生产可用；但 GitHub 仓库描述写 “1:1 drop in replacement feature parity”，而同仓库 README 写的是 “The goal is a 1:1…”。这是本条最大的吠点。
- **Trigger.dev**：官方博客与官方状态页（L1）一致：未经授权访问了“a subset of organizations”的环境变量，受影响数量、起始时间均未给。状态页在 2026-11-01 22:50 UTC（北京时间 11-02 06:50）标为 Resolved。

## 一、扫描说法勘误

| 扫描/线索说法 | 核对结果 | 处理 |
|---|---|---|
| D1 线索：“页面显示 Oct 21, 2026 与 Oct 26, 2026 两个日期，须分清发布与更新” | Neon 原文日期为 **Oct 21, 2026**（页面仅有日期，无时刻）。页面上的 **Oct 26** 属于页尾“More from Neon”的其他文章（Stripe 数据管道、Docs 搜索），不是本文更新。原文无更新标记。 | 按 Oct 21 记发布；不写“更新于 Oct 26”。 |
| D1 线索：“延到何时是作者预计还是官方日程” | 官方日程存在：RC1 页（2026-10-15）写 GA 计划日 2026-11-19，RC2 2026-11-12。Haas 的 “most likely … November” 是作者预计。 | 正文可写“官方计划 11 月 19 日”，但要带条件；不要把作者的预计写成官方日期。 |
| D1 线索：HN 50312408 “697 分 426 评” | 同一链接现查为 698 分 427 评（heat 接口 699/427；条目接口 698/427）。该 HN 链接指向 Haas 的 blogspot 转载页，不是 Neon 原页（Neon 原页的 HN 为 50281809，5 分）。 | 数字随时间变化，记现查值并注明时刻；链接以转载页为准并注明“转载”。 |
| D2 线索：HN 50331402 “428 分 268 评” | 现查 433 分 269 评（条目接口 432/268）。 | 同上。 |
| D2 线索：“百万被裁开发者是口号” | 官网有 “We are the 1,000,000” 与 “A million engineers were let go”，无来源。 | 同意定性为口号，不作事实。 |
| D3 线索：HN 50331784 “147 分 74 评” | 搜索接口 148 分，条目接口 147 分，评论 74。 | 记 147–148 分，并注明两接口不一致。 |
| D3 线索：“攻击者未经授权访问‘a subset of organizations’的环境变量，已阻断利用路径，已邮件通知受影响账号” | ✅ 与博客原文一致。 | 无需勘误。 |
| D1 线索：2025 同期 REL_18_STABLE 724 次、2026 同期 master 1443、REL_19_STABLE 1169 | ✅ 与 Neon 原文及 Haas 博客转载一致（作者自己的计数，见下）。 | 无需勘误。 |

## 二、条目

### D1 PostgreSQL 19 延期与 AI 找 bug

**一句事实（草稿）**：PostgreSQL 19 正式版官方计划于 2026 年 11 月 19 日发布，附“除非发现需要延期或另一个候选版本的问题”；委员 Robert Haas 在 Neon 博文中称 PG19 “most likely won’t be released until November”，并把延期归因于 LLM 找到的 bug 增多。

| 项 | 内容 |
|---|---|
| 官方 RC1 页（L1） | https://www.postgresql.org/about/news/postgresql-19-rc-1-released-3409/ → `sources/d-pg19-rc1.txt`（`.html` 同名） |
| RC1 原句 | “The planned date for the general availability of PostgreSQL 19 is November 19, 2026.”；“Unless an issue is discovered that requires a delay or an additional release candidate, PostgreSQL 19 should be made generally available on November 19, 2026.”；RC2 “on November 12, 2026”。 |
| RC1 发布时刻 | 页面 “Posted on 2026-10-15”，官方未给时刻，北京时间记 2026-10-15（日期）。 |
| 官方路线图（L1，与 RC1 冲突） | https://www.postgresql.org/developer/roadmap/ → `sources/d-pg-roadmap.txt`。抓取版原句：“The next major release of PostgreSQL is planned to be the 19 release. This release is planned for October 2026.”（无更新时间；抓取于 2026-11-02 23:49 北京时间）。判断：页面未更新（inferred），不据此写 10 月。 |
| 原文（L2，署名作者 Robert Haas，页内署 “Postgres Committer, Databricks”） | https://neon.com/blog/postgres-19-is-late-and-its-ais-fault → `sources/d-neon-postgres19.txt`/`.html`。页面日期 **Oct 21, 2026**（无时刻）。原句：“PostgreSQL 19 most likely won’t be released until November, and it’s not impossible that the project could decide to postpone the release further.” |
| 转载（L2） | http://rhaas.blogspot.com/2026/10/postgres-19-is-late-and-its-ais-fault.html → `sources/d-rhaas-blog.txt`/`.html`。页内标注 “Friday, October 30, 2026”，“11:53 AM”（博客时区未标，不换算），并注明 “This blog post originally appeared at https://neon.com/…”。HN 50312408 指向这个转载页。 |
| 提交数（L2，作者统计，未独立核对 git 仓库） | 原句：“Between April 8th, 2025 and September 25, 2025, there were 1037 commits to PostgreSQL’s master branch and 724 commits to REL_18_STABLE. In 2026, during the same date range, there were 1443 commits to the master branch and 1169 commits to REL_19_STABLE.” 状态：**部分支持**（数字来自作者，未独立复核）。 |
| 原文自相矛盾（需标注） | 同段紧接：“if you look at the number of commits that went into the master branch only between feature freeze and September 25th, it actually declined slightly from last year to this year, but the number of back-patched commits jumped by more than 60%.” 但给出的 master 数字是 1037 → 1443（增加），与 “declined slightly” 不符。“back-patched” 的 +60% 与 724→1169（+61.5%）算术一致。 |
| 归因（L2，作者判断） | 原句：“The short answer is that LLMs happened.”；“And I think that’s also why we had so many late reverts, and why the release itself is delayed.”；作者自承：“It’s possible that some of this jump is the result of committers making more mistakes this year than they did last year, but I don’t think anyone who works on the project really believes that’s the main cause.” |
| 未核 | 文中提到的三项大功能 revert（FOR PORTION OF、SQL/PGQ、MERGE/SPLIT PARTITION）及 beta 页（beta-4 页抓取超时，未重试）。 |

**热度（HN，2026-11-02 23:49 北京时间）**：50312408（转载页，10-31 09:38 北京时间）698 分 / 427 评；50281809（Neon 原页，10-29 06:26）5 分 / 0 评；50306359（转载页，10-31 00:34）4 分；50119991（The Register 报道，10-16 23:55）227 分 / 93 评；50334549（LWN，11-02 17:04）3 分；50309439（Phoronix，10-31 04:06）12 分。Reddit：查询失败（见 capture-log-D）。

### D2 OccupySaaS

**一句事实（草稿）**：OccupySaaS 自称“由被裁工程师用 AI 与公开文档做的 clean-room、Apache-2.0 开源替代”，首批两个项目为 ZeroAuthy（对标 Auth0 接口）与 SalesCompel（对标 Salesforce 接口），官网问答写明“Not yet”生产可用。

| 项 | 内容 |
|---|---|
| 官网（L1，项目自述） | https://occupysaas.com/ → `sources/d-occupysaas-home.txt`/`.html`。页头 “Est. October 2026 · Apache-2.0”（无日期）。 |
| 官网原句：生产可用 | “Is it production-ready? Not yet. Both first projects are early and say so plainly in their READMEs, along with exactly which features work today.” |
| 官网原句：方法 | “AI agents implement from the spec alone — contracts first (openapi.yaml), then code, then tests — under human review at every merge.” |
| 官网原句：口号 | “We are the 1,000,000” / “A million engineers were let go while the platforms kept growing.”（无来源，定性为口号。） |
| ZeroAuthy 仓库描述（L1，GitHub 页） | https://github.com/occupysaas/zeroauthy → `sources/d-occupysaas-zeroauthy-gh.html`/`.txt`。仓库 About：“A clean room recreation of Auth0 with 1:1 drop in replacement feature parity.” 同仓库 README 原句：“Status: early.”；“The goal is a 1:1, drop-in replacement for Auth0's Authentication API and Management API.” 状态：**与说法不符**（描述把目标写成既成事实；标“goal”的是 README）。 |
| SalesCompel 仓库描述（L1） | https://github.com/occupysaas/salescompel → `sources/d-occupysaas-salescompel-gh.html`/`.txt`。About：“A clean room recreation of Salesforce with 1:1 drop in replacement feature parity.” README 原句：“The candidate is not production-ready or certified Salesforce-compatible.” 状态：**与说法不符**（同上）。 |
| Clean-room 政策（L1，仓库文件） | https://github.com/occupysaas/zeroauthy/blob/main/CLEAN_ROOM.md → `sources/d-occupysaas-zeroauthy-CLEAN_ROOM.md`（raw 取得）。允许阅读 Auth0 开源 SDK 源码（“their source code may be read and studied”），并允许对 Auth0 Management API 做“Firewalled”观察（在自有租户上，走文档端点，记行为笔记）。官网列出的允许来源是“Public API docs / Open standards / Public SDK behaviour / Black-box test results”。两者都是合规陈述，但措辞范围不同（inferred，需正文按事实写，不替它下判断）。 |
| 仓库数据（L1，GitHub API / 页面） | ZeroAuthy：99 star，728 commits，Apache-2.0，首个提交 2026-10-13T18:06:47Z（北京时间 10-14 02:06），首个提交信息 “docs: add sub-project 1 design spec and clean-room policy”。SalesCompel：71 star，140 commits，Apache-2.0，首个提交 2026-10-15T02:51:20Z（北京时间 10-15 10:51），信息 “first commit”。website 仓库：3 star。（取数时刻 2026-11-02 23:49 北京时间。） |
| 组织页 | https://github.com/occupysaas（匿名抓取，无登录信息）。 |

**热度（HN，2026-11-02 23:49 北京时间）**：50331402（官网，11-02 08:17）433 分 / 269 评（条目接口 432/268）；50324966（11-01 20:26）6 分 / 4 评。Reddit：查询失败。

### D3 Trigger.dev 安全事件

**一句事实（草稿）**：Trigger.dev 称 2026 年 11 月 1 日发生安全事件，攻击者未经授权访问了“部分组织”的环境变量；已阻断所用利用路径，并已邮件通知受影响账号所有者。

| 项 | 内容 |
|---|---|
| 官方博客（L1，CTO 署名公告） | https://trigger.dev/blog/security-incident-nov-2026 → `sources/d-trigger-incident.txt`/`.html`。页面标题 “Security Disclosure: Sunday November 1st”，页内标 “Incident report · November 1, 2026”（官方未给时刻）。原句：“There has been a security incident at Trigger.dev. We've blocked the exploit used in this attack, but our investigation has identified exposure across a subset of organizations. The attacker gained unauthorized access to environment variables for these organizations. We have emailed all impacted account owners with steps to mitigate.” |
| 状态页事件（L1） | https://status.trigger.dev/incident/1110434 → `sources/d-trigger-incident-status.txt`/`.html`。状态：Resolved，时刻 “Nov 1, 2026 at 10:50pm UTC”（北京时间 2026-11-02 06:50）。原句：“The attacker gained unauthorized access to environment variables for these organizations.”；“We will follow up with more details.”。受影响服务：API。 |
| 状态页总览（L1） | https://status.trigger.dev → `sources/d-trigger-status.txt`。页面 “Last updated on Nov 2, 2026 at 3:49pm UTC”（抓取时刻）。 |
| 未给 | 受影响组织数量；攻击起始与持续时间；被访问的变量类型与具体数量；攻击入口。博客与状态页均未给。 |
| 背景（L1，不要混为本事件） | https://status.trigger.dev/incident/1083239 → `sources/d-trigger-incident-oct06.txt`。2026-10-06（UTC 12:21–15:10，北京时间 20:21–23:10）事件：“Some sensitive environment variables synced from Vercel were cleared”，原因为 Vercel 集成同步时把敏感环境变量覆盖为空值。官方未说明两事件是否相关（不作推断）。 |
| 博客作者在社交平台的转发（L2，署名） | CTO 同样内容的社交平台帖（链接在博客页上，平台名不进正文）。内容与博客一致，不单独采用。 |

**热度（HN，2026-11-02 23:49 北京时间）**：50331784（博客，11-02 09:09）148 分 / 74 评（条目接口 147/74）；50338312（theainewsreport 转述，11-02 23:38）1 分。Reddit：查询失败。

## 三、可能的吠点（D 组简录，仅线索，L6）

- PG19：作者的因果链“LLM 找到 bug → 修复慢 → revert 多 → 延期”是作者判断，文中没有给出各 revert 与 LLM 发现的 bug 的逐条对应。文中提交数表述矛盾（见上）。HN 评论 50332078（2 回复）质疑：“Is no one entertaining the possibility that some contributions were lower quality due to AI?”（https://news.ycombinator.com/item?id=50332078）。注意：HN 评论接口不给评论分数，这里只能按回复数排序，不能说“高赞”。
- OccupySaaS：仓库描述与 README 的状态对比（见 D2）是最硬的吠点。HN 高回复评论以观点为主（例 50332355：“There’s already tons of FOSS alternatives…”，31 回复；50335535：指出这些人被裁前也在实施锁定，17 回复），都是观点，不计事实。
- Trigger.dev：状态页在博客未给受影响数量与起止时间的同时把事件标为 Resolved；HN 评论 50333043：“It’s bold to set status page incident to resolved.”（18 回复，https://news.ycombinator.com/item?id=50333043）；50333116：“We exposed your env vars but we told you about it over email … We dont know how or to what extend though.”（7 回复，https://news.ycombinator.com/item?id=50333116）。两条都是社区观点，线索级（L6）。

## 四、待核或未找到

- PG19 “三大功能 revert 的原因”一手依据：未核（beta 页抓取超时，未重试）。
- PG19 提交数：未独立核对 git.postgresql.org 计数。
- OccupySaaS 的“1:1”：未找到项目方对“goal”与“description”不一致的更正记录（未检查 Issues）。
- Trigger.dev：未找到受影响数量与起始时间的任何一手数字。

## 五、各条级别汇总

| 条目 | 一手来源级别 | 说明 |
|---|---|---|
| D1 官方计划日 2026-11-19 | L1 | 官方 RC1 页，有附带条件 |
| D1 “most likely … November”、提交数、归因 | L2 | 委员个人署名文章（Databricks 博客转载），作者计数 |
| D1 官方路线图 “October 2026” | L1（过时） | 与 RC1 页冲突，以 RC1 页为准（inferred） |
| D2 项目官网、仓库描述、README、CLEAN_ROOM | L1 | 项目方自述；“1:1”是描述，“goal”是 README |
| D2 仓库 star、commits、首次提交 | L1（GitHub 原始数据） | 2026-11-02 23:49 北京时间取数 |
| D3 博客、状态页 | L1 | 公司官方博客与官方状态页 |
| 全部热度数字 | HN 原始接口（英文社区热度） | 非一手，仅记热度 |

## 六、交付清单

- 新增文件（本组）：`docs/2611/1101/sources/evidence-D.md`、`capture-log-D.md`；存档 `d-*`（`.html` 原件、`.txt`、`.headers`、`.json`），脚本 `d-hn-digest.py`，热度 `d-heat.json`（Reddit 查询失败）、`d-heat-hn-run1.json`（HN 首次取数）。
- 不入库的 HTML 原件：`d-*.html`（派工单规定只存本地）。
- 已超过 1 MB 的文本存档：无（最大 `d-rhaas-blog.html`，98 KB；HN 原始 JSON 最大 247 KB；均低于 1 MB）。
