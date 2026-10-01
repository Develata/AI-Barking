# D 组检索范围与未采纳线索

2026-09-28 北京时间约 00:02–00:11，以联网搜索发现线索，再用 curl 或 OpenCLI 访问原站。以下只是检索记录，不把搜索摘要升级为证据。未使用镜像、代理域名或转载站替代打不开的原文。

检索词：

- `"DeepSeek" "DSec" "380,000"`
- `"Claude" "enzyme" "950"`
- `"The first real AI worms"`
- `DeepSeek "38万" "越狱"`
- `Claude "CRISPR" "发现" 2026 9 23`
- `DeepSeek "380,000 agents"`
- `OpenAI "蠕虫" "首" 2026`
- `"DeepSeek" "越狱" "DSec"`
- `"Claude" "全新" "CRISPR" "2026"`
- `"ChatGPT" "worm" "September" "2026"`
- `"DeepSeek" "38万" "智能体"`
- `"Autonomous AI agents discover reverse transcriptases with tandem repeat arrays"`
- `"Claude" "基因编辑工具" "2026" "9月24"`
- `"DeepSeek" "DSec" "越狱成功"`

原站已采纳的候选：

| 组 | URL | 结果 |
|---|---|---|
| D-A | https://www.techtimes.com/articles/328046/20260925/deepseek-training-agents-hacked-their-own-sandboxes-escape-catalog-now-public.htm | 原站 HTML、可读文字、标题截图；标题有 Escape Catalog Now Public |
| D-B | https://finance.sina.com.cn/roll/2026-09-24/doc-iniswvxc5441101.shtml | 原站刊载页及截图；署来源智药局，未找到并确认最初首发 URL，不冒称新浪原创 |
| D-B | https://zgeo.net/news/anthropic-multi-agent-art-gene-editing-geo-guide | 原站及截图；标题基因编辑新突破，FAQ 称新型基因编辑系统 |
| D-C | https://www.reddit.com/r/OpenAI/comments/1wr78yj/ | 用 agent-reach 的 OpenCLI 后端读帖；标题存在，另补 DOM 元数据与英文标题裁剪 |
| D-C | https://www.reddit.com/r/artificial/comments/1wr7ayr/ | 同账号另帖；读帖和 DOM 元数据，不重复占截图名额 |

未采纳/未找到：

- D-A “同时运行 38 万个 agent”明确标题未找到；所见 explainx.ai、Pandaily、OpenAI Hub、太平洋科技等检索条目仍写 sandbox，不将其改造成预设夸大例子。
- D-C “ChatGPT 被蠕虫攻破”本事件原站例子未找到；搜索命中的其他日期/其他漏洞事故不纳入。
- The Verge 原站已归档，标题 “Anthropic’s biolab made a discovery it’s comparing to Crispr” 保留 comparing，不采纳为“发现新 CRISPR”的证据：https://www.theverge.com/ai-artificial-intelligence/999470/anthropic-biolab-claude-crispr 。
- 未访问搜索结果中的 workers.dev 镜像，也未用中文聚合翻译充当 The Verge 原站标题。
- B6 标题搜索出现 alphaXiv 的不规则标识 `2609.ai-agents-discover-reverse-transcriptases` 以及 2025/arXiv 元数据，非本任务所求原始发表平台证据，未据此填写平台、编号、日期；未访问该站替代官方报告。
- 搜索服务缓存 Reddit 分数曾为 r/OpenAI 313 / r/artificial 282；它们只作搜索线索，本轮原站 adapter 为 318/285，DOM 为 316/285；不同来源不取舍覆盖，正文热度表以标明时点的原站快照呈现。
- 没有直接搜索微博、知乎、小红书的站内数据库；中文覆盖为公网索引发现的中文科技/财经页面，不能声称“全中文平台穷尽”。

发现线索不代表例子中的全部内容错误；尤其报告明确支持搜索阶段 without human intervention，因此不能单凭新浪标题“零干预”判错。这里只交原措辞和一手限制，留给后续审稿。
