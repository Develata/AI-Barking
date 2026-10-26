# 1025 来源与制作状态

期号1025（上期1024+1）；线索为 Develata 回贴的 ChatGPT 扫描（北京时间10/26 17:01）与 WorkBuddy 扫描（1026-18），只作线索。Develata 10/26 定主帖：皮肤癌 CNN 对比、Kimi 额度事故【跟进1023/1024】。

## 主帖

1. 皮肤癌测试：[Lynch Lab 原文](https://lynchlab.net/llm-skin-cancer)（作者个人网站，L1 作者自报，2026-10-25，官方未给时刻）；[数据包](https://d2gaqzonlm544h.cloudfront.net/skin-ai/data/llm-skin-cancer-test.zip)；CNN 模型页 `/skin-ai-inception`；作者 10-11 前作（drmagnuslynch.com，只测两个 LLM）；[MILK10k](https://doi.org/10.34970/648456)（ISIC，CC BY-NC）；HN 50234186 主帖（`a-hn-algolia.json`、`a-heat.json`）；[AI/TLDR](https://ai-tldr.dev/releases/lynch-lab-cnn-vs-llm-skin-cancer/)（L5 聚合站，“fewer misses”指分类错误，不算夸大实例）。证据 `evidence-a.md`。
2. Kimi 额度事故【跟进1023/1024】：[Kimi 官方帖](https://x.com/Kimi_Moonshot/status/2114450610724708492)（L1，北京10/26 04:14:44）；中文公告《致用户：关于近期使用体验问题的致歉与补偿》（官方原帖未直接读到，转录见 [IT之家/凤凰网](https://tech.ifeng.com/c/8xWrSDBLrWR)、[21经济网转财联社](https://m.21jingji.com/article/20261026/herald/bf1e90e1d6a314bec72c3dac18eb3625.html)，L4）；@KimiDevs 10/25 Kimi Code CLI 2.3.2 与 Desktop v1.1.3 更新（L1）；[Kimi Code 文档](https://www.kimi.com/code/docs/en/kimi-code-cli/customization/agents.html)；用户投诉 HN 50222093（L6）；新浪数码标题（L4，见 evidence-b）。Reuters 原站 401，未绕。证据 `evidence-b.md`。

## 速览

- IBM CEO：[The Register](https://www.theregister.com/systems/2026/10/22/ibm-ceo-says-open-weight-models-are-now-irresistibly-good-and-cheap-on-prem/5304640)（L4；IBM 讲稿无此句，转录未取到）。
- Borcherds：[The Daily Californian](https://www.dailycal.org/news/campus/academics/ai-use-leads-uc-berkeley-math-professor-to-see-no-point-in-grading-homework/article_bdf1eae2-c08b-491f-b9f1-81a7cea177de.html)（L4）。
- r/opensource：[版主原帖](https://www.reddit.com/r/opensource/comments/1xelcuf/your_tokens_arent_valuable_no_more_ai/)（L1，版面管理方公告，仅限该子版）。
- CapCut：[官方帖](https://x.com/capcutapp/status/2114062937094823955)（L1）。
- 未用：Wiz Gheist、ThinkRail、Nanbeige4.3-33B、Gary King 论文、Meta 条款推迟（未找到材料）。不收：佐治亚州选举数据、Convovo“AI 选票”（风控）。

证据 `evidence-c.md`。

## 输入与取证

派工单 `.handoff/2026-10-25-1025-evidence.md`。A/B 组 Sonnet 5.5（harness 用时约49、64分钟，B 组在60分钟时被催交），C 组 Haiku 5.5（约24分钟，模型 ID 自报 claude-haiku-5-5）。

## 待办与限制

- 未复跑任何实验；CNN 逐例输出不在数据包里，CNN 数字只能引作者自报。
- Kimi 中文公告的官方原帖与精确发出时刻未取到。
