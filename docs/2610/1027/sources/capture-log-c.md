# 1027 期 C 组抓取与截图记录

执行：Claude Sonnet 5.5（claude-sonnet-5-5）。本机时区 EDT（UTC-4），北京时间 = 本机 + 12 h；下表时刻为 `barking` 日志写入的 +08:00 北京时间。所有页面匿名访问，未登录、未输入凭据、未接受 Cookie 弹窗（HF 与 Cognition 页面的浏览器翻译插件浮窗已在裁切中去掉）。

工具：`cargo run --release --manifest-path tools/barking/Cargo.toml -- fetch|shot|heat`；`opencli` 1.8.8 已连接（doctor：Everything looks good）；`curl --max-time 60`；`opencli brave search`（只找线索，不当来源）。未用 firecrawl，未用 Reddit/X 登录，未运行读取浏览器 Cookie 的工具。

## 一、网页与 JSON 存档（`barking fetch`，清单 `c-urls*.tsv`，结果 `c-fetch-log.jsonl`）

| 北京时刻（10-28） | 条目 | URL | 结果 |
|---|---|---|---|
| 20:29:50 | aa-v5 | https://artificialanalysis.ai/articles/artificial-analysis-intelligence-index-v5 | ok（正文 209 行，含全部文字；图表为客户端图，txt 无图内数字） |
| 20:29:53 | aa-changelog | https://artificialanalysis.ai/changelog | ok |
| 20:29:52 | aa-method | https://artificialanalysis.ai/methodology/intelligence-benchmarking | ok（页首 v5.0；表格在窄屏横向滚动，txt 含 Private 列符号） |
| 20:29:53 | aa-models | https://artificialanalysis.ai/models/ | ok（榜单入口，未精读） |
| 20:29:54 | aa-leaderboard | https://artificialanalysis.ai/leaderboards/models | ok（表被拆成逐行文本；已用脚本按行取名称/分数/成本，未入库） |
| 20:29:55 / 20:29:57 | aa-m-kimi、aa-m-ling | https://artificialanalysis.ai/models/kimi-k3-1-pro ；…/ling-3-1-flash-1019 | ok |
| 20:29:50–51 | hf-ling、hf-ling-config、hf-ling-api | https://huggingface.co/inclusionAI/Ling-3.1-flash-1019 ；…/raw/main/config.json ；https://huggingface.co/api/models/inclusionAI/Ling-3.1-flash-1019 | ok |
| 20:29:52 | cognition-fc2 | https://cognition.ai/blog/frontier-code-2.0 | ok（数据表在 txt；图表需浏览器渲染） |
| 20:34:29–39 | v5-m-*（19 个模型页：GPT-5.6 Terra 各档 6、GPT-6.1 Terra 各档 5（xhigh/high/medium/low/non-reasoning）、Sol max/high/medium、Astra、Opus、Argon、DeepSeek、GLM、Haiku） | https://artificialanalysis.ai/models/<slug> | 全部 ok（slug 取自 `docs/2610/1026/sources/b-fetch-log.jsonl` 的同一批 URL，便于逐页对照 v4.3.2） |
| 20:34:39–40 | aa-fc2-eval、aa-progress、aa-kimi-article | …/evaluations/frontiercode-2-0 ；…/methodology/progress-index ；…/articles/kimi-k3-1-pro-launch-benchmarks | ok |
| 20:37:17–19 | gtht-10jqka、qq-kimi、sina-v5、trendingtopics-v5、officechai-v5、techapple-v5、cnmo-kimi | 见 evidence C9 / 夸大实例 | ok |
| 20:37:18 | gtht-investing | https://cn.investing.com/news/stock-market-news/article-3631311 | **blocked**（HTTP 403，Cloudflare 质询）；未改用其他渠道；同稿已由同花顺与网易订阅转载页存档 |
| 20:38:28–30 | gtht-163、alphasignal-v5 | https://www.163.com/dy/article/LAAD7FNI05198UNI.html ；https://alphasignal.ai/news/artificial-analysis-moves-intelligence-index-to-60-private-tests-to-stop-ai | ok |
| 20:38:45 | smzdm-kimi-trend | https://post.smzdm.com/p/a26x9zkp/ | ok |
| 20:51:23 | aa-m-glmflash、aa-m-qwenflash | https://artificialanalysis.ai/models/glm-5-3-flash ；…/qwen3-8-flash-next | ok（核对 HN 评论所称 “N/A”） |
| 20:52:07 | v5-m-terra-max | https://artificialanalysis.ai/models/gpt-6-1-terra | ok |

## 二、其他请求

