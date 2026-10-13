# 1013 期 A 组抓取日志（Codex CLI 默认常驻守护进程，openai/codex #48195）

时间为北京时间（UTC+8）；本机美东（EDT）= 北京 − 12 小时；UTC = 北京 − 8 小时。时刻取自命令发出时的 `date -u` 读数与脚本记录（`a-browser-records.jsonl`），为近似值。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。

工具：`gh api`（已登录的只读调用，只 GET；未加 reaction、未 star、未 subscribe、未评论）；curl（浏览器 UA，匿名）；`npm view` / npm registry JSON（只查元数据，**未安装**）；Playwright Core + 本机 Chrome（匿名、全新临时 profile、不登录、不带 Cookie、en-US、UA 去掉 Headless 标记；脚本 `a-browser.mjs`、`a-fetch.mjs`、`a-gh-shot.mjs`、`a-xshot.mjs`，前三者改写自 1012 同名脚本，`a-xshot.mjs` 本期未用）；HN Algolia 与 Firebase API（`a-heat.py`）；WebSearch（4 次）、firecrawl_search（仅 1 次，额度告急，未用 scrape）。未使用 opencli / agent-reach，未登录任何站点，未运行读取浏览器 Cookie 的工具，**未运行 Codex CLI，未启动守护进程**。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 19:26 | `repos/openai/codex/issues/48195`、`…/comments`、`…/timeline` | gh api | 200；16 条评论，timeline 120 事件 | `a-issue-48195.json`、`a-issue-48195-comments.json`、`a-issue-48195-timeline.json`、`a-issue-48195.txt`（我用 python 把正文与评论拼成文本） |
| 19:26 | `repos/openai/codex/commits/main`、`repos/openai/codex`、`releases`、npm view `@openai/codex` | gh api、npm view | main = `90f00bd`（2026-10-13T10:43:13Z）；npm latest 0.162.1 | `a-main-head.txt`、`a-npm-latest.json` |
| 19:27–19:31 | `contents/<path>?ref=90f00bd…`（features/lib.rs、tui/startup_orchestration.rs、tui/daemon_startup.rs、tui/cli.rs、cli/main.rs、app-server-daemon 的 README 与 10 个源文件、install-context/lib.rs、scripts/install/install.sh） | gh api（raw） | 全部 200 | `a-src-*`（单文件最大 180 KB） |
| 19:30 | `git/trees/90f00bd…?recursive=1`（用于列文件，存 scratchpad，不入库）；`a-tree-app-server-daemon.tsv` 为其中 app-server-daemon 目录的文件清单 | gh api | 200，10137 项，未截断 | `a-tree-app-server-daemon.tsv` |
| 19:31–19:33 | `commits?path=…`（app-server-daemon、features/lib.rs）、`search/commits?q=daemon_auto_start`、`commits/<sha>`（75ec81c、653e5fbb、70e8fe1b、b0659c53、2cc65cdd、6b43e6fe、27065a91、d6093d32、8f82b8a3）、`compare/<a>...<tag>`（0.155–0.157 各标签） | gh api | 全部 200；`search/issues?q=daemon_auto_start…is:pr` 仅命中 #47179 | `a-git-log-app-server-daemon.tsv`、`a-git-log-features-lib.tsv`、`a-commit-75ec81c862.json`、`a-commit-messages-daemon.txt` |
| 19:33 | `releases/tags/rust-v0.156.0…v0.162.1`（14 个）、`releases/latest` | gh api | `releases?per_page=100` 一次性分页 **504（HTTP 超时）**——改为按标签逐个取，全部成功；失败产生的空文件已删除（自己刚生成的试验文件） | `a-release-rust-v0.15x…v0.162.1.json` |
| 19:34 | https://developers.openai.com/codex/config-reference 、`/codex/changelog`、`/codex/config-basic`、`/codex/cli/reference`、`/codex/cli/features` | curl（浏览器 UA）+ `a-html2txt.py` | 全部 200（0.4–1.4 MB HTML）；页面是静态渲染含正文，未遇 challenge | `a-doc-*.html/.headers/.txt`（HTML 不入库；其中 config-reference 1.4 MB） |
| 19:35 | 相关 issue：#53547、#40969、#41188、#46468、#48050、#42457、#37537、#48778、#49326、#50299、#51201、#52067、#52648、#51763、#51961、#49418、#51787 及各评论 | gh api | 除 **#37537 404**（issues 与 pulls 路径都 404；页面 302→404）外全部 200；#37537 的 JSON 文件为 404 体，已删除 | `a-issue-<n>.json`、`a-comments-<n>.json` |
| 19:40 | `ml-pkgs` commit b51c036；nixpkgs PR #567051 及其 files、issue #567047 | gh api | 200 | `a-ml-pkgs-b51c036.json`、`a-nixpkgs-pr-567051.json` |
| 19:40 | Homebrew / Scoop / AUR / Arch 包源：`search/issues`（org:Homebrew、repo:ScoopInstaller/Main、repo:NixOS/nixpkgs）；`aur.archlinux.org/rpc` 与 cgit PKGBUILD（openai-codex-bin/-git/-reasoning）；`archlinux.org/packages/search/json`；`gitlab.archlinux.org/…/openai-codex/-/raw/main/PKGBUILD`；`Homebrew/homebrew-cask/Casks/c/codex.rb`（gh api）；本机 `scoop cat codex`（只读读 manifest） | gh api / curl / scoop | 均 200；结论见 evidence-a A9.3 | 无单独存档（摘句在 evidence-a） |
| 19:41 | HN：Algolia `items/50060694`、Firebase `item/50060694`、`topstories` | curl | 200；572/276，首页第 33 | `a-hn-50060694.json`、`a-hn-50060694-fb.json`、`a-hn-50060694.txt`（`a-hn-flatten.py` 生成）、`a-hn-50060694-comments-selected.txt` |
| 19:42 | firecrawl_search 一次（“Codex CLI background daemon default auto-update…”） | firecrawl | 成功，返回 15 条摘要，并提示**账户额度不足**；后续不再调用，未用 scrape | 无（仅线索） |
| 19:42–20:02 | WebSearch 4 次（英文 2、中文 2；标准与扩展各半） | WebSearch | 返回二手报道与 issue 链接；其中 dev.to “robust_true_try” 一篇的细节（搜索摘要自评与其他来源明显不符）**未采信** | 无 |
| 19:42 | developersdigest、El Solitario、dev.to（thereleasedesk）、getmoshi、dev.to（robust_true_try）、byteiota、蓝点网、daily.dev、danielvaughan 博客、lilting.ch | curl | 全部 200 | `a-m-<站名>.html/.headers/.txt`（HTML 不入库）；dev.to（robust_true_try）、getmoshi、lilting、danielvaughan 仅存档，证据表未引 |
| 19:43–19:45 | https://github.com/openai/codex/issues/48195 | Playwright，`a-browser.mjs`（匿名，CSS 1100，DPR 2） | 200，匿名渲染页（显示 “Sign in / Sign up”）；渲染文本存档；截图 01、02（先在 scratchpad 试截看版面，未入库） | `a-gh-issue-48195-rendered.txt`、`images/01-issue-48195-top.png`、`images/02-issue-48195-request-workaround.png`；脚本参数 `a-shots-issue-48195.json` |
| 19:46–19:47 | GitHub blob permalink：`…/blob/90f00bd…/codex-rs/features/src/lib.rs#L1027-L1033`、`…/tui/src/startup_orchestration.rs#L508-L511` | Playwright，`a-gh-shot.mjs`（匿名，视口 1100，DPR 2） | 成功；截到左侧文件树与分支 “90f00bd”，高亮行即定义/条件处 | `images/04-features-lib-daemon-auto-start.png`、`images/05-startup-auto-start-condition.png` |
| 19:48 | https://github.com/openai/codex/releases/tag/rust-v0.157.0 | 同上 | 成功 | `images/06-release-0157-notes.png` |
| 19:49–19:50 | https://github.com/openai/codex/issues/53547 | `a-gh-shot.mjs` 试截 → `a-browser.mjs` | 成功；渲染文本存档；07 标题区、08 描述段、09 预期行为段 | `a-gh-issue-53547-rendered.txt`、`images/07-…`、`08-…`、`09-…`；`a-shots-issue-53547.json` |
| 19:51 | https://github.com/openai/codex/commit/75ec81c8623c0ca50b129de5a82521a23d37b726 | `a-browser.mjs`（首次因元素定位失败 `end missing: GitOrigin-RevId`，**仅存档了渲染文本**，无图）→ 改用 `a-gh-shot.mjs` 以坐标截图 | 成功：10（提交说明）、11（lib.rs diff；先截一版，后以 `OVERWRITE=1` 去掉顶部 30 px 重截覆盖了自己的上一版；顶部仍露出半行 “selected.”，是 diff 第 109 行长注释的折行尾部，为 GitHub 粘性文件头下的原页内容，已打开确认，保留） | `a-gh-commit-75ec81c-rendered.txt`、`images/10-…`、`11-…`；`a-shots-commit-47179.json` 为失败那次的参数 |
| 19:54–19:56 | …/blob/90f00bd…/codex-rs/app-server-daemon/README.md | `a-browser.mjs`（首次定位错位，截图从段落中间开始，**已删除重做**）→ `a-gh-shot.mjs`（滚动到 y=700 后截 945 px） | 成功；README 渲染页文本存档；12 号图用 `OVERWRITE=1` 覆盖了我自己刚生成的上一版（非他人文件）；最终版（高 945 px）已重新打开确认：环境变量段、5 分钟/每小时段、`settings.json` 示例完整，底部落在代码块之后 | `a-gh-asd-readme-rendered.txt`、`images/12-daemon-readme-env-update.png`；`a-shots-readme.json` 为失败那次的参数 |
| 19:59 | https://www.reddit.com/r/codex/comments/1wqjqim/… | Playwright，`a-fetch.mjs` | 200；Reddit 页面在浏览器渲染时自带一次 JS 校验跳转（`js_challenge` 参数），由浏览器正常完成，**未做任何额外绕过**；匿名页不显示评论得分；最终 URL 带的 token 未写入日志 | `a-reddit-codex-1wqjqim.txt` |
| 19:59–20:00 | Reddit 站内搜索：r/codex、全站、r/OpenAI、r/ChatGPTCoding（关键词 “daemon”/“codex daemon”，sort=new） | `a-fetch.mjs` | 200；r/ChatGPTCoding 无结果；r/OpenAI 无相关；r/codex 有 2 个 9/26 前后的相关帖 | `a-reddit-search-codex-daemon.txt`、`a-reddit-search-all.txt`、`a-reddit-search-openai.txt`、`a-reddit-search-ccoding.txt` |
| 20:00 | https://aihot.news/ 、`/hot` | `a-fetch.mjs`（匿名 Playwright；curl 此前在 1012 期遇 bot 挑战页，本期未用 curl 试） | 200，渲染成功；首页有本条（评分 66），`/hot` 无 | `a-aihot-home.txt`、`a-aihot-hot.txt` |
| 20:04 | HN Firebase/Algolia/topstories + GitHub issue 计数 | `a-heat.py` | 574/277，首页第 35；#48195 reactions 179 | `a-heat.json` |
| 20:05 | `search/issues?q=repo:openai/codex is:pr daemon created:>2026-10-09`、`commits?sha=main&since=2026-10-12` | gh api | 仅 #53173、#53631（均为功能/测试修补）；main HEAD 仍为 `90f00bd` | `a-prs-daemon-since-1009.tsv` |

