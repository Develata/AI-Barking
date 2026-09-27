# 0927 抓取与截图日志

所有时间为北京时间 +08:00。curl 首轮时间为发起时间；浏览器输出/截图时间取本轮文件写入完成时刻，不能精确到服务器返回瞬间。截图文件名属于 0927 期，但取证时北京时间已为 2026-09-28。

## 环境与范围

- 开始时核实 main / ded0f347a04e5158bdf984be36ee71bc90323334，仅任务 handoff 未跟踪。
- 读取 AGENTS.md、EDITORIAL.md；未读写 docs/0926/ 和 `.handoff/2026-09-27-0927-evidence.md`。用户“不要读写”优先于矛盾的上一期格式参考要求。
- 使用 agent-reach、opencli-usage、opencli-browser 技能；读取 PDF 技能处理已下载报告。OpenCLI 1.8.7，agent-reach 1.5.0；未安装/更新软件。
- `opencli browser --help`、`opencli doctor`、`agent-reach doctor --json` 均已运行。Doctor 因两个 Bridge profile 同连而报默认连接失败；使用命令前置 `opencli --profile 5df8dbx7 browser ...` 成功。Doctor 对 `--profile` 的自身检查仍失败，但实际浏览器操作已验证成功；未更改全局 profile。
- 曾把 `--profile` 放子命令末尾，返回 unknown option；改用已文档化的全局参数位置。没有修补工具或修改用户配置。
- Reddit 用 agent-reach 指定的 OpenCLI 后端复用已有会话；无登录、凭据输入、Cookie 同意或社交写操作。只读模型攻击示例，未运行其载荷。
- OpenCLI doctor 失败时按 opencli-autofix 的 BROWSER_CONNECT 边界处理，未改工具代码。更新检查显示 agent-reach 已最新；OpenCLI 更新提示忽略。
- 自动审批拒绝运行 `python docs/0927/sources/build-evidence.py`，返回“approval required by policy, but AskForApproval is set to Never”，未提供具体风险原因；此脚本未执行。改为直接写入静态 evidence.md 并使用只读引文核对。未删除草案，明确标为非证据。

## 页面、PDF、社交记录

| 原站 URL | 存档 | 北京时间 | 工具 | 成功/失败与说明 |
|---|---|---|---|---|
| https://arxiv.org/abs/2609.22978 | a-arxiv-abs.html、.md、-paragraphs.md | 2026-09-28 00:02:16 | curl + BeautifulSoup 文字提取 | HTTP 200，正文完整可读；保留所有作者与提交历史 |
| https://arxiv.org/html/2609.22978v1 | a-arxiv-paper.html、.md、-paragraphs.md | 2026-09-28 00:02:16 | curl + BeautifulSoup | HTTP 200；HTML 的部分交叉引用显示 ()，数学表示可能有 ∼ / \sim 重复，未补造缺失原文 |
| https://arxiv.org/pdf/2609.22978v1 | a-arxiv-paper.pdf | 2026-09-28 00:02:16 | curl、pdfinfo | HTTP 200；pdfinfo 可解析，31 页、786319 bytes；摘要历史的 504 KB 作为原站原话保留，不改写为下载大小 |
| https://news.ycombinator.com/item?id=49859112 | a-hn.html、.md | 2026-09-28 00:02:16 | curl + BeautifulSoup | HTTP 200；299 points、94 comments；原站全文含评论 |
| https://www.anthropic.com/news/claude-discovers-novel-enzyme-system | b-anthropic.html、.md、-paragraphs.md | 2026-09-28 00:02:16 | curl + BeautifulSoup | HTTP 200；HTML title 与可见 h1 长短不同，按 h1 取标题 |
| https://www-cdn.anthropic.com/22573675ada52a8ca8a97a1a4b4326b2f208a071.pdf | b-technical-report.pdf、.md、-normalized.md | 2026-09-28 00:05:24 完成 | curl、pdftotext -enc UTF-8 -layout、pdfinfo | 首次 45 秒超时，收到 14852480/21011275 bytes，pdftotext 报缺少 xref/trailer；同一官方 URL 断点续传成功。最终 21011275 bytes、40 页可解析；UTF-8 重新提取，未把残缺 PDF 当成功 |
| https://news.ycombinator.com/item?id=49820134 | b-hn.html、.md | 2026-09-28 00:02:16 | curl + BeautifulSoup | HTTP 200；780 points、799 comments |
| https://alignment.openai.com/misalignment-reports/self-replicating-prompt-injections-exist/ | c-openai.html、.md、-paragraphs.md | 2026-09-28 00:02:16 | curl + BeautifulSoup | HTTP 200；含日期框、Summary、模型节与参考文献 |
| https://www.reddit.com/r/OpenAI/comments/1wr78yj/ | c-reddit-openai.json | 2026-09-28 00:03:44 | agent-reach 路由：OpenCLI reddit read --limit 1 --depth 1 -f json | 成功；原帖及少量评论，score 318；适配器未输出评论总数 |
| https://www.reddit.com/r/artificial/comments/1wr7ayr/ | c-reddit-artificial.json | 2026-09-28 00:04:02 | 同上 | 成功；score 285；未输出评论总数 |
| https://www.reddit.com/r/OpenAI/comments/1wr78yj/ | c-reddit-openai-metadata.json、-state.txt | 2026-09-28 00:07:45 | OpenCLI browser state / 只读 DOM eval | score 316、comment-count 20、UTC 发帖时间；已登录页有自动双语显示，元数据取原帖英文 post-title，不把翻译当原文 |
| https://www.reddit.com/r/artificial/comments/1wr7ayr/ | c-reddit-artificial-metadata.json、-state.txt | 2026-09-28 00:09:56 | OpenCLI browser state / 只读 DOM eval | score 285、comment-count 41；与 read 时点分开保留 |
| https://www.techtimes.com/articles/328046/20260925/deepseek-training-agents-hacked-their-own-sandboxes-escape-catalog-now-public.htm | d-a-techtimes.html、.md | 2026-09-28 00:06:25 | curl + BeautifulSoup | HTTP 200；原标题、作者、EDT 时间均可读 |
| https://finance.sina.com.cn/roll/2026-09-24/doc-iniswvxc5441101.shtml | d-b-sina.html、.md | 2026-09-28 00:06:25 | curl + BeautifulSoup | HTTP 200；正文与标题可读；署来源智药局，不冒称首发 |
| https://www.theverge.com/ai-artificial-intelligence/999470/anthropic-biolab-claude-crispr | d-b-verge.html、.md | 2026-09-28 00:06:25 | curl + BeautifulSoup | HTTP 200；明确 comparing 的标题不列为“发现新 CRISPR”实例 |
| https://zgeo.net/news/anthropic-multi-agent-art-gene-editing-geo-guide | d-b-zgeo.html、.md | 2026-09-28 00:11:52 完成 | curl + BeautifulSoup | HTTP 200；标题与 FAQ 措辞存档 |

