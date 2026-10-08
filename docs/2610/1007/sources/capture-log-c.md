# 1007 C 组抓取日志（Anthropic 扩大 Cyber Verification Program）

时间一律北京时间（UTC+8）。本机时区 EDT（UTC-4），本机 2026-10-07 即北京 2026-10-08；抓取时刻不等于新闻发布时刻。取证窗口：北京 2026-10-08 01:23–02:00（本机 10-07 13:23–14:00）。

## 一、逐次抓取记录

机器可读版：[c-http-records.tsv](c-http-records.tsv)（第 1 列为 UTC，北京时间 = +8 小时；列依次为 UTC 时刻、文件、URL、HTTP 状态、最终 URL、字节数、类型）、[c-browser-records.jsonl](c-browser-records.jsonl)（Playwright 浏览器读取/截图）。**HTTP 200 不等于拿到有效正文**，以“判定”栏为准。

### 1.1 curl（普通浏览器 User-Agent，匿名）

首行（`c-cvp-page.html`）在建立台账文件之前抓取，不在 c-http-records.tsv 里；其余各行与该文件一一对应。

| 北京时间 | URL | 文件 | 工具 | HTTP | 判定 |
|---|---|---|---|---|---|
| 10-08 01:23:34 | https://www.anthropic.com/news/cyber-verification-program | `c-cvp-page.html`、`c-cvp-page.txt` | curl | 200 | **主源**。正文完整；与 01:38 浏览器渲染文本逐句比对一致（差别仅为本脚本加的标题/列表标记），`article:modified_time` 未变（2026-10-07T08:52:51Z）。 |
| 10-08 01:24:35 | https://www.anthropic.com/glasswing | `c-glasswing.html` | curl | 200 | 一手：Glasswing 页，含 4/7 公告与 10/6 新增条目。 |
| 10-08 01:24:38 | https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude-opus-and-sonnet | `c-help-cvp.html` | curl | 200 | 一手：帮助中心 CVP 条文（URL 已被站方重定向到 14604842-cyber-verification-program）。 |
| 10-08 01:24:42 | https://claude.com/blog/how-comcast-booz-allen-use-claude-mythos-to-secure-their-codebases | `c-claude-blog-booz-comcast.html` | curl | 200 | 一手：claude.com 博客（被重定向到 /resources/articles/…），Comcast/Booz Allen 案例，2026-10-06。 |
| 10-08 01:24:46 | https://www.anthropic.com/news/enterprise-frontier-safeguards | `c-efs.html` | curl | 200 | 一手：Enterprise Frontier Safeguards 公告（2026-09-01）。仅作背景。 |
| 10-08 01:24:49 | https://portal.anthropic.com/programs/cvp | `c-portal-cvp.html` | curl | 200 | **失败/不可用**：重定向到登录页（portal.anthropic.com/login?returnTo=…），保存的是登录页，不含 CVP 申请内容；只在登录态可见，不另存、不用。 |
| 10-08 01:24:54 | https://support.claude.com/en/articles/16764810-assign-a-program-to-workspaces-in-claude-console | `c-help-assign.html` | curl | 200 | 一手：控制台“分配 Program 到工作区”帮助页，仅背景，未引用。 |
| 10-08 01:24:57 | https://www.reuters.com/legal/litigation/anthropic-opens-its-most-powerful-ai-models-more-security-teams-2026-10-06/ | `c-reuters.html` | curl | 401 | **失败**：curl 返回 HTTP 401（774 字节挑战页）。正文改用 firecrawl_scrape 取得，见 c-reuters.md。 |
| 10-08 01:24:59 | https://www.anthropic.com/legal/aup | `c-usage-policy.html` | curl | 200 | 一手：Usage Policy（页面标“Effective September 15, 2025”）。 |
| 10-08 01:26:07 | https://www.anthropic.com/research/glasswing-initial-update | `c-glasswing-initial-update.html` | curl | 200 | 一手：Project Glasswing: An initial update（2026-05-22）。 |
| 10-08 01:26:10 | https://www.anthropic.com/news/expanding-project-glasswing | `c-expanding-glasswing.html` | curl | 200 | 一手：Expanding Project Glasswing（2026-06-02）。 |
| 10-08 01:26:12 | https://thehackernews.com/2026/10/anthropic-expands-claude-access-for.html | `c-hackernews-thn.html` | curl | 200 | L4：The Hacker News。 |
| 10-08 01:26:15 | https://www.helpnetsecurity.com/2026/10/07/anthropic-expands-cyber-verification-program/ | `c-helpnetsecurity.html` | curl | 200 | L4：Help Net Security。 |
| 10-08 01:26:18 | https://www.theregister.com/security/2026/10/07/anthropic-reconfigures-its-cool-kids-security-program/5301509 | `c-theregister.html` | curl | 200 | L4：The Register。 |
| 10-08 01:26:23 | https://siliconangle.com/2026/10/06/anthropic-folds-project-glasswing-into-an-expanded-three-tier-cyber-verification-program/ | `c-siliconangle.html` | curl | 200 | L4：SiliconANGLE。 |
| 10-08 01:26:28 | https://qz.com/anthropic-cyber-verification-program-expansion-three-tiers-100626 | `c-qz.html` | curl | 200 | L4：Quartz。 |
| 10-08 01:27:49 | https://www.anthropic.com/news/claude-opus-4-7 | `c-opus-4-7.html` | curl | 200 | 一手：Opus 4.7 发布（2026-04-16），CVP 首次出现。 |
| 10-08 01:27:52 | https://www.anthropic.com/claude-opus-5-5 | `c-opus-5-5.html` | curl | 200 | 一手：Opus 5.5 发布页（含“即将扩大 CVP”句）。 |
| 10-08 01:28:01 | https://www.irregular.com/research/cyscenariobench | `c-irregular-cyscenariobench.html` | curl | 200 | L3（基准方）：Irregular 的 CyScenarioBench 方法页（2025-12-05，页面自称 working draft）。 |
| 10-08 01:28:05 | https://www.irregular.com/research/assessing-claude-opus-5.5-against-offensive-security-benchmarks | `c-irregular-opus55.html` | curl | 200 | L3（基准方）：Irregular 对 Opus 5.5 的评估（2026-09-22）。 |
| 10-08 01:30:40 | https://web.archive.org/web/20260810203405id_/https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude-opus-and-sonnet | `c-wayback-help-cvp-20260810.html` | curl | 200 | web.archive.org 2026-08-10 20:34:05 UTC 快照（--compressed 解压），仅用于查看帮助页“以前写了什么”。快照 CDX 间歇返回“Temporarily Offline”，只取到这一份。 |
| 10-08 01:32:33 | https://www.donews.com/news/detail/8/6731971.html | `c-zh-donews.html` | curl | 200 | 中文媒体：DoNews 快讯（页面自带“内容由智能模型自动生成”免责声明）。 |
| 10-08 01:32:38 | https://post.smzdm.com/p/am93nzrk/ | `c-zh-smzdm.html` | curl | 200 | 中文：什么值得买“今日 AI 圈动态”汇总帖。 |
| 10-08 01:32:48 | https://hao.cnyes.com/post/271301 | `c-zh-cnyes.html` | curl | 200 | 中文（台湾）：钜亨号 hao.cnyes.com 作者帖。 |
| 10-08 01:32:54 | https://www.toast.com.cn/news/2026-10-06-anthropic-%E6%8A%8A%E7%BD%91%E7%BB%9C%E9%AA%8C%E8%AF%81%E8%AE%A1%E5%88%92%E6%89%A9%E4%B8%BA%E4%B8%89%E6%A1%A3%E5%90%91%E9%98%B2%E5%BE%A1%E6%96%B9%E5%BC%80%E6%94%BE%E6%9B%B4%E5%B0%91%E6%8A%A4%E6%A0%8F%E7%9A%84-claude | `c-zh-toast.html` | curl | 200 | 中文：拓实科技。 |
| 10-08 01:32:57 | https://www.ithome.com.tw/news/179470 | `c-zh-ithome-tw.html` | curl | 200 | 中文（台湾）：iThome。 |
| 10-08 01:33:00 | https://www.epochtimes.com/b5/26/10/7/n14865175.htm | `c-zh-epochtimes.html` | curl | 200 | 中文（繁体）：大纪元。 |
| 10-08 01:33:03 | https://www.technology.org/2026/10/07/anthropic-cyber-verification-program-three-tiers/ | `c-technology-org.html` | curl | 200 | L5：technology.org。 |
| 10-08 01:33:09 | https://www.reddit.com/r/SecOpsDaily/comments/1wzsn8f/anthropic_expands_claude_access_for_vetted_cyber/ | `c-reddit-secopsdaily.html` | curl | 200 | **失败**：Reddit 返回 403 阻断页（8 KB 的 HTML 壳），无帖子内容。 |
| 10-08 01:33:47 | https://www.ithome.com/1/010/127.htm | `c-zh-ithome.html` | curl | 200 | 中文（大陆）：IT之家，转述路透。 |
| 10-08 01:33:50 | https://finance.sina.com.cn/stock/usstock/c/2026-10-07/doc-iniuirnx5226808.shtml | `c-zh-sina.html` | curl | 200 | 中文（大陆）：新浪财经环球市场播报。 |
| 10-08 01:33:51 | https://www.163.com/dy/article/L8JVK29705527PMU.html | `c-zh-163.html` | curl | 200 | 中文（大陆）：网易号（IT时代网）。 |
| 10-08 01:34:22 | https://www.securityweek.com/anthropic-introduces-3-tier-cyber-verification-program-for-ai-access/ | `c-securityweek.html` | curl | 200 | L4：SecurityWeek。 |
| 10-08 01:34:25 | https://securityaffairs.com/200521/ai/anthropic-creates-three-tiers-for-claude-cyber-access.html | `c-securityaffairs.html` | curl | 200 | L4：Security Affairs。 |
| 10-08 01:34:29 | https://www.mahmudhasan.pro/blog/anthropic-cyber-verification-program-expansion | `c-mahmudhasan.html` | curl | 200 | L5：个人博客。 |
| 10-08 01:34:33 | https://finin2min.com/finnews/economy-policy/anthropic-cyber-verification-program-129000-vulnerabilities-october-2026/ | `c-finin2min.html` | curl | 200 | L5：Finin2min。 |
| 10-08 01:34:37 | https://www.unite.ai/anthropic-expands-cyber-verification-program-to-three-access-tiers/ | `c-unite-ai.html` | curl | 200 | L5：Unite.AI。 |
| 10-08 01:34:40 | https://aiweekly.co/alerts/anthropic-opens-three-tier-cvp-folding-in-project-glasswing | `c-aiweekly.html` | curl | 200 | L5：aiweekly.co。 |
| 10-08 01:34:43 | https://valueaddvc.com/pulse/anthropic-cyber-verification-program-expansion-2026 | `c-valueaddvc.html` | curl | 200 | L5：Value Add VC Pulse。 |
| 10-08 01:34:47 | https://bregg.com/blog/anthropic-cyber-verification-program-expansion-three-tiers-healthcare-2026-10-07 | `c-bregg.html` | curl | 200 | L5：bregg.com。 |
| 10-08 01:35:29 | https://www.163.com/dy/article/L8LL22LC0511A5GF.html | `c-zh-163-safe.html` | curl | 200 | 中文（大陆）：网易号“安全圈”。 |
| 10-08 01:35:29 | https://thebotpost.com/ai-news/anthropic-reported-claude-chat-to-police-florida-woman-arrested | `c-botpost.html` | curl | 200 | L5：The Bot Post 另一篇文章页（用来找到下一行文章的链接）。 |
| 10-08 01:35:49 | https://thebotpost.com/ai-news/anthropic-glasswing-129000-vulnerabilities-cyber-verification-program-claude-mythos | `c-botpost-glasswing.html` | curl | 200 | L5：The Bot Post “Claude Found 129,000 Software Bugs …”。 |
| 10-08 01:36:01 | https://www.vulncheck.com/blog/anthropic-glasswing-cves | `c-vulncheck-cves.html` | curl | 200 | L3/L5：VulnCheck 博客（2026-04-15）。 |
| 10-08 01:36:04 | https://www.vulncheck.com/blog/anthropic-glasswing-receipts | `c-vulncheck-receipts.html` | curl | 200 | L3/L5：VulnCheck “Glasswing Receipts”（页面无发布日期；THN 称“上月底”，页内称“约 5 个月大”）。 |
| 10-08 01:36:36 | https://www.vulncheck.com/blog/state-of-exploitation-1h-2026 | `c-vulncheck-1h2026.html` | curl | 200 | L3/L5：VulnCheck 1H-2026 State of Exploitation（页面引用日期 2026-07-28）。 |
| 10-08 01:36:40 | https://develata.me/news/AI_ML/2026/20261007 | `c-develata-news-20261007.html` | curl | 200 | 线索页：Develata 新闻页 2026-10-07。grep verification/glasswing/cyber 无命中（页面首条是与本题无关的内容），记“未见该条”。 |
| 10-08 01:41:54 | https://www.csoonline.com/article/4231812/anthropic-widens-access-to-ai-cyber-capabilities-for-vetted-security-teams.html | `c-cso.html` | curl | 200 | L4：CSO Online（含 IDC 分析师评语）。 |
| 10-08 01:51:21 | https://www.anthropic.com/claude-opus-5-5-system-card | `c-opus55-system-card.pdf`（17,795,106 字节，230 页，仅本地）、`c-opus55-system-card.txt`（`pdftotext -enc UTF-8 -layout`） | curl | 200 | 一手：Opus 5.5 System Card（封面 September 22, 2026）；§3.3.2 CyScenarioBench 在第 51 页（67.6%），图注在第 52 页。链接取自 Irregular 页与 Opus 5.5 发布页。 |

