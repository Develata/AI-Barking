# 1005 D 组抓取与截图日志

工作区 `E:\gitclone\AI-Barking`；基线 HEAD `ac064e681343b7ce6401448a250b1ab2f01d4c63`。浏览器会话始终 `1005-d`，无 commit/push/delete；未改1004与其他组。时间均北京时间2026-10-06，除明确说明外精确到毫秒的值在下面机器台账。

## 每次抓取记录

逐次完整记录（URL、工具、北京时间、结果、文件基名）：[d-capture-records.jsonl](d-capture-records.jsonl)。它是本日志的逐次台账组成部分，每行一次抓取，非抽样。截图逐次记录：[d-screenshot-records.jsonl](d-screenshot-records.jsonl)。`captured; validate body separately` 仅代表写盘成功，最终验收以本文件为准；特别是下述定价页记录已否决，不能误当正文成功。

| 文件基名（一般同时有.json、.txt） | 内容验收 |
|---|---|
| d-beam | 官方全文成功；初次正文只含当前可见编码表，另用d-beam-layout补全部4表。 |
| d-banked、d-paid-reset | 两份帮助中心全文成功，含适用、到期、周周期变化；不是challenge。 |
| d-ads-new、d-ads-trial | 自动定位中文页，仅留路线记录；逐字引文采用随后英文版本。 |
| d-ads-new-en、d-ads-trial-en | 明确en-US官方全文成功。正文后的推荐文章日期不当作主文发布时间。 |
| d-nieman | 文字全文成功，含15+、声明、guardrail、许可、法律限定、无买卖证据、水印；不另存漫画素材。 |
| d-tibo-28、d-tibo-pro、d-claude-reset、d-tibo-timeline | 公开卡片DOM成功，但默认机器翻译；英文原文另取公开字段。时间线只取得当时渲染卡片，不声称完整历史。 |
| d-tibo-28-fields、d-tibo-pro-fields、d-claude-reset-fields、d-tibo-day1 | 公开帖子白名单字段；长推full_text截断处以后面的note文件为准。未保存抓取者导航、菜单或头像。 |
| d-tibo-pro-note.json、d-tibo-day1-note.json | 公开note_tweet完整英文长文成功。 |
| d-tibo-prior、d-claude-expiry | 直接原帖成功，非只有搜索摘录；前帖关系、到期回复父帖ID可核。 |
| d-x-openai-search、d-x-claude-search | 两次限定Latest搜索，分别取得2、7卡；按有界样本解释，不能证明无漏检。 |
| d-reset-tracker、d-claude-tracker | 公开监控站成功，仅L5线索，未替代官方帖。未另读opentherank，因为已有监控线索和官方回溯。 |
| d-pro-tiers | 官方帮助全文成功：through10/29、after that date，未找到20×→10×数对。 |
| d-ithome、d-qbit | 两篇中文原媒体成功，均保留“否则/要么”条件。未把转载站当原站。 |
| d-beam-techcrunch、d-beam-reuters | 两家原站正文成功；TC未独立验证，Reuters明确归因Reflection。 |
| d-avclub | 原站全文成功，定位错误转引；无独立复现依据。 |
| d-hn-beam-page、d-hn-nieman-page | HN公开页面成功，含帖子分数、评论数和评论；未登录HN。 |
| d-hn-beam-comments.json、d-hn-nieman-comments.json | 公开评论ID、作者、正文、发布时间；score=null表示网页不公开评论分数，不能填0或借用帖子分数。 |
| d-beam-layout.json | 四张原table outerHTML全部存档，含全部行、列、NR；隐藏tab的innerText可能连在一起，需用HTML单元格核数，不能按连串数字猜分隔。 |
| d-viral-x-status.json、d-viral-reddit-status.json | 两原帖存在公开post元素，无不可用提示；只保存URL、标题、布尔状态与时间，无头像/漫画图。 |
| d-openai-pricing.json、d-openai-pricing.txt | **否决**：公开URL跳到登录态套餐弹窗。09:40:52这次曾写入页面内容，发现后立即原地替换为REMOVED占位，当前文件不保留账户内容，不作为定价证据。脚本随后加入chatgpt.com跳转阻断，不再读取此页面。 |

## 搜索、失败与重试

