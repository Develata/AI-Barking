# 1119 期 D 组（速览）抓取记录

时间为北京时间（UTC+8），取自各次命令的时间戳；opencli 步骤的时间为相邻命令的近似值。

## 一、barking fetch（清单 `d-fetch-list-1.tsv`，2026-11-20 22:12:19 起，并发 6，4 秒）

| name | URL | 工具 | 结果 | 说明 |
|---|---|---|---|---|
| house-pr | https://foreignaffairs.house.gov/news/press-releases/chairman-brian-mast-exercises-legislative-authority-to-publicly-release-ai-chip-license-information-for-china | barking fetch | ok（200） | html、txt、headers 已存 |
| cnbc-chips | https://www.cnbc.com/2026/11/19/nvidia-amd-china-chip-sales-congress.html | barking fetch | ok（200） | 同上；页面含大量导航 |
| panw-ir | https://investors.paloaltonetworks.com/news-releases/news-release-details/palo-alto-networks-acquire-applied-compute-bringing-secure-open | barking fetch | blocked（403） | 改由 opencli 取，见第三节 |
| appliedcompute-raise | https://www.appliedcompute.com/company/100m-run-rate-fundraise | barking fetch | ok（200） | 同上 |
| gdmunion | https://www.gdmunion.co.uk | barking fetch | ok（200） | 同上 |
| reuters-osorio | https://www.reuters.com/legal/government/man-wrongly-jailed-after-ai-mismatch-can-pursue-lawsuit-now-2026-11-10/ | barking fetch | blocked（401） | 改由 opencli 取，见第三节 |
| techdirt-osorio | https://www.techdirt.com/2026/11/19/court-tells-cops-that-the-4th-amendment-doesnt-cover-ai-generated-probable-cause/ | barking fetch | blocked（403，Cloudflare 质询） | 改由 opencli 取，见第三节 |
| gemini-mac | https://blog.google/innovation-and-ai/products/gemini-app/computer-use-mac/ | barking fetch | ok（200） | 同上 |
| anthropic-blackhole | https://www.anthropic.com/research/claude-black-hole-physics | barking fetch | ok（200） | 同上 |
| techcrunch-ipo | https://techcrunch.com/2026/11/19/anthropics-7-founders-cashed-out-nearly-2b-in-record-breaking-ipo/ | barking fetch | ok（200） | 同上 |
| mai-image27 | https://microsoft.ai/news/introducing-mai-image-2-7/ | barking fetch | ok（200） | 同上 |
| edge-aion | https://blogs.windows.com/msedgedev/2026/11/19/hybrid-intelligence-on-the-web-introducing-aion-instruct-and-prompt-api-in-microsoft-edge/ | barking fetch | ok（200） | 同上 |
| openai-accounting | https://openai.com/business/solutions/accounting/ | barking fetch | ok（200） | 未出现 403，无需改用 render |
| openai-accounting-help | https://help.openai.com/en/articles/20001709-using-chatgpt-for-accounting-work | barking fetch | ok（200） | 同上 |
| mit-datacenter | https://news.mit.edu/2026/study-reveals-data-centers-long-distance-damage-air-quality-1119 | barking fetch | ok（200） | 同上 |

汇总：ok 12，blocked 3，failed 0。blocked 条目没有 `.txt`。

## 二、barking fetch 第二、三批

| 时间 | 清单 | name | URL | 工具 | 结果 |
|---|---|---|---|---|---|
| 22:14:47 | `d-fetch-list-2.tsv` | edgar-424b4 | https://www.sec.gov/Archives/edgar/data/2130357/000162828026074944/anthropicpbc-424b4.htm | barking fetch（默认 UA） | blocked（403） |
| 约 22:15 | 同上 | edgar-424b4 | 同上 | curl，`-A evidence-collector/1.0`，`--max-time 180` | 403，未取到；覆盖了 barking 写下的 403 页面 |
| 约 22:16 | `d-fetch-list-3.tsv` | osorio-ruling-pdf | https://fingfx.thomsonreuters.com/gfx/legaldocs/gkplennlgvb/Osorio%20Kilmer.pdf | barking fetch（PDF） | ok（200，330,309 字节，1112 行文本） |
| 22:15:56 | 同上 | documentcloud-osorio | https://www.documentcloud.org/documents/28909866-ai-isnt-pc/ | barking fetch | blocked（403） |

