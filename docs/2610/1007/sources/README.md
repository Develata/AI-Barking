# 1007 来源入口

事实核验见 [fact-check.md](fact-check.md)；取证清单见 `evidence-a.md`（犹他州）、`evidence-b.md`（Mistral）、`evidence-c.md`（Anthropic）、`evidence-d.md`、`evidence-e.md`（速览）；总览与抓取日志见 [evidence.md](evidence.md)、[capture-log.md](capture-log.md)（分组日志 `capture-log-a.md` … `capture-log-e.md`）。存档前缀 `a-`…`e-` 对应各组；PDF、HTML、图片原件只存本地与网盘，清单见 `offsite.tsv`（发布后生成）。

## 主帖一：Claude Haiku-5.5

- Anthropic 发布页：https://www.anthropic.com/claude-haiku-5-5 （`f-haiku-5-5.html` 不入库，文本 `f-haiku-5-5.txt`；页面日期 2026-10-07，无时刻；是滚动动画页，无头 Chrome 只渲出首屏，脚注与定价表无法截图，正文以 curl 取得的文本为准）
- 系统卡：https://www.anthropic.com/claude-haiku-5-5-system-card （PDF `g-haiku-system-card.pdf`，13 MB，不入库；文本 `g-haiku-system-card.txt`；§8.4 Terminal-Bench 4.0 在 PDF 第 115–116 页）
- 定价文档：https://platform.claude.com/docs/en/about-claude/pricing （`g-pricing-doc.txt`，截图 51）；迁移指南：https://platform.claude.com/docs/en/models/haiku-5-5/migration-guide （`g-migration-guide.txt`，截图 52）；effort 文档：https://platform.claude.com/docs/en/build-with-claude/effort （`g-effort-doc.txt`）
- Artificial Analysis：Haiku-5.5 Max https://artificialanalysis.ai/models/claude-haiku-5-5 （`g-aa-haiku-max.txt`）、中档 https://artificialanalysis.ai/models/claude-haiku-5-5-medium （`g-aa-haiku-medium.txt`）、发布页 https://artificialanalysis.ai/models/releases/claude-haiku-5-5 （`g-aa-haiku-release.txt`）；GPT-6 Luna Max https://artificialanalysis.ai/models/gpt-6-luna （`g-aa-luna-max.txt`）。抓取北京 2026-10-08 13:05–13:13。
- 热度与社区（L6，不入正文）：HN https://news.ycombinator.com/item?id=49996437 （770 分 / 380 评论，北京 10-08 13:02，`g-hn-haiku.json`、`g-hn-haiku-item.json`）；VentureBeat 同日稿 https://venturebeat.com/technology/anthropic-launches-claude-haiku-5-5-with-90-api-price-reduction-matching-gpt-6-luna （curl 429，未存档，只经 firecrawl 读到）。
- 线索：ChatGPT 扫描（北京 10-08 11:06）称 AA 指数 43、约 243.4 tok/s，与 AA 页一致。

## 主帖二：犹他州 Nolla Health

- 协议原件：https://commerce.utah.gov/wp-content/uploads/2026/10/RMA-Nolla-Health.pdf （50 页，`a-rma-nolla-health.pdf` 不入库；文本 `a-rma-nolla-health.txt`；Section 3.E 在 PDF 页 2，4.10–4.11 在页 40–41；PDF 元数据标注 “uncertified copy”，见 `evidence-a.md` A1）
- 州方试点页：https://commerce.utah.gov/ai/regulatory-relief-4/authorized-pilots/ （`a-oaip-authorized-pilots.txt`，抓取北京 10-08 01:30）
- 州方新闻稿（2026-10-05）：https://commerce.utah.gov/2026/10/05/utah-enhances-pro-human-ai-initiative-with-new-healthcare-pillar-adds-new-pilots-to-its-regulatory-sandbox-and-partners-with-third-party-evaluators/ （`a-commerce-healthcare-pillar.txt`）
- 第三方评估方页：https://commerce.utah.gov/ai/third-party-evaluators/ （`a-oaip-third-party-evaluators.txt`）
- Nolla 官网博客：https://www.nollahealth.com/blog/ai-prescriptions-utah （`a-nolla-blog-utah.txt`）；官网产品页与常见问题：https://www.nollahealth.com/ai-prescriptions （`a-nolla-ai-prescriptions.txt`、`a-nolla-faq.txt`）
- PR Newswire：https://www.prnewswire.com/news-releases/nolla-health-announces-nations-first-ai-to-issue-initial-prescriptions-302897659.html （`a-prnewswire-body.txt`）
- 创始人帖（浏览量与原话；`a-wenus-x-thread.md`、`a-wenus-fxtwitter.json`；卡片截图未取得）
- 洛杉矶时报转载的彭博稿：https://www.latimes.com/business/story/2026-10-05/ai-startup-prescribes-acne-medication-without-doctors-direct-oversight （`a-latimes-body.txt`；彭博原页 403，未存档）
- Doctronic 先例：医疗执照委员会信 https://commerce.utah.gov/wp-content/uploads/2026/04/doctronic-letter-from-medical-board.pdf ；州方回信 https://commerce.utah.gov/wp-content/uploads/2026/04/Medical-Board-Doctronic-Response.pdf （正文未用，留作背景）
- 热度：HN https://news.ycombinator.com/item?id=49981197 （134 分 / 123 评论，北京 10-08 01:52）；创始人帖浏览量 2,292,523（10-08 01:46）。AIHOT 与 Develata 当日页均未收录该条。

## 主帖三：Anthropic 网络验证计划

