# 0928 抓取日志

北京时间 UTC+8。逐文件时间采用写入完成时的 LastWriteTimeUtc + 8；它是抓取/处理完成近似时间，不冒充请求发起时间。curl 请求开始时间、HTTP状态与重定向另见 http-capture.tsv。派生图记录加工时间，原始截图时间另列。

## 工具与失败边界

- opencli 1.8.7；已执行 browser --help、doctor；浏览器扩展连接成功。agent-reach doctor --json 后按可用 OpenCLI 后端访问 X/Reddit/B站。普通网页用 curl.exe 与 opencli browser；PDF 用 pdftotext / pdftoppm，图像用 PowerShell System.Drawing 1:1 裁剪拼接。未安装软件、未登录、未输入凭据、未接受非必要 Cookie。
- Inc 和 Axios 的 curl HTML 返回403，保留响应体；同一原站浏览器可读，另存逐字 Markdown 与浏览器 HTML。Inc 点击原页面 Expand to continue reading 后保存 expanded 版本；前一版本不冒充全文。
- 两个 Threads 原链接均仅见登录提示，失败；高管回应只能由 Inc 内嵌原图转录，单独标媒体截图。WSJ 仅抓到标题、日期、导语和开头，未取到全文。
- Anthropic full-page 截图发生视口高度/布局重排，出现黑区或错位；a-sonnet-full*.png 及 a-footnotes*、a-availability*、a-end-crop、a-crop-test 等保留为未采用尝试。最终用可见视口截图裁剪；02采用 a-safeguards-direct 和 a-notes-good。
- b-reddit-metrics.json 搜索没有命中目标，不用于目标热度；后续 b-reddit-dom-metrics.json 从目标帖 DOM 取得 score/comments。c-before-report-detail.json 正文被接口截断，不作为全文。
- a-font-audit.json / c-font-audit.json 返回空数组（会话页面已变化），不构成字号验收。d-9to5mac-time.json 未取得时间，使用可见页头。PDF字体提取有乱码，作者名按原页图转录；render-errors 文件保留渲染警告。
- Python 调用曾被自动审批机制拒绝：approval required by policy, but AskForApproval is set to Never。改用现有 PowerShell/.NET 完成，没有申请安装或绕过审批。

## 截图规则与未满足项

14张PNG全部为原站/原PDF裁剪，未重绘、改字、调色、缩放；拼接线为2px灰线，空白补齐宽度。精确像素坐标在 crop-captures.ps1。浏览器默认100%缩放；浅色正文，但 Anthropic 标题区沿用原站深色底，未强制改站点样式。MiMo表格原生小字部分可能低于14px，本轮未能证明所有引文字高均≥14px，因此保留为候选图，不声称图像规格全部验收。Inc标题在原页面有自然重叠，截图忠实保留。CASP网页无日级日期，03不补造；PDF封面只有September2026。

## 全部源文件

“成功/原始返回”仅表示取得文件；全文性及失败响应按上方和 evidence.md 判断。HTML外链资源未打包，不能保证离线复原外观；可读Markdown另存。原文Markdown可能带浏览器提取的结构标记，不是改写正文。

