# 1012 期 C 组抓取日志

时间为北京时间（UTC+8）；本机美东 = 北京 − 12 小时。时刻为命令发出时的近似值（取自 `date -u` 读数与脚本记录 `c-browser-records.jsonl`）。

工具：`gh api`（GitHub REST，匿名可读的公开数据；本机 gh 已登录，响应中不含查询者信息）、curl（普通浏览器 UA，经本机代理）、HN Algolia（`c-heat.py` 及 curl）、Bluesky 公开 API（`public.api.bsky.app`）、fxtwitter 公开 JSON、Playwright Core + 本机 Chrome（匿名、全新临时 profile、DPR 2、不登录，`c-browser.mjs` 改写自 1010 `a-browser.mjs`；`c-shot-probe.mjs` 仅探测不截图）、WebSearch（共 6 次，仅取线索）、`git clone --bare --filter=blob:none`（到会话临时目录，统计提交，不入库）。未登录任何站点，未输入凭据，未运行读取浏览器 Cookie 的工具，未发帖/点赞/评论/关注，未使用镜像站或代理域名。web.archive.org 未使用。

脚本（本目录）：`c-html2txt.py`（复制自 1010 `a-html2txt.py`，未改）、`c-fetch.sh`（curl + 转文本）、`c-hn-flatten.py`（HN Algolia JSON 展平）、`c-heat.py`（热度快照）、`c-browser.mjs`、`c-shot-probe.mjs`。`c-browser.mjs` 相对 1010 版的改动：前缀 `c-`、图片编号 25–36、截图超时 180 秒、可选环境变量 `SCROLL`（先滚动以载入懒加载图片）、`BLOCKADS`（拦截广告与 Cookie 横幅脚本的请求，仅 TechRadar 用）。

## 1. 抓取

