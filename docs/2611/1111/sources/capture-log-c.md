# C 组 capture-log（北京时间，UTC+8，本机时间）

执行方 claude-sonnet-5-5；时段 2026-11-12 17:53–18:08。工具：`barking fetch/html2txt/heat/shot render|lines|cropbox|stack`（封装脚本 `c-bk.sh` 只负责在 macOS 上设 BARKING_CHROME 后调用已编译的 barking）、opencli 浏览器（仅一次，读 Cybernews 页内文字并截了 4 张试验图，见下）、curl（HN Algolia API）、WebSearch（只作线索，未存档）。

隐私自限：没有打开、访问任何 grok.me 应用或报告点名的泄露页面（HN Algolia 搜索结果里出现的 `*.grok.me` 条目是 HN 用户自己发布的普通作品，同样没有打开）；没有下载、转存泄露数据；Claroc 报告里的图片文件（`/img/ghostshares2-*.png`）没有单独下载，截图来自对报告页的整页渲染。Ars 文中出现的真实 grok.me 主机名（X 广告方案）已在 `c-ars.txt` 与 `c-ars.html` 中替换为占位符 `[grok.me-app-hostname-redacted-by-collector]`（用 `sed -i` 就地改了这两个文件，是对存档的唯一改动，已在此登记）。

| 时间 | URL（已去查询参数） | 工具 | 结果 |
|---|---|---|---|
| 17:53:15 | blog.claroc.io 报告、the-decoder.com 文、claroc.io 首页、blog.claroc.io 博客首页（`c-list1.tsv`） | barking fetch | 4 个 ok：`c-report`（71 KB，322 行文本）、`c-thedecoder`、`c-claroc-home`（空壳 SPA，正文只有标题与 meta 描述，页面文字靠截图）、`c-claroc-blog` |
| 17:54:17 | HN 条目 50446608、50456277、50450625；HN 搜索 “Claroc Grok”“SpaceXSI Grok apps”；Reddit 搜索 “Claroc Grok apps”“GhostShares Grok” | `barking heat --item … --hn … --reddit …` | HN 三条 ok（5/0、11/0、6/0）；Reddit 两次搜索被重定向到登录页，**失败**，记入 `c-heat.json` errors，不重试 |
| 17:54:31 | arstechnica.com 文、blog.claroc.io 的 Muse、Railway、LinkDrift 三篇、rss.xml、grok.com/.well-known/security.txt（`c-list2.tsv`） | barking fetch | 6 个 ok。Ars 页内文字完整；security.txt 是 `Contact: mailto:vulnerabilities@x.ai`，只为核对报告所说的“security.txt 地址”，不是 grok.me 应用 |
| 17:5x | HN Algolia 搜索（claroc / grok build apps / grok.me / 文章标题）与条目 50456277 | curl `hn.algolia.com/api/v1`（`--max-time` 30–60） | ok，仅用于找帖子 ID 与分数 |
| 17:5x | 三次 WebSearch（中英文，关键词含 Claroc、35,578、37,571、94.7%） | WebSearch | 得到媒体清单线索（THN、Odaily、ChainCatcher、Cybernews、Unite、IBTimes、Crypto Briefing、FourWeekMBA、DEV Community 等），只作线索 |
| 17:55:41 | thehackernews.com、odaily.news、cybernews.com、unite.ai、ibtimes.co.uk、cryptobriefing.com、chaincatcher.com、fourweekmba.com（`c-list3.tsv`） | barking fetch | 7 个 ok；**cybernews.com 403 blocked**（Cloudflare 质询），保留质询页 `c-cybernews.html/.headers`，不绕过 |
| 17:56 | cybernews.com/security/groks-data-leak-spacexsi/ | opencli 浏览器（后台窗口，会话 `c-shot`，在维护者本机 Chrome 的浏览器配置里打开，未登录该站、未点通知弹窗） | ok：用页内 `innerText` 存成 `c-cybernews.txt`（首行含 URL）。注：这是页面文字，不是原始 HTML。页面里有通知授权弹窗，没有点击 |
| 17:57 | blog.claroc.io 报告页整页渲染（700 CSS px，DPR 2，高 12000/15000 CSS px） | `barking shot render`（第一次高度不足被截断，改 15000 重渲得完整 1400×30000 px，名 `report2`） | ok；中间图在系统临时目录 `…/T/barking-shot/26111111-c/`（不在仓库） |
| 17:58–18:03 | 截图 25–32、36（报告页裁切）；33（claroc.io 首页）；34（Muse 报告页）；35（THN） | `barking shot cropbox / stack` | 12 张，见下 |
| 17:59 | arstechnica.com 文 headless 渲染（`ars`、`ars2`） | `barking shot render` | 页面被广告/同意层遮挡，顶部整屏空白，**未截** |
| 18:01 | Cybernews、Odaily、THN、Ars 的 opencli 试验截图 | opencli browser screenshot（后台窗口，`--width 1400`，zoom 2） | 图落在会话 scratchpad（不在仓库）：显示浏览器翻译插件图标、去广告插件提示、通知授权弹窗，按“把浏览器插件图标裁掉”要求弃用，改用 headless 或不截。已 `close` 会话 |
| 18:02 | Odaily headless 渲染 | `barking shot render` | 桌面布局在 700 px 宽下右侧被裁，**未截**（文字存档足够） |
| 18:05 | HN 三帖分数复核 | curl Algolia | 5 / 0、11 / 0、6 / 0，与 17:54 一致 |

