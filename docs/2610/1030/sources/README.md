# 1030 来源与制作状态

期号1030（上期1029+1）；线索为 Develata 回贴的 ChatGPT 滚动扫描（北京 10-31 06:05）与 WorkBuddy 扫描（1031-06，北京 06:00），只作线索。Develata 10/31 按推荐定主帖：两家200美元档改规则（Claude Ultra＋OpenAI Pro 200，跟进0929）→ 苹果国行 Apple 智能接入千问。备选未取证主帖：谷歌太空 TPU（改入速览）、YC Minecraft 智能体实验、Reddit 搜索引荐（改入速览）、nesformer（改入速览）。

## 主帖

1. 两家200美元档改规则：
   - Claude：[定价页](https://claude.com/pricing)、[Ultra 帮助页](https://support.claude.com/en/articles/17728952-what-is-the-ultra-plan)、[Max 帮助页](https://support.claude.com/en/articles/11049741-what-is-the-max-plan) 与其 [2026-10-24 快照](https://web.archive.org/web/20261024051700/https://support.claude.com/en/articles/11049741-what-is-the-max-plan)、[Fable 额度页](https://support.claude.com/en/articles/15424964-claude-fable-models-on-your-plan)（L1）；Anthropic 官方海外社媒帖（北京 10-31 01:29:07，存 `a-fxtweet.json`）；订阅产品负责人 Mike Overell 帖（L2，01:31:27，`a-fx-mboverell.json`）；Kahn v. Anthropic PBC，N.D. Cal. 3:26-cv-05763 [案卷](https://www.courtlistener.com/docket/)（CourtListener，`a-cl-docket.txt`）、[起诉状](https://storage.courtlistener.com/recap/gov.uscourts.cand.472161/gov.uscourts.cand.472161.1.0.pdf)、驳回起诉动议（ECF 39，`a-cl-doc39-pdf.txt`）。
   - OpenAI：[ChatGPT Pro 帮助页](https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers)（L1，updatedAt 北京 10-30 01:02:55；Cloudflare 拦 curl，渲染存 `b-pro-help-rendered.md`）；员工 Tibo 撤回帖（L2，北京 10-30 00:56:07，`b-fx-2115850175868727621.json`）；0929 期旧页面存档 `docs/2609/0929/sources/e-pro-tiers-help.md`。
   - 夸大实例：投资界“突发，ChatGPT Pro所有档位额度，官宣翻倍”（https://news.pedaily.cn/202610/571121.shtml ）；人人都是产品经理“Claude宣布取消5小时限制！首批Ultra用户已「解封」”（https://www.woshipm.com/ai/6487179.html ）。
2. 苹果国行 Apple 智能：[苹果中国新闻稿《Apple 智能即将登陆中国》](https://www.apple.com.cn/newsroom/2026/10/apple-intelligence-features-are-coming-to-users-in-china/)（L1，RSS 条目 updated 北京 10-31 04:59:25；IT之家 05:02 已报道）；Apple 智能中国页（`c-apple-ai-china-page.txt`）；阿里声明只有媒体转载（L4，新浪转21世纪经济报道，北京 07:57）；IT之家 05:02 首条 https://www.ithome.com/1/020/389.htm ；夸大实例新浪转文汇报 https://k.sina.cn/article_5953740931_162dee083067043w24.html 。

## 速览

- 太空 TPU：[Google 博文](https://blog.google/innovation-and-ai/models-and-research/google-research/project-suncatcher-satellite-response/)（L1，跟进1002）。
- Reddit：[Q3 股东信](https://s203.q4cdn.com/380862485/files/doc_financials/2026/q3/v2/Q3-26-Shareholder-Letter.pdf)、[SEC 8-K 新闻稿](https://www.sec.gov/Archives/edgar/data/1713445/000171344526000114/earningspressreleaseq326.htm)（L1）；The Verge 转述 CEO 归因 AI Mode（L4，未写）。
- 训练退出：[OpenAI 公告](https://openai.com/index/helping-websites-express-ai-training-preferences/)（L1）；[Google](https://developers.google.com/search/blog/2026/10/ai-training-control)、[Microsoft](https://blogs.bing.com/search/2026/10/your-content-your-choice-clearer-preferences-for-training-and-grounding/)（L1，未写）；[IETF 草案](https://datatracker.ietf.org/doc/draft-kelley-aipref-training-preference-signal/)（个人提交）。
- 暴力转介：[OpenAI 公告](https://openai.com/index/community-safety-report-and-new-protections-for-teens/)（L1）。
- NES：[nesformer 仓库](https://github.com/dbreunig/nesformer)（L1）；[HN 50306442](https://news.ycombinator.com/item?id=50306442)。
- Proton：[404 Media](https://www.404media.co/protons-ceo-cancels-ai-re-interviews/)（L4）。
- 未收：YC Minecraft/Mindustry（条目压不进40字且 harness 限定不能删）、Amazon Alexa 购物（仅美国、热度无）。不收：Cursor（1029 已发）、维基媒体意见书（1029 速览已收）、众议院“AI 泡沫纾困”决议（涉外国政治）、黑石与 Anthropic 评级（FT 付费墙、融资）、DeepSeek 融资（L5）、SigNoz SRE 文（2月旧文）、GitHub 故障（无根因）。

## 输入与取证

派工单 `.handoff/2026-10-30-1030-evidence.md`（文件名日期为工具按本机美东日期生成）。A/B/C 组 Claude Sonnet 5.5（26、27、19分钟），D/E 组 Claude Haiku 5.5 effort high（16、15分钟）。取证全由 Claude 系代理完成，反向核验须逐行核对事实清单与存档。opencli 扩展未连接，截图走 `barking shot`；B 组改用 bun 驱动有界面 Chrome（临时 profile、未登录）过 Cloudflare 取 help.openai.com。

## 扫描说法勘误（已核原文）

- Claude：“20x 变 10x 是降额”不成立，旧 20x 是每次会话（5小时）倍数，新 10x 是每周倍数，官方帖称每周上限不变；“集体诉讼 Kahn v. Anthropic、3:26-cv-05763、6/14 提起、与 Smith 案合并”属实；WorkBuddy 的“驳回动议”应为“请求驳回起诉的动议”；11/6 听审是否照常不明（10/30 有无正文的命令记录）。
- OpenAI：撤回发生在北京 10-30 凌晨（员工帖 00:56、帮助页 01:02），不是扫描说的“10/30 当天凌晨撤回 10/30 的降额”那样同步；“翻倍”只对 Pro 200 低额度者成立。
- 苹果：“不跳转独立 App”是 IT之家写法，苹果稿无；“仅限中国购买或定位在中国”“不用于训练”出自阿里声明（L4）；机型是 iPhone 16 及后续、Duo、15 Pro、15 Pro Max、Air，不是“15 Pro 及以上”。
- Suncatcher：1 kW、20分钟、4颗 TPU 均不在谷歌三篇页面。
- Reddit：股东信未点名谷歌；登录测试已定于 Q4、在美国以外部分市场，不是“考虑”。
- robots.txt：Anthropic 只是员工称“已开始实施”；草案为个人提交。
- Proton：“全员”应为“几乎全公司”；“90% 代码由 AI 写”原话在付费墙后，未找到一手来源。

## 风控

- 标题与封面不写“政府”。速览“暴力转介”为 OpenAI 自报的执法转介数据，不属既往受限两类，Develata 10/31 确认保留。
- 起诉状截图含原告姓名（公开案卷、案名即含），不收集其他个人信息。

## 待办与限制

- D 组向 sec.gov 发请求时在 User-Agent 里带了 Develata 的邮箱（SEC 要求联系方式），共五次，已发生、无法撤回；存档与证据文件中无该邮箱。
- 已删除：各组回报的待删除文件（空壳、被拦页、未引用存档）与作废的批注底图、截图。B 组 `b-cdp-shot.mjs`、A 组 `a-xcard.ts` 两个自写脚本保留作记录。
