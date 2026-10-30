# 1029 期 取证 C 组：速览（Google 系）证据表（evidence-C）

执行方：claude-haiku-5-5。时间为北京时间（UTC+8，2026-10-30 起的抓取）。存档一律在 `docs/2610/1029/sources/` 下，文件名以 `c-` 开头。URL 已去掉查询参数。截图：C 组不需要，未做。

| 说法 | 级 | 一手来源 URL | 原文摘句（逐字） | 存档 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| C1 Gemini 应用内直接编辑文件（PDF、Docs、Sheets、Slides、Markdown、TXT、代码） | L1 | https://blog.google/innovation-and-ai/products/gemini-app/edit-files-directly/ ；https://workspaceupdates.googleblog.com/2026/10/create-preview-and-edit-files-directly-in-the-Gemini-app.html | 博文：“PDF preview and editing is supported on both the web and mobile, while all other formats (Google Docs, Google Sheets, slide decks, Markdown files, TXT files, and code) are supported on the web only.”；“These capabilities are now available to all users in all languages where the Gemini app is available.” Workspace 更新：“Available to all Google Workspace customers, Workspace Individual subscribers, and users with personal Google accounts”；Rollout 为 “Rapid Release and Scheduled Release domains … Available now”。 | `c-c1-gemini-blog.html/.txt`；`c-c1-workspace.html/.txt` | 官方只给日期 Oct 29, 2026（北京时刻：官方未给时刻）。Workspace 版另写“Workspace 管理员无控制项”（Admins: There is no admin control for this feature）。Workspace 版列出 LaTeX，博文未列。 | 已找到 |
| C2 Gemini API Computer Use 正式可用（Interactions 与 GenerateContent API） | L1 | https://ai.google.dev/gemini-api/docs/changelog （英文 `?hl=en`）；https://ai.google.dev/gemini-api/docs/computer-use | 更新记录：“Computer Use generally available (GA): Released the Computer Use tool to general availability (GA) across the Interactions and GenerateContent APIs.” 文档：“You can use Computer Use with the following models: Gemini 4 Argon (gemini-4-argon) … Gemini 3.8 Flash (gemini-3.8-flash) … Gemini 3.5 Flash-Lite (gemini-3.5-flash-lite) … Gemini 3 Flash Preview (gemini-3-flash-preview): Preview model supporting computer use.” | `c-c2-gemini-changelog-en.txt`（英文全文，条目在 October 30, 2026 段）；`c-c2-computer-use-docs.html/.txt`；`c-c2-gemini-changelog.headers` 为 barking fetch 失败（跳转过多）留下的记录，未用 | 官方只给日期 Oct 30, 2026（北京时刻：官方未给时刻）。changelog 未列模型，模型清单取自 computer-use 文档页（文档页无日期，抓取时为 2026-10-30 的版本，可能已变）。“只限 API”：changelog 只写 “across the Interactions and GenerateContent APIs”，未说明是否有其他入口。 | 已找到（模型清单仅来自文档页，未核版本历史） |
| C3 Alphabet Q3：营收同比 +25%，Cloud 收入 +103% | L1（营收与 Cloud）；L4（AI Mode 说法） | https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q3-2026/ | CEO 博文（Oct 28, 2026）：“Alphabet revenues grew 25% year-over-year.”；“Google Cloud revenues grew 103%.” 第三方逐字稿（CFO 段）：“Consolidated revenues were $127.6 billion, up 25% in both reported and constant currency.” Schindler 段：“So first, there was FX. It was the single largest driver of Search deceleration relative to Q2.”；“It’s worth noting that the strong growth in AI Mode queries had an impact because of the lower monetization we were seeing in AI Mode.” | `c-c3-alphabet-q3-ceo.html/.txt`；`c-c3-singjupost-transcript.html` 与 `c-c3-transcript-singjupost.txt`（第三方逐字稿，非官方） | 博文日期 Oct 28, 2026（北京时刻：官方未给时刻；美东时间）。营收与 Cloud 数字取自 CEO 发言稿，口径是 year-over-year 与 Alphabet 整体。“汇率是最大因素”限定为 Search 相对 Q2 的增速放缓，不是整体营收的原因；CFO 同段写 headline 25% 在报告货币与固定汇率下一致（up 25% in both reported and constant currency）。AI Mode 的 “lower monetization” 原句未写比较基准（scan 的“低于传统搜索”是补写），未见 IR 官方逐字稿。Search 营收 $63.3 billion，+12%（CFO 段）。 | 部分支持（营收 25%、Cloud 103% 已找到；AI Mode 说法只有第三方逐字稿，比较基准未核） |
| C4 Google 关停 Jules，经验并入 Antigravity | L5（仅 IT之家转述） | 未找到一手来源（官方 X 原帖未找到；jules.google 首页与文档页无此公告） | IT之家（北京 2026/10/30 11:10:29）：“随着我们将重心转向打造统一的体验，Jules 将逐步停止运营。”（中文转述，原英文未找到） | `c-c4-ithome-jules.html/.txt`；`c-c4-jules-home.html/.txt`；`c-c4-jules-docs.html/.txt`；`c-c4-fx-googlejules-profile.json`（@googlejules 资料，见下） | 停止时间表：未找到。付费用户处理：未找到。Web 搜索（英文关键词）未找到相关官方公告。@googlejules 的 fxtwitter 资料：粉丝 190、verified=false，无法据此确认是 Google 官方账号。 | 未找到一手来源 |
| C5 Waymo × 丰田：日本测试 bZ4X 车队 | L1 | https://waymo.com/blog/2026/10/toyota-bz4x-partnership | “Toyota and Waymo will begin initial operations to test and validate an autonomous Toyota bZ4X fleet in Japan in 2027, with deployment to the United Kingdom and other Waymo-serviced markets planned for the following years.” 另：“will accelerate scaling safety-validated Level 4 autonomous vehicles globally”；车辆在丰田高冈工厂（Takaoka plant）组装。 | `c-c5-waymo-toyota.html/.txt` | 官方日期 October 30, 2026（北京时刻：官方未给时刻）。原文未写城市、车队规模、是否有安全员、是否载客、是否无人驾驶运营（测试阶段）。“Japan’s first fully autonomous ride-hailing service” 是背景描述，不是本次测试的说明。 | 已找到（城市、规模、安全员、载客均未写） |
| C6 Chrome 自动填表可引用 Gmail 信息 | L1 | https://blog.google/products-and-platforms/products/chrome/enhanced-autofill-gmail/ | “Starting today, Chrome’s enhanced autofill feature can automatically suggest information pulled directly from Gmail as you fill out online forms.”；“These capabilities are beginning to roll out in the U.S. on desktop, Android and iOS for Gemini in Chrome users with Personal Intelligence enabled.”；“You can manage whether enhanced autofill is enabled and what it can access via your Chrome settings.” | `c-c6-chrome-autofill.html/.txt` | 官方日期 Oct 28, 2026（北京时刻：官方未给时刻）。地区：美国；资格：Gemini in Chrome 用户且启用 Personal Intelligence；平台：桌面、Android、iOS；“逐步开放”（beginning to roll out）。默认开关状态：原文未写。原文只写“可以管理是否启用”，未写“可在设置关闭”的具体路径。 | 已找到（默认开关状态未写） |

