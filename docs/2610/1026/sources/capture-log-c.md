# C 组抓取日志（1026 期，微软约 6 万人可申请自愿离职）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8；本机美东 EDT，北京 = 本机 + 12 小时）。机器记录见 `c-fetch-log.jsonl`、`c-shots-log.jsonl`、`c-heat.json`。所有抓取未登录、未发帖、未输入凭据；`barking fetch` 为匿名 GET；opencli 为后台窗口的已有浏览器会话，只读页面、不点击、不接受 Cookie。HTML/PDF/大图只存本地。

## 一、barking fetch 批量存档（前缀 c；清单 `c-list.tsv`…`c-list6.tsv`）

| 北京时间 | name | URL | 结果 |
|---|---|---|---|
| 22:52 | msft-blog | https://blogs.microsoft.com/blog/2026/10/26/introducing-a-voluntary-exit-offer/ | ok（203 行文本） |
| 22:52 | seattletimes | https://www.seattletimes.com/business/microsoft/microsoft-offers-mass-buyouts-for-roughly-a-quarter-of-its-workforce/ | ok（704 行；匿名 GET 返回全文 HTML，但页面渲染被订阅弹窗遮罩，见第四节） |
| 22:52 | reuters-bi | https://www.reuters.com/technology/microsoft-is-offering-buyouts-around-60000-employees-business-insider-reports-2026-10-26/ | blocked（HTTP 401，无 txt）；改用 opencli，见第二节 |
| 22:52 | hn-api-search | https://hn.algolia.com/api/v1/search?query=microsoft%20buyouts&tags=story&numericFilters=created_at_i%3E1792900000 | ok（JSON，13.5 KB） |
| 22:52 | msft-fy26-10k | https://www.microsoft.com/investor/reports/ar25/index.html | ok（实为 FY2025 年度报告，清单名误起，内容用于 C3 的 228,000 上年基数） |
| 22:54 | bi-memo | https://www.businessinsider.com/microsoft-is-offering-buyouts-read-the-memo-2026-10 | ok（匿名 GET 返回 HTML，页面标 “available exclusively to Business Insider subscribers”，见第四节） |
| 22:54 | cnbc | https://www.cnbc.com/2026/10/26/microsoft-msft-voluntary-buyouts.html | ok |
| 22:54 | verge | https://www.theverge.com/tech/1018666/microsoft-expands-buyouts-60000-employees-eligible | ok |
| 22:54 | kuow | https://www.kuow.org/technology/2026-10-26/microsoft-offers-voluntary-buyouts-to-a-quarter-of-its-workforce | ok |
| 22:54 | ars | https://arstechnica.com/gadgets/2026/10/microsoft-offers-voluntary-buyouts-to-60000-employees/ | ok |
| 22:54 | yahoo | https://finance.yahoo.com/technology/article/microsoft-offers-voluntary-buyout-to-60000-employees-160011034.html | ok 但 txt 只有 “Oops, something went wrong” 类内容，无正文，未使用 |
| 22:54 | nyt | https://www.nytimes.com/2026/10/26/technology/microsoft-buyouts-ai-spending.html | ok（匿名 GET 得到正文；NYT 为订阅站，见第四节） |
| 22:54 | bloomberg | https://www.bloomberg.com/news/articles/2026-10-26/microsoft-to-offer-buyouts-to-60-000-workers-in-latest-cost-cut | ok 但只有标题与 “Takeaways by Bloomberg AI” 摘要，正文付费墙 |
| 22:54 | hn-item | https://news.ycombinator.com/item?id=50245524 | ok（1.2 MB html 仅本地；txt 可入库） |
| 22:54 | hn-algolia | https://hn.algolia.com/api/v1/items/50245524 | ok（JSON） |
| 22:57 | msft-ar26 | https://www.microsoft.com/investor/reports/ar26/index.html | ok |
| 22:57 | msft-fy26q4-pr | https://www.microsoft.com/en-us/investor/earnings/fy-2026-q4/press-release-webcast | ok（财报新闻稿页，未用） |
| 22:57 | msft-jul-blog | https://blogs.microsoft.com/blog/2026/07/06/the-latest-in-our-company-transformation/ | ok |
| 22:57 | geekwire | https://www.geekwire.com/2026/microsoft-makes-new-voluntary-exit-offer-to-more-than-a-quarter-of-its-workforce-including-some-execs/ | blocked（HTTP 403），未取 |
| 22:59 | msft-q4-call | https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q4 | ok（电话会稿，Wednesday, July 29, 2026） |
| 22:59 | msft-q3-call | https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q3 | ok（Wednesday, April 29, 2026） |
| 22:59 | cnbc-q4 | https://www.cnbc.com/2026/07/29/microsoft-msft-q4-earnings-report-2026.html | ok |
| 23:01 | theweek | https://www.theweek.in/news/biz-tech/2026/10/27/even-vp-jobs-are-not-safe-from-ai-what-microsofts-voluntary-layoffs-mean.html | blocked（拒绝访问页），未取 |
| 23:01 | ibtimes-sg、investmentnews、sina-fin、sina-fin2、kr36、ifeng、sina-xwn、cnbc-apr、cnn-apr | 见 `c-list5.tsv` | ok（cnn-apr 5.6 MB html 仅本地，txt 可入库） |
| 23:01 | gulfbusiness | https://gulfbusiness.com/en/2026/tech/microsoft-offers-buyouts-to-60000-employees-amid-ai-spending-push/ | blocked（HTTP 403），未取 |
| 23:02 | sina-xwn2、watcherguru、siliconangle、ctv | 见 `c-list6.tsv` | ok |

