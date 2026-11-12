# 1111 配图说明

标题：AI找到玛雅遗址？作者自己改口了

本期期号1111（上期1110+1）；主帖事件：48个 AI 智能体“发现”玛雅遗址与作者更正（HN 近千分，更正于北京 11-12 凌晨）、AWS Emulator“完全由智能体编写”（北京 11-12 凌晨）、Claroc 报告 Grok 应用无需登录（报告 11-10，媒体 11-11 起）。卡片日期写11月12日；取证与截图在北京 11-12 17:52 起。主帖顺序：玛雅 → AWS → Grok。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共16张（封面＋15），公众号不插三张事件卡与三张吠点卡。

Claroc 报告页是深色底，高亮按 OCR 定位；底图都由取证截图用 `barking shot cropbox` 裁出，像素未改。

## 封面

封面抽签：画风=粉笔涂鸦；底色=满幅场景；标题=顶部横排；看板娘=大特写；动作=扮演

- 抽签种子 1794644421188270408（`cover-draw` 按期号定种子，A、B 两组未拼取值）。主题对应：整面旧黑板上，用彩色粉笔画了一座玛雅阶梯金字塔，还圈了起来，像宣布“新发现”；旁边用磁铁钉着一张泛黄旧地图，上面早已画着同一座金字塔，对应“原是重新发现”。看板娘扮成野外考古队员，戴宽檐探险帽、穿卡其马甲，一手捏粉笔、一手挠头，回头看旧地图，尴尬想笑，担任“吠”的角色。
- 主标题取正文标题，两句分两行；副标题“原是重新发现”对应第一帖的作者更正。未写 AWS、Grok 两帖。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write，`-C` 为草稿目录）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张。选 B：故事一眼看懂（圈起来的“新发现”对上旧地图），墨绿黑板与 1108 深色黏土、1109 红绿年画、1110 浅色水彩都不同。A 组（亮橙底纸雕金字塔立在书页上）文字也无误，但“书里早有”的寓意要想一下才明白，落选。
- 验收：逐字核对主标题、副标题与胸牌，无多字、错字或乱码；黑板与地图上没有字母或数字。看板娘两只手（一只捏粉笔、一只挠头）、两只耳朵、一条尾巴，保持二次元画风。竖版 1086×1448；横版 1923×818，按居中 1:1（818×818）裁切，主标题与看板娘都在框内。已与 1108 竖版封面并排看过；1109、1110 只按 README 记录的风格比较。

B 组提示词（选中）：

> 竖版：粉笔涂鸦风格：一整面墨绿色旧黑板铺满画面，有擦痕和粉笔灰，黑板槽里放着几截彩色粉笔。主标题用白色粉笔大字横排写在黑板顶部，两行「AI找到玛雅遗址？」「作者自己改口了」，副标题用黄色粉笔小字写在主标题下方。黑板中部用彩色粉笔画了一座玛雅阶梯金字塔，被一个大圆圈圈起来、旁边画了感叹号形状的星星（不要任何字母或数字）；金字塔旁边用磁铁钉着一张泛黄的旧地图纸，地图上已经用墨线画着同一座金字塔——表示早有人画过。看板娘是大特写，上半身占画面下半部一半以上，扮成野外考古队员：戴卡其色宽檐探险帽、女仆装外披一件卡其色马甲，一只手捏着粉笔停在半空，另一只手挠头，转头看向那张旧地图，表情尴尬又想笑。
>
> 横版：粉笔涂鸦风格的横幅：墨绿色旧黑板铺满画面，有擦痕和粉笔灰。主标题白色粉笔大字横排在画面上部正中，两行「AI找到玛雅遗址？」「作者自己改口了」，副标题黄色粉笔小字在下。中间偏左用彩色粉笔画一座被圆圈圈起的玛雅阶梯金字塔，旁边磁铁钉着一张泛黄旧地图，上面已画着同一座金字塔。看板娘大特写在画面中央偏右，扮成考古队员（卡其色宽檐探险帽、卡其色马甲），一手捏粉笔一手挠头，回头看旧地图，尴尬想笑。主标题、金字塔与看板娘集中在画面正中方形区域内，两侧只留黑板与粉笔灰。

A 组提示词（落选）：

> 竖版：纸艺手工风格：多层纸雕加剪纸拼贴，纸张有厚度和投影。整幅高饱和亮橙色纯色底。画面主体是一本摊开的大纸书（书页上只用细横线表示文字，没有任何可读字），右页上是一座用绿色和赭石色卡纸层层叠出的立体玛雅阶梯金字塔，从书页里立起来——表示这座金字塔早就在书里。金字塔顶端插着一面刚插上去的小纸旗（旗面空白无字），像在宣布“新发现”，可金字塔本来就印在书里。主标题用粗大的白色剪纸字斜放，沿金字塔的斜坡从左下往右上排成两行，分为「AI找到玛雅遗址？」和「作者自己改口了」；副标题小一号白色剪纸字放在书页下沿。看板娘是站在书旁边的小人，只有金字塔一半高，双手捂住脸颊、张大嘴，一副“原来早就有了”的震惊表情。只保留纸书、金字塔、小纸旗和看板娘，留出空间。
>
> 横版：纸艺手工风格的横幅：多层纸雕加剪纸拼贴，亮橙色纯色底。画面正中是一本摊开的大纸书（书页只有细横线，无可读字），书页上立起一座绿色赭石色卡纸叠出的玛雅阶梯金字塔；金字塔顶插着一面空白小纸旗。主标题粗大白色剪纸字沿金字塔斜坡斜放，分两行「AI找到玛雅遗址？」「作者自己改口了」，副标题小字在书页下沿。看板娘是书左侧的小人，双手捂脸张嘴震惊。主标题、金字塔与看板娘集中在画面中央方形区域，两侧只留亮橙底与少量纸叶片。