没有最终仍打不开的一手主页面；失败发生在工具参数/连接检查、PDF 首次传输和截图中间步骤。未找到的报告编号/DOI、成功逃逸等属于证据缺口，不伪装成网络失败。网页 HTML 没有打包下载全部外部图片/字体资源，不保证完全离线重现；可读文字与浏览器原图另存。

## 最终候选截图（一图一行）

全部来自 OpenCLI browser 的真实原站截图，浅色背景。执行过 Control+0；裁剪/拼接不做缩放、重绘、改字、调色或标注。引文 CSS 字号：摘要约 14px，arXiv 正文 16px，Anthropic 正文 17px，OpenAI 正文约 14.67px；D 组截大标题。字体实际笔画像素高度随字体而异，不把 CSS 字号冒充每个字符黑色像素包围盒高度。若验收要求字形黑色像素也必须 ≥14px，需另行人工判定，本轮未通过放大伪造满足。

坐标是原始 PNG 的 (left, top, right, bottom)，右/下边界不含；两块以上沿竖向按列出的顺序拼接，间隔 **2px RGB(128,128,128)**，宽度一致，未缩放。详细机器记录见 crop-manifest.json。

| PNG（images/） | 原站 URL | 原始截图 / 北京时间 | 裁剪与拼接 | 最终尺寸 | 检查 |
|---|---|---|---|---|---|
| 01-dsec-abstract.png | https://arxiv.org/abs/2609.22978 | a-abstract-raw.png / 2026-09-28 00:03:24 | (10,80,1140,853)，单块 | 1130×773 | 标题、作者、摘要规模、提交日期；去除侧栏浮窗 |
| 02-dsec-agent-behaviors.png | https://arxiv.org/html/2609.22978v1#S6.SS4 | a-section64-raw.png / 2026-09-28 00:06:23 | (195,55,1055,516)，单块 | 860×461 | 6.4 标题与全部 A4 段落，包括 XFS |
| 03-dsec-accidents.png | https://arxiv.org/html/2609.22978v1#S6.SS4 | a-section64-raw.png / 2026-09-28 00:06:23 | (195,525,1055,1093)，单块 | 860×568 | grep、错误 container、yes 与 6.5 防护边界 |
| 04-enzyme-title-known-rt.png | https://www.anthropic.com/news/claude-discovers-novel-enzyme-system | b-page-stable-raw.png / 2026-09-28 00:09:03 | (50,150,1200,340) + (50,1908,1200,2058)，1 条 2px 灰线 | 1150×342 | 标题/日期与已知 RT 段落均完整 |
| 05-enzyme-function-unknown.png | 同上 | b-page-stable-raw.png / 2026-09-28 00:09:03 | (290,1656,960,1910) + (290,5215,960,5415)，1 条灰线 | 670×456 | 功能未知段 + 阵列类比及 RNA 实验段 |
| 06-enzyme-agents-humans.png | 同上 | b-page-stable-raw.png / 2026-09-28 00:09:03 | (290,2280,960,2586) + (290,3043,960,3243) + (290,3260,960,3750)，2 条灰线 | 670×1000 | 搜索数字、人类全部实验、审查/解释流程完整 |
| 07-openai-worm-summary.png | https://alignment.openai.com/misalignment-reports/self-replicating-prompt-injections-exist/ | c-page-raw.png / 2026-09-28 00:06:26 | (288,125,940,447)，单块 | 652×322 | 标题、日期框、完整 Summary |
| 08-openai-worm-models.png | 同上 | c-page-raw.png / 2026-09-28 00:06:26 | (288,5210,940,5322)，单块 | 652×112 | Responsible models and impact 标题/全段 |
| 10-overclaim-dsec-techtimes.png | https://www.techtimes.com/articles/328046/20260925/deepseek-training-agents-hacked-their-own-sandboxes-escape-catalog-now-public.htm | d-a-raw.png / 2026-09-28 00:08:23 | (5,22,1240,370)，单块 | 1235×348 | 原站标识、标题、作者和 EDT 时间 |
| 10-overclaim-enzyme-sina.png | https://finance.sina.com.cn/roll/2026-09-24/doc-iniswvxc5441101.shtml | d-b-clean-raw.png / 2026-09-28 00:10:56 | (115,172,1128,451)，单块 | 1013×279 | 关闭客户端推广弹窗后截图；标题/时间/来源 |
| 10-overclaim-enzyme-zgeo.png | https://zgeo.net/news/anthropic-multi-agent-art-gene-editing-geo-guide | d-b-zgeo-raw.png / 2026-09-28 00:12:24 | (230,176,1020,585)，单块 | 790×409 | 标题、作者、日期；裁掉右下聊天浮窗 |
| 10-overclaim-worm-reddit.png | https://www.reddit.com/r/OpenAI/comments/1wr78yj/ | d-c-reddit-raw.png / 2026-09-28 00:09:05 | (288,82,910,213)，单块 | 622×131 | 只保留原帖英文标题、作者/社区/相对时间；自动中文译文在裁剪范围外，未改原字 |

