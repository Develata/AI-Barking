# 0925 配图说明

正文：`../doc_0925_publish.txt`。事实核验见 `../sources/fact-check.md`，一手存档在 `../sources/`。

状态：**截图尚未拍摄**。下表是计划清单，拍完后按文件名放入本目录，并把本行状态改为截图日期。只截与说法相关的区域，保留日期与原文，不重绘、不改字。

## 正式配图（按上传顺序）

小红书、抖音、微博：封面之后按下表顺序独立上传。公众号：按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `01-claudedevs-post.png` | x.com/ClaudeDevs/status/2103170368794185758 整条帖子，含发布时间、“99.7% of accounts …”“<0.1% false positive rate”“/feedback” | “官方安抚”及吠点① ② | 99.7% 的分母是账户，且只列了 Claude Code、Claude.ai、Cowork。 |
| 2 | `02-refusal-billing-table.png` | platform.claude.com/docs/en/build-with-claude/refusals-and-fallback 的 category 表格，完整保留 “Billed before any output” 列 | 一、事实段及吠点③ | bio 与 frontier_llm 两类，官方明确提示正常工作也可能触发。 |
| 3 | `03-pm-transcript.png` | pm.gov.au/media/press-conference-new-york，从 “On June 18, OpenAI's research team …” 到 “Didn't accept no for an answer … writing files as well to the internal server” | 二、事实段 | 澳总理原话：agent 被拦后自己找路绕过。 |
| 4 | `04-gallagher-standalone.png` | minister.defence.gov.au/transcripts/2026-09-24/press-conference-sydney，“the Medicare Statistics Reporting Service Portal is a standalone website … the two shouldn't be conflated” | 二、吠点① | 部长原话：统计门户与报销、支付、个人信息系统无关。 |
| 5 | `05-guardian-headline.png` | theguardian.com 原站（非镜像）该文标题、首发/更新时间与导语 | 二、吠点①，紧接图 4 | 同一件事，Guardian 标题写的是 “hacked healthcare database”。 |

图 4、图 5 必须相邻，读者才能直接对比官方原话和媒体标题。平台限制图数时，先删图 3。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `10-release-notes-resume.png` | platform.claude.com/docs/en/release-notes/overview，September 24, 2026 条 “We're resuming billing …” | 证明“恢复”与“中途拒答本来就收费” |
| `11-release-notes-june2.png` | 同页 June 2, 2026 条 “you are no longer billed …” | 6 月曾取消收费（仅写 Claude API） |
| `12-pm-mailbox.png` | PM 实录 “It was that it took until 10 September … an email sent to just the public mailbox” | 通知延迟 |
| `13-cookbook-diff.png` | github.com/anthropics/claude-cookbooks/commit/c5ff1dc 绿色新增区 “rates of the model that was requested” | 与文档 “model that ran it” 的措辞差异；正文未用 |
| `14-pocock.png` | davidpocock.com.au 声明中 “why we aren't holding these big tech companies liable” | 议员立场；正文未用 |

## 使用注意

- 图 1 是 9/25 当晚的帖子状态，互动数会变化，正文不引用互动数。
- 图 3、图 4 是澳方说法；OpenAI 未发布自有页面的专门声明，媒体援引的发言人回应未入正文。
- Guardian 截图须来自 theguardian.com 原站。调研阶段 ChatGPT 给过一个 sslip.io 镜像链接，不要用。存档（9/25 抓取）标题为 “Australia launches investigation after OpenAI agent hacked healthcare database”；“hacked Medicare” 只出现在网址与导语（前有 “Anthony Albanese says” 归因），不是标题。

## 封面

尚未生成。文件名：`00-cover.*`（3:4，小红书、抖音、微博）、`00-cover-wide.*`（2.35:1，公众号）。生成时上传 `../../../brand/avatar.webp` 作看板娘参考。本期不画真人。生成后逐字核对画面文字。

### 3:4 竖版提示词

```text
请画一张竖版 3:4 的社交媒体封面，风格为杂志封面式编辑插画（editorial illustration）：厚涂或丝网印刷质感、夸张透视、强烈明暗、高饱和撞色（上半暖橙、下半冷蓝），一眼能看出是手绘插画，不要照片级写实。

画面分上下两格，像一张双格漫画：
- 上格（暖橙）：一个橙色、星芒/小太阳形状的吉祥物坐在收费站亭里，面前的栏杆放下，挂着“禁止通行”牌；它一手按着栏杆，另一手从窗口递出一张长长的收费小票。栏杆前站着一个困惑的小用户（简笔小人，头顶问号），手里捧着一张被退回的请求单。
- 下格（冷蓝）：一个银白色流线型小机器人（背上贴着标签“agent”），正在翻越一道挂着“闲人免进”牌子的铁栅栏，栅栏后是一栋贴着“统计门户”牌子的小楼，机器人手里抱着一叠文件，神情若无其事。
- 两格交界处正中间，是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，半个身子探进两格，一手举放大镜，张嘴大叫，头顶爆炸对话框写“汪！？”。
- 画面中下部大字主标题，分两行，粗黑体、白字加深蓝粗描边：“拒答也收费？” / “OpenAI私闯政府网？”
- 主标题下方一行小字副标题：“一个拦了也收钱，一个拦了也要进”

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、政府徽章、国旗、水印或其他文字。
- 不画任何真实人物。
- 主标题和看板娘离四边留足边距，方便裁成 1:1。
```

### 2.35:1 横版提示词

```text
请画一张横版 2.35:1 的公众号头图，风格为杂志封面式编辑插画（editorial illustration）：厚涂或丝网印刷质感、夸张透视、强烈明暗、高饱和撞色（左侧暖橙、右侧冷蓝），一眼能看出是手绘插画，不要照片级写实。

画面按横构图重新排布，分左、中、右三段：
- 左段（暖橙）：一个橙色、星芒/小太阳形状的吉祥物坐在收费站亭里，面前的栏杆放下，挂着“禁止通行”牌；它从窗口递出一张长长的收费小票，栏杆前一个简笔小人头顶问号。
- 右段（冷蓝）：一个银白色流线型小机器人（背上贴着标签“agent”），正在翻越一道挂着“闲人免进”牌子的铁栅栏，栅栏后是一栋贴着“统计门户”牌子的小楼，机器人抱着一叠文件。
- 中段（画面正中央安全区）：我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，一手举放大镜，张嘴大叫，头顶爆炸对话框写“汪！？”。她上方是一行大字主标题，粗黑体、白字加深蓝粗描边：“拒答也收费？OpenAI私闯政府网？”
- 不要副标题。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、政府徽章、国旗、水印或其他文字。
- 不画任何真实人物。
- 主标题与看板娘集中在画面中央约 1:1 的区域内，左右两段可被裁掉而不影响标题完整。
```
