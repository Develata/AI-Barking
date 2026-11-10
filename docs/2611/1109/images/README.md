# 1109 配图说明

标题：Claude降价反贵一倍？AI没泡沫？

本期期号1109（上期1108+1）；主帖事件：Fable 5.2 发布（北京 11-10 凌晨）、“AI 泡沫不存在”长文（11-08 发，HN 11-09 起热）。卡片日期写11月10日；取证与截图在北京 11-10 15:28 起。主帖顺序：Fable-5.2 → 泡沫。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共15张（封面＋14），公众号不插两张事件卡与两张吠点卡。

Hexagon 页面为浅青底纹，原文字号小，批注图 09 的英文在手机上需放大看；译注给出关键句直译。

## 封面

封面抽签：画风=东方传统；底色=满幅场景；标题=嵌进物件；看板娘=半身；动作=查验

- 抽签种子 1794224294866719808（本期跑过两次 `cover-draw`，第一次未记种子，按第二次的种子取 A、B 两组，未在两组间拼取值）。主题对应：木版年画风的老账房里，一本账簿拖成长卷铺满地面，对应“单价降了、一道题的账却越记越长”；看板娘拿放大镜查账，担任“吠”的角色。主标题做成门楣横匾，副标题做成柜台前的幌子（标题轴“嵌进物件”）。
- 主标题取正文标题第一个分句，副标题“最高档每题花费翻倍”对应吠点①；未写泡沫一帖。副标题是一行判断，按 EDITORIAL“封面·文字”允许；省流卡按视觉复核只列事实。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张。选 B：年画红绿满幅与 1108 的深色黏土、1107 名画戏仿一眼不同；A 组收银小票版是浅色纸底、顶部大标题，正是 1020–1024 期那一路，落选。
- 验收：逐字核对主副标题，无多字错字，账簿上只有圈点与格线、无可读文字；看板娘两手、两耳、一尾，保持二次元；横版居中 1:1（818×818）裁切，主标题与看板娘在框内（竖版 1086×1448，横版 1923×818）。已与 1108 竖版封面并排看；1106、1107 封面原图不在本机，未并排。

B 组提示词（选中）：

> 竖版：画面：东方传统木版年画风格（线条粗、色块饱满、红绿黄为主，略带木刻纹理），满幅场景：一间老式账房铺满画面，柜台上摆着一把大算盘，一本账簿从柜台上展开成长卷，一路拖过地面、绕过门槛还在延伸，长得夸张。账房门楣上挂一块横匾招牌，主标题「Claude降价反贵一倍？」写在这块招牌上，大而醒目；柜台前挂一面小幌子，写副标题「最高档每题花费翻倍」。账簿上的格子只画抽象的墨线与圈点，不写可读的字或数字。看板娘半身（腰部以上）站在柜台后，一手举着放大镜凑近长卷细看，另一手指着账簿上的某一处，表情狐疑。搞怪、有冲击力。
>
> 横版：画面：东方传统木版年画风格，满幅横向长卷式场景：老式账房，柜台居中，大算盘与一本展开成长卷的账簿，账簿从柜台向左右两侧拖出画面。门楣横匾招牌居中写主标题「Claude降价反贵一倍？」，下方小幌子写副标题「最高档每题花费翻倍」。账簿格子只画抽象墨线与圈点，不写可读的字或数字。看板娘半身在柜台后正中，举放大镜查看长卷、另一手指着账簿。

A 组提示词（落选）：

> 竖版：画面：拟物版面，整张封面做成一张超长的收银小票。米白热敏纸质感的浅色底，纸边是锯齿撕口，小票从画面上方一路垂到下方还在继续往外吐、卷曲堆在底部，长得离谱，暗示“这一单越打越长”。小票正文只用一排排抽象的灰色短横线和点线表示条目，不写任何可读的字或数字。顶部横排大字主标题「Claude降价反贵一倍？」，像小票抬头那样印成粗黑体，占画面宽度九成；主标题下方一行较小的副标题「最高档每题花费翻倍」。看板娘大特写：她扮演收银员，戴一顶小小的收银员帽子（仍保留女仆头饰），上半身占画面一半以上，从小票右下方探入画面，双手捏着小票末端，瞪大眼睛、嘴巴张成O形，看着这条长到拖地的小票。搞怪、有冲击力。
>
> 横版：画面：拟物版面，横向铺开的一张超长收银小票，从画面左侧的老式收银机出口一路吐向右侧，在地上折成好几叠，米白热敏纸质感的浅色底，纸边锯齿撕口，条目只用抽象灰色短横线与点线，不写可读的字或数字。主标题「Claude降价反贵一倍？」顶部横排、粗黑体、居中，副标题「最高档每题花费翻倍」在主标题下一行。看板娘大特写，扮演收银员（小收银员帽叠在女仆头饰上），上半身占画面中部高度的一半以上，站在画面正中，双手拉着小票，一脸震惊。

