# D 组证据：速览（1112 期）

取证时间：2026-11-13 22:28 起至约 22:58 北京时间（UTC+8）。无截图（D 组按派工不截图）。图表数字取自本组下载的图片本地读取（不入库），图中数值为目视读取。抓取与检索记录见 `capture-log-d.md`。

## 一、结论概要

- **D1 腾讯混元 Hy Realtime**：官方公众号“腾讯混元”原文已存档（2026-11-13 16:32）。上线范围与“近期开放腾讯云 API”已找到。扫描称“自建基础智商测试集 61.01 分，对比豆包 APP 40.63 等”**与说法不符**：图中 61.01 是 Gemini-3.8-Flash，Hy Realtime 为 39.33，图中无豆包、GPT-Live、Gemini Live、Qwen APP。
- **D2 ElevenLabs Scribe v3**：官方博客已存档（Nov 10, 2026）。厂商数字为 **27%**（扫描写“约 25%”）。AA 第 2 名 1.9% 属实，但 **第 1 名是 Fun-Realtime-ASR-preview 1.7%**，扫描写成 Qwen3.5 Omni Flash，该模型在 AA 为 13.5%。
- **D3 Google DeepMind × 奔驰试点**：DeepMind 原文已存档（Nov 10, 2026，时刻官方未给）。96% 为 “real-world proof-of-concept” 自报数字，未披露试验次数与时长。
- **D4 窗口内补充**：按派工单候选源（HN 首页、官方 newsroom、develata.me 日报页）检索，窗口内（11-12 06:00 起）满足“正在热 + 一手材料”的新条目只有 **1 条（Claude 认证事故，状态页 L1）**，另有 1 条热点但按速览规则建议不收（Anthropic IPO）。其余候选见第四节。

---

## 二、逐条证据

### D1 腾讯混元 Hy Realtime（公众号原文）

一手来源：公众号“腾讯混元”文章 `https://mp.weixin.qq.com/s?src=11&timestamp=1794580512&ver=7086&signature=8xdJl7Zdnl77RuDuAJJSeIiHJql6R8LOeXAHMNyj5L4X1aPIW37of50bldfy-RMbC0gx8Zf6UIExa6sgeD6hOK9ORcUrU5u0kkmahsR36NOTeHLRdJkaI3V0h3xYdzgP&new=1`（标题“实时语音模型 Hy Realtime 发布：能说会唱、还能调用工具”；存档 `sources/d-wx-hy-official/` 下 .md，仅正文文本）。页面显示发布时间 **2026年11月13日 16:32**（北京时间，页面地区标注“广东”）。

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图/图件文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| D1-1 | 腾讯混元发布实时语音模型 Hy Realtime（11-13） | L1（公众号“腾讯混元”） | 上段 mp 链接 | “今天，我们发布全新实时语音模型 Hy Realtime。” | 无（正文 .md） | 2026-11-13 16:32 北京（页面显示） | 已找到 |
| D1-2 | 已在元宝语音通话、腾讯客服上线，腾讯云 API“近期”开放 | L1 | 同上 | “Hy Realtime 已在元宝语音通话、腾讯客服等产品上线，近期也将上线腾讯云，对外提供 API 服务。” | 无 | API 当时未开放（原文“近期也将”） | 已找到（API 未开放，以原文为准） |
| D1-3 | 内部自建基础智商测试集 61.01 分，对比豆包 APP 40.63、GPT-Live APP 32.10、Gemini Live APP 27.93、Qwen APP 21.40 | L1 | 同上 | 正文只写：“在覆盖数学、科学、逻辑推理、知识问答、复杂指令、文本理解、多轮等多个维度的内部自建基础智商测试集上，非思考模式的中文表现不俗。”（无具体数字，数字在配图中） | `sources/d-wx-hy-img1.png`，图题“基础智商评测[自建]”，基准名 Hy-Eval-Audio3.0（图中读数：Gemini-3.8-Flash **61.01**；Qwen3.8-Omni-Flash 40.92；Hy Realtime **39.33**（深色段）；Hy2-O 19.54（浅色段）；Seed2.0-Lite 36.75；GPT-Realtime-2.1 30.43） | 非思考模式为正文口径；图中对比对象为 Gemini-3.8-Flash、Qwen3.8-Omni-Flash、Seed2.0-Lite、GPT-Realtime-2.1 | **与说法不符**：61.01 属 Gemini-3.8-Flash，不是 Hy Realtime；图中无豆包、GPT-Live、Gemini Live、Qwen APP 及对应分数 |
| D1-4 | 图注：Gemini-3.8-Flash 为 low think，其他关闭 think | L1 | 同上 | “注：Gemini-3.8-Flash 为 low think 模式，其他模型均关闭 think 模式。” | `d-wx-hy-img1.png`、`d-wx-hy-img2.png` 图底注 | 两张图的注释相同，只对 Gemini 一个模型标 low think | 已找到 |
| D1-5 | 在语音交互多个公开 Benchmark 上综合表现领先（正文） | L1 | 同上 | “在语音交互领域多个公开 Benchmark 上，综合客观表现处于领先水平。” | `d-wx-hy-img2.png`（公开 Benchmark，四项；深色段为 Hy Realtime：Big Bench Audio 98.60、Voice Bench 92.20、Open Audio Bench 85.84、Wild Speech Bench 75.35） | 图中对照：Gemini-3.8-Flash 98.50 / 92.26 / 86.85 / 73.61；Qwen3.8-Omni-Flash 99.10 / 86.76 / 82.67 / 71.36；Seed2.0-Lite 91.80 / 87.10 / 85.26 / 74.80；GPT-Realtime-2.1 87.10 / 89.15 / 81.00 / 71.37 | **部分支持**：四项中只有 Wild Speech Bench 为图中最高；Big Bench Audio 上 Qwen 高 0.50，Voice Bench 与 Open Audio Bench 上 Gemini 分别高 0.06、1.01。图中未说明柱上第二个数字含义（推测为 Hy2-O，未证实） |

