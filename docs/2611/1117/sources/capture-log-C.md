# 1117 期 C 组（速览）抓取日志

执行：Claude Haiku 5.5（`claude-haiku-5-5`）。时间为北京时间（本机 UTC+8），取自 `c-fetch-log.jsonl`、`c-heat.json`、命令终端输出与各接口返回的时间戳。文件前缀 `c-`。未登录任何账户，未提交表单，未发帖、点赞、评论、关注。未使用 opencli 与 headless Chrome（C 组不需截图）；未绕过付费墙；未使用镜像或转载站。

## 抓取

| 北京时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 2026-11-18 16:27:15 | 批次 1：`docs/2611/1117/sources/c-fetch-list-1.tsv`（8 条） | `barking fetch docs/2611/1117 c …` | 8 条全部 200 / ok，2 秒完成；`c-fetch-log.jsonl` 有记录 |
| 16:27:16–18 | https://blog.google/innovation-and-ai/products/gemini-app/tasks-in-gemini-free-new-features/ （gem-tasks） | 同上 | ok。`c-gem-tasks.html`（379,254 B）、`.txt`（953 行）、`.headers` |
| 16:27:17 | https://blog.google/products-and-platforms/platforms/google-pay/wallet-available-ios-united-states/ （wallet-ios） | 同上 | ok。`c-wallet-ios.*`（720 行） |
| 16:27:17 | https://runta.com/blog/introducing-frontierharness-eval-v1-1/ （frontierharness） | 同上 | ok。`c-frontierharness.*`（296 行） |
| 16:27:17 | https://blog.google/security/how-google-is-making-cybersecurity-advice-from-ai-more-helpful/ （helpbench） | 同上 | ok。`c-helpbench.*`（337 行） |
| 16:27:17 | https://openai.com/index/advancing-computer-use-for-healthcare-teams/ （openai-health） | 同上 | ok，**未遇 403**（未用 BARKING_CHROME 渲染路线）。`c-openai-health.*`（336 行） |
| 16:27:17 | https://www.prnewswire.com/news-releases/isomorphic-labs-announces-5-billion-series-c-funding-302942062.html （isomorphic） | 同上 | ok。`c-isomorphic.*`（1,159 行）。最终 URL 与请求 URL 相同（canonical 302942062） |
| 16:27:17–18 | https://microsoft.ai/news/introducing-mai-image-2-7/ （mai-image27） | 同上 | ok。`c-mai-image27.*`（503 行） |
| 16:27:18 | https://microsoft.ai/news/introducing-mai-cyber-2-flash/ （mai-cyber2flash） | 同上 | ok。`c-mai-cyber2flash.*`（466 行） |
| 16:28:21–23 | https://artificialanalysis.ai/text-to-image/leaderboard （aa-image-edit，我方误用的 URL） | 同上 | **failed，HTTP 404**，无 `.txt`，不重试。残留 `c-aa-image-edit.html`、`c-aa-image-edit.headers`，已列入待删除 |
| 16:28:24 | 批次 2：`c-fetch-list-2.tsv`；https://artificialanalysis.ai/image/leaderboard/editing （aa-image-edit-b） | 同上 | ok。`c-aa-image-edit-b.*`（`.txt` 732 行，12,564 B；`.html` 2 MB，仅本地） |
| 16:28:25 | https://arxiv.org/abs/2606.24819 （helpbench-abs） | 同上 | ok。`c-helpbench-abs.*`（241 行） |
| 16:28:38 | https://arxiv.org/pdf/2606.24819 （helpbench-paper） | 同上（响应为 PDF，按 180 秒限时重新请求） | ok。`c-helpbench-paper.pdf`（752,034 B，仅本地）、`.txt`（1,917 行，`pdftotext -enc UTF-8 -layout`）、`.headers` |

批次 1 与批次 2 的清单均保存在 `sources/`（`c-fetch-list-1.tsv`、`c-fetch-list-2.tsv`）。

时区与 HTTP 头：所有 `.headers` 由 `barking fetch` 去掉抓取地点与标识。

## 核对与脚本（非抓取）

| 北京时间 | 动作 | 结果 |
|---|---|---|
| 16:27–16:40 | 读取 `c-*.txt` 的正文段，用 `grep` 与 `sed -n` 摘句；用 `python3 -I` 提取 `<meta>`、JSON-LD `datePublished`、`<time>` 元数据 | 摘句与时刻据此写入 `evidence-C.md` |
| 16:40 | 用 `python3 -I` 解析 `c-hn-50532191.json`、`c-hn-50535898.json` 的评论树，按内容挑选评论（API 不返回评论得分，故未按分排序） | 仅选 2–3 条有理有据的评论 |
| 16:4x | 检索 `c-helpbench-paper.txt` 的 §4.1、§4.3、§5、§5.2 | 证实 "14%" 与评分定义位置 |
| 16:4x | 用 `python3 -I` 检查 `c-openai-health.html` 中的 `dateTime`：`2026-11-16T06:00` 属页内另一篇相关文章，本文只有 `November 16, 2026` | 本文时刻记为官方未给 |

## 热度

