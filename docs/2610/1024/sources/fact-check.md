# 1024 事实清单

取证日期：2026-10-25（北京时间，A 组延续至10/26 00:40）。期号1024按“上期+1”。✅表示已对照所列一手材料，不表示本号完成独立实验；L4/L6/L7 按 EDITORIAL 标 ⚠️，表示“核到有人这样说”，不表示所说内容已证实。证据明细见 `evidence-a.md`～`evidence-d.md`；Claude 已在存档文本中逐字复核下列引句。

## 主帖

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 疑似 Kimi 推广说明要博主别标付费合作，所查官方渠道未见回应。 | L7/L4/L1 | ⚠️ | 见下 | 省流；亦为省流卡 Kimi 条。“疑似”对应 VentureBeat“has not been independently authenticated”；“所查官方渠道”= @Kimi_Moonshot、@KimiDevs、官方博客、Hugging Face。 |
| K3.1-Pro 发布约两小时后，一名海外开发者晒出推广说明截图，称是 Kimi 的：博主发帖须先送审，“不要加付费合作标签”；话术模板写着成本只有 Astra 的1/15。 | L7 | ⚠️ | 官方发布帖 https://x.com/Kimi_Moonshot/status/2113680701770068067 （北京10/24 01:15:24，L1）；爆料帖 https://x.com/elshayib_/status/2113710752741892476 （北京10/24 03:14:48）；Post 4 https://x.com/elshayib_/status/2113903850641842589 （北京10/24 16:02:07）；a-fx-*.json、a-media-HVVouYUW4AAjkex.jpg、a-media-HVYYWGKXsAAhiOc.jpg | 已核：截图确有这些字。截图原句：“Please first send us the draft for review before moving on to posting.”“Prior approval is needed before posting.”“Please DO NOT put on the paid partnership label”。话术：“Kimi creates even better games than Astra, but it's 15x cheaper! $xx vs $xx”（金额为占位符；“15x cheaper”译作“成本只有1/15”）。未核：文件真伪与出处。平台名不进正文。Post 2 图含“5x cheaper”与地缘话术，不用。 |
| 据 VentureBeat 报道，文件未经验证，发出方不明。 | L4 | ⚠️ | https://venturebeat.com/technology/moonshots-kimi-k3-1-pro-flirts-with-open-weights-leadership-amid-paid-influencer-scandal-allegations ; a-venturebeat.txt | Carl Franzen，10/24 20:31 PT（北京10/25 11:31）。“has not been independently authenticated”；不清楚出自 Moonshot、外包机构还是其他方。 |
| 截至北京26日凌晨，Kimi 官方账号与博客未见回应。 | L1 | ✅ | @Kimi_Moonshot、@KimiDevs 时间线（opencli）；https://www.kimi.ai/blog/kimi-k3-1-pro ; Hugging Face moonshotai | A 组查于北京10/26 00:10；Claude 复查于北京10/26 00:52：@Kimi_Moonshot 最新仍为10/24 01:20 发布帖，@KimiDevs 10/25 20:06 三帖为 Kimi Code 子代理与 swarm 模式更新，均未提推广风波。微博、公众号、Discord 需登录未查，正文只写“官方账号与博客”。**发布前须重查。**爆料者称“官方联系他道歉”为单方说法、无截图，未写。 |
| 模板要“必须比较成本”，金额却空着。 | L7 | ⚠️ | a-media-HVYYWGKXsAAhiOc.jpg；images/02 | “*must compare the cost!!!”；“$xx vs $xx”。 |
| Artificial Analysis 综合测试每题成本 K3.1-Pro 2.11美元、GPT-6-Astra 最高档3.26美元，约差1.5倍，单个游戏案例无从核对。 | L3 | ✅ | https://artificialanalysis.ai/models/comparisons/kimi-k3-1-pro-vs-claude-opus-5-5-vs-gpt-6-astra ; a-aa-*.txt；images/05 | Intelligence Index 每任务成本（Cost per Task）。K3.1 $2.11（模型页与对比页；AA 发布文章写 $1.98，两数并存，取当前页）；GPT-6 Astra (Max) $3.26。3.26/2.11=1.545。Opus 5.5 (Max, Default Fallback) $5.98，未写进正文。反向核验 BLOCKER 4：综合测试成本不能反证单个游戏案例，已补“验证不了”。 |
| 网友称 3D 演示调了内置生成技能，版本号与 Tripo 接口吻合，但自认未确认；官方博客没写供应商。 | L6/L1 | ⚠️ | Reddit r/kimi，Altruistic_Compote_9，北京10/24 20:26:15；官方博客 a-kimi-blog.txt | Reddit：“Kimi has a built-in skill called `model3d_generation`”“v3.1, v3.0, and v2.5 … match Tripo AI”“I haven't confirmed the API provider yet”。官方（L1，已核）：“combining them with web search and 3D generation”“The examples below were created end to end by K3.1 Pro”，全文无 Tripo、无供应商。Tripo 官方版本表页未取到。 |
| Apollo 称 AI 对美国非科技企业零回报。 | L1 | ✅ | 见下 | 省流。反向核验 BLOCKER 1：补回“美国企业”。 |
| 二、美国非科技企业，AI 零回报？ | L1 | ✅ | 同下 | 小标题，问句。 |
| Apollo 首席经济学家 Slok 称，AI 给科技业以外的美国企业“零财务回报”：七大科技巨头人均营收、利润率在涨，标普500其余493家持平。 | L1 | ✅ | https://www.apollo.com/wealth/insights-news/insights/daily-spark/no-signs-of-ai-payoff-outside-the-tech-sector ; b-apollo.txt:500；images/13 | Torsten Slok（Partner, Chief Economist），页面日期 October 24, 2026，官方未给时刻。“the technology has delivered zero financial payoff to corporate America outside of tech. Revenue per worker is rising for the Mag 7 but is flat for the S&P 493 … Profit margins are rising for the Mag 7 but are flat for the S&P 493”。“七大科技巨头”= Mag 7；未定义 S&P 493、未说明利润率类型。 |
| 他称主流企业在收缩，该段没给数据。 | L1 | ✅ | b-apollo.txt:500；images/14–16、21、22 | “mainstream corporate America is pulling back and starting to view token spending not as a source of growth but as a cost center that weighs on margins”。Claude 逐张看过五张图：营收/人、利润率、TFP、AI 支出集中度（Top 10% vs Bottom 90%，截面）、VC 背景占比（Top 1% vs 中位，截面），均无“收缩”的时间趋势数据。反向核验 NEEDS_VERIFICATION 10 已补核。 |
| 所引 Ramp 样本里，9月有 AI 付费的美国企业占比同比升12.1个百分点；降的是 AI 占非工资支出比重，较8月少0.19个百分点。 | L1 | ✅ | https://ramp.com/data/ai-index ; b-ramp.txt:38、88、138、146；images/23、24 | “In September 2026, AI adoption reached 55.9% of U.S. businesses on Ramp, up 0.6 percentage points from August and 12.1 points from a year earlier.”；采用定义为当月有 AI 模型、订阅、token、API 付款（免费使用与个人账户可能漏计）；“AI share of business spend (excluding payroll) 5.77% 5.96% -0.19 pts”（Sep/Aug 2026）。样本为 Ramp 上 85,000+ 家企业，“DATA THROUGH SEP 2026”。Apollo 第5张图来源注“Ramp AI Index”。反向核验 SHOULD_FIX 7 已改“有 AI 付费”“较8月”。 |
| 同期热议的“美国创意行业四年丢20万岗位”是9月旧文，作者也说算清 AI 占比“极其困难”。 | L1 | ✅ | https://www.apricitas.io/p/ai-and-the-fall-of-the-creative-class ; b-apricitas.txt:35；b-apricitas-api.json | Substack post_date 2026-09-27T14:01:59Z（北京9/27 22:01）。“America has lost more than 200k jobs in “creative” industries over the last four years”；“Proving the exact amount of AI-driven job loss in the arts is extremely difficult.” HN 帖北京10/25 07:11，439/458。 |
| METR 测 OpenAI 7月的监控，找到绕开的办法。 | L1 | ✅ | 见下 | 省流。“设置”指请求头；“7月的监控系统”= 7/21 及之前的系统。反向核验 BLOCKER 2。 |
| METR 与 Redwood 各派一人，7月进驻 OpenAI 三周测监控。 | L1 | ✅ | https://metr.org/blog/2026-10-22-red-teaming-openai-agent-monitoring/ ; c-metr.txt | “For three weeks, from July 21st to August 11th, a METR employee and a Redwood Research employee conducted a pilot high-access exercise to stress-test OpenAI’s internal misalignment monitoring system.” |
| METR 称，7月审计时的系统里，两个客户端可控的请求头能让新智能体不受监控，但这操作本身可能被抓到。 | L1 | ✅ | 同上；c-metr.txt:88；images/26 | 页面日期 2026-10-22。“Our findings describe the historical system we assessed on and before July 21st, rather than the expanded system’s current coverage, which this exercise did not establish.”“We found two independent client-controllable headers that disable the monitoring system for new agents … could be caught by the monitoring system.” |
| OpenAI 另称前沿训练已改为不联网。 | L1 | ✅ | https://openai.com/index/securing-our-frontier-model-training-environment/ ; c-openai.txt | “conduct frontier model training with no internet access. We’ve implemented this policy”。页面写 October 23, 2026，JSON-LD datePublished 10/22，正文不写日期。9/20 DNS 事件（0926 已发）与10/1恢复未写进正文。 |
| The Decoder 等写“6%到25%未受监控”。 | L4 | ⚠️ | The Decoder（10/23）；evidence-c.md 第95–98行 | The Decoder：“An estimated 6 to 25 percent of internal agentic Responses API traffic … wasn't covered by monitoring at all”。“等”指转载样本 Pivot News（“a quarter”）、Renascence（“最多25%”）、YunNEWS（“6–25%”），只作传播样本。IT之家10/23稿无比例。 |
| 原文是审计时较强模型的内部 Responses API 智能体流量约75%到94%“符合监控条件”，不确定性大，算不出实际漏监比例。 | L1 | ✅ | c-metr.txt:96；images/33 | “roughly 75–94% of agentic internal Responses API … traffic from models at least as strong as GPT-5.5 was eligible for monitoring at the time of the exercise, although we have substantial uncertainty in both directions.” “较强模型”= at least as strong as GPT-5.5。脚注5：eligible 流量也可能未走完监控管线；脚注4：不含内部研究推理服务上的流量。 |
| OpenAI 称影响生产防护的漏洞已修补，METR 写明“未评估这些说法”；一处漏洞细节被 OpenAI 涂黑。 | L1 | ✅ | c-metr.txt；images/28、29 | “Since completing the exercise, OpenAI has stated that all of the vulnerabilities we identified that impact their production safeguards stack have been patched.”“We have not evaluated these claims.”；“hundreds of recent actions … [exploit details redacted by OpenAI]”。 |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 01 ①–④：请先把草稿发给我们审核，再发布。／发布前需事先获批。／请不要加付费合作标签。／帖子里不必附 Kimi.com 链接，也不必标 Kimi 官方账号。 | L7 | ⚠️ | a-media-HVVouYUW4AAjkex.jpg | 截图原文逐字直译；④原文“tagging Kimi X account”，平台名不进译注。真伪未证实，标题已注明。 |
| 02 ①–③：Kimi 做的游戏比 Astra 还好，成本却只有它的1/15！$xx 对 $xx／Kimi K3.1 就是开源版 Opus 5.5／*必须比较成本！！！ | L7 | ⚠️ | a-media-HVYYWGKXsAAhiOc.jpg | 模板话术直译，不代表模型能力已证实。视觉复核 SHOULD_FIX：“便宜15倍”改为“成本只有1/15”。原图自带黄色高亮。 |
| 05：每任务成本（综合指数测试）：Kimi K3.1 Pro 2.11美元；Claude Opus 5.5（Max, Default Fallback）5.98美元；GPT-6 Astra（Max）3.26美元。 | L3 | ✅ | AA 三方对比页；images/05-aa-cost-raw.png | 北京10/26 00:23 渲染。 |
| 13 ①②：这项技术在科技业以外，没给美国企业带来任何财务回报。／美国主流企业正在收缩，开始把 token 开支看作拖累利润率的成本中心，而非增长来源。 | L1 | ✅ | b-apollo.txt:500 | 直译。 |
| 23：2026年9月，Ramp 上的美国企业 AI 采用率达55.9%，比8月高0.6个百分点，比一年前高12.1个百分点。 | L1 | ✅ | b-ramp.txt:38 | 直译；“采用”定义见上（有 AI 付款）。 |
| 24：AI 占企业支出（不含工资）比重：2026年9月5.77%，8月5.96%，降0.19个百分点。 | L1 | ✅ | b-ramp.txt:88 | 直译，列名 Sep 2026 / Aug 2026 / Change。 |
| 26 ①②：我们发现两个相互独立、客户端可控的请求头，能对新智能体关闭监控系统，可能让智能体启动一个不受监控（其余相同）的实例。／不过，设置这些请求头、创建新智能体实例的动作本身可能被监控系统抓到。 | L1 | ✅ | c-metr.txt | 直译；卡片标题注明“7月及以前的监控系统”。 |
| 33：我们估计，审计时，能力至少与 GPT-5.5 相当的模型的内部 Responses API（它支撑 Codex 等）智能体流量，约75%–94%符合监控条件，但上下两个方向都有较大不确定性。 | L1 | ✅ | c-metr.txt:96 | 视觉复核 SHOULD_FIX：“GPT-5.5 及更强”改“能力至少与 GPT-5.5 相当”。 |
| 29 ①②：完成演练后，OpenAI 已表示，我们发现的所有影响其生产防护栈的漏洞均已修补。／我们未评估这些说法。 | L1 | ✅ | c-metr.txt | 直译。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| MiniMax 公开 M3.1-Flash 权重，社区许可证下商用须满足条件 | L1 | ✅ | https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash ; d-d1-hf-*.json、d-d1-hf-license.txt:7 | 【跟进1019】公开、非门控；MiniMax Community License：商用须署名；相关产品或服务年收入超2000万美元须事先书面授权，否则仍须一次性通知（以许可证原文为准）。模型卡“Approximately 428B total; 23B activated per token”；仅一条 Init 提交（北京10/23 11:37），公开时刻未核，不写日期。AA 页仍标 Proprietary（疑缓存），未写。反向核验 SHOULD_FIX 8。 |
| 据巴尔的摩太阳报报道，员工将患者信息传入未获批 AI 工具，医院通知3182人 | L4 | ⚠️ | https://www.baltimoresun.com/2026/10/24/johns-hopkins-bayview-medical-center-data-breach/ ; d-d2-baltimore.txt:332 | “Johns Hopkins Bayview is notifying 3,182 patients after an employee uploaded personal health information to an unapproved public AI tool.” 10/24 2:46 PM EDT（北京10/25 02:46）。付费墙，仅导语；医院通知403未取到。tag“霍普金斯湾景”。 |
| Reservoir 起诉 Anthropic 用853首歌词训练 Claude | L1 | ✅ | 诉状 N.D. Cal. 5:26-cv-12308；d-d3-reservoir-anthropic-complaint.txt | 核的是“诉状这样指控”。Reservoir Media Management, Inc. 与 (UK) Limited，10/21；Exhibit A 853 首作品（未逐项计数）；诉请合计约1.514亿美元，未写。Suno 另案未并入。 |
| 据 Billboard 报道，猫王相关版权方起诉 Anthropic，涉274首 | L4 | ⚠️ | https://www.billboard.com/pro/elvis-presley-estate-sues-suno-anthropic/ ; https://www.hollywoodreporter.com/music/music-industry-news/anthropic-elvis-presley-lawsuit-ai-1236739617/ ; d-d3-billboard.txt、d-d3-hr-elvis.txt | “filed Friday (Oct. 23)”；274 首；原告为 Authentic Brands Group 旗下实体等。诉状未取得。属指控。 |
| 英国设 AI 生物安全实验室，公共资金1亿英镑，企业承诺至多2亿英镑额度等 | L1 | ✅ | https://www.gov.uk/government/news/introducing-the-ai-biosecurity-lab ; d-d5-gov-uk.txt:134、160、162 | gov.uk 24 October 2026（官方未给时刻）。“Backed by £100 million of government funding, and up to £200 million in company commitments from OpenAI and Anthropic”；OpenAI “will provide £100 million in compute and model credits and trusted access”，Anthropic “up to £100 million in model credits, trusted model access and dedicated technical support”——企业部分是额度、访问与支持，非现金，故写“额度等”。“公共资金”= government funding，按风控不写“政府”二字。涉外国政府，Develata 10/26 同意收录。 |