**X（推特）**：没有可用的 X 搜索入口；fxtwitter 只能按已知 status id 取帖，本组没有已知 id，故**未抓取任何 X 页面**；A2.6 因此只能写“未见（检索受限）”。

## 2. 失败 / 受限记录

- 官方文档站 `developers.openai.com` 的 `/codex/app-server` 页未逐页核是否写到 daemon（A6.3 标“部分支持”）。
- `releases?per_page=100` 分页 504，改按标签取（见上）。
- #37537 404（issues/pulls 均不存在）。
- firecrawl 额度不足，仅 1 次搜索；未用 scrape。
- Reddit/HN 评论得分不可得；X 无检索。
- commit 页与 README 页的首次自动定位截图失败，已改坐标截图并重做；试验图（仅在 scratchpad 的 `probe*.png`、`i53547.png`、`rel.png`，以及 images/ 里首版错位的 12 号图）均已删除或覆盖，且都是我自己生成的。

## 3. 本组新增文件

文本（可入库）：`evidence-a.md`、`capture-log-a.md`、`a-*.txt`、`a-*.json`、`a-*.tsv`、`a-*.md`（无）、`a-src-*`（源码与脚本存档，含 `.rs/.sh/.md`）、`a-*.mjs`、`a-*.py`、`a-browser-records.jsonl`、`a-*.headers`。
**只存本地、不入库**：`a-doc-*.html`、`a-m-*.html`（媒体与官方文档页 HTML 原件）、`images/*.png`。
大于 1 MB 的文件：`a-doc-config-reference.html`（约 1.4 MB，HTML 原件，不入库）、`a-doc-cli-reference.html`（约 1.0 MB，同）。文本类文件均小于 1 MB（最大 `a-issue-48195-timeline.json`、`a-m-*.txt` 等为数百 KB）。

## 4. 交付前自查

- 对 `docs/2610/1013/` 中本组文件 grep：维护者 GitHub 登录名（用 `gh api user --jq .login` 取到后 grep）、本机用户名 `QQ` 的绝对路径、邮箱、`C:/Users`、`AppData`：**本组文件无命中**（grep “QQ” 只命中 base64/节点 ID 与他人 HTML 中的字符串，非用户名）。脚本里的 Chrome 路径是 `C:/Program Files/Google/Chrome/Application/chrome.exe`（非用户目录），Playwright 路径用 `os.homedir()`，无用户名。截图是匿名渲染页，右上角为 “Sign in / Sign up”。
- 注意（非本组文件）：同一目录下 `evidence-d.md` 含有维护者 GitHub 登录名（自查时发现，D 组的内容，我未动，请 Claude 合并前处理）。
- 11 张截图（01、02、04–12）已逐张打开目视确认文字清楚、范围正确；03 空缺（无需要）。
- `git status --short`：只新增 `docs/2610/1013/` 下文件（含他组与派工单）；未改动仓库其他文件，未 commit、未 push，未删除他人文件。
