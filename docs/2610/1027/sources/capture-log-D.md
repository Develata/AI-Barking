# 1027 期 D 组（速览）抓取日志

时区：本机 EDT（UTC-4）；下表时刻均为北京时间（UTC+8）。`barking fetch` 的逐条记录在 `d-fetch-log.jsonl`。工具：`barking fetch`（默认 curl 式匿名 GET，桌面 Chrome UA）、`barking html2txt`、`barking heat`、`curl`、`opencli browser`。未截图，未登录，未绕付费墙，未发帖或点击。

## 一、`barking fetch` 批量存档（组前缀 d）

| 时间（北京） | 清单 | 条目 | 结果 | 备注 |
|---|---|---|---|---|
| 20:29:13 | `sources/d-fetch-list.tsv` | d1-anthropic-drug、d2-brangus-survival、d3-axiom-cfsg-gh、d3-axiom-cfsg-site、d4-liquid-lfm3、d5-replit-cloud、d6-ai2-novelty、d7-openai-oracle、d8-apple-mbp-m6、d8-apple-imac-m6 | 10 条 ok，0 blocked，0 failed | d4-liquid-lfm3 的 .txt 1146 行，d6 的 html 1.16 MB（仅本地） |
| 20:31:20 | `sources/d-fetch-list-d4.tsv` | d4-lfm3-17b、d4-lfm3-vl-2b、d4-lfm3-audio-2b、d4-lfm3-omni-2b、d4-d1-2b、d4-d1-omni-2b | 6 条 ok | 首次提交因名称含 “.” 被拒（exit 2，未写任何文件），改名 `lfm3-17b` 后重跑 |
| 20:32:15 | `sources/d-fetch-list-d7b.tsv` | d7-openai-oracle-2024ref（实为 2026-10-08 Oracle 案例页 openai.com/index/oracle/） | ok | 文件名中的 “2024ref” 有误，保留不改名 |
| 20:34 左右 | `sources/d-fetch-list-d4b.tsv` | d4-lfm-license（LFM3-1.7B 仓库 LICENSE） | ok | 10,574 字节 |
| 20:35:35 | `sources/d-fetch-list-d1.tsv` | d1-bighat-report（bighatbio 报告页）、d1-divergence-preprint（PDF）、d1-generate-report（PDF） | bighat 页 blocked（正文 28 个非空白字符，跳转到 CDN PDF 的 HTML 壳）；divergence ok（21 MB PDF）；generate ok（先 200 后 PDF，按 180 秒限时重请求） | bighat 页的拦截页保留为 `d-d1-bighat-report.html`，无 .txt |
| 20:35–20:36 | `sources/d-fetch-list-d1c.tsv` | d1-bighat-pdf（bighat 页实际跳转到的 CDN PDF，`bighat_pilot_retro_final_v2.pdf`） | ok，5.5 MB，1898 行 txt | 报告正文（“about 1,010 design variants (1,232 wells)”）从此取得 |
| 20:37 | `sources/d-fetch-list-d6.tsv` | d6-arxiv-abs（arXiv 2610.02022 摘要页） | ok | v1 2026-10-01，v3 2026-10-26 |

失败（blocked/failed）汇总：1 条 blocked（d1-bighat-report，见上）；其余为 0。

## 二、`curl`（非 barking fetch 的一次性请求）

| 时间（北京） | 请求 | 结果 |
|---|---|---|
| 约 20:40 | `https://api.github.com/repos/AxiomMath/CFSG`（`-H User-Agent: barking-evidence`） | 200；存 `sources/d-d3-gh-api-repo.json`；created_at 2026-10-27T13:19:32Z，pushed_at 2026-10-27T15:07:45Z，license Apache-2.0，open_issues_count 0 |
| 约 20:40 | `https://api.github.com/repos/AxiomMath/CFSG/commits?per_page=5` | 200；存 `sources/d-d3-gh-api-commits.json`；仅 1 个提交 “Initial commit”，2026-10-27T15:07:38Z |
| 约 20:40 | `https://axiommath.ai/research/the-enormous-theorem-formalized/` | 200，136,619 字节；存 `sources/d-d3-axiom-blog.html`，经 `barking html2txt` 转为 `d-d3-axiom-blog.txt`（173 行）。未经 barking fetch，无 .headers |
| 约 20:45 | `https://hn.algolia.com/api/v1/items/<id>`（id: 50265609、50264853、50261647、50262229、50261352、50258538） | 200；存 `sources/d-hn-item-<id>.json`。Algolia 返回的评论不含分数，所以 HN 评论 “得分” 在 evidence 中写不可得 |
| 约 20:45 | `https://hn.algolia.com/api/v1/search?query=…&tags=story`（Replit Cloud Agents、novelty judges、Old Ideas, Novel Problems、Ai2 novelty、Replit self-driving company、LFM3 Liquid） | 200；仅用于查找，无文件 |

