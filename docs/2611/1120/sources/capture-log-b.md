# 1120 期 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区 UTC+8），取自 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json` 与命令输出。文件前缀 `b-`。未登录任何账户，未接受任何 Cookie 横幅，未提交表单，未发帖、点赞、评论、关注。`.headers` 由 `barking fetch` 去掉抓取地点与标识。opencli 浏览器扩展未连接，本组没有使用 opencli；截图用 `barking shot`（PDF 渲页用 poppler；AITOP 页用 headless Chrome，`BARKING_CHROME` 指向 Chrome 应用二进制，匿名，700 CSS px 宽、DPR 2）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 18:25:01–18:25:27 | https://arxiv.org/abs/2611.21885 、…/abs/2611.21885v1 、…/abs/2611.21885v2 、…/abs/2611.21885v3 ；https://arxiv.org/html/2611.21885 、…v1 、…v2 、…v3 ；https://arxiv.org/pdf/2611.21885 、…v1 、…v2 ；https://export.arxiv.org/api/query?id_list=2611.21885 ；HN 条目页与 Algolia 条目 50577265 | `barking fetch`（`b-fetch-list-1.tsv`） | 除 v3 外全部 ok：`b-abs*`、`b-html*`、`b-pdf*`（63 页，7.3 MB，本地不入库）、`b-api.xml`、`b-hn*`；**abs v3 与 html v3：HTTP 404（failed）**，只留下 404 页（`b-abs-v3.*`、`b-html-v3.*`）；打开 txt 确认是论文正文；`b-html`=`b-html-v2`、`b-pdf`=`b-pdf-v2` 字节相同（SHA-256 一致），因此最新版即 v2 |
| 18:29:05 | HN 搜索 “Language Models Can Rewrite Their Own Weights”“Qwen3.5-27B rewrite weights”；条目 50577265；Reddit 搜索 “Language Models Can Rewrite Their Own Weights”“Qwen3.5 self-rewriting weights” | `barking heat docs/2611/1120 b` | HN 成功（`b-heat.json`、`b-hn-50577265.json`：413 分 / 120 评，含 38 条一级评论）；**Reddit 两次均失败**：重定向到 old.reddit.com 登录页，响应非 JSON，不登录、不重试 |
| 18:29–18:30 | Exa 检索（`web_search_exa`，共 7 次：论文标题英文、中文主题、AITechArchive、thevalue.engineering、中文媒体名（机器之心/量子位/新智元）、“11%”英文与中文；中文媒体名那次没有命中这三家） | Exa MCP | 只作线索与二手稿清单；被引页面均另行 `barking fetch` 存档后核对原文（Exa 的发布时间字段只作线索，已标“未核”）。Exa 摘录的 xenospectrum 英文页因 Cloudflare 未能存档，未作证据 |
| 18:29:33–18:29:36 | https://www.jiajunfan.com/projects/self-rewriting/ 、…/publication/2026-11-17-Self-Rewriting ；https://huggingface.co/papers/2611.21885 ；aitecharchive.com 文章；xenospectrum.com/en/qwen-weights-self-editing-trials/ ；thevalue.engineering 文章；gigazine.net/gsc_news/en/…；aitop.news/event/62367 ；hub.baai.ac.cn/view/59883 ；aivio.news 文章；www.sina.cn 微博页；www.bugfree.site ；rikiyat.github.io/blog/weight-space-programming/ | `barking fetch`（`b-fetch-list-2.tsv`） | 除 xenospectrum 外 ok（`b-proj`、`b-pub`、`b-hf`、`b-m-aitecharchive`、`b-m-thevalue`、`b-m-gigazine`、`b-m-aitop`、`b-m-baai`、`b-m-aivio`、`b-m-sina`、`b-m-bugfree`、`b-m-rikiya`）；**`m-xenospectrum`：blocked，HTTP 403（Cloudflare 质询），未绕过**，无 .txt |
| 18:30 | GitHub 公开检索 API `https://api.github.com/search/repositories?q=2611.21885` 及两条关键词检索；`https://huggingface.co/api/models?search=self-rewriting`；`https://huggingface.co/api/papers/2611.21885` | curl（`--max-time 40`；GitHub 未认证） | 命中 1 个第三方仓库 `DukeCaesar/models-rewrite-own-weights`；其余 0；HF 模型检索返回空数组；HF papers API 的 `githubRepo` 为 null，`projectPage` 指向作者项目页（命令输出，未存档） |
| 18:30:04 | `api.github.com/repos/DukeCaesar/models-rewrite-own-weights`、其 commits（5 条）与 `raw.githubusercontent.com/…/README.md`；上面的 GitHub 检索结果 | `barking fetch`（`b-fetch-list-3.tsv`） | ok（`b-gh-dc-*`、`b-gh-search.json`）：创建 2026-11-19 14:14:34Z，唯一提交 14:15:01Z，README 自述“numerical replication is not yet verified” |
| 18:30:30 | https://aitecharchive.com/articles/weight-space-programming-qwen-weight-edit-audit ；https://arxiv.org/abs/2611.08070 | `barking fetch`（`b-fetch-list-4.tsv`） | ok（`b-m-aita-audit.*`、`b-abs-2611-08070.*`） |
| 18:30:57 | https://www.ai-club.cn/news-article/65059 ；https://zicq.com/articles/…zh.html ；https://aiproducthub.cn/newsflash/language-models-can-rewrite-their-own-weights-rsi/ | `barking fetch`（`b-fetch-list-5.tsv`） | 全部 ok（`b-m-aiclub`、`b-m-zicq`、`b-m-aiproducthub`；aiproducthub 只作线索，未引用） |
| 18:38:27 | https://api.fxtwitter.com/status/2123828039276773655（AITOP 页引用的 X 帖，为核对“Stanford”措辞来源） | `barking fetch`（`b-fetch-list-6.tsv`） | ok，`b-fx-rohan.json`（公开 JSON，不含抓取者信息；第三方帖主为公开账号，仅作线索，不截图、不入正文） |
| 18:2x–18:3x | 无 | 本地 `diff`、`grep`、`pdftotext` | v1/v2 文本逐行比较；PDF 逐页检索图表所在页；论文数字与引文均从 `b-html.txt`、`b-pdf.txt` 核对 |