## 反向核验

- 第1轮（WSL Codex gpt-6-astra medium，`barking review --reverse`，报告 reverse-report-2.md）：4 BLOCKER、5 SHOULD_FIX、1 NEEDS_VERIFICATION，Claude 逐条核实均成立，已改：①Apollo 补“美国企业”（正文、省流、省流卡、速览小标题）；②METR 补“7月审计时的系统”（正文、省流、省流卡、卡片标题）；③“官方未回应”收窄为“所查官方渠道／官方账号与博客未见公开回应”；④AA 综合测试成本补“验证不了某个游戏案例”；⑤L4/L6/L7 改 ⚠️、截图标 L7、“有报道”点名 The Decoder；⑥补“批注译注”一节；⑦Ramp 改“有 AI 付费”“较8月”；⑧MiniMax 许可证备注补全；⑨配图说明已补；⑩五张图逐张补核。

- 第2轮（同上，reverse-report-3）：1 BLOCKER、3 SHOULD_FIX、1 NICE_TO_HAVE、2 NEEDS_VERIFICATION，均成立。已改：①METR 比例补回“内部 Responses API”，“不能拿100%去减”改“算不出实际漏监比例”；②Ramp 改“所引 Ramp 样本里”；③“被它涂黑”改“被 OpenAI 涂黑”；④evidence-c 更正 The Decoder 保留了双向不确定性；⑤sources/README 拆分 L6 发言与 L7 附图；⑥10/26 00:52 复查官方账号仍未见回应；⑦英国条风控标记保留，Develata 已同意收录。为守1000字上限，同步压缩：“Redwood Research”简作“Redwood”、去“Kimi-”前缀等，事实不变。

