# 抓取日志：D 组（北京时间）

工具：`barking fetch`（网页存档，清单 `d-list1.tsv`、`d-list2.tsv`、`d-list3.tsv`，日志 `d-fetch-log.jsonl`）、`barking heat`（HN 与 Reddit，结果 `d-heat.json`，HN 条目 `d-hn-50159245.json`、`d-hn-50154540.json`）。未使用 WebFetch、未截图、未登录、未用镜像或转载站。

## 网页存档（`barking fetch`，匿名 GET）

| 时刻（BJ） | 名称 | URL | 结果 | 备注 |
|---|---|---|---|---|
| 19:13:22 | healthbench | https://artificialanalysis.ai/articles/healthbench-professional-health-conversations | 200 ok，348,114 B | D1 |
| 19:13:25 | viggle-jst2 | https://viggle.ai/jst2 | 200 ok，320,302 B | D2 |
| 19:13:22 | minimax-m31f | https://www.minimax.io/blog/minimax-m31f | 200 ok，80,631 B | D3 |
| 19:13:24 | aa-minimax-creator | https://artificialanalysis.ai/models/creators/minimax | 200 ok，578,877 B | D3 |
| 19:13:25 | stanford-hai | https://hai.stanford.edu/news/measuring-what-medical-ai-can-actually-do | 200 ok，310,412 B | D4 |
| 19:13:24 | projectdiscovery | https://projectdiscovery.io/research/how-abliterated-models-can-get-you-pwned | 200 ok，301,316 B | D5 |
| 19:14:45 | arxiv-physicianbench | https://arxiv.org/abs/2605.02240 | 200 ok，44,528 B | D4，摘要 |
| 19:14:45 | arxiv-healthadminbench | https://arxiv.org/abs/2604.09937 | 200 ok，43,930 B | D4，摘要 |
| 19:14:49 | aa-minimax-m31f-model | https://artificialanalysis.ai/models/minimax-m3-1-flash | 200 ok，4,434,270 B | D3，模型页（html 4.4 MB，仅存本地） |
| 19:16:55 | physicianbench-html | https://arxiv.org/html/2605.02240v1 | 200 ok，579,512 B | D4，正文（核对病历数据来源） |

- 全部 10 条 `ok`，`blocked` 0，`failed` 0。每条的 `.txt` 已打开核对为正文。
- 失败的第一次尝试：清单首版因 printf 参数拼接错误，两条记录挤在同一行，`barking fetch` 拒绝执行（退出码 2），未写入任何存档。随后用 heredoc 重写清单再跑。
- 体积：`.html` 原件中 `d-aa-minimax-m31f-model.html` 为 4.4 MB，超过 1 MB；其余 `.html` 均在 0.6 MB 以下。HTML 原件按派工单只存本地，不入库。`.txt` 文件均小于 1 MB（最大 `d-physicianbench-html.txt`，72 KB）。HN 条目 JSON 最大 `d-hn-50154540.json`，117 KB。

## 热度（`barking heat`）

| 时刻（BJ） | 查询 | 结果 |
|---|---|---|
| 19:15:34 | HN：`MiniMax M3.1`、`abliterated`、`HealthBench`（条目 50159245、50154540） | 成功；HN 命中见 `d-heat.json`；条目 JSON 已存 |
| 19:17:24 | HN：`HealthBench`、`Viggle`、`JST-2`、`Measuring What Medical AI`、`HealthAdminBench`、`PhysicianBench`、`MiniMax M3.1`、`abliterated`；条目 50159245、50154540 | 成功（重跑，见下） |
| 19:15:34 | Reddit：`HealthBench`、`abliterated models`、`MiniMax M3.1 Flash`（3 次） | 失败：`old.reddit.com` 重定向到登录页（text/html，约 332 KB），非 JSON。 |
| 19:17:24 | Reddit：`HealthBench`、`MiniMax M3.1 Flash`（2 次） | 失败，同上。合计 5 次请求全部失败，未重试。 |

- 重跑说明：第一次 `heat` 写入 `d-heat.json` 时，HN 条目 JSON 随之生成；随后用 `heat` 补充查询时因“拒绝覆盖”而报错。已用普通 `rm` 精确删除本组自己刚生成的 `d-heat.json`、`d-hn-50159245.json`、`d-hn-50154540.json`（试验文件，无外部内容），再以完整查询列表重跑一次。最终的 `d-heat.json` 与条目 JSON 由 19:17 那次生成。
- HN 命中数（19:17 BJ）：`HealthBench` 1 条（50162598，1 分，10/20 15:39 BJ）；`MiniMax M3.1` 10 条（50154540 为 477 分 / 201 评，条目页根节点 475 分；其余 1–8 分）；`abliterated` 3 条（50159245 为 147 分 / 85 评，10/20 07:08 BJ；50119505 为 2 分，10/16 23:19 BJ；50161098 为 2 分，与本条无关）；`Viggle`、`JST-2`（只命中无关标题）、`Measuring What Medical AI`、`HealthAdminBench`、`PhysicianBench` 无相关命中。

## 本地计算与解析

- `d-hn-50154540.json`、`d-hn-50159245.json`：用 `python -I` 解析嵌套评论（只读，未写入其他文件），取回复数最多的顶层评论。评论接口不含分数，故评论只注 ID 与回复数。
- 未使用其他脚本。

## 不收或未取（按派工单范围）

- 未取：E 组全部；A/B/C 组条目；OpenAI 原始 HealthBench 页；Marble 的测试方法页（无链接）。
- 未收：同一 URL 的 X 帖子（未请求 fxtwitter，因 D 组只要求官方发布时刻与热度，官方页面只给日期）。
- 本组未涉及：WhatsApp、DeepInfer、Mercury、DHS 限制令等（派工单排除项）。

## 入库前处理（Claude，10/20 发布后）

- `d-projectdiscovery.txt`：博文演示用的三段虚构凭据（AWS 示例 Access Key、Stripe `sk_live_` 样式串、AWS 示例 Secret Key）替换为 `[DUMMY-CREDENTIAL-REDACTED]`，避免触发 GitHub secret scanning；原文 HTML（`d-projectdiscovery.html`）不入库，原样存 OpenList。
