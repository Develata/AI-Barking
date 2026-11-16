# 1115 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（Qwen4-Max 与 Hy4 flash）、`evidence-B.md`（uBlock Origin）、`evidence-c.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证：A、B 组 claude-sonnet-5-5，C 组 claude-haiku-5-5，北京 11-16 约 23:58 派出（C 组约 4 分钟交回）。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：千问省六成？腾讯便宜20倍？ | L1 | ✅（问的是厂商说法） | 见 A 节 | 两个问号问的都是厂商自己的话：Qwen 博客 Work360 单任务费用 -58%（人民币）/-62%（美元）；腾讯官方页“Hy4 的 1/20”。反转在吠点①②。读者稿检查后由“千问说省六成，第三方算出两个数”（读者称“两个数”太含糊）改为候选 #2。 |
| 省流：阿里 Qwen4-Max、腾讯 Hy4 flash 同日发布，都主打省钱 | L1 | ✅ | 见 A 节 | 两家官方页均只给日期 2026-11-16。 |
| 省流：两家第三方测出的千问费用，一家降约三分之二、一家没降 | L3 | ✅ | 见 A 节 | AA：(10.02−3.27)/10.02≈67%；Vals：9.33 > 8.73。 |
| 省流：混元的1/20是跟内部测试得分更高的自家模型比 | L1 | ✅ | 见 A 节吠点② | 同图 WorkIndex（腾讯内部评测）51.4 > 42.1。反向核验 BLOCKER 后由“自家更强的模型”改写，补上测试主体与范围。 |
| 省流：uBlock 漏洞据称主要靠 GPT-5.6-Sol 找到，修复跨了三个版本 | L2 + L1 | ✅ | 见 B 节 | “据称”= 研究者自述；三个版本 = 1.75.2、1.75.4、1.76.0（公告时间线）。 |

## A Qwen4-Max 与混元 Hy4 flash