### 1.2 官方图表原图（Anthropic CDN，curl，仅存本地，不入库）

| 北京时间 | URL | 文件 | 说明 |
|---|---|---|---|
| 10-08 01:24:09–01:24:17 | https://www-cdn.anthropic.com/images/4zrzovbb/website/ 下 2b1d3817…-1920x2322.png、bdb05797…-1920x1080.png、172894c3…-1920x862.png | `c-assets/` 下同名 PNG | 正文三张图（档位总表 / CyScenarioBench 柱状图 / Glasswing survey results 表）的原图，用于核对数字与目视；配图仍以页面截图为准。 |
| 10-08 01:25:43 | 帮助中心档位总表图（support.claude.com 的 intercomcdn 带签名链接，签名不记） | `c-assets/help-cvp-tiers.png`、`c-assets/help-tiers-url.txt` | 首次取到的是 256×256 的聊天图标 SVG（正则匹配错第一张 img），按 width=1693 重取成功。该图与新闻页档位总表的差别见 evidence 的 C3。 |

### 1.3 firecrawl / 搜索 / API

| 北京时间 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 约 01:27 | Reuters 报道正文 | firecrawl_scrape（maxAge 0） | 成功，statusCode 200，完整正文，存为 [c-reuters.md](c-reuters.md)；页面元数据 `article:content_tier` 为 `metered`，抓取结果无付费墙提示，**未做任何绕过**。curl 直连同一 URL 为 401（见上表）。 |
| 01:25–01:44 | 若干 firecrawl_search / Exa 搜索 | firecrawl_search、web_search_exa | 仅用于发现报道与社区讨论，结论一律回到页面存档核对；搜索摘录不当作存档。 |
| 01:31–01:44 | Hacker News Algolia API（search_by_date 与 items）、Firebase item | curl / Python urllib | 成功：`c-hn-q-*.json`、`c-hn-item-*.json`、[c-hn-searches.json](c-hn-searches.json)。HN 不公开评论分数。 |
| 01:43 | Reddit：r/SecOpsDaily 帖子 JSON、站内搜索 JSON | curl | **失败**：HTTP 403，`c-reddit-secopsdaily.json`、`c-reddit-search.json`（各 189,908 字节）实为 Reddit 阻断页 HTML，不是数据；`c-reddit-secopsdaily.html`（8 KB）同为阻断壳。firecrawl_scrape 对 reddit 返回 “we do not support this site”。没有用镜像或代理绕过。 |
| 01:30–01:44 | web.archive.org CDX 与快照 | curl | 部分成功：CDX 查询可用，取到 2026-08-10 20:34:05 UTC 的旧版帮助页快照（`c-wayback-help-cvp-20260810.*`）；archive.org 可用性接口 429，随后若干请求返回 “Temporarily Offline”。 |
| 01:3x | opencli / agent-reach | opencli doctor；opencli daemon restart | **失败**：Daemon 正常，Extension 未连接；按派工单做了一次 daemon restart，仍未连接，按连接失败规则停止，没有改适配器或扩展。`agent-reach doctor` 显示 twitter-cli 配置了凭据，但其验证会在失败时自动读取浏览器 Cookie，**未执行 twitter 读取**。所以 X 上的 Anthropic 原帖与浏览量未取得（见 evidence 的 C6）。 |

