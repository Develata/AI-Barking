# 1006 来源入口

事实核验见 [fact-check.md](fact-check.md)；取证清单见 `evidence-a.md`（Claude 日记报警）、`evidence-b.md`（SemiAnalysis，本期最终未用）、`evidence-d.md`（速览）；数学条由 Claude 直接取证，存档前缀 `m-`。截图记录见 `m-shots.jsonl`；抓取日志见 `capture-log-a.md`、`capture-log-b.md`、`capture-log-d.md`。

## 主帖一：OpenAI 数学手稿

- OpenAI 博文：https://openai.com/index/sharing-ai-progress-in-mathematics/ （2026-10-06；curl 403，正文经网页读取核对，未本地存档）
- 仓库：https://github.com/openai/math （README `m-oai-README.md`；目录 `m-oai-CONTENTS.md`；形式化清单 `m-oai-lean-formalization.yaml`；003 号形式化范围 `m-oai-lean-docs-003.md`；Comparator 挑战文件 `m-oai-QuasiRiemannHypothesis.lean`；拟黎曼猜想论文 `m-oai-qrh-7-8.pdf`，不入库）。唯一提交 adc7f12，`2026-10-06T21:58:50Z`。
- formalization.yaml 规范 v0.4：https://github.com/mathlib-initiative/formalization.yaml （`m-formalization-schema-v0.4.json`）
- IAS 数学与 AI 顾问组（AGMAI）10/6 声明：https://agmai.org/statement-oct6/ （`m-agmai-oct6.txt`）；9/29 建议：https://agmai.org/general-sep29/ （`m-agmai-sep29.txt`）
- 《科学美国人》：https://www.scientificamerican.com/article/openai-unleashes-hundreds-more-math-results-upon-a-field-already-in-shock/ （`m-sciam.txt`，背景，正文未引）
- 流行说法“一夜攻克722个数学难题”：36氪标题，经新浪转载 https://k.sina.com.cn/article_5953466437_162dab0450670beafa.html （`m-sina-36kr.html`，2026-10-07 09:35）
- 热度：HN https://news.ycombinator.com/item?id=49984923 （北京 10-07 约 565 分 / 487 评论）

## 主帖二：Claude 对话被报警

- WINK News：https://www.winknews.com/news/woman-arrested-after-ai-threat-against-lee-county-sheriffs-office-investigators/article_3d4c5915-7015-43c0-b86a-d7fa5eadf958.html （`a-wink.txt`，已脱敏；原始 HTML 不入库）
- WBBH / Gulf Coast News：https://www.gulfcoastnewsnow.com/article/florida-woman-arrest-ai-threat-sheriff-lee-county/73968592 （`a-gulfcoast.txt`，已脱敏）
- Anthropic 隐私政策：https://www.anthropic.com/legal/privacy （`a-anthropic-privacy.txt`）
- Anthropic 隐私中心（训练开关与安全分类器）：https://privacy.claude.com/en/articles/12109829-how-do-i-change-my-model-improvement-privacy-settings （`a-claude-classifier.txt`）
- 逮捕报告原件未取得（查过的位置见 `evidence-a.md` A1）。
- 热度：HN https://news.ycombinator.com/item?id=49961057

## 速览

- Erdős 问题网站：https://www.erdosproblems.com/forum/thread/blog:9 ，转载于 https://terrytao.wordpress.com/2026/10/06/changes-to-the-erdos-problems-web-site/ （`m-bloom-erdos.txt`）
- Claude for Google Workspace：https://claude.com/resources/articles/claude-now-works-in-google-docs-sheets-and-slides （`d-claude-workspace.txt`）
- Mistral Large 4：https://mistral.ai/news/mistral-large-4/ （`d-mistral-blog.txt`）；AA：https://artificialanalysis.ai/models/mistral-large-4 （`d-aa.txt`）
- DeepSeek 融资：https://www.bloomberg.com/news/articles/2026-10-06/deepseek-to-raise-at-least-12-billion-in-tencent-backed-funding （`d-bloomberg.txt`）

## 本期未用

- SemiAnalysis 订阅限额实测：https://newsletter.semianalysis.com/p/anthropic-subscriptions-offer-5x （`b-semianalysis.txt`、`b-semianalysis-post.json`、`b-figures/` 全部 21 张原图；按 token 数约 2.8 倍、按 API 标价折算约 5.6 倍的复算见 `evidence-b.md`）。因改发数学条移出本期。
