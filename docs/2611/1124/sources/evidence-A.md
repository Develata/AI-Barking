# 1124 A 组取证：MAGI-2 与 Artificial Analysis（AA）视频榜

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间一律北京时间（UTC+8，本机时钟），括号保留原时区。抓取记录见 `capture-log-A.md`，批量抓取逐条结果见 `a-fetch-log.jsonl`，截图动作见 `a-shots-log.jsonl`，热度见 `a-heat.json`。
级别按 `EDITORIAL.md` “事实分级”。**核心结论：（已对 AA 页面核实）AA 四个子榜里 MAGI-2 只在两个图生视频子榜排第一；文生视频带音频榜排第 7；文生视频无音频榜排第 2；AA 页还给 MAGI-2 标了 “Provisional result”（见 A1）。（已对存档核实）Sand.ai 官方英文博客与官方 X 发布帖没写“全球第一”，博客写的是“四项任务平均 Elo 1150.3”。（未核实）中文通稿的原件未取得：多家中文媒体首句同文写“在 AA 榜单中登顶全球第一”（网易写“据公司介绍”），无子榜限定，只有爱范儿、智东西、ZPedia 等在正文限定了图生视频；无法确定无限定措辞是通稿自带还是媒体删的。**

## 一、取证表

### A1　AA 四个子榜（本机 opencli 渲染后读取；“无音频”是页内切换，URL 参数 `?audio-output=false`，服务端直出 HTML 只含带音频视图，所以无音频视图存档的是渲染后 DOM）

读取时刻：I2V 带音频 02:03:44；I2V 无音频 02:04:00；T2V 带音频 02:04:29；T2V 无音频 02:04:43；02:27:53 用 curl 复读 I2V FAQ 句，仍为“MAGI-2 … Elo score of 1163”。四榜版本号：AA-Video-I2V v2.1、AA-Video-I2V-Silent v2.1、AA-Video-T2V v2.1、AA-Video-T2V-Silent v2.1。表格列：Range（排名区间）、Model、Elo（±95% 区间半宽，另给“95% interval a to b”）、Samples、Released、Pricing。

