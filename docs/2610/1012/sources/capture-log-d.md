# 1013 期 D 组抓取日志（D1–D10）

时间为北京时间（UTC+8）。抓取时刻 UTC 记录于下表后换算（UTC+8）。工具：`d-fetch.sh`（curl，普通浏览器 UA，匿名，未登录，未使用浏览器会话；HTML 经 `d-html2txt.py` 转文本，脚本从 1012 期复制，逻辑未改）；`gh api` / `gh pr view`（只读调用）；fxtwitter JSON；HN Algolia（`d-heat.py`、`d-heat-search.py`）；WebSearch（仅一次，作 D4 线索）。未运行读取本机 Cookie 的工具，未绕过挑战页、付费墙或登录，未用镜像或转载站作为依据，未在任何站点留下痕迹（无发帖、点赞、star、订阅）。

## 一、抓取记录

| 北京时间 | 目标 | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 10-13 19:26 | https://research.meta.ai/blog/muse-wonder/ | curl（d-fetch.sh） | HTTP 200，333 KB，页面日期 October 12, 2026 | d-d1-meta-wonder.html（原件，仅本地）、d-d1-meta-wonder.txt |
| 10-13 19:26 | https://artificialanalysis.ai/articles/muse-wonder | curl | HTTP 200，358 KB，文章日期 October 12, 2026 | d-d1-aa-wonder.html（仅本地）、d-d1-aa-wonder.txt |
| 10-13 19:26 | https://x.ai/news/x-subscription | curl | HTTP 200，169 KB，日期 Oct 12, 2026 | d-d2-xai-sub.html（仅本地）、d-d2-xai-sub.txt |
| 10-13 19:26 | https://github.com/qgis/QGIS-Enhancement-Proposals/blob/master/qep-408-ai-tool-policy.md | curl | HTTP 429（限流），未用 | d-d3-qep408.html（1.4 KB 错误页，未引用） |
| 10-13 19:27 | https://raw.githubusercontent.com/qgis/QGIS-Enhancement-Proposals/master/qep-408-ai-tool-policy.md | curl | HTTP 200，550 B；与下行 gh api 取得的 master 版本逐字节一致（cmp 核对），作为第二来源 | d-d3-qep408-retry.html（原始响应体，文本，与 d-d3-qep408-master.md 一致） |
| 10-13 19:27 | GitHub API：repos/qgis/QGIS-Enhancement-Proposals/contents/qep-408-ai-tool-policy.md（master，及 ref=d9efd1a3f9、ref=a7ad25831d） | gh api（只读） | 成功，base64 解码为 md | d-d3-qep408-master.md、d-d3-qep408-at-d9efd1a3f9.md、d-d3-qep408-at-a7ad2583.md |
| 10-13 19:28 | GitHub API：pulls/387、commits（path=qep-408，path 对应 130.md） | gh pr view / gh api | 成功 | d-d3-pr387.json、d-d3-pr387-comments.tsv、d-d3-qep408-commits.tsv |
| 10-13 19:27 | https://artificialanalysis.ai/articles/aa-3d | curl | HTTP 200，912 KB（未超 1 MB，仅本地存 html） | d-d5-aa3d.html（仅本地）、d-d5-aa3d.txt |
| 10-13 19:44 | https://artificialanalysis.ai/methodology/aa-3d（文章中链接） | curl | HTTP 200，346 KB；确认 55 个任务与人类偏好投票 | d-d5-aa3d-methodology.html（仅本地）、d-d5-aa3d-methodology.txt |
| 10-13 19:44 | https://www.gamesindustry.biz/epic-games-delays-disney-game-and-tells-developers-to-go-ai-first | curl | HTTP 200，207 KB；转述 Bloomberg，引 Epic 通讯总监 Jake Jones 声明 | d-d8-gamesindustry.html（仅本地）、d-d8-gamesindustry.txt |
| 10-13 19:44 | HN Algolia：Epic/Disney 关键词搜索（d-heat-d8.py） | curl/urllib | 成功；Bloomberg 链接的 HN 投稿标题与 URL 一致 | d-heat-d8.json |
| 10-13 19:45 | intel.com 新闻稿 https://www.intel.com/content/www/us/en/newsroom/news/artificial-intelligence/intel-accelerates-open-infrastructure-agentic-ai-era.html 与 …/intel-to-expand-ai-accelerator-portfolio-with-new-gpu.html（WebSearch 限定 intel.com 域所得 URL） | curl | 两条均 HTTP 403（bot 拦截），未绕过 | d-d4-intel-newsroom-agentic.html、d-d4-intel-newsroom-expand.html（错误页，未引用） |
| 10-13 19:45 | 本期 daily-scan.md 文末“近 3 天速览已收”与“已发过”（本地文件，grep 与 sed 读取） | 本地读取 | 与 D1–D10 无重合（最近 3 天条目：Vals、Sakana、凯奇等；“已发过”含 1006 OpenAI 数学手稿，D9 为其跟进） | 未存档（本地文件） |
| 10-13 19:26 | https://artificialanalysis.ai/articles/frontiercode | curl | HTTP 200，475 KB | d-d6-aa-frontiercode.html（仅本地）、d-d6-aa-frontiercode.txt |
| 10-13 19:26 | https://arxiv.org/abs/2610.13928 | curl | HTTP 200，42 KB，Submitted 9 Oct 2026 | d-d7-arxiv.html（仅本地）、d-d7-arxiv.txt |
| 10-13 19:26 | http://export.arxiv.org/api/query?id_list=2610.13928（经 https 重定向） | curl | HTTP 200，published 2026-10-09T17:55:06Z | d-d7-arxiv-api.html（实为 Atom XML，2.8 KB，文本，可入库前与 Claude 确认） |
| 10-13 19:28 | https://arxiv.org/html/2610.13928v1 | curl | HTTP 200，296 KB，作者单位与正文 | d-d7-arxiv-html.html（仅本地）、d-d7-arxiv-html.txt |
| 10-13 19:27 | https://www.argmin.net/p/faster-fourier-transforms | curl | HTTP 200，207 KB，Oct 12, 2026 | d-d9-argmin.html（仅本地）、d-d9-argmin.txt |
| 10-13 19:27 | https://github.com/openai/math/blob/main/lean/docs/130.md | curl | HTTP 200，242 KB，main 分支 | d-d9-openai-130.html（仅本地）、d-d9-openai-130.txt |
| 10-13 19:28 | https://github.com/openai/math/commits（path=lean/docs/130.md，gh api） | gh api（只读） | 最近提交 2026-10-08T05:03:50Z | d-d9-openai-130-commits.tsv |
| 10-13 19:27 | https://yyhh.org/blog/2026/10/ai-is-a-tool/ | curl | HTTP 200，17 KB，Mon 12 October 2026 | d-d10-yyhh.html（仅本地）、d-d10-yyhh.txt |
| 10-13 19:29 | https://www.linkedin.com/pulse/ai-tool-rich-hickey-kjl8e | curl（匿名，未登录；页面含登录提示，正文可见） | HTTP 200，157 KB，“Published Oct 12, 2026”，作者 Rich Hickey | d-d10-hickey-linkedin.html（仅本地）、d-d10-hickey-linkedin.txt |
| 10-13 19:29 | https://arstechnica.com/gaming/2026/10/report-epic-games-goes-all-in-on-ai-for-disney-collaboration/ | curl | HTTP 200，151 KB，页面显示 Oct 12, 2026 4:59 pm（未标时区） | d-d8-arstech.html（仅本地）、d-d8-arstech.txt |
| 10-13 19:30 | https://api.fxtwitter.com/TimSweeneyEpic/status/2109705826688549304 | curl（fxtwitter 公开 JSON） | HTTP 200，created Mon Oct 12 18:00:40 +0000 2026，文本已核；无 replying_to 字段，未展开后续线程（回复数 4088） | d-d8-sweeney-fx.json（JSON 文本，可入库） |
| 10-13 19:30 | https://www.bloomberg.com/news/articles/2026-10-12/epic-games-delays-disney-project-asks-developers-to-use-more-ai | curl | **HTTP 403，页面标题 “Bloomberg - Are you a robot?”（bot 挑战页）；未绕过，未取得正文** | d-d8-bloomberg.html（挑战页，仅本地，未引用） |
| 10-13 19:26 | https://www.intel.com/content/www/us/en/products/details/discrete-gpus/data-center-gpu/d-series.html | curl | **HTTP 403；未取得正文，未绕过** | d-d4-intel-d270.html（497 B 错误页，未引用） |
| 10-13 19:32 | WebSearch：“Intel D270 GPU 480GB LPDDR5X inference announcement” | WebSearch（一次） | 结果无名为 D270 的产品，统称 Crescent Island；仅二手报道 | d-d4-websearch-intel.txt（摘要，非原文） |
| 10-13 19:29 | HN Algolia：story_<id>（D1、D3、D4、D7、D9、D10a、D10b） | d-heat.py | 成功，取数 19:29 | d-heat.json |
| 10-13 19:32 | HN Algolia：关键词（D2、D5、D6、D8、D4、D9、D10、D1、D7） | d-heat-search.py | 成功，取数 19:32；D2、D5 无相关命中 | d-heat-search.json |