| 约时刻 | URL | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 13:10 | https://api.github.com/orgs/storytold/repos（57 个仓库） | gh api | 200 | `c-gh-storytold-repos.json` |
| 13:11 | repos/storytold/{photocraft,lightcraft,vectorcraft,filmcraft,pdfcraft,effectcraft,designcraft,gridcraft,cadcraft,deckcraft,soundcraft,wordcraft,craft-launcher,craft-libs,artcraft,artcraft-services}（元数据与根目录文件列表） | gh api | 200 | `c-gh-repos-detail.jsonl` |
| 13:12 | 上列仓库 README（raw） | gh api（`Accept: application/vnd.github.raw`） | 200 | `c-readme-<repo>.md` ×16 |
| 13:13 | https://hn.algolia.com/api/v1/items/{49958850,49993722,49981449,50010170}；搜索 artcraft / photocraft / getartcraft.com / github.com/storytold | curl | 200 | `c-hn-<id>.json/.txt`、`c-hn-search-*.json` |
| 13:14 | HN 评论 49934844、49928088、49879369；echelon 评论搜索 | curl | 200 | `c-hn-comment-*.json`、`c-hn-echelon-comments.txt` |
| 13:15 | https://getartcraft.com/ 、/apps、/apps/photocraft、/about（404 页）、/pricing | curl | 200（/about 为 404 页面但状态 200）；Next.js 站，存文本 | `c-home-*.html/.txt/.headers` |
| 13:16 | https://www.techradar.com/pro/one-dev-is-recreating-7-adobe-apps-in-rust-for-free-so-i-tested-it-out | curl | 200，1.9 MB；last-modified 10/10 13:01:22 GMT | `c-techradar.html/.txt/.headers` |
| 13:17–13:20 | photocraft 发布、tags、贡献者、README 提交史、HEIC/保存相关 issue 与 PR（#355、#372、#2234、#2611） | gh api | 200 | `c-gh-photocraft-*.json/.tsv` |
| 13:20 | `git clone --bare --filter=blob:none https://github.com/storytold/photocraft.git`（会话临时目录） | git | 成功；统计提交与 Claude 尾注 | 统计结果 `c-gh-commit-stats.txt`（不存仓库副本；邮箱字段不记录） |
| 13:21 | https://public.api.bsky.app/xrpc/app.bsky.feed.getPostThread（O'Malley 帖） | curl | 第一次命令因并行未生成文件，重取 200（0.96 MB） | `c-bsky-thread.json` |
| 13:22 | 同站 getAuthorFeed（jamesomalley.co.uk，posts_no_replies 与 posts_with_replies） | python urllib | 200 | `c-bsky-authorfeed.json` |
| 13:22 | https://aihot.news/ 、/all、/hot、/daily | curl | / 与 /all 返回 JS 挑战（988 B）；/hot 与 /daily 200 | `c-aihot-*.html/.txt` |
| 13:24 | https://github.com/storytold/photocraft | Playwright 探测与截图 | 成功 | 图 25 |
| 13:25–13:32 | TechRadar 渲染截图 | Playwright（BLOCKADS=1，SCROLL=1） | 首次整页截图超时（等待字体）；加广告脚本拦截与超时 180 秒后成功；首次截图因图片未加载留空白，加滚动后重截 | 图 27、28；`c-techradar-render.txt` |
| 13:33–13:34 | GitHub photocraft README 状态框 | Playwright | 首次按 y 区间截错位置（README 动态变化），改用文本锚点重截（OVERWRITE） | 图 29 |
| 13:35 | https://news.ycombinator.com/item?id=49934844 、49959605 | Playwright | 成功；评论 49934844 首次高度不足以含全文，重截一次（OVERWRITE，旧图被覆盖） | 图 26、30 |
| 13:36 | https://embed.bsky.app/embed/did:plc:l2mdjnwjrwpns76ol45t66sf/app.bsky.feed.post/3mxbl36nkms2q | Playwright | 成功；首次高度不足含计数，重截一次 | 图 31；`c-bsky-embed.txt` |
| 13:37 | https://github.com/storytold/vectorcraft/commit/87cf1ff3e626369d5ceec2103ca5213e97e315c7 | Playwright | 成功；首次高度含 diff，重截缩短 | 图 32；`c-gh-vectorcraft-commit.txt` |
| 13:37–13:39 | https://github.com/storytold/photocraft/pull/372 、/releases/tag/v0.5.0 | Playwright | PR 页成功；v0.5.0 发布页的两次试截图（HEIC 行、页首）不含时间信息，未采用，**已删除** | 图 33；`c-gh-photocraft-pr372.txt`、`c-gh-photocraft-v050.txt`（发布页文本，截取时留的） |
| 13:40 | vectorcraft CLAUDE.md 提交史及 87cf1ff 前后各版 | gh api | 200 | `c-gh-vectorcraft-claudemd-commits.tsv`、`c-vectorcraft-CLAUDE*.md`、`c-gh-vectorcraft-commit-87cf1ff.json` |
| 13:41 | photocraft AGENTS.md、ROADMAP.md、docs/contributing.md、docs/target-app-parity.md；lightcraft CLAUDE.md | gh api（raw） | 200 | `c-photocraft-*.md`、`c-lightcraft-CLAUDE.md` |
| 13:42 | artcraft README 提交史与许可证提交（8abde75）；artcraft-services LICENSE.md（只读首 25 行） | gh api | 200 | 写入 evidence-c.md；README 在 `c-readme-artcraft*.md` |
| 13:44–13:50 | WebSearch ×6（Adobe 回应、媒体、中文报道、Opus 5.5） | WebSearch | 得到媒体链接与线索 | 线索写进 evidence-c.md |
| 13:46–13:49 | Futurism、Vice、PetaPixel、TechSpot、Phoronix、OMG Ubuntu、AppleInsider、Tedium、orcarouter、HotHardware、techgogogo、80aj、kju.ai、Ars Technica、Gizmodo | curl（`c-fetch.sh`） | 全部 200（Gizmodo 第一次 URL 猜错得 404，已删，从 Vice 文内取正确链接重取成功）；Albawaba 403 `Malformed Browser Request`（已删，未存） | `c-media-<name>.html/.txt/.headers` |
| 13:50 | https://www.reddit.com/r/SaaS/comments/1wwnhbt/…/.json、old.reddit.com | curl | **403**，不硬绕 | 无 |
| 13:50 | https://aihot.news/ 、/all | Playwright（匿名） | 返回 `{"error":"blocked","code":567}`，bot 拦截，不绕过 | 已删除（只有一行错误） |
| 13:50 | https://aihot.news/api/public/items?q=Adobe 、?q=ArtCraft（AIHOT 公开接口） | curl | 200 | `c-aihot-api-adobe.json`、`c-aihot-api-artcraft.json` |
| 13:51 | https://api.fxtwitter.com/status/2109501787736813663 （卡兹克） | curl | 200 | `c-fx-khazix.json`、`c-x-khazix-article.txt` |
| 13:51 | https://mp.weixin.qq.com/s?__biz=…（卡兹克公众号） | curl | 200，但只有空壳，未存档（已删） | 无 |
| 13:54 | `python c-heat.py`（HN 12 条、GitHub 18 个仓库、Bluesky 计数） | 脚本 | 全部成功 | `c-heat.json` |
| 13:56 | photocraft v0.5.0 标签的 .github/workflows、packaging/{linux,windows,macos}、apps/photocraft/Cargo.toml（grep heif） | gh api | 200 | 结果写入 evidence-c.md C5，无存档 |
| 13:57 | https://news.adobe.com/ | curl | 200 | `c-adobe-news.html/.txt` |
| 13:55 | FilmCraft releases/issues/PR（#259、#399、#129、#236、#76）；PhotoCraft/FilmCraft v0.5.0 对照 | gh api | 200 | `c-gh-filmcraft-releases.json` |

失败或未取得：Reddit（403）；AIHOT 网页（bot 拦截）；卡兹克公众号正文；Albawaba；微博转载页、Notebookcheck、kocpc 未抓取；开发者 X 账号与 getartcraft.com/news 未读；HN/Reddit 评论得分；Adobe 新闻室仅读首页（未站内搜索）。