| 说法 | 级 | 一手来源 URL | 原文摘句（表格行，逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| MAGI-2 在 AA 图生视频带音频榜（AA-Video-I2V v2.1）排第 1 | L3 | https://artificialanalysis.ai/video/leaderboard/image-to-video | `1 | 1-3 | MAGI-2 | 1163 ±14 95% interval 1149 to 1177 | 5,943 | Nov 2026 | $4.80 /min`；第 2 行 `2 | 1-3 | Dreamina Seedance 2.5 | 1158 ±14 95% interval 1144 to 1172 | 7,384 | Jul 2026 | $34.12 /min`；第 3 行 Utopai X 1148 ±13（Range 2-5，“No API available”） | `images/01-aa-i2v-audio.png`（前 12 行）、`images/01-aa-i2v-audio-full.png`（全 39 行 + Pricing 脚注） | 榜首与第 2 名差 5 分（1163 vs 1158），两者 Range 都是 1–3，95% 区间区间重叠（按 Range 提示原句，Range 是“基于 Elo 置信区间的合理排名范围”）；样本 5,943 vs 7,384；FAQ 原句：“MAGI-2 currently leads AA-Video-I2V v2.1 with an Elo score of 1163.” | 已找到 |
| MAGI-2 在 AA 图生视频无音频榜（AA-Video-I2V-Silent v2.1）排第 1 | L3 | 同上，点 “No Audio”（`…/image-to-video?audio-output=false`） | `1 | 1-2 | MAGI-2 | 1153 ±14 95% interval 1139 to 1167 | 6,277 | Nov 2026 | $4.80 /min`；`2 | 1-2 | Utopai X | 1147 ±15 95% interval 1132 to 1162 | 4,676 | Sep 2026 | No API available`；`3 | 3-8 | Dreamina Seedance 2.5 | 1128 ±15 …` | `images/04-aa-i2v-silent.png`（前 12 行）、`images/04-aa-i2v-silent-full.png`（全 44 行 + 脚注） | 榜首与第 2 名差 6 分，Range 都是 1–2；Seedance 2.5 在此榜第 3、落后 25 分；页内说明句：“Ranks video models on animating a source image into silent video, by Elo from pairwise human preference votes.” | 已找到 |
| MAGI-2 在 AA 文生视频带音频榜（AA-Video-T2V v2.1）**不在前五**，排第 7 | L3 | https://artificialanalysis.ai/video/leaderboard/text-to-video | `7 | 4-11 | MAGI-2 | 1150 ±13 95% interval 1137 to 1163 | 5,441 | Nov 2026 | $4.80 /min`；榜首 `1 | 1-3 | Wan 3.0 | 1175 ±12 95% interval 1163 to 1187 | 9,201 | Aug 2026 | $12.00 /min`；第 2 `Utopai X 1173`，第 3 `MiniMax H3 (768p) 1164`，第 4 `Hyperflow H3 1156`（“Coming soon”），第 5 `Dreamina Seedance 2.5 1154 ±12`，第 6 `Gemini Omni Flash 1.2 1152` | `images/02-aa-t2v-audio.png`（前 12 行；第 7 行为 MAGI-2）、`images/02-aa-t2v-audio-full.png`（全 38 行 + 脚注） | MAGI-2 与榜首差 25 分；Range 4–11；与 FAQ 摘要句“Top models … 1. Wan 3.0 (Elo 1175), 2. Utopai X (Elo 1173), 3. MiniMax H3 (768p) (Elo 1164), 4. Hyperflow H3 (Elo 1156), and 5. Dreamina Seedance 2.5 (Elo 1154)”一致；**这是文生页的默认视图（With Audio）**；“AA 默认展示哪张”只证实到：文生页默认带音频、tab 顺序文生在前；派工单“默认展示的那张是文生带音频”这一说法我只验证了页内默认，没有 AA 官方对“默认榜”的说明 | 已找到 |
| MAGI-2 在 AA 文生视频无音频榜（AA-Video-T2V-Silent v2.1）排第 2 | L3 | 同上，点 “No Audio”（`…/text-to-video?audio-output=false`） | `1 | 1-5 | HeyGen Video 1 | 1147 ±14 95% interval 1133 to 1161 | 7,035 | Oct 2026 | $2.40 /min`；`2 | 1-7 | MAGI-2 | 1144 ±15 95% interval 1129 to 1159 | 5,313 | Nov 2026 | $4.80 /min`；第 3、4 并列 `Hyperflow H3 1141`、`Utopai X 1141` | `images/03-aa-t2v-silent.png`（前 12 行）、`images/03-aa-t2v-silent-full.png`（全 43 行 + 脚注） | 第 2 名，落后榜首 3 分，Range 1–7（与前 7 名区间重叠）；说明句：“Ranks video models on generating silent video from a text prompt …” | 已找到 |
| 榜单“默认展示”与定义 | L3 | 同上各页；Category 菜单见 `a-aa-category-menu.txt` | 页顶 tab：“Video Arena / Text to Video / Image to Video / Video Editing / Compare Models / Compare Providers”；I2V 带音频说明：“Ranks video models on animating a source image into video with audio, by Elo from pairwise human preference votes.”；T2V 带音频：“Ranks video models on generating video with audio from a text prompt, by Elo from pairwise human preference votes.”；Category 默认 “Overall”，菜单原句：“Overall — All use cases and capabilities, each weighted equally”；FAQ：“Rankings are based on blind votes from our validated private evaluators.” | 01–04 图内均含说明句 | AA 视频板块共 **三个** 排行页（文生、图生、视频编辑，tab 另有 Video Arena、Compare Models、Compare Providers）；每页再有 With Audio / No Audio 切换，才是“四个子榜”（AA 把无音频版本命名为 -Silent）。页面首个 tab 是 Text to Video，其默认视图是带音频版；Image to Video 页打了 “Updated” 标签。另有 Category 过滤（用例/能力），本组只读 Overall | 已找到 |
| AA 页自带的两条限定 | L3 | 各榜页（悬停提示用 DOM 事件触发，非原生悬停） | MAGI-2 名字后的虚线圆圈图标 aria-label=“Provisional result”，提示全文：“Provisional result: this model was added to the leaderboard recently. Score and rank may change as we collect and verify more votes.”（Dreamina Seedance 2.5 Flash (720p)、PixVerse V6 Max 等新模型同样带该图标；静态 HTML 里另有 “Provisional result: this model was released recently.” 字样）；表头 Range 旁 ⓘ 提示全文：“The range of plausible rankings based on this model's Elo confidence interval.” | 虚线圆圈在 `images/01…04` 及 `-full` 图里可见（图标本身，不含提示文字，未截提示气泡） | 02:38–02:39 读取；“Hide research previews”默认勾选，取消勾选后 T2V 带音频榜 MAGI-2 仍是第 7（1150，Range 4–11），多出 MAGI-2 Preview（第 21，1040）；Category 过滤默认 Overall | 已找到 |
| 榜单 Elo 每天漂移 | L3 + L5 | 本组多次读数 | 本机 02:03 读 I2V：MAGI-2 1163 / Seedance 2.5 1158；02:27:53 只复读了 FAQ 中 MAGI-2 的 1163（未复读 Seedance 2.5）；Claude 01:55 读 1163/1158；扫描称 11-25 当日（北京 13:15 的 ZPedia 搜狐稿）写“1165 … Seedance 2.5（1161 分）”；另有 rikoko（Nov 25，“supplied” 快照）写 1,165/1,161 且样本 5,740/7,162（本组读数 5,943/7,384），seeddance.io 评测稿写 I2V 无音频 1151 / T2V 无音频 1144（本组读 1153/1144）；这些都是第三方稿里的当时读数，原页读不到 | 01–04 及各 `-full` 图 | 各读数时刻不同；第三方转述的 1165/1161 在原榜页上读不到，只能当“当时的读数”；差值 4–5 分、小于 ±14 | 部分支持（漂移事实已找到，扫描所列读数只在第三方稿里） |
| AA 官方社媒有无发帖宣布 MAGI-2 名次 | L1 | X 站内搜索（已登录会话，只读；不存档） | 搜 “from:ArtificialAnlys MAGI”“MAGI-2 from:ArtificialAnlys”“from:ArtificialAnlys video arena since:2026-11-23”：均 “No results”；搜 “from:ArtificialAnlys Sand” 只出 2025-05 的无关帖 | 无 | 02:19–02:20 查；X 搜索可能不全；未经 fxtwitter（没有帖子 ID 可取） | 未找到一手来源 |

四榜前三对照（均为 02:03–02:05 读数）：

| 子榜 | 第 1 | 第 2 | 第 3 | MAGI-2 |
|---|---|---|---|---|
| I2V（带音频） | MAGI-2 1163 | Seedance 2.5 1158 | Utopai X 1148 | 第 1，Range 1–3 |
| I2V-Silent | MAGI-2 1153 | Utopai X 1147 | Seedance 2.5 1128 | 第 1，Range 1–2 |
| T2V（带音频） | Wan 3.0 1175 | Utopai X 1173 | MiniMax H3 (768p) 1164 | 第 7，1150，Range 4–11 |
| T2V-Silent | HeyGen Video 1 1147 | MAGI-2 1144 | Hyperflow H3 1141 / Utopai X 1141 | 第 2，Range 1–7 |

