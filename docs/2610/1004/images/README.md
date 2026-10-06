# 1004 配图说明

正文：`../doc_1004_publish.txt`。来源见[来源入口](../sources/README.md)，事实核验见 `../sources/fact-check.md`，取证清单见 `../sources/evidence.md`，一手存档在 `../sources/`。卡片文字与批注内容写在 `cards.toml`，由 `barking card` 渲染。

状态：01–29 于北京时间 2026-10-04 由 Codex（gpt-6-luna max，四组并行）从原站截取（`../sources/capture-log.md`），2 倍像素比、视口约 700 CSS px。30 号起为 Claude 补抓或加工的底图与批注截图：30、35 的整页底图由 Claude 补抓，各 `-raw` 底图从 07、10、15、19、21 及 30、35 整页纵向或横向裁出，像素未改；裁切避开抓取浏览器右侧的翻译插件浮标（粉色圆形图标，不是原站内容）。`00-tldr.png` 为省流卡，`00-roundup.png` 为速览聚合图。图片不入库，发布后存 OpenList。

## 正式配图（按上传顺序）

小红书、抖音：封面之后按下表顺序独立上传。公众号：正文开头就是省流，默认不插省流卡，其余按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `00-tldr.png` | 省流卡（本号整理，非原文截图） | 省流 | 每条一句事实，逐字取自正文。 |
| 2 | `00-roundup.png` | 速览聚合图（本号整理，非原文截图） | 速览（不在正文） | 主帖三条标“详见前页”；其余五条为速览：额度重置两条、Altman、AA、Finances。 |
| 3 | `30-guardian-headline.png` | 《卫报》报道标题与正文开头（批注版） | 一、吠点① | ① 标题称“safety leader”；② 正文只写领导安全报告的撰写。 |
| 4 | `31-openai-slack-report.png` | OpenAI Slack 事件报告摘要（批注版） | 一、吠点② | ① 官方称不认为这起事件属于失准；② 涉事模型此前在其他方面存在失准。 |
| 5 | `32-gemini-access-table.png` | Google Gemini 帮助页：生效句与模型访问表（批注版） | 二、事实段与吠点①② | ① 10月9日先对无订阅用户生效；② AI Plus 收邮件；③ 无订阅只有 Flash-Lite；④ AI Plus 无 Pro。 |
| 6 | `33-kolibri-blog.png` | Aleph Alpha 博文开头（批注版） | 三、事实段 | ① 总参数78B、激活3B；② 最长1M token；③ Apache 2.0。 |
| 7 | `34-kolibri-context.png` | Kolibri 技术报告 p.41（批注版） | 三、吠点② | ① 1M 是训练长度之外的评测；② 训练序列最长256k token。 |
| 8 | `35-kolibri-p98.png` | Kolibri 技术报告 p.98（批注版） | 三、事实段与吠点③ | ① Qwen3.8 27B 综合分最高，激活参数近8倍；② 对比的 MoE 里 Kolibri 最高。 |

平台限制图数时，依次删图 7、6、4。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `30-guardian-headline-raw.png`、`31-openai-slack-report-raw.png`、`32-gemini-access-table-raw.png`、`33-kolibri-blog-raw.png`、`34-kolibri-context-raw.png`、`35-kolibri-p98-raw.png` | 6 张批注图的原始底图 | 批注底图（像素未改） |
| `30-guardian-headline-full.png`、`35-kolibri-p98-full.png` | 《卫报》页面前 1400 CSS px（含标题）；技术报告 p.98 整页 | 30、35 的整页原截图 |
| `36-kolibri-table-raw.png` | 技术报告表 28 表头与 Overall 两行 | 备用：Kolibri 75.5/70.8，Qwen3.8 27B 80.2/79.9；缩进 1080 宽后字太小，未做批注卡 |
| `01-atlantic-opening.png` | The Atlantic 署名文开头 | 付费墙遮挡后文，只有开头可读；与 30 对照 |
| `02-reuters-response.png` | Reuters 报道 | OpenAI 回应原句、“12 次前沿模型发布的安全报告” |
| `03-reports-index.png` | OpenAI 报告索引 | 三份报告均标 Oct 2 更新、事件日期 |
| `04-perl-report.png`、`05-eda-summary.png`、`06-eda-id-attempt.png`、`07-slack-summary.png`、`08-slack-cot-response.png` | 三份官方事件报告 | 04 复制源码（训练任务）；05–06 EDA 主机与 `id`、没拿到答案；07 为 31 的原截图，08 含 CoT “we may die” 上下文 |
| `09-marcus-post.png` | OpenAI 员工 Williams 的帖子卡片 | 原文 “We don’t consider this behavior misaligned”，无 “yet”；仅作核验，不入正文 |
| `10-gemini-access-table.png`、`11-gemini-rollout-scope.png`、`12-gemini-usage-limits.png`、`13-gemini-old-model-access.png`、`14-reddit-access-claim.png` | Gemini 帮助页整图、范围句、5月限额段，旧版另一页 Gemini 3 访问表（存档快照），网帖原文 | 10 为 32 的原截图；14 为“不付费完全不能用”的出处，来源页不写平台名 |
| `15-kolibri-blog-claims.png`、`16-kolibri-blog-benchmarks.png` | 博文整页与官方 benchmark 表 | 15 为 33 的原截图；16 博文表（含 AA-Omniscience −32.8） |
| `17-tech-report-abstract.png`、`18-tech-report-main-claims.png`、`19-tech-report-context-extrapolation.png`、`20-tech-report-baseline-protocol.png`、`21-tech-report-posttraining-table-1.png`、`22-tech-report-posttraining-table-2.png`、`23-tech-report-customer-proxies.png`、`24-tech-report-grounding-figure.png` | 技术报告摘要、主张、p.41 上下文段、八基线协议、表 28/29、客户代理、图 47 | 19 为 34 的原截图；21 含 Overall 与 AA-Omniscience 全行；其余备查 |
| `25-tibo-global-reset-announcement.png`、`26-tibo-reset-propagated.png`、`27-openai-banked-reset-eligibility.png`、`28-claude-reset-announcement.png`、`29-claude-reset-deadline.png` | Tibo 预告与确认帖、OpenAI 帮助页 banked reset 条件、ClaudeDevs 9/22 与 9/28 帖 | 速览“额度重置”两条的依据；X 帖卡片只在内部留存，不入卡片 |

