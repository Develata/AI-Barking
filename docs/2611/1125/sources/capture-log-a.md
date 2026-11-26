# 1125 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，时间为北京时间（本机时钟，UTC+8），起 21:38，截至 22:05 完成主体。批量抓取逐条结果在 `a-fetch-log.jsonl`，清单 `a-fetch-list-1.tsv … -7.tsv`，截图动作在 `a-shots-log.jsonl`，热度 `a-heat.json`。批量抓取走 `barking fetch`（每条限时 60 s）；个别一次性请求用 `curl --max-time 60/120`；渲染页与已登录页面的只读阅读用 opencli（session `a-shot`）。不用 firecrawl，未运行读取浏览器 Cookie 的工具，未登录、未发帖、未点赞、未关注。浏览器里的 X、Reddit 是维护者已登录的会话，只做只读浏览；X 帖子卡片截图只含帖子本身（已裁掉导航、头部与回复框），存档 JSON 均来自 fxtwitter 公开接口（只含被查帖子的公开字段）。

## 一、批量存档（barking fetch）

| 批 | 北京时刻 | 内容 | 结果 |
|---|---|---|---|
| 1 | 21:39:32 | help 20001625、20001516、11369540（Codex rate card）、developers.openai.com/codex/pricing、sovereignmagazine 稿、凤凰网（智东西稿）、腾讯新闻（Odaily）、codex/changelog、status.openai.com | 9 条全部 ok。`a-codex-pricing.*` 实为 learn.chatgpt.com/docs/pricing 重定向后的同一页（头里 URL 即此）；页头 Last-Modified Thu, 26 Nov 2026 12:01:46 GMT（北京 20:01:46） |
| 2 | 21:39:54 | help 20001275、learn.chatgpt.com/docs/pricing（同上页）、help 20001498（banked reset，不展开） | 3 条 ok |
| 3 | 21:44:14 | openai.com 发布页（introducing-gpt-6-2-astra-sol）、codex/agent-configuration/speed、codex/models、techcrunch.com/tag/openai（只为找 11-25 那篇的 URL） | 4 条 ok |
| 4 | 21:44:24 | TechCrunch 11-25 稿 | ok（`a-techcrunch-1125.txt`；“12:03 PM PST · November 25, 2026”） |
| 5 | 约 21:47 | 网易号（智东西同稿）、Business Insider 台湾版、鉅亨网（BlockBeats）、80aj（聚合站，未采用）、Odaily 原站 | 4 条 ok；**`cn-bitw`（businessinsider.tw/article/10313）403 Cloudflare 挑战，blocked，未绕过**（其 .html/.headers 是挑战页） |
| 6 | 21:54:14 | help 9793128（Pro 套餐）、help 12642688（个人版 credits） | 2 条 ok |
| 7 | 约 21:58 | 新浪（宝玉xp 微博稿）、Yahoo 香港（newmobilelife 稿）、金融站 jhth（未采用）、Notebookcheck 中文版、V2EX、LINUX DO | 新浪、Yahoo 香港、jhth ok；**Notebookcheck、V2EX、LINUX DO 均 403 Cloudflare 挑战（blocked，未绕过）**。Notebookcheck 改用 opencli 浏览器读页面文本，存 `a-cn-notebookcheck.txt`（导语两句是浏览器读到的页面文本，另行附在文件末尾）；V2EX、LINUX DO 只在搜索结果里见到标题，没有读到正文 |

每个 `ok` 条目都打开 `.txt` 看过：help 20001625、20001275、20001516、凤凰网、TechCrunch、Odaily、鉅亨、Yahoo 香港、新浪均为正文；`a-codex-pricing.txt`、`a-codex-changelog.txt` 为整页文本（含导航），目标段落已用 grep 与截图确认。

## 二、Wayback 快照（只用来看“页面以前写了什么”，非官方）

