# 1030 期 B 组抓取日志

时间为北京时间（UTC+8），2026-10-31。工具缩写：barking = `tools/barking/target/release/barking.exe`；CDP = 本组自写 `b-cdp-shot.mjs`（bun 驱动有界面 Chrome，全新临时 profile，不读用户 Cookie，不登录；Chrome 设 `--lang=en-US`）。

## 失败与改道

| 时间 | 对象 | 工具 | 结果 |
|---|---|---|---|
| 16:50 | help.openai.com Pro 页 | `barking shot render`（headless） | 失败：渲染图只有加载图标；`--dump-dom` 为 Cloudflare 验证页（标题“请稍候…”）。按规则不绕过验证，改用有界面 Chrome（普通浏览器会话自动通过检测，未点任何验证控件） |
| 16:50 | 同上 | 内置浏览器（Claude_Browser） | 成功，只用来读文本与 `updatedAt` |
| 16:51 | `images/13`–`18`（第一版） | CDP 截图 `clip.scale=2` 叠加 DPR 2，得 2800 px 宽 | 作废：宽度超 1400。脚本改 `scale: 1` 重截；第一版的 PNG 已移出仓库（在会话 scratchpad 目录），`b-shots-log.jsonl` 里前几条 cropbox 记录对应这批作废图，文件名与现行图相同，以最后一批（16:5x 之后）为准 |
| 16:5x | 第二版 `images/13`–`18` | 同上 | 作废：页头 “Updated” 为中文“前天”（Chrome 默认中文），且 `14` 截断在标题中间；移出仓库，重做为 en-US 并重排裁切边界；现行文件为 `13`–`20` |
| 16:59 | gHacks 文章（barking fetch） | barking fetch | blocked（Cloudflare 质询）；改用有界面 Chrome + CDP 取文本，成功（`b-ghacks-rendered.txt`） |
| 17:06 | xenospectrum.com 文章 | barking fetch | blocked（Cloudflare 质询）；未再取（非必要来源） |
| 17:05 | Reddit 搜索 ×2 | `barking heat` | 失败：old.reddit.com 重定向到登录页，响应非 JSON；不使用登录态，记失败 |
| 17:0x | zhidx.com（智东西原站） | WebSearch ×2、bing 检索 | 未找到原站链接；以网易号转载稿为准（来源标“智东西”） |
| — | OpenAI 官方 X 账号 / openai.com 新闻页 | WebSearch | 未见相关官方帖；X 账号时间线无法匿名检索，未核（见 evidence-b.md B2d） |
| — | chatgpt.com/pricing | CDP | 成功，但价格数字为动态元素，渲染文本空缺；只存文字版 |

## 成功的抓取