## 封面

已生成（Codex CLI 内置图像生成，gpt-6-astra medium，参考图 `../../../../common_images/profile_picture.png`）：[00-cover.png](00-cover.png)（1086×1448，3:4，小红书、抖音）、[00-cover-wide.png](00-cover-wide.png)（1921×819，约 2.35:1，公众号）。上传顺序排在所有配图之前。发布时按平台要求声明 AI 生成。封面图不入库，发布后存 OpenList。

本期画风：**复古游乐园入场须知牌与票根**——“身高限制尺”隐喻模型访问档位，出口门隐喻辞职，奶油底、深青绿与番茄红双色、粗黑描边；看板娘戴验票员袖章、拿检票钳当质疑者。与 1003 作业本红笔、1002 航空邮件、1001 报刊社论漫画、0930 工程蓝图、0929 80 年代家电说明书区分开。

主体（做减法，三件物 + 看板娘）：
- OpenAI：卡通安全员（黄安全帽、反光背心，圆脸小人，非真人）抱纸箱走出出口门，对应标题“AI安全员辞职？”。
- Gemini：游乐园身高限制尺，三档横杠只有最低一档亮绿灯、另两档挂锁，旁边票根写“10月9日”，对应“免费Gemini没了？”与副标题“免费版没取消”（最低一档仍亮着）。
- Kolibri：蜂鸟（Kolibri 即德语蜂鸟）头戴歪歪扭扭的纸板皇冠，冠上画问号，隐喻“自封”，不写文字。

候选与挑选：每种尺寸 2 张（两次并行生成，同一提示词）。选第 2 组：看板娘胸牌保留“AI 吠点”（第 1 组胸牌没有字）；竖版背景更干净、主体之间留白更大（第 1 组背景叠了摩天轮、过山车、云朵，较拥挤）；横版主标题与副标题、看板娘完整落在居中 1:1 框（x 约 551–1370）内。逐字核对：画面文字只有“AI安全员辞职？”“免费Gemini没了？”“免费版没取消”“10月9日”与胸牌“AI 吠点”，无错字、乱码与 logo；左下角有一个空白票根框（竖版），无字。

### 3:4 竖版提示词

竖版 3:4 封面插画。画风：复古游乐园入场须知牌与票根——奶油色底，深青绿与番茄红两色为主，粗黑描边，扁平、手绘感、略带搞怪，非写实、无照片质感、无真人。
画面最上方是全画面最醒目的主标题，粗体两行：“AI安全员辞职？”“免费Gemini没了？”。标题下方一行较小的红色字副标题：“免费版没取消”。
画面中部三个主体，彼此之间留出大块空白，不堆细节：
1. 左：一个卡通安全员（戴黄色安全帽、穿反光背心的圆脸小人，非真人，身上不写字），抱着一个纸箱从一扇小门走出去，门上方有一盏绿色出口小灯（只画图形，不写字）。
2. 中：一根游乐园“身高限制尺”立柱，有三档横杠，只有最低一档亮着绿灯，另外两档横杠上各挂着一把大挂锁；立柱旁有一张小票根，票根上写“10月9日”。
3. 右：一只卡通蜂鸟，头顶戴着一顶歪歪扭扭的手工纸板皇冠，皇冠上画一个大问号。
看板娘（按参考图）站在右下角，较小，戴着验票员红色袖章，一手拿检票钳、一手叉腰歪头质疑，表情狡黠。
除“AI安全员辞职？”“免费Gemini没了？”“免费版没取消”“10月9日”外，画面中不出现任何其他文字、字母、数字或 logo（含各公司商标）。用常用字，字迹清楚，手机缩略图下主标题可读。

### 2.35:1 横版提示词

横版 2.35:1 封面插画，按横构图重新排版（不是竖版裁切）。画风同上：复古游乐园入场须知牌与票根，奶油色底，深青绿与番茄红两色为主，粗黑描边，扁平、手绘、略带搞怪，非写实、无真人。
主标题放在画面正中央安全区，粗体一行：“AI安全员辞职？免费Gemini没了？”，其正下方一行较小的红色字副标题：“免费版没取消”。主标题与副标题都完整落在画面中央、宽度等于画面高度的正方形范围内（若一行放不下，可分两行，仍须在该正方形内）。看板娘（按参考图）也放在这个中央正方形内、主标题下方偏右，较小，戴验票员红袖章、手拿检票钳歪头质疑。
左侧：卡通安全员（黄色安全帽、反光背心的圆脸小人，非真人，身上不写字）抱着纸箱走出一扇小门，门上方一盏绿色出口小灯（只画图形）。
右侧偏上：一根游乐园“身高限制尺”立柱，三档横杠，只有最低一档亮绿灯、另两档挂大挂锁，旁边一张小票根写“10月9日”；右侧偏下：一只卡通蜂鸟，头戴歪歪扭扭的手工纸板皇冠，皇冠上画一个大问号。
三个主体之间留出大块空白。除“AI安全员辞职？免费Gemini没了？”“免费版没取消”“10月9日”外，不出现任何其他文字、字母、数字或 logo（含各公司商标）。
