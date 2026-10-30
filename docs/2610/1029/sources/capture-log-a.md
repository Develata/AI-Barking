# 1029 期 A 组（Cursor 扩展市场 Claude Code 扩展投毒）抓取日志

执行方：claude-sonnet-5-5。时间一律北京时间（UTC+8）；本机时钟为美东 EDT（北京 = 本机 + 12 小时）。开工约 2026-10-30 23:08，写日志时约 23:50。
工具：`barking fetch`（存档，响应头已由工具脱敏）、`barking shot`（headless 渲染/裁切/拼接）、opencli（SPA 与被 headless 拦截的页面）、curl（API 单次请求）、fxtwitter（X 帖 JSON）、WebSearch（只用于发现线索）。
安全：全程未下载、安装、解包、运行任何 VSIX、payload、修复脚本（`remediate-macos.sh`、`remediate-windows.ps1` 的链接未点击、未请求）；未访问 IOC 中的 IP 与端口；自查命令只截图、未执行。

## 一、抓取记录

| 北京时间 | URL | 工具 | 结果 | 存档文件 |
|---|---|---|---|---|
| 10-30 23:08 | https://cursor.com/blog/claude-code-extension-security-notice | barking fetch | ok（200；`Date` 响应头 15:08 GMT） | a-cursor-blog.html/.txt/.headers |
| 10-30 23:08 | https://opensourcemalware.com/blog/trojanized-claude-code-extension-plants-persistent-ssh-backdoor | barking fetch | ok（正文为服务端渲染，18 KB） | a-osm-blog.* |
| 10-30 23:09 | https://api.fxtwitter.com/status/2116074289053175808 | curl | ok（200） | a-cursor-x-fx.json |
| 10-30 23:10 | https://registry.npmjs.org/@anthropic-ai/claude-code | curl（完整 packument 1.5 MB，不入库） | ok；只抽取 `time`、`dist-tags` 子集入库 | a-npm-times.json |
| 10-30 23:10 | https://open-vsx.org/api/Anthropic/claude-code 与 `/versions` | curl | ok | a-openvsx-ext.json、a-openvsx-versions.json |
| 10-30 23:10 | https://api.github.com/repos/anthropics/claude-code/releases?per_page=8 | curl | ok | a-gh-releases.json |
| 10-30 23:10 | https://web.archive.org/cdx/search/cdx?url=cursor.com/blog/claude-code-extension-security-notice&output=json | curl | ok；4 个快照 | a-wayback-cdx.json |
| 10-30 23:10 | https://api.github.com/search/issues?q=repo:anthropics/claude-code+2.1.313 | curl | ok；6 条全是无关 issue（computer use 类） | a-gh-issues-2.1.313.json |
| 10-30 23:12 | BleepingComputer、The Hacker News、Help Net Security、OX Security、CellCog 各一篇 | barking fetch | 全部 ok | a-bleeping、a-thehackernews、a-helpnet、a-ox、a-cellcog |
| 10-30 23:12 | https://cursor.com/marketplace | barking fetch | ok，但页面是 Cursor 插件目录（Plugins），无 Claude Code 扩展条目，与 VS Code 扩展市场无关，**未采用** | a-cursor-marketplace.html（4.9 MB，不入库）/.txt |
| 10-30 23:12 | https://status.cursor.com/ | barking fetch | blocked（403），没有换来源重试 | 无 |
| 10-30 23:12 | https://status.claude.com/ | barking fetch | ok；页面无与本事件相关的事故条目（“No incidents reported today”） | a-anthropic-status.* |
| 10-30 23:12 | web.archive.org 的 `id_` 快照 ×4（20261030084321、103226、114430、124831） | barking fetch | ok；这四份与实时页文本 diff 为空（仅构建 nonce 不同） | a-wb1–a-wb4.* |
| 10-30 23:19 | https://platform.twitter.com/embed/Tweet.html?id=2116074289053175808 | headless Chrome（barking shot render） | 失败：空白页（本机 headless 到不了该域） | 无 |
| 10-30 23:20 | 同上 | opencli（foreground，zoom 2，`--width 1040`） | ok；用 opencli 才渲染成功。`--window background` 视口为 0 无法截图；zoom 2 配 `--width 1400` 超时 60 s，改 `--width 1040` 成功 | 图 08 |
| 10-30 23:24 | OSM 文章 | headless | 失败：SPA 只渲染出页脚 | 无 |
| 10-30 23:25 | OSM 文章 | opencli（foreground） | ok；全页截图出现重复拼接，改为按元素滚动后截视口 | 图 09 |
| 10-30 23:26 | cursor.com/en-US/blog/…（英文版） | barking shot render | ok。opencli 打开 `cursor.com/blog/…` 会被重定向到 `/cn/blog/…`（中文版），且 zoom 2 时代码块横向溢出使截图缩放异常，故正文截图退回 headless 渲染英文版 `/en-US/` | 图 01–07 |
| 10-30 23:28 | 搜索线索：HN Algolia、`barking heat` | barking heat / curl | HN 找到 3 个主帖；Reddit 搜索被重定向到登录页，**失败**（不登录、不换镜像） | a-heat.json、a-hn-heat.json、a-hn-hn-50300683.json、a-hn-hn-50301983.json、a-hn-hn-50301004.json |
| 10-30 23:33 | doublepulsar.com 文章、phorion.io 文章 | barking fetch | ok | a-doublepulsar.*、a-phorion.* |
| 10-30 23:34 | 同 doublepulsar 页面 | headless | 失败：Cloudflare 拦截（Sorry, you have been blocked） | 无 |
| 10-30 23:36 | 同 doublepulsar 页面 | opencli（foreground，zoom 2） | ok | 图 12b |
| 10-30 23:36 | https://api.github.com/search/issues?q=repo:anthropics/claude-code+cursor+malicious… 与 `+backdoor` | curl | ok；无相关 issue（命中全是无关 issue） | a-gh-issues-malicious.json、a-gh-issues-backdoor.json |
| 10-30 23:37 | https://www.stepsecurity.io/blog/claude-code-extension-trojanized | barking fetch | ok | a-stepsec.* |
| 10-30 23:38 | https://cursor.com/cn/blog/claude-code-extension-security-notice | barking fetch | ok（Cursor 官方中文版） | a-cursor-blog-cn.* |
| 10-30 23:40 | https://aihot.news | curl | 失败（curl 报 schannel SEC_E_INVALID_TOKEN；随后用 opencli（Chrome）再试一次，Chrome 报“此网站无法提供安全连接 / 发送的响应无效”（约 23:48）；不再重试，也未改用镜像；AIHOT 条目标题改由 a-agendum-94.json 里的转引核对） | 无 |
| 10-30 23:41 | https://api.github.com/repos/geekjourneyx/agendum/issues/94 | curl | ok；该 issue 引用了 AIHOT 的标题（含“最多影响 5000 用户”，15:46） | a-agendum-94.json |

