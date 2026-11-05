# 1104 期 B 组抓取日志

执行：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8；本机美东 EST + 13 小时），按本机时钟折算，分钟级。批量抓取逐条结果在 `b-fetch-log.jsonl`，截图动作在 `b-shots-log.jsonl`，热度在 `b-heat.json`。清单文件：`b-fetch-list.tsv`、`b-fetch-list2.tsv` … `b-fetch-list7.tsv`（`list` 即第 1 批）。

## 一、barking fetch（批次与结果）

| 批次 | 时刻 | 条目 | 结果 |
|---|---|---|---|
| 1 | 17:21 | HF 模型页、README、LICENSE、config.json、commits 页；Wayback 15:33:01Z 快照（`id_` 原始页）；vLLM 配方页；AA 模型页、AA 榜单页；AMD 技术文章；theainewsreport；r40 PDF；peterstran；Kimi 博客 | 13 条 ok；theainewsreport 协议错误 failed（`curl -L` 单独重试同样无返回），改用 HN 标题与其他稿件 |
| 2 | 17:23 | AA 10-23 发布文、AA 方法页、AA 指数评测页 | 3 条 ok |
| 3 | 17:23 | AA v5 发布文 | ok |
| 4 | 17:24 | IT之家昇腾稿、凤凰网、Pandaily、TechNode、vLLM 博客、orcarouter、daily.dev、同花顺、CNMO | 9 条 ok；Pandaily txt 仅 106 字节（JS 壳，无正文） |
| 5 | 17:24 | vLLM Ascend 文档、SGLang issue 页与 API | 3 条 ok |
| 6 | 17:26 | IT之家 490、新浪转载、什么值得买两篇、掘金、AIDeepThink、Cocoloop、星链 API、腾讯云开发者社区 | ok 5；IT之家 490、什么值得买两篇、星链 API 连接失败（重试一次仍失败，不再试） |
| 7 | 17:28 | 三文阁、gpt68、AILog、cnblogs | 3 条 ok；gpt68 协议错误 |

HTML、PDF 原件只存本地，不入库（Claude 事后传网盘）；单个 txt/json 均小于 1 MB。`.txt` 最大的是 AA 方法页与 AA 榜单页（各数百 KB 内，未超 1 MB）。

## 二、curl、opencli 与其他单次请求

| 时刻 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 17:19 | HF API：`/api/models/moonshotai/Kimi-K3.1-Pro`（含 `?blobs=true`）、`/commits/main`、`/tree/main?recursive=true` | curl | 成功，存 `b-hf-api.json`、`b-hf-api-blobs.json`、`b-hf-commits.json`、`b-hf-tree.json`；提交作者字段含 HF 公开用户名与头像链接（Moonshot 仓库提交者，公开），未处理 |
| 17:19 | HF `tree/main?expand=true`（只在脚本内统计最后提交日期，未存盘） | curl | 首页 50 条，49 条为 11-02T04:13:51Z，1 条为 11-05T07:11:43Z |
| 17:20 | `barking heat`：HN “Kimi K3.1 Pro”、“…weights”、“…Recalls”、“…fake OpenAI ciphertext”；Reddit “Kimi K3.1 Pro” | barking | HN 成功，存 `b-heat.json`；Reddit 登录墙，响应不是 JSON，失败，未绕过 |
| 17:20 | HN 主帖 Algolia items：`hn.algolia.com/api/v1/items/50366999` | curl | 成功，存 `b-hn-50366999.json`（评论无分数字段） |
| 17:20 | Wayback availability 与 CDX（HF 模型页、组织页、API 页） | curl | 模型页自 11-01 起 6 条快照，最早 15:33:01Z；组织页最早 23:14:55Z；API 页无快照 |
| 17:20 | HF 讨论 API，去作者字段后存 `b-hf-discussions.json` | curl + python | 6 条（#2–#7），#1 不在列表 |
| 17:21 | Kimi 官方 X 时间线 | `opencli twitter tweets Kimi_Moonshot --limit 15` | 成功（公开内容，未存盘，结果用于确认帖子 id 与时刻） |
| 17:21 | fxtwitter：2118010528123773297（官方放权重帖）、2113680701770068067（10-23 官方发布帖）、2118025552963493935（AMD day-0 帖） | `api.fxtwitter.com/status/<id>` | 成功，存 `b-fx-<id>.json`（`reposted_by` 为 Kimi 官方账号，公开）；一开始用 Kimi 转发帖 id 2118220088675545317 查到 AMD 内容，已删除该文件并改用 AMD 原帖 id（唯一一次 `rm`，对象是自己刚生成的重复文件） |
| 17:22 | ModelScope API `moonshotai/Kimi-K3.1-Pro` | curl | 成功，存 `b-modelscope-api.json`；`CreatedTime` 1793808159 |
| 17:25 | 搜索（WebSearch）：昇腾适配、开源第一/46.5、MIT/3 万亿、全球第四 | WebSearch | 仅用于找稿件；结论均回到存档原文核对 |
| 17:45 | `opencli weixin search "昇腾 0 Day 支持 Kimi K3.1 Pro"` | opencli | 取到 10 条标题与摘要，存 `b-weixin-search-titles.txt`（跳转链接含 token，未存）；原文未点开（1103 期同类 10 条全遇验证码页，未绕过）；**华为计算公众号原文失败** |
| 17:42 | AA 模型页复查 | curl | `isOpenWeights:false`、“proprietary”、中位数 12.5 与 17:21 相同（未存盘） |

