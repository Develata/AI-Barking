# 1126 期 C 组（速览）取证结果

执行：Antigravity 取证子代理（C 组）。北京时间 11-27 21:12 起取证，21:24 完成。
范围：派工单 C1–C8（含 C8 自选补齐 5 条：C8a–C8e）。无截图（C 组不需要）。存档、摘句、热度均在本目录 `c*` 文件中；抓取过程见 `capture-log-c.md`。

约定：时间一律北京时间（UTC+8），括号保留原时区；官方只给日期的写“官方未给时刻”。状态只用：`已找到` / `部分支持` / `与说法不符` / `未找到一手来源`。热度为 `barking heat` 取得的 HN 快照（取得时刻 11-27 21:13–21:20），会随时间变化。

本组交付 **12 条候选**（派工要求至少 7–8 条干净可用）：
- **干净可用、无风控且非旧闻（9 条）**：C1（L4）、C2（L1）、C4（L1/L2）、C5（L4）、C7（L1）、C8a（L1）、C8b（L1）、C8c（L4）、C8d（L1，11-21 至 11-24 发布，略早于 36h 窗）。
- **需主执行方 / Develata 裁决（3 条）**：C3（L1，涉韩国政府监管机构 KFTC，标风控）、C6（L1，模型 9 月已开源，11-25 仅交 arXiv 论文，标旧闻勘误）、C8e（L4，含英国 FCA 沙盒，标风控备选）。

---

