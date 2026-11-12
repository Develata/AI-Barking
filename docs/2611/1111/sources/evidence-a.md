# 1111 期 A 组取证：48个AI智能体“发现”玛雅遗址与作者更正

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。取证时刻 2026-11-12 17:52–18:20（北京时间，本机 UTC+8）。时间一律北京时间，括号内为原时区。文件前缀 `a-`，存档在 `docs/2611/1111/sources/`，截图在 `docs/2611/1111/images/`（01–12）。HTML/图片原件只存本地，文本存档（txt/json）可入库，均小于 1 MB（最大 `a-hn-main.html` 536 KB 属 HTML 原件不入库；`a-wb-hn-20261112065215.html` 529 KB 同）。

## 一、证据表

| # | 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|---|
| A1a | 作者 11/11 更新段：已在 Estrada-Belli et al. 2025 测绘，智能体漏读文献，是重新发现 | L1（作者本人博客） | https://pietromiotti.com/latest/maya-site/ （存档 `a-blog.html/.txt`） | "Update (11 November): it turns out the site had already been mapped in Estrada-Belli et al. 2025. The agents missed it while going through the literature, so they rediscovered it rather than discovered it (it just wasn't in the register of Mexico's National Institute of Anthropology and History, INAH, yet). Still work to do then! The rest of the story still holds." | 01-blog-title-update.png | 页面标题已改为 “My AI Agents (Re)discovered a Maya Site in 72 Hours”，题下 “5 min read (Last edit: 11 Nov)”。响应头 Last-Modified：2026-11-12 06:00:54（北京；Wed, 11 Nov 2026 22:00:54 GMT），这是页面**最后一次**改动，不是更新段首次出现的时刻。更新段首次出现时刻官方未给：Wayback 2026-11-11 23:56:04（北京；15:56:04 UTC）快照仍无更新段、标题仍为 “Found an Unmapped Maya Site”，但引言里 NASA 数据集一句已改；作者 X 更正帖 2026-11-12 00:39:47（北京；11-11 16:39:47 UTC）、HN 更正评论 00:53:16（北京；16:53:16 UTC）。故更新段上线不早于 23:56、一般不晚于 00:53（后者是推断：作者自己的更正帖/评论出现的时刻） | 已找到 |
| A1b | 更新前版本措辞（Wayback） | L1（存档；只用于“当时页面写了什么”） | https://web.archive.org/web/20261110201337/https://pietromiotti.com/latest/maya-site/ （存档 `a-wb-blog-20261110.html/.txt`，取 id_ 原始版） | 标题 "My AI Agents Found an Unmapped Maya Site"；引言 "my AI agents discovered a Maya site in Mexico that, as far as anyone can tell, no one had reported before."；INAH 回信后 "To be clear, it's not official yet. But as far as anyone can tell, nobody had put this one on a map before."；文献段 "others used lidar to estimate how many people lived in the Maya lowlands (Estrada-Belli et al. 2025)"；NASA 数据句 "found that NASA has collected a lot of lidar data in a huge open dataset"、"That night, I created a token on the NASA portal, asked the agents to analyze the images overnight, and went to bed."；结尾 "In the report I offered to help with a field check led by INAH. … so it's now up to the local INAH centre to decide whether to send a team." | 07-wayback-original-intro-full.png（标题+引言整段、“To be clear”、文献段三段拼接） | 快照时刻 2026-11-11 04:13:37（北京；11-10 20:13:37 UTC），即作者 X 帖（11-11 02:19:51 北京）后约两小时。该快照里 Estrada-Belli 2025 **已在文末“进一步阅读”里被引用**，描述为 “used lidar to estimate how many people lived in the Maya lowlands”；更新后改成 “mapped settlements across the Maya lowlands, including this very site”。对比 diff 见 `a-wb-blog-20261110.txt` vs `a-blog.txt`。Wayback 渲染图字体为后备字体（`if_` 模式未加载网页字体），文字未改。第二个快照 `a-wb-blog-20261111.*`（11-11 23:56:04 北京）为中间态：标题与引言未改，但 NASA 数据集/token 句已改成 “Their region is much better covered by public lidar.” / “That night, I asked the agents to analyze it and went to bed.” | 已找到 |
| A2a | 48 个智能体 | L1 | 同 A1a | "48 agents went through them one by one like very patient archaeologists, looking for pyramids, platforms, and plazas while filtering out natural hills, sinkholes, and roads. There was no special model or classifier involved: the agents literally looked at the pictures, following a precise guide." | 02-blog-48-agents.png | 原文**没有**写这 48 个智能体用的是哪家模型、什么框架、是否雇主提供，也没写它们跑了多久。仅写：头脑风暴时 “a few agents (Gemini, ChatGPT, Claude and Grok)”，Claude 建议翻成西班牙语（“at least, that's what Claude suggested”），Gemini 生成示意图（“Generated with Gemini from the lidar layout”）。“72 hours” 的原文口径是 “from a random thought while traveling to an official reply from the Mexican government”（10-18 起意 → 10-21 INAH 回信），不是 48 个智能体跑 72 小时 | 已找到 |
| A2b | 约 1200 平方公里 | L1 | 同 A1a | "They turned the lidar strips (about 1,200 km² of jungle) into images" | 02-blog-48-agents.png | “about”；数据来源：原版 “NASA has collected … a huge open dataset”（链接 gliht.gsfc.nasa.gov，即 NASA G-LiHT），更新版改为 “public lidar”；图注 “Lidar: bare ground, 2013”；X 帖视频角标 “NASA G-LiHT lidar, 2013”。博文未说明 1200 km² 是否全部为同一次飞行 | 已找到 |
| A2c | 10 月 18—21 日 | L1 | 同 A1a | 小标题 "18 October" "19 October" "20 October" "21 October"；"INAH replied: they checked their records and found no registered archaeological site at that location." | 02、04 | 年份：INAH 回函与作者来函抬头均为 2026 年。18 日起意；18 日夜让智能体分析；19 日晨看报告，“a few hours later” 查文献/档案；20 日改报告、译西语、发信；21 日 INAH 回信 | 已找到 |
| A2d | 作者非考古学家 | L1 | 同 A1a | "Disclaimer: I'm not an archaeologist." | 06-blog-not-archaeologist-models.png | 另有结尾 “I'm not an archaeologist, so I didn't know this was supposed to be hard.” | 已找到 |
| A2e | “Google SWE” 出处 | L2/L6 | HN 提交标题（Wayback 快照 `a-wb-hn-20261111131931.txt`、`…161721`、`…180933`、`…203205`）；作者 X 简介（`a-x-2120219901814043087.json` 的 author.description） | HN 标题（提交者 eyvindn）：“Google SWE points 48 AI agents at NASA Lidar, finds unmapped Maya pyramid in 72h”，链接带 `?maya` 参数；X 简介：“Research @Google Paradigm of Intelligence”；作者博客首页（Wayback 2026 年快照 `a-wb-home.txt`，快照时刻未取；当前页与之文字一致）：“Im an AI researcher” “currently living in Zurich”，**未写雇主**；INAH 回函抬头称他 “INVESTIGADOR INDEPENDIENTE” | 08-hn-title-google-swe.png（Wayback 21:19:31 北京快照 vs 现标题）；04（回函抬头） | “Google SWE” 不是作者博文里的话，是 HN 提交标题的措辞（该标题在 20:32 UTC 快照仍在，06:52 UTC 快照已改成 “My AI agents found an unmapped Maya site?”，即北京 11-12 04:32 至 14:52 之间被改）。作者在 X 简介自称在 Google 做研究（L2：自述，非职衔验证）；“SWE”（软件工程师）这一职衔我没有找到作者本人或 Google 一手页面的出处，标未找到。HN 评论对“是否用了雇主 token”有疑问（见吠点），作者未回答；博文未写 | 部分支持 |
| A3 | INAH 回信“无登记” | L1（作者公布的回函截图） | 同 A1a（回函图 `https://pietromiotti.com/latest/maya-site/inah-reply.jpg`，Wayback 存档 `a-inah-reply-img.jpg`） | “Se realizó la revisión correspondiente de la información proporcionada en el Sistema de Información Geográfica de esta Subdirección, sin que se identificara un sitio arqueológico registrado en la ubicación señalada en su informe.” 作者译文：“…no registered archaeological site was identified at the location given in your report.” | 04-blog-inah-reply.png | 回函：Dirección de Registro Público de Monumentos y Zonas Arqueológicos e Históricos，Oficio No. 401.3S.4-2026/1476，Ciudad de México，2026-10-21。检索范围是该司**自己的地理信息系统**（登记册），不是文献。落款官员姓名与市县已被作者涂黑；图中只有作者姓名、苏黎世（作者自己公布）。无私人邮箱/电话。作者来函 10-20（见博文 “Zurich, Suiza, a 20 de octubre de 2026”，截图未收，坐标在保密附件） | 已找到 |
| A4 | Estrada-Belli et al. 2025 论文 | L1/L3（期刊元数据） | https://doi.org/10.1016/j.jasrep.2025.105288 → 落地页被验证页拦（`a-paper.html` 为 2.7 KB 跳转/验证页）；元数据取自 Crossref `a-paper-crossref.json`、OpenAlex `a-paper-openalex.json` | 标题 “New regional-scale Classic Maya population density estimates and settlement distribution models through airborne lidar scanning”；作者 Francisco Estrada-Belli、Marcello A. Canuto（Tulane）、Ivan Šprajc（Slovenian Academy of Sciences and Arts）、Juan Carlos Fernández-Diaz（University of Houston）；Journal of Archaeological Science: Reports，vol 66，article 105288，2025 年 10 月刊；摘要（OpenAlex）：“…we aggregate a diverse set of public and privately commissioned lidar mapping, including reprocessed environmental lidar data from Southern Campeche and Southern Quintana Roo, Mexico with enhanced structure visibility, to update population estimates to between 9.5 and 16 million people in a large central area of the Classic Maya Lowlands.” | 无（ScienceDirect 页 curl 返回 403，opencli 浏览器出现 “Are you a robot?” 验证码，不绕过；未截） | Crossref：DOI 记录 created 2025-07-07（UTC）、issued 2025-10；版权 CC BY-NC-ND（Crossref license `vor` 起始 2025-07-11）；OpenAlex `publication_date` 2025-07-07，hybrid OA。Wayback 对 doi.org 该 DOI 的最早快照 2025-07-29（302 跳转，`a-wb-cdx.json` 之外另查，未存档）；ScienceDirect 文章页的全部 Wayback 快照（2025-07-29 至 2026-09-22 共 19 个）状态均为 403/302，没有可用的页面内容，故“看不到第 14 页”成立。**我没能看到第 13–14 页或任何涉及该遗址的位置图**：“第 14 页” 出自作者 HN/X 自述，“第 13–14 页显示结构” 出自 HN 评论 50454005（tangotaylor）；二者我均未能独立核对。论文整体是区域人口估算，Crossref/OpenAlex 摘要没提单个遗址 | 部分支持 |
| A5 | 热度（HN 三帖与 X 两帖的分数/浏览） | — | HN API（`a-hn-50452455.json`、`a-hn-50449945.json`、`a-hn-50446316.json`、`a-heat.json`）；X（fxtwitter JSON `a-x-2120219901814043087.json`、`a-x-2120557109410775430.json`） | 见下文“热度” | 08、09、10、11 | 取得时刻 2026-11-12 17:53（HN）、约 18:00（X）北京 | 已找到 |
| A6 | 夸大实例 | L4/L5 | 见下文“夸大说法实例” | — | 12 | — | 已找到 |