## 截图清单（`docs/2611/1111/images/`，均已逐张打开目视确认文字清楚、范围正确）

| 文件 | 内容 | 渲染参数 | 备注 |
|---|---|---|---|
| 25-claroc-report-top.png | 报告标题、日期、署名、题图、In short 要点（含 43,551/37,571/35,578、24 个应用、30 家组织、SpaceXSI 回应） | 700 CSS px，DPR 2（1400×3420） | 深色页；题图是 Grok Build 模式界面截图 |
| 26-claroc-35578-open.png | “35,578 open apps, and what was in them” 节：分母、HTTP 401 定义、1/10、2%、已确认示例（止于图 5 之前，不含时间表脚本图） | 同上（1400×1500） | 深色页 |
| 27-claroc-30-orgs.png | 30 家组织构成（26 家 >1,000 员工、6 家 Fortune 500、18+12、行业分布） | 同上（1400×610） | 深色页 |
| 28-claroc-two-test-apps.png | “We built two apps…”：两个测试应用、默认预选、权限倒置叙述与图 6（示意） | 同上（1400×2020） | 深色页；图 6 自称示意 |
| 29-claroc-spacexsi-says.png | “What SpaceXSI says”：厂商回应转述与 Claroc 反驳 | 同上（1400×1360） | 深色页 |
| 30-claroc-method-limits.png | “How we did this”：43,601 名称、AI 筛选与人工复核、650–800、不登录不提交表单、写删只在自建应用、副本将删除 | 同上（1400×930） | 深色页 |
| 31-claroc-timeline-why-published.png | 通报时间线（11-08、11-09、11-10）与“为何现在发布” | 同上（1400×1620） | 深色页 |
| 32-claroc-where-we-fit.png | “Where Claroc fits”服务推广与 “Book a 30-minute exposure check” | 同上（1400×1720） | 深色页；含 sales@claroc.io（公司销售邮箱，非个人） |
| 33-claroc-home-stealth.png | claroc.io 首页：“Built by security practitioners. In stealth, working closely with a small group of security teams.” | 700 CSS px，DPR 2（1400×580，从 1400×2400 整页裁出，裁掉渐变题图与底部 Cookie 横幅） | 深色页；未点击 Accept |
| 34-claroc-muse-first-report.png | 第一份 GhostShares（Muse）标题、日期 Oct 18、摘要、“Updated Oct 20, 2026” | 同上（1400×1525） | 深色页 |
| 35-thn-headline-lede.png | The Hacker News 标题、作者、日期与导语（拼接标题行与正文前段，中间的题图与广告区已省，间隔 24 px，底色自动） | 同上（1400×1594） | 浅色页；含 “This chat builder shares the Grok Build name with a separate terminal coding tool” |
| 36-claroc-territory-map-fig2.png | 图 2：SpaceXSI 销售区域图（“North Central Territory Mapper”，594 accounts），账户名与城市被黑条覆盖 | 同上（1400×1490） | 深色页底；已目视确认无未打码个人信息；含图说 “Account names and cities are covered.” |

未截图并说明：图 1（入职计划，含员工名）、图 3（X 提案，含金额）、图 5（时间表脚本，含密码字段，Claroc 称已逐值打码）不截，避免任何疑虑；Ars 页（见上）；Cybernews（浏览器插件污染，且页面副题末尾的 “b” 已在 evidence 里记录）；HN 帖截图（三帖 0 评论、分数低，未截）。`c-shots-log.jsonl` 有每次截图的机器记录。

## 自查（交付前）

- 对 `docs/2611/1111/` 的 `c-*` 文件与 25–36 号图 grep：无本机用户名、账号、Cookie 痕迹（对本机用户名与绝对路径前缀的 grep 在 `sources/c-*` 无命中；`c-shots-log.jsonl` 与 `c-fetch-log.jsonl` 记仓库相对路径）。唯一的真实 grok.me 主机名已替换；剩下的 `quiet-moss-lantern-bay.grok.me` 来自 Claroc 图 4 的示意（图注 “Illustrative. Every hostname is made up.”）。
- `.headers` 由 barking 自动裁剪，不含抓取地点或标识。
- 注意：`c-fetch-log.jsonl` 里 `ars` 一行的 sha256 与字节数是替换主机名之前的值，与现在的 `c-ars.html`（及 `c-ars.txt`）不再一致（已就地替换，见上）；后续上传网盘记 offsite.tsv 时请按现文件重算，不要当作损坏。
- 待删除文件（仓库内）：无。仓库外的试验文件（不入库，可留可清）：会话 scratchpad 下的 `m-cybernews.png`、`m-odaily.png`、`m-thn.png`、`m-ars.png`、`c-fig2.png`、`ars2-top.png`、`v-c-odaily.png`、`v-c-thn.png`、`v-c-claroc-index.png`、`v-c-claroc-home.png`；系统临时目录 `barking-shot/26111111-c/` 下的 `report.png`、`report2.png`、`ars*.png`、`odaily.png`、`thn.png`、`muse.png`、`claroc-*.png` 及各 `*-chrome.log`；`$TMPDIR` 下的 `report2-lines.txt`、`ars-lines.txt`、`ars2-lines.txt`、`muse-lines.txt`、`thn-lines.txt`。仓库内 `sources/c-securitytxt.*`、`c-rss.*`、`c-linkdrift.*`、`c-railway.*` 是附带存档，可留可不留（文本小）。
