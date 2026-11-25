# 1124 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，时间为北京时间（本机时钟，UTC+8）。批量抓取逐条结果在 `a-fetch-log.jsonl`，清单 `a-fetch-list-1.tsv … -8.tsv`，截图动作在 `a-shots-log.jsonl`，热度 `a-heat.json`。均走 `barking fetch`（每条限时 60 s）；个别一次性请求用 `curl --max-time 60`；渲染页用 opencli（session `a-shot`）。不用 firecrawl，未读取浏览器 Cookie，未登录、未发帖、未点赞。

## 一、批量存档（barking fetch）

| 批 | 北京时刻 | 内容 | 结果 |
|---|---|---|---|
| 1 | 02:01:10 | AA 图生/文生榜页、sand.ai 首页与 /magi、GitHub SandAI-org 与 MAGI-1、HF sand-ai；另试 AA 的 -silent 路径与 /video/leaderboard、sand.ai/blog | AA 两页 ok；**`…/image-to-video-silent`、`…/text-to-video-silent`、`/video/leaderboard`、`sand.ai/blog` 404（路径不存在，failed）**；其余 ok |
| 2 | 02:05:08 | AA 带 `?audio-output=false` 的两个 URL、sand.ai/blog/magi-2/、/research、platform.sand.ai、sand.ai/magi/pricing、vidmuse、GitHub/HF 组织页、HF MAGI-2 | 博客与 research ok；**AA 的 `?audio-output=false` URL 服务端直出的仍是带音频视图（与默认页字节数相同 1978021/1930043），所以 `a-aa-i2v-silent-url.*`、`a-aa-t2v-silent-url.*` 是重复文件，无音频视图改用 opencli 渲染后 DOM 存档**；platform.sand.ai blocked（JS 壳 51 字符）；sand.ai/magi/pricing 500 failed；vidmuse 403 Cloudflare 质询（未绕过）；`huggingface.co/sand-ai/MAGI-2` 401（该仓库不存在） |
| 3 | 02:05:47 | MAGI-2 Preview：官方博客、GitHub 仓库与 README、HF 模型卡 | 4 条全部 ok |
| 4 | 02:06:21 | 中文报道 11 篇（新浪、投资界、网易、21 财经、爱范儿、虎嗅、搜狐×3、同花顺转载上证报稿、腾讯新闻）、英文 KuCoin 快讯、lookonchain、seeddance.io 指南、rikoko、beatapi | 16 条全部 ok；抽查 .txt 为正文（虎嗅快讯只有短文） |
| 5 | 02:09:15 | Higgsfield 发布稿、i10x、nglobenews、launchvideo、火山方舟模型价格页（docs.volcengine.com 与 ark.volcengine.com）、aitop100、新浪微博稿 | 8 条全部 ok；价格页 .txt 含 Seedance 2.5 价格表 |
| 6 | 02:27:10 | seeddance.io 模型页、apiyi、dev.to、imagera、腾讯新闻 | 5 条 ok |
| 7 | 02:28 | 36 氪英文版（智东西稿） | ok |
| 8 | 02:30 | 36 氪中文版 | ok |
| 9 | 02:37 | 17173、界面新闻（摩尔线程）、北京商报新浪稿、AGI Hunt、techritual、runtimewire、21 财经北京商报稿 | 17173、界面、新浪北京商报稿、runtimewire ok；**AGI Hunt、techritual 403 Cloudflare 质询（blocked，未绕过）**；21 财经北京商报稿两次超时 failed。AGI Hunt 改用 opencli 浏览器打开（页面直接可读，无质询），存 `a-agihunt-info-p-*.txt` |
| 10 | 02:43 | seeddance.io review、vs 两篇 | 2 条 ok |

## 二、opencli（渲染后页面）