## 汇总表（按 `templates/codex-evidence.md` 第 4 条）

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 DeepSeek 网页/App 灰度“本轮消耗额度”卡片，仍免费 | L4（报道）/ L1（定价页） | `https://original.ifeng.com/c/8yOj7tdntji`；`https://www.ithome.com/1/033/157.htm`；`https://api-docs.deepseek.com/zh-cn/quick_start/pricing/` | “个人日常聊天服务目前仍然免费……卡片显著位置仍标注‘免费’字样”；“(2) 空闲时段价格为高峰时段价格的一半。北京时间周一至周五（不含中国法定节假日）9:00 - 12:00、14:00 - 18:00 为高峰时段” | 无（C 组免截） | 官方无收费公告（更新日志最新 11-13）；IT之家图按空闲价（0.02/1/4元），凤凰实测按高峰价（2/8元）；北京 11-27 | 已找到 |
| C2 Neuralink 获 FDA 批准开展 ALS 关键性临床试验 | L1 | `https://neuralink.com/updates/neuralink-receives-fda-approval-to-launch-link-pivotal-study/`；`https://neuralink.com/trials/` | “Neuralink has received approval from the U.S. Food and Drug Administration (FDA) to launch the LINK Pivotal Study, the first pivotal trial for an implantable brain-computer interface (BCI) in the United States.”；“Neuralink devices are investigational and have not been approved by the FDA or other regulatory authorities.” | 无（C 组免截） | 针对 ALS 重度瘫痪，已开放招募；“美国首个”为官方自述；器械未获批上市；页标 2026-11-25，无时刻 | 已找到 |
| C3 韩国 KFTC 公布对苹果 Apple Intelligence 广告案审查报告（风控） | L1 | `https://www.korea.kr/briefing/pressReleaseView.do?newsId=156790945`；`https://www.korea.kr/common/download.do?fileId=198597020&tblKey=GMN` | “심사보고서는 심사관이 조사 과정에서 파악한 위법성 및 그에 대한 조치 의견을 기재한 것으로서 위원회 최종 판단을 구속하지 않음.”；“피심인들은 심사보고서 수령일로부터 6주 내 서면 의견 제출” | 无（C 组免截） | 官方新闻稿未列罚金数字（仅算相关销售额约 3.3527 万亿韩元）；536 亿等系媒体推算；KST 11-26 12:00 解禁（北京 11:30） | 已找到 |
| C4 Google 支持团队在社区回应已修复 Gemini 网页端高内存占用 | L1/L2（官方支持账号）；L4（AA） | `https://www.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage/`；`https://www.androidauthority.com/google-gemini-chrome-ram-hogging-issue-fixed-3739606/` | “Hi there. Thanks for letting us know. This has now been resolved, so please let us know if you are still experiencing any issues with high memory usage in Chrome.”；“The company didn’t explain what was wrong or what it fixed.” | 无（C 组免截） | 主帖 11-14 11:00Z；GoogleHelpCommunity 回复 11-24 17:15Z（北京 11-25 01:15）；未给原因；6.4GB/8.4GB 出自 AA 引述他例 | 已找到 |
| C5 SAP 考虑 AI 转型裁员至少 1 万人 | L4（外媒）；L1（含回应） | `https://www.channelnewsasia.com/business/sap-job-cuts-ai-restructuring-6585781`；`https://news.sbs.co.kr/english/article.do?news_id=N1008866096` | “German software giant SAP is considering cutting at least 10,000 jobs in an AI-driven restructuring, Bloomberg News reported on Thursday (Nov 26), citing people familiar with the matter.”；“"Beyond this, no decisions have been made regarding any further measures. We do not comment on speculation," the company added.” | 无（C 组免截） | 影响全球员工至少 9%（FT 写约 11.3 万人）；未与劳方协商、未定案；Reuters 原链接 401 被拦；CNA 北京 11-27 00:33 | 已找到 |
| C6 Yandex 开源 800 亿参数基础模型 AliceAI-Foundation-80B-A3B-Base | L1 | `https://arxiv.org/abs/2611.34529`；`https://huggingface.co/yandex/AliceAI-Foundation-80B-A3B-Base`；`https://habr.com/ru/companies/yandex/articles/1083300/` | “We present AliceAI-Foundation-80B-A3B-Base, a mixture-of-experts language model with 80 billion total parameters and approximately 3 billion active parameters per token, trained from scratch at Yandex and released under the Apache 2.0 license.” | 无（C 组免截） | 模型 9 月 21 日已在 HF/Habr 开源；11 月 25 日 15:14Z（北京 23:14）仅提交 arXiv 论文；属 Pretrained 基础模型 | 部分支持 |
| C7 Google Cogentic 论文称提出无条件亚指数时间整数分解算法 | L1 | `https://github.com/google-research/google-research/tree/4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/integer_factorization` | “This paper and its Lean 4 formalization were entirely obtained by Cogentic (arXiv preprint arXiv:2609.40324), an agentic framework for mathematical discovery, using Gemini 4 as the base model.”；“This is not an official Google product.” | 无（C 组免截） | 期望时间 L_N[1/3,105]、空间 L_N[1/3,40]；原文明言未优化常数；Lean 4 形式化将另行发布；commit 北京 11-25 18:21 | 已找到 |
| C8a 月之暗面开放 Kimi for Science 科研支持计划 | L1 | `https://www.kimi.com/news/kimi-for-science-program`；`https://www.kimi.com/news/` | “入选申请者可获得最高 15 万元 Kimi API 模型调用额度（约合 70 亿 tokens），具体额度根据研究需求与评审结果确定，并非现金资助。”；“首期截止时间 12月8日，更多批次将陆续开放” | 无（C 组免截） | 面向自然科学、工程学及交叉学科；非现金资助；列表页标 2026-11-26，无时刻 | 已找到 |
| C8b Kimi 发布游戏开发与创意工作解决方案 | L1 | `https://www.kimi.com/news/kimi-game-solutions`；`https://www.kimi.com/news/` | “基于 Kimi K3.1 系列模型、Kimi Code 与 Kimi Work 系列 Agent 产品，以及 Unity、Unreal Engine、Blender 等专业工具插件” | 无（C 组免截） | 含 20 小时无人干预复现等官方自述演示；列表页标 2026-11-26，无时刻 | 已找到 |
| C8c 腾讯混元发布专业级 3D 模型 Hy3D 3.6 | L4 | `https://tech.ifeng.com/c/8yOKHwwsCrm`；`https://cloud.tencent.com/product/ai3d` | “腾讯混元今天（27 日）正式发布最新一代 3D 大模型 Hy3D 3.6。新版本面向专业 3D 创作，重点优化了高模和 PBR 材质贴图的生成能力。” | 无（C 组免截） | 凤凰网转 IT之家北京 11-27 11:46；门户 3d.hunyuan.tencent.com 需登录，腾讯云旧文档未见 3.6 | 部分支持 |
| C8d 小红书开源 3D 代码数据框架 3DCodeFactory | L1 | `https://huggingface.co/datasets/3DCodeFactory/3DCodeFactory`；`https://github.com/3DCodeFactory/3DCodeFactory`；`https://arxiv.org/abs/2611.28783` | “The paper reports results for a corpus of 5,045 verified trajectories (5.52B tokens).”；“Xiaohongshu Inc. · Dots Studio” | 无（C 组免截） | Apache 2.0；HF 数据集发布 6,414 条（303 GB）；arXiv 11-21、GH 11-23、HF 11-24（略早于 36h 窗） | 已找到 |
| C8e 英国完成首例 AI 智能体无卡银行支付买披萨（风控备选） | L4 | `https://www.theguardian.com/business/2026/nov/27/ai-agent-cardless-transaction-pizza-papa-john-uk-fca-truelayer` | “The order of nine mini pizzas was made as part of the Financial Conduct Authority’s (FCA) regulated sandbox trials to test the technology, using Lloyds Bank and payment specialist TrueLayer.” | 无（C 组免截） | 属英国 FCA 监管沙盒测试；涉英国金融监管机构标风控；北京 11-27 17:53 | 已找到 |

---

## 逐条详情

