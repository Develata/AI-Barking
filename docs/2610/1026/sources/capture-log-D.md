# 1026 期 D 组抓取日志

时间为北京时间（UTC+8）。工具：`barking fetch`（curl 式匿名 GET，结果同时记入 `d-fetch-log.jsonl`）、`barking html2txt`、`barking heat`、`opencli browser d`（opencli 浏览器会话 `d`，只读取页，未登录、未接受 Cookie、未发帖）、WebSearch（只作线索，不存档）。

## 1. barking fetch 批次

| 时间（北京） | 清单 | name | URL（已去查询参数） | 结果 | 原因 / 备注 |
|---|---|---|---|---|---|
| 10-27 22:53 | `d-list.tsv` | s1-sec | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm | blocked 403 | 见下：改 opencli |
| 10-27 22:53 | 同上 | s1-news | https://www.anthropic.com/news/s1-sec | ok 200 | 正文 69 行 txt |
| 10-27 22:53 | 同上 | noyb | https://noyb.eu/en/ai-training-platform-outlier-illegally-surveils-european-platform-workers | ok 200 | |
| 10-27 22:53 | 同上 | cf-kv | https://blog.cloudflare.com/kv-cache-exposure-remediation/ | ok 200 | |
| 10-27 22:53 | 同上 | sglang-cf | https://www.lmsys.org/blog/2026-10-26-cf-security | ok 200 | |
| 10-27 22:53 | 同上 | 404media | https://www.404media.co/chatgpt-keeps-telling-people-404-media-can-solve-their-tech-support-problems/ | ok 200 | 正文后半为付费内容（“This post is for paid members only”），只用可见部分 |
| 10-27 22:53 | 同上 | reuters-meta-offer | https://www.reuters.com/legal/transactional/meta-launches-35-billion-stock-offering-fund-ai-expansion-2026-10-26/ | blocked 401 | 改 opencli，见下 |
| 10-27 22:53 | 同上 | reuters-nvidia | https://www.reuters.com/technology/nvidia-meta-partner-muse-agent-nvidia-invests-3-billion-2026-10-27/ | blocked 401 | 改 opencli，见下 |
| 10-27 22:53 | 同上 | ieee-wiki | https://spectrum.ieee.org/wikipedia-wikiproject-ai-writing-cleanup | ok 200 | |
| 10-27 22:53 | 同上 | wp-ai-cleanup | https://en.wikipedia.org/wiki/Wikipedia:WikiProject_AI_Cleanup | ok 200 | |
| 10-27 23:02 | `d-list2.tsv` | meta-ir-pricing | https://investor.atmeta.com/investor-news/press-release-details/2026/Meta-Announces-Pricing-of-Offering-of-Depositary-Shares-Representing-Series-A-Mandatory-Convertible-Preferred-Stock-and-Offering-of-Class-A-Common-Stock-in-Connection-with-Forward-Sale-Agreements/default.aspx | blocked 403 | Cloudflare 质询；改 opencli |
| 10-27 23:02 | 同上 | nvidia-newsroom | https://nvidianews.nvidia.com/news/nvidia-and-meta-to-expand-collaboration-on-muse-and-muse-family-of-models-amid-robust-growth | ok 200 | |
| 10-27 23:02 | 同上 | meta-fwp-upsize | https://www.sec.gov/Archives/edgar/data/0001326801/000119312526432549/d382225dfwp.htm | blocked 403 | 改 opencli（同 URL 已由 opencli 取得） |
| 10-27 23:05 | `d-list3.tsv` | diplab | https://diplab.eu/report-new-diplab-report-surveillance-at-scale/ | ok 200 | 报告页 |
| 10-27 23:05 | 同上 | sec-index-s1 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/0001628280-26-067168-index.htm | blocked 403 | 改 opencli（见下） |
| 10-27 23:05 | `d-list4.tsv` | diplab-pdf | https://diplab.eu/wp-content/uploads/2026/10/Surveillance_at_scale_diplab_october_2026-1.pdf | ok 200 | PDF 30 MB，本地存；pdftotext 出 `d-diplab-pdf.txt` |

日志文件：`sources/d-fetch-log.jsonl`（16 行，对应上列 barking fetch 的 16 次请求）。

## 2. opencli 浏览器取页（会话 `d`）

