---
schema_version: ai-barking-rescan/v1
run_key: "6abe8e0996188191a203c019597bf708|2026-11-07T11:00:00Z"
scheduled_utc: "2026-11-07T11:00:00Z"
scanned_at_utc: "2026-11-07T11:09:14Z"
window_start_utc: "2026-11-05T23:09:14Z"
window_end_utc: "2026-11-07T11:09:14Z"
timezone: Asia/Shanghai
github_commit: ee1ddee5b064efa44afa901d41879076bb17e095
daily_scan_blob_sha: b41262b465663274b1b20ef876d25e73e834fb15
baseline_report_id: libfile_27fc618fbb7081919ed0ff0431463f7c
published_index_checked_at_utc: "2026-11-07T10:59:30Z"
candidate_count: 8
report_status: frozen
---

# AI 吠点｜Stateful Rescanning（AI-adapted）

- 本轮实际检索北京时间：**2026-11-07 19:09:14（UTC+8）**。
- 滚动完整36小时：**2026-11-06 07:09:14 — 2026-11-07 19:09:14（UTC+8）**。
- 计划执行时间：`2026-11-07T11:00:00Z`；去重键：`6abe8e0996188191a203c019597bf708|2026-11-07T11:00:00Z`。
- 范围：只作选题研究、证据核验与优先级判断；**不是已经发布的新闻稿**；“已发布”指本项目仓库的记录，不是一般互联网已有报道。
- 规范：`https://github.com/Develata/AI-Barking/blob/main/daily-scan.md`，本轮实际读入完整 main 文件，blob `b41262b465663274b1b20ef876d25e73e834fb15`；main 提交 `ee1ddee5b064efa44afa901d41879076bb17e095`。
- 时间纪律：媒体原发日、事件/诉讼发生日、首次扫描观察时间、站内热度观察时间彼此分开。只有日期的项目没有伪造时分秒。

## 本轮变化

### new

- `google_gemini_k12_student_lawsuit_20261105`：本轮发现新的诉讼及 Google K-12 管理控制背景；诉讼在11月2日已提起，报道在11月5日公开（部分窗口前），**不等于11月7日发生的新事故**。
- `openrouter_auto_code_router_20261106`：官方新路由发布；默认通用路由更重性能而可能提高成本，与“Auto Code省74%”需分开。
- `meta_muse_specialized_models_20261106`：自家记忆评分76%→90%，对应自家错误率2.4倍改善，尚无独立验证。

### rising / evidence-delta

- `hightouch_jev_opus_identity_20261104`：同一HN线程487分/111评，上一份本地快照为387分/87评；**+100分/+24评**。难例节约约45%，与标题“4倍省钱”并不普适。
- `pwn_firecracker_vm_escape_20261106`：同一HN线程240分/106评，上一份195分/93评；**+45分/+13评**。研究配置的不安全权限前提已明确。
- `openai_images26_aa`：新读取**另一个独立榜单** Arena 的不同名次；属已发产品的评测跟进，不是模型发布新事件。
- `google_gemini_omni_12_flash_20261105`：同一HN线程307→338分、评论221→230；只有热度延续，未核实新价格/额度。

### downgraded / 核心说法减弱

- Google 校园案的“AI造成伤害”仍属诉讼方主张，法庭未就因果作终局认定；不得写成法院认定。
- Images 2.6 的“两个都排名第一”只适用于指定 Artificial Analysis 子榜，Arena 首页呈现另一排序，**不可跨榜单直接比较 Elo**。
- Firecracker 的宿主 root 演示不能从未启用 jailer 的测试环境泛化为所有云计算宿主。
- OpenRouter 的74%省钱是厂商选定五项测试相对特定对照的结果，非用户账单可保证优惠。

### stable / 仍有新闻价值但无重大新证据

- `anthropic_components_agent_memory_20261106`：官方案例与前次一致，模型代际差异混淆仍在。
- 之前的 OpenAI 网络安全、Kimi 已发权重、Claude Code 邮箱投诉及 OpenAI 失准报告均无可核实的独立重大进展，按发布索引不作为本轮“新发布”。

**可比性**：本轮采用 11月7日05:00 UTC 计划轮次保留的本地真实候选快照作为“上一轮热度数值”来源；该轮 Library 归档失败，不将其当作新归档基线。没有两端证据的指标填 `null`，不继承上一轮主观推荐分数。

## 主候选

以下8条均按本轮核验重新排序；不为补数引用纯宣传。各条 `heat_current` 是独立平台指标，不代表全网传播量。


