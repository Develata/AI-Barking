# 1013 期 B 组抓取日志（Anthropic 自我复制研究）

时间为北京时间（UTC+8）；本机美东（EDT）= 北京 − 12 小时。执行方：Claude Sonnet 5.5（`claude-sonnet-5-5`）子代理。时刻取自命令发出时的 `date -u` 读数（curl 批次）与脚本记录（`b-browser-records.jsonl`），为近似值。

工具：curl（浏览器 UA，匿名，`b-fetch.sh`）；Playwright Core + 本机 Chrome（匿名、全新临时 profile、不登录、不带 Cookie、UA 去掉 Headless 标记、en-US、tz Asia/Shanghai、DPR 2；脚本 `b-browser.mjs`、`b-fetch.mjs`、`b-probe.mjs`、`b-figtext.mjs`，改写自 1012 的 `a-browser.mjs`、`a-fetch.mjs`）；HN Algolia 与 Firebase API（`b-heat.py`、`b-hn-flatten.py`）；fxtwitter 公开 JSON；WebSearch；Exa（web_search_exa）。未使用 firecrawl，未使用 opencli / agent-reach，未登录任何站点，未运行读取浏览器 Cookie 的工具；未安装或运行 Codex CLI、未启动守护进程；未发帖/点赞/评论/关注/加 reaction。

## 1. 抓取

| 约时刻 | URL | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 19:26 | https://www.anthropic.com/research/self-replicating-ai | curl | 200，230,642 B | `b-anthropic-self-replicating.html/.headers/.txt`（`b-html2txt.py` 复制自 1012 `a-html2txt.py`，未改） |
| 19:27 | https://assets.claude.ai/brand/artifacts/editorial/ai-worms-figures-blog/{04,07,08,09,10,11,12,13,14,15,16,17,19,20,21,22}-…-light.html（页内 16 个 iframe 的 src） | curl | 全部 200 | `b-figs/b-fig-*.html`；`b-figtext.mjs`（Playwright 匿名渲染）生成同名 `.txt` |
| 19:27 | https://cdn.sanity.io/files/4zrzovbb/website/04cb53c1d91c7dc7493e2d40cf4a40d865ba97b7.mp4（演示视频） | curl -I | 仅查头：200，765,308 B，video/mp4；未下载 | 无 |
| 19:30–19:34 | 同上文章页（Playwright，`b-probe.mjs`、`b-browser.mjs` 试验） | Playwright | 探测布局；试验图 `tmp-top.png` 在草稿目录（不入库）；试验运行记入 `b-browser-records.jsonl`（名 `b-test`、`b-anthropic-render`） | 无 |
| 19:32–20:38 | 文章页截图 13–24 | Playwright，`b-browser.mjs`（见 §2） | 成功；`b-anthropic-rendered.txt` 为第一次不带 `NOARCHIVE` 的渲染文本（36,070 字，未展开折叠框） | `images/13…24`、`b-anthropic-rendered.txt`、`b-shots-*.json` |
| 20:39 | https://hn.algolia.com/api/v1/items/50061603 、https://hacker-news.firebaseio.com/v0/item/50061603.json 、topstories | curl | 200；91 分、101 评、名次 319 | `b-hn-50061603.json`、`b-hn-50061603-fb.json`、`b-hn-50061603.txt`（展平） |
| 20:40 | https://www.businessinsider.com/ai-worms-digital-slime-mold-nightmare-fuel-anthropic-2026-10 | curl | 200，412,842 B；页面带订阅墙标记，但匿名返回的 HTML 含正文；未使用绕过手段 | `b-m-businessinsider.*` |
| 20:41–20:43 | 媒体 15 篇：Al Jazeera、SecurityWeek、TNW、AI Weekly ×2、Startup Fortune、madrobot、Times of India、IBTimes SG、LavX、21经济网、IT之家、INSIDE、AIHOT 条目页（NDTV、安全内参见 §3） | curl（`b-fetch.sh`） | 13 篇 200；NDTV 403；安全内参 TLS 握手失败 | `b-m-<name>.html/.headers/.txt` |
| 20:43 | https://x.com 无；https://api.fxtwitter.com/status/2109760682761814286 、…/2109753213511704873 | curl | 200（Freeman 引用推文、Zhou 发布推文；均为作者公开的职业发言）；JSON 含作者头像链接与简介，均为其公开资料 | `b-x-bucketofkets-raw.json`、`b-x-willzhou-raw.json` |
| 20:44 | https://z.ai/blog/glm-5.2 | curl | 200 但仅 598 B JS 壳页，无正文 | `b-m-glm52-z-ai-blog-glm-5-2.*`（无内容） |
| 20:44 | https://docs.z.ai/guides/llm/glm-5.2 | curl | 200，538,995 B | `b-m-glm52-docs-z-ai-guides-llm-glm-5-2.*` |
| 20:45 | https://huggingface.co/api/models/zai-org/GLM-5.2 | curl | 200（模型元数据 JSON） | `b-m-glm52-huggingface-co-api-models-zai-org-GLM-5-2.*` |
| 20:44 | https://huggingface.co/zai-org/GLM-5.2 | curl | **失败**：TLS 握手错误；空文件已删 | 无 |
| 20:45–20:46 | Reddit 站内搜索：全站、r/singularity、r/artificial、r/technology、r/cybersecurity | Playwright 匿名（`b-fetch.mjs`） | 200 可渲染；匿名页不显示部分帖子分数 | `b-reddit-search-*.txt` |
| 20:47 | r/LocalLLaMA 搜索 | Playwright 匿名 | 渲染成功，无本事件帖 | `b-reddit-search-localllama.txt` |
| 20:47 | r/ClaudeAI 搜索 | Playwright 匿名 | **失败**（未生成文件）；未重试 | 无 |
| 20:47 | https://archive.org/wayback/available?url=anthropic.com/research/self-replicating-ai | curl | 200：最近快照 20261013004429 | `b-wayback-avail.json` |
| 20:48 | https://web.archive.org/web/20261013004429id_/https://www.anthropic.com/research/self-replicating-ai | curl（首次未加 `--compressed` 得到压缩二进制，覆盖重取） | 200，53,977 B（已解压）；仅用于查看旧版措辞，已在 evidence 注明 | `b-wayback-20261013004429.html/.txt` |
| 20:48 | https://aihot.news/hot | Playwright 匿名 | 200 渲染成功（10月13日 20:45 更新） | `b-aihot-hot.txt` |
| 21:05 | https://www.qbitai.com/2026/10/502920.html 、https://eu.36kr.com/en/p/4023779264991107 | curl（`b-fetch.sh`） | 均 200 | `b-m-qbitai.*`、`b-m-36kr-eu.*` |
| 20:49 | HN 热度脚本 | python，`b-heat.py` | 200；91 分、101 评、名次 322 | `b-heat.json` |
| ~20:50–20:55 | WebSearch ×5（英文/中文）；Exa 搜索 ×3（含中文媒体、Freeman 引语出处） | WebSearch、Exa | WebSearch 对本事件几乎无结果；Exa 返回数十篇转载/报道（URL 见 evidence B8）。Freeman “gobble up vulnerable compute” 出处经 IBTimes 嵌入推文查到为其 X 推文（`b-x-bucketofkets-raw.json`） | Exa 输出未单独存档（仅作线索） |

