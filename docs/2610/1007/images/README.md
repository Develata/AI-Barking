# 1007 配图说明

正文：`../doc_1007_publish.txt`。来源见[来源入口](../sources/README.md)，事实核验见 `../sources/fact-check.md`，一手存档在 `../sources/`。卡片文字与批注内容写在 `cards.toml`，由 `barking card` 渲染。

状态：批注底图（`-raw`）取自各取证组与 Claude 的截图，像素未改：`41`–`43` 为协议 PDF 页渲染（A 组），`44` 为无头 Chrome 页面截图（C 组），`51`、`52` 为 Claude 用无头 Chrome（700 CSS px、2 倍像素比）渲染官方文档页后按 Tesseract 锚点行或像素框裁切（`../sources/g-shot.py`、`g-shots-log.jsonl`），`53`、`54` 为 Artificial Analysis 两个模型页的页头与摘要区裁切，`55` 为 Haiku-5.5 系统卡 PDF 第 115 页与第 116 页的两段裁切上下拼接（中间留 24 px 白边；本期未用）。`01`–`34`、`e-` 开头为 A/B/C/E 组取证截图，本期只作核验。

## 封面

已生成（Codex CLI 内置图像生成，gpt-6-astra medium，参考图 `../../../../common_images/profile_picture.png`）：[00-cover.png](00-cover.png)（1086×1448，3:4，小红书、抖音）、[00-cover-wide.png](00-cover-wide.png)（1919×820，约 2.34:1，公众号）。上传顺序排在所有配图之前。发布时按平台要求声明 AI 生成。封面图不入库，发布后存 OpenList。

本期画风：**日式超市特卖传单**——亮黄底带低对比放射条纹，粗圆黑体标题（黑字白描边、红色错位投影），红底白字圆角副标题条，红白条纹桌布的商品堆头台；呼应“降价”“打折”，副标题“三条都要打折”点出三条都带限定条件。与 1006 教室黑板粉笔画、1005 档案室证物吊牌、1004 游乐园须知牌、1003 作业本红笔、1002 航空邮件、1001 报刊社论漫画区分开。

主体（做减法，三件物 + 看板娘，同一张堆头台上）：
- Haiku：圆滚滚的卡通小蓝鸟玩偶（不是任何公司标志）；
- AI 开药：橙色药瓶，标签留白；
- 12.9 万漏洞：装满卡通虫子的玻璃罐（本期标题未写该条，画面保留，对应省流第三句）；
- 每件挂一张只画问号的红色星形价签。看板娘（按参考图）拿标价枪、歪头质疑，胸牌保留“AI 吠点”。

重画记录：第一版（同画风“超市促销海报”，现存为备用图 `00-cover-alt-sticker.png`、`00-cover-wide-alt-sticker.png`）被 Develata 判为“有点丑”，原因：三件物体像互不相干的贴纸散在平涂黄底上，与看板娘的细腻画风不统一，竖版左下大片空白，横版元素分散，毛笔标题粗糙。第二版改为同一画风、同一场景（堆头台），并加粗圆黑体标题与副标题条。第二版出 2 组候选（同一提示词，两次并行生成），选第 1 组：竖版放射条纹饱和、看板娘居中偏右且表情最足、标题与主体不重叠；横版色调饱和、标题与副标题清楚分开。落选的第 2 组：竖版看板娘的呆毛压到标题“批”字，横版黄底发白、副标题条贴住标题下缘。每种尺寸累计：第一版 2 张（2 次运行）加第二版 2 张，另有选题变更前的两组（旧三条标题）作废，不计入。

标题变更：Develata 把标题改为“Haiku降价90%？美AI开药获批？”（第二问加“美”字）。封面未重画，改用 Codex 图像编辑（gpt-6-astra medium，输入看板娘参考图加选中的封面）各做 1 次，只在标题第二行最前面加“美”字、其余不动；竖版 1086×1448，横版 1919×820。累计封面生成：选题变更前旧三条标题 2 组作废；当前标题的第一版 2 张、第二版 2 张（选 1 组）与这次 2 次编辑，每种尺寸共 5 次，达到上限，没有超出。

逐字核对（放大 4 倍看标题、副标题与胸牌）：画面文字只有“Haiku降价90%？”“美AI开药获批？”“三条都要打折”与胸牌“AI 吠点”，无错字、乱码、logo、国旗或官方建筑图案，标题与封面不含“政府”二字；横版主标题、副标题与看板娘主体都在居中 1:1 范围（x 约 550–1369）内，仅看板娘发梢最下端贴近范围左缘，药瓶被范围右缘截去一部分。此项由 Claude 目视核对（含“美”字，放大看标题第二行），未再送 Codex 视觉复核（视觉复核时看的是第一版封面）。

### 3:4 竖版提示词