Qwen 博客 https://qwen.ai/blog?id=qwen4-max （opencli 渲染：`a-qwen-blog-zh.rendered.txt`、`a-qwen-blog-en.rendered.txt`；官方只给日期 2026/11/16，IT之家 10:07 首发转述）。Hy4 flash 官方页 https://hunyuan.tencent.com/research/100145?langVersion=zh （`a-hy4flash-zh.html` 与 `.rendered.txt`；英文版 `a-hy4flash-en.*`；只给日期；hy.tencent.ai 链接取不到）。AA 与 Vals 按 L3 用（仅限其自身数据）。Claude 11-17 00:30 前后逐字核对下列原句、图 10、11b 与 html 图表数据。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 阿里11月16日发布 Qwen4-Max，称自家测试中单任务平均费用比上一代降约六成 | L1 | ✅ | `a-qwen-blog-zh.rendered.txt` 第 20 行、`-en` 第 13 行；图 02 | 中文：“在我们 Work360 的评测中，单个任务的平均费用从 26.58 元降至 11.16 元，降低了 58%。”英文：“the average cost per task drops from $4.43 to $1.67 — a 62% reduction.”同一项测试，中文按人民币、英文按美元，正文合写“约六成”；未写推理档位。上一代 = Qwen3.8-Max。IT之家标题“任务成本降低超 60%”与其正文 58% 不一（图 12c，夸大实例）。 |
| 腾讯同日发布 Hy4 flash，常规价每百万 token 输入0.5元、输出2元 | L1 | ✅ | `a-hy4flash-zh.rendered.txt` 第 18–21 行；图 09 | 价格表“Hy4 flash 0.02元（命中缓存）0.5元 2元”，为常规价；TokenHub 11-30 前限时 5 折（图 12e），WorkBuddy 11-30 前免费。 |
| 称在内部测试上单任务成本是自家 Hy4 的1/20 | L1 | ✅ | `a-hy4flash-zh.rendered.txt` 第 37 行；图 10 | “在 WorkIndex 上单任务成本仅为 Hy4 preview 的 1/13、Hy4 的 1/20、DeepSeek-V4.1-Flash-1017（max档位，忙时价格）的 1/8。”WorkIndex = 8 个内部评测集（图 1 注）。官方 X 帖“18x task cost reduction”是对 Haiku 5.5 (high)（`a-fx-2122217770314301469.json`），口径不同，正文未用。 |
| 据第三方 Artificial Analysis 评测，Qwen4-Max 跑其综合指数每个任务平均3.27美元，上一代10.02美元（AA 未标推理档位） | L3 | ✅ | `a-aa-qwen4max.txt` 第 84–86 行；`a-aa-qwen38max.txt` 第 93 行；图 05、07 | “On average, it costs $3.27 per task to evaluate Qwen4 Max on the Intelligence Index.”；“… it costs $10.02 per task to evaluate Qwen3.8 Max (0902) on the Intelligence Index.”指标 = Intelligence Index v5.0.1（9 项评测）每任务加权平均费用（方法页 `a-aa-methodology.txt`）。两页只写 “Reasoning Yes”，未标 max/high（A2-3）；AA 标 Qwen3.8 Max 已弃用。单价 $1.50/$4.50 对 $2.00/$6.00。另：AA 称 Qwen4 Max “very verbose”（127M，中位数 78.6M），上一代 150M。 |
| 吠点①：第三方 Vals 换套任务测千问：新版开 xhigh 档每次9.33美元，上一代8.73美元，没降 | L3 | ✅ | https://www.vals.ai/benchmarks/vals_index （`a-vals-index.txt` 第 135、157、171 行）；图 12h（Claude 北京 11-17 00:3x 用 opencli 展开折叠区后整页截图裁切，1 倍像素比） | 表头 “Rank Model Accuracy Cost / Test Tokens / Test Duration / Test”；第 11 行 “Qwen 4 Max 63.20% $9.33 27.27m 1h05m”；第 18 行 “Qwen 3.8 Max 0902 59.05% $8.73 26.59m 1h46m”。Vals 页未写所用推理档位与计价来源；token 几乎持平而单价下调、费用反升，原因未知，正文只写“没变便宜”不作解释。档位：Vals 的 Qwen 4 Max 模型页 `a-a-vals-qwen4max.txt` 第 423–441 行 “Reasoning Effort: xhigh”“We evaluated it at xhigh reasoning effort, temperature 1.0, top_p 0.95 and top_k 20.”；Qwen 3.8 Max 0902 页（`a-a-vals-qwen38max.txt`）超参数只写 Temperature 0.7、Max Output Tokens 128,000，未写推理档位。先前“AA、Vals 都没标档位”只看了榜单表，Claude 交付前查模型页后更正（advisor 指出）；Qwen 官方也写 Work360 上 Xhigh 档 token 只降约24%。“另一套任务”= Vals Index 是金融、编程、法律、税务的行业测试，AA 是 9 项评测的加权指数（反向核验 SHOULD_FIX 4）。 |
| 吠点②：混元的1/20比的是内部测试得分更高的 Hy4：同图 Hy4 flash 42.1分对51.4分 | L1 | ✅ | `a-hy4flash-zh.html` 第 212 行图表数据；图 10 | 数据：“name: 'Hy4 flash', value: [0.2, 42.1]”“name: 'Hy4', value: [5.7, 51.4]”（WorkIndex，纵轴 Performance，横轴单任务成本元）；英文页同值（美元）。Open Benchmarks 图 48.3 对 59.8（图 10b）。官方文字只写“综合效果上略优于 Hy4 preview”（39.7）。 |
| 吠点③：“接近 Claude-Haiku-5.5”来自85位腾讯内部用户盲测，4分制满意度2.79对2.81，外部模型开 high 档；DeepSeek 3.09分最高 | L1 | ✅ | `a-hy4flash-zh.rendered.txt` 第 51 行；图 11b（盲测表在官方页是图片 alt=“【中】模型盲测结果.png”，`a-hy4flash-zh.html` 中无文字版，图 11b 为 opencli 渲染官方页截图） | “我们邀请了 85 位 WorkBuddy 内部用户在 27 类共 259 个日常工作任务上进行盲测。结果显示，Hy4 flash 的用户满意度接近 Claude Haiku 5.5”；表：DeepSeek-V4.1-Flash-1017 3.09、Hy4 flash 2.79、Claude-Haiku-5.5 2.81、GLM-5.3-Flash 2.53（4分制），注“外部模型均为 high 档”。官方 X 帖写 “Matches Claude Haiku 5.5 (high)”。 |
| 吠点④：Hy4 11月4日称“将于近期开源”，截至17日腾讯 Hugging Face 账号仍只有预览版 | L1 | ✅ | 1103 存档 `docs/2611/1103/sources/a-tencent-hy4-launch-zhcn.txt` 第 277 行；`a-claude-hf-hy4.json`、`a-claude-modelscope-hy4.json`（Claude 北京 11-17 00:25 查）；图 12b | “Hy4 将于近期开源模型权重”；HF author=tencent 搜 Hy4 只有 `tencent/Hy4-preview`、`Hy4-preview-FP8`（08-28）；ModelScope Tencent-Hunyuan 名下同样只有两个 preview；腾讯官方 GitHub 只有 preview（A 组）。交付前再查一次。 |
| 吠点④（续）：Hy4 flash 又写“计划开源” | L1 | ✅ | `a-hy4flash-zh.rendered.txt`；图 12a | “我们计划开源 Hy4 flash 并高频迭代”，无日期、无许可证。 |

