# 1117 期 B 组抓取日志

时刻均为北京时间（本机即 UTC+8）。工具：`barking fetch`（限时并发，日志 `b-fetch-log.jsonl`）、`barking shot`（headless Chrome，700 CSS px、DPR 2；日志 `b-shots-log.jsonl`）、`barking heat`、`curl`（个别请求，`--max-time` 30–60 秒）、WebSearch（只作线索，不作证据）。
opencli 未使用（扩展未连接，按派工单直接用 barking shot）。未登录任何站点，未输入凭据，未使用本机浏览器 Cookie。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 16:27:11 | https://microsoft.ai/news/introducing-mai-code-2-and-mai-code-2-flash/ | barking fetch | ok，`b-blog.*` |
| 16:27:12 | https://microsoft.ai/news/microsoft-ignite-2026/ | barking fetch | ok，`b-ignite.*` |
| 16:27:13 | https://microsoft.ai/models/mai-code-2 | barking fetch | ok，`b-modelpage.*` |
| 16:27:11 | Reuters 报道（见 evidence-B 第 3 节） | barking fetch | blocked，HTTP 401，仅拦截页；付费墙/校验不绕过 |
| 16:27:58 | https://microsoft.ai/pdf/MAI-Code-2-And-MAI-Code-2-Flash-Model-Card.pdf?v=3 | barking fetch | ok，`b-modelcard.pdf|.txt` |
| 16:27:58 | https://microsoft.ai/pdf/MAI-Code-2-Data-Card.pdf 与 …Flash-Data-Card.pdf | barking fetch | ok，`b-datacard.*`、`b-flashdatacard.*`（文本里无价格/成本字样，未用） |
| 16:27:59 | https://docs.github.com/en/copilot/reference/ai-models/supported-models | barking fetch | ok，`b-copilotmodels.*` |
| 16:27:59 | https://github.com/features/copilot/plans | barking fetch | ok，`b-copilotplans.*`（模型清单文字顺序里 “Free plan” 段含 MAI Code 2，与 Changelog 的套餐范围不一致，未取舍，以 Changelog 为准） |
| 16:27:59 | https://docs.github.com/en/copilot/concepts/billing/copilot-requests | barking fetch | ok，`b-copilotmult.*`（旧版 premium request 倍率页，无 MAI 行，未用） |
| 16:27:59 | https://github.blog/changelog/ | barking fetch | ok，`b-ghchangelog.*`（列表页，用来找到下面两条条目链接） |
| 16:28:00 | https://github.blog/changelog/2026-11-17-mai-code-2-is-now-available-in-github-copilot/ | barking fetch | failed，404（猜测的 URL，作废） |
| 16:28:16 | https://azure.microsoft.com/en-us/pricing/details/ai-foundry/ | barking fetch | ok，`b-foundry.*`；文本无 “MAI” |
| 16:28:41 | https://docs.github.com/en/copilot/reference/copilot-billing/models-and-pricing | barking fetch | ok，`b-copilotpricing.*`，含 MAI Code 2 价格与脚注 4、5 |
| 16:33:11 | https://artificialanalysis.ai/evaluations/frontiercode | barking fetch | ok，`b-aa-frontiercode.*`（静态页，图表数值读不出；无 MAI Code 2） |
| 16:33:11 | https://artificialanalysis.ai/models/gpt-6-1-sol | barking fetch | ok，`b-aa-sol.*` |
| 16:33:11 | https://artificialanalysis.ai/models/mai-code-2 | barking fetch | failed，404（猜测） |
| 16:33:54 | https://artificialanalysis.ai/models/creators/microsoft-ai；…/models/mai-code-2-0 | barking fetch | failed，404（猜测）；同一来源失败两次，停止 |
| 16:3x | https://api.fxtwitter.com/status/2122758850566910255（Suleyman 的 X 长文） | curl | ok，`b-suleyman-fx.json`（公开 JSON，无抓取者信息） |
| 16:34 | web.archive.org 快照 20261118003654（博客）、20261117184519（模型页） | curl（先查 `archive.org/wayback/available`，再取 `id_` 原文） | ok，`b-blog-wayback-20261118003654.html`、`b-modelpage-wayback-20261117184519.html`（原为 gzip，已解压成 HTML；仅用于看“以前写了什么”） |
| 16:35 | web.archive.org 20261118030821（Ignite 总览） | curl | 得到 “The Wayback Machine has not archived that URL” 的提示页，存为 `b-ignite-wayback-20261118030821.html`（4.6 KB，无内容）；同目录 `.gz` 是未解压原文件 |
| 16:37:01–06 | tbreak、CNMO、80aj、新浪、17173、cnblogs 报道页 | barking fetch | ok，`b-xtbreak.*`、`b-xcnmo.*`、`b-x80aj.*`、`b-xsina.*`、`b-x17173.*`、`b-xcnblogs.*` |
| 16:37:01 | techzine.eu 与 fourweekmba.com 的报道页 | barking fetch | blocked，HTTP 403，未读到原文 |
| 16:37:31 | GitHub Changelog 两条（MAI Code 2 上线；MAI-Code-1.1-Flash 12 月下线） | barking fetch | ok，`b-ghmai.*`、`b-ghdepr.*`（后者：2026-12-18 下线 MAI-Code-1.1-Flash，未用） |
| 16:32:54 | HN `50535885`、HN 搜索 “MAI Code 2” | barking heat（前缀 `b-hn`；`b-` 的第一次运行同时试了 Reddit） | HN ok：350 分 / 153 评，评论树 `b-hn-hn-50535885.json`，热度 `b-hn-heat.json`；Reddit 被登录页拦截（`b-heat.json` 记 errors），另用 curl 取 www.reddit.com search.json 返回 “Blocked”，不再重试 |
| 16:44 | HN 条目 50535230（GPT-6.2 帖，复核时间） | barking heat（前缀 `b-hn62`） | ok：提交 2026-11-17T17:46:43Z，1251 分；`b-hn62-heat.json`、`b-hn62-hn-50535230.json` |
| 16:44 | https://artificialanalysis.ai/evaluations/frontiercode 渲染 | barking shot render（700 与 1000 CSS px 两次，中间图） | ok：图表默认 27 of 83 models，读到 Opus 5.5 (max) 52.2%、GPT-6.1 Sol (max) 49.1%；无 MAI Code 2 |
| 16:3x | WebSearch 两次（英文、中文查询） | WebSearch | 只用来发现二手报道与 Suleyman 帖链接，搜索摘要中的断言未作证据 |

