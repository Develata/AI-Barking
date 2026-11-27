# 1126 期 C 组抓取与热度日志

执行：Antigravity 取证子代理（C 组）。北京时间 11-27 21:12–21:24。
工具：`barking fetch`（stdin 清单批量，日志 `c-fetch-log.jsonl`）、`barking heat`（HN 条目、关键词与 Reddit，日志 `c-heat.json`、`c2-heat.json`、`c3-heat.json`）、`opencli browser`（session `c-reddit`、`c-reddit-ts`、`c-hy3d`，后台窗口，只读不登录）、`search_web`（发现 URL）与 Python 一次性重定向解析。
未截图（C 组不需要）。未使用 firecrawl。未绕过付费墙或登录。存档中没有抓取者身份信息（经 `rg` 校验 `ZERO_PERSONAL_INFO`）。

## 一、批量存档（`barking fetch`）

### 第一批（21:14:35–21:14:41，共 27 条：ok 19，blocked 5，failed 3）

| 时间（北京） | 名称 | URL | 结果 | 说明 |
|---|---|---|---|---|
| 21:14:35 | c1-deepseek-pricing | https://api-docs.deepseek.com/zh-cn/quick_start/pricing/ | 200 ok | 中文官方 API 定价页（峰谷时段定义） |
| 21:14:35 | c1-deepseek-pricing-en | https://api-docs.deepseek.com/quick_start/pricing | 200 ok | 英文定价页 |
| 21:14:35 | c1-deepseek-updates | https://api-docs.deepseek.com/zh-cn/updates/ | 200 ok | 更新日志（最新至 2026-11-13） |
| 21:14:36 | c1-ithome | https://www.ithome.com/1/033/157.htm | 200 ok | IT之家 11-27 08:21 报道 |
| 21:14:37 | c1-ifeng-tech | https://tech.ifeng.com/c/8yO4BybuYcl | 200 ok | 凤凰网大风号转 IT之家 11-27 08:33 |
| 21:14:37 | c1-ifeng-orig | https://original.ifeng.com/c/8yOj7tdntji | 200 ok | 凤凰网科技实测 11-27 18:09 |
| 21:14:36 | c2-neuralink-update | https://neuralink.com/updates/neuralink-receives-fda-approval-to-launch-link-pivotal-study/ | 200 ok | Neuralink 官方公告（Nov 25, 2026） |
| 21:14:36 | c2-neuralink-trials | https://neuralink.com/trials/ | 200 ok | 临床试验说明（写明尚未获 FDA 批准） |
| 21:14:36 | c3-macrumors | https://www.macrumors.com/2026/11/26/apple-korea-regulator-iphone-16-ai-advertising/ | 200 ok | MacRumors 报道，含苹果声明 |
| 21:14:37 | c3-koreatimes | https://www.koreatimes.co.kr/amp/business/companies/20261126/apple-korea-faces-potential-50-mil-fine-over-deceptive-iphone-ai-ads | 202 blocked | 无正文（被拦截），见待删除 |
| 21:14:37 | c3-chosun | https://www.chosun.com/english/market-money-en/2026/11/26/MY5HPJMQH5D6DPP6IPQNYT5KYQ/ | 200 blocked | 正文过短（JS 渲染），见待删除 |
| 21:14:38 | c3-businesskorea | https://www.businesskorea.co.kr/news/articleView.html?idxno=282367 | 200 ok | BusinessKorea 报道，含 268–536 亿韩元推算 |
| 21:14:38 | c4-androidauthority | https://www.androidauthority.com/google-gemini-chrome-ram-hogging-issue-fixed-3739606/ | 200 ok | Android Authority 报道 |
| 21:14:38 | c4-reddit-json | https://www.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage.json | 403 blocked | HTTP 403，改用 opencli，见待删除 |
| 21:14:38 | c4-reddit-old | https://old.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage/ | 200 blocked | 登录墙/JS，改用 opencli，见待删除 |
| 21:14:39 | c5-reuters | https://www.reuters.com/technology/sap-considers-least-10000-job-cuts-ai-restructuring-bloomberg-reports-2026-11-26/ | 401 blocked | HTTP 401，改采 CNA/SBS，见待删除 |
| 21:14:39 | c5-cna | https://www.channelnewsasia.com/business/sap-job-cuts-ai-restructuring-6585781 | 200 ok | CNA 报道，含 SAP 原文回应 |
| 21:14:39 | c6-yandex-hf | https://huggingface.co/yandex/AliceAI-Foundation-80B-A3B-Base | 200 ok | HF 模型卡（俄文主卡） |
| 21:14:39 | c6-yandex-hf-api | https://huggingface.co/api/models/yandex/AliceAI-Foundation-80B-A3B-Base | 200 ok | HF API 元数据（createdAt 09-12） |
| 21:14:40 | c6-yandex-arxiv | https://arxiv.org/abs/2611.34529 | 200 ok | arXiv 摘要与提交时刻（11-25 15:14Z） |
| 21:14:40 | c7-github-tree | https://github.com/google-research/google-research/tree/4a7bc8ab0bc770e71a259b96b99010a/cogentic_research_papers/integer_factorization | 404 failed | 派工单 hash 缺段，见待删除 |
| 21:14:40 | c7-github-readme | https://raw.githubusercontent.com/.../4a7bc8ab0bc770e71a259b96b99010a/.../README.md | 404 failed | 同上，见待删除 |
| 21:14:40 | c7-github-parent-readme | https://raw.githubusercontent.com/.../4a7bc8ab0bc770e71a259b96b99010a/cogentic_research_papers/README.md | 404 failed | 同上，见待删除 |
| 21:14:41 | c8-kimi-science | https://www.kimi.com/news/kimi-for-science-program | 200 ok | Kimi for Science 官网页 |
| 21:14:41 | c8-3dcodefactory-hf | https://huggingface.co/datasets/3DCodeFactory/3DCodeFactory | 200 ok | HF 数据集卡 |
| 21:14:41 | c8-3dcodefactory-hf-api | https://huggingface.co/api/datasets/3DCodeFactory/3DCodeFactory | 200 ok | HF 数据集 API JSON |
| 21:14:41 | c8-3dcodefactory-arxiv | https://arxiv.org/abs/2611.28783 | 200 ok | arXiv 摘要（11-21 提交） |

