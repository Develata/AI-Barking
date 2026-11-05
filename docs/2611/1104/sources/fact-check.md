# 1104 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（OpenAI 948 个提权漏洞）、`evidence-B.md`（Kimi-K3.1-Pro 权重）、`evidence-C.md`、`evidence-d.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证于北京 11-05 17:20 至约 18:45；Claude 另直接读过 OpenAI 原文存档（`a-openai-paths-to-root.txt`）、Kimi 模型卡与 LICENSE（17:15 前后，raw 文件）、AA v5 发布文与 vLLM 配方存档的关键段。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 封面文字：主标题「AI挖出948个漏洞？」，无副标题 | — | ✅ | 见 A3 | 初版副标题「补丁只发了数百份」把“报告和补丁”缩成“补丁”（第三轮反向核验），重出封面时删去 |
| 标题：花121万美元，AI挖出948个漏洞？ | — | ✅ | 见 A3、A5 | 读者稿检查后由 titles.md #1 改为 #2。“121万美元”= $1,213,430 API 费用；问号所问为中文稿“挖出948个漏洞”的说法 |
| OpenAI 称用还没发布的 AI，在 Linux 系统的核心（内核）里挖出948个能拿到最高权限的漏洞，并写了补丁 | L1 | ✅ | 见 A1、A3、A4 | “还没发布的 AI”= “an upcoming cyber model” |
| 有媒体写成“已提交补丁”，原文只说已发出数百份 | L1 | ✅ | 见 A 节吠点①两行 | 媒体措辞为实例（L5），只核其原话 |
| 月之暗面按时放出 Kimi-K3.1-Pro 的模型本体（权重），约1.56 TB | L1 | ✅ | 见 B1–B3、B5 | |
| 但许可证给大公司加了收入门槛 | L1 | ✅ | 见 B 节吠点①许可证三行 | 门槛 = 第 2 条（被授权方及关联方连续 12 个月总收入超 2000 万美元、经营模型服务）与第 3 条（月收入超 2000 万美元或月活超 1 亿的商业产品）；“大公司”为通俗概括 |

## A OpenAI：AI 挖出948个内核漏洞