## 截图（headless Chrome，700 CSS px，DPR 2；中间图在系统临时目录，库内是裁切成品）

| 文件 | 来源页（URL） | 内容 | 说明 |
|---|---|---|---|
| 13-blog-headline-98-40.png | 微软博客 | 从 “Performance and speed” 到 Flash 对比段：“Production quality at less than half the price”、not yet tuned、98%/40%/96%/5%、64% | 上沿多带了上段最后一行 “competing models.”（完整行，未截断） |
| 14-blog-title-top.png | 微软博客 | 标题、日期 “Nov 17, 2026”、题图、首段 | |
| 15-chart-code2-avg.png | 微软博客 | 图表“98%/40%/96%/5%”标题、平均通过率与平均成本 | 页面可见部分不显示 “promotional” |
| 16-chart-code2-bench.png | 微软博客 | 同一图表的分基准通过率与成本（四项基准全部行） | 高 2480 px |
| 17-chart-flash.png | 微软博客 | Flash 图表 | **废弃**：下沿带入下一节的半截文字（“d coding”），改用 18 |
| 18-chart-flash-full.png | 微软博客 | Flash 图表整张（平均与分基准，三项基准） | 高 3060 px |
| 19-blog-availability.png | 微软博客 | “Get started”可用范围段 | |
| 20-blog-footnotes.png | 微软博客 | “About these results” 与脚注 1–3（含 “50% promotional price”、MSBench、258 annotators） | |
| 21-copilot-price-mai.png | GitHub Docs Models and pricing | MAI 价格表（1000 CSS px 宽渲染） | **废弃**：宽 2000 px，超过 1400 px 上限，改用 22 |
| 22-copilot-price-mai.png | 同上（700 CSS px 渲染） | “Microsoft” 表：MAI-Code-1.1-Flash、MAI Code 2、MAI Code 2 Flash 价格 | 含脚注标号 [4][5]，脚注原文见 23 |
| 23-copilot-price-footnotes.png | 同上 | 脚注 4、5（促销价、2027-05-17 起价格） | |
| 24-copilot-price-sol-opus.png | 同上 | 列头 + GPT-6.1 Sol 两行 + GPT-6.2 Sol 两行 + Claude Opus 5.5 一行（四段拼接，浅蓝条是拼接间隔，非页面元素） | Opus 行末栏 “Not applicable” 折行，下沿紧贴该行 |
| 25-blog-wayback-67pct.png | web.archive.org 快照 20261118003654 的微软博客 | “Production quality …” 到 “reviewers preferred its results over GPT-6 Sol 67% of the time” | **编号超出 13–24 区间**（区间已用尽）；快照仅用于看旧值 |
| 26-ignite-lineup-98-40.png | 微软 Ignite 总览 | 阵容前三条（98%/40%、Flash 65%/20%） | 下沿带入下一条的半截文字，改用 28 |
| 27-ignite-code-section.png | 微软 Ignite 总览 | “Incredible coding performance …” 段（98%/40% of the price、not yet tuned、64%） | 超出区间编号；下沿切在 Flash 段中间（半行），文字主体完整 |
| 28-ignite-lineup-clean.png | 微软 Ignite 总览 | 同 26，裁到第三条结束 | 超出区间编号 |
| 30-aa-frontiercode11-score.png | AA FrontierCode 1.1 评测页（700 CSS px 渲染） | “FrontierCode 1.1: Score” 柱状图（默认 27 of 83 models）与图题 | 超出区间编号；图右侧被页面的横向滚动截断（最右一根柱不完整） |
| 29-ghchangelog-availability.png | GitHub Changelog（MAI Code 2 与 Flash） | “billed under usage-based billing”、可用套餐、“annual plans”、可选界面 | 深色底页面；超出区间编号 |

