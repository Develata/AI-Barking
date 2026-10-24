# 1023 期 B 组抓取日志（Block Dessa）

时间均为北京时间（UTC+8）；本机为美东（EDT），北京 = 本机 + 12 小时。工具：`barking fetch/shot/html2txt/heat`（仓库自带）、curl、opencli、Exa 搜索（候选来源发现用，只作线索）。`.headers` 文件仅存 URL、状态、Content-Type、Date，其余响应头按工具规则未存。fetch 的逐条结果另见 `b-fetch-log.jsonl`，截图动作见 `b-shots-log.jsonl`，热度见 `b-heat.json`。

## 一、页面与文档存档

| 时间 | URL | 工具 | 结果 | 存档文件 |
|---|---|---|---|---|
| 19:52 | https://block.xyz/dessa | barking fetch | 200 ok，1,069,761 字节，394 行 txt；人工核对 txt 为正文（含隐私声明） | b-dessa.html/.txt/.headers |
| 19:56 | https://www.theverge.com/ai-artificial-intelligence/1017906/jack-dorsey-block-dessa-ai-voice-speech | barking fetch（curl） | 200 ok | b-verge.* |
| 19:56 | https://www.indiatoday.in/technology/news/story/jack-dorseys-block-makes-ai-that-talks-like-humans-it-runs-without-internet-on-macbook-3024055-2026-10-24 | barking fetch | 200 ok | b-indiatoday.* |
| 19:56 | https://www.unite.ai/block-unveils-dessa-a-full-duplex-speech-model-trained-on-conversation/ | barking fetch | 200 ok | b-unite.* |
| 19:56 | https://runtimewire.com/article/block-dessa-speech-only-model | barking fetch | 200 ok（短稿，仅摘要，原文指向 Jack 帖） | b-runtimewire.* |
| 19:56 | https://www.aichatdaily.com/ai-models/block-unveils-dessa-full-duplex-voice-model-trained-only | barking fetch | 200 ok | b-aichatdaily.* |
| 19:56 | https://www.ndtvprofit.com/technology/jack-dorseys-block-unveils-dessa-ai-model-that-laughs-chats-with-a-few-awkward-moments-listen-in-12228676 | barking fetch | 失败：403 blocked，无 txt；未改用 opencli（该稿只是转引 Ragavan 帖，非必需，Exa 摘录已含引文） | b-ndtvprofit.html/.headers（558 字节错误页） |
| 19:56 | https://app.dealroom.co/news/note/jack-dorsey-s-block-unveils-dessa-a-full-duplex-voice-ai-trained-only-on-human-speech | barking fetch | 失败：403，Cloudflare 质询；未绕过，未用其内容（仅在 Exa 摘录里见到，二手） | b-dealroom.html/.headers（质询页） |
| 20:11 | https://sunmedia.tw/news/technology/1792795699-…（完整 URL 见 b-urls2.tsv） | barking fetch | 200 ok | b-sunmedia.* |
| 20:11 | https://huggingnews.com/ai/block-launches-dessa-voice-ai-trained-without-text-or-pretrained-models-3f84c4a2 | barking fetch | 200 ok | b-huggingnews.* |
| 20:11 | https://cellcog.ai/blog/block-dessa/ | barking fetch | 200 ok | b-cellcog.* |
| 20:12 | https://himiao.ai/news/2026-10-24-block-dessa-voice-model | barking fetch | 200 ok | b-himiao.* |
| 20:14 | https://news.ycombinator.com/item?id=50213925 | barking fetch | 200 ok（评论分数页面不显示） | b-hn.* |
| 20:14 | https://explainx.ai/blog/block-dessa-speech-only-full-duplex-model-trained-without-text-macbook-2026 | barking fetch | 200 ok | b-explainx.* |
| 20:14 | https://tsnmedia.org/block-dessa-speech-model/ | barking fetch | 200 ok | b-tsn.* |
| 20:14 | https://00011000.com/news/block-dessa-speech-only-voice-model | barking fetch | 200 ok | b-shuohe00011000.*（文件名前缀为误写，内容即 00011000.com 稿） |
| 20:17 | https://deploymentsafety.openai.com/gpt-live-1-1/gpt-live-1-1.pdf | barking fetch（PDF） | 200 ok，txt 经 poppler 转出（系统卡，标 2026-10-15） | b-oai-live11.pdf/.txt/.headers |
| 20:17 | https://openai.com/index/introducing-gpt-live-1-in-the-api/ | barking fetch | 200 ok（2026-09-10 发布页） | b-oai-live1-api.* |
| 20:13 | http://web.archive.org/web/20261023190837id_/https://block.xyz/dessa | curl（Wayback，仅用于查看该页面此前写了什么） | 200，gzip 压缩，解压后 1,069,761 字节；先前误把压缩原文当 HTML 转文本得到乱码 txt，已用 `rm` 精确路径删除该试验文件，重新解压后用 `barking html2txt` 生成 b-wayback-dessa.txt（394 行） | b-wayback-dessa.html/.txt |

fetch 清单文件（`b-urls.tsv` 在第二次运行后只剩第二批，已手工补回 dessa 一行；`b-urls2/3/4.tsv` 为后三批）：b-urls.tsv、b-urls2.tsv、b-urls3.tsv、b-urls4.tsv。

## 二、X 帖（公开 JSON，不含抓取者信息）

均用 `curl --max-time 60 https://api.fxtwitter.com/status/<id>`，2026-10-24 19:58–20:00 取，状态 200：