### A2　Sand.ai 官方口径

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 官方发布稿：MAGI-2 480B、约 28B 激活、4T token、30 秒 1080p 25fps、最多 20 份素材 | L1 | https://sand.ai/blog/magi-2/ | “Today, we are launching MAGI-2—our full-scale flagship model … MAGI-2 scales the base model more than 4× to 480B total parameters and ~28B active parameters and expands pre-training data roughly 3× to 4T tokens. It generates up to 30 seconds of 1080p, 25 fps native audio-video, supports omni-reference across images, video, and audio (up to 20 media items per native request), precision timeline control, and video editing.” | `images/05-sand-blog-hero.png`、`images/06-sand-blog-aa-avg-opensource.png` | 页面日期 “NOVEMBER 24, 2026 · RESEARCH”（官方只给日期、未给时刻、未标时区）；组成：最多 10 图、5 视频、5 音频（同文 §1） | 已找到 |
| **官方并未写“榜单第一”，写的是四项任务平均 Elo** | L1 | 同上 | “raising our 4-task average Elo on Artificial Analysis Video Arena v2.1 by +151.8 points over MAGI-2 Preview (998.5 → 1150.3)”；图注：“AA Video Arena Elo shows the 4-task average on Artificial Analysis Video Arena v2.1 (I2V, I2V-Silent, T2V-Silent, and T2V; Elo bars plotted from a 950 baseline).” 对 `a-sand-blog-magi2.txt` 与 `a-sand-home.txt` 用 `grep -E "first|No\. ?1|leading|best|SOTA|state-of-the-art|第一"`：命中只有 “head ownership first”“efficiency-first”“leading to stragglers” 等无关用法，**没有“第一 / 最强 / SOTA”类榜单表述**；官方自我定位用词只有 “Frontier”（博客标题 “A 480B Frontier Video Generation Model”）与首页的 “MAGI-2 possesses world-class video generation capabilities” | `images/06-sand-blog-aa-avg-caption.png`（柱状图与图注）、`images/06-sand-blog-aa-avg-opensource.png` | 官方 1150.3 是四榜平均；本组 02:03–02:05 读到的四榜 Elo（1163/1153/1150/1144）自算均值 1152.5，与 1150.3 差 2.2（自算，仅作 Elo 漂移旁证，L5 级别推断） | 已找到 |
| 官方“将开源”原句 | L1 | 同上 | “We will follow up soon with open-source releases and a detailed technical report covering full architectural specifications, system details, and evaluations.”；结尾：“Next, … we will open-source MAGI-2 and publish a comprehensive technical report” | `images/06-…`（前一句） | 无时间表、无许可证 | 已找到 |
| 官方自述局限 | L1 | 同上 §6 | “MAGI-2 still has clear room to improve: extreme high-speed motion can occasionally produce trajectory or continuity artifacts; medium- and wide-shot 2D animation can appear overly smooth in fine local motion; complex soundscapes still have room for richer layering and physical realism; and preserving pre-trained generalization through post-training remains an active research focus.” | 无 | 厂商自述 | 已找到 |
| 官方中文发布稿（公众号 / 官网中文页） | L1 | 未找到 | 搜索未返回 mp.weixin.qq.com 的 Sand.ai 原稿；中文媒体稿措辞高度一致（疑为同一通稿，推断），见 A6；17173（11-25 15:34:07）写：“即日起，MAGI-2 视频模型的 API 服务已正式开放申请，首批优先面向战略合作客户提供服务。企业及开发者可前往 Sand.ai 官方网站（sand.ai）申请接入。同时，个人创作者可通过 LibTV 体验 MAGI-2 模型的视频生成能力。” | 无 | 官网 sand.ai 无 /zh；platform.sand.ai 要登录（不存档） | 未找到一手来源 |
| 发布时刻 | L1 + L6 | 官方博客日期；官方 X 账号 @magiai_hq 帖（@SandAI_HQ 的简介写 “@vidmuse_ai & @magiai_hq are out now.”，据此认定为官方号） | 博客 “November 24, 2026”（无时刻）；X 帖 “Introducing MAGI-2 by Sand.ai, exclusively on @higgsfield … $0.08 per second of generated video. 400B-parameter open weights coming soon. … Now available worldwide on our launch partner Higgsfield’s platform and via the Higgsfield API.” 时间 Tue Nov 24 18:09:28 +0000 2026（北京 11-25 02:09:28）；中文媒体 11-25 13:15 起 | 无 | 中文稿都写“11 月 25 日”发布；官方博客写 11 月 24 日 | 已找到 |