### C1 DeepSeek 网页/App 灰度“本轮消耗额度”卡片，仍免费
- 建议的一句事实（40 字）：`据凤凰网科技实测，DeepSeek 灰度显示本轮估算价，聊天仍免费且无收费公告。`
- 关键限定词原句：
  - 凤凰网科技（`c-c1-ifeng-orig.txt`）：“个人日常聊天服务目前仍然免费……虽然已经提前打表了，但DeepSeek暂时还不收费”、“卡片显著位置仍标注‘免费’字样”、“截至发稿，DeepSeek官方网站、开发者文档及公开社交媒体账号，均未发现任何关于面向个人用户聊天服务启动收费的公告或充值入口。”、“在工作日的高峰时段，输入缓存未命中部分为2元/百万Token，输出部分为8元/百万Token”。
  - IT之家（`c-c1-ithome.txt`）：“昨日起，有部分 DeepSeek 用户反馈，网页端和移动端 App 内正在灰度测试‘显示本轮消耗额度’功能……根据用户提供的截图，额度按‘空闲时段’价格估算。……输入缓存命中部分，单价为 0.02 元 / 百万 tokens……输入缓存未命中部分，单价为 1 元 / 百万 tokens……输出部分单价为 4 元 / 百万 tokens”。
  - 官方 API 定价页（`c-c1-deepseek-pricing.txt`）：“(2) 空闲时段价格为高峰时段价格的一半。北京时间周一至周五（不含中国法定节假日）9:00 - 12:00、14:00 - 18:00 为高峰时段；其余时段，包括周末及中国法定节假日全天均为空闲时段。”（`DeepSeek-V4.1-Flash-1017`）。
- 来源 URL 与存档：
  - 凤凰网科技：`https://original.ifeng.com/c/8yOj7tdntji`（`c-c1-ifeng-orig.html` / `.txt`）
  - IT之家：`https://www.ithome.com/1/033/157.htm`（`c-c1-ithome.html` / `.txt`）及凤凰大风号转 IT之家 `https://tech.ifeng.com/c/8yO4BybuYcl`（`c-c1-ifeng-tech.html` / `.txt`）
  - DeepSeek 官方定价页：`https://api-docs.deepseek.com/zh-cn/quick_start/pricing/`（`c-c1-deepseek-pricing.html` / `.txt`；英文版 `c-c1-deepseek-pricing-en.html` / `.txt`）
  - DeepSeek 更新日志与新闻页：`https://api-docs.deepseek.com/zh-cn/updates/`（`c-c1-deepseek-updates.html` / `.txt`，最新条目 `2026-11-13`）；`https://www.deepseek.com/news/`（`c-c1-deepseek-news.html` / `.txt`，最新条目 `2026 年 9 月 10 日`）；`https://www.deepseek.com/`（`c-c1-deepseek-www.html` / `.txt`）。
- 发布时刻：IT之家 `2026/11/27 8:21:03`（北京时间）；凤凰网科技 `2026年11月27日 18:09:35`（北京时间，Updated `18:11:29`）。官方无公告。
- 级别：L4（实测报道）/ L1（官方 API 定价口径与无收费公告核对）。
- 热度（`c-heat.json`）：HN 关键词 `DeepSeek` 命中 13 条，均与本次灰度卡片无关。
- 状态：已找到。

### C2 Neuralink 获 FDA 批准开展 ALS 关键性临床试验
- 建议的一句事实（39 字）：`Neuralink 称获 FDA 批 ALS 关键试验并招募，器械未获批上市。`
- 关键限定词原句：
  - 官方新闻稿（`c-c2-neuralink-update.txt`）：“Neuralink has received approval from the U.S. Food and Drug Administration (FDA) to launch the LINK Pivotal Study, the first pivotal trial for an implantable brain-computer interface (BCI) in the United States. The goal of the study is to evaluate the safety and effectiveness of the Neuralink BCI system in individuals with severe paralysis due to amyotrophic lateral sclerosis (ALS).”
  - 招募与上市进度（`c-c2-neuralink-update.txt`）：“Enrollment for the LINK Pivotal Study is now open and we look forward to continuing to expand access to the study over time.”；“The approval of the LINK Pivotal Study represents an important step toward bringing implantable BCI technology to market...”
  - 器械未获批上市（`c-c2-neuralink-trials.txt`）：“Neuralink devices are investigational and have not been approved by the FDA or other regulatory authorities.”
- 一手来源 URL 与存档：
  - `https://neuralink.com/updates/neuralink-receives-fda-approval-to-launch-link-pivotal-study/`（`c-c2-neuralink-update.html` / `.txt`）
  - `https://neuralink.com/trials/`（`c-c2-neuralink-trials.html` / `.txt`）
  - `https://neuralink.com/trials/device-control/`（`c-c2-neuralink-device-control.html` / `.txt`）
- 发布时刻：官方页标 `November 25, 2026`（官方未给时刻；按美西/美东仍为 11-25 或北京 11-26 凌晨）；HN 50654405 提交于北京 `11-26 20:06`。
- 级别：L1（Neuralink 官方公告）。
- 热度（`c-heat.json` / `c3-heat.json`）：HN `50654405`（3 分 / 0 评，取得时刻 11-27 21:13 / 21:20）；Reddit `Neuralink LINK` 登录墙失败（`c3-heat.json`）。
- 状态：已找到。