## 扫描说法勘误

- C1：与扫描一致。补充：Workspace 版把 LaTeX 列入可编辑的 PDF/文档类；博文版不列。“手机与网页均可编辑 PDF”与原文一致。
- C2：与扫描一致（Interactions 与 GenerateContent API；GA）。扫描未列模型，本组补：文档页列出 Gemini 4 Argon、Gemini 3.8 Flash、Gemini 3.5 Flash-Lite、Gemini 3 Flash Preview（后者标为 preview）。
- C3：营收 +25%、Cloud +103% 与扫描一致。“汇率是搜索环比放缓最大因素”与原文一致，但原文限定为 Search 相对 Q2 的增速放缓（single largest driver of Search deceleration relative to Q2），不是全公司营收放缓。“AI Mode 商业化低于传统搜索”：原文只写 “lower monetization we were seeing in AI Mode”，比较基准未写，属于补写。第三方逐字稿另列营收 $127.6 billion；财经网站 MarketBeat 报 $102.35 billion、+14%，与官方 25% 及第三方逐字稿冲突，本组记录不取舍（该数字未核）。
- C4：扫描的“中文稿据 @GoogleJules 官方 X”未能核到原帖。IT之家引用了 X 声明的中文译文，原英文未找到。“经验并入 Antigravity”与 IT之家转述一致，但只有中文转述。
- C5：与扫描一致（2027 年起日本测试，bZ4X）。扫描未提的 “deployment to the United Kingdom and other Waymo-serviced markets planned for the following years” 原文有。
- C6：扫描“可在设置关闭”是推断；原文只写“可以管理是否启用”，未写默认状态与关闭路径。“美国、Personal Intelligence”与原文一致。

## 热度（barking heat，c-heat.json）

- 取得时刻 2026-10-30T23:11:51+08:00；HN 搜索窗口自 2026-10-23 起；6 组查询（见 `c-heat.json`）：命中 0 条，Reddit 未查。
- 注：`barking heat` 因 `c-heat.json` 已存在而拒绝覆盖，不能直接重跑。若要用更宽的查询重跑，需先删除该文件（由 Claude 收尾时处理，本组未 rm）。

## 未找到 / 待核

- C4 全部：官方 X 原帖；英文原文；停止时间表；付费用户处理。
- C3：IR 官方逐字稿（abc.xyz/investor 返回 Cloudflare challenge，见 `c-c3-abc-investor.html`，记为 blocked）；AI Mode 比较基准；MarketBeat 与官方营收数字冲突。
- C2：changelog 的 “只限 API” 是否成立；computer-use 文档页版本历史。
- 第三方逐字稿（singjupost）仅作线索，不替代 IR 原文；正文引用前需回到 IR 原文。

## 本组未收（按派工单）

额度重置；Gemini 4 Carbon 与 AA v5（1028 已发）；AA HLE-Diamond；Google《The Economics of Intelligence》。