## 三、opencli browser（本机 Chrome 扩展会话，`--window background`；会话为维护者自己的浏览器配置，未登录的验证见下方第九节）

| 约时间 | session | URL | 操作 | 结果 |
|---|---|---|---|---|
| 约 22:13 | d-panw | https://investors.paloaltonetworks.com/news-releases/news-release-details/palo-alto-networks-acquire-applied-compute-bringing-secure-open | open，sleep 4 秒，extract | ok，正文 10185 字符（已转 `d-panw-opencli.txt`） |
| 约 22:13 | d-reuters | https://www.reuters.com/legal/government/man-wrongly-jailed-after-ai-mismatch-can-pursue-lawsuit-now-2026-11-10/ | open，sleep 4 秒，extract | ok，正文 8447 字符（已转 `d-reuters-opencli.txt`） |
| 约 22:13 | d-techdirt | https://www.techdirt.com/2026/11/19/court-tells-cops-that-the-4th-amendment-doesnt-cover-ai-generated-probable-cause/ | open，sleep 5 秒，extract | extract 只得 242 字符（评论片段，非正文）；改用 eval，见下 |
| 约 22:14 | d-techdirt | 同上 | eval：`document.body.innerText` 切片（前 6000 字） | ok；正文已取（`d-techdirt-opencli.txt`，仅前 6000 字） |
| 约 22:14 | d-techdirt | 同上 | eval：`document.documentElement.outerHTML` | ok，181,833 字符，存 `d-techdirt.opencli.html` |
| 约 22:14 | d-techdirt | 同上 | eval：PDF 链接检索 | ok，得 DocumentCloud 链接（见 `d-techdirt-links.json`） |
| 约 22:17 | d-panw | 同上（session 仍开） | eval：outerHTML | ok，46,822 字符，存 `d-panw-opencli.html`（由 `d-panw-html.json` 转出） |
| 约 22:17 | d-reuters | 同上 | eval：outerHTML | ok，746,869 字符，存 `d-reuters-opencli.html` |
| 约 22:15–22:16 | d-edgar | https://www.sec.gov/Archives/edgar/data/2130357/000162828026074944/anthropicpbc-424b4.htm | open，sleep 6 秒；eval 页面结构；eval 检索关键句（textContent 上下文） | ok。页面标题 “Anthropic, PBC - 424(b)(4)”；HTML 约 14 MB；检索命中 “voluntary, revocable”、80%、$83.00、23,648,646 股；摘录存 `d-edgar-424b4-excerpt.txt` |

注：opencli 会话的 HTML 与 txt 未经 barking 的 `html2txt`，txt 由 opencli 的 extract 或 eval 直接转出，故无 `.headers`。

## 四、EDGAR 检索

| 时间 | 工具 | 请求 | 结果 |
|---|---|---|---|
| 约 22:14 | curl，`-A evidence-collector/1.0`，`--max-time 60` | https://efts.sec.gov/LATEST/search-index?q=%22voluntary%2C%20revocable%20expressions%22&dateRange=custom&startdt=2026-11-01&enddt=2026-11-20 | 200，7 条命中（存 `d-edgar-search.json`）；424B4 申报日 2026-11-19 |

## 五、barking heat（2026-11-20 22:15:58 北京）

| 查询 | 结果 |
|---|---|
| HN 条目 50554009、50557885、50567735、50563148 | 全部取到，并存 `d-hn-<id>.json` |
| HN 搜索：“Nvidia AMD China chip licenses” | 0 条 |
| HN 搜索：“Gemini computer use Mac” | 2 条（50571810、50563179） |
| HN 搜索：“Anthropic IPO founders” | 0 条 |
| HN 搜索：“MAI Image 2.7” | 1 条（50536193） |
| HN 搜索：“Aion Instruct Edge”、“ChatGPT accounting”、“data centers premature deaths” | 各 0 条 |
| Reddit 搜索：“Anthropic black hole physics” | 失败：返回登录页（HTML，非 JSON），记入 `d-heat.json` 的 errors |

汇总见 `d-heat.json`。

## 五之二、补充抓取（第二轮，22:20 起）

