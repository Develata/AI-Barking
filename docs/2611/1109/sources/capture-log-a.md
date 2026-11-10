# 1109 期 A 组抓取日志

执行：Claude Sonnet 5.5。时间为北京时间（UTC+8，本机时区即北京）。文件前缀 `a-`，截图编号 01–12（另 12b）。fetch 的逐条记录在 `a-fetch-log.jsonl`，截图动作记录在 `a-shots-log.jsonl`，HN 热度在 `a-heat.json`。

## 抓取

| 时刻（北京） | URL | 工具 | 结果 |
|---|---|---|---|
| 11-10 15:29 | anthropic.com/claude-fable-and-mythos-5-2；artificialanalysis.ai/models/claude-fable-5-2（及 `-xhigh`）、/leaderboards/models；vals.ai/benchmarks；platform.claude.com 的 pricing、models overview；HN 50429068、50429906、50431144 | `barking fetch`（`a-list.tsv`） | 10 条全部 ok（docs.claude.com 跳转到 platform.claude.com，已看过 `.txt` 是正文） |
| 15:29 | AA 各档与对照：fable-5-2-high/medium/low、opus-5-5(max/xhigh/high)、gemini-4-carbon、gpt-6-astra、fable-5-1 模型页；methodology；faq | `barking fetch`（`a-list2.tsv`） | 11 条 ok；页面静态源里内嵌 Next.js payload，含各评测分数、成本、token、耗时，用 `a-aa-extract.py` 提取为 `a-aa-extract-out.tsv` |
| 15:55 | vals.ai/benchmarks/vals_index、/models/anthropic_claude-fable-5-2、/benchmarks/swebench；系统卡 PDF（anthropic.com/document/…-system-card）；docs fable-5-2 overview、fast-mode、effort | `barking fetch`（`a-list3.tsv`） | 7 条 ok；vals-model 首次连接失败后自动重试成功；**`/benchmarks/swebench_multimodal` 404（failed，URL 写错，未再试）**；系统卡 PDF 264 页 23.9 MB，`pdftotext` 全文 504 KB |
| 15:57 | HN 50429068、50429906、50431144 | `barking heat --item` | `a-heat.json`、`a-hn-<id>.json`：493 分/752 条；974 分/6 条；338 分/244 条 |
| 15:58 | cognition.com/blog/frontier-code-2.0；cognition.com/frontiercode；web.archive.org/web/20261028175237/…frontier-code-2.0 | `barking fetch`（`a-list4.tsv`、`a-list5.tsv`） | 3 条 ok；榜单曲线为客户端渲染，只有 JSON-LD；web.archive 快照为博客文章页，仅用于查看 10-28 页面当时写了什么 |
| 16:0x | api.fxtwitter.com/status/2119867114450268322、/2119867707546071125 | curl（`--max-time 60`） | 200；两条均为 Anthropic 官方账号公开帖，JSON 不含抓取者信息；`a-x-<id>.json` |
| 16:1x | exa 搜索（Fable 5.2 报道、中文报道、官方 X 帖） | mcp exa `web_search_exa` | 仅作线索与定位；Firstpost、TechTrends360、IT之家只取到搜索摘要 |
| 16:18 | VentureBeat、智东西、36kr EU、智通财经、Decrypt（Yahoo）、Firstpost、Reuters（StreetInsider） | `barking fetch`（`a-list6.tsv`） | 5 条 ok；**Firstpost 403、StreetInsider 403（Cloudflare 质询），blocked，未用镜像，未再试** |
| 16:26 | AA 的 Fable 5.1 xhigh/high/medium/low 模型页、AA /evaluations/frontiercode-2-0、Vals Opus 5.5 与 Sonnet 5.5 模型页、IT之家原文 | `barking fetch`（`a-list7.tsv`） | 8 条 ok；Vals 对 Opus 5.5 与 Sonnet 5.5 的 Compute Effort 均为 max |
| 16:0x | WebSearch（中英文关键词） | 搜索引擎 | 无 Fable 5.2 相关结果（索引滞后），弃用 |

自写脚本（均在 `docs/2611/1109/sources/`）：`a-aa-extract.py`（AA payload → TSV）。各项评测名次与每任务输出 token 是当场内联脚本计算的，结果已写入 `evidence-a.md`；原始数据可由 `a-aa-*.html` 重算、`a-oc-pos.js`、`a-oc-shot.sh`（复制自 1108）、`a-oc-run.sh`（把 JS 文件按 `__VAR__` 替换后交给 `opencli browser eval`）、`a-oc-table.js`（早期试验，未用于成品）、`a-oc-isolate.js`、`a-oc-prescroll.js`、`a-oc-iso.sh`（隔离元素截图）。

## 截图方法与说明

- 浏览器：`opencli browser`（会话 `a-shot`、`a-fg`，后台窗口；所截页面均未显示登录态；`document.visibilityState` 在后台为 hidden）。
- 隔离截图（`a-oc-iso.sh` + `a-oc-isolate.js`）：为避免高视口截图时滚动错位，把要截的原始元素（段落、表格、图）按原顺序移到一个干净容器顶部、隐藏页面其余部分后截图；元素与数据不改，只去掉站点导航栏与浏览器翻译插件浮层（`#immersive-translate-*`）。段落的字体、字号、行距在移动前固化，保持与原页一致；段间距为 20 px，所以与原页略不同；站点链接下划线在个别段落里没有保留。
- 图表入场动画（仅 03）：后台标签页不跑 CSS 动画，图上的点和线停在透明初始态；隔离脚本里加一条样式把动画强制到结束态（`animation:none; opacity:1`），点位与坐标不变，并与 `a-release-chart-points.txt`（SVG 内 aria-label 逐字）和系统卡 Figure 8.4.A 对照过。
- 版式：除另注，均为 CSS zoom 1.5–2、输出宽 1400 px（约 1.5–2 倍像素）；`12b` 为 1 倍像素（页面视口截图，宽 870 px，字号较小但清楚）。
- 裁切：`barking shot cropbox|stack`；PDF 页用 `barking shot pdf --dpi 164`（1394 px 宽）。

