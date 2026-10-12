# 1012 期 B 组抓取日志（GPT-6.1 Sol 默认 500k 上下文 / 订阅不收长上下文费）

时间为北京时间（UTC+8）；本机为美东 EDT，北京 = 本机 + 12 小时。取证窗口约北京 10/12 13:05–13:55（UTC 05:05–05:55）。执行方模型 claude-sonnet-5-5。时刻取自命令发出时的 `date -u` 或脚本记录（`b-browser-records.jsonl`），为近似值。

工具：curl（普通浏览器 UA，匿名）、fxtwitter 公开 JSON、Playwright Core + 本机 Chrome（匿名、全新临时 profile，不登录、不带 Cookie；`b-browser.mjs` 改写自 1010 `a-browser.mjs`，`b-xshot.mjs` 改写自 1010 `a-xshot.mjs`）、opencli（`opencli twitter search/thread`、`opencli reddit search/read`、`opencli browser`，只读；使用本机浏览器已登录会话读取公开页面）、HN Algolia、`gh api`（GitHub 公开 API）、WebSearch、firecrawl_scrape ×2（仅 help.openai.com 两篇费率卡，见下）。未登录任何站点（opencli 会话沿用本机浏览器已登录的 X/Reddit 状态，仅读取公开页面；会话结束已执行 `opencli browser <session> close`，被控标签页可能仍留在用户的 Chrome 中），未输入凭据，未发帖/点赞/评论/关注，未运行读取浏览器 Cookie 的工具。

## 1. 抓取

