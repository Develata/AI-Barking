# 1121 期 B 组抓取记录

执行方 Claude Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8，本机时区），取自各次命令输出（`b-fetch-log.jsonl` 的 `time_bj`）。命令前均已 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"`；opencli 浏览器扩展未连接，未使用；未登录任何站点、未输入凭据、未接受 Cookie、未使用镜像或代理站。barking 记录文件：`b-fetch-log.jsonl`（fetch，78 行含重试）、`b-shots-log.jsonl`（shot）、`b-heat.json`（heat）。清单文件 `b-fetch-list-1.tsv` … `b-fetch-list-11.tsv`。

## 一、barking fetch（工具：`tools/barking/target/release/barking fetch docs/2611/1121 b <清单>`，默认 UA）

| 时间 | 清单 | name → URL | 结果 |
|---|---|---|---|
| 22:59:39–23:00:04 | list-1 | arxiv-abs、arxiv-pdf（76 页，916,436 字节，SHA-256 d91a79c5dbf1fc662f0c5c1bb2b690ac3d83d9ef5bf872bde4cdaf7cc9e76005）、gh-3sum-tree、gh-readme-raw（SHA-256 9fe93a17e286…）、gh-commits（页面壳，内容改用 gh api）、gh-repo-readme-raw | ok 200；arxiv-abs、arxiv-pdf 首次连接失败，重试成功 |
| 23:00:01 | list-1 | arxiv-api → https://export.arxiv.org/api/query?id_list=2610.06783 | **failed**（连接失败，重试 1 次）；23:00 用 curl 重取成功，存 `b-arxiv-api.xml`（200，3418 字节） |
| 23:00:03 | list-1 | fx-rohan → https://api.fxtwitter.com/search?q=3SUM | **failed**（猜的接口不存在）；改用推文 ID 直取，见二 |
| 23:01:17–23:01:22 | list-2 | chan-abs/chan-pdf（arXiv 2610.42012）、geom-abs（2610.32162）、mathworld、aiweekly、explainx、kingy；anthropic-news、anthropic-research、anthropic-red（red.anthropic.com）；compblog、shtetl、godel（rjlipton.com）、wot（windowsontheory.org）；vw-home、vw-lecture12 | ok 200 |
| 同上 | list-2 | digg → https://digg.com/ai/zg6v9jm2 | **failed** 404（搜索结果里的链接已失效） |
| 同上 | list-2 | alphaxiv → https://alphaxiv.org/abs/2610.06783 | **blocked** 403（Cloudflare 质询，`b-alphaxiv.html` 是质询页，不绕过） |
| 同上 | list-2 | alman-home → https://www.cs.columbia.edu/~josh/ | blocked（正文过短，是重定向跳板页）；改取 https://joshalman.com/（curl，200，`b-alman-home2.*`），无该论文条目 |
| 同上 | list-2 | vw-survey → https://people.csail.mit.edu/virgi/eccentric-proc.pdf | **failed** 404（猜错文件名）；23:04 改取主页链接的 `eccentri.pdf` 成功（见下） |
| 23:02:41–23:02:44 | list-3 | simp-abs/simp-pdf（2610.15663）、hyper-abs（2611.03880）、compblog-3sum、bigospeedrun、formal-readme-html、formal-remarks、formal-machine、gh-issues、gh-pulls（PR #28）、quanta-search | ok 200 |
| 同上 | list-3 | vw-fgc-survey → …/eccentric-survey.pdf | **failed** 404（猜错） |
| 23:03:33–23:03:35 | list-4 | anthropic-science、anthropic-frt（red.anthropic.com/2026/）、gh-3sum-docs-remarks-html、shtetl-feed、lipton-feed、wot-feed、compblog-feed | ok 200（feed 为 XML，无 .txt，用 Python 标准库解析） |
| 23:04:15 | list-5 | vw-icm-survey → https://people.csail.mit.edu/virgi/eccentri.pdf（32 页，SHA-256 2f9dcf780203…） | ok 200 |
| 23:04:39–23:04:41 | list-6 | sohu、chronicle、cellcog、postcutoff、aiproofwatch、ie-substack、wikipedia-3sum、preproc（arXiv 2610.21961，页面无本事件相关） | ok 200 |
| 23:05:12–23:05:23 | list-7 | sinacn-claude、sina-finance、aiwz、aichaai、newland、xiaoyu、binaryverse | ok 200 |
| 同上 | list-7 | vsnews | blocked（正文过短，JS 渲染） |
| 同上 | list-7 | aihot-item → https://aihot.news/items/d32df9vzpliz9m348fug74inq | **failed**（协议错误） |
| 同上 | list-7 | aizj → https://aizj.cn/8380.html | **failed**（连接失败，重试 1 次） |
| 23:09:16–23:09:18 | list-8 | newruntime-vvw、yt-alman（YouTube 页，`.txt` 仅 7 行，字幕未入页）、cail-alman、cmu-vvw、yale-alman、mitnews-sciam、whataifound、paytonsuh、functor | ok 200 |
| 23:09:35 | list-9 | vvw-thoughts → https://people.csail.mit.edu/virgi/thoughts.txt | **failed**（连接失败，重试 1 次）；curl https 与 http 各一次，均 000；改用 Exa web_fetch（见二） |
| 23:11:11 | list-10 | palomar、palomar-gh | ok 200 |
| 约 23:13 | list-11 | anthropic-crypto → https://www.anthropic.com/research/discovering-cryptographic-weaknesses | ok 200（Alman 演讲提到的 Anthropic 密码学博客；grep 无 clique/3SUM/APSP） |