竖版 3:4 封面插画。画风：日式超市特卖传单——统一的厚涂赛璐璐手绘，所有物件和看板娘用同一套粗深色描边与同一套上色光影，整体像一幅完整的画，不要贴纸拼贴感，不要扁平矢量剪贴画感；搞怪、有冲击力，非写实、无真人。背景是明亮的黄色，带深浅两种黄的淡淡放射条纹（低对比，不抢主体），不画其他背景细节。
最上方是主标题，分两行：“Haiku降价90%？”“美AI开药获批？”。粗圆黑体字（不用毛笔字），黑字加粗白色描边，再加红色错位投影，像特卖海报标题，是全画面最醒目的元素，宽度占画面约九成。标题下方一条小号的红底白字圆角横条，写副标题：“三条都要打折”。
画面中下部是一张商品堆头台（矮桌，台面铺红白条纹桌布），台上并排摆三件主体，体量大，彼此留空隙，不堆小物件：
1. 左：圆滚滚的卡通小蓝鸟玩偶（不是任何公司标志，身上不写字）；
2. 中：橙色药瓶（瓶身标签留白）；
3. 右：装满小小卡通虫子的玻璃罐。
每件主体都挂一张红色星形价签，价签上只画一个白色大问号。
看板娘（按参考图）站在堆头台后方右侧，大小与主体相近，不遮挡三件主体，一手拿标价枪（枪上不写字）对着药瓶，歪头质疑，表情狡黠。
除“Haiku降价90%？”“美AI开药获批？”“三条都要打折”与胸牌“AI 吠点”外，画面中不出现任何其他文字、字母、数字、符号或 logo（含各公司商标，不画国旗、官方建筑或印章）。用常用字，字迹清楚，手机缩略图下主标题可读。

### 2.35:1 横版提示词

横版 2.35:1 封面插画，按横构图重新排版（不是竖版裁切）。画风同上：日式超市特卖传单，统一的厚涂赛璐璐手绘，所有物件和看板娘同一套描边与上色，不要贴纸拼贴感；明亮黄色背景，带低对比的深浅黄放射条纹，非写实、无真人。
主标题放在画面正中央安全区，粗圆黑体字（不用毛笔字），黑字加粗白色描边与红色错位投影：“Haiku降价90%？美AI开药获批？”（一行放不下可分两行）；其正下方一条小号红底白字圆角横条，写副标题：“三条都要打折”。主标题与副标题都完整落在画面中央、宽度等于画面高度的正方形范围内。
画面下部横贯一张长条商品堆头台（红白条纹桌布），三件主体都摆在台面上：左端是圆滚滚的卡通小蓝鸟玩偶（身上不写字）；正中是看板娘（按参考图，在中央正方形内、副标题下方），一手拿标价枪（枪上不写字）、歪头质疑；右端左边一个橙色药瓶（标签留白），右边一个装满小小卡通虫子的玻璃罐。每件主体挂一张红色星形价签，价签上只画一个白色大问号。主体之间留出空隙，不堆小物件。
除“Haiku降价90%？美AI开药获批？”“三条都要打折”与胸牌“AI 吠点”外，不出现任何其他文字、字母、数字、符号或 logo（含各公司商标，不画国旗、官方建筑或印章）。

## 正式配图（按上传顺序）

三个平台的上传顺序一致：封面之后按下表顺序（封面 → 省流卡 → 速览图 → 批注截图）上传；公众号也插省流卡，批注截图可按“对应段落”手动排版。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `00-tldr.png` | 省流卡（本号整理，非原文截图） | 省流 | 每条一句事实，逐字取自正文。 |
| 2 | `00-roundup.png` | 速览聚合图（本号整理，非原文截图） | 速览（不在正文） | 主帖三条标“详见前页”；其余七条：GPT-6 Intelligent UI、Anthropic 月度 API 额度、SynthID 检测器、Nemotron、openTPU、METR、核电增容。 |
| 3 | `51-haiku-pricing.png` | Anthropic 定价文档：Haiku 5.5 行（批注版） | 一、事实段与吠点① | 10万 token 以内输入价 $0.10，超过为 $0.50。“降90%”的官方原句在发布页脚注（`../sources/f-haiku-5-5.txt`），发布页是滚动动画页，无法截图。 |
| 4 | `52-tokenizer.png` | Haiku 5.5 迁移指南：分词器（批注版） | 一、吠点① | 同样文本约多 30% token。 |
| 5 | `53-aa-haiku.png` | Artificial Analysis：Haiku 5.5（Max）页头与摘要（批注版） | 一、吠点② | 指数 43；每个指数任务 $0.21；输出 440M token。 |
| 6 | `54-aa-luna.png` | Artificial Analysis：GPT-6 Luna（Max）页头与摘要（批注版） | 一、吠点② | 指数 38；每个指数任务 $0.07；输出 140M token。 |
| 7 | `41-rma-not-approval.png` | 犹他州协议第 3.E 条（批注版） | 二、吠点① | “仅是授予监管缓执，并不构成认可或批准”。州方网站的同义表述见 `fact-check.md`。 |
| 8 | `42-rma-stages.png` | Nolla 提案 4.11 分阶段部署（批注版） | 二、吠点② | ① 第一阶段每张处方由两名独立的持证医生审；② 第三阶段每月抽样复核至少 10%。 |
| 9 | `43-rma-96pct.png` | Nolla 提案：真实世界一致率（批注版） | 二、吠点③ | “最近1000次治疗”中约 96% 一致；公司自述。 |
| 10 | `44-cvp-numbers.png` | Anthropic 公告：12.9万、5500、5 倍（批注版） | 三、事实段与吠点①② | ① 合作方 4–7 月；② 自家扫描 4–10 月；③ “预计”至少高 5 倍。 |

