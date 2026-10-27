# 1026 期 B 组抓取记录

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8），本机美东（EDT），北京 = 本机 + 12 小时。机器记录另见 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`。清单文件 `b-list1.tsv`–`b-list5.tsv`。

## 一、网页与 PDF 存档（`barking fetch`，前缀 b）

| 北京时间 | name | URL | 结果 |
|---|---|---|---|
| 22:52 | aa-article | https://artificialanalysis.ai/articles/gpt-6-1-terra-pushes-the-cost-efficiency-frontier | ok（图表为客户端图，txt 只含文字） |
| 22:52 | aa-model、aa-releases | https://artificialanalysis.ai/models/gpt-6-1-terra ；…/models/releases/gpt-6-1-terra | ok |
| 22:52 | aa-leaderboard | https://artificialanalysis.ai/leaderboards/models | ok（表格被拆成逐行文本，只用于读 Terra/Sol/Astra/Luna 各档成本） |
| 22:52 | aa-method-intel、aa-method-omni | https://artificialanalysis.ai/methodology/intelligence-benchmarking ；…/evaluations/omniscience | ok |
| 22:52 | aa-method-cai、aa-eval-tb | …/methodology/coding-agent-index ；…/evaluations/terminal-bench-hard | failed（HTTP 404，URL 猜错；已另取有效页，见下） |
| 22:52 | openai-terra | https://openai.com/index/introducing-gpt-6-1-terra/ | ok（curl 返回英文正文，200；这次没有被 403） |
| 22:59 | aa-m-56terra（及 -xhigh/-high/-medium/-low）、aa-m-61sol、aa-m-astra、aa-m-luna、aa-m-terra-xhigh、aa-m-terra-low、aa-faq、aa-method、aa-eval-index、aa-cai | https://artificialanalysis.ai/models/… ；/faq ；/methodology ；/evaluations ；/agents/coding-agents | 全部 ok。5.6 Terra 页带 “This model is deprecated” 提示 |
| 23:01 | aa-method-agents | https://artificialanalysis.ai/methodology/coding-agents-benchmarking | ok |
| 23:01 | aa-m-56terra-nr、aa-m-terra-high、aa-m-terra-medium、aa-m-terra-nr、aa-m-sol-medium、aa-m-sol-high | https://artificialanalysis.ai/models/… | 全部 ok |
| 23:01 | openai-syscard、openai-sol、openai-astra | https://deploymentsafety.openai.com/gpt-6-1-terra ；https://openai.com/index/introducing-gpt-6-1-sol/ ；https://openai.com/index/gpt-6-astra/ | 全部 ok（只核对 Terminal-Bench harness 说明；Sol、Astra 发布页未精读） |
| 23:17 | oa-devdocs | https://developers.openai.com/api/docs/models/gpt-6.1-terra | ok |
| 23:2x | unwire、ie、decoder、huxiu、ifanr、qq、36kr | 见 `b-list5.tsv` | 全部 ok；qq（腾讯新闻）txt 仅 19 行，只取其中两段 |
| 23:2x | ghacks | https://www.ghacks.net/2026/10/27/openai-launches-gpt-6-1-terra-with-api-pricing-50-lower-than-gpt-5-6-terra/ | blocked（Cloudflare 质询），未绕、未用 |

## 二、其他请求

| 北京时间 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 23:0x | AA 文章内 5 张图的原图（sanity CDN，PNG） | curl | 只下载到本机临时目录用于读数，**未入库、未作为配图**；配图一律用页面渲染截图 |
| 23:1x | https://openai.com/news/rss.xml | curl | 200，取 pubDate（仅日期） |
| 23:1x | https://hn.algolia.com/api/v1/search?query=GPT-6.1 Terra | curl | 200，取 HN 条目 id 与提交时刻 |
| 23:17 | `barking heat`（HN 条目 50248124、50249076；Reddit 搜索 “GPT-6.1 Terra”） | barking | HN 成功（`b-hn-50248124.json`、`b-hn-50249076.json`、`b-heat.json`）；Reddit 被重定向到登录页，记入 errors，未登录、未绕 |
| 23:1x | https://api.fxtwitter.com/status/2114781865811063259 | curl | 200，存 `b-fx-2114781865811063259.json`（AA 官方账号公开字段，无抓取者信息）；首帖时刻 2026-10-26T18:11:02Z |
| 23:1x | `opencli twitter tweets ArtificialAnlys --limit 40` | opencli | 成功，只用于核对 AA 官方 Terra 帖子时刻（18:11:02Z，与 fxtwitter 一致）；时间线 JSON 未入库 |
| 23:1x | https://artificialanalysis.ai/agents/coding-agents | opencli（后台窗口，只读文本） | 成功，只用于搜 “Astra” 与 “77”：页面只有 Codex - GPT-6 Astra (max) 等图例，未见 77；未存档 |
| 23:2x | WebSearch（中、英文各 1–2 次） | WebSearch | 仅用于找报道线索；未把搜索摘要当来源，引用的都来自存档页 |

## 三、截图（`images/13`–`24`）

渲染工具：AA 页面用 `barking shot render`（headless Chrome，700 CSS px，DPR 2）；OpenAI 发布页 headless 渲染得到空白，改用 opencli 浏览器（`--window background`；站点按浏览器语言 zh-CN 返回中文页，改打开 `https://openai.com/en-US/index/introducing-gpt-6-1-terra/` 后 OpenAI 重定向回英文规范 URL；页面 `zoom 2`、`--width 1400`；为得到干净画面，截图前在页面里把固定/粘性导航与浏览器翻译插件的悬浮按钮设为隐藏，只改显示，不动数据、图表与文字）。OpenAI 页面是深色底。所有图只裁切、拼接，不改像素，裁切记录见 `b-shots-log.jsonl`。页面均未登录，截图内无账号信息。

