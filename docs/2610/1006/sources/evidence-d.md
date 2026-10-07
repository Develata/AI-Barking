# 1006 D组取证清单

只取证，不是发布稿。抓取发生于北京时间2026-10-07凌晨（原本机时间2026-10-06 EDT）；本文中的“本次”指本次快照，不回填为10月6日白天已知。基线 `85f6bb9ce45dc6084d37d30b4c8e3a8113d0f353`。仅新增D组文件，无提交、推送、删除；A/B组同期新增文件不归本组。

状态“已找到”只表示找到相应原文，不代表媒体匿名信源已获公司证实。L4仍必须写“据某媒体报道”。逐次抓取时间和失败见 [capture-log-d.md](capture-log-d.md)，文件清单见 [d-manifest.md](d-manifest.md)。`.html` 是匿名HTTP响应原件；浏览器可见文字以 `*-browser.txt` 为准，不能把响应中的隐藏正文视为付费墙授权。

## 逐条证据

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| D1-1：10/4以后OpenAI新全局/banked重置或新到期日 | L1/L2待取得；监控仅L5 | https://help.openai.com/en/articles/20001498-how-banked-codex-resets-work ；https://x.com/thsottiaux ；https://x.com/OpenAI | 未取得新的官方公告原句。 | 无，不生成25 | 未发现；不是确认没有。三监控站未提供新事件；OpenCLI断连，Twitter搜索两次失败；帮助页HTTP403/浏览器空正文。已发10/3重置和28天承诺不重复收录。 | 未找到一手来源 |
| D1-2：10/4以后Anthropic新全局/banked重置或新到期日 | L1待取得；监控仅L5 | https://x.com/ClaudeDevs ；https://x.com/claudeai ；https://support.claude.com/en/articles/17007452-what-is-a-limit-reset | “Limit resets are given occasionally to eligible plans, and set your usage limits back to full when you choose to use one.” | 无，不生成26 | 未发现新的官方重置公告；帮助页是一般规则，不能拿来证明新发放。监控仍列9/22 banked、10/22到期，均已发过；官方社交搜索失败。 | 未找到一手来源 |
| D1-3：Claude重置的一般限定 | L1 | https://support.claude.com/en/articles/17007452-what-is-a-limit-reset | “Your weekly limits still reset on their usual day and time.”；“If your limit reset has an expiry, the expiry date will be shown in Settings > Usage.” | 无 | `d-claude-reset-help.txt`；第二句HTML实体已还原为`>`。不推断新到期日/新套餐；当前页仅写Updated over 2 weeks ago。 | 已找到 |
| D2-1：Mistral Large 4是公开预览，月底才放权重 | L1 | https://mistral.ai/news/mistral-large-4/ | “Today, we’re launching a public preview of Mistral Large 4.”；“Weights drop end of this month.” | ../images/27-d-mistral-context.png；../images/27-d-mistral-opening.png | `d-mistral-blog.txt`、`d-mistral-browser.txt`；页面2026-10-06，官方未给时刻/时区。月底指2026年10月底，官方该页未给10/27或10/31具体日。 | 已找到 |
| D2-2：1T与1.05T、49B激活参数 | L1 | https://mistral.ai/news/mistral-large-4/ ；https://docs.mistral.ai/models/mistral-large-4-0 | 博客：“ML4 is a 1 trillion-parameter natively multimodal model with 49 billion active parameters.”；文档：“It features 49B active parameters and 1.05T total parameters, and a 1.6B vision encoder.” | ../images/27-d-mistral-opening.png；../images/28-d-mistral-prices.png | 两种总量表述同时保留；49B一致。不能据此自行确定1.05T是否包含1.6B视觉编码器。文档日期2026-10-06，官方未给时刻。 | 已找到 |
| D2-3：从零训练用3,800块GPU | L1 | https://mistral.ai/news/mistral-large-4/ | “ML4 was trained from scratch on 3,800 NVIDIA Grace Blackwell GPUs in Mistral’s own datacenters in Europe.” | ../images/27-d-mistral-gpus-clean.png | 博客另写“At our current scale (3k GPUs), a single training run produces roughly 33 billion tokens per day”，位置是Reinforcement learning at scale，不是同一数字口径。 | 已找到 |
| D2-4：reduced moderation与expanded cyber capabilities适用谁 | L1 | https://mistral.ai/news/mistral-large-4/ | “Until then, we are red-teaming the model in real-world settings with cybersecurity leaders, vetted partners, and state authorities, who will access the same model with reduced moderation and expanded cyber capabilities.” | ../images/27-d-mistral-opening.png | 限定于该组红队合作对象，不能写成公众API全部取消审核。这里只存原文限定，不选取涉外国政府题材作速览。 | 已找到 |
| D2-5：博客与文档/价格页两套价格 | L1 | https://mistral.ai/news/mistral-large-4/ ；https://docs.mistral.ai/models/mistral-large-4-0 ；https://docs.mistral.ai/inference/pricing | 博客卡片：“Input (/M tokens)” “$1.36”；“Output (/M tokens)” “$4.18”。文档价格依序为1.36→0.68、0.14→0.07、4.18→2.09；pricing行“Mistral Large 4 ↗ $0.68 $0.07 $2.09”。 | ../images/28-d-mistral-prices.png | 单位美元/百万token，输入/缓存输入/输出。HTTP原文旧价有line-through；700 CSS窄版只展示新价、三个/M TOKENS，截图本身不显示旧价/三个角色标签，须结合文档HTML与价格页。 | 已找到 |
| D2-6：低价是否预览期折扣、持续多久 | L1待取得 | https://docs.mistral.ai/inference/pricing ；https://docs.mistral.ai/models/mistral-large-4-0 ；https://x.com/MistralAI/status/2107456586813730854 | 没有找到写明折扣原因或截止日的官方原句。 | 无 | 两套价格正好相差50%只是算术；不能升级为“预览期半价直到月底”。官方帖oEmbed取得截断开头、无价格；Twitter全文读取失败。 | 部分支持 |
| D2-7：AA快照38、64/225、Proprietary、116.1 tok/s、1.36/4.18 | L3 | https://artificialanalysis.ai/models/mistral-large-4 | “Proprietary model”；“Mistral Large 4 Preview scores 38 on the Artificial Analysis Intelligence Index”；“No, Mistral Large 4 Preview is proprietary. The model weights are not publicly available.” | ../images/29-d-aa-summary.png | `d-aa.txt`、`d-aa-browser.txt`；摘要显示#64/225、116.1 Output tokens per second、In $1.36 Out $4.18。是抓取时榜单与Mistral API测量，非永久排名/所有平台速度。 | 已找到 |
| D3-1：彭博称接近筹得至少800亿元，可能接近1000亿元 | L4 | https://www.bloomberg.com/news/articles/2026-10-06/deepseek-to-raise-at-least-12-billion-in-tencent-backed-funding | “DeepSeek is close to securing at least 80 billion yuan ($12 billion) in its latest round of funding”；“Based on term sheets signed, the final tally could approach 100 billion yuan, the people added.” | 无，30–31未取得路透正文，不以别家图替代 | `d-bloomberg.txt`公开前两段；引用是连续摘句非完整句。北京时间10/6 13:00发布（05:00 UTC）、19:41更新（11:41 UTC）。不是已完成融资。HTTP后续无正文；浏览器为人机验证页，未挑战/绕墙。 | 已找到 |
| D3-2：路透的more than 80 billion、11.93B美元、7月目标估值5000亿元等原句 | L4待取得 | https://www.reuters.com/world/asia-pacific/deepseek-raise-least-12-billion-tencent-backed-funding-bloomberg-news-reports-2026-10-06/ | 未取得路透原文；派工单及转载搜索摘要不作为逐字依据。 | 无 | HTTP401、web工具不可打开、匿名Chrome正文0字符。未访问转载站替代。发布/更新时刻未核到。 | 未找到一手来源 |
| D3-3：融资额与估值区分；补充CNBC原始采访 | L4 | https://www.cnbc.com/2026/10/06/deepseek-funding-round.html | “DeepSeek is considering expanding its latest funding round to as much as 100 billion yuan ($14.9 billion)”；“DeepSeek is seeking a valuation of about 500 billion yuan, or $75 billion, the people said.” | 无 | `d-cnbc-deepseek.txt`；分别是考虑中的融资上限1000亿元、寻求估值约5000亿元，不是同一指标。CNBC引知情者；不能冒充路透证据。北京时间10/6 20:59（08:59 AM EDT）。 | 已找到 |
| D3-4：腾讯、宁德为本轮大额出资方 | L4 | https://www.bloomberg.com/news/articles/2026-10-06/deepseek-to-raise-at-least-12-billion-in-tencent-backed-funding | “Battery pioneer Contemporary Amperex Technology Co. Ltd. and WeChat-operator Tencent Holdings Ltd. have committed among the largest amounts in the current financing, which closes soon, people familiar with the matter said.” | 无 | 彭博公开第2段；among the largest并非确定只有两家或已经完成交割。未给各自本轮金额。 | 已找到 |
| D3-5：腾讯约100亿元、宁德约50亿元属于本轮 | L5（TNW回溯线索） | https://thenextweb.com/news/deepseek-12bn-funding-round-tencent-catl | “Founder Liang Wenfeng was the largest investor in that first round, putting in about 20 billion yuan of his own money. Tencent added roughly 10 billion yuan and CATL about 5 billion yuan” | 无 | `d-tnw-deepseek.txt`明确that first round；上段是first outside funding earlier this year。这些数额属于首轮叙述，不能放入这轮融资。两数未回溯到彭博/路透原句。北京时间10/6 16:29:35（08:29:35 UTC）。 | 与说法不符 |
| D3-6：DeepSeek/腾讯/宁德是否回应 | L4（CNBC）；L1未找到 | https://www.cnbc.com/2026/10/06/deepseek-funding-round.html ；https://www.deepseek.com/ | “DeepSeek, CATL, Geely Auto, Monolith Management and Loyal Valley Capital did not respond to CNBC's requests for comment.” | 无 | 只支持CNBC发稿时DeepSeek/宁德等未回应CNBC，不支持腾讯未回应，也不代表从未回应。彭博公开段落未含回应；路透未取得。官方域名检索未找到公告；api-docs/news与updates返回入门页，已否决为新闻栏目采集。 | 部分支持 |
| D4-1：Liquid d1的19–200倍成本说法 | L1自报 | https://www.liquid.ai/blog/d1-decision-model | “We tested d1 against GPT-6.1 Sol and Claude Opus 5.5 on six real applications”；“It costs 19x to 200x less than both models and answers significantly faster on every task.” | ../images/32-d-liquid-method-clean.png（方法限定，不是成本结果图） | 仅六个结构化决策应用、自报；不是所有LLM任务便宜200倍。页面2026-10-05，官方未给时刻/时区。原文是19x to 200x，不是数学符号×。 | 已找到 |
| D4-2：Liquid methodology只跑一次 | L1自报 | https://www.liquid.ai/blog/d1-decision-model | “Methodology. We ran each application once per model on October 5, 2026, with the d1 Playground's comparison script.”；“Costs use list prices, without prompt-cache discounts.” | ../images/32-d-liquid-method-clean.png | 默认reasoning设置；聊天模型JSON作答；最多8个并发；d1按$0.04/百万输入token，非独立重复测量。 | 已找到 |
| D4-3：d1输出类型及不生成token | L1 | https://www.liquid.ai/blog/d1-decision-model | “reads them in one forward pass, and returns the probabilities, without generating any tokens.”；“Noul: a yes/no question, answered with a probability between 0 and 1.”；“Choice: pick one label among many, answered with a probability per label.”；“Score: a position on a scale, weighted by the probability of each level.” | 无 | 逐字取浏览器文本，类型Noul按原文，不擅改Bool；无输出token不等于没有输入成本/不输出结果。 | 已找到 |
| D4-4：d1开放权重表述 | L1 | https://www.liquid.ai/blog/d1-decision-model | “we plan to release open weights for upcoming models on Hugging Face soon.” | 无 | 是upcoming models的计划，未承诺当前d1权重已开放；未给具体日期。 | 已找到 |
| D4-5：Dust不声称当前算力效率足以替代backprop | L1研究方自述 | https://qlabs.sh/research/dust | “We do not attempt to make it compute-efficient enough to replace backprop today.” | 无 | Introduction末；未独立复现。页标October 2026，官方未给具体日/时刻；HN发布时间不能代替研究发布时间。 | 已找到 |
| D4-6：Dust最大测试模型规模243M | L1研究方自述 | https://qlabs.sh/research/dust | “We test this directly by training four sizes, 2M, 7M, 38M and 243M parameters” | 无 | 4.1 Overparameterization，连续短摘；实验固定10M tokens。Figure 4同列2.0M/7.3M/38M/243M；本文正文近似写7M。243M是该规模实验最大值，1B tokens不是1B参数。 | 已找到 |

