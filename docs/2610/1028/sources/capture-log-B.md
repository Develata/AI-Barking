# B 组抓取日志（1028 期，nullpt.rs 指纹脚本）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；`barking` 的机器记录另见 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`。清单文件为 /tmp 下的临时 tsv（未入库）。不运行、不下载脚本；`servicepipe.tech/static/checkjs/...` 从未被请求。

## 一、barking fetch 批量存档（`b-<name>.html|.txt|.headers`）

| 时间 | name | URL | 结果 |
|---|---|---|---|
| 19:40 | article | https://nullpt.rs/fingerprinting-bioweapons | ok（39 KB，265 行；正文完整，含作者贴出的代码块与危险文本，仅存档不转述） |
| 19:42 | daybreak-overview | https://help.openai.com/en/articles/20001258-openai-daybreak-trusted-access-for-cyber-overview | ok（正文含 Blue/Red 表与 FAQ；“Updated: 20 hours ago”） |
| 19:42 | daybreak-api | https://developers.openai.com/api/docs/guides/daybreak | ok（1804 行） |
| 19:42 | astra-yahoo | https://tech.yahoo.com/ai/chatgpt/articles/openai-starts-rolling-astra-flagship-142942057.html | ok（Yahoo 转载 Astra 发布，仅作背景，未在证据表引用） |
| 19:42 | aitldr | https://ai-tldr.dev/releases/nullpt-anti-llm-fingerprinting-script/ | ok |
| 19:42 | tildes | https://tildes.net/~comp/1wo6/why_is_this_fingerprinting_script_creating_bioweapons | ok |
| 19:42 | yiichen | https://yiichenhi.substack.com/p/1-ep226 | ok（大页，含其他条目；本条在“3.”） |
| 19:42 | servicepipe-ru | https://servicepipe.ru/ | blocked（HTTP 200，正文 0 字符，需 JS）；未改用浏览器取，原因见下“失败” |
| 19:42 | servicepipe-tech | https://servicepipe.tech/ | blocked（403） |
| 19:42 | nullpt-blog | https://nullpt.rs/ | ok（博客首页，未用） |
| 19:42 | fp-garden | https://fingerprint.garden | ok（需 JS，txt 只有框架文字，未用） |
| 19:44 | agents-radar | https://github.com/845421145-lang/agents-radar/issues/680 | ok |
| 19:44 | zeli-digest | https://zeli.app/digest/2026-10-28 | ok |
| 19:58 | astra-openai | https://openai.com/index/gpt-6-astra/ | ok（2.7 MB html，仅本地、不入库；txt 38 KB） |
| 19:40 | feed.rss | https://nullpt.rs/feed.rss（curl，--max-time 60） | ok（9.7 KB；条目无正文日期，未用） |

## 二、HN 与热度

- `curl https://hn.algolia.com/api/v1/items/50276998` → `b-hn-item-50276998.json`（111 KB；1302 分，37 条顶层评论，共 242 条嵌套评论）。
- `barking heat docs/2610/1028 b --item 50276998 --hn "fingerprinting bioweapons" --hn "nullpt.rs" --reddit "fingerprinting script bioweapons"`，19:41：HN 50276998 为 1303 分／243 评，北京 10-29 01:13 提交 → `b-heat.json`、`b-hn-50276998.json`（与 Algolia 文件同大小，内容重复，两份都留）。Reddit 搜索被重定向到登录页（响应不是 JSON），失败一次即放弃，未做第二次尝试；记为未取。
- HN 评论分数不对外显示，因此证据表只记评论 ID、发言人、时刻与回复数。关键词扫描（servicepipe、veritas、Astra、Daybreak、Node 等）只用本地 JSON。

## 三、截图（`docs/2610/1028/images/13–24`）

工具：`barking shot … render`（headless Chrome，700 CSS px，DPR 2，Windows 绝对路径输出）后 `cropbox/stack` 只裁切；记录在 `b-shots-log.jsonl`。深色底页面截图：nullpt.rs（黑底）、HN（浅色）、OpenAI 帮助中心（白底）。