### A3　价格

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| MAGI-2 官方 API 价格：0.08 美元/秒（1080p） | L1 | https://platform.sand.ai/docs/model-pricing/（公开文档页，无需登录；存 `a-sand-platform-model-pricing.txt` / `.dom.html`） | “Top-up is not supported for the time being. Please follow our X account SandAI_HQ for updates on the top-up launch date.” 表：“Model magi-2 / Resolution 1080p / Pricing 0.08 USD/s” | `images/07-sand-platform-pricing.png` | 02:21 读取；**同页明说暂不支持充值**；get-started 页：“Use a model enabled for your organization, such as `magi-2`.” | 已找到 |
| “0.5 元/秒”“1080P” | L1 缺失 / L4 | 官方页未找到；中文媒体稿 | 21 财经（新京报贝壳财经，北京 11-25 16:46）：“定价为每秒0.5元，为Seedance 2.5价格的1/10”；36 氪（智东西 16:56）：“The official list price of 1080P MAGI-2 is 0.5 yuan per second” | `images/10-zh-xjb-beike-top1-v2.png` | 官方公开页只有美元价 0.08 USD/s（`/docs/credits/` 写 “Users on the China site can also pay by business bank transfer … Alipay on the China site and Stripe-hosted checkout on the global site”，即存在中国站；本组读的是 global 站文档，**中国站价页未取得**，猜测的域名均无法解析）；人民币价只见媒体稿；rikoko 写 “A separate Chinese hosting offer quotes CNY 0.5 per second for MAGI-2”；AGI Hunt 摘要（AI 生成，转述公众号“卡尔的AI沃茨”）写“MAGI-2 的 1080P 官方价约 0.5 元/秒，约为 Seedance 2.5 同规格（约 5 元/秒）的十分之一，目前在 LibTV 内测”，其“约 5 元/秒”与火山方舟官方 3.74 元/秒（输入不含视频）不同（“官方刊例价”是媒体用语），Sand.ai 官方 X 帖同样写美元（$0.08 per second，经 Higgsfield） | 部分支持（0.5 元只有媒体出处） |
| Seedance 2.5 官方价（火山方舟）：1080p，输入不含视频 | L1 | https://docs.volcengine.com/docs/ark/model-pricing?lang=zh（及 ark.volcengine.com 同页） | token 单价表：“Seedance 2.5 / 1080p / 输入不含视频 77.00 / 输入包含视频 46.00”（单位：元/M token）；“价格示例”：“以下价格均按 16:9、输出视频时长 5 秒估算。” Seedance 2.5：“视频单价（元/个）480p 3.36 / 720p 7.56 / 1080p 18.71 / 4K 30.13；折合价格（元/秒）0.67 / 1.51 / 3.74 / 6.03”；Seedance 2.5 flash：“折合价格（元/秒）0.44 / 0.99 / 不支持 / 不支持” | `images/08-volc-seedance25-token-price.png`、`images/09-volc-seedance25-5s-price.png` | 02:23 读取；示例价限定“输入不含视频”页签、16:9、5 秒；含输入视频的页签数字不同（1080p 5 秒区间 20.12～78.25）；未自算比例 | 已找到 |
| AA 页标注的 API 价（美元/分钟） | L3 | AA 各榜页 Pricing 列 | MAGI-2 “$4.80 /min”；Dreamina Seedance 2.5 “$34.12 /min”；Seedance 2.5 Flash (720p) “$9.00 /min”；页脚注：“API Pricing reflects the cost to generate 1 minute of video on the model creator's API at the settings we benchmark (1080p where offered)” | 01–04 图含 Pricing 列 | 未自算比例；Utopai X “No API available”，Hyperflow H3 “Coming soon” | 已找到 |
| “约为 Seedance 2.5 的 1/10” | L4 | 见 A6 | 新浪科技：“定价为每秒0.5元，为Seedance2.5价格的1/10”；上证报：只写“定价为每秒0.5元”；21 财经同新浪 | 10 | 媒体写“同规格”；Sand.ai 官方页未见“1/10”（全文 grep blog：无） | 部分支持（出自媒体稿，官方页未见） |
| “便宜约 87%”（BlockBeats 英文快讯） | L5 | **BlockBeats 原快讯未取到**（搜索无 theblockbeats.info 结果） | 已存档的 KuCoin 转载（署 BlockBeats）写的是 “approximately one-tenth the cost of Seedance 2.5 at the same specification”，**没有 87%**；已存档 rikoko、i10x 等第三方页写 “about 14%”“roughly one seventh”（它们用美元/分钟或美元/秒自算） | 无 | 本组 73 个 `a-*.txt` 存档用 `grep -E "87 ?%|87％"` 命中 0 个文件；第三方自算：i10x “$2.40 on MAGI-2 and $17.07 on Seedance 2.5 … about 14% of ByteDance’s price”（Seedance 价取 BytePlus ModelArk，16:9、无视频输入） | 未找到一手来源（原文未取到；转载稿无 87%） |
| “低于 Seedance 2.5 Flash” | — | 未找到 | 本组存档均无此说法；官方 Flash 价：720p 0.99 元/秒、不支持 1080p（火山方舟页） | 09 | Flash 只到 720p，与 MAGI-2 的 1080p 价不同规格 | 未找到一手来源 |
| Higgsfield 上的价格 | L3（合作方官方） | https://higgsfield.ai/announcing-magi-2 | “At 2.5 credits per second, it outperforms Seedance 2.0, Wan 3.0, and Kling 4.0 in blind tests” | 12 | 独家合作平台（官方 X 帖称 “exclusively on @higgsfield”），按 credits 计 | 已找到 |

### A4　开源状态

| 说法 | 级 | 一手来源 URL | 原文摘句 / 数据 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| MAGI-2（480B）权重尚未发布 | L1 | https://huggingface.co/api/models?author=sand-ai（02:05、02:27 两次）；https://api.github.com/orgs/SandAI-org/repos；GitHub 仓库搜索；HF 模型搜索 “MAGI-2” | HF `sand-ai` 组织下只有 `sand-ai/MAGI-2-preview`（createdAt 2026-08-03T12:34:20Z，北京 08-03 20:34）与 `sand-ai/MAGI-1`；GitHub `SandAI-org` 下只有 `MAGI-2-preview`（created_at 2026-08-04T12:47:38Z，pushed 2026-08-06）等，**没有 MAGI-2 正式版仓库**；HF 搜索里的 `appmana/MAGI-2-preview-int8`、`saaivignesh20/MAGI-2-preview-FP8` 是第三方对 Preview 的量化 | 无 | 02:27 复查仍无；文本存档 `a-hf-sand-ai-models-api.json`、`a-gh-sandai-repos-api.json` | 已找到 |
| MAGI-2 Preview：114B 总参数、约 6B 激活、Apache-2.0 | L1 | https://huggingface.co/sand-ai/MAGI-2-preview ；https://github.com/SandAI-org/MAGI-2-preview | HF 模型卡：“Magi-2 Preview is a 114B-parameter unified audio-video generation model that activates just 6B parameters per token.”；“Clips are 10 seconds long”；“NVIDIA Hopper GPUs, 8 of them.”；“License: apache-2.0”（GitHub API：`Apache-2.0`） | 无 | Preview 博客日期 August 5, 2026；Preview 与现 MAGI-2 是两个模型 | 已找到 |
| “将尽快开放模型权重”原句 | L1 / L4 | 官方博客见 A2；中文稿 | 官方博客用的是 “We will follow up soon with open-source releases”；“将尽快开放 MAGI-2 模型权重”是中文媒体对官方表态的转述（新浪、21 财经、上证报），官方中文原文未找到；官方 X 帖：“400B-parameter open weights coming soon.” | 06 | 无时间表、无许可证；官方博客 480B、官方 X 帖与 Higgsfield 稿写 400B（见 A5） | 部分支持（官方英文有“将开源”；中文原句只在媒体稿） |
| Higgsfield 页标题 “Open Weights 400B” | L3 | https://higgsfield.ai/announcing-magi-2 | 标题：“Announcing MAGI-2: Open Weights 400B Parameters Multimodal-to-Video Model”；正文：“The 400B-parameter video model accepts up to 20 media references, with open weights coming soon.” | 12 | 标题与正文自相矛盾（现无权重）；参数写 400B，与官方博客 480B 不一致 | 已找到 |

