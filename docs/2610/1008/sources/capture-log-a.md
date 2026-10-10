# 1008 期 A 组抓取日志

时间为北京时间（UTC+8）。本机时区美东，本机 10/8 夜间 = 北京 10/9 上午；以下“约 HH:MM”为命令发出时的近似值（会话内 `date -u` 首次读数 2026-10-09T01:37:27Z = 北京 09:37:27）。截图与裁切的精确时刻在 `a-shots-log.jsonl`。

工具：curl（普通浏览器 UA）、GitHub REST API（`api.github.com`）、`git clone --filter=blob:none --no-checkout`（blobless 克隆到会话临时目录，只用于 `git show`/`git diff --name-status`，不入库）、HN Algolia、Mastodon API、firecrawl（`firecrawl_scrape` / `firecrawl_search`）、poppler（Scoop：`~/scoop/apps/poppler/current/bin`：`pdftotext -enc UTF-8 -layout`、`-bbox-layout`、`pdftoppm -r 144`）、headless Chrome（`--headless=new --force-device-scale-factor=2 --window-size=700,H`；个别页注明）、Python 标准库 + Pillow。未使用 opencli / agent-reach（未尝试连接扩展）、未运行读取本机浏览器 Cookie 的工具、未登录任何站点。

## 1. 抓取

| 约时刻 | 对象 URL | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 09:37 | https://raw.githubusercontent.com/openai/math/main/history.md | curl | 200，2281 B | `a-history.md` |
| 09:37 | `api.github.com/repos/openai/math/commits/3014888`、`…/commits?per_page=10` | curl | 200；commit 的 files 数组只有 300 项（截断） | 改存摘要 `a-commit-3014888.summary.json`、`a-math-commits.json` |
| 09:38 | `api.github.com/repos/openai/math/git/trees/{adc7f12,3014888,fd4aeeb}?recursive=1` | curl | 200，各 15 MB，`truncated: true`（5.8 万条），不可用 | **已删除**（试验文件）；改用 blobless 克隆 |
| 09:39 | `https://github.com/openai/math.git` | git blobless clone | 成功（约 17 s） | 仅会话临时目录 |
| 09:40 | `git diff --name-status adc7f12 3014888 / fd4aeeb` | git | 1185 条（3014888 与 fd4aeeb 树相同） | `a-commit-3014888-name-status.txt`、`a-3014888-to-fd4aeeb-name-status.txt`（空） |
| 09:41 | `git show` 三篇撤稿 README（fd4aeeb 与 adc7f12）、`lean/formalization.yaml`、`README.md`、`CONTENTS.md`（两版） | git | 成功；1006 存档的 yaml/README/CONTENTS 与 adc7f12 逐字节相同（`cmp`） | `a-withdrawn-README-*.md`、`a-formalization-fd4aeeb.yaml`、`a-math-README-fd4aeeb.md`；脚本 `a-yaml-diff.py`，输出 `a-yaml-diff.out.txt` |
| 09:42 | https://arxiv.org/abs/2610.08144 | curl | 200 | `a-ns-arxiv-abs.html` |
| 09:42 | https://arxiv.org/pdf/2610.08144v1 | curl | 200，1.7 MB，25 页 | `a-ns-lean-critique-v1.pdf`、`.txt` |
| 09:42 | http://export.arxiv.org/api/query?id_list=2610.08144 | curl | 301（http），改 https 后 200 | `a-ns-arxiv-api.xml`（正文未用） |
| 09:43 | https://cdn.openai.com/pdf/32d9f210-…/navier-stokes.pdf | curl | 200，2.9 MB，167 页 | `a-oai-ns-paper.pdf`、`.txt` |
| 09:43 | https://openai.com/index/navier-stokes-solution/ | curl | **403**（Cloudflare 挑战页，10 KB） | 已删除该无效 html |
| 09:44 | 同上 | firecrawl_scrape（markdown + rawHtml，maxAge 0） | 200，正文取得 | `a-oai-ns-page.md`、`a-oai-ns-page.html` |
| 09:45 | `api.github.com/repos/openai/NavierStokesAndEuler`、`…/commits`、`…/compare/8937a8f...f9e8bc5`、issues/pulls | curl | repo/commits/compare/issues 200；`pulls` **404**；compare 的 files 数组 188 项，后段 additions/deletions 显示 0（API 统计不全） | `a-nsrepo-info.json`、`a-nsrepo-commits.json`、`a-nsrepo-compare.summary.json`（原 1.2 MB json 已删，改存摘要） |
| 09:46 | `https://github.com/openai/NavierStokesAndEuler.git` | git blobless clone | 成功 | 仅会话临时目录；`a-nsrepo-8937a8f-to-f9e8bc5-name-status.txt`、`a-nsrepo-README-*.md`、`a-nsrepo-formalization-f9e8bc5.yaml`、`a-lean-*.lean`（片段） |
| 09:47 | HN Algolia `search`（OpenAI Withdraws、Navier-Stokes lost in translation、Math 2.0、Mathocalypse、karagila.org、ahmath.org、AGMAI、Navier-Stokes Lean、terrytao） | curl | 200 | 结果见 `a-heat.json` |
| 09:48 | https://www.ahmath.org/statements、/members、/home、/faq、/activities | curl | 200 | `a-ahm-*.html/.txt` |
| 09:48 | https://agmai.org/、https://agmai.org/general-sep29/ | curl | 200 | `a-agmai-*.html/.txt` |
| 09:48 | https://karagila.org/2026/openai-pp/、http://karagila.org/feed.xml | curl | 200 | `a-karagila.*`、`a-karagila-feed.xml` |
| 09:48 | https://scottaaronson.blog/?p=10169 | curl | 200 | `a-aaronson.html/.txt` |
| 09:48 | https://mathstodon.xyz/@tao/117395267721642920；`/api/v1/statuses/117395269325940185` 与 `/context` | curl | 200 | `a-tao-mastodon-*` |
| 09:48 | https://terrytao.wordpress.com/2026/09/22/why-i-agreed-to-join-agmai/、/、/2026/10/07/ahm-statement-…/ | curl | 200 | `a-tao-blog-*.html/.txt` |
| 09:48 | https://openai.com/index/sharing-ai-progress-in-mathematics/ | curl | **403** | 已删除该无效 html |
| 09:48 | https://twitter.com/danintheory/status/2108065033070789090 | curl（未登录） | 200（页面 `<title>`、og 描述、`article:published_time` 可读） | `a-x-danintheory.html`（未登录抓取，不含抓取者信息） |
| 09:50 | https://archive.org/wayback/available…、web.archive.org/cdx… | curl | **429 / “Temporarily Offline”**，未取到 | 无 |
| 09:52 | `a-heat.py`：HN `items/<id>` ×13 | python urllib | 首次 **SSL EOF**，加 User-Agent 与重试后成功；取得时刻 2026-10-09T01:52:15Z（北京 09:52:15） | `a-heat.json`、`a-hn-<id>.json` |
| 09:53 | https://www.reddit.com/search.json?… | curl | **403**（Reddit 拦截）；一次 schannel 握手失败 | 无 |
| 09:53 | https://www.reddit.com/r/math/comments/1x02ein/… | firecrawl_scrape | **不支持该站点** | 无 |
| 09:54 | https://old.reddit.com/r/math/comments/1x02ein/… | 内置浏览器（navigate） | **被安全限制拒绝**，不硬绕 | 无 |
| 09:53–09:56 | firecrawl_search ×6（Reddit 摘要、质疑论文、中文撤稿、中文 NS Lean 质疑、AHM/陶哲轩中文、OpenAI 对质疑的回应） | firecrawl | 只得链接与摘要 | 结果引用在 `evidence-a.md` |
| 09:55 | m.36kr.com/p/4017059444330631、k.sina.com.cn/…、m.sohu.com/a/1085183213_115831、news.pedaily.cn/…、winzheng.com/… | curl | 200（36kr 页面正文为空壳 170 字，未采用） | `a-sina-withdraw.*`、`a-sohu-cnmo.*`、`a-pedaily.*`、`a-winzheng.*`、`a-m36kr-withdraw.*` |
| 09:55 | damtp.cam.ac.uk 上的 `Navier-Stokes_Experiment.pdf`（7 页）、`NavierStokes_LostInTranslation_Final.pdf`（31 页） | curl | 200 | `a-damtp-ns-experiment.pdf/.txt`、`a-damtp-ns-final.pdf/.txt` |
| 09:57 | https://www.163.com/dy/article/L8PLN2CK0511DSSR.html | curl | 200 | `a-163-tao-boycott.html/.txt` |

