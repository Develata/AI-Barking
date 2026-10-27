# A 组抓取日志（1026 期，OpenAI 与 Anthropic 同日称推翻 ETH）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；本机美东（EDT），北京 = 本机 + 12 小时。机器记录另见 `a-fetch-log.jsonl`（每条含 time_bj、sha256）、`a-shots-log.jsonl`、`a-heat.json`。所有抓取未登录、未发帖；curl 与 barking 不带 Cookie；opencli 浏览器会话只用于打开公开页面截图。HTML/PDF/图片原件只存本地。浏览器里装了翻译插件，截图前用 `eval` 把插件悬浮图标（`#immersive-translate-popup`）设为 `display:none`，并把页面 `zoom` 设为 2；OpenAI 博文还隐藏了左侧固定目录（否则与正文重叠）。这些只改页面外壳，不改正文。

## 一、barking fetch 批量存档（`a-<name>.html|.pdf|.txt|.headers`）

| 时间 | name | URL | 结果 |
|---|---|---|---|
| 22:53 | oa-blog | https://openai.com/index/building-scientific-understanding-with-academic-communities/ | ok（370 KB，294 行文本；curl 直接成功，未被 403；正文完整） |
| 22:53 | oa-preprints-dir | https://github.com/openai/math/tree/main/preprints | ok（目录页 HTML 773 KB，仅本地；只用于找文件夹名） |
| 22:53 | oa-lean378 / oa-lean079 | raw.githubusercontent.com/openai/math/main/lean/docs/378.md、079.md | ok |
| 22:53 | an-readme / an-paper | raw.githubusercontent.com/anthropics/formal-math/main/eth/README.md、paper.pdf | ok（PDF 900 KB） |
| 22:53 | agmai-home / agmai-about | https://agmai.org/ 、https://agmai.org/about | agmai-home ok（含全部声明）；agmai-about 取得的是 WordPress 示例页（“This is an example of a page”），无用 |
| 22:53 | hn-oa-blog / hn-oa-eth | HN Algolia 搜索 JSON | ok |
| 22:54 | oa-eth-paper / oa-ls-paper | raw…/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf、…/Local-smoothing-for-the-Euclidean-wave-equation-October-25-2026/paper.pdf | ok（888 KB、810 KB；pdftotext 3815、3403 行） |
| 22:54 | an-eth-mainresults / an-eth-challenge / an-repo-readme / oa-math-readme | raw.githubusercontent.com（Anthropic eth/MainResults.lean、Challenge.lean、根 README；OpenAI 根 README） | ok |
| 22:56 | oa-lean-sat / oa-lean-ls / oa-lean-readme / oa-history / oa-comp-readme / oa-formalization-yaml | raw.githubusercontent.com/openai/math/main/lean/ComparatorChallenges/*.lean、lean/README.md、history.md、lean/formalization.yaml | ok（yaml 的 txt 82 KB） |
| 22:59 | hexagon-eth / hexagon-abs / hexagon-home | https://hexagonmath.org/pdf/2610.00577v1 、/abs/2610.00577 、/ | ok（PDF 按 180 秒重取） |
| 22:59 | hn-item-blog / hn-item-eth / hn-item-an | HN Algolia items 50251344 / 50251661 / 50255798 | ok（JSON，含全部评论） |
| 22:59 | anthropic-news | https://www.anthropic.com/news | ok（422 KB；无 ETH 条目） |
| 23:03 | agmai-general | https://agmai.org/general-sep29/ | ok |
| 23:03 | agmai-sep22 / oct6 / oct25 / oct26 | 我猜的子页 URL（about-agmai、openai-release、disadvantages-designating-mathematicians、request-immediate-release） | **failed 404**（声明都在首页锚点，没有独立页面；404 页已存，无用，列待删除） |
| 23:03 | ias-nccr | https://www.ias.edu/nccr/projects | **blocked 403**（Cloudflare 质询）；改用 opencli 浏览器，文本摘录存 `a-ias-nccr-agmai.txt` |
| 23:03 | agmai-feed | https://agmai.org/feed/ | ok（只有频道头，无条目；仅取 lastBuildDate） |
| 23:2x | media-mindstudio / media-uniteai / media-pillitteri | 三篇媒体（URL 见 evidence-a.md 第 3 节） | ok |
| 23:2x | media-xenospectrum / media-fourweekmba | https://xenospectrum.com/en/openai-exponential-time-hypothesis-false-claim/ 、https://fourweekmba.com/ai-openai-says-its-ai-refuted-the-exponential-time-hypothesis/ | **blocked 403**，未重试、未取得，不引用 |
| 23:3x | media-tnw / media-forklog / media-officechai | 三篇媒体 | ok |
| 23:3x | media-sciam / media-cryptobriefing / media-devto-3 / media-aiweekly | SciAm、Crypto Briefing、DEV、AI Weekly | ok（SciAm 全文可读，未遇付费墙） |
| 23:4x | oa-advisory / oa-oct6 | https://openai.com/index/advisory-group-on-mathematics-and-ai/ 、https://openai.com/index/sharing-ai-progress-in-mathematics/ | ok（oa-oct6 存档但未引用） |

清单文件：`a-list1.tsv`…`a-list9.tsv`。

## 二、个别请求（curl，`--max-time 60`）

| 时间 | URL | 结果 |
|---|---|---|
| 22:55 | GitHub API：`repos/anthropics/formal-math/commits?path=eth`、`…/commits?per_page=10`、`repos/anthropics/formal-math`；`repos/openai/math/commits?…`（三条路径过滤与总表）；`repos/openai/math/contents/lean`、`…/SubexponentialSAT`、`…/SqrtLogSpace` | ok，存 `a-an-commits-eth.json`、`a-gh-*.json`、`a-gh-openai-*.json`（匿名，未带 token；用了约 10 次） |
| 23:0x | https://api.fxtwitter.com/status/2114891838377320784、…837265834264、…835403633038（Williams 10-27 串）；…/2114833319431209352 与 `/2/thread/…`（Williams 10-26 串）；…/2115047886396227873（Cowen）；…/2114833687296847930（Dan Roberts） | ok，存 `a-fx-*.json`（公开 JSON；含作者公开资料字段，作者均为公开的研究者/商业人士，非抓取者） |
| 23:0x | https://blog.computationalcomplexity.org/ | ok，`a-fortnow-blog.html`（无 ETH 帖） |
| 23:0x | https://scottaaronson.blog/ 与 `/?p=10239` | ok，`a-aaronson-home.html`、`a-aaronson-post.html`（含评论 #4、#29） |
| 23:1x | https://terrytao.wordpress.com/ ；https://mathstodon.xyz/@tao | ok；mathstodon 为 JS 壳，无内容 |
| 23:3x | https://docs.google.com/document/d/1adB0IhJgLzsmbfCgoLrtoJTjnlPf74UnyjMo35P5JHo/export?format=txt | ok，4,844 字节，存 `a-eth-group-letter.txt`（可变文档，无时间戳） |
| 23:3x | https://ethdiscussion.blog/ | ok，`a-ethdiscussion.html` |
| 23:3x | `a-hn-search-eth2.json`、`a-hn-search-pnp.json`（HN Algolia search_by_date） | ok，无新增相关条目 |

## 三、barking heat（22:59）

`barking heat docs/2610/1026 a --hn …` 六组查询（Exponential Time Hypothesis / Three new discoveries… / 2^o(n) Time Algorithm for 3-SAT / anthropics formal-math eth / Yang-Mills OpenAI / AGMAI）：HN 全部成功；未查 Reddit（派工单无已知 Reddit 帖）。结果 `a-heat.json`。数字见 evidence-a.md 的 A11。

## 四、WebSearch（只作线索，搜索摘要不作来源）

约 8 次（英文 6、中文 1、Anthropic 公告 1）。找到的 SciAm、TNW、MindStudio、Crypto Briefing 等都已逐一用 fetch 存档后再引用；两次搜索未命中 Anthropic 官方公告、中文媒体。

## 五、opencli 浏览器与 barking shot

会话名 `a-shot`（后台窗口 `--window background`，均未触发 `captureScreenshot` 超时）。

| 时间 | 页面 | 动作与结果 |
|---|---|---|
| 23:05 | OpenAI 博文 | `open` → `eval zoom=2` → `screenshot --width 1400 --full-page`（1400×20317）。第一次左侧固定目录与正文重叠、右侧有翻译插件图标，弃用（移至 scratchpad，未入库）；隐藏目录与插件图标后重截 `oa-blog-full3.png`，裁出 01–04 |
| 23:06 | OpenAI 博文 | `barking shot render`（headless Chrome，700×9000，DPR 2）输出整页空白，弃用 |
| 23:11 | OpenAI ETH 预印本 PDF、Anthropic PDF | `barking shot pdf`（144 dpi）第 1、2 页与 Anthropic 第 3、40 页；裁切拼接出 05、10 |
| 23:12 | GitHub 378.md / 079.md | opencli 截图，裁掉页眉（含已登录头像）后得 06、07；GitHub 相对时间 “18 hours ago”保留在图中 |
| 23:14 | Anthropic eth/README.md | opencli 全页截图（1400×4715），裁出 08（含面包屑与正文，去掉提交行）、09（止于 axioms 代码块之前，避免被页面横向截断） |
| 23:16 | agmai.org | opencli 全页截图（1400×12814），裁出 11（About 全节）、12（10-26 声明全文 + 10-25 声明标题日期与“we recommend”段） |
| 23:3x | ias.edu/nccr/projects | opencli 打开并用 `eval` 读取页面文本（未截图） |

`a-shots-log.jsonl` 里有 6 条指向已弃用的试验文件（`test-probe.png`、`t1.png`、`t2-probe.png`、`t3-probe.png`、`t4-probe.png`，以及被重做的 `05-oa-eth-abstract.png`、`06-oa-eth-thm11.png`、`07-oa-lean378-scope.png`、`09`/`10` 旧版），这些图已移到 scratchpad 或被替换，不在 `images/` 里；日志不删，请 Claude 合并时忽略这几行。

截图与存档均不含抓取者信息：截图的 GitHub 页眉（头像、通知）已裁掉；`fx` JSON 里只有公开的研究者账号；对 `sources/a-*` grep 抓取者用户名、本机路径、凭据字样均无命中。