## 可能的吠点

- Mistral当前是API预览；open-weight是计划/定位，AA当前仍标Proprietary，二者不能合成“权重已公开”。来源D2-1、D2-7。
- Mistral受限合作方获得降低审核与扩展网安能力的版本，不能外推到公众API；来源D2-4。
- Mistral两套价都真实存在；来源没有给出半价期限，不能由月底放权重推导月底结束折扣；来源D2-5、D2-6。
- Mistral文档上下文写1M，AA摘要写524k；仅记录来源差异，未额外测试服务实际上限。来源`d-mistral-doc.txt`与`d-aa.txt`。
- DeepSeek的800亿/1000亿是融资额，5000亿是寻求估值；彭博前两段里的500亿元是最初融资目标，三者不要混。来源D3-1、D3-3及`d-bloomberg.txt`。
- 腾讯100亿元、宁德50亿元是TNW首轮叙述，不是最新这轮的已核实投资额。来源D3-5。
- Liquid成本倍数以六个应用、每模型每应用一次、列表价格且不计缓存折扣为条件；开放权重是未来模型计划。来源D4-1至D4-4。
- Dust与反向传播竞争的损失表现不等于更省算力，最大参数规模与最大token规模也不能混。来源D4-5、D4-6。

### 社区质疑，仅L6线索

