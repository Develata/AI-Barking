# 1012 来源入口

> 制作时目录为 `docs/2610/1013/`（派工单 `.handoff/2026-10-13-1013-evidence.md`）；发布后 Develata 决定定为 1012 期（上期改为 1011）。取证文件、日志与脚本里的 1013 路径保持原样。

正文：`../doc_1012_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（Codex）、`evidence-b.md`（Anthropic）、`evidence-c.md`（GLM-Legal）、`evidence-d.md`（速览）；抓取日志：`capture-log-*.md`、`m-browser-records.jsonl`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high）。耗时：D 约 20 分钟、C 约 38 分钟、A 约 58 分钟、B 约 115 分钟。Haiku 第三次试用：10 条全部交回，摘句经脚本在存档中检出；Intel D270 官网 403 未收。B 组清理旧图时误删 C 组 25–29 号截图，C 组已重截并核对。

选题：两份 ChatGPT 扫描（北京 10/13 11:09、17:10），Develata 选 Codex 守护进程 + Anthropic 电视盒复制研究 + GLM-Legal。风控排除：Anthropic 反蒸馏（点名中国公司）、英国 AISI（外国政府）。

## 一手来源

- Codex issue #48195：https://github.com/openai/codex/issues/48195
- Codex 0.157.0 更新说明：https://github.com/openai/codex/releases/tag/rust-v0.157.0
- 默认开启提交 #47179：https://github.com/openai/codex/commit/75ec81c8623c0ca50b129de5a82521a23d37b726
- 守护进程 README（commit 90f00bd）：https://github.com/openai/codex/blob/90f00bd89ed3263b4d5993849cccb795f4330449/codex-rs/app-server-daemon/README.md
- 功能开关定义：https://github.com/openai/codex/blob/90f00bd89ed3263b4d5993849cccb795f4330449/codex-rs/features/src/lib.rs#L1027-L1033
- 官方配置参考页：https://developers.openai.com/codex/config-reference
- Anthropic 研究：https://www.anthropic.com/research/self-replicating-ai
- 量子位报道：https://www.qbitai.com/2026/10/502920.html
- 智谱 GLM-Legal：https://z.ai/blog/glm-legal
- Vals 模型页：https://www.vals.ai/models/zai_glm-legal ；Harvey LAB 榜：https://www.vals.ai/benchmarks/hlab ；Legal Research Bench：https://www.vals.ai/benchmarks/legal_research
- 速览：AA Muse Wonder https://artificialanalysis.ai/articles/muse-wonder ；xAI https://x.ai/news/x-subscription ；QGIS QEP-408 https://github.com/qgis/QGIS-Enhancement-Proposals/blob/master/qep-408-ai-tool-policy.md ；arXiv 2610.13928 https://arxiv.org/abs/2610.13928 ；AA-3D https://artificialanalysis.ai/articles/aa-3d ；Epic CEO 帖 status/2109705826688549304（海外社媒）

本号机器观察见 `m-local-codex-observation.md`（只读，路径已脱敏）。