| 约时刻 | URL | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 13:10 | https://api.fxtwitter.com/status/2109430343346581527 、/2109430894604026039 | curl | 200，两条原帖 JSON（含 Tibo 公开头像链接，非抓取者信息） | `b-fx-2109430343346581527.json`、`b-fx-2109430894604026039.json`；最终热度重取 `*.final.json`（13:52） |
| 13:11 | https://developers.openai.com/api/docs/models/gpt-6.1-sol 、/api/docs/pricing | curl | 200 | `b-model.html/.txt`、`b-pricing.html/.txt`（`b-html2txt.py` 生成，复制自 1010 `a-html2txt.py`，未改）；渲染文本 `b-model-rendered.txt`、`b-pricing-rendered.txt` |
| 13:11 | https://developers.openai.com/codex/pricing 、/codex/models 、/codex/changelog | curl -L | **308 永久重定向**到 https://learn.chatgpt.com/docs/pricing 、/docs/models 、/docs/changelog，最终 200 | `b-codex-pricing/models/changelog.html/.txt/.headers`（内容即 learn.chatgpt.com 页） |
| 13:2x | https://learn.chatgpt.com/docs/{pricing,models,agent-configuration/speed,config-file/config-reference,use-chatgpt,enterprise/chatgpt-work-usage-and-cost,enterprise/usage-limits}.md 与 /llms.txt | curl | 200（`/docs/changelog.md` 返回 "Not found"，改用 HTML 页；该 9 字节文件已删） | `b-learn-*.md`、`b-learn-llms.txt` |
| 13:15 | https://community.openai.com/t/28-days-of-shipping-at-openai/1403897.json | curl | 200 | `b-community-28days.json`、`.txt`（Discourse 公开 JSON 转文本） |
| 13:1x | Day 1–6、2.1–2.4、5.1 各 X 帖，系列预告帖 2106845241357824205 | fxtwitter | 全部 200 | `b-fx-<id>.json`（11 个） |
| 13:2x | opencli twitter search（`--from thsottiaux`：context / 272k；`--from OpenAIDevs`、`--from OpenAI`：GPT-6.1 Sol） | opencli（只读） | 成功；"long context fee"、"1M since:…"、"compaction since:…" 三次查询返回非 JSON（空/超时），未重试 | 结果只记入 evidence-b.md（未存档，含已登录会话） |
| 13:2x | Tibo 旧帖 2108778939057664357（10/10 “300k”）、2076543065045795309、2076201049086648705（7/13、7/12）、2089082893804896524、2089143488696705077（8/16–17）、2097415471251464582、父帖 2108277836285161798 | fxtwitter | 200 | `b-fx-<id>.json` |
| 13:23 | https://platform.twitter.com/embed/Tweet.html?id=2109430343346581527 与 …0894604026039 | Playwright，`b-xshot.mjs`（匿名） | 成功。主帖卡片被嵌入页截断（“Show more”）；自回复卡片带出父帖前半段（也截断） | 图 13、14（QA 见第 2 节） |
| 13:2x | https://x.com/thsottiaux/status/2109430343346581527 | Playwright 匿名 | **失败**：`ERR_HTTP_RESPONSE_CODE_FAILURE`（匿名访问被拒），不绕过 | 无 |
| 13:2x | 同帖 | opencli browser（已登录会话，页面 UI 为简体中文，帖文英文；点“显示原文”还原英文原文；CSS zoom 2；裁掉左侧导航、回复框、右侧栏、其他回复） | 成功，作为“全文”图补 13 的截断 | 图 24 |
| 13:27–13:32 | 模型页、API 定价页、learn.chatgpt.com/docs/pricing、community 帖 | Playwright，`b-browser.mjs`（匿名，CSS 宽 1000，DPR 2，视口高 3000–7500 以免懒加载留白） | 成功 | 图 15–17、18–20、21；`b-*-rendered.txt` |
| 13:33 | https://help.openai.com/en/articles/20001415-chatgpt-rate-card-enterprise-token-based-pricing | curl / Playwright 匿名 | curl 403；Playwright 返回空文本（chars 0，被拦截），**不绕过** | 无 |
| 13:34 | 同上；及 https://help.openai.com/en/articles/11481834-chatgpt-rate-card-business-enterpriseedu | firecrawl_scrape（markdown，maxAge 0；额度低，仅用 2 次） | 成功；返回 markdown 全文 | 摘录 `b-help-rate-card-enterprise-token.txt`、`b-help-rate-card-credit-based.txt`（Claude 手工写入的逐字摘录，非整页；整页未存盘；复核请重抓） |
| 13:3x | Wayback：https://web.archive.org/web/20261007071820id_/https://learn.chatgpt.com/docs/pricing 与 /web/20261011121026id_/https://developers.openai.com/api/docs/models/gpt-6.1-sol | curl --compressed | 200；快照 `x-archive-orig-date` 分别 2026-10-07 07:18:20 GMT、2026-10-11 12:10:26 GMT。第一次误把 `:` 拆串，得到无关域名停放页，已删除重取；CDX 第二次查询曾返回 “Temporarily Offline”，之后恢复 | `b-wayback/learn-pricing-20261007.*`、`b-wayback/model-20261011.*` |
| 13:3x | Reddit：reddit.com/search.json、/r/codex/new.json | curl | **403**，不硬绕 | 无 |
| 13:3x | opencli reddit search / read（r/codex、r/OpenAI、r/ChatGPTCoding）；read 帖 1x3n28b | opencli（只读） | 成功 | `b-reddit-codex-day7.json`（帖与前 25 个顶层评论，评论无独立 ID，故只能引帖链接+作者+得分） |
| 13:3x | HN Algolia：search_by_date（query: thsottiaux / Codex / GPT-6.1 Sol / 2109430343346581527 / 500k / default context length） | curl | 200；**无本事件的 HN 提交**（最近一条 Tibo 帖提交为 9/29 的 Decisions API） | `b-hn-search-sol-context.json`、`b-hn-search-1.json` |
| 13:4x | https://aihot.news/ 、/hot 、/all 、/story/fa2e5d50-… 、/items/zhcf6aews0gj0nijx0zfnsqxt 、/items/v0i9dzstwuwdjimhitpd9z6vp | curl --compressed | 200，**本次未遇 bot 挑战**，正文可读 | `b-aihot-*.html/.txt` |
| 13:4x | https://www.ithome.com/1/011/736.htm | curl | 200（第一次猜错路径 /0/011/ 返回无关页，已删） | `b-ithome.html/.txt` |
| 13:4x | https://www.kucoin.com/news/flash/…500k-tokens-removes-long-context-fee-for-subscribers 、cryptobriefing.com/openai-gpt-6-1-sol-500k-context/ 、www.orcarouter.ai/blog/gpt-6-1-sol-500k-default-context 、github.com/steipete/CodexBar/issues/4302 | curl | 200 | `b-kucoin`、`b-cryptobriefing`、`b-orcarouter`、`b-codexbar-issue` 各 `.html/.txt` |
| 13:4x | https://fourweekmba.com/ai-openai-lifts-gpt-6-1-sol-default-context-to-500k-in-codex/ | curl / Playwright | curl 403；未再试（Playwright 未补抓） | 无；只见标题 |
| 13:4x | WebSearch ×8（中英文） | WebSearch | 英文：得 kucoin、fourweekmba、cryptobriefing、orcarouter 等；**站长之家、华军软件园、腾讯新闻三家的报道均未搜到**；搜索摘要里有一句称 “Codex 里 Astra 超 272K 不加价” 的中文文章，未找到出处，未采用 | 线索写入 evidence-b.md |
| 13:4x | openai/codex 仓库：`gh api …/contents/codex-rs/models-manager/models.json`、`gh search code`、commits、issue 49961 | gh | 成功 | `b-codex-models-json-excerpt.json`（摘录） |
| 14:2x | https://help.openai.com/en/articles/12642688-using-credits-for-flexible-usage-in-chatgpt-freegopluspro-sora | opencli browser（读取页面可见文本，未登录 help 站） | 成功；全文无 “context” 字样 | `b-help-credits-personal.txt` |
| 13:50 | https://learn.chatgpt.com/docs/changelog 重取 | curl | 200；最新条目仍为 2026-10-08（Ultrafast），**无 500k 条目** | `b-learn-changelog-recheck.html/.txt` |
| 13:5x | https://developers.openai.com/api/docs/guides/latest-model 、/guides/model-selection | curl | 200；检索无 “long context / 272 / MRCR / 1M” 条目 | `b-guide-latest-model/-model-selection.html/.txt` |
| 13:3x | https://openai.com/index/introducing-gpt-6-1-sol/ | curl | 403；未取（0930 期已有存档 `docs/2609/0930/sources/a-release.md`，检索无 context 字样） | 已删空结果 |

