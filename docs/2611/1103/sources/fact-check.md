# 1103 期事实清单

时间一律北京时间。证据细节见 `evidence-A.md`（腾讯 Hy4 与 Kimi 权重）、`evidence-B.md`（Postman 插件）、`evidence-c.md`、`evidence-D.md`（速览）；截图在 `images/`（“图 NN”指取证截图编号）。取证在北京 11-04 20:46–21:30；Claude 21:32 用 Hugging Face API 再查一次 `moonshotai` 与 `tencent`（结果同 A 组）。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：腾讯最强开源没权重？软件偷装AI插件？ | — | ✅ | 见 A、B 节 | 读者稿检查后由 titles.md #1 改为 #8。前问：腾讯官方稿称“跻身最强开源模型之列”，权重未发布（A 节吠点①）；后问：“软件”= Postman，问的是 HN 标题 “Postman silently auto-installs AI plugins”一类说法（图 24），“偷装”对应 silently；Postman 方称下载页与更新流程有提示（B 节吠点③），故用问号 |
| 腾讯发布大模型 Hy4，自称“跻身最强开源模型之列”，模型本体（权重）却只说“近期”开放 | L1 | ✅ | 见 A 节 | “近期”出自 tencent.com 简体稿（图 05a）；混元 Blog 写“我们计划开源模型权重”，无时间词；英文稿 “in the coming days” |
| 有开发者反映，接口调试软件 Postman 的桌面版升级时自动给 Codex、Claude Code 装插件 | L6 | ⚠️ | 见 B 节 | 用户报告，句中署“有开发者反映”；两名用户日志一致，无受控复现（反向核验第1轮 BLOCKER 1 后改写） |
| 网传它“共享所有 API 数据”，目前没见证据 | L6 | ⚠️ | 见 B 节吠点① | “网传”= HN 提交者标题与其 X 帖；“没见证据”的检索范围见 B 节吠点① |