| 文件 | URL | 完成时间（北京） | 工具/结果 |
|---|---|---|---|
| a-aa-body.md | https://artificialanalysis.ai/models/claude-sonnet-5-5 | 2026-09-29 02:57:13 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-aa-home.json | https://artificialanalysis.ai/ | 2026-09-29 02:55:15 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-aa-home.md | https://artificialanalysis.ai/ | 2026-09-29 02:55:15 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-aa-model.html | https://artificialanalysis.ai/models/claude-sonnet-5-5 | 2026-09-29 02:56:16 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| a-aa-model.json | https://artificialanalysis.ai/models/claude-sonnet-5-5 | 2026-09-29 02:56:14 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-aa-model.md | https://artificialanalysis.ai/models/claude-sonnet-5-5 | 2026-09-29 02:56:14 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-aa-network.json | https://artificialanalysis.ai/models/claude-sonnet-5-5 | 2026-09-29 03:00:27 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-aa-schema.json | https://artificialanalysis.ai/models/claude-sonnet-5-5 | 2026-09-29 03:00:28 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-aa-tb-body.md | https://artificialanalysis.ai/evaluations/terminalbench-4-0 | 2026-09-29 03:01:30 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-aa-tb.html | https://artificialanalysis.ai/evaluations/terminalbench-4-0 | 2026-09-29 03:03:44 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| a-aa-tb.json | https://artificialanalysis.ai/evaluations/terminalbench-4-0 | 2026-09-29 03:01:29 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-aa-tb.md | https://artificialanalysis.ai/evaluations/terminalbench-4-0 | 2026-09-29 03:01:29 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-availability-direct.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:03:14 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-availability-tall.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:03:40 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-benchmark.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:59:22 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-crop-test.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:01:50 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-cyber.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:59:24 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-end-crop.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:04:48 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-font-audit.json | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:12:43 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-footnotes-anchor.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:02:39 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-footnotes-end.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:00:58 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-footnotes-final.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:01:21 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-footnotes.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:59:28 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-hn-item.json | https://news.ycombinator.com/item?id=49881850 | 2026-09-29 03:08:06 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-hn-search.json | https://news.ycombinator.com/item?id=49881850 | 2026-09-29 02:56:37 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-notes-default.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:04:51 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-notes-good.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:05:07 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-safeguards-direct.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:03:07 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-safety-anchor.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:02:32 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-sonnet-extract.json | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:51:10 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| a-sonnet-full-settled.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 03:01:03 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-sonnet-full.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:51:17 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-sonnet-viewport.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:58:31 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| a-sonnet.html | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:51:43 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| a-sonnet.md | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:51:10 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-system-card.html | https://www.anthropic.com/claude-sonnet-5-5-system-card | 2026-09-29 02:51:46 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| a-system-card.pdf | https://www.anthropic.com/claude-sonnet-5-5-system-card | 2026-09-29 02:51:46 UTC+8 | curl 原PDF；成功 |
| a-system-card.txt | https://www.anthropic.com/claude-sonnet-5-5-system-card | 2026-09-29 02:53:46 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| a-title.png | https://www.anthropic.com/claude-sonnet-5-5 | 2026-09-29 02:59:15 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| b-author-thread.json | https://x.com/_achan96_/status/2104578598115885530 | 2026-09-29 03:03:52 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-author-x.json | https://x.com/_achan96_/status/2104578598115885530 | 2026-09-29 02:59:34 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-authors-02.png | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 03:04:26 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| b-authors-render-errors.txt | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 03:04:26 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-authors-transcription.md | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 03:06:51 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-axios-extract.json | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 02:56:50 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-axios-schema.json | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 03:00:29 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-axios.html | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 02:51:49 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| b-axios.md | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 02:56:50 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-casp-extract.json | https://casp.ac/reports/intelligence-explosion | 2026-09-29 02:52:14 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-casp-viewport.png | https://casp.ac/reports/intelligence-explosion | 2026-09-29 02:52:21 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| b-casp.html | https://casp.ac/reports/intelligence-explosion | 2026-09-29 02:51:47 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| b-casp.md | https://casp.ac/reports/intelligence-explosion | 2026-09-29 02:52:14 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-guardian-extract.json | https://www.theguardian.com/technology/2026/sep/28/ai-godfathers-warn-of-runaway-intelligence-explosion | 2026-09-29 02:55:47 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-guardian-schema.json | https://www.theguardian.com/technology/2026/sep/28/ai-godfathers-warn-of-runaway-intelligence-explosion | 2026-09-29 03:00:30 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-guardian.html | https://www.theguardian.com/technology/2026/sep/28/ai-godfathers-warn-of-runaway-intelligence-explosion | 2026-09-29 02:51:48 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| b-guardian.md | https://www.theguardian.com/technology/2026/sep/28/ai-godfathers-warn-of-runaway-intelligence-explosion | 2026-09-29 02:55:47 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-hn-item.json | https://news.ycombinator.com/item?id=49879079 | 2026-09-29 03:08:06 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-hn-search.json | https://news.ycombinator.com/item?id=49879079 | 2026-09-29 02:56:38 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-paper-reading.txt | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 03:05:49 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-paper-render-04.png | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 02:55:37 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| b-paper-render-05.png | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 02:55:38 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| b-paper-render-06.png | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 02:55:39 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| b-paper-utf8.txt | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 02:54:20 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-paper.pdf | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 02:52:54 UTC+8 | curl 原PDF；成功 |
| b-paper.txt | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf | 2026-09-29 02:53:47 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-reddit-dom-metrics.json | https://www.reddit.com/r/technology/comments/1wshicc/ | 2026-09-29 03:12:20 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-reddit-metrics.json | https://www.reddit.com/r/technology/comments/1wshicc/ | 2026-09-29 03:07:23 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-reddit.json | https://www.reddit.com/r/technology/comments/1wshicc/ | 2026-09-29 02:53:19 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-render-errors.txt | https://casp.ac/reports/intelligence-explosion | 2026-09-29 02:55:38 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| b-wsj-extract.json | https://www.wsj.com/tech/ai/top-ai-researchers-call-for-urgent-oversight-of-self-improving-systems-49bae9b4 | 2026-09-29 02:56:58 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-wsj-time.json | https://www.wsj.com/tech/ai/top-ai-researchers-call-for-urgent-oversight-of-self-improving-systems-49bae9b4 | 2026-09-29 03:00:31 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| b-wsj.md | https://www.wsj.com/tech/ai/top-ai-researchers-call-for-urgent-oversight-of-self-improving-systems-49bae9b4 | 2026-09-29 02:56:58 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| c-before-report-detail.json | https://www.reddit.com/r/LocalLLaMA/comments/1woa5d3/ | 2026-09-29 03:12:27 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-before-reports.json | https://www.reddit.com/search/?q=mimo%20loop | 2026-09-29 03:08:05 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-font-audit.json | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition | 2026-09-29 03:12:42 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-mimo-extract.json | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition | 2026-09-29 02:52:39 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-mimo-full.png | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition | 2026-09-29 02:54:23 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| c-mimo.html | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition | 2026-09-29 02:51:50 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| c-mimo.md | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition | 2026-09-29 02:52:40 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| c-opencode-search.json | https://www.reddit.com/r/opencodeCLI/comments/1wrtyfb/ | 2026-09-29 03:01:57 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-opencode-thread.json | https://www.reddit.com/r/opencodeCLI/comments/1wrtyfb/ | 2026-09-29 03:03:58 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-reddit-metrics.json | https://www.reddit.com/r/LocalLLaMA/comments/1wrq71o/ | 2026-09-29 03:07:41 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-reddit.json | https://www.reddit.com/r/LocalLLaMA/comments/1wrq71o/ | 2026-09-29 02:53:25 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| c-x.json | https://x.com/XiaomiMiMoDevs/status/2104251067039191324 | 2026-09-29 02:53:48 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| crop-captures.ps1 | 本地整理；原始URL见关联存档 | 2026-09-29 03:16:45 UTC+8 | 本地处理脚本 |
| d-9to5mac-extract.json | https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/ | 2026-09-29 02:56:36 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-9to5mac-time.json | https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/ | 2026-09-29 03:05:10 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-9to5mac.html | https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/ | 2026-09-29 02:51:59 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| d-9to5mac.md | https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/ | 2026-09-29 02:56:36 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-download-extract.json | https://ai.meta.com/muse/download/ | 2026-09-29 02:57:12 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-download.html | https://ai.meta.com/muse/download/ | 2026-09-29 02:51:59 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| d-download.md | https://ai.meta.com/muse/download/ | 2026-09-29 02:57:12 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-inc-body.md | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:54:01 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-inc-browser.html | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:54:02 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| d-inc-expanded.html | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:55:36 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| d-inc-expanded.json | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:55:08 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-inc-expanded.md | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:55:08 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-inc-extract.json | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:53:10 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-inc-full.png | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:55:34 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| d-inc-schema.json | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 03:05:09 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-inc-singleton.webp | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 03:04:26 UTC+8 | curl 原站嵌图；成功 |
| d-inc.html | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:51:51 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| d-inc.md | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 | 2026-09-29 02:53:10 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-meta-extract.json | https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/ | 2026-09-29 02:53:40 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-meta-full.png | https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/ | 2026-09-29 02:54:25 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| d-meta.html | https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/ | 2026-09-29 02:51:58 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| d-meta.md | https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/ | 2026-09-29 02:53:40 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-reddit-metrics.json | https://www.reddit.com/r/privacy/comments/1wo4vzy/ | 2026-09-29 03:07:47 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-reddit.json | https://www.reddit.com/r/privacy/comments/1wo4vzy/ | 2026-09-29 02:53:30 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-response-links.json | https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/ | 2026-09-29 03:12:28 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-singleton-bug.json | https://www.threads.com/@davidsingleton/post/Ddml0eHEj3E | 2026-09-29 02:59:41 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-singleton-bug.md | https://www.threads.com/@davidsingleton/post/Ddml0eHEj3E | 2026-09-29 02:59:41 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-singleton-extract.json | https://www.threads.com/share/JEVmarjjh/ | 2026-09-29 02:58:39 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| d-singleton-image-transcription.md | https://www.threads.com/share/JEVmarjjh/ | 2026-09-29 03:06:51 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| d-singleton.md | https://www.threads.com/share/JEVmarjjh/ | 2026-09-29 02:58:39 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-a-bilibili-browser.json | https://www.bilibili.com/video/BV1bZac6NEdd/ | 2026-09-29 03:02:07 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| e-a-bilibili-full.png | https://www.bilibili.com/video/BV1bZac6NEdd/ | 2026-09-29 03:02:08 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| e-a-bilibili.html | https://www.bilibili.com/video/BV1bZac6NEdd/ | 2026-09-29 03:06:53 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| e-a-bilibili.md | https://www.bilibili.com/video/BV1bZac6NEdd/ | 2026-09-29 03:02:07 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-a-bilibili.txt | https://www.bilibili.com/video/BV1bZac6NEdd/ | 2026-09-29 03:00:32 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-a-meta.json | https://www.bilibili.com/video/BV1bZac6NEdd/ | 2026-09-29 03:05:53 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| e-b-axios-full.png | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 03:02:41 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| e-b-axios.html | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 03:06:57 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| e-b-axios.md | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion | 2026-09-29 02:56:50 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-c-chan-full.png | https://chanmeng.org/newsletter/2026-09-28 | 2026-09-29 03:03:16 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| e-c-chan.html | https://chanmeng.org/newsletter/2026-09-28 | 2026-09-29 03:06:55 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| e-c-chan.json | https://chanmeng.org/newsletter/2026-09-28 | 2026-09-29 03:00:22 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| e-c-chan.md | https://chanmeng.org/newsletter/2026-09-28 | 2026-09-29 03:00:22 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-c-final-url.txt | https://chanmeng.org/newsletter/2026-09-28 | 2026-09-29 03:05:52 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-d-final-url.txt | https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma | 2026-09-29 03:05:51 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| e-d-icwork-full.png | https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma | 2026-09-29 03:02:40 UTC+8 | opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图 |
| e-d-icwork.html | https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma | 2026-09-29 03:06:55 UTC+8 | curl / browser HTML；状态见 http-capture.tsv |
| e-d-icwork.json | https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma | 2026-09-29 03:00:28 UTC+8 | opencli extract/eval/social；HN为curl API；原始返回 |
| e-d-icwork.md | https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma | 2026-09-29 03:00:28 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| evidence.md | 本地整理；原始URL见关联存档 | 2026-09-29 03:18:40 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |
| finish-audit.ps1 | 本地整理；原始URL见关联存档 | 2026-09-29 03:18:22 UTC+8 | 本地处理脚本 |
| http-capture.tsv | 本地整理；原始URL见关联存档 | 2026-09-29 03:06:55 UTC+8 | 浏览器逐字提取/PDF文本或本地记录；按文件名区分 |

