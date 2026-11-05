# 1104 配图说明

标题：花121万美元，AI挖出948个漏洞？

本期期号1104；新闻日期为2026-11-04（美国）至11-05（北京），卡片日期写11月5日；取证与截图在北京 11-05 17:20 至约 18:45（批注图来源均写 2026-11-05 存档）。主帖顺序：OpenAI 948 个内核漏洞 → Kimi-K3.1-Pro 放出权重。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共16张（封面＋15），公众号不插两张事件卡与两张吠点卡（共11张＋封面）。

## 封面

封面抽签：画风=东方传统；底色=深色；标题=压在下方；看板娘=半身；动作=扮演

- 抽签种子 1794698029151981582（排除：画风 立体模型/粉笔涂鸦/绘本手绘；底色 亮色；标题 手写字；看板娘 探头；动作 动手/查验）。
- 生成：Codex CLI（gpt-6-astra，medium）内置图像生成，附看板娘参考图；A、B 两组各出竖版、横版各 1 张，共 4 张候选；选中 A 后，因第三轮反向核验指出副标题「补丁只发了数百份」把“报告和补丁”缩成“补丁”，用同一 A 组提示词去掉副标题重出一次（竖版、横版各 1 张，每种尺寸累计 3 张）。
- 选中 A：浮世绘木版画深夜场景，一座古城墙（系统内核）被挖出许多发光的洞，一只披机械甲片的大鼹鼠（挖漏洞的 AI）从墙根钻出，墙上只零星贴着几块膏药补丁，呼应“挖出948个漏洞、补上的不多”；看板娘半身扮打更人，提灯笼照向墙洞。主标题毛笔书法白字压在下部。发布用的是去掉副标题的重出版。
- 落选 B（实物摄影、满幅场景、顶部横排、全身、旁观）：玩具挖掘机挖瑞士奶酪、看板娘坐木块吃西瓜旁观。画面完成度可以，但看板娘没有按提示词做成手办质感，与场景的实拍感割裂；主题寓意（奶酪洞）不如 A 的“挖墙洞”直接。
- 逐字核对（重出版竖版、横版都放大看过）：主标题「AI挖出948个漏洞？」、胸牌「AI 吠点」，无副标题、无多余文字、logo 或真人。主标题为正文标题的后一分句。
- 看板娘：两只正常人手（一手提灯笼、一手张开，表情惊呼），两只垂耳，一条鲸尾，二次元 Q 版画风；与前 3 期（1101 粉笔、1102 绘本、1103 立体模型）不像同一系列。
- 横版居中 1:1 裁切（x 552–1370）：主标题与看板娘完整在框内。

A 组提示词（竖版；横版另加“横构图：城墙横贯画面，鼹鼠在左，看板娘在中央偏右，标题在中央下部”及横版尺寸要求）：

> 画面：浮世绘木版画风格的深夜场景，底色是深靛蓝夜空与墨黑。主体是一座巨大的古代城墙（象征系统内核），墙上被挖出许多发着橙黄微光的小洞，一只身披机械甲片、眼睛发光的大鼹鼠（象征挖漏洞的 AI 模型）正从墙根钻出，爪下堆着碎砖。城墙前只零星贴着几块白色膏药补丁，大部分洞还空着。看板娘半身（腰部以上）出镜，换装扮演提着灯笼的打更人（头戴小斗笠、肩披短蓑衣，但仍穿女仆装），一手举灯笼照向墙洞，一脸惊讶。主标题用粗壮的毛笔书法白字，压在画面下部、横跨主体下缘；副标题用较小的书法字放在主标题下方。浮世绘的粗轮廓线、平涂色块、木纹肌理，留出适度空白，不要拥挤。
>
> （硬性要求按 `templates/codex-cover.md` 逐字照抄：主标题「AI挖出948个漏洞？」；初版另有副标题「补丁只发了数百份」，重出版删去副标题及其位置描述。）

B 组提示词（落选）：

