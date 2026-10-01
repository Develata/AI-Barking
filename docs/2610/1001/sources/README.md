# 1001 来源入口与限制

本页供读者回到原始来源。页面存档与截图为北京时间 2026-10-02 00:3x 至 01:2x 抓取；期次目录按制作当日美国当地日期命名为 `1001`。派工文件为 `.handoff/2026-10-01-1001-evidence.md`。选题线索来自四份在对话中回贴、未单独存档的扫描（北京时间 10/1 10:05、23:07、23:07、23:09）；其中一份称“官方表里 Argon 只在 DeepSWE 领先”，经核与官方原图不符，未采用。

| 内容 | 来源 |
|---|---|
| Gemini 4 Argon 发布、开放范围、入门价与脚注 | [Google：Gemini 4 Argon](https://blog.google/innovation-and-ai/models-and-research/gemini-models/gemini-4-argon/)、[Google DeepMind 官方 X 帖](https://x.com/GoogleDeepMind/status/2105388084154056939) |
| 官方对比表与测试方法 | [对比表原图](https://storage.googleapis.com/gweb-uniblog-publish-prod/original_images/gemini-4-argon_table_blog.gif)、[Methodology](https://deepmind.google/models/evals-methodology/gemini-4-argon) |
| 第三方评测（指数、幻觉率、准确率、成本） | [AA 文章](https://artificialanalysis.ai/articles/gemini-4-argon-google-top-three-labs)、[AA 模型页](https://artificialanalysis.ai/models/gemini-4-argon)、[AA 编码 agent 对比](https://artificialanalysis.ai/agents/coding-agents/comparisons/antigravity-cli-vs-codex) |
| “解决了幻觉”社区说法 | [Reddit r/singularity](https://www.reddit.com/r/singularity/comments/1wuj72j/gemini_4_argon_solved_hallucinations/) |
| OpenAI 蒸馏行动 | [OpenAI：Disrupting a coordinated model-distillation campaign](https://openai.com/index/disrupting-a-coordinated-model-distillation-campaign/) |
| 月之暗面未立即回应（媒体） | [CNBC](https://www.cnbc.com/2026/10/01/openai-chinas-moonshot-ai-kimi.html)、[The Register](https://www.theregister.com/security/2026/09/30/irony-alert-openai-whines-that-chinese-model-stole-its-special-ip-that-it-stole-from-everybody-else/5300285)；另见 [CyberScoop](https://cyberscoop.com/openai-moonshot-ai-model-distillation-attack/) |
| 背景研究（窗口前，未入正文） | [arXiv 2608.09867](https://arxiv.org/abs/2608.09867)、[研究者 9 月更新 PDF](https://stolen-thoughts.com/stolen_thoughts_update.pdf) |
| Anthropic 机器人研究 | [Anthropic：What work can robots do?](https://www.anthropic.com/research/what-work-can-robots-do)（页内 PDF 与 Appendix A–F 另存 `c-anthropic.pdf`、`c-appendix.pdf`） |
| 另一篇机器人实验（未入正文，勿与就业暴露口径混用） | [Anthropic：Claude plays robotics](https://www.anthropic.com/research/claude-plays-robotics) |

## 阅读限制

- Argon：官方页只标 “Sep 30, 2026”，无时刻与时区；“北京时间 10 月 1 日凌晨”取自 Google DeepMind 官方 X 帖时间（UTC 9/30 20:03:30）。对比表是谷歌自家对比，表内只有 Astra、Fable 5.1、Opus 5.5 三个对手，“最高 13 项”只在这 4 个模型中比较；DeepSWE 的 Argon 成绩为谷歌自测。AA 幻觉率“最低”限 Intelligence Index 45 分以上的模型，Argon 为 high、Astra 为 max，档位不同。AA 称入门价至少一个月，谷歌未给截止日。
- Argon 的 Bloomberg 报道（内部员工质疑编码表现、谷歌否认）只核到付费墙外导语，员工原话与否认细节未核到，不入正文；Terminal-bench 4.0 中 Opus-5.5 的数值，谷歌表（66.4%）与 AA（60%）冲突，来源条件不同，未取舍。
- 蒸馏：OpenAI 页只给日期（September 30, 2026），无时刻与时区，官方 X 帖未找到；报告写 users 而非账号，写 16,000 requests 而非“约”，脚注写明是尝试；页面未给归因依据。Moonshot/Kimi 官方回应未找到（已查官网、官方 X、研究博客、微博与公众号的可见内容，不宣称完整检索）；CNBC 与 The Register 的“未立即回应”只限各自截稿时点。研究者更新 PDF 记载 9/27 Azure 缓解、9/28 原提取无法复现，旧漏洞状态不能当作现状。
- 机器人：74% 与 34% 是物理任务及全部工作时间的加权份额，不是岗位占比；0.3% 是成本低于人工的工作任务份额；“40 年”是年降 3% 的外推情景，作者保留新型机器人与制造流程改写路径的可能；暴露等级、时间占比与成本估算由 Claude 联网完成，附录有稳健性检查。官方 X 帖时间未找到。社交热度偏低（Reddit 5 分，HN、X 未定位到相关帖），靠 Yahoo Finance 等媒体跟进。
- 热度数字为北京时间 10/2 00:45–00:57 的抓取瞬间值，不是扫描时点，也不代表全站最高。
- 取证清单（[evidence.md](evidence.md)）、抓取日志（[capture-log.md](capture-log.md)）与[事实核验](fact-check.md)均保留为工作档案；配图说明见[这里](../images/README.md)。`*.py`、`*.ps1` 为取证时使用的抓取与整理脚本；`d-rejected-*.png` 是版面错位的失败截图，仅作诊断，不是配图；`c-anthropic.pdf` 约 12 MB，为官方 PDF 原件。
- X、Reddit 档案只保存公开帖子的文本、时间与链接，不含抓取者信息；档案里出现的第三方用户名来自公开帖文。

发现影响正文的错误时，在本期增加 `CORRECTION.md`，保留更正原因与来源。
