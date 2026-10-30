# 1029 来源与制作状态

期号1029（上期1028+1）；线索为 Develata 回贴的 ChatGPT 独立滚动扫描（手动轮，运行键 manual-2026-10-30T13:28:00Z，北京21:30）与 WorkBuddy 扫描（1030-18，北京18:02），只作线索。Develata 10/30 按推荐定主帖：Cursor 扩展市场 Claude Code 扩展投毒 → CLEAR 分词器税。备选未取证：Epoch AI 开放权重模型 Firefox 补丁空窗实验（美方机构以国产模型做攻击演示，风控存疑）。

## 主帖

1. Cursor 扩展投毒：[Cursor 安全公告](https://cursor.com/blog/claude-code-extension-security-notice)（L1，页面 Oct 29, 2026，官方未给时刻；[中文版](https://cursor.com/cn/blog/claude-code-extension-security-notice)）；Cursor 官方海外社媒帖（L1，北京10/30 15:46:39，存 `a-cursor-x-fx.json`）；[Felix Rieseberg 的 HN 评论](https://news.ycombinator.com/item?id=50304129)（L2，北京10/30 22:08；GitHub 资料写 Anthropic）；[Kevin Beaumont 博文](https://doublepulsar.com/anthropic-hacked-to-deliver-remote-access-trojan-to-claude-developer-laptops-worldwide-1148a92baae2)（研究者本人，夸大标题实例）；[OpenSourceMalware 分析](https://opensourcemalware.com/blog/trojanized-claude-code-extension-plants-persistent-ssh-backdoor)、[Phorion](https://phorion.io)（技术交叉核对，未进正文）；[npm](https://registry.npmjs.org/@anthropic-ai/claude-code)、[Open VSX](https://open-vsx.org/api/Anthropic/claude-code)、[GitHub releases](https://github.com/anthropics/claude-code/releases) 版本史（均无 2.1.313，跳号原因无证据）；HN [50300683](https://news.ycombinator.com/item?id=50300683)（约190分／298评）、[50301983](https://news.ycombinator.com/item?id=50301983)（393分）。
2. CLEAR 分词器税：[CLEAR 报告 The Tokenizer Tax](https://clear-languages.org/blog/the-tokenizer-tax)（L1，2026-09-29，官方未给时刻；JS 渲染，存 opencli 渲染文本 `b-report-rendered.txt`）；[CLEAR 成员页](https://clear-languages.org/coalition)；[TII Falcon 3 发布稿](https://www.tii.ae/news/falcon-3-uaes-technology-innovation-institute-launches-worlds-most-powerful-small-ai-models)（L1，2024-12-17）；[Rest of World](https://restofworld.org/2026/ai-tokenizer-tax-non-english-languages-cost-inequity/)（L4，页面日期10/26）；[IT之家转载](https://www.ithome.com/1/020/026.htm)（北京10/30 12:25，凤凰网、网易同文）；HN [50250910](https://news.ycombinator.com/item?id=50250910)（25分／1评，热度主要在中文转载）。

## 速览

- Model Spec：[OpenAI 博文](https://openai.com/index/updated-model-spec)、[规范 2026-10-29 版](https://model-spec.openai.com/2026-10-29.html)、[CHANGELOG](https://github.com/openai/model_spec/blob/main/CHANGELOG.md)（L1；版本页横幅称“有更新版本”与 CHANGELOG 顶条矛盾，未解）。
- Gemini 编辑文件：[Google 博文](https://blog.google/innovation-and-ai/products/gemini-app/edit-files-directly/)、[Workspace 更新](https://workspaceupdates.googleblog.com/2026/10/create-preview-and-edit-files-directly-in-the-Gemini-app.html)（L1）。
- Wikimedia：[基金会新闻](https://wikimediafoundation.org/news/2026/10/30/wikimedia-foundation-amicus-brief-new-york-times-v-open-ai-case/)、[动议函与意见书 PDF](https://commons.wikimedia.org/wiki/File:26.10.29_WMF_motion_and_amicus_In_re_OAI.pdf)（L1，SDNY 1:25-md-03143，需法院批准）。
- Waymo：[Waymo 博文](https://waymo.com/blog/2026/10/toyota-bz4x-partnership)（L1）。
- Alphabet：[Google CEO 博文](https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q3-2026/)（L1；abc.xyz IR 被质询页拦截）。
- Amazon：[Q3 新闻稿](https://ir.aboutamazon.com/news-release/news-release-details/2026/Amazon-com-Announces-Third-Quarter-Results/)（L1）。
- Style Sketch：[TechCrunch](https://techcrunch.com/2026/10/29/meta-quietly-launches-an-ai-fashion-app-called-style-sketch/)（L4）、[App Store 印尼区](https://apps.apple.com/id/app/style-sketch-outfit-planner/id6794461626)（L1）。
- Fowler：[LLM Voice](https://martinfowler.com/bliki/LLM-Voice.html)（L2）；[HN 50290177](https://news.ycombinator.com/item?id=50290177)。
- 未收：Gemini API Computer Use 正式可用（圈内）、Chrome 自动填表读 Gmail（窗口前，默认开关未写）、Sierra ARR（圈内、自报）、腾讯 Hy-Nomas（热度低）、Jules 关停（无官方一手，资料账号未认证）。不收：WIRED 五角大楼承包商报道（涉外国军方、匿名单源）、DeepSeek Cyber（L5）、OpenAI 前沿模型监控（1020 已发同一线）、AA HLE-Diamond 与 Carbon（1028 已发）。

## 输入与取证

派工单 `.handoff/2026-10-29-1029-evidence.md`（文件名日期为工具按期号生成）。A/B 组 Claude Sonnet 5.5（36、21分钟），C/D 组 Claude Haiku 5.5 effort high（6、8分钟）。取证全由 Claude 系代理完成，反向核验须逐行核对事实清单与存档。opencli 已连接。A 组按要求未下载、未运行恶意扩展与修复脚本，未访问 IOC。

## 扫描说法勘误（已核原文）

- Cursor：“最多4100用户”在所有已存来源（博文中英文、官方帖、各媒体与研究者文章）中找不到，官方一致为 5,000；“bug、大多失败”原文为 “due to errors in the malicious code, this step would fail” 与 “most macOS and Windows hosts”；“66 行加载器、不需开始对话”出自 OpenSourceMalware，不是 Cursor；ChatGPT 扫描称官方页受限不能核，实际可读。
- CLEAR：中文转载写了“在其中一个 AI 模型中”，没写成所有模型；“CLEAR 与 TII 有关”无一手依据；Rest of World 页面日期是10/26 不是10/28（10/28 是 Computerworld）；“1.5至5倍”是 Rest of World 写法，原报告 1.5–3.1 倍加一例外。
- Wikimedia：意见书主张的是 ChatGPT 的 RAG（检索）功能使用副本不属合理使用，不是“复述原文”；且是申请递交，法院未批准。
- Alphabet：“AI Mode 变现低于传统搜索”原话无比较基准，只有第三方逐字稿。
- Style Sketch：iOS 与 Android 都已在三国上架，“仅 iPhone”只是 iOS 页标签。

## 风控

- Cursor 一条不写攻击者 IP、端口和命令；批注图不放自查命令截图以外的 IOC。
- 标题与封面不写“政府”；WIRED 五角大楼稿不进任何位置。

## 待办与限制

- Cursor 博文最早的 Wayback 快照晚于官方帖（北京15:46），之前是否改过无证据。
- Reddit 各题检索均被登录页拦截。
- CLEAR 报告未给中文相对英文的比值，也没公开数据；本号未自行跑分词器。
