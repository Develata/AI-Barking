# 1114 期 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区，2026-11-15），取自 `a-fetch-log.jsonl`、`a-shots-log.jsonl` 与终端时间。文件前缀 `a-`。未登录任何账户，未提交表单，未发帖。`opencli doctor` 通过（extension 已连接）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 20:54:12 | 研究页英文/德文/索引、Lovable 11-10 博客与博客索引、条款、HN 条目页、Handelsblatt（旧 slug）（`a-list1.tsv`） | `barking fetch` | 7 ok；**Handelsblatt 旧 slug 页 blocked（正文 12 个非空白字符）**，`a-handelsblatt.*` 无 `.txt`，改用新 slug（见下）。研究页 `Last-Modified` 2026-11-03T22:05:17Z |
| 20:54:34 | HN 条目 50499017 | `barking heat --item`（前缀 `a`） | 成功，866 分 / 317 评（嵌套评论数）；`a-heat.json`、`a-hn-50499017.json`（Algolia 评论树，318 节点，**不含评论得分**） |
| 20:55:00 | HN `--hn "Lovable"`、`"Lovable apps expose passport"`、`"crissyfield"`；Reddit 两条 | `barking heat`（前缀 `a-hn`，因 `a-heat.json` 已存在） | HN 成功（主帖 866 分 / 338 评，11-15 01:43 北京）；**Reddit 两条搜索被重定向到登录页，未取**，记入 `a-hn-heat.json` errors |
| 20:55 | HN Algolia `users/neathack`、`search?query=crissyfield.de` | curl（`--max-time 30`） | 账号 karma 299、about 为空；该域名 4 条提交：50338419（neathack，2026-11-02T15:45:10Z，2 分）、50348900（taubek，11-03T06:54:34Z，5 分）、50499017（armcat，11-14T17:43:10Z）、15665879（neathack，2017，Kickbase 旧文） |
| 20:5x | 网络检索（Exa `web_search_exa`，三组：英文、德文、中文关键词） | Exa | 仅作线索；产出报道清单与 Handelsblatt 现用 slug、Jansen LinkedIn 帖、LILOX、Cybernews、Sifted 等 |
| 20:55:43 | Cybernews、Sifted、Techzine、RuntimeWire、SaaS Feeds、Yuri Island、OmniTools、PrivacyAffairs、LILOX、Kernelpost、FourWeekMBA、Trending Topics、LinkedIn（`a-list2.tsv`，清单末尾因 `printf` 报错被截断） | `barking fetch` | RuntimeWire / SaaS Feeds / youlidao / OmniTools / PrivacyAffairs / Kernelpost / Trending Topics ok；**Cybernews、Sifted、LILOX（Cloudflare 质询 403）、Techzine、FourWeekMBA（403）blocked**，未绕过；LinkedIn 清单项因 URL 被截断为 404 |
| 20:55:54 | LinkedIn 帖（`--force` 重取）、Handelsblatt 新 slug、四个 Lovable 营销页、docs.lovable.dev 安全文档、lovable.dev/security、Supabase changelog（`a-list3.tsv`） | `barking fetch` | 9 条全 ok。`a-handelsblatt2.txt` 仅含匿名可见部分（`isAccessibleForFree: False`），**未绕过付费墙**；页面 `Last-Modified` 2026-11-13T10:24:02Z；`a-linkedin-jansen.*` 为匿名公开帖视图（含登录入口，无访客标识）。`a-handelsblatt2.html`（约 0.97 MB）、`a-supabase-chg.html`（约 1.0 MB）只存本地 |
| 20:58:51 | crissyfield 2017 旧文、公司团队页、NVD CVE-2025-48757、Supabase 讨论 4547、UpGuard 博客（`a-list4.tsv`） | `barking fetch` | 团队页、Supabase 讨论、UpGuard 列表页 ok（`a-crissy-team.txt` 确认创始人 Dr. Thomas Jansen）；**旧文 404（URL 为猜测）、NVD blocked（需 JS）**，未再取 |
| 20:59 | 中文稿 pasqualepillitteri、unternehmensportal、DrWeb（`a-list5.tsv`） | `barking fetch` | 3 条 ok |
| 21:0x | Cybernews、LILOX、Sifted、Techzine | `opencli browser extract`（真实 Chrome 窗口 background；Cybernews 手动一次，其余用本组脚本 `a-opencli-save.py`） | Cybernews 取到全文（`a-cybernews.txt`）；LILOX、Techzine 取到（`a-lilox-opencli.txt`、`a-techzine-opencli.txt`）；**Sifted 仅标题与导语可读，正文付费墙后是乱码，未读**（`a-sifted-opencli.txt`）。未遇到验证码页（遇到则停止，未发生） |

