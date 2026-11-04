# 1103 来源与制作状态

期号1103（上期1102+1）；线索为 Develata 回贴的 WorkBuddy 扫描（1104-18，北京 11-04 18:00）与 ChatGPT 滚动扫描（北京 11-04 18:55），只作线索。Develata 11/04 按 Claude 建议选定主帖：腾讯 Hy4“最强开源”与 Kimi-K3.1-Pro 权重到期【Kimi 部分跟进1023】→ Postman 桌面版自动装 AI 插件；速览由 Claude 从两份扫描中挑选（原六条，反向核验第2轮后移出 FlashProve 与 Redwood，剩四条）（两份扫描本轮都没给“速览候选”一节）。

## 主帖

1. 腾讯 Hy4：[腾讯新闻腾讯官方账号](https://news.qq.com/rain/a/20261104A074Z600)（L1，北京 11-04 12:52）；[腾讯官网简体稿](https://www.tencent.com/zh-cn/articles/)与英文稿（L1，只给日期；含“将于近期开源模型权重”一句的是简体稿，`a-` 存档）；混元 Blog（L1，opencli 渲染，`a-hy-blog-hy4.md`、`a-hy-blog-hy4-en.md`）；[Hy4-preview 模型卡](https://huggingface.co/tencent/Hy4-preview)；HF/GitHub/ModelScope 组织列表 API（`a-` JSON）。Kimi：[官方博客](https://www.kimi.com/en/blog/kimi-k3-1-pro)（L1）、官方 X 帖 fxtwitter JSON；HF `moonshotai` 列表。
   - 夸大实例：什么值得买用户稿“月之暗面 Kimi K3.1 Pro 今天正式开源全部权重”（北京 11-04 08:56）；21 财经转财联社“腾讯史上最强开源模型，来了”（19:56，正文写“将于近期”）。未找到把 Hy4 权重写成“已发布”的稿子。
   - 路透社（L4）正文由 opencli 首次渲染时页面直接给出，未登录、未绕过订阅框，只作线索，正文不引；这份正文文本属付费媒体全文，不入公开仓库（Claude 移出，本地草稿目录保留）。
2. Postman：[GitHub issue 14370](https://github.com/postmanlabs/postman-app-support/issues/14370)（L6 用户报告，gh API JSON 含全部评论）；[postman-plugin 源码固定 commit 6c8e452e](https://github.com/postmanlabs/postman-plugin/tree/6c8e452e0f4237f4bcc0901cb1ac442dec707e49)（L1）；[Postman 文档 detected-agents](https://learning.postman.com/docs/postman-plugin/install/detected-agents/) 与 per-agent（L1）；[发行说明](https://www.postman.com/release-notes/postman-app/)（L1）；[HN 50354960](https://news.ycombinator.com/item?id=50354960)；[The New Stack](https://thenewstack.io/postman-plugin-coding-agents/)（L4，页脚披露其所有者 Insight Partners 投资 Postman 与 OpenAI）。
   - 夸大实例：HN 标题“attempts to share all API data”；提交者 X 帖“literally intercepts and sends all of your API data”。

## 速览

- 手表：[Apple Support 148354](https://support.apple.com/en-us/148354)（L1）与 Audio Intelligence 隐私概览 PDF（只存本地）；The Verge、MacRumors（L4）。
- Siri：[Apple Developer 新闻](https://developer.apple.com/news/?id=gdk2umrf)（L1）与 Siri AI Developer Preview 文档（JSON 版存档）。
- 学校账号：[Google 教育博客](https://blog.google/products-and-platforms/products/education/ai-learning-era-updates-2026/)（L1）；[HN 50353192](https://news.ycombinator.com/item?id=50353192)（链接 NYT 报道）。
- AMD：[AMD 第三季度财报新闻稿](https://ir.amd.com/news-events/press-releases/detail/1303/amd-reports-third-quarter-2026-financial-results)（L1）；电话会转写为第三方（stockanalysis.com），未引。
- 不收：Anthropic FlashProve（[研究页](https://www.anthropic.com/research/provable-inference)，条件装不下）、Redwood 博客 Alec Harris 文（[原文](https://blog.redwoodresearch.org/p/openai-agents-keep-disabling-monitoring)，属作者建议）；取证存档仍在 `d-` 文件中。其余见 `fact-check.md`“速览”末行。

## 输入与取证

派工单 `.handoff/2026-11-03-1103-evidence.md`（文件名日期由 `barking handoff` 生成）。opencli 预检正常。A/B 组 Claude Sonnet 5.5（约44、29分钟，harness 计时），C/D 组 Claude Haiku 5.5 effort high（约15、14分钟）；均未到60分钟催交。Claude 直接核读：腾讯新闻版全文“开源/权重/系统卡”句（20:40）、腾讯官网简体稿截图、12项对比图逐项读数、HF API（20:37、21:32）、HN 热度（20:40）、Postman issue gh API（20:45）。

热度（HN，北京 11-04 20:40–20:58）：Postman 50354960 1344→1359 分、453–456 评；Google 学校账号（NYT）632/436；Apple Watch“always-listening”评论文 66/34；Tencent Hy4 7/2；Siri 开放 4/0；FlashProve 各帖 ≤10 分；AMD ≤3 分。中文侧 Hy4 为腾讯新闻、IT之家、澎湃、新京报、财联社等同稿扩散。

## 扫描说法勘误（已核原文）

- WorkBuddy：Hy4 官方稿发布时刻“14:06”与腾讯新闻版 12:52 不符；“将于近期开源模型权重”在腾讯官网简体稿里有，腾讯新闻版无（两份扫描都当成同一官方稿）。
- WorkBuddy：“770B 总参数 / 49B 激活 / 1M 上下文”是 preview 的参数，Hy4 正式版官方稿没写。
- WorkBuddy：AMD“财报原文无 2027 年指引”只对新闻稿成立，电话会转写有 2027 年营收增长 “more than 80%”；本期不写 2027 年数字。
- WorkBuddy：把 Postman、Google 学校账号等列为“已收”，指的是其自身扫描轮次，本号未发过。
- ChatGPT：Apple Watch 功能“11月3日 watchOS 27.3 beta 开启”只见 The Verge、MacRumors，苹果原文是将来时；FlashProve stars 631 为较早快照（取证时681）；42.9%/57.1% 是召回率，不在 OpenAI 10/29 正文、只在图表数据里。
- ChatGPT：Siri App Store 提交“2027 年春”的年份为推断，官方只写 “in the spring”。

## 时效

Kimi-K3.1-Pro 的“by Nov 4”按美国时区到北京 11-05 13:00（美东）/16:00（太平洋）才过；Hy4 权重也可能随时放出。**发布前再查一次 HF `moonshotai` 与 `tencent`**，任一放出则改写 A 节吠点①或③。

## 已删除的临时文件

北京 11-04 22:59 收尾时一次删除各组回报的试验与废弃文件（A 组 `a-list-hf.py` 与 19 个 `a-tmp-*.png`；B 组 6 个 `images/tmp*.png`、截图试验脚本、重复与空文件、Reddit 登录墙空壳；C 组重复抓取的 Google 博客；D 组 403 页与未引用的 OpenAI 首页）。Claude 改裁批注底图后的旧版（`hy4-weights-soon-raw.png`、`postman-hn-title-raw.png`、`publish/13-postman-hn-title.png`）移出仓库，不上传。同时复查 HF：`moonshotai` 仍无 K3.1，`tencent` 仍只有 Hy4-preview。
