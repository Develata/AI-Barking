# 1029 期 D 组抓取日志

执行方：claude-haiku-5-5。时间为北京时间。工具：`barking fetch`（匿名 GET，结果记入 `d-fetch-log.jsonl`）、opencli 浏览器（已连接的会话，只读取 innerText）、Exa 搜索（只用于定位一手 URL 与核对，不作证据存档）、HN Algolia 与 `barking heat`（热度）。未登录、未发帖、未点击任何控件，未使用 Cookie 工具。

## 抓取

| 时刻（北京） | 条目 | URL | 工具 | 结果 | 说明 |
|---|---|---|---|---|---|
| 23:08 | D1 | https://openai.com/index/updated-model-spec/ | barking fetch | ok | 正文 258 行；curl 403 未出现 |
| 23:08 | D1 | https://model-spec.openai.com/ | barking fetch | blocked | 正文过短（JS 渲染），改用 opencli，见 23:12 |
| 23:08 | D2 | https://www.theverge.com/policy/1022880/wikipedia-openai-new-york-times-copyright | barking fetch | ok | 590 行 |
| 23:08 | D3 | https://ir.aboutamazon.com/news-release/…/Amazon-com-Announces-Third-Quarter-Results/ | barking fetch | blocked | 403，Cloudflare 质询；改 opencli |
| 23:08 | D4 | https://apps.apple.com/us/app/style-sketch-outfit-planner/id6794461626 | barking fetch | failed | 404（美区未上架，推断） |
| 23:08 | D4 | https://techcrunch.com/2026/10/29/meta-quietly-launches-an-ai-fashion-app-called-style-sketch/ | barking fetch | ok | 326 行 |
| 23:08 | D5 | https://sierra.ai/blog/our-customers-ambition-is-accelerating-our-growth | barking fetch | ok | 188 行 |
| 23:08 | D6 | https://github.com/Tencent-Hunyuan/Hy-Nomas | barking fetch | ok | 931 行；星数 408 取自此页 |
| 23:08 | D6 | https://arxiv.org/abs/2610.36729 | barking fetch | ok | 232 行 |
| 23:08 | D7 | https://martinfowler.com/bliki/LLM-Voice.html | barking fetch | ok | 285 行 |
| 23:10 | D2 | https://wikimediafoundation.org/news/2026/10/30/…/ | barking fetch | ok | 297 行 |
| 23:10 | D2 | https://diff.wikimedia.org/2026/10/30/ | barking fetch | ok | 误取的 Diff 首页，未用，见待删除 |
| 23:10 | D2 | https://commons.wikimedia.org/wiki/File:26.10.29_WMF_motion_and_amicus_In_re_OAI.pdf | barking fetch | ok | Commons 文件页 |
| 23:10 | D2 | https://diff.wikimedia.org/2026/10/30/why-wikimedia-is-speaking-up-in-a-major-ai-copyright-case/ | barking fetch | ok | 基金会 Diff 文章 |
| 23:10 | D1 | https://openai.com/news/rss.xml | barking fetch | ok | 取 pubDate（.xml，未生成 .txt） |
| 23:10 | D2 | https://upload.wikimedia.org/…/26.10.29_WMF_motion_and_amicus_In_re_OAI.pdf | barking fetch | ok | 意见书 PDF，27 页（动议函 3 页另计），txt 由 pdftotext 生成 |
| 23:10 | D4 | https://apps.apple.com/app/style-sketch-outfit-planner/id6794461626 | barking fetch | failed | 404（无国家码的链接，误写，见待删除） |
| 23:11 | D4 | https://apps.apple.com/id/app/style-sketch-outfit-planner/id6794461626 | barking fetch | ok | 印尼区 App Store 页 |
| 23:11 | D1 | https://model-spec.openai.com/2026-10-29.html | barking fetch | ok | 2.5 MB HTML，txt 5899 行（仅本地用于摘句） |
| 23:11 | D1 | https://github.com/openai/model_spec/blob/main/CHANGELOG.md | barking fetch | ok | 版本史，取 v2026.10.29 与 v2026.08.18 |
| 23:11 | D1 | https://github.com/openai/model_spec | barking fetch | ok | 仓库页，未用 |
| 23:11 | D1 | https://model-spec.openai.com/ | barking fetch | blocked | 同上（第二次） |
| 23:11 | D6 | https://huggingface.co/tencent/Hy-Nomas | barking fetch | ok | 许可 Apache-2.0 |
| 23:11 | D6 | http://export.arxiv.org/api/query?id_list=2610.36729 | barking fetch | ok | 发布时间 2026-10-27T06:48:59Z |
| 23:12 | D3 | https://ir.aboutamazon.com/…（同上） | opencli browser d3-ir open + eval | ok | innerText 41 KB，存 `d-d3-amazon-opencli.txt` |
| 23:12 | D1 | https://model-spec.openai.com/ | opencli browser d1-spec open + eval | ok | 页头 October 29, 2026；前 3000 字符摘读，未存档为文件 |

