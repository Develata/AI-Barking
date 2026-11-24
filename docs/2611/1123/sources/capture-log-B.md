# 1123 期 B 组抓取记录

执行方 Claude Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8，本机时区），取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl` 的 `time_bj` 与命令时刻；任务约 22:57 开始，约 23:10 因催交停止。命令前均 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"`（经临时包装脚本，不在仓库）。未登录任何站点、未输入凭据、未接受 Cookie、未下载模型权重（GGUF）、未使用镜像或代理站。opencli doctor：扩展已连接（22:5x）。清单文件：`b-fetch-list-1.tsv` … `b-fetch-list-6.tsv`。

## 一、barking fetch（全部 ok，无 blocked/failed，除注明）

| 北京时间 | 清单 | name 与 URL | 结果 |
|---|---|---|---|
| 22:57:38 | list-1 | hf-card（huggingface.co/nari-labs/Qwen3.8-27B-NariQ-GGUF）、hf-readme-raw（…/raw/main/README.md）、hf-api（/api/models/…）、hf-commits-api、hf-tree-api、hf-commits-page、fork-repo（github.com/nari-labs/nari-llama.cpp）、fork-readme-raw、arxiv-abs（arxiv.org/abs/2611.26157）、arxiv-pdf、arxiv-api（export.arxiv.org） | 11 条全 ok；arxiv-pdf 7 页，pdftotext 正常 |
| 22:58:26 | list-2 | fork-readme-8e0e016e、fork-qual、fork-metal、fork-windows、fork-notice、fork-license、fork-nariq-readme（raw.githubusercontent.com/nari-labs/nari-llama.cpp/8e0e016e/…）、hf-license、hf-notice、hf-sha256sums、fork-release（releases/tag/b12431-nariv0）、fork-issues、fork-pulls、hf-community（/discussions）、hf-base-card（Qwen/Qwen3.8-27B） | 15 条全 ok |
| 23:01:58 | list-3 | juejin-1124（juejin.cn/post/7711225699185410102） | ok；页面 datePublished 2026-11-24T00:31:20Z（北京 08:31:20） |
| 23:02:14 | list-4 | zh-mobdome、zh-zmj-news、zh-zmj-article、zh-openai-hub、zh-cnaihot、zh-aidailyradar、bittide；zh-zeli（zeli.app）、en-imasters（imasters.com.br） | 7 条 ok；zh-zeli 403 blocked，en-imasters 429 blocked（重试一次仍失败，不再试，未改用别的来源）；bittide 页中未含 NariQ |
| 23:02:56 | list-5 | en-technewsreel、en-nextfor、hn-page（news.ycombinator.com/item?id=50611402）、fork-pr3（/pull/3）、fork-readme-page | 5 条全 ok |
| 23:09:35 | list-6 | hf-readme-rev1/rev2/revprev（HF raw 三个旧 commit 的 README）、unsloth-q38（unsloth.ai/docs/models/qwen3.8）、unsloth-dyn31、fork-hip-readme（damarges/nari-llama-amd） | 6 条全 ok；unsloth 两页只存档未读 |

## 二、其他请求

| 北京时间 | 工具 | 目标 | 结果 |
|---|---|---|---|
| 22:5x | curl --max-time 60 | huggingface.co/…/resolve/main/images/{agent-kl,deepswe,terminal-bench}.png | 首次 agent-kl 未返回，其余 302；加 -L 重试 3 张全部 200（263–268 KB），存 `b-hf-img-*.png`（图片，不入库） |
| 22:5x | gh api（GitHub 公共 API，本机 gh 默认认证，仅读） | repos/nari-labs/nari-llama.cpp（元数据、commits、releases、issues、pulls）、compare ggml-org:master…nari-labs:nariq/b12431、search/issues repo:ggml-org/llama.cpp NariQ / nari-labs / trellis nari | 成功；上游命中 0；compare：领先 18、落后 275；存 `b-gh-release.json`、`b-gh-fork-issues.json` |
| 22:59:42 | barking heat | `--item 50611402 --hn NariQ --reddit NariQ --reddit "Qwen3.8-27B NariQ"` | HN 成功（181 分 / 35 评），存 `b-hn-50611402.json`、`b-heat.json`；Reddit 2 次请求被重定向到 old.reddit.com/login（响应非 JSON），已记入 errors |
| 23:0x | opencli reddit search / subreddit LocalLLaMA | NariQ、r/LocalLLaMA/new | 2 次均 SECURITY_BLOCK（HTTP 403，Reddit network security）；不重试、不绕过 |
| 23:0x | WebSearch ×2、exa web_search_exa ×5 | 找中文同题稿与 Reddit 线索 | 只用于找 URL，内容均另行抓取原页；未找到网易号同题稿；Reddit 原帖 URL 未得（bittide 聚合页称 r/LocalLLaMA 有帖，未取到原帖） |
| 23:0x | curl POST api.juejin.cn/…/list_by_user | 衍辉channel 文章列表 | 返回 “must login”，改用 opencli 浏览器匿名打开用户文章页，从中读得 11.24 篇 URL；`b-juejin-user-list.json` 为失败响应（待删） |
| 23:0x | opencli browser（session b-shot，window background） | juejin.cn/user/1151943917965207/posts | 读取页面文本与链接（页面为未登录态，顶栏显示“登录/注册”）；未截图 |