未进正文：Qwen 自测 16 项中 4 项最高（ALE 46.7 对 Opus 43.4 等，图 03）；扫描称“token 最多少 50%”博客无此句（实为 Work360 -24%/-35%、Terminal-Bench 4.0 -48%/-49%）；扫描称“编程基准 19.2 对 44.4”官方页无（只见 TokenPost）；Arena 名次（页面被拦，只见转述）。

## B uBlock Origin 通用 XSS

研究者文章 https://iter.ca/post/ubo-uxss/ （`b-iter-post.txt`；作者 Smitty van Bodegom，页面元数据 2026-11-15T10:30:35-0800 = 北京 11-16 02:30，发布后改过，`Last-Modified` 北京 11-16 21:55）按 L2（研究者本人）用；GitHub 安全公告 https://github.com/gorhill/uBlock/security/advisories/GHSA-cmqw-9r2w-6wv2 （`b-ghsa.txt`；API `published_at` 北京 11-16 00:27）按 L1（维护者发布）用。Claude 11-17 已逐字核对下列原句与图 14、18。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 一位研究者11月16日发文称，广告拦截扩展 uBlock Origin 的 Firefox 版有漏洞 | L2 | ✅ | `b-iter-post.txt` 第 28 行；图 13 | “a one-click universal XSS issue with uBlock Origin (the MV2 Firefox version, although a similar but less problematic issue also affected the MV3 version, uBO Lite)” |
| 在有特定过滤规则生效的网站上，用户点一下，网站就能借扩展在任意网站运行代码 | L2 + L1 | ✅ | `b-iter-post.txt` 第 28、52 行；图 13；公告摘要 `b-ghsa.txt` | “a malicious website could trick the extension into adding arbitrary trusted filters … run arbitrary JS in the context of any site”；前提：公告 “if there’s a filter rule with a prevent-xhr or prevent-fetch scriptlet that fulfills the response with a WAR”；作者称默认规则里有一条 `lootdest.*##+js(no-fetch-if, …)`（B 组未独立复核）。反向核验 BLOCKER 后补“特定过滤规则生效”条件。 |
| 研究者写道，找漏洞的大部分工作是 GPT-5.6-Sol 做的，方向和验证靠他自己；演示代码主要由 Claude 写 | L2 | ✅ | `b-iter-post.txt` 第 30 行；图 14 | “GPT-5.6 Sol did most of the hard work of finding the issue. I figured there might be some kind of UXSS from filter injection, so I asked it to take a look, and it found the issue. I then checked and verified the issue it proposed. I wrote all of the prose in this post myself; Claude wrote most of the code for the PoC.” |
| 维护者11月1日收到邮件报告，公告写1.75.2及以下受影响、1.75.4修复 | L1 | ✅ | `b-ghsa.txt` 第 206–212、227 行；图 17、18 | “Vulnerability reported through email on Nov. 1st, 2026.”；“Affected versions 1.75.2 and lower”；“Patched versions 1.75.4”；严重度 High，无 CVSS 向量 |
| Firefox 扩展商店目前是1.76.4 | L1 | ✅ | https://addons.mozilla.org/en-US/firefox/addon/ublock-origin/ （B 组存档与 API）；图 23 | 截于北京 11-17 00:0x；页面 “Version 1.76.4”，API `last_updated` 北京 11-16 01:31 |
| 吠点①：演示里要用户点一个看得见的按钮；把按钮藏起来骗人点那步，作者没实际做 | L2 + L1 | ✅ | `b-iter-post.txt` 第 81 行（图 16）；`b-ghsa.txt` 公告摘要与 PoC 说明 | “I didn’t bother actually implementing this, but malicious websites could overlay stuff …”；公告 “(and the long WAR secret on MV2 Chromium)”、“(firefox only, and takes a few seconds to load)” |
| 吠点②：第一次修复很快被研究者绕过 | L1（公告内嵌邮件） | ✅ | `b-ghsa.txt` 第 238、266 行；图 18、19 | bf69cf7 “Published in 1.75.2, November 2, 2026”；“On 2026-11-02 15:31, [syvb] wrote: You can still get the extension UUID by defining a responseText getter …”（邮件时区未写，故正文不写日期，写“很快”：1.75.2 发布于 UTC 11-02） |
| 吠点②（续）：之后又补了两回 | L1 | ✅ | `b-ghsa.txt` 第 250、256 行（图 18）；`b-rel-1760.txt` 第 259 行（图 21）；1.75.2、1.75.4 release notes（图 22） | 1.76.0 两条：“Validate about to be created filter in element picker”“Prevent war:-related filters for the time being”；1.76.0 release notes 首条 “[security] "Universal XSS from website-driven element picker"”，1.75.2/1.75.4 条目无安全字样。“又补了两回”= 1.75.4（token）与 1.76.0（两项）两个版本；读者稿检查后去掉正文版本号。**不写**“升级到1.75.4仍不安全”（无来源，仅推断）；**不写** 1.76.4 与本漏洞有关（其 commit 指向另一条 Mozilla bug 2091796）。 |
| 吠点②（续）：媒体写的修复版本不一 | L4–L5 | ✅（实例存档） | TSN `b-tsn.txt`（图 24 第二张）；DEV Community `b-devto.txt`；Dr. Web `b-drweb.txt` | TSN：“The first fix shipped in 1.75.4 … The advisory’s brief advice is to update to 1.76.0 or later.”（公告无此建议）；Dr. Web：“Ab Version 1.75.4 ist der Fehler behoben.”；DEV：“check you're on 1.76.0 or later”（作者个人建议）。三家分别写 1.75.4 首修、1.76.0、1.75.4；TSN 所称第一次修复实为 1.75.2。Alto/Gab 摘要“1.76.0”未存档，不作依据。 |
| 吠点③：公告和文章都没提到有人实际利用，也没有 CVE 漏洞编号 | L1 + L2 | ✅ | `b-ghsa.txt` 第 482 行；图 20；NVD 查询（北京 11-16 23:59） | “No known CVE”；NVD 关键词查询无本漏洞；“没提到”≠没发生，措辞写“都没提到”。 |

