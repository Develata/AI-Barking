# 1126 配图说明

上传：手动上传竖版首图时请先选 `publish/00-cover.png`、再选 `publish/00-cover-wide.png`；其余 `01` 至 `14` 可全选并按文件名升序依次上传（注意按文件名升序时 `publish/00-cover-wide.png` 会排在 `publish/00-cover.png` 前）。小红书、抖音按全部顺序上传；公众号同序但不插事件卡与吠点卡（03、04、09、10）。卡片内容见 `cards.toml`，由 `barking card` 渲染。

## 封面

封面抽签：画风=拟物版面；底色=满幅场景；标题=顶部横排；看板娘=角标；动作=反应

- **抽签与工具**：`barking cover-draw docs/2611/1126` 种子 `1795495571012149272`；Codex `v0.167.2`（`gpt-6.2-sol` + 内置 `imagegen`）生成。首轮对比 A/B 后选用 B 组（拟物面板）；二轮视觉/反向复核修订主标题并避免绿灯大对勾误导，按同一 B 组抽签轴重新生成新版 B 组（累计第 2 次，3:4 竖版 `1086×1448` 与 2.35:1 横版 `1922×818`），再用 `ffmpeg` 将仓库根目录通用头像原图裁为直径为短边 1/6 的圆形角标叠至右下角。
- **对比与验收**：放大逐字核对主标题「Gmail接口已列发信，默认没确认发不了」（19 个汉字/字母/逗号；按标点单计共 19–20 口径以内）与数字「23」「30」全部锐利无错漏、无多余英文或 logo；右侧「30」面板仅保留半开透明防护罩下的小锁与手按确认键及一旁实体工具清单卡（无绿灯、无绿色对勾）；横版居中 1:1 裁切框完整包含标题铭牌、两块面板与清单卡；右下角圆形角标位于空桌角未遮挡主体；与前 3 期（1123 绘本浅色书房、1124 立体泥塑红金台阶、1125 复古橘色加油机）显著区分。
- **选中与落选理由**：最终选用 **B 组（第 2 版）**（拟物「23」与「30」两套并列工具面板＋「30」清单卡与未通电确认键，呼应主端点同轮返回 23/30 两种结果与“文档已列发信、默认未确认发不了”，不作新旧切换时间暗示，角标规避胸牌小字与占比特判）；**B 组第 1 版淘汰**（旧标题含「禁发信」且绿灯对勾易误读为已实测发信）；**A 组落选**（浅底接近 1123 且横版胸牌小字易糊）。

### 提示词（选中组：B 组第 2 版）

- **3:4 竖版**：
  - 画面：温暖木纹与浅灰工作垫铺满整幅画面的「满幅场景」，画风为质感细腻的「拟物版面」（实体桌面控制台与微缩立体信件机关）。画面正上方采用高对比白底深墨色简洁长条横牌「顶部横排」排版，清晰锐利排印主标题「Gmail接口已列发信，默认没确认发不了」（分两行居中不折字）。画面中下方并置两块带触感的实体金属控制面板，展示 23 与 30 两种返回结果：左侧金属面板标有数字「23」，上面印着信封图标与红色警示指示灯；右侧拉丝银控制台标有数字「30」，旁边摆着一张画有多行工具条目与信封浮雕图标的实体「30」工具清单卡片，控制台正中是一枚盖着半开透明防护罩、按钮上刻着小锁与手按确认图标的实体未通电按键（不要画亮起的绿灯、绿色拨动开关或绿色大对勾卡片），直观传达“清单已列发信工具、但默认须人工按键确认才发得出”。右下角留出一块干净柔焦的木纹桌面留白区域。
  - 硬性要求：画面里只允许出现这些文字：主标题，逐字为「Gmail接口已列发信，默认没确认发不了」；关键数字，逐字为「23」「30」。不要其他文字、数字、字母、logo、国旗、品牌标志或真人。画面里不画本号看板娘（蓝发、垂耳、鲸尾的女仆少女）。右下角留出直径约为画面短边六分之一的圆形空位，不放文字或物件。抽到的画风只用于场景与标题。主标题在手机缩略图下清楚可读。竖版：3:4 竖版，1080×1440 或同等比例。