## A Hy4 称“最强开源”，权重没放

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 北京11月4日中午，腾讯混元发布大模型 Hy4 | L1 | ✅ | 腾讯新闻腾讯官方账号 https://news.qq.com/rain/a/20261104A074Z600 （`a-` 存档，图 01）；tencent.com 中英文稿（图 05、05a） | 腾讯新闻版 12:52；IT之家 12:59:42 已转；官网稿与混元 Blog 只给日期。WorkBuddy 扫描称 14:06，与原文不符（evidence-A 勘误） |
| 首次能看图、操作电脑 | L1 | ✅ | 腾讯新闻版 “Hy4 首次引入视觉能力”；“Hy4 也首次支持电脑操作（Computer Use），可以结合图形界面与命令行”（evidence-A A5-5） | 官方自述，无独立评测 |
| 价格与8月的预览版相同 | L1 | ✅ | 腾讯新闻版 “性能显著超过 Hy4 preview 的同时定价维持一致”；混元 Blog 输入6元、输出18元、缓存命中0.3元/百万 Tokens（A5-6） | 预览版 HF 仓库 2026-08-27 创建、腾讯官网 8/28 发稿；TokenHub 至12/31 五折为限时优惠 |
| 官方稿标题称它“跻身最强开源模型之列” | L1 | ✅ | 腾讯新闻版标题“Hy4正式发布，跻身最强开源模型之列”；tencent.com 简体稿标题“腾讯混元发布Hy4，跻身最强开源模型之列”（图 05a） | — |
| 正文称其在金融众测中“综合表现位居开源模型第一” | L1 | ✅ | 腾讯新闻版、混元 Blog 金融段（图 04、04a）“基于 579 个真实金融任务的众测显示，Hy4 综合表现位居开源模型第一” | 第1轮 BLOCKER 6 后补“在金融众测中” |
| ①腾讯官网稿只写“将于近期开源模型权重” | L1 | ✅ | tencent.com 简体稿首段 “Hy4 将于近期开源模型权重，发布模型系统卡片”（图 05a，批注图 hy4-open-soon） | 腾讯新闻版无此句；英文稿 “plans to release model weights and system cards in the coming days”；Blog “我们计划开源模型权重……届时也会发布模型系统卡片”，无时间词；路透 “later”。各版本都没给日期（evidence-A A2） |
| ①截至北京4日21:30，Hugging Face 上只有8月的预览版 | L1 | ✅ | HF API `author=tencent`（A 组 20:46–21:28 多次、Claude 21:32）：Hy4 只有 `tencent/Hy4-preview` 与 `-FP8`；图 08、09 | 另查 GitHub `Tencent-Hunyuan`、ModelScope 均无 Hy4 正式版（GitCode 为 SPA 壳，未验证）；正文只写 Hugging Face |
| ②这个金融众测由腾讯与自家金融科技团队组织 | L1 | ✅ | 混元 Blog “我们联合腾讯金融科技 FiT 开展了 213 位专家的金融众测……基于 579 个真实金融任务的众测显示，Hy4 综合表现位居开源模型第一”（图 04a，批注图 hy4-finance-first）；英文稿 “conducted by the Tencent Hy team in partnership with Tencent Financial Technology” | 213 位中153位来自外部金融机构；Blog 另有 TenureTrail（腾讯内部基准）“位居开源模型第一”，与该句不是同一句 |
| ②只和3个开源模型比 | L1 | ✅ | 同上括号：Hy4 3.04/4；DeepSeek-V4.1-Flash-1017 2.97/4；Kimi K3.1 Pro 2.68/4；GLM-5.3 2.68/4 | — |
| ②官方自己的对比图里，Hy4 在12项测试中都低于 Claude-Opus-5.5 | L1 | ✅ | 腾讯新闻版与 Blog 同图（图 02，批注图 hy4-bench-chart） | Claude 逐项复核读数：30.0/56.5、31.0/46.4、37.1/43.4、44.4/66.4、41.0/43.4、56.2/58.8、70.0/80.0、59.9/60.7、35.2/44.2、19.8/56.2、53.6/59.0、78.9/81.8，12/12 低于；同图与 Kimi K3.1 Pro 6:6。图中4项标 (internal)：3DArena-Modeling、E-Bench 2.0、Hy-BrowseComp 2.0、TenureTrail（Claude 看图核对；第1轮 SHOULD_FIX 11） |
| ③拿来比的 Kimi-K3.1-Pro 也没放权重 | L1 | ✅ | HF API `author=moonshotai`：最新 `moonshotai/Kimi-K3`（2026-06-13）；`Kimi-K3.1-Pro`、`Kimi-K3.1` 均 401（图 10） | A 组 20:46、21:12、21:24、21:28，Claude 21:32；GitHub `MoonshotAI` 亦无 |
| ③月之暗面10月称最迟11月4日公开 | L1 | ✅ | https://www.kimi.com/en/blog/kimi-k3-1-pro Availability 节 “Full model weights will be released by Nov 4, 2026.”（图 11，批注图 kimi-by-nov4）；摘要段 “The full model weights will be released by Nov 4.”；官方 X 帖北京 10-24 01:15 “Open weights by Nov 4, 2026.” | 博客 10-24（北京）；1023 期已核“by”含当日、未给时刻与时区；第1轮 SHOULD_FIX 7 后由“11月4日前”改为“最迟11月4日” |
| ③官方没给时区，按美国时间，4日要到北京5日下午才过完 | L1＋推算 | ✅ | — | 美东 EST（UTC-5）11/4 24:00 = 北京 11/5 13:00；太平洋 PST（UTC-8）= 北京 11/5 16:00。北京时间 11/4 当天到 24:00。**发布前须再查 HF**，若已放出则改写本条 |