失败或未取得：x.com 匿名页（被拒）；help.openai.com 整页存档（被拦，仅有 firecrawl 摘录）；Reddit JSON（403，改用 opencli）；站长之家、华军软件园、腾讯新闻（未找到）；FourWeekMBA 正文（403）；OpenAI 官方账号（OpenAI、OpenAIDevs）关于本事项的帖（opencli 搜索 “GPT-6.1 Sol” 最新结果止于 10/8，未见）；官方 500k 区间准确率数据（未见）；opencli X 搜索三次空返回。

## 2. 截图（图 13–21、24；22–23 未使用）

全部 2 倍像素比（图 13、14 为 X 嵌入页自带比例，宽 1096；图 24 见下），浅色底。逐张打开目视确认文字清楚、范围正确。

| 文件 | 内容 | QA |
|---|---|---|
| `13-tibo-day7-context.png` | X 官方嵌入卡片：Tibo 主帖（7:45 PM · Oct 11, 2026 美东 = 北京 10/12 07:45；9.4K 赞；1.2K 回复） | **被嵌入页截断**：正文止于 “keeping a lot of context and Show more”，后两句（“sweet spot”“最大 1M”）看不到；完整文字见图 24。卡片 UI 为英文 |
| `14-tibo-no-long-context-fee.png` | X 官方嵌入卡片：自回复（7:48 PM · Oct 11 美东 = 北京 07:48；2.2K 赞）；上方带出父帖前半段 | 自回复整句完整；父帖部分同样被截断，仅作上下文 |
| `24-tibo-day7-context-full-text.png` | 主帖全文（“Day 7/” 至 “maximum context length of GPT-6.1 Sol of 1M.”）加头像、名称、时间行与互动数：“下午7:45 · 2026年10月11日 · 100.1万 查看”，1,268 回复 / 955 转发 / “1万” 赞 / 743 书签（取数约北京 10/12 14:18，页面把赞显示为“1万”）。来源：opencli 已登录会话（页面界面为简体中文，点“显示原文”还原英文），CSS zoom 2，视口高 1700 以避开浏览器侧边悬浮按钮，裁切到数字行为止；**不含**“相关/查看引用”行、回复框（含抓取者头像）、其他回复与导航。时间行是本机时区（EDT）的 7:45 PM，与 fxtwitter UTC 23:45:59 互证 | 帖文英文；“显示翻译”、时间行与“查看”为中文界面标签；名称行右侧圆形图标是浏览器翻译插件图标。无抓取者头像/名称入镜（已目视确认；首次取图曾带入回复框，已弃用重取）。分辨率为 CSS zoom 放大渲染，非原生 DPR 2 |
| `15-api-sol-spec.png` | 模型页顶部：标题、规格行、简介与右侧 “1,050,000 context window / 128,000 max output tokens / Apr 30, 2026 knowledge cutoff” | 清楚；右缘有一条页面滚动条细线 |
| `16-api-sol-price-272k.png` | 模型页 Pricing 区：Input $2.00 / Cached input $0.10 / Cache writes $2.50 / Output $10.00 与下方脚注（含 “Prompts with more than 272K input tokens …”“Fast mode prices are 2x Standard. Batch and Flex prices are 50% lower than Standard.”“Ultrafast mode prices are 6x Standard.”） | 清楚 |
| `17-api-pricing-long-context.png` | API 定价页 Flagship models（Standard 页签）：Short context / Long context 两组列，gpt-6.1-sol 长上下文 $4.00 / $0.20 / $5.00 / $15.00；含页脚注 | 清楚。**此页本身没有写 272K 阈值**，阈值只在模型页脚注（图 16） |
| `18-codex-pricing-usage-faq.png` | Codex 定价页（learn.chatgpt.com/docs/pricing）FAQ：“…extended sessions that require the agent to hold more context will use significantly more per message.”“Model choice, context, reasoning, tool use, retrieval, and caching all affect usage, so prompt length alone isn’t a reliable estimate.” | 清楚；末段 “For model recommendations, see Models.” |
| `19-codex-pricing-api-separate.png` | 同页顶部：标题、“ChatGPT Work and Codex share usage…”、“API token prices are separate from subscription usage; don’t use them to estimate included tasks.” | 清楚 |
| `20-codex-pricing-token-rates.png` | 同页 Token rates 表：GPT-6.1 Sol 50 / 2.5 / 250 credits（每百万 token），整表含全部行与脚 “A typical GPT-5.6 Sol task may use 2-15 credits.” | 清楚；**表中每个模型只有一档费率，无长上下文档位** |
| `21-community-28days-tracker.png` | OpenAI Developer Community 帖 “28 days of Shipping at OpenAI”（sam.saffron 发，自动随 X 更新）：Day 1–7 表，第 7 行写 “…remains configurable up to 1M, and incurs no long-context fees when usage is drawn from a subscription.” | 清楚；Day 8 “Pending” 行被切半；该表第 7 行是自动摘要，非 Tibo 原话 |

未做：官方文档里“500k / 订阅长上下文费”的专门段落（不存在，见 evidence-b.md B3）。

## 3. 试验文件与自查

- 试验文件已删：空的 `b-learn-changelog.md`（learn.chatgpt.com/docs/changelog.md 返回 “Not found”，9 字节）；第一次误拆 URL 得到的停放页 `b-wayback/learn-pricing-20261007.html` 与其 headers（已重取）；猜错路径的 `b-ithome.*`（已重取）；空的 `b-help-rate-card.*` 与 `b-reddit-err.txt`、`b-launch-sol61.*`。临时截图在系统临时目录。
- `docs/2610/1012/` 内对抓取者用户名与邮箱关键词的 grep：命中 `a-tc-gemini-1b.html`、`capture-log-d.md`、`d-d1-tao-feed.xml`（A/D 组文件，非本组，未动），本组文件无命中。
- 本组存档不含抓取者头像、显示名、handle；`b-reddit-codex-day7.json` 只含公开帖与评论作者的 Reddit 用户名；opencli 的 X 搜索/线程结果（含登录态读取）未写盘。
- 单个文件超过 1 MB：无（最大 `b-codex-changelog.html` 807 KB）；HTML 原件按规则只存本地。