### 1.4 匿名 headless Chrome（Playwright Core，CSS 宽 700、DPR 2、临时 profile，无登录态）

脚本：[c-browser.mjs](c-browser.mjs)（锚点文字定位后按元素上下沿裁切，只裁切不改像素）；各次参数见 `c-shots-*.json`；全部记录见 c-browser-records.jsonl。页面 DOM 文本/元数据另存 `c-cvp-news.{json,txt}`、`c-help-cvp-browser.{json,txt}`、`c-irregular-browser.{json,txt}`。`c-html2txt.py` 是把存档 HTML 抽成文本的辅助脚本（只用标准库）。

| 北京时间 | 页面 | 输出 |
|---|---|---|
| 01:38:22 | https://www.anthropic.com/news/cyber-verification-program | 25、26、27、28、29 |
| 01:39:01 | 同上（补截 clean 版） | 30 |
| 01:40:10 | https://support.claude.com/en/articles/14604842-cyber-verification-program | 31、32、33 |
| 01:40:48 | https://www.irregular.com/research/assessing-claude-opus-5.5-against-offensive-security-benchmarks | 34 |

另有两次只读布局/整页预览（`--layout`、`--full`），整页图写在会话临时目录，不在仓库。

## 二、截图 QA（均已目视核对）

宽度全部 1400 px（CSS 700 × DPR 2）。无登录态，截图中没有抓取者信息，也没有浏览器扩展图标。没有重绘、没有改数字、没有用转载图。

