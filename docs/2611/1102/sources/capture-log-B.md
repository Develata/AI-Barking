# B 组 capture-log（北京时间，UTC+8）

执行方 claude-sonnet-5-5；时段 2026-11-03 19:10 – 约 20:00（北京；本机 America/New_York EST 06:10–约 07:00）。工具：curl、`barking fetch` / `html2txt` / `heat` / `shot`、opencli browser（会话 b、c、d、e，`--window background`，未登录、未接受 Cookie、未发帖）、WebSearch（只作线索，不存档）。机器日志：`sources/b-fetch-log.jsonl`（fetch）、`sources/b-shots-log.jsonl`（shot）、`sources/b-heat.json`（heat）。

## 1. SEC 文件

| 时间（北京） | URL（已去查询参数） | 工具 | 结果 |
|---|---|---|---|
| 19:10 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm | curl，`-A "AI-Barking research"` | 403，页面为 “Your Request Originates from an Undeclared Automated Tool”（SEC 要求 User-Agent 含公司信息；该页是 4,818 字节的拦截页，已被后续覆盖，`b-s1a.headers` 仍是这次 403 的响应头，**不是**正文的头，建议删） |
| 19:10 | 同上 | curl，`-A "AI-Barking/1.0 (public-document research, non-commercial)"` | 403，同上（同一来源失败两次，改 opencli；未在 UA 里放邮箱或个人信息） |
| 19:11 | 同上 | opencli browser 会话 b：`open` → `get html` | ok。13,955,780 字符（EDGAR 页内的 `<document>` 包装 + 正文）。包装层带有浏览器扩展写入的属性 `data-immersive-translate-page-theme`（翻译插件，属本机浏览器，不含账号信息）。存为 `b-s1a.html` |
| 19:11 | 同上 | `barking html2txt` | `b-s1a.txt`，47,246 行，1,668,185 字节（超 1 MB，建议不入库；相关章节按行号摘录为 `b-s1a-excerpts.txt` 55 KB） |
| 19:12 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm（首版） | opencli 会话 b：`open` → `get html` → `html2txt` | ok。`b-s1-first.html`（13.8 MB）、`b-s1-first.txt`（46,388 行，1,637,558 字节，建议不入库）。用于逐句对照 |
| 19:12 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/0001628280-26-068676-index.htm | opencli 会话 b/c：`open` → `extract` | ok。Accepted 2026-11-02 09:24:24（美东 EST），Documents 50。存为 `b-edgar-index-s1a.json` |
| 19:30 | …/000162828026067168/0001628280-26-067168-index.htm（首版索引） | opencli 会话 c：`open` → `extract` | ok。Accepted 2026-10-26 09:00:11，Documents 87。`b-edgar-index-s1.json` |

两版差异的方法：把两份 txt 去掉页眉页脚与空白后，(a) 对关键短语统计在两版全文中的命中次数，(b) 对个别段落并排阅读。脚本放在会话暂存目录，没有写入仓库；结论以 evidence-B.md 为准。

## 2. barking fetch 批次

