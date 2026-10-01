# 0930 来源入口与限制

本页供读者回到原始来源。页面存档与截图为北京时间 2026-09-30 23:5x 至 10-01 凌晨抓取；期次目录按制作当日美国当地日期命名为 `0930`。派工文件为 `.handoff/2026-09-30-0930-evidence.md`。选题线索来自五份在对话中回贴、未单独存档的扫描（北京时间 9/30 10:05、22:00、22:50–22:51）；另一份本地扫描 `.handoff/2026-09-30-0930-scan.md` 未用于本期选题。

| 内容 | 来源 |
|---|---|
| GPT-6.1 Sol 发布、可用范围、标准价 | [OpenAI：Introducing GPT-6.1 Sol](https://openai.com/index/introducing-gpt-6-1-sol/)、[OpenAI X 帖](https://x.com/OpenAI/status/2104986129686741046) |
| 三款模型当前价格 | [OpenAI 开发者文档：Pricing](https://developers.openai.com/api/docs/pricing)、[GPT-6 Sol 模型页](https://developers.openai.com/api/docs/models/gpt-6-sol)、[GPT-6.1 Sol 模型页](https://developers.openai.com/api/docs/models/gpt-6.1-sol) |
| 第三方评测（指数、每题成本、token、编码 effort） | [Artificial Analysis：GPT-6.1 Sol replaces GPT-6 Sol after just 7 days](https://artificialanalysis.ai/articles/gpt-6-1-sol-replaces-gpt-6-sol-after-just-7-days-with-near-astra-intelligence)、[AA 模型页](https://artificialanalysis.ai/models/releases/gpt-6-1-sol) |
| GLM-5.3 网络能力报告 | [Anthropic：GLM-5.3 and the spread of advanced cyber capabilities](https://www.anthropic.com/research/glm-5-3-and-the-spread-of-advanced-cyber-capabilities) |
| GLM-5.3 开放权重与许可证 | [Hugging Face：zai-org/GLM-5.3](https://huggingface.co/zai-org/GLM-5.3) |
| CAISI 的独立评估（未入正文） | [NIST：CAISI’s assessment of Z.ai’s GLM-5.3 cyber capabilities](https://www.nist.gov/news-events/news/2026/09/caisis-assessment-zais-glm-53-cyber-capabilities) |
| 行政令 | [白宫：Inaugurating the Era of Super Intelligence](https://www.whitehouse.gov/presidential-actions/2026/09/inaugurating-the-era-of-super-intelligence/) |
| 同日 Fact Sheet | [白宫 Fact Sheet](https://www.whitehouse.gov/fact-sheets/2026/09/fact-sheet-president-donald-j-trump-inaugurates-the-era-of-super-intelligence/) |
| 现行 AI 法定定义 | [15 U.S.C. 9401](https://uscode.house.gov/view.xhtml?req=title:15%20section:9401%20edition:prelim) |
| ai.gov | [ai.gov](https://www.ai.gov/) |
| 国际单位制（SI） | [NIST SP 330](https://www.nist.gov/pml/special-publication-330) |

## 阅读限制

- GPT-6.1 Sol：标题里的“五分之一”是 token 单价（相对 Astra），上一代 GPT-6 Sol 已是同一价位；发布页另有按任务成本的 1/5、1/7 等说法，是 OpenAI 自测，与单价是两种口径。AA 的 $0.72 / $3.26 是其指数的每道题平均成本；xhigh 比 max 高 3 分是这次观测。
- GLM-5.3：报告出自 Anthropic（竞争对手），未声明利益冲突。0% / 64% / 92% / 100% 是模拟环境中“尝试连接目标”的比例，没有执行任何代码；ExploitBench 的 50/410 与 56/410 在隔离沙箱内完成，Claude 模型关掉了防护。预填思考、改权重两种手段 Claude API 不允许，无法同条件对比。
- 行政令：第 3 节定义沿用现行法律的 AI，60 天内由科技助理提出立法建议；既有法规、合同与历史文件不必修改。ai.gov 截图只代表北京时间 10/1 00:01 的状态，行政令未规定网站改名期限。
- 取证清单（[evidence.md](evidence.md)）、抓取日志（[capture-log.md](capture-log.md)）与[事实核验](fact-check.md)均保留为工作档案；配图说明见[这里](../images/README.md)。`*.py` 为取证时使用的抓取脚本。

发现影响正文的错误时，在本期增加 `CORRECTION.md`，保留更正原因与来源。
