# 1108 期 C 组抓取与检索记录

时间为北京时间（UTC+8）。存档文件在 `docs/2611/1108/sources/c-*`；机器可读日志见 `c-fetch-log.jsonl` 与 `c-heat.json`。

| 时间（北京） | 动作 | 工具 | 对象 | 结果 |
|---|---|---|---|---|
| 18:06:04 | 批量存档（清单 10 条） | `barking fetch` | nvidia-board-statement；nvidia-message；ernie-release；ernie-hf-card；google-hypertension-blog；nature-hypertension；claude-personal-finance；anthropic-agent-memory；ft-anthropic-generators；reuters-anthropic-generators | 8 条 ok。ft-anthropic-generators：HTTP 403，Cloudflare 质询（cf-mitigated: challenge），blocked，无 txt；未用浏览器重试（不过 bot 检测）。reuters-anthropic-generators：curl HTTP 401，blocked，无 txt，改用 opencli |
| 18:06:43 | 批量存档（2 条） | `barking fetch` | ernie-hf-license（raw LICENSE）；ernie-hf-license-blob（LICENSE 页面） | 2 条 ok |
| 18:07:18 | 批量存档（1 条） | `barking fetch` | claude-help-personal-finance（帮助中心） | ok |
| 约 18:07–18:08 | 隐私削减：`c-nvidia-message.txt` 与 `.html` 改写 | Bash 写入 | nvidia-message | 原页含与公司调查无关的私人内容，已删除，仅保留官方摘录段落（“Regarding the national security concerns ... will cooperate fully.”） |
| 约 18:08 | 热度（首次） | `barking heat` | HN 查询 ERNIE、Personal Finance、components of agent memory、hypertension smartwatch、Nvidia board investigation；条目 50416801 | 写入的 `c-hn-50416801.json` 含评论正文，已删为仅元数据。查询 “Nvidia board investigation” 命中一条涉及私人生活的媒体标题，已从 `c-heat.json` 删除，标题与内容未记入任何文件 |
| 18:09:35 | 热度（重取） | `barking heat` | HN 八组查询（ERNIE、Personal Finance、agent memory、AI-PPG-ACC-HTN、Google smartwatch hypertension、Anthropic gas generators OnePWR、Coinbase AI agent chief of staff、Brockman AI death option）；条目 50416801 | 旧文件移至 scratchpad 后重取。`c-hn-50416801.json` 再次削减为元数据（嵌套评论 326 条为工具输出数字，正文不存档）。`c-heat.json` 加 note |
| 约 18:10 | 打开 Reuters 原文 | opencli browser（session c-reuters，`open`、`eval` 取 innerText） | reuters-anthropic-generators | 正文取得，存为 `c-reuters-anthropic-generators.txt`（零宽空格已去除）；18:11 释放 session lease |
| 约 18:10 | FT 原文 | 未重试 | ft-anthropic-generators | 仅 curl 403 Cloudflare 质询；未用浏览器重试，未绕过质询与付费墙 |
| 约 18:11 | 检索 1 | WebSearch | “Coinbase CEO Brian Armstrong AI agent chief of staff” | 无 chief-of-staff 原文；线索见 evidence-C.md C7a（均未核） |
| 约 18:11 | 检索 2 | WebSearch | “Greg Brockman AI death ‘option’ interview 2026” | 仅聚合转述（L5），无原始访谈；见 evidence-C.md C7b |
| 约 18:12 | 检索 3 | WebSearch | 中文：Google 智能手表 高血压 AI Nature 2026 敏感度 65.8% | 找到 aiposthub.com、知乎文章与 Engadget 标题（未读全文）；Engadget 标题称 Pixel Watch 功能，未核，仅作线索 |
| 约 18:12 | 清理 | Bash（非 rm） | scratchpad `c-heat-old.json` | 内含私人生活标题，已清空为零字节；文件仍在 scratchpad，见回报待删除清单 |
| 约 18:13 | 清理 | Bash（非 rm） | scratchpad `c-hn-50416801-old.json` | 内含 HN 评论正文（私人生活讨论），已清空为零字节；待删除 |
| 约 18:13 | 占位 | Bash 写入 | `c-ft-anthropic-generators.html` | Cloudflare 质询页，已替换为一行占位注释（原页无正文）；原始 headers 保留 |

## 说明与假设

- 官方页面只给日期时，统一写“官方未给时刻”。Reuters 页面给出 GMT+8 时刻 20:54，即北京时间。
- 为 `barking heat` 设置的 HN 查询词与 Reddit 无关；Reddit 未查（C 组热度只需 HN）。
- Nature 论文只取到摘要与发表日期（05 Nov 2026），未核 Methods 与补充材料。
- ERNIE 许可证以 HF 上的 LICENSE 文本为准；许可证本身的版本号与发布日期（v1.0，November 5, 2026）来自该文件。
- NVIDIA 官方声明中点名了 CEO，按原文逐字引用（官方声明，非媒体所称的第三人）；媒体报道中的私人细节、国家与第三人姓名未存档、未摘录。
- C7 检索范围：每条 1 次 WebSearch，未对 Coinbase 或 Brockman 的官网、Davos 或 WSJ 视频逐一核对，结论“未找到”仅限本次检索。
