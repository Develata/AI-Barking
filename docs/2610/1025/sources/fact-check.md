# 1025 事实清单

日期按北京时间；与原时区不同时在备注注明。

## 主帖

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：10年前的AI比GPT准，却漏18个癌 | L1 | ✅ | 同下 | “10年前”= Inception v3 “first published by Google in 2015”；“比GPT准”= 六分类 125 对 84；“漏18个癌”= 30 个黑色素瘤中按预设阈值放走 18 个（preset threshold）。黑色素瘤属皮肤癌。 |
| 省流：皮肤科医生实测，专门训练过的十年前老 AI，这次认皮肤病比 GPT 和 Claude 准，却放走了更多黑色素瘤（一种皮肤癌）。 | L1 | ✅ | 同下 | 作者自报，单人测试；“十年前”指架构（2015），权重为作者新近微调，作者标题自称“A 10-year-old CNN”。反向核验第1轮 SHOULD_FIX 4。 |
| 一、老 AI 按预设线放走18个黑色素瘤 | L1 | ✅ | 同下 | 小标题；“老 AI”= Inception v3 CNN；“预设线”= preset threshold（视觉复核 SHOULD_FIX 2）。 |
| 盖伊医院皮肤科医生 Magnus Lynch 从公开数据集抽了200个皮肤病变，都经活检确诊，每个有一张近照、一张皮肤镜图 | L1 | ✅ | https://lynchlab.net/llm-skin-cancer ; a-article.txt:16–26、114；images/02 | “Consultant Dermatologist and Mohs Surgeon at Guy's Hospital”；MILK10k（ISIC，2025-07-01 发布，CC BY-NC）；“I drew 200 lesions at random, all confirmed by histopathology”；每例 clinical close-up + dermoscopic image。文章 25 October 2026，官方未给时刻。 |
| 他把照片交给两个大模型 GPT-6-Astra、Claude-Fable-5.1，再交给一个老 AI：谷歌2015年发表的 Inception-v3 结构 | L1 | ✅ | a-article.txt:16、40 | “a convolutional neural network (CNN) first published by Google in 2015”。 |
| 作者用约6.8万张皮肤图专门训练过它 | L1 | ✅ | a-article.txt:40；images/04 | “fine-tuned all layers on 68,011 dermoscopic and clinical images from the ISIC Archive”；pretrained on everyday photographs。测试的 200 例及同病灶、同患者照片训练前已剔除（作者自述）。 |
| 三者看照片答题：是六种病里的哪种，能不能放病人回家 | L1 | ✅ | a-article.txt:38、40 | LLM：“the most likely of the six diagnoses (or “unsure”) … whether the lesion could be safely discharged”，两图、同一提示词、默认设置、无拒答。CNN 从同样六种中选，放走判断按预设阈值（换算口径原文未写，见 evidence-a A3e）。 |
| 200个病变的诊断题，专门训练过的老 AI 对了125个，GPT 对84个，Claude 对85个 | L1 | ✅ | a-article.txt:58–61；images/07 | 表：Correct diagnosis (of 200) GPT-6 Astra 84 (42%)、Claude Fable 5.1 85 (42%)、CNN 125 (62%)。无置信区间。A 组用原始回复复算 LLM 数字一致；CNN 逐例输出不在数据包，只能引作者自报。 |
| 30个黑色素瘤里，老 AI 按作者事先设好的判定线放走18个，GPT 放走3个，Claude 放走4个 | L1 | ✅ | a-article.txt:83–86；images/07 | “Melanomas discharged (of 30)”：3、4、“18 at its preset threshold; 2 or 1 when matched”。“事先设好的判定线”= preset threshold，模型页称在验证集上设定（evidence-a A3e）。18 的换算口径原文未写，与模型页单图数字不一一对应（evidence-a A3e），按作者自报写。 |
| 大模型漏得少，是因为几乎都不放：100个良性病变，GPT 只放回家32个，Claude 22个，老 AI 放回76个 | L1 | ✅ | a-article.txt:78–81；images/07 | “Safe lesions discharged (of 100)” 32、22、“76 at its preset threshold”。“几乎都留下”：不应放走的 100 例 LLM 留下 97、96（表），前作 “managed this largely by keeping almost everything”（a-drmagnuslynch.txt）。 |
| 作者称换种判定线设法，老 AI 只放走1到2个，但没说怎么设、会多留多少良性的 | L1 | ✅ | a-article.txt:76、81、86 | “2 or 1 when matched”；“matched”原文未定义，“换一种判定线设法”对应 when matched，matched 的比较对象原文未定义，正文不写成“与大模型看齐”（反向核验第1轮 SHOULD_FIX 3）；调后良性放走数表中未给（只写 76 at its preset threshold）。 |
| （已撤销的旧判断，未入正文）AI/TLDR 写“fewer misses”——指六分类错误更少，并非黑色素瘤漏放更少，“与表相反”不成立 | L5 | ⚠️ | https://ai-tldr.dev/releases/lynch-lab-cnn-vs-llm-skin-cancer/ ; a-aitldr.txt | AI/TLDR：“A decade-old image classifier made fewer misses than two frontier LLMs”，页面日期 2026-10-25。指其写法，不点名。 |
| 这是一位医生发在自己网站上的测试：黑色素瘤只有30个，大模型每例只答一次，作者自己说只够看出趋势 | L1 | ✅ | a-article.txt:90；images/08 | “Two hundred lesions are enough to show the pattern but not to rank close results, and 30 melanomas give wide uncertainty”；“Each LLM case was run once; answers vary between runs.” lynchlab.net 为作者个人网站。 |
| 这些病变都因看着可疑才被切下化验，比门诊常见的难认 | L1 | ✅ | a-article.txt:26–28、92 | “Every lesion was biopsied, including the benign ones, so these are harder to diagnose: a mole that somebody decided to cut out is one that worried a clinician.”；Caveats “Hard cases”。 |
| 省流：Kimi 为订阅额度被白白耗掉道歉，说补回25%，用得少的人拿不满。 | L1 | ✅ | 同下 | “白白耗掉”= 官方 “unnecessary quota consumption” / “不必要的额度消耗”；“拿不满”见吠点①行。 |
| 二、Kimi 道歉：订阅额度被白白耗掉 | L1 | ✅ | 同下 | 小标题。 |
| Kimi-K3.1-Pro 上线后，有付费用户称，5小时窗口内的额度几分钟就用完了 | L6 | ⚠️ | HN 50222093（b-hn-50222093.json）；images/23 | “I paid $19 for Kimi Code to test K3.1 Pro … The entire 5-hour quota was exhausted within minutes.” 北京 10/24 23:16。作者披露自己开发竞品；Reddit Ultra 用户另称一个提示耗尽会话额度（L6）。正文用“有付费用户称”。“5小时窗口”= Kimi Code 文档“每 5 小时的滚动频率窗口”（b-code-member-zh.txt:258，images/15）；不写成定时刷新（反向核验第2轮 SHOULD_FIX 1）。 |
| 25日晚，Kimi 开发者账号先宣布更新编程工具：AI 默认不再自己分出帮手干活，要用得在指令里明确要求，或打开集群模式 | L1 | ✅ | @KimiDevs 2114327832646004990、…835930472778、…838140592411（b-fx-*.json）；images/20–22 | 北京 10/25 20:06:51–53。CLI 2.3.2：“Handle tasks directly by default instead of proactively launching subagents; request delegation explicitly … or an enabled multi-agent mode.” Desktop v1.1.3：“To have it delegate tasks, explicitly request delegation or enable swarm mode.” “帮手”= subagent（子 Agent），“集群模式”= swarm mode；“指令里明确要求”涵盖 “your message, AGENTS.md, a skill”。官网 changelog 取证时 CLI 最新列 2.3.0、Desktop 1.1.1，故写“宣布”（反向核验第1轮 SHOULD_FIX 6、NV 10）。只写时间先后，不写因果（道歉帖未提这次更新）。 |
| 26日凌晨，Kimi 致歉，承认 K3.1-Pro 订阅出现限流和“不必要的额度消耗” | L1 | ✅ | https://x.com/Kimi_Moonshot/status/2114450610724708492 ; b-fx-2114450610724708492.json；中文公告转录 b-21jingji-2.txt、b-ifeng.txt；images/13、14、24 | UTC 10/25 20:14:44 = 北京 10/26 04:14:44。“We're sorry for the rate limits and unnecessary quota consumption … These were our issues”；中文“对于近期Kimi会员使用K3.1 Pro时出现的限流及不必要的额度消耗，我们向大家致歉”（官方微博/公众号原帖未直接读到，中文以媒体转录为准，L4）。 |
| 官方称给所有档位的订阅会员重置一次：补回月度总额的25%，相当于一周用量；当前用量不足25%的，重置为0 | L1 | ✅ | 同上 | EN：“every subscriber on every plan gets a one-time usage reset, restoring 25% of their monthly quota (usage below 25% resets to zero), equal to one week of usage”；中文：“为所有档位的订阅会员进行一次重置：所有有效期内的会员都将获得月度总额的25%作为补偿，相当于一周的用量。如果当前用量不足月度总额的25%，将重置为0。”正文照官方口径写“重置”“补回”，不写具体扣减公式（evidence-b B2 记为部分支持；反向核验第1轮 SHOULD_FIX 5）。国内 kimi.com（人民币）与海外 kimi.ai（美元）两套价目，官方未分别说明。到账：EN “end of day PST”，中文“今天结束前”。 |
| 官方还称在减少不必要的帮手调用，并缩短输出 | L1 | ✅ | 同上 | “We've also adjusted the plan's parameters and are reducing unnecessary subagent calls and output length”；中文“已调整配置，并正在减少不必要的子Agent调用和输出长度”。 |
| 说是补回25%，但用得少的人只是用量归零，拿回的不到25% | L1 | ✅ | 同上 | 由“usage below 25% resets to zero / 当前用量不足月度总额的25%将重置为0”直接推出：已用 x<25% 者用量回到 0，拿回 x<25%；不依赖其余用户的扣减公式。 |
| （未入正文）新浪数码标题写成“发放25%月会员额度” | L4 | ⚠️ | https://weibo.com/2/detail/5358242814755360 ; evidence-b.md:99 | 标题“Kimi致歉并宣布补偿，发放25%月会员额度”，取证约北京 21:30 显示“9小时前”，精确时刻未取。 |
| 修好后额度够不够用，致歉公告没给数字，也没说受影响人数 | L1 | ✅ | 中英文公告全文；evidence-b B2、B7 | 范围：所查为官方英文帖、中文公告转录、@KimiDevs 四条、Kimi Code 文档与两个定价页；官方状态页与官方回复帖未逐一检索。 |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 07：诊断正确（共200例）：GPT-6 Astra 84，Claude Fable 5.1 85，CNN 125 | L1 | ✅ | a-article.txt:58–61 | 表行 “Correct diagnosis (of 200)”；高亮为整行矩形（rects）。 |
| 07：良性病变被放走（共100例）：32、22；CNN 按预设阈值为76 | L1 | ✅ | a-article.txt:78–81 | “Safe lesions discharged (of 100)”；“76 at its preset threshold”。 |
| 07：黑色素瘤被放走（共30例）：3、4；CNN 按预设阈值为18，“matched”时为2或1 | L1 | ✅ | a-article.txt:83–86 | “18 at its preset threshold; 2 or 1 when matched”；matched 保留原词不译（原文未定义）。 |
| 08：200个病变足以显示规律，但不足以给接近的结果排名；只有30个黑色素瘤，最关键的那个数字不确定性很大。 | L1 | ✅ | a-article.txt:90 | “Two hundred lesions are enough to show the pattern but not to rank close results, and 30 melanomas give wide uncertainty on the figure that matters most.” |
| 08：大模型每个病例只跑一次；不同运行之间，答案可能不同。 | L1 | ✅ | a-article.txt:90 | “Each LLM case was run once; answers vary between runs.” |
| 08：每个病变都做过活检。 | L1 | ✅ | a-article.txt:92 | “Every lesion was biopsied.” |
| 24：对 K3.1 Pro 订阅套餐出现的限流和不必要的额度消耗，我们深感抱歉。 | L1 | ✅ | b-fx-2114450610724708492.json | “We're sorry for the rate limits and unnecessary quota consumption on the K3.1 Pro subscription plan.” 截图为登录态页面裁切帖子正文（B 组），时间按美东显示“下午4:14 2026年10月25日”= 北京10/26 04:14。 |
| 24：所有套餐的每位订阅者获得一次性用量重置，恢复月度额度的25%（用量低于25%的重置为零），相当于一周用量。 | L1 | ✅ | 同上 | “every subscriber on every plan gets a one-time usage reset, restoring 25% of their monthly quota (usage below 25% resets to zero), equal to one week of usage.” |
| 24：正在减少不必要的子 Agent 调用和输出长度 | L1 | ✅ | 同上 | “reducing unnecessary subagent calls and output length”。 |
| 22：默认由 Agent 自己完成任务；要让它分派任务，需明确提出委派，或启用集群模式。 | L1 | ✅ | b-fx-2114327838140592411.json | Kimi Code Desktop v1.1.3：“By default, the Agent completes tasks on its own. To have it delegate tasks, explicitly request delegation or enable swarm mode.” 北京10/25 20:06:53。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据 The Register 报道，IBM CEO 称开放权重模型达前沿九成效果 | L4 | ⚠️ | https://www.theregister.com/systems/2026/10/22/ibm-ceo-says-open-weight-models-are-now-irresistibly-good-and-cheap-on-prem/5304640 ; c-c-register.txt | 原句：“anywhere from 25 percent to 10 percent of the token cost of the frontier models. That is very attractive given that they’re now 90 percent as effective.” 10%–25% 是 token 成本，不是总成本；“90 percent as effective”无具名基准。导语 “90 percent of the performance at ten percent of the price” 是报道的引语，不当原话。IBM 3Q26 prepared remarks（c-c-ibm-q3-remarks.txt）无此句，电话会转录未取到。页面可见时间 05:26 UTC（北京 13:26），JSON-LD 为 04:26:39Z，两者冲突，以可见时间为准（反向核验第1轮 SHOULD_FIX 8）。卡片只写“九成效果”，成本口径 10%–25%（token 成本）留在本行（第1轮 BLOCKER 2：“一到四分之一”译错，已删）。HN 317/220（北京 10/26 09:47 提交）。措辞待卡片定稿时按 40 字压缩。 |
| 据伯克利学生报报道，菲尔兹奖得主 Borcherds 改按作业完成情况给分 | L4 | ⚠️ | https://www.dailycal.org/news/campus/academics/ai-use-leads-uc-berkeley-math-professor-to-see-no-point-in-grading-homework/article_bdf1eae2-c08b-491f-b9f1-81a7cea177de.html ; c-c-dailycal.txt | 伯克利学生报 = The Daily Californian（UC Berkeley 独立学生报）。原句：“no longer grades his students’ homework”；“no point” in paying readers to grade for correctness；“currently grades on completion if a student hands in their homework”；他自称 “not a very good system”。10/23（页面日期，元数据北京 21:15）。HN 221/320。 |
| r/opensource 版主称，用 AI 或面向 AI 的项目不再欢迎 | L1 | ✅ | https://www.reddit.com/r/opensource/comments/1xelcuf/your_tokens_arent_valuable_no_more_ai/ ; c-reddit-opensource-1xelcuf.json | 版面管理方的一手公告（版主 Wolvereness 署名、置顶规则帖；按 L1 官方社区公告处理，反向核验第1轮 SHOULD_FIX 7）：“if your project uses AI, is made by AI, or is intended for AI, it's not welcome on r/opensource.” 另列门槛：项目满 1 年且活跃维护、多名贡献者（subjectively 3+）、账号满 1 年、不会英语另找社区等。只是 Reddit 一个子版（约 39.4 万订阅）。原帖时刻未取到；The Verge 10/25 13:25 UTC 报道。HN 88/111。截至北京 10/26 约 20:45 未见撤回（未逐条核编辑记录）。 |
| CapCut 称其插件即将上线 ChatGPT，可在对话中生成与剪辑视频 | L1 | ✅ | https://x.com/capcutapp/status/2114062937094823955 ; c-c-capcut-fx.json | “our new CapCut plugin is coming soon to @ChatGPT”；“brings CapCut's video editing and generation tools alongside ChatGPT”，草稿再到 CapCut 网页编辑器精修。UTC 10/24 18:34（北京 10/25 02:34）。即将上线，未给日期。官方账号在海外社媒，正文与卡片不写平台名。 |