## B 开发者称 Postman 自动装插件

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 10月下旬，有开发者在 GitHub 反映 | L6 | ✅ | https://github.com/postmanlabs/postman-app-support/issues/14370 （`b-issue.json`，图 13） | 帖子存在与发帖时间已核：创建 2026-10-25T19:03:26Z（北京 10-26 03:03）；仍 open；作者 indicava，macOS，Postman 12.33.4 |
| 接口调试软件 Postman 的桌面版一升级，就给 Codex、Claude Code 装上自家插件 | L6 | ⚠️ | issue 日志 “{"source":"auto-existing-upgrade","results":["claude-code:installed","codex:installed"],…}”；“autoInstall.plugin.existing-user finished”（图 14，批注图 postman-auto-install） | 用户报告，句首署“有开发者在 GitHub 反映”；作者日志 2026-10-24T09:34Z；“Observed sequence (not yet reproduced on a clean Mac)” |
| 另一名用户也贴出了同样的安装记录 | L6 | ⚠️ | issue 评论 ronthedrummer（#issuecomment-6618575621）：12.31.3→12.35.1，日志 2026-11-03T22:58–22:59Z（北京 11-04 06:58–06:59），`auto-existing-upgrade` 装入 CLI 与 Claude Code、Codex 插件 | 评论存在已核；其 config.toml 无 `hooks.state`/`trusted_hash` |
| Postman 文档写明，它的安装程序会替用户批准插件在 Codex 里的启动脚本 | L1 | ✅ | https://learning.postman.com/docs/postman-plugin/install/detected-agents/ “On Codex, the plugin adds a session-start hook that guides Codex to Postman’s skills. The installer approves the hook for you.”（图 23，批注图 postman-docs-approve）；README “There is no separate hook prompt”（图 22）；源码 codex-hooks.ts L25–28、L85 写 `hooks.state…trusted_hash`，codex.ts L196 调用，固定 commit 6c8e452e（图 18、19） | “启动脚本”= session-start hook。只写文档所述功能，不写桌面升级时一定执行成功（第1轮 BLOCKER 1） |
| 用 Codex 自己的方式装，则要用户自己审核 | L1 | ✅ | detected-agents “If you install Codex with its own method instead, you review and approve the hook yourself.”（图 23）；per-agent “The hook installs untrusted. The next time Codex starts, it shows "Hooks need review."” | — |
| ①Hacker News 转帖标题说它“试图共享你所有的 API 数据” | L6 | ✅ | HN 50354960 标题 “Postman silently auto-installs AI plugins and attempts to share all API data”（图 24，批注图 postman-hn-headline） | 标题存在已核；提交者 sidgarimella，北京 11-04 01:22:37；同日 X 帖 “literally intercepts and sends all of your API data”（北京 11-04 02:31）；HN 1354–1359 分 |
| ①发帖人却写明，报告没说数据被传了出去 | L6 | ✅ | issue 正文 “This report establishes unwanted integration and instruction changes; it does not claim to establish data exfiltration.”（图 16，批注图 postman-no-exfil） | 原文存在已核 |
| ①截至北京4日21时，帖子评论和报道里都没见这类证据 | 本号检索 | ✅ | evidence-B B5、B7-2 | 范围即句中所写：issue 48 条评论（21:11）、HN 主帖与 50360947（单人自述无抓包）、The New Stack（“the GitHub report provides no evidence”）、DEV；Reddit 登录墙未取。第1轮 BLOCKER 5 后加时间与范围 |
| ①启动脚本的作用是让 AI 遇到接口任务先用 Postman 的工具 | L1 | ✅ | hooks/session-start-context.md（固定 commit）“Before responding to any non-trivial API engineering task … load the `postman:api-engineer` skill and follow it.”（图 21）；文档 “The hook makes Codex reach for Postman’s skills more reliably.”（图 23） | 末段称用户指令优先 |
| ②Postman 的更新说明只写了随软件装命令行工具，没写明升级会把插件装进你已有的 Codex、Claude Code | L1 | ✅ | https://www.postman.com/release-notes/postman-app/ ：12.31.7（10/9）“When you install or update the Postman app, the Postman CLI is now installed with it.”；12.30.4（9/30）“connect Claude Code, Codex, Cursor, or OpenCode directly to Postman with the new Postman plugin”（需用户自己连接）；12.35.2（11/4）只写 “Some critical bug fixes” | “更新说明”= 发行说明。下载页现写 “The Postman app includes Postman CLI & Postman plugins”（10-26 Wayback 快照无此句，仅一份快照）；官方口吻评论称更新流程里有提示，升级弹窗未核 |
| ③GitHub 上用官方口吻回复、说会改进的账号 | L6 | ✅ | zecoquit 评论（#issuecomment-6618267843，北京 11-04 07:36:43）“We’re going to make the opt-in clearer, especially for existing users, and we’re reviewing the hook approval and session-start behavior you flagged.”（图 20，批注图 postman-official-voice） | 评论存在已核；👍4 👎42 😕2 |
| ③资料里没有 Postman 身份标识 | L1（GitHub API） | ✅ | `b-user-zecoquit.json`：name/company/bio 全空、0 公开仓库；authorAssociation NONE；不在 postmanlabs 25 名公开成员中 | 只说明身份未能确认，不写“不是 Postman 员工” |
| ③截至4日21时，Postman 博客和更新说明里也没有回应 | 本号检索 | ✅ | `b-blog.txt`、发行说明、`b-status.txt`；插件仓库评论后无提交（`main` 9d5544f）；The New Stack “The company had not responded by the time of publication.” | 范围即句中所写（博客、更新说明）；官方 X、社区论坛、Discord 未查，正文不写“官方没回应”（第1轮 BLOCKER 5） |
| ③发帖人也没在全新的电脑上重现 | L6 | ✅ | issue 正文 “Observed sequence (not yet reproduced on a clean Mac)” | 原文存在已核 |

