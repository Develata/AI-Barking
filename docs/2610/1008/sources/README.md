# 1008 来源入口

事实核验见 [fact-check.md](fact-check.md)；取证清单见 `evidence-a.md`（数学撤稿与 Navier–Stokes 的 Lean 质疑）、`evidence-b.md`（Anthropic 使用政策）、`evidence-c.md`（速览）；抓取日志见 `capture-log-a.md`、`capture-log-b.md`、`capture-log-c.md`。PDF 与 HTML 原件不入库，存 OpenList（大小与 SHA-256 记入 `offsite.tsv`）。

## 主帖一：OpenAI 撤稿与 Navier–Stokes 的 Lean 质疑

- OpenAI 数学仓库更新日志：https://github.com/openai/math/blob/main/history.md （`a-history.md`；提交 `3014888` 北京 10-08 13:03:50，合并 `fd4aeeb` 13:20:00；`a-commit-3014888.summary.json`、`a-commit-3014888-name-status.txt`）
- 三篇撤稿说明：`preprints/<目录>/README.md` @fd4aeeb（`a-withdrawn-README-*.md`）
- 仓库 README（“unformalized results could have issues”）：`a-math-README-fd4aeeb.md`；形式化清单 `a-formalization-fd4aeeb.yaml`，与 1006 存档比对见 `a-yaml-diff.py`、`a-yaml-diff.out.txt`
- 质疑论文：https://arxiv.org/abs/2610.08144 （PDF `a-ns-lean-critique-v1.pdf` 不入库，文本 `a-ns-lean-critique-v1.txt`；作者 DAMTP 主页另有 31 页版 `a-damtp-ns-final.txt`，配套“错译清单” `a-damtp-ns-experiment.txt`）
- OpenAI NS 论文：https://cdn.openai.com/pdf/32d9f210-8b73-45e0-91bc-82a30aef8a9a/navier-stokes.pdf （PDF `a-oai-ns-paper.pdf` 不入库，文本 `a-oai-ns-paper.txt`；(8.19) 在 p.95，(10.19) 在 p.123）
- OpenAI NS 官方页：https://openai.com/index/navier-stokes-solution/ （`a-oai-ns-page.md`；页面日期 2026-09-08）
- OpenAI NS Lean 仓库：https://github.com/openai/NavierStokesAndEuler @f9e8bc5（README `a-nsrepo-README-f9e8bc5.md`；被引定理 `a-lean-norm_derivativeWord_inverse_le.lean`（`NavierStokes/SmoothFamilyTorusInverse.lean` 第 1059–1080 行）、`a-lean-inverse_finiteJets.lean`、`a-lean-xJet-def.lean`、`a-lean-exists_uniform_actual_pressure_flux_bound.lean`（`R3/PressureFlux.lean` 第 576 行）；两次提交差异 `a-nsrepo-8937a8f-to-f9e8bc5-name-status.txt`）
- 反应（未入正文；AHM 入速览）：AHM 声明 https://www.ahmath.org/statements （`a-ahm-statements.txt`）；AGMAI 9/29 声明（`a-agmai-sep29.txt`）；陶哲轩 Mastodon https://mathstodon.xyz/@tao/117395269325940185 （`a-tao-mastodon-*.json`）与博客转载页（`a-tao-blog-ahm.txt`）；Karagila https://karagila.org/2026/openai-pp/ （`a-karagila.txt`）；Aaronson https://scottaaronson.blog/?p=10169 （`a-aaronson.txt`）
- 热度：HN items（`a-hn-*.json`、`a-heat.json`，北京 10-09 09:52）
- 夸大实例：网易号转量子位“陶哲轩带头宣战”（`a-163-tao-boycott.txt`）

## 主帖二：Anthropic 禁止虐待模型

- 公告：https://www.anthropic.com/news/2026-usage-policy-update （`b-announce.txt`；published_time 2026-10-08T17:00:00Z）
- 使用政策正文：https://www.anthropic.com/legal/aup （`b-aup.txt`、PDF 文本 `b-aup-pdf.txt`；旧版 `b-aup-prev.txt`）
- 结束对话研究页（2025-08-15）：https://www.anthropic.com/research/end-subset-conversations （`b-end-subset.txt`）
- Claude Code 工具文档与变更日志摘录：`b-cc-tools.txt`、`b-cc-changelog-excerpt.txt`；系统卡摘录 `b-systemcard-excerpts.txt`
- 媒体（L4，未入正文，作背景与夸大实例）：The Verge https://www.theverge.com/ai-artificial-intelligence/1008100/anthropic-new-usage-policy-abuse-claude （`b-verge.txt`）；TechCrunch（`b-techcrunch.txt`）；The Decoder（`b-decoder.txt`）；36氪转新智元（`b-36kr.txt`）；其余见 `evidence-b.md` B5、B7
- 热度：HN（`b-hn-*.json`）

## 速览

- 购物 AI：https://arxiv.org/abs/2609.24927 （`c-arxiv-2609.24927v2.txt`，数字核对 `c-c1-numcheck.txt`）；Quartz 转述 `c-quartz.txt`
- OpenAI 收入：CNBC https://www.cnbc.com/2026/10/08/open-ai-revenue-nvidia-oracle-coreweave.html （`c-cnbc.txt`）；FT 经转述 `c-yahoo-ft-revenue.txt`（FT 原文 403 未读）
- AHM：见主帖一“反应”
- ts-rust：https://github.com/pingdotgg/ts-rust （`c-ts-rust-readme.md` 及历史版本 `c-ts-rust-readme-*.md`）
- USA Today：路透社 https://www.reuters.com/legal/legalindustry/usa-today-sues-openai-copyright-infringement-over-ai-training-2026-10-08/ （`c-reuters-usatoday.txt`）；起诉状文本 `c-usatoday-complaint.txt`（PDF 不入库）
- Whistle：https://cactuscompute.com/blog/whistle （`c-whistle.txt`、`c-whistle-hf-readme.md`）
- OSS Scanner：https://www.anthropic.com/news/anthropic-cyber-mission （`c-anthropic-cyber.txt`）；https://red.anthropic.com/oss-scanner （`c-oss-scanner-red.txt`）
- 11 正方形：https://github.com/Queuingtheorydotcom/11SquaresFormalized （`c-11sq-readme.md`、`c-11sq-verif.md`）；上游 `c-11sqopt-*.md`；第三方登记册 `c-jlevy-11.txt`（L5，背景）
- 热度：`c-heat.json`、`c-hn-*.json`

## 取证过程说明

- 线索：ChatGPT 扫描（北京 10-09 04:59）、WorkBuddy 扫描（10-09 06:02）、Develata 转来的 NS 质疑线索；Claude 用 HN Algolia 补扫（10-09 约 09:00）。
- 分组取证由 Sonnet 子代理完成（A 数学、B Anthropic、C 速览），派工单 `../../../../.handoff/2026-10-08-1008-evidence.md`；Claude 回读关键存档、核对截图与卡片。
- Reddit 全部拒绝访问（403 / 工具不支持），未取得 Reddit 热度；FT、Bloomberg 付费墙未绕过；Wayback 离线。
