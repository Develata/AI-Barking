# A 组抓取日志（1025 期，十年前 CNN 对前沿 LLM 的皮肤癌测试）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；本机美东（EDT），北京 = 本机 + 12 小时。机器记录另见 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json`。所有抓取未登录、未发帖；抓取不带 Cookie；HTML/PDF/zip/图片原件只存本地。

## 一、barking fetch 批量存档（`a-<name>.html|.txt|.headers`）

| 时间 | name | URL | 结果 |
|---|---|---|---|
| 20:33 | article | https://lynchlab.net/llm-skin-cancer | ok（31 KB，119 行文本；正文完整） |
| 20:36 | inception | https://lynchlab.net/skin-ai-inception | ok（模型说明页，文本 54 行） |
| 20:36 | drmagnuslynch | https://www.drmagnuslynch.com/can-llms-diagnose-skin-cancer | ok（作者 10-11 前作，202 行） |
| 20:36 | milk-doi | https://doi.org/10.34970/648456 | ok（落到 ISIC Archive MILK10k 数据集页） |
| 20:36 | isic-home | https://www.isic-archive.com | ok（HTML 1.37 MB，仅本地；txt 可入库） |
| 20:36 | lynch-home | https://lynchlab.net/ | ok（实验室主页） |
| 20:38 | hn-item | https://news.ycombinator.com/item?id=50233779 | ok（250 条评论；586 分） |
| 20:38 | hn-algolia | https://hn.algolia.com/api/v1/items/50233779 | ok（JSON，无 txt） |
| 20:40 | gigazine | https://gigazine.net/gsc_news/en/20261026-cnn-vs-llm-skin-cancer-diagnosis/ | ok |
| 20:40 | hyperai | https://hyper.ai/en/stories/1786dbecdb7427e1a4e20b85fba1ab94 | ok |
| 20:40 | devto-beats | https://dev.to/trillioniar_s_14a3c313e14/a-10-year-old-cnn-beats-gpt-6-at-skin-cancer-and-llm-prices-fall-80-1jl2 | ok |
| 20:40 | devto-es | https://dev.to/lu1tr0n/cnn-de-2015-supera-a-gpt-6-astra-y-claude-62-vs-42-en-diagnostico-de-piel-22f7 | ok（西语，仅存档，未引用） |
| 20:40 | aitldr | https://ai-tldr.dev/releases/lynch-lab-cnn-vs-llm-skin-cancer/ | ok |
| 20:40 | techbytes | https://techbytes.app/posts/a-10-year-old-cnn-outperforms-frontier-llms-for-skin/ | ok |
| 20:40 | aigeek | https://aigeeknews.com/ | ok（中文聚合首页，无该题条目；仅存档，未引用） |
| 20:41 | elsolitario | https://elsolitario.org/2026/10/25/un-cnn-de-2015-le-gana-a-gpt-6-astra/ | ok |
| 20:41 | promptzone | https://www.promptzone.com/ayaka_nkrumah/a-10-year-old-cnn-outperforms-gpt-4o-on-skin-cancer-diagnosis-3p1k | ok |
| 20:41 | habr | https://habr.com/ru/articles/1099308/ | ok（俄语，仅存档，未引用） |
| 20:41 | cisoclub | https://cisoclub.ru/nejroset-2015-goda-oboshla-dve-peredovye-jazykovye-modeli-v-teste-na-diagnostiku-raka-kozhi | ok（HTML 498 KB，仅存档，未引用） |
| 20:41 | drweb | https://www.drweb.de/hautkrebs-diagnose-bildmodell-sprachmodelle/ | ok（德语，HTML 632 KB，仅存档，未引用） |

清单文件：`a-list1.tsv`…`a-list5.tsv`（每次 fetch 的输入）。无 blocked / failed 条目。

## 二、个别请求

