# 1127 来源与制作状态

期号 1127（上期 1126 + 1）。线索为 Develata 11-28 回贴的两份扫描：ChatGPT 定时重扫 `AI-Barking_stateful_rescan_20261128T110000Z`（北京 11-28 19:07）与 WorkBuddy HTML 选题扫描 `1128-18`（北京 11-28 18:01），主执行方对照“已发过”去重并现查 HN 热度。Develata 北京 11-28 约 23:30 拍板主帖：一、小米汽车世界模型 2.0“全面 SOTA”口径；二、美国北卡公寓 AI 助手邮件与驱逐案。经读者稿检查与五轮反向核验、三轮视觉复核，第四轮后由 Develata 拍板，标题定为「小米称视频分第一，同表美观度垫底」（候选与弃用理由见 `titles.md`）。

派工单 `.handoff/2026-11-28-1127-evidence.md`（`barking handoff` 生成时把文件名日期写成 11-27、时区写成美东，主执行方已改名并改正时区一行）。派工前 `opencli doctor`（北京 23:31）：Browser Bridge 已连接。取证子代理：A、B 组 Claude Sonnet 5.5，C 组 Claude Haiku 5.5（均为 Claude 家族，与主执行方同家族）。

## 扫描勘误

- **A 组**：扫描称 Imaging Quality 69.1 属 HunyuanVideo1.5，实为 LingBotWorld（Hunyuan 69.0）；“全面 SOTA”只出现在小米技术微博头条文章与转载稿，论文摘要与正文没有；“汽车与机器人共用一套大脑”在论文里是共用架构与预训练底座，部署权重按场景分开；扫描所说“只和开源比”属推断，论文未标开闭源，正文改写为不涉开闭源的 TriWorldBench、LIBERO Plus 排名。
- **B 组**：扫描称 WRAL 议员表态稿 11-27，实为美东 11-20 晚（北京 11-21 08:16）；未找到 WRAL 11-27 新稿。“无家可归”无出处，WRAL 写“周末住酒店”；简体中文未找到“房东翻脸”式标题，夸大实例为英文 Futurism、Moneywise 标题与台湾商傳媒。“楼下卫生间天花板”一句 WRAL 写“one section of an email said”，未说是 AI 写的。11-19 的申请 WRAL 称上诉、房东称撤销裁定动议，正文写“申请被驳回”。
- **C 组**：OpenAI 反诉苹果的路透社原文 401 未取，改以 CNBC 为 L4 依据；OS-Omni 的 68.5% 为宏平均，微平均下 GPT-6-Astra 67.0% 略高于 Opus-5.5 66.7%；Humphries 文中模型原话为 “probably”；HFE 为 “elementary” 基础题。

## 主帖

1. 小米汽车世界模型 2.0（A 组，`evidence-a.md`、`capture-log-a.md`）：
   - 技术报告 [arXiv 2611.01677](https://arxiv.org/abs/2611.01677)（v1 北京 11-01 15:53:48，`a-arxiv-pdf.txt`）；[项目页](https://xiaomiautoworldmodel.github.io/CausalWorldModel/)；[小米技术微博头条文章](https://weibo.com/ttarticle/p/show?id=2309405369910959407174)（页面显示 11-27 17:00 已编辑）与[短帖](https://weibo.com/6486870325/RtkHBt2aH)；雷军帖（11-28 12:25，无 SOTA 字样）。
   - 转载对照：[IT之家](https://www.ithome.com/1/033/614.htm)（原样转述“全面取得 SOTA”）；[搜狐号](https://www.sohu.com/a/1107623848_122066676)（扩成“多个领域取得SOTA”）；[CNMO](https://smartcar.cnmo.com/news/823484.html)（碰撞率句保留了“闭环仿真”，初稿误列为夸大实例，第一轮反向核验后撤回）。
   - 热度：小米技术头条文章阅读数“26万+”；HN 无讨论；r/SelfDrivingCars 11 分。
2. 北卡公寓 AI 邮件与驱逐（B 组，`evidence-b.md`、`capture-log-b.md`）：
   - WRAL [首报](https://www.wral.com/news/investigates/eviction-raleigh-apartment-ai-leasing-agent-november-2026/)（北京 11-20 07:35）、[议员跟进](https://www.wral.com/news/investigates/raleigh-eviction-after-ai-bots-promise-push-for-protection-nov-2026/)（北京 11-21 08:16）、[房东回应](https://www.wral.com/news/investigates/apartment-response-ai-bot-raleigh-eviction-november-2026/)（北京 11-25 08:21）。
   - [Moneywise 经 Yahoo Finance](https://finance.yahoo.com/real-estate/articles/raleigh-woman-just-got-evicted-220000991.html)（房东发言人 Leach 直接引语）；[Futurism](https://futurism.com/artificial-intelligence/landlord-ai-deal-evicts-woman)。
   - 热度：HN [50673521](https://news.ycombinator.com/item?id=50673521)（北京 11-28 23:34 采样 198 分 / 155 评，链 Substack 转述）。
   - 未进正文：北卡参议员 Burgin 的“Bot Bill”（[S963](https://www.ncleg.gov/BillLookUp/2025/S963)、[S624](https://www.ncleg.gov/BillLookUp/2025/S624) 对应关系未证实）；Moffatt v. Air Canada（仅第三方托管副本）。

## 速览（C 组，`evidence-c.md`、`capture-log-c.md`）

- 谷歌 DCA：[Google Research 博客](https://research.google/blog/supercharging-transformer-residual-connections/)；[arXiv 2502.06785](https://arxiv.org/abs/2502.06785)；HN 50672565。
- Nielsen 下架教材：[原站声明](https://neuralnetworksanddeeplearning.com/)；HN 50670757。
- GPT-6.2-Astra 素描本：[Humphries Substack](https://generativehistory.substack.com/p/gpt-62-astra-solves-a-historical)；HN 50643158。
- DeepMind 质谱技能包：[DeepMind 博客](https://deepmind.google/blog/interpreting-mass-spectra-with-google-antigravity/)。
- OS-Omni：[arXiv 2611.34232](https://arxiv.org/abs/2611.34232)。
- Google HFE：[Google 博客](https://blog.google/innovation-and-ai/models-and-research/google-deepmind/humanitys-first-exam/)。
- OpenAI 反诉苹果：[法院卷宗 Document 217](https://storage.courtlistener.com/recap/gov.uscourts.cand.474095/gov.uscourts.cand.474095.217.0_1.pdf)（反诉，11/27 提交；`c-c8-court-217.txt`）；[CNBC](https://www.cnbc.com/2026/11/27/openai-countersues-apple-for-stealing-trade-secrets-ft-reports.html)；[卫报](https://www.theguardian.com/technology/2026/nov/27/openai-countersues-apple-trade-secrets)；[BBC](https://www.bbc.com/news/articles/cm15ekqvlezeo)。
- 未收：蚂蚁灵波 LingBot（性能数字仅见量子位）、Convex、Construct22、自托管、量子引力评议、日本 AI 法草案、两州诉 Grok（风控或核实度不足）。