目视确认：以上图均已逐张打开（含 18、28、30），文字清楚、范围与说明一致。页面均为匿名渲染，无登录栏、头像或抓取者信息。

## 编号偏差（给 Claude 合并用）

本组区间 13–24 已用尽（含两张废弃图），25–30 为超出区间的增补，与 A 组 01–12 无冲突：25 博客 Wayback 旧值 67%；26/28 Ignite 阵容；27 Ignite 编码段；29 GitHub Changelog 可用范围；30 AA FrontierCode 1.1 分数图。重编号由 Claude 决定，本组没有改名。

## 其他说明

- 本组曾有一次误含 `rm` 的命令被权限拦截（清理 `.gz` 临时文件），未执行；此后没有再用 `rm`，`.gz` 留在待删除清单。
- 自写的辅助脚本都在会话 scratchpad（不在 `sources/`）：解码博客与 Ignite 内嵌图表 iframe、比对 Wayback 副本与现版、读取 AA 页面数据等；未入库。
- `b-hn-hn-50535885.json`、`b-hn62-hn-50535230.json` 的评论树文本里有 HN 公开用户名（评论者），无抓取者信息。

## 待删除文件（Claude 收尾时一次删除；本组未执行任何删除）

- `docs/2611/1117/images/17-chart-flash.png`（废弃，被 18 取代）
- `docs/2611/1117/images/21-copilot-price-mai.png`（废弃，宽 2000 px）
- `docs/2611/1117/images/26-ignite-lineup-98-40.png`（被 28 取代）
- `docs/2611/1117/sources/b-ignite-wayback-20261118030821.gz` 与 `b-ignite-wayback-20261118030821.html`（Wayback 无 Ignite 快照，空内容）
- `docs/2611/1117/sources/b-modelpage-wayback-20261117184519.gz`（已解压为同名 `.html`，`.gz` 冗余）
- `docs/2611/1117/sources/b-heat.json`（第一次 heat 运行的残留，只含 Reddit 失败记录；可保留作失败证据）
- 猜测 URL 失败产生的 404 页：`b-ghblog.*`、`b-aa-mai.*`、`b-aa-ms.*`、`b-aa-mai2.*`（若存在）

临时脚本在会话 scratchpad，不在库内。