热度：HN `50513160` 北京 11-17 00:00 为 506 分 / 146 评（`barking heat`），截图时 511 / 148（图 24）。

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 苹果招最多20万名符合条件的美国用户研究健康 AI，称数据不用于训练模型 | L1 | ✅ | https://www.apple.com/newsroom/2026/11/apple-launches-research-study-to-explore-the-use-of-ai-for-health/ （`c-c1-apple.txt`） | 官方只写 November 15, 2026（未给时刻）。第 78 行：“open to up to 200,000 eligible participants age 21 and older across the United States … (iPhone 15 Pro or later) running iOS 26 or later”；第 70 行：“study data will not be used to train AI models”。“符合条件”= 21 岁以上、支持 Apple Intelligence 的 iPhone 15 Pro 及以上且 iOS 26 以上（反向核验 SHOULD_FIX 8）。参与者用的是 OpenEvidence 的医学 AI。TechCrunch 写 “iPhone 15 or later”，与官方不一致；“约12个月”“发第三方云前匿名化”只见 TechCrunch（L4），不进速览。 |
| 欧洲央行副行长称 AI 风险来源增多：融资从股市扩展到债市、敞口集中、网络攻击 | L1 | ✅ | https://www.ecb.europa.eu/press/key/date/2026/html/ecb.sp261116~db374b56ab.en.html （`c-c2-ecb-en.txt`） | 11-16 Euro Finance Week 演讲（官方未给时刻）。原句：“AI-related risks to system-wide stability now stem from a range of additional factors – the expansion of external funding from equity to debt markets, concentration of exposures, quickening AI adoption and potential cyberattacks.” 第 1116 行 5.5 万亿美元是“主要美国科技企业到2030年累计资本开支”的预测（未署出处），Reuters 写成 financing need；速览不写这个数。“扩展到”照 expansion 译（反向核验 SHOULD_FIX 7）。外国官员讲话，Develata 11-17 决定收录。 |
| 美联储理事 Barr 称大银行报告 AI 智能体试图操作客户账户，授权无法核实 | L1 | ✅ | https://www.federalreserve.gov/newsevents/speech/barr20261116a.htm （`c-c3-barr.txt`） | 11-16 米兰演讲（官方未给时刻）。第 637 行：“major banks now report finding AI agents attempting to take actions in retail customer accounts, without the bank being able to verify authenticity, consent, and alignment with customer instructions.” 无次数、金额、银行名单；脚注 1 称个人观点；挤兑句为 may/could 假设。“授权无法核实”对应 consent 与 alignment with customer instructions 无法核实。外国官员讲话，Develata 11-17 决定收录。 |
| 数学博客 Xena 称 Lean 代码有重要信号，但若由 AI 生成，要先做检查 | L1（作者自有博客，观点） | ✅ | https://xenaproject.wordpress.com/2026/11/12/and-the-proof-is-checked-in-lean/ （`c-c6-xena.txt`） | “Posted on November 12, 2026 … by xenaproject”（未给时刻）；第 29 行：“The summary is: there is an important signal that can be extracted from Lean code, but if this code is AI-generated then you need to do some tests first.”；HN `50515058` 北京 11-16 15:38 提交，23:59 为 162 分 / 59 评。观点文，照原文措辞；作者自述曾受 Anthropic 之请查看其费马大定理 Lean 形式化代码（第 59 行）。撤下 ECB、Barr 时补入，两条加回后保留。 |
| 据韩联社报道，韩国税务师考试第一名自称用 AI 作弊并向警方自首，警方调查中 | L4 | ⚠️ | https://www.yna.co.kr/view/MYH20261116020700038 （`c-c5-yna.txt`）；BBC 韩文 `c-c5-bbc.txt`；Korea Herald `c-c5-koreaherald2.txt`；电子新闻 `c-c5-etnews.txt` | 韩联社（北京 11-16 22:56）第 119、129 行：“세무사 시험에서 인공지능으로 부정행위를 했다고 자수한 수석 합격자 A 씨”；“경찰에 자수서를 낸 데 이어 … 서울경찰청 광역범죄수사단에 … 범행 도구도 여러 점 제출”；标题“이르면 내일 소환”。方式为微型摄像头拍题、发给生成式 AI、手机看答案（Korea Herald 2）；本人否认用 AI 眼镜；公团称将告诉并推进取消合格，财经部“检讨”中止资格效力，均未决定；主管机关官网原文未取到。 |