### C3 韩国 KFTC 公布对苹果 Apple Intelligence 广告案审查报告【风控提示】
- 建议的一句事实（38 字）：`韩国公正委发苹果 AI 广告案审查报告，未列罚金，苹果有6周申辩，尚未裁决。`
- 关键限定词原句：
  - KFTC 官方新闻稿 PDF（`c-c3-korea-press-pdf.txt`）：“공정거래위원회(위원장 주병기, 이하 ‘공정위’) 사무처는 애플 인코퍼레이티드(Apple Inc.) 및 애플코리아(유)(이하 ‘피심인들’)의 ｢표시·광고의 공정화에 관한 법률｣(이하 ‘표시광고법’) 위반 혐의에 대해 행위 사실, 위법성 및 조치 의견 등을 기재한 심사보고서를 위원회에 제출 (’26.11.24.)하였다. 이에 따라 위 사건에 대한 심의 절차가 개시되었고, 같은 날 피심인들에게도 심사보고서를 송부하였다.”
  - 非最终裁决：“※ 심사보고서는 심사관이 조사 과정에서 파악한 위법성 및 그에 대한 조치 의견을 기재한 것으로서 위원회 최종 판단을 구속하지 않음. 향후 독립된 위원회 심의를 거쳐 사건에 대한 최종 판단이 이루어질 예정임에 유의하여 주시기 바람”
  - 未列罚金数字，只写处罚意见与销售额：“시정명령(향후 행위금지명령, 공표명령) 및 과징금 부과 의견을 제시하였다. 또한, 심사관은 이 사건 표시광고법 위반 혐의와 관련된 매출액을 약 3조 3,527억 원으로 산정하였다.”
  - 6 周申辩：“피심인들은 심사보고서 수령일로부터 6주 내 서면 의견 제출, 증거자료의 열람·복사 신청 등 방어권을 충분히 보장받을 수 있다.”
  - 苹果回应（MacRumors `c-c3-macrumors.txt`）：“We have cooperated with the KFTC's investigation and disagree with the conclusions in the Examiner's Report. We look forward to presenting a fulsome defense before the Commission.”
- 一手来源 URL 与存档：
  - 官方新闻稿页：`https://www.korea.kr/briefing/pressReleaseView.do?newsId=156790945`（`c-c3-korea-press.html` / `.txt`）
  - 官方新闻稿 PDF：`https://www.korea.kr/common/download.do?fileId=198597020&tblKey=GMN`（`c-c3-korea-press-pdf.pdf` / `.txt`）
  - 媒体对比：`https://www.macrumors.com/2026/11/26/apple-korea-regulator-iphone-16-ai-advertising/`（`c-c3-macrumors.html` / `.txt`）；`https://www.businesskorea.co.kr/news/articleView.html?idxno=282367`（`c-c3-businesskorea.html` / `.txt`）。
- 发布时刻：KFTC 官方 PDF 标注 `보도시점 2026. 11. 26.(목) 12:00 / 배포 2026. 11. 26.(목) 08:30 < 11. 27.(금) 조간 >`（KST 11-26 12:00 解禁 = **北京时间 2026-11-26 11:30**；08:30 KST 分发 = 北京 07:30）。MacRumors 为 `2026-11-26 3:19 am PST`（北京 11-26 19:19）；BusinessKorea 为 `2026.11.27 11:41` KST（北京 10:41）。
- 级别：L1（韩国政策简报网 KFTC 原始新闻稿及 PDF）。
- 热度（`c-heat.json`）：HN 关键词 `KFTC Apple` 0 命中。
- **风控提示**：本条涉及外国政府监管机构（韩国公平交易委员会）执法程序及美韩数字贸易摩擦，按 `EDITORIAL.md` 速览风控规则标“风控”，由主执行方 / Develata 决定是否收录。
- 状态：已找到。

### C4 Google 支持团队在社区回应已修复 Gemini 网页端高内存占用
- 建议的一句事实（40 字）：`Google 支持称已修 Gemini 网页 Chrome 高内存，未说明原因。`
- 关键限定词原句：
  - Reddit 官方支持回复（`c-c4-reddit-opencli.txt`、`c-c4-reddit-meta.json`）：`GoogleHelpCommunity`（带 `Official Google Support` 徽章）：“Hi there. Thanks for letting us know. This has now been resolved, so please let us know if you are still experiencing any issues with high memory usage in Chrome.”（评论者 `Demon_Young`：“5.4 gigs for a review page is wild…”）。
  - Android Authority（`c-c4-androidauthority.txt`）：“The company didn’t explain what was wrong or what it fixed.”；“Another reported 6.4GB of RAM being used by the Gemini web app, with usage peaking at 8.4GB.”；“However, we haven’t been able to independently verify the fix.”
- 一手来源 URL 与存档：
  - Reddit 原帖：`https://www.reddit.com/r/GeminiAI/comments/1xvzqlw/memory_usage/`（`c-c4-reddit-opencli.txt`、`c-c4-reddit-meta.json`，opencli 取得；直接抓取 403/登录墙）
  - Android Authority：`https://www.androidauthority.com/google-gemini-chrome-ram-hogging-issue-fixed-3739606/`（`c-c4-androidauthority.html` / `.txt`）