| 时间 | 清单 | name | URL | 结果 | 备注 |
|---|---|---|---|---|---|
| 19:16 | b-list1.tsv | cnbc | https://www.cnbc.com/2026/11/02/anthropic-ant-s-1-quarter-financials.html | ok 200 | 378 行 txt；发布 Mon Nov 2 2026 9:32 AM EST；JSON-LD datePublished 2026-11-02T14:32:13Z |
| 同上 | 同上 | tpp | https://www.techpolicy.press/the-sec-should-block-anthropics-ipo-to-protect-the-public/ | ok 200 | datePublished 2026-11-02T21:34:46Z |
| 同上 | 同上 | bi-internal | https://www.businessinsider.com/anthropic-claude-internal-ai-use-exceeds-employee-pay-tokens-2026-11 | ok 200 | 页面带订阅提示，但正文数段可见（含“prices Anthropic charges customers”那句）；datePublished 2026-11-01T15:00:01Z；未绕付费墙 |
| 同上 | 同上 | bi-pay | https://www.businessinsider.com/anthropic-ipo-filing-q3-personnel-costs-2026-11 | ok 200 | 另一篇（HN 50346009 “pay packages cost … more than $100k a month per employee”），本组未用，存档备查 |
| 同上 | 同上 | anthropic-ipo | https://ipo.anthropic.com/us/ | blocked 200（正文仅 31 字符） | 无 txt，不用；`b-anthropic-ipo.html/.headers` 为空壳 |
| 19:17 | b-list2.tsv | regen | https://www.regenerator1.com/p/anthropics-growth-just-flattened | ok 200 | HN 50344958 的原文，页面日期 Oct 15, 2026；作者 Henry Blodget；依据 NYT 10/14 报道推算 |
| 19:17 | b-list3.tsv | yicai | https://www.yicai.com/news/103422903.html | ok 200 | 第一财经 2026-11-03 10:43:34 |
| 同上 | 同上 | thepaper-slowdown | https://www.thepaper.cn/newsDetail_forward_34394715 | ok 200 | 澎湃 11-03 13:47 |
| 同上 | 同上 | thepaper-loss | https://www.thepaper.cn/newsDetail_forward_34393004 | ok 200 | 澎湃号（财联社稿）11-03 08:14 |
| 同上 | 同上 | jiemian | https://www.jiemian.com/article/15287521.html | ok 200 | 界面新闻 11-03 15:23（页面含两处标题重复，正文同） |
| 同上 | 同上 | 21jingji | https://m.21jingji.com/article/20261103/herald/50772c170d79b150aefb905b6e992215.html | ok 200 | 页面署“来源：财联社”，11-03 07:51 |
| 同上 | 同上 | sina-wm | https://finance.sina.com.cn/wm/2026-11-03/doc-inixmnqf8830096.shtml | ok 200 | 第一财经稿转载 |
| 同上 | 同上 | sina-roll | https://finance.sina.com.cn/roll/2026-11-03/doc-inixkrkq2621761.shtml | ok 200 | 财联社稿转载 |
| 同上 | 同上 | 163 | https://www.163.com/dy/article/LAP6NM8A05198NMR.html | ok 200 | 网易号，署名栏未细核 |
| 同上 | 同上 | chinacom | https://news.china.com/socialgd/10000169/20261103/49836296.html | ok 200 | 页内转自科创板日报 11-03 13:29:47 |
| 同上 | 同上 | fx168 | https://www.fx168news.com/article/1111543 | ok 200 | |
| 19:18 | b-list4.tsv | ifeng-hidden-debt | https://tech.ifeng.com/c/8xfSgBuz3Bx | ok 200 | IT之家稿，2026-10-31 11:31:15（北京），经凤凰网大风号 |
| 同上 | 同上 | sina-hidden-debt | https://finance.sina.com.cn/stock/usstock/c/2026-10-31/doc-inixaraz7617421.shtml | ok 200 | 同主题新浪稿，未细读 |
| 同上 | 同上 | futunn-hidden-debt | https://news.futunn.com/post/1002996894/anthropic-s-ipo-filing-discloses-future-off-balance-sheet-liabilities | blocked 200（正文 8 字符） | 无 txt；`b-futunn-hidden-debt.html` 是空壳 |

## 3. 单次请求

| 时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 19:30 | https://hn.algolia.com/api/v1/items/50337439 | curl `--max-time 60` | ok，`b-hn-50337439.json`（CNBC 帖，233 分，嵌套评论含 minimaxir 80 回复等） |

## 4. barking heat

| 时间 | 命令要点 | 结果 |
|---|---|---|
| 19:15 | `heat docs/2611/1102 b --item 50344919 --item 50339494 --item 50344958 --hn "Anthropic S-1" --hn "Anthropic IPO" --hn "cnbc.com Anthropic" --reddit "Anthropic S-1" --reddit "Anthropic IPO"` | HN 3 条搜索与 3 个条目成功，写入 `b-heat.json`（`b-hn-50344919.json` 等）。Reddit 两次搜索都失败：`old.reddit.com` 返回登录跳转页（非 JSON），已记入 `b-heat.json` 的 errors，未重试 |

## 5. WebSearch（线索，未存档）

- “Anthropic 招股书 修订版 …”、“财联社 Anthropic 修订版招股书 …”、“华尔街见闻 Anthropic 更新招股书 …”：找到 yicai、澎湃、界面、21财经、新浪、网易、china.com、fx168 等链接，均再用 `barking fetch` 取原页，不以搜索摘要为据。
- “彭博 Anthropic 近9000亿美元 隐性债务 …” 与 “Bloomberg Anthropic IPO filing nearly $900 billion …”：找到 IT之家/凤凰网/新浪/金十等中文转述；彭博 10/31 原文 URL 未找到（只见彭博 10-26、11-02 的其他稿，未打开）。
- “… seven times cash …”：无结果。

## 6. 截图（images/ 13–24 区间；工具：opencli 会话 b/c/d/e 截图 + `barking shot cropbox/stack`）