检索组合、平台路由见 [d-search-notes.md](d-search-notes.md)。没有登录、输入凭据、接受Cookie、点赞、评论或关注。

| 北京时间 | URL/动作 | 工具 | 结果与处理 |
|---|---|---|---|
| 09:14前后（未留秒级时间） | AgentReach doctor via conda dl | PowerShell | dl环境不存在；直接已安装agent-reach成功。OpenCLI doctor连接正常，版本1.8.7/extension1.0.24；未升级。 |
| 09:14–09:24检索窗口 | Exa中文Codex28天查询 | mcporter/Exa | 免费限额，未配置新密钥；改普通网页检索发现原站。 |
| 同窗口 | HN Algolia查询Beam与cartoonists | 两条curl写盘命令 | 自动审批拒绝：approval required by policy / AskForApproval Never。没有执行/生成目标文件；改读公开HN页面，没有重跑被拒命令。 |
| 同窗口 | 猜测Reuters artificial-intelligence子路径 | web.run open | 未取到原文；搜索找到正确technology路径后浏览器读取成功。 |
| 09:17–09:19 | Tibo帖显示原文按钮 | OpenCLI click/eval | 点击后仍机器翻译；停止重复操作，改公开帖子字段白名单。d-tibo-28-original批次Ctrl-C中止，未生成交付文件。 |
| 09:24:41 | Nieman文字截图30 | CDP | Unable to capture screenshot；09:27:58重拍成功。 |
| 09:24:52 | Nieman截图34 | CDP | 曾写盘但图像为重复/错位内容；d-34-nieman-no-sales.png判废，保留失败记录，不作配图。 |
| 09:26前后（未留秒级时间） | Nieman直接viewport截图31 | OpenCLI screenshot | 得到700px宽单倍、且顶部文字被遮挡；d-31-nieman-viewport.png判废。 |
| 09:32:01、09:33:15、09:34:24 | Nieman许可段32 | CDP | 30秒超时，停止挂起任务；改为最小1100px高viewport、先滚动再截viewport并做几何裁切，09:35:20成功。未改文字。 |
| 09:35:44 | Nieman法律段33 | CDP | 30秒超时，后续扩大viewport再试；最终结果见截图台账与QA。 |
| 09:37–09:39附近（未留精确时刻） | Beam整表初次 | CDP | Page.captureScreenshot 115秒超时；工具提示可能native dialog，但本组未核到对话框原因，不把提示当确定根因。 |
| 同阶段 | 本地d-build-evidence.py整理命令批 | exec_command | 自动审批拒绝，未执行，未重跑。该脚本仅留为未执行辅助记录；不能称其输出已生成。 |
| 09:42前后 | Page.bringToFront | OpenCLI CDP | 方法不在允许列表，移除该调用；未通过其他工具绕过焦点限制。 |
| 09:42:40 | Beam编码表 | CDP | d-28-beam-coding.png只含左侧列，视觉验收失败；缩放+展开原滚动容器后full版本成功。 |
| 09:40:52 | https://openai.com/chatgpt/pricing/ | OpenCLI | 登录态跳转，否决并替换为占位；不是公开定价证据。 |

## 截图方法与验收原则

- 原站页面，DPR=2，最终宽不超过1400物理像素。`d-shoot.mjs`只做滚动、viewport截图和几何裁切；Python/Pillow未重绘或修图。截图没有补字、改数或制作替代表格。
- Beam原表是横向滚动容器。`d-detail-shots.mjs`设置页面zoom=0.42，仅展开原滚动容器宽度与左边距，让全部列可见；完整保留原表DOM内容、tab标题、NR说明。渲染仍DPR2。因为固定1400上限，字较小，编辑可放大阅读原文件；不要再裁掉比较列。具体clip和layout写在JSONL。
- X只截article卡片（1190px宽），没有侧栏、抓取者菜单、回复框或其他回复；截图呈现平台机器翻译，英文逐字以公开原帖字段为准。帖主头像的占位圆来自页面当时渲染，不是抓取者身份。未改图。
- Nieman只取文字段落，未把漫画作品当作配图。所有可用图都需逐张view_image目视核验；失败图保留但不引用为可用配图。

最终QA表、文件清单及隐私检查补记在下方。

## 最终QA