| 时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 20:33 | https://d2gaqzonlm544h.cloudfront.net/skin-ai/data/llm-skin-cancer-test.zip | curl（HEAD 后 GET，--max-time 120） | 200，562,309 字节，存 `a-data-zip/llm-skin-cancer-test.zip`（仅本地）；用 Python 标准库 `zipfile` 解到 `a-data-zip/x/`（只读文本；未运行包内任何脚本）。这是作者在文章里链接的数据包，按派工单“代码或数据仓库链接一并存档”下载 |
| 20:50 | https://www.drmagnuslynch.com/static/data/llm-skin-cancer-test.zip | curl HEAD | 只回了代理的 “200 Connection established” 行，未取得响应头（未重试，未下载；与上面 cloudfront 那份是否同一文件**未核**） |
| 20:37 | Reddit 搜索（barking heat 内部） | barking heat | 失败：返回登录页（old.reddit.com/login），记入 `a-heat.json` errors；未换来源重试（Reddit 无已知主帖） |
| 20:37 | HN Algolia 搜索（barking heat） | barking heat | ok，命中 1 条（id 50233779，586 分 / 250 评） |
| 20:41–20:45 | WebSearch 四次（英文、中文关键词） | WebSearch | 用于找二次传播稿；结果里的链接已逐一用 fetch 存档，搜索摘要本身不作为来源 |

## 三、截图（`docs/2610/1025/images/`；记录见 `a-shots-log.jsonl`）

| 时间 | 文件 | 来源 | 方法 |
|---|---|---|---|
| 20:43 | （中间图，仓库外）a-full.png 1400×14113 | https://lynchlab.net/llm-skin-cancer | opencli browser `a-shot`：open --window background → eval zoom=2 → screenshot --width 1400 --full-page |
| 20:45 | 02、04、05、06、07、08、09、10 | 同上 | barking shot cropbox（自 a-full.png），浅色页 |
| 20:45 | 01、03 首版 | 同上 | cropbox 自 a-full.png；首版 01 右侧带浏览器翻译插件浮标、首版 03 缺一张懒加载的皮肤镜图，两张已弃用（移到仓库外的 scratchpad，`a-shots-log.jsonl` 里仍有这两行旧记录） |
| 20:46 | 01-header-fig1.png（定版） | 同上 | barking shot render（headless Chrome，700 CSS px，DPR 2）→ cropbox |
| 20:49 | 03-examples-fig2.png（定版） | 同上 | render --height 7100 全页 → cropbox |
| 20:52 | 11-cnn-model-card.png | https://lynchlab.net/skin-ai-inception | render（深色页）→ cropbox |
| 20:53 | 12-hn-card.png | https://news.ycombinator.com/item?id=50233779 | render → cropbox（593 分 / 250 评） |

均只裁切不改像素；宽 1400 px；已逐张打开目视确认文字清楚、边界落在文字行之间。浏览器翻译插件浮标只在 opencli 全页图首屏出现，定版 01 已用无头渲染避开。

## 四、本组自写脚本与计算输出

- `a-verify-llm.py`、`a-verify-llm.out.txt`：读 zip 解出的 `results/*-v2.jsonl`（LLM 回复）复算 84/85、top-2、留下/放走数、分病种、并集等，不依赖作者的脚本；输出与原文表格一致。
- `a-wilson.out.txt`：对原文数字补算 Wilson 95% 区间（非原文，仅供参考）。
- `a-hn-parse.py`：解析 HN 页面存档，按页面顺序列出顶层评论、sungam 的发言。

## 五、没做/没取到

- Reddit：搜索被登录墙挡，未取到。
- HN 评论得分：匿名页面不显示，未取到（用页面顺序代替）。
- 中文媒体同题报道：两轮搜索未找到。
- 10-11 前作“Which lesions would each model send home?”一节未截图（01–12 编号已用满）；CNN 论文级来源：Inception v3 原论文未取（原文只称 “first published by Google in 2015”，未核论文日期）。
- CNN 的逐例输出不在数据包里，CNN 数字（125、18/30、77、76 等）只能引作者自报，无法独立复算。
