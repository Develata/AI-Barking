# 1015 期 D 组（速览：产品与公司）抓取日志

执行方：claude-haiku-5-5（D 组，派工单 `.handoff/2026-10-15-1015-evidence.md`）。
时区：北京时间（UTC+8）；本机 EDT 与 UTC 时刻已换算。抓取批次 2026-10-16 19:41–20:03。

## 抓取记录（barking fetch，正式批次）

正式批次为 2026-10-16 19:59:11 起的一次 `barking fetch docs/2610/1015 d <清单>`（清单存于 scratchpad，不入库），并发 6，同一域名至多 2。结果日志 `d-fetch-log.jsonl`。

| 北京时间 | 名称 | URL | 结果 | 存档 |
|---|---|---|---|---|
| 19:59 | d1-anthropic | https://www.anthropic.com/engineering/redesigning-source-control | 200，164787 B，ok | `d-d1-anthropic.html`、`.txt`、`.headers` |
| 19:59 | d1-sunshowers | https://sunshowers.io/posts/anthropic-mononoke-2/ | 200，17663 B，ok | `d-d1-sunshowers.*` |
| 19:59 | d2-ithome | https://www.ithome.com/1/013/848.htm | 200，23665 B，ok | `d-d2-ithome.*` |
| 19:59 | d2-yicai | https://m.yicai.com/news/103395363.html | 200，42108 B，ok | `d-d2-yicai.*` |
| 19:59 | d3-fastmode | https://platform.claude.com/docs/en/build-with-claude/fast-mode | 200，675591 B，ok | `d-d3-fastmode.*` |
| 19:59 | d3-pricing | https://platform.claude.com/docs/en/about-claude/pricing | 200，1193388 B，ok | `d-d3-pricing.*` |
| 19:59 | d4-index | https://alignment.openai.com/misalignment-reports/ | 200，60249 B，ok | `d-d4-index.*` |
| 19:59 | d5-0539edu | http://www.0539edu.com/news/5690762/（转载/投稿页，非首发） | 200，49876 B，ok | `d-d5-0539edu.*`（页内含仓库镜像链接，未跟随） |
| 19:59 | d5-runtimewire | https://runtimewire.com/article/tencent-workbuddy-claude-code-internal-text-allegation | 200，63839 B，ok | `d-d5-runtimewire.*` |
| 19:59 | d6-swarmcha | https://swarmcha.se/posts/chinese-agent-fleet-full-report | 200，225368 B，ok | `d-d6-swarmcha.*` |
| 20:01 | d4-wallet | https://alignment.openai.com/misalignment-reports/using-a-funded-test-cryptocurrency-wallet-without-authorization/ | 200，24096 B，ok（第二批，清单另写） | `d-d4-wallet.*` |
| 20:01 | d4-dashboard | https://alignment.openai.com/misalignment-reports/accessing-and-changing-internal-systems-during-dashboard-research/ | 200，49757 B，ok | `d-d4-dashboard.*` |
| 20:01 | d4-stats-secrets | https://alignment.openai.com/misalignment-reports/retrieving-server-secrets-and-bypassing-authorization-on-a-government-statistics-website/ | 200，35447 B，ok | `d-d4-stats-secrets.*` |
| 20:01 | d4-external-browser | https://alignment.openai.com/misalignment-reports/circumventing-internet-restrictions-through-an-external-browser/ | 200，20901 B，ok | `d-d4-external-browser.*` |

以上共 14 条 ok，0 条 blocked，0 条 failed。`.txt` 均已打开核对为正文（D6 报告 2912 行，D4 报告 111–281 行）。

## curl 单次请求（fxtwitter JSON，公开 API）

| 北京时间 | URL | 结果 | 存档 |
|---|---|---|---|
| 20:00 | https://api.fxtwitter.com/status/2110825294655562048（ClaudeDevs 推文） | 200，JSON（code 200） | `d-d3-tweet.json` |
| 20:00 | https://api.fxtwitter.com/status/2110953344579846187（0xLogicrw 转述帖） | 200，JSON（code 200） | `d-d5-xrepost.json` |

JSON 中的头像链接为帖文作者（ClaudeDevs、0xLogicrw）的公开资料，不含抓取者信息；`grep` 确认无 develata、无抓取者昵称。

## 首次批次的误存与清理

- 首次 `barking fetch` 时清单写的是 `d-d1-anthropic` 等带 `d-` 前缀的名称，工具又自动加 `d-`，产出 `d-d-*` 双前缀文件（约 30 个，含 `.html`、`.txt`、`.headers`）与 `d-fetch-log.jsonl`，时刻 19:41:33–35。
- 处理：这些是本组自己生成的文件。已删除 `docs/2610/1015/sources/d-d-*` 与 `d-fetch-log.jsonl`，清单去掉前缀后重跑（见上表 19:59 批次）。删除的仅为本组误生成的文件，未涉及其他组。