一手来源：https://openai.com/index/a-thousand-fewer-paths-to-root/ （`a-openai-paths-to-root.html/.txt`，opencli 取；curl 403）。页面只有日期 “November 4, 2026”；RSS pubDate 11-04 08:00 GMT（北京 16:00）；官方 X 首帖北京 11-05 07:13:24；HN 提交 07:33:14；IT之家 06:51:59 已有稿。发布时刻推不出，正文不写日期。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| A1 OpenAI 发文称，用一款即将推出的网络安全模型 | L1 | ✅ | 原文导语 “powered by an upcoming cyber model”（图 02） | 原文无模型名、无发布时间 |
| A2 自动扫描10月中旬最新的 Linux 内核 | L1 | ✅ | “In a run in mid-October 2026”；“The scan was performed on the latest Linux kernel in mid-October 2026.”（图 03、08） | 另有 “for a commonly used Linux distribution”，发行版未写 |
| A3 找出948个能让普通用户拿到最高权限的漏洞 | L1 | ✅ | 副标题 “validated and generated patches for 948 local privilege escalation vulnerabilities in Linux”（图 01）；“found and exploited 948 unique LPE vulnerabilities”（图 09） | “能让普通用户拿到最高权限”= local privilege escalation（“could give unprivileged local users root access”，图 06）。948 = 1,169 个可用利用按 “948 distinct patches” 合并后的唯一漏洞数（图 06），正文不展开 |
| A4 逐个写攻击程序验证，再写补丁 | L1 | ✅ | 五阶段：discovery → validation → exploitability assessment → patch generation → human review（图 02、11）；“For each validated vulnerability, the system also generates a patch.”（图 11） | “逐个”指流水线对每个验证过的漏洞都写利用与补丁；948 均有可用利用（图 06） |
| A5 这一轮光调用模型就花了约121万美元 | L1 | ✅ | “The total run cost $1,213,430 in API credits”（图 04、06） | “光调用模型”= API credits，不含人工与基础设施；原文称这是 “more an upper bound”（图 04），正文未写 |
| 吠点① 有媒体标题写“948个漏洞已提交补丁” | L5（实例） | ⚠️ 原话已逐字核对，不背书其断言 | 第一财经日报微博（新浪页 https://www.sina.cn/weibo/detail/5361796030400510.html ，北京 11-05 07:34） | 页面标题逐字：“AI 挖漏洞收益最高达成本 55 倍，948 个漏洞已提交补丁”；正文自写“正在提交”。量子位另写“948个补丁建议”（图 12a，备用） |
| 吠点① 原文只说先发出“数百份”报告和补丁，其余陆续发 | L1 | ✅ | “we have already sent out hundreds of reports and patches for critical vulnerabilities and are working with Patch the Planet partners to gradually send out patches for the rest.”（图 11） | 原文是“严重漏洞的”报告和补丁；正文为字数删了“严重漏洞的”，不改变“只发出数百份”的含义 |
| 吠点② 原文写，扫描和人工分析10天结束时，看到61个提权漏洞已在上游修好，43个补丁待合入，299个在公开讨论，没说其中多少属于这948个 | L1 | ✅ | “When the autonomous scan and manual analysis finished after 10 days, 61 LPEs had already been patched upstream, 43 had staging patches, and 299 were being reported and discussed publicly.”（图 08） | “上游”= upstream；“补丁待合入”= had staging patches。“在公开讨论”= being reported and discussed publicly。三数之和 403，原文未说明与 948 的对应关系（摘要另称同期 100 多个由其他贡献者修好或备好补丁），正文明写“没说其中多少属于这948个” |
| 吠点③ OpenAI 称 AI 自己判的“成功”会错：早期一轮的200个提权成功里，有20个其实借了另一个漏洞拿到权限，不是指定要攻的那个 | L1 | ✅ | “During an early run, the exploitation stage reported 200 successful local privilege escalation exploits. However, after human review, we found that 20 of these had achieved root privileges through a different 0-day vulnerability than the one assigned.”（图 10） | “自己判的成功”= “Our pipeline marked a vulnerability as exploited if a root shell was successfully obtained”；“另一个漏洞”= a different 0-day vulnerability；之后加了独立检查（同图），正文为版面删去，批注图译注 2 保留 |