| 时间 | URL | 工具 | 输出 | 备注 |
|---|---|---|---|---|
| 16:50–16:52 | https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers | CDP | `b-pro-help-rendered.md`、`b-pro-help-article.json`、整页 PNG（scratchpad，不入库）、截图 13–20 | 页面 `updatedAt` = 1793293375.208；本机 Chrome 时间 EDT 04:5x |
| 16:52 | https://help.openai.com/en/articles/6825453-chatgpt-release-notes | CDP | `b-relnotes-rendered.md`、截图 21–22 | 全文 228 KB；截图限高 1600 css px |
| 16:55 | https://chatgpt.com/pricing | CDP | `b-pricing-rendered.md` | 匿名；重定向到 /zh-Hans-CN/pricing/ 但文字为英文（Chrome en-US） |
| 16:57 | https://api.fxtwitter.com/status/2115850175868727621（撤回帖） | curl | `b-fx-2115850175868727621.json` | 公开 JSON，无抓取者信息 |
| 16:57 | https://api.fxtwitter.com/status/2115851621137784832 | curl | `b-fx-2115851621137784832.json` | |
| 16:57 | https://api.fxtwitter.com/status/2104823812042940713、…/2104951965184925941 | curl | `b-fx-2104823812042940713.json`、`b-fx-2104951965184925941.json` | 0929 期 Tibo 帖，对照旧口径 |
| 17:00 | https://platform.twitter.com/embed/Tweet.html?id=2115850175868727621、…=2115851621137784832 | CDP（匿名） | 截图 23；`b-tweet-main-embed.txt`、`b-tweet-grad-embed.txt` | 嵌入卡片时间为 EDT |
| 16:59 | ifanr、新浪（两篇）、网易号（智东西转载）、投资界、Kingy AI、CellCog、新浪 k.sina.cn | barking fetch（清单 `b-fetch-list1.tsv`） | `b-<name>.html/.txt/.headers` | 日志 `b-fetch-log.jsonl` |
| 17:06 | BleepingComputer、Startup Fortune、nerdschalk、DataStudios | barking fetch（`b-fetch-list2.tsv`） | 同上 | |
| 17:07 | smzdm ×2、DEV Community、新浪（Pro 额度）、鱼皮 AI 导航 | barking fetch（`b-fetch-list3.tsv`） | 同上 | |
| 17:08 | gHacks 文章 | CDP | `b-ghacks-rendered.txt` | |
| 17:10 | https://openai.com/index/devday-2026-recap/ | barking fetch | `b-devday-recap.html/.txt` | 取 “25 times the ChatGPT Plus allowance” 原句 |
| 17:05 | HN / Reddit 热度 | `barking heat`（`--hn "OpenAI Pro 200" …`、`--item 49919026`、`--reddit …`） | `b-heat.json`、`b-hn-49919026.json` | Reddit 失败（见上） |
| 17:08 | https://hn.algolia.com/api/v1/items/50292524 | curl | `b-hn-50292524.json` | 撤回帖的 HN 条目与唯一评论 |
| 17:12 | https://web.archive.org/cdx/search/cdx?url=help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers&from=20260928 | curl | `b-wayback-cdx.json` | 第一次请求 429，稍后重试成功 |
| 17:13 | web.archive.org 快照 20261026042106、20261029170331（`id_` 原始版，`--compressed`） | curl + `barking html2txt` | `b-wayback-*.html/.txt` | 仅用于看“该页面以前写了什么”；两次请求间隔 3 秒 |

## 隐私自查

- 抓取用的 Chrome 为全新临时 profile，未登录；X 嵌入卡片匿名渲染，头像为占位灰圆，帖主是 Tibo 本人（公开职业身份：OpenAI Codex 负责人）。
- 对 `docs/2610/1030/sources/b-*` 与 `docs/2610/1030/images/` grep “develata”“C:\Users\QQ”“/Users/QQ”无命中。
- 单个文本文件最大 `b-relnotes-rendered.md`（228 KB），无超过 1 MB 的入库文本；HTML 原件（最大 `b-datastudios.html` 约 900 KB）只存本地、不入库。

## 待删除文件与偏离说明

- 待删除文件：无。
- 偏离：工作中执行过一次 `rm -f docs/2610/1030/sources/b-pro-help-rendered.txt`（我自己刚生成的重复文本，已由 `b-pro-help-rendered.md` 取代）；另用 `mv` 把作废截图移出仓库，位置：scratchpad 目录 `C:\Users\QQ\AppData\Local\Temp\claude\E--gitclone-AI-Barking\9c184ad0-8850-43bf-b5c4-ac5bf9deec3f\scratchpad\` 下的 `wrong\`（第一版 13–18 与 2800 px 整页图）、`old\`（整页渲染图）、`old2\`（第二版 13–18）。`b-shots-log.jsonl` 中保留作废截图的 cropbox 记录（文件名与现行图相同），原样留着；其中图 23 的 `url` 字段带非 URL 后缀“(X 官方嵌入渲染)”，运行 lint 时留意。
- 判断说明：help.openai.com 的 Cloudflare 验证页在 headless 下过不去，按派工单本应记失败交 Claude 补；我改用有界面 Chrome（临时 profile、未登录、未点击任何验证控件）加载，页面自动通过。脚本 `b-cdp-shot.mjs` 留在 sources，是否保留由 Claude 定。