补充：官网快照（opencli 22:37）首页与 `hunyuan.tencent.com/news`（404）均未见 Hy Realtime 条目，只能说明快照时点无此条目，不否定公众号原文。

### D2 ElevenLabs Scribe v3 / v3 Live

一手来源：`https://elevenlabs.io/blog/scribe-v3`（存档 `sources/d-d-scribe-v3-blog.{html,txt}`；页面 “Published Nov 10, 2026”，**官方未给时刻**）。

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图/图件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| D2-1 | 厂商称 102 语言词错率降约 25%（相对 Scribe v2） | L1 | 上段博客 | “Across 102 languages on FLEURS, Scribe v3 reduces WER by 27% compared with Scribe v2 and improves in every language tested.” | 无 | 相对降幅（非绝对值）；厂商“internal version of the FLEURS benchmark”，原文：“tests short, clean read speech in 102 languages” | **与说法不符**：数字为 27%，不是约 25% |
| D2-2 | AA 榜 Scribe v3 词错率 1.9%，次于 Qwen3.5 Omni Flash 1.7% | L3（AA 原始页） | `https://artificialanalysis.ai/speech-to-text/non-streaming`（存档 `sources/d-aa-stt-nonstreaming.{html,txt}`，快照 2026-11-13 22:30） | “The top speech to text models by accuracy (AA-WER) are: 1. Fun-Realtime-ASR-preview (1.7%), 2. Scribe v3, ElevenLabs (1.9%), 3. MAI-Transcribe-2 (2.0%), 4. Zoom Scribe V2.1 Pro (2.1%), and 5. Voxtral Small Transcribe 3 (2.1%).” | 无 | AA-WER v2 = AgentTalk 50% + VoxPopuli-Cleaned-AA 25% + Earnings22-Cleaned-AA 25%（指标定义页 `artificialanalysis.ai/methodology/speech-to-text`，本组未存档）。页面计数：“47 of 83 models”（WER 指数图）与“40 of 64 models”（价格/速度图）并存 | **部分支持**：Scribe v3 1.9% 与第 2 名属实；但 1.7% 属 Fun-Realtime-ASR-preview，**Qwen3.5 Omni Flash 为 13.5%**（表内）；“64 个模型”只见于局部图的计数，总计数为 83 |
| D2-3 | （未在扫描中）Scribe v3 Live 相对 Scribe v2 Realtime | L1 | 同上 | 第 81 段：“makes 43% fewer errors than Scribe v2 Realtime”；第 133 段：“reduces semantic WER by 43% and WER by 31% compared with Scribe v2 Realtime.” | 无 | 同一博客内两处表述不一致：81 段未注明“semantic”，133 段为 semantic WER 43%、WER 31% | 已找到（内部不一致，以 133 段为准更完整） |
| D2-4 | 博客称 Scribe v3 Live 有说话人分离、多语切换（扫描未提） | L1 | 同上 | “Scribe v3 Live adds a multilingual mode for conversations that switch languages mid-sentence.” | 无 | 厂商口径 | 已找到 |