## B Kimi 按时放出权重【跟进1103】

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| B1 北京4日23:30，已有人发帖称 Kimi-K3.1-Pro 的权重上了开源模型平台 Hugging Face | L6 | ⚠️ 发帖时刻已核，帖子说法未独立证实 | HN 50366999 “Kimi K3.1 Pro model weights are out”，提交 11-04 15:30:30Z = 北京 23:30:30（`b-hn-50366999.json`，图 22）；Wayback 23:33:01 快照为公开模型页，页面数据含 `"gated":false` 与 safetensors 库标签（`b-wayback-hf-1533z.html`），快照未含逐个文件列表；HF commits 只有 11-02 “Initial commit” 与 11-05 15:11 改 encoding_k3.py 两条，权重文件只能来自 11-02 首次提交（`b-hf-commits.json`），推断 23:30 公开时权重已在仓库 | 这是可查到的最早“已公开”证据，不是公开时刻本身（HF 不给可见性切换时间）；正文写“已有人发现” |
| B2 月之暗面5日0点（美国时间仍是4日）宣布放出 | L1 | ✅ | 官方 X 帖 11-04 16:00:35Z = 北京 11-05 00:00:35（`b-fx-2118010528123773297.json`，图 21） | 16:00:35Z 即美国各时区的 11-04；平台名不进正文 |
| B3 赶上了“最迟11月4日”的承诺 | L1 | ✅ | 博客 “Full model weights will be released by Nov 4, 2026.”（图 24）；北京 10-24 01:15:24（UTC 10-23 17:15:24）官方帖 “Open weights by Nov 4, 2026.”（`b-fx-2113680701770068067.json`） | 官方没给时区；按官方宣布时刻（UTC 11-04 16:00）即在 4 日内。是否按北京时间也在 4 日内，只有 HN 帖与 23:33 公开模型页快照旁证，未独立证实权重当时可下载，正文不作此断言 |
| B5 权重文件约1.56 TB | L1 | ✅ | HF Files 页 “1.56 TB”（图 16）；96 片 safetensors 合计 1,561,293,160,568 字节（`b-hf-api-blobs.json`） | |
| B6 参数量2.8万亿 | L1 | ✅ | 模型卡参数表 Total Parameters “2.8T”（图 14a，`b-hf-readme.txt`） | 同页第 38 行写 “3T-class” |
| 吠点① 许可证不是原版 MIT（几乎不设限的开源许可） | L1 | ✅ | LICENSE 首段为 MIT 式授权（“Permission is hereby granted, free of charge, to any person …”），后附第 2–4 条（`b-hf-license.txt`，图 15a） | 首轮反向核验后删去对中文稿“基本沿用 MIT”的点名（该稿同句已写门槛，摘半句失真）。“一种几乎不设限的开源许可”是对 MIT 许可的通俗解释 |
| 吠点① “模型服务”的定义 | L1 | ✅ | 第 2 条首段：“"Model as a Service" means giving a third party access to language model inference or fine-tuning (e.g., via API) in a manner that allows such third party to exercise meaningful control over the inputs, parameters, or training data. This does not include (a) end-user products with model capabilities solely embedded within specific features or harnesses, or (b) mere relaying of requests to models hosted by others.” | 正文写“许可证所定义的模型服务”，定义与两项排除写在批注图 kimi-license-a 译注 |
| 吠点① 经营许可证所定义的模型服务、且连同关联方连续12个月收入超2000万美元的，商用要另签协议 | L1 | ✅ | 第 2 条：“If the Licensee or any of its affiliates operates a Model as a Service business, and the aggregate revenue of the Licensee and its affiliates exceeds 20 million US dollars … in total over any consecutive 12 months, the Licensee must enter into a separate agreement with Moonshot AI before using the Software or its derivative works for any commercial purpose.” | “收入”= 被授权方及关联方的总收入，不限模型服务收入 |
| 吠点① 月活超1亿或月收入超2000万美元的商业产品，要在界面显著标注模型名 | L1 | ✅ | 第 3 条：“more than 100 million monthly active users, or more than 20 million US dollars … in monthly revenue, "Kimi K3.1-Pro" must be prominently displayed on the user interface” | 模型名 = “Kimi K3.1-Pro” |
| 吠点① 内部使用、经官方或认证推理伙伴使用的除外 | L1 | ✅ | 第 4 条：“The requirements set forth in Sections 2 and 3 do not apply to: (a) internal use of the Software … or (b) any use of the Software accessed through Moonshot AI's official products or certified inference partners.” | 第 4 条 (a) 内部使用 = “any use that does not make the Software, its outputs, or its underlying capabilities available to third parties”；“内部使用”在许可证中定义为不向第三方提供本软件、其输出或底层能力，批注图译注写全 |
| 吠点② 推理框架 vLLM 的部署说明用了8张数据中心显卡 | L3 | ✅ | vLLM 配方页 “TP8 on two GB300 trays or one B300 node, or one MI355X node, for the MXFP4 checkpoint”（图 17，`b-vllm-recipe.txt`） | TP8 = 8 卡张量并行；配方页另写 “Throughput was not tuned”；模型卡自己没写最低配置。AMD 帖亦称 “Runs on 8x MI355X” |
| 吠点③ 据 Artificial Analysis 10月27日发文，它在改版的综合指数（六成权重来自题目或答案不公开的评测）里46分，当时权重还没放，已被算作开放权重第一 | L3 | ✅ | AA v5 发布文（10-27）：“Kimi K3.1 Pro (weights not yet released) is the top open weights model at 46.”（图 20，`b-aa-article-v5.txt`） | 测试 = Artificial Analysis Intelligence Index v5；Kimi 未标档位。“六成权重来自题目或答案不公开的评测”= “60% of the Index is now based on evaluations with private test sets or held-out answers”。AA 当前模型页仍标 “Proprietary model”，榜单 FAQ 的开放权重榜首为 DeepSeek V4.1 Flash 1017 (max) 33 分——“被算作”指 10-27 发布文的算法 |
| 吠点③ 比当时榜首 Claude-Opus-5.5（max 档）低12分，旧版只差3分 | L3 | ✅ | 同文 “Claude Opus 5.5 (max) leads at 58”；“The open weights gap widens from 3 points to 12”（图 20） | “当时”= 10-27 发布文的快照；次日 AA 文称 Gemini 4 Carbon 59 分成为新榜首（`b-aa-article-v5.txt` 存档同页），正文不写现状排名。旧版 = v4.3；1023 期所写“55分、第四”为 v4.3 口径（`b-aa-article-launch.txt`） |