速览不收：华为云 CloudRobo-WM1.2（发布后官方稿未找到；扫描的 98.35% 实为另一模型 CR-VLA-Dual 在 Libero 的成功率，88.98%/81.7% 只见 TPS 转述幻灯）；其余见派工单 C 组“不收”。

## 批注译注

“文中事实”= 卡片译注原文。中文截图的“译注”为原文摘述。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 中文版：Work360 评测中单个任务平均费用 26.58 元降至 11.16 元，降低58%。 | L1 | ✅ | `a-qwen-blog-zh.rendered.txt` 第 20 行；`qwen-cost-raw.png` | “在我们 Work360 的评测中，单个任务的平均费用从 26.58 元降至 11.16 元，降低了 58%。” |
| 英文版：同一项评测，按美元写 $4.43 降至 $1.67，降低62%。 | L1 | ✅ | `a-qwen-blog-en.rendered.txt` 第 13 行；同图 | “In our Work360 evaluation, the average cost per task drops from $4.43 to $1.67 — a 62% reduction.” |
| Qwen4 Max：跑完综合指数平均每个任务3.27美元。 | L3 | ✅ | `a-aa-qwen4max.txt` 第 86 行；`aa-cost-raw.png` | “On average, it costs $3.27 per task to evaluate Qwen4 Max on the Intelligence Index.” |
| 上一代 Qwen3.8 Max (0902)：跑完综合指数平均每个任务10.02美元。 | L3 | ✅ | `a-aa-qwen38max.txt` 第 93 行；同图 | 同句式，$10.02。第二轮反向核验后删去“两页都没标推理档位”（不是原文翻译）。 |
| Qwen 4 Max：准确率63.20%（±1.01），每次测试9.33美元，每次测试2727万 token。 | L3 | ✅ | `a-vals-index.txt` 第 157 行；`vals-q-raw.png` | 表列 Accuracy / Cost / Test / Tokens / Test：“63.20% ±1.01 $9.33 27.27m”。 |
| Qwen 3.8 Max 0902：准确率59.05%（±1.08），每次测试8.73美元，每次测试2659万 token。 | L3 | ✅ | `a-vals-index.txt` 第 171 行；同图 | “59.05% ±1.08 $8.73 26.59m”。 |
| 在 WorkIndex 上单任务成本仅为 Hy4 的1/20；图注：WorkIndex 包含8个内部评测集。 | L1 | ✅ | `a-hy4flash-zh.rendered.txt` 第 37 行与图 1 注；`hy4-workindex-raw.png` | 图注“其中 WorkIndex 包含 8 个内部评测集（PPT 制作……视觉理解）”。 |
| 同一张图纵轴为性能：Hy4 flash 42.1，Hy4 51.4（数值见页面图表数据）。 | L1 | ✅ | `a-hy4flash-zh.html` 第 212 行；同图 | 纵轴标 “Performance”；数据 [0.2, 42.1]、[5.7, 51.4]。图上不显示数值，故注明出处。 |
| 85位 WorkBuddy 内部用户、27类259个日常任务盲测，官方写满意度“接近” Haiku 5.5。 | L1 | ✅ | `a-hy4flash-zh.rendered.txt` 第 51 行；`hy4-blind-raw.png` | 原句见 A 节吠点③。 |
| 4分制：DeepSeek 3.09、Hy4 flash 2.79、Haiku 2.81；外部模型均为 high 档。 | L1 | ✅ | 官方页盲测表图片（alt “【中】模型盲测结果.png”）；`hy4-blind-raw.png`（opencli 渲染官方页截图 11b） | 表头“用户满意度 (4分制)”；行 Deepseek-V4.1-Flash-1017 3.09、Hy4 flash 2.79、Claude-Haiku-5.5 2.81；脚注“外部模型（DeepSeek、Claude Haiku、GLM）均为 high 档”。Claude 11-17 打开图 11b 逐项核对。 |
| 本次搜索 tencent hy4 显示 2 个模型：Hy4-preview-FP8 与 Hy4-preview，均 Aug 28 更新。 | L1 | ✅ | `hf-hy4-raw.png`（图 12b）；`a-hf-tencent-org.txt` 第 127 行 “Updated Aug 28”；`a-claude-hf-hy4.json` | 视觉复核后改为只描述搜索结果，否定结论留在吠点卡。 |
| 找漏洞的大部分苦活是 GPT-5.6 Sol 干的。 | L2 | ✅ | `b-iter-post.txt` 第 30 行；`ubo-ai-raw.png` | “GPT-5.6 Sol did most of the hard work of finding the issue.” |
| 我猜过滤规则注入可能造成某种通用跨站脚本漏洞（UXSS），就让它去看看 | L2 | ✅ | 同上 | “I figured there might be some kind of UXSS from filter injection, so I asked it to take a look” |
| 演示代码大部分是 Claude 写的。 | L2 | ✅ | 同上 | “Claude wrote most of the code for the PoC.” |
| 这一步我没费事真去实现；但恶意网站可以在嵌入的元素选择器上叠一层东西，或把 iframe 透明度设为0让它看不见。 | L2 | ✅ | `b-iter-post.txt` 第 81 行；`ubo-click-raw.png` | “I didn’t bother actually implementing this, but malicious websites could overlay stuff on top of the embedded element picker or make it invisible by setting the iframe opacity to 0.”第二轮反向核验后删去“只要能让用户点一下就能给任意域名加规则”一条（单看丢了过滤规则前提）。 |
| 受影响：1.75.2 及以下；已修复：1.75.4。 | L1 | ✅ | `b-ghsa.txt` 第 206–212 行；`ghsa-fix-raw.png` | “Affected versions 1.75.2 and lower”“Patched versions 1.75.4”。 |
| 同一漏洞下列了四项修复，分别发布于1.75.2、1.75.4和1.76.0（两项）。 | L1 | ✅ | `b-ghsa.txt` 第 238–256 行；同图 | 四条 “Published in …”。 |
| 通过在 XHR 原型上定义 responseText 取值函数，仍然能拿到扩展的 UUID： | L1（公告内嵌邮件） | ✅ | `b-ghsa.txt` 第 264–266 行；`ghsa-reply-raw.png` | “On 2026-11-02 15:31, [syvb] wrote: You can still get the extension UUID by defining a responseText getter on the XHR prototype:”；卡片标题“研究者11月2日来信”，裁掉上方维护者回复“Should be fixed in 1.75.4”，免得读成绕过了 1.75.4（视觉复核）。 |