被拒或失败：Handelsblatt 旧 slug（blocked）、Cybernews/Sifted/LILOX/Techzine/FourWeekMBA 的直接抓取（blocked，其中前四个改用 opencli 浏览器取到）、NVD（blocked）、crissyfield 2017 旧文（404）、LinkedIn 首次（URL 截断导致 404，已 `--force` 重取成功）、Reddit 搜索（登录重定向）。**未查**：Lovable 官方 X 账号与 Reddit r/lovable（无匿名检索接口；登录态内容不存档），UpGuard 原文全文（只拿到博客列表页，未定位到文章页）。

## 截图

渲染来源：`barking shot render`（headless Chrome，`BARKING_CHROME` 指向 Chrome 应用二进制，700 CSS px、DPR 2，匿名无登录）；裁切用 `barking shot cropbox` / `stack`（只裁切，不改像素）。HN 截图为匿名视图，显示的用户名均为公开账号名，页面右上 “login” 字样为未登录标记，无抓取者信息。

| 文件 | 来源页 | 说明 |
|---|---|---|
| `01-crissy-title.png` | 研究页 | 标题与导语（1400×400）；**页面无署名/日期** |
| `02-crissy-numbers.png` | 研究页 | 方法流程图 + 159,041 / 34,545 / 24,664 / 10,560 / 7,403 / 30%（1400×680） |
| `03-crissy-limitations.png` | 研究页 | Limitations 两段，下沿带出下一节标题 “Data Leaks”（1400×790） |
| `04-crissy-supabase-only.png` | 研究页 | Methodology 首段：只查 Supabase、自家数据库未检查（1400×390） |
| `05-crissy-forum-case.png` | 研究页 | Data Leaks 标题 + 论坛案例（1400×350）；**标题块顶边被裁掉约一两像素行（标题色块上沿贴边）**，正文完整 |
| `06-crissy-passport-case.png` | 研究页 | 物业、护照、其他内容、28 万封邮件、“could not establish whether … misused”（1400×710） |
| `07-crissy-responsibility.png` | 研究页 | “The Promise”：营销 vs 条款责任，含 “Lina Knees reported … November 2, 2026”（1400×850） |
| `08-lovable-blog-title.png` | lovable.dev 博客 | “Published November 10, 2026” + 标题；**深色底页面**（1400×370） |
| `09-lovable-blog-still-need-coming.png` | lovable.dev 博客 | “What we still need you to do” + “What's coming this month”；**深色底页面**（1400×1240） |
| `10-hn-title-author.png` | HN 条目页 | 标题行与点数（867 分 / 338 评，截图时刻约 20:57）+ 作者自述评论；两段拼接，中间的橙色条是拼接底色（`stack` 的 `--bg auto`）；下沿 “reply” 字样被裁一半（1400×334） |
| `11-hn-neathack-4000.png` | HN 评论 50499661 | 作者引原文 30% 句 + “nearly 4,000 websites” + “15,9%”（1400×715） |
| `12-hn-lovable-igor.png` | HN 评论 50502105 | Lovable 安全负责人的回应（1400×520） |

过程说明：`05`–`07` 第一次裁切边界切在文字行中间（论坛案例半行、Handelsblatt 行半行），重裁后用 `mv -f` 覆盖了本组自己的同名文件（`barking shot` 拒绝覆盖，先裁到 `tmp-05/06/07.png` 再 `mv`）；`a-shots-log.jsonl` 里因此有 `tmp-05.png`、`tmp-06.png`、`tmp-07.png` 及首次 05–07 的记录，以最后一次为准。编号区间 01–12 用满，夸大标题实例只截了 HN 标题（10），其余实例仅存档文本。未截 Handelsblatt（付费墙）。

## 自查

- 对 `docs/2611/1114/` 下本组文件（a- 前缀与 images/01–12）grep 过抓取者标识：无头像链接、昵称、handle、邮箱泄露（LinkedIn 页存档为匿名视图）。
- 评论得分不可得：Algolia 评论树与匿名 HN 页面都不显示。

## 假设

- 把 `neathack` 视为“自称的作者”，不证实身份；`igorandri` 视为“自称的 Lovable 安全负责人”。
- 北京时间换算：UTC+8；Handelsblatt 页面时间 “03.11.2026 - 16:39 Uhr” 按 JSON-LD 的 UTC 值 15:39Z 换算。