## 二、opencli 浏览器

| 北京时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 22:56 | Reuters 原文（同上） | `opencli browser c-rt open … --window background` + `eval document.body.innerText` | 页面可读，文本存为 `c-reuters-bi.txt`（我自己写的 DOM innerText 导出）。随后截图发现页面有 “Subscribe to Reuters to continue reading” 弹窗遮挡，因此 Reuters 的文本取自被弹窗盖住的 DOM，不该算“公开可读”；建议不入库、不作为引文依据（其核心事实有 CNBC、NYT、Verge 可替代），由 Claude 决定 |
| 23:05–23:30 | 微软官方博客两篇、CNBC、The Verge、微软年报页、SiliconANGLE | `opencli browser c-shot open` → `eval document.documentElement.style.zoom='2'` → `screenshot --width 1400 [--full-page|--height N]`，再用 `barking shot cropbox` 裁切 | 成功，成图见 `images/25–36`；每次裁切记入 `c-shots-log.jsonl` |
| 23:10 | Seattle Times | 同上 | 页面被灰色订阅弹窗遮罩、正文只露出首段（`c-tmp/st-full.png`），未出成图；`barking shot render`（headless Chrome）被 CloudFront 拦截（“The request could not be satisfied”） |
| 23:13 | Reuters | 同上 | 订阅弹窗遮罩，未出成图 |
| 23:15 | Business Insider、IBTimes SG | `--full-page` 截图报 “Unable to capture screenshot”；IBTimes 非整页截图满是广告 | 未出成图 |
| 23:22 | 微软 Q4/Q3 电话会稿页 | `eval` 锚点滚动 + 截图 | 滚动后截图位置不对（页面内部滚动），未出成图；文本存档完整，摘句见 `evidence-c.md` |
| 23:28 | 微软 7/6 博客 | 先 `--window background` 截图超时（`Page.captureScreenshot timed out after 60s`），改 `--window foreground` 成功 | 出图 28、34 |

## 三、热度（barking heat）

| 北京时间 | 命令 | 结果 |
|---|---|---|
| 23:02 | `barking heat docs/2610/1026 c --item 50245524 --item 50250937 --hn "Microsoft buyouts" --reddit "Microsoft voluntary exit offer buyouts 60,000" --since 1792900000` | HN 搜索命中 13 条，两条目写入 `c-hn-50245524.json`、`c-hn-50250937.json`、`c-heat.json`；Reddit 被重定向到登录页，失败（`c-heat.json` 的 errors 有记录），未再试 |

## 四、付费墙与不确定处（须 Claude 过目）

- Seattle Times、BI、NYT、Reuters 都是有订阅/计量墙的站。除 Reuters（401）外，匿名 `barking fetch` 都收到了含正文的 HTML，没有任何绕过动作；但浏览器里 Seattle Times、Reuters 显示订阅弹窗。是否把这几份 `.txt` 入库、是否在正文里引它们的原句，由 Claude 判断。我没有为它们截图，也没有去掉遮罩。
- 因此 `evidence-c.md` 里 Seattle Times、BI、Reuters 的摘句只当“线索/L4 转述”，微软官方页面、CNBC、The Verge、微软年报与财报电话会稿是可截图/可核对的页面。
- `c-yahoo.txt` 无正文；`c-bloomberg.txt` 只有摘要；`c-msft-fy26-10k.*` 实为 FY2025 年度报告。

## 五、文件体积与入库建议