全部来自 SEC 原页（会话 b/d 浏览器渲染，zoom 1.7，视口 1400×900 以上）与 EDGAR 索引页（会话 c，zoom 1.2）以及凤凰网页（会话 e，zoom 1）。**分辨率低于 2 倍像素比**：SEC 文件页为 1.7 倍（文件页栏宽 816 CSS px，1.7 倍后约 1387 px，在 1400 px 内；放大到 2 倍则文字栏超出 1400 px 宽），EDGAR 索引页 1.2 倍（放大到 1.5 以上时页头响应式布局变形，“SEC Accession No.”变成白字叠在浅底上），凤凰网页 1 倍（放大后固定广告层压住正文）。均只裁切、拼接，没改像素。浏览器右侧有一枚翻译插件悬浮图标，裁切时已避开；SEC 索引页弹出的满意度调查弹窗用页面脚本隐藏后才截（没点“Yes”，也没提交任何表单）。

| 文件 | 内容 | 来源 URL | 备注 |
|---|---|---|---|
| 13-edgar-s1a-filing.png | EDGAR 申报详情：Form S-1/A、Filing Date 2026-11-02、Accepted 2026-11-02 09:24:24、Documents 50、Accession 号；页底 Filer：Anthropic, PBC | …/0001628280-26-068676-index.htm | 上下两段拼接，中间 50 行文件清单省略（白底隔开）；成品是第三次尝试，前两次见待删除 |
| 14-op-loss-para.png | MD&A：九个月营收/经营亏损段、Q3 经营亏损 567 / Q2 974 / 50 亿短期算力 / 20 亿配捐段、Adjusted 段 | …/anthropicpbc-sx1ax1.htm | 单张连续裁切 |
| 15-quarterly-income-table.png | “Quarterly Consolidated Statements of Operations”整表（七个季度，含脚注） | 同上 | 单张连续裁切，单位 in millions |
| 16-adj-operating-income-table.png | “Quarterly Adjusted Operating Income (Loss) and Adjusted Operating Margin”整表含脚注 | 同上 | 渲染页脚 163 |
| 17-obligations-table.png | “Contractual Obligations”首段（895.4 / 213.8）、承诺分类表、脚注 1–4 | 同上 | 渲染页脚 173；脚注 5 在下一页（18 号图顶部） |
| 18-not-in-table-69-8b.png | 脚注 5（150–175 亿未定额）、表外项：17.4 / 451.2 / 100.2 / 6.6 / 69.8 与 4.3 | 同上 | 单张连续裁切（跨页边界之内的部分） |
| 19-vendor-commitments.png | 微软、谷歌、亚马逊、SpaceXAI 段 | 同上 | 上下两段拼接：谷歌段在页脚 174 处跨页，拼接去掉页脚与页眉（原句连续） |
| 20-amd-nscale-lambda.png | AMD、Nscale、Lambda 段 | 同上 | 单张连续裁切（页脚 175 之前） |
| 21-broadcom-lease-cash.png | 设备租赁（博通 161.2）+ 市场风险节 “$95.6 billion … $36.3 billion” 句 | 同上 | 上下两段拼接，跨页（175/176）处去掉页脚页眉 |
| 22-internal-claude-usage.png | “The value of our own internal Claude usage at list prices …” 段与新增 “However, we do not anticipate …” 句 | 同上 | 渲染页 188；上一页脚显示 187 |
| 23-violating-customers-5-7pct.png | “approximately 5% to 7% of our revenue for the nine months ended September 30, 2026” 段 | 同上 | 渲染页 146 |
| 24-first-s1-blank-table.png | 首版“Contractual Obligations”：合计 “approximately $ ,” 与承诺表所有数字栏空白 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm（首版） | 单张连续裁切；与 17 号图对照用。**与 24-media-… 同号，超出 B 组 13–24 区间的单号上限，两张共用 24** |
| 27-media-ifeng-hidden-debt.png | 凤凰网转载 IT之家稿“Anthropic未来‘隐性债务’规模近9000亿美元…” | https://tech.ifeng.com/c/8xfSgBuz3Bx | 页面 1 倍像素；含站内小广告条（页面原样），页边两侧广告与热门文章栏已裁掉；该稿作者栏无个人信息 |

候选配图“经营亏损 567 那段”“895.4 与 69.8 那段”“承诺分类表”“终止权/附条件段”“供应商段”“内部 AI 用量原句”已各有一张；“利润表 Q3 列”以整表出图（15）；“夸大标题实例”用 24-media-…；没有单独的“终止权段”图，该句在 17 号图上半部。没有做 EDGAR 文件“封面页”图（EDGAR 索引图已含文件名与受理时刻）。

## 7. 失败与未取得

