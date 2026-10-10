# 1009 期 A 组抓取日志

时间为北京时间（UTC+8）；本机美东 10/9 夜间 = 北京 10/10 上午。下列“约时刻”为命令发出时近似值（会话内 `date -u` 读数 2026-10-10T03:33:41Z = 北京 11:33:41）。截图精确时刻见 `a-browser-records.jsonl`、`a-shots-log.jsonl`。

工具：curl（普通浏览器 UA，匿名）、firecrawl（`firecrawl_scrape` / `firecrawl_search`）、Playwright Core + 本机 Chrome（匿名、全新临时 profile，不登录，脚本 `a-browser.mjs`，改写自 1007 `b-browser.mjs`）、headless Chrome 整页渲染（`a-shot.py render`，仅用于看版面）、HN Algolia（`a-heat.py`）、PIL。未使用 opencli / agent-reach，未运行读取浏览器 Cookie 的工具，未登录任何站点。

## 1. 抓取

| 约时刻 | URL | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 11:34 | https://www.anthropic.com/research/investigating-unintended-model-actions | curl | 200，165 KB，正文完整 | `a-anthropic-report.html/.txt`（txt 由 `a-html2txt.py` 生成） |
| 11:34 | https://www.cbsnews.com/news/philadelphia-police-anthropic-ai-false-homicide-tip/ | curl | 200，624 KB | `a-cbs.html/.txt` |
| 11:35 | WaPo 两篇（curl，无 -m） | curl | 连接无响应，被后台超时，**失败**（exit 35） | 无 |
| 11:36 | phillypolice.com：/news/、/、/wp-json/wp/v2/{search,posts,news-blotter,notifications,ppd_spotlight,pages,media}、站点地图、/?s=anthropic；phillyunsolvedmurders.com；phila.gov/?s=anthropic、/media/(403) | curl | 200 但无 Anthropic 相关条目；news-blotter 最新为 10/9 07:08 寻人通告；phila.gov/media 403 | 仅试探，不存档（试验文件在临时目录） |
| 11:37 | firecrawl_search ×~12（警方稿、visa、State Department、HN/Reddit、中文、夸大标题等） | firecrawl | 得到链接与摘要；个别 500 错误重试 | 引用于 evidence-a.md |
| 11:38 | https://www.reuters.com/world/us/anthropic-ai-model-submits-false-homicide-tip-police-website-2026-10-09/ | firecrawl_scrape（maxAge 0） | 200，正文完整 | 手工逐字存档 `a-reuters-tip.txt` |
| 11:42 | Anthropic 7/30、9/9、8/31 三页 | curl | 200 | `a-jul30/a-sep9/a-aug31.html/.txt`；首次循环误生成 `a-a-jul30.html`、`a-a-sep9.html`（同一页面的重复，已删除） |
| 11:43 | NBC10、Fox29、TechCrunch、6abc、AFP | curl | 200 | `a-nbc/a-fox29/a-tc/a-6abc/a-afp.html/.txt` |
| 11:43 | Startup Fortune | curl | 200 | `a-startupfortune.html/.txt` |
| 11:43 | NDTV Profit、Firstpost、WSJ | curl | 403 / 403 / 401，**失败**，已删除无效文件 | 无 |
| 11:44 | newscord.org、yahoo.com 聚合页 | curl | 200，聚合/转载页，仅作线索，**已删除** | 无 |
| 11:44 | https://www.nytimes.com/2026/10/09/technology/anthropic-rogue-ai-agents.html | curl / firecrawl_scrape | curl 403（挑战页，已删除）；firecrawl “不支持该站点” | 无 |
| 11:50 | Seattle Times 转载页（页内署 The New York Times，写明 “originally published at nytimes.com”） | firecrawl_scrape（onlyMainContent，maxAge 0） | 200，正文完整。**注意**：任务规则“不用镜像/转载站”，该页是 NYT 稿的授权转载（非抓取镜像），仅因 NYT 原站不可读而采用，状态栏已注明，由 Claude 取舍 | 手工逐字存档 `a-nyt-seattletimes.txt` |
| 11:50 | WaPo 两篇 | firecrawl_scrape（maxAge 0） | 200；第一篇付费墙，只见导语；第二篇正文可见。不绕过 | 摘记 `a-wapo-paywall.txt` |
| 11:46 | HN Algolia `search`（10 组关键词）+ `items/<id>`（50027118、50028239、50025713、50028121、50028365、50029330） | python urllib / curl | 200；取得时刻 2026-10-10T03:45:39Z / 03:46:09Z | `a-heat.json`、`a-hn-<id>.json` |
| 11:47 | https://www.reddit.com/r/philadelphia/comments/1x1uavx/.json、/search.json | curl | **403（Reddit 拦截）**，不硬绕；返回的 190 KB 拦截页已删除 | 无 |
| 11:48 | 币界网 528btc（curl 985 B JS 壳；firecrawl 仅 meta，正文需登录）；marsbit | curl / firecrawl | 528btc 仅得 meta（已删除 curl 壳）；marsbit 200 | `a-marsbit.html/.txt`；528btc 只在 evidence 引 meta |
| 11:52 | NYPost、Fox Business、CTV | curl | NYPost 200（Reuters 稿）；Fox Business、CTV 无响应；Fox Business 另用 firecrawl_scrape 取得 200，正文只在 evidence 中摘引（页内嵌 Anthropic X 帖，含头像链接，**未存档**以免带入第三方信息） | `a-nypost.html/.txt` |
| 11:53 | 6abc 页面（headless Chrome，Playwright） | a-browser.mjs | 成功；渲染文本 `a-6abc-browser.txt` | 同左 |

## 2. 截图

2 倍像素比、宽 1400 px；用 Playwright 按元素 innerText 起止定位，只做裁切，不改像素。01–04、06–11 来自 Anthropic 报告页（700 CSS px 宽，11:53–11:55）；05 来自 6abc（11:57）。03 为两段（“Although the impact…”段与 Remediation 首段）纵向拼接，中间 40px 背景色空白；05 删除了页面中间的广告空白带（784–1454 px），前后各保留 60px。边界落在文字行之间（第二轮 pt/pb=10 后重截，第一轮的顶部残字/底部露出下一段已消除）。已逐张目视确认（01 全图；02 全图；01/03/04/06 拼图总览；07/08/09/10/11/02 拼图总览；05 上下两半总览）：文字清楚、范围正确，无受害人姓名与街道（报告正文本身已用 “[the street named on the page]”）。

## 3. 清理记录

删除（均为本人生成、无效或重复的文件）：`a-a-jul30.html`、`a-a-sep9.html`（重复）、`a-nyt.html/.txt`（403 页）、`a-newscord.*`、`a-yahoo.*`（聚合页）、Reddit 403 页、528btc curl 壳、NDTV/Firstpost/WSJ 无效页；`images/` 下 03 的临时片段在临时目录。第一轮截图用 pt/pb 24–30 生成后被第二轮覆盖（OVERWRITE=1）。

## 4. 隐私与泄露自查

本组入库文本中不含抓取者头像、显示名、handle；未存任何 X 页面。费城案件：受害人姓名与街道在已存档页面与报告中均未出现（警方声明与报道只写 “the street named on the page”）。因此未做 `[REDACTED]` 替换。交付前对 `docs/2610/1009/` 做了 grep（见回报）。

## 5. 假设

- 本机时区美东，北京 = 本机 + 12h；EDT 时间换算 +12h。
- Seattle Times 页作为 NYT 稿的授权转载使用，已显式标注。
- HN 评论“得分”接口不提供，故评论链接不附分数，仅附帖子总分。
