# A 组抓取日志（1027 期，Google Genie 4）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）；本机美东（EDT），北京 = 本机 + 12 小时（`date` 显示 08:28 EDT 即 20:28 北京）。机器记录另见 `a-fetch-log.jsonl`（每条含 time_bj、sha256）、`a-shots-log.jsonl`、`a-heat.json`。所有抓取未登录、未发帖；curl 与 barking 不带 Cookie；opencli 浏览器只用于打开公开页面。HTML 原件只存本地。

## 一、barking fetch 批量存档（`a-<name>.html|.txt|.headers`）

清单文件 `a-list1.tsv`–`a-list4.tsv`。

| 时间 | name | URL | 结果 |
|---|---|---|---|
| 20:29 | blog | https://deepmind.google/blog/genie-4-a-new-generation-of-world-simulation/ | ok（226 KB，857 行；正文完整；JSON-LD 有 datePublished/dateModified） |
| 20:29 | model-page | https://deepmind.google/models/genie/ | ok（含 Limitations 与 Disclaimer；当前该 URL 已是 Genie 4 页） |
| 20:29 | genie3-blog | https://deepmind.google/blog/genie-3-a-new-frontier-for-world-models/ | ok（August 5, 2025） |
| 20:31 | tc / ars / cb / mlq / digg | TechCrunch、Ars Technica、Crypto Briefing、MLQ、Digg 聚合页 | ok（Digg 与 MLQ 只作线索，证据表未直接引用其数字） |
| 20:32 | cls / yicai / pedaily / ifanr / heise / verge / startupfortune / googleblog / projgenie / yt | 见 evidence-a.md | ok（yt 的 txt 仅 7 行，视频说明取自 HTML 的 shortDescription） |
| 20:32 | bloomberg | https://www.bloomberg.com/news/articles/2026-10-27/google-upgrades-ai-world-model-that-rattled-video-game-stocks | HTTP 200 但只有前两段（付费墙），未绕过；全文改读 Bloomberg Línea 西语版 |
| 20:32 | threerouter | https://www.threerouter.com/blog/210 | **blocked**（SPA，正文过短）；改用 opencli 读取 `article` 的 innerText，存 `a-media-threerouter-article.json`（仅该篇文章文本） |
| 20:33 | zeitgraph / llmsblog / techmeme / blm-linea | graph.indiethinkers.com、llms.blog、techmeme.com、bloomberglinea.com | ok（前三者只用来找 X 帖子链接；blm-linea 为 Bloomberg 稿西语版） |

## 二、个别请求（curl，`--max-time 60`）

| 时间 | URL | 结果 |
|---|---|---|
| 20:30 | https://hn.algolia.com/api/v1/items/50261319 | ok，存 `a-hn-item-genie4.json`（含全部评论；HN 不提供评论分数，只能按树顺序） |
| 20:3x | https://api.fxtwitter.com/status/<id>：2115114028338962672（@GoogleDeepMind）、2115232001275924722（Hassabis）、2115129174826647863（Kilpatrick）、2115141038289682582（Kulkarni）、2115119959839756613（Fruchter）、2115123325341204567（Parker-Holder）、2115129573016998365（Hadsell）、2115115252690145519（Pichai）、2115115017913999585（Holynski）、2115120818116334084 / 2115127527241085010 / 2115273222061015169（Mollick）、2115114238943568324（Moore） | 全部 ok，存 `a-fx-<id>.json`（公开 JSON；作者均为公开的机构账号或公开职业身份的人，无抓取者信息） |
| 20:5x | https://deepmind.google/models/model-cards/genie-4/ （猜测 URL，用于找 model card） | 404；https://modelcards.withgoogle.com/ 200（未深入，首页无 Genie 4 条目可核） |
| 20:5x | https://www.threerouter.com/blog/210 （curl） | 200 但是 SPA 外壳，无正文 |

## 三、barking heat（20:30:43）

`barking heat docs/2610/1027 a --hn "Genie 4" --reddit "Genie 4"`：HN 命中 3 条，主帖 50261319 为 927 分 / 536 评（00:06:01 北京）；Reddit 搜索被重定向到登录页（`old.reddit.com/login`），失败，已记入 `a-heat.json` errors，未登录、未重试。HN 页面 20:4x 渲染显示 934 分 / 538 评。

## 四、WebSearch 与 Exa（只作线索，搜索摘要不作来源）

