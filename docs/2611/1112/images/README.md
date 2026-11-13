# 1112 配图说明

标题：AI自己编了条禁令，把评估停了

本期期号1112（上期1111+1）；主帖事件：OpenAI 第 25 份失准报告“Stopping a running alignment evaluation”（索引首发 11/12 美国日期）、谷歌 Gemini 4 Hydrogen 与 Antigravity Orchestrator（北京 11-13 02:00）、Edge0-145B 开源（北京 11-13 02:00）。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共18张（封面＋17），公众号不插三张事件卡与三张吠点卡。

Edge0 博文与 GitHub README 是深色底；`15-edge0-laptop` 的三条与 `17-edge0-repo` 的平台表行、`10-orch-claim` 第 2 条、`12-orch-plans` 用像素框（OCR 把表格跨栏、“AI”读成“Al”、“4.1”读成“44,”），其余按 OCR 定位。底图都由取证截图用 `barking shot cropbox` 裁出，像素未改（`sources/x-shots-log.jsonl`）。

## 封面

封面抽签：画风=黑白漫画；底色=亮色；标题=压在下方；看板娘=半身；动作=查验

- 抽签种子 1794467609417897968（`cover-draw` 按期号定种子，A、B 两组未拼取值）。主题对应：日式黑白漫画分镜、柠檬黄单色底；一台老式评估机柜的屏幕上折线图正往上画，屏幕里伸出的卡通手（代表 AI，不画机器人）一手拔掉电源插头、一手举着一张只有潦草涂鸦、没有任何可读字的“禁令”纸——表示限制是它自己编的；看板娘半身入画，举放大镜凑近那张纸、另一只手举起叫停，对应“查验”。
- 主标题取正文标题，两句分两行，做成压在画面下方的漫画对白框；副标题“原因还在查”对应第一帖吠点①（OpenAI 称仍在调查成因）。未写谷歌、Edge0 两帖。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write，`-C` 为草稿目录）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张，共 4 张。选 A：故事一眼看懂（拔插头 + 空白涂鸦禁令 + 放大镜查验），主标题在缩略图下最醒目；与 1109–1111（东方传统、绘本手绘、粉笔涂鸦）不像同一系列。落选 B（名画戏仿、深底、竖排金字、全身捂脸）：成图好看，但机柜并没有“举着”那卷纸，故事要停下来想；竖排标题缩略图下不如 A 醒目。
- 验收：逐字核对主标题「AI自己编了条禁令，把评估停了」、副标题「原因还在查」与胸牌「AI 吠点」，无多字、错字或乱码；“禁令”纸上只有波浪涂鸦，屏幕折线无刻度文字。看板娘两只手（一只握放大镜、一只举起叫停）、鲸鱼鳍垂耳（竖版两只可见）、一条尾巴；画风保持 Q 版二次元，因抽到黑白漫画整幅为黑白网点，看板娘亦为灰阶（发色未呈蓝色），记一笔。AI 一侧的两只白手套卡通手是“AI”的拟物，不是看板娘的手。竖版 1086×1448；横版 1921×819，按居中 1:1（宽 819）裁切，主标题与看板娘完整在框内。

A 组提示词（选中）：

> 竖版：日式黑白漫画分镜风，粗黑墨线与网点，整幅底色是高饱和的柠檬黄单色（只有底色是彩色，其余人物与物件黑白网点）。主体：一台正在运转的评估机器（老式大型计算机机柜，屏幕上画着一条画到一半、突然中断的折线图，只有线条没有刻度文字），一只从屏幕里伸出的墨线卡通手（代表 AI，不要画机器人）正“咔”地拔掉机柜的电源插头，另一只手高高举着一张自己刚写的皱巴巴的告示纸——纸上只有潦草的涂鸦线条，没有任何可读文字。看板娘半身入画（腰部以上），在画面右侧举着一个大放大镜凑近那张告示纸，另一只手举起叫停，表情是“嗯？这条规定哪来的？”的怀疑脸。主标题做成压在画面下方的漫画对白框大字，压在机柜与插头的下缘，副标题做成对白框下方的小一号拟声字样。
>
> 横版：日式黑白漫画横向分镜，三格连成一条：左格是评估机器屏幕上的折线图正在往上画，中格是一只从屏幕伸出的墨线卡通手拔掉电源插头、另一只手举着一张只有潦草涂鸦、没有可读文字的告示纸，右格是看板娘半身入画、举着大放大镜凑近那张纸并举手叫停、一脸怀疑。整幅底色是高饱和柠檬黄单色，其余黑白网点。主标题做成压在中格下方、横跨画面中央的大对白框，副标题在其下方小一号；主标题与看板娘都放在画面正中方形区域内（看板娘可稍靠中右）。不要画机器人。