平台限制图数时，依次删图 9、4、10。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `00-cover-alt-sticker.png`、`00-cover-wide-alt-sticker.png` | 第一版封面（贴纸拼贴风） | 被 Develata 判“有点丑”后换下，留作备用，不入正式配图 |
| `41-rma-not-approval-raw.png`、`42-rma-stages-raw.png`、`43-rma-96pct-raw.png`、`44-cvp-numbers-raw.png`、`51-haiku-pricing-raw.png`、`52-tokenizer-raw.png`、`53-aa-haiku-raw.png`、`54-aa-luna-raw.png`、`55-haiku-tbench-raw.png` | 各批注图的原始底图 | 批注底图（像素未改；`53`、`54` 为页面区域裁切，`55` 为两段拼接） |
| `01-a-rma-sec2-3e-4b.png`、`02-a-rma-sec19-abc.png`、`03-a-rma-sec19-def.png`、`04-a-rma-411-stages.png`、`05-a-rma-96pct.png`、`06-a-rma-15-insurance.png`、`07-a-oaip-nolla-active.png`、`08-a-oaip-not-endorsement.png`、`09-a-nolla-blog-first-stages.png`、`10-a-prn-first-stages.png`、`11-a-latimes-stages.png`、`12-a-latimes-whyte.png`、`13-a-latimes-third-party.png`、`14-a-oaip-release-nolla.png` | A 组：协议 3.E、Section 19、4.11、96%、保险前置条件、州方页、Nolla 官网与 PR、LA Times 各段、州方新闻稿 | 核验；不入正式配图 |
| `25-c-cvp-numbers.png`、`26-c-cvp-survey-table.png`、`27-c-cvp-cyscenario-text.png`、`28-c-cvp-cyscenario-chart.png`、`29-c-cvp-tier-table.png`、`30-c-cvp-tier-table-clean.png`、`31-c-help-overview.png`、`32-c-help-how-to-apply.png`、`33-c-help-once-approved.png`、`34-c-irregular-opus55-67-6.png` | C 组：公告数字段、调查图表、CyScenarioBench 文字与柱图、档位总表、帮助中心各节、Irregular 评估页 | 核验；`29-c-cvp-tier-table.png` 上沿切进上一行，不用 |
| `15-b-blog-49b-active.png`、`16-b-aa-cyber-index-chart.png`、`16-b-blog-title-preview.png`、`17-b-blog-preview-weights.png`、`18-b-doc-52b-description.png`、`18-b-doc-weights-coming-soon.png`、`19-b-blog-cybench-chart.png`、`19-b-doc-price-card.png`、`20-b-blog-cybench-near-zero.png`、`21-b-arena-post-card.png`、`21-b-arena-webdev-header.png`、`22-b-arena-webdev-row45.png`、`23-b-aa-summary.png`、`24-b-doc-sale-tooltip.png`、`45-cvp-tiers-raw.png`、`45-cvp-tiers.png`、`46-ml4-weights-raw.png`、`46-ml4-weights.png`、`47-ml4-doc-52b-raw.png`、`47-ml4-doc-52b.png`、`48-aa-cyber-raw.png`、`48-aa-cyber.png` | B 组与撤下的 Mistral 卡片：博客 49B 段、AA 图、文档页 52B、WEIGHTS 页签、Cybench 图、Arena 帖与榜单行、价格悬浮提示；`45`–`48` 为按撤下的 Mistral / Anthropic 档位底图渲染的批注卡 | 本期撤下，不入正式配图；`20-b-blog-cybench-near-zero.png` 记录“近零”一句的上下文 |
| `e-grok-bot-post-raw.png`、`e-grok-bot-post-raw2.png`、`e-grok-bot-post.png` | E 组：SpaceX/Grok 机器人帖子卡片（官方嵌入页渲染） | 本期不用；留作后续 |