### 第二至六批（21:15:04–21:19:58，共 20 条：ok 15，blocked 4，failed 1）

| 时间（北京） | 名称 | URL | 结果 | 说明 |
|---|---|---|---|---|
| 21:15:04 | c7-github-tree-ok | https://github.com/google-research/google-research/tree/4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/integer_factorization | 200 ok | 修正 40 位 hash 后成功 |
| 21:15:04 | c7-github-readme-ok | https://raw.githubusercontent.com/.../4a7bc8ab0bc770e71a259b7966c500496b99010a/.../README.md | 200 ok | README 原文 |
| 21:15:05 | c7-github-parent-readme-ok | https://raw.githubusercontent.com/.../4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/README.md | 200 ok | 论文列表页 |
| 21:15:05 | c3-korea-docviewer | https://www.korea.kr/common/docViewer.do?fileId=198597019&tblKey=GMN | 200 blocked | JS 在线查看器，改取新闻稿页与 PDF，见待删除 |
| 21:15:06 | c5-sbs | https://news.sbs.co.kr/english/article.do?news_id=N1008866096 | 200 ok | SBS 英文网 11-27 08:17 KST |
| 21:15:37 | c3-korea-press | https://www.korea.kr/briefing/pressReleaseView.do?newsId=156790945 | 200 ok | 韩国政策简报网 KFTC 新闻稿页面 |
| 21:15:37 | c7-paper-pdf | https://raw.githubusercontent.com/.../4a7bc8ab0bc770e71a259b7966c500496b99010a/.../paper.pdf | 200 ok | 论文 PDF（1,072,968 B，不入库；txt 4443 行） |
| 21:15:38 | c2-neuralink-device-control | https://neuralink.com/trials/device-control/ | 200 ok | Telepathy 试验详情 |
| 21:15:39 | c6-yandex-hf-en | https://huggingface.co/yandex/AliceAI-Foundation-80B-A3B-Base/raw/main/README_en.md | 200 ok | 英文版模型卡 |
| 21:16:03 | c3-korea-press-pdf | https://www.korea.kr/common/download.do?fileId=198597020&tblKey=GMN | 200 ok | KFTC 官方新闻稿 PDF（不入库；txt 77 行） |
| 21:16:03 | c6-yandex-habr | https://habr.com/ru/companies/yandex/articles/1083300/ | 200 ok | Yandex 官方 Habr 博文（09-21 发表，11-25 修改） |
| 21:16:03 | c7-github-commit | https://api.github.com/repos/google-research/google-research/commits/4a7bc8ab0bc770e71a259b7966c500496b99010a | 200 ok | commit 时间戳 `2026-11-25T10:21:27Z` |
| 21:16:03 | c8-kimi-news | https://www.kimi.com/news/ | 200 ok | Kimi 资讯列表页（显示 11-26 日期） |
| 21:16:04 | c8-3dcodefactory-gh | https://github.com/3DCodeFactory/3DCodeFactory | 200 ok | 3DCodeFactory GitHub 仓库 |
| 21:16:05 | c1-deepseek-www | https://www.deepseek.com/ | 200 ok | DeepSeek 首页 |
| 21:16:25 | c2-neuralink-fx | https://api.fxtwitter.com/status/1993382759885873200 | failed | `io: unexpected end of file` 连接失败（该 ID 也偏旧未用），见待删除 |
| 21:17:14 | c1-deepseek-news | https://www.deepseek.com/news/ | 200 ok | DeepSeek 官方动态页（最新 2026-09-10） |
| 21:17:14 | c8-kimi-game | https://www.kimi.com/news/kimi-game-solutions | 200 ok | Kimi 游戏开发与创意工作解决方案 |
| 21:17:15 | c8-3dcodefactory-commit | https://api.github.com/repos/3DCodeFactory/3DCodeFactory/commits/main | 200 ok | commit 时间 `2026-11-23T12:21:12Z` |
| 21:18:05 | c6-yandex-habr-comments | https://habr.com/ru/companies/yandex/articles/1083300/comments/ | 200 ok | Habr 评论页（HTML 1,457,508 B，不入库；证实 9 月 21 日首发） |
| 21:18:06 | c8-hn-algolia-top | https://hn.algolia.com/api/v1/search?tags=front_page&hitsPerPage=30 | 200 ok | HN 首页榜单快照（用于 C8 选题排查） |
| 21:18:48 | c8-guardian-pizza | https://www.theguardian.com/business/2026/nov/27/ai-agent-cardless-transaction-pizza-papa-john-uk-fca-truelayer | 200 ok | 《卫报》报道 |
| 21:18:49 | c8-hy3d-pandaily | https://pandaily.com/tencent-hunyuan-hy3d-3-6-professional-3d-assets | 200 blocked | JS/拦截，见待删除 |
| 21:18:49 | c8-doubao-aibase | https://www.aibase.com/zh/news/32576 | 200 ok | 聚合站 L5 仅线索查证，见待删除 |
| 21:18:50 | c8-hy3d-ifeng | https://tech.ifeng.com/c/8yOKHwwsCrm | 200 ok | 凤凰网大风号转 IT之家 Hy3D 3.6 报道 |
| 21:19:10 | c8-hy3d-cloud | https://cloud.tencent.com/product/ai3d | 200 ok | 腾讯云混元生 3D 产品页 |
| 21:19:10 | c8-hy3d-official | https://3d.hunyuan.tencent.com/ | 200 blocked | JS 渲染且需登录，见待删除 |
| 21:19:58 | c8-hy3d-doc-intro | https://cloud.tencent.com/document/product/1804/120696 | 200 ok | 腾讯云混元生 3D 产品概述（写明迁至 TokenHub） |
| 21:19:58 | c8-hy3d-doc-api | https://cloud.tencent.com/document/product/1804/120838 | 200 ok | 腾讯云 API 概览（未见 3.6） |
| 21:19:58 | c8-kimi-news-en | https://www.kimi.com/en/news/kimi-for-science-program | 200 ok | 跳回中文版，与 `c8-kimi-science` 重复，见待删除 |

