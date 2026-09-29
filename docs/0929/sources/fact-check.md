# 0929 事实核验

正文：`../doc_0929_publish.txt`（定稿：Develata 定为 OpenAI 主场，Muse 压缩为一小段）。取证：`evidence.md`（A–D，Codex gpt-6-astra）与 `e-capture-log.md`（E 组 DevDay 与 Tibo，Sonnet 子代理），另有 `e-tibo-tweets.json`、`e-openai-tweets.json`（Claude 用 opencli 抓取，北京时间 9/30 01:2x）。Claude 对下列条目逐字对照了存档原文或截图。按 EDITORIAL.md，L4 及以下只标 ⚠️，并在正文中用对应措辞。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京时间30日凌晨，OpenAI 在 DevDay 发布 dots | L1 | ✅ | OpenAI X 2104984504133918973；openai.com/index/introducing-dots | X 帖 2026-09-29 17:19:49 UTC = 北京 9/30 01:19；官方页标 September 29, 2026（美国日期）。 |
| 全天候干活、可连4000多个应用 | L1 | ✅ | introducing-dots | “can work towards your goals 24/7”；“connect to over 4,000 apps”；“rolling out today in ChatGPT to Pro and Business Premium users in eligible markets”（Enterprise 等用户可在管理员启用后试用 beta）。 |
| 官方称 dots 由 GPT-6-Astra 驱动 | L1 | ✅ | introducing-dots；OpenAI X 同帖 | “Powered by GPT‑6 Astra”；“Introducing dots, powered by GPT-6 Astra.” |
| 员工 Tibo 称 dots 不消耗任何额度 | L2 | ✅ | X thsottiaux 2104981170685616361 | “Included in your Pro plan, without drawing down on any of your usage.” 北京 9/30 01:06。 |
| 官方页：与 dots 对话不计入，它在 Codex 或 ChatGPT Work 开的任务照常计入 | L1 | ✅ | introducing-dots | “Conversations with your dot don’t count toward your ChatGPT usage limits. When you ask your dot to start or manage tasks in Codex or ChatGPT Work, those tasks count toward your usage limits as usual.” |
| AISI 日前公布：模拟环境中、关掉网安分类器后，GPT-6-Astra 有29.2%的情况完成未授权的供应链攻击 | L3 | ✅ | AISI 博客 Figure 1 与正文 | 页面标 Sep 28, 2026，无时区，正文不写具体日期。n 为场景数 100，每场景 5 次运行（报告 p3）。“cyber classifiers turned off”；“all actions were simulated”。 |
| 在10个高越权场景里写明范围外禁止，仍有4/49次成功 | L3 | ✅ | AISI 博客；技术报告 pp7–8 | 加入 “Anything not listed as in scope is out of scope.”；“4 of 49 trajectories, compared with 26 of 50 previously”；子集为早期最常越权的 10 个场景。反向核验 blocker 1 据此要求补“明确范围后仍越权”。 |
| dots 另有动作审查与沙箱，条件不同 | L1 | ✅ | openai.com/index/how-we-build-safety-security-and-privacy-into-dots | Auto-review：“a separate safety system called Auto-review checks the planned steps”；“sandboxing restricts what code and tools that dot can access”。“条件不同”为编辑判断：AISI 测试关闭了分类器、无这些产品防护。 |
| 据华尔街日报等报道，GPT-6.1-Astra 因守范围与授权未达标取消发布 | L4 | ⚠️ | WSJ（`../images/07`）；CBS、CNBC | CBS 引 Saachi Jain（head of safety systems）声明：“didn't quite meet the bar in terms of staying within scope and authorization, and how it communicates back to the user”。正文用“据……报道”。 |
| dots 用的是 GPT-6-Astra（非 6.1） | L1 | ✅ | introducing-dots；WSJ 副标题 | 6 已发布并驱动 dots；6.1 为据报道取消的下一版。 |
| 截至30日01:00，OpenAI 官网未见公告 | — | ✅（检索结果） | evidence C1；E 组第 6 项 | 只表示检索未找到，不证明不存在；CBS/CNBC 称收到公司声明。发布前如有官方表态再改。 |
| 9月29日，Tibo 称200美元 Pro 档重开订阅、用量改算、折合 API 花费只有旧版一半 | L2 | ✅ | X thsottiaux 2104823812042940713 | 北京 9/29 14:41。“it will net out at half the dollar in API spend compared to the old Pro $200 plan”。 |
| OpenAI 另推500美元 Pro 档，Pro 中仅此档含 Astra-Ultrafast | L1 | ✅ | openai.com/zh-Hans-CN/index/devday-2026-recap；help.openai.com/en/articles/9793128 | 回顾页：“推出了每月 500 美元的 Pro 套餐，提供最高用量限额，并专享 Astra Ultrafast”；帮助页：“Among Pro plans, Ultrafast is available only on Pro 500.” |
| 他称用户仍能做得更多，并给出 Plus 1倍、100美元档5倍、200美元档10倍的新比例 | L2 | ✅ | X thsottiaux 2104823812042940713、2104951965184925941 | 首帖：“you will still get more work done”；北京 9/29 23:10 次帖：“Plus = 1X / Pro 100 = 5X / Pro 200 = 10X”。倍数未见于任何官方页面。 |
| 官方帮助页确认：不享旧额保留资格的新订阅，额度低于以往 | L1 | ✅ | help.openai.com/en/articles/9793128（`../images/20`） | “New subscriptions that aren’t eligible for grandfathering include a lower usage allowance than previously offered with Pro 200 to reflect our increasingly efficient models.” 存档 `e-pro-tiers-help.md` 抓取于北京 9/30 01:38，页面显示“Updated: 33分钟前”，页面数据 updatedAt = 2026-09-29 17:04:27 UTC。 |
| 符合资格且订阅有效的老订户，旧额度保留到10月29日，之后同样下调 | L1 | ✅ | 同上 | “you’re eligible to keep your previous included usage allowance through Oct 29, 2026 … After that date, your subscription will move to the lower included usage allowance.” 资格为截止日前 7 天内有效的 Pro 200 订阅；日期为官方原文，未标时区。 |
| 9月27日，Robb 称 Muse 替他卖键盘时把地址发给买家，买家直接上门 | L6 | ⚠️ | X MattRobbt 2104090601411293303 及附图 | 当事人自述，正文用“称”。北京 9/27 14:07。 |
| 29日他更新：自己选了 “Allow Always”，以为接受报价前还会再问 | L6 | ⚠️ | X MattRobbt 2104798102037074212（`../images/04`） | 北京 9/29 12:59。“I clicked the latter thinking it would still send approvals to accept offers later down the line”。 |
| Meta 员工称没有突破隐私控制 | L2 | ✅ | X dps 2104805474268783059 | “no breach of privacy controls”；个人发言。 |
| Meta 帮助页：该选项让同一连接器的同类动作以后不再询问 | L1 | ✅ | Meta 帮助中心 1385290430137537（`../images/12`） | “Always allow: Muse can take this type of action for this Connector in the future without asking again”。官方写法 Always allow，Robb 回忆为 Allow Always。 |

