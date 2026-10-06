# 1005 D 组取证

仅为取证交接，不是发布正文。抓取使用 `opencli browser 1005-d`；北京时间 2026-10-06，精确抓取时间见各 JSON 的 `captured_bj` 和 capture-log-d.md。官方自述、媒体报道、社区推测分别标级；未进行模型或广告效果实测。

## 逐项清单

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| D1a：Tibo 的 28 天承诺 | L2 | https://x.com/thsottiaux/status/2106845241357824205 | Over the next 28 days, each day we’ll either ship one thing that is a clear improvement and relevant for most codex/work users or ship a full reset. Let the improvements begin. | 见下方截图验收 | 北京 10-05 04:33:43（UTC 10-04 20:33:43）；公开 time.datetime 与原帖 created_at 相符。引用帖，不把它写成每日无条件重置。d-tibo-28-fields.json | 已找到 |
| D1a：被引用的上一帖 | L2 | https://x.com/thsottiaux/status/2106610099720720811 | All right, we’re locking in. Only things being worked on are simplifications, more efficiency for more usage, groundbreaking features or new models. | 见下方截图验收 | 北京 10-04 12:59:21（UTC 10-04 04:59:21）；完整第二段及公开计数在 d-tibo-28-fields.json 的 quoted 字段。派工的 Ok we are locking in 不是逐字原文。 | 已找到 |
| D1b：后续改进与是否已重置 | L2 | https://x.com/thsottiaux/status/2107158998495748264 | We have optimized the default speed to be ~50% faster across GPT-6 Astra and GPT-6.1 Sol through the subscription across all our products and partners using Sign in With ChatGPT (including OpenCode, Pi, Amp, Devin, ...). | — | 北京 10-06 01:20:29（UTC 10-05 17:20:29）；Day 1 承诺两小时内感受到，无需用户调整；这是员工提速宣告，不是本组实测，更不是已发生重置。完整长推 d-tibo-day1-note.json。限定搜索内未找到新的实际重置公告，不能证明不存在。 | 部分支持 |
| D1c：banked reset 适用与到期 | L1 | https://help.openai.com/en/articles/20001498-how-banked-codex-resets-work | Eligibility, affected usage limits, delivery timing, and expiration vary by offer, plan, workspace, and region. Future resets are not guaranteed. | ../images/d-25-banked-expiration.png | 促销实例涉及 Plus/Pro/Business，不能泛化所有未来活动。到期以账户或 offer 显示为准；过期不能补发。需用户点用，global 自动到账。d-banked.txt，Eligibility and expiration 节。 | 已找到 |
| D1c：付费重置 | L1 | https://help.openai.com/en/articles/20001507-paid-weekly-work-and-codex-rate-limit-resets | Buying a reset is available to eligible ChatGPT Plus and Pro personal accounts on ChatGPT web and the Codex desktop app. It is not available on Free, Go, Business, Enterprise, or Edu plans. | ../images/d-26-paid-eligibility.png | 即时生效，不能储存；不是增加独立余额。新的周周期自重置后的第一次 Work/Codex 请求算起，七天后周重置，不一定付款七天后。d-paid-reset.txt，Overview/Availability/FAQ。页面只给相对更新时间。 | 已找到 |
| D1d：中文是否报道为每日重置 | L4 | https://www.ithome.com/1/009/761.htm ; https://www.qbitai.com/2026/10/501700.html | 他表示，OpenAI 接下来每天都会发布一项“对大多数 Codex / Work 用户而言有明显改善且具备实际意义”的功能更新，否则就提供一次“重置”。<br>要么交付一项对大多数Codex/Work用户明显有用的改进，要么来一次完整的额度重置。 | — | IT之家北京 10-05 09:47:27；量子位北京 10-05 10:50:46。两篇均保留条件，未找到这两篇写成无条件每日重置。英文社区样本见社区节，不把戏谑自动判成事实误读。 | 与说法不符 |
| D1e：10/30 与 20×→10× | L1/L2 | https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers ; https://x.com/thsottiaux/status/2104823812042940713 | Eligible customers can use the previous included allowance through October 29, 2026 while their Pro 200 subscription is active.<br>After that date, your subscription will move to the lower included usage allowance.<br>it will net out at half the dollar in API spend compared to the old Pro $200 plan. | — | 官方支持保留至 10/29、之后下调；10/30 是按日历推导，未给切换时区/时刻。Tibo 的 half 指 API 标价折算，不是明确 Plus 倍率；不能拿一半自行证明 20×→10×。该数对仍未找到指定一手支持。完整 d-pro-tiers.txt、d-tibo-pro-note.json。 | 部分支持 |
| D1f：Claude 9/22 reset 与期限 | L1 | https://x.com/ClaudeDevs/status/2102438800836489554 ; https://x.com/ClaudeDevs/status/2102438803013333469 | Pro, Max, and Team users get a reset to use anytime<br>If you're on Pro, Max, or Team, your reset is available today in Settings → Usage. Apply it any time until Oct 22. | — | 原 UTC 09-22 16:44:06，即北京 09-23 00:44:06；9/22 是原时区日期。到期只写 Oct 22，未给时区。9/28 官方又写 before Oct 22。d-claude-reset-fields.json、d-x-claude-search.json。 | 已找到 |
| D1f：9/22 以来无新限额动向 | L1 | https://x.com/claudeai/status/2105721630051692804 | For two weeks, start a design, deck, or doc in the Claude app, and the work that follows in that conversation uses 50% less of your usage limits. | — | 北京 10-02 02:08:53（UTC 10-01 18:08:53）。找到限额优惠，不能说毫无新动向；它不是新 banked reset。1004 已收，不在本轮重写正文。有界搜索未找到此后新的重置。 | 与说法不符 |
| D2：参数与开放时间 | L1 | https://reflection.ai/blog/introducing-beam | Beam is a sparse Mixture-of-Experts model with 501 billion total parameters, 23 billion active, built for coding, reasoning, and agentic workloads.<br>We will release the weights, technical report, model card, and developer artifacts later this month. | 见截图验收 | 官方 Oct 5, 2026，未给时刻；仍在 red-teaming/evaluations，early access signup。不能写成权重已公开。d-beam.txt 开头。 | 已找到 |
| D2：许可 | L1 | https://reflection.ai/blog/introducing-beam | This month, we will release the weights under an Apache 2.0 license, along with documentation and the full stack for running, evaluating, and fine-tuning the model. | 见截图验收 | The Path Ahead 节；未来式许可承诺，不是本组已下载核验许可证。 | 已找到 |
| D2：3–4× 与估算口径 | L1 | https://reflection.ai/blog/introducing-beam | On advanced reasoning benchmarks, it achieves scores comparable to GLM-5.2 while using 3–4× less inference compute.<br>These estimates exclude prompt prefill, context-dependent attention operations, and serving overhead, so they represent an approximate compute comparison rather than measured inference cost. | 见截图验收 | Figure 2：FLOPs ≈ 2 × active parameter count × mean generated tokens per attempt；生成 token 含 reasoning+final。使用 AA/DataCurve 数据；不等同全栈价格或实测成本。 | 已找到 |
| D2：能力落后与整表 | L1 | https://reflection.ai/blog/introducing-beam | Where frontier open models like Kimi K3 remain ahead on raw capability, Beam's advantage is efficiency at inference time.<br>NR denotes scores that have not been reported. | 见截图验收 | 全部 4 个 tab 的 table HTML 在 d-beam-layout.json。Beam/Kimi K3：DeepSWE 44.4/68.0，Terminal 2.1 80.1/88.3，HLE no tools 36.2/46.9；还有更高分的其他模型，不能只挑弱者。原文自报。 | 已找到 |
| D2：媒体跟进与独立复现 | L4 | https://techcrunch.com/2026/10/05/reflection-debuts-beam-a-open-weight-ai-model-to-rival-chinese-models-at-lower-compute-cost/ ; https://www.reuters.com/technology/nvidia-backed-reflection-unveils-first-ai-model-take-chinese-open-models-2026-10-05/ | Reflection’s performance claims haven’t been independently verified<br>Reflection said Beam is competitive with Chinese AI startup Z.ai's GLM‑5.2 and is closing in on Qwen3.8‑Max on coding and agentic tasks. | — | TC 北京 10-06 03:33（原10-05 12:33 PDT）；Reuters 北京10-06 04:50:01.408（原UTC20:50:01.408），可见4:50 PM EDT。Reuters 明确 said，未见它自己复现。原文软空格见各 txt。 | 已找到 |
| D3：视觉广告位置与上线范围 | L1 | https://openai.com/en-US/index/new-chatgpt-ads-format-and-measurement/ | Initially, we’ll test this new ad format during image generation in ChatGPT. Ads will be clearly labeled, and remain separate from the image being created.<br>Testing will begin later this month in the US with an initial group of advertisers. | ../images/d-29-ads-context.png | 官方 10-05，仅日期。广告和生成图分离；本月晚些美国首批试点，不能写已全球上线或把广告嵌入生成图。d-ads-new-en.txt:L40–42。 | 已找到 |
| D3：周触达与商业数字 | L1（合作伙伴结果由官方转述） | https://openai.com/en-US/index/new-chatgpt-ads-format-and-measurement/ | ChatGPT reaches 1.2 billion people each week. | — | 同页报告 WW CPA低15.3%（DVR/Rockerbox）、Dose增量购买67%来自净新客户（WorkMagic）、Portland Leather访客93%为新访客（Triple Whale）。这些是测量伙伴归因结果，不是本组独立审计，也不是尚未开测的新视觉广告成效。“reaches”不擅改精确定义为活跃账户数。完整原句见下方原文定位。 | 已找到 |
| D3：套餐层级是否改变 | L1 | https://openai.com/en-US/index/testing-ads-in-chatgpt/ | The test will be for logged-in adult users on the Free and Go subscription tiers. Plus, Pro, Business, Enterprise, and Education tiers will not have ads. | — | 旧页原发02-09、更新08-11，仅日期；新公告未宣告改变套餐范围。旧广告项目已拓展多国，不能把新视觉试点美国范围说成整个广告项目只在美国。 | 已找到 |
| D3：匹配使用聊天，广告主看不到聊天 | L1 | https://openai.com/en-US/index/testing-ads-in-chatgpt/ | During the test, we decide which ad to show by matching ads submitted by advertisers with the topic of your conversation, your past chats, and past interactions with ads.<br>Advertisers do not have access to your chats, chat history, memories, or personal details. Advertisers only receive aggregate information about how their ads perform such as number of views or clicks. | — | Answer independence / Conversation privacy 两节；不把“广告主看不到”改写成“广告选择不使用聊天”。 | 已找到 |
| D4：记者记录 15 位以上署名 | L4 | https://www.niemanlab.org/2026/10/chatgpt-is-adding-real-cartoonists-signatures-to-fake-new-yorker-cartoons/ | In all, I documented more than 15 New Yorker cartoonists whose signatures have been used by OpenAI’s image generator without permission or compensation. | ../images/d-30-nieman-tests-response.png | Andrew Deck 亲自测试并收集网络样例的报道；不是本组独立复现，也不必然意味着所有15位都在记者自测中出现。d-nieman.txt:L30。 | 已找到 |
| D4：OpenAI 声明与未修尽 | L4（媒体取得的公司声明） | https://www.niemanlab.org/2026/10/chatgpt-is-adding-real-cartoonists-signatures-to-fake-new-yorker-cartoons/ | We believe the future of creativity is one that is fundamentally human, and our focus is on building tools that empower human creativity and creators,<br>We very much appreciate the community flagging bugs and unintended behavior by our models, so we can address them.<br>Still, as of the publication of this story, it continues to sign some of the generic cartoons it generates with the names of real New Yorker cartoonists. | ../images/d-30-nieman-tests-response.png ; ../images/d-31-nieman-guardrail.png | d-nieman.txt:L32–34。通知后出现 guardrail 提示，但发稿时仍未修尽；不能说完全没行动，也不能说已完全修好。声明本轮只在原报道找到，不升格为独立官方页面。 | 已找到 |
| D4：Condé Nast 授权与漫画训练 | L4 | https://www.niemanlab.org/2026/10/chatgpt-is-adding-real-cartoonists-signatures-to-fake-new-yorker-cartoons/ | a spokesperson for The New Yorker told me Condé Nast has never granted an LLM developer permission to train models on its cartoons. | ../images/d-32-nieman-license.png | 2024协议条款不公开；记者还称看过数份标准漫画家合同，不允许授权AI训练。未取得协议/训练数据，不能据此断定具体训练来源。d-nieman.txt:L46、54。 | 已找到 |
| D4：法律限定与买卖证据 | L4 | https://www.niemanlab.org/2026/10/chatgpt-is-adding-real-cartoonists-signatures-to-fake-new-yorker-cartoons/ | attribution is a very small part of the fair use inquiry<br>ChatGPT isn’t usually reproducing any specific cartoon in these examples; rather, it’s mimicking a more general style.<br>While I found no evidence that these AI-generated cartoons are being bought or sold, there are signs that image generators are already displacing the work of professional cartoonists. | ../images/d-33-nieman-law.png ; ../images/d-34-nieman-sales-watermark.png | Grimmelmann 的采访意见，不是判决。本组不提供独立法律结论；right of publicity 在报道中另有商业用途证据门槛。d-nieman.txt:L79–90。 | 已找到 |
| D4：Baltimore Sun 水印 | L4 | https://www.niemanlab.org/2026/10/chatgpt-is-adding-real-cartoonists-signatures-to-fake-new-yorker-cartoons/ | A quick check with OpenAI’s watermark identification tool shows the illustrations were created with OpenAI’s products. | 文字存档 | 原文是记者叙述自己的 check，不是 Baltimore Sun 宣布自查。本组未取得验证回执，未重做。d-nieman.txt:L92。 | 已找到 |
| D4：独立复现或后续更正 | L4/L5检索线索 | https://www.avclub.com/ai-new-yorker-cartoons-stand-up-comics | A recent report from Nieman Labs reveals that the signatures of real New Yorker artists are being slapped on AI-generated New Yorker cartoons without their knowledge or consent. | — | A.V. Club是跟随报道，不是独立复现。有界搜索未找到新独立复现、官方更正；未找到不等于不存在。该页面还存在错误转引，见夸大实例。 | 未找到一手来源 |
| D4：病毒帖/Reddit可见性 | L6 | https://x.com/woofknight/status/2092798998608331240 ; https://www.reddit.com/r/ChatGPT/comments/1u6gbv3/i_asked_chatgpt_to_make_a_new_yorker_style/ | 只记 URL 与状态，不摘人物资料、不保存漫画素材 | — | 北京10-06 09:40:13与09:40:31，两链接均存在公开帖子元素，无不可用提示；d-viral-x-status.json、d-viral-reddit-status.json。仅确认本浏览器可见，不保证所有地区/未登录环境可见。 | 已找到 |

