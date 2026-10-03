# 1002 来源入口与限制

本页供读者回到原始来源。页面存档与截图为北京时间 2026-10-02 21:2x 至 22:1x 抓取；期次目录按制作当日美国当地日期命名为 `1002`。派工文件为 `.handoff/2026-10-02-1002-evidence.md`。选题线索来自四份在对话中回贴、未单独存档的扫描（北京时间 10/2 10:58、12:01、16:58、20:50）；其中的若干错误说法已对原页核正，见 [fact-check.md](fact-check.md) 末节。

| 内容 | 来源 |
|---|---|
| Suncatcher 原型卫星入轨 | [Google：Our Project Suncatcher prototype satellite is in orbit](https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-prototype/)、[SpaceX：Transporter-18](https://www.spacex.com/launches/transporter18/)、[Planet 新闻稿](https://investors.planet.com/news/news-details/2026/Planet-Launches-Suncatcher-Tanager-2-and-18-SuperDove-Satellites/default.aspx) |
| 辐射与散热测试、2027 计划 | [Google：Behind Project Suncatcher](https://blog.google/innovation-and-ai/models-and-research/google-research/google-project-suncatcher-facts/)（9/24）、[Google 2025 年首发博文](https://blog.google/innovation-and-ai/technology/research/google-project-suncatcher/) |
| 论文：81 星示例编队、发射价情景 | [Joule：Toward a future space-based, highly scalable AI infrastructure system design](https://www.cell.com/joule/fulltext/S2542-4351(26)00362-4) |
| 4 颗 TPU、约 15 分钟、Gemma（媒体） | [NPR](https://www.npr.org/2026/10/01/nx-s1-5983697/project-suncatcher-google-ai-data-center-space)；另见 [Cocoloop 中文](https://news.cocoloop.cn/2026/10/google-suncatcher-tpu-in-orbit/)、[英文](https://news.cocoloop.cn/en/2026/10/google-suncatcher-tpu-in-orbit/)（写 Gemini） |
| OpenAI 审查进度（100+ 机构、50PB、7000 GPU） | [OpenAI：The Hugging Face incident and other third-party impact from misaligned models](https://openai.com/hugging-face-incident-and-misalignment/)、[8/26 技术报告页](https://openai.com/index/hugging-face-incident-and-the-road-ahead/) |
| 加州调查传票 | [加州司法部新闻稿](https://oag.ca.gov/news/press-releases/part-ongoing-investigation-attorney-general-bonta-serves-investigative-subpoena)、[Guardian（Reuters 稿）](https://www.theguardian.com/us-news/2026/oct/01/california-opens-investigation-openai-hack) |
| 未入正文：英国 AISI 恢复评测 | [AISI：Building a more secure environment for evaluating dangerous capabilities](https://www.aisi.gov.uk/blog/building-a-more-secure-environment-for-evaluating-dangerous-capabilities) |
| 未入正文：OpenAI 员工离职报道 | [BBC](https://www.bbc.com/news/articles/c6y9z9r4ejzwo)、[WSJ](https://www.wsj.com/tech/ai/openai-parts-ways-with-researchers-who-allegedly-shared-confidential-information-aebac528)（只见公开导语） |
| Clef / Clef-flash 发布、对比表、延迟表 | [Cloudflare：Introducing Clef](https://blog.cloudflare.com/clef-decision-models/)、[演示榜](https://clef-evals.workers-ai-mle.workers.dev/) |
| Jev Decision Index（上游社区榜） | [Hugging Face Space（multimodalart）](https://huggingface.co/spaces/multimodalart/jev-decision-index)、[方法页](https://multimodalart-jev-decision-index.static.hf.space/methodology.html) |
| 模型权重与许可证 | [Cloudflare/clef](https://huggingface.co/Cloudflare/clef)、[Cloudflare/clef-flash](https://huggingface.co/Cloudflare/clef-flash) |
| Jev 介绍与价格 | [TypeSafe：Introducing System One Models and Jev](https://typesafe.ai/blog/introducing-system-one-models-and-jev) |
| 训练数据未公开（媒体） | [The Register](https://www.theregister.com/ai-and-ml/2026/10/01/cloudflare-tries-to-outplay-jev-with-open-weight-clef-models/5300649) |

## 阅读限制

- Suncatcher：谷歌博文页面标 Oct 01, 2026，页面元数据发布时间 2026-10-01T23:30:00Z（北京 10/2 07:30，仅见于一次页面元数据读取，待核）；发射时刻取 SpaceX 任务页（10/1 11:32 PT = 北京 10/2 02:32），谷歌官方 X 帖未取到有效帖，互动数未核。4 颗 TPU、Gemma、每次约 15 分钟只来自 NPR 一篇署名报道，未注明消息来源，谷歌与 Planet 页面均未写，不能升级为谷歌的说法。Joule PDF 返回 403，页码未取得；发射价 200 美元/公斤是论文的学习曲线情景，论文现价参照为 3,600 美元/公斤。HN 上入轨帖热度很低（5 分），上线热度主要来自 NPR 等媒体与 Reddit；热度数字为抓取瞬间值。
- OpenAI：时间线页是原地更新的动态页面，“数十家”与“超过 100 家”两处口径同时存在于当前页面；“截至 9 月 26 日”是 OpenAI 自述的统计截止，9/30 条目官方只给日期。约 50PB、7000 块 GPU、日耗超 50 万美元均为 OpenAI 自述，未经第三方核实。加州新闻稿官宣日 10/1，送达日 9/30；司法部 9 月对 Hugging Face 事件的正式调查官宣稿未找到独立页面。Reuters 原站标题与导语已较早期索引改动，正文只引 Guardian 转载稿的可见文字。FTC 2026 年调查的官方文件未找到。离职报道只核到 BBC 与 WSJ 公开导语，不入正文。
- Clef：Cloudflare 博文只给日期与元数据时间（北京 10/1 23:34，修改 10/2 00:15）。对比表是 Cloudflare 自报；非 Cloudflare 模型的成绩与基准套件取自上游社区榜（Decision Index 0.2.1，9/28 快照，“Unofficial”），上游榜未见 Clef。演示榜的名次（Clef 61.2、Jev 57.9、Clef-flash 57.1）是演示站按上游公式重算，未跑的基准计 0。Clef 的延迟测试硬件、并发、是否含网络，一手页面未写；Jev 延迟含网络往返（上游方法页）。HN 上有用户自测称托管 Clef 的 p50 约 850 毫秒、Jev 约 110 毫秒（评论 id 49928781），为单人自测、未核，只作线索，未入正文。
- 热度数字（HN、Reddit、AIHOT）为北京时间 10/2 21:20–21:40 抓取的瞬间值，不是扫描时点，也不代表全站最高。
- 取证清单（[evidence.md](evidence.md)）、抓取日志（[capture-log.md](capture-log.md)）与[事实核验](fact-check.md)均保留为工作档案；配图说明见[这里](../images/README.md)。`*.py`、`*.ps1` 为取证时使用的抓取与整理脚本；`c-failed-jev-ranking.png` 是失败的拼接截图，仅作诊断，不是配图。
- X、Reddit 档案只保存公开帖子的文本、时间与链接，不含抓取者信息；档案里出现的第三方用户名来自公开帖文。

发现影响正文的错误时，在本期增加 `CORRECTION.md`，保留更正原因与来源。
