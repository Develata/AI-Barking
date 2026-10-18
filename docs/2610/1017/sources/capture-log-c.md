# C 组取证日志（Meta 商业条款）

**C1/C2 结论（2026-10-18 19:44 北京时间读取，匿名 headless Chrome）：能独立复现。** `https://www.facebook.com/legal/commercial_terms_preview?locale=en_US` 的爬取授权段不含 "train"（以 "research, and maintain advertising and business products … and maintain the integrity and security of Meta Company Products" 结尾）；同一页 `?locale=en_GB` 的同一段仍写 "Meta may use any content from such domains, web pages or URLs to provide, develop, improve, research, train and maintain the integrity of, the Meta Company Products, services and features …"；两版生效日均为 2026-10-30，页面未写哪个版本适用于谁。约 20:33 复读，结果不变。额外：fr_FR、zh_CN 版仍含 train/entraîner/训练。

时间说明：北京时间 = 本机美东 + 12 小时。本机时钟在约 20:35 北京之后出现过约 1 小时的跳变（`c-dump.sh` 打印的时刻与文件时间戳不一致）；20:33 之前的时刻以 barking 工具日志与文件时间戳为准。

## 工具与通用做法

- Meta 页面：`curl` 直连 facebook.com 返回 HTTP 400（空壳错误页，c-meta-ct-en_US-curl.*、c-meta-ct-en_GB-curl.*，保留作失败记录）。改用匿名 headless Chrome：`c-dump.sh`（本目录，全新 user-data-dir、无 Cookie、不登录，`--dump-dom`；后期加 `--lang=en-US --accept-lang=en-US`），再 `barking html2txt` 转文本。页面无需登录即可看全文。
- 截图：`barking shot render`（headless Chrome，700 CSS px、DPR 2）→ `lines` 找锚点 → `crop/cropbox/stack`，每次动作记入 `c-shots-log.jsonl`。四张图均已逐张打开目视确认：文字清楚、范围含完整的爬取授权段，边界落在文字行间。
- 批量存档：`barking fetch`，清单 c-manifest-1.tsv … c-manifest-4.tsv，结果见 `c-fetch-log.jsonl`。
- 未登录、未用 Cookie、未发帖。opencli 仅用于：X 嵌入页试截（见下）、`opencli reddit search`（只读）。

## 抓取记录

