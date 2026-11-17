# 1116 期 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区 UTC+8），2026-11-17 19:24–20:00，取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json` 与终端时间。文件前缀 `b-`。未登录任何账户、未提交表单、未发帖；opencli 用真实 Chrome 但仅匿名读页面（Reddit 遇登录墙即停，没有登录）。`opencli doctor` 开工时通过。未用 firecrawl，未运行读取浏览器 Cookie 的工具。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 19:24:28 | Apple Support：en-us / zh-cn / zh-tw / zh-hk 的 127901；en-us 127893、121115、108047；zh-cn 108047（`b-fetch-list-1.tsv`） | `barking fetch` | 8 条 HTTP 200 / ok。`.txt` 已目视核对为正文 |
| 19:25:35 | 9to5Mac、MacRumors、The Verge、iDropNews、Macworld、IT之家、凤凰网、动察、NOWnews、鏡報；Apple Developer 发行说明页、Releases 列表与 `?id=11162026a`；Apple Newsroom 首页（`b-fetch-list-2.tsv`） | `barking fetch` | 14 ok；**AppleInsider、INSIDE 403 blocked**（Cloudflare 质询）。**假 ok**：`b-dev-relnotes-ios273.txt` 仅 5 行（页面 JS 渲染，状态 ok 不等于拿到正文），改用 JSON 接口 |
| 19:26 | `https://developer.apple.com/tutorials/data/documentation/ios-ipados-release-notes/ios-ipados-27_3-release-notes.json` | curl（`--max-time 60`，UA 设为 Mozilla/5.0） | 第一次无 UA 返回 000 无文件，第二次加 UA 返回 200，15 KB，`b-dev-relnotes-ios273.json`（Beta 2 全文，无用量额度条目） |
| 19:26:33 | Wayback 快照 `…/web/20260910160538id_/…127901`（快照时间戳 2026-09-10 16:05:38 UTC，即北京 09-11 00:05）与 `…/20261022045652id_/…127901`；Apple Newsroom 三篇（Apple Intelligence、WWDC26 总稿、Services）；MacRumors 09-09 与 Beta 2 汇总稿；Mac Observer 两篇（`b-fetch-list-3.tsv`） | `barking fetch` | 7 ok；**Mac Observer 两篇 403 blocked**。Wayback 仅用于“页面以前写了什么”，已在证据文件注明 |
| 19:26 | Wayback CDX `web.archive.org/cdx/search/cdx?url=support.apple.com/en-us/127901&output=json` | curl（`--max-time 40`） | 成功，`b-wayback-cdx-127901.json`（14 条快照，09-10 至 11-17） |
| 19:26:53 | HN 条目 50520857（Algolia） | `barking heat --item`（前缀 `b`） | 2 分、评论 0，`b-heat.json`、`b-hn-50520857.json` |
| 19:26:55 | HN 搜索 “Siri AI usage limits iCloud+”；Reddit 搜索 | `barking heat`（前缀 `b-hn`） | HN 命中 0 条；**Reddit 重定向登录页，失败**（`b-hn-heat.json` errors） |
| 19:27 | Algolia 检索 “Siri AI”“iCloud+”“Apple Intelligence usage”“iOS 27.3”（`created_at_i>1794700000`） | curl | 命中 `50520857`（02:16）与二次提交 `50526863`（11:02，1 分 0 评论），均无评论 |
| 19:27:27 | Apple Support 127893 zh-cn、121115 zh-cn、148587 en；苹果中国 10-31 新闻稿（`b-fetch-list-4.tsv`） | `barking fetch` | 4 ok |
| 19:30 | AppleInsider 11-16 文章 | opencli（`--session b-save`，background，匿名；脚本 `b-opencli-save.sh`） | 成功，`b-appleinsider.txt/.html`（文本 7 KB；页面无 `article:published_time`，时间取自正文 “Mon Nov 16 2026, 01:16 PM EST”） |
| 19:30–19:31 | INSIDE、Mac Observer 11-17、Mac Observer 09-12 | opencli（同上） | Mac Observer 两篇成功；INSIDE 第一次脚本报 `innerText` null（页面未载入），第二次成功。存 `b-inside.*`、`b-macobserver-nov17.*`、`b-macobserver-sep12.*` |
| 19:59 | Reddit `old.reddit.com/r/apple/search?q=siri+ai+usage+limits…` | opencli（匿名） | **登录墙**（“访问旧版 Reddit 需要账户”），未登录、未绕过，记失败 |
| 19:3x–19:5x | Exa `web_search_exa` 共 9 次（英、中、繁体关键词，含 Reddit 域名限定） | Exa | 仅作线索：发现 MacRumors、9to5Mac、AppleInsider、Verge、Macworld、iDropNews、The Threshold、Asia Business Daily、Apple 中国新闻稿与 6 月新闻稿；**不用 Exa 高亮作逐字引文**，引文均取自本地存档 |

## 截图

渲染：`barking shot render`（headless Chrome，匿名无登录，`BARKING_CHROME` 指向 `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`）。系统深色偏好使 Apple Support 页、Verge、Wayback 页渲染为**深色底**，IT之家亦深色，Apple 中国新闻稿为浅色。裁切与拼接只用 `barking shot cropbox | stack`（不改像素）。每次动作记入 `b-shots-log.jsonl`。

