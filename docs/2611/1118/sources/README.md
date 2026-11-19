# 1118 来源与制作状态

期号1118（上期1117+1）；线索为 Develata 回贴的两份扫描：ChatGPT 定时任务的 stateful rescan（`AI吠点-邮件-2026-11-19-1316.md`，扫描终点北京 11-19 13:13）与 WorkBuddy HTML 选题扫描 1119-18（北京 11-19 18:02）。Claude 北京 11-19 21:06 用 HN Algolia 现查热度并直取 AA、Decision、Wiz、谷歌、DeepSeek、nginx.org、CERT-EU、Censys 等一手页核对后列候选，Develata 选 NGINX + Gemini 4 Neon 两条主帖；Anthropic IPO 因当晚才开盘，只进速览。

派工单 `.handoff/2026-11-19-1118-evidence.md`（`barking handoff` 生成的文件名日期 11-18 与“本机时区为美东”有误，已改为 11-19 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor`：Browser Bridge 扩展已连接。A、B 组 Sonnet 5.5、C 组 Haiku 5.5；A 组因流停滞、B 组因额度用尽在写最终回报前中断，`evidence-*.md` 与 `capture-log-*.md` 均已写盘，Claude 直接读存档验收。

## 主帖

1. NGINX HTTP/2 漏洞：[Wiz 博客](https://www.wiz.io/blog/nginx-http2-rce-vulnerabilities)（页面 11-18）、[nginx.org 安全公告](https://nginx.org/en/security_advisories.html)与[首页新闻](https://nginx.org/)（2026-11-18 发布 1.30.7、1.31.9）、F5 公告 [K000163926](https://my.f5.com/manage/s/article/K000163926)（CVE-2026-123007）与 [K000163928](https://my.f5.com/manage/s/article/K000163928)（CVE-2026-123633）、[CERT-EU 2026-022](https://www.cert.europa.eu/publications/security-advisories/2026-022/)（北京 11-19 00:30）、[Censys 公告](https://censys.com/advisory/nginx2root/)与[追踪看板](https://nginx2root.censys.com/)、[Xint 博客](https://xint.io/blog/nginx2root-remote-code-execution)、[Rapid7 博客](https://www.rapid7.com/blog/post/ve-cve-2026-123007-critical-nginx-http-2-integer-underflow-leads-to-rce-fixed/)、Metnir 博客、Netcraft 九月调查、Ubuntu USN-9159-1、Debian 安全追踪。HN 50547374 1428 分 / 608 评（北京 11-19 21:06）。
   - “五队独立发现”是 Wiz 原话；F5 致谢逐 CVE 各四方、名单不同（123633 有 Unit 42，不在 Wiz 名单）。
   - AI 角色：Wiz 用 Atlas 发现、两大模型验证；Ikotas 用 GPT-6 Astra（F5 致谢）；Metnir 用流水线 + 公开模型；Rapid7 写明由 Stephen Fewer 发现，全文无 AI。
   - 评级：nginx.org 123007 major、123633 medium；F5 CVSS 3.1 8.1（High）、4.0 9.2（Critical）；Wiz 只写 9.2。
   - 夸大实例：团子技术日报“2.25亿网站待修”、CSO/The Register/Security Affairs 把“AI 找到”概括为全部；奇安信 CERT 写“EXP 状态 已公开”。
2. Gemini 4 Neon：[谷歌博客](https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-neon/)（北京 11-19 05:00）、[AA 测评](https://artificialanalysis.ai/articles/gemini-4-neon-intelligence-gain)（11-18，官方未给时刻）、AA [Neon](https://artificialanalysis.ai/models/gemini-4-neon) 与 [Carbon](https://artificialanalysis.ai/models/gemini-4-carbon) 模型页、[Gemini API 定价页](https://ai.google.dev/gemini-api/docs/pricing)（北京 11-19 21:3x 取）。HN 50552424 895 分 / 704 评。
   - Neon (high) $3.74 来自 AA 模型页内嵌图表数据与 AA 文章图 1 柱标签，AA 文字未写。
   - 促销期 Carbon (high) 约 $3.88 为本号推算（$7.76÷2），前提是同样 token 用量、促销价恰为标准价一半。
   - “60% of the cost” 是谷歌博客页内 Glean 引语；NewsBytes、36氪、ETtoday、链捕手、Neowin 等把它写成谷歌或 Neon 的事实。The Decoder、heise 写得准确。
   - 定价页静态抓取为空壳，用 opencli 渲染取文（`b-pricing-rendered.txt`，账号标识已替换）。

## 速览

- [Anthropic IPO 定价](https://www.anthropic.com/news/anthropic-prices-initial-public-offering)（11-18，官方未给时刻；预计 11/19 纳斯达克交易，未核实际开盘）。
- [Genie 4 / Project Genie](https://blog.google/innovation-and-ai/models-and-research/google-deepmind/genie-4-now-available-project-genie/)（北京 11-19 02:00）。
- 【跟进1016】[DeepSeek 定价页](https://api-docs.deepseek.com/quick_start/pricing)与 [10/17 公告](https://api-docs.deepseek.com/news/news261017)：服务端搜索按次收费 11/1 00:00 起、11/18 12:00 止（北京），实收 17 天 12 小时；扫描“18 天”不用。
- [DaisyRose-27B 博文](https://www.radixark.com/blog/daisyrose-27b)（北京 11-18 20:00）与 [HF 模型卡](https://huggingface.co/RadixArk/DaisyRose-27B)。博文“TB 4.1 5.6%→12.0%”里 5.6% 是 Qwen3.8-27B 的 TB 4.0 成绩（HF 卡脚注），不上图。
- [Kokoro 研究者原文](https://medium.com/@alonhertz1/one-click-from-180-million-downloads-taking-over-the-hugging-face-account-behind-an-ai-model-ac61c16305b7)与 [BleepingComputer](https://www.bleepingcomputer.com/news/security/researchers-hijack-hugging-face-account-for-popular-kokoro-ai-model/)（北京 11-18 02:21）。下载量两源不一，不上图。
- [OpenAI Research Acceleration](https://openai.com/index/research-acceleration-jagged-frontier/)（11-18，官方未给时刻）。9 月基线受 8/21–9/10 harness 变更压低（脚注 2），不上图。
- 备用未收：微软 Decision 1.1（“79x cheaper” 无口径、“4.4x lower cost” 无基线，HN 8 分）、NewCore Grok Bot 31 次请求（厂商自报、卖网关）、Stitch Loop（HN 4 分）。
- 未收：亚利桑那州刑事调查（风控）、英伟达换帅与 GPU 租赁、腾讯元宝、KPMG 审计试验、Science“AI 人设集群”、豆包 SeedRealtime。

## 备注

- Reddit 三组均被登录页拦截，无 Reddit 热度；HN 评论得分 Algolia 不公开。
- `c-ms-decision-ga.txt` 原为 2.17 MB（6 段页内图表 iframe 的 base64），已把这 6 行替换为占位说明后入库（约 21 KB）；原件 `c-ms-decision-ga.html` 存网盘。
- `a-kev-json.json`、`a-kev-json-2150.json`（CISA KEV 目录，各 1.86 MB，两次内容相同）不入库、收尾删除，查询结果与 SHA-256 记在 `a-kev-summary.txt`。
- 收尾删除（执行方列入待删除）：`c-news-index.*`（误抓的 quick_start 页）、`c-hn-50554374.json`（B 组条目误放 C 组）；图片 `images/08c-f5-k123633-description.png`（含浏览器翻译插件图标）、`images/12-rapid7-impact.png`（拼接边界切到图注）、`images/12e-metnir-pipeline-found.png`（副题切半行）、`images/23-pricing-carbon-neon.png`（切到半行）；北京 11-20 收尾时一次删除，取证记录里仍提到它们。