| 北京时刻 | 对象 | 方式 | 结果 |
|---|---|---|---|
| 21:4x | `archive.org/wayback/available` 与 CDX 查询 learn.chatgpt.com/docs/pricing、help 20001625、help 20001275 | curl | 有快照：pricing 7 个（UTC 11-15 07:25、11-18 03:56、11-19 16:12、11-23 01:52、11-24 14:05、11-25 14:05、11-26 03:16）；20001275 两个（11-16 21:25、11-18 00:25）；20001625 在 11-17 之后、11-26 之前没有快照。availability 接口有一次 5 秒超时，改用 CDX |
| 21:4x | 取 pricing 的 7 个快照（`web.archive.org/web/<ts>id_/…`） | curl `--compressed` | 存 `a-wayback-learn-pricing-<ts>.html/.txt`（第一次忘了 `--compressed` 得到 gzip 乱码，已用 `--force` 重转，乱码的 txt 已被覆盖）。比较结果见 evidence A4-2 |
| 21:4x | 取 20001275 的 11-18 00:25 UTC 快照 | curl `--compressed` | 存 `a-wayback-help-20001275-20261118002513.html/.txt`；Ultrafast 6× 规则与现页相同 |

## 三、X（fxtwitter 公开 JSON 与 opencli 只读）

- `curl https://api.fxtwitter.com/status/<id>`（21:3x–21:4x）：2125784147889393806（Tibo 原帖，存 `a-x-tibo-2125784147889393806.raw.json`；名字里的 “.raw” 是我留的，内容就是 fxtwitter 原样 JSON，字段只有被查帖子作者的公开信息）、2125428139233054830 / 2125426980686602732 / 2125427689460785413 / 2125513664027615251 / 2125586570749337813（用户转贴邮件的帖）、2122753991000133943 / 2122754551187132438 / 2122929829863735594 / 2122929634916679820（Tibo 11-17、11-18 的帖）→ `a-x-<id>.json`；`/2/thread/<id>`（只有 1 条）、`/thsottiaux` 用户资料（未存档）。
- opencli 只读：`opencli twitter thread 2125784147889393806 --limit 300`（取到 96 条回复，其中 Tibo 本人 0 条，输出只留 scratchpad，未入库）；`twitter tweets thsottiaux --limit 40`（找 Tibo 上线帖与 11-25 合并帖的 ID，未入库）；`twitter search`（5 次：邮件相关用户帖、`from:thsottiaux since:2026-11-25 filter:replies`、`from:OpenAIDevs GPT-6.2 usage`、`from:OpenAI usage GPT-6.2 billing`），只读摘要，未入库，未点赞、未回复。
- 用户邮件截图（email1.jpg、email2.jpg 等）只下载到 scratchpad 目视读文本，没有入库（转贴图，规则“不用转载图”）。邮件英文原文已抄进 evidence A3-4。
- 帖子卡片截图：x.com 页面用 `opencli browser`（session `a-shot`）打开，用 JS 把左栏、侧栏、回复框、Grok/私信抽屉设为 display:none，`zoom=2`，`screenshot --width 1400`，再 `barking shot cropbox` 只裁出帖子。官方嵌入页 `platform.twitter.com/embed/Tweet.html` 先试了（`barking shot render` 空白；opencli 能渲染但长帖被截成 “Show more”，只见前半），改用 x.com 页面。

## 四、其他请求

- `barking heat`（21:42:51）：`--item 50651117`（HN，存 `a-hn-50651117.json`）、HN 搜索 “GPT-6.2 Astra usage”“GPT-6.2 billing”；两条 Reddit 搜索被重定向到登录页（失败，记入 `a-heat.json` 的 errors）。
- Reddit（21:55–21:58）：`opencli reddit search` → SECURITY_BLOCK（403）；改用 opencli 浏览器打开 r/codex 搜索页（只读）取得三帖的票数与评论数，并在页面内 `fetch('/r/codex/comments/1y5q43m.json')` 取该帖的公开评论 JSON。该响应含登录会话令牌字段，**原始响应只在 scratchpad，没有入库**；入库的 `a-reddit-codex-1y5q43m.json` 只含帖与评论的公开字段（id、作者名、分数、时间、正文、permalink）。已 grep 确认入库文件里没有该字段。
- WebSearch（7 次）：找官方页、中文稿、TechCrunch 11-25 稿（只搜到标签页，URL 从标签页读出）、Business Insider 稿；搜索结果只作线索，引用的页面都已回到原站读过或标为“只见标题”。
- ITmedia AI+：没有找到（两次搜索无命中，未做站内检索）。

