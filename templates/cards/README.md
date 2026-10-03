# 省流卡、速览聚合图与批注截图

规则见 `EDITORIAL.md`“省流卡与批注截图”与“速览”，命名见 `AGENTS.md`。2026-10-04 起每期加速览聚合图（`roundup.html`，与省流卡同一套纸白配色，主帖条目一行、标“详见前页”）；没有主帖的速览专帖另由 `cover.html` 渲染两张模板封面。版式 2026-10-02 经 Develata 认可；2026-10-03 起图上不放吠点（与正文重复），省流卡只列事实，批注图只有原文批注与译注。批注图的“原文截图”标签与译注框用低饱和墨蓝（`annot.html` 顶部的 `--accent: #3a5f8f`，底 `#e8eef5`）；荧光笔四色只用于高亮与编号，编号圆点已调柔，与蓝框明度相近。省流卡不用墨蓝（整张蓝框太闷）：纸白卡片，左侧彩条与标签取荧光笔四色（金、赭橙、鼠尾草绿、紫），按条目顺序循环，与批注图的编号同一套颜色。2026-10-03 配色：原先的亮宝蓝 #1f4fd1 与橙、黄、绿高亮近乎互补，同框时刺眼，故降饱和度。

| 文件 | 作用 |
|---|---|
| `tldr.html`、`roundup.html`、`annot.html` | 省流卡、速览聚合图、批注图的版式模板，`barking card` 填充；改版式只改这里 |
| `cover.html` | 速览专帖（`kind = "roundup"`）的模板封面，3:4 与 2.35:1 共用，看板娘取 `common_images/profile_picture.png` |
| `example.toml` | `images/cards.toml` 样例（0926 期），字段说明在注释里 |
| `visual-review.md` | 视觉复核提示词 |

工具：Chrome 无头模式渲染（荧光笔依赖 CSS 正片叠底；Typst 0.15 尚无混合模式，见 typst/typst#8815），Tesseract 定位关键句，ffmpeg 放大底图。安装：`scoop install tesseract tesseract-languages`（ffmpeg、Chrome 已有）。找不到 Chrome 时用环境变量 `BARKING_CHROME` 指定。Scoop 安装语言包时设置了用户环境变量 `TESSDATA_PREFIX`，安装前已打开的终端要重开才生效。

## 步骤

1. 每张要批注的截图另存一份底图 `images/NN-name-raw.png`（原始裁切，像素不改）。底图宽度尽量窄（页面 CSS 宽约 700 px 以内），否则缩进 1080 宽的卡片后英文太小；高度则放宽：2026-10-03 起图上不再占地方放吠点，底图在两三条译注时可截到约 600 CSS px 高（版面自动按高度缩放，被压得过小会报错）。除关键段落外，把标题、前后文、图表的标题与图例、卡片边框、页面配色等背景留全，让读者看出这是哪一页、哪一块，不要只掐一两行。取证按 2 倍像素比截图，读者放大后仍清晰。
2. 复制 `example.toml` 为 `images/cards.toml`，填写省流卡、速览与各批注图。速览的主帖条目从正文逐字摘短标题；其余条目的句子先写进 `sources/fact-check.md` 的“速览”一节，再原样抄到 `text`。省流卡的 `fact` 从正文逐字摘取（可加句末标点；不相邻的几段用“；”拼接，每段各自核对；不要断章丢掉同句里改变含义的限定）；`quote` 写英文原句，`gloss` 照 `fact-check.md` 的译法。
3. 渲染：

   ```bash
   cargo run --release --manifest-path tools/barking/Cargo.toml -- card docs/<YYMM>/<MMDD>
   ```

   只重渲某几张：在期次目录后跟文件名，如 `00-tldr.png`。工具用 OCR 找到每条 `quote` 并逐行加高亮；以下情况直接报错，不会猜位置：找不到原句、出现不止一次、起止落在一个词中间（如 state-of-the-art 只引半截）、识别置信度低于 60、`underline` 不在本条原句内。OCR 认不准的图（拼接图、低清图）删去 `quote`，改写 `rects`（底图像素 `[x, y, w, h]`，每行一个；此时不能用 `underline`）。版面放不下（省流卡或速览条目超出、速览的主帖条目折成两行、模板封面的标签放不下、批注图底图被压到铺满宽度的 75% 以下、标题或页脚过长）也报错，按提示删减。中间 HTML 在系统临时目录 `barking-card/<进程号>/`，报错信息里会给出路径。
4. 自查：逐张打开成图；高亮处再裁出来放大看（例：`ffmpeg -i out.png -vf "crop=1080:420:0:600" zoom.png`）。确认高亮没有扫进邻句、下划线落在目标限定词下方、引号与换行正常、页脚不压内容。
5. 视觉复核（必做，跨模型）：按 `visual-review.md` 填好提示词，附上全部卡片与对应的 `-raw` 底图，交给 WSL 中的 Codex（`gpt-6-astra`，effort `medium`，只读）：

   ```bash
   MSYS_NO_PATHCONV=1 wsl -d Debian -e bash -lc 'cd "$1" && exec timeout 1200 "$0" exec -m gpt-6-astra -c "model_reasoning_effort=\"medium\"" -s read-only --skip-git-repo-check --ephemeral -C "$1" -o "$2" --image=images/00-tldr.png --image=images/01-x.png --image=images/01-x-raw.png - < "$3"' /home/deve/.bun/bin/codex "<期次目录的 WSL 路径>" "<报告的 WSL 路径>" "<提示词的 WSL 路径>"
   ```

   每张图写成一个 `--image=` 参数；路径用 `wsl -d Debian -e wslpath -a` 转换。后台运行，等完成通知后读报告。Codex 不可用或超时时，改派 Sonnet 子代理按同一提示词看图复核，并在记录中注明复核方。Claude 逐条核实意见再改，结论记入 `sources/fact-check.md`。
6. `barking lint`：省流卡与速览主帖条目的文字须在正文同一行中逐字出现，速览其余条目须逐字列在事实清单“速览”一节且级别与措辞相符（均为 error）；cards.toml 改过而图未重渲、省流卡不在正式配图第一行、`fact-check.md` 没有“视觉复核”记录，均给 warn。
7. `images/README.md` 的正式配图里，省流卡列第 1 行、速览聚合图第 2 行（速览专帖只有速览图，列第 1 行），批注图替代原截图；`-raw` 底图列入备用图（用途写“批注底图”）。1002 期起，包括卡片在内的全部图片都不入 Git，发布后由 `barking offsite --upload` 存 OpenList（路径与 SHA-256 见 `sources/offsite.tsv`）；日后重渲，先从 OpenList 同一路径取回底图。
