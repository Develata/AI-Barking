# 1110 配图说明

标题：谷歌有人读你邮件？Claude变笨了？

本期期号1110（上期1109+1）；主帖事件：Gemini 关联应用人工审核（HN 11-10 凌晨起热）、Opus-5.5“11月3日变笨”之争（原帖北京 11-08）、Gemini-4-Carbon 向付费用户开放（北京 11-11 凌晨）。卡片日期写11月11日；取证与截图在北京 11-11 14:25 起。主帖顺序：Gemini → Opus-5.5 → Carbon。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共20张（封面＋19），公众号不插三张事件卡与三张吠点卡。

Theo 的图是深色底，只高亮推理 token 一行（12 号），译注给出对应数字；Melaugh 与 Theo 的图来自海外社媒，图题写作者与发帖日期，页脚写“海外社媒原文截图”。

## 封面

封面抽签：画风=绘本手绘；底色=浅色；标题=压在下方；看板娘=探头；动作=旁观

- 抽签种子 1794394539867013368（`cover-draw` 按期号定种子，A、B 两组未拼取值）。主题对应：儿童绘本水彩里，一封摊开的信被悬空的放大镜照着，字迹只画成看不清的波浪线，对应“邮件有人看”；旁边立着一张画了缩短思考气泡的便签，对应“Opus 想得少了”之争；看板娘从信纸后探出、捧着茶旁观，担任“吠”的角色。
- 主标题取正文标题，两句分两行；副标题“1月就写在帮助页”对应第一帖吠点①（Wayback 2026-01-14 UTC 快照）。未写 Carbon 一帖。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张。选 B：浅色水彩纸底、标题压在下方，与 1108 深色黏土、1109 红绿年画一眼不同；A 组深蓝夜色剪纸与 1108 的深色底太近，胸牌“吠点”二字放大后笔画走样，落选。
- 验收：逐字核对主副标题与胸牌，无多字错字；信纸与便签上只有波浪线和空白气泡、无可读文字；看板娘两手（捧杯）、两耳、一尾，保持二次元；横版居中 1:1（818×818）裁切，主标题与看板娘在框内（竖版 1086×1448，横版 1922×818）。横版胸牌很小，“吠”字左旁略糊，缩略图下不可辨，接受。已与 1108、1109 竖版封面并排看；1106、1107 封面原图不在本机，未并排。

B 组提示词（选中）：

> 竖版：儿童绘本风格的水彩加彩铅插画，米白纸底，笔触柔和。画面主体是一封摊开在桌面上的信，信纸上方悬着一只大大的放大镜（放大镜悬空，没有手拿着），放大镜下的信纸字迹被画成模糊的波浪线，看不出任何字。信旁边立着一张小小的便签纸，上面画着一个缩短了的思考气泡（只有线条，没有字）。看板娘从信纸后面探出半个身子，只露出头、垂耳和双手扒在信纸边缘，手里捧着一杯茶，偷偷旁观，表情好奇。主标题用深墨色圆润手写体压在画面下方三分之一处，盖在桌面上，分两行；副标题小字在主标题下面。画面只保留信、放大镜、便签和看板娘，多留白。
>
> 横版：儿童绘本风格的水彩加彩铅横幅插画，米白纸底。画面中央是一封摊开的信，上方悬着一只大放大镜（没有手拿着），信纸字迹画成看不清的波浪线；信右边立着一张画了缩短思考气泡的小便签（没有字）。看板娘从信纸后探出头和双手，捧着一杯茶偷看。主标题用深墨色圆润手写体放在画面中段偏下、压在桌面上，分两行，副标题小字在下面。主标题、信与看板娘集中在画面正中，两侧只留米白纸底和淡淡的水彩晕染。

A 组提示词（落选）：

