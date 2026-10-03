# 1001 配图说明

正文：`../doc_1001_publish.txt`。来源见[来源入口](../sources/README.md)，事实核验见 `../sources/fact-check.md`，取证清单见 `../sources/evidence.md`，一手存档在 `../sources/`。

状态：01–39 于北京时间 2026-10-02 00:3x 至 01:2x 由 Codex（gpt-6-astra）从原站截取或下载（`../sources/capture-log.md`）；其中 08、12、13、15、19、20 为官方原图等比缩放，09、10、28–39 为官方 PDF 原页渲染。40、41、42、43、44 为 Claude 从 15、30、38、32 裁出单页或单区域；45 为 Claude 用 pdftoppm 从 `c-appendix.pdf` 渲染第 7 页；裁切只取区域，不重绘、不改字。

## 正式配图（按上传顺序）

小红书、抖音：封面之后按下表顺序独立上传。公众号：按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `02-argon-rollout-price.png` | Google 博文：发布段与限时首发价句 | 一、事实段、吠点② | 第一段 “rolling out to a set of trusted cyber defenders”；开放给开发者、企业与消费者只写 “as soon as possible”，未给日期；第三段 “introductory price”：每百万 token 输入 $2、输出 $10。 |
| 2 | `07-argon-price-footnote.png` | Google 博文页底脚注 1 | 一、吠点② | 脚注：入门期结束后适用 $4 / $20。 |
| 3 | `40-aa-omniscience-panels.png` | AA 文章图表（从 15 裁出）：AA-Omniscience Accuracy 与 Non-Hallucination Rate | 一、吠点① | 左：答对率（答对 ÷ 全部题目），Gemini 4 Argon（high）50%，GPT-6 Astra（max）63%；右：非幻觉率 = 1 − 幻觉率，Argon 85%（幻觉率 15%）居首。AA 定义：幻觉率 = 错答 ÷（错答 + 部分答对 + 未作答），即没答对的题里错答占比；“综合指数 45 分以上”指 Intelligence Index。 |
| 4 | `08-argon-benchmark-table.png` | Google 博文官方对比表（原图缩放） | 一、吠点③ | 底色标出每行最高值：蓝底为 Argon，灰底为别家（CWE-bench v1 为与 Argon 并列）。别家更高的 5 行：FrontierSWE v2、Terminal-bench 4.0、PostTrainBench、Terminal-Bench Science 0.1、OSWorld-2.0。厂商自家对比，对手只有 Astra、Fable 5.1、Opus 5.5；来源与测试条件不统一：Argon 的 DeepSWE、Terminal-bench 4.0 等为谷歌自测，对手成绩部分取自榜单与系统卡（见备用图 09）。 |
| 5 | `22-openai-observed.png` | OpenAI 报告 What we observed 与 Our assessment of attribution | 二、事实段、吠点② | 7 月 24、25 日 16,000 requests、over 4,000 users；more than 15,000 users 的集群，by July 28 已阻断；归因节：unclear whether all operators … single actor；core cluster … individuals associated with Moonshot AI。 |
| 6 | `24-openai-footnote.png` | OpenAI 报告页底脚注 | 二、吠点① | “These figures describe attempted, not necessarily successful, extractions.” |
| 7 | `41-robots-key-findings.png` | Anthropic 研究 PDF 第 2 页 Key findings（从 30 裁出） | 三、事实段 | 第二条：74% 对应 34% of working hours，“mostly in limited settings”；第四条：0.3% of job tasks 与 40 年。 |
| 8 | `43-robots-majority-rule.png` | Anthropic 研究 PDF 第 6 页（从 32 裁出） | 三、吠点① | 页首：“Task exposure is set by majority rule: the least structured environment in which robots can do at least half of a task’s examples, weighted by time.” |
| 9 | `45-robots-appendix-a4.png` | Anthropic 附录 PDF 第 7 页（`c-appendix.pdf` 渲染） | 三、吠点① | 页首公式：任务内各情形按权重 w 相加，≥0.5 才计入；A.4：“Excluding ratings that rely on related robots decreases the share of exposed physical work from about three-quarters to a half.”（含相关任务的能力迁移推断）。 |
| 10 | `42-robots-cost-70pct.png` | Anthropic 研究 PDF 第 19 页 Figure 8 与正文（从 38 裁出） | 三、事实段、吠点② | 倒数第二段：34%（技术上能做）与 0.3%（成本有竞争力）均以全部工时（all work time）为分母；最后一段：成本需降约 70% 才对 10% of human work 具成本竞争力，年降 3% 约 40 年，并写明新型机器人或制造流程可能改写路径。图注：成本与工资比较，任务按时间与就业加权。 |
| 11 | `44-robots-cost-method.png` | Anthropic 研究 PDF 第 16 页（从 32 裁出） | 三、吠点③ | “We prompt Claude to estimate costs using web search and the list of robots cited for each task.”；“we first ask Claude to estimate how much a human worker typically produces on a task in a year.” |
| 12 | `36-robots-cost-example.png` | Anthropic 研究 PDF 第 17 页 | 三、吠点③ | “these cost estimates are approximate”；打包工例子：机器人组合超 $200 万、替代约 14 人年。 |

