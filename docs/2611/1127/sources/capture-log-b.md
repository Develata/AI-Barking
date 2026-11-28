# 1127 B 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。本机 macOS，UTC+8（北京时间）；取证 2026-11-28 23:32 起，约 23:55 写成本文。批量抓取逐条结果在 `b-fetch-log.jsonl`（含北京时刻），截图动作在 `b-shots-log.jsonl`，热度在 `b-heat.json`。清单文件为 `b-fetch-list-1.tsv` 至 `b-fetch-list-8.tsv`。`BARKING_CHROME` 指向 `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`。`opencli doctor` 23:32 确认 Browser Bridge 已连接。

## 一、网页存档（barking fetch）

| 批 | 北京时刻 | 清单 | 结果 |
|---|---|---|---|
| 1 | 23:32:31 | WRAL 议员稿（`wral-investigates`）、HN 帖页 | 2 条 ok |
| 2 | 23:32:39 | WRAL 首报（`wral-first`） | ok |
| 3 | 23:32:56 | WRAL 房东回应稿、WBTV（CNN Newsource 转载 WRAL）、Yahoo Finance（Moneywise 转载）、explainx.ai、ai.lounge.ai | 4 条 ok；`ailounge` HTTP 429，blocked |
| 4 | 23:33:29 | ncleg.gov S963 与 S624 页面与文本、Burgin 议员页、moneywise.com 原站、Futurism、Yahoo（WRAL 转述） | `ncleg-burgin` HTTP 500 failed（重试一次仍败）；`moneywise` HTTP 403 Cloudflare 质询，blocked；其余 ok |
| 5 | 23:34:04 | WRAL 9 月稿（Burgin 自称 “Bot Bill”）、Atlanta Black Star、AfroTech、乐联网 `leho.com.tw`、Yahoo（另一转载）、Glonce | `leho` HTTP 403 blocked；其余 ok |
| 6 | 23:35:57 | 简体与繁体中文转载（yam、民生电子报、80aj）、TechTimes、Newser、Unilad、The Mary Sue、Substack（HN 帖所链） | 8 条 ok |
| 7 | 23:36:19 | 法律背景：Moffatt（CanLII、BC CRT）、加州 BPC § 17941、Utah Code | `law-moffatt-canlii` 403 blocked；`law-moffatt-crt` 404 failed；`law-ca-sb1001` 403 Cloudflare blocked；`law-ut-*` 两条 HTTP 200 但正文是 “Request Rejected”（WAF 拒绝页，245 字节），实为失败，不可用 |
| 8 | 约 23:40 | Moffatt 决定书 PDF 副本（loyaltylobby.com，第三方托管）、ABA 文章 | PDF ok（386 行文本，页眉 “2024 BCCRT 149 (CanLII)”）；`law-moffatt-aba` 403 Cloudflare blocked |

`ok` 的 `.txt` 已抽查是正文（WRAL 三篇、Yahoo/Moneywise、Futurism、yam、80aj、ncleg 页面与文本、Moffatt）。WBTV 与 Yahoo 的 `.txt` 含大量导航，正文在其中。Yahoo/Moneywise 页的正文折叠（`Story Continues`），Leach 完整声明在 `.txt` 里有，截图时见第三节。

## 二、个别请求（curl / opencli / 搜索）

