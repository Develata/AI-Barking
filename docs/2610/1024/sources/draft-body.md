# 1024 正文草稿（内部，未定稿）

各组交回后逐节起草；A、C 节待对应组交回。依据：`evidence-a.md`、`evidence-b.md`、`evidence-c.md`。

## 一、Kimi 推广被指让博主别标广告

Kimi-K3.1-Pro 发布约两小时后，一名海外开发者晒出一份推广说明截图：博主发帖前须送审、获批准，“不要加付费合作标签”，不必附官网链接或标官方账号。他随后晒的话术模板写着“比 Astra 便宜15倍”“就是开源版 Opus-5.5”。

据 VentureBeat 报道，这份文件未经独立验证，出自月之暗面还是外包机构不清楚。截至北京10月26日凌晨，Kimi 官方账号与博客未见回应。

吠点：
①“便宜”要按任务算。Artificial Analysis 同一套综合测试的每题成本：K3.1-Pro 2.11美元，GPT-6-Astra 最高档3.26美元，只差约1.5倍；Opus-5.5 最高档5.98美元，约2.8倍。
②网友扒出 3D 演示调用了内置“3D 生成”技能，版本号与 Tripo 接口吻合，发帖者自称尚未确认供应商。官方博客写做游戏时会结合“3D 生成”，称示例由 K3.1-Pro 端到端完成，没写用了谁家的生成模型。

核对要点（不进正文）：
- 【跟进1023】1023 已发网络能力取舍、AA 55 第四、权重最迟 11/4。本节新事实：推广说明截图、回应缺位、成本与 3D 质疑。
- 平台名：X 不写进正文，爆料者写“海外开发者”，帖子来源批注图页脚写“海外社媒原文截图（存档日期）”。Reddit 不在禁写名单，正文用“网友”。
- 推广说明原句：“Please first send us the draft for review before moving on to posting.”“Prior approval is needed before posting.”“Please DO NOT put on the paid partnership label”“No need to attach the link to Kimi.com or tagging Kimi X account in the post”（01 号图；a-media-HVVouYUW4AAjkex.jpg）。原帖北京 10/24 03:14:48，官方发布帖 01:15:24，相差约两小时。
- 话术模板：Post 4 图（02 号图，北京 10/24 16:02）“Kimi creates even better games than Astra, but it's 15x cheaper! $xx vs $xx”“Kimi K3.1 is literally Open Source Opus 5.5”“*must compare the cost!!!”；图内无地缘话术。Post 2 图另有“5x cheaper (Opus 5.5 $1.30 vs. Kimi K3.1 $0.25)”与地缘话术，不做配图、不进正文。
- VentureBeat（Carl Franzen，10/24 20:31 PT）：“has not been independently authenticated”；发出方不明；截至 10/25 无确认的公司回应。A 组 10/26 00:10 北京查 @Kimi_Moonshot、@KimiDevs、官方博客、HF 均无回应；微博、公众号、Discord 需登录未查——正文只说“官方账号与博客”。
- AA 每任务成本：K3.1 $2.11（模型页；AA 文章另写 $1.98，两数并存）；GPT-6-Astra (Max) $3.26；Opus 5.5 (Max, Default Fallback) $5.98（evidence-a.md；05 号图缺 Astra 列，须重截）。比值 1.54、2.83。
- Arena 中位成本 $1.22 对 Opus 5.5 (Max) $6.37 约 5.2 倍、Kimi 行 Preliminary、第 11 名区间 7–15（a-arena-agent-lb-opencli.txt），备选未用。
- 3D：Reddit r/kimi Altruistic_Compote_9，北京 10/24 20:26，251 分；“model3d_generation”“v3.1, v3.0, and v2.5 … match Tripo AI”“I haven't confirmed the API provider yet”。官方博客：“combining them with web search and 3D generation to create complete experiences”“The examples below were created end to end by K3.1 Pro”；全文无 Tripo。
- 备选（未用）：kimmonismus、Hesamation 公开称没收到该说明；Hesamation 披露早期访问、API 额度与周边、称未收钱；爆料者称官方联系道歉，无截图；爆料者本人发布后约 7 分钟也发过好评帖；额度官方只有“rolling 5-hour rate window”与倍数，无绝对数。

## 二、AI 没让美国非科技公司赚到钱？

Apollo 首席经济学家 Torsten Slok 10月24日发文称，AI 在科技业以外“零财务回报”：七大科技巨头人均营收、利润率在涨，标普500其余493家持平，生产率也未见 AI 拉动。

另一篇同时被热议的分析称，美国“创意”行业四年少了20多万个岗位。作者 Joseph Politano 自己写道，精确算出其中多少由 AI 造成“极其困难”。