平台限制图数时，依次删图 12、8、2。

截图右侧中部的粉色圆形图标是抓取浏览器的翻译插件浮标，不是原站内容，不含账号信息。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `01-argon-title.png`、`06-argon-rollout-soon.png` | Google 博文标题区、结尾 Rolling out soon 段 | 结尾写明开放顺序：先付费 API 与 Google AI Ultra 订阅者 |
| `03-argon-internal-work.png` | Google 博文内部成果段 | 300 TiB 是 “once rolled out”；Zircon 80 万行迁移仍在审计，正文因篇幅未写 |
| `04-argon-output-limit.png`、`05-argon-cyber.png` | 输出上限 64K→1M；“without cyber guardrails” 句 | 对象限受信任防御者与谷歌内部团队 |
| `09-argon-methodology.png`、`10-argon-methodology-osworld.png` | Google 方法页 PDF 第 2、3 页 | DeepSWE v1.1 的 Argon 成绩为自测（mini-swe agent），Astra 取自榜单，Fable 5.1 与 Opus 5.5 取自系统卡；OSWorld 2.0：3 次取最大、offline partial。正文因篇幅未写 |
| `11-aa-title-cost.png`、`12-aa-token-cost.png` | AA 文章标题与成本、token 图 | $1.99 约为 Astra 的六成、Sol-Max 的 2.7 倍；折扣结束后 $3.98；输出 token 约 62k 对 27k |
| `13-aa-agent-benchmarks.png` | AA 文章智能体基准图 | Terminal Bench 4：Argon 57%，AA 的 Opus-5.5 为 60%，与 Google 表中 66.4% 冲突 |
| `14-aa-omniscience-details.png`、`15-aa-omniscience-chart.png` | AA 文章 Omniscience 段落、全部评测图 | 40 的原文段落与完整图 |
| `16-aa-model.png` | AA 模型页 | 多处标 “Not publicly available” |
| `17-aa-agents-table.png`、`18-aa-agents-charts.png` | AA 编码 agent 对比页 | Antigravity CLI + Argon 64 对 Codex + GPT-6.1 Sol 63；成本 $5.84 对 $1.04 |
| `19-argon-deepswe.png`、`20-argon-automationbench.png` | Google 博文单项图 | DeepSWE v1.1、AutomationBench |
| `21-openai-title.png`、`23-openai-partners.png` | OpenAI 报告标题与开篇、What comes next | 开篇 “did not break our encryption, compromise a database…”；Partner-hosted deployments 仍需同等保护。正文因篇幅未写 |
| `25-cyberscoop-title.png`、`26-cnbc-title.png`、`27-register-title.png` | 三家媒体标题区 | CNBC 与 The Register 的“未立即回应”句在正文末段，见存档 |
| `28-research-update-title.png`、`29-research-update-timeline.png` | 研究者 9 月更新 PDF | 9/27 Azure 缓解、9/28 原提取无法复现 |
| `30-robots-title-findings.png`、`31-robots-rubric.png`、`32-robots-method.png` | Anthropic PDF 页 1–2、4–5、6 与 16 | 封面页与 Key findings；E0–E3 定义；Claude 评分方法（41、43 为其中单页裁出） |
| `33-robots-figure2.png` | Anthropic PDF 第 7 页 Figure 2 | 物理任务份额：E0 26.3%、E1 49.8%（专为机器人搭建的环境）、E2 22.0%、E3 1.9%（非结构化环境）；正文因篇幅删去 |
| `34-robots-figure3.png`、`35-robots-capabilities.png` | PDF 页 8–9、15 | 74% 与 34% 的分母；能力阻碍约 70% 的物理任务（与成本降 70% 是两个量） |
| `37-robots-figure7.png` | PDF 页 18 Figure 7 | 五个体力职业的机器人成本；打包工、出租车司机接近或已具竞争力 |
| `38-robots-cost-decline.png`、`39-robots-footnote.png` | PDF 页 19–20、22 | 42 的完整两页；脚注 10 “rely on Claude” |

