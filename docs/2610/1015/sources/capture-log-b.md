# 1015 期 B 组抓取日志（Meta Muse Spark Open）

执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。北京时间（UTC+8）；本机美东 EDT = 北京 − 12 小时。开工 19:39，收工约 20:50（约 70 分钟）。工具：`barking fetch / shot / heat`、curl（浏览器 UA，匿名）、opencli（本机浏览器窗口，仅匿名公开页，未登录、未输入凭据）、HN Algolia、python3 -I 解析页面内嵌 JSON。未用 firecrawl，未用镜像站。时刻为近似值。

| 约时刻 | URL / 对象 | 工具 | 结果 | 存档 |
|---|---|---|---|---|
| 19:39 | Meta 博客、评测方法 PDF、AA 模型页、AA 模型榜、PyTorch 博客、Apple 研究页 | barking fetch | 全部 200 ok | `b-meta-blog.*`、`b-meta-eval-pdf.*`、`b-aa-model.*`、`b-aa-index.*`、`b-pytorch.*`、`b-apple.*` |
| 19:39 | https://huggingface.co/api/models/facebook/muse-spark-open（猜测路径） | barking fetch | 401 blocked（路径错）；失败文件已删 | 无 |
| 19:40 | https://huggingface.co/api/models/meta-models/Muse-Spark-Open-816B-A42B | curl | 200，含 cardData.extra_gated_prompt（许可全文） | `b-hf-api.json` |
| 19:40 | HF 模型页、HF 集合页 | barking fetch | 200 ok（模型卡与许可全文匿名可见） | `b-hf-page.*`、`b-hf-collection.*` |
| 19:40 | https://huggingface.co/blog/mlx-muse-spark（猜测） | barking fetch | 404 failed；文件已删 | 无 |
| 19:41 | HF raw LICENSE.md、USE_POLICY.md；research.meta.ai 上猜测的许可路径 | curl | 401 / 404，未重试 | 无 |
| 19:45 | Meta 基准表原图 light/dark v6（URL 取自博客 HTML） | curl | 200，4320×5200 | `b-benchmark-scorecard-original.png`、`b-benchmark-scorecard-dark-original.png` |
| 19:48 | Meta 博客 headless 渲染（700 CSS px，DPR 2，高 3600） | barking shot render | 成功；基准表与头图懒加载为空白占位，标题与正文可用 | 中间图在 `%TEMP%\barking-shot\26101015-b\`（不入库） |
| 19:55 | 基准表原图（本地 HTML 内 `<img style="width:700px">`，渲染为 1400 px 宽） | barking shot render | 成功；仅缩放，不重绘 | 中间图 `scorecard.png` |
| 20:03 | AA 模型页 headless 渲染 | barking shot render | 成功；图表客户端渲染为空白 | 中间图 `aa-model.png` |
| 20:05 | AA 模型页（opencli） | opencli browser | 打开成功，滚动未生效，页面带浏览器翻译插件图标，未采用；会话已关闭 | 无 |
| 20:10 | Apple 页 Table 2、Table 3，读 DOM 并点击 65,536 标签 | opencli browser | 成功，两档数字见 evidence-b.md | 无文件 |
| 20:15 | Apple 页 headless 渲染 | barking shot render | 高 9000 成功但未到 Table 3（已删中间图）；高 16000 失败（Chrome 未写图） | 无 |
| 20:20 | Apple Table 3（opencli，zoom=2） | opencli browser | 版式溢出，未采用；会话已关闭 | 无 |
| 20:40 | HF 模型页 headless 渲染 | barking shot render | Chrome 正常退出但未写图，未重试 | 无 |
| 20:45 | HN 搜索与帖子 50107379 | barking heat（失败：io unexpected end of file，删去 `b-heat.json`）；改 HN Algolia curl | 成功：854 分 / 427 评论；items 接口不含评论得分 | `b-hn-search.json`、`b-hn-50107379.json` |

## 图片（`images/`，2 倍像素、宽 1400 px，只裁切、拼接、缩放，不改数字）

| 文件 | 内容 |
|---|---|
| `13-meta-title-intro.png` | Meta 博客标题、日期与首两段正文（含 “most capable US open-weight model”、816B/42B、1M-token）；中间懒加载的空白头图已裁去 |
| `14-meta-scorecard.png` | Meta 官方基准表原图（light-v6）整图，11 行 × 8 列；图内无 AA 指数行 |
| `15-aa-muse-summary.png` | AA 模型页顶部：#10/119、39、速度、成本、Comparison Summary、技术规格 |

13、14 已逐张打开目视确认；15 为整页渲染后的裁切，预览整页时已看到该区域文字清楚、边界在文字行之间。

## 删除的文件（均为自己生成的失败/试验文件）

`b-hf-api.html/.headers`（401）、`b-hf-apple.*`（404）、`b-aa-index-ranked.json`（被 `b-aa-ranked-all.json` 取代）、`b-heat.json`（失败）、`%TEMP%` 下失败的渲染中间图。

## 假设

- 北京时间换算：HN 提交 2026-10-15T17:47:39Z = 10-16 01:47 北京。
- AA 排名使用页面内嵌 JSON；“前面的模型”按 AA 的 Open weights / Large 同类给出，全体排名另注。
- `15` 为 headless 渲染，速度数字（102.7）与存档文本（102.0）因实时波动略异。