## 二、失败与未取得

1. Intel 官方 D270 页（403）：未取得原文。D4 状态为“未找到一手来源”。
2. Bloomberg 原报道（403 挑战页）：未取得原文。D8 中“据彭博社报道”的内容仅经 Ars 转述，已标注。
3. QGIS QEP blob 页（429）与 raw 地址（错误页）：改用 `gh api` 取同一文件，内容已核。
4. HN 关键词无命中：D2（xAI 合并订阅）、D5（AA-3D）。非抓取失败。
5. Ars 时间戳未标时区（“Oct 12, 2026 4:59 pm”），北京时间未换算。
6. Hickey 原文以 LinkedIn 匿名访客视图取得，页面要求登录才能看评论；正文可读，评论未存。

## 三、版本与口径说明

- OpenAI 的 130.md 取 `main` 分支地址（非固定 commit），仅用于记录当时可见内容；文件最近提交 2026-10-08T05:03:50Z（`d-d9-openai-130-commits.tsv`）。
- QEP-408 取 master 版本（`d-d3-qep408-master.md`，header 写 Version 2.0，Date 2026/09/24）；旧版见 `d-d3-qep408-at-d9efd1a3f9.md`。
- AA 页面数据为“as at 12 October 2026”，实时榜单会变化。
- HN 数字随取数时刻变化，记录取数时刻。

## 四、脚本与中间文件

- `d-fetch.sh`：curl 包装（UA、重定向、记录 header），HTML 调用 `d-html2txt.py`。
- `d-html2txt.py`：从 1012 期复制，未改逻辑。
- `d-heat.py`、`d-heat-search.py`：HN Algolia 查询。
- `d-verify-quotes.py`：核对 evidence-d.md 中逐字摘句在存档中可 grep 到（51 条，全部 OK）。
- 本地试验文件：无。

## 五、入库提示（供 Claude 事后合并）

- 文本存档（.txt、.md、.json、.xml、.tsv、.headers）可入库；`.html` 原件只存本地（`d-d8-sweeney-fx.json` 与 `d-d7-arxiv-api.xml` 已按内容改为正确扩展名，可入库）。
- 单个文件均未超过 1 MB（最大 `d-d5-aa3d.html`，912 KB，为原件，仅本地）。
- 本组未生成任何截图（D 组不截图）。
- 本组未改动 `docs/2610/1013/` 以外的文件。
