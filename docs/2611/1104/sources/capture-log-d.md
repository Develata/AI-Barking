# 1104 期 D 组抓取与热度日志

执行方：claude-haiku-5-5。全部时间为北京时间（UTC+8）。工具：`barking fetch`（curl 限时并发，结果同步记入 `d-fetch-log.jsonl`）与 `barking heat`（HN Algolia 与 Reddit）。未用 opencli、未用 WebFetch、未截图、未登录任何站点。

## 网页存档（barking fetch，2026-11-05 17:19:06 起，6 条，ok 6 / blocked 0 / failed 0，用时 4 秒）

清单：`sources/d-fetch-list.tsv`（name 与 URL 一行一条，去掉查询参数）。每条产出 `.html`（原件，不入库，由 Claude 用 `barking offsite` 记录）、`.headers` 与 `.txt`（文本存档，入库）。

| 时刻（北京） | 名称 | URL | 工具 | 结果 | 字节 | 备注 |
|---|---|---|---|---|---|---|
| 17:19:10 | d1-conductor-announcement | https://www.conductor.build/announcement | barking fetch（curl） | ok | 57876 | txt 91 行；正文即公告 |
| 17:19:08 | d1-conductor-changelog | https://www.conductor.build/changelog | barking fetch（curl） | ok | 121004 | txt 644 行；0.93.0 条目在前段 |
| 17:19:09 | d2-humanlayer-blog | https://www.humanlayer.dev/blog/humanlayer-joins-factory | barking fetch（curl） | ok | 44856 | txt 56 行；标题 “HumanLayer is joining Factory | HumanLayer Blog” |
| 17:19:09 | d2-factory-news | https://factory.com/news/humanlayer | barking fetch（curl） | ok | 134741 | txt 130 行；页面标题 “HumanLayer is joining Factory | Factory” |
| 17:19:07 | d3-openai-covert-underperformance | https://alignment.openai.com/misalignment-reports/covert-underperformance-on-safety-research-tasks/ | barking fetch（curl） | ok | 33054 | txt 185 行；200 且正文完整（非 challenge 页） |
| 17:19:07 | d4-anthropic-vault-particles | https://www.anthropic.com/research/investigating-vault-particles-with-claude | barking fetch（curl） | ok | 195821 | txt 177 行；正文完整 |

各 `.html` 的 sha256 见 `d-fetch-log.jsonl`。

响应头备注（用于发布时刻判断，不是发布时刻本身）：
- d3 OpenAI 页 `Last-Modified: Thu, 05 Nov 2026 08:55:25 GMT`（北京 11-05 16:55:25）。
- d2 Factory 页 meta `datePublished` 2026-11-04T00:00:00.000Z、`dateModified` 2026-11-04T06:06:46.000Z（北京 11-04 14:06:46）。
- d2 HumanLayer 博客 meta `article:published_time` 11/04/2026（只有日期）。
- d4 Anthropic 页 meta `datePublished` 2026-11-04T00:00:00.000Z、`dateModified` 2026-11-04T19:51:49.000Z（北京 11-05 03:51:49）。

未抓取（有意）：Conductor 的 `/privacy` 与 Factory 的 `/news` 列表；D 组所需事实已在上述页面内完整出现，不另取。

## 热度（barking heat，HN 搜索自 2026-11-01 00:00 北京起）

结果写入 `sources/d-heat.json`（最终版，含全部查询）。

| 时刻（北京） | 查询 | 结果 | 工具 |
|---|---|---|---|
| 17:19:46 | HN：Conductor is joining Meta / Conductor Meta / HumanLayer Factory / HumanLayer joins Factory / covert underperformance / Astra sabotage safety research underperform / vault particles Claude / Claude vault | 命中 Conductor 2 条（ID 50369503 173 分 182 评；50369840 50 分 2 评）；Conductor 另有 50371911（1 分 1 评）；HumanLayer 50369245（6 分 0 评）；OpenAI 报告 50376313（5 分 1 评）；Anthropic 页 50371274（3 分 0 评）；“HumanLayer joins Factory” 0 条 | barking heat（HN Algolia） |
| 17:20:49 | 完整查询集重跑（追加 HumanLayer、Factory acquires、Astra、Anthropic vault；Reddit 四条） | HN 结果同上，Astra 一词命中 58 条（多为其他 Astra 话题，未采用）；HN 9 组；Reddit 0 组 | barking heat |

说明：
- 第一次（17:19:46）结果先写入 `d-heat.json`，随后因追加查询需重写，已把第一版移到 scratchpad（`/tmp/claude-1000/-home-develata-gitclone-AI-Barking/ced38392-0f81-4a1c-bfba-ccfc2196bc1f/scratchpad/d-heat-first.json`，不在仓库内）；最终 `d-heat.json` 为第二次完整重跑的结果。
- `barking heat` 拒绝覆盖已有 `d-heat.json` 的行为在第一次追加时出现；按上面处理，未加参数绕过。

## 失败的请求

| 时刻（北京） | 来源 | 查询 | 失败原因 |
|---|---|---|---|
| 17:20:49 | Reddit（old.reddit.com search.json） | Conductor Meta | 响应为登录页（最终 URL `old.reddit.com/login/?reason=lor2`，text/html），不是 JSON |
| 17:20:49 | Reddit | HumanLayer | 同上 |
| 17:20:49 | Reddit | Astra underperform | 同上 |
| 17:20:49 | Reddit | vault particles Claude | 同上 |

Reddit 在此环境下不可用（需登录），按“同一来源失败两次就记失败、换来源”的规则，未重试，未找替代，D 组热度以 HN 为准。

## 未做的事

- 未截图（D 组不需要）。
- 未摘录 HN 评论（D 组不要求）。
- 未取 Anthropic 的预印本 PDF（页面只说明已投稿、未上线；本组未需要 PDF 原文）。
- 全部 6 个网页均为公开页面，未触及付费墙或登录态；未绕过任何限制。
