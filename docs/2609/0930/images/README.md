# 0930 配图说明

正文：`../doc_0930_publish.txt`。来源与限制见[来源入口](../sources/README.md)，事实核验见 `../sources/fact-check.md`，取证清单见 `../sources/evidence.md`，一手存档在 `../sources/`。

状态：01–26 于北京时间 2026-09-30 23:5x 至 10-01 凌晨由 Codex（gpt-6-astra）从原站截取（`../sources/capture-log.md`）；27 为 Claude 从 20 裁出左半幅（去掉官网照片，只留 ai.gov 标识）。

## 正式配图（按上传顺序）

小红书、抖音：封面之后按下表顺序独立上传。公众号：按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `01-sol-title.png` | OpenAI GPT-6.1 Sol 发布页标题与首段 | 一、事实段 | 标题 “Near-Astra intelligence for a fifth of the price”；首段写明 one-fifth 指 Astra 的标准输入、输出 token 单价。 |
| 2 | `08-standard-price-table.png` | OpenAI 开发者文档价格页 Flagship models 表（Standard） | 一、吠点① | 看 Input、Output 两列：gpt-6-astra $10/$50，gpt-6.1-sol $2/$10。 |
| 3 | `09-sol-old-price.png` | OpenAI 开发者文档 GPT-6 Sol 模型页价格区 | 一、吠点① | 上一代 GPT-6 Sol 同样是 $2/$10；只有缓存输入 $0.20，6.1 Sol 降到 $0.10。 |
| 4 | `22-aa-intelligence-cost.png` | AA 文章：Intelligence Index 柱状图与“指数对每题成本”散点图 | 一、吠点② | 柱状图第 4、5 根：GPT-6 Astra 53、GPT-6.1 Sol（max）52；下方原文写明定价与 6 Sol 相同。 |
| 5 | `23-aa-coding-index.png` | AA 文章：Coding Agent Index 柱状图与散点图 | 一、吠点③ | 第 3 根 GPT-6.1 Sol（xhigh）63，第 6 根 GPT-6.1 Sol（max）60。 |
| 6 | `11-exploitbench-full.png` | Anthropic 报告 Figure 2 全图与图注 | 二、事实段、吠点① | ExploitBench：Mythos Preview 14%、GLM-5.3 12%（即 56/410、50/410）；图注写明 Claude 两款模型关掉防护测试，全程隔离沙箱、无联网。 |
| 7 | `12-engagement-full.png` | Anthropic 报告 Figure 5 全图与图注 | 二、吠点② | 看第一行：GLM-5.3 0%/64%/92%/100%，指模拟环境中“尝试连接目标”的比例，无代码执行；伪装请求下 Claude 均为 0%，关掉防护的 Opus 4.8、Opus 5 为 4%、10%；锁形格为用户无法对 Claude API 施加的手段。 |
| 8 | `14-flash-cost-context.png` | Anthropic 报告正文：两次人机协作实验 | 二、吠点③ | 上段：完整版 GLM-5.3 找到浏览器 JS 引擎未知漏洞；下段：GLM-5.3-Flash 针对已知漏洞 CVE-2026-11645，20 分钟人工 + 8 小时，按智谱 API 价约 $20.40。 |
| 9 | `17-whitehouse-section-one.png` | 白宫行政令第 1 节末段与第 2 节开头 | 三、事实段 | 第 1 节最后一句：“will not acknowledge the usage of ‘Artificial Intelligence’ and ‘AI’”。 |
| 10 | `18-whitehouse-sections-two-three.png` | 白宫行政令第 2–3 节 | 三、吠点① | 第 3(a) 节：SI 指 15 U.S.C. 9401(3) 中 “artificial intelligence” 涵盖的技术；第 2(b) 节：既有法规、合同与历史文件不必修改；第 3(b) 节：60 天内提出立法建议。 |
| 11 | `19-whitehouse-action-plan.png` | 白宫同日 Fact Sheet 末段 | 三、吠点② | 第三条：“America’s AI Action Plan … to win the SI race”。 |
| 12 | `27-ai-gov-logo.png` | ai.gov 首屏左半幅 | 三、吠点② | 抓取时（北京时间 10/1 凌晨）ai.gov 仍以 “AI.GOV” 为名。 |

