# 1003 抓取日志

范围：仅本期目录；未提交、推送或删除文件。逐次采集记录源文件为 [d-capture-records.jsonl](d-capture-records.jsonl)，时间带 +08:00 即北京时间。以下表由该原始日志转换；脚本/截图补记另列。数字和页面内容是否支持说法以 evidence.md 为准，HTTP 200 不自动算正文成功。

## 环境与方法

- 起始核对 main / 0da80df；已有未跟踪派工单 `.handoff/2026-10-03-1003-evidence.md`，未改它。opencli doctor 正常，v1.8.7 扩展已连接；agent-reach doctor 确认社交后端；未安装升级。
- 普通页面：无 cookie 的 requests GET、普通浏览器 User-Agent；社交：OpenCLI 搜索/公开帖子卡片；X 超时 150 秒。没有登录、发帖、点赞、评论、关注或读取凭据。
- 网页截图：OpenCLI 所连浏览器，CDP 设置 700 CSS px / deviceScaleFactor=2；输出 ≤1400 物理像素宽。Meta 有内部 scrollArea，最终用较高视口定位各节截图，未改原文/数字。01–07 为最终成功版本。
- PDF 截图：pdftoppm 从已下载官方原件渲染整页，宽 1400 px，等价于 700 CSS px 的 2 倍呈现；08–13 为六篇标记示例，15 为 DAYJOB p.6、16 为 p.8。并非重绘表格。
- 20、21 包含官方两表全部行列与表注。22 只含 Tavus 帖子卡片；视频留白处黏性页标题叠层未遮盖事实证据，无抓取者信息。
- 图片只本地保留，未上传网盘。没有生成 images/README.md、cards.toml、fact-check.md 或发布稿。没有包为 ZIP。

## 失败、恢复与缺口

- OUP Turing 1950：公开 GET 403；浏览器 Cloudflare 验证页，未绕过。Turing Archive `https://www.turingarchive.org/browse.php/B/9` 搜索工具打开失败。没有改用转载、代理或镜像，所以 C4 原文仍缺。
- Kida 出版社页面 HTTP 202 空响应；只补存 DOI/Crossref 元数据，未取得正文。Springer evolution-algebra 页 HTTP 200 但为 challenge 页，不算取得原文。
- Threads `https://www.threads.com/@aiatmeta` 标题可读但未取得目标帖正文、URL、时间。
- AIHOT DAYJOB 普通搜索/全文 relevance 搜索均显示 0；HN DAYJOB 返回无关条目；X 目标组合检索也含无关结果，不据此报目标热度。Tao/mathstodon 没找到可核目标帖。
- 取证初期 OpenCLI screenshot 参数调用失败；CDP Runtime.evaluate 被拒。改用 OpenCLI 公共 DOM eval 和允许的 Page.captureScreenshot。未修改安装的 adapter。
- Meta 初次窄视口截图因内滚动容器出现空白，随后同文件名保存正确区域；DAYJOB HTML 截图曾不完整，最终改为官方 PDF 整页。交付引用的是最终文件；保留失败记录，不把失败算成成功。
- b-extract-pdf.py 需要未安装的 fitz，运行失败；改用现有 pdftotext -enc UTF-8 -layout 和 pdftoppm。该脚本是失败尝试记录，非推荐重跑入口。
- 若干通用归档/汇总脚本调用遭自动审批拒绝，提示 `approval required by policy, but AskForApproval is set to Never`，未给具体理由。公开 GET 和明确限于目标文件的读写可继续；汇总 d-build-evidence.py 未执行成功，最终 evidence.md 通过直接文件补丁写入，**该生成脚本不是交付真源，不要用它覆盖已审阅清单**。
- a-scroll.mjs 是被后续 a-deep-shots.mjs 取代的滚动尝试；d-screenshot-contact.jpg 是本地视觉 QA 缩略图，不是候选配图。

## 搜索补记

Web 搜索用于发现链接，不能作为一手事实证据；正式采用页面均另存本地。检索主题包括 Meta 六论文/并行工作原件、Nilradical、Hu–Wen、Tao/mathstodon，DAYJOB 24% 工作措辞和社区讨论，Tavus Protos/1300万/Community Note、中英文图灵测试措辞、Turing 1950 原出版页。补记无法可靠恢复每个 Web 查询的精确秒级时刻，不伪造时间；本轮均为北京时间 2026-10-03，页面的逐次采集时间见下表。

## 逐次采集（北京时间）