## 二、热度

| 项 | 数字（取得时刻北京） | 提交时刻 | 备注 |
|---|---|---|---|
| HN 50452455（提交者 eyvindn）现标题 “My AI agents found an unmapped Maya site?” | 990 分 / HN 页面 359 条评论（17:53；18:00 页面显示 361；Algolia 嵌套计 356） | 2026-11-11 20:05:22（12:05:22 UTC） | 原标题 “Google SWE points 48 AI agents at NASA Lidar, finds unmapped Maya pyramid in 72h”，链接带 `?maya`。Wayback 分数轨迹：21:19:31 北京 73 分/25 评；11-12 00:17:21 587/230；02:09:33 767/284；04:32:05 851；14:52:15 968（最后一次快照，标题已改） |
| HN 50449945（提交者 nnx）“My AI Agents Found an Unmapped Maya Site in Mexico” | 11 分 / 3 条（Algolia 嵌套） | 2026-11-11 13:09:10（05:09:10 UTC） | |
| HN 50446316（提交者 pvitz）“My AI Agents Found an Unmapped Maya Site” | 10 分 / 0 条 | 2026-11-11 05:04:58（11-10 21:04:58 UTC） | 三帖中最早，链接为博文原 URL |
| HN 50450936（提交者 tosh，指向作者 X 帖，派工单没列，我顺手查到） | 2 分 / 2 条（`a-hn-4.txt`） | 2026-11-11 16:23:27（08:23:27 UTC） | 标题 “Found an unmapped Maya site in Mexico, using Lidar data from a NASA dataset” |
| X 原帖（@PietroMiotti，status 2120219901814043087） | 20,005 赞 / 2,068 转 / 1,320,891 浏览 / 267 回复（fxtwitter，约 18:00） | 2026-11-11 02:19:51（11-10 18:19:51 UTC） | 文本：“My AI agents found an unmapped Maya site in Mexico, using LiDAR data from a NASA dataset. 72 hours from a random thought on vacation to a reply from Mexico's national archaeology institute. What a time to be alive…”，附 13 秒视频；fxtwitter 的 `community_note` 字段为空（第三方镜像曾显示“读者补充背景”，我没有一手确认，不采信） |
| X 更正帖（status 2120557109410775430） | 112 赞 / 5 转 / 4,093 浏览 / 14 回复 | 2026-11-12 00:39:47（11-11 16:39:47 UTC） | 文本：“Update: the site had already been discovered. It appears in a 2025 paper by Estrada-Belli, Canuto, Šprajc & Fernandez-Diaz (page 14) https://doi.org/10.1016/j.jasrep.2025.105288”。更正帖浏览量约为原帖的 0.3%（4,093 / 1,320,891） |
| Reddit | 未找到（`reddit.com/search.json` 返回 403；exa 搜索也未返回 Reddit 帖） | — | 如实记失败，不凑数 |