| 北京时刻 | 动作 | 结果 |
|---|---|---|
| 23:34:12 | `barking heat`：`--item 50673521`、`--hn "Olivia eviction"`、`--hn "landlord's AI assistant"`、`--reddit "Raleigh eviction AI Olivia"` | HN 帖 198 分 / 155 评（含嵌套），另存 `b-hn-50673521.json`（评论树）；`Olivia eviction` 命中 0 条；Reddit `old.reddit.com/search.json` 重定向到登录页，失败，记入 `b-heat.json` 的 `errors`，不绕过 |
| 约 23:36 | opencli 渲染 `https://leginfo.legislature.ca.gov/...BPC&sectionNum=17941.`，取 innerText | 成功，存 `b-law-ca-bpc-17941.txt`（纯文本；该页 fetch 被 Cloudflare 拦，渲染后取到） |
| 约 23:36 | opencli 打开 `decisions.civilresolutionbc.ca/.../525448/index.do`、`le.utah.gov/xcode/Title13/Chapter75/...`、`canlii.org` | 404 / 404 / 空页；放弃 |
| 约 23:38 | WRAL 时间戳核实：opencli 打开首报页，`eval` 读取 `.timestamp` 元素与 `Intl` 时区 | 静态文本 `11/19/2026, 11:35:08 PM` 与 meta `2026-11-19T23:35:08Z` 一致；浏览器渲染后显示 `上午7:35 2026年11月20日`（UTC+8），确认原值为 UTC |
| 约 23:47 | `curl` 探测 `portal-nc.tylertech.cloud/Portal/` 与 `nccourts.gov`（`--max-time 40`） | 均 HTTP 403（前者 118 字节）。按规则不绕过；未检索租客案卷（私人个体） |
| 多次 | WebSearch（WRAL 其他稿件、中文报道、Burgin 法案、Olivia 供应商、Moffatt 全文来源、Burgin 2023–2024 提案） | 只作发现链接；所有引用以抓取到的原文为准；Olivia 供应商与 2023–2024 届提案未找到 |

## 三、截图（全部 2 倍像素比，宽度 ≤ 1400 px；逐张目视确认）

方法：`barking shot render`（headless Chrome，700 CSS px 宽，DPR 2，`--lang=en-US`；注：页面中 WRAL 时间戳仍被 JS 本地化为中文格式，UTC+8，截图里 “上午7:35 2026年11月20日” = UTC 11-19 23:35）；用 `barking shot lines` OCR 找行坐标，`cropbox` / `stack` 裁切拼接（只裁切不改像素；拼接缝隙为浅灰 #f7f7f7，深色底页为 #111518）。Yahoo/Moneywise 页和 Futurism、yam 另述。所有截图均排除了页面里的租客面部视频封面（WRAL 页顶部视频画面与 Yahoo 页主图）与广告位。

| 编号 | 文件 | 来源与取法 | 备注 |
|---|---|---|---|
| 13 | `13-wral-first-report.png` | WRAL 首报，render + cropbox（标题、导语、Posted/Updated 行） | 时间戳本地化显示（UTC+8） |
| 14 | `14-wral-nov8-emails.png` | WRAL 首报，render + stack（Nov 8 邮件引文段、“never identified… signed” 段） | 跳过页面中的空白广告位 |
| 15 | `15-wral-court-timeline.png` | WRAL 房东回应稿，render + stack（标题与 Posted；Nov 8、Nov 16、Leach 声明、WRAL 查法院文件、Nov 18/23 引语） | |
| 16 | `16-moneywise-leach-full.png` | Yahoo Finance 转载 Moneywise：opencli（background 窗口）打开；用 `eval` 把 3 个折叠（`display:none`）的节点设为可见、隐藏 “Story Continues” 按钮后，整页 `zoom:2` 截图，再 stack（标题与作者日期块；Leach 完整声明及 “neither confirmed nor denied” 句） | **深色底**（页面深色主题）；只取标题块与声明块，排除主图（含租客面部）；展开折叠只改显示不改文字 |
| 17 | `17-wral-ai-labeling.png` | WRAL 议员稿，render + stack（标题；“documents, which were provided by Mack… didn't disclose”；“hole in the downstairs bathroom ceiling”；2025-12 邮件；“Best regards” 署名） | |
| 18 | `18-wral-burgin-botbill.png` | WRAL 议员稿，render + stack（标题；Burgin 采访；“It's ridiculous”；Leach 周五声明；Pinocchio；Bot Bill） | |
| 19 | `19-headlines-futurism-yam.png` | Futurism 与 yam 蕃新聞（商傳媒）标题与日期，render + stack | Futurism 顶部为 stock photo，已不含；yam 配图为示意图，已不含 |
| 20 | `20-hn-50673521.png` | HN 帖，render + cropbox（标题、域名、分数、提交者、评论数） | 截图时 199 分 / 156 评（北京约 23:42） |
| 21 | `21-wral-mack-prior-cases.png` | WRAL 房东回应稿，render + stack | “similar eviction proceedings” 段与租客引语 |
| 22 | `22-ncleg-s963-status.png` | ncleg.gov S963 页面，render + cropbox（标题、Last Action、Sponsors、全部 History） | 浅色 |
| 23 | `23-ncleg-s963-chatbot-disclosure-v2.png` | S963 First Edition PDF（`barking shot pdf` 渲第 4、5、7 页）+ stack | 第 4–5 页 “covered platform” 定义；第 7 页 § 170-3(3) 身份披露义务；`v2` 为重做版，旧版见“待删除” |
| 24 | `24-moffatt-para27.png` | Moffatt 决定书 PDF 副本第 6 页，render pdf + stack | 第 27 段；副本来自第三方托管（CanLII 官方页 403） |