## 搜索与热度

| 时刻（北京） | 查询 | 工具 | 结果 |
|---|---|---|---|
| 23:12 | `barking heat docs/2610/1029 d --item 50290177 --hn Hy-Nomas --hn "Style Sketch" --hn "Model Spec" --hn "Wikimedia amicus OpenAI" --hn "Amazon third quarter results" --hn "Sierra ARR" --reddit "Style Sketch"` | barking heat | 写入 `d-heat.json`。HN：Hy-Nomas 14 分 0 评（10-27 23:24）；Fowler 条目 50290177 540 分 288 评（提交 10-29 22:57:55）；“Model Spec” 关键词命中 24 条，多为无关标题（如 Dessa、Tesla），只取 50293081（1 分，10-30 01:46）；Wikimedia、Amazon、Sierra、Style Sketch 无命中。Reddit 返回登录页（HTML，非 JSON），失败。 |
| 23:14 | 同上 HN Algolia 近 7 天 “Wikimedia OpenAI” | curl HN Algolia | 0 条 |
| 23:14 | “Sierra ARR” | curl HN Algolia | 无相关 |
| 23:14 | “Style Sketch” | curl HN Algolia | 无相关（命中 Implementing a…，与 Meta 应用无关） |

## 搜索定位（非证据存档）

Exa 搜索用于定位 D2 的 WMF 新闻、Diff、Commons PDF、CourtListener 案卷页（1:25-md-03143）与 D4 的 TechCrunch 发布时刻；D4 的 AppInsider 与 Storyboard18 等二手页只用于对照“仅三国”与 17+/18+ 差异，未存档，不作证据。

## 失败与限制

- Reddit 热度不可用（登录重定向）。
- D1 model-spec 主页经 curl 与 barking fetch 均 blocked，改用 opencli 读页头与概述，版本页横幅矛盾未能解决。
- D3 curl 403 Cloudflare，改用 opencli 成功。
- D4 美区 App Store 页 404，记为未上架美区（推断，未经 Apple 官方确认）。
- D2 法院案卷页仅通过搜索摘要核对（未存档）；法院对动议的批准状态未找到。

## 待删除文件（不 rm，由 Claude 收尾时一次删除）

- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d1-model-spec.headers`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d1-model-spec.html`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d1-spec-latest.headers`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d1-spec-latest.html`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d3-amazon-q3.headers`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d3-amazon-q3.html`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d4-style-sketch-us.headers`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d4-style-sketch-us.html`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d4-style-sketch-id.headers`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d4-style-sketch-id.html`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d2-diff-blog.headers`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d2-diff-blog.html`
- `E:\gitclone\AI-Barking\docs\2610\1029\sources\d-d2-diff-blog.txt`

注：`d-d1-spec-repo.*`、`d-d1-openai-rss.*` 与 `d-d6-arxiv-api.*` 保留（分别支撑版本核对与 arXiv 时刻）。`d-heat.json` 的“Model Spec”桶为噪声命中，不作依据；如需要可由 Claude 决定是否删除重跑。

## 体积

所有 `d-*.txt` 均小于 300 KB。HTML 原件（`d-d1-spec-2026-10-29.html` 2.5 MB、`d-d6-github.html` 430 KB 等）与 PDF（`d-d2-amicus-pdf.pdf`）只存本地，不入库。