注：HN 页面对匿名访客**不显示评论分数**。下面评论的“热度”改用其所在楼的回复数与排序位置，不是分数。

## 三、可能的吠点（线索，L6；进正文前回原文核实）

1. 更正前的版本**早已在文末引用了同一篇论文**，只是写成“others used lidar to estimate how many people lived in the Maya lowlands”（Wayback 2026-11-11 04:13:37 北京快照；图 07 与图 05 对照）。更正后改为“mapped settlements …, including this very site”。可说：论文在原文里，只是当时没把它和这处遗址对上。出处：`a-wb-blog-20261110.txt` 第 104 行 vs `a-blog.txt`。
2. “未登记”不等于“未被发现”：INAH 回函只写检索其“Sistema de Información Geográfica”未识别出登记遗址（图 04）；作者自己更新后也写 “it just wasn't in the register … yet”。原文起初就写 “it's not official yet”，但同时写了 “nobody had put this one on a map before”（原版，Wayback）。
3. 原标题/引言用 “discovered … no one had reported before”，更新前并没有写过“智能体查过文献”的证据；文中唯一的查重动作是作者的指令 “Search both the literature and the archives.”，智能体回的是 “One major village doesn't seem to have been registered.”（图 02）——回答的措辞是 “registered”，不是“published”。
4. 博文没有公布智能体对话、提示词、转录，也没写 48 个智能体用的是哪家模型（只在头脑风暴一句列了 Gemini/ChatGPT/Claude/Grok）。HN 评论要求转录（embedding-shape，50453722，所在楼 6 回复：“Now I'm very interested in seeing the transcript/session from what the agent actually did.”）。
5. 成本/资源口径没写：不知是作者自费还是雇主 token。HN 评论 StilesCrisis（50453202）：“I think it matters because "I used 48 agents running for 72 hours" either means you invested $50,000 of your own money, or you work for a company that gives you unlimited tokens.”；toephu2（50457699、50457764）：“It's not clear whether the author used personal tokens or Google's token allowance (given to each worker) to perform this analysis.” agnosticmantis（50453520）指出 “72 小时” 是起意到回信的墙钟时间，不是 48 智能体跑 72 小时：“The title reads as if the 48 agents took 72 hours to find this. The text doesn't exactly say that”。
6. 博文自己标注：解读（古典期、统治者居所、水库）是 “speculation, from a long chat with the agents”；水库 “Or it's just a hole.”；“The site in 3D … Heights exaggerated ~1.5×”（图 03 与 02 之外的原文段，已存 `a-blog.txt`）；“Generated with Gemini from the lidar layout” 的 AD 700 复原图是生成图。
7. 数据不是智能体“发现”的新数据：同一套 NASA 环境激光雷达（G-LiHT，2013）早被考古学界重新处理使用；论文摘要也写用了 “reprocessed environmental lidar data from Southern Campeche and Southern Quintana Roo”。博文原版自己写 “how lidar collected for environmental research can be reused for archaeology (Golden et al. 2016)”。
8. 检测本身不能算没用：该遗址被 48 个通用智能体从图像里挑出来，而专业考古团队也从激光雷达里测到了它，等于一个“对上了专业结果”的检出（sysnote 的评论，L5，仅作线索）；吠点点在“新颖性判断”而不在“检出”。这条需要拿到论文第 14 页确认“同一遗址”后才能用；我看不到该页（见 A4），不能自行判定“就是同一处”，只有作者自己承认这一点。
9. 谁先指出“早已发表”的措辞差异：The Verge 引 Sarah Parcak 说 Estrada-Belli “published the site years ago”（Verge 转述；Parcak 的 Bluesky 原帖文字见 HN 评论 50453745 的引文 “ages ago”），而 Crossref 的 DOI 记录创建于 2025-07-07、期刊卷期为 2025 年 10 月，距博文（2026-11-10）约 16 个月。两处说法不一致，我不取舍，只记录；正文用“2025 年论文”更稳。
10. 作者更正帖影响面：原帖 1.32M 浏览、更正帖 4,093 浏览（见热度表）。