- 发布时刻：Reddit 主帖 `2026-11-14T11:00:59.437Z`（北京 11-14 19:00）；`GoogleHelpCommunity` 回复 **`2026-11-24T17:15:11.343Z`（北京 2026-11-25 01:15）**；Android Authority 报道 `Nov 26, 2026 — 2:41 AM ET`（**北京 2026-11-26 15:41**）。
- 级别：L1/L2（`Official Google Support` 账号在社区公开发言），L4（Android Authority）。
- 热度（`c-heat.json`）：HN 关键词 `Gemini memory` 无相关命中；Reddit 该官方支持回复得分 `2`。
- 状态：已找到。

### C5 SAP 考虑 AI 转型裁员至少 1 万人
- 建议的一句事实（37 字）：`据亚洲新闻台援引彭博报道，SAP 考虑 AI 转型裁员至少1万人，未定案。`
- 关键限定词原句：
  - Channel NewsAsia（`c-c5-cna.txt`）：“German software giant SAP is considering cutting at least 10,000 jobs in an AI-driven restructuring, Bloomberg News reported on Thursday (Nov 26), citing people familiar with the matter.”；“The restructuring would affect at least 9 per cent of SAP's global workforce, Bloomberg said, adding that the total number of job cuts could change after labour negotiations.”；“The plans have yet to be discussed with employee representatives, the Financial Times reported, citing people familiar with the matter.”
  - SAP 回应（`c-c5-cna.txt`）：“"We are continuously reviewing how our organisation and workforce need to evolve to remain successful. As part of this ongoing transformation, roles and functions will naturally change over time," SAP said. "Beyond this, no decisions have been made regarding any further measures. We do not comment on speculation," the company added.”
- 来源 URL 与存档：
  - Channel NewsAsia：`https://www.channelnewsasia.com/business/sap-job-cuts-ai-restructuring-6585781`（`c-c5-cna.html` / `.txt`）
  - SBS News（英文）：`https://news.sbs.co.kr/english/article.do?news_id=N1008866096`（`c-c5-sbs.html` / `.txt`）
  - 注：Reuters 链接 `https://www.reuters.com/technology/sap-considers-least-10000-job-cuts-ai-restructuring-bloomberg-reports-2026-11-26/` 返回 HTTP 401（已记失败）。
- 发布时刻：CNA `27 Nov 2026 12:33AM`（新加坡时间 SGT，与北京同区：**2026-11-27 00:33**；Updated `01:34PM`）；SBS `2026.11.27 08:17`（KST，北京 07:17）。（彭博社原文日期为 11-26）。
- 级别：L4（CNA / SBS 转述彭博与 FT，含 SAP 官方回应引语）。
- 热度（`c-heat.json` / `c3-heat.json`）：HN `50658287`（7 分 / 3 评，北京 11-27 02:46 提交，21:13/21:20 采）；Reddit 登录墙失败。
- 状态：已找到。

### C6 Yandex 为 9 月开源的 800 亿参数基础模型 AliceAI-Foundation-80B-A3B-Base 提交 arXiv 论文【旧闻勘误】
- 建议的一句事实（34 字）：`Yandex 为9月开源 80B 基础模型 AliceAI 发论文。`
- 关键限定词原句：
  - arXiv 摘要（`c-c6-yandex-arxiv.txt`）：“We present AliceAI-Foundation-80B-A3B-Base, a mixture-of-experts language model with 80 billion total parameters and approximately 3 billion active parameters per token, trained from scratch at Yandex and released under the Apache 2.0 license.”；“Training comprises general pretraining on 17.5 trillion tokens, two context-extension stages, and a dedicated stage for reasoning and agentic interaction.”
  - 模型卡（`c-c6-yandex-hf-en.txt`）：“supports a context length of up to 262,144 tokens.”；“pretrained model and is provided as-is, without additional post-training / alignment.”（注：仅 Base 版）。
- 一手来源 URL 与存档：
  - arXiv：`https://arxiv.org/abs/2611.34529`（`c-c6-yandex-arxiv.html` / `.txt`）
  - Hugging Face：`https://huggingface.co/yandex/AliceAI-Foundation-80B-A3B-Base`（`c-c6-yandex-hf.html` / `.txt`、`c-c6-yandex-hf-en.txt`、`c-c6-yandex-hf-api.json`）
  - Habr 官方博文：`https://habr.com/ru/companies/yandex/articles/1083300/`（`c-c6-yandex-habr.html` / `.txt`、`c-c6-yandex-habr-comments.html` / `.txt`）
- 发布时刻：
  - **模型开源时刻**：HF API `createdAt` `2026-09-12T10:32:07.000Z`（北京 09-12 18:32）、`lastModified` `2026-11-03T08:24:35.000Z`（北京 11-03 16:24）；Habr 博文 `datePublished` `2026-09-21T09:45:43+03:00`（北京 09-21 14:45），且评论区确实始于 9 月 21 日；Habr `dateModified` 为 `2026-11-25T12:22:19+03:00`（北京 11-25 17:22）。
  - **arXiv 提交时刻**：`Wed, 25 Nov 2026 15:14:56 UTC`（**北京 2026-11-25 23:14**）。
