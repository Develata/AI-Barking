# 1026 期 D 组取证：速览（不截图）

取证时刻：2026-10-27 北京 22:53–23:07（抓取与热度）。时间一律北京时间（UTC+8），括号保留原时区；官方页只有日期的，写“官方未给时刻”。级别按 EDITORIAL.md 事实分级（L1–L7）。

存档说明：文本存档在 `sources/d-*.txt`；HTML 与 PDF 原件只存本地（不入库）。抓取与 opencli 取页的逐条记录见 `capture-log-D.md`。

## 1. 逐条取证

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| D1 Anthropic S-1：2026 年上半年营收 16,286、营业亏损 (693)、公允价值变动 (72,388)、其他支出 (8,757)、净亏损 (80,748)（百万美元） | L1 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm （存档 `d-s1-sec.html` / `d-s1-sec.txt`；`d-s1-sec.txt` 行 2620–2830 为汇总表） | 表头 “(in millions, except share and per share amounts)”；“Six Months Ended June 30,” 下 2025 / 2026 两列；行名 “Revenue” 2026 列 “$16,286”；“Operating loss” 2026 列 “(693)”；“Change in fair value of convertible notes, derivative liabilities, and SAFEs” 2026 列 “(72,388)”；“Other income (expense), net” 2026 列 “(8,757)”；“Net loss(2)” 2026 列 “$(80,748)”。MD&A：“The decrease was primarily driven by the initial fair value recognition and change in fair value of the assets and liabilities recognized under the note purchase agreements described in Note 4” | — | 口径为 Six Months Ended June 30, 2026（H1），单位百万美元。净亏损 80,748 中公允价值变动 72,388，约占 90%（推断，由表格算出，非原文表述）。“其他支出 8,757”原文归因于 note purchase agreements 的初始公允价值确认与公允价值变动。代码 “ANT” 与“尚未生效”出自 Anthropic 公告（见下一行） | 已找到 |
| D1 推迟新基座预训练一个多月 | L1 | 同上（`d-s1-sec.txt` 约第 5800–5812 行，风险因素段）；公告 https://www.anthropic.com/news/s1-sec （存档 `d-s1-news.txt`） | “we recently chose to delay pre-training a new frontier base model for more than a month to allow time to further enhance our security measures related to preventing model weight exfiltration and protecting our intellectual property.” 公告：“Anthropic intends to list its Class A common stock on the Nasdaq Global Select Market under the symbol “ANT.”” “A registration statement relating to these securities has been filed with the SEC but has not yet become effective.” | — | S-1 原文为 risk factor 中的 “we recently chose”，未给具体日期。公告页元数据 datePublished 2026-10-26T00:00:00Z，疑为日期占位，官方未给时刻 | 已找到 |
| D1 S-1 递交时刻 | L1 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/0001628280-26-067168-index.htm （opencli 取页，存 `d-sec-index-s1-opencli.json`；curl 403，见 capture-log） | EDGAR 页面：“Filing Date 2026-10-26”，“Accepted 2026-10-26 09:00:11”（Form S-1） | — | EDGAR 时间按美东（SEC 标准，本条为推断）换算：北京 2026-10-26 21:00:11。Anthropic 公告只写 “Oct 26, 2026”，官方未给时刻 | 已找到 |
| D2 noyb 等对 Scale AI 提 5 项投诉（法、意、荷、奥），指 Outlier 平台监控标注员 | L1（noyb 作为投诉方发布自身行动的新闻稿，属该主体官方发布，写“noyb 称”；Scale AI 无回应材料） | https://noyb.eu/en/ai-training-platform-outlier-illegally-surveils-european-platform-workers （存档 `d-noyb.txt`） | “has filed 5 complaints against Scale AI in France, Italy, the Netherlands and Austria.” “which employs more than 900,000 “freelance” workers.” “trackers from companies such as TikTok, LinkedIn, Facebook, the survey platform Sprig and the data analysis company Datadog are placed on their devices.” “Persona” 段：“This includes video recordings of their face and uploading their ID.” | — | noyb 页面日期 “27 October 2026”，官方未给时刻。“5 complaints” 为 noyb 所述投诉数量，不是裁决。Scale AI 的说法仅为 noyb 页面转述：“Scale AI claims that this is done to prevent identity fraud and to “verify the workers’ skills”.”（来源未注明） | 部分支持（事件与 5 件投诉、900,000 已找到；“最多 19 种追踪器”不符，见第 3 节） |
| D2 Reuters 10/27 08:44 UTC 报道 | — | 未核 | — | — | 本轮未检索 Reuters 对该投诉的报道 | 未找到一手来源（未核） |
| D2 “最多 19 种追踪器” | L1（DiPLab/Reversing.Works 报告，2026-10-15） | noyb 页面未给数量；报告 PDF https://diplab.eu/wp-content/uploads/2026/10/Surveillance_at_scale_diplab_october_2026-1.pdf （存本地 `d-diplab-pdf.pdf`，文本 `d-diplab-pdf.txt`）；报告页 https://diplab.eu/report-new-diplab-report-surveillance-at-scale/ （存档 `d-diplab.txt`） | 报告原句：“the detection of 16 trackers on workers' computers:”；“During our analysis, we found 16 unique trackers that collected workers' personal information and sent it to third parties.” | — | 报告发布于 2026-10-15（DiPLab 站点日期）。noyb 页面与报告均无 “19” 的数字 | 与说法不符（DiPLab 报告为 16；仅核了 noyb 页与报告，noyb 的 5 份投诉 PDF 与 Reuters 稿未核，“19”可能出自那里） |
| D2 Persona 相关说法 | L1（noyb 原文） | 同上 | “In October 2025, approximately 70,000 photos of government issued IDs stored with Persona were exposed by hackers.” | — | noyb 投诉方的陈述，未经独立核实 | 已找到（作为 noyb 陈述） |
| D3 Cloudflare：Workers AI 上 GLM-5.3 有 155 条响应含其他请求内容 | L1 | https://blog.cloudflare.com/kv-cache-exposure-remediation/ （存档 `d-cf-kv.txt`）；SGLang https://www.lmsys.org/blog/2026-10-26-cf-security （存档 `d-sglang-cf.txt`） | “Between August 27 and September 9, a small number of customers using the GLM 5.3 Workers AI model received responses that contained data from other customers' requests made to the same model shortly before theirs. In total, 155 responses were affected, all to prompts longer than 681,280 tokens.” | — | 单位为“响应（responses）”，不是客户或会话；原文为“were affected”（已确认），不是“可能”。影响期 2026-08-27 至 09-09。博文元数据 datePublished 2026-10-26T22:01:23.889Z（北京 2026-10-27 06:01:23），修改 2026-10-27T08:31:47Z（北京 16:31）；SGLang 文 “October 26, 2026”，官方未给时刻 | 部分支持（数字与影响期一致；“可能混入”弱于原文） |
| D3 KV 池与上下文 | L1 | 同上 | “We deployed GLM 5.3 with a 1,048,576-token context window, but the KV cache pool on our prefill GPUs only had room for 681,280 tokens.” | — | — | 已找到 |
| D3 修复与版本 | L1 | 同上（CF 与 SGLang 两篇） | “The patch for this issue was released in SGLang v0.5.20, with follow-ups in v0.5.21 and v0.5.22, and is available for all other operators dependent on SGLang to use. We highly recommend all operators update SGLang dependencies to v0.5.22 or later.”；“We have remediated the issue, and we restored the model on September 9.” | — | CF 博文 datePublished 2026-10-26T22:01:23.889Z（北京 2026-10-27 06:01）；SGLang 文 2026-10-26，官方未给时刻；SGLang 文内 #43719 “merged on October 12, 2026” | 已找到 |
| D3 是否逐一通知受影响客户 | L1 | 同上 | 原文未见通知客户的描述（已检索 notif/contact/email 等关键词） | — | — | 未找到 |
| D4 404 Media：ChatGPT 多次建议用户找 404 Media 记者解决账号恢复等技术支持问题 | L4（404 Media 报道，ChatGPT 对话未公开） | https://www.404media.co/chatgpt-keeps-telling-people-404-media-can-solve-their-tech-support-problems/ （存档 `d-404media.txt`；页面后半为付费内容，未绕过） | “The email seemed AI-generated, like many others we have gotten recently.” “And, increasingly, ChatGPT is telling people that have normal tech support problems that they have discovered something journalistically notable and that 404 Media, specifically, can help.” ChatGPT 对话为作者转述，原文：“ChatGPT then gave him a list of several news outlets, including 404 Media”。 | — | 报道发布 2026-10-26T15:49:44Z（北京 2026-10-26 23:49:44）。样本为主动来信的用户，报道未给发生率。OpenAI 是否回应：可见部分未见，付费段未取 | 已找到（据 404 Media 报道，L4；ChatGPT 对话未公开，OpenAI 回应在付费段未取得） |
| D5 Meta 股票融资：总额 975 亿美元（普通股 25 亿 + 可转换优先股 125 亿 + 最多 600 亿美元 ATM） | L1（Meta 定价公告与 SEC 免费写作说明书；NVIDIA 官方公告） | Meta 定价公告 https://investor.atmeta.com/investor-news/press-release-details/2026/Meta-Announces-Pricing-of-Offering-of-Depositary-Shares-Representing-Series-A-Mandatory-Convertible-Preferred-Stock-and-Offering-of-Class-A-Common-Stock-in-Connection-with-Forward-Sale-Agreements/default.aspx （存档 `d-meta-ir-pricing.txt`）；SEC FWP 发行（10/26）https://www.sec.gov/Archives/edgar/data/0001326801/000119312526431036/d116268dfwp.htm （存档 `d-meta-d116268dfwp.txt`）；定价条款 FWP https://www.sec.gov/Archives/edgar/data/0001326801/000119312526432549/d382225dfwp.htm （存档 `d-meta-d382225dfwp.txt`）；NVIDIA 公告 https://nvidianews.nvidia.com/news/nvidia-and-meta-to-expand-collaboration-on-muse-and-muse-family-of-models-amid-robust-growth （存档 `d-nvidia-newsroom.txt`）；Reuters 10/27 https://www.reuters.com/legal/transactional/meta-launches-35-billion-stock-offering-fund-ai-expansion-2026-10-26/ （存 `d-reuters-meta.json`） | 定价公告：“(ii) $25.0 billion of shares of Class A common stock”、“(i) 250,000,000 series A depositary shares … at a public offering price of $50 per Depositary Share”，净收益 “approximately $12.5 billion”。FWP 10/26：“The Company plans to launch an approximately $60 billion at-the-market (“ATM”) program upon the commencement of trading of the Company’s previously announced Class C common stock”；“Such program is not expected to commence until the first quarter of 2027 following the commencement of trading of the Company’s previously announced Class C Common Stock.”；“The Company has not entered into any agreement with respect …”。定价条款 FWP：“Meta Platforms, Inc. has increased the size of the Common Stock Offering to $25.0 billion”。NVIDIA：“In addition, NVIDIA will invest $3 billion in Meta’s equity offering.”。Reuters：“raised the planned size of its largest-ever equity offerings to $97.5 billion” | — | 975 为加总推断（Reuters 报 97.5，Meta 官方文件未写总数）：25.0（普通股，定价时由 22.5 上调，定价条款 FWP）+ 12.5（优先股，250,000,000 × $50 = $12.5B 毛额，定价公告称净额约 $12.5B）+ 60（ATM 上限，FWP 431036）。初始 950 为推断（22.5 + 12.5 + 60，与 FWP 的 “approximately $35 billion” 与 “$60 billion” 相符；WSJ 线索写 “up to $95B”）。NVIDIA 的 30 亿为普通股发行的一部分（原文 “in Meta’s equity offering”，未称另加）。ATM 尚未签约（“has not entered into any agreement”），时间与规模不保证 | 已找到 |
| D5 Meta 发布与定价时刻 | L1 | 同上 | 定价公告 “MENLO PARK, Calif., Oct. 26, 2026 /PRNewswire/”；IR 页面日期显示 “October 27, 2026”；FWP 10/26 “Issuer Free Writing Prospectus dated October 26, 2026”；NVIDIA 页 “October 26, 2026”；Reuters 日期行 “Oct 27 (Reuters)”，报道称 Meta “on Tuesday raised”，NVIDIA “said on Tuesday” | — | 官方文件署 10/26（Menlo Park 当地日期，未给时刻）；Reuters 10/27 两篇写 “on Tuesday”。日期不一致，原样记录，不推断原因。IR 页面 “October 27, 2026” 为站点显示日期 | 已找到 |
| D6 维基百科编辑清理 AI 生成内容（WikiProject AI Cleanup） | L1（WikiProject 页）；IEEE Spectrum 为 10/21 专访，L2–L3 | https://spectrum.ieee.org/wikipedia-wikiproject-ai-writing-cleanup （存档 `d-ieee-wiki.txt`）；https://en.wikipedia.org/wiki/Wikipedia:WikiProject_AI_Cleanup （存档 `d-wp-ai-cleanup.txt`） | IEEE 专访标题 “How Wikipedia’s Editors Keep the Online Encyclopedia Human-Written”；“a volunteer effort launched in 2023 to find and remove AI-generated material from Wikipedia”；“English Wikipedia recently prohibited editors from using LLMs to generate articles.”；Lebleu：“In late 2025, it flagged perhaps 20 percent of new articles and drafts as AI-generated. In early 2026, that went down to 15 or 16 percent. But we don’t yet know whether that’s because our recent policies have had an effect or because models are getting more powerful and Pangram hasn’t had time to adapt.” | — | IEEE 发布 2026-10-21T13:00:02Z（北京 2026-10-21 21:00:02），修改 2026-10-26T13:05:56Z（北京 10-26 21:05:56）。它是“5 Questions”访谈，不是新闻事件。Pangram 检测为作者与 Pangram 合作估计，作者称为“very conservative lower bound” | 已找到（项目存在与政策原文已核；日期与性质见勘误） |

