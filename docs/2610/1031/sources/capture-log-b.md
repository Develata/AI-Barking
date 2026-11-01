# B 组 capture-log（北京时间，UTC+8）

执行方 claude-sonnet-5-5；时段 2026-11-02 02:31–03:25。逐条请求的 URL、结果、时刻、字节数与 SHA-256 在机器日志 `sources/b-fetch-log.jsonl`（`barking fetch`）、`sources/b-shots-log.jsonl`（`barking shot` 与自写 `b-cdp-shot.mjs`）、`sources/b-heat.json`。下表只记批次、工具与例外。自写脚本：`sources/b-cdp-shot.mjs`（bun 驱动匿名 headless Chrome，全新临时 profile，不读用户 Cookie，不登录，用于 X 官方嵌入页）；清单 `sources/b-fetch-list1.tsv`–`b-fetch-list8.tsv`。

## 批量存档（barking fetch）

| 时间 | 清单 | 对象 | 结果 |
|---|---|---|---|
| 02:32 | list1 | WIRED 原文、Anthropic 模型总览、HN 50316468、HN 50286286 | 全部 ok（WIRED 全文可读，无付费墙） |
| 02:32 | list2 | Anthropic 弃用页、模型 ID 页 | ok |
| 02:32 | list3 | Opus 5、Sonnet 5 模型页 | ok |
| 02:33 | list4 | TokenPost、EconoTimes、The Decoder、Startup Fortune | ok；DigitalCitizen、XenoSpectrum 403 Cloudflare 质询，blocked，未再取（非必要来源） |
| 02:34 | list5 | HuggingNews | ok |
| 02:36 | list6 | IT之家、腾讯新闻（爱范儿稿）、新浪（IT之家转载）、网易号（DeepTech）、凤凰网 | 全部 ok |
| 02:36 | list7 | explainx.ai、Crypto Briefing | ok |
| 02:36 | list8 | exintherapeutics.com 首页 | blocked（正文仅 48 个非空白字符，需 JavaScript 渲染），未再取 |

## 单次请求

| 时间 | URL | 工具 | 结果 |
|---|---|---|---|
| 02:31 | https://api.fxtwitter.com/status/2115442964574908902 | curl | `b-fx-2115442964574908902.json`，公开 JSON，成功 |
| 02:3x | https://api.fxtwitter.com/2/thread/2115442964574908902 | curl | `b-fx-thread-2115442964574908902.json`，8 条帖串，成功 |
| 02:3x | https://api.fxtwitter.com/2/thread/2116285926121091382 | curl | `b-fx-thread-2116285926121091382.json`，对比帖串 7 条，成功 |
| 02:3x | fxtwitter `/2/profile/…`、`/2/search?…` | curl | 404，不可用（所以找不到对比帖 ID 靠别的路径，见下） |
| 02:3x | https://hn.algolia.com/api/v1/search…（两次） | curl | 成功，仅用于列提交 |
| 02:3x | 对比帖串的 X 状态 ID | 找到途径：WebSearch 与 Exa 搜索结果里出现了 X 镜像站（nitter.gq、teapaw.org）的转录文字；我用 curl 打开 nitter.gq/OcanaSantero/with_replies 只为读出状态 ID（2116285912628052110 等），随后一律用 fxtwitter JSON 与 X 官方嵌入页取内容。镜像站内容未存档、未引用为证据 | 成功 |
| 02:3x | https://x.com/OcanaSantero/status/…（headless Chrome 直接开） | 自写 CDP | 403 “Access to x.com was denied”，按规则不绕过，改用官方嵌入页 |
| 02:36 | `barking heat`（`--hn` 三组、`--item` 三个、`--reddit` 一组） | barking | HN 成功（`b-heat.json`、`b-hn-*.json`）；Reddit 失败：old.reddit.com 重定向到登录页，响应非 JSON，不使用登录态 |
| 02:3x | WebSearch ×4、Exa 搜索 ×2 | 内置 | 用于找中英文报道与 X 帖 ID，结果只作线索 |

## 截图（`barking shot` 与 `b-cdp-shot.mjs`）

- `images/13`–`19`：X 官方嵌入页（`platform.twitter.com/embed/Tweet.html?id=<id>&hideThread=true`），`b-cdp-shot.mjs` 在匿名 headless Chrome（视口 700 CSS px，DPR 2，时区设为 Asia/Shanghai，语言 en-US）渲染后整卡存到会话临时目录，再用 `barking shot … stack` 拼成：文字段 + 页脚（时间、点赞、回复数）。帖内照片或视频（设备与活体小鼠照片、视频缩略图）不入图，所以中间有一段被去掉；`13` `15` `16` `17` `18` `19` 如此处理，`14`（无配图的帖）整卡裁出。帖子里的作者头像与账号名是 CEO 本人（被取证对象），不是抓取者；页面匿名，未登录。
- 第一轮尝试作废（嵌入页内的媒体未被脚本隐藏、拼接坐标按 DOM 量得有偏差）：6 张 PNG 已移到会话临时目录（`<scratchpad>/t/bad/`），不在仓库内；现行文件是第二轮用像素扫描定位照片段后拼的。`b-shots-log.jsonl` 里前面的 `render`（tw-*）条目对应中间图，不是成品。
- `images/20`、`21`：WIRED 整页 `barking shot render --width 700 --height 9000`（中间图 `wiredfull`），按 `lines` 的 OCR 坐标 `stack`；页面有广告穿插，所以段落之间做了拼接，拼接处在文字行之间。
- `images/22`：IT之家 `render` 后 `lines --lang chi_sim+eng` 定位，拼标题与“开颅和缝合”段。
- `images/23`：Anthropic Opus 5、Sonnet 5 页各取顶部（Legacy 横幅与标题）。
- `images/24`：网易号页面在 700 CSS px 宽度下版式被截断（右侧溢出），改用 `--width 1000 --scale 1.4`（宽度仍为 1400 px，但像素比为 1.4，不是 2；原因：该页不是响应式，700 CSS px 排版会裁掉右半边）。
- 深色底：无。
- 未截：The Decoder（页面被 Cookie 同意遮罩盖住，不点“接受”）、爱范儿/腾讯新闻（配图含手术界面截图）、对比帖里的设备与活体照片。

## 重复尝试与失败汇总

- 嵌入页第一次 `barking shot render` 只得到空白图（嵌入页需要等 JS），改自写 CDP 脚本等待 `article` 出现，成功。
- DigitalCitizen、XenoSpectrum：403 质询，未取。
- exintherapeutics.com：JS 页，未取；未找到 Substack 或技术报告链接，因此“报告已发布”一项记为未找到。
- Reddit：失败。

## 假设

- WIRED 页面时间 “Oct 31, 2026 6:00 AM” 按美东（EDT）理解；原页未标注时区。
- 帖子页脚时间按我渲染时的 Asia/Shanghai 设置显示，等同北京时间；与 fxtwitter 的 UTC 时间交叉一致（例如 13:58 UTC → 21:58）。
- 中间图（`/tmp/barking-shot/26101031-b/…`、会话临时目录）不入库、不在仓库内。