| 北京时刻（约） | URL / 命令 | 工具 | 结果 |
|---|---|---|---|
| 20:30 | `opencli twitter tweets ArtificialAnlys --limit 30` | opencli | 成功；只用于读 AA 官方 v5 帖时刻（`created_at` 2026-10-28T02:11:22Z）；时间线输出未入库 |
| 20:3x | https://api.fxtwitter.com/status/2115265133467635877 ；…/2113688083145314416 ；…/2115132488775524793 | curl | 200；存 `c-fx-2115265133467635877.json`（AA v5 首帖）、`c-fx-2113688083145314416.json`（AA Kimi K3.1 Pro 首帖，2026-10-23T17:44:44Z）、`c-fx-2115132488775524793.json`（Cognition FrontierCode 2.0 首帖，2026-10-27T17:24:17Z）；均为公开机构账号字段，无抓取者信息 |
| 20:3x | `opencli twitter tweets cognition --limit 15` | opencli | 成功；只用于核对 Cognition 帖时刻；未入库 |
| 20:31 | https://huggingface.co/api/models?author=moonshotai&sort=lastModified&direction=-1&limit=15 | curl | 200，存 `c-hf-moonshot-api.json`；无 Kimi-K3.1 条目 |
| 20:31 | https://huggingface.co/api/models/moonshotai/Kimi-K3.1-Pro 、…/Kimi-K3.1 、…/Kimi-K3.1-Pro-Base | curl | 均 HTTP 401（HF 对不存在 / 非公开仓库的响应） |
| 20:3x | https://huggingface.co/inclusionAI/Ling-3.1-flash/raw/main/config.json 与 …/api/models/inclusionAI/Ling-3.1-flash 、…/Ling-3.1-flash-1019/commits/main | curl | 200；只读取 `max_position_embeddings`、`rope_scaling`、`createdAt`、提交时间；未存档（前代 config 值 262144 / null 记入 evidence C7-5） |
| 20:33 | `barking heat docs/2610/1027 c --item 50268868 --item 50269651 --hn "Artificial Analysis Intelligence Index v5" --reddit "…v5" --reddit "Artificial Analysis v5 Kimi K3.1"` | barking | HN 成功（`c-heat.json`、`c-hn-50268868.json`、`c-hn-50269651.json`）；Reddit 两次被重定向到登录页，**未取到**（记入 errors，未登录） |
| 20:3x | HN Algolia `https://hn.algolia.com/api/v1/search?query=Artificial%20Analysis%20Intelligence%20Index&tags=story` | curl | 200；用于找 v5 相关帖（id 50268868、50269651）与 v4.2/v4.3 历史帖时刻；结果未入库 |
| 20:3x | WebSearch ×2（中文） | WebSearch | 返回的是 10-24 的旧页面索引，没有 v5 页面；**未使用其结果** |
| 20:3x–20:4x | `opencli brave search`（中文约 8 条检索，如“国泰海通 Kimi K3.1 Pro Artificial Analysis 55分”“Artificial Analysis Intelligence Index v5 Kimi 55分 降至46分”） | opencli | 只用于找线索与 URL；摘要不当来源；所引稿件均另行 `fetch` 存档 |
| 20:2x–20:4x | `opencli 36kr search "Artificial Analysis"` | opencli | 返回无关稿；未用 |

## 三、截图（`images/25`–`36`）

渲染：AA 页面、Hugging Face 页面与 raw config.json 用 `barking shot render`（headless Chrome，700 CSS px 宽，DPR 2）；方法页表格与 Cognition 图表在 headless 下不能完整显示，改用 opencli 浏览器（`--window background`，视口 1264 CSS px，`screenshot --width 1400`，**像素比 1×**）。所有图均为浅色底；只裁切、拼接，未改像素。截图页面均未登录，图内无账号信息；所有成图均不含站点导航栏（Log in / Premium 按钮等）；`25` 含 “All articles / 日期 / 标题” 页头行。