## 时间与互动数

- Tibo 28天帖：北京10-05 04:33:43（UTC10-04 20:33:43）；页面浏览器时区 America/New_York，4:33 PM 是 EDT。引用的前帖北京10-04 12:59:21（UTC04:59:21）。两段原文、关系与完整公开计数在 `d-tibo-28-fields.json`。
- 北京10-06 09:15:41 抓到：浏览6,612,469、回复4,422、页面转帖3,600、赞25,537、书签2,545。09:19:30 再抓：浏览6,615,730、回复4,422、赞25,539、书签2,545；公开组件 retweet_count=1,456，页面汇总转帖仍3,600。两个字段口径不同，不能自行等同或相减解释；抓取动态也不得当成矛盾。扫描658.3万/4421/3597/约2.5万/2543是更早快照，未重现那个时点。
- Tibo Pro帖：北京09-29 14:41:17（UTC09-29 06:41:17）。Day1帖：北京10-06 01:20:29（UTC10-05 17:20:29）。前帖、Day1、Pro长推的原文来源见相应 `*-note.json`；初次 DOM 自动翻译不作为英文逐字引文。
- Claude官方9/22主帖与期限回复：北京09-23 00:44:06（UTC09-22 16:44:06）。9/28再次提醒：https://x.com/ClaudeDevs/status/2104641323198472430 ，北京09-29 02:36:08（UTC09-28 18:36:08）。到期日未标时区，不造一个UTC零点。
- Beam与新广告官方页：2026-10-05，官方未给时刻；不能换算为一个精确北京时间。广告旧页原发2026-02-09、更新2026-08-11，同样仅日期。帮助页相对 Updated 不视为精确发布时刻。
- TC：北京10-06 03:33（原10-05 12:33 PM PDT）。Reuters：北京10-06 04:50:01.408（原UTC10-05 20:50:01.408），元数据更新北京时间04:54:09.333；不是两个发布时间。
- Nieman：可见 `Oct. 5, 2026, 4:01 p.m.`，未给时区；元数据只日期。因此北京时间未核实。若假设美东EDT，则是北京10-06 04:01，但只能标推算，不能写作已核实发布时刻。
- IT之家：北京10-05 09:47:27；量子位：北京10-05 10:50:46。A.V. Club：北京10-06 07:24:42（JSON-LD UTC10-05 23:24:42）。

