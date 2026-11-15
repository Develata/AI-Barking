# 1114 期 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区），取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl` 与终端时间，日期 2026-11-15。文件前缀 `b-`。`opencli doctor` 由派工方确认过。未登录任何账户、未提交表单、未发帖点赞。HTML/headers 原件不入库（只存本地）；`.txt`、`.json`、`b-repo/` 文本可入库。

## 抓取

| 时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 20:54 | https://deepnote.com/research/benchmark | `barking fetch` | ok，885 KB html，2108 行 txt；响应头 Date: Fri, 13 Nov 2026 15:29:50 GMT（疑为缓存渲染时间，非发布时间）；正文完整（榜单、方法、表、局限都在） |
| 20:54 | https://github.com/deepnote/agent-initiative-benchmark | `barking fetch` | ok（仓库首页） |
| 20:54 | https://huggingface.co/datasets/deepnote/pagemantle-company-dataset | `barking fetch` | ok |
| 20:55 | https://api.github.com/repos/deepnote/agent-initiative-benchmark/commits、`/git/trees/main?recursive=1`、`/commits/main` | curl（`--max-time 60`，匿名） | ok；结果用于核对提交时间与文件树（树 JSON 在临时目录，未入库） |
| 20:56–20:58 | https://raw.githubusercontent.com/deepnote/agent-initiative-benchmark/main/<各文件> | curl（`--max-time 60`，循环），共 103 个文本文件到 `b-repo/`（README、docs、src 的 py 与评审/agent 提示词、8 个 task 的 config 与 package JSON、LICENSE 等；已跳过 `assets/*.webp`（单个 6–11 MB）、`requirements.lock`、`prompts/tools/`、`prompts/guides/`） | ok；全部小于 1 MB（`b-repo/` 合计约 2.3 MB，182 个文件，含 `b-repo/tasks/` 的 JSON，入库前请决定是否保留 tasks 目录） |
| 20:58 | HN 搜索（`barking heat`） | `barking heat docs/2611/1114 b --hn … --reddit …` | HN ok（`b-heat.json`）；Reddit 两次失败：返回登录页（`old.reddit.com/login`），记入 errors |
| 20:59 | HN item 50485628 | algolia API（curl） | ok：1 条评论，内容已摘入 evidence；`barking heat --item` 因 `b-heat.json` 已存在拒绝覆盖，未重跑，评论文本直接记在 evidence 里 |
| 20:59 | 新智元稿：163.com 网易号、finance.sina.com.cn、vibecafe.ai、daily.dev、dev.to、casrai.org | `barking fetch`（`b-zh-163`、`b-zh-sina`、`b-vibecafe`、`b-dailydev`、`b-devto`、`b-casrai`） | 全部 ok；打开 `.txt` 确认是正文（daily.dev 为聚合页，评论内容未取到） |
| 21:00 | https://api.github.com 之外：https://api.fxtwitter.com/status/2121256499552915586、…/2121259512414032320 | curl（`--max-time 30`） | ok；存 `b-x-<id>.json`（官方公开 JSON，含作者公开资料与互动数，不含抓取者信息） |
| 21:03 起 | https://deepnote.com/research/benchmark | opencli browser `--session b-shot --window background`，`eval` 设 zoom 1.3（表与成本段用 1.0），`screenshot --width 1400` | 见下“截图”；CSS zoom 2 时图表被截（轴只到 4），故改 1.3；opencli 的 `scrollIntoView` 因页面 smooth scroll 不生效，改 `scrollBy` |
| 21:13 | https://raw.githubusercontent.com/deepnote/agent-initiative-benchmark/4d66be81b4fc44ba0fa0fc48320be3ebfa717e3e/src/benchmark/prompts/judge/evaluate_agent_run.txt | opencli browser | 首次 `open` 因我写错 sha 超时（115 s CDP timeout），改完整 sha 后成功 |
| 21:15 | https://www.163.com/dy/article/LBN9SFNU0511ABV6.html | opencli browser | ok；页面跟贴数显示 2（20:59 存档为 0） |
| 21:17 | https://platform.twitter.com/embed/Tweet.html?id=2121256499552915586 | 先 `barking shot render`（headless Chrome）：出图全白，不可用；改 opencli browser 渲染成功 | 只裁帖子卡片 |

## 失败 / 未取到

- Reddit：`barking heat` 两次返回登录页；`opencli reddit search` 被 “blocked by network security” 403（SECURITY_BLOCK）。不再重试。
- 新智元微信公众号原文：搜索（两次）未返回 mp.weixin.qq.com；以网易号版与新浪转载为准。
- 同花顺 / 量子位 / 机器之心 / 36氪转载：搜索未找到。
- 逐题结果（600 次运行）：公开仓库与 HF 数据集均无。
- 创始人 X 帖的 132 条回复：未采集。
- 阅读数：163、新浪页面均不显示。

## 截图（opencli browser，匿名、无登录栏；用 `barking shot cropbox|stack` 只裁切不改像素；日志见 `b-shots-log.jsonl`）

| 文件 | 来源页 | 方式/缩放 | 裁掉的东西 |
|---|---|---|---|
| 13–18、20、25、26 | deepnote.com/research/benchmark | zoom 1.3，宽 1400 px（约 1.3 倍像素；zoom 2 会让图表错位） | 站点固定导航条、浏览器翻译插件图标（右侧浮标，部分图仍在右缘，位于图外空白区） |
| 19、21 | 同上 | zoom 1.0，宽 1400 px（1x 像素；表格在 1.3 倍时右侧列被截） | 固定导航条 |
| 22、27 | GitHub raw 视图（深色底） | opencli，宽 1400 px | 无；27 取代 22 |
| 23、28、29 | 163.com 网易号 | zoom 1.3 | 广告浮窗、侧栏；23 为两段拼接，底色深灰 |
| 24 | X 官方嵌入页 | opencli，宽 1400 px，再裁 716×1168 | 只含帖子卡片 |

目视确认：裁后成品已逐张打开的有 19、20、22、23、25、27；其余（13、14、15、16、17、18、21、24、26、28、29）打开的是裁前整图，裁切框按整图坐标取在文字行之间，裁后未逐张重开，Claude 终审时请扫一眼。24 的裁切边界按整图（716×1168）取，卡片外为空白。

## 假设

- 把 `b-fetch-log.jsonl` 的“ok”仅当“拿到文件”，正文均已打开核对。
- 创始人身份取自官方帖子作者与搜索摘要，没有单独核对 Deepnote 官网 about 页。
- “超时都没发消息”取自官方句 “reached the 180-second limit before any message was sent”。

## 待删除文件（精确路径，由 Claude 收尾一次删除）

- /Users/develata/gitclone/AI-Barking/docs/2611/1114/images/22-repo-judge-prompt.png（被 27 取代，第 7 项开头被裁掉半行）
