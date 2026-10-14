# 1013 来源入口

正文：`../doc_1013_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（Character.ai）、`evidence-b.md`（OpenAI 窃听案）、`evidence-c.md`（Gemini-4-Argon）、`evidence-d.md`（速览）；抓取日志：`capture-log-*.md`、`*-shots-log.jsonl`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high）。耗时：D 约 22 分钟、B 约 171 分钟、C 约 183 分钟、A 约 204 分钟（截图多用 headless Chrome 与 PDF 渲染；中途 Develata 要求改用 opencli 直接截图，已写入 `templates/codex-evidence.md`）。Haiku 第四次试用：8 条全部交回，67 条摘句经脚本在存档中检出；MSRC 产品列表与标题口径的差异由它主动指出。

选题：两份 ChatGPT 扫描（北京 10/14 05:11、11:17）与一份 WorkBuddy 扫描（12:00），Develata 选 Character.ai 破产账本 + OpenAI 窃听案 + Gemini-4-Argon 正式开放（跟进 1001）。扫描勘误：WorkBuddy 的 Codex 守护进程（1012 已发）、隐藏能力论文（1012 速览已收）、“月烧820万”未注口径（Doc 18 ¶27 为净现金消耗）、“员工向彭博爆料只会刷榜”与原文不符。风控排除：FAA 空管 AI、英国上议院 AI 声音、IMF 金融稳定报告、科学家呼吁国际监管机构、Meta 跨国反诈（涉外国政府与政治）。Scion 只进速览（尚无开源版本），下一期视有无更合适的选题再定。

隐私：OpenAI 窃听案当事人是刑事被告。发布正文、卡片不写姓名与国籍；取证截图中姓名与国籍用黑条遮盖；入库的文本存档只把姓名替换为“[当事人]”，国籍词保留原文（司法部原文标题即含国籍）；含姓名的 HTML/PDF 原件只存本地与网盘。

## 一手来源

- Character.ai 官方博客：https://blog.character.ai/an-update-on-character-ai/
- Character.ai 新闻稿（Business Wire）：https://www.businesswire.com/news/home/20261012656385/en/Character.ai-Initiates-Voluntary-Chapter-11-Proceeding-to-Facilitate-Value-Maximizing-Sale-Process
- 破产案卷（D. Del. 26-11603）：https://cases.stretto.com/chartech ；CourtListener https://www.courtlistener.com/docket/74951888/character-technologies-inc/ ；CEO 声明 Doc 18 经 404 Media 文内链接的 DocumentCloud 取得（`a-doc18.txt`）
- 404 Media：https://www.404media.co/character-ais-bankruptcy-documents-show-the-economics-of-ai-dont-work/
- 司法部新闻稿（N.D. Cal.）：https://www.justice.gov/usao-ndca/pr/saudi-national-arrested-installing-surreptitious-audio-recording-device-openai-san
- 刑事控告书（N.D. Cal. 3:26-mj-71329）：CourtListener RECAP（`b-courtlistener-docket-74958115-documents.json`）
- OpenAI Newsroom 声明：https://x.com/OpenAINewsroom/status/2110095403198849132
- 谷歌 Argon 正式开放公告：https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/
- Gemini API 价格页：https://ai.google.dev/gemini-api/docs/pricing
- ADK 优化实验：https://developers.googleblog.com/en/optimizing-a-200000-line-python-codebase-overnight-with-gemini-4-argon/ ；commit https://github.com/google/adk-python/commit/1754b3eeb6cdb2c16e3b30619a12066623df132e
- 速览：苹果 HomeView https://www.apple.com/newsroom/2026/10/meet-apple-homeview-a-new-way-to-enjoy-entertainment-and-manage-the-home/ ；Anthropic Scion https://www.anthropic.com/engineering/redesigning-source-control ；MSRC CVE-2026-103572 https://msrc.microsoft.com/update-guide/vulnerability/CVE-2026-103572 ；研究者博客 https://embracethered.com/blog/posts/2026/visual-studio-copilot-remote-code-execution/ ；XBOW https://xbow.com/blog/xbow-built-a-chrome-full-chain-exploit
