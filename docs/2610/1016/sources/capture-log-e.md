# 1016 期 E 组抓取日志

时区：北京时间（UTC+8）。工具：`barking fetch`（网页与 PDF 存档）、`barking heat`（HN / Reddit 热度）、`e-verify-quotes.py`（摘句核对，本组自写，仅读取 `e-*.txt`）。未截图，未使用 opencli、firecrawl、WebFetch、curl、headless Chrome；未登录，未访问付费墙与镜像站，未用 web.archive.org。未读取任何浏览器 Cookie。

## 一、网页与 PDF 存档（barking fetch）

| 清单 | 条目 | 开始（北京时间） | 结果 | 存档 |
|---|---|---|---|---|
| `sources/e-manifest.tsv` | e1-quantum（https://www.anthropic.com/research/designing-efficient-quantum-circuits） | 2026-10-17 20:49:28 | 200 ok，181,462 字节 | e-e1-quantum.html / .txt / .headers |
| 同上 | e1-arxiv-abs（https://arxiv.org/abs/2610.22367） | 同上 | 200 ok，42,302 字节 | e-e1-arxiv-abs.html / .txt / .headers |
| 同上 | e2-expectations（https://www.anthropic.com/research/ai-expectations-gap） | 同上 | 200 ok，205,306 字节 | e-e2-expectations.html / .txt / .headers |
| 同上 | e3-google-wetlab（https://research.google/blog/grounding-ai-agents-in-biological-reality-building-blocks-for-autonomous-wet-labs/） | 同上 | 200 ok，190,777 字节 | e-e3-google-wetlab.html / .txt / .headers |
| 同上 | e4-starskirmish（https://starskirmish.com/bench/v0-2/） | 同上 | 200 ok，21,944 字节 | e-e4-starskirmish.html / .txt / .headers |
| `sources/e-manifest-2.tsv` | e1-arxiv-html（https://arxiv.org/html/2610.22367v1） | 2026-10-17 20:50:20 | 200 ok，1,073,889 字节 | e-e1-arxiv-html.html / .txt / .headers |
| 同上 | e1-arxiv-pdf（https://arxiv.org/pdf/2610.22367v1） | 2026-10-17 20:50:24 | 200 ok，2,380,628 字节；工具备注“响应为 PDF，按 180 秒限时重新请求” | e-e1-arxiv-pdf.pdf / .txt / .headers |
| `sources/e-manifest-3.tsv` | e3-aptamer-pdf（https://www.gstatic.com/science-ai/agentic_aptamer_sandbox.pdf） | 2026-10-17 20:51:59 | 200 ok，8,077,293 字节 | e-e3-aptamer-pdf.pdf / .txt / .headers |

- 明细日志：`sources/e-fetch-log.jsonl`（三次运行合计 8 条，全部 ok，blocked 0，failed 0）。
- 已目视确认 E1 博客、E2、E3 博客、E4 与 arXiv 摘要页、论文 PDF 的 `.txt` 为正文（非验证页）；arXiv HTML 的 `.txt` 只做了检索，未逐段通读；各页的 `.headers` 由 barking 按公开规则重写，只保留状态、URL、日期、类型，其余头未存档。
- 时刻取自页面元数据（`article:published_time` / `datePublished`）的为 E1 博客与 E2 页面；E3 博客与 E4 页面只有日期，记“官方未给时刻”。

## 二、热度（barking heat）

| 时间（北京） | 文件 | 查询 | 结果 |
|---|---|---|---|
| 20:53:11 | `sources/e-heat.json` | HN：“Designing more efficient quantum circuits with Claude”；“Discovering state-of-the-art quantum circuits with large language models”；“Anthropic AI expectations gap Nobel”；“Grounding AI agents in biological reality wet labs”；“Aptamers as a sandbox lab-in-the-loop agentic”；“StarSkirmish”；“Gemini 4 Argon StarCraft”。Reddit：“StarSkirmish”、“Claude quantum circuits” | HN 7 组中 3 组命中，均为两个相关帖：量子博客帖（152 分 / 32 评，10-17 02:00，id 50121939）与 StarSkirmish 帖（228 分 / 102 评，10-16 22:18，id 50118688，被两个查询命中）；其余 4 组 0 命中。**Reddit 2 个查询均失败**：old.reddit.com 的 search.json 被重定向到登录页（Content-Type text/html），解析不了 JSON。同一来源失败两次，按派工规则未再重试，Reddit 热度记为失败 |
| 20:56:43 | `sources/e-x-heat.json` | HN：“AI Expectations Gap”；“Anthropic employees expect AI Nobel-level discovery survey”；“aptamer agentic wet lab Google”；“Anthropic quantum circuits Claude Mythos” | “AI Expectations Gap”命中 2 条：50125602，4 分 / 2 评，10-17 06:27；50129987，1 分 / 0 评，10-17 19:06。其余 3 组 0 命中。Reddit 未查 |