## 可能的吠点

1. Tibo 的 `either ... or ...` 不保证每天赠送额度，也未在该帖定义 full reset 的套餐、地区、banked/global 类型；帮助页的历史促销范围不能直接套给这28天承诺。
2. banked reset 会改原周重置日期；付费即时重置提前取用正常周额度，不是额外叠加一周余额。出处：两份帮助页的 What happens / Overview / FAQ。
3. 10/30是日历推导，20×→10×缺指定官方数对；“API标价金额减半”不等于每种模型、任务、可完成工作一律减半。
4. Beam正文虽称 open-weight，权重仍待月底；官方明确还有更强的raw capability模型。效率图排除prefill、attention、serving，不能写成实测成本便宜3–4倍。
5. 图像广告与生成图分离，并未宣布付费套餐加广告；合作伙伴成效数字不是尚未上线的新格式的实验效果。
6. Nieman不是“15张抄袭作品”或法庭侵权裁决；记者记录的是15位以上署名，且未找到这些图买卖证据。OpenAI已有guardrail变化，但记者发稿时仍能得到部分真名署名。

## 社区线索（L6，不作事实确认）

- Beam HN https://news.ycombinator.com/item?id=49969183 ：北京10-06 09:20:11，300 points / 77 comments；扫描293/76为旧快照。默认讨论第一条 Ariarule 提出地图任务至少2025年8月已出现，并附 LessWrong 链接。评论 https://news.ycombinator.com/item?id=49969374 。公开页面不显示该评论得分，记“不可取得”，不能用帖子300分冒充评论得分。
- 此评论引述官方曾写 `This puzzle is a few days old`，但本组当前官方存档未见这句。可核实“评论者提出此质疑”；不能据此单独确认曾经页面文字或编辑历史，更不能推导训练泄漏已证实。
- Nieman HN https://news.ycombinator.com/item?id=49971846 ：北京10-06 09:20:19，198 points / 92 comments；扫描169/67不是本轮数值。评论细节见后续补查，不把HN帖子分数冒充评论赞数。
- Reddit/Tibo误读定向入口：https://www.reddit.com/r/codex/comments/1wxq5ng/new_tibo_post/ 与 https://www.reddit.com/r/codex/comments/1wyehd5/day_1_tibo/ 。搜索命中不等于已核正文，状态另记。没有把“希望不改进只重置”的玩笑自动判成媒体错误。

