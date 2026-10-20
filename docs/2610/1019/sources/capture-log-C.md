# 1019 期 C 组抓取日志

时区：北京时间（UTC+8）；本机为美东（EDT），北京时间 = 本机时间 + 12 小时。执行模型 Sonnet 5.5（claude-sonnet-5-5）。2026-10-20，19:12–19:50。未登录任何站点、未接受 Cookie、未使用镜像或代理域名、未读取浏览器 Cookie、未使用 firecrawl、未绕过 Cloudflare 或付费墙。

## 一、网页存档（barking fetch，清单 c-list1…c-list7.tsv，日志 c-fetch-log.jsonl）

| 清单 | 开始 | 条目与结果 |
|---|---|---|
| c-list1.tsv | 19:12:59 | ok：blog、home、docs-models、openai-pricing（企业套餐页，不是 API 价目，不引用）、anthropic-pricing、anthropic-models、aa-luna、aa-haiku、or-mercury、or-luna、or-haiku；or-mercury-api 取到 JSON（存为 .json）。failed（404）：pricing（https://www.inceptionlabs.ai/pricing）、docs-pricing（docs `/get-started/pricing`）、aa-mercury3（AA 无 Mercury 3 页面，404）。 |
| c-list2.tsv | 19:14:37 | 全 ok：docs-models-page、site-models、docs-start、docs-llms、research |
| c-list3.tsv | 19:14:57 | 全 ok：docs-reasoning、docs-streaming、docs-faq、docs-ratelimits |
| c-list4.tsv | 19:15:29 | 全 ok：oai-api-pricing、oai-luna-model、oai-models |
| c-list5.tsv | 19:15:58 | ok：aa-inception、aa-luna-medium、aa-haiku-medium、aa-mercury25、aa-leaderboard；failed（404）：aa-mercury3-b（`/models/mercury-3-medium`） |
| c-list6.tsv | 19:17:41 | ok：m-unite、m-deepview、m-shattered、m-digg、m-orcarouter、m-runtimewire、baseten、venice；blocked（403，Cloudflare 质询，未绕过）：m-marktechpost、m-neuron、m-fourweek |
| c-list7.tsv | 19:19:51 | ok：pr-morningstar、pr-yahoo、m-ie、m-startupfortune、m-orca-nobench；blocked（403）：m-linuxdo（改用 opencli，见下） |

注：c-list1 的 openai-pricing 抓到的是企业版套餐页，已改取 developers.openai.com 的 API 价目与模型页（c-list4）。AA 页面大（1–4 MB html，不入库）；所有 .html/.pdf/图片只存本地。

## 二、其他请求

