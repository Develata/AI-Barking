# 1102 来源与制作状态

期号1102（上期1101+1）；线索为 Develata 回贴的 WorkBuddy 扫描（1103-06，北京 11-03 06:00）与 ChatGPT 滚动扫描（第 11 轮，北京 11-03 13:09），只作线索。Develata 11/03 选定主帖：Astra 改 Qwen 权重 → Anthropic 招股书修订版【跟进1026速览】；速览“推荐全收”（message threads 因限定词压不进 40 字未收，见 fact-check 速览节）。

## 主帖

1. Astra 改权重：[作者博文](https://rikiyat.github.io/blog/weight-space-programming/)（L2，页面标 Published November 2, 2026，无时刻；第一作者帖北京 11-03 01:51:48；`a-blog.*`）；[实验日志仓库](https://github.com/RikiyaT/lm_understand-logs)（L2，含 Astra 任务提示 `a-logs-exp8-p50c-t001-prompt.txt` 与结果 `a-logs-exp8-eval-RESULTS_excerpt.md`）；[Qwen3.5-0.8B 官方模型卡](https://huggingface.co/Qwen/Qwen3.5-0.8B)（L1，`a-qwen-card.*`）；批评 [Sean Goedecke](https://www.seangoedecke.com/llms-are-not-superhuman-at-reading-and-writing-neural-network-weights/)（L6 线索）；[HN 50341174](https://news.ycombinator.com/item?id=50341174)。
   - 夸大实例：凤凰网 APPSO“GPT-6 Astra爆改Qwen大脑：只动0.016%参数，数学成绩疯涨”（北京 11-03 15:49）；IT之家“数学成绩提升至原来 6 倍”（10:00）；KuCoin/TechFlow 快讯 “enhance Qwen's mathematical ability by 6x”（evidence-A A7）。
2. Anthropic 招股书修订版：[EDGAR S-1/A](https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm)（L1，受理北京 11-02 22:24:24；全文 txt 超 1 MB 不入库，相关章节摘录 `b-s1a-excerpts.txt`）；[首版 S-1](https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm)（L1，对照“承诺”栏空白）；[CNBC](https://www.cnbc.com/2026/11/02/anthropic-ant-s-1-quarter-financials.html)（L4）；[Tech Policy Press](https://www.techpolicy.press/the-sec-should-block-anthropics-ipo-to-protect-the-public/)（观点文）。
   - 夸大实例：[界面新闻](https://www.jiemian.com/article/15287521.html)“近万亿长期算力负债”（北京 11-03 15:23）；[凤凰网转 IT之家](https://tech.ifeng.com/c/8xfSgBuz3Bx)“隐性债务近9000亿美元”（10-31，S-1/A 前，口径引自彭博）。

## 速览

- Gemini：[Gemini API 弃用页](https://ai.google.dev/gemini-api/docs/deprecations)与[更新日志](https://ai.google.dev/gemini-api/docs/changelog)（L1，跳转 aistudio.google.com/docs/…；`c-gemini-*-aistudio.txt`）。
- 版权诉讼：[纽约时报起诉书](https://nytco-assets.nytimes.com/2026/11/new-york-times-brave-complaint-11-2-26.pdf)（S.D.N.Y. 1:26-cv-09549，Filed 11/02/26；`c-brave-nyt-complaint.*`）；另案[起诉书](https://storage.courtlistener.com/recap/gov.uscourts.nysd.676310/gov.uscourts.nysd.676310.1.0.pdf)（`c-brave-dailynews-complaint.*`）；[The Verge](https://www.theverge.com/tech/1024588/brave-lawsuit-ai-scraping-new-york-times)、[TechCrunch](https://techcrunch.com/2026/11/02/new-york-times-and-other-outlets-sue-web-browser-brave-over-alleged-content-scraping/)（L4）。C 组原先查到的是旧的 News Corp 案，Claude 改查补齐。
- 无声输入：[Subvocal 官网](https://subvocal.company/)与预购条款（L1，渲染文本 `c-subvocal-*-opencli.txt`、`c-subvocal-site-expanded.txt`）；[HN 50340088](https://news.ycombinator.com/item?id=50340088)。
- 高盛：[彭博社](https://www.bloomberg.com/news/articles/2026-11-02/goldman-restricts-traders-and-bankers-from-using-some-ai-tools)（L4，付费墙，未绕过；正文经 Yahoo、PYMNTS 公开转载）。
- Git：[OpenAI《Scaling Git at OpenAI》](https://openai.com/index/scaling-git-at-openai/)（L1，`c-c4-openai-pando.txt`）；[HN 50345924](https://news.ycombinator.com/item?id=50345924)。
- 电网：[OpenAI Grid Protectors](https://openai.com/index/grid-protectors-program/)（L1）；[Georgia Power 新闻稿](https://www.georgiapower.com/news-hub/press-releases/georgia-power-to-serve-openai-project-in-effingham-county.html)（L1）。
- 安全事件：[WorkOS 安全公告](https://workos.com/security/bulletins/october-30th-2026-data-security-incident)（L1）；[HN 50345712](https://news.ycombinator.com/item?id=50345712)。
- 不收：Claude message threads（限定词压不进 40 字、HN 1 分）；OpenAI LifeVal、腾讯混元 Hyra 2.0、Deep Principle Hydrogen-1.0（未选）；Grok 额度砍半（无官方数字）；Oracle、Meta 融资与 AI 股下跌（行情）；OpenAI“AI 算力园区”（风控）；GPT-6.2 Astra 灰测、Mythos 5.5 泄露、DeepSeek Creator Mode（无一手）；额度重置。

## 输入与取证

派工单 `.handoff/2026-11-02-1102-evidence.md`（文件名日期按期号生成）。opencli 预检正常。A/B 组 Claude Sonnet 5.5（约25、24分钟，harness 计时；B 组另补截两张约5分钟），C/D 组 Claude Haiku 5.5 effort high（约8、6分钟）。Claude 直接核读：博文 TL;DR 与实验8、S-1/A 经营亏损与承诺段、EDGAR 受理时间（submissions JSON）、Brave 两份起诉书。

热度（北京 11-03 19:00–19:15，HN）：Astra 改权重 2692 分 813 评（HN 首页 AI 类第一）；Anthropic 相关帖 Tech Policy Press 364/268、BI 内部用量 330/251、“growth flattened” 210/239、CNBC 233/274；Subvocal 708/343；Pando 236/151；WorkOS 70/9；高盛 26/4；Brave 两帖 14–15 分。Reddit 全部被登录墙拦下。

## 扫描说法勘误（已核原文）

- Astra：作者单位四人均为 MIT CSAIL，但这是作者个人博客，不是 MIT 官方稿；ChatGPT 扫描的“Gemma 6.8%→7.0%”只是 5 轮与 50 轮两行，另两行 7.6、7.7；博文数字来自含失败示例的第 3 版提示。
- 招股书：“8954亿首次出现”属实（首版空白）；“一年内到期 700 亿”原文 698 亿且其中 43 亿附条件；“2138 亿可取消”原文是附终止权或前提条件；“1000 亿 9/30 后新签”是 1002 亿与 4512 亿并列相加，不是其中一部分；“现金七倍”未找到出处；“内部 AI 用量超工资”“5%–7% 违规客户收入”首版已有，不是修订版新增；“中文稿多把净亏写成经营压力”在 10 篇样本里未见。
- Brave：被点名的 Anthropic 等五家不是被告。
- Grid：“约 70 万户”是 1,000 MW 的规模换算；“9.5 亿美元节省”是 Georgia Power 整个大用户组合的预计。
- WorkOS：框架名 ARTEX 只见于 HN 上自称 CEO 的评论。
- message threads：35% 的前提是“对多数 Claude Code 用户默认开启”。

## 风控

- “美国证监会”为中性申报事实，未涉外国政府政治或美方点名中国公司；Develata 11/03 终审确认保留。
- 正文与卡片未出现大陆无法访问的平台名；作者帖按“研究者公布”写，不写平台。
- 截图与存档未含抓取者身份；C 组 AI Studio 渲染文本里的登录邮箱已替换为占位符。

## 未入库的大文本

`b-s1a.txt`（1.67 MB）、`b-s1-first.txt`（1.64 MB）、`a-logs-fulltree.json`（2.6 MB）超过 1 MB 提交上限，已删去：前两份可由离线存的 `b-s1a.html`、`b-s1-first.html` 经 `barking html2txt` 重建（入库的是相关章节摘录 `b-s1a-excerpts.txt`）；后者是 GitHub 文件树 API 原始输出，可重取。取证记录里引用的这两份 txt 行号以重建结果为准。
