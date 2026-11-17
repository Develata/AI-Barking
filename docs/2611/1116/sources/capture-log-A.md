# 1116 期 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区），取自 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json` 与终端时间。文件前缀 `a-`。未登录任何账户，未提交表单，未发帖、点赞、评论、关注。浏览器为 opencli 驱动的真实 Chrome（会话 `a-shot`，后台窗口）；本组成品截图基本用 `barking shot render`（headless Chrome，700 CSS px、DPR 2，匿名，`BARKING_CHROME` 指向 Chrome 应用二进制）。X 帖子卡片用 X 官方嵌入页经 opencli 渲染（匿名，浅色主题）。存档的 `.headers` 已由 `barking fetch` 去掉抓取地点与标识。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 19:24:32 | https://xusheng.dev/posts/windows-coding-agent-telemetry/ | `barking fetch`（`a-fetch-list-1.tsv`） | ok。`a-blog.html`、`a-blog.txt`（195 行）、`.headers`（Last-Modified: Mon, 16 Nov 2026 22:06:36 GMT） |
| 19:24:33 | https://xusheng.dev/posts/index.xml 、 https://xusheng.dev/index.xml 、 https://xusheng.dev/ | 同上 | ok。RSS 的 pubDate 为 `Mon, 16 Nov 2026 00:00:00 -0500`（日期占位）；`a-about.*` 为作者关于页；`a-rss2.*` 是重复的 RSS（待删） |
| 19:24:33 | https://support.microsoft.com/en-us/windows/privacy/diagnostics-feedback-and-privacy-in-windows | 同上 | ok，txt 630 行，是正文（含“sample”条款） |
| 19:24:32 | https://api.fxtwitter.com/status/2122147747390468384 | 同上 | ok，JSON（公开推文；无抓取者信息，不需替换） |
| 19:24:33 | https://hn.algolia.com/api/v1/items/50524736 ；https://news.ycombinator.com/item?id=50524736 | 同上 | ok（`a-hn-algolia.json`，含嵌套评论 476 条，**不含评论得分**；`a-hn-page.*`） |
| 19:25:21 | https://learn.microsoft.com/en-us/windows/privacy/optional-diagnostic-data ；…/configure-windows-diagnostic-data-in-your-organization | `barking fetch`（`a-fetch-list-2.tsv`） | ok |
| 19:25:21 | https://learn.microsoft.com/en-us/windows/privacy/required-diagnostic-data（猜测 URL） | 同上 | **failed，HTTP 404**，不重试（留有 `a-ms-required.*` 空壳，待删） |
| 19:25:21–25 | 二手稿 10 篇（`a-fetch-list-2.tsv` 另含 Learn 三条）：tech.fxxz.com、telegra.ph、gigazine.net（会员全文，只得标题与摘要）、theterminal.space、warpnav.com、blocktempo.com、news.lavx.hu、reveneau.com、glonce.com、gptclub.ai | 同上 | 全部 ok（`a-m-*`；加 list 3 的 5 篇共 15 篇）；`ok` 已打开 txt 目视是正文 |
| 19:25:46 | HN：`--item 50524736` 与标题搜索 | `barking heat docs/2611/1116 a` | 624 分 / 481 评，提交北京 11-17 06:58:20。`a-heat.json`、`a-hn-50524736.json` |
| 19:25–19:28 | Exa 检索（`web_search_exa`，共 6 次：首轮英文媒体、中文媒体、微软回应三次；其后英文大站、中文大站、Tessera/Cloud Updates 三次；专查微软回应的 1 次） | Exa MCP | 仅作线索，产出二手稿清单；未发现微软回应；Tessera / Cloud Updates 无命中 |
| 19:28:02 | 二手稿补抓：80aj.com、ai-club.cn（OSChina 转载）、drweb.de、ilsoftware.it、bytechap.com；fxtwitter 线程 https://api.fxtwitter.com/2/thread/2122147747390468384 与作者资料 https://api.fxtwitter.com/CrackmesOne（`a-fetch-list-3.tsv`） | `barking fetch` | 全 ok。线程里作者只有 2 条（02:00:25Z 与 02:16:43Z）；回帖不在其中 |
| 19:28 | 博文三张图：agent-names.jpeg、codex-matcher.jpeg、us-optional-diagnostics-default.png（后者实为 JPEG，3440×1440） | curl（`--max-time 60`） | 成功。`a-blogimg-*`，不入库 |
| 19:29–19:31 | 博文整页渲染（700 CSS px，DPR 2，1400×18400；另做 900 px 宽版供比对，未采用）；HN 页；X 嵌入页（headless 版空白，**放弃**）；Learn configure 页与支持页整页；飞翔科技频道页 | `barking shot render` | 中间图在系统临时目录 `barking-shot/26111116-a/` |
| 19:3x | X 嵌入页 https://platform.twitter.com/embed/Tweet.html?id=2122147747390468384&theme=light | opencli browser open + eval（zoom=2，等待图片加载）+ screenshot | 成功；帖子卡片完整，匿名（有 “Follow” 按钮，未登录）。中间图在临时目录 |
| 19:3x | Learn configure 页用 opencli 滚动截图 | opencli | 滚动定位失败（截到页脚），**放弃**，改为整页渲染后切片 OCR |
| 19:3x | 整页 OCR | tesseract 经 `barking shot lines` | 48000 px 高的整页超过 tesseract 上限，改用 `sips` 把整图按 6000 px 切片存到 scratchpad，再 `lines --from` 找锚点 |
| 19:38:29–45 | Reddit 搜索两条（“Windows 11 coding agents telemetry”“MissionControlAggregator”） | `barking heat docs/2611/1116 a-r --reddit …` | **失败：重定向到 Reddit 登录页**，未取（`a-r-heat.json` 的 errors） |
| 19:38 | Wayback 可用性 API → https://web.archive.org/web/20261116230024id_/https://xusheng.dev/posts/windows-coding-agent-telemetry/ | curl（`--max-time 40`）+ `barking fetch`（`a-fetch-list-4.tsv`） | 成功（第一次可用性 API 在 5 秒超时，第二次返回）。快照时间戳 UTC 2026-11-16 23:00:24（北京 11-17 07:00:24）；`a-wayback-blog.txt` 与 `a-blog.txt` 逐字相同。**仅用于证明“当时已有同样正文”** |