已扫HN公开原帖/API与Reddit公开读取结果；没有拿帖子总分冒充评论分数。HN评论API的points为null，Reddit读取未暴露分数，所以**无法验证所选评论为高赞**。评论排序不等于赞数。

| 事件 | 评论链接、原句/线索 | 得分与限制 |
|---|---|---|
| Mistral | https://www.reddit.com/r/MistralAI/comments/1wz22ti/comment/pe7q06a/ ：“This says nothing... there's basically no open weights model from US or Europe except Mistral Medium 3.5.” 质疑宣传比较集很窄；后半的“只有Medium 3.5”未经核实，不能当事实。 | 评论分数未显示；Best列表来自web工具缓存，不是实时登录态采样；永久链接单独打开cache miss。摘录见`d-community.md`。 |
| Mistral | https://news.ycombinator.com/item?id=49978204 ：“Looks like it's about a year behind still. i.e. its intelligence is behind models from roughly a year ago.” 并链接Vals榜。 | points=null；仅作查榜线索，没把“一年”当事实。`d-hn-mistral.html`是API JSON原件。其主帖查询快照863分/515评论。 |
| Dust | https://news.ycombinator.com/item?id=49971488 ：“It sounds like this is less computationally efficient than backprop, but more easily parallelizable. Is that fair?” | points=null；仅为问题，不能当已证实并行优势。`d-hn-dust-api.html`；主帖API248分。 |
| Dust | https://news.ycombinator.com/item?id=49974026 ：评论质疑无梯度方法在平滑目标上的效率，并列三项参考材料。 | points=null；未审查评论引用论文的数学论证；不据评论断言Dust无用。 |
| Liquid | HN查询找到 https://news.ycombinator.com/item?id=49967491 （2分、0评论），另一旧d1条目3分、0评论。 | 没有找到可摘的有依据评论；不凑质疑。`d-hn-liquid-query.html`。 |
| DeepSeek | https://news.ycombinator.com/item?id=49974897 为5分、1评论；唯一评论问电池厂商为何投资。 | 该评论是无证据猜测，不收为有理有据质疑；高赞质疑未找到。 |
| 重置 | 已有28天承诺讨论属于1005已收；本轮未找到新的官方重置事件。 | 不把社区催重置或例行个人周期当全局重置。 |

