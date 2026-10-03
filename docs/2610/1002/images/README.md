# 1002 配图说明

正文：`../doc_1002_publish.txt`。来源见[来源入口](../sources/README.md)，事实核验见 `../sources/fact-check.md`，取证清单见 `../sources/evidence.md`，一手存档在 `../sources/`。卡片文字与批注内容写在 `cards.toml`，由 `barking card` 渲染。

状态：01–29 于北京时间 2026-10-02 21:2x 至 22:1x 由 Codex（gpt-6-astra）从原站截取（`../sources/capture-log.md`），2 倍像素比。30–35 为批注截图；其 `-raw` 底图中，30、31、32、35 为 Claude 从 01、02、09、27 裁出的区域，33、34 为 Claude 用无头 Chrome（临时用户目录、2 倍像素比、视口 700 CSS px）于北京时间 10/2 约 23:4x 从原站重新截取后裁出（为让英文在 1080 宽卡片里更大），原图像素均未改。`00-tldr.png` 为省流卡。

## 正式配图（按上传顺序）

小红书、抖音：封面之后按下表顺序独立上传。公众号：正文开头就是省流，默认不插省流卡，其余按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `00-tldr.png` | 省流卡（本号整理，非原文截图） | 省流、各条吠点 | 每条一句事实加最能纠正说法的吠点；文字逐字取自正文。 |
| 2 | `30-suncatcher-google.png` | Google 博文（10/1）：取得联系句与 “Over the coming weeks” 句（批注版） | 一、事实段、吠点① | ① “confirmed contact … operating as expected”；② “Over the coming weeks, we'll gather in-orbit data …”，即谷歌称未来数周将收集在轨数据。 |
| 3 | `31-suncatcher-ground-test.png` | Google 9/24 说明页：Hardware survival 一节（批注版） | 一、吠点① | ① 质子束设施是地面测试；② “Initial results … total ionizing dose greater than … a five-year space mission”，限定是总电离剂量。 |
| 4 | `32-openai-100-orgs.png` | OpenAI 时间线页 9/30 条目：规模与通知段（批注版） | 二、事实段、吠点① | ① 约 7,000 块 GPU、每天超 50 万美元；② “As of September 26 … over 100 organizations”；③ “Notification does not mean …”。 |
| 5 | `33-california-subpoena.png` | 加州司法部新闻稿（10 月 1 日）：开篇两句（批注版） | 二、事实段、吠点② | ① “served an investigative subpoena … ongoing investigation”；② 传票属于更广泛调查的一部分。 |
| 6 | `34-clef-leaderboard.png` | Cloudflare 演示榜：Decision Index 对延迟图（批注版） | 三、吠点① | 自报说明；三个点：Clef 61.2（#1）、Jev 57.9、Clef-flash 57.1，横轴为中位延迟。 |
| 7 | `35-clef-self-reported.png` | Cloudflare 演示榜 Methodology：Self-reported entries（批注版） | 三、吠点② | “have not been reproduced by the upstream maintainers”；延迟在作者自己的部署栈上测，“not directly comparable”。 |

平台限制图数时，依次删图 7、5、3。

