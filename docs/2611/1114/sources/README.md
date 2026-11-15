# 1114 来源与制作状态

期号1114（上期1113+1）；线索为 Develata 回贴的两份扫描：ChatGPT 定时任务的 stateful rescan（北京 11-15 19:06）与 HTML 选题扫描 1115-06（北京 11-15 06:00）。Claude 对照“已发过”、往期 `fact-check.md` 与“近 3 天速览已收”去重、现查 HN 热度（Algolia，北京 11-15 20:20 前后）后列候选；Develata 定 Lovable 与 Deepnote 两条主帖。速览候选两份扫描都不足，Claude 另扫 HN 40 小时内 50 分以上的帖子，并派一个 Sonnet 子代理查各厂商官网与 RSS，补出速览条目。

派工单 `.handoff/2026-11-14-1114-evidence.md`（文件名日期是 `barking handoff` 按美东算的；派工单正文里它写的“本机时区为美东”已改为北京时间，本机为 macOS、UTC+8）。派工前 `opencli doctor` 通过。三组执行方：A、B 为 Claude Sonnet 5.5（`claude-sonnet-5-5`），C 为 Claude Haiku 5.5（`claude-haiku-5-5`），北京 11-15 约 20:54 派出。取证全由 Claude 家族完成，反向核验时由 Codex 逐行核对事实清单与存档。

## 主帖

