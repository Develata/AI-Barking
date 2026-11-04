# 1103 配图说明

标题：腾讯最强开源没权重？软件偷装AI插件？

本期期号1103；新闻日期为2026-11-04（北京），卡片日期写11月4日；取证与截图在北京 11-04 20:46 至 21:30（批注图来源均写 2026-11-04 存档）。主帖顺序：腾讯 Hy4 → Postman 插件。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共16张（封面＋15），公众号不插两张事件卡与两张吠点卡：正文插图11张，另有横版封面1张。

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字摘自正文事实段 |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋手表、Siri、学校账号、AMD |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（腾讯 Hy4），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（腾讯 Hy4），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-hy4-open-soon.png | 第一条事实段、吠点① | 腾讯官网简体稿首段：“Hy 首次跻身最强开源模型之列”“Hy4 将于近期开源模型权重，发布模型系统卡片” |
| 6 | publish/06-hy4-finance-first.png | 第一条吠点② | 混元博客金融段：联合腾讯金融科技 FiT 的213位专家众测、579个任务，Hy4 综合表现位居开源模型第一（四个分数） |
| 7 | publish/07-hy4-bench-chart.png | 第一条吠点② | 腾讯官方12项测试对比图（整图含图例与脚注），标出 Hy4 与 Claude Opus 5.5 图例 |
| 8 | publish/08-kimi-by-nov4.png | 第一条吠点③ | Kimi 官方博客 Availability 节：“Full model weights will be released by Nov 4, 2026.” |
| 9 | publish/09-event-2.png | 第二条事实段 | 事件卡（Postman）；公众号不插 |
| 10 | publish/10-bark-2.png | 第二条吠点 | 吠点卡（Postman）；公众号不插 |
| 11 | publish/11-postman-auto-install.png | 第二条事实段 | GitHub 帖：发帖人贴出的 Postman 日志，安装来源 auto-existing-upgrade，Claude Code、Codex 插件已安装 |
| 12 | publish/12-postman-docs-approve.png | 第二条事实段、吠点② | Postman 官方文档：“The installer approves the hook for you.”；改用 Codex 自己的方式安装则由用户审核 |
| 13 | publish/13-postman-hn-headline.png | 第二条吠点① | Hacker News 转帖标题：“…attempts to share all API data”，1359 分 |
| 14 | publish/14-postman-no-exfil.png | 第二条吠点① | GitHub 帖：发帖人写明报告“does not claim to establish data exfiltration” |
| 15 | publish/15-postman-official-voice.png | 第二条吠点③ | GitHub 帖：zecoquit 用官方口吻回复（👍4 👎42） |

## 备用图

| 文件 | 用途 |
|---|---|
| hy4-open-soon-raw.png、hy4-finance-first-raw.png、hy4-bench-chart-raw.png、kimi-by-nov4-raw.png、postman-auto-install-raw.png、postman-docs-approve-raw.png、postman-hn-headline-raw.png、postman-no-exfil-raw.png、postman-official-voice-raw.png | 批注底图（由 05a（只留首段）、04a、02、11、14（只留第1节）、23、24（裁去右侧 login）、16、20 裁出，像素未改） |
| 01-tx-news-top.png、03-tx-news-responsible.png、04-tx-news-finance.png、05-tencentcom-en-lead.png、06-tencentcom-en-finance.png、07-tencentcom-en-cyber.png | 腾讯新闻版页首与“负责任的开源”段、金融段；腾讯官网英文稿首段（“in the coming days”）、金融段、网络安全段 |
| 03a-hy-blog-weights-limits.png | 混元博客：“我们计划开源模型权重”与“局限性”一节（与顶尖闭源模型“仍然存在较大差距”） |
| 08-hf-hy4-preview-head.png、09-hf-tencent-models-recent.png、10-hf-moonshotai-models-recent.png | Hugging Face：Hy4-preview 模型卡（Apache-2.0）、tencent 组织最近创建的模型（无 Hy4 正式版）、moonshotai 组织（无 K3.1） |
| 12-smzdm-kimi-today.png | 夸大实例：什么值得买用户稿“月之暗面 Kimi K3.1 Pro 今天正式开源全部权重”（北京 11-04 08:56） |
| 13-issue-title-body.png、15-issue-hook-installer.png、17-issue-requested-response.png | GitHub 帖标题与正文、安装器与钩子段、发帖人要求的回应 |
| 18-code-codex-hooks-trust.png、19-code-codex-callsite.png、21-code-session-start-context.png、22-readme-no-separate-hook-prompt.png | 插件源码（写 trusted_hash、调用点，多段拼接）、启动钩子注入的文字、README “There is no separate hook prompt” |
| 02-tx-news-bench-chart.png、04a-hy-blog-finance.png、05a-tencentcom-zhcn-lead.png、11-kimi-blog-availability.png、14-issue-log-config.png、16-issue-no-exfil-claim.png、20-issue-official-voice-comment.png、23-docs-installer-approves-hook.png、24-hn-main-title.png | 批注底图的取证原图 |

A 组截图多于编号区间，用 03a、04a、05a 字母后缀；B 组 GitHub 截图用匿名无头 Chrome 渲染（内置浏览器为登录态），宽 1850–2800 px，超过取证规范的 1400 px，批注底图未缩放。

## 封面

封面抽签：画风=立体模型；底色=亮色；标题=手写字；看板娘=探头；动作=查验