| 时刻 | URL / 命令 | 工具 | 结果 |
|---|---|---|---|
| 19:13 | 博客内 9 张 framerusercontent 图（含对比表与规模图原图）逐张下载 | curl --max-time 60 | 全部 200，存为 c-img-<id>.png/.webp；对比表 YO5OXn25…（1364×1678）与规模图 uA54aGA7…（1296×1298）即官方原图 |
| 19:17 | https://api.fxtwitter.com/status/2112222911218291051、2112236719068369299、2112256049311072339、2112226539018629459、2112225779782398131 | curl | 200，存为 c-fx-<id>.raw.json（公开帖 JSON，无抓取者信息；含作者公开资料与媒体 URL，未改写） |
| 19:17 | https://api.fxtwitter.com/status/2112258558599512078（AA 帖） | curl | 404 NOT_FOUND；同 id 的 `ArtificialAnlys/status/…` 路径也 404 |
| 19:18 | https://cdn.syndication.twimg.com/tweet-result?id=2112258558599512078&token=a | curl | 返回 TweetTombstone “This Post was deleted by the Post author.”（未存档，仅记结论） |
| 19:19 | https://api.vxtwitter.com/ArtificialAnlys/status/2112258558599512078 | curl | 200（缓存时刻 2026-10-19 19:33:53 UTC），存为 c-vx-2112258558599512078.raw.json；含 AA 原文与被引 Inception 帖，无抓取者信息 |
| 19:20 | https://pbs.twimg.com/media/HVArbGnaYAApnzP.png（Inception 发布帖配图） | curl | 200，c-img-x-inception-post-media.png（1200×1019），即 images/33 |
| 19:20:02 | `barking heat docs/2610/1019 c --hn "Mercury 3" --hn "Inception Mercury" --hn "diffusion LLM Inception" --reddit "Mercury 3 Inception diffusion" --reddit "Inception Labs Mercury" --since 1792300000` | barking | HN：“Mercury 3” 命中 1 条（50155245，249 分 / 89 评）；另两个关键词 0 条。Reddit 两次请求失败：响应被重定向到 old.reddit.com 登录页（非 JSON，已记入 c-heat.json 的 errors） |
| 19:20:34 | `barking heat docs/2610/1019 c-hn --item 50155245` | barking | 成功：c-hn-heat.json、c-hn-hn-50155245.json（含嵌套评论；c-heat.json 已被上一条占用，故换前缀 c-hn） |
| 19:3x | `opencli reddit search "Mercury 3 Inception diffusion"` | opencli（匿名搜索接口，无登录动作） | 成功：c-reddit-search-opencli.yaml（r/singularity 107 分 / 20 评；r/aicuriosity 41 分 / 7 评） |
| 19:3x | `opencli browser c-shot open https://linux.do/t/topic/3047166` 并 eval 读首帖 `.cooked` 文本 | opencli（OPENCLI_PROFILE=5df8dbx7；公开页只读） | 成功：c-m-linuxdo.txt（只含帖子文本与标题，无账号信息）；curl 此页 403 |
| 19:2x | WebSearch × 6（中英文关键词） | WebSearch | 找到 X 官方帖、AA 提醒转述、Business Wire、多家英文媒体、LINUX DO/locdd 中文社区帖；**未找到主流中文科技媒体报道**，**未找到 TinyFish 自己的 Mind2Web 公开数据** |
| 19:3x | 抽取 c-anthropic-pricing.html 中表格 | python -I（标准库，只读存档） | c-anthropic-pricing-table.txt |

## 三、截图与裁切（均 2 倍像素除另注；宽度 ≤1400 px；只裁切不改像素；记录在 c-shots-log.jsonl）

opencli 浏览器截图（OPENCLI_PROFILE=5df8dbx7，session c-shot，window background，`zoom=2`，`--width 1400`）用于 Inception 博客与 docs 页（无需登录的页面）；Anthropic、OpenAI、OpenRouter、HN 页用 `barking shot render`（headless Chrome，匿名）——因为 opencli 在 Anthropic 页面的截图里出现了本机已登录的 Console 组织名（左下角侧栏），该截图弃用（仅在系统临时目录，未入库、未使用），改走 headless。