截图右侧中部的粉色圆形图标是抓取浏览器的翻译插件浮标，不是原站内容，不含账号信息；正式图的底图已避开它。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `30-suncatcher-google-raw.png`、`31-suncatcher-ground-test-raw.png`、`32-openai-100-orgs-raw.png`、`33-california-subpoena-raw.png`、`34-clef-leaderboard-raw.png`、`35-clef-self-reported-raw.png` | 6 张批注图的原始底图 | 批注底图（像素未改） |
| `01-suncatcher-prototype.png` | Google 博文整页（10/1） | 30 的原截图；页面日期 Oct 01, 2026 |
| `02-suncatcher-hardware.png`、`03-suncatcher-cooling.png` | Google 9/24 说明页 Hardware survival、Cooling in space | 辐射为地面质子束测试；散热“So far … thermal vacuum chamber … We'll see how … works in space” |
| `04-suncatcher-scale.png`、`05-suncatcher-power.png` | Google 9/24 说明页 | 每星未来数十块 TPU、2027 年两星激光链路；LEO 最高 8 倍太阳能发电 |
| `06-suncatcher-npr.png` | NPR 正文 | 4 颗 TPU、冰箱大小、Gemma、每次 15 分钟（因散热限制）；记者叙述，未注明消息来源 |
| `07-suncatcher-joule.png` | Joule 论文摘要 | 81 星示例编队；总电离剂量；发射价可能 ≤$200/kg（mid-2030s，LEO） |
| `08-openai-review-start.png`、`14-openai-review-human.png` | OpenAI 时间线页 9/30 条目 | 50 PB、7,000 GPU；四步筛查与人工复核（正文因篇幅未写） |
| `09-openai-review-notification.png` | 同上 | 32 的原截图（含 GPU 与通知段） |
| `10-openai-dozens-servers.png` | OpenAI 8/26 报告 | “dozens of Hugging Face servers”，指服务器数，不是被通知机构数 |
| `11-california-subpoena.png` | 加州司法部新闻稿 | 33 的原截图（含 Bonta 引语） |
| `12-aisi-resume-controls.png`、`13-aisi-monitoring.png` | 英国 AISI 博客（Oct 1, 2026） | 恢复大部分评测、暂停联网；CoT 监控“fragile”。正文因篇幅未写 |
| `15-guardian-ftc-context.png` | Guardian（Reuters 稿） | “first official US enforcement action”紧跟 FTC 调查 |
| `16-clef-title.png`、`17-clef-release.png` | Cloudflare 博文标题区、发布段 | “fully open-sourcing … Apache 2.0”；“currently the leader … Jev Decision Index” |
| `18-clef-benchmark-table.png` | Cloudflare 博文对比表（10 行 × 6 列） | 全表截全；When2Call、BRIGHT 上 Jev 最高 |
| `19-clef-latency-table.png` | Cloudflare 博文延迟表 | 中位 209.3 / 38.8 / 524.1 ms，p95 238.6 / 122.4 / 536.0 ms |
| `20-jev-space.png`、`21-jev-index.png` | Hugging Face Space（multimodalart） | Jev Decision Index 属第三方社区；“Not affiliated with TypeSafe AI” |
| `22-clef-license.png`、`23-clef-flash-card.png`、`28-clef-model-card.png` | Hugging Face 模型卡 | Apache-2.0；Qwen 基座；权重与推理代码，未见训练数据 |
| `24-clef-pricing.png`、`25-clef-flash-pricing.png`、`29-jev-pricing.png` | Workers AI 定价、TypeSafe Jev 页 | 输入价每百万 token：0.24 / 0.09 / 0.042 美元；正文因篇幅未写 |
| `26-clef-demo.png`、`27-clef-methodology.png` | Cloudflare 演示榜整页、Methodology 页 | 34、35 的原截图 |

另有 `../sources/c-failed-jev-ranking.png`：上游完整榜单的失败拼接截图，仅作诊断，不是配图。

## 封面

已生成（Codex CLI 内置图像生成，gpt-6-astra，参考图 `../../../../common_images/profile_picture.png`）：[00-cover.png](00-cover.png)（1086×1448，3:4，小红书、抖音）、[00-cover-wide.png](00-cover-wide.png)（1921×819，约 2.35:1，公众号）。上传顺序排在所有配图之前。发布时按平台要求声明 AI 生成。封面图不入库，发布后存 OpenList。

本期画风：**复古航空邮件（邮票 + 邮戳 + 红蓝斜条纹信封边）**——三条分别对应“航空信”（上太空）、“通知信”（OpenAI 通报）和“特快”（Clef 的快）；奶油白底、黑色钢笔墨线、红蓝白三色，与 1001 报刊社论漫画、0930 工程蓝图、0929 80 年代家电说明书、0928 复古科幻杂志、0927 波普丝网海报区分开。

主体（做减法，三枚邮票 + 看板娘）：
- 谷歌：一颗方盒卫星，两翼太阳能板，机身芯片上写“TPU”；邮票内不写别的字。
- OpenAI：红色邮筒塞满信封，小牌子写“OpenAI称：通知不等于被攻破”（写明是 OpenAI 的说法，对应正文吠点①）。
- Cloudflare：秒表表盘写“39毫秒”，小牌子分两行写“Clef-flash”“自报中位延迟”（把 39 毫秒只挂在 Clef-flash 上，并写明是自报的中位延迟，对应正文吠点①②与反向核验意见）。
- 看板娘：举放大镜，对话框“汪！看限定词”。不画任何公司 logo、真人。

### 3:4 竖版提示词

