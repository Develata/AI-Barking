# 1016 期 A 组抓取日志（DeepSeek 1017 更新与搜索 API）

执行方：claude-sonnet-5-5（Claude Code 子代理）。时刻为北京时间（UTC+8；本机美东 EDT = 北京 − 12 小时）。开工约 20:48，收尾约 21:15（约 27 分钟，在 60 分钟目标内）。fetch 日志里的时间是本机时钟换算。

工具：`barking fetch / shot / heat`（仓库自带）、curl（官方图表 PNG、HF API、fxtwitter JSON、HN Algolia）、WebSearch（共 5 次，仅找线索与 URL）、firecrawl_search（1 次，**额度耗尽，402，未得结果**）、uv + pillow（一次预览，非产出）。未登录任何站点，未运行读取浏览器 Cookie 的工具，未用镜像/代理站。opencli 未使用（Browser Bridge 未连接，按指示直接用 `barking shot`）。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档（`sources/a-…`） |
|---|---|---|---|---|
| 20:48 | https://api-docs.deepseek.com/zh-cn/updates/ 、/updates/ | barking fetch | 200 ok | `updates-zh`、`updates-en` |
| 20:48 | https://api-docs.deepseek.com/zh-cn/guides/search/ 、/guides/search/ | barking fetch | 200 ok | `search-zh`、`search-en` |
| 20:48 | 价格页中英文 `/quick_start/pricing`、`/zh-cn/quick_start/pricing` | barking fetch | 200 ok | `pricing-en`、`pricing-zh` |
| 20:48 | HF 模型卡与 API（`huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017`、`/api/models/…`） | barking fetch | 200 ok；API 响应头只存了部分（工具自动略去抓取地标识） | `hf-card`、`hf-api`（`.json`） |
| 20:48 | https://artificialanalysis.ai/ 、`/providers/deepseek`；20:52 `/models/deepseek-v4-1-flash`、`/leaderboards/models` | barking fetch | 200 ok；全部 grep “1017” 为 0 | `aa`、`aa-deepseek`、`aa-model`、`aa-lb` |
| 20:48 | https://openrouter.ai/deepseek/deepseek-v4.1-flash-1017 | barking fetch | 200 ok，页面存在 | `openrouter` |
| 20:48 | https://lmarena.ai/leaderboard | barking fetch | **403 blocked**（Cloudflare 质询，cf-mitigated: challenge）；按规则不绕过 | `lmarena.html/.headers`（失败证据） |
| 20:48 | https://aihot.news | barking fetch | 200 ok | `aihot` |
| 20:49 | 新闻页 `news261017`（中英）、`news260910`（中） | barking fetch | 200 ok | `news261017-zh/-en`、`news260910-zh` |
| 20:49 | 接入页：Claude Code（中英）、Codex（中英）、WorkBuddy（中）；Anthropic API 指南（中英）；API 参考 `/zh-cn/api/search`；DeepSeek Harness quickstart | barking fetch | 全部 200 ok；Harness quickstart 是 VitePress 页，正文有限 | `cc-zh/-en`、`codex-zh/-en`、`workbuddy-zh`、`anthropic-api-zh/-en`、`api-search-zh`、`harness-quickstart` |
| 20:50 | 官方图表原图：`/zh-cn/img/v4.1_261017_benchmark_table_cn.png`、`…_perf_cost_cn.png`、`v4.1_260910_benchmarktable_cn.png`、`v4.1_260910_benchmark.png` | curl | 200，各 67–560 KB | `a-img-*.png`（原件，不入库） |
| 20:52 | `barking heat`：HN 3 组（“DeepSeek-V4.1-Flash-1017”“DeepSeek V4.1 Flash”“deepseek search api”）+ Reddit 1 组 | barking heat | HN 命中见 `a-heat.json`；“deepseek search api” 0 条；**Reddit 搜索被重定向登录页，失败**（记入 errors） | `a-heat.json` |
| 20:52 | 官方 X 帖 `https://api.fxtwitter.com/status/2111375640717033777` | curl | 200；发帖时间 Sat Oct 17 08:35:54 +0000 2026（北京 16:35:54）；2/4–4/4 未取（无法由 id 推出） | `a-x-deepseek-ai.raw.json` |
| 20:53 | AIHOT 事件页 `/story/f8045374-…`、条目页 `/items/…`（共 8 个） | barking fetch | 200 ok | `aihot-story`、`aihot-item-search`、`ai-wechat`、`ai-ithome`、`ai-decoder`、`ai-tcat`、`ai-mtp`、`ai-wolf`、`ai-changelog` |
| 20:55 | 媒体：The Decoder、IT之家、MarkTechPost、公众号原文 | barking fetch | Decoder、IT之家 ok；**MarkTechPost 403 blocked**（Cloudflare）；**公众号原文页 200 但正文仅 43 个字符（需 JS 渲染），记 blocked，无 `.txt`**，不绕过 | `decoder`、`ithome`；`marktechpost.html/.headers`、`wechat.html/.headers`（失败证据） |
| 20:55 | X 帖 Thom_Wolf `2111401132840890440`、testingcatalog `2111380259295653976` | curl（fxtwitter） | 200 | `a-x-Thom_Wolf.raw.json`、`a-x-testingcatalog.raw.json` |
| 20:56 | 腾讯新闻（2 篇）、凤凰网、KuCoin、AI Weekly、BigGo、AlphaSignal | barking fetch | 全部 200 ok | `qq-1`、`qq-2`、`ifeng`、`kucoin-search`、`aiweekly`、`biggo`、`alphasignal` |
| 21:00 | HN 条目评论 `hn.algolia.com/api/v1/items/50129045`、`…/50129121` | curl | 200（Algolia 数据不含评论分数） | `a-hn-50129045.json`、`a-hn-50129121.json` |
| 21:05 | DeepSeek Harness `guide/providers`、`guide/mcp-memory` | barking fetch | 200 ok；无 Search/Fetch 默认开关描述 | `harness-providers`、`harness-mcp` |
| 20:5x–21:00 | WebSearch ×5、firecrawl_search ×1 | — | WebSearch 用于找 URL（美区索引，不支持新内容全覆盖）；firecrawl **402 额度耗尽**；两轮找“暴击 GPT-6 Astra”“双王炸”“搜索免费”类标题均无果 | 无 |