### HN 高赞/高回复评论（评论分数不可见，按所在楼规模）

- LordOmlette（50453616，北京 11-11 22:08:57，该楼 134 条回复，位列顶部）：指向 Sarah Parcak 的 Bluesky 帖称遗址早已发现。https://news.ycombinator.com/item?id=50453616
- tangotaylor（50454005，22:40:19，该楼 74 条回复）：“Here is the referenced publication. Pages 13-14 show the structures.” 链接 ScienceDirect。https://news.ycombinator.com/item?id=50454005
- wosk（50453745，22:19:56）：引 Parcak 帖并给 DOI：“Basically: already known, not in the Mexican Gov database, but in published literature... and open-access.” 图 09。https://news.ycombinator.com/item?id=50453745
- strix_varius（50454717，18 小时前）：论文图里就能看到，“the title of this seems incorrect ("finds unmapped Maya pyramid")”。https://news.ycombinator.com/item?id=50454717
- wholinator2（50454514，该楼 9 条回复）：自己的经验，让智能体做文献检索后常回“prior art has reduced the novelty of the discovery”——只是个人经验，L6。https://news.ycombinator.com/item?id=50454514
- nickalaso（50454564，该楼 14 条回复）：“his primary mistake was not turning the agent swarm on a proper research task that exhaustively determined whether it had been discovered or not.” https://news.ycombinator.com/item?id=50454564
- miotts（作者，50455574，北京 11-12 00:53:16，3 条回复）：更正，见 A1。图 09。https://news.ycombinator.com/item?id=50455574