| 文件 | 来源 | 内容 | 方法与备注 |
|---|---|---|---|
| `25-aa-v5-intro-lists.png`（1400×1689） | v5 公告 | 返回链接+日期+标题；导语（含 “60% of the Index”）至 “Removed from the Index” 的全部列表 | render（`v5article`，1400×18000）→ stack 两段：[0,150,1400,480] + [0,1040,1400,2375]（中间为横幅图，略去） |
| `26-aa-v5-weights-results.png`（1400×1000） | v5 公告 | 四类权重 + Key results 三条（含 “open weights gap widens from 3 points to 12”） | cropbox [0,2400,1400,3400] |
| `27-aa-v5-score-chart.png`（1400×600） | v5 公告 | Intelligence Index 柱状图，图标题 “v5.0 incorporates 9 evaluations”，含 Kimi 46、Ling 29 | cropbox [0,3440,1400,4040]；图中柱无图例色块，无档位以外的标签 |
| `28-aa-v5-openweights-gap.png`（1400×1010） | v5 公告 | “Open weights models fall further behind in v5 … 3-point gap in v4.3” 段 + “by Open Weights / Proprietary” 图（含图例；Kimi 为黑柱 Proprietary） | cropbox [0,11830,1400,12840] |
| `29-aa-v5-method-table.png`（1330×660） | 方法页 | v5.0 评测套件完整表：Category / Evaluation / Questions / Repeats / Response Type / Scoring / Intelligence Index Weighting / Tool Usage / **Private** | opencli 浏览器；**为让整张表可见，截图前用脚本改了页面显示：表格容器 overflow 改 visible、宽度 1280px、隐藏 fixed/sticky 元素（导航栏）；不动表中文字与数字**；1× 像素；cropbox [0,440,1330,1100]；原图中右侧浏览器翻译插件图标已裁掉 |
| `30-aa-changelog-v5-terra.png`（1400×2612） | Changelog | 27 Oct 的 v5 条目（含 “Removed from the Index: GDPval-AA v2, SciCode and AA-LCR”）；26 Oct 的 GPT-6.1 Terra 各档条目，现显示 30/29/38/42/45/51 | render（`changelog`）→ stack 两段：[0,845,1400,1745] + [0,2050,1400,3738] |
| `31-aa-kimi-launch-v432.png`（1400×1635） | AA 文章 10-23（现状页） | 日期、标题 “scoring 55”、图（图标题 “v4.3.2 incorporates 10 evaluations”）、“narrow from 12 points to 3” | render（`kimiold`）→ cropbox [0,150,1400,1785] |
| `32-aa-kimi-model-v5.png`（1400×1290） | AA Kimi K3.1 Pro 模型页 | “Proprietary model”、Intelligence 46（#20 / 51，Updated）、$3.68、110M、Comparison Summary 开头 | render（`kimi`）→ cropbox [0,240,1400,1530]；页头导航与 Log in 区已裁去 |
| `33-hf-ling-card-v432.png`（1400×1160） | HF Ling 模型卡 | Introduction 全段（含 “1M-token context window”“Intelligence Index v4.3.2, up from 41”）及其下 v4.3.2 柱图（箭头标 Ling 3.1 Flash 1019 = 44） | render（`hfling`）→ cropbox [0,2500,1400,3660]；左侧细竖线是 HF 页面目录按钮的一部分，无账号信息 |
| `34-aa-ling-model-v5.png`（1400×1290） | AA Ling 模型页 | “Open weights model”、Intelligence 29（#4 / 16，Updated）、$3.19、240M | render（`ling`）→ cropbox [0,240,1400,1530] |
| `35-hf-ling-config-maxpos.png`（1400×1069） | HF raw config.json | 上段含 `"max_position_embeddings": 262144,`；下段含 `"rope_scaling": null,` `"rope_theta": 6000000,`；两段之间有间隔（省略 num_nextn_predict_layers 等） | render（`hfconfig`，1400×13000）→ stack [0,2848,1400,3500] + [0,6567,1400,6960]；**等宽字体，白底** |
| `36-cognition-fc11-vs-20.png`（780×595） | Cognition 博文 | “FrontierCode 1.1 vs. FrontierCode 2.0” 折线图（含各模型 1.1 与 2.0 数值）与脚注 “Each model is shown at its best reasoning effort.” | opencli 浏览器，1× 像素；cropbox [380,165,1160,760]；原图左侧站点导航与右侧翻译插件图标已裁掉；图为 1.1 标签页（FrontierCode 1.1）下的对比图，博文中 FrontierCode 2.0 分数图本身（客户端渲染）未单独截 |

截图目视确认：`25`–`36` 均已逐张打开成图（`29` 的 Private 列、`36` 的脚注均在框内），文字清楚、范围正确。`35-hf-ling-config-262144.png` 为第一版，上下段拼缝处有半行字符被切（边界落在行中），已由 `35-hf-ling-config-maxpos.png` 取代。

## 四、未完成 / 未做

- Wayback 查 v5 站点切换时刻：未做。
- 蚂蚁官方新闻稿：未检索。
- Reddit：被登录页拦截。
- `36` 为 1× 像素；`29` 为 1× 像素。若要在批注图里放大到 1080 宽，这两张可能略虚。
- Cognition FrontierCode 2.0 的分数图（柱状图）和 AA FrontierCode 2.0 的分数图本身未截。
- 未留“仍写 55 分的中文稿”截图（只找到发布于 v5 之前的稿，见 evidence C9）。

## 五、待删除文件

- `docs/2610/1027/images/35-hf-ling-config-262144.png`（被 `35-hf-ling-config-maxpos.png` 取代）

其余 `c-*` 文件均为本组存档，保留。`c-urls.tsv`…`c-urls6.tsv` 为 fetch 清单，保留。

## 六、自查

- 单个文本文件超过 1 MB：无（最大 `c-aa-leaderboard.txt` 与 `c-aa-method.txt` 均小于 1 MB；`.html` 原件不入库，其中多个超过 1 MB，约 1–4.5 MB，由 Claude 事后处理）。
- 抓取者信息：存档文本与 JSON 内 grep 邮箱、用户名、本机路径未命中；fxtwitter JSON 只含 AA 与 Cognition 的公开账号字段。
