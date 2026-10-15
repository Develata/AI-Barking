# 1014 期 B 组抓取日志（科学家公开信与安全专家）

北京时间（UTC+8）；本机美东（EDT）+12 小时 = 北京；UTC + 8 = 北京。时刻取自命令的系统时钟与 `b-shots-log.jsonl`/`b-browser-records.jsonl`，为近似值。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理；全程约 35 分钟（17:10–17:45 北京）。

工具：curl（浏览器 UA，匿名）；Playwright Core + 本机 Chrome（匿名全新临时 profile，不登录，不带 Cookie，en-US）；fxtwitter API；HN Algolia 与 `barking heat`；`barking html2txt`/`shot`；WebSearch ×5；firecrawl_search ×4（仅搜索，未用 scrape）。未登录任何站点，未运行读取浏览器 Cookie 的工具，未使用镜像站（Wayback 快照只用于查看页面“当时写了什么”）。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 17:10 | https://ai-security-letter.org/ | curl | 200；GitHub Pages；Last-Modified 10-15 07:22:50 GMT；“463 Signatories” | `b-letter.html/.headers`；`b-letter.txt`（html2txt，17 个媒体联络人邮箱已替换为 `[email]`） |
| 17:11 | Wayback CDX 与 `web.archive.org/web/20261013182429id_/…`、`…20261014111946id_/…` | curl（先试 `archive.org/wayback/available` 得 429，改用 CDX） | 200；响应为 gzip，解压后存；406 与 415 | `b-letter-wayback-<ts>.html/.txt`（txt 邮箱已替换）。10-04 两份快照仅 811 字节占位页，未存 |
| 17:12 | Reuters 原页 `reuters.com/legal/litigation/scientists-call-international-body-oversee-advanced-ai-2026-10-13/` | curl | **401**，仅 JS 校验页（774 字节，无正文，失败）；html 已删，保留 `b-m-reuters.headers` | `b-m-reuters.headers` |
| 17:12 | BNN Bloomberg（加拿大通讯社稿）、Futurism、technology.org、Quartz、CSO Online | curl | 全 200 | `b-m-bnn`、`b-m-futurism`、`b-m-technologyorg`、`b-m-qz`、`b-m-cso`（.html/.headers/.txt） |
| 17:12 | fxtwitter：Bengio 帖 `2110061471937757320` | curl | 200 | `b-x-bengio-2110061471937757320.json`（未登录取数，无抓取者信息） |
| 17:14 | Anthropic `anthropic.com/news/investigating-incidents-cybersecurity-evals` | curl | 200 | `b-anthropic-evals.*` |
| 17:15 | The Decoder、CellCog、Superpower Daily | curl | 全 200 | `b-m-decoder`、`b-m-cellcog`、`b-m-superpower` |
| 17:16 | YouTube oEmbed 与 watch 页（Dwarkesh 播客元数据） | curl | 200，只取标题/发布时间 | 未存档（元数据写入 evidence） |
| 17:17 | Yahoo Canada（Reuters 转载全文）；Dwarkesh 文字稿 | curl | 200 | `b-m-yahoo-reuters.*`、`b-dwarkesh-brown.*` |
| 17:18 | Fortune（LeCun 采访） | curl | 200 | `b-m-fortune-lecun.*` |
| 17:20 | `barking heat docs/2610/1014 b --item 50081380 --item 50091788 --reddit …`（3 组 Reddit 搜索） | barking | HN 两条成功；**3 个 Reddit 搜索请求失败**（重定向到登录页，响应非 JSON） | `b-heat.json`、`b-hn-50081380.json`、`b-hn-50091788.json` |
| 17:22 | fxtwitter：Noam Brown 帖 `2100998240586137701` | curl | 200 | `b-x-brown-2100998240586137701.json` |
| 17:22 | bitbase、报价宝、TechFlow（英） | curl | 全 200 | `b-m-bitbase`、`b-m-baojiabao`、`b-m-techflow` |
| 17:23 | RFI 中文版 | curl；后用 Playwright（`b-fetch.mjs`） | curl **403**（失败）；Playwright 200，正文完整 | `b-m-rfi.*`（html 为 Playwright 渲染结果） |
| 17:24 | Reddit r/accelerate 帖 | curl（old.reddit，返回欢迎页，无正文，失败）；后用 Playwright（`b-fetch.mjs`） | Playwright 200，帖标题、评论与 `score` 属性可见；未登录 | `b-reddit-accelerate.html/.txt`（html 本地） |
| 17:25 | 搜索（firecrawl_search ×4、WebSearch ×5） | — | 返回二手摘要与链接；摘要中的说法一律回原页核对后才采信（如搜索摘要称“Reuters 报 LeCun 未签”→ 回到 Yahoo 转载的 Reuters 全文核对） | 无 |

## 2. 截图

| 约时刻 | 图 | 工具 | 说明 |
|---|---|---|---|
| 17:22–17:23 | `13-letter-opening.png`、`14-letter-recommendations.png`、`15-letter-signatory-count.png` | `barking shot render`（700 CSS px，DPR 2）+ `cropbox`；记录在 `b-shots-log.jsonl` | 15 显示“463 Signatories”（17:22 北京）；白底页，无登录信息 |
| 17:24–17:25 | `16-cso-expert-quotes.png`、`17-cso-headline.png` | `b-browser.mjs`（Playwright，DPR 2，隐藏 fixed/sticky） | 首次渲染 `barking shot render` 时页面带通知弹窗与灰色遮罩（试验图在临时目录，未入库）；改用脚本隐藏弹窗层。16 首版顶部带上一行标题残字，`OVERWRITE=1` 重截（自己生成的试验件，已覆盖）。`b-shots-cso.json` 为最后一次（16 号）的参数，17 号参数为：a=“AI agents aren’t ‘going rogue.’”，b=“Cyber experts say AI labs’”，pt/pb=24 |
| 17:25 | `18-anthropic-three-incidents.png`、`19-anthropic-harness-failure.png` | `b-browser.mjs` | 19 首版顶部带上一条目残字，`OVERWRITE=1` 重截。18 含页面头图（插画），保留完整上下文，尺寸 1400×2466 |

逐张已用 Read 打开目视确认文字清楚、范围正确。opencli：`opencli browser b-shot open …` 返回 “Browser Bridge extension not connected”（Chrome 未开或扩展未连接），未使用，如实记录。

## 3. 失败 / 受限

- Reuters 原页 401；以 Yahoo Canada 转载页代替（含 Reuters 署名与 JSON-LD 时间）。
- Reddit 搜索（barking heat）被登录墙拦；用匿名 Playwright 取单帖页代替，仅得 r/accelerate 一帖，未全面搜索 Reddit 其他社区的相关帖。
- RFI curl 403，Playwright 成功。
- opencli 未连接。
- Wayback available API 429，用 CDX 替代。

## 4. 自查

- 对 `docs/2610/1014/` 的 B 组文件 grep 本机用户名、邮箱、“develata”：无命中（Playwright 全新匿名 profile；fxtwitter 公开 JSON 仅含推文作者公开信息）。
- 入库 txt 中信站学者邮箱已替换。
- 单文件超过 1 MB：`b-reddit-accelerate.html`（1.3 MB，仅本地，不入库）；txt/json 均小于 1 MB。
- 试验文件：`b-m-reuters.html`（401 空页）已删除；其余试验图在系统临时目录，未入库。