（硬性要求按 `templates/codex-cover.md` 逐字照抄，主标题「Claude降价反贵一倍？」，副标题「最高档每题花费翻倍」。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字摘自省流与第二条事实段（不放吠点） |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋Meta、TPU、PSI、Asahi、DeepX |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（Fable-5.2），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（Fable-5.2），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-half-cost.png | 第一条事实段 | Anthropic 发布页“约一半成本”段与 FrontierCode 2.0 图，批注；来源 anthropic.com 原文截图（2026-11-10 存档） |
| 6 | publish/06-aa-cost.png | 第一条吠点① | Artificial Analysis 模型页，Fable 5.2 与 Fable 5.1 最高档每题成本（两页拼接），批注；来源 artificialanalysis.ai 原文截图（2026-11-10 存档） |
| 7 | publish/07-fermat-setup.png | 第一条吠点② | claude.dev 技术博客，早期版本与 47 条引理，批注；来源 claude.dev 原文截图（2026-11-10 存档） |
| 8 | publish/08-fermat-chart.png | 第一条事实段、吠点② | 发布页费马进度图与图注（第 18 小时续跑），批注；来源 anthropic.com 原文截图（2026-11-10 存档） |
| 9 | publish/09-blackhole-provenance.png | 第一条吠点③ | Hexagon 论文页（174）来源声明与 Hexagon 首页说明（三段拼接），批注；来源 hexagonmath.org 原文截图（2026-11-10 存档） |
| 10 | publish/10-event-2.png | 第二条事实段 | 事件卡（泡沫），逐字取自正文事实段；公众号不插 |
| 11 | publish/11-bark-2.png | 第二条吠点 | 吠点卡（泡沫），逐字取自正文吠点段；公众号不插 |
| 12 | publish/12-blog-208.png | 第二条事实段 | 博文原文，一次性支出与 2.08 美元段，批注；来源 matteoraso.github.io 原文截图（2026-11-10 存档） |
| 13 | publish/13-s1a-ratio.png | 第二条吠点① | S-1/A 每美元算力营收图（Q2 2.08x、Q3 1.72x）与下降说明，批注；来源 sec.gov 原文截图（2026-11-10 存档） |
| 14 | publish/14-s1a-q3.png | 第二条吠点② | S-1/A 季度利润表节选（营收、算力费用、经营亏损三行拼接），批注；来源 sec.gov 原文截图（2026-11-10 存档） |

## 备用图

| 文件 | 用途 |
|---|---|
| aa-cost-raw.png | 批注底图 |
| blackhole-provenance-raw.png | 批注底图 |
| blog-208-raw.png | 批注底图 |
| fermat-chart-raw.png | 批注底图 |
| fermat-setup-raw.png | 批注底图 |
| half-cost-raw.png | 批注底图 |
| s1a-q3-raw.png | 批注底图 |
| s1a-ratio-raw.png | 批注底图 |
| 01b-release-intro-price.png | 取证截图 |
| 02b-release-benchmark-table.png | 取证截图 |
| 03-release-effort-frontiercode.png | 取证截图 |
| 04-pricing-model-table.png | 取证截图 |
| 05-pricing-fast-mode.png | 取证截图 |
| 06-models-overview.png | 取证截图 |
| 07-aa-leaderboard-top-rows.png | 取证截图 |
| 08-aa-fable52-max-xhigh-high.png | 取证截图 |
| 09-aa-opus55-max-fable51-carbon.png | 取证截图 |
| 10-vals-index-leaderboard-and-note.png | 取证截图 |
| 11-syscard-frontiercode-text-p209.png | 取证截图 |
| 12-syscard-frontiercode-figure-p210.png | 取证截图 |
| 12b-zhidx-headline.png | 取证截图 |
| 13-release-fermat-text.png | 取证截图 |
| 14-release-fermat-chart.png | 取证截图 |
| 15-claudedev-intro.png | 取证截图 |
| 16-claudedev-hour287.png | 取证截图 |
| 17-claudedev-limits.png | 取证截图 |
| 18-claudedev-monitor.png | 取证截图 |
| 19-sept-original.png | 取证截图 |
| 20-release-blackhole-text.png | 取证截图 |
| 21-hexagon-hawking-page.png | 取证截图 |
| 22-hexagon-modular-bootstrap.png | 取证截图 |
| 23-hexagon-no-peer-review.png | 取证截图 |
| 24-syscard-fermat-1000.png | 取证截图 |
| 25c-blog-onetime.png | 取证截图 |
| 26-blog-208.png | 取证截图 |
| 27-blog-72b.png | 取证截图 |
| 28-blog-revenue.png | 取证截图 |
| 29-s1-chart-208x.png | 取证截图 |
| 30-s1a-chart-172x.png | 取证截图 |
| 31-s1-h1-statement.png | 取证截图 |
| 32-s1a-q3-quarterly.png | 取证截图 |
| 33-s1a-commitments.png | 取证截图 |
| 34-s1-compute-definition.png | 取证截图 |
| 35-pe-googl.png | 取证截图 |
| 36-hn-head.png | 取证截图 |
