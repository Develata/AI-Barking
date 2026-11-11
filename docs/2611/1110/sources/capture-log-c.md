# C 组 capture-log（北京时间，UTC+8，本机时间）

执行方 claude-sonnet-5-5；时段 2026-11-11 14:30–14:55。工具：`barking fetch/html2txt/heat/shot render|lines|cropbox|pdf`（封装脚本 `c-bk.sh` 只负责在 macOS 上设 BARKING_CHROME 后调用已编译的 barking）、headless Chrome `--dump-dom`（渲染 JS 页）、curl（fxtwitter API）、WebSearch（只作线索，未存档）。opencli 扩展未连接，未使用。全程未登录、未接受 Cookie、未发帖、未点赞。机器日志：`c-fetch-log.jsonl`、`c-shots-log.jsonl`、`c-heat.json`。

| 时间 | URL（已去查询参数） | 工具 | 结果 |
|---|---|---|---|
| 14:30:55 | blog.google Carbon 博文；AA MKA 文；AA changelog；AA 模型页 Carbon、Fable 5.2；AA 榜单；ai.google.dev 的 pricing、models、changelog、models/gemini-4-carbon、rate-limits；HN 50442303（清单 c-list1.tsv） | barking fetch | 7 个 ok；AA `/evaluations/mka`（路径写错）与 `/articles/gemini-4-carbon` 404 failed；ai.google.dev 五页 200 但正文只有 JS 外壳，blocked |
| 14:31:39 | AA MKA 榜单页 `/evaluations/mka`、aistudio.google.com 三页、Carbon 模型卡 PDF、1028 博文、cursorbench、wandr、zapier、antigravity 博文、AA methodology（c-list2.tsv，`--force`） | barking fetch | aa-mka-board、modelcard、blog-1028、cursorbench、wandr、zapier、antigravity、aa-methodology ok；aistudio 三页 blocked（JS 外壳） |
| 14:3x | ai.google.dev 的 pricing、models、changelog、models/gemini-4-carbon、rate-limits，aistudio.google.com 的 whats-new-gemini-4-carbon 与 pricing | headless Chrome `--dump-dom`（临时 profile，无登录态）+ barking html2txt | ok，`c-*-rendered.html/.txt`；定价页无“最后更新”日期 |
| 14:35:58 | Engadget、9to5Google、Blockonomi、Seeking Alpha、blog.google 开发者博文、gemini.google 的 release-notes 与 subscriptions（c-list3.tsv） | barking fetch | Engadget、9to5Google、Blockonomi、开发者博文、release-notes、subscriptions ok；Seeking Alpha 403 blocked，不再重试 |
| 14:36 | x.com/sundarpichai/status/2120185896469496180 | curl `api.fxtwitter.com/status/<id>`（id 取自 Blockonomi 页内嵌入） | ok，`c-fx-2120185896469496180.json` |
| 14:36 | x.com/Google/status/2120186185423159331、x.com/GoogleAIStudio/status/2120187576489222302（id 由 WebSearch 得到） | 同上 | ok，`c-fx-2120186185423159331.json`、`c-fx-2120187576489222302.json` |
| 14:36:41 | Android Authority、Digital Trends、BigGo、OrcaRouter、Technobezz（c-list4.tsv） | barking fetch | 四个 ok；Technobezz 403 Cloudflare 质询，blocked，不绕过、不重试 |
| 14:37 | blog.google MKA 博文、Hugging Face MKA 数据集页、AA methodology（c-list5.tsv） | barking fetch | 3 个 ok（c-aa-mka-method 与 c-aa-methodology 是同一页的重复存档） |
| 14:37:32 | storage.googleapis.com/deepmind-media/papers/mka_tech_paper.pdf（c-list6.tsv） | barking fetch | ok，25 页，`c-mka-paper.pdf/.txt` |
| 14:38 | HN 50442303 与搜索“Gemini 4 Carbon” | `barking heat --item 50442303 --hn "Gemini 4 Carbon" --reddit "Gemini 4 Carbon"` | HN ok；Reddit 搜索被重定向登录页，**失败**，已记入 c-heat.json errors |
| 14:38 | x.com/GoogleAI/status/2120187753329791341（id 取自 HN 搜索结果） | curl fxtwitter | ok，`c-fx-2120187753329791341.json` |
| 14:38:25 | 新浪、硬是要學、电脑王阿达、DataLearner（c-list7.tsv） | barking fetch | 4 个 ok（DataLearner 未引用） |
| 14:3x | github.com/perplexityai/wandr 与 raw README（c-list8.tsv） | barking fetch | ok；未查到 Carbon 成绩行 |
| 14:3x–14:48 | 截图 25–36，见下 | `c-bk.sh shot render/lines/cropbox/pdf` | 12 张；中间图在系统临时目录 `…/T/barking-shot/26111110-c/`（不在仓库） |
| 14:5x | Zapier Carbon 报告页、Cursor 模型价格页（c-list9.tsv） | barking fetch | 2 个 ok；另读 c-blog-dev.txt（开发者博文，无新增价格或开放范围信息，只重复“paid tier … Ultra”） |
| 14:5x | 两次中文搜索（WebSearch，只作线索，未存档）与一次 HN 评论二次扫描 | WebSearch；python 读 c-hn-50442303.json | 未命中 IT之家、36氪等简体站；HN 评论补充见 evidence-c.md |
| 14:4x | blog.google Carbon 博文脚注 1（折叠在文末 “More Information”） | headless 渲染；另做本地副本 `c-blog-footnote-render.html`（插入点击脚本，两次均未展开） | **失败**：折叠区没展开，未截；脚注原句仅存在于 `c-blog.txt`。同一来源渲染时，`--height` 过大（11000 以上）时第二次起返回 “Error: Server Error”，改用 height 9400 / 10300 成功 |

