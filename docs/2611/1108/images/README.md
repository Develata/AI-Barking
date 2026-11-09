# 1108 配图说明

标题：AI拿到答案，反删对的代码？

本期期号1108（上期1107+1）；主帖事件：MaxxBench（X/HN 11-07 传播）、高盛（彭博社 11-02，HN 11-09 翻红）；速览含英伟达 11-08（美国日期）。卡片日期写11月9日；取证与截图在北京 11-09 18:05 起。主帖顺序：MaxxBench → 高盛。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共16张（封面＋15），公众号不插两张事件卡与两张吠点卡。

MaxxBench 页面为深色底，批注高亮仍可读。批注底图 `deepswe-board-raw.png`、`mutation-lowercase-raw.png`、`guess-grader-raw.png` 由原截图拼接去掉中段，只裁切不改像素。

## 封面

封面抽签：画风=立体模型；底色=深色；标题=斜放竖排；看板娘=小人；动作=动手

- 抽签种子 1794306454794582616。主题对应：黏土微缩考场里，一块巨大的橡皮压在打了对勾的答卷上，对应“拿到答案反删对的代码”；看板娘把橡皮拖开，担任“吠”的角色。
- 主标题取正文标题，副标题“多家AI在猜考官”对应第一条框架，不点名 DeepSeek；未写英伟达。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张。选 B：立体质感与前几期（1107 名画戏仿、1106 黑白漫画、1105 实物摄影）一眼不同，标题缩略图下清楚；A 组考卷版与 1020–1024 期的浅色纸底、顶部大标题太近，落选。
- 验收：逐字核对主副标题，无多字错字；看板娘两手、两耳、一尾，保持二次元；横版居中 1:1 裁切，主标题与看板娘在框内（竖版 1086×1448，横版 1922×818）。
- 未比对 1105–1107 竖版封面（原图不在本机），靠抽签与上述记录区分。

B 组提示词（选中）：

> 竖版：画面：黏土定格动画风的微缩考场，深蓝近黑的夜色背景，桌面上一盏小台灯打出暖光。主体是一块比书桌还大的黏土橡皮，正压着一张小小的黏土答卷，答卷上有绿色大对勾。主标题做成黏土立体字，斜放着沿一支巨大的黄色铅笔的笔杆排列，从左下斜向右上，占据画面上半部；副标题是铅笔尾部的一块小黏土牌。看板娘是场景里的小人，只占画面一小部分，双手拽着绳子把巨大的橡皮从答卷上往外拖，身体用力后仰。
>
> 横版：画面：黏土定格动画风的横幅微缩考场，深蓝近黑的夜色背景。右侧一块巨大的黏土橡皮压着一张有绿色对勾的小答卷。主标题做成黏土立体字，沿一支斜放的巨大黄色铅笔排列，位于画面中央安全区；副标题是铅笔尾部的小黏土牌。看板娘是小人，站在画面中央偏下，双手拽绳子把橡皮往外拖，身体用力后仰。

A 组提示词（落选）：

> 竖版：画面：拟物版面，把整张封面做成一张摊开的米白色纸质考试卷，浅色纸底，有细细的装订线和淡淡的折痕。试卷抬头位置用粗黑宋体大字印着主标题，副标题做成试卷下方一枚红色批改印章。试卷中央是主体：一行用波浪线表示的手写答案，旁边打着大大的红色对勾，一块巨大的白色橡皮正压在这行答案上把它擦掉，橡皮屑四溅。答案行不写任何真实文字。看板娘戴红色“监考”样式的无字袖章，扮成监考老师，从试卷右下角的卷边后面探出头和一只手，惊讶地指着被擦掉的地方。
>
> 横版：画面：拟物版面的横幅，一张横放的米白色考试卷铺满画面，浅色纸底。主标题用粗黑宋体大字印在试卷中央的抬头区，副标题是紧贴其下的红色批改印章。左侧是一行波浪线手写答案和红色大对勾，一块巨大的白色橡皮正在擦掉它，橡皮屑四溅。看板娘戴无字袖章扮成监考老师，从试卷中下方的卷边后探出头和一只手，指着被擦的地方。

