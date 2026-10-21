# A 组取证与截图日志（1021）

采集日：2026-10-21。此日志记录三篇原站材料的获取方式、截图源图和最终裁切。浏览器截图均用 OpenCLI 的 foreground 页面；图片像素只由 `barking shot cropbox` 裁切或 `barking shot stack` 原样纵向拼接，没有重排、描画、调数字或缩放。原站页面浏览器报告 `devicePixelRatio=1`，所以日志如实记录 CSS zoom；未把放大页面称为硬件 DPR 2。

## 原文存档

| 来源 | 操作与状态 | 本地档案 | 完整性边界 |
|---|---|---|---|
| OpenAI Alignment 监控报告 | `barking fetch`；2026-10-21 15:53:05 +08:00；HTTP 200 | `a-monitoring.html`（61,051 bytes）、`a-monitoring.txt`（28,679 bytes）、`a-monitoring.headers` | 原站 HTML 与文本提取成功；SHA-256 和响应元数据见 `a-fetch-log.jsonl`。 |
| OpenAI Help Center 锁定聊天说明 | `barking fetch`；2026-10-21 14:53:04 +08:00；HTTP 200 | `a-locked-chats.html`（103,371 bytes）、`a-locked-chats.txt`（16,168 bytes）、`a-locked-chats.headers` | 原站 HTML 与文本提取成功；页面显示相对更新时间 “14 hours ago”，此时间会滚动。 |
| SEC EDGAR SpaceX Q3 exhibit | `barking fetch`；2026-10-21 14:53:04 +08:00；HTTP 403 | `a-spacex-q3.html`（4,818 bytes）、`a-spacex-q3.headers` | 只保存了 403 响应，不把它说成原文存档。 |
| SEC EDGAR SpaceX Q3 exhibit，替代读取 | Jina Reader 文本投影；OpenCLI 打开 SEC 原站并读取渲染 DOM | `a-spacex-q3-jina.md`（42,988 bytes）；`a-spacex-q3-browser.html`（834,772 bytes） | Jina 是转换后的文本投影；browser HTML 是浏览器渲染 DOM 快照，含浏览器注入的属性和缩放状态。二者都不是 SEC 原始 HTTP 响应。最终数字另外以直接原站截图中的表格核对。 |

`a-fetch-log.jsonl` 是批量抓取时间、状态、大小和哈希的机器记录。原始 URL 均为公开页面；没有使用登录态、个人账号或用户私密内容。

## 原站截图与交付底图

所有像素框按 `[x0,y0,x1,y1)` 解释；截图比例列的是 OpenCLI 页面的 CSS zoom。OpenCLI 未提供设备 DPR 设置参数，本机 Chromium 报告 DPR=1。OpenCLI 在可读的原站位置生成全视口截图后，使用仓库的 barking 工具裁出证据区域。

