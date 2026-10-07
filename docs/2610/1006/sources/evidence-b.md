# 1006 B 组证据清单（部分完成，未通过完整交付验收）

本轮采集：北京时间 2026-10-07 凌晨（本机 2026-10-06 EDT，UTC−4）。仅处理 B 组；不写发布正文，不下编辑结论。工作区基线 `85f6bb9ce45dc6084d37d30b4c8e3a8113d0f353`。

**交付限制：未取得浏览器全文/HTML、原图与截图 15–24；没有把搜索摘要、历史存档或条件性算式当作本轮原图核验。** OpenCLI 扩展未连接；恢复尝试和两次自动审批拒绝见 [capture-log-b.md](capture-log-b.md)。下列“已找到”只说明该子项原文可见，不代表截图验收通过。网页读取工具的文本视图与搜索索引可能不同步，热度只记所见快照。

## B1–B9

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1 原文免费部分、作者、发布时间、付费墙 | L3 | https://newsletter.semianalysis.com/p/anthropic-subscriptions-offer-5x | “Third party plans are worse than first party”；“This post is for paid subscribers” | 无，未取得 | 原站文本可见作者 Andrew Megalaa、Max Kan、Dylan Patel；页标 Oct 05, 2026 / Paid。墙位于该小节首段后。未拿到 JSON-LD/post_date，准确发布时刻未核；未形成要求的 body.innerText + HTML 全文存档 | 部分支持 |
| B2 全部原图、方法/额度/散点图及 A/B 截图 | L3 | https://newsletter.semianalysis.com/p/anthropic-subscriptions-offer-5x | 原图文字未独立读取，不把派工单图名或数字当作已核引文 | 无；15–21 全缺 | 文本提取保留图前后段和来源行，但未返回 substackcdn 原图链接；全部原图数量、标题/图例/脚注、像素尺寸均未验收。没有用转载图、重绘或替代图 | 部分支持 |
| B3 “~5x”“~4x”语境与各档比值 | L3；算式为本组复算 | https://newsletter.semianalysis.com/p/anthropic-subscriptions-offer-5x | “Anthropic is an overwhelmingly better deal, offering ~5x the API-equivalent value across the board.”；“we don’t think this offsets the ~4x higher API-equivalent value offered by Opus 5.5 on Claude plans.” | 无 | 两句可在免费原文核对；5×限定 Opus 5.5 vs GPT-6.1 Sol。4×在每美元比较之后，具体散点未读，不判前后矛盾。详见下表及条件性复算 JSON | 部分支持 |
| B4 两家公开定价页的倍数字样 | L1 | https://chatgpt.com/pricing/ ; https://claude.com/pricing | OpenAI：“Your choice of 3 usage tiers”；“Longer Codex and ChatGPT Work sessions”。Claude：“Choose 5x or 20x more usage than Pro*” | 无；22–23 未取得 | OpenAI 未登录公开文本视图中检索 5x/20x 无匹配，不能据此证明所有语言、交互状态及历史版本已删除。Claude FAQ 限定每 5 小时窗口；不是每月总量倍数。官方页未给发布时刻 | 部分支持 |
| B5 Tibo 降额两帖、halving 与 10/29 出处 | L2；帮助中心 L1 | https://x.com/thsottiaux/status/2104823812042940713 ; https://x.com/thsottiaux/status/2104951965184925941 ; https://help.openai.com/en/articles/9793128-about-chatgpt-pro-tiers | 历史首帖：“it will net out at half the dollar in API spend compared to the old Pro $200 plan.” 帮助页：“Eligible customers can use the previous included allowance through October 29, 2026 while their Pro 200 subscription is active.” | 无；24 未取得 | 两帖本轮实时打开失败；复用0929公开原帖存档。首帖北京时间9/29 14:41:17（06:41:17 UTC）；次帖23:10:31（15:10:31 UTC）。两帖未写10/29；本轮官方帮助页确认日期。到期日未注明时区，不能补为北京时间零点 | 部分支持 |
| B6 10/5–10/6 默认速度提升约50% | L2 | https://x.com/thsottiaux/status/2107158998495748264 | 历史原帖：“Day 1/”；“We have optimized the default speed to be ~50% faster across GPT-6 Astra and GPT-6.1 Sol” | 无 | 本轮原站失败；1005公开存档英文完整字段可回查。北京时间10/6 01:20:29（10/5 17:20:29 UTC，13:20:29 EDT）。范围为订阅及 Sign in With ChatGPT 产品/合作伙伴；并非本组测速，也不是额度增加50% | 部分支持 |
| B7 HN、Reddit、X 与中文报道热度 | L6；媒体自身标题为原站字段 | https://news.ycombinator.com/item?id=49975345 ; https://www.reddit.com/r/codex/comments/1wyjx8u/openai_offers_5x_less_value_than_anthropic_at/ ; https://news.cnyes.com/news/id/6622335 | HN：“Anthropic Subscriptions Offer 5x+ More Value Than OpenAI” | 无 | HN网页读取快照75分/81评论；Reddit搜索索引主帖975分。分数来源、时效、其他帖子见 b-source-notes.md；Algolia失败，未统计全部相关帖合计；X量级未取得 | 部分支持 |
| B8 实际夸大标题/文章 | L6；媒体标题自身为事实，不替代评测 | https://weibo.com/2/detail/5350898318704655 ; https://www.reddit.com/r/codex/comments/1wyjx8u/openai_offers_5x_less_value_than_anthropic_at/ | 中文搜索索引：“同样 200 美元，Claude 订阅的 Token 用量约为 OpenAI 的 5 倍”；英文帖子标题：“OpenAI offers 5x less value than Anthropic at same pricepoint (source: semianalysis)” | 无 | 中文原帖直开失败，仅索引线索；英文原帖可见但其标题是泛称value，不能偷换成明确声称能力强5倍。指定的“强5倍/多干5倍活/便宜5倍”且原站、时刻齐全的实例未找到 | 部分支持 |
| B9 HN/Reddit 有依据的质疑 | L6 | https://news.ycombinator.com/item?id=49975345 ; https://www.reddit.com/r/ClaudeAI/comments/1wz02ul/anthropic_subscriptions_offer_5x_more_value_than/ | Reddit literum：“How did you define and measure "value"?” | 无 | 已见任务token效率、不同tokenizer、实际使用率的质疑；评论深链提取失败、HN评论分数未公开，不以帖子分数代替。详细作者/摘句/分数来源见下节 | 部分支持 |