仅见 Exa 检索摘要、**未存档**：AI Tracker、dropdigital.pl、hegai.media、insightex.si、ai-daily.ax0x.ai、renzhiwake.com 等。

被拒或失败：Learn `required-diagnostic-data`（404）、X 嵌入页 headless 渲染（空白）、opencli 滚动截图、tesseract 整页 OCR（过高）、Reddit（登录重定向）、Wayback 第一次（超时）。未做：他人回帖（匿名不可见）、GIGAZINE 会员全文、微软 Feedback Hub / Message Center、博文 CV PDF。

## 截图

均匿名；逐张已打开确认文字清楚、范围正确。`images/` 里 13–23 号及 `b-tmp-*` 是 B 组的文件，不是本组。

| 文件 | 来源页 | 说明 |
|---|---|---|
| `01-blog-title.png` | xusheng.dev 博文 | 站点导航、标题、副标题（1400×1010）；**深色底页面** |
| `02-blog-default-claim.png` | 同上 | 开头两段：AI 辅助 + “enabled by default … uploaded to Microsoft”（1400×1050）；深色底 |
| `03-blog-agent-names.png` | 同上 | “Following the names” 标题 + Binary Ninja 字符串图 + 4 个 CLI 槽 / 5 个桌面槽段（1400×1450）；深色底。字符串图内字很小，原图另存 `sources/a-blogimg-agent-names.jpeg` |
| `03b-blog-devtools-publisher.png` | 同上 | `CLICodingAgentUsage`、开发工具名单与 publisher 检查（1400×640）；深色底 |
| `04-blog-discard-bitmap.png` | 同上 | “bit” 与 “appears to discard those details … bitmap” 与 “no prompt text or source code” 段（1400×730）；深色底 |
| `05a-blog-feature-on-vm.png` | 同上 | “Is it actually running on a normal machine?” 标题与 VM 上功能已启用段（1400×820）；深色底 |
| `05b-blog-baseline-vm.png` | 同上 | “The public ISO’s baseline VM … disabled by image default” 段（1400×280）；深色底 |
| `06-blog-rename-test.png` | 同上 | “Testing it in practice”：cmd.exe 改名 codex.exe（1400×700）；深色底 |
| `06b-blog-25h-etw-event.png` | 同上 | 窗口起点拨前 25 小时、Provider/Time/Version/CliAgentSubCohorts 事件与 “1 << 2” 句（1400×940）；深色底。代码块里 Provider 行被页面宽度截断（站点自身显示） |
| `07-blog-queue-json.png` | 同上 | “When optional diagnostic data is allowed … queues” 句与上传队列 JSON（1400×880）；深色底。`"name"` 行被页面宽度截断，完整事件名见 txt |
| `07b-blog-us-default-optional.png` | 同上 | 美国区域干净安装、Include Optional 的 OOBE 截图与结论段（1400×1280）；深色底 |
| `08-ms-learn-optional-app-activity.png` | learn.microsoft.com configure 页 | “Optional diagnostic data” 小节与 “App activity, …” 条目（1400×1290）；**深色底页面**（渲染时页面跟随系统深色） |
| `09-x-post-card.png` | X 官方嵌入页（opencli） | 作者首发帖卡片（1100×1300）：正文、三张图、10:00 AM · Nov 16, 2026（北京时间）、3.2K 赞、“Read 117 replies”。卡片内含作者（Crackmes.one / @CrackmesOne，公共账号，兼作者个人号）的头像与账号名，这是被引帖子的发布者，不是抓取者；匿名视图 |
| `10-hn-title.png` | news.ycombinator.com/item?id=50524736 | HN 标题、624 points、483 comments、cwaffles、12 hours ago（1400×240）；匿名，右上角为 “login” 而非用户名 |
| `11-fxxz-default-claim.png` | tech.fxxz.com 文章 | 二手夸大实例：标题、导语、“这一功能默认启用…”段（1400×830）；**深色底页面** |
| `12-ms-support-optional-sample.png` | support.microsoft.com 隐私文档 | 必需/可选诊断数据定义与 “small percentage of devices (sample)” 段（1400×1890）；浅色底；页头与翻译插件图标未入框 |