被拒或失败：arXiv v3（abs、html，404）；xenospectrum（403）；Reddit（登录重定向，两次）。HN 评论分数不公开，未取得。

## 截图

均匿名，已逐张打开确认文字清楚、范围正确。图 13–23 来自论文 v2 PDF（`b-pdf.pdf`，63 页），先用 `barking shot pdf` 以 160 dpi 渲成 1360×1760 的整页，再用 `cropbox`/`stack` 裁切，裁切边界落在文字行或图表框之间；裁出宽 1065 px，不超过 1400 px。图 24 为浅色底网页（AITOP），headless Chrome 渲染 1400 px 宽、DPR 2。PDF 白底，无深色页。

| 文件 | 来源 | 工具与说明 |
|---|---|---|
| `13-abstract.png`（1065×1040） | PDF p1 | cropbox；标题、11 位作者与 6 个单位上标、完整摘要框含 “Date: November 19, 2026” |
| `14-fig1-eight-sessions.png`（1065×730） | PDF p2 Fig. 1 | cropbox；三联图（57.2→+0.5→+4.0…→+6.6，被截断答案占比，同种子示例）与完整图注（“each session itself starts from the base weights”） |
| `15-tab8-first-protocol-eval.png`（1065×794） | PDF p36 Tab. 8 + p35 | stack 两段：Tab. 8 含表注（57.2 58.3 +1.1* … 0.038）；App. D.3 “The evaluation fixed in advance.” 全段，含末句“The edits of the second protocol are evaluated on the same 490 problems … 6.6 and 4.0” |
| `16-fig26-repair-text-33pct.png`（1065×545） | PDF p46 Fig. 26 | cropbox；三条柱（58.5/0.0/33.0）与图注（“about 56% of the base model’s 58.5%”） |
| `17-tab12-image-repair.png`（1065×1065） | PDF p52 Tab. 12 | cropbox；E.6 标题、Tab. 12 含表注与全部六榜行与合计行、其后两段（含 “repair was found and tested on text math problems only”“83% of the base model’s 84.3%”） |
| `18-fig12-repair-session.png`（1065×1175） | PDF p12 Fig. 12 | cropbox；八张卡片（1397× outlier；“Clipped values exceeding 15× the median”）与完整图注 |
| `19-tab2-scale-and-repair.png`（1065×430） | PDF p16 Tab. 2 | cropbox；表注与 2B/4B/9B/27B 全部行（own task、held-out、damaged copy） |
| `20-appendixA-discussion.png`（1065×1255） | PDF p26 附录 A | cropbox；整页三段（Why we also test on a damaged copy／What scale adds／Future directions）；**论文没有 Limitations 节，用本页代替候选图 20** |
| `21-tab13-second-protocol.png`（1065×1345） | PDF p62 E.9 + Tab. 13 | cropbox；E.9 两段、Tab. 13 含表注与八个会话全部行（+0.5/p=0.25、+4.0、+6.6、五次撤回） |
| `22-tab1-found-changed.png`（1065×629） | PDF p14 | stack 两段：§3.4 首段（“caps the random values rather than restoring the original ones”）；Tab. 1（含 “the named matrix alone: 55.0%”） |
| `23-repair-caps-random-values.png`（1065×782） | PDF p44–45 | stack 三段：E.3 “Only the largest model repairs a damaged copy of itself” 段（规则、20 个矩阵、0.013%、56%）；其续句 “The repair does not restore the original weights, which the model never reads: its rule caps the random values”；“The repair writes values of its own, not the original ones” 段（余弦 0.002、范数 3.85/825/39.7） |
| `24-aitop-stanford-headline.png`（1344×1127） | https://aitop.news/event/62367 | `render` + cropbox；标题“Stanford 论文：…”、摘要段、“Current read”与“Last updated 2026/11/20 17:17:18”；已裁去顶栏与导航；页面浅色；无登录信息 |

