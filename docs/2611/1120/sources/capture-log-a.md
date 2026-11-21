# 1120 期 A 组抓取记录

时间为北京时间（UTC+8，本机时区），取自各次命令输出。执行方 Claude Sonnet 5.5。命令前均已 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"`；opencli 浏览器扩展未连接，未使用。barking 记录文件：`a-fetch-log.jsonl`（fetch）、`a-shots-log.jsonl`（shot）、`a-heat.json`（heat）。

## 一、barking fetch（工具：`tools/barking/target/release/barking fetch docs/2611/1120 a <清单>`，默认 UA）

| 时间 | 清单 | name → URL | 结果 |
|---|---|---|---|
| 18:24:59 | `a-fetch-list-1.tsv` | sutherland → https://math.mit.edu/~drew/grh/ | ok 200 |
| 同上 | 同上 | alpoge-home → https://alpo.ge/ | ok 200（与 1119 存档逐字节相同） |
| 同上 | 同上 | lipponen-readme → github.com/aapolipponen/rh-lean-check-lumi（HTML）；lipponen-readme-raw、lipponen-docs-readme、lipponen-07、lipponen-10 → raw.githubusercontent.com 的 README.md、docs/README.md、docs/07-named-theorems.md、docs/10-open-frontier.md | ok 200 |
| 同上 | 同上 | tao → terrytao.wordpress.com/2026/11/20/unforced-euler-singularity-…/；aaronson → scottaaronson.blog/?p=10344；vakil → math216.wordpress.com/2026/11/20/hurried-announcement-alessandro-masullo-grh/ | ok 200 |
| 同上 | 同上 | masullo-pdf → …/the-music-of-the-primes.pdf（6,756,996 字节，4,094,155 字节文本）；remark-pdf → https://alpo.ge/remark.pdf（245,114 字节） | ok 200；remark SHA-256 与 1119 一致 |
| 18:25:22 | `a-fetch-list-2.tsv` | sutherland-audit-log、-nanoda-log、-con-leche-compare-log、-con-leche-master-log、-lean4lean-timeouts、-comparator-json、-sha256sums → https://math.mit.edu/~drew/grh/<文件> | ok 200（7 条） |
| 18:28:44 | `a-fetch-list-3.tsv` | sciam、techcrunch、nature、quanta-updates、sina、qq-0121、qq-0120、huxiu、anthropic-news、bostonglobe、mccorvie-readme、refine-report、glazer-teapawt、xena-buzzard（URL 见清单文件） | 全部 ok 200；Nature 为付费墙前部分；`.txt` 已逐一打开核对为正文 |
| 18:29:29 | `a-fetch-list-4.tsv` | 163-tao、ifanr、smzdm-tao、smzdm-tao2、ifeng-tech、sinacn-weibo、17173 | 全部 ok 200 |
| 18:31:29 | `a-fetch-list-5.tsv` | nielsen-philarchive（https://philarchive.org/rec/NIEIOT-2）、nielsen-philpapers-rec（https://philpapers.org/rec/NIEIOT-2）、nielsen-nieiom-pdf（https://philpapers.org/archive/NIEIOM.pdf） | **blocked 403**（Cloudflare 质询，`cf-mitigated: challenge`）；`a-nielsen-*.html` 是质询页，非正文。不绕过 |
| 同上 | 同上 | islam-zenodo → https://zenodo.org/records/24404855；mccorvie-repo → https://github.com/mccorvie/riemann-masullo | ok 200 |
| 18:31:46 | `a-fetch-list-6.tsv` | mccorvie-errata → raw.githubusercontent.com/mccorvie/riemann-masullo/main/ERRATA.md | ok 200 |
| 18:31:55 | `a-fetch-list-7.tsv` | refine-readme → raw …/music-of-primes-report-by-refine/main/README.md | **failed 404**（该仓库无 README），留下 `a-refine-readme.bin/.headers` 404 桩 |
| 18:32:05 | `a-fetch-list-8.tsv` | refine-tex → raw …/main/report.tex | ok 200 |
| 18:37:14 | `a-fetch-list-9.tsv` | sutherland-recheck、lipponen-readme-recheck | ok 200；与首次取档 `diff` 无差异 |

