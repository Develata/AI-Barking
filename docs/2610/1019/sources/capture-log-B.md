# 1019 期 B 组抓取日志

时区：北京时间（UTC+8）；本机为美东（EDT），北京时间 = 本机时间 + 12 小时。执行模型 claude-sonnet-5-5。未登录、未输入凭据、未接受 Cookie、未用镜像或代理域名（web.archive.org 快照只用于“页面以前写了什么”，已在证据表注明）。opencli 因同时连着两个浏览器 profile，用环境变量 `OPENCLI_PROFILE=5df8dbx7` 选了其一，没有执行会改配置的 `opencli profile use`。

## 一、网页存档（barking fetch，清单 `b-manifest.tsv`、`b-manifest-2.tsv`、`b-manifest-3.tsv`；明细 `b-fetch-log.jsonl`）

| 开始 | 条目（URL） | 结果 | 存档 |
|---|---|---|---|
| 19:12:48 | redact（https://redact.dev/blog/whatsapp-help-pages-hidden-ai-instructions） | 200 ok，420 行文本，是正文 | b-redact.html/.txt/.headers |
| 19:12:49 | faq-now（https://faq.whatsapp.com/2138577903196467） | **failed**：HTTP 400（curl 被拒；之后带浏览器 UA 的 curl 再试一次仍 400，停止）；改用 opencli，见第二节 | b-faq-now.html（错误页，无 .txt） |
| 同上 | hn-item（https://hn.algolia.com/api/v1/items/50159341） | 200 ok | b-hn-item.json |
| 19:12:50 | redact-home（https://redact.dev/） | 200 ok | b-redact-home.* |
| 同上 | wired（https://www.wired.com/story/whatsapp-help-pages-leaked-hidden-instructions-for-a-meta-chatbot/） | 200 ok，208 行文本，是正文，无付费墙 | b-wired.* |
| 19:23:20 | notebookcheck（https://www.notebookcheck.net/WhatsApp-help-pages-told-its-AI-not-to-mention-privacy-regulators.1428920.0.html） | **blocked**：403，Cloudflare 质询；不重试、不绕过 | b-notebookcheck.html（质询页，无 .txt） |
| 同上 | newsbytes（https://www.newsbytesapp.com/news/science/meta-accidentally-leaks-whatsapp-ai-chatbot-s-hidden-rules/story） | **blocked**：403 | 无有效存档 |
| 19:23:22 | mindgrow（https://mindgrow.io/technology/a-bug-exposed-the-hidden-rules-for-whatsapp-s-support-chatbo-128363） | 200 ok | b-mindgrow.* |
| 19:23:30 | devto（https://dev.to/hacksgr/whatsapps-hidden-instructions-forbade-ai-from-sending-users-to-data-protection-authorities-4bm5） | 首次连接失败，自动重试后 200 ok | b-devto.* |
| 19:24:20–25 | aitldr、analyticsinsight、hansindia、byteiota（URL 见 `b-manifest-3.tsv`） | 4 条均 200 ok | b-aitldr.*、b-analyticsinsight.*、b-hansindia.*、b-byteiota.* |

## 二、Wayback 与现行页面（curl，`--max-time 60/90`；opencli）