- 抽签：`barking cover-draw docs/2611/1103`。A 组：画风=像素游戏；底色=满幅场景；标题=顶部横排；看板娘=大特写；动作=旁观。
- 本期画风：纸艺加黏土的桌面微缩场景，亮黄底。拆开的大礼盒里空空的，只有一张折起的空白卡片（对应号称开源、模型本体没放出来）；旁边一台小笔记本，玩具小货车往屏幕里卸拼图形状的插件（对应升级时自动装插件）。主标题用马克笔手写在牛皮纸便签上；看板娘从笔记本屏幕后探出，举放大镜查验空礼盒（探头＋查验）。
- 生成：Codex CLI gpt-6-astra medium 图像生成（Linux `-s workspace-write`，图落在草稿目录），附看板娘参考图。A、B 各出竖版 1086×1448、横版 1921×819 各一张。
- 选中 B：主标题「腾讯最强开源没权重？」「软件偷装AI插件？」、胸牌「AI 吠点」逐字核对，无其他文字（礼盒上的小鲸鱼为图形）；看板娘一手拿放大镜、一手扶屏幕，共两只手，两只垂耳、一条尾巴，不戴手套；横版居中 1:1（宽 819）裁切时两行主标题与看板娘完整。与前 3 期（1102 绘本水彩夜空、1101 亮橙粉笔涂鸦、1031 名画戏仿）不像同一系列。
- 落选 A（像素 RPG 室内，空宝箱飘出沙漏，屏幕里像素搬运工往插槽塞插件卡带；看板娘右下大特写斜眼旁观）：文字正确、画面完整，标题更醒目；但两个隐喻要靠游戏常识读，B 的空礼盒与送货车一眼可懂。

竖版提示词（B，选中）：

> 画面：立体模型风格（纸艺加黏土的桌面微缩场景，柔和棚拍光、浅景深），亮黄色底。画面中央是一只拆开的大礼盒，彩带散开、盒盖掀起，里面却是空的，只有一张折起的小空白卡片（象征号称开源、模型本体没放出来）。礼盒右边是一台打开的小笔记本电脑模型，一辆玩具小货车正从键盘上开过，往屏幕里卸下几块拼图形状的插件。主标题用黑色马克笔手写在一张贴在画面上方的牛皮纸便签上，分两行，字大而清楚。看板娘从笔记本电脑屏幕后面探出上半身（探头），一只手举着放大镜对准空礼盒查验，另一只手扶着屏幕边。
画面里只允许出现这些文字：主标题，逐字为「腾讯最强开源没权重？」「软件偷装AI插件？」（两行）；看板娘胸前圆牌上的「AI 吠点」。不要其他文字、数字、字母、logo、国旗、品牌标志或真人。看板娘必须和附带的参考图是同一个角色、同一种画法：Q版二头身到二头半身的萌系二次元少女，大头、大圆眼、脸颊红晕，赛璐璐平涂上色和清晰描边，不要写实或半写实、不要成人身材比例。外形照参考图：深蓝到浅蓝渐变长卷发、头顶一根呆毛、白色荷叶边女仆头饰、浅蓝蝴蝶结发饰；两只向下垂的深蓝色鲸鱼鳍状垂耳（内侧白色绒边），不是竖起的猫耳或狼耳；深蓝白色女仆装、白围裙上有小鲸鱼图案；一条带白色圆点的深蓝鲸鱼尾巴；胸前「AI 吠点」圆牌；两只正常的人手（不戴手套）。抽到的画风只用于场景与标题，看板娘本人保持参考图画风（2026-10-27 起）。她只有两只手、两只耳朵、一条尾巴。主标题在手机缩略图下清楚可读。3:4 竖版，1080×1440 或同等比例。

竖版提示词（A，落选）：

> 画面：16位像素游戏风格的满幅场景，像一款复古 RPG 的室内关卡，色块清晰、像素颗粒可见。画面左侧是一只金色像素宝箱，箱盖大开、闪着光，可箱子里空空如也，只飘出几粒灰尘和一个小沙漏（象征“近期开源”的权重迟迟不到）。画面右侧是一台像素风台式电脑，屏幕里一个戴鸭舌帽的小像素搬运工正偷偷把两块拼图形状的插件卡带塞进两个插槽（象征软件升级时偷偷装插件）。主标题用粗像素字体横排在画面顶部，像游戏标题栏，分两行。看板娘以大特写出现在画面右下角，只露出头和肩、占画面约三分之一，侧过脸斜眼旁观这两件事，表情怀疑（旁观，不动手）。
画面里只允许出现这些文字：主标题，逐字为「腾讯最强开源没权重？」「软件偷装AI插件？」（两行）；看板娘胸前圆牌上的「AI 吠点」。不要其他文字、数字、字母、logo、国旗、品牌标志或真人。看板娘必须和附带的参考图是同一个角色、同一种画法：Q版二头身到二头半身的萌系二次元少女，大头、大圆眼、脸颊红晕，赛璐璐平涂上色和清晰描边，不要写实或半写实、不要成人身材比例。外形照参考图：深蓝到浅蓝渐变长卷发、头顶一根呆毛、白色荷叶边女仆头饰、浅蓝蝴蝶结发饰；两只向下垂的深蓝色鲸鱼鳍状垂耳（内侧白色绒边），不是竖起的猫耳或狼耳；深蓝白色女仆装、白围裙上有小鲸鱼图案；一条带白色圆点的深蓝鲸鱼尾巴；胸前「AI 吠点」圆牌；两只正常的人手（不戴手套）。抽到的画风只用于场景与标题，看板娘本人保持参考图画风（2026-10-27 起）。她只有两只手、两只耳朵、一条尾巴。主标题在手机缩略图下清楚可读。3:4 竖版，1080×1440 或同等比例。

横版提示词与竖版相同，另加“横版构图：主体与标题、看板娘都放在画面中间约 1:1 的区域内，左右只延伸背景”，比例改为 2.35:1。