## 三、截图（`docs/2611/1104/images/`，B 组区间 13–24）

headless Chrome（`barking shot render --width 700`，DPR 2，宽 1400 px），匿名无登录。AA 页面 17:2x 渲染，浅色底；HF 页浅色底。

| 文件 | 来源 | 工具 | 备注 |
|---|---|---|---|
| `13-hf-model-head.png` | HF 模型页 | render + cropbox | 标题、标签、License: kimi-k3.1-pro、Files / Community 标签；窄屏布局；Like 676 |
| `14-modelcard-intro.png` | 同上 | render（高 4200）+ cropbox | “1. Model Introduction”，含 3T-class |
| `14a-modelcard-summary.png` | 同上 | 同上 | “2. Model Summary” 整表与脚注（2.8T、104B、93 层、896 专家、1048576、MXFP4） |
| `15a-hf-license-text.png` | HF `raw/main/LICENSE` | render + cropbox | 52 行全文；等宽字体，无 HF 页框 |
| `15-hf-license.png` | HF `blob/main/LICENSE` | render + cropbox | **弃用**：窄屏下右侧代码行被截断，保留文件待删除 |
| `16-hf-files-size.png` | HF `tree/main` | render + stack | 路径栏 1.56 TB、2 commits、文件列表；两段拼接（中间去掉了含提交作者的一行）；首片 2.34 GB |
| `17-vllm-recipe-hardware.png` | vLLM 配方页 | render + stack | 页首摘要与标签 + 硬件/变体配置器两段拼接，间隔 24 px，背景白 |
| `18-aa-model-summary.png` | AA 模型页 | render + cropbox | “Proprietary model”、#22/68、46、36.0 分钟、#61/64 等四个磁贴 |
| `18a-aa-model-comparison.png` | 同上 | 同上 | “Comparison Summary”（median 37、median 12.5 minutes）与 Technical specifications |
| `19-aa-faq-open-weights-best.png` | AA 榜单页 FAQ | opencli（zoom 2，视口 1256×698）+ cropbox | 点开手风琴“Which is the best open weights model…”，裁掉浮动表头；分辨率为页面 CSS zoom 2 的视口截图，宽 1236 px |
| `20-aa-v5-open-weights-gap.png` | AA v5 发布文 | render + cropbox | Key results 三条 |
| `21-x-weights-post.png` | X 官方嵌入卡（platform.twitter.com/embed，加 `--virtual-time-budget=10000` 才渲染出内容） | render + cropbox | 只含帖子卡片，匿名；卡上 “11:00 AM · Nov 4, 2026” 是本机美东时区显示（= 16:00 UTC）；点赞 7.2K 为截图时 |
| `22-hn-main-title.png` | HN 主帖 | render + cropbox | 标题、523 分、146 评、17 hours ago；未登录 |
| `23a-cocoloop-5-6tb-title.png` | Cocoloop | render + stack | 标题 + 正文两段拼接，中间为 24 px 底色条 |
| `23-cocoloop-5-6tb-title.png` | 同上 | 同上 | **弃用**：第二段首行被切半，保留待删除 |
| `24-kimi-blog-weights-promise.png` | Kimi 博客 Availability 段 | opencli（zoom 2，视口 1256×698，强制 opacity 1 以显示渐显文字）+ cropbox | 含 “Full model weights will be released by Nov 4, 2026.”；背景为灰色，宽 1236 px |

没有截图的候选项：AA 耗时图（AA 模型页“Time per Task”图为 “No publicly available API” 空图，耗时数字在 18、18a 里）；Kimi 10-23 官方 X 卡（嵌入卡折叠了 “Open weights by Nov 4, 2026.” 一句，原文在 JSON）；HF 模型页的 Wayback 快照（只存文本）。

## 四、使用到的假设与限制

- 北京时间 = UTC + 8；X 的 `created_at` 为 UTC，换算后写北京时间。
- “最早公开证据”只取“能被匿名访问的页面或帖子的时间戳”，不推断 HF 可见性切换时刻。
- 对比图与页面数字均为目视读取或页面文本，未做 OCR 校对；截图均已逐张打开确认文字清楚。
- 浏览器未登录；没有输入凭据；没有对 X、HN、HF 点赞、评论、关注。AA FAQ 手风琴与 Kimi 博客的渐显文字是页面 UI 操作，不涉及账号。
- 存档中含他人公开信息：HF 提交者用户名（`b-hf-commits.json`）、GitHub issue 作者名（`b-sglang-issue-api.json`）、HN 评论者名（`b-hn-50366999.json`）；均为其公开发言，未收集其他个人信息。