## 封面

抖音替换封面（2026-10-02）：[00-cover-douyin.png](00-cover-douyin.png)（1086×1448，3:4）。本期在抖音、公众号受限后，抖音版删去第二条（OpenAI 蒸馏报告）并换此封面；Codex CLI（gpt-6-astra）生成 2 张候选，选第 1 张（举牌更干净）。逐字核对（Claude）：“Argon解决幻觉？”“机器人能做74%体力活？”“Argon”“仅限受信任者”“能做，未必划算”“汪！看脚注”“AI 吠点”，无 logo、无真人脸。原封面与正文不变，新封面不用于小红书。

已生成（Codex CLI 内置图像生成，gpt-6-astra，参考图 `../../../../common_images/profile_picture.png`）：[00-cover.png](00-cover.png)（1086×1448，3:4，小红书、抖音）、[00-cover-wide.png](00-cover-wide.png)（1921×819，约 2.35:1，公众号）。上传顺序排在所有配图之前。发布时按平台要求声明 AI 生成。

候选与挑选：竖版、横版各 8 张候选（8 组），选第 8 组（`cover-8`），候选图不入库。累计超出规范的“每种尺寸最多 5 张”（8 组），原因是文字依反向核验意见逐轮改写，而不是画面挑不出：
- 第 1、2 组：机器人格牌上写“能做74%”“划算0.3%”。反向核验指出两个百分比分母不同（体力任务时间 / 全部工时），提示词改写后重生成；画面可用，因文字过时不入库。
- 第 3、4 组：牌子改为“体力任务能做74%”“仅0.3%工时划算”，选了第 4 组；第二轮反向核验指出两个杯子（OpenAI、Kimi）被吸管连通并挂着“1.6万次尝试”，把整个行动的请求量与 Kimi 直接绑定，超出 OpenAI 只把一个核心集群归因于月之暗面相关人员的范围（BLOCKER），并指出 74% 数字牌限定不全，因此作废。
- 第 5 组：改成单个 OpenAI 杯子、吸管通向一群剪影、只圈出一小撮并标“核心簇：月之暗面相关人员”；牌子没写明是 OpenAI 的归因，且“簇”字在小字号下渲染不清，淘汰。
- 第 6 组：牌子加上“OpenAI称”，但“簇”字渲染成“策”（横版）、近似“籁”（竖版），淘汰；随后把正文与封面的术语统一改为更常用的“核心集群”。
- 第 7 组：竖版、横版均无错字，但机器人格价签“全部工时仅0.3%划算”缺美国与研究估计的限定（第三轮反向核验 SHOULD_FIX），改为删去价签。
- 第 8 组：机器人格只留牌子“能做，未必划算”，画面里不再有 74%、0.3% 等数字；竖版、横版放大后无错字；横版居中 819×819 裁切（x 551–1370）内主标题两行、看板娘与对话框完整。

已逐字核对（Claude，竖横版均放大到 4 倍看小字）：主标题“Argon解决幻觉？”“Kimi被点名？”；第一格宝石“Argon”、牌子“仅限受信任者”（人群只有剪影）；第二格杯子“OpenAI”、标签“1.6万次尝试，不一定成功”（OpenAI 报告里的独立统计，挂在杯子上，不与任何品牌连线）、只圈出人群里的一小撮并指向牌子“OpenAI称：核心集群涉及月之暗面相关人员”；第三格大牌“能做，未必划算”（无价签、无数字，74% 与 0.3% 的限定留给正文）；看板娘对话框“汪！看脚注”、圆牌“AI 吠点”。无 logo、无真人脸、无多余文字。封面上的标注沿用历期做法（如 0930 有 8 处），EDITORIAL 里“另加至多一行关键数字或副标题”按标题区理解；已删去风险最大的数字牌与 Kimi 杯子。

本期画风：**报刊社论漫画（三格）**——本期三条都是“口径对脚注”：奶油色旧报纸底、黑色钢笔墨线与交叉排线、只用朱红色一种点缀，报头式标题 + 三个粗边漫画格，呼应“看脚注”。与 0930 工程蓝图、0929 80 年代家电说明书、0928 复古科幻杂志、0927 波普丝网海报区分开（本期无渐变与大色块，只有墨线、留白与一点红）。