## 夸大说法实例

已找到英文实例：A.V. Club，Matt Schimkowitz，https://www.avclub.com/ai-new-yorker-cartoons-stand-up-comics ，北京10-06 07:24:42（UTC10-05 23:24:42）。原句见 `d-avclub.txt`：将 `This prompt may violate our guardrails concerning similarity to third-party content` 写成 OpenAI 告诉 Nieman 的话，随后称 `but has done nothing about it.`。Nieman原报道将前者明确归给ChatGPT拒绝提示，而且通知后出现新提示，所以“什么也没做”与原报道不符。本组没有把有防护写成问题已修好。

中文每日无条件重置实例：本轮两篇原站样本中**未找到**。IT之家“否则”、量子位“要么…要么…”均未省略条件。新智元/机器之心转载搜索结果只作定位线索，未用转载替代原站；不为了填此栏判它们为夸大。

## 扫描说法勘误

| 扫描说法 | 复核 |
|---|---|
| Tibo时间不确定，20:33 UTC | time.datetime已确认UTC20:33:43，北京04:33:43；原页面4:33 PM为浏览器美东时区。 |
| 引用前帖的 Ok we are locking in 等英文 | 非逐字，应以 `All right, we’re locking in...` 为准。 |
| 互动数 | 本轮更晚且动态；组件 retweet_count 与页面汇总另有口径差，全部保留。 |
| 中文报道“每天重置” | 两篇原站均保留改进或重置条件；未找到目标误写。 |
| 10/30、20×→10× | 官方支持through10/29+after that；未明确时区和这对Plus倍率，Tibo仅API标价金额减半。 |
| Claude没有新动向 | 未找到新reset，但找到10/1特定对话两周50%用量优惠，不能概括毫无限额更新。 |
| Beam Apache2.0与later this month | 均证实，但是未来承诺；不可写现已开源可下载。 |
| Beam三个落后分数 | 三对数值吻合；另有模型分数更高，须保留完整表。 |
| 图像广告层级/时间 | 新格式本月晚些美国首批；图像分离；旧页Free/Go，未见新页改层级。 |
| Nieman时间 | 日期与4:01p.m.可见，但时区未给，北京04:01仅条件推算。 |
| OpenAI声明 | 原报道中是human creativity与感谢bug反馈；guardrail提示不是发言人声明。 |
| Baltimore Sun用水印工具查出 | 应写记者报道中通过工具检查Sun图片；并非Sun自查公告。 |