## 待删除文件（本组留下的试验/废弃/重复文件，请 Claude 收尾时一次删除；`rm` 未调用）

- `docs/2611/1120/images/23-repair-not-original-weights.png`（第一版，下沿切到半行，已由 `23-repair-caps-random-values.png` 取代）
- `docs/2611/1120/sources/b-pdf-v2.pdf`、`b-pdf-v2.headers`、`b-pdf-v2.txt`（与 `b-pdf.*` 字节相同）
- `docs/2611/1120/sources/b-html-v2.html`、`b-html-v2.headers`、`b-html-v2.txt`（与 `b-html.*` 字节相同）
- `docs/2611/1120/sources/b-m-xenospectrum.headers`、`b-m-xenospectrum.html`（403 质询页）
- `docs/2611/1120/sources/b-abs-v3.headers`、`b-abs-v3.html`、`b-html-v3.headers`、`b-html-v3.html`（v3 的 404 页）

## 体量与泄露自查

- 单个超过 1 MB 的文件（均为 PDF，不入库）：`sources/b-pdf.pdf`（6.9 MB）、`b-pdf-v1.pdf`、`b-pdf-v2.pdf`（后者待删）。HTML 原件也不入库（`b-html*.html` 约 0.86 MB，`b-proj.html` 等小于 1 MB）。
- 抓取者信息：`grep` 本组文件无本机用户名与本机绝对路径；未存任何 X 页面；`.headers` 已由 barking 去标识。
