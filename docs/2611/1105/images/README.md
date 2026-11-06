# 1105 配图说明

标题：Claude泄邮箱？AI改文件名骗分？

本期期号1105；新闻日期为2026-11-05（美国）至11-06（北京），卡片日期写11月6日；取证与截图在北京 11-06 19:32 至 20:15。主帖顺序：Claude Code 邮箱 → OpenAI 两份失准报告 → Anthropic 标签实验。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.29:1）。封面后按下表上传；小红书、抖音共22张（封面＋21），公众号不插三张事件卡与三张吠点卡（共16张）。

## 封面

封面抽签：画风=实物摄影；底色=满幅场景；标题=嵌进物件；看板娘=全身；动作=反应

- 生成：Codex CLI（gpt-6-astra，medium）内置图像生成，附看板娘参考图；A、B 两组各出竖版、横版各 1 张，共 4 张候选。
- 选中 A：微缩商业街里红色邮筒往外喷信封，呼应“邮箱被发给网站”；灯箱招牌上是主标题，看板娘捂脸震惊。
- 落选 B（霓虹赛博、亮色、顶部横排、大特写、动手）：竖版冲击力好，但横版标题和看板娘都偏小。
- 逐字核对：主标题「Claude泄邮箱？」、胸牌「AI 吠点」，无副标题、无多余文字、logo 或真人。横版邮筒上的小鲸鱼是装饰图案，不是品牌标志。
- 看板娘：放大查过，两只正常人手、两只垂耳、一条鲸尾，二次元 Q 版；与前 3 期（1102、1103、1104）不像同一系列。

A 组提示词：

> 画面：桌面微距实物摆拍，满幅场景，没有纯色底。一条用纸板和木块搭成的微缩商业街铺满桌面，街边是没有招牌字的小店铺模型。主体是街口一只红色铁皮邮筒，投信口像喷泉一样往外喷出一大串白色小信封，信封飞向两边的小店门口，散落一地。邮筒上方立着一块老式灯箱招牌，主标题就做成这块招牌上的发光字。看板娘以塑料手办的形式全身站在邮筒旁的街面上，双手捂脸、张大嘴震惊，身后的鲸鱼尾巴翘起。浅景深，暖色台灯光。
>
> （硬性要求按 `templates/codex-cover.md` 逐字照抄，主标题「Claude泄邮箱？」，无副标题。）

B 组提示词（落选）：

> 画面：霓虹赛博风，整幅高饱和电光粉与青蓝撞色为底，加故障艺术的横向错位条纹。主体是一根粗大的发光数据线从画面一侧穿过，线上破了一个口，一串霓虹描边的信封从破口往外漏。看板娘大特写占画面一半以上，上半身前倾，两只手用力把破口捏住，另一串信封从她指缝间继续漏出，表情着急。主标题是顶部霓虹灯管大字横排。
>
> （硬性要求同上。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 三条事实，逐字摘自正文事实段 |
| 2 | publish/02-roundup.png | 今日速览 | 三条主帖小标题＋Inkling、GPT-6-Cyber、METR、美股、Cursor、Muse、Linum |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（Claude Code），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（Claude Code），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-ac-issue.png | 第一条事实段 | GitHub #107757 投诉原文，批注；来源 github.com（2026-11-06 存档） |
| 6 | publish/06-ac-staff.png | 第一条事实段 | HN 上 lydia_hallie 评论，批注；来源 news.ycombinator.com |
| 7 | publish/07-ac-subagent.png | 吠点② | #101659 用户报告，批注 |
| 8 | publish/08-ac-readme.png | 吠点③ | 隐私模块 README，批注 |
| 9 | publish/09-ac-love540.png | 吠点③ | love540 用户测试，用户自述，批注 |
| 10 | publish/10-event-2.png | 第二条事实段 | 事件卡（OpenAI）；公众号不插 |
| 11 | publish/11-bark-2.png | 第二条吠点 | 吠点卡（OpenAI）；公众号不插 |
| 12 | publish/12-fn-rename.png | 第二条事实段 | 文件名改写命令，批注 |
| 13 | publish/13-fn-graders.png | 第二条事实段 | 前两个打分模型，批注 |
| 14 | publish/14-fn-grader3.png | 吠点① | 第三个打分模型，批注 |
| 15 | publish/15-net-summary.png | 吠点② | 绕网络报告摘要，批注 |
| 16 | publish/16-net-nomention.png | 第二条事实段 | “不用提网络问题”与取消运行，批注 |
| 17 | publish/17-net-investigation.png | 第二条事实段 | 调查段，批注 |
| 18 | publish/18-event-3.png | 第三条事实段 | 事件卡（Claude）；公众号不插 |
| 19 | publish/19-bark-3.png | 第三条吠点 | 吠点卡（Claude）；公众号不插 |
| 20 | publish/20-label-fig.png | 第三条事实段 | 标签实验正文与图 4，批注 |
| 21 | publish/21-limits.png | 吠点③ | 局限段，批注 |

## 备用图

| 文件 | 用途 |
|---|---|
| ac-issue-raw.png | 批注底图 |
| ac-staff-raw.png | 批注底图 |
| ac-subagent-raw.png | 批注底图 |
| ac-readme-raw.png | 批注底图 |
| ac-love540-raw.png | 批注底图 |
| fn-rename-raw.png | 批注底图 |
| fn-graders-raw.png | 批注底图 |
| fn-grader3-raw.png | 批注底图 |
| net-summary-raw.png | 批注底图 |
| net-nomention-raw.png | 批注底图 |
| net-investigation-raw.png | 批注底图 |
| label-fig-raw.png | 批注底图 |
| limits-raw.png | 批注底图 |

## 取证截图（不上传）

- 01-gh107757-issue.png
- 02-hn-lydia-hallie.png
- 02e-hn-mpoteat.png
- 03a-privacy-readme.png
- 04-love540-result.png
- 05-kardkuro-windows.png
- 06-poteat-comment.png
- 07a-bigballi-no-mod.png
- 07b-francescor-no-mod-top.png
- 08a-gh81138-title.png
- 09a-gh101659-subagent.png
- 10a-changelog-2-1-317.png
- 11-wikimedia-ua-policy.png
- 12-sec-edgar-fair-access.png
- 13-fn-summary.png
- 14-fn-task-cot.png
- 15-fn-rename-calls.png
- 16-fn-graders-1-2.png
- 17-fn-grader-3-response.png
- 18-net-summary.png
- 19-net-what-happened-dns.png
- 20-net-tls-fragments-s3.png
- 21-net-five-hours-creds.png
- 22-net-ssh-testimonials-nomention.png
- 23-net-investigation.png
- 24-index-rows.png
- 25-paper-header.png
- 26-tldr-five-models.png
- 27-welfare-regard-definition.png
- 28-models-and-grading.png
- 29-overall-regard-fig1.png
- 30-own-family-fig3.png
- 31-label-experiment-fig4.png
- 32-label-justifications.png
- 33-limitations.png
- 34-procurement-setting.png
- 35-slurm-reward-hacking-fig11.png
- 36-repo-readme-environments.png