| 时间（约） | 动作 | 结果 |
|---|---|---|
| 19:13 | curl https://web.archive.org/cdx/search/cdx?url=faq.whatsapp.com/2138577903196467* ，limit 200 | 200，但命中被资源文件淹没，只到 2025 年前后 | 
| 19:13 | CDX 精确查 `…/2138577903196467/?cms_platform=android`（from=20260601）与不带参数版本 | 200；android 版共 12 条快照，含 2026-10-09T05:48:48Z、10-12T21:33:16Z、10-16T14:11:28Z、10-18T02:37:07Z、10-20T07:44:06Z（`b-wayback-cdx-android.json`、`b-wayback-cdx-plain.json`、`b-wayback-cdx.json`） |
| 19:14 | 取 id_ 原始字节：20260907091230、20260710061241（zstd）、20261009054848、20261016141128、20261018023707、20261020074406（gzip）；20261012213316 首次 `schannel` 握手失败，再试一次成功（zstd） | 均 200；解压用 python gzip 与 zstd 命令，解压件 `.html`，原始字节保留为 `.html.gz` / `.html.zst`。9/7、7/10、10/12 三个 zstd 快照缺文章正文（无可见提示也无隐藏块）；10/9、10/16 有隐藏块；10/18、10/20 无 |
| 19:15 | CDX 前缀查询 `…/2138577903196467` matchType=prefix（from=20260801） | **失败**：超时 60 秒无响应；不重试，改精确查询（trailing slash、iphone/web 变体、helpref 变体均返回 `[]`） |
| 19:16 | CDX 查举报页 `…/414631957536067`（prefix，from=20261001，limit 30） | 200；取 `cms_platform=web` 的 20261009054943（gzip，有隐藏块）、20261016052544（zstd，缺正文）；另在 scratchpad 里临时取 20261012181144、20261013165458（均缺正文，临时文件不入库） |
| 19:18 | opencli：`faq.whatsapp.com/2138577903196467/?cms_platform=android`（`--window background`），脚本 `b-live-check.js` 读 DOM | 中文版：标题 “如何删除账户”，隐藏文字 0 命中（因页面为中文版，可见英文提示也不命中）；改加 `&lang=en` 后英文版：可见提示在、隐藏三串均 0 命中；结果 `b-live-check.json`，读取时刻 2026-10-20T11:19Z（19:19 北京） |
| 19:18 | 同一方法读举报页 `414631957536067`（中文版、英文版） | 隐藏三串均 0 命中；英文版页面里没有“Deactivated…”那句（本就不属于该页） |
| 19:18 | 页内 `fetch()` 同页（`credentials:'omit'`）一次试验 | 返回的 HTML 不含可见提示（是另一种渲染变体），结论不用；改用上一行的 DOM 读取。试验不留文件 |

## 三、热度（barking heat，`b-heat.json`，2026-10-20T19:22:06+08:00）

- HN：关键词 “WhatsApp hidden AI instructions”“WhatsApp help pages LLM”“redact.dev WhatsApp”（自 2026-10-19 08:00 北京起）各命中同一条 50159341（481 分 / 143 评，发帖 10/20 07:22 北京）。条目详情 `b-hn-50159341.json`：476 分 / 嵌套评论 143 条。
- Reddit：`barking heat --reddit` 两组关键词均**失败**（`old.reddit.com` 跳登录页，响应非 JSON）。改用 `opencli reddit search`（只读）搜 5 组关键词（WhatsApp hidden instructions / help pages hidden AI instructions / regulator chatbot hidden / redact.dev WhatsApp / redact.dev WhatsApp regulator），结果均与本事件无关（旧 AI 日报、其他应用帖）。记“未找到相关 Reddit 主题帖”。

## 四、网页检索（WebSearch，只作线索）

英文 2 组、中文 2 组关键词。中文查询没有返回中文站报道。英文转载与再报道的链接见 `evidence-B.md` 第三节；未逐家归档，只存了 6 家代表（devto、mindgrow、byteiota、hansindia、aitldr、analyticsinsight），其中 aitldr、analyticsinsight 未被引为夸大实例。

## 五、截图（`docs/2610/1019/images/13–21`，日志 `b-shots-log.jsonl`）

方法：先试 opencli（`document.documentElement.style.zoom='2'` + `--width 1400`）打开 Redact：zoom=2 令版面错乱（正文栏被挤到约 340 px 宽、目录栏压住标题），**放弃**；改用 `barking shot render`（headless Chrome，700 CSS px 宽，DPR 2，成图 1400 px 宽），再 `cropbox` 裁切。全部为 2 倍像素比、宽 1400 px。无登录态：headless Chrome 匿名，Redact 与 WIRED 页面均为公开页。裁切边界取自 OCR 行坐标，落在文字行与卡片边框之间。