```text
请画一张竖版 3:4 的社交媒体封面，风格为复古航空邮件：奶油白信封纸底，画面四周一圈红蓝相间的斜条纹航空信封边框，邮票为锯齿边，黑色钢笔墨线加扁平色块，只用红、蓝、奶油白三种颜色，整体搞怪有冲击力，一眼能看出是手绘插画，不要照片级写实，不要 3D 渲染。

构图原则：做减法。画面分上下两部分：上部约占四分之一高度，是主标题；下部是 2×2 的四格：左上、右上、左下各是一枚锯齿边邮票，右下是看板娘。每个格子只有一个主体，主体完整留在邮票边框内、不伸出边框，周围留出大块空白，不画多余背景细节，整体不要拥挤。

- 主标题（画面上部，最大最醒目）：分两行，粗黑体，黑字配红色下划线：“谷歌TPU上太空？” / “OpenAI通报超百家”，最宽一行约占画面宽度的 85%。
- 左上邮票：一颗小方盒卫星，两侧展开太阳能板，机身贴一块小芯片，芯片上写“TPU”。邮票内除“TPU”外不写任何字。
- 右上邮票：一个红色邮筒，塞满白色信封，几封信从口里飘出来；邮票下方挂一块小牌子，写“OpenAI称：通知不等于被攻破”（分两行写，字要清楚）。邮筒本身不写字。
- 左下邮票：一块大秒表，表盘中央写“39毫秒”，秒表下方挂一个小标签，分两行写“Clef-flash”和“自报中位延迟”（字要清楚）。
- 右下：我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，一手举放大镜，张嘴大叫，旁边对话框写“汪！看限定词”。该格没有边框，不画邮票。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印、品牌名或其他文字。
- 不画任何真实人物，不出现可辨认的人脸（看板娘角色除外）。
```

### 2.35:1 横版提示词

```text
请画一张横版 2.35:1 的公众号头图，风格为复古航空邮件：奶油白信封纸底，画面四周一圈红蓝相间的斜条纹航空信封边框，邮票为锯齿边，黑色钢笔墨线加扁平色块，只用红、蓝、奶油白三种颜色，整体搞怪有冲击力，一眼能看出是手绘插画，不要照片级写实，不要 3D 渲染。

构图原则：做减法。全画面分左、中、右三段，每枚邮票只有一个主体，主体完整留在邮票边框内、不伸出边框，周围留出大块空白，不画多余背景细节。

- 中央正方形安全区（宽度约为整幅画宽度的 40%，无边框）：上部是大标题，分两行、居中对齐，粗黑体，黑字配红色下划线：“谷歌TPU上太空？” / “OpenAI通报超百家”，最宽一行不超过安全区宽度的 80%；下部是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，一手举放大镜，张嘴大叫，旁边对话框写“汪！看限定词”。标题和看板娘必须完整留在这个安全区内，裁掉左右两段后一个字都不能缺。
- 左段：一枚锯齿边邮票，里面是一颗小方盒卫星，两侧展开太阳能板，机身贴一块小芯片，芯片上写“TPU”。邮票内除“TPU”外不写任何字。
- 右段上：一枚锯齿边邮票，里面是一个红色邮筒，塞满白色信封，几封信从口里飘出来；邮票下方挂一块小牌子，写“OpenAI称：通知不等于被攻破”（分两行写，字要清楚）。邮筒本身不写字。
- 右段下：一枚锯齿边邮票，里面是一块大秒表，表盘中央写“39毫秒”，秒表下方挂一个小标签，分两行写“Clef-flash”和“自报中位延迟”（字要清楚）。
- 左段、右段的主体都不得伸进中央安全区，与安全区之间留出明显的空隙。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印、品牌名或其他文字。
- 不画任何真实人物，不出现可辨认的人脸（看板娘角色除外）。
```

候选与挑选：竖版、横版各 4 张候选（4 组），选第 4 组（`cover-4`），候选图不入库。未超规范的“每种尺寸最多 5 张”：
- 第 1、2 组：标签只写“Clef-flash”。反向核验指出“39毫秒”缺统计口径（SHOULD_FIX），提示词改为加“自报中位延迟”；另外第 2 组横版卫星与邮票边框相交、第 1 组横版标题偏小，不入选。画面文字当时逐字无误。
- 第 3 组：标签与边框问题已改，竖版可用；横版放大镜镜片里出现类似“7B”的无关字形，淘汰。
- 第 4 组：竖版、横版均无错字，主体都留在邮票内；横版居中 819×819 裁切（x 551–1370）内主标题两行、看板娘与对话框完整。选中。

已逐字核对（Claude，竖横版均放大 3–4 倍看小字）：主标题“谷歌TPU上太空？”“OpenAI通报超百家”；卫星芯片“TPU”；邮筒牌子“OpenAI称：通知不等于被攻破”；秒表“39毫秒”，标签“Clef-flash”“自报中位延迟”；对话框“汪！看限定词”。画面里没有其他文字、logo 或水印。