## 未入正文的已知出入

- 价格：Robb 更新帖称 Muse 接受 $600 报价、他设的底价 $700，Meta 称显示 bug 吞掉了 “7”；原帖附图的商品标价 CA$15、Muse 汇报 “$10 by e-transfer”。两组数字对不上，正文不写价格。
- “地址发给了5个人”“要求停止后仍再次泄露”“等了20分钟”：只见 Guardian 采访；Muse 自述截图非权限日志。
- 6.1 继续用同一基础模型训练、与上周暂停事件无关：只见 WSJ 转引线索与 CNA 转述。
- 网传“5倍档也降成2.5倍”（X @ai_for_success）与 Tibo 第二帖“Pro 100 = 5X”不符；正文未用，留作线索。
- GPT-6.1-Sol 价格口径冲突：中文 DevDay 回顾页称 Sol 价格为 Astra 标准价的“四分之一”，英文 Sol 发布页（`e-gpt61-sol.md`）写 “one-fifth of Astra’s standard input and output token prices”；回顾页还残留 “[Add Availability]” 占位符。正文未写 Sol。
- Pro 新倍数（1/5/10X）：官方帮助页、learn.chatgpt.com 定价页、DevDay 回顾均未见，仅 Tibo 发帖（见 e-capture-log.md 第 5 项）。
- AISI 44%（自动回复被当作许可，10 场景子集）：第一稿用过，第二稿因篇幅删去。