## 五、截图（逐张已打开目视确认）

| 文件 | 来源与方法 | 确认 |
|---|---|---|
| 01-tibo-post-card.png | x.com（opencli，zoom 2，见三）→ cropbox | 全文可见，含 “10:50 AM · Nov 26, 2026”“3.7M Views”与互动数；无登录者信息；深色底 |
| 02-changelog-62-entry.png | `barking shot render developers.openai.com/codex/changelog?lang=en`（700 px，DPR 2，28000 px 高中间图）→ cropbox | 2026-11-17 条目完整；下方有一块黑底空白 |
| 03-pricing-plus-estimates.png | `render developers.openai.com/codex/pricing?lang=en`（700 px，DPR 2）→ cropbox | “Included usage”节与整张估计表齐全，含表下 “Local messages …” 句；深色底 |
| 04-help-usage-limits.png | opencli（zoom 2，隐去横幅）→ cropbox | 全文可见；页头显示“Updated: 前天”（浏览器本机语种），无头像/登录栏 |
| 05-ifeng-zhidx-title-lede.png | opencli 打开凤凰网页（zoom 2，滚动触发懒加载）→ cropbox | 标题、署名、时间“2026年11月26日 11:04:36 来自北京”、导语、“40%和25%”句清楚；页内“广告”条与头图在内 |
| 06-ifeng-zhidx-email-marketing.png | 同页，两次滚动截图 → `barking shot stack`（4 段，间隔 28 px，背景白） | 邮件段、“不少用户并不买账”、API 价、Reddit 观点、“仍会发现…事实好像并非如此”、“用户反馈称”、“营销方式”句、文内 Reddit 评论图（稿件自己已打码用户名）均清楚；段间的广告空白已被拼接去掉，段落边界落在文字行之间 |
| 07-hn-title-score-v2.png | `render news.ycombinator.com/item?id=50651117`（700 px，DPR 2）→ stack 2 段 | 标题、346 分、192 评、顶评全文清楚，底边无半行字 |
| 08-hn-comments-quota-data-v2.png | `render …item?id=50653971/50652147/50653734`（DPR 2）→ stack 3 段 | 三条评论全文与各自头行“on: OpenAI cuts GPT-6.2 usage…”清楚；底边有一线下一个回复输入框的上缘（页面元素） |
| 09-openai-launch-usage-sentence.png | opencli 打开 `openai.com/en-US/index/…`（`/en-US/` 才是英文；默认路径被浏览器本机语种跳成中文；zoom 2）→ cropbox | 首段“fewer output tokens, so you can get more done within your usage limits”完整；深色底；已避开浏览器翻译浮标 |
| 10-pricing-token-rates.png | pricing 同一张 DPR 2 中间图 → stack 2 段 | Token rates 表、“Credit prices alone don’t determine included subscription usage”句、USD 价句清楚；深色底 |
| 11-tibo-launch-efficiency-post.png | x.com（同 01）→ cropbox | 正文、Agents’ Last Exam 图、时间 “2:11 AM · Nov 18, 2026”、互动数清楚；页面粘性标题栏遮住了该帖头部，卡片不含账号头部；深色底 |
| 12-reddit-codex-post-summary.png | opencli 打开 reddit.com（只读，隐藏页头与侧栏，**1 倍像素**，因 zoom 2 会让 Reddit 版式被裁）→ stack 2 段 | 帖标题、内嵌 Tibo 帖图、902 票/470 评、置顶 GPT 摘要全文清楚；不含登录栏与广告；约 450 px 宽，只作补充 |

2 倍像素比的例外：12（1 倍）。其余约 2 倍（02、03、10 为 headless 700 px 版式 DPR 2）；宽度均 ≤1400 px。以上 12 张图是 A 组 01–12 区间全部。