- 超过 1 MB 的文件：`c-cnn-apr.html`（5.6 MB）、`c-hn-item.html`（1.2 MB），均为 `.html` 原件，只存本地不入库；其余文本文件均小于 1 MB（最大 `c-hn-algolia.json` 约 0.58 MB）。
- `c-tmp/` 目录（含整页截图原图、辅助脚本 `shotfn.sh`、`anchorshot.sh`、`anchorshot2.sh`）是本组工作中间物，不入库。
- 自查：`grep -i` 未在 `c-*` 文件中发现抓取者的邮箱、昵称或本机用户名；HN JSON 含 HN 公开用户名与评论，属公开内容。

## 六、截图索引（`docs/2610/1026/images/`，均逐张打开核对）

| 文件 | 内容 | 来源 URL | 备注 |
|---|---|---|---|
| 25-msft-veo-intro.png | 官方博客标题、日期、署名、首两段（含 “Today, we’re introducing…”“builds on…Voluntary Retirement Program”） | blogs.microsoft.com/…/introducing-a-voluntary-exit-offer/ | 2× 像素，790 px 宽；左缘 4 px 灰色页面边 |
| 26-msft-veo-scope.png | “What employees should know”段：地区、level 70、“Some organizations…” | 同上 | |
| 27-msft-veo-full.png | 官方博客全文（用于证明无 AI/6 万字样） | 同上 | 790×3136 |
| 28-msft-jul-header-4800.png | 7/6 裁员信标题段与 “around 4,800 roles” | blogs.microsoft.com/…/the-latest-in-our-company-transformation/ | 标题首行被裁掉一半（“transformation”），可用 34 配合 |
| 29-cnbc-headline-keypoints.png | CNBC 标题、发布/更新时间、Key Points | cnbc.com/2026/10/26/microsoft-msft-voluntary-buyouts.html | 作者头像与署名在图内（记者公开身份）；1120 px 宽 |
| 30-cnbc-60000-27pct.png | CNBC 正文：“Around 60,000 people are eligible, according to a person familiar…”“about 27%” | 同上 | |
| 37-verge-headline.png | The Verge 标题、时间、摘要框（取代 31；超出 25–36 区间，因 25–36 已用满） | theverge.com/tech/1018666/… | 窄栏版式；已去掉浏览器翻译插件图标（两段拼接，无间隙） |
| 32-verge-participating-teams.png | The Verge 参与/不参与部门段 | 同上 | 450 px 宽 |
| 33-msft-ar26-employees.png | 微软 2026 年报 “As of June 30, 2026, we employed approximately 223,000 people” | microsoft.com/investor/reports/ar26/ | 只含正文段，不含顶部固定导航 |
| 34-msft-jul-not-replaced-by-ai.png | 7/6 信 “the roles eliminated today are not being replaced by AI” | 同 28 | |
| 36-siliconangle-affected-workers.png | SiliconANGLE 标题与 “approximately 60,000 workers affected”“job cuts”（夸大实例） | siliconangle.com/2026/10/26/… | 作者头像在图内（署名记者） |

## 七、待删除文件（精确路径，Claude 收尾时一次 rm）

- `E:\gitclone\AI-Barking\docs\2610\1026\images\29-cnbc-headline.png`（右侧被截断，已由 29-cnbc-headline-keypoints.png 取代）
- `E:\gitclone\AI-Barking\docs\2610\1026\images\31-verge-headline.png`（右上角带浏览器翻译插件图标，已由 37-verge-headline.png 取代）
- `E:\gitclone\AI-Barking\docs\2610\1026\images\35-siliconangle-affected-workers.png`（右缘混入侧栏，已由 36 取代）
- `E:\gitclone\AI-Barking\docs\2610\1026\images\c-tmp-cnbc-a.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\images\c-tmp-cnbc-b.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\images\c-tmp-rt-top.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\images\c-tmp-st-mid.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\images\c-tmp-st-top.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\images\c-tmp-verge-a.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\sources\c-tmp\`（整个目录，含整页截图原图与辅助脚本；`c-shots-log.jsonl` 里引用它的行可保留作记录）
- `E:\gitclone\AI-Barking\docs\2610\1026\sources\c-tmp$1-full.png`
- `E:\gitclone\AI-Barking\docs\2610\1026\sources\c-tmp${1}-full.png`（上两个是 shell 变量写错产生的误命名文件）
- 建议不入库（由 Claude 决定）：`c-reuters-bi.txt`（见第二节）；无用存档 `c-yahoo.*`、`c-msft-fy26q4-pr.*`（财报新闻稿页，未使用）、`c-theweek.html`、`c-gulfbusiness.html`、`c-geekwire.html`（blocked 页面）、`c-list*.tsv` 可保留。