## 搜索（仅作线索，结果未存档）

| 北京时间 | 工具 | 查询 | 结果 | 用途 |
|---|---|---|---|---|
| 20:00 | WebSearch | 腾讯 WorkBuddy CodeBuddy DeepSeek 故障 积分 返还 国内版 手动领取 公告 | 无官方公告命中；命中 IT之家 2026-03 旧闻（服务不稳定，无补偿）与 DeepSeek 6 月自动返还旧闻 | D2 一手来源，未找到 |
| 20:01 | WebSearch | WorkBuddy Claude Code 494 字符 逐字节 张小明 csdn OR juejin OR zhihu | 无命中 | D5 首发原站，未找到 |
| 20:02 | WebSearch | New York Times Dylan Freedman Tencent agent fleet Amap Swarm Chasers | 无 NYT 报道命中；其他媒体报道（aiweekly、runtimewire 等）仅作背景 | D6 NYT，未找到 |

未使用 firecrawl 检索（额度有限，本组以 curl 与 WebSearch 为主）。

## 热度（barking heat，一次运行）

命令：`barking heat docs/2610/1015 d --hn ×6 --item 50107559 --reddit ×2`，20:02:26 取得；HN 搜索窗口自 2026-10-09 20:02 起。写入 `d-heat.json`（本组生成，未删除）。

| HN 查询 | 命中 | 结果 |
|---|---|---|
| Scion Anthropic Mononoke | 2 | 50106738：244 分 / 79 评（10-16 01:05，“More on Anthropic's Scion ripping off Mononoke”）；50094694：420 分 / 157 评（10-15 04:30，“Anthropic's Scion is heavily inspired by Mononoke”） |
| WorkBuddy Claude Code | 0 | — |
| Opus 5.5 fast mode | 0 | — |
| OpenAI misalignment reports | 1 | 50116522：2 分 / 0 评（10-16 18:14，钱包报告页） |
| Tencent agent fleet Amap | 0 | — |
| DeepSeek outage refund Tencent | 0 | — |
| 条目 50107559 | 取到 | 员工回复所在条目，嵌套评论 35 条，写入 `d-hn-50107559.json`；HN 员工评论 nickg-anthropic 三条（01:59、02:14、02:36 北京） |

Reddit 两条搜索（Anthropic Scion、WorkBuddy Claude Code）均失败：`old.reddit.com` 重定向至登录页，响应非 JSON（与 1014 期相同）。未重试。

## 其他动作

- 未访问 0539edu 文章中提到的源码仓库、镜像（Gitee 等）或任何软件包；未下载、未解包、未比对。
- 未访问 NYT 相关 URL（未取得 URL）；未访问腾讯官网、公众号、CodeBuddy 官方文档（未取得准确 URL，搜索无命中）。
- 未登录、未使用浏览器会话、未截图（派工单：D 组不截图）。
- 误操作：在收尾阶段误调用一次 `spawn_task` 生成了占位标题的任务芯片（task_82797ca7），已立即调用 `dismiss_task` 撤回，撤回结果显示该芯片不再对用户可见。

## 本组新增文件

- `docs/2610/1015/sources/evidence-d.md`
- `docs/2610/1015/sources/capture-log-d.md`
- `docs/2610/1015/sources/d-d1-anthropic.{html,txt,headers}`、`d-d1-sunshowers.{html,txt,headers}`
- `d-d2-ithome.*`、`d-d2-yicai.*`
- `d-d3-fastmode.*`、`d-d3-pricing.*`、`d-d3-tweet.json`
- `d-d4-index.*`、`d-d4-wallet.*`、`d-d4-dashboard.*`、`d-d4-stats-secrets.*`、`d-d4-external-browser.*`
- `d-d5-0539edu.*`、`d-d5-runtimewire.*`、`d-d5-xrepost.json`
- `d-d6-swarmcha.*`
- `d-hn-50107559.json`、`d-heat.json`、`d-fetch-log.jsonl`

## 大文件

`d-d3-pricing.html` 约 1.2 MB、`d-d3-fastmode.html` 约 0.68 MB、`d-d6-swarmcha.html` 约 0.23 MB。仅 `.html` 原件超过 1 MB 的有 `d-d3-pricing.html`，按派工单“只存本地、不入库”的 HTML 原件规则，由 Claude 事后处理；`.txt` 文本存档均小于 1 MB。
