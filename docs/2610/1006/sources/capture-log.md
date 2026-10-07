# 1006 抓取日志（索引）

- A 组：`capture-log-a.md`；B 组：`capture-log-b.md`；D 组：`capture-log-d.md`（均为 Codex 执行记录，原样保留）。
- Claude 补抓（北京时间 2026-10-07）：
  - curl：WINK、WBBH 原文 HTML（不入库，脱敏文本见 `a-wink.txt`、`a-gulfcoast.txt`）；Anthropic 隐私政策与隐私中心页；AGMAI 两页；Bloom 的 Erdős 公告（陶哲轩博客转载）；《科学美国人》；claude.com Workspace 文章；新浪转载的 36氪文章。openai.com 博文 curl 返回 403，经网页读取工具核对正文，未本地存档。
  - Substack 公开 API：SemiAnalysis 文章 JSON 与免费部分 body_html、21 张原图。
  - raw.githubusercontent.com：openai/math 的 README、CONTENTS、lean/README、formalization.yaml、lean/docs/003.md、Comparator 挑战文件与配置、拟黎曼猜想论文 PDF；mathlib-initiative/formalization.yaml 的 schema。
  - GitHub API：openai/math 提交 adc7f12 时间、仓库创建时间。
  - HN Algolia：各帖分数与评论检索。
  - 截图：`m-shot.py`（无头 Chrome 700 CSS px、DPR 2，Tesseract 锚点裁切），记录在 `m-shots.jsonl`；WINK 与 Anthropic 隐私政策首次截图超时（180 秒），改用 `--timeout=30000` 重截成功。论文首页用 `pdftoppm` 渲染后裁切。
