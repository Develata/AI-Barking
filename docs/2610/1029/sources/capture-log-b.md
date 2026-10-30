# 1029 期 B 组抓取日志

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；本机美东（EDT），北京 = 本机 + 12 小时。所有抓取未登录、未发帖；curl 与 barking 不带 Cookie；opencli 浏览器只用于打开公开页面（会话名 `bshot`、`rotw`、`ith`，已 close）。浏览器装有翻译类扩展，截图前已用样式隐藏其悬浮按钮；HTML 原件里含该扩展注入的两处标记（`b-report-rendered.html` 等），不影响正文。机器记录：`b-fetch-log.jsonl`（含 time_bj、sha256）、`b-shots-log.jsonl`、`b-heat.json`。

## 一、barking fetch / curl

| 时间 | 名称 | URL | 工具 | 结果 |
|---|---|---|---|---|
| 23:05 | report（空壳） | https://clear-languages.org/blog/the-tokenizer-tax | opencli 打开 | 成功渲染；`b-clear-blog.html` 等 curl 版只有约 2 KB 空壳 |
| 23:09 | github-repo / github-api / github-readme | https://github.com/clear-coalition-ai/token-cost-equity 及 API、README | barking fetch | ok |
| 23:09 | rotw | https://restofworld.org/2026/ai-tokenizer-tax-non-english-languages-cost-inequity/ | barking fetch | ok（`b-rotw.html/.txt`，页面日期 26 October 2026） |
| 23:09 | clear-home / clear-problem / clear-coalition / clear-blog | https://clear-languages.org/ 、/problem 、/coalition 、/blog | barking fetch | blocked（正文过短，JS 渲染）；改用 opencli，见第二节 |
| 23:18 | ithome / ifeng / netease163 / magica | IT之家、凤凰网、网易、Magica（见 evidence） | barking fetch | ok |
| 23:18 | letsdatascience | https://letsdatascience.com/news/study-finds-ai-tokenizers-charge-a-hidden-tax-on-non-english-a3c3e43e | barking fetch | blocked（Cloudflare 质询，403），不重试，未使用 |
| 23:19 | tii-falcon3 / lilt-clear / infobae | TII 发布稿（2024-12-17）、LILT 博文（2026-02-10）、Infobae（2026-10-26） | barking fetch | ok |
| 23:25 | computerworld | https://www.computerworld.com/article/4241844/ai-models-are-much-more-expensive-for-less-common-languages.html | barking fetch | ok（Oct 28, 2026） |
| 23:2x | gh 源码 | https://raw.githubusercontent.com/clear-coalition-ai/token-cost-equity/main/src/evaluate_equity.py 、00_data_raw/model_lists/hugging_face_models.csv 、scripts/03_analysis/token_count_equity_metrics.py（前 80 行）、00_data_raw/README.md | curl（--max-time 60） | 200；存 `b-gh-evaluate_equity.py`、`b-gh-hf-models.csv`、`b-gh-equity-metrics-head.py`；未运行任何代码 |
| 23:1x | HN 条目 50250910 | https://hn.algolia.com/api/v1/items/50250910 | curl | 200，存 `b-hn-50250910.json`（不含抓取者信息） |

## 二、opencli 渲染（公开页面，未登录）

| 时间 | URL | 产物 | 说明 |
|---|---|---|---|
| 23:05 | https://clear-languages.org/blog/the-tokenizer-tax | `b-report-rendered.html`、`b-report-rendered.txt`（`document.body.innerText`） | 全文含表 1、表 2、3 张图（图片托管在 firebasestorage，图中文字只在截图里） |
| 23:07 | https://clear-languages.org/coalition 、/problem 、/ 、/blog | `b-clear-*-rendered.html/.txt`、`b-clear-coalition-domtext.txt`（从 DOM 取出隐藏的 bio） | “Show bio” 折叠内容在 DOM 里已有，直接读取，没有点击 |

## 三、热度

| 时间 | 命令 | 结果 |
|---|---|---|
| 23:17 | `barking heat docs/2610/1029 b --hn "tokenizer tax" --hn "clear-languages.org" --hn "restofworld tokenizer" --reddit ×3` | HN：50250910（25 分 / 1 评）；Reddit：3 次请求都被重定向到登录页，失败（`b-heat.json` errors），不重试 |
| 23:1x | HN Algolia 按 URL 搜 clear-languages.org、按标题搜 | 无命中 / 仅 50250910 |

## 四、截图（opencli，全部为 CSS zoom 方式）