| 约时间 | 工具 | 请求 | 结果 |
|---|---|---|---|
| 约 22:2x | opencli open + eval 结构 | 通稿页 https://foreignaffairs.house.gov/news/press-releases/chairman-brian-mast-exercises-legislative-authority-to-publicly-release-ai-chip-license-information-for-china | 通稿正文 “You can read the disclosed documents here:” 的链接在静态 HTML 中缺失；渲染后找到两个 Scribd 嵌入（access_key 为公开嵌入参数） |
| 约 22:2x | opencli open + eval innerText | Scribd 嵌入 1：https://www.scribd.com/embeds/1098651862/content?start_page=1&view_mode=scroll&access_key=key-WkuMowyLbX5OPTJOKBvc | ok，2,924 字符，存 `d-scribd-1-disclosure.txt`（原始包装 `d-scribd1-raw.json`） |
| 约 22:2x | 同上 | Scribd 嵌入 2：https://www.scribd.com/embeds/1098651808/content?start_page=1&view_mode=scroll&access_key=key-KRkbe0VANx9ckdpxqyl9 | ok，4,941 字符，存 `d-scribd-2-disclosure.txt`（原始包装 `d-scribd2-raw.json`） |
| 约 22:3x | opencli eval（已开会话 d-edgar） | 424B4 “Selling Stockholders”与“voluntary”段落的上下文 | ok，存 `d-edgar-sections.json` |
| 约 22:3x | curl，UA evidence-collector/1.0，--max-time 60 | https://www.courtlistener.com/api/rest/v4/search/?q=%22Osorio%22%20%22Kilmer%22&type=o | 401（需认证），未使用任何凭据；响应存 `d-courtlistener-search.json` |
| 约 22:4x | opencli open + eval innerText | AA 图像编辑榜 https://artificialanalysis.ai/image/leaderboard/editing | ok，9,830 字符，存 `d-aa-image-editing-leaderboard.txt`（原始包装 `d-aa-leaderboard-raw.json`） |
| 约 22:4x | barking html2txt --force | `d-techdirt.opencli.html` → `d-techdirt-opencli.txt` | 写入 791 行、46,273 字节（替换此前仅前 6000 字的切片） |
| 约 22:4x | barking heat（第一次） | 4 个 HN 条目 + 7 个短查询 | 条目全部取到，但 `d-heat.json` 与 `d-hn-<id>.json` 已存在，barking 拒绝覆盖；第一次输出改名保留为 `d-heat-run1.json` 与 `d-hn-run1-<id>.json` |
| 约 22:4x | barking heat（第二次） | 同上，查询改为 H200、Anthropic IPO、Aion、ChatGPT Accounting、data center air quality、Kilmer、MAI-Image、Gemini Mac | 4 个条目成功；短查询多命中无关条目（如 “Kilmer” 命中 killer robots），仅 “MAI-Image” 与 “Gemini Mac” 有相关命中（后者含 MacRumors 1 分）；Reddit 仍失败 |

## 五之三、发布时刻元数据（HTML 内 published_time / datePublished，换算为北京时间）

- 通稿：2026-11-19T14:18:08-05:00 → 北京 11-20 03:18
- CNBC：2026-11-19T19:55:41Z → 北京 11-20 03:55；dateModified 2026-11-20T00:53:55Z → 北京 11-20 08:53
- Applied Compute：2026-11-02T23:24:00Z → 北京 11-03 07:24
- Gemini 博客：2026-11-19T17:00:00Z → 北京 11-20 01:00
- Anthropic 研究页：2026-11-17T00:00:00Z（疑为日期占位）→ 北京 11-17 08:00
- TechCrunch：2026-11-20T00:43:50Z → 北京 11-20 08:43
- MAI Image：2026-11-17T18:16:36Z → 北京 11-18 02:16
- Edge 博客：2026-11-19T23:08:46Z → 北京 11-20 07:08
- Techdirt：2026-11-19T21:19:47Z → 北京 11-20 05:19
- 无元数据：UTAW 站、OpenAI 会计页与帮助页、MIT（页面日期 November 19, 2026）、PANW（“Nov. 18, 2026”）

## 六、未取到或未取的条目

