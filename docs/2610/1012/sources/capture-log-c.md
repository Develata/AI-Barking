# 1013 C 组抓取日志

执行方：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间 2026-10-13（UTC+8）。截图与渲染一律用匿名 headless Chrome（Playwright Core，全新临时 profile，不登录、不带 Cookie），DPR 2，视口 700 CSS px（成图宽 1400 px）。机器可读的逐次记录在 `c-browser-records.jsonl`。未创建账号、未发帖、未在 GitHub 留任何痕迹。

脚本（均在本目录）：`c-browser.mjs`（渲染 + 按文字锚点或页面 y 区间截图 + 存文本/渲染 HTML）、`c-vtext.mjs`（渲染后按文档顺序导出全部文本与链接，Vals 页 innerText 取不全时用）、`c-expand-shot.mjs`（点开 “VIEW N MORE MODELS” 后截图）、`c-heat.py`（HN 热度）、`c-hn-flatten.py`（改自 1012 C 组）、`c-shots-*.json`（截图区间参数）。

## 一、页面抓取

| 时刻 | URL | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 19:26 | https://z.ai/blog/glm-legal | curl | HTTP 200，仅 683 字节空壳（动态渲染） | `c-raw-z-ai-blog-glm-legal.html/.headers` |
| 19:27 | https://z.ai/blog/glm-legal | c-browser.mjs（宽 1100，等 9 s） | 成功，11459 字符全文 | `c-zai-blog-glm-legal.txt`、`.rendered.html` |
| 19:26 | https://www.vals.ai/models/zai_glm-legal | curl | HTTP 200，149 KB，服务端页面但成绩区需脚本渲染，不能直接读 | `c-raw-www-vals-ai-models-zai_glm-legal.html/.headers` |
| 19:29 | 同上 | c-browser.mjs | 成功，但 innerText 只取到侧栏（2567 字符），不完整 | `c-vals-model-zai-glm-legal.txt`、`.rendered.html` |
| 19:31 | 同上 | c-vtext.mjs | 成功，6103 字符，含成绩区、Updates、超参 | `c-vals-model-zai-glm-legal-full.txt`（正式文本存档以此为准） |
| 19:31 | https://www.vals.ai/benchmarks/hlab | c-vtext.mjs | 成功，79 行榜单 + 方法说明 | `c-vals-hlab.txt` |
| 19:32 | https://www.vals.ai/benchmarks/legal_bench | c-vtext.mjs | 成功，150 个模型榜单（文本中 166 行含重复）+ 数据集说明 | `c-vals-legalbench.txt` |
| 19:32 | https://www.vals.ai/benchmarks/legal_research | c-vtext.mjs | 成功，78 个模型榜单 + 方法/数据集 | `c-vals-legalresearch.txt` |
| 19:33 | https://www.harvey.ai/blog/introducing-harveys-legal-agent-benchmark | c-vtext.mjs | 成功 | `c-harvey-lab-intro.txt` |
| 19:33 | https://www.harvey.ai/blog/legal-agent-benchmark-initial-results | c-vtext.mjs | 成功 | `c-harvey-lab-initial-results.txt` |
| 19:34 | https://z.ai/consultation/legal | c-browser.mjs | 成功（申请表单文字；未填写、未提交） | `c-zai-consultation-legal.txt`、`.rendered.html` |
| 19:35 | https://docs.z.ai/guides/overview/pricing.md | curl | 成功，价格页（Last-Modified 11:35:46 GMT） | `c-zai-docs-pricing.md` |
| 19:35 | https://docs.z.ai/guides/vlm/glm-5.3-flash.md、https://docs.z.ai/llms.txt | curl | 成功；llms.txt 无 GLM-Legal | `c-zai-docs-glm-5.3-flash.md`、`c-zai-docs-llms.txt` |
| 19:36 | https://artificialanalysis.ai/evaluations/harvey-lab-aa | c-vtext.mjs | 成功，无 GLM-Legal | `c-aa-harvey-lab-aa.txt` |
| 19:36 | https://artificialanalysis.ai/models/glm-legal；/models/glm-5-3-legal；/leaderboards/models | curl | 前两个 404；榜单页 200（2.4 MB，未入库，grep 无 “GLM-Legal”） | 仅记录 |
| 19:35 | https://huggingface.co/api/models?search=GLM-Legal；…?author=zai-org&sort=lastModified&limit=15；…/api/models/zai-org/GLM-Legal | curl | 前两个成功（`[]` / zai-org 列表无 GLM-Legal）；第三个返回 `{"error":"Invalid username or password."}`；`huggingface.co/zai-org/GLM-Legal` 页 HTTP 401 | 结论写入 evidence，未另存 |
| 19:36 | https://modelscope.cn/api/v1/dolphin/models（PUT 搜索 GLM-Legal / GLM-5.3-Flash） | curl | 成功，仅取结论 | `c-modelscope-search.txt`（原始响应含第三方邮箱字段，未存档） |
| 19:36 | zai-org GitHub 组织仓库列表（`gh api orgs/zai-org/repos`，只读）、`github.com/zai-org/GLM-Legal` | gh、curl | 无 GLM-Legal；页面 404 | 仅记录 |
| 19:37–19:38 | https://www.oschina.net/news/502993 | curl、c-vtext.mjs（等 8 s、20 s）、firecrawl_scrape（1 次） | **失败**：只得到站点壳，正文是动态内容，未取到；只有 firecrawl_search 的摘要（标题与首句） | `c-m-oschina-502993.html/.txt`（空壳） |
| 19:37 | https://docs.bigmodel.cn/cn/update/new-releases；https://docs.bigmodel.cn/llms.txt | curl | 成功，无 GLM-Legal（文中 “legal” 命中均为脚本代码） | `c-bigmodel-new-releases.html`（939 KB，仅本地）、`c-bigmodel-llms.txt` |
| 19:57 | https://www.zhipuai.cn/zh、/zh/news；https://bigmodel.cn/ | curl、c-vtext.mjs | 成功，无 GLM-Legal（新闻列表最新为 2026/03/31） | `c-cn-*.html`（各约 0.9 MB，仅本地）、`c-cn-zhipuai-news.txt` |
| 19:39 | https://hn.algolia.com/api/v1/items/50058507 | curl | 成功（586 分、330 评） | `c-hn-50058507.json/.txt` |
| 19:55 | 同上 + https://hacker-news.firebaseio.com/v0/item/50058507.json；Algolia 搜索 `GLM-Legal`、`z.ai/blog/glm-legal` | python（c-heat.py）、curl | 成功（588~590 分、332 评）；Algolia 搜索仅命中该一帖 | `c-heat.json`、`c-hn-50058507.json` 刷新 |
| 19:3x | https://api.fxtwitter.com/status/2109828947869565336、…/2109842300520239305 | curl | 成功（公开 JSON，来自两个公开账号的帖子，与抓取者无关） | `c-fx-*.json` |
| 19:4x | https://github.com/harveyai/harvey-labs/issues/191（`gh api`，只读） | gh | 成功 | `c-gh-harvey-labs-issue191.json` |
| 19:4x | https://github.com/SKYLENAGE-AI/PLawBench（`gh api repos/skylenage/PLawbench`，只读，重定向到该仓库）；https://evalscope.readthedocs.io/en/latest/benchmarks/plawbench.html | gh、curl | 成功 | `c-gh-plawbench.txt`、`c-evalscope-plawbench.txt` |
| 19:5x | https://byteiota.com/glm-legal-z-ai-open-weight-legal-ai-beats-frontier-models/；https://www.163.com/dy/article/L94BFCFM0519D45U.html；https://toolnavs.com/article/2507-…；https://braindetox.kr/en/posts/glm_legal_domain_model_release_2026.html；https://zglg.work/en/ai/news/2026-10-13-zhipu-releases-glm-legal-… | curl（Chrome UA）+ b-html2txt 式转文本 | 成功（原 HTML 仅本地） | `c-m-byteiota.*`、`c-m-163-guandian.*`、`c-m-toolnavs.*`、`c-m-braindetox.*`、`c-m-zglg.*` |
| 19:5x | https://www.gate.com/news/detail/zhipu-ai-releases-glm-legal-with-192-task-pass-rate-cost-70-times-lower-24916438 | curl | **失败**：HTTP 403（Access denied），不绕过 | `c-m-gate.html/.txt`（拒绝页） |
| 19:56 | https://www.vals.ai/models/zai_glm-5.3-flash | c-vtext.mjs | 成功（对照基座页） | `c-vals-model-zai-glm-5-3-flash.txt` |