未收：Wiz Gheist（与 AI 无直接关系）、ThinkRail（仓库 7 月已建，本期新意不明）、Nanbeige4.3-33B（热度低）、Gary King 论文（HN 2–4 分；对照组限定词放不进一行）、Meta 条款推迟（未找到任何 11/20 材料，扫描说法不成立）。

## 读者稿检查

Codex gpt-6-astra medium（read-only），报告 `style-review.md`。采纳：首段长句拆开；“老网络”改“老 AI”；“黑色素瘤”加“一种皮肤癌”；“预设阈值”改“作者事先设好的判定线”；“看照片答题”点明是测试；删“海外科技站写成漏得更少”与“新浪数码标题”两句（读者不在乎，且前者经反向核验不成立）；“5小时额度”改“每5小时刷新一次的额度”；“子 Agent”改“帮手”；省流与吠点①去掉重复的纠正句式。不采纳：开场与 Slogan 是固定栏目语；“限流”保留（常用词）。标题首选维持候选1（读者排第2；排第1的候选7不符合事实：测试不是让病人拍照问）。

## 反向核验

- 第1轮（WSL Codex gpt-6-astra medium，`barking review --reverse`，审的是读者稿检查前的正文）：2 BLOCKER、6 SHOULD_FIX、2 NEEDS_VERIFICATION。①AI/TLDR“fewer misses”指六分类错误更少，“表里正相反”不成立——成立，该句已随读者稿检查删去；②IBM“一到四分之一”译错——成立，卡片只写“九成效果”；③matched 未定义——成立，改“换一种判定线设法”；④省流丢专门训练——成立，补“专门训练过的”“这次”，架构与权重年龄区分记入清单；⑤补偿公式是解释——成立，正文改照官方口径“重置”“补回”，吠点只用“不足25%重置为0”推出的结论；⑥委派条件被缩窄——成立，改“在指令里明确要求”；⑦版主公告分级——成立，改 L1；⑧Register 时间——成立，记两值；NV⑨卡片待第2轮；NV⑩官网 changelog 未见新版本——正文改“宣布”。
- 第2轮（同上，reverse-report-2，审读者稿检查与第1轮修改之后的正文、卡片与清单）：PASS，6 SHOULD_FIX、1 NEEDS_VERIFICATION，均成立已改：①“每5小时刷新一次”改“5小时窗口内”（官方为滚动窗口）；②省流卡补测试范围与专门训练（与视觉复核①合并）；③Borcherds 改“按作业完成情况给分”（on completion）；④Kimi 吠点②收窄为“致歉公告没给数字，也没说受影响人数”；⑤AI/TLDR 行标为已撤销旧判断、改 L5，README 去掉“夸大实例”；⑥README 中 r/opensource 改 L1；NV⑦补“制作元信息”一节。

