# AI 吠点 · AI-Barking

聊 AI 沸点，轻松识破吠点。

AI 吠点是由 **[Develata](https://github.com/Develata)** 主理的独立内容品牌，记录 AI 新闻里的变化、条件与疑问。本仓库公开保存正文、配图说明、来源、制作工具、完整工作材料及 Git 历史，也作为项目的历史备份；图片与原件存于网盘，网页展示留待后续建设。

作者与维护者：Develata · [个人网站](https://develata.me/)

## 内容目录

期次编号沿用制作目录，不代表各平台的实际发布时间。各期目录按月份归档为 `docs/<YYMM>/<MMDD>/`；2026-10-01 改为按月归档之前写成的工作文件（`.handoff/`、各期 `sources/` 中的日志与脚本）仍引用旧路径 `docs/<MMDD>/`。文章和图片中的价格、榜单与事件状态以各期记录的时点为准。

| 期次 | 正文 | 配图与上传顺序 | 来源与限制 |
|---|---|---|---|
| 2026 / 0924 | [Opus 5.5 完爆 Astra？](docs/2609/0924/doc_0924_publish.txt) | [配图说明](docs/2609/0924/images/README.md) | [来源入口](docs/2609/0924/sources/README.md) |
| 2026 / 0925 | [拒答也收费？OpenAI私闯政府网？](docs/2609/0925/doc_0925_publish.txt) | [配图说明](docs/2609/0925/images/README.md) | [来源入口](docs/2609/0925/sources/README.md) |
| 2026 / 0926 | [OpenAI停训？Claude物理突破？](docs/2609/0926/doc_0926_publish.txt) | [配图说明](docs/2609/0926/images/README.md) | [来源入口](docs/2609/0926/sources/README.md) |
| 2026 / 0927 | [沙箱逃逸？AI蠕虫来了？新基因编辑？](docs/2609/0927/doc_0927_publish.txt) | [配图说明](docs/2609/0927/images/README.md) | [来源入口](docs/2609/0927/sources/README.md) |
| 2026 / 0928 | [Sonnet跑赢Opus？智能爆炸？](docs/2609/0928/doc_0928_publish.txt) | [配图说明](docs/2609/0928/images/README.md) | [来源入口](docs/2609/0928/sources/README.md) |
| 2026 / 0929 | [模拟越权29%，Astra照样上岗？](docs/2609/0929/doc_0929_publish.txt) | [配图说明](docs/2609/0929/images/README.md) | [来源入口](docs/2609/0929/sources/README.md) |
| 2026 / 0930 | [国产模型逼近Mythos？五分之一价？](docs/2609/0930/doc_0930_publish.txt) | [配图说明](docs/2609/0930/images/README.md) | [来源入口](docs/2609/0930/sources/README.md) |
| 2026 / 1001 | [Argon解决幻觉？Kimi被点名？](docs/2610/1001/doc_1001_publish.txt) | [配图说明](docs/2610/1001/images/README.md) | [来源入口](docs/2610/1001/sources/README.md) |
| 2026 / 1002 | [谷歌TPU上太空？OpenAI通报超百家](docs/2610/1002/doc_1002_publish.txt) | [配图说明](docs/2610/1002/images/README.md) | [来源入口](docs/2610/1002/sources/README.md) |

正文采用可直接复制的纯文本，图片单独保存。阅读或发布时不需要依赖 Markdown 插图。

## 公开归档范围

- 发布内容：各期 `_publish.txt`、配图说明、来源入口与限制、后续更正记录（图片本身自 1002 期起不入库，见下），以及品牌素材、编辑规范、通用模板和制作工具。
- 工作档案：原始草稿、逐轮编辑与核验笔记、代理派工和交接、取证执行日志、文本存档、社交媒体原始采样及已有 Git 历史一并保存。历史文件中的“内部”指编辑流程用途，不代表访问受限。
- 各期全部图片（封面、省流卡、批注截图、备用图、取证截图）以及 `sources/` 中的 PDF、整页 HTML 与音视频原件不放进本仓库，只入库文字；它们存于维护者的网盘（OpenList），各期 `sources/offsite.tsv` 记录文件名、大小与 SHA-256。0924–1001 期原先已入库的这类文件，于 2026-10-02 补传网盘并改写 Git 历史移出，改写前的完整历史由维护者另存备份；此后各提交哈希与此前不同。
- 工作档案可能包含未采纳的假设、失败采集、过期路径与尚未完成的核验；文件存在不等于采集成功或事实已确认。阅读成稿以 `_publish.txt` 为入口，判断证据以对应核验状态和限制为准。
- 公开存档不代表独立复现了厂商测试，也不代表所有原始网页都采集完整。未完成的核验、样本偏差与测试条件继续明确记录。
- Develata 署名及维护者自愿公开的联系方式可以保留；账户凭据、登录状态与他人的非公开信息不在公开范围内。
- 浏览器会话状态、临时缓存和构建产物不上传。GitHub 保存的是已提交并推送的文件与历史，不代替这些本地运行数据的备份。

已有 Git 历史也属于公开范围。只删除当前文件或添加忽略规则，不能让旧提交中的材料退出公开历史。

## 制作与更正

选题、事实分级与核验流程见 [EDITORIAL.md](EDITORIAL.md)，图文交付约定见 [AGENTS.md](AGENTS.md)。制作过程使用 AI 辅助，最终编辑与发布决定由 Develata 负责。AI 生成封面及其提示词记录在各期配图说明中。

如发现错误，可在 [Issues](https://github.com/Develata/AI-Barking/issues) 中附上原文位置与来源。更正记录放在对应期次的 `CORRECTION.md`，不以删除原帖代替说明。

机械格式检查工具位于 `tools/barking/`：

```sh
cargo run --release --manifest-path tools/barking/Cargo.toml -- lint docs/2609/0925
```

格式检查通过不等于事实核验完成；历史稿件的已知格式例外见 `docs/.lint-ignore`。

## 素材与转载

第三方网页截图、引文及品牌标识按各期说明标注来源，本仓库不代替原权利方授予使用许可。原创内容与工具代码尚未指定统一的开放许可；如需转载或复用，请先联系维护者。