> 竖版：纸艺手工风格的立体剪纸场景，深夜蓝黑底色，纸张有层叠阴影。画面主体是一只用白色卡纸折成的大信封，信封封口被掀开一角，里面露出一张叠起来的信纸；信封侧面开了一个圆形剪纸小窗，窗里一只剪纸做的大眼睛正往里看（只是一只纸眼睛，不是人脸）。画面右下角是第二件小物件：一个剪纸思考气泡，里面原本的长纸条被剪成一小段短纸条，象征“想得少了”。主标题用白色粉笔质感的手写字写在画面上部的深色夜空里，副标题用小一号手写字写在主标题下面。看板娘是站在信封旁边的小人，只有信封三分之一高，双手捂住脸颊、张大嘴做出震惊反应，像在吐槽“居然有人看”。画面只保留信封、纸眼睛、思考气泡和看板娘四样东西，留出空间。
>
> 横版：纸艺手工风格的立体剪纸横幅，深夜蓝黑底色，层叠纸张阴影。画面中央是白色卡纸折成的大信封，封口掀开一角，信封侧面圆形小窗里一只剪纸大眼睛往里看（只是纸眼睛，不是人脸）；信封右侧是一个剪纸思考气泡，里面的长纸条被剪成一小段。主标题用白色粉笔质感手写字写在信封上方，分两行，副标题小字在其下。看板娘是信封左侧的小人，双手捂脸张嘴震惊。主标题、信封与看板娘都集中在画面正中，两侧只留深色背景和少量纸星星。

（硬性要求按 `templates/codex-cover.md` 逐字照抄，主标题「谷歌有人读你邮件？Claude变笨了？」分两行，副标题「1月就写在帮助页」。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 三条事实，逐字摘自三个主帖的事实段（不放吠点） |
| 2 | publish/02-roundup.png | 今日速览 | 三条主帖小标题＋DeepMind、curl、亚马逊、投毒、OpenAI、Claude、Qwen |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（Gemini），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（Gemini），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-help-review.png | 第一条事实段 | Gemini 帮助页“人工审核”问答，批注；来源 support.google.com 原文截图（2026-11-11 存档） |
| 6 | publish/06-jan-snapshot.png | 第一条吠点① | 同一帮助页 2026-01-14 网页存档快照，批注；来源 web.archive.org 存档快照 |
| 7 | publish/07-blog-promise.png | 第一条吠点① | 谷歌 2023 年博文原承诺与 10/16 更新说明（标题与正文两段拼接），批注；来源 blog.google 原文截图（2026-11-11 存档） |
| 8 | publish/08-event-2.png | 第二条事实段 | 事件卡（Opus-5.5），逐字取自正文事实段；公众号不插 |
| 9 | publish/09-bark-2.png | 第二条吠点 | 吠点卡（Opus-5.5），逐字取自正文吠点段；公众号不插 |
| 10 | publish/10-melaugh-footer.png | 第二条吠点① | Melaugh 图 1（措辞变化、UTC 时刻与页脚），批注；来源海外社媒原文截图（2026-11-11 存档） |
| 11 | publish/11-melaugh-upfront.png | 第二条事实段 | Melaugh 图 2（high 档首次调用工具前思考 token 84→54；标题与中段两段拼接），批注；来源海外社媒原文截图 |
| 12 | publish/12-theo-work.png | 第二条事实段 | Theo 反驳图的表格与页脚（两段拼接，每条回复平均推理 token 369→413；深色底），批注；来源海外社媒原文截图（2026-11-11 存档） |
| 13 | publish/13-thinking-summary.png | 第二条吠点① | Claude 官方文档“Summarized thinking”节，批注；来源 platform.claude.com 原文截图（2026-11-11 存档） |
| 14 | publish/14-event-3.png | 第三条事实段 | 事件卡（Carbon），逐字取自正文事实段；公众号不插 |
| 15 | publish/15-bark-3.png | 第三条吠点 | 吠点卡（Carbon），逐字取自正文吠点段；公众号不插 |
| 16 | publish/16-carbon-pricing.png | 第三条事实段、吠点① | Gemini API 定价页 Carbon 一节（免费档不可用、体验价与 12/1 起价格），批注；来源 ai.google.dev 原文截图（2026-11-11 存档） |
| 17 | publish/17-blog-6x.png | 第三条事实段 | 谷歌发布博文 MKA 段（排第二、“低 6 倍以上”即不到六分之一），批注；来源 blog.google 原文截图（2026-11-11 存档） |
| 18 | publish/18-aa-cost.png | 第三条吠点② | 博文链接的 AA 榜单 MKA 每题平均成本柱图（Fable 5.2 $31.98、Carbon $6.29），批注；来源 artificialanalysis.ai 原文截图（2026-11-11 存档） |
| 19 | publish/19-mka-cost.png | 第三条吠点② | DeepMind MKA 技术报告表 4 与 4.2 节（6.26 倍、+2.22 pp 不显著），批注；来源 storage.googleapis.com 论文截图（2026-11-11 存档） |