## 视觉复核

- WSL Codex gpt-6-astra medium（`barking review --visual`，10 张图）：PASS，4 SHOULD_FIX，均成立已改：①省流卡皮肤癌条补“200个病变”“专门训练过的”（正文同步）；②速览第1条与小标题改“老 AI 按预设判定线放走18个黑色素瘤”；③Kimi 致歉句补“K3.1-Pro 订阅”；④08 译注②改“不同运行之间，答案可能不同”。高亮位置、底图完整、平台名均未见问题。

## 制作元信息

- 卡片日期“10月26日”为期次展示日期（北京时间）；四张批注图页脚“2026-10-26 存档”：07、08 底图 A 组截于北京 10/26 20:32–21:05（a-shots-log.jsonl），22、24 底图 B 组截于北京 10/26 20:34–21:35（b-shots-log.jsonl、capture-log-b.md）。反向核验第2轮 NV 7。

## 待办

- 已删除（北京10/26深夜，一次 rm）：sources/c-meta-blog.html、c-reddit-err.txt、a-aigeek.*；仓库外 B 组登录态时间线 JSON 三份与登录态弃图四张。
- 发布前：Kimi 中文公告官方原帖仍未直接读到；如 Kimi 发布补偿到账或修复数据，正文吠点②需重查。