## 二、curl、gh api、Exa（个别一次性请求）

| 时间 | 工具 | URL/操作 | 结果 |
|---|---|---|---|
| 23:00 | curl `--max-time 30` | https://export.arxiv.org/api/query?id_list=2610.06783 | 200，`b-arxiv-api.xml`（published 2026-10-05T17:44:29Z；updated 字段 2026-11-22T15:00:13Z 为查询时刻） |
| 23:01 | gh api | repos/anthropics/formal-math/commits?path=3sum-apsp；repos/…/commits?per_page=30；repos/anthropics/formal-math；commits/e1a4e65081 | ok；`b-gh-commits-3sum.json`（仅 1 个提交）、`b-gh-commits-all.json`、`b-gh-repo.json`、`b-gh-commit-e1a4e65.json`。写盘后把提交作者邮箱替换为 `[REDACTED]` |
| 23:01 | gh api | actions/runs/37411656515 与 /jobs；search/issues?q=repo:anthropics/formal-math+3sum | ok；`b-gh-ci-run.json`、`b-gh-ci-jobs.tsv`（verify (3sum-apsp) success，2026-10-06T04:00:44Z–04:40:08Z）；搜索只返回 #28、#29 |
| 23:01–23:02 | Algolia（hn.algolia.com） | 查询 3SUM、APSP、Alman Vassilevska、2610.06783、Subquadratic 3SUM（stories） | 仅 49977437、49973854 相关 |
| 23:02:05 | `barking heat docs/2611/1121 b --item 49977437 --item 49973854 --hn "3SUM Made Simpler" --hn "bigospeedrun" --hn "Anthropic Claude 3SUM" --hn "k-SUM hypothesis refuted" --since 1759000000 --reddit "3SUM APSP Claude" --reddit "Truly Subquadratic 3SUM" --reddit "3SUM hypothesis refuted"` | | HN 条目 2 个 ok（`b-hn-49977437.json`、`b-hn-49973854.json`）；HN 搜索无新帖；**Reddit 3 次全部失败**（响应为 old.reddit.com 登录页，不登录），记入 `b-heat.json` errors |
| 约 23:06 | curl | https://api.fxtwitter.com/status/2124213113583174076 | 200，`b-fx-2124213113583174076.json`（公开帖；含作者公开资料，无抓取者信息） |
| 约 23:08 | WebSearch（standard / extended）与 Exa 搜索 | 英文与中文关键词（Claude 3SUM APSP、Rohan Paul、Quanta、Anthropic 员工发言、VW 说明等） | 用于发现链接；结果页只作线索，已对用到的页面逐一取档或在 evidence 里注明“经 Exa 摘要” |
| 23:10 | Exa web_fetch | https://people.csail.mit.edu/virgi/thoughts.txt | 取回全文，原样写入 `b-vvw-thoughts.txt`（文首加取档说明）；YouTube 自动字幕摘录（来自 Exa 搜索高亮，未听全程）写入 `b-alman-talk-excerpts.txt` |
| 23:10 | Python 标准库 | 对 `b-paytonsuh.txt` 的邮箱替换为 `[EMAIL-REDACTED]`（第三方博文里作者公开邮箱，不收集） | 4 处 |
| 23:12 | shasum | 校验和 | 见上表 |