主体（做减法，三个 + 看板娘）：
- Argon：红丝绒围栏里一块发光宝石，牌子“仅限受信任者”，围栏外只画人群剪影。
- OpenAI：一个“OpenAI”杯子，几根吸管通向一群剪影，标签“1.6万次尝试，不一定成功”；只把人群里的一小撮圈出来，牌子写“OpenAI称：核心集群涉及月之暗面相关人员”（写明是 OpenAI 的归因）。请求量是 OpenAI 报告里的独立统计，不与任何杯子或品牌直接连线，归因只画到核心集群。
- 机器人：机械臂举着大牌“能做，未必划算”；不画价签、不出现数字（74% 与 0.3% 的限定——美国、研究估计、不同分母——放不进封面，留给正文）。
- 看板娘：举放大镜，对话框“汪！看脚注”。不画任何公司 logo、真人。

### 3:4 竖版提示词

```text
请画一张竖版 3:4 的社交媒体封面，风格为报刊社论漫画：奶油色旧报纸底，黑色钢笔墨线与交叉排线，只用一种朱红色作点缀，整体搞怪有冲击力，一眼能看出是手绘插画，不要照片级写实，不要 3D 渲染。

构图原则：做减法。画面分为上下两部分：上部是报头式标题横幅；下部是三个粗黑边框的漫画格，每格只有一个主体物件，主体周围留出大块奶油色空白，不画多余背景细节。

- 报头标题（画面上部，占约四分之一高度，最大最醒目）：分两行，粗黑体（报刊标题字），黑字配朱红色下划线：“Argon解决幻觉？” / “Kimi被点名？”。
- 第一格（中部，横跨整个宽度，较大）：一圈红丝绒围栏，围栏里立着一块发光的宝石，宝石上写“Argon”；围栏上挂着一块小牌子写“仅限受信任者”；围栏外有一小群被挡在外面的人群，只画黑色剪影，不画面部。
- 第二格（下部左）：一个大杯子，杯身写“OpenAI”；杯口伸出几根吸管，吸管另一端伸向一群只画黑色剪影、不画面部的小人；杯子旁挂着一个小标签，写“1.6万次尝试，不一定成功”。人群里只有一小撮剪影被红色圆圈圈起来，圈旁立一块小牌子，写“OpenAI称：核心集群涉及月之暗面相关人员”（可分成两到三行写，字要清楚）。全格只有这一个杯子，不画第二个杯子。
- 第三格（下部右）：一只机器人手臂，举着一块大牌子写“能做，未必划算”。不画价签，画面里不出现任何数字。
- 看板娘（画面最下方，在漫画格外的奶油色空白里，尺寸中等，不压住漫画格）：我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，一手举放大镜，张嘴大叫，头顶对话框写“汪！看脚注”。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印、品牌名或其他文字。
- 不画任何真实人物，不出现可辨认的人脸。
- 标题和看板娘离四边留足边距，方便裁成 1:1。
```

### 2.35:1 横版提示词

```text
请画一张横版 2.35:1 的公众号头图，风格为报刊社论漫画：奶油色旧报纸底，黑色钢笔墨线与交叉排线，只用一种朱红色作点缀，整体搞怪有冲击力，一眼能看出是手绘插画，不要照片级写实，不要 3D 渲染。

构图原则：做减法。全画面分左、中、右三段，漫画格用粗黑边框，每格只有一个主体物件，主体周围留出大块奶油色空白，不画多余背景细节。

- 中央正方形安全区（宽度约为整幅画宽度的 40%，无边框）：上部是报头式大标题，分两行、居中对齐，粗黑体，黑字配朱红色下划线：“Argon解决幻觉？” / “Kimi被点名？”，最宽一行不超过安全区宽度的 80%；下部是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，一手举放大镜，张嘴大叫，旁边对话框写“汪！看脚注”。标题和看板娘必须完整留在这个安全区内，裁掉左右两段后一个字都不能缺。
- 左段（一个漫画格）：一圈红丝绒围栏，围栏里立着一块发光的宝石，宝石上写“Argon”；围栏上挂着一块小牌子写“仅限受信任者”。
- 右段上格：一个大杯子，杯身写“OpenAI”；杯口伸出几根吸管，吸管另一端伸向一群只画黑色剪影、不画面部的小人；杯子旁挂着一个小标签，写“1.6万次尝试，不一定成功”。人群里只有一小撮剪影被红色圆圈圈起来，圈旁立一块小牌子，写“OpenAI称：核心集群涉及月之暗面相关人员”（可分成两到三行写，字要清楚）。全格只有这一个杯子，不画第二个杯子。
- 右段下格：一只机器人手臂，举着一块大牌子写“能做，未必划算”。不画价签，画面里不出现任何数字。
- 左段、右段的主体都不得伸进中央安全区，与安全区之间留出明显的空隙。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印、品牌名或其他文字。
- 不画任何真实人物，不出现可辨认的人脸。
```
