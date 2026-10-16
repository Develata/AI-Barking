# 1015 来源入口

正文：`../doc_1015_publish.txt`；配图：`../images/README.md`；事实核验：`fact-check.md`；取证清单：`evidence-a.md`（商务部、科技部技术出口目录）、`evidence-b.md`（Muse Spark Open）、`evidence-c.md`（ARC-AGI-3 与 Argon）、`evidence-d.md`（速览：产品与公司）、`evidence-e.md`（速览：研究与模型）；抓取日志：`capture-log-*.md`、`*-fetch-log.jsonl`、`*-shots-log.jsonl`。

取证执行方：A、B、C 组 Sonnet 5.5 子代理，D、E 组 Haiku 5.5（均 effort high），派工单 60 分钟目标。耗时：E 约 10 分钟、C 约 20 分钟、D 约 27 分钟、B 约 87 分钟（60 分钟时催交）、A 约 90 分钟（中途网络断开一次后续跑；为转换附件 1 的 `.wps` 用 Scoop 装了 LibreOffice 26.8.1）。opencli 扩展已连接。Haiku 第六次试用（D、E 两组）：摘句与口径完整，E 组主动指出 Goodfire 的 90% 是“neuron poles”而非神经元、Krea 演示中的 8 秒为示意时长；D 组发现钱包报告无“一次交易”说法、员工 HN 回复时刻的时区换算。

选题：一份 ChatGPT 扫描（北京 10/16 17:04）与一份 WorkBuddy 扫描（12:00）。选题前 Claude 派两个 Sonnet 代理逐条核一手页面。Develata 选商务部 AI 技术出口目录 + Muse Spark Open + ARC-AGI-3 harness 差距；中途追加“WorkBuddy 被指抄袭 Claude Code”，先定只进速览、不做软件包比对；两轮反向核验与视觉复核均判 BLOCKER（RuntimeWire 自述主要来源是社交帖、首发博客未找到，L4 资格存疑；指控只针对国际桌面版，速览 40 字内写不下），Develata 决定撤下。扫描勘误：ChatGPT 把丹麦案“113”写成子类数（实为特别严重类图片张数），Eiliko 远程改回答写成记者自己验证（实为研究者对记者的设备操作）；WorkBuddy 把 fast mode 写成只限 Opus 5.5（Opus 5、4.8 仍在研究预览），Muse 许可门槛写成 MaaS 业务收入（实为被许可方及关联方总收入），“ARC Prize 说过标准框架才是更有意义的门槛”无原句。风控排除：丹麦最高法院 AI 生成未成年人违法图像判决（题材）、PromptArmor 开放/闭源对比（开放一侧全为中国模型）、Eiliko（深圳厂商被指控）、Swarmchasers 腾讯智能体报告（中国公司被美方研究者指控）、WorkBuddy 抄袭指控（见上）。

计数说明：附件 1 的 AI 部分表格只有 11 行；“23项”按行内编号的控制要点计数，同口径禁止 14、限制 26、总计 40 全部吻合（推断口径，正文写“按附件逐条数”）。附件 PDF 为 LibreOffice 对官方 `.wps` 的渲染，版式非官方原貌。

## 一手来源

- 商务部 科技部公告2026年第48号：https://www.mofcom.gov.cn/zcfb/dwmygl/art/2026/art_61446ab4beee4f63973d4d0ab3097648.html （附件 1 `.wps` 由该页下载链接取得）
- 商务部新闻发言人答记者问：https://www.mofcom.gov.cn/zwgk/zcjd/art/2026/art_b11d5c1e9374421e93e85dc87eba6e38.html
- Meta 发布页：https://research.meta.ai/blog/muse-spark-open ；评测方法：https://research.meta.ai/static/muse-spark-open-evaluation-methodology ；HF 模型卡与许可：https://huggingface.co/meta-models/Muse-Spark-Open-816B-A42B ；PyTorch 博客：https://pytorch.org/blog/muse-spark-open/
- Artificial Analysis：https://artificialanalysis.ai/models/muse-spark-open
- Apple MLX 分布式推理：https://machinelearning.apple.com/research/mlx-distributed-inference
- ARC Prize 结果页：https://arcprize.org/results/google-gemini-4-argon 、https://arcprize.org/results/openai-gpt-6-astra ；Astra 博客：https://arcprize.org/blog/astra ；测试政策：https://arcprize.org/policy
- 速览：Claude fast mode https://platform.claude.com/docs/en/build-with-claude/fast-mode ；IT之家 https://www.ithome.com/1/013/848.htm ；RuntimeWire https://runtimewire.com/article/tencent-workbuddy-claude-code-internal-text-allegation ；OpenAI 失准报告 https://alignment.openai.com/misalignment-reports/ ；Anthropic Scion https://www.anthropic.com/engineering/redesigning-source-control 、sunshowers https://sunshowers.io/posts/anthropic-mononoke-2/ ；西蒙斯基金会 https://www.simonsfoundation.org/news/simons-foundation-launches-15m-effort-to-help-mathematicians-navigate-ai-era/