| 北京时间 | URL / 查询 | 工具 | 结果 | 文件 |
|---|---|---|---|---|
| 2026-10-03T22:00:59.739449+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | opencli browser open/eval | 成功提取；内容另行核验 | a-meta-blog-browser.json |
| 2026-10-03T22:01:14.572159+08:00 | https://arxiv.org/abs/2610.01306 | opencli browser open/eval | 成功提取；内容另行核验 | b-dayjob-abs-browser.json |
| 2026-10-03T22:01:33.646527+08:00 | https://www.tavus.io/griffin | opencli browser open/eval | 成功提取；内容另行核验 | c-griffin-browser.json |
| 2026-10-03T22:01:48.677091+08:00 | https://ai.meta.com/research/publications/tightness-of-the-cycle-based-relaxation-for-completed-length-three-alpha-cycles/ | requests GET no cookies | HTTP 200; final=https://ai.meta.com/research/publications/tightness-of-the-cycle-based-relaxation-for-completed-length-three-alpha-cycles/ | a-paper-4.html |
| 2026-10-03T22:01:50.898381+08:00 | https://ai.meta.com/research/publications/the-strict-threshold-for-gaussian-ellipsoid-fitting/ | requests GET no cookies | HTTP 200; final=https://ai.meta.com/research/publications/the-strict-threshold-for-gaussian-ellipsoid-fitting/ | a-paper-1.html |
| 2026-10-03T22:01:51.289034+08:00 | https://ai.meta.com/research/publications/semiabelian-groups-need-not-be-monomial/ | requests GET no cookies | HTTP 200; final=https://ai.meta.com/research/publications/semiabelian-groups-need-not-be-monomial/ | a-paper-3.html |
| 2026-10-03T22:01:51.657419+08:00 | https://research.nvidia.com/labs/amri/projects/video-fdb/ | opencli browser open/eval | 成功提取；内容另行核验 | c-videofdb-browser.json |
| 2026-10-03T22:01:52.069908+08:00 | https://ai.meta.com/research/publications/string-two-point-function-height-function-on-a-curve/ | requests GET no cookies | HTTP 200; final=https://ai.meta.com/research/publications/string-two-point-function-height-function-on-a-curve/ | a-paper-5.html |
| 2026-10-03T22:01:54.002767+08:00 | https://ai.meta.com/research/publications/finite-time-blow-up-of-radial-negative-energy-solutions-for-the-mass-critical-biharmonic-nonlinear-schrodinger-equation/ | requests GET no cookies | HTTP 200; final=https://ai.meta.com/research/publications/finite-time-blow-up-of-radial-negative-energy-solutions-for-the-mass-critical-biharmonic-nonlinear-schrodinger-equation/ | a-paper-2.html |
| 2026-10-03T22:01:54.831908+08:00 | https://arxiv.org/abs/2608.12415 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/2608.12415 | a-parallel-delacerda.html |
| 2026-10-03T22:01:57.322338+08:00 | https://arxiv.org/abs/2609.25023 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/2609.25023 | a-parallel-huwen.html |
| 2026-10-03T22:01:57.926927+08:00 | https://ai.meta.com/research/publications/on-solvable-evolution-algebras-and-a-conjecture-by-garcia-martinez-and-perez-rodriguez/ | requests GET no cookies | HTTP 200; final=https://ai.meta.com/research/publications/on-solvable-evolution-algebras-and-a-conjecture-by-garcia-martinez-and-perez-rodriguez/ | a-paper-6.html |
| 2026-10-03T22:01:58.221950+08:00 | https://arxiv.org/abs/2608.27372 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/2608.27372 | a-parallel-koehler.html |
| 2026-10-03T22:01:58.836496+08:00 | https://arxiv.org/abs/2608.10184 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/2608.10184 | a-parallel-misiakiewicz.html |
| 2026-10-03T22:02:01.547539+08:00 | https://arxiv.org/pdf/2610.01306 | requests GET no cookies | HTTP 200; final=https://arxiv.org/pdf/2610.01306 | b-dayjob-pdf.pdf |
| 2026-10-03T22:02:04.124903+08:00 | https://arxiv.org/html/2610.01306v1 | requests GET no cookies | HTTP 200; final=https://arxiv.org/html/2610.01306v1 | b-dayjob-html.html |
| 2026-10-03T22:02:52.301163+08:00 | https://scontent-atl3-2.xx.fbcdn.net/v/t39.2365-6/830563730_2200751717147921_1010512089847660542_n.pdf?_nc_cat=101&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=_BEXTabGvJ4Q7kNvwFzWIdP&_nc_oc=Adr1ZtNvtOI_5ZEcC-cQev8CerrEQJKd8Li3VdCrBTNE6Q5B3RkxaCDJstgrZMRpBg8&_nc_zt=14&_nc_ht=scontent-atl3-2.xx&_nc_gid=3_g2yB0NpSaR-XsG4LrNCQ&_nc_ss=7920f&oh=00_AQM9NwsBMdSsmiqOApTMe-Ymw_IP1mOjJad1vva6PpUeYA&oe=6AC6DC95 | requests GET no cookies | HTTP 200; final=https://scontent-atl3-2.xx.fbcdn.net/v/t39.2365-6/830563730_2200751717147921_1010512089847660542_n.pdf?_nc_cat=101&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=_BEXTabGvJ4Q7kNvwFzWIdP&_nc_oc=Adr1ZtNvtOI_5ZEcC-cQev8CerrEQJKd8Li3VdCrBTNE6Q5B3RkxaCDJstgrZMRpBg8&_nc_zt=14&_nc_ht=scontent-atl3-2.xx&_nc_gid=3_g2yB0NpSaR-XsG4LrNCQ&_nc_ss=7920f&oh=00_AQM9NwsBMdSsmiqOApTMe-Ymw_IP1mOjJad1vva6PpUeYA&oe=6AC6DC95 | a-paper-1-full.pdf |
| 2026-10-03T22:02:52.323874+08:00 | https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/831208730_1717352456595264_1896576906550560801_n.pdf?_nc_cat=103&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=AWMpekBdTNcQ7kNvwErfHq4&_nc_oc=AdrIo4ddnmZ--P719RDpaTjsRdUwwZqTB-mmulEjZ6hYx58EGH6kw36EeiK0cXzn65g&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=5jQLmK-nh0CvDiytxkr8mw&_nc_ss=7920f&oh=00_AQMmyotDMXgyUcZGWBmeccv5KosB0shp7vIqUjexSZ9GGQ&oe=6AC6E985 | requests GET no cookies | HTTP 200; final=https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/831208730_1717352456595264_1896576906550560801_n.pdf?_nc_cat=103&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=AWMpekBdTNcQ7kNvwErfHq4&_nc_oc=AdrIo4ddnmZ--P719RDpaTjsRdUwwZqTB-mmulEjZ6hYx58EGH6kw36EeiK0cXzn65g&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=5jQLmK-nh0CvDiytxkr8mw&_nc_ss=7920f&oh=00_AQMmyotDMXgyUcZGWBmeccv5KosB0shp7vIqUjexSZ9GGQ&oe=6AC6E985 | a-paper-3-full.pdf |
| 2026-10-03T22:02:52.856605+08:00 | https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/831505460_28533076623010200_3137112670568097685_n.pdf?_nc_cat=110&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=1v_KOSUHizQQ7kNvwHBP4lg&_nc_oc=AdogiixydOV6ktAa0jn3yRm0hbZBNZC-D7-41YkI4tp3ojLlc0pwIOfZ9D4fM7ELiIo&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=0tVl-Vz7Rb071sP13WU-OQ&_nc_ss=7920f&oh=00_AQP3NBBwCm5L3v66KDQ33xFyvUmQyfMkjqVAR4tHEr6leQ&oe=6AC6C708 | requests GET no cookies | HTTP 200; final=https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/831505460_28533076623010200_3137112670568097685_n.pdf?_nc_cat=110&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=1v_KOSUHizQQ7kNvwHBP4lg&_nc_oc=AdogiixydOV6ktAa0jn3yRm0hbZBNZC-D7-41YkI4tp3ojLlc0pwIOfZ9D4fM7ELiIo&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=0tVl-Vz7Rb071sP13WU-OQ&_nc_ss=7920f&oh=00_AQP3NBBwCm5L3v66KDQ33xFyvUmQyfMkjqVAR4tHEr6leQ&oe=6AC6C708 | a-paper-2-full.pdf |
| 2026-10-03T22:02:56.612957+08:00 | https://scontent-atl3-2.xx.fbcdn.net/v/t39.2365-6/835979520_1842745737083680_1008015214019231305_n.pdf?_nc_cat=104&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=Qm6e_ps_vnYQ7kNvwHtEgbd&_nc_oc=AdrO_CB1XI6Dvw9O8TNOig6UcQYd9Y3apIeF4K7NUimWA21xsBIT0MNpUjcloRn9Abg&_nc_zt=14&_nc_ht=scontent-atl3-2.xx&_nc_gid=xdIJNE1QWgZYC9H2WxrIZQ&_nc_ss=7920f&oh=00_AQMwoyYwhzA8jwUlAc55c7uWloUOlNTceor4x_USp1yD0Q&oe=6AC6C3A1 | requests GET no cookies | HTTP 200; final=https://scontent-atl3-2.xx.fbcdn.net/v/t39.2365-6/835979520_1842745737083680_1008015214019231305_n.pdf?_nc_cat=104&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=Qm6e_ps_vnYQ7kNvwHtEgbd&_nc_oc=AdrO_CB1XI6Dvw9O8TNOig6UcQYd9Y3apIeF4K7NUimWA21xsBIT0MNpUjcloRn9Abg&_nc_zt=14&_nc_ht=scontent-atl3-2.xx&_nc_gid=xdIJNE1QWgZYC9H2WxrIZQ&_nc_ss=7920f&oh=00_AQMwoyYwhzA8jwUlAc55c7uWloUOlNTceor4x_USp1yD0Q&oe=6AC6C3A1 | a-paper-6-full.pdf |
| 2026-10-03T22:02:57.128563+08:00 | https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/830682166_1315967147179779_6528021118074843377_n.pdf?_nc_cat=106&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=CAz5dWB1jvAQ7kNvwGLu8Fl&_nc_oc=AdpnnyOmD_P-Mdt_iREV0mx5o1Tz9BzHQ-5DwNu6Q19v0pPUFtJ1Lt7sCL5e2VTxrFA&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=X1OrP0F5hSEnKmK2h5vDGQ&_nc_ss=7920f&oh=00_AQOwI6mhNEKo1B-tlOUp4QjLvKdwJc0L-qk_o_2VgAgEBA&oe=6AC6D8A9 | requests GET no cookies | HTTP 200; final=https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/830682166_1315967147179779_6528021118074843377_n.pdf?_nc_cat=106&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=CAz5dWB1jvAQ7kNvwGLu8Fl&_nc_oc=AdpnnyOmD_P-Mdt_iREV0mx5o1Tz9BzHQ-5DwNu6Q19v0pPUFtJ1Lt7sCL5e2VTxrFA&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=X1OrP0F5hSEnKmK2h5vDGQ&_nc_ss=7920f&oh=00_AQOwI6mhNEKo1B-tlOUp4QjLvKdwJc0L-qk_o_2VgAgEBA&oe=6AC6D8A9 | a-paper-4-full.pdf |
| 2026-10-03T22:02:58.560676+08:00 | https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/830612858_1470339108316714_4667400450549396254_n.pdf?_nc_cat=100&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=Eh7XNph86f0Q7kNvwFFwD0F&_nc_oc=Adon8SkXA3Dldgsr059vCB9SLc8u1LtGlt_zwR1qziRQUqLeLvx0DO4y1hye-BABjvs&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=8Qf_fRSlHt1pkB2_AMSLaA&_nc_ss=7920f&oh=00_AQMZjyofA71m-7txGd-kMvG8Ny9lUNrCmtps3C9WSRWVYQ&oe=6AC6D2F2 | requests GET no cookies | HTTP 200; final=https://scontent-atl3-1.xx.fbcdn.net/v/t39.2365-6/830612858_1470339108316714_4667400450549396254_n.pdf?_nc_cat=100&ccb=1-7&_nc_sid=3c67a6&_nc_ohc=Eh7XNph86f0Q7kNvwFFwD0F&_nc_oc=Adon8SkXA3Dldgsr059vCB9SLc8u1LtGlt_zwR1qziRQUqLeLvx0DO4y1hye-BABjvs&_nc_zt=14&_nc_ht=scontent-atl3-1.xx&_nc_gid=8Qf_fRSlHt1pkB2_AMSLaA&_nc_ss=7920f&oh=00_AQMZjyofA71m-7txGd-kMvG8Ny9lUNrCmtps3C9WSRWVYQ&oe=6AC6D2F2 | a-paper-5-full.pdf |
| 2026-10-03T22:04:01.316970+08:00 | https://x.com/search?q=(from:AIatMeta research) OR (from:tavus Griffin) OR DAYJOB since:2026-09-30 | agent-reach / OpenCLI twitter search | 成功；30条上限；public posts only | d-x-search.json |
| 2026-10-03T22:06:46.207817+08:00 | https://surgehq.ai/blog/dayjob | requests GET no cookies | HTTP 200; final=https://surgehq.ai/blog/dayjob | b-surge-blog.html |
| 2026-10-03T22:06:47.049265+08:00 | https://arxiv.org/abs/2605.30256 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/2605.30256 | c-videofdb-abs.html |
| 2026-10-03T22:06:47.959234+08:00 | https://research.nvidia.com/labs/amri/projects/video-fdb/ | requests GET no cookies | HTTP 200; final=https://research.nvidia.com/labs/amri/projects/video-fdb/ | c-videofdb-page.html |
| 2026-10-03T22:06:49.632731+08:00 | https://github.com/surge-ai/dayjob | requests GET no cookies | HTTP 200; final=https://github.com/surge-ai/dayjob | b-harness.html |
| 2026-10-03T22:06:49.996279+08:00 | https://huggingface.co/datasets/surgeai/DAYJOB-healthcare | requests GET no cookies | HTTP 200; final=https://huggingface.co/datasets/surgeai/DAYJOB-healthcare | b-health-dataset.html |
| 2026-10-03T22:06:51.079037+08:00 | https://huggingface.co/datasets/surgeai/DAYJOB-finance | requests GET no cookies | HTTP 200; final=https://huggingface.co/datasets/surgeai/DAYJOB-finance | b-finance-dataset.html |
| 2026-10-03T22:06:51.796015+08:00 | https://github.com/alunik/kourovka-lean/blob/5a6b2c18e326b7b0281f00b64cade629acbfe1f5/Kourovka/Problems/P21_68/README.md | requests GET no cookies | HTTP 200; final=https://github.com/alunik/kourovka-lean/blob/5a6b2c18e326b7b0281f00b64cade629acbfe1f5/Kourovka/Problems/P21_68/README.md | a-nilradical.html |
| 2026-10-03T22:06:51.901563+08:00 | https://github.com/alunik/kourovka-lean | requests GET no cookies | HTTP 200; final=https://github.com/alunik/kourovka-lean | a-nilradical-repo.html |
| 2026-10-03T22:06:53.033815+08:00 | https://www.degruyterbrill.com/document/doi/10.1515/jgth-2024-0010/html | requests GET no cookies | HTTP 202; final=https://www.degruyterbrill.com/document/doi/10.1515/jgth-2024-0010/html | a-original-kida.html |
| 2026-10-03T22:06:53.215794+08:00 | https://link.springer.com/article/10.1007/s00013-026-02251-0 | requests GET no cookies | HTTP 200; final=https://link.springer.com/article/10.1007/s00013-026-02251-0 | a-original-algebra.html |
| 2026-10-03T22:06:53.277203+08:00 | https://arxiv.org/abs/2507.12831 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/2507.12831 | a-original-optimization.html |
| 2026-10-03T22:06:53.484536+08:00 | https://k.sina.com.cn/article_5952915705_162d248f906703pv9i.html | requests GET no cookies | HTTP 200; final=https://k.sina.com.cn/article_5952915705_162d248f906703pv9i.html | d-meta-sina.html |
| 2026-10-03T22:06:54.954649+08:00 | https://protos.com/tag/tavus/ | requests GET no cookies | HTTP 200; final=https://protos.com/tag/tavus/ | c-protos-index.html |
| 2026-10-03T22:06:55.438294+08:00 | https://www.ai-primer.com/engineer/stories/meta-muse-assisted-math-results | requests GET no cookies | HTTP 200; final=https://www.ai-primer.com/engineer/stories/meta-muse-assisted-math-results | d-meta-primer.html |
| 2026-10-03T22:07:01.437172+08:00 | https://arxiv.org/pdf/2605.30256 | requests GET no cookies | HTTP 200; final=https://arxiv.org/pdf/2605.30256 | c-videofdb-pdf.pdf |
| 2026-10-03T22:07:02.843276+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\02-meta-probability.png","y":1979.625,"width":1400,"height":2000,"dpr":2} (node:34560) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 02-meta-probability.png |
| 2026-10-03T22:07:05.996367+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\03-meta-pde.png","y":3448.125,"width":1400,"height":2100,"dpr":2} (node:25616) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 03-meta-pde.png |
| 2026-10-03T22:07:07.751886+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\04-meta-group.png","y":4586.8125,"width":1400,"height":2100,"dpr":2} (node:11560) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 04-meta-group.png |
| 2026-10-03T22:07:09.381392+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\05-meta-optimization.png","y":6029.15625,"width":1400,"height":2000,"dpr":2} (node:44396) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 05-meta-optimization.png |
| 2026-10-03T22:07:11.001702+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\06-meta-arithmetic.png","y":7298.34375,"width":1400,"height":2000,"dpr":2} (node:10376) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 06-meta-arithmetic.png |
| 2026-10-03T22:07:12.608950+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\07-meta-algebra.png","y":8746.8232421875,"width":1400,"height":2100,"dpr":2} (node:50040) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 07-meta-algebra.png |
| 2026-10-03T22:08:26.201670+08:00 | https://research.nvidia.com/labs/amri/projects/video-fdb/ | opencli eval public tables | 两张表 DOM | c-videofdb-tables.json |
| 2026-10-03T22:08:26.804759+08:00 | https://arxiv.org/html/2610.01306v1 | opencli eval public tables | 三张表 DOM | b-dayjob-tables.json |
| 2026-10-03T22:09:13.638983+08:00 | https://x.com/search | agent-reach / OpenCLI twitter search | query="Muse Spark" OR "DAYJOB" since:2026-10-01; 成功返回; 次数见文件 | d-x-search-focused.json |
| 2026-10-03T22:09:17.872785+08:00 | https://news.ycombinator.com | agent-reach / OpenCLI hackernews search | query=Solving Open Research Problems; 成功返回; 次数见文件 | d-hn-meta.json |
| 2026-10-03T22:09:21.203574+08:00 | https://news.ycombinator.com | agent-reach / OpenCLI hackernews search | query=DAYJOB; 成功返回; 次数见文件 | d-hn-dayjob.json |
| 2026-10-03T22:09:38.751181+08:00 | https://reddit.com/search | agent-reach / OpenCLI reddit search | query=Griffin Turing; 成功返回; 次数见文件 | d-reddit.json |
| 2026-10-03T22:09:52.003970+08:00 | 见 a-paper-downloads.json 第 1 项 | Poppler PDF original page raster | page=5; width=1400; 700 CSS equivalent at 2x; 0 | 08-meta-paper-1-labels.png |
| 2026-10-03T22:09:52.396147+08:00 | 见 a-paper-downloads.json 第 2 项 | Poppler PDF original page raster | page=1; width=1400; 700 CSS equivalent at 2x; 0 | 09-meta-paper-2-labels.png |
| 2026-10-03T22:09:52.878490+08:00 | 见 a-paper-downloads.json 第 3 项 | Poppler PDF original page raster | page=4; width=1400; 700 CSS equivalent at 2x; 0 | 10-meta-paper-3-labels.png |
| 2026-10-03T22:09:53.258315+08:00 | 见 a-paper-downloads.json 第 4 项 | Poppler PDF original page raster | page=3; width=1400; 700 CSS equivalent at 2x; 0 | 11-meta-paper-4-labels.png |
| 2026-10-03T22:09:53.726405+08:00 | 见 a-paper-downloads.json 第 5 项 | Poppler PDF original page raster | page=2; width=1400; 700 CSS equivalent at 2x; 0 | 12-meta-paper-5-labels.png |
| 2026-10-03T22:09:54.165456+08:00 | 见 a-paper-downloads.json 第 6 项 | Poppler PDF original page raster | page=2; width=1400; 700 CSS equivalent at 2x; 0 | 13-meta-paper-6-labels.png |
| 2026-10-03T22:10:09.263131+08:00 | https://www.aitoollab.cn/articles/ai-digital-human-tools-2026/ | requests GET no cookies | HTTP 200; final=https://www.aitoollab.cn/articles/ai-digital-human-tools-2026/ | d-griffin-cn.html |
| 2026-10-03T22:10:11.023137+08:00 | https://academic.oup.com/mind/article/LIX/236/433/986238 | requests GET no cookies | HTTP 403; final=https://academic.oup.com/mind/article/LIX/236/433/986238 | c-turing-original.html |
| 2026-10-03T22:10:11.192263+08:00 | https://api.github.com/users/alunik | requests GET no cookies | HTTP 200; final=https://api.github.com/users/alunik | a-nilradical-user.json |
| 2026-10-03T22:10:11.564007+08:00 | https://protos.com/tavus-call-bot-sparks-ai-scam-psychosis-fears/ | requests GET no cookies | HTTP 200; final=https://protos.com/tavus-call-bot-sparks-ai-scam-psychosis-fears/ | c-protos.html |
| 2026-10-03T22:10:12.616810+08:00 | https://tech-ish.com/2026/10/02/tavus-griffin/ | requests GET no cookies | HTTP 200; final=https://tech-ish.com/2026/10/02/tavus-griffin-ai-fooled-48-percent-video-call/ | d-griffin-en.html |
| 2026-10-03T22:10:14.604652+08:00 | https://api.github.com/repos/alunik/kourovka-lean/commits/5a6b2c18e326b7b0281f00b64cade629acbfe1f5 | requests GET no cookies | HTTP 200; final=https://api.github.com/repos/alunik/kourovka-lean/commits/5a6b2c18e326b7b0281f00b64cade629acbfe1f5 | a-nilradical-commit.json |
| 2026-10-03T22:11:44.492405+08:00 | https://x.com/tavus/status/2105704169009246248 | OpenCLI article only eval | 英文原文及拟议注释；无导航/抓取者信息 | c-tavus-x-card.json |
| 2026-10-03T22:11:44.998291+08:00 | https://aihot.news | OpenCLI main public content | 首页已加载部分；非全部历史 | d-aihot.json |
| 2026-10-03T22:12:49.481635+08:00 | https://arxiv.org/abs/1503.01741 | requests GET no cookies | HTTP 200; final=https://arxiv.org/abs/1503.01741 | a-original-pde.html |
| 2026-10-03T22:12:50.060694+08:00 | https://arxiv.org/pdf/1503.01741 | requests GET no cookies | HTTP 200; final=https://arxiv.org/pdf/1503.01741 | a-original-pde-full.pdf |
| 2026-10-03T22:12:50.448703+08:00 | https://nilradical.ai/results/kourovka-21-68/ | requests GET no cookies | HTTP 200; final=https://nilradical.ai/results/kourovka-21-68/ | a-nilradical-result.html |
| 2026-10-03T22:12:50.826061+08:00 | https://nilradical.ai/ | requests GET no cookies | HTTP 200; final=https://nilradical.ai/ | a-nilradical-site.html |
| 2026-10-03T22:12:51.779101+08:00 | https://arxiv.org/pdf/2507.12831 | requests GET no cookies | HTTP 200; final=https://arxiv.org/pdf/2507.12831 | a-original-optimization-full.pdf |
| 2026-10-03T22:15:25.211352+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\02-meta-probability.png","y":1979.625,"width":1400,"height":2000,"dpr":2} (node:27640) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 02-meta-probability.png |
| 2026-10-03T22:15:31.456305+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\03-meta-pde.png","y":3448.125,"width":1400,"height":2100,"dpr":2} (node:48688) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 03-meta-pde.png |
| 2026-10-03T22:15:37.866917+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\04-meta-group.png","y":4586.8125,"width":1400,"height":2100,"dpr":2} (node:53848) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 04-meta-group.png |
| 2026-10-03T22:15:45.323864+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\05-meta-optimization.png","y":6029.15625,"width":1400,"height":2000,"dpr":2} (node:2272) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 05-meta-optimization.png |
| 2026-10-03T22:15:55.173036+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\06-meta-arithmetic.png","y":7298.34375,"width":1400,"height":2000,"dpr":2} (node:29152) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 06-meta-arithmetic.png |
| 2026-10-03T22:15:55.174333+08:00 | https://www.reddit.com/r/singularity/comments/1wv7q40/ | OpenCLI reddit read | top 12 comments; public fields | d-reddit-comments.json |
| 2026-10-03T22:16:01.382939+08:00 | https://research.meta.ai/blog/solving-open-research-problems-together | OpenCLI CDP DPR2 screenshot | {"path":"docs\\2610\\1003\\images\\07-meta-algebra.png","y":8746.8232421875,"width":1400,"height":2100,"dpr":2} (node:37908) [UNDICI-EHPA] Warning: EnvHttpProxyAgent is experimental, expect them to change at any time. (Use `node --trace-warnings ...` to show where the warning was created)    Update available: v1.8.7 → v1.8.8   Run: npm install -g @jackwener/opencli   | 07-meta-algebra.png |
| 2026-10-03T22:17:06.209619+08:00 | https://x.com/AIatMeta/status/2106099776035152231 | OpenCLI article only | 公开帖正文、互动、UTC时间；已切英文 | a-meta-x-card.json |
| 2026-10-03T22:17:06.210174+08:00 | https://academic.oup.com/mind/article/LIX/236/433/986238 | requests + OpenCLI browser | HTTP 403 / Cloudflare 安全验证；未绕过；未取得全文 | c-turing-original.html |
| 2026-10-03T22:19:37.449874+08:00 | https://aihot.news/all?q=DAYJOB | OpenCLI public content | 结果见存档 | d-aihot-dayjob.json |
| 2026-10-03T22:19:37.962410+08:00 | https://academic.oup.com/mind/article/LIX/236/433/986238 | OpenCLI public content | 结果见存档 | c-turing-browser-failure.json |
| 2026-10-03T22:21:50.672376+08:00 | https://aihot.news/all?q=DAYJOB&tab=relevance | OpenCLI browser search | 全文相关 0 条 | d-aihot-dayjob-fullsearch.json |
| 2026-10-03T22:21:50.672862+08:00 | https://www.threads.com/@aiatmeta | OpenCLI browser | 标题可读；main 正文未取到，未确认帖URL/时间 |  |
| 2026-10-03T22:23:29.647900+08:00 | https://arxiv.org/src/2610.01306v1/anc/data/human_time_ranges.csv | requests GET | 200 | b-human_time_ranges.csv |
| 2026-10-03T22:23:29.861020+08:00 | https://arxiv.org/src/2610.01306v1/anc/data/leaderboard_finance.csv | requests GET | 200 | b-leaderboard_finance.csv |
| 2026-10-03T22:23:29.873022+08:00 | https://arxiv.org/src/2610.01306v1/anc/data/leaderboard_healthcare.csv | requests GET | 200 | b-leaderboard_healthcare.csv |
| 2026-10-03T22:24:15.462384+08:00 | https://arxiv.org/pdf/2507.12831v1 | requests public GET | 200 | a-original-optimization-v1.pdf |
| 2026-10-03T22:24:17.800550+08:00 | https://api.crossref.org/works/10.1515/jgth-2024-0010 | requests public GET | 200 | a-kida-metadata.json |
| 2026-10-03T22:24:19.459363+08:00 | https://api.crossref.org/works/10.1007/s00013-026-02251-0 | requests public GET | 200 | a-algebra-metadata.json |