## 二、curl / Python（个别一次性请求）

| 时间 | 工具 | URL/操作 | 结果 |
|---|---|---|---|
| 18:25 | curl `--max-time 60` | https://api.github.com/repos/aapolipponen/rh-lean-check-lumi/commits?per_page=8 与仓库元数据 | ok；`a-lipponen-commits.json`。仓库 pushed_at 2026-11-20T23:59:33Z，最近提交 82d91dbb 23:56:59Z |
| 18:26 | curl | https://leanprover.zulipchat.com/#narrow/…/Discussion.3A.20Alp.C3.B6ge… | 200，但为 JS 壳页（“Public view of Lean”），无讨论串内容 |
| 18:26 | curl | https://leanprover-community.github.io/archive/ | 200，首页可打开，未使用（存档滞后） |
| 18:26 | curl | https://leanprover.zulipchat.com/json/messages（无 narrow 的 channel 名称） | 401 “Not logged in”；改 narrow 含 `channels: web-public` 后 400（频道名须用数字 id 113488）；再次 200 |
| 18:27:21–18:27:23 | `a-zulip-fetch.py`（Python 标准库，匿名接口，间隔 1 秒） | 讨论串 “Discussion: Alpöge Proves the Riemann Hypothesis” | ok：465 条，北京 11-20 21:58:06 至 11-21 18:25:04；原始 `a-zulip-alpoge-raw.json`（含邮箱、头像字段，待删），清洗后 `a-zulip-alpoge.json/.txt`（仅显示名、时间、正文） |
| 约 18:28 | 同上 | 讨论串 “Formalising Masullo's proof of GRH” | ok：106 条；原始 `a-zulip-masullo-raw.json`（待删），清洗后 `a-zulip-masullo.json/.txt` |
| 18:29 | curl | https://teapawt.org/ElliotGlazer、/AnthropicAI、/claudeai、/__alpoge__、/DarioAmodei | 200（X 镜像，页面存于临时目录，未入库）；仅用于确认帖子存在与时间 |
| 18:29 | curl | https://api.fxtwitter.com/status/2123853069599547786、2123845630124847322、2123853740574945473 | ok；`a-fx-<id>.json`（均为公开帖，含作者公开资料） |
| 18:29 | curl | https://api.fxtwitter.com/ElliotGlazer、/AnthropicAI | 404（fxtwitter 不提供用户页）；未再重试 |
| 18:30 | curl | https://www.reddit.com/r/math/s/qGW7nK8Ryt | 跳转到 `/r/math/comments/1y1b6l8/comment/…` 后只返回 8 KB 页面壳（登录/验证），无正文 |
| 18:30 | curl | https://old.reddit.com/r/math/comments/1y1b6l8/ ；https://www.reddit.com/r/math/comments/1y1b6l8/.json | 登录页重定向 / 403。不登录、不绕过，**失败，已停止** |

## 三、barking heat（18:30:45）

`barking heat docs/2611/1120 a --item 50574968 --item 50579568 --item 50578358 --item 50581084 --reddit "Riemann Hypothesis Alpöge" --reddit "Masullo Riemann" --reddit "Masullo GRH Vakil" --reddit "Theorem 11.1.2.1"`：4 条 HN ok（`a-hn-<id>.json`）；4 条 Reddit 搜索全部失败（响应为 old.reddit.com 登录页），记入 `a-heat.json` 的 errors。

## 四、截图与页面渲染（barking shot，headless Chrome）

除另注外为 700 CSS px 宽、DPR 2（图宽 1400 px）。逐张已打开目视确认。详细每步见 `a-shots-log.jsonl`。

