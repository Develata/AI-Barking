# 1115 期 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区，2026-11-16 至 11-17），取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json` 与终端时间。文件前缀 `b-`。未登录任何账户、未提交表单、未发帖。`opencli doctor` 通过（extension 已连接，v1.0.27）。macOS 上 `barking shot` 用 `BARKING_CHROME` 指向 Chrome 应用二进制。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 23:59:00 | iter.ca 文章；GHSA 公告；Releases 1.75.4、1.76.0、1.76.4 与列表页；AMO 页与 AMO API v5；Chrome 应用商店页；NVD 关键词接口；GitHub 全局公告 API（`b-list1.tsv`） | `barking fetch` | 11 条 HTTP 200 / ok。注意：`ghadv`（`/advisories?affects=ublock`）只有 5 字节（空数组），无信息；`cws`（Chrome 应用商店）正文空壳，`b-cws.txt` 仅导航；`amo` 的 `.txt` 为服务端渲染正文（版本栏可见）。`b-iter-post.headers` 的 `Last-Modified`: Mon, 16 Nov 2026 13:55:49 GMT |
| 00:00 | GHSA 仓库级 API `repos/gorhill/uBlock/security-advisories/GHSA-cmqw-9r2w-6wv2` | curl（`--max-time 30`，匿名） | 成功，`b-ghsa-api.json`：`cve_id: null`、`severity: high`、`published_at` 2026-11-15T16:27:56Z、`updated_at` 2026-11-16T13:25:44Z、`vulnerable_version_range` “1.75.2 and lower”、`patched_versions` “1.75.4” |
| 00:00 | GitHub 全局公告 API `advisories/GHSA-cmqw-9r2w-6wv2` 与 `advisories?ghsa_id=…` | curl | 404 / `[]`（全局库无条目） |
| 00:00 | uBlock 仓库 releases API（`b-releases-api.json`）；7 个相关 commit（`b-commit-<hash>.json`：bf69cf7、9859477、b1f7d27、150df12、cbb22a5、5466554、73ef1b2） | curl（匿名 GitHub REST） | 全部成功 |
| 00:0x | uBOL-home releases / commits / security-advisories API（`b-ubol-releases-api.json`、`b-ubol-commits-api.json`；advisories 返回 `[]`） | curl | 成功 |
| 00:00:24 | HN 条目 50513160（Algolia） | `barking heat --item`（前缀 `b`） | 506 分；含嵌套评论 133 条；`b-heat.json`、`b-hn-50513160.json`（不含评论得分） |
| 00:00:32 | HN 搜索 “Universal XSS in uBlock Origin”；Reddit 搜索 | `barking heat`（前缀 `b-hn`） | HN 命中 1 条（506 分 / 146 评，11-16 10:04 北京）；**Reddit 搜索重定向到登录页，未取**，记入 `b-hn-heat.json` errors |
| 00:0x | CWS 更新检查接口 `clients2.google.com/service/update2/crx`（`prodversion=140.0`，两次：无 `v` 与 `v=0.0.1`） | curl | 两次都返回 `status="noupdate"`（`b-cws-updatecheck.xml`）；未得版本号 |
| 00:0x | AMO 页 | opencli（真实 Chrome，background，zoom 2，未登录；已隐藏浏览器翻译插件的 shadow DOM 图标） | 取到 “Version 1.76.4 / Last updated a day ago (Nov 16, 2026)”，截图 `23-amo-version.png` |
| 00:0x | Chrome 应用商店页 | opencli；`barking shot render`（headless） | opencli：“The extensions gallery cannot be scripted.”（拒绝，未绕过）；headless 渲染为空白。**版本号未取到** |
| 00:0x | Exa 检索（英文、中文关键词，共三组） | Exa `web_search_exa` | 仅作线索；产出 TSN、PithWire、Dr. Web、fxxz、DEV Community、AGI Hunt、Alto、探How、memedata、VN Read、freedom.tech 等 |
| 00:08:32 | PithWire、TSN、Dr. Web、fxxz、DEV Community、whatsnew.fyi（`b-list2.tsv`） | `barking fetch` | 5 ok；**whatsnew.fyi 429 blocked**（未重试、未改用其他来源，非必需）。`b-drweb.html` 约 0.64 MB、`b-cws.html` 约 0.64 MB，均只存本地（不入库），均低于 1 MB |

被拒或失败：Reddit 搜索（登录重定向）、Chrome 应用商店（脚本被拒/空壳）、whatsnew.fyi（429）、HN 评论得分（Algolia 不含，HN 不公开）。**未查**：AMO 版本历史页、Firefox 以外商店（Edge、Opera）、uBlock 官方 Wiki、Mozilla bugzilla 2091796（未登录访问可能受限，且与本 XSS 的关系已由 commit 信息说明）、`lootdest.pages.dev` 演示页（为避免执行 PoC，不访问）。

## 截图