## 2. 热度（HN，barking heat，取得 2026-10-27 23:04 北京）

| 条目 | HN 帖 | 分 / 评 | 北京时刻（HN 提交） |
|---|---|---|---|
| D1 | “Anthropic S-1”（sec.gov 链接，id 50242939） | 1667 / 1606 | 10-26 21:04 |
| D1 | “Anthropic files S-1 with the SEC …”（anthropic.com 链接，id 50242957） | 25 / 1 | 10-26 21:06 |
| D2 | 无命中（查询 “Scale AI noyb”） | — | — |
| D3 | “Cross-request KV cache exposure in the GLM-5.3 Workers AI model (155 responses)”（id 50252159） | 162 / 61 | 10-27 06:41 |
| D4 | “ChatGPT Keeps Telling People 404 Media Can Solve Their Tech Support Problems”（id 50247630） | 253 / 94 | 10-27 01:32 |
| D5 | 无 Meta 融资主帖命中（查询 “Meta stock offering”，仅 WSJ “Meta Plans to Raise Up to $95B for AI Push” 4 / 0，10-26 20:01）；“Nvidia Meta Muse”无命中 | 4 / 0（WSJ 线索） | 10-26 20:01 |
| D6 | “Wikipedia’s Cleanup of AI-Generated Content”（id 50257326，IEEE 链接） | 175 / 82 | 10-27 19:08 |
| D6 | 同链接的早期帖（id 50182203） | 5 / 1 | 10-21 23:00 |

