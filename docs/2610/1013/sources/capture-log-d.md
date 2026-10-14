# 1013 期 D 组抓取日志（D1–D8）

时间为北京时间（UTC+8）。工具：`d-fetch.sh`（curl，普通浏览器 UA，匿名，未登录，未用浏览器会话；HTML 经 `d-html2txt.py` 转文本，脚本从 1012 期复制，逻辑未改）；HN Algolia（`d-heat.py`、`d-heat-search.py` 的查询，见下）；MSRC 与 NVD 公开 JSON/XML（curl）；WebSearch 一次（D4 谷歌一侧）。未运行读取本机 Cookie 的工具，未绕过挑战页、付费墙或登录，未用镜像或转载站作为依据，未在任何站点留下痕迹（无发帖、点赞、订阅、star）。本机时间为 EDT（UTC-4），文件时间戳按此换算。

## 一、抓取记录

| 北京时间 | 目标 | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 16:56:41 | https://www.apple.com/newsroom/2026/10/meet-apple-homeview-a-new-way-to-enjoy-entertainment-and-manage-the-home/ | d-fetch.sh | HTTP 200，273 KB，新闻稿日期 October 13, 2026 | d-d1-apple-homeview.html（仅本地）、d-d1-apple-homeview.txt |
| 16:57:23 | https://www.anthropic.com/engineering/redesigning-source-control | d-fetch.sh | HTTP 200，162 KB，Published Oct 13, 2026 | d-d2-anthropic-scion.html（仅本地）、d-d2-anthropic-scion.txt |
| 16:57:27 | https://embracethered.com/blog/posts/2026/visual-studio-copilot-remote-code-execution/ | d-fetch.sh | HTTP 200，23 KB，日期 Oct 13, 2026 | d-d3-embracethered.html（仅本地）、d-d3-embracethered.txt |
| 16:57:31 | https://msrc.microsoft.com/update-guide/vulnerability/CVE-2026-103572 | d-fetch.sh | HTTP 200，但 2.7 KB 的 SPA 壳（无正文，未引用） | d-d3-msrc.html（仅本地）、d-d3-msrc.txt（60 B，未引用） |
| 16:59 | https://api.msrc.microsoft.com/sug/v2.0/en-US/vulnerability/CVE-2026-103572（需 `--http1.1`，HTTP/2 连接被截断一次后成功） | curl | HTTP 200，3.8 KB JSON：severity Important，baseScore 8.8，exploited No，releaseDate 2026-10-13T07:00:00-07:00 | d-d3-msrc-api-sug.json |
| 16:59 | https://api.msrc.microsoft.com/cvrf/v3.0/updates（需 `--http1.1`） | curl | HTTP 200，49 KB；用于找到 2026-Oct 的 CvrfUrl | d-d3-msrc-api-cvrf-updates.json（49 KB 文本，定位用） |
| 17:01 | https://api.msrc.microsoft.com/cvrf/v3.0/cvrf/2026-Oct（需 `--http1.1`，约 40 MB XML） | curl | HTTP 200；全文件（约 40 MB）因超大未入库，存于会话 scratchpad（不在仓库内）；仅抽取 CVE-2026-103572 条目与产品名 | d-d3-msrc-cve-2026-103572-cvrf-entry.xml（5.6 KB）、d-d3-msrc-product-names.txt |
| 17:00 | https://services.nvd.nist.gov/rest/json/cves/2.0?cveId=CVE-2026-103572 | curl | HTTP 200，2 KB；vulnStatus Awaiting Analysis，published 2026-10-13T19:17:53 UTC（北京时间 10-14 03:17） | d-d3-nvd.json |
| 16:58 | https://xbow.com/blog/xbow-built-a-chrome-full-chain-exploit | d-fetch.sh | HTTP 200，190 KB，日期 October 13, 2026 | d-d4-xbow.html（仅本地）、d-d4-xbow.txt |
| 16:58 | 谷歌一侧（D4）：WebSearch “Google Chrome VRP $250,000 XBOW full-chain exploit 2026 bounty”（一次） | WebSearch | 未找到谷歌官方记录；仅见社交媒体转述（未核，不采信）与 Chromium VRP FAQ、媒体对 2024–2026 奖励上限的报道 | 未存档（摘要，非原文，不引用） |
| 16:58 | https://www.pi.website/blog/api-beta | d-fetch.sh | HTTP 200，80 KB，Published October 13, 2026 | d-d6-pi-api.html（仅本地）、d-d6-pi-api.txt |
| 16:57 | https://about.fb.com/news/2026/10/introducing-your-algorithm-facebook/ | d-fetch.sh | HTTP 200，485 KB，October 13, 2026 | d-d5-meta-algorithm.html（仅本地）、d-d5-meta-algorithm.txt |
| 16:58 | https://www.liquid.ai/blog/lfm2-5-24b-a2b | d-fetch.sh | HTTP 200，1.8 MB HTML（仅本地，超过 1 MB；txt 约 13 KB 入库），OCT 13, 2026 | d-d7-liquid-lfm.html（仅本地）、d-d7-liquid-lfm.txt |
| 17:03 | https://www.liquid.ai/lfm-license | d-fetch.sh | HTTP 200，180 KB；Liquid 许可证页 | d-d7-lfm-license.html（仅本地）、d-d7-lfm-license.txt |
| 17:03 | https://huggingface.co/api/models/LiquidAI/LFM2.5-24B-A2B | curl | HTTP 200，8.6 KB JSON；license 字段 other，cardData license lfm1.0 | d-d7-hf-api-model.json |
| 17:03 | https://huggingface.co/LiquidAI/LFM2.5-24B-A2B/resolve/main/LICENSE | curl -L | HTTP 200，10.6 KB；LFM Open License v1.0，Threshold 为 $10,000,000 | d-d7-hf-LICENSE.txt |
| 16:58 | https://artificialanalysis.ai/articles/artificial-analysis-decision-index | d-fetch.sh | HTTP 200，555 KB，October 13, 2026，页面注明为 launch 版本（“as of launch”） | d-d8-aa-decision.html（仅本地）、d-d8-aa-decision.txt |
| 17:04 | HN Algolia：按一手 URL 查询（restrictSearchableAttributes=url）（d-heat.py） | urllib | 成功（D1、D2、D3、D4、D6、D7 有命中；D5、D8 无命中） | d-heat.json |
| 17:04 | HN Algolia：标题关键词补查（D5、D8、D2、D4、D7、D3 的查询，内联 Python，结果写 d-heat-search.json） | urllib | D5、D8、D2 无 2026-10-12 之后命中；D4、D7、D3 命中与 URL 查询一致 | d-heat-search.json |