WebSearch 约 5 次（前 3 次的检索结果还未收录 Genie 4，只找到 Genie 3 与 Project Genie 的旧文，其结论“官方没有 Genie 4”是搜索索引滞后，已被官方页面存档推翻，未采用）；Exa 3 次（找到 Bloomberg、Bloomberg Línea、财联社、第一财经、机器之心转载、爱范儿、heise、Startup Fortune、tech-insider、ThreeRouter 等线索，凡引用者均已 fetch 存档再核）。

## 五、截图（opencli 与 barking shot）

会话名 `a-shot`、`a-shot2`、`a-shot3`、`a-shot4`（后台窗口 `--window background`，均未触发 `captureScreenshot` 超时，一次 `--full-page` 在模型页超时，见下）。

| 时间 | 页面 | 动作与结果 |
|---|---|---|
| 20:35 | DeepMind 博文 | opencli `zoom=2` 全页截图（1400×38560）：版面与滚动不对应，无法按锚点裁切；不再使用，文件在 scratchpad，未入库 |
| 20:38 | DeepMind 博文 | 改用 `barking shot render`（headless Chrome，700 CSS px，DPR 2，`--height 12000`，实得 1400×24000，完整含 Related posts 之前全部内容）；裁出 01–05 |
| 20:40 | Genie 3 博文 | 同法 render（1400×24000）；01 级版面（日期、标题、作者）与“24 frames per second … 720p”段 `stack` 拼成 06；Limitations 区裁出 07 |
| 20:41 | TechCrunch | render（1400×12000）裁出 09（标题、作者与时间 “9:30 AM PDT · October 27, 2026”，到“two minutes”段止） |
| 20:42 | The Verge | render（1400×10000）；标题块（含作者与 “Oct 27, 2026, 12:00 PM EDT”）与“about two minutes long”段 `stack` 拼成 10，拼接缝用白色（`--bg #ffffff`）；cookie 弹窗未入图 |
| 20:47 | 模型页 Limitations | 无头渲染空白（滚动驱动深色页，渲染 2 次均无内容）；opencli 打开后页面 `scrollY` 被锁在 715，无法滚到 Limitations。改为：用 `eval` 把该区块（含全部六项，文字原样）克隆进一个白底浮层（宽 690 CSS px，`zoom:2`，隐藏翻译插件图标），再按 1400×2400 视口截图，裁出 08。**这是 DOM 克隆，非页面原貌**（原页底色为深色、三列排版保持），文字未改，已在 evidence-a.md 注明。 |
| 20:48 | Hacker News 帖 | render（1400×2000）；帖卡（标题、分数、评论数）与页面最靠前评论 `stack` 拼成 11；页面为未登录态，右上角是 “login” 字样，无抓取者信息 |
| 20:51 | 投资界（机器之心稿） | render（1400×6000）裁左栏标题块出 12（标题、导语、作者/来源/更新时间） |

截图与存档均不含抓取者信息：截图全为未登录公开页或克隆出的公开文字；`fx` JSON 为公开账号；对 `sources/a-*`、`images/` grep 抓取者用户名、本机路径、“develata”均无命中。

`a-shots-log.jsonl` 的 `06-genie3-blog-duration.png`、`10-verge-two-minutes.png` 各有两行：首版（06 在“Try Project Genie”按钮处切断，10 拼接缝为薄荷绿）已移到 scratchpad，以 `06b`/`10b` 重做后改回正式文件名；日志里 `06b-…`、`10b-…` 两行对应现在的 06、10，请 Claude 合并时按此对应。

## 六、待删除文件（精确路径）

仓库内无需删除的试验文件。以下在 scratchpad（会话临时目录，不入库）：`a-blog-full.png`、`a-old06.png`、`a-old10.png`、`a-m1.png`–`a-m8.png`、`a-v1.png`、`a-mfull.png`（未生成）及各 `a-p*.png`、`a-q*.png`、`a-sheet*.png`；位于 `%TEMP%\barking-shot\26101027-a\` 的 render 中间图（blogr、g3、tc、verge、hn、ped、three、model、model2）。

仓库内的废弃/低价值存档（可留可删，由 Claude 决定）：`docs/2610/1027/sources/a-threerouter.html` 与 `a-threerouter.headers`（blocked 页外壳）；`a-digg.*`、`a-techmeme.*`、`a-zeitgraph.*`、`a-llmsblog.*`（找 X 帖链接用的聚合页）；`a-yt.html`（1.3 MB，HTML 原件只存本地，不入库）。HTML 文件均按规则只存本地。

## 七、体积

文本存档单个均小于 1 MB（最大 `a-hn-item-genie4.json` 约数百 KB）；HTML 超 1 MB 的仅 `a-yt.html`，不入库。