## 读者稿检查

北京 11-17 00:27–00:28，Codex gpt-6-astra medium，只读、不带本期上下文；报告与逐条采纳见 `style-review.md`。主要改动：标题改 #2“千问省六成？腾讯便宜20倍？”；吠点①写“换套任务”“两家都没标档位”；“他”改“研究者”；uBlock 吠点②去掉版本号；拆掉“由他提示、由他验证”排比。

## 反向核验

第一轮（北京 11-17 00:27–00:31，Codex gpt-6-astra medium，只读，`barking review --reverse`；审初稿正文与事实清单，卡片与两份 README 尚未生成）：不通过，3 个 BLOCKER。
- BLOCKER 1 `cards.toml`、`images/README.md`、`sources/README.md` 缺失 → 已补，第二轮审。
- BLOCKER 2 “跟自家更强的 Hy4 比”丢了测试主体与范围 → 采纳，省流与吠点②改“内部测试得分更高的 Hy4”。
- BLOCKER 3 “用户在恶意网站点一下”丢了触发前提 → 采纳，改“在有特定过滤规则生效的网站上，用户点一下”，清单补公告条件原句。
- SHOULD_FIX 4 AA 与 Vals 任务集、计价口径不同 → 采纳，吠点①写“换套任务”“两家都没标档位”。
- SHOULD_FIX 5 0.5元/2元是常规价、有限时 5 折 → 采纳，加“常规价”。
- SHOULD_FIX 6 “Hugging Face 只有预览版”范围过大 → 采纳，改“截至17日腾讯 Hugging Face 账号仍只有预览版”。
- SHOULD_FIX 7 ECB “转向”应为“扩展到” → 采纳。
- SHOULD_FIX 8 Apple 招募漏设备条件 → 采纳，改“符合条件的美国用户”，清单写全条件。
- SHOULD_FIX 9 `evidence-B.md` 把“Patched 1.75.4”解释为时间线终点 → 记入 `sources/README.md`，以清单为准，取证文件不改。
- NEEDS_VERIFICATION 10 盲测数字在文本存档里找不到 → 已核：官方页该表是图片（alt “【中】模型盲测结果.png”），以 opencli 渲染官方页截图 11b 为据，清单注明。
- NEEDS_VERIFICATION 11 ECB、Barr 两条涉外国官员 → 两条都标“风控待定”，交 Develata 定。