方法：`b-shot.sh`（本目录）。`opencli browser <会话> open` → 注入样式隐藏滚动条、悬浮层与导航 → `document.documentElement.style.zoom = innerWidth/700`（窗口宽 1264 px，即 700 CSS px 版式放大约 1.806 倍，截图宽 1264 px，不是 1400 px / 2 倍像素比；字号相当于 2 倍屏下约 632 CSS px 版式）→ `scrollTo` 起点后 `screenshot --height H`。踩坑：加 `--width 1400` 会使排版宽度变化、坐标错位，且报告表 1 会在窄版式下被横向裁掉“CLEAR Grade”列，所以不用；高度覆盖后视口有时不复位，靠页尾加 padding 避免滚动被钳位；曾有一次截图内容重复（视口状态不一致），已重拍。所有截图已逐张打开目视确认文字清楚。

| 文件 | 范围 | 备注 |
|---|---|---|
| `images/13-report-title-intro.png` | 标题、副标题、作者、日期、“From text to tokens” | 页面浅色底；导航栏已隐藏；无登录信息 |
| `images/14-method-verbosity.png` | “Measuring cost inequity at scale” 与 “But some languages are naturally more verbose” 两节 | FLORES-200、max/min 定义、Shan 42%、阿姆哈拉语 ~2x 到 ~8x |
| `images/15-beyond-20-grading.png` | “Beyond 20 languages”（含 1.5–3.1x、A–F 门槛、Gini/Kimi K3 句） | |
| `images/16-table1-full.png` | 表 1 全表：引导句、表头、19 行、表注 | 整表一张，宽 1264、高 3762 |
| `images/17-labs-multilingual.png` | “Some labs are intentionally multilingual” 全节 | Llama 3→4、NLLB、Gemini/Gemma |
| `images/18-qwen36-fig2.png` | Qwen 3 / Kimi 退化语言、Qwen 3.6 退步句、图 2 与图注 | 起点在“Degradation…”节中段（该节标题与首段未入图） |
| `images/19-alt-measure.png` | “A different take on equity” 引导段 | |
| `images/20-table2-full.png` | 表 2 引导段、全表 19 行、表注（含“English or Mandarin Chinese (Simplified)”） | 整表一张，宽 1264、高 4970，字号偏小，放大后可读 |
| `images/21-script-fig3.png` | “The problem is largely driven by the script” 节：图 3、图注与后两段 | |
| `images/22-pricing-call.png` | “Why this matters for AI adoption” 与 “A call to action for labs”（含 Fix pricing now） | |
| `images/23-restofworld-article.png` | Rest of World 文章头图、标题、日期、第 1–2 段与“1.5 and 5 times … 17 times”段 | 由 `barking shot cropbox` 从 1264×3410 裁到 1264×3345（裁边落在行间）；页面自带的“翻译成中文”下拉与浏览器悬浮按钮已用样式隐藏；无登录信息 |
| `images/24-ithome-reprint.png` | IT之家 转载稿：面包屑、标题、时间、前几段与“1.5 至 5 倍”“17 倍”段 | 右侧浮动排行榜已隐藏；“评论：30”为页面公开计数；无登录信息 |

## 五、未做 / 失败

- 中文转载：只存 IT之家、凤凰网、网易 3 篇；同花顺、驱动中国未存。
- Reddit 搜索接口全部被重定向到登录页（不登录、不绕过）。
- letsdatascience.com 被 Cloudflare 拦截，未用。
- 报告图 1 的原图（firebasestorage）未下载，截图 `13`/`14` 范围不含图 1（RoW 文章截图 `23` 含其复本，RoW 页面给出同一例句，标注为 OpenAI tokenizer）；需要图 1 可补一张。
- 未核对 Anthropic 官方 Fable 5 价格（超出 B 组范围）。

## 六、假设

- IT之家、凤凰网、网易页面上的时间按北京时间处理（页面未写时区）。
- 表 1 的“前 20 大语言”例外是 Falcon 3，来自读表（报告正文没点名）。
- 报告页面所见即发布版本；未用 web.archive.org 查历史修订，也没有发现更新记录（页面无“updated”字样）。

## 七、待删除文件

无（`docs/2610/1029/sources/b-shot.sh` 是本组脚本，保留；临时截图在系统临时目录 `%TEMP%\bshots\` 与误生成的 `%TEMP%\bshots$N.png`、`%TEMP%\bshots${N}.png`，不在仓库内，可由 Claude 清理）。
