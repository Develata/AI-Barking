# 1014 期 D 组（速览）抓取日志

执行方：claude-haiku-5-5（D 组，派工单 `.handoff/2026-10-14-1014-evidence.md`）。
时区：北京时间（UTC+8）；本机 EDT 时刻已换算。抓取批次约 2026-10-15 17:05–17:30。

## 抓取记录

| 北京时间 | URL | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 17:08 | https://rea.tools/ | curl（UA 为普通 Chrome） | 200，54587 B | `d-rea-tools.html`，`d-rea-tools.txt`（barking html2txt） |
| 17:08 | https://raw.githubusercontent.com/morluto/rea/main/README.md | curl | 200，22177 B | `d-rea-readme.md`（raw 文本，起初误存为 .html，已改名并把 .html 移出仓库目录） |
| 17:08 | https://api.github.com/repos/morluto/rea | curl | 200（响应头 Date 2026-10-15 09:10:45 GMT） | `d-gh-rea-api.html`（JSON） |
| 17:08 | https://api.github.com/repos/morluto/rea/releases?per_page=10 | curl | 200，292502 B | `d-gh-rea-releases.html`（JSON） |
| 17:08 | https://raw.githubusercontent.com/morluto/rea/main/DISCLAIMER.md | curl | 200，7105 B | `d-rea-disclaimer.md` |
| 17:20 | https://rea.tools/faq/ | curl | 200，29147 B | `d-rea-faq.html`，`d-rea-faq.txt` |
| 17:08 | https://raw.githubusercontent.com/rea-githu/REA/main/README.md（山寨仓库，只读文本） | curl | 200，1759 B；**未下载 zip、未解压、未运行任何文件** | `d-scam-rea-githu-readme.md` |
| 17:08 | https://api.github.com/repos/rea-githu/REA（山寨仓库元数据） | curl | 200 | `d-scam-rea-githu-api.json` |
| 17:08 | https://anoma.ly/notes/opencode-2-0/ | curl | 200，854904 B | `d-opencode2.html`，`d-opencode2.txt`（见下“清理”） |
| 17:08 | https://typesafe.ai/blog/jev-1-14 | curl | 200，220654 B | `d-jev114.html`，`d-jev114.txt` |
| 17:08 | https://x.ai/news/grok-voice-3 | curl | 200，206360 B | `d-grok-voice3.html`，`d-grok-voice3.txt` |
| 17:08 | https://developers.openai.com/api/docs/deprecations | curl | 200，499585 B | `d-openai-deprec.html`，`d-openai-deprec.txt` |
| 17:08 | https://research.google/blog/analyzing-technical-documents-with-deep-review/ | curl | 200，187585 B | `d-google-deep-review.html`，`d-google-deep-review.txt` |
| 17:08 | https://noma.security/noma-labs/sneakpeek-how-the-preview-pane-in-claude-code-desktop-can-exfiltrate-your-data | curl | 200，92468 B | `d-noma-sneakpeek.html`，`d-noma-sneakpeek.txt` |
| 17:10 | https://artificialanalysis.ai/speech-to-speech | curl | 200，1158213 B（响应头 Date 2026-10-15 09:10:12 GMT） | `d-aa-speech.html`，`d-aa-speech.txt` |
| 17:10 | https://artificialanalysis.ai/leaderboards/speech-to-speech | curl | 404（未用） | — |
| 17:10 | https://artificialanalysis.ai/speech-to-speech-leaderboard | curl | 404（未用） | — |
| 17:10 | https://artificialanalysis.ai/ | curl | 200（仅探测，未用） | — |
| 17:20 | firecrawl_search：Noma SneakPeek / Anthropic advisory / CVE（一次，creditsUsed 2） | firecrawl | 返回 5 条：Noma 原文、generalanalysis 第三方复盘、Dark Reading、Claude 官方页面（无关）等；未见 Anthropic 一手公告或 CVE 编号 | 未存档（仅线索，引用的原句均已在 Noma 原文 `d-noma-sneakpeek.txt` 中核对） |
| 17:13 | `barking heat`（首次，HN 9 组、Reddit 3 组） | barking | HN 成功；Reddit 3 条失败（old.reddit.com 重定向到登录页，响应非 JSON）；写入 `d-heat.json` | 该文件随后被删除（见下） |
| 17:16 | `barking heat docs/2610/1014 d`（第二次，HN 8 组 + 6 条 item 详情、Reddit 3 组） | barking | HN 成功，共 6 个 item 详情；Reddit 3 条请求失败（old.reddit.com 重定向到登录页，`expected value at line 2 column 5`）；HN 查询 “OpenAI deprecate GPT-4o API”、“GPT-4.1 deprecation” 0 条命中 | `d-heat.json`，`d-hn-50028275.json`，`d-hn-50094994.json`，`d-hn-50095353.json`，`d-hn-50090731.json`，`d-hn-50093909.json`，`d-hn-50099375.json` |