- 级别：L1（Yandex 官方团队论文与仓库）。
- 热度（`c-heat.json` / `c2-heat.json`）：HN 关键词 `AliceAI-Foundation` 0 命中；`AliceAI` 无相关命中；Reddit 登录墙失败。
- 状态：部分支持（参数、Apache 2.0、17.5T、256K、Base 版均属实；但“近期新开源”不成立，实为 9 月开源模型在 11-25 补交 arXiv 论文）。

### C7 Google Cogentic 论文称提出无条件亚指数时间整数分解算法
- 建议的一句事实（37 字）：`Google 发 Gemini 4 生成的整数分解论文，仅理论界、非官方。`
- 关键限定词原句：
  - README（`c-c7-github-readme-ok.txt`）：“This paper and its Lean 4 formalization were entirely obtained by Cogentic (arXiv preprint arXiv:2609.40324), an agentic framework for mathematical discovery, using Gemini 4 as the base model. There was no human interaction other than the initial prompt for Cogentic to generate a full proof.”
  - Lean 4 与发布状态：“A complete, unconditional, Mathlib-only Lean 4 (`v4.35.0`) formalization of the entire proof has also been completed and will be released in a companion submission.”
  - 免责与产品定位：“This is not an official Google product.”
  - 论文正文与常量说明（`c-c7-paper-pdf.txt`）：“computes the complete prime factorization of every integer N ≥ 2 in worst-case expected time L_N [1/3, 105] and worst-case deterministic workspace L_N [1/3, 40]”；“We have chosen transparent explicit constants (105 and 40) rather than optimizing them at the expense of clarity.”
- 一手来源 URL 与存档：
  - GitHub 目录：`https://github.com/google-research/google-research/tree/4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/integer_factorization`（`c-c7-github-tree-ok.html` / `.txt`）
  - README：`https://raw.githubusercontent.com/google-research/google-research/4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/integer_factorization/README.md`（`c-c7-github-readme-ok.txt`）与上级 README（`c-c7-github-parent-readme-ok.txt`）
  - 论文 PDF：`https://raw.githubusercontent.com/google-research/google-research/4a7bc8ab0bc770e71a259b7966c500496b99010a/cogentic_research_papers/integer_factorization/paper.pdf`（`c-c7-paper-pdf.pdf` / `.txt`）
  - GitHub Commit API：`https://api.github.com/repos/google-research/google-research/commits/4a7bc8ab0bc770e71a259b7966c500496b99010a`（`c-c7-github-commit.json`）
- 发布时刻：GitHub commit `committer.date` `2026-11-25T10:21:27Z`（**北京 2026-11-25 18:21**；`author.date` `10:11:06Z` = 北京 18:11）。
- 级别：L1（google-research 官方仓库，内标非官方产品）。
- 热度（`c-heat.json` / `c2-heat.json` / `c-hn-*.json`）：
  - HN `50642086`（整批 6 篇目录贴）：511 分 / 372 评（北京 11-25 21:33 提交，21:13/21:17 采）
  - HN `50651627`（本篇单帖）：152 分（21:13 采）→ 153 分（21:17 采）/ 27 评（北京 11-26 12:59 提交）。
  - Reddit 登录墙失败。
- 状态：已找到。

### C8 自选核查条目（5 条，供主执行方挑选）

#### C8a 月之暗面开放 Kimi for Science 科研支持计划【推荐】
- 建议的一句事实（37 字）：`Kimi 开放科研支持计划，最高供15万元 API 额度，12月8日截止。`
- 关键限定词原句（`c-c8-kimi-science.txt`）：“Kimi for Science 首期将面向自然科学、工程学及相关交叉学科的科研人员开放申请”、“入选申请者可获得最高 15 万元 Kimi API 模型调用额度（约合 70 亿 tokens），具体额度根据研究需求与评审结果确定，并非现金资助。”、“首期截止时间 12月8日，更多批次将陆续开放”。
- 一手来源 URL 与存档：`https://www.kimi.com/news/kimi-for-science-program`（`c-c8-kimi-science.html` / `.txt`）、列表页 `https://www.kimi.com/news/`（`c-c8-kimi-news.html` / `.txt`）。
- 发布时刻：列表页标 `2026-11-26`（官方未给时刻；媒体多于 11-27 报道）。
- 级别：L1（月之暗面官网）。
- 热度（`c2-heat.json` / `c3-heat.json`）：HN `50665920`（10 分 / 1 评，北京 11-27 20:03 提交，21:17/21:20 采）。
- 状态：已找到。

#### C8b Kimi 发布游戏开发与创意工作解决方案【推荐】
- 建议的一句事实（38 字）：`Kimi 发游戏方案，借插件连 Unity、虚幻、Blender 协助开发。`
- 关键限定词原句（`c-c8-kimi-game.txt`）：“基于 Kimi K3.1 系列模型、Kimi Code 与 Kimi Work 系列 Agent 产品，以及 Unity、Unreal Engine、Blender 等专业工具插件”、“在获得本地项目访问与命令执行权限后”、“在一次演示中，Kimi 在没有人工干预的情况下连续工作 20 小时”。
- 一手来源 URL 与存档：`https://www.kimi.com/news/kimi-game-solutions`（`c-c8-kimi-game.html` / `.txt`）、`https://www.kimi.com/news/`（`c-c8-kimi-news.txt`）。
- 发布时刻：列表页标 `2026-11-26`（官方未给时刻）。
- 级别：L1（月之暗面官网）。
- 热度（`c2-heat.json`）：HN `Kimi game` 0 命中。
- 状态：已找到。