（硬性要求按 `templates/codex-cover.md` 逐字照抄：主标题「AI找到玛雅遗址？作者自己改口了」分两行，副标题「原是重新发现」。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 三条事实，玛雅一条逐字取自第一条事实段，AWS、Grok 两条取自省流句（不放吠点） |
| 2 | publish/02-roundup.png | 今日速览 | 三条主帖小标题＋谷歌、Ring、DeepMind、台积电 |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（玛雅），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（玛雅），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-maya-update.png | 第一条事实段 | Miotti 博文现版本标题与文首更正段，批注；来源 pietromiotti.com 原文截图（2026-11-12 存档） |
| 6 | publish/06-maya-original.png | 第一条吠点① | 同一博文更正前版本（“no one had reported before”与文末引同一论文），批注；来源 web.archive.org 存档快照（北京 2026-11-11 04:13） |
| 7 | publish/07-event-2.png | 第二条事实段 | 事件卡（AWS），逐字取自正文事实段；公众号不插 |
| 8 | publish/08-bark-2.png | 第二条吠点 | 吠点卡（AWS），逐字取自正文吠点段；公众号不插 |
| 9 | publish/09-aws-claim.png | 第二条事实段 | AWS 官方文“written entirely by autonomous agents … hasn’t written or reviewed”段，批注；来源 aboutamazon.com 原文截图（2026-11-12 存档） |
| 10 | publish/10-aws-gate.png | 第二条吠点① | 官方文合并门禁段（智能体审查、数百个智能体），批注；来源 aboutamazon.com 原文截图 |
| 11 | publish/11-aws-readme.png | 第二条吠点② | 仓库 README 开头（“Don’t use it for production workloads or to test security.”），批注；来源 github.com 原文截图（2026-11-12 存档） |
| 12 | publish/12-event-3.png | 第三条事实段 | 事件卡（Grok），逐字取自正文事实段；公众号不插 |
| 13 | publish/13-bark-3.png | 第三条吠点 | 吠点卡（Grok），逐字取自正文吠点段；公众号不插 |
| 14 | publish/14-grok-open.png | 第三条事实段、吠点① | Claroc 报告“35,578 open apps”段（94.7%、HTTP 401），批注；来源 blog.claroc.io 原文截图（2026-11-12 存档，深色底） |
| 15 | publish/15-grok-test.png | 第三条事实段、吠点① | Claroc 报告自建团队应用测试段与图 6（示意图），批注；来源 blog.claroc.io 原文截图（深色底） |

## 备用图

| 文件 | 用途 |
|---|---|
| maya-update-raw.png | 批注底图 |
| maya-original-raw.png | 批注底图 |
| aws-claim-raw.png | 批注底图 |
| aws-gate-raw.png | 批注底图 |
| aws-readme-raw.png | 批注底图 |
| grok-open-raw.png | 批注底图 |
| grok-test-raw.png | 批注底图 |
| 01-blog-title-update.png | A 组取证截图 |
| 02-blog-48-agents.png | A 组取证截图 |
| 03-blog-lidar-zoom.png | A 组取证截图 |
| 04-blog-inah-reply.png | A 组取证截图 |
| 05-blog-literature.png | A 组取证截图 |
| 06-blog-not-archaeologist-models.png | A 组取证截图 |
| 07-wayback-original-intro-full.png | A 组取证截图 |
| 08-hn-title-google-swe.png | A 组取证截图 |
| 09-hn-doi-and-author-reply.png | A 组取证截图 |
| 10-x-original-post-card.png | A 组取证截图 |
| 11-x-correction-post-card.png | A 组取证截图 |
| 12-media-verge-techradar-headlines.png | A 组取证截图 |
| 13-aws-official-claim.png | B 组取证截图 |
| 14-aws-kiro-prompt.png | B 组取证截图 |
| 15-aws-three-teams-tests.png | B 组取证截图 |
| 16-aws-merge-gate.png | B 组取证截图 |
| 17-aws-python-interpreter.png | B 组取证截图 |
| 18-vogels-spec-tests.png | B 组取证截图 |
| 19-vogels-engineer-job.png | B 组取证截图 |
| 20-gh-repo-files.png | B 组取证截图 |
| 21-gh-readme-production.png | B 组取证截图 |
| 23-gh-readme-license-pr.png | B 组取证截图 |
| 24-gh-readme-telemetry.png | B 组取证截图 |
| 25-claroc-report-top.png | C 组取证截图 |
| 26-claroc-35578-open.png | C 组取证截图 |
| 27-claroc-30-orgs.png | C 组取证截图 |
| 28-claroc-two-test-apps.png | C 组取证截图 |
| 29-claroc-spacexsi-says.png | C 组取证截图 |
| 30-claroc-method-limits.png | C 组取证截图 |
| 31-claroc-timeline-why-published.png | C 组取证截图 |
| 32-claroc-where-we-fit.png | C 组取证截图 |
| 33-claroc-home-stealth.png | C 组取证截图 |
| 34-claroc-muse-first-report.png | C 组取证截图 |
| 35-thn-headline-lede.png | C 组取证截图 |
| 36-claroc-territory-map-fig2.png | C 组取证截图 |
| 37-evalsuite-readme.png | B 组取证截图 |
| 38-gh-things-do-not-work.png | B 组取证截图 |
| 39-docs-limitations.png | B 组取证截图 |
| 40-hn-50456631-title.png | B 组取证截图 |
| 42-news-siliconangle.png | B 组取证截图 |
| 43-news-zhiding.png | B 组取证截图 |
| 44-aws-fidelity-doc.png | B 组取证截图 |