> 画面：桌面实物微距摆拍，满幅场景、没有纯色底。主体是一大块布满孔洞的瑞士奶酪（象征布满漏洞的系统），一台黄色玩具小挖掘机正把奶酪挖出更多的洞，挖斗里装着碎奶酪屑；奶酪表面只零星贴着三四片小小的白色创可贴，大部分洞没贴。看板娘以 Q 版手办的形式全身入画，坐在画面一角的一叠小木块上，捧着一片西瓜边吃边旁观，表情是看热闹的坏笑。暖色台灯光、浅景深、背景是虚化的木桌和工作台。主标题用粗黑无衬线大字横排在画面顶部，副标题小一号放在主标题下方。
>
> （硬性要求同上。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字摘自正文事实段 |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋Images 2.6、Conductor、Astra、Ripple、Claude 结构 |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（OpenAI），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（OpenAI），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-oa-948-hero.png | 第一条事实段 | OpenAI 原文标题与导语：“validated and generated patches for 948 local privilege escalation vulnerabilities … $1.2 million” |
| 6 | publish/06-oa-patches-sent.png | 第一条吠点① | OpenAI 原文补丁生成一节：“already sent out hundreds of reports and patches for critical vulnerabilities” |
| 7 | publish/07-oa-maintainers.png | 第一条吠点② | OpenAI 原文：10天结束时 61 个已在上游修好、43 个有待合入补丁、299 个在公开报告讨论 |
| 8 | publish/08-oa-wrong-0day.png | 第一条吠点③ | OpenAI 原文“Using the wrong 0-day”：早期一轮 200 个成功里 20 个借了另一个 0-day；此后加独立检查 |
| 9 | publish/09-event-2.png | 第二条事实段 | 事件卡（Kimi）；公众号不插 |
| 10 | publish/10-bark-2.png | 第二条吠点 | 吠点卡（Kimi）；公众号不插 |
| 11 | publish/11-kimi-promise.png | 第二条事实段 | Kimi 官方博客：“Full model weights will be released by Nov 4, 2026.” |
| 12 | publish/12-kimi-license-a.png | 第二条吠点① | Hugging Face 上的 LICENSE 首段授权与第 2 条：经营模型服务、被授权方及关联方连续12个月总收入超2000万美元须另签协议 |
| 13 | publish/13-kimi-license-b.png | 第二条吠点① | LICENSE 第 3、4 条：月活超1亿或月收入超2000万美元的商业产品或服务须显著标注；内部使用与经官方产品或认证推理伙伴使用除外 |
| 14 | publish/14-kimi-vllm.png | 第二条吠点② | vLLM 部署说明：“TP8 on two GB300 trays or one B300 node, or one MI355X node”；“~1.56 TB on disk” |
| 15 | publish/15-kimi-aa-v5.png | 第二条吠点③ | Artificial Analysis 10月27日 v5 发布文：Claude Opus 5.5（max）58分居首；开放权重差距从3分拉大到12分，Kimi 46分 |

## 备用图

| 文件 | 用途 |
|---|---|
| oa-948-hero-raw.png、oa-patches-sent-raw.png、oa-maintainers-raw.png、oa-wrong-0day-raw.png、kimi-promise-raw.png、kimi-license-a-raw.png、kimi-license-b-raw.png、kimi-vllm-raw.png、kimi-aa-v5-raw.png | 批注底图（kimi-license-a 为 LICENSE 首段与第 2 条拼接，kimi-license-b 为第 3、4 条；kimi-vllm 为配方页页首与 Variant 两段拼接） |
| oa-other-contrib-raw.png | 原拟批注底图（摘要“100多个由其他贡献者修好”），首轮读者稿检查后正文删去该句，未用 |
| 01-oa-hero.png、02-oa-intro-upcoming-model.png、03-oa-summary-findings.png、03a-oa-summary-reflections.png、04-oa-cost-upper-bound.png、04a-oa-funnel-chart.png | OpenAI 原文页首、导语（“upcoming cyber model”）、摘要、反思、成本上限句、官方漏斗图 |
| 05-oa-validation-1724.png、06-oa-exploit-1169-948.png、06a-oa-exploit-cost-success-fail.png、07-oa-validation-breakdown-chart.png、07a-oa-validation-cost-fp-tp.png | 验证阶段 1,724、利用阶段 1,169→948、成本与官方饼图 |
| 08-oa-maintainers-61-43-299.png、09-oa-stronger-models-166-948.png、09a-oa-novel-techniques.png、10-oa-wrong-0day-200-20.png、11-oa-patch-generation-reports.png | 修补进度、7 月与 10 月两轮对比、“10 种新技术”、用错 0-day、补丁生成 |
| 12-hn-main-and-author-reply.png | Hacker News 主帖与自称作者的答疑 |
| 12a-qbitai-948-patch-suggestions.png | 夸大实例：量子位“OpenAI提出了948个补丁建议”“AI用10天，干了人类差不多50年的活”（北京 11-05 09:54） |
| 13-hf-model-head.png、14-modelcard-intro.png、14a-modelcard-summary.png、15a-hf-license-text.png、16-hf-files-size.png | HF 模型页、模型卡（“3T-class”与参数表 2.8T）、LICENSE 全文、Files 页 1.56 TB |
| 17-vllm-recipe-hardware.png | vLLM 配方页全图（含硬件选择与 MXFP4 1874 GB 变体） |
| 18-aa-model-summary.png、18a-aa-model-comparison.png、19-aa-faq-open-weights-best.png、20-aa-v5-open-weights-gap.png | AA 模型页（仍标 Proprietary）、对比、榜单 FAQ（开放权重榜首 DeepSeek V4.1 Flash 1017 max 33 分）、v5 发布文 |
| 21-x-weights-post.png、22-hn-main-title.png | 月之暗面放权重官方帖（北京 11-05 00:00）、HN 主帖（23:30） |
| 23a-cocoloop-5-6tb-title.png | 夸大实例：Cocoloop 标题“文件大到 5.6TB”（BF16 估算） |
| 24-kimi-blog-weights-promise.png | Kimi 博客承诺原句（批注底图来源） |