## 批注译注

| 图 | 原文 | 译注 | 备注 |
|---|---|---|---|
| hy4-open-soon | “Hy 首次跻身最强开源模型之列” | 原文照录 | 中文原文，rects 定位 |
| hy4-open-soon | “Hy4 将于近期开源模型权重，发布模型系统卡片” | 原文照录 | 第1轮 SHOULD_FIX 9 后删去“没给日期” |
| hy4-finance-first | 联合腾讯金融科技 FiT 开展 213 位专家的金融众测 | 同左 | 中文原文 |
| hy4-finance-first | 基于579个真实金融任务的众测……第一（四个分数） | 原文照录，含四个分数 | 第1轮 SHOULD_FIX 9 后删去“对照只有……三个” |
| hy4-bench-chart | 图例 Hy4 / Claude Opus 5.5 | 图例：蓝色实心柱为 Hy4，橙色星标为 Claude Opus 5.5 | 读数见 A 节②；比较结论只在吠点卡 |
| kimi-by-nov4 | Full model weights will be released by Nov 4, 2026. | 完整模型权重将最迟于2026年11月4日发布 | “by”含当日；Kimi 中文页写“11月4日前”（第2轮反向核验指出），批注对应英文页 |
| postman-auto-install | Postman's own log explicitly records automatic installation | Postman 自己的日志明确记下了自动安装 | 小标题是发帖人写的 |
| postman-auto-install | {"source":"auto-existing-upgrade","results":["claude-code:installed","codex:installed"],…} | 安装来源 auto-existing-upgrade（现有用户升级时自动安装）；Claude Code、Codex 插件均已安装 | rects 定位代码行 |
| postman-docs-approve | The installer approves the hook for you. | 安装程序会替你批准这个钩子 | — |
| postman-docs-approve | If you install Codex with its own method instead, you review and approve the hook yourself. | 如果改用 Codex 自己的方式安装，就由你自己审核并批准这个钩子 | — |
| postman-hn-headline | Postman silently auto-installs AI plugins and attempts to share all API data | Postman 悄悄自动安装 AI 插件，并试图共享所有 API 数据 | rects 定位 |
| postman-no-exfil | This report establishes unwanted integration and instruction changes; it does not claim to establish data exfiltration. | 这份报告确认的是不想要的集成与指令改动；它不主张已确认数据外传 | — |
| postman-official-voice | We’re going to make the opt-in clearer, … you flagged. | 我们会把选择开启（opt-in）讲得更清楚，尤其对现有用户；我们也在复查你们指出的钩子批准与会话启动行为 | — |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 苹果称部分 Apple Watch 将能回看前15秒对话文字，仅英语、中国不可用 | L1 | ✅ | Apple Support https://support.apple.com/en-us/148354 （Published November 03, 2026；`c-c1-*`）“Live Rewind recalls what was just said in the last 15 seconds by showing a text snippet on your Apple Watch when you double-press the Digital Crown”；“These features will be available in beta later this year with watchOS 27.3.”；隐私概览 “Audio is not recorded or stored.” | “新款”= Apple Watch Series 12、Ultra 4（另需 iPhone 16 或更新）；逐项 opt-in；仅英语；初期欧盟不可用、中国不可用；18岁以下不可用（MacRumors 写13岁，以苹果为准）。The Verge、MacRumors 称 27.3 测试版已推出（L4），苹果原文为将来时，第1轮 BLOCKER 4 后改写：苹果原文为将来时（“will be available in beta later this year”），故写“将能”；“部分”= Series 12、Ultra 4（第2轮视觉复核后由“指定款”改，以腾出“仅英语”）；“Siri Recap and Live Rewind will be available in English to start”；“Siri Recap and Live Rewind are not available in China.” |
| 苹果向所有开发者开放 Siri AI 预览，接入的 App 春季起可提交审核 | L1 | ✅ | Apple Developer https://developer.apple.com/news/?id=gdk2umrf （11-03，未给时刻）“a new Developer Preview enables any developer to integrate their app with Siri AI”；“with submission to the App Store opening up in the spring” | 预览随 iOS/iPadOS/macOS/visionOS 27.3 测试版；“spring”未写年份；第1轮 SHOULD_FIX 10 后“上架”改“提交审核”；欧盟 iOS、iPadOS、watchOS 暂不可用；预览功能不对 TestFlight 外部测试与 App Store 版本开放 |
| Google 称中小学生账号的 Gemini 与搜索将默认关闭，需管理员开启 | L1 | ✅ | https://blog.google/products-and-platforms/products/education/ai-learning-era-updates-2026/ （11-03，未给时刻；`c-c3-google-blog-edu.txt`）“Access to these products will be entirely off for K-12 student accounts until school administrators explicitly turn them on.” | “中小学生账号”= K-12 student accounts；“管理员”= school administrators。产品为 Gemini、Gemini Notebook、Gemini in Classroom、Google Search 四项；原文为将来时（“will be entirely off”），细节“in the coming months”，官方未给生效日期（第1轮 BLOCKER 2 后加“将”）；NYT 称“early next year”为媒体说法，不写。HN 50353192（NYT 链接）632 分 436 评 |
| AMD 第三季度营收134亿美元，数据中心营收同比增长97% | L1 | ✅ | https://ir.amd.com/news-events/press-releases/detail/1303/amd-reports-third-quarter-2026-financial-results （北京 11-04 05:15）“Third quarter revenue grew 45% year-over-year to a record $13.4 billion”；“Data Center segment revenue was a record $8.5 billion, up 97% year-over-year” | 表内 13,377、8,546（百万美元）。电话会转写（stockanalysis.com，第三方）称 2027 年营收增长 “more than 80%”，未核官方回放，不写 |
| 不收 | — | — | — | Anthropic FlashProve（第2轮反向核验：证明限于整数模型、贪心解码、1,024-token 上下文，40 字装不下这些条件）；Redwood 博客 Alec Harris 文（第2轮反向核验：作者建议属评论，不合速览只收事实）；Hy4 的 TenureTrail 等其他“第一”（篇幅）；AIHOT 把作者写成 Redwood（圈内）；日经3.2万亿美元（付费墙）；Codex 训练开关传闻（无官方证据）；Grok 额度（用户自测）；甲骨文与腾讯（风控）；额度重置 |

