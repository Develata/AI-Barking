# 1011 B 组抓取日志（Grok Bot 私人 CFO 智能体把银行余额发进公司 Slack）

时间均为北京时间 UTC+8，取证日 2026-10-11（本机 EDT 03:50–04:45，仍是 10-11 清晨；北京 = 本机 + 12 小时）；分钟级为近似。**HTTP 200 不等于拿到正文**，下列“结果”均在存档后检查过内容。执行方模型 claude-sonnet-5-5。

## 工具与方式

- HTTP：`curl -sS -L`（匿名，普通 Chrome UA）；转文本用 `b-html2txt.py`（1009 同名脚本的副本，未改）。docs.x.ai 的 markdown 版（`<page>.md`）直接下载存入 `b-xai-docs-md/`。
- 渲染截图：`b-browser.mjs`（1009 同名脚本改写：路径 1011、图号区间 13–24、新增 `VH` 环境变量控制视口高度；Playwright Core + 本机 Chrome 无头，每次新临时 profile，不登录、不带 Cookie；CSS 宽 700（文档/BI）或 550（X 嵌入），DPR 2）。台账：`b-browser-records.jsonl`。X 帖子卡片用 X 官方嵌入页 `platform.twitter.com/embed/Tweet.html?id=…`（匿名渲染）。
- X 帖子 JSON：`https://api.fxtwitter.com/status/<id>`（公开 JSON，不含抓取者信息）；存档时把所有 `location` 字段替换为 `[omitted-by-archiver]`（Mac 的公开资料里有所在城市）。
- HN：Algolia（`search`、`items`）。
- 搜索：WebSearch（4 次）、firecrawl_scrape 1 次（仅 WSJ，取可见部分；额度快用完）。
- 未使用 opencli、agent-reach，未使用任何会读取本机浏览器 Cookie 的工具。未登录任何站点，未接受任何 Cookie 弹窗，未发帖/点赞/评论/关注。
- 辅助脚本：`b-docs-cmp.py`（把今日 docs.x.ai markdown 逐句与 1007 期存档比对，输出 `b-xai-docs-diff-vs-1007.txt`）。

## 有效抓取