未做的候选：派工单“17 关于 AI 邮件标注的通知（如有）”只有 WRAL 的描述，没有邮件原件，已用 WRAL 描述段（17）；“18 Burgin 表态或法案页”做成 18、22、23 三张。

## 四、失败与受限

| 项 | 原因 | 处理 |
|---|---|---|
| NC 法院门户 `portal-nc.tylertech.cloud`、`nccourts.gov` | 匿名请求 HTTP 403 | 不绕过；法院文件只有 WRAL 转述与房东声明；未检索租客姓名 |
| Moneywise 原站、ABA、`leho.com.tw`、CanLII、加州 leginfo（fetch）、`ai.lounge.ai` | 403 / Cloudflare 质询 / 429 | Moneywise 改取 Yahoo Finance 转载版；加州条文改用 opencli 渲染取得；其余记失败 |
| `le.utah.gov` Utah Code | WAF “Request Rejected”（fetch 与 opencli 均得） | 未取得，不写 Utah 条文 |
| BC CRT 官方决定书 | 404 / CanLII 403 | 用 loyaltylobby.com 托管的 PDF 副本，仅作背景并注明 |
| `ncleg-burgin`（议员页） | HTTP 500，重试一次仍败 | 议员身份改用 WRAL 稿与 ncleg 法案页的 “Burgin (Primary)” |
| Reddit 搜索 | 重定向登录页 | 不绕过 |
| Yahoo Finance 页折叠正文 | 前端折叠；“Story Continues” 按钮在 background 窗口下 native click 被拒（`Native input requires a visible browser page`） | 改用 `eval` 取消 `display:none` 后截图；窗口 foreground 时截图命令返回 `Unable to capture screenshot`，改回 background |
| 第一次 headless 渲染 Yahoo 页 | 折叠后只渲染到 “Story Continues” 之前 | 同上 |

## 五、自查

- 截图自查：13–24 已逐张打开目视，文字清楚，裁切落在行间；无租客面部照片、视频帧；无登录栏、头像或抓取者信息（HN 截图顶栏有 “login” 字样，是 HN 匿名页面的登录入口，未登录）。
- `docs/2611/1127/` 范围内 grep：`b-` 前缀文件与本组 `images/13–24` 无抓取者姓名、邮箱、本机用户名；HN 提交者与评论者用户名是 HN 公开信息，评论摘录里保留 HN 用户名作出处。
- 单个文件大于 1 MB 的文本存档：无（最大 `b-yahoo-finance.html` 890 KB，HTML 原件本地不入库）。

## 六、待删除文件（精确路径，由主执行方收尾时一次删除）

- `docs/2611/1127/images/16-moneywise-leach-statement.png`（底部 “agent.” 被截断，已用 `16-moneywise-leach-full.png` 重做）
- `docs/2611/1127/images/23-ncleg-s963-chatbot-disclosure.png`（拼接段首尾被切半行，已用 `-v2` 重做）
- `docs/2611/1127/sources/b-law-ut-13-75-103.headers`、`b-law-ut-13-75-103.html`、`b-law-ut-13-75-103.txt`
- `docs/2611/1127/sources/b-law-ut-13-2-12.headers`、`b-law-ut-13-2-12.html`、`b-law-ut-13-2-12.txt`
  （以上两组 Utah 条目是 WAF “Request Rejected” 页，`b-fetch-log.jsonl` 里被记作 ok，实为无效）

另有 `/private/tmp/claude-501/` 下的临时截图与 `/tmp/portal.html`，在仓库外，无需处理。
