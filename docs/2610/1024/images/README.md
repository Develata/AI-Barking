# 1024 配图说明

标题：Kimi 推广被曝：别标付费合作？

本期期号1024；新闻与取证日期为2026-10-25，卡片日期写10月25日。用户确定主帖顺序：Kimi 推广说明风波【跟进1023】 → Apollo 与 Apricitas → METR 审计 OpenAI 监控【跟进1020】。

封面二选一：小红书/抖音用 `00-cover.png`（3:4），公众号用 `00-cover-wide.png`（约2.35:1）。封面后按下表上传，单个平台总计12张。公众号也插省流卡。正文独立可读。

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | 00-tldr.png | 省流 | 三条事实，逐字摘自正文事实段 |
| 2 | 00-roundup.png | 今日速览 | 三条主帖标题＋MiniMax、霍普金斯湾景、Reservoir、猫王、英国 AI 生物安全实验室五条速览 |
| 3 | 01-kimi-brief-doc.png | 第一条事实段 | 爆料者所晒推广说明截图（原图带“Guest User 19082”水印与蓝色手绘箭头，均为原图所有）：送审、事先获批、DO NOT 付费合作标签、不必链官网与标官方账号；标题注明未经独立验证 |
| 4 | 02-kimi-post4-doc.png | 第一条事实段 | 同一爆料者所晒话术模板截图：“15x cheaper”（金额为占位符 $xx）、“literally Open Source Opus 5.5”、“must compare the cost”；原图自带黄色高亮，本号高亮叠加其上 |
| 5 | 05-aa-cost.png | 第一条吠点① | Artificial Analysis 三模型对比页：Cost per Task $2.11 / $5.98 / $3.26（Opus 5.5 为 Max, Default Fallback，Astra 为 Max）；表头条与成本段竖拼 |
| 6 | 13-apollo-zero.png | 第二条事实段、吠点① | Apollo Daily Spark 导语：“zero financial payoff”与“pulling back … cost center”句 |
| 7 | 23-ramp-summary.png | 第二条吠点① | Ramp AI Index 摘要：9月采用率55.9%，环比+0.6、同比+12.1个百分点 |
| 8 | 24-ramp-share.png | 第二条吠点① | Ramp 指标表表头条与目标段竖拼：AI 占企业支出（不含工资）9月5.77%、8月5.96% |
| 9 | 26-metr-headers-ctx.png | 第三条事实段 | METR：两个客户端可控请求头可关监控；“could be caught”限定句同框 |
| 10 | 33-metr-ratios.png | 第三条吠点① | METR：约75–94%“eligible for monitoring”，“substantial uncertainty in both directions” |
| 11 | 29-metr-notevaluated-ctx.png | 第三条吠点② | METR：OpenAI 称影响生产防护栈的漏洞已修补；“We have not evaluated these claims.” |

## 备用图

| 文件 | 用途 |
|---|---|
| 01-kimi-brief-doc-raw.png、02-kimi-post4-doc-raw.png、05-aa-cost-raw.png、13-apollo-zero-raw.png、23-ramp-summary-raw.png、24-ramp-share-raw.png、26-metr-headers-ctx-raw.png、33-metr-ratios-raw.png、29-metr-notevaluated-ctx-raw.png | 批注底图（01/02 由爆料帖附图原件转 PNG，像素未改；26/29 取自视觉复核后重截的 26-metr-headers-raw.png、29-metr-notevaluated-raw.png（页面标题块与发现段竖拼），再裁去下文以免卡片过小） |
| 01-kimi-brief-raw.png、02-kimi-post4-raw.png、03-kimi-official-raw.png | 帖子卡片截图（嵌入页折叠了文字与图片，不作底图）；03 为 Kimi 官方发布帖 |
| 26-metr-headers-raw.png、29-metr-notevaluated-raw.png | METR 标题块与发现段竖拼原版（较长，ctx 底图由此裁出） |
| 24-ramp-indicators-raw.png | Ramp 指标表全表（24-ramp-share-raw.png 的来源；全表缩进卡片后过小） |
| 04-arena-agent-raw.png | Arena Agent 榜四段竖拼（同页滚动截取），Kimi 第11、$1.22，第10 Fable 5 (High)；未用 |
| 06-kimi-3d-raw.png | 爆料者转述 Reddit 3D 帖的卡片（文字不全，仅证帖子存在） |
| 14-apollo-revenue-raw.png、15-apollo-margin-raw.png、16-apollo-tfp-raw.png、21-apollo-concentration-raw.png、22-apollo-vc-raw.png | Apollo 五张图（官方图表 PDF 渲页，含图注） |
| 17-apollo-judgment-raw.png | Apollo “intelligence is mostly judgment”段 |
| 18-apricitas-200k-raw.png、19-apricitas-difficult-raw.png、20-apricitas-books-raw.png | Apricitas 20万岗位、“extremely difficult”段、32%/62% 与书籍段 |
| 25-metr-scope-raw.png、27-metr-compaction-raw.png、28-metr-redacted-raw.png、32-metr-footnotes-raw.png、36-metr-tools-raw.png | METR 审计范围、10%压缩段、涂黑漏洞段、脚注（注入 CSS 显示弹窗脚注，文字原样）、只读工具段 |
| 30-openai-escape-raw.png、31-openai-airgap-raw.png、34-openai-policy-dns-raw.png、35-openai-cache-raw.png、36-openai-resume-raw.png | OpenAI 训练环境帖：逃逸、气隙、政策与 DNS、缓存侧信道、10/1 恢复 |

