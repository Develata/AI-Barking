# 1119 期 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区 UTC+8），取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json` 与命令输出。文件前缀 `b-`。未登录任何账户，未接受任何 Cookie 横幅，未提交表单，未发帖、点赞、评论、关注。`.headers` 由 `barking fetch` 去掉抓取地点与标识。浏览器：`barking shot` 的 headless Chrome（匿名，`BARKING_CHROME` 指向 Chrome 应用二进制）与 opencli 浏览器扩展（Browser Bridge；只读取公开页面；页面均显示 “Sign In” 未登录状态）。按派工单，B 组不截 Signal 应用（本组没有操作 Signal 客户端）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 22:12:01–22:12:03 | https://signal.org/blog/protecting-your-privacy/ ；https://signal.org/blog/rss.xml ；https://support.signal.org/hc/en-us/articles/360043469312-Screen-Security ；https://www.reuters.com/legal/litigation/signal-restricts-screenshots-block-ai-agents-reading-messages-2026-11-20/ ；HN 页 `50573902`、`50572348` ；GitHub API `signalapp/Signal-iOS` 与 `Signal-Android` releases（各 5 条） ；https://signal.org/android/apk/ | `barking fetch`（`b-fetch-list-1.tsv`） | blog、rss、两个 HN 页、两个 GitHub release JSON、android/apk ok（`b-blog.*`、`b-rss.xml`、`b-hn1/2.*`、`b-gh-ios.json`、`b-gh-and.json`、`b-sig-release.*`；apk 页只是下载页，无用）；**blocked**：支持页（HTTP 403，Cloudflare 质询）、路透（HTTP 401）。blog 的 txt 已打开，是正文 |
| 22:1x | 支持页 | opencli browser（`bsup` 会话，`eval document.body.innerText`） | 成功，`b-support-live.txt`（匿名） |
| 22:1x | 路透原页 | opencli browser（`brt` 会话，`eval document.documentElement.innerText`） | 成功渲染出全文（页面未被付费墙拦；页头显示 “Sign In / Subscribe”，未登录，未绕过任何付费墙）：`b-reuters-live.txt`。首次 `document.body.innerText` 因页面未载完返回空，等待后重试成功 |
| 22:13:18–22:13:20 | https://www.theverge.com/ai-artificial-intelligence/1038390/signal-blocking-ai-agent-screenshots-mobile-app ；https://www.apple.com/newsroom/2026/09/siri-ai-a-profoundly-more-capable-and-personal-assistant-is-here/ ；https://support.google.com/gemini/answer/15850607?hl=en ；https://www.eff.org/deeplinks/2026/10/victory-you-can-now-exclude-apps-siri-ai ；https://signal.org/blog/signal-doesnt-recall/ | `barking fetch`（`b-fetch-list-2.tsv`） | 全部 ok（`b-verge.*`、`b-apple-siri.*`、`b-google-gemini.*`、`b-eff.*`、`b-sig-recall.*`） |
| 22:13:43 | HN 条目 `50573902`、`50572348`；HN 搜索 “Signal screenshots AI”、“Signal Siri Gemini screen”；Reddit 搜索 “Signal blocking AI screenshots”、“Signal Siri AI Gemini screen” | `barking heat docs/2611/1119 b` | HN 成功（`b-heat.json`、`b-hn-50573902.json`、`b-hn-50572348.json`）；**Reddit 两次失败**：重定向到 old.reddit.com 登录页，响应非 JSON，不登录、不重试 |
| 22:1x | Reddit 搜索 “Signal AI screenshots Siri” | `opencli reddit search` | 失败：SECURITY_BLOCK（HTTP 403 “blocked by network security”），未重试 |
| 22:1x | HN Algolia 搜索 “signal screenshots”（近一周） | curl（`--max-time 40`） | 200，只命中 `50573902` |
| 22:1x | https://signal.org/blog/images/protecting-your-privacy-1.png 与 …-2.png | curl（`--max-time 60`） | 200，`b-blog-img1.png`、`b-blog-img2.png`（Signal 博文内嵌设置示意图，仅存本地） |
| 22:1x | Exa 检索（`web_search_exa`，共 5 次：英文主题、中文主题、苹果 Excluded Apps、Gemini Intelligence、夸大标题中英文） | Exa MCP | 仅作线索与二手稿清单；被引用的页面均另行 `barking fetch` 存档后核对原文 |
| 22:1x | 报道页 Android Authority、The Next Web、DataNorth、Cybersecurity News、Sina、Snape Report、CNA（路透转载）、Gate、路透 Muse 调查 | `barking fetch`（`b-fetch-list-3.tsv`） | androidauth、tnw、datanorth、cybersecnews、sina、snape、cna ok；**blocked**：Gate（HTTP 403）、路透 Muse 调查（HTTP 401，11/18 稿，背景未要求展开，未用 opencli 重取） |
| 22:1x | https://blog.google/products-and-platforms/platforms/android/gemini-intelligence/ ；https://blog.google/security/android-gemini-intelligence-security-privacy/ ；苹果支持页 “Turn off and restrict access to Apple Intelligence features on iPhone” ；MacRumors Excluded Apps 教程 | `barking fetch`（`b-fetch-list-4.tsv`） | 全部 ok（`b-google-gi.*`、`b-google-gi-sec.*`、`b-apple-restrict.*`、`b-macrumors-excl.*`） |
| 22:17:19 | https://support.apple.com/guide/iphone/change-siri-settings-iphc28624b81abc/27/ios/27 | `barking fetch`（`b-fetch-list-5.tsv`） | ok，`b-apple-excl.*`（含 “Exclude apps from Siri, Search, and suggestions”） |
| 22:1x | GitHub 搜索提交 `siri OR assistant OR "screen security"`（Signal-iOS、Signal-Android）；Signal-iOS 最新提交列表；提交详情 `9256c9583`、`119f06be0`（iOS），`da1483930`、`e5c6df484`、`66ef088c5`、`231b2be3b`（Android）；compare 各 tag 与提交 | curl（GitHub REST，匿名，`--max-time 40`） | 200，`b-gh-ios-commit-*.json`、`b-gh-and-commit-*.json`；compare 结果只在 evidence 里转述（未存档 JSON）。匿名 API 有速率限制，未触发 |