## 2. 截图

2 倍像素比，宽 ≤ 1400 px。裁切边界落在文字行之间；PDF 页用 pdftoppm 144 dpi（宽 1191/1224 px）。逐张已用 Read 打开目视确认（文字清楚、范围正确）；05、06、08 在最后一次改裁切逻辑后重看过；04 同。

| 文件 | 来源 | 方法 | 备注 |
|---|---|---|---|
| 01-history-withdrawals.png、02-history-fixes-formalized.png | https://github.com/openai/math/blob/main/history.md | headless Chrome 700 CSS px（窗口高 1800），`a-shot.py cropbox`（北京 10:01:45） | 01：页头到撤稿三条与“now carry notices”；02：Fixes 全部与 300/719 句。页面是 GitHub 的渲染预览。 |
| 03-crit-example31.png | 质疑论文 PDF p.4（3.1 与 Example 3.1 至 Summary）+ p.6（源论文第 96 页引文与“five additional derivatives”句） | `a-pdf-shots.py build`，配置 `a-pdf-shots-config.py` | 两页拼接，保留页眉页码 |
| 04-crit-figure3.png | 质疑论文 PDF p.5 整页 | 同上 | Figure 3（ChatGPT 对话截图）及图注；对话里的行号与 f9e8bc5 不符，见 evidence A3-5 |
| 05-crit-example33.png | PDF p.8（Example 3.3 与 (3.4)(3.5)）+ p.10（三条原因） | 同上 | |
| 06-crit-disclaimer.png | PDF p.2（“merely tells us”与 Disclaimer） | 同上 | |
| 07-oai-819.png | OpenAI 论文 p.95（Lemma 8.6、(8.19)）+ p.96（“Four more derivatives…”段） | 同上 | 页眉含页码 |
| 08-oai-1019.png | OpenAI 论文 p.122（Pressure flux 起）+ p.123（至 (10.19)） | 同上 | 页眉含页码 |
| 09-lean-m5-theorem.png | `NavierStokes/SmoothFamilyTorusInverse.lean` 第 1058–1080 行 @f9e8bc5 | **本地等宽渲染**（`a-lean-render.py`，文本取自 `git show`，加仓库/提交/路径标题行；字符未改） | GitHub 的 blob 页（带 `#L1059-L1080`）在 headless Chrome 下用 `--virtual-time-budget=20000` 渲染为**空白页**（大文件），故改本地渲染；字体回退使个别符号间距与 GitHub 不同 |
| 10-oai-page-lean.png | https://openai.com/index/navier-stokes-solution/ | headless Chrome 700 CSS px，`--virtual-time-budget=25000` 与普通 Chrome UA 才通过挑战页（首次渲染只得到加载图标，已丢弃）；`a-stack.py` 拼接日期标题 + 首段 + “The result” 段 | 页面日期 “September 8, 2026”入图 |
| 11-ahm-statement.png | https://www.ahmath.org/statements | headless Chrome 700 CSS px，`cropbox` | 声明全文（标题到署名） |
| 12-karagila-desk-rejection.png | https://karagila.org/2026/openai-pp/ | headless Chrome 700 CSS px，`a-stack.py`：标题与日期 + “desk rejection”起的三段 | 页面写 “Oct 08 2026, 10:25”（时区见 RSS +0100） |
| 13-tao-blog-ahm-guestpost.png | https://terrytao.wordpress.com/2026/10/07/ahm-statement-on-openais-october-6-release-of-mathematical-documents/ | headless Chrome **1100 CSS px** 宽（700 px 下主题布局溢出，文字被右侧截掉），DPR 2，裁文章栏 1040 px 宽 | 含 guest post 说明 |
| 14-agmai-stop-testing.png | https://agmai.org/general-sep29/ | headless Chrome 700 CSS px，`a-stack.py` | 含 “September 29, 2026”与首段 |