- **2.35:1 横版**：
  - 画面：温暖木纹与浅灰工作垫铺满整幅画面的「满幅场景」，画风为质感细腻的「拟物版面」（实体桌面控制台与微缩立体信件机关）。独立宽幅横构图：正中方形区域顶部为高对比白底深墨色长条「顶部横排」标题区，清晰排印「Gmail接口已列发信，默认没确认发不了」；正中方形区域下方左右并排摆放两块拟物面板——左侧标「23」的红灯信封面板，右侧标「30」的拉丝银控制台配实体多行工具清单卡与半开透明防护罩下的小锁手按确认按键（不要亮绿灯，也不要绿色对勾）；左右两侧延伸为柔焦桌面边缘，主体与标题完全落在正中 1:1 方形区域内。右下角留出干净柔焦的桌面空白角标位置。
  - 硬性要求：画面里只允许出现这些文字：主标题，逐字为「Gmail接口已列发信，默认没确认发不了」；关键数字，逐字为「23」「30」。不要其他文字、数字、字母、logo、国旗、品牌标志或真人。画面里不画本号看板娘（蓝发、垂耳、鲸尾的女仆少女）。右下角留出直径约为画面短边六分之一的圆形空位，不放文字或物件。抽到的画风只用于场景与标题。主标题在手机缩略图下清楚可读。横版：2.35:1 横版，约 1920×816，独立横构图；主标题完整落在画面正中的方形区域内，便于再裁出 1:1 缩略图。

### 提示词（落选组：A 组——`封面抽签：画风=技术图版；底色=浅色；标题=手写字；看板娘=探头；动作=扮演`，旧标题版）

- **3:4 / 2.35:1**：浅米白工程图纸底色、墨蓝「手写字」主标题「报道称Gmail接口禁发信，文档已列发信」，左右并置红灯「23」信封模块与绿灯「30」列表图板，Q 版蓝发垂耳鲸尾女仆少女看板娘（胸前「AI 吠点」圆牌，不超过画面 1/10）自右下边缘「探头」手持探针指向右侧。因底色与 1123 偏近且横版胸牌小字易糊落选。

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条，逐字取自正文省流 |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋DeepSeek、3DCodeFactory、Gemini、SAP、Cogentic、Kimi、混元，共 9 条 |
| 3 | publish/03-event-1.png | 一、事实与对你 | 公众号不插 |
| 4 | publish/04-bark-1.png | 一、吠点①–③ | 公众号不插 |
| 5 | publish/05-ws-mcp-deadline.png | 一、事实（2027-02-01 强制期限与 99% 标准档） | googleblog.com 原文截图（底图 `ws-mcp-deadline-raw.png`，裁拼自 `16-ws-updates-blog.png`；像素框＋OCR） |
| 6 | publish/06-mcp-v1-v1self.png | 一、事实与吠点①（9 个 GA 服务器、30 工具 vs 25 工具） | developers.google.com MCP 工具总览（底图 `mcp-v1-v1self-raw.png` = `21-mcp-server-tools.png`；像素框） |
| 7 | publish/07-mcp-elicitation.png | 一、吠点②（send_message 人工确认弹窗与下拉持久选项） | developers.google.com Form Elicitation 指南（底图 `mcp-elicitation-raw.png`，裁拼自 `27-mcp-elicitation.png`；像素框） |
| 8 | publish/08-mcp-scaled-billing.png | 一、吠点③（标准档 vs 付费档 MCP 免费额度清零） | developers.google.com 定价与档位说明（底图 `mcp-scaled-billing-raw.png`，裁拼自 `22-quota-pricing.png`；OCR） |
| 9 | publish/09-event-2.png | 二、事实与对你 | 公众号不插 |
| 10 | publish/10-bark-2.png | 二、吠点①–③ | 公众号不插 |
| 11 | publish/11-fable-training-limit.png | 二、事实（训练限部分研发能力、推理不改、少数主题明面回退） | anthropic.com Fable-5.2 发布文（底图 `fable-training-limit-raw.png`，裁自 `01-anthropic-safeguards.png`；OCR） |
| 12 | publish/12-edwinarbus-reply.png | 二、事实与吠点①（员工回应、Fable-5 旧事） | `@edwinarbus` 回复（海外社媒原文截图；底图 `edwinarbus-reply-raw.png` = `09-edwinarbus-reply-v2.png`；像素框＋OCR） |
| 13 | publish/13-sweeney-clarify.png | 二、事实与吠点②（斯威尼原帖与澄清） | `@TimSweeneyEpic` 原帖与澄清（海外社媒原文截图；底图 `sweeney-clarify-raw.png` = `05-sweeney-posts-v2.png`；OCR） |
| 14 | publish/14-refusal-billing.png | 二、对你与吠点②③（正常 ML 误触、零输出前拦截计费） | platform.claude.com 拒绝分类与计费文档（底图 `refusal-billing-raw.png`，裁拼自 `02-platform-refusal-fallback.png`；OCR＋像素框） |