被拒或失败：支持页 curl（Cloudflare 质询，已改 opencli）、路透 curl（401，已改 opencli 读到全文）、Gate（403）、路透 Muse 调查（401，未用 opencli 重取）、Reddit（登录重定向与安全拦截）。未取：App Store 与 Google Play 的 Signal 更新说明页（动态页，未尝试；GitHub release 说明与提交已足以说明版本线索）。

## 截图

均匿名，已逐张打开确认文字清楚、范围正确（`22` 例外，见下）。

| 文件 | 来源页 | 工具与说明 |
|---|---|---|
| `13`–`14`、`16`–`20` | Signal 博客 | **本地副本**：把 `b-blog.html` 另存为 `b-blog-fn.html`，仅做两处修改：加 `<base href="https://signal.org/">` 与 `.footnotes{display:block !important}`，并去掉 littlefoot 脚注弹窗脚本（该脚本把页面底部脚注折叠成弹窗，默认不可见）。正文文字未改。用 `barking shot render file:///…/b-blog-fn.html blogfn --height 5500`（700 CSS px，DPR 2，输出 1400×11000）后 `cropbox` 裁切。裁切边界落在文字行之间。 |
| `15-signal-settings-ios-android.png` | Signal 博客 | 同一张渲染图；含博文内嵌的两张设置示意图（深色图），两图之间是页面白色间隔 |
| `21-signal-support-screen-security.png` | Signal 支持页 | opencli，`zoom 1.5` 后 `screenshot --width 1400`，裁掉浏览器翻译插件图标（右侧粉色圆标）与侧栏；约 1.5 倍像素（该页版式在 700 CSS px 下文字被截断，因而没有用 2 倍） |
| `22-reuters-default-on-popup.png` | 路透 | **上缘切在文字行中，不要使用，列入待删除**；由 `23` 替代 |
| `23-reuters-default-on-popup.png` | 路透 | opencli，`zoom 1.5`；为避开固定页头与广告块，截图前用 DOM 脚本把固定/粘性定位元素设为隐藏并滚动到该段（页面文字未改）；页面登录栏 “Sign In” 不在裁切范围内 |
| `24-verge-headline-blocking.png` | The Verge | `barking shot render` 700 CSS px，DPR 2，`cropbox`；深色底；Cookie/条款横幅在下方，未入框 |

所有 opencli 截图的会话（`bsup`、`brt`、`bblog`、`baa`）为匿名未登录；`baa`（Android Authority）截图因浏览器广告拦截占位符（“已移除广告”）出现在页面中，未采用，没有落到 `images/`（临时文件在草稿目录）。

## 自查

- 对 `docs/2611/1119/` 下 B 组文件 grep 抓取者个人信息（用户名、邮箱、头像链接）：无命中；Signal 博客页作者头像链接（avatars.githubusercontent.com/meredith-signal）出现在存档 HTML 里，属被报道方公开信息，非抓取者。
- 本组没有引用社交平台帖子，因此无需占位符替换。
- 修改过的存档副本只有 `b-blog-fn.html`（用于渲染脚注，见上）；原始存档 `b-blog.html` 未动。
