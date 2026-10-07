# D组社区原始线索与搜索范围

本次采集北京时间2026-10-07 00:41起；精确HTTP/API时刻在d-http-records.jsonl。Web搜索与Exa时刻下列记分钟级窗口，不能视为结果页面发布时间。

## Reddit公开摘录

00:55–00:56，OpenCLI桥不可用后，web工具读取原站 https://www.reddit.com/r/MistralAI/comments/1wz22ti/its_here_le_chonk_large_4/ ，未使用代理域名或登录态。返回的是缓存内容，仍显示15m ago，与当前采集时间不一致，因此不换算绝对时间、不当实时票数。

评论作者ClaudeMMM；链接 https://www.reddit.com/r/MistralAI/comments/1wz22ti/comment/pe7q06a/ （来自原页评论链接；直接打开cache miss）。原文：

> It is the best open weights model from US or Europe on aggregated benchmarks.
> This says nothing... there's basically no open weights model from US or Europe except Mistral Medium 3.5.

评论分数未暴露，无法验证“高赞”。没有保存导航、抓取者身份、头像或登录菜单。仅把比较集合的质疑作L6线索，未采纳“只有Medium 3.5”的事实断言。

## HN

- Mistral：Algolia公开API，搜索`Mistral Large 4`；原帖49977979、49978116等；搜索原件d-hn-query.html（实际为JSON响应），评论树d-hn-mistral.html。筛选有指标或比较范围依据的质疑，分数为null就写未公开。
- Dust：原帖49970871与API全文分别d-hn-dust.html、d-hn-dust-api.html；未拿帖子248分替代评论分数。
- Liquid：搜索`liquid d1`，d-hn-liquid-query.html；本次10/5发布帖49967491仅2分0评论，未找到可采的质疑。
- DeepSeek：搜索`DeepSeek 12 billion`，d-hn-deepseek-query.html、d-hn-deepseek.html；5分1评论。唯一评论猜CATL卖电池，没提供依据，不收作质疑。

## 搜索与失败边界

- 00:43–00:58，用web搜索：Mistral Large 4 pricing/preview/discount、原站新闻/文档；DeepSeek October 6 Reuters/Bloomberg/融资；Dust backpropagation；Liquid d1；官方域名DeepSeek/Tencent/CATL公告；中英文夸大标题；HN/Reddit事件讨论。搜索结果只发现线索，证据表以原站存档为准。
- 00:44–00:55，重置搜索包含`site:x.com/thsottiaux reset since:2026-10-04`、`site:x.com/ClaudeDevs reset October 2026`、`Codex reset October 6, 2026`、`Claude reset October 5, 2026`、`ClaudeDevs claudeai reset October 6 2026`。搜索引擎有无关旧结果，不以无结果证明无新帖。
- 00:52–00:56，Exa搜索Tibo reset October 6、Reuters DeepSeek、TNW DeepSeek；TNW精确URL由Exa发现，随后原站存档。Exa给TNW发布时间08:56:50.127Z，原站JSON-LD08:29:35+00:00；採原站且记录出入。
- OpenCLI browser 1006-d不可用；Twitter CLI使用既有显式Cookie配置，仅把凭据放子进程环境、不打印/落盘。两次OpenAI重置搜索、一次Claude搜索、一次Mistral主帖读取均失败于ClientTransaction初始化。未升级或修改环境/其他组文件。
- Mistral官方oEmbed成功，但长帖截断；不据截断部分断言全文没有折扣。官方源URL https://x.com/MistralAI/status/2107456586813730854 ，API请求参数是资源标识所需，不是营销跟踪参数。
- 最终英文夸大实例未找到；中文赢政天下标题“完成”已有原站存档，与其正文“接近完成”直接冲突。其他新闻含未来时态/preview的，不为凑实例判夸大。
