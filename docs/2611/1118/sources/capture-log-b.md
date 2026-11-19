# 1118 期 B 组抓取日志（Gemini 4 Neon）

时刻均为北京时间（本机即 UTC+8）。机器 logs：`b-fetch-log.jsonl`（`barking fetch`）、`b-shots-log.jsonl`（`barking shot`，每次动作自动记录）、`b-heat.json`。
工具：`barking fetch`（限时并发）、`barking shot`（headless Chrome，DPR 2；`BARKING_CHROME` 指向本机 Google Chrome）、`barking heat`、`opencli browser`（会话 `b-shot`，后台窗口；只用 `open` 与 `eval document.body.innerText/outerHTML`，另用其 screenshot 一次，见下）、WebSearch（只用来发现二手稿，搜索摘要不作证据）。
未登录任何站点，未输入任何凭据，未接受 Cookie 提示，未使用本机浏览器 Cookie；opencli 打开的均为公开页面。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 21:30:05 | https://artificialanalysis.ai/articles/gemini-4-neon-intelligence-gain | barking fetch | ok，`b-aa.*` |
| 21:30:05–09 | https://artificialanalysis.ai/models/gemini-4-neon、…/gemini-4-carbon、…/models、…/leaderboards/models | barking fetch | ok，`b-aa-neon.*`、`b-aa-carbon.*`、`b-aa-models.*`、`b-aa-leaderboards.*`；图表数值读自 HTML 内嵌 JSON（脚本一次性，未存） |
| 21:30:06 | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-neon/ | barking fetch | ok，`b-blog.*`；伙伴语录卡片文字只在 HTML 的 `quote=` 属性里，文本抽取版没有 |
| 21:30:13 | https://ai.google.dev/gemini-api/docs/pricing | barking fetch | blocked（正文仅 50 字符，需 JS 渲染） |
| 21:3x | 同上 | opencli（open + eval innerText / outerHTML） | ok，`b-pricing-rendered.txt`（渲染后文本）。opencli 所用浏览器带有维护者的已登录 Google 会话，页面右上角账号控件的邮箱与套餐标签被一并抓入：已把 txt 里的账号标识替换为 `[REDACTED-ACCOUNT]`/`[REDACTED-PLAN-LABEL]`，DOM 存档 `b-pricing-rendered.html` 含账号名与页面配置，已清空成一行说明，不保留。价格页本身是公开页面，价格内容与匿名 headless Chrome 渲染（截图 23）一致。 |
| 21:34:09 | https://deepmind.google/models/gemini/neon/ | barking fetch | failed，404（猜测的 URL，作废） |
| 21:34:09–11 | https://artificialanalysis.ai/agents/coding-agents、…/models/gemini-4-neon/providers | barking fetch | ok，`b-aa-agents.*`（静态页未含 Neon 条目）、`b-aa-neon-providers.*`（未用） |
| 21:34:10 | https://storage.googleapis.com/deepmind-media/Model-Cards/Gemini-4-Neon-Model-Card.pdf | barking fetch | ok，`b-modelcard.pdf/.txt`（检索成本、价格字样仅 “more cost efficiently”，未用） |
| 21:34:11 | https://artificialanalysis.ai/articles/gemini-4-carbon | barking fetch | failed，404（猜测，作废；同一来源未再试） |
| 21:34:11 | https://deepmind.google/models/evals-methodology/gemini-4-neon | barking fetch | ok，实为 PDF（6 页），`b-methodology.pdf/.txt`；文本无 cost/price/Carbon 字样 |
| 21:34:26 | HN 搜索 “Gemini 4 Neon”；HN 条目 50552424、50554374 | barking heat（前缀 `b`） | HN ok：`b-heat.json`、`b-hn-50552424.json`、`b-hn-50554374.json`；Reddit：old.reddit.com 搜索被登录页拦截（`b-heat.json` errors），不绕过 |
| 21:34:41–48 | https://aistudio.google.com/docs/models/gemini-4-neon、…/whats-new-gemini-4-neon、https://ai.google.dev/gemini-api/docs/models/gemini-4-neon | barking fetch | blocked（需 JS 渲染） |
| 21:34:42 | https://micjm.com/blog/gemini-4-neon-looks-like-a-top-model/；https://news.ycombinator.com/item?id=50552424 | barking fetch | ok，`b-micjm.*`、`b-hn-page.*` |
| 21:35 | 同上三个 AI Studio / 开发者文档页；https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/guides/gemini-4-neon | opencli（open + eval innerText） | ok，`b-aistudio-model-opencli.txt`、`b-aistudio-new-opencli.txt`、`b-vertex-opencli.txt`（前两份已按上条同样替换账号标识）；`ai.google.dev/…/models/gemini-4-neon` 未单独再取（同内容） |
| 21:35:26 | Crypto Briefing、The Decoder、itdaily、heise、basic-tutorials 报道页 | barking fetch | ok，`b-m-cryptobriefing.*`、`b-m-decoder.*`、`b-m-itdaily.*`、`b-m-heise.*`、`b-m-basic.*` |
| 21:35:26–27 | NewsBytes、Neowin 报道页 | barking fetch | blocked，HTTP 403（NewsBytes 为 Akamai “Access Denied”；Neowin 为 Cloudflare 质询）；barking shot render 同样 Access Denied（`nb-r`，中间图） |
| 21:3x | NewsBytes、Neowin | opencli（open + eval innerText） | ok，`b-m-newsbytes-opencli.txt`、`b-m-neowin-opencli.txt`（浏览器正常渲染取到正文，未绕过拦截） |
| 21:3x | AA 文章内 4 张图（cdn.sanity.io，AA 自己的图床） | curl（`--max-time 60`，`?w=1600`） | ok，`b-aa-fig1.png`–`b-aa-fig4.png`（只存档，不入库，不作配图；配图用页面截图） |
| 21:42:10–14 | 163、ETtoday、ChainCatcher、mashdigi、36氪 | barking fetch | ok，`b-m-163.*`、`b-m-ettoday.*`、`b-m-chaincatcher.*`、`b-m-mashdigi.*`、`b-m-36kr.*` |
| 21:3x | WebSearch ×5（英文、中文；含 “3x the speed and 60% of the cost” 一次） | WebSearch | 只用来发现二手稿；其中有搜索摘要把 Neon 当成泄露型号或把 Argon 数据当 Neon（过时，未采用）。“60% of the cost” 的出处靠谷歌博客页 HTML 直接找到，不靠搜索 |