补充：AA 的 ElevenLabs 提供方页（`d-aa-elevenlabs-models.txt`）静态文本中未见 Scribe v3 行，未另行核实其数字。ElevenLabs 博客称 16 语种真实场景数据集“We’ll be open sourcing this soon”，尚未开源。

### D3 Google DeepMind × 奔驰工厂试点（Gemini Robotics 2.5）

一手来源：`https://deepmind.google/blog/gemini-robotics-2-5-accelerates-real-world-physical-intelligence/`（存档 `sources/d-dm-gr25-accelerates.{html,txt}`；页面日期 “November 10, 2026”，**官方未给时刻**）。

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图/图件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| D3-1 | DeepMind 与 Mercedes-Benz 工厂试点，96% 成功率 | L1 | 上段 | “During a real-world proof-of-concept on Mercedes-Benz production lines, our models helped Apptronik’s Apollo 2 humanoid pick and pack automotive parts with a 96% success rate.” | 无 | 自报；**概念验证（proof-of-concept）**；生产线：Kecskemét（匈牙利）与 Sindelfingen（德国）；任务为 kitting（从散装箱取件、按每车顺序分盘）。原文未写试验次数、时长、失败判据（正文检索 “trial / attempt / number of” 无命中） | **部分支持**：数字与场景属实，但为厂商自报的概念验证，非独立评测 |
| D3-2 | 试点结论 | L1 | 同上 | “Achieving a 96% success rate, this pilot shows promising progress toward widespread real-world industrial applications.” | 无 | 同上 | 已找到 |
| D3-3 | 模型名称 | L1 | 同上 | 博客标题与正文均为 “Gemini Robotics 2.5”（动作模型 Gemini Robotics 2.5，具身推理 Gemini Robotics ER） | 无 | 次级媒体有写作 “Gemini 4 Argon”，仅见于二手摘要，未在一手页面核到 | 与部分转述不符（一手为 Gemini Robotics 2.5） |

HN 热度（见第五节）：6 分 1 评（11-11 22:09 北京）。

---

## 三、扫描说法勘误（须写入正文前核对）

1. **D1 “61.01 分，对比豆包 APP 40.63 …”**：与一手图不符。61.01 是 Gemini-3.8-Flash；Hy Realtime 为 39.33；图中对比对象与扫描列的四个 APP 不同。“非思考模式”来自正文原句。
2. **D1 “图注 Gemini-3.8-Flash 为 low think，其他关闭 think”**：一手图注属实，但它只对 Gemini 一个模型说明 low think 档，其余关闭 think；扫描未说明“其他”指的是哪些模型。
3. **D2 “厂商自称跨 102 种语言词错率降约 25%”**：厂商原数字为 27%，并且是相对 Scribe v2 的降幅，测量集为内部版 FLEURS（短句干净朗读）。
4. **D2 “AA 64 个模型中次于 Qwen3.5 Omni Flash 1.7%”**：1.7% 属 Fun-Realtime-ASR-preview；Qwen3.5 Omni Flash 为 13.5%；页面计数并存 83 与 64。
5. **D3 “96% 为厂商特定测试”**：与一手原文一致（proof-of-concept、自报）；扫描未给次数与时长，正文同样不披露。
6. **D3 模型名**：一手为 Gemini Robotics 2.5；次级转述中“Gemini 4 Argon”不可靠。

---

## 四、D4 窗口内补充（11-12 06:00 至今）

检索口径：HN 首页快照（`sources/d-hn-front.html`）、Anthropic/Google 新闻页（listing）、develata.me 日报页（`/daily/news/AI_ML/2026/20261112` 与 `20261113`，已存档，作为线索 L5）、xAI/NVIDIA/Meta/Mistral/中国大模型厂商的 WebSearch（只作线索，未据此引用）。`aihot.news` 本组两次请求均失败（协议错误/无连接），未能打开。

