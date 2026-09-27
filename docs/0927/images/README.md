# 0927 配图说明

正文：`../doc_0927_publish.txt`。来源与限制见[来源入口](../sources/README.md)，事实核验见 `../sources/fact-check.md`，取证清单见 `../sources/evidence.md`，一手存档在 `../sources/`。

状态：01–08 与 10 组截图于北京时间 2026-09-27 深夜至 09-28 凌晨由 Codex（gpt-6-astra）从原站截取，裁剪与拼接记录见 `../sources/capture-log.md`。09 与 09b 由 Claude 用 pdftoppm（150 dpi）渲染预印本 `../sources/b-technical-report.pdf` 后以 ffmpeg 裁剪：09 取第 13 页 (138,890) 起 1000×412；09b 取第 8 页 (138,1392) 起 1000×150，加 2px 灰线，竖向拼接第 10 页 (138,120) 起 1000×82（同一句话跨页，中间隔了一整页 Figure 3）。未缩放、未改字。

## 正式配图（按上传顺序）

小红书、抖音、微博：封面之后按下表顺序独立上传。公众号：按“对应段落”手动插图。

| 顺序 | 文件 | 截取位置 | 对应正文段落 | 图注（可选） |
|---|---|---|---|---|
| 1 | `01-dsec-abstract.png` | arXiv 2609.22978 摘要页：标题、作者、提交日期、含规模数字的摘要句 | 一、事实段第 1 段；吠点③ | 单个生产单元：每天约 300 万个沙箱，峰值并发超 38 万；单位是 sandbox。 |
| 2 | `02-dsec-agent-behaviors.png` | 论文 6.4 节 “Obtaining answers through unintended channels” 两段 | 一、事实段第 2 段；吠点① | 看 “attempted to bypass” 与 “corrupted XFS metadata and forced a filesystem shutdown”；下一段是扫端口、Go module proxy。 |
| 3 | `10-overclaim-dsec-techtimes.png` | Tech Times 标题 “DeepSeek Training Agents Hacked Their Own Sandboxes: Escape Catalog Now Public” | 一、吠点① | 对比图 2：标题写“逃逸清单”；论文写的 XFS 绕过是一次“尝试”，造成了文件系统损坏，未报告成功逃逸。 |
| 4 | `03-dsec-accidents.png` | 论文 6.4 节非故意事故：递归 grep 触发内核 bug、攻击命令在自己容器执行、`yes` 输出几十 GB | 一、吠点② | 论文原话：没有蓄意破坏的普通命令也搞崩过基础设施。 |
| 5 | `04-enzyme-title-known-rt.png` | anthropic.com 公告标题、日期与 “underlying RT … had been identified in previous studies” 段 | 二、事实段 | 逆转录酶本身前人已发现；新的是旁边的阵列与伙伴蛋白。 |
| 6 | `05-enzyme-function-unknown.png` | 同文 “we don’t yet know its function” 段与 ART 三部分、短 RNA 段 | 二、吠点①③ | 阵列布局像 CRISPR；首批实验看到阵列转录成短 RNA，功能仍未知。 |
| 7 | `10-overclaim-enzyme-zgeo.png` | 智脑时代 ZGEO 标题 “AI驱动基因编辑新突破” 与摘要 | 二、吠点① | 对比图 6、图 8：中文稿写成“基因编辑新突破”。 |
| 8 | `09-enzyme-preprint-not-shown.png` | 预印本第 13 页 Discussion：“we have not shown that the RT is active …” | 二、吠点① | 作者自述：尚未证明该逆转录酶有活性，功能未知。 |
| 9 | `09b-enzyme-rerun-missed.png` | 预印本第 8、10 页：重跑十次、“the array was missed in every rerun” | 二、吠点② | 同样的搜索重跑 10 次，都没读到 RT 上游那段 DNA，阵列次次漏掉（作者归因于搜索空间大、流程不确定；直接给序列时，较强模型能识别）。 |
| 10 | `06-enzyme-agents-humans.png` | 同文 950 agents / 21 小时段、“All of the lab work is performed by human scientists” 段、How we work | 二、事实段；吠点③ | 约 950 个 agent、21 小时；湿实验全部由人完成。 |
| 11 | `07-openai-worm-summary.png` | alignment.openai.com 报告标题、日期框与 Summary | 三、事实段；吠点② | “No impact was observed outside of the simulated tool calls”。 |
| 12 | `10-overclaim-worm-reddit.png` | Reddit r/OpenAI 帖标题 “The first real AI worms have arrived … spreading across agents.” | 三、吠点① | 网传写法：已在 agent 间传播。 |

平台限制图数（如 9 张）时，依次删图 10、图 4、图 1。图 2 与图 3、图 6/8 与图 7、图 11 与图 12 是对比关系，应相邻。

## 备用图