- D1：路透 2026-05 报道（称“尚无一颗交付”）未取。委员会披露件附表已覆盖“是否出货”的口径。
- D6：Hexagon 预印本页（hexagonmath.org）未取。
- D7：424B4 全文未存（14 MB）；只存命中段落。
- D8：AA 榜页已取（见第五之二节），但 AA 样本数与微软脚注时点不同（5,526 / 6,600 对 5,478，不可直接比较）。
- D10：发布日期未取到。
- 额外线索（DistroKid、OpenAI dots、Google Stitch）未查。
- 社区评论（吠点线索）按派工单 D 组不做。

## 七、假设与处理说明

1. Reuters 与判决书的“周五”与“2026-11-09”冲突，以判决书为准，写入勘误。
2. HN 热度以取样时刻为准；扫描中的分数与取样不同，记取样值。
3. Techdirt 页面标注的时间 “Thu, Nov 19th 2026 01:19pm” 未标时区，北京时间换算为推定。
4. 判决书 PDF 取自 Reuters 页内链接（fingfx.thomsonreuters.com）；CourtListener 搜索需认证，未取；DocumentCloud 403。
5. 本机 Chrome 会话：Techdirt 页面 class 含 `user-not-logged-in`，即该会话未登录 Techdirt；PANW、Reuters、Scribd 嵌入页的 HTML 中未见账号菜单或登录标记（已检索 sign out、log out、my account、avatar 等）。Techdirt 评论区的头像均为匿名或默认头像，属于第三方评论内容。

## 八、待删除文件（本组自生成的废弃中间件，待 Claude 收尾时一次删除）

以下文件在本组工作中生成，内容为错误页或中间封装，已有对应的 txt 或 html 正式存档，请收尾时一并删除：

- docs/2611/1119/sources/d-panw-ir.html
- docs/2611/1119/sources/d-panw-ir.headers
- docs/2611/1119/sources/d-reuters-osorio.html
- docs/2611/1119/sources/d-reuters-osorio.headers
- docs/2611/1119/sources/d-techdirt-osorio.html
- docs/2611/1119/sources/d-techdirt-osorio.headers
- docs/2611/1119/sources/d-documentcloud-osorio.html
- docs/2611/1119/sources/d-documentcloud-osorio.headers
- docs/2611/1119/sources/d-edgar-424b4.html
- docs/2611/1119/sources/d-edgar-424b4.headers
- docs/2611/1119/sources/d-panw-opencli.json
- docs/2611/1119/sources/d-reuters-opencli.json
- docs/2611/1119/sources/d-techdirt-opencli.json
- docs/2611/1119/sources/d-techdirt-eval.json
- docs/2611/1119/sources/d-techdirt-html.json
- docs/2611/1119/sources/d-panw-html.json
- docs/2611/1119/sources/d-reuters-html.json
- docs/2611/1119/sources/d-edgar-snippets.json（如果需要保留可不删；摘录已写入 d-edgar-424b4-excerpt.txt）
- docs/2611/1119/sources/d-scribd1-raw.json（opencli 原始包装，已转 txt）
- docs/2611/1119/sources/d-scribd2-raw.json（同上）
- docs/2611/1119/sources/d-aa-leaderboard-raw.json（同上）
- docs/2611/1119/sources/d-courtlistener-search.json（401 响应体）
- docs/2611/1119/sources/d-heat-run1.json 与 docs/2611/1119/sources/d-hn-run1-50554009.json、d-hn-run1-50557885.json、d-hn-run1-50567735.json、d-hn-run1-50563148.json（第一次热度运行的输出，已被第二次覆盖）
- /tmp/x（工作区外的 opencli 输出重定向临时文件，非本仓库文件）

注：`d-edgar-snippets.json` 与 `d-edgar-sections.json` 是检索原始输出，内容已转入摘录与 evidence；由 Claude 决定是否删除。

## 九、会话未登录验证

见第七节第 5 条。

## 十、时间表

- 22:12 分派开工，22:12:19 第一批 fetch，4 秒完成。
- 约 22:13–22:14 opencli 三页抽取与 Techdirt 正文。
- 22:14:47 EDGAR fetch 403；约 22:15 curl 403；约 22:15 opencli 打开 EDGAR 招股书并检索。
- 22:15:56 判决书 PDF 取得；22:15:58 热度采样。
- 22:17 PANW 与 Reuters 页面 HTML 保存。
- 22:19 交付文件 `evidence-d.md` 与本文件。