#### C8c 腾讯混元发布专业级 3D 生成模型 Hy3D 3.6
- 建议的一句事实（39 字）：`据 IT之家报道，腾讯混元发布专业级 3D 模型 Hy3D 3.6，上线腾讯云。`
- 关键限定词原句（`c-c8-hy3d-ifeng.txt`）：“腾讯混元今天（27 日）正式发布最新一代 3D 大模型 Hy3D 3.6。新版本面向专业 3D 创作，重点优化了高模和 PBR 材质贴图的生成能力。”、“目前，Hy3D 3.6 已经上线腾讯云 Token Hub，对外提供 API 服务。腾讯 AI 创意智能体 Miora、腾讯游戏旗下专业资产生成平台 LAP、专业影视 AI 平台 WorkRally 等也已接入”。
- 来源 URL 与存档：凤凰网科技转 IT之家 `https://tech.ifeng.com/c/8yOKHwwsCrm`（`c-c8-hy3d-ifeng.html` / `.txt`）；腾讯云混元生 3D 产品页 `https://cloud.tencent.com/product/ai3d`（`c-c8-hy3d-cloud.txt`，列生 3D API 计费 0.09–0.12 元/积分、新客赠 100 积分）；旧文档 `https://cloud.tencent.com/document/product/1804/120696`（`c-c8-hy3d-doc-intro.txt`）写正迁至 TokenHub、原平台仅列 3.0/3.1。`3d.hunyuan.tencent.com` 需登录未进。
- 发布时刻：凤凰网科技转 IT之家 `2026年11月27日 11:46:37`（北京时间）。
- 级别：L4（IT之家报道，无公开未登录 L1 页面）。
- 热度（`c2-heat.json`）：HN `Hy3D` 无相关命中。
- 状态：部分支持（L4 报道与腾讯云旧产品页均已存档，但 TokenHub 与混元 3D 官网未登录态下无 3.6 公开页面）。

#### C8d 小红书 Dots Studio 开源 3D 代码数据框架 3DCodeFactory（时间略早备选）
- 建议的一句事实（37 字）：`小红书开源 3D 代码框架 3DCodeFactory，含5045条轨迹。`
- 关键限定词原句（`c-c8-3dcodefactory-hf.txt`）：“The paper reports results for a corpus of 5,045 verified trajectories (5.52B tokens).”；“The Qwen SFT release contains 6,414 training records in 401 Zstandard-compressed JSONL shards (302,582,627,266 bytes).”；“Xiaohongshu Inc. · Dots Studio”。
- 一手来源 URL 与存档：`https://huggingface.co/datasets/3DCodeFactory/3DCodeFactory`（`c-c8-3dcodefactory-hf.html` / `.txt`、`c-c8-3dcodefactory-hf-api.json`）、`https://github.com/3DCodeFactory/3DCodeFactory`（`c-c8-3dcodefactory-gh.html` / `.txt`、`c-c8-3dcodefactory-commit.json`）、`https://arxiv.org/abs/2611.28783`（`c-c8-3dcodefactory-arxiv.html` / `.txt`）。
- 发布时刻：arXiv 提交 `2026-11-21 12:58:42 UTC`（北京 11-21 20:58）；GitHub 首次提交 `2026-11-23 12:21:12 UTC`（北京 11-23 20:21）；HF `lastModified` `2026-11-24T15:03:22Z`（北京 11-24 23:03）。
- 级别：L1（官方团队仓库/论文）。
- 热度（`c-heat.json` / `c2-heat.json`）：HN `3DCodeFactory` 0 命中。
- 状态：已找到。

#### C8e 英国完成首例 AI 智能体无卡银行支付买披萨【风控备选】
- 建议的一句事实（37 字）：`据卫报报道，英国完成首例 AI 智能体无卡银行支付买披萨，属监管沙盒测试。`
- 关键限定词原句（`c-c8-guardian-pizza.txt`）：“An order of Papa John’s margherita pizzas has become the first purchase in the UK to be made using an AI agent using cardless payment technology. The order of nine mini pizzas was made as part of the Financial Conduct Authority’s (FCA) regulated sandbox trials to test the technology, using Lloyds Bank and payment specialist TrueLayer.”
- 来源 URL 与存档：`https://www.theguardian.com/business/2026/nov/27/ai-agent-cardless-transaction-pizza-papa-john-uk-fca-truelayer`（`c-c8-guardian-pizza.html` / `.txt`）。
- 发布时刻：`Fri 27 Nov 2026 04.53 EST`（**北京 2026-11-27 17:53**；Last modified `07.28 EST` = 北京 20:28）。
- 级别：L4（《卫报》）。
- 热度（`c3-heat.json`）：HN `50665732`（10 分 / 11 评，北京 11-27 19:41 提交，21:20 采）。
- **风控提示**：涉及英国金融行为监管局（FCA）沙盒监管，仅供备选。
- 状态：已找到。