## 2. 截图

2 倍像素比，宽 1400 px（CSS 700 px，嵌入帖 Bluesky 为 600 px 宽，输出 1200 px），按 CSS 像素区间或文本锚点裁切，只裁切不改像素。均用 Read 工具逐张目视确认：

| 文件 | 来源与区间 | 目视确认 |
|---|---|---|
| 25-photocraft-readme-top.png | github.com/storytold/photocraft y 0–1700（1400×3400）；浅色页，窄屏布局 | 清楚；含 48.8k stars、README 标题、clean-room 句与 early alpha 徽章 |
| 26-hn-echelon-opus55.png | HN item 49934844 y 0–562（1400×1124） | 清楚，评论全文，无输入框 |
| 27-techradar-heic.png | TechRadar y 4835–5861（经滚动载入图片） | 清楚；含小节标题、配图、HEIC 段至 “what Photoshop is most often used for.”；底边无下一图残边（第二次重截去掉 14 px） |
| 28-techradar-filmcraft.png | TechRadar y 6710–8252 | 清楚；含小节标题、两张配图、至 “should be baked in from the start.”，止于 FINAL THOUGHTS 标题之前 |
| 29-photocraft-readme-status.png | GitHub README 状态框（文本锚点，上 50 px、下 30 px） | 清楚；Important 框全文 |
| 30-hn-echelon-alpha-parity.png | HN item 49959605 y 0–284（1400×568） | 清楚；评论全文，无输入框 |
| 31-bsky-omalley-decompiled.png | Bluesky 官方嵌入（600 CSS px）y 0–790（1200×1580） | 清楚；含计数与时间 |
| 32-vectorcraft-commit-cleanroom-rule.png | GitHub commit 87cf1ff y 0–585 | 清楚；提交信息全文、作者 “bflatastic and claude” |
| 33-photocraft-pr372-heic.png | GitHub PR #372 y 0–966 | 清楚；标题、Merged 状态、说明至 “Saving still works through every other format.” |

深色底：仅 TechRadar 文中应用截图与 HN 无；页面均为浅色。技术说明：页头为 GitHub / HN 站点页头，显示 “Sign in” / “login”，无任何登录态。图 27/28 内的应用截图属 TechRadar 页面本身所载（“Image credit: ArtCraft // Future”）。图 25 的 “Report repository” 等为 GitHub 默认页面元素。

未做的候选：26（开发者自述）已用 HN 评论；`images/README.md` 等不写。编号 34–36 空缺。

## 3. 清理记录

- 本人生成并删除的试验文件：`images/25-probe.png`（改名为 `25-photocraft-readme-top.png`）；`images/34-photocraft-v050-release-top.png`（试截后未采用，已删除）；v0.5.0 发布页 HEIC 行试截图 `33-photocraft-v050-release-heic.png`（被 PR #372 页截图覆盖前移至会话临时目录，已不在仓库）；`c-err.txt`、`c-gizmodo*`（404）、`c-media-albawaba*`、`c-media-wechat-khazix*`、`c-aihot-pw-*.txt` 已删除；`c-browser-records.jsonl` 里对应 aihot-pw 两行已删（脚本记录了被拦截的两次试验）。
- 被覆盖：`26-…png`、`27-…png`、`29-…png`、`30-…png`、`31-…png`、`32-…png`、`33-…png` 各被后一轮截图覆盖一次（OVERWRITE，旧图为本人生成）。
- 临时探测与 `s.json`、旧版截图在会话 scratchpad（会话临时目录），不在仓库。
- 未改动仓库其他文件；未 commit、未 push；未改动其他组文件。
- `git clone --bare` 的副本在会话临时目录，不在仓库。

## 4. 隐私与泄露自查

- 全程匿名 headless Chrome（全新临时 profile），未登录；截图页头显示 Sign in / login，无账号信息。
- `docs/2610/1012/sources/c-*` 对 “develata”“echelon@gmail”“@gmail.com” grep：无命中。提交者个人邮箱（git 提交元数据中公开可见）未写入任何存档；`c-gh-commit-stats.txt` 只含聚合统计。
- GitHub 响应 JSON 含被调查对象（storytold 组织与成员）的公开头像链接，非抓取者。`c-bsky-thread.json`、`c-bsky-authorfeed.json` 含 Bluesky 上公开发帖者的头像链接与显示名（被取证对象）；无抓取者信息。
- 私人个体：只记录开发者公开职业身份（GitHub 公开 bio、媒体署名）与公开发言；未收集住址等。HN 用户名仅作发言署名。
- 超过 1 MB 的文件：`c-techradar.html`（1.9 MB，HTML 原件，只存本地，不入库）。`c-bsky-thread.json` 约 0.96 MB（接近上限，可选择不入库，保留 `c-bsky-embed.txt` 与证据表引用）。HTML 原件（`c-*.html`）、图片均只存本地。
