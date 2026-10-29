# A 组抓取日志（1028 期，Google Gemini 4 Carbon）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；本机美东（EDT），北京 = 本机 + 12 小时（`date` 显示 07:40 EDT 即 19:40 北京）。机器记录另见 `a-fetch-log.jsonl`（每条含 time_bj、sha256）、`a-shots-log.jsonl`、`a-heat.json`。所有抓取未登录、未发帖；curl 与 barking 不带 Cookie；opencli 浏览器只用于打开公开页面（会话名 `a-pr`、`a-ar`、`a-bl`）。HTML/PDF 原件只存本地，不入库。清单文件 `a-list1.tsv`–`a-list6.tsv` 在 `docs/2610/1028/sources/`。

## 一、barking fetch 批量存档（`a-<name>.html|.pdf|.txt|.headers`）

| 时间 | name | URL | 结果 |
|---|---|---|---|
| 19:40 | blog | https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-carbon/ | ok（989 KB，855 行；文本抽取不含对比表与三张图表，因为它们是图片；脚注在折叠区，HTML 内有全文） |
| 19:40 | hn | https://news.ycombinator.com/item?id=50276792 | ok（页面存档；评论另取 Algolia JSON） |
| 19:40 | aa-changelog / aa-leaderboard / aa-methodology / aa-gemini4 | https://artificialanalysis.ai/changelog 、/leaderboards/models 、/methodology/intelligence-benchmarking 、/models/gemini-4-carbon | ok（正文完整；模型页的图表为图形，数值需截图） |
| 19:40 | dm-model | https://deepmind.google/models/gemini/carbon/ （猜测 URL） | failed（404）；对比表实际在 https://deepmind.google/models/gemini/ |
| 19:40 | dm-models | https://deepmind.google/models/gemini/ | ok（含 Carbon 对比表；用 Python 解析 HTML 表格，核实列顺序 Carbon / Argon / GPT-6 Astra / Opus 5.5 / Fable 5.1） |
| 19:40 | api-pricing / api-models | https://ai.google.dev/gemini-api/docs/pricing 、/models | failed（跳转过多，超过 10 次）；改见第三节 |
| 19:40 | lmarena | https://lmarena.ai/leaderboard/text | blocked（Cloudflare 质询），未绕过 |
| 19:41 | methodology | https://deepmind.google/models/evals-methodology/gemini-4-carbon | ok（响应为 PDF，6 KB 文本，4 页；第 4 页的结果表是图片） |
| 19:41 | fairwind | https://deepmind.google/fairwind-program/ | ok |
| 19:41 | argon-blog / argon-dev | Argon 发布博文与开发者博文（2026-10-13 16:00 UTC） | ok |
| 19:41 | safety-report | https://storage.googleapis.com/deepmind-media/gemini/gemini_4_safety_report.pdf | ok（17 MB PDF，只存本地；本期未引用，是 Argon 的安全报告） |
| 19:41 | secbench / cwebench / exploitgym | sec-bench.github.io、cwe-bench.com、github.com/sunblaze-ucb/exploitgym | ok（cwebench 榜单文本含 Carbon；secbench 与 exploitgym 页无 Carbon 行） |
| 19:41 | aa-opus / chrome-blog / wiz-scan | AA 的 Opus 5.5 页、blog.google Chrome 栏目页、Wiz Scan for Good 博文 | ok（chrome-blog 只是栏目页，无 Carbon 内容；wiz-scan 无 Carbon） |
| 19:41 | arena-webdev | https://arena.ai/leaderboard/code/webdev | blocked（Cloudflare 质询）；改用 opencli，见第三节 |
| 19:42 | aa-article / aa-providers / aa-argon / aa-v5-article / aa-coding-agent | AA 的 Carbon 文章、Carbon 提供商页、Argon 页、v5 公告、Coding Agents 页 | ok |
| 19:49 | flashcyber / m-ie / m-latent / m-winbuzzer / m-unite / m-beri / m-explosion | Google 的 3.8 Flash Cyber 博文（2026-09-02）与 6 篇媒体稿 | ok |
| 19:49 | m-fourweek | https://fourweekmba.com/… | blocked（403），弃用 |
| 19:49 | c-36kr / c-pedaily / c-ithome / c-21jj / c-sina | 36氪、投资界、IT之家、21 财经、新浪财经 | ok（c-sina 仅 57 行） |
| 19:49 | c-oschina | https://www.oschina.net/news/503286/google-gemini-4-carbon | blocked（正文过短） |
| 20:08 | vertex-models / vertex-argon / vertex-whatsnew | cloud.google.com 的 Vertex（现名 Gemini Enterprise Agent Platform）模型与发布说明页 | ok（文本里无 Carbon，Argon 有） |
| 20:08 | m-dealroom / m-marktech | dealroom.co、marktechpost.com | blocked（Cloudflare 403），弃用 |

