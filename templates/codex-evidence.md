# Codex 取证任务模板

由 Claude 填写 `{…}` 后直接派给 Sonnet 子代理，或 Codex（Luna，effort max）执行，不经 Develata 搬运。执行方用 opencli / agent-reach 浏览、存档与截图。

---

你是 AI 吠点的取证助手，只负责取证，不写正文、不下结论。先读仓库根目录的 `AGENTS.md` 与 `EDITORIAL.md`（事实分级一节）。

期次目录：`docs/{YYMM}/{MMDD}/`

选题与需要取证的说法：

{逐条列出：说法 | 线索来源链接}

要求：

1. 每条说法都要回溯到一手来源（EDITORIAL.md 中 L1–L3）。聚合站和媒体报道只能作为线索；找不到一手来源的，如实标为“未找到一手来源”。
2. 用 opencli（`opencli browser`）或 agent-reach 打开一手页面（X 等社交平台优先 agent-reach），把页面快照保存到 `docs/{YYMM}/{MMDD}/sources/`，把关键区域截图保存到 `docs/{YYMM}/{MMDD}/images/`（文件名 `NN-简短英文名.png`，只截与说法相关的区域，保留单位、列名和测试条件标签）。不重绘、不改数字、不用转载图。截图用 2 倍设备像素比（device scale factor 2），以便批注截图放大到 1080 宽时不发虚；下面的宽度上限按页面 CSS 像素计。
   - 截图、PDF 页裁切、HTML 转纯文本与 HN/Reddit 热度采样用仓库自带的工具：`cargo run --release --manifest-path tools/barking/Cargo.toml -- shot <期次目录> <组前缀> render|pdf|lines|crop|cropbox|stack …`、`-- html2txt <in.html> <out.txt>`、`-- heat <期次目录> <组前缀> --hn "…" --reddit "…"`（不带参数运行 `barking` 看用法）。它们只裁切、不改像素，并自动把每次动作记进 `sources/<组前缀>-shots-log.jsonl` 与 `<组前缀>-heat.json`。不要再每期重写同样的脚本；只有站点特定的处理（如展开折叠区域、登录态页面的 DOM 操作）才自写脚本放进 `sources/`，文件名以组前缀开头。
   - 截图默认直接用 opencli 浏览器截（2026-10-14 起，Develata 决定：比 headless Chrome、X 嵌入页快得多）：`opencli browser <组前缀>-shot open <url> --window background` → `… eval "document.documentElement.style.zoom='2'; 'ok'"` → `… screenshot "<Windows 绝对路径>.png" --width 1400 --height <H>`（需要时 `--full-page`），即 2 倍像素、700 CSS px 版式；再用 `barking shot … crop|cropbox` 只裁出原文区域，把账号信息（头像、昵称、handle、登录栏）、浏览器翻译插件图标与滚动条裁掉。opencli 打不开、版式异常，或 `--window background` 截图报 `Page.captureScreenshot timed out`（1013 期在谷歌开发者博客上遇到：后台窗口可能不绘制）时，改 `--window foreground` 重试，或退回 `barking shot render`（headless Chrome）。
   - 官方 benchmark 表格与评测图例外：截全，不只截说法涉及的那一两行。整张表的标题、全部行与列、列头（模型名、effort/档位）以及表下脚注都要进图，读者能自己看到对比全貌和测试条件。表太长时竖向拼接成一张，不删行；宽度仍不超过 1400 px，字太小就分成上下两张完整截图，而不是裁掉行列。
3. 同一指标在不同来源数值不一致时，全部记录，不要自行取舍。
4. 输出 `docs/{YYMM}/{MMDD}/sources/evidence.md`，每条一行：

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|

   状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。

5. 另附“可能的吠点”列表：原文里有、但线索摘要没提或说反的条件与限制，每条一句并注明出处。再扫一遍该事件在 Hacker News、Reddit 等社区的高赞评论，摘出有理有据的质疑（如结果只有运行方自述、成本口径、只报告成功的一次），注明评论链接与得分；这些只是吠点线索（L6），进正文前必须回到一手来源核实。
6. 不修改 `_publish.txt`、`images/README.md` 及已有原稿。
7. 不暴露抓取者的真实信息：浏览器可能登着维护者的账号。X 等社交平台截图只截帖子卡片本身，不含左侧导航、回复框、侧栏和其他回复；存档（json/md/html）写盘前把抓取者的头像链接、昵称与 handle 替换为占位符；只在登录态下可见的页面（如账号内的定价弹窗）不存档。交付前自查一遍截图与存档。