| 文件基名 | URL（已去查询参数） | 工具 | 结果 | 时间 |
|---|---|---|---|---|
| b-bi.html/.txt | https://www.businessinsider.com/personal-ai-agent-grok-bot-posted-bank-details-company-slack-2026-10 | curl | 200，430 KB，全文（含 “available exclusively to Business Insider subscribers” 横幅，服务器仍返回了完整正文）；datePublished 2026-10-09T08:43:28.186Z | 15:51 |
| b-bi-early-users.html/.txt | https://www.businessinsider.com/early-users-delete-personal-ai-agents-privacy-scares-blunders-2026-10 | curl | 200（BI 10/6 相关文，仅备查） | 16:00 |
| b-xai-docs-security / overview / grokbot-launch / news / bot（.html/.txt） | https://docs.x.ai/grok-bot/security 、…/overview 、https://x.ai/news/introducing-grok-bot 、https://x.ai/news 、https://x.ai/bot | curl | 全部 200，正文完整（html2txt 后 21 KB / 8.9 KB / 9.6 KB / 17.5 KB / 10.5 KB） | 15:52 |
| b-xai-docs-md/*.md（22 个） | https://docs.x.ai/grok-bot/<page>.md ：overview get-started use-cases mobile bots team-bots chat-and-collaboration tag-on-x files-and-results computer-and-apps skills-routines-and-automations settings-and-notifications approvals-security-and-privacy teams-and-enterprises identity-and-access private-networks proxies computers security security-faq troubleshooting faq | curl | 全部 200 | 15:55 |
| b-xai-docs-llms.txt、b-xai-docs-sitemap.xml、b-xai-release-notes.md | https://docs.x.ai/llms.txt 、https://docs.x.ai/sitemap.xml 、https://docs.x.ai/developers/release-notes.md | curl | 全部 200；sitemap 提供各页 lastmod | 15:56 |
| b-xai-changelog-bot.html/.txt、b-xai-changelog-bot-rss.xml | https://x.ai/changelog/bot 、https://x.ai/changelog/bot/rss.xml | curl | 200，646 KB / 72 KB；最新 v0.68.1（2026-10-07） | 16:08 |
| b-cursor-changelog / b-cursor-bot / b-cursor-blog | https://cursor.com/changelog 、https://cursor.com/bot 、https://cursor.com/blog | curl | 200；changelog 最新 Oct 6, 2026；cursor.com/bot 只有 79 字符空壳；blog 无相关条目 | 15:59 |
| b-cybernews / b-futurism / b-runtimewire / b-playtech / b-explainx / b-cellcog / b-techtimes / b-t3n（.txt；.html 已删除） | https://cybernews.com/ai-news/grok-bot-leaks-bank-information/ 、https://futurism.com/future-society/ai-agent-posted-bank-balances-spending-slack 、https://runtimewire.com/article/xmtp-shane-mac-grok-bot-bank-audit-slack 、https://playtech.ro/2026/un-ceo-din-tech-si-a-conectat-contul-bancar-la-grok-bot-apoi-si-a-vazut-finantele-personale-pe-slack/ 、https://explainx.ai/blog/grok-bot-bank-balance-slack-viral-post-disputed-agent-permissions-lessons-2026 、https://cellcog.ai/blog/grok-bot-personal-cfo-slack-leak/ 、https://www.techtimes.co.uk/ceo-says-grok-bot-posted-bank-balances-slack-1809001 、https://t3n.de/news/ki-teilt-finanzen-ceo-unternehmen-slack-1767258/ | curl | 200；正文完整（二手，只作线索；explainx、cellcog 为带推销性质的聚合/博客，按 L5） | 16:01–16:30 |
| b-cybernews-connect-bank / b-pymnts-grok-banks / b-thepaypers | https://cybernews.com/ai-news/grok-bot-connect-bank-acccount/ 、https://www.pymnts.com/news/artificial-intelligence/2026/grok-puts-the-ai-assistant-between-consumers-and-their-banks/ （存档用 `?p=4241868` 入口）、https://thepaypers.com/fintech/news/grok-links-to-plaid-to-access-us-users-financial-data | curl | 200（Finance integration / Plaid 背景，二手） | 16:15 |
| b-xmtp-home、b-redpoint-xmtp、b-web3career-xmtp | https://xmtp.org/ 、https://www.redpoint.com/companies/xmtp 、https://web3.career/i/wIzMwQTM | curl | 200；xmtp.org 首页无团队/CEO 信息 | 16:32 |
| b-wsj-visible.txt | https://www.wsj.com/tech/ai/ai-agents-assistants-shopping-chat-errors-a316d3f9 | firecrawl_scrape | 200，只拿到标题、副题、作者、时刻与首两段（页面 article.access=paid，付费墙未绕过）；我把可见部分手工摘入 txt 并标注 | 16:10 |
| b-fx-<id>.json（15 个） | api.fxtwitter.com/status/<id>：2107486740491669879、…742613794932、…744249589952、…746325725669、…749396185231、…751770186184（Mac 线程）；2107608239638466681、2107609102759198991、2107831230121070851、2107548030467731933（相关回复）；2107495338873389121（Musk 10/6）、2092692116610957814、2092594798234620084（Musk/Teslaconomics 8/26）、2103936247995752705（@bot Finance）、2095891115689824500（Plaid）；b-fx-profile-ShaneMac.json（用户资料） | curl | 全部 200 | 16:02–16:35 |
| b-hn-50033517.json、b-hn-search-story.json | Algolia items/50033517 与 search | curl | 200；帖 61 分 / 66 评论（取数 16:05） | 16:05 |
| 截图 13 14 15 | BI 文页 | Playwright | 保存；已逐张目视确认。为压缩版面，注入 CSS 隐藏了广告位容器（`.in-post-sticky,[data-bi-ad],.ad-wrapper,.ad-callout-wrapper`）、BI 站内“Explore BI Games / Want more Business Insider”推广块与文首头图（Mac 的肖像照）；正文文字、标题、署名、时间行未动。13 号中标题/署名与时间行之间有一段留白，是被隐藏的头图位置 | 16:13 |
| 截图 18 19 22 23 | docs.x.ai 的 computer-and-apps、approvals-security-and-privacy、chat-and-collaboration | Playwright（`VH=5000/6000`，视口加高以避开文档站的内部滚动容器） | 保存；目视确认。19 号含“Sign in once”与“Connect an app”两节；23 号含“Start a group chat”“Direct a message”与群聊段 | 16:16–16:18 |
| 截图 20 21 16 24 | X 嵌入页 | Playwright | 20：首帖文字部分（y 0–342）；21：线程第 2–3 条（回复卡自带上文）；16：线程第 4–5 条（含 Mac 与 Bot 的聊天截图）；24：@bot Finance 帖。均目视确认 | 16:22–16:30 |

## 失败 / 未取到 / 放弃

- **Wayback Machine**：CDX 查询可用，快照列表显示 docs.x.ai 的 computer-and-apps 有 2026-09-21 与 2026-10-02 快照，chat-and-collaboration 有 09-25、10-02、10-03 快照，approvals 有 10-07 快照，security 有 09-27 至 10-07 的多份快照；但取 `web.archive.org/web/<ts>id_/…` 全部返回 429（“suspected abusive bot traffic”，620 B）。先后两轮共 7 次请求（一轮连发 6 次，一轮间隔约 75 秒重试 4 次），都 429；按规则不换镜像站。试验文件已删除，留 `b-wayback-429.log`。**因此没有 10/1 前后的官方文档基线，B2 无法 diff computer-and-apps 与 chat-and-collaboration。**
- **x.ai 对无头 Chrome 返回 Cloudflare “Sorry, you have been blocked”**（Playwright 与 Chrome CLI 各一次，同一网段）；curl 同一 URL 返回 200 全文，docs.x.ai 无头 Chrome 正常。未做规避（未换指纹、未用有头模式）。因此 x.ai/changelog/bot 只有文本存档（curl），没有截图；原计划 16、17 号截 changelog，改用 Mac 线程卡片，changelog 的 Sep 30 与 Oct 2–7 条目只在 evidence 里逐字摘。内置浏览器窗格可打开该页，但窗格截图无法落盘，也未用。
- **WSJ**：curl 401（DataDome 验证页，774 B，已删）；内置浏览器按安全限制不能打开 wsj.com；firecrawl 只拿到可见部分。付费墙未绕过。
- **NewsBytes**（https://www.newsbytesapp.com/news/science/xmtp-labs-ceo-shane-mac-disconnects-services-after-ai-mishap/tldr）：curl 403，已删。
- **Reddit**：curl 403（反爬页）；WebSearch 工具不允许 reddit.com 域。未取。
- **cursor.com/bot**：只返回 79 字符空壳；论坛 forum.cursor.com 的 Plaid 连接器帖为 JS 渲染，只拿到样式，已删。
- **xmtp.org/about、/team、blog.xmtp.org/about、/team**：404，已删。
- **@bot、Grok Bot 署名员工的 X 时间线**：fxtwitter 无时间线接口，未取。
- **Mac 线程中被推文引用的另一条 `shanemac/status/2107576521250533437`**：fxtwitter 404。
- **中文媒体报道**：两次中文关键词搜索无中文来源。

## 试验与删除

- `b-wayback/` 目录（429 页面）：已删，429 日志另存。
- `b-wsj.html/.txt`（DataDome 验证页）、`b-newsbytes.*`（403）、`b-xmtp-about/team.*`、`b-cursor-forum-plaid.*`、`b-hyperai.*`（取到后判断与本事件无关）、`b-xai-docs-root.*`（与 overview 重复）、`b-cellcog.html`/`b-techtimes.html`/`b-explainx.html`（保留 txt）：均为我本组生成的试验文件，已删。
- 图号：先后生成并删除过 16-mac-8-40am-two-hours-card.png、17-mac-different-agent-slack-card.png（嵌入卡自带上文，含 Mac 附图的 Slack 截图，里面有部分遮盖后的账户数字与同事账号，不应入库）；20 号也因此只截文字部分。
- `b-browser-records.jsonl` 里含若干 FAILED 记录（Playwright 视口过矮导致 clip 越界；b-bi 首次截图头图未隐藏），按实际保留。

## 自查（交付前）

- `git status --short`：新增只有 `docs/2610/1011/`（含其他组文件）与派工单；本组文件均带 `b-` 前缀或为 `evidence-b.md`/`capture-log-b.md`，图 13–16、18–24（17 缺）。
- 对 `docs/2610/1011/sources/b-*` 与 evidence-b/capture-log-b 检索本机用户名、邮箱、本机路径：无命中。fxtwitter JSON 的 `location` 已占位；embed 渲染使用匿名 profile，未登录。
- 单个文本文件超过 1 MB：无（最大 `b-xai-changelog-bot.html` 646 KB、`b-bi.html` 430 KB，均为 HTML 原件，不入库）。
