# 0924 取证附件

总报告：`../usage-evidence.md`。本目录是内部证据，不是发布正文。保留失败采集档案，不能以文件存在推定采集成功。

核心文件：

- `x_raw.jsonl`：按 tweet id 去重的全部搜索返回；保留多查询命中信息。
- `x_coded.jsonl` / `x_excluded.jsonl`：第一轮纳入编码 / 互斥排除理由；raw_index 为原始文件从 0 开始的行号。
- `x_blind20.jsonl`：固定 seed 20260924；只含 id、url、text。Claude 复编码前不要查看编码表、报告分布或其他编码文件。
- `protocol.json` / `search_ledger.jsonl`：固定窗口、检索词、排序、限额与每次请求记录。最后一次请求的 429 见 `x_search_08_top.stderr.txt`，空 stdout 不是成功的零结果。
- `coding_decisions.tsv`：逐条语义决策的紧凑输入，编码缩写见首行。`screening.json` 仅为初筛中间结果，不是最终排除表。
- `build_x_stats.py` / `x_stats.json`：编码展开、原文短摘定位、盲抽、完整精度统计。离线重跑不会访问 X。
- `arena-extract.json`、`openrouter-extract.json`、`epoch-extract.json`：精确型号及数值摘录。
- `build_report.py`：从上述决策与来源快照离线构造报告。
- `baseline.json` / `verify.py` / `validation.txt` / `git-bash-validation.txt`：原有文件 SHA-256 基线、核验程序与真实核验输出。
- `new-files.txt`：本次所有新增文件，路径相对仓库根。其余原始 CSV/浏览器 JSON/网页文本及请求日志也在此目录。

Epoch 官方数据下载是一个包含多 benchmark 的公共导出；本次只在内存解包，保留导出的 CSV，没有创建 ZIP 副本。非本任务 benchmark 的 CSV 不参与结论。`fetch_supplement.py` 是未成功启动的补充采集脚本，不能把它列的 URL 当作已保存网页；相应需要的事实已通过原始网页/论文与浏览工具核实，见总报告链接。部分首次 JS/页面读取为空或失败，后续有效文件名在报告内明确标注。

## 新图来源与上传限制

四张图均为 2026-09-24 UTC 对原站浏览器视图的直接截图，未拼图或改写数字；是否用于正式稿件由编辑决定，本次不改原有配图顺序说明。

| 建议本组顺序 | 文件（位于 ../../images/） | 来源与区域 | 限制 |
|---|---|---|---|
| 1 | 30-arena-webdev.png | https://arena.ai/leaderboard/code ，表头/日期/Opus/Astra/Fable/Sol 行 | 更新日 9 月 23 日；max effort；Opus 与 Astra 的 rank spread 重叠 |
| 2（补充） | 30-arena-text.png | https://arena.ai/leaderboard/text ，Style Control 总榜 | 页面仍标 9 月 13 日；不能用于评价 9 月 22 日才发布的三款模型 |
| 3（补充） | 31-openrouter.png | https://openrouter.ai/anthropic/claude-opus-5.5#activity ，日 token 图 | 24 日未完成；右侧是日图数值，不是报告截至 23 日的 month 窗合计 |
| 4 | 32-frontiermath.png | https://epoch.ai/benchmarks/frontiermath-tier-4-v2 ，Tier 4 表格 | ± 是 SE；Opus 5.5 未列出；不能拿 Opus 5 顶替 |

本次没有生成发布稿、with_images 副本、压缩包，也没有修改正文、原配图 README 或 fact-check。