## 夸大说法实例

| 发布方/发布时间 | 原站URL及逐字原句 | 原文核对与边界 |
|---|---|---|
| 赢政天下，署名News Factory；北京时间2026-10-06 15:45:19（JSON-LD `+08:00`）；本次页面修改时间10/7 00:50:23 | https://www.winzheng.com/article/deepseek-12b-pre-ipo-round-tencent-catl-2027-ipo ：“DeepSeek完成120亿美元Pre-IPO融资 腾讯宁德时代领投2027科创板上市” | `d-overclaim-zh.txt`、`.html`。标题“完成”与它自己正文“正接近完成”及彭博“is close to securing”不一致。本轮只判融资完成时态，不扩展核IPO地点。 |
| 英文实例 | 查过TNW两篇、英文新闻搜索与HN相关线索，**未找到足够支持判定为夸大的英文原站实例**。 | TNW Mistral标题现在是“Europe’s Mistral launches Large 4 to challenge China’s lead in open AI models”，正文保留preview与未来权重日期，不因URL含open-weight就判夸大。 |
| D1/D2/D4其他实例 | 未找到足够证据的确定实例。 | 不把“可能有人这么写”当实际标题；不把报道与官方的3,800/约4,000精度差自动叫夸大。 |

## 扫描说法勘误