| 文件 | 来源 | 内容 | 方法 |
|---|---|---|---|
| 13-aa-takeaways.png | AA 文章 | 页头（标题、日期）+ 价格段 + Key takeaways 全部四条（含 Pareto 档位句） | render 后 stack 两段（1400×1884） |
| 14-aa-intel-index.png | AA 文章 | Intelligence Index 柱状图与 Intelligence vs Cost 散点图（含档位标签、图例） | render + cropbox（1400×1000） |
| 15-aa-coding-agent-index.png | AA 文章 | Coding Agent Index 柱图与 vs Cost 散点图 | cropbox（1400×1040） |
| 16-aa-omniscience.png | AA 文章 | AA-Omniscience Index、Accuracy、Hallucination Rate 三图（含定义说明） | cropbox（1400×1360） |
| 17-aa-terminal-bench.png | AA 文章 | AA 的 Terminal-Bench 4.0 柱图 | cropbox（1400×450） |
| 18-aa-evaluations.png | AA 文章 | Intelligence Index 各子评测小图集（含 AA-Omniscience Non-Hallucination Rate、GDP.pdf 等） | cropbox（1400×1890；字较小） |
| 19-aa-model-page.png | AA 模型页 | #25/221、48、$0.40、130M、Comparison Summary、1M 上下文 | render 后 stack 两段（1400×2464） |
| 20-oa-intro.png | OpenAI 发布页 | 开头两段（价格比前代与 Sol 低 50%、ChatGPT Work 与 Codex 额度） | opencli 截图 + cropbox（1380×820） |
| 21-oa-factual-error.png | OpenAI 发布页 | 小标题 + 错误率段落 + “Factual error rate on difficult prompts” 图 | opencli 两张 stack（1380×1744） |
| 22-oa-terminal-bench.png | OpenAI 发布页 | Terminal-Bench 4.0 段落（50.3%、21.5%、Sol/Astra 仍是更好选择）+ 图 | opencli 两张 stack（1380×1634） |
| 23-oa-pricing.png | OpenAI 发布页 | “Pricing and availability” 段 | opencli + cropbox（1380×500） |
| 24-aa-releases-by-effort.png | AA release 页 | Terra 6.1 各档位 Intelligence 柱图与 Cost per Task 柱图 | render 后 stack 两段（1400×1374；下沿边框被裁） |

注意：这些截图已逐张打开目视，文字可读；21、22 的图表区在 OpenAI 页面里没有数值标注，读点位需要目测。

## 四、失败与偏差

- AA 方法页两个猜测的 URL 404（见上），已改用有效页。
- OpenAI 发布页 headless Chrome 渲染为空白（原因未查）；opencli 浏览器首次打开是中文版，已改英文。
- Reddit 搜索被重定向到登录页，未取到。
- 一次 `barking shot cropbox` 试验误把输出路径指到仓库外，被拒绝（未生成文件）；`images/` 下无本组废弃文件。
- OpenAI 发布页的 Agents’ Last Exam、MMMU-Pro、OSWorld 2.1、Terminal-Bench-Science 0.1 四张图，因编号区间限制未入库。

## 五、待删除文件

无（`images/` 下 13–24 之外无 b 组文件；`C:\Users\QQ\AppData\Local\Temp\barking-shot\26101026-b\` 与 scratchpad 里的中间文件不在仓库内）。