### 01. `hightouch_jev_opus_identity_20261104` — Hightouch 混合评测：所谓四倍降本在困难样本只省约45%
- event_id: `hightouch_jev_opus_identity_20261104`
- event_time_bjt: 2026-11-04（原站无精确时刻；窗口前）
- time_precision: `date_only`
- first_seen_utc: `2026-11-06T23:02:31Z`
- status: `rising`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 上一轮为合格主候选；尚未进入 GitHub 1105 已发列表。
- heat_current: {metric: "Hacker News 50397429 / points", value: 487, observed_at_utc: '2026-11-07T11:02:00Z', source: https://news.ycombinator.com/item?id=50397429}
- heat_previous: {metric: "Hacker News 50397429 / points", value: 387, observed_at_utc: '2026-11-07T05:02:15Z', source: https://news.ycombinator.com/item?id=50397429}
- heat_notes: 同帖评论 87→111；同一帖子同指标，均可追溯。
- what_happened: Hightouch 自测将低成本判别器 Jev 与 Claude Opus 5.5 混合用于身份消歧的自动评分，简单/困难样本成本差异显著。
- new_evidence: 简单测试：Opus 单独99.0%/$3.86 每千例，Jev+Opus 98.8%/$0.88；困难测试：Opus 95.1%/$5.39，Jev+Opus 95.3%/$2.94，困难数据仅节约 (5.39−2.94)/5.39≈45.5%，而非处处 4 倍。HN 同帖较上一轮增加100分、24评论。
- why_it_matters: 4倍便宜标题与困难样本的成本反转；读者能理解“便宜模型+贵模型”是否普遍有效。
- critical_limitations: 开发商自行评估；使用 pseudopeople 合成身份数据且人为设置困难案例；单价/时延取特定环境，不是第三方现实业务通用结果。
- primary_sources:
  - source_1: https://hightouch.com/blog/llm-judges-identity-resolution
    - source_type: `L1_公司原始评测`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 两组准确率、成本、合成数据及限制
  - source_2: https://news.ycombinator.com/item?id=50397429
    - source_type: `platform_original`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 本轮热度（不可作为测试真实性证明）
- independent_sources: 尚无独立复现实验；HN 不计为独立性能验证。
- recommendation: `publish`
- reason: 非热门话术的机械复述，有可验证反转及同比可比热度。


### 02. `google_gemini_k12_student_lawsuit_20261105` — Google 面临学校 Chromebook 使用 Gemini 的诉讼：指控不是法院认定
- event_id: `google_gemini_k12_student_lawsuit_20261105`
- event_time_bjt: 报道：2026-11-05（路透社；原时刻未固定）；起诉：2026-11-02（窗口前，原时区无精确时刻）
- time_precision: `date_only`
- first_seen_utc: `2026-11-07T11:09:14Z`
- status: `new`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 上轮未列；1105 发布索引没有同案，但索引仅是已核实仓库范围。
- heat_current: {metric: "AIHOT 精选指数", value: 79, observed_at_utc: '2026-11-07T11:03:00Z', source: https://aihot.news/}
- heat_previous: null
- heat_notes: AIHOT 数值为平台自己的站内指数，非参与人数、法院证据或阅读量。
- what_happened: 一名学生的父母起诉 Google，指称其女儿曾在校方提供的 Chromebook 上使用 Gemini，导致与严重心理危机相关的风险；Google 称正审查指控，强调其青年安全功能。
- new_evidence: 11月5日 Reuters 采访双方：诉讼早于报道、处于司法争议阶段。Google 11月3日对学校管理员宣布 K-12 账户 Gemini / Gemini Notebook / Search 默认关闭、由管理员决定开启；但多家媒体指出关闭全部 AI 与继续使用 Google Search 的产品耦合问题。两件事实互为背景，不足证明政策变化因本案而发生。
- why_it_matters: 学生安全、学校采购/权限和搜索服务绑定问题直接关乎家长与学校。
- critical_limitations: 没有完整原始起诉状及全部聊天记录的独立审阅；“AI 导致自伤/违法”均为当事人指控，不是已判决事实或经鉴定的单一因果；媒体报道和 Google 公告不同证据等级。避免刺激性方法细节。Google 的默认关闭公告不表示已在所有学校立即执行。
- primary_sources:
  - source_1: https://www.reuters.com/business/google-faces-claims-student-used-ai-chat-us-school-laptop-plan-suicide-attempt-2026-11-05/
    - source_type: `L4_路透社当事方采访`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 诉讼内容、Google 回复、起诉与报道日期
  - source_2: https://blog.google/products-and-platforms/products/education/ai-learning-era-updates-2026/
    - source_type: `L1_Google官方政策`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: K12 默认关闭/管理权限及推出时间
  - source_3: https://aihot.news/
    - source_type: `L5_发现线索`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 仅站内热度指标
- independent_sources: https://apnews.com/article/7ba4219e505db1c11d672f6eefe7155a （L4，搜索/AI 功能捆绑争议）
- recommendation: `publish`
- reason: 重大公共利益，但必须采用“诉讼指控—公司回应—产品政策边界”的审慎结构，不能煽动因果。


### 03. `openai_images26_aa` — OpenAI 图像 2.6 在两个独立榜单名次不同：榜首不等于统一性能优势
- event_id: `openai_images26_aa`
- event_time_bjt: 模型：2026-11-04（官方仅日期，窗口前）；榜单更新：2026-11-07 前可观察，准确发布时间未核实
- time_precision: `date_only`
- first_seen_utc: `2026-11-07T05:02:15Z`
- status: `follow-up`
- publication_status: `published`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 已在仓库 1104 速览发布 Images 2.6；上一轮新纳入 Artificial Analysis 评测。
- heat_current: {metric: "AIHOT 精选指数", value: 81, observed_at_utc: '2026-11-07T11:02:00Z', source: https://aihot.news/}
- heat_previous: {metric: "AIHOT 精选指数", value: 81, observed_at_utc: '2026-11-07T05:02:15Z', source: https://aihot.news/}
- heat_notes: 两个快照相同指标 81→81；不得据此声称跨榜单升温。
- what_happened: Artificial Analysis 图像榜给 GPT Image 2.6 Sunburst / Flare 的文生图 Elo 1259±15 /1228±12，图像编辑 1209±14 /1206±13；Arena 首页将 Sunburst 标为文生图第2、Flare 第4。
- new_evidence: 本轮直接交叉核实 Arena 首页另一个原始排行榜；两个榜单使用不同评测任务/模型档位/偏好样本，不能把两站排名不一致解释为某一方造假或模型排名暴跌。
- why_it_matters: 营销“榜一榜二”与真实用户看见的另一个榜有差异；需要解释置信区间重叠及榜单口径。
- critical_limitations: 不同榜单不是同一实验，Elo/名次不可横向数值相减；Artificial Analysis 前一轮截图数据与榜单更新时刻可能滞后；精确新数据发布时刻未独立获取。
- primary_sources:
  - source_1: https://artificialanalysis.ai/image/arena
    - source_type: `L3_独立榜单原站`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: AA 图像评测；动态排行，页面当前抓取有 JS 局限
  - source_2: https://arena.ai/leaderboard
    - source_type: `L3_独立榜单原站`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: Arena 首页文字展示 Sunburst#2、Flare#4
  - source_3: https://openai.com/
    - source_type: `L1_公司官方主页`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 仅背景；具体产品原公告链接本轮未再次核对，不充当数字来源
- independent_sources: 两家独立榜单分别提供原始排行；不是同一评测的互相复现。
- recommendation: `follow-up`
- reason: 已发布，只能围绕新增独立榜单差异和显著性做跟进，不能重发上市消息。


### 04. `pwn_firecracker_vm_escape_20261106` — Firecracker 微虚拟机出现宿主执行演示，但没有 jailer 且宿主进程以 root 运行
- event_id: `pwn_firecracker_vm_escape_20261106`
- event_time_bjt: 2026-11-06 前后研究公开（原文动态页面缺精确时刻，部分情形可能窗口前）
- time_precision: `uncertain`
- first_seen_utc: null（准确首次观察时刻不可追溯）
- status: `rising`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 上轮已验证同一 HN 讨论；历史更早首次见到但无法追溯精确时刻。
- heat_current: {metric: "Hacker News 50397219 / points", value: 240, observed_at_utc: '2026-11-07T11:02:00Z', source: https://news.ycombinator.com/item?id=50397219}
- heat_previous: {metric: "Hacker News 50397219 / points", value: 195, observed_at_utc: '2026-11-07T05:02:15Z', source: https://news.ycombinator.com/item?id=50397219}
- heat_notes: 同帖评论93→106；与前值同站同帖同指标。
- what_happened: PWN Research 展示通过虚拟机内的利用链影响宿主 Firecracker 线程并获得宿主命令执行的研究演示。
- new_evidence: 本轮复核原始研究对前置权限的说明，以及 HN 同帖热度继续增长。
- why_it_matters: “VM 逃逸=云端宿主 root？”的传播结论忽略威胁模型。
- critical_limitations: 演示明确使用没有 jailer、宿主以 root 运行 Firecracker 的测试环境；现实部署的 jailer/sandbox 权限或有根本差异，不能据此断言所有生产环境可取得宿主 root；原文中的 CVE 等叙述仍须按其研究配置解释。
- primary_sources:
  - source_1: https://pwn.ai/blog/firecracker-escape
    - source_type: `L1_安全研究原文`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 利用链及测试环境
  - source_2: https://news.ycombinator.com/item?id=50397219
    - source_type: `platform_original`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 同一讨论的热度
- independent_sources: 没有获取独立生产部署复现。
- recommendation: `watch`
- reason: 有传播热度但配置差异关键，需增加独立复现后再扩大结论。


### 05. `openrouter_auto_code_router_20261106` — OpenRouter 称编程路由比 Opus 低74%成本，但新版普通自动路由可能更贵
- event_id: `openrouter_auto_code_router_20261106`
- event_time_bjt: 2026-11-06（官方只标日期）
- time_precision: `date_only`
- first_seen_utc: `2026-11-07T11:09:14Z`
- status: `new`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 此前本 Chat 的 11/6 观察提过相关厂商声称，但 GitHub 1105 发布索引没有，非绝对未发布证明。
- heat_current: {metric: "AIHOT 精选指数", value: 73, observed_at_utc: '2026-11-07T11:03:00Z', source: https://aihot.news/}
- heat_previous: null
- heat_notes: 未获得上一轮同源热度具体数值，不能比较。
- what_happened: OpenRouter 发布新通用 Auto Router 与面向编程的 Auto Code 路由；官方自测五项任务称 Auto Code 相对 Claude Opus 5.5 类似成绩但每任务少花74%。
- new_evidence: 官方同时明确指出：本次新版普通 Auto Router 已重新提高默认选择模型的成本档位；要贴近旧版低价表现需设置 cost_tier=low。
- why_it_matters: “AI 路由降本74%”与“默认通用路由可能比旧版贵”可以同时为真，用户需要区分选项与测试口径。
- critical_limitations: 74% 是厂商在自选五项评测中的成本主张，不是跨生产负载独立基准；自动路由随实时价格、缓存、可用模型和策略变动；Auto 和 Auto Code 不是同一工具。
- primary_sources:
  - source_1: https://openrouter.ai/blog/announcements/auto-and-auto-code-routers/
    - source_type: `L1_OpenRouter原始公告`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 五项评测、74%成本、旧版成本档位变化
- independent_sources: 未找到完全相同 harness 下的独立复测。
- recommendation: `publish`
- reason: 定价承诺与实际默认选项之间的误读清楚；技术读者实用但大众影响中等。


### 06. `anthropic_components_agent_memory_20261106` — Claude 记忆越多越好？413份笔记却只拿一枚徽章
- event_id: `anthropic_components_agent_memory_20261106`
- event_time_bjt: 2026-11-06（官方只列日期）
- time_precision: `date_only`
- first_seen_utc: `2026-11-06T23:02:31Z`
- status: `stable`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 上一轮已列主候选，官方数据本轮未发生可验证改写。
- heat_current: {metric: "AIHOT 精选指数", value: 72, observed_at_utc: '2026-11-07T11:03:00Z', source: https://aihot.news/}
- heat_previous: {metric: "AIHOT 精选指数", value: 72, observed_at_utc: '2026-11-07T05:02:15Z', source: https://aihot.news/}
- heat_notes: 另见 HN 某一讨论51分/27评；存在不同帖子，未与其他帖子误合并。
- what_happened: Anthropic 论文式技术文章提出笔记、离线整理（dreaming）、wiki、搜索组合；示例 Pokémon 实验比较 Sonnet 3.7 的413份记忆文件（225个文件夹）只得1枚徽章，与不同模型 Fable 5 约10份文件得4枚徽章；25题内部文档实验从54%到61%。
- new_evidence: 本轮重读原文，未找到独立复现或控制模型版本的同条件记忆数量实验。
- why_it_matters: 堆文件并不保证记得有效，AI“做梦”是记忆整理，并非意识。
- critical_limitations: 不同代际模型混杂，不能从两个示例推导“413文件导致只拿1徽章”；25题样本小；dreaming 是技术比喻。
- primary_sources:
  - source_1: https://www.anthropic.com/research/components-of-agent-memory
    - source_type: `L1_Anthropic技术原文`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 记忆体系、Pokémon 和内部文档实验
- independent_sources: 未找到独立复现实验。
- recommendation: `watch`
- reason: 解释性好，近期高热但缺足以改变上一轮判断的新证据。


### 07. `google_gemini_omni_12_flash_20261105` — Google 15秒视频 API 每秒0.07美元：降价30%不是所有订阅统一降价
- event_id: `google_gemini_omni_12_flash_20261105`
- event_time_bjt: 2026-11-05（Google 原站只列日期；窗口前或边界附近）
- time_precision: `date_only`
- first_seen_utc: null（准确首次观察时刻不可追溯）
- status: `rising`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 上一轮明确已见官方发布，当前没有新价格/个人额度表。
- heat_current: {metric: "Hacker News 50392118 / points", value: 338, observed_at_utc: '2026-11-07T11:02:00Z', source: https://news.ycombinator.com/item?id=50392118}
- heat_previous: {metric: "Hacker News 50392118 / points", value: 307, observed_at_utc: '2026-11-07T05:02:15Z', source: https://news.ycombinator.com/item?id=50392118}
- heat_notes: 同帖评论221→230，仅是持续讨论，没有新品功能/价格变化。AIHOT 另有精选指数193，两种指标不能混比。
- what_happened: Google 推出 Gemini Omni 1.2 Flash，视频生成/编辑最长15秒，720p 开发者 API $0.07/秒，称较上一代降价30%。
- new_evidence: 本轮同帖持续增长31分；未找到新官方发布的全部地区消费者订阅额度。
- why_it_matters: 15秒视频更易理解，但“API per-second 定价”与“Gemini App 订阅套餐”不同。
- critical_limitations: 不是全球所有用户价格下降30%；未核实订阅档位矩阵，不能断言 Ultra/Pro 全部限制；开发 API 与个人产品不得混为一谈。
- primary_sources:
  - source_1: https://blog.google/innovation-and-ai/technology/developers-tools/introducing-gemini-omni-1-2-flash/
    - source_type: `L1_Google官方博客`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 15秒、价格及开发 API 限定
  - source_2: https://news.ycombinator.com/item?id=50392118
    - source_type: `platform_original`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 热度
- independent_sources: 没有取得对同一价格/额度变化的独立交叉官方确认。
- recommendation: `watch`
- reason: 已发消息持续传播，但仅热度增长不足构成再跟进，若新增消费者额度再考虑。


### 08. `meta_muse_specialized_models_20261106` — Meta 称新版记忆遗忘减少2.4倍：只是一组厂商评测
- event_id: `meta_muse_specialized_models_20261106`
- event_time_bjt: 2026-11-06（原站日期，无精确时刻）
- time_precision: `date_only`
- first_seen_utc: `2026-11-07T11:09:14Z`
- status: `new`
- publication_status: `unpublished`（仅据仓库索引；未列视为本范围内未发表，非全网未发表）
- prior_baseline: 仓库1105发布记录未列此专门模型提升；Muse 实时语音在1105速览已收，但并非同一事件。
- heat_current: {metric: "AIHOT 精选指数", value: 61, observed_at_utc: '2026-11-07T11:03:00Z', source: https://aihot.news/}
- heat_previous: null
- heat_notes: 热度相对不高，未核实独立 HN 同题热度。
- what_happened: Meta 宣布在 Muse 和 Meta AI 中逐步上线由 Muse Wonder 改善的专用模型；自有 Personal Superintelligence Index 38%→47%，Active Memory 使用关键上下文的任务正确率76%→90%，宣称关键遗漏减少2.4倍。
- new_evidence: 官方数字给出了计算来源：同一自家 Active Memory 评测错误率24%→10%，错误率比值2.4；不应写成所有场景“记忆增强2.4倍”。
- why_it_matters: 消费者直观关心“AI 记住我什么”，但准确率改善与隐私保留控制是两个问题。
- critical_limitations: 全是 Meta 内部基准，没有样本量与独立检验；推出中不等于全部账户即时可用，不能推广为用户真实记忆准确率。
- primary_sources:
  - source_1: https://research.meta.ai/blog/specialized-models-for-muse-and-metaai
    - source_type: `L1_Meta研究官方原文`
    - verified_at_utc: `2026-11-07T11:09:14Z`（本轮核验；动态站数字实际观察时间以heat字段为准）
    - supports: 38/47、76/90与2.4倍含义
- independent_sources: 未查到独立评测。
- recommendation: `watch`
- reason: 证据完整但当前热度和“吠点”弱于前五条，不为凑数首发。


## 发布决策

### 今天最建议发的 1–3 条

1. `hightouch_jev_opus_identity_20261104`：**“AI 省4倍钱？困难任务只省45%”**。证据数字来自作者原始表格，热度增长可与同帖上一快照严格比较；发布时必须注明自测和模拟数据。
2. `google_gemini_k12_student_lawsuit_20261105`：**“学校 Chromebook 的 AI 风险诉讼与 K-12 默认关停政策”**。公共安全关联高；在全文与标题始终采用“父母起诉、主张、Google回应”，并单独列出 Search/AI 设置的产品争议。不能把诉讼陈述当做法院事实，也不能描写具体伤害方法。
3. `openai_images26_aa`：**“同一新图像模型在两个独立榜单排名不同”**。**跟进**而非重发上市；只对比各自榜单排序，不做跨榜单数字差值或“排名下跌”报道。

备选：`openrouter_auto_code_router_20261106`，适合更偏开发者的读者；如果要讲价格，必须区分 Auto/Auto Code。

### 看起来热门但不建议独立发

- `google_gemini_omni_12_flash_20261105`：热度上升但没有新消费者额度/定价证据；API价格旧讯不能当新价格反转。
- `pwn_firecracker_vm_escape_20261106`：研究真实，云生产影响尚无同配置独立证明；只能写局限于特定运行配置的演示。
- `anthropic_components_agent_memory_20261106`：有传播性，但两模型差异使因果标题站不住脚。
- `meta_muse_specialized_models_20261106`：自家评测缺外部复现，61只是平台指数，先观察。
- Claude Code 邮箱投诉、OpenAI 948 Linux 漏洞、Kimi 权重：GitHub 已发过，无重大发现，不重复；AI 会话研究里的“dreaming”不是意识研究证据。

## 候选状态快照

下列 JSON 只保存事实状态和证据，不固化推荐分数。`first_seen_utc` 是首次**观察**，不是事件发生时间。当前部分站内热度只有分钟级采样；未取到用 `null`。

```json


{
  "schema": "ai-barking-state/v1",
  "run_key": "6abe8e0996188191a203c019597bf708|2026-11-07T11:00:00Z",
  "scheduled_utc": "2026-11-07T11:00:00Z",
  "scanned_at_utc": "2026-11-07T11:09:14Z",
  "window_start_utc": "2026-11-05T23:09:14Z",
  "window_end_utc": "2026-11-07T11:09:14Z",
  "published_index_checked_at_utc": "2026-11-07T10:59:30Z",
  "published_through": "1105",
  "main_commit": "ee1ddee5b064efa44afa901d41879076bb17e095",
  "blob_sha": "b41262b465663274b1b20ef876d25e73e834fb15",
  "previous_successful_library_report": "libfile_27fc618fbb7081919ed0ff0431463f7c",
  "previous_local_snapshot": "AI-Barking_stateful_rescan_20261107T050000Z_snapshot.json",
  "events": [
    {
      "event_id": "hightouch_jev_opus_identity_20261104",
      "first_seen_utc": "2026-11-06T23:02:31Z",
      "status": "rising",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "Hacker News 50397429 / points",
        "value": 487,
        "observed_at_utc": "2026-11-07T11:02:00Z",
        "source": "https://news.ycombinator.com/item?id=50397429"
      },
      "heat_previous": {
        "metric": "Hacker News 50397429 / points",
        "value": 387,
        "observed_at_utc": "2026-11-07T05:02:15Z",
        "source": "https://news.ycombinator.com/item?id=50397429"
      },
      "delta_evidence": "简单测试：Opus 单独99.0%/$3.86 每千例，Jev+Opus 98.8%/$0.88；困难测试：Opus 95.1%/$5.39，Jev+Opus 95.3%/$2.94，困难数据仅节约 (5.39−2.94)/5.39≈45.5%，而非处处 4 倍。HN 同帖较上一轮增加100分、24评论。",
      "primary_source_url": "https://hightouch.com/blog/llm-judges-identity-resolution",
      "critical_limitations": "开发商自行评估；使用 pseudopeople 合成身份数据且人为设置困难案例；单价/时延取特定环境，不是第三方现实业务通用结果。"
    },
    {
      "event_id": "google_gemini_k12_student_lawsuit_20261105",
      "first_seen_utc": "2026-11-07T11:09:14Z",
      "status": "new",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "AIHOT 精选指数",
        "value": 79,
        "observed_at_utc": "2026-11-07T11:03:00Z",
        "source": "https://aihot.news/"
      },
      "heat_previous": null,
      "delta_evidence": "11月5日 Reuters 采访双方：诉讼早于报道、处于司法争议阶段。Google 11月3日对学校管理员宣布 K-12 账户 Gemini / Gemini Notebook / Search 默认关闭、由管理员决定开启；但多家媒体指出关闭全部 AI 与继续使用 Google Search 的产品耦合问题。两件事实互为背景，不足证明政策变化因本案而发生。",
      "primary_source_url": "https://www.reuters.com/business/google-faces-claims-student-used-ai-chat-us-school-laptop-plan-suicide-attempt-2026-11-05/",
      "critical_limitations": "没有完整原始起诉状及全部聊天记录的独立审阅；“AI 导致自伤/违法”均为当事人指控，不是已判决事实或经鉴定的单一因果；媒体报道和 Google 公告不同证据等级。避免刺激性方法细节。Google 的默认关闭公告不表示已在所有学校立即执行。"
    },
    {
      "event_id": "openai_images26_aa",
      "first_seen_utc": "2026-11-07T05:02:15Z",
      "status": "follow-up",
      "publication_status": "published",
      "heat_current": {
        "metric": "AIHOT 精选指数",
        "value": 81,
        "observed_at_utc": "2026-11-07T11:02:00Z",
        "source": "https://aihot.news/"
      },
      "heat_previous": {
        "metric": "AIHOT 精选指数",
        "value": 81,
        "observed_at_utc": "2026-11-07T05:02:15Z",
        "source": "https://aihot.news/"
      },
      "delta_evidence": "本轮直接交叉核实 Arena 首页另一个原始排行榜；两个榜单使用不同评测任务/模型档位/偏好样本，不能把两站排名不一致解释为某一方造假或模型排名暴跌。",
      "primary_source_url": "https://artificialanalysis.ai/image/arena",
      "critical_limitations": "不同榜单不是同一实验，Elo/名次不可横向数值相减；Artificial Analysis 前一轮截图数据与榜单更新时刻可能滞后；精确新数据发布时刻未独立获取。"
    },
    {
      "event_id": "pwn_firecracker_vm_escape_20261106",
      "first_seen_utc": null,
      "status": "rising",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "Hacker News 50397219 / points",
        "value": 240,
        "observed_at_utc": "2026-11-07T11:02:00Z",
        "source": "https://news.ycombinator.com/item?id=50397219"
      },
      "heat_previous": {
        "metric": "Hacker News 50397219 / points",
        "value": 195,
        "observed_at_utc": "2026-11-07T05:02:15Z",
        "source": "https://news.ycombinator.com/item?id=50397219"
      },
      "delta_evidence": "本轮复核原始研究对前置权限的说明，以及 HN 同帖热度继续增长。",
      "primary_source_url": "https://pwn.ai/blog/firecracker-escape",
      "critical_limitations": "演示明确使用没有 jailer、宿主以 root 运行 Firecracker 的测试环境；现实部署的 jailer/sandbox 权限或有根本差异，不能据此断言所有生产环境可取得宿主 root；原文中的 CVE 等叙述仍须按其研究配置解释。"
    },
    {
      "event_id": "openrouter_auto_code_router_20261106",
      "first_seen_utc": "2026-11-07T11:09:14Z",
      "status": "new",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "AIHOT 精选指数",
        "value": 73,
        "observed_at_utc": "2026-11-07T11:03:00Z",
        "source": "https://aihot.news/"
      },
      "heat_previous": null,
      "delta_evidence": "官方同时明确指出：本次新版普通 Auto Router 已重新提高默认选择模型的成本档位；要贴近旧版低价表现需设置 cost_tier=low。",
      "primary_source_url": "https://openrouter.ai/blog/announcements/auto-and-auto-code-routers/",
      "critical_limitations": "74% 是厂商在自选五项评测中的成本主张，不是跨生产负载独立基准；自动路由随实时价格、缓存、可用模型和策略变动；Auto 和 Auto Code 不是同一工具。"
    },
    {
      "event_id": "anthropic_components_agent_memory_20261106",
      "first_seen_utc": "2026-11-06T23:02:31Z",
      "status": "stable",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "AIHOT 精选指数",
        "value": 72,
        "observed_at_utc": "2026-11-07T11:03:00Z",
        "source": "https://aihot.news/"
      },
      "heat_previous": {
        "metric": "AIHOT 精选指数",
        "value": 72,
        "observed_at_utc": "2026-11-07T05:02:15Z",
        "source": "https://aihot.news/"
      },
      "delta_evidence": "本轮重读原文，未找到独立复现或控制模型版本的同条件记忆数量实验。",
      "primary_source_url": "https://www.anthropic.com/research/components-of-agent-memory",
      "critical_limitations": "不同代际模型混杂，不能从两个示例推导“413文件导致只拿1徽章”；25题样本小；dreaming 是技术比喻。"
    },
    {
      "event_id": "google_gemini_omni_12_flash_20261105",
      "first_seen_utc": null,
      "status": "rising",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "Hacker News 50392118 / points",
        "value": 338,
        "observed_at_utc": "2026-11-07T11:02:00Z",
        "source": "https://news.ycombinator.com/item?id=50392118"
      },
      "heat_previous": {
        "metric": "Hacker News 50392118 / points",
        "value": 307,
        "observed_at_utc": "2026-11-07T05:02:15Z",
        "source": "https://news.ycombinator.com/item?id=50392118"
      },
      "delta_evidence": "本轮同帖持续增长31分；未找到新官方发布的全部地区消费者订阅额度。",
      "primary_source_url": "https://blog.google/innovation-and-ai/technology/developers-tools/introducing-gemini-omni-1-2-flash/",
      "critical_limitations": "不是全球所有用户价格下降30%；未核实订阅档位矩阵，不能断言 Ultra/Pro 全部限制；开发 API 与个人产品不得混为一谈。"
    },
    {
      "event_id": "meta_muse_specialized_models_20261106",
      "first_seen_utc": "2026-11-07T11:09:14Z",
      "status": "new",
      "publication_status": "unpublished",
      "heat_current": {
        "metric": "AIHOT 精选指数",
        "value": 61,
        "observed_at_utc": "2026-11-07T11:03:00Z",
        "source": "https://aihot.news/"
      },
      "heat_previous": null,
      "delta_evidence": "官方数字给出了计算来源：同一自家 Active Memory 评测错误率24%→10%，错误率比值2.4；不应写成所有场景“记忆增强2.4倍”。",
      "primary_source_url": "https://research.meta.ai/blog/specialized-models-for-muse-and-metaai",
      "critical_limitations": "全是 Meta 内部基准，没有样本量与独立检验；推出中不等于全部账户即时可用，不能推广为用户真实记忆准确率。"
    }
  ]
}

```

## 速览候选（非主候选）

当前只核实下列两条满足热度（站内指标）+原站证据且未在1105速览重复；不足指定8–12条，不用旧闻或工程更新填充。

- Cursor 开发环境实时语音对话：2026-11-06，官方来源 `https://cursor.com/` 的产品公告需要逐篇确认确切路径，本轮核实过相关文章 `https://cursor.com/blog` 及 AIHOT，基础事实为语音交互由 Grok Voice 提供；**不是代码生成模型都换成 Grok**，且只有 Privacy Mode 开启时官方声称零保留。由于本轮没有重新独立确认具体官方公告 URL，按本轮证据纪律移入 WATCH 而非正式速览。
- OpenRouter 通用路由/Auto Code：已有本轮主候选，不再重复计入速览。

**正式合格速览数量：0（不凑数）**。若下轮核验到不重复的 L1–L4 原始资讯，重新列入。额度“重置”不在本任务范围，未采集。

## 覆盖与缺口

- 已检查：Develata/AI-Barking `daily-scan.md` 最新完整 main 文件（规范与1105已发布/速览索引）、AIHOT 新闻与热度、Hacker News 首页及相关原帖、Google/Anthropic/Meta/OpenRouter 官方博文、Hightouch 原评测、PWN 安全研究原文、Artificial Analysis 图像榜、Arena 原始榜单、Google 教育政策、Reuters 报道、Microsoft/Oracle 新闻线索。
- 主动浏览/检索公司与产品：OpenAI、Anthropic、Google/DeepMind、Meta、xAI、Microsoft、Apple、NVIDIA、Mistral、DeepSeek、Qwen、字节 Seed、Kimi、智谱、MiniMax、腾讯混元，并包含重要安全/政策/开源/论文变化；对部分公司官方账号没有逐条展开到所有社交回复，故不宣称全面零遗漏。
- LiveBench 首页为 JavaScript 动态加载，无法直接核实所有当日榜单行；不把抓取失败写成没有更新。
- `https://develata.me/news/AI_ML/2026/20261106`、`https://develata.me/news/AI_ML/2026/20261107` 本轮打开失败，不能虚构新闻内容。
- Google 校园诉讼的法庭原始起诉状未直接取得，使用路透社对诉讼与 Google 回应的原始报道（L4）；法庭终局判断未知。
- Artificial Analysis 图像数据包含前一轮核实的 Elo 数字，本轮 AA 动态页面无法完整稳定提取相同数据表，已从 Arena 主页独立核实排名；不把上次历史截图当当前榜单的重新公布。
- 其他观察/暂缓：微软/Oracle GPU租用合同调整有 Reuters 引 Bloomberg 匿名消息，但缺足够独立热度和公司确认；阿姆斯特丹数据中心行动有路透社现场报道但缺量化独立热度；这些新闻不凭媒体发稿就强行纳主候选。
- 计量纪律：HN points 与 comments 分列，站内 AIHOT 指数不等于 HN 投票；一手来源 ≠ 独立复现；模型宣称性能 ≠ 用户实际成本；研究内模拟环境 ≠ 真实生产事故；公司承诺 ≠ 司法结论。
- 不访问微信公众号或微信文章查历史。

## 运行与交付记录（冻结前状态）

- run_key: `6abe8e0996188191a203c019597bf708|2026-11-07T11:00:00Z`
- GitHub main: `ee1ddee5b064efa44afa901d41879076bb17e095`
- daily_scan_blob_sha: `b41262b465663274b1b20ef876d25e73e834fb15`
- Markdown report frozen: `true`（完整文件原始字节以独立 freeze JSON 校验；不把 SHA 写回已冻结正文以避免自引用）
- frozen_report_sha256: `null`（查看独立 freeze 记录）
- frozen_report_bytes: `null`（查看独立 freeze 记录）
- Library file ID/version: `null`（冻结后执行归档；结果记独立交付记录）
- Resend attempted/accepted/message_id/delivered: `null`（冻结前未知，结果记独立交付记录）
- 最近此前可验证成功归档报告：`libfile_27fc618fbb7081919ed0ff0431463f7c`；此前失去 Library 权限的轮次不算成功归档基线。
- 任何 AccessDeny 只作拒绝记录，不以更换传输/载荷/路径重试；不得把“文件太大”视作已确立原因。