## 二、opencli 浏览器取证（只读，不登录）

| 时间（北京） | session | URL | 结果 | 存档 |
|---|---|---|---|---|
| 21:17 | c-reddit | https://www.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage/ | 读取页面正文，取得 `GoogleHelpCommunity`（Official Google Support）回复原文 | `c-c4-reddit-opencli.txt` |
| 21:18 | c-reddit-ts | https://www.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage/ | 提取 DOM `created-timestamp` 与 `time[datetime]` 精确 UTC 时刻 | `c-c4-reddit-meta.json` |
| 21:19 | c-hy3d | https://3d.hunyuan.tencent.com/ | 页面显示“登录后开启3D创作之旅”，未登录 | `c-c8-hy3d-opencli.txt` |

## 三、热度（`barking heat`）

### 第一批（`c-heat.json`，取得 11-27 21:13:09 北京）
- `50651627`（C7 单篇）：Unconditional Integer Factorization in Subexponential Time，152 分 / 27 评（11-26 12:59 北京提交）→ `c-hn-50651627.json`
- `50642086`（C7 整批目录）：Google releases Cogentic papers on integer factorization, discrete logs and more，511 分 / 372 评（11-25 21:33 北京提交）→ `c-hn-50642086.json`
- 关键词 `Neuralink LINK`：`50654405`，3 分 / 0 评（11-26 20:06 北京）
- 关键词 `SAP 10,000`：`50658287`，7 分 / 3 评（11-27 02:46 北京）
- 关键词 `AliceAI-Foundation`、`KFTC Apple`、`3DCodeFactory`：0 命中
- 关键词 `Gemini memory`：无相关命中
- 关键词 `DeepSeek`：命中 13 条，无与灰度卡片相关命中