## 备用图

| 文件 | 用途 |
|---|---|
| aa-cost-raw.png | 批注底图 |
| blog-6x-raw.png | 批注底图 |
| blog-promise-raw.png | 批注底图 |
| carbon-pricing-raw.png | 批注底图 |
| help-review-raw.png | 批注底图 |
| jan-snapshot-raw.png | 批注底图 |
| melaugh-footer-raw.png | 批注底图 |
| melaugh-upfront-raw.png | 批注底图 |
| mka-cost-raw.png | 批注底图 |
| theo-work-raw.png | 批注底图 |
| thinking-summary-raw.png | 批注底图 |
| 01-help-human-review.png | 取证截图 |
| 02-help-dont-connect.png | 取证截图 |
| 03-help-human-reviewed-per-notice.png | 取证截图 |
| 04-help-keep-activity-train.png | 取证截图 |
| 05-help-eligibility.png | 取证截图 |
| 06-blog-promise-and-update.png | 取证截图 |
| 07-wayback-blog-1012-before-update.png | 取证截图 |
| 08-hub-human-review-chats.png | 取证截图 |
| 09-hub-retention-three-years.png | 取证截图 |
| 10-hub-keepactivity-off.png | 取证截图 |
| 11-readme-title-clarification.png | 取证截图 |
| 12-workspace-hub-not-human-reviewed.png | 取证截图 |
| 12b-hub-connected-apps-keepactivity.png | 取证截图 |
| 12c-hub-72h-feedback-connected-apps.png | 取证截图 |
| 12d-workspace-hub-personal-intelligence-no.png | 取证截图 |
| 12e-wayback-help-20260114-human-review.png | 取证截图 |
| 12f-help-personal-intelligence-requirements.png | 取证截图 |
| 13-melaugh-card.png | 取证截图 |
| 14-melaugh-wording-chart.png | 取证截图 |
| 15-melaugh-effort-chart.png | 取证截图 |
| 16-melaugh-single-bug.png | 取证截图 |
| 17-theo-card.png | 取证截图 |
| 18-theo-chart.png | 取证截图 |
| 19-thinking-doc-summarized.png | 取证截图 |
| 22-thinking-doc-never-raw-cot.png | 取证截图 |
| 24-effort-doc-default-medium.png | 取证截图 |
| 25-blog-price-ultra.png | 取证截图 |
| 26-pricing-carbon.png | 取证截图 |
| 27-blog-mka-6x.png | 取证截图 |
| 28-aa-mka-cost.png | 取证截图 |
| 29-aa-mka-climb-wsr.png | 取证截图 |
| 30-aa-intelligence-index-board.png | 取证截图 |
| 31-blog-internal-bench.png | 取证截图 |
| 32-mka-paper-cost-table.png | 取证截图 |
| 33-aa-mka-overview.png | 取证截图 |
| 34-aa-mka-cost-per-task.png | 取证截图 |
| 35-zapier-automationbench.png | 取证截图 |
| 36-blog-getting-started.png | 取证截图 |
| 37-ccode-effort-default-medium.png | 取证截图 |
| 38-ccode-opus55-medium-thinks-more.png | 取证截图 |
| 39-ccode-redacted-thinking-default.png | 取证截图 |
| 40-hn-bcherny-redact-thinking.png | 取证截图 |
| 41-hn-nerf-thread-header.png | 取证截图 |
| 43-livenerf-latest.png | 取证截图 |
| 45-livenerf-results-table.png | 取证截图 |