## 制作和来源

截图由各取证组用 opencli / barking shot 取得，日志见 `sources/*-shots-log.jsonl` 与 `capture-log-*.md`。批注卡由 Sonnet 子代理按 cards.toml 渲染：01 第3条、02 全部、23、24、33、29 第1条与 05 用手量像素框（OCR 把 “AI” 读成 “Al”、黄底粗体读不出、低置信度），其余用 OCR 原句定位。高亮与译注为本号后加，原文像素不改。全部图片仅本地保留，发布后按 workflow 上传 OpenList。

## 封面

封面抽签：画风=复古印刷；底色=浅色；标题=嵌进物件；看板娘=半身；动作=查验
（2026-10-26 按成图补记，供 barking cover-draw 排除近期取值；本期封面当时未抽签。）

画风：1950年代美式杂志平面广告插画（水粉平涂、粗墨线、有限的奶油黄、薄荷绿、珊瑚红配色），呼应“推广/广告”主题；上期为复古丝网印刷体育海报。主标题做成广告牌上的广告语，即“标题嵌进物件”。广告牌角落一张空白小标签正被揭下（不写字），看板娘拿放大镜从下方查看那张标签。主标题「“别标广告”？」为问句，不含品牌名、不宣称数字，不画任何公司 logo。

生成方式：Codex CLI 内置图像生成（gpt-6-astra medium），参考仓库通用看板娘全身设定图；本期 `-s danger-full-access` 被本机权限拦下，改为 `-s workspace-write` 并以草稿子目录为工作区运行，结果相同。竖横各2候选：
- 竖版选候选1：主标题逐字为「“别标广告”？」；胸牌放大4倍为「AI／吠点」；两只正常人手（一手托腮、一手持放大镜）；机器人无标志；揭起的空白标签无字。候选2文字也正确，但机器人手部变形、构图更挤，落选。
- 横版选候选1：主标题逐字正确，居中820×820方形裁切（x=549–1369）后标题与看板娘完整；胸牌放大4倍为「AI／吠点」；两只正常人手。候选2构图更干净，但胸牌“点”字笔画走样，落选。

尺寸：竖版1086×1448（精确3:4），横版1918×820（约2.34:1）。候选留在会话草稿目录，不入库。发布时声明封面为AI生成。

### 3:4 竖版提示词

为中文AI新闻栏目「AI 吠点」制作编辑插画封面，1950年代美式杂志平面广告插画风格：水粉平涂、粗墨线、轻微纸张颗粒，配色限奶油黄底、薄荷绿、珊瑚红与墨黑，大块留白，强烈视觉钩子，手机缩略图可读。画面中央偏上是一块复古路边广告牌，广告牌上的广告语就是唯一的超大主标题，逐字为「“别标广告”？」，用复古手绘广告字体，分一到两行，占画面宽度七成；画面中不得出现其他任何文字、数字、字母或logo。广告牌左侧画一个笑眯眯、圆滚滚的通用玩具机器人（无任何标志），竖起大拇指做推销状。广告牌右下角有一张小小的空白方形标签（上面没有字）正被揭起一半，边角卷起。参考图仅用于保持看板娘外貌：蓝色长发、狗耳、鲸鱼尾、女仆装、胸前「AI 吠点」圆牌，双手是正常的人手、不戴手套。看板娘半身出现在画面右下，一手举放大镜凑近那张卷起的空白标签，另一手托腮，挑眉怀疑。不要真人、不要人群、不要额外道具。3:4竖版，1080×1440或同等比例。

### 2.35:1 横版提示词

为中文AI新闻栏目「AI 吠点」制作编辑插画封面，1950年代美式杂志平面广告插画风格：水粉平涂、粗墨线、轻微纸张颗粒，配色限奶油黄底、薄荷绿、珊瑚红与墨黑，大块留白，强烈视觉钩子，手机缩略图可读。画面是一块横向的复古路边广告牌，广告牌上的广告语就是唯一的超大主标题，逐字为「“别标广告”？」，用复古手绘广告字体，一行或两行，完整置于画面宽度35%至65%之内、画面上半部；画面中不得出现其他任何文字、数字、字母或logo。广告牌左端画一个笑眯眯、圆滚滚的通用玩具机器人（无任何标志），竖起大拇指做推销状。广告牌下沿中间有一张小小的空白方形标签（上面没有字）正被揭起一半。参考图仅用于保持看板娘外貌：蓝色长发、狗耳、鲸鱼尾、女仆装、胸前「AI 吠点」圆牌，双手是正常的人手、不戴手套。小巧的看板娘半身完整位于画面宽度40%至62%的下半部，举放大镜凑近那张卷起的空白标签，挑眉怀疑，头发、狗耳、双手、胸牌完整。右侧大片留白。2.35:1横版，1920×819或同等比例，独立横构图；中央方形裁切后标题与看板娘必须完整保留。
