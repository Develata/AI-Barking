# 1012 来源入口

事实核验见 [fact-check.md](fact-check.md)；取证清单见 `evidence-a.md`（Karpathy 漏斗）、`evidence-b.md`（GPT-6.1 Sol 500k）、`evidence-c.md`（ArtCraft）、`evidence-d.md`（速览）；抓取日志见 `capture-log-a.md`–`capture-log-d.md`；派工单见 `../../../../.handoff/2026-10-12-1012-evidence.md`。Claude 重截批注底图的脚本与记录：`m-shot.mjs`、`m-browser-records.jsonl`、`m-raw-crops.py`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D 组 Haiku 5.5（均 effort high）。耗时：D 约 21 分钟、A 约 49 分钟、C 约 64 分钟、B 约 81 分钟。Haiku 第二次试用回读：10 条全部交回，摘句可在存档检出，主动查出扫描 6 处错（Vals 方法论页、陶哲轩首发日期、Campfire 原句、凯奇“500 亿”为分期承诺、Step 5 仍为计划、Clef GPQA 只在模型卡）；未发现身份泄露或断章摘句。

## 主帖一：Karpathy 的“5000人”漏斗

- 原帖：https://x.com/karpathy/status/2109361546505966046 （`a-x-karpathy-raw.json`、`a-x-karpathy.txt`；首版与编辑版比对见 `a-x-karpathy-original-*.json`）；引用的 4 月旧帖 https://x.com/karpathy/status/2042334451611693415 （`a-x-karpathy-april.txt`）
- 加入 Anthropic：https://x.com/karpathy/status/2056753169888334312 （`a-x-karpathy-anthropic-join-raw.json`）
- 夸大实例：虎嗅 https://www.huxiu.com/article/4896682.html （`a-m-huxiu.txt`）；其他见 `evidence-a.md` A5
- 官方用户数（未入正文）：OpenAI、Google、Meta 各页存档见 `evidence-a.md` A4
- 热度：HN https://news.ycombinator.com/item?id=50047422 （360 分 / 525 评，北京 10/12 13:32）；原帖约 290 万浏览；AIHOT 热点榜 NO.01

## 主帖二：GPT-6.1 Sol 默认上下文 500k

- Tibo 主帖：https://x.com/thsottiaux/status/2109430343346581527 ；自回复：https://x.com/thsottiaux/status/2109430894604026039 ；10/10 回帖：https://x.com/thsottiaux/status/2108778939057664357 （`b-fx-*.json`）
- API 模型页：https://developers.openai.com/api/docs/models/gpt-6.1-sol （`b-model.txt`；Wayback 10/11 快照对照见 `b-wayback/`）
- Codex 定价页：https://learn.chatgpt.com/docs/pricing （`b-codex-pricing.txt`）
- 其他（未入正文）：Codex changelog、help.openai.com 费率卡摘录、OpenAI 开发者社区“28 days of shipping”汇总帖、IT之家等报道（`evidence-b.md`）
- 热度：主帖约 96 万浏览 / 9,795 赞（北京 13:52）；r/codex “Day 7” 帖 571 分 / 186 评；HN 无提交

## 主帖三：ArtCraft 复刻 Adobe 套件

- PhotoCraft 仓库与 README：https://github.com/storytold/photocraft （`c-readme-photocraft.md`；提交统计 `c-gh-commit-stats.txt`；releases `c-gh-photocraft-releases.json`）
- 官网：https://getartcraft.com/apps （`c-home-apps.txt`）
- “decompiled” 源帖：https://bsky.app/profile/jamesomalley.co.uk/post/3mxbl36nkms2q （`c-bsky-thread.json`）
- HN：https://news.ycombinator.com/item?id=49958850 、https://news.ycombinator.com/item?id=49993722 、https://news.ycombinator.com/item?id=49981449 （`c-hn-*.json`）
- TechRadar 实测（未入正文）：https://www.techradar.com/pro/one-dev-is-recreating-7-adobe-apps-in-rust-for-free-so-i-tested-it-out （`c-techradar.txt`）
- 其他媒体与夸大实例：`evidence-c.md` C10

## 速览

- 陶哲轩《Math 2.0》：https://teorth.github.io/tao-web/slides/math-2-0-caltech-2026.pdf （`d-d4-tao-slides.txt`）；Mastodon 首发 https://mathstodon.xyz/@tao/117395269325940185
- Vals：https://www.vals.ai/blogs/multi-agent-vibe-code-bench （`d-d3-vals-multi-agent.txt`）
- Sakana MASS：https://pub.sakana.ai/mass/ （`d-d2-mass-page.txt`）；arXiv https://arxiv.org/abs/2610.12176
- 凯奇：Variety https://variety.com/2026/tv/news/nicolas-cage-ai-waiver-amazon-spider-noir-1236907564/ （`d-d7-variety-us-orig.txt`）
- 未收：Qwen3-1.7B 置信度实验（D5，概率口径与署名放不进 40 字，反向核验后删）、Illustration Beyond Proof（D1，HN 51/0）、Campfire 复查（D6，限定词放不进 40 字）、BetterWispr（D8）、Step 5 权重（D9，仍为计划）、Clef-omni（D10，热度低）
- 热度：`d-heat.json`