## 三、截图与页面渲染（barking shot，headless Chrome）

除另注外为 700 CSS px 宽、DPR 2；PDF 渲页为 160 dpi（宽 1360 px，不是 DPR 2）。逐张已打开目视确认。每步见 `b-shots-log.jsonl`。输出在 `docs/2611/1121/images/`。

| 时间 | 编号文件 | 来源与操作 | 备注 |
|---|---|---|---|
| 约 23:05 | `13-arxiv-abstract.png`（**作废**） | `render` https://arxiv.org/abs/2610.06783（高 1800）→ `cropbox` [0,112,1400,1905] | 浅色底；headless 未执行 MathJax，摘要内 TeX 以源码显示（`$3$SUM`），不是页面如此；含 “[Submitted on 5 Oct 2026]”、标题、作者、摘要、Comments、Submission history “[v1] Mon, 5 Oct 2026 17:44:29 UTC (93 KB)”；顶栏登录条已裁掉；页面显示的 “From: Josh Alman” 是 arXiv 公开提交人 |
| 约 23:13 | `13-arxiv-abstract-mathjax.png` | 同页重渲：`render … --chrome-arg=--virtual-time-budget=8000` → `cropbox` [0,112,1400,1905] | 公式已正常渲染，取代 `13-arxiv-abstract.png`；浅色底；其余内容同上 |
| 约 23:05 | `14-pdf-p1-claude-statement.png` | `pdf` b-arxiv-pdf.pdf 第 1 页 → `cropbox` [150,240,1220,1520] | 含标题、作者单位、摘要、两段 Claude 声明；左侧 arXiv 水印已裁；非 2 倍像素 |
| 约 23:05 | `15-pdf-p63-acknowledgments-methodology.png` | 第 63 页 → `cropbox` [150,150,1220,1250] | 从标题 “Acknowledgments and Methodology” 到 Columbia/MIT 免责声明，未含参考文献 |
| 约 23:06 | `16-pdf-p12-13-slightly-better-exponents.png` | 第 12、13 页 → `stack`（p12 [150,1455,1220,1612] + p13 [150,170,1220,275]，间隔 12 px） | 句子跨页，两段之间的空白是拼接间隔（页脚页码已裁） |
| 约 23:06 | `19-pdf-p12-impractical-constants.png` | 第 12 页 → `cropbox` [150,720,1220,1030] | “Combinatorial algorithms” 全段 |
| 约 23:06 | `17-gh-3sum-apsp-commit.png` | `render` https://github.com/anthropics/formal-math/commits/main/3sum-apsp（高 1400）→ `cropbox` [0,340,1400,1010] | **深色底**；提交者头像是 Anthropic 提交账号 ecprice-ant 的公开头像，非抓取者；未登录（页面右上角为未登录图标，已裁） |
| 约 23:07 | `20-gh-readme-not-proved.png` | `render` https://github.com/anthropics/formal-math/blob/main/3sum-apsp/README.md（高 7200，图 14400 px）→ `lines` 找锚点 → `cropbox` [0,9570,1400,11955] | **深色底**；从 “The claims say that programs exist.” 到 “What is not proved” 五个要点末句；首次渲染（高 4200）只到文首，未用 |
| 约 23:07 | `18-cn-sina-itshidai-3sum.png` | `render` https://www.sina.cn/weibo/detail/5367910961776034.html（高 2000）→ `cropbox` [0,292,1400,1660] | 浅色底；含发布方 “IT时代网”、“26-11-22 04:32”、正文、配图；移动版页面。**首次裁切 `18-cn-itshidai-sina-3sum.png`（起点 340，切掉头像上半）作废，见“待删除文件”** |
| 约 23:08 | `21-cn-sohu-3sum.png`（**作废**，非 2 倍） | `render` 同页 宽 1200、DPR 1.2 → `cropbox` [180,80,1020,910] | 700 宽版式把正文右侧截断，故改宽视口；输出 840×830 px |
| 约 23:13 | `21-cn-sohu-3sum-2x.png` | `render` https://www.sohu.com/a/1104699242_122066678 `--width 1200 --height 1400 --scale 2`（图 2400×2800）→ `cropbox` [320,130,1680,1510] | 真 2 倍像素；浅色底；含标题、“2026-11-22 04:27 来源: 财法观天下”与全文，裁去导航与侧栏 |
| 约 23:08 | `22-vw-icm-3sum-hypothesis.png` | `pdf` b-vw-icm-survey.pdf 第 5 页 → `cropbox` [150,432,1220,625] | 综述印刷页 5：3-SUM Hypothesis 与 Hypothesis 2 |
| 约 23:08 | `23-chan-p1-anthropic-found.png` | `pdf` b-chan-pdf.pdf 第 1 页 → `cropbox` [150,240,1220,1312] | Chan 等 v2 第 1 页：标题、作者、摘要与引言首句 |
| 约 23:08 | `24-vw-icm-apsp-hypothesis.png` | 综述第 5 页 [150,1062,1220,1385] + 第 6 页 [150,155,1220,228] → `stack` | APSP 问题与 Hypothesis 3 |

