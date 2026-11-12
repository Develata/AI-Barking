# 抓取日志：D 组（速览，1111 期）

时间为北京时间（UTC+8）。带秒的时刻取自工具输出（`barking fetch` / `barking heat` 的开始时间）；未带秒的为相对顺序的约数，按事件先后排列，不代表精确时刻。逐条网络记录同时在 `d-fetch-log.jsonl`。

| 时间（北京） | 工具 | 目标 | 结果 |
|---|---|---|---|
| 17:53:12 | `barking fetch` 批次 1（10 条，清单 `d-list1.tsv`） | Google 博客 Decisions；Amazon Ring；Ring 产品页；DeepMind 博客；Nature 新闻；OpenAI 速览页；AFT 新闻稿；Reuters TSMC；Anthropic Worse Together 博客；arXiv 摘要页 | 9 条 ok；Reuters 401 Unauthorized，记 blocked，无 txt |
| 17:54:11 | `barking fetch` 批次 2（4 条，清单 `d-list2.tsv`） | OpenAI × Upwork 全文 PDF；OpenAI 要点 PDF；AI Studio Decisions 文档；Reuters 另一地址 | PDF 两份 ok；AI Studio 文档正文过短（JS 壳），blocked；Reuters 另一地址 401，blocked |
| 17:54:39 | `barking fetch` 批次 3（2 条，清单 `d-list3.tsv`） | AFT 签署版 MOA PDF；微软 AI 安全与隐私标准事实单 PDF（未审阅，本次未引用） | 两份 ok（MOA 文本 1515 行） |
| 17:54:55 | `barking fetch` 批次 4（2 条，清单 `d-list4.tsv`） | arXiv HTML v1；MAMUBench GitHub 仓库 | 两份 ok |
| 17:55:25 | `barking heat`（`--item 50456791`；`--hn` 7 个查询） | HN：Gemini Decisions API、Ring Drone Cam、AlphaGenome 2、OpenAI Upwork freelance、AFT Anthropic binding agreement schools、TSMC Anthropic OpenAI investment、Worse Together multi-user multi-agent | HN 7 组，命中 2、12、1、0、0、1、1 条；条目 50456791 含 130 条嵌套评论，存 `d-hn-50456791.json`；0 个请求失败；Reddit 未采样 |
| 17:55:37 | `barking fetch` 批次 5（2 条，清单 `d-list5.tsv`） | TSMC 官方新闻稿 pr.tsmc.com/english/news/3352；AI Studio 文档（与 17:54:11 同一地址，第二次） | TSMC 403 Cloudflare 质询，blocked；AI Studio 同批次 blocked。第二次抓取的 `d-hn-gemini-free-aistudio.*` 与首次的同 URL 重复，见待删除清单 |
| 约 17:55:40–17:55:55 | WebSearch（两次，TSMC 与 EdWeek） | “TSMC board approves ... Anthropic OpenAI”；“EdWeek AFT Anthropic legally binding ... schools” | TSMC 搜索返回 SEC 6-K 链接，是后续抓取的依据；摘要为模型生成，只作线索。EdWeek 搜索返回 2026-09 的微软与 AFT 相关报道，未找到 11 月的 EdWeek 报道 |
| 17:55:59 | `barking fetch` 1 条（清单 `d-list6.tsv`） | SEC EDGAR 台积电 6-K（董事会决议） | 403 Forbidden，blocked；改用下一行的 opencli 读取页面 |
| 约 17:56–17:58 | `opencli browser`（session `d-tsmc`）open + eval | SEC 6-K 页面，`document.body.innerText` | 成功；全文存档 `d-tsmc-sec-6k-board-opencli.txt`（已删去含 `@` 的行，仅为 SEC 联系行）。决议 1–5 完整可核 |
| 约 17:58 | `opencli browser`（session `d-gem`）open + eval | AI Studio Decisions 文档 | 成功；存档 `d-gemini-decisions-docs-opencli.txt`（已删去账号邮箱行）。首次 `main` 选择器为空，改用 `document.body` |
| 约 17:58 | `opencli browser`（session `d-gem`）open + eval | Gemini Decision 1 模型页 https://ai.google.dev/gemini-api/docs/models/gemini-decision-1 | 成功；存档 `d-gemini-decision-1-model-opencli.txt`（已删去账号邮箱行）。页面无基准表，无 92.2 / 0.0250 字样 |
| 约 17:59 | `opencli browser`（session `d-gem`）open + eval | Gemini Decisions 博客文章（article 元素） | 成功；存档 `d-gemini-blog-opencli.txt`（已删去账号邮箱行）。博客 HTML 另存 `d-gemini-blog.html`，经 python 检索无 92.2 与 0.0250 |
| 约 17:59 | `opencli browser`（session `d-gem`）eval（free 关键词过滤） | 定价页文字 | 只见通用免费层描述，未见 gemini-decision-1 条目的价格数值或免费说明 |
| 约 17:59 | `opencli browser`（session `d-gem`）open + eval | Gemini API 定价页 https://ai.google.dev/gemini-api/docs/pricing | 成功；存档 `d-gemini-pricing-opencli.txt`（已删去账号邮箱行）。Decision 条目仅列 “Standard”，价格数值未出现在页面文本中 |
| 约 18:00 | WebSearch | “Gemini Decision 1 model page benchmarks ECE ...” | 未找到 Google 一手模型页、模型卡或“9 项第一 / 92.2% / ECE 0.0250”的出处（摘要为第三方聚合站线索） |
| 约 18:02 | 本地脚本式清理（sed 与 python 单行） | `d-aft-agreement-pdf.txt`、`d-aft-press.txt`、`d-aft-press.html`、`d-worse-together-arxiv-html.txt`、`d-worse-together-arxiv-html.html`、`d-gemini-*-opencli.txt` | 邮箱替换为“[邮箱已隐去]”；协议文本中两处签名证书 IP 替换为“[IP已隐去]”；opencli 存档中含 `@` 的行删去。HTML 中机构通用地址（Springer 版权、example.com、SEC opendata）保留 |
| 约 18:04 | `opencli browser close`（`d-tsmc`、`d-gem`） | 两个 opencli 会话 | 已关闭 |

## 失败与未取得正文

- Reuters 报道（`reuters-tsmc`，及 `tsmc-reuters-alt` 的另一地址）：401 Unauthorized，未取得正文，仅存 headers 与 html 原件。
- TSMC 新闻稿（pr.tsmc.com/english/news/3352）：403 Cloudflare 质询，未取得正文。6-K 为同源备案，已取得原文，不影响 D6。
- SEC 6-K 直接 fetch：403；以 opencli 读取页面文本为准。
- AI Studio Decisions 文档（fetch 工具）：JS 壳，正文过短；以 opencli 读取。
- Gemini 定价页 Decision 条目的价格数值：未从页面文本读出，未核实；价格以博客原文为准。
- Nature 新闻：注册墙截断，仅前半段可读，全文未取得。
- 未采样 Reddit（`barking heat --reddit` 未运行）；HN 查询 OpenAI Upwork 与 AFT 两组为 0 命中。

## sha256 说明

`d-fetch-log.jsonl` 中的 sha256 为抓取时的原始响应体。`.txt` 是由原始体派生的，本来就不会与之相等，这里不计。真正被改动的只有两份 HTML 原件：`d-aft-press.html` 与 `d-worse-together-arxiv-html.html`（邮箱替换），它们的 sha256 与日志不再一致；原件未另存。