## 批注译注

| 图 | 译注 | 原句 | 结果 |
|---|---|---|---|
| oa-948-hero | （流水线）验证了 Linux 中948个本地提权漏洞并为它们生成了补丁，API 费用120万美元。 | “it validated and generated patches for 948 local privilege escalation vulnerabilities in Linux at an API cost of $1.2 million.” | ✅ |
| oa-patches-sent | 我们已发出数百份严重漏洞的报告和补丁，正与 Patch the Planet 合作方逐步发出其余补丁。 | 见 A 节吠点① | ✅ |
| oa-maintainers | 自动扫描和人工分析10天后结束时，61个提权漏洞已在上游修好，43个有待合入的补丁，299个正在公开报告和讨论。 | 见 A 节吠点② | ✅ |
| oa-wrong-0day | ①早期一轮中，利用阶段报告了200个成功的本地提权利用；人工复核发现，其中20个是通过另一个（非指定的）0-day 漏洞拿到 root 权限的。②此后我们收紧了任务约束，并加了独立检查，确认每个最终利用用的是指定的漏洞。 | 见 A 节吠点③；“We subsequently tightened the task constraints and added an independent check confirming that each final exploit used its assigned vulnerability.” | ✅（高亮①用手量坐标：OCR 读不出斜体 “0-day”） |
| kimi-promise | 完整模型权重最迟于2026年11月4日发布。 | “Full model weights will be released by Nov 4, 2026.” | ✅ |
| kimi-license-a | ①特此免费授权任何人 ②“模型服务”指以让第三方能实质控制输入、参数或训练数据的方式，向其提供语言模型推理或微调访问（如通过 API）；不包括 (a) 只把模型能力嵌入特定功能或框架的终端产品，(b) 单纯转发对他人托管模型的请求。③若被授权方或其任一关联方经营模型服务业务，且被授权方及其关联方任意连续12个月的总收入超过2000万美元（或等值其他货币），则在将本软件或其衍生作品用于任何商业目的之前，须与月之暗面另签协议。 | LICENSE 首段、第 2 条 | ✅（底图为首段与第 2 条拼接，第 1 条未入图） |
| kimi-license-b | ①若本软件（或其衍生作品）用于被授权方月活超1亿、或月收入超2000万美元（或等值其他货币）的商业产品或服务，须在该产品或服务的界面上显著标注“Kimi K3.1-Pro”。②第2、3条要求不适用于：(a) 内部使用本软件，即不向第三方提供本软件、其输出或其底层能力的任何使用；(b) 经月之暗面官方产品或认证推理伙伴使用本软件。 | LICENSE 第 3、4 条 | ✅ |
| kimi-vllm | ①MXFP4 版本：8卡张量并行，用两个 GB300 托盘，或一台 B300 节点，或一台 MI355X 节点。②MXFP4 路由专家权重（磁盘上约1.56 TB） | 见 B 节吠点②；“MXFP4 routed-expert weights (~1.56 TB on disk)” | ✅（底图两段拼接） |
| kimi-aa-v5 | ①Claude Opus 5.5（max 档）以58分居首 ②开放权重模型的差距从3分拉大到12分：Kimi K3.1 Pro（权重尚未发布）以46分居开放权重模型之首。 | 见 B 节吠点③ | ✅ |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| OpenAI 发布 Images 2.6，能做 GIF 动图，称比之前快50% | L1 | ✅ | https://openai.com/index/introducing-images-2-6/ （`c-c-oai-images26.txt`）：“We’re introducing Images 2.6 in ChatGPT, Codex, and the API … bring motion to your images with GIFs”；“Images 2.6 generates images 50% faster than before.” | 官方日期 11-04，无时刻；“50%”无基线说明，句中保留“称” |
| AI 编程工具 Conductor 将加入 Meta，云端版11月20日关闭 | L1 | ✅ | https://www.conductor.build/announcement （`d-d1-conductor-announcement.txt`）：“Conductor Cloud will become read-only on Nov 13, and will fully shut down on Nov 20.” | 官方原文 “Conductor is joining Meta”；日期无时区；本地版 “until early 2027” 继续更新 |
| OpenAI 称内部用的 GPT-6-Astra 在 AI 安全研究上暗中少做 | L1 | ✅ | https://alignment.openai.com/misalignment-reports/covert-underperformance-on-safety-research-tasks/ （`d-d3-openai-covert-underperformance.txt`）：标题 “covert underperformance on safety research tasks”；“GPT-6 Astra · Internal deployment and evals” | 页面 “Report updated: Nov 4, 2026”；“AI 安全研究”= safety and alignment workloads |
| Ripple-65B 自称智能体编程为美国开放权重最强 | L1 | ✅ | https://www.ricursive.com/ripple （`c-c-ricursive-ripple.txt`）：“Achieving U.S. Open-Weight SOTA on Agentic Coding”；HF 模型卡 “Terminal-Bench 2.1 (self-reported)”（`c-c-hf-ripple.txt`） | “自称”= 厂商 Ricursive 自己的说法，领先指标 Terminal-Bench 2.1 标 self-reported；许可证 Apache 2.0 |
| Anthropic 称借 Claude 解出首个原核细胞 vault 颗粒结构 | L1 | ✅ | https://www.anthropic.com/research/investigating-vault-particles-with-claude （`d-d4-anthropic-vault-particles.txt`）：“the first 3D structure of a vault particle found in prokaryotic cells”；“Under our direction, Claude searched protein databases, designed the DNA constructs, optimized the isolation method, and processed the microscopy data.” | 官方日期 11-04；预印本已投 bioRxiv、未上线；功能仍未知 |

