# 1022 事实清单

取证日期：2026-10-23（北京时间）。本期号为1022，不表示新闻均发生于10月22日。✅表示已对照所列一手材料，不表示本号完成独立实验。

## 主帖

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic 称，Claude 为239个软件测试入口写出约1.8亿行证明代码，由专门的程序检查。 | L1 | ✅ | https://www.anthropic.com/research/formal-verification ; preflight-formal.txt | 原文239 entry points、180 million lines；专门的程序指Rocq证明检查器，名字移至图注和存档；覆盖123库，正文省略库数；代码行数不是定理数。本号未独立复跑。 |
| 发现错误就先修补，再证明改过的版本符合要求。 | L1 | ✅ | 同上；preflight-formal.txt | 原文patches后重新生成规格。未发现错误的入口无需先修补；不是说全部入口都有已知错误。 |
| 它要证明，所有符合条件的输入都不会触发约定的错误。 | L1 | ✅ | 同上；preflight-formal.txt | “符合条件”与后文运行条件相连；约定的错误指规格涉及的崩溃、内存安全检查错误与断言失败，不是所有错误类别。 |
| 证明只覆盖指定入口 | L1 | ✅ | 同上；preflight-formal.txt | 原文明确不意味着libraries contain no bugs；省流保留不能当成整套软件无漏洞保证。 |
| 输入须小于64 MB，程序单线程运行、静态链接、内存预算固定，还限定了 RISC-V 处理器和 Linux 系统模型 | L1 | ✅ | 同上；preflight-formal.txt | Introduction的less than64 MB；static binary、fixed budget与模型范围均保留，不推广全部x86/ARM或并行环境。 |
| 证明通过，规则也可能没写全，机器模型也可能与现实有差距。 | L1 | ✅ | 同上；preflight-formal.txt | 通俗改写spec完整性及机器模型/现实差距；不是推导已被证明有误。 |
| AI 获准先改代码再证明，可能为了过关改掉原本功能。研究方承认这点，用惩罚改动、记录和审计补丁等办法约束。 | L1 | ✅ | 同上；preflight-formal.txt | “改题过关”为脚注6奖励作弊风险的解释；仍保留“可能”，不说已发生。原文另奖励证明改变输入少。 |
| 软银公告：为处理 PHP 漏洞，旗下三个品牌暂停紧急灾害警报。涉及 SoftBank、Y!mobile、LINEMO 用户的地震、海啸和灾害避难信息。 | L1 | ✅ | https://www.softbank.jp/corp/news/info/2026/20261023_01/ ; preflight-softbank.txt | 紧急速報メール为运营商警报服务，不译作普通电子邮件，不推广全部通信。 |
| 暂停始于北京时间10月23日05:47，预计26日完成维护。 | L1 | ✅ | 同上；preflight-softbank.txt | 原文日本时间10月23日06:47；JST UTC+9转UTC+8减一小时，预计日期不是恢复事实。 |
| 官方建议临时使用 Yahoo! 防灾速报等服务接收信息。 | L1 | ✅ | 同上；preflight-softbank.txt | 仅转述公告建议，不声称替代服务覆盖和机制相同。 |
| 这份公告说的是漏洞维护，没有确认系统已被攻陷；26日也只是预计完成日期。 | L1 | ✅ | 同上；preflight-softbank.txt | 限定已读公告内容；不下“未遭入侵”的结论，不把预计日期写为恢复事实。 |
| Canva 发布自研图像模型 Craft，已在 Dream Lab 和公共 API 预览中提供。 | L1 | ✅ | https://www.canva.com/newsroom/news/canva-craft/ ; b-web-reader-archive.txt | 官网Availability与正文首段，官方社区补充API preview；不是本号实测可用。 |
| Magic Layers 可把生成图转成可编辑的设计元素 | L1 | ✅ | 同上 | 原文can transform；不是Craft直接输出原生分层设计文档。 |
| 官方援引 DesignArena 宣传自己是“第二名实验室”，但 Craft 这个模型排第四。 | L1 | ✅ | 同上 | 整句仍为官方公告排名口径，不把动态名次当本号独立当前排名；“排名对象换了”是对该口径的解释。 |
| Craft 的 MCP 接入仍待推出。 | L1 | ✅ | 同上 | 取证时官网Canva MCP (Coming Soon)；移到事实段，保留未推出状态。 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 国办文件鼓励企业探索建立失业缓冲期制度 | L1 | ✅ | https://www.gov.cn/zhengce/zhengceku/202610/content_7084108.htm ; preflight-policy.txt | 第六项，鼓励探索，不是统一强制缓冲时长。 |
| Replit 推出故障修复智能体，自动发布须用户开启 | L1 | ✅ | https://replit.com/blog/replit-monitoring-agent ; preflight-replit.txt | auto-publish开关与autofix不同；能力为厂商产品说明，未独立测成功率。 |
| Uber 将向超3万名员工提供 ChatGPT Work | L1 | ✅ | https://openai.com/index/uber-chatgpt-work-rollout/ ; preflight-uber.txt | will make available，面向资格，不是3万人已经使用或实证生产率。 |

