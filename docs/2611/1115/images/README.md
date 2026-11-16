# 1115 配图说明

上传：全选 `images/publish/`，按名称排序即上传顺序（封面两张在最前）。小红书、抖音按全部顺序上传；公众号同序但不插事件卡与吠点卡（03、04、11、12）。卡片内容见 `cards.toml`，由 `barking card` 渲染。

## 封面

封面抽签：画风=实物摄影；底色=亮色；标题=顶部横排；看板娘=半身；动作=反应

- `cover-draw` 按期号定种子，A、B 两组未拼取值。主题对应：实物摄影取“桌面产品静物”，高饱和薄荷绿底配一块亮橙背景纸：一台复古黄铜天平，两边托盘各一堆硬币、一多一少明显倾斜，对应“同样说省钱，两家第三方称出两个数”；看板娘以手办形式半身入画，双手捂脸、从指缝瞪眼震惊（反应）。主标题粗黑字横排在顶部。无副标题。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write，`-C` 为草稿目录）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张，共 4 张（竖版 1086×1448，横版约 1922×818）。选 B。A（黏土鲸鱼收银机吐出长收据、标题沿纸带斜排、看板娘大特写拽纸带）点子更贴题，但浅纸底、上方斜标题、右下大特写三样与 1113 封面一致，缩略图一眼同系列；横版的头发与尾巴也超出中央 1:1 方框，故淘汰。
- 验收：逐字核对主标题「千问省六成？腾讯便宜20倍？」，无多字、错字或乱码；硬币与天平无文字、图案或品牌标志。看板娘两只手（捂脸）、两只垂耳、一条尾巴，没有手套。胸牌放大约 4 倍核对：竖版「AI 吠点」正确；第一张横版胸牌第二行读作「咲点」（右半成了“关”），交付前发现，横版另出一张（提示词末尾加“‘吠’是口字旁加‘犬’，不是‘咲’或‘笑’”，并附第一张横版作构图参考），新图胸牌「AI 吠点」正确。新横版按居中 1:1（x 552–1370）裁切：标题左端“腾”与右端问号各被切掉约 10–27 像素、尾巴尖出框，公众号裁缩略图时可略向左移；横版累计 2 张（上限 5）。

B 组提示词（选中）：

> 竖版：实物摄影风：桌面产品静物摆拍，微距浅景深，整幅高饱和薄荷绿底配一点亮橙撞色。主体：一台复古黄铜天平，两边托盘里各放一小堆闪亮硬币，一边多一边少，天平明显倾斜（硬币上没有任何字或图案）。主标题用粗壮的黑色大字横排在画面顶部。看板娘以手办形式出现，半身入画（腰部以上），站在天平旁，双手捂脸、从指缝里露出瞪大的眼睛，一脸震惊（反应）。手办仍是参考图的二次元画法与涂装。只保留天平一个主体。
>
> 横版：实物摄影风横幅：桌面静物摆拍，微距浅景深，整幅高饱和薄荷绿底，右侧一块亮橙色背景纸撞色。正中一台复古黄铜天平，两个托盘里各一小堆硬币（硬币上没有字或图案），一高一低。主标题用粗黑大字横排在顶部中央。看板娘手办半身站在天平右侧，双手捂脸、指缝里瞪大眼睛震惊。

A 组提示词（落选；抽签：画风=立体模型；底色=浅色；标题=斜放竖排；看板娘=大特写；动作=动手）：

> 竖版：立体模型风：黏土定格动画质感的桌面微缩场景，米白纸色底，柔和的影子。主体：一台圆滚滚的黏土收银机，正往外吐一条长得夸张的空白收据纸带（纸面只有浅灰横线，没有任何字和数字），纸带一路弯曲飘过画面。主标题沿着这条弯曲的收据纸带斜着排列，像印在纸带上的粗黑大字。看板娘大特写，上半身占画面一半以上，从画面右下角探进来，双手用力拽住收据纸带往外拉（动手），表情半信半疑地眯眼看纸带。只保留收银机与纸带一个主体，不堆杂物。
>
> 横版：立体模型风横幅：黏土定格动画质感的微缩场景，浅灰米白底。画面中央一台圆滚滚的黏土收银机，向左右两侧吐出一条长长的空白收据纸带（只有浅灰横线，没有字和数字），纸带在中间拱成一道斜坡。主标题斜放、沿着中段纸带排成粗黑大字。看板娘大特写，脸和上半身占中央方形区域的一半，双手攥着纸带往自己这边拉，眯眼怀疑。

