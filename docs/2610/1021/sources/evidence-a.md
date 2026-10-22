# 1022 期 A 组证据记录

采集范围仅限派工列出的三篇官方页面。级别均为 L1（官方发布）。页面标示的发布日期均为 2026-10-21，均未给出具体发布时间；本表状态只表示官方页面是否支持对应说法，不等于独立验证。

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| A1：Claude Duet 研究预览从美国 Pro、Max 用户开始，须候补申请 | L1 | https://claude.com/resources/articles/claude-duet | “starting with US Pro and Max subscribers.” | 01-duet-raw.png | Anthropic 官方页日期 2026-10-21；页面说的是候补研究预览，不表示所有用户已开放；官方未给时刻。 | 已找到 |
| A1：Duet 纳入 Pro/Max 计划额度，与 Claude 其余使用共享额度 | L1 | https://claude.com/resources/articles/claude-duet | “same usage limits as the rest of Claude.” | 01-duet-raw.png | 后文还说明后台检查计入额度；金融模型、演示文稿等较大任务需要更多模型轮次和后台工作。官方未报告具体消耗量。 | 已找到 |
| A1：连接账户每数小时检查一次 | L1 | https://claude.com/resources/articles/claude-duet | “checks your connected accounts every few hours” | 02-duet-controls-raw.png | 原文把账户检查频率与通知频率分开：用户可设置希望接收什么、多久收到通知；没有说用户可设置账户轮询间隔。Anthropic 称持续检查会更快消耗计划额度。 | 已找到 |
| A1：发送消息、付款、签署文件或删除前会等待批准 | L1 | https://claude.com/resources/articles/claude-duet | “waits for your approval.” | 02-duet-controls-raw.png | 需批准的动作例子是发送消息、付款、签署文件、删除；证券交易及账户间转账即使获批也禁止。页面另称最敏感级信息会拒绝发送，健康或个人财务信息仅在明确指示时发送。 | 已找到 |
| A2：广告流程使用自定义、后训练的前沿 LLM | L1 | https://openai.com/index/building-chatgpt-ad-stack/ | “custom, post-trained frontier LLM” | 03-ads-raw.png | OpenAI 工程文章对自家架构的描述；该模型发起检索、排序、相关性评估和竞价工具调用。此处是公司自述，不是独立审计。 | 已找到 |
| A2：广告系统跨轮次与跨多段对话保留信息状态 | L1 | https://openai.com/index/building-chatgpt-ad-stack/ | “information is remembered across turns of a conversation and across multiple conversations” | 03-ads-raw.png | 原文限定为广告系统的 stateful ads sessions；不应扩写成 ChatGPT 答复模型本身的记忆，或推断其记忆的全部数据范围。 | 已找到 |
| A2：“独立模型/服务” | L1 | https://openai.com/index/building-chatgpt-ad-stack/ | “separately from ChatGPT’s answer generation, in its own service” | 03-ads-raw.png | 页面明确称广告选择与答复生成分开运行、位于自己的服务；同时提及自定义后训练 LLM。页面没有说明该 LLM 与答复模型使用不同权重，因此“独立服务”有支持，“独立模型”需收窄或另找证据。 | 部分支持 |
| A3：ChatGPT Ads 新增 26 个国家，覆盖总数为 89 国 | L1 | https://openai.com/index/chatgpt-ads-expands-26-new-countries/ | “26 additional countries”; “89 countries worldwide” | —（派工要求仅存档核验） | OpenAI 官方页日期 2026-10-21；26 是本次增加数，89 是扩展后的总覆盖数，不是本次新增数。正文举例但未列出完整 26 国名单；官方未给时刻。 | 已找到 |
| A3：广告仅在 Free、Go 计划展示；Plus、Pro、Enterprise 无广告 | L1 | https://openai.com/index/chatgpt-ads-expands-26-new-countries/ | “only on the Free and Go plans”; “Plus, Pro, and Enterprise subscriptions will remain ad-free.” | —（派工要求仅存档核验） | 只覆盖文章明确点名的计划；不要据此扩写未提及计划。官方未给具体生效时刻。 | 已找到 |

## 可能的吠点

- Anthropic 将“每数小时检查连接账户”与“用户可设置通知频率”分别表述；不要把后者说成用户可调轮询间隔。来源：Duet 官方页上述机制段。
- 共享计划额度包含后台检查；较大任务需要更多模型轮次和后台工作，因此“共用额度”不代表各类任务消耗相同。来源：Duet 官方页预览说明。
- 审批不是所有读取或草拟动作都要等用户确认；文章列出发送消息、付款、签署文件、删除等动作，并指出证券交易和账户间转账即使获批也不允许。来源：Duet 官方页上述机制段。
- Anthropic 明说这是早期预览，可能误判线程或紧急程度；个人差事能力较弱，当前还不能查看其浏览器。来源：Duet 官方页预览说明。
- OpenAI 称广告选择服务与答复生成分开，但同一段也说广告流程读取当前轮次与对话历史；“服务分开”不等同于广告不使用对话上下文。是否采用独立模型权重，本文未说明。来源：OpenAI 工程文章首段。
- OpenAI 给出新增 26 国及扩展后 89 国的总数，但正文只列举部分市场，未列完整名单。来源：OpenAI 广告扩展页。

## 夸大说法实例

本组按派工只核三篇指定官方 URL，未搜索媒体或社区夸大实例；不能据此断言不存在相关说法。留待主代理决定是否另行核查。

## 扫描说法勘误

- A1 的预览、美国 Pro/Max 候补、共享计划额度、每数小时账户检查及敏感动作需批准，均能在 Anthropic 页面找到支持；通知频率与账户检查频率须分开表述。
- A2 的跨轮次/跨对话状态、定制后训练 LLM、独立于答复生成的广告服务均是 OpenAI 的架构自述；“独立模型”没有被页面明确证明，建议正文只写“独立服务”和“自定义后训练 LLM”，不要推断模型权重独立。
- A3 的“新增 26 国、总覆盖 89 国”及所列计划范围均与 OpenAI 页面一致。完整国家名单未见于该页正文。
- 交接材料提到扫描摘要可能有误，但本组未收到 ChatGPT、WorkBuddy 两份扫描原文；以上勘误只对照了交接表内的三条说法。

## 社区质疑线索

按派工范围仅取指定官方 URL，未搜索 Hacker News、Reddit 或其他社区评论；本节不提供 L6 线索。