### A5　厂商口径、无第三方复现的说法（均标“厂商自述”）

| 说法 | 级 | 出处 | 原句 | 第三方复现？ | 状态 |
|---|---|---|---|---|---|
| “480B 同期尺寸最大” | L4（媒体转述） | 新京报/新浪/上证报 | “模型参数量480B同期尺寸最大”；Higgsfield：“the largest SOTA open source omni-reference model to date” | 无；权重未发布，参数量无法独立核对；Higgsfield 稿与官方 X 帖写 400B，官方博客写 480B | 部分支持（厂商自述） |
| 数独自主解题（未用提示词增强 PE） | L1 | 官方博客 §5 | “On 9×9 Sudoku probes, for example, the model preserves all given digits and fills in the remaining cells so that every row, column, and 3×3 subgrid satisfies 1–9.” 前文：“the model shows emerging problem-solving ability (alongside failure cases typical of early emergence)”；示例标注 “480B pre-trained checkpoint, PE off” | 无（官方自己写明是预训练检查点、且后训练后这些行为“noticeably harder to elicit”）（§5 末） | 已找到（厂商自述；官方有限定） |
| 时间轴控制（切点命中），第三方一次性复测 | L5（seeddance.io，竞品相关站，一次生成） | https://www.seeddance.io/blog/magi-2-review | “In our run every cut landed within 0.44 seconds of its scripted time”；“The file was 1080p at 25 fps for $2.40, and it took 85 minutes to arrive.”；“This is one generation, so each result below is ‘what happened once’, not a success rate.” | 同站另一篇：“MAGI-2 ran through a third-party API that resells it at Sand.ai's list price, because Sand.ai's own platform was not accepting top-ups yet.”；Kling 4.0 同脚本 0.08 秒内 | 11-25 测试；与官方“0.08 秒”无可比关系（不同视频、不同模型） | 部分支持（L5 线索） |
| 12 个切点的误差 | L4 | 新浪科技、网易智能 | “在官方公布的一段测试视频中，12个切点全部命中，最大偏差为0.08秒。”（新浪 11-25 13:54；网易 14:30:56） | 无；**“平均误差 0.028 秒”在本组全部 73 个 `a-*.txt` 存档里 grep 不到**（`grep -l -E "0\.028" a-*.txt` 命中 0 个文件），媒体稿写的是“最大偏差 0.08 秒”，官方博客未见该数 | 与说法不符（扫描所写 0.028 秒找不到；媒体写最大偏差 0.08 秒） |
| 提示词上限 2 万字符 | L4 | 新京报等；36 氪 | 新京报稿：“MAGI-2支持最多2万字符的提示词”；i10x 写 “The API accepts prompts of up to 15,000 characters.” | 无 | 部分支持（官方博客未见，数字在媒体稿；第三方对 API 写 15,000） |
| Higgsfield 盲测：MAGI-2 排第 2（1146，对 Seedance 2.5 的 1219） | L3（合作方自家榜，利益相关） | https://higgsfield.ai/announcing-magi-2 | “MAGI-2 entered the Higgsfield Video Arena in second place out of six models, with a 1146 score and a 60.5% overall win rate.” “The one model ahead of it is Seedance 2.5, which won 61% of their matchups.” 快照 “Snapshot: Nov 24, 2026, 16:00 UTC”（北京 11-25 00:00） | 与 AA 不同榜、不同样本（六个模型）；Higgsfield 是 MAGI-2 的独家发布平台 | 已找到 |

### A6　夸大实例与热度

发布时刻均为页面标注（北京时间，无另行换算）。