| 文件 | 来源页 | 内容 | 备注 |
|---|---|---|---|
| 01-release-intro-price.png（**作废，见 01b**） | 发布页 | 同一底层模型/两版区别、“up to 30% faster”与 $8/$40、fast mode $16/$80、“When to use…start with Claude Opus 5.5” 四段 | 浅色底；隔离截图；段落按原顺序，中间页面上的其他段落未含 |
| 02-release-benchmark-table.png（**作废，见 02b**：字体与原页不同） | 发布页 | “Benchmark performance” 引导句 + 完整跑分表（全部行与列、列头）+ 表下脚注 | 浅色底；隔离截图；表在原页是窄栏横向裁切的，截图用自然宽度 896 px 全部显示 |
| 01b-release-intro-price.png | 发布页 | 与 01 同四段；段落按原页顺序，**第 3 段与第 4 段之间略去原页两段（“Safety and alignment.”“We describe more about our safety testing below…”），图中以一道空隙示意** | 浅色底；隔离截图（字体已固化为原页字体）；用 `barking shot stack` 拼两段 |
| 02b-release-benchmark-table.png | 发布页 | 与 02 同内容，重拍：字体固化为原页字体 | 浅色底；隔离截图 |
| 03-release-effort-frontiercode.png | 发布页 | “Effort and efficiency” 段 + FrontierCode 2.0 Accuracy vs. cost 图（图例、坐标轴）+ 图下说明 | 浅色底；图点位取动画结束态（见上）；图上标注只有 Fable 5.2 的 Low/Med/High/Xhigh/Max |
| 04-pricing-model-table.png | 价格页 | “The following table shows pricing for all Claude models” + 整张模型价格表（含展开的 Additional models，Fable 5.1 $10/$50 等） | 浅色底；隔离截图，表格边框线在隔离后消失，数值与行列完整 |
| 05-pricing-fast-mode.png | 价格页 | Fast mode 说明 + 价格表（Fable 5.2 $16/$80；Opus 5.5 $8/$40；Opus 5 / 4.8 $10/$50） | 隔离截图 |
| 06-models-overview.png | models overview | “start with Claude Opus 5.5” 引导句 + 对比表（价格、API ID、默认 effort、1M 上下文、128K 最大输出） | 隔离截图；“Additional details” 折叠项未展开 |
| 07-aa-leaderboard-top-rows.png | AA 榜单 | Intelligence Index / Cost per Task / Time per Task / First Answer Token 列与前 21 行 | **节选**：表有 287 行，隐藏了第 22 行以后；右侧最后一列（链接列）已裁掉；无页面标题 |
| 08-aa-fable52-max-xhigh-high.png | AA 三个模型页 | Fable 5.2 max / xhigh / high 的 Model summary：指数、名次、耗时、成本、输出 token（275M / 144M / 85.3M） | 隔离截图（标题 + Model summary），三张竖向拼接 |
| 09-aa-opus55-max-fable51-carbon.png | AA 三个模型页 | Opus 5.5 max、Fable 5.1 max、Gemini 4 Carbon high 的同一摘要 | 同上 |
| 10-vals-index-leaderboard-and-note.png | Vals 榜单页 + Fable 5.2 模型页 | 榜单前 8 行（准确率、Cost / Test、Tokens / Test、Duration / Test）+ Nov 9 更新说明（“We evaluated it at High effort.”、SWE-bench Multimodal 62.29% 对 Opus 5.5 70.21%） | 榜单为隔离截图（样式简化，内容为原文本）；更新说明为模型页直截（zoom 1.5），右侧带一条灰色竖线 |
| 11-syscard-frontiercode-text-p209.png | 系统卡 PDF p209 | §8.4 FrontierCode 正文：132 题、mean@5、22/660 超时、各 effort 分数与成本、“single result from Cognition” | PDF 渲页，164 dpi |
| 12-syscard-frontiercode-figure-p210.png | 系统卡 PDF p210 | Figure 8.4.A 与图注（Cognition 跑、Claude Code / Codex CLI / Antigravity CLI、y 轴起点 10%） | PDF 渲页 |
| 12b-zhidx-headline.png | 智东西 | 夸大例：标题、导语“几乎碾压Fable 5.1、Opus 5.5、GPT-6 Astra”、嵌入的官方帖 | 页面视口截图，1 倍像素；翻译插件浮层与右侧栏已裁掉；页面含站点登录栏，已裁；嵌入的是 Anthropic 官方账号帖 |

逐张目视确认：01b、02b、03（01 与 02 作废）、04、05（缩略读数）、06、07、08、09、10、11、12、12b 均已打开核对文字清楚、范围正确。

## 隐私与自查

- 存档来源均未登录；`.headers` 由工具去掉了抓取者信息；fxtwitter JSON 只含官方账号公开帖。
- 所截页面均未显示登录态；截图中无抓取者头像、昵称或 handle（`12b` 中是 Anthropic 官方账号的公开帖卡片）。
- 交付前对 `docs/2611/1109/` 做过用户名与邮箱关键词检索（见回报）。

## 废弃与待删除文件

见回报“待删除文件”清单。