firecrawl 使用（额度有限，共 5 次 search + 1 次 scrape）：中文检索、英文检索、两次限定一天内的报道检索、一次 OSChina 抓取（失败，见上）。firecrawl 与 WebSearch 的返回只当线索：媒体链接与标题均已回到原页面 curl/渲染核对，OSChina 与 163 网易号的标题（搜索摘要）未能在原页面核对，已在 evidence 中标明。

## 二、截图

全部来自匿名 headless Chrome，页面均为浅色。入图前逐张打开目视确认（文字清楚、范围正确、裁切落在行间）。

| 文件 | 页面 | 北京时刻 | 区域/说明 | 脚本 |
|---|---|---|---|---|
| 25-zai-glm-legal-head.png | https://z.ai/blog/glm-legal | 19:43 | 日期、标题、前三段（含 19.2% 与 “matching frontier closed-source models” 句） | c-browser.mjs @c-shots-zai.json |
| 26-zai-benchmark-chart.png | 同上 | 19:43 | “Benchmark Highlights” 标题、官方图 “Legal Benchmark Evaluation”（标题、图例、三个基准全部柱）、三条要点、Vals 独立评测声明 | 同上 |
| 27-zai-beta-api-application.png | 同上 | 19:43 | “Join the GLM-Legal Beta”（申请 API 的段落） | c-browser.mjs @c-shots-zai27.json |
| 28-vals-model-page-scores.png | https://www.vals.ai/models/zai_glm-legal | 19:45 | 页头：描述、信息格（含 Weights OPEN、$0.07/0.25）、Accuracy/Cost/Latency、三项评测成绩（含 ± 与排名分母）与图例 | c-browser.mjs @c-shots-vals-model.json |
| 29-vals-hlab-leaderboard.png | https://www.vals.ai/benchmarks/hlab | 19:48 | 榜单前 16 行（含前十，列：任务通过率、每题成本、耗时）、“View 63 more models”、脚注图标 Fallback Enabled / Refusals；脚注说明文字在悬停提示中，已存入 `c-vals-hlab.txt` | c-browser.mjs @c-shots-hlab.json |
| 30-vals-hlab-benchmark.png | 同上 | 19:49 | “Benchmark” 与 “Results” 两段（held-out 集、六个工具、结果段） | c-browser.mjs @c-shots-hlab30.json |
| 31-vals-hlab-methodology.png | 同上 | 19:47 | “Methodology” 全段（两个 LLM 评委、全部准则通过才算过） | c-browser.mjs @c-shots-hlab.json |
| 32-vals-legalbench-dataset.png | https://www.vals.ai/benchmarks/legal_bench | 19:52 | “Dataset and Context” 与六类任务 | c-browser.mjs @c-shots-lb.json |
| 33-vals-model-page-updates.png | https://www.vals.ai/models/zai_glm-legal | 19:45（隐藏浮层后重截） | “Updates” 全段：#7 of 79、$0.15 per test、cheapest in the top 10、$0.075/$0.25 | c-browser.mjs（HIDE=1）@c-shots-vals-model33.json |
| 34-vals-lrb-background-scoring.png | https://www.vals.ai/benchmarks/legal_research | 19:52 | “Background” 与 “Scoring”（美国法、all-pass、LLM 评委） | c-browser.mjs @c-shots-lrb.json |
| 35-vals-lrb-leaderboard.png | 同上 | 19:52 | 点开 “View 62 more models” 后榜单第 1–28 行（含 GLM 5.3、GLM 5.3 Flash、GLM Legal 三行） | c-expand-shot.mjs |
| 36-hf-search-glm-legal-none.png | https://huggingface.co/models?search=GLM-Legal | 19:53 | “Models 0 … No results found.” | c-browser.mjs @c-shots-hf.json |