## 中间截图、观测文件与失败说明

- `a-behaviors-raw.png`（00:05:05）实际仍是摘要页：另一顺序命令在等待 PDF 提取，导航尚未完成便先截了图。未用于 02/03；待导航成功后重新取 `a-section64-raw.png`。保留原文件，不删除，不把错页冒充行为截图。
- `b-page-raw.png`（00:05:17）使用强制宽度 1264，全页截取改变可用宽度/换行，按恢复后的 DOM 坐标裁会截断段落。未采用其裁图；改为不强制宽度的 `b-page-stable-raw.png`（原图 1249×7974），重新裁后逐图目视验证。
- `b-crop-check.png`（00:09:59）是稳定原图的测试裁剪，检查已知 RT 段完整，不是另一个配图。
- `d-b-raw.png`（00:09:36）含新浪客户端推广弹窗，未用于候选图；通过 state/find 定位“关闭”链接并正常点击，再得 `d-b-clean-raw.png`。不是 Cookie 同意。
- `d-c-reddit-raw.png` 含用户已有界面的双语显示；最终图仅裁英文原标题区，没有复制翻译或重绘文字。原始截图及 state 含页面界面，非浏览器凭据/会话数据库。
- `a-browser-state.txt`、`b-browser-state.txt`、`c-browser-state.txt`、`d-a-browser-state.txt`、`d-b-browser-state.txt`、`d-b-zgeo-state.txt` 为各同名原站只读 DOM 观测；`b-dom-blocks.json`、`b-dom-stable.json`、`c-dom-blocks.json` 是文字/坐标/字号测量，用于裁剪定位。
- `fetch-results.json`、`d-fetch-results.json` 是 curl URL/HTTP 状态/时间/错误记录；`crop-manifest.json` 是原图时间、尺寸与裁剪坐标。
- `.md` 文字提取没有改写原文；PDF 文本保留连字符与分页，阅读时可对照原 PDF。报告是文字核验，未做新科学实验或验证论文所有结论。
- `build-evidence.py` 是未执行草案，非证据与非验收程序；保留原因见上。最终 evidence.md 由静态写入并另行检查。

## 验收边界

最终候选图 12 张（指定 8 张 + D 组 4 张），每张在上表和 crop-manifest.json 对应，宽度均 ≤1400。引文逐字检查使用 HTML 实体解码、空白归一化，不容忍替词。B6 的编号/DOI 与 A7 的成功逃逸留缺口；D 组未找到项明示。

未生成正文、images/README.md、ZIP；未 commit/push、安装、删除或修改基线已有文件。最终 git status 另见 verification.md。

最终检查另发现其他进程在本轮工作期间新增 `docs/0927/doc_0927_publish.txt`；本代理未创建、读取或修改，未计入本轮交付。目录级 `git status --short` 会把它和本轮文件共同折叠到 `?? docs/0927/`，因此不能仅凭该行排除并发写入。
