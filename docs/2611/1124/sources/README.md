# 1124 来源与制作状态

期号1124（上期1123+1）；线索为 Develata 回贴的 WorkBuddy HTML 选题扫描 1125-18（生成于北京 11-25 18:02）。同时附来的 ChatGPT rescan 邮件为 11-19 13:16 的旧件（NGINX、Gemini 4 Neon 已于 1118 发过，IPO 进过 1121 速览），未用。Develata 北京 11-26 约 02:00 选定主帖：MAGI-2 与 Copilot 论文。

派工单 `.handoff/2026-11-26-1124-evidence.md`（`barking handoff` 生成的文件名日期 11-24 与“本机时区为美东”有误，已改为 11-26 与北京时间）。派工前 `opencli doctor`（北京 02:00）：Browser Bridge 已连接。A、B、C 三组均为 Sonnet 5.5（effort high），北京 02:01–02:47。

扫描勘误：
- 扫描“不建议发”一节按自己前几轮去重，不按 `daily-scan.md`“已发过”：矩阵乘法 9/4（HN 904 分）、Tidal、Anthropic 跌破发行价都没发过（Claude 查 HN 与“已发过”）。
- 扫描称 MAGI-2 在 I2V 1165 对 1161：A 组 02:03 读到 1163 对 1158，属 Elo 漂移；1161 可能是 MAGI-2 入榜前 Seedance 2.5 的读数（推断）。
- 扫描称“12 切点平均误差 0.028 秒”“seeddance.io 称前三 9 分内可翻转”“BlockBeats 便宜约 87%”：A 组均未找到；媒体稿写“最大偏差 0.08 秒”。
- 扫描称“40% 不是论文主结论，是第三方推算”：与原文不符。论文 p.3 与 §6.2.3 自己写“roughly 40%”（compliers 中的初级软件工程师，Wald −0.513 对数点）。
- 扫描称 Rohan Paul 转发于 13:43：发帖为北京 13:29:48，13:43 是聚合站收录时刻（B 组）。
- 扫描称混元 Hy4-flash GGUF 官方表里没有 64GB Mac mini：模型卡 11-25 14:23 的修订已加 M4 Pro 64GB 一行（解码 17.3 tok/s）；扫描所引“四档”“410.27 GiB”与官方原话在 HF README 各修订里未找到（C 组）。
- 扫描称 ABot-World“WBench 双第一”：只见自提交行，未进速览（C 组）。

## 主帖

1. MAGI-2（A 组，`evidence-A.md`、`capture-log-A.md`）：
   - [Artificial Analysis 图生视频榜](https://artificialanalysis.ai/video/leaderboard/image-to-video)、[文生视频榜](https://artificialanalysis.ai/video/leaderboard/text-to-video)（各含带音频 / 无音频视图，北京 11-26 02:03–02:05 读；MAGI-2 标“Provisional result”）。
   - [Sand.ai 官方博客](https://sand.ai/blog/magi-2/)（November 24, 2026）；官方发布帖（北京 11-25 02:09，海外社媒）；[platform.sand.ai 价格文档](https://platform.sand.ai/docs/model-pricing)。
   - 夸大实例：[新浪财经](https://finance.sina.com.cn/tech/shenji/2026-11-25/doc-inizxpkf9563675.shtml)（“AA榜全球第一，价格仅为Seedance2.5 1/10”）；投资界、网易科技、上证报；KuCoin 转载 BlockBeats 英文快讯写成“文生视频榜第一”。较准确：爱范儿、智东西、ZPedia。
   - 价格参照：[火山方舟模型价格](https://docs.volcengine.com/docs/ark/model-pricing)（Seedance 2.5 1080p 折合 3.74 元/秒）。
   - 热度：HN 无相关帖；Reddit 被登录墙拦下。
2. Copilot 论文（B 组，`evidence-B.md`、`capture-log-B.md`）：
   - [arXiv 2611.31083](https://arxiv.org/abs/2611.31083)（v1 北京 11-24 01:33；PDF 存本地）；7 月 NBER 幻灯片（月度 36%，B 组存档）。
   - 夸大实例：[爱查 AI](https://www.aichaai.com/ai-hotspots/101771)（14:30）；AI 圈报；Rohan Paul 帖（海外社媒）；BedrockNews 把 29% 写成初级岗降幅。
   - 热度：HN 无；Reddit 被登录墙拦下。

## 速览（C 组，`evidence-C.md`、`capture-log-C.md`）

- Tidal：官方账号长文（海外社媒，北京 11-24 22:22）；[Variety](https://variety.com/2026/music/news/tidal-bans-ai-generated-music-1236956924/)。
- OpenAI 9/4：[openai/math 预印本](https://github.com/openai/math/blob/main/preprints/The-matrix-multiplication-exponent-is-nine-fourths-November-24-2026/paper.pdf)。
- 高德 ABot-World：[GitHub amap-cvlab/ABot-World](https://github.com/amap-cvlab/ABot-World)（README 自报 Iris 5B 单张 RTX 5090 21 FPS）。
- 腾讯 Hy-MT3：[Hugging Face tencent/Hy-MT3-30B-A3B](https://huggingface.co/tencent/Hy-MT3-30B-A3B)。
- Mantic × Thinking Machines：[thinkingmachines.ai](https://thinkingmachines.ai/news/training-a-scalable-forecasting-agent)。
- 未收：腾讯 Hy4-flash GGUF（Mac mini 条件 40 字内写不全，反向核验后撤下）、LessWrong 侧信道（个人研究帖，限定词多）、Google Cogentic 整数分解（数学题材已收一条）、Anthropic 股价（11-25 收盘取证时未出）、千问 Go（无官方一手）、PrismML。

## 待删除与不入库

（交付时补）