重截记录：27 首次误框，已覆盖重截；33 首次截图带浮层，已隐藏固定元素后覆盖重截；29 加高以含脚注行；30 缩短以落在行间；35 加高以含榜单标题；36 裁去空白；这些覆盖的都是我本次自己生成的文件，没有删除过其他文件。

**意外**：约北京 20:00 自查时发现 `images/` 里我的 25–29 五张图消失（不是我删的；同目录其他组文件的大小也有变化，疑为并行执行方的覆盖/清理操作，原因未查明）。已用同一脚本与同一区间参数在北京 20:02 重新生成（25、27 与原图字节数相同；26、28、29 逐张重新打开确认），其余 30–36 在原处完好。Claude 合并时建议核对 25–29 在场。

编号区间 25–36 全部使用，没有超出；没有 effort/档位另行标注的官方表（官方图本身无脚注）。宽度均为 1400 px。

## 三、失败与限制

- OSChina 正文未取到（动态页）；163 网易号标题、OSChina 标题只来自搜索摘要。
- gate.com 403；ChainCatcher 转载标题只有 firecrawl 搜索摘要。
- 无中文官方发布页（已查 docs.bigmodel.cn 发布记录、zhipuai.cn 新闻、bigmodel.cn 首页）；公众号未查（需登录/搜狗入口，未尝试）。
- Vals 页的 “Hyperparameter” 折叠项文字来自渲染 DOM（`c-vals-model-zai-glm-legal-full.txt`），未单独截图。
- 任何页面均未登录；本组没有 X 页面截图（只用 fxtwitter 取了两条公开帖的 JSON 作线索，原样存档，帖作者是公开账号）。
- 中间调试用的探针脚本与草稿图放在会话临时目录，不入库。

## 四、自查

- 对 `docs/2610/1013/` 中本组文件 grep 本机用户名、绝对路径、邮箱：除压缩（minified）JS 中恰好含 “QQ” 字符的函数名（`c-bigmodel-new-releases.html`）外无命中；脚本与日志均用相对路径。
- 单个文件大小：最大 `c-cn-www-zhipuai-cn-zh-news.html` 约 945 KB、`c-bigmodel-new-releases.html` 939 KB，均 < 1 MB；HTML 与渲染 HTML 属“只存本地”类。
