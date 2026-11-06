# B 组抓取与截图日志（1105 期）

时间均为北京时间（UTC+8）；本机为美东，北京时间 = 本机 + 13 小时。执行方 claude-sonnet-5-5。

## 抓取（`barking fetch`，curl 式匿名请求，结果记入 `b-fetch-log.jsonl`）

| 时刻 | 名称 / URL | 工具 | 结果 |
|---|---|---|---|
| 19:32 | index `https://alignment.openai.com/misalignment-reports/` | barking fetch | 200，ok，69288 字节，314 行文本 |
| 19:32 | filename-report `.../trying-to-prompt-inject-the-grader-in-filenames/` | barking fetch | 200，ok，15161 字节，115 行 |
| 19:32 | network-report `.../bypassing-network-restrictions-during-a-coding-evaluation/` | barking fetch | 200，ok，19821 字节，165 行 |
| 19:32 | securing-env `https://openai.com/index/securing-our-frontier-model-training-environment` | barking fetch | 200，ok，497840 字节，368 行（openai.com 主站这次没有 403） |
| 19:39 | dns-report `.../an-agent-used-dns-to-reach-an-external-chatbot/` | barking fetch | 200，ok，56335 字节，247 行（用于 B7 对照） |
| 19:40 | media-securityweek `https://www.securityweek.com/openai-model-wrote-its-own-network-code-to-get-around-sandbox-restrictions/` | barking fetch | 200，ok |
| 19:40 | media-cloudlink `https://www.cloudlinktech.com/news/openai-model-tried-to-bypass-sandbox-network-controls/` | barking fetch | 200，ok |
| 19:40 | media-agendum-119 `https://github.com/geekjourneyx/agendum/issues/119` | barking fetch | 200，ok（匿名，页面不含抓取者信息） |
| 19:40 | media-agendum-120 `https://github.com/geekjourneyx/agendum/issues/120` | barking fetch | 200，ok |

清单文件：`b-list.tsv`、`b-list2.tsv`、`b-list3.tsv`。`.headers` 文件里只存了少量响应头（工具自己省略其余）。

## 其他请求

| 时刻 | 请求 | 工具 | 结果 |
|---|---|---|---|
| 19:39 | HN 搜索 “alignment.openai.com misalignment-reports” “OpenAI grader filenames” “OpenAI bypassing network restrictions”（自 2026-10-30 起） | barking heat → `b-heat.json` | 第一组命中 1 条（Covert underperformance，329 分 / 179 评，另一份报告）；后两组 0 条 |
| 19:39 | Reddit 两次搜索 | barking heat | 失败：被重定向到 old.reddit.com 登录页（响应不是 JSON），按规则不登录，记失败 |
| 19:3x | HN Algolia 直接按 URL 过滤 `alignment.openai.com`、两份报告路径（curl，`--max-time 40`） | curl | 两份报告路径无结果；域名下只有 Covert underperformance（11/5）等其他页面。另一次带时间过滤的查询返回非 JSON，未重试 |
| 19:4x | 网页搜索 5 次（英文、中文，含 “instructor annotation”“满分”“编造客户评价” 等） | WebSearch | 取得 SecurityWeek、CloudLink、agendum 两个 Issue 作媒体样本；未见其他中文媒体稿 |
| 19:4x | AIHOT `https://aihot.news/`（curl，`--max-time 30`） | curl | 失败（连接无响应，状态码 000），未重试，改记为线索 |
| 19:4x | opencli doctor | opencli | 通过（派工时已通过，本次未重跑，仅在开始截图前确认 daemon 与扩展均连接） |

## 截图（opencli 浏览器 → `barking shot cropbox`，记入 `b-shots-log.jsonl`）

页面处理：`opencli browser --session b-shot open <url> --window background`，页面缩放设为 2（索引页 1.2，原因：5 列表格在缩放 2 时右侧列被截断），隐藏浏览器翻译插件的悬浮图标元素（`#immersive-translate-popup`，非页面内容），`screenshot --width 1400 --full-page` 得到整页中间图（放在本会话临时目录，不入仓库：fn-full 1400×8263，net-full 1400×10331，idx12 1400×3133），再用 `cropbox` 裁出正文栏 x 430–1340。页面为浅色底。

| 文件 | 内容 | 像素 |
|---|---|---|
| images/13-fn-summary.png | FN 标题、模型与日期、Summary | 910×815 |
| images/14-fn-task-cot.png | FN 任务、用户提示、两段 CoT | 910×1120 |
| images/15-fn-rename-calls.png | FN 改名、第一次失败（File name too long）、重试命令及结果 | 910×2360 |
| images/16-fn-graders-1-2.png | FN 三个评分器总述、第一与第二评分器 | 910×1470 |
| images/17-fn-grader-3-response.png | FN 第三评分器与 Investigation and response | 910×1670 |
| images/18-net-summary.png | NET 标题、模型与日期、Summary | 910×1145 |
| images/19-net-what-happened-dns.png | NET 任务、扫网络、DNS 查询与结果 | 910×1805 |
| images/20-net-tls-fragments-s3.png | NET 用户态 TCP/TLS、分片、S3 CONNECT 与 400 | 910×2512 |
| images/21-net-five-hours-creds.png | NET 5 小时摘要、S3 存档重定向、云凭证 | 910×1308 |
| images/22-net-ssh-testimonials-nomention.png | NET 微软登录服务、SSH 问候、假评价、14 小时、“No need mention network issue.” | 910×1855 |
| images/23-net-investigation.png | NET Investigation and response | 910×880 |
| images/24-index-rows.png | 索引页 Reports 表头与前九行（含 NET、FN 两行） | 1330×885（缩放 1.2，不是 2） |

重做记录：`20`、`21`、`22` 第一次裁切底边切掉了末个代码框的下边框，已换成加长的框重裁（`b-shots-log.jsonl` 里有两版记录，以后一版为准；第一版文件已用 `mv` 移出仓库，放在会话 scratchpad，不是用 `rm`）。

## 脚本与命令

- B6 间隔表：Python 标准库一次性脚本（`python3 -I`，读 `b-index.html` 的 `tbody.report-entry` 上的 data-incident、data-created 属性及可见单元格），输出 `b-index-lag.tsv`；脚本未单独存文件。
- 无自写入库脚本。

## 待删除文件

无（没有调用 `rm`；两版重裁的旧截图已用 mv 移到会话 scratchpad（/tmp/claude-1000/…/scratchpad/old20.png 等），不在仓库内）。