## 最终截图补记

以下时间为最终文件写入时间（北京时间），不是可复原的每一次截图请求开始时间；失败/替换过程见前述记录。网页使用 OpenCLI/CDP，PDF 使用 Poppler。

| 文件 | 来源 | 最终写入时间（补记） | 结果 |
|---|---|---|---|
| 01-meta-principles.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:11:34.6499290+08:00 | 成功；最终文件已检查 |
| 02-meta-probability.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:20:10.7232779+08:00 | 成功；最终文件已检查 |
| 03-meta-pde.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:20:36.8416279+08:00 | 成功；最终文件已检查 |
| 04-meta-group.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:20:37.4379986+08:00 | 成功；最终文件已检查 |
| 05-meta-optimization.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:20:37.9609663+08:00 | 成功；最终文件已检查 |
| 06-meta-arithmetic.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:20:38.4266050+08:00 | 成功；最终文件已检查 |
| 07-meta-algebra.png | https://research.meta.ai/blog/solving-open-research-problems-together | 2026-10-03T22:20:38.6956744+08:00 | 成功；最终文件已检查 |
| 08-meta-paper-1-labels.png | a-paper-downloads.json 第 1 项官方 PDF | 2026-10-03T22:09:51.9936156+08:00 | 成功；最终文件已检查 |
| 09-meta-paper-2-labels.png | a-paper-downloads.json 第 2 项官方 PDF | 2026-10-03T22:09:52.3851507+08:00 | 成功；最终文件已检查 |
| 10-meta-paper-3-labels.png | a-paper-downloads.json 第 3 项官方 PDF | 2026-10-03T22:09:52.8670588+08:00 | 成功；最终文件已检查 |
| 11-meta-paper-4-labels.png | a-paper-downloads.json 第 4 项官方 PDF | 2026-10-03T22:09:53.2473928+08:00 | 成功；最终文件已检查 |
| 12-meta-paper-5-labels.png | a-paper-downloads.json 第 5 项官方 PDF | 2026-10-03T22:09:53.7170617+08:00 | 成功；最终文件已检查 |
| 13-meta-paper-6-labels.png | a-paper-downloads.json 第 6 项官方 PDF | 2026-10-03T22:09:54.1558075+08:00 | 成功；最终文件已检查 |
| 14-dayjob-abstract.png | https://arxiv.org/abs/2610.01306 | 2026-10-03T22:17:03.0238896+08:00 | 成功；最终文件已检查 |
| 15-dayjob-results-table.png | https://arxiv.org/abs/2610.01306 | 2026-10-03T22:21:22.1497484+08:00 | 成功；最终文件已检查 |
| 16-dayjob-case.png | https://arxiv.org/abs/2610.01306 | 2026-10-03T22:23:30.4386626+08:00 | 成功；最终文件已检查 |
| 17-griffin-results.png | https://www.tavus.io/griffin | 2026-10-03T22:13:42.9946609+08:00 | 成功；最终文件已检查 |
| 18-griffin-method.png | https://www.tavus.io/griffin | 2026-10-03T22:13:54.3661682+08:00 | 成功；最终文件已检查 |
| 19-griffin-availability.png | https://www.tavus.io/griffin | 2026-10-03T22:14:00.8537721+08:00 | 成功；最终文件已检查 |
| 20-videofdb-perception.png | https://research.nvidia.com/labs/amri/projects/video-fdb/ | 2026-10-03T22:15:37.7657209+08:00 | 成功；最终文件已检查 |
| 21-videofdb-generation.png | https://research.nvidia.com/labs/amri/projects/video-fdb/ | 2026-10-03T22:19:51.7819979+08:00 | 成功；最终文件已检查 |
| 22-tavus-x-note.png | https://x.com/tavus/status/2105704169009246248 | 2026-10-03T22:21:26.5413659+08:00 | 成功；最终文件已检查 |

## 交付检查

21 个编号条目、允许的状态词、全部引用文件存在、01–22 连续编号、宽度不超过 1400 px 均通过；DAYJOB 30 行与官方 HTML 逐行一致。隐私关键词扫描没有抓取者身份或本机用户路径；avatar/gravatar 命中是公开作者资料及网站通用说明。git status 仅本期目录和原有派工单；未 commit/push。

- 未入库：`a-nilradical-commit.json`（1.25 MB，GitHub commit API 响应）超过单文件 1 MB 入库上限，经 Develata 同意只留本地；对应 commit 公开可查：https://github.com/alunik/kourovka-lean/commit/5a6b2c18e326b7b0281f00b64cade629acbfe1f5