### 第二批（`c2-heat.json`，取得 11-27 21:17:20 北京）
- 关键词 `Kimi for Science`：`50665920`，10 分 / 1 评（11-27 20:03 北京）
- 关键词 `Cogentic`：`50642086`（511 分 / 372 评）、`50651627`（153 分 / 27 评）、`50641847`（15 分 / 1 评）等
- 关键词 `Kimi game`、`3DCodeFactory`：0 命中；`Hy3D`、`AliceAI`：无相关命中

### 第三批（`c3-heat.json`，取得 11-27 21:20:07 北京）
- HN 单帖存档：`c3-hn-50654405.json`（3 分 / 0 评）、`c3-hn-50658287.json`（7 分 / 3 评）、`c3-hn-50665920.json`（10 分 / 1 评）、`c3-hn-50665732.json`（10 分 / 11 评）。
- Reddit 搜索（`Neuralink LINK`、`SAP 10000`、`AliceAI-Foundation`、`Cogentic factorization`）：均跳转 `old.reddit.com/login/` 登录墙返回 HTML，已如实记失败。

## 四、待删除文件清单（精确路径，工作中未调用 `rm`）

- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c2-neuralink-fx.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c3-chosun.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c3-chosun.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c3-korea-docviewer.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c3-korea-docviewer.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c3-koreatimes.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c3-koreatimes.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c4-reddit-json.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c4-reddit-json.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c4-reddit-old.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c4-reddit-old.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c5-reuters.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c5-reuters.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c7-github-parent-readme.bin`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c7-github-parent-readme.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c7-github-readme.bin`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c7-github-readme.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c7-github-tree.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c7-github-tree.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-doubao-aibase.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-doubao-aibase.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-doubao-aibase.txt`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-hy3d-official.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-hy3d-official.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-hy3d-pandaily.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-hy3d-pandaily.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-kimi-news-en.headers`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-kimi-news-en.html`
- `/Users/develata/gitclone/AI-Barking/docs/2611/1126/sources/c-c8-kimi-news-en.txt`

## 五、文件大小与不入库原件核查

- **超过 1 MB 的文件（共 2 个，均属不入库原件，无 txt/md/json 超 1 MB）**：
  - `c-c6-yandex-habr-comments.html`（1,457,508 B）
  - `c-c7-paper-pdf.pdf`（1,072,968 B）
- **不入库原件**：全部 `c-*.html` 与 `c-*.pdf`（`c-c3-korea-press-pdf.pdf`、`c-c7-paper-pdf.pdf`）按约定只存本地与网盘，不入 Git 仓库。
- **隐私检查**：`rg -i "develata|/Users/develata" docs/2611/1126/sources/c*` 输出为空，无抓取者个人信息泄露。