## 假设与边界

不从没有搜索结果推导没有事件；两次X限定搜索覆盖了返回卡片，不是完整时间线归档。使用公开帖子字段提取原文，不保存抓取者账户菜单、头像、Cookie或私有价格弹窗。帮助页取公开英文正文；广告明确使用en-US原文。所有截图仅原站渲染，未改文字数字，漫画作品不作配图。图片与JSON/文本的最终清单、成功/失败、尺寸及QA见 capture-log-d.md。

## 补充原文与社区核对

D1a 前帖第二段原文：`Sometimes you have to invest ahead of the curve, but feedback is clear that you all want things to get simpler. On it.` 09:40:31直接读取前帖，`is_quote_status=false`，未见回复父帖字段；赞14,816、回复1,981、书签914、浏览2,615,817；DOM转帖602、组件retweet_count354，分别记录。完整见 `d-tibo-prior.json`。

D1c 到期原句：`Check the expiration shown in your account or offer. Once an unused reset expires, it cannot be restored or reissued.`（d-banked.txt，Eligibility and expiration）。这不是统一固定到期日。

D3 商业测量完整段落（d-ads-new-en.txt:L54）：`Early partner findings illustrate strong performance across different measurement approaches. According to DV Rockerbox, WeightWatchers’ attributed cost per acquisition on ChatGPT Ads was 15.3% lower than its blended paid-search benchmark. WorkMagic reported statistically significant lift for wellness brand Dose, with 67% of incremental purchases coming from net-new customers. And according to Triple Whale, 93% of Portland Leather’s visitors from ChatGPT Ads were new.` 对比基准是blended paid-search，不是所有渠道或所有广告主；67%的分母是增量购买，93%的分母是该来源访客。