## 四、夸大说法实例（原站 URL、原句逐字、发布时刻、发布方、是否更新）

**未找到与派工单所写完全一致的中文标题“谷歌工程师用 48 个 AI 智能体发现玛雅金字塔”**。找到的最接近实例如下（均已存档于 `sources/a-*.txt`，未存档的注明）：

| 发布方 / 语言 | URL | 标题与原句（逐字） | 发布时刻（北京；原时区） | 是否已更新为“重新发现” |
|---|---|---|---|---|
| TechRadar（David Nield，英文） | https://www.techradar.com/ai-platforms-assistants/google-researcher-discovers-unmapped-maya-site-in-mexico-using-48-ai-agents-project-takes-less-than-72-hours-after-agents-scour-nasa-lidar-data （`a-techradar.*`） | 标题 “Google researcher discovers unmapped Maya site in Mexico using 48 AI agents — project takes less than 72 hours after agents scour NASA Lidar data”；要点 “Experts are now investigating the site that the AI has flagged up”、“The discovery came from a scan of LiDAR data collected by Mexico”；正文 “Miotti tasked 48 agents from different AI apps with scouring through a NASA database of LiDAR information collected across Peru”、“Representatives from INAH got in touch, and the experts there are investigating further: a real result, in 72 hours.” | 2026-11-11 23:00:02（15:00:02 UTC，页面元数据） | **否**：全文无 “already/Estrada/update” 字样（grep 确认，`a-techradar.txt`）。与博文不符之处：博文数据来自墨西哥玛雅区而非“秘鲁”（博文写 “Peru isn't well covered”）；博文没有说 INAH “experts are investigating”，只写 “it's now up to the local INAH centre to decide whether to send a team”（原版） |
| The Verge（Robert Hart，英文） | https://www.theverge.com/ai-artificial-intelligence/1031498/an-ai-agent-swarm-may-have-unearthed-an-unmapped-maya-site （`a-verge.*`） | “An AI agent swarm may have unearthed an unmapped Maya site. Google researcher Pietro Miotti says 48 AI agents combing NASA Lidar data spotted signs of a previously unrecorded ancient site in Mexico. He's not an archeologist, and the find still needs fieldwork to verify” | 2026-11-11 19:04:02（11:04:02 UTC，页面“Posted”） | **是，部分**：页内加了 “Update: This post was updated to include context from Sarah Parcak about the site's earlier publication.”，标题仍为 “unmapped”；元数据 `dateModified` 仍是 11:04:02 UTC（Verge 没更新元数据时刻）。图 12 |
| ZICQ 智客（中文，AI 聚合站，L5） | https://zicq.com/articles/n-0460704fe730-… （`a-zicq.*`） | 标题 “Google调度48个SWE智能体72小时扫描NASA激光雷达，发现未测绘玛雅金字塔”；正文 “Google 的一项实验让 48 个 SWE 智能体并行分析 NASA 激光雷达数据，并报告在 72 小时内发现一座此前未被测绘的玛雅金字塔。” | 页面元数据只有日期 2026-11-11；exa 给 17:53:42 UTC（11-12 01:53 北京，未核） | **否**：全文无 “Estrada/已被/重新发现”。把“作者一个人用 48 个通用 AI 智能体”写成“Google 调度 48 个 SWE 智能体” |
| Yuri Island 有粒岛（中文，AI 生成情报站，L5） | https://youlidao.ai/zh/intelligence/48-ai-agents-find-unmapped-maya-pyramid-72-hours （`a-youlidao.*`） | 标题 “48个AI智能体72小时内发现未标注玛雅金字塔”；“谷歌共部署48个SWE AI智能体”；“发现结果为一座此前未被记录的玛雅金字塔结构，之前没有任何人类调查标记过它”；“智能体完成了人类需要花几个月才能做到的事，并带来了真实的考古发现” | 2026-11-12 05:03:06（页面元数据 2026-11-11T21:03:06 UTC；页内显示 “2026/11/11 21:03”） | **否**：全文无更正 |
| 智讯 AI News（中文，聚合站，L5） | https://www.ainews.xin/article/17189 （`a-ainews-xin.*`） | “谷歌 SWE 项目用 48 个 AI 智能体 72 小时发现未标注玛雅金字塔”；摘要 “谷歌 SWE 项目将 48 个 AI 智能体用于分析 NASA 激光雷达数据，在 72 小时内发现一座此前未被测绘的玛雅金字塔。” | 2026-11-11 20:05:22（12:05:22 UTC，等于 HN 提交时刻）；modified 21:00:59 北京 | **否** |
| 智能 zmj.net（中文，L5） | https://www.zmj.net/news/4240 （`a-zmj.*`） | 标题 “48个AI智能体72小时从NASA雷达数据中发现玛雅金字塔”；“在 72 小时内找到一处此前未被测绘记录的玛雅金字塔。项目作者 Pietro Miotti 以网页形式公布了这一结果，标题将其描述为"Google SWE 把 48 个 AI 智能体指向 NASA 的 Lidar 数据"。” | 2026-11-11 22:05:46（14:05:46 UTC，页面元数据） | **否**；且把 HN 提交标题误当成作者页面标题（作者博文标题从来不含 “Google SWE”） |
| ic.work（中文，Ada Vector） | https://www.ic.work/article/google-engineer-uses-48-ai-agents-nasa-g-liht-lidar-unregist （`a-icwork.*`） | 标题 “用 NASA G-LiHT 与 48 个 AI Agent 72 小时找玛雅遗址，真突破在哪”；要点 “谷歌工程师 Pietro Miotti 耗时不到 72 小时，调度 48 个通用 AI Agent 扫过 1200 平方公里 NASA G-LiHT 公开激光雷达数据，筛出一处墨西哥官方未登记的疑似玛雅遗址” | 2026-11-11 21:16:01（13:16:01 UTC） | **否**（全文无 Estrada/已被测绘；但文内已区分“未登记 ≠ 未发现”，相对克制）；“谷歌工程师”“调度 48 个 Agent”仍属措辞放大 |
| 新浪（微博博主“爱可可-爱生活”，中文） | https://www.sina.cn/weibo/detail/5364338283250749.html （`a-sina-weibo.*`） | 标题 “72 小时 AI 找遗址，专家说：不是发现，是重新发现”；“后来他更新承认，这处地点其实已在 2025 年的学术研究中被测绘，只是代理检索文献时漏掉了，所以严格说不是“发现”，而是“重新发现”。” | 2026-11-12 07:56（页面显示） | **是**（对照用，非夸大例）。个人博主作者，非媒体，L6 |