**像素比说明（偏离规定）**：规定 DPR 2、宽 ≤ 1400 px。Apple Support 倍数表在 700 CSS px 视口下**第 6 列（12 TB）被页面截掉**（试渲 `s127901`，1400×6800），所以 13–17、18、19、20、22 用 930 CSS px 视口 + DPR 1.5（输出 1395 px 宽，仍 ≤ 1400），换取完整表格。字体清晰度已目视确认。23、24 用 700 CSS px + DPR 2。

| 文件 | 来源页 | 说明 |
|---|---|---|
| `13-support-intro-categories.png` | 127901 en | 标题、首段、“feature categories”、四类、“Limits may vary”、Home 摄像头段（1195×1205，深色底；页面导航栏裁掉） |
| `14-support-siri-limit.png` | 127901 en | “About the Siri AI limit” 全段（1195×495，深色） |
| `15-support-device-reset.png` | 127901 en | “What happens when you hit…” 含 “Each device has its own…” 与 “When your daily usage limit resets” 全段（1195×775，深色） |
| `16-support-multiplier-table.png` | 127901 en | “How to increase…” 加量段、表题、**全 6 列、两行倍数表**（1195×1015，深色）。表的 12 TB 列在页面自身排版中略超出分隔线，非裁切造成 |
| `17-support-region-fineprint.png` | 127901 en | 页末细则（含两句 “not available in”）与 “Published Date: November 16, 2026”（1195×390，深色） |
| `18-wayback-0910-older-page.png` | Wayback 09-10 快照 | 旧版标题 “Learn about Apple Intelligence usage limits”、“In the future, Apple will implement…”、“Published Date: September 09, 2026”（1170×1430，深色）。Wayback 工具栏已裁掉；**属历史快照，须注明** |
| `19-zhcn-support-sept-text.png` | 127901 zh-cn | 9 月旧文，“要增加对此类功能的使用权限，需要付费”，“发布日期： 2026 年 09 月 09 日”（1170×1105，深色） |
| `20-siri-ai-availability-china.png` | 127893 en | 拼接 2 段（间隔灰条）：区域段（EU、China mainland）与页末脚注 + “Published Date: November 12, 2026”（1100×950，深色） |
| `21-cn-newsroom-no-siri-ai.png` | 苹果中国新闻稿（10-31） | 拼接 3 段：标题与日期、首段（含 “12 月 iOS 27.3 正式发布”）、“本次更新不包含 Siri AI …”（1100×953，浅色） |
| `22-icloud-price-us-china.png` | 108047 en | 拼接 6 段：计费说明句、NA 表头、美国行、APAC 表头、中国大陆行、脚注 1–4（1195×707，深色）。**两张表各取一行，非连续截图**（OCR 定位行；脚注 5 已裁掉以免切字） |
| `23-verge-headline-avoid-limits.png` | The Verge | 标题、题注 “…to avoid limits.”、作者与 “Nov 17, 2026, 3:24 AM GMT+8”（1400×550，深色；作者名为文章署名，非抓取者） |
| `24-ithome-50gb-free-list.png` | IT之家 | 拼接 2 段：标题末行与发布时间、含 “50GB（免费）” 的档位列表（1400×654，深色） |

目视确认：13、14、15、16、17、18、20、21、22、23、24 已逐张打开确认文字清楚、范围正确；19 由已目视确认的整页渲染裁出，裁切框覆盖标题至 “发布日期”，未单独再打开。

未截：6-08 新闻稿用量脚注（页面超长，OCR 未能定位，文字存档在 `b-newsroom-wwdc26-intelligence.txt`）；鏡報（弹窗广告遮挡）；zh-tw/zh-hk；中国大陆 108047 页。

## 自查

- 登录态/身份：所有页面匿名；Apple 页、Newsroom、Verge、IT之家截图无登录栏；Wayback 工具栏已裁；`grep -ri develata` 对 `sources/b-*` 与 `images/` 无命中。`.headers` 文件只保留 5 个响应头，其余由 barking 声明未存档。
- 大文件：`images/b-tmp-jun-low.png` 约 1.3 MB（临时图，非入库对象，列入待删除）。文本存档中单个 > 1 MB 的无；`.html` 原件按规定只存本地。

## 待删除文件（精确路径，均为我自己的试验文件，未执行删除）

- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/images/b-tmp-jun-a.png`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/images/b-tmp-jun-b.png`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/images/b-tmp-jun-low.png`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/images/b-tmp-low.png`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/images/b-tmp-price-first.png`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/images/b-tmp-price-second.png`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/sources/.b-opencli-appleinsider.json`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/sources/.b-opencli-inside.json`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/sources/.b-opencli-macobserver-nov17.json`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1116/sources/.b-opencli-macobserver-sep12.json`

此外，`/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/f616401c-cac1-467f-aa3a-174704ced72a/scratchpad/low.png` 为 sips 试裁产生的一份仓库外临时文件，非仓库内，可忽略。