编号说明：A 组区间 01–12；03b、05a、05b、06b、07b 用字母后缀，由 Claude 整理。未再单独截“HN 评论”“二手夸大实例（Reveneau、bytechap）”——后两者只存了文本存档（`a-m-reveneau.*`、`a-m-bytechap.*`），需要图的话可再补。

## 隐私自查

对 `docs/2611/1116/` 里本组文件 grep 本机用户名、家目录路径、邮箱：无命中（命中项只有作者关于页上其本人公开的邮箱与 discord 名，以及 HN 评论里谈论 Linux/Proton 的路径示例）。fxtwitter JSON、HN 评论只含被引者的公开信息。截图里 HN 页为匿名视图，X 卡片为匿名嵌入页。

## 待删除文件清单（不调用 rm，由 Claude 收尾时一次删除）

- `docs/2611/1116/sources/a-rss2.xml`、`a-rss2.headers`（与 `a-rss.xml` 重复的 RSS）
- `docs/2611/1116/sources/a-ms-required.html`、`a-ms-required.headers`（猜测 URL 404 的空壳）
- `docs/2611/1116/sources/a-hn-algolia.json`、`a-hn-algolia.headers`（与 `a-hn-50524736.json` 逐字节相同；`evidence-A.md` 已改引后者）

单个超过 1 MB 的文本存档：无（`a-rss.xml` 771 KB、`a-hn-page.html` 733 KB 均不足 1 MB；`.html` 原件按规定不入库）。
