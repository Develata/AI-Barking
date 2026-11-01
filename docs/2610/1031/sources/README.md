# 1031 来源与制作状态

期号1031（上期1030+1）；线索为 Develata 回贴的 ChatGPT 滚动扫描（北京 11-01 19:08）与 WorkBuddy 扫描（1102-00，北京 11-02 00:01），只作线索。Develata 11/01 同意 Claude 建议的主帖：Exin 让 AI 智能体给活鼠脑内注射 → ChatGPT 与 Codex 共用额度 → DeepSeek 超级智能体原型。

## 主帖

1. AI 智能体给活鼠脑内打针：CEO Gabriel Ocana-Santero 海外社媒首帖 status/2115442964574908902（L2，北京 10-28 21:58）与帖串、三模型对比帖串 status/2116285912628052110（L2，北京 10-31 05:47:34）；[WIRED](https://www.wired.com/story/an-ai-agent-did-brain-surgery-on-live-mouse/)（L4）；Anthropic [Opus 5](https://platform.claude.com/docs/en/models/opus-5/overview)、[Sonnet 5](https://platform.claude.com/docs/en/models/sonnet-5/overview) 模型页（L1，Legacy）。
   - 夸大实例：[IT之家](https://www.ithome.com/1/020/737.htm)“让 AI 自己做脑部手术，初创公司成功让智能体给小鼠治疗癫痫”；[网易号 DeepTech](https://www.163.com/dy/article/LAM28HOG05119734.html)“AI首次操刀活鼠脑部实验”；[The Decoder](https://the-decoder.com/a-claude-model-autonomously-chose-a-gene-therapy-and-operated-on-a-mouse-brain-with-no-human-decision-making/)“with no human decision-making”。
2. ChatGPT 和 Codex 共用额度：[OpenAI 帮助页](https://help.openai.com/en/articles/20001625-understanding-chatgpt-and-codex-usage-limits)（L1，updatedAt 北京 11-01 00:17:48；Cloudflare 拦 curl 与 headless，有界面 Chrome 渲染存 `a-help-usage-rendered.txt`）；[Business 模型与限额页](https://help.openai.com/en/articles/12003714-chatgpt-business-models-and-limits)（L1）；[Codex 定价页](https://developers.openai.com/codex/pricing)（L1）。用户转贴的通知邮件（HN 50324512、50326057，L6，未写）。
   - 夸大实例：[爱范儿（腾讯新闻）](https://news.qq.com/rain/a/20261101A05QJV00)“无限畅聊的时代要结束了”（正文压字数删去）。
3. DeepSeek 超级智能体原型：[dsh-superagent 仓库](https://github.com/deepseek-ai/dsh-superagent-prototype)（L1，创建北京 10-31 17:32:46）；微信公众号“DeepSeek 产品部”[原文](https://mp.weixin.qq.com/s/zoS7GlReWPhfCjENGxXCdg)（L1，北京 10-31 20:29:47）与 DeepSeekHarness 英文版 status/2116857257245987236。
   - 夸大实例：[新浪托管自媒体稿](https://k.sina.cn/article_5953740931_162dee083067043yzs.html)“这 AI 牛马，没人 @ 都自己干活”（该稿所指为内部实例，同文又写得切 auto 模式）；Pandaily 帖 “Finds and Fixes Bugs on Its Own”。

## 速览

- HackerTube：[Karpathy 博文](https://karpathy.bearblog.dev/hacker-tube/)（L2）；[HN 50319510](https://news.ycombinator.com/item?id=50319510)。
- 框架成本（撤出速览：小样本条件压不进40字）：[Nolan Lawson 博文](https://nolanlawson.com/2026/10/28/the-ai-cost-of-javascript-frameworks/)（L2，10-28，窗口前）；[HN 50323088](https://news.ycombinator.com/item?id=50323088)。
- 安全事件：[Wiz “Black October”](https://www.wiz.io/blog/black-october-for-ai-developer-platforms)（L3，厂商自家研究）；[Northflank 状态页](https://status.northflank.com/history/1)（L1，未单列）。
- Grok 水印：[xAI 开发者文档](https://docs.x.ai/developers/advanced-api-usage/watermarking)（L1，页面署名 SpaceXSI）。
- ChatGPT 漏洞：[Parameter 报告](https://www.parameter.ai/labs/we-found-a-gap-at-openai)（L3，研究方自述）。
- Loop-9B：[GitHub](https://github.com/OpenSenseNova/SenseNova-Loop)、[Hugging Face](https://huggingface.co/sensenova/SenseNova-Loop-9B-Preview)（L1）；对比图 `e-sensenova-benchmark.png`（Claude 下载，官方仓库 assets，只存本地）。
- 苹果：[WIRED 专访](https://www.wired.com/story/i-spent-30-minutes-with-apples-ceo-now-i-know-why-john-ternus-got-the-job/)（L4）。
- 未收：Leo de Moura 访谈（观点，HN 208 分）、Northflank 单列（并入 Wiz 九家）。不收：巴西律师提示注入（5 月旧判）、Memdoor（原页拒绝访问）、Anthropic 因训练封号（当事人原帖不可读）、Nace Drex（赞助稿）、Claude Ultra / 苹果入华 / Cursor 插件（已发）、OpenRouter 代码审查教程（原页打不开）、额度重置。

## 输入与取证

派工单 `.handoff/2026-10-31-1031-evidence.md`（文件名日期为工具按期号日期生成；工具按 10-31 推算的“北京 = 本机 + 12 小时”有误，美东 11-01 已退夏令时，派工单已手改为 +13）。A/B/C 组 Claude Sonnet 5.5（约15、27、17分钟），D/E 组 Claude Haiku 5.5 effort high（各约6分钟）。opencli 浏览器扩展未连接，截图全走 `barking shot` 与组内 CDP 脚本。取证全由 Claude 系代理完成，反向核验须逐行核对事实清单与 `sources/`。

## 扫描说法勘误（已核原文）

- 共用额度：“12 月”与邮件的 “coming weeks” 出自不同文件，不矛盾；帮助页只写月份，没有日期；Work 与 Codex 本来就共用额度（Codex 定价页）；“小范围测试”只修饰 Chat/Work 合并界面。
- Exin：CEO 首帖是北京 10-28，不是 10-29；三模型对比帖是 10-31 05:47，不是 11-01；腾讯新闻那篇是爱范儿稿，不是 IT之家同稿；“输出被截断”那段话 CEO 帖里出自 Opus 5，IT之家写成 Opus 5.5；拒绝原因 CEO 与 WIRED 两版不一致；伦理审批一手来源未提。
- DeepSeek：“7×24”“always-on”字面不是 DeepSeek 措辞，但文章写“持久：始终在线”，含义相同（第一轮反向核验 BLOCKER 1，正文不再以此为吠点）；文章确有内部实例“无人 @ 主动识别缺陷并提交修复”，但发布版默认是观察模式；“约 7 小时涨 1600 星”“AIHOT 139”无一手来源。
- HackerTube：“约两小时做成”原文只有 “This takes ~2 hours now.”，所指不明，不写。
- Loop-9B：“媲美 27B”不在页面文字中；对比图 8 项中 6 项高于 Qwen3.5-27B，Terminal Bench 2.1 与 τ³-Banking 更低；两页均未写“商汤”。
- Wiz：“超 10 万美元”原文为 “likely … at list price”；80 万、170 万是调用数，不全是成功数。
- Grok 水印：官方文档只覆盖 API 文本输出；@grok 账号 8 月曾称无任何文本水印。

## 风控

- 活体动物实验：所有截图只含文字与示意，不收手术现场、动物伤口或解剖画面。
- 标题与封面不写“政府”；本期无涉外国政府与政治题材。
- 海外社媒截图只含 CEO 本人与官方、媒体账号卡片，无抓取者信息。