| 扫描说法 | 是否被本次存档证实 | 处理 |
|---|---|---|
| Mistral 1T vs 1.05T | 两者均有官方原文 | 博客1T、文档1.05T并列，不擅选一个称另一个错误。 |
| GPU 3,800 vs 4,000 vs 3k | 官方从零训练3,800；TNW约4,000；官方RL段3k | 3k不是从零训练GPU数；约4,000是媒体近似表述，不冒充官方逐字。 |
| 两套价格 | 博客1.36/4.18，文档及价格页0.68/0.07/2.09；旧价划线在HTML可核 | 折扣原因、期限未找到，勿写“预览期到月底”。 |
| AA标Proprietary | 已证实 | 当前还明确权重不公开；38/64名/116.1也已核。 |
| DeepSeek至少800亿元/可能1000亿元 | 彭博前两段支持“接近取得”“could approach” | 不能写已融资到账；路透11.93B美元口径未取得。 |
| DeepSeek估值5000亿元 | CNBC原始采访支持“seeking a valuation of about 500 billion yuan” | 路透“7月启动”与相应措辞未核；CNBC来源单列，不能换署路透。 |
| 腾讯100亿元、宁德50亿元 | TNW有两数，但明确说首轮 | 若扫描意指本轮，则与存档不符；上游彭博/路透这两个金额未核得。 |
| 公司回应 | CNBC记DeepSeek与CATL未回应CNBC | 腾讯回应状态、彭博回应段、路透回应段未取得；官网公告未找到，不写“公司从未回应”。 |

## 发布时刻与本次假设

- Mistral博客与文档：2026-10-06，官方未给时刻/时区，不能强行转换为北京时间某一小时；官方X oEmbed只给10/6且正文截断，未用ID推算精确发帖时刻。
- AA：模型发布日期2026-10-06，无发布时刻；榜单数据是本次抓取时点，非发稿时间。
- 彭博：北京时间10/6 13:00发布、19:41更新（原05:00/11:41 UTC）。
- 路透：时刻未核到。
- CNBC：北京时间10/6 20:59（原08:59 AM EDT）。TNW DeepSeek：北京时间10/6 16:29:35（08:29:35 UTC）；Exa摘要另列08:56:50.127Z，与原站不符，本表采用原站并保留冲突记录。
- Liquid：2026-10-05，官方未给时刻/时区。Dust：October 2026，官方未给具体日/时刻。HN Dust发表于北京时间10/6 05:15:07（10/5 21:15:07 UTC），仅是社区提交时间。
- 范围假设：用北京时间10/4起为重置调查起点；CLI日期过滤只是有界搜索，不保证覆盖时区边缘；没有找到新事件则不制作25/26。未取到路透则不制作30/31；不存在的文件不填截图列。
- 权限假设：公开原站HTTP、无登录态Chrome、既有Twitter显式配置的只读请求属于此次取证授权；不登录、不接受非必要Cookie、不付费、不绕墙。临时Chrome profile由浏览器在系统临时目录维护，未复制到仓库，未主动删除。