**未核线索（未列入上表）**：Adafruit Blog《A Google engineer points 48 AI agents at NASA Lidar and finds an unmapped Maya pyramid》（Anne Barela，页面写 “November 11, 2026 AT 4:27 pm”，时区未知）。我这边抓取被 Cloudflare 质询拦截（403），页面未存档；再用 opencli 浏览器试开被权限检查拒绝（判定为绕过验证），按规则不再试。标题与“后来更新为重新发现”一句只来自搜索摘录（exa），**未亲自核对原页**。kongchang（控场AI，中文，标题 “48个AI智能体72小时发现玛雅金字塔，Google SWE如何做到的”）同样抓取被拦（429），只见搜索摘录：“一支由Google软件工程师（SWE）组建的团队，把48个AI智能体指向了NASA公开的激光雷达（LiDAR）数据”，亦未核。

另：英文 HN 聚合站 ByteBrief、PithWire（`a-pithwire.*`，“Google SWE points 48 AI agents …” 复述 HN 标题）、Yidinghui（`a-yidinghui.*`）也在 11-11 复述 “Google SWE” 标题。已更正的对照例：sysnote（`a-sysnote.*`，2026-11-12 01:08:37 北京，标题 “Pietro Miotti's 48 AI agents didn't find an unmapped Maya site: it was on page 14 of a paper his own post linked”，L5）、RuntimeWire（`a-runtimewire.*`，11-11 04:16 北京发布、11-12 03:18 北京更新，付费墙后只见摘要）。大陆主流媒体（36氪、新浪科技、澎湃、腾讯等）的报道我在 exa 搜索里没有搜到，如需要可再找。

## 五、扫描说法勘误（与已知情况/扫描线索对照）