## 反向核验

- 第1轮（Codex gpt-6-astra medium，`barking review --reverse --run`，报告 `/tmp/barking-review/1103/reverse-report-2.md`，北京 11-04 约22:30）：BLOCKER 6、SHOULD_FIX 5，Claude 逐条回原文核实，均成立，全部按最小改法修改：
  1. 省流与省流卡把 Postman 用户报告写成普遍事实 → 省流加“有开发者反映”，省流卡改摘事实段“有开发者在 GitHub 反映……”；事实段把“标成已信任”改为 Postman 文档所述安装程序功能（L1），不写升级时一定执行成功。
  2. Google 速览写成现状 → 加“将”。
  3. FlashProve 速览把单 token 证明写成整段回答、倍率缺条件 → 改为“可核验回答中某个 token 是否出自所承诺的模型”，删倍率。
  4. Apple Watch 速览丢可用范围 → “新款”改“指定款”，加“中国不可用”，“年内测试版”改“将能”。
  5. “官方至今没回应”“没人拿出证据”超出检索范围 → 两句都加“截至北京4日21时”与所查渠道；账号改为“资料里没有 Postman 身份标识”。
  6. 事件卡丢“金融众测”范围 → 事实段改为“正文称其在金融众测中……”，吠点②相应改写。
  7. “11月4日前”排除当日 → 改“最迟11月4日”，加“官方没给时区”。
  8. L6 条目误标 ✅ → 用户报告的内容（自动安装、第二份记录、网传）改 ⚠️；“帖子确实这样写”的行保留 ✅ 并在备注写明只核了存在。
  9. 两条译注加了本号判断 → 删“没给日期”“对照只有……三个”与对比图的比较结论，译注只留原文。
  10. Siri“上架”应为“提交” → 改“接入的 App 春季起可提交审核”。
  11. 内部基准数3与4矛盾 → Claude 看图核为4项，已改。
