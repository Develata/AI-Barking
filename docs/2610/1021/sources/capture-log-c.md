# 1022 期 C 组采集日志

按北京时间（UTC+8）记录。C 组不截图，只归档原文、摘句并采样热度。所有归档正文与响应头由 `barking` 工具生成；逐条字节数、SHA-256 和结果见 `c-fetch-log.jsonl`。

## 原文存档

| 北京时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 2026-10-22 16:45:11 | https://openai.com/index/scaling-ai-research-for-complex-biological-data/ | `barking fetch`（批量清单 `c-fetch-list.tsv`） | HTTP 200，`ok`；456,774 字节，生成 `.html`、`.headers`、`.txt`。页面标注 2026-10-21，未给发布时间时刻。 |
| 2026-10-22 16:45:10 | https://mistral.ai/news/voxtral-3/ | `barking fetch`（批量清单 `c-fetch-list.tsv`） | HTTP 200，`ok`；585,227 字节，生成 `.html`、`.headers`、`.txt`。页面 JSON-LD：首次发布 2026-10-20 12:00:22 UTC（北京时间 20:00:22）；修改于 2026-10-21 09:56:32.450 UTC（北京时间 17:56:32.450）。 |
| 2026-10-22 16:45:10 | https://research.google/blog/empowering-frontline-maternal-care-with-ai-assisted-ultrasound/ | `barking fetch`（批量清单 `c-fetch-list.tsv`） | HTTP 200，`ok`；182,283 字节，生成 `.html`、`.headers`、`.txt`。页面标注 2026-10-21，未给发布时间时刻。 |
| 2026-10-22 16:47:35 | https://jamanetwork.com/journals/jamanetworkopen/fullarticle/2851451 | `barking fetch`（批量清单 `c-google-papers.tsv`） | HTTP 403，`blocked`；Cloudflare challenge，保存 `.html` 与 `.headers`，无 `.txt`。未重试或推测论文发布日期。 |
| 2026-10-22 16:47:43 | https://www.nature.com/articles/s43856-022-00194-5 | `barking fetch`（批量清单 `c-google-papers.tsv`） | HTTP 200，`ok`；418,374 字节，生成 `.html`、`.headers`、`.txt`。原文为 Communications Medicine 论文，正式发表日 2022-10-11。 |

两份输入清单分别为 `c-fetch-list.tsv` 与 `c-google-papers.tsv`。所有请求均通过批量存档执行，没有单 URL 手工重试。没有下载 PDF 或生成截图。单个文件均小于 1 MB。

## 热度采样

2026-10-22 16:46:26 使用 `barking heat`，窗口从 2026-10-15 16:46:26 起；原始结果在 `c-heat.json`。

- HN 搜索 OpenAI GPT-Rosalind / 小鼠脑细胞：0 条；Google AI-assisted ultrasound / gestational age：0 条。它们只表示本次 HN 查询没有匹配结果，不代表全网无人讨论。
- HN 搜索 “Mistral Voxtral 3” 命中官方发布页：50168396，761 points、198 comments；HN 时间 2026-10-21 00:02:07（北京时间），分数为本次采样时的快照。
- Reddit 三条对应查询均返回登录页 HTML，不能解析成搜索结果；没有评论数或热度数值可报，不作替代估计。
- AIHOT 线索来自派工消息：本轮页面已见脑细胞稿与超声稿。这里只记作聚合站热度线索（L5），未用作事实来源，也未独立读取页面或补写其互动数。

## 发布日期与范围

- OpenAI 官方页只标 2026-10-21；未标具体时刻。原文限定为对既有实验数据的分析，用途是提出研究线索。
- Mistral 官方页的 JSON-LD 提供 UTC 发布时间和修改时间，已换算为北京时间。50+ 语言对应 Voxtral Small Transcribe 3；同一页面对 Realtime Transcribe 3 写了 35+ 与 40+ 两种口径，均照录。
- Google Research 页面只标 2026-10-21；未标具体时刻。新介绍引用 JAMA Network Open 研究，但其官方页面返回 403 challenge，本轮无法核实该论文发布日期。页面引用的前期 Communications Medicine 论文正式发表于 2022-10-11；两者日期分开记录。