| 文件 | 内容 | 判定 |
|---|---|---|
| 25-c-cvp-numbers.png | “Giving defenders the advantage” 标题 + 129,000 / 5,500 / 33,000 / “at least five times higher” 段 + “months or even years” 段 | 可用。 |
| 26-c-cvp-survey-table.png | “Project Glasswing survey results” 官方表（副标题 “Based on a survey of 33 partners plus open-source code scanned by Anthropic”）+ 图注（lower bound、33 partner reports、fewer than 50%） | 可用；表的全部行列与列头在内。 |
| 27-c-cvp-cyscenario-text.png | “Testing the efficacy of our tiers” 全段：10 挑战 × 5 次、三档结果、67.6% | 可用；上沿有少量空白。 |
| 28-c-cvp-cyscenario-chart.png | CyScenarioBench 柱状图（标题、副标题 “Solve rate on Opus 5.5 based on Cyber Verification Program tier”、y 轴、四根柱）+ 官方图注 | 可用。 |
| 29-c-cvp-tier-table.png | 档位总表 | **不使用**：上沿切进上一段末行（露出 “form.” 的字顶），违反“裁切边界落在文字行之间”；按不删除规则保留，30 替代。 |
| 30-c-cvp-tier-table-clean.png | “Below, we share an overview…” 引导句 + 档位总表（Who it’s for / Uses / Requirements / Account types）+ 图注 | 可用。 |
| 31-c-help-overview.png | 帮助中心页标题、“Updated October 2026” 说明框、Overview 两段（含 “reduced blocking classifiers”） | 可用。页面写 “Updated today”，指抓取当日。 |
| 32-c-help-how-to-apply.png | How to apply：七个工作日、个人申请、“Tier C”、所需材料三项 | 可用。 |
| 33-c-help-once-approved.png | Once you’re approved（Usage Policy 仍完全适用、可审查/收窄/撤销）+ Security controls（12 月 15 日期限） | 可用。 |
| 34-c-irregular-opus55-67-6.png | Irregular 对 Opus 5.5 的评估：Testing Configuration（含 “cyber mitigations disabled”）与 Overall Assessment（67.6%、61.7%、53.0%、<1%） | 可用，但**缺页面标题与日期**（上沿从分享栏起，H1 与 “September 22, 2026” 不在图内）；日期以文字存档为准。34 是 C 组最后一个编号，无法再补一张。 |