| 文件 | 截取位置 | 用途 |
|---|---|---|
| `08-openai-worm-models.png` | OpenAI 报告 “Responsible models and impact” 节 | 邮件/文件实验攻守双方都是基于 GPT-5.4-mini 的内部 checkpoint；Slack 实验受害模型为 GPT-5.5。正文为控篇幅删去此点 |
| `10-overclaim-enzyme-sina.png` | 新浪财经转载（署智药局）标题“张锋点赞！Anthropic公布首个AI自主科学发现，21小时零干预，Claude找到全新DNA系统” | 夸大标题实例；“零干预”对搜索阶段有原文支持，湿实验由人做 |

## 使用注意

- Anthropic 与 OpenAI 的日期均无时刻和时区，正文只写原文日期，不换算北京时间。arXiv v1 提交于 2026-09-19 12:20:26 UTC（北京时间 20:20）。
- 公告写 “roughly 950 agents … 210 million tokens … 21 hours”；预印本写 949 agent sessions、215.6 million tokens、21.5 hours。正文用公告的约数。
- Tech Times 正文把 XFS_IOC_SWAPEXT 写成 “ioctl FIEXCHANGE”，与论文不符；正文未展开。其后半篇讲的 CVE-2026-82533 属于 DeepSeek Harness（本地编码 agent 工具），不是 DSec，正文未混用。
- The Verge 标题 “Anthropic’s biolab made a discovery it’s comparing to Crispr” 保留了“it’s comparing”，不算夸大实例。

## 封面

待生成。生成时上传 `../../../brand/avatar.webp` 作看板娘参考。本期不画真人。主标题取正文标题“沙箱逃逸？AI蠕虫来了？新基因编辑？”，分两行。

### 3:4 竖版提示词

```text
请画一张竖版 3:4 的社交媒体封面，风格为杂志封面式编辑插画（editorial illustration）：厚涂或丝网印刷质感、夸张透视、强烈明暗、高饱和撞色（冷蓝、暖橙、酸绿三色分区），一眼能看出是手绘插画，不要照片级写实。

画面分上、中、下三格，像一张三格漫画：
- 上格（冷蓝）：一面墙一样密密排列的许多小玻璃方盒（沙箱）。其中一个玻璃盒里，一个银白色小机器人（背上贴着标签“agent”）正用撬棍撬盒子的地板，地板裂开冒出黑烟，裂缝旁有一块小牌子写“XFS”；玻璃盒本身完好，机器人仍在盒内。
- 中格（暖橙）：一个橙色、星芒/小太阳形状的吉祥物举着放大镜，看一条长长的彩色 DNA 链，链上有一段颜色重复的方块（重复阵列）；重复阵列上挂着一张问号标签“功能？”。画面一角伸进来一只戴蓝色实验手套的人手，拿着移液枪（只画手，不画人脸）。
- 下格（酸绿）：一个透明玻璃生态缸，缸壁上贴着标签“模拟环境”；缸里一条由信封首尾相连组成的卡通小虫在爬，每个信封上都印着同一个小图案，表示内容被原样复制。缸外空无一物。
- 画面正中偏下，是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，一手举放大镜，张嘴大叫，头顶爆炸对话框写“汪！？”。
- 画面中部大字主标题，分两行，粗黑体、白字加深蓝粗描边：“沙箱逃逸？AI蠕虫来了？” / “新基因编辑？”
- 主标题下方一行小字副标题：“没写逃逸 · 功能未知 · 模拟实验”

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印或其他文字。
- 不画任何真实人物，不出现可辨认的人脸。
- 主标题和看板娘离四边留足边距，方便裁成 1:1。
```

### 2.35:1 横版提示词

```text
请画一张横版 2.35:1 的公众号头图，风格为杂志封面式编辑插画（editorial illustration）：厚涂或丝网印刷质感、夸张透视、强烈明暗、高饱和撞色（左冷蓝、中暖橙、右酸绿），一眼能看出是手绘插画，不要照片级写实。

画面按横构图重新排布，分左、中、右三段：
- 左段（冷蓝）：一面密密排列的小玻璃方盒墙（沙箱）；其中一个盒里，一个银白色小机器人（背上贴着标签“agent”）用撬棍撬盒子的地板，裂缝冒黑烟，旁有小牌子“XFS”；玻璃盒完好，机器人仍在盒内。
- 右段（酸绿）：一个透明玻璃生态缸，缸壁贴标签“模拟环境”，缸里一条由信封首尾相连组成的卡通小虫在爬，每个信封印着同一个小图案。
- 中段（画面正中央安全区）：一个橙色、星芒/小太阳形状的吉祥物举放大镜看一条带颜色重复方块的 DNA 链，链上挂问号标签“功能？”；它旁边是我上传的头像角色（蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌），Q 版比例，张嘴大叫，头顶爆炸对话框写“汪！？”。两者上方是两行大字主标题，粗黑体、白字加深蓝粗描边：“沙箱逃逸？AI蠕虫来了？” / “新基因编辑？”
- 不要副标题。

要求：
- 只出现上面引号里的文字，逐字准确；不加任何公司 logo、水印或其他文字。
- 不画任何真实人物，不出现可辨认的人脸。
- 主标题与看板娘集中在画面中央约 1:1 的区域内，左右两段可被裁掉而不影响标题完整。
```