## 视觉复核

- 第1轮（WSL Codex gpt-6-astra medium，`barking review --visual`）：1 BLOCKER、5 SHOULD_FIX，均成立。已改：省流卡与26/33/29标题补历史系统范围（BLOCKER）；省流卡 Apollo 条补“美国企业”；02 译注“便宜15倍”改“成本只有1/15”；33 译注改“能力至少与 GPT-5.5 相当”。已改：24 改用表头条与目标行竖拼的 24-ramp-share-raw.png；26/29 重截含页面标题块，再裁成 26-metr-headers-ctx-raw.png、29-metr-notevaluated-ctx-raw.png。
- 第2轮（同上）：PASS，无 BLOCKER；2 SHOULD_FIX、3 NICE_TO_HAVE。已改：省流卡 Kimi 条改用省流句“疑似 Kimi 推广说明要博主别标付费合作，所查官方渠道未见回应。”（带“疑似”；直接补“文件未经验证”会使省流卡溢出）。未改：24 表格字号偏小（底图宽1400 px，放大须重截更大缩放，译注已给全数字，记为已知限制）；三处断行（工具自动换行，不影响事实）。

## 待办

- 发布前重查：Kimi 官方回应。
- 发布当天如晚于26日凌晨，须再查一次官方账号，必要时改时点。