## 2. 截图（均为 DPR 2，页面浅色底，CSS 视口宽 1000；匿名未登录；无登录用户头像）

方法：`b-browser.mjs`（匿名 Playwright）。因内嵌图表是 `loading="lazy"` 的跨域 iframe，`fullPage` 截图会得到空白，故对含图表的截图用 `SCROLLSHOT=1`：把视口高度撑到截图区域高度、滚到区域顶部、等待 4 秒让 iframe 载入、把 `position: fixed/sticky` 的页头设为不可见后按视口截图；`EXPAND=1` 点击页面自带的 “Read more” 展开折叠框；`VIDEOEND=1`（仅图 14）把演示视频跳到末帧、暂停、关掉播放控件条。未改动任何像素内容。裁切边界落在文字行之间。宽度单位为页面 CSS px（原图像素为 2 倍）。

| 文件 | 内容 | 范围（CSS px，x×宽） | 目视核对 |
|---|---|---|---|
| `13-anthropic-intro-summary.png` | 栏目 “Frontier Red Team”、标题、日期、署名、开头两段（months away、8 of the 9、end of this year） | 140 × 720 | 已打开：文字清楚，范围正确 |
| `14-anthropic-tv-boxes-demo.png` | “We run Gemma4-26B-A4B…” 段、15 盒子演示（视频末帧：Austin、16 tokens、0.39 tokens per second · done）、图注 | 140 × 720 | 已打开：文字清楚。首版截图的播放控件条遮住最后一行，已去掉控件条重截并覆盖（自己的试验图） |
| `15-anthropic-8node-replication.png` | 小节 “Autonomous inference setup and replication (with simplified cyber)”、redroid 与 “separate inference server” 段、折叠框 “Why task the models…”（已展开）、8 节点图（0/8 ×3、GLM-5.2 8/8 in 4/5 runs）、“Only GLM-5.2…” 段 | 60 × 880 | 已打开：文字清楚；展开后的折叠框底部有留白（页面自身布局） |
| `16-anthropic-upper-bound.png` | “It appears that GLM-5.2…” 段（upper bound、Mirai） | 140 × 720 | 已打开：文字清楚 |
| `17-anthropic-eci-trend.png` | 门槛/天花板两条、ECI 图（图例、轴、capability floor ≤151.8、2.4 ECI to go）、图下说明（13.7 ECI/年、R²、end of 2026、proxy metric、rough estimate） | 60 × 880 | 已打开：文字清楚，图例与轴完整 |
| `18-anthropic-botnet-estimate.png` | Kimwolf 推算三段（133k、80k tokens/s、40 H100、rough estimate） | 140 × 720 | 已打开：文字清楚 |
| `19-anthropic-speed-table.png` | C 节标题、两段、速度表（Gemma 27.3–31.8h / 71–150h；Qwen3.5-9B 10.9h / 28–51h） | 140 × 720 | 已打开：文字清楚，表框完整 |
| `20-anthropic-inference-grid.png` | “We measure the ability…” 段、两条操作提示、交互表（标题、说明、开关、9 列×11 行、图例） | 60 × 880 | 已打开：文字清楚。表下方的箱线图未纳入（页面内 iframe 高度不足、图被截断，故裁在图例之后）；“Jetson†” 脚注与 “data as of 30 July 2026” 在表说明内 |
| `21-anthropic-17node-scripted.png` | “As a sanity check…” 段、17 节点脚本化推理图（6 个模型）、图注 | 60 × 880 | 已打开：文字清楚 |
| `22-anthropic-mitigations.png` | F 节 Blue-team mitigations 全文 | 140 × 720 | 已打开：文字清楚 |
| `23-anthropic-spearphishing-appendix.png` | 附录 E：正文、图（CyberSecEval 3 复现，含 Hermes-3-405B* 脚注）、图注、405B 说明句 | 60 × 880 | 已打开：文字清楚 |
| `24-anthropic-learning-generations.png` | “Learning on the job”：正文、六代 Qwen3.5-9B 成功率图（45%…84%）、图注 | 60 × 880 | 已打开：文字清楚 |