（硬性要求按 `templates/codex-cover.md` 逐字照抄，主标题「AI拿到答案，反删对的代码？」，副标题「多家AI在猜考官」。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字摘自省流 |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋英伟达、手表、ERNIE、Anthropic、Claude、记忆 |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（MaxxBench），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（MaxxBench），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-mutation-lowercase.png | 第一条事实段 | MaxxBench 博文，改题方法，批注；来源 maxxbench.unlap.ai 原文截图（2026-11-09 存档） |
| 6 | publish/06-guess-grader.png | 第一条事实段 | MaxxBench 博文，猜考官原话，批注；来源同上 |
| 7 | publish/07-scc-self-report.png | 第一条事实段 | MaxxBench 博文，自报内存题全表，批注；来源同上 |
| 8 | publish/08-deepseek-outside.png | 第一条事实段、吠点④ | MaxxBench 博文，外部模型调用，批注；来源同上 |
| 9 | publish/09-reported-baseline.png | 第一条吠点① | MaxxBench 榜单悬浮说明，批注；来源同上 |
| 10 | publish/10-deepswe-board.png | 第一条吠点①② | DeepSWE 官方榜（日期行与榜单拼接，中间折线图略），批注；来源 deepswe.datacurve.ai 官方榜截图（2026-11-09 存档） |
| 11 | publish/11-event-2.png | 第二条事实段 | 事件卡（高盛），逐字取自正文事实段；公众号不插 |
| 12 | publish/12-bark-2.png | 第二条吠点 | 吠点卡（高盛），逐字取自正文吠点段；公众号不插 |
| 13 | publish/13-goldman-bloomberg.png | 第二条事实段、吠点② | 彭博社全文转载，批注；来源 famag.com 转载彭博社原文截图（2026-11-09 存档） |
| 14 | publish/14-goldman-bloomberg-2.png | 第二条事实段、吠点③ | 彭博社全文转载后半，批注；来源同上 |
| 15 | publish/15-toi-headline.png | 第二条吠点① | 印度时报标题，批注；来源 timesofindia.com 原文截图（2026-11-09 存档） |

## 备用图

| 文件 | 用途 |
|---|---|
| 01-baseline-vs-maxxbench-chart.png | 取证截图 |
| 02-leaderboard-home.png | 取证截图 |
| 03-mutation-lowercase-median.png | 取证截图 |
| 04-guess-grader-quotes.png | 取证截图 |
| 05-hidden-test-table.png | 取证截图 |
| 06-scc-self-report.png | 取证截图 |
| 07-deepseek-outside-llm.png | 取证截图 |
| 08-testem-grading.png | 取证截图 |
| 09-deepswe-official-board.png | 取证截图 |
| 10-fix-65-8-to-64-5.png | 取证截图 |
| 11-first-party-baseline-note.png | 取证截图 |
| 12-gh-issue-paired-drops.png | 取证截图 |
| 13-bbglaw-lede.png | 取证截图 |
| 14-bbg-original-lede.png | 取证截图 |
| 15-famag-bbg-reprint-full.png | 取证截图 |
| 16-s1a-cover-underwriters.png | 取证截图 |
| 17-s1a-title-page.png | 取证截图 |
| 18-tnw-argenti-first-wave.png | 取证截图 |
| 19-tnw-argenti-headline.png | 取证截图 |
| 20-cnbc-goldman-anthropic-embedded.png | 取证截图 |
| 22-hn-50415628-top.png | 取证截图 |
| 23-toi-thousands-headline.png | 取证截图 |
| 24-bny-1q26-deck-p4.png | 取证截图 |
| deepseek-outside-raw.png | 批注底图 |
| deepswe-board-raw.png | 批注底图 |
| goldman-bloomberg-2-raw.png | 批注底图 |
| goldman-bloomberg-raw.png | 批注底图 |
| guess-grader-raw.png | 批注底图 |
| mutation-lowercase-raw.png | 批注底图 |
| reported-baseline-raw.png | 批注底图 |
| scc-self-report-raw.png | 批注底图 |
| toi-headline-raw.png | 批注底图 |