## 三、`barking heat`（HN）

运行三次，时刻约 20:33 北京：

1. 第一次（10 个 query）写入 `sources/d-heat.json`。
2. 第二次（另加 Replit、Ai2 等 query）因 `d-heat.json` 已存在而被拒（“拒绝覆盖”）。
3. 为完成第二次查询，将第一次的 `d-heat.json` 移到 scratchpad，第三次运行（合并全部 query）重新写入 `sources/d-heat.json`。第一次的文件仅留在 scratchpad，未入库，内容与第三次重叠。

第三次运行的结果摘要（北京时间）：

| query | 命中 | 结论 |
|---|---|---|
| early-progress-drug-development | 50265609，246 分 / 246 评，10-28 04:50 | D1 |
| survival-llm | 50264853，392 分 / 148 评，10-28 03:55（另一条 4 分 / 1 评） | D2 |
| AxiomMath/CFSG、Classification of Finite Simple Groups | 50261647，218 分 / 68 评，10-28 00:29 | D3 |
| lfm3 | 50262229，147 分 / 57 评，10-28 01:04，链接 liquid.ai/blog/lfm3 | D4 |
| Replit Cloud Agents、Old Ideas, Novel Problems | 0 条 | D5、D6 无 HN 帖 |
| novelty | 16 条，均无关（如 “Novel” 同形词） | 不采用 |
| oracle-apps-in-chatgpt | 50261365，1 分 / 0 评，10-28 00:09 | D7 |
| New 14-inch MacBook Pro with M6 | 50261352，378 分 / 521 评，10-28 00:08（apple.com/in）；50258521，165 分 / 201 评，10-27 21:01（newsroom） | D8 |
| iMac M6 | 50258538，347 分 / 436 评，10-27 21:02（apple.com/imac）；50258557，6 分 / 1 评；50259769（Reuters），3 分 / 0 评 | D8 |
| Reddit（Survival LLM、Axiom CFSG、LFM3、Replit Cloud Agents、MacBook Pro M6） | 5 次全部失败：old.reddit.com 返回登录跳转页（HTML，非 JSON），`warn: 响应不是 JSON` | 未取得 Reddit 热度，不重试 |

## 四、`opencli browser`

| 时间（北京） | 操作 | 结果 |
|---|---|---|
| 约 20:36 | `opencli browser d-bighat open https://www.bighatbio.com/bighat-anthropic-collaboration-report-10-27-2026 --window background` | 页面跳转到 CDN PDF，opencli 输出 “Update available: v1.8.8 → v2.0.2”（未升级） |
| 约 20:36 | `opencli browser d-bighat eval`（读取 innerText） | 长度 0（PDF 查看器），已转用 barking fetch 取 PDF |

注意：opencli 会话 `d-bighat` 仍绑定一个后台标签页，未关闭（可由 Develata 收尾时清理）。

## 五、未取得或失败的项

- BigHat 报告 HTML 页：blocked（已由 PDF 替代取得同一报告）。
- Reddit：全部 5 个查询失败（登录跳转）。
- Replit 与 Ai2：HN 无当期帖；未找到 Reddit 替代来源。
- Divergence 博文中的 “three designs that could bind to IgE”：预印本文本中未找到对应数字（未找到，已在 evidence 中标注）。
- Anthropic 博文中的 BigHat “100x” 聚集速率：报告文本中未找到（已标注）。
- D2 “不能代替急救” 类免责声明：原页未找到（已标注）。

## 六、待删除文件（精确路径）

- `E:\gitclone\AI-Barking\docs\2610\1027\sources\d-fetch-list-d7.tsv`：早期误写的清单，只含 d7 与一条未取的 `d4-lfm3-blog-skip`（LFM3-1.7B 页面），无实际取证结果，已由 `d-fetch-list-d7b.tsv` 替代。

其余 `d-fetch-list-*.tsv` 是实际取证的清单，保留。

## 七、本组输出文件（`docs/2610/1027/sources/`）

- 交付：`evidence-D.md`、`capture-log-D.md`
- 文本入库：`d-*.txt`、`d-*.headers`（.headers 为小文件，已入库候选）、`d-fetch-log.jsonl`、`d-heat.json`、`d-hn-item-*.json`、`d-d3-gh-api-*.json`、清单 `d-fetch-list*.tsv`
- 仅本地（不入库，按派工单，待 Claude 传网盘）：`d-*.html`、`d-*.pdf`（`d-d1-bighat-pdf.pdf` 5.5 MB、`d-d1-divergence-preprint.pdf` 21 MB、`d-d1-generate-report.pdf` 2.9 MB）

超过 1 MB 的文本文件：无（txt 最大为 `d-d4-liquid-lfm3.txt` 等，均小于 1 MB）。超过 1 MB 的 HTML：`d-d6-ai2-novelty.html`（1.16 MB，仅本地）。