| 文件 | 来源 | 内容 | 目视确认 |
|---|---|---|---|
| images/25-blog-intro.png（1380×829） | opencli 截图，两段竖向拼接（页面标题与作者行 + 开头两段，段间 24 px，底色 #faf7f1 取自页面浅色底） | 博客标题、“Inception Team”、开头两段（outperforms … 4–8x faster；40x larger）；裁掉导航、Cookie 条、浏览器翻译图标、滚动条 | 是 |
| images/26-benchmark-table.png（1364×1678） | 官方原图 c-img-YO5OXn25….png，字节拷贝（sha256 37d41201…） | 对比表全图（标题、全部 12 项与速度/价格行、三列标题、无脚注——原图本身没有脚注） | 是，逐行已对照抄入 evidence-C.md 表 C2 |
| images/27-blog-glance-price.png（1380×575） | opencli 截图单段 | “Mercury 3 at a glance” 全段（Quality/Speed/Context/Price 与 50% off、缓存折扣同框） | 是 |
| images/28-quality-vs-size.png（1296×1298） | 官方原图 c-img-uA54aGA7….png，字节拷贝（sha256 8727a486…） | 规模图（含标题、副标题、轴、各点标注与脚注） | 是 |
| images/29-docs-pricing-table.png（885×528，**1 倍像素**） | opencli 截图，docs 模型页表格（页面布局需 ≥1350 CSS px 才放得下价格列，2 倍像素会超过 1400 px，故取 1 倍） | 价格列原价 → 首发价（Mercury 3 / Voice / Decide 与 2.5 / 2 / Edit 2）；右侧 Context Window 列被页面自身的横向滚动容器截断（与存档文本核对：Mercury 3 “Chat: 1M”） | 是 |
| images/30-anthropic-haiku-pricing.png（1320×1220） | `barking shot render`（700 CSS px，DPR 2）+ cropbox | Anthropic 价目表 Fable/Opus/Sonnet/Haiku 5.5 行，Haiku 5.5 的 ≤10 万与 >10 万两档；“Hits and refreshes” 缓存命中列被页面横向裁掉（数字取自存档文本，见 c-anthropic-pricing-table.txt） | 是 |
| images/31-openai-luna-pricing.png（1300×555） | `barking shot render`（1000 CSS px，DPR 1.4）+ cropbox | OpenAI API 价目 Standard，Short/Long context，gpt-6-luna 行（$0.10 / $0.01 / $0.125 / $0.50；$0.20 / $0.02 / $0.25 / $0.75）；272K 阈值写在模型页文字里，图中无 | 是 |
| images/32-openrouter-performance.png（1400×2690） | `barking shot render`（700 CSS px，DPR 2）+ cropbox | OpenRouter Mercury 3 的 Performance 区（Throughput 180 tok/s、Latency 0.78 s；图表 P50–P99；Reasoning effort All、1 week） | 是；页面右上角为 “Sign Up”，无账号信息 |
| images/33-intelligence-vs-speed.png（1200×1019） | Inception 官方 X 发布帖的配图原图（c-img-x-inception-post-media.png，字节拷贝，sha256 e4833f5f…；1 倍像素） | “Intelligence vs. speed” 散点图（轴 47.5–51.5）、脚注 “Intelligence Index is an unweighted mean across the benchmarks.” | 是；图内无平台名 |

失败/未做的截图：
- Inception X 发布帖的卡片截图：`barking shot render` 渲染 `platform.twitter.com/embed/Tweet.html?id=2112236719068369299`（19:30）得到整页空白，未重试第二次；以 fxtwitter 公开 JSON 存档（c-fx-…raw.json）代替。X 帖卡片不进图。
- HN 页面渲染（中间图 hn-render 在系统临时目录）：页面按其他排序展示，联合创始人的评论不在首屏，未出图；评论以 `barking heat` 的 JSON 存档为准。
- AA 的 Mercury 3 缺页，无截图；AA 的提醒帖在 syndication 接口显示已删，无法截卡片。
- 弃用的试验文件：images/25-blog-intro.png 首次拼接时间隙底色被自动取成页面顶部的蓝绿色横幅色，已用 `rm`（精确路径）删除后以 `--bg "#faf7f1"` 重做（c-shots-log.jsonl 里保留两条记录，19:22:36 与 19:23:02）；系统临时目录与 scratchpad 中的中间图未入库。

## 四、自查

- 交付前 grep `docs/2610/1019/` 的 c- 前缀文件与 images：无本机账号名、邮箱、头像链接；fxtwitter/vxtwitter JSON 内含的是被取帖作者的公开资料，不是抓取者的。
- 响应头文件已由 `barking fetch` 去掉暴露抓取地点或标识的字段。
- 体积：单文件超过 1 MB 的均为 .html 原件（c-aa-*.html、c-anthropic-pricing.html、c-openai-pricing.html、c-oai-*.html 等），按规则“只存本地、不入库”；入库的文本存档（.txt/.json/.md/.jsonl/.tsv/.yaml）均 <1 MB（最大的文本存档 c-aa-leaderboard.txt 约 45 KB）；无单个入库文本超过 1 MB。