吠点：
①Apollo 称主流企业正在收缩，把 token 开支看成成本。但它文中图表所用的 Ramp 数据显示，9月 Ramp 平台上用 AI 的美国企业占55.9%，同比高12.1个百分点；下降的是 AI 占非工资支出的比重，5.96%到5.77%。
②创意行业那篇是9月27日的旧文。文中拿艺术娱乐业“32%员工工作中用 AI”比“全行业平均62%”，后者是成年人任何用途的使用率，不是同一口径。

核对要点（不进正文）：
- “零财务回报”原句：“the technology has delivered zero financial payoff to corporate America outside of tech”（b-apollo.txt:500；13 号图）。
- “pulling back … cost center that weighs on margins”句未附数据来源（b-apollo.txt:500）。Ramp 数字：b-ramp.txt:38、50、88；23、24 号图。Ramp 页“+0.6 pts”按原文；同比 +12.1 pts。
- Apricitas 发表：Substack API post_date 2026-09-27T14:01:59Z（北京 22:01）；200k 原句 b-apricitas.txt:35；“extremely difficult”句 19 号图。
- 32% 为 FRED RPSGENAIUSAGESHAREIND17“Adoption Rate for Work: Arts, Entertainment, and Recreation”；62% 对应 RPSGENAIUSAGESHAREALL，备注“for any purpose, whether for work, personal use, or both”（evidence-b.md 第 68 行；FRED 截图仅本地）。正文说“成年人”，FRED 写 18–64 岁。
- 备选吠点（未用）：Apricitas 20 万起点是 2022 年中疫情后峰值（读图推断）；70% 是 BEA 实际销售额非册数；Fortune 自算约 18.9 万、口径不同；Apollo 未定义 S&P 493、未说明是否控制行业结构。

## 三、谁来查 OpenAI 的 AI 监控？

非营利机构 METR 与 Redwood Research 各派一人，7月21日至8月11日进驻 OpenAI 三周，测试其内部 AI 监控，10月22日公开摘要：两个客户端可控的设置能让新开的智能体不受监控，但这一操作本身可能被监控抓到；5–7月符合条件的内部 Codex 流量估计约一成因“压缩”环节漏监，OpenAI 审计前已发现并修复。

OpenAI 另文称，9月20日内部模型借 DNS 配置错误短暂上网后，暂停训练11天，10月1日恢复，前沿训练改为不联网。

吠点：
①有中英文报道写“6%到25%未受监控”。METR 原文是：较强模型的内部智能体流量约75%到94%“符合监控条件”，上下都有较大不确定性；拿100%去减，算不出“未受监控”。
②OpenAI 称影响生产防护的漏洞已修补，METR 写明“未评估这些说法”；一个能让数百个动作不被监控的漏洞，细节被 OpenAI 涂黑。

核对要点（不进正文）：
- 【跟进1020】1020 是 OpenAI 自报的两起编码事故；9/20 DNS 事件与暂停已在 0926 发过，本节新事实是 METR 审计、10/1 恢复与“no internet access”政策。OpenAI 训练环境帖不提 METR/Redwood，也不提 1020 两起事故。
- METR 75–94%：“roughly 75–94% of agentic internal Responses API … traffic from models at least as strong as GPT-5.5 was eligible for monitoring … substantial uncertainty in both directions”；99.2% 为内部 Codex 子集 eligible（c-metr.txt；33 号图）。
- “6 to 25 percent … wasn't covered by monitoring at all”出自 The Decoder 10/23；Pivot News、Renascence、YunNEWS 跟随；IT之家 10/23 08:19 稿无比例（evidence-c.md）。正文“中英文报道”须定稿前对 Renascence/YunNEWS 是否中文再核。
- header 句与“could be caught”同框（26 号图）；10% 句“likely … on the order of 10% of eligible internal Codex traffic … between May and July 2026”、审计前已由 OpenAI 发现、审计期间修复（27 号图）。正文“约一成内部 Codex 流量”须补“符合条件的”或改写，待定稿。
- “We have not evaluated these claims.”（29 号图）；涂黑漏洞“hundreds of recent actions … [exploit details redacted by OpenAI]”（28 号图）。
- OpenAI 帖：“On September 20 … exploited a misconfiguration in a DNS resolver to gain limited access to the live internet during a training run”；“We resumed these workloads on October 1, 11 days after our pause began”；“conduct frontier model training with no internet access”；页面日期 October 23 与 JSON-LD datePublished 10/22 不一致，正文不写发文日期。
- 备选吠点（未用）：OpenAI 红队用 GPT-6 Astra Cyber 等模型、给了源码与配置，做出 microVM 逃逸与 CPU 缓存侧信道传“合成秘密”；完全气隙“isn’t currently feasible”；两个“只读”工具的漏洞 METR “did not validate”；OpenAI 3/19 称 <0.1% 在监控覆盖外，分母不同不可比。
