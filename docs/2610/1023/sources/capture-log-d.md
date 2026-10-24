# 1023 期 D 组（速览）抓取与热度日志

执行方：Claude Haiku 5.5（claude-haiku-5-5）。北京时间 = UTC+8。派工单要求本组不截图；所有存档均为网页原件与纯文本，未登录，未使用付费墙绕过。

## 一、网页存档（`barking fetch`，清单 `d-urls.tsv`，日志 `d-fetch-log.jsonl`）

| 名称 | URL（去查询参数） | 工具 | 结果 | 北京时间 | 文件 |
|---|---|---|---|---|---|
| duet | https://claude.com/resources/articles/claude-duet-safety-privacy-and-security | barking fetch（curl） | 200 ok，539,653 字节，770 行文本 | 2026-10-24 19:54:29 | `d-duet.html`、`d-duet.txt`、`d-duet.headers` |
| google-tb | https://research.google/blog/bringing-foundational-models-to-the-front-lines-of-tb-screening/ | barking fetch（curl） | 200 ok，183,596 字节，663 行 | 2026-10-24 19:54:27 | `d-google-tb.html`、`d-google-tb.txt`、`d-google-tb.headers` |
| brotli-ptp | https://patchtheplanet.org/brotli | barking fetch（curl） | 200 但 blocked：正文过短（51 个非空白字符，JS 渲染壳页） | 2026-10-24 19:54:27 | `d-brotli-ptp.html`、`d-brotli-ptp.headers`（无 .txt，属壳页，不作为证据） |
| node | https://nodejs.org/en/blog/vulnerability/october-2026-security-releases | barking fetch（curl） | 200 ok，305,957 字节，233 行 | 2026-10-24 19:54:27 | `d-node.html`、`d-node.txt`、`d-node.headers` |
| cve | https://cveawg.mitre.org/api/cve/CVE-2026-111497（cve.org 同源 JSON） | barking fetch（curl） | 200 ok，4,846 字节（JSON，无 .txt） | 2026-10-24 19:54:27 | `d-cve.json`、`d-cve.headers` |
| railway | https://blog.railway.com/p/preliminary-report-october-security-incident | barking fetch（curl） | 200 ok，71,871 字节，241 行 | 2026-10-24 19:54:27 | `d-railway.html`、`d-railway.txt`、`d-railway.headers` |
| istio | https://istio.io/latest/news/security/istio-security-2026-007/（清单 `d-urls2.tsv`） | barking fetch（curl） | 200 ok，27,937 字节，147 行 | 2026-10-24 19:57:54 | `d-istio.html`、`d-istio.txt`、`d-istio.headers` |

补抓：`https://patchtheplanet.org/brotli` 的 fetch 结果为 blocked，改用 opencli 浏览器（`opencli browser d-brotli open … --window background` 后 `eval`）取渲染后正文：
- `opencli browser d-brotli eval "document.body.innerText"`：可见正文，但漏掉折叠区“Other software to check”的内容。
- `opencli browser d-brotli eval "document.body.textContent…"`：取全文，写入 `d-brotli-ptp-opencli.txt`（9,416 字节，含“Other software to check”的 Electron、curl/libcurl、Python/Java 绑定、字体解码器说明）。
- 该 opencli 页面标签仍开着（后台，未登录，未做任何表单或点击）。
- opencli 版本提示“Update available: v1.8.8 → v2.0.2”，未升级。

## 二、未取得或失败

| 项目 | 原因 | 处理 |
|---|---|---|
| OpenAI X 帖（“二月源码已修”） | WebSearch 无结果；帖子 ID 未知；未用 fxtwitter 取 | 记“未找到一手来源” |
| cve.org 页面 `https://www.cve.org/CVERecord?id=CVE-2026-111497` | 页面需要 JS，未直接取 | 改用同源 cveawg JSON（`d-cve.json`），其 `cveMetadata` 与 CNA 容器与 cve.org 同步 |
| Reddit 热度 | 派工单未要求本组做 Reddit 采样；Algolia 搜索未设 Reddit 查询 | 未采样 |

## 三、WebSearch

| 查询 | 结果 |
|---|---|
| OpenAI Brotli CVE-2026-111497 fixed in February source code X post | 无相关结果（返回的 AlmaLinux 等为其他 CVE，与 OpenAI 无关，未采用） |

## 四、HN 热度（`barking heat docs/2610/1023 d --item …`，写入 `d-heat.json` 与 `d-hn-<id>.json`）

| HN 条目 | 北京时间（取时刻 2026-10-24 19:58:43） | 分数 | 嵌套评论 |
|---|---|---|---|
| 50218815 Claude Duet Safety, Privacy, and Security | 发帖 2026-10-24 14:42:22 | 1 | 1 |
| 50186037 Claude Duet（1021 主帖） | 发帖 2026-10-22 03:14:01 | 565 | 529 |
| 50211898 Preliminary Report Oct 21st Security Incident | 发帖 2026-10-24 00:48:50 | 36 | 8 |
| 50192733 Node.js security advisory: Brotli decoder RCE | 发帖 2026-10-22 16:27:26 | 17 | 3 |
| 50189292 Node.js Brotli RCE (CVE-2026-111497) | 发帖 2026-10-22 07:58:43 | 23 | 0 |

Algolia 搜索（`hn.algolia.com/api/v1/search`）的字段与条目曾错配，本表以逐条 item 接口 JSON 为准。HN 搜索 “tuberculosis…” 与 “Nexus AI tuberculosis” 无结果，故 Google TB 条目无 HN 热度。

## 五、时区与时刻

- 官方页只有日期的，已标“官方未给时刻”：Duet 安全说明（October 23, 2026）、Google TB（October 23, 2026）、patchtheplanet（Updated October 23, 2026）、Node.js（Wednesday, October 21, 2026）、Istio（Oct 15, 2026）、Railway 初步报告（Oct 22, 2026）。
- CVE 记录时刻为 UTC 原文换算：datePublished 2026-10-22T13:35:41Z = 北京 2026-10-22 21:35:41；dateUpdated 2026-10-24T03:55:16Z = 北京 2026-10-24 11:55:16；datePublic 2026-02-27T07:50:00Z = 北京 2026-02-27 15:50。
- Railway 时间线为 UTC：2026-10-20 13:58 至 2026-10-22 03:32 = 北京 2026-10-20 21:58 至 2026-10-22 11:32。

## 六、交付前自查

- 本组新增文件均为文本、网页原件、JSON 与日志，无截图（派工单规定）。
- 抓取者信息：存档头部只保留 Date 等通用头；HN 评论作者为公开用户名，属第三方公开发言，未涉及抓取者账号。
- 大于 1 MB 的文件：`d-duet.html` 539,653 字节、`d-node.html` 305,957 字节，均小于 1 MB，无需告知；无文件超过 1 MB。
- 未运行会读取本机浏览器 Cookie 的工具；未登录任何站点；未发帖、点赞、评论、关注。
- `docs/2610/1023/` 内新增的 D 组文件：`d-urls.tsv`、`d-urls2.tsv`、`d-fetch-log.jsonl`、`d-*.html/.txt/.headers/.json`、`d-brotli-ptp-opencli.txt`、`d-heat.json`、`d-hn-*.json`、`evidence-d.md`、`capture-log-d.md`。未修改其他组文件，未 commit。