`a-shots-log.jsonl` 里同名文件（12、10）出现多次，是我在目视检查后因裁切边界过紧而删除重做的试验版本（旧版文件已覆盖删除，只留最后一版）；另有一条 `taoahm_wide` 渲染记录为事后手工补记（时间标“约”）。

## 3. 我删除的自己生成的文件（均为试验或无效文件）

- `a-tree-adc7f12.json`、`a-tree-3014888.json`、`a-tree-fd4aeeb.json`（各 15 MB、被 API 截断，不可用）。
- `a-oai-ns-page.html` 的首个 curl 版本（403 挑战页）、`a-oai-sharing-progress.html`（403）；后者同名未保留，前者被 firecrawl 版本替换。
- `a-commit-3014888.json`（1.1 MB）、`a-nsrepo-compare.json`（1.2 MB）：替换为去掉 patch 的 `.summary.json`。
- `a-x-tmp.txt`、`/tmp/x.txt`（html2txt 试验输出）。
- 若干截图试验版本：`images/03`–`08`（改裁切算法后重做）、`images/05`（再延长）、`images/10`（中段裁入标题残行）、`images/12`（两次，边界）。

## 4. 单个超过 1 MB 的文件（请决定是否入库）

只有 PDF（不入库，传网盘）：`a-ns-lean-critique-v1.pdf`（1.7 MB）、`a-damtp-ns-final.pdf`（1.7 MB）、`a-oai-ns-paper.pdf`（2.9 MB）。文本文件中最大的是 `a-oai-ns-paper.txt`（约 0.68 MB），不超限。`a-oai-ns-page.html`（0.48 MB）与各 `.html` 原件属“HTML 原件只存本地”。

## 5. 抓取者信息自查

对 `docs/2610/1008/` 下本组文件 grep 了 `develata`、本机用户名、邮箱：只命中网页自带内容（AHM 联系邮箱 ahmathorg@gmail.com、网站“腾讯QQ”分享按钮、PDF 二进制噪声）。`a-x-danintheory.html` 是未登录抓取的公开页面，不含抓取者头像/显示名。Mastodon 上下文 JSON（`a-tao-mastodon-context.json`）含他人公开回复，未改动。

## 6. 本组产出文件清单

`sources/`：`evidence-a.md`、`capture-log-a.md`、脚本 `a-yaml-diff.py`、`a-pdf-shots.py`（+`a-pdf-shots-config.py`）、`a-shot.py`、`a-stack.py`、`a-lean-render.py`、`a-heat.py`、`a-html2txt.py`，以及上表各存档与 `a-shots-log.jsonl`。`images/`：01–14。
