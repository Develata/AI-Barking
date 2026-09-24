# 0924 配图说明

正文：`../doc_0924_publish.txt`（2026-09-23 按 `EDITORIAL.md` 重写）。原稿 `../doc_0924.md` 保留不动。事实核验见 `../sources/fact-check.md`，取证明细见 `../sources/evidence.md`。

全部图片为原站或官方 PDF 的实时截图，未重绘、未改数字。截图日期 2026-09-23。

## 正式配图（按上传顺序）

小红书、抖音、微博：按下表顺序独立上传。公众号：同一正文，按“对应段落”手动插图。

| 顺序 | 文件 | 对应正文段落 | 图注（可选，一句话） |
|---|---|---|---|
| 1 | [01-claude-price.png](01-claude-price.png) | “先看价格”Opus 5.5 段 | 输入、输出单价各降 20%，缓存读取降 60%。 |
| 2 | [02-openai-price.png](02-openai-price.png) | Sol / Luna 价格段 | 标准处理、短上下文价格，美元／百万 Token。 |
| 3 | [04-aa.png](04-aa.png) | “AA综合指数确实是它领先” | 看 Opus 5.5 与 Astra 两行；注意 max / fallback 标签。 |
| 5 | [06-anthropic-benchmark-table.png](06-anthropic-benchmark-table.png) | “但Terminal-Bench 4.0一个榜两种结果” | 看 Terminal-Bench 4.0 一行：66.4% 对 57.9%。 |
| 6 | [10-aa-terminal-bench.png](10-aa-terminal-bench.png) | 同上段，紧接图 5 | 同一个 Terminal-Bench 4.0，AA 测出 59.6% 打平。 |
| 7 | [32-frontiermath.png](32-frontiermath.png) | “数学反过来” | FrontierMath 最难档：Astra 领先，Claude 最高为 Fable 5。已裁掉 Cookie 弹窗与浏览器插件图标，原图在 `../sources/usage/32-frontiermath-original.png`。 |

图 5、图 6 必须相邻，读者才能直接对比同一个榜的两种结果。LiveBench 段已从正文删除，原图 4 移入备用。平台限制图数时，优先删图 3。

## 备用图

| 文件 | 用途 |
|---|---|
| [03a-claude-reset.png](03a-claude-reset.png) | Claude 额度重置原文；到期日 10 月 22 日不在此图中，来自 @ClaudeDevs 原帖。 |
| [03b-openai-reset.png](03b-openai-reset.png) | OpenAI 官方社区第 4 楼 Banked Reset 公告。 |
| [06-anthropic-benchmark.png](06-anthropic-benchmark.png) | 图 5 的完整脚注版，含测试配置与回退说明。 |
| [05-livebench.png](05-livebench.png) | LiveBench 前三名（正文已删该段）。 |
| [30-arena-text.png](30-arena-text.png) | Arena Text 榜（页面 9 月 13 日，早于新模型发布，勿用）。 |
| [30-arena-webdev.png](30-arena-webdev.png) | Arena WebDev：Opus 5.5 与 Astra 排名区间均 1–2（1000 字版正文已删该句）。 |
| [31-openrouter.png](31-openrouter.png) | OpenRouter 使用量（窗口口径，非自发布累计）。 |
| [11-opus-security-test-setup.png](11-opus-security-test-setup.png) | 系统卡 §6.4.9 测试设定（p.119）。本期未用，留作候选选题。 |
| [12-opus-security-test-results.png](12-opus-security-test-results.png) | 系统卡 §6.4.9 结果与 Anthropic 解释（p.120）。同上。 |

## 使用注意

- 图 2 只含标准处理、短上下文价格，不代表长上下文、Batch、Flex 等全部计费情形。
- 图 3、图 4、图 6 是截图时点数据，榜单之后可能变化。AA 指数为 v4.3；LiveBench 页面版本 2026-06-25。
- 图 5 中 Astra 与 5.6 Sol 的分数为 Anthropic 转引 OpenAI 数据，不是 Anthropic 同场横测。

## 封面

已生成：[00-cover.webp](00-cover.webp)（3:4，2026-09-24）。已逐字核对：记分牌“官方表 66.4 : 57.9”“AA 实测 59.6 : 59.6”、标题、副标题、“汪！？”均无误；两位 CEO 无台词。上传顺序排在所有配图之前。公众号横版 `00-cover-wide.*` 待生成。

### 生成记录

文件名：`00-cover.*`（3:4 主图）、`00-cover-wide.*`（公众号 2.35:1）。生成时上传 `../../brand/avatar.webp` 作看板娘参考。风格：编辑插画（非写实），两位 CEO 可辨认，但不配任何台词。提示词：

```text
请画一张竖版 3:4 的社交媒体封面，风格为杂志封面式编辑插画（editorial illustration）：厚涂或丝网印刷质感、夸张透视、强烈明暗、高饱和撞色（左半橙色、右半深蓝），一眼能看出是手绘插画，不要照片级写实。

画面：
- 中间是拳击擂台，镜头略仰视，速度线和飞溅的碎纸营造冲击感。
- 左角：一个橙色、星芒/小太阳形状的吉祥物拳手，胸前写“Opus 5.5”。它身后的教练席上站着 Anthropic CEO Dario Amodei 的插画形象（卷发、眼镜），双臂抱胸，神情专注。
- 右角：一个银白色流线型机器人拳手，胸前写“GPT-6 Astra”。它身后的教练席上站着 OpenAI CEO Sam Altman 的插画形象，手扶围绳，神情专注。
- 擂台上方挂两块记分牌：左牌写“官方表 66.4 : 57.9”，右牌写“AA 实测 59.6 : 59.6”。
- 擂台中央是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，当裁判：一手举放大镜对着记分牌，张嘴大叫，头顶爆炸对话框写“汪！？”。
- 画面中下部大字主标题，分两行，粗黑体、白字加蓝色粗描边：“Opus 5.5” / “完爆 GPT-6 Astra？”
- 主标题下方一行小字副标题：“同一个榜，换个测法，领先就没了”

要求：
- 只出现上面引号里的文字，逐字准确；两位 CEO 不配任何对话框、台词或口号；不加公司 logo、水印或其他文字。
- 两位 CEO 是配角，体量小于中央裁判和两名拳手。
- 主标题和裁判离四边留足边距，方便裁成 1:1。
```

公众号横版：同一对话中追加“保持角色、风格与人物不变，改为横版 2.35:1：左侧擂台与两块记分牌，右侧裁判吠叫；标题改为一行‘Opus 5.5 完爆 GPT-6 Astra？’，去掉副标题。”

若模型拒绝绘制真人，退回为两位 CEO 的背影剪影（仍按左橙右蓝站位），其余不变。生成后逐字核对记分牌数字与标题。