HN 评论里未摘取有理有据的高赞质疑（本轮只做热度，未做评论抽样；如需进入正文，另行补采）。

## 3. 扫描说法勘误

1. D2“最多 19 种追踪器”：noyb 页面未给数量；DiPLab 报告（2026-10-15）原文为 “16 unique trackers” 与 “the detection of 16 trackers”。与说法不符，以报告为准。
2. D2“Reuters 10/27 08:44 UTC”：本轮未检索 Reuters 对 noyb 投诉的报道，未核。
3. D3“155 条响应可能混入”：CF 原文为已确认的 “155 responses were affected”，“可能”弱于原文。数字与影响期一致；单位是响应，不是客户。
4. D6 发布日期“10/27”：IEEE 页面标注发布 2026-10-21（北京 10-21 21:00），修改于 10-26。10/27 是 HN 转载日。另，该文为专访（5 Questions），不是新闻事件。
5. D5“950 → 975 亿”：初始 950 为 22.5 + 12.5 + 60（FWP 10/26 写 “approximately $35 billion” 与 “approximately $60 billion”，Reuters/WSJ 写 “$95 billion”）；975 = 25 + 12.5 + 60（定价条款 FWP 与 Reuters 10/27）。与扫描一致。
6. D1 “净亏损多为账面”：表格上公允价值变动 (72,388) 占净亏损 (80,748) 约 90%，属推断。其余数字与 S-1 原文一致。
7. D4：扫描未写错，但“多次建议”的一手材料（ChatGPT 对话）未公开，按 L4 处理。

## 4. 风控与其他提示

- D2：noyb 原文列出客户时写 “Among its customers: Meta, OpenAI, Google – and the US government.” 若收入速览，注意“涉外国政府”的风控类别（EDITORIAL 风控一节）。
- D5、D3、D1、D4、D6 未见风控题材（D3 的 GLM 为中国公司 Z.ai 的模型，但披露方是 Cloudflare，非美方机构点名指控，按规则不列为风控）。

## 5. 未完成 / 失败

- D2 Reuters 报道、Scale AI 回应：未取得。
- D4 404 Media 付费段与 OpenAI 回应：未取得（付费墙，不绕过）。
- D1 SEC 原页 curl 403，已改 opencli 取得；EDGAR 索引页同样改 opencli。
- D5 Reuters 页 curl 401，已改 opencli；Meta IR 页 curl 403（Cloudflare 质询），已改 opencli 取得；IR 页面 extract 为空，改用 get html 后转文本。
- D3 CF 与 SGLang 的修复日期只给版本号，未给发布日期（官方未给）。
- D6 “English Wikipedia recently prohibited …” 的政策页（Wikipedia:Writing_articles_with_large_language_models）未单独存档，仅记 IEEE 的转述。
