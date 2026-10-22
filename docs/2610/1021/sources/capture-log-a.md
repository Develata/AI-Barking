# 1022 期 A 组采集日志

所有时间均为北京时间（UTC+8）。采集日期为 2026-10-22。Anthropic 与 OpenAI 页面均把发布日期标为 2026-10-21，未显示具体发布时间。只访问派工列出的三个官方 URL；未搜索其他来源。

## 网页批量存档

| 北京时间 | URL | 工具/结果 | 备注 |
|---|---|---|---|
| 16:43:47 | https://openai.com/index/chatgpt-ads-expands-26-new-countries/ | `barking fetch`，HTTP 200，`ok` | 436,547 字节；已写 `.html`、`.headers`、`.txt`；SHA-256 见 `a-fetch-log.jsonl`。只作文字核验，按派工未截图。 |
| 16:43:48 | https://openai.com/index/building-chatgpt-ad-stack/ | `barking fetch`，HTTP 200，`ok` | 561,589 字节；已写 `.html`、`.headers`、`.txt`；SHA-256 见 `a-fetch-log.jsonl`。 |
| 16:43:51 | https://claude.com/resources/articles/claude-duet | `barking fetch`，HTTP 200，`ok` | 573,971 字节；已写 `.html`、`.headers`、`.txt`；SHA-256 见 `a-fetch-log.jsonl`。 |

采集采用 `barking fetch` 的匿名 GET；三条均无跳转、阻拦或正文解析失败。单个存档文件均小于 1 MB。

## 截图

| 北京时间 | URL | 工具/结果 | 备注 |
|---|---|---|---|
| 16:44:25 | https://claude.com/resources/articles/claude-duet | `barking shot render`，1400×1200 | 正文区为空白，仅导航可见；未用作证据。 |
| 16:45:02 | https://claude.com/resources/articles/claude-duet#how-claude-duet-works | `barking shot render`，1400×1200 | 正文区为空白；未用作证据。 |
| 16:45:27 | https://openai.com/index/building-chatgpt-ad-stack/ | `barking shot render`，1400×1200 | 页面仅显示加载标记；未用作证据。 |
| 16:50:26 | https://claude.com/resources/articles/claude-duet | Playwright CLI 匿名访客页，700×600 CSS、DPR 2 | 等页面动画显示后截取介绍标题与美国 Pro/Max 候补信息；作为 `01` 第一段。 |
| 16:53:59 | https://claude.com/resources/articles/claude-duet#how-claude-duet-works | Playwright CLI 匿名访客页，900×600 CSS、DPR 2 | 截取机制小节标题；作为 `02` 第一段。 |
| 16:56:43 | https://claude.com/resources/articles/claude-duet | Playwright CLI 匿名访客页，700×600 CSS、DPR 2 | 截取 Pro/Max 计划额度说明及候补入口；作为 `01` 第二段。 |
| 16:57:46 | https://claude.com/resources/articles/claude-duet | Playwright CLI 匿名访客页，700×750 CSS、DPR 2 | 截取每数小时账户检查与需批准动作；作为 `02` 第二段。 |
| 16:58:07 | https://claude.com/resources/articles/claude-duet | `barking shot stack` | 生成 `01-duet-raw.png`，1400×1019，原像素两段纵向拼接。 |
| 16:58:22 | https://claude.com/resources/articles/claude-duet | `barking shot stack` | 生成 `02-duet-controls-raw.png`，1400×1084，原像素两段纵向拼接。 |
| 16:59:20 | https://openai.com/index/building-chatgpt-ad-stack/ | Playwright CLI 匿名访客页，700×750 CSS、DPR 2 | 截取官方发布日期、文章标题与作者；作为 `03` 第一段。 |
| 16:59:58 | https://openai.com/index/building-chatgpt-ad-stack/ | Playwright CLI 匿名访客页，700×750 CSS、DPR 2 | 截取广告状态、定制 LLM、与答复生成分开的服务段；作为 `03` 第二段。 |
| 17:00:31 | https://openai.com/index/building-chatgpt-ad-stack/ | `barking shot stack` | 生成 `03-ads-raw.png`，1400×1214，原像素两段纵向拼接。 |

### 三张交付图的拼接与省略

以下都是同一官方页面的非连续区域纵向拼接；图片之间的浅色空隙是 `barking shot stack` 加入的分隔底色。未把中间未采集的页面内容伪装成连续原图，也未编辑 DOM、增绘文字或重绘页面。坐标为源截图像素的左、上、右、下；每个裁切矩形、源图尺寸和输出尺寸也记录于 `a-shots-log.jsonl`。

- `01-duet-raw.png`（1400×1019）：先取 `duet-intro-live.png`（1400×1200）的 `(0,380,1400,1135)`，再取同页 `duet-plan-live2.png`（1400×1200）的 `(0,250,1400,490)`，中间留 24 px 分隔。第一段保留介绍标题及 US Pro/Max 候补上下文；第二段是后文计划额度说明。页面发布日期从官方页面文字存档核对，未出现在这张截图中。两段之间页面内容省略，因此不可视为整页连续截图。
- `02-duet-controls-raw.png`（1400×1084）：先取 `duet-controls-wide.png`（1800×1200）的 `(0,450,1400,560)`，再取同页 `duet-controls-approvals-live.png`（1400×1500）的 `(0,250,1400,1200)`，中间留 24 px 分隔。第一段只保留“How Claude Duet works”小节标题；标题下到“Receive a message...”之前的两段（分别以“Your context from Claude is the foundation.”、“Work continues when you step away.”开头）未纳入图中。第二段从后续段落开始展示每数小时账户检查与需审批动作。该图明确为同页拼接、中间省略，不是连续原页截图。
- `03-ads-raw.png`（1400×1214）：先取 `ads-stack-title-live.png`（1400×1500）的 `(0,180,1400,600)`，再取同页 `ads-stack-core-live.png`（1400×1500）的 `(0,340,1400,1110)`，中间留 24 px 分隔。第一段保留页面日期、标题和作者；标题后的开篇背景段（以“Traditional advertising systems are built to respond...”开头）未纳入图中，第二段从后续“Our ads system supports...”架构段落开始，展示广告系统 LLM、跨轮次状态及与答复生成分离的独立服务描述。该图明确为同页拼接、中间省略，不是连续原页截图。

三张最终图均已逐张打开检查，文字清晰、原文范围对应，未重绘或改动页面像素；裁切/拼接过程记录在 `a-shots-log.jsonl`。早期三次 `barking shot render` 只留下工具的临时渲染和日志，没有作为交付图。由于站点在加载后才显示正文，最终截图通过匿名 Playwright 访客浏览器等待页面显示，再由 `barking shot stack` 原像素裁切与拼接；未使用登录态或用户 Cookie。A3 依派工只存档核验，不截图。

## 失败与未做项

- 外部网页抓取失败：无。三条官方来源均一次批量取得成功；未对来源原地重试。
- 截图不可用：三次 `barking shot render` 只呈现空白正文/加载标记，均已在上表注明；随后通过匿名浏览器完成对应截图。
- 社区高赞评论与非官方夸大实例：未检索，因本组范围限定为三条官方 URL；不能据此判断是否存在。
- A1/A2/A3 的发布页只有日期，没有具体时刻；相关事实按官方自述记录，未作独立验证。
- A2 的“独立服务”有页面支持；页面未说明广告 LLM 与答复模型是否采用不同权重，因此不把它记成已证实的“独立模型”。