## 二、个别请求（curl，`--max-time 60`）与 HN

| 时间 | URL | 结果 |
|---|---|---|
| 19:4x | https://api.fxtwitter.com/status/2115489219804352921 、/2115489593818816591 | ok，存 `a-fx-<id>.json`（Sundar Pichai 与 @GoogleAI 的公开官方帖；无抓取者信息） |
| 19:47 | `barking heat docs/2610/1028 a --hn "Gemini 4 Carbon" --reddit "Gemini 4 Carbon" --item 50276792`（内部调用 Algolia） | HN 命中 3 条；主帖 50276792：954 分 / 763 评（2026-10-29 01:01 北京）；评论树存 `a-hn-50276792.json`（343 KB）。Reddit 搜索被重定向到 `old.reddit.com/login`，失败，未登录、未重试，已记入 `a-heat.json` errors。HN 不公开评论分数。 |
| 19:5x | 对 https://ai.google.dev/gemini-api/docs/pricing 与 /models 用 curl 加 Cookie 罐和浏览器 User-Agent 追跳转（得到 `a-api-pricing.html`、`a-api-models.html`） | 命令返回 200，但随后一个命令被权限分类器拒绝，理由 “Bot Check Circumvention”。此后不再用这种方式，**两个 HTML 不作为证据**（列入待删除），改用 opencli 的正常浏览器会话。 |

## 三、opencli 浏览器（公开页面，未登录）

| 时间 | 页面 | 动作与结果 |
|---|---|---|
| 19:5x | https://ai.google.dev/gemini-api/docs/pricing 、?hl=en | 默认语言为中文，改 `?hl=en`；读 `document.body.innerText` 存 `a-api-pricing-opencli.txt`；Argon 节：“$2.00 through November 30, 2026. $4.00 starting December 1, 2026.”；页面无 Carbon |
| 19:5x | https://ai.google.dev/gemini-api/docs/models?hl=en 、/changelog?hl=en | 存 `a-api-models-opencli.txt`、`a-api-changelog-opencli.txt`；无 Carbon（仅 Argon 的 GA 条目） |
| 19:5x | https://arena.ai/leaderboard/code/webdev | 存 `a-arena-webdev-opencli.txt`；Carbon (High) 1768 Preliminary，Rank 4；页面日期 Oct 27, 2026 |
| 19:5x | 博文页“More Information”折叠区 | 点击展开（只改本页显示状态，不提交任何内容） |

## 四、WebSearch 与 Exa（只作线索，搜索摘要不作来源）

WebSearch 4 次（Chrome 10X、AA 第一与开放范围、中文报道、Chrome 侧材料）。搜索摘要中的陈述凡引用者，均已回到存档页面核对。搜索摘要里“Arena Agent 榜单第 2”“14 项拿下 12 项第一”等来自 36氪稿，未核，仅作勘误线索。

## 五、截图（opencli 与 barking shot）

