# 1023 证据汇总

用户确定主线：Kimi-K3.1-Pro 网络能力取舍、Block Dessa、Anthropic 与英国 AI 安全研究所事故重放；速览 Duet 记忆、Google 结核筛查、Brotli。两份扫描是线索，不作为既有核验。逐条证据、摘句、勘误与夸大实例见分组文件：

- `evidence-a.md`：Kimi（博客、AA、HF、价格页、媒体）
- `evidence-b.md`：Dessa（官方页、Wayback、负责人帖、媒体）
- `evidence-c.md`：Anthropic / AISI（研究页、AISI 两篇博客与两份 PDF、媒体）
- `evidence-d.md`：速览（Duet、Google、Brotli、Railway）

## 主代理复核（Claude 在原件上亲自核对）

- Kimi 官方图表 xdev-bench-hard：对照版90.5、K3.1 Pro 81.0、Kimi K3 33.3（同 kimi-code harness），GPT-5.6 Sol 行标 Third-party evaluation（images/02-kimi-xdev-chart-raw.png）。
- AA 原文“launches at 4th … scoring 55”（a-aa-article.txt 第1、28行）。
- Dessa：HTML 含“The waitlist opens soon.”“Download coming soon.”；负责人帖原文“first AI model … trained entirely on speech!”。
- Anthropic 图16数值（Mythos 5 83%、Opus 5.5 6%、Mythos 5.1 13%、内部模型0%、Opus 4.7 19%）；摘要67%与正文62%不一致；同页另处写87%，原文未解释差异。
- AISI 技术报告 §3.2：Mythos 5 60%、GPT-6 Astra 32%；第2节排除 GPT-6 Astra（模拟意识过高），第3节纳入。

## 扫描说法勘误（汇总）

- WorkBuddy：Brotli 是“红队智能体在训练环境中发现”——无依据，OpenAI 训练环境报告全文无 Brotli，补丁页只称 Node.js 公告致谢 OpenAI Codex Security；“二月源码已修”只出自未打开的 X 帖，未找到一手；中文媒体把83%写成“87%”——取证组读的7篇无此写法，87%本身在 Anthropic 原文。
- ChatGPT：Kimi“由90.5%降至81.0%”易误读为相对上一代下降；“Reuters 直接采访”原站401未能核；速览表5条来源 URL 为空。
- 两份：AA Cyber Index 64% 在 AA 原站未找到。