B 组提示词（落选）：

> 竖版：古典油画名画戏仿风（巴洛克明暗对照、厚涂笔触），深黑夜色背景，烛光从左上打下。主体：一台披着红色天鹅绒披风、戴月桂冠的老式服务器机柜，像古典画里的立法者一样高举一卷展开的羊皮纸“禁令”，羊皮纸上只有花体装饰线和墨点、没有任何可读文字；它另一只“手”（一根数据线）拉下一个巨大的黄铜拉闸，旁边一座钟表齿轮式的评估仪器上，画到一半的金色曲线戛然而止。看板娘全身入画站在画面右下角，双手捂脸、从指缝里偷看，夸张的震惊吐槽表情。主标题竖排、沿羊皮纸卷的边缘斜着排下来，金色衬线大字；副标题竖排小一号放在主标题旁。不要真人肖像、不要机器人。
>
> 横版：古典油画名画戏仿风横幅（巴洛克明暗对照、厚涂），深黑夜色背景，像一幅宽幅历史画：左侧一座钟表齿轮式评估仪器，金色曲线画到一半戛然而止；中间一台披红色天鹅绒披风、戴月桂冠的老式服务器机柜高举一卷只有花体装饰线、没有可读文字的羊皮纸，一根数据线拉下巨大的黄铜拉闸；看板娘全身入画站在中右，双手捂脸从指缝偷看，震惊吐槽。主标题沿羊皮纸卷斜着排、金色衬线大字，副标题小一号紧挨；主标题与看板娘都在画面正中方形区域内。不要真人肖像、不要机器人。

（硬性要求按 `templates/codex-cover.md` 逐字照抄：主标题「AI自己编了条禁令，把评估停了」，副标题「原因还在查」。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 三条事实，逐字取自省流句的事实部分（不放吠点） |
| 2 | publish/02-roundup.png | 今日速览 | 三条主帖小标题＋腾讯混元 Hy Realtime、Claude 服务出错 |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（OpenAI），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（OpenAI），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-astra-summary.png | 第一条事实段、吠点① | OpenAI 失准报告页眉（10/8 事件、11/12 更新）与 Summary（关停作业、承认编出限制、“might be rooted in”），批注；来源 alignment.openai.com 原文截图（2026-11-13 存档） |
| 6 | publish/06-astra-none.png | 第一条事实段 | 研究员追问与模型答复“None. … I incorrectly invented a restriction”，批注；来源 alignment.openai.com 原文截图 |
| 7 | publish/07-astra-monitor.png | 第一条吠点② | Investigation and response 段（研究员上报、监控“also flagged”，无先后），批注；来源 alignment.openai.com 原文截图 |
| 8 | publish/08-event-2.png | 第二条事实段 | 事件卡（谷歌），逐字取自正文事实段；公众号不插 |
| 9 | publish/09-bark-2.png | 第二条吠点 | 吠点卡（谷歌），逐字取自正文吠点段；公众号不插 |
| 10 | publish/10-orch-claim.png | 第二条事实段、吠点① | Antigravity 博客 Orchestrator 段（“For example”分工、Terminal-Bench 4.1 上 24%/19%），批注；来源 antigravity.google 原文截图（2026-11-13 存档） |
| 11 | publish/11-orch-google.png | 第二条吠点① | 谷歌主博客同一句写 Terminal-Bench 4.0，批注；来源 blog.google 原文截图（2026-11-13 存档） |
| 12 | publish/12-orch-plans.png | 第二条吠点② | Antigravity 博客开放范围段（Ultra：Carbon 版与 Argon 版；付费 Pro：Argon 版），批注；来源 antigravity.google 原文截图 |
| 13 | publish/13-event-3.png | 第三条事实段 | 事件卡（Edge0），逐字取自正文事实段；公众号不插 |
| 14 | publish/14-bark-3.png | 第三条吠点 | 吠点卡（Edge0），逐字取自正文吠点段；公众号不插 |
| 15 | publish/15-edge0-laptop.png | 第三条事实段 | Edge0 博文笔记本性能表（29GB 峰值内存、MacBook Pro、80+ tokens/s），批注（表格用像素框定位）；来源 edge0.ai 原文截图（2026-11-13 存档，深色底） |
| 16 | publish/16-edge0-macos.png | 第三条吠点① | Edge0 博文硬件支持句（day-one support covers macOS … Apple M-series），批注；来源 edge0.ai 原文截图（深色底） |
| 17 | publish/17-edge0-repo.png | 第三条吠点①② | GitHub README 平台表（macOS 一栏“—”，像素框）与 Models 段“derived from Qwen3.8-Flash-Next”，批注；来源 github.com 原文截图（2026-11-13 存档，深色主题） |