| 说法/实例 | 级 | 来源 URL | 逐字原句 | 时间 / 发布方 | 截图 | 状态 |
|---|---|---|---|---|---|---|
| 新浪科技（标题 + 首句） | L4 | https://finance.sina.com.cn/tech/shenji/2026-11-25/doc-inizxpkf9563675.shtml | 标题：“Sand.ai发布MAGI-2：AA榜全球第一，价格仅为Seedance2.5 1/10”；首句：“在Artificial Analysis（AA）榜单中登顶全球第一” | 2026-11-25 13:54，新浪科技 | 无 | 已找到 |
| 投资界（标题） | L4/L5 | https://news.pedaily.cn/202611/572361.shtml | “Sand.ai发布MAGI-2：登顶全球第一，1080P每秒仅0.5元，将尽快开源”；导语：“登顶AA榜单参数量最大” | 更新于 2026-11-25 13:18，来源“投资界综合” | 无 | 已找到 |
| 网易科技（网易智能） | L4 | https://www.163.com/tech/article/LCJAP92V00098IEO.html | 标题：“Sand.ai发布MAGI-2：登顶全球第一 价格1080P每秒0.5元”；正文：“据公司介绍，该模型总参数量达4800亿，在Artificial Analysis（AA）榜单中排名全球第一” | 2026-11-25 14:30:56，网易智能 | 无 | 已找到 |
| 上证报·中国证券网（经同花顺转载） | L4 | http://stock.10jqka.com.cn/20261125/c682198478.shtml（来源标 cnstock.com/commonDetail/833387） | 标题：“Sand.ai发布新一代视频生成模型MAGI-2”；首句：“Sand.ai正式发布新一代视频生成模型MAGI-2，该模型在Artificial Analysis(AA)榜单中登顶全球第一。” | 2026-11-25 15:38:03，上证报中国证券网记者罗茂林 | 无 | 已找到（标题不夸，首句夸） |
| 新京报·贝壳财经（经 21 财经） | L4 | https://www.21jingji.com/article/20261125/herald/ad8f06b12497bceead6630ca98cf2682.html | 标题：“视频模型MAGI-2发布：1080P每秒0.5元，将尽快开源”；首句：“Sand.ai 正式发布新一代视频生成模型 MAGI-2，在 Artificial Analysis（AA）榜单中登顶全球第一。” | 2026年11月25日 16:46，新京报，记者罗亦丹 | `images/10-zh-xjb-beike-top1-v2.png`（只截文章本身） | 已找到（标题不夸，首句夸） |
| 36 氪（智东西稿） | L4 | https://eu.36kr.com/zh/p/4084870019197572（英文版 /en/p/4084870019197572） | 中文标题：“视频模型黑马杀到全球第一，30秒长镜头一次生成，价格打到Seedance 1/10”（页 `<title>` 另作“视频模型黑马登顶全球第一 30秒长镜头一键生成 价格低至Seedance十分之一”）；**正文限定**（中文原句见 `a-cn-36kr-zh.txt`；英文版：“MAGI-2 has reached No.1 in the world in the image-to-video (I2V) dimension”） | 2026-11-25 16:56，智东西（36氪转载） | 无 | 已找到（标题夸、正文限定图生视频） |
| 爱范儿 | L4 | https://www.ifanr.com/1691445 | 标题：“Seedance 2.5 级别，价格只要 1/10，我们实测了计划开源的 MAGI-2”；正文：“MAGI-2 在图生视频（I2V）维度超越 Seedance 2.5，位居全球第一” | 2026-11-25 13:21，爱范儿（含实测） | 无 | 已找到（限定了 I2V） |
| 搜狐号（含 AI 生成内容标注） | L5 | https://www.sohu.com/a/1106198897_121885030 | “该模型在ArtificialAnalysis（AA）榜单中荣登全球第一” | 2026-11-25 14:26，搜狐号“拾起我们的青春”，页内标“含AI生成内容” | 无 | 已找到 |
| 搜狐“ZPedia” | L5 | https://m.sohu.com/a/1106148839_122074763 | “在第三方评测平台 Artificial Analysis 最新更新的图生视频总榜（AA-Video-I2V v2.1）上，MAGI-2 以 1165 的 Elo 评分登顶全球第一，排在第二名 Seedance 2.5（1161 分）之前。”（限定了图生视频，读数 1165/1161） | 2026-11-25 13:15 | 无 | 已找到 |
| 英文快讯（KuCoin 转载，署 BlockBeats / ME News） | L5 | https://www.kucoin.com/news/flash/sand-ai-launches-480b-parameter-magi-2-video-model-achieves-global-ranking | 标题：“Sand.ai launches the 480B-parameter MAGI-2 video model, achieving global ranking.”；正文：“ranks first globally on the Artificial Analysis text-to-video leaderboard, surpassing Seedance 2.5”；同页 AI 摘要又写 “tops Artificial Analysis’s image-to-video leaderboard” | 页标 Release Time 11/25/2026 21:16:27（站点时区未标注）；正文称 “November 25 (UTC+8)”；末尾 “(Source: BlockBeats)” | `images/11-en-kucoin-t2v-claim.png` | 已找到（**写成文生视频榜第一，与 AA 实际相反：文生带音频第 7、无音频第 2**） |
| **第三方逐榜核对文（seeddance.io，竞品相关站，须披露利益）** | L5 | https://www.seeddance.io/blog/magi-2-review | “On the Artificial Analysis video leaderboards on November 25, 2026, MAGI-2 was first for image to video with audio (Elo 1165, with Seedance 2.5 at 1161) and first for image to video without audio (1151), second for text to video without audio (1144), and outside the top five for text to video with audio.” 结论句：“Independent leaderboards place it first for image to video and lower for text to video with audio.” | 站点自卖 Seedance、Kling 等多模型（“Black Friday −50%” 促销）；11-25 读数 | 无 | 已找到（**与本组四榜读数一致，是写对的稿**） |
| seeddance.io 指南页 | L5 | https://www.seeddance.io/blog/magi-2-guide ；https://www.seeddance.io/models/magi-2 | 引官方：“by Sand.ai's own numbers: its four-task average Elo on Artificial Analysis Video Arena v2.1 rose from 998.5 for the Preview to 1150.3.”；“The Elo figure is the average of four arena tasks … as reported by Sand.ai.” | 站点自卖 Seedance 等多模型（页内 “Black Friday” 促销） | 无 | 部分支持（该页如实转述官方 1150.3；**扫描所说“前三名 9 分以内，单次评测采样噪声可翻转排序”未在 seeddance.io 的 guide、models、review、vs 四页找到**） |
| “1158 排第二”的工具站 | — | 未找到 | 本组 `a-*.txt` 存档里 “1158” 只出现在 AA 官方页本身（Seedance 2.5 的 1158） | — | 无 | 未找到一手来源 |
| 公众号转述（AGI Hunt 摘要，AI 生成；opencli 取，`a-agihunt-info-*.txt`） | L5 | https://agihunt.info/p/1a21215c879883b8b9e9cb92331（curl 被 Cloudflare 质询拦，改用浏览器打开，未绕过质询） | “在 AA 榜单拿下图生视频与无声图生视频全球第一，无声文生视频全球第二，超越 Seedance 2.5。”；“最多 16 份多模态参考（10 张图、5 段视频、5 段音频）” | 页标 “9 小时前”，作者“卡尔的AI沃茨”（微信公众号，原文未取）；摘要是 AI 写的，参考份数 16 与官方 20 不符 | 无 | 已找到（L5；**有子榜限定的版本，与本组四榜读数吻合**） |
| 第三方对“AA 未收录 480B”的相反说法 | L5 | https://beatapi.io/blog/what-is-magi-2 | “Artificial Analysis's public AA-Video-T2V v2.1 and AA-Video-I2V v2.1 leaderboards do not include the 480B model yet; the newest Sand.ai entry there is still Preview (T2V 1039±12, I2V 967±15) … I2V's top five are Dreamina Seedance 2.5 (1161), Utopai X (1151), Boreal-H3 (1148), Dreamina Seedance 2.5 …” | 稿标 Published: Nov 25, 2026（无时刻）；该稿读到的 AA 表里 MAGI-2 尚未收录、Seedance 2.5 为 I2V 第 1（1161）；与本组读数（MAGI-2 已收录并第 1）矛盾，说明 AA 在 11-25 当天才把 480B 版加入榜单，Seedance 2.5 的 1161 是 MAGI-2 入榜前的读数（推断） | 无 | 已找到（仅作旁证：AA 收录时点晚于/紧随官方发布，具体时刻未知） |
| 第三方：独家与价格 | L5 | https://i10x.ai/news/magi-2-sand-ai-480b-video-model-price-per-second | “Sand.ai’s pricing page currently says account top-ups are not supported yet.”；“Sand.ai says 480 billion parameters; Higgsfield’s announcement says 400 billion.”；“MAGI-2 comes in at about 14% of ByteDance’s price, or roughly one seventh.”（自算） | i10X News，页内称基于 11-25 的刊例价 | 无 | 已找到（L5，线索） |