## 二、截图记录（均在 `docs/2610/1029/images/`，2 倍像素比，宽度均 ≤ 1400 px）

| 文件 | 来源与方式 | 备注 |
|---|---|---|
| 01-cursor-notice-head.png | Cursor 英文版博文，headless 渲染 + cropbox | 浅色底；含日期、标题、目录、开头段 |
| 02-cursor-time-window.png | 同上 | “What happened” 标题与时间窗段 |
| 03-cursor-payload-steps.png | 同上 | 四条 bullet（含 “this step would fail”） |
| 04-cursor-fail-vs-still-works.png | 同上 | “Note: The attacker server no longer functions.” 至 “Additionally…” 段 |
| 05-cursor-what-we-did.png | 同上 | “What we did” 四条 |
| 06-cursor-what-you-should-do.png | 同上 | 五步处置 |
| 07-cursor-check-commands.png | 同上 | 自查命令段。**页面里代码块横向滚动，截图右侧命令被截断**（macOS 命令只见到 `|| ` 前；PowerShell 命令末尾被截）；完整原文见 a-cursor-blog.txt。不要把它当作“命令完整”的证据 |
| 08-cursor-x-card.png | X 官方嵌入页，opencli 截 + cropbox | Cursor 官方账号公开帖卡片；只截卡片，裁掉滚动条与浏览器翻译插件图标；卡片自身截断在 “It does not impact other extensions Show more”，完整正文见 a-cursor-x-fx.json；卡片时间 3:46 AM 为本机 EDT，即北京 10-30 15:46 |
| 09-osm-update-and-scope.png | OSM 文章，opencli 视口截图三段 stack | **深色底页面**；含日期、标题、副标题、作者署名（页面显示 “6mile (Paul McCarty)”，头像已用 CSS 隐藏）、UPDATE 段、“What is (not) malicious”；段间有 12 px 拼接缝，部分段落边界处有半截文字行（“loader inserted before the legitimate” 被截一半；“What is malicious:” 标题行未进图，只见其下第一条）；页面顶部固定导航与 Sign In 已裁掉；浏览器翻译插件浮标已用 CSS 隐藏 |
| 10-helpnet-says-open-vsx-v2.png | Help Net Security，headless 渲染 + stack | 标题与首段 + “The company spotted the malicious package, version 2.1.313, on Open VSX” 段；**v2 为正式版**，旧版 10-helpnet-says-open-vsx.png 顶部带编辑头像残片，待删 |
| 11-thn-correction-note-v2.png | The Hacker News，headless 渲染 + stack | 标题与日期、“distributed only through Cursor's marketplace…” 段、文末更正说明；**v2 为正式版**，旧版 11-thn-correction-note.png 最后一行被截半，待删 |
| 12-cellcog-title-open-vsx.png | CellCog 博客，headless 渲染 + stack | 标题 “Claude Code Extension Backdoored on Open VSX and Cursor” 与 “What happened?” 一行 |
| 12b-doublepulsar-anthropic-hacked-v2.png | Kevin Beaumont 的 DoublePulsar（Medium）文章，opencli（zoom 2，视口截图）+ cropbox | 标题、署名、前三段；页面上写 “5 hours ago”（相对时间），绝对发布时间见 evidence 的 A9；作者头像已用 CSS 隐藏；**v2 为正式版**，旧版 12b-doublepulsar-anthropic-hacked.png 标题顶部被裁，待删 |

对 opencli 页面做过的显示层改动（不改文字、不改数字）：隐藏浏览器翻译插件浮标、固定定位元素与小头像；OSM 页隐藏侧栏目录。

未截：npm / Open VSX 版本列表页（时间紧、未尝试；版本时间以 API JSON 存档为准，见 a-npm-times.json、a-openvsx-versions.json、a-gh-releases.json）；OSM 的 Attack Chain 表（页面里表格横向滚动被裁，三次重试不稳，放弃；表格文字见 a-osm-blog.txt）；超出 12 号区间不再追加。

## 三、其他说明

- 没有访问 X 的登录态页面，没有点击卡片内的 “Show more”（它是跳转链接）。
- 抓取时没有登录任何站点；浏览器里的 Medium 页面显示 “Follow” 按钮是未登录状态。
- 本组没有写入 `docs/2610/1029/` 以外的文件。