## 译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 这些证明包含约1.8亿行 Rocq 代码。 | L1 | ✅ | preflight-formal.txt；01-formal-raw.png | about 180 million lines of Rocq，代码行数不是命题数。 |
| 假设程序是单线程静态二进制，使用固定内存预算，在采用 Linux 的 RISC-V 处理器上运行。 | L1 | ✅ | preflight-formal.txt；01-formal-raw.png | single-threaded static binary with a fixed memory budget on a RISC-V CPU using Linux OS；补出同句前部假设语义。 |
| 为应对 PHP 漏洞，正在紧急维护，并暂停向 SoftBank、Y!mobile 和 LINEMO 用户发送紧急警报。 | L1 | ✅ | preflight-softbank.txt；02-softbank-raw.png | 当前维护段原文完整直译；不扩大为全部通信。 |
| 维护预计于10月26日（周一）完成。 | L1 | ✅ | preflight-softbank.txt；02-softbank-raw.png | 预计限定词保留。 |
| Craft 由 Canva Research 从零构建。 | L1 | ✅ | b-web-reader-archive.txt；03-canva-raw.png | Craft was built from scratch by Canva Research，不译成全球首个。 |

## 反向核验记录

当前版本说明：用户随后要求主代理自审并优化文案；现稿900字，改写与逐句回查见 `self-review.md`。下列Astra报告对应改写前版本，不冒充当前900字稿的独立复核结果。

2026-10-23，gpt-6-astra 独立只读审读：PASS；0 BLOCKER、0 MAJOR、1 MINOR（Canva官方社区直接链接索引），已在来源README补齐。审查正文、卡片文字、事实清单与点名存档，不包括成图、联网状态或完整证明独立运行。报告见 `reverse-review-astra.md`。

Claude Sonnet CLI尝试未能运行：本地设置含无效权限规则；仅加载用户设置后，仍因OAuth会话已过期失败。失败回执保留在 `reverse-review-sonnet.txt`。因此本轮为Astra独立模型审读，不标为跨模型家族复核，也不把论文报道核验称为Rocq独立验收。

## 视觉复核

当前卡片说明：省流第一句随本次文案更新，其他卡片内容未改；渲染后五张均由主代理实际打开核对。下列Astra报告为本次更新前记录，最新自审见 `self-review.md`。

2026-10-23，gpt-6-astra独立逐张打开九张图，第一轮PASS，无BLOCKER/SHOULD_FIX；包含省流、速览、竖封面及三组批注图/raw，高亮和限定词均通过。第二轮实际打开最终横封面与819×819居中预览，PASS，无阻断项。记录见 `visual-review-astra.md`。不称跨模型家族核验；真实平台二次压缩未测。
