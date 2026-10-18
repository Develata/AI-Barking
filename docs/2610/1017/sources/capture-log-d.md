# 1017 期 D 组（速览）抓取日志

执行方：claude-haiku-5-5（D 组，派工单 `.handoff/2026-10-17-1017-evidence.md`）。
时区：北京时间（UTC+8）。抓取时段 2026-10-18 19:40–19:49。
工具：仓库自带 `barking fetch`、`barking heat`；搜索用 WebSearch（限定 techcrunch.com 的一次）。未使用 firecrawl（额度已用完）、未截图、未用浏览器、未运行读取本机 Cookie 的工具。D7、D8 复用 1016 存档，未重抓。

## 一、网页与 PDF 存档（barking fetch，日志 `d-fetch-log.jsonl`）

| 北京时间 | 名称 | URL | 结果 | 存档 / 备注 |
|---|---|---|---|---|
| 19:40:55 | d1-lean-blog | https://leodemoura.github.io/blog/2026-10-15-lean-now-ships-with-verified-kernels/ | 200 ok，37958 B，33 行 | `d-d1-lean-blog.*` |
| 19:40:55 | d2-amazon-bee | https://www.aboutamazon.com/news/devices/amazon-personal-ai-bee-frontier-bee-eclipse | 200 ok，221869 B，247 行 | `d-d2-amazon-bee.*` |
| 19:40:55 | d3-apple-news | https://developer.apple.com/news/ | 200 ok，525468 B，2499 行 | `d-d3-apple-news.*`（列表页） |
| 19:40:55 | d4-hunyuan | https://memory.hunyuan.tencent.com/ | 200 ok，168966 B，602 行 | `d-d4-hunyuan.*` |
| 19:40:55 | d5-nvidia-news | https://nvidianews.nvidia.com/news/nvidia-rtx-spark-pcs-available-today-a-new-beginning-for-windows-pcs | 200 ok，83306 B，393 行 | `d-d5-nvidia-news.*` |
| 19:40:55 | d5-nvidia-blog | https://blogs.nvidia.com/blog/local-ai-rtx-spark-windows-laptops-celebration/ | 200 ok，120675 B，196 行 | `d-d5-nvidia-blog.*` |
| 19:40:55 | d9-elorian-glimpse | https://elorian.ai/blog/introducing-glimpse | 200 ok，50315 B，589 行 | `d-d9-elorian-glimpse.*` |
| 19:43:14 | d10-aa-reflection-beam | https://artificialanalysis.ai/articles/reflection-beam | 200 ok，352883 B，152 行 | `d-d10-aa-reflection-beam.*` |
| 19:43:14 | d10-aa-model | https://artificialanalysis.ai/models/reflection-beam | 200 ok，4313936 B，737 行 | `d-d10-aa-model.*`（html 约 4.3 MB，未入库前需确认大小，见下） |
| 19:43:14 | d2-geekwire-bee | https://www.geekwire.com/2026/amazons-new-bee-ai-devices-have-a-bystander-problem-says-former-aws-security-engineer/ | **403 blocked**（5683 B，无 .txt） | 只存了 403 页面的 `.html`、`.headers`，正文未取得；不绕过 |
| 19:43:14 | d3-apple-news-nonconsent | https://developer.apple.com/news/?id=vvzx6cul | 200 ok，118598 B，472 行 | `d-d3-apple-news-nonconsent.*` |
| 19:43:14 | d3-apple-guidelines-news | https://developer.apple.com/news/?id=92th7e5i | 200 ok，109222 B，474 行 | `d-d3-apple-guidelines-news.*` |
| 19:43:14 | d4-hy-memory-github | https://github.com/Tencent-Hunyuan/Hy-Memory | 200 ok，408899 B，795 行 | `d-d4-hy-memory-github.*` |
| 19:43:51 | d6-techcrunch-replit | https://techcrunch.com/2026/10/17/replit-is-no-longer-working-with-anthropic-ceo-says/ | 200 ok，245616 B，355 行 | `d-d6-techcrunch-replit.*` |
| 19:44:52 | d6-replit-docs-model-selector | https://docs.replit.com/features/agent/model-selector | 200 ok，608211 B，756 行 | `d-d6-replit-docs-model-selector.*` |
| 19:44:52 | d6-replit-free-models-blog | https://replit.com/blog/free-the-models | 200 ok，232140 B，451 行 | `d-d6-replit-free-models-blog.*` |
| 19:44:52 | d6-replit-free-mode-blog | https://replit.com/blog/replit-introduces-free-mode | 200 ok，178584 B，348 行 | `d-d6-replit-free-mode-blog.*` |
| 19:46:25 | d4-license-raw | https://raw.githubusercontent.com/Tencent-Hunyuan/Hy-Memory/main/LICENSE | 200 ok，9650 B，61 行 | `d-d4-license-raw.txt`（无 html，仅纯文本） |

清单文件：`d-manifest-1.tsv` 至 `d-manifest-5.tsv`（与上表一一对应，供复跑）。

