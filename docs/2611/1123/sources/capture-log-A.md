# 1123 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，UTC+8，时间为北京时间（本机时钟）。批量抓取逐条结果在 `a-fetch-log.jsonl`，热度在 `a-heat.json`，清单 `a-fetch-list-1.tsv … -7.tsv`。均走 `barking fetch`（每条限时 60 s，PDF 180 s）；个别一次性请求用 `gh api`、`curl --max-time 60`。不用 firecrawl，不读取浏览器 Cookie，未登录、未发帖。**被催交，未做任何截图。**

## 一、批量存档（barking fetch）

| 批 | 北京时刻 | 内容 | 结果 |
|---|---|---|---|
| 1 | 22:58:00 | GitHub：vp-vnp 目录页、README raw（取自提交 918c8e27）、根 README（main）、提交页、PR #51、Statement/Challenge/Solution/Paper.lean、comparator.json、lakefile.toml、lean-toolchain、NOTICE、paper.pdf、Actions 页、仓库首页 | 16 条全部 ok；`.txt` 抽查为正文。提交页 HTML 1.4 MB（只存本地，不入库） |
| 2 | 23:05:34 | openai/math：README、论文 PDF、根 README、history、仓库页、PR #5；HN 主帖页与 OpenAI 帖页；friedgil.com；Aaronson、Fortnow、Kalai 博客首页；anthropic.com 新闻/研究页；claude.com 博客 | 15 条全部 ok |
| 3 | 23:06:51 | OpenAI 的 Lean：ComparatorChallenges/VPVNP.lean 与 .json、ClassSeparation.lean | 3 条 ok |
| 4 | 约 23:07 | Viola 博客 | ok（首次连接失败，自动重试成功） |
| 5 | 23:07:5x | 英文报道：36Kr 英文站、Longbridge、emergent.sh、OfficeChai、Interesting Engineering、Korea Daily | **Longbridge 403：blocked，未取**；其余 ok |
| 6 | 23:07:58 | 中文：网易号 3 篇（新智元、机器之心 Pro、华尔街见闻）、凤凰网、什么值得买 | 5 条 ok |
| 7 | 约 23:10 | arXiv：Bürgisser 综述 abs 与 PDF（2406.06217）、Mahajan 综述 abs 与 PDF（1307.3863）、GCT 综述 abs（0907.2850）；ACM 与 doi.org 的 Valiant 1979 页 | arXiv 5 条 ok；**ACM、doi.org 403（Cloudflare 质询）：blocked，未绕过**，改用 Crossref 取出版信息 |

## 二、个别请求

- `gh api`：anthropics/formal-math 的提交列表（含 path=vp-vnp）、仓库元数据、PR #51 元数据与评审（rfurman、minilek-ant、steinke-ant 账号名为 GitHub 公开账号）、issues 列表（30 条，大部分是无关垃圾）、PR #54；openai/math 的提交、PR #5、preprints 目录列表；Actions 运行列表与两次运行的 jobs 摘要。约 22:58–23:08。首次一次 `gh api` 与一次 `git clone` 遇 TLS EOF，重试成功。
- `gh run view 45272623486 --log`：只把含 “Build completed / nanoda / Your solution is okay / declaration uses sorry / Exporting” 的行存为 `a-gh-ci-push-run-45272623486-excerpt.txt`（整份日志数 MB，不存）。
- `git clone --depth 1 https://github.com/anthropics/formal-math.git`：放在 scratchpad（会话 scratchpad 下的 `fm` 目录，不入库），用来统计 Lean 行数与 grep；HEAD 11b6c7e3。
- `curl https://api.crossref.org/works/10.1145/800135.804419` → `a-bg-valiant-crossref.json`。
- WebSearch（2 次）：找中英文夸大实例与人类评论，结果仅作线索，已回溯到原站存档。
- `barking heat`（23:03:40）：HN 9 个条目；Reddit 两次搜索均被重定向到 old.reddit.com 登录页，记失败。

## 三、失败与未做

| 对象 | 原因 | 处理 |
|---|---|---|
| Longbridge 报道 | 403 | 未用 |
| dl.acm.org / doi.org 的 Valiant 1979 页 | Cloudflare 质询 | 不绕过；用 Crossref 与 Bürgisser 综述的参考文献 |
| Reddit 热度 | 搜索接口要求登录 | 未取到 |
| X / Mathstodon 专家帖、官方 X | 被催交，未查 | 列入未完成 |
| 本机 Lean 构建（A4） | 本机无 v4.33.1，未启动 | 以作者方 CI 记录代替（A4 行） |
| 截图 01–12 | 被催交 | 全部未做；`images/` 目录未建立 |

## 四、待删除文件

无。（scratchpad 里的浅克隆在仓库外，未入库。）

## 五、体量与泄露自查

- 入库的文本里最大：`a-paper-pdf.txt` 330,745 字节、`a-gh-commit.txt` 49,432 字节，均不到 1 MB。不入库的 HTML/PDF 原件：`a-gh-commit.html` 1.45 MB 等，`a-paper-pdf.pdf` 1.02 MB，`a-oai-paper-pdf.pdf` 0.21 MB，`a-bg-arxiv-2406-06217.pdf` 0.98 MB，`a-bg-arxiv-1307-3863.pdf` 0.31 MB。
- 抓取者信息：均匿名请求，未登录；存档里的评论者与 GitHub 账号为对方公开账号。

