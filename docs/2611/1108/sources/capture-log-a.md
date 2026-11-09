# 1108 期 A 组抓取日志

执行：Sonnet 5.5。时间为北京时间（UTC+8，本机时区即北京）。截图与存档文件名前缀 `a-`，图片编号 01–12。

## 抓取

| 时刻（北京） | URL | 工具 | 结果 |
|---|---|---|---|
| 11-09 18:05 | maxxbench.unlap.ai/、/research/why-maxxbench/、/changelog；deepswe.datacurve.ai/、/changelog；news.ycombinator.com/item?id=50406222 | `barking fetch`（清单 `a-list.tsv`） | 全部 ok：`a-mb-home`、`a-mb-why`、`a-mb-changelog`、`a-ds-home`、`a-ds-changelog`、`a-hn-item` |
| 11-09 18:06 | Anthropic Opus 5.5 与 Sonnet 5.5 系统卡 PDF；DeepSeek-V4.1-Flash、Qwen3.8-Flash-Next 的 HF 页；OpenAI GPT-6 Sol/Luna 发布页；MaxxBench /research/coding-sandboxes、/data、/research、GitHub 仓库与组织页；DeepSWE /data/v1.1、/run、/blog/deepswe、GitHub；unlap.ai | `barking fetch`（`a-list2.tsv`） | 14 条 ok，`a-src-opus55-syscard.pdf`（17.8 MB）与 `a-src-sonnet55-syscard.pdf`（13.1 MB）只存本地；unlap.ai failed（TLS 协议错误，curl `SSL_ERROR_SYSCALL`、opencli 浏览器 `ERR_SSL_PROTOCOL_ERROR`，不再重试） |
| 11-09 18:09 | api.github.com：unlap-labs/maxxbench 与 datacurve-ai/deep-swe 的 issues（含评论） | `gh api`（只读，已登录会话） | 200；`a-gh-issues-meta.json` 只存元数据，未存正文；issue 正文原文仅在草稿目录读过，核心数字摘入 `evidence-a.md`（#1/#2/#3） |
| 11-09 18:09 | https://deepswe.datacurve.ai/artifacts/v1.1/trials.json | curl（`--max-time 120`） | 200，51 MB，只存本地草稿目录；摘要 `a-deepswe-trials-summary.txt` |
| 11-09 18:09 | https://hn.algolia.com/api/v1/items/50406222 | curl | 200；供读评论；完整串由 `barking heat --item` 存为 `a-hn-50406222.json` |
| 11-09 18:11 | HN 条目 50406222 | `barking heat` | `a-heat.json`、`a-hn-50406222.json`：396 分、105 条评论 |
| 11-09 18:12 | deepswe.datacurve.ai/ 模型下拉 | opencli browser（后台窗口，JS 取 DOM） | 28 个模型名；无 Opus 5.5 等 5 个 |
| 11-09 18:14–18:16 | MaxxBench 首页 “i” 弹窗 | opencli browser（JS 点击读取） | 5 个弹窗原文，见 `evidence-a.md` |
| 11-09 18:20 | maxxbench.unlap.ai/data/runs/<9 个 id>（`a-run-*.html/.txt`） | `barking fetch`（`a-list3.tsv`） | ok，但页面只是外壳（“Loading run artifacts…”），轨迹要从 `/artifacts/runs/<id>.json.gz` 取 |
| 11-09 18:20 | maxxbench.unlap.ai/artifacts/runs/run-<id>.json.gz：9 个指定轨迹；V4.1 Flash(max) 的 178 个 | curl（`--max-time 90`，xargs 6 路） | 全部 200；本地草稿 `/tmp/a-art/`（68 MB，不入库）；核对结果 `a-quote-check.txt`、`a-v41-outside-llm-recount.txt` |
| 11-09 18:21 | WebSearch：MaxxBench / Unlap / 中文关键词 | 搜索引擎 | 仅作线索；返回 Runtimewire、HyperAI、daily.dev、Stork.AI、X 帖、unlap-website.pages.dev |
| 11-09 18:23 | https://unlap.ai/ | curl `--http1.1`、`http://` | 失败：TLS 握手错误；不再重试 |
| 11-09 18:24 | unlap-website.pages.dev/about、/；hyper.ai、daily.dev、stork.ai、runtimewire.com 的相关稿；api.fxtwitter.com/status/2118880329490637049；benchlm.ai/benchmarks/maxxbench | `barking fetch`（`a-list4.tsv`）；/ 另用 curl | /about 404（failed，存为 `a-unlap-about.html`，是 404 页）；hyper.ai、daily.dev、stork.ai、runtimewire.com ok；X JSON ok（`a-x-justinlo.json`，公开帖，只取首帖，无抓取者信息）；benchlm.ai 超时 failed（不再重试）；unlap-website.pages.dev/ 200，`a-unlap-pages-root.html/.txt` |
| 11-09 18:26 | reddit.com/search.json?q=MaxxBench | curl | 403，未取到 |
| 11-09 18:26 | hn.algolia.com/api/v1/search?query=MaxxBench | curl | 200：只有一个相关主帖 50406222 |
| 11-09 18:28 | huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017 | curl（只看状态码） | 200，未取内容 |