| 状态 id | 作者（公开） | 存档 | 发布时刻 |
|---|---|---|---|
| 2113671324400861645 | Ragavan Thurairatnam（Dessa lead） | b-x-2113671324400861645.json | 2026-10-23 16:38:08 UTC = 北京 10-24 00:38:08 |
| 2113691903866872251 | 同上 | b-x-2113691903866872251.json | 17:59:55 UTC = 北京 10-24 01:59:55 |
| 2113697902916751661 | Jack Dorsey | b-x-2113697902916751661.json（约 376 KB，<1 MB） | 18:23:45 UTC = 北京 10-24 02:23:45 |
| 2113704911749911028 | Ragavan Thurairatnam | b-x-2113704911749911028.json | 18:51:36 UTC = 北京 10-24 02:51:36 |

失败/未取：`https://api.fxtwitter.com/RagavanThurair`（用户页）返回 404 “User not found”，改取单帖；Dorsey 回复（India Today 引用 “speech to speech. no llm.” 与 “Yes”）与 Jack 对自己帖的后续回复（teapawt 镜像站摘录里见到 “small enough to run on your macbook”）：fxtwitter 不提供回复串，镜像站不用，故未取得一手原帖——标“未找到一手来源”。

## 三、社区热度

| 时间 | 查询 | 工具 | 结果 |
|---|---|---|---|
| 19:53 | HN “Dessa conversation model” + item 50213925 | barking heat | 405 分 / 149 评；b-heat.json、b-hn-50213925.json（嵌套评论 148 条）；Reddit 搜索被重定向到登录页，失败（记入 b-heat.json errors） |
| 20:16 | HN Algolia items/50213925 | curl | 407 分 / 150 评（取时最新）；帖子创建 2026-10-23T19:09:54Z |
| 20:02 | Reddit 搜索与读取 | opencli reddit search / read（opencli 自带 Reddit 适配器，非浏览器 Cookie 工具；未登录、未发帖） | r/SesameAI 帖 1xdobay 35 分 / 19 评；r/RuntimeWire 帖 1 分 / 0 评；存档 b-reddit-1xdobay.txt（6.8 KB） |

## 四、一手发布渠道排查

| 时间 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 20:0x | https://huggingface.co/api/organizations/blockxyz/overview | curl | 200：Block 认证组织，numModels/numSpaces/numDatasets/numPapers 均 0；`/api/models?author=blockxyz` 返回 [] |
| 20:0x | https://huggingface.co/api/models?search=dessa | curl | 200；结果均为无关用户（dessabba_ju-lora 等），无 Block 条目 |
| 20:0x | https://huggingface.co/api/organizations/block/overview | curl | 404 |
| 20:0x | https://api.github.com/orgs/block/repos?per_page=100&sort=pushed | curl（匿名 API） | 200；无名称含 dessa 的仓库；`/repos/block/dessa` 404；代码搜索 401（需登录，未用） |
| 20:10 | https://export.arxiv.org/api/query?search_query=all:Dessa+AND+all:speech | curl | 200；无条目 |
| 20:10 | https://dessa.com | curl -I 式 | 302 → https://block.xyz/dessa |
| 20:13 | https://web.archive.org/cdx/search/cdx?url=block.xyz/dessa | curl | 仅 1 个快照：20261023190837（200，416,591 字节压缩长度） |
| 19:5x–20:2x | Exa 搜索（英中，若干轮） | web_search_exa | 只用于发现来源与二手稿，所列二手稿内容均以存档或摘录为准；Exa 摘录里的 teapawt.org（X 镜像）、00011000.com 等未当一手来源 |

## 五、截图

| 时间 | 动作 | 工具 | 结果 |
|---|---|---|---|
| 19:58 | opencli 打开实时页面（background 窗口），eval 查可见/隐藏按钮；截一张 1400×3000 页首试验图 | opencli browser | 成功；实时页面可见按钮含 “Download”，“Join the waitlist” 为 hidden；试验图右缘有浏览器翻译插件图标，已用 `rm` 精确路径删除，不入交付 |
| 20:07 | headless Chrome 渲染 https://block.xyz/dessa（700 CSS px，DPR 2，高 3200 → 1400×6400） | barking shot render | 成功，中间图在临时目录 |
| 20:08–20:09 | 同页高 8190 → 1400×16380 | barking shot render | 成功 |
| 20:07–20:09 | 裁切 13、14、15、16 号图；中途 13 号第一次裁偏（标题被截断）、15 号两次起点不当、另裁过一张 17-dessa-masthead-raw.png（只有 logo，无 Download 按钮，无信息）；这些试验文件均已用 `rm` 精确路径删除，`b-shots-log.jsonl` 里保留了对应动作记录 | barking shot cropbox | 最终保留 4 张：13-dessa-speech-only-raw.png、14-dessa-local-raw.png、15-dessa-gptlive-raw.png、16-dessa-safety-access-raw.png，均逐张打开目视确认文字清楚、范围正确 |

深色底页面；无头渲染不含浏览器扩展图标与滚动条。

## 六、交付前自查

- 泄露检查：对 `docs/2610/1023/` 下本组文件（b-*、evidence-b.md、capture-log-b.md、13–16 号图）grep `develata|C:\Users`，无命中。宽泛的 `QQ` 在 b-dessa.html（base64 串 “…ONQQ+Dhhv…”）、b-cellcog.html、PNG/PDF 二进制里有巧合匹配，均为噪声，不是路径。
- 已知超 1 MB 的文件（均为 HTML 原件，不入库）：b-dessa.html、b-wayback-dessa.html（各约 1.07 MB）。文本存档均小于 1 MB（最大 b-x-2113697902916751661.json 约 376 KB）。
- 假设：opencli 的 reddit 命令被当作 API 适配器使用，未核实其是否读取本机浏览器 Cookie；GB200 NVL72 是整机柜系统为 NVIDIA 产品常识，未另取 NVIDIA 页面；“无头渲染不显示 Download 按钮”未深究原因。