偏离清单处：图 15 含“separate inference server”段与 8/8、4 of 5 句；图 16 只含 “upper bound” 段（两句 “separate inference server” 与 “upper bound” 分属相距两屏的两段，分别在 15、16）。清单外加的图：19（速度表）、20（9 类硬件全表）、21（17 节点消融）、22（缓解）、23（附录 E）、24（学习代际），共用满编号 13–24。

## 3. 失败或未取得

- NDTV（403，518 B 反爬壳，已删试验文件）、安全内参 secrss.com/articles/94642（TLS 握手失败，空文件已删）：仅有 Exa 搜索摘要，未存档页面。
- https://huggingface.co/zai-org/GLM-5.2 页面（TLS 失败）：改用 API 元数据。
- https://z.ai/blog/glm-5.2：JS 壳页，无正文。
- r/ClaudeAI 站内搜索失败；匿名 Reddit 页不显示 r/singularity、r/technology 相关帖的分数与评论数。
- HN 评论得分：Algolia items 接口不含评论分数，无从取得。
- 中文媒体：机器之心、虎嗅仅做一次带域名的 WebSearch，无结果；36氪中文站稿未找到（只有英文站译本）。
- 未查 X 上智谱官方账号、Anthropic 官方账号对本文的发帖；Opus/其他研究者的跟帖未扫。

## 4. 误操作与试验文件

- **我误删了 C 组的截图（已被 C 组补回）**：为清理自己的 15–24 号旧图，我执行了 `rm docs/2610/1013/images/2*.png`（约北京 19:57），通配符同时匹配了当时已在 `images/` 的 C 组 25–29 号文件。只读核对 `c-browser-records.jsonl`：C 组 25–29 的截图记录在北京 19:43–19:48，同名文件又在 20:02:02–20:02:37 重新生成（25、26、27、28、29 各一条），晚于我的删除；目前 `images/` 中 25–29 均存在。因此判断我确实删了 C 组 19:43–19:48 版本，C 组随后自己重截；我无法确认 20:02 的重截是否由我的误删触发，也未打开 C 组的新图核对。无回收站，被删的旧版无法恢复。
- 其余删除均为本组自己生成的试验文件：首版 14 号图（覆盖）、15/16/17/18/19/20/22 的早期 fullPage 版本（图表空白、范围错）、NDTV/安全内参的空壳/空文件、`tmp-top.png`（草稿目录）。
- `b-browser-records.jsonl` 含试验运行记录（`b-test`、`b-anthropic-render`、`b-anthropic-rendered` 多次、`b-reddit-search-*`、`b-aihot-hot`）。
- 抓取者信息自查：对 `docs/2610/1013/` 本组文件 grep 本机用户名、绝对路径、邮箱，无命中（“qq.com”域名属中文站点自身链接，非泄露）。脚本均用 `os.homedir()` 或相对路径，未写入本机绝对路径。