- AA 四个子榜：`a-aa-capture.sh <页> <audio|silent>` 打开 `https://artificialanalysis.ai/video/leaderboard/<image-to-video|text-to-video>`；无音频视图用 JS 点击 “No Audio” 切换（原生点击被 “Native input requires a visible browser page” 拒绝，改用 DOM 事件，仅切视图）。输出 `a-aa-<页>-<audio|silent>.table.txt`（表格文本）、`.dom.html`（渲染后 DOM，约 2 MB，只存本地不入库）、`.captured-at.txt`。读取时刻：I2V 带音频 02:03:44、I2V 无音频 02:04:00、T2V 带音频 02:04:29、T2V 无音频 02:04:43。Category 菜单文本存 `a-aa-category-menu.txt`。
- platform.sand.ai：首页要登录（`/app/login/`，**登录态页面未存档**）；公开文档页 `/docs/model-pricing/`、`/docs/credits/`、`/docs/get-started/` 以 opencli 取 `a-sand-platform-*.txt`、`.dom.html`（02:2x）。
- 截图（12 个编号，另有 01–04 的 `-full` 整表版共 16 张图，2 倍像素比的要求有例外，见下）：AA 四榜用缩放 1.3（宽 1400，所以实际约 1.3 倍像素，而非 2 倍；若用 2 倍则 700 CSS px 布局会丢掉 Samples/Released/Pricing 列，故取 1.3）；sand.ai 博客缩放 1.5/2；platform 价页缩放 2；火山方舟、21 财经、KuCoin、Higgsfield 为缩放 1–1.5。全部由 `barking shot cropbox|stack` 从中间图裁出，记录在 `a-shots-log.jsonl`。
- X 站内搜索（已登录会话，只读；02:19–02:20）：4 次搜索 AA 官方账号关于 MAGI-2 的帖子，均无结果；搜索结果与登录会话信息**未存档**。
- 补充 opencli（02:37–02:39）：AA 页悬停提示（Provisional result、Range ⓘ）与“Hide research previews”取消勾选测试（DOM 事件，仅切视图）；AGI Hunt（浏览器直开）、vidmuse.ai/en/magi-2（首页可读，无价格，`a-vidmuse-ai-en-magi-2.txt`）。AA 四榜整表整页图（`-full`）02:4x 补拍，缩放 1.3，宽 1400。
- Reddit：`opencli reddit search "MAGI-2"` → SECURITY_BLOCK（403）；未重试。

- 截图备注：05（sand.ai 博客头图，黑底）、11（KuCoin，深色页，点了 “Reject All” 拒绝 Cookie；页内左侧悬浮的点赞/分享按钮遮住第 20 行“through”半个词）、12（Higgsfield，深色页，其下方榜单柱状图当时未渲染出来，只截了文字段与榜单标题行）、01–04（页首 AA 导航条已裁掉，页面显示未登录，右上浏览器翻译插件浮标已用 CSS 隐藏）。逐张已打开目视确认文字清楚、范围正确。

## 三、个别请求

- `curl`：HF API（`/api/models?author=sand-ai`、`/api/models/sand-ai/MAGI-2-preview`、`/api/models?search=MAGI-2`，02:05 与 02:27 两次）、GitHub API（`/orgs/SandAI-org/repos`、`/repos/SandAI-org/MAGI-2-preview`、仓库搜索）→ `a-hf-sand-ai-models-api.json`、`a-hf-magi2-preview-api.json`、`a-gh-sandai-repos-api.json`、`a-gh-magi2-preview-api.json`；AA 页 FAQ 句复读（02:27:53）；`api.fxtwitter.com/status/2125290721003680155` → `a-x-magiai-hq-2125290721003680155.json`（官方发布帖；公开 JSON，不含抓取者信息）；`api.fxtwitter.com/<账号>` 取了 @SandAI_HQ 与 @magiai_hq 两个账号的公开简介（未存档）；HN Algolia 手查 4 个关键词。
- `barking heat`（02:26:22）：HN 0 命中；Reddit 重定向登录页，失败（记入 `a-heat.json` 的 errors）。
- WebSearch（8 次）：找中文媒体、Seedance 价格页、AA/官方 X 帖、英文引文；结果只作线索，引用的页面都已回到原站存档（上证报稿经同花顺转载页，来源标 cnstock.com，**cnstock 原站未直接取**）。