（硬性要求按 `templates/codex-cover.md` 逐字照抄：主标题「千问省六成？腾讯便宜20倍？」，无副标题。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字取自正文事实段（不放吠点）；uBlock 一条只取到“有漏洞” |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋苹果健康 AI 研究、Xena 博客谈“Lean 检查过”、韩国税务师考试 AI 作弊自首 |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（千问·混元），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（千问·混元），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-qwen-cost.png | 第一条事实段 | Qwen 博客中英文成本句（58% 人民币、62% 美元，同一项 Work360 评测；深色页面），批注；来源 qwen.ai 原文截图（2026-11-16 存档） |
| 6 | publish/06-aa-cost.png | 第一条事实段 | AA 两代千问每任务费用（$3.27、$10.02；两页拼接），批注；来源 artificialanalysis.ai 原文截图（2026-11-16，两页） |
| 7 | publish/07-vals-q.png | 第一条吠点① | Vals Index 表头与第 11、18 行拼接（$9.33、$8.73；展开折叠区后整页截图裁切，1 倍像素比），批注；来源 vals.ai 原文截图（2026-11-17 存档，表头与两行拼接） |
| 8 | publish/08-hy4-workindex.png | 第一条事实段、吠点② | Hy4 flash 官方页“Hy4 的 1/20”句与 WorkIndex 图（Hy4 flash 42.1、Hy4 51.4），批注；来源 hunyuan.tencent.com 原文截图（2026-11-16 存档） |
| 9 | publish/09-hy4-blind.png | 第一条吠点③ | 85 位内部用户盲测句与盲测表（4分制表头、3.09、2.79、2.81、外部模型 high 档脚注），批注；来源同上 |
| 10 | publish/10-hf-hy4.png | 第一条吠点④ | Hugging Face 搜“tencent hy4”的两个结果（Hy4-preview-FP8、Hy4-preview，Aug 28 更新；深色页面），批注；来源 huggingface.co 原文截图（2026-11-17 存档） |
| 11 | publish/11-event-2.png | 第二条事实段 | 事件卡（uBlock），逐字取自正文事实段；公众号不插 |
| 12 | publish/12-bark-2.png | 第二条吠点 | 吠点卡（uBlock），逐字取自正文吠点段；公众号不插 |
| 13 | publish/13-ubo-ai.png | 第二条事实段 | 研究者博文 AI 分工段（GPT-5.6 Sol 找、他提示方向并验证、Claude 写演示代码），批注；来源 iter.ca 原文截图（2026-11-17 存档） |
| 14 | publish/14-ubo-click.png | 第二条吠点① | “Getting the click”段首句（I didn’t bother actually implementing this, but … opacity to 0），批注；来源同上 |
| 15 | publish/15-ghsa-fix.png | 第二条事实段、吠点② | 安全公告版本字段与四项修复时间线（同页两段拼接，深色页面），批注；来源 github.com 原文截图（2026-11-17，同页两段） |
| 16 | publish/16-ghsa-reply.png | 第二条吠点② | 公告内嵌研究者 11-02 来信：仍能拿到扩展 UUID（裁掉上方维护者回复；深色页面），批注；来源 github.com 原文截图（2026-11-17 存档） |

## 备用图

| 文件 | 用途 |
|---|---|
| qwen-cost-raw.png | 批注底图 |
| aa-cost-raw.png | 批注底图 |
| vals-q-raw.png | 批注底图 |
| hy4-workindex-raw.png | 批注底图 |
| hy4-blind-raw.png | 批注底图 |
| hf-hy4-raw.png | 批注底图 |
| ubo-ai-raw.png | 批注底图 |
| ubo-click-raw.png | 批注底图 |
| ghsa-fix-raw.png | 批注底图 |
| ghsa-reply-raw.png | 批注底图 |
| 01-qwen-title-en.png | 取证截图（A 组） |
| 02-qwen-cost-zh-en.png | 取证截图（A 组） |
| 03-qwen-bench-table.png | 取证截图（A 组） |
| 04-qwen-token-chart.png | 取证截图（A 组） |
| 05-aa-qwen4max-summary.png | 取证截图（A 组） |
| 06-aa-cost-per-task-chart.png | 取证截图（A 组） |
| 07-aa-qwen38max-summary.png | 取证截图（A 组） |
| 08-hy4flash-title-zh.png | 取证截图（A 组） |
| 09-hy4flash-params-price-zh.png | 取证截图（A 组） |
| 10-hy4flash-1over20-workindex-fig1.png | 取证截图（A 组） |
| 10b-hy4flash-open-benchmarks-fig1.png | 取证截图（A 组） |
| 11a-hy4flash-fig2-cost.png | 取证截图（A 组） |
| 11b-hy4flash-blind-test.png | 取证截图（A 组） |
| 12a-hy4flash-availability-open-source.png | 取证截图（A 组） |
| 12b-hf-tencent-hy4-search.png | 取证截图（A 组） |
| 12c-ithome-headline-over60.png | 取证截图（A 组，夸大实例） |
| 12d-hy4flash-workbuddy-quote.png | 取证截图（A 组） |
| 12e-tokenhub-price-5off.png | 取证截图（A 组） |
| 12f-vals-index-table.png | 取证截图（A 组，只含 Qwen4 行） |
| 12h-vals-table-rows.png | 取证截图（Claude 补截，含 Qwen3.8 行） |
| 13-iter-header.png | 取证截图（B 组） |
| 14-iter-ai-credit.png | 取证截图（B 组） |
| 15-iter-uuid.png | 取证截图（B 组） |
| 16-iter-opt-click.png | 取证截图（B 组） |
| 17-ghsa-versions.png | 取证截图（B 组） |
| 18-ghsa-timeline.png | 取证截图（B 组） |
| 19-ghsa-email-bypass.png | 取证截图（B 组） |
| 20-ghsa-cve-severity.png | 取证截图（B 组） |
| 21-releases-176x.png | 取证截图（B 组） |
| 22-releases-175x.png | 取证截图（B 组） |
| 23-amo-version.png | 取证截图（B 组） |
| 24-hn-title.png | 取证截图（B 组） |
| 24-tsn-exaggeration.png | 取证截图（B 组，夸大实例） |
