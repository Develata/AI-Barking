# 0927 来源入口与限制

本页供读者回到原始来源。页面存档与截图为北京时间 2026-09-27 深夜至 09-28 凌晨抓取；期次目录按制作当日美国当地日期命名为 `0927`。派工文件为 `.handoff/2026-09-27-0927b-evidence.md`（同日另一个 `2026-09-27-0927-evidence.md` 属于上一期，该期目录已更名为 `docs/0926/`）。

| 内容 | 来源 |
|---|---|
| DSec 规模、agent 行为与事故 | [arXiv 2609.22978：DeepSeek Elastic Compute (DSec)](https://arxiv.org/abs/2609.22978)，第 6.4、6.5 节 |
| “逃逸清单”标题写法 | [Tech Times](https://www.techtimes.com/articles/328046/20260925/deepseek-training-agents-hacked-their-own-sandboxes-escape-catalog-now-public.htm) |
| 酶系统公告 | [Anthropic：Claude discovers a novel enzyme system with CRISPR-like repeats](https://www.anthropic.com/news/claude-discovers-novel-enzyme-system) |
| 酶系统预印本 | [Autonomous AI agents discover reverse transcriptases with tandem repeat arrays（Anthropic 官方 PDF）](https://www-cdn.anthropic.com/22573675ada52a8ca8a97a1a4b4326b2f208a071.pdf) |
| “基因编辑新突破”写法 | [智脑时代 ZGEO](https://zgeo.net/news/anthropic-multi-agent-art-gene-editing-geo-guide) |
| 自我复制提示注入 | [OpenAI：Self-replicating prompt injections exist](https://alignment.openai.com/misalignment-reports/self-replicating-prompt-injections-exist/) |
| “AI 蠕虫已在 agent 间传播”写法 | [Reddit r/OpenAI](https://www.reddit.com/r/OpenAI/comments/1wr78yj/)、[r/artificial](https://www.reddit.com/r/artificial/comments/1wr7ayr/) |

## 阅读限制

- DSec 的规模数字是 DeepSeek 自报的单个 scale unit 生产数据，无第三方核实；第 8 章性能实验在另一套 10 节点测试集群上完成。
- “论文没写逃逸”“预印本未见 bioRxiv/arXiv/DOI 版本”“OpenAI 报告没演示连环感染”均为否定性结论，依据是对存档全文的检索与通读，见 `fact-check.md` 备注。
- Anthropic 公告与预印本的规模数字不同（约 950 agents / 21 小时 / 2.1 亿 token 对 949 sessions / 21.5 小时 / 2.156 亿 token），正文取公告约数。
- Anthropic 与 OpenAI 页面日期均无时区，正文照录原文日期。
- Tech Times 后半篇讨论的 CVE-2026-82533 属于 DeepSeek Harness，不是 DSec。
- 正式配图、原始截图、取证清单（[evidence.md](evidence.md)）、抓取日志（[capture-log.md](capture-log.md)）与[事实核验](fact-check.md)均保留为工作档案；配图说明见[这里](../images/README.md)。

发现影响正文的错误时，在本期增加 `CORRECTION.md`，保留更正原因与来源。