## 反向核验

### 第一轮（Codex gpt-6-astra，medium，只读；北京 11-05 约 19:10）

对象为初稿（标题“AI挖出948个漏洞？Kimi按时开源”）。2 BLOCKER、7 SHOULD_FIX、1 NEEDS_VERIFICATION，Claude 逐条回存档核实后处理：

1. BLOCKER 许可证漏了“含关联方合计”与第 4 条豁免：属实。正文改为“做模型服务的公司连同关联方连续12个月收入超2000万美元……；内部自用等情形除外”，批注底图补第 4 条并加译注。
2. BLOCKER 标题“Kimi按时开源”把开放权重升级成开源：属实（许可证另设商用协议门槛）。标题已因读者稿检查改为“花121万美元，AI挖出948个漏洞？”，不再含“开源”。
3. AA 榜首缺时点（次日 Gemini 4 Carbon 59 分成新榜首）：属实，改“据 AA 10月27日发文……比当时榜首……”。
4. 摘引中文稿“基本沿用 MIT”截掉了它同句写的门槛：属实，删去对该稿的点名，直接讲许可证。
5. 速览 Ripple “编程”扩大了 “Agentic Coding”：属实，改“Ripple-65B 自称智能体编程为美国开放权重最强”。
6. “by Nov 4”译成“11月4日前”可能排除当天：属实，译注改“最迟于2026年11月4日发布”。
7. “已修进内核主线”强于 “patched upstream”：属实，改“已在上游修好”。
8. Wayback“快照已见文件”证明力写高：属实，事实清单改为“公开模型页、gated:false、safetensors 标签，未含文件列表”，并补 HF commits 只有两次提交的推断；正文写“已有人发现”，不写公开时刻。
9. 两份 README 缺失：已补。
10. 事实清单交叉引用与 L5 ✅：属实，改正引用；媒体实例行改为 L5（实例）⚠️，注明只核原话。