- 为什么是 `e-x`：`e-heat.json` 已存在，`barking heat` 拒绝覆盖（报错“已有 e-heat.json，拒绝覆盖”），该次未执行任何查询，也未写入。我没有删除或覆盖 `e-heat.json`，而是用组前缀 `e-x` 另存一次补查（文件名仍以 `e-` 开头）。`e-x-heat.json` 是本组新增的补充文本文件（可入库）。
- 量子博客的 arXiv 标题查询在 HN 0 命中。E3 的 HN 查询全部 0 命中。

## 三、摘句核对（本组自写脚本）

- 脚本：`sources/e-verify-quotes.py`（只读 `sources/e-*.txt`，UTF-8 精确子串匹配，用 `python3 -I` 运行）。
- 第一次运行：57 条中 1 条未命中（“Games are Protoss versus Protoss on three ladder maps.”），原因是原文在“three”后换行。已将该条改为“Heartbreak Ridge, Benzene and Destination”（原文同段，单行）并重跑：57/57 命中。另有一次控制台编码报错（Windows GBK），在脚本里加 `sys.stdout.reconfigure(encoding="utf-8")` 修复，不影响匹配结果。
- 脚本以外的人工核对：论文表格数值（Extended Data Table 1、SI Table S3）取自 PDF 文本的版面抽取，行列靠空白对齐，未经图片核对；“1.9·10 6”在抽取中断开，按上下文读作 1.9·10^6。

## 四、文件与大小

- 文本（可入库，`.txt` 与 `.json`、`.tsv`、`.jsonl`、`.py`、`.headers`、`.md`）：最大为 `e-e3-aptamer-pdf.txt`（142,797 字节）与 `e-e1-arxiv-pdf.txt`（269,058 字节），均远小于 1 MB。没有超过 1 MB 的文本文件。
- 本地存档（不入库，交 Claude 传网盘）：`e-e1-quantum.html`（181 KB）、`e-e1-arxiv-abs.html`（42 KB）、`e-e1-arxiv-html.html`（1.07 MB）、`e-e2-expectations.html`（205 KB）、`e-e3-google-wetlab.html`（191 KB）、`e-e4-starskirmish.html`（22 KB）、`e-e1-arxiv-pdf.pdf`（2.38 MB）、`e-e3-aptamer-pdf.pdf`（8.08 MB）。
- 本组文件清单：`e-manifest.tsv`、`e-manifest-2.tsv`、`e-manifest-3.tsv`、`e-fetch-log.jsonl`、`e-heat.json`、`e-x-heat.json`、`e-verify-quotes.py`，上述存档文件，`evidence-e.md`、`capture-log-e.md`。
- 试验文件：无。补查用的 `e-x` 前缀没有产生需要删除的废弃文件。

## 五、泄露自查

- 对 `docs/2610/1016/sources/e-*` 检索 `avatar`、`handle`、`Cookie`、`x-forwarded`、`remote-addr`、`cf-connecting` 等。命中的只是正文内容（如“handle only extremely rare inputs”、arXiv 页面的 MathJax Cookie 脚本说明、BibTeX Handler 链接），没有抓取者的头像、昵称、handle 或 IP。
- `.headers` 文件只保留状态、URL、日期、类型，其余头未存档（工具备注“避免暴露抓取地点或标识”）。
- 没有 X 帖子、没有私人个体资料，也没有登录态页面。

## 六、时间线（北京时间）

| 时间 | 动作 | 结果 |
|---|---|---|
| 20:49 | 读派工单、EDITORIAL 与模板，建 manifest，`barking fetch` 第一批 | 5 ok |
| 20:50 | 第二批（arXiv HTML 与 PDF） | 2 ok |
| 20:52 | 第三批（gstatic PDF） | 1 ok |
| 20:53 | `barking heat` 第一次 | HN 命中 2 个相关帖，Reddit 失败 2 次 |
| 20:56 | `barking heat` 第二次因覆盖被拒（未执行），改用 `e-x` 补查 | HN 1 组命中 |
| 21:00 前后 | 读存档、摘句、`e-verify-quotes.py` 核对、写 `evidence-e.md` 与本日志 | 57/57 摘句命中 |

## 七、假设与说明

- 官方页面未给时刻的只写日期（E3 博客 October 16, 2026；E4 页面 October 15, 2026），不编造时刻。
- E1 博客的时刻取 `article:published_time` 2026-10-16T17:29:00Z（北京 2026-10-17 01:29）；修改时间 17:32:09Z。arXiv 时刻取页面“Submitted on 15 Oct 2026”与“Thu, 15 Oct 2026 17:59:20 UTC”（北京 2026-10-16 01:59）。
- E2 时刻取 `datePublished` 2026-10-16T16:45:00Z（北京 2026-10-17 00:45）。
- HN 时刻为 `barking heat` 换算的北京时间。
- E4 的 roster 写法“13 roster bots, the 10 competitive human written bots and the 3 demo bots”，我按 13 = 10 + 3 读，属推断。
- E1-6 的 Lean 证明只见于博客；论文 PDF 与 arXiv HTML 存档文本中没有“Lean”（`grep -w` 计数为 0）。
- E2 的 39% 分母：正文未给确切数字，推断为美国成年人样本的一半。
- 未修改仓库中的其他文件，未 commit、未 push。