## 备用图

| 文件 | 内容 | 用途 |
|---|---|---|
| ws-mcp-deadline-raw.png、mcp-v1-v1self-raw.png、mcp-elicitation-raw.png、mcp-scaled-billing-raw.png、fable-training-limit-raw.png、edwinarbus-reply-raw.png、sweeney-clarify-raw.png、refusal-billing-raw.png | 批注底图 | 批注底图 |
| 01-anthropic-safeguards.png | Fable-5.2 发布文 AI 研发与反蒸馏全段 | 取证 |
| 02-platform-refusal-fallback.png | Platform 拒绝类别与计费规则页 | 取证 |
| 03-astropulse-post.png | `@RealAstropulse` 24k vs 4k 对比主帖 | 取证 |
| 04-astropulse-breakdown.png | `@RealAstropulse` thread 拆解 | 取证 |
| 05-sweeney-posts-v2.png | Tim Sweeney 首帖与澄清帖 | 取证 |
| 06-aa-default-fallback-v2.png | Artificial Analysis 榜首 `Default Fallback` 标注 | 取证 |
| 07-cn-blocktempo-coverage.png | 动区 11-27 报道“尚未回應” | 取证 |
| 08-hn-50664325.png | HN 50664325 标题与分数 | 取证 |
| 09-edwinarbus-reply-v2.png | `@edwinarbus` 两次公开回复 | 取证 |
| 10-support-switched-models.png | Claude Help Center 自动切换开关说明 | 取证 |
| 11-system-card-1-5.png | Fable-5.2 System Card §1.5 | 取证 |
| 12-skirano-donotsabotage.png | `@skirano` 原帖 | 取证 |
| 13-cn-ithome-sweeney.png | IT之家遗漏 Sweeney 澄清与机翻截图 | 取证 |
| 16-ws-updates-blog.png | Workspace Updates 公告 | 取证 |
| 17-ai-tools-addendum.png | Workspace AI Tools Addendum 全文 | 取证 |
| 18-gmail-mcp-ref.png | `gmailmcp.googleapis.com` 30 vs 25 工具列表 | 取证 |
| 19-gmail-send-message.png | `send_message` 官方工具页 | 取证 |
| 21-mcp-server-tools.png | 9 个 GA MCP 服务器工具数总表 | 取证 |
| 22-quota-pricing.png | REST 每日免费额度 vs Scaled MCP 零免费额度 | 取证 |
| 23-techstackups.png | TechStackups 原文 | 取证 |
| 25-hn-50653477.png | HN 50653477 与 `bugthe0ry` 指出已支持发信 | 取证 |
| 26-blockbeats.png | 动察 Beating 标题与正文 | 取证 |
| 27-mcp-elicitation.png | Form elicitation 与 `/mcp/v1self` 说明 | 取证 |
| 28-workspace-cli.png | 官方 `gwscli` 概览 | 取证 |
| 29-quota-roadmap.png | Usage tiers 里程碑表 | 取证 |