| 北京时间 | 对象 | URL | 工具 | 结果 |
|---|---|---|---|---|
| 19:38 | Elsop 文章 | https://www.elsop.com/meta-commercial-terms-train-removed/ | curl | 200，c-elsop.html/.txt |
| 19:39 | Meta 预览 US/GB | …/legal/commercial_terms_preview?locale=en_US、en_GB | curl（UA 为普通浏览器） | HTTP 400 空壳，失败，改 headless Chrome |
| 19:41 / 19:44 | Meta 预览 US / GB | 同上 | 匿名 headless Chrome dump-dom | 成功：c-meta-ct-en_US-dom.*、c-meta-ct-en_GB-dom.*；US 无 train，GB 有 1 处 |
| 19:46–19:48 | Wayback 10/6、10/17（US、GB）、自助广告条款预览 10/17（US、GB）、The Verge、Nieman Lab、ppc.land、Meta 爬虫页 | 见 c-manifest-1.tsv | barking fetch | 7 ok；Nieman Lab 403（blocked）；爬虫页 400（failed）。Wayback 用 raw（id_）取文本 |
| 19:48 | Wayback CDX 索引 | https://web.archive.org/cdx/search/cdx?url=facebook.com/legal/…_preview（prefix，from 20261001） | curl | 成功，c-wayback-cdx-ct.txt、c-wayback-cdx-ssat.txt；只列到 10/6（10/17 带 locale 的快照未被索引，但可取） |
| 19:48 | Wayback 10/4、10/5 快照（商业条款 ×2、自助广告条款 ×2） | 见 c-manifest-2.tsv | barking fetch | 4 ok，均含 train |
| 19:49 | Nieman Lab | …niemanlab.org/2026/10/metas-new-commercial-terms-… | curl（普通浏览器 UA，一次性） | 200，c-nieman.html/.txt；原因：barking fetch 的 UA 被 403 |
| 19:50 | Andy Stone 帖 | https://api.fxtwitter.com/status/2111101865769201850 | curl | 200，c-fx-stone-2111101865769201850.raw.json（无抓取者信息） |
| 19:47–19:50 | Verge 前篇 /tech/1012846 | 见 c-manifest-3.tsv | barking fetch | ok，c-verge-first.*（未细读） |
| 19:50–19:56 | 自助广告条款预览 US/GB（实时）、现行自助广告条款 US、现行商业条款 US/GB、预览 en_AU/fr_FR/zh_CN、Meta 商业帮助中心说明页 | facebook.com/legal/…；/business/help/2481113349080785 | c-dump.sh | 全部成功；en_AU 回落中文（非受支持语言） |
| 19:58–20:05 | 截图：ct-us、ct-gb、wb-ct-1006（Wayback 带工具栏）、ssat-current | 同上 | barking shot render | 首次同时开 4 个渲染时 ssat-current 渲染空白（图不入库），单独重渲成功；第一次 render ct-us 因我加的 `timeout` 被杀，留下 .rendering 中间图（在系统临时目录，不入库） |
| 20:05–20:08 | 图 25、26、27、28 | — | barking shot crop/cropbox/stack | 成功。27 第一次裁切切掉了 Wayback 工具栏上沿，已删除（`rm docs/2610/1017/images/27-meta-ct-wayback-1006-crawl.png`，我自己生成的试验文件）后重裁；28 第一次中段下沿切到下一条 “7.” 的半行，同样 rm 后重做 |
| 20:09 | Stone 帖 X 嵌入页 | https://platform.twitter.com/embed/Tweet.html?id=2111101865769201850 | barking shot render | 空白图（已删临时 png，不入库） |
| 20:10 | 同上 | 同上、及 https://x.com/andymstone/status/2111101865769201850 | opencli browser（session c-shot，已关闭） | 嵌入卡片正文被截断（“Show more”）；x.com 页面为已登录态并被自动翻译成中文，不作为证据，不存档、不截图。结论：不做 X 卡片截图 |
| 20:11 | Meta 爬虫页（英文） | https://developers.facebook.com/docs/sharing/webmasters/web-crawlers/ | c-dump.sh（先无 --lang 得中文 c-meta-crawlers-zhCN.*；加 --lang=en-US 得英文 c-meta-crawlers-en.*） | 成功 |
| 20:22 | HN/Reddit 热度 | HN 50111852、50132627；搜索 “Meta commercial terms” | barking heat | HN 成功（c-heat.json、c-hn-*.json）；Reddit 搜索被登录页拦截，记入 c-heat.json 的 errors |
| 20:27 | Reddit 搜索 | `opencli reddit search "Meta terms scrape news sites"` | opencli（只读） | 成功，c-reddit-search-1.json（15 条，仅前 6 条相关）；第二个查询无输出，空文件已 rm |
| 20:2x–20:3x | 夸大报道 | tnw、gizmodo、techjuice、entarabi、cryptobriefing；fourweekmba | barking fetch（c-manifest-4.tsv） | 前 5 个 ok；fourweekmba 403，未存档。同时用 WebSearch 找标题（只用标题与链接，引句取自存档） |
| ~20:33 | 复读 US/GB 预览 | …/commercial_terms_preview?locale=en_US、en_GB | c-dump.sh | 成功，c-meta-ct-recheck-en_US/GB.*；US train=0，GB train=1，与首次一致 |

## 删除过的文件（均为我本次生成）

- images/27-meta-ct-wayback-1006-crawl.png 与 images/28-meta-ssat-current-research-training.png 各一次（裁切边界不合格后重做，已重新生成）。
- sources/c-reddit-search-2.json（空文件）、c-dump-*.out 与 c-dump-r1/r2.out（c-dump.sh 的临时输出）。
- 系统临时目录里的 stone.png 与空白 ssat-current.png（不在仓库内）。

## 自查

- 在 docs/2610/1017/ 的 c- 文件中 grep develata、gmail、本机用户名：只命中 Facebook 页面脚本里自带的域名列表（"gmail.com" 出现在站点白名单），无抓取者信息；fxtwitter JSON 只含 Stone 的公开信息。Reddit JSON 含公开发帖者用户名，仅作热度记录。
- `barking fetch` 的 `.headers` 已由工具省略多数响应头；dump-dom 的 HTML 来自匿名无 Cookie 会话。
- 单个文件超过 1 MB 的：无（最大 c-meta-crawlers-en.html 约 0.67 MB）；html/png 按规则只存本地。c- 前缀文件总计约 15 MB（含 html），其中 HTML 原件不入库。

## 未完成/未做

- X 帖卡片截图（原因见上）。
- 其余语言版（日、韩、德、西等）是否保留 train 未查；仅查了 fr_FR、zh_CN。
- Wayback 10/6 与 10/17 之间没有更细的快照，US 版删除的具体时刻无法确定。
- Verge 前篇 /tech/1012846 已存档未读；FourWeekMBA 页面被 403，未取。
- 未找到“Meta 放弃训练”类更夸张的中文标题（见 evidence-c.md 第三节）。
