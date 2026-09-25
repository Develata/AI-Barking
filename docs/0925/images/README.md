# 0925 配图说明

正文：`../doc_0925_publish.txt`。事实核验见 `../sources/fact-check.md`，一手存档在 `../sources/`。

状态：截图于 2026-09-26 凌晨（北京时间）由 Codex 从原站截取，日志见 `../sources/capture-log.md`。图 4、6 由 Claude 用 ffmpeg 无损裁剪/竖向拼接（灰色细线为拼接处），未缩放、改字；裁前原图在 `../sources/screenshots-full/`。图 5 从图 4 的同一张原图裁出。

## 正式配图（按上传顺序）

小红书、抖音、微博：封面之后按下表顺序独立上传。公众号：按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `01-claudedevs-post.png` | x.com/ClaudeDevs/status/2103170368794185758 整条帖子，含发布时间、“99.7% of accounts …”“<0.1% false positive rate”“/feedback” | “官方安抚”及吠点① ② | 99.7% 的分母是账户，且只列了 Claude Code、Claude.ai、Cowork。 |
| 2 | `02-refusal-billing-table.png` | platform.claude.com/docs/en/build-with-claude/refusals-and-fallback 的 category 表格，完整保留 “Billed before any output” 列 | 一、事实段及吠点③ | bio 与 frontier_llm 两类，官方明确提示正常工作也可能触发。 |
| 3 | `03-pm-transcript.png` | pm.gov.au/media/press-conference-new-york，从 “On June 18, OpenAI's research team …” 到 “Didn't accept no for an answer … writing files as well to the internal server” | 二、事实段 | 澳总理原话：agent 被拦后自己找路绕过。 |
| 4 | `04-gallagher-standalone.png` | minister.defence.gov.au/transcripts/2026-09-24/press-conference-sydney，“the Medicare Statistics Reporting Service Portal is a standalone website … the two shouldn't be conflated” | 二、吠点① | 部长原话：统计门户与报销、支付、个人信息系统无关。 |
| 5 | `15-marles-minor.png` | 同一记者会，代理总理 Marles：“the impact of this incident is relatively minor … No individual's medical data was accessed here” | 二、吠点① | 代理总理原话：影响相对较小，没有个人医疗数据被访问。 |
| 6 | `05-guardian-headline.png` | theguardian.com 原站该文标题、副标题与导语（已裁掉 Getty 人物照片） | 二、吠点①，紧接图 5 | 同一件事，Guardian 标题写的是 “hacked healthcare database”。 |

图 4、5、6 必须相邻，读者才能直接对比官方原话和媒体标题。平台限制图数时，先删图 3，再删图 5。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `10-release-notes-resume.png` | platform.claude.com/docs/en/release-notes/overview，September 24, 2026 条 “We're resuming billing …” | 证明“恢复”与“中途拒答本来就收费” |
| `11-release-notes-june2.png` | 同页 June 2, 2026 条 “you are no longer billed …” | 6 月曾取消收费（仅写 Claude API） |
| `12-pm-mailbox.png` | PM 实录 “It was that it took until 10 September … an email sent to just the public mailbox” | 通知延迟 |
| `13-cookbook-diff.png` | github.com/anthropics/claude-cookbooks/commit/c5ff1dc 绿色新增区 “rates of the model that was requested” | 与文档 “model that ran it” 的措辞差异；正文未用。代码字号 12 px 偏小，日期只显示 “yesterday”（实际 2026-09-24 17:05 UTC） |
| `14-pocock.png` | davidpocock.com.au 声明中 “why we aren't holding these big tech companies liable” | 议员立场；正文未用 |

## 使用注意

- 图 1 显示的“下午2:11 · 2026年9月24日”是截图浏览器所在时区的本地时间；原帖时间为 2026-09-24 17:11:05 UTC，即北京时间 9 月 25 日 01:11。浏览量会变化，正文不引用。
- 图 6 页面显示的是更新时间 “Wed 23 Sep 2026 22.06 EDT”，首发时间（17.11 EDT）截图中未出现。
- 图 3、图 4 是澳方说法；OpenAI 未发布自有页面的专门声明，媒体援引的发言人回应未入正文。
- Guardian 截图须来自 theguardian.com 原站。调研阶段 ChatGPT 给过一个 sslip.io 镜像链接，不要用。存档与截图（9/25–26）标题均为 “Australia launches investigation after OpenAI agent hacked healthcare database”；“hacked Medicare” 只出现在网址与导语（前有 “Anthony Albanese says” 归因），不是标题。

## 封面

已生成（2026-09-26）：[00-cover.png](00-cover.png)（1086×1448，3:4，小红书、抖音、微博）、[00-cover-wide.png](00-cover-wide.png)（1999×786，约 2.54:1，公众号）。上传顺序排在所有配图之前。生成时上传 `../../../brand/avatar.webp` 作看板娘参考；本期不画真人。

已逐字核对：主标题“拒答也收费？”“OpenAI私闯政府网？”、副标题“一个拦了也收钱，一个拦了也要进”（仅竖版）、“汪！？”“禁止通行”“闲人免进”“统计门户”“agent”“AI 吠点”圆牌，均与提示词一致，无多余文字、logo 或真人。

横版注意：生成比例约 2.54:1，比 2.35:1 更宽；公众号上传时用平台裁剪框裁到 2.35:1，标题完整。但标题横贯全宽，公众号再裁 1:1 缩略图时标题会被截断，只剩看板娘与“汪！？”；如需 1:1 缩略图带标题，需重新生成。

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