## 三、barking shot（headless Chrome，`render` 700 CSS px、DPR 2；详见 `b-shots-log.jsonl`）

中间图在会话临时目录的 barking-shot 子目录，不在仓库。渲染：hfcard、hfcardl（试 `--blink-settings=preferredColorScheme=0`，页面仍为深色，弃用）、fork、juejin（首次高度 5500 不够，改 6500 重渲为 juejin2）、mob、hn。

最终图（`docs/2611/1123/images/`，均宽 1400 px、2 倍像素；已逐张打开目视确认）：

| 文件 | 来源页面 | 裁切 | 备注 |
|---|---|---|---|
| 13-hf-card-head.png | 模型卡 | 标题、fork required、match BF16 要点 | 深色底 |
| 14-hf-score-table.png | 模型卡 | 分数表全表头、全行与脚注 | 深色底 |
| 15-hf-limits.png | 模型卡 | Known limits and future work 全节 | 深色底 |
| 16-hf-memory-table.png | 模型卡 | 采样参数一段、显存表、README 链接句 | 深色底，起首为半段正文（从“The sampling flags …”整句起） |
| 17-hf-speed-table.png | 模型卡 | Decode speed 说明、吞吐表、参数脚注 | 深色底 |
| 18-fork-hardware.png | fork README（GitHub 页，匿名） | Hardware 节全表 + 其后一段 | 深色底（GitHub 暗色） |
| 19-hf-deepswe-chart.png | 模型卡 | DeepSWE 图（白底图表整图）+ 图注 | 官方图表整张，含图例与标题；外围深色 |
| 20-juejin-nariq-item.png | 掘金 11.24 篇 | 第 6 条整条（标题、配图、三段正文、编者按、来源行） | 浅色底；浏览器匿名，顶栏已不在裁切内 |
| 21-hn-comments.png | HN 50611402 | 3 段评论拼接（WithinReason、hugmynutus 与作者回复、作者 Mac 回复） | 浅色底；拼接缝为 HN 橙色（工具自动取色）；评论间用拼接，非连续原页；账号栏不在裁切内 |
| 22-hf-terminal-bench-chart.png | 模型卡 | Terminal-Bench 图 + 图注 | 深色底外围 |
| 23-mobdome-claims.png | mobdome 文 | 两段拼接（runtime 与测试机环境；速度与品质） | 浅色底；拼接缝为绿色（工具自动取色）；原文繁体 |
| 24-fork-head.png | fork README | 仓库名与首段（Stock llama.cpp, Ollama and LM Studio cannot load these files yet） | 深色底 |

已弃用的试验图已移出仓库（移到会话临时目录，不是删除仓库文件）：18-fork-hardware 初版（底部裁到半行）、24-fork-head-perf（表格被页面宽度截断右两列）、21-hn-comments 初版（reply 行被切）、预览图 prev*。仓库内无试验残留；`b-shots-log.jsonl` 里仍有这些动作的记录行，对应文件已不在 images/。

## 四、待删除文件清单（供 Claude 收尾时一次 rm）

- `docs/2611/1123/sources/b-juejin-user-list.json`（失败响应 “must login”）

（其余 `b-fetch-list-*.tsv` 等是本组产物，保留。）

## 五、交付前泄露自查

- 对 `docs/2611/1123/sources/b-*` 与 `images/` grep “develata”“gitclone”：无命中。
- 抓取均匿名（headless Chrome / curl / barking fetch / 匿名 opencli 会话），截图无登录栏、头像、昵称；HN 评论图只含评论者公开 HN 用户名；`b-hf-commits-api.json` 含 HF 提交者的公开用户名与头像链接（模型作者，非抓取者）；`b-gh-fork-issues.json`、`b-fork-pr3.*` 含外部 PR 作者的公开 GitHub 用户名。
- `b-hf-img-*.png`、`b-*.pdf`、`b-*.html` 为原件，不入库；单个文本文件超 1 MB 的：`b-unsloth-q38.html`（1.9 MB）、`b-unsloth-dyn31.html`（1.0 MB）、`b-hf-base-card.html`（0.95 MB）均为 html（不入库）；文本 txt 最大约几十 KB。