### A7　热度

| 项 | 结果 | 来源 / 状态 |
|---|---|---|
| Hacker News | `barking heat`（02:26:22）搜 “MAGI-2”“Sand.ai”“Seedance 2.5”（2026-11-23 起）：均命中 0 条；另用 Algolia 手查 “MAGI-2”“Sand.ai”“Sand AI video”：无相关帖（命中里只有无关评论） | `a-heat.json` |
| Reddit | `barking heat` 的搜索接口被重定向到登录页，两次均失败；`opencli reddit search "MAGI-2"` 返回 SECURITY_BLOCK（403，“blocked by network security”），未重试，未绕过 | 未取到 |
| X（官方发布帖） | @magiai_hq 的发布帖 11-24 18:09:28 UTC：fxtwitter 字段 likes 1351、retweets 210、replies 208、bookmarks 710、views 426105（02:2x 取）；第三方统计页 launchvideo.com 写 “178K views”，两者不一致（均只记录，不取舍） | `a-x-magiai-hq-2125290721003680155.json`（公开 JSON，不含抓取者信息） |
| 中文媒体是否属实 | 扫描所列：上证报·中国证券网（15:38 已确认）、新京报·贝壳财经（16:46 已确认，经 21 财经）、36 氪（智东西，16:56 已确认）、凤凰网（未直接取，只搜到 ifanr 转载的凤凰网稿，未存档）；另确认新浪科技 13:54、网易 14:30、投资界 13:18、爱范儿 13:21 | 见 A6 |

## 二、可能的吠点（线索；写成“说法 vs 证据”）

1. 说法：中文媒体“在 AA 榜单登顶全球第一”（新浪科技、上证报、新京报、网易等首句，无子榜限定）。证据：AA 视频板块有文生、图生、视频编辑三个排行页，每页再分带音频/无音频，AA 页把无音频版命名为 -Silent。本组读数：图生带音频第 1、图生无音频第 1；**文生带音频（AA 文生页默认视图）第 7，Elo 1150，榜首 Wan 3.0 1175**；文生无音频第 2（1144，HeyGen 1147）。（AA 四页，02:03–02:05）
2. 说法：“登顶第一”。证据：图生带音频榜 MAGI-2 1163（±14，区间 1149–1177）对 Seedance 2.5 1158（±14，1144–1172），差 5 分，两者 Range 都是 1–3；图生无音频榜 1153 对 Utopai X 1147（Range 都是 1–2）。AA 自己的排名区间表明第 1、2、3 名的排名区间重叠。（AA 页 Range 与 95% interval 列）
3. 说法：官方首发稿里的榜单依据。证据：Sand.ai 官方博客只写“4-task average Elo … 998.5 → 1150.3（相对 Preview +151.8）”，没写“第一”；“第一”不是官方英文稿的措辞。（官方博客 §摘要段，原文见 A2）
4. 说法：榜首数字稳定。证据：同一子榜读数随票数漂移：搜狐 ZPedia（11-25 13:15）写 1165/1161，本组 02:03 与 02:27 读 1163/1158；样本数 5,943（MAGI-2）对 7,384（Seedance 2.5）。（A1）
5. 说法：“价格约为 Seedance 2.5 的 1/10”（新京报、新浪等）。证据：MAGI-2 官方价页 0.08 USD/s（1080p，来自 platform.sand.ai 文档）；人民币 0.5 元/秒只见媒体稿；Seedance 2.5 官方（火山方舟）1080p、输入不含视频、16:9、5 秒的折合价为 3.74 元/秒（含输入视频的页签价格另算，最低 token 用量另有限制）；AA 页标 MAGI-2 $4.80/min 对 Seedance 2.5 $34.12/min。几处口径各有出处，未自算比例；各来源写出的比例有 1/10、“about 14%”“roughly one seventh”三种。
6. 说法：“API 已开放、可用”。证据：官方价页同页写 “Top-up is not supported for the time being.”；官方文档写 “Use a model enabled for your organization”；官方 X 帖说 “exclusively on @higgsfield … launch partner Higgsfield’s platform and via the Higgsfield API”。（A3）
7. 说法：“开源”（新浪/上证报写“将尽快开放权重”，Higgsfield 标题写 “Open Weights 400B”）。证据：HF 与 GitHub 官方组织下 11-26 02:27 仍只有 MAGI-2 Preview（114B，Apache-2.0）；官方原话是 “We will follow up soon with open-source releases”，无时间、无许可证；Higgsfield 正文也只说 “open weights coming soon”。
8. 说法：“480B 同期最大”。证据：官方博客写 480B/~28B 激活，官方 X 帖与 Higgsfield 稿写 400B；权重未发布，无法第三方核对。
9. 说法：Higgsfield 盲测“第二”可作旁证。证据：该榜只有六个模型，Higgsfield 是 MAGI-2 的独家发布合作方（有商业利益）；其盲测榜上 Seedance 2.5 1219 对 MAGI-2 1146，“Seedance 2.5 … won 61% of their matchups”（L3 合作方官方，利益相关）。
10. 说法：评测者口径。证据：AA 页 FAQ 写 “blind votes from our validated private evaluators”，每榜是 AA 自己的评测者人群；Category 默认 “Overall — All use cases and capabilities, each weighted equally”，另有用例与能力分项过滤，本组未取分项数据。
11. 说法：数独“自主解题”、12 切点命中是官方演示。证据：数独属“480B pre-trained checkpoint, PE off”，官方自述后训练后相关行为 “noticeably harder to elicit”；切点数字出自官方演示视频，媒体写“最大偏差 0.08 秒”（扫描所写 0.028 秒找不到）。
12. 说法：社区质疑（L6，仅线索）。HN 本期无相关帖、Reddit 查询被拦，**没有可摘的社区高赞质疑**。

