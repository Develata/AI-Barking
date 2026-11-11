# 1110 期取证日志：D 组（速览）

时间为北京时间（UTC+8）。工具：`barking fetch`、`barking heat`（经 `zsh -lc 'cargo run … tools/barking'`）、curl（个别一次性请求）、WebSearch（只找线索，结论以一手页为准）。本组未截图，`d-shots-log.jsonl` 不存在，无截图动作。

## 批量存档（`barking fetch docs/2611/1110 d`）

| 时间（BJ） | 名称 | URL | 工具 | 结果 | 备注 |
|---|---|---|---|---|---|
| 14:31:02 起（一批，并发 6） | d1-alphaproof-blog | https://deepmind.google/blog/introducing-alphaproof-code/ | barking fetch | ok（200，154,076 字节，703 行） | |
| 同上 | d1-code-verification | https://google-deepmind.github.io/code-verification | barking fetch | ok（200，1,985,046 字节，2,010 行） | 即技术论文页（Verso 版）；txt 约 0.36 MB |
| 同上 | d2-vulncheck | https://vulncheck.com/blog/anthropic-glasswing-november-update | barking fetch | ok（200，251,936 字节，225 行） | 第三方分析，L3 |
| 同上 | d2-anthropic-cvd | https://red.anthropic.com/2026/cvd/ | barking fetch | ok（200，124,896 字节，900 行） | 台账截至 11/02 |
| 同上 | d3-strands | https://strandsagents.com/blog/amazon-saves-millions-replacing-sonnet-with-strands-decider/ | barking fetch | ok（200，138,316 字节，843 行） | |
| 同上 | d4-homeopathic | https://alignment.anthropic.com/2026/homeopathic-learning/ | barking fetch | ok（200，25,817 字节，88 行） | |
| 同上 | d4-arxiv | https://arxiv.org/abs/2611.10110 | barking fetch | ok（200，41,687 字节，233 行） | 摘要页，未取 PDF |
| 同上 | d5-openai-cot | https://alignment.openai.com/cot-monitorability-model-lifecycle/ | barking fetch | ok（200，48,880 字节，298 行） | 未被 403 |
| 同上 | d6-openrouter | https://openrouter.ai/blog/insights/mcp-vs-cli-vs-skills/ | barking fetch | ok（200，121,946 字节，432 行） | |
| 同上 | d8-claude-sheets | https://claude.com/resources/articles/sheets-whiteboards-data | barking fetch | ok（200，535,039 字节，750 行） | |
| 14:33:45 | d7-token-plan | https://qwencloud.com/pricing/token-plan | barking fetch（第二批清单） | ok（200，180,221 字节，229 行） | 定价页，未见 Qwen4-Max；落地域名 www.qwencloud.com |

第一批 10 条全部 ok，无 blocked/failed。日志：`docs/2611/1110/sources/d-fetch-log.jsonl`。

## 单次请求与 API

| 时间（BJ） | 名称 | URL | 工具 | 结果 | 备注 |
|---|---|---|---|---|---|
| 约 14:35 | curl GitHub 搜索，curl 仓库 `label:vuln:NONE` | https://api.github.com/search/issues?q=repo:curl/curl+label:%22vuln%3ANONE%22&per_page=1 | curl `--max-time 60` | 200；总数 51（`vuln:LOW` 8，`vuln:ASSESS` 1；同样方式计数） | 仓库全部 issue，未限定本批 Anthropic 报告；存 `d-d2-curl-labels-vuln-none.json` |
| 约 14:38 | HN Firebase 条目 50449130 | https://hacker-news.firebaseio.com/v0/item/50449130.json | curl `--max-time 60` | ok；score 24，descendants 1，链接指向 X 帖 | 存 `d-hn-50449130-raw.json` |
| 约 14:38 | fxtwitter 帖子 | https://api.fxtwitter.com/Alibaba_Qwen/status/2120335993643684315 | curl `--max-time 60` | 200 | 存 `d-d7-qwen4max-fxtwitter-raw.json`；帖内图片未下载（仅存本地，本组不需截图） |

## 检索（只找线索，结论以一手页为准）

| 时间（BJ） | 查询 | 工具 | 结果 |
|---|---|---|---|
| 约 14:37 | “Qwen4-Max preview Alibaba Token Plan Alibaba_Qwen” | WebSearch（standard） | 无 Qwen4-Max 结果；返回的是 Qwen3.8-Max-Preview（7 月 19 日、Token Plan 独占）的报道，未采信 |
| 约 14:39 | “"fully model-generated" "without human review or triage" OSS Scanner curl Anthropic” | WebSearch（standard） | 只找到二手报道（implicator.ai、AlphaSignal、Neoteo、SmartScope）；Anthropic 官方原文未找到 |

## 热度（`barking heat`）

| 时间（BJ） | 调用 | 结果 | 备注 |
|---|---|---|---|
| 14:32:20 | `heat docs/2611/1110 d --item 50439193 --item 50447074 --item 50436640 --item 50443716` | 四条写入 `d-hn-<id>.json`：50439193 608 分/182 嵌套评论；50447074 373 分/160；50436640 702 分/246；50443716 170 分/137 | 该次结果原写入 `d-heat.json`，随后改名为 `d-heat-items.json`（见下），内容未变 |
| 约 14:37 | 改名：`mv d-heat.json d-heat-items.json` | 仅移动，不删除 | 原因：`barking heat` 拒绝覆盖已有 `d-heat.json`（报错“拒绝覆盖（改用别的组前缀，或先删除）”），为保留两批结果而改名 |
| 约 14:37 | `heat docs/2611/1110 d --hn "Homeopathic Learning" --hn "monitorability chain-of-thought" --hn "OpenRouter MCP CLI skills" --hn "Qwen"` | 前三个关键词无命中；“Qwen”返回 10 条，其中 50449130 “Qwen4-Max is now in Preview” 24 分、1 评 | 写入 `d-heat.json`（当前版本） |
| 约 14:40 | `heat docs/2611/1110 d --item 50449130` | 失败：拒绝覆盖 `d-heat.json` | 改用上面关键词结果中的同一行与 `d-hn-50449130-raw.json` 记录；未再覆盖 |

HN 数字为抓取时刻快照（约 14:32–14:37 BJ），与扫描回贴的数字有 1–2 分的差异，已在 `evidence-D.md` 勘误中列出。

## 失败与未取

- 无 blocked，无 failed。
- 未取的：D7 的 Qwen 官方博客与模型卡（未找到 URL）；D2 的 Anthropic OSS Scanner 官方页（未找到 URL）；D4 的 arXiv PDF（只存摘要页，PDF 仅存本地且本组不需要）。
- X 帖卡片未截图（D 组不截图）。

## 隐私与自检

- 抓取未登录任何账户，未使用浏览器会话。
- 文本存档中的外部链接（LinkedIn 档案、YouTube 频道）为原文公开链接，保留；无维护者账号或抓取者信息。
- `grep` 结果：`d-d2-vulncheck.txt` 含对 curl 维护者 LinkedIn 页的链接（原文公开），未改动。

## 待删除文件

无。