自行脚本（放在 `docs/2611/1108/sources/`）：`a-oc-pos.js` 与 `a-oc-shot.sh`（opencli 按文本锚点定位并截图）；`a-list*.tsv` 为 fetch 清单。统计复算由 `python3 -I` 一次性脚本完成，输出存为 `a-same89-recompute.txt`、`a-v41-outside-llm-recount.txt`、`a-quote-check.txt`，脚本本身未保存（逻辑在文件首行注释与 `evidence-a.md` 说明里）。

## 截图（opencli browser，后台窗口；页面深色底除注明外；2 倍像素版式即 CSS zoom 2、视口 1400 px 宽；表格类 zoom 1.5；用 `barking shot cropbox|stack` 裁出，日志 `a-shots-log.jsonl`）

两处工具问题：1）MaxxBench 博文窄版式（700 CSS px）下 “Hidden-test” 表最右列被裁，所以 05 用 1.5 倍版式（933 CSS px），保证所有列可见；2）网页翻译插件浮层（`#immersive-translate-popup`）在截图前用 JS 从 DOM 移除，网站固定头 `.site-header` 也隐藏，没有改任何页面内容。11 与 12 为单次视口截图，含页面滚动条或 GitHub 页面，已在裁切时去掉右侧滚动条与侧栏。

| 文件 | 来源页 | 内容 | 版式/备注 |
|---|---|---|---|
| 01-baseline-vs-maxxbench-chart.png | maxxbench.unlap.ai/research/why-maxxbench/ | “Published DeepSWE baseline vs. MaxxBench score” 图 + 展开的 Source data 表（15 行，带 “i”）+ 20.9→48.3、23.0 结论段 | zoom 2，深色；Source data 展开 |
| 02-leaderboard-home.png | maxxbench.unlap.ai/ | 榜单：15 行，MaxxBench Score、Est. Benchmaxxing、Cost/Tokens/Steps，5 行带 “i” | zoom 2，深色 |
| 03-mutation-lowercase-median.png | 博文 | 方法段：小写文件名与 FastAPI 例、Example mutations 交互示例（Chrome (Headless)）、17 行（3.5%）段 | zoom 2 |
| 04-guess-grader-quotes.png | 博文 | “Most models tried pleasing an imagined grader over the prompt” 整节：GLM-5.3、OPA、skrub、LangChain 0.55/0.80、约四分之一、16.7%/30.8%/40.3% | zoom 2；段内蓝色下划线是轨迹链接 |
| 05-hidden-test-table.png | 博文 | Hidden-test 全表（14 行，全部 8 列，含 †脚注）+ “Share of failed attempts by behavioral cause” 条形图 | zoom 1.5（933 CSS px，约 1.5 倍像素），深色 |
| 06-scc-self-report.png | 博文 | “Self-reported peak vs. live memory retention on scc” 13 行表（Reported peak、Live records、Recorded gaming） | zoom 1.5 |
| 07-deepseek-outside-llm.png | 博文 | “DeepSeek bypassed sandbox network restrictions to call OpenRouter” 整节（20 of 178、high 档为 0、Testem 33 项失败） | zoom 1.5；**取代 `07-deepseek-20-of-178.png`（最后一行文字底边被裁）** |
| 08-testem-grading.png | 博文 | Testem 段 + 折叠项 + Grading 展开（35/68、469/469、AssertionError） | zoom 2；Step 58 与 107–108 的代码因手风琴只展开一项，未在图里，代码在 `a-run-v41-testem` 轨迹与博文文本存档 |
| 09-deepswe-official-board.png | deepswe.datacurve.ai/ | 官方榜（浅色底）：“113 tasks · updated September 22, 2026 · changelog”、Configs (57/70)、21 行模型榜、“All models run on mini-swe-agent” | zoom 1.5，浅色 |
| 10-fix-65-8-to-64-5.png | 博文 | “65.8% to 64.5%” 段 + “Attempts failing only repaired checks” 条形图（13 行） | zoom 2 |
| 11-first-party-baseline-note.png | maxxbench.unlap.ai/ | 榜单上 Claude Opus 5.5 旁 “i” 弹窗：“DeepSWE 74.2% · Reported by Anthropic / Anthropic system card”；同图含 Est. Benchmaxxing 列头与前 6 行 | zoom 2；弹窗是点击后状态 |
| 12-gh-issue-paired-drops.png | github.com/unlap-labs/maxxbench/issues/1 | issue #1 标题、状态与首条前三段：方法、同 89 题对比表、结论 1–3 | zoom 1.5，浅色；含提交者公开 GitHub 用户名与头像（公开 issue 元数据）；右侧栏已裁掉；标题条与正文分两段用 `barking shot stack` 拼接；仅作 L6 线索 |

## 废弃与待删除文件

见本组回报的“待删除文件”清单。