## 三、夸大说法实例（汇总，详见 A6）

- 中文：新浪科技“AA榜全球第一”（13:54）、投资界“登顶全球第一”（13:18）、网易科技“登顶全球第一”（14:30）、36 氪/智东西“视频模型黑马杀到全球第一”（16:56；正文限定 I2V）。
- 英文：KuCoin 转载 BlockBeats 快讯，正文写“first globally on the Artificial Analysis text-to-video leaderboard”，AA 实际文生带音频第 7、无音频第 2。
- 限定较好的稿：爱范儿（“在图生视频（I2V）维度”）、ZPedia（“图生视频总榜”）。

## 四、扫描说法勘误

- “AA 视频榜分四个子榜（文生/图生 × 带音频/不带音频）”：基本成立，但 AA 页有三个排行页（文生、图生、视频编辑），“四个子榜”是文生、图生各自的带音频/无音频切换；本组按四个取证。
- 扫描“11-25 当日 I2V 为 MAGI-2 1165 对 Seedance 2.5 1161”：这组数字只在搜狐 ZPedia（13:15）与 dev.to 稿里；本组读数 1163/1158，属 Elo 漂移。
- 补：beatapi（Published: Nov 25, 2026，无时刻）读到的 AA 表里 MAGI-2 尚未收录，I2V 第 1 为 Seedance 2.5 1161；搜狐 ZPedia（13:15）与 dev.to 写的 “1161” 与之吻合，说明 1161 是 MAGI-2 入榜前后的读数，本组 02:03 起读到的是 1158（推断；AA 页无历史版本可核）。
- “另一工具站写 AA 给 1158、排第二”：1158 是 AA 页上 Seedance 2.5 的数字；没找到把 MAGI-2 写成 1158 的页面。
- “第三方 seeddance.io 提示前三名 9 分以内……”：已存档的 seeddance.io 四页（guide、models、review、vs）都没有这句；该站 review 页有逐榜核对，但写的是“first for image to video … second for text to video without audio … outside the top five for text to video with audio”。
- “12 切点平均误差 0.028 秒”：未找到；媒体稿写的是“12 个切点全部命中，最大偏差为 0.08 秒”。
- “1080P 刊例价 0.5 元/秒、官方”：Sand.ai 官方公开文档页只有 0.08 USD/s；0.5 元只见中文媒体稿。
- “英文快讯写便宜约 87%”：BlockBeats 原文未取到；已存档的 KuCoin 转载（署 BlockBeats）只写“one-tenth”；87% 未在 73 个存档文本里找到（第三方有 “about 14%” 即约 86% 的自算，不是同一说法）。
- “API 开放申请、首批限战略客户”：官方文档只给到 “Top-up is not supported for the time being.” 和 “model enabled for your organization”；“战略客户”出自第三方 beatapi（“Initial API onboarding favors strategic partners”，L5）。
- “官方 MAGI-2 专区未列参数、价格、榜单数字”：sand.ai 首页只有 “MAGI-2 possesses world-class video generation capabilities” 等；参数与榜单数字在 /blog/magi-2/，价格在 platform.sand.ai 文档页。
- “MAGI-2 Preview 114B”属实（HF 模型卡）。
- 官方发布日期：博客 November 24, 2026，官方 X 帖北京 11-25 02:09；中文媒体 11-25 13:15 起，不是“11-25 首发”的唯一依据。

## 五、对读者

- 能用吗：官方公开价页写 “Top-up is not supported for the time being.”，官方文档写调用需 “a model enabled for your organization”（platform.sand.ai/docs/model-pricing、/docs/get-started，02:21 读）。
- 价格：官方公开页 1080p 0.08 美元/秒（同上）；Higgsfield 上 “2.5 credits per second”（higgsfield.ai/announcing-magi-2）；人民币 0.5 元/秒只见媒体稿；Seedance 2.5 火山方舟 1080p、输入不含视频、16:9、5 秒折合 3.74 元/秒（docs.volcengine.com 模型价格页）。
- 开放范围：MAGI-2 在 Higgsfield 平台与其 API 上线（官方 X 帖：“Now available worldwide on our launch partner Higgsfield’s platform and via the Higgsfield API”）；Sand.ai 自家 API 在文档里写了价格但暂不支持充值。
- 申请/体验：17173（11-25 15:34，L4/L5，转述官方发布内容）写 API “已正式开放申请，首批优先面向战略合作客户”，企业与开发者到 sand.ai 申请接入，个人创作者可通过 LibTV 体验；官方文档页的口径是暂不支持充值。第三方 seeddance.io 称自己通过转售 API 取得，一条 30 秒 1080p 视频花费 $2.40、“took 85 minutes to arrive”（L5，一次性体验）。
- 开源：官方英文原句 “We will follow up soon with open-source releases”；无时间表、无许可证；HF 与 GitHub 官方组织目前只有 Preview（Apache-2.0，8 张 Hopper GPU）。
- 不受影响/不适用：想自己本地部署的个人用户目前拿不到 480B 权重；即便放出，官方 Preview 的模型卡就要 8 张 NVIDIA Hopper GPU。
- 榜单层面：读者问“国产视频模型是不是世界第一”——AA 页上 MAGI-2 在图生两个子榜第 1，在文生带音频榜第 7、无音频榜第 2；子榜内第 1 与第 2 的 Range 重叠。