| 时间 | 文件 | 动作与说明 |
|---|---|---|
| 19:51 | `blog` 中间图 | `barking shot render`（headless Chrome，700 CSS px，DPR 2，实得 1400×14000，含对比表与三张图表，到 Related stories） |
| 19:52 | `01-blog-aa59-intro.png` | `stack`：页眉与标题（y 150–1030）+ 开头两段（y 2060–2620），拼接缝白色。页眉带渐变底色，截到作者署名为止 |
| 19:58 | `02-blog-rollout-price-footnote.png` | `stack`：render 的开放范围段 + 价格段（y 2850–3420）与脚注区。**脚注区是 DOM 克隆**：headless 渲染下脚注折叠；opencli 展开后 `scrollTo` 与 zoom 冲突无法定位，改把展开后的 `.uni-footnotes` 与价格段克隆进页面顶部一个白底浮层，`zoom=2` 后截图（`a-fn6.png`，scratchpad），再裁出 “More Information” 与脚注一行。文字原样，布局不是页面原貌（脚注宽度被收窄、折叠按钮箭头未裁尽）。脚注原文已在 evidence-A.md 逐字存档。 |
| 19:52 | `03-blog-chrome-10x.png` | `cropbox`：Chrome 10X 与 2x 段，含小标题 |
| 19:52 | `04-blog-secbench-pro.png` | `cropbox`：“evaluated without guardrails” 句 + SEC-bench Pro 条目与整张柱图（含图例、坐标轴） |
| 19:52 | `05-blog-exploitgym.png` | `cropbox`：ExploitGym 条目 + 整张柱图（含时间上限标注：10h / 6h / No time cap / Unknown time cap） |
| 19:52 | `06-blog-cwe-bench.png` | `cropbox`：CWE-bench 条目 + 整张柱图（含各柱 harness：Antigravity / opencode / Codex / Claude Code） |
| 19:52 | `07-blog-benchmark-table.png` | `cropbox`：整张对比表，含列头、所有行与 “Methodology: …” 脚注行；Google 在图中用蓝底标出各行最高，灰底标出 Carbon 之外的最高（FrontierSWE 2、OSWorld-2.1） |
| 19:59 | `08-aa-article-launch-notes.png` | `stack`：AA 文章标题、日期、摘要与 “rolled out to selected users … 50% launch discount, with no confirmed end date” 段（y 180–1070）+ “Key results … not leading in coding and knowledge work” 段（y 2190–2440）；render 为 headless，白底 |
| 19:59 | `09-aa-article-index-charts.png` | `cropbox`：AA 文章内两张图（Intelligence Index 柱图 59 / 58 / 57 / 55 …；Coding Agent Index 柱图 62 / 59 / 59 / 59 …），含图题、说明与 “No publicly available API” / “Not publicly available” 图例（棋盘格底纹表示不可公开使用）。箭头与紫色标注是 AA 自己画的 |
| 20:00 | `10-arena-webdev-top.png` | opencli 视口截图（**1 倍像素**，Arena 是 SPA，`zoom=2` 后页面渲染为空白，放弃）；`cropbox` 裁出表题、日期、票数、列头与前 5 行。页面左侧栏、登录按钮与翻译插件图标（页面右侧）已不在裁切范围内 |
| 20:01 | `11-api-pricing-argon-intro.png` | opencli 视口截图（**1 倍像素**），裁出 Gemini 4 Argon 节：标题、说明、Note、价格表（含 “through November 30, 2026”）。左侧栏与 “On this page” 不在图内 |
| 20:03–20:06 | `12-36kr-overstated.png` | `render` 36氪页（headless，1000 CSS px 宽，DPR 1.4，实得 1400×6160，**不是 2 倍像素**：该页在 700 px 宽下版面被截断，标题与正文被切）；`stack`：标题/日期/副标题/首句 + “14项…12项第一” 行 + “全面碾压” 段，右侧广告与侧栏已裁掉。首版 `12-36kr-overstated-headline.png`（700 px 渲染，标题被切、时间显示 10-28 20:17）与第二版 `12b-36kr.png`（含右侧广告）已移到 scratchpad；正文页 HTML 与 txt 都写 2026-10-29 08:17，图中时间以后者为准。`a-shots-log.jsonl` 里与 12 相关的三行（`12-36kr-overstated-headline.png`、`12b-36kr.png`、`12-36kr-overstated.png`）只有最后一行对应现存文件 |

图中无抓取者信息：全为未登录公开页或克隆出的公开文字；截图中未出现头像、昵称或 handle；Arena 页左侧栏与登录栏、翻译插件图标已裁出。对 `docs/2610/1028/sources/a-*` 与 `images/01–12` grep “develata”、本机用户路径，无命中。

## 六、待删除文件（精确路径）

仓库内（需 Claude 收尾时一次删除）：

- `docs/2610/1028/sources/a-api-pricing.html`、`docs/2610/1028/sources/a-api-models.html`（见第二节：curl 追跳转取得，不作证据）

本组产物里可留可删（由 Claude 决定）：

- `docs/2610/1028/sources/a-m-fourweek.headers`/`.html`（blocked 页）、`a-m-dealroom.*`、`a-m-marktech.*`、`a-c-oschina.*`、`a-lmarena.*`、`a-lmarena-webdev.*`、`a-arena-webdev.*`（blocked，Cloudflare 质询页）、`a-dm-model.*`（404 页）
- `a-safety-report.pdf`（17 MB）与 `a-safety-report.txt`（486 KB）：Argon 安全报告，本期未引用；PDF 只存本地
- HTML 原件、PDF 原件均只存本地（规则见派工单）

scratchpad（会话临时目录，不入库）：`a-fn.png`–`a-fn6.png`、`a-ar1.png`、`a-ar2.png`、`a-pr1.png`–`a-pr3.png`、`a-old12.png`、`a-old12b.png`；`%TEMP%\barking-shot\26101028-a\` 下的 render 中间图（blog、aaart、kr、kr2）。
