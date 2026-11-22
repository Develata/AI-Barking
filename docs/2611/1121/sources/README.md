# 1121 来源与制作状态

期号1121（上期1120+1）；线索为 Develata 回贴的 ChatGPT 定时任务 stateful rescan（`AI-Barking_stateful_rescan_20261122T110000Z.md`，扫描终点北京 11-22 19:10）与 WorkBuddy HTML 选题扫描 1122-18（生成于北京 11-22 18:03）。Claude 北京 22:20–22:55 现查 HN（Algolia）、AIHOT 精选与一手页面后提候选；Develata 北京约 22:55 定主帖：①AI 推理供应链提示词泄露（OpenRouter + Together 的自家公告）；②Claude 发现的 3SUM/APSP 算法论文（10-05 已挂 arXiv，11-22 重新被转发）。风控：Novita→MiniMax 转发与 Dragos 报告里涉及中国公司的段落只存档、不进正文与卡片（Develata 同时拍板）。

派工单 `.handoff/2026-11-22-1121-evidence.md`（`barking handoff` 生成的文件名日期 11-21 与“本机时区为美东”有误，已改为 11-22 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor`（北京 22:55）：Browser Bridge 扩展未连接，截图全部走 `barking shot`。A、B 组 Sonnet 5.5，C 组（速览）Haiku 5.5，北京约 22:58 派出。

扫描勘误（写进派工单与本期核验）：
- ChatGPT rescan 称 3SUM/APSP 论文“正文没有说 Claude……新发现”并按旧闻拒发；论文第 1 页摘要后写明“Claude, an AI model developed by Anthropic, discovered the algorithm”，日期 10-05 判断正确。
- ChatGPT rescan 称 Together 整改含“PR 密钥检测”：属实（整改第 2 条“Checking all pull requests for credentials.”）。派工单“已知情况”里 Claude 写的“未见‘PR 密钥检测’”是初核读漏（文本过滤时丢了这条短行），A 组纠正；派工单原文不改。
- WorkBuddy 1122-18 称中文标题都写“111 名考生 AI 作弊”；光明网稿写“查处了111名违规携带智能设备的考生”，未夸大；中央社写“111起涉及電子設備的違規案件”，《中央日报》英文版标题写 98 人，数字口径未定（本期不收，另涉风控）。
- 机器码实验帖（HN 50546587）提交于 11-18，在窗口前；ChatGPT rescan 写“first_seen 11-22”。

## 主帖

1. AI 推理供应链提示词泄露（A 组，`evidence-a.md`、`capture-log-a.md`）：
   - [OpenRouter 公告](https://openrouter.ai/blog/announcements/data-processing-incidents-on-inference-providers/)（页面 11/22/2026，官方未给时刻）；[OpenRouter ZDR 文档](https://openrouter.ai/docs/guides/features/zdr)与 ZDR 博文（`a-or-zdr.txt`、`a-or-zdr-blog.txt`）。
   - [Together 安全通报](https://www.together.ai/blog/security-event-prompt-leakage-for-requests-in-june-and-july-of-2026)（Published 11/21/2026，署名 Vipul Ved Prakash）；[Together ZDR 文档](https://docs.together.ai/docs/zero-data-retention)（北京 23:08 取）。
   - [Dragos 报告](https://www.dragos.com/blog/ai-supply-chain-security-canary-tokens)（datePublished 2026-11-19T13:00Z，作者 HD Moore）。
   - 夸大实例：[The Hacker News](https://thehackernews.com/2026/11/openrouter-drops-3-providers-after.html)（未进正文）。另存 Phala 初步报告、Ars Technica、Novita 通告（仅存档，风控）。
   - 热度（HN，北京 23:03）：OpenRouter 帖 224 分 / 79 评，Dragos 帖 89 / 21，Together 帖 37 / 8。
2. 3SUM 论文（B 组，`evidence-b.md`、`capture-log-b.md`）：
   - [arXiv 2610.06783](https://arxiv.org/abs/2610.06783)（v1 2026-10-05 17:44:29 UTC，只有 v1）；[Lean 形式化](https://github.com/anthropics/formal-math/tree/main/3sum-apsp)（提交 2026-10-06T04:00:29Z）。
   - [Vassilevska Williams 个人页](https://people.csail.mit.edu/virgi/thoughts.txt)（无日期，首句称论文挂出一周后）。
   - 转发实例：[搜狐号“财法观天下”](https://www.sohu.com/a/1104699242_122066678)（11-22 04:27）；新浪微博 IT 时代网（11-22 04:32）；源头为海外社媒 Rohan Paul 帖（北京 11-22 02:47:26）。
   - 热度：HN 仅 10 月两帖（110 分 / 50 评、33 / 4），11-22 无新帖；AIHOT 11-22 热度 77。

## 速览（C 组，`evidence-c.md`、`capture-log-c.md`）

- 腾讯 Hy4：[Hugging Face 模型卡](https://huggingface.co/tencent/Hy4)与 Tencent Hy License（第 10.2、10.3 条）。
- Anthropic IPO 交割：[官方新闻](https://www.anthropic.com/news/ipo-close)（Nov 20, 2026）。
- Sutherland 复核页：[math.mit.edu/~drew/grh/](https://math.mit.edu/~drew/grh/)（last updated 2026-11-22 14:10 UTC）。
- OpenAI MedChemBench：[官方页](https://openai.com/index/introducing-medchembench/)（November 20, 2026）。
- Runway：[工程博客](https://runway.com/news/engineering/building-a-software-factory)（November 20, 2026）。

## 待删除与不入库

`images/` 下作废裁切与旧底图、A 组含提交者姓名邮箱的 `sources/a-gh-xorl-commits.json`、404 与质询页桩，在提交前一次删除（清单见 Claude 交付说明）。`a-or-changelog.txt`（2.1 MB）、`a-or-provider-routing.txt`（1.3 MB）超过 1 MB，已删去；同名 `.html` 原件随网盘保存，需要时用 `barking html2txt` 重生成。