## 六、截图（第二轮，北京 2026-11-25 01:12–01:35；`barking shot` render / pdf / cropbox / stack，记录在 `a-shots-log.jsonl`）

均为匿名 headless Chrome（`render`，无登录栏）或本地 PDF 渲页（`pdf`，144 dpi）后只裁切、拼接，不改像素；逐张打开目视确认。GitHub 页面为深色底（headless 默认跟随系统外观）。PDF 页图为白底 144 dpi（A4 页 1191×1684，等同 2 倍像素比），裁切边界落在文字行之间。

| 文件 | 来源 | 渲染/裁切 | 备注 |
|---|---|---|---|
| 01-gh-readme-top.png（1400×800） | https://github.com/anthropics/formal-math/tree/main/vp-vnp | render `--width 700 --height 2600`（DPR 2）；cropbox [0,2210,1400,3010] | 深色底；README 标题到 “Research artifact” 段，只含正文，文件列表、提交作者行与页眉裁掉 |
| 02-gh-readme-lean.png（1400×990） | 同上 | cropbox [0,3270,1400,4260] | 深色底；“Lean proof”小节顶行贴边但未截断 |
| 03-paper-p1-abstract.png（920×435） | `a-paper-pdf.pdf` p.1 | pdf 渲页；cropbox [140,190,1060,625] | 标题、署名、摘要 |
| 04-paper-p2-theorem.png（920×334） | 同 p.2 | cropbox [140,218,1060,552] | 1.1 Results 到 Corollary 1.2；第一次裁到下一段行首被切（已作废） |
| 05-paper-appq-q1.png（1051×306） | 同 p.66 | cropbox [140,694,1191,1000] | Q.1 全段与末句；第一次含上一行残影（已作废）。Table 3 在 p.67，未截 |
| 06-paper-p1-quasipoly.png（920×165） | 同 p.1 | cropbox [140,1301,1060,1466] | 第一次上下有相邻行残影（已作废） |
| 07-gh-pr51.png（1400×1355） | https://github.com/anthropics/formal-math/pull/51 | render `--width 700 --height 2400`（DPR 2）；cropbox [0,335,1400,1690] | 深色底；匿名页面只显示相对时间（“20 hours ago”）与 “Nov 23, 2026”，绝对时刻以 GitHub API 为准（evidence A1-3）；含 steinke-ant、minilek-ant、rfurman 三个公开 GitHub 账号名，不含抓取者信息 |
| 08-oai-paper-p1.png（920×375） | `a-oai-paper-pdf.pdf` p.1（来源 https://github.com/openai/math/blob/main/preprints/VP-and-VNP-are-different-November-23-2026/paper.pdf） | pdf 渲页；cropbox [150,225,1070,600] | 标题、署名 OpenAI、摘要 |
| 09-oai-root-readme.png（1400×1012） | https://github.com/openai/math | render `--width 700 --height 3000`；cropbox [0,2240,1400,3252] | 深色底；含 “Some of the unformalized results could have issues.” 与 “We defer to the math community completely on matters of credit, precedence, and concurrent work.” |
| 09-oai-history-nov6.png（1400×615） | https://github.com/openai/math/blob/main/history.md | render `--width 700 --height 1200`；cropbox [0,780,1400,1395] | 深色底；“This result was obtained on November 6, 2026.” |
| 10-zh-163-xzy.png（1060×432） | https://www.163.com/dy/article/LCGA4F6V0511ABV6.html | render `--width 1000 --height 13500 --scale 1.2`（DPR 1.2，整页 1200×16200）；stack 两段（标题与时间来源行；“最关键的是……还需要等待人类数学家核验”两段，白底拼接） | **像素比 1.2，非 2**：700 宽版式会把标题截到右侧之外，页面 4 倍高度又受 Chrome 截图上限（约 16384 px）限制；两段间隔 24 px。文章在 163 页面全长约 13000 CSS px，所需句子在约 11700 px 处 |
| 11-zh-163-wsj.png（1220×354） | https://www.163.com/dy/article/LCGC2BG805198NMR.html | render `--width 1000 --height 4200 --scale 1.4`（DPR 1.4）；stack 两段（标题与时间来源行；“不过整项成果没有经过人类数学家的人工审稿……”） | DPR 1.4，理由同上 |
| 12-emergent-claim.png（1400×576） | https://emergent.sh/news/claude-solves-vp-vs-vnp-arithmetic-complexity | render `--width 700 --height 3600`（DPR 2）；stack 两段（标题；要点列表含 “Independent validation by human mathematicians and automated proof checkers”） | 白底；页面带作者署名行与头像，已裁掉，发布时间 “Nov 24, 2026 10:31 AM”（时区未标）见页面 |

### 作废与待删除文件（均不在仓库内，仅为说明）

被作废的第一版裁切（04、05、06 的第一版，10、11 的第一版）已用 `mv` 移到会话 scratchpad 的 `badshots/`，不在仓库里；仓库 `images/` 里没有留下需要删除的 A 组文件。另一个误生成的占位文件 `12-placeholder-dummy.png` 同样已 `mv` 出仓库。中间渲染图在 `$TMPDIR/barking-shot/26111123-a/`。

### 未截

论文 Table 3（p.67）、HN 帖标题与分数、Viola 博客、Statement.lean 主定理片段、论文 Q.3 差异清单。