| 时间（北京） | URL | 动作 | 结果 | 产出文件 |
|---|---|---|---|---|
| 10-27 22:55 | SEC S-1 htm | open → get html | ok（页面 13.8 MB，SEC 文档包装） | `d-s1-sec.html`（本地存，不入库）→ `barking html2txt` → `d-s1-sec.txt`（1.64 MB，超过 1 MB，见回报） |
| 10-27 23:01 | Reuters Meta 10/26 | open → extract | ok（标题 “Meta eyes record $97.5 billion equity raise to fund AI expansion”） | `d-reuters-meta.json` |
| 10-27 23:01 | Reuters NVIDIA 10/27 | open → extract | ok（“Nvidia, Meta deepen AI ties with $3 billion investment deal”） | `d-reuters-nvidia.json` |
| 10-27 23:02 | Meta IR 定价公告 | open → extract（空）→ wait 5s → extract（仍空）→ get html | 第二次 extract 仍无正文；get html 取得 78 KB | `d-meta-ir-pricing.html` → `d-meta-ir-pricing.txt`（20 KB）；`d-meta-ir-pricing.json` 为空 extract，作废（见待删除清单） |
| 10-27 23:05 | EDGAR 索引页（S-1） | open → extract | ok（Filing Date 2026-10-26；Accepted 2026-10-26 09:00:11） | `d-sec-index-s1-opencli.json` |
| 10-27 23:06 | SEC FWP 431036 `d116268dfwp.htm`（launch） | open → get html → html2txt | ok | `d-meta-d116268dfwp.html`、`.txt` |
| 10-27 23:07 | SEC FWP 432549 `d382225dfwp.htm`（pricing term sheet） | open → get html → html2txt | ok | `d-meta-d382225dfwp.html`、`.txt` |
| 10-27 23:07 | SEC 424B5 `d94886d424b5.htm`（Class A 普通股） | open → get html → html2txt | ok（仅普通股 22.5 亿美元条款） | `d-meta-424b5.html`、`.txt` |
| 10-27 23:08 | SEC 424B5 `d133631d424b5.htm`（优先股） | open → get html → html2txt | ok（优先股条款；未出现 60 亿 ATM） | `d-meta-424b5-atm.html`、`.txt`（文件名沿用“atm”，实际为优先股补充说明书，见勘误） |

说明：Meta 定价公告（IR）的 html 是 opencli 取得，IR 页面的“October 27, 2026”为站点显示日期，PR 正文为 “MENLO PARK, Calif., Oct. 26, 2026”。

## 3. WebSearch（线索，未存档）

| 时间（北京） | 查询要点 | 用途 |
|---|---|---|
| 10-27 23:00 左右 | Meta $35 billion stock offering | 找到 Meta IR 定价公告、FWP、Reuters 等线索（以上列原站为准） |
| 同上 | NVIDIA $3 billion Meta Muse | 找到 NVIDIA 官方公告线索（nvidianews.nvidia.com） |
| 同上 | NVIDIA newsroom press release | 同上 |
| 同上 | Meta pricing public offering | 找到 investor.atmeta.com 与 SEC FWP 线索 |
| 同上 | Meta launch “at-the-market” $60 billion | 找到 FWP 线索；ATM 原句以 SEC FWP 431036 为准 |

搜索结果中的二手报道（gurufocus、tradingview、tikr 等）未作为依据使用。

## 4. barking heat

| 时间（北京） | 命令要点 | 结果 |
|---|---|---|
| 10-27 23:04 | `heat docs/2610/1026 d --hn "Anthropic S-1" --hn "Scale AI noyb" --hn "GLM-5.3 Cloudflare" --hn "404 Media ChatGPT" --hn "Meta stock offering" --hn "Nvidia Meta Muse" --hn "Wikipedia AI cleanup" --hn "Cloudflare SGLang"` | 成功，写入 `d-heat.json`（HN 8 组，0 失败） |
| 10-27 23:10 左右 | 追加 `--hn "Meta Muse NVIDIA" --hn "Scale AI Outlier" --hn "Meta equity offering 97.5" --hn "Wikipedia AI-generated content"` | 拒绝覆盖（`d-heat.json` 已存在），未执行；未补热度的条目：D5 NVIDIA 公告、D2 Scale AI |

## 5. 失败与未取得

- SEC S-1 htm、EDGAR 索引、Reuters ×2、Meta IR、Meta FWP 升额版：curl 被 403/401 拦截，已改 opencli；opencli 对 SEC 与 Meta IR 成功，Reuters 成功。
- Reuters 对 noyb 投诉的报道（ChatGPT 称 10/27 08:44 UTC）：未检索，未核。
- 404 Media 付费段：未取得，不绕过。
- Scale AI 对 noyb 投诉的回应：未找到。
- CF 通知客户：原文未见，未找到。
- HN 评论摘录：本轮未做。

## 6. 待删除文件（本组自生成的废弃或空文件）

- `docs/2610/1026/sources/d-meta-ir-pricing.json`（空 extract，已由 `.html`→`.txt` 替代）
- `docs/2610/1026/sources/d-reuters-meta-offer.html`、`d-reuters-nvidia.html`（401 错误页，正文由 opencli 的 `.json` 取得；`barking fetch` 的 blocked 条目保留正文与 headers，这两份 html 为 774 字节错误页，建议删）
- `docs/2610/1026/sources/d-meta-fwp-upsize.html`、`d-sec-index-s1.html`（403 错误页；对应 URL 已由 opencli 取得）
- `docs/2610/1026/sources/d-meta-424b5-atm.*` 命名误导（实际为优先股补充说明书，不是 60 亿 ATM），如需改名，由 Claude 决定（不是删除）

以上仅列清单，本组未调用 rm。
