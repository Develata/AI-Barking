# C组补充：社区热度与实际误读抽查

## C10 社区热度

| 项目 | 原站URL | 抓取北京时间 | 发布时间（北京；原UTC） | 当前计数 | 存档 |
|---|---|---|---|---|---|
| Clef r/LocalLLaMA 原帖 | https://www.reddit.com/r/LocalLLaMA/comments/1wv4zzi/clef_open_weights_decision_model_by_cloudflare/ | 2026-10-02T21:39:04.437452+08:00 | 2026-10-02 01:04:38；2026-10-01T17:04:38.566Z | 372票，109评论 | d-c-clef-reddit.json |
| Jev HN 首发文章讨论 | https://news.ycombinator.com/item?id=49717558 | 2026-10-02 21:39:25+08:00 | 2026-09-16 03:25:03；2026-09-15T19:25:03Z | 1989分，520评论 | d-c-jev-hn.json |
| Jev Reddit 解释讨论（不是首发帖） | https://www.reddit.com/r/LocalLLaMA/comments/1wleg4w/what_is_jev_and_what_is_it_used_for/ | 2026-10-02T21:39:23.120411+08:00 | 2026-09-20 19:20:05；2026-09-20T11:20:05.448Z | 472票，369评论 | d-c-jev-reddit.json |

计数是单帖抓取快照，不是用户规模/独立验证数据。Reddit适配器此前失败，直接原帖DOM元数据读取成功；只存帖子和前20条可见评论，不含抓取者导航、头像或账户信息。HN公开Algolia API提供帖子计数；评论分数未据此推造。

### Clef评论（L6，仅选题线索）

- Hobofan94，55票，https://www.reddit.com/r/LocalLLaMA/comments/1wv4zzi/comment/pd9amx4/ ：“For a decision model that's pretty heavy. Most of the other leading ones are 4B or less.”
- milkipedia，48票，https://www.reddit.com/r/LocalLLaMA/comments/1wv4zzi/comment/pd9aypd/ ：“You're not wrong, but when I look at the index they chose to highlight in their blog, it's clear that bigger (relatively speaking) models are finding their place.

https://huggingface.co/spaces/multimodalart/jev-decision-index”
- Embarrassed_Soup_279，8票，https://www.reddit.com/r/LocalLLaMA/comments/1wv4zzi/comment/pd9w93o/ ：“fast yes, but gpu poor people (like me) dont want to use their precious memory for only a decision model as we cant fit anything else”

这些评论的有效疑问是本地部署占用内存；不是对Cloudflare分数造假的证据。55票评论把其他领先模型概括为4B及以下，应回原榜核对，不能作为模型规模事实。

## C11 中英文实际误读抽查

结论：本轮未找到足以作为“CF自建Jev Decision Index”“完整Clef是38.8ms”“全面胜过Jev”的明确中英文媒体反例，不为满足配额把正常报道硬标夸大。

| 原站 | 标题/日期/发布方 | 核对结果 | 存档 |
|---|---|---|---|
| https://aistify.com/cloudflare-clef-clef-flash-open-decision-models/ | Cloudflare Releases Clef and Clef-flash Open Decision Models；Daniel Mercer，AIstify；发布北京10-02 01:34:02（原UTC10-01 17:34:02），更新北京10-02 19:15:14 | 原文清楚区分38.8/209.3并写“Those are company-reported benchmark figures, rather than guaranteed response times for every deployment.”，不是误读实例 | d-c-media-aistify-browser.json / .txt |
| https://news.lavx.hu/zh-Hans/article/cloudflare-fa-bu-clef-jue-ce-mo-xing-ji-qiang-hua-xue-xi-wei-tiao-ping-tai | Cloudflare 发布 Clef 决策模型及强化学习微调平台；Elena Varga，LavX；北京10-02 00:49（原10-01 16:49UTC） | 逐模型列“Clef 的中位响应时间为209.3毫秒，Clef-flash 的中位响应时间为38.8毫秒”；未找到指定类型误读 | d-c-media-lavx-browser.json / .txt |
| https://ai-blog.cloud/tool/cloudflare-clef-open-source-decision-model/ | Cloudflare 开源 Clef 决策模型：把智能体的判断环节交给专用小模型；AI潮汐；2026-10-02，未给时区时刻 | 正文明确“但不是全面领先，有三项是输的”，并区分两个模型延迟。导语省略Laya例外且后文明确Laya5.8ms，是摘要限定不足的候选，不能说整篇宣称全胜 | d-c-media-ai-blog-browser.json / .txt |
| https://byteiota.com/cloudflare-clef-decision-models-agents/ | Cloudflare Clef: Open-Source Decision Models for Agents | 搜索片段带CF claim与独立测试区分；原站HTTP TLS失败，浏览器state空DOM报错，未取得正文，故不作为实际误读证据 | 失败详情d-c-capture-log.jsonl |

### 可进一步检查但尚不构成已证实错误的中文摘要

AI潮汐导语逐字：“27B 多模态模型只输出概率不写字，官方称在 43 项评测里比同类决策模型更快，同时上线自己的强化学习微调服务。”

对照Cloudflare原文的“except for Laya”限定，以及该稿后文自己列出Laya中位5.8ms，导语过度概括；但它署名“官方称”，且并未把38.8ms错给完整Clef。以“省略例外”描述更准确，不可升级为“独立证实全面胜过所有模型”。

## 搜索范围与缺口

本轮检索关键词包括 Cloudflare Clef 决策模型、Clef 自建、Clef 全面 Jev、Clef 13倍、Clef 碾压、Clef independent benchmark、Clef fully open source。抽查实际原站3篇中英正文及1次失败；不是全网穷尽。Jev Reddit首发帖尚未定位，以HN首发帖和Reddit解释讨论分别展示，不混称。没有截图，没有修改C组现有文件。
