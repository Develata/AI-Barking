# 1022 期 C 组事实清单

C 组只核派工的三条速览；不截图。以下事实句均不超过 40 字。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/北京时间 | 状态 |
|---|---|---|---|---|---|---|
| C1：OpenAI 与科研伙伴以数千 agent 分析小鼠脑细胞数据，32 分钟 | L1 | https://openai.com/index/scaling-ai-research-for-complex-biological-data/ | “used thousands of AI agents powered by GPT‑Rosalind to analyze and interpret existing experimental data from millions of mouse brain cells in just 32 minutes.” | 无（C 组不截图） | OpenAI 页面日期为 2026-10-21，官方未给时刻。分析对象是既有实验数据；文章描述的是可供后续验证的生物学联系与研究方向，不是治疗效果或临床结果。 | 已找到 |
| C2：Mistral 发布 Voxtral 3 语音产品 | L1 | https://mistral.ai/news/voxtral-3/ | “Accurate transcription across 50+ languages.” / “Voxtral Small Transcribe 3” | 无（C 组不截图） | 页面 JSON-LD 的发布时间为 2026-10-20 12:00:22 UTC（北京时间 2026-10-20 20:00:22），修改时间为 2026-10-21 09:56:32.450 UTC（北京时间 2026-10-21 17:56:32.450）。50+ 指 Small Transcribe 3 转写；同页对 Realtime Transcribe 3 分别写 35+ 与 40+ 语言，TTS 写 35+ 语言、50+ 方言，原文数字有差异，不能泛化为所有模块。 | 已找到 |
| C3：Google 研究用盲扫超声估算孕周 | L1 | https://research.google/blog/empowering-frontline-maternal-care-with-ai-assisted-ultrasound/；关联新研究：https://jamanetwork.com/journals/jamanetworkopen/fullarticle/2851451；前期研究：https://www.nature.com/articles/s43856-022-00194-5 | “primary cohort of 385 participants between 16 and 36 weeks of pregnancy” / “fetal age from blind sweep ultrasounds” | 无（C 组不截图） | Google Research 页面日期为 2026-10-21，官方未给时刻；范围是孕 16–36 周的孕周估算，不是全面诊断。关联 JAMA Network Open 页面本轮返回 403 Cloudflare challenge，未能核实论文自身发布日期。Google 引用的前期 Communications Medicine 论文发表于 2022-10-11；不要把旧论文日期写成新介绍或新研究的日期。 | 已找到 |

## C 组事实摘句

- C1：OpenAI 等称：数千智能体32分钟梳理既有数百万小鼠脑细胞数据，提出线索。
- C2：Mistral 称，Voxtral-3 的转写支持50+语种。
- C3：Google 称，AI 盲扫超声在孕16–36周估算孕周。

## 扫描说法勘误

- C1：扫描的“既有小鼠脑细胞数据、32 分钟”与 OpenAI 页面一致；页面描述的是从已有实验数据提出研究线索，后续机制仍需实验验证，不支持写成疾病治愈或已产生临床效果。
- C2：50+ 语言只对应 Voxtral Small Transcribe 3 的转写。页面对 Realtime Transcribe 3 在正文写 35+、型号卡写 40+；TTS 为 35+ 语言和 50+ 方言。保留这处原文数字差异，不概括成所有模块都支持 50+ 语言。
- C3：Google 页面说的是盲扫超声的孕周估算；主队列为孕 16–36 周，不能扩写成全面孕期诊断。2026-10-21 是 Google Research 新介绍的页面日期，旧 Communications Medicine 论文的正式发表日为 2022-10-11；关联 JAMA 论文的页面本轮被 Cloudflare 拦截，具体发布日期未核实。