**未抓取（有意）**：HF 模型页 `huggingface.co/tencent/Hy-Memory-30B-A3B`、论文 `arxiv.org/pdf/2610.22196`（D4 复核未做）；HF `reflection/Beam-1.0-501B-A23B`（D10 只据 AA 页面）；Reddit 全部（见第三节）。

**入库提示**：所有 `.txt`、`.json`、`.md`、`.tsv` 均小于 1 MB（最大 `d-d3-apple-news.txt` 约 149 KB）。`.html`、`.headers` 与 `d-d10-aa-model.html`（约 4.3 MB）属 HTML 原件，按规则只存本地、不入库；`.pdf` 本组无。

## 二、1016 期复用（不重抓）

- D7：`docs/2610/1016/sources/e-e3-google-wetlab.html`、`.txt`（1016 E 组，URL `https://research.google/blog/grounding-ai-agents-in-biological-reality-building-blocks-for-autonomous-wet-labs/`，页面日期 October 16, 2026）。
- D8：`docs/2610/1016/sources/d-waymo-toronto.html`、`.txt`（1016 D 组，URL `https://waymo.com/blog/2026/10/waymo-in-toronto/`，页面日期 October 16, 2026）。

## 三、热度（barking heat，`d-heat.json`、`d-hn-50114758.json`）

取得时刻 2026-10-18 19:49:04（北京）；HN 搜索自 2026-10-11 起。

| 条目 | 查询或 ID | 结果（分 / 评 / 北京时间） |
|---|---|---|
| D1 | HN 50114758（条目 ID） | 152 分 / 50 评 / 10-16 14:16（同时存了嵌套评论 50 条至 `d-hn-50114758.json`） |
| D1 | “Lean kernels verified” | 命中 1 条，即 50114758 |
| D2 | “Bee Frontier Eclipse Amazon” | 50102536（10 / 1 / 10-15 20:12）；50118395（3 / 0 / 10-16 21:51） |
| D3 | “Apple App Review nonconsensual” | 0 条 |
| D4 | “Hy-Memory” | 0 条 |
| D5 | “RTX Spark laptops” | 无 NVIDIA 本稿直接命中；相关 50071267（3 / 1 / 10-13 21:18）、50119594（1 / 0 / 10-16 23:25）、50139919（2 / 0 / 10-18 18:59） |
| D6 | “Replit Anthropic” | 50129206（2 / 1 / 10-17 16:56）；50125160（2 / 0 / 10-17 05:44） |
| D6 | Reddit “Replit Anthropic” | **失败**：返回 old.reddit.com 登录页，无法解析 |
| D7 | “Google Research wet lab agents binders” | 0 条 |
| D8 | “Waymo Toronto”（HN） | 50118981（326 / 443 / 10-16 22:44） |
| D8 | Reddit “Waymo Toronto” | **失败**：登录页 |
| D9 | “Glimpse Elorian” | 50128667（2 / 0 / 10-17 15:04） |
| D10 | “Reflection Beam” | 6 条，含 50120173（1 / 0 / 10-17 00:08，AA 模型页）、50118213（12 / 1 / 10-16 21:34）、50118552（19 / 2 / 10-16 22:05）、50118549（6 / 1 / 10-16 22:05）、50118782（10 / 0 / 10-16 22:28）、50119226（6 / 0 / 10-16 23:01） |

注：D1 线索的 HN 数据为“129/45”，今日取得为 152/50，以今日为准。

## 四、试验输出的处理

- 首次 `barking heat` 调用写出了 `d-heat.json` 与 `d-hn-50114758.json`，随后因“已有输出，拒绝覆盖”报错；这两个文件是本组自己生成的试验输出，用精确路径 `rm` 删除后重跑（未用 `-f` / `-r` / 通配符）。当前 `d-heat.json`、`d-hn-50114758.json` 为重跑结果。
- `barking heat` 的“用法”调用与一次 `cargo run` 不带参数的用法查看，均无文件产出。

## 五、摘句核验

- `d-quotes.tsv`：37 条摘句与存档文件的 grep -F 核对。36 条一次命中；“Glimpse is available through API and our chat UI today.”在原文中被换行切开，已改为不跨行的完整片段，复核后命中。
- 检索摘要（WebSearch 结果）中的句子不作为原文引用，只作线索：包括 GeekWire 批评内容、Reflection 的 v4.3.2、Replit 相关媒体摘要。

## 六、失败与未完成

- GeekWire（D2 补充）：403 blocked，正文未取得，无法引用原句。
- Reddit（D6、D8）：登录页，无数据。
- HF 模型页（D4）与 arXiv 论文（D4）、HF 权重页（D10）：未抓取。
- D6 的 Replit 官方确认原句：未找到（已在 evidence-d.md 记为未找到）。
- 无截图（D 组不截图，按派工单）。
- 本组用时约 9 分钟（19:40–19:49，抓取与热度）；另有核验与写作，未超过 60 分钟目标。