1. **“Google SWE” 不是作者博文或作者自称**：它是 HN 提交标题（eyvindn，`?maya` 链接）的措辞；博文首页称作者 “AI researcher … currently living in Zurich”，X 简介为 “Research @Google Paradigm of Intelligence”，INAH 回函抬头为 “INVESTIGADOR INDEPENDIENTE”。HN 标题后改为 “My AI agents found an unmapped Maya site?”。新闻里 “Google researcher / Google engineer / Google SWE” 三种称呼都有。
2. **“72 小时”**：是起意（10-18）到 INAH 回信（10-21）的墙钟时间，不是 48 个智能体运行 72 小时；博文也没写智能体跑了多久。
3. **“48 个智能体用的是什么模型”**：博文没写。写了的只有头脑风暴用 Gemini/ChatGPT/Claude/Grok，翻译建议来自 Claude，复原图来自 Gemini。博文里 “agents” 和 “48 agents” 不要混写成某家的模型。
4. **更新前后的措辞差异**（已存档对比，见 A1b）：原版 “discovered … as far as anyone can tell, no one had reported before”，且原版已引用 Estrada-Belli 2025（描述为人口估算研究）；更新版把文献描述改成 “including this very site”。更新版还改动了 NASA 数据集/token 句（原版 “created a token on the NASA portal”，更新版 “public lidar”）和结尾（删去 “field check led by INAH” 一段，改为 “now in the hands of … INAH and the archaeologists who mapped it first”）。
5. **“HN 网友指向第 13—14 页”**：属实（tangotaylor 评论：“Pages 13-14 show the structures.”），作者自己写 “page 14”；两者我都没能独立核对（ScienceDirect 验证码）。
6. **“INAH 回信只写该坐标无登记”**：属实，原文用 “Sistema de Información Geográfica de esta Subdirección” 说明查的是自己的 GIS 登记册。
7. **HN 数字**：派工单 989/359 与我 17:53 取得的 990/359（页面）一致；评论数在 356（Algolia 嵌套）至 361（18:00 页面）之间浮动。
8. **论文发表日期**：Crossref 的 DOI 记录创建于 2025-07-07，OpenAlex 的 `publication_date` 为 2025-07-07，期刊卷期为 2025 年 10 月（卷 66）；Wayback 对该 DOI 的最早快照为 2025-07-29。我没有看到出版社页面上的“在线发表日期”，上面是推断：论文至迟在 2025 年 7 月底已可由 DOI 解析。
9. 派工单“中文转载标题多为‘谷歌工程师用 48 个 AI 智能体发现玛雅金字塔’”：**未找到该原句**，只找到变体（见第四节）；中文聚合站多把 “Google SWE” 误写成“Google 的 48 个 SWE 智能体”。

## 六、截图清单（`docs/2611/1111/images/`，均已逐张打开目视确认）

| 文件 | 内容 | 备注 |
|---|---|---|
| 01-blog-title-update.png | 博文标题、“Last edit: 11 Nov”、更新段与首段 | 浅色底，1400 px 宽，2 倍像素 |
| 02-blog-48-agents.png | “19 October” 小标题 + “48 agents went through them one by one”段 + 作者提问与 “One major village doesn't seem to have been registered.” | 两段拼接，中间略去 4 张缩略图（间隔 24 px） |
| 03-blog-lidar-zoom.png | 作者原图：山顶激光雷达图与文明中心放大（标注 Plaza、Pyramid 8 m+、Raised courtyard、Reservoir?，比例尺 100 m / 50 m） | 作者原图，坐标已被作者隐去；非转载图 |
| 04-blog-inah-reply.png | “21 October” + INAH 回函（高亮原句）+ 作者图注 + “At that point I thought nobody had put it on a map before. I was wrong (see below).” | 地名与官员姓名已被作者涂黑 |
| 05-blog-literature.png | 更新版文献段（“including this very site (Estrada-Belli et al. 2025)”）+ “What's next” | |
| 06-blog-not-archaeologist-models.png | “Disclaimer: I'm not an archaeologist.” + 四个聊天机器人头脑风暴段 + “That night, I asked the agents to analyze it and went to bed.” | 两段拼接 |
| 07-wayback-original-intro-full.png | 更新前版本：原标题、“discovered … no one had reported before”、“nobody had put this one on a map before”、原文献段 | Wayback 快照 2026-11-11 04:13:37 北京；**后备字体渲染，文字未改**；三段拼接 |
| 08-hn-title-google-swe.png | HN 帖：Wayback 21:19:31 北京快照标题 “Google SWE points 48 AI agents …”（73 分/25 评）与现标题 “My AI agents found an unmapped Maya site?”（990 分/361 评） | 两段拼接，间隔为 HN 橙色底 |
| 09-hn-doi-and-author-reply.png | HN 评论 wosk（给出 DOI）与作者 miotts 更正评论 | 两段拼接；评论分数 HN 不显示 |
| 10-x-original-post-card.png | 作者 X 原帖卡片（“My AI agents found an unmapped Maya site in Mexico …”，约 20K 赞） | X 官方嵌入匿名渲染，只含帖子卡片；账号头像/昵称是作者本人的公开账号，非抓取者 |
| 11-x-correction-post-card.png | 作者 X 更正帖卡片（“Update: the site had already been discovered …”，112 赞） | 同上 |
| 12-media-verge-techradar-headlines.png | The Verge 帖（含 “Update: … Sarah Parcak”）、TechRadar 标题与署名、TechRadar 三条要点 | 三段拼接；TechRadar 页面显示 “Published 19 hours ago” 是渲染时的相对时间，发布时刻见第四节 |