## 候选截图与裁剪/拼接

下表源文件名可回溯上表URL与原始抓取时间；每行对应且仅对应images内一张PNG。

| PNG | 来源文件与裁剪坐标(x,y,w,h) | 加工时间（北京） | URL |
|---|---|---|---|
| 01-sonnet-benchmarks.png | @(@('sources/a-title.png',180,280,900,160),@('sources/a-benchmark.png',180,40,900,470))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://www.anthropic.com/claude-sonnet-5-5 |
| 02-sonnet-footnotes.png | @(@('sources/a-safeguards-direct.png',300,425,650,230),@('sources/a-notes-good.png',300,115,650,580))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://www.anthropic.com/claude-sonnet-5-5 |
| 03-casp-title-authors.png | @(,@('sources/b-casp-viewport.png',25,20,825,600))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://casp.ac/reports/intelligence-explosion |
| 06-mimo-repetition-rates.png | @(@('sources/c-mimo-full.png',200,115,880,965),@('sources/c-mimo-full.png',200,1555,880,380))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition |
| 07-mimo-rl-worsening.png | @(,@('sources/c-mimo-full.png',200,1935,880,655))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition |
| 08-mimo-cost.png | @(@('sources/c-mimo-full.png',200,3720,880,260),@('sources/c-mimo-full.png',200,8230,880,130))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition |
| 09-muse-inc-datasource.png | @(@('sources/d-inc-full.png',215,615,820,370),@('sources/d-inc-full.png',215,5300,820,190))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202 |
| 10-muse-meta-promise.png | @(@('sources/d-meta-full.png',30,270,890,160),@('sources/d-meta-full.png',30,3750,890,88))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/ |
| 04-casp-caveats.png | @(@('sources/b-paper-render-06.png',75,70,417,590),@('sources/b-paper-render-06.png',500,750,417,250))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf |
| 05-casp-millions.png | @(@('sources/b-paper-render-04.png',500,1020,417,275),@('sources/b-paper-render-05.png',75,697,417,205))；多段时竖拼2px分隔 | 2026-09-29 03:16:45 UTC+8 | https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf |
| 11-overclaim-a-bilibili.png | @(,@('sources/e-a-bilibili-full.png',35,75,1170,82))；多段时竖拼2px分隔 | 2026-09-29 03:16:46 UTC+8 | https://www.bilibili.com/video/BV1bZac6NEdd/ |
| 11-overclaim-b-axios.png | @(,@('sources/e-b-axios-full.png',260,360,740,235))；多段时竖拼2px分隔 | 2026-09-29 03:16:46 UTC+8 | https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion |
| 11-overclaim-c-chan.png | @(,@('sources/e-c-chan-full.png',240,2015,710,205))；多段时竖拼2px分隔 | 2026-09-29 03:16:46 UTC+8 | https://chanmeng.org/newsletter/2026-09-28 |
| 11-overclaim-d-icwork.png | @(@('sources/e-d-icwork-full.png',65,92,760,160),@('sources/e-d-icwork-full.png',75,715,745,177))；多段时竖拼2px分隔 | 2026-09-29 03:16:46 UTC+8 | https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma |

补充派生：d-9to5mac-readable.txt 从 d-9to5mac.md 仅去除链接语法，保留原文字词，便于逐字搜索；e-search-log.md 为检索边界说明；validation.md 为本地验收；file-manifest.tsv 列全部文件。d-inc-singleton.webp 原图URL：https://img-cdn.inc.com/image/upload/f_webp,q_auto,c_fit,w_1024/vip/2026/09/CleanShot-2026-09-19-at-16.20.06@2x.png 。

- 2026-09-28（Claude）：删除 `a-system-card.html`——与 `a-system-card.pdf` 逐字节相同（cmp 一致，13 MB），避免重复入库；file-manifest.tsv 中该行作废。