## 5×、4×与条件性复算

5×句位于 OpenAI vs Anthropic 小节，上一段从 Astra/Fable 转入 Opus 5.5/GPT-6.1 Sol；下一段转向 token 数图。派工单提供的后半句经本轮原文核对为：“You could argue that this is unfair for OAI because 6.1 Sol is much cheaper per token than Opus 5.5, but the gap is still massive even if you switch to comparing the number of tokens.”

4×句位于 OpenAI’s new $500 plan and massive limit reduction 小节末尾，随后进入 API 降价问题。此前文字讨论 Pro 100/200/500 的每美元量、Claude 套餐相对所有 OpenAI 套餐的价值；该句前半段提 OpenAI Pro 没有5小时窗口限制，更容易实际用满额度。这是语境记录，**未取得散点图，不能确认“4×就是Opus对Astra”或判“与5×矛盾”**。

下表输入数字全部来自派工单“已知情况”，尚未独立对照原图；只验证除法。单位 API-equivalent value 为美元，token 为十亿（B）；分子均 Anthropic。

| 同月费 | API价值算式 | 比值 | token算式 | 比值 |
|---|---|---|---|---|
| $200，Max20x/Pro200 | 11726 ÷ 2084 | 5.6267 | 28.6 ÷ 10.2 | 2.8039 |
| $100，Max5x/Pro100 | 5725 ÷ 1055 | 5.4265 | 14.0 ÷ 5.1 | 2.7451 |
| $20，ClaudePro/Plus | 1178 ÷ 211 | 5.5829 | 2.9 ÷ 1.0 | 2.9000 |

候选解释算式：11726 ÷ 2897 = 4.0476（Opus/Max20x 对 Astra/Pro200）。数值吻合约4×，但仅凭吻合不足以证明作者所指；散点图缺口仍保留。其余档位 Astra 图值本轮未从原图取得，不借二手站补齐。计算机可复算数据见 [b-conditional-ratios.json](b-conditional-ratios.json)。

## 可能的吠点（条件与限制，不作编辑定论）

1. 原文把价值定义在套餐、模型、workload组合上，不能直接转为模型能力倍数；出处：SemiAnalysis原文 Methodology / OpenAI vs Anthropic。
2. 原文的 ±5% 是仪表读数反推的测量范围，不是所有现实任务、账号或月份的总体统计保证；500M cache read 不动表是其免费假设阈值，不是厂商承诺；出处：Computing the Results / Caveats。
3. 5小时窗口会影响人能否实际用满按周/月折算的额度，原文承认 OpenAI Pro 无此窗口；出处：4×句上下文。
4. Claude 官网倍数明确按每5小时窗口，另有周限制；出处：https://claude.com/pricing FAQ。原文研究按月折算，两种数字不可直接互换。
5. A/B段未点名厂商，不能归因 OpenAI 或 Anthropic；出处：Catching a Provider A/B Test。
6. 原文推介付费 Subscriptions Dashboard / Tokenomics Model，属于需披露的商业背景；出处：方法部分之前的产品介绍。
7. 派工单workload四项百分比加总99.9%，可由显示精度造成，但原图未取得，不擅自归一化、更正数字或认定原因。