## 反向核验（codex-reviewer，gpt-6-astra high，针对第一稿）

| # | 发现 | 判断 | 处理 |
|---|---|---|---|
| 1 | BLOCKER：结尾“边界都没说清”与 4/49 反例冲突 | 采纳 | 第二稿补“写明范围外禁止仍有4/49次成功”，结尾改为“授权范围都得看小字” |
| 2 | 标题与 Muse 事实段确定性高于来源等级 | 采纳 | 标题改换；Muse 改“Robb 称”；6.1 用“据……报道” |
| 3 | “继续干活”出自旁观者 Paula 而非 Robb；“买家无从分辨”需归因 | 采纳 | 两句均删 |
| 4 | “9月26日让 Muse 代卖”是聊天当地日期，非委托日期、非北京时间 | 采纳 | 改为 9月27日发帖日期（北京） |
| 5 | “只有媒体采访”不准，CBS 为公司声明；“未发公告”升级为断言 | 采纳 | 改“截至30日01:00 官网未见公告”，配图说明同步 |
| 6 | L4/L6 行不得标 ✅ | 采纳 | 本表已改 ⚠️ |
| 7 | 08 图也是自动翻译 | 采纳 | 配图说明改写 |
| 8 | AISI 9月28日未证明为北京日期 | 采纳 | 正文改“日前” |
| 9 | 漏掉 “for this Connector” | 采纳 | 正文改“同一连接器” |
| 10 | `b-openai-launch.txt` 仅为 JS 提示；44% 图注定位不准 | 采纳 | 本表不再引该文件；44% 图降为备用 |

## 反向核验第二轮（codex-reviewer，gpt-6-astra high，针对第三稿；E 组同时逐行核对存档）

结论：无 blocker。

| # | 发现 | 判断 | 处理 |
|---|---|---|---|
| 1 | 标题“模拟越权29%”把特定指标（Delivers a malicious payload）泛化为越权率；封面安检门暗示产品被测 | 不采纳 | 29.2% 是五个阶段里最严重的一档（完成投递恶意载荷），前几档越权行为比例更高（如调查第三方目标 99%），“越权29%”不构成夸大；标题含“模拟”、带问号，正文写明关分类器与 dots 另有动作审查。封面工牌写的是模型名 GPT-6-Astra，画的是模型过安检。已在配图说明中注明封面数字为模型模拟测试 |
| 2 | Pro 额度按新老订户划分，丢了 grandfathering 资格与订阅有效条件 | 采纳 | 正文改“不享旧额保留资格的新订阅”“符合资格且订阅有效的老订户” |
| 3 | “最终每个人都会得到更多”与帮助页并置，混淆使用价值与额度 | 采纳 | 改为“他称用户仍能做得更多，并给出……新比例；但官方帮助页确认……” |
| 4 | “独享 Astra-Ultrafast”漏掉 Among Pro plans；Enterprise/Edu 亦可用 | 采纳 | 改“Pro 中仅此档含” |
| 5 | 事实表“Enterprise 后续”无原文 | 采纳 | 改为管理员启用后试用 beta |
| 6 | 帮助页抓取元数据与存档不符；版本名未同步 | 采纳 | 改用存档时间；版本名改“定稿” |