1. Lovable 应用数据暴露研究：[Crissy Field 研究原文](https://www.crissyfield.de/research/lovable-en/)（英文页无日期、无个人署名；Last-Modified 北京 11-04 06:05）、[德文版](https://www.crissyfield.de/research/lovable/)、[HN 50499017](https://news.ycombinator.com/item?id=50499017)（北京 11-15 01:43 提交，866 分 / 338 评，11-15 20:55）、[Lovable 安全负责人 HN 评论](https://news.ycombinator.com/item?id=50502105)（自称，北京 11-15 07:19）、[Handelsblatt](https://www.handelsblatt.com/technik/ki/lovable-hat-die-beliebte-ki-coding-plattform-ein-sicherheitsproblem/100264923.html)（付费墙，只存可见部分，页面日期 03.11.2026 16:39 德国时间）、[Lovable 条款](https://lovable.dev/terms)（Last Updated: August 28, 2026）、[Lovable founders 页](https://lovable.dev/en/founders)、[Lovable 11-10 博客](https://lovable.dev/blog/find-it-fix-it-ship-it)（不是对研究的回应）。
   - 30% 的分母是 24,664 个“加载即发数据库记录”的网站；原文 Limitations 明说不应读作确认的泄露率、只适用于这 24,664 个、Common Crawl 样本不代表全部 Lovable 应用。原文只查了 Supabase，Lovable 自家数据库未查（正文因字数未写）。
   - 首报日期两处不一致：研究页写 Handelsblatt 11-02，Handelsblatt 页面显示 11-03；自称作者的 HN 账号北京 11-02 23:45 已提交同一 URL。正文只写“11月初”。
   - Lovable 回应：HN 署名评论（身份自称）与 Handelsblatt 题注“Laut Lovable ist das ein branchenweites Problem.”；没有找到正式声明稿。官方 X 与 Reddit 未查成。
   - 自称作者的 HN 评论称用本地模型筛出近 4,000 个“泄露隐私敏感数据”的站点，自称占比 15.9%，与报告已知分母都对不上，方法未公开（L6），正文不写。
   - 条款“No Sensitive Data”一条是本号另查的，研究原文只概括“条款把责任划给客户”。
   - 去重：1107 期 Apiiro（公开仓库里的 Claude Code 本地配置含密钥）、1111 期 Claroc（Grok Build 应用未登录可打开）与本条对象、方法都不同，见 evidence-A 第 A10 行。
2. Deepnote Agent Initiative Benchmark：[官方页](https://deepnote.com/research/benchmark)（无页面日期）、[GitHub 仓库](https://github.com/deepnote/agent-initiative-benchmark)（“Present our results”提交北京 11-13 15:48）、[评审提示 evaluate_agent_run.txt](https://github.com/deepnote/agent-initiative-benchmark/blob/4d66be81b4fc44ba0fa0fc48320be3ebfa717e3e/src/benchmark/prompts/judge/evaluate_agent_run.txt)、[HF 数据集](https://huggingface.co/datasets/deepnote/pagemantle-company-dataset)、[新智元（网易号）](https://www.163.com/dy/article/LBN9SFNU0511ABV6.html)（北京 11-14 17:16）、[新浪财经转载](https://finance.sina.com.cn/stock/t/2026-11-14/doc-iniyszsz2786927.shtml)、创始人帖（北京 11-13 22:58，fxtwitter JSON 存档）、[HN 50485628](https://news.ycombinator.com/item?id=50485628)（2 分 / 1 评）。
   - 热度主要在中文侧与创始人帖（387,707 次浏览）；HN 很冷，热度门槛偏弱，记一笔。同花顺转载未找到。
   - “Gemini 那10次沉默都是超时”是本号推算：没发消息 17 + 10 = 27，等于官方“发消息前撞上 180 秒”的 27 次。逐题结果未公开。Grok 同样的算术对不上（没发消息 16 + 6 = 22，官方超时 18，官方未解释；evidence-b 第一节写成 16 + 4 = 20 有误，以整表为准），正文不写。
   - 评分规则出自公开仓库 v1.0 的评审提示，无法证明官方 600 次运行用的是同一份。
   - 官方页内部口径不一：小节标题 “Two OpenAI models beat the newest Fable” 对正文 “inside Sol’s”；表里 Opus 5.5 Helped 40/40 对正文 “a step behind at helping”。正文不写。
   - 新智元稿另有“花430倍的钱，只多11%的分数”（官方是 10 分制差 1.1 分）、“多花1.8倍”（官方 1.8×）、“9成都能搞定”（官方只统计发出消息）等出入，见 evidence-b 第五节；正文只点标题。

## 速览

- [Google Research：Antigravity /teamwork](https://research.google/blog/advancing-science-engineering-with-antigravitys-teamwork-feature/)（11-13，HN 206 分）。Navier–Stokes 句原文是复现某近期证明的形式化，扫描写成“复现已有证明”不准。
- [Project Zero 披露政策](https://projectzero.google/2026/11/disclosure-policy.html)（11-13，HN 271 分）。
- [Gemini × Google Wallet](https://blog.google/innovation-and-ai/products/gemini-app/google-wallet-gemini-app/)（北京 11-14 01:00；HN 未找到帖子）。官方未写年龄或账户类型限制。
- [DeepSeek API 更新日志](https://api-docs.deepseek.com/updates)（11-13；HN 未找到帖子）。扫描的“澎湃：1:1 复刻 DeepSeek App”未找到该稿。
- [EFF：隐私概念又被转移](https://www.eff.org/deeplinks/2026/11/tech-companies-are-trying-shift-entire-concept-privacy-again)（北京 11-14 05:29，HN 563 分）。
- 未收：Christiano 多伦多讲座（[Ars Technica](https://arstechnica.com/ai/2026/11/ai-is-the-most-likely-reason-youll-die-openai-board-member-warns/)，引语未与录像核对，Ars 把名字拼作 Cristiano；HN 无帖）；英伟达“借投资初创制造短缺”（只有 TechPowerUp 转述《华尔街日报》）；Blackwell 芯片指纹、谷歌学校 AI 概览政策（风控）；意大利罚 Meta；ggml/llama.cpp 1.0、JAX 版 DeepSeek V3（圈内工程）；Lexifina 用 Lean 查美国税法（11-10）；Claude Meetings、Claude Code Projects（1111、1113 已判不收）。

## 备注

- 工具问题：`barking heat` 遇到已存在的 `c-heat.json` 拒绝覆盖，C 组 C3、C4、C6 的热度改用 curl 查 Algolia（未找到帖子）。
- B 组截图 13–24 用尽后顺延到 25–29（C 组本期不截图）。
- 交付前已删除：A 组列出的被拒、404 或空抓取残留（`a-handelsblatt`、`a-crissy-kickbase-2017`、`a-nvd-cve`、`a-fourweekmba`、`a-cybernews`、`a-sifted`、`a-techzine`、`a-lilox` 各自的 `.html/.headers`；其中 Cybernews、Sifted、Techzine、LILOX 的正文另有 opencli 取得的 `.txt` 保留）、`images/22-repo-judge-prompt.png`（被 27 取代）、`images/judge-rules.png` 与 `images/dn-intervals-raw.png`（重裁前的底图）。fetch-log 与 shots-log 里仍留有这些条目的记录。
- 批注底图 `judge-rules-raw.png`、`lovable-terms-raw.png` 由 Claude 用 `barking shot render` 重渲原页后拼接（GitHub raw 视图、lovable.dev/terms，Chrome 渲染为深色底），记在 `d-shots-log.jsonl`；其余底图从取证截图裁出。