清单文件：`a-list.tsv`、`a-list2.tsv`、`a-list3.tsv`、`a-list4.tsv`、`a-list5.tsv`、`a-list6.tsv`、`a-list7.tsv`、`a-list8.tsv`（`barking fetch` 的输入，纯文本）；fetch 日志 `a-fetch-log.jsonl`，截图日志 `a-shots-log.jsonl`。

## 2. 图片清单（`images/`，区间 01–12，本组用 10 张；2 倍像素，宽 1400，浅色底）

均由 `barking shot render`（headless Chrome，700 CSS px，DPR 2）渲出整页后只裁切，不改像素；已逐张打开目视确认文字清楚、范围正确。顶栏（Search/Fetch 横幅、导航）均已裁掉；站点页面无登录态。

| 文件 | 内容 | 说明 |
|---|---|---|
| `01-changelog-1017-two-entries.png` | 更新日志 2026-10-17 两条整段：“开放自研搜索 API”、“DeepSeek-V4.1-Flash-1017 更新”（含 11 项成绩列表、脚注、“特别针对 DeepSeek Harness、腾讯 CodeBuddy、WorkBuddy……”段） | 1400×2040；来自 `updates-zh`；列表是文字，非官方表格图 |
| `02-search-billing-zh.png` | 搜索指南“计费”整节：计费表、成功定义、11/1 那句 | 1400×930 |
| `03-search-qps-zh.png` | 指南“限速”节：Search 5、Fetch 5、账号级与目标域名 | 1400×720 |
| `04-changelog-0910-benchmarks.png` | 更新日志 2026-09-10 条：发布段、19 项成绩列表、脚注 | 1400×1710（用于对比） |
| `05-news-1017-benchmark-table.png` | 新闻页“性能–费用的帕累托改进”：官方表格（DeepSeek 1017、V4.1 Flash、Kimi K3、GLM 5.3、GPT 5.6-Sol、Claude Opus 5）、脚注、优化段 | 1400×1820；表格是官方图片原样渲染；含页面标题与引言 |
| `06-news-1017-search-api-billing.png` | 新闻页“开放自研搜索 API”整节（含价格、“两周后（北京时间 2026 年 11 月 1 日 00:00）”） | 1400×880 |
| `07-news-1017-deepswe-cost-chart.png` | 新闻页 DeepSWE v1.1 通过率与每任务费用图（含图注、数据来源、汇率注释与图下标题） | 1400×1158；官方图原样渲染 |
| `08-claude-code-web-search-zh.png` | Claude Code 接入页“使用 Claude Code 的 Web Search 功能”段 | 1400×570；下方官方演示截图已裁掉（文字行之间） |
| `09-codex-web-search-zh.png` | Codex 接入页“联网搜索”节 | 1400×750 |
| `10-pricing-search-fetch-zh.png` | 价格页“网页搜索与抓取价格”节 | 1400×800 |

未截：官方 X 帖卡片（抓取到 JSON 已含全文，且无登录态需求；如要图可补，须只截帖子卡片）；HF 模型卡；英文版页面。11、12 号空。
渲染中间图在 `%TEMP%\barking-shot\26101016-a\`（不入库）。

## 3. 试验文件与清理

未删除任何文件。没有生成需要清理的试验文件（本组在 `images/` 的 01–12 区间只有上述 10 张正式截图；第一次 `cropbox` 对 `/dev/null` 的试探因路径校验被拒，未产生文件）。

## 4. 泄露自查

对 `docs/2610/1016/` 全目录 grep “develata”“Users\QQ”（大小写不敏感）：无命中（含本组全部文本存档与 JSON）。X JSON 来自 fxtwitter 公开接口，内容是官方账号与公开用户的公开帖，无抓取者信息；HF API 响应头已被工具自动略去抓取标识。AIHOT 页面里的登录/订阅提示来自站点。

## 5. 单文件大小

超过 1 MB 的文本存档：无。超过 1 MB 的 HTML 原件（不入库）：`a-aa.html`（1.4 MB）、`a-aa-deepseek.html`（1.0 MB）、`a-aa-model.html`（1.5 MB）、`a-aa-lb.html`（1.8 MB）；其 `.txt` 文本版均小于 1 MB（`a-aa-lb.txt` 约 5.6 千行，已存）。如需入库请确认 `.txt` 大小。

## 6. 假设

- “官方未给时刻”：更新日志、新闻页、指南页只有日期；X 帖时刻来自 fxtwitter JSON（`created_at` UTC，换算北京时间）；公众号 16:11 来自 AIHOT 显示，未核。
- 官方表格图（05、07 与 `a-img-*.png`）数值由我目视读出，并与 HF 卡内文字表逐项对照；一致。
- A6：AA/OpenRouter 页面均无“1017”独立成绩标识，我不推断 AA 条目是否已含 1017。