| 候选 | 一手材料 | 时间（北京） | 热度 | 判断 |
|---|---|---|---|---|
| **Claude 认证与服务错误（状态页事故）** | `https://status.claude.com/incidents/nn11wk485zfn`（存档 `sources/d-claude-incident.{html,txt}`，L1） | 原文 “Investigating … Posted Nov 13, 2026 - 09:58 UTC”（北京 17:58）；“Monitoring … 11:05 UTC”；“Resolved … 11:58 UTC”（北京 19:58）。摘句：“We are investigating an issue with users authenticating to Claude.ai and Claude Code.”；“The issue affecting claude.ai and Claude Cowork has been resolved.” | HN 50481187，346 分 / 300 评（11-13 18:18 北京） | **候选，建议 Claude 决定是否收**：一手、窗口内、热度高；但属事故，影响时长约 2 小时，普通读者关注度有限。事实只写状态页原文，不写原因 |
| Anthropic IPO（热点，融资类） | `https://ipo.anthropic.com/us/`（存档 `d-anth-ipo.*`，L1）；CNBC 报道 `…/2026/11/10/anthropic-sets-price-range-of-70-to-78-in-upcoming-ipo.html`（存档 `d-cnbc-ipo-range.*`，L4） | 价格区间 CNBC 原文：“Published Tue, Nov 10 2026 6:40 AM EST”；“Anthropic said it will offer 825 million shares in its upcoming IPO at a price range between $70 and $78 per share, according to an updated filing.”（北京 11-10 19:40） | HN 50465829，973 分 / 882 评（11-12 19:46 北京） | **不建议收**：融资与上市事件，速览规则“融资合同等不收”；且定价区间发布于 11-10，早于窗口 |
| Microsoft Research “Lazarus” 权重后门 | `https://www.microsoft.com/en-us/research/blog/lazare-veni-foras-reprogramming-dormant-neurons-to-backdoor-ai-agents/`（listing 确认，WebFetch 只得到导语，未存档全文） | 2026-11-10（页面无时刻） | 未查 HN | **窗口外且未取全文**（核心数字“少于 10 个神经元”只见于次级聚合，本组未核）。不收，除非 Claude 决定另派取证 |
| NVIDIA GR00T 2 | 无一手页面存档（`research.nvidia.com/labs/gear/gr00t-2` 仅在搜索结果中出现） | CoRL 发布，次级报道写 2026-11-09 | 未查 | **窗口外**；次级报道对开放时间说法不一。不收 |
| Meta Muse Ember（Muse Ember 1.0） | 未取一手页 | 次级搜索写 11-09 | 未查 | **窗口外**，未取证。不收 |
| Jensen Huang 辞任 Nvidia CEO（HN 热点） | AP 页 `apnews.com/article/nvidia-megaspeed-china-singapore-huang-…`，**403 Cloudflare 质询，未取到正文**（`d-ap-huang.*`） | HN 50467087，3764 分 / 3256 评（11-12 21:52 北京） | 极高 | **不收**：一手未取到；URL 与 HN 标题涉及中国、新加坡、出口相关与个人关系的未证实报道，按风控与隐私考虑不收，Claude 如要收需另派取证并判断风控 |
| 其他：Grok 生成儿童图像诉讼；美国数据商向中国实验室出售 RL 训练环境（ChinaTalk） | 次级媒体 / 调查报道 | 11-12 至 13 | — | **不收**：前者是诉讼与监管，无一手官方材料；后者为“美方点名指控中国公司”，属风控类 |
| DeepMind AlphaProof Code（Signal 形式化验证）；Gemini Decision 1 | 一手页面未重取 | 11-12 至 13（develata 日报线索） | — | **不收**：1110 与 1111 已收（近 3 天速览已收表），无实质新进展 |
| Helsing Clarity（武器目标建议平台） | 次级线索 | — | — | **不收**：未取一手，且涉及武器决策与人工授权争议，题材判断留给 Claude |

结论：D4 按门槛（正在热 + 一手 + 窗口内 + 非风控）合格的条目 **0 至 1 条**（仅 Claude 事故一条，且是否收待 Claude 决定），未凑数。

---

## 五、热度（`barking heat`，北京时间，`sources/d-heat.json`）

| 条目 | HN 命中 | Reddit |
|---|---|---|
| Scribe v3 | 50457165，2 分 / 0 评，11-12 03:06（“Scribe v3 and v3 Live”，链接 elevenlabs 博客） | 检索失败（Reddit JSON 返回登录页，记失败，未重试） |
| Gemini Robotics 2.5（DeepMind 试点） | 50453617，6 分 / 1 评，11-11 22:09；50441849，2 分 / 0 评，11-10 23:35 | 同上 |
| Hy Realtime（腾讯混元） | 无命中 | 未检索（同一 Reddit 限制） |
| Claude 认证事故（D4 候选） | 50481187，346 分 / 300 评，11-13 18:18 | — |
| Anthropic IPO（D4 不建议收） | 50465829，973 分 / 882 评，11-12 19:46 | — |
| Jensen Huang 辞任（D4 不收） | 50467087，3764 分 / 3256 评，11-12 21:52 | — |

D 组按派工单只记热度数字，未逐条摘 HN 评论。HN 上 Scribe v3 帖 0 评、Gemini Robotics 2.5 帖共 1 评，均无可摘的质疑评论。Hy Realtime 的 HN 与 Reddit 均未查到，Reddit 检索受登录页限制未能完成。
