# 1019 来源入口

正文：`../doc_1019_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-A.md`（DeepInfer 开源与知乎技术报告）、`evidence-B.md`（WhatsApp 帮助页隐藏指令）、`evidence-C.md`（Mercury 3）、`evidence-D.md`、`evidence-E.md`（速览）；抓取日志：`capture-log-A.md` 至 `capture-log-E.md`；派工单：`../../../../.handoff/2026-10-19-1019-evidence.md`（文件名日期按本机美东时间生成）。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D、E 组 Haiku 5.5（均 effort high），派工单 60 分钟目标。耗时（harness 计时）：D 约 11 分钟、E 约 12 分钟、B 约 32 分钟、C 约 35 分钟、A 约 104 分钟（60 分钟时催交，交回时 A7–A9 部分未完成，见 `evidence-A.md`）。

选题：一份 ChatGPT 扫描（北京 10/20 17:06）与一份 WorkBuddy 扫描（18:02）。WorkBuddy 首推 DeepInfer 与知乎报告的38美元，ChatGPT 首推 WhatsApp 隐藏指令；Mercury 3 是 Develata 自己刷到的线索。Claude 抽查后建议三条主帖，Develata 拍板。扫描勘误：WorkBuddy 称知乎报告两节“统计表无内容”，实为图片（A 组截图 03、04）；WorkBuddy 把“谷价”理解成低谷电价，报告未解释；ChatGPT 扫描把“多数隐藏内容是一般客服说明”记成 WhatsApp 发言人的话，实为 Redact CEO 在 WIRED 的话；派工单里“HN 无 Mercury 3 帖”是 Claude 查询条件写错（实有 HN 50155245，249 分）。

## 一手来源

- DeepInfer：https://github.com/deepseek-ai/DeepInfer （README、`eval/README.md`）；deepseek-recipe：https://github.com/deepseek-ai/deepseek-recipe ；知乎技术报告：https://zhuanlan.zhihu.com/p/2095560295930508953 ；官方 X 帖：https://x.com/deepseek_ai/status/2112476496283775135 ；DeepSeek 价格页：https://api-docs.deepseek.com/zh-cn/quick_start/pricing ；公众号文章经搜狗微信跳转取得（链接带签名，存档 `a-wx-official.json`）；快科技：https://m.mydrivers.com/newsview/1159039.html 。
- WhatsApp：Redact https://redact.dev/blog/whatsapp-help-pages-hidden-ai-instructions ；WIRED https://www.wired.com/story/whatsapp-help-pages-leaked-hidden-instructions-for-a-meta-chatbot/ ；删号页 https://faq.whatsapp.com/2138577903196467 、举报页 https://faq.whatsapp.com/414631957536067 ；互联网档案馆快照见 `b-wb-*.html`（10/9 05:48、10/16 14:11、10/18 02:37、10/20 07:44 UTC）；Redact 官网 https://redact.dev/ 。
- Mercury 3：https://www.inceptionlabs.ai/blog/introducing-mercury-3 ；文档 https://docs.inceptionlabs.ai/get-started/models （价格）与推理档说明（`c-docs-reasoning.txt`）；OpenRouter https://openrouter.ai/inception/mercury-3 ；OpenAI 价格 https://developers.openai.com/api/docs/pricing ；Anthropic 价格 https://platform.claude.com/docs/en/about-claude/pricing ；AA https://artificialanalysis.ai/models/mercury-3 （404）；HN https://news.ycombinator.com/item?id=50155245 。
- 速览：路透社 https://www.reuters.com/world/openai-seeks-accelerate-fundraising-ahead-anthropic-ipo-sources-say-2026-10-19/ ；MiniMax https://www.minimax.io/blog/minimax-m31f 、AA https://artificialanalysis.ai/models/minimax-m3-1-flash ；AA HealthBench https://artificialanalysis.ai/articles/healthbench-professional-health-conversations ；Viggle https://viggle.ai/jst2 ；ProjectDiscovery https://projectdiscovery.io/research/how-abliterated-models-can-get-you-pwned 。

## 待核验与未完成

- 知乎报告“谷价”与价格页“空闲时段”是否同义，官方未写；正文只并列。
- 38美元没有卡数、卡型、集群数，无法换算单位成本。
- WhatsApp：机器人是否照做、受影响人数、隐藏文本存在多久均未知；Redact 称 9/4 副本已有该提示，B 组取到的 9/7 快照缺正文，未能印证。
- Mercury 3：参数量、GPU 型号（联合创始人 HN 评论称 Blackwell）、对比表对手速度 ~130/~240 的来源均未给；首发五折无截止时间。AA 一条提醒帖（称发布稿里 AA-Omniscience 与 GDPval-AA v2 分数“do not follow our methodology”）在 X syndication 显示已删，未用。
- OpenRouter 速度为页面读数，随时变化；“1 week”图表与顶部 P50 摘要是两组数，正文只用顶部摘要。
- `d-projectdiscovery.txt` 含博文演示用的虚构凭据字符串，入库前处理（见 `docs/workflow/publish.md` 的凭据检查）。
- B 组 45 号批注底图第一次裁切（全高）作废重裁，`b-shots-log.jsonl` 保留两次记录。
- 未进正文：Browser Use Bench v2.1、斯坦福 PhysicianBench、GitHub Copilot 下线旧模型、OpenAI COBOL 测试（限定词放不进速览40字，第一轮反向核验后撤下）；美国 DHS 对 DeepSeek 的限制令、欧盟 AI Office 问询、Alfozan 案（风控，未取证）。