- 第2轮（同上，报告 `reverse-report-3.md`，北京 11-04 约23:00）：BLOCKER 1、SHOULD_FIX 5，均成立：
  1. 第二主帖小标题“Postman 升级，自动装 AI 插件”把用户报告写成确定行为（速览、事件卡、吠点卡标题同源）→ 改“开发者称 Postman 升级自动装插件”，速览图放不下一行，再缩为“开发者称 Postman 自动装插件”（升级一事在事实段）。
  2. FlashProve 速览缺整数模型、贪心解码、1,024-token 条件 → 移出速览。
  3. Kimi 译注“11月4日前”未随正文同步 → 改“最迟于2026年11月4日”；另记 Kimi 中文页本身写“11月4日前”。
  4. Apple Watch 速览缺语言范围 → 加“仅英语”（18岁以下与欧盟初期不可用写在清单备注）。
  5. Redwood 条目是作者建议，不合速览只收事实 → 移出速览。
  6. evidence-A 第33行“3项 internal”与读数表4项矛盾 → 在 evidence-A 追加勘误。

## 视觉复核

- 第1轮（Codex gpt-6-astra medium，`barking review --visual --run`，26 张图，报告 `visual-report-2.md`，北京 11-04 约23:00）：BLOCKER 1、SHOULD_FIX 5。
  - BLOCKER：08-kimi-by-nov4 译注“11月4日前”→ 已改（同反向核验第2轮第3条）。
  - 07-hy4-bench-chart 第二个高亮右端碰到 GPT 图标 → rects 收窄到“5.5”止。
  - 02-roundup Apple Watch 缺“仅英语”→ 已加；AMD“数据中心业务”→ 改“数据中心营收”。
  - 07 对比图数值偏小、11/14/15 GitHub 截图字小：未改。对比图腾讯原图即为 1150 px 宽的12格小图，取不到更高清版本；拆格放大会把一张图拆成多张、版面按高度缩得更小。GitHub 截图为匿名无头 Chrome 925 CSS px 宽渲染，原文行宽固定，裁不窄。依据另有译注与清单读数。