## 四、失败与未做

| 对象 | 原因 | 处理 |
|---|---|---|
| AA 官方社媒宣布 MAGI-2 名次的帖子 | X 搜索无结果，无帖子 ID | 记“未找到一手来源” |
| Sand.ai 官方中文发布稿（公众号/官网中文页） | 搜索未返回；sand.ai 无中文页 | 记未找到；中文通稿措辞是否带子榜限定因此无法判定 |
| Sand.ai 中国站价页 | `/docs/credits/` 提到 China site，但域名未知，猜测域名无法解析 | 记未取得 |
| 06 图 | 底边切在图表柱状图中间、正文上有页面自带的滚动进度竖线叠影（页面自身元素） | 已另补 `images/06-sand-blog-aa-avg-caption.png`（柱状图 + 图注 “AA Video Arena Elo shows the 4-task average … (I2V, I2V-Silent, T2V-Silent, and T2V; Elo bars plotted from a 950 baseline).”，同样有竖线叠影） |
| platform.sand.ai 控制台、vidmuse.ai | 要登录 / Cloudflare 质询 | 不存档、不绕过 |
| sand.ai/magi/pricing（MAGI-1 价页） | 500 | 本组不需要（MAGI-2 价在 platform 文档页） |
| BlockBeats 原快讯（含“便宜约 87%”） | 未找到；KuCoin 转载稿无 87% | 记未找到 |
| seeddance.io “前三名 9 分以内”原句 | 已存档两页均无 | 记未找到 |
| 上证报 cnstock.com 原页、新京报贝壳财经原页、凤凰网 | 用同花顺/21 财经转载页代替；凤凰网只见转载线索 | 注明转载 |
| Reddit r/StableDiffusion 等 | 登录页重定向、403 | 未取到 |
| AA 无音频视图的静态 HTML | 服务端不输出 | 存渲染后 DOM |
| AA 分项 Category 数据 | 本组只读 Overall | 未取 |

## 五、待删除文件（精确路径，均在 `docs/2611/1124/`）

失败抓取留下的错误页或空壳（均无用）：
`sources/a-aa-i2v-silent.headers`、`sources/a-aa-i2v-silent.html`、`sources/a-aa-t2v-silent.headers`、`sources/a-aa-t2v-silent.html`、`sources/a-aa-video.headers`、`sources/a-aa-video.html`、`sources/a-hf-magi2.headers`、`sources/a-hf-magi2.html`、`sources/a-sand-blog.headers`、`sources/a-sand-blog.html`、`sources/a-sand-platform.headers`、`sources/a-sand-platform.html`、`sources/a-sand-pricing.headers`、`sources/a-sand-pricing.html`、`sources/a-vidmuse-magi2.headers`、`sources/a-vidmuse-magi2.html`。
重复且易误导（服务端并未返回无音频视图）：`sources/a-aa-i2v-silent-url.headers`、`.html`、`.txt`，`sources/a-aa-t2v-silent-url.headers`、`.html`、`.txt`。
截图：`images/10-zh-xjb-beike-top1.png`（顶部切到面包屑半行，已由 `10-zh-xjb-beike-top1-v2.png` 取代）。
scratchpad 中间截图在仓库外（会话 scratchpad 下 `raw-*.png`，含四个 AA 榜 1500 px 高的整页中间图），不入库。

## 六、体量与入库注意

- 单个超过 1 MB 的文件：`a-aa-*.html` 与 `a-aa-*.dom.html`（约 1.9–2.0 MB）、`a-volc-ark-pricing.html`（1.7 MB）等 HTML 只存本地不入库；无超过 1 MB 的 txt/json。
- 泄露自查：对 `docs/2611/1124/` 用本机用户名与 X 账号 handle 做 grep（不文字写入此处），只命中 `sources/capture-log-B.md`（B 组文件，请 Claude 核对）；A 组文件与截图无抓取者头像、昵称、handle；X 页面未截图。

## 七、时间线

起 02:01，截图与存档 02:03–02:30，写交卷文件 02:3x，总用时约 40 分钟，未超 60 分钟。