| 图片 | 验收 |
|---|---|
| d-25-banked-expiration.png、d-26-paid-eligibility.png | 可用；分别1400×1256、1400×1032，完整到期/适用段。 |
| d-25-tibo-28.png、d-25-tibo-prior.png | 可用；1190×992、1190×754，只有帖子卡片。平台机器翻译需与原文JSON并读。 |
| d-27-beam-preview-readable.png、d-27-beam-license-readable.png | 可用；1400×866、1400×418，全文段落完整，正常页面缩放。 |
| d-28-beam-coding-full.png、d-28-beam-reasoning-full.png、d-28-beam-tools-full.png、d-28-beam-general-full.png | 可用；1284×586、1284×446、1284×528、1284×432，全部8个模型列、7/5/5/4数据行、tab标题和NR说明均完整。 |
| d-28-beam-efficiency.png | 备用：完整双图、图例、轴、公式脚注，但沿用了0.42缩放，文字偏小。 |
| d-28-beam-efficiency-readable.png | 备用：1400×2452，正常缩放，双图完整；浏览器翻译悬浮按钮接近第一图右下角，另拍clean版免遮挡。 |
| d-29-ads-context.png | 可用；1400×650，三段上下文完整，含图像分离与试点时间地区。 |
| d-30-nieman-tests-response.png、d-31-nieman-guardrail.png、d-32-nieman-license.png、d-33-nieman-law.png | 可用；1400×962、1400×1158、1400×482、1400×722，只含文字，均已目视检查。33最小viewport1800重试成功。 |
| d-34-nieman-sales-watermark.png | 可用；1400×530，页面zoom0.8、DPR2，仅文字，含无买卖证据和水印。右上角2025年日期来自相邻推荐文章，不是本文日期，勿误读。09:46:08正常缩放重试超时后，09:48:23成功。 |
| d-27-beam-preview.png、d-27-beam-license.png | 备用：沿用了0.42缩放、文字小；许可图下沿带下一段残行，采用readable版。 |
| d-29-ads-format.png | 备用：三段事实完整，但上沿有标题残片；采用context版。 |
| d-28-beam-coding.png | 不可用：只截左侧列。 |
| d-31-nieman-viewport.png | 不可用：单倍且遮挡。 |
| d-34-nieman-no-sales.png | 不可用：错位重复，且不是目标段。 |

备用/失败图片按“不删除”要求保留，不应列入正式上传顺序。截图文件名前d-为本次派工要求，编号都在25–34。

## 终检记录

- `git diff --stat` 仍为原有 EDITORIAL.md +1、daily-scan.md +3；`git diff --check`无空白错误，仅LF/CRLF提示。本组没改这些文件、1004或其他组；其他组同期新增文件不归本组认领。
- 对整个1005的json/txt/html完成隐私关键词扫描，D组唯一QQ相关命中是IT之家公共 `connect.qq.com` 分享URL，不是抓取者身份；未命中抓取者handle、头像地址、会话令牌以及被否决定价页的账户内容。也目视检查了两张社交卡片。
- 本组截至终检没有>1MB文件；未提交、未上传、未生成ZIP。
- 另一次只读Python图片尺寸枚举命令也被自动审批拒绝（approval required by policy / AskForApproval Never），没有执行、没有重跑。上表尺寸来自成功截图的clip×DPR记录与目视查看，不声称该Python枚举通过。
- `d-build-evidence.py`未执行；实际evidence与本日志人工按原文整理。`d-tibo-reddit-community.json`补查成功，909帖子分数与30条渲染评论只代表该抓取快照。
- 定价页意外跳转记录已脱敏为占位；该次历史机器台账的“captured”结果在此明确被否决。后续不访问登录态定价弹窗。

最后验收：`d-28-beam-efficiency-clean.png`（北京09:49:43，1400×2452）已目视通过，完整双图、两套图例/坐标轴、整段估算脚注。只将浏览器扩展的 `immersive-translate-popup` 设为不可见，避免其遮图；未改变原站正文或图。它替代readable版作为选用图。总计17张选用图、其余8张为备用/失败图，均保留。

新增文件逐一列于 [d-manifest.md](d-manifest.md)，附每份文件字节数和SHA-256；清单自身不做自引用哈希。交付仅本组前缀文件及两份指定例外evidence-d.md/capture-log-d.md。本次没有为其他组做编辑或验收。