- SEC 直连 curl 被 403 两次，改 opencli 后成功。
- ipo.anthropic.com、futunn 的页面：blocked（正文过短），无 txt，未改用别的方式。
- Reddit 搜索：登录跳转页，失败，未重试。
- 华尔街见闻、彭博 10/31 原文、NYT 10/14 报道：未取。
- “现金的七倍”：未找到出处。
- 首版与 S-1/A 的逐字 diff：只做关键短语检索与并排阅读，不是全文 diff。

## 8. 待删除文件（精确路径，Claude 收尾时一次删除；本组未调用 rm）

- `docs/2611/1102/images/13-edgar-index.png`（第一次拼接，背景为深色，且有翻译插件图标碎片；已由 13-edgar-s1a-filing.png 取代）
- `docs/2611/1102/images/13-edgar-filing.png`（第二次拼接，右侧裁掉了 Accession 号；已由 13-edgar-s1a-filing.png 取代）
- `docs/2611/1102/sources/b-err.txt`（空文件，重定向 stderr 的残留）
- `docs/2611/1102/sources/b-s1a.headers`（403 拦截页的响应头，与 b-s1a.html 正文无关，易误导）
- `docs/2611/1102/sources/b-anthropic-ipo.html`、`docs/2611/1102/sources/b-anthropic-ipo.headers`（blocked 空壳）
- `docs/2611/1102/sources/b-futunn-hidden-debt.html`、`docs/2611/1102/sources/b-futunn-hidden-debt.headers`（blocked 空壳）
- `docs/2611/1102/sources/b-shots-log.jsonl` 中 13-edgar-index / 13-edgar-filing 两行为废弃尝试的记录，不删文件，只在此说明

另：`sources/b-list1.tsv` … `b-list4.tsv`（fetch 清单）、`b-find.js`、`b-shot.sh` 为本组自写，保留。体积提示：`b-s1a.txt`（1.67 MB）、`b-s1-first.txt`（1.64 MB）、`b-s1a.html`、`b-s1-first.html`（各约 13.9 MB）超 1 MB，txt 建议不入库，html 属原件只存本地/网盘。

## 9. 补截两张（2026-11-03，协调方追加；北京时间约 19:35–19:45（以 b-shots-log.jsonl 为准））

会话 b 的页面已被浏览器回收，重新 `open` S-1/A 并设 zoom 1.7 后截图；界面新闻用会话 e、zoom 1.5。均只裁切、拼接，没改像素。

| 文件 | 内容 | 来源 | 尺寸 / 像素比 |
|---|---|---|---|
| 25-mdna-2031-2045.png | S-1/A MD&A（“Compute Investments”节；我的 `b-s1a.txt` 行 14386–14392）整段：“As of and subsequent to September 30, 2026, we have entered into purchase commitments and lease agreements … approximately $895.4 billion. These commitments include approximately $447.5 billion expected to be incurred before December 31, 2030, and substantially all of the remainder is expected to be incurred between 2031 and 2045. Approximately $69.8 billion …”；上文取到 “Anthropic Direct …” 段，下文取到 “We intend to fund these investments …$146.6 billion of liquidity …”整段（含尚空白的净募资额） | https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm | 1120×794 px，zoom 1.7（约 1.7 倍；页面栏 816 CSS px，放大到 2 倍会超过 1400 px 宽）。单张连续裁切 |
| 26-jiemian-debt.png | 界面新闻 https://www.jiemian.com/article/15287521.html ：上段为标题、导语（“截至9月底，Anthropic各项承诺支出总额达8950亿美元，其中约4480亿美元预计将在2030年底前发生。”）、作者栏；下段为“有评论称…”与“至于公司控制权结构方面…在公司背负近万亿长期算力负债的前提下，这一条款放大了外部股东的风险敞口。”两段 | 同上 | 1070×660 px，zoom 1.5（约 1.5 倍；再放大页面布局变形）。两段用 `barking shot stack` 拼接，中间原文省略约 1,600 CSS px（含配图、数段正文），只留 40 px 白底间隔，**视觉上近乎连续，不要当作原文相邻**。页面作者栏显示相对时间“4小时前”（取页时刻 11-03 19:40 左右），绝对发布时间为 txt 副本里的“2026/11/03 15:23 来源：界面新闻”（北京），截图里没有绝对时间；标题区没有媒体 logo 以外的账号信息 |

26 号图中“近万亿长期算力负债”一句与“各项承诺支出总额达8950亿美元”导语同图。此次未新增待删除文件（中间截图 r1/r2/j1/j2/j3 在会话暂存目录，不在仓库）。