## 本组文本清理与试验文件

- `barking html2txt` 对 `anoma.ly` 页面输出词被逐词断行，无法通读。改用 `d-clean-html.py`（本组自写的 Python 标准库脚本，去标签、合并空白）重做，结果覆盖 `d-opencode2.txt`。同样的清理也覆盖了 `d-jev114.txt`、`d-grok-voice3.txt`、`d-google-deep-review.txt`、`d-noma-sneakpeek.txt`、`d-openai-deprec.txt`、`d-aa-speech.txt`。清理前后内容未改，只是分行。
- `barking heat` 首次运行写入的 `d-heat.json` 是本组自己的试验文件，工具拒绝覆盖，已删除后按第二次运行重写。删除的是本组自生成的文件，记入此处。
- `d-rea-readme-raw.html` 为误存的 README 副本，已移出 `sources/` 目录（移至 scratchpad），同 raw 内容保留在 `d-rea-readme.md`。

## 发布时刻（本组关键页面的官方标注）

| 条目 | 官方页面标注 | 北京时间 |
|---|---|---|
| D1 REA 官方仓库 | 创建 2026-04-14T00:51:32Z | 2026-04-14 08:51（官方未给页面发布时刻） |
| D1 release 6.1.0 – 6.5.0 | 2026-10-09T01:08:45Z ～ 2026-10-14T12:32:45Z | 10-09 09:08 ～ 10-14 20:32 |
| D2 OpenCode 2.0 | `article:published_time` 2026-10-12（无时刻） | 2026-10-12（官方未给时刻） |
| D3 Jev 1.14 | 页面标注 Oct 14, 2026 | 2026-10-14（官方未给时刻） |
| D4 Grok Voice 3.0 | 页面标注 Oct 14, 2026；AA 数据日期 October 13, 2026 | 2026-10-14（官方未给时刻） |
| D5 OpenAI 退役公告 | “On October 14, 2026, we notified developers …” | 2026-10-14（官方未给时刻） |
| D6 Deep Review | October 14, 2026 | 2026-10-14（官方未给时刻） |
| D7 Noma 文章 | October 14, 2026；时间线只有日期 | 2026-10-14（官方未给时刻） |

## 热度（barking heat，取得时刻北京时间 2026-10-15 17:16）

| 条目 | HN 条目 | 分 / 评 | 投稿时刻 |
|---|---|---|---|
| D1 REA | 50028275（rea.tools） | 805 / 337 | 10-10 08:37 |
| D2 OpenCode 2.0 | 50094994（anoma.ly） | 598 / 232 | 10-15 04:48 |
| D3 Jev 1.14 | 50095353（typesafe.ai） | 283 / 91 | 10-15 05:11 |
| D4 Grok Voice 3.0 | 50090731（x.ai） | 6 / 1 | 10-15 00:48 |
| D6 Deep Review | 50093909（research.google） | 3 / 0 | 10-15 03:43 |
| D7 Noma SneakPeek | 50099375（noma.security） | 3 / 0 | 10-15 13:02 |
| D5 OpenAI 退役 | 无命中 | — | — |
| Reddit（REA、OpenCode 2.0、Jev） | 全部失败（登录重定向） | — | — |

以上热度只是快照，分数随时间变化。HN 评论未摘录（D 组不要求）。

## 假设

- 北京时间：`barking heat` 与 GitHub 的换算以 UTC+8 处理；本机 `date` 输出为 EDT，已按 +12 小时换算。
- 页面无时刻的官方条目，只写日期并注明“官方未给时刻”，不推测时刻。
- AA 页是动态榜单，本期以 2026-10-15 09:10 GMT 前后的快照为准；xAI 页引用的 AA 数据日期为 2026-10-13。
- D7 的修复版本与时间线视为研究方自述，不当作 Anthropic 一手确认。