候选配图对应：129,000/33,000/5× 段 = 25；“33 partner reports / <50% patched” 段 = 26；CyScenarioBench 两档结果段 = 27（图表 28）；CVP 申请页关键段 = 31–33；67.6% 出处 = 34；档位总表 = 30。

## 三、未取得 / 失败项（如实）

1. Reuters：curl 401；正文由 firecrawl 取得（见上）。没有浏览器截图（未尝试，不绕过挑战页）。
2. CVP 申请入口 portal.anthropic.com/programs/cvp：重定向到登录页，只在登录态可见，不当证据；`c-portal-cvp.html` 是登录页，不是申请页内容。
3. Reddit 全部失败（403），无法给出 Reddit 帖子分数；X/Twitter 未读取（opencli 扩展断开；twitter-cli 会自动读 Cookie）。Anthropic 官方 X 帖与浏览量未取得。
4. AIHOT（aihot.news）：curl 只拿到 988 字节 JS 壳（`c-aihot-home.html`），未用浏览器渲染，无法确认是否收录；Develata 当日页已存档，无本题（见 evidence 的 C6）。
5. BleepingComputer、The Record：用 firecrawl_search 限定域名检索未命中本事件，**不等于它们没报道**（检索覆盖有限）。
6. CVP 旧公告：没有单独的“CVP 发布公告”页面；首次出现于 Opus 4.7 发布页（2026-04-16）和旧版帮助页。旧版帮助页只取到 Wayback 2026-08-10 一份快照，未取得 9 月的版本。
7. 帮助中心 “CVP Security Requirements” 文章：未抓取（C 组清单未要求，帮助页只是链接）。
8. CyScenarioBench：没有 Anthropic 自己的基准说明页；基准方 Irregular 有方法页（working draft）与 Opus 5.5 评估页，已存档。
9. VulnCheck “2/300 在野利用”的原始出处：THN 转述 `2 of the 300`，The Register（10-07 稿）转述 `fewer than 0.5 percent of the 225`。补查一次（Exa）：二者看来是 Garrity 追踪表在不同日期的快照（225 条 → 300 条），但只拿到搜索摘录（含 Kitploit 转载的追踪表 README 摘录），GitHub 原页与 VulnCheck 对应博文未取到，**不存档、只作线索**；按派工单状态记“未找到一手来源”。已存档的 VulnCheck 三页（`c-vulncheck-*.txt`）不含 2/300 这一数字。