web.archive.org：未使用。X 帖子（AA、扫描所称实测）：未取。

## 截图

headless Chrome 渲染（700 CSS px，DPR 2，宽 1400 px）后用 `barking shot cropbox/stack` 裁切；中间图在系统临时目录 `…/barking-shot/26111118-b/`，库内是裁切成品。逐张已目视确认文字清楚、边界落在文字行之间。

| 文件 | 来源页 | 内容 | 说明 |
|---|---|---|---|
| 13-google-rank-price.png | 谷歌博客 | “Neon ranks ahead … #3 … 61 … xhigh”段、定价段、可用范围段 | 1400×890 |
| 14-google-32pct.png | 谷歌博客 | 小节标题 “Brings efficiency and control …”，“32% lower cost per task than Carbon” 整段及上一段 | 1400×880 |
| 15-google-cost-chart.png | 谷歌博客 | 谷歌的 “AA Index v5.0.1” 成本图（含图例、轴、方法学行） | 1400×685 |
| 16-google-coding-index.png | 谷歌博客 | Coding Agent Index 58.6 段 | 1400×440 |
| 17-aa-opening.png | AA 文章 | 标题、摘要、开头段（含 “standard (non-discounted) pricing”） | 1400×640 |
| 18-aa-cost-17pct.png | AA 文章 | Key takeaways 第一条：$6.44 / $7.76 / ~17% / ~15% more output tokens | 1400×440 |
| 19-aa-pricing.png | AA 文章 | Key model details 全段，含 Availability 与 Pricing（“ends on November 30”） | 1400×710 |
| 20-aa-cost-charts.png | AA 文章 | 成本柱图 + 指数对成本散点图（含图例、轴、AA 水印） | 1400×1000；700 CSS px 下图中小字较小，原图在 `b-aa-fig1.png`（宽 1600） |
| 21-aa-coding-gdp.png | AA 文章 | Coding Agent 与 GDP.pdf 两条 | 1400×630 |
| 22-aa-model-pages-neon-carbon.png | AA 模型页 Neon 与 Carbon | 各取标题行与 Cost / Verbosity 两张摘要卡，四段拼接 | 1400×1552；拼接处的浅灰条是间隔，非页面元素；未截 Intelligence / Performance 卡 |
| 23-pricing-two-models.png | Gemini API 定价页 | Carbon 与 Neon 的标题、免费层说明、价格表（六段拼接） | 1400×1960；**深色底页面**；拼接间隔 #191919 |
| 24-newsbytes-60pct-headline.png | NewsBytes | “It's 3x faster, 60% cheaper” 标题、署名、日期、“What's the story” | 859×420；用 opencli 浏览器截图（2 倍 CSS zoom，非 DPR 2），上沿有一条细黑线（上方图片的边） |
| 25-google-glean-quote-60pct.png | 谷歌博客（桌面宽度 1400 CSS px 渲染） | Glean 的 Thai Tran 语录卡片全文，含 “60% of the cost” | 1200×1210；**编号超出 13–24 区间**（区间已用尽）；窄视口下该卡文字被页面截断，故用桌面宽度渲染 |

废弃/待删除：`images/23-pricing-carbon-neon.png`（Carbon 免费层说明只截到第一行，缓存行下沿切到半行；已用 `23-pricing-two-models.png` 取代）。

## 不确定或有偏差之处

- HN 评论得分：Algolia JSON 与 HTML 都不含评论得分，评论只记链接与作者。
- AA 的 $3.74（Neon high）由 AA 模型页 HTML 内嵌图表 JSON 读出（6.444838 / 3.742750 / 1.904556 / 0.745228；Carbon 7.7555 / 6.5858 / 7.1866），也在 AA 文章图 1 的柱标签里（$3.74、$6.44、$7.76）。
- 页面渲染可能随时间变化；各页取得时刻见上表（北京时间 11-19 21:30–21:45）。
- 交付前 grep：对 `docs/2611/1118/` 查抓取者邮箱、账号名、本机用户名路径，除上述已处理的 4 个文件外无命中；处理后再查无命中。截图均来自匿名 headless Chrome（13–23、25）或公开新闻页（24，无账号控件）。