## 六、失败与未做

| 对象 | 原因 | 处理 |
|---|---|---|
| Business Insider 台湾版稿（businessinsider.tw/article/10313） | 403 Cloudflare 挑战 | 只见搜索结果里的标题；未绕过 |
| Notebookcheck 中文版 | curl 403 Cloudflare 挑战 | 用 opencli 浏览器读页面文本（一次读取时的页内时间 “26.11.2026 14:46”，时区未标） |
| V2EX《OpenAI 又砍额度了》、LINUX DO《GPT你真无敌了…》 | 403 Cloudflare 挑战 | 只见标题 |
| ITmedia AI+ 原稿 | 搜索无命中 | 未找到 |
| IT之家稿 | 搜索无命中（扫描让“优先”看，但没找到对应稿） | 未找到 |
| Tibo 在该帖下的补充回复 | 抓到约 96/5226 条回复，无 Tibo 本人；`from:thsottiaux filter:replies` 无相关 | 记未找到 |
| 邮件第二封（HN Aurornis 提到的 clarification） | 没有找到原文 | 未找到 |
| 官方站对计费修正的书面说明 | 查过 help 20001625/20001275/20001516/9793128、套餐页、changelog、status | 未找到 |
| 帮助页 20001625 的 updatedAt 绝对时刻 | 静态 HTML 与页面状态里没有（1031 期是用 CDP 取的） | 只记“Updated: 2 days ago（21:39 读）” |
| help 20001625 的 11-18～11-26 Wayback 快照 | CDX 最后一个有效快照是 11-17 | 无法对比修正前后；只与 1031 期存档比，见 evidence A5-1 |
| `barking heat` 的 Reddit 搜索 | 被重定向到登录页 | 改用浏览器页面与页内 JSON（见四） |

## 七、待删除文件（精确路径，均在 `docs/2611/1125/`）

初版或试验图（均已被 v2 或新文件取代）：
`images/03-pricing-test-wide.png`、`images/07-hn-title-score.png`、`images/08-hn-comments-quota-data.png`、`images/09-openai-launch-usage-claim.png`。

失败抓取留下的挑战页（均无用）：
`sources/a-cn-bitw.headers`、`sources/a-cn-bitw.html`、`sources/a-cn-notebookcheck.headers`、`sources/a-cn-notebookcheck.html`、`sources/a-cn-v2ex.headers`、`sources/a-cn-v2ex.html`、`sources/a-cn-linuxdo.headers`、`sources/a-cn-linuxdo.html`。

只为找 URL 的标签页（无用）：`sources/a-techcrunch-try.headers`、`sources/a-techcrunch-try.html`、`sources/a-techcrunch-try.txt`。

可选（未采用的聚合站稿，保留也无妨）：`sources/a-cn-80aj.*`、`sources/a-cn-jhth.*`、`sources/a-sovereign-mag.*`。

仓库外：scratchpad 下的 `thread.json`（96 条 X 回复）、`rd1.raw`（含会话字段的 Reddit 原始响应）、各中间截图、邮件转贴图，以及 `/tmp/nbc.json`、`/tmp/nbc2.json`、`/tmp/x`（空目录）；会话结束后可清理，均不入库。`barking shot` 的中间图在 `/var/folders/…/barking-shot/26111125-a/`。

## 八、大文件与不入库项

- `sources/a-codex-changelog.html` 5.28 MB（只存本地，不入库）；`sources/a-codex-changelog.txt` 1.13 MB（文本，**超过 1 MB，入库前请先决定**：本组只用其中 2026-11-17 条目，可只留摘录）；`sources/a-status-openai.html` 0.98 MB（只存本地）。
- HTML/截图原件（`a-*.html`、`a-wayback-*.html`、`images/*.png`）按规则只存本地，不入库。
- 泄露自查：对 `docs/2611/1125/` grep 用户名与会话令牌字段名，只命中 B、C 组文件（它们自己的文字），A 组文件无命中；截图已目视确认无登录者头像、昵称与 handle。
