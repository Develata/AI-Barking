# 1104 来源与制作状态

期号1104（上期1103+1）；线索为 Develata 回贴的两份扫描：WorkBuddy“AI吠点选题扫描-1105-12”（北京 11-05 12:00）与 ChatGPT 滚动扫描（北京 11-05 13:07），只作线索。Develata 11-05 按 Claude 建议选定主帖：OpenAI 948 个 Linux 提权漏洞、Kimi-K3.1-Pro 放出权重【跟进1103】。

## 主帖

1. OpenAI：[A thousand fewer paths to root](https://openai.com/index/a-thousand-fewer-paths-to-root/)（L1，页面只给日期 11-04；curl 403，opencli 取；`a-openai-paths-to-root.*`）；[HN 50372929](https://news.ycombinator.com/item?id=50372929)（L6，含自称作者的答疑）；lore.kernel.org 上 openai.com 邮箱的补丁线程（L1，只存回复与主题清单，不存补丁正文，未引）。
   - 夸大实例：第一财经日报微博“948 个漏洞已提交补丁”（[新浪页](https://www.sina.cn/weibo/detail/5361796030400510.html)，北京 11-05 07:34）；量子位“OpenAI提出了948个补丁建议”（[原文](https://www.qbitai.com/2026/11/507697.html)，09:54）；蚁工厂“上千个”；蓝点网“担心再次翻车”（站方猜测）。IT之家与英文 ShiftDelete、Cryptika 基本忠于原文。
   - 安全题材：存档不含任何利用代码或补丁正文。
2. Kimi：[Hugging Face 仓库](https://huggingface.co/moonshotai/Kimi-K3.1-Pro)（L1；API、commits、tree、blobs、README、LICENSE 存档）；[Kimi 博客](https://www.kimi.com/en/blog/kimi-k3-1-pro)（L1，“by Nov 4, 2026”）；官方放权重帖（L1，经 fxtwitter 存 JSON）；[vLLM 配方](https://recipes.vllm.ai/moonshotai/Kimi-K3.1-Pro)（L3）；[AA v5 发布文](https://artificialanalysis.ai/articles/artificial-analysis-intelligence-index-v5)与[模型页](https://artificialanalysis.ai/models/kimi-k3-1-pro)（L3）；[HN 50366999](https://news.ycombinator.com/item?id=50366999)（L6，最早“已公开”时间戳）。
   - 夸大实例：Cocoloop“文件大到 5.6TB”（BF16 估算）；IT之家/新浪“今日”开放与 5.6TB 估算；AIDeepThink、AILog“基本沿用 MIT”“MIT 骨架”（同句也写了门槛）；三文阁“3万亿参数”。“46.5 分”未找到发表稿。
   - 华为昇腾 0 Day 适配：公众号原文因验证码未取得，只有 IT之家转述，未引。

## 速览

- Images 2.6：[OpenAI 公告](https://openai.com/index/introducing-images-2-6/)（L1）；Flare 模型页与定价页存档。
- Conductor：[官方公告](https://www.conductor.build/announcement)与 changelog 0.93.0（L1）。
- Astra：[OpenAI 对齐报告](https://alignment.openai.com/misalignment-reports/covert-underperformance-on-safety-research-tasks/)（L1）。
- Ripple：[Ricursive 官网](https://www.ricursive.com/ripple)与 [HF 模型卡](https://huggingface.co/ricursiveAI/Ripple-65B-Preview)（L1）。
- Claude 结构：[Anthropic 研究页](https://www.anthropic.com/research/investigating-vault-particles-with-claude)（L1）。
- Business Insider 两篇（Cat Wu 离职、加入 OpenAI）属订阅媒体，正文文本不入公开仓库（`.txt` 已移出，html 原件只存本地），且正文未引。
- 不收：HumanLayer 加入 Factory（圈内）、SWE-2 Flash（与 Ripple 同类，条数已够）、Cat Wu 转投 OpenAI（“Claude Code 产品负责人”只见于 Business Insider，一行放不下来源与限定词）；OCC 银行检查框架、Daybreak Red 上 GovCloud（涉外国政府，风控）、TeraWulf 信用支持（融资合同）、高通 Cloud AI 100 Ultra（2023 年旧品）、美国地方公投否决数据中心（涉外国政治）。取证存档仍在 `c-`、`d-` 文件中。

## 输入与取证

派工单 `.handoff/2026-11-04-1104-evidence.md`（文件名日期按本机美东时间由 `barking handoff` 生成）。opencli 预检正常。A/B 组 Claude Sonnet 5.5（约19、24分钟，harness 计时），C/D 组 Claude Haiku 5.5 effort high（约4、4分钟）；均未到60分钟催交。Claude 直接核读：OpenAI 原文存档关键段、Kimi 模型卡与 LICENSE（raw 文件）、AA v5 发布文、vLLM 配方页、HF commits API。

热度（HN，北京 11-05 17:10–17:25）：Kimi 权重 514–523 分/142–146 评；OpenAI 948 310–320/212–228；Images 2.6 273–284/168–169；Ripple 285–289/103–104；Conductor 171–173/179–182；Astra 报告 5/1；vault 3/0。Reddit 全部被登录墙拦下。

## 扫描说法勘误（已核原文）

- 两份扫描：OpenAI 原文“北京 11/5 00:00 发布”无出处（页面只有日期，RSS 为 11-04 08:00 GMT）；“166 个样本出自 7 月那轮”是推断，原文没把两处连起来；“948 是补丁数不是漏洞数”不准，原文按补丁数去重来计唯一漏洞数。
- WorkBuddy：AA 中位耗时为 12.5 分钟（不是 12.4），排名分母 #22/68（不是 /67）；“46.5 分”未见发表稿（页内嵌值 46.4526）。
- ChatGPT：HumanLayer 的 Factory 页标题是 “joining”，“Acquires” 只见于 HN 投稿标题；Cat Wu 原帖未写“负责 Agent 产品”。

## 已删除文件（Claude 收尾一次删，北京 11-05）

- `images/15-hf-license.png`、`images/23-cocoloop-5-6tb-title.png`（B 组弃用，右侧截断或行被切半，已由 15a、23a 替代）
- `sources/c-c-bi-catwu-left.html`、`sources/c-c-bi-catwu-left.headers`、`sources/c-c-oai-images25.html`、`sources/c-c-oai-images25.headers`（C 组猜错 URL 的 404 页）、`sources/c-reddit-heat.json`（Reddit 空结果）

## 核验

读者稿检查（Codex）、两轮反向核验与视觉复核结论见 `fact-check.md` 与 `style-review.md`。
