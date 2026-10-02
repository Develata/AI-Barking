# 省流卡与批注截图模板

规则见 `EDITORIAL.md`“省流卡与批注截图”，命名见 `AGENTS.md`。两张模板都以 0926 期为样例，版式已获 Develata 认可（2026-10-02）。

| 文件 | 产出 |
|---|---|
| `tldr.html` | `images/00-tldr.png`，省流卡 |
| `annot.html` | `images/NN-name.png`，批注截图（底图 `NN-name-raw.png`） |

工具：Chrome 无头模式渲染（荧光笔依赖 CSS 正片叠底 `mix-blend-mode: multiply`；Typst 0.15 尚无混合模式，见 typst/typst#8815），Tesseract 取词框（`scoop install tesseract tesseract-languages`），ffmpeg 放大与裁切检查。字体用系统已装的 Noto Sans SC。

## 步骤

1. 把模板复制到 Claude 的 scratchpad，按模板顶部注释改内容。中间 HTML 不入库。
2. 批注截图先取关键句的词框。放大 3 倍识别更稳，坐标再除以 3：

   ```bash
   ffmpeg -v error -y -i images/NN-name-raw.png -vf scale=iw*3:ih*3:flags=lanczos /tmp/ocr3.png
   tesseract /tmp/ocr3.png stdout --psm 6 -l eng -c tessedit_create_tsv=1 | awk -F'\t' 'NR>1 && $12!="" {printf "%s\t%d\t%d\t%d\t%d\t%s\n", $12, $7/3, $8/3, ($7+$9)/3, ($8+$10)/3, $11}'
   ```

   输出列：词、left、top、right、bottom、置信度。中文媒体截图改用 `-l chi_sim+eng`。一句跨几行，就按行各取首词 left、末词 right，各放一个 `.hl`；高度取该行词框上下缘，上下各留约 1 px。置信度低于 80 或找不到原句的词时，不要猜坐标：放大看图手量，或换更清晰的底图。0926 图 1 的实测：OCR 词框与手量坐标相差不超过 2 px。
3. 渲染（1080×1440，`file:///` 用绝对路径）：

   ```bash
   "/c/Program Files/Google/Chrome/Application/chrome.exe" --headless=new --disable-gpu --hide-scrollbars --force-device-scale-factor=1 --allow-file-access-from-files --window-size=1080,1440 --screenshot=E:/.../images/00-tldr.png file:///C:/.../tldr.html
   ```

4. 核对：打开成图逐处检查；高亮区域再用 ffmpeg 裁出来放大看（例：`ffmpeg -i out.png -vf "crop=1080:420:0:600" zoom.png`）。确认高亮没有扫进邻句、下划线落在目标限定词上、引号与换行正常、页脚不压内容。
5. 在 `images/README.md` 正式配图中，省流卡列第 1 行，批注图替代原截图；`-raw` 底图列入备用图（用途“批注底图”）。