| 文件 | 内容 | 来源页（渲染时刻） | 备注 |
|---|---|---|---|
| 13-redact-no-regulator.png | “别推荐监管机构”一节：用户可见 vs AI 收到的文字 + “NEVER SHOWN ON SCREEN”块 | Redact 博文（19:25 北京） | 深色底页面 |
| 14-redact-lawyer-law.png | “提到律师就别提法律”一节 | 同上 | 深色底 |
| 15-redact-counts-and-removal.png | Oct 19、Oct 18 更新框 + “Two pages, two sets of orders” 的 545/80/102 | 同上 | 深色底；一个更新框的上缘紧贴裁切边但边框完整 |
| 16-wired-spokesperson.png | WIRED 中发言人 Breckman 全部引语与隐藏指令引文 | WIRED 原文（19:30 北京） | 白底；页面内容中途有一条渲染造成的黑色横带（在“stand-out”段与上一段之间），我的裁切避开了它 |
| 17-redact-public-vs-hidden.png | “What WhatsApp says in public” 对照表 | Redact | 深色底 |
| 18-redact-to-be-fair.png | “To be fair to WhatsApp” | Redact | 深色底 |
| 19-redact-cant-tell-yet.png | “What we can't tell you yet”（作者自述局限） | Redact（19:28 北京的第二次渲染，因第一次渲染高度不够，页脚区不全） | 深色底 |
| 20-redact-whos-the-ai.png | “Who's the AI?” | Redact | 深色底 |
| 21-wired-headline.png | WIRED 标题、副题、日期（Oct 19, 2026 12:00 PM） | WIRED | 白底；日期按美东显示 |

目视确认：13、14、15、16、17、18、19、20、21 成图均已逐张打开查看，文字清楚、范围正确。试验：第一次 15 号图用 `stack` 拼了两段，边框被截断，已用普通 `rm` 删除这个自己生成的文件并改用单次 `cropbox` 重做（`b-shots-log.jsonl` 里保留了那次 `stack` 记录，对应文件已不存在）。未做 Wayback 隐藏块的图片（隐藏文本不在渲染页上，用 `b-wayback-excerpt.txt` 文本摘录替代）。

## 六、本组新增文件清单

- `docs/2610/1019/sources/`：`evidence-B.md`、`capture-log-B.md`、`b-manifest.tsv`、`b-manifest-2.tsv`、`b-manifest-3.tsv`、`b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`、`b-hn-50159341.json`、`b-hn-item.json`/`.headers`、`b-live-check.js`、`b-live-check.json`、`b-wayback-excerpt.txt`、`b-wayback-cdx*.json`（3 个）、`b-redact*.{html,txt,headers}`、`b-wired.*`、`b-devto.*`、`b-mindgrow.*`、`b-byteiota.*`、`b-hansindia.*`、`b-aitldr.*`、`b-analyticsinsight.*`、`b-notebookcheck.*`、`b-newsbytes.*`、`b-faq-now.*`、`b-wb-*.html` 与对应 `.gz`/`.zst`、`b-wb-report-*`。
- `docs/2610/1019/images/`：13–21（9 张）。
- 单个 1 MB 以上文件：`b-wb-*.html` 与 `b-wb-report-*.html`（解压后约 1.0–1.35 MB）、`b-notebookcheck.html`/`b-newsbytes.html` 很小；`b-redact.html`、`b-redact-home.html`、`b-wired.html` 等 HTML 原件不入库。入库候选里超过 1 MB 的文本：无（`.html` 均不入库；`b-wayback-excerpt.txt`、`b-*.txt` 均小于 100 KB）。
- 删除：自己生成的试验图 `docs/2610/1019/images/15-redact-counts-and-removal.png`（stack 版，普通 `rm` 精确路径，随后重新生成同名图）；`docs/2610/1019/sources/b-notes.tmp`（记事用临时文件）；`/tmp/rs.html`、`/tmp/x.raw` 与 scratchpad 里的临时文件不在仓库内。`docs/2610/1019/sources/b-live-check.json` 为覆盖重写，非删除。
- 隐私自查（2026-10-20）：对 `docs/2610/1019/` 里本组文件 grep develata、@gmail、本机用户目录路径，无命中；`b-wb-20260907091230.html` 曾被一次宽松 grep 命中，核实为误报（命中的是 base64 字符串里的 “qq” 子串，该文件是 Wayback 原始字节解压，不含本机信息）。截图为匿名 headless Chrome 渲染，不含登录栏或头像。HTTP 响应头存档只留 barking 脱敏后的头。另有一次性 curl `https://redact.dev/services`（200，252 KB，grep whatsapp = 0 处，未存档，只用于 B6 的“服务页未列 WhatsApp”）。