| 交付文件 | 原站 URL / 截图源图 | 截图条件与裁切 | 实际用途 |
|---|---|---|---|
| `images/01-monitoring-raw.png`（935×540） | OpenAI Alignment；`a-monitoring-outcome-resizehook.png`（1400×1400） | OpenCLI session `a-monitor3`，foreground，CSS zoom=2；`cropbox [425,120,1360,660)`。 | 完整保留 “The misalignment monitor still ran…” 结果句及紧随的 “Final response” 框；左目录、滚动条及右下翻译按钮均不在裁片内。旧裁片曾为1365×1150，按统筹要求另存 `a-monitoring-outcome-before-recrop.png`；最终raw为正文列935×540。 |
| `images/02-astra-test-raw.png`（935×400） | OpenAI Alignment；`a-monitoring-astra-test-view.png`（1400×1800） | OpenCLI session `a-monitor3`，foreground，CSS zoom=2；`cropbox [425,135,1360,535)`。 | 保留完整“迁移删除监控集成并修改测试”段落及 orange “New test”断言；裁片从正文段开始，未包含其上方“Copying a reference implementation”小节标题；标题由 `evidence-A.md` 的原文位置记录，没有拼造。 |
| `images/02-astra-denial-raw.png`（935×215） | OpenAI Alignment；`a-monitoring-astra-denial-view.png`（1400×1000） | OpenCLI session `a-monitor3`，foreground，CSS zoom=2；`cropbox [425,85,1360,300)`。 | 保留 “Final response” 和下一节标题；初始全视口截图顶部的用户问题被截断，正式裁片不包含它，只用来呈现原站答复。 |
| `images/03-locked-chats-raw.png`（1160×1054） | OpenAI Help Center；`a-locked-chats-change-view2.png`、`a-locked-chats-reset-view.png`（各1400×1200） | OpenCLI session `a-locked-chats`，foreground，CSS zoom=2；stack两段：`[100,60,1260,400)` 与 `[100,60,1260,750)`，段间空白24px，白底。 | 上段含“更改 PIN 不删聊天、须输入当前 PIN”；下段含“忘记本身不删、重置删除范围及不可撤销”。帮助中心文章栏本身偏窄，最终裁片保留右侧官方目录作为同页背景，宽1160px；避开目录内滚动条和最右浏览器滚动条，不改像素。初版660px正文窄裁片另存为 `a-locked-chats-raw-narrow.png`；随后尝试的1260px宽裁片含目录滚动条，已由当前1160px版本取代。`a-shots-log.jsonl` 保留全部尝试，最后一行对应当前03文件。 |
| `images/04-spacex-results-raw.png`（1140×785） | SEC EDGAR；`a-spacex-q3-financial-highlights-view2.png`（1400×1000） | OpenCLI session `a-sec-table`，foreground，CSS zoom=1.55；`cropbox [120,5,1260,790)`。 | 完整呈现 “Third Quarter Financial Highlights” 标题、`$, in millions`、5个期间列头、Revenue、Income (Loss) from Operations、Segment Adjusted EBITDA、Capex全部行列。缩放1.55是为在1400px视口内完整显示表格；截图上的字清楚，未删表行列。 |
| `images/05-spacex-segment-raw.png`（1140×765） | SEC EDGAR；`a-spacex-q3-si-segment-view.png`（1400×1000） | OpenCLI session `a-sec-table`，foreground，CSS zoom=1.55；`cropbox [120,15,1260,780)`。 | 完整呈现 “SI Segment Operating and Financial Data”、单位、5个期间列头及所有行；含广告538、SI solutions & infrastructure 7,022、SI revenues 7,560、营业利润、EBITDA、Capex。 |

`a-shots-log.jsonl` 自动记录最终 cropbox/stack 动作和像素框。OpenCLI 原始截图不由该JSONL记载，以上表格补记了其文件名与全图尺寸。

## 操作备注与缺口

- 初版 `01-monitoring-raw.png` 含左侧目录、页顶代码尾片和下一节；统筹审核后已复制为 `a-monitoring-outcome-before-recrop.png`，并按正文列重新裁切为当前01。中间尝试过一个475×410的裁框，结果随后删除；自动日志仍保留命令历史，最新的01日志行才对应当前正式文件。
- Astra 测试截图保留完整段落和代码断言，但不含上方小节标题；否认截图只支持答复本身。完整小节标题、原文位置和两者先后关系由官方原文 `a-monitoring.txt` 对照。
- SpaceX 曾先把 `businesshighlightssegment_.jpg` 误作财务表定位目标；实际它是配图式 Business Highlights 信息图，未作为04/05证据。改为在同一 SEC 原站定位 HTML 表格后截取04与05。第一张财务表试截图标题贴近顶边，最终采用 `financial-highlights-view2` 再裁，标题有完整上边距。
- SEC 某旧OpenCLI session在高页长截图上没有及时返回；改用新 foreground session `a-sec-table` 截取目标表，04、05全表均成功。不会用图像重绘代替原站表。
- 物理设备DPR2未验证：OpenCLI没有DPR控制选项且回报DPR=1。使用CSS zoom（OpenAI/Help Center=2；SEC=1.55）后输出仍为1400×1000的浏览器视口像素。需严格硬件DPR2时，应在发布前另用支持 `deviceScaleFactor=2` 的采集环境复截；当前图像内容可读且列行完整。
- SEC原始HTML直抓仍为HTTP403；目前可审计的原站证据是OpenCLI真实页面截图，辅以明确标为转换文本的Jina投影和明确标为渲染DOM的浏览器快照。没有把这些替代材料冒称原始响应。