| 时间 | 编号文件 | 来源与操作 | 备注 |
|---|---|---|---|
| 18:32 | `01-sutherland-summary-table.png` | `render` https://math.mit.edu/~drew/grh/ → `cropbox` [0,60,1400,3500] | 含页面“last updated 2026-11-21 09:30 UTC”行、Summary 段与全部结果表行；浅色底 |
| 18:32 | `02-sutherland-what-verified.png` | 同一渲染，`cropbox` [0,3520,1400,4140] | 代码块右端被页面自身横向滚动截断（页面如此） |
| 18:33 | `03-lipponen-readme-does-not-establish.png` | `render` github.com/aapolipponen/rh-lean-check-lumi/blob/main/README.md，`cropbox` [0,6740,1400,7920] | 首次裁切（[0,6700,…]）顶部切到半行，已作废换成此版（日志里首次文件名为 `03-…`、`11-lipponen-readme-limits.png` 与 `12b-…`，现存文件为 mv 改名后的版本） |
| 18:33 | `12-lipponen-readme-status.png` | 同上，`cropbox` [0,1260,1400,1860] | 状态行 “updated 2026-11-20 22:30 UTC”；首次 [0,1200,1400,2450] 作废 |
| 18:34 | `04-lipponen-docs07-true-placeholders.png` | `render` https://raw.githubusercontent.com/aapolipponen/rh-lean-check-lumi/main/docs/07-named-theorems.md（Markdown 源码视图），`cropbox` [0,13105,1400,13990] | 渲染页（github.com blob）在 700 px 宽下表格过长（>48000 px，OCR 不能处理），改取 raw 源码视图 |
| 18:35 | `05-vakil-announcement-update.png` | `render` Vakil 博文，700 宽，`cropbox` [0,440,1100,1245] | 页面右侧栏被视口截断（非裁切）；博文全文含更新段在内，下接 PDF 嵌入框未取 |
| 18:35 | `06-masullo-p6-acknowledgements.png` | `pdf` a-masullo-pdf.pdf 第 6 页，160 dpi → `cropbox` [100,130,1300,880] | PDF 渲页 160 dpi（宽 1360），不是 DPR 2 |
| 18:35 | `07-masullo-p199-return-here.png` | 第 199 页，160 dpi，`cropbox` [100,130,1300,1600] | 脚注 “1Return here.” 在页底 |
| 18:35 | `11-masullo-p522-ch13.png` | 第 522 页，160 dpi，`cropbox` [100,130,1300,1600] | 第 13 章正文页，含 “Theorem ??” 与三条脚注 |
| 18:36 | `08-tao-grh-paragraph.png` | `render` Tao 博文，**1000 CSS px 宽、DPR 1.4（图宽 1400）**，`cropbox` [490,440,1215,1315] | 700 px 宽版式会把正文截断，故改宽视口；清晰度略低于 2 倍；裁出标题到 “See also …” |
| 18:37 | `09-aaronson-post.png` | `render` https://scottaaronson.blog/?p=10344，**1000 px 宽、DPR 1.4**，`cropbox` [200,395,1180,2352] | 首次裁到 2340 切掉末行，已作废重裁；深色底外框，正文浅色 |
| 18:37 | `10-remark-p2-only-lean.png` | `pdf` a-remark-pdf.pdf 第 2 页，160 dpi，`cropbox` [100,130,1300,1600] | 即 PDF 第 2 页（含页码 2），“only present Lean”与脚注 2 |

### 目视检查结论

01–12 均已打开确认：文字清晰、范围正确。已知限制：02 代码块右端、05 页面右栏被页面自身版式截断；04 是源码视图；08、09 为 DPR 1.4；06、07、10、11 为 160 dpi PDF 渲页。

### 作废的中间图

两次裁切不当的图已改名移出仓库（`mv` 到临时目录，不在 `docs/` 下）：首次 `03`（顶部半行）、首次 `12`（顶部半行、底部切表格）、首次 `09`（末行被切）。

## 五、其他

- `a-fetch-list-*.tsv`：本组清单。`a-zulip-fetch.py`：Zulip 匿名取串脚本。
- 其他渲染中间图在 `/var/folders/…/barking-shot/26111120-a/`（`sutherland`、`lipponen-readme`、`lip07`、`lip07b`、`lip07raw`、`vakil`、`tao`、`taow`、`aaron`、`masullo-p4/p6/p199/p480/p522`、`remark-p1/p2`），不入库。
- 交付前对 `docs/2611/1120/` 做 grep：未发现抓取者的用户名或本机路径；X 内容仅 3 条公开帖的 fxtwitter JSON。