## 二、失败与未取得

1. MSRC 动态页（msrc.microsoft.com/update-guide/…）：返回 SPA 壳，无正文。改用 MSRC 公开 sug 接口与 CVRF 接口取得同一 CVE 的字段，数据来源仍为 MSRC。
2. MSRC API 首次 HTTP/2 请求两次被截断（schannel close_notify），改用 `--http1.1` 后成功；未使用 `-k`，证书校验未关闭。
3. Intel 等未涉及 D 组，未抓取。
4. 谷歌 VRP 一侧（D4）：未找到官方公开记录，记为“未找到”。
5. HN 标题查询 D2 无 2026-10-12 之后命中，D2 的热度数字取自 URL 查询（50073633）。
6. D1 HN：新闻稿 URL 的帖只有 5 分；热度数字较高者指向产品页 https://www.apple.com/apple-homeview/，已在 evidence-d.md 中分别记录。
7. Liquid 原文为 1.8 MB HTML，仅本地保存，不入库；其文本存档 13 KB 入库。
8. 本组未截图（D 组规则）。

## 三、版本与口径说明

- 发布时刻：页面可见文本只给日期时，另查 HTML 元数据（datePublished、article:published_time、dateModified）。Apple dateModified 为 2026-10-13T13:04:15Z（修改时间，非发布时刻）；Anthropic published_time 为 2026-10-13T00:00:00Z（疑为日期占位），dateModified 16:27:06Z；embracethered 2026-10-13T15:00:40-07:00（北京时间 10-14 06:00）；XBOW 2026-10-13T15:20:00Z（北京时间 10-13 23:20）；Meta 2026-10-13T16:00:29Z（北京时间 10-14 00:00:29）；Pi、Liquid、AA 仅日期。MSRC 2026-10-13T07:00:00-07:00（北京时间 22:00）。
- 隐私检查：对 d-* 文件与两份 md 做 QQ、develata、AppData、Users 路径的 grep。命中仅一处：d-d3-embracethered.txt 第 112 行的 `C:\Users\<user>\AppData\...`，为博客原文的占位路径，不含本机信息。d-d5-meta-algorithm.txt 含页面抓取会话追踪参数（URL 内 session_id、last_nav_impression_id），判断为匿名页面会话 ID，与个人身份无关；入库前可去掉。

- AA Decision Index 页面为 launch 版本（“as of launch”），实时榜单会变；取数为 2026-10-14 北京时间 16:58。
- Apple、Anthropic、Meta、XBOW、Pi、Liquid、AA 的官方页只有日期，无发布时刻，写“官方未给时刻”。MSRC 与 HN 有时间戳，按北京时间写。
- HN 数字随取数时刻变化，记录取数时刻。
- “逐字摘句”以 d-verify-quotes.py 核对：67 条，最终重跑 67/67 通过。

## 四、脚本与中间文件

- `d-fetch.sh`：curl 包装（UA、重定向、记录 header），HTML 调用 `d-html2txt.py`。
- `d-html2txt.py`：从 1012 期复制，未改逻辑。
- `d-heat.py`：按 URL 查询 HN Algolia，写 `d-heat.json`。
- `d-heat-search.py`：标题关键词补查。首次为内联 Python 运行，之后保存为本文件并重跑（17:12），结果写 `d-heat-search.json`，两次结果一致。
- `d-verify-quotes.py`：核对 evidence-d.md 中逐字摘句在存档中可 grep 到。
- `d-d3-msrc-product-names.txt`：从 CVRF 抽取的产品名摘录（手工整理，注明出处）。
- scratchpad 中的 `msrc-cvrf-2026-Oct-full.json`（实为 XML，约 40 MB）：会话临时文件，未入库，可删。
