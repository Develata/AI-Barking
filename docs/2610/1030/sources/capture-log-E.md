# 1030 期取证 E 组：抓取日志

执行方：Haiku 5.5（claude-haiku-5-5）。时刻均为北京时间（UTC+8）。工具：`barking fetch`（网页与 API JSON）、`barking heat`（HN 与 Reddit）。E 组不截图，没有用 `barking shot`。`barking fetch` 原始日志在 `sources/e-fetch-log.jsonl`，热度在 `sources/e-heat.json`。

## 抓取

| 北京时间 | 实际文件名（前缀 `e-e-`） | URL | 工具 | 结果 | 字节 | 说明 |
|---|---|---|---|---|---|---|
| 16:48:46 | `e-e-suncatcher.*` | https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-satellite-response/ | barking fetch | ok（200） | 380,059 | 响应头 Date 08:48:46 GMT；正文已核 |
| 16:48:46 | `e-e-ycgames.*` | https://labs.ycombinator.com/projects/games/index.html | barking fetch | ok（200） | 169,078 | JS 渲染内容已在 `.txt`（111 KB）中，未用 `barking shot render` |
| 16:48:46 | `e-e-ycharness.*` | https://github.com/yc-software/minecraft-agent-harness | barking fetch | ok（200） | 291,128 | 仓库页：MIT，Stars 71，1 commit |
| 16:48:46 | `e-e-nesformer.*` | https://github.com/dbreunig/nesformer | barking fetch | ok（200） | 320,192 | 仓库页：197 stars；README 已核；仓库页无发布日期 |
| 16:49:13 | `e-e-404-proton-reinterview.*` | https://www.404media.co/proton-to-re-interview-staff-about-ai-to-see-who-gets-to-keep-their-job/ | barking fetch | ok（200） | 51,488 | 正文在付费墙处截断（"This post is for paid members only"），只存可见部分 |
| 16:49:13 | `e-e-404-proton-cancel.*` | https://www.404media.co/protons-ceo-cancels-ai-re-interviews/ | barking fetch | ok（200） | 59,104 | 全文可见 |
| 16:49:13 | `e-e-404-proton-reckoning.*` | https://www.404media.co/manifestos-fear-and-loser-thinking-inside-protons-ai-reckoning/ | barking fetch | ok（200） | 37,916 | 正文在付费墙处截断，可见部分到 "Subscribe" 之前 |
| 16:52:12 | `e-e-x-pichai.json` | https://api.fxtwitter.com/status/2116214866444943853 | barking fetch | ok（200） | 5,846 | 官方账号 sundarpichai 的公开帖 JSON（fxtwitter 不返回线程字段） |
| 16:52:13 | `e-e-x-ycombinator.json` | https://api.fxtwitter.com/status/2115904914526548472 | barking fetch | ok（200） | 6,562 | 官方账号 ycombinator 的公开帖 JSON；JSON 只有本帖字段，无线程或会话字段 |
| 16:53:26 | `e-e-suncatcher-launch.*` | https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-prototype/ | barking fetch | ok（200） | 379,579 | Google 10-01 发射博文；无 kW、分钟、芯片数 |
| 16:53:38 | `e-e-suncatcher-facts.*` | https://blog.google/innovation-and-ai/models-and-research/google-research/google-project-suncatcher-facts/ | barking fetch | ok（200） | 361,804 | Google 官方事实页；“Future designs ... dozens of TPU chips” 所在 |
| 16:59:07 | `e-e-tnw-proton.*` | https://thenextweb.com/news/proton-ai-re-interviews-staff-andy-yen-404-media | barking fetch | ok（200） | 185,574 | TNW 转述 404 Media；页面标注 "Published October 30, 2026 - 4:50 pm UTC"，Exa 元数据给 11:54Z，两者不一致，未取舍 |
| 16:59:07 | `e-e-verge-proton.*` | https://www.theverge.com/news/1023707/proton-now-wont-re-interview-staff-actually | barking fetch | ok（200） | 334,720 | The Verge 转述 404 Media 的取消；页面 "Posted Oct 30, 2026 at 7:55 PM UTC" |

清单文件：`sources/e-fetch-list1.tsv`（batch 1）、`e-fetch-list2.tsv`（404 三篇）、`e-fetch-list3.tsv`（fxtwitter 两条）、`e-fetch-list4.tsv`（发射博文）、`e-fetch-list5.tsv`（事实页）、`e-fetch-list6.tsv`（TNW 与 Verge）。

未抓取但在检索中见到、已在 evidence-E.md 注明的：
- TechCrunch 10-01、NPR（扫描称含 1 kW、20 分钟、4 颗 TPU）：不在派工范围，未取。
- Herald（biz.heraldcorp.com/article/10921351）：仅用于勘误（每队 50 人），未存档。
- 搜索摘要中 YC 官方 X 线程的后续推文（“50 vs 50”）：只见于镜像站摘录，未存档、未核原帖。镜像站不作依据。
- madrobot.blog、aiweekly.co、theterminal.space、cho.sh 等转述：仅作线索，未存档，不作依据。

## 热度

| 北京时间 | 命令 | 结果 | 说明 |
|---|---|---|---|
| 16:49:46 | `barking heat docs/2610/1030 e --item 50296486 --item 50306442 --item 50295112 --hn "Proton AI re-interview" --hn "nesformer" --hn "Suncatcher"`（不含 Reddit） | ok：写出 `e-heat.json` 与 `e-hn-*.json` | HN 数据 |
| 16:52 | 同一命令加 `--reddit` 三个查询（两次尝试） | 两次都因 `barking` 拒绝覆盖已有 `e-heat.json` 或 `e-hn-*.json` 而中止，未写入 | 工具不支持覆盖；见下一行的处理 |
| 16:52:39 | 同上（先把已有的 `e-heat.json` 与 `e-hn-*.json` 移到 scratchpad，再重跑） | HN 3 组、条目 3 个 HN 原帖 JSON 写入；Reddit 3 个查询失败 | 仓库内 `e-heat.json` 与 `e-hn-*.json` 是这次重跑的产物，内容与 16:49 一致 |

Reddit 失败详情：3 个查询（"Proton AI re-interview"、"nesformer"、"Minecraft Mindustry agent swarms"）都返回 `old.reddit.com/login/?reason=lor2` 的登录页（HTML，约 339 KB），不是 JSON。未重试、未改用其他域名。

被移到 scratchpad 的旧文件（仅为允许重跑，未删除、未覆盖仓库外的任何东西）：
- `<本会话 scratchpad>\e-heat-prev.json`
- `<本会话 scratchpad>\heat-prev\e-hn-50296486.json`、`e-hn-50306442.json`、`e-hn-50295112.json`

## 失败与未取

- 404 Media 两篇的付费墙后正文：未绕过，记为“截断”，不算失败。
- Reddit 热度：3 个查询失败（见上）。
- 没有运行 opencli 或 headless Chrome：本组的页面都能用 `barking fetch` 拿到正文。

## 隐私与留存检查

- 响应头只保留 Content-Type、Date、Last-Modified 等，其余 `.headers` 文件头部已注明"其余响应头未存档"（由 `barking fetch` 处理）。
- fxtwitter JSON 与 HN JSON 中出现的是公开帖子作者与公开评论作者（官方账号、HN 用户名），不是抓取者的头像、昵称或 handle。
- 未存档 Cookie、登录态或本机浏览器数据。
- 未写入任何真实身份信息。