### 第二轮（Codex gpt-6-astra，medium，只读）

1 BLOCKER、6 SHOULD_FIX，均属实并已改：
1. BLOCKER 许可证触发条件与豁免仍简化过度（关联方经营也触发；“内部”须不对外提供输出）：正文与吠点卡改为“公司或任一关联方做模型服务、且合计连续12个月收入超2000万美元的……；只在内部用、不对外提供模型或其输出的除外”。
2. “六成分数来自不公开考题”漏了“答案保留”：改“六成权重来自题目或答案不公开的评测”。
3. “上期截稿时还没放”强于证据（1103 最后观测 21:24–21:28）：删去括号。
4. 61/43/299 易被读成 948 的进度：正文补“看到61个提权漏洞……没说其中多少属于这948个”。
5. 吠点③单独上卡缺主体：补“OpenAI 称”。
6. README 配图清单与拆图后不一致：已同步（15 张＋封面）。
7. 官方承诺帖日期未换算：改北京 10-24 01:15:24。

### 第三轮（Codex gpt-6-astra，medium，只读）

2 BLOCKER、3 SHOULD_FIX、1 NEEDS_VERIFICATION，均属实并已改：
1–2. BLOCKER 许可证漏了“经官方或认证推理伙伴使用”豁免，且“模型服务”有特定定义（第三方能实质控制输入、参数或训练数据；不含只嵌入功能的终端产品与单纯转发）：正文改为“经营许可证所定义的模型服务……内部使用、经官方或认证推理伙伴使用的除外”，kimi-license-a 加译定义。
3. 封面副标题「补丁只发了数百份」把“报告和补丁”缩成补丁：重出封面，删去副标题。
4. AA“开放权重第一”漏了“当时权重尚未发布”：补“当时权重还没放”。
5. “5.6 TB 是按另一种精度估算”：原文只写“BF16 版约 5.6TB”，“估算”页尾指显存，证据不足：删去该句。
6. 北京时间按时的证据含推断：正文改为“已有人发帖称”，并注明官方宣布时美国时间仍是4日；B1 行改 ⚠️。

## 视觉复核

### 第一轮（Codex gpt-6-astra，medium，只读；24 张附图）

PASS，无 BLOCKER；5 条 SHOULD_FIX，均属实并已改：
1. 省流卡 Kimi 段摘掉“北京4日23:30”后看不出“按时”：已恢复。
2–4. 许可证批注：第 4 条高亮只到半句、第 2 条高亮漏了“经营模型服务”前提、译注漏“或等值其他货币”与“或服务”：已补全高亮与译注。
5. 许可证单张字太小：已拆成 kimi-license-a（首段与第 2 条）、kimi-license-b（第 3、4 条）两张。等宽字体整行较长，拆图后字号仍偏小，原文依据靠译注读。