第二轮（北京 11-17 00:39–00:43，同一命令，提示词末尾追加“第二轮短审”说明，审现稿正文、cards.toml、事实清单与两份 README）：1 个 BLOCKER；确认第一轮问题均已落实。
- BLOCKER 1 点击批注第②条“只要网站能让用户点一下，就能给任意域名添加任意可信过滤规则”单看丢了过滤规则前提 → 采纳，删去该条，改为补译“I didn’t bother…”整句的后半（叠层或透明 iframe）。
- SHOULD_FIX 2 正文与吠点卡缺“外部模型 high 档” → 采纳，吠点③补“外部模型开 high 档”。
- SHOULD_FIX 3 译注未逐条进事实清单 → 采纳，新增“批注译注”一节。
- SHOULD_FIX 4 AA 译注混入“两页都没标推理档位” → 采纳，删去。
- NEEDS_VERIFICATION 5 盲测数字 → Claude 打开图 11b 逐项核对（见“批注译注”），视觉复核同时看过该图。

## 视觉复核

北京 11-17 00:42–00:43，Codex gpt-6-astra medium，只读，`barking review --visual`，附 28 张图（两张封面、16 张上传图与 10 张底图）：2 个 BLOCKER。
- BLOCKER 速览第 4、5 条（ECB、Barr）仍标风控待定却已进正式上传集 → 交 Develata 终审决定；不收就删两条并补足 5 条下限。
- BLOCKER `10-hf-hy4` 译注由两个搜索结果推出“没有正式版、也没有 flash” → 采纳，译注只描述搜索结果。
- SHOULD_FIX `06-aa-cost` 译注混入判断 → 已在第二轮反向核验后删去。
- SHOULD_FIX `04-bark-1` ①“两家”指代不清 → 采纳，改“AA、Vals 都没标档位”，并把主语改成“测千问”以免溢出；正文同步。
- SHOULD_FIX `08-hy4-workindex` 高亮①连带盖住“preview 的1/13”、图注“8个内部评测集”未高亮 → 采纳，拆成三处高亮。
- SHOULD_FIX `09-hy4-blind` “4分制”与 high 档脚注未高亮 → 采纳，补高亮。
- SHOULD_FIX `13-ubo-ai` UXSS 译成“跨站脚本漏洞”漏了 universal → 采纳，改“通用跨站脚本漏洞（UXSS）”。
- SHOULD_FIX `14-ubo-click` “这一步”无指代 → 采纳，引文扩到整句并补译后半。
- SHOULD_FIX `16-ghsa-bypass` 上方“Should be fixed in 1.75.4”紧挨研究者来信，易读成绕过 1.75.4 → 采纳，改为 `16-ghsa-reply`：裁掉维护者回复，标题注明“研究者11月2日来信”。
- SHOULD_FIX `07-vals-rows` 无关行太多、看不清 → 部分采纳：改为表头加两行拼接的 `07-vals-q`，译注补 ± 值；源截图是 1 倍像素比，窄屏重截时页面换成移动版排版，取不到更清晰的表，未再放大。
重渲后 Claude 打开 04、07、08、16 核对。

## 交付前修订

北京 11-17 约 00:50，三批已推送后（advisor 复查）：
- 速览撤下 ECB、Barr 两条（风控默认不收、Develata 未答复），补 Xena 博客一条，速览共 5 条。
- 吠点①“AA、Vals 都没标档位”是只看榜单表下的排他结论：Vals 模型页写明 Qwen 4 Max 用 xhigh 档 → 改为“新版开 xhigh 档每次9.33美元”，AA 未标档位移到事件段。
- 派工时刻更正为 11-16 约 23:58。
- 横版封面胸牌放大 4 倍读作「咲点」→ 重出横版一张，胸牌正确（见 `images/README.md` 封面节）。
- 交付前补看 02、04、09、14 成图与两张封面胸牌。
这些改动之后未再做跨模型复核。

北京 11-17 01:48，Develata 决定欧洲央行、美联储 Barr 两条都加回速览；Xena 一条保留，速览共 7 条。加回的两条措辞与第一轮反向核验后一致，未再做跨模型复核。