| 文件 | 来源 | 内容与范围 | 风控检查（已逐张打开目视） |
|---|---|---|---|
| 13-nullpt-title.png | nullpt.rs 页首（render，`--virtual-time-budget=12000` 等标题打字动画结束） | 标题、日期、作者、Preamble 前三段（到 “navigator.webdriver … benign canvases”）；标题完整 | 无危险内容 |
| 14-servicepipe-script-url.png | nullpt.rs | “I wanted to know which script …” 至脚本 URL 一行；未含下一节的函数名 | 含脚本 URL（作者原文所给，非操作内容） |
| 15-astra-refusal.png | nullpt.rs | “This was all pretty alarming …” 至 Astra 拒绝截图及图注 | 图内只有模型说明与界面提示，无危险词；“This was all pretty alarming” 段前的函数名行已被排除 |
| 16-blue-toggle-glm.png | nullpt.rs | “Maybe unsurprisingly …”“Daybreak Blue …”“GLM-5.3 … complied!”三句 | 无 |
| 17-trip-safeguards.png | nullpt.rs | 作者对伪系统提示的概括两句与 Node 引导句 | 已避开伪提示正文与代码块（裁切上边界落在提示正文末行之后、下边界在代码块之前） |
| 18-node-also-decoy.png | nullpt.rs | “However, this is likely also anti-LLM obfuscation …” 一句 | 已避开代码块与其后的字符串清单 |
| 19-standard-antibot.png | nullpt.rs | “What does it actually do?” 一节：navigator/WebGL 信号与作者评价 | 无危险内容 |
| 20-whos-behind.png | nullpt.rs（2 段拼接） | “Who's behind this?” 含 servicepipe.ru 首页图与图注，加 “Special Thanks” 两条（Sshell、Tranco） | 无；拼接间隙 24 px 黑底 |
| 21-radar-exaggeration.png | GitHub agents-radar Issue #680（render） | 表格里该条的一行：标题、两个分数 22/0、摘要句 | 无；页面为匿名渲染，无账号信息 |
| 22-hn-post-card.png | news.ycombinator.com/item?id=50276998（render，匿名） | 标题、1308 分、18 小时前、245 评论；含提交者 HN 用户名（公开信息，非抓取者） | 无 |
| 23-hn-comments-doubt.png | HN 评论永久链接 50281962、50279808（2 段拼接） | arbol、zb3 两条评论全文 | 无；perching_aix（50281002）含粗口，未截 |
| 24-openai-daybreak-table.png | help.openai.com Daybreak 概览（opencli，background 窗口） | “The table below describes the current trusted access levels:” 与 Standard/Blue/Red 三行表 | **1 倍像素（720 px 宽），不是 2 倍**：该页在 opencli 下 `zoom:2` 会把表格挤成单字换行，退回 1 倍；字体仍清楚。未改像素、未放大。无抓取者信息（页面右侧目录与聊天按钮已裁掉） |

没有放入 13–24 的候选：HN 评论 perching_aix（粗口）；servicepipe.ru 官方页（取不到）。

## 四、失败与异常

1. opencli 对 nullpt.rs：`zoom:2` 后全页截图左侧被裁（容器宽于视口），放弃，改用 `barking shot render`（headless Chrome 正常）。
2. headless 首次渲染 nullpt.rs 页首时标题打字动画停在半截（`13-nullpt-title-intro.png`，标题不全，作废），改加 `--virtual-time-budget=12000` 重渲出 13-nullpt-title.png。
3. help.openai.com 在 headless 下是 Cloudflare 验证页（“Verifying...”）与加载动画，改用 opencli（background 窗口）。
4. servicepipe.ru（JS 才有内容）与 servicepipe.tech（403）取不到；未用 opencli 去开：一是 servicepipe.tech 是脚本所在站，不宜主动访问；二是 servicepipe.ru 的内容已有作者截图。因此“服务商有无官方回应”没能到其官网核对，只靠搜索与 HN。
5. Reddit 搜索失败（登录页）。
6. WebSearch 四次（Astra/Daybreak、servicepipe 事件、中文检索、回应检索）用于线索；证据表只引用已存档原页面。未找到 servicepipe 的公开回应。
7. 其他 agent 的临时文件（images/ 下 25–33、tmp-j.png）与本组无关，未动。

## 五、待删除文件清单（本组产生，精确路径，请 Claude 收尾时一次删除）

- docs/2610/1028/images/13-nullpt-title-intro.png（作废：标题被打字动画截断）
- docs/2610/1028/images/b-tmp-astra.png
- docs/2610/1028/images/b-tmp-day.png
- docs/2610/1028/images/b-tmp-day1.png
- docs/2610/1028/images/b-tmp-day2.png
- docs/2610/1028/images/b-tmp-day3.png
- docs/2610/1028/images/b-tmp-hn-arbol.png
- docs/2610/1028/images/b-tmp-hn-post.png
- docs/2610/1028/images/b-tmp-radar.png
- docs/2610/1028/images/b-tmp-top.png
- docs/2610/1028/images/b-tmp-view.png

另：`b-shots-log.jsonl` 里有这些临时图的记录行，可保留。scratchpad 下 b-full.png、b-day-t1/t2/t3.png 为临时图（库外）。