### 失败与未截

- **Rohan Paul 帖子卡片**：`render` https://platform.twitter.com/embed/Tweet.html?id=2124213113583174076（两种参数）在匿名 headless Chrome 下输出整张空白图（两次）；`render` https://fxtwitter.com/rohanpaul_ai/status/2124213113583174076 Chrome 未写出图（日志在系统临时目录）。不再重试。公开 JSON 已存档（`b-fx-2124213113583174076.json`）。无 `23-` 帖子卡片图（23 号已用于 Chan 论文页）。
- Reddit 全部取不到（见二）。

## 四、泄露自查（交付前）

对 `docs/2611/1121/` 做了 grep（本组文件）：个人邮箱一律替换为 `[EMAIL-REDACTED]`（涉及 `b-gh-commits-3sum.json`、`b-gh-commits-all.json`、`b-gh-ci-run.json`、`b-simp-pdf.txt`、`b-cmu-vvw.txt`、`b-xiaoyu.txt`、`b-aiproofwatch.txt`、`b-godel.txt`、`b-paytonsuh.txt`；GitHub noreply、机构公共联络邮箱保留；原始 `.html`/`.pdf` 不入库未改）；`b-fx-2124213113583174076.json` 只含 Rohan Paul 的公开资料；未保存抓取者头像、昵称或 handle；截图 17 中的头像属于 Anthropic 提交账号 ecprice-ant；未取登录态页面。

## 五、待删除文件（由 Claude 收尾时一次删除）

- `docs/2611/1121/images/18-cn-itshidai-sina-3sum.png`（首次裁切，头像被切半，已被 `18-cn-sina-itshidai-3sum.png` 取代）
- `docs/2611/1121/images/13-arxiv-abstract.png`（TeX 源码未渲染，已被 `13-arxiv-abstract-mathjax.png` 取代）
- `docs/2611/1121/images/21-cn-sohu-3sum.png`（非 2 倍像素，已被 `21-cn-sohu-3sum-2x.png` 取代）
- 可选（404 或质询页桩，无内容价值）：`docs/2611/1121/sources/b-digg.html`、`b-digg.headers`、`b-alphaxiv.html`、`b-alphaxiv.headers`、`b-vw-survey.html`、`b-vw-survey.headers`、`b-vsnews.html`、`b-vsnews.headers`、`b-aihot-item.headers`、`b-vvw-thoughts.headers`、`b-arxiv-api.headers`（该条 fetch 失败的残留，`b-arxiv-api.xml` 为 curl 重取）、`b-fx-rohan.headers`
- 工作区的 `docs/10-mai-footnote.png` 与 images/ 下 `x-discard-07-first-crop.png`、01–07 号文件属 A 组或既有，未碰。

## 六、体积提示

单个文件超过 1 MB：只有 `b-yt-alman.html`（1,395,641 字节，YouTube 页面 HTML，属“HTML 原件，不入库”）。入库文本最大为 `b-arxiv-pdf.txt`（约 290 KB）与 `b-gh-commits-all.json`（约 159 KB）。PDF（`b-arxiv-pdf.pdf`、`b-chan-pdf.pdf`、`b-simp-pdf.pdf`、`b-vw-icm-survey.pdf`、`b-vw-lecture12.pdf`）与 `.html`、图片只存本地。