未截的候选：论文落地页标题区（ScienceDirect 验证码）；Sarah Parcak 的 Bluesky 帖（见下）。

## 七、假设与限制

- 作者个人信息：只记其公开博客的自我介绍（AI researcher、苏黎世）、X 简介和 INAH 回函抬头；未收集其他个人信息。
- **Sarah Parcak 的 Bluesky 帖**（Verge 与 HN 引用）：该帖的 Bluesky 官方嵌入页显示 “The author of this post has requested their posts not be displayed on external sites.”，故**未截图、未嵌入、证据表不引其全文**。公开信息只用 Verge 的转述与 HN 评论。我曾用 Bluesky 公开 API 读过该帖（帖 3n2235hwwsk2m 时刻 2026-11-11 13:36:05 UTC，即 21:36:05 北京，早于 HN 的 DOI 评论 22:19:56 与作者更正），已把那次存档 `a-bsky-parcak.json` 列入待删除，建议删除。
- 第三方镜像站（nitter 等）只在 exa 搜索摘录里出现，未打开、未存档。
- 评论分数不可见（HN）；用楼内回复数代替。
- 事实判断（“同一遗址是否就是论文第 14 页那处”）：只有作者自述支持，我无法看论文页，故 A1 状态仅限于“作者这样写”；论文 DOI 与元数据我另行核了。

## 八、验证 / 推断 / 未验证

- **已验证（亲自打开存档或原页）**：博文现版与更新段全文；更新前版本（Wayback 11-11 04:13:37 北京）措辞与 diff；INAH 回函图（作者公开版）；作者 X 两帖原文与计数（fxtwitter JSON + 匿名嵌入渲染）；HN 三帖元数据与标题/分数演变（Algolia + Wayback）；论文元数据（Crossref、OpenAlex）；The Verge、TechRadar、ZICQ、youlidao、ainews.xin、zmj.net、ic.work、新浪微博博主页的标题与原句（本地 `.txt`）。
- **推断**：更新段上线时刻（不早于 11-11 23:56 北京，一般不晚于 11-12 00:53，依据 Wayback 缺失更新段 + 作者 X/HN 更正时刻）；HN 标题被改动的时间窗（11-12 04:32 至 14:52 北京）；论文“至迟 2025 年 7 月底可由 DOI 解析”；Adafruit 帖时区；“Google SWE” 是 HN 提交者措辞而非作者话。
- **未验证**：“该遗址就是论文第 14 页（或第 13–14 页）所示那处”只有作者自述与 HN 评论两个来源，我没能看到论文那几页（ScienceDirect 验证码、Wayback 快照均为 403），**不能写成已核实**；作者 X 简介里的 Google 研究职位只有他自己的简介；Adafruit 与 kongchang 的标题；Parcak 帖的原文与时刻（未存档）；X 上是否有 Community Note（fxtwitter 为空，第三方镜像曾显示，未采信）。

## 九、待删除/不得入库的文件（精确路径）

- `docs/2611/1111/images/07-wayback-original-claims.png`（裁切边界落在文字行内，被截断）
- `docs/2611/1111/images/07-wayback-original-lead.png`（第 4 行被截断）
- `docs/2611/1111/images/07-wayback-original-intro.png`（第 4 行被截断）
- `docs/2611/1111/sources/a-bsky-parcak.json`（**不得提交**：Parcak 在 Bluesky 设置了不在外部站点显示其帖子）
- `docs/2611/1111/sources/a-reddit-search.json`（403 页，无内容）
- `docs/2611/1111/sources/a-adafruit.headers`、`a-adafruit.html`（Cloudflare 质询页）
- `docs/2611/1111/sources/a-kongchang.headers`、`a-kongchang.html`（429 页）
- `docs/2611/1111/sources/a-gigazine.headers`、`a-gigazine.html`（404 页）
- `docs/2611/1111/sources/a-llms-blog.headers`（请求失败的残留）
- `docs/2611/1111/sources/a-paper.html`、`a-paper.headers`（Elsevier 跳转页，已脱敏，可删可留；无正文价值）