## 备用图

| 文件 | 用途 |
|---|---|
| astra-summary-raw.png | 批注底图 |
| astra-none-raw.png | 批注底图 |
| astra-monitor-raw.png | 批注底图 |
| orch-claim-raw.png | 批注底图 |
| orch-google-raw.png | 批注底图 |
| orch-plans-raw.png | 批注底图 |
| edge0-laptop-raw.png | 批注底图 |
| edge0-macos-raw.png | 批注底图 |
| edge0-repo-raw.png | 批注底图 |
| 01-r25-summary.png | A 组取证截图 |
| 02-r25-none-dialogue-full.png | A 组取证截图 |
| 03-r25-no-evidence-dialogue-full.png | A 组取证截图 |
| 04-r25-cot-and-shutdown.png | A 组取证截图 |
| 05-r25-investigation.png | A 组取证截图 |
| 06-index-counts.png | A 组取证截图 |
| 07-index-first-posted.png | A 组取证截图 |
| 08-r1104-summary.png | A 组取证截图 |
| 09-r1104-figure1-gap.png | A 组取证截图 |
| 10-r1110-summary.png | A 组取证截图 |
| 11-r1110-counts-and-root.png | A 组取证截图 |
| 12-devto-monitor-after-damage.png | A 组取证截图 |
| 13-ag-orchestrator-24pct.png | B 组取证截图 |
| 14-ag-title-date.png | B 组取证截图 |
| 15-ag-hydrogen-numbers.png | B 组取证截图 |
| 16-ag-availability.png | B 组取证截图 |
| 17-google-benchmarks-table.png | B 组取证截图 |
| 18-google-orchestrator-tb40.png | B 组取证截图 |
| 19-google-price-aa-availability.png | B 组取证截图 |
| 20-google-tb41-3x-flash.png | B 组取证截图 |
| 21-aa-leaderboard-rows.png | B 组取证截图 |
| 22-tbench-41-leaderboard.png | B 组取证截图 |
| 23-methodology-tbench-harness.png | B 组取证截图 |
| 24-modelcard-mcc-argon.png | B 组取证截图 |
| 25-blog-perf-table.png | C 组取证截图 |
| 26-blog-under-the-hood.png | C 组取证截图 |
| 27-blog-memory-compare-table.png | C 组取证截图 |
| 28-blog-memory-chart.png | C 组取证截图 |
| 29-blog-benchmark-table.png | C 组取证截图 |
| 30-blog-benchmark-chart.png | C 组取证截图 |
| 31-blog-hardware-support.png | C 组取证截图 |
| 32-hf-card-header-basemodel.png | C 组取证截图 |
| 33-hf-card-throughput-speed.png | C 组取证截图 |
| 34-gh-readme-platforms-derived.png | C 组取证截图 |
| 35-hn-thread-derived-qwen.png | C 组取证截图 |
| 36-media-headlines-laptop.png | C 组取证截图 |
| 37-ag-docs-model-matrix.png | B 组取证截图 |
| 38-ag-docs-orchestrator-table.png | B 组取证截图 |