## 截图渲染参数（`c-shots-log.jsonl` 有机器记录）

- 25、26、27、30、31、33、34、36：700 CSS px 宽、DPR 2（1400 px 宽）。
- 28、29、35：920 CSS px、DPR 1.5（1380 px 宽）。原因：AA 榜单柱图与 Zapier 表格在 700 px 版式下被截断（最右侧 Fable 5.2 的柱和 Cost 列看不到），“官方图表截全”优先于 2 倍像素比；1380 px 缩到 1080 仍大于输出宽度。28 已裁掉页面右下角“切换到中文”弹窗。
- 32：PDF 第 11 页，poppler 165 dpi（1365 px 宽），裁掉页眉页脚外的空白。
- 全部为浅色页面；AA 榜单、Zapier、PDF 页没有头像、昵称或登录栏；截图前后均无登录态，没有 X 页面截图（X 只取 fxtwitter JSON，未截）。

## 流程中被我移走、不在仓库里的试验图

为判断版面，我另截了几张中间图后用 `mv` 移到会话临时目录（不是 `rm`）：`31-blog-internal-bench.png` 第一版（底边切到下一条），`28-aa-mka-cost.png` 第一版（含弹窗）与 `30-aa-intelligence-index-board.png` 第一版（底行被切），以及若干 `scratch-*.png`。机器日志 `c-shots-log.jsonl` 里仍有它们的记录行，对应文件不在 `images/`。

## 自查（交付前）

- 对 `docs/2611/1110/` 本组文件 grep 抓取者信息（本机用户名、账号、Cookie）无命中（见回报）；fxtwitter JSON 只含 Google、Sundar Pichai、@GoogleAIStudio、@GoogleAI 的公开帖与公开账号信息，没有抓取者信息；.headers 由 barking 自动裁剪。
- 25–36 均已逐张打开目视确认文字清楚、范围正确。

## 脚本与待处理说明

- `c-bk.sh`：封装 barking（设 BARKING_CHROME），路径相对脚本自身，不含用户名路径。
- `c-aa-mka-extract.py`：从 AA 榜单页的内嵌 JSON 抽 MKA 字段的试验脚本，输出行错位（模型记录与创建者记录交错），只用来确认载荷里有 Carbon 的 price1mInputTokens=4、price1mOutputTokens=20、cacheHitPrice=0.2 与 climbWsr；成本数字不在 HTML 里（浏览器内计算），所以 $6.29 与 $31.98 取自截图 28。证据只依赖这些字段与截图，不依赖该脚本输出，列入待删除。
- 价格推算用了一次性 python 命令（不入库），算式与输入写在 evidence-c.md 的 C3 小节。