### 社区质疑（L6，仅线索）

| 平台/作者 | 内容（引号内为原句） | 得分与出处 | 限制 |
|---|---|---|---|
| HN viraptor | 质疑不同模型完成同一任务的token消耗不同，不能只比API折算容量 | https://news.ycombinator.com/item?id=49975345 ，作者viraptor；评论分数未公开 | 已见主帖评论正文；点击评论深链失败，未取得评论ID。不能标高赞已证 |
| HN hervem | 质疑不同tokenizer导致token单位不可直接等同 | 同上，作者hervem；评论分数未公开 | 未把该质疑当实测结论；深链未取得 |
| HN k7peak | 质疑订阅用户未用完额度时，对利润的推断如何变化 | 同上，作者k7peak；评论分数未公开 | 可回看原文对使用率的假设，不能仅据评论反推利润 |
| Reddit literum | “How did you define and measure "value"?” | https://www.reddit.com/r/ClaudeAI/comments/1wz02ul/anthropic_subscriptions_offer_5x_more_value_than/ ，搜索索引+10 | 原帖文本视图可见作者与原句；+10只来自索引，评论深链未取得 |
| Reddit Souvik_Dutta | “Comparing based on token pricing only won't give you a good picture.” | 同上，搜索索引+12 | 原帖正文可见原句；其关于Sol更省token的主张未独立验证，不能当事实 |

## 夸大说法实例

- **中文待复核实例**：宝玉xp，https://weibo.com/2/detail/5350898318704655 。原句见B8；搜索索引显示“10小时前”，未提供可验证绝对时刻/时区，不换算成精确北京时刻。正文索引同时解释API等价价值与模型范围，开头却把5倍写为Token用量；这是口径混用的候选实例，仍需原帖/原图复核。没有把微博重新列为运营平台。
- **英文标题范围过宽候选**：r/codex，https://www.reddit.com/r/codex/comments/1wyjx8u/openai_offers_5x_less_value_than_anthropic_at/ ，标题见B8；搜索索引标 Monday October 05 2026，无精确时区。它未在标题写模型和workload，也未明确声称“强5倍”，应避免替其加上不存在的意思。
- **中文媒体线索**：机器之心标题经新浪署名刊载为“刚刚，OpenAI宣布GPT-6提速50%，额度却被锤只有Claude的1/5”，https://finance.sina.com.cn/tech/roll/2026-10-06/doc-iniufsvm2174314.shtml ，索引标2026-10-06 07:40；新浪转载不能充当机器之心原站存档。机器之心站点本轮只返回登录/文章库外壳，未取得该文原站URL与元数据。
- **未找到**：原站与绝对发布时间齐全、明确说“Claude比ChatGPT强5倍”“能多干5倍活”或“订阅便宜5倍”的中英文成对实例。检索无命中不代表不存在。

## 扫描说法勘误

| 扫描说法 | 本轮核验结果 |
|---|---|
| 发布时刻/modifiedTime | 页面日期10/5已见；post_date未取得。索引线索20:01:09Z换算为北京10/6 04:01:09，但仍非元数据核验。派工单modifiedTime 20:54:04Z也未复核；即便成立亦是修改时间（北京10/6 04:54:04），不能当发布时刻 |
| ~5×与~4×前后不一致 | 两句存在；比较语境不同。4×具体散点未核，不能判矛盾；候选比值4.0476仅为推断 |
| token数也是5× | 按派工单输入复算约2.75–2.90×；原图缺口未补齐，标条件性算式 |
| A/B测试点名厂商 | 与可见原文不符：未点名；不据段落顺序猜厂商 |
| OpenAI价格页删去倍数字样 | 当前公开文本无5x/20x；尚无本轮HTML、历史页面对照和套餐卡截图，不能将“未见”升级为历史删除过程已证 |
| 提速50% | 历史官方员工原帖支持“默认速度约50%”；本轮实时原帖未取得，不标本轮已核。不是额度提升50%，也不是对所有模式的独立测速 |
| 10月29日出自两个Tibo帖 | 历史两帖均未写该日期；本轮帮助中心可核。首帖说明API折算减半，次帖给1/5/10X；日期另有L1来源 |

## 验收结论

B1–B9均有行、状态限于规定四种；但整体仅部分完成。截图引用数为0，图片存在性检查不能据此声称B2/B4/B5通过。待补：免费全文DOM及HTML、发布时间元数据、所有原图与15–24截图、散点图实际数值、实时官方帖、HN合计/X热度、评论深链及可核得分、原站夸大实例时刻。未commit/push/delete；并行A/D文件及1005原有未跟踪文件不归本组认领或修改。
