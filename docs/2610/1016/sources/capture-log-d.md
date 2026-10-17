# 1016 期 D 组（速览：产品与硬件）抓取日志

执行方：claude-haiku-5-5（D 组，派工单 `.handoff/2026-10-16-1016-evidence.md`）。
时区：北京时间（UTC+8）；页面原文 UTC 或美东时刻已换算。抓取批次 2026-10-17 20:49–20:53。
工具：仅用仓库自带 `barking`（fetch、heat）。未使用 WebFetch、firecrawl、截图与浏览器。无 curl 单次请求。

## 抓取记录（barking fetch）

**第一批**：2026-10-17 20:49:25 起，`barking fetch docs/2610/1016 d <清单>`，并发 6，同一清单域名至多 2。日志 `d-fetch-log.jsonl`。

| 北京时间 | 名称 | URL | 结果 | 存档 |
|---|---|---|---|---|
| 20:49 | rtx-verge | https://www.theverge.com/tech/1012748/nvidia-rtx-spark-windows-arm-laptops-asus-hp-lenovo-impressions | 200，660365 B，ok，927 行 | `d-rtx-verge.html`、`.txt`、`.headers` |
| 20:49 | rtx-nvidia | https://nvidianews.nvidia.com/news/nvidia-rtx-spark-pcs-available-today-a-new-beginning-for-windows-pcs | 200，83306 B，ok，393 行 | `d-rtx-nvidia.*` |
| 20:49 | surface-verge | https://www.theverge.com/tech/1012628/microsoft-surface-laptop-ultra-review | 200，547495 B，ok，804 行 | `d-surface-verge.*` |
| 20:49 | cr-claudedev | https://claude.dev/blog/solving-code-review-at-anthropic/ | 200，229613 B，ok，317 行 | `d-cr-claudedev.*` |
| 20:49 | ultramarine | https://www.paperinstruments.com/ultramarine | 200，168876 B，ok，162 行 | `d-ultramarine.*` |
| 20:49 | waymo-toronto | https://waymo.com/blog/2026/10/waymo-in-toronto/ | 200，70168 B，ok，169 行 | `d-waymo-toronto.*` |

第一批结果：ok 6，blocked 0，failed 0；用时 2 秒。

**第二批**：2026-10-17 20:51:31，补存 Ultramarine 技术报告页（D3 发布日期与“权重”口径需要）。

| 北京时间 | 名称 | URL | 结果 | 存档 |
|---|---|---|---|---|
| 20:51 | ultramarine-blog | https://www.paperinstruments.com/blog/ultramarine | 200，772020 B，ok，1359 行 | `d-ultramarine-blog.*` |

## 热度（barking heat）

HN 搜索取“命中数/前 N 条”，由工具写入 `d-heat.json`（北京时间，抓取时刻 20:52）。

- **运行一（20:52:16）**：HN 四组查询（“RTX Spark”、“Ultramarine”、“solving code review at anthropic”、“Waymo Toronto”）。写入 `d-heat.json`。
- **运行二（20:52:44）**：该工具“拒绝覆盖已有 `d-heat.json`”，报错未写入。
- **运行三（20:52:44 起）**：先把运行一产出的 `d-heat.json` 移到 scratchpad（`d-heat-first-run.json`，不在仓库内，内容与运行三相同的四组结果一致），再一次性运行全部查询（HN 六组、Reddit 三组），写入新的 `d-heat.json`。以运行三为准。

运行三结果（HN，北京时间）：

| 查询 | HN 条目 | 分/评 | 时刻（北京） | 标题 |
|---|---|---|---|---|
| RTX Spark | 50038461 | 6 / 5 | 10-11 08:20 | Microsoft Launches Surface Laptop Ultra, Surface RTX Spark Dev Box |
| RTX Spark | 50050540 | 5 / 1 | 10-12 10:20 | Nvidia, Microsoft Launch Windows PCs with RTX Spark and AI Agents |
| RTX Spark | 50118565 | 4 / 0 | 10-16 22:06 | Nvidia RTX Spark PCs Released（NVIDIA 上市稿） |
| RTX Spark | 50071267 | 3 / 1 | 10-13 21:18 | Nvidia Nemotron 3.5 Bolt（与 D1 无直接关系） |
| RTX Spark | 50119594 | 1 / 0 | 10-16 23:25 | Nvidia RTX Spark: AI Platform, Laptop Chip and DLSS 5 |
| RTX Spark laptops | （同上前三条） | — | — | 无 The Verge 主帖 |
| Surface Laptop Ultra | 50038461 | 6 / 5 | 10-11 08:20 | 同上 |
| Ultramarine | 50106747 | **279 / 90** | 10-16 01:06 | Ultramarine 1 Flash（paperinstruments.com/ultramarine） |
| Ultramarine | 50092655 | 24 / 1 | 10-15 02:32 | Ultramarine 1 Flash: Frontier Model 100X Cheaper |
| Ultramarine | 50093128 | 3 / 0 | 10-15 03:01 | Ultramarine Flash v1: scaling nonverifiable RL（技术报告页） |
| solving code review at anthropic | 50122951 | **382 / 352** | 10-17 03:04 | How the Claude Code team does code review（claude.dev） |
| solving code review at anthropic | 50116568 | 2 / 2 | 10-16 18:20 | 同上（早先提交） |
| Waymo Toronto | 50118981 | **315 / 416** | 10-16 22:44 | Waymo in Toronto |

Reddit（三组查询）：**全部失败**。三次均返回 `old.reddit.com/login/` 登录页（HTML，非 JSON），报错记入 `d-heat.json` 的 `errors`：
- “RTX Spark laptop”
- “Ultramarine 1 Flash”
- “Waymo Toronto”

## 辅助文件与清理

- 清单、引文核对表与热度旧文件均放在会话 scratchpad（本机临时目录，不在仓库内，未入库）：`d-manifest.tsv`、`d-manifest2.tsv`、`d-quotes.txt`、`d-heat-first-run.json`。
- 仓库内没有新建的自写脚本；`docs/2610/1016/sources/` 下 D 组文件仅为 `d-*` 存档与本日志、`evidence-d.md`。
- 未删除任何仓库文件。

## 入库与大小

- 文本存档（可入库）：`d-*.txt`、`d-*.headers`、`d-fetch-log.jsonl`、`d-heat.json`、`evidence-d.md`、`capture-log-d.md`。最大 `.txt` 为 `d-ultramarine-blog.txt`，56981 B，远低于 1 MB。
- HTML 原件（只存本地、不入库，待 Claude 传网盘登记 `offsite.tsv`）：`d-rtx-verge.html`（660365 B）、`d-rtx-nvidia.html`（83306 B）、`d-surface-verge.html`（547495 B）、`d-cr-claudedev.html`（229613 B）、`d-ultramarine.html`（168876 B）、`d-ultramarine-blog.html`（772020 B）、`d-waymo-toronto.html`（70168 B）。
- `.headers` 只存部分响应头（工具已省略其余头，避免暴露抓取地点与标识）。
- 隐私检查：对 D 组全部 `d-*` 存档与 `evidence-d.md`、本日志做了抓取者身份与本机路径检索（账号名、用户目录、仓库盘符路径、开发者账号），无命中。

## 失败与未完成

- 抓取：0 条 blocked、0 条 failed。
- 热度：Reddit 三组失败（见上）；HN 中未找到 The Verge RTX Spark 实测文的主帖。
- 派工单未要求 D 组截图、HN/Reddit 高赞质疑（仅 A/B/C 组）与“可能的吠点”节，故未做。