## 四、已知的瑕疵与处理

- `c-cvp-page.txt` 等文本由 `c-html2txt.py` 抽取，带 `#` 标题标记和 `- ` 列表标记；逐字摘句以原文为准，已用脚本把 evidence 里的逐字引文逐条回查（见终检记录）。
- 其他组并行写同一目录；本组只创建 `c-` 前缀文件与 25–34 号图。`c-reddit-*.json`、`c-portal-cvp.html` 是失败/无效内容，按“不删除”规则保留，不代表证据。

## 五、终检记录

1. **逐字引文回查**：`python c-verify-quotes.py` 把 evidence-c.md 反引号内 167 段（≥25 字符）逐字摘句回查到本目录存档文本（空白归一化、HTML 实体反解、去掉本脚本加的 `#`/`- ` 标记）。未命中 6 段，全部是**图内文字**（官方图表 “True positive vulnerabilities” ×3、图表副标题 “Based on a survey of 33 partners plus open-source code scanned by Anthropic”、总表 Specialized Access 的 “Who it’s for” 一格、“Identity verification, security attestations”），无法对文本回查；已目视核对 25–34 号截图，并对原图/截图做了 tesseract 抽查（“True positive vulnerabilities”、副标题、“critical safety systems” 与帮助页图的 “critical infrastructure” 均读到）。其余引文全部命中。
2. **隐私 grep**（范围：本目录 c- 前缀文件）：关键词 develata0 / Develata / 本机用户路径 / Bearer / auth_token / ct0 / sessionid。命中均属预期：`c-develata-news-20261007.*` 与 `c-http-records.tsv` 中的 URL 是 Develata 的公开新闻页（线索页，不是抓取者凭据）；`c-zh-*` 里的 “QQ” 是网页分享按钮文字；`c-opus55-system-card.txt` 里的 news.qq.com 是系统卡的参考链接；`c-browser.mjs` 里只有 `os.homedir()`，没有用户名。`signature=` 只出现在 `c-help-cvp.html`（站方页面自带的 intercom 签名素材 URL，是被抓页面内容，不是抓取者凭据；`c-assets/help-tiers-url.txt` 里已把签名替换为占位符）。没有抓取者头像、显示名、handle；社交平台没有存档。**范围说明**：上述 grep 只检查了 C 组 c- 前缀文件；对 `docs/2610/1007/` 其他组文件（a-doctronic PDF、b-wayback HTML 等）用同一模式 grep 也有关键词命中，我未检查，属他组范围，不能据此说整个目录已清洁。
3. **截图**：25–34 全部存在，宽 1400；29 号不用（见第二节）；34 号缺页标题（见第二节）。均已目视核对。
4. **文件体积**：可入库的文本文件最大为 `c-opus55-system-card.txt`（467 KB）；`c-reddit-search.json`、`c-reddit-secopsdaily.json`（各 189,908 字节）是 Reddit 403 阻断页，**不是数据**，建议不入库（单文件均 < 1 MB，无需另问）。`c-opus55-system-card.pdf`（17.8 MB）、全部 HTML、`c-assets/` 与 images 下 PNG 被 .gitignore 排除，仅存本地，后续传网盘；大小与 SHA-256 见 [c-manifest.tsv](c-manifest.tsv)。
5. **工作区**：`git status --short` 只多出 `docs/2610/1007/`（含他组文件）；未改动任何已跟踪文件；没有 commit、push、删除；`docs/2610/1005/sources/c-repo-tree.json` 未触碰。
6. **状态用词**：evidence-c.md 每条清单行状态均为 已找到 / 部分支持 / 与说法不符 / 未找到一手来源 四种之一（含括号说明）。