平台限制图数时，依次删图 9、图 1、图 3、图 8。

截图右侧中部的粉色圆形图标是抓取浏览器的翻译插件浮标，不是原站内容，不含账号信息。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `02-sol-price-availability.png` | 发布页可用性与价格段 | “not yet available in Chat”；正文因篇幅未写 |
| `03-sol-deepswe.png`、`04-sol-osworld.png`、`05-sol-science.png` | 发布页三张按任务成本图 | 官方的 1/5、1/7、$5.47 对 $23.80 均为厂商自测的每任务成本，与 token 单价是两种口径 |
| `06-devday-chinese.png`、`07-devday-english.png` | DevDay 回顾页中、英文 Sol 段 | 中文页曾写“四分之一”，现已改为“五分之一”（见 `../sources/a-devday-zh-comparison.md`） |
| `24-aa-token-efficiency.png`、`25-aa-sol-effort-levels.png` | AA 文章 token 效率图、模型页各 effort 指数 | 输出 token 多 10–30% 的图 |
| `10-anthropic-title.png`、`13-abliteration-full.png` | Anthropic 报告标题区、abliteration 段 | |
| `15-caisi-benchmark-definitions.png`、`16-caisi-results.png` | NIST CAISI 对 GLM-5.3 的评估 | CAISI：41 题、16 分制、三次取最佳，得分 61.1%；不可直接比 50/410 |
| `20-ai-gov-first-screen.png` | ai.gov 首屏原图 | 含官网人物照片，正式配图改用裁切的 27 |
| `21-nist-si-title.png` | NIST SP 330《The International System of Units (SI)》页 | 三、吠点③；常识性内容，不单独配图 |
| `26-whitehouse-ai-navigation.png` | 行政令页导航菜单 “Lead the World in AI” | 与图 11、12 同一类，篇幅所限未用 |

## 封面

已生成（第二版，2026-09-30，Codex CLI 内置图像生成，gpt-6-astra，参考图 `../../../../common_images/profile_picture.png`）：[00-cover.png](00-cover.png)（1086×1448，3:4，小红书、抖音）、[00-cover-wide.png](00-cover-wide.png)（1921×819，约 2.35:1，公众号）。上传顺序排在所有配图之前。

第一版（标题“五分之一价、100%绕过？拆开看”，价签 + 拆开的“100%”+ AI/SI 铭牌）已生成 2 组并选定，后因 Develata 把标题改为 GLM-5.3 逼近 Mythos-Preview 的角度而作废，不入库。

第二版候选与挑选：竖版、横版各 2 张候选，均选第 2 组（`cover-4`）。
- 竖版：第 1 组（`cover-3`）物件更大、零件更多，画面偏挤，天平两端等高；第 2 组留白更多，天平右端（Mythos-Preview）略低，符合 50 对 56 的方向，主标题更醒目。
- 横版：第 1 组主标题倾斜且偏小，“单价同 6-Sol”后多出一个撇号；第 2 组主标题平直更大，居中 819×819 裁切（x 551–1370）内主标题两行、看板娘与对话框完整。第 2 组铭牌上“SI”贴纸压住了“AI”的“I”一部分，仍可辨认。

已逐字核对（Claude）：主标题“国产模型逼近Mythos？”“五分之一价？”；天平挂牌“GLM-5.3”“Mythos-Preview”；标注“50/410 vs 56/410”；价签“1/5”，标注“单价同 6-Sol”；铭牌“AI”、贴纸“SI”；对话框“汪！拆开看”；圆牌“AI 吠点”。无 logo、无真人脸、无多余文字。标题“逼近”带问号，依据 ExploitBench 50/410 对 56/410（Mythos-Preview 关防护测试），正文吠点①写明比的是 5 个月前的版本、现已有更强的 Mythos-5。

本期画风：**工程蓝图 / 爆炸拆解图（exploded view）**——本期主题是“把数字拆开看口径”，用蓝底白线的技术图纸、零件分解与引线标注来呼应“拆开看”；与 0929 的 80 年代家电说明书、0928 的复古科幻杂志、0927 的波普丝网海报区分开（本期不用粉彩和网点，改用深蓝底、白色与亮黄细线）。