---

## 扫描说法勘误（独立复核结果）

| 扫描/派工单说法 | 核验结果 | 依据 |
|---|---|---|
| C1 “11-26 起灰度显示单轮 Token 与估算金额卡片、标‘免费’、无官方收费公告，凤凰网科技实测等写成‘打表计费’，截图混用空闲/高峰时段价” | 已核实：IT之家图按空闲价（0.02/1/4 元），凤凰网科技标题写“打表计费”且实测用高峰价（2/8 元），正文均标明仍免费、无公告 | `c-c1-ithome.txt`、`c-c1-ifeng-orig.txt`、`c-c1-deepseek-pricing.txt` |
| C2 “11-25（美东）Neuralink 宣布 FDA 批准 LINK Pivotal Study，自称美国首个植入式 BCI 关键性试验；媒体有写成‘上市在即’” | 已核实（官方页日期 Nov 25 无时刻；确自称 “the first pivotal trial … in the United States”，`/trials/` 明写尚未获 FDA 批准） | `c-c2-neuralink-update.txt`、`c-c2-neuralink-trials.txt` |
| C3 “11-26 KFTC 公布审查报告（11-24 提交）、6 周申辩、尚未裁决；KFTC 原文未列金额（法定上限 2%），媒体有 4–5 个金额版本” | 已核实：KFTC 官方 PDF 未列任何具体罚金，只写相关销售额约 3.3527 万亿韩元及 6 周书面意见权；BusinessKorea 的 268 亿–536 亿韩元系按 0.8%–1.6% 费率推算 | `c-c3-korea-press-pdf.txt`、`c-c3-businesskorea.txt`、`c-c3-macrumors.txt` |
| C4 “11-26 Android Authority 报道 Google 支持团队在 Reddit 回复‘单标签超 5GB’帖称已修复，另有用户报 8.4GB” | 部分支持：Google 支持确在 11-24 17:15Z（北京 11-25 01:15）回复称已解决且未说明原因；但 6.4GB/8.4GB 峰值是 AA 引述另一位用户的过往报告，非 `That_Discussion_7513` 原帖 | `c-c4-reddit-opencli.txt`、`c-c4-reddit-meta.json`、`c-c4-androidauthority.txt` |
| C5 “11-26/27 报道 SAP 考虑裁员至少 1 万人（9%–10%），正与劳方谈判尚未定案，官方回应笼统” | 部分支持：人数、比例（at least 9 per cent）、官方笼统回应均核实；但 FT 报道称计划**尚未与员工代表讨论**（yet to be discussed），彭博写最终规模可能在劳工谈判后变化 | `c-c5-cna.txt`、`c-c5-sbs.txt` |
| C6 “Yandex 开源 800 亿参数基础模型 AliceAI-Foundation-80B-A3B-Base” | **部分支持 / 时间重大勘误**：参数、3B 激活、256K、17.5T、Apache 2.0 均属实；但模型早在 **2026-09-21** 已在 HF 与 Habr 开源（HF `createdAt` 09-12，评论区始于 9 月 21 日），11-25 提交的只是 arXiv 论文 | `c-c6-yandex-hf-api.json`、`c-c6-yandex-habr.html`、`c-c6-yandex-habr-comments.txt`、`c-c6-yandex-arxiv.txt` |
| C7 派工单 URL `.../tree/4a7bc8ab0bc770e71a259b96b99010a/...` | **URL 勘误**：派工单里的 commit hash 漏了 `7966c50049`（返回 404）；实际 40 位 hash 为 `4a7bc8ab0bc770e71a259b7966c500496b99010a` | `c-hn-50651627.json`、`c-c7-github-commit.json` |
| C8 “Kimi 科研支持计划 11-27 发布” | 官网列表页 `/news/` 标注日期为 **`2026-11-26`**（国内媒体多在 11-27 报道） | `c-c8-kimi-news.txt` |

---

## 给主执行方的提示（非事实，供选题）

1. 本组共有 **9 条无风控且干净可用的候选**：推荐优先挑 **C1（DeepSeek 灰度卡片仍免费）、C2（Neuralink 临床试验）、C4（Gemini 网页内存修复）、C5（SAP 裁员考虑）、C7（Google Cogentic 整数分解论文）、C8a（Kimi for Science）、C8b（Kimi 游戏开发方案）、C8c（混元 Hy3D 3.6）** 共 8 条；若觉得 C8a 与 C8b 同属 Kimi 只需一条，可用 C8d（小红书 3DCodeFactory，11-21–24）补位，或去掉 C8b 仍剩 7 条非主帖候选（加 2 条主帖共 9 条，完全满足 7–10 条要求）。
2. **建议不收**：C3（韩国政府监管）、C8e（英国 FCA 沙盒）均含外国政府/监管机构风控风险；C6（Yandex AliceAI）实为 9 月已开源模型的论文补发。
3. **无隐私/平台名违规**：已核对 C1–C8 所有建议的一句事实，每条严格 ≤40 字符，且不含 X/Twitter/YouTube/Facebook/Instagram/Telegram 平台名。