Nieman HN补查：`d-hn-nieman-comments.json` 北京09:40:04，公开评论不提供分数，故不能标“高赞已核实”。默认排序靠前的有据质疑包括：

- https://news.ycombinator.com/item?id=49972876 ，gruez区分风格、身份/商标与谁发布给第三人的责任；分数未公开。仅L6法律讨论，不采纳为法律定论。
- https://news.ycombinator.com/item?id=49973052 ，colechristensen反驳百科记录署名与把署名放到仿作上不是同一行为；分数未公开。可提示编辑避免把问题简化成风格版权，但“fraud”仍是网友判断，不能直接写事实。
- https://news.ycombinator.com/item?id=49973102 ，Isamu将假署名解释为视觉模式与语义理解未打通；分数未公开。这是机制猜测，没有实验或模型内部证据。

Beam HN补查：`d-hn-beam-comments.json` 北京09:39:59，https://news.ycombinator.com/item?id=49970567 追问泛化评测是否禁用联网/工具；分数未公开。该问题可要求测试条件，不能据此断言模型用了联网或训练污染。

Reddit补查：`d-tibo-reddit-community.json`，北京10-06 09:47:18，原帖909分，发帖北京10-05 04:46:55.837（原UTC10-04 20:46:55.837）。取可见30条评论，未拿自动生成的置顶总结当事实。