| 北京时间 | 动作 | 结果 |
|---|---|---|
| 16:28:38 | `barking heat docs/2611/1117 c --item 50532191 --item 50535898 --hn "FrontierHarness" --hn "HelpBench" --hn "Isomorphic Labs" --hn "MAI Image 2.7" --hn "MAI Cyber 2" --hn "Google Wallet iOS" --hn "Gemini tasks spending cap" --hn "OpenAI healthcare computer use" --reddit "FrontierHarness" --reddit "HelpBench"` | 写入 `c-heat.json`（HN 8 组、条目 2 个）。Reddit 2 条失败：重定向到 `old.reddit.com/login` |
| 16:3x | 第二次 `barking heat`（查询 "Gemini Tasks"、"Put Gemini to work"、"AP2 agent payments Gemini"、"Gemini 4 Hydrogen"） | **未写入**：工具拒绝覆盖已有 `c-heat.json`（退出提示“拒绝覆盖”）。未删除旧文件（按规则不 rm）；改用下一行的直接查询 |
| 16:29 | 直接调 HN Algolia 搜索接口（curl，`--max-time 60`），查询 "Gemini Tasks" 与 "Put Gemini to work"，时间下限 `created_at_i > 1794000000` | 输出 `c-hn-search-gemini-tasks.json`（3 条命中，均无关）、`c-hn-search-put-gemini-to-work.json`（2 条命中，均无关）。结论：HN 无相关主帖 |
| 16:28–16:40 | `barking heat` 之外，按 HN 条目 ID 取 `c-hn-50532191.json`、`c-hn-50535898.json`（`barking heat --item` 写入） | 评论树保存为 JSON（`c-hn-50532191.json` 75 KB，`c-hn-50535898.json` 19 KB） |

Reddit 结果：查询 FrontierHarness 与 HelpBench 均失败（登录重定向，非 JSON）。同一来源失败两次，按规则不再重试其他条目。C 组其余条目的 Reddit 热度未取，记为失败/未取。

HN 数据（取得时刻 16:28 北京，除注明外为搜索接口）：

| 条目 | HN 条目 | 分 / 评 | 提交时刻（北京） |
|---|---|---|---|
| C2 | 50534509 Google Wallet 上线美国 iOS | 2 / 0 | 2026-11-18 01:05 |
| C3 | 50532191 What is the best harness for each model? | 368 / 125（搜索）；367 / 124（item） | 2026-11-17 22:53 |
| C4 | 无相关主帖（"HelpBench" 命中 50471281，无关） | — | — |
| C5 | 50535898 Advancing computer-use agents for healthcare teams | 47 / 30 | 2026-11-18 02:26 |
| C5 另 | 50519201 同 URL（重复提交） | 3 / 0 | 2026-11-17 00:11 |
| C6 | PR 本身无 HN 主帖；50537890 "Isomorphic Labs $5B Series C"（CNBC 链接） | 12 / 2 | 2026-11-18 04:42 |
| C7 | 50536193 MAI Image 2.7: Frontier quality at a fraction of the cost | 3 / 0 | 2026-11-18 02:44 |
| C8 | 50536171 MAI Cyber 2 Flash | 4 / 0 | 2026-11-18 02:43 |
| C1 | 无相关主帖 | — | — |

## 被拒、失败与未做

- 失败：`c-aa-image-edit`（404，误用 URL）；Reddit 两条（登录重定向）。
- 未做：C 组无截图（按派工单不需要）；未在 Reddit 对 C1、C2、C5–C8 取热度（Reddit 已失败）；未存档 Oct 27 的 MAI Image 2.7 首发原页（只据 Microsoft 页相关报道列表）；未存档 Microsoft 或 OpenAI 的 Trust 页面与 GPT-6 Cyber 比较（原文未给）。
- 线索未查（派工单列为“有余力再查”）：ChatGPT Wallet（openai.com/index/introducing-chatgpt-wallet/）与“Google AI 回答让搜索流量降 40%”（revscope.co）。时间不足，未查，未入 `evidence-C.md`。

## 待删除文件（本组自产的废弃文件，须 Claude 收尾时一次删除）

- `docs/2611/1117/sources/c-aa-image-edit.html`（404 页，误用 URL 的残留）
- `docs/2611/1117/sources/c-aa-image-edit.headers`（同上）

以下文件属有效记录，保留：`c-fetch-list-1.tsv`、`c-fetch-list-2.tsv`、`c-hn-search-gemini-tasks.json`、`c-hn-search-put-gemini-to-work.json`、`c-heat.json`。

## 隐私与体积

- `docs/2611/1117/` 中本组文件 grep 本机用户名、家目录路径：无命中（`.headers` 已去掉抓取地点）。HN 评论中的用户名为公开账户，仅在正文中以评论 ID 引用，未附其他个人信息。
- 入库候选的 `.txt` 均小于 1 MB（最大 `c-helpbench-paper.txt` 130 KB、`c-mai-image27.txt` 196 KB）。`.html`、`.pdf` 属原件，只存本地，不入库（`c-aa-image-edit-b.html` 2 MB 亦然）。