渲染来源：`barking shot render`（headless Chrome，700 CSS px 宽、DPR 2，匿名无登录；GitHub 页因系统深色偏好渲染为深色底）与 opencli（AMO）。裁切用 `barking shot cropbox` / `stack`（只裁切）。第一次用 opencli + zoom 2 截 iter.ca 时版式被撑宽（左右被裁），改用 headless 渲染重做；旧图见“待删除”。所有图均已逐张打开目视，文字清楚、范围正确。

| 文件 | 来源页 | 说明 |
|---|---|---|
| `13-iter-header.png` | iter.ca 文章 | 站点导航 + 标题 + 署名与日期 + 首段（范围、效果、“fixed now”）（1400×690） |
| `14-iter-ai-credit.png` | iter.ca 文章 | AI 分工段（GPT-5.6 Sol、Claude 写 PoC、自写正文）（1400×255） |
| `15-iter-uuid.png` | iter.ca 文章 | “Figuring out the element picker URL”：Firefox UUID、默认 filter、XHR 钩子（1400×725） |
| `16-iter-opt-click.png` | iter.ca 文章 | opt 换行注入、“main issue”段、“Getting the click”（含 “I didn’t bother actually implementing this”）与信任/重编译段（1400×1110） |
| `17-ghsa-versions.png` | GHSA 公告 | 标题、High 徽标、Affected / Patched（1400×580）；**深色底**；含相对时间 “published … yesterday” |
| `18-ghsa-timeline.png` | GHSA 公告 | “Update: Blog post…”、报告日期、四条 commit 及 “Published in 1.75.2 / 1.75.4 / 1.76.0 / 1.76.0”（1400×860）；深色底 |
| `19-ghsa-email-bypass.png` | GHSA 公告 | 维护者 “Should be fixed in 1.75.4 which is now live on AMO.” + 研究者 “You can still get the extension UUID…” 邮件（1400×850）；深色底。邮件正文下沿 “syvb” 后紧接维护者此前回复（未入图） |
| `20-ghsa-cve-severity.png` | GHSA 公告 | Impact、Severity High、“CVE ID No known CVE”、Weaknesses 标题（1400×630）；深色底。Weaknesses 内容（No CWEs）在图外，见 `b-ghsa.txt` |
| `21-releases-176x.png` | GitHub Release 1.76.0 与 1.76.4 | 版本头（含 “released this last week / yesterday” 相对时间、commit 号）+ release notes 的安全条目；两版各一段，中间黑条为拼接间隔（1400×1242）；深色底。1.76.0 只截到前三条（后 15 条非安全条目未入图） |
| `22-releases-175x.png` | GitHub Release 1.75.2 与 1.75.4 | 同上，“released this 2 weeks ago” 相对时间（1400×992）；深色底 |
| `23-amo-version.png` | Firefox AMO | “More information” 栏：Version 1.76.4、Last updated a day ago (Nov 16, 2026)（1400×893）；深色底；未带登录信息（页面显示 “Log in”）；加载时页面不含登录者信息 |
| `24-hn-title.png` | HN 条目页 | 标题与 “511 points by zaik 14 hours ago … 148 comments”（1400×215）；匿名视图，右上 “login” 为未登录标记，用户名为公开提交者账号 |
| `24-tsn-exaggeration.png` | TSN 稿 | 标题、署名时间 + 首段（含 “The first fix shipped in 1.75.4 …” 与 “The advisory’s brief advice is to update to 1.76.0 or later.”）；两段，黑条为间隔（1400×694）；深色底 |

相对时间对应的绝对时间：GHSA published_at 11-16 00:27:56；1.76.0 北京 11-09 01:00:40；1.76.4 北京 11-16 01:24:24；1.75.2 北京 11-03 00:40:39；1.75.4 北京 11-03 21:56:23。

截图与存档的身份自查：`grep -ril "develata\|gitclone" docs/2611/1115/sources/b-*` 无命中；截图中无登录者头像或用户名（GitHub 页为未登录匿名渲染；AMO 为 “Log in” 状态；HN 为 “login” 状态）。GHSA 公告的 “Credits” 区有 syvb 的公开头像与账号，但不在任何一张截图内。

## 待删除（由 Claude 一次 rm）

- `docs/2611/1115/images/13-iter-title.png`（首版，页面被撑宽、左右被裁）
- `docs/2611/1115/images/14-iter-ai-split.png`（同上）
- `docs/2611/1115/images/15-iter-uuid-leak.png`（同上）
- `docs/2611/1115/images/16-iter-click.png`（同上）
- `docs/2611/1115/images/21-releases-1760-1764.png`（版本头底行被切一半，已由 `21-releases-176x.png` 取代）
- `docs/2611/1115/images/22-releases-1752-1754.png`（同上，由 `22-releases-175x.png` 取代）
- `docs/2611/1115/images/b-tmp-iter-full.png`（opencli 试验图）