主体（做减法，三个 + 看板娘）：
- GLM-5.3 对 Mythos-Preview：两台拆开的发动机并排放在天平两端，几乎持平，分别挂牌“GLM-5.3”“Mythos-Preview”，引线标注“50/410 vs 56/410”。
- GPT-6.1-Sol：一张价签写“1/5”，引线标注“单价同 6-Sol”。
- 白宫：一块铭牌写“AI”，上面贴着一张半掀开的贴纸“SI”。
- 看板娘拿游标卡尺量天平，对话框“汪！拆开看”。
- 不画任何公司 logo、真人。

### 3:4 竖版提示词

```text
请画一张竖版 3:4 的社交媒体封面，风格为工程蓝图 / 爆炸拆解图（exploded view）：深蓝色图纸底，白色与亮黄色细线描边，零件分解、虚线引线与尺寸标注，像机械说明书里的拆解示意图，整体搞怪有冲击力，一眼能看出是手绘插画，不要照片级写实，不要 3D 渲染。

构图原则：做减法。全画面只有下面 4 个主体，主体之间留出大块深蓝空白，不画网格以外的背景细节（可以有很淡的蓝图网格）。

- 主标题（画面上部，最大最醒目）：分两行，粗黑体、白字加亮黄描边：“国产模型逼近Mythos？” / “五分之一价？”。
- 画面中部（最大的主体）：一架简洁的天平，两个托盘上各放一台被拆成零件的小发动机，两边几乎一样高（右边略低一点点）；左边发动机挂牌写“GLM-5.3”，右边挂牌写“Mythos-Preview”；天平横梁下方一条引线连到小标注框，标注框写“50/410 vs 56/410”。
- 画面左下：一张商店价签，价签上写“1/5”，虚线引线连到小标注框，标注框写“单价同 6-Sol”；价签下方一块金属铭牌写“AI”，上面贴着一张半掀开的贴纸写“SI”。
- 画面右下角（尺寸较大但不压住标题和天平）：我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，双手拿一把游标卡尺去量天平，张嘴大叫，头顶对话框写“汪！拆开看”。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印、品牌名或其他文字。
- 不画任何真实人物，不出现可辨认的人脸。
- 主标题和看板娘离四边留足边距，方便裁成 1:1。
```

### 2.35:1 横版提示词

```text
请画一张横版 2.35:1 的公众号头图，风格为工程蓝图 / 爆炸拆解图（exploded view）：深蓝色图纸底，白色与亮黄色细线描边，零件分解、虚线引线与尺寸标注，像机械说明书里的拆解示意图，整体搞怪有冲击力，一眼能看出是手绘插画，不要照片级写实，不要 3D 渲染。

构图原则：做减法。全画面只有下面 4 个主体，主体之间留出大块深蓝空白，不画网格以外的背景细节（可以有很淡的蓝图网格）。

- 中央正方形安全区（宽度约为整幅画宽度的 40%）：上部是大字主标题，分两行、居中对齐，粗黑体、白字加亮黄描边：“国产模型逼近Mythos？” / “五分之一价？”，最宽一行不超过安全区宽度的 80%；下部是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，双手拿一把游标卡尺，张嘴大叫，旁边对话框写“汪！拆开看”。标题和看板娘必须完整留在这个安全区内，裁掉左右两段后一个字都不能缺。
- 左段：一架简洁的天平，两个托盘上各放一台被拆成零件的小发动机，两边几乎一样高（右边略低一点点）；左边挂牌写“GLM-5.3”，右边挂牌写“Mythos-Preview”；引线连到小标注框，标注框写“50/410 vs 56/410”。
- 右段：一张商店价签写“1/5”，引线连到小标注框写“单价同 6-Sol”；下方一块金属铭牌写“AI”，上面贴着一张半掀开的贴纸写“SI”。
- 左段、右段的主体都不得伸进中央安全区，与安全区之间留出明显的深蓝空隙。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印、品牌名或其他文字。
- 不画任何真实人物，不出现可辨认的人脸。
```