- https://www.reddit.com/r/codex/comments/1wxq5ng/comment/pdvwfog/ ，99分，质疑谁定义 `clear and relevant improvement`，认为这会诱发用户争论以争取重置；属有依据的激励结构质疑，不是实际效果验证。
- https://www.reddit.com/r/codex/comments/1wxq5ng/comment/pdw1vvr/ ，91分，担心每天交付压力导致小功能仓促发布，并明确区分改进与重置，不能把它列为“每日无条件重置”的误读。
- https://www.reddit.com/r/codex/comments/1wxq5ng/comment/pdvutqe/ ，576分，原句 `Wow. They must be bleeding subscribers.` 是订户流失猜测，没有数据支持，不因高分升格事实。
- https://www.reddit.com/r/codex/comments/1wxq5ng/comment/pdvuxh2/ ，12分，原句 `key id take the reset lol` 是偏好/戏谑，不据此认定发帖者误解了承诺。

补充校准：Claude主帖的 `Pro, Max, and Team users get a reset to use anytime` 后接短链，无句号；上表已保持不补句号，完整原始串以d-claude-reset-fields.json为准。帮助页/员工帖仍未找到20×→10×数对；公开pricing链接跳转账户弹窗后已否决并移除其内容，没有把私有界面作为公开证据。

截图时计数另有变化：Tibo28天卡片在09:44:01显示665.6万浏览、4425回复、3605页面转帖、2.5万赞、2546书签；前帖09:44:15显示261.6万浏览、1981回复、602页面转帖、1.4万赞、914书签。显示的缩写不反推出精确值，也不覆盖早先JSON精确计数。

## 已验收截图索引

此表为最终选用版本；所有图已实际打开检查。备用/失败版本只留在capture-log-d.md，不作正式配图建议。

| 清单 | 图 |
|---|---|
| D1a 28天承诺与引用前帖 | [d-25-tibo-28.png](../images/d-25-tibo-28.png)、[d-25-tibo-prior.png](../images/d-25-tibo-prior.png) |
| D1c 两帮助页 | [d-25-banked-expiration.png](../images/d-25-banked-expiration.png)、[d-26-paid-eligibility.png](../images/d-26-paid-eligibility.png) |
| D2 开放时间、许可 | [d-27-beam-preview-readable.png](../images/d-27-beam-preview-readable.png)、[d-27-beam-license-readable.png](../images/d-27-beam-license-readable.png) |
| D2 编码/推理/工具/通用四张完整表 | [d-28-beam-coding-full.png](../images/d-28-beam-coding-full.png)、[d-28-beam-reasoning-full.png](../images/d-28-beam-reasoning-full.png)、[d-28-beam-tools-full.png](../images/d-28-beam-tools-full.png)、[d-28-beam-general-full.png](../images/d-28-beam-general-full.png) |
| D2 完整效率双图与脚注 | [d-28-beam-efficiency-clean.png](../images/d-28-beam-efficiency-clean.png) |
| D3 图像广告条件 | [d-29-ads-context.png](../images/d-29-ads-context.png) |
| D4 15+及声明 | [d-30-nieman-tests-response.png](../images/d-30-nieman-tests-response.png) |
| D4 新guardrail及未修尽 | [d-31-nieman-guardrail.png](../images/d-31-nieman-guardrail.png) |
| D4 授权限定 | [d-32-nieman-license.png](../images/d-32-nieman-license.png) |
| D4 法律限定 | [d-33-nieman-law.png](../images/d-33-nieman-law.png) |
| D4 无买卖证据、水印检查 | [d-34-nieman-sales-watermark.png](../images/d-34-nieman-sales-watermark.png) |

合计17张验收候选图。Beam整表仅展开原横向容器、保留所有原单元格，未重绘；效率clean版仅隐藏浏览器扩展悬浮UI，原站图、正文和数字未改。Nieman全为文字截图。精确方法、失败过程、隐私例外处理均见[capture-log-d.md](capture-log-d.md)；全部新增文件见[d-manifest.md](d-manifest.md)。