- 公告：https://www.anthropic.com/news/cyber-verification-program （`c-` 存档，见 `c-manifest.tsv`；published_time 2026-10-06T19:00:00Z）
- 帮助中心：https://support.claude.com/en/articles/14604842-cyber-verification-program
- Project Glasswing：https://www.anthropic.com/glasswing ；5/22 更新 https://www.anthropic.com/research/glasswing-initial-update ；6/2 扩大 https://www.anthropic.com/news/expanding-project-glasswing （口径与本次不同，正文未用）
- Opus 5.5 系统卡 §3.3.2：https://www.anthropic.com/claude-opus-5-5-system-card （67.6% 基线，PDF 不入库）
- Reuters：https://www.reuters.com/legal/litigation/anthropic-opens-its-most-powerful-ai-models-more-security-teams-2026-10-06/ （正文由 firecrawl 取得，背景）
- 中文报道中的“解除限制”写法：钜亨号、网易号·安全圈（`evidence-c.md` C7，仅作夸大实例，正文不引）

## 备用（本期撤下）：Mistral-Large-4

- 博客：https://mistral.ai/news/mistral-large-4/ （`b-blog.txt`；图表原图 `b-assets/` 不入库；发布后图表被替换过，见 `evidence-b.md` B2）
- 文档页：https://docs.mistral.ai/models/mistral-large-4-0 （现版 52B；web.archive.org 快照 `b-wayback-doc-*.html` 只用于看“以前写了什么”：北京 10-06 21:17、22:26 为 49B，10-07 16:29 为 52B）
- 定价页：https://docs.mistral.ai/inference/pricing ；生命周期页：https://docs.mistral.ai/models/model-lifecycle
- Artificial Analysis：模型页 https://artificialanalysis.ai/models/mistral-large-4 ；评测文章 https://artificialanalysis.ai/articles/mistral-large-4-france-ai （图 “Artificial Analysis Cyber Index” 与 “Score by Benchmark”）
- Hugging Face 检索（匿名 API，北京 10-08 01:32）：https://huggingface.co/api/models?author=mistralai&search=large-4
- Arena：https://arena.ai/leaderboard/code/webdev/overall （Code Arena WebDev 第 45，1534 分，正文未用）
- 热度：HN https://news.ycombinator.com/item?id=49977979 （1970 分 / 1173 评论，北京 10-08 01:47）；AIHOT 热点榜第 1（277，北京 10-08 01:16）

## 速览

- GPT-6 Intelligent UI：https://openai.com/index/gpt-6-for-everyone/ （`g-openai-gpt6-intelligent-ui.txt`，HTML 不入库；页面日期 2026-10-07；Help Center 模型页只经 firecrawl 读到、未存档，口径与公告不同，见 fact-check 速览）
- Anthropic 月度 API 额度：同 Haiku-5.5 发布页 “Further updates”（`f-haiku-5-5.txt`）
- SynthID：https://blog.google/innovation-and-ai/models-and-research/google-deepmind/synth-id-ai-content/ ；https://synthid.com （`d-synthid-blog.txt`、`d-synthid-site-rendered.txt`）
- Nemotron：https://huggingface.co/blog/nvidia/nemotron-ioi-and-imo-2026 ；arXiv https://arxiv.org/abs/2609.02849 、https://arxiv.org/abs/2609.10712 （`d-nemotron-blog.txt`、`d-arxiv-*`）
- openTPU：https://github.com/FeSens/openTPU （`d-opentpu-readme.md`）；HN https://news.ycombinator.com/item?id=49980715
- METR：https://metr.org/blog/2026-10-06-ai-systems-could-cover-up-misbehavior/ （`d-metr-blog.txt`）；issue #4318 https://github.com/UKGovernmentBEIS/inspect_ai/issues/4318
- Google × Constellation：https://blog.google/company-news/why-were-backing-americas-existing-nuclear-plants/ ；https://www.constellationenergy.com/news/2026/10/google-and-constellation-announce-landmark-agreement-to-bring-890-mw-of-new-nuclear-capacity-to-pjm-grid.html （`e-google-blog-nuclear.txt`、`e-constellation.txt`）

## 取证过程说明

- 线索：ChatGPT 两份（北京 10-07 16:56、23:10）、WorkBuddy 两份（10-07 18:00、10-08 00:03）、AIHOT 热点榜（10-08 01:16）。扫描说法与一手页面不符之处见 `fact-check.md` 的“选题变更记录”。
- 分组取证由 Sonnet 子代理完成（A 犹他州、B Mistral、C Anthropic、D/E 速览），Claude 回读关键页面。B 组的 `twitter-cli` 曾自动尝试读取本机浏览器 Cookie，解密失败、未读到任何凭据，已停用并记入 `capture-log-b.md`。
- 批注底图（`images/41`–`44`、`51`–`54` 的 `-raw`）：41–43 为协议 PDF 页渲染，44 为无头 Chrome 页面截图（均为取证组截图的拷贝）；51–54 为 Claude 用 `g-shot.py`（无头 Chrome、700 CSS px、2 倍像素比）渲染官方文档页与 Artificial Analysis 页后裁切；撤下的 `45`–`48` 与 `55` 保留为备用。
- 单个文件超过 1 MB 的原件（协议 PDF 4.5 MB、Opus 5.5 系统卡 PDF 17.8 MB、`e-unsloth-decision.html` 1.17 MB）只存本地与网盘，不入库；`b-reddit-*.json`、`c-reddit-*.json`、`c-portal-cvp.html` 为拦截页或登录页，不作证据。